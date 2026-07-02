#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_reactive_block_55(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign21650_e16692, assign21650_e16692_d_n0, assign21650_e16692_d_n2, assign21650_e16692_d_n4, assign21650_e16692_d_n5, assign21650_e16692_d_n6, assign21650_e16692_d_n7, assign21650_e16692_d_n8, assign21650_e16692_d_n9, assign21650_e16692_d_n10, assign21650_e16692_d_n13,) = {
    if (locals.var_guard409 != 0.0) {
        let assign21650_e16686: f64 = (locals.var_vdsemodenml * locals.var_rdd);
        let assign21650_e16689: f64 = (locals.var_vdsemodervs * locals.var_rsd);
        let assign21650_e16690: f64 = (assign21650_e16686 + assign21650_e16689);
        (assign21650_e16690, ((locals.var_vdsemodenml * locals.var_rdd_dn0) + (locals.var_vdsemodervs * locals.var_rsd_dn0)), ((locals.var_vdsemodenml * locals.var_rdd_dn2) + (locals.var_vdsemodervs * locals.var_rsd_dn2)), ((locals.var_vdsemodenml * locals.var_rdd_dn4) + (locals.var_vdsemodervs * locals.var_rsd_dn4)), ((locals.var_vdsemodenml * locals.var_rdd_dn5) + (locals.var_vdsemodervs * locals.var_rsd_dn5)), ((locals.var_vdsemodenml * locals.var_rdd_dn6) + (locals.var_vdsemodervs * locals.var_rsd_dn6)), ((locals.var_vdsemodenml * locals.var_rdd_dn7) + (locals.var_vdsemodervs * locals.var_rsd_dn7)), ((locals.var_vdsemodenml * locals.var_rdd_dn8) + (locals.var_vdsemodervs * locals.var_rsd_dn8)), ((locals.var_vdsemodenml * locals.var_rdd_dn9) + (locals.var_vdsemodervs * locals.var_rsd_dn9)), ((locals.var_vdsemodenml * locals.var_rdd_dn10) + (locals.var_vdsemodervs * locals.var_rsd_dn10)), ((locals.var_vdsemodenml * locals.var_rdd_dn13) + (locals.var_vdsemodervs * locals.var_rsd_dn13)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign21650_e16692;
        locals.var_t0_dn0 = assign21650_e16692_d_n0;
        locals.var_t0_dn2 = assign21650_e16692_d_n2;
        locals.var_t0_dn4 = assign21650_e16692_d_n4;
        locals.var_t0_dn5 = assign21650_e16692_d_n5;
        locals.var_t0_dn6 = assign21650_e16692_d_n6;
        locals.var_t0_dn7 = assign21650_e16692_d_n7;
        locals.var_t0_dn8 = assign21650_e16692_d_n8;
        locals.var_t0_dn9 = assign21650_e16692_d_n9;
        locals.var_t0_dn10 = assign21650_e16692_d_n10;
        locals.var_t0_dn13 = assign21650_e16692_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign21690_e16724, assign21690_e16724_d_n0, assign21690_e16724_d_n2, assign21690_e16724_d_n4, assign21690_e16724_d_n5, assign21690_e16724_d_n6, assign21690_e16724_d_n7, assign21690_e16724_d_n8, assign21690_e16724_d_n9, assign21690_e16724_d_n10, assign21690_e16724_d_n13,) = {
    if (locals.var_guard409 != 0.0) {
        let assign21690_e16718: f64 = (locals.var_vdsemodenml * locals.var_rsd);
        let assign21690_e16721: f64 = (locals.var_vdsemodervs * locals.var_rdd);
        let assign21690_e16722: f64 = (assign21690_e16718 + assign21690_e16721);
        (assign21690_e16722, ((locals.var_vdsemodenml * locals.var_rsd_dn0) + (locals.var_vdsemodervs * locals.var_rdd_dn0)), ((locals.var_vdsemodenml * locals.var_rsd_dn2) + (locals.var_vdsemodervs * locals.var_rdd_dn2)), ((locals.var_vdsemodenml * locals.var_rsd_dn4) + (locals.var_vdsemodervs * locals.var_rdd_dn4)), ((locals.var_vdsemodenml * locals.var_rsd_dn5) + (locals.var_vdsemodervs * locals.var_rdd_dn5)), ((locals.var_vdsemodenml * locals.var_rsd_dn6) + (locals.var_vdsemodervs * locals.var_rdd_dn6)), ((locals.var_vdsemodenml * locals.var_rsd_dn7) + (locals.var_vdsemodervs * locals.var_rdd_dn7)), ((locals.var_vdsemodenml * locals.var_rsd_dn8) + (locals.var_vdsemodervs * locals.var_rdd_dn8)), ((locals.var_vdsemodenml * locals.var_rsd_dn9) + (locals.var_vdsemodervs * locals.var_rdd_dn9)), ((locals.var_vdsemodenml * locals.var_rsd_dn10) + (locals.var_vdsemodervs * locals.var_rdd_dn10)), ((locals.var_vdsemodenml * locals.var_rsd_dn13) + (locals.var_vdsemodervs * locals.var_rdd_dn13)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign21690_e16724;
        locals.var_t0_dn0 = assign21690_e16724_d_n0;
        locals.var_t0_dn2 = assign21690_e16724_d_n2;
        locals.var_t0_dn4 = assign21690_e16724_d_n4;
        locals.var_t0_dn5 = assign21690_e16724_d_n5;
        locals.var_t0_dn6 = assign21690_e16724_d_n6;
        locals.var_t0_dn7 = assign21690_e16724_d_n7;
        locals.var_t0_dn8 = assign21690_e16724_d_n8;
        locals.var_t0_dn9 = assign21690_e16724_d_n9;
        locals.var_t0_dn10 = assign21690_e16724_d_n10;
        locals.var_t0_dn13 = assign21690_e16724_d_n13;
        locals.var_t0_rv = 0.0;

        let assign21730_e16749: f64 = if locals.var_vbs > locals.var_vbs_bnd { 1.0 } else { 0.0 };
        locals.var_guard422 = assign21730_e16749;
        locals.var_guard422_rv = 0.0;

        let (assign21740_e16755, assign21740_e16755_d_n0, assign21740_e16755_d_n2, assign21740_e16755_d_n4, assign21740_e16755_d_n5, assign21740_e16755_d_n6, assign21740_e16755_d_n7, assign21740_e16755_d_n8, assign21740_e16755_d_n9, assign21740_e16755_d_n10, assign21740_e16755_d_n13,) = {
    if (locals.var_guard422 != 0.0) {
        let assign21740_e16753: f64 = (locals.var_vbs - locals.var_vbs_bnd);
        (assign21740_e16753, (-locals.var_vbs_bnd_dn0), (-locals.var_vbs_bnd_dn2), (-locals.var_vbs_bnd_dn4), (locals.var_vbs_dn5 - locals.var_vbs_bnd_dn5), (-locals.var_vbs_bnd_dn6), (locals.var_vbs_dn7 - locals.var_vbs_bnd_dn7), (locals.var_vbs_dn8 - locals.var_vbs_bnd_dn8), (-locals.var_vbs_bnd_dn9), (-locals.var_vbs_bnd_dn10), (-locals.var_vbs_bnd_dn13),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign21740_e16755;
        locals.var_t1_dn0 = assign21740_e16755_d_n0;
        locals.var_t1_dn2 = assign21740_e16755_d_n2;
        locals.var_t1_dn4 = assign21740_e16755_d_n4;
        locals.var_t1_dn5 = assign21740_e16755_d_n5;
        locals.var_t1_dn6 = assign21740_e16755_d_n6;
        locals.var_t1_dn7 = assign21740_e16755_d_n7;
        locals.var_t1_dn8 = assign21740_e16755_d_n8;
        locals.var_t1_dn9 = assign21740_e16755_d_n9;
        locals.var_t1_dn10 = assign21740_e16755_d_n10;
        locals.var_t1_dn13 = assign21740_e16755_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign21750_e16761, assign21750_e16761_d_n0, assign21750_e16761_d_n2, assign21750_e16761_d_n4, assign21750_e16761_d_n5, assign21750_e16761_d_n6, assign21750_e16761_d_n7, assign21750_e16761_d_n8, assign21750_e16761_d_n9, assign21750_e16761_d_n10, assign21750_e16761_d_n13,) = {
    if (locals.var_guard422 != 0.0) {
        let assign21750_e16759: f64 = (locals.var_vbs_max - locals.var_vbs_bnd);
        (assign21750_e16759, (locals.var_vbs_max_dn0 - locals.var_vbs_bnd_dn0), (locals.var_vbs_max_dn2 - locals.var_vbs_bnd_dn2), (locals.var_vbs_max_dn4 - locals.var_vbs_bnd_dn4), (locals.var_vbs_max_dn5 - locals.var_vbs_bnd_dn5), (locals.var_vbs_max_dn6 - locals.var_vbs_bnd_dn6), (locals.var_vbs_max_dn7 - locals.var_vbs_bnd_dn7), (locals.var_vbs_max_dn8 - locals.var_vbs_bnd_dn8), (locals.var_vbs_max_dn9 - locals.var_vbs_bnd_dn9), (locals.var_vbs_max_dn10 - locals.var_vbs_bnd_dn10), (locals.var_vbs_max_dn13 - locals.var_vbs_bnd_dn13),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign21750_e16761;
        locals.var_t2_dn0 = assign21750_e16761_d_n0;
        locals.var_t2_dn2 = assign21750_e16761_d_n2;
        locals.var_t2_dn4 = assign21750_e16761_d_n4;
        locals.var_t2_dn5 = assign21750_e16761_d_n5;
        locals.var_t2_dn6 = assign21750_e16761_d_n6;
        locals.var_t2_dn7 = assign21750_e16761_d_n7;
        locals.var_t2_dn8 = assign21750_e16761_d_n8;
        locals.var_t2_dn9 = assign21750_e16761_d_n9;
        locals.var_t2_dn10 = assign21750_e16761_d_n10;
        locals.var_t2_dn13 = assign21750_e16761_d_n13;
        locals.var_t2_rv = 0.0;

        let (assign21760_e16767, assign21760_e16767_d_n0, assign21760_e16767_d_n2, assign21760_e16767_d_n4, assign21760_e16767_d_n5, assign21760_e16767_d_n6, assign21760_e16767_d_n7, assign21760_e16767_d_n8, assign21760_e16767_d_n9, assign21760_e16767_d_n10, assign21760_e16767_d_n13,) = {
    if (locals.var_guard422 != 0.0) {
        let assign21760_e16765: f64 = (locals.var_t1 / locals.var_t2);
        (assign21760_e16765, (((locals.var_t1_dn0 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn0)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn2 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn2)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn4 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn4)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn5 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn5)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn6 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn6)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn7 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn7)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn8 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn8)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn9 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn9)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn10 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn10)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn13 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn13)) / (locals.var_t2 * locals.var_t2)),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13,)
    }
};
        locals.var_tmf1 = assign21760_e16767;
        locals.var_tmf1_dn0 = assign21760_e16767_d_n0;
        locals.var_tmf1_dn2 = assign21760_e16767_d_n2;
        locals.var_tmf1_dn4 = assign21760_e16767_d_n4;
        locals.var_tmf1_dn5 = assign21760_e16767_d_n5;
        locals.var_tmf1_dn6 = assign21760_e16767_d_n6;
        locals.var_tmf1_dn7 = assign21760_e16767_d_n7;
        locals.var_tmf1_dn8 = assign21760_e16767_d_n8;
        locals.var_tmf1_dn9 = assign21760_e16767_d_n9;
        locals.var_tmf1_dn10 = assign21760_e16767_d_n10;
        locals.var_tmf1_dn13 = assign21760_e16767_d_n13;
        locals.var_tmf1_rv = 0.0;

        let (assign21770_e16773, assign21770_e16773_d_n0, assign21770_e16773_d_n2, assign21770_e16773_d_n4, assign21770_e16773_d_n5, assign21770_e16773_d_n6, assign21770_e16773_d_n7, assign21770_e16773_d_n8, assign21770_e16773_d_n9, assign21770_e16773_d_n10, assign21770_e16773_d_n13,) = {
    if (locals.var_guard422 != 0.0) {
        let assign21770_e16771: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign21770_e16771, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)), ((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)), ((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn13 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn13)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign21770_e16773;
        locals.var_tmf2_dn0 = assign21770_e16773_d_n0;
        locals.var_tmf2_dn2 = assign21770_e16773_d_n2;
        locals.var_tmf2_dn4 = assign21770_e16773_d_n4;
        locals.var_tmf2_dn5 = assign21770_e16773_d_n5;
        locals.var_tmf2_dn6 = assign21770_e16773_d_n6;
        locals.var_tmf2_dn7 = assign21770_e16773_d_n7;
        locals.var_tmf2_dn8 = assign21770_e16773_d_n8;
        locals.var_tmf2_dn9 = assign21770_e16773_d_n9;
        locals.var_tmf2_dn10 = assign21770_e16773_d_n10;
        locals.var_tmf2_dn13 = assign21770_e16773_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign21780_e16779, assign21780_e16779_d_n0, assign21780_e16779_d_n2, assign21780_e16779_d_n4, assign21780_e16779_d_n5, assign21780_e16779_d_n6, assign21780_e16779_d_n7, assign21780_e16779_d_n8, assign21780_e16779_d_n9, assign21780_e16779_d_n10, assign21780_e16779_d_n13,) = {
    if (locals.var_guard422 != 0.0) {
        let assign21780_e16777: f64 = (locals.var_tmf2 * locals.var_tmf1);
        (assign21780_e16777, ((locals.var_tmf2_dn0 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn0)), ((locals.var_tmf2_dn2 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn2)), ((locals.var_tmf2_dn4 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn4)), ((locals.var_tmf2_dn5 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn5)), ((locals.var_tmf2_dn6 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn6)), ((locals.var_tmf2_dn7 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn7)), ((locals.var_tmf2_dn8 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn8)), ((locals.var_tmf2_dn9 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn9)), ((locals.var_tmf2_dn10 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn10)), ((locals.var_tmf2_dn13 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn13)),)
    } else {
        (locals.var_tmf3, locals.var_tmf3_dn0, locals.var_tmf3_dn2, locals.var_tmf3_dn4, locals.var_tmf3_dn5, locals.var_tmf3_dn6, locals.var_tmf3_dn7, locals.var_tmf3_dn8, locals.var_tmf3_dn9, locals.var_tmf3_dn10, locals.var_tmf3_dn13,)
    }
};
        locals.var_tmf3 = assign21780_e16779;
        locals.var_tmf3_dn0 = assign21780_e16779_d_n0;
        locals.var_tmf3_dn2 = assign21780_e16779_d_n2;
        locals.var_tmf3_dn4 = assign21780_e16779_d_n4;
        locals.var_tmf3_dn5 = assign21780_e16779_d_n5;
        locals.var_tmf3_dn6 = assign21780_e16779_d_n6;
        locals.var_tmf3_dn7 = assign21780_e16779_d_n7;
        locals.var_tmf3_dn8 = assign21780_e16779_d_n8;
        locals.var_tmf3_dn9 = assign21780_e16779_d_n9;
        locals.var_tmf3_dn10 = assign21780_e16779_d_n10;
        locals.var_tmf3_dn13 = assign21780_e16779_d_n13;
        locals.var_tmf3_rv = 0.0;

        let (assign21790_e16785, assign21790_e16785_d_n0, assign21790_e16785_d_n2, assign21790_e16785_d_n4, assign21790_e16785_d_n5, assign21790_e16785_d_n6, assign21790_e16785_d_n7, assign21790_e16785_d_n8, assign21790_e16785_d_n9, assign21790_e16785_d_n10, assign21790_e16785_d_n13,) = {
    if (locals.var_guard422 != 0.0) {
        let assign21790_e16783: f64 = (locals.var_tmf2 * locals.var_tmf2);
        (assign21790_e16783, ((locals.var_tmf2_dn0 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn0)), ((locals.var_tmf2_dn2 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn2)), ((locals.var_tmf2_dn4 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn4)), ((locals.var_tmf2_dn5 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn5)), ((locals.var_tmf2_dn6 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn6)), ((locals.var_tmf2_dn7 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn7)), ((locals.var_tmf2_dn8 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn8)), ((locals.var_tmf2_dn9 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn9)), ((locals.var_tmf2_dn10 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn10)), ((locals.var_tmf2_dn13 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn13)),)
    } else {
        (locals.var_tmf4, locals.var_tmf4_dn0, locals.var_tmf4_dn2, locals.var_tmf4_dn4, locals.var_tmf4_dn5, locals.var_tmf4_dn6, locals.var_tmf4_dn7, locals.var_tmf4_dn8, locals.var_tmf4_dn9, locals.var_tmf4_dn10, locals.var_tmf4_dn13,)
    }
};
        locals.var_tmf4 = assign21790_e16785;
        locals.var_tmf4_dn0 = assign21790_e16785_d_n0;
        locals.var_tmf4_dn2 = assign21790_e16785_d_n2;
        locals.var_tmf4_dn4 = assign21790_e16785_d_n4;
        locals.var_tmf4_dn5 = assign21790_e16785_d_n5;
        locals.var_tmf4_dn6 = assign21790_e16785_d_n6;
        locals.var_tmf4_dn7 = assign21790_e16785_d_n7;
        locals.var_tmf4_dn8 = assign21790_e16785_d_n8;
        locals.var_tmf4_dn9 = assign21790_e16785_d_n9;
        locals.var_tmf4_dn10 = assign21790_e16785_d_n10;
        locals.var_tmf4_dn13 = assign21790_e16785_d_n13;
        locals.var_tmf4_rv = 0.0;

        let (assign21800_e16799, assign21800_e16799_d_n0, assign21800_e16799_d_n2, assign21800_e16799_d_n4, assign21800_e16799_d_n5, assign21800_e16799_d_n6, assign21800_e16799_d_n7, assign21800_e16799_d_n8, assign21800_e16799_d_n9, assign21800_e16799_d_n10, assign21800_e16799_d_n13,) = {
    if (locals.var_guard422 != 0.0) {
        let assign21800_e16790: f64 = (1.0 + locals.var_tmf1);
        let assign21800_e16792: f64 = (assign21800_e16790 + locals.var_tmf2);
        let assign21800_e16794: f64 = (assign21800_e16792 + locals.var_tmf3);
        let assign21800_e16796: f64 = (assign21800_e16794 + locals.var_tmf4);
        let assign21800_e16797: f64 = (1.0 / assign21800_e16796);
        (assign21800_e16797, (-((((locals.var_tmf1_dn0 + locals.var_tmf2_dn0) + locals.var_tmf3_dn0) + locals.var_tmf4_dn0) / (assign21800_e16796 * assign21800_e16796))), (-((((locals.var_tmf1_dn2 + locals.var_tmf2_dn2) + locals.var_tmf3_dn2) + locals.var_tmf4_dn2) / (assign21800_e16796 * assign21800_e16796))), (-((((locals.var_tmf1_dn4 + locals.var_tmf2_dn4) + locals.var_tmf3_dn4) + locals.var_tmf4_dn4) / (assign21800_e16796 * assign21800_e16796))), (-((((locals.var_tmf1_dn5 + locals.var_tmf2_dn5) + locals.var_tmf3_dn5) + locals.var_tmf4_dn5) / (assign21800_e16796 * assign21800_e16796))), (-((((locals.var_tmf1_dn6 + locals.var_tmf2_dn6) + locals.var_tmf3_dn6) + locals.var_tmf4_dn6) / (assign21800_e16796 * assign21800_e16796))), (-((((locals.var_tmf1_dn7 + locals.var_tmf2_dn7) + locals.var_tmf3_dn7) + locals.var_tmf4_dn7) / (assign21800_e16796 * assign21800_e16796))), (-((((locals.var_tmf1_dn8 + locals.var_tmf2_dn8) + locals.var_tmf3_dn8) + locals.var_tmf4_dn8) / (assign21800_e16796 * assign21800_e16796))), (-((((locals.var_tmf1_dn9 + locals.var_tmf2_dn9) + locals.var_tmf3_dn9) + locals.var_tmf4_dn9) / (assign21800_e16796 * assign21800_e16796))), (-((((locals.var_tmf1_dn10 + locals.var_tmf2_dn10) + locals.var_tmf3_dn10) + locals.var_tmf4_dn10) / (assign21800_e16796 * assign21800_e16796))), (-((((locals.var_tmf1_dn13 + locals.var_tmf2_dn13) + locals.var_tmf3_dn13) + locals.var_tmf4_dn13) / (assign21800_e16796 * assign21800_e16796))),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn13,)
    }
};
        locals.var_tmf0 = assign21800_e16799;
        locals.var_tmf0_dn0 = assign21800_e16799_d_n0;
        locals.var_tmf0_dn2 = assign21800_e16799_d_n2;
        locals.var_tmf0_dn4 = assign21800_e16799_d_n4;
        locals.var_tmf0_dn5 = assign21800_e16799_d_n5;
        locals.var_tmf0_dn6 = assign21800_e16799_d_n6;
        locals.var_tmf0_dn7 = assign21800_e16799_d_n7;
        locals.var_tmf0_dn8 = assign21800_e16799_d_n8;
        locals.var_tmf0_dn9 = assign21800_e16799_d_n9;
        locals.var_tmf0_dn10 = assign21800_e16799_d_n10;
        locals.var_tmf0_dn13 = assign21800_e16799_d_n13;
        locals.var_tmf0_rv = 0.0;

        let (assign21810_e16820, assign21810_e16820_d_n0, assign21810_e16820_d_n2, assign21810_e16820_d_n4, assign21810_e16820_d_n5, assign21810_e16820_d_n6, assign21810_e16820_d_n7, assign21810_e16820_d_n8, assign21810_e16820_d_n9, assign21810_e16820_d_n10, assign21810_e16820_d_n13,) = {
    if (locals.var_guard422 != 0.0) {
        let assign21810_e16804: f64 = (2.0 * locals.var_tmf1);
        let assign21810_e16805: f64 = (1.0 + assign21810_e16804);
        let assign21810_e16808: f64 = (3.0 * locals.var_tmf2);
        let assign21810_e16809: f64 = (assign21810_e16805 + assign21810_e16808);
        let assign21810_e16812: f64 = (4.0 * locals.var_tmf3);
        let assign21810_e16813: f64 = (assign21810_e16809 + assign21810_e16812);
        let assign21810_e16814: f64 = (-assign21810_e16813);
        let assign21810_e16816: f64 = (assign21810_e16814 * locals.var_tmf0);
        let assign21810_e16818: f64 = (assign21810_e16816 * locals.var_tmf0);
        (assign21810_e16818, (((((-(((2.0 * locals.var_tmf1_dn0) + (3.0 * locals.var_tmf2_dn0)) + (4.0 * locals.var_tmf3_dn0))) * locals.var_tmf0) + (assign21810_e16814 * locals.var_tmf0_dn0)) * locals.var_tmf0) + (assign21810_e16816 * locals.var_tmf0_dn0)), (((((-(((2.0 * locals.var_tmf1_dn2) + (3.0 * locals.var_tmf2_dn2)) + (4.0 * locals.var_tmf3_dn2))) * locals.var_tmf0) + (assign21810_e16814 * locals.var_tmf0_dn2)) * locals.var_tmf0) + (assign21810_e16816 * locals.var_tmf0_dn2)), (((((-(((2.0 * locals.var_tmf1_dn4) + (3.0 * locals.var_tmf2_dn4)) + (4.0 * locals.var_tmf3_dn4))) * locals.var_tmf0) + (assign21810_e16814 * locals.var_tmf0_dn4)) * locals.var_tmf0) + (assign21810_e16816 * locals.var_tmf0_dn4)), (((((-(((2.0 * locals.var_tmf1_dn5) + (3.0 * locals.var_tmf2_dn5)) + (4.0 * locals.var_tmf3_dn5))) * locals.var_tmf0) + (assign21810_e16814 * locals.var_tmf0_dn5)) * locals.var_tmf0) + (assign21810_e16816 * locals.var_tmf0_dn5)), (((((-(((2.0 * locals.var_tmf1_dn6) + (3.0 * locals.var_tmf2_dn6)) + (4.0 * locals.var_tmf3_dn6))) * locals.var_tmf0) + (assign21810_e16814 * locals.var_tmf0_dn6)) * locals.var_tmf0) + (assign21810_e16816 * locals.var_tmf0_dn6)), (((((-(((2.0 * locals.var_tmf1_dn7) + (3.0 * locals.var_tmf2_dn7)) + (4.0 * locals.var_tmf3_dn7))) * locals.var_tmf0) + (assign21810_e16814 * locals.var_tmf0_dn7)) * locals.var_tmf0) + (assign21810_e16816 * locals.var_tmf0_dn7)), (((((-(((2.0 * locals.var_tmf1_dn8) + (3.0 * locals.var_tmf2_dn8)) + (4.0 * locals.var_tmf3_dn8))) * locals.var_tmf0) + (assign21810_e16814 * locals.var_tmf0_dn8)) * locals.var_tmf0) + (assign21810_e16816 * locals.var_tmf0_dn8)), (((((-(((2.0 * locals.var_tmf1_dn9) + (3.0 * locals.var_tmf2_dn9)) + (4.0 * locals.var_tmf3_dn9))) * locals.var_tmf0) + (assign21810_e16814 * locals.var_tmf0_dn9)) * locals.var_tmf0) + (assign21810_e16816 * locals.var_tmf0_dn9)), (((((-(((2.0 * locals.var_tmf1_dn10) + (3.0 * locals.var_tmf2_dn10)) + (4.0 * locals.var_tmf3_dn10))) * locals.var_tmf0) + (assign21810_e16814 * locals.var_tmf0_dn10)) * locals.var_tmf0) + (assign21810_e16816 * locals.var_tmf0_dn10)), (((((-(((2.0 * locals.var_tmf1_dn13) + (3.0 * locals.var_tmf2_dn13)) + (4.0 * locals.var_tmf3_dn13))) * locals.var_tmf0) + (assign21810_e16814 * locals.var_tmf0_dn13)) * locals.var_tmf0) + (assign21810_e16816 * locals.var_tmf0_dn13)),)
    } else {
        (locals.var_vbscldvbs, locals.var_vbscldvbs_dn0, locals.var_vbscldvbs_dn2, locals.var_vbscldvbs_dn4, locals.var_vbscldvbs_dn5, locals.var_vbscldvbs_dn6, locals.var_vbscldvbs_dn7, locals.var_vbscldvbs_dn8, locals.var_vbscldvbs_dn9, locals.var_vbscldvbs_dn10, locals.var_vbscldvbs_dn13,)
    }
};
        locals.var_vbscldvbs = assign21810_e16820;
        locals.var_vbscldvbs_dn0 = assign21810_e16820_d_n0;
        locals.var_vbscldvbs_dn2 = assign21810_e16820_d_n2;
        locals.var_vbscldvbs_dn4 = assign21810_e16820_d_n4;
        locals.var_vbscldvbs_dn5 = assign21810_e16820_d_n5;
        locals.var_vbscldvbs_dn6 = assign21810_e16820_d_n6;
        locals.var_vbscldvbs_dn7 = assign21810_e16820_d_n7;
        locals.var_vbscldvbs_dn8 = assign21810_e16820_d_n8;
        locals.var_vbscldvbs_dn9 = assign21810_e16820_d_n9;
        locals.var_vbscldvbs_dn10 = assign21810_e16820_d_n10;
        locals.var_vbscldvbs_dn13 = assign21810_e16820_d_n13;
        locals.var_vbscldvbs_rv = 0.0;

        let (assign21820_e16828, assign21820_e16828_d_n0, assign21820_e16828_d_n2, assign21820_e16828_d_n4, assign21820_e16828_d_n5, assign21820_e16828_d_n6, assign21820_e16828_d_n7, assign21820_e16828_d_n8, assign21820_e16828_d_n9, assign21820_e16828_d_n10, assign21820_e16828_d_n13,) = {
    if (locals.var_guard422 != 0.0) {
        let assign21820_e16825: f64 = (1.0 - locals.var_tmf0);
        let assign21820_e16826: f64 = (locals.var_t2 * assign21820_e16825);
        (assign21820_e16826, ((locals.var_t2_dn0 * assign21820_e16825) + (locals.var_t2 * (-locals.var_tmf0_dn0))), ((locals.var_t2_dn2 * assign21820_e16825) + (locals.var_t2 * (-locals.var_tmf0_dn2))), ((locals.var_t2_dn4 * assign21820_e16825) + (locals.var_t2 * (-locals.var_tmf0_dn4))), ((locals.var_t2_dn5 * assign21820_e16825) + (locals.var_t2 * (-locals.var_tmf0_dn5))), ((locals.var_t2_dn6 * assign21820_e16825) + (locals.var_t2 * (-locals.var_tmf0_dn6))), ((locals.var_t2_dn7 * assign21820_e16825) + (locals.var_t2 * (-locals.var_tmf0_dn7))), ((locals.var_t2_dn8 * assign21820_e16825) + (locals.var_t2 * (-locals.var_tmf0_dn8))), ((locals.var_t2_dn9 * assign21820_e16825) + (locals.var_t2 * (-locals.var_tmf0_dn9))), ((locals.var_t2_dn10 * assign21820_e16825) + (locals.var_t2 * (-locals.var_tmf0_dn10))), ((locals.var_t2_dn13 * assign21820_e16825) + (locals.var_t2 * (-locals.var_tmf0_dn13))),)
    } else {
        (locals.var_ty, locals.var_ty_dn0, locals.var_ty_dn2, locals.var_ty_dn4, locals.var_ty_dn5, locals.var_ty_dn6, locals.var_ty_dn7, locals.var_ty_dn8, locals.var_ty_dn9, locals.var_ty_dn10, locals.var_ty_dn13,)
    }
};
        locals.var_ty = assign21820_e16828;
        locals.var_ty_dn0 = assign21820_e16828_d_n0;
        locals.var_ty_dn2 = assign21820_e16828_d_n2;
        locals.var_ty_dn4 = assign21820_e16828_d_n4;
        locals.var_ty_dn5 = assign21820_e16828_d_n5;
        locals.var_ty_dn6 = assign21820_e16828_d_n6;
        locals.var_ty_dn7 = assign21820_e16828_d_n7;
        locals.var_ty_dn8 = assign21820_e16828_d_n8;
        locals.var_ty_dn9 = assign21820_e16828_d_n9;
        locals.var_ty_dn10 = assign21820_e16828_d_n10;
        locals.var_ty_dn13 = assign21820_e16828_d_n13;
        locals.var_ty_rv = 0.0;

        let (assign21830_e16838, assign21830_e16838_d_n0, assign21830_e16838_d_n2, assign21830_e16838_d_n4, assign21830_e16838_d_n5, assign21830_e16838_d_n6, assign21830_e16838_d_n7, assign21830_e16838_d_n8, assign21830_e16838_d_n9, assign21830_e16838_d_n10, assign21830_e16838_d_n13,) = {
    if (locals.var_guard422 != 0.0) {
        let assign21830_e16832: f64 = (1.0 - locals.var_tmf0);
        let assign21830_e16835: f64 = (locals.var_tmf1 * locals.var_vbscldvbs);
        let assign21830_e16836: f64 = (assign21830_e16832 + assign21830_e16835);
        (assign21830_e16836, ((-locals.var_tmf0_dn0) + ((locals.var_tmf1_dn0 * locals.var_vbscldvbs) + (locals.var_tmf1 * locals.var_vbscldvbs_dn0))), ((-locals.var_tmf0_dn2) + ((locals.var_tmf1_dn2 * locals.var_vbscldvbs) + (locals.var_tmf1 * locals.var_vbscldvbs_dn2))), ((-locals.var_tmf0_dn4) + ((locals.var_tmf1_dn4 * locals.var_vbscldvbs) + (locals.var_tmf1 * locals.var_vbscldvbs_dn4))), ((-locals.var_tmf0_dn5) + ((locals.var_tmf1_dn5 * locals.var_vbscldvbs) + (locals.var_tmf1 * locals.var_vbscldvbs_dn5))), ((-locals.var_tmf0_dn6) + ((locals.var_tmf1_dn6 * locals.var_vbscldvbs) + (locals.var_tmf1 * locals.var_vbscldvbs_dn6))), ((-locals.var_tmf0_dn7) + ((locals.var_tmf1_dn7 * locals.var_vbscldvbs) + (locals.var_tmf1 * locals.var_vbscldvbs_dn7))), ((-locals.var_tmf0_dn8) + ((locals.var_tmf1_dn8 * locals.var_vbscldvbs) + (locals.var_tmf1 * locals.var_vbscldvbs_dn8))), ((-locals.var_tmf0_dn9) + ((locals.var_tmf1_dn9 * locals.var_vbscldvbs) + (locals.var_tmf1 * locals.var_vbscldvbs_dn9))), ((-locals.var_tmf0_dn10) + ((locals.var_tmf1_dn10 * locals.var_vbscldvbs) + (locals.var_tmf1 * locals.var_vbscldvbs_dn10))), ((-locals.var_tmf0_dn13) + ((locals.var_tmf1_dn13 * locals.var_vbscldvbs) + (locals.var_tmf1 * locals.var_vbscldvbs_dn13))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign21830_e16838;
        locals.var_t0_dn0 = assign21830_e16838_d_n0;
        locals.var_t0_dn2 = assign21830_e16838_d_n2;
        locals.var_t0_dn4 = assign21830_e16838_d_n4;
        locals.var_t0_dn5 = assign21830_e16838_d_n5;
        locals.var_t0_dn6 = assign21830_e16838_d_n6;
        locals.var_t0_dn7 = assign21830_e16838_d_n7;
        locals.var_t0_dn8 = assign21830_e16838_d_n8;
        locals.var_t0_dn9 = assign21830_e16838_d_n9;
        locals.var_t0_dn10 = assign21830_e16838_d_n10;
        locals.var_t0_dn13 = assign21830_e16838_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign21840_e16843, assign21840_e16843_d_n0, assign21840_e16843_d_n2, assign21840_e16843_d_n4, assign21840_e16843_d_n5, assign21840_e16843_d_n6, assign21840_e16843_d_n7, assign21840_e16843_d_n8, assign21840_e16843_d_n9, assign21840_e16843_d_n10, assign21840_e16843_d_n13,) = {
    if (locals.var_guard422 != 0.0) {
        let assign21840_e16841: f64 = (-locals.var_vbscldvbs);
        (assign21840_e16841, (-locals.var_vbscldvbs_dn0), (-locals.var_vbscldvbs_dn2), (-locals.var_vbscldvbs_dn4), (-locals.var_vbscldvbs_dn5), (-locals.var_vbscldvbs_dn6), (-locals.var_vbscldvbs_dn7), (-locals.var_vbscldvbs_dn8), (-locals.var_vbscldvbs_dn9), (-locals.var_vbscldvbs_dn10), (-locals.var_vbscldvbs_dn13),)
    } else {
        (locals.var_vbscldvbs, locals.var_vbscldvbs_dn0, locals.var_vbscldvbs_dn2, locals.var_vbscldvbs_dn4, locals.var_vbscldvbs_dn5, locals.var_vbscldvbs_dn6, locals.var_vbscldvbs_dn7, locals.var_vbscldvbs_dn8, locals.var_vbscldvbs_dn9, locals.var_vbscldvbs_dn10, locals.var_vbscldvbs_dn13,)
    }
};
        locals.var_vbscldvbs = assign21840_e16843;
        locals.var_vbscldvbs_dn0 = assign21840_e16843_d_n0;
        locals.var_vbscldvbs_dn2 = assign21840_e16843_d_n2;
        locals.var_vbscldvbs_dn4 = assign21840_e16843_d_n4;
        locals.var_vbscldvbs_dn5 = assign21840_e16843_d_n5;
        locals.var_vbscldvbs_dn6 = assign21840_e16843_d_n6;
        locals.var_vbscldvbs_dn7 = assign21840_e16843_d_n7;
        locals.var_vbscldvbs_dn8 = assign21840_e16843_d_n8;
        locals.var_vbscldvbs_dn9 = assign21840_e16843_d_n9;
        locals.var_vbscldvbs_dn10 = assign21840_e16843_d_n10;
        locals.var_vbscldvbs_dn13 = assign21840_e16843_d_n13;
        locals.var_vbscldvbs_rv = 0.0;

        let (assign21850_e16849, assign21850_e16849_d_n0, assign21850_e16849_d_n2, assign21850_e16849_d_n4, assign21850_e16849_d_n5, assign21850_e16849_d_n6, assign21850_e16849_d_n7, assign21850_e16849_d_n8, assign21850_e16849_d_n9, assign21850_e16849_d_n10, assign21850_e16849_d_n13,) = {
    if (locals.var_guard422 != 0.0) {
        let assign21850_e16847: f64 = (locals.var_vbs_bnd + locals.var_ty);
        (assign21850_e16847, (locals.var_vbs_bnd_dn0 + locals.var_ty_dn0), (locals.var_vbs_bnd_dn2 + locals.var_ty_dn2), (locals.var_vbs_bnd_dn4 + locals.var_ty_dn4), (locals.var_vbs_bnd_dn5 + locals.var_ty_dn5), (locals.var_vbs_bnd_dn6 + locals.var_ty_dn6), (locals.var_vbs_bnd_dn7 + locals.var_ty_dn7), (locals.var_vbs_bnd_dn8 + locals.var_ty_dn8), (locals.var_vbs_bnd_dn9 + locals.var_ty_dn9), (locals.var_vbs_bnd_dn10 + locals.var_ty_dn10), (locals.var_vbs_bnd_dn13 + locals.var_ty_dn13),)
    } else {
        (locals.var_vbscl, locals.var_vbscl_dn0, locals.var_vbscl_dn2, locals.var_vbscl_dn4, locals.var_vbscl_dn5, locals.var_vbscl_dn6, locals.var_vbscl_dn7, locals.var_vbscl_dn8, locals.var_vbscl_dn9, locals.var_vbscl_dn10, locals.var_vbscl_dn13,)
    }
};
        locals.var_vbscl = assign21850_e16849;
        locals.var_vbscl_dn0 = assign21850_e16849_d_n0;
        locals.var_vbscl_dn2 = assign21850_e16849_d_n2;
        locals.var_vbscl_dn4 = assign21850_e16849_d_n4;
        locals.var_vbscl_dn5 = assign21850_e16849_d_n5;
        locals.var_vbscl_dn6 = assign21850_e16849_d_n6;
        locals.var_vbscl_dn7 = assign21850_e16849_d_n7;
        locals.var_vbscl_dn8 = assign21850_e16849_d_n8;
        locals.var_vbscl_dn9 = assign21850_e16849_d_n9;
        locals.var_vbscl_dn10 = assign21850_e16849_d_n10;
        locals.var_vbscl_dn13 = assign21850_e16849_d_n13;
        locals.var_vbscl_rv = 0.0;

        let (assign21860_e16855, assign21860_e16855_d_n0, assign21860_e16855_d_n2, assign21860_e16855_d_n4, assign21860_e16855_d_n5, assign21860_e16855_d_n6, assign21860_e16855_d_n7, assign21860_e16855_d_n8, assign21860_e16855_d_n9, assign21860_e16855_d_n10, assign21860_e16855_d_n13,) = {
    if (locals.var_guard422 != 0.0) {
        let assign21860_e16853: f64 = (1.0 / locals.var_t2);
        (assign21860_e16853, (-(locals.var_t2_dn0 / (locals.var_t2 * locals.var_t2))), (-(locals.var_t2_dn2 / (locals.var_t2 * locals.var_t2))), (-(locals.var_t2_dn4 / (locals.var_t2 * locals.var_t2))), (-(locals.var_t2_dn5 / (locals.var_t2 * locals.var_t2))), (-(locals.var_t2_dn6 / (locals.var_t2 * locals.var_t2))), (-(locals.var_t2_dn7 / (locals.var_t2 * locals.var_t2))), (-(locals.var_t2_dn8 / (locals.var_t2 * locals.var_t2))), (-(locals.var_t2_dn9 / (locals.var_t2 * locals.var_t2))), (-(locals.var_t2_dn10 / (locals.var_t2 * locals.var_t2))), (-(locals.var_t2_dn13 / (locals.var_t2 * locals.var_t2))),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn13,)
    }
};
        locals.var_t3 = assign21860_e16855;
        locals.var_t3_dn0 = assign21860_e16855_d_n0;
        locals.var_t3_dn2 = assign21860_e16855_d_n2;
        locals.var_t3_dn4 = assign21860_e16855_d_n4;
        locals.var_t3_dn5 = assign21860_e16855_d_n5;
        locals.var_t3_dn6 = assign21860_e16855_d_n6;
        locals.var_t3_dn7 = assign21860_e16855_d_n7;
        locals.var_t3_dn8 = assign21860_e16855_d_n8;
        locals.var_t3_dn9 = assign21860_e16855_d_n9;
        locals.var_t3_dn10 = assign21860_e16855_d_n10;
        locals.var_t3_dn13 = assign21860_e16855_d_n13;
        locals.var_t3_rv = 0.0;

        let (assign21870_e16861, assign21870_e16861_d_n0, assign21870_e16861_d_n2, assign21870_e16861_d_n4, assign21870_e16861_d_n5, assign21870_e16861_d_n6, assign21870_e16861_d_n7, assign21870_e16861_d_n8, assign21870_e16861_d_n9, assign21870_e16861_d_n10, assign21870_e16861_d_n13,) = {
    if (locals.var_guard422 != 0.0) {
        let assign21870_e16859: f64 = (locals.var_t1 * locals.var_t3);
        (assign21870_e16859, ((locals.var_t1_dn0 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn0)), ((locals.var_t1_dn2 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn2)), ((locals.var_t1_dn4 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn4)), ((locals.var_t1_dn5 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn5)), ((locals.var_t1_dn6 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn6)), ((locals.var_t1_dn7 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn7)), ((locals.var_t1_dn8 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn8)), ((locals.var_t1_dn9 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn9)), ((locals.var_t1_dn10 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn10)), ((locals.var_t1_dn13 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn13)),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn13,)
    }
};
        locals.var_t4 = assign21870_e16861;
        locals.var_t4_dn0 = assign21870_e16861_d_n0;
        locals.var_t4_dn2 = assign21870_e16861_d_n2;
        locals.var_t4_dn4 = assign21870_e16861_d_n4;
        locals.var_t4_dn5 = assign21870_e16861_d_n5;
        locals.var_t4_dn6 = assign21870_e16861_d_n6;
        locals.var_t4_dn7 = assign21870_e16861_d_n7;
        locals.var_t4_dn8 = assign21870_e16861_d_n8;
        locals.var_t4_dn9 = assign21870_e16861_d_n9;
        locals.var_t4_dn10 = assign21870_e16861_d_n10;
        locals.var_t4_dn13 = assign21870_e16861_d_n13;
        locals.var_t4_rv = 0.0;

        let (assign21880_e16867, assign21880_e16867_d_n0, assign21880_e16867_d_n2, assign21880_e16867_d_n4, assign21880_e16867_d_n5, assign21880_e16867_d_n6, assign21880_e16867_d_n7, assign21880_e16867_d_n8, assign21880_e16867_d_n9, assign21880_e16867_d_n10, assign21880_e16867_d_n13,) = {
    if (locals.var_guard422 != 0.0) {
        let assign21880_e16865: f64 = (locals.var_t4 * locals.var_t4);
        (assign21880_e16865, ((locals.var_t4_dn0 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn0)), ((locals.var_t4_dn2 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn2)), ((locals.var_t4_dn4 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn4)), ((locals.var_t4_dn5 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn5)), ((locals.var_t4_dn6 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn6)), ((locals.var_t4_dn7 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn7)), ((locals.var_t4_dn8 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn8)), ((locals.var_t4_dn9 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn9)), ((locals.var_t4_dn10 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn10)), ((locals.var_t4_dn13 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn13)),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn13,)
    }
};
        locals.var_t5 = assign21880_e16867;
        locals.var_t5_dn0 = assign21880_e16867_d_n0;
        locals.var_t5_dn2 = assign21880_e16867_d_n2;
        locals.var_t5_dn4 = assign21880_e16867_d_n4;
        locals.var_t5_dn5 = assign21880_e16867_d_n5;
        locals.var_t5_dn6 = assign21880_e16867_d_n6;
        locals.var_t5_dn7 = assign21880_e16867_d_n7;
        locals.var_t5_dn8 = assign21880_e16867_d_n8;
        locals.var_t5_dn9 = assign21880_e16867_d_n9;
        locals.var_t5_dn10 = assign21880_e16867_d_n10;
        locals.var_t5_dn13 = assign21880_e16867_d_n13;
        locals.var_t5_rv = 0.0;

        let (assign21890_e16881, assign21890_e16881_d_n0, assign21890_e16881_d_n2, assign21890_e16881_d_n4, assign21890_e16881_d_n5, assign21890_e16881_d_n6, assign21890_e16881_d_n7, assign21890_e16881_d_n8, assign21890_e16881_d_n9, assign21890_e16881_d_n10, assign21890_e16881_d_n13,) = {
    if (locals.var_guard422 != 0.0) {
        let assign21890_e16871: f64 = (1.0 + locals.var_t4);
        let assign21890_e16875: f64 = (1.0 + locals.var_t4);
        let assign21890_e16877: f64 = (assign21890_e16875 + locals.var_t5);
        let assign21890_e16878: f64 = (locals.var_t5 * assign21890_e16877);
        let assign21890_e16879: f64 = (assign21890_e16871 + assign21890_e16878);
        (assign21890_e16879, (locals.var_t4_dn0 + ((locals.var_t5_dn0 * assign21890_e16877) + (locals.var_t5 * (locals.var_t4_dn0 + locals.var_t5_dn0)))), (locals.var_t4_dn2 + ((locals.var_t5_dn2 * assign21890_e16877) + (locals.var_t5 * (locals.var_t4_dn2 + locals.var_t5_dn2)))), (locals.var_t4_dn4 + ((locals.var_t5_dn4 * assign21890_e16877) + (locals.var_t5 * (locals.var_t4_dn4 + locals.var_t5_dn4)))), (locals.var_t4_dn5 + ((locals.var_t5_dn5 * assign21890_e16877) + (locals.var_t5 * (locals.var_t4_dn5 + locals.var_t5_dn5)))), (locals.var_t4_dn6 + ((locals.var_t5_dn6 * assign21890_e16877) + (locals.var_t5 * (locals.var_t4_dn6 + locals.var_t5_dn6)))), (locals.var_t4_dn7 + ((locals.var_t5_dn7 * assign21890_e16877) + (locals.var_t5 * (locals.var_t4_dn7 + locals.var_t5_dn7)))), (locals.var_t4_dn8 + ((locals.var_t5_dn8 * assign21890_e16877) + (locals.var_t5 * (locals.var_t4_dn8 + locals.var_t5_dn8)))), (locals.var_t4_dn9 + ((locals.var_t5_dn9 * assign21890_e16877) + (locals.var_t5 * (locals.var_t4_dn9 + locals.var_t5_dn9)))), (locals.var_t4_dn10 + ((locals.var_t5_dn10 * assign21890_e16877) + (locals.var_t5 * (locals.var_t4_dn10 + locals.var_t5_dn10)))), (locals.var_t4_dn13 + ((locals.var_t5_dn13 * assign21890_e16877) + (locals.var_t5 * (locals.var_t4_dn13 + locals.var_t5_dn13)))),)
    } else {
        (locals.var_t7, locals.var_t7_dn0, locals.var_t7_dn2, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn13,)
    }
};
        locals.var_t7 = assign21890_e16881;
        locals.var_t7_dn0 = assign21890_e16881_d_n0;
        locals.var_t7_dn2 = assign21890_e16881_d_n2;
        locals.var_t7_dn4 = assign21890_e16881_d_n4;
        locals.var_t7_dn5 = assign21890_e16881_d_n5;
        locals.var_t7_dn6 = assign21890_e16881_d_n6;
        locals.var_t7_dn7 = assign21890_e16881_d_n7;
        locals.var_t7_dn8 = assign21890_e16881_d_n8;
        locals.var_t7_dn9 = assign21890_e16881_d_n9;
        locals.var_t7_dn10 = assign21890_e16881_d_n10;
        locals.var_t7_dn13 = assign21890_e16881_d_n13;
        locals.var_t7_rv = 0.0;

        let (assign21900_e16903, assign21900_e16903_d_n0, assign21900_e16903_d_n2, assign21900_e16903_d_n4, assign21900_e16903_d_n5, assign21900_e16903_d_n6, assign21900_e16903_d_n7, assign21900_e16903_d_n8, assign21900_e16903_d_n9, assign21900_e16903_d_n10, assign21900_e16903_d_n13,) = {
    if (locals.var_guard422 != 0.0) {
        let assign21900_e16886: f64 = (2.0 * locals.var_t4);
        let assign21900_e16887: f64 = (1.0 + assign21900_e16886);
        let assign21900_e16890: f64 = (3.0 * locals.var_t5);
        let assign21900_e16891: f64 = (assign21900_e16887 + assign21900_e16890);
        let assign21900_e16894: f64 = (4.0 * locals.var_t4);
        let assign21900_e16896: f64 = (assign21900_e16894 * locals.var_t5);
        let assign21900_e16897: f64 = (assign21900_e16891 + assign21900_e16896);
        let assign21900_e16900: f64 = (locals.var_t7 * locals.var_t7);
        let assign21900_e16901: f64 = (assign21900_e16897 / assign21900_e16900);
        (assign21900_e16901, ((((((2.0 * locals.var_t4_dn0) + (3.0 * locals.var_t5_dn0)) + (((4.0 * locals.var_t4_dn0) * locals.var_t5) + (assign21900_e16894 * locals.var_t5_dn0))) * assign21900_e16900) - (assign21900_e16897 * ((locals.var_t7_dn0 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn0)))) / (assign21900_e16900 * assign21900_e16900)), ((((((2.0 * locals.var_t4_dn2) + (3.0 * locals.var_t5_dn2)) + (((4.0 * locals.var_t4_dn2) * locals.var_t5) + (assign21900_e16894 * locals.var_t5_dn2))) * assign21900_e16900) - (assign21900_e16897 * ((locals.var_t7_dn2 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn2)))) / (assign21900_e16900 * assign21900_e16900)), ((((((2.0 * locals.var_t4_dn4) + (3.0 * locals.var_t5_dn4)) + (((4.0 * locals.var_t4_dn4) * locals.var_t5) + (assign21900_e16894 * locals.var_t5_dn4))) * assign21900_e16900) - (assign21900_e16897 * ((locals.var_t7_dn4 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn4)))) / (assign21900_e16900 * assign21900_e16900)), ((((((2.0 * locals.var_t4_dn5) + (3.0 * locals.var_t5_dn5)) + (((4.0 * locals.var_t4_dn5) * locals.var_t5) + (assign21900_e16894 * locals.var_t5_dn5))) * assign21900_e16900) - (assign21900_e16897 * ((locals.var_t7_dn5 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn5)))) / (assign21900_e16900 * assign21900_e16900)), ((((((2.0 * locals.var_t4_dn6) + (3.0 * locals.var_t5_dn6)) + (((4.0 * locals.var_t4_dn6) * locals.var_t5) + (assign21900_e16894 * locals.var_t5_dn6))) * assign21900_e16900) - (assign21900_e16897 * ((locals.var_t7_dn6 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn6)))) / (assign21900_e16900 * assign21900_e16900)), ((((((2.0 * locals.var_t4_dn7) + (3.0 * locals.var_t5_dn7)) + (((4.0 * locals.var_t4_dn7) * locals.var_t5) + (assign21900_e16894 * locals.var_t5_dn7))) * assign21900_e16900) - (assign21900_e16897 * ((locals.var_t7_dn7 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn7)))) / (assign21900_e16900 * assign21900_e16900)), ((((((2.0 * locals.var_t4_dn8) + (3.0 * locals.var_t5_dn8)) + (((4.0 * locals.var_t4_dn8) * locals.var_t5) + (assign21900_e16894 * locals.var_t5_dn8))) * assign21900_e16900) - (assign21900_e16897 * ((locals.var_t7_dn8 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn8)))) / (assign21900_e16900 * assign21900_e16900)), ((((((2.0 * locals.var_t4_dn9) + (3.0 * locals.var_t5_dn9)) + (((4.0 * locals.var_t4_dn9) * locals.var_t5) + (assign21900_e16894 * locals.var_t5_dn9))) * assign21900_e16900) - (assign21900_e16897 * ((locals.var_t7_dn9 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn9)))) / (assign21900_e16900 * assign21900_e16900)), ((((((2.0 * locals.var_t4_dn10) + (3.0 * locals.var_t5_dn10)) + (((4.0 * locals.var_t4_dn10) * locals.var_t5) + (assign21900_e16894 * locals.var_t5_dn10))) * assign21900_e16900) - (assign21900_e16897 * ((locals.var_t7_dn10 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn10)))) / (assign21900_e16900 * assign21900_e16900)), ((((((2.0 * locals.var_t4_dn13) + (3.0 * locals.var_t5_dn13)) + (((4.0 * locals.var_t4_dn13) * locals.var_t5) + (assign21900_e16894 * locals.var_t5_dn13))) * assign21900_e16900) - (assign21900_e16897 * ((locals.var_t7_dn13 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn13)))) / (assign21900_e16900 * assign21900_e16900)),)
    } else {
        (locals.var_vbscldvbs, locals.var_vbscldvbs_dn0, locals.var_vbscldvbs_dn2, locals.var_vbscldvbs_dn4, locals.var_vbscldvbs_dn5, locals.var_vbscldvbs_dn6, locals.var_vbscldvbs_dn7, locals.var_vbscldvbs_dn8, locals.var_vbscldvbs_dn9, locals.var_vbscldvbs_dn10, locals.var_vbscldvbs_dn13,)
    }
};
        locals.var_vbscldvbs = assign21900_e16903;
        locals.var_vbscldvbs_dn0 = assign21900_e16903_d_n0;
        locals.var_vbscldvbs_dn2 = assign21900_e16903_d_n2;
        locals.var_vbscldvbs_dn4 = assign21900_e16903_d_n4;
        locals.var_vbscldvbs_dn5 = assign21900_e16903_d_n5;
        locals.var_vbscldvbs_dn6 = assign21900_e16903_d_n6;
        locals.var_vbscldvbs_dn7 = assign21900_e16903_d_n7;
        locals.var_vbscldvbs_dn8 = assign21900_e16903_d_n8;
        locals.var_vbscldvbs_dn9 = assign21900_e16903_d_n9;
        locals.var_vbscldvbs_dn10 = assign21900_e16903_d_n10;
        locals.var_vbscldvbs_dn13 = assign21900_e16903_d_n13;
        locals.var_vbscldvbs_rv = 0.0;

        let (assign21910_e16908, assign21910_e16908_d_n0, assign21910_e16908_d_n2, assign21910_e16908_d_n4, assign21910_e16908_d_n5, assign21910_e16908_d_n6, assign21910_e16908_d_n7, assign21910_e16908_d_n8, assign21910_e16908_d_n9, assign21910_e16908_d_n10, assign21910_e16908_d_n13,) = {
    if (locals.var_guard422 == 0.0) {
        (locals.var_vbs, 0.0, 0.0, 0.0, locals.var_vbs_dn5, 0.0, locals.var_vbs_dn7, locals.var_vbs_dn8, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vbscl, locals.var_vbscl_dn0, locals.var_vbscl_dn2, locals.var_vbscl_dn4, locals.var_vbscl_dn5, locals.var_vbscl_dn6, locals.var_vbscl_dn7, locals.var_vbscl_dn8, locals.var_vbscl_dn9, locals.var_vbscl_dn10, locals.var_vbscl_dn13,)
    }
};
        locals.var_vbscl = assign21910_e16908;
        locals.var_vbscl_dn0 = assign21910_e16908_d_n0;
        locals.var_vbscl_dn2 = assign21910_e16908_d_n2;
        locals.var_vbscl_dn4 = assign21910_e16908_d_n4;
        locals.var_vbscl_dn5 = assign21910_e16908_d_n5;
        locals.var_vbscl_dn6 = assign21910_e16908_d_n6;
        locals.var_vbscl_dn7 = assign21910_e16908_d_n7;
        locals.var_vbscl_dn8 = assign21910_e16908_d_n8;
        locals.var_vbscl_dn9 = assign21910_e16908_d_n9;
        locals.var_vbscl_dn10 = assign21910_e16908_d_n10;
        locals.var_vbscl_dn13 = assign21910_e16908_d_n13;
        locals.var_vbscl_rv = 0.0;

        let (assign21920_e16913, assign21920_e16913_d_n0, assign21920_e16913_d_n2, assign21920_e16913_d_n4, assign21920_e16913_d_n5, assign21920_e16913_d_n6, assign21920_e16913_d_n7, assign21920_e16913_d_n8, assign21920_e16913_d_n9, assign21920_e16913_d_n10, assign21920_e16913_d_n13,) = {
    if (locals.var_guard422 == 0.0) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vbscldvbs, locals.var_vbscldvbs_dn0, locals.var_vbscldvbs_dn2, locals.var_vbscldvbs_dn4, locals.var_vbscldvbs_dn5, locals.var_vbscldvbs_dn6, locals.var_vbscldvbs_dn7, locals.var_vbscldvbs_dn8, locals.var_vbscldvbs_dn9, locals.var_vbscldvbs_dn10, locals.var_vbscldvbs_dn13,)
    }
};
        locals.var_vbscldvbs = assign21920_e16913;
        locals.var_vbscldvbs_dn0 = assign21920_e16913_d_n0;
        locals.var_vbscldvbs_dn2 = assign21920_e16913_d_n2;
        locals.var_vbscldvbs_dn4 = assign21920_e16913_d_n4;
        locals.var_vbscldvbs_dn5 = assign21920_e16913_d_n5;
        locals.var_vbscldvbs_dn6 = assign21920_e16913_d_n6;
        locals.var_vbscldvbs_dn7 = assign21920_e16913_d_n7;
        locals.var_vbscldvbs_dn8 = assign21920_e16913_d_n8;
        locals.var_vbscldvbs_dn9 = assign21920_e16913_d_n9;
        locals.var_vbscldvbs_dn10 = assign21920_e16913_d_n10;
        locals.var_vbscldvbs_dn13 = assign21920_e16913_d_n13;
        locals.var_vbscldvbs_rv = 0.0;

        let assign21930_e16916: f64 = (locals.var_vbscldvbs * locals.var_vds);
        let assign21930_e16918: f64 = (assign21930_e16916 / 2.0);
        locals.var_t1 = assign21930_e16918;
        locals.var_t1_dn0 = (((locals.var_vbscldvbs_dn0 * locals.var_vds) + (locals.var_vbscldvbs * locals.var_vds_dn0)) / 2.0);
        locals.var_t1_dn2 = (((locals.var_vbscldvbs_dn2 * locals.var_vds) + (locals.var_vbscldvbs * locals.var_vds_dn2)) / 2.0);
        locals.var_t1_dn4 = (((locals.var_vbscldvbs_dn4 * locals.var_vds) + (locals.var_vbscldvbs * locals.var_vds_dn4)) / 2.0);
        locals.var_t1_dn5 = (((locals.var_vbscldvbs_dn5 * locals.var_vds) + (locals.var_vbscldvbs * locals.var_vds_dn5)) / 2.0);
        locals.var_t1_dn6 = (((locals.var_vbscldvbs_dn6 * locals.var_vds) + (locals.var_vbscldvbs * locals.var_vds_dn6)) / 2.0);
        locals.var_t1_dn7 = (((locals.var_vbscldvbs_dn7 * locals.var_vds) + (locals.var_vbscldvbs * locals.var_vds_dn7)) / 2.0);
        locals.var_t1_dn8 = (((locals.var_vbscldvbs_dn8 * locals.var_vds) + (locals.var_vbscldvbs * locals.var_vds_dn8)) / 2.0);
        locals.var_t1_dn9 = (((locals.var_vbscldvbs_dn9 * locals.var_vds) + (locals.var_vbscldvbs * locals.var_vds_dn9)) / 2.0);
        locals.var_t1_dn10 = (((locals.var_vbscldvbs_dn10 * locals.var_vds) + (locals.var_vbscldvbs * locals.var_vds_dn10)) / 2.0);
        locals.var_t1_dn13 = (((locals.var_vbscldvbs_dn13 * locals.var_vds) + (locals.var_vbscldvbs * locals.var_vds_dn13)) / 2.0);
        locals.var_t1_rv = 0.0;

        let assign21940_e16921: f64 = (2.0 * locals.var_t1);
        let assign21940_e16923: f64 = (assign21940_e16921 / p.p262);
        locals.var_tmf1 = assign21940_e16923;
        locals.var_tmf1_dn0 = ((2.0 * locals.var_t1_dn0) / p.p262);
        locals.var_tmf1_dn2 = ((2.0 * locals.var_t1_dn2) / p.p262);
        locals.var_tmf1_dn4 = ((2.0 * locals.var_t1_dn4) / p.p262);
        locals.var_tmf1_dn5 = ((2.0 * locals.var_t1_dn5) / p.p262);
        locals.var_tmf1_dn6 = ((2.0 * locals.var_t1_dn6) / p.p262);
        locals.var_tmf1_dn7 = ((2.0 * locals.var_t1_dn7) / p.p262);
        locals.var_tmf1_dn8 = ((2.0 * locals.var_t1_dn8) / p.p262);
        locals.var_tmf1_dn9 = ((2.0 * locals.var_t1_dn9) / p.p262);
        locals.var_tmf1_dn10 = ((2.0 * locals.var_t1_dn10) / p.p262);
        locals.var_tmf1_dn13 = ((2.0 * locals.var_t1_dn13) / p.p262);
        locals.var_tmf1_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_56(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let assign21950_e16928: f64 = (1.0 / 2.0);
        let assign21950_e16932: f64 = (1.0 / 6.0);
        let assign21950_e16936: f64 = (1.0 / 24.0);
        let assign21950_e16940: f64 = (1.0 / 120.0);
        let assign21950_e16944: f64 = (1.0 / 720.0);
        let assign21950_e16948: f64 = (1.0 / 5040.0);
        let assign21950_e16949: f64 = (locals.var_tmf1 * assign21950_e16948);
        let assign21950_e16950: f64 = (assign21950_e16944 + assign21950_e16949);
        let assign21950_e16951: f64 = (locals.var_tmf1 * assign21950_e16950);
        let assign21950_e16952: f64 = (assign21950_e16940 + assign21950_e16951);
        let assign21950_e16953: f64 = (locals.var_tmf1 * assign21950_e16952);
        let assign21950_e16954: f64 = (assign21950_e16936 + assign21950_e16953);
        let assign21950_e16955: f64 = (locals.var_tmf1 * assign21950_e16954);
        let assign21950_e16956: f64 = (assign21950_e16932 + assign21950_e16955);
        let assign21950_e16957: f64 = (locals.var_tmf1 * assign21950_e16956);
        let assign21950_e16958: f64 = (assign21950_e16928 + assign21950_e16957);
        let assign21950_e16959: f64 = (locals.var_tmf1 * assign21950_e16958);
        let assign21950_e16960: f64 = (1.0 + assign21950_e16959);
        locals.var_tmf2 = assign21950_e16960;
        locals.var_tmf2_dn0 = ((locals.var_tmf1_dn0 * assign21950_e16958) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign21950_e16956) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign21950_e16954) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign21950_e16952) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign21950_e16950) + (locals.var_tmf1 * (locals.var_tmf1_dn0 * assign21950_e16948)))))))))));
        locals.var_tmf2_dn2 = ((locals.var_tmf1_dn2 * assign21950_e16958) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign21950_e16956) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign21950_e16954) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign21950_e16952) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign21950_e16950) + (locals.var_tmf1 * (locals.var_tmf1_dn2 * assign21950_e16948)))))))))));
        locals.var_tmf2_dn4 = ((locals.var_tmf1_dn4 * assign21950_e16958) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign21950_e16956) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign21950_e16954) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign21950_e16952) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign21950_e16950) + (locals.var_tmf1 * (locals.var_tmf1_dn4 * assign21950_e16948)))))))))));
        locals.var_tmf2_dn5 = ((locals.var_tmf1_dn5 * assign21950_e16958) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign21950_e16956) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign21950_e16954) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign21950_e16952) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign21950_e16950) + (locals.var_tmf1 * (locals.var_tmf1_dn5 * assign21950_e16948)))))))))));
        locals.var_tmf2_dn6 = ((locals.var_tmf1_dn6 * assign21950_e16958) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign21950_e16956) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign21950_e16954) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign21950_e16952) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign21950_e16950) + (locals.var_tmf1 * (locals.var_tmf1_dn6 * assign21950_e16948)))))))))));
        locals.var_tmf2_dn7 = ((locals.var_tmf1_dn7 * assign21950_e16958) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign21950_e16956) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign21950_e16954) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign21950_e16952) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign21950_e16950) + (locals.var_tmf1 * (locals.var_tmf1_dn7 * assign21950_e16948)))))))))));
        locals.var_tmf2_dn8 = ((locals.var_tmf1_dn8 * assign21950_e16958) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign21950_e16956) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign21950_e16954) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign21950_e16952) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign21950_e16950) + (locals.var_tmf1 * (locals.var_tmf1_dn8 * assign21950_e16948)))))))))));
        locals.var_tmf2_dn9 = ((locals.var_tmf1_dn9 * assign21950_e16958) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign21950_e16956) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign21950_e16954) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign21950_e16952) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign21950_e16950) + (locals.var_tmf1 * (locals.var_tmf1_dn9 * assign21950_e16948)))))))))));
        locals.var_tmf2_dn10 = ((locals.var_tmf1_dn10 * assign21950_e16958) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign21950_e16956) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign21950_e16954) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign21950_e16952) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign21950_e16950) + (locals.var_tmf1 * (locals.var_tmf1_dn10 * assign21950_e16948)))))))))));
        locals.var_tmf2_dn13 = ((locals.var_tmf1_dn13 * assign21950_e16958) + (locals.var_tmf1 * ((locals.var_tmf1_dn13 * assign21950_e16956) + (locals.var_tmf1 * ((locals.var_tmf1_dn13 * assign21950_e16954) + (locals.var_tmf1 * ((locals.var_tmf1_dn13 * assign21950_e16952) + (locals.var_tmf1 * ((locals.var_tmf1_dn13 * assign21950_e16950) + (locals.var_tmf1 * (locals.var_tmf1_dn13 * assign21950_e16948)))))))))));
        locals.var_tmf2_rv = 0.0;

        let assign21960_e16963: f64 = (1.0 / 2.0);
        let assign21960_e16967: f64 = (1.0 / 3.0);
        let assign21960_e16971: f64 = (1.0 / 8.0);
        let assign21960_e16975: f64 = (1.0 / 30.0);
        let assign21960_e16979: f64 = (1.0 / 144.0);
        let assign21960_e16983: f64 = (1.0 / 840.0);
        let assign21960_e16984: f64 = (locals.var_tmf1 * assign21960_e16983);
        let assign21960_e16985: f64 = (assign21960_e16979 + assign21960_e16984);
        let assign21960_e16986: f64 = (locals.var_tmf1 * assign21960_e16985);
        let assign21960_e16987: f64 = (assign21960_e16975 + assign21960_e16986);
        let assign21960_e16988: f64 = (locals.var_tmf1 * assign21960_e16987);
        let assign21960_e16989: f64 = (assign21960_e16971 + assign21960_e16988);
        let assign21960_e16990: f64 = (locals.var_tmf1 * assign21960_e16989);
        let assign21960_e16991: f64 = (assign21960_e16967 + assign21960_e16990);
        let assign21960_e16992: f64 = (locals.var_tmf1 * assign21960_e16991);
        let assign21960_e16993: f64 = (assign21960_e16963 + assign21960_e16992);
        locals.var_tmf3 = assign21960_e16993;
        locals.var_tmf3_dn0 = ((locals.var_tmf1_dn0 * assign21960_e16991) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign21960_e16989) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign21960_e16987) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign21960_e16985) + (locals.var_tmf1 * (locals.var_tmf1_dn0 * assign21960_e16983)))))))));
        locals.var_tmf3_dn2 = ((locals.var_tmf1_dn2 * assign21960_e16991) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign21960_e16989) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign21960_e16987) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign21960_e16985) + (locals.var_tmf1 * (locals.var_tmf1_dn2 * assign21960_e16983)))))))));
        locals.var_tmf3_dn4 = ((locals.var_tmf1_dn4 * assign21960_e16991) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign21960_e16989) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign21960_e16987) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign21960_e16985) + (locals.var_tmf1 * (locals.var_tmf1_dn4 * assign21960_e16983)))))))));
        locals.var_tmf3_dn5 = ((locals.var_tmf1_dn5 * assign21960_e16991) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign21960_e16989) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign21960_e16987) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign21960_e16985) + (locals.var_tmf1 * (locals.var_tmf1_dn5 * assign21960_e16983)))))))));
        locals.var_tmf3_dn6 = ((locals.var_tmf1_dn6 * assign21960_e16991) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign21960_e16989) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign21960_e16987) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign21960_e16985) + (locals.var_tmf1 * (locals.var_tmf1_dn6 * assign21960_e16983)))))))));
        locals.var_tmf3_dn7 = ((locals.var_tmf1_dn7 * assign21960_e16991) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign21960_e16989) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign21960_e16987) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign21960_e16985) + (locals.var_tmf1 * (locals.var_tmf1_dn7 * assign21960_e16983)))))))));
        locals.var_tmf3_dn8 = ((locals.var_tmf1_dn8 * assign21960_e16991) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign21960_e16989) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign21960_e16987) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign21960_e16985) + (locals.var_tmf1 * (locals.var_tmf1_dn8 * assign21960_e16983)))))))));
        locals.var_tmf3_dn9 = ((locals.var_tmf1_dn9 * assign21960_e16991) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign21960_e16989) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign21960_e16987) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign21960_e16985) + (locals.var_tmf1 * (locals.var_tmf1_dn9 * assign21960_e16983)))))))));
        locals.var_tmf3_dn10 = ((locals.var_tmf1_dn10 * assign21960_e16991) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign21960_e16989) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign21960_e16987) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign21960_e16985) + (locals.var_tmf1 * (locals.var_tmf1_dn10 * assign21960_e16983)))))))));
        locals.var_tmf3_dn13 = ((locals.var_tmf1_dn13 * assign21960_e16991) + (locals.var_tmf1 * ((locals.var_tmf1_dn13 * assign21960_e16989) + (locals.var_tmf1 * ((locals.var_tmf1_dn13 * assign21960_e16987) + (locals.var_tmf1 * ((locals.var_tmf1_dn13 * assign21960_e16985) + (locals.var_tmf1 * (locals.var_tmf1_dn13 * assign21960_e16983)))))))));
        locals.var_tmf3_rv = 0.0;

        let assign21970_e16996: f64 = (p.p262 / locals.var_tmf2);
        locals.var_vzadd = assign21970_e16996;
        locals.var_vzadd_dn0 = (-((p.p262 * locals.var_tmf2_dn0) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_vzadd_dn2 = (-((p.p262 * locals.var_tmf2_dn2) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_vzadd_dn4 = (-((p.p262 * locals.var_tmf2_dn4) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_vzadd_dn5 = (-((p.p262 * locals.var_tmf2_dn5) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_vzadd_dn6 = (-((p.p262 * locals.var_tmf2_dn6) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_vzadd_dn7 = (-((p.p262 * locals.var_tmf2_dn7) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_vzadd_dn8 = (-((p.p262 * locals.var_tmf2_dn8) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_vzadd_dn9 = (-((p.p262 * locals.var_tmf2_dn9) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_vzadd_dn10 = (-((p.p262 * locals.var_tmf2_dn10) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_vzadd_dn13 = (-((p.p262 * locals.var_tmf2_dn13) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_vzadd_rv = 0.0;

        let assign21980_e16998: f64 = (-2.0);
        let assign21980_e17000: f64 = (assign21980_e16998 * locals.var_tmf3);
        let assign21980_e17003: f64 = (locals.var_tmf2 * locals.var_tmf2);
        let assign21980_e17004: f64 = (assign21980_e17000 / assign21980_e17003);
        locals.var_t2 = assign21980_e17004;
        locals.var_t2_dn0 = ((((assign21980_e16998 * locals.var_tmf3_dn0) * assign21980_e17003) - (assign21980_e17000 * ((locals.var_tmf2_dn0 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn0)))) / (assign21980_e17003 * assign21980_e17003));
        locals.var_t2_dn2 = ((((assign21980_e16998 * locals.var_tmf3_dn2) * assign21980_e17003) - (assign21980_e17000 * ((locals.var_tmf2_dn2 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn2)))) / (assign21980_e17003 * assign21980_e17003));
        locals.var_t2_dn4 = ((((assign21980_e16998 * locals.var_tmf3_dn4) * assign21980_e17003) - (assign21980_e17000 * ((locals.var_tmf2_dn4 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn4)))) / (assign21980_e17003 * assign21980_e17003));
        locals.var_t2_dn5 = ((((assign21980_e16998 * locals.var_tmf3_dn5) * assign21980_e17003) - (assign21980_e17000 * ((locals.var_tmf2_dn5 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn5)))) / (assign21980_e17003 * assign21980_e17003));
        locals.var_t2_dn6 = ((((assign21980_e16998 * locals.var_tmf3_dn6) * assign21980_e17003) - (assign21980_e17000 * ((locals.var_tmf2_dn6 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn6)))) / (assign21980_e17003 * assign21980_e17003));
        locals.var_t2_dn7 = ((((assign21980_e16998 * locals.var_tmf3_dn7) * assign21980_e17003) - (assign21980_e17000 * ((locals.var_tmf2_dn7 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn7)))) / (assign21980_e17003 * assign21980_e17003));
        locals.var_t2_dn8 = ((((assign21980_e16998 * locals.var_tmf3_dn8) * assign21980_e17003) - (assign21980_e17000 * ((locals.var_tmf2_dn8 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn8)))) / (assign21980_e17003 * assign21980_e17003));
        locals.var_t2_dn9 = ((((assign21980_e16998 * locals.var_tmf3_dn9) * assign21980_e17003) - (assign21980_e17000 * ((locals.var_tmf2_dn9 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn9)))) / (assign21980_e17003 * assign21980_e17003));
        locals.var_t2_dn10 = ((((assign21980_e16998 * locals.var_tmf3_dn10) * assign21980_e17003) - (assign21980_e17000 * ((locals.var_tmf2_dn10 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn10)))) / (assign21980_e17003 * assign21980_e17003));
        locals.var_t2_dn13 = ((((assign21980_e16998 * locals.var_tmf3_dn13) * assign21980_e17003) - (assign21980_e17000 * ((locals.var_tmf2_dn13 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn13)))) / (assign21980_e17003 * assign21980_e17003));
        locals.var_t2_rv = 0.0;

        let assign21990_e17007: f64 = if locals.var_vzadd < 1e-12 { 1.0 } else { 0.0 };
        locals.var_guard423 = assign21990_e17007;
        locals.var_guard423_rv = 0.0;

        let (assign22000_e17011, assign22000_e17011_d_n0, assign22000_e17011_d_n2, assign22000_e17011_d_n4, assign22000_e17011_d_n5, assign22000_e17011_d_n6, assign22000_e17011_d_n7, assign22000_e17011_d_n8, assign22000_e17011_d_n9, assign22000_e17011_d_n10, assign22000_e17011_d_n13,) = {
    if (locals.var_guard423 != 0.0) {
        (1e-12, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vzadd, locals.var_vzadd_dn0, locals.var_vzadd_dn2, locals.var_vzadd_dn4, locals.var_vzadd_dn5, locals.var_vzadd_dn6, locals.var_vzadd_dn7, locals.var_vzadd_dn8, locals.var_vzadd_dn9, locals.var_vzadd_dn10, locals.var_vzadd_dn13,)
    }
};
        locals.var_vzadd = assign22000_e17011;
        locals.var_vzadd_dn0 = assign22000_e17011_d_n0;
        locals.var_vzadd_dn2 = assign22000_e17011_d_n2;
        locals.var_vzadd_dn4 = assign22000_e17011_d_n4;
        locals.var_vzadd_dn5 = assign22000_e17011_d_n5;
        locals.var_vzadd_dn6 = assign22000_e17011_d_n6;
        locals.var_vzadd_dn7 = assign22000_e17011_d_n7;
        locals.var_vzadd_dn8 = assign22000_e17011_d_n8;
        locals.var_vzadd_dn9 = assign22000_e17011_d_n9;
        locals.var_vzadd_dn10 = assign22000_e17011_d_n10;
        locals.var_vzadd_dn13 = assign22000_e17011_d_n13;
        locals.var_vzadd_rv = 0.0;

        let assign22010_e17014: f64 = (locals.var_vbscl + locals.var_vzadd);
        locals.var_vbsz = assign22010_e17014;
        locals.var_vbsz_dn0 = (locals.var_vbscl_dn0 + locals.var_vzadd_dn0);
        locals.var_vbsz_dn2 = (locals.var_vbscl_dn2 + locals.var_vzadd_dn2);
        locals.var_vbsz_dn4 = (locals.var_vbscl_dn4 + locals.var_vzadd_dn4);
        locals.var_vbsz_dn5 = (locals.var_vbscl_dn5 + locals.var_vzadd_dn5);
        locals.var_vbsz_dn6 = (locals.var_vbscl_dn6 + locals.var_vzadd_dn6);
        locals.var_vbsz_dn7 = (locals.var_vbscl_dn7 + locals.var_vzadd_dn7);
        locals.var_vbsz_dn8 = (locals.var_vbscl_dn8 + locals.var_vzadd_dn8);
        locals.var_vbsz_dn9 = (locals.var_vbscl_dn9 + locals.var_vzadd_dn9);
        locals.var_vbsz_dn10 = (locals.var_vbscl_dn10 + locals.var_vzadd_dn10);
        locals.var_vbsz_dn13 = (locals.var_vbscl_dn13 + locals.var_vzadd_dn13);
        locals.var_vbsz_rv = 0.0;

        let assign22020_e17018: f64 = (2.0 * locals.var_vzadd);
        let assign22020_e17019: f64 = (locals.var_vds + assign22020_e17018);
        locals.var_vdsz = assign22020_e17019;
        locals.var_vdsz_dn0 = (locals.var_vds_dn0 + (2.0 * locals.var_vzadd_dn0));
        locals.var_vdsz_dn2 = (locals.var_vds_dn2 + (2.0 * locals.var_vzadd_dn2));
        locals.var_vdsz_dn4 = (locals.var_vds_dn4 + (2.0 * locals.var_vzadd_dn4));
        locals.var_vdsz_dn5 = (locals.var_vds_dn5 + (2.0 * locals.var_vzadd_dn5));
        locals.var_vdsz_dn6 = (locals.var_vds_dn6 + (2.0 * locals.var_vzadd_dn6));
        locals.var_vdsz_dn7 = (locals.var_vds_dn7 + (2.0 * locals.var_vzadd_dn7));
        locals.var_vdsz_dn8 = (locals.var_vds_dn8 + (2.0 * locals.var_vzadd_dn8));
        locals.var_vdsz_dn9 = (locals.var_vds_dn9 + (2.0 * locals.var_vzadd_dn9));
        locals.var_vdsz_dn10 = (locals.var_vds_dn10 + (2.0 * locals.var_vzadd_dn10));
        locals.var_vdsz_dn13 = (locals.var_vds_dn13 + (2.0 * locals.var_vzadd_dn13));
        locals.var_vdsz_rv = 0.0;

        let assign22030_e17022: f64 = (locals.var_vgs + locals.var_vzadd);
        locals.var_vgsz = assign22030_e17022;
        locals.var_vgsz_dn0 = locals.var_vzadd_dn0;
        locals.var_vgsz_dn2 = locals.var_vzadd_dn2;
        locals.var_vgsz_dn4 = locals.var_vzadd_dn4;
        locals.var_vgsz_dn5 = (locals.var_vgs_dn5 + locals.var_vzadd_dn5);
        locals.var_vgsz_dn6 = (locals.var_vgs_dn6 + locals.var_vzadd_dn6);
        locals.var_vgsz_dn7 = (locals.var_vgs_dn7 + locals.var_vzadd_dn7);
        locals.var_vgsz_dn8 = locals.var_vzadd_dn8;
        locals.var_vgsz_dn9 = locals.var_vzadd_dn9;
        locals.var_vgsz_dn10 = locals.var_vzadd_dn10;
        locals.var_vgsz_dn13 = locals.var_vzadd_dn13;
        locals.var_vgsz_rv = 0.0;

        let assign22040_e17025: f64 = (locals.var_qnsub_esi * locals.var_cox0_inv);
        let assign22040_e17027: f64 = (assign22040_e17025 * locals.var_cox0_inv);
        locals.var_t1 = assign22040_e17027;
        locals.var_t1_dn0 = ((locals.var_qnsub_esi_dn0 * locals.var_cox0_inv) * locals.var_cox0_inv);
        locals.var_t1_dn2 = ((locals.var_qnsub_esi_dn2 * locals.var_cox0_inv) * locals.var_cox0_inv);
        locals.var_t1_dn4 = ((locals.var_qnsub_esi_dn4 * locals.var_cox0_inv) * locals.var_cox0_inv);
        locals.var_t1_dn5 = ((locals.var_qnsub_esi_dn5 * locals.var_cox0_inv) * locals.var_cox0_inv);
        locals.var_t1_dn6 = ((locals.var_qnsub_esi_dn6 * locals.var_cox0_inv) * locals.var_cox0_inv);
        locals.var_t1_dn7 = ((locals.var_qnsub_esi_dn7 * locals.var_cox0_inv) * locals.var_cox0_inv);
        locals.var_t1_dn8 = ((locals.var_qnsub_esi_dn8 * locals.var_cox0_inv) * locals.var_cox0_inv);
        locals.var_t1_dn9 = ((locals.var_qnsub_esi_dn9 * locals.var_cox0_inv) * locals.var_cox0_inv);
        locals.var_t1_dn10 = ((locals.var_qnsub_esi_dn10 * locals.var_cox0_inv) * locals.var_cox0_inv);
        locals.var_t1_dn13 = ((locals.var_qnsub_esi_dn13 * locals.var_cox0_inv) * locals.var_cox0_inv);
        locals.var_t1_rv = 0.0;

        let assign22050_e17030: f64 = (locals.var_vgs - locals.var_vfb);
        locals.var_t2 = assign22050_e17030;
        locals.var_t2_dn0 = 0.0;
        locals.var_t2_dn2 = 0.0;
        locals.var_t2_dn4 = 0.0;
        locals.var_t2_dn5 = locals.var_vgs_dn5;
        locals.var_t2_dn6 = locals.var_vgs_dn6;
        locals.var_t2_dn7 = locals.var_vgs_dn7;
        locals.var_t2_dn8 = 0.0;
        locals.var_t2_dn9 = 0.0;
        locals.var_t2_dn10 = 0.0;
        locals.var_t2_dn13 = 0.0;
        locals.var_t2_rv = 0.0;

        let assign22060_e17034: f64 = (2.0 / locals.var_t1);
        let assign22060_e17038: f64 = (1.0 / locals.var_betatnom);
        let assign22060_e17039: f64 = (locals.var_t2 - assign22060_e17038);
        let assign22060_e17041: f64 = (assign22060_e17039 - locals.var_vbscl);
        let assign22060_e17042: f64 = (assign22060_e17034 * assign22060_e17041);
        let assign22060_e17043: f64 = (1.0 + assign22060_e17042);
        locals.var_t3 = assign22060_e17043;
        locals.var_t3_dn0 = (((-((2.0 * locals.var_t1_dn0) / (locals.var_t1 * locals.var_t1))) * assign22060_e17041) + (assign22060_e17034 * (locals.var_t2_dn0 - locals.var_vbscl_dn0)));
        locals.var_t3_dn2 = (((-((2.0 * locals.var_t1_dn2) / (locals.var_t1 * locals.var_t1))) * assign22060_e17041) + (assign22060_e17034 * (locals.var_t2_dn2 - locals.var_vbscl_dn2)));
        locals.var_t3_dn4 = (((-((2.0 * locals.var_t1_dn4) / (locals.var_t1 * locals.var_t1))) * assign22060_e17041) + (assign22060_e17034 * (locals.var_t2_dn4 - locals.var_vbscl_dn4)));
        locals.var_t3_dn5 = (((-((2.0 * locals.var_t1_dn5) / (locals.var_t1 * locals.var_t1))) * assign22060_e17041) + (assign22060_e17034 * (locals.var_t2_dn5 - locals.var_vbscl_dn5)));
        locals.var_t3_dn6 = (((-((2.0 * locals.var_t1_dn6) / (locals.var_t1 * locals.var_t1))) * assign22060_e17041) + (assign22060_e17034 * (locals.var_t2_dn6 - locals.var_vbscl_dn6)));
        locals.var_t3_dn7 = (((-((2.0 * locals.var_t1_dn7) / (locals.var_t1 * locals.var_t1))) * assign22060_e17041) + (assign22060_e17034 * (locals.var_t2_dn7 - locals.var_vbscl_dn7)));
        locals.var_t3_dn8 = (((-((2.0 * locals.var_t1_dn8) / (locals.var_t1 * locals.var_t1))) * assign22060_e17041) + (assign22060_e17034 * (locals.var_t2_dn8 - locals.var_vbscl_dn8)));
        locals.var_t3_dn9 = (((-((2.0 * locals.var_t1_dn9) / (locals.var_t1 * locals.var_t1))) * assign22060_e17041) + (assign22060_e17034 * (locals.var_t2_dn9 - locals.var_vbscl_dn9)));
        locals.var_t3_dn10 = (((-((2.0 * locals.var_t1_dn10) / (locals.var_t1 * locals.var_t1))) * assign22060_e17041) + (assign22060_e17034 * (locals.var_t2_dn10 - locals.var_vbscl_dn10)));
        locals.var_t3_dn13 = (((-((2.0 * locals.var_t1_dn13) / (locals.var_t1 * locals.var_t1))) * assign22060_e17041) + (assign22060_e17034 * (locals.var_t2_dn13 - locals.var_vbscl_dn13)));
        locals.var_t3_rv = 0.0;

        let assign22070_e17046: f64 = (locals.var_t3 * locals.var_t3);
        let assign22070_e17049: f64 = (4.0 * 0.001);
        let assign22070_e17051: f64 = (assign22070_e17049 * 0.001);
        let assign22070_e17052: f64 = (assign22070_e17046 + assign22070_e17051);
        let assign22070_e17053: f64 = (assign22070_e17052).sqrt();
        locals.var_tmf2 = assign22070_e17053;
        locals.var_tmf2_dn0 = (((locals.var_t3_dn0 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn0)) / (2.0 * assign22070_e17053));
        locals.var_tmf2_dn2 = (((locals.var_t3_dn2 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn2)) / (2.0 * assign22070_e17053));
        locals.var_tmf2_dn4 = (((locals.var_t3_dn4 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn4)) / (2.0 * assign22070_e17053));
        locals.var_tmf2_dn5 = (((locals.var_t3_dn5 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn5)) / (2.0 * assign22070_e17053));
        locals.var_tmf2_dn6 = (((locals.var_t3_dn6 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn6)) / (2.0 * assign22070_e17053));
        locals.var_tmf2_dn7 = (((locals.var_t3_dn7 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn7)) / (2.0 * assign22070_e17053));
        locals.var_tmf2_dn8 = (((locals.var_t3_dn8 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn8)) / (2.0 * assign22070_e17053));
        locals.var_tmf2_dn9 = (((locals.var_t3_dn9 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn9)) / (2.0 * assign22070_e17053));
        locals.var_tmf2_dn10 = (((locals.var_t3_dn10 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn10)) / (2.0 * assign22070_e17053));
        locals.var_tmf2_dn13 = (((locals.var_t3_dn13 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn13)) / (2.0 * assign22070_e17053));
        locals.var_tmf2_rv = 0.0;

        let assign22080_e17058: f64 = (locals.var_t3 / locals.var_tmf2);
        let assign22080_e17059: f64 = (1.0 + assign22080_e17058);
        let assign22080_e17060: f64 = (0.5 * assign22080_e17059);
        locals.var_t5 = assign22080_e17060;
        locals.var_t5_dn0 = (0.5 * (((locals.var_t3_dn0 * locals.var_tmf2) - (locals.var_t3 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_t5_dn2 = (0.5 * (((locals.var_t3_dn2 * locals.var_tmf2) - (locals.var_t3 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_t5_dn4 = (0.5 * (((locals.var_t3_dn4 * locals.var_tmf2) - (locals.var_t3 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_t5_dn5 = (0.5 * (((locals.var_t3_dn5 * locals.var_tmf2) - (locals.var_t3 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_t5_dn6 = (0.5 * (((locals.var_t3_dn6 * locals.var_tmf2) - (locals.var_t3 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_t5_dn7 = (0.5 * (((locals.var_t3_dn7 * locals.var_tmf2) - (locals.var_t3 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_t5_dn8 = (0.5 * (((locals.var_t3_dn8 * locals.var_tmf2) - (locals.var_t3 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_t5_dn9 = (0.5 * (((locals.var_t3_dn9 * locals.var_tmf2) - (locals.var_t3 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_t5_dn10 = (0.5 * (((locals.var_t3_dn10 * locals.var_tmf2) - (locals.var_t3 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_t5_dn13 = (0.5 * (((locals.var_t3_dn13 * locals.var_tmf2) - (locals.var_t3 * locals.var_tmf2_dn13)) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_t5_rv = 0.0;

        let assign22090_e17064: f64 = (locals.var_t3 + locals.var_tmf2);
        let assign22090_e17065: f64 = (0.5 * assign22090_e17064);
        locals.var_t4 = assign22090_e17065;
        locals.var_t4_dn0 = (0.5 * (locals.var_t3_dn0 + locals.var_tmf2_dn0));
        locals.var_t4_dn2 = (0.5 * (locals.var_t3_dn2 + locals.var_tmf2_dn2));
        locals.var_t4_dn4 = (0.5 * (locals.var_t3_dn4 + locals.var_tmf2_dn4));
        locals.var_t4_dn5 = (0.5 * (locals.var_t3_dn5 + locals.var_tmf2_dn5));
        locals.var_t4_dn6 = (0.5 * (locals.var_t3_dn6 + locals.var_tmf2_dn6));
        locals.var_t4_dn7 = (0.5 * (locals.var_t3_dn7 + locals.var_tmf2_dn7));
        locals.var_t4_dn8 = (0.5 * (locals.var_t3_dn8 + locals.var_tmf2_dn8));
        locals.var_t4_dn9 = (0.5 * (locals.var_t3_dn9 + locals.var_tmf2_dn9));
        locals.var_t4_dn10 = (0.5 * (locals.var_t3_dn10 + locals.var_tmf2_dn10));
        locals.var_t4_dn13 = (0.5 * (locals.var_t3_dn13 + locals.var_tmf2_dn13));
        locals.var_t4_rv = 0.0;

        let assign22100_e17068: f64 = if locals.var_t4 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard424 = assign22100_e17068;
        locals.var_guard424_rv = 0.0;

        let (assign22110_e17072, assign22110_e17072_d_n0, assign22110_e17072_d_n2, assign22110_e17072_d_n4, assign22110_e17072_d_n5, assign22110_e17072_d_n6, assign22110_e17072_d_n7, assign22110_e17072_d_n8, assign22110_e17072_d_n9, assign22110_e17072_d_n10, assign22110_e17072_d_n13,) = {
    if (locals.var_guard424 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn13,)
    }
};
        locals.var_t4 = assign22110_e17072;
        locals.var_t4_dn0 = assign22110_e17072_d_n0;
        locals.var_t4_dn2 = assign22110_e17072_d_n2;
        locals.var_t4_dn4 = assign22110_e17072_d_n4;
        locals.var_t4_dn5 = assign22110_e17072_d_n5;
        locals.var_t4_dn6 = assign22110_e17072_d_n6;
        locals.var_t4_dn7 = assign22110_e17072_d_n7;
        locals.var_t4_dn8 = assign22110_e17072_d_n8;
        locals.var_t4_dn9 = assign22110_e17072_d_n9;
        locals.var_t4_dn10 = assign22110_e17072_d_n10;
        locals.var_t4_dn13 = assign22110_e17072_d_n13;
        locals.var_t4_rv = 0.0;

        let (assign22120_e17076, assign22120_e17076_d_n0, assign22120_e17076_d_n2, assign22120_e17076_d_n4, assign22120_e17076_d_n5, assign22120_e17076_d_n6, assign22120_e17076_d_n7, assign22120_e17076_d_n8, assign22120_e17076_d_n9, assign22120_e17076_d_n10, assign22120_e17076_d_n13,) = {
    if (locals.var_guard424 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn13,)
    }
};
        locals.var_t5 = assign22120_e17076;
        locals.var_t5_dn0 = assign22120_e17076_d_n0;
        locals.var_t5_dn2 = assign22120_e17076_d_n2;
        locals.var_t5_dn4 = assign22120_e17076_d_n4;
        locals.var_t5_dn5 = assign22120_e17076_d_n5;
        locals.var_t5_dn6 = assign22120_e17076_d_n6;
        locals.var_t5_dn7 = assign22120_e17076_d_n7;
        locals.var_t5_dn8 = assign22120_e17076_d_n8;
        locals.var_t5_dn9 = assign22120_e17076_d_n9;
        locals.var_t5_dn10 = assign22120_e17076_d_n10;
        locals.var_t5_dn13 = assign22120_e17076_d_n13;
        locals.var_t5_rv = 0.0;

        let assign22130_e17079: f64 = (locals.var_t4 + 1e-25);
        locals.var_t4 = assign22130_e17079;
        locals.var_t4_dn0 = locals.var_t4_dn0;
        locals.var_t4_dn2 = locals.var_t4_dn2;
        locals.var_t4_dn4 = locals.var_t4_dn4;
        locals.var_t4_dn5 = locals.var_t4_dn5;
        locals.var_t4_dn6 = locals.var_t4_dn6;
        locals.var_t4_dn7 = locals.var_t4_dn7;
        locals.var_t4_dn8 = locals.var_t4_dn8;
        locals.var_t4_dn9 = locals.var_t4_dn9;
        locals.var_t4_dn10 = locals.var_t4_dn10;
        locals.var_t4_dn13 = locals.var_t4_dn13;
        locals.var_t4_rv = 0.0;

        let assign22140_e17081: f64 = (locals.var_t4).sqrt();
        locals.var_tx = assign22140_e17081;
        locals.var_tx_dn0 = (locals.var_t4_dn0 / (2.0 * assign22140_e17081));
        locals.var_tx_dn2 = (locals.var_t4_dn2 / (2.0 * assign22140_e17081));
        locals.var_tx_dn4 = (locals.var_t4_dn4 / (2.0 * assign22140_e17081));
        locals.var_tx_dn5 = (locals.var_t4_dn5 / (2.0 * assign22140_e17081));
        locals.var_tx_dn6 = (locals.var_t4_dn6 / (2.0 * assign22140_e17081));
        locals.var_tx_dn7 = (locals.var_t4_dn7 / (2.0 * assign22140_e17081));
        locals.var_tx_dn8 = (locals.var_t4_dn8 / (2.0 * assign22140_e17081));
        locals.var_tx_dn9 = (locals.var_t4_dn9 / (2.0 * assign22140_e17081));
        locals.var_tx_dn10 = (locals.var_t4_dn10 / (2.0 * assign22140_e17081));
        locals.var_tx_dn13 = (locals.var_t4_dn13 / (2.0 * assign22140_e17081));
        locals.var_tx_rv = 0.0;

        let assign22150_e17086: f64 = (1.0 - locals.var_tx);
        let assign22150_e17087: f64 = (locals.var_t1 * assign22150_e17086);
        let assign22150_e17088: f64 = (locals.var_t2 + assign22150_e17087);
        locals.var_pslsat = assign22150_e17088;
        locals.var_pslsat_dn0 = (locals.var_t2_dn0 + ((locals.var_t1_dn0 * assign22150_e17086) + (locals.var_t1 * (-locals.var_tx_dn0))));
        locals.var_pslsat_dn2 = (locals.var_t2_dn2 + ((locals.var_t1_dn2 * assign22150_e17086) + (locals.var_t1 * (-locals.var_tx_dn2))));
        locals.var_pslsat_dn4 = (locals.var_t2_dn4 + ((locals.var_t1_dn4 * assign22150_e17086) + (locals.var_t1 * (-locals.var_tx_dn4))));
        locals.var_pslsat_dn5 = (locals.var_t2_dn5 + ((locals.var_t1_dn5 * assign22150_e17086) + (locals.var_t1 * (-locals.var_tx_dn5))));
        locals.var_pslsat_dn6 = (locals.var_t2_dn6 + ((locals.var_t1_dn6 * assign22150_e17086) + (locals.var_t1 * (-locals.var_tx_dn6))));
        locals.var_pslsat_dn7 = (locals.var_t2_dn7 + ((locals.var_t1_dn7 * assign22150_e17086) + (locals.var_t1 * (-locals.var_tx_dn7))));
        locals.var_pslsat_dn8 = (locals.var_t2_dn8 + ((locals.var_t1_dn8 * assign22150_e17086) + (locals.var_t1 * (-locals.var_tx_dn8))));
        locals.var_pslsat_dn9 = (locals.var_t2_dn9 + ((locals.var_t1_dn9 * assign22150_e17086) + (locals.var_t1 * (-locals.var_tx_dn9))));
        locals.var_pslsat_dn10 = (locals.var_t2_dn10 + ((locals.var_t1_dn10 * assign22150_e17086) + (locals.var_t1 * (-locals.var_tx_dn10))));
        locals.var_pslsat_dn13 = (locals.var_t2_dn13 + ((locals.var_t1_dn13 * assign22150_e17086) + (locals.var_t1 * (-locals.var_tx_dn13))));
        locals.var_pslsat_rv = 0.0;

        let assign22160_e17091: f64 = (locals.var_pslsat - locals.var_pb2c);
        locals.var_vdsats = assign22160_e17091;
        locals.var_vdsats_dn0 = (locals.var_pslsat_dn0 - locals.var_pb2c_dn0);
        locals.var_vdsats_dn2 = (locals.var_pslsat_dn2 - locals.var_pb2c_dn2);
        locals.var_vdsats_dn4 = (locals.var_pslsat_dn4 - locals.var_pb2c_dn4);
        locals.var_vdsats_dn5 = (locals.var_pslsat_dn5 - locals.var_pb2c_dn5);
        locals.var_vdsats_dn6 = (locals.var_pslsat_dn6 - locals.var_pb2c_dn6);
        locals.var_vdsats_dn7 = (locals.var_pslsat_dn7 - locals.var_pb2c_dn7);
        locals.var_vdsats_dn8 = (locals.var_pslsat_dn8 - locals.var_pb2c_dn8);
        locals.var_vdsats_dn9 = (locals.var_pslsat_dn9 - locals.var_pb2c_dn9);
        locals.var_vdsats_dn10 = (locals.var_pslsat_dn10 - locals.var_pb2c_dn10);
        locals.var_vdsats_dn13 = (locals.var_pslsat_dn13 - locals.var_pb2c_dn13);
        locals.var_vdsats_rv = 0.0;

        let assign22170_e17094: f64 = (locals.var_vdsats - 0.1);
        let assign22170_e17096: f64 = (assign22170_e17094 - 0.05);
        locals.var_tmf1 = assign22170_e17096;
        locals.var_tmf1_dn0 = locals.var_vdsats_dn0;
        locals.var_tmf1_dn2 = locals.var_vdsats_dn2;
        locals.var_tmf1_dn4 = locals.var_vdsats_dn4;
        locals.var_tmf1_dn5 = locals.var_vdsats_dn5;
        locals.var_tmf1_dn6 = locals.var_vdsats_dn6;
        locals.var_tmf1_dn7 = locals.var_vdsats_dn7;
        locals.var_tmf1_dn8 = locals.var_vdsats_dn8;
        locals.var_tmf1_dn9 = locals.var_vdsats_dn9;
        locals.var_tmf1_dn10 = locals.var_vdsats_dn10;
        locals.var_tmf1_dn13 = locals.var_vdsats_dn13;
        locals.var_tmf1_rv = 0.0;

        let assign22180_e17099: f64 = (4.0 * 0.1);
        let assign22180_e17101: f64 = (assign22180_e17099 * 0.05);
        locals.var_tmf2 = assign22180_e17101;
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

        let (assign22190_e17108, assign22190_e17108_d_n0, assign22190_e17108_d_n2, assign22190_e17108_d_n4, assign22190_e17108_d_n5, assign22190_e17108_d_n6, assign22190_e17108_d_n7, assign22190_e17108_d_n8, assign22190_e17108_d_n9, assign22190_e17108_d_n10, assign22190_e17108_d_n13,) = {
    if (locals.var_tmf2 > 0.0) {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    } else {
        let assign22190_e17107: f64 = (-locals.var_tmf2);
        (assign22190_e17107, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn13),)
    }
};
        locals.var_tmf2 = assign22190_e17108;
        locals.var_tmf2_dn0 = assign22190_e17108_d_n0;
        locals.var_tmf2_dn2 = assign22190_e17108_d_n2;
        locals.var_tmf2_dn4 = assign22190_e17108_d_n4;
        locals.var_tmf2_dn5 = assign22190_e17108_d_n5;
        locals.var_tmf2_dn6 = assign22190_e17108_d_n6;
        locals.var_tmf2_dn7 = assign22190_e17108_d_n7;
        locals.var_tmf2_dn8 = assign22190_e17108_d_n8;
        locals.var_tmf2_dn9 = assign22190_e17108_d_n9;
        locals.var_tmf2_dn10 = assign22190_e17108_d_n10;
        locals.var_tmf2_dn13 = assign22190_e17108_d_n13;
        locals.var_tmf2_rv = 0.0;

        let assign22200_e17111: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign22200_e17113: f64 = (assign22200_e17111 + locals.var_tmf2);
        let assign22200_e17114: f64 = (assign22200_e17113).sqrt();
        locals.var_tmf2 = assign22200_e17114;
        locals.var_tmf2_dn0 = ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign22200_e17114));
        locals.var_tmf2_dn2 = ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign22200_e17114));
        locals.var_tmf2_dn4 = ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign22200_e17114));
        locals.var_tmf2_dn5 = ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign22200_e17114));
        locals.var_tmf2_dn6 = ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign22200_e17114));
        locals.var_tmf2_dn7 = ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign22200_e17114));
        locals.var_tmf2_dn8 = ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign22200_e17114));
        locals.var_tmf2_dn9 = ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign22200_e17114));
        locals.var_tmf2_dn10 = ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign22200_e17114));
        locals.var_tmf2_dn13 = ((((locals.var_tmf1_dn13 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn13)) + locals.var_tmf2_dn13) / (2.0 * assign22200_e17114));
        locals.var_tmf2_rv = 0.0;

        let assign22210_e17119: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign22210_e17120: f64 = (1.0 + assign22210_e17119);
        let assign22210_e17121: f64 = (0.5 * assign22210_e17120);
        locals.var_t6 = assign22210_e17121;
        locals.var_t6_dn0 = (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_t6_dn2 = (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_t6_dn4 = (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_t6_dn5 = (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_t6_dn6 = (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_t6_dn7 = (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_t6_dn8 = (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_t6_dn9 = (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_t6_dn10 = (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_t6_dn13 = (0.5 * (((locals.var_tmf1_dn13 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn13)) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_t6_rv = 0.0;

        let assign22220_e17126: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign22220_e17127: f64 = (0.5 * assign22220_e17126);
        let assign22220_e17128: f64 = (0.1 + assign22220_e17127);
        locals.var_vdsats = assign22220_e17128;
        locals.var_vdsats_dn0 = (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0));
        locals.var_vdsats_dn2 = (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2));
        locals.var_vdsats_dn4 = (0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4));
        locals.var_vdsats_dn5 = (0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5));
        locals.var_vdsats_dn6 = (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6));
        locals.var_vdsats_dn7 = (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7));
        locals.var_vdsats_dn8 = (0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8));
        locals.var_vdsats_dn9 = (0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9));
        locals.var_vdsats_dn10 = (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10));
        locals.var_vdsats_dn13 = (0.5 * (locals.var_tmf1_dn13 + locals.var_tmf2_dn13));
        locals.var_vdsats_rv = 0.0;

        let assign22230_e17131: f64 = (locals.var_vds / locals.var_vdsats);
        locals.var_t1 = assign22230_e17131;
        locals.var_t1_dn0 = (((locals.var_vds_dn0 * locals.var_vdsats) - (locals.var_vds * locals.var_vdsats_dn0)) / (locals.var_vdsats * locals.var_vdsats));
        locals.var_t1_dn2 = (((locals.var_vds_dn2 * locals.var_vdsats) - (locals.var_vds * locals.var_vdsats_dn2)) / (locals.var_vdsats * locals.var_vdsats));
        locals.var_t1_dn4 = (((locals.var_vds_dn4 * locals.var_vdsats) - (locals.var_vds * locals.var_vdsats_dn4)) / (locals.var_vdsats * locals.var_vdsats));
        locals.var_t1_dn5 = (((locals.var_vds_dn5 * locals.var_vdsats) - (locals.var_vds * locals.var_vdsats_dn5)) / (locals.var_vdsats * locals.var_vdsats));
        locals.var_t1_dn6 = (((locals.var_vds_dn6 * locals.var_vdsats) - (locals.var_vds * locals.var_vdsats_dn6)) / (locals.var_vdsats * locals.var_vdsats));
        locals.var_t1_dn7 = (((locals.var_vds_dn7 * locals.var_vdsats) - (locals.var_vds * locals.var_vdsats_dn7)) / (locals.var_vdsats * locals.var_vdsats));
        locals.var_t1_dn8 = (((locals.var_vds_dn8 * locals.var_vdsats) - (locals.var_vds * locals.var_vdsats_dn8)) / (locals.var_vdsats * locals.var_vdsats));
        locals.var_t1_dn9 = (((locals.var_vds_dn9 * locals.var_vdsats) - (locals.var_vds * locals.var_vdsats_dn9)) / (locals.var_vdsats * locals.var_vdsats));
        locals.var_t1_dn10 = (((locals.var_vds_dn10 * locals.var_vdsats) - (locals.var_vds * locals.var_vdsats_dn10)) / (locals.var_vdsats * locals.var_vdsats));
        locals.var_t1_dn13 = (((locals.var_vds_dn13 * locals.var_vdsats) - (locals.var_vds * locals.var_vdsats_dn13)) / (locals.var_vdsats * locals.var_vdsats));
        locals.var_t1_rv = 0.0;

        let assign22240_e17134: f64 = locals.var_t1;
        locals.var_tmf1 = assign22240_e17134;
        locals.var_tmf1_dn0 = locals.var_t1_dn0;
        locals.var_tmf1_dn2 = locals.var_t1_dn2;
        locals.var_tmf1_dn4 = locals.var_t1_dn4;
        locals.var_tmf1_dn5 = locals.var_t1_dn5;
        locals.var_tmf1_dn6 = locals.var_t1_dn6;
        locals.var_tmf1_dn7 = locals.var_t1_dn7;
        locals.var_tmf1_dn8 = locals.var_t1_dn8;
        locals.var_tmf1_dn9 = locals.var_t1_dn9;
        locals.var_tmf1_dn10 = locals.var_t1_dn10;
        locals.var_tmf1_dn13 = locals.var_t1_dn13;
        locals.var_tmf1_rv = 0.0;

        let assign22250_e17137: f64 = (locals.var_tmf1 * locals.var_tmf1);
        locals.var_tmf2 = assign22250_e17137;
        locals.var_tmf2_dn0 = ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0));
        locals.var_tmf2_dn2 = ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2));
        locals.var_tmf2_dn4 = ((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4));
        locals.var_tmf2_dn5 = ((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5));
        locals.var_tmf2_dn6 = ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6));
        locals.var_tmf2_dn7 = ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7));
        locals.var_tmf2_dn8 = ((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8));
        locals.var_tmf2_dn9 = ((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9));
        locals.var_tmf2_dn10 = ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10));
        locals.var_tmf2_dn13 = ((locals.var_tmf1_dn13 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn13));
        locals.var_tmf2_rv = 0.0;

        let assign22260_e17140: f64 = (locals.var_tmf2 * locals.var_tmf1);
        locals.var_tmf3 = assign22260_e17140;
        locals.var_tmf3_dn0 = ((locals.var_tmf2_dn0 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn0));
        locals.var_tmf3_dn2 = ((locals.var_tmf2_dn2 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn2));
        locals.var_tmf3_dn4 = ((locals.var_tmf2_dn4 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn4));
        locals.var_tmf3_dn5 = ((locals.var_tmf2_dn5 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn5));
        locals.var_tmf3_dn6 = ((locals.var_tmf2_dn6 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn6));
        locals.var_tmf3_dn7 = ((locals.var_tmf2_dn7 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn7));
        locals.var_tmf3_dn8 = ((locals.var_tmf2_dn8 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn8));
        locals.var_tmf3_dn9 = ((locals.var_tmf2_dn9 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn9));
        locals.var_tmf3_dn10 = ((locals.var_tmf2_dn10 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn10));
        locals.var_tmf3_dn13 = ((locals.var_tmf2_dn13 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn13));
        locals.var_tmf3_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_57(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let assign22270_e17143: f64 = (locals.var_tmf2 * locals.var_tmf2);
        locals.var_tmf4 = assign22270_e17143;
        locals.var_tmf4_dn0 = ((locals.var_tmf2_dn0 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn0));
        locals.var_tmf4_dn2 = ((locals.var_tmf2_dn2 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn2));
        locals.var_tmf4_dn4 = ((locals.var_tmf2_dn4 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn4));
        locals.var_tmf4_dn5 = ((locals.var_tmf2_dn5 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn5));
        locals.var_tmf4_dn6 = ((locals.var_tmf2_dn6 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn6));
        locals.var_tmf4_dn7 = ((locals.var_tmf2_dn7 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn7));
        locals.var_tmf4_dn8 = ((locals.var_tmf2_dn8 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn8));
        locals.var_tmf4_dn9 = ((locals.var_tmf2_dn9 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn9));
        locals.var_tmf4_dn10 = ((locals.var_tmf2_dn10 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn10));
        locals.var_tmf4_dn13 = ((locals.var_tmf2_dn13 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn13));
        locals.var_tmf4_rv = 0.0;

        let assign22280_e17147: f64 = (1.0 + locals.var_tmf1);
        let assign22280_e17149: f64 = (assign22280_e17147 + locals.var_tmf2);
        let assign22280_e17151: f64 = (assign22280_e17149 + locals.var_tmf3);
        let assign22280_e17153: f64 = (assign22280_e17151 + locals.var_tmf4);
        let assign22280_e17154: f64 = (1.0 / assign22280_e17153);
        locals.var_tx = assign22280_e17154;
        locals.var_tx_dn0 = (-((((locals.var_tmf1_dn0 + locals.var_tmf2_dn0) + locals.var_tmf3_dn0) + locals.var_tmf4_dn0) / (assign22280_e17153 * assign22280_e17153)));
        locals.var_tx_dn2 = (-((((locals.var_tmf1_dn2 + locals.var_tmf2_dn2) + locals.var_tmf3_dn2) + locals.var_tmf4_dn2) / (assign22280_e17153 * assign22280_e17153)));
        locals.var_tx_dn4 = (-((((locals.var_tmf1_dn4 + locals.var_tmf2_dn4) + locals.var_tmf3_dn4) + locals.var_tmf4_dn4) / (assign22280_e17153 * assign22280_e17153)));
        locals.var_tx_dn5 = (-((((locals.var_tmf1_dn5 + locals.var_tmf2_dn5) + locals.var_tmf3_dn5) + locals.var_tmf4_dn5) / (assign22280_e17153 * assign22280_e17153)));
        locals.var_tx_dn6 = (-((((locals.var_tmf1_dn6 + locals.var_tmf2_dn6) + locals.var_tmf3_dn6) + locals.var_tmf4_dn6) / (assign22280_e17153 * assign22280_e17153)));
        locals.var_tx_dn7 = (-((((locals.var_tmf1_dn7 + locals.var_tmf2_dn7) + locals.var_tmf3_dn7) + locals.var_tmf4_dn7) / (assign22280_e17153 * assign22280_e17153)));
        locals.var_tx_dn8 = (-((((locals.var_tmf1_dn8 + locals.var_tmf2_dn8) + locals.var_tmf3_dn8) + locals.var_tmf4_dn8) / (assign22280_e17153 * assign22280_e17153)));
        locals.var_tx_dn9 = (-((((locals.var_tmf1_dn9 + locals.var_tmf2_dn9) + locals.var_tmf3_dn9) + locals.var_tmf4_dn9) / (assign22280_e17153 * assign22280_e17153)));
        locals.var_tx_dn10 = (-((((locals.var_tmf1_dn10 + locals.var_tmf2_dn10) + locals.var_tmf3_dn10) + locals.var_tmf4_dn10) / (assign22280_e17153 * assign22280_e17153)));
        locals.var_tx_dn13 = (-((((locals.var_tmf1_dn13 + locals.var_tmf2_dn13) + locals.var_tmf3_dn13) + locals.var_tmf4_dn13) / (assign22280_e17153 * assign22280_e17153)));
        locals.var_tx_rv = 0.0;

        let assign22290_e17158: f64 = (2.0 * locals.var_tmf1);
        let assign22290_e17159: f64 = (1.0 + assign22290_e17158);
        let assign22290_e17162: f64 = (3.0 * locals.var_tmf2);
        let assign22290_e17163: f64 = (assign22290_e17159 + assign22290_e17162);
        let assign22290_e17166: f64 = (4.0 * locals.var_tmf3);
        let assign22290_e17167: f64 = (assign22290_e17163 + assign22290_e17166);
        let assign22290_e17168: f64 = (-assign22290_e17167);
        let assign22290_e17170: f64 = (assign22290_e17168 * locals.var_tx);
        let assign22290_e17172: f64 = (assign22290_e17170 * locals.var_tx);
        locals.var_t0 = assign22290_e17172;
        locals.var_t0_dn0 = (((((-(((2.0 * locals.var_tmf1_dn0) + (3.0 * locals.var_tmf2_dn0)) + (4.0 * locals.var_tmf3_dn0))) * locals.var_tx) + (assign22290_e17168 * locals.var_tx_dn0)) * locals.var_tx) + (assign22290_e17170 * locals.var_tx_dn0));
        locals.var_t0_dn2 = (((((-(((2.0 * locals.var_tmf1_dn2) + (3.0 * locals.var_tmf2_dn2)) + (4.0 * locals.var_tmf3_dn2))) * locals.var_tx) + (assign22290_e17168 * locals.var_tx_dn2)) * locals.var_tx) + (assign22290_e17170 * locals.var_tx_dn2));
        locals.var_t0_dn4 = (((((-(((2.0 * locals.var_tmf1_dn4) + (3.0 * locals.var_tmf2_dn4)) + (4.0 * locals.var_tmf3_dn4))) * locals.var_tx) + (assign22290_e17168 * locals.var_tx_dn4)) * locals.var_tx) + (assign22290_e17170 * locals.var_tx_dn4));
        locals.var_t0_dn5 = (((((-(((2.0 * locals.var_tmf1_dn5) + (3.0 * locals.var_tmf2_dn5)) + (4.0 * locals.var_tmf3_dn5))) * locals.var_tx) + (assign22290_e17168 * locals.var_tx_dn5)) * locals.var_tx) + (assign22290_e17170 * locals.var_tx_dn5));
        locals.var_t0_dn6 = (((((-(((2.0 * locals.var_tmf1_dn6) + (3.0 * locals.var_tmf2_dn6)) + (4.0 * locals.var_tmf3_dn6))) * locals.var_tx) + (assign22290_e17168 * locals.var_tx_dn6)) * locals.var_tx) + (assign22290_e17170 * locals.var_tx_dn6));
        locals.var_t0_dn7 = (((((-(((2.0 * locals.var_tmf1_dn7) + (3.0 * locals.var_tmf2_dn7)) + (4.0 * locals.var_tmf3_dn7))) * locals.var_tx) + (assign22290_e17168 * locals.var_tx_dn7)) * locals.var_tx) + (assign22290_e17170 * locals.var_tx_dn7));
        locals.var_t0_dn8 = (((((-(((2.0 * locals.var_tmf1_dn8) + (3.0 * locals.var_tmf2_dn8)) + (4.0 * locals.var_tmf3_dn8))) * locals.var_tx) + (assign22290_e17168 * locals.var_tx_dn8)) * locals.var_tx) + (assign22290_e17170 * locals.var_tx_dn8));
        locals.var_t0_dn9 = (((((-(((2.0 * locals.var_tmf1_dn9) + (3.0 * locals.var_tmf2_dn9)) + (4.0 * locals.var_tmf3_dn9))) * locals.var_tx) + (assign22290_e17168 * locals.var_tx_dn9)) * locals.var_tx) + (assign22290_e17170 * locals.var_tx_dn9));
        locals.var_t0_dn10 = (((((-(((2.0 * locals.var_tmf1_dn10) + (3.0 * locals.var_tmf2_dn10)) + (4.0 * locals.var_tmf3_dn10))) * locals.var_tx) + (assign22290_e17168 * locals.var_tx_dn10)) * locals.var_tx) + (assign22290_e17170 * locals.var_tx_dn10));
        locals.var_t0_dn13 = (((((-(((2.0 * locals.var_tmf1_dn13) + (3.0 * locals.var_tmf2_dn13)) + (4.0 * locals.var_tmf3_dn13))) * locals.var_tx) + (assign22290_e17168 * locals.var_tx_dn13)) * locals.var_tx) + (assign22290_e17170 * locals.var_tx_dn13));
        locals.var_t0_rv = 0.0;

        let assign22300_e17176: f64 = (1.0 - locals.var_tx);
        let assign22300_e17177: f64 = assign22300_e17176;
        locals.var_tx = assign22300_e17177;
        locals.var_tx_dn0 = (-locals.var_tx_dn0);
        locals.var_tx_dn2 = (-locals.var_tx_dn2);
        locals.var_tx_dn4 = (-locals.var_tx_dn4);
        locals.var_tx_dn5 = (-locals.var_tx_dn5);
        locals.var_tx_dn6 = (-locals.var_tx_dn6);
        locals.var_tx_dn7 = (-locals.var_tx_dn7);
        locals.var_tx_dn8 = (-locals.var_tx_dn8);
        locals.var_tx_dn9 = (-locals.var_tx_dn9);
        locals.var_tx_dn10 = (-locals.var_tx_dn10);
        locals.var_tx_dn13 = (-locals.var_tx_dn13);
        locals.var_tx_rv = 0.0;

        let assign22310_e17179: f64 = (-locals.var_t0);
        locals.var_t0 = assign22310_e17179;
        locals.var_t0_dn0 = (-locals.var_t0_dn0);
        locals.var_t0_dn2 = (-locals.var_t0_dn2);
        locals.var_t0_dn4 = (-locals.var_t0_dn4);
        locals.var_t0_dn5 = (-locals.var_t0_dn5);
        locals.var_t0_dn6 = (-locals.var_t0_dn6);
        locals.var_t0_dn7 = (-locals.var_t0_dn7);
        locals.var_t0_dn8 = (-locals.var_t0_dn8);
        locals.var_t0_dn9 = (-locals.var_t0_dn9);
        locals.var_t0_dn10 = (-locals.var_t0_dn10);
        locals.var_t0_dn13 = (-locals.var_t0_dn13);
        locals.var_t0_rv = 0.0;

        let assign22320_e17182: f64 = (locals.var_tx * locals.var_tx);
        locals.var_fmdvds = assign22320_e17182;
        locals.var_fmdvds_dn0 = ((locals.var_tx_dn0 * locals.var_tx) + (locals.var_tx * locals.var_tx_dn0));
        locals.var_fmdvds_dn2 = ((locals.var_tx_dn2 * locals.var_tx) + (locals.var_tx * locals.var_tx_dn2));
        locals.var_fmdvds_dn4 = ((locals.var_tx_dn4 * locals.var_tx) + (locals.var_tx * locals.var_tx_dn4));
        locals.var_fmdvds_dn5 = ((locals.var_tx_dn5 * locals.var_tx) + (locals.var_tx * locals.var_tx_dn5));
        locals.var_fmdvds_dn6 = ((locals.var_tx_dn6 * locals.var_tx) + (locals.var_tx * locals.var_tx_dn6));
        locals.var_fmdvds_dn7 = ((locals.var_tx_dn7 * locals.var_tx) + (locals.var_tx * locals.var_tx_dn7));
        locals.var_fmdvds_dn8 = ((locals.var_tx_dn8 * locals.var_tx) + (locals.var_tx * locals.var_tx_dn8));
        locals.var_fmdvds_dn9 = ((locals.var_tx_dn9 * locals.var_tx) + (locals.var_tx * locals.var_tx_dn9));
        locals.var_fmdvds_dn10 = ((locals.var_tx_dn10 * locals.var_tx) + (locals.var_tx * locals.var_tx_dn10));
        locals.var_fmdvds_dn13 = ((locals.var_tx_dn13 * locals.var_tx) + (locals.var_tx * locals.var_tx_dn13));
        locals.var_fmdvds_rv = 0.0;

        let assign22330_e17185: f64 = if locals.var_flg_qmetemp == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard425 = assign22330_e17185;
        locals.var_guard425_rv = 0.0;

        let (assign22340_e17189,) = {
    if (locals.var_guard425 != 0.0) {
        (0.0,)
    } else {
        (locals.var_flg_qme,)
    }
};
        locals.var_flg_qme = assign22340_e17189;
        locals.var_flg_qme_rv = 0.0;

        let (assign22350_e17194,) = {
    if (locals.var_guard425 == 0.0) {
        (1.0,)
    } else {
        (locals.var_flg_qme,)
    }
};
        locals.var_flg_qme = assign22350_e17194;
        locals.var_flg_qme_rv = 0.0;

        locals.var_t1 = locals.var_qnsub_esi2;
        locals.var_t1_dn0 = locals.var_qnsub_esi2_dn0;
        locals.var_t1_dn2 = locals.var_qnsub_esi2_dn2;
        locals.var_t1_dn4 = locals.var_qnsub_esi2_dn4;
        locals.var_t1_dn5 = locals.var_qnsub_esi2_dn5;
        locals.var_t1_dn6 = locals.var_qnsub_esi2_dn6;
        locals.var_t1_dn7 = locals.var_qnsub_esi2_dn7;
        locals.var_t1_dn8 = locals.var_qnsub_esi2_dn8;
        locals.var_t1_dn9 = locals.var_qnsub_esi2_dn9;
        locals.var_t1_dn10 = locals.var_qnsub_esi2_dn10;
        locals.var_t1_dn13 = locals.var_qnsub_esi2_dn13;
        locals.var_t1_rv = 0.0;

        let assign22370_e17198: f64 = (locals.var_t1 * locals.var_pb20);
        let assign22370_e17199: f64 = (assign22370_e17198).sqrt();
        locals.var_t2 = assign22370_e17199;
        locals.var_t2_dn0 = (((locals.var_t1_dn0 * locals.var_pb20) + (locals.var_t1 * locals.var_pb20_dn0)) / (2.0 * assign22370_e17199));
        locals.var_t2_dn2 = (((locals.var_t1_dn2 * locals.var_pb20) + (locals.var_t1 * locals.var_pb20_dn2)) / (2.0 * assign22370_e17199));
        locals.var_t2_dn4 = (((locals.var_t1_dn4 * locals.var_pb20) + (locals.var_t1 * locals.var_pb20_dn4)) / (2.0 * assign22370_e17199));
        locals.var_t2_dn5 = (((locals.var_t1_dn5 * locals.var_pb20) + (locals.var_t1 * locals.var_pb20_dn5)) / (2.0 * assign22370_e17199));
        locals.var_t2_dn6 = (((locals.var_t1_dn6 * locals.var_pb20) + (locals.var_t1 * locals.var_pb20_dn6)) / (2.0 * assign22370_e17199));
        locals.var_t2_dn7 = (((locals.var_t1_dn7 * locals.var_pb20) + (locals.var_t1 * locals.var_pb20_dn7)) / (2.0 * assign22370_e17199));
        locals.var_t2_dn8 = (((locals.var_t1_dn8 * locals.var_pb20) + (locals.var_t1 * locals.var_pb20_dn8)) / (2.0 * assign22370_e17199));
        locals.var_t2_dn9 = (((locals.var_t1_dn9 * locals.var_pb20) + (locals.var_t1 * locals.var_pb20_dn9)) / (2.0 * assign22370_e17199));
        locals.var_t2_dn10 = (((locals.var_t1_dn10 * locals.var_pb20) + (locals.var_t1 * locals.var_pb20_dn10)) / (2.0 * assign22370_e17199));
        locals.var_t2_dn13 = (((locals.var_t1_dn13 * locals.var_pb20) + (locals.var_t1 * locals.var_pb20_dn13)) / (2.0 * assign22370_e17199));
        locals.var_t2_rv = 0.0;

        let assign22380_e17202: f64 = (locals.var_pb20 + locals.var_vfb);
        let assign22380_e17205: f64 = (locals.var_t2 * locals.var_cox0_inv);
        let assign22380_e17206: f64 = (assign22380_e17202 + assign22380_e17205);
        locals.var_vthq = assign22380_e17206;
        locals.var_vthq_dn0 = (locals.var_pb20_dn0 + (locals.var_t2_dn0 * locals.var_cox0_inv));
        locals.var_vthq_dn2 = (locals.var_pb20_dn2 + (locals.var_t2_dn2 * locals.var_cox0_inv));
        locals.var_vthq_dn4 = (locals.var_pb20_dn4 + (locals.var_t2_dn4 * locals.var_cox0_inv));
        locals.var_vthq_dn5 = (locals.var_pb20_dn5 + (locals.var_t2_dn5 * locals.var_cox0_inv));
        locals.var_vthq_dn6 = (locals.var_pb20_dn6 + (locals.var_t2_dn6 * locals.var_cox0_inv));
        locals.var_vthq_dn7 = (locals.var_pb20_dn7 + (locals.var_t2_dn7 * locals.var_cox0_inv));
        locals.var_vthq_dn8 = (locals.var_pb20_dn8 + (locals.var_t2_dn8 * locals.var_cox0_inv));
        locals.var_vthq_dn9 = (locals.var_pb20_dn9 + (locals.var_t2_dn9 * locals.var_cox0_inv));
        locals.var_vthq_dn10 = (locals.var_pb20_dn10 + (locals.var_t2_dn10 * locals.var_cox0_inv));
        locals.var_vthq_dn13 = (locals.var_pb20_dn13 + (locals.var_t2_dn13 * locals.var_cox0_inv));
        locals.var_vthq_rv = 0.0;

        let assign22390_e17209: f64 = if locals.var_flg_qme == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard426 = assign22390_e17209;
        locals.var_guard426_rv = 0.0;

        let (assign22400_e17213, assign22400_e17213_d_n0, assign22400_e17213_d_n2, assign22400_e17213_d_n4, assign22400_e17213_d_n5, assign22400_e17213_d_n6, assign22400_e17213_d_n7, assign22400_e17213_d_n8, assign22400_e17213_d_n9, assign22400_e17213_d_n10, assign22400_e17213_d_n13,) = {
    if (locals.var_guard426 != 0.0) {
        (locals.var_tox0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_toxe, locals.var_toxe_dn0, locals.var_toxe_dn2, locals.var_toxe_dn4, locals.var_toxe_dn5, locals.var_toxe_dn6, locals.var_toxe_dn7, locals.var_toxe_dn8, locals.var_toxe_dn9, locals.var_toxe_dn10, locals.var_toxe_dn13,)
    }
};
        locals.var_toxe = assign22400_e17213;
        locals.var_toxe_dn0 = assign22400_e17213_d_n0;
        locals.var_toxe_dn2 = assign22400_e17213_d_n2;
        locals.var_toxe_dn4 = assign22400_e17213_d_n4;
        locals.var_toxe_dn5 = assign22400_e17213_d_n5;
        locals.var_toxe_dn6 = assign22400_e17213_d_n6;
        locals.var_toxe_dn7 = assign22400_e17213_d_n7;
        locals.var_toxe_dn8 = assign22400_e17213_d_n8;
        locals.var_toxe_dn9 = assign22400_e17213_d_n9;
        locals.var_toxe_dn10 = assign22400_e17213_d_n10;
        locals.var_toxe_dn13 = assign22400_e17213_d_n13;
        locals.var_toxe_rv = 0.0;

        let (assign22410_e17217, assign22410_e17217_d_n0, assign22410_e17217_d_n2, assign22410_e17217_d_n4, assign22410_e17217_d_n5, assign22410_e17217_d_n6, assign22410_e17217_d_n7, assign22410_e17217_d_n8, assign22410_e17217_d_n9, assign22410_e17217_d_n10, assign22410_e17217_d_n13,) = {
    if (locals.var_guard426 != 0.0) {
        (locals.var_cox0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_cox, locals.var_cox_dn0, locals.var_cox_dn2, locals.var_cox_dn4, locals.var_cox_dn5, locals.var_cox_dn6, locals.var_cox_dn7, locals.var_cox_dn8, locals.var_cox_dn9, locals.var_cox_dn10, locals.var_cox_dn13,)
    }
};
        locals.var_cox = assign22410_e17217;
        locals.var_cox_dn0 = assign22410_e17217_d_n0;
        locals.var_cox_dn2 = assign22410_e17217_d_n2;
        locals.var_cox_dn4 = assign22410_e17217_d_n4;
        locals.var_cox_dn5 = assign22410_e17217_d_n5;
        locals.var_cox_dn6 = assign22410_e17217_d_n6;
        locals.var_cox_dn7 = assign22410_e17217_d_n7;
        locals.var_cox_dn8 = assign22410_e17217_d_n8;
        locals.var_cox_dn9 = assign22410_e17217_d_n9;
        locals.var_cox_dn10 = assign22410_e17217_d_n10;
        locals.var_cox_dn13 = assign22410_e17217_d_n13;
        locals.var_cox_rv = 0.0;

        let (assign22420_e17221, assign22420_e17221_d_n0, assign22420_e17221_d_n2, assign22420_e17221_d_n4, assign22420_e17221_d_n5, assign22420_e17221_d_n6, assign22420_e17221_d_n7, assign22420_e17221_d_n8, assign22420_e17221_d_n9, assign22420_e17221_d_n10, assign22420_e17221_d_n13,) = {
    if (locals.var_guard426 != 0.0) {
        (locals.var_cox0_inv, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_cox_inv, locals.var_cox_inv_dn0, locals.var_cox_inv_dn2, locals.var_cox_inv_dn4, locals.var_cox_inv_dn5, locals.var_cox_inv_dn6, locals.var_cox_inv_dn7, locals.var_cox_inv_dn8, locals.var_cox_inv_dn9, locals.var_cox_inv_dn10, locals.var_cox_inv_dn13,)
    }
};
        locals.var_cox_inv = assign22420_e17221;
        locals.var_cox_inv_dn0 = assign22420_e17221_d_n0;
        locals.var_cox_inv_dn2 = assign22420_e17221_d_n2;
        locals.var_cox_inv_dn4 = assign22420_e17221_d_n4;
        locals.var_cox_inv_dn5 = assign22420_e17221_d_n5;
        locals.var_cox_inv_dn6 = assign22420_e17221_d_n6;
        locals.var_cox_inv_dn7 = assign22420_e17221_d_n7;
        locals.var_cox_inv_dn8 = assign22420_e17221_d_n8;
        locals.var_cox_inv_dn9 = assign22420_e17221_d_n9;
        locals.var_cox_inv_dn10 = assign22420_e17221_d_n10;
        locals.var_cox_inv_dn13 = assign22420_e17221_d_n13;
        locals.var_cox_inv_rv = 0.0;

        let (assign22430_e17229, assign22430_e17229_d_n0, assign22430_e17229_d_n2, assign22430_e17229_d_n4, assign22430_e17229_d_n5, assign22430_e17229_d_n6, assign22430_e17229_d_n7, assign22430_e17229_d_n8, assign22430_e17229_d_n9, assign22430_e17229_d_n10, assign22430_e17229_d_n13,) = {
    if (locals.var_guard426 != 0.0) {
        let assign22430_e17225: f64 = (locals.var_cnst0 * locals.var_cnst0);
        let assign22430_e17227: f64 = (assign22430_e17225 * locals.var_cox_inv);
        (assign22430_e17227, ((((locals.var_cnst0_dn0 * locals.var_cnst0) + (locals.var_cnst0 * locals.var_cnst0_dn0)) * locals.var_cox_inv) + (assign22430_e17225 * locals.var_cox_inv_dn0)), ((((locals.var_cnst0_dn2 * locals.var_cnst0) + (locals.var_cnst0 * locals.var_cnst0_dn2)) * locals.var_cox_inv) + (assign22430_e17225 * locals.var_cox_inv_dn2)), ((((locals.var_cnst0_dn4 * locals.var_cnst0) + (locals.var_cnst0 * locals.var_cnst0_dn4)) * locals.var_cox_inv) + (assign22430_e17225 * locals.var_cox_inv_dn4)), ((((locals.var_cnst0_dn5 * locals.var_cnst0) + (locals.var_cnst0 * locals.var_cnst0_dn5)) * locals.var_cox_inv) + (assign22430_e17225 * locals.var_cox_inv_dn5)), ((((locals.var_cnst0_dn6 * locals.var_cnst0) + (locals.var_cnst0 * locals.var_cnst0_dn6)) * locals.var_cox_inv) + (assign22430_e17225 * locals.var_cox_inv_dn6)), ((((locals.var_cnst0_dn7 * locals.var_cnst0) + (locals.var_cnst0 * locals.var_cnst0_dn7)) * locals.var_cox_inv) + (assign22430_e17225 * locals.var_cox_inv_dn7)), ((((locals.var_cnst0_dn8 * locals.var_cnst0) + (locals.var_cnst0 * locals.var_cnst0_dn8)) * locals.var_cox_inv) + (assign22430_e17225 * locals.var_cox_inv_dn8)), ((((locals.var_cnst0_dn9 * locals.var_cnst0) + (locals.var_cnst0 * locals.var_cnst0_dn9)) * locals.var_cox_inv) + (assign22430_e17225 * locals.var_cox_inv_dn9)), ((((locals.var_cnst0_dn10 * locals.var_cnst0) + (locals.var_cnst0 * locals.var_cnst0_dn10)) * locals.var_cox_inv) + (assign22430_e17225 * locals.var_cox_inv_dn10)), ((((locals.var_cnst0_dn13 * locals.var_cnst0) + (locals.var_cnst0 * locals.var_cnst0_dn13)) * locals.var_cox_inv) + (assign22430_e17225 * locals.var_cox_inv_dn13)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign22430_e17229;
        locals.var_t0_dn0 = assign22430_e17229_d_n0;
        locals.var_t0_dn2 = assign22430_e17229_d_n2;
        locals.var_t0_dn4 = assign22430_e17229_d_n4;
        locals.var_t0_dn5 = assign22430_e17229_d_n5;
        locals.var_t0_dn6 = assign22430_e17229_d_n6;
        locals.var_t0_dn7 = assign22430_e17229_d_n7;
        locals.var_t0_dn8 = assign22430_e17229_d_n8;
        locals.var_t0_dn9 = assign22430_e17229_d_n9;
        locals.var_t0_dn10 = assign22430_e17229_d_n10;
        locals.var_t0_dn13 = assign22430_e17229_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign22440_e17235, assign22440_e17235_d_n0, assign22440_e17235_d_n2, assign22440_e17235_d_n4, assign22440_e17235_d_n5, assign22440_e17235_d_n6, assign22440_e17235_d_n7, assign22440_e17235_d_n8, assign22440_e17235_d_n9, assign22440_e17235_d_n10, assign22440_e17235_d_n13,) = {
    if (locals.var_guard426 != 0.0) {
        let assign22440_e17233: f64 = (locals.var_t0 * locals.var_cox_inv);
        (assign22440_e17233, ((locals.var_t0_dn0 * locals.var_cox_inv) + (locals.var_t0 * locals.var_cox_inv_dn0)), ((locals.var_t0_dn2 * locals.var_cox_inv) + (locals.var_t0 * locals.var_cox_inv_dn2)), ((locals.var_t0_dn4 * locals.var_cox_inv) + (locals.var_t0 * locals.var_cox_inv_dn4)), ((locals.var_t0_dn5 * locals.var_cox_inv) + (locals.var_t0 * locals.var_cox_inv_dn5)), ((locals.var_t0_dn6 * locals.var_cox_inv) + (locals.var_t0 * locals.var_cox_inv_dn6)), ((locals.var_t0_dn7 * locals.var_cox_inv) + (locals.var_t0 * locals.var_cox_inv_dn7)), ((locals.var_t0_dn8 * locals.var_cox_inv) + (locals.var_t0 * locals.var_cox_inv_dn8)), ((locals.var_t0_dn9 * locals.var_cox_inv) + (locals.var_t0 * locals.var_cox_inv_dn9)), ((locals.var_t0_dn10 * locals.var_cox_inv) + (locals.var_t0 * locals.var_cox_inv_dn10)), ((locals.var_t0_dn13 * locals.var_cox_inv) + (locals.var_t0 * locals.var_cox_inv_dn13)),)
    } else {
        (locals.var_cnstcoxi, locals.var_cnstcoxi_dn0, locals.var_cnstcoxi_dn2, locals.var_cnstcoxi_dn4, locals.var_cnstcoxi_dn5, locals.var_cnstcoxi_dn6, locals.var_cnstcoxi_dn7, locals.var_cnstcoxi_dn8, locals.var_cnstcoxi_dn9, locals.var_cnstcoxi_dn10, locals.var_cnstcoxi_dn13,)
    }
};
        locals.var_cnstcoxi = assign22440_e17235;
        locals.var_cnstcoxi_dn0 = assign22440_e17235_d_n0;
        locals.var_cnstcoxi_dn2 = assign22440_e17235_d_n2;
        locals.var_cnstcoxi_dn4 = assign22440_e17235_d_n4;
        locals.var_cnstcoxi_dn5 = assign22440_e17235_d_n5;
        locals.var_cnstcoxi_dn6 = assign22440_e17235_d_n6;
        locals.var_cnstcoxi_dn7 = assign22440_e17235_d_n7;
        locals.var_cnstcoxi_dn8 = assign22440_e17235_d_n8;
        locals.var_cnstcoxi_dn9 = assign22440_e17235_d_n9;
        locals.var_cnstcoxi_dn10 = assign22440_e17235_d_n10;
        locals.var_cnstcoxi_dn13 = assign22440_e17235_d_n13;
        locals.var_cnstcoxi_rv = 0.0;

        let (assign22450_e17246, assign22450_e17246_d_n0, assign22450_e17246_d_n2, assign22450_e17246_d_n4, assign22450_e17246_d_n5, assign22450_e17246_d_n6, assign22450_e17246_d_n7, assign22450_e17246_d_n8, assign22450_e17246_d_n9, assign22450_e17246_d_n10, assign22450_e17246_d_n13,) = {
    if (locals.var_guard426 == 0.0) {
        let assign22450_e17240: f64 = (locals.var_vgs - locals.var_vbs);
        let assign22450_e17242: f64 = (assign22450_e17240 - locals.var_vthq);
        let assign22450_e17244: f64 = (assign22450_e17242 + p.p236);
        (assign22450_e17244, (-locals.var_vthq_dn0), (-locals.var_vthq_dn2), (-locals.var_vthq_dn4), ((locals.var_vgs_dn5 - locals.var_vbs_dn5) - locals.var_vthq_dn5), (locals.var_vgs_dn6 - locals.var_vthq_dn6), ((locals.var_vgs_dn7 - locals.var_vbs_dn7) - locals.var_vthq_dn7), ((-locals.var_vbs_dn8) - locals.var_vthq_dn8), (-locals.var_vthq_dn9), (-locals.var_vthq_dn10), (-locals.var_vthq_dn13),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn13,)
    }
};
        locals.var_t5 = assign22450_e17246;
        locals.var_t5_dn0 = assign22450_e17246_d_n0;
        locals.var_t5_dn2 = assign22450_e17246_d_n2;
        locals.var_t5_dn4 = assign22450_e17246_d_n4;
        locals.var_t5_dn5 = assign22450_e17246_d_n5;
        locals.var_t5_dn6 = assign22450_e17246_d_n6;
        locals.var_t5_dn7 = assign22450_e17246_d_n7;
        locals.var_t5_dn8 = assign22450_e17246_d_n8;
        locals.var_t5_dn9 = assign22450_e17246_d_n9;
        locals.var_t5_dn10 = assign22450_e17246_d_n10;
        locals.var_t5_dn13 = assign22450_e17246_d_n13;
        locals.var_t5_rv = 0.0;

        let (assign22460_e17264, assign22460_e17264_d_n0, assign22460_e17264_d_n2, assign22460_e17264_d_n4, assign22460_e17264_d_n5, assign22460_e17264_d_n6, assign22460_e17264_d_n7, assign22460_e17264_d_n8, assign22460_e17264_d_n9, assign22460_e17264_d_n10, assign22460_e17264_d_n13,) = {
    if (locals.var_guard426 == 0.0) {
        let assign22460_e17251: f64 = (locals.var_t5 * locals.var_t5);
        let assign22460_e17255: f64 = (1e-9 * 0.01);
        let assign22460_e17256: f64 = (4.0 * assign22460_e17255);
        let assign22460_e17259: f64 = (1e-9 * 0.01);
        let assign22460_e17260: f64 = (assign22460_e17256 * assign22460_e17259);
        let assign22460_e17261: f64 = (assign22460_e17251 + assign22460_e17260);
        let assign22460_e17262: f64 = (assign22460_e17261).sqrt();
        (assign22460_e17262, (((locals.var_t5_dn0 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn0)) / (2.0 * assign22460_e17262)), (((locals.var_t5_dn2 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn2)) / (2.0 * assign22460_e17262)), (((locals.var_t5_dn4 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn4)) / (2.0 * assign22460_e17262)), (((locals.var_t5_dn5 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn5)) / (2.0 * assign22460_e17262)), (((locals.var_t5_dn6 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn6)) / (2.0 * assign22460_e17262)), (((locals.var_t5_dn7 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn7)) / (2.0 * assign22460_e17262)), (((locals.var_t5_dn8 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn8)) / (2.0 * assign22460_e17262)), (((locals.var_t5_dn9 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn9)) / (2.0 * assign22460_e17262)), (((locals.var_t5_dn10 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn10)) / (2.0 * assign22460_e17262)), (((locals.var_t5_dn13 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn13)) / (2.0 * assign22460_e17262)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign22460_e17264;
        locals.var_tmf2_dn0 = assign22460_e17264_d_n0;
        locals.var_tmf2_dn2 = assign22460_e17264_d_n2;
        locals.var_tmf2_dn4 = assign22460_e17264_d_n4;
        locals.var_tmf2_dn5 = assign22460_e17264_d_n5;
        locals.var_tmf2_dn6 = assign22460_e17264_d_n6;
        locals.var_tmf2_dn7 = assign22460_e17264_d_n7;
        locals.var_tmf2_dn8 = assign22460_e17264_d_n8;
        locals.var_tmf2_dn9 = assign22460_e17264_d_n9;
        locals.var_tmf2_dn10 = assign22460_e17264_d_n10;
        locals.var_tmf2_dn13 = assign22460_e17264_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign22470_e17275, assign22470_e17275_d_n0, assign22470_e17275_d_n2, assign22470_e17275_d_n4, assign22470_e17275_d_n5, assign22470_e17275_d_n6, assign22470_e17275_d_n7, assign22470_e17275_d_n8, assign22470_e17275_d_n9, assign22470_e17275_d_n10, assign22470_e17275_d_n13,) = {
    if (locals.var_guard426 == 0.0) {
        let assign22470_e17271: f64 = (locals.var_t5 / locals.var_tmf2);
        let assign22470_e17272: f64 = (1.0 + assign22470_e17271);
        let assign22470_e17273: f64 = (0.5 * assign22470_e17272);
        (assign22470_e17273, (0.5 * (((locals.var_t5_dn0 * locals.var_tmf2) - (locals.var_t5 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t5_dn2 * locals.var_tmf2) - (locals.var_t5 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t5_dn4 * locals.var_tmf2) - (locals.var_t5 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t5_dn5 * locals.var_tmf2) - (locals.var_t5 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t5_dn6 * locals.var_tmf2) - (locals.var_t5 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t5_dn7 * locals.var_tmf2) - (locals.var_t5 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t5_dn8 * locals.var_tmf2) - (locals.var_t5 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t5_dn9 * locals.var_tmf2) - (locals.var_t5 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t5_dn10 * locals.var_tmf2) - (locals.var_t5 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t5_dn13 * locals.var_tmf2) - (locals.var_t5 * locals.var_tmf2_dn13)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn13,)
    }
};
        locals.var_t3 = assign22470_e17275;
        locals.var_t3_dn0 = assign22470_e17275_d_n0;
        locals.var_t3_dn2 = assign22470_e17275_d_n2;
        locals.var_t3_dn4 = assign22470_e17275_d_n4;
        locals.var_t3_dn5 = assign22470_e17275_d_n5;
        locals.var_t3_dn6 = assign22470_e17275_d_n6;
        locals.var_t3_dn7 = assign22470_e17275_d_n7;
        locals.var_t3_dn8 = assign22470_e17275_d_n8;
        locals.var_t3_dn9 = assign22470_e17275_d_n9;
        locals.var_t3_dn10 = assign22470_e17275_d_n10;
        locals.var_t3_dn13 = assign22470_e17275_d_n13;
        locals.var_t3_rv = 0.0;

        let (assign22480_e17284, assign22480_e17284_d_n0, assign22480_e17284_d_n2, assign22480_e17284_d_n4, assign22480_e17284_d_n5, assign22480_e17284_d_n6, assign22480_e17284_d_n7, assign22480_e17284_d_n8, assign22480_e17284_d_n9, assign22480_e17284_d_n10, assign22480_e17284_d_n13,) = {
    if (locals.var_guard426 == 0.0) {
        let assign22480_e17281: f64 = (locals.var_t5 + locals.var_tmf2);
        let assign22480_e17282: f64 = (0.5 * assign22480_e17281);
        (assign22480_e17282, (0.5 * (locals.var_t5_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_t5_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_t5_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_t5_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_t5_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_t5_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_t5_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_t5_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_t5_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_t5_dn13 + locals.var_tmf2_dn13)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign22480_e17284;
        locals.var_t2_dn0 = assign22480_e17284_d_n0;
        locals.var_t2_dn2 = assign22480_e17284_d_n2;
        locals.var_t2_dn4 = assign22480_e17284_d_n4;
        locals.var_t2_dn5 = assign22480_e17284_d_n5;
        locals.var_t2_dn6 = assign22480_e17284_d_n6;
        locals.var_t2_dn7 = assign22480_e17284_d_n7;
        locals.var_t2_dn8 = assign22480_e17284_d_n8;
        locals.var_t2_dn9 = assign22480_e17284_d_n9;
        locals.var_t2_dn10 = assign22480_e17284_d_n10;
        locals.var_t2_dn13 = assign22480_e17284_d_n13;
        locals.var_t2_rv = 0.0;

        let assign22490_e17287: f64 = if locals.var_t2 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard427 = assign22490_e17287;
        locals.var_guard427_rv = 0.0;

        let (assign22500_e17294, assign22500_e17294_d_n0, assign22500_e17294_d_n2, assign22500_e17294_d_n4, assign22500_e17294_d_n5, assign22500_e17294_d_n6, assign22500_e17294_d_n7, assign22500_e17294_d_n8, assign22500_e17294_d_n9, assign22500_e17294_d_n10, assign22500_e17294_d_n13,) = {
    if ((locals.var_guard426 == 0.0) && (locals.var_guard427 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign22500_e17294;
        locals.var_t2_dn0 = assign22500_e17294_d_n0;
        locals.var_t2_dn2 = assign22500_e17294_d_n2;
        locals.var_t2_dn4 = assign22500_e17294_d_n4;
        locals.var_t2_dn5 = assign22500_e17294_d_n5;
        locals.var_t2_dn6 = assign22500_e17294_d_n6;
        locals.var_t2_dn7 = assign22500_e17294_d_n7;
        locals.var_t2_dn8 = assign22500_e17294_d_n8;
        locals.var_t2_dn9 = assign22500_e17294_d_n9;
        locals.var_t2_dn10 = assign22500_e17294_d_n10;
        locals.var_t2_dn13 = assign22500_e17294_d_n13;
        locals.var_t2_rv = 0.0;

        let (assign22510_e17301, assign22510_e17301_d_n0, assign22510_e17301_d_n2, assign22510_e17301_d_n4, assign22510_e17301_d_n5, assign22510_e17301_d_n6, assign22510_e17301_d_n7, assign22510_e17301_d_n8, assign22510_e17301_d_n9, assign22510_e17301_d_n10, assign22510_e17301_d_n13,) = {
    if ((locals.var_guard426 == 0.0) && (locals.var_guard427 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn13,)
    }
};
        locals.var_t3 = assign22510_e17301;
        locals.var_t3_dn0 = assign22510_e17301_d_n0;
        locals.var_t3_dn2 = assign22510_e17301_d_n2;
        locals.var_t3_dn4 = assign22510_e17301_d_n4;
        locals.var_t3_dn5 = assign22510_e17301_d_n5;
        locals.var_t3_dn6 = assign22510_e17301_d_n6;
        locals.var_t3_dn7 = assign22510_e17301_d_n7;
        locals.var_t3_dn8 = assign22510_e17301_d_n8;
        locals.var_t3_dn9 = assign22510_e17301_d_n9;
        locals.var_t3_dn10 = assign22510_e17301_d_n10;
        locals.var_t3_dn13 = assign22510_e17301_d_n13;
        locals.var_t3_rv = 0.0;

        let (assign22520_e17308, assign22520_e17308_d_n0, assign22520_e17308_d_n2, assign22520_e17308_d_n4, assign22520_e17308_d_n5, assign22520_e17308_d_n6, assign22520_e17308_d_n7, assign22520_e17308_d_n8, assign22520_e17308_d_n9, assign22520_e17308_d_n10, assign22520_e17308_d_n13,) = {
    if (locals.var_guard426 == 0.0) {
        let assign22520_e17306: f64 = (locals.var_t2 + 1e-25);
        (assign22520_e17306, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign22520_e17308;
        locals.var_t2_dn0 = assign22520_e17308_d_n0;
        locals.var_t2_dn2 = assign22520_e17308_d_n2;
        locals.var_t2_dn4 = assign22520_e17308_d_n4;
        locals.var_t2_dn5 = assign22520_e17308_d_n5;
        locals.var_t2_dn6 = assign22520_e17308_d_n6;
        locals.var_t2_dn7 = assign22520_e17308_d_n7;
        locals.var_t2_dn8 = assign22520_e17308_d_n8;
        locals.var_t2_dn9 = assign22520_e17308_d_n9;
        locals.var_t2_dn10 = assign22520_e17308_d_n10;
        locals.var_t2_dn13 = assign22520_e17308_d_n13;
        locals.var_t2_rv = 0.0;

        let (assign22530_e17315, assign22530_e17315_d_n0, assign22530_e17315_d_n2, assign22530_e17315_d_n4, assign22530_e17315_d_n5, assign22530_e17315_d_n6, assign22530_e17315_d_n7, assign22530_e17315_d_n8, assign22530_e17315_d_n9, assign22530_e17315_d_n10, assign22530_e17315_d_n13,) = {
    if (locals.var_guard426 == 0.0) {
        let assign22530_e17313: f64 = (1.0 / locals.var_t2);
        (assign22530_e17313, (-(locals.var_t2_dn0 / (locals.var_t2 * locals.var_t2))), (-(locals.var_t2_dn2 / (locals.var_t2 * locals.var_t2))), (-(locals.var_t2_dn4 / (locals.var_t2 * locals.var_t2))), (-(locals.var_t2_dn5 / (locals.var_t2 * locals.var_t2))), (-(locals.var_t2_dn6 / (locals.var_t2 * locals.var_t2))), (-(locals.var_t2_dn7 / (locals.var_t2 * locals.var_t2))), (-(locals.var_t2_dn8 / (locals.var_t2 * locals.var_t2))), (-(locals.var_t2_dn9 / (locals.var_t2 * locals.var_t2))), (-(locals.var_t2_dn10 / (locals.var_t2 * locals.var_t2))), (-(locals.var_t2_dn13 / (locals.var_t2 * locals.var_t2))),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn13,)
    }
};
        locals.var_t3 = assign22530_e17315;
        locals.var_t3_dn0 = assign22530_e17315_d_n0;
        locals.var_t3_dn2 = assign22530_e17315_d_n2;
        locals.var_t3_dn4 = assign22530_e17315_d_n4;
        locals.var_t3_dn5 = assign22530_e17315_d_n5;
        locals.var_t3_dn6 = assign22530_e17315_d_n6;
        locals.var_t3_dn7 = assign22530_e17315_d_n7;
        locals.var_t3_dn8 = assign22530_e17315_d_n8;
        locals.var_t3_dn9 = assign22530_e17315_d_n9;
        locals.var_t3_dn10 = assign22530_e17315_d_n10;
        locals.var_t3_dn13 = assign22530_e17315_d_n13;
        locals.var_t3_rv = 0.0;

        let (assign22540_e17325, assign22540_e17325_d_n0, assign22540_e17325_d_n2, assign22540_e17325_d_n4, assign22540_e17325_d_n5, assign22540_e17325_d_n6, assign22540_e17325_d_n7, assign22540_e17325_d_n8, assign22540_e17325_d_n9, assign22540_e17325_d_n10, assign22540_e17325_d_n13,) = {
    if (locals.var_guard426 == 0.0) {
        let assign22540_e17319: f64 = (-1.0);
        let assign22540_e17322: f64 = (locals.var_t2 * locals.var_t2);
        let assign22540_e17323: f64 = (assign22540_e17319 / assign22540_e17322);
        (assign22540_e17323, (-((assign22540_e17319 * ((locals.var_t2_dn0 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn0))) / (assign22540_e17322 * assign22540_e17322))), (-((assign22540_e17319 * ((locals.var_t2_dn2 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn2))) / (assign22540_e17322 * assign22540_e17322))), (-((assign22540_e17319 * ((locals.var_t2_dn4 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn4))) / (assign22540_e17322 * assign22540_e17322))), (-((assign22540_e17319 * ((locals.var_t2_dn5 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn5))) / (assign22540_e17322 * assign22540_e17322))), (-((assign22540_e17319 * ((locals.var_t2_dn6 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn6))) / (assign22540_e17322 * assign22540_e17322))), (-((assign22540_e17319 * ((locals.var_t2_dn7 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn7))) / (assign22540_e17322 * assign22540_e17322))), (-((assign22540_e17319 * ((locals.var_t2_dn8 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn8))) / (assign22540_e17322 * assign22540_e17322))), (-((assign22540_e17319 * ((locals.var_t2_dn9 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn9))) / (assign22540_e17322 * assign22540_e17322))), (-((assign22540_e17319 * ((locals.var_t2_dn10 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn10))) / (assign22540_e17322 * assign22540_e17322))), (-((assign22540_e17319 * ((locals.var_t2_dn13 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn13))) / (assign22540_e17322 * assign22540_e17322))),)
    } else {
        (locals.var_t7, locals.var_t7_dn0, locals.var_t7_dn2, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn13,)
    }
};
        locals.var_t7 = assign22540_e17325;
        locals.var_t7_dn0 = assign22540_e17325_d_n0;
        locals.var_t7_dn2 = assign22540_e17325_d_n2;
        locals.var_t7_dn4 = assign22540_e17325_d_n4;
        locals.var_t7_dn5 = assign22540_e17325_d_n5;
        locals.var_t7_dn6 = assign22540_e17325_d_n6;
        locals.var_t7_dn7 = assign22540_e17325_d_n7;
        locals.var_t7_dn8 = assign22540_e17325_d_n8;
        locals.var_t7_dn9 = assign22540_e17325_d_n9;
        locals.var_t7_dn10 = assign22540_e17325_d_n10;
        locals.var_t7_dn13 = assign22540_e17325_d_n13;
        locals.var_t7_rv = 0.0;

        let (assign22550_e17333, assign22550_e17333_d_n0, assign22550_e17333_d_n2, assign22550_e17333_d_n4, assign22550_e17333_d_n5, assign22550_e17333_d_n6, assign22550_e17333_d_n7, assign22550_e17333_d_n8, assign22550_e17333_d_n9, assign22550_e17333_d_n10, assign22550_e17333_d_n13,) = {
    if (locals.var_guard426 == 0.0) {
        let assign22550_e17330: f64 = (locals.var_vthq).abs();
        let assign22550_e17331: f64 = (2.0 * assign22550_e17330);
        (assign22550_e17331, (2.0 * if locals.var_vthq >= 0.0 { locals.var_vthq_dn0 } else { (-locals.var_vthq_dn0) }), (2.0 * if locals.var_vthq >= 0.0 { locals.var_vthq_dn2 } else { (-locals.var_vthq_dn2) }), (2.0 * if locals.var_vthq >= 0.0 { locals.var_vthq_dn4 } else { (-locals.var_vthq_dn4) }), (2.0 * if locals.var_vthq >= 0.0 { locals.var_vthq_dn5 } else { (-locals.var_vthq_dn5) }), (2.0 * if locals.var_vthq >= 0.0 { locals.var_vthq_dn6 } else { (-locals.var_vthq_dn6) }), (2.0 * if locals.var_vthq >= 0.0 { locals.var_vthq_dn7 } else { (-locals.var_vthq_dn7) }), (2.0 * if locals.var_vthq >= 0.0 { locals.var_vthq_dn8 } else { (-locals.var_vthq_dn8) }), (2.0 * if locals.var_vthq >= 0.0 { locals.var_vthq_dn9 } else { (-locals.var_vthq_dn9) }), (2.0 * if locals.var_vthq >= 0.0 { locals.var_vthq_dn10 } else { (-locals.var_vthq_dn10) }), (2.0 * if locals.var_vthq >= 0.0 { locals.var_vthq_dn13 } else { (-locals.var_vthq_dn13) }),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn13,)
    }
};
        locals.var_t4 = assign22550_e17333;
        locals.var_t4_dn0 = assign22550_e17333_d_n0;
        locals.var_t4_dn2 = assign22550_e17333_d_n2;
        locals.var_t4_dn4 = assign22550_e17333_d_n4;
        locals.var_t4_dn5 = assign22550_e17333_d_n5;
        locals.var_t4_dn6 = assign22550_e17333_d_n6;
        locals.var_t4_dn7 = assign22550_e17333_d_n7;
        locals.var_t4_dn8 = assign22550_e17333_d_n8;
        locals.var_t4_dn9 = assign22550_e17333_d_n9;
        locals.var_t4_dn10 = assign22550_e17333_d_n10;
        locals.var_t4_dn13 = assign22550_e17333_d_n13;
        locals.var_t4_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_58(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign22560_e17342, assign22560_e17342_d_n0, assign22560_e17342_d_n2, assign22560_e17342_d_n4, assign22560_e17342_d_n5, assign22560_e17342_d_n6, assign22560_e17342_d_n7, assign22560_e17342_d_n8, assign22560_e17342_d_n9, assign22560_e17342_d_n10, assign22560_e17342_d_n13,) = {
    if (locals.var_guard426 == 0.0) {
        let assign22560_e17338: f64 = (locals.var_t5 - locals.var_vgs);
        let assign22560_e17340: f64 = (assign22560_e17338 + locals.var_vfb);
        (assign22560_e17340, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, (locals.var_t5_dn5 - locals.var_vgs_dn5), (locals.var_t5_dn6 - locals.var_vgs_dn6), (locals.var_t5_dn7 - locals.var_vgs_dn7), locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn13,)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn13,)
    }
};
        locals.var_t6 = assign22560_e17342;
        locals.var_t6_dn0 = assign22560_e17342_d_n0;
        locals.var_t6_dn2 = assign22560_e17342_d_n2;
        locals.var_t6_dn4 = assign22560_e17342_d_n4;
        locals.var_t6_dn5 = assign22560_e17342_d_n5;
        locals.var_t6_dn6 = assign22560_e17342_d_n6;
        locals.var_t6_dn7 = assign22560_e17342_d_n7;
        locals.var_t6_dn8 = assign22560_e17342_d_n8;
        locals.var_t6_dn9 = assign22560_e17342_d_n9;
        locals.var_t6_dn10 = assign22560_e17342_d_n10;
        locals.var_t6_dn13 = assign22560_e17342_d_n13;
        locals.var_t6_rv = 0.0;

        let assign22570_e17345: f64 = if locals.var_t6 > locals.var_t4 { 1.0 } else { 0.0 };
        locals.var_guard428 = assign22570_e17345;
        locals.var_guard428_rv = 0.0;

        let (assign22580_e17352, assign22580_e17352_d_n0, assign22580_e17352_d_n2, assign22580_e17352_d_n4, assign22580_e17352_d_n5, assign22580_e17352_d_n6, assign22580_e17352_d_n7, assign22580_e17352_d_n8, assign22580_e17352_d_n9, assign22580_e17352_d_n10, assign22580_e17352_d_n13,) = {
    if ((locals.var_guard426 == 0.0) && (locals.var_guard428 != 0.0)) {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn13,)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn13,)
    }
};
        locals.var_t4 = assign22580_e17352;
        locals.var_t4_dn0 = assign22580_e17352_d_n0;
        locals.var_t4_dn2 = assign22580_e17352_d_n2;
        locals.var_t4_dn4 = assign22580_e17352_d_n4;
        locals.var_t4_dn5 = assign22580_e17352_d_n5;
        locals.var_t4_dn6 = assign22580_e17352_d_n6;
        locals.var_t4_dn7 = assign22580_e17352_d_n7;
        locals.var_t4_dn8 = assign22580_e17352_d_n8;
        locals.var_t4_dn9 = assign22580_e17352_d_n9;
        locals.var_t4_dn10 = assign22580_e17352_d_n10;
        locals.var_t4_dn13 = assign22580_e17352_d_n13;
        locals.var_t4_rv = 0.0;

        let (assign22590_e17365, assign22590_e17365_d_n0, assign22590_e17365_d_n2, assign22590_e17365_d_n4, assign22590_e17365_d_n5, assign22590_e17365_d_n6, assign22590_e17365_d_n7, assign22590_e17365_d_n8, assign22590_e17365_d_n9, assign22590_e17365_d_n10, assign22590_e17365_d_n13,) = {
    if (locals.var_guard426 == 0.0) {
        let assign22590_e17357: f64 = (1.0 / locals.var_t4);
        let assign22590_e17359: f64 = (assign22590_e17357 - locals.var_t3);
        let assign22590_e17362: f64 = (1e-9 * 0.01);
        let assign22590_e17363: f64 = (assign22590_e17359 - assign22590_e17362);
        (assign22590_e17363, ((-(locals.var_t4_dn0 / (locals.var_t4 * locals.var_t4))) - locals.var_t3_dn0), ((-(locals.var_t4_dn2 / (locals.var_t4 * locals.var_t4))) - locals.var_t3_dn2), ((-(locals.var_t4_dn4 / (locals.var_t4 * locals.var_t4))) - locals.var_t3_dn4), ((-(locals.var_t4_dn5 / (locals.var_t4 * locals.var_t4))) - locals.var_t3_dn5), ((-(locals.var_t4_dn6 / (locals.var_t4 * locals.var_t4))) - locals.var_t3_dn6), ((-(locals.var_t4_dn7 / (locals.var_t4 * locals.var_t4))) - locals.var_t3_dn7), ((-(locals.var_t4_dn8 / (locals.var_t4 * locals.var_t4))) - locals.var_t3_dn8), ((-(locals.var_t4_dn9 / (locals.var_t4 * locals.var_t4))) - locals.var_t3_dn9), ((-(locals.var_t4_dn10 / (locals.var_t4 * locals.var_t4))) - locals.var_t3_dn10), ((-(locals.var_t4_dn13 / (locals.var_t4 * locals.var_t4))) - locals.var_t3_dn13),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13,)
    }
};
        locals.var_tmf1 = assign22590_e17365;
        locals.var_tmf1_dn0 = assign22590_e17365_d_n0;
        locals.var_tmf1_dn2 = assign22590_e17365_d_n2;
        locals.var_tmf1_dn4 = assign22590_e17365_d_n4;
        locals.var_tmf1_dn5 = assign22590_e17365_d_n5;
        locals.var_tmf1_dn6 = assign22590_e17365_d_n6;
        locals.var_tmf1_dn7 = assign22590_e17365_d_n7;
        locals.var_tmf1_dn8 = assign22590_e17365_d_n8;
        locals.var_tmf1_dn9 = assign22590_e17365_d_n9;
        locals.var_tmf1_dn10 = assign22590_e17365_d_n10;
        locals.var_tmf1_dn13 = assign22590_e17365_d_n13;
        locals.var_tmf1_rv = 0.0;

        let (assign22600_e17378, assign22600_e17378_d_n0, assign22600_e17378_d_n2, assign22600_e17378_d_n4, assign22600_e17378_d_n5, assign22600_e17378_d_n6, assign22600_e17378_d_n7, assign22600_e17378_d_n8, assign22600_e17378_d_n9, assign22600_e17378_d_n10, assign22600_e17378_d_n13,) = {
    if (locals.var_guard426 == 0.0) {
        let assign22600_e17371: f64 = (1.0 / locals.var_t4);
        let assign22600_e17372: f64 = (4.0 * assign22600_e17371);
        let assign22600_e17375: f64 = (1e-9 * 0.01);
        let assign22600_e17376: f64 = (assign22600_e17372 * assign22600_e17375);
        (assign22600_e17376, ((4.0 * (-(locals.var_t4_dn0 / (locals.var_t4 * locals.var_t4)))) * assign22600_e17375), ((4.0 * (-(locals.var_t4_dn2 / (locals.var_t4 * locals.var_t4)))) * assign22600_e17375), ((4.0 * (-(locals.var_t4_dn4 / (locals.var_t4 * locals.var_t4)))) * assign22600_e17375), ((4.0 * (-(locals.var_t4_dn5 / (locals.var_t4 * locals.var_t4)))) * assign22600_e17375), ((4.0 * (-(locals.var_t4_dn6 / (locals.var_t4 * locals.var_t4)))) * assign22600_e17375), ((4.0 * (-(locals.var_t4_dn7 / (locals.var_t4 * locals.var_t4)))) * assign22600_e17375), ((4.0 * (-(locals.var_t4_dn8 / (locals.var_t4 * locals.var_t4)))) * assign22600_e17375), ((4.0 * (-(locals.var_t4_dn9 / (locals.var_t4 * locals.var_t4)))) * assign22600_e17375), ((4.0 * (-(locals.var_t4_dn10 / (locals.var_t4 * locals.var_t4)))) * assign22600_e17375), ((4.0 * (-(locals.var_t4_dn13 / (locals.var_t4 * locals.var_t4)))) * assign22600_e17375),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign22600_e17378;
        locals.var_tmf2_dn0 = assign22600_e17378_d_n0;
        locals.var_tmf2_dn2 = assign22600_e17378_d_n2;
        locals.var_tmf2_dn4 = assign22600_e17378_d_n4;
        locals.var_tmf2_dn5 = assign22600_e17378_d_n5;
        locals.var_tmf2_dn6 = assign22600_e17378_d_n6;
        locals.var_tmf2_dn7 = assign22600_e17378_d_n7;
        locals.var_tmf2_dn8 = assign22600_e17378_d_n8;
        locals.var_tmf2_dn9 = assign22600_e17378_d_n9;
        locals.var_tmf2_dn10 = assign22600_e17378_d_n10;
        locals.var_tmf2_dn13 = assign22600_e17378_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign22610_e17389, assign22610_e17389_d_n0, assign22610_e17389_d_n2, assign22610_e17389_d_n4, assign22610_e17389_d_n5, assign22610_e17389_d_n6, assign22610_e17389_d_n7, assign22610_e17389_d_n8, assign22610_e17389_d_n9, assign22610_e17389_d_n10, assign22610_e17389_d_n13,) = {
    if (locals.var_guard426 == 0.0) {
        let (assign22610_e17387, assign22610_e17387_d_n0, assign22610_e17387_d_n2, assign22610_e17387_d_n4, assign22610_e17387_d_n5, assign22610_e17387_d_n6, assign22610_e17387_d_n7, assign22610_e17387_d_n8, assign22610_e17387_d_n9, assign22610_e17387_d_n10, assign22610_e17387_d_n13,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
            } else {
                let assign22610_e17386: f64 = (-locals.var_tmf2);
                (assign22610_e17386, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn13),)
            }
        };
        (assign22610_e17387, assign22610_e17387_d_n0, assign22610_e17387_d_n2, assign22610_e17387_d_n4, assign22610_e17387_d_n5, assign22610_e17387_d_n6, assign22610_e17387_d_n7, assign22610_e17387_d_n8, assign22610_e17387_d_n9, assign22610_e17387_d_n10, assign22610_e17387_d_n13,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign22610_e17389;
        locals.var_tmf2_dn0 = assign22610_e17389_d_n0;
        locals.var_tmf2_dn2 = assign22610_e17389_d_n2;
        locals.var_tmf2_dn4 = assign22610_e17389_d_n4;
        locals.var_tmf2_dn5 = assign22610_e17389_d_n5;
        locals.var_tmf2_dn6 = assign22610_e17389_d_n6;
        locals.var_tmf2_dn7 = assign22610_e17389_d_n7;
        locals.var_tmf2_dn8 = assign22610_e17389_d_n8;
        locals.var_tmf2_dn9 = assign22610_e17389_d_n9;
        locals.var_tmf2_dn10 = assign22610_e17389_d_n10;
        locals.var_tmf2_dn13 = assign22610_e17389_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign22620_e17399, assign22620_e17399_d_n0, assign22620_e17399_d_n2, assign22620_e17399_d_n4, assign22620_e17399_d_n5, assign22620_e17399_d_n6, assign22620_e17399_d_n7, assign22620_e17399_d_n8, assign22620_e17399_d_n9, assign22620_e17399_d_n10, assign22620_e17399_d_n13,) = {
    if (locals.var_guard426 == 0.0) {
        let assign22620_e17394: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign22620_e17396: f64 = (assign22620_e17394 + locals.var_tmf2);
        let assign22620_e17397: f64 = (assign22620_e17396).sqrt();
        (assign22620_e17397, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign22620_e17397)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign22620_e17397)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign22620_e17397)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign22620_e17397)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign22620_e17397)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign22620_e17397)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign22620_e17397)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign22620_e17397)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign22620_e17397)), ((((locals.var_tmf1_dn13 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn13)) + locals.var_tmf2_dn13) / (2.0 * assign22620_e17397)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign22620_e17399;
        locals.var_tmf2_dn0 = assign22620_e17399_d_n0;
        locals.var_tmf2_dn2 = assign22620_e17399_d_n2;
        locals.var_tmf2_dn4 = assign22620_e17399_d_n4;
        locals.var_tmf2_dn5 = assign22620_e17399_d_n5;
        locals.var_tmf2_dn6 = assign22620_e17399_d_n6;
        locals.var_tmf2_dn7 = assign22620_e17399_d_n7;
        locals.var_tmf2_dn8 = assign22620_e17399_d_n8;
        locals.var_tmf2_dn9 = assign22620_e17399_d_n9;
        locals.var_tmf2_dn10 = assign22620_e17399_d_n10;
        locals.var_tmf2_dn13 = assign22620_e17399_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign22630_e17410, assign22630_e17410_d_n0, assign22630_e17410_d_n2, assign22630_e17410_d_n4, assign22630_e17410_d_n5, assign22630_e17410_d_n6, assign22630_e17410_d_n7, assign22630_e17410_d_n8, assign22630_e17410_d_n9, assign22630_e17410_d_n10, assign22630_e17410_d_n13,) = {
    if (locals.var_guard426 == 0.0) {
        let assign22630_e17406: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign22630_e17407: f64 = (1.0 + assign22630_e17406);
        let assign22630_e17408: f64 = (0.5 * assign22630_e17407);
        (assign22630_e17408, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn13 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn13)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn13,)
    }
};
        locals.var_t6 = assign22630_e17410;
        locals.var_t6_dn0 = assign22630_e17410_d_n0;
        locals.var_t6_dn2 = assign22630_e17410_d_n2;
        locals.var_t6_dn4 = assign22630_e17410_d_n4;
        locals.var_t6_dn5 = assign22630_e17410_d_n5;
        locals.var_t6_dn6 = assign22630_e17410_d_n6;
        locals.var_t6_dn7 = assign22630_e17410_d_n7;
        locals.var_t6_dn8 = assign22630_e17410_d_n8;
        locals.var_t6_dn9 = assign22630_e17410_d_n9;
        locals.var_t6_dn10 = assign22630_e17410_d_n10;
        locals.var_t6_dn13 = assign22630_e17410_d_n13;
        locals.var_t6_rv = 0.0;

        let (assign22640_e17423, assign22640_e17423_d_n0, assign22640_e17423_d_n2, assign22640_e17423_d_n4, assign22640_e17423_d_n5, assign22640_e17423_d_n6, assign22640_e17423_d_n7, assign22640_e17423_d_n8, assign22640_e17423_d_n9, assign22640_e17423_d_n10, assign22640_e17423_d_n13,) = {
    if (locals.var_guard426 == 0.0) {
        let assign22640_e17415: f64 = (1.0 / locals.var_t4);
        let assign22640_e17419: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign22640_e17420: f64 = (0.5 * assign22640_e17419);
        let assign22640_e17421: f64 = (assign22640_e17415 - assign22640_e17420);
        (assign22640_e17421, ((-(locals.var_t4_dn0 / (locals.var_t4 * locals.var_t4))) - (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), ((-(locals.var_t4_dn2 / (locals.var_t4 * locals.var_t4))) - (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), ((-(locals.var_t4_dn4 / (locals.var_t4 * locals.var_t4))) - (0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4))), ((-(locals.var_t4_dn5 / (locals.var_t4 * locals.var_t4))) - (0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5))), ((-(locals.var_t4_dn6 / (locals.var_t4 * locals.var_t4))) - (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), ((-(locals.var_t4_dn7 / (locals.var_t4 * locals.var_t4))) - (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))), ((-(locals.var_t4_dn8 / (locals.var_t4 * locals.var_t4))) - (0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8))), ((-(locals.var_t4_dn9 / (locals.var_t4 * locals.var_t4))) - (0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9))), ((-(locals.var_t4_dn10 / (locals.var_t4 * locals.var_t4))) - (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), ((-(locals.var_t4_dn13 / (locals.var_t4 * locals.var_t4))) - (0.5 * (locals.var_tmf1_dn13 + locals.var_tmf2_dn13))),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign22640_e17423;
        locals.var_t2_dn0 = assign22640_e17423_d_n0;
        locals.var_t2_dn2 = assign22640_e17423_d_n2;
        locals.var_t2_dn4 = assign22640_e17423_d_n4;
        locals.var_t2_dn5 = assign22640_e17423_d_n5;
        locals.var_t2_dn6 = assign22640_e17423_d_n6;
        locals.var_t2_dn7 = assign22640_e17423_d_n7;
        locals.var_t2_dn8 = assign22640_e17423_d_n8;
        locals.var_t2_dn9 = assign22640_e17423_d_n9;
        locals.var_t2_dn10 = assign22640_e17423_d_n10;
        locals.var_t2_dn13 = assign22640_e17423_d_n13;
        locals.var_t2_rv = 0.0;

        let (assign22650_e17432, assign22650_e17432_d_n0, assign22650_e17432_d_n2, assign22650_e17432_d_n4, assign22650_e17432_d_n5, assign22650_e17432_d_n6, assign22650_e17432_d_n7, assign22650_e17432_d_n8, assign22650_e17432_d_n9, assign22650_e17432_d_n10, assign22650_e17432_d_n13,) = {
    if (locals.var_guard426 == 0.0) {
        let assign22650_e17428: f64 = (p.p235 * locals.var_t2);
        let assign22650_e17430: f64 = (assign22650_e17428 + p.p237);
        (assign22650_e17430, (p.p235 * locals.var_t2_dn0), (p.p235 * locals.var_t2_dn2), (p.p235 * locals.var_t2_dn4), (p.p235 * locals.var_t2_dn5), (p.p235 * locals.var_t2_dn6), (p.p235 * locals.var_t2_dn7), (p.p235 * locals.var_t2_dn8), (p.p235 * locals.var_t2_dn9), (p.p235 * locals.var_t2_dn10), (p.p235 * locals.var_t2_dn13),)
    } else {
        (locals.var_dtox, locals.var_dtox_dn0, locals.var_dtox_dn2, locals.var_dtox_dn4, locals.var_dtox_dn5, locals.var_dtox_dn6, locals.var_dtox_dn7, locals.var_dtox_dn8, locals.var_dtox_dn9, locals.var_dtox_dn10, locals.var_dtox_dn13,)
    }
};
        locals.var_dtox = assign22650_e17432;
        locals.var_dtox_dn0 = assign22650_e17432_d_n0;
        locals.var_dtox_dn2 = assign22650_e17432_d_n2;
        locals.var_dtox_dn4 = assign22650_e17432_d_n4;
        locals.var_dtox_dn5 = assign22650_e17432_d_n5;
        locals.var_dtox_dn6 = assign22650_e17432_d_n6;
        locals.var_dtox_dn7 = assign22650_e17432_d_n7;
        locals.var_dtox_dn8 = assign22650_e17432_d_n8;
        locals.var_dtox_dn9 = assign22650_e17432_d_n9;
        locals.var_dtox_dn10 = assign22650_e17432_d_n10;
        locals.var_dtox_dn13 = assign22650_e17432_d_n13;
        locals.var_dtox_rv = 0.0;

        let (assign22660_e17437, assign22660_e17437_d_n0, assign22660_e17437_d_n2, assign22660_e17437_d_n4, assign22660_e17437_d_n5, assign22660_e17437_d_n6, assign22660_e17437_d_n7, assign22660_e17437_d_n8, assign22660_e17437_d_n9, assign22660_e17437_d_n10, assign22660_e17437_d_n13,) = {
    if (locals.var_guard426 == 0.0) {
        (p.p235, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t7, locals.var_t7_dn0, locals.var_t7_dn2, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn13,)
    }
};
        locals.var_t7 = assign22660_e17437;
        locals.var_t7_dn0 = assign22660_e17437_d_n0;
        locals.var_t7_dn2 = assign22660_e17437_d_n2;
        locals.var_t7_dn4 = assign22660_e17437_d_n4;
        locals.var_t7_dn5 = assign22660_e17437_d_n5;
        locals.var_t7_dn6 = assign22660_e17437_d_n6;
        locals.var_t7_dn7 = assign22660_e17437_d_n7;
        locals.var_t7_dn8 = assign22660_e17437_d_n8;
        locals.var_t7_dn9 = assign22660_e17437_d_n9;
        locals.var_t7_dn10 = assign22660_e17437_d_n10;
        locals.var_t7_dn13 = assign22660_e17437_d_n13;
        locals.var_t7_rv = 0.0;

        let assign22670_e17440: f64 = (locals.var_dtox * 1000000000000.0);
        let assign22670_e17442: f64 = if assign22670_e17440 < locals.var_tox0 { 1.0 } else { 0.0 };
        locals.var_guard429 = assign22670_e17442;
        locals.var_guard429_rv = 0.0;

        let (assign22680_e17449, assign22680_e17449_d_n0, assign22680_e17449_d_n2, assign22680_e17449_d_n4, assign22680_e17449_d_n5, assign22680_e17449_d_n6, assign22680_e17449_d_n7, assign22680_e17449_d_n8, assign22680_e17449_d_n9, assign22680_e17449_d_n10, assign22680_e17449_d_n13,) = {
    if ((locals.var_guard426 == 0.0) && (locals.var_guard429 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dtox, locals.var_dtox_dn0, locals.var_dtox_dn2, locals.var_dtox_dn4, locals.var_dtox_dn5, locals.var_dtox_dn6, locals.var_dtox_dn7, locals.var_dtox_dn8, locals.var_dtox_dn9, locals.var_dtox_dn10, locals.var_dtox_dn13,)
    }
};
        locals.var_dtox = assign22680_e17449;
        locals.var_dtox_dn0 = assign22680_e17449_d_n0;
        locals.var_dtox_dn2 = assign22680_e17449_d_n2;
        locals.var_dtox_dn4 = assign22680_e17449_d_n4;
        locals.var_dtox_dn5 = assign22680_e17449_d_n5;
        locals.var_dtox_dn6 = assign22680_e17449_d_n6;
        locals.var_dtox_dn7 = assign22680_e17449_d_n7;
        locals.var_dtox_dn8 = assign22680_e17449_d_n8;
        locals.var_dtox_dn9 = assign22680_e17449_d_n9;
        locals.var_dtox_dn10 = assign22680_e17449_d_n10;
        locals.var_dtox_dn13 = assign22680_e17449_d_n13;
        locals.var_dtox_rv = 0.0;

        let (assign22690_e17456,) = {
    if ((locals.var_guard426 == 0.0) && (locals.var_guard429 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_flg_qme,)
    }
};
        locals.var_flg_qme = assign22690_e17456;
        locals.var_flg_qme_rv = 0.0;

        let (assign22700_e17463, assign22700_e17463_d_n0, assign22700_e17463_d_n2, assign22700_e17463_d_n4, assign22700_e17463_d_n5, assign22700_e17463_d_n6, assign22700_e17463_d_n7, assign22700_e17463_d_n8, assign22700_e17463_d_n9, assign22700_e17463_d_n10, assign22700_e17463_d_n13,) = {
    if (locals.var_guard426 == 0.0) {
        let assign22700_e17461: f64 = (locals.var_tox0 + locals.var_dtox);
        (assign22700_e17461, locals.var_dtox_dn0, locals.var_dtox_dn2, locals.var_dtox_dn4, locals.var_dtox_dn5, locals.var_dtox_dn6, locals.var_dtox_dn7, locals.var_dtox_dn8, locals.var_dtox_dn9, locals.var_dtox_dn10, locals.var_dtox_dn13,)
    } else {
        (locals.var_toxe, locals.var_toxe_dn0, locals.var_toxe_dn2, locals.var_toxe_dn4, locals.var_toxe_dn5, locals.var_toxe_dn6, locals.var_toxe_dn7, locals.var_toxe_dn8, locals.var_toxe_dn9, locals.var_toxe_dn10, locals.var_toxe_dn13,)
    }
};
        locals.var_toxe = assign22700_e17463;
        locals.var_toxe_dn0 = assign22700_e17463_d_n0;
        locals.var_toxe_dn2 = assign22700_e17463_d_n2;
        locals.var_toxe_dn4 = assign22700_e17463_d_n4;
        locals.var_toxe_dn5 = assign22700_e17463_d_n5;
        locals.var_toxe_dn6 = assign22700_e17463_d_n6;
        locals.var_toxe_dn7 = assign22700_e17463_d_n7;
        locals.var_toxe_dn8 = assign22700_e17463_d_n8;
        locals.var_toxe_dn9 = assign22700_e17463_d_n9;
        locals.var_toxe_dn10 = assign22700_e17463_d_n10;
        locals.var_toxe_dn13 = assign22700_e17463_d_n13;
        locals.var_toxe_rv = 0.0;

        let (assign22710_e17470, assign22710_e17470_d_n0, assign22710_e17470_d_n2, assign22710_e17470_d_n4, assign22710_e17470_d_n5, assign22710_e17470_d_n6, assign22710_e17470_d_n7, assign22710_e17470_d_n8, assign22710_e17470_d_n9, assign22710_e17470_d_n10, assign22710_e17470_d_n13,) = {
    if (locals.var_guard426 == 0.0) {
        let assign22710_e17468: f64 = (locals.var_c_eox / locals.var_toxe);
        (assign22710_e17468, (-((locals.var_c_eox * locals.var_toxe_dn0) / (locals.var_toxe * locals.var_toxe))), (-((locals.var_c_eox * locals.var_toxe_dn2) / (locals.var_toxe * locals.var_toxe))), (-((locals.var_c_eox * locals.var_toxe_dn4) / (locals.var_toxe * locals.var_toxe))), (-((locals.var_c_eox * locals.var_toxe_dn5) / (locals.var_toxe * locals.var_toxe))), (-((locals.var_c_eox * locals.var_toxe_dn6) / (locals.var_toxe * locals.var_toxe))), (-((locals.var_c_eox * locals.var_toxe_dn7) / (locals.var_toxe * locals.var_toxe))), (-((locals.var_c_eox * locals.var_toxe_dn8) / (locals.var_toxe * locals.var_toxe))), (-((locals.var_c_eox * locals.var_toxe_dn9) / (locals.var_toxe * locals.var_toxe))), (-((locals.var_c_eox * locals.var_toxe_dn10) / (locals.var_toxe * locals.var_toxe))), (-((locals.var_c_eox * locals.var_toxe_dn13) / (locals.var_toxe * locals.var_toxe))),)
    } else {
        (locals.var_cox, locals.var_cox_dn0, locals.var_cox_dn2, locals.var_cox_dn4, locals.var_cox_dn5, locals.var_cox_dn6, locals.var_cox_dn7, locals.var_cox_dn8, locals.var_cox_dn9, locals.var_cox_dn10, locals.var_cox_dn13,)
    }
};
        locals.var_cox = assign22710_e17470;
        locals.var_cox_dn0 = assign22710_e17470_d_n0;
        locals.var_cox_dn2 = assign22710_e17470_d_n2;
        locals.var_cox_dn4 = assign22710_e17470_d_n4;
        locals.var_cox_dn5 = assign22710_e17470_d_n5;
        locals.var_cox_dn6 = assign22710_e17470_d_n6;
        locals.var_cox_dn7 = assign22710_e17470_d_n7;
        locals.var_cox_dn8 = assign22710_e17470_d_n8;
        locals.var_cox_dn9 = assign22710_e17470_d_n9;
        locals.var_cox_dn10 = assign22710_e17470_d_n10;
        locals.var_cox_dn13 = assign22710_e17470_d_n13;
        locals.var_cox_rv = 0.0;

        let (assign22720_e17480, assign22720_e17480_d_n0, assign22720_e17480_d_n2, assign22720_e17480_d_n4, assign22720_e17480_d_n5, assign22720_e17480_d_n6, assign22720_e17480_d_n7, assign22720_e17480_d_n8, assign22720_e17480_d_n9, assign22720_e17480_d_n10, assign22720_e17480_d_n13,) = {
    if (locals.var_guard426 == 0.0) {
        let assign22720_e17474: f64 = (-locals.var_c_eox);
        let assign22720_e17477: f64 = (locals.var_toxe * locals.var_toxe);
        let assign22720_e17478: f64 = (assign22720_e17474 / assign22720_e17477);
        (assign22720_e17478, (-((assign22720_e17474 * ((locals.var_toxe_dn0 * locals.var_toxe) + (locals.var_toxe * locals.var_toxe_dn0))) / (assign22720_e17477 * assign22720_e17477))), (-((assign22720_e17474 * ((locals.var_toxe_dn2 * locals.var_toxe) + (locals.var_toxe * locals.var_toxe_dn2))) / (assign22720_e17477 * assign22720_e17477))), (-((assign22720_e17474 * ((locals.var_toxe_dn4 * locals.var_toxe) + (locals.var_toxe * locals.var_toxe_dn4))) / (assign22720_e17477 * assign22720_e17477))), (-((assign22720_e17474 * ((locals.var_toxe_dn5 * locals.var_toxe) + (locals.var_toxe * locals.var_toxe_dn5))) / (assign22720_e17477 * assign22720_e17477))), (-((assign22720_e17474 * ((locals.var_toxe_dn6 * locals.var_toxe) + (locals.var_toxe * locals.var_toxe_dn6))) / (assign22720_e17477 * assign22720_e17477))), (-((assign22720_e17474 * ((locals.var_toxe_dn7 * locals.var_toxe) + (locals.var_toxe * locals.var_toxe_dn7))) / (assign22720_e17477 * assign22720_e17477))), (-((assign22720_e17474 * ((locals.var_toxe_dn8 * locals.var_toxe) + (locals.var_toxe * locals.var_toxe_dn8))) / (assign22720_e17477 * assign22720_e17477))), (-((assign22720_e17474 * ((locals.var_toxe_dn9 * locals.var_toxe) + (locals.var_toxe * locals.var_toxe_dn9))) / (assign22720_e17477 * assign22720_e17477))), (-((assign22720_e17474 * ((locals.var_toxe_dn10 * locals.var_toxe) + (locals.var_toxe * locals.var_toxe_dn10))) / (assign22720_e17477 * assign22720_e17477))), (-((assign22720_e17474 * ((locals.var_toxe_dn13 * locals.var_toxe) + (locals.var_toxe * locals.var_toxe_dn13))) / (assign22720_e17477 * assign22720_e17477))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign22720_e17480;
        locals.var_t1_dn0 = assign22720_e17480_d_n0;
        locals.var_t1_dn2 = assign22720_e17480_d_n2;
        locals.var_t1_dn4 = assign22720_e17480_d_n4;
        locals.var_t1_dn5 = assign22720_e17480_d_n5;
        locals.var_t1_dn6 = assign22720_e17480_d_n6;
        locals.var_t1_dn7 = assign22720_e17480_d_n7;
        locals.var_t1_dn8 = assign22720_e17480_d_n8;
        locals.var_t1_dn9 = assign22720_e17480_d_n9;
        locals.var_t1_dn10 = assign22720_e17480_d_n10;
        locals.var_t1_dn13 = assign22720_e17480_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign22730_e17487, assign22730_e17487_d_n0, assign22730_e17487_d_n2, assign22730_e17487_d_n4, assign22730_e17487_d_n5, assign22730_e17487_d_n6, assign22730_e17487_d_n7, assign22730_e17487_d_n8, assign22730_e17487_d_n9, assign22730_e17487_d_n10, assign22730_e17487_d_n13,) = {
    if (locals.var_guard426 == 0.0) {
        let assign22730_e17485: f64 = (locals.var_toxe / locals.var_c_eox);
        (assign22730_e17485, (locals.var_toxe_dn0 / locals.var_c_eox), (locals.var_toxe_dn2 / locals.var_c_eox), (locals.var_toxe_dn4 / locals.var_c_eox), (locals.var_toxe_dn5 / locals.var_c_eox), (locals.var_toxe_dn6 / locals.var_c_eox), (locals.var_toxe_dn7 / locals.var_c_eox), (locals.var_toxe_dn8 / locals.var_c_eox), (locals.var_toxe_dn9 / locals.var_c_eox), (locals.var_toxe_dn10 / locals.var_c_eox), (locals.var_toxe_dn13 / locals.var_c_eox),)
    } else {
        (locals.var_cox_inv, locals.var_cox_inv_dn0, locals.var_cox_inv_dn2, locals.var_cox_inv_dn4, locals.var_cox_inv_dn5, locals.var_cox_inv_dn6, locals.var_cox_inv_dn7, locals.var_cox_inv_dn8, locals.var_cox_inv_dn9, locals.var_cox_inv_dn10, locals.var_cox_inv_dn13,)
    }
};
        locals.var_cox_inv = assign22730_e17487;
        locals.var_cox_inv_dn0 = assign22730_e17487_d_n0;
        locals.var_cox_inv_dn2 = assign22730_e17487_d_n2;
        locals.var_cox_inv_dn4 = assign22730_e17487_d_n4;
        locals.var_cox_inv_dn5 = assign22730_e17487_d_n5;
        locals.var_cox_inv_dn6 = assign22730_e17487_d_n6;
        locals.var_cox_inv_dn7 = assign22730_e17487_d_n7;
        locals.var_cox_inv_dn8 = assign22730_e17487_d_n8;
        locals.var_cox_inv_dn9 = assign22730_e17487_d_n9;
        locals.var_cox_inv_dn10 = assign22730_e17487_d_n10;
        locals.var_cox_inv_dn13 = assign22730_e17487_d_n13;
        locals.var_cox_inv_rv = 0.0;

        let (assign22740_e17494, assign22740_e17494_d_n0, assign22740_e17494_d_n2, assign22740_e17494_d_n4, assign22740_e17494_d_n5, assign22740_e17494_d_n6, assign22740_e17494_d_n7, assign22740_e17494_d_n8, assign22740_e17494_d_n9, assign22740_e17494_d_n10, assign22740_e17494_d_n13,) = {
    if (locals.var_guard426 == 0.0) {
        let assign22740_e17492: f64 = (1.0 / locals.var_c_eox);
        (assign22740_e17492, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign22740_e17494;
        locals.var_t1_dn0 = assign22740_e17494_d_n0;
        locals.var_t1_dn2 = assign22740_e17494_d_n2;
        locals.var_t1_dn4 = assign22740_e17494_d_n4;
        locals.var_t1_dn5 = assign22740_e17494_d_n5;
        locals.var_t1_dn6 = assign22740_e17494_d_n6;
        locals.var_t1_dn7 = assign22740_e17494_d_n7;
        locals.var_t1_dn8 = assign22740_e17494_d_n8;
        locals.var_t1_dn9 = assign22740_e17494_d_n9;
        locals.var_t1_dn10 = assign22740_e17494_d_n10;
        locals.var_t1_dn13 = assign22740_e17494_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign22750_e17503, assign22750_e17503_d_n0, assign22750_e17503_d_n2, assign22750_e17503_d_n4, assign22750_e17503_d_n5, assign22750_e17503_d_n6, assign22750_e17503_d_n7, assign22750_e17503_d_n8, assign22750_e17503_d_n9, assign22750_e17503_d_n10, assign22750_e17503_d_n13,) = {
    if (locals.var_guard426 == 0.0) {
        let assign22750_e17499: f64 = (locals.var_cnst0 * locals.var_cnst0);
        let assign22750_e17501: f64 = (assign22750_e17499 * locals.var_cox_inv);
        (assign22750_e17501, ((((locals.var_cnst0_dn0 * locals.var_cnst0) + (locals.var_cnst0 * locals.var_cnst0_dn0)) * locals.var_cox_inv) + (assign22750_e17499 * locals.var_cox_inv_dn0)), ((((locals.var_cnst0_dn2 * locals.var_cnst0) + (locals.var_cnst0 * locals.var_cnst0_dn2)) * locals.var_cox_inv) + (assign22750_e17499 * locals.var_cox_inv_dn2)), ((((locals.var_cnst0_dn4 * locals.var_cnst0) + (locals.var_cnst0 * locals.var_cnst0_dn4)) * locals.var_cox_inv) + (assign22750_e17499 * locals.var_cox_inv_dn4)), ((((locals.var_cnst0_dn5 * locals.var_cnst0) + (locals.var_cnst0 * locals.var_cnst0_dn5)) * locals.var_cox_inv) + (assign22750_e17499 * locals.var_cox_inv_dn5)), ((((locals.var_cnst0_dn6 * locals.var_cnst0) + (locals.var_cnst0 * locals.var_cnst0_dn6)) * locals.var_cox_inv) + (assign22750_e17499 * locals.var_cox_inv_dn6)), ((((locals.var_cnst0_dn7 * locals.var_cnst0) + (locals.var_cnst0 * locals.var_cnst0_dn7)) * locals.var_cox_inv) + (assign22750_e17499 * locals.var_cox_inv_dn7)), ((((locals.var_cnst0_dn8 * locals.var_cnst0) + (locals.var_cnst0 * locals.var_cnst0_dn8)) * locals.var_cox_inv) + (assign22750_e17499 * locals.var_cox_inv_dn8)), ((((locals.var_cnst0_dn9 * locals.var_cnst0) + (locals.var_cnst0 * locals.var_cnst0_dn9)) * locals.var_cox_inv) + (assign22750_e17499 * locals.var_cox_inv_dn9)), ((((locals.var_cnst0_dn10 * locals.var_cnst0) + (locals.var_cnst0 * locals.var_cnst0_dn10)) * locals.var_cox_inv) + (assign22750_e17499 * locals.var_cox_inv_dn10)), ((((locals.var_cnst0_dn13 * locals.var_cnst0) + (locals.var_cnst0 * locals.var_cnst0_dn13)) * locals.var_cox_inv) + (assign22750_e17499 * locals.var_cox_inv_dn13)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign22750_e17503;
        locals.var_t0_dn0 = assign22750_e17503_d_n0;
        locals.var_t0_dn2 = assign22750_e17503_d_n2;
        locals.var_t0_dn4 = assign22750_e17503_d_n4;
        locals.var_t0_dn5 = assign22750_e17503_d_n5;
        locals.var_t0_dn6 = assign22750_e17503_d_n6;
        locals.var_t0_dn7 = assign22750_e17503_d_n7;
        locals.var_t0_dn8 = assign22750_e17503_d_n8;
        locals.var_t0_dn9 = assign22750_e17503_d_n9;
        locals.var_t0_dn10 = assign22750_e17503_d_n10;
        locals.var_t0_dn13 = assign22750_e17503_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign22760_e17510, assign22760_e17510_d_n0, assign22760_e17510_d_n2, assign22760_e17510_d_n4, assign22760_e17510_d_n5, assign22760_e17510_d_n6, assign22760_e17510_d_n7, assign22760_e17510_d_n8, assign22760_e17510_d_n9, assign22760_e17510_d_n10, assign22760_e17510_d_n13,) = {
    if (locals.var_guard426 == 0.0) {
        let assign22760_e17508: f64 = (locals.var_t0 * locals.var_cox_inv);
        (assign22760_e17508, ((locals.var_t0_dn0 * locals.var_cox_inv) + (locals.var_t0 * locals.var_cox_inv_dn0)), ((locals.var_t0_dn2 * locals.var_cox_inv) + (locals.var_t0 * locals.var_cox_inv_dn2)), ((locals.var_t0_dn4 * locals.var_cox_inv) + (locals.var_t0 * locals.var_cox_inv_dn4)), ((locals.var_t0_dn5 * locals.var_cox_inv) + (locals.var_t0 * locals.var_cox_inv_dn5)), ((locals.var_t0_dn6 * locals.var_cox_inv) + (locals.var_t0 * locals.var_cox_inv_dn6)), ((locals.var_t0_dn7 * locals.var_cox_inv) + (locals.var_t0 * locals.var_cox_inv_dn7)), ((locals.var_t0_dn8 * locals.var_cox_inv) + (locals.var_t0 * locals.var_cox_inv_dn8)), ((locals.var_t0_dn9 * locals.var_cox_inv) + (locals.var_t0 * locals.var_cox_inv_dn9)), ((locals.var_t0_dn10 * locals.var_cox_inv) + (locals.var_t0 * locals.var_cox_inv_dn10)), ((locals.var_t0_dn13 * locals.var_cox_inv) + (locals.var_t0 * locals.var_cox_inv_dn13)),)
    } else {
        (locals.var_cnstcoxi, locals.var_cnstcoxi_dn0, locals.var_cnstcoxi_dn2, locals.var_cnstcoxi_dn4, locals.var_cnstcoxi_dn5, locals.var_cnstcoxi_dn6, locals.var_cnstcoxi_dn7, locals.var_cnstcoxi_dn8, locals.var_cnstcoxi_dn9, locals.var_cnstcoxi_dn10, locals.var_cnstcoxi_dn13,)
    }
};
        locals.var_cnstcoxi = assign22760_e17510;
        locals.var_cnstcoxi_dn0 = assign22760_e17510_d_n0;
        locals.var_cnstcoxi_dn2 = assign22760_e17510_d_n2;
        locals.var_cnstcoxi_dn4 = assign22760_e17510_d_n4;
        locals.var_cnstcoxi_dn5 = assign22760_e17510_d_n5;
        locals.var_cnstcoxi_dn6 = assign22760_e17510_d_n6;
        locals.var_cnstcoxi_dn7 = assign22760_e17510_d_n7;
        locals.var_cnstcoxi_dn8 = assign22760_e17510_d_n8;
        locals.var_cnstcoxi_dn9 = assign22760_e17510_d_n9;
        locals.var_cnstcoxi_dn10 = assign22760_e17510_d_n10;
        locals.var_cnstcoxi_dn13 = assign22760_e17510_d_n13;
        locals.var_cnstcoxi_rv = 0.0;

        locals.var_vbsz2 = locals.var_vbsz;
        locals.var_vbsz2_dn0 = locals.var_vbsz_dn0;
        locals.var_vbsz2_dn2 = locals.var_vbsz_dn2;
        locals.var_vbsz2_dn4 = locals.var_vbsz_dn4;
        locals.var_vbsz2_dn5 = locals.var_vbsz_dn5;
        locals.var_vbsz2_dn6 = locals.var_vbsz_dn6;
        locals.var_vbsz2_dn7 = locals.var_vbsz_dn7;
        locals.var_vbsz2_dn8 = locals.var_vbsz_dn8;
        locals.var_vbsz2_dn9 = locals.var_vbsz_dn9;
        locals.var_vbsz2_dn10 = locals.var_vbsz_dn10;
        locals.var_vbsz2_dn13 = locals.var_vbsz_dn13;
        locals.var_vbsz2_rv = 0.0;

        locals.var_t1 = locals.var_qnsub_esi2;
        locals.var_t1_dn0 = locals.var_qnsub_esi2_dn0;
        locals.var_t1_dn2 = locals.var_qnsub_esi2_dn2;
        locals.var_t1_dn4 = locals.var_qnsub_esi2_dn4;
        locals.var_t1_dn5 = locals.var_qnsub_esi2_dn5;
        locals.var_t1_dn6 = locals.var_qnsub_esi2_dn6;
        locals.var_t1_dn7 = locals.var_qnsub_esi2_dn7;
        locals.var_t1_dn8 = locals.var_qnsub_esi2_dn8;
        locals.var_t1_dn9 = locals.var_qnsub_esi2_dn9;
        locals.var_t1_dn10 = locals.var_qnsub_esi2_dn10;
        locals.var_t1_dn13 = locals.var_qnsub_esi2_dn13;
        locals.var_t1_rv = 0.0;

        let assign22790_e17516: f64 = (locals.var_pb20 - locals.var_vbsz2);
        let assign22790_e17517: f64 = (locals.var_t1 * assign22790_e17516);
        let assign22790_e17518: f64 = (assign22790_e17517).sqrt();
        locals.var_qb0 = assign22790_e17518;
        locals.var_qb0_dn0 = (((locals.var_t1_dn0 * assign22790_e17516) + (locals.var_t1 * (locals.var_pb20_dn0 - locals.var_vbsz2_dn0))) / (2.0 * assign22790_e17518));
        locals.var_qb0_dn2 = (((locals.var_t1_dn2 * assign22790_e17516) + (locals.var_t1 * (locals.var_pb20_dn2 - locals.var_vbsz2_dn2))) / (2.0 * assign22790_e17518));
        locals.var_qb0_dn4 = (((locals.var_t1_dn4 * assign22790_e17516) + (locals.var_t1 * (locals.var_pb20_dn4 - locals.var_vbsz2_dn4))) / (2.0 * assign22790_e17518));
        locals.var_qb0_dn5 = (((locals.var_t1_dn5 * assign22790_e17516) + (locals.var_t1 * (locals.var_pb20_dn5 - locals.var_vbsz2_dn5))) / (2.0 * assign22790_e17518));
        locals.var_qb0_dn6 = (((locals.var_t1_dn6 * assign22790_e17516) + (locals.var_t1 * (locals.var_pb20_dn6 - locals.var_vbsz2_dn6))) / (2.0 * assign22790_e17518));
        locals.var_qb0_dn7 = (((locals.var_t1_dn7 * assign22790_e17516) + (locals.var_t1 * (locals.var_pb20_dn7 - locals.var_vbsz2_dn7))) / (2.0 * assign22790_e17518));
        locals.var_qb0_dn8 = (((locals.var_t1_dn8 * assign22790_e17516) + (locals.var_t1 * (locals.var_pb20_dn8 - locals.var_vbsz2_dn8))) / (2.0 * assign22790_e17518));
        locals.var_qb0_dn9 = (((locals.var_t1_dn9 * assign22790_e17516) + (locals.var_t1 * (locals.var_pb20_dn9 - locals.var_vbsz2_dn9))) / (2.0 * assign22790_e17518));
        locals.var_qb0_dn10 = (((locals.var_t1_dn10 * assign22790_e17516) + (locals.var_t1 * (locals.var_pb20_dn10 - locals.var_vbsz2_dn10))) / (2.0 * assign22790_e17518));
        locals.var_qb0_dn13 = (((locals.var_t1_dn13 * assign22790_e17516) + (locals.var_t1 * (locals.var_pb20_dn13 - locals.var_vbsz2_dn13))) / (2.0 * assign22790_e17518));
        locals.var_qb0_rv = 0.0;

        let assign22800_e17521: f64 = (0.5 * locals.var_t1);
        let assign22800_e17523: f64 = (assign22800_e17521 / locals.var_qb0);
        locals.var_t2 = assign22800_e17523;
        locals.var_t2_dn0 = ((((0.5 * locals.var_t1_dn0) * locals.var_qb0) - (assign22800_e17521 * locals.var_qb0_dn0)) / (locals.var_qb0 * locals.var_qb0));
        locals.var_t2_dn2 = ((((0.5 * locals.var_t1_dn2) * locals.var_qb0) - (assign22800_e17521 * locals.var_qb0_dn2)) / (locals.var_qb0 * locals.var_qb0));
        locals.var_t2_dn4 = ((((0.5 * locals.var_t1_dn4) * locals.var_qb0) - (assign22800_e17521 * locals.var_qb0_dn4)) / (locals.var_qb0 * locals.var_qb0));
        locals.var_t2_dn5 = ((((0.5 * locals.var_t1_dn5) * locals.var_qb0) - (assign22800_e17521 * locals.var_qb0_dn5)) / (locals.var_qb0 * locals.var_qb0));
        locals.var_t2_dn6 = ((((0.5 * locals.var_t1_dn6) * locals.var_qb0) - (assign22800_e17521 * locals.var_qb0_dn6)) / (locals.var_qb0 * locals.var_qb0));
        locals.var_t2_dn7 = ((((0.5 * locals.var_t1_dn7) * locals.var_qb0) - (assign22800_e17521 * locals.var_qb0_dn7)) / (locals.var_qb0 * locals.var_qb0));
        locals.var_t2_dn8 = ((((0.5 * locals.var_t1_dn8) * locals.var_qb0) - (assign22800_e17521 * locals.var_qb0_dn8)) / (locals.var_qb0 * locals.var_qb0));
        locals.var_t2_dn9 = ((((0.5 * locals.var_t1_dn9) * locals.var_qb0) - (assign22800_e17521 * locals.var_qb0_dn9)) / (locals.var_qb0 * locals.var_qb0));
        locals.var_t2_dn10 = ((((0.5 * locals.var_t1_dn10) * locals.var_qb0) - (assign22800_e17521 * locals.var_qb0_dn10)) / (locals.var_qb0 * locals.var_qb0));
        locals.var_t2_dn13 = ((((0.5 * locals.var_t1_dn13) * locals.var_qb0) - (assign22800_e17521 * locals.var_qb0_dn13)) / (locals.var_qb0 * locals.var_qb0));
        locals.var_t2_rv = 0.0;

        let assign22810_e17526: f64 = (locals.var_pb20 + locals.var_vfb);
        let assign22810_e17529: f64 = (locals.var_qb0 * locals.var_cox_inv);
        let assign22810_e17530: f64 = (assign22810_e17526 + assign22810_e17529);
        let assign22810_e17532: f64 = (assign22810_e17530 + locals.var_ptovr);
        locals.var_vthp = assign22810_e17532;
        locals.var_vthp_dn0 = ((locals.var_pb20_dn0 + ((locals.var_qb0_dn0 * locals.var_cox_inv) + (locals.var_qb0 * locals.var_cox_inv_dn0))) + locals.var_ptovr_dn0);
        locals.var_vthp_dn2 = ((locals.var_pb20_dn2 + ((locals.var_qb0_dn2 * locals.var_cox_inv) + (locals.var_qb0 * locals.var_cox_inv_dn2))) + locals.var_ptovr_dn2);
        locals.var_vthp_dn4 = ((locals.var_pb20_dn4 + ((locals.var_qb0_dn4 * locals.var_cox_inv) + (locals.var_qb0 * locals.var_cox_inv_dn4))) + locals.var_ptovr_dn4);
        locals.var_vthp_dn5 = ((locals.var_pb20_dn5 + ((locals.var_qb0_dn5 * locals.var_cox_inv) + (locals.var_qb0 * locals.var_cox_inv_dn5))) + locals.var_ptovr_dn5);
        locals.var_vthp_dn6 = ((locals.var_pb20_dn6 + ((locals.var_qb0_dn6 * locals.var_cox_inv) + (locals.var_qb0 * locals.var_cox_inv_dn6))) + locals.var_ptovr_dn6);
        locals.var_vthp_dn7 = ((locals.var_pb20_dn7 + ((locals.var_qb0_dn7 * locals.var_cox_inv) + (locals.var_qb0 * locals.var_cox_inv_dn7))) + locals.var_ptovr_dn7);
        locals.var_vthp_dn8 = ((locals.var_pb20_dn8 + ((locals.var_qb0_dn8 * locals.var_cox_inv) + (locals.var_qb0 * locals.var_cox_inv_dn8))) + locals.var_ptovr_dn8);
        locals.var_vthp_dn9 = ((locals.var_pb20_dn9 + ((locals.var_qb0_dn9 * locals.var_cox_inv) + (locals.var_qb0 * locals.var_cox_inv_dn9))) + locals.var_ptovr_dn9);
        locals.var_vthp_dn10 = ((locals.var_pb20_dn10 + ((locals.var_qb0_dn10 * locals.var_cox_inv) + (locals.var_qb0 * locals.var_cox_inv_dn10))) + locals.var_ptovr_dn10);
        locals.var_vthp_dn13 = ((locals.var_pb20_dn13 + ((locals.var_qb0_dn13 * locals.var_cox_inv) + (locals.var_qb0 * locals.var_cox_inv_dn13))) + locals.var_ptovr_dn13);
        locals.var_vthp_rv = 0.0;

        locals.var_pb20b = locals.var_pb20;
        locals.var_pb20b_dn0 = locals.var_pb20_dn0;
        locals.var_pb20b_dn2 = locals.var_pb20_dn2;
        locals.var_pb20b_dn4 = locals.var_pb20_dn4;
        locals.var_pb20b_dn5 = locals.var_pb20_dn5;
        locals.var_pb20b_dn6 = locals.var_pb20_dn6;
        locals.var_pb20b_dn7 = locals.var_pb20_dn7;
        locals.var_pb20b_dn8 = locals.var_pb20_dn8;
        locals.var_pb20b_dn9 = locals.var_pb20_dn9;
        locals.var_pb20b_dn10 = locals.var_pb20_dn10;
        locals.var_pb20b_dn13 = locals.var_pb20_dn13;
        locals.var_pb20b_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_59(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        locals.var_t0 = 0.95;
        locals.var_t0_dn0 = 0.0;
        locals.var_t0_dn2 = 0.0;
        locals.var_t0_dn4 = 0.0;
        locals.var_t0_dn5 = 0.0;
        locals.var_t0_dn6 = 0.0;
        locals.var_t0_dn7 = 0.0;
        locals.var_t0_dn8 = 0.0;
        locals.var_t0_dn9 = 0.0;
        locals.var_t0_dn10 = 0.0;
        locals.var_t0_dn13 = 0.0;
        locals.var_t0_rv = 0.0;

        let (assign22840_e17540,) = {
    if (locals.var_uc_codep > 1.0) {
        (0.0,)
    } else {
        (1.0,)
    }
};
        locals.var_t4 = assign22840_e17540;
        locals.var_t4_dn0 = 0.0;
        locals.var_t4_dn2 = 0.0;
        locals.var_t4_dn4 = 0.0;
        locals.var_t4_dn5 = 0.0;
        locals.var_t4_dn6 = 0.0;
        locals.var_t4_dn7 = 0.0;
        locals.var_t4_dn8 = 0.0;
        locals.var_t4_dn9 = 0.0;
        locals.var_t4_dn10 = 0.0;
        locals.var_t4_dn13 = 0.0;
        locals.var_t4_rv = 0.0;

        let assign22850_e17543: f64 = (locals.var_t0 * locals.var_pb20b);
        let assign22850_e17546: f64 = (locals.var_t4 * locals.var_vbsz2);
        let assign22850_e17547: f64 = (assign22850_e17543 - assign22850_e17546);
        let assign22850_e17549: f64 = (assign22850_e17547 - 0.001);
        locals.var_t1 = assign22850_e17549;
        locals.var_t1_dn0 = (((locals.var_t0_dn0 * locals.var_pb20b) + (locals.var_t0 * locals.var_pb20b_dn0)) - ((locals.var_t4_dn0 * locals.var_vbsz2) + (locals.var_t4 * locals.var_vbsz2_dn0)));
        locals.var_t1_dn2 = (((locals.var_t0_dn2 * locals.var_pb20b) + (locals.var_t0 * locals.var_pb20b_dn2)) - ((locals.var_t4_dn2 * locals.var_vbsz2) + (locals.var_t4 * locals.var_vbsz2_dn2)));
        locals.var_t1_dn4 = (((locals.var_t0_dn4 * locals.var_pb20b) + (locals.var_t0 * locals.var_pb20b_dn4)) - ((locals.var_t4_dn4 * locals.var_vbsz2) + (locals.var_t4 * locals.var_vbsz2_dn4)));
        locals.var_t1_dn5 = (((locals.var_t0_dn5 * locals.var_pb20b) + (locals.var_t0 * locals.var_pb20b_dn5)) - ((locals.var_t4_dn5 * locals.var_vbsz2) + (locals.var_t4 * locals.var_vbsz2_dn5)));
        locals.var_t1_dn6 = (((locals.var_t0_dn6 * locals.var_pb20b) + (locals.var_t0 * locals.var_pb20b_dn6)) - ((locals.var_t4_dn6 * locals.var_vbsz2) + (locals.var_t4 * locals.var_vbsz2_dn6)));
        locals.var_t1_dn7 = (((locals.var_t0_dn7 * locals.var_pb20b) + (locals.var_t0 * locals.var_pb20b_dn7)) - ((locals.var_t4_dn7 * locals.var_vbsz2) + (locals.var_t4 * locals.var_vbsz2_dn7)));
        locals.var_t1_dn8 = (((locals.var_t0_dn8 * locals.var_pb20b) + (locals.var_t0 * locals.var_pb20b_dn8)) - ((locals.var_t4_dn8 * locals.var_vbsz2) + (locals.var_t4 * locals.var_vbsz2_dn8)));
        locals.var_t1_dn9 = (((locals.var_t0_dn9 * locals.var_pb20b) + (locals.var_t0 * locals.var_pb20b_dn9)) - ((locals.var_t4_dn9 * locals.var_vbsz2) + (locals.var_t4 * locals.var_vbsz2_dn9)));
        locals.var_t1_dn10 = (((locals.var_t0_dn10 * locals.var_pb20b) + (locals.var_t0 * locals.var_pb20b_dn10)) - ((locals.var_t4_dn10 * locals.var_vbsz2) + (locals.var_t4 * locals.var_vbsz2_dn10)));
        locals.var_t1_dn13 = (((locals.var_t0_dn13 * locals.var_pb20b) + (locals.var_t0 * locals.var_pb20b_dn13)) - ((locals.var_t4_dn13 * locals.var_vbsz2) + (locals.var_t4 * locals.var_vbsz2_dn13)));
        locals.var_t1_rv = 0.0;

        let assign22860_e17552: f64 = (locals.var_t1 * locals.var_t1);
        let assign22860_e17555: f64 = (4.0 * locals.var_t0);
        let assign22860_e17557: f64 = (assign22860_e17555 * locals.var_pb20b);
        let assign22860_e17559: f64 = (assign22860_e17557 * 0.001);
        let assign22860_e17560: f64 = (assign22860_e17552 + assign22860_e17559);
        let assign22860_e17561: f64 = (assign22860_e17560).sqrt();
        locals.var_t2 = assign22860_e17561;
        locals.var_t2_dn0 = ((((locals.var_t1_dn0 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn0)) + ((((4.0 * locals.var_t0_dn0) * locals.var_pb20b) + (assign22860_e17555 * locals.var_pb20b_dn0)) * 0.001)) / (2.0 * assign22860_e17561));
        locals.var_t2_dn2 = ((((locals.var_t1_dn2 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn2)) + ((((4.0 * locals.var_t0_dn2) * locals.var_pb20b) + (assign22860_e17555 * locals.var_pb20b_dn2)) * 0.001)) / (2.0 * assign22860_e17561));
        locals.var_t2_dn4 = ((((locals.var_t1_dn4 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn4)) + ((((4.0 * locals.var_t0_dn4) * locals.var_pb20b) + (assign22860_e17555 * locals.var_pb20b_dn4)) * 0.001)) / (2.0 * assign22860_e17561));
        locals.var_t2_dn5 = ((((locals.var_t1_dn5 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn5)) + ((((4.0 * locals.var_t0_dn5) * locals.var_pb20b) + (assign22860_e17555 * locals.var_pb20b_dn5)) * 0.001)) / (2.0 * assign22860_e17561));
        locals.var_t2_dn6 = ((((locals.var_t1_dn6 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn6)) + ((((4.0 * locals.var_t0_dn6) * locals.var_pb20b) + (assign22860_e17555 * locals.var_pb20b_dn6)) * 0.001)) / (2.0 * assign22860_e17561));
        locals.var_t2_dn7 = ((((locals.var_t1_dn7 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn7)) + ((((4.0 * locals.var_t0_dn7) * locals.var_pb20b) + (assign22860_e17555 * locals.var_pb20b_dn7)) * 0.001)) / (2.0 * assign22860_e17561));
        locals.var_t2_dn8 = ((((locals.var_t1_dn8 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn8)) + ((((4.0 * locals.var_t0_dn8) * locals.var_pb20b) + (assign22860_e17555 * locals.var_pb20b_dn8)) * 0.001)) / (2.0 * assign22860_e17561));
        locals.var_t2_dn9 = ((((locals.var_t1_dn9 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn9)) + ((((4.0 * locals.var_t0_dn9) * locals.var_pb20b) + (assign22860_e17555 * locals.var_pb20b_dn9)) * 0.001)) / (2.0 * assign22860_e17561));
        locals.var_t2_dn10 = ((((locals.var_t1_dn10 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn10)) + ((((4.0 * locals.var_t0_dn10) * locals.var_pb20b) + (assign22860_e17555 * locals.var_pb20b_dn10)) * 0.001)) / (2.0 * assign22860_e17561));
        locals.var_t2_dn13 = ((((locals.var_t1_dn13 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn13)) + ((((4.0 * locals.var_t0_dn13) * locals.var_pb20b) + (assign22860_e17555 * locals.var_pb20b_dn13)) * 0.001)) / (2.0 * assign22860_e17561));
        locals.var_t2_rv = 0.0;

        let assign22870_e17564: f64 = (locals.var_t0 * locals.var_pb20b);
        let assign22870_e17568: f64 = (locals.var_t1 + locals.var_t2);
        let assign22870_e17569: f64 = (0.5 * assign22870_e17568);
        let assign22870_e17570: f64 = (assign22870_e17564 - assign22870_e17569);
        locals.var_t3 = assign22870_e17570;
        locals.var_t3_dn0 = (((locals.var_t0_dn0 * locals.var_pb20b) + (locals.var_t0 * locals.var_pb20b_dn0)) - (0.5 * (locals.var_t1_dn0 + locals.var_t2_dn0)));
        locals.var_t3_dn2 = (((locals.var_t0_dn2 * locals.var_pb20b) + (locals.var_t0 * locals.var_pb20b_dn2)) - (0.5 * (locals.var_t1_dn2 + locals.var_t2_dn2)));
        locals.var_t3_dn4 = (((locals.var_t0_dn4 * locals.var_pb20b) + (locals.var_t0 * locals.var_pb20b_dn4)) - (0.5 * (locals.var_t1_dn4 + locals.var_t2_dn4)));
        locals.var_t3_dn5 = (((locals.var_t0_dn5 * locals.var_pb20b) + (locals.var_t0 * locals.var_pb20b_dn5)) - (0.5 * (locals.var_t1_dn5 + locals.var_t2_dn5)));
        locals.var_t3_dn6 = (((locals.var_t0_dn6 * locals.var_pb20b) + (locals.var_t0 * locals.var_pb20b_dn6)) - (0.5 * (locals.var_t1_dn6 + locals.var_t2_dn6)));
        locals.var_t3_dn7 = (((locals.var_t0_dn7 * locals.var_pb20b) + (locals.var_t0 * locals.var_pb20b_dn7)) - (0.5 * (locals.var_t1_dn7 + locals.var_t2_dn7)));
        locals.var_t3_dn8 = (((locals.var_t0_dn8 * locals.var_pb20b) + (locals.var_t0 * locals.var_pb20b_dn8)) - (0.5 * (locals.var_t1_dn8 + locals.var_t2_dn8)));
        locals.var_t3_dn9 = (((locals.var_t0_dn9 * locals.var_pb20b) + (locals.var_t0 * locals.var_pb20b_dn9)) - (0.5 * (locals.var_t1_dn9 + locals.var_t2_dn9)));
        locals.var_t3_dn10 = (((locals.var_t0_dn10 * locals.var_pb20b) + (locals.var_t0 * locals.var_pb20b_dn10)) - (0.5 * (locals.var_t1_dn10 + locals.var_t2_dn10)));
        locals.var_t3_dn13 = (((locals.var_t0_dn13 * locals.var_pb20b) + (locals.var_t0 * locals.var_pb20b_dn13)) - (0.5 * (locals.var_t1_dn13 + locals.var_t2_dn13)));
        locals.var_t3_rv = 0.0;

        let (assign22880_e17578, assign22880_e17578_d_n0, assign22880_e17578_d_n2, assign22880_e17578_d_n4, assign22880_e17578_d_n5, assign22880_e17578_d_n6, assign22880_e17578_d_n7, assign22880_e17578_d_n8, assign22880_e17578_d_n9, assign22880_e17578_d_n10, assign22880_e17578_d_n13,) = {
    if (locals.var_uc_codep == 1.0) {
        let assign22880_e17576: f64 = (p.p366 * locals.var_vdsz);
        (assign22880_e17576, (p.p366 * locals.var_vdsz_dn0), (p.p366 * locals.var_vdsz_dn2), (p.p366 * locals.var_vdsz_dn4), (p.p366 * locals.var_vdsz_dn5), (p.p366 * locals.var_vdsz_dn6), (p.p366 * locals.var_vdsz_dn7), (p.p366 * locals.var_vdsz_dn8), (p.p366 * locals.var_vdsz_dn9), (p.p366 * locals.var_vdsz_dn10), (p.p366 * locals.var_vdsz_dn13),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        locals.var_t5 = assign22880_e17578;
        locals.var_t5_dn0 = assign22880_e17578_d_n0;
        locals.var_t5_dn2 = assign22880_e17578_d_n2;
        locals.var_t5_dn4 = assign22880_e17578_d_n4;
        locals.var_t5_dn5 = assign22880_e17578_d_n5;
        locals.var_t5_dn6 = assign22880_e17578_d_n6;
        locals.var_t5_dn7 = assign22880_e17578_d_n7;
        locals.var_t5_dn8 = assign22880_e17578_d_n8;
        locals.var_t5_dn9 = assign22880_e17578_d_n9;
        locals.var_t5_dn10 = assign22880_e17578_d_n10;
        locals.var_t5_dn13 = assign22880_e17578_d_n13;
        locals.var_t5_rv = 0.0;

        let assign22890_e17581: f64 = (locals.var_pb20b - locals.var_t3);
        let assign22890_e17583: f64 = (assign22890_e17581 + locals.var_t5);
        locals.var_pbsum = assign22890_e17583;
        locals.var_pbsum_dn0 = ((locals.var_pb20b_dn0 - locals.var_t3_dn0) + locals.var_t5_dn0);
        locals.var_pbsum_dn2 = ((locals.var_pb20b_dn2 - locals.var_t3_dn2) + locals.var_t5_dn2);
        locals.var_pbsum_dn4 = ((locals.var_pb20b_dn4 - locals.var_t3_dn4) + locals.var_t5_dn4);
        locals.var_pbsum_dn5 = ((locals.var_pb20b_dn5 - locals.var_t3_dn5) + locals.var_t5_dn5);
        locals.var_pbsum_dn6 = ((locals.var_pb20b_dn6 - locals.var_t3_dn6) + locals.var_t5_dn6);
        locals.var_pbsum_dn7 = ((locals.var_pb20b_dn7 - locals.var_t3_dn7) + locals.var_t5_dn7);
        locals.var_pbsum_dn8 = ((locals.var_pb20b_dn8 - locals.var_t3_dn8) + locals.var_t5_dn8);
        locals.var_pbsum_dn9 = ((locals.var_pb20b_dn9 - locals.var_t3_dn9) + locals.var_t5_dn9);
        locals.var_pbsum_dn10 = ((locals.var_pb20b_dn10 - locals.var_t3_dn10) + locals.var_t5_dn10);
        locals.var_pbsum_dn13 = ((locals.var_pb20b_dn13 - locals.var_t3_dn13) + locals.var_t5_dn13);
        locals.var_pbsum_rv = 0.0;

        let assign22900_e17585: f64 = (locals.var_pbsum).sqrt();
        locals.var_sqrt_pbsum = assign22900_e17585;
        locals.var_sqrt_pbsum_dn0 = (locals.var_pbsum_dn0 / (2.0 * assign22900_e17585));
        locals.var_sqrt_pbsum_dn2 = (locals.var_pbsum_dn2 / (2.0 * assign22900_e17585));
        locals.var_sqrt_pbsum_dn4 = (locals.var_pbsum_dn4 / (2.0 * assign22900_e17585));
        locals.var_sqrt_pbsum_dn5 = (locals.var_pbsum_dn5 / (2.0 * assign22900_e17585));
        locals.var_sqrt_pbsum_dn6 = (locals.var_pbsum_dn6 / (2.0 * assign22900_e17585));
        locals.var_sqrt_pbsum_dn7 = (locals.var_pbsum_dn7 / (2.0 * assign22900_e17585));
        locals.var_sqrt_pbsum_dn8 = (locals.var_pbsum_dn8 / (2.0 * assign22900_e17585));
        locals.var_sqrt_pbsum_dn9 = (locals.var_pbsum_dn9 / (2.0 * assign22900_e17585));
        locals.var_sqrt_pbsum_dn10 = (locals.var_pbsum_dn10 / (2.0 * assign22900_e17585));
        locals.var_sqrt_pbsum_dn13 = (locals.var_pbsum_dn13 / (2.0 * assign22900_e17585));
        locals.var_sqrt_pbsum_rv = 0.0;

        let assign22910_e17588: f64 = if p.p140 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard430 = assign22910_e17588;
        locals.var_guard430_rv = 0.0;

        let (assign22920_e17592, assign22920_e17592_d_n0, assign22920_e17592_d_n2, assign22920_e17592_d_n4, assign22920_e17592_d_n5, assign22920_e17592_d_n6, assign22920_e17592_d_n7, assign22920_e17592_d_n8, assign22920_e17592_d_n9, assign22920_e17592_d_n10, assign22920_e17592_d_n13,) = {
    if (locals.var_guard430 != 0.0) {
        (locals.var_qnsub_esi2, locals.var_qnsub_esi2_dn0, locals.var_qnsub_esi2_dn2, locals.var_qnsub_esi2_dn4, locals.var_qnsub_esi2_dn5, locals.var_qnsub_esi2_dn6, locals.var_qnsub_esi2_dn7, locals.var_qnsub_esi2_dn8, locals.var_qnsub_esi2_dn9, locals.var_qnsub_esi2_dn10, locals.var_qnsub_esi2_dn13,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign22920_e17592;
        locals.var_t1_dn0 = assign22920_e17592_d_n0;
        locals.var_t1_dn2 = assign22920_e17592_d_n2;
        locals.var_t1_dn4 = assign22920_e17592_d_n4;
        locals.var_t1_dn5 = assign22920_e17592_d_n5;
        locals.var_t1_dn6 = assign22920_e17592_d_n6;
        locals.var_t1_dn7 = assign22920_e17592_d_n7;
        locals.var_t1_dn8 = assign22920_e17592_d_n8;
        locals.var_t1_dn9 = assign22920_e17592_d_n9;
        locals.var_t1_dn10 = assign22920_e17592_d_n10;
        locals.var_t1_dn13 = assign22920_e17592_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign22930_e17598, assign22930_e17598_d_n0, assign22930_e17598_d_n2, assign22930_e17598_d_n4, assign22930_e17598_d_n5, assign22930_e17598_d_n6, assign22930_e17598_d_n7, assign22930_e17598_d_n8, assign22930_e17598_d_n9, assign22930_e17598_d_n10, assign22930_e17598_d_n13,) = {
    if (locals.var_guard430 != 0.0) {
        let assign22930_e17596: f64 = (p.p224 - locals.var_vbsz2);
        (assign22930_e17596, (-locals.var_vbsz2_dn0), (-locals.var_vbsz2_dn2), (-locals.var_vbsz2_dn4), (-locals.var_vbsz2_dn5), (-locals.var_vbsz2_dn6), (-locals.var_vbsz2_dn7), (-locals.var_vbsz2_dn8), (-locals.var_vbsz2_dn9), (-locals.var_vbsz2_dn10), (-locals.var_vbsz2_dn13),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign22930_e17598;
        locals.var_t2_dn0 = assign22930_e17598_d_n0;
        locals.var_t2_dn2 = assign22930_e17598_d_n2;
        locals.var_t2_dn4 = assign22930_e17598_d_n4;
        locals.var_t2_dn5 = assign22930_e17598_d_n5;
        locals.var_t2_dn6 = assign22930_e17598_d_n6;
        locals.var_t2_dn7 = assign22930_e17598_d_n7;
        locals.var_t2_dn8 = assign22930_e17598_d_n8;
        locals.var_t2_dn9 = assign22930_e17598_d_n9;
        locals.var_t2_dn10 = assign22930_e17598_d_n10;
        locals.var_t2_dn13 = assign22930_e17598_d_n13;
        locals.var_t2_rv = 0.0;

        let (assign22940_e17604, assign22940_e17604_d_n0, assign22940_e17604_d_n2, assign22940_e17604_d_n4, assign22940_e17604_d_n5, assign22940_e17604_d_n6, assign22940_e17604_d_n7, assign22940_e17604_d_n8, assign22940_e17604_d_n9, assign22940_e17604_d_n10, assign22940_e17604_d_n13,) = {
    if (locals.var_guard430 != 0.0) {
        let assign22940_e17602: f64 = (locals.var_t2 + 1e-25);
        (assign22940_e17602, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn13,)
    }
};
        locals.var_t3 = assign22940_e17604;
        locals.var_t3_dn0 = assign22940_e17604_d_n0;
        locals.var_t3_dn2 = assign22940_e17604_d_n2;
        locals.var_t3_dn4 = assign22940_e17604_d_n4;
        locals.var_t3_dn5 = assign22940_e17604_d_n5;
        locals.var_t3_dn6 = assign22940_e17604_d_n6;
        locals.var_t3_dn7 = assign22940_e17604_d_n7;
        locals.var_t3_dn8 = assign22940_e17604_d_n8;
        locals.var_t3_dn9 = assign22940_e17604_d_n9;
        locals.var_t3_dn10 = assign22940_e17604_d_n10;
        locals.var_t3_dn13 = assign22940_e17604_d_n13;
        locals.var_t3_rv = 0.0;

        let (assign22950_e17615, assign22950_e17615_d_n0, assign22950_e17615_d_n2, assign22950_e17615_d_n4, assign22950_e17615_d_n5, assign22950_e17615_d_n6, assign22950_e17615_d_n7, assign22950_e17615_d_n8, assign22950_e17615_d_n9, assign22950_e17615_d_n10, assign22950_e17615_d_n13,) = {
    if (locals.var_guard430 != 0.0) {
        let assign22950_e17608: f64 = (locals.var_t3 * locals.var_t3);
        let assign22950_e17611: f64 = (4.0 * 0.001);
        let assign22950_e17612: f64 = (assign22950_e17608 + assign22950_e17611);
        let assign22950_e17613: f64 = (assign22950_e17612).sqrt();
        (assign22950_e17613, (((locals.var_t3_dn0 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn0)) / (2.0 * assign22950_e17613)), (((locals.var_t3_dn2 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn2)) / (2.0 * assign22950_e17613)), (((locals.var_t3_dn4 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn4)) / (2.0 * assign22950_e17613)), (((locals.var_t3_dn5 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn5)) / (2.0 * assign22950_e17613)), (((locals.var_t3_dn6 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn6)) / (2.0 * assign22950_e17613)), (((locals.var_t3_dn7 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn7)) / (2.0 * assign22950_e17613)), (((locals.var_t3_dn8 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn8)) / (2.0 * assign22950_e17613)), (((locals.var_t3_dn9 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn9)) / (2.0 * assign22950_e17613)), (((locals.var_t3_dn10 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn10)) / (2.0 * assign22950_e17613)), (((locals.var_t3_dn13 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn13)) / (2.0 * assign22950_e17613)),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn13,)
    }
};
        locals.var_t4 = assign22950_e17615;
        locals.var_t4_dn0 = assign22950_e17615_d_n0;
        locals.var_t4_dn2 = assign22950_e17615_d_n2;
        locals.var_t4_dn4 = assign22950_e17615_d_n4;
        locals.var_t4_dn5 = assign22950_e17615_d_n5;
        locals.var_t4_dn6 = assign22950_e17615_d_n6;
        locals.var_t4_dn7 = assign22950_e17615_d_n7;
        locals.var_t4_dn8 = assign22950_e17615_d_n8;
        locals.var_t4_dn9 = assign22950_e17615_d_n9;
        locals.var_t4_dn10 = assign22950_e17615_d_n10;
        locals.var_t4_dn13 = assign22950_e17615_d_n13;
        locals.var_t4_rv = 0.0;

        let (assign22960_e17623, assign22960_e17623_d_n0, assign22960_e17623_d_n2, assign22960_e17623_d_n4, assign22960_e17623_d_n5, assign22960_e17623_d_n6, assign22960_e17623_d_n7, assign22960_e17623_d_n8, assign22960_e17623_d_n9, assign22960_e17623_d_n10, assign22960_e17623_d_n13,) = {
    if (locals.var_guard430 != 0.0) {
        let assign22960_e17620: f64 = (locals.var_t3 + locals.var_t4);
        let assign22960_e17621: f64 = (0.5 * assign22960_e17620);
        (assign22960_e17621, (0.5 * (locals.var_t3_dn0 + locals.var_t4_dn0)), (0.5 * (locals.var_t3_dn2 + locals.var_t4_dn2)), (0.5 * (locals.var_t3_dn4 + locals.var_t4_dn4)), (0.5 * (locals.var_t3_dn5 + locals.var_t4_dn5)), (0.5 * (locals.var_t3_dn6 + locals.var_t4_dn6)), (0.5 * (locals.var_t3_dn7 + locals.var_t4_dn7)), (0.5 * (locals.var_t3_dn8 + locals.var_t4_dn8)), (0.5 * (locals.var_t3_dn9 + locals.var_t4_dn9)), (0.5 * (locals.var_t3_dn10 + locals.var_t4_dn10)), (0.5 * (locals.var_t3_dn13 + locals.var_t4_dn13)),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn13,)
    }
};
        locals.var_t5 = assign22960_e17623;
        locals.var_t5_dn0 = assign22960_e17623_d_n0;
        locals.var_t5_dn2 = assign22960_e17623_d_n2;
        locals.var_t5_dn4 = assign22960_e17623_d_n4;
        locals.var_t5_dn5 = assign22960_e17623_d_n5;
        locals.var_t5_dn6 = assign22960_e17623_d_n6;
        locals.var_t5_dn7 = assign22960_e17623_d_n7;
        locals.var_t5_dn8 = assign22960_e17623_d_n8;
        locals.var_t5_dn9 = assign22960_e17623_d_n9;
        locals.var_t5_dn10 = assign22960_e17623_d_n10;
        locals.var_t5_dn13 = assign22960_e17623_d_n13;
        locals.var_t5_rv = 0.0;

        let (assign22970_e17633, assign22970_e17633_d_n0, assign22970_e17633_d_n2, assign22970_e17633_d_n4, assign22970_e17633_d_n5, assign22970_e17633_d_n6, assign22970_e17633_d_n7, assign22970_e17633_d_n8, assign22970_e17633_d_n9, assign22970_e17633_d_n10, assign22970_e17633_d_n13,) = {
    if (locals.var_guard430 != 0.0) {
        let assign22970_e17629: f64 = (locals.var_t3 / locals.var_t4);
        let assign22970_e17630: f64 = (1.0 + assign22970_e17629);
        let assign22970_e17631: f64 = (0.5 * assign22970_e17630);
        (assign22970_e17631, (0.5 * (((locals.var_t3_dn0 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn0)) / (locals.var_t4 * locals.var_t4))), (0.5 * (((locals.var_t3_dn2 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn2)) / (locals.var_t4 * locals.var_t4))), (0.5 * (((locals.var_t3_dn4 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn4)) / (locals.var_t4 * locals.var_t4))), (0.5 * (((locals.var_t3_dn5 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn5)) / (locals.var_t4 * locals.var_t4))), (0.5 * (((locals.var_t3_dn6 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn6)) / (locals.var_t4 * locals.var_t4))), (0.5 * (((locals.var_t3_dn7 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn7)) / (locals.var_t4 * locals.var_t4))), (0.5 * (((locals.var_t3_dn8 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn8)) / (locals.var_t4 * locals.var_t4))), (0.5 * (((locals.var_t3_dn9 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn9)) / (locals.var_t4 * locals.var_t4))), (0.5 * (((locals.var_t3_dn10 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn10)) / (locals.var_t4 * locals.var_t4))), (0.5 * (((locals.var_t3_dn13 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn13)) / (locals.var_t4 * locals.var_t4))),)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn13,)
    }
};
        locals.var_t6 = assign22970_e17633;
        locals.var_t6_dn0 = assign22970_e17633_d_n0;
        locals.var_t6_dn2 = assign22970_e17633_d_n2;
        locals.var_t6_dn4 = assign22970_e17633_d_n4;
        locals.var_t6_dn5 = assign22970_e17633_d_n5;
        locals.var_t6_dn6 = assign22970_e17633_d_n6;
        locals.var_t6_dn7 = assign22970_e17633_d_n7;
        locals.var_t6_dn8 = assign22970_e17633_d_n8;
        locals.var_t6_dn9 = assign22970_e17633_d_n9;
        locals.var_t6_dn10 = assign22970_e17633_d_n10;
        locals.var_t6_dn13 = assign22970_e17633_d_n13;
        locals.var_t6_rv = 0.0;

        let (assign22980_e17639, assign22980_e17639_d_n0, assign22980_e17639_d_n2, assign22980_e17639_d_n4, assign22980_e17639_d_n5, assign22980_e17639_d_n6, assign22980_e17639_d_n7, assign22980_e17639_d_n8, assign22980_e17639_d_n9, assign22980_e17639_d_n10, assign22980_e17639_d_n13,) = {
    if (locals.var_guard430 != 0.0) {
        let assign22980_e17637: f64 = (1.0 / locals.var_t5);
        (assign22980_e17637, (-(locals.var_t5_dn0 / (locals.var_t5 * locals.var_t5))), (-(locals.var_t5_dn2 / (locals.var_t5 * locals.var_t5))), (-(locals.var_t5_dn4 / (locals.var_t5 * locals.var_t5))), (-(locals.var_t5_dn5 / (locals.var_t5 * locals.var_t5))), (-(locals.var_t5_dn6 / (locals.var_t5 * locals.var_t5))), (-(locals.var_t5_dn7 / (locals.var_t5 * locals.var_t5))), (-(locals.var_t5_dn8 / (locals.var_t5 * locals.var_t5))), (-(locals.var_t5_dn9 / (locals.var_t5 * locals.var_t5))), (-(locals.var_t5_dn10 / (locals.var_t5 * locals.var_t5))), (-(locals.var_t5_dn13 / (locals.var_t5 * locals.var_t5))),)
    } else {
        (locals.var_t7, locals.var_t7_dn0, locals.var_t7_dn2, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn13,)
    }
};
        locals.var_t7 = assign22980_e17639;
        locals.var_t7_dn0 = assign22980_e17639_d_n0;
        locals.var_t7_dn2 = assign22980_e17639_d_n2;
        locals.var_t7_dn4 = assign22980_e17639_d_n4;
        locals.var_t7_dn5 = assign22980_e17639_d_n5;
        locals.var_t7_dn6 = assign22980_e17639_d_n6;
        locals.var_t7_dn7 = assign22980_e17639_d_n7;
        locals.var_t7_dn8 = assign22980_e17639_d_n8;
        locals.var_t7_dn9 = assign22980_e17639_d_n9;
        locals.var_t7_dn10 = assign22980_e17639_d_n10;
        locals.var_t7_dn13 = assign22980_e17639_d_n13;
        locals.var_t7_rv = 0.0;

        let (assign22990_e17645, assign22990_e17645_d_n0, assign22990_e17645_d_n2, assign22990_e17645_d_n4, assign22990_e17645_d_n5, assign22990_e17645_d_n6, assign22990_e17645_d_n7, assign22990_e17645_d_n8, assign22990_e17645_d_n9, assign22990_e17645_d_n10, assign22990_e17645_d_n13,) = {
    if (locals.var_guard430 != 0.0) {
        let assign22990_e17643: f64 = (p.p223 * locals.var_t7);
        (assign22990_e17643, (p.p223 * locals.var_t7_dn0), (p.p223 * locals.var_t7_dn2), (p.p223 * locals.var_t7_dn4), (p.p223 * locals.var_t7_dn5), (p.p223 * locals.var_t7_dn6), (p.p223 * locals.var_t7_dn7), (p.p223 * locals.var_t7_dn8), (p.p223 * locals.var_t7_dn9), (p.p223 * locals.var_t7_dn10), (p.p223 * locals.var_t7_dn13),)
    } else {
        (locals.var_bs12, locals.var_bs12_dn0, locals.var_bs12_dn2, locals.var_bs12_dn4, locals.var_bs12_dn5, locals.var_bs12_dn6, locals.var_bs12_dn7, locals.var_bs12_dn8, locals.var_bs12_dn9, locals.var_bs12_dn10, locals.var_bs12_dn13,)
    }
};
        locals.var_bs12 = assign22990_e17645;
        locals.var_bs12_dn0 = assign22990_e17645_d_n0;
        locals.var_bs12_dn2 = assign22990_e17645_d_n2;
        locals.var_bs12_dn4 = assign22990_e17645_d_n4;
        locals.var_bs12_dn5 = assign22990_e17645_d_n5;
        locals.var_bs12_dn6 = assign22990_e17645_d_n6;
        locals.var_bs12_dn7 = assign22990_e17645_d_n7;
        locals.var_bs12_dn8 = assign22990_e17645_d_n8;
        locals.var_bs12_dn9 = assign22990_e17645_d_n9;
        locals.var_bs12_dn10 = assign22990_e17645_d_n10;
        locals.var_bs12_dn13 = assign22990_e17645_d_n13;
        locals.var_bs12_rv = 0.0;

        let (assign23000_e17652, assign23000_e17652_d_n0, assign23000_e17652_d_n2, assign23000_e17652_d_n4, assign23000_e17652_d_n5, assign23000_e17652_d_n6, assign23000_e17652_d_n7, assign23000_e17652_d_n8, assign23000_e17652_d_n9, assign23000_e17652_d_n10, assign23000_e17652_d_n13,) = {
    if (locals.var_guard430 != 0.0) {
        let assign23000_e17648: f64 = (-locals.var_bs12);
        let assign23000_e17650: f64 = (assign23000_e17648 * locals.var_t7);
        (assign23000_e17650, (((-locals.var_bs12_dn0) * locals.var_t7) + (assign23000_e17648 * locals.var_t7_dn0)), (((-locals.var_bs12_dn2) * locals.var_t7) + (assign23000_e17648 * locals.var_t7_dn2)), (((-locals.var_bs12_dn4) * locals.var_t7) + (assign23000_e17648 * locals.var_t7_dn4)), (((-locals.var_bs12_dn5) * locals.var_t7) + (assign23000_e17648 * locals.var_t7_dn5)), (((-locals.var_bs12_dn6) * locals.var_t7) + (assign23000_e17648 * locals.var_t7_dn6)), (((-locals.var_bs12_dn7) * locals.var_t7) + (assign23000_e17648 * locals.var_t7_dn7)), (((-locals.var_bs12_dn8) * locals.var_t7) + (assign23000_e17648 * locals.var_t7_dn8)), (((-locals.var_bs12_dn9) * locals.var_t7) + (assign23000_e17648 * locals.var_t7_dn9)), (((-locals.var_bs12_dn10) * locals.var_t7) + (assign23000_e17648 * locals.var_t7_dn10)), (((-locals.var_bs12_dn13) * locals.var_t7) + (assign23000_e17648 * locals.var_t7_dn13)),)
    } else {
        (locals.var_t8, locals.var_t8_dn0, locals.var_t8_dn2, locals.var_t8_dn4, locals.var_t8_dn5, locals.var_t8_dn6, locals.var_t8_dn7, locals.var_t8_dn8, locals.var_t8_dn9, locals.var_t8_dn10, locals.var_t8_dn13,)
    }
};
        locals.var_t8 = assign23000_e17652;
        locals.var_t8_dn0 = assign23000_e17652_d_n0;
        locals.var_t8_dn2 = assign23000_e17652_d_n2;
        locals.var_t8_dn4 = assign23000_e17652_d_n4;
        locals.var_t8_dn5 = assign23000_e17652_d_n5;
        locals.var_t8_dn6 = assign23000_e17652_d_n6;
        locals.var_t8_dn7 = assign23000_e17652_d_n7;
        locals.var_t8_dn8 = assign23000_e17652_d_n8;
        locals.var_t8_dn9 = assign23000_e17652_d_n9;
        locals.var_t8_dn10 = assign23000_e17652_d_n10;
        locals.var_t8_dn13 = assign23000_e17652_d_n13;
        locals.var_t8_rv = 0.0;

        let (assign23010_e17664, assign23010_e17664_d_n0, assign23010_e17664_d_n2, assign23010_e17664_d_n4, assign23010_e17664_d_n5, assign23010_e17664_d_n6, assign23010_e17664_d_n7, assign23010_e17664_d_n8, assign23010_e17664_d_n9, assign23010_e17664_d_n10, assign23010_e17664_d_n13,) = {
    if (locals.var_guard430 != 0.0) {
        let assign23010_e17656: f64 = (0.93 * locals.var_pb20);
        let assign23010_e17659: f64 = (locals.var_vbsz2 + locals.var_bs12);
        let assign23010_e17660: f64 = (assign23010_e17656 - assign23010_e17659);
        let assign23010_e17662: f64 = (assign23010_e17660 - 0.001);
        (assign23010_e17662, ((0.93 * locals.var_pb20_dn0) - (locals.var_vbsz2_dn0 + locals.var_bs12_dn0)), ((0.93 * locals.var_pb20_dn2) - (locals.var_vbsz2_dn2 + locals.var_bs12_dn2)), ((0.93 * locals.var_pb20_dn4) - (locals.var_vbsz2_dn4 + locals.var_bs12_dn4)), ((0.93 * locals.var_pb20_dn5) - (locals.var_vbsz2_dn5 + locals.var_bs12_dn5)), ((0.93 * locals.var_pb20_dn6) - (locals.var_vbsz2_dn6 + locals.var_bs12_dn6)), ((0.93 * locals.var_pb20_dn7) - (locals.var_vbsz2_dn7 + locals.var_bs12_dn7)), ((0.93 * locals.var_pb20_dn8) - (locals.var_vbsz2_dn8 + locals.var_bs12_dn8)), ((0.93 * locals.var_pb20_dn9) - (locals.var_vbsz2_dn9 + locals.var_bs12_dn9)), ((0.93 * locals.var_pb20_dn10) - (locals.var_vbsz2_dn10 + locals.var_bs12_dn10)), ((0.93 * locals.var_pb20_dn13) - (locals.var_vbsz2_dn13 + locals.var_bs12_dn13)),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13,)
    }
};
        locals.var_tmf1 = assign23010_e17664;
        locals.var_tmf1_dn0 = assign23010_e17664_d_n0;
        locals.var_tmf1_dn2 = assign23010_e17664_d_n2;
        locals.var_tmf1_dn4 = assign23010_e17664_d_n4;
        locals.var_tmf1_dn5 = assign23010_e17664_d_n5;
        locals.var_tmf1_dn6 = assign23010_e17664_d_n6;
        locals.var_tmf1_dn7 = assign23010_e17664_d_n7;
        locals.var_tmf1_dn8 = assign23010_e17664_d_n8;
        locals.var_tmf1_dn9 = assign23010_e17664_d_n9;
        locals.var_tmf1_dn10 = assign23010_e17664_d_n10;
        locals.var_tmf1_dn13 = assign23010_e17664_d_n13;
        locals.var_tmf1_rv = 0.0;

        let (assign23020_e17674, assign23020_e17674_d_n0, assign23020_e17674_d_n2, assign23020_e17674_d_n4, assign23020_e17674_d_n5, assign23020_e17674_d_n6, assign23020_e17674_d_n7, assign23020_e17674_d_n8, assign23020_e17674_d_n9, assign23020_e17674_d_n10, assign23020_e17674_d_n13,) = {
    if (locals.var_guard430 != 0.0) {
        let assign23020_e17669: f64 = (0.93 * locals.var_pb20);
        let assign23020_e17670: f64 = (4.0 * assign23020_e17669);
        let assign23020_e17672: f64 = (assign23020_e17670 * 0.001);
        (assign23020_e17672, ((4.0 * (0.93 * locals.var_pb20_dn0)) * 0.001), ((4.0 * (0.93 * locals.var_pb20_dn2)) * 0.001), ((4.0 * (0.93 * locals.var_pb20_dn4)) * 0.001), ((4.0 * (0.93 * locals.var_pb20_dn5)) * 0.001), ((4.0 * (0.93 * locals.var_pb20_dn6)) * 0.001), ((4.0 * (0.93 * locals.var_pb20_dn7)) * 0.001), ((4.0 * (0.93 * locals.var_pb20_dn8)) * 0.001), ((4.0 * (0.93 * locals.var_pb20_dn9)) * 0.001), ((4.0 * (0.93 * locals.var_pb20_dn10)) * 0.001), ((4.0 * (0.93 * locals.var_pb20_dn13)) * 0.001),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign23020_e17674;
        locals.var_tmf2_dn0 = assign23020_e17674_d_n0;
        locals.var_tmf2_dn2 = assign23020_e17674_d_n2;
        locals.var_tmf2_dn4 = assign23020_e17674_d_n4;
        locals.var_tmf2_dn5 = assign23020_e17674_d_n5;
        locals.var_tmf2_dn6 = assign23020_e17674_d_n6;
        locals.var_tmf2_dn7 = assign23020_e17674_d_n7;
        locals.var_tmf2_dn8 = assign23020_e17674_d_n8;
        locals.var_tmf2_dn9 = assign23020_e17674_d_n9;
        locals.var_tmf2_dn10 = assign23020_e17674_d_n10;
        locals.var_tmf2_dn13 = assign23020_e17674_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign23030_e17684, assign23030_e17684_d_n0, assign23030_e17684_d_n2, assign23030_e17684_d_n4, assign23030_e17684_d_n5, assign23030_e17684_d_n6, assign23030_e17684_d_n7, assign23030_e17684_d_n8, assign23030_e17684_d_n9, assign23030_e17684_d_n10, assign23030_e17684_d_n13,) = {
    if (locals.var_guard430 != 0.0) {
        let (assign23030_e17682, assign23030_e17682_d_n0, assign23030_e17682_d_n2, assign23030_e17682_d_n4, assign23030_e17682_d_n5, assign23030_e17682_d_n6, assign23030_e17682_d_n7, assign23030_e17682_d_n8, assign23030_e17682_d_n9, assign23030_e17682_d_n10, assign23030_e17682_d_n13,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
            } else {
                let assign23030_e17681: f64 = (-locals.var_tmf2);
                (assign23030_e17681, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn13),)
            }
        };
        (assign23030_e17682, assign23030_e17682_d_n0, assign23030_e17682_d_n2, assign23030_e17682_d_n4, assign23030_e17682_d_n5, assign23030_e17682_d_n6, assign23030_e17682_d_n7, assign23030_e17682_d_n8, assign23030_e17682_d_n9, assign23030_e17682_d_n10, assign23030_e17682_d_n13,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign23030_e17684;
        locals.var_tmf2_dn0 = assign23030_e17684_d_n0;
        locals.var_tmf2_dn2 = assign23030_e17684_d_n2;
        locals.var_tmf2_dn4 = assign23030_e17684_d_n4;
        locals.var_tmf2_dn5 = assign23030_e17684_d_n5;
        locals.var_tmf2_dn6 = assign23030_e17684_d_n6;
        locals.var_tmf2_dn7 = assign23030_e17684_d_n7;
        locals.var_tmf2_dn8 = assign23030_e17684_d_n8;
        locals.var_tmf2_dn9 = assign23030_e17684_d_n9;
        locals.var_tmf2_dn10 = assign23030_e17684_d_n10;
        locals.var_tmf2_dn13 = assign23030_e17684_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign23040_e17693, assign23040_e17693_d_n0, assign23040_e17693_d_n2, assign23040_e17693_d_n4, assign23040_e17693_d_n5, assign23040_e17693_d_n6, assign23040_e17693_d_n7, assign23040_e17693_d_n8, assign23040_e17693_d_n9, assign23040_e17693_d_n10, assign23040_e17693_d_n13,) = {
    if (locals.var_guard430 != 0.0) {
        let assign23040_e17688: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign23040_e17690: f64 = (assign23040_e17688 + locals.var_tmf2);
        let assign23040_e17691: f64 = (assign23040_e17690).sqrt();
        (assign23040_e17691, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign23040_e17691)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign23040_e17691)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign23040_e17691)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign23040_e17691)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign23040_e17691)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign23040_e17691)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign23040_e17691)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign23040_e17691)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign23040_e17691)), ((((locals.var_tmf1_dn13 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn13)) + locals.var_tmf2_dn13) / (2.0 * assign23040_e17691)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign23040_e17693;
        locals.var_tmf2_dn0 = assign23040_e17693_d_n0;
        locals.var_tmf2_dn2 = assign23040_e17693_d_n2;
        locals.var_tmf2_dn4 = assign23040_e17693_d_n4;
        locals.var_tmf2_dn5 = assign23040_e17693_d_n5;
        locals.var_tmf2_dn6 = assign23040_e17693_d_n6;
        locals.var_tmf2_dn7 = assign23040_e17693_d_n7;
        locals.var_tmf2_dn8 = assign23040_e17693_d_n8;
        locals.var_tmf2_dn9 = assign23040_e17693_d_n9;
        locals.var_tmf2_dn10 = assign23040_e17693_d_n10;
        locals.var_tmf2_dn13 = assign23040_e17693_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign23050_e17703, assign23050_e17703_d_n0, assign23050_e17703_d_n2, assign23050_e17703_d_n4, assign23050_e17703_d_n5, assign23050_e17703_d_n6, assign23050_e17703_d_n7, assign23050_e17703_d_n8, assign23050_e17703_d_n9, assign23050_e17703_d_n10, assign23050_e17703_d_n13,) = {
    if (locals.var_guard430 != 0.0) {
        let assign23050_e17699: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign23050_e17700: f64 = (1.0 + assign23050_e17699);
        let assign23050_e17701: f64 = (0.5 * assign23050_e17700);
        (assign23050_e17701, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn13 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn13)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign23050_e17703;
        locals.var_t0_dn0 = assign23050_e17703_d_n0;
        locals.var_t0_dn2 = assign23050_e17703_d_n2;
        locals.var_t0_dn4 = assign23050_e17703_d_n4;
        locals.var_t0_dn5 = assign23050_e17703_d_n5;
        locals.var_t0_dn6 = assign23050_e17703_d_n6;
        locals.var_t0_dn7 = assign23050_e17703_d_n7;
        locals.var_t0_dn8 = assign23050_e17703_d_n8;
        locals.var_t0_dn9 = assign23050_e17703_d_n9;
        locals.var_t0_dn10 = assign23050_e17703_d_n10;
        locals.var_t0_dn13 = assign23050_e17703_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign23060_e17715, assign23060_e17715_d_n0, assign23060_e17715_d_n2, assign23060_e17715_d_n4, assign23060_e17715_d_n5, assign23060_e17715_d_n6, assign23060_e17715_d_n7, assign23060_e17715_d_n8, assign23060_e17715_d_n9, assign23060_e17715_d_n10, assign23060_e17715_d_n13,) = {
    if (locals.var_guard430 != 0.0) {
        let assign23060_e17707: f64 = (0.93 * locals.var_pb20);
        let assign23060_e17711: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign23060_e17712: f64 = (0.5 * assign23060_e17711);
        let assign23060_e17713: f64 = (assign23060_e17707 - assign23060_e17712);
        (assign23060_e17713, ((0.93 * locals.var_pb20_dn0) - (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), ((0.93 * locals.var_pb20_dn2) - (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), ((0.93 * locals.var_pb20_dn4) - (0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4))), ((0.93 * locals.var_pb20_dn5) - (0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5))), ((0.93 * locals.var_pb20_dn6) - (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), ((0.93 * locals.var_pb20_dn7) - (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))), ((0.93 * locals.var_pb20_dn8) - (0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8))), ((0.93 * locals.var_pb20_dn9) - (0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9))), ((0.93 * locals.var_pb20_dn10) - (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), ((0.93 * locals.var_pb20_dn13) - (0.5 * (locals.var_tmf1_dn13 + locals.var_tmf2_dn13))),)
    } else {
        (locals.var_t10, locals.var_t10_dn0, locals.var_t10_dn2, locals.var_t10_dn4, locals.var_t10_dn5, locals.var_t10_dn6, locals.var_t10_dn7, locals.var_t10_dn8, locals.var_t10_dn9, locals.var_t10_dn10, locals.var_t10_dn13,)
    }
};
        locals.var_t10 = assign23060_e17715;
        locals.var_t10_dn0 = assign23060_e17715_d_n0;
        locals.var_t10_dn2 = assign23060_e17715_d_n2;
        locals.var_t10_dn4 = assign23060_e17715_d_n4;
        locals.var_t10_dn5 = assign23060_e17715_d_n5;
        locals.var_t10_dn6 = assign23060_e17715_d_n6;
        locals.var_t10_dn7 = assign23060_e17715_d_n7;
        locals.var_t10_dn8 = assign23060_e17715_d_n8;
        locals.var_t10_dn9 = assign23060_e17715_d_n9;
        locals.var_t10_dn10 = assign23060_e17715_d_n10;
        locals.var_t10_dn13 = assign23060_e17715_d_n13;
        locals.var_t10_rv = 0.0;

        let (assign23070_e17724, assign23070_e17724_d_n0, assign23070_e17724_d_n2, assign23070_e17724_d_n4, assign23070_e17724_d_n5, assign23070_e17724_d_n6, assign23070_e17724_d_n7, assign23070_e17724_d_n8, assign23070_e17724_d_n9, assign23070_e17724_d_n10, assign23070_e17724_d_n13,) = {
    if (locals.var_guard430 != 0.0) {
        let assign23070_e17720: f64 = (locals.var_pb20 - locals.var_t10);
        let assign23070_e17721: f64 = (locals.var_t1 * assign23070_e17720);
        let assign23070_e17722: f64 = (assign23070_e17721).sqrt();
        (assign23070_e17722, (((locals.var_t1_dn0 * assign23070_e17720) + (locals.var_t1 * (locals.var_pb20_dn0 - locals.var_t10_dn0))) / (2.0 * assign23070_e17722)), (((locals.var_t1_dn2 * assign23070_e17720) + (locals.var_t1 * (locals.var_pb20_dn2 - locals.var_t10_dn2))) / (2.0 * assign23070_e17722)), (((locals.var_t1_dn4 * assign23070_e17720) + (locals.var_t1 * (locals.var_pb20_dn4 - locals.var_t10_dn4))) / (2.0 * assign23070_e17722)), (((locals.var_t1_dn5 * assign23070_e17720) + (locals.var_t1 * (locals.var_pb20_dn5 - locals.var_t10_dn5))) / (2.0 * assign23070_e17722)), (((locals.var_t1_dn6 * assign23070_e17720) + (locals.var_t1 * (locals.var_pb20_dn6 - locals.var_t10_dn6))) / (2.0 * assign23070_e17722)), (((locals.var_t1_dn7 * assign23070_e17720) + (locals.var_t1 * (locals.var_pb20_dn7 - locals.var_t10_dn7))) / (2.0 * assign23070_e17722)), (((locals.var_t1_dn8 * assign23070_e17720) + (locals.var_t1 * (locals.var_pb20_dn8 - locals.var_t10_dn8))) / (2.0 * assign23070_e17722)), (((locals.var_t1_dn9 * assign23070_e17720) + (locals.var_t1 * (locals.var_pb20_dn9 - locals.var_t10_dn9))) / (2.0 * assign23070_e17722)), (((locals.var_t1_dn10 * assign23070_e17720) + (locals.var_t1 * (locals.var_pb20_dn10 - locals.var_t10_dn10))) / (2.0 * assign23070_e17722)), (((locals.var_t1_dn13 * assign23070_e17720) + (locals.var_t1 * (locals.var_pb20_dn13 - locals.var_t10_dn13))) / (2.0 * assign23070_e17722)),)
    } else {
        (locals.var_qbmm, locals.var_qbmm_dn0, locals.var_qbmm_dn2, locals.var_qbmm_dn4, locals.var_qbmm_dn5, locals.var_qbmm_dn6, locals.var_qbmm_dn7, locals.var_qbmm_dn8, locals.var_qbmm_dn9, locals.var_qbmm_dn10, locals.var_qbmm_dn13,)
    }
};
        locals.var_qbmm = assign23070_e17724;
        locals.var_qbmm_dn0 = assign23070_e17724_d_n0;
        locals.var_qbmm_dn2 = assign23070_e17724_d_n2;
        locals.var_qbmm_dn4 = assign23070_e17724_d_n4;
        locals.var_qbmm_dn5 = assign23070_e17724_d_n5;
        locals.var_qbmm_dn6 = assign23070_e17724_d_n6;
        locals.var_qbmm_dn7 = assign23070_e17724_d_n7;
        locals.var_qbmm_dn8 = assign23070_e17724_d_n8;
        locals.var_qbmm_dn9 = assign23070_e17724_d_n9;
        locals.var_qbmm_dn10 = assign23070_e17724_d_n10;
        locals.var_qbmm_dn13 = assign23070_e17724_d_n13;
        locals.var_qbmm_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_60(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign23080_e17730, assign23080_e17730_d_n0, assign23080_e17730_d_n2, assign23080_e17730_d_n4, assign23080_e17730_d_n5, assign23080_e17730_d_n6, assign23080_e17730_d_n7, assign23080_e17730_d_n8, assign23080_e17730_d_n9, assign23080_e17730_d_n10, assign23080_e17730_d_n13,) = {
    if (locals.var_guard430 != 0.0) {
        let assign23080_e17728: f64 = (locals.var_t0 / locals.var_qbmm);
        (assign23080_e17728, (((locals.var_t0_dn0 * locals.var_qbmm) - (locals.var_t0 * locals.var_qbmm_dn0)) / (locals.var_qbmm * locals.var_qbmm)), (((locals.var_t0_dn2 * locals.var_qbmm) - (locals.var_t0 * locals.var_qbmm_dn2)) / (locals.var_qbmm * locals.var_qbmm)), (((locals.var_t0_dn4 * locals.var_qbmm) - (locals.var_t0 * locals.var_qbmm_dn4)) / (locals.var_qbmm * locals.var_qbmm)), (((locals.var_t0_dn5 * locals.var_qbmm) - (locals.var_t0 * locals.var_qbmm_dn5)) / (locals.var_qbmm * locals.var_qbmm)), (((locals.var_t0_dn6 * locals.var_qbmm) - (locals.var_t0 * locals.var_qbmm_dn6)) / (locals.var_qbmm * locals.var_qbmm)), (((locals.var_t0_dn7 * locals.var_qbmm) - (locals.var_t0 * locals.var_qbmm_dn7)) / (locals.var_qbmm * locals.var_qbmm)), (((locals.var_t0_dn8 * locals.var_qbmm) - (locals.var_t0 * locals.var_qbmm_dn8)) / (locals.var_qbmm * locals.var_qbmm)), (((locals.var_t0_dn9 * locals.var_qbmm) - (locals.var_t0 * locals.var_qbmm_dn9)) / (locals.var_qbmm * locals.var_qbmm)), (((locals.var_t0_dn10 * locals.var_qbmm) - (locals.var_t0 * locals.var_qbmm_dn10)) / (locals.var_qbmm * locals.var_qbmm)), (((locals.var_t0_dn13 * locals.var_qbmm) - (locals.var_t0 * locals.var_qbmm_dn13)) / (locals.var_qbmm * locals.var_qbmm)),)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn13,)
    }
};
        locals.var_t9 = assign23080_e17730;
        locals.var_t9_dn0 = assign23080_e17730_d_n0;
        locals.var_t9_dn2 = assign23080_e17730_d_n2;
        locals.var_t9_dn4 = assign23080_e17730_d_n4;
        locals.var_t9_dn5 = assign23080_e17730_d_n5;
        locals.var_t9_dn6 = assign23080_e17730_d_n6;
        locals.var_t9_dn7 = assign23080_e17730_d_n7;
        locals.var_t9_dn8 = assign23080_e17730_d_n8;
        locals.var_t9_dn9 = assign23080_e17730_d_n9;
        locals.var_t9_dn10 = assign23080_e17730_d_n10;
        locals.var_t9_dn13 = assign23080_e17730_d_n13;
        locals.var_t9_rv = 0.0;

        let (assign23090_e17738, assign23090_e17738_d_n0, assign23090_e17738_d_n2, assign23090_e17738_d_n4, assign23090_e17738_d_n5, assign23090_e17738_d_n6, assign23090_e17738_d_n7, assign23090_e17738_d_n8, assign23090_e17738_d_n9, assign23090_e17738_d_n10, assign23090_e17738_d_n13,) = {
    if (locals.var_guard430 != 0.0) {
        let assign23090_e17734: f64 = (locals.var_qb0 - locals.var_qbmm);
        let assign23090_e17736: f64 = (assign23090_e17734 * locals.var_cox_inv);
        (assign23090_e17736, (((locals.var_qb0_dn0 - locals.var_qbmm_dn0) * locals.var_cox_inv) + (assign23090_e17734 * locals.var_cox_inv_dn0)), (((locals.var_qb0_dn2 - locals.var_qbmm_dn2) * locals.var_cox_inv) + (assign23090_e17734 * locals.var_cox_inv_dn2)), (((locals.var_qb0_dn4 - locals.var_qbmm_dn4) * locals.var_cox_inv) + (assign23090_e17734 * locals.var_cox_inv_dn4)), (((locals.var_qb0_dn5 - locals.var_qbmm_dn5) * locals.var_cox_inv) + (assign23090_e17734 * locals.var_cox_inv_dn5)), (((locals.var_qb0_dn6 - locals.var_qbmm_dn6) * locals.var_cox_inv) + (assign23090_e17734 * locals.var_cox_inv_dn6)), (((locals.var_qb0_dn7 - locals.var_qbmm_dn7) * locals.var_cox_inv) + (assign23090_e17734 * locals.var_cox_inv_dn7)), (((locals.var_qb0_dn8 - locals.var_qbmm_dn8) * locals.var_cox_inv) + (assign23090_e17734 * locals.var_cox_inv_dn8)), (((locals.var_qb0_dn9 - locals.var_qbmm_dn9) * locals.var_cox_inv) + (assign23090_e17734 * locals.var_cox_inv_dn9)), (((locals.var_qb0_dn10 - locals.var_qbmm_dn10) * locals.var_cox_inv) + (assign23090_e17734 * locals.var_cox_inv_dn10)), (((locals.var_qb0_dn13 - locals.var_qbmm_dn13) * locals.var_cox_inv) + (assign23090_e17734 * locals.var_cox_inv_dn13)),)
    } else {
        (locals.var_dqb, locals.var_dqb_dn0, locals.var_dqb_dn2, locals.var_dqb_dn4, locals.var_dqb_dn5, locals.var_dqb_dn6, locals.var_dqb_dn7, locals.var_dqb_dn8, locals.var_dqb_dn9, locals.var_dqb_dn10, locals.var_dqb_dn13,)
    }
};
        locals.var_dqb = assign23090_e17738;
        locals.var_dqb_dn0 = assign23090_e17738_d_n0;
        locals.var_dqb_dn2 = assign23090_e17738_d_n2;
        locals.var_dqb_dn4 = assign23090_e17738_d_n4;
        locals.var_dqb_dn5 = assign23090_e17738_d_n5;
        locals.var_dqb_dn6 = assign23090_e17738_d_n6;
        locals.var_dqb_dn7 = assign23090_e17738_d_n7;
        locals.var_dqb_dn8 = assign23090_e17738_d_n8;
        locals.var_dqb_dn9 = assign23090_e17738_d_n9;
        locals.var_dqb_dn10 = assign23090_e17738_d_n10;
        locals.var_dqb_dn13 = assign23090_e17738_d_n13;
        locals.var_dqb_rv = 0.0;

        let (assign23100_e17748, assign23100_e17748_d_n0, assign23100_e17748_d_n2, assign23100_e17748_d_n4, assign23100_e17748_d_n5, assign23100_e17748_d_n6, assign23100_e17748_d_n7, assign23100_e17748_d_n8, assign23100_e17748_d_n9, assign23100_e17748_d_n10, assign23100_e17748_d_n13,) = {
    if (locals.var_guard430 != 0.0) {
        let assign23100_e17742: f64 = (2.0 * 1.6021918e-19);
        let assign23100_e17744: f64 = (assign23100_e17742 * locals.var_ef_nsubc);
        let assign23100_e17746: f64 = (assign23100_e17744 * 1.034943e-10);
        (assign23100_e17746, ((assign23100_e17742 * locals.var_ef_nsubc_dn0) * 1.034943e-10), ((assign23100_e17742 * locals.var_ef_nsubc_dn2) * 1.034943e-10), ((assign23100_e17742 * locals.var_ef_nsubc_dn4) * 1.034943e-10), ((assign23100_e17742 * locals.var_ef_nsubc_dn5) * 1.034943e-10), ((assign23100_e17742 * locals.var_ef_nsubc_dn6) * 1.034943e-10), ((assign23100_e17742 * locals.var_ef_nsubc_dn7) * 1.034943e-10), ((assign23100_e17742 * locals.var_ef_nsubc_dn8) * 1.034943e-10), ((assign23100_e17742 * locals.var_ef_nsubc_dn9) * 1.034943e-10), ((assign23100_e17742 * locals.var_ef_nsubc_dn10) * 1.034943e-10), ((assign23100_e17742 * locals.var_ef_nsubc_dn13) * 1.034943e-10),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign23100_e17748;
        locals.var_t1_dn0 = assign23100_e17748_d_n0;
        locals.var_t1_dn2 = assign23100_e17748_d_n2;
        locals.var_t1_dn4 = assign23100_e17748_d_n4;
        locals.var_t1_dn5 = assign23100_e17748_d_n5;
        locals.var_t1_dn6 = assign23100_e17748_d_n6;
        locals.var_t1_dn7 = assign23100_e17748_d_n7;
        locals.var_t1_dn8 = assign23100_e17748_d_n8;
        locals.var_t1_dn9 = assign23100_e17748_d_n9;
        locals.var_t1_dn10 = assign23100_e17748_d_n10;
        locals.var_t1_dn13 = assign23100_e17748_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign23110_e17757, assign23110_e17757_d_n0, assign23110_e17757_d_n2, assign23110_e17757_d_n4, assign23110_e17757_d_n5, assign23110_e17757_d_n6, assign23110_e17757_d_n7, assign23110_e17757_d_n8, assign23110_e17757_d_n9, assign23110_e17757_d_n10, assign23110_e17757_d_n13,) = {
    if (locals.var_guard430 != 0.0) {
        let assign23110_e17753: f64 = (locals.var_pb2c - locals.var_vbsz2);
        let assign23110_e17754: f64 = (locals.var_t1 * assign23110_e17753);
        let assign23110_e17755: f64 = (assign23110_e17754).sqrt();
        (assign23110_e17755, (((locals.var_t1_dn0 * assign23110_e17753) + (locals.var_t1 * (locals.var_pb2c_dn0 - locals.var_vbsz2_dn0))) / (2.0 * assign23110_e17755)), (((locals.var_t1_dn2 * assign23110_e17753) + (locals.var_t1 * (locals.var_pb2c_dn2 - locals.var_vbsz2_dn2))) / (2.0 * assign23110_e17755)), (((locals.var_t1_dn4 * assign23110_e17753) + (locals.var_t1 * (locals.var_pb2c_dn4 - locals.var_vbsz2_dn4))) / (2.0 * assign23110_e17755)), (((locals.var_t1_dn5 * assign23110_e17753) + (locals.var_t1 * (locals.var_pb2c_dn5 - locals.var_vbsz2_dn5))) / (2.0 * assign23110_e17755)), (((locals.var_t1_dn6 * assign23110_e17753) + (locals.var_t1 * (locals.var_pb2c_dn6 - locals.var_vbsz2_dn6))) / (2.0 * assign23110_e17755)), (((locals.var_t1_dn7 * assign23110_e17753) + (locals.var_t1 * (locals.var_pb2c_dn7 - locals.var_vbsz2_dn7))) / (2.0 * assign23110_e17755)), (((locals.var_t1_dn8 * assign23110_e17753) + (locals.var_t1 * (locals.var_pb2c_dn8 - locals.var_vbsz2_dn8))) / (2.0 * assign23110_e17755)), (((locals.var_t1_dn9 * assign23110_e17753) + (locals.var_t1 * (locals.var_pb2c_dn9 - locals.var_vbsz2_dn9))) / (2.0 * assign23110_e17755)), (((locals.var_t1_dn10 * assign23110_e17753) + (locals.var_t1 * (locals.var_pb2c_dn10 - locals.var_vbsz2_dn10))) / (2.0 * assign23110_e17755)), (((locals.var_t1_dn13 * assign23110_e17753) + (locals.var_t1 * (locals.var_pb2c_dn13 - locals.var_vbsz2_dn13))) / (2.0 * assign23110_e17755)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign23110_e17757;
        locals.var_t2_dn0 = assign23110_e17757_d_n0;
        locals.var_t2_dn2 = assign23110_e17757_d_n2;
        locals.var_t2_dn4 = assign23110_e17757_d_n4;
        locals.var_t2_dn5 = assign23110_e17757_d_n5;
        locals.var_t2_dn6 = assign23110_e17757_d_n6;
        locals.var_t2_dn7 = assign23110_e17757_d_n7;
        locals.var_t2_dn8 = assign23110_e17757_d_n8;
        locals.var_t2_dn9 = assign23110_e17757_d_n9;
        locals.var_t2_dn10 = assign23110_e17757_d_n10;
        locals.var_t2_dn13 = assign23110_e17757_d_n13;
        locals.var_t2_rv = 0.0;

        let (assign23120_e17767, assign23120_e17767_d_n0, assign23120_e17767_d_n2, assign23120_e17767_d_n4, assign23120_e17767_d_n5, assign23120_e17767_d_n6, assign23120_e17767_d_n7, assign23120_e17767_d_n8, assign23120_e17767_d_n9, assign23120_e17767_d_n10, assign23120_e17767_d_n13,) = {
    if (locals.var_guard430 != 0.0) {
        let assign23120_e17761: f64 = (locals.var_pb2c + locals.var_vfb);
        let assign23120_e17764: f64 = (locals.var_t2 * locals.var_cox_inv);
        let assign23120_e17765: f64 = (assign23120_e17761 + assign23120_e17764);
        (assign23120_e17765, (locals.var_pb2c_dn0 + ((locals.var_t2_dn0 * locals.var_cox_inv) + (locals.var_t2 * locals.var_cox_inv_dn0))), (locals.var_pb2c_dn2 + ((locals.var_t2_dn2 * locals.var_cox_inv) + (locals.var_t2 * locals.var_cox_inv_dn2))), (locals.var_pb2c_dn4 + ((locals.var_t2_dn4 * locals.var_cox_inv) + (locals.var_t2 * locals.var_cox_inv_dn4))), (locals.var_pb2c_dn5 + ((locals.var_t2_dn5 * locals.var_cox_inv) + (locals.var_t2 * locals.var_cox_inv_dn5))), (locals.var_pb2c_dn6 + ((locals.var_t2_dn6 * locals.var_cox_inv) + (locals.var_t2 * locals.var_cox_inv_dn6))), (locals.var_pb2c_dn7 + ((locals.var_t2_dn7 * locals.var_cox_inv) + (locals.var_t2 * locals.var_cox_inv_dn7))), (locals.var_pb2c_dn8 + ((locals.var_t2_dn8 * locals.var_cox_inv) + (locals.var_t2 * locals.var_cox_inv_dn8))), (locals.var_pb2c_dn9 + ((locals.var_t2_dn9 * locals.var_cox_inv) + (locals.var_t2 * locals.var_cox_inv_dn9))), (locals.var_pb2c_dn10 + ((locals.var_t2_dn10 * locals.var_cox_inv) + (locals.var_t2 * locals.var_cox_inv_dn10))), (locals.var_pb2c_dn13 + ((locals.var_t2_dn13 * locals.var_cox_inv) + (locals.var_t2 * locals.var_cox_inv_dn13))),)
    } else {
        (locals.var_vth0, locals.var_vth0_dn0, locals.var_vth0_dn2, locals.var_vth0_dn4, locals.var_vth0_dn5, locals.var_vth0_dn6, locals.var_vth0_dn7, locals.var_vth0_dn8, locals.var_vth0_dn9, locals.var_vth0_dn10, locals.var_vth0_dn13,)
    }
};
        locals.var_vth0 = assign23120_e17767;
        locals.var_vth0_dn0 = assign23120_e17767_d_n0;
        locals.var_vth0_dn2 = assign23120_e17767_d_n2;
        locals.var_vth0_dn4 = assign23120_e17767_d_n4;
        locals.var_vth0_dn5 = assign23120_e17767_d_n5;
        locals.var_vth0_dn6 = assign23120_e17767_d_n6;
        locals.var_vth0_dn7 = assign23120_e17767_d_n7;
        locals.var_vth0_dn8 = assign23120_e17767_d_n8;
        locals.var_vth0_dn9 = assign23120_e17767_d_n9;
        locals.var_vth0_dn10 = assign23120_e17767_d_n10;
        locals.var_vth0_dn13 = assign23120_e17767_d_n13;
        locals.var_vth0_rv = 0.0;

        let (assign23130_e17777, assign23130_e17777_d_n0, assign23130_e17777_d_n2, assign23130_e17777_d_n4, assign23130_e17777_d_n5, assign23130_e17777_d_n6, assign23130_e17777_d_n7, assign23130_e17777_d_n8, assign23130_e17777_d_n9, assign23130_e17777_d_n10, assign23130_e17777_d_n13,) = {
    if (locals.var_guard430 != 0.0) {
        let assign23130_e17771: f64 = (0.5 * locals.var_t1);
        let assign23130_e17773: f64 = (assign23130_e17771 / locals.var_t2);
        let assign23130_e17775: f64 = (assign23130_e17773 * locals.var_cox_inv);
        (assign23130_e17775, ((((((0.5 * locals.var_t1_dn0) * locals.var_t2) - (assign23130_e17771 * locals.var_t2_dn0)) / (locals.var_t2 * locals.var_t2)) * locals.var_cox_inv) + (assign23130_e17773 * locals.var_cox_inv_dn0)), ((((((0.5 * locals.var_t1_dn2) * locals.var_t2) - (assign23130_e17771 * locals.var_t2_dn2)) / (locals.var_t2 * locals.var_t2)) * locals.var_cox_inv) + (assign23130_e17773 * locals.var_cox_inv_dn2)), ((((((0.5 * locals.var_t1_dn4) * locals.var_t2) - (assign23130_e17771 * locals.var_t2_dn4)) / (locals.var_t2 * locals.var_t2)) * locals.var_cox_inv) + (assign23130_e17773 * locals.var_cox_inv_dn4)), ((((((0.5 * locals.var_t1_dn5) * locals.var_t2) - (assign23130_e17771 * locals.var_t2_dn5)) / (locals.var_t2 * locals.var_t2)) * locals.var_cox_inv) + (assign23130_e17773 * locals.var_cox_inv_dn5)), ((((((0.5 * locals.var_t1_dn6) * locals.var_t2) - (assign23130_e17771 * locals.var_t2_dn6)) / (locals.var_t2 * locals.var_t2)) * locals.var_cox_inv) + (assign23130_e17773 * locals.var_cox_inv_dn6)), ((((((0.5 * locals.var_t1_dn7) * locals.var_t2) - (assign23130_e17771 * locals.var_t2_dn7)) / (locals.var_t2 * locals.var_t2)) * locals.var_cox_inv) + (assign23130_e17773 * locals.var_cox_inv_dn7)), ((((((0.5 * locals.var_t1_dn8) * locals.var_t2) - (assign23130_e17771 * locals.var_t2_dn8)) / (locals.var_t2 * locals.var_t2)) * locals.var_cox_inv) + (assign23130_e17773 * locals.var_cox_inv_dn8)), ((((((0.5 * locals.var_t1_dn9) * locals.var_t2) - (assign23130_e17771 * locals.var_t2_dn9)) / (locals.var_t2 * locals.var_t2)) * locals.var_cox_inv) + (assign23130_e17773 * locals.var_cox_inv_dn9)), ((((((0.5 * locals.var_t1_dn10) * locals.var_t2) - (assign23130_e17771 * locals.var_t2_dn10)) / (locals.var_t2 * locals.var_t2)) * locals.var_cox_inv) + (assign23130_e17773 * locals.var_cox_inv_dn10)), ((((((0.5 * locals.var_t1_dn13) * locals.var_t2) - (assign23130_e17771 * locals.var_t2_dn13)) / (locals.var_t2 * locals.var_t2)) * locals.var_cox_inv) + (assign23130_e17773 * locals.var_cox_inv_dn13)),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn13,)
    }
};
        locals.var_t3 = assign23130_e17777;
        locals.var_t3_dn0 = assign23130_e17777_d_n0;
        locals.var_t3_dn2 = assign23130_e17777_d_n2;
        locals.var_t3_dn4 = assign23130_e17777_d_n4;
        locals.var_t3_dn5 = assign23130_e17777_d_n5;
        locals.var_t3_dn6 = assign23130_e17777_d_n6;
        locals.var_t3_dn7 = assign23130_e17777_d_n7;
        locals.var_t3_dn8 = assign23130_e17777_d_n8;
        locals.var_t3_dn9 = assign23130_e17777_d_n9;
        locals.var_t3_dn10 = assign23130_e17777_d_n10;
        locals.var_t3_dn13 = assign23130_e17777_d_n13;
        locals.var_t3_rv = 0.0;

        let (assign23140_e17783, assign23140_e17783_d_n0, assign23140_e17783_d_n2, assign23140_e17783_d_n4, assign23140_e17783_d_n5, assign23140_e17783_d_n6, assign23140_e17783_d_n7, assign23140_e17783_d_n8, assign23140_e17783_d_n9, assign23140_e17783_d_n10, assign23140_e17783_d_n13,) = {
    if (locals.var_guard430 != 0.0) {
        let assign23140_e17781: f64 = (1.034943e-10 * locals.var_cox_inv);
        (assign23140_e17781, (1.034943e-10 * locals.var_cox_inv_dn0), (1.034943e-10 * locals.var_cox_inv_dn2), (1.034943e-10 * locals.var_cox_inv_dn4), (1.034943e-10 * locals.var_cox_inv_dn5), (1.034943e-10 * locals.var_cox_inv_dn6), (1.034943e-10 * locals.var_cox_inv_dn7), (1.034943e-10 * locals.var_cox_inv_dn8), (1.034943e-10 * locals.var_cox_inv_dn9), (1.034943e-10 * locals.var_cox_inv_dn10), (1.034943e-10 * locals.var_cox_inv_dn13),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign23140_e17783;
        locals.var_t1_dn0 = assign23140_e17783_d_n0;
        locals.var_t1_dn2 = assign23140_e17783_d_n2;
        locals.var_t1_dn4 = assign23140_e17783_d_n4;
        locals.var_t1_dn5 = assign23140_e17783_d_n5;
        locals.var_t1_dn6 = assign23140_e17783_d_n6;
        locals.var_t1_dn7 = assign23140_e17783_d_n7;
        locals.var_t1_dn8 = assign23140_e17783_d_n8;
        locals.var_t1_dn9 = assign23140_e17783_d_n9;
        locals.var_t1_dn10 = assign23140_e17783_d_n10;
        locals.var_t1_dn13 = assign23140_e17783_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign23150_e17787, assign23150_e17787_d_n0, assign23150_e17787_d_n2, assign23150_e17787_d_n4, assign23150_e17787_d_n5, assign23150_e17787_d_n6, assign23150_e17787_d_n7, assign23150_e17787_d_n8, assign23150_e17787_d_n9, assign23150_e17787_d_n10, assign23150_e17787_d_n13,) = {
    if (locals.var_guard430 != 0.0) {
        (locals.var_wdplp, locals.var_wdplp_dn0, locals.var_wdplp_dn2, locals.var_wdplp_dn4, locals.var_wdplp_dn5, locals.var_wdplp_dn6, locals.var_wdplp_dn7, locals.var_wdplp_dn8, locals.var_wdplp_dn9, locals.var_wdplp_dn10, locals.var_wdplp_dn13,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign23150_e17787;
        locals.var_t2_dn0 = assign23150_e17787_d_n0;
        locals.var_t2_dn2 = assign23150_e17787_d_n2;
        locals.var_t2_dn4 = assign23150_e17787_d_n4;
        locals.var_t2_dn5 = assign23150_e17787_d_n5;
        locals.var_t2_dn6 = assign23150_e17787_d_n6;
        locals.var_t2_dn7 = assign23150_e17787_d_n7;
        locals.var_t2_dn8 = assign23150_e17787_d_n8;
        locals.var_t2_dn9 = assign23150_e17787_d_n9;
        locals.var_t2_dn10 = assign23150_e17787_d_n10;
        locals.var_t2_dn13 = assign23150_e17787_d_n13;
        locals.var_t2_rv = 0.0;

        let (assign23160_e17795, assign23160_e17795_d_n0, assign23160_e17795_d_n2, assign23160_e17795_d_n4, assign23160_e17795_d_n5, assign23160_e17795_d_n6, assign23160_e17795_d_n7, assign23160_e17795_d_n8, assign23160_e17795_d_n9, assign23160_e17795_d_n10, assign23160_e17795_d_n13,) = {
    if (locals.var_guard430 != 0.0) {
        let assign23160_e17792: f64 = (p.p140 * p.p140);
        let assign23160_e17793: f64 = (1.0 / assign23160_e17792);
        (assign23160_e17793, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn13,)
    }
};
        locals.var_t4 = assign23160_e17795;
        locals.var_t4_dn0 = assign23160_e17795_d_n0;
        locals.var_t4_dn2 = assign23160_e17795_d_n2;
        locals.var_t4_dn4 = assign23160_e17795_d_n4;
        locals.var_t4_dn5 = assign23160_e17795_d_n5;
        locals.var_t4_dn6 = assign23160_e17795_d_n6;
        locals.var_t4_dn7 = assign23160_e17795_d_n7;
        locals.var_t4_dn8 = assign23160_e17795_d_n8;
        locals.var_t4_dn9 = assign23160_e17795_d_n9;
        locals.var_t4_dn10 = assign23160_e17795_d_n10;
        locals.var_t4_dn13 = assign23160_e17795_d_n13;
        locals.var_t4_rv = 0.0;

        let (assign23170_e17809, assign23170_e17809_d_n0, assign23170_e17809_d_n2, assign23170_e17809_d_n4, assign23170_e17809_d_n5, assign23170_e17809_d_n6, assign23170_e17809_d_n7, assign23170_e17809_d_n8, assign23170_e17809_d_n9, assign23170_e17809_d_n10, assign23170_e17809_d_n13,) = {
    if (locals.var_guard430 != 0.0) {
        let assign23170_e17800: f64 = (p.p137 - locals.var_pb20b);
        let assign23170_e17801: f64 = (2.0 * assign23170_e17800);
        let assign23170_e17803: f64 = (assign23170_e17801 * locals.var_t1);
        let assign23170_e17805: f64 = (assign23170_e17803 * locals.var_t2);
        let assign23170_e17807: f64 = (assign23170_e17805 * locals.var_t4);
        (assign23170_e17807, (((((((2.0 * (-locals.var_pb20b_dn0)) * locals.var_t1) + (assign23170_e17801 * locals.var_t1_dn0)) * locals.var_t2) + (assign23170_e17803 * locals.var_t2_dn0)) * locals.var_t4) + (assign23170_e17805 * locals.var_t4_dn0)), (((((((2.0 * (-locals.var_pb20b_dn2)) * locals.var_t1) + (assign23170_e17801 * locals.var_t1_dn2)) * locals.var_t2) + (assign23170_e17803 * locals.var_t2_dn2)) * locals.var_t4) + (assign23170_e17805 * locals.var_t4_dn2)), (((((((2.0 * (-locals.var_pb20b_dn4)) * locals.var_t1) + (assign23170_e17801 * locals.var_t1_dn4)) * locals.var_t2) + (assign23170_e17803 * locals.var_t2_dn4)) * locals.var_t4) + (assign23170_e17805 * locals.var_t4_dn4)), (((((((2.0 * (-locals.var_pb20b_dn5)) * locals.var_t1) + (assign23170_e17801 * locals.var_t1_dn5)) * locals.var_t2) + (assign23170_e17803 * locals.var_t2_dn5)) * locals.var_t4) + (assign23170_e17805 * locals.var_t4_dn5)), (((((((2.0 * (-locals.var_pb20b_dn6)) * locals.var_t1) + (assign23170_e17801 * locals.var_t1_dn6)) * locals.var_t2) + (assign23170_e17803 * locals.var_t2_dn6)) * locals.var_t4) + (assign23170_e17805 * locals.var_t4_dn6)), (((((((2.0 * (-locals.var_pb20b_dn7)) * locals.var_t1) + (assign23170_e17801 * locals.var_t1_dn7)) * locals.var_t2) + (assign23170_e17803 * locals.var_t2_dn7)) * locals.var_t4) + (assign23170_e17805 * locals.var_t4_dn7)), (((((((2.0 * (-locals.var_pb20b_dn8)) * locals.var_t1) + (assign23170_e17801 * locals.var_t1_dn8)) * locals.var_t2) + (assign23170_e17803 * locals.var_t2_dn8)) * locals.var_t4) + (assign23170_e17805 * locals.var_t4_dn8)), (((((((2.0 * (-locals.var_pb20b_dn9)) * locals.var_t1) + (assign23170_e17801 * locals.var_t1_dn9)) * locals.var_t2) + (assign23170_e17803 * locals.var_t2_dn9)) * locals.var_t4) + (assign23170_e17805 * locals.var_t4_dn9)), (((((((2.0 * (-locals.var_pb20b_dn10)) * locals.var_t1) + (assign23170_e17801 * locals.var_t1_dn10)) * locals.var_t2) + (assign23170_e17803 * locals.var_t2_dn10)) * locals.var_t4) + (assign23170_e17805 * locals.var_t4_dn10)), (((((((2.0 * (-locals.var_pb20b_dn13)) * locals.var_t1) + (assign23170_e17801 * locals.var_t1_dn13)) * locals.var_t2) + (assign23170_e17803 * locals.var_t2_dn13)) * locals.var_t4) + (assign23170_e17805 * locals.var_t4_dn13)),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn13,)
    }
};
        locals.var_t5 = assign23170_e17809;
        locals.var_t5_dn0 = assign23170_e17809_d_n0;
        locals.var_t5_dn2 = assign23170_e17809_d_n2;
        locals.var_t5_dn4 = assign23170_e17809_d_n4;
        locals.var_t5_dn5 = assign23170_e17809_d_n5;
        locals.var_t5_dn6 = assign23170_e17809_d_n6;
        locals.var_t5_dn7 = assign23170_e17809_d_n7;
        locals.var_t5_dn8 = assign23170_e17809_d_n8;
        locals.var_t5_dn9 = assign23170_e17809_d_n9;
        locals.var_t5_dn10 = assign23170_e17809_d_n10;
        locals.var_t5_dn13 = assign23170_e17809_d_n13;
        locals.var_t5_rv = 0.0;

        let (assign23180_e17815, assign23180_e17815_d_n0, assign23180_e17815_d_n2, assign23180_e17815_d_n4, assign23180_e17815_d_n5, assign23180_e17815_d_n6, assign23180_e17815_d_n7, assign23180_e17815_d_n8, assign23180_e17815_d_n9, assign23180_e17815_d_n10, assign23180_e17815_d_n13,) = {
    if (locals.var_guard430 != 0.0) {
        let assign23180_e17813: f64 = (locals.var_t5 * locals.var_sqrt_pbsum);
        (assign23180_e17813, ((locals.var_t5_dn0 * locals.var_sqrt_pbsum) + (locals.var_t5 * locals.var_sqrt_pbsum_dn0)), ((locals.var_t5_dn2 * locals.var_sqrt_pbsum) + (locals.var_t5 * locals.var_sqrt_pbsum_dn2)), ((locals.var_t5_dn4 * locals.var_sqrt_pbsum) + (locals.var_t5 * locals.var_sqrt_pbsum_dn4)), ((locals.var_t5_dn5 * locals.var_sqrt_pbsum) + (locals.var_t5 * locals.var_sqrt_pbsum_dn5)), ((locals.var_t5_dn6 * locals.var_sqrt_pbsum) + (locals.var_t5 * locals.var_sqrt_pbsum_dn6)), ((locals.var_t5_dn7 * locals.var_sqrt_pbsum) + (locals.var_t5 * locals.var_sqrt_pbsum_dn7)), ((locals.var_t5_dn8 * locals.var_sqrt_pbsum) + (locals.var_t5 * locals.var_sqrt_pbsum_dn8)), ((locals.var_t5_dn9 * locals.var_sqrt_pbsum) + (locals.var_t5 * locals.var_sqrt_pbsum_dn9)), ((locals.var_t5_dn10 * locals.var_sqrt_pbsum) + (locals.var_t5 * locals.var_sqrt_pbsum_dn10)), ((locals.var_t5_dn13 * locals.var_sqrt_pbsum) + (locals.var_t5 * locals.var_sqrt_pbsum_dn13)),)
    } else {
        (locals.var_dvth0, locals.var_dvth0_dn0, locals.var_dvth0_dn2, locals.var_dvth0_dn4, locals.var_dvth0_dn5, locals.var_dvth0_dn6, locals.var_dvth0_dn7, locals.var_dvth0_dn8, locals.var_dvth0_dn9, locals.var_dvth0_dn10, locals.var_dvth0_dn13,)
    }
};
        locals.var_dvth0 = assign23180_e17815;
        locals.var_dvth0_dn0 = assign23180_e17815_d_n0;
        locals.var_dvth0_dn2 = assign23180_e17815_d_n2;
        locals.var_dvth0_dn4 = assign23180_e17815_d_n4;
        locals.var_dvth0_dn5 = assign23180_e17815_d_n5;
        locals.var_dvth0_dn6 = assign23180_e17815_d_n6;
        locals.var_dvth0_dn7 = assign23180_e17815_d_n7;
        locals.var_dvth0_dn8 = assign23180_e17815_d_n8;
        locals.var_dvth0_dn9 = assign23180_e17815_d_n9;
        locals.var_dvth0_dn10 = assign23180_e17815_d_n10;
        locals.var_dvth0_dn13 = assign23180_e17815_d_n13;
        locals.var_dvth0_rv = 0.0;

        let (assign23190_e17823, assign23190_e17823_d_n0, assign23190_e17823_d_n2, assign23190_e17823_d_n4, assign23190_e17823_d_n5, assign23190_e17823_d_n6, assign23190_e17823_d_n7, assign23190_e17823_d_n8, assign23190_e17823_d_n9, assign23190_e17823_d_n10, assign23190_e17823_d_n13,) = {
    if (locals.var_guard430 != 0.0) {
        let assign23190_e17819: f64 = (0.5 * locals.var_t5);
        let assign23190_e17821: f64 = (assign23190_e17819 / locals.var_sqrt_pbsum);
        (assign23190_e17821, ((((0.5 * locals.var_t5_dn0) * locals.var_sqrt_pbsum) - (assign23190_e17819 * locals.var_sqrt_pbsum_dn0)) / (locals.var_sqrt_pbsum * locals.var_sqrt_pbsum)), ((((0.5 * locals.var_t5_dn2) * locals.var_sqrt_pbsum) - (assign23190_e17819 * locals.var_sqrt_pbsum_dn2)) / (locals.var_sqrt_pbsum * locals.var_sqrt_pbsum)), ((((0.5 * locals.var_t5_dn4) * locals.var_sqrt_pbsum) - (assign23190_e17819 * locals.var_sqrt_pbsum_dn4)) / (locals.var_sqrt_pbsum * locals.var_sqrt_pbsum)), ((((0.5 * locals.var_t5_dn5) * locals.var_sqrt_pbsum) - (assign23190_e17819 * locals.var_sqrt_pbsum_dn5)) / (locals.var_sqrt_pbsum * locals.var_sqrt_pbsum)), ((((0.5 * locals.var_t5_dn6) * locals.var_sqrt_pbsum) - (assign23190_e17819 * locals.var_sqrt_pbsum_dn6)) / (locals.var_sqrt_pbsum * locals.var_sqrt_pbsum)), ((((0.5 * locals.var_t5_dn7) * locals.var_sqrt_pbsum) - (assign23190_e17819 * locals.var_sqrt_pbsum_dn7)) / (locals.var_sqrt_pbsum * locals.var_sqrt_pbsum)), ((((0.5 * locals.var_t5_dn8) * locals.var_sqrt_pbsum) - (assign23190_e17819 * locals.var_sqrt_pbsum_dn8)) / (locals.var_sqrt_pbsum * locals.var_sqrt_pbsum)), ((((0.5 * locals.var_t5_dn9) * locals.var_sqrt_pbsum) - (assign23190_e17819 * locals.var_sqrt_pbsum_dn9)) / (locals.var_sqrt_pbsum * locals.var_sqrt_pbsum)), ((((0.5 * locals.var_t5_dn10) * locals.var_sqrt_pbsum) - (assign23190_e17819 * locals.var_sqrt_pbsum_dn10)) / (locals.var_sqrt_pbsum * locals.var_sqrt_pbsum)), ((((0.5 * locals.var_t5_dn13) * locals.var_sqrt_pbsum) - (assign23190_e17819 * locals.var_sqrt_pbsum_dn13)) / (locals.var_sqrt_pbsum * locals.var_sqrt_pbsum)),)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn13,)
    }
};
        locals.var_t6 = assign23190_e17823;
        locals.var_t6_dn0 = assign23190_e17823_d_n0;
        locals.var_t6_dn2 = assign23190_e17823_d_n2;
        locals.var_t6_dn4 = assign23190_e17823_d_n4;
        locals.var_t6_dn5 = assign23190_e17823_d_n5;
        locals.var_t6_dn6 = assign23190_e17823_d_n6;
        locals.var_t6_dn7 = assign23190_e17823_d_n7;
        locals.var_t6_dn8 = assign23190_e17823_d_n8;
        locals.var_t6_dn9 = assign23190_e17823_d_n9;
        locals.var_t6_dn10 = assign23190_e17823_d_n10;
        locals.var_t6_dn13 = assign23190_e17823_d_n13;
        locals.var_t6_rv = 0.0;

        let (assign23200_e17839, assign23200_e17839_d_n0, assign23200_e17839_d_n2, assign23200_e17839_d_n4, assign23200_e17839_d_n5, assign23200_e17839_d_n6, assign23200_e17839_d_n7, assign23200_e17839_d_n8, assign23200_e17839_d_n9, assign23200_e17839_d_n10, assign23200_e17839_d_n13,) = {
    if (locals.var_guard430 != 0.0) {
        let assign23200_e17828: f64 = (p.p137 - locals.var_pb20b);
        let assign23200_e17829: f64 = (2.0 * assign23200_e17828);
        let assign23200_e17831: f64 = (assign23200_e17829 * 1.034943e-10);
        let assign23200_e17833: f64 = (assign23200_e17831 * locals.var_t2);
        let assign23200_e17835: f64 = (assign23200_e17833 * locals.var_t4);
        let assign23200_e17837: f64 = (assign23200_e17835 * locals.var_sqrt_pbsum);
        (assign23200_e17837, ((((((((2.0 * (-locals.var_pb20b_dn0)) * 1.034943e-10) * locals.var_t2) + (assign23200_e17831 * locals.var_t2_dn0)) * locals.var_t4) + (assign23200_e17833 * locals.var_t4_dn0)) * locals.var_sqrt_pbsum) + (assign23200_e17835 * locals.var_sqrt_pbsum_dn0)), ((((((((2.0 * (-locals.var_pb20b_dn2)) * 1.034943e-10) * locals.var_t2) + (assign23200_e17831 * locals.var_t2_dn2)) * locals.var_t4) + (assign23200_e17833 * locals.var_t4_dn2)) * locals.var_sqrt_pbsum) + (assign23200_e17835 * locals.var_sqrt_pbsum_dn2)), ((((((((2.0 * (-locals.var_pb20b_dn4)) * 1.034943e-10) * locals.var_t2) + (assign23200_e17831 * locals.var_t2_dn4)) * locals.var_t4) + (assign23200_e17833 * locals.var_t4_dn4)) * locals.var_sqrt_pbsum) + (assign23200_e17835 * locals.var_sqrt_pbsum_dn4)), ((((((((2.0 * (-locals.var_pb20b_dn5)) * 1.034943e-10) * locals.var_t2) + (assign23200_e17831 * locals.var_t2_dn5)) * locals.var_t4) + (assign23200_e17833 * locals.var_t4_dn5)) * locals.var_sqrt_pbsum) + (assign23200_e17835 * locals.var_sqrt_pbsum_dn5)), ((((((((2.0 * (-locals.var_pb20b_dn6)) * 1.034943e-10) * locals.var_t2) + (assign23200_e17831 * locals.var_t2_dn6)) * locals.var_t4) + (assign23200_e17833 * locals.var_t4_dn6)) * locals.var_sqrt_pbsum) + (assign23200_e17835 * locals.var_sqrt_pbsum_dn6)), ((((((((2.0 * (-locals.var_pb20b_dn7)) * 1.034943e-10) * locals.var_t2) + (assign23200_e17831 * locals.var_t2_dn7)) * locals.var_t4) + (assign23200_e17833 * locals.var_t4_dn7)) * locals.var_sqrt_pbsum) + (assign23200_e17835 * locals.var_sqrt_pbsum_dn7)), ((((((((2.0 * (-locals.var_pb20b_dn8)) * 1.034943e-10) * locals.var_t2) + (assign23200_e17831 * locals.var_t2_dn8)) * locals.var_t4) + (assign23200_e17833 * locals.var_t4_dn8)) * locals.var_sqrt_pbsum) + (assign23200_e17835 * locals.var_sqrt_pbsum_dn8)), ((((((((2.0 * (-locals.var_pb20b_dn9)) * 1.034943e-10) * locals.var_t2) + (assign23200_e17831 * locals.var_t2_dn9)) * locals.var_t4) + (assign23200_e17833 * locals.var_t4_dn9)) * locals.var_sqrt_pbsum) + (assign23200_e17835 * locals.var_sqrt_pbsum_dn9)), ((((((((2.0 * (-locals.var_pb20b_dn10)) * 1.034943e-10) * locals.var_t2) + (assign23200_e17831 * locals.var_t2_dn10)) * locals.var_t4) + (assign23200_e17833 * locals.var_t4_dn10)) * locals.var_sqrt_pbsum) + (assign23200_e17835 * locals.var_sqrt_pbsum_dn10)), ((((((((2.0 * (-locals.var_pb20b_dn13)) * 1.034943e-10) * locals.var_t2) + (assign23200_e17831 * locals.var_t2_dn13)) * locals.var_t4) + (assign23200_e17833 * locals.var_t4_dn13)) * locals.var_sqrt_pbsum) + (assign23200_e17835 * locals.var_sqrt_pbsum_dn13)),)
    } else {
        (locals.var_t7, locals.var_t7_dn0, locals.var_t7_dn2, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn13,)
    }
};
        locals.var_t7 = assign23200_e17839;
        locals.var_t7_dn0 = assign23200_e17839_d_n0;
        locals.var_t7_dn2 = assign23200_e17839_d_n2;
        locals.var_t7_dn4 = assign23200_e17839_d_n4;
        locals.var_t7_dn5 = assign23200_e17839_d_n5;
        locals.var_t7_dn6 = assign23200_e17839_d_n6;
        locals.var_t7_dn7 = assign23200_e17839_d_n7;
        locals.var_t7_dn8 = assign23200_e17839_d_n8;
        locals.var_t7_dn9 = assign23200_e17839_d_n9;
        locals.var_t7_dn10 = assign23200_e17839_d_n10;
        locals.var_t7_dn13 = assign23200_e17839_d_n13;
        locals.var_t7_rv = 0.0;

        let (assign23210_e17852, assign23210_e17852_d_n0, assign23210_e17852_d_n2, assign23210_e17852_d_n4, assign23210_e17852_d_n5, assign23210_e17852_d_n6, assign23210_e17852_d_n7, assign23210_e17852_d_n8, assign23210_e17852_d_n9, assign23210_e17852_d_n10, assign23210_e17852_d_n13,) = {
    if (locals.var_guard430 != 0.0) {
        let assign23210_e17842: f64 = (-2.0);
        let assign23210_e17844: f64 = (assign23210_e17842 * locals.var_t1);
        let assign23210_e17846: f64 = (assign23210_e17844 * locals.var_t2);
        let assign23210_e17848: f64 = (assign23210_e17846 * locals.var_t4);
        let assign23210_e17850: f64 = (assign23210_e17848 * locals.var_sqrt_pbsum);
        (assign23210_e17850, (((((((assign23210_e17842 * locals.var_t1_dn0) * locals.var_t2) + (assign23210_e17844 * locals.var_t2_dn0)) * locals.var_t4) + (assign23210_e17846 * locals.var_t4_dn0)) * locals.var_sqrt_pbsum) + (assign23210_e17848 * locals.var_sqrt_pbsum_dn0)), (((((((assign23210_e17842 * locals.var_t1_dn2) * locals.var_t2) + (assign23210_e17844 * locals.var_t2_dn2)) * locals.var_t4) + (assign23210_e17846 * locals.var_t4_dn2)) * locals.var_sqrt_pbsum) + (assign23210_e17848 * locals.var_sqrt_pbsum_dn2)), (((((((assign23210_e17842 * locals.var_t1_dn4) * locals.var_t2) + (assign23210_e17844 * locals.var_t2_dn4)) * locals.var_t4) + (assign23210_e17846 * locals.var_t4_dn4)) * locals.var_sqrt_pbsum) + (assign23210_e17848 * locals.var_sqrt_pbsum_dn4)), (((((((assign23210_e17842 * locals.var_t1_dn5) * locals.var_t2) + (assign23210_e17844 * locals.var_t2_dn5)) * locals.var_t4) + (assign23210_e17846 * locals.var_t4_dn5)) * locals.var_sqrt_pbsum) + (assign23210_e17848 * locals.var_sqrt_pbsum_dn5)), (((((((assign23210_e17842 * locals.var_t1_dn6) * locals.var_t2) + (assign23210_e17844 * locals.var_t2_dn6)) * locals.var_t4) + (assign23210_e17846 * locals.var_t4_dn6)) * locals.var_sqrt_pbsum) + (assign23210_e17848 * locals.var_sqrt_pbsum_dn6)), (((((((assign23210_e17842 * locals.var_t1_dn7) * locals.var_t2) + (assign23210_e17844 * locals.var_t2_dn7)) * locals.var_t4) + (assign23210_e17846 * locals.var_t4_dn7)) * locals.var_sqrt_pbsum) + (assign23210_e17848 * locals.var_sqrt_pbsum_dn7)), (((((((assign23210_e17842 * locals.var_t1_dn8) * locals.var_t2) + (assign23210_e17844 * locals.var_t2_dn8)) * locals.var_t4) + (assign23210_e17846 * locals.var_t4_dn8)) * locals.var_sqrt_pbsum) + (assign23210_e17848 * locals.var_sqrt_pbsum_dn8)), (((((((assign23210_e17842 * locals.var_t1_dn9) * locals.var_t2) + (assign23210_e17844 * locals.var_t2_dn9)) * locals.var_t4) + (assign23210_e17846 * locals.var_t4_dn9)) * locals.var_sqrt_pbsum) + (assign23210_e17848 * locals.var_sqrt_pbsum_dn9)), (((((((assign23210_e17842 * locals.var_t1_dn10) * locals.var_t2) + (assign23210_e17844 * locals.var_t2_dn10)) * locals.var_t4) + (assign23210_e17846 * locals.var_t4_dn10)) * locals.var_sqrt_pbsum) + (assign23210_e17848 * locals.var_sqrt_pbsum_dn10)), (((((((assign23210_e17842 * locals.var_t1_dn13) * locals.var_t2) + (assign23210_e17844 * locals.var_t2_dn13)) * locals.var_t4) + (assign23210_e17846 * locals.var_t4_dn13)) * locals.var_sqrt_pbsum) + (assign23210_e17848 * locals.var_sqrt_pbsum_dn13)),)
    } else {
        (locals.var_t8, locals.var_t8_dn0, locals.var_t8_dn2, locals.var_t8_dn4, locals.var_t8_dn5, locals.var_t8_dn6, locals.var_t8_dn7, locals.var_t8_dn8, locals.var_t8_dn9, locals.var_t8_dn10, locals.var_t8_dn13,)
    }
};
        locals.var_t8 = assign23210_e17852;
        locals.var_t8_dn0 = assign23210_e17852_d_n0;
        locals.var_t8_dn2 = assign23210_e17852_d_n2;
        locals.var_t8_dn4 = assign23210_e17852_d_n4;
        locals.var_t8_dn5 = assign23210_e17852_d_n5;
        locals.var_t8_dn6 = assign23210_e17852_d_n6;
        locals.var_t8_dn7 = assign23210_e17852_d_n7;
        locals.var_t8_dn8 = assign23210_e17852_d_n8;
        locals.var_t8_dn9 = assign23210_e17852_d_n9;
        locals.var_t8_dn10 = assign23210_e17852_d_n10;
        locals.var_t8_dn13 = assign23210_e17852_d_n13;
        locals.var_t8_rv = 0.0;

        let (assign23220_e17858, assign23220_e17858_d_n0, assign23220_e17858_d_n2, assign23220_e17858_d_n4, assign23220_e17858_d_n5, assign23220_e17858_d_n6, assign23220_e17858_d_n7, assign23220_e17858_d_n8, assign23220_e17858_d_n9, assign23220_e17858_d_n10, assign23220_e17858_d_n13,) = {
    if (locals.var_guard430 != 0.0) {
        let assign23220_e17856: f64 = (locals.var_vthp - locals.var_vth0);
        (assign23220_e17856, (locals.var_vthp_dn0 - locals.var_vth0_dn0), (locals.var_vthp_dn2 - locals.var_vth0_dn2), (locals.var_vthp_dn4 - locals.var_vth0_dn4), (locals.var_vthp_dn5 - locals.var_vth0_dn5), (locals.var_vthp_dn6 - locals.var_vth0_dn6), (locals.var_vthp_dn7 - locals.var_vth0_dn7), (locals.var_vthp_dn8 - locals.var_vth0_dn8), (locals.var_vthp_dn9 - locals.var_vth0_dn9), (locals.var_vthp_dn10 - locals.var_vth0_dn10), (locals.var_vthp_dn13 - locals.var_vth0_dn13),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign23220_e17858;
        locals.var_t1_dn0 = assign23220_e17858_d_n0;
        locals.var_t1_dn2 = assign23220_e17858_d_n2;
        locals.var_t1_dn4 = assign23220_e17858_d_n4;
        locals.var_t1_dn5 = assign23220_e17858_d_n5;
        locals.var_t1_dn6 = assign23220_e17858_d_n6;
        locals.var_t1_dn7 = assign23220_e17858_d_n7;
        locals.var_t1_dn8 = assign23220_e17858_d_n8;
        locals.var_t1_dn9 = assign23220_e17858_d_n9;
        locals.var_t1_dn10 = assign23220_e17858_d_n10;
        locals.var_t1_dn13 = assign23220_e17858_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign23230_e17868, assign23230_e17868_d_n0, assign23230_e17868_d_n2, assign23230_e17868_d_n4, assign23230_e17868_d_n5, assign23230_e17868_d_n6, assign23230_e17868_d_n7, assign23230_e17868_d_n8, assign23230_e17868_d_n9, assign23230_e17868_d_n10, assign23230_e17868_d_n13,) = {
    if (locals.var_guard430 != 0.0) {
        let assign23230_e17863: f64 = (locals.var_uc_scp3 * locals.var_pbsum);
        let assign23230_e17865: f64 = (assign23230_e17863 / p.p140);
        let assign23230_e17866: f64 = (locals.var_uc_scp1 + assign23230_e17865);
        (assign23230_e17866, ((locals.var_uc_scp3 * locals.var_pbsum_dn0) / p.p140), ((locals.var_uc_scp3 * locals.var_pbsum_dn2) / p.p140), ((locals.var_uc_scp3 * locals.var_pbsum_dn4) / p.p140), ((locals.var_uc_scp3 * locals.var_pbsum_dn5) / p.p140), ((locals.var_uc_scp3 * locals.var_pbsum_dn6) / p.p140), ((locals.var_uc_scp3 * locals.var_pbsum_dn7) / p.p140), ((locals.var_uc_scp3 * locals.var_pbsum_dn8) / p.p140), ((locals.var_uc_scp3 * locals.var_pbsum_dn9) / p.p140), ((locals.var_uc_scp3 * locals.var_pbsum_dn10) / p.p140), ((locals.var_uc_scp3 * locals.var_pbsum_dn13) / p.p140),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign23230_e17868;
        locals.var_t2_dn0 = assign23230_e17868_d_n0;
        locals.var_t2_dn2 = assign23230_e17868_d_n2;
        locals.var_t2_dn4 = assign23230_e17868_d_n4;
        locals.var_t2_dn5 = assign23230_e17868_d_n5;
        locals.var_t2_dn6 = assign23230_e17868_d_n6;
        locals.var_t2_dn7 = assign23230_e17868_d_n7;
        locals.var_t2_dn8 = assign23230_e17868_d_n8;
        locals.var_t2_dn9 = assign23230_e17868_d_n9;
        locals.var_t2_dn10 = assign23230_e17868_d_n10;
        locals.var_t2_dn13 = assign23230_e17868_d_n13;
        locals.var_t2_rv = 0.0;

        let (assign23240_e17876, assign23240_e17876_d_n0, assign23240_e17876_d_n2, assign23240_e17876_d_n4, assign23240_e17876_d_n5, assign23240_e17876_d_n6, assign23240_e17876_d_n7, assign23240_e17876_d_n8, assign23240_e17876_d_n9, assign23240_e17876_d_n10, assign23240_e17876_d_n13,) = {
    if (locals.var_guard430 != 0.0) {
        let assign23240_e17873: f64 = (locals.var_uc_scp2 * locals.var_vdsz);
        let assign23240_e17874: f64 = (locals.var_t2 + assign23240_e17873);
        (assign23240_e17874, (locals.var_t2_dn0 + (locals.var_uc_scp2 * locals.var_vdsz_dn0)), (locals.var_t2_dn2 + (locals.var_uc_scp2 * locals.var_vdsz_dn2)), (locals.var_t2_dn4 + (locals.var_uc_scp2 * locals.var_vdsz_dn4)), (locals.var_t2_dn5 + (locals.var_uc_scp2 * locals.var_vdsz_dn5)), (locals.var_t2_dn6 + (locals.var_uc_scp2 * locals.var_vdsz_dn6)), (locals.var_t2_dn7 + (locals.var_uc_scp2 * locals.var_vdsz_dn7)), (locals.var_t2_dn8 + (locals.var_uc_scp2 * locals.var_vdsz_dn8)), (locals.var_t2_dn9 + (locals.var_uc_scp2 * locals.var_vdsz_dn9)), (locals.var_t2_dn10 + (locals.var_uc_scp2 * locals.var_vdsz_dn10)), (locals.var_t2_dn13 + (locals.var_uc_scp2 * locals.var_vdsz_dn13)),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn13,)
    }
};
        locals.var_t3 = assign23240_e17876;
        locals.var_t3_dn0 = assign23240_e17876_d_n0;
        locals.var_t3_dn2 = assign23240_e17876_d_n2;
        locals.var_t3_dn4 = assign23240_e17876_d_n4;
        locals.var_t3_dn5 = assign23240_e17876_d_n5;
        locals.var_t3_dn6 = assign23240_e17876_d_n6;
        locals.var_t3_dn7 = assign23240_e17876_d_n7;
        locals.var_t3_dn8 = assign23240_e17876_d_n8;
        locals.var_t3_dn9 = assign23240_e17876_d_n9;
        locals.var_t3_dn10 = assign23240_e17876_d_n10;
        locals.var_t3_dn13 = assign23240_e17876_d_n13;
        locals.var_t3_rv = 0.0;

        let (assign23250_e17882, assign23250_e17882_d_n0, assign23250_e17882_d_n2, assign23250_e17882_d_n4, assign23250_e17882_d_n5, assign23250_e17882_d_n6, assign23250_e17882_d_n7, assign23250_e17882_d_n8, assign23250_e17882_d_n9, assign23250_e17882_d_n10, assign23250_e17882_d_n13,) = {
    if (locals.var_guard430 != 0.0) {
        let assign23250_e17880: f64 = (p.p221 + locals.var_vdsz);
        (assign23250_e17880, locals.var_vdsz_dn0, locals.var_vdsz_dn2, locals.var_vdsz_dn4, locals.var_vdsz_dn5, locals.var_vdsz_dn6, locals.var_vdsz_dn7, locals.var_vdsz_dn8, locals.var_vdsz_dn9, locals.var_vdsz_dn10, locals.var_vdsz_dn13,)
    } else {
        (locals.var_vdx, locals.var_vdx_dn0, locals.var_vdx_dn2, locals.var_vdx_dn4, locals.var_vdx_dn5, locals.var_vdx_dn6, locals.var_vdx_dn7, locals.var_vdx_dn8, locals.var_vdx_dn9, locals.var_vdx_dn10, locals.var_vdx_dn13,)
    }
};
        locals.var_vdx = assign23250_e17882;
        locals.var_vdx_dn0 = assign23250_e17882_d_n0;
        locals.var_vdx_dn2 = assign23250_e17882_d_n2;
        locals.var_vdx_dn4 = assign23250_e17882_d_n4;
        locals.var_vdx_dn5 = assign23250_e17882_d_n5;
        locals.var_vdx_dn6 = assign23250_e17882_d_n6;
        locals.var_vdx_dn7 = assign23250_e17882_d_n7;
        locals.var_vdx_dn8 = assign23250_e17882_d_n8;
        locals.var_vdx_dn9 = assign23250_e17882_d_n9;
        locals.var_vdx_dn10 = assign23250_e17882_d_n10;
        locals.var_vdx_dn13 = assign23250_e17882_d_n13;
        locals.var_vdx_rv = 0.0;

        let (assign23260_e17888, assign23260_e17888_d_n0, assign23260_e17888_d_n2, assign23260_e17888_d_n4, assign23260_e17888_d_n5, assign23260_e17888_d_n6, assign23260_e17888_d_n7, assign23260_e17888_d_n8, assign23260_e17888_d_n9, assign23260_e17888_d_n10, assign23260_e17888_d_n13,) = {
    if (locals.var_guard430 != 0.0) {
        let assign23260_e17886: f64 = (locals.var_vdx * locals.var_vdx);
        (assign23260_e17886, ((locals.var_vdx_dn0 * locals.var_vdx) + (locals.var_vdx * locals.var_vdx_dn0)), ((locals.var_vdx_dn2 * locals.var_vdx) + (locals.var_vdx * locals.var_vdx_dn2)), ((locals.var_vdx_dn4 * locals.var_vdx) + (locals.var_vdx * locals.var_vdx_dn4)), ((locals.var_vdx_dn5 * locals.var_vdx) + (locals.var_vdx * locals.var_vdx_dn5)), ((locals.var_vdx_dn6 * locals.var_vdx) + (locals.var_vdx * locals.var_vdx_dn6)), ((locals.var_vdx_dn7 * locals.var_vdx) + (locals.var_vdx * locals.var_vdx_dn7)), ((locals.var_vdx_dn8 * locals.var_vdx) + (locals.var_vdx * locals.var_vdx_dn8)), ((locals.var_vdx_dn9 * locals.var_vdx) + (locals.var_vdx * locals.var_vdx_dn9)), ((locals.var_vdx_dn10 * locals.var_vdx) + (locals.var_vdx * locals.var_vdx_dn10)), ((locals.var_vdx_dn13 * locals.var_vdx) + (locals.var_vdx * locals.var_vdx_dn13)),)
    } else {
        (locals.var_vdx2, locals.var_vdx2_dn0, locals.var_vdx2_dn2, locals.var_vdx2_dn4, locals.var_vdx2_dn5, locals.var_vdx2_dn6, locals.var_vdx2_dn7, locals.var_vdx2_dn8, locals.var_vdx2_dn9, locals.var_vdx2_dn10, locals.var_vdx2_dn13,)
    }
};
        locals.var_vdx2 = assign23260_e17888;
        locals.var_vdx2_dn0 = assign23260_e17888_d_n0;
        locals.var_vdx2_dn2 = assign23260_e17888_d_n2;
        locals.var_vdx2_dn4 = assign23260_e17888_d_n4;
        locals.var_vdx2_dn5 = assign23260_e17888_d_n5;
        locals.var_vdx2_dn6 = assign23260_e17888_d_n6;
        locals.var_vdx2_dn7 = assign23260_e17888_d_n7;
        locals.var_vdx2_dn8 = assign23260_e17888_d_n8;
        locals.var_vdx2_dn9 = assign23260_e17888_d_n9;
        locals.var_vdx2_dn10 = assign23260_e17888_d_n10;
        locals.var_vdx2_dn13 = assign23260_e17888_d_n13;
        locals.var_vdx2_rv = 0.0;

        let (assign23270_e17902, assign23270_e17902_d_n0, assign23270_e17902_d_n2, assign23270_e17902_d_n4, assign23270_e17902_d_n5, assign23270_e17902_d_n6, assign23270_e17902_d_n7, assign23270_e17902_d_n8, assign23270_e17902_d_n9, assign23270_e17902_d_n10, assign23270_e17902_d_n13,) = {
    if (locals.var_guard430 != 0.0) {
        let assign23270_e17892: f64 = (locals.var_t1 * locals.var_dvth0);
        let assign23270_e17894: f64 = (assign23270_e17892 * locals.var_t3);
        let assign23270_e17896: f64 = (assign23270_e17894 + locals.var_dqb);
        let assign23270_e17899: f64 = (locals.var_msc / locals.var_vdx2);
        let assign23270_e17900: f64 = (assign23270_e17896 - assign23270_e17899);
        (assign23270_e17900, ((((((locals.var_t1_dn0 * locals.var_dvth0) + (locals.var_t1 * locals.var_dvth0_dn0)) * locals.var_t3) + (assign23270_e17892 * locals.var_t3_dn0)) + locals.var_dqb_dn0) - (-((locals.var_msc * locals.var_vdx2_dn0) / (locals.var_vdx2 * locals.var_vdx2)))), ((((((locals.var_t1_dn2 * locals.var_dvth0) + (locals.var_t1 * locals.var_dvth0_dn2)) * locals.var_t3) + (assign23270_e17892 * locals.var_t3_dn2)) + locals.var_dqb_dn2) - (-((locals.var_msc * locals.var_vdx2_dn2) / (locals.var_vdx2 * locals.var_vdx2)))), ((((((locals.var_t1_dn4 * locals.var_dvth0) + (locals.var_t1 * locals.var_dvth0_dn4)) * locals.var_t3) + (assign23270_e17892 * locals.var_t3_dn4)) + locals.var_dqb_dn4) - (-((locals.var_msc * locals.var_vdx2_dn4) / (locals.var_vdx2 * locals.var_vdx2)))), ((((((locals.var_t1_dn5 * locals.var_dvth0) + (locals.var_t1 * locals.var_dvth0_dn5)) * locals.var_t3) + (assign23270_e17892 * locals.var_t3_dn5)) + locals.var_dqb_dn5) - (-((locals.var_msc * locals.var_vdx2_dn5) / (locals.var_vdx2 * locals.var_vdx2)))), ((((((locals.var_t1_dn6 * locals.var_dvth0) + (locals.var_t1 * locals.var_dvth0_dn6)) * locals.var_t3) + (assign23270_e17892 * locals.var_t3_dn6)) + locals.var_dqb_dn6) - (-((locals.var_msc * locals.var_vdx2_dn6) / (locals.var_vdx2 * locals.var_vdx2)))), ((((((locals.var_t1_dn7 * locals.var_dvth0) + (locals.var_t1 * locals.var_dvth0_dn7)) * locals.var_t3) + (assign23270_e17892 * locals.var_t3_dn7)) + locals.var_dqb_dn7) - (-((locals.var_msc * locals.var_vdx2_dn7) / (locals.var_vdx2 * locals.var_vdx2)))), ((((((locals.var_t1_dn8 * locals.var_dvth0) + (locals.var_t1 * locals.var_dvth0_dn8)) * locals.var_t3) + (assign23270_e17892 * locals.var_t3_dn8)) + locals.var_dqb_dn8) - (-((locals.var_msc * locals.var_vdx2_dn8) / (locals.var_vdx2 * locals.var_vdx2)))), ((((((locals.var_t1_dn9 * locals.var_dvth0) + (locals.var_t1 * locals.var_dvth0_dn9)) * locals.var_t3) + (assign23270_e17892 * locals.var_t3_dn9)) + locals.var_dqb_dn9) - (-((locals.var_msc * locals.var_vdx2_dn9) / (locals.var_vdx2 * locals.var_vdx2)))), ((((((locals.var_t1_dn10 * locals.var_dvth0) + (locals.var_t1 * locals.var_dvth0_dn10)) * locals.var_t3) + (assign23270_e17892 * locals.var_t3_dn10)) + locals.var_dqb_dn10) - (-((locals.var_msc * locals.var_vdx2_dn10) / (locals.var_vdx2 * locals.var_vdx2)))), ((((((locals.var_t1_dn13 * locals.var_dvth0) + (locals.var_t1 * locals.var_dvth0_dn13)) * locals.var_t3) + (assign23270_e17892 * locals.var_t3_dn13)) + locals.var_dqb_dn13) - (-((locals.var_msc * locals.var_vdx2_dn13) / (locals.var_vdx2 * locals.var_vdx2)))),)
    } else {
        (locals.var_dvthlp, locals.var_dvthlp_dn0, locals.var_dvthlp_dn2, locals.var_dvthlp_dn4, locals.var_dvthlp_dn5, locals.var_dvthlp_dn6, locals.var_dvthlp_dn7, locals.var_dvthlp_dn8, locals.var_dvthlp_dn9, locals.var_dvthlp_dn10, locals.var_dvthlp_dn13,)
    }
};
        locals.var_dvthlp = assign23270_e17902;
        locals.var_dvthlp_dn0 = assign23270_e17902_d_n0;
        locals.var_dvthlp_dn2 = assign23270_e17902_d_n2;
        locals.var_dvthlp_dn4 = assign23270_e17902_d_n4;
        locals.var_dvthlp_dn5 = assign23270_e17902_d_n5;
        locals.var_dvthlp_dn6 = assign23270_e17902_d_n6;
        locals.var_dvthlp_dn7 = assign23270_e17902_d_n7;
        locals.var_dvthlp_dn8 = assign23270_e17902_d_n8;
        locals.var_dvthlp_dn9 = assign23270_e17902_d_n9;
        locals.var_dvthlp_dn10 = assign23270_e17902_d_n10;
        locals.var_dvthlp_dn13 = assign23270_e17902_d_n13;
        locals.var_dvthlp_rv = 0.0;

        let (assign23280_e17907, assign23280_e17907_d_n0, assign23280_e17907_d_n2, assign23280_e17907_d_n4, assign23280_e17907_d_n5, assign23280_e17907_d_n6, assign23280_e17907_d_n7, assign23280_e17907_d_n8, assign23280_e17907_d_n9, assign23280_e17907_d_n10, assign23280_e17907_d_n13,) = {
    if (locals.var_guard430 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dvthlp, locals.var_dvthlp_dn0, locals.var_dvthlp_dn2, locals.var_dvthlp_dn4, locals.var_dvthlp_dn5, locals.var_dvthlp_dn6, locals.var_dvthlp_dn7, locals.var_dvthlp_dn8, locals.var_dvthlp_dn9, locals.var_dvthlp_dn10, locals.var_dvthlp_dn13,)
    }
};
        locals.var_dvthlp = assign23280_e17907;
        locals.var_dvthlp_dn0 = assign23280_e17907_d_n0;
        locals.var_dvthlp_dn2 = assign23280_e17907_d_n2;
        locals.var_dvthlp_dn4 = assign23280_e17907_d_n4;
        locals.var_dvthlp_dn5 = assign23280_e17907_d_n5;
        locals.var_dvthlp_dn6 = assign23280_e17907_d_n6;
        locals.var_dvthlp_dn7 = assign23280_e17907_d_n7;
        locals.var_dvthlp_dn8 = assign23280_e17907_d_n8;
        locals.var_dvthlp_dn9 = assign23280_e17907_d_n9;
        locals.var_dvthlp_dn10 = assign23280_e17907_d_n10;
        locals.var_dvthlp_dn13 = assign23280_e17907_d_n13;
        locals.var_dvthlp_rv = 0.0;

        let assign23290_e17910: f64 = (1.034943e-10 * locals.var_cox_inv);
        locals.var_t1 = assign23290_e17910;
        locals.var_t1_dn0 = (1.034943e-10 * locals.var_cox_inv_dn0);
        locals.var_t1_dn2 = (1.034943e-10 * locals.var_cox_inv_dn2);
        locals.var_t1_dn4 = (1.034943e-10 * locals.var_cox_inv_dn4);
        locals.var_t1_dn5 = (1.034943e-10 * locals.var_cox_inv_dn5);
        locals.var_t1_dn6 = (1.034943e-10 * locals.var_cox_inv_dn6);
        locals.var_t1_dn7 = (1.034943e-10 * locals.var_cox_inv_dn7);
        locals.var_t1_dn8 = (1.034943e-10 * locals.var_cox_inv_dn8);
        locals.var_t1_dn9 = (1.034943e-10 * locals.var_cox_inv_dn9);
        locals.var_t1_dn10 = (1.034943e-10 * locals.var_cox_inv_dn10);
        locals.var_t1_dn13 = (1.034943e-10 * locals.var_cox_inv_dn13);
        locals.var_t1_rv = 0.0;

        locals.var_t2 = locals.var_wdpl;
        locals.var_t2_dn0 = locals.var_wdpl_dn0;
        locals.var_t2_dn2 = locals.var_wdpl_dn2;
        locals.var_t2_dn4 = locals.var_wdpl_dn4;
        locals.var_t2_dn5 = locals.var_wdpl_dn5;
        locals.var_t2_dn6 = locals.var_wdpl_dn6;
        locals.var_t2_dn7 = locals.var_wdpl_dn7;
        locals.var_t2_dn8 = locals.var_wdpl_dn8;
        locals.var_t2_dn9 = locals.var_wdpl_dn9;
        locals.var_t2_dn10 = locals.var_wdpl_dn10;
        locals.var_t2_dn13 = locals.var_wdpl_dn13;
        locals.var_t2_rv = 0.0;

        let assign23310_e17914: f64 = (locals.var_lgate - p.p139);
        locals.var_t3 = assign23310_e17914;
        locals.var_t3_dn0 = 0.0;
        locals.var_t3_dn2 = 0.0;
        locals.var_t3_dn4 = 0.0;
        locals.var_t3_dn5 = 0.0;
        locals.var_t3_dn6 = 0.0;
        locals.var_t3_dn7 = 0.0;
        locals.var_t3_dn8 = 0.0;
        locals.var_t3_dn9 = 0.0;
        locals.var_t3_dn10 = 0.0;
        locals.var_t3_dn13 = 0.0;
        locals.var_t3_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_61(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let assign23320_e17918: f64 = (locals.var_t3 * locals.var_t3);
        let assign23320_e17919: f64 = (1.0 / assign23320_e17918);
        locals.var_t4 = assign23320_e17919;
        locals.var_t4_dn0 = (-(((locals.var_t3_dn0 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn0)) / (assign23320_e17918 * assign23320_e17918)));
        locals.var_t4_dn2 = (-(((locals.var_t3_dn2 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn2)) / (assign23320_e17918 * assign23320_e17918)));
        locals.var_t4_dn4 = (-(((locals.var_t3_dn4 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn4)) / (assign23320_e17918 * assign23320_e17918)));
        locals.var_t4_dn5 = (-(((locals.var_t3_dn5 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn5)) / (assign23320_e17918 * assign23320_e17918)));
        locals.var_t4_dn6 = (-(((locals.var_t3_dn6 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn6)) / (assign23320_e17918 * assign23320_e17918)));
        locals.var_t4_dn7 = (-(((locals.var_t3_dn7 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn7)) / (assign23320_e17918 * assign23320_e17918)));
        locals.var_t4_dn8 = (-(((locals.var_t3_dn8 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn8)) / (assign23320_e17918 * assign23320_e17918)));
        locals.var_t4_dn9 = (-(((locals.var_t3_dn9 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn9)) / (assign23320_e17918 * assign23320_e17918)));
        locals.var_t4_dn10 = (-(((locals.var_t3_dn10 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn10)) / (assign23320_e17918 * assign23320_e17918)));
        locals.var_t4_dn13 = (-(((locals.var_t3_dn13 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn13)) / (assign23320_e17918 * assign23320_e17918)));
        locals.var_t4_rv = 0.0;

        let assign23330_e17923: f64 = (p.p137 - locals.var_pb20b);
        let assign23330_e17924: f64 = (2.0 * assign23330_e17923);
        let assign23330_e17926: f64 = (assign23330_e17924 * locals.var_t1);
        let assign23330_e17928: f64 = (assign23330_e17926 * locals.var_t2);
        let assign23330_e17930: f64 = (assign23330_e17928 * locals.var_t4);
        locals.var_t5 = assign23330_e17930;
        locals.var_t5_dn0 = (((((((2.0 * (-locals.var_pb20b_dn0)) * locals.var_t1) + (assign23330_e17924 * locals.var_t1_dn0)) * locals.var_t2) + (assign23330_e17926 * locals.var_t2_dn0)) * locals.var_t4) + (assign23330_e17928 * locals.var_t4_dn0));
        locals.var_t5_dn2 = (((((((2.0 * (-locals.var_pb20b_dn2)) * locals.var_t1) + (assign23330_e17924 * locals.var_t1_dn2)) * locals.var_t2) + (assign23330_e17926 * locals.var_t2_dn2)) * locals.var_t4) + (assign23330_e17928 * locals.var_t4_dn2));
        locals.var_t5_dn4 = (((((((2.0 * (-locals.var_pb20b_dn4)) * locals.var_t1) + (assign23330_e17924 * locals.var_t1_dn4)) * locals.var_t2) + (assign23330_e17926 * locals.var_t2_dn4)) * locals.var_t4) + (assign23330_e17928 * locals.var_t4_dn4));
        locals.var_t5_dn5 = (((((((2.0 * (-locals.var_pb20b_dn5)) * locals.var_t1) + (assign23330_e17924 * locals.var_t1_dn5)) * locals.var_t2) + (assign23330_e17926 * locals.var_t2_dn5)) * locals.var_t4) + (assign23330_e17928 * locals.var_t4_dn5));
        locals.var_t5_dn6 = (((((((2.0 * (-locals.var_pb20b_dn6)) * locals.var_t1) + (assign23330_e17924 * locals.var_t1_dn6)) * locals.var_t2) + (assign23330_e17926 * locals.var_t2_dn6)) * locals.var_t4) + (assign23330_e17928 * locals.var_t4_dn6));
        locals.var_t5_dn7 = (((((((2.0 * (-locals.var_pb20b_dn7)) * locals.var_t1) + (assign23330_e17924 * locals.var_t1_dn7)) * locals.var_t2) + (assign23330_e17926 * locals.var_t2_dn7)) * locals.var_t4) + (assign23330_e17928 * locals.var_t4_dn7));
        locals.var_t5_dn8 = (((((((2.0 * (-locals.var_pb20b_dn8)) * locals.var_t1) + (assign23330_e17924 * locals.var_t1_dn8)) * locals.var_t2) + (assign23330_e17926 * locals.var_t2_dn8)) * locals.var_t4) + (assign23330_e17928 * locals.var_t4_dn8));
        locals.var_t5_dn9 = (((((((2.0 * (-locals.var_pb20b_dn9)) * locals.var_t1) + (assign23330_e17924 * locals.var_t1_dn9)) * locals.var_t2) + (assign23330_e17926 * locals.var_t2_dn9)) * locals.var_t4) + (assign23330_e17928 * locals.var_t4_dn9));
        locals.var_t5_dn10 = (((((((2.0 * (-locals.var_pb20b_dn10)) * locals.var_t1) + (assign23330_e17924 * locals.var_t1_dn10)) * locals.var_t2) + (assign23330_e17926 * locals.var_t2_dn10)) * locals.var_t4) + (assign23330_e17928 * locals.var_t4_dn10));
        locals.var_t5_dn13 = (((((((2.0 * (-locals.var_pb20b_dn13)) * locals.var_t1) + (assign23330_e17924 * locals.var_t1_dn13)) * locals.var_t2) + (assign23330_e17926 * locals.var_t2_dn13)) * locals.var_t4) + (assign23330_e17928 * locals.var_t4_dn13));
        locals.var_t5_rv = 0.0;

        let assign23340_e17933: f64 = (locals.var_t5 * locals.var_sqrt_pbsum);
        locals.var_dvth0 = assign23340_e17933;
        locals.var_dvth0_dn0 = ((locals.var_t5_dn0 * locals.var_sqrt_pbsum) + (locals.var_t5 * locals.var_sqrt_pbsum_dn0));
        locals.var_dvth0_dn2 = ((locals.var_t5_dn2 * locals.var_sqrt_pbsum) + (locals.var_t5 * locals.var_sqrt_pbsum_dn2));
        locals.var_dvth0_dn4 = ((locals.var_t5_dn4 * locals.var_sqrt_pbsum) + (locals.var_t5 * locals.var_sqrt_pbsum_dn4));
        locals.var_dvth0_dn5 = ((locals.var_t5_dn5 * locals.var_sqrt_pbsum) + (locals.var_t5 * locals.var_sqrt_pbsum_dn5));
        locals.var_dvth0_dn6 = ((locals.var_t5_dn6 * locals.var_sqrt_pbsum) + (locals.var_t5 * locals.var_sqrt_pbsum_dn6));
        locals.var_dvth0_dn7 = ((locals.var_t5_dn7 * locals.var_sqrt_pbsum) + (locals.var_t5 * locals.var_sqrt_pbsum_dn7));
        locals.var_dvth0_dn8 = ((locals.var_t5_dn8 * locals.var_sqrt_pbsum) + (locals.var_t5 * locals.var_sqrt_pbsum_dn8));
        locals.var_dvth0_dn9 = ((locals.var_t5_dn9 * locals.var_sqrt_pbsum) + (locals.var_t5 * locals.var_sqrt_pbsum_dn9));
        locals.var_dvth0_dn10 = ((locals.var_t5_dn10 * locals.var_sqrt_pbsum) + (locals.var_t5 * locals.var_sqrt_pbsum_dn10));
        locals.var_dvth0_dn13 = ((locals.var_t5_dn13 * locals.var_sqrt_pbsum) + (locals.var_t5 * locals.var_sqrt_pbsum_dn13));
        locals.var_dvth0_rv = 0.0;

        let assign23350_e17936: f64 = (locals.var_t5 / 2.0);
        let assign23350_e17938: f64 = (assign23350_e17936 / locals.var_sqrt_pbsum);
        locals.var_t6 = assign23350_e17938;
        locals.var_t6_dn0 = ((((locals.var_t5_dn0 / 2.0) * locals.var_sqrt_pbsum) - (assign23350_e17936 * locals.var_sqrt_pbsum_dn0)) / (locals.var_sqrt_pbsum * locals.var_sqrt_pbsum));
        locals.var_t6_dn2 = ((((locals.var_t5_dn2 / 2.0) * locals.var_sqrt_pbsum) - (assign23350_e17936 * locals.var_sqrt_pbsum_dn2)) / (locals.var_sqrt_pbsum * locals.var_sqrt_pbsum));
        locals.var_t6_dn4 = ((((locals.var_t5_dn4 / 2.0) * locals.var_sqrt_pbsum) - (assign23350_e17936 * locals.var_sqrt_pbsum_dn4)) / (locals.var_sqrt_pbsum * locals.var_sqrt_pbsum));
        locals.var_t6_dn5 = ((((locals.var_t5_dn5 / 2.0) * locals.var_sqrt_pbsum) - (assign23350_e17936 * locals.var_sqrt_pbsum_dn5)) / (locals.var_sqrt_pbsum * locals.var_sqrt_pbsum));
        locals.var_t6_dn6 = ((((locals.var_t5_dn6 / 2.0) * locals.var_sqrt_pbsum) - (assign23350_e17936 * locals.var_sqrt_pbsum_dn6)) / (locals.var_sqrt_pbsum * locals.var_sqrt_pbsum));
        locals.var_t6_dn7 = ((((locals.var_t5_dn7 / 2.0) * locals.var_sqrt_pbsum) - (assign23350_e17936 * locals.var_sqrt_pbsum_dn7)) / (locals.var_sqrt_pbsum * locals.var_sqrt_pbsum));
        locals.var_t6_dn8 = ((((locals.var_t5_dn8 / 2.0) * locals.var_sqrt_pbsum) - (assign23350_e17936 * locals.var_sqrt_pbsum_dn8)) / (locals.var_sqrt_pbsum * locals.var_sqrt_pbsum));
        locals.var_t6_dn9 = ((((locals.var_t5_dn9 / 2.0) * locals.var_sqrt_pbsum) - (assign23350_e17936 * locals.var_sqrt_pbsum_dn9)) / (locals.var_sqrt_pbsum * locals.var_sqrt_pbsum));
        locals.var_t6_dn10 = ((((locals.var_t5_dn10 / 2.0) * locals.var_sqrt_pbsum) - (assign23350_e17936 * locals.var_sqrt_pbsum_dn10)) / (locals.var_sqrt_pbsum * locals.var_sqrt_pbsum));
        locals.var_t6_dn13 = ((((locals.var_t5_dn13 / 2.0) * locals.var_sqrt_pbsum) - (assign23350_e17936 * locals.var_sqrt_pbsum_dn13)) / (locals.var_sqrt_pbsum * locals.var_sqrt_pbsum));
        locals.var_t6_rv = 0.0;

        let assign23360_e17942: f64 = (p.p137 - locals.var_pb20b);
        let assign23360_e17943: f64 = (2.0 * assign23360_e17942);
        let assign23360_e17945: f64 = (assign23360_e17943 * 1.034943e-10);
        let assign23360_e17947: f64 = (assign23360_e17945 * locals.var_t2);
        let assign23360_e17949: f64 = (assign23360_e17947 * locals.var_t4);
        let assign23360_e17951: f64 = (assign23360_e17949 * locals.var_sqrt_pbsum);
        locals.var_t7 = assign23360_e17951;
        locals.var_t7_dn0 = ((((((((2.0 * (-locals.var_pb20b_dn0)) * 1.034943e-10) * locals.var_t2) + (assign23360_e17945 * locals.var_t2_dn0)) * locals.var_t4) + (assign23360_e17947 * locals.var_t4_dn0)) * locals.var_sqrt_pbsum) + (assign23360_e17949 * locals.var_sqrt_pbsum_dn0));
        locals.var_t7_dn2 = ((((((((2.0 * (-locals.var_pb20b_dn2)) * 1.034943e-10) * locals.var_t2) + (assign23360_e17945 * locals.var_t2_dn2)) * locals.var_t4) + (assign23360_e17947 * locals.var_t4_dn2)) * locals.var_sqrt_pbsum) + (assign23360_e17949 * locals.var_sqrt_pbsum_dn2));
        locals.var_t7_dn4 = ((((((((2.0 * (-locals.var_pb20b_dn4)) * 1.034943e-10) * locals.var_t2) + (assign23360_e17945 * locals.var_t2_dn4)) * locals.var_t4) + (assign23360_e17947 * locals.var_t4_dn4)) * locals.var_sqrt_pbsum) + (assign23360_e17949 * locals.var_sqrt_pbsum_dn4));
        locals.var_t7_dn5 = ((((((((2.0 * (-locals.var_pb20b_dn5)) * 1.034943e-10) * locals.var_t2) + (assign23360_e17945 * locals.var_t2_dn5)) * locals.var_t4) + (assign23360_e17947 * locals.var_t4_dn5)) * locals.var_sqrt_pbsum) + (assign23360_e17949 * locals.var_sqrt_pbsum_dn5));
        locals.var_t7_dn6 = ((((((((2.0 * (-locals.var_pb20b_dn6)) * 1.034943e-10) * locals.var_t2) + (assign23360_e17945 * locals.var_t2_dn6)) * locals.var_t4) + (assign23360_e17947 * locals.var_t4_dn6)) * locals.var_sqrt_pbsum) + (assign23360_e17949 * locals.var_sqrt_pbsum_dn6));
        locals.var_t7_dn7 = ((((((((2.0 * (-locals.var_pb20b_dn7)) * 1.034943e-10) * locals.var_t2) + (assign23360_e17945 * locals.var_t2_dn7)) * locals.var_t4) + (assign23360_e17947 * locals.var_t4_dn7)) * locals.var_sqrt_pbsum) + (assign23360_e17949 * locals.var_sqrt_pbsum_dn7));
        locals.var_t7_dn8 = ((((((((2.0 * (-locals.var_pb20b_dn8)) * 1.034943e-10) * locals.var_t2) + (assign23360_e17945 * locals.var_t2_dn8)) * locals.var_t4) + (assign23360_e17947 * locals.var_t4_dn8)) * locals.var_sqrt_pbsum) + (assign23360_e17949 * locals.var_sqrt_pbsum_dn8));
        locals.var_t7_dn9 = ((((((((2.0 * (-locals.var_pb20b_dn9)) * 1.034943e-10) * locals.var_t2) + (assign23360_e17945 * locals.var_t2_dn9)) * locals.var_t4) + (assign23360_e17947 * locals.var_t4_dn9)) * locals.var_sqrt_pbsum) + (assign23360_e17949 * locals.var_sqrt_pbsum_dn9));
        locals.var_t7_dn10 = ((((((((2.0 * (-locals.var_pb20b_dn10)) * 1.034943e-10) * locals.var_t2) + (assign23360_e17945 * locals.var_t2_dn10)) * locals.var_t4) + (assign23360_e17947 * locals.var_t4_dn10)) * locals.var_sqrt_pbsum) + (assign23360_e17949 * locals.var_sqrt_pbsum_dn10));
        locals.var_t7_dn13 = ((((((((2.0 * (-locals.var_pb20b_dn13)) * 1.034943e-10) * locals.var_t2) + (assign23360_e17945 * locals.var_t2_dn13)) * locals.var_t4) + (assign23360_e17947 * locals.var_t4_dn13)) * locals.var_sqrt_pbsum) + (assign23360_e17949 * locals.var_sqrt_pbsum_dn13));
        locals.var_t7_rv = 0.0;

        let assign23370_e17953: f64 = (-2.0);
        let assign23370_e17955: f64 = (assign23370_e17953 * locals.var_t1);
        let assign23370_e17957: f64 = (assign23370_e17955 * locals.var_t2);
        let assign23370_e17959: f64 = (assign23370_e17957 * locals.var_t4);
        let assign23370_e17961: f64 = (assign23370_e17959 * locals.var_sqrt_pbsum);
        locals.var_t8 = assign23370_e17961;
        locals.var_t8_dn0 = (((((((assign23370_e17953 * locals.var_t1_dn0) * locals.var_t2) + (assign23370_e17955 * locals.var_t2_dn0)) * locals.var_t4) + (assign23370_e17957 * locals.var_t4_dn0)) * locals.var_sqrt_pbsum) + (assign23370_e17959 * locals.var_sqrt_pbsum_dn0));
        locals.var_t8_dn2 = (((((((assign23370_e17953 * locals.var_t1_dn2) * locals.var_t2) + (assign23370_e17955 * locals.var_t2_dn2)) * locals.var_t4) + (assign23370_e17957 * locals.var_t4_dn2)) * locals.var_sqrt_pbsum) + (assign23370_e17959 * locals.var_sqrt_pbsum_dn2));
        locals.var_t8_dn4 = (((((((assign23370_e17953 * locals.var_t1_dn4) * locals.var_t2) + (assign23370_e17955 * locals.var_t2_dn4)) * locals.var_t4) + (assign23370_e17957 * locals.var_t4_dn4)) * locals.var_sqrt_pbsum) + (assign23370_e17959 * locals.var_sqrt_pbsum_dn4));
        locals.var_t8_dn5 = (((((((assign23370_e17953 * locals.var_t1_dn5) * locals.var_t2) + (assign23370_e17955 * locals.var_t2_dn5)) * locals.var_t4) + (assign23370_e17957 * locals.var_t4_dn5)) * locals.var_sqrt_pbsum) + (assign23370_e17959 * locals.var_sqrt_pbsum_dn5));
        locals.var_t8_dn6 = (((((((assign23370_e17953 * locals.var_t1_dn6) * locals.var_t2) + (assign23370_e17955 * locals.var_t2_dn6)) * locals.var_t4) + (assign23370_e17957 * locals.var_t4_dn6)) * locals.var_sqrt_pbsum) + (assign23370_e17959 * locals.var_sqrt_pbsum_dn6));
        locals.var_t8_dn7 = (((((((assign23370_e17953 * locals.var_t1_dn7) * locals.var_t2) + (assign23370_e17955 * locals.var_t2_dn7)) * locals.var_t4) + (assign23370_e17957 * locals.var_t4_dn7)) * locals.var_sqrt_pbsum) + (assign23370_e17959 * locals.var_sqrt_pbsum_dn7));
        locals.var_t8_dn8 = (((((((assign23370_e17953 * locals.var_t1_dn8) * locals.var_t2) + (assign23370_e17955 * locals.var_t2_dn8)) * locals.var_t4) + (assign23370_e17957 * locals.var_t4_dn8)) * locals.var_sqrt_pbsum) + (assign23370_e17959 * locals.var_sqrt_pbsum_dn8));
        locals.var_t8_dn9 = (((((((assign23370_e17953 * locals.var_t1_dn9) * locals.var_t2) + (assign23370_e17955 * locals.var_t2_dn9)) * locals.var_t4) + (assign23370_e17957 * locals.var_t4_dn9)) * locals.var_sqrt_pbsum) + (assign23370_e17959 * locals.var_sqrt_pbsum_dn9));
        locals.var_t8_dn10 = (((((((assign23370_e17953 * locals.var_t1_dn10) * locals.var_t2) + (assign23370_e17955 * locals.var_t2_dn10)) * locals.var_t4) + (assign23370_e17957 * locals.var_t4_dn10)) * locals.var_sqrt_pbsum) + (assign23370_e17959 * locals.var_sqrt_pbsum_dn10));
        locals.var_t8_dn13 = (((((((assign23370_e17953 * locals.var_t1_dn13) * locals.var_t2) + (assign23370_e17955 * locals.var_t2_dn13)) * locals.var_t4) + (assign23370_e17957 * locals.var_t4_dn13)) * locals.var_sqrt_pbsum) + (assign23370_e17959 * locals.var_sqrt_pbsum_dn13));
        locals.var_t8_rv = 0.0;

        let assign23380_e17964: f64 = (locals.var_uc_sc3 / locals.var_lgate);
        locals.var_t1 = assign23380_e17964;
        locals.var_t1_dn0 = 0.0;
        locals.var_t1_dn2 = 0.0;
        locals.var_t1_dn4 = 0.0;
        locals.var_t1_dn5 = 0.0;
        locals.var_t1_dn6 = 0.0;
        locals.var_t1_dn7 = 0.0;
        locals.var_t1_dn8 = 0.0;
        locals.var_t1_dn9 = 0.0;
        locals.var_t1_dn10 = 0.0;
        locals.var_t1_dn13 = 0.0;
        locals.var_t1_rv = 0.0;

        let assign23390_e17968: f64 = (locals.var_t1 * locals.var_pbsum);
        let assign23390_e17969: f64 = (locals.var_uc_sc1 + assign23390_e17968);
        locals.var_t4 = assign23390_e17969;
        locals.var_t4_dn0 = ((locals.var_t1_dn0 * locals.var_pbsum) + (locals.var_t1 * locals.var_pbsum_dn0));
        locals.var_t4_dn2 = ((locals.var_t1_dn2 * locals.var_pbsum) + (locals.var_t1 * locals.var_pbsum_dn2));
        locals.var_t4_dn4 = ((locals.var_t1_dn4 * locals.var_pbsum) + (locals.var_t1 * locals.var_pbsum_dn4));
        locals.var_t4_dn5 = ((locals.var_t1_dn5 * locals.var_pbsum) + (locals.var_t1 * locals.var_pbsum_dn5));
        locals.var_t4_dn6 = ((locals.var_t1_dn6 * locals.var_pbsum) + (locals.var_t1 * locals.var_pbsum_dn6));
        locals.var_t4_dn7 = ((locals.var_t1_dn7 * locals.var_pbsum) + (locals.var_t1 * locals.var_pbsum_dn7));
        locals.var_t4_dn8 = ((locals.var_t1_dn8 * locals.var_pbsum) + (locals.var_t1 * locals.var_pbsum_dn8));
        locals.var_t4_dn9 = ((locals.var_t1_dn9 * locals.var_pbsum) + (locals.var_t1 * locals.var_pbsum_dn9));
        locals.var_t4_dn10 = ((locals.var_t1_dn10 * locals.var_pbsum) + (locals.var_t1 * locals.var_pbsum_dn10));
        locals.var_t4_dn13 = ((locals.var_t1_dn13 * locals.var_pbsum) + (locals.var_t1 * locals.var_pbsum_dn13));
        locals.var_t4_rv = 0.0;

        let assign23400_e17973: f64 = (locals.var_uc_sc2 * locals.var_vdsz);
        let assign23400_e17977: f64 = (p.p150 * locals.var_pbsum);
        let assign23400_e17978: f64 = (1.0 + assign23400_e17977);
        let assign23400_e17979: f64 = (assign23400_e17973 * assign23400_e17978);
        let assign23400_e17980: f64 = (locals.var_t4 + assign23400_e17979);
        locals.var_t5 = assign23400_e17980;
        locals.var_t5_dn0 = (locals.var_t4_dn0 + (((locals.var_uc_sc2 * locals.var_vdsz_dn0) * assign23400_e17978) + (assign23400_e17973 * (p.p150 * locals.var_pbsum_dn0))));
        locals.var_t5_dn2 = (locals.var_t4_dn2 + (((locals.var_uc_sc2 * locals.var_vdsz_dn2) * assign23400_e17978) + (assign23400_e17973 * (p.p150 * locals.var_pbsum_dn2))));
        locals.var_t5_dn4 = (locals.var_t4_dn4 + (((locals.var_uc_sc2 * locals.var_vdsz_dn4) * assign23400_e17978) + (assign23400_e17973 * (p.p150 * locals.var_pbsum_dn4))));
        locals.var_t5_dn5 = (locals.var_t4_dn5 + (((locals.var_uc_sc2 * locals.var_vdsz_dn5) * assign23400_e17978) + (assign23400_e17973 * (p.p150 * locals.var_pbsum_dn5))));
        locals.var_t5_dn6 = (locals.var_t4_dn6 + (((locals.var_uc_sc2 * locals.var_vdsz_dn6) * assign23400_e17978) + (assign23400_e17973 * (p.p150 * locals.var_pbsum_dn6))));
        locals.var_t5_dn7 = (locals.var_t4_dn7 + (((locals.var_uc_sc2 * locals.var_vdsz_dn7) * assign23400_e17978) + (assign23400_e17973 * (p.p150 * locals.var_pbsum_dn7))));
        locals.var_t5_dn8 = (locals.var_t4_dn8 + (((locals.var_uc_sc2 * locals.var_vdsz_dn8) * assign23400_e17978) + (assign23400_e17973 * (p.p150 * locals.var_pbsum_dn8))));
        locals.var_t5_dn9 = (locals.var_t4_dn9 + (((locals.var_uc_sc2 * locals.var_vdsz_dn9) * assign23400_e17978) + (assign23400_e17973 * (p.p150 * locals.var_pbsum_dn9))));
        locals.var_t5_dn10 = (locals.var_t4_dn10 + (((locals.var_uc_sc2 * locals.var_vdsz_dn10) * assign23400_e17978) + (assign23400_e17973 * (p.p150 * locals.var_pbsum_dn10))));
        locals.var_t5_dn13 = (locals.var_t4_dn13 + (((locals.var_uc_sc2 * locals.var_vdsz_dn13) * assign23400_e17978) + (assign23400_e17973 * (p.p150 * locals.var_pbsum_dn13))));
        locals.var_t5_rv = 0.0;

        let assign23410_e17983: f64 = (locals.var_dvth0 * locals.var_t5);
        locals.var_dvthsc = assign23410_e17983;
        locals.var_dvthsc_dn0 = ((locals.var_dvth0_dn0 * locals.var_t5) + (locals.var_dvth0 * locals.var_t5_dn0));
        locals.var_dvthsc_dn2 = ((locals.var_dvth0_dn2 * locals.var_t5) + (locals.var_dvth0 * locals.var_t5_dn2));
        locals.var_dvthsc_dn4 = ((locals.var_dvth0_dn4 * locals.var_t5) + (locals.var_dvth0 * locals.var_t5_dn4));
        locals.var_dvthsc_dn5 = ((locals.var_dvth0_dn5 * locals.var_t5) + (locals.var_dvth0 * locals.var_t5_dn5));
        locals.var_dvthsc_dn6 = ((locals.var_dvth0_dn6 * locals.var_t5) + (locals.var_dvth0 * locals.var_t5_dn6));
        locals.var_dvthsc_dn7 = ((locals.var_dvth0_dn7 * locals.var_t5) + (locals.var_dvth0 * locals.var_t5_dn7));
        locals.var_dvthsc_dn8 = ((locals.var_dvth0_dn8 * locals.var_t5) + (locals.var_dvth0 * locals.var_t5_dn8));
        locals.var_dvthsc_dn9 = ((locals.var_dvth0_dn9 * locals.var_t5) + (locals.var_dvth0 * locals.var_t5_dn9));
        locals.var_dvthsc_dn10 = ((locals.var_dvth0_dn10 * locals.var_t5) + (locals.var_dvth0 * locals.var_t5_dn10));
        locals.var_dvthsc_dn13 = ((locals.var_dvth0_dn13 * locals.var_t5) + (locals.var_dvth0 * locals.var_t5_dn13));
        locals.var_dvthsc_rv = 0.0;

        let assign23420_e17986: f64 = (1.0 / locals.var_cox);
        locals.var_t1 = assign23420_e17986;
        locals.var_t1_dn0 = (-(locals.var_cox_dn0 / (locals.var_cox * locals.var_cox)));
        locals.var_t1_dn2 = (-(locals.var_cox_dn2 / (locals.var_cox * locals.var_cox)));
        locals.var_t1_dn4 = (-(locals.var_cox_dn4 / (locals.var_cox * locals.var_cox)));
        locals.var_t1_dn5 = (-(locals.var_cox_dn5 / (locals.var_cox * locals.var_cox)));
        locals.var_t1_dn6 = (-(locals.var_cox_dn6 / (locals.var_cox * locals.var_cox)));
        locals.var_t1_dn7 = (-(locals.var_cox_dn7 / (locals.var_cox * locals.var_cox)));
        locals.var_t1_dn8 = (-(locals.var_cox_dn8 / (locals.var_cox * locals.var_cox)));
        locals.var_t1_dn9 = (-(locals.var_cox_dn9 / (locals.var_cox * locals.var_cox)));
        locals.var_t1_dn10 = (-(locals.var_cox_dn10 / (locals.var_cox * locals.var_cox)));
        locals.var_t1_dn13 = (-(locals.var_cox_dn13 / (locals.var_cox * locals.var_cox)));
        locals.var_t1_rv = 0.0;

        let assign23430_e17989: f64 = (locals.var_t1 * locals.var_t1);
        locals.var_t2 = assign23430_e17989;
        locals.var_t2_dn0 = ((locals.var_t1_dn0 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn0));
        locals.var_t2_dn2 = ((locals.var_t1_dn2 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn2));
        locals.var_t2_dn4 = ((locals.var_t1_dn4 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn4));
        locals.var_t2_dn5 = ((locals.var_t1_dn5 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn5));
        locals.var_t2_dn6 = ((locals.var_t1_dn6 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn6));
        locals.var_t2_dn7 = ((locals.var_t1_dn7 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn7));
        locals.var_t2_dn8 = ((locals.var_t1_dn8 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn8));
        locals.var_t2_dn9 = ((locals.var_t1_dn9 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn9));
        locals.var_t2_dn10 = ((locals.var_t1_dn10 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn10));
        locals.var_t2_dn13 = ((locals.var_t1_dn13 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn13));
        locals.var_t2_rv = 0.0;

        let assign23440_e17994: f64 = (locals.var_uc_wfc / locals.var_weff);
        let assign23440_e17995: f64 = (locals.var_cox + assign23440_e17994);
        let assign23440_e17996: f64 = (1.0 / assign23440_e17995);
        locals.var_t3 = assign23440_e17996;
        locals.var_t3_dn0 = (-(locals.var_cox_dn0 / (assign23440_e17995 * assign23440_e17995)));
        locals.var_t3_dn2 = (-(locals.var_cox_dn2 / (assign23440_e17995 * assign23440_e17995)));
        locals.var_t3_dn4 = (-(locals.var_cox_dn4 / (assign23440_e17995 * assign23440_e17995)));
        locals.var_t3_dn5 = (-(locals.var_cox_dn5 / (assign23440_e17995 * assign23440_e17995)));
        locals.var_t3_dn6 = (-(locals.var_cox_dn6 / (assign23440_e17995 * assign23440_e17995)));
        locals.var_t3_dn7 = (-(locals.var_cox_dn7 / (assign23440_e17995 * assign23440_e17995)));
        locals.var_t3_dn8 = (-(locals.var_cox_dn8 / (assign23440_e17995 * assign23440_e17995)));
        locals.var_t3_dn9 = (-(locals.var_cox_dn9 / (assign23440_e17995 * assign23440_e17995)));
        locals.var_t3_dn10 = (-(locals.var_cox_dn10 / (assign23440_e17995 * assign23440_e17995)));
        locals.var_t3_dn13 = (-(locals.var_cox_dn13 / (assign23440_e17995 * assign23440_e17995)));
        locals.var_t3_rv = 0.0;

        let assign23450_e17999: f64 = (locals.var_t3 * locals.var_t3);
        locals.var_t4 = assign23450_e17999;
        locals.var_t4_dn0 = ((locals.var_t3_dn0 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn0));
        locals.var_t4_dn2 = ((locals.var_t3_dn2 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn2));
        locals.var_t4_dn4 = ((locals.var_t3_dn4 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn4));
        locals.var_t4_dn5 = ((locals.var_t3_dn5 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn5));
        locals.var_t4_dn6 = ((locals.var_t3_dn6 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn6));
        locals.var_t4_dn7 = ((locals.var_t3_dn7 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn7));
        locals.var_t4_dn8 = ((locals.var_t3_dn8 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn8));
        locals.var_t4_dn9 = ((locals.var_t3_dn9 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn9));
        locals.var_t4_dn10 = ((locals.var_t3_dn10 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn10));
        locals.var_t4_dn13 = ((locals.var_t3_dn13 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn13));
        locals.var_t4_rv = 0.0;

        let assign23460_e18002: f64 = (locals.var_t1 - locals.var_t3);
        locals.var_t5 = assign23460_e18002;
        locals.var_t5_dn0 = (locals.var_t1_dn0 - locals.var_t3_dn0);
        locals.var_t5_dn2 = (locals.var_t1_dn2 - locals.var_t3_dn2);
        locals.var_t5_dn4 = (locals.var_t1_dn4 - locals.var_t3_dn4);
        locals.var_t5_dn5 = (locals.var_t1_dn5 - locals.var_t3_dn5);
        locals.var_t5_dn6 = (locals.var_t1_dn6 - locals.var_t3_dn6);
        locals.var_t5_dn7 = (locals.var_t1_dn7 - locals.var_t3_dn7);
        locals.var_t5_dn8 = (locals.var_t1_dn8 - locals.var_t3_dn8);
        locals.var_t5_dn9 = (locals.var_t1_dn9 - locals.var_t3_dn9);
        locals.var_t5_dn10 = (locals.var_t1_dn10 - locals.var_t3_dn10);
        locals.var_t5_dn13 = (locals.var_t1_dn13 - locals.var_t3_dn13);
        locals.var_t5_rv = 0.0;

        let assign23470_e18006: f64 = (locals.var_t2 - locals.var_t4);
        let assign23470_e18007: f64 = (locals.var_qb0 * assign23470_e18006);
        locals.var_t6 = assign23470_e18007;
        locals.var_t6_dn0 = ((locals.var_qb0_dn0 * assign23470_e18006) + (locals.var_qb0 * (locals.var_t2_dn0 - locals.var_t4_dn0)));
        locals.var_t6_dn2 = ((locals.var_qb0_dn2 * assign23470_e18006) + (locals.var_qb0 * (locals.var_t2_dn2 - locals.var_t4_dn2)));
        locals.var_t6_dn4 = ((locals.var_qb0_dn4 * assign23470_e18006) + (locals.var_qb0 * (locals.var_t2_dn4 - locals.var_t4_dn4)));
        locals.var_t6_dn5 = ((locals.var_qb0_dn5 * assign23470_e18006) + (locals.var_qb0 * (locals.var_t2_dn5 - locals.var_t4_dn5)));
        locals.var_t6_dn6 = ((locals.var_qb0_dn6 * assign23470_e18006) + (locals.var_qb0 * (locals.var_t2_dn6 - locals.var_t4_dn6)));
        locals.var_t6_dn7 = ((locals.var_qb0_dn7 * assign23470_e18006) + (locals.var_qb0 * (locals.var_t2_dn7 - locals.var_t4_dn7)));
        locals.var_t6_dn8 = ((locals.var_qb0_dn8 * assign23470_e18006) + (locals.var_qb0 * (locals.var_t2_dn8 - locals.var_t4_dn8)));
        locals.var_t6_dn9 = ((locals.var_qb0_dn9 * assign23470_e18006) + (locals.var_qb0 * (locals.var_t2_dn9 - locals.var_t4_dn9)));
        locals.var_t6_dn10 = ((locals.var_qb0_dn10 * assign23470_e18006) + (locals.var_qb0 * (locals.var_t2_dn10 - locals.var_t4_dn10)));
        locals.var_t6_dn13 = ((locals.var_qb0_dn13 * assign23470_e18006) + (locals.var_qb0 * (locals.var_t2_dn13 - locals.var_t4_dn13)));
        locals.var_t6_rv = 0.0;

        let assign23480_e18010: f64 = (locals.var_qb0 * locals.var_t5);
        let assign23480_e18013: f64 = (locals.var_uc_wvth0 / locals.var_wg);
        let assign23480_e18014: f64 = (assign23480_e18010 + assign23480_e18013);
        locals.var_dvthw = assign23480_e18014;
        locals.var_dvthw_dn0 = ((locals.var_qb0_dn0 * locals.var_t5) + (locals.var_qb0 * locals.var_t5_dn0));
        locals.var_dvthw_dn2 = ((locals.var_qb0_dn2 * locals.var_t5) + (locals.var_qb0 * locals.var_t5_dn2));
        locals.var_dvthw_dn4 = ((locals.var_qb0_dn4 * locals.var_t5) + (locals.var_qb0 * locals.var_t5_dn4));
        locals.var_dvthw_dn5 = ((locals.var_qb0_dn5 * locals.var_t5) + (locals.var_qb0 * locals.var_t5_dn5));
        locals.var_dvthw_dn6 = ((locals.var_qb0_dn6 * locals.var_t5) + (locals.var_qb0 * locals.var_t5_dn6));
        locals.var_dvthw_dn7 = ((locals.var_qb0_dn7 * locals.var_t5) + (locals.var_qb0 * locals.var_t5_dn7));
        locals.var_dvthw_dn8 = ((locals.var_qb0_dn8 * locals.var_t5) + (locals.var_qb0 * locals.var_t5_dn8));
        locals.var_dvthw_dn9 = ((locals.var_qb0_dn9 * locals.var_t5) + (locals.var_qb0 * locals.var_t5_dn9));
        locals.var_dvthw_dn10 = ((locals.var_qb0_dn10 * locals.var_t5) + (locals.var_qb0 * locals.var_t5_dn10));
        locals.var_dvthw_dn13 = ((locals.var_qb0_dn13 * locals.var_t5) + (locals.var_qb0 * locals.var_t5_dn13));
        locals.var_dvthw_rv = 0.0;

        let assign23490_e18017: f64 = (locals.var_dvthsc + locals.var_dvthlp);
        let assign23490_e18019: f64 = (assign23490_e18017 + locals.var_dvthw);
        let assign23490_e18021: f64 = (assign23490_e18019 + locals.var_dvthsm);
        locals.var_dvth = assign23490_e18021;
        locals.var_dvth_dn0 = ((locals.var_dvthsc_dn0 + locals.var_dvthlp_dn0) + locals.var_dvthw_dn0);
        locals.var_dvth_dn2 = ((locals.var_dvthsc_dn2 + locals.var_dvthlp_dn2) + locals.var_dvthw_dn2);
        locals.var_dvth_dn4 = ((locals.var_dvthsc_dn4 + locals.var_dvthlp_dn4) + locals.var_dvthw_dn4);
        locals.var_dvth_dn5 = ((locals.var_dvthsc_dn5 + locals.var_dvthlp_dn5) + locals.var_dvthw_dn5);
        locals.var_dvth_dn6 = ((locals.var_dvthsc_dn6 + locals.var_dvthlp_dn6) + locals.var_dvthw_dn6);
        locals.var_dvth_dn7 = ((locals.var_dvthsc_dn7 + locals.var_dvthlp_dn7) + locals.var_dvthw_dn7);
        locals.var_dvth_dn8 = ((locals.var_dvthsc_dn8 + locals.var_dvthlp_dn8) + locals.var_dvthw_dn8);
        locals.var_dvth_dn9 = ((locals.var_dvthsc_dn9 + locals.var_dvthlp_dn9) + locals.var_dvthw_dn9);
        locals.var_dvth_dn10 = ((locals.var_dvthsc_dn10 + locals.var_dvthlp_dn10) + locals.var_dvthw_dn10);
        locals.var_dvth_dn13 = ((locals.var_dvthsc_dn13 + locals.var_dvthlp_dn13) + locals.var_dvthw_dn13);
        locals.var_dvth_rv = 0.0;

        let assign23500_e18025: f64 = (locals.var_pb2 - locals.var_vbsz);
        let assign23500_e18026: f64 = (locals.var_qnsub_esi2 * assign23500_e18025);
        let assign23500_e18027: f64 = (assign23500_e18026).sqrt();
        locals.var_t2 = assign23500_e18027;
        locals.var_t2_dn0 = (((locals.var_qnsub_esi2_dn0 * assign23500_e18025) + (locals.var_qnsub_esi2 * (locals.var_pb2_dn0 - locals.var_vbsz_dn0))) / (2.0 * assign23500_e18027));
        locals.var_t2_dn2 = (((locals.var_qnsub_esi2_dn2 * assign23500_e18025) + (locals.var_qnsub_esi2 * (locals.var_pb2_dn2 - locals.var_vbsz_dn2))) / (2.0 * assign23500_e18027));
        locals.var_t2_dn4 = (((locals.var_qnsub_esi2_dn4 * assign23500_e18025) + (locals.var_qnsub_esi2 * (locals.var_pb2_dn4 - locals.var_vbsz_dn4))) / (2.0 * assign23500_e18027));
        locals.var_t2_dn5 = (((locals.var_qnsub_esi2_dn5 * assign23500_e18025) + (locals.var_qnsub_esi2 * (locals.var_pb2_dn5 - locals.var_vbsz_dn5))) / (2.0 * assign23500_e18027));
        locals.var_t2_dn6 = (((locals.var_qnsub_esi2_dn6 * assign23500_e18025) + (locals.var_qnsub_esi2 * (locals.var_pb2_dn6 - locals.var_vbsz_dn6))) / (2.0 * assign23500_e18027));
        locals.var_t2_dn7 = (((locals.var_qnsub_esi2_dn7 * assign23500_e18025) + (locals.var_qnsub_esi2 * (locals.var_pb2_dn7 - locals.var_vbsz_dn7))) / (2.0 * assign23500_e18027));
        locals.var_t2_dn8 = (((locals.var_qnsub_esi2_dn8 * assign23500_e18025) + (locals.var_qnsub_esi2 * (locals.var_pb2_dn8 - locals.var_vbsz_dn8))) / (2.0 * assign23500_e18027));
        locals.var_t2_dn9 = (((locals.var_qnsub_esi2_dn9 * assign23500_e18025) + (locals.var_qnsub_esi2 * (locals.var_pb2_dn9 - locals.var_vbsz_dn9))) / (2.0 * assign23500_e18027));
        locals.var_t2_dn10 = (((locals.var_qnsub_esi2_dn10 * assign23500_e18025) + (locals.var_qnsub_esi2 * (locals.var_pb2_dn10 - locals.var_vbsz_dn10))) / (2.0 * assign23500_e18027));
        locals.var_t2_dn13 = (((locals.var_qnsub_esi2_dn13 * assign23500_e18025) + (locals.var_qnsub_esi2 * (locals.var_pb2_dn13 - locals.var_vbsz_dn13))) / (2.0 * assign23500_e18027));
        locals.var_t2_rv = 0.0;

        let assign23510_e18030: f64 = (locals.var_pb2 + locals.var_vfb);
        let assign23510_e18033: f64 = (locals.var_t2 * locals.var_cox0_inv);
        let assign23510_e18034: f64 = (assign23510_e18030 + assign23510_e18033);
        let assign23510_e18036: f64 = (assign23510_e18034 - locals.var_dvth);
        locals.var_vth = assign23510_e18036;
        locals.var_vth_dn0 = ((locals.var_pb2_dn0 + (locals.var_t2_dn0 * locals.var_cox0_inv)) - locals.var_dvth_dn0);
        locals.var_vth_dn2 = ((locals.var_pb2_dn2 + (locals.var_t2_dn2 * locals.var_cox0_inv)) - locals.var_dvth_dn2);
        locals.var_vth_dn4 = ((locals.var_pb2_dn4 + (locals.var_t2_dn4 * locals.var_cox0_inv)) - locals.var_dvth_dn4);
        locals.var_vth_dn5 = ((locals.var_pb2_dn5 + (locals.var_t2_dn5 * locals.var_cox0_inv)) - locals.var_dvth_dn5);
        locals.var_vth_dn6 = ((locals.var_pb2_dn6 + (locals.var_t2_dn6 * locals.var_cox0_inv)) - locals.var_dvth_dn6);
        locals.var_vth_dn7 = ((locals.var_pb2_dn7 + (locals.var_t2_dn7 * locals.var_cox0_inv)) - locals.var_dvth_dn7);
        locals.var_vth_dn8 = ((locals.var_pb2_dn8 + (locals.var_t2_dn8 * locals.var_cox0_inv)) - locals.var_dvth_dn8);
        locals.var_vth_dn9 = ((locals.var_pb2_dn9 + (locals.var_t2_dn9 * locals.var_cox0_inv)) - locals.var_dvth_dn9);
        locals.var_vth_dn10 = ((locals.var_pb2_dn10 + (locals.var_t2_dn10 * locals.var_cox0_inv)) - locals.var_dvth_dn10);
        locals.var_vth_dn13 = ((locals.var_pb2_dn13 + (locals.var_t2_dn13 * locals.var_cox0_inv)) - locals.var_dvth_dn13);
        locals.var_vth_rv = 0.0;

        let assign23520_e18039: f64 = (locals.var_cnst0 * locals.var_cox_inv);
        locals.var_fac1 = assign23520_e18039;
        locals.var_fac1_dn0 = ((locals.var_cnst0_dn0 * locals.var_cox_inv) + (locals.var_cnst0 * locals.var_cox_inv_dn0));
        locals.var_fac1_dn2 = ((locals.var_cnst0_dn2 * locals.var_cox_inv) + (locals.var_cnst0 * locals.var_cox_inv_dn2));
        locals.var_fac1_dn4 = ((locals.var_cnst0_dn4 * locals.var_cox_inv) + (locals.var_cnst0 * locals.var_cox_inv_dn4));
        locals.var_fac1_dn5 = ((locals.var_cnst0_dn5 * locals.var_cox_inv) + (locals.var_cnst0 * locals.var_cox_inv_dn5));
        locals.var_fac1_dn6 = ((locals.var_cnst0_dn6 * locals.var_cox_inv) + (locals.var_cnst0 * locals.var_cox_inv_dn6));
        locals.var_fac1_dn7 = ((locals.var_cnst0_dn7 * locals.var_cox_inv) + (locals.var_cnst0 * locals.var_cox_inv_dn7));
        locals.var_fac1_dn8 = ((locals.var_cnst0_dn8 * locals.var_cox_inv) + (locals.var_cnst0 * locals.var_cox_inv_dn8));
        locals.var_fac1_dn9 = ((locals.var_cnst0_dn9 * locals.var_cox_inv) + (locals.var_cnst0 * locals.var_cox_inv_dn9));
        locals.var_fac1_dn10 = ((locals.var_cnst0_dn10 * locals.var_cox_inv) + (locals.var_cnst0 * locals.var_cox_inv_dn10));
        locals.var_fac1_dn13 = ((locals.var_cnst0_dn13 * locals.var_cox_inv) + (locals.var_cnst0 * locals.var_cox_inv_dn13));
        locals.var_fac1_rv = 0.0;

        let assign23530_e18042: f64 = (locals.var_fac1 * locals.var_fac1);
        locals.var_fac1p2 = assign23530_e18042;
        locals.var_fac1p2_dn0 = ((locals.var_fac1_dn0 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn0));
        locals.var_fac1p2_dn2 = ((locals.var_fac1_dn2 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn2));
        locals.var_fac1p2_dn4 = ((locals.var_fac1_dn4 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn4));
        locals.var_fac1p2_dn5 = ((locals.var_fac1_dn5 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn5));
        locals.var_fac1p2_dn6 = ((locals.var_fac1_dn6 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn6));
        locals.var_fac1p2_dn7 = ((locals.var_fac1_dn7 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn7));
        locals.var_fac1p2_dn8 = ((locals.var_fac1_dn8 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn8));
        locals.var_fac1p2_dn9 = ((locals.var_fac1_dn9 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn9));
        locals.var_fac1p2_dn10 = ((locals.var_fac1_dn10 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn10));
        locals.var_fac1p2_dn13 = ((locals.var_fac1_dn13 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn13));
        locals.var_fac1p2_rv = 0.0;

        locals.var_dppg = 0.0;
        locals.var_dppg_dn0 = 0.0;
        locals.var_dppg_dn2 = 0.0;
        locals.var_dppg_dn4 = 0.0;
        locals.var_dppg_dn5 = 0.0;
        locals.var_dppg_dn6 = 0.0;
        locals.var_dppg_dn7 = 0.0;
        locals.var_dppg_dn8 = 0.0;
        locals.var_dppg_dn9 = 0.0;
        locals.var_dppg_dn10 = 0.0;
        locals.var_dppg_dn13 = 0.0;
        locals.var_dppg_rv = 0.0;

        let assign23550_e18046: f64 = if locals.var_flg_pgd == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard431 = assign23550_e18046;
        locals.var_guard431_rv = 0.0;

        let (assign23560_e18050, assign23560_e18050_d_n0, assign23560_e18050_d_n2, assign23560_e18050_d_n4, assign23560_e18050_d_n5, assign23560_e18050_d_n6, assign23560_e18050_d_n7, assign23560_e18050_d_n8, assign23560_e18050_d_n9, assign23560_e18050_d_n10, assign23560_e18050_d_n13,) = {
    if (locals.var_guard431 != 0.0) {
        (locals.var_vgsz, locals.var_vgsz_dn0, locals.var_vgsz_dn2, locals.var_vgsz_dn4, locals.var_vgsz_dn5, locals.var_vgsz_dn6, locals.var_vgsz_dn7, locals.var_vgsz_dn8, locals.var_vgsz_dn9, locals.var_vgsz_dn10, locals.var_vgsz_dn13,)
    } else {
        (locals.var_t7, locals.var_t7_dn0, locals.var_t7_dn2, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn13,)
    }
};
        locals.var_t7 = assign23560_e18050;
        locals.var_t7_dn0 = assign23560_e18050_d_n0;
        locals.var_t7_dn2 = assign23560_e18050_d_n2;
        locals.var_t7_dn4 = assign23560_e18050_d_n4;
        locals.var_t7_dn5 = assign23560_e18050_d_n5;
        locals.var_t7_dn6 = assign23560_e18050_d_n6;
        locals.var_t7_dn7 = assign23560_e18050_d_n7;
        locals.var_t7_dn8 = assign23560_e18050_d_n8;
        locals.var_t7_dn9 = assign23560_e18050_d_n9;
        locals.var_t7_dn10 = assign23560_e18050_d_n10;
        locals.var_t7_dn13 = assign23560_e18050_d_n13;
        locals.var_t7_rv = 0.0;

        let (assign23570_e18054, assign23570_e18054_d_n0, assign23570_e18054_d_n2, assign23570_e18054_d_n4, assign23570_e18054_d_n5, assign23570_e18054_d_n6, assign23570_e18054_d_n7, assign23570_e18054_d_n8, assign23570_e18054_d_n9, assign23570_e18054_d_n10, assign23570_e18054_d_n13,) = {
    if (locals.var_guard431 != 0.0) {
        (locals.var_cnstpgd, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign23570_e18054;
        locals.var_t0_dn0 = assign23570_e18054_d_n0;
        locals.var_t0_dn2 = assign23570_e18054_d_n2;
        locals.var_t0_dn4 = assign23570_e18054_d_n4;
        locals.var_t0_dn5 = assign23570_e18054_d_n5;
        locals.var_t0_dn6 = assign23570_e18054_d_n6;
        locals.var_t0_dn7 = assign23570_e18054_d_n7;
        locals.var_t0_dn8 = assign23570_e18054_d_n8;
        locals.var_t0_dn9 = assign23570_e18054_d_n9;
        locals.var_t0_dn10 = assign23570_e18054_d_n10;
        locals.var_t0_dn13 = assign23570_e18054_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign23580_e18060, assign23580_e18060_d_n0, assign23580_e18060_d_n2, assign23580_e18060_d_n4, assign23580_e18060_d_n5, assign23580_e18060_d_n6, assign23580_e18060_d_n7, assign23580_e18060_d_n8, assign23580_e18060_d_n9, assign23580_e18060_d_n10, assign23580_e18060_d_n13,) = {
    if (locals.var_guard431 != 0.0) {
        let assign23580_e18058: f64 = (locals.var_t7 - p.p152);
        (assign23580_e18058, locals.var_t7_dn0, locals.var_t7_dn2, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn13,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn13,)
    }
};
        locals.var_t3 = assign23580_e18060;
        locals.var_t3_dn0 = assign23580_e18060_d_n0;
        locals.var_t3_dn2 = assign23580_e18060_d_n2;
        locals.var_t3_dn4 = assign23580_e18060_d_n4;
        locals.var_t3_dn5 = assign23580_e18060_d_n5;
        locals.var_t3_dn6 = assign23580_e18060_d_n6;
        locals.var_t3_dn7 = assign23580_e18060_d_n7;
        locals.var_t3_dn8 = assign23580_e18060_d_n8;
        locals.var_t3_dn9 = assign23580_e18060_d_n9;
        locals.var_t3_dn10 = assign23580_e18060_d_n10;
        locals.var_t3_dn13 = assign23580_e18060_d_n13;
        locals.var_t3_rv = 0.0;

        let assign23590_e18063: f64 = (-3.0);
        let assign23590_e18064: f64 = if locals.var_t3 < assign23590_e18063 { 1.0 } else { 0.0 };
        locals.var_guard432 = assign23590_e18064;
        locals.var_guard432_rv = 0.0;

        let (assign23600_e18070, assign23600_e18070_d_n0, assign23600_e18070_d_n2, assign23600_e18070_d_n4, assign23600_e18070_d_n5, assign23600_e18070_d_n6, assign23600_e18070_d_n7, assign23600_e18070_d_n8, assign23600_e18070_d_n9, assign23600_e18070_d_n10, assign23600_e18070_d_n13,) = {
    if ((locals.var_guard431 != 0.0) && (locals.var_guard432 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn13,)
    }
};
        locals.var_t6 = assign23600_e18070;
        locals.var_t6_dn0 = assign23600_e18070_d_n0;
        locals.var_t6_dn2 = assign23600_e18070_d_n2;
        locals.var_t6_dn4 = assign23600_e18070_d_n4;
        locals.var_t6_dn5 = assign23600_e18070_d_n5;
        locals.var_t6_dn6 = assign23600_e18070_d_n6;
        locals.var_t6_dn7 = assign23600_e18070_d_n7;
        locals.var_t6_dn8 = assign23600_e18070_d_n8;
        locals.var_t6_dn9 = assign23600_e18070_d_n9;
        locals.var_t6_dn10 = assign23600_e18070_d_n10;
        locals.var_t6_dn13 = assign23600_e18070_d_n13;
        locals.var_t6_rv = 0.0;

        let (assign23610_e18076, assign23610_e18076_d_n0, assign23610_e18076_d_n2, assign23610_e18076_d_n4, assign23610_e18076_d_n5, assign23610_e18076_d_n6, assign23610_e18076_d_n7, assign23610_e18076_d_n8, assign23610_e18076_d_n9, assign23610_e18076_d_n10, assign23610_e18076_d_n13,) = {
    if ((locals.var_guard431 != 0.0) && (locals.var_guard432 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dppg, locals.var_dppg_dn0, locals.var_dppg_dn2, locals.var_dppg_dn4, locals.var_dppg_dn5, locals.var_dppg_dn6, locals.var_dppg_dn7, locals.var_dppg_dn8, locals.var_dppg_dn9, locals.var_dppg_dn10, locals.var_dppg_dn13,)
    }
};
        locals.var_dppg = assign23610_e18076;
        locals.var_dppg_dn0 = assign23610_e18076_d_n0;
        locals.var_dppg_dn2 = assign23610_e18076_d_n2;
        locals.var_dppg_dn4 = assign23610_e18076_d_n4;
        locals.var_dppg_dn5 = assign23610_e18076_d_n5;
        locals.var_dppg_dn6 = assign23610_e18076_d_n6;
        locals.var_dppg_dn7 = assign23610_e18076_d_n7;
        locals.var_dppg_dn8 = assign23610_e18076_d_n8;
        locals.var_dppg_dn9 = assign23610_e18076_d_n9;
        locals.var_dppg_dn10 = assign23610_e18076_d_n10;
        locals.var_dppg_dn13 = assign23610_e18076_d_n13;
        locals.var_dppg_rv = 0.0;

        let assign23620_e18079: f64 = if locals.var_t3 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard433 = assign23620_e18079;
        locals.var_guard433_rv = 0.0;

        let (assign23630_e18104, assign23630_e18104_d_n0, assign23630_e18104_d_n2, assign23630_e18104_d_n4, assign23630_e18104_d_n5, assign23630_e18104_d_n6, assign23630_e18104_d_n7, assign23630_e18104_d_n8, assign23630_e18104_d_n9, assign23630_e18104_d_n10, assign23630_e18104_d_n13,) = {
    if (((locals.var_guard431 != 0.0) && (locals.var_guard432 == 0.0)) && (locals.var_guard433 != 0.0)) {
        let assign23630_e18091: f64 = (1.0 / 3.0);
        let assign23630_e18092: f64 = (2.0 * assign23630_e18091);
        let assign23630_e18095: f64 = (locals.var_t3 * 3.0);
        let assign23630_e18098: f64 = (1.0 / 27.0);
        let assign23630_e18099: f64 = (assign23630_e18095 * assign23630_e18098);
        let assign23630_e18100: f64 = (assign23630_e18092 + assign23630_e18099);
        let assign23630_e18101: f64 = (locals.var_t3 * assign23630_e18100);
        let assign23630_e18102: f64 = (1.0 + assign23630_e18101);
        (assign23630_e18102, ((locals.var_t3_dn0 * assign23630_e18100) + (locals.var_t3 * ((locals.var_t3_dn0 * 3.0) * assign23630_e18098))), ((locals.var_t3_dn2 * assign23630_e18100) + (locals.var_t3 * ((locals.var_t3_dn2 * 3.0) * assign23630_e18098))), ((locals.var_t3_dn4 * assign23630_e18100) + (locals.var_t3 * ((locals.var_t3_dn4 * 3.0) * assign23630_e18098))), ((locals.var_t3_dn5 * assign23630_e18100) + (locals.var_t3 * ((locals.var_t3_dn5 * 3.0) * assign23630_e18098))), ((locals.var_t3_dn6 * assign23630_e18100) + (locals.var_t3 * ((locals.var_t3_dn6 * 3.0) * assign23630_e18098))), ((locals.var_t3_dn7 * assign23630_e18100) + (locals.var_t3 * ((locals.var_t3_dn7 * 3.0) * assign23630_e18098))), ((locals.var_t3_dn8 * assign23630_e18100) + (locals.var_t3 * ((locals.var_t3_dn8 * 3.0) * assign23630_e18098))), ((locals.var_t3_dn9 * assign23630_e18100) + (locals.var_t3 * ((locals.var_t3_dn9 * 3.0) * assign23630_e18098))), ((locals.var_t3_dn10 * assign23630_e18100) + (locals.var_t3 * ((locals.var_t3_dn10 * 3.0) * assign23630_e18098))), ((locals.var_t3_dn13 * assign23630_e18100) + (locals.var_t3 * ((locals.var_t3_dn13 * 3.0) * assign23630_e18098))),)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn13,)
    }
};
        locals.var_t6 = assign23630_e18104;
        locals.var_t6_dn0 = assign23630_e18104_d_n0;
        locals.var_t6_dn2 = assign23630_e18104_d_n2;
        locals.var_t6_dn4 = assign23630_e18104_d_n4;
        locals.var_t6_dn5 = assign23630_e18104_d_n5;
        locals.var_t6_dn6 = assign23630_e18104_d_n6;
        locals.var_t6_dn7 = assign23630_e18104_d_n7;
        locals.var_t6_dn8 = assign23630_e18104_d_n8;
        locals.var_t6_dn9 = assign23630_e18104_d_n9;
        locals.var_t6_dn10 = assign23630_e18104_d_n10;
        locals.var_t6_dn13 = assign23630_e18104_d_n13;
        locals.var_t6_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_62(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign23640_e18129, assign23640_e18129_d_n0, assign23640_e18129_d_n2, assign23640_e18129_d_n4, assign23640_e18129_d_n5, assign23640_e18129_d_n6, assign23640_e18129_d_n7, assign23640_e18129_d_n8, assign23640_e18129_d_n9, assign23640_e18129_d_n10, assign23640_e18129_d_n13,) = {
    if (((locals.var_guard431 != 0.0) && (locals.var_guard432 == 0.0)) && (locals.var_guard433 != 0.0)) {
        let assign23640_e18117: f64 = (1.0 / 3.0);
        let assign23640_e18121: f64 = (1.0 / 27.0);
        let assign23640_e18122: f64 = (locals.var_t3 * assign23640_e18121);
        let assign23640_e18123: f64 = (assign23640_e18117 + assign23640_e18122);
        let assign23640_e18124: f64 = (locals.var_t3 * assign23640_e18123);
        let assign23640_e18125: f64 = (1.0 + assign23640_e18124);
        let assign23640_e18126: f64 = (locals.var_t3 * assign23640_e18125);
        let assign23640_e18127: f64 = (1.0 + assign23640_e18126);
        (assign23640_e18127, ((locals.var_t3_dn0 * assign23640_e18125) + (locals.var_t3 * ((locals.var_t3_dn0 * assign23640_e18123) + (locals.var_t3 * (locals.var_t3_dn0 * assign23640_e18121))))), ((locals.var_t3_dn2 * assign23640_e18125) + (locals.var_t3 * ((locals.var_t3_dn2 * assign23640_e18123) + (locals.var_t3 * (locals.var_t3_dn2 * assign23640_e18121))))), ((locals.var_t3_dn4 * assign23640_e18125) + (locals.var_t3 * ((locals.var_t3_dn4 * assign23640_e18123) + (locals.var_t3 * (locals.var_t3_dn4 * assign23640_e18121))))), ((locals.var_t3_dn5 * assign23640_e18125) + (locals.var_t3 * ((locals.var_t3_dn5 * assign23640_e18123) + (locals.var_t3 * (locals.var_t3_dn5 * assign23640_e18121))))), ((locals.var_t3_dn6 * assign23640_e18125) + (locals.var_t3 * ((locals.var_t3_dn6 * assign23640_e18123) + (locals.var_t3 * (locals.var_t3_dn6 * assign23640_e18121))))), ((locals.var_t3_dn7 * assign23640_e18125) + (locals.var_t3 * ((locals.var_t3_dn7 * assign23640_e18123) + (locals.var_t3 * (locals.var_t3_dn7 * assign23640_e18121))))), ((locals.var_t3_dn8 * assign23640_e18125) + (locals.var_t3 * ((locals.var_t3_dn8 * assign23640_e18123) + (locals.var_t3 * (locals.var_t3_dn8 * assign23640_e18121))))), ((locals.var_t3_dn9 * assign23640_e18125) + (locals.var_t3 * ((locals.var_t3_dn9 * assign23640_e18123) + (locals.var_t3 * (locals.var_t3_dn9 * assign23640_e18121))))), ((locals.var_t3_dn10 * assign23640_e18125) + (locals.var_t3 * ((locals.var_t3_dn10 * assign23640_e18123) + (locals.var_t3 * (locals.var_t3_dn10 * assign23640_e18121))))), ((locals.var_t3_dn13 * assign23640_e18125) + (locals.var_t3 * ((locals.var_t3_dn13 * assign23640_e18123) + (locals.var_t3 * (locals.var_t3_dn13 * assign23640_e18121))))),)
    } else {
        (locals.var_dppg, locals.var_dppg_dn0, locals.var_dppg_dn2, locals.var_dppg_dn4, locals.var_dppg_dn5, locals.var_dppg_dn6, locals.var_dppg_dn7, locals.var_dppg_dn8, locals.var_dppg_dn9, locals.var_dppg_dn10, locals.var_dppg_dn13,)
    }
};
        locals.var_dppg = assign23640_e18129;
        locals.var_dppg_dn0 = assign23640_e18129_d_n0;
        locals.var_dppg_dn2 = assign23640_e18129_d_n2;
        locals.var_dppg_dn4 = assign23640_e18129_d_n4;
        locals.var_dppg_dn5 = assign23640_e18129_d_n5;
        locals.var_dppg_dn6 = assign23640_e18129_d_n6;
        locals.var_dppg_dn7 = assign23640_e18129_d_n7;
        locals.var_dppg_dn8 = assign23640_e18129_d_n8;
        locals.var_dppg_dn9 = assign23640_e18129_d_n9;
        locals.var_dppg_dn10 = assign23640_e18129_d_n10;
        locals.var_dppg_dn13 = assign23640_e18129_d_n13;
        locals.var_dppg_rv = 0.0;

        let (assign23650_e18159, assign23650_e18159_d_n0, assign23650_e18159_d_n2, assign23650_e18159_d_n4, assign23650_e18159_d_n5, assign23650_e18159_d_n6, assign23650_e18159_d_n7, assign23650_e18159_d_n8, assign23650_e18159_d_n9, assign23650_e18159_d_n10, assign23650_e18159_d_n13,) = {
    if (((locals.var_guard431 != 0.0) && (locals.var_guard432 == 0.0)) && (locals.var_guard433 == 0.0)) {
        let assign23650_e18142: f64 = (1.0 / 3.0);
        let assign23650_e18143: f64 = (2.0 * assign23650_e18142);
        let assign23650_e18147: f64 = (3.0 * 0.0402052934513951);
        let assign23650_e18150: f64 = (locals.var_t3 * 4.0);
        let assign23650_e18152: f64 = (assign23650_e18150 * 0.148148111111111);
        let assign23650_e18153: f64 = (assign23650_e18147 + assign23650_e18152);
        let assign23650_e18154: f64 = (locals.var_t3 * assign23650_e18153);
        let assign23650_e18155: f64 = (assign23650_e18143 + assign23650_e18154);
        let assign23650_e18156: f64 = (locals.var_t3 * assign23650_e18155);
        let assign23650_e18157: f64 = (1.0 + assign23650_e18156);
        (assign23650_e18157, ((locals.var_t3_dn0 * assign23650_e18155) + (locals.var_t3 * ((locals.var_t3_dn0 * assign23650_e18153) + (locals.var_t3 * ((locals.var_t3_dn0 * 4.0) * 0.148148111111111))))), ((locals.var_t3_dn2 * assign23650_e18155) + (locals.var_t3 * ((locals.var_t3_dn2 * assign23650_e18153) + (locals.var_t3 * ((locals.var_t3_dn2 * 4.0) * 0.148148111111111))))), ((locals.var_t3_dn4 * assign23650_e18155) + (locals.var_t3 * ((locals.var_t3_dn4 * assign23650_e18153) + (locals.var_t3 * ((locals.var_t3_dn4 * 4.0) * 0.148148111111111))))), ((locals.var_t3_dn5 * assign23650_e18155) + (locals.var_t3 * ((locals.var_t3_dn5 * assign23650_e18153) + (locals.var_t3 * ((locals.var_t3_dn5 * 4.0) * 0.148148111111111))))), ((locals.var_t3_dn6 * assign23650_e18155) + (locals.var_t3 * ((locals.var_t3_dn6 * assign23650_e18153) + (locals.var_t3 * ((locals.var_t3_dn6 * 4.0) * 0.148148111111111))))), ((locals.var_t3_dn7 * assign23650_e18155) + (locals.var_t3 * ((locals.var_t3_dn7 * assign23650_e18153) + (locals.var_t3 * ((locals.var_t3_dn7 * 4.0) * 0.148148111111111))))), ((locals.var_t3_dn8 * assign23650_e18155) + (locals.var_t3 * ((locals.var_t3_dn8 * assign23650_e18153) + (locals.var_t3 * ((locals.var_t3_dn8 * 4.0) * 0.148148111111111))))), ((locals.var_t3_dn9 * assign23650_e18155) + (locals.var_t3 * ((locals.var_t3_dn9 * assign23650_e18153) + (locals.var_t3 * ((locals.var_t3_dn9 * 4.0) * 0.148148111111111))))), ((locals.var_t3_dn10 * assign23650_e18155) + (locals.var_t3 * ((locals.var_t3_dn10 * assign23650_e18153) + (locals.var_t3 * ((locals.var_t3_dn10 * 4.0) * 0.148148111111111))))), ((locals.var_t3_dn13 * assign23650_e18155) + (locals.var_t3 * ((locals.var_t3_dn13 * assign23650_e18153) + (locals.var_t3 * ((locals.var_t3_dn13 * 4.0) * 0.148148111111111))))),)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn13,)
    }
};
        locals.var_t6 = assign23650_e18159;
        locals.var_t6_dn0 = assign23650_e18159_d_n0;
        locals.var_t6_dn2 = assign23650_e18159_d_n2;
        locals.var_t6_dn4 = assign23650_e18159_d_n4;
        locals.var_t6_dn5 = assign23650_e18159_d_n5;
        locals.var_t6_dn6 = assign23650_e18159_d_n6;
        locals.var_t6_dn7 = assign23650_e18159_d_n7;
        locals.var_t6_dn8 = assign23650_e18159_d_n8;
        locals.var_t6_dn9 = assign23650_e18159_d_n9;
        locals.var_t6_dn10 = assign23650_e18159_d_n10;
        locals.var_t6_dn13 = assign23650_e18159_d_n13;
        locals.var_t6_rv = 0.0;

        let (assign23660_e18187, assign23660_e18187_d_n0, assign23660_e18187_d_n2, assign23660_e18187_d_n4, assign23660_e18187_d_n5, assign23660_e18187_d_n6, assign23660_e18187_d_n7, assign23660_e18187_d_n8, assign23660_e18187_d_n9, assign23660_e18187_d_n10, assign23660_e18187_d_n13,) = {
    if (((locals.var_guard431 != 0.0) && (locals.var_guard432 == 0.0)) && (locals.var_guard433 == 0.0)) {
        let assign23660_e18173: f64 = (1.0 / 3.0);
        let assign23660_e18178: f64 = (locals.var_t3 * 0.148148111111111);
        let assign23660_e18179: f64 = (0.0402052934513951 + assign23660_e18178);
        let assign23660_e18180: f64 = (locals.var_t3 * assign23660_e18179);
        let assign23660_e18181: f64 = (assign23660_e18173 + assign23660_e18180);
        let assign23660_e18182: f64 = (locals.var_t3 * assign23660_e18181);
        let assign23660_e18183: f64 = (1.0 + assign23660_e18182);
        let assign23660_e18184: f64 = (locals.var_t3 * assign23660_e18183);
        let assign23660_e18185: f64 = (1.0 + assign23660_e18184);
        (assign23660_e18185, ((locals.var_t3_dn0 * assign23660_e18183) + (locals.var_t3 * ((locals.var_t3_dn0 * assign23660_e18181) + (locals.var_t3 * ((locals.var_t3_dn0 * assign23660_e18179) + (locals.var_t3 * (locals.var_t3_dn0 * 0.148148111111111))))))), ((locals.var_t3_dn2 * assign23660_e18183) + (locals.var_t3 * ((locals.var_t3_dn2 * assign23660_e18181) + (locals.var_t3 * ((locals.var_t3_dn2 * assign23660_e18179) + (locals.var_t3 * (locals.var_t3_dn2 * 0.148148111111111))))))), ((locals.var_t3_dn4 * assign23660_e18183) + (locals.var_t3 * ((locals.var_t3_dn4 * assign23660_e18181) + (locals.var_t3 * ((locals.var_t3_dn4 * assign23660_e18179) + (locals.var_t3 * (locals.var_t3_dn4 * 0.148148111111111))))))), ((locals.var_t3_dn5 * assign23660_e18183) + (locals.var_t3 * ((locals.var_t3_dn5 * assign23660_e18181) + (locals.var_t3 * ((locals.var_t3_dn5 * assign23660_e18179) + (locals.var_t3 * (locals.var_t3_dn5 * 0.148148111111111))))))), ((locals.var_t3_dn6 * assign23660_e18183) + (locals.var_t3 * ((locals.var_t3_dn6 * assign23660_e18181) + (locals.var_t3 * ((locals.var_t3_dn6 * assign23660_e18179) + (locals.var_t3 * (locals.var_t3_dn6 * 0.148148111111111))))))), ((locals.var_t3_dn7 * assign23660_e18183) + (locals.var_t3 * ((locals.var_t3_dn7 * assign23660_e18181) + (locals.var_t3 * ((locals.var_t3_dn7 * assign23660_e18179) + (locals.var_t3 * (locals.var_t3_dn7 * 0.148148111111111))))))), ((locals.var_t3_dn8 * assign23660_e18183) + (locals.var_t3 * ((locals.var_t3_dn8 * assign23660_e18181) + (locals.var_t3 * ((locals.var_t3_dn8 * assign23660_e18179) + (locals.var_t3 * (locals.var_t3_dn8 * 0.148148111111111))))))), ((locals.var_t3_dn9 * assign23660_e18183) + (locals.var_t3 * ((locals.var_t3_dn9 * assign23660_e18181) + (locals.var_t3 * ((locals.var_t3_dn9 * assign23660_e18179) + (locals.var_t3 * (locals.var_t3_dn9 * 0.148148111111111))))))), ((locals.var_t3_dn10 * assign23660_e18183) + (locals.var_t3 * ((locals.var_t3_dn10 * assign23660_e18181) + (locals.var_t3 * ((locals.var_t3_dn10 * assign23660_e18179) + (locals.var_t3 * (locals.var_t3_dn10 * 0.148148111111111))))))), ((locals.var_t3_dn13 * assign23660_e18183) + (locals.var_t3 * ((locals.var_t3_dn13 * assign23660_e18181) + (locals.var_t3 * ((locals.var_t3_dn13 * assign23660_e18179) + (locals.var_t3 * (locals.var_t3_dn13 * 0.148148111111111))))))),)
    } else {
        (locals.var_dppg, locals.var_dppg_dn0, locals.var_dppg_dn2, locals.var_dppg_dn4, locals.var_dppg_dn5, locals.var_dppg_dn6, locals.var_dppg_dn7, locals.var_dppg_dn8, locals.var_dppg_dn9, locals.var_dppg_dn10, locals.var_dppg_dn13,)
    }
};
        locals.var_dppg = assign23660_e18187;
        locals.var_dppg_dn0 = assign23660_e18187_d_n0;
        locals.var_dppg_dn2 = assign23660_e18187_d_n2;
        locals.var_dppg_dn4 = assign23660_e18187_d_n4;
        locals.var_dppg_dn5 = assign23660_e18187_d_n5;
        locals.var_dppg_dn6 = assign23660_e18187_d_n6;
        locals.var_dppg_dn7 = assign23660_e18187_d_n7;
        locals.var_dppg_dn8 = assign23660_e18187_d_n8;
        locals.var_dppg_dn9 = assign23660_e18187_d_n9;
        locals.var_dppg_dn10 = assign23660_e18187_d_n10;
        locals.var_dppg_dn13 = assign23660_e18187_d_n13;
        locals.var_dppg_rv = 0.0;

        let (assign23670_e18204, assign23670_e18204_d_n0, assign23670_e18204_d_n2, assign23670_e18204_d_n4, assign23670_e18204_d_n5, assign23670_e18204_d_n6, assign23670_e18204_d_n7, assign23670_e18204_d_n8, assign23670_e18204_d_n9, assign23670_e18204_d_n10, assign23670_e18204_d_n13,) = {
    if (locals.var_guard431 != 0.0) {
        let assign23670_e18191: f64 = (locals.var_dppg - 1.0);
        let assign23670_e18194: f64 = (locals.var_dppg - 1.0);
        let assign23670_e18195: f64 = (assign23670_e18191 * assign23670_e18194);
        let assign23670_e18198: f64 = (4.0 * 0.05);
        let assign23670_e18200: f64 = (assign23670_e18198 * 0.05);
        let assign23670_e18201: f64 = (assign23670_e18195 + assign23670_e18200);
        let assign23670_e18202: f64 = (assign23670_e18201).sqrt();
        (assign23670_e18202, (((locals.var_dppg_dn0 * assign23670_e18194) + (assign23670_e18191 * locals.var_dppg_dn0)) / (2.0 * assign23670_e18202)), (((locals.var_dppg_dn2 * assign23670_e18194) + (assign23670_e18191 * locals.var_dppg_dn2)) / (2.0 * assign23670_e18202)), (((locals.var_dppg_dn4 * assign23670_e18194) + (assign23670_e18191 * locals.var_dppg_dn4)) / (2.0 * assign23670_e18202)), (((locals.var_dppg_dn5 * assign23670_e18194) + (assign23670_e18191 * locals.var_dppg_dn5)) / (2.0 * assign23670_e18202)), (((locals.var_dppg_dn6 * assign23670_e18194) + (assign23670_e18191 * locals.var_dppg_dn6)) / (2.0 * assign23670_e18202)), (((locals.var_dppg_dn7 * assign23670_e18194) + (assign23670_e18191 * locals.var_dppg_dn7)) / (2.0 * assign23670_e18202)), (((locals.var_dppg_dn8 * assign23670_e18194) + (assign23670_e18191 * locals.var_dppg_dn8)) / (2.0 * assign23670_e18202)), (((locals.var_dppg_dn9 * assign23670_e18194) + (assign23670_e18191 * locals.var_dppg_dn9)) / (2.0 * assign23670_e18202)), (((locals.var_dppg_dn10 * assign23670_e18194) + (assign23670_e18191 * locals.var_dppg_dn10)) / (2.0 * assign23670_e18202)), (((locals.var_dppg_dn13 * assign23670_e18194) + (assign23670_e18191 * locals.var_dppg_dn13)) / (2.0 * assign23670_e18202)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign23670_e18204;
        locals.var_tmf2_dn0 = assign23670_e18204_d_n0;
        locals.var_tmf2_dn2 = assign23670_e18204_d_n2;
        locals.var_tmf2_dn4 = assign23670_e18204_d_n4;
        locals.var_tmf2_dn5 = assign23670_e18204_d_n5;
        locals.var_tmf2_dn6 = assign23670_e18204_d_n6;
        locals.var_tmf2_dn7 = assign23670_e18204_d_n7;
        locals.var_tmf2_dn8 = assign23670_e18204_d_n8;
        locals.var_tmf2_dn9 = assign23670_e18204_d_n9;
        locals.var_tmf2_dn10 = assign23670_e18204_d_n10;
        locals.var_tmf2_dn13 = assign23670_e18204_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign23680_e18216, assign23680_e18216_d_n0, assign23680_e18216_d_n2, assign23680_e18216_d_n4, assign23680_e18216_d_n5, assign23680_e18216_d_n6, assign23680_e18216_d_n7, assign23680_e18216_d_n8, assign23680_e18216_d_n9, assign23680_e18216_d_n10, assign23680_e18216_d_n13,) = {
    if (locals.var_guard431 != 0.0) {
        let assign23680_e18210: f64 = (locals.var_dppg - 1.0);
        let assign23680_e18212: f64 = (assign23680_e18210 / locals.var_tmf2);
        let assign23680_e18213: f64 = (1.0 + assign23680_e18212);
        let assign23680_e18214: f64 = (0.5 * assign23680_e18213);
        (assign23680_e18214, (0.5 * (((locals.var_dppg_dn0 * locals.var_tmf2) - (assign23680_e18210 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_dppg_dn2 * locals.var_tmf2) - (assign23680_e18210 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_dppg_dn4 * locals.var_tmf2) - (assign23680_e18210 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_dppg_dn5 * locals.var_tmf2) - (assign23680_e18210 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_dppg_dn6 * locals.var_tmf2) - (assign23680_e18210 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_dppg_dn7 * locals.var_tmf2) - (assign23680_e18210 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_dppg_dn8 * locals.var_tmf2) - (assign23680_e18210 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_dppg_dn9 * locals.var_tmf2) - (assign23680_e18210 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_dppg_dn10 * locals.var_tmf2) - (assign23680_e18210 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_dppg_dn13 * locals.var_tmf2) - (assign23680_e18210 * locals.var_tmf2_dn13)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn13,)
    }
};
        locals.var_t6 = assign23680_e18216;
        locals.var_t6_dn0 = assign23680_e18216_d_n0;
        locals.var_t6_dn2 = assign23680_e18216_d_n2;
        locals.var_t6_dn4 = assign23680_e18216_d_n4;
        locals.var_t6_dn5 = assign23680_e18216_d_n5;
        locals.var_t6_dn6 = assign23680_e18216_d_n6;
        locals.var_t6_dn7 = assign23680_e18216_d_n7;
        locals.var_t6_dn8 = assign23680_e18216_d_n8;
        locals.var_t6_dn9 = assign23680_e18216_d_n9;
        locals.var_t6_dn10 = assign23680_e18216_d_n10;
        locals.var_t6_dn13 = assign23680_e18216_d_n13;
        locals.var_t6_rv = 0.0;

        let (assign23690_e18226, assign23690_e18226_d_n0, assign23690_e18226_d_n2, assign23690_e18226_d_n4, assign23690_e18226_d_n5, assign23690_e18226_d_n6, assign23690_e18226_d_n7, assign23690_e18226_d_n8, assign23690_e18226_d_n9, assign23690_e18226_d_n10, assign23690_e18226_d_n13,) = {
    if (locals.var_guard431 != 0.0) {
        let assign23690_e18221: f64 = (locals.var_dppg - 1.0);
        let assign23690_e18223: f64 = (assign23690_e18221 + locals.var_tmf2);
        let assign23690_e18224: f64 = (0.5 * assign23690_e18223);
        (assign23690_e18224, (0.5 * (locals.var_dppg_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_dppg_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_dppg_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_dppg_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_dppg_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_dppg_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_dppg_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_dppg_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_dppg_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_dppg_dn13 + locals.var_tmf2_dn13)),)
    } else {
        (locals.var_dppg, locals.var_dppg_dn0, locals.var_dppg_dn2, locals.var_dppg_dn4, locals.var_dppg_dn5, locals.var_dppg_dn6, locals.var_dppg_dn7, locals.var_dppg_dn8, locals.var_dppg_dn9, locals.var_dppg_dn10, locals.var_dppg_dn13,)
    }
};
        locals.var_dppg = assign23690_e18226;
        locals.var_dppg_dn0 = assign23690_e18226_d_n0;
        locals.var_dppg_dn2 = assign23690_e18226_d_n2;
        locals.var_dppg_dn4 = assign23690_e18226_d_n4;
        locals.var_dppg_dn5 = assign23690_e18226_d_n5;
        locals.var_dppg_dn6 = assign23690_e18226_d_n6;
        locals.var_dppg_dn7 = assign23690_e18226_d_n7;
        locals.var_dppg_dn8 = assign23690_e18226_d_n8;
        locals.var_dppg_dn9 = assign23690_e18226_d_n9;
        locals.var_dppg_dn10 = assign23690_e18226_d_n10;
        locals.var_dppg_dn13 = assign23690_e18226_d_n13;
        locals.var_dppg_rv = 0.0;

        let assign23700_e18229: f64 = if locals.var_dppg < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard434 = assign23700_e18229;
        locals.var_guard434_rv = 0.0;

        let (assign23710_e18235, assign23710_e18235_d_n0, assign23710_e18235_d_n2, assign23710_e18235_d_n4, assign23710_e18235_d_n5, assign23710_e18235_d_n6, assign23710_e18235_d_n7, assign23710_e18235_d_n8, assign23710_e18235_d_n9, assign23710_e18235_d_n10, assign23710_e18235_d_n13,) = {
    if ((locals.var_guard431 != 0.0) && (locals.var_guard434 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dppg, locals.var_dppg_dn0, locals.var_dppg_dn2, locals.var_dppg_dn4, locals.var_dppg_dn5, locals.var_dppg_dn6, locals.var_dppg_dn7, locals.var_dppg_dn8, locals.var_dppg_dn9, locals.var_dppg_dn10, locals.var_dppg_dn13,)
    }
};
        locals.var_dppg = assign23710_e18235;
        locals.var_dppg_dn0 = assign23710_e18235_d_n0;
        locals.var_dppg_dn2 = assign23710_e18235_d_n2;
        locals.var_dppg_dn4 = assign23710_e18235_d_n4;
        locals.var_dppg_dn5 = assign23710_e18235_d_n5;
        locals.var_dppg_dn6 = assign23710_e18235_d_n6;
        locals.var_dppg_dn7 = assign23710_e18235_d_n7;
        locals.var_dppg_dn8 = assign23710_e18235_d_n8;
        locals.var_dppg_dn9 = assign23710_e18235_d_n9;
        locals.var_dppg_dn10 = assign23710_e18235_d_n10;
        locals.var_dppg_dn13 = assign23710_e18235_d_n13;
        locals.var_dppg_rv = 0.0;

        let (assign23720_e18241, assign23720_e18241_d_n0, assign23720_e18241_d_n2, assign23720_e18241_d_n4, assign23720_e18241_d_n5, assign23720_e18241_d_n6, assign23720_e18241_d_n7, assign23720_e18241_d_n8, assign23720_e18241_d_n9, assign23720_e18241_d_n10, assign23720_e18241_d_n13,) = {
    if ((locals.var_guard431 != 0.0) && (locals.var_guard434 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn13,)
    }
};
        locals.var_t6 = assign23720_e18241;
        locals.var_t6_dn0 = assign23720_e18241_d_n0;
        locals.var_t6_dn2 = assign23720_e18241_d_n2;
        locals.var_t6_dn4 = assign23720_e18241_d_n4;
        locals.var_t6_dn5 = assign23720_e18241_d_n5;
        locals.var_t6_dn6 = assign23720_e18241_d_n6;
        locals.var_t6_dn7 = assign23720_e18241_d_n7;
        locals.var_t6_dn8 = assign23720_e18241_d_n8;
        locals.var_t6_dn9 = assign23720_e18241_d_n9;
        locals.var_t6_dn10 = assign23720_e18241_d_n10;
        locals.var_t6_dn13 = assign23720_e18241_d_n13;
        locals.var_t6_rv = 0.0;

        let (assign23730_e18247, assign23730_e18247_d_n0, assign23730_e18247_d_n2, assign23730_e18247_d_n4, assign23730_e18247_d_n5, assign23730_e18247_d_n6, assign23730_e18247_d_n7, assign23730_e18247_d_n8, assign23730_e18247_d_n9, assign23730_e18247_d_n10, assign23730_e18247_d_n13,) = {
    if (locals.var_guard431 != 0.0) {
        let assign23730_e18245: f64 = (locals.var_dppg * locals.var_t0);
        (assign23730_e18245, ((locals.var_dppg_dn0 * locals.var_t0) + (locals.var_dppg * locals.var_t0_dn0)), ((locals.var_dppg_dn2 * locals.var_t0) + (locals.var_dppg * locals.var_t0_dn2)), ((locals.var_dppg_dn4 * locals.var_t0) + (locals.var_dppg * locals.var_t0_dn4)), ((locals.var_dppg_dn5 * locals.var_t0) + (locals.var_dppg * locals.var_t0_dn5)), ((locals.var_dppg_dn6 * locals.var_t0) + (locals.var_dppg * locals.var_t0_dn6)), ((locals.var_dppg_dn7 * locals.var_t0) + (locals.var_dppg * locals.var_t0_dn7)), ((locals.var_dppg_dn8 * locals.var_t0) + (locals.var_dppg * locals.var_t0_dn8)), ((locals.var_dppg_dn9 * locals.var_t0) + (locals.var_dppg * locals.var_t0_dn9)), ((locals.var_dppg_dn10 * locals.var_t0) + (locals.var_dppg * locals.var_t0_dn10)), ((locals.var_dppg_dn13 * locals.var_t0) + (locals.var_dppg * locals.var_t0_dn13)),)
    } else {
        (locals.var_dppg, locals.var_dppg_dn0, locals.var_dppg_dn2, locals.var_dppg_dn4, locals.var_dppg_dn5, locals.var_dppg_dn6, locals.var_dppg_dn7, locals.var_dppg_dn8, locals.var_dppg_dn9, locals.var_dppg_dn10, locals.var_dppg_dn13,)
    }
};
        locals.var_dppg = assign23730_e18247;
        locals.var_dppg_dn0 = assign23730_e18247_d_n0;
        locals.var_dppg_dn2 = assign23730_e18247_d_n2;
        locals.var_dppg_dn4 = assign23730_e18247_d_n4;
        locals.var_dppg_dn5 = assign23730_e18247_d_n5;
        locals.var_dppg_dn6 = assign23730_e18247_d_n6;
        locals.var_dppg_dn7 = assign23730_e18247_d_n7;
        locals.var_dppg_dn8 = assign23730_e18247_d_n8;
        locals.var_dppg_dn9 = assign23730_e18247_d_n9;
        locals.var_dppg_dn10 = assign23730_e18247_d_n10;
        locals.var_dppg_dn13 = assign23730_e18247_d_n13;
        locals.var_dppg_rv = 0.0;

        let (assign23740_e18255, assign23740_e18255_d_n0, assign23740_e18255_d_n2, assign23740_e18255_d_n4, assign23740_e18255_d_n5, assign23740_e18255_d_n6, assign23740_e18255_d_n7, assign23740_e18255_d_n8, assign23740_e18255_d_n9, assign23740_e18255_d_n10, assign23740_e18255_d_n13,) = {
    if (locals.var_guard431 != 0.0) {
        let assign23740_e18251: f64 = (1.0 - locals.var_dppg);
        let assign23740_e18253: f64 = (assign23740_e18251 - 0.05);
        (assign23740_e18253, (-locals.var_dppg_dn0), (-locals.var_dppg_dn2), (-locals.var_dppg_dn4), (-locals.var_dppg_dn5), (-locals.var_dppg_dn6), (-locals.var_dppg_dn7), (-locals.var_dppg_dn8), (-locals.var_dppg_dn9), (-locals.var_dppg_dn10), (-locals.var_dppg_dn13),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13,)
    }
};
        locals.var_tmf1 = assign23740_e18255;
        locals.var_tmf1_dn0 = assign23740_e18255_d_n0;
        locals.var_tmf1_dn2 = assign23740_e18255_d_n2;
        locals.var_tmf1_dn4 = assign23740_e18255_d_n4;
        locals.var_tmf1_dn5 = assign23740_e18255_d_n5;
        locals.var_tmf1_dn6 = assign23740_e18255_d_n6;
        locals.var_tmf1_dn7 = assign23740_e18255_d_n7;
        locals.var_tmf1_dn8 = assign23740_e18255_d_n8;
        locals.var_tmf1_dn9 = assign23740_e18255_d_n9;
        locals.var_tmf1_dn10 = assign23740_e18255_d_n10;
        locals.var_tmf1_dn13 = assign23740_e18255_d_n13;
        locals.var_tmf1_rv = 0.0;

        let (assign23750_e18263, assign23750_e18263_d_n0, assign23750_e18263_d_n2, assign23750_e18263_d_n4, assign23750_e18263_d_n5, assign23750_e18263_d_n6, assign23750_e18263_d_n7, assign23750_e18263_d_n8, assign23750_e18263_d_n9, assign23750_e18263_d_n10, assign23750_e18263_d_n13,) = {
    if (locals.var_guard431 != 0.0) {
        let assign23750_e18259: f64 = 4.0;
        let assign23750_e18261: f64 = (assign23750_e18259 * 0.05);
        (assign23750_e18261, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign23750_e18263;
        locals.var_tmf2_dn0 = assign23750_e18263_d_n0;
        locals.var_tmf2_dn2 = assign23750_e18263_d_n2;
        locals.var_tmf2_dn4 = assign23750_e18263_d_n4;
        locals.var_tmf2_dn5 = assign23750_e18263_d_n5;
        locals.var_tmf2_dn6 = assign23750_e18263_d_n6;
        locals.var_tmf2_dn7 = assign23750_e18263_d_n7;
        locals.var_tmf2_dn8 = assign23750_e18263_d_n8;
        locals.var_tmf2_dn9 = assign23750_e18263_d_n9;
        locals.var_tmf2_dn10 = assign23750_e18263_d_n10;
        locals.var_tmf2_dn13 = assign23750_e18263_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign23760_e18273, assign23760_e18273_d_n0, assign23760_e18273_d_n2, assign23760_e18273_d_n4, assign23760_e18273_d_n5, assign23760_e18273_d_n6, assign23760_e18273_d_n7, assign23760_e18273_d_n8, assign23760_e18273_d_n9, assign23760_e18273_d_n10, assign23760_e18273_d_n13,) = {
    if (locals.var_guard431 != 0.0) {
        let (assign23760_e18271, assign23760_e18271_d_n0, assign23760_e18271_d_n2, assign23760_e18271_d_n4, assign23760_e18271_d_n5, assign23760_e18271_d_n6, assign23760_e18271_d_n7, assign23760_e18271_d_n8, assign23760_e18271_d_n9, assign23760_e18271_d_n10, assign23760_e18271_d_n13,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
            } else {
                let assign23760_e18270: f64 = (-locals.var_tmf2);
                (assign23760_e18270, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn13),)
            }
        };
        (assign23760_e18271, assign23760_e18271_d_n0, assign23760_e18271_d_n2, assign23760_e18271_d_n4, assign23760_e18271_d_n5, assign23760_e18271_d_n6, assign23760_e18271_d_n7, assign23760_e18271_d_n8, assign23760_e18271_d_n9, assign23760_e18271_d_n10, assign23760_e18271_d_n13,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign23760_e18273;
        locals.var_tmf2_dn0 = assign23760_e18273_d_n0;
        locals.var_tmf2_dn2 = assign23760_e18273_d_n2;
        locals.var_tmf2_dn4 = assign23760_e18273_d_n4;
        locals.var_tmf2_dn5 = assign23760_e18273_d_n5;
        locals.var_tmf2_dn6 = assign23760_e18273_d_n6;
        locals.var_tmf2_dn7 = assign23760_e18273_d_n7;
        locals.var_tmf2_dn8 = assign23760_e18273_d_n8;
        locals.var_tmf2_dn9 = assign23760_e18273_d_n9;
        locals.var_tmf2_dn10 = assign23760_e18273_d_n10;
        locals.var_tmf2_dn13 = assign23760_e18273_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign23770_e18282, assign23770_e18282_d_n0, assign23770_e18282_d_n2, assign23770_e18282_d_n4, assign23770_e18282_d_n5, assign23770_e18282_d_n6, assign23770_e18282_d_n7, assign23770_e18282_d_n8, assign23770_e18282_d_n9, assign23770_e18282_d_n10, assign23770_e18282_d_n13,) = {
    if (locals.var_guard431 != 0.0) {
        let assign23770_e18277: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign23770_e18279: f64 = (assign23770_e18277 + locals.var_tmf2);
        let assign23770_e18280: f64 = (assign23770_e18279).sqrt();
        (assign23770_e18280, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign23770_e18280)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign23770_e18280)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign23770_e18280)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign23770_e18280)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign23770_e18280)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign23770_e18280)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign23770_e18280)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign23770_e18280)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign23770_e18280)), ((((locals.var_tmf1_dn13 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn13)) + locals.var_tmf2_dn13) / (2.0 * assign23770_e18280)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign23770_e18282;
        locals.var_tmf2_dn0 = assign23770_e18282_d_n0;
        locals.var_tmf2_dn2 = assign23770_e18282_d_n2;
        locals.var_tmf2_dn4 = assign23770_e18282_d_n4;
        locals.var_tmf2_dn5 = assign23770_e18282_d_n5;
        locals.var_tmf2_dn6 = assign23770_e18282_d_n6;
        locals.var_tmf2_dn7 = assign23770_e18282_d_n7;
        locals.var_tmf2_dn8 = assign23770_e18282_d_n8;
        locals.var_tmf2_dn9 = assign23770_e18282_d_n9;
        locals.var_tmf2_dn10 = assign23770_e18282_d_n10;
        locals.var_tmf2_dn13 = assign23770_e18282_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign23780_e18292, assign23780_e18292_d_n0, assign23780_e18292_d_n2, assign23780_e18292_d_n4, assign23780_e18292_d_n5, assign23780_e18292_d_n6, assign23780_e18292_d_n7, assign23780_e18292_d_n8, assign23780_e18292_d_n9, assign23780_e18292_d_n10, assign23780_e18292_d_n13,) = {
    if (locals.var_guard431 != 0.0) {
        let assign23780_e18288: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign23780_e18289: f64 = (1.0 + assign23780_e18288);
        let assign23780_e18290: f64 = (0.5 * assign23780_e18289);
        (assign23780_e18290, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn13 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn13)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn13,)
    }
};
        locals.var_t9 = assign23780_e18292;
        locals.var_t9_dn0 = assign23780_e18292_d_n0;
        locals.var_t9_dn2 = assign23780_e18292_d_n2;
        locals.var_t9_dn4 = assign23780_e18292_d_n4;
        locals.var_t9_dn5 = assign23780_e18292_d_n5;
        locals.var_t9_dn6 = assign23780_e18292_d_n6;
        locals.var_t9_dn7 = assign23780_e18292_d_n7;
        locals.var_t9_dn8 = assign23780_e18292_d_n8;
        locals.var_t9_dn9 = assign23780_e18292_d_n9;
        locals.var_t9_dn10 = assign23780_e18292_d_n10;
        locals.var_t9_dn13 = assign23780_e18292_d_n13;
        locals.var_t9_rv = 0.0;

        let (assign23790_e18302, assign23790_e18302_d_n0, assign23790_e18302_d_n2, assign23790_e18302_d_n4, assign23790_e18302_d_n5, assign23790_e18302_d_n6, assign23790_e18302_d_n7, assign23790_e18302_d_n8, assign23790_e18302_d_n9, assign23790_e18302_d_n10, assign23790_e18302_d_n13,) = {
    if (locals.var_guard431 != 0.0) {
        let assign23790_e18298: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign23790_e18299: f64 = (0.5 * assign23790_e18298);
        let assign23790_e18300: f64 = (1.0 - assign23790_e18299);
        (assign23790_e18300, (-(0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (-(0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), (-(0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4))), (-(0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5))), (-(0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), (-(0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))), (-(0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8))), (-(0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9))), (-(0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), (-(0.5 * (locals.var_tmf1_dn13 + locals.var_tmf2_dn13))),)
    } else {
        (locals.var_dppg, locals.var_dppg_dn0, locals.var_dppg_dn2, locals.var_dppg_dn4, locals.var_dppg_dn5, locals.var_dppg_dn6, locals.var_dppg_dn7, locals.var_dppg_dn8, locals.var_dppg_dn9, locals.var_dppg_dn10, locals.var_dppg_dn13,)
    }
};
        locals.var_dppg = assign23790_e18302;
        locals.var_dppg_dn0 = assign23790_e18302_d_n0;
        locals.var_dppg_dn2 = assign23790_e18302_d_n2;
        locals.var_dppg_dn4 = assign23790_e18302_d_n4;
        locals.var_dppg_dn5 = assign23790_e18302_d_n5;
        locals.var_dppg_dn6 = assign23790_e18302_d_n6;
        locals.var_dppg_dn7 = assign23790_e18302_d_n7;
        locals.var_dppg_dn8 = assign23790_e18302_d_n8;
        locals.var_dppg_dn9 = assign23790_e18302_d_n9;
        locals.var_dppg_dn10 = assign23790_e18302_d_n10;
        locals.var_dppg_dn13 = assign23790_e18302_d_n13;
        locals.var_dppg_rv = 0.0;

        let assign23800_e18305: f64 = if locals.var_vbs > locals.var_vbs_bnd_local { 1.0 } else { 0.0 };
        locals.var_guard441 = assign23800_e18305;
        locals.var_guard441_rv = 0.0;

        let (assign23810_e18313, assign23810_e18313_d_n0, assign23810_e18313_d_n2, assign23810_e18313_d_n4, assign23810_e18313_d_n5, assign23810_e18313_d_n6, assign23810_e18313_d_n7, assign23810_e18313_d_n8, assign23810_e18313_d_n9, assign23810_e18313_d_n10, assign23810_e18313_d_n13,) = {
    if ((p.p37 != 0.0) && (locals.var_guard441 != 0.0)) {
        let assign23810_e18311: f64 = (locals.var_vbs - locals.var_vbs_bnd_local);
        (assign23810_e18311, (-locals.var_vbs_bnd_local_dn0), (-locals.var_vbs_bnd_local_dn2), (-locals.var_vbs_bnd_local_dn4), (locals.var_vbs_dn5 - locals.var_vbs_bnd_local_dn5), (-locals.var_vbs_bnd_local_dn6), (locals.var_vbs_dn7 - locals.var_vbs_bnd_local_dn7), (locals.var_vbs_dn8 - locals.var_vbs_bnd_local_dn8), (-locals.var_vbs_bnd_local_dn9), (-locals.var_vbs_bnd_local_dn10), (-locals.var_vbs_bnd_local_dn13),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign23810_e18313;
        locals.var_t1_dn0 = assign23810_e18313_d_n0;
        locals.var_t1_dn2 = assign23810_e18313_d_n2;
        locals.var_t1_dn4 = assign23810_e18313_d_n4;
        locals.var_t1_dn5 = assign23810_e18313_d_n5;
        locals.var_t1_dn6 = assign23810_e18313_d_n6;
        locals.var_t1_dn7 = assign23810_e18313_d_n7;
        locals.var_t1_dn8 = assign23810_e18313_d_n8;
        locals.var_t1_dn9 = assign23810_e18313_d_n9;
        locals.var_t1_dn10 = assign23810_e18313_d_n10;
        locals.var_t1_dn13 = assign23810_e18313_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign23820_e18321, assign23820_e18321_d_n0, assign23820_e18321_d_n2, assign23820_e18321_d_n4, assign23820_e18321_d_n5, assign23820_e18321_d_n6, assign23820_e18321_d_n7, assign23820_e18321_d_n8, assign23820_e18321_d_n9, assign23820_e18321_d_n10, assign23820_e18321_d_n13,) = {
    if ((p.p37 != 0.0) && (locals.var_guard441 != 0.0)) {
        let assign23820_e18319: f64 = (locals.var_vbs_max_local - locals.var_vbs_bnd_local);
        (assign23820_e18319, (locals.var_vbs_max_local_dn0 - locals.var_vbs_bnd_local_dn0), (locals.var_vbs_max_local_dn2 - locals.var_vbs_bnd_local_dn2), (locals.var_vbs_max_local_dn4 - locals.var_vbs_bnd_local_dn4), (locals.var_vbs_max_local_dn5 - locals.var_vbs_bnd_local_dn5), (locals.var_vbs_max_local_dn6 - locals.var_vbs_bnd_local_dn6), (locals.var_vbs_max_local_dn7 - locals.var_vbs_bnd_local_dn7), (locals.var_vbs_max_local_dn8 - locals.var_vbs_bnd_local_dn8), (locals.var_vbs_max_local_dn9 - locals.var_vbs_bnd_local_dn9), (locals.var_vbs_max_local_dn10 - locals.var_vbs_bnd_local_dn10), (locals.var_vbs_max_local_dn13 - locals.var_vbs_bnd_local_dn13),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign23820_e18321;
        locals.var_t2_dn0 = assign23820_e18321_d_n0;
        locals.var_t2_dn2 = assign23820_e18321_d_n2;
        locals.var_t2_dn4 = assign23820_e18321_d_n4;
        locals.var_t2_dn5 = assign23820_e18321_d_n5;
        locals.var_t2_dn6 = assign23820_e18321_d_n6;
        locals.var_t2_dn7 = assign23820_e18321_d_n7;
        locals.var_t2_dn8 = assign23820_e18321_d_n8;
        locals.var_t2_dn9 = assign23820_e18321_d_n9;
        locals.var_t2_dn10 = assign23820_e18321_d_n10;
        locals.var_t2_dn13 = assign23820_e18321_d_n13;
        locals.var_t2_rv = 0.0;

        let (assign23830_e18329, assign23830_e18329_d_n0, assign23830_e18329_d_n2, assign23830_e18329_d_n4, assign23830_e18329_d_n5, assign23830_e18329_d_n6, assign23830_e18329_d_n7, assign23830_e18329_d_n8, assign23830_e18329_d_n9, assign23830_e18329_d_n10, assign23830_e18329_d_n13,) = {
    if ((p.p37 != 0.0) && (locals.var_guard441 != 0.0)) {
        let assign23830_e18327: f64 = (locals.var_t1 / locals.var_t2);
        (assign23830_e18327, (((locals.var_t1_dn0 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn0)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn2 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn2)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn4 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn4)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn5 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn5)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn6 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn6)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn7 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn7)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn8 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn8)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn9 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn9)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn10 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn10)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn13 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn13)) / (locals.var_t2 * locals.var_t2)),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13,)
    }
};
        locals.var_tmf1 = assign23830_e18329;
        locals.var_tmf1_dn0 = assign23830_e18329_d_n0;
        locals.var_tmf1_dn2 = assign23830_e18329_d_n2;
        locals.var_tmf1_dn4 = assign23830_e18329_d_n4;
        locals.var_tmf1_dn5 = assign23830_e18329_d_n5;
        locals.var_tmf1_dn6 = assign23830_e18329_d_n6;
        locals.var_tmf1_dn7 = assign23830_e18329_d_n7;
        locals.var_tmf1_dn8 = assign23830_e18329_d_n8;
        locals.var_tmf1_dn9 = assign23830_e18329_d_n9;
        locals.var_tmf1_dn10 = assign23830_e18329_d_n10;
        locals.var_tmf1_dn13 = assign23830_e18329_d_n13;
        locals.var_tmf1_rv = 0.0;

        let (assign23840_e18337, assign23840_e18337_d_n0, assign23840_e18337_d_n2, assign23840_e18337_d_n4, assign23840_e18337_d_n5, assign23840_e18337_d_n6, assign23840_e18337_d_n7, assign23840_e18337_d_n8, assign23840_e18337_d_n9, assign23840_e18337_d_n10, assign23840_e18337_d_n13,) = {
    if ((p.p37 != 0.0) && (locals.var_guard441 != 0.0)) {
        let assign23840_e18335: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign23840_e18335, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)), ((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)), ((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn13 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn13)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign23840_e18337;
        locals.var_tmf2_dn0 = assign23840_e18337_d_n0;
        locals.var_tmf2_dn2 = assign23840_e18337_d_n2;
        locals.var_tmf2_dn4 = assign23840_e18337_d_n4;
        locals.var_tmf2_dn5 = assign23840_e18337_d_n5;
        locals.var_tmf2_dn6 = assign23840_e18337_d_n6;
        locals.var_tmf2_dn7 = assign23840_e18337_d_n7;
        locals.var_tmf2_dn8 = assign23840_e18337_d_n8;
        locals.var_tmf2_dn9 = assign23840_e18337_d_n9;
        locals.var_tmf2_dn10 = assign23840_e18337_d_n10;
        locals.var_tmf2_dn13 = assign23840_e18337_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign23850_e18345, assign23850_e18345_d_n0, assign23850_e18345_d_n2, assign23850_e18345_d_n4, assign23850_e18345_d_n5, assign23850_e18345_d_n6, assign23850_e18345_d_n7, assign23850_e18345_d_n8, assign23850_e18345_d_n9, assign23850_e18345_d_n10, assign23850_e18345_d_n13,) = {
    if ((p.p37 != 0.0) && (locals.var_guard441 != 0.0)) {
        let assign23850_e18343: f64 = (locals.var_tmf2 * locals.var_tmf1);
        (assign23850_e18343, ((locals.var_tmf2_dn0 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn0)), ((locals.var_tmf2_dn2 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn2)), ((locals.var_tmf2_dn4 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn4)), ((locals.var_tmf2_dn5 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn5)), ((locals.var_tmf2_dn6 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn6)), ((locals.var_tmf2_dn7 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn7)), ((locals.var_tmf2_dn8 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn8)), ((locals.var_tmf2_dn9 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn9)), ((locals.var_tmf2_dn10 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn10)), ((locals.var_tmf2_dn13 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn13)),)
    } else {
        (locals.var_tmf3, locals.var_tmf3_dn0, locals.var_tmf3_dn2, locals.var_tmf3_dn4, locals.var_tmf3_dn5, locals.var_tmf3_dn6, locals.var_tmf3_dn7, locals.var_tmf3_dn8, locals.var_tmf3_dn9, locals.var_tmf3_dn10, locals.var_tmf3_dn13,)
    }
};
        locals.var_tmf3 = assign23850_e18345;
        locals.var_tmf3_dn0 = assign23850_e18345_d_n0;
        locals.var_tmf3_dn2 = assign23850_e18345_d_n2;
        locals.var_tmf3_dn4 = assign23850_e18345_d_n4;
        locals.var_tmf3_dn5 = assign23850_e18345_d_n5;
        locals.var_tmf3_dn6 = assign23850_e18345_d_n6;
        locals.var_tmf3_dn7 = assign23850_e18345_d_n7;
        locals.var_tmf3_dn8 = assign23850_e18345_d_n8;
        locals.var_tmf3_dn9 = assign23850_e18345_d_n9;
        locals.var_tmf3_dn10 = assign23850_e18345_d_n10;
        locals.var_tmf3_dn13 = assign23850_e18345_d_n13;
        locals.var_tmf3_rv = 0.0;

        let (assign23860_e18353, assign23860_e18353_d_n0, assign23860_e18353_d_n2, assign23860_e18353_d_n4, assign23860_e18353_d_n5, assign23860_e18353_d_n6, assign23860_e18353_d_n7, assign23860_e18353_d_n8, assign23860_e18353_d_n9, assign23860_e18353_d_n10, assign23860_e18353_d_n13,) = {
    if ((p.p37 != 0.0) && (locals.var_guard441 != 0.0)) {
        let assign23860_e18351: f64 = (locals.var_tmf2 * locals.var_tmf2);
        (assign23860_e18351, ((locals.var_tmf2_dn0 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn0)), ((locals.var_tmf2_dn2 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn2)), ((locals.var_tmf2_dn4 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn4)), ((locals.var_tmf2_dn5 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn5)), ((locals.var_tmf2_dn6 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn6)), ((locals.var_tmf2_dn7 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn7)), ((locals.var_tmf2_dn8 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn8)), ((locals.var_tmf2_dn9 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn9)), ((locals.var_tmf2_dn10 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn10)), ((locals.var_tmf2_dn13 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn13)),)
    } else {
        (locals.var_tmf4, locals.var_tmf4_dn0, locals.var_tmf4_dn2, locals.var_tmf4_dn4, locals.var_tmf4_dn5, locals.var_tmf4_dn6, locals.var_tmf4_dn7, locals.var_tmf4_dn8, locals.var_tmf4_dn9, locals.var_tmf4_dn10, locals.var_tmf4_dn13,)
    }
};
        locals.var_tmf4 = assign23860_e18353;
        locals.var_tmf4_dn0 = assign23860_e18353_d_n0;
        locals.var_tmf4_dn2 = assign23860_e18353_d_n2;
        locals.var_tmf4_dn4 = assign23860_e18353_d_n4;
        locals.var_tmf4_dn5 = assign23860_e18353_d_n5;
        locals.var_tmf4_dn6 = assign23860_e18353_d_n6;
        locals.var_tmf4_dn7 = assign23860_e18353_d_n7;
        locals.var_tmf4_dn8 = assign23860_e18353_d_n8;
        locals.var_tmf4_dn9 = assign23860_e18353_d_n9;
        locals.var_tmf4_dn10 = assign23860_e18353_d_n10;
        locals.var_tmf4_dn13 = assign23860_e18353_d_n13;
        locals.var_tmf4_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_63(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign23870_e18369, assign23870_e18369_d_n0, assign23870_e18369_d_n2, assign23870_e18369_d_n4, assign23870_e18369_d_n5, assign23870_e18369_d_n6, assign23870_e18369_d_n7, assign23870_e18369_d_n8, assign23870_e18369_d_n9, assign23870_e18369_d_n10, assign23870_e18369_d_n13,) = {
    if ((p.p37 != 0.0) && (locals.var_guard441 != 0.0)) {
        let assign23870_e18360: f64 = (1.0 + locals.var_tmf1);
        let assign23870_e18362: f64 = (assign23870_e18360 + locals.var_tmf2);
        let assign23870_e18364: f64 = (assign23870_e18362 + locals.var_tmf3);
        let assign23870_e18366: f64 = (assign23870_e18364 + locals.var_tmf4);
        let assign23870_e18367: f64 = (1.0 / assign23870_e18366);
        (assign23870_e18367, (-((((locals.var_tmf1_dn0 + locals.var_tmf2_dn0) + locals.var_tmf3_dn0) + locals.var_tmf4_dn0) / (assign23870_e18366 * assign23870_e18366))), (-((((locals.var_tmf1_dn2 + locals.var_tmf2_dn2) + locals.var_tmf3_dn2) + locals.var_tmf4_dn2) / (assign23870_e18366 * assign23870_e18366))), (-((((locals.var_tmf1_dn4 + locals.var_tmf2_dn4) + locals.var_tmf3_dn4) + locals.var_tmf4_dn4) / (assign23870_e18366 * assign23870_e18366))), (-((((locals.var_tmf1_dn5 + locals.var_tmf2_dn5) + locals.var_tmf3_dn5) + locals.var_tmf4_dn5) / (assign23870_e18366 * assign23870_e18366))), (-((((locals.var_tmf1_dn6 + locals.var_tmf2_dn6) + locals.var_tmf3_dn6) + locals.var_tmf4_dn6) / (assign23870_e18366 * assign23870_e18366))), (-((((locals.var_tmf1_dn7 + locals.var_tmf2_dn7) + locals.var_tmf3_dn7) + locals.var_tmf4_dn7) / (assign23870_e18366 * assign23870_e18366))), (-((((locals.var_tmf1_dn8 + locals.var_tmf2_dn8) + locals.var_tmf3_dn8) + locals.var_tmf4_dn8) / (assign23870_e18366 * assign23870_e18366))), (-((((locals.var_tmf1_dn9 + locals.var_tmf2_dn9) + locals.var_tmf3_dn9) + locals.var_tmf4_dn9) / (assign23870_e18366 * assign23870_e18366))), (-((((locals.var_tmf1_dn10 + locals.var_tmf2_dn10) + locals.var_tmf3_dn10) + locals.var_tmf4_dn10) / (assign23870_e18366 * assign23870_e18366))), (-((((locals.var_tmf1_dn13 + locals.var_tmf2_dn13) + locals.var_tmf3_dn13) + locals.var_tmf4_dn13) / (assign23870_e18366 * assign23870_e18366))),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn13,)
    }
};
        locals.var_tmf0 = assign23870_e18369;
        locals.var_tmf0_dn0 = assign23870_e18369_d_n0;
        locals.var_tmf0_dn2 = assign23870_e18369_d_n2;
        locals.var_tmf0_dn4 = assign23870_e18369_d_n4;
        locals.var_tmf0_dn5 = assign23870_e18369_d_n5;
        locals.var_tmf0_dn6 = assign23870_e18369_d_n6;
        locals.var_tmf0_dn7 = assign23870_e18369_d_n7;
        locals.var_tmf0_dn8 = assign23870_e18369_d_n8;
        locals.var_tmf0_dn9 = assign23870_e18369_d_n9;
        locals.var_tmf0_dn10 = assign23870_e18369_d_n10;
        locals.var_tmf0_dn13 = assign23870_e18369_d_n13;
        locals.var_tmf0_rv = 0.0;

        let (assign23880_e18392, assign23880_e18392_d_n0, assign23880_e18392_d_n2, assign23880_e18392_d_n4, assign23880_e18392_d_n5, assign23880_e18392_d_n6, assign23880_e18392_d_n7, assign23880_e18392_d_n8, assign23880_e18392_d_n9, assign23880_e18392_d_n10, assign23880_e18392_d_n13,) = {
    if ((p.p37 != 0.0) && (locals.var_guard441 != 0.0)) {
        let assign23880_e18376: f64 = (2.0 * locals.var_tmf1);
        let assign23880_e18377: f64 = (1.0 + assign23880_e18376);
        let assign23880_e18380: f64 = (3.0 * locals.var_tmf2);
        let assign23880_e18381: f64 = (assign23880_e18377 + assign23880_e18380);
        let assign23880_e18384: f64 = (4.0 * locals.var_tmf3);
        let assign23880_e18385: f64 = (assign23880_e18381 + assign23880_e18384);
        let assign23880_e18386: f64 = (-assign23880_e18385);
        let assign23880_e18388: f64 = (assign23880_e18386 * locals.var_tmf0);
        let assign23880_e18390: f64 = (assign23880_e18388 * locals.var_tmf0);
        (assign23880_e18390, (((((-(((2.0 * locals.var_tmf1_dn0) + (3.0 * locals.var_tmf2_dn0)) + (4.0 * locals.var_tmf3_dn0))) * locals.var_tmf0) + (assign23880_e18386 * locals.var_tmf0_dn0)) * locals.var_tmf0) + (assign23880_e18388 * locals.var_tmf0_dn0)), (((((-(((2.0 * locals.var_tmf1_dn2) + (3.0 * locals.var_tmf2_dn2)) + (4.0 * locals.var_tmf3_dn2))) * locals.var_tmf0) + (assign23880_e18386 * locals.var_tmf0_dn2)) * locals.var_tmf0) + (assign23880_e18388 * locals.var_tmf0_dn2)), (((((-(((2.0 * locals.var_tmf1_dn4) + (3.0 * locals.var_tmf2_dn4)) + (4.0 * locals.var_tmf3_dn4))) * locals.var_tmf0) + (assign23880_e18386 * locals.var_tmf0_dn4)) * locals.var_tmf0) + (assign23880_e18388 * locals.var_tmf0_dn4)), (((((-(((2.0 * locals.var_tmf1_dn5) + (3.0 * locals.var_tmf2_dn5)) + (4.0 * locals.var_tmf3_dn5))) * locals.var_tmf0) + (assign23880_e18386 * locals.var_tmf0_dn5)) * locals.var_tmf0) + (assign23880_e18388 * locals.var_tmf0_dn5)), (((((-(((2.0 * locals.var_tmf1_dn6) + (3.0 * locals.var_tmf2_dn6)) + (4.0 * locals.var_tmf3_dn6))) * locals.var_tmf0) + (assign23880_e18386 * locals.var_tmf0_dn6)) * locals.var_tmf0) + (assign23880_e18388 * locals.var_tmf0_dn6)), (((((-(((2.0 * locals.var_tmf1_dn7) + (3.0 * locals.var_tmf2_dn7)) + (4.0 * locals.var_tmf3_dn7))) * locals.var_tmf0) + (assign23880_e18386 * locals.var_tmf0_dn7)) * locals.var_tmf0) + (assign23880_e18388 * locals.var_tmf0_dn7)), (((((-(((2.0 * locals.var_tmf1_dn8) + (3.0 * locals.var_tmf2_dn8)) + (4.0 * locals.var_tmf3_dn8))) * locals.var_tmf0) + (assign23880_e18386 * locals.var_tmf0_dn8)) * locals.var_tmf0) + (assign23880_e18388 * locals.var_tmf0_dn8)), (((((-(((2.0 * locals.var_tmf1_dn9) + (3.0 * locals.var_tmf2_dn9)) + (4.0 * locals.var_tmf3_dn9))) * locals.var_tmf0) + (assign23880_e18386 * locals.var_tmf0_dn9)) * locals.var_tmf0) + (assign23880_e18388 * locals.var_tmf0_dn9)), (((((-(((2.0 * locals.var_tmf1_dn10) + (3.0 * locals.var_tmf2_dn10)) + (4.0 * locals.var_tmf3_dn10))) * locals.var_tmf0) + (assign23880_e18386 * locals.var_tmf0_dn10)) * locals.var_tmf0) + (assign23880_e18388 * locals.var_tmf0_dn10)), (((((-(((2.0 * locals.var_tmf1_dn13) + (3.0 * locals.var_tmf2_dn13)) + (4.0 * locals.var_tmf3_dn13))) * locals.var_tmf0) + (assign23880_e18386 * locals.var_tmf0_dn13)) * locals.var_tmf0) + (assign23880_e18388 * locals.var_tmf0_dn13)),)
    } else {
        (locals.var_vbscldvbs__blk436, locals.var_vbscldvbs__blk436_dn0, locals.var_vbscldvbs__blk436_dn2, locals.var_vbscldvbs__blk436_dn4, locals.var_vbscldvbs__blk436_dn5, locals.var_vbscldvbs__blk436_dn6, locals.var_vbscldvbs__blk436_dn7, locals.var_vbscldvbs__blk436_dn8, locals.var_vbscldvbs__blk436_dn9, locals.var_vbscldvbs__blk436_dn10, locals.var_vbscldvbs__blk436_dn13,)
    }
};
        locals.var_vbscldvbs__blk436 = assign23880_e18392;
        locals.var_vbscldvbs__blk436_dn0 = assign23880_e18392_d_n0;
        locals.var_vbscldvbs__blk436_dn2 = assign23880_e18392_d_n2;
        locals.var_vbscldvbs__blk436_dn4 = assign23880_e18392_d_n4;
        locals.var_vbscldvbs__blk436_dn5 = assign23880_e18392_d_n5;
        locals.var_vbscldvbs__blk436_dn6 = assign23880_e18392_d_n6;
        locals.var_vbscldvbs__blk436_dn7 = assign23880_e18392_d_n7;
        locals.var_vbscldvbs__blk436_dn8 = assign23880_e18392_d_n8;
        locals.var_vbscldvbs__blk436_dn9 = assign23880_e18392_d_n9;
        locals.var_vbscldvbs__blk436_dn10 = assign23880_e18392_d_n10;
        locals.var_vbscldvbs__blk436_dn13 = assign23880_e18392_d_n13;
        locals.var_vbscldvbs__blk436_rv = 0.0;

        let (assign23890_e18402, assign23890_e18402_d_n0, assign23890_e18402_d_n2, assign23890_e18402_d_n4, assign23890_e18402_d_n5, assign23890_e18402_d_n6, assign23890_e18402_d_n7, assign23890_e18402_d_n8, assign23890_e18402_d_n9, assign23890_e18402_d_n10, assign23890_e18402_d_n13,) = {
    if ((p.p37 != 0.0) && (locals.var_guard441 != 0.0)) {
        let assign23890_e18399: f64 = (1.0 - locals.var_tmf0);
        let assign23890_e18400: f64 = (locals.var_t2 * assign23890_e18399);
        (assign23890_e18400, ((locals.var_t2_dn0 * assign23890_e18399) + (locals.var_t2 * (-locals.var_tmf0_dn0))), ((locals.var_t2_dn2 * assign23890_e18399) + (locals.var_t2 * (-locals.var_tmf0_dn2))), ((locals.var_t2_dn4 * assign23890_e18399) + (locals.var_t2 * (-locals.var_tmf0_dn4))), ((locals.var_t2_dn5 * assign23890_e18399) + (locals.var_t2 * (-locals.var_tmf0_dn5))), ((locals.var_t2_dn6 * assign23890_e18399) + (locals.var_t2 * (-locals.var_tmf0_dn6))), ((locals.var_t2_dn7 * assign23890_e18399) + (locals.var_t2 * (-locals.var_tmf0_dn7))), ((locals.var_t2_dn8 * assign23890_e18399) + (locals.var_t2 * (-locals.var_tmf0_dn8))), ((locals.var_t2_dn9 * assign23890_e18399) + (locals.var_t2 * (-locals.var_tmf0_dn9))), ((locals.var_t2_dn10 * assign23890_e18399) + (locals.var_t2 * (-locals.var_tmf0_dn10))), ((locals.var_t2_dn13 * assign23890_e18399) + (locals.var_t2 * (-locals.var_tmf0_dn13))),)
    } else {
        (locals.var_ty, locals.var_ty_dn0, locals.var_ty_dn2, locals.var_ty_dn4, locals.var_ty_dn5, locals.var_ty_dn6, locals.var_ty_dn7, locals.var_ty_dn8, locals.var_ty_dn9, locals.var_ty_dn10, locals.var_ty_dn13,)
    }
};
        locals.var_ty = assign23890_e18402;
        locals.var_ty_dn0 = assign23890_e18402_d_n0;
        locals.var_ty_dn2 = assign23890_e18402_d_n2;
        locals.var_ty_dn4 = assign23890_e18402_d_n4;
        locals.var_ty_dn5 = assign23890_e18402_d_n5;
        locals.var_ty_dn6 = assign23890_e18402_d_n6;
        locals.var_ty_dn7 = assign23890_e18402_d_n7;
        locals.var_ty_dn8 = assign23890_e18402_d_n8;
        locals.var_ty_dn9 = assign23890_e18402_d_n9;
        locals.var_ty_dn10 = assign23890_e18402_d_n10;
        locals.var_ty_dn13 = assign23890_e18402_d_n13;
        locals.var_ty_rv = 0.0;

        let (assign23900_e18414, assign23900_e18414_d_n0, assign23900_e18414_d_n2, assign23900_e18414_d_n4, assign23900_e18414_d_n5, assign23900_e18414_d_n6, assign23900_e18414_d_n7, assign23900_e18414_d_n8, assign23900_e18414_d_n9, assign23900_e18414_d_n10, assign23900_e18414_d_n13,) = {
    if ((p.p37 != 0.0) && (locals.var_guard441 != 0.0)) {
        let assign23900_e18408: f64 = (1.0 - locals.var_tmf0);
        let assign23900_e18411: f64 = (locals.var_tmf1 * locals.var_vbscldvbs__blk436);
        let assign23900_e18412: f64 = (assign23900_e18408 + assign23900_e18411);
        (assign23900_e18412, ((-locals.var_tmf0_dn0) + ((locals.var_tmf1_dn0 * locals.var_vbscldvbs__blk436) + (locals.var_tmf1 * locals.var_vbscldvbs__blk436_dn0))), ((-locals.var_tmf0_dn2) + ((locals.var_tmf1_dn2 * locals.var_vbscldvbs__blk436) + (locals.var_tmf1 * locals.var_vbscldvbs__blk436_dn2))), ((-locals.var_tmf0_dn4) + ((locals.var_tmf1_dn4 * locals.var_vbscldvbs__blk436) + (locals.var_tmf1 * locals.var_vbscldvbs__blk436_dn4))), ((-locals.var_tmf0_dn5) + ((locals.var_tmf1_dn5 * locals.var_vbscldvbs__blk436) + (locals.var_tmf1 * locals.var_vbscldvbs__blk436_dn5))), ((-locals.var_tmf0_dn6) + ((locals.var_tmf1_dn6 * locals.var_vbscldvbs__blk436) + (locals.var_tmf1 * locals.var_vbscldvbs__blk436_dn6))), ((-locals.var_tmf0_dn7) + ((locals.var_tmf1_dn7 * locals.var_vbscldvbs__blk436) + (locals.var_tmf1 * locals.var_vbscldvbs__blk436_dn7))), ((-locals.var_tmf0_dn8) + ((locals.var_tmf1_dn8 * locals.var_vbscldvbs__blk436) + (locals.var_tmf1 * locals.var_vbscldvbs__blk436_dn8))), ((-locals.var_tmf0_dn9) + ((locals.var_tmf1_dn9 * locals.var_vbscldvbs__blk436) + (locals.var_tmf1 * locals.var_vbscldvbs__blk436_dn9))), ((-locals.var_tmf0_dn10) + ((locals.var_tmf1_dn10 * locals.var_vbscldvbs__blk436) + (locals.var_tmf1 * locals.var_vbscldvbs__blk436_dn10))), ((-locals.var_tmf0_dn13) + ((locals.var_tmf1_dn13 * locals.var_vbscldvbs__blk436) + (locals.var_tmf1 * locals.var_vbscldvbs__blk436_dn13))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign23900_e18414;
        locals.var_t0_dn0 = assign23900_e18414_d_n0;
        locals.var_t0_dn2 = assign23900_e18414_d_n2;
        locals.var_t0_dn4 = assign23900_e18414_d_n4;
        locals.var_t0_dn5 = assign23900_e18414_d_n5;
        locals.var_t0_dn6 = assign23900_e18414_d_n6;
        locals.var_t0_dn7 = assign23900_e18414_d_n7;
        locals.var_t0_dn8 = assign23900_e18414_d_n8;
        locals.var_t0_dn9 = assign23900_e18414_d_n9;
        locals.var_t0_dn10 = assign23900_e18414_d_n10;
        locals.var_t0_dn13 = assign23900_e18414_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign23910_e18421, assign23910_e18421_d_n0, assign23910_e18421_d_n2, assign23910_e18421_d_n4, assign23910_e18421_d_n5, assign23910_e18421_d_n6, assign23910_e18421_d_n7, assign23910_e18421_d_n8, assign23910_e18421_d_n9, assign23910_e18421_d_n10, assign23910_e18421_d_n13,) = {
    if ((p.p37 != 0.0) && (locals.var_guard441 != 0.0)) {
        let assign23910_e18419: f64 = (-locals.var_vbscldvbs__blk436);
        (assign23910_e18419, (-locals.var_vbscldvbs__blk436_dn0), (-locals.var_vbscldvbs__blk436_dn2), (-locals.var_vbscldvbs__blk436_dn4), (-locals.var_vbscldvbs__blk436_dn5), (-locals.var_vbscldvbs__blk436_dn6), (-locals.var_vbscldvbs__blk436_dn7), (-locals.var_vbscldvbs__blk436_dn8), (-locals.var_vbscldvbs__blk436_dn9), (-locals.var_vbscldvbs__blk436_dn10), (-locals.var_vbscldvbs__blk436_dn13),)
    } else {
        (locals.var_vbscldvbs__blk436, locals.var_vbscldvbs__blk436_dn0, locals.var_vbscldvbs__blk436_dn2, locals.var_vbscldvbs__blk436_dn4, locals.var_vbscldvbs__blk436_dn5, locals.var_vbscldvbs__blk436_dn6, locals.var_vbscldvbs__blk436_dn7, locals.var_vbscldvbs__blk436_dn8, locals.var_vbscldvbs__blk436_dn9, locals.var_vbscldvbs__blk436_dn10, locals.var_vbscldvbs__blk436_dn13,)
    }
};
        locals.var_vbscldvbs__blk436 = assign23910_e18421;
        locals.var_vbscldvbs__blk436_dn0 = assign23910_e18421_d_n0;
        locals.var_vbscldvbs__blk436_dn2 = assign23910_e18421_d_n2;
        locals.var_vbscldvbs__blk436_dn4 = assign23910_e18421_d_n4;
        locals.var_vbscldvbs__blk436_dn5 = assign23910_e18421_d_n5;
        locals.var_vbscldvbs__blk436_dn6 = assign23910_e18421_d_n6;
        locals.var_vbscldvbs__blk436_dn7 = assign23910_e18421_d_n7;
        locals.var_vbscldvbs__blk436_dn8 = assign23910_e18421_d_n8;
        locals.var_vbscldvbs__blk436_dn9 = assign23910_e18421_d_n9;
        locals.var_vbscldvbs__blk436_dn10 = assign23910_e18421_d_n10;
        locals.var_vbscldvbs__blk436_dn13 = assign23910_e18421_d_n13;
        locals.var_vbscldvbs__blk436_rv = 0.0;

        let (assign23920_e18429, assign23920_e18429_d_n0, assign23920_e18429_d_n2, assign23920_e18429_d_n4, assign23920_e18429_d_n5, assign23920_e18429_d_n6, assign23920_e18429_d_n7, assign23920_e18429_d_n8, assign23920_e18429_d_n9, assign23920_e18429_d_n10, assign23920_e18429_d_n13,) = {
    if ((p.p37 != 0.0) && (locals.var_guard441 != 0.0)) {
        let assign23920_e18427: f64 = (locals.var_vbs_bnd_local + locals.var_ty);
        (assign23920_e18427, (locals.var_vbs_bnd_local_dn0 + locals.var_ty_dn0), (locals.var_vbs_bnd_local_dn2 + locals.var_ty_dn2), (locals.var_vbs_bnd_local_dn4 + locals.var_ty_dn4), (locals.var_vbs_bnd_local_dn5 + locals.var_ty_dn5), (locals.var_vbs_bnd_local_dn6 + locals.var_ty_dn6), (locals.var_vbs_bnd_local_dn7 + locals.var_ty_dn7), (locals.var_vbs_bnd_local_dn8 + locals.var_ty_dn8), (locals.var_vbs_bnd_local_dn9 + locals.var_ty_dn9), (locals.var_vbs_bnd_local_dn10 + locals.var_ty_dn10), (locals.var_vbs_bnd_local_dn13 + locals.var_ty_dn13),)
    } else {
        (locals.var_vbscl__blk435, locals.var_vbscl__blk435_dn0, locals.var_vbscl__blk435_dn2, locals.var_vbscl__blk435_dn4, locals.var_vbscl__blk435_dn5, locals.var_vbscl__blk435_dn6, locals.var_vbscl__blk435_dn7, locals.var_vbscl__blk435_dn8, locals.var_vbscl__blk435_dn9, locals.var_vbscl__blk435_dn10, locals.var_vbscl__blk435_dn13,)
    }
};
        locals.var_vbscl__blk435 = assign23920_e18429;
        locals.var_vbscl__blk435_dn0 = assign23920_e18429_d_n0;
        locals.var_vbscl__blk435_dn2 = assign23920_e18429_d_n2;
        locals.var_vbscl__blk435_dn4 = assign23920_e18429_d_n4;
        locals.var_vbscl__blk435_dn5 = assign23920_e18429_d_n5;
        locals.var_vbscl__blk435_dn6 = assign23920_e18429_d_n6;
        locals.var_vbscl__blk435_dn7 = assign23920_e18429_d_n7;
        locals.var_vbscl__blk435_dn8 = assign23920_e18429_d_n8;
        locals.var_vbscl__blk435_dn9 = assign23920_e18429_d_n9;
        locals.var_vbscl__blk435_dn10 = assign23920_e18429_d_n10;
        locals.var_vbscl__blk435_dn13 = assign23920_e18429_d_n13;
        locals.var_vbscl__blk435_rv = 0.0;

        let (assign23930_e18437, assign23930_e18437_d_n0, assign23930_e18437_d_n2, assign23930_e18437_d_n4, assign23930_e18437_d_n5, assign23930_e18437_d_n6, assign23930_e18437_d_n7, assign23930_e18437_d_n8, assign23930_e18437_d_n9, assign23930_e18437_d_n10, assign23930_e18437_d_n13,) = {
    if ((p.p37 != 0.0) && (locals.var_guard441 != 0.0)) {
        let assign23930_e18435: f64 = (1.0 / locals.var_t2);
        (assign23930_e18435, (-(locals.var_t2_dn0 / (locals.var_t2 * locals.var_t2))), (-(locals.var_t2_dn2 / (locals.var_t2 * locals.var_t2))), (-(locals.var_t2_dn4 / (locals.var_t2 * locals.var_t2))), (-(locals.var_t2_dn5 / (locals.var_t2 * locals.var_t2))), (-(locals.var_t2_dn6 / (locals.var_t2 * locals.var_t2))), (-(locals.var_t2_dn7 / (locals.var_t2 * locals.var_t2))), (-(locals.var_t2_dn8 / (locals.var_t2 * locals.var_t2))), (-(locals.var_t2_dn9 / (locals.var_t2 * locals.var_t2))), (-(locals.var_t2_dn10 / (locals.var_t2 * locals.var_t2))), (-(locals.var_t2_dn13 / (locals.var_t2 * locals.var_t2))),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn13,)
    }
};
        locals.var_t3 = assign23930_e18437;
        locals.var_t3_dn0 = assign23930_e18437_d_n0;
        locals.var_t3_dn2 = assign23930_e18437_d_n2;
        locals.var_t3_dn4 = assign23930_e18437_d_n4;
        locals.var_t3_dn5 = assign23930_e18437_d_n5;
        locals.var_t3_dn6 = assign23930_e18437_d_n6;
        locals.var_t3_dn7 = assign23930_e18437_d_n7;
        locals.var_t3_dn8 = assign23930_e18437_d_n8;
        locals.var_t3_dn9 = assign23930_e18437_d_n9;
        locals.var_t3_dn10 = assign23930_e18437_d_n10;
        locals.var_t3_dn13 = assign23930_e18437_d_n13;
        locals.var_t3_rv = 0.0;

        let (assign23940_e18445, assign23940_e18445_d_n0, assign23940_e18445_d_n2, assign23940_e18445_d_n4, assign23940_e18445_d_n5, assign23940_e18445_d_n6, assign23940_e18445_d_n7, assign23940_e18445_d_n8, assign23940_e18445_d_n9, assign23940_e18445_d_n10, assign23940_e18445_d_n13,) = {
    if ((p.p37 != 0.0) && (locals.var_guard441 != 0.0)) {
        let assign23940_e18443: f64 = (locals.var_t1 * locals.var_t3);
        (assign23940_e18443, ((locals.var_t1_dn0 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn0)), ((locals.var_t1_dn2 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn2)), ((locals.var_t1_dn4 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn4)), ((locals.var_t1_dn5 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn5)), ((locals.var_t1_dn6 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn6)), ((locals.var_t1_dn7 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn7)), ((locals.var_t1_dn8 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn8)), ((locals.var_t1_dn9 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn9)), ((locals.var_t1_dn10 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn10)), ((locals.var_t1_dn13 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn13)),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn13,)
    }
};
        locals.var_t4 = assign23940_e18445;
        locals.var_t4_dn0 = assign23940_e18445_d_n0;
        locals.var_t4_dn2 = assign23940_e18445_d_n2;
        locals.var_t4_dn4 = assign23940_e18445_d_n4;
        locals.var_t4_dn5 = assign23940_e18445_d_n5;
        locals.var_t4_dn6 = assign23940_e18445_d_n6;
        locals.var_t4_dn7 = assign23940_e18445_d_n7;
        locals.var_t4_dn8 = assign23940_e18445_d_n8;
        locals.var_t4_dn9 = assign23940_e18445_d_n9;
        locals.var_t4_dn10 = assign23940_e18445_d_n10;
        locals.var_t4_dn13 = assign23940_e18445_d_n13;
        locals.var_t4_rv = 0.0;

        let (assign23950_e18453, assign23950_e18453_d_n0, assign23950_e18453_d_n2, assign23950_e18453_d_n4, assign23950_e18453_d_n5, assign23950_e18453_d_n6, assign23950_e18453_d_n7, assign23950_e18453_d_n8, assign23950_e18453_d_n9, assign23950_e18453_d_n10, assign23950_e18453_d_n13,) = {
    if ((p.p37 != 0.0) && (locals.var_guard441 != 0.0)) {
        let assign23950_e18451: f64 = (locals.var_t4 * locals.var_t4);
        (assign23950_e18451, ((locals.var_t4_dn0 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn0)), ((locals.var_t4_dn2 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn2)), ((locals.var_t4_dn4 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn4)), ((locals.var_t4_dn5 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn5)), ((locals.var_t4_dn6 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn6)), ((locals.var_t4_dn7 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn7)), ((locals.var_t4_dn8 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn8)), ((locals.var_t4_dn9 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn9)), ((locals.var_t4_dn10 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn10)), ((locals.var_t4_dn13 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn13)),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn13,)
    }
};
        locals.var_t5 = assign23950_e18453;
        locals.var_t5_dn0 = assign23950_e18453_d_n0;
        locals.var_t5_dn2 = assign23950_e18453_d_n2;
        locals.var_t5_dn4 = assign23950_e18453_d_n4;
        locals.var_t5_dn5 = assign23950_e18453_d_n5;
        locals.var_t5_dn6 = assign23950_e18453_d_n6;
        locals.var_t5_dn7 = assign23950_e18453_d_n7;
        locals.var_t5_dn8 = assign23950_e18453_d_n8;
        locals.var_t5_dn9 = assign23950_e18453_d_n9;
        locals.var_t5_dn10 = assign23950_e18453_d_n10;
        locals.var_t5_dn13 = assign23950_e18453_d_n13;
        locals.var_t5_rv = 0.0;

        let (assign23960_e18469, assign23960_e18469_d_n0, assign23960_e18469_d_n2, assign23960_e18469_d_n4, assign23960_e18469_d_n5, assign23960_e18469_d_n6, assign23960_e18469_d_n7, assign23960_e18469_d_n8, assign23960_e18469_d_n9, assign23960_e18469_d_n10, assign23960_e18469_d_n13,) = {
    if ((p.p37 != 0.0) && (locals.var_guard441 != 0.0)) {
        let assign23960_e18459: f64 = (1.0 + locals.var_t4);
        let assign23960_e18463: f64 = (1.0 + locals.var_t4);
        let assign23960_e18465: f64 = (assign23960_e18463 + locals.var_t5);
        let assign23960_e18466: f64 = (locals.var_t5 * assign23960_e18465);
        let assign23960_e18467: f64 = (assign23960_e18459 + assign23960_e18466);
        (assign23960_e18467, (locals.var_t4_dn0 + ((locals.var_t5_dn0 * assign23960_e18465) + (locals.var_t5 * (locals.var_t4_dn0 + locals.var_t5_dn0)))), (locals.var_t4_dn2 + ((locals.var_t5_dn2 * assign23960_e18465) + (locals.var_t5 * (locals.var_t4_dn2 + locals.var_t5_dn2)))), (locals.var_t4_dn4 + ((locals.var_t5_dn4 * assign23960_e18465) + (locals.var_t5 * (locals.var_t4_dn4 + locals.var_t5_dn4)))), (locals.var_t4_dn5 + ((locals.var_t5_dn5 * assign23960_e18465) + (locals.var_t5 * (locals.var_t4_dn5 + locals.var_t5_dn5)))), (locals.var_t4_dn6 + ((locals.var_t5_dn6 * assign23960_e18465) + (locals.var_t5 * (locals.var_t4_dn6 + locals.var_t5_dn6)))), (locals.var_t4_dn7 + ((locals.var_t5_dn7 * assign23960_e18465) + (locals.var_t5 * (locals.var_t4_dn7 + locals.var_t5_dn7)))), (locals.var_t4_dn8 + ((locals.var_t5_dn8 * assign23960_e18465) + (locals.var_t5 * (locals.var_t4_dn8 + locals.var_t5_dn8)))), (locals.var_t4_dn9 + ((locals.var_t5_dn9 * assign23960_e18465) + (locals.var_t5 * (locals.var_t4_dn9 + locals.var_t5_dn9)))), (locals.var_t4_dn10 + ((locals.var_t5_dn10 * assign23960_e18465) + (locals.var_t5 * (locals.var_t4_dn10 + locals.var_t5_dn10)))), (locals.var_t4_dn13 + ((locals.var_t5_dn13 * assign23960_e18465) + (locals.var_t5 * (locals.var_t4_dn13 + locals.var_t5_dn13)))),)
    } else {
        (locals.var_t7, locals.var_t7_dn0, locals.var_t7_dn2, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn13,)
    }
};
        locals.var_t7 = assign23960_e18469;
        locals.var_t7_dn0 = assign23960_e18469_d_n0;
        locals.var_t7_dn2 = assign23960_e18469_d_n2;
        locals.var_t7_dn4 = assign23960_e18469_d_n4;
        locals.var_t7_dn5 = assign23960_e18469_d_n5;
        locals.var_t7_dn6 = assign23960_e18469_d_n6;
        locals.var_t7_dn7 = assign23960_e18469_d_n7;
        locals.var_t7_dn8 = assign23960_e18469_d_n8;
        locals.var_t7_dn9 = assign23960_e18469_d_n9;
        locals.var_t7_dn10 = assign23960_e18469_d_n10;
        locals.var_t7_dn13 = assign23960_e18469_d_n13;
        locals.var_t7_rv = 0.0;

        let (assign23970_e18493, assign23970_e18493_d_n0, assign23970_e18493_d_n2, assign23970_e18493_d_n4, assign23970_e18493_d_n5, assign23970_e18493_d_n6, assign23970_e18493_d_n7, assign23970_e18493_d_n8, assign23970_e18493_d_n9, assign23970_e18493_d_n10, assign23970_e18493_d_n13,) = {
    if ((p.p37 != 0.0) && (locals.var_guard441 != 0.0)) {
        let assign23970_e18476: f64 = (2.0 * locals.var_t4);
        let assign23970_e18477: f64 = (1.0 + assign23970_e18476);
        let assign23970_e18480: f64 = (3.0 * locals.var_t5);
        let assign23970_e18481: f64 = (assign23970_e18477 + assign23970_e18480);
        let assign23970_e18484: f64 = (4.0 * locals.var_t4);
        let assign23970_e18486: f64 = (assign23970_e18484 * locals.var_t5);
        let assign23970_e18487: f64 = (assign23970_e18481 + assign23970_e18486);
        let assign23970_e18490: f64 = (locals.var_t7 * locals.var_t7);
        let assign23970_e18491: f64 = (assign23970_e18487 / assign23970_e18490);
        (assign23970_e18491, ((((((2.0 * locals.var_t4_dn0) + (3.0 * locals.var_t5_dn0)) + (((4.0 * locals.var_t4_dn0) * locals.var_t5) + (assign23970_e18484 * locals.var_t5_dn0))) * assign23970_e18490) - (assign23970_e18487 * ((locals.var_t7_dn0 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn0)))) / (assign23970_e18490 * assign23970_e18490)), ((((((2.0 * locals.var_t4_dn2) + (3.0 * locals.var_t5_dn2)) + (((4.0 * locals.var_t4_dn2) * locals.var_t5) + (assign23970_e18484 * locals.var_t5_dn2))) * assign23970_e18490) - (assign23970_e18487 * ((locals.var_t7_dn2 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn2)))) / (assign23970_e18490 * assign23970_e18490)), ((((((2.0 * locals.var_t4_dn4) + (3.0 * locals.var_t5_dn4)) + (((4.0 * locals.var_t4_dn4) * locals.var_t5) + (assign23970_e18484 * locals.var_t5_dn4))) * assign23970_e18490) - (assign23970_e18487 * ((locals.var_t7_dn4 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn4)))) / (assign23970_e18490 * assign23970_e18490)), ((((((2.0 * locals.var_t4_dn5) + (3.0 * locals.var_t5_dn5)) + (((4.0 * locals.var_t4_dn5) * locals.var_t5) + (assign23970_e18484 * locals.var_t5_dn5))) * assign23970_e18490) - (assign23970_e18487 * ((locals.var_t7_dn5 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn5)))) / (assign23970_e18490 * assign23970_e18490)), ((((((2.0 * locals.var_t4_dn6) + (3.0 * locals.var_t5_dn6)) + (((4.0 * locals.var_t4_dn6) * locals.var_t5) + (assign23970_e18484 * locals.var_t5_dn6))) * assign23970_e18490) - (assign23970_e18487 * ((locals.var_t7_dn6 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn6)))) / (assign23970_e18490 * assign23970_e18490)), ((((((2.0 * locals.var_t4_dn7) + (3.0 * locals.var_t5_dn7)) + (((4.0 * locals.var_t4_dn7) * locals.var_t5) + (assign23970_e18484 * locals.var_t5_dn7))) * assign23970_e18490) - (assign23970_e18487 * ((locals.var_t7_dn7 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn7)))) / (assign23970_e18490 * assign23970_e18490)), ((((((2.0 * locals.var_t4_dn8) + (3.0 * locals.var_t5_dn8)) + (((4.0 * locals.var_t4_dn8) * locals.var_t5) + (assign23970_e18484 * locals.var_t5_dn8))) * assign23970_e18490) - (assign23970_e18487 * ((locals.var_t7_dn8 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn8)))) / (assign23970_e18490 * assign23970_e18490)), ((((((2.0 * locals.var_t4_dn9) + (3.0 * locals.var_t5_dn9)) + (((4.0 * locals.var_t4_dn9) * locals.var_t5) + (assign23970_e18484 * locals.var_t5_dn9))) * assign23970_e18490) - (assign23970_e18487 * ((locals.var_t7_dn9 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn9)))) / (assign23970_e18490 * assign23970_e18490)), ((((((2.0 * locals.var_t4_dn10) + (3.0 * locals.var_t5_dn10)) + (((4.0 * locals.var_t4_dn10) * locals.var_t5) + (assign23970_e18484 * locals.var_t5_dn10))) * assign23970_e18490) - (assign23970_e18487 * ((locals.var_t7_dn10 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn10)))) / (assign23970_e18490 * assign23970_e18490)), ((((((2.0 * locals.var_t4_dn13) + (3.0 * locals.var_t5_dn13)) + (((4.0 * locals.var_t4_dn13) * locals.var_t5) + (assign23970_e18484 * locals.var_t5_dn13))) * assign23970_e18490) - (assign23970_e18487 * ((locals.var_t7_dn13 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn13)))) / (assign23970_e18490 * assign23970_e18490)),)
    } else {
        (locals.var_vbscldvbs__blk436, locals.var_vbscldvbs__blk436_dn0, locals.var_vbscldvbs__blk436_dn2, locals.var_vbscldvbs__blk436_dn4, locals.var_vbscldvbs__blk436_dn5, locals.var_vbscldvbs__blk436_dn6, locals.var_vbscldvbs__blk436_dn7, locals.var_vbscldvbs__blk436_dn8, locals.var_vbscldvbs__blk436_dn9, locals.var_vbscldvbs__blk436_dn10, locals.var_vbscldvbs__blk436_dn13,)
    }
};
        locals.var_vbscldvbs__blk436 = assign23970_e18493;
        locals.var_vbscldvbs__blk436_dn0 = assign23970_e18493_d_n0;
        locals.var_vbscldvbs__blk436_dn2 = assign23970_e18493_d_n2;
        locals.var_vbscldvbs__blk436_dn4 = assign23970_e18493_d_n4;
        locals.var_vbscldvbs__blk436_dn5 = assign23970_e18493_d_n5;
        locals.var_vbscldvbs__blk436_dn6 = assign23970_e18493_d_n6;
        locals.var_vbscldvbs__blk436_dn7 = assign23970_e18493_d_n7;
        locals.var_vbscldvbs__blk436_dn8 = assign23970_e18493_d_n8;
        locals.var_vbscldvbs__blk436_dn9 = assign23970_e18493_d_n9;
        locals.var_vbscldvbs__blk436_dn10 = assign23970_e18493_d_n10;
        locals.var_vbscldvbs__blk436_dn13 = assign23970_e18493_d_n13;
        locals.var_vbscldvbs__blk436_rv = 0.0;

        let (assign23980_e18500, assign23980_e18500_d_n0, assign23980_e18500_d_n2, assign23980_e18500_d_n4, assign23980_e18500_d_n5, assign23980_e18500_d_n6, assign23980_e18500_d_n7, assign23980_e18500_d_n8, assign23980_e18500_d_n9, assign23980_e18500_d_n10, assign23980_e18500_d_n13,) = {
    if ((p.p37 != 0.0) && (locals.var_guard441 == 0.0)) {
        (locals.var_vbs, 0.0, 0.0, 0.0, locals.var_vbs_dn5, 0.0, locals.var_vbs_dn7, locals.var_vbs_dn8, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vbscl__blk435, locals.var_vbscl__blk435_dn0, locals.var_vbscl__blk435_dn2, locals.var_vbscl__blk435_dn4, locals.var_vbscl__blk435_dn5, locals.var_vbscl__blk435_dn6, locals.var_vbscl__blk435_dn7, locals.var_vbscl__blk435_dn8, locals.var_vbscl__blk435_dn9, locals.var_vbscl__blk435_dn10, locals.var_vbscl__blk435_dn13,)
    }
};
        locals.var_vbscl__blk435 = assign23980_e18500;
        locals.var_vbscl__blk435_dn0 = assign23980_e18500_d_n0;
        locals.var_vbscl__blk435_dn2 = assign23980_e18500_d_n2;
        locals.var_vbscl__blk435_dn4 = assign23980_e18500_d_n4;
        locals.var_vbscl__blk435_dn5 = assign23980_e18500_d_n5;
        locals.var_vbscl__blk435_dn6 = assign23980_e18500_d_n6;
        locals.var_vbscl__blk435_dn7 = assign23980_e18500_d_n7;
        locals.var_vbscl__blk435_dn8 = assign23980_e18500_d_n8;
        locals.var_vbscl__blk435_dn9 = assign23980_e18500_d_n9;
        locals.var_vbscl__blk435_dn10 = assign23980_e18500_d_n10;
        locals.var_vbscl__blk435_dn13 = assign23980_e18500_d_n13;
        locals.var_vbscl__blk435_rv = 0.0;

        let (assign23990_e18507, assign23990_e18507_d_n0, assign23990_e18507_d_n2, assign23990_e18507_d_n4, assign23990_e18507_d_n5, assign23990_e18507_d_n6, assign23990_e18507_d_n7, assign23990_e18507_d_n8, assign23990_e18507_d_n9, assign23990_e18507_d_n10, assign23990_e18507_d_n13,) = {
    if ((p.p37 != 0.0) && (locals.var_guard441 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vbscldvbs__blk436, locals.var_vbscldvbs__blk436_dn0, locals.var_vbscldvbs__blk436_dn2, locals.var_vbscldvbs__blk436_dn4, locals.var_vbscldvbs__blk436_dn5, locals.var_vbscldvbs__blk436_dn6, locals.var_vbscldvbs__blk436_dn7, locals.var_vbscldvbs__blk436_dn8, locals.var_vbscldvbs__blk436_dn9, locals.var_vbscldvbs__blk436_dn10, locals.var_vbscldvbs__blk436_dn13,)
    }
};
        locals.var_vbscldvbs__blk436 = assign23990_e18507;
        locals.var_vbscldvbs__blk436_dn0 = assign23990_e18507_d_n0;
        locals.var_vbscldvbs__blk436_dn2 = assign23990_e18507_d_n2;
        locals.var_vbscldvbs__blk436_dn4 = assign23990_e18507_d_n4;
        locals.var_vbscldvbs__blk436_dn5 = assign23990_e18507_d_n5;
        locals.var_vbscldvbs__blk436_dn6 = assign23990_e18507_d_n6;
        locals.var_vbscldvbs__blk436_dn7 = assign23990_e18507_d_n7;
        locals.var_vbscldvbs__blk436_dn8 = assign23990_e18507_d_n8;
        locals.var_vbscldvbs__blk436_dn9 = assign23990_e18507_d_n9;
        locals.var_vbscldvbs__blk436_dn10 = assign23990_e18507_d_n10;
        locals.var_vbscldvbs__blk436_dn13 = assign23990_e18507_d_n13;
        locals.var_vbscldvbs__blk436_rv = 0.0;

        let (assign24000_e18512, assign24000_e18512_d_n0, assign24000_e18512_d_n2, assign24000_e18512_d_n4, assign24000_e18512_d_n5, assign24000_e18512_d_n6, assign24000_e18512_d_n7, assign24000_e18512_d_n8, assign24000_e18512_d_n9, assign24000_e18512_d_n10, assign24000_e18512_d_n13,) = {
    if (p.p37 == 0.0) {
        (locals.var_vbs, 0.0, 0.0, 0.0, locals.var_vbs_dn5, 0.0, locals.var_vbs_dn7, locals.var_vbs_dn8, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vbscl__blk435, locals.var_vbscl__blk435_dn0, locals.var_vbscl__blk435_dn2, locals.var_vbscl__blk435_dn4, locals.var_vbscl__blk435_dn5, locals.var_vbscl__blk435_dn6, locals.var_vbscl__blk435_dn7, locals.var_vbscl__blk435_dn8, locals.var_vbscl__blk435_dn9, locals.var_vbscl__blk435_dn10, locals.var_vbscl__blk435_dn13,)
    }
};
        locals.var_vbscl__blk435 = assign24000_e18512;
        locals.var_vbscl__blk435_dn0 = assign24000_e18512_d_n0;
        locals.var_vbscl__blk435_dn2 = assign24000_e18512_d_n2;
        locals.var_vbscl__blk435_dn4 = assign24000_e18512_d_n4;
        locals.var_vbscl__blk435_dn5 = assign24000_e18512_d_n5;
        locals.var_vbscl__blk435_dn6 = assign24000_e18512_d_n6;
        locals.var_vbscl__blk435_dn7 = assign24000_e18512_d_n7;
        locals.var_vbscl__blk435_dn8 = assign24000_e18512_d_n8;
        locals.var_vbscl__blk435_dn9 = assign24000_e18512_d_n9;
        locals.var_vbscl__blk435_dn10 = assign24000_e18512_d_n10;
        locals.var_vbscl__blk435_dn13 = assign24000_e18512_d_n13;
        locals.var_vbscl__blk435_rv = 0.0;

        let (assign24010_e18517, assign24010_e18517_d_n0, assign24010_e18517_d_n2, assign24010_e18517_d_n4, assign24010_e18517_d_n5, assign24010_e18517_d_n6, assign24010_e18517_d_n7, assign24010_e18517_d_n8, assign24010_e18517_d_n9, assign24010_e18517_d_n10, assign24010_e18517_d_n13,) = {
    if (p.p37 == 0.0) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vbscldvbs__blk436, locals.var_vbscldvbs__blk436_dn0, locals.var_vbscldvbs__blk436_dn2, locals.var_vbscldvbs__blk436_dn4, locals.var_vbscldvbs__blk436_dn5, locals.var_vbscldvbs__blk436_dn6, locals.var_vbscldvbs__blk436_dn7, locals.var_vbscldvbs__blk436_dn8, locals.var_vbscldvbs__blk436_dn9, locals.var_vbscldvbs__blk436_dn10, locals.var_vbscldvbs__blk436_dn13,)
    }
};
        locals.var_vbscldvbs__blk436 = assign24010_e18517;
        locals.var_vbscldvbs__blk436_dn0 = assign24010_e18517_d_n0;
        locals.var_vbscldvbs__blk436_dn2 = assign24010_e18517_d_n2;
        locals.var_vbscldvbs__blk436_dn4 = assign24010_e18517_d_n4;
        locals.var_vbscldvbs__blk436_dn5 = assign24010_e18517_d_n5;
        locals.var_vbscldvbs__blk436_dn6 = assign24010_e18517_d_n6;
        locals.var_vbscldvbs__blk436_dn7 = assign24010_e18517_d_n7;
        locals.var_vbscldvbs__blk436_dn8 = assign24010_e18517_d_n8;
        locals.var_vbscldvbs__blk436_dn9 = assign24010_e18517_d_n9;
        locals.var_vbscldvbs__blk436_dn10 = assign24010_e18517_d_n10;
        locals.var_vbscldvbs__blk436_dn13 = assign24010_e18517_d_n13;
        locals.var_vbscldvbs__blk436_rv = 0.0;

        let assign24020_e18520: f64 = (locals.var_vbscldvbs__blk436 * locals.var_vds);
        let assign24020_e18522: f64 = (assign24020_e18520 / 2.0);
        locals.var_t1 = assign24020_e18522;
        locals.var_t1_dn0 = (((locals.var_vbscldvbs__blk436_dn0 * locals.var_vds) + (locals.var_vbscldvbs__blk436 * locals.var_vds_dn0)) / 2.0);
        locals.var_t1_dn2 = (((locals.var_vbscldvbs__blk436_dn2 * locals.var_vds) + (locals.var_vbscldvbs__blk436 * locals.var_vds_dn2)) / 2.0);
        locals.var_t1_dn4 = (((locals.var_vbscldvbs__blk436_dn4 * locals.var_vds) + (locals.var_vbscldvbs__blk436 * locals.var_vds_dn4)) / 2.0);
        locals.var_t1_dn5 = (((locals.var_vbscldvbs__blk436_dn5 * locals.var_vds) + (locals.var_vbscldvbs__blk436 * locals.var_vds_dn5)) / 2.0);
        locals.var_t1_dn6 = (((locals.var_vbscldvbs__blk436_dn6 * locals.var_vds) + (locals.var_vbscldvbs__blk436 * locals.var_vds_dn6)) / 2.0);
        locals.var_t1_dn7 = (((locals.var_vbscldvbs__blk436_dn7 * locals.var_vds) + (locals.var_vbscldvbs__blk436 * locals.var_vds_dn7)) / 2.0);
        locals.var_t1_dn8 = (((locals.var_vbscldvbs__blk436_dn8 * locals.var_vds) + (locals.var_vbscldvbs__blk436 * locals.var_vds_dn8)) / 2.0);
        locals.var_t1_dn9 = (((locals.var_vbscldvbs__blk436_dn9 * locals.var_vds) + (locals.var_vbscldvbs__blk436 * locals.var_vds_dn9)) / 2.0);
        locals.var_t1_dn10 = (((locals.var_vbscldvbs__blk436_dn10 * locals.var_vds) + (locals.var_vbscldvbs__blk436 * locals.var_vds_dn10)) / 2.0);
        locals.var_t1_dn13 = (((locals.var_vbscldvbs__blk436_dn13 * locals.var_vds) + (locals.var_vbscldvbs__blk436 * locals.var_vds_dn13)) / 2.0);
        locals.var_t1_rv = 0.0;

        let assign24030_e18525: f64 = (2.0 * locals.var_t1);
        let assign24030_e18527: f64 = (assign24030_e18525 / p.p262);
        locals.var_tmf1 = assign24030_e18527;
        locals.var_tmf1_dn0 = ((2.0 * locals.var_t1_dn0) / p.p262);
        locals.var_tmf1_dn2 = ((2.0 * locals.var_t1_dn2) / p.p262);
        locals.var_tmf1_dn4 = ((2.0 * locals.var_t1_dn4) / p.p262);
        locals.var_tmf1_dn5 = ((2.0 * locals.var_t1_dn5) / p.p262);
        locals.var_tmf1_dn6 = ((2.0 * locals.var_t1_dn6) / p.p262);
        locals.var_tmf1_dn7 = ((2.0 * locals.var_t1_dn7) / p.p262);
        locals.var_tmf1_dn8 = ((2.0 * locals.var_t1_dn8) / p.p262);
        locals.var_tmf1_dn9 = ((2.0 * locals.var_t1_dn9) / p.p262);
        locals.var_tmf1_dn10 = ((2.0 * locals.var_t1_dn10) / p.p262);
        locals.var_tmf1_dn13 = ((2.0 * locals.var_t1_dn13) / p.p262);
        locals.var_tmf1_rv = 0.0;

        let assign24040_e18532: f64 = (1.0 / 2.0);
        let assign24040_e18536: f64 = (1.0 / 6.0);
        let assign24040_e18540: f64 = (1.0 / 24.0);
        let assign24040_e18544: f64 = (1.0 / 120.0);
        let assign24040_e18548: f64 = (1.0 / 720.0);
        let assign24040_e18552: f64 = (1.0 / 5040.0);
        let assign24040_e18553: f64 = (locals.var_tmf1 * assign24040_e18552);
        let assign24040_e18554: f64 = (assign24040_e18548 + assign24040_e18553);
        let assign24040_e18555: f64 = (locals.var_tmf1 * assign24040_e18554);
        let assign24040_e18556: f64 = (assign24040_e18544 + assign24040_e18555);
        let assign24040_e18557: f64 = (locals.var_tmf1 * assign24040_e18556);
        let assign24040_e18558: f64 = (assign24040_e18540 + assign24040_e18557);
        let assign24040_e18559: f64 = (locals.var_tmf1 * assign24040_e18558);
        let assign24040_e18560: f64 = (assign24040_e18536 + assign24040_e18559);
        let assign24040_e18561: f64 = (locals.var_tmf1 * assign24040_e18560);
        let assign24040_e18562: f64 = (assign24040_e18532 + assign24040_e18561);
        let assign24040_e18563: f64 = (locals.var_tmf1 * assign24040_e18562);
        let assign24040_e18564: f64 = (1.0 + assign24040_e18563);
        locals.var_tmf2 = assign24040_e18564;
        locals.var_tmf2_dn0 = ((locals.var_tmf1_dn0 * assign24040_e18562) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign24040_e18560) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign24040_e18558) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign24040_e18556) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign24040_e18554) + (locals.var_tmf1 * (locals.var_tmf1_dn0 * assign24040_e18552)))))))))));
        locals.var_tmf2_dn2 = ((locals.var_tmf1_dn2 * assign24040_e18562) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign24040_e18560) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign24040_e18558) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign24040_e18556) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign24040_e18554) + (locals.var_tmf1 * (locals.var_tmf1_dn2 * assign24040_e18552)))))))))));
        locals.var_tmf2_dn4 = ((locals.var_tmf1_dn4 * assign24040_e18562) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign24040_e18560) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign24040_e18558) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign24040_e18556) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign24040_e18554) + (locals.var_tmf1 * (locals.var_tmf1_dn4 * assign24040_e18552)))))))))));
        locals.var_tmf2_dn5 = ((locals.var_tmf1_dn5 * assign24040_e18562) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign24040_e18560) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign24040_e18558) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign24040_e18556) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign24040_e18554) + (locals.var_tmf1 * (locals.var_tmf1_dn5 * assign24040_e18552)))))))))));
        locals.var_tmf2_dn6 = ((locals.var_tmf1_dn6 * assign24040_e18562) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign24040_e18560) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign24040_e18558) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign24040_e18556) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign24040_e18554) + (locals.var_tmf1 * (locals.var_tmf1_dn6 * assign24040_e18552)))))))))));
        locals.var_tmf2_dn7 = ((locals.var_tmf1_dn7 * assign24040_e18562) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign24040_e18560) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign24040_e18558) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign24040_e18556) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign24040_e18554) + (locals.var_tmf1 * (locals.var_tmf1_dn7 * assign24040_e18552)))))))))));
        locals.var_tmf2_dn8 = ((locals.var_tmf1_dn8 * assign24040_e18562) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign24040_e18560) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign24040_e18558) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign24040_e18556) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign24040_e18554) + (locals.var_tmf1 * (locals.var_tmf1_dn8 * assign24040_e18552)))))))))));
        locals.var_tmf2_dn9 = ((locals.var_tmf1_dn9 * assign24040_e18562) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign24040_e18560) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign24040_e18558) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign24040_e18556) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign24040_e18554) + (locals.var_tmf1 * (locals.var_tmf1_dn9 * assign24040_e18552)))))))))));
        locals.var_tmf2_dn10 = ((locals.var_tmf1_dn10 * assign24040_e18562) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign24040_e18560) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign24040_e18558) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign24040_e18556) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign24040_e18554) + (locals.var_tmf1 * (locals.var_tmf1_dn10 * assign24040_e18552)))))))))));
        locals.var_tmf2_dn13 = ((locals.var_tmf1_dn13 * assign24040_e18562) + (locals.var_tmf1 * ((locals.var_tmf1_dn13 * assign24040_e18560) + (locals.var_tmf1 * ((locals.var_tmf1_dn13 * assign24040_e18558) + (locals.var_tmf1 * ((locals.var_tmf1_dn13 * assign24040_e18556) + (locals.var_tmf1 * ((locals.var_tmf1_dn13 * assign24040_e18554) + (locals.var_tmf1 * (locals.var_tmf1_dn13 * assign24040_e18552)))))))))));
        locals.var_tmf2_rv = 0.0;

        let assign24050_e18567: f64 = (1.0 / 2.0);
        let assign24050_e18571: f64 = (1.0 / 3.0);
        let assign24050_e18575: f64 = (1.0 / 8.0);
        let assign24050_e18579: f64 = (1.0 / 30.0);
        let assign24050_e18583: f64 = (1.0 / 144.0);
        let assign24050_e18587: f64 = (1.0 / 840.0);
        let assign24050_e18588: f64 = (locals.var_tmf1 * assign24050_e18587);
        let assign24050_e18589: f64 = (assign24050_e18583 + assign24050_e18588);
        let assign24050_e18590: f64 = (locals.var_tmf1 * assign24050_e18589);
        let assign24050_e18591: f64 = (assign24050_e18579 + assign24050_e18590);
        let assign24050_e18592: f64 = (locals.var_tmf1 * assign24050_e18591);
        let assign24050_e18593: f64 = (assign24050_e18575 + assign24050_e18592);
        let assign24050_e18594: f64 = (locals.var_tmf1 * assign24050_e18593);
        let assign24050_e18595: f64 = (assign24050_e18571 + assign24050_e18594);
        let assign24050_e18596: f64 = (locals.var_tmf1 * assign24050_e18595);
        let assign24050_e18597: f64 = (assign24050_e18567 + assign24050_e18596);
        locals.var_tmf3 = assign24050_e18597;
        locals.var_tmf3_dn0 = ((locals.var_tmf1_dn0 * assign24050_e18595) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign24050_e18593) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign24050_e18591) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign24050_e18589) + (locals.var_tmf1 * (locals.var_tmf1_dn0 * assign24050_e18587)))))))));
        locals.var_tmf3_dn2 = ((locals.var_tmf1_dn2 * assign24050_e18595) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign24050_e18593) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign24050_e18591) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign24050_e18589) + (locals.var_tmf1 * (locals.var_tmf1_dn2 * assign24050_e18587)))))))));
        locals.var_tmf3_dn4 = ((locals.var_tmf1_dn4 * assign24050_e18595) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign24050_e18593) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign24050_e18591) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign24050_e18589) + (locals.var_tmf1 * (locals.var_tmf1_dn4 * assign24050_e18587)))))))));
        locals.var_tmf3_dn5 = ((locals.var_tmf1_dn5 * assign24050_e18595) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign24050_e18593) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign24050_e18591) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign24050_e18589) + (locals.var_tmf1 * (locals.var_tmf1_dn5 * assign24050_e18587)))))))));
        locals.var_tmf3_dn6 = ((locals.var_tmf1_dn6 * assign24050_e18595) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign24050_e18593) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign24050_e18591) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign24050_e18589) + (locals.var_tmf1 * (locals.var_tmf1_dn6 * assign24050_e18587)))))))));
        locals.var_tmf3_dn7 = ((locals.var_tmf1_dn7 * assign24050_e18595) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign24050_e18593) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign24050_e18591) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign24050_e18589) + (locals.var_tmf1 * (locals.var_tmf1_dn7 * assign24050_e18587)))))))));
        locals.var_tmf3_dn8 = ((locals.var_tmf1_dn8 * assign24050_e18595) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign24050_e18593) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign24050_e18591) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign24050_e18589) + (locals.var_tmf1 * (locals.var_tmf1_dn8 * assign24050_e18587)))))))));
        locals.var_tmf3_dn9 = ((locals.var_tmf1_dn9 * assign24050_e18595) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign24050_e18593) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign24050_e18591) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign24050_e18589) + (locals.var_tmf1 * (locals.var_tmf1_dn9 * assign24050_e18587)))))))));
        locals.var_tmf3_dn10 = ((locals.var_tmf1_dn10 * assign24050_e18595) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign24050_e18593) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign24050_e18591) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign24050_e18589) + (locals.var_tmf1 * (locals.var_tmf1_dn10 * assign24050_e18587)))))))));
        locals.var_tmf3_dn13 = ((locals.var_tmf1_dn13 * assign24050_e18595) + (locals.var_tmf1 * ((locals.var_tmf1_dn13 * assign24050_e18593) + (locals.var_tmf1 * ((locals.var_tmf1_dn13 * assign24050_e18591) + (locals.var_tmf1 * ((locals.var_tmf1_dn13 * assign24050_e18589) + (locals.var_tmf1 * (locals.var_tmf1_dn13 * assign24050_e18587)))))))));
        locals.var_tmf3_rv = 0.0;

        let assign24060_e18600: f64 = (p.p262 / locals.var_tmf2);
        locals.var_vzadd__blk437 = assign24060_e18600;
        locals.var_vzadd__blk437_dn0 = (-((p.p262 * locals.var_tmf2_dn0) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_vzadd__blk437_dn2 = (-((p.p262 * locals.var_tmf2_dn2) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_vzadd__blk437_dn4 = (-((p.p262 * locals.var_tmf2_dn4) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_vzadd__blk437_dn5 = (-((p.p262 * locals.var_tmf2_dn5) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_vzadd__blk437_dn6 = (-((p.p262 * locals.var_tmf2_dn6) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_vzadd__blk437_dn7 = (-((p.p262 * locals.var_tmf2_dn7) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_vzadd__blk437_dn8 = (-((p.p262 * locals.var_tmf2_dn8) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_vzadd__blk437_dn9 = (-((p.p262 * locals.var_tmf2_dn9) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_vzadd__blk437_dn10 = (-((p.p262 * locals.var_tmf2_dn10) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_vzadd__blk437_dn13 = (-((p.p262 * locals.var_tmf2_dn13) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_vzadd__blk437_rv = 0.0;

        let assign24070_e18602: f64 = (-2.0);
        let assign24070_e18604: f64 = (assign24070_e18602 * locals.var_tmf3);
        let assign24070_e18607: f64 = (locals.var_tmf2 * locals.var_tmf2);
        let assign24070_e18608: f64 = (assign24070_e18604 / assign24070_e18607);
        locals.var_t2 = assign24070_e18608;
        locals.var_t2_dn0 = ((((assign24070_e18602 * locals.var_tmf3_dn0) * assign24070_e18607) - (assign24070_e18604 * ((locals.var_tmf2_dn0 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn0)))) / (assign24070_e18607 * assign24070_e18607));
        locals.var_t2_dn2 = ((((assign24070_e18602 * locals.var_tmf3_dn2) * assign24070_e18607) - (assign24070_e18604 * ((locals.var_tmf2_dn2 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn2)))) / (assign24070_e18607 * assign24070_e18607));
        locals.var_t2_dn4 = ((((assign24070_e18602 * locals.var_tmf3_dn4) * assign24070_e18607) - (assign24070_e18604 * ((locals.var_tmf2_dn4 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn4)))) / (assign24070_e18607 * assign24070_e18607));
        locals.var_t2_dn5 = ((((assign24070_e18602 * locals.var_tmf3_dn5) * assign24070_e18607) - (assign24070_e18604 * ((locals.var_tmf2_dn5 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn5)))) / (assign24070_e18607 * assign24070_e18607));
        locals.var_t2_dn6 = ((((assign24070_e18602 * locals.var_tmf3_dn6) * assign24070_e18607) - (assign24070_e18604 * ((locals.var_tmf2_dn6 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn6)))) / (assign24070_e18607 * assign24070_e18607));
        locals.var_t2_dn7 = ((((assign24070_e18602 * locals.var_tmf3_dn7) * assign24070_e18607) - (assign24070_e18604 * ((locals.var_tmf2_dn7 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn7)))) / (assign24070_e18607 * assign24070_e18607));
        locals.var_t2_dn8 = ((((assign24070_e18602 * locals.var_tmf3_dn8) * assign24070_e18607) - (assign24070_e18604 * ((locals.var_tmf2_dn8 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn8)))) / (assign24070_e18607 * assign24070_e18607));
        locals.var_t2_dn9 = ((((assign24070_e18602 * locals.var_tmf3_dn9) * assign24070_e18607) - (assign24070_e18604 * ((locals.var_tmf2_dn9 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn9)))) / (assign24070_e18607 * assign24070_e18607));
        locals.var_t2_dn10 = ((((assign24070_e18602 * locals.var_tmf3_dn10) * assign24070_e18607) - (assign24070_e18604 * ((locals.var_tmf2_dn10 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn10)))) / (assign24070_e18607 * assign24070_e18607));
        locals.var_t2_dn13 = ((((assign24070_e18602 * locals.var_tmf3_dn13) * assign24070_e18607) - (assign24070_e18604 * ((locals.var_tmf2_dn13 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn13)))) / (assign24070_e18607 * assign24070_e18607));
        locals.var_t2_rv = 0.0;

        let assign24080_e18611: f64 = if locals.var_vzadd__blk437 < 1e-12 { 1.0 } else { 0.0 };
        locals.var_guard442 = assign24080_e18611;
        locals.var_guard442_rv = 0.0;

        let (assign24090_e18615, assign24090_e18615_d_n0, assign24090_e18615_d_n2, assign24090_e18615_d_n4, assign24090_e18615_d_n5, assign24090_e18615_d_n6, assign24090_e18615_d_n7, assign24090_e18615_d_n8, assign24090_e18615_d_n9, assign24090_e18615_d_n10, assign24090_e18615_d_n13,) = {
    if (locals.var_guard442 != 0.0) {
        (1e-12, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vzadd__blk437, locals.var_vzadd__blk437_dn0, locals.var_vzadd__blk437_dn2, locals.var_vzadd__blk437_dn4, locals.var_vzadd__blk437_dn5, locals.var_vzadd__blk437_dn6, locals.var_vzadd__blk437_dn7, locals.var_vzadd__blk437_dn8, locals.var_vzadd__blk437_dn9, locals.var_vzadd__blk437_dn10, locals.var_vzadd__blk437_dn13,)
    }
};
        locals.var_vzadd__blk437 = assign24090_e18615;
        locals.var_vzadd__blk437_dn0 = assign24090_e18615_d_n0;
        locals.var_vzadd__blk437_dn2 = assign24090_e18615_d_n2;
        locals.var_vzadd__blk437_dn4 = assign24090_e18615_d_n4;
        locals.var_vzadd__blk437_dn5 = assign24090_e18615_d_n5;
        locals.var_vzadd__blk437_dn6 = assign24090_e18615_d_n6;
        locals.var_vzadd__blk437_dn7 = assign24090_e18615_d_n7;
        locals.var_vzadd__blk437_dn8 = assign24090_e18615_d_n8;
        locals.var_vzadd__blk437_dn9 = assign24090_e18615_d_n9;
        locals.var_vzadd__blk437_dn10 = assign24090_e18615_d_n10;
        locals.var_vzadd__blk437_dn13 = assign24090_e18615_d_n13;
        locals.var_vzadd__blk437_rv = 0.0;

        let assign24100_e18618: f64 = (locals.var_vbscl__blk435 + locals.var_vzadd__blk437);
        locals.var_vbsz__blk438 = assign24100_e18618;
        locals.var_vbsz__blk438_dn0 = (locals.var_vbscl__blk435_dn0 + locals.var_vzadd__blk437_dn0);
        locals.var_vbsz__blk438_dn2 = (locals.var_vbscl__blk435_dn2 + locals.var_vzadd__blk437_dn2);
        locals.var_vbsz__blk438_dn4 = (locals.var_vbscl__blk435_dn4 + locals.var_vzadd__blk437_dn4);
        locals.var_vbsz__blk438_dn5 = (locals.var_vbscl__blk435_dn5 + locals.var_vzadd__blk437_dn5);
        locals.var_vbsz__blk438_dn6 = (locals.var_vbscl__blk435_dn6 + locals.var_vzadd__blk437_dn6);
        locals.var_vbsz__blk438_dn7 = (locals.var_vbscl__blk435_dn7 + locals.var_vzadd__blk437_dn7);
        locals.var_vbsz__blk438_dn8 = (locals.var_vbscl__blk435_dn8 + locals.var_vzadd__blk437_dn8);
        locals.var_vbsz__blk438_dn9 = (locals.var_vbscl__blk435_dn9 + locals.var_vzadd__blk437_dn9);
        locals.var_vbsz__blk438_dn10 = (locals.var_vbscl__blk435_dn10 + locals.var_vzadd__blk437_dn10);
        locals.var_vbsz__blk438_dn13 = (locals.var_vbscl__blk435_dn13 + locals.var_vzadd__blk437_dn13);
        locals.var_vbsz__blk438_rv = 0.0;

        let assign24110_e18622: f64 = (2.0 * locals.var_vzadd__blk437);
        let assign24110_e18623: f64 = (locals.var_vds + assign24110_e18622);
        locals.var_vdsz__blk439 = assign24110_e18623;
        locals.var_vdsz__blk439_dn0 = (locals.var_vds_dn0 + (2.0 * locals.var_vzadd__blk437_dn0));
        locals.var_vdsz__blk439_dn2 = (locals.var_vds_dn2 + (2.0 * locals.var_vzadd__blk437_dn2));
        locals.var_vdsz__blk439_dn4 = (locals.var_vds_dn4 + (2.0 * locals.var_vzadd__blk437_dn4));
        locals.var_vdsz__blk439_dn5 = (locals.var_vds_dn5 + (2.0 * locals.var_vzadd__blk437_dn5));
        locals.var_vdsz__blk439_dn6 = (locals.var_vds_dn6 + (2.0 * locals.var_vzadd__blk437_dn6));
        locals.var_vdsz__blk439_dn7 = (locals.var_vds_dn7 + (2.0 * locals.var_vzadd__blk437_dn7));
        locals.var_vdsz__blk439_dn8 = (locals.var_vds_dn8 + (2.0 * locals.var_vzadd__blk437_dn8));
        locals.var_vdsz__blk439_dn9 = (locals.var_vds_dn9 + (2.0 * locals.var_vzadd__blk437_dn9));
        locals.var_vdsz__blk439_dn10 = (locals.var_vds_dn10 + (2.0 * locals.var_vzadd__blk437_dn10));
        locals.var_vdsz__blk439_dn13 = (locals.var_vds_dn13 + (2.0 * locals.var_vzadd__blk437_dn13));
        locals.var_vdsz__blk439_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_64(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let assign24120_e18626: f64 = (locals.var_vgs + locals.var_vzadd__blk437);
        locals.var_vgsz__blk440 = assign24120_e18626;
        locals.var_vgsz__blk440_dn0 = locals.var_vzadd__blk437_dn0;
        locals.var_vgsz__blk440_dn2 = locals.var_vzadd__blk437_dn2;
        locals.var_vgsz__blk440_dn4 = locals.var_vzadd__blk437_dn4;
        locals.var_vgsz__blk440_dn5 = (locals.var_vgs_dn5 + locals.var_vzadd__blk437_dn5);
        locals.var_vgsz__blk440_dn6 = (locals.var_vgs_dn6 + locals.var_vzadd__blk437_dn6);
        locals.var_vgsz__blk440_dn7 = (locals.var_vgs_dn7 + locals.var_vzadd__blk437_dn7);
        locals.var_vgsz__blk440_dn8 = locals.var_vzadd__blk437_dn8;
        locals.var_vgsz__blk440_dn9 = locals.var_vzadd__blk437_dn9;
        locals.var_vgsz__blk440_dn10 = locals.var_vzadd__blk437_dn10;
        locals.var_vgsz__blk440_dn13 = locals.var_vzadd__blk437_dn13;
        locals.var_vgsz__blk440_rv = 0.0;

        let assign24130_e18629: f64 = (locals.var_vgs - locals.var_vfb);
        let assign24130_e18631: f64 = (assign24130_e18629 + locals.var_dvth);
        let assign24130_e18633: f64 = (assign24130_e18631 - locals.var_dppg);
        locals.var_vgp = assign24130_e18633;
        locals.var_vgp_dn0 = (locals.var_dvth_dn0 - locals.var_dppg_dn0);
        locals.var_vgp_dn2 = (locals.var_dvth_dn2 - locals.var_dppg_dn2);
        locals.var_vgp_dn4 = (locals.var_dvth_dn4 - locals.var_dppg_dn4);
        locals.var_vgp_dn5 = ((locals.var_vgs_dn5 + locals.var_dvth_dn5) - locals.var_dppg_dn5);
        locals.var_vgp_dn6 = ((locals.var_vgs_dn6 + locals.var_dvth_dn6) - locals.var_dppg_dn6);
        locals.var_vgp_dn7 = ((locals.var_vgs_dn7 + locals.var_dvth_dn7) - locals.var_dppg_dn7);
        locals.var_vgp_dn8 = (locals.var_dvth_dn8 - locals.var_dppg_dn8);
        locals.var_vgp_dn9 = (locals.var_dvth_dn9 - locals.var_dppg_dn9);
        locals.var_vgp_dn10 = (locals.var_dvth_dn10 - locals.var_dppg_dn10);
        locals.var_vgp_dn13 = (locals.var_dvth_dn13 - locals.var_dppg_dn13);
        locals.var_vgp_rv = 0.0;

        let assign24140_e18636: f64 = (locals.var_vfb - locals.var_dvth);
        let assign24140_e18638: f64 = (assign24140_e18636 + locals.var_dppg);
        let assign24140_e18640: f64 = (assign24140_e18638 + locals.var_vbscl__blk435);
        locals.var_vgs_fb = assign24140_e18640;
        locals.var_vgs_fb_dn0 = (((-locals.var_dvth_dn0) + locals.var_dppg_dn0) + locals.var_vbscl__blk435_dn0);
        locals.var_vgs_fb_dn2 = (((-locals.var_dvth_dn2) + locals.var_dppg_dn2) + locals.var_vbscl__blk435_dn2);
        locals.var_vgs_fb_dn4 = (((-locals.var_dvth_dn4) + locals.var_dppg_dn4) + locals.var_vbscl__blk435_dn4);
        locals.var_vgs_fb_dn5 = (((-locals.var_dvth_dn5) + locals.var_dppg_dn5) + locals.var_vbscl__blk435_dn5);
        locals.var_vgs_fb_dn6 = (((-locals.var_dvth_dn6) + locals.var_dppg_dn6) + locals.var_vbscl__blk435_dn6);
        locals.var_vgs_fb_dn7 = (((-locals.var_dvth_dn7) + locals.var_dppg_dn7) + locals.var_vbscl__blk435_dn7);
        locals.var_vgs_fb_dn8 = (((-locals.var_dvth_dn8) + locals.var_dppg_dn8) + locals.var_vbscl__blk435_dn8);
        locals.var_vgs_fb_dn9 = (((-locals.var_dvth_dn9) + locals.var_dppg_dn9) + locals.var_vbscl__blk435_dn9);
        locals.var_vgs_fb_dn10 = (((-locals.var_dvth_dn10) + locals.var_dppg_dn10) + locals.var_vbscl__blk435_dn10);
        locals.var_vgs_fb_dn13 = (((-locals.var_dvth_dn13) + locals.var_dppg_dn13) + locals.var_vbscl__blk435_dn13);
        locals.var_vgs_fb_rv = 0.0;

        let assign24150_e18643: f64 = if locals.var_uc_codep != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard443 = assign24150_e18643;
        locals.var_guard443_rv = 0.0;

        let assign24160_e18646: f64 = if p.p42 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard444 = assign24160_e18646;
        locals.var_guard444_rv = 0.0;

        let assign24170_e18649: f64 = if p.p42 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard445 = assign24170_e18649;
        locals.var_guard445_rv = 0.0;

        let assign24180_e18652: f64 = if p.p42 == 3.0 { 1.0 } else { 0.0 };
        locals.var_guard446 = assign24180_e18652;
        locals.var_guard446_rv = 0.0;

        let (assign24190_e18658, assign24190_e18658_d_n0, assign24190_e18658_d_n2, assign24190_e18658_d_n4, assign24190_e18658_d_n5, assign24190_e18658_d_n6, assign24190_e18658_d_n7, assign24190_e18658_d_n8, assign24190_e18658_d_n9, assign24190_e18658_d_n10, assign24190_e18658_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) {
        (locals.var_vbipn, locals.var_vbipn_dn0, locals.var_vbipn_dn2, locals.var_vbipn_dn4, locals.var_vbipn_dn5, locals.var_vbipn_dn6, locals.var_vbipn_dn7, locals.var_vbipn_dn8, locals.var_vbipn_dn9, locals.var_vbipn_dn10, locals.var_vbipn_dn13,)
    } else {
        (locals.var_vbi_dep, locals.var_vbi_dep_dn0, locals.var_vbi_dep_dn2, locals.var_vbi_dep_dn4, locals.var_vbi_dep_dn5, locals.var_vbi_dep_dn6, locals.var_vbi_dep_dn7, locals.var_vbi_dep_dn8, locals.var_vbi_dep_dn9, locals.var_vbi_dep_dn10, locals.var_vbi_dep_dn13,)
    }
};
        locals.var_vbi_dep = assign24190_e18658;
        locals.var_vbi_dep_dn0 = assign24190_e18658_d_n0;
        locals.var_vbi_dep_dn2 = assign24190_e18658_d_n2;
        locals.var_vbi_dep_dn4 = assign24190_e18658_d_n4;
        locals.var_vbi_dep_dn5 = assign24190_e18658_d_n5;
        locals.var_vbi_dep_dn6 = assign24190_e18658_d_n6;
        locals.var_vbi_dep_dn7 = assign24190_e18658_d_n7;
        locals.var_vbi_dep_dn8 = assign24190_e18658_d_n8;
        locals.var_vbi_dep_dn9 = assign24190_e18658_d_n9;
        locals.var_vbi_dep_dn10 = assign24190_e18658_d_n10;
        locals.var_vbi_dep_dn13 = assign24190_e18658_d_n13;
        locals.var_vbi_dep_rv = 0.0;

        let (assign24200_e18666, assign24200_e18666_d_n0, assign24200_e18666_d_n2, assign24200_e18666_d_n4, assign24200_e18666_d_n5, assign24200_e18666_d_n6, assign24200_e18666_d_n7, assign24200_e18666_d_n8, assign24200_e18666_d_n9, assign24200_e18666_d_n10, assign24200_e18666_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) {
        let assign24200_e18664: f64 = (1.6021918e-19 * locals.var_uc_ndepm);
        (assign24200_e18664, (1.6021918e-19 * locals.var_uc_ndepm_dn0), (1.6021918e-19 * locals.var_uc_ndepm_dn2), (1.6021918e-19 * locals.var_uc_ndepm_dn4), (1.6021918e-19 * locals.var_uc_ndepm_dn5), (1.6021918e-19 * locals.var_uc_ndepm_dn6), (1.6021918e-19 * locals.var_uc_ndepm_dn7), (1.6021918e-19 * locals.var_uc_ndepm_dn8), (1.6021918e-19 * locals.var_uc_ndepm_dn9), (1.6021918e-19 * locals.var_uc_ndepm_dn10), (1.6021918e-19 * locals.var_uc_ndepm_dn13),)
    } else {
        (locals.var_q_ndepm, locals.var_q_ndepm_dn0, locals.var_q_ndepm_dn2, locals.var_q_ndepm_dn4, locals.var_q_ndepm_dn5, locals.var_q_ndepm_dn6, locals.var_q_ndepm_dn7, locals.var_q_ndepm_dn8, locals.var_q_ndepm_dn9, locals.var_q_ndepm_dn10, locals.var_q_ndepm_dn13,)
    }
};
        locals.var_q_ndepm = assign24200_e18666;
        locals.var_q_ndepm_dn0 = assign24200_e18666_d_n0;
        locals.var_q_ndepm_dn2 = assign24200_e18666_d_n2;
        locals.var_q_ndepm_dn4 = assign24200_e18666_d_n4;
        locals.var_q_ndepm_dn5 = assign24200_e18666_d_n5;
        locals.var_q_ndepm_dn6 = assign24200_e18666_d_n6;
        locals.var_q_ndepm_dn7 = assign24200_e18666_d_n7;
        locals.var_q_ndepm_dn8 = assign24200_e18666_d_n8;
        locals.var_q_ndepm_dn9 = assign24200_e18666_d_n9;
        locals.var_q_ndepm_dn10 = assign24200_e18666_d_n10;
        locals.var_q_ndepm_dn13 = assign24200_e18666_d_n13;
        locals.var_q_ndepm_rv = 0.0;

        let (assign24210_e18674, assign24210_e18674_d_n0, assign24210_e18674_d_n2, assign24210_e18674_d_n4, assign24210_e18674_d_n5, assign24210_e18674_d_n6, assign24210_e18674_d_n7, assign24210_e18674_d_n8, assign24210_e18674_d_n9, assign24210_e18674_d_n10, assign24210_e18674_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) {
        let assign24210_e18672: f64 = (locals.var_uc_ndepm * locals.var_uc_ndepm);
        (assign24210_e18672, ((locals.var_uc_ndepm_dn0 * locals.var_uc_ndepm) + (locals.var_uc_ndepm * locals.var_uc_ndepm_dn0)), ((locals.var_uc_ndepm_dn2 * locals.var_uc_ndepm) + (locals.var_uc_ndepm * locals.var_uc_ndepm_dn2)), ((locals.var_uc_ndepm_dn4 * locals.var_uc_ndepm) + (locals.var_uc_ndepm * locals.var_uc_ndepm_dn4)), ((locals.var_uc_ndepm_dn5 * locals.var_uc_ndepm) + (locals.var_uc_ndepm * locals.var_uc_ndepm_dn5)), ((locals.var_uc_ndepm_dn6 * locals.var_uc_ndepm) + (locals.var_uc_ndepm * locals.var_uc_ndepm_dn6)), ((locals.var_uc_ndepm_dn7 * locals.var_uc_ndepm) + (locals.var_uc_ndepm * locals.var_uc_ndepm_dn7)), ((locals.var_uc_ndepm_dn8 * locals.var_uc_ndepm) + (locals.var_uc_ndepm * locals.var_uc_ndepm_dn8)), ((locals.var_uc_ndepm_dn9 * locals.var_uc_ndepm) + (locals.var_uc_ndepm * locals.var_uc_ndepm_dn9)), ((locals.var_uc_ndepm_dn10 * locals.var_uc_ndepm) + (locals.var_uc_ndepm * locals.var_uc_ndepm_dn10)), ((locals.var_uc_ndepm_dn13 * locals.var_uc_ndepm) + (locals.var_uc_ndepm * locals.var_uc_ndepm_dn13)),)
    } else {
        (locals.var_ndepm2, locals.var_ndepm2_dn0, locals.var_ndepm2_dn2, locals.var_ndepm2_dn4, locals.var_ndepm2_dn5, locals.var_ndepm2_dn6, locals.var_ndepm2_dn7, locals.var_ndepm2_dn8, locals.var_ndepm2_dn9, locals.var_ndepm2_dn10, locals.var_ndepm2_dn13,)
    }
};
        locals.var_ndepm2 = assign24210_e18674;
        locals.var_ndepm2_dn0 = assign24210_e18674_d_n0;
        locals.var_ndepm2_dn2 = assign24210_e18674_d_n2;
        locals.var_ndepm2_dn4 = assign24210_e18674_d_n4;
        locals.var_ndepm2_dn5 = assign24210_e18674_d_n5;
        locals.var_ndepm2_dn6 = assign24210_e18674_d_n6;
        locals.var_ndepm2_dn7 = assign24210_e18674_d_n7;
        locals.var_ndepm2_dn8 = assign24210_e18674_d_n8;
        locals.var_ndepm2_dn9 = assign24210_e18674_d_n9;
        locals.var_ndepm2_dn10 = assign24210_e18674_d_n10;
        locals.var_ndepm2_dn13 = assign24210_e18674_d_n13;
        locals.var_ndepm2_rv = 0.0;

        let (assign24220_e18684, assign24220_e18684_d_n0, assign24220_e18684_d_n2, assign24220_e18684_d_n4, assign24220_e18684_d_n5, assign24220_e18684_d_n6, assign24220_e18684_d_n7, assign24220_e18684_d_n8, assign24220_e18684_d_n9, assign24220_e18684_d_n10, assign24220_e18684_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) {
        let assign24220_e18680: f64 = (1.6021918e-19 * locals.var_uc_ndepm);
        let assign24220_e18682: f64 = (assign24220_e18680 * 1.034943e-10);
        (assign24220_e18682, ((1.6021918e-19 * locals.var_uc_ndepm_dn0) * 1.034943e-10), ((1.6021918e-19 * locals.var_uc_ndepm_dn2) * 1.034943e-10), ((1.6021918e-19 * locals.var_uc_ndepm_dn4) * 1.034943e-10), ((1.6021918e-19 * locals.var_uc_ndepm_dn5) * 1.034943e-10), ((1.6021918e-19 * locals.var_uc_ndepm_dn6) * 1.034943e-10), ((1.6021918e-19 * locals.var_uc_ndepm_dn7) * 1.034943e-10), ((1.6021918e-19 * locals.var_uc_ndepm_dn8) * 1.034943e-10), ((1.6021918e-19 * locals.var_uc_ndepm_dn9) * 1.034943e-10), ((1.6021918e-19 * locals.var_uc_ndepm_dn10) * 1.034943e-10), ((1.6021918e-19 * locals.var_uc_ndepm_dn13) * 1.034943e-10),)
    } else {
        (locals.var_q_ndepm_esi, locals.var_q_ndepm_esi_dn0, locals.var_q_ndepm_esi_dn2, locals.var_q_ndepm_esi_dn4, locals.var_q_ndepm_esi_dn5, locals.var_q_ndepm_esi_dn6, locals.var_q_ndepm_esi_dn7, locals.var_q_ndepm_esi_dn8, locals.var_q_ndepm_esi_dn9, locals.var_q_ndepm_esi_dn10, locals.var_q_ndepm_esi_dn13,)
    }
};
        locals.var_q_ndepm_esi = assign24220_e18684;
        locals.var_q_ndepm_esi_dn0 = assign24220_e18684_d_n0;
        locals.var_q_ndepm_esi_dn2 = assign24220_e18684_d_n2;
        locals.var_q_ndepm_esi_dn4 = assign24220_e18684_d_n4;
        locals.var_q_ndepm_esi_dn5 = assign24220_e18684_d_n5;
        locals.var_q_ndepm_esi_dn6 = assign24220_e18684_d_n6;
        locals.var_q_ndepm_esi_dn7 = assign24220_e18684_d_n7;
        locals.var_q_ndepm_esi_dn8 = assign24220_e18684_d_n8;
        locals.var_q_ndepm_esi_dn9 = assign24220_e18684_d_n9;
        locals.var_q_ndepm_esi_dn10 = assign24220_e18684_d_n10;
        locals.var_q_ndepm_esi_dn13 = assign24220_e18684_d_n13;
        locals.var_q_ndepm_esi_rv = 0.0;

        let (assign24230_e18692, assign24230_e18692_d_n0, assign24230_e18692_d_n2, assign24230_e18692_d_n4, assign24230_e18692_d_n5, assign24230_e18692_d_n6, assign24230_e18692_d_n7, assign24230_e18692_d_n8, assign24230_e18692_d_n9, assign24230_e18692_d_n10, assign24230_e18692_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) {
        let assign24230_e18690: f64 = (1.6021918e-19 * locals.var_ef_nsubc);
        (assign24230_e18690, (1.6021918e-19 * locals.var_ef_nsubc_dn0), (1.6021918e-19 * locals.var_ef_nsubc_dn2), (1.6021918e-19 * locals.var_ef_nsubc_dn4), (1.6021918e-19 * locals.var_ef_nsubc_dn5), (1.6021918e-19 * locals.var_ef_nsubc_dn6), (1.6021918e-19 * locals.var_ef_nsubc_dn7), (1.6021918e-19 * locals.var_ef_nsubc_dn8), (1.6021918e-19 * locals.var_ef_nsubc_dn9), (1.6021918e-19 * locals.var_ef_nsubc_dn10), (1.6021918e-19 * locals.var_ef_nsubc_dn13),)
    } else {
        (locals.var_q_nsub__blk544, locals.var_q_nsub__blk544_dn0, locals.var_q_nsub__blk544_dn2, locals.var_q_nsub__blk544_dn4, locals.var_q_nsub__blk544_dn5, locals.var_q_nsub__blk544_dn6, locals.var_q_nsub__blk544_dn7, locals.var_q_nsub__blk544_dn8, locals.var_q_nsub__blk544_dn9, locals.var_q_nsub__blk544_dn10, locals.var_q_nsub__blk544_dn13,)
    }
};
        locals.var_q_nsub__blk544 = assign24230_e18692;
        locals.var_q_nsub__blk544_dn0 = assign24230_e18692_d_n0;
        locals.var_q_nsub__blk544_dn2 = assign24230_e18692_d_n2;
        locals.var_q_nsub__blk544_dn4 = assign24230_e18692_d_n4;
        locals.var_q_nsub__blk544_dn5 = assign24230_e18692_d_n5;
        locals.var_q_nsub__blk544_dn6 = assign24230_e18692_d_n6;
        locals.var_q_nsub__blk544_dn7 = assign24230_e18692_d_n7;
        locals.var_q_nsub__blk544_dn8 = assign24230_e18692_d_n8;
        locals.var_q_nsub__blk544_dn9 = assign24230_e18692_d_n9;
        locals.var_q_nsub__blk544_dn10 = assign24230_e18692_d_n10;
        locals.var_q_nsub__blk544_dn13 = assign24230_e18692_d_n13;
        locals.var_q_nsub__blk544_rv = 0.0;

        let (assign24240_e18700,) = {
    if ((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) {
        let assign24240_e18698: f64 = (1.6021918e-19 * 1.6021918e-19);
        (assign24240_e18698,)
    } else {
        (locals.var_c_qe2,)
    }
};
        locals.var_c_qe2 = assign24240_e18700;
        locals.var_c_qe2_rv = 0.0;

        let (assign24250_e18708,) = {
    if ((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) {
        let assign24250_e18706: f64 = (1.034943e-10 * 1.034943e-10);
        (assign24250_e18706,)
    } else {
        (locals.var_c_esi2,)
    }
};
        locals.var_c_esi2 = assign24250_e18708;
        locals.var_c_esi2_rv = 0.0;

        let (assign24260_e18716, assign24260_e18716_d_n0, assign24260_e18716_d_n2, assign24260_e18716_d_n4, assign24260_e18716_d_n5, assign24260_e18716_d_n6, assign24260_e18716_d_n7, assign24260_e18716_d_n8, assign24260_e18716_d_n9, assign24260_e18716_d_n10, assign24260_e18716_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) {
        let assign24260_e18714: f64 = (locals.var_uc_depthn * locals.var_uc_depthn);
        (assign24260_e18714, ((locals.var_uc_depthn_dn0 * locals.var_uc_depthn) + (locals.var_uc_depthn * locals.var_uc_depthn_dn0)), ((locals.var_uc_depthn_dn2 * locals.var_uc_depthn) + (locals.var_uc_depthn * locals.var_uc_depthn_dn2)), ((locals.var_uc_depthn_dn4 * locals.var_uc_depthn) + (locals.var_uc_depthn * locals.var_uc_depthn_dn4)), ((locals.var_uc_depthn_dn5 * locals.var_uc_depthn) + (locals.var_uc_depthn * locals.var_uc_depthn_dn5)), ((locals.var_uc_depthn_dn6 * locals.var_uc_depthn) + (locals.var_uc_depthn * locals.var_uc_depthn_dn6)), ((locals.var_uc_depthn_dn7 * locals.var_uc_depthn) + (locals.var_uc_depthn * locals.var_uc_depthn_dn7)), ((locals.var_uc_depthn_dn8 * locals.var_uc_depthn) + (locals.var_uc_depthn * locals.var_uc_depthn_dn8)), ((locals.var_uc_depthn_dn9 * locals.var_uc_depthn) + (locals.var_uc_depthn * locals.var_uc_depthn_dn9)), ((locals.var_uc_depthn_dn10 * locals.var_uc_depthn) + (locals.var_uc_depthn * locals.var_uc_depthn_dn10)), ((locals.var_uc_depthn_dn13 * locals.var_uc_depthn) + (locals.var_uc_depthn * locals.var_uc_depthn_dn13)),)
    } else {
        (locals.var_tn2, locals.var_tn2_dn0, locals.var_tn2_dn2, locals.var_tn2_dn4, locals.var_tn2_dn5, locals.var_tn2_dn6, locals.var_tn2_dn7, locals.var_tn2_dn8, locals.var_tn2_dn9, locals.var_tn2_dn10, locals.var_tn2_dn13,)
    }
};
        locals.var_tn2 = assign24260_e18716;
        locals.var_tn2_dn0 = assign24260_e18716_d_n0;
        locals.var_tn2_dn2 = assign24260_e18716_d_n2;
        locals.var_tn2_dn4 = assign24260_e18716_d_n4;
        locals.var_tn2_dn5 = assign24260_e18716_d_n5;
        locals.var_tn2_dn6 = assign24260_e18716_d_n6;
        locals.var_tn2_dn7 = assign24260_e18716_d_n7;
        locals.var_tn2_dn8 = assign24260_e18716_d_n8;
        locals.var_tn2_dn9 = assign24260_e18716_d_n9;
        locals.var_tn2_dn10 = assign24260_e18716_d_n10;
        locals.var_tn2_dn13 = assign24260_e18716_d_n13;
        locals.var_tn2_rv = 0.0;

        let (assign24270_e18726, assign24270_e18726_d_n0, assign24270_e18726_d_n2, assign24270_e18726_d_n4, assign24270_e18726_d_n5, assign24270_e18726_d_n6, assign24270_e18726_d_n7, assign24270_e18726_d_n8, assign24270_e18726_d_n9, assign24270_e18726_d_n10, assign24270_e18726_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) {
        let assign24270_e18722: f64 = (2.0 * 1.034943e-10);
        let assign24270_e18724: f64 = (assign24270_e18722 / locals.var_q_ndepm);
        (assign24270_e18724, (-((assign24270_e18722 * locals.var_q_ndepm_dn0) / (locals.var_q_ndepm * locals.var_q_ndepm))), (-((assign24270_e18722 * locals.var_q_ndepm_dn2) / (locals.var_q_ndepm * locals.var_q_ndepm))), (-((assign24270_e18722 * locals.var_q_ndepm_dn4) / (locals.var_q_ndepm * locals.var_q_ndepm))), (-((assign24270_e18722 * locals.var_q_ndepm_dn5) / (locals.var_q_ndepm * locals.var_q_ndepm))), (-((assign24270_e18722 * locals.var_q_ndepm_dn6) / (locals.var_q_ndepm * locals.var_q_ndepm))), (-((assign24270_e18722 * locals.var_q_ndepm_dn7) / (locals.var_q_ndepm * locals.var_q_ndepm))), (-((assign24270_e18722 * locals.var_q_ndepm_dn8) / (locals.var_q_ndepm * locals.var_q_ndepm))), (-((assign24270_e18722 * locals.var_q_ndepm_dn9) / (locals.var_q_ndepm * locals.var_q_ndepm))), (-((assign24270_e18722 * locals.var_q_ndepm_dn10) / (locals.var_q_ndepm * locals.var_q_ndepm))), (-((assign24270_e18722 * locals.var_q_ndepm_dn13) / (locals.var_q_ndepm * locals.var_q_ndepm))),)
    } else {
        (locals.var_c_2esipq_ndepm, locals.var_c_2esipq_ndepm_dn0, locals.var_c_2esipq_ndepm_dn2, locals.var_c_2esipq_ndepm_dn4, locals.var_c_2esipq_ndepm_dn5, locals.var_c_2esipq_ndepm_dn6, locals.var_c_2esipq_ndepm_dn7, locals.var_c_2esipq_ndepm_dn8, locals.var_c_2esipq_ndepm_dn9, locals.var_c_2esipq_ndepm_dn10, locals.var_c_2esipq_ndepm_dn13,)
    }
};
        locals.var_c_2esipq_ndepm = assign24270_e18726;
        locals.var_c_2esipq_ndepm_dn0 = assign24270_e18726_d_n0;
        locals.var_c_2esipq_ndepm_dn2 = assign24270_e18726_d_n2;
        locals.var_c_2esipq_ndepm_dn4 = assign24270_e18726_d_n4;
        locals.var_c_2esipq_ndepm_dn5 = assign24270_e18726_d_n5;
        locals.var_c_2esipq_ndepm_dn6 = assign24270_e18726_d_n6;
        locals.var_c_2esipq_ndepm_dn7 = assign24270_e18726_d_n7;
        locals.var_c_2esipq_ndepm_dn8 = assign24270_e18726_d_n8;
        locals.var_c_2esipq_ndepm_dn9 = assign24270_e18726_d_n9;
        locals.var_c_2esipq_ndepm_dn10 = assign24270_e18726_d_n10;
        locals.var_c_2esipq_ndepm_dn13 = assign24270_e18726_d_n13;
        locals.var_c_2esipq_ndepm_rv = 0.0;

        let (assign24280_e18736, assign24280_e18736_d_n0, assign24280_e18736_d_n2, assign24280_e18736_d_n4, assign24280_e18736_d_n5, assign24280_e18736_d_n6, assign24280_e18736_d_n7, assign24280_e18736_d_n8, assign24280_e18736_d_n9, assign24280_e18736_d_n10, assign24280_e18736_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) {
        let assign24280_e18733: f64 = (2.0 * 1.034943e-10);
        let assign24280_e18734: f64 = (locals.var_q_ndepm / assign24280_e18733);
        (assign24280_e18734, (locals.var_q_ndepm_dn0 / assign24280_e18733), (locals.var_q_ndepm_dn2 / assign24280_e18733), (locals.var_q_ndepm_dn4 / assign24280_e18733), (locals.var_q_ndepm_dn5 / assign24280_e18733), (locals.var_q_ndepm_dn6 / assign24280_e18733), (locals.var_q_ndepm_dn7 / assign24280_e18733), (locals.var_q_ndepm_dn8 / assign24280_e18733), (locals.var_q_ndepm_dn9 / assign24280_e18733), (locals.var_q_ndepm_dn10 / assign24280_e18733), (locals.var_q_ndepm_dn13 / assign24280_e18733),)
    } else {
        (locals.var_c_2esipq_ndepm_inv, locals.var_c_2esipq_ndepm_inv_dn0, locals.var_c_2esipq_ndepm_inv_dn2, locals.var_c_2esipq_ndepm_inv_dn4, locals.var_c_2esipq_ndepm_inv_dn5, locals.var_c_2esipq_ndepm_inv_dn6, locals.var_c_2esipq_ndepm_inv_dn7, locals.var_c_2esipq_ndepm_inv_dn8, locals.var_c_2esipq_ndepm_inv_dn9, locals.var_c_2esipq_ndepm_inv_dn10, locals.var_c_2esipq_ndepm_inv_dn13,)
    }
};
        locals.var_c_2esipq_ndepm_inv = assign24280_e18736;
        locals.var_c_2esipq_ndepm_inv_dn0 = assign24280_e18736_d_n0;
        locals.var_c_2esipq_ndepm_inv_dn2 = assign24280_e18736_d_n2;
        locals.var_c_2esipq_ndepm_inv_dn4 = assign24280_e18736_d_n4;
        locals.var_c_2esipq_ndepm_inv_dn5 = assign24280_e18736_d_n5;
        locals.var_c_2esipq_ndepm_inv_dn6 = assign24280_e18736_d_n6;
        locals.var_c_2esipq_ndepm_inv_dn7 = assign24280_e18736_d_n7;
        locals.var_c_2esipq_ndepm_inv_dn8 = assign24280_e18736_d_n8;
        locals.var_c_2esipq_ndepm_inv_dn9 = assign24280_e18736_d_n9;
        locals.var_c_2esipq_ndepm_inv_dn10 = assign24280_e18736_d_n10;
        locals.var_c_2esipq_ndepm_inv_dn13 = assign24280_e18736_d_n13;
        locals.var_c_2esipq_ndepm_inv_rv = 0.0;

        let (assign24290_e18746, assign24290_e18746_d_n0, assign24290_e18746_d_n2, assign24290_e18746_d_n4, assign24290_e18746_d_n5, assign24290_e18746_d_n6, assign24290_e18746_d_n7, assign24290_e18746_d_n8, assign24290_e18746_d_n9, assign24290_e18746_d_n10, assign24290_e18746_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) {
        let assign24290_e18742: f64 = (2.0 * 1.034943e-10);
        let assign24290_e18744: f64 = (assign24290_e18742 * locals.var_q_ndepm);
        (assign24290_e18744, (assign24290_e18742 * locals.var_q_ndepm_dn0), (assign24290_e18742 * locals.var_q_ndepm_dn2), (assign24290_e18742 * locals.var_q_ndepm_dn4), (assign24290_e18742 * locals.var_q_ndepm_dn5), (assign24290_e18742 * locals.var_q_ndepm_dn6), (assign24290_e18742 * locals.var_q_ndepm_dn7), (assign24290_e18742 * locals.var_q_ndepm_dn8), (assign24290_e18742 * locals.var_q_ndepm_dn9), (assign24290_e18742 * locals.var_q_ndepm_dn10), (assign24290_e18742 * locals.var_q_ndepm_dn13),)
    } else {
        (locals.var_c_2esi_q_ndepm, locals.var_c_2esi_q_ndepm_dn0, locals.var_c_2esi_q_ndepm_dn2, locals.var_c_2esi_q_ndepm_dn4, locals.var_c_2esi_q_ndepm_dn5, locals.var_c_2esi_q_ndepm_dn6, locals.var_c_2esi_q_ndepm_dn7, locals.var_c_2esi_q_ndepm_dn8, locals.var_c_2esi_q_ndepm_dn9, locals.var_c_2esi_q_ndepm_dn10, locals.var_c_2esi_q_ndepm_dn13,)
    }
};
        locals.var_c_2esi_q_ndepm = assign24290_e18746;
        locals.var_c_2esi_q_ndepm_dn0 = assign24290_e18746_d_n0;
        locals.var_c_2esi_q_ndepm_dn2 = assign24290_e18746_d_n2;
        locals.var_c_2esi_q_ndepm_dn4 = assign24290_e18746_d_n4;
        locals.var_c_2esi_q_ndepm_dn5 = assign24290_e18746_d_n5;
        locals.var_c_2esi_q_ndepm_dn6 = assign24290_e18746_d_n6;
        locals.var_c_2esi_q_ndepm_dn7 = assign24290_e18746_d_n7;
        locals.var_c_2esi_q_ndepm_dn8 = assign24290_e18746_d_n8;
        locals.var_c_2esi_q_ndepm_dn9 = assign24290_e18746_d_n9;
        locals.var_c_2esi_q_ndepm_dn10 = assign24290_e18746_d_n10;
        locals.var_c_2esi_q_ndepm_dn13 = assign24290_e18746_d_n13;
        locals.var_c_2esi_q_ndepm_rv = 0.0;

        let (assign24300_e18756, assign24300_e18756_d_n0, assign24300_e18756_d_n2, assign24300_e18756_d_n4, assign24300_e18756_d_n5, assign24300_e18756_d_n6, assign24300_e18756_d_n7, assign24300_e18756_d_n8, assign24300_e18756_d_n9, assign24300_e18756_d_n10, assign24300_e18756_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) {
        let assign24300_e18752: f64 = (2.0 * 1.034943e-10);
        let assign24300_e18754: f64 = (assign24300_e18752 / locals.var_q_nsub__blk544);
        (assign24300_e18754, (-((assign24300_e18752 * locals.var_q_nsub__blk544_dn0) / (locals.var_q_nsub__blk544 * locals.var_q_nsub__blk544))), (-((assign24300_e18752 * locals.var_q_nsub__blk544_dn2) / (locals.var_q_nsub__blk544 * locals.var_q_nsub__blk544))), (-((assign24300_e18752 * locals.var_q_nsub__blk544_dn4) / (locals.var_q_nsub__blk544 * locals.var_q_nsub__blk544))), (-((assign24300_e18752 * locals.var_q_nsub__blk544_dn5) / (locals.var_q_nsub__blk544 * locals.var_q_nsub__blk544))), (-((assign24300_e18752 * locals.var_q_nsub__blk544_dn6) / (locals.var_q_nsub__blk544 * locals.var_q_nsub__blk544))), (-((assign24300_e18752 * locals.var_q_nsub__blk544_dn7) / (locals.var_q_nsub__blk544 * locals.var_q_nsub__blk544))), (-((assign24300_e18752 * locals.var_q_nsub__blk544_dn8) / (locals.var_q_nsub__blk544 * locals.var_q_nsub__blk544))), (-((assign24300_e18752 * locals.var_q_nsub__blk544_dn9) / (locals.var_q_nsub__blk544 * locals.var_q_nsub__blk544))), (-((assign24300_e18752 * locals.var_q_nsub__blk544_dn10) / (locals.var_q_nsub__blk544 * locals.var_q_nsub__blk544))), (-((assign24300_e18752 * locals.var_q_nsub__blk544_dn13) / (locals.var_q_nsub__blk544 * locals.var_q_nsub__blk544))),)
    } else {
        (locals.var_c_2esipq_nsub, locals.var_c_2esipq_nsub_dn0, locals.var_c_2esipq_nsub_dn2, locals.var_c_2esipq_nsub_dn4, locals.var_c_2esipq_nsub_dn5, locals.var_c_2esipq_nsub_dn6, locals.var_c_2esipq_nsub_dn7, locals.var_c_2esipq_nsub_dn8, locals.var_c_2esipq_nsub_dn9, locals.var_c_2esipq_nsub_dn10, locals.var_c_2esipq_nsub_dn13,)
    }
};
        locals.var_c_2esipq_nsub = assign24300_e18756;
        locals.var_c_2esipq_nsub_dn0 = assign24300_e18756_d_n0;
        locals.var_c_2esipq_nsub_dn2 = assign24300_e18756_d_n2;
        locals.var_c_2esipq_nsub_dn4 = assign24300_e18756_d_n4;
        locals.var_c_2esipq_nsub_dn5 = assign24300_e18756_d_n5;
        locals.var_c_2esipq_nsub_dn6 = assign24300_e18756_d_n6;
        locals.var_c_2esipq_nsub_dn7 = assign24300_e18756_d_n7;
        locals.var_c_2esipq_nsub_dn8 = assign24300_e18756_d_n8;
        locals.var_c_2esipq_nsub_dn9 = assign24300_e18756_d_n9;
        locals.var_c_2esipq_nsub_dn10 = assign24300_e18756_d_n10;
        locals.var_c_2esipq_nsub_dn13 = assign24300_e18756_d_n13;
        locals.var_c_2esipq_nsub_rv = 0.0;

        let (assign24310_e18766, assign24310_e18766_d_n0, assign24310_e18766_d_n2, assign24310_e18766_d_n4, assign24310_e18766_d_n5, assign24310_e18766_d_n6, assign24310_e18766_d_n7, assign24310_e18766_d_n8, assign24310_e18766_d_n9, assign24310_e18766_d_n10, assign24310_e18766_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) {
        let assign24310_e18763: f64 = (2.0 * 1.034943e-10);
        let assign24310_e18764: f64 = (locals.var_q_nsub__blk544 / assign24310_e18763);
        (assign24310_e18764, (locals.var_q_nsub__blk544_dn0 / assign24310_e18763), (locals.var_q_nsub__blk544_dn2 / assign24310_e18763), (locals.var_q_nsub__blk544_dn4 / assign24310_e18763), (locals.var_q_nsub__blk544_dn5 / assign24310_e18763), (locals.var_q_nsub__blk544_dn6 / assign24310_e18763), (locals.var_q_nsub__blk544_dn7 / assign24310_e18763), (locals.var_q_nsub__blk544_dn8 / assign24310_e18763), (locals.var_q_nsub__blk544_dn9 / assign24310_e18763), (locals.var_q_nsub__blk544_dn10 / assign24310_e18763), (locals.var_q_nsub__blk544_dn13 / assign24310_e18763),)
    } else {
        (locals.var_c_2esipq_nsub_inv, locals.var_c_2esipq_nsub_inv_dn0, locals.var_c_2esipq_nsub_inv_dn2, locals.var_c_2esipq_nsub_inv_dn4, locals.var_c_2esipq_nsub_inv_dn5, locals.var_c_2esipq_nsub_inv_dn6, locals.var_c_2esipq_nsub_inv_dn7, locals.var_c_2esipq_nsub_inv_dn8, locals.var_c_2esipq_nsub_inv_dn9, locals.var_c_2esipq_nsub_inv_dn10, locals.var_c_2esipq_nsub_inv_dn13,)
    }
};
        locals.var_c_2esipq_nsub_inv = assign24310_e18766;
        locals.var_c_2esipq_nsub_inv_dn0 = assign24310_e18766_d_n0;
        locals.var_c_2esipq_nsub_inv_dn2 = assign24310_e18766_d_n2;
        locals.var_c_2esipq_nsub_inv_dn4 = assign24310_e18766_d_n4;
        locals.var_c_2esipq_nsub_inv_dn5 = assign24310_e18766_d_n5;
        locals.var_c_2esipq_nsub_inv_dn6 = assign24310_e18766_d_n6;
        locals.var_c_2esipq_nsub_inv_dn7 = assign24310_e18766_d_n7;
        locals.var_c_2esipq_nsub_inv_dn8 = assign24310_e18766_d_n8;
        locals.var_c_2esipq_nsub_inv_dn9 = assign24310_e18766_d_n9;
        locals.var_c_2esipq_nsub_inv_dn10 = assign24310_e18766_d_n10;
        locals.var_c_2esipq_nsub_inv_dn13 = assign24310_e18766_d_n13;
        locals.var_c_2esipq_nsub_inv_rv = 0.0;

        let (assign24320_e18774, assign24320_e18774_d_n0, assign24320_e18774_d_n2, assign24320_e18774_d_n4, assign24320_e18774_d_n5, assign24320_e18774_d_n6, assign24320_e18774_d_n7, assign24320_e18774_d_n8, assign24320_e18774_d_n9, assign24320_e18774_d_n10, assign24320_e18774_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) {
        let assign24320_e18772: f64 = (locals.var_uc_ndepm / locals.var_ef_nsubc);
        (assign24320_e18772, (((locals.var_uc_ndepm_dn0 * locals.var_ef_nsubc) - (locals.var_uc_ndepm * locals.var_ef_nsubc_dn0)) / (locals.var_ef_nsubc * locals.var_ef_nsubc)), (((locals.var_uc_ndepm_dn2 * locals.var_ef_nsubc) - (locals.var_uc_ndepm * locals.var_ef_nsubc_dn2)) / (locals.var_ef_nsubc * locals.var_ef_nsubc)), (((locals.var_uc_ndepm_dn4 * locals.var_ef_nsubc) - (locals.var_uc_ndepm * locals.var_ef_nsubc_dn4)) / (locals.var_ef_nsubc * locals.var_ef_nsubc)), (((locals.var_uc_ndepm_dn5 * locals.var_ef_nsubc) - (locals.var_uc_ndepm * locals.var_ef_nsubc_dn5)) / (locals.var_ef_nsubc * locals.var_ef_nsubc)), (((locals.var_uc_ndepm_dn6 * locals.var_ef_nsubc) - (locals.var_uc_ndepm * locals.var_ef_nsubc_dn6)) / (locals.var_ef_nsubc * locals.var_ef_nsubc)), (((locals.var_uc_ndepm_dn7 * locals.var_ef_nsubc) - (locals.var_uc_ndepm * locals.var_ef_nsubc_dn7)) / (locals.var_ef_nsubc * locals.var_ef_nsubc)), (((locals.var_uc_ndepm_dn8 * locals.var_ef_nsubc) - (locals.var_uc_ndepm * locals.var_ef_nsubc_dn8)) / (locals.var_ef_nsubc * locals.var_ef_nsubc)), (((locals.var_uc_ndepm_dn9 * locals.var_ef_nsubc) - (locals.var_uc_ndepm * locals.var_ef_nsubc_dn9)) / (locals.var_ef_nsubc * locals.var_ef_nsubc)), (((locals.var_uc_ndepm_dn10 * locals.var_ef_nsubc) - (locals.var_uc_ndepm * locals.var_ef_nsubc_dn10)) / (locals.var_ef_nsubc * locals.var_ef_nsubc)), (((locals.var_uc_ndepm_dn13 * locals.var_ef_nsubc) - (locals.var_uc_ndepm * locals.var_ef_nsubc_dn13)) / (locals.var_ef_nsubc * locals.var_ef_nsubc)),)
    } else {
        (locals.var_ndepmpnsub, locals.var_ndepmpnsub_dn0, locals.var_ndepmpnsub_dn2, locals.var_ndepmpnsub_dn4, locals.var_ndepmpnsub_dn5, locals.var_ndepmpnsub_dn6, locals.var_ndepmpnsub_dn7, locals.var_ndepmpnsub_dn8, locals.var_ndepmpnsub_dn9, locals.var_ndepmpnsub_dn10, locals.var_ndepmpnsub_dn13,)
    }
};
        locals.var_ndepmpnsub = assign24320_e18774;
        locals.var_ndepmpnsub_dn0 = assign24320_e18774_d_n0;
        locals.var_ndepmpnsub_dn2 = assign24320_e18774_d_n2;
        locals.var_ndepmpnsub_dn4 = assign24320_e18774_d_n4;
        locals.var_ndepmpnsub_dn5 = assign24320_e18774_d_n5;
        locals.var_ndepmpnsub_dn6 = assign24320_e18774_d_n6;
        locals.var_ndepmpnsub_dn7 = assign24320_e18774_d_n7;
        locals.var_ndepmpnsub_dn8 = assign24320_e18774_d_n8;
        locals.var_ndepmpnsub_dn9 = assign24320_e18774_d_n9;
        locals.var_ndepmpnsub_dn10 = assign24320_e18774_d_n10;
        locals.var_ndepmpnsub_dn13 = assign24320_e18774_d_n13;
        locals.var_ndepmpnsub_rv = 0.0;

        let (assign24330_e18784, assign24330_e18784_d_n0, assign24330_e18784_d_n2, assign24330_e18784_d_n4, assign24330_e18784_d_n5, assign24330_e18784_d_n6, assign24330_e18784_d_n7, assign24330_e18784_d_n8, assign24330_e18784_d_n9, assign24330_e18784_d_n10, assign24330_e18784_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) {
        let assign24330_e18781: f64 = (1.0 + locals.var_ndepmpnsub);
        let assign24330_e18782: f64 = (1.0 / assign24330_e18781);
        (assign24330_e18782, (-(locals.var_ndepmpnsub_dn0 / (assign24330_e18781 * assign24330_e18781))), (-(locals.var_ndepmpnsub_dn2 / (assign24330_e18781 * assign24330_e18781))), (-(locals.var_ndepmpnsub_dn4 / (assign24330_e18781 * assign24330_e18781))), (-(locals.var_ndepmpnsub_dn5 / (assign24330_e18781 * assign24330_e18781))), (-(locals.var_ndepmpnsub_dn6 / (assign24330_e18781 * assign24330_e18781))), (-(locals.var_ndepmpnsub_dn7 / (assign24330_e18781 * assign24330_e18781))), (-(locals.var_ndepmpnsub_dn8 / (assign24330_e18781 * assign24330_e18781))), (-(locals.var_ndepmpnsub_dn9 / (assign24330_e18781 * assign24330_e18781))), (-(locals.var_ndepmpnsub_dn10 / (assign24330_e18781 * assign24330_e18781))), (-(locals.var_ndepmpnsub_dn13 / (assign24330_e18781 * assign24330_e18781))),)
    } else {
        (locals.var_ndepmpnsub_inv1, locals.var_ndepmpnsub_inv1_dn0, locals.var_ndepmpnsub_inv1_dn2, locals.var_ndepmpnsub_inv1_dn4, locals.var_ndepmpnsub_inv1_dn5, locals.var_ndepmpnsub_inv1_dn6, locals.var_ndepmpnsub_inv1_dn7, locals.var_ndepmpnsub_inv1_dn8, locals.var_ndepmpnsub_inv1_dn9, locals.var_ndepmpnsub_inv1_dn10, locals.var_ndepmpnsub_inv1_dn13,)
    }
};
        locals.var_ndepmpnsub_inv1 = assign24330_e18784;
        locals.var_ndepmpnsub_inv1_dn0 = assign24330_e18784_d_n0;
        locals.var_ndepmpnsub_inv1_dn2 = assign24330_e18784_d_n2;
        locals.var_ndepmpnsub_inv1_dn4 = assign24330_e18784_d_n4;
        locals.var_ndepmpnsub_inv1_dn5 = assign24330_e18784_d_n5;
        locals.var_ndepmpnsub_inv1_dn6 = assign24330_e18784_d_n6;
        locals.var_ndepmpnsub_inv1_dn7 = assign24330_e18784_d_n7;
        locals.var_ndepmpnsub_inv1_dn8 = assign24330_e18784_d_n8;
        locals.var_ndepmpnsub_inv1_dn9 = assign24330_e18784_d_n9;
        locals.var_ndepmpnsub_inv1_dn10 = assign24330_e18784_d_n10;
        locals.var_ndepmpnsub_inv1_dn13 = assign24330_e18784_d_n13;
        locals.var_ndepmpnsub_inv1_rv = 0.0;

        let (assign24340_e18792,) = {
    if ((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) {
        let assign24340_e18790: f64 = (1e-12 * 1000.0);
        (assign24340_e18790,)
    } else {
        (locals.var_ps_conv3,)
    }
};
        locals.var_ps_conv3 = assign24340_e18792;
        locals.var_ps_conv3_rv = 0.0;

        let (assign24350_e18800,) = {
    if ((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) {
        let assign24350_e18798: f64 = (1e-10 * 1000.0);
        (assign24350_e18798,)
    } else {
        (locals.var_ps_conv23,)
    }
};
        locals.var_ps_conv23 = assign24350_e18800;
        locals.var_ps_conv23_rv = 0.0;

        let (assign24360_e18806, assign24360_e18806_d_n0, assign24360_e18806_d_n2, assign24360_e18806_d_n4, assign24360_e18806_d_n5, assign24360_e18806_d_n6, assign24360_e18806_d_n7, assign24360_e18806_d_n8, assign24360_e18806_d_n9, assign24360_e18806_d_n10, assign24360_e18806_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_phi_s0_dep, locals.var_phi_s0_dep_dn0, locals.var_phi_s0_dep_dn2, locals.var_phi_s0_dep_dn4, locals.var_phi_s0_dep_dn5, locals.var_phi_s0_dep_dn6, locals.var_phi_s0_dep_dn7, locals.var_phi_s0_dep_dn8, locals.var_phi_s0_dep_dn9, locals.var_phi_s0_dep_dn10, locals.var_phi_s0_dep_dn13,)
    }
};
        locals.var_phi_s0_dep = assign24360_e18806;
        locals.var_phi_s0_dep_dn0 = assign24360_e18806_d_n0;
        locals.var_phi_s0_dep_dn2 = assign24360_e18806_d_n2;
        locals.var_phi_s0_dep_dn4 = assign24360_e18806_d_n4;
        locals.var_phi_s0_dep_dn5 = assign24360_e18806_d_n5;
        locals.var_phi_s0_dep_dn6 = assign24360_e18806_d_n6;
        locals.var_phi_s0_dep_dn7 = assign24360_e18806_d_n7;
        locals.var_phi_s0_dep_dn8 = assign24360_e18806_d_n8;
        locals.var_phi_s0_dep_dn9 = assign24360_e18806_d_n9;
        locals.var_phi_s0_dep_dn10 = assign24360_e18806_d_n10;
        locals.var_phi_s0_dep_dn13 = assign24360_e18806_d_n13;
        locals.var_phi_s0_dep_rv = 0.0;

        let (assign24370_e18812, assign24370_e18812_d_n0, assign24370_e18812_d_n2, assign24370_e18812_d_n4, assign24370_e18812_d_n5, assign24370_e18812_d_n6, assign24370_e18812_d_n7, assign24370_e18812_d_n8, assign24370_e18812_d_n9, assign24370_e18812_d_n10, assign24370_e18812_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_phi_sl_dep, locals.var_phi_sl_dep_dn0, locals.var_phi_sl_dep_dn2, locals.var_phi_sl_dep_dn4, locals.var_phi_sl_dep_dn5, locals.var_phi_sl_dep_dn6, locals.var_phi_sl_dep_dn7, locals.var_phi_sl_dep_dn8, locals.var_phi_sl_dep_dn9, locals.var_phi_sl_dep_dn10, locals.var_phi_sl_dep_dn13,)
    }
};
        locals.var_phi_sl_dep = assign24370_e18812;
        locals.var_phi_sl_dep_dn0 = assign24370_e18812_d_n0;
        locals.var_phi_sl_dep_dn2 = assign24370_e18812_d_n2;
        locals.var_phi_sl_dep_dn4 = assign24370_e18812_d_n4;
        locals.var_phi_sl_dep_dn5 = assign24370_e18812_d_n5;
        locals.var_phi_sl_dep_dn6 = assign24370_e18812_d_n6;
        locals.var_phi_sl_dep_dn7 = assign24370_e18812_d_n7;
        locals.var_phi_sl_dep_dn8 = assign24370_e18812_d_n8;
        locals.var_phi_sl_dep_dn9 = assign24370_e18812_d_n9;
        locals.var_phi_sl_dep_dn10 = assign24370_e18812_d_n10;
        locals.var_phi_sl_dep_dn13 = assign24370_e18812_d_n13;
        locals.var_phi_sl_dep_rv = 0.0;

        let (assign24380_e18818, assign24380_e18818_d_n0, assign24380_e18818_d_n2, assign24380_e18818_d_n4, assign24380_e18818_d_n5, assign24380_e18818_d_n6, assign24380_e18818_d_n7, assign24380_e18818_d_n8, assign24380_e18818_d_n9, assign24380_e18818_d_n10, assign24380_e18818_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_q_s0, locals.var_q_s0_dn0, locals.var_q_s0_dn2, locals.var_q_s0_dn4, locals.var_q_s0_dn5, locals.var_q_s0_dn6, locals.var_q_s0_dn7, locals.var_q_s0_dn8, locals.var_q_s0_dn9, locals.var_q_s0_dn10, locals.var_q_s0_dn13,)
    }
};
        locals.var_q_s0 = assign24380_e18818;
        locals.var_q_s0_dn0 = assign24380_e18818_d_n0;
        locals.var_q_s0_dn2 = assign24380_e18818_d_n2;
        locals.var_q_s0_dn4 = assign24380_e18818_d_n4;
        locals.var_q_s0_dn5 = assign24380_e18818_d_n5;
        locals.var_q_s0_dn6 = assign24380_e18818_d_n6;
        locals.var_q_s0_dn7 = assign24380_e18818_d_n7;
        locals.var_q_s0_dn8 = assign24380_e18818_d_n8;
        locals.var_q_s0_dn9 = assign24380_e18818_d_n9;
        locals.var_q_s0_dn10 = assign24380_e18818_d_n10;
        locals.var_q_s0_dn13 = assign24380_e18818_d_n13;
        locals.var_q_s0_rv = 0.0;

        let (assign24390_e18824, assign24390_e18824_d_n0, assign24390_e18824_d_n2, assign24390_e18824_d_n4, assign24390_e18824_d_n5, assign24390_e18824_d_n6, assign24390_e18824_d_n7, assign24390_e18824_d_n8, assign24390_e18824_d_n9, assign24390_e18824_d_n10, assign24390_e18824_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_q_sl, locals.var_q_sl_dn0, locals.var_q_sl_dn2, locals.var_q_sl_dn4, locals.var_q_sl_dn5, locals.var_q_sl_dn6, locals.var_q_sl_dn7, locals.var_q_sl_dn8, locals.var_q_sl_dn9, locals.var_q_sl_dn10, locals.var_q_sl_dn13,)
    }
};
        locals.var_q_sl = assign24390_e18824;
        locals.var_q_sl_dn0 = assign24390_e18824_d_n0;
        locals.var_q_sl_dn2 = assign24390_e18824_d_n2;
        locals.var_q_sl_dn4 = assign24390_e18824_d_n4;
        locals.var_q_sl_dn5 = assign24390_e18824_d_n5;
        locals.var_q_sl_dn6 = assign24390_e18824_d_n6;
        locals.var_q_sl_dn7 = assign24390_e18824_d_n7;
        locals.var_q_sl_dn8 = assign24390_e18824_d_n8;
        locals.var_q_sl_dn9 = assign24390_e18824_d_n9;
        locals.var_q_sl_dn10 = assign24390_e18824_d_n10;
        locals.var_q_sl_dn13 = assign24390_e18824_d_n13;
        locals.var_q_sl_rv = 0.0;

        let (assign24400_e18830, assign24400_e18830_d_n0, assign24400_e18830_d_n2, assign24400_e18830_d_n4, assign24400_e18830_d_n5, assign24400_e18830_d_n6, assign24400_e18830_d_n7, assign24400_e18830_d_n8, assign24400_e18830_d_n9, assign24400_e18830_d_n10, assign24400_e18830_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_q_s0_dep, locals.var_q_s0_dep_dn0, locals.var_q_s0_dep_dn2, locals.var_q_s0_dep_dn4, locals.var_q_s0_dep_dn5, locals.var_q_s0_dep_dn6, locals.var_q_s0_dep_dn7, locals.var_q_s0_dep_dn8, locals.var_q_s0_dep_dn9, locals.var_q_s0_dep_dn10, locals.var_q_s0_dep_dn13,)
    }
};
        locals.var_q_s0_dep = assign24400_e18830;
        locals.var_q_s0_dep_dn0 = assign24400_e18830_d_n0;
        locals.var_q_s0_dep_dn2 = assign24400_e18830_d_n2;
        locals.var_q_s0_dep_dn4 = assign24400_e18830_d_n4;
        locals.var_q_s0_dep_dn5 = assign24400_e18830_d_n5;
        locals.var_q_s0_dep_dn6 = assign24400_e18830_d_n6;
        locals.var_q_s0_dep_dn7 = assign24400_e18830_d_n7;
        locals.var_q_s0_dep_dn8 = assign24400_e18830_d_n8;
        locals.var_q_s0_dep_dn9 = assign24400_e18830_d_n9;
        locals.var_q_s0_dep_dn10 = assign24400_e18830_d_n10;
        locals.var_q_s0_dep_dn13 = assign24400_e18830_d_n13;
        locals.var_q_s0_dep_rv = 0.0;

        let (assign24410_e18836, assign24410_e18836_d_n0, assign24410_e18836_d_n2, assign24410_e18836_d_n4, assign24410_e18836_d_n5, assign24410_e18836_d_n6, assign24410_e18836_d_n7, assign24410_e18836_d_n8, assign24410_e18836_d_n9, assign24410_e18836_d_n10, assign24410_e18836_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_q_sl_dep, locals.var_q_sl_dep_dn0, locals.var_q_sl_dep_dn2, locals.var_q_sl_dep_dn4, locals.var_q_sl_dep_dn5, locals.var_q_sl_dep_dn6, locals.var_q_sl_dep_dn7, locals.var_q_sl_dep_dn8, locals.var_q_sl_dep_dn9, locals.var_q_sl_dep_dn10, locals.var_q_sl_dep_dn13,)
    }
};
        locals.var_q_sl_dep = assign24410_e18836;
        locals.var_q_sl_dep_dn0 = assign24410_e18836_d_n0;
        locals.var_q_sl_dep_dn2 = assign24410_e18836_d_n2;
        locals.var_q_sl_dep_dn4 = assign24410_e18836_d_n4;
        locals.var_q_sl_dep_dn5 = assign24410_e18836_d_n5;
        locals.var_q_sl_dep_dn6 = assign24410_e18836_d_n6;
        locals.var_q_sl_dep_dn7 = assign24410_e18836_d_n7;
        locals.var_q_sl_dep_dn8 = assign24410_e18836_d_n8;
        locals.var_q_sl_dep_dn9 = assign24410_e18836_d_n9;
        locals.var_q_sl_dep_dn10 = assign24410_e18836_d_n10;
        locals.var_q_sl_dep_dn13 = assign24410_e18836_d_n13;
        locals.var_q_sl_dep_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_65(
        locals: &mut StampLocals,
    ) {
        let (assign24420_e18842, assign24420_e18842_d_n0, assign24420_e18842_d_n2, assign24420_e18842_d_n4, assign24420_e18842_d_n5, assign24420_e18842_d_n6, assign24420_e18842_d_n7, assign24420_e18842_d_n8, assign24420_e18842_d_n9, assign24420_e18842_d_n10, assign24420_e18842_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_q_b0_dep, locals.var_q_b0_dep_dn0, locals.var_q_b0_dep_dn2, locals.var_q_b0_dep_dn4, locals.var_q_b0_dep_dn5, locals.var_q_b0_dep_dn6, locals.var_q_b0_dep_dn7, locals.var_q_b0_dep_dn8, locals.var_q_b0_dep_dn9, locals.var_q_b0_dep_dn10, locals.var_q_b0_dep_dn13,)
    }
};
        locals.var_q_b0_dep = assign24420_e18842;
        locals.var_q_b0_dep_dn0 = assign24420_e18842_d_n0;
        locals.var_q_b0_dep_dn2 = assign24420_e18842_d_n2;
        locals.var_q_b0_dep_dn4 = assign24420_e18842_d_n4;
        locals.var_q_b0_dep_dn5 = assign24420_e18842_d_n5;
        locals.var_q_b0_dep_dn6 = assign24420_e18842_d_n6;
        locals.var_q_b0_dep_dn7 = assign24420_e18842_d_n7;
        locals.var_q_b0_dep_dn8 = assign24420_e18842_d_n8;
        locals.var_q_b0_dep_dn9 = assign24420_e18842_d_n9;
        locals.var_q_b0_dep_dn10 = assign24420_e18842_d_n10;
        locals.var_q_b0_dep_dn13 = assign24420_e18842_d_n13;
        locals.var_q_b0_dep_rv = 0.0;

        let (assign24430_e18848, assign24430_e18848_d_n0, assign24430_e18848_d_n2, assign24430_e18848_d_n4, assign24430_e18848_d_n5, assign24430_e18848_d_n6, assign24430_e18848_d_n7, assign24430_e18848_d_n8, assign24430_e18848_d_n9, assign24430_e18848_d_n10, assign24430_e18848_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_q_bl_dep, locals.var_q_bl_dep_dn0, locals.var_q_bl_dep_dn2, locals.var_q_bl_dep_dn4, locals.var_q_bl_dep_dn5, locals.var_q_bl_dep_dn6, locals.var_q_bl_dep_dn7, locals.var_q_bl_dep_dn8, locals.var_q_bl_dep_dn9, locals.var_q_bl_dep_dn10, locals.var_q_bl_dep_dn13,)
    }
};
        locals.var_q_bl_dep = assign24430_e18848;
        locals.var_q_bl_dep_dn0 = assign24430_e18848_d_n0;
        locals.var_q_bl_dep_dn2 = assign24430_e18848_d_n2;
        locals.var_q_bl_dep_dn4 = assign24430_e18848_d_n4;
        locals.var_q_bl_dep_dn5 = assign24430_e18848_d_n5;
        locals.var_q_bl_dep_dn6 = assign24430_e18848_d_n6;
        locals.var_q_bl_dep_dn7 = assign24430_e18848_d_n7;
        locals.var_q_bl_dep_dn8 = assign24430_e18848_d_n8;
        locals.var_q_bl_dep_dn9 = assign24430_e18848_d_n9;
        locals.var_q_bl_dep_dn10 = assign24430_e18848_d_n10;
        locals.var_q_bl_dep_dn13 = assign24430_e18848_d_n13;
        locals.var_q_bl_dep_rv = 0.0;

        let (assign24440_e18854, assign24440_e18854_d_n0, assign24440_e18854_d_n2, assign24440_e18854_d_n4, assign24440_e18854_d_n5, assign24440_e18854_d_n6, assign24440_e18854_d_n7, assign24440_e18854_d_n8, assign24440_e18854_d_n9, assign24440_e18854_d_n10, assign24440_e18854_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_q_sub0_dep, locals.var_q_sub0_dep_dn0, locals.var_q_sub0_dep_dn2, locals.var_q_sub0_dep_dn4, locals.var_q_sub0_dep_dn5, locals.var_q_sub0_dep_dn6, locals.var_q_sub0_dep_dn7, locals.var_q_sub0_dep_dn8, locals.var_q_sub0_dep_dn9, locals.var_q_sub0_dep_dn10, locals.var_q_sub0_dep_dn13,)
    }
};
        locals.var_q_sub0_dep = assign24440_e18854;
        locals.var_q_sub0_dep_dn0 = assign24440_e18854_d_n0;
        locals.var_q_sub0_dep_dn2 = assign24440_e18854_d_n2;
        locals.var_q_sub0_dep_dn4 = assign24440_e18854_d_n4;
        locals.var_q_sub0_dep_dn5 = assign24440_e18854_d_n5;
        locals.var_q_sub0_dep_dn6 = assign24440_e18854_d_n6;
        locals.var_q_sub0_dep_dn7 = assign24440_e18854_d_n7;
        locals.var_q_sub0_dep_dn8 = assign24440_e18854_d_n8;
        locals.var_q_sub0_dep_dn9 = assign24440_e18854_d_n9;
        locals.var_q_sub0_dep_dn10 = assign24440_e18854_d_n10;
        locals.var_q_sub0_dep_dn13 = assign24440_e18854_d_n13;
        locals.var_q_sub0_dep_rv = 0.0;

        let (assign24450_e18860, assign24450_e18860_d_n0, assign24450_e18860_d_n2, assign24450_e18860_d_n4, assign24450_e18860_d_n5, assign24450_e18860_d_n6, assign24450_e18860_d_n7, assign24450_e18860_d_n8, assign24450_e18860_d_n9, assign24450_e18860_d_n10, assign24450_e18860_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_q_subl_dep, locals.var_q_subl_dep_dn0, locals.var_q_subl_dep_dn2, locals.var_q_subl_dep_dn4, locals.var_q_subl_dep_dn5, locals.var_q_subl_dep_dn6, locals.var_q_subl_dep_dn7, locals.var_q_subl_dep_dn8, locals.var_q_subl_dep_dn9, locals.var_q_subl_dep_dn10, locals.var_q_subl_dep_dn13,)
    }
};
        locals.var_q_subl_dep = assign24450_e18860;
        locals.var_q_subl_dep_dn0 = assign24450_e18860_d_n0;
        locals.var_q_subl_dep_dn2 = assign24450_e18860_d_n2;
        locals.var_q_subl_dep_dn4 = assign24450_e18860_d_n4;
        locals.var_q_subl_dep_dn5 = assign24450_e18860_d_n5;
        locals.var_q_subl_dep_dn6 = assign24450_e18860_d_n6;
        locals.var_q_subl_dep_dn7 = assign24450_e18860_d_n7;
        locals.var_q_subl_dep_dn8 = assign24450_e18860_d_n8;
        locals.var_q_subl_dep_dn9 = assign24450_e18860_d_n9;
        locals.var_q_subl_dep_dn10 = assign24450_e18860_d_n10;
        locals.var_q_subl_dep_dn13 = assign24450_e18860_d_n13;
        locals.var_q_subl_dep_rv = 0.0;

        let (assign24460_e18866, assign24460_e18866_d_n0, assign24460_e18866_d_n2, assign24460_e18866_d_n4, assign24460_e18866_d_n5, assign24460_e18866_d_n6, assign24460_e18866_d_n7, assign24460_e18866_d_n8, assign24460_e18866_d_n9, assign24460_e18866_d_n10, assign24460_e18866_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_phib_ref, locals.var_phib_ref_dn0, locals.var_phib_ref_dn2, locals.var_phib_ref_dn4, locals.var_phib_ref_dn5, locals.var_phib_ref_dn6, locals.var_phib_ref_dn7, locals.var_phib_ref_dn8, locals.var_phib_ref_dn9, locals.var_phib_ref_dn10, locals.var_phib_ref_dn13,)
    }
};
        locals.var_phib_ref = assign24460_e18866;
        locals.var_phib_ref_dn0 = assign24460_e18866_d_n0;
        locals.var_phib_ref_dn2 = assign24460_e18866_d_n2;
        locals.var_phib_ref_dn4 = assign24460_e18866_d_n4;
        locals.var_phib_ref_dn5 = assign24460_e18866_d_n5;
        locals.var_phib_ref_dn6 = assign24460_e18866_d_n6;
        locals.var_phib_ref_dn7 = assign24460_e18866_d_n7;
        locals.var_phib_ref_dn8 = assign24460_e18866_d_n8;
        locals.var_phib_ref_dn9 = assign24460_e18866_d_n9;
        locals.var_phib_ref_dn10 = assign24460_e18866_d_n10;
        locals.var_phib_ref_dn13 = assign24460_e18866_d_n13;
        locals.var_phib_ref_rv = 0.0;

        let (assign24470_e18878, assign24470_e18878_d_n0, assign24470_e18878_d_n2, assign24470_e18878_d_n4, assign24470_e18878_d_n5, assign24470_e18878_d_n6, assign24470_e18878_d_n7, assign24470_e18878_d_n8, assign24470_e18878_d_n9, assign24470_e18878_d_n10, assign24470_e18878_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) {
        let assign24470_e18873: f64 = (10.0 * 2.220446049250313e-16);
        let assign24470_e18875: f64 = (assign24470_e18873 * 10000000.0);
        let assign24470_e18876: f64 = (locals.var_vgp + assign24470_e18875);
        (assign24470_e18876, locals.var_vgp_dn0, locals.var_vgp_dn2, locals.var_vgp_dn4, locals.var_vgp_dn5, locals.var_vgp_dn6, locals.var_vgp_dn7, locals.var_vgp_dn8, locals.var_vgp_dn9, locals.var_vgp_dn10, locals.var_vgp_dn13,)
    } else {
        (locals.var_vgp, locals.var_vgp_dn0, locals.var_vgp_dn2, locals.var_vgp_dn4, locals.var_vgp_dn5, locals.var_vgp_dn6, locals.var_vgp_dn7, locals.var_vgp_dn8, locals.var_vgp_dn9, locals.var_vgp_dn10, locals.var_vgp_dn13,)
    }
};
        locals.var_vgp = assign24470_e18878;
        locals.var_vgp_dn0 = assign24470_e18878_d_n0;
        locals.var_vgp_dn2 = assign24470_e18878_d_n2;
        locals.var_vgp_dn4 = assign24470_e18878_d_n4;
        locals.var_vgp_dn5 = assign24470_e18878_d_n5;
        locals.var_vgp_dn6 = assign24470_e18878_d_n6;
        locals.var_vgp_dn7 = assign24470_e18878_d_n7;
        locals.var_vgp_dn8 = assign24470_e18878_d_n8;
        locals.var_vgp_dn9 = assign24470_e18878_d_n9;
        locals.var_vgp_dn10 = assign24470_e18878_d_n10;
        locals.var_vgp_dn13 = assign24470_e18878_d_n13;
        locals.var_vgp_rv = 0.0;

        let (assign24480_e18890, assign24480_e18890_d_n0, assign24480_e18890_d_n2, assign24480_e18890_d_n4, assign24480_e18890_d_n5, assign24480_e18890_d_n6, assign24480_e18890_d_n7, assign24480_e18890_d_n8, assign24480_e18890_d_n9, assign24480_e18890_d_n10, assign24480_e18890_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) {
        let assign24480_e18884: f64 = (locals.var_cox * locals.var_cox);
        let __rspice_inv_cse_0: f64 = 1.0 / locals.var_cnst0;
        let assign24480_e18886: f64 = (assign24480_e18884 * __rspice_inv_cse_0);
        let assign24480_e18888: f64 = (assign24480_e18886 * __rspice_inv_cse_0);
        (assign24480_e18888, ((((((((locals.var_cox_dn0 * locals.var_cox) + (locals.var_cox * locals.var_cox_dn0)) * locals.var_cnst0) - (assign24480_e18884 * locals.var_cnst0_dn0)) / (locals.var_cnst0 * locals.var_cnst0)) * locals.var_cnst0) - (assign24480_e18886 * locals.var_cnst0_dn0)) / (locals.var_cnst0 * locals.var_cnst0)), ((((((((locals.var_cox_dn2 * locals.var_cox) + (locals.var_cox * locals.var_cox_dn2)) * locals.var_cnst0) - (assign24480_e18884 * locals.var_cnst0_dn2)) / (locals.var_cnst0 * locals.var_cnst0)) * locals.var_cnst0) - (assign24480_e18886 * locals.var_cnst0_dn2)) / (locals.var_cnst0 * locals.var_cnst0)), ((((((((locals.var_cox_dn4 * locals.var_cox) + (locals.var_cox * locals.var_cox_dn4)) * locals.var_cnst0) - (assign24480_e18884 * locals.var_cnst0_dn4)) / (locals.var_cnst0 * locals.var_cnst0)) * locals.var_cnst0) - (assign24480_e18886 * locals.var_cnst0_dn4)) / (locals.var_cnst0 * locals.var_cnst0)), ((((((((locals.var_cox_dn5 * locals.var_cox) + (locals.var_cox * locals.var_cox_dn5)) * locals.var_cnst0) - (assign24480_e18884 * locals.var_cnst0_dn5)) / (locals.var_cnst0 * locals.var_cnst0)) * locals.var_cnst0) - (assign24480_e18886 * locals.var_cnst0_dn5)) / (locals.var_cnst0 * locals.var_cnst0)), ((((((((locals.var_cox_dn6 * locals.var_cox) + (locals.var_cox * locals.var_cox_dn6)) * locals.var_cnst0) - (assign24480_e18884 * locals.var_cnst0_dn6)) / (locals.var_cnst0 * locals.var_cnst0)) * locals.var_cnst0) - (assign24480_e18886 * locals.var_cnst0_dn6)) / (locals.var_cnst0 * locals.var_cnst0)), ((((((((locals.var_cox_dn7 * locals.var_cox) + (locals.var_cox * locals.var_cox_dn7)) * locals.var_cnst0) - (assign24480_e18884 * locals.var_cnst0_dn7)) / (locals.var_cnst0 * locals.var_cnst0)) * locals.var_cnst0) - (assign24480_e18886 * locals.var_cnst0_dn7)) / (locals.var_cnst0 * locals.var_cnst0)), ((((((((locals.var_cox_dn8 * locals.var_cox) + (locals.var_cox * locals.var_cox_dn8)) * locals.var_cnst0) - (assign24480_e18884 * locals.var_cnst0_dn8)) / (locals.var_cnst0 * locals.var_cnst0)) * locals.var_cnst0) - (assign24480_e18886 * locals.var_cnst0_dn8)) / (locals.var_cnst0 * locals.var_cnst0)), ((((((((locals.var_cox_dn9 * locals.var_cox) + (locals.var_cox * locals.var_cox_dn9)) * locals.var_cnst0) - (assign24480_e18884 * locals.var_cnst0_dn9)) / (locals.var_cnst0 * locals.var_cnst0)) * locals.var_cnst0) - (assign24480_e18886 * locals.var_cnst0_dn9)) / (locals.var_cnst0 * locals.var_cnst0)), ((((((((locals.var_cox_dn10 * locals.var_cox) + (locals.var_cox * locals.var_cox_dn10)) * locals.var_cnst0) - (assign24480_e18884 * locals.var_cnst0_dn10)) / (locals.var_cnst0 * locals.var_cnst0)) * locals.var_cnst0) - (assign24480_e18886 * locals.var_cnst0_dn10)) / (locals.var_cnst0 * locals.var_cnst0)), ((((((((locals.var_cox_dn13 * locals.var_cox) + (locals.var_cox * locals.var_cox_dn13)) * locals.var_cnst0) - (assign24480_e18884 * locals.var_cnst0_dn13)) / (locals.var_cnst0 * locals.var_cnst0)) * locals.var_cnst0) - (assign24480_e18886 * locals.var_cnst0_dn13)) / (locals.var_cnst0 * locals.var_cnst0)),)
    } else {
        (locals.var_afact, locals.var_afact_dn0, locals.var_afact_dn2, locals.var_afact_dn4, locals.var_afact_dn5, locals.var_afact_dn6, locals.var_afact_dn7, locals.var_afact_dn8, locals.var_afact_dn9, locals.var_afact_dn10, locals.var_afact_dn13,)
    }
};
        locals.var_afact = assign24480_e18890;
        locals.var_afact_dn0 = assign24480_e18890_d_n0;
        locals.var_afact_dn2 = assign24480_e18890_d_n2;
        locals.var_afact_dn4 = assign24480_e18890_d_n4;
        locals.var_afact_dn5 = assign24480_e18890_d_n5;
        locals.var_afact_dn6 = assign24480_e18890_d_n6;
        locals.var_afact_dn7 = assign24480_e18890_d_n7;
        locals.var_afact_dn8 = assign24480_e18890_d_n8;
        locals.var_afact_dn9 = assign24480_e18890_d_n9;
        locals.var_afact_dn10 = assign24480_e18890_d_n10;
        locals.var_afact_dn13 = assign24480_e18890_d_n13;
        locals.var_afact_rv = 0.0;

        let (assign24490_e18902, assign24490_e18902_d_n0, assign24490_e18902_d_n2, assign24490_e18902_d_n4, assign24490_e18902_d_n5, assign24490_e18902_d_n6, assign24490_e18902_d_n7, assign24490_e18902_d_n8, assign24490_e18902_d_n9, assign24490_e18902_d_n10, assign24490_e18902_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) {
        let __rspice_inv_cse_1: f64 = 1.0 / locals.var_nin;
        let assign24490_e18896: f64 = (locals.var_afact * __rspice_inv_cse_1);
        let assign24490_e18898: f64 = (assign24490_e18896 * __rspice_inv_cse_1);
        let assign24490_e18900: f64 = (assign24490_e18898 * locals.var_ndepm2);
        (assign24490_e18900, ((((((((locals.var_afact_dn0 * locals.var_nin) - (locals.var_afact * locals.var_nin_dn0)) / (locals.var_nin * locals.var_nin)) * locals.var_nin) - (assign24490_e18896 * locals.var_nin_dn0)) / (locals.var_nin * locals.var_nin)) * locals.var_ndepm2) + (assign24490_e18898 * locals.var_ndepm2_dn0)), ((((((((locals.var_afact_dn2 * locals.var_nin) - (locals.var_afact * locals.var_nin_dn2)) / (locals.var_nin * locals.var_nin)) * locals.var_nin) - (assign24490_e18896 * locals.var_nin_dn2)) / (locals.var_nin * locals.var_nin)) * locals.var_ndepm2) + (assign24490_e18898 * locals.var_ndepm2_dn2)), ((((((((locals.var_afact_dn4 * locals.var_nin) - (locals.var_afact * locals.var_nin_dn4)) / (locals.var_nin * locals.var_nin)) * locals.var_nin) - (assign24490_e18896 * locals.var_nin_dn4)) / (locals.var_nin * locals.var_nin)) * locals.var_ndepm2) + (assign24490_e18898 * locals.var_ndepm2_dn4)), ((((((((locals.var_afact_dn5 * locals.var_nin) - (locals.var_afact * locals.var_nin_dn5)) / (locals.var_nin * locals.var_nin)) * locals.var_nin) - (assign24490_e18896 * locals.var_nin_dn5)) / (locals.var_nin * locals.var_nin)) * locals.var_ndepm2) + (assign24490_e18898 * locals.var_ndepm2_dn5)), ((((((((locals.var_afact_dn6 * locals.var_nin) - (locals.var_afact * locals.var_nin_dn6)) / (locals.var_nin * locals.var_nin)) * locals.var_nin) - (assign24490_e18896 * locals.var_nin_dn6)) / (locals.var_nin * locals.var_nin)) * locals.var_ndepm2) + (assign24490_e18898 * locals.var_ndepm2_dn6)), ((((((((locals.var_afact_dn7 * locals.var_nin) - (locals.var_afact * locals.var_nin_dn7)) / (locals.var_nin * locals.var_nin)) * locals.var_nin) - (assign24490_e18896 * locals.var_nin_dn7)) / (locals.var_nin * locals.var_nin)) * locals.var_ndepm2) + (assign24490_e18898 * locals.var_ndepm2_dn7)), ((((((((locals.var_afact_dn8 * locals.var_nin) - (locals.var_afact * locals.var_nin_dn8)) / (locals.var_nin * locals.var_nin)) * locals.var_nin) - (assign24490_e18896 * locals.var_nin_dn8)) / (locals.var_nin * locals.var_nin)) * locals.var_ndepm2) + (assign24490_e18898 * locals.var_ndepm2_dn8)), ((((((((locals.var_afact_dn9 * locals.var_nin) - (locals.var_afact * locals.var_nin_dn9)) / (locals.var_nin * locals.var_nin)) * locals.var_nin) - (assign24490_e18896 * locals.var_nin_dn9)) / (locals.var_nin * locals.var_nin)) * locals.var_ndepm2) + (assign24490_e18898 * locals.var_ndepm2_dn9)), ((((((((locals.var_afact_dn10 * locals.var_nin) - (locals.var_afact * locals.var_nin_dn10)) / (locals.var_nin * locals.var_nin)) * locals.var_nin) - (assign24490_e18896 * locals.var_nin_dn10)) / (locals.var_nin * locals.var_nin)) * locals.var_ndepm2) + (assign24490_e18898 * locals.var_ndepm2_dn10)), ((((((((locals.var_afact_dn13 * locals.var_nin) - (locals.var_afact * locals.var_nin_dn13)) / (locals.var_nin * locals.var_nin)) * locals.var_nin) - (assign24490_e18896 * locals.var_nin_dn13)) / (locals.var_nin * locals.var_nin)) * locals.var_ndepm2) + (assign24490_e18898 * locals.var_ndepm2_dn13)),)
    } else {
        (locals.var_afact2, locals.var_afact2_dn0, locals.var_afact2_dn2, locals.var_afact2_dn4, locals.var_afact2_dn5, locals.var_afact2_dn6, locals.var_afact2_dn7, locals.var_afact2_dn8, locals.var_afact2_dn9, locals.var_afact2_dn10, locals.var_afact2_dn13,)
    }
};
        locals.var_afact2 = assign24490_e18902;
        locals.var_afact2_dn0 = assign24490_e18902_d_n0;
        locals.var_afact2_dn2 = assign24490_e18902_d_n2;
        locals.var_afact2_dn4 = assign24490_e18902_d_n4;
        locals.var_afact2_dn5 = assign24490_e18902_d_n5;
        locals.var_afact2_dn6 = assign24490_e18902_d_n6;
        locals.var_afact2_dn7 = assign24490_e18902_d_n7;
        locals.var_afact2_dn8 = assign24490_e18902_d_n8;
        locals.var_afact2_dn9 = assign24490_e18902_d_n9;
        locals.var_afact2_dn10 = assign24490_e18902_d_n10;
        locals.var_afact2_dn13 = assign24490_e18902_d_n13;
        locals.var_afact2_rv = 0.0;

        let (assign24500_e18920, assign24500_e18920_d_n0, assign24500_e18920_d_n2, assign24500_e18920_d_n4, assign24500_e18920_d_n5, assign24500_e18920_d_n6, assign24500_e18920_d_n7, assign24500_e18920_d_n8, assign24500_e18920_d_n9, assign24500_e18920_d_n10, assign24500_e18920_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) {
        let assign24500_e18908: f64 = (locals.var_c_2esipq_ndepm * locals.var_ef_nsubc);
        let assign24500_e18911: f64 = (locals.var_ef_nsubc + locals.var_uc_ndepm);
        let assign24500_e18912: f64 = (assign24500_e18908 / assign24500_e18911);
        let assign24500_e18914: f64 = (-locals.var_vbscl__blk435);
        let assign24500_e18916: f64 = (assign24500_e18914 + locals.var_vbi_dep);
        let assign24500_e18917: f64 = (assign24500_e18912 * assign24500_e18916);
        let assign24500_e18918: f64 = (assign24500_e18917).sqrt();
        (assign24500_e18918, ((((((((locals.var_c_2esipq_ndepm_dn0 * locals.var_ef_nsubc) + (locals.var_c_2esipq_ndepm * locals.var_ef_nsubc_dn0)) * assign24500_e18911) - (assign24500_e18908 * (locals.var_ef_nsubc_dn0 + locals.var_uc_ndepm_dn0))) / (assign24500_e18911 * assign24500_e18911)) * assign24500_e18916) + (assign24500_e18912 * ((-locals.var_vbscl__blk435_dn0) + locals.var_vbi_dep_dn0))) / (2.0 * assign24500_e18918)), ((((((((locals.var_c_2esipq_ndepm_dn2 * locals.var_ef_nsubc) + (locals.var_c_2esipq_ndepm * locals.var_ef_nsubc_dn2)) * assign24500_e18911) - (assign24500_e18908 * (locals.var_ef_nsubc_dn2 + locals.var_uc_ndepm_dn2))) / (assign24500_e18911 * assign24500_e18911)) * assign24500_e18916) + (assign24500_e18912 * ((-locals.var_vbscl__blk435_dn2) + locals.var_vbi_dep_dn2))) / (2.0 * assign24500_e18918)), ((((((((locals.var_c_2esipq_ndepm_dn4 * locals.var_ef_nsubc) + (locals.var_c_2esipq_ndepm * locals.var_ef_nsubc_dn4)) * assign24500_e18911) - (assign24500_e18908 * (locals.var_ef_nsubc_dn4 + locals.var_uc_ndepm_dn4))) / (assign24500_e18911 * assign24500_e18911)) * assign24500_e18916) + (assign24500_e18912 * ((-locals.var_vbscl__blk435_dn4) + locals.var_vbi_dep_dn4))) / (2.0 * assign24500_e18918)), ((((((((locals.var_c_2esipq_ndepm_dn5 * locals.var_ef_nsubc) + (locals.var_c_2esipq_ndepm * locals.var_ef_nsubc_dn5)) * assign24500_e18911) - (assign24500_e18908 * (locals.var_ef_nsubc_dn5 + locals.var_uc_ndepm_dn5))) / (assign24500_e18911 * assign24500_e18911)) * assign24500_e18916) + (assign24500_e18912 * ((-locals.var_vbscl__blk435_dn5) + locals.var_vbi_dep_dn5))) / (2.0 * assign24500_e18918)), ((((((((locals.var_c_2esipq_ndepm_dn6 * locals.var_ef_nsubc) + (locals.var_c_2esipq_ndepm * locals.var_ef_nsubc_dn6)) * assign24500_e18911) - (assign24500_e18908 * (locals.var_ef_nsubc_dn6 + locals.var_uc_ndepm_dn6))) / (assign24500_e18911 * assign24500_e18911)) * assign24500_e18916) + (assign24500_e18912 * ((-locals.var_vbscl__blk435_dn6) + locals.var_vbi_dep_dn6))) / (2.0 * assign24500_e18918)), ((((((((locals.var_c_2esipq_ndepm_dn7 * locals.var_ef_nsubc) + (locals.var_c_2esipq_ndepm * locals.var_ef_nsubc_dn7)) * assign24500_e18911) - (assign24500_e18908 * (locals.var_ef_nsubc_dn7 + locals.var_uc_ndepm_dn7))) / (assign24500_e18911 * assign24500_e18911)) * assign24500_e18916) + (assign24500_e18912 * ((-locals.var_vbscl__blk435_dn7) + locals.var_vbi_dep_dn7))) / (2.0 * assign24500_e18918)), ((((((((locals.var_c_2esipq_ndepm_dn8 * locals.var_ef_nsubc) + (locals.var_c_2esipq_ndepm * locals.var_ef_nsubc_dn8)) * assign24500_e18911) - (assign24500_e18908 * (locals.var_ef_nsubc_dn8 + locals.var_uc_ndepm_dn8))) / (assign24500_e18911 * assign24500_e18911)) * assign24500_e18916) + (assign24500_e18912 * ((-locals.var_vbscl__blk435_dn8) + locals.var_vbi_dep_dn8))) / (2.0 * assign24500_e18918)), ((((((((locals.var_c_2esipq_ndepm_dn9 * locals.var_ef_nsubc) + (locals.var_c_2esipq_ndepm * locals.var_ef_nsubc_dn9)) * assign24500_e18911) - (assign24500_e18908 * (locals.var_ef_nsubc_dn9 + locals.var_uc_ndepm_dn9))) / (assign24500_e18911 * assign24500_e18911)) * assign24500_e18916) + (assign24500_e18912 * ((-locals.var_vbscl__blk435_dn9) + locals.var_vbi_dep_dn9))) / (2.0 * assign24500_e18918)), ((((((((locals.var_c_2esipq_ndepm_dn10 * locals.var_ef_nsubc) + (locals.var_c_2esipq_ndepm * locals.var_ef_nsubc_dn10)) * assign24500_e18911) - (assign24500_e18908 * (locals.var_ef_nsubc_dn10 + locals.var_uc_ndepm_dn10))) / (assign24500_e18911 * assign24500_e18911)) * assign24500_e18916) + (assign24500_e18912 * ((-locals.var_vbscl__blk435_dn10) + locals.var_vbi_dep_dn10))) / (2.0 * assign24500_e18918)), ((((((((locals.var_c_2esipq_ndepm_dn13 * locals.var_ef_nsubc) + (locals.var_c_2esipq_ndepm * locals.var_ef_nsubc_dn13)) * assign24500_e18911) - (assign24500_e18908 * (locals.var_ef_nsubc_dn13 + locals.var_uc_ndepm_dn13))) / (assign24500_e18911 * assign24500_e18911)) * assign24500_e18916) + (assign24500_e18912 * ((-locals.var_vbscl__blk435_dn13) + locals.var_vbi_dep_dn13))) / (2.0 * assign24500_e18918)),)
    } else {
        (locals.var_w_bsub0, locals.var_w_bsub0_dn0, locals.var_w_bsub0_dn2, locals.var_w_bsub0_dn4, locals.var_w_bsub0_dn5, locals.var_w_bsub0_dn6, locals.var_w_bsub0_dn7, locals.var_w_bsub0_dn8, locals.var_w_bsub0_dn9, locals.var_w_bsub0_dn10, locals.var_w_bsub0_dn13,)
    }
};
        locals.var_w_bsub0 = assign24500_e18920;
        locals.var_w_bsub0_dn0 = assign24500_e18920_d_n0;
        locals.var_w_bsub0_dn2 = assign24500_e18920_d_n2;
        locals.var_w_bsub0_dn4 = assign24500_e18920_d_n4;
        locals.var_w_bsub0_dn5 = assign24500_e18920_d_n5;
        locals.var_w_bsub0_dn6 = assign24500_e18920_d_n6;
        locals.var_w_bsub0_dn7 = assign24500_e18920_d_n7;
        locals.var_w_bsub0_dn8 = assign24500_e18920_d_n8;
        locals.var_w_bsub0_dn9 = assign24500_e18920_d_n9;
        locals.var_w_bsub0_dn10 = assign24500_e18920_d_n10;
        locals.var_w_bsub0_dn13 = assign24500_e18920_d_n13;
        locals.var_w_bsub0_rv = 0.0;

        let assign24510_e18923: f64 = if locals.var_w_bsub0 > locals.var_uc_depthn { 1.0 } else { 0.0 };
        locals.var_guard559 = assign24510_e18923;
        locals.var_guard559_rv = 0.0;

        let (assign24520_e18931, assign24520_e18931_d_n0, assign24520_e18931_d_n2, assign24520_e18931_d_n4, assign24520_e18931_d_n5, assign24520_e18931_d_n6, assign24520_e18931_d_n7, assign24520_e18931_d_n8, assign24520_e18931_d_n9, assign24520_e18931_d_n10, assign24520_e18931_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard559 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vgp0, locals.var_vgp0_dn0, locals.var_vgp0_dn2, locals.var_vgp0_dn4, locals.var_vgp0_dn5, locals.var_vgp0_dn6, locals.var_vgp0_dn7, locals.var_vgp0_dn8, locals.var_vgp0_dn9, locals.var_vgp0_dn10, locals.var_vgp0_dn13,)
    }
};
        locals.var_vgp0 = assign24520_e18931;
        locals.var_vgp0_dn0 = assign24520_e18931_d_n0;
        locals.var_vgp0_dn2 = assign24520_e18931_d_n2;
        locals.var_vgp0_dn4 = assign24520_e18931_d_n4;
        locals.var_vgp0_dn5 = assign24520_e18931_d_n5;
        locals.var_vgp0_dn6 = assign24520_e18931_d_n6;
        locals.var_vgp0_dn7 = assign24520_e18931_d_n7;
        locals.var_vgp0_dn8 = assign24520_e18931_d_n8;
        locals.var_vgp0_dn9 = assign24520_e18931_d_n9;
        locals.var_vgp0_dn10 = assign24520_e18931_d_n10;
        locals.var_vgp0_dn13 = assign24520_e18931_d_n13;
        locals.var_vgp0_rv = 0.0;

        let (assign24530_e18939, assign24530_e18939_d_n0, assign24530_e18939_d_n2, assign24530_e18939_d_n4, assign24530_e18939_d_n5, assign24530_e18939_d_n6, assign24530_e18939_d_n7, assign24530_e18939_d_n8, assign24530_e18939_d_n9, assign24530_e18939_d_n10, assign24530_e18939_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard559 != 0.0)) {
        (locals.var_uc_depthn, locals.var_uc_depthn_dn0, locals.var_uc_depthn_dn2, locals.var_uc_depthn_dn4, locals.var_uc_depthn_dn5, locals.var_uc_depthn_dn6, locals.var_uc_depthn_dn7, locals.var_uc_depthn_dn8, locals.var_uc_depthn_dn9, locals.var_uc_depthn_dn10, locals.var_uc_depthn_dn13,)
    } else {
        (locals.var_w_b0, locals.var_w_b0_dn0, locals.var_w_b0_dn2, locals.var_w_b0_dn4, locals.var_w_b0_dn5, locals.var_w_b0_dn6, locals.var_w_b0_dn7, locals.var_w_b0_dn8, locals.var_w_b0_dn9, locals.var_w_b0_dn10, locals.var_w_b0_dn13,)
    }
};
        locals.var_w_b0 = assign24530_e18939;
        locals.var_w_b0_dn0 = assign24530_e18939_d_n0;
        locals.var_w_b0_dn2 = assign24530_e18939_d_n2;
        locals.var_w_b0_dn4 = assign24530_e18939_d_n4;
        locals.var_w_b0_dn5 = assign24530_e18939_d_n5;
        locals.var_w_b0_dn6 = assign24530_e18939_d_n6;
        locals.var_w_b0_dn7 = assign24530_e18939_d_n7;
        locals.var_w_b0_dn8 = assign24530_e18939_d_n8;
        locals.var_w_b0_dn9 = assign24530_e18939_d_n9;
        locals.var_w_b0_dn10 = assign24530_e18939_d_n10;
        locals.var_w_b0_dn13 = assign24530_e18939_d_n13;
        locals.var_w_b0_rv = 0.0;

        let (assign24540_e18947, assign24540_e18947_d_n0, assign24540_e18947_d_n2, assign24540_e18947_d_n4, assign24540_e18947_d_n5, assign24540_e18947_d_n6, assign24540_e18947_d_n7, assign24540_e18947_d_n8, assign24540_e18947_d_n9, assign24540_e18947_d_n10, assign24540_e18947_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard559 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_phi_b0_dep, locals.var_phi_b0_dep_dn0, locals.var_phi_b0_dep_dn2, locals.var_phi_b0_dep_dn4, locals.var_phi_b0_dep_dn5, locals.var_phi_b0_dep_dn6, locals.var_phi_b0_dep_dn7, locals.var_phi_b0_dep_dn8, locals.var_phi_b0_dep_dn9, locals.var_phi_b0_dep_dn10, locals.var_phi_b0_dep_dn13,)
    }
};
        locals.var_phi_b0_dep = assign24540_e18947;
        locals.var_phi_b0_dep_dn0 = assign24540_e18947_d_n0;
        locals.var_phi_b0_dep_dn2 = assign24540_e18947_d_n2;
        locals.var_phi_b0_dep_dn4 = assign24540_e18947_d_n4;
        locals.var_phi_b0_dep_dn5 = assign24540_e18947_d_n5;
        locals.var_phi_b0_dep_dn6 = assign24540_e18947_d_n6;
        locals.var_phi_b0_dep_dn7 = assign24540_e18947_d_n7;
        locals.var_phi_b0_dep_dn8 = assign24540_e18947_d_n8;
        locals.var_phi_b0_dep_dn9 = assign24540_e18947_d_n9;
        locals.var_phi_b0_dep_dn10 = assign24540_e18947_d_n10;
        locals.var_phi_b0_dep_dn13 = assign24540_e18947_d_n13;
        locals.var_phi_b0_dep_rv = 0.0;

        let (assign24550_e18961, assign24550_e18961_d_n0, assign24550_e18961_d_n2, assign24550_e18961_d_n4, assign24550_e18961_d_n5, assign24550_e18961_d_n6, assign24550_e18961_d_n7, assign24550_e18961_d_n8, assign24550_e18961_d_n9, assign24550_e18961_d_n10, assign24550_e18961_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard559 != 0.0)) {
        let assign24550_e18956: f64 = (locals.var_c_2esipq_ndepm_inv * locals.var_w_b0);
        let assign24550_e18958: f64 = (assign24550_e18956 * locals.var_w_b0);
        let assign24550_e18959: f64 = (locals.var_phi_b0_dep - assign24550_e18958);
        (assign24550_e18959, (locals.var_phi_b0_dep_dn0 - ((((locals.var_c_2esipq_ndepm_inv_dn0 * locals.var_w_b0) + (locals.var_c_2esipq_ndepm_inv * locals.var_w_b0_dn0)) * locals.var_w_b0) + (assign24550_e18956 * locals.var_w_b0_dn0))), (locals.var_phi_b0_dep_dn2 - ((((locals.var_c_2esipq_ndepm_inv_dn2 * locals.var_w_b0) + (locals.var_c_2esipq_ndepm_inv * locals.var_w_b0_dn2)) * locals.var_w_b0) + (assign24550_e18956 * locals.var_w_b0_dn2))), (locals.var_phi_b0_dep_dn4 - ((((locals.var_c_2esipq_ndepm_inv_dn4 * locals.var_w_b0) + (locals.var_c_2esipq_ndepm_inv * locals.var_w_b0_dn4)) * locals.var_w_b0) + (assign24550_e18956 * locals.var_w_b0_dn4))), (locals.var_phi_b0_dep_dn5 - ((((locals.var_c_2esipq_ndepm_inv_dn5 * locals.var_w_b0) + (locals.var_c_2esipq_ndepm_inv * locals.var_w_b0_dn5)) * locals.var_w_b0) + (assign24550_e18956 * locals.var_w_b0_dn5))), (locals.var_phi_b0_dep_dn6 - ((((locals.var_c_2esipq_ndepm_inv_dn6 * locals.var_w_b0) + (locals.var_c_2esipq_ndepm_inv * locals.var_w_b0_dn6)) * locals.var_w_b0) + (assign24550_e18956 * locals.var_w_b0_dn6))), (locals.var_phi_b0_dep_dn7 - ((((locals.var_c_2esipq_ndepm_inv_dn7 * locals.var_w_b0) + (locals.var_c_2esipq_ndepm_inv * locals.var_w_b0_dn7)) * locals.var_w_b0) + (assign24550_e18956 * locals.var_w_b0_dn7))), (locals.var_phi_b0_dep_dn8 - ((((locals.var_c_2esipq_ndepm_inv_dn8 * locals.var_w_b0) + (locals.var_c_2esipq_ndepm_inv * locals.var_w_b0_dn8)) * locals.var_w_b0) + (assign24550_e18956 * locals.var_w_b0_dn8))), (locals.var_phi_b0_dep_dn9 - ((((locals.var_c_2esipq_ndepm_inv_dn9 * locals.var_w_b0) + (locals.var_c_2esipq_ndepm_inv * locals.var_w_b0_dn9)) * locals.var_w_b0) + (assign24550_e18956 * locals.var_w_b0_dn9))), (locals.var_phi_b0_dep_dn10 - ((((locals.var_c_2esipq_ndepm_inv_dn10 * locals.var_w_b0) + (locals.var_c_2esipq_ndepm_inv * locals.var_w_b0_dn10)) * locals.var_w_b0) + (assign24550_e18956 * locals.var_w_b0_dn10))), (locals.var_phi_b0_dep_dn13 - ((((locals.var_c_2esipq_ndepm_inv_dn13 * locals.var_w_b0) + (locals.var_c_2esipq_ndepm_inv * locals.var_w_b0_dn13)) * locals.var_w_b0) + (assign24550_e18956 * locals.var_w_b0_dn13))),)
    } else {
        (locals.var_phi_j0_dep, locals.var_phi_j0_dep_dn0, locals.var_phi_j0_dep_dn2, locals.var_phi_j0_dep_dn4, locals.var_phi_j0_dep_dn5, locals.var_phi_j0_dep_dn6, locals.var_phi_j0_dep_dn7, locals.var_phi_j0_dep_dn8, locals.var_phi_j0_dep_dn9, locals.var_phi_j0_dep_dn10, locals.var_phi_j0_dep_dn13,)
    }
};
        locals.var_phi_j0_dep = assign24550_e18961;
        locals.var_phi_j0_dep_dn0 = assign24550_e18961_d_n0;
        locals.var_phi_j0_dep_dn2 = assign24550_e18961_d_n2;
        locals.var_phi_j0_dep_dn4 = assign24550_e18961_d_n4;
        locals.var_phi_j0_dep_dn5 = assign24550_e18961_d_n5;
        locals.var_phi_j0_dep_dn6 = assign24550_e18961_d_n6;
        locals.var_phi_j0_dep_dn7 = assign24550_e18961_d_n7;
        locals.var_phi_j0_dep_dn8 = assign24550_e18961_d_n8;
        locals.var_phi_j0_dep_dn9 = assign24550_e18961_d_n9;
        locals.var_phi_j0_dep_dn10 = assign24550_e18961_d_n10;
        locals.var_phi_j0_dep_dn13 = assign24550_e18961_d_n13;
        locals.var_phi_j0_dep_rv = 0.0;

        let (assign24560_e18969, assign24560_e18969_d_n0, assign24560_e18969_d_n2, assign24560_e18969_d_n4, assign24560_e18969_d_n5, assign24560_e18969_d_n6, assign24560_e18969_d_n7, assign24560_e18969_d_n8, assign24560_e18969_d_n9, assign24560_e18969_d_n10, assign24560_e18969_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard559 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vds_maxb0, locals.var_vds_maxb0_dn0, locals.var_vds_maxb0_dn2, locals.var_vds_maxb0_dn4, locals.var_vds_maxb0_dn5, locals.var_vds_maxb0_dn6, locals.var_vds_maxb0_dn7, locals.var_vds_maxb0_dn8, locals.var_vds_maxb0_dn9, locals.var_vds_maxb0_dn10, locals.var_vds_maxb0_dn13,)
    }
};
        locals.var_vds_maxb0 = assign24560_e18969;
        locals.var_vds_maxb0_dn0 = assign24560_e18969_d_n0;
        locals.var_vds_maxb0_dn2 = assign24560_e18969_d_n2;
        locals.var_vds_maxb0_dn4 = assign24560_e18969_d_n4;
        locals.var_vds_maxb0_dn5 = assign24560_e18969_d_n5;
        locals.var_vds_maxb0_dn6 = assign24560_e18969_d_n6;
        locals.var_vds_maxb0_dn7 = assign24560_e18969_d_n7;
        locals.var_vds_maxb0_dn8 = assign24560_e18969_d_n8;
        locals.var_vds_maxb0_dn9 = assign24560_e18969_d_n9;
        locals.var_vds_maxb0_dn10 = assign24560_e18969_d_n10;
        locals.var_vds_maxb0_dn13 = assign24560_e18969_d_n13;
        locals.var_vds_maxb0_rv = 0.0;

        let (assign24570_e18977, assign24570_e18977_d_n0, assign24570_e18977_d_n2, assign24570_e18977_d_n4, assign24570_e18977_d_n5, assign24570_e18977_d_n6, assign24570_e18977_d_n7, assign24570_e18977_d_n8, assign24570_e18977_d_n9, assign24570_e18977_d_n10, assign24570_e18977_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard559 != 0.0)) {
        (locals.var_vgp0, locals.var_vgp0_dn0, locals.var_vgp0_dn2, locals.var_vgp0_dn4, locals.var_vgp0_dn5, locals.var_vgp0_dn6, locals.var_vgp0_dn7, locals.var_vgp0_dn8, locals.var_vgp0_dn9, locals.var_vgp0_dn10, locals.var_vgp0_dn13,)
    } else {
        (locals.var_vgp0old, locals.var_vgp0old_dn0, locals.var_vgp0old_dn2, locals.var_vgp0old_dn4, locals.var_vgp0old_dn5, locals.var_vgp0old_dn6, locals.var_vgp0old_dn7, locals.var_vgp0old_dn8, locals.var_vgp0old_dn9, locals.var_vgp0old_dn10, locals.var_vgp0old_dn13,)
    }
};
        locals.var_vgp0old = assign24570_e18977;
        locals.var_vgp0old_dn0 = assign24570_e18977_d_n0;
        locals.var_vgp0old_dn2 = assign24570_e18977_d_n2;
        locals.var_vgp0old_dn4 = assign24570_e18977_d_n4;
        locals.var_vgp0old_dn5 = assign24570_e18977_d_n5;
        locals.var_vgp0old_dn6 = assign24570_e18977_d_n6;
        locals.var_vgp0old_dn7 = assign24570_e18977_d_n7;
        locals.var_vgp0old_dn8 = assign24570_e18977_d_n8;
        locals.var_vgp0old_dn9 = assign24570_e18977_d_n9;
        locals.var_vgp0old_dn10 = assign24570_e18977_d_n10;
        locals.var_vgp0old_dn13 = assign24570_e18977_d_n13;
        locals.var_vgp0old_rv = 0.0;

        let (assign24580_e18985, assign24580_e18985_d_n0, assign24580_e18985_d_n2, assign24580_e18985_d_n4, assign24580_e18985_d_n5, assign24580_e18985_d_n6, assign24580_e18985_d_n7, assign24580_e18985_d_n8, assign24580_e18985_d_n9, assign24580_e18985_d_n10, assign24580_e18985_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard559 != 0.0)) {
        (locals.var_phi_j0_dep, locals.var_phi_j0_dep_dn0, locals.var_phi_j0_dep_dn2, locals.var_phi_j0_dep_dn4, locals.var_phi_j0_dep_dn5, locals.var_phi_j0_dep_dn6, locals.var_phi_j0_dep_dn7, locals.var_phi_j0_dep_dn8, locals.var_phi_j0_dep_dn9, locals.var_phi_j0_dep_dn10, locals.var_phi_j0_dep_dn13,)
    } else {
        (locals.var_phi_j0_dep_old, locals.var_phi_j0_dep_old_dn0, locals.var_phi_j0_dep_old_dn2, locals.var_phi_j0_dep_old_dn4, locals.var_phi_j0_dep_old_dn5, locals.var_phi_j0_dep_old_dn6, locals.var_phi_j0_dep_old_dn7, locals.var_phi_j0_dep_old_dn8, locals.var_phi_j0_dep_old_dn9, locals.var_phi_j0_dep_old_dn10, locals.var_phi_j0_dep_old_dn13,)
    }
};
        locals.var_phi_j0_dep_old = assign24580_e18985;
        locals.var_phi_j0_dep_old_dn0 = assign24580_e18985_d_n0;
        locals.var_phi_j0_dep_old_dn2 = assign24580_e18985_d_n2;
        locals.var_phi_j0_dep_old_dn4 = assign24580_e18985_d_n4;
        locals.var_phi_j0_dep_old_dn5 = assign24580_e18985_d_n5;
        locals.var_phi_j0_dep_old_dn6 = assign24580_e18985_d_n6;
        locals.var_phi_j0_dep_old_dn7 = assign24580_e18985_d_n7;
        locals.var_phi_j0_dep_old_dn8 = assign24580_e18985_d_n8;
        locals.var_phi_j0_dep_old_dn9 = assign24580_e18985_d_n9;
        locals.var_phi_j0_dep_old_dn10 = assign24580_e18985_d_n10;
        locals.var_phi_j0_dep_old_dn13 = assign24580_e18985_d_n13;
        locals.var_phi_j0_dep_old_rv = 0.0;

        let (assign24590_e18993,) = {
    if (((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard559 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_lp_s0,)
    }
};
        locals.var_lp_s0 = assign24590_e18993;
        locals.var_lp_s0_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_66(
        locals: &mut StampLocals,
    ) {
        let mut assign24600_loop_guard: usize = 0;
        while {
            let assign24600_cond_e19002: f64 = (150.0 + 1.0);
            let assign24600_cond_e19004: f64 = if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard559 != 0.0)) && (locals.var_lp_s0 <= assign24600_cond_e19002)) { 1.0 } else { 0.0 };
            assign24600_cond_e19004 != 0.0
        } {
            assign24600_loop_guard += 1;
            assert!(assign24600_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign24600_body0_e19017, assign24600_body0_e19017_d_n0, assign24600_body0_e19017_d_n2, assign24600_body0_e19017_d_n4, assign24600_body0_e19017_d_n5, assign24600_body0_e19017_d_n6, assign24600_body0_e19017_d_n7, assign24600_body0_e19017_d_n8, assign24600_body0_e19017_d_n9, assign24600_body0_e19017_d_n10, assign24600_body0_e19017_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard559 != 0.0)) {
        let assign24600_body0_e19013: f64 = (locals.var_phi_b0_dep - locals.var_phi_j0_dep);
        let assign24600_body0_e19014: f64 = (locals.var_c_2esipq_ndepm * assign24600_body0_e19013);
        let assign24600_body0_e19015: f64 = (assign24600_body0_e19014).sqrt();
        (assign24600_body0_e19015, (((locals.var_c_2esipq_ndepm_dn0 * assign24600_body0_e19013) + (locals.var_c_2esipq_ndepm * (locals.var_phi_b0_dep_dn0 - locals.var_phi_j0_dep_dn0))) / (2.0 * assign24600_body0_e19015)), (((locals.var_c_2esipq_ndepm_dn2 * assign24600_body0_e19013) + (locals.var_c_2esipq_ndepm * (locals.var_phi_b0_dep_dn2 - locals.var_phi_j0_dep_dn2))) / (2.0 * assign24600_body0_e19015)), (((locals.var_c_2esipq_ndepm_dn4 * assign24600_body0_e19013) + (locals.var_c_2esipq_ndepm * (locals.var_phi_b0_dep_dn4 - locals.var_phi_j0_dep_dn4))) / (2.0 * assign24600_body0_e19015)), (((locals.var_c_2esipq_ndepm_dn5 * assign24600_body0_e19013) + (locals.var_c_2esipq_ndepm * (locals.var_phi_b0_dep_dn5 - locals.var_phi_j0_dep_dn5))) / (2.0 * assign24600_body0_e19015)), (((locals.var_c_2esipq_ndepm_dn6 * assign24600_body0_e19013) + (locals.var_c_2esipq_ndepm * (locals.var_phi_b0_dep_dn6 - locals.var_phi_j0_dep_dn6))) / (2.0 * assign24600_body0_e19015)), (((locals.var_c_2esipq_ndepm_dn7 * assign24600_body0_e19013) + (locals.var_c_2esipq_ndepm * (locals.var_phi_b0_dep_dn7 - locals.var_phi_j0_dep_dn7))) / (2.0 * assign24600_body0_e19015)), (((locals.var_c_2esipq_ndepm_dn8 * assign24600_body0_e19013) + (locals.var_c_2esipq_ndepm * (locals.var_phi_b0_dep_dn8 - locals.var_phi_j0_dep_dn8))) / (2.0 * assign24600_body0_e19015)), (((locals.var_c_2esipq_ndepm_dn9 * assign24600_body0_e19013) + (locals.var_c_2esipq_ndepm * (locals.var_phi_b0_dep_dn9 - locals.var_phi_j0_dep_dn9))) / (2.0 * assign24600_body0_e19015)), (((locals.var_c_2esipq_ndepm_dn10 * assign24600_body0_e19013) + (locals.var_c_2esipq_ndepm * (locals.var_phi_b0_dep_dn10 - locals.var_phi_j0_dep_dn10))) / (2.0 * assign24600_body0_e19015)), (((locals.var_c_2esipq_ndepm_dn13 * assign24600_body0_e19013) + (locals.var_c_2esipq_ndepm * (locals.var_phi_b0_dep_dn13 - locals.var_phi_j0_dep_dn13))) / (2.0 * assign24600_body0_e19015)),)
    } else {
        (locals.var_w_b0, locals.var_w_b0_dn0, locals.var_w_b0_dn2, locals.var_w_b0_dn4, locals.var_w_b0_dn5, locals.var_w_b0_dn6, locals.var_w_b0_dn7, locals.var_w_b0_dn8, locals.var_w_b0_dn9, locals.var_w_b0_dn10, locals.var_w_b0_dn13,)
    }
};
            locals.var_w_b0 = assign24600_body0_e19017;
            locals.var_w_b0_dn0 = assign24600_body0_e19017_d_n0;
            locals.var_w_b0_dn2 = assign24600_body0_e19017_d_n2;
            locals.var_w_b0_dn4 = assign24600_body0_e19017_d_n4;
            locals.var_w_b0_dn5 = assign24600_body0_e19017_d_n5;
            locals.var_w_b0_dn6 = assign24600_body0_e19017_d_n6;
            locals.var_w_b0_dn7 = assign24600_body0_e19017_d_n7;
            locals.var_w_b0_dn8 = assign24600_body0_e19017_d_n8;
            locals.var_w_b0_dn9 = assign24600_body0_e19017_d_n9;
            locals.var_w_b0_dn10 = assign24600_body0_e19017_d_n10;
            locals.var_w_b0_dn13 = assign24600_body0_e19017_d_n13;
            locals.var_w_b0_rv = 0.0;
            let assign24600_body1_e19021: f64 = (locals.var_uc_depthn - 1e-8);
            let assign24600_body1_e19026: f64 = if ((locals.var_w_b0 > assign24600_body1_e19021) && (1e-8 >= 0.0)) { 1.0 } else { 0.0 };
            locals.var_guard560 = assign24600_body1_e19026;
            locals.var_guard560_rv = 0.0;
            let (assign24600_body2_e19040, assign24600_body2_e19040_d_n0, assign24600_body2_e19040_d_n2, assign24600_body2_e19040_d_n4, assign24600_body2_e19040_d_n5, assign24600_body2_e19040_d_n6, assign24600_body2_e19040_d_n7, assign24600_body2_e19040_d_n8, assign24600_body2_e19040_d_n9, assign24600_body2_e19040_d_n10, assign24600_body2_e19040_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard559 != 0.0)) && (locals.var_guard560 != 0.0)) {
        let assign24600_body2_e19036: f64 = (locals.var_w_b0 - locals.var_uc_depthn);
        let assign24600_body2_e19038: f64 = (assign24600_body2_e19036 + 1e-8);
        (assign24600_body2_e19038, (locals.var_w_b0_dn0 - locals.var_uc_depthn_dn0), (locals.var_w_b0_dn2 - locals.var_uc_depthn_dn2), (locals.var_w_b0_dn4 - locals.var_uc_depthn_dn4), (locals.var_w_b0_dn5 - locals.var_uc_depthn_dn5), (locals.var_w_b0_dn6 - locals.var_uc_depthn_dn6), (locals.var_w_b0_dn7 - locals.var_uc_depthn_dn7), (locals.var_w_b0_dn8 - locals.var_uc_depthn_dn8), (locals.var_w_b0_dn9 - locals.var_uc_depthn_dn9), (locals.var_w_b0_dn10 - locals.var_uc_depthn_dn10), (locals.var_w_b0_dn13 - locals.var_uc_depthn_dn13),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13,)
    }
};
            locals.var_tmf1 = assign24600_body2_e19040;
            locals.var_tmf1_dn0 = assign24600_body2_e19040_d_n0;
            locals.var_tmf1_dn2 = assign24600_body2_e19040_d_n2;
            locals.var_tmf1_dn4 = assign24600_body2_e19040_d_n4;
            locals.var_tmf1_dn5 = assign24600_body2_e19040_d_n5;
            locals.var_tmf1_dn6 = assign24600_body2_e19040_d_n6;
            locals.var_tmf1_dn7 = assign24600_body2_e19040_d_n7;
            locals.var_tmf1_dn8 = assign24600_body2_e19040_d_n8;
            locals.var_tmf1_dn9 = assign24600_body2_e19040_d_n9;
            locals.var_tmf1_dn10 = assign24600_body2_e19040_d_n10;
            locals.var_tmf1_dn13 = assign24600_body2_e19040_d_n13;
            locals.var_tmf1_rv = 0.0;
            let (assign24600_body3_e19052, assign24600_body3_e19052_d_n0, assign24600_body3_e19052_d_n2, assign24600_body3_e19052_d_n4, assign24600_body3_e19052_d_n5, assign24600_body3_e19052_d_n6, assign24600_body3_e19052_d_n7, assign24600_body3_e19052_d_n8, assign24600_body3_e19052_d_n9, assign24600_body3_e19052_d_n10, assign24600_body3_e19052_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard559 != 0.0)) && (locals.var_guard560 != 0.0)) {
        let assign24600_body3_e19050: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign24600_body3_e19050, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)), ((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)), ((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn13 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn13)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn4, locals.var_x2_dn5, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn8, locals.var_x2_dn9, locals.var_x2_dn10, locals.var_x2_dn13,)
    }
};
            locals.var_x2 = assign24600_body3_e19052;
            locals.var_x2_dn0 = assign24600_body3_e19052_d_n0;
            locals.var_x2_dn2 = assign24600_body3_e19052_d_n2;
            locals.var_x2_dn4 = assign24600_body3_e19052_d_n4;
            locals.var_x2_dn5 = assign24600_body3_e19052_d_n5;
            locals.var_x2_dn6 = assign24600_body3_e19052_d_n6;
            locals.var_x2_dn7 = assign24600_body3_e19052_d_n7;
            locals.var_x2_dn8 = assign24600_body3_e19052_d_n8;
            locals.var_x2_dn9 = assign24600_body3_e19052_d_n9;
            locals.var_x2_dn10 = assign24600_body3_e19052_d_n10;
            locals.var_x2_dn13 = assign24600_body3_e19052_d_n13;
            locals.var_x2_rv = 0.0;
            let (assign24600_body4_e19064, assign24600_body4_e19064_d_n0, assign24600_body4_e19064_d_n2, assign24600_body4_e19064_d_n4, assign24600_body4_e19064_d_n5, assign24600_body4_e19064_d_n6, assign24600_body4_e19064_d_n7, assign24600_body4_e19064_d_n8, assign24600_body4_e19064_d_n9, assign24600_body4_e19064_d_n10, assign24600_body4_e19064_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard559 != 0.0)) && (locals.var_guard560 != 0.0)) {
        let assign24600_body4_e19062: f64 = (1e-8 * 1e-8);
        (assign24600_body4_e19062, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn4, locals.var_xmax2_dn5, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn8, locals.var_xmax2_dn9, locals.var_xmax2_dn10, locals.var_xmax2_dn13,)
    }
};
            locals.var_xmax2 = assign24600_body4_e19064;
            locals.var_xmax2_dn0 = assign24600_body4_e19064_d_n0;
            locals.var_xmax2_dn2 = assign24600_body4_e19064_d_n2;
            locals.var_xmax2_dn4 = assign24600_body4_e19064_d_n4;
            locals.var_xmax2_dn5 = assign24600_body4_e19064_d_n5;
            locals.var_xmax2_dn6 = assign24600_body4_e19064_d_n6;
            locals.var_xmax2_dn7 = assign24600_body4_e19064_d_n7;
            locals.var_xmax2_dn8 = assign24600_body4_e19064_d_n8;
            locals.var_xmax2_dn9 = assign24600_body4_e19064_d_n9;
            locals.var_xmax2_dn10 = assign24600_body4_e19064_d_n10;
            locals.var_xmax2_dn13 = assign24600_body4_e19064_d_n13;
            locals.var_xmax2_rv = 0.0;
            let (assign24600_body5_e19074, assign24600_body5_e19074_d_n0, assign24600_body5_e19074_d_n2, assign24600_body5_e19074_d_n4, assign24600_body5_e19074_d_n5, assign24600_body5_e19074_d_n6, assign24600_body5_e19074_d_n7, assign24600_body5_e19074_d_n8, assign24600_body5_e19074_d_n9, assign24600_body5_e19074_d_n10, assign24600_body5_e19074_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard559 != 0.0)) && (locals.var_guard560 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
            locals.var_xp = assign24600_body5_e19074;
            locals.var_xp_dn0 = assign24600_body5_e19074_d_n0;
            locals.var_xp_dn2 = assign24600_body5_e19074_d_n2;
            locals.var_xp_dn4 = assign24600_body5_e19074_d_n4;
            locals.var_xp_dn5 = assign24600_body5_e19074_d_n5;
            locals.var_xp_dn6 = assign24600_body5_e19074_d_n6;
            locals.var_xp_dn7 = assign24600_body5_e19074_d_n7;
            locals.var_xp_dn8 = assign24600_body5_e19074_d_n8;
            locals.var_xp_dn9 = assign24600_body5_e19074_d_n9;
            locals.var_xp_dn10 = assign24600_body5_e19074_d_n10;
            locals.var_xp_dn13 = assign24600_body5_e19074_d_n13;
            locals.var_xp_rv = 0.0;
            let (assign24600_body6_e19084, assign24600_body6_e19084_d_n0, assign24600_body6_e19084_d_n2, assign24600_body6_e19084_d_n4, assign24600_body6_e19084_d_n5, assign24600_body6_e19084_d_n6, assign24600_body6_e19084_d_n7, assign24600_body6_e19084_d_n8, assign24600_body6_e19084_d_n9, assign24600_body6_e19084_d_n10, assign24600_body6_e19084_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard559 != 0.0)) && (locals.var_guard560 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
            locals.var_xmp = assign24600_body6_e19084;
            locals.var_xmp_dn0 = assign24600_body6_e19084_d_n0;
            locals.var_xmp_dn2 = assign24600_body6_e19084_d_n2;
            locals.var_xmp_dn4 = assign24600_body6_e19084_d_n4;
            locals.var_xmp_dn5 = assign24600_body6_e19084_d_n5;
            locals.var_xmp_dn6 = assign24600_body6_e19084_d_n6;
            locals.var_xmp_dn7 = assign24600_body6_e19084_d_n7;
            locals.var_xmp_dn8 = assign24600_body6_e19084_d_n8;
            locals.var_xmp_dn9 = assign24600_body6_e19084_d_n9;
            locals.var_xmp_dn10 = assign24600_body6_e19084_d_n10;
            locals.var_xmp_dn13 = assign24600_body6_e19084_d_n13;
            locals.var_xmp_rv = 0.0;
            let (assign24600_body7_e19094,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard559 != 0.0)) && (locals.var_guard560 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign24600_body7_e19094;
            locals.var_m0_rv = 0.0;
            let (assign24600_body8_e19104,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard559 != 0.0)) && (locals.var_guard560 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
            locals.var_mm = assign24600_body8_e19104;
            locals.var_mm_rv = 0.0;
            let (assign24600_body9_e19114, assign24600_body9_e19114_d_n0, assign24600_body9_e19114_d_n2, assign24600_body9_e19114_d_n4, assign24600_body9_e19114_d_n5, assign24600_body9_e19114_d_n6, assign24600_body9_e19114_d_n7, assign24600_body9_e19114_d_n8, assign24600_body9_e19114_d_n9, assign24600_body9_e19114_d_n10, assign24600_body9_e19114_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard559 != 0.0)) && (locals.var_guard560 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    }
};
            locals.var_arg = assign24600_body9_e19114;
            locals.var_arg_dn0 = assign24600_body9_e19114_d_n0;
            locals.var_arg_dn2 = assign24600_body9_e19114_d_n2;
            locals.var_arg_dn4 = assign24600_body9_e19114_d_n4;
            locals.var_arg_dn5 = assign24600_body9_e19114_d_n5;
            locals.var_arg_dn6 = assign24600_body9_e19114_d_n6;
            locals.var_arg_dn7 = assign24600_body9_e19114_d_n7;
            locals.var_arg_dn8 = assign24600_body9_e19114_d_n8;
            locals.var_arg_dn9 = assign24600_body9_e19114_d_n9;
            locals.var_arg_dn10 = assign24600_body9_e19114_d_n10;
            locals.var_arg_dn13 = assign24600_body9_e19114_d_n13;
            locals.var_arg_rv = 0.0;
            let (assign24600_body10_e19124, assign24600_body10_e19124_d_n0, assign24600_body10_e19124_d_n2, assign24600_body10_e19124_d_n4, assign24600_body10_e19124_d_n5, assign24600_body10_e19124_d_n6, assign24600_body10_e19124_d_n7, assign24600_body10_e19124_d_n8, assign24600_body10_e19124_d_n9, assign24600_body10_e19124_d_n10, assign24600_body10_e19124_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard559 != 0.0)) && (locals.var_guard560 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
            locals.var_dnm = assign24600_body10_e19124;
            locals.var_dnm_dn0 = assign24600_body10_e19124_d_n0;
            locals.var_dnm_dn2 = assign24600_body10_e19124_d_n2;
            locals.var_dnm_dn4 = assign24600_body10_e19124_d_n4;
            locals.var_dnm_dn5 = assign24600_body10_e19124_d_n5;
            locals.var_dnm_dn6 = assign24600_body10_e19124_d_n6;
            locals.var_dnm_dn7 = assign24600_body10_e19124_d_n7;
            locals.var_dnm_dn8 = assign24600_body10_e19124_d_n8;
            locals.var_dnm_dn9 = assign24600_body10_e19124_d_n9;
            locals.var_dnm_dn10 = assign24600_body10_e19124_d_n10;
            locals.var_dnm_dn13 = assign24600_body10_e19124_d_n13;
            locals.var_dnm_rv = 0.0;
            let (assign24600_body11_e19136, assign24600_body11_e19136_d_n0, assign24600_body11_e19136_d_n2, assign24600_body11_e19136_d_n4, assign24600_body11_e19136_d_n5, assign24600_body11_e19136_d_n6, assign24600_body11_e19136_d_n7, assign24600_body11_e19136_d_n8, assign24600_body11_e19136_d_n9, assign24600_body11_e19136_d_n10, assign24600_body11_e19136_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard559 != 0.0)) && (locals.var_guard560 != 0.0)) {
        let assign24600_body11_e19134: f64 = (locals.var_xp * locals.var_x2);
        (assign24600_body11_e19134, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn13 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn13)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
            locals.var_xp = assign24600_body11_e19136;
            locals.var_xp_dn0 = assign24600_body11_e19136_d_n0;
            locals.var_xp_dn2 = assign24600_body11_e19136_d_n2;
            locals.var_xp_dn4 = assign24600_body11_e19136_d_n4;
            locals.var_xp_dn5 = assign24600_body11_e19136_d_n5;
            locals.var_xp_dn6 = assign24600_body11_e19136_d_n6;
            locals.var_xp_dn7 = assign24600_body11_e19136_d_n7;
            locals.var_xp_dn8 = assign24600_body11_e19136_d_n8;
            locals.var_xp_dn9 = assign24600_body11_e19136_d_n9;
            locals.var_xp_dn10 = assign24600_body11_e19136_d_n10;
            locals.var_xp_dn13 = assign24600_body11_e19136_d_n13;
            locals.var_xp_rv = 0.0;
            let (assign24600_body12_e19148, assign24600_body12_e19148_d_n0, assign24600_body12_e19148_d_n2, assign24600_body12_e19148_d_n4, assign24600_body12_e19148_d_n5, assign24600_body12_e19148_d_n6, assign24600_body12_e19148_d_n7, assign24600_body12_e19148_d_n8, assign24600_body12_e19148_d_n9, assign24600_body12_e19148_d_n10, assign24600_body12_e19148_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard559 != 0.0)) && (locals.var_guard560 != 0.0)) {
        let assign24600_body12_e19146: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign24600_body12_e19146, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn13 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn13)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
            locals.var_xmp = assign24600_body12_e19148;
            locals.var_xmp_dn0 = assign24600_body12_e19148_d_n0;
            locals.var_xmp_dn2 = assign24600_body12_e19148_d_n2;
            locals.var_xmp_dn4 = assign24600_body12_e19148_d_n4;
            locals.var_xmp_dn5 = assign24600_body12_e19148_d_n5;
            locals.var_xmp_dn6 = assign24600_body12_e19148_d_n6;
            locals.var_xmp_dn7 = assign24600_body12_e19148_d_n7;
            locals.var_xmp_dn8 = assign24600_body12_e19148_d_n8;
            locals.var_xmp_dn9 = assign24600_body12_e19148_d_n9;
            locals.var_xmp_dn10 = assign24600_body12_e19148_d_n10;
            locals.var_xmp_dn13 = assign24600_body12_e19148_d_n13;
            locals.var_xmp_rv = 0.0;
            let (assign24600_body13_e19160, assign24600_body13_e19160_d_n0, assign24600_body13_e19160_d_n2, assign24600_body13_e19160_d_n4, assign24600_body13_e19160_d_n5, assign24600_body13_e19160_d_n6, assign24600_body13_e19160_d_n7, assign24600_body13_e19160_d_n8, assign24600_body13_e19160_d_n9, assign24600_body13_e19160_d_n10, assign24600_body13_e19160_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard559 != 0.0)) && (locals.var_guard560 != 0.0)) {
        let assign24600_body13_e19158: f64 = (locals.var_xp * locals.var_x2);
        (assign24600_body13_e19158, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn13 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn13)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
            locals.var_xp = assign24600_body13_e19160;
            locals.var_xp_dn0 = assign24600_body13_e19160_d_n0;
            locals.var_xp_dn2 = assign24600_body13_e19160_d_n2;
            locals.var_xp_dn4 = assign24600_body13_e19160_d_n4;
            locals.var_xp_dn5 = assign24600_body13_e19160_d_n5;
            locals.var_xp_dn6 = assign24600_body13_e19160_d_n6;
            locals.var_xp_dn7 = assign24600_body13_e19160_d_n7;
            locals.var_xp_dn8 = assign24600_body13_e19160_d_n8;
            locals.var_xp_dn9 = assign24600_body13_e19160_d_n9;
            locals.var_xp_dn10 = assign24600_body13_e19160_d_n10;
            locals.var_xp_dn13 = assign24600_body13_e19160_d_n13;
            locals.var_xp_rv = 0.0;
            let (assign24600_body14_e19172, assign24600_body14_e19172_d_n0, assign24600_body14_e19172_d_n2, assign24600_body14_e19172_d_n4, assign24600_body14_e19172_d_n5, assign24600_body14_e19172_d_n6, assign24600_body14_e19172_d_n7, assign24600_body14_e19172_d_n8, assign24600_body14_e19172_d_n9, assign24600_body14_e19172_d_n10, assign24600_body14_e19172_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard559 != 0.0)) && (locals.var_guard560 != 0.0)) {
        let assign24600_body14_e19170: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign24600_body14_e19170, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn13 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn13)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
            locals.var_xmp = assign24600_body14_e19172;
            locals.var_xmp_dn0 = assign24600_body14_e19172_d_n0;
            locals.var_xmp_dn2 = assign24600_body14_e19172_d_n2;
            locals.var_xmp_dn4 = assign24600_body14_e19172_d_n4;
            locals.var_xmp_dn5 = assign24600_body14_e19172_d_n5;
            locals.var_xmp_dn6 = assign24600_body14_e19172_d_n6;
            locals.var_xmp_dn7 = assign24600_body14_e19172_d_n7;
            locals.var_xmp_dn8 = assign24600_body14_e19172_d_n8;
            locals.var_xmp_dn9 = assign24600_body14_e19172_d_n9;
            locals.var_xmp_dn10 = assign24600_body14_e19172_d_n10;
            locals.var_xmp_dn13 = assign24600_body14_e19172_d_n13;
            locals.var_xmp_rv = 0.0;
            let (assign24600_body15_e19184, assign24600_body15_e19184_d_n0, assign24600_body15_e19184_d_n2, assign24600_body15_e19184_d_n4, assign24600_body15_e19184_d_n5, assign24600_body15_e19184_d_n6, assign24600_body15_e19184_d_n7, assign24600_body15_e19184_d_n8, assign24600_body15_e19184_d_n9, assign24600_body15_e19184_d_n10, assign24600_body15_e19184_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard559 != 0.0)) && (locals.var_guard560 != 0.0)) {
        let assign24600_body15_e19182: f64 = (locals.var_xp + locals.var_xmp);
        (assign24600_body15_e19182, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn4 + locals.var_xmp_dn4), (locals.var_xp_dn5 + locals.var_xmp_dn5), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn8 + locals.var_xmp_dn8), (locals.var_xp_dn9 + locals.var_xmp_dn9), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn13 + locals.var_xmp_dn13),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    }
};
            locals.var_arg = assign24600_body15_e19184;
            locals.var_arg_dn0 = assign24600_body15_e19184_d_n0;
            locals.var_arg_dn2 = assign24600_body15_e19184_d_n2;
            locals.var_arg_dn4 = assign24600_body15_e19184_d_n4;
            locals.var_arg_dn5 = assign24600_body15_e19184_d_n5;
            locals.var_arg_dn6 = assign24600_body15_e19184_d_n6;
            locals.var_arg_dn7 = assign24600_body15_e19184_d_n7;
            locals.var_arg_dn8 = assign24600_body15_e19184_d_n8;
            locals.var_arg_dn9 = assign24600_body15_e19184_d_n9;
            locals.var_arg_dn10 = assign24600_body15_e19184_d_n10;
            locals.var_arg_dn13 = assign24600_body15_e19184_d_n13;
            locals.var_arg_rv = 0.0;
            let (assign24600_body16_e19194, assign24600_body16_e19194_d_n0, assign24600_body16_e19194_d_n2, assign24600_body16_e19194_d_n4, assign24600_body16_e19194_d_n5, assign24600_body16_e19194_d_n6, assign24600_body16_e19194_d_n7, assign24600_body16_e19194_d_n8, assign24600_body16_e19194_d_n9, assign24600_body16_e19194_d_n10, assign24600_body16_e19194_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard559 != 0.0)) && (locals.var_guard560 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
            locals.var_dnm = assign24600_body16_e19194;
            locals.var_dnm_dn0 = assign24600_body16_e19194_d_n0;
            locals.var_dnm_dn2 = assign24600_body16_e19194_d_n2;
            locals.var_dnm_dn4 = assign24600_body16_e19194_d_n4;
            locals.var_dnm_dn5 = assign24600_body16_e19194_d_n5;
            locals.var_dnm_dn6 = assign24600_body16_e19194_d_n6;
            locals.var_dnm_dn7 = assign24600_body16_e19194_d_n7;
            locals.var_dnm_dn8 = assign24600_body16_e19194_d_n8;
            locals.var_dnm_dn9 = assign24600_body16_e19194_d_n9;
            locals.var_dnm_dn10 = assign24600_body16_e19194_d_n10;
            locals.var_dnm_dn13 = assign24600_body16_e19194_d_n13;
            locals.var_dnm_rv = 0.0;
            let assign24600_body17_e19209: f64 = if ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0)) { 1.0 } else { 0.0 };
            locals.var_guard561 = assign24600_body17_e19209;
            locals.var_guard561_rv = 0.0;
            let assign24600_body18_e19212: f64 = if 2.0 == 1.0 { 1.0 } else { 0.0 };
            locals.var_guard562 = assign24600_body18_e19212;
            locals.var_guard562_rv = 0.0;
            let (assign24600_body19_e19226,) = {
    if ((((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard559 != 0.0)) && (locals.var_guard560 != 0.0)) && (locals.var_guard561 != 0.0)) && (locals.var_guard562 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
            locals.var_mm = assign24600_body19_e19226;
            locals.var_mm_rv = 0.0;
            let assign24600_body20_e19229: f64 = if 2.0 == 2.0 { 1.0 } else { 0.0 };
            locals.var_guard563 = assign24600_body20_e19229;
            locals.var_guard563_rv = 0.0;
            let (assign24600_body21_e19246,) = {
    if (((((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard559 != 0.0)) && (locals.var_guard560 != 0.0)) && (locals.var_guard561 != 0.0)) && (locals.var_guard562 == 0.0)) && (locals.var_guard563 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
            locals.var_mm = assign24600_body21_e19246;
            locals.var_mm_rv = 0.0;
            let assign24600_body22_e19249: f64 = if 2.0 == 4.0 { 1.0 } else { 0.0 };
            locals.var_guard564 = assign24600_body22_e19249;
            locals.var_guard564_rv = 0.0;
            let (assign24600_body23_e19269,) = {
    if ((((((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard559 != 0.0)) && (locals.var_guard560 != 0.0)) && (locals.var_guard561 != 0.0)) && (locals.var_guard562 == 0.0)) && (locals.var_guard563 == 0.0)) && (locals.var_guard564 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
            locals.var_mm = assign24600_body23_e19269;
            locals.var_mm_rv = 0.0;
            let assign24600_body24_e19272: f64 = if 2.0 == 8.0 { 1.0 } else { 0.0 };
            locals.var_guard565 = assign24600_body24_e19272;
            locals.var_guard565_rv = 0.0;
            let (assign24600_body25_e19295,) = {
    if (((((((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard559 != 0.0)) && (locals.var_guard560 != 0.0)) && (locals.var_guard561 != 0.0)) && (locals.var_guard562 == 0.0)) && (locals.var_guard563 == 0.0)) && (locals.var_guard564 == 0.0)) && (locals.var_guard565 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
            locals.var_mm = assign24600_body25_e19295;
            locals.var_mm_rv = 0.0;
            let (assign24600_body26_e19307,) = {
    if (((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard559 != 0.0)) && (locals.var_guard560 != 0.0)) && (locals.var_guard561 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign24600_body26_e19307;
            locals.var_m0_rv = 0.0;
            let mut assign24600_body27_loop_guard: usize = 0;
            while {
                let assign24600_body27_cond_e19320: f64 = if ((((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard559 != 0.0)) && (locals.var_guard560 != 0.0)) && (locals.var_guard561 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
                assign24600_body27_cond_e19320 != 0.0
            } {
                assign24600_body27_loop_guard += 1;
                assert!(assign24600_body27_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
                let (assign24600_body27_body0_e19333, assign24600_body27_body0_e19333_d_n0, assign24600_body27_body0_e19333_d_n2, assign24600_body27_body0_e19333_d_n4, assign24600_body27_body0_e19333_d_n5, assign24600_body27_body0_e19333_d_n6, assign24600_body27_body0_e19333_d_n7, assign24600_body27_body0_e19333_d_n8, assign24600_body27_body0_e19333_d_n9, assign24600_body27_body0_e19333_d_n10, assign24600_body27_body0_e19333_d_n13,) = {
    if (((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard559 != 0.0)) && (locals.var_guard560 != 0.0)) && (locals.var_guard561 != 0.0)) {
        let assign24600_body27_body0_e19331: f64 = (locals.var_dnm).sqrt();
        (assign24600_body27_body0_e19331, (locals.var_dnm_dn0 / (2.0 * assign24600_body27_body0_e19331)), (locals.var_dnm_dn2 / (2.0 * assign24600_body27_body0_e19331)), (locals.var_dnm_dn4 / (2.0 * assign24600_body27_body0_e19331)), (locals.var_dnm_dn5 / (2.0 * assign24600_body27_body0_e19331)), (locals.var_dnm_dn6 / (2.0 * assign24600_body27_body0_e19331)), (locals.var_dnm_dn7 / (2.0 * assign24600_body27_body0_e19331)), (locals.var_dnm_dn8 / (2.0 * assign24600_body27_body0_e19331)), (locals.var_dnm_dn9 / (2.0 * assign24600_body27_body0_e19331)), (locals.var_dnm_dn10 / (2.0 * assign24600_body27_body0_e19331)), (locals.var_dnm_dn13 / (2.0 * assign24600_body27_body0_e19331)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
                locals.var_dnm = assign24600_body27_body0_e19333;
                locals.var_dnm_dn0 = assign24600_body27_body0_e19333_d_n0;
                locals.var_dnm_dn2 = assign24600_body27_body0_e19333_d_n2;
                locals.var_dnm_dn4 = assign24600_body27_body0_e19333_d_n4;
                locals.var_dnm_dn5 = assign24600_body27_body0_e19333_d_n5;
                locals.var_dnm_dn6 = assign24600_body27_body0_e19333_d_n6;
                locals.var_dnm_dn7 = assign24600_body27_body0_e19333_d_n7;
                locals.var_dnm_dn8 = assign24600_body27_body0_e19333_d_n8;
                locals.var_dnm_dn9 = assign24600_body27_body0_e19333_d_n9;
                locals.var_dnm_dn10 = assign24600_body27_body0_e19333_d_n10;
                locals.var_dnm_dn13 = assign24600_body27_body0_e19333_d_n13;
                locals.var_dnm_rv = 0.0;
                let (assign24600_body27_body1_e19347,) = {
    if (((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard559 != 0.0)) && (locals.var_guard560 != 0.0)) && (locals.var_guard561 != 0.0)) {
        let assign24600_body27_body1_e19345: f64 = (locals.var_m0 + 1.0);
        (assign24600_body27_body1_e19345,)
    } else {
        (locals.var_m0,)
    }
};
                locals.var_m0 = assign24600_body27_body1_e19347;
                locals.var_m0_rv = 0.0;
            }
            let (assign24600_body28_e19371, assign24600_body28_e19371_d_n0, assign24600_body28_e19371_d_n2, assign24600_body28_e19371_d_n4, assign24600_body28_e19371_d_n5, assign24600_body28_e19371_d_n6, assign24600_body28_e19371_d_n7, assign24600_body28_e19371_d_n8, assign24600_body28_e19371_d_n9, assign24600_body28_e19371_d_n10, assign24600_body28_e19371_d_n13,) = {
    if (((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard559 != 0.0)) && (locals.var_guard560 != 0.0)) && (locals.var_guard561 == 0.0)) {
        let (assign24600_body28_e19369, assign24600_body28_e19369_d_n0, assign24600_body28_e19369_d_n2, assign24600_body28_e19369_d_n4, assign24600_body28_e19369_d_n5, assign24600_body28_e19369_d_n6, assign24600_body28_e19369_d_n7, assign24600_body28_e19369_d_n8, assign24600_body28_e19369_d_n9, assign24600_body28_e19369_d_n10, assign24600_body28_e19369_d_n13,) = {
            if (locals.var_dnm == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign24600_body28_e19366: f64 = (2.0 * 2.0);
                let assign24600_body28_e19367: f64 = (1.0 / assign24600_body28_e19366);
                let assign24600_body28_e19368: f64 = (locals.var_dnm).powf(assign24600_body28_e19367);
                (assign24600_body28_e19368, if 0.0 == 0.0 && ((assign24600_body28_e19367) as f64).is_finite() && ((assign24600_body28_e19367) as f64).fract() == 0.0 { if assign24600_body28_e19367 == 0.0 { 0.0 } else { (assign24600_body28_e19367 * ((locals.var_dnm).powf(assign24600_body28_e19367 - 1.0) * locals.var_dnm_dn0)) } } else { (assign24600_body28_e19368 * (assign24600_body28_e19367 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign24600_body28_e19367) as f64).is_finite() && ((assign24600_body28_e19367) as f64).fract() == 0.0 { if assign24600_body28_e19367 == 0.0 { 0.0 } else { (assign24600_body28_e19367 * ((locals.var_dnm).powf(assign24600_body28_e19367 - 1.0) * locals.var_dnm_dn2)) } } else { (assign24600_body28_e19368 * (assign24600_body28_e19367 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign24600_body28_e19367) as f64).is_finite() && ((assign24600_body28_e19367) as f64).fract() == 0.0 { if assign24600_body28_e19367 == 0.0 { 0.0 } else { (assign24600_body28_e19367 * ((locals.var_dnm).powf(assign24600_body28_e19367 - 1.0) * locals.var_dnm_dn4)) } } else { (assign24600_body28_e19368 * (assign24600_body28_e19367 * (locals.var_dnm_dn4 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign24600_body28_e19367) as f64).is_finite() && ((assign24600_body28_e19367) as f64).fract() == 0.0 { if assign24600_body28_e19367 == 0.0 { 0.0 } else { (assign24600_body28_e19367 * ((locals.var_dnm).powf(assign24600_body28_e19367 - 1.0) * locals.var_dnm_dn5)) } } else { (assign24600_body28_e19368 * (assign24600_body28_e19367 * (locals.var_dnm_dn5 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign24600_body28_e19367) as f64).is_finite() && ((assign24600_body28_e19367) as f64).fract() == 0.0 { if assign24600_body28_e19367 == 0.0 { 0.0 } else { (assign24600_body28_e19367 * ((locals.var_dnm).powf(assign24600_body28_e19367 - 1.0) * locals.var_dnm_dn6)) } } else { (assign24600_body28_e19368 * (assign24600_body28_e19367 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign24600_body28_e19367) as f64).is_finite() && ((assign24600_body28_e19367) as f64).fract() == 0.0 { if assign24600_body28_e19367 == 0.0 { 0.0 } else { (assign24600_body28_e19367 * ((locals.var_dnm).powf(assign24600_body28_e19367 - 1.0) * locals.var_dnm_dn7)) } } else { (assign24600_body28_e19368 * (assign24600_body28_e19367 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign24600_body28_e19367) as f64).is_finite() && ((assign24600_body28_e19367) as f64).fract() == 0.0 { if assign24600_body28_e19367 == 0.0 { 0.0 } else { (assign24600_body28_e19367 * ((locals.var_dnm).powf(assign24600_body28_e19367 - 1.0) * locals.var_dnm_dn8)) } } else { (assign24600_body28_e19368 * (assign24600_body28_e19367 * (locals.var_dnm_dn8 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign24600_body28_e19367) as f64).is_finite() && ((assign24600_body28_e19367) as f64).fract() == 0.0 { if assign24600_body28_e19367 == 0.0 { 0.0 } else { (assign24600_body28_e19367 * ((locals.var_dnm).powf(assign24600_body28_e19367 - 1.0) * locals.var_dnm_dn9)) } } else { (assign24600_body28_e19368 * (assign24600_body28_e19367 * (locals.var_dnm_dn9 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign24600_body28_e19367) as f64).is_finite() && ((assign24600_body28_e19367) as f64).fract() == 0.0 { if assign24600_body28_e19367 == 0.0 { 0.0 } else { (assign24600_body28_e19367 * ((locals.var_dnm).powf(assign24600_body28_e19367 - 1.0) * locals.var_dnm_dn10)) } } else { (assign24600_body28_e19368 * (assign24600_body28_e19367 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign24600_body28_e19367) as f64).is_finite() && ((assign24600_body28_e19367) as f64).fract() == 0.0 { if assign24600_body28_e19367 == 0.0 { 0.0 } else { (assign24600_body28_e19367 * ((locals.var_dnm).powf(assign24600_body28_e19367 - 1.0) * locals.var_dnm_dn13)) } } else { (assign24600_body28_e19368 * (assign24600_body28_e19367 * (locals.var_dnm_dn13 / locals.var_dnm))) },)
            }
        };
        (assign24600_body28_e19369, assign24600_body28_e19369_d_n0, assign24600_body28_e19369_d_n2, assign24600_body28_e19369_d_n4, assign24600_body28_e19369_d_n5, assign24600_body28_e19369_d_n6, assign24600_body28_e19369_d_n7, assign24600_body28_e19369_d_n8, assign24600_body28_e19369_d_n9, assign24600_body28_e19369_d_n10, assign24600_body28_e19369_d_n13,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
            locals.var_dnm = assign24600_body28_e19371;
            locals.var_dnm_dn0 = assign24600_body28_e19371_d_n0;
            locals.var_dnm_dn2 = assign24600_body28_e19371_d_n2;
            locals.var_dnm_dn4 = assign24600_body28_e19371_d_n4;
            locals.var_dnm_dn5 = assign24600_body28_e19371_d_n5;
            locals.var_dnm_dn6 = assign24600_body28_e19371_d_n6;
            locals.var_dnm_dn7 = assign24600_body28_e19371_d_n7;
            locals.var_dnm_dn8 = assign24600_body28_e19371_d_n8;
            locals.var_dnm_dn9 = assign24600_body28_e19371_d_n9;
            locals.var_dnm_dn10 = assign24600_body28_e19371_d_n10;
            locals.var_dnm_dn13 = assign24600_body28_e19371_d_n13;
            locals.var_dnm_rv = 0.0;
            let (assign24600_body29_e19383, assign24600_body29_e19383_d_n0, assign24600_body29_e19383_d_n2, assign24600_body29_e19383_d_n4, assign24600_body29_e19383_d_n5, assign24600_body29_e19383_d_n6, assign24600_body29_e19383_d_n7, assign24600_body29_e19383_d_n8, assign24600_body29_e19383_d_n9, assign24600_body29_e19383_d_n10, assign24600_body29_e19383_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard559 != 0.0)) && (locals.var_guard560 != 0.0)) {
        let assign24600_body29_e19381: f64 = (1.0 / locals.var_dnm);
        (assign24600_body29_e19381, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn4 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn5 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn8 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn9 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn13 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
            locals.var_dnm = assign24600_body29_e19383;
            locals.var_dnm_dn0 = assign24600_body29_e19383_d_n0;
            locals.var_dnm_dn2 = assign24600_body29_e19383_d_n2;
            locals.var_dnm_dn4 = assign24600_body29_e19383_d_n4;
            locals.var_dnm_dn5 = assign24600_body29_e19383_d_n5;
            locals.var_dnm_dn6 = assign24600_body29_e19383_d_n6;
            locals.var_dnm_dn7 = assign24600_body29_e19383_d_n7;
            locals.var_dnm_dn8 = assign24600_body29_e19383_d_n8;
            locals.var_dnm_dn9 = assign24600_body29_e19383_d_n9;
            locals.var_dnm_dn10 = assign24600_body29_e19383_d_n10;
            locals.var_dnm_dn13 = assign24600_body29_e19383_d_n13;
            locals.var_dnm_rv = 0.0;
            let (assign24600_body30_e19397, assign24600_body30_e19397_d_n0, assign24600_body30_e19397_d_n2, assign24600_body30_e19397_d_n4, assign24600_body30_e19397_d_n5, assign24600_body30_e19397_d_n6, assign24600_body30_e19397_d_n7, assign24600_body30_e19397_d_n8, assign24600_body30_e19397_d_n9, assign24600_body30_e19397_d_n10, assign24600_body30_e19397_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard559 != 0.0)) && (locals.var_guard560 != 0.0)) {
        let assign24600_body30_e19393: f64 = (locals.var_tmf1 * 1e-8);
        let assign24600_body30_e19395: f64 = (assign24600_body30_e19393 * locals.var_dnm);
        (assign24600_body30_e19395, (((locals.var_tmf1_dn0 * 1e-8) * locals.var_dnm) + (assign24600_body30_e19393 * locals.var_dnm_dn0)), (((locals.var_tmf1_dn2 * 1e-8) * locals.var_dnm) + (assign24600_body30_e19393 * locals.var_dnm_dn2)), (((locals.var_tmf1_dn4 * 1e-8) * locals.var_dnm) + (assign24600_body30_e19393 * locals.var_dnm_dn4)), (((locals.var_tmf1_dn5 * 1e-8) * locals.var_dnm) + (assign24600_body30_e19393 * locals.var_dnm_dn5)), (((locals.var_tmf1_dn6 * 1e-8) * locals.var_dnm) + (assign24600_body30_e19393 * locals.var_dnm_dn6)), (((locals.var_tmf1_dn7 * 1e-8) * locals.var_dnm) + (assign24600_body30_e19393 * locals.var_dnm_dn7)), (((locals.var_tmf1_dn8 * 1e-8) * locals.var_dnm) + (assign24600_body30_e19393 * locals.var_dnm_dn8)), (((locals.var_tmf1_dn9 * 1e-8) * locals.var_dnm) + (assign24600_body30_e19393 * locals.var_dnm_dn9)), (((locals.var_tmf1_dn10 * 1e-8) * locals.var_dnm) + (assign24600_body30_e19393 * locals.var_dnm_dn10)), (((locals.var_tmf1_dn13 * 1e-8) * locals.var_dnm) + (assign24600_body30_e19393 * locals.var_dnm_dn13)),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn13,)
    }
};
            locals.var_tmf0 = assign24600_body30_e19397;
            locals.var_tmf0_dn0 = assign24600_body30_e19397_d_n0;
            locals.var_tmf0_dn2 = assign24600_body30_e19397_d_n2;
            locals.var_tmf0_dn4 = assign24600_body30_e19397_d_n4;
            locals.var_tmf0_dn5 = assign24600_body30_e19397_d_n5;
            locals.var_tmf0_dn6 = assign24600_body30_e19397_d_n6;
            locals.var_tmf0_dn7 = assign24600_body30_e19397_d_n7;
            locals.var_tmf0_dn8 = assign24600_body30_e19397_d_n8;
            locals.var_tmf0_dn9 = assign24600_body30_e19397_d_n9;
            locals.var_tmf0_dn10 = assign24600_body30_e19397_d_n10;
            locals.var_tmf0_dn13 = assign24600_body30_e19397_d_n13;
            locals.var_tmf0_rv = 0.0;
            let (assign24600_body31_e19413, assign24600_body31_e19413_d_n0, assign24600_body31_e19413_d_n2, assign24600_body31_e19413_d_n4, assign24600_body31_e19413_d_n5, assign24600_body31_e19413_d_n6, assign24600_body31_e19413_d_n7, assign24600_body31_e19413_d_n8, assign24600_body31_e19413_d_n9, assign24600_body31_e19413_d_n10, assign24600_body31_e19413_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard559 != 0.0)) && (locals.var_guard560 != 0.0)) {
        let assign24600_body31_e19407: f64 = (1e-8 * locals.var_xmp);
        let assign24600_body31_e19409: f64 = (assign24600_body31_e19407 * locals.var_dnm);
        let assign24600_body31_e19411: f64 = (assign24600_body31_e19409 / locals.var_arg);
        (assign24600_body31_e19411, ((((((1e-8 * locals.var_xmp_dn0) * locals.var_dnm) + (assign24600_body31_e19407 * locals.var_dnm_dn0)) * locals.var_arg) - (assign24600_body31_e19409 * locals.var_arg_dn0)) / (locals.var_arg * locals.var_arg)), ((((((1e-8 * locals.var_xmp_dn2) * locals.var_dnm) + (assign24600_body31_e19407 * locals.var_dnm_dn2)) * locals.var_arg) - (assign24600_body31_e19409 * locals.var_arg_dn2)) / (locals.var_arg * locals.var_arg)), ((((((1e-8 * locals.var_xmp_dn4) * locals.var_dnm) + (assign24600_body31_e19407 * locals.var_dnm_dn4)) * locals.var_arg) - (assign24600_body31_e19409 * locals.var_arg_dn4)) / (locals.var_arg * locals.var_arg)), ((((((1e-8 * locals.var_xmp_dn5) * locals.var_dnm) + (assign24600_body31_e19407 * locals.var_dnm_dn5)) * locals.var_arg) - (assign24600_body31_e19409 * locals.var_arg_dn5)) / (locals.var_arg * locals.var_arg)), ((((((1e-8 * locals.var_xmp_dn6) * locals.var_dnm) + (assign24600_body31_e19407 * locals.var_dnm_dn6)) * locals.var_arg) - (assign24600_body31_e19409 * locals.var_arg_dn6)) / (locals.var_arg * locals.var_arg)), ((((((1e-8 * locals.var_xmp_dn7) * locals.var_dnm) + (assign24600_body31_e19407 * locals.var_dnm_dn7)) * locals.var_arg) - (assign24600_body31_e19409 * locals.var_arg_dn7)) / (locals.var_arg * locals.var_arg)), ((((((1e-8 * locals.var_xmp_dn8) * locals.var_dnm) + (assign24600_body31_e19407 * locals.var_dnm_dn8)) * locals.var_arg) - (assign24600_body31_e19409 * locals.var_arg_dn8)) / (locals.var_arg * locals.var_arg)), ((((((1e-8 * locals.var_xmp_dn9) * locals.var_dnm) + (assign24600_body31_e19407 * locals.var_dnm_dn9)) * locals.var_arg) - (assign24600_body31_e19409 * locals.var_arg_dn9)) / (locals.var_arg * locals.var_arg)), ((((((1e-8 * locals.var_xmp_dn10) * locals.var_dnm) + (assign24600_body31_e19407 * locals.var_dnm_dn10)) * locals.var_arg) - (assign24600_body31_e19409 * locals.var_arg_dn10)) / (locals.var_arg * locals.var_arg)), ((((((1e-8 * locals.var_xmp_dn13) * locals.var_dnm) + (assign24600_body31_e19407 * locals.var_dnm_dn13)) * locals.var_arg) - (assign24600_body31_e19409 * locals.var_arg_dn13)) / (locals.var_arg * locals.var_arg)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
            locals.var_t0 = assign24600_body31_e19413;
            locals.var_t0_dn0 = assign24600_body31_e19413_d_n0;
            locals.var_t0_dn2 = assign24600_body31_e19413_d_n2;
            locals.var_t0_dn4 = assign24600_body31_e19413_d_n4;
            locals.var_t0_dn5 = assign24600_body31_e19413_d_n5;
            locals.var_t0_dn6 = assign24600_body31_e19413_d_n6;
            locals.var_t0_dn7 = assign24600_body31_e19413_d_n7;
            locals.var_t0_dn8 = assign24600_body31_e19413_d_n8;
            locals.var_t0_dn9 = assign24600_body31_e19413_d_n9;
            locals.var_t0_dn10 = assign24600_body31_e19413_d_n10;
            locals.var_t0_dn13 = assign24600_body31_e19413_d_n13;
            locals.var_t0_rv = 0.0;
            let (assign24600_body32_e19427, assign24600_body32_e19427_d_n0, assign24600_body32_e19427_d_n2, assign24600_body32_e19427_d_n4, assign24600_body32_e19427_d_n5, assign24600_body32_e19427_d_n6, assign24600_body32_e19427_d_n7, assign24600_body32_e19427_d_n8, assign24600_body32_e19427_d_n9, assign24600_body32_e19427_d_n10, assign24600_body32_e19427_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard559 != 0.0)) && (locals.var_guard560 != 0.0)) {
        let assign24600_body32_e19423: f64 = (locals.var_uc_depthn - 1e-8);
        let assign24600_body32_e19425: f64 = (assign24600_body32_e19423 + locals.var_tmf0);
        (assign24600_body32_e19425, (locals.var_uc_depthn_dn0 + locals.var_tmf0_dn0), (locals.var_uc_depthn_dn2 + locals.var_tmf0_dn2), (locals.var_uc_depthn_dn4 + locals.var_tmf0_dn4), (locals.var_uc_depthn_dn5 + locals.var_tmf0_dn5), (locals.var_uc_depthn_dn6 + locals.var_tmf0_dn6), (locals.var_uc_depthn_dn7 + locals.var_tmf0_dn7), (locals.var_uc_depthn_dn8 + locals.var_tmf0_dn8), (locals.var_uc_depthn_dn9 + locals.var_tmf0_dn9), (locals.var_uc_depthn_dn10 + locals.var_tmf0_dn10), (locals.var_uc_depthn_dn13 + locals.var_tmf0_dn13),)
    } else {
        (locals.var_w_b0, locals.var_w_b0_dn0, locals.var_w_b0_dn2, locals.var_w_b0_dn4, locals.var_w_b0_dn5, locals.var_w_b0_dn6, locals.var_w_b0_dn7, locals.var_w_b0_dn8, locals.var_w_b0_dn9, locals.var_w_b0_dn10, locals.var_w_b0_dn13,)
    }
};
            locals.var_w_b0 = assign24600_body32_e19427;
            locals.var_w_b0_dn0 = assign24600_body32_e19427_d_n0;
            locals.var_w_b0_dn2 = assign24600_body32_e19427_d_n2;
            locals.var_w_b0_dn4 = assign24600_body32_e19427_d_n4;
            locals.var_w_b0_dn5 = assign24600_body32_e19427_d_n5;
            locals.var_w_b0_dn6 = assign24600_body32_e19427_d_n6;
            locals.var_w_b0_dn7 = assign24600_body32_e19427_d_n7;
            locals.var_w_b0_dn8 = assign24600_body32_e19427_d_n8;
            locals.var_w_b0_dn9 = assign24600_body32_e19427_d_n9;
            locals.var_w_b0_dn10 = assign24600_body32_e19427_d_n10;
            locals.var_w_b0_dn13 = assign24600_body32_e19427_d_n13;
            locals.var_w_b0_rv = 0.0;
            let (assign24600_body33_e19437, assign24600_body33_e19437_d_n0, assign24600_body33_e19437_d_n2, assign24600_body33_e19437_d_n4, assign24600_body33_e19437_d_n5, assign24600_body33_e19437_d_n6, assign24600_body33_e19437_d_n7, assign24600_body33_e19437_d_n8, assign24600_body33_e19437_d_n9, assign24600_body33_e19437_d_n10, assign24600_body33_e19437_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard559 != 0.0)) && (locals.var_guard560 != 0.0)) {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
            locals.var_t0 = assign24600_body33_e19437;
            locals.var_t0_dn0 = assign24600_body33_e19437_d_n0;
            locals.var_t0_dn2 = assign24600_body33_e19437_d_n2;
            locals.var_t0_dn4 = assign24600_body33_e19437_d_n4;
            locals.var_t0_dn5 = assign24600_body33_e19437_d_n5;
            locals.var_t0_dn6 = assign24600_body33_e19437_d_n6;
            locals.var_t0_dn7 = assign24600_body33_e19437_d_n7;
            locals.var_t0_dn8 = assign24600_body33_e19437_d_n8;
            locals.var_t0_dn9 = assign24600_body33_e19437_d_n9;
            locals.var_t0_dn10 = assign24600_body33_e19437_d_n10;
            locals.var_t0_dn13 = assign24600_body33_e19437_d_n13;
            locals.var_t0_rv = 0.0;
            let (assign24600_body34_e19448, assign24600_body34_e19448_d_n0, assign24600_body34_e19448_d_n2, assign24600_body34_e19448_d_n4, assign24600_body34_e19448_d_n5, assign24600_body34_e19448_d_n6, assign24600_body34_e19448_d_n7, assign24600_body34_e19448_d_n8, assign24600_body34_e19448_d_n9, assign24600_body34_e19448_d_n10, assign24600_body34_e19448_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard559 != 0.0)) && (locals.var_guard560 == 0.0)) {
        (locals.var_w_b0, locals.var_w_b0_dn0, locals.var_w_b0_dn2, locals.var_w_b0_dn4, locals.var_w_b0_dn5, locals.var_w_b0_dn6, locals.var_w_b0_dn7, locals.var_w_b0_dn8, locals.var_w_b0_dn9, locals.var_w_b0_dn10, locals.var_w_b0_dn13,)
    } else {
        (locals.var_w_b0, locals.var_w_b0_dn0, locals.var_w_b0_dn2, locals.var_w_b0_dn4, locals.var_w_b0_dn5, locals.var_w_b0_dn6, locals.var_w_b0_dn7, locals.var_w_b0_dn8, locals.var_w_b0_dn9, locals.var_w_b0_dn10, locals.var_w_b0_dn13,)
    }
};
            locals.var_w_b0 = assign24600_body34_e19448;
            locals.var_w_b0_dn0 = assign24600_body34_e19448_d_n0;
            locals.var_w_b0_dn2 = assign24600_body34_e19448_d_n2;
            locals.var_w_b0_dn4 = assign24600_body34_e19448_d_n4;
            locals.var_w_b0_dn5 = assign24600_body34_e19448_d_n5;
            locals.var_w_b0_dn6 = assign24600_body34_e19448_d_n6;
            locals.var_w_b0_dn7 = assign24600_body34_e19448_d_n7;
            locals.var_w_b0_dn8 = assign24600_body34_e19448_d_n8;
            locals.var_w_b0_dn9 = assign24600_body34_e19448_d_n9;
            locals.var_w_b0_dn10 = assign24600_body34_e19448_d_n10;
            locals.var_w_b0_dn13 = assign24600_body34_e19448_d_n13;
            locals.var_w_b0_rv = 0.0;
            let (assign24600_body35_e19459, assign24600_body35_e19459_d_n0, assign24600_body35_e19459_d_n2, assign24600_body35_e19459_d_n4, assign24600_body35_e19459_d_n5, assign24600_body35_e19459_d_n6, assign24600_body35_e19459_d_n7, assign24600_body35_e19459_d_n8, assign24600_body35_e19459_d_n9, assign24600_body35_e19459_d_n10, assign24600_body35_e19459_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard559 != 0.0)) && (locals.var_guard560 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
            locals.var_t0 = assign24600_body35_e19459;
            locals.var_t0_dn0 = assign24600_body35_e19459_d_n0;
            locals.var_t0_dn2 = assign24600_body35_e19459_d_n2;
            locals.var_t0_dn4 = assign24600_body35_e19459_d_n4;
            locals.var_t0_dn5 = assign24600_body35_e19459_d_n5;
            locals.var_t0_dn6 = assign24600_body35_e19459_d_n6;
            locals.var_t0_dn7 = assign24600_body35_e19459_d_n7;
            locals.var_t0_dn8 = assign24600_body35_e19459_d_n8;
            locals.var_t0_dn9 = assign24600_body35_e19459_d_n9;
            locals.var_t0_dn10 = assign24600_body35_e19459_d_n10;
            locals.var_t0_dn13 = assign24600_body35_e19459_d_n13;
            locals.var_t0_rv = 0.0;
            let (assign24600_body36_e19471, assign24600_body36_e19471_d_n0, assign24600_body36_e19471_d_n2, assign24600_body36_e19471_d_n4, assign24600_body36_e19471_d_n5, assign24600_body36_e19471_d_n6, assign24600_body36_e19471_d_n7, assign24600_body36_e19471_d_n8, assign24600_body36_e19471_d_n9, assign24600_body36_e19471_d_n10, assign24600_body36_e19471_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard559 != 0.0)) {
        let assign24600_body36_e19467: f64 = (locals.var_phi_j0_dep - locals.var_vbscl__blk435);
        let assign24600_body36_e19469: f64 = (assign24600_body36_e19467 + locals.var_vbi_dep);
        (assign24600_body36_e19469, ((locals.var_phi_j0_dep_dn0 - locals.var_vbscl__blk435_dn0) + locals.var_vbi_dep_dn0), ((locals.var_phi_j0_dep_dn2 - locals.var_vbscl__blk435_dn2) + locals.var_vbi_dep_dn2), ((locals.var_phi_j0_dep_dn4 - locals.var_vbscl__blk435_dn4) + locals.var_vbi_dep_dn4), ((locals.var_phi_j0_dep_dn5 - locals.var_vbscl__blk435_dn5) + locals.var_vbi_dep_dn5), ((locals.var_phi_j0_dep_dn6 - locals.var_vbscl__blk435_dn6) + locals.var_vbi_dep_dn6), ((locals.var_phi_j0_dep_dn7 - locals.var_vbscl__blk435_dn7) + locals.var_vbi_dep_dn7), ((locals.var_phi_j0_dep_dn8 - locals.var_vbscl__blk435_dn8) + locals.var_vbi_dep_dn8), ((locals.var_phi_j0_dep_dn9 - locals.var_vbscl__blk435_dn9) + locals.var_vbi_dep_dn9), ((locals.var_phi_j0_dep_dn10 - locals.var_vbscl__blk435_dn10) + locals.var_vbi_dep_dn10), ((locals.var_phi_j0_dep_dn13 - locals.var_vbscl__blk435_dn13) + locals.var_vbi_dep_dn13),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
            locals.var_t1 = assign24600_body36_e19471;
            locals.var_t1_dn0 = assign24600_body36_e19471_d_n0;
            locals.var_t1_dn2 = assign24600_body36_e19471_d_n2;
            locals.var_t1_dn4 = assign24600_body36_e19471_d_n4;
            locals.var_t1_dn5 = assign24600_body36_e19471_d_n5;
            locals.var_t1_dn6 = assign24600_body36_e19471_d_n6;
            locals.var_t1_dn7 = assign24600_body36_e19471_d_n7;
            locals.var_t1_dn8 = assign24600_body36_e19471_d_n8;
            locals.var_t1_dn9 = assign24600_body36_e19471_d_n9;
            locals.var_t1_dn10 = assign24600_body36_e19471_d_n10;
            locals.var_t1_dn13 = assign24600_body36_e19471_d_n13;
            locals.var_t1_rv = 0.0;
            let assign24600_body37_e19475: f64 = 0.1;
            let assign24600_body37_e19480: f64 = if ((locals.var_t1 < assign24600_body37_e19475) && (0.1 >= 0.0)) { 1.0 } else { 0.0 };
            locals.var_guard566 = assign24600_body37_e19480;
            locals.var_guard566_rv = 0.0;
            let (assign24600_body38_e19494, assign24600_body38_e19494_d_n0, assign24600_body38_e19494_d_n2, assign24600_body38_e19494_d_n4, assign24600_body38_e19494_d_n5, assign24600_body38_e19494_d_n6, assign24600_body38_e19494_d_n7, assign24600_body38_e19494_d_n8, assign24600_body38_e19494_d_n9, assign24600_body38_e19494_d_n10, assign24600_body38_e19494_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard559 != 0.0)) && (locals.var_guard566 != 0.0)) {
        let assign24600_body38_e19490: f64 = 0.1;
        let assign24600_body38_e19492: f64 = (assign24600_body38_e19490 - locals.var_t1);
        (assign24600_body38_e19492, (-locals.var_t1_dn0), (-locals.var_t1_dn2), (-locals.var_t1_dn4), (-locals.var_t1_dn5), (-locals.var_t1_dn6), (-locals.var_t1_dn7), (-locals.var_t1_dn8), (-locals.var_t1_dn9), (-locals.var_t1_dn10), (-locals.var_t1_dn13),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13,)
    }
};
            locals.var_tmf1 = assign24600_body38_e19494;
            locals.var_tmf1_dn0 = assign24600_body38_e19494_d_n0;
            locals.var_tmf1_dn2 = assign24600_body38_e19494_d_n2;
            locals.var_tmf1_dn4 = assign24600_body38_e19494_d_n4;
            locals.var_tmf1_dn5 = assign24600_body38_e19494_d_n5;
            locals.var_tmf1_dn6 = assign24600_body38_e19494_d_n6;
            locals.var_tmf1_dn7 = assign24600_body38_e19494_d_n7;
            locals.var_tmf1_dn8 = assign24600_body38_e19494_d_n8;
            locals.var_tmf1_dn9 = assign24600_body38_e19494_d_n9;
            locals.var_tmf1_dn10 = assign24600_body38_e19494_d_n10;
            locals.var_tmf1_dn13 = assign24600_body38_e19494_d_n13;
            locals.var_tmf1_rv = 0.0;
            let (assign24600_body39_e19506, assign24600_body39_e19506_d_n0, assign24600_body39_e19506_d_n2, assign24600_body39_e19506_d_n4, assign24600_body39_e19506_d_n5, assign24600_body39_e19506_d_n6, assign24600_body39_e19506_d_n7, assign24600_body39_e19506_d_n8, assign24600_body39_e19506_d_n9, assign24600_body39_e19506_d_n10, assign24600_body39_e19506_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard559 != 0.0)) && (locals.var_guard566 != 0.0)) {
        let assign24600_body39_e19504: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign24600_body39_e19504, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)), ((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)), ((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn13 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn13)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn4, locals.var_x2_dn5, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn8, locals.var_x2_dn9, locals.var_x2_dn10, locals.var_x2_dn13,)
    }
};
            locals.var_x2 = assign24600_body39_e19506;
            locals.var_x2_dn0 = assign24600_body39_e19506_d_n0;
            locals.var_x2_dn2 = assign24600_body39_e19506_d_n2;
            locals.var_x2_dn4 = assign24600_body39_e19506_d_n4;
            locals.var_x2_dn5 = assign24600_body39_e19506_d_n5;
            locals.var_x2_dn6 = assign24600_body39_e19506_d_n6;
            locals.var_x2_dn7 = assign24600_body39_e19506_d_n7;
            locals.var_x2_dn8 = assign24600_body39_e19506_d_n8;
            locals.var_x2_dn9 = assign24600_body39_e19506_d_n9;
            locals.var_x2_dn10 = assign24600_body39_e19506_d_n10;
            locals.var_x2_dn13 = assign24600_body39_e19506_d_n13;
            locals.var_x2_rv = 0.0;
            let (assign24600_body40_e19518, assign24600_body40_e19518_d_n0, assign24600_body40_e19518_d_n2, assign24600_body40_e19518_d_n4, assign24600_body40_e19518_d_n5, assign24600_body40_e19518_d_n6, assign24600_body40_e19518_d_n7, assign24600_body40_e19518_d_n8, assign24600_body40_e19518_d_n9, assign24600_body40_e19518_d_n10, assign24600_body40_e19518_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard559 != 0.0)) && (locals.var_guard566 != 0.0)) {
        let assign24600_body40_e19516: f64 = (0.1 * 0.1);
        (assign24600_body40_e19516, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn4, locals.var_xmax2_dn5, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn8, locals.var_xmax2_dn9, locals.var_xmax2_dn10, locals.var_xmax2_dn13,)
    }
};
            locals.var_xmax2 = assign24600_body40_e19518;
            locals.var_xmax2_dn0 = assign24600_body40_e19518_d_n0;
            locals.var_xmax2_dn2 = assign24600_body40_e19518_d_n2;
            locals.var_xmax2_dn4 = assign24600_body40_e19518_d_n4;
            locals.var_xmax2_dn5 = assign24600_body40_e19518_d_n5;
            locals.var_xmax2_dn6 = assign24600_body40_e19518_d_n6;
            locals.var_xmax2_dn7 = assign24600_body40_e19518_d_n7;
            locals.var_xmax2_dn8 = assign24600_body40_e19518_d_n8;
            locals.var_xmax2_dn9 = assign24600_body40_e19518_d_n9;
            locals.var_xmax2_dn10 = assign24600_body40_e19518_d_n10;
            locals.var_xmax2_dn13 = assign24600_body40_e19518_d_n13;
            locals.var_xmax2_rv = 0.0;
            let (assign24600_body41_e19528, assign24600_body41_e19528_d_n0, assign24600_body41_e19528_d_n2, assign24600_body41_e19528_d_n4, assign24600_body41_e19528_d_n5, assign24600_body41_e19528_d_n6, assign24600_body41_e19528_d_n7, assign24600_body41_e19528_d_n8, assign24600_body41_e19528_d_n9, assign24600_body41_e19528_d_n10, assign24600_body41_e19528_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard559 != 0.0)) && (locals.var_guard566 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
            locals.var_xp = assign24600_body41_e19528;
            locals.var_xp_dn0 = assign24600_body41_e19528_d_n0;
            locals.var_xp_dn2 = assign24600_body41_e19528_d_n2;
            locals.var_xp_dn4 = assign24600_body41_e19528_d_n4;
            locals.var_xp_dn5 = assign24600_body41_e19528_d_n5;
            locals.var_xp_dn6 = assign24600_body41_e19528_d_n6;
            locals.var_xp_dn7 = assign24600_body41_e19528_d_n7;
            locals.var_xp_dn8 = assign24600_body41_e19528_d_n8;
            locals.var_xp_dn9 = assign24600_body41_e19528_d_n9;
            locals.var_xp_dn10 = assign24600_body41_e19528_d_n10;
            locals.var_xp_dn13 = assign24600_body41_e19528_d_n13;
            locals.var_xp_rv = 0.0;
            let (assign24600_body42_e19538, assign24600_body42_e19538_d_n0, assign24600_body42_e19538_d_n2, assign24600_body42_e19538_d_n4, assign24600_body42_e19538_d_n5, assign24600_body42_e19538_d_n6, assign24600_body42_e19538_d_n7, assign24600_body42_e19538_d_n8, assign24600_body42_e19538_d_n9, assign24600_body42_e19538_d_n10, assign24600_body42_e19538_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard559 != 0.0)) && (locals.var_guard566 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
            locals.var_xmp = assign24600_body42_e19538;
            locals.var_xmp_dn0 = assign24600_body42_e19538_d_n0;
            locals.var_xmp_dn2 = assign24600_body42_e19538_d_n2;
            locals.var_xmp_dn4 = assign24600_body42_e19538_d_n4;
            locals.var_xmp_dn5 = assign24600_body42_e19538_d_n5;
            locals.var_xmp_dn6 = assign24600_body42_e19538_d_n6;
            locals.var_xmp_dn7 = assign24600_body42_e19538_d_n7;
            locals.var_xmp_dn8 = assign24600_body42_e19538_d_n8;
            locals.var_xmp_dn9 = assign24600_body42_e19538_d_n9;
            locals.var_xmp_dn10 = assign24600_body42_e19538_d_n10;
            locals.var_xmp_dn13 = assign24600_body42_e19538_d_n13;
            locals.var_xmp_rv = 0.0;
            let (assign24600_body43_e19548,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard559 != 0.0)) && (locals.var_guard566 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign24600_body43_e19548;
            locals.var_m0_rv = 0.0;
            let (assign24600_body44_e19558,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard559 != 0.0)) && (locals.var_guard566 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
            locals.var_mm = assign24600_body44_e19558;
            locals.var_mm_rv = 0.0;
            let (assign24600_body45_e19568, assign24600_body45_e19568_d_n0, assign24600_body45_e19568_d_n2, assign24600_body45_e19568_d_n4, assign24600_body45_e19568_d_n5, assign24600_body45_e19568_d_n6, assign24600_body45_e19568_d_n7, assign24600_body45_e19568_d_n8, assign24600_body45_e19568_d_n9, assign24600_body45_e19568_d_n10, assign24600_body45_e19568_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard559 != 0.0)) && (locals.var_guard566 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    }
};
            locals.var_arg = assign24600_body45_e19568;
            locals.var_arg_dn0 = assign24600_body45_e19568_d_n0;
            locals.var_arg_dn2 = assign24600_body45_e19568_d_n2;
            locals.var_arg_dn4 = assign24600_body45_e19568_d_n4;
            locals.var_arg_dn5 = assign24600_body45_e19568_d_n5;
            locals.var_arg_dn6 = assign24600_body45_e19568_d_n6;
            locals.var_arg_dn7 = assign24600_body45_e19568_d_n7;
            locals.var_arg_dn8 = assign24600_body45_e19568_d_n8;
            locals.var_arg_dn9 = assign24600_body45_e19568_d_n9;
            locals.var_arg_dn10 = assign24600_body45_e19568_d_n10;
            locals.var_arg_dn13 = assign24600_body45_e19568_d_n13;
            locals.var_arg_rv = 0.0;
            let (assign24600_body46_e19578, assign24600_body46_e19578_d_n0, assign24600_body46_e19578_d_n2, assign24600_body46_e19578_d_n4, assign24600_body46_e19578_d_n5, assign24600_body46_e19578_d_n6, assign24600_body46_e19578_d_n7, assign24600_body46_e19578_d_n8, assign24600_body46_e19578_d_n9, assign24600_body46_e19578_d_n10, assign24600_body46_e19578_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard559 != 0.0)) && (locals.var_guard566 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
            locals.var_dnm = assign24600_body46_e19578;
            locals.var_dnm_dn0 = assign24600_body46_e19578_d_n0;
            locals.var_dnm_dn2 = assign24600_body46_e19578_d_n2;
            locals.var_dnm_dn4 = assign24600_body46_e19578_d_n4;
            locals.var_dnm_dn5 = assign24600_body46_e19578_d_n5;
            locals.var_dnm_dn6 = assign24600_body46_e19578_d_n6;
            locals.var_dnm_dn7 = assign24600_body46_e19578_d_n7;
            locals.var_dnm_dn8 = assign24600_body46_e19578_d_n8;
            locals.var_dnm_dn9 = assign24600_body46_e19578_d_n9;
            locals.var_dnm_dn10 = assign24600_body46_e19578_d_n10;
            locals.var_dnm_dn13 = assign24600_body46_e19578_d_n13;
            locals.var_dnm_rv = 0.0;
            let (assign24600_body47_e19590, assign24600_body47_e19590_d_n0, assign24600_body47_e19590_d_n2, assign24600_body47_e19590_d_n4, assign24600_body47_e19590_d_n5, assign24600_body47_e19590_d_n6, assign24600_body47_e19590_d_n7, assign24600_body47_e19590_d_n8, assign24600_body47_e19590_d_n9, assign24600_body47_e19590_d_n10, assign24600_body47_e19590_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard559 != 0.0)) && (locals.var_guard566 != 0.0)) {
        let assign24600_body47_e19588: f64 = (locals.var_xp * locals.var_x2);
        (assign24600_body47_e19588, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn13 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn13)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
            locals.var_xp = assign24600_body47_e19590;
            locals.var_xp_dn0 = assign24600_body47_e19590_d_n0;
            locals.var_xp_dn2 = assign24600_body47_e19590_d_n2;
            locals.var_xp_dn4 = assign24600_body47_e19590_d_n4;
            locals.var_xp_dn5 = assign24600_body47_e19590_d_n5;
            locals.var_xp_dn6 = assign24600_body47_e19590_d_n6;
            locals.var_xp_dn7 = assign24600_body47_e19590_d_n7;
            locals.var_xp_dn8 = assign24600_body47_e19590_d_n8;
            locals.var_xp_dn9 = assign24600_body47_e19590_d_n9;
            locals.var_xp_dn10 = assign24600_body47_e19590_d_n10;
            locals.var_xp_dn13 = assign24600_body47_e19590_d_n13;
            locals.var_xp_rv = 0.0;
            let (assign24600_body48_e19602, assign24600_body48_e19602_d_n0, assign24600_body48_e19602_d_n2, assign24600_body48_e19602_d_n4, assign24600_body48_e19602_d_n5, assign24600_body48_e19602_d_n6, assign24600_body48_e19602_d_n7, assign24600_body48_e19602_d_n8, assign24600_body48_e19602_d_n9, assign24600_body48_e19602_d_n10, assign24600_body48_e19602_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard559 != 0.0)) && (locals.var_guard566 != 0.0)) {
        let assign24600_body48_e19600: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign24600_body48_e19600, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn13 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn13)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
            locals.var_xmp = assign24600_body48_e19602;
            locals.var_xmp_dn0 = assign24600_body48_e19602_d_n0;
            locals.var_xmp_dn2 = assign24600_body48_e19602_d_n2;
            locals.var_xmp_dn4 = assign24600_body48_e19602_d_n4;
            locals.var_xmp_dn5 = assign24600_body48_e19602_d_n5;
            locals.var_xmp_dn6 = assign24600_body48_e19602_d_n6;
            locals.var_xmp_dn7 = assign24600_body48_e19602_d_n7;
            locals.var_xmp_dn8 = assign24600_body48_e19602_d_n8;
            locals.var_xmp_dn9 = assign24600_body48_e19602_d_n9;
            locals.var_xmp_dn10 = assign24600_body48_e19602_d_n10;
            locals.var_xmp_dn13 = assign24600_body48_e19602_d_n13;
            locals.var_xmp_rv = 0.0;
            let (assign24600_body49_e19614, assign24600_body49_e19614_d_n0, assign24600_body49_e19614_d_n2, assign24600_body49_e19614_d_n4, assign24600_body49_e19614_d_n5, assign24600_body49_e19614_d_n6, assign24600_body49_e19614_d_n7, assign24600_body49_e19614_d_n8, assign24600_body49_e19614_d_n9, assign24600_body49_e19614_d_n10, assign24600_body49_e19614_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard559 != 0.0)) && (locals.var_guard566 != 0.0)) {
        let assign24600_body49_e19612: f64 = (locals.var_xp * locals.var_x2);
        (assign24600_body49_e19612, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn13 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn13)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
            locals.var_xp = assign24600_body49_e19614;
            locals.var_xp_dn0 = assign24600_body49_e19614_d_n0;
            locals.var_xp_dn2 = assign24600_body49_e19614_d_n2;
            locals.var_xp_dn4 = assign24600_body49_e19614_d_n4;
            locals.var_xp_dn5 = assign24600_body49_e19614_d_n5;
            locals.var_xp_dn6 = assign24600_body49_e19614_d_n6;
            locals.var_xp_dn7 = assign24600_body49_e19614_d_n7;
            locals.var_xp_dn8 = assign24600_body49_e19614_d_n8;
            locals.var_xp_dn9 = assign24600_body49_e19614_d_n9;
            locals.var_xp_dn10 = assign24600_body49_e19614_d_n10;
            locals.var_xp_dn13 = assign24600_body49_e19614_d_n13;
            locals.var_xp_rv = 0.0;
            let (assign24600_body50_e19626, assign24600_body50_e19626_d_n0, assign24600_body50_e19626_d_n2, assign24600_body50_e19626_d_n4, assign24600_body50_e19626_d_n5, assign24600_body50_e19626_d_n6, assign24600_body50_e19626_d_n7, assign24600_body50_e19626_d_n8, assign24600_body50_e19626_d_n9, assign24600_body50_e19626_d_n10, assign24600_body50_e19626_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard559 != 0.0)) && (locals.var_guard566 != 0.0)) {
        let assign24600_body50_e19624: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign24600_body50_e19624, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn13 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn13)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
            locals.var_xmp = assign24600_body50_e19626;
            locals.var_xmp_dn0 = assign24600_body50_e19626_d_n0;
            locals.var_xmp_dn2 = assign24600_body50_e19626_d_n2;
            locals.var_xmp_dn4 = assign24600_body50_e19626_d_n4;
            locals.var_xmp_dn5 = assign24600_body50_e19626_d_n5;
            locals.var_xmp_dn6 = assign24600_body50_e19626_d_n6;
            locals.var_xmp_dn7 = assign24600_body50_e19626_d_n7;
            locals.var_xmp_dn8 = assign24600_body50_e19626_d_n8;
            locals.var_xmp_dn9 = assign24600_body50_e19626_d_n9;
            locals.var_xmp_dn10 = assign24600_body50_e19626_d_n10;
            locals.var_xmp_dn13 = assign24600_body50_e19626_d_n13;
            locals.var_xmp_rv = 0.0;
            let (assign24600_body51_e19638, assign24600_body51_e19638_d_n0, assign24600_body51_e19638_d_n2, assign24600_body51_e19638_d_n4, assign24600_body51_e19638_d_n5, assign24600_body51_e19638_d_n6, assign24600_body51_e19638_d_n7, assign24600_body51_e19638_d_n8, assign24600_body51_e19638_d_n9, assign24600_body51_e19638_d_n10, assign24600_body51_e19638_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard559 != 0.0)) && (locals.var_guard566 != 0.0)) {
        let assign24600_body51_e19636: f64 = (locals.var_xp + locals.var_xmp);
        (assign24600_body51_e19636, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn4 + locals.var_xmp_dn4), (locals.var_xp_dn5 + locals.var_xmp_dn5), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn8 + locals.var_xmp_dn8), (locals.var_xp_dn9 + locals.var_xmp_dn9), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn13 + locals.var_xmp_dn13),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    }
};
            locals.var_arg = assign24600_body51_e19638;
            locals.var_arg_dn0 = assign24600_body51_e19638_d_n0;
            locals.var_arg_dn2 = assign24600_body51_e19638_d_n2;
            locals.var_arg_dn4 = assign24600_body51_e19638_d_n4;
            locals.var_arg_dn5 = assign24600_body51_e19638_d_n5;
            locals.var_arg_dn6 = assign24600_body51_e19638_d_n6;
            locals.var_arg_dn7 = assign24600_body51_e19638_d_n7;
            locals.var_arg_dn8 = assign24600_body51_e19638_d_n8;
            locals.var_arg_dn9 = assign24600_body51_e19638_d_n9;
            locals.var_arg_dn10 = assign24600_body51_e19638_d_n10;
            locals.var_arg_dn13 = assign24600_body51_e19638_d_n13;
            locals.var_arg_rv = 0.0;
            let (assign24600_body52_e19648, assign24600_body52_e19648_d_n0, assign24600_body52_e19648_d_n2, assign24600_body52_e19648_d_n4, assign24600_body52_e19648_d_n5, assign24600_body52_e19648_d_n6, assign24600_body52_e19648_d_n7, assign24600_body52_e19648_d_n8, assign24600_body52_e19648_d_n9, assign24600_body52_e19648_d_n10, assign24600_body52_e19648_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard559 != 0.0)) && (locals.var_guard566 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
            locals.var_dnm = assign24600_body52_e19648;
            locals.var_dnm_dn0 = assign24600_body52_e19648_d_n0;
            locals.var_dnm_dn2 = assign24600_body52_e19648_d_n2;
            locals.var_dnm_dn4 = assign24600_body52_e19648_d_n4;
            locals.var_dnm_dn5 = assign24600_body52_e19648_d_n5;
            locals.var_dnm_dn6 = assign24600_body52_e19648_d_n6;
            locals.var_dnm_dn7 = assign24600_body52_e19648_d_n7;
            locals.var_dnm_dn8 = assign24600_body52_e19648_d_n8;
            locals.var_dnm_dn9 = assign24600_body52_e19648_d_n9;
            locals.var_dnm_dn10 = assign24600_body52_e19648_d_n10;
            locals.var_dnm_dn13 = assign24600_body52_e19648_d_n13;
            locals.var_dnm_rv = 0.0;
            let assign24600_body53_e19663: f64 = if ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0)) { 1.0 } else { 0.0 };
            locals.var_guard567 = assign24600_body53_e19663;
            locals.var_guard567_rv = 0.0;
            let assign24600_body54_e19666: f64 = if 2.0 == 1.0 { 1.0 } else { 0.0 };
            locals.var_guard568 = assign24600_body54_e19666;
            locals.var_guard568_rv = 0.0;
            let (assign24600_body55_e19680,) = {
    if ((((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard559 != 0.0)) && (locals.var_guard566 != 0.0)) && (locals.var_guard567 != 0.0)) && (locals.var_guard568 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
            locals.var_mm = assign24600_body55_e19680;
            locals.var_mm_rv = 0.0;
            let assign24600_body56_e19683: f64 = if 2.0 == 2.0 { 1.0 } else { 0.0 };
            locals.var_guard569 = assign24600_body56_e19683;
            locals.var_guard569_rv = 0.0;
            let (assign24600_body57_e19700,) = {
    if (((((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard559 != 0.0)) && (locals.var_guard566 != 0.0)) && (locals.var_guard567 != 0.0)) && (locals.var_guard568 == 0.0)) && (locals.var_guard569 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
            locals.var_mm = assign24600_body57_e19700;
            locals.var_mm_rv = 0.0;
            let assign24600_body58_e19703: f64 = if 2.0 == 4.0 { 1.0 } else { 0.0 };
            locals.var_guard570 = assign24600_body58_e19703;
            locals.var_guard570_rv = 0.0;
            let (assign24600_body59_e19723,) = {
    if ((((((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard559 != 0.0)) && (locals.var_guard566 != 0.0)) && (locals.var_guard567 != 0.0)) && (locals.var_guard568 == 0.0)) && (locals.var_guard569 == 0.0)) && (locals.var_guard570 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
            locals.var_mm = assign24600_body59_e19723;
            locals.var_mm_rv = 0.0;
            let assign24600_body60_e19726: f64 = if 2.0 == 8.0 { 1.0 } else { 0.0 };
            locals.var_guard571 = assign24600_body60_e19726;
            locals.var_guard571_rv = 0.0;
            let (assign24600_body61_e19749,) = {
    if (((((((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard559 != 0.0)) && (locals.var_guard566 != 0.0)) && (locals.var_guard567 != 0.0)) && (locals.var_guard568 == 0.0)) && (locals.var_guard569 == 0.0)) && (locals.var_guard570 == 0.0)) && (locals.var_guard571 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
            locals.var_mm = assign24600_body61_e19749;
            locals.var_mm_rv = 0.0;
            let (assign24600_body62_e19761,) = {
    if (((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard559 != 0.0)) && (locals.var_guard566 != 0.0)) && (locals.var_guard567 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign24600_body62_e19761;
            locals.var_m0_rv = 0.0;
            let mut assign24600_body63_loop_guard: usize = 0;
            while {
                let assign24600_body63_cond_e19774: f64 = if ((((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard559 != 0.0)) && (locals.var_guard566 != 0.0)) && (locals.var_guard567 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
                assign24600_body63_cond_e19774 != 0.0
            } {
                assign24600_body63_loop_guard += 1;
                assert!(assign24600_body63_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
                let (assign24600_body63_body0_e19787, assign24600_body63_body0_e19787_d_n0, assign24600_body63_body0_e19787_d_n2, assign24600_body63_body0_e19787_d_n4, assign24600_body63_body0_e19787_d_n5, assign24600_body63_body0_e19787_d_n6, assign24600_body63_body0_e19787_d_n7, assign24600_body63_body0_e19787_d_n8, assign24600_body63_body0_e19787_d_n9, assign24600_body63_body0_e19787_d_n10, assign24600_body63_body0_e19787_d_n13,) = {
    if (((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard559 != 0.0)) && (locals.var_guard566 != 0.0)) && (locals.var_guard567 != 0.0)) {
        let assign24600_body63_body0_e19785: f64 = (locals.var_dnm).sqrt();
        (assign24600_body63_body0_e19785, (locals.var_dnm_dn0 / (2.0 * assign24600_body63_body0_e19785)), (locals.var_dnm_dn2 / (2.0 * assign24600_body63_body0_e19785)), (locals.var_dnm_dn4 / (2.0 * assign24600_body63_body0_e19785)), (locals.var_dnm_dn5 / (2.0 * assign24600_body63_body0_e19785)), (locals.var_dnm_dn6 / (2.0 * assign24600_body63_body0_e19785)), (locals.var_dnm_dn7 / (2.0 * assign24600_body63_body0_e19785)), (locals.var_dnm_dn8 / (2.0 * assign24600_body63_body0_e19785)), (locals.var_dnm_dn9 / (2.0 * assign24600_body63_body0_e19785)), (locals.var_dnm_dn10 / (2.0 * assign24600_body63_body0_e19785)), (locals.var_dnm_dn13 / (2.0 * assign24600_body63_body0_e19785)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
                locals.var_dnm = assign24600_body63_body0_e19787;
                locals.var_dnm_dn0 = assign24600_body63_body0_e19787_d_n0;
                locals.var_dnm_dn2 = assign24600_body63_body0_e19787_d_n2;
                locals.var_dnm_dn4 = assign24600_body63_body0_e19787_d_n4;
                locals.var_dnm_dn5 = assign24600_body63_body0_e19787_d_n5;
                locals.var_dnm_dn6 = assign24600_body63_body0_e19787_d_n6;
                locals.var_dnm_dn7 = assign24600_body63_body0_e19787_d_n7;
                locals.var_dnm_dn8 = assign24600_body63_body0_e19787_d_n8;
                locals.var_dnm_dn9 = assign24600_body63_body0_e19787_d_n9;
                locals.var_dnm_dn10 = assign24600_body63_body0_e19787_d_n10;
                locals.var_dnm_dn13 = assign24600_body63_body0_e19787_d_n13;
                locals.var_dnm_rv = 0.0;
                let (assign24600_body63_body1_e19801,) = {
    if (((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard559 != 0.0)) && (locals.var_guard566 != 0.0)) && (locals.var_guard567 != 0.0)) {
        let assign24600_body63_body1_e19799: f64 = (locals.var_m0 + 1.0);
        (assign24600_body63_body1_e19799,)
    } else {
        (locals.var_m0,)
    }
};
                locals.var_m0 = assign24600_body63_body1_e19801;
                locals.var_m0_rv = 0.0;
            }
            let (assign24600_body64_e19825, assign24600_body64_e19825_d_n0, assign24600_body64_e19825_d_n2, assign24600_body64_e19825_d_n4, assign24600_body64_e19825_d_n5, assign24600_body64_e19825_d_n6, assign24600_body64_e19825_d_n7, assign24600_body64_e19825_d_n8, assign24600_body64_e19825_d_n9, assign24600_body64_e19825_d_n10, assign24600_body64_e19825_d_n13,) = {
    if (((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard559 != 0.0)) && (locals.var_guard566 != 0.0)) && (locals.var_guard567 == 0.0)) {
        let (assign24600_body64_e19823, assign24600_body64_e19823_d_n0, assign24600_body64_e19823_d_n2, assign24600_body64_e19823_d_n4, assign24600_body64_e19823_d_n5, assign24600_body64_e19823_d_n6, assign24600_body64_e19823_d_n7, assign24600_body64_e19823_d_n8, assign24600_body64_e19823_d_n9, assign24600_body64_e19823_d_n10, assign24600_body64_e19823_d_n13,) = {
            if (locals.var_dnm == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign24600_body64_e19820: f64 = (2.0 * 2.0);
                let assign24600_body64_e19821: f64 = (1.0 / assign24600_body64_e19820);
                let assign24600_body64_e19822: f64 = (locals.var_dnm).powf(assign24600_body64_e19821);
                (assign24600_body64_e19822, if 0.0 == 0.0 && ((assign24600_body64_e19821) as f64).is_finite() && ((assign24600_body64_e19821) as f64).fract() == 0.0 { if assign24600_body64_e19821 == 0.0 { 0.0 } else { (assign24600_body64_e19821 * ((locals.var_dnm).powf(assign24600_body64_e19821 - 1.0) * locals.var_dnm_dn0)) } } else { (assign24600_body64_e19822 * (assign24600_body64_e19821 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign24600_body64_e19821) as f64).is_finite() && ((assign24600_body64_e19821) as f64).fract() == 0.0 { if assign24600_body64_e19821 == 0.0 { 0.0 } else { (assign24600_body64_e19821 * ((locals.var_dnm).powf(assign24600_body64_e19821 - 1.0) * locals.var_dnm_dn2)) } } else { (assign24600_body64_e19822 * (assign24600_body64_e19821 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign24600_body64_e19821) as f64).is_finite() && ((assign24600_body64_e19821) as f64).fract() == 0.0 { if assign24600_body64_e19821 == 0.0 { 0.0 } else { (assign24600_body64_e19821 * ((locals.var_dnm).powf(assign24600_body64_e19821 - 1.0) * locals.var_dnm_dn4)) } } else { (assign24600_body64_e19822 * (assign24600_body64_e19821 * (locals.var_dnm_dn4 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign24600_body64_e19821) as f64).is_finite() && ((assign24600_body64_e19821) as f64).fract() == 0.0 { if assign24600_body64_e19821 == 0.0 { 0.0 } else { (assign24600_body64_e19821 * ((locals.var_dnm).powf(assign24600_body64_e19821 - 1.0) * locals.var_dnm_dn5)) } } else { (assign24600_body64_e19822 * (assign24600_body64_e19821 * (locals.var_dnm_dn5 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign24600_body64_e19821) as f64).is_finite() && ((assign24600_body64_e19821) as f64).fract() == 0.0 { if assign24600_body64_e19821 == 0.0 { 0.0 } else { (assign24600_body64_e19821 * ((locals.var_dnm).powf(assign24600_body64_e19821 - 1.0) * locals.var_dnm_dn6)) } } else { (assign24600_body64_e19822 * (assign24600_body64_e19821 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign24600_body64_e19821) as f64).is_finite() && ((assign24600_body64_e19821) as f64).fract() == 0.0 { if assign24600_body64_e19821 == 0.0 { 0.0 } else { (assign24600_body64_e19821 * ((locals.var_dnm).powf(assign24600_body64_e19821 - 1.0) * locals.var_dnm_dn7)) } } else { (assign24600_body64_e19822 * (assign24600_body64_e19821 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign24600_body64_e19821) as f64).is_finite() && ((assign24600_body64_e19821) as f64).fract() == 0.0 { if assign24600_body64_e19821 == 0.0 { 0.0 } else { (assign24600_body64_e19821 * ((locals.var_dnm).powf(assign24600_body64_e19821 - 1.0) * locals.var_dnm_dn8)) } } else { (assign24600_body64_e19822 * (assign24600_body64_e19821 * (locals.var_dnm_dn8 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign24600_body64_e19821) as f64).is_finite() && ((assign24600_body64_e19821) as f64).fract() == 0.0 { if assign24600_body64_e19821 == 0.0 { 0.0 } else { (assign24600_body64_e19821 * ((locals.var_dnm).powf(assign24600_body64_e19821 - 1.0) * locals.var_dnm_dn9)) } } else { (assign24600_body64_e19822 * (assign24600_body64_e19821 * (locals.var_dnm_dn9 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign24600_body64_e19821) as f64).is_finite() && ((assign24600_body64_e19821) as f64).fract() == 0.0 { if assign24600_body64_e19821 == 0.0 { 0.0 } else { (assign24600_body64_e19821 * ((locals.var_dnm).powf(assign24600_body64_e19821 - 1.0) * locals.var_dnm_dn10)) } } else { (assign24600_body64_e19822 * (assign24600_body64_e19821 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign24600_body64_e19821) as f64).is_finite() && ((assign24600_body64_e19821) as f64).fract() == 0.0 { if assign24600_body64_e19821 == 0.0 { 0.0 } else { (assign24600_body64_e19821 * ((locals.var_dnm).powf(assign24600_body64_e19821 - 1.0) * locals.var_dnm_dn13)) } } else { (assign24600_body64_e19822 * (assign24600_body64_e19821 * (locals.var_dnm_dn13 / locals.var_dnm))) },)
            }
        };
        (assign24600_body64_e19823, assign24600_body64_e19823_d_n0, assign24600_body64_e19823_d_n2, assign24600_body64_e19823_d_n4, assign24600_body64_e19823_d_n5, assign24600_body64_e19823_d_n6, assign24600_body64_e19823_d_n7, assign24600_body64_e19823_d_n8, assign24600_body64_e19823_d_n9, assign24600_body64_e19823_d_n10, assign24600_body64_e19823_d_n13,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
            locals.var_dnm = assign24600_body64_e19825;
            locals.var_dnm_dn0 = assign24600_body64_e19825_d_n0;
            locals.var_dnm_dn2 = assign24600_body64_e19825_d_n2;
            locals.var_dnm_dn4 = assign24600_body64_e19825_d_n4;
            locals.var_dnm_dn5 = assign24600_body64_e19825_d_n5;
            locals.var_dnm_dn6 = assign24600_body64_e19825_d_n6;
            locals.var_dnm_dn7 = assign24600_body64_e19825_d_n7;
            locals.var_dnm_dn8 = assign24600_body64_e19825_d_n8;
            locals.var_dnm_dn9 = assign24600_body64_e19825_d_n9;
            locals.var_dnm_dn10 = assign24600_body64_e19825_d_n10;
            locals.var_dnm_dn13 = assign24600_body64_e19825_d_n13;
            locals.var_dnm_rv = 0.0;
            let (assign24600_body65_e19837, assign24600_body65_e19837_d_n0, assign24600_body65_e19837_d_n2, assign24600_body65_e19837_d_n4, assign24600_body65_e19837_d_n5, assign24600_body65_e19837_d_n6, assign24600_body65_e19837_d_n7, assign24600_body65_e19837_d_n8, assign24600_body65_e19837_d_n9, assign24600_body65_e19837_d_n10, assign24600_body65_e19837_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard559 != 0.0)) && (locals.var_guard566 != 0.0)) {
        let assign24600_body65_e19835: f64 = (1.0 / locals.var_dnm);
        (assign24600_body65_e19835, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn4 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn5 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn8 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn9 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn13 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
            locals.var_dnm = assign24600_body65_e19837;
            locals.var_dnm_dn0 = assign24600_body65_e19837_d_n0;
            locals.var_dnm_dn2 = assign24600_body65_e19837_d_n2;
            locals.var_dnm_dn4 = assign24600_body65_e19837_d_n4;
            locals.var_dnm_dn5 = assign24600_body65_e19837_d_n5;
            locals.var_dnm_dn6 = assign24600_body65_e19837_d_n6;
            locals.var_dnm_dn7 = assign24600_body65_e19837_d_n7;
            locals.var_dnm_dn8 = assign24600_body65_e19837_d_n8;
            locals.var_dnm_dn9 = assign24600_body65_e19837_d_n9;
            locals.var_dnm_dn10 = assign24600_body65_e19837_d_n10;
            locals.var_dnm_dn13 = assign24600_body65_e19837_d_n13;
            locals.var_dnm_rv = 0.0;
            let (assign24600_body66_e19851, assign24600_body66_e19851_d_n0, assign24600_body66_e19851_d_n2, assign24600_body66_e19851_d_n4, assign24600_body66_e19851_d_n5, assign24600_body66_e19851_d_n6, assign24600_body66_e19851_d_n7, assign24600_body66_e19851_d_n8, assign24600_body66_e19851_d_n9, assign24600_body66_e19851_d_n10, assign24600_body66_e19851_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard559 != 0.0)) && (locals.var_guard566 != 0.0)) {
        let assign24600_body66_e19847: f64 = (locals.var_tmf1 * 0.1);
        let assign24600_body66_e19849: f64 = (assign24600_body66_e19847 * locals.var_dnm);
        (assign24600_body66_e19849, (((locals.var_tmf1_dn0 * 0.1) * locals.var_dnm) + (assign24600_body66_e19847 * locals.var_dnm_dn0)), (((locals.var_tmf1_dn2 * 0.1) * locals.var_dnm) + (assign24600_body66_e19847 * locals.var_dnm_dn2)), (((locals.var_tmf1_dn4 * 0.1) * locals.var_dnm) + (assign24600_body66_e19847 * locals.var_dnm_dn4)), (((locals.var_tmf1_dn5 * 0.1) * locals.var_dnm) + (assign24600_body66_e19847 * locals.var_dnm_dn5)), (((locals.var_tmf1_dn6 * 0.1) * locals.var_dnm) + (assign24600_body66_e19847 * locals.var_dnm_dn6)), (((locals.var_tmf1_dn7 * 0.1) * locals.var_dnm) + (assign24600_body66_e19847 * locals.var_dnm_dn7)), (((locals.var_tmf1_dn8 * 0.1) * locals.var_dnm) + (assign24600_body66_e19847 * locals.var_dnm_dn8)), (((locals.var_tmf1_dn9 * 0.1) * locals.var_dnm) + (assign24600_body66_e19847 * locals.var_dnm_dn9)), (((locals.var_tmf1_dn10 * 0.1) * locals.var_dnm) + (assign24600_body66_e19847 * locals.var_dnm_dn10)), (((locals.var_tmf1_dn13 * 0.1) * locals.var_dnm) + (assign24600_body66_e19847 * locals.var_dnm_dn13)),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn13,)
    }
};
            locals.var_tmf0 = assign24600_body66_e19851;
            locals.var_tmf0_dn0 = assign24600_body66_e19851_d_n0;
            locals.var_tmf0_dn2 = assign24600_body66_e19851_d_n2;
            locals.var_tmf0_dn4 = assign24600_body66_e19851_d_n4;
            locals.var_tmf0_dn5 = assign24600_body66_e19851_d_n5;
            locals.var_tmf0_dn6 = assign24600_body66_e19851_d_n6;
            locals.var_tmf0_dn7 = assign24600_body66_e19851_d_n7;
            locals.var_tmf0_dn8 = assign24600_body66_e19851_d_n8;
            locals.var_tmf0_dn9 = assign24600_body66_e19851_d_n9;
            locals.var_tmf0_dn10 = assign24600_body66_e19851_d_n10;
            locals.var_tmf0_dn13 = assign24600_body66_e19851_d_n13;
            locals.var_tmf0_rv = 0.0;
            let (assign24600_body67_e19867, assign24600_body67_e19867_d_n0, assign24600_body67_e19867_d_n2, assign24600_body67_e19867_d_n4, assign24600_body67_e19867_d_n5, assign24600_body67_e19867_d_n6, assign24600_body67_e19867_d_n7, assign24600_body67_e19867_d_n8, assign24600_body67_e19867_d_n9, assign24600_body67_e19867_d_n10, assign24600_body67_e19867_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard559 != 0.0)) && (locals.var_guard566 != 0.0)) {
        let assign24600_body67_e19861: f64 = (0.1 * locals.var_xmp);
        let assign24600_body67_e19863: f64 = (assign24600_body67_e19861 * locals.var_dnm);
        let assign24600_body67_e19865: f64 = (assign24600_body67_e19863 / locals.var_arg);
        (assign24600_body67_e19865, ((((((0.1 * locals.var_xmp_dn0) * locals.var_dnm) + (assign24600_body67_e19861 * locals.var_dnm_dn0)) * locals.var_arg) - (assign24600_body67_e19863 * locals.var_arg_dn0)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn2) * locals.var_dnm) + (assign24600_body67_e19861 * locals.var_dnm_dn2)) * locals.var_arg) - (assign24600_body67_e19863 * locals.var_arg_dn2)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn4) * locals.var_dnm) + (assign24600_body67_e19861 * locals.var_dnm_dn4)) * locals.var_arg) - (assign24600_body67_e19863 * locals.var_arg_dn4)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn5) * locals.var_dnm) + (assign24600_body67_e19861 * locals.var_dnm_dn5)) * locals.var_arg) - (assign24600_body67_e19863 * locals.var_arg_dn5)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn6) * locals.var_dnm) + (assign24600_body67_e19861 * locals.var_dnm_dn6)) * locals.var_arg) - (assign24600_body67_e19863 * locals.var_arg_dn6)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn7) * locals.var_dnm) + (assign24600_body67_e19861 * locals.var_dnm_dn7)) * locals.var_arg) - (assign24600_body67_e19863 * locals.var_arg_dn7)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn8) * locals.var_dnm) + (assign24600_body67_e19861 * locals.var_dnm_dn8)) * locals.var_arg) - (assign24600_body67_e19863 * locals.var_arg_dn8)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn9) * locals.var_dnm) + (assign24600_body67_e19861 * locals.var_dnm_dn9)) * locals.var_arg) - (assign24600_body67_e19863 * locals.var_arg_dn9)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn10) * locals.var_dnm) + (assign24600_body67_e19861 * locals.var_dnm_dn10)) * locals.var_arg) - (assign24600_body67_e19863 * locals.var_arg_dn10)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn13) * locals.var_dnm) + (assign24600_body67_e19861 * locals.var_dnm_dn13)) * locals.var_arg) - (assign24600_body67_e19863 * locals.var_arg_dn13)) / (locals.var_arg * locals.var_arg)),)
    } else {
        (locals.var_t7, locals.var_t7_dn0, locals.var_t7_dn2, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn13,)
    }
};
            locals.var_t7 = assign24600_body67_e19867;
            locals.var_t7_dn0 = assign24600_body67_e19867_d_n0;
            locals.var_t7_dn2 = assign24600_body67_e19867_d_n2;
            locals.var_t7_dn4 = assign24600_body67_e19867_d_n4;
            locals.var_t7_dn5 = assign24600_body67_e19867_d_n5;
            locals.var_t7_dn6 = assign24600_body67_e19867_d_n6;
            locals.var_t7_dn7 = assign24600_body67_e19867_d_n7;
            locals.var_t7_dn8 = assign24600_body67_e19867_d_n8;
            locals.var_t7_dn9 = assign24600_body67_e19867_d_n9;
            locals.var_t7_dn10 = assign24600_body67_e19867_d_n10;
            locals.var_t7_dn13 = assign24600_body67_e19867_d_n13;
            locals.var_t7_rv = 0.0;
            let (assign24600_body68_e19881, assign24600_body68_e19881_d_n0, assign24600_body68_e19881_d_n2, assign24600_body68_e19881_d_n4, assign24600_body68_e19881_d_n5, assign24600_body68_e19881_d_n6, assign24600_body68_e19881_d_n7, assign24600_body68_e19881_d_n8, assign24600_body68_e19881_d_n9, assign24600_body68_e19881_d_n10, assign24600_body68_e19881_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard559 != 0.0)) && (locals.var_guard566 != 0.0)) {
        let assign24600_body68_e19877: f64 = 0.1;
        let assign24600_body68_e19879: f64 = (assign24600_body68_e19877 - locals.var_tmf0);
        (assign24600_body68_e19879, (-locals.var_tmf0_dn0), (-locals.var_tmf0_dn2), (-locals.var_tmf0_dn4), (-locals.var_tmf0_dn5), (-locals.var_tmf0_dn6), (-locals.var_tmf0_dn7), (-locals.var_tmf0_dn8), (-locals.var_tmf0_dn9), (-locals.var_tmf0_dn10), (-locals.var_tmf0_dn13),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
            locals.var_t2 = assign24600_body68_e19881;
            locals.var_t2_dn0 = assign24600_body68_e19881_d_n0;
            locals.var_t2_dn2 = assign24600_body68_e19881_d_n2;
            locals.var_t2_dn4 = assign24600_body68_e19881_d_n4;
            locals.var_t2_dn5 = assign24600_body68_e19881_d_n5;
            locals.var_t2_dn6 = assign24600_body68_e19881_d_n6;
            locals.var_t2_dn7 = assign24600_body68_e19881_d_n7;
            locals.var_t2_dn8 = assign24600_body68_e19881_d_n8;
            locals.var_t2_dn9 = assign24600_body68_e19881_d_n9;
            locals.var_t2_dn10 = assign24600_body68_e19881_d_n10;
            locals.var_t2_dn13 = assign24600_body68_e19881_d_n13;
            locals.var_t2_rv = 0.0;
            let (assign24600_body69_e19891, assign24600_body69_e19891_d_n0, assign24600_body69_e19891_d_n2, assign24600_body69_e19891_d_n4, assign24600_body69_e19891_d_n5, assign24600_body69_e19891_d_n6, assign24600_body69_e19891_d_n7, assign24600_body69_e19891_d_n8, assign24600_body69_e19891_d_n9, assign24600_body69_e19891_d_n10, assign24600_body69_e19891_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard559 != 0.0)) && (locals.var_guard566 != 0.0)) {
        (locals.var_t7, locals.var_t7_dn0, locals.var_t7_dn2, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn13,)
    } else {
        (locals.var_t7, locals.var_t7_dn0, locals.var_t7_dn2, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn13,)
    }
};
            locals.var_t7 = assign24600_body69_e19891;
            locals.var_t7_dn0 = assign24600_body69_e19891_d_n0;
            locals.var_t7_dn2 = assign24600_body69_e19891_d_n2;
            locals.var_t7_dn4 = assign24600_body69_e19891_d_n4;
            locals.var_t7_dn5 = assign24600_body69_e19891_d_n5;
            locals.var_t7_dn6 = assign24600_body69_e19891_d_n6;
            locals.var_t7_dn7 = assign24600_body69_e19891_d_n7;
            locals.var_t7_dn8 = assign24600_body69_e19891_d_n8;
            locals.var_t7_dn9 = assign24600_body69_e19891_d_n9;
            locals.var_t7_dn10 = assign24600_body69_e19891_d_n10;
            locals.var_t7_dn13 = assign24600_body69_e19891_d_n13;
            locals.var_t7_rv = 0.0;
            let (assign24600_body70_e19902, assign24600_body70_e19902_d_n0, assign24600_body70_e19902_d_n2, assign24600_body70_e19902_d_n4, assign24600_body70_e19902_d_n5, assign24600_body70_e19902_d_n6, assign24600_body70_e19902_d_n7, assign24600_body70_e19902_d_n8, assign24600_body70_e19902_d_n9, assign24600_body70_e19902_d_n10, assign24600_body70_e19902_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard559 != 0.0)) && (locals.var_guard566 == 0.0)) {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
            locals.var_t2 = assign24600_body70_e19902;
            locals.var_t2_dn0 = assign24600_body70_e19902_d_n0;
            locals.var_t2_dn2 = assign24600_body70_e19902_d_n2;
            locals.var_t2_dn4 = assign24600_body70_e19902_d_n4;
            locals.var_t2_dn5 = assign24600_body70_e19902_d_n5;
            locals.var_t2_dn6 = assign24600_body70_e19902_d_n6;
            locals.var_t2_dn7 = assign24600_body70_e19902_d_n7;
            locals.var_t2_dn8 = assign24600_body70_e19902_d_n8;
            locals.var_t2_dn9 = assign24600_body70_e19902_d_n9;
            locals.var_t2_dn10 = assign24600_body70_e19902_d_n10;
            locals.var_t2_dn13 = assign24600_body70_e19902_d_n13;
            locals.var_t2_rv = 0.0;
            let (assign24600_body71_e19913, assign24600_body71_e19913_d_n0, assign24600_body71_e19913_d_n2, assign24600_body71_e19913_d_n4, assign24600_body71_e19913_d_n5, assign24600_body71_e19913_d_n6, assign24600_body71_e19913_d_n7, assign24600_body71_e19913_d_n8, assign24600_body71_e19913_d_n9, assign24600_body71_e19913_d_n10, assign24600_body71_e19913_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard559 != 0.0)) && (locals.var_guard566 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t7, locals.var_t7_dn0, locals.var_t7_dn2, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn13,)
    }
};
            locals.var_t7 = assign24600_body71_e19913;
            locals.var_t7_dn0 = assign24600_body71_e19913_d_n0;
            locals.var_t7_dn2 = assign24600_body71_e19913_d_n2;
            locals.var_t7_dn4 = assign24600_body71_e19913_d_n4;
            locals.var_t7_dn5 = assign24600_body71_e19913_d_n5;
            locals.var_t7_dn6 = assign24600_body71_e19913_d_n6;
            locals.var_t7_dn7 = assign24600_body71_e19913_d_n7;
            locals.var_t7_dn8 = assign24600_body71_e19913_d_n8;
            locals.var_t7_dn9 = assign24600_body71_e19913_d_n9;
            locals.var_t7_dn10 = assign24600_body71_e19913_d_n10;
            locals.var_t7_dn13 = assign24600_body71_e19913_d_n13;
            locals.var_t7_rv = 0.0;
            let (assign24600_body72_e19924, assign24600_body72_e19924_d_n0, assign24600_body72_e19924_d_n2, assign24600_body72_e19924_d_n4, assign24600_body72_e19924_d_n5, assign24600_body72_e19924_d_n6, assign24600_body72_e19924_d_n7, assign24600_body72_e19924_d_n8, assign24600_body72_e19924_d_n9, assign24600_body72_e19924_d_n10, assign24600_body72_e19924_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard559 != 0.0)) {
        let assign24600_body72_e19921: f64 = (locals.var_c_2esipq_nsub * locals.var_t2);
        let assign24600_body72_e19922: f64 = (assign24600_body72_e19921).sqrt();
        (assign24600_body72_e19922, (((locals.var_c_2esipq_nsub_dn0 * locals.var_t2) + (locals.var_c_2esipq_nsub * locals.var_t2_dn0)) / (2.0 * assign24600_body72_e19922)), (((locals.var_c_2esipq_nsub_dn2 * locals.var_t2) + (locals.var_c_2esipq_nsub * locals.var_t2_dn2)) / (2.0 * assign24600_body72_e19922)), (((locals.var_c_2esipq_nsub_dn4 * locals.var_t2) + (locals.var_c_2esipq_nsub * locals.var_t2_dn4)) / (2.0 * assign24600_body72_e19922)), (((locals.var_c_2esipq_nsub_dn5 * locals.var_t2) + (locals.var_c_2esipq_nsub * locals.var_t2_dn5)) / (2.0 * assign24600_body72_e19922)), (((locals.var_c_2esipq_nsub_dn6 * locals.var_t2) + (locals.var_c_2esipq_nsub * locals.var_t2_dn6)) / (2.0 * assign24600_body72_e19922)), (((locals.var_c_2esipq_nsub_dn7 * locals.var_t2) + (locals.var_c_2esipq_nsub * locals.var_t2_dn7)) / (2.0 * assign24600_body72_e19922)), (((locals.var_c_2esipq_nsub_dn8 * locals.var_t2) + (locals.var_c_2esipq_nsub * locals.var_t2_dn8)) / (2.0 * assign24600_body72_e19922)), (((locals.var_c_2esipq_nsub_dn9 * locals.var_t2) + (locals.var_c_2esipq_nsub * locals.var_t2_dn9)) / (2.0 * assign24600_body72_e19922)), (((locals.var_c_2esipq_nsub_dn10 * locals.var_t2) + (locals.var_c_2esipq_nsub * locals.var_t2_dn10)) / (2.0 * assign24600_body72_e19922)), (((locals.var_c_2esipq_nsub_dn13 * locals.var_t2) + (locals.var_c_2esipq_nsub * locals.var_t2_dn13)) / (2.0 * assign24600_body72_e19922)),)
    } else {
        (locals.var_w_sub0, locals.var_w_sub0_dn0, locals.var_w_sub0_dn2, locals.var_w_sub0_dn4, locals.var_w_sub0_dn5, locals.var_w_sub0_dn6, locals.var_w_sub0_dn7, locals.var_w_sub0_dn8, locals.var_w_sub0_dn9, locals.var_w_sub0_dn10, locals.var_w_sub0_dn13,)
    }
};
            locals.var_w_sub0 = assign24600_body72_e19924;
            locals.var_w_sub0_dn0 = assign24600_body72_e19924_d_n0;
            locals.var_w_sub0_dn2 = assign24600_body72_e19924_d_n2;
            locals.var_w_sub0_dn4 = assign24600_body72_e19924_d_n4;
            locals.var_w_sub0_dn5 = assign24600_body72_e19924_d_n5;
            locals.var_w_sub0_dn6 = assign24600_body72_e19924_d_n6;
            locals.var_w_sub0_dn7 = assign24600_body72_e19924_d_n7;
            locals.var_w_sub0_dn8 = assign24600_body72_e19924_d_n8;
            locals.var_w_sub0_dn9 = assign24600_body72_e19924_d_n9;
            locals.var_w_sub0_dn10 = assign24600_body72_e19924_d_n10;
            locals.var_w_sub0_dn13 = assign24600_body72_e19924_d_n13;
            locals.var_w_sub0_rv = 0.0;
            let (assign24600_body73_e19934, assign24600_body73_e19934_d_n0, assign24600_body73_e19934_d_n2, assign24600_body73_e19934_d_n4, assign24600_body73_e19934_d_n5, assign24600_body73_e19934_d_n6, assign24600_body73_e19934_d_n7, assign24600_body73_e19934_d_n8, assign24600_body73_e19934_d_n9, assign24600_body73_e19934_d_n10, assign24600_body73_e19934_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard559 != 0.0)) {
        let assign24600_body73_e19932: f64 = (locals.var_w_b0 * locals.var_q_ndepm);
        (assign24600_body73_e19932, ((locals.var_w_b0_dn0 * locals.var_q_ndepm) + (locals.var_w_b0 * locals.var_q_ndepm_dn0)), ((locals.var_w_b0_dn2 * locals.var_q_ndepm) + (locals.var_w_b0 * locals.var_q_ndepm_dn2)), ((locals.var_w_b0_dn4 * locals.var_q_ndepm) + (locals.var_w_b0 * locals.var_q_ndepm_dn4)), ((locals.var_w_b0_dn5 * locals.var_q_ndepm) + (locals.var_w_b0 * locals.var_q_ndepm_dn5)), ((locals.var_w_b0_dn6 * locals.var_q_ndepm) + (locals.var_w_b0 * locals.var_q_ndepm_dn6)), ((locals.var_w_b0_dn7 * locals.var_q_ndepm) + (locals.var_w_b0 * locals.var_q_ndepm_dn7)), ((locals.var_w_b0_dn8 * locals.var_q_ndepm) + (locals.var_w_b0 * locals.var_q_ndepm_dn8)), ((locals.var_w_b0_dn9 * locals.var_q_ndepm) + (locals.var_w_b0 * locals.var_q_ndepm_dn9)), ((locals.var_w_b0_dn10 * locals.var_q_ndepm) + (locals.var_w_b0 * locals.var_q_ndepm_dn10)), ((locals.var_w_b0_dn13 * locals.var_q_ndepm) + (locals.var_w_b0 * locals.var_q_ndepm_dn13)),)
    } else {
        (locals.var_q_b0_dep, locals.var_q_b0_dep_dn0, locals.var_q_b0_dep_dn2, locals.var_q_b0_dep_dn4, locals.var_q_b0_dep_dn5, locals.var_q_b0_dep_dn6, locals.var_q_b0_dep_dn7, locals.var_q_b0_dep_dn8, locals.var_q_b0_dep_dn9, locals.var_q_b0_dep_dn10, locals.var_q_b0_dep_dn13,)
    }
};
            locals.var_q_b0_dep = assign24600_body73_e19934;
            locals.var_q_b0_dep_dn0 = assign24600_body73_e19934_d_n0;
            locals.var_q_b0_dep_dn2 = assign24600_body73_e19934_d_n2;
            locals.var_q_b0_dep_dn4 = assign24600_body73_e19934_d_n4;
            locals.var_q_b0_dep_dn5 = assign24600_body73_e19934_d_n5;
            locals.var_q_b0_dep_dn6 = assign24600_body73_e19934_d_n6;
            locals.var_q_b0_dep_dn7 = assign24600_body73_e19934_d_n7;
            locals.var_q_b0_dep_dn8 = assign24600_body73_e19934_d_n8;
            locals.var_q_b0_dep_dn9 = assign24600_body73_e19934_d_n9;
            locals.var_q_b0_dep_dn10 = assign24600_body73_e19934_d_n10;
            locals.var_q_b0_dep_dn13 = assign24600_body73_e19934_d_n13;
            locals.var_q_b0_dep_rv = 0.0;
            let (assign24600_body74_e19947, assign24600_body74_e19947_d_n0, assign24600_body74_e19947_d_n2, assign24600_body74_e19947_d_n4, assign24600_body74_e19947_d_n5, assign24600_body74_e19947_d_n6, assign24600_body74_e19947_d_n7, assign24600_body74_e19947_d_n8, assign24600_body74_e19947_d_n9, assign24600_body74_e19947_d_n10, assign24600_body74_e19947_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard559 != 0.0)) {
        let assign24600_body74_e19941: f64 = (-1.034943e-10);
        let assign24600_body74_e19943: f64 = (assign24600_body74_e19941 / locals.var_w_b0);
        let assign24600_body74_e19945: f64 = (assign24600_body74_e19943 * locals.var_t0);
        (assign24600_body74_e19945, (((-((assign24600_body74_e19941 * locals.var_w_b0_dn0) / (locals.var_w_b0 * locals.var_w_b0))) * locals.var_t0) + (assign24600_body74_e19943 * locals.var_t0_dn0)), (((-((assign24600_body74_e19941 * locals.var_w_b0_dn2) / (locals.var_w_b0 * locals.var_w_b0))) * locals.var_t0) + (assign24600_body74_e19943 * locals.var_t0_dn2)), (((-((assign24600_body74_e19941 * locals.var_w_b0_dn4) / (locals.var_w_b0 * locals.var_w_b0))) * locals.var_t0) + (assign24600_body74_e19943 * locals.var_t0_dn4)), (((-((assign24600_body74_e19941 * locals.var_w_b0_dn5) / (locals.var_w_b0 * locals.var_w_b0))) * locals.var_t0) + (assign24600_body74_e19943 * locals.var_t0_dn5)), (((-((assign24600_body74_e19941 * locals.var_w_b0_dn6) / (locals.var_w_b0 * locals.var_w_b0))) * locals.var_t0) + (assign24600_body74_e19943 * locals.var_t0_dn6)), (((-((assign24600_body74_e19941 * locals.var_w_b0_dn7) / (locals.var_w_b0 * locals.var_w_b0))) * locals.var_t0) + (assign24600_body74_e19943 * locals.var_t0_dn7)), (((-((assign24600_body74_e19941 * locals.var_w_b0_dn8) / (locals.var_w_b0 * locals.var_w_b0))) * locals.var_t0) + (assign24600_body74_e19943 * locals.var_t0_dn8)), (((-((assign24600_body74_e19941 * locals.var_w_b0_dn9) / (locals.var_w_b0 * locals.var_w_b0))) * locals.var_t0) + (assign24600_body74_e19943 * locals.var_t0_dn9)), (((-((assign24600_body74_e19941 * locals.var_w_b0_dn10) / (locals.var_w_b0 * locals.var_w_b0))) * locals.var_t0) + (assign24600_body74_e19943 * locals.var_t0_dn10)), (((-((assign24600_body74_e19941 * locals.var_w_b0_dn13) / (locals.var_w_b0 * locals.var_w_b0))) * locals.var_t0) + (assign24600_body74_e19943 * locals.var_t0_dn13)),)
    } else {
        (locals.var_q_b0_dep_dpd, locals.var_q_b0_dep_dpd_dn0, locals.var_q_b0_dep_dpd_dn2, locals.var_q_b0_dep_dpd_dn4, locals.var_q_b0_dep_dpd_dn5, locals.var_q_b0_dep_dpd_dn6, locals.var_q_b0_dep_dpd_dn7, locals.var_q_b0_dep_dpd_dn8, locals.var_q_b0_dep_dpd_dn9, locals.var_q_b0_dep_dpd_dn10, locals.var_q_b0_dep_dpd_dn13,)
    }
};
            locals.var_q_b0_dep_dpd = assign24600_body74_e19947;
            locals.var_q_b0_dep_dpd_dn0 = assign24600_body74_e19947_d_n0;
            locals.var_q_b0_dep_dpd_dn2 = assign24600_body74_e19947_d_n2;
            locals.var_q_b0_dep_dpd_dn4 = assign24600_body74_e19947_d_n4;
            locals.var_q_b0_dep_dpd_dn5 = assign24600_body74_e19947_d_n5;
            locals.var_q_b0_dep_dpd_dn6 = assign24600_body74_e19947_d_n6;
            locals.var_q_b0_dep_dpd_dn7 = assign24600_body74_e19947_d_n7;
            locals.var_q_b0_dep_dpd_dn8 = assign24600_body74_e19947_d_n8;
            locals.var_q_b0_dep_dpd_dn9 = assign24600_body74_e19947_d_n9;
            locals.var_q_b0_dep_dpd_dn10 = assign24600_body74_e19947_d_n10;
            locals.var_q_b0_dep_dpd_dn13 = assign24600_body74_e19947_d_n13;
            locals.var_q_b0_dep_dpd_rv = 0.0;
            let (assign24600_body75_e19958, assign24600_body75_e19958_d_n0, assign24600_body75_e19958_d_n2, assign24600_body75_e19958_d_n4, assign24600_body75_e19958_d_n5, assign24600_body75_e19958_d_n6, assign24600_body75_e19958_d_n7, assign24600_body75_e19958_d_n8, assign24600_body75_e19958_d_n9, assign24600_body75_e19958_d_n10, assign24600_body75_e19958_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard559 != 0.0)) {
        let assign24600_body75_e19954: f64 = (-locals.var_w_sub0);
        let assign24600_body75_e19956: f64 = (assign24600_body75_e19954 * locals.var_q_nsub__blk544);
        (assign24600_body75_e19956, (((-locals.var_w_sub0_dn0) * locals.var_q_nsub__blk544) + (assign24600_body75_e19954 * locals.var_q_nsub__blk544_dn0)), (((-locals.var_w_sub0_dn2) * locals.var_q_nsub__blk544) + (assign24600_body75_e19954 * locals.var_q_nsub__blk544_dn2)), (((-locals.var_w_sub0_dn4) * locals.var_q_nsub__blk544) + (assign24600_body75_e19954 * locals.var_q_nsub__blk544_dn4)), (((-locals.var_w_sub0_dn5) * locals.var_q_nsub__blk544) + (assign24600_body75_e19954 * locals.var_q_nsub__blk544_dn5)), (((-locals.var_w_sub0_dn6) * locals.var_q_nsub__blk544) + (assign24600_body75_e19954 * locals.var_q_nsub__blk544_dn6)), (((-locals.var_w_sub0_dn7) * locals.var_q_nsub__blk544) + (assign24600_body75_e19954 * locals.var_q_nsub__blk544_dn7)), (((-locals.var_w_sub0_dn8) * locals.var_q_nsub__blk544) + (assign24600_body75_e19954 * locals.var_q_nsub__blk544_dn8)), (((-locals.var_w_sub0_dn9) * locals.var_q_nsub__blk544) + (assign24600_body75_e19954 * locals.var_q_nsub__blk544_dn9)), (((-locals.var_w_sub0_dn10) * locals.var_q_nsub__blk544) + (assign24600_body75_e19954 * locals.var_q_nsub__blk544_dn10)), (((-locals.var_w_sub0_dn13) * locals.var_q_nsub__blk544) + (assign24600_body75_e19954 * locals.var_q_nsub__blk544_dn13)),)
    } else {
        (locals.var_q_sub0_dep, locals.var_q_sub0_dep_dn0, locals.var_q_sub0_dep_dn2, locals.var_q_sub0_dep_dn4, locals.var_q_sub0_dep_dn5, locals.var_q_sub0_dep_dn6, locals.var_q_sub0_dep_dn7, locals.var_q_sub0_dep_dn8, locals.var_q_sub0_dep_dn9, locals.var_q_sub0_dep_dn10, locals.var_q_sub0_dep_dn13,)
    }
};
            locals.var_q_sub0_dep = assign24600_body75_e19958;
            locals.var_q_sub0_dep_dn0 = assign24600_body75_e19958_d_n0;
            locals.var_q_sub0_dep_dn2 = assign24600_body75_e19958_d_n2;
            locals.var_q_sub0_dep_dn4 = assign24600_body75_e19958_d_n4;
            locals.var_q_sub0_dep_dn5 = assign24600_body75_e19958_d_n5;
            locals.var_q_sub0_dep_dn6 = assign24600_body75_e19958_d_n6;
            locals.var_q_sub0_dep_dn7 = assign24600_body75_e19958_d_n7;
            locals.var_q_sub0_dep_dn8 = assign24600_body75_e19958_d_n8;
            locals.var_q_sub0_dep_dn9 = assign24600_body75_e19958_d_n9;
            locals.var_q_sub0_dep_dn10 = assign24600_body75_e19958_d_n10;
            locals.var_q_sub0_dep_dn13 = assign24600_body75_e19958_d_n13;
            locals.var_q_sub0_dep_rv = 0.0;
            let (assign24600_body76_e19971, assign24600_body76_e19971_d_n0, assign24600_body76_e19971_d_n2, assign24600_body76_e19971_d_n4, assign24600_body76_e19971_d_n5, assign24600_body76_e19971_d_n6, assign24600_body76_e19971_d_n7, assign24600_body76_e19971_d_n8, assign24600_body76_e19971_d_n9, assign24600_body76_e19971_d_n10, assign24600_body76_e19971_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard559 != 0.0)) {
        let assign24600_body76_e19965: f64 = (-1.034943e-10);
        let assign24600_body76_e19967: f64 = (assign24600_body76_e19965 / locals.var_w_sub0);
        let assign24600_body76_e19969: f64 = (assign24600_body76_e19967 * locals.var_t7);
        (assign24600_body76_e19969, (((-((assign24600_body76_e19965 * locals.var_w_sub0_dn0) / (locals.var_w_sub0 * locals.var_w_sub0))) * locals.var_t7) + (assign24600_body76_e19967 * locals.var_t7_dn0)), (((-((assign24600_body76_e19965 * locals.var_w_sub0_dn2) / (locals.var_w_sub0 * locals.var_w_sub0))) * locals.var_t7) + (assign24600_body76_e19967 * locals.var_t7_dn2)), (((-((assign24600_body76_e19965 * locals.var_w_sub0_dn4) / (locals.var_w_sub0 * locals.var_w_sub0))) * locals.var_t7) + (assign24600_body76_e19967 * locals.var_t7_dn4)), (((-((assign24600_body76_e19965 * locals.var_w_sub0_dn5) / (locals.var_w_sub0 * locals.var_w_sub0))) * locals.var_t7) + (assign24600_body76_e19967 * locals.var_t7_dn5)), (((-((assign24600_body76_e19965 * locals.var_w_sub0_dn6) / (locals.var_w_sub0 * locals.var_w_sub0))) * locals.var_t7) + (assign24600_body76_e19967 * locals.var_t7_dn6)), (((-((assign24600_body76_e19965 * locals.var_w_sub0_dn7) / (locals.var_w_sub0 * locals.var_w_sub0))) * locals.var_t7) + (assign24600_body76_e19967 * locals.var_t7_dn7)), (((-((assign24600_body76_e19965 * locals.var_w_sub0_dn8) / (locals.var_w_sub0 * locals.var_w_sub0))) * locals.var_t7) + (assign24600_body76_e19967 * locals.var_t7_dn8)), (((-((assign24600_body76_e19965 * locals.var_w_sub0_dn9) / (locals.var_w_sub0 * locals.var_w_sub0))) * locals.var_t7) + (assign24600_body76_e19967 * locals.var_t7_dn9)), (((-((assign24600_body76_e19965 * locals.var_w_sub0_dn10) / (locals.var_w_sub0 * locals.var_w_sub0))) * locals.var_t7) + (assign24600_body76_e19967 * locals.var_t7_dn10)), (((-((assign24600_body76_e19965 * locals.var_w_sub0_dn13) / (locals.var_w_sub0 * locals.var_w_sub0))) * locals.var_t7) + (assign24600_body76_e19967 * locals.var_t7_dn13)),)
    } else {
        (locals.var_q_sub0_dep_dpd, locals.var_q_sub0_dep_dpd_dn0, locals.var_q_sub0_dep_dpd_dn2, locals.var_q_sub0_dep_dpd_dn4, locals.var_q_sub0_dep_dpd_dn5, locals.var_q_sub0_dep_dpd_dn6, locals.var_q_sub0_dep_dpd_dn7, locals.var_q_sub0_dep_dpd_dn8, locals.var_q_sub0_dep_dpd_dn9, locals.var_q_sub0_dep_dpd_dn10, locals.var_q_sub0_dep_dpd_dn13,)
    }
};
            locals.var_q_sub0_dep_dpd = assign24600_body76_e19971;
            locals.var_q_sub0_dep_dpd_dn0 = assign24600_body76_e19971_d_n0;
            locals.var_q_sub0_dep_dpd_dn2 = assign24600_body76_e19971_d_n2;
            locals.var_q_sub0_dep_dpd_dn4 = assign24600_body76_e19971_d_n4;
            locals.var_q_sub0_dep_dpd_dn5 = assign24600_body76_e19971_d_n5;
            locals.var_q_sub0_dep_dpd_dn6 = assign24600_body76_e19971_d_n6;
            locals.var_q_sub0_dep_dpd_dn7 = assign24600_body76_e19971_d_n7;
            locals.var_q_sub0_dep_dpd_dn8 = assign24600_body76_e19971_d_n8;
            locals.var_q_sub0_dep_dpd_dn9 = assign24600_body76_e19971_d_n9;
            locals.var_q_sub0_dep_dpd_dn10 = assign24600_body76_e19971_d_n10;
            locals.var_q_sub0_dep_dpd_dn13 = assign24600_body76_e19971_d_n13;
            locals.var_q_sub0_dep_dpd_rv = 0.0;
            let (assign24600_body77_e19987, assign24600_body77_e19987_d_n0, assign24600_body77_e19987_d_n2, assign24600_body77_e19987_d_n4, assign24600_body77_e19987_d_n5, assign24600_body77_e19987_d_n6, assign24600_body77_e19987_d_n7, assign24600_body77_e19987_d_n8, assign24600_body77_e19987_d_n9, assign24600_body77_e19987_d_n10, assign24600_body77_e19987_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard559 != 0.0)) {
        let assign24600_body77_e19980: f64 = (locals.var_vgp0 - locals.var_phi_b0_dep);
        let assign24600_body77_e19981: f64 = (locals.var_cox * assign24600_body77_e19980);
        let assign24600_body77_e19983: f64 = (assign24600_body77_e19981 + locals.var_q_b0_dep);
        let assign24600_body77_e19985: f64 = (assign24600_body77_e19983 + locals.var_q_sub0_dep);
        (assign24600_body77_e19985, ((((locals.var_cox_dn0 * assign24600_body77_e19980) + (locals.var_cox * (locals.var_vgp0_dn0 - locals.var_phi_b0_dep_dn0))) + locals.var_q_b0_dep_dn0) + locals.var_q_sub0_dep_dn0), ((((locals.var_cox_dn2 * assign24600_body77_e19980) + (locals.var_cox * (locals.var_vgp0_dn2 - locals.var_phi_b0_dep_dn2))) + locals.var_q_b0_dep_dn2) + locals.var_q_sub0_dep_dn2), ((((locals.var_cox_dn4 * assign24600_body77_e19980) + (locals.var_cox * (locals.var_vgp0_dn4 - locals.var_phi_b0_dep_dn4))) + locals.var_q_b0_dep_dn4) + locals.var_q_sub0_dep_dn4), ((((locals.var_cox_dn5 * assign24600_body77_e19980) + (locals.var_cox * (locals.var_vgp0_dn5 - locals.var_phi_b0_dep_dn5))) + locals.var_q_b0_dep_dn5) + locals.var_q_sub0_dep_dn5), ((((locals.var_cox_dn6 * assign24600_body77_e19980) + (locals.var_cox * (locals.var_vgp0_dn6 - locals.var_phi_b0_dep_dn6))) + locals.var_q_b0_dep_dn6) + locals.var_q_sub0_dep_dn6), ((((locals.var_cox_dn7 * assign24600_body77_e19980) + (locals.var_cox * (locals.var_vgp0_dn7 - locals.var_phi_b0_dep_dn7))) + locals.var_q_b0_dep_dn7) + locals.var_q_sub0_dep_dn7), ((((locals.var_cox_dn8 * assign24600_body77_e19980) + (locals.var_cox * (locals.var_vgp0_dn8 - locals.var_phi_b0_dep_dn8))) + locals.var_q_b0_dep_dn8) + locals.var_q_sub0_dep_dn8), ((((locals.var_cox_dn9 * assign24600_body77_e19980) + (locals.var_cox * (locals.var_vgp0_dn9 - locals.var_phi_b0_dep_dn9))) + locals.var_q_b0_dep_dn9) + locals.var_q_sub0_dep_dn9), ((((locals.var_cox_dn10 * assign24600_body77_e19980) + (locals.var_cox * (locals.var_vgp0_dn10 - locals.var_phi_b0_dep_dn10))) + locals.var_q_b0_dep_dn10) + locals.var_q_sub0_dep_dn10), ((((locals.var_cox_dn13 * assign24600_body77_e19980) + (locals.var_cox * (locals.var_vgp0_dn13 - locals.var_phi_b0_dep_dn13))) + locals.var_q_b0_dep_dn13) + locals.var_q_sub0_dep_dn13),)
    } else {
        (locals.var_y1, locals.var_y1_dn0, locals.var_y1_dn2, locals.var_y1_dn4, locals.var_y1_dn5, locals.var_y1_dn6, locals.var_y1_dn7, locals.var_y1_dn8, locals.var_y1_dn9, locals.var_y1_dn10, locals.var_y1_dn13,)
    }
};
            locals.var_y1 = assign24600_body77_e19987;
            locals.var_y1_dn0 = assign24600_body77_e19987_d_n0;
            locals.var_y1_dn2 = assign24600_body77_e19987_d_n2;
            locals.var_y1_dn4 = assign24600_body77_e19987_d_n4;
            locals.var_y1_dn5 = assign24600_body77_e19987_d_n5;
            locals.var_y1_dn6 = assign24600_body77_e19987_d_n6;
            locals.var_y1_dn7 = assign24600_body77_e19987_d_n7;
            locals.var_y1_dn8 = assign24600_body77_e19987_d_n8;
            locals.var_y1_dn9 = assign24600_body77_e19987_d_n9;
            locals.var_y1_dn10 = assign24600_body77_e19987_d_n10;
            locals.var_y1_dn13 = assign24600_body77_e19987_d_n13;
            locals.var_y1_rv = 0.0;
            let (assign24600_body78_e19995, assign24600_body78_e19995_d_n0, assign24600_body78_e19995_d_n2, assign24600_body78_e19995_d_n4, assign24600_body78_e19995_d_n5, assign24600_body78_e19995_d_n6, assign24600_body78_e19995_d_n7, assign24600_body78_e19995_d_n8, assign24600_body78_e19995_d_n9, assign24600_body78_e19995_d_n10, assign24600_body78_e19995_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard559 != 0.0)) {
        (locals.var_cox, locals.var_cox_dn0, locals.var_cox_dn2, locals.var_cox_dn4, locals.var_cox_dn5, locals.var_cox_dn6, locals.var_cox_dn7, locals.var_cox_dn8, locals.var_cox_dn9, locals.var_cox_dn10, locals.var_cox_dn13,)
    } else {
        (locals.var_y11, locals.var_y11_dn0, locals.var_y11_dn2, locals.var_y11_dn4, locals.var_y11_dn5, locals.var_y11_dn6, locals.var_y11_dn7, locals.var_y11_dn8, locals.var_y11_dn9, locals.var_y11_dn10, locals.var_y11_dn13,)
    }
};
            locals.var_y11 = assign24600_body78_e19995;
            locals.var_y11_dn0 = assign24600_body78_e19995_d_n0;
            locals.var_y11_dn2 = assign24600_body78_e19995_d_n2;
            locals.var_y11_dn4 = assign24600_body78_e19995_d_n4;
            locals.var_y11_dn5 = assign24600_body78_e19995_d_n5;
            locals.var_y11_dn6 = assign24600_body78_e19995_d_n6;
            locals.var_y11_dn7 = assign24600_body78_e19995_d_n7;
            locals.var_y11_dn8 = assign24600_body78_e19995_d_n8;
            locals.var_y11_dn9 = assign24600_body78_e19995_d_n9;
            locals.var_y11_dn10 = assign24600_body78_e19995_d_n10;
            locals.var_y11_dn13 = assign24600_body78_e19995_d_n13;
            locals.var_y11_rv = 0.0;
            let (assign24600_body79_e20005, assign24600_body79_e20005_d_n0, assign24600_body79_e20005_d_n2, assign24600_body79_e20005_d_n4, assign24600_body79_e20005_d_n5, assign24600_body79_e20005_d_n6, assign24600_body79_e20005_d_n7, assign24600_body79_e20005_d_n8, assign24600_body79_e20005_d_n9, assign24600_body79_e20005_d_n10, assign24600_body79_e20005_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard559 != 0.0)) {
        let assign24600_body79_e20003: f64 = (locals.var_q_b0_dep_dpd + locals.var_q_sub0_dep_dpd);
        (assign24600_body79_e20003, (locals.var_q_b0_dep_dpd_dn0 + locals.var_q_sub0_dep_dpd_dn0), (locals.var_q_b0_dep_dpd_dn2 + locals.var_q_sub0_dep_dpd_dn2), (locals.var_q_b0_dep_dpd_dn4 + locals.var_q_sub0_dep_dpd_dn4), (locals.var_q_b0_dep_dpd_dn5 + locals.var_q_sub0_dep_dpd_dn5), (locals.var_q_b0_dep_dpd_dn6 + locals.var_q_sub0_dep_dpd_dn6), (locals.var_q_b0_dep_dpd_dn7 + locals.var_q_sub0_dep_dpd_dn7), (locals.var_q_b0_dep_dpd_dn8 + locals.var_q_sub0_dep_dpd_dn8), (locals.var_q_b0_dep_dpd_dn9 + locals.var_q_sub0_dep_dpd_dn9), (locals.var_q_b0_dep_dpd_dn10 + locals.var_q_sub0_dep_dpd_dn10), (locals.var_q_b0_dep_dpd_dn13 + locals.var_q_sub0_dep_dpd_dn13),)
    } else {
        (locals.var_y12, locals.var_y12_dn0, locals.var_y12_dn2, locals.var_y12_dn4, locals.var_y12_dn5, locals.var_y12_dn6, locals.var_y12_dn7, locals.var_y12_dn8, locals.var_y12_dn9, locals.var_y12_dn10, locals.var_y12_dn13,)
    }
};
            locals.var_y12 = assign24600_body79_e20005;
            locals.var_y12_dn0 = assign24600_body79_e20005_d_n0;
            locals.var_y12_dn2 = assign24600_body79_e20005_d_n2;
            locals.var_y12_dn4 = assign24600_body79_e20005_d_n4;
            locals.var_y12_dn5 = assign24600_body79_e20005_d_n5;
            locals.var_y12_dn6 = assign24600_body79_e20005_d_n6;
            locals.var_y12_dn7 = assign24600_body79_e20005_d_n7;
            locals.var_y12_dn8 = assign24600_body79_e20005_d_n8;
            locals.var_y12_dn9 = assign24600_body79_e20005_d_n9;
            locals.var_y12_dn10 = assign24600_body79_e20005_d_n10;
            locals.var_y12_dn13 = assign24600_body79_e20005_d_n13;
            locals.var_y12_rv = 0.0;
            let (assign24600_body80_e20023, assign24600_body80_e20023_d_n0, assign24600_body80_e20023_d_n2, assign24600_body80_e20023_d_n4, assign24600_body80_e20023_d_n5, assign24600_body80_e20023_d_n6, assign24600_body80_e20023_d_n7, assign24600_body80_e20023_d_n8, assign24600_body80_e20023_d_n9, assign24600_body80_e20023_d_n10, assign24600_body80_e20023_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard559 != 0.0)) {
        let assign24600_body80_e20015: f64 = (locals.var_ndepmpnsub * locals.var_phi_b0_dep);
        let assign24600_body80_e20017: f64 = (assign24600_body80_e20015 + locals.var_vbscl__blk435);
        let assign24600_body80_e20019: f64 = (assign24600_body80_e20017 - locals.var_vbi_dep);
        let assign24600_body80_e20020: f64 = (locals.var_ndepmpnsub_inv1 * assign24600_body80_e20019);
        let assign24600_body80_e20021: f64 = (locals.var_phi_j0_dep - assign24600_body80_e20020);
        (assign24600_body80_e20021, (locals.var_phi_j0_dep_dn0 - ((locals.var_ndepmpnsub_inv1_dn0 * assign24600_body80_e20019) + (locals.var_ndepmpnsub_inv1 * ((((locals.var_ndepmpnsub_dn0 * locals.var_phi_b0_dep) + (locals.var_ndepmpnsub * locals.var_phi_b0_dep_dn0)) + locals.var_vbscl__blk435_dn0) - locals.var_vbi_dep_dn0)))), (locals.var_phi_j0_dep_dn2 - ((locals.var_ndepmpnsub_inv1_dn2 * assign24600_body80_e20019) + (locals.var_ndepmpnsub_inv1 * ((((locals.var_ndepmpnsub_dn2 * locals.var_phi_b0_dep) + (locals.var_ndepmpnsub * locals.var_phi_b0_dep_dn2)) + locals.var_vbscl__blk435_dn2) - locals.var_vbi_dep_dn2)))), (locals.var_phi_j0_dep_dn4 - ((locals.var_ndepmpnsub_inv1_dn4 * assign24600_body80_e20019) + (locals.var_ndepmpnsub_inv1 * ((((locals.var_ndepmpnsub_dn4 * locals.var_phi_b0_dep) + (locals.var_ndepmpnsub * locals.var_phi_b0_dep_dn4)) + locals.var_vbscl__blk435_dn4) - locals.var_vbi_dep_dn4)))), (locals.var_phi_j0_dep_dn5 - ((locals.var_ndepmpnsub_inv1_dn5 * assign24600_body80_e20019) + (locals.var_ndepmpnsub_inv1 * ((((locals.var_ndepmpnsub_dn5 * locals.var_phi_b0_dep) + (locals.var_ndepmpnsub * locals.var_phi_b0_dep_dn5)) + locals.var_vbscl__blk435_dn5) - locals.var_vbi_dep_dn5)))), (locals.var_phi_j0_dep_dn6 - ((locals.var_ndepmpnsub_inv1_dn6 * assign24600_body80_e20019) + (locals.var_ndepmpnsub_inv1 * ((((locals.var_ndepmpnsub_dn6 * locals.var_phi_b0_dep) + (locals.var_ndepmpnsub * locals.var_phi_b0_dep_dn6)) + locals.var_vbscl__blk435_dn6) - locals.var_vbi_dep_dn6)))), (locals.var_phi_j0_dep_dn7 - ((locals.var_ndepmpnsub_inv1_dn7 * assign24600_body80_e20019) + (locals.var_ndepmpnsub_inv1 * ((((locals.var_ndepmpnsub_dn7 * locals.var_phi_b0_dep) + (locals.var_ndepmpnsub * locals.var_phi_b0_dep_dn7)) + locals.var_vbscl__blk435_dn7) - locals.var_vbi_dep_dn7)))), (locals.var_phi_j0_dep_dn8 - ((locals.var_ndepmpnsub_inv1_dn8 * assign24600_body80_e20019) + (locals.var_ndepmpnsub_inv1 * ((((locals.var_ndepmpnsub_dn8 * locals.var_phi_b0_dep) + (locals.var_ndepmpnsub * locals.var_phi_b0_dep_dn8)) + locals.var_vbscl__blk435_dn8) - locals.var_vbi_dep_dn8)))), (locals.var_phi_j0_dep_dn9 - ((locals.var_ndepmpnsub_inv1_dn9 * assign24600_body80_e20019) + (locals.var_ndepmpnsub_inv1 * ((((locals.var_ndepmpnsub_dn9 * locals.var_phi_b0_dep) + (locals.var_ndepmpnsub * locals.var_phi_b0_dep_dn9)) + locals.var_vbscl__blk435_dn9) - locals.var_vbi_dep_dn9)))), (locals.var_phi_j0_dep_dn10 - ((locals.var_ndepmpnsub_inv1_dn10 * assign24600_body80_e20019) + (locals.var_ndepmpnsub_inv1 * ((((locals.var_ndepmpnsub_dn10 * locals.var_phi_b0_dep) + (locals.var_ndepmpnsub * locals.var_phi_b0_dep_dn10)) + locals.var_vbscl__blk435_dn10) - locals.var_vbi_dep_dn10)))), (locals.var_phi_j0_dep_dn13 - ((locals.var_ndepmpnsub_inv1_dn13 * assign24600_body80_e20019) + (locals.var_ndepmpnsub_inv1 * ((((locals.var_ndepmpnsub_dn13 * locals.var_phi_b0_dep) + (locals.var_ndepmpnsub * locals.var_phi_b0_dep_dn13)) + locals.var_vbscl__blk435_dn13) - locals.var_vbi_dep_dn13)))),)
    } else {
        (locals.var_y2, locals.var_y2_dn0, locals.var_y2_dn2, locals.var_y2_dn4, locals.var_y2_dn5, locals.var_y2_dn6, locals.var_y2_dn7, locals.var_y2_dn8, locals.var_y2_dn9, locals.var_y2_dn10, locals.var_y2_dn13,)
    }
};
            locals.var_y2 = assign24600_body80_e20023;
            locals.var_y2_dn0 = assign24600_body80_e20023_d_n0;
            locals.var_y2_dn2 = assign24600_body80_e20023_d_n2;
            locals.var_y2_dn4 = assign24600_body80_e20023_d_n4;
            locals.var_y2_dn5 = assign24600_body80_e20023_d_n5;
            locals.var_y2_dn6 = assign24600_body80_e20023_d_n6;
            locals.var_y2_dn7 = assign24600_body80_e20023_d_n7;
            locals.var_y2_dn8 = assign24600_body80_e20023_d_n8;
            locals.var_y2_dn9 = assign24600_body80_e20023_d_n9;
            locals.var_y2_dn10 = assign24600_body80_e20023_d_n10;
            locals.var_y2_dn13 = assign24600_body80_e20023_d_n13;
            locals.var_y2_rv = 0.0;
            let (assign24600_body81_e20031, assign24600_body81_e20031_d_n0, assign24600_body81_e20031_d_n2, assign24600_body81_e20031_d_n4, assign24600_body81_e20031_d_n5, assign24600_body81_e20031_d_n6, assign24600_body81_e20031_d_n7, assign24600_body81_e20031_d_n8, assign24600_body81_e20031_d_n9, assign24600_body81_e20031_d_n10, assign24600_body81_e20031_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard559 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_y21, locals.var_y21_dn0, locals.var_y21_dn2, locals.var_y21_dn4, locals.var_y21_dn5, locals.var_y21_dn6, locals.var_y21_dn7, locals.var_y21_dn8, locals.var_y21_dn9, locals.var_y21_dn10, locals.var_y21_dn13,)
    }
};
            locals.var_y21 = assign24600_body81_e20031;
            locals.var_y21_dn0 = assign24600_body81_e20031_d_n0;
            locals.var_y21_dn2 = assign24600_body81_e20031_d_n2;
            locals.var_y21_dn4 = assign24600_body81_e20031_d_n4;
            locals.var_y21_dn5 = assign24600_body81_e20031_d_n5;
            locals.var_y21_dn6 = assign24600_body81_e20031_d_n6;
            locals.var_y21_dn7 = assign24600_body81_e20031_d_n7;
            locals.var_y21_dn8 = assign24600_body81_e20031_d_n8;
            locals.var_y21_dn9 = assign24600_body81_e20031_d_n9;
            locals.var_y21_dn10 = assign24600_body81_e20031_d_n10;
            locals.var_y21_dn13 = assign24600_body81_e20031_d_n13;
            locals.var_y21_rv = 0.0;
            let (assign24600_body82_e20039, assign24600_body82_e20039_d_n0, assign24600_body82_e20039_d_n2, assign24600_body82_e20039_d_n4, assign24600_body82_e20039_d_n5, assign24600_body82_e20039_d_n6, assign24600_body82_e20039_d_n7, assign24600_body82_e20039_d_n8, assign24600_body82_e20039_d_n9, assign24600_body82_e20039_d_n10, assign24600_body82_e20039_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard559 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_y22, locals.var_y22_dn0, locals.var_y22_dn2, locals.var_y22_dn4, locals.var_y22_dn5, locals.var_y22_dn6, locals.var_y22_dn7, locals.var_y22_dn8, locals.var_y22_dn9, locals.var_y22_dn10, locals.var_y22_dn13,)
    }
};
            locals.var_y22 = assign24600_body82_e20039;
            locals.var_y22_dn0 = assign24600_body82_e20039_d_n0;
            locals.var_y22_dn2 = assign24600_body82_e20039_d_n2;
            locals.var_y22_dn4 = assign24600_body82_e20039_d_n4;
            locals.var_y22_dn5 = assign24600_body82_e20039_d_n5;
            locals.var_y22_dn6 = assign24600_body82_e20039_d_n6;
            locals.var_y22_dn7 = assign24600_body82_e20039_d_n7;
            locals.var_y22_dn8 = assign24600_body82_e20039_d_n8;
            locals.var_y22_dn9 = assign24600_body82_e20039_d_n9;
            locals.var_y22_dn10 = assign24600_body82_e20039_d_n10;
            locals.var_y22_dn13 = assign24600_body82_e20039_d_n13;
            locals.var_y22_rv = 0.0;
            let (assign24600_body83_e20053, assign24600_body83_e20053_d_n0, assign24600_body83_e20053_d_n2, assign24600_body83_e20053_d_n4, assign24600_body83_e20053_d_n5, assign24600_body83_e20053_d_n6, assign24600_body83_e20053_d_n7, assign24600_body83_e20053_d_n8, assign24600_body83_e20053_d_n9, assign24600_body83_e20053_d_n10, assign24600_body83_e20053_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard559 != 0.0)) {
        let assign24600_body83_e20047: f64 = (locals.var_y11 * locals.var_y22);
        let assign24600_body83_e20050: f64 = (locals.var_y21 * locals.var_y12);
        let assign24600_body83_e20051: f64 = (assign24600_body83_e20047 - assign24600_body83_e20050);
        (assign24600_body83_e20051, (((locals.var_y11_dn0 * locals.var_y22) + (locals.var_y11 * locals.var_y22_dn0)) - ((locals.var_y21_dn0 * locals.var_y12) + (locals.var_y21 * locals.var_y12_dn0))), (((locals.var_y11_dn2 * locals.var_y22) + (locals.var_y11 * locals.var_y22_dn2)) - ((locals.var_y21_dn2 * locals.var_y12) + (locals.var_y21 * locals.var_y12_dn2))), (((locals.var_y11_dn4 * locals.var_y22) + (locals.var_y11 * locals.var_y22_dn4)) - ((locals.var_y21_dn4 * locals.var_y12) + (locals.var_y21 * locals.var_y12_dn4))), (((locals.var_y11_dn5 * locals.var_y22) + (locals.var_y11 * locals.var_y22_dn5)) - ((locals.var_y21_dn5 * locals.var_y12) + (locals.var_y21 * locals.var_y12_dn5))), (((locals.var_y11_dn6 * locals.var_y22) + (locals.var_y11 * locals.var_y22_dn6)) - ((locals.var_y21_dn6 * locals.var_y12) + (locals.var_y21 * locals.var_y12_dn6))), (((locals.var_y11_dn7 * locals.var_y22) + (locals.var_y11 * locals.var_y22_dn7)) - ((locals.var_y21_dn7 * locals.var_y12) + (locals.var_y21 * locals.var_y12_dn7))), (((locals.var_y11_dn8 * locals.var_y22) + (locals.var_y11 * locals.var_y22_dn8)) - ((locals.var_y21_dn8 * locals.var_y12) + (locals.var_y21 * locals.var_y12_dn8))), (((locals.var_y11_dn9 * locals.var_y22) + (locals.var_y11 * locals.var_y22_dn9)) - ((locals.var_y21_dn9 * locals.var_y12) + (locals.var_y21 * locals.var_y12_dn9))), (((locals.var_y11_dn10 * locals.var_y22) + (locals.var_y11 * locals.var_y22_dn10)) - ((locals.var_y21_dn10 * locals.var_y12) + (locals.var_y21 * locals.var_y12_dn10))), (((locals.var_y11_dn13 * locals.var_y22) + (locals.var_y11 * locals.var_y22_dn13)) - ((locals.var_y21_dn13 * locals.var_y12) + (locals.var_y21 * locals.var_y12_dn13))),)
    } else {
        (locals.var_dety, locals.var_dety_dn0, locals.var_dety_dn2, locals.var_dety_dn4, locals.var_dety_dn5, locals.var_dety_dn6, locals.var_dety_dn7, locals.var_dety_dn8, locals.var_dety_dn9, locals.var_dety_dn10, locals.var_dety_dn13,)
    }
};
            locals.var_dety = assign24600_body83_e20053;
            locals.var_dety_dn0 = assign24600_body83_e20053_d_n0;
            locals.var_dety_dn2 = assign24600_body83_e20053_d_n2;
            locals.var_dety_dn4 = assign24600_body83_e20053_d_n4;
            locals.var_dety_dn5 = assign24600_body83_e20053_d_n5;
            locals.var_dety_dn6 = assign24600_body83_e20053_d_n6;
            locals.var_dety_dn7 = assign24600_body83_e20053_d_n7;
            locals.var_dety_dn8 = assign24600_body83_e20053_d_n8;
            locals.var_dety_dn9 = assign24600_body83_e20053_d_n9;
            locals.var_dety_dn10 = assign24600_body83_e20053_d_n10;
            locals.var_dety_dn13 = assign24600_body83_e20053_d_n13;
            locals.var_dety_rv = 0.0;
            let (assign24600_body84_e20063, assign24600_body84_e20063_d_n0, assign24600_body84_e20063_d_n2, assign24600_body84_e20063_d_n4, assign24600_body84_e20063_d_n5, assign24600_body84_e20063_d_n6, assign24600_body84_e20063_d_n7, assign24600_body84_e20063_d_n8, assign24600_body84_e20063_d_n9, assign24600_body84_e20063_d_n10, assign24600_body84_e20063_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard559 != 0.0)) {
        let assign24600_body84_e20061: f64 = (locals.var_y22 / locals.var_dety);
        (assign24600_body84_e20061, (((locals.var_y22_dn0 * locals.var_dety) - (locals.var_y22 * locals.var_dety_dn0)) / (locals.var_dety * locals.var_dety)), (((locals.var_y22_dn2 * locals.var_dety) - (locals.var_y22 * locals.var_dety_dn2)) / (locals.var_dety * locals.var_dety)), (((locals.var_y22_dn4 * locals.var_dety) - (locals.var_y22 * locals.var_dety_dn4)) / (locals.var_dety * locals.var_dety)), (((locals.var_y22_dn5 * locals.var_dety) - (locals.var_y22 * locals.var_dety_dn5)) / (locals.var_dety * locals.var_dety)), (((locals.var_y22_dn6 * locals.var_dety) - (locals.var_y22 * locals.var_dety_dn6)) / (locals.var_dety * locals.var_dety)), (((locals.var_y22_dn7 * locals.var_dety) - (locals.var_y22 * locals.var_dety_dn7)) / (locals.var_dety * locals.var_dety)), (((locals.var_y22_dn8 * locals.var_dety) - (locals.var_y22 * locals.var_dety_dn8)) / (locals.var_dety * locals.var_dety)), (((locals.var_y22_dn9 * locals.var_dety) - (locals.var_y22 * locals.var_dety_dn9)) / (locals.var_dety * locals.var_dety)), (((locals.var_y22_dn10 * locals.var_dety) - (locals.var_y22 * locals.var_dety_dn10)) / (locals.var_dety * locals.var_dety)), (((locals.var_y22_dn13 * locals.var_dety) - (locals.var_y22 * locals.var_dety_dn13)) / (locals.var_dety * locals.var_dety)),)
    } else {
        (locals.var_rev11, locals.var_rev11_dn0, locals.var_rev11_dn2, locals.var_rev11_dn4, locals.var_rev11_dn5, locals.var_rev11_dn6, locals.var_rev11_dn7, locals.var_rev11_dn8, locals.var_rev11_dn9, locals.var_rev11_dn10, locals.var_rev11_dn13,)
    }
};
            locals.var_rev11 = assign24600_body84_e20063;
            locals.var_rev11_dn0 = assign24600_body84_e20063_d_n0;
            locals.var_rev11_dn2 = assign24600_body84_e20063_d_n2;
            locals.var_rev11_dn4 = assign24600_body84_e20063_d_n4;
            locals.var_rev11_dn5 = assign24600_body84_e20063_d_n5;
            locals.var_rev11_dn6 = assign24600_body84_e20063_d_n6;
            locals.var_rev11_dn7 = assign24600_body84_e20063_d_n7;
            locals.var_rev11_dn8 = assign24600_body84_e20063_d_n8;
            locals.var_rev11_dn9 = assign24600_body84_e20063_d_n9;
            locals.var_rev11_dn10 = assign24600_body84_e20063_d_n10;
            locals.var_rev11_dn13 = assign24600_body84_e20063_d_n13;
            locals.var_rev11_rv = 0.0;
            let (assign24600_body85_e20074, assign24600_body85_e20074_d_n0, assign24600_body85_e20074_d_n2, assign24600_body85_e20074_d_n4, assign24600_body85_e20074_d_n5, assign24600_body85_e20074_d_n6, assign24600_body85_e20074_d_n7, assign24600_body85_e20074_d_n8, assign24600_body85_e20074_d_n9, assign24600_body85_e20074_d_n10, assign24600_body85_e20074_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard559 != 0.0)) {
        let assign24600_body85_e20070: f64 = (-locals.var_y12);
        let assign24600_body85_e20072: f64 = (assign24600_body85_e20070 / locals.var_dety);
        (assign24600_body85_e20072, ((((-locals.var_y12_dn0) * locals.var_dety) - (assign24600_body85_e20070 * locals.var_dety_dn0)) / (locals.var_dety * locals.var_dety)), ((((-locals.var_y12_dn2) * locals.var_dety) - (assign24600_body85_e20070 * locals.var_dety_dn2)) / (locals.var_dety * locals.var_dety)), ((((-locals.var_y12_dn4) * locals.var_dety) - (assign24600_body85_e20070 * locals.var_dety_dn4)) / (locals.var_dety * locals.var_dety)), ((((-locals.var_y12_dn5) * locals.var_dety) - (assign24600_body85_e20070 * locals.var_dety_dn5)) / (locals.var_dety * locals.var_dety)), ((((-locals.var_y12_dn6) * locals.var_dety) - (assign24600_body85_e20070 * locals.var_dety_dn6)) / (locals.var_dety * locals.var_dety)), ((((-locals.var_y12_dn7) * locals.var_dety) - (assign24600_body85_e20070 * locals.var_dety_dn7)) / (locals.var_dety * locals.var_dety)), ((((-locals.var_y12_dn8) * locals.var_dety) - (assign24600_body85_e20070 * locals.var_dety_dn8)) / (locals.var_dety * locals.var_dety)), ((((-locals.var_y12_dn9) * locals.var_dety) - (assign24600_body85_e20070 * locals.var_dety_dn9)) / (locals.var_dety * locals.var_dety)), ((((-locals.var_y12_dn10) * locals.var_dety) - (assign24600_body85_e20070 * locals.var_dety_dn10)) / (locals.var_dety * locals.var_dety)), ((((-locals.var_y12_dn13) * locals.var_dety) - (assign24600_body85_e20070 * locals.var_dety_dn13)) / (locals.var_dety * locals.var_dety)),)
    } else {
        (locals.var_rev12, locals.var_rev12_dn0, locals.var_rev12_dn2, locals.var_rev12_dn4, locals.var_rev12_dn5, locals.var_rev12_dn6, locals.var_rev12_dn7, locals.var_rev12_dn8, locals.var_rev12_dn9, locals.var_rev12_dn10, locals.var_rev12_dn13,)
    }
};
            locals.var_rev12 = assign24600_body85_e20074;
            locals.var_rev12_dn0 = assign24600_body85_e20074_d_n0;
            locals.var_rev12_dn2 = assign24600_body85_e20074_d_n2;
            locals.var_rev12_dn4 = assign24600_body85_e20074_d_n4;
            locals.var_rev12_dn5 = assign24600_body85_e20074_d_n5;
            locals.var_rev12_dn6 = assign24600_body85_e20074_d_n6;
            locals.var_rev12_dn7 = assign24600_body85_e20074_d_n7;
            locals.var_rev12_dn8 = assign24600_body85_e20074_d_n8;
            locals.var_rev12_dn9 = assign24600_body85_e20074_d_n9;
            locals.var_rev12_dn10 = assign24600_body85_e20074_d_n10;
            locals.var_rev12_dn13 = assign24600_body85_e20074_d_n13;
            locals.var_rev12_rv = 0.0;
            let (assign24600_body86_e20085, assign24600_body86_e20085_d_n0, assign24600_body86_e20085_d_n2, assign24600_body86_e20085_d_n4, assign24600_body86_e20085_d_n5, assign24600_body86_e20085_d_n6, assign24600_body86_e20085_d_n7, assign24600_body86_e20085_d_n8, assign24600_body86_e20085_d_n9, assign24600_body86_e20085_d_n10, assign24600_body86_e20085_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard559 != 0.0)) {
        let assign24600_body86_e20081: f64 = (-locals.var_y21);
        let assign24600_body86_e20083: f64 = (assign24600_body86_e20081 / locals.var_dety);
        (assign24600_body86_e20083, ((((-locals.var_y21_dn0) * locals.var_dety) - (assign24600_body86_e20081 * locals.var_dety_dn0)) / (locals.var_dety * locals.var_dety)), ((((-locals.var_y21_dn2) * locals.var_dety) - (assign24600_body86_e20081 * locals.var_dety_dn2)) / (locals.var_dety * locals.var_dety)), ((((-locals.var_y21_dn4) * locals.var_dety) - (assign24600_body86_e20081 * locals.var_dety_dn4)) / (locals.var_dety * locals.var_dety)), ((((-locals.var_y21_dn5) * locals.var_dety) - (assign24600_body86_e20081 * locals.var_dety_dn5)) / (locals.var_dety * locals.var_dety)), ((((-locals.var_y21_dn6) * locals.var_dety) - (assign24600_body86_e20081 * locals.var_dety_dn6)) / (locals.var_dety * locals.var_dety)), ((((-locals.var_y21_dn7) * locals.var_dety) - (assign24600_body86_e20081 * locals.var_dety_dn7)) / (locals.var_dety * locals.var_dety)), ((((-locals.var_y21_dn8) * locals.var_dety) - (assign24600_body86_e20081 * locals.var_dety_dn8)) / (locals.var_dety * locals.var_dety)), ((((-locals.var_y21_dn9) * locals.var_dety) - (assign24600_body86_e20081 * locals.var_dety_dn9)) / (locals.var_dety * locals.var_dety)), ((((-locals.var_y21_dn10) * locals.var_dety) - (assign24600_body86_e20081 * locals.var_dety_dn10)) / (locals.var_dety * locals.var_dety)), ((((-locals.var_y21_dn13) * locals.var_dety) - (assign24600_body86_e20081 * locals.var_dety_dn13)) / (locals.var_dety * locals.var_dety)),)
    } else {
        (locals.var_rev21, locals.var_rev21_dn0, locals.var_rev21_dn2, locals.var_rev21_dn4, locals.var_rev21_dn5, locals.var_rev21_dn6, locals.var_rev21_dn7, locals.var_rev21_dn8, locals.var_rev21_dn9, locals.var_rev21_dn10, locals.var_rev21_dn13,)
    }
};
            locals.var_rev21 = assign24600_body86_e20085;
            locals.var_rev21_dn0 = assign24600_body86_e20085_d_n0;
            locals.var_rev21_dn2 = assign24600_body86_e20085_d_n2;
            locals.var_rev21_dn4 = assign24600_body86_e20085_d_n4;
            locals.var_rev21_dn5 = assign24600_body86_e20085_d_n5;
            locals.var_rev21_dn6 = assign24600_body86_e20085_d_n6;
            locals.var_rev21_dn7 = assign24600_body86_e20085_d_n7;
            locals.var_rev21_dn8 = assign24600_body86_e20085_d_n8;
            locals.var_rev21_dn9 = assign24600_body86_e20085_d_n9;
            locals.var_rev21_dn10 = assign24600_body86_e20085_d_n10;
            locals.var_rev21_dn13 = assign24600_body86_e20085_d_n13;
            locals.var_rev21_rv = 0.0;
            let (assign24600_body87_e20095, assign24600_body87_e20095_d_n0, assign24600_body87_e20095_d_n2, assign24600_body87_e20095_d_n4, assign24600_body87_e20095_d_n5, assign24600_body87_e20095_d_n6, assign24600_body87_e20095_d_n7, assign24600_body87_e20095_d_n8, assign24600_body87_e20095_d_n9, assign24600_body87_e20095_d_n10, assign24600_body87_e20095_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard559 != 0.0)) {
        let assign24600_body87_e20093: f64 = (locals.var_y11 / locals.var_dety);
        (assign24600_body87_e20093, (((locals.var_y11_dn0 * locals.var_dety) - (locals.var_y11 * locals.var_dety_dn0)) / (locals.var_dety * locals.var_dety)), (((locals.var_y11_dn2 * locals.var_dety) - (locals.var_y11 * locals.var_dety_dn2)) / (locals.var_dety * locals.var_dety)), (((locals.var_y11_dn4 * locals.var_dety) - (locals.var_y11 * locals.var_dety_dn4)) / (locals.var_dety * locals.var_dety)), (((locals.var_y11_dn5 * locals.var_dety) - (locals.var_y11 * locals.var_dety_dn5)) / (locals.var_dety * locals.var_dety)), (((locals.var_y11_dn6 * locals.var_dety) - (locals.var_y11 * locals.var_dety_dn6)) / (locals.var_dety * locals.var_dety)), (((locals.var_y11_dn7 * locals.var_dety) - (locals.var_y11 * locals.var_dety_dn7)) / (locals.var_dety * locals.var_dety)), (((locals.var_y11_dn8 * locals.var_dety) - (locals.var_y11 * locals.var_dety_dn8)) / (locals.var_dety * locals.var_dety)), (((locals.var_y11_dn9 * locals.var_dety) - (locals.var_y11 * locals.var_dety_dn9)) / (locals.var_dety * locals.var_dety)), (((locals.var_y11_dn10 * locals.var_dety) - (locals.var_y11 * locals.var_dety_dn10)) / (locals.var_dety * locals.var_dety)), (((locals.var_y11_dn13 * locals.var_dety) - (locals.var_y11 * locals.var_dety_dn13)) / (locals.var_dety * locals.var_dety)),)
    } else {
        (locals.var_rev22, locals.var_rev22_dn0, locals.var_rev22_dn2, locals.var_rev22_dn4, locals.var_rev22_dn5, locals.var_rev22_dn6, locals.var_rev22_dn7, locals.var_rev22_dn8, locals.var_rev22_dn9, locals.var_rev22_dn10, locals.var_rev22_dn13,)
    }
};
            locals.var_rev22 = assign24600_body87_e20095;
            locals.var_rev22_dn0 = assign24600_body87_e20095_d_n0;
            locals.var_rev22_dn2 = assign24600_body87_e20095_d_n2;
            locals.var_rev22_dn4 = assign24600_body87_e20095_d_n4;
            locals.var_rev22_dn5 = assign24600_body87_e20095_d_n5;
            locals.var_rev22_dn6 = assign24600_body87_e20095_d_n6;
            locals.var_rev22_dn7 = assign24600_body87_e20095_d_n7;
            locals.var_rev22_dn8 = assign24600_body87_e20095_d_n8;
            locals.var_rev22_dn9 = assign24600_body87_e20095_d_n9;
            locals.var_rev22_dn10 = assign24600_body87_e20095_d_n10;
            locals.var_rev22_dn13 = assign24600_body87_e20095_d_n13;
            locals.var_rev22_rv = 0.0;
            let assign24600_body88_e20098: f64 = (locals.var_rev11 * locals.var_y1);
            let assign24600_body88_e20101: f64 = (locals.var_rev12 * locals.var_y2);
            let assign24600_body88_e20102: f64 = (assign24600_body88_e20098 + assign24600_body88_e20101);
            let assign24600_body88_e20103: f64 = (assign24600_body88_e20102).abs();
            let assign24600_body88_e20105: f64 = if assign24600_body88_e20103 > 0.5 { 1.0 } else { 0.0 };
            locals.var_guard572 = assign24600_body88_e20105;
            locals.var_guard572_rv = 0.0;
            let (assign24600_body89_e20131, assign24600_body89_e20131_d_n0, assign24600_body89_e20131_d_n2, assign24600_body89_e20131_d_n4, assign24600_body89_e20131_d_n5, assign24600_body89_e20131_d_n6, assign24600_body89_e20131_d_n7, assign24600_body89_e20131_d_n8, assign24600_body89_e20131_d_n9, assign24600_body89_e20131_d_n10, assign24600_body89_e20131_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard559 != 0.0)) && (locals.var_guard572 != 0.0)) {
        let assign24600_body89_e20117: f64 = (locals.var_rev11 * locals.var_y1);
        let assign24600_body89_e20120: f64 = (locals.var_rev12 * locals.var_y2);
        let assign24600_body89_e20121: f64 = (assign24600_body89_e20117 + assign24600_body89_e20120);
        let (assign24600_body89_e20127,) = {
            if (assign24600_body89_e20121 >= 0.0) {
                (1.0,)
            } else {
                let assign24600_body89_e20126: f64 = (-1.0);
                (assign24600_body89_e20126,)
            }
        };
        let assign24600_body89_e20128: f64 = (0.5 * assign24600_body89_e20127);
        let assign24600_body89_e20129: f64 = (locals.var_vgp0 - assign24600_body89_e20128);
        (assign24600_body89_e20129, locals.var_vgp0_dn0, locals.var_vgp0_dn2, locals.var_vgp0_dn4, locals.var_vgp0_dn5, locals.var_vgp0_dn6, locals.var_vgp0_dn7, locals.var_vgp0_dn8, locals.var_vgp0_dn9, locals.var_vgp0_dn10, locals.var_vgp0_dn13,)
    } else {
        (locals.var_vgp0, locals.var_vgp0_dn0, locals.var_vgp0_dn2, locals.var_vgp0_dn4, locals.var_vgp0_dn5, locals.var_vgp0_dn6, locals.var_vgp0_dn7, locals.var_vgp0_dn8, locals.var_vgp0_dn9, locals.var_vgp0_dn10, locals.var_vgp0_dn13,)
    }
};
            locals.var_vgp0 = assign24600_body89_e20131;
            locals.var_vgp0_dn0 = assign24600_body89_e20131_d_n0;
            locals.var_vgp0_dn2 = assign24600_body89_e20131_d_n2;
            locals.var_vgp0_dn4 = assign24600_body89_e20131_d_n4;
            locals.var_vgp0_dn5 = assign24600_body89_e20131_d_n5;
            locals.var_vgp0_dn6 = assign24600_body89_e20131_d_n6;
            locals.var_vgp0_dn7 = assign24600_body89_e20131_d_n7;
            locals.var_vgp0_dn8 = assign24600_body89_e20131_d_n8;
            locals.var_vgp0_dn9 = assign24600_body89_e20131_d_n9;
            locals.var_vgp0_dn10 = assign24600_body89_e20131_d_n10;
            locals.var_vgp0_dn13 = assign24600_body89_e20131_d_n13;
            locals.var_vgp0_rv = 0.0;
            let (assign24600_body90_e20157, assign24600_body90_e20157_d_n0, assign24600_body90_e20157_d_n2, assign24600_body90_e20157_d_n4, assign24600_body90_e20157_d_n5, assign24600_body90_e20157_d_n6, assign24600_body90_e20157_d_n7, assign24600_body90_e20157_d_n8, assign24600_body90_e20157_d_n9, assign24600_body90_e20157_d_n10, assign24600_body90_e20157_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard559 != 0.0)) && (locals.var_guard572 != 0.0)) {
        let assign24600_body90_e20143: f64 = (locals.var_rev21 * locals.var_y1);
        let assign24600_body90_e20146: f64 = (locals.var_rev22 * locals.var_y2);
        let assign24600_body90_e20147: f64 = (assign24600_body90_e20143 + assign24600_body90_e20146);
        let (assign24600_body90_e20153,) = {
            if (assign24600_body90_e20147 >= 0.0) {
                (1.0,)
            } else {
                let assign24600_body90_e20152: f64 = (-1.0);
                (assign24600_body90_e20152,)
            }
        };
        let assign24600_body90_e20154: f64 = (0.5 * assign24600_body90_e20153);
        let assign24600_body90_e20155: f64 = (locals.var_phi_j0_dep - assign24600_body90_e20154);
        (assign24600_body90_e20155, locals.var_phi_j0_dep_dn0, locals.var_phi_j0_dep_dn2, locals.var_phi_j0_dep_dn4, locals.var_phi_j0_dep_dn5, locals.var_phi_j0_dep_dn6, locals.var_phi_j0_dep_dn7, locals.var_phi_j0_dep_dn8, locals.var_phi_j0_dep_dn9, locals.var_phi_j0_dep_dn10, locals.var_phi_j0_dep_dn13,)
    } else {
        (locals.var_phi_j0_dep, locals.var_phi_j0_dep_dn0, locals.var_phi_j0_dep_dn2, locals.var_phi_j0_dep_dn4, locals.var_phi_j0_dep_dn5, locals.var_phi_j0_dep_dn6, locals.var_phi_j0_dep_dn7, locals.var_phi_j0_dep_dn8, locals.var_phi_j0_dep_dn9, locals.var_phi_j0_dep_dn10, locals.var_phi_j0_dep_dn13,)
    }
};
            locals.var_phi_j0_dep = assign24600_body90_e20157;
            locals.var_phi_j0_dep_dn0 = assign24600_body90_e20157_d_n0;
            locals.var_phi_j0_dep_dn2 = assign24600_body90_e20157_d_n2;
            locals.var_phi_j0_dep_dn4 = assign24600_body90_e20157_d_n4;
            locals.var_phi_j0_dep_dn5 = assign24600_body90_e20157_d_n5;
            locals.var_phi_j0_dep_dn6 = assign24600_body90_e20157_d_n6;
            locals.var_phi_j0_dep_dn7 = assign24600_body90_e20157_d_n7;
            locals.var_phi_j0_dep_dn8 = assign24600_body90_e20157_d_n8;
            locals.var_phi_j0_dep_dn9 = assign24600_body90_e20157_d_n9;
            locals.var_phi_j0_dep_dn10 = assign24600_body90_e20157_d_n10;
            locals.var_phi_j0_dep_dn13 = assign24600_body90_e20157_d_n13;
            locals.var_phi_j0_dep_rv = 0.0;
            let (assign24600_body91_e20176, assign24600_body91_e20176_d_n0, assign24600_body91_e20176_d_n2, assign24600_body91_e20176_d_n4, assign24600_body91_e20176_d_n5, assign24600_body91_e20176_d_n6, assign24600_body91_e20176_d_n7, assign24600_body91_e20176_d_n8, assign24600_body91_e20176_d_n9, assign24600_body91_e20176_d_n10, assign24600_body91_e20176_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard559 != 0.0)) && (locals.var_guard572 == 0.0)) {
        let assign24600_body91_e20169: f64 = (locals.var_rev11 * locals.var_y1);
        let assign24600_body91_e20172: f64 = (locals.var_rev12 * locals.var_y2);
        let assign24600_body91_e20173: f64 = (assign24600_body91_e20169 + assign24600_body91_e20172);
        let assign24600_body91_e20174: f64 = (locals.var_vgp0 - assign24600_body91_e20173);
        (assign24600_body91_e20174, (locals.var_vgp0_dn0 - (((locals.var_rev11_dn0 * locals.var_y1) + (locals.var_rev11 * locals.var_y1_dn0)) + ((locals.var_rev12_dn0 * locals.var_y2) + (locals.var_rev12 * locals.var_y2_dn0)))), (locals.var_vgp0_dn2 - (((locals.var_rev11_dn2 * locals.var_y1) + (locals.var_rev11 * locals.var_y1_dn2)) + ((locals.var_rev12_dn2 * locals.var_y2) + (locals.var_rev12 * locals.var_y2_dn2)))), (locals.var_vgp0_dn4 - (((locals.var_rev11_dn4 * locals.var_y1) + (locals.var_rev11 * locals.var_y1_dn4)) + ((locals.var_rev12_dn4 * locals.var_y2) + (locals.var_rev12 * locals.var_y2_dn4)))), (locals.var_vgp0_dn5 - (((locals.var_rev11_dn5 * locals.var_y1) + (locals.var_rev11 * locals.var_y1_dn5)) + ((locals.var_rev12_dn5 * locals.var_y2) + (locals.var_rev12 * locals.var_y2_dn5)))), (locals.var_vgp0_dn6 - (((locals.var_rev11_dn6 * locals.var_y1) + (locals.var_rev11 * locals.var_y1_dn6)) + ((locals.var_rev12_dn6 * locals.var_y2) + (locals.var_rev12 * locals.var_y2_dn6)))), (locals.var_vgp0_dn7 - (((locals.var_rev11_dn7 * locals.var_y1) + (locals.var_rev11 * locals.var_y1_dn7)) + ((locals.var_rev12_dn7 * locals.var_y2) + (locals.var_rev12 * locals.var_y2_dn7)))), (locals.var_vgp0_dn8 - (((locals.var_rev11_dn8 * locals.var_y1) + (locals.var_rev11 * locals.var_y1_dn8)) + ((locals.var_rev12_dn8 * locals.var_y2) + (locals.var_rev12 * locals.var_y2_dn8)))), (locals.var_vgp0_dn9 - (((locals.var_rev11_dn9 * locals.var_y1) + (locals.var_rev11 * locals.var_y1_dn9)) + ((locals.var_rev12_dn9 * locals.var_y2) + (locals.var_rev12 * locals.var_y2_dn9)))), (locals.var_vgp0_dn10 - (((locals.var_rev11_dn10 * locals.var_y1) + (locals.var_rev11 * locals.var_y1_dn10)) + ((locals.var_rev12_dn10 * locals.var_y2) + (locals.var_rev12 * locals.var_y2_dn10)))), (locals.var_vgp0_dn13 - (((locals.var_rev11_dn13 * locals.var_y1) + (locals.var_rev11 * locals.var_y1_dn13)) + ((locals.var_rev12_dn13 * locals.var_y2) + (locals.var_rev12 * locals.var_y2_dn13)))),)
    } else {
        (locals.var_vgp0, locals.var_vgp0_dn0, locals.var_vgp0_dn2, locals.var_vgp0_dn4, locals.var_vgp0_dn5, locals.var_vgp0_dn6, locals.var_vgp0_dn7, locals.var_vgp0_dn8, locals.var_vgp0_dn9, locals.var_vgp0_dn10, locals.var_vgp0_dn13,)
    }
};
            locals.var_vgp0 = assign24600_body91_e20176;
            locals.var_vgp0_dn0 = assign24600_body91_e20176_d_n0;
            locals.var_vgp0_dn2 = assign24600_body91_e20176_d_n2;
            locals.var_vgp0_dn4 = assign24600_body91_e20176_d_n4;
            locals.var_vgp0_dn5 = assign24600_body91_e20176_d_n5;
            locals.var_vgp0_dn6 = assign24600_body91_e20176_d_n6;
            locals.var_vgp0_dn7 = assign24600_body91_e20176_d_n7;
            locals.var_vgp0_dn8 = assign24600_body91_e20176_d_n8;
            locals.var_vgp0_dn9 = assign24600_body91_e20176_d_n9;
            locals.var_vgp0_dn10 = assign24600_body91_e20176_d_n10;
            locals.var_vgp0_dn13 = assign24600_body91_e20176_d_n13;
            locals.var_vgp0_rv = 0.0;
            let (assign24600_body92_e20195, assign24600_body92_e20195_d_n0, assign24600_body92_e20195_d_n2, assign24600_body92_e20195_d_n4, assign24600_body92_e20195_d_n5, assign24600_body92_e20195_d_n6, assign24600_body92_e20195_d_n7, assign24600_body92_e20195_d_n8, assign24600_body92_e20195_d_n9, assign24600_body92_e20195_d_n10, assign24600_body92_e20195_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard559 != 0.0)) && (locals.var_guard572 == 0.0)) {
        let assign24600_body92_e20188: f64 = (locals.var_rev21 * locals.var_y1);
        let assign24600_body92_e20191: f64 = (locals.var_rev22 * locals.var_y2);
        let assign24600_body92_e20192: f64 = (assign24600_body92_e20188 + assign24600_body92_e20191);
        let assign24600_body92_e20193: f64 = (locals.var_phi_j0_dep - assign24600_body92_e20192);
        (assign24600_body92_e20193, (locals.var_phi_j0_dep_dn0 - (((locals.var_rev21_dn0 * locals.var_y1) + (locals.var_rev21 * locals.var_y1_dn0)) + ((locals.var_rev22_dn0 * locals.var_y2) + (locals.var_rev22 * locals.var_y2_dn0)))), (locals.var_phi_j0_dep_dn2 - (((locals.var_rev21_dn2 * locals.var_y1) + (locals.var_rev21 * locals.var_y1_dn2)) + ((locals.var_rev22_dn2 * locals.var_y2) + (locals.var_rev22 * locals.var_y2_dn2)))), (locals.var_phi_j0_dep_dn4 - (((locals.var_rev21_dn4 * locals.var_y1) + (locals.var_rev21 * locals.var_y1_dn4)) + ((locals.var_rev22_dn4 * locals.var_y2) + (locals.var_rev22 * locals.var_y2_dn4)))), (locals.var_phi_j0_dep_dn5 - (((locals.var_rev21_dn5 * locals.var_y1) + (locals.var_rev21 * locals.var_y1_dn5)) + ((locals.var_rev22_dn5 * locals.var_y2) + (locals.var_rev22 * locals.var_y2_dn5)))), (locals.var_phi_j0_dep_dn6 - (((locals.var_rev21_dn6 * locals.var_y1) + (locals.var_rev21 * locals.var_y1_dn6)) + ((locals.var_rev22_dn6 * locals.var_y2) + (locals.var_rev22 * locals.var_y2_dn6)))), (locals.var_phi_j0_dep_dn7 - (((locals.var_rev21_dn7 * locals.var_y1) + (locals.var_rev21 * locals.var_y1_dn7)) + ((locals.var_rev22_dn7 * locals.var_y2) + (locals.var_rev22 * locals.var_y2_dn7)))), (locals.var_phi_j0_dep_dn8 - (((locals.var_rev21_dn8 * locals.var_y1) + (locals.var_rev21 * locals.var_y1_dn8)) + ((locals.var_rev22_dn8 * locals.var_y2) + (locals.var_rev22 * locals.var_y2_dn8)))), (locals.var_phi_j0_dep_dn9 - (((locals.var_rev21_dn9 * locals.var_y1) + (locals.var_rev21 * locals.var_y1_dn9)) + ((locals.var_rev22_dn9 * locals.var_y2) + (locals.var_rev22 * locals.var_y2_dn9)))), (locals.var_phi_j0_dep_dn10 - (((locals.var_rev21_dn10 * locals.var_y1) + (locals.var_rev21 * locals.var_y1_dn10)) + ((locals.var_rev22_dn10 * locals.var_y2) + (locals.var_rev22 * locals.var_y2_dn10)))), (locals.var_phi_j0_dep_dn13 - (((locals.var_rev21_dn13 * locals.var_y1) + (locals.var_rev21 * locals.var_y1_dn13)) + ((locals.var_rev22_dn13 * locals.var_y2) + (locals.var_rev22 * locals.var_y2_dn13)))),)
    } else {
        (locals.var_phi_j0_dep, locals.var_phi_j0_dep_dn0, locals.var_phi_j0_dep_dn2, locals.var_phi_j0_dep_dn4, locals.var_phi_j0_dep_dn5, locals.var_phi_j0_dep_dn6, locals.var_phi_j0_dep_dn7, locals.var_phi_j0_dep_dn8, locals.var_phi_j0_dep_dn9, locals.var_phi_j0_dep_dn10, locals.var_phi_j0_dep_dn13,)
    }
};
            locals.var_phi_j0_dep = assign24600_body92_e20195;
            locals.var_phi_j0_dep_dn0 = assign24600_body92_e20195_d_n0;
            locals.var_phi_j0_dep_dn2 = assign24600_body92_e20195_d_n2;
            locals.var_phi_j0_dep_dn4 = assign24600_body92_e20195_d_n4;
            locals.var_phi_j0_dep_dn5 = assign24600_body92_e20195_d_n5;
            locals.var_phi_j0_dep_dn6 = assign24600_body92_e20195_d_n6;
            locals.var_phi_j0_dep_dn7 = assign24600_body92_e20195_d_n7;
            locals.var_phi_j0_dep_dn8 = assign24600_body92_e20195_d_n8;
            locals.var_phi_j0_dep_dn9 = assign24600_body92_e20195_d_n9;
            locals.var_phi_j0_dep_dn10 = assign24600_body92_e20195_d_n10;
            locals.var_phi_j0_dep_dn13 = assign24600_body92_e20195_d_n13;
            locals.var_phi_j0_dep_rv = 0.0;
            let assign24600_body93_e20198: f64 = (locals.var_vgp0 - locals.var_vgp0old);
            let assign24600_body93_e20199: f64 = (assign24600_body93_e20198).abs();
            let assign24600_body93_e20204: f64 = (locals.var_phi_j0_dep - locals.var_phi_j0_dep_old);
            let assign24600_body93_e20205: f64 = (assign24600_body93_e20204).abs();
            let assign24600_body93_e20208: f64 = if ((assign24600_body93_e20199 <= 1e-12) && (assign24600_body93_e20205 <= 1e-12)) { 1.0 } else { 0.0 };
            locals.var_guard573 = assign24600_body93_e20208;
            locals.var_guard573_rv = 0.0;
            let (assign24600_body94_e20220,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard559 != 0.0)) && (locals.var_guard573 != 0.0)) {
        let assign24600_body94_e20218: f64 = (150.0 + 1.0);
        (assign24600_body94_e20218,)
    } else {
        (locals.var_lp_s0,)
    }
};
            locals.var_lp_s0 = assign24600_body94_e20220;
            locals.var_lp_s0_rv = 0.0;
            let (assign24600_body95_e20228, assign24600_body95_e20228_d_n0, assign24600_body95_e20228_d_n2, assign24600_body95_e20228_d_n4, assign24600_body95_e20228_d_n5, assign24600_body95_e20228_d_n6, assign24600_body95_e20228_d_n7, assign24600_body95_e20228_d_n8, assign24600_body95_e20228_d_n9, assign24600_body95_e20228_d_n10, assign24600_body95_e20228_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard559 != 0.0)) {
        (locals.var_vgp0, locals.var_vgp0_dn0, locals.var_vgp0_dn2, locals.var_vgp0_dn4, locals.var_vgp0_dn5, locals.var_vgp0_dn6, locals.var_vgp0_dn7, locals.var_vgp0_dn8, locals.var_vgp0_dn9, locals.var_vgp0_dn10, locals.var_vgp0_dn13,)
    } else {
        (locals.var_vgp0old, locals.var_vgp0old_dn0, locals.var_vgp0old_dn2, locals.var_vgp0old_dn4, locals.var_vgp0old_dn5, locals.var_vgp0old_dn6, locals.var_vgp0old_dn7, locals.var_vgp0old_dn8, locals.var_vgp0old_dn9, locals.var_vgp0old_dn10, locals.var_vgp0old_dn13,)
    }
};
            locals.var_vgp0old = assign24600_body95_e20228;
            locals.var_vgp0old_dn0 = assign24600_body95_e20228_d_n0;
            locals.var_vgp0old_dn2 = assign24600_body95_e20228_d_n2;
            locals.var_vgp0old_dn4 = assign24600_body95_e20228_d_n4;
            locals.var_vgp0old_dn5 = assign24600_body95_e20228_d_n5;
            locals.var_vgp0old_dn6 = assign24600_body95_e20228_d_n6;
            locals.var_vgp0old_dn7 = assign24600_body95_e20228_d_n7;
            locals.var_vgp0old_dn8 = assign24600_body95_e20228_d_n8;
            locals.var_vgp0old_dn9 = assign24600_body95_e20228_d_n9;
            locals.var_vgp0old_dn10 = assign24600_body95_e20228_d_n10;
            locals.var_vgp0old_dn13 = assign24600_body95_e20228_d_n13;
            locals.var_vgp0old_rv = 0.0;
            let (assign24600_body96_e20236, assign24600_body96_e20236_d_n0, assign24600_body96_e20236_d_n2, assign24600_body96_e20236_d_n4, assign24600_body96_e20236_d_n5, assign24600_body96_e20236_d_n6, assign24600_body96_e20236_d_n7, assign24600_body96_e20236_d_n8, assign24600_body96_e20236_d_n9, assign24600_body96_e20236_d_n10, assign24600_body96_e20236_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard559 != 0.0)) {
        (locals.var_phi_j0_dep, locals.var_phi_j0_dep_dn0, locals.var_phi_j0_dep_dn2, locals.var_phi_j0_dep_dn4, locals.var_phi_j0_dep_dn5, locals.var_phi_j0_dep_dn6, locals.var_phi_j0_dep_dn7, locals.var_phi_j0_dep_dn8, locals.var_phi_j0_dep_dn9, locals.var_phi_j0_dep_dn10, locals.var_phi_j0_dep_dn13,)
    } else {
        (locals.var_phi_j0_dep_old, locals.var_phi_j0_dep_old_dn0, locals.var_phi_j0_dep_old_dn2, locals.var_phi_j0_dep_old_dn4, locals.var_phi_j0_dep_old_dn5, locals.var_phi_j0_dep_old_dn6, locals.var_phi_j0_dep_old_dn7, locals.var_phi_j0_dep_old_dn8, locals.var_phi_j0_dep_old_dn9, locals.var_phi_j0_dep_old_dn10, locals.var_phi_j0_dep_old_dn13,)
    }
};
            locals.var_phi_j0_dep_old = assign24600_body96_e20236;
            locals.var_phi_j0_dep_old_dn0 = assign24600_body96_e20236_d_n0;
            locals.var_phi_j0_dep_old_dn2 = assign24600_body96_e20236_d_n2;
            locals.var_phi_j0_dep_old_dn4 = assign24600_body96_e20236_d_n4;
            locals.var_phi_j0_dep_old_dn5 = assign24600_body96_e20236_d_n5;
            locals.var_phi_j0_dep_old_dn6 = assign24600_body96_e20236_d_n6;
            locals.var_phi_j0_dep_old_dn7 = assign24600_body96_e20236_d_n7;
            locals.var_phi_j0_dep_old_dn8 = assign24600_body96_e20236_d_n8;
            locals.var_phi_j0_dep_old_dn9 = assign24600_body96_e20236_d_n9;
            locals.var_phi_j0_dep_old_dn10 = assign24600_body96_e20236_d_n10;
            locals.var_phi_j0_dep_old_dn13 = assign24600_body96_e20236_d_n13;
            locals.var_phi_j0_dep_old_rv = 0.0;
            let (assign24600_body97_e20246,) = {
    if (((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard559 != 0.0)) {
        let assign24600_body97_e20244: f64 = (locals.var_lp_s0 + 1.0);
        (assign24600_body97_e20244,)
    } else {
        (locals.var_lp_s0,)
    }
};
            locals.var_lp_s0 = assign24600_body97_e20246;
            locals.var_lp_s0_rv = 0.0;
        }

    }

    pub(super) fn stamp_reactive_block_67(
        locals: &mut StampLocals,
    ) {
        let (assign24610_e20254, assign24610_e20254_d_n0, assign24610_e20254_d_n2, assign24610_e20254_d_n4, assign24610_e20254_d_n5, assign24610_e20254_d_n6, assign24610_e20254_d_n7, assign24610_e20254_d_n8, assign24610_e20254_d_n9, assign24610_e20254_d_n10, assign24610_e20254_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard559 != 0.0)) {
        (locals.var_phi_j0_dep, locals.var_phi_j0_dep_dn0, locals.var_phi_j0_dep_dn2, locals.var_phi_j0_dep_dn4, locals.var_phi_j0_dep_dn5, locals.var_phi_j0_dep_dn6, locals.var_phi_j0_dep_dn7, locals.var_phi_j0_dep_dn8, locals.var_phi_j0_dep_dn9, locals.var_phi_j0_dep_dn10, locals.var_phi_j0_dep_dn13,)
    } else {
        (locals.var_phi_j0_dep_acc, locals.var_phi_j0_dep_acc_dn0, locals.var_phi_j0_dep_acc_dn2, locals.var_phi_j0_dep_acc_dn4, locals.var_phi_j0_dep_acc_dn5, locals.var_phi_j0_dep_acc_dn6, locals.var_phi_j0_dep_acc_dn7, locals.var_phi_j0_dep_acc_dn8, locals.var_phi_j0_dep_acc_dn9, locals.var_phi_j0_dep_acc_dn10, locals.var_phi_j0_dep_acc_dn13,)
    }
};
        locals.var_phi_j0_dep_acc = assign24610_e20254;
        locals.var_phi_j0_dep_acc_dn0 = assign24610_e20254_d_n0;
        locals.var_phi_j0_dep_acc_dn2 = assign24610_e20254_d_n2;
        locals.var_phi_j0_dep_acc_dn4 = assign24610_e20254_d_n4;
        locals.var_phi_j0_dep_acc_dn5 = assign24610_e20254_d_n5;
        locals.var_phi_j0_dep_acc_dn6 = assign24610_e20254_d_n6;
        locals.var_phi_j0_dep_acc_dn7 = assign24610_e20254_d_n7;
        locals.var_phi_j0_dep_acc_dn8 = assign24610_e20254_d_n8;
        locals.var_phi_j0_dep_acc_dn9 = assign24610_e20254_d_n9;
        locals.var_phi_j0_dep_acc_dn10 = assign24610_e20254_d_n10;
        locals.var_phi_j0_dep_acc_dn13 = assign24610_e20254_d_n13;
        locals.var_phi_j0_dep_acc_rv = 0.0;

        let (assign24620_e20264, assign24620_e20264_d_n0, assign24620_e20264_d_n2, assign24620_e20264_d_n4, assign24620_e20264_d_n5, assign24620_e20264_d_n6, assign24620_e20264_d_n7, assign24620_e20264_d_n8, assign24620_e20264_d_n9, assign24620_e20264_d_n10, assign24620_e20264_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard559 != 0.0)) {
        let assign24620_e20262: f64 = (locals.var_uc_depthn * locals.var_ndepmpnsub);
        (assign24620_e20262, ((locals.var_uc_depthn_dn0 * locals.var_ndepmpnsub) + (locals.var_uc_depthn * locals.var_ndepmpnsub_dn0)), ((locals.var_uc_depthn_dn2 * locals.var_ndepmpnsub) + (locals.var_uc_depthn * locals.var_ndepmpnsub_dn2)), ((locals.var_uc_depthn_dn4 * locals.var_ndepmpnsub) + (locals.var_uc_depthn * locals.var_ndepmpnsub_dn4)), ((locals.var_uc_depthn_dn5 * locals.var_ndepmpnsub) + (locals.var_uc_depthn * locals.var_ndepmpnsub_dn5)), ((locals.var_uc_depthn_dn6 * locals.var_ndepmpnsub) + (locals.var_uc_depthn * locals.var_ndepmpnsub_dn6)), ((locals.var_uc_depthn_dn7 * locals.var_ndepmpnsub) + (locals.var_uc_depthn * locals.var_ndepmpnsub_dn7)), ((locals.var_uc_depthn_dn8 * locals.var_ndepmpnsub) + (locals.var_uc_depthn * locals.var_ndepmpnsub_dn8)), ((locals.var_uc_depthn_dn9 * locals.var_ndepmpnsub) + (locals.var_uc_depthn * locals.var_ndepmpnsub_dn9)), ((locals.var_uc_depthn_dn10 * locals.var_ndepmpnsub) + (locals.var_uc_depthn * locals.var_ndepmpnsub_dn10)), ((locals.var_uc_depthn_dn13 * locals.var_ndepmpnsub) + (locals.var_uc_depthn * locals.var_ndepmpnsub_dn13)),)
    } else {
        (locals.var_w_sub0, locals.var_w_sub0_dn0, locals.var_w_sub0_dn2, locals.var_w_sub0_dn4, locals.var_w_sub0_dn5, locals.var_w_sub0_dn6, locals.var_w_sub0_dn7, locals.var_w_sub0_dn8, locals.var_w_sub0_dn9, locals.var_w_sub0_dn10, locals.var_w_sub0_dn13,)
    }
};
        locals.var_w_sub0 = assign24620_e20264;
        locals.var_w_sub0_dn0 = assign24620_e20264_d_n0;
        locals.var_w_sub0_dn2 = assign24620_e20264_d_n2;
        locals.var_w_sub0_dn4 = assign24620_e20264_d_n4;
        locals.var_w_sub0_dn5 = assign24620_e20264_d_n5;
        locals.var_w_sub0_dn6 = assign24620_e20264_d_n6;
        locals.var_w_sub0_dn7 = assign24620_e20264_d_n7;
        locals.var_w_sub0_dn8 = assign24620_e20264_d_n8;
        locals.var_w_sub0_dn9 = assign24620_e20264_d_n9;
        locals.var_w_sub0_dn10 = assign24620_e20264_d_n10;
        locals.var_w_sub0_dn13 = assign24620_e20264_d_n13;
        locals.var_w_sub0_rv = 0.0;

        let (assign24630_e20280, assign24630_e20280_d_n0, assign24630_e20280_d_n2, assign24630_e20280_d_n4, assign24630_e20280_d_n5, assign24630_e20280_d_n6, assign24630_e20280_d_n7, assign24630_e20280_d_n8, assign24630_e20280_d_n9, assign24630_e20280_d_n10, assign24630_e20280_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard559 != 0.0)) {
        let assign24630_e20272: f64 = (locals.var_c_2esipq_nsub_inv * locals.var_w_sub0);
        let assign24630_e20274: f64 = (assign24630_e20272 * locals.var_w_sub0);
        let assign24630_e20276: f64 = (assign24630_e20274 + locals.var_vbscl__blk435);
        let assign24630_e20278: f64 = (assign24630_e20276 - locals.var_vbi_dep);
        (assign24630_e20278, ((((((locals.var_c_2esipq_nsub_inv_dn0 * locals.var_w_sub0) + (locals.var_c_2esipq_nsub_inv * locals.var_w_sub0_dn0)) * locals.var_w_sub0) + (assign24630_e20272 * locals.var_w_sub0_dn0)) + locals.var_vbscl__blk435_dn0) - locals.var_vbi_dep_dn0), ((((((locals.var_c_2esipq_nsub_inv_dn2 * locals.var_w_sub0) + (locals.var_c_2esipq_nsub_inv * locals.var_w_sub0_dn2)) * locals.var_w_sub0) + (assign24630_e20272 * locals.var_w_sub0_dn2)) + locals.var_vbscl__blk435_dn2) - locals.var_vbi_dep_dn2), ((((((locals.var_c_2esipq_nsub_inv_dn4 * locals.var_w_sub0) + (locals.var_c_2esipq_nsub_inv * locals.var_w_sub0_dn4)) * locals.var_w_sub0) + (assign24630_e20272 * locals.var_w_sub0_dn4)) + locals.var_vbscl__blk435_dn4) - locals.var_vbi_dep_dn4), ((((((locals.var_c_2esipq_nsub_inv_dn5 * locals.var_w_sub0) + (locals.var_c_2esipq_nsub_inv * locals.var_w_sub0_dn5)) * locals.var_w_sub0) + (assign24630_e20272 * locals.var_w_sub0_dn5)) + locals.var_vbscl__blk435_dn5) - locals.var_vbi_dep_dn5), ((((((locals.var_c_2esipq_nsub_inv_dn6 * locals.var_w_sub0) + (locals.var_c_2esipq_nsub_inv * locals.var_w_sub0_dn6)) * locals.var_w_sub0) + (assign24630_e20272 * locals.var_w_sub0_dn6)) + locals.var_vbscl__blk435_dn6) - locals.var_vbi_dep_dn6), ((((((locals.var_c_2esipq_nsub_inv_dn7 * locals.var_w_sub0) + (locals.var_c_2esipq_nsub_inv * locals.var_w_sub0_dn7)) * locals.var_w_sub0) + (assign24630_e20272 * locals.var_w_sub0_dn7)) + locals.var_vbscl__blk435_dn7) - locals.var_vbi_dep_dn7), ((((((locals.var_c_2esipq_nsub_inv_dn8 * locals.var_w_sub0) + (locals.var_c_2esipq_nsub_inv * locals.var_w_sub0_dn8)) * locals.var_w_sub0) + (assign24630_e20272 * locals.var_w_sub0_dn8)) + locals.var_vbscl__blk435_dn8) - locals.var_vbi_dep_dn8), ((((((locals.var_c_2esipq_nsub_inv_dn9 * locals.var_w_sub0) + (locals.var_c_2esipq_nsub_inv * locals.var_w_sub0_dn9)) * locals.var_w_sub0) + (assign24630_e20272 * locals.var_w_sub0_dn9)) + locals.var_vbscl__blk435_dn9) - locals.var_vbi_dep_dn9), ((((((locals.var_c_2esipq_nsub_inv_dn10 * locals.var_w_sub0) + (locals.var_c_2esipq_nsub_inv * locals.var_w_sub0_dn10)) * locals.var_w_sub0) + (assign24630_e20272 * locals.var_w_sub0_dn10)) + locals.var_vbscl__blk435_dn10) - locals.var_vbi_dep_dn10), ((((((locals.var_c_2esipq_nsub_inv_dn13 * locals.var_w_sub0) + (locals.var_c_2esipq_nsub_inv * locals.var_w_sub0_dn13)) * locals.var_w_sub0) + (assign24630_e20272 * locals.var_w_sub0_dn13)) + locals.var_vbscl__blk435_dn13) - locals.var_vbi_dep_dn13),)
    } else {
        (locals.var_phi_j0_dep, locals.var_phi_j0_dep_dn0, locals.var_phi_j0_dep_dn2, locals.var_phi_j0_dep_dn4, locals.var_phi_j0_dep_dn5, locals.var_phi_j0_dep_dn6, locals.var_phi_j0_dep_dn7, locals.var_phi_j0_dep_dn8, locals.var_phi_j0_dep_dn9, locals.var_phi_j0_dep_dn10, locals.var_phi_j0_dep_dn13,)
    }
};
        locals.var_phi_j0_dep = assign24630_e20280;
        locals.var_phi_j0_dep_dn0 = assign24630_e20280_d_n0;
        locals.var_phi_j0_dep_dn2 = assign24630_e20280_d_n2;
        locals.var_phi_j0_dep_dn4 = assign24630_e20280_d_n4;
        locals.var_phi_j0_dep_dn5 = assign24630_e20280_d_n5;
        locals.var_phi_j0_dep_dn6 = assign24630_e20280_d_n6;
        locals.var_phi_j0_dep_dn7 = assign24630_e20280_d_n7;
        locals.var_phi_j0_dep_dn8 = assign24630_e20280_d_n8;
        locals.var_phi_j0_dep_dn9 = assign24630_e20280_d_n9;
        locals.var_phi_j0_dep_dn10 = assign24630_e20280_d_n10;
        locals.var_phi_j0_dep_dn13 = assign24630_e20280_d_n13;
        locals.var_phi_j0_dep_rv = 0.0;

        let (assign24640_e20292, assign24640_e20292_d_n0, assign24640_e20292_d_n2, assign24640_e20292_d_n4, assign24640_e20292_d_n5, assign24640_e20292_d_n6, assign24640_e20292_d_n7, assign24640_e20292_d_n8, assign24640_e20292_d_n9, assign24640_e20292_d_n10, assign24640_e20292_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard559 != 0.0)) {
        let assign24640_e20289: f64 = (locals.var_c_2esipq_ndepm_inv * locals.var_tn2);
        let assign24640_e20290: f64 = (locals.var_phi_j0_dep + assign24640_e20289);
        (assign24640_e20290, (locals.var_phi_j0_dep_dn0 + ((locals.var_c_2esipq_ndepm_inv_dn0 * locals.var_tn2) + (locals.var_c_2esipq_ndepm_inv * locals.var_tn2_dn0))), (locals.var_phi_j0_dep_dn2 + ((locals.var_c_2esipq_ndepm_inv_dn2 * locals.var_tn2) + (locals.var_c_2esipq_ndepm_inv * locals.var_tn2_dn2))), (locals.var_phi_j0_dep_dn4 + ((locals.var_c_2esipq_ndepm_inv_dn4 * locals.var_tn2) + (locals.var_c_2esipq_ndepm_inv * locals.var_tn2_dn4))), (locals.var_phi_j0_dep_dn5 + ((locals.var_c_2esipq_ndepm_inv_dn5 * locals.var_tn2) + (locals.var_c_2esipq_ndepm_inv * locals.var_tn2_dn5))), (locals.var_phi_j0_dep_dn6 + ((locals.var_c_2esipq_ndepm_inv_dn6 * locals.var_tn2) + (locals.var_c_2esipq_ndepm_inv * locals.var_tn2_dn6))), (locals.var_phi_j0_dep_dn7 + ((locals.var_c_2esipq_ndepm_inv_dn7 * locals.var_tn2) + (locals.var_c_2esipq_ndepm_inv * locals.var_tn2_dn7))), (locals.var_phi_j0_dep_dn8 + ((locals.var_c_2esipq_ndepm_inv_dn8 * locals.var_tn2) + (locals.var_c_2esipq_ndepm_inv * locals.var_tn2_dn8))), (locals.var_phi_j0_dep_dn9 + ((locals.var_c_2esipq_ndepm_inv_dn9 * locals.var_tn2) + (locals.var_c_2esipq_ndepm_inv * locals.var_tn2_dn9))), (locals.var_phi_j0_dep_dn10 + ((locals.var_c_2esipq_ndepm_inv_dn10 * locals.var_tn2) + (locals.var_c_2esipq_ndepm_inv * locals.var_tn2_dn10))), (locals.var_phi_j0_dep_dn13 + ((locals.var_c_2esipq_ndepm_inv_dn13 * locals.var_tn2) + (locals.var_c_2esipq_ndepm_inv * locals.var_tn2_dn13))),)
    } else {
        (locals.var_phi_b0_dep, locals.var_phi_b0_dep_dn0, locals.var_phi_b0_dep_dn2, locals.var_phi_b0_dep_dn4, locals.var_phi_b0_dep_dn5, locals.var_phi_b0_dep_dn6, locals.var_phi_b0_dep_dn7, locals.var_phi_b0_dep_dn8, locals.var_phi_b0_dep_dn9, locals.var_phi_b0_dep_dn10, locals.var_phi_b0_dep_dn13,)
    }
};
        locals.var_phi_b0_dep = assign24640_e20292;
        locals.var_phi_b0_dep_dn0 = assign24640_e20292_d_n0;
        locals.var_phi_b0_dep_dn2 = assign24640_e20292_d_n2;
        locals.var_phi_b0_dep_dn4 = assign24640_e20292_d_n4;
        locals.var_phi_b0_dep_dn5 = assign24640_e20292_d_n5;
        locals.var_phi_b0_dep_dn6 = assign24640_e20292_d_n6;
        locals.var_phi_b0_dep_dn7 = assign24640_e20292_d_n7;
        locals.var_phi_b0_dep_dn8 = assign24640_e20292_d_n8;
        locals.var_phi_b0_dep_dn9 = assign24640_e20292_d_n9;
        locals.var_phi_b0_dep_dn10 = assign24640_e20292_d_n10;
        locals.var_phi_b0_dep_dn13 = assign24640_e20292_d_n13;
        locals.var_phi_b0_dep_rv = 0.0;

        let (assign24650_e20300, assign24650_e20300_d_n0, assign24650_e20300_d_n2, assign24650_e20300_d_n4, assign24650_e20300_d_n5, assign24650_e20300_d_n6, assign24650_e20300_d_n7, assign24650_e20300_d_n8, assign24650_e20300_d_n9, assign24650_e20300_d_n10, assign24650_e20300_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard559 != 0.0)) {
        (locals.var_phi_b0_dep, locals.var_phi_b0_dep_dn0, locals.var_phi_b0_dep_dn2, locals.var_phi_b0_dep_dn4, locals.var_phi_b0_dep_dn5, locals.var_phi_b0_dep_dn6, locals.var_phi_b0_dep_dn7, locals.var_phi_b0_dep_dn8, locals.var_phi_b0_dep_dn9, locals.var_phi_b0_dep_dn10, locals.var_phi_b0_dep_dn13,)
    } else {
        (locals.var_phi_s0_dep, locals.var_phi_s0_dep_dn0, locals.var_phi_s0_dep_dn2, locals.var_phi_s0_dep_dn4, locals.var_phi_s0_dep_dn5, locals.var_phi_s0_dep_dn6, locals.var_phi_s0_dep_dn7, locals.var_phi_s0_dep_dn8, locals.var_phi_s0_dep_dn9, locals.var_phi_s0_dep_dn10, locals.var_phi_s0_dep_dn13,)
    }
};
        locals.var_phi_s0_dep = assign24650_e20300;
        locals.var_phi_s0_dep_dn0 = assign24650_e20300_d_n0;
        locals.var_phi_s0_dep_dn2 = assign24650_e20300_d_n2;
        locals.var_phi_s0_dep_dn4 = assign24650_e20300_d_n4;
        locals.var_phi_s0_dep_dn5 = assign24650_e20300_d_n5;
        locals.var_phi_s0_dep_dn6 = assign24650_e20300_d_n6;
        locals.var_phi_s0_dep_dn7 = assign24650_e20300_d_n7;
        locals.var_phi_s0_dep_dn8 = assign24650_e20300_d_n8;
        locals.var_phi_s0_dep_dn9 = assign24650_e20300_d_n9;
        locals.var_phi_s0_dep_dn10 = assign24650_e20300_d_n10;
        locals.var_phi_s0_dep_dn13 = assign24650_e20300_d_n13;
        locals.var_phi_s0_dep_rv = 0.0;

        let (assign24660_e20308, assign24660_e20308_d_n0, assign24660_e20308_d_n2, assign24660_e20308_d_n4, assign24660_e20308_d_n5, assign24660_e20308_d_n6, assign24660_e20308_d_n7, assign24660_e20308_d_n8, assign24660_e20308_d_n9, assign24660_e20308_d_n10, assign24660_e20308_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard559 != 0.0)) {
        (locals.var_phi_b0_dep, locals.var_phi_b0_dep_dn0, locals.var_phi_b0_dep_dn2, locals.var_phi_b0_dep_dn4, locals.var_phi_b0_dep_dn5, locals.var_phi_b0_dep_dn6, locals.var_phi_b0_dep_dn7, locals.var_phi_b0_dep_dn8, locals.var_phi_b0_dep_dn9, locals.var_phi_b0_dep_dn10, locals.var_phi_b0_dep_dn13,)
    } else {
        (locals.var_psbmax, locals.var_psbmax_dn0, locals.var_psbmax_dn2, locals.var_psbmax_dn4, locals.var_psbmax_dn5, locals.var_psbmax_dn6, locals.var_psbmax_dn7, locals.var_psbmax_dn8, locals.var_psbmax_dn9, locals.var_psbmax_dn10, locals.var_psbmax_dn13,)
    }
};
        locals.var_psbmax = assign24660_e20308;
        locals.var_psbmax_dn0 = assign24660_e20308_d_n0;
        locals.var_psbmax_dn2 = assign24660_e20308_d_n2;
        locals.var_psbmax_dn4 = assign24660_e20308_d_n4;
        locals.var_psbmax_dn5 = assign24660_e20308_d_n5;
        locals.var_psbmax_dn6 = assign24660_e20308_d_n6;
        locals.var_psbmax_dn7 = assign24660_e20308_d_n7;
        locals.var_psbmax_dn8 = assign24660_e20308_d_n8;
        locals.var_psbmax_dn9 = assign24660_e20308_d_n9;
        locals.var_psbmax_dn10 = assign24660_e20308_d_n10;
        locals.var_psbmax_dn13 = assign24660_e20308_d_n13;
        locals.var_psbmax_rv = 0.0;

        let (assign24670_e20316, assign24670_e20316_d_n0, assign24670_e20316_d_n2, assign24670_e20316_d_n4, assign24670_e20316_d_n5, assign24670_e20316_d_n6, assign24670_e20316_d_n7, assign24670_e20316_d_n8, assign24670_e20316_d_n9, assign24670_e20316_d_n10, assign24670_e20316_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard559 != 0.0)) {
        (locals.var_phi_b0_dep, locals.var_phi_b0_dep_dn0, locals.var_phi_b0_dep_dn2, locals.var_phi_b0_dep_dn4, locals.var_phi_b0_dep_dn5, locals.var_phi_b0_dep_dn6, locals.var_phi_b0_dep_dn7, locals.var_phi_b0_dep_dn8, locals.var_phi_b0_dep_dn9, locals.var_phi_b0_dep_dn10, locals.var_phi_b0_dep_dn13,)
    } else {
        (locals.var_vgp1, locals.var_vgp1_dn0, locals.var_vgp1_dn2, locals.var_vgp1_dn4, locals.var_vgp1_dn5, locals.var_vgp1_dn6, locals.var_vgp1_dn7, locals.var_vgp1_dn8, locals.var_vgp1_dn9, locals.var_vgp1_dn10, locals.var_vgp1_dn13,)
    }
};
        locals.var_vgp1 = assign24670_e20316;
        locals.var_vgp1_dn0 = assign24670_e20316_d_n0;
        locals.var_vgp1_dn2 = assign24670_e20316_d_n2;
        locals.var_vgp1_dn4 = assign24670_e20316_d_n4;
        locals.var_vgp1_dn5 = assign24670_e20316_d_n5;
        locals.var_vgp1_dn6 = assign24670_e20316_d_n6;
        locals.var_vgp1_dn7 = assign24670_e20316_d_n7;
        locals.var_vgp1_dn8 = assign24670_e20316_d_n8;
        locals.var_vgp1_dn9 = assign24670_e20316_d_n9;
        locals.var_vgp1_dn10 = assign24670_e20316_d_n10;
        locals.var_vgp1_dn13 = assign24670_e20316_d_n13;
        locals.var_vgp1_rv = 0.0;

        let assign24680_e20319: f64 = if locals.var_vgp > locals.var_vgp0 { 1.0 } else { 0.0 };
        locals.var_guard574 = assign24680_e20319;
        locals.var_guard574_rv = 0.0;

        let (assign24690_e20329,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard559 != 0.0)) && (locals.var_guard574 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_depmode,)
    }
};
        locals.var_depmode = assign24690_e20329;
        locals.var_depmode_rv = 0.0;

        let assign24700_e20332: f64 = if locals.var_vgp > locals.var_vgp1 { 1.0 } else { 0.0 };
        locals.var_guard575 = assign24700_e20332;
        locals.var_guard575_rv = 0.0;

        let (assign24710_e20345,) = {
    if (((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard559 != 0.0)) && (locals.var_guard574 == 0.0)) && (locals.var_guard575 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_depmode,)
    }
};
        locals.var_depmode = assign24710_e20345;
        locals.var_depmode_rv = 0.0;

        let (assign24720_e20359,) = {
    if (((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard559 != 0.0)) && (locals.var_guard574 == 0.0)) && (locals.var_guard575 == 0.0)) {
        (2.0,)
    } else {
        (locals.var_depmode,)
    }
};
        locals.var_depmode = assign24720_e20359;
        locals.var_depmode_rv = 0.0;

        let (assign24730_e20368, assign24730_e20368_d_n0, assign24730_e20368_d_n2, assign24730_e20368_d_n4, assign24730_e20368_d_n5, assign24730_e20368_d_n6, assign24730_e20368_d_n7, assign24730_e20368_d_n8, assign24730_e20368_d_n9, assign24730_e20368_d_n10, assign24730_e20368_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard559 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vgp0, locals.var_vgp0_dn0, locals.var_vgp0_dn2, locals.var_vgp0_dn4, locals.var_vgp0_dn5, locals.var_vgp0_dn6, locals.var_vgp0_dn7, locals.var_vgp0_dn8, locals.var_vgp0_dn9, locals.var_vgp0_dn10, locals.var_vgp0_dn13,)
    }
};
        locals.var_vgp0 = assign24730_e20368;
        locals.var_vgp0_dn0 = assign24730_e20368_d_n0;
        locals.var_vgp0_dn2 = assign24730_e20368_d_n2;
        locals.var_vgp0_dn4 = assign24730_e20368_d_n4;
        locals.var_vgp0_dn5 = assign24730_e20368_d_n5;
        locals.var_vgp0_dn6 = assign24730_e20368_d_n6;
        locals.var_vgp0_dn7 = assign24730_e20368_d_n7;
        locals.var_vgp0_dn8 = assign24730_e20368_d_n8;
        locals.var_vgp0_dn9 = assign24730_e20368_d_n9;
        locals.var_vgp0_dn10 = assign24730_e20368_d_n10;
        locals.var_vgp0_dn13 = assign24730_e20368_d_n13;
        locals.var_vgp0_rv = 0.0;

        let (assign24740_e20377, assign24740_e20377_d_n0, assign24740_e20377_d_n2, assign24740_e20377_d_n4, assign24740_e20377_d_n5, assign24740_e20377_d_n6, assign24740_e20377_d_n7, assign24740_e20377_d_n8, assign24740_e20377_d_n9, assign24740_e20377_d_n10, assign24740_e20377_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard559 == 0.0)) {
        (locals.var_vgp0, locals.var_vgp0_dn0, locals.var_vgp0_dn2, locals.var_vgp0_dn4, locals.var_vgp0_dn5, locals.var_vgp0_dn6, locals.var_vgp0_dn7, locals.var_vgp0_dn8, locals.var_vgp0_dn9, locals.var_vgp0_dn10, locals.var_vgp0_dn13,)
    } else {
        (locals.var_vgp1, locals.var_vgp1_dn0, locals.var_vgp1_dn2, locals.var_vgp1_dn4, locals.var_vgp1_dn5, locals.var_vgp1_dn6, locals.var_vgp1_dn7, locals.var_vgp1_dn8, locals.var_vgp1_dn9, locals.var_vgp1_dn10, locals.var_vgp1_dn13,)
    }
};
        locals.var_vgp1 = assign24740_e20377;
        locals.var_vgp1_dn0 = assign24740_e20377_d_n0;
        locals.var_vgp1_dn2 = assign24740_e20377_d_n2;
        locals.var_vgp1_dn4 = assign24740_e20377_d_n4;
        locals.var_vgp1_dn5 = assign24740_e20377_d_n5;
        locals.var_vgp1_dn6 = assign24740_e20377_d_n6;
        locals.var_vgp1_dn7 = assign24740_e20377_d_n7;
        locals.var_vgp1_dn8 = assign24740_e20377_d_n8;
        locals.var_vgp1_dn9 = assign24740_e20377_d_n9;
        locals.var_vgp1_dn10 = assign24740_e20377_d_n10;
        locals.var_vgp1_dn13 = assign24740_e20377_d_n13;
        locals.var_vgp1_rv = 0.0;

        let (assign24750_e20386, assign24750_e20386_d_n0, assign24750_e20386_d_n2, assign24750_e20386_d_n4, assign24750_e20386_d_n5, assign24750_e20386_d_n6, assign24750_e20386_d_n7, assign24750_e20386_d_n8, assign24750_e20386_d_n9, assign24750_e20386_d_n10, assign24750_e20386_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard559 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_psbmax, locals.var_psbmax_dn0, locals.var_psbmax_dn2, locals.var_psbmax_dn4, locals.var_psbmax_dn5, locals.var_psbmax_dn6, locals.var_psbmax_dn7, locals.var_psbmax_dn8, locals.var_psbmax_dn9, locals.var_psbmax_dn10, locals.var_psbmax_dn13,)
    }
};
        locals.var_psbmax = assign24750_e20386;
        locals.var_psbmax_dn0 = assign24750_e20386_d_n0;
        locals.var_psbmax_dn2 = assign24750_e20386_d_n2;
        locals.var_psbmax_dn4 = assign24750_e20386_d_n4;
        locals.var_psbmax_dn5 = assign24750_e20386_d_n5;
        locals.var_psbmax_dn6 = assign24750_e20386_d_n6;
        locals.var_psbmax_dn7 = assign24750_e20386_d_n7;
        locals.var_psbmax_dn8 = assign24750_e20386_d_n8;
        locals.var_psbmax_dn9 = assign24750_e20386_d_n9;
        locals.var_psbmax_dn10 = assign24750_e20386_d_n10;
        locals.var_psbmax_dn13 = assign24750_e20386_d_n13;
        locals.var_psbmax_rv = 0.0;

        let (assign24760_e20395, assign24760_e20395_d_n0, assign24760_e20395_d_n2, assign24760_e20395_d_n4, assign24760_e20395_d_n5, assign24760_e20395_d_n6, assign24760_e20395_d_n7, assign24760_e20395_d_n8, assign24760_e20395_d_n9, assign24760_e20395_d_n10, assign24760_e20395_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard559 == 0.0)) {
        (locals.var_vgp0, locals.var_vgp0_dn0, locals.var_vgp0_dn2, locals.var_vgp0_dn4, locals.var_vgp0_dn5, locals.var_vgp0_dn6, locals.var_vgp0_dn7, locals.var_vgp0_dn8, locals.var_vgp0_dn9, locals.var_vgp0_dn10, locals.var_vgp0_dn13,)
    } else {
        (locals.var_vds_maxb0, locals.var_vds_maxb0_dn0, locals.var_vds_maxb0_dn2, locals.var_vds_maxb0_dn4, locals.var_vds_maxb0_dn5, locals.var_vds_maxb0_dn6, locals.var_vds_maxb0_dn7, locals.var_vds_maxb0_dn8, locals.var_vds_maxb0_dn9, locals.var_vds_maxb0_dn10, locals.var_vds_maxb0_dn13,)
    }
};
        locals.var_vds_maxb0 = assign24760_e20395;
        locals.var_vds_maxb0_dn0 = assign24760_e20395_d_n0;
        locals.var_vds_maxb0_dn2 = assign24760_e20395_d_n2;
        locals.var_vds_maxb0_dn4 = assign24760_e20395_d_n4;
        locals.var_vds_maxb0_dn5 = assign24760_e20395_d_n5;
        locals.var_vds_maxb0_dn6 = assign24760_e20395_d_n6;
        locals.var_vds_maxb0_dn7 = assign24760_e20395_d_n7;
        locals.var_vds_maxb0_dn8 = assign24760_e20395_d_n8;
        locals.var_vds_maxb0_dn9 = assign24760_e20395_d_n9;
        locals.var_vds_maxb0_dn10 = assign24760_e20395_d_n10;
        locals.var_vds_maxb0_dn13 = assign24760_e20395_d_n13;
        locals.var_vds_maxb0_rv = 0.0;

        let (assign24770_e20404, assign24770_e20404_d_n0, assign24770_e20404_d_n2, assign24770_e20404_d_n4, assign24770_e20404_d_n5, assign24770_e20404_d_n6, assign24770_e20404_d_n7, assign24770_e20404_d_n8, assign24770_e20404_d_n9, assign24770_e20404_d_n10, assign24770_e20404_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard559 == 0.0)) {
        (locals.var_w_bsub0, locals.var_w_bsub0_dn0, locals.var_w_bsub0_dn2, locals.var_w_bsub0_dn4, locals.var_w_bsub0_dn5, locals.var_w_bsub0_dn6, locals.var_w_bsub0_dn7, locals.var_w_bsub0_dn8, locals.var_w_bsub0_dn9, locals.var_w_bsub0_dn10, locals.var_w_bsub0_dn13,)
    } else {
        (locals.var_w_b0, locals.var_w_b0_dn0, locals.var_w_b0_dn2, locals.var_w_b0_dn4, locals.var_w_b0_dn5, locals.var_w_b0_dn6, locals.var_w_b0_dn7, locals.var_w_b0_dn8, locals.var_w_b0_dn9, locals.var_w_b0_dn10, locals.var_w_b0_dn13,)
    }
};
        locals.var_w_b0 = assign24770_e20404;
        locals.var_w_b0_dn0 = assign24770_e20404_d_n0;
        locals.var_w_b0_dn2 = assign24770_e20404_d_n2;
        locals.var_w_b0_dn4 = assign24770_e20404_d_n4;
        locals.var_w_b0_dn5 = assign24770_e20404_d_n5;
        locals.var_w_b0_dn6 = assign24770_e20404_d_n6;
        locals.var_w_b0_dn7 = assign24770_e20404_d_n7;
        locals.var_w_b0_dn8 = assign24770_e20404_d_n8;
        locals.var_w_b0_dn9 = assign24770_e20404_d_n9;
        locals.var_w_b0_dn10 = assign24770_e20404_d_n10;
        locals.var_w_b0_dn13 = assign24770_e20404_d_n13;
        locals.var_w_b0_rv = 0.0;

        let (assign24780_e20415, assign24780_e20415_d_n0, assign24780_e20415_d_n2, assign24780_e20415_d_n4, assign24780_e20415_d_n5, assign24780_e20415_d_n6, assign24780_e20415_d_n7, assign24780_e20415_d_n8, assign24780_e20415_d_n9, assign24780_e20415_d_n10, assign24780_e20415_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard559 == 0.0)) {
        let assign24780_e20413: f64 = (locals.var_w_b0 * locals.var_ndepmpnsub);
        (assign24780_e20413, ((locals.var_w_b0_dn0 * locals.var_ndepmpnsub) + (locals.var_w_b0 * locals.var_ndepmpnsub_dn0)), ((locals.var_w_b0_dn2 * locals.var_ndepmpnsub) + (locals.var_w_b0 * locals.var_ndepmpnsub_dn2)), ((locals.var_w_b0_dn4 * locals.var_ndepmpnsub) + (locals.var_w_b0 * locals.var_ndepmpnsub_dn4)), ((locals.var_w_b0_dn5 * locals.var_ndepmpnsub) + (locals.var_w_b0 * locals.var_ndepmpnsub_dn5)), ((locals.var_w_b0_dn6 * locals.var_ndepmpnsub) + (locals.var_w_b0 * locals.var_ndepmpnsub_dn6)), ((locals.var_w_b0_dn7 * locals.var_ndepmpnsub) + (locals.var_w_b0 * locals.var_ndepmpnsub_dn7)), ((locals.var_w_b0_dn8 * locals.var_ndepmpnsub) + (locals.var_w_b0 * locals.var_ndepmpnsub_dn8)), ((locals.var_w_b0_dn9 * locals.var_ndepmpnsub) + (locals.var_w_b0 * locals.var_ndepmpnsub_dn9)), ((locals.var_w_b0_dn10 * locals.var_ndepmpnsub) + (locals.var_w_b0 * locals.var_ndepmpnsub_dn10)), ((locals.var_w_b0_dn13 * locals.var_ndepmpnsub) + (locals.var_w_b0 * locals.var_ndepmpnsub_dn13)),)
    } else {
        (locals.var_w_sub0, locals.var_w_sub0_dn0, locals.var_w_sub0_dn2, locals.var_w_sub0_dn4, locals.var_w_sub0_dn5, locals.var_w_sub0_dn6, locals.var_w_sub0_dn7, locals.var_w_sub0_dn8, locals.var_w_sub0_dn9, locals.var_w_sub0_dn10, locals.var_w_sub0_dn13,)
    }
};
        locals.var_w_sub0 = assign24780_e20415;
        locals.var_w_sub0_dn0 = assign24780_e20415_d_n0;
        locals.var_w_sub0_dn2 = assign24780_e20415_d_n2;
        locals.var_w_sub0_dn4 = assign24780_e20415_d_n4;
        locals.var_w_sub0_dn5 = assign24780_e20415_d_n5;
        locals.var_w_sub0_dn6 = assign24780_e20415_d_n6;
        locals.var_w_sub0_dn7 = assign24780_e20415_d_n7;
        locals.var_w_sub0_dn8 = assign24780_e20415_d_n8;
        locals.var_w_sub0_dn9 = assign24780_e20415_d_n9;
        locals.var_w_sub0_dn10 = assign24780_e20415_d_n10;
        locals.var_w_sub0_dn13 = assign24780_e20415_d_n13;
        locals.var_w_sub0_rv = 0.0;

        let (assign24790_e20432, assign24790_e20432_d_n0, assign24790_e20432_d_n2, assign24790_e20432_d_n4, assign24790_e20432_d_n5, assign24790_e20432_d_n6, assign24790_e20432_d_n7, assign24790_e20432_d_n8, assign24790_e20432_d_n9, assign24790_e20432_d_n10, assign24790_e20432_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard559 == 0.0)) {
        let assign24790_e20424: f64 = (locals.var_c_2esipq_nsub_inv * locals.var_w_sub0);
        let assign24790_e20426: f64 = (assign24790_e20424 * locals.var_w_sub0);
        let assign24790_e20428: f64 = (assign24790_e20426 + locals.var_vbscl__blk435);
        let assign24790_e20430: f64 = (assign24790_e20428 - locals.var_vbi_dep);
        (assign24790_e20430, ((((((locals.var_c_2esipq_nsub_inv_dn0 * locals.var_w_sub0) + (locals.var_c_2esipq_nsub_inv * locals.var_w_sub0_dn0)) * locals.var_w_sub0) + (assign24790_e20424 * locals.var_w_sub0_dn0)) + locals.var_vbscl__blk435_dn0) - locals.var_vbi_dep_dn0), ((((((locals.var_c_2esipq_nsub_inv_dn2 * locals.var_w_sub0) + (locals.var_c_2esipq_nsub_inv * locals.var_w_sub0_dn2)) * locals.var_w_sub0) + (assign24790_e20424 * locals.var_w_sub0_dn2)) + locals.var_vbscl__blk435_dn2) - locals.var_vbi_dep_dn2), ((((((locals.var_c_2esipq_nsub_inv_dn4 * locals.var_w_sub0) + (locals.var_c_2esipq_nsub_inv * locals.var_w_sub0_dn4)) * locals.var_w_sub0) + (assign24790_e20424 * locals.var_w_sub0_dn4)) + locals.var_vbscl__blk435_dn4) - locals.var_vbi_dep_dn4), ((((((locals.var_c_2esipq_nsub_inv_dn5 * locals.var_w_sub0) + (locals.var_c_2esipq_nsub_inv * locals.var_w_sub0_dn5)) * locals.var_w_sub0) + (assign24790_e20424 * locals.var_w_sub0_dn5)) + locals.var_vbscl__blk435_dn5) - locals.var_vbi_dep_dn5), ((((((locals.var_c_2esipq_nsub_inv_dn6 * locals.var_w_sub0) + (locals.var_c_2esipq_nsub_inv * locals.var_w_sub0_dn6)) * locals.var_w_sub0) + (assign24790_e20424 * locals.var_w_sub0_dn6)) + locals.var_vbscl__blk435_dn6) - locals.var_vbi_dep_dn6), ((((((locals.var_c_2esipq_nsub_inv_dn7 * locals.var_w_sub0) + (locals.var_c_2esipq_nsub_inv * locals.var_w_sub0_dn7)) * locals.var_w_sub0) + (assign24790_e20424 * locals.var_w_sub0_dn7)) + locals.var_vbscl__blk435_dn7) - locals.var_vbi_dep_dn7), ((((((locals.var_c_2esipq_nsub_inv_dn8 * locals.var_w_sub0) + (locals.var_c_2esipq_nsub_inv * locals.var_w_sub0_dn8)) * locals.var_w_sub0) + (assign24790_e20424 * locals.var_w_sub0_dn8)) + locals.var_vbscl__blk435_dn8) - locals.var_vbi_dep_dn8), ((((((locals.var_c_2esipq_nsub_inv_dn9 * locals.var_w_sub0) + (locals.var_c_2esipq_nsub_inv * locals.var_w_sub0_dn9)) * locals.var_w_sub0) + (assign24790_e20424 * locals.var_w_sub0_dn9)) + locals.var_vbscl__blk435_dn9) - locals.var_vbi_dep_dn9), ((((((locals.var_c_2esipq_nsub_inv_dn10 * locals.var_w_sub0) + (locals.var_c_2esipq_nsub_inv * locals.var_w_sub0_dn10)) * locals.var_w_sub0) + (assign24790_e20424 * locals.var_w_sub0_dn10)) + locals.var_vbscl__blk435_dn10) - locals.var_vbi_dep_dn10), ((((((locals.var_c_2esipq_nsub_inv_dn13 * locals.var_w_sub0) + (locals.var_c_2esipq_nsub_inv * locals.var_w_sub0_dn13)) * locals.var_w_sub0) + (assign24790_e20424 * locals.var_w_sub0_dn13)) + locals.var_vbscl__blk435_dn13) - locals.var_vbi_dep_dn13),)
    } else {
        (locals.var_phi_j0_dep, locals.var_phi_j0_dep_dn0, locals.var_phi_j0_dep_dn2, locals.var_phi_j0_dep_dn4, locals.var_phi_j0_dep_dn5, locals.var_phi_j0_dep_dn6, locals.var_phi_j0_dep_dn7, locals.var_phi_j0_dep_dn8, locals.var_phi_j0_dep_dn9, locals.var_phi_j0_dep_dn10, locals.var_phi_j0_dep_dn13,)
    }
};
        locals.var_phi_j0_dep = assign24790_e20432;
        locals.var_phi_j0_dep_dn0 = assign24790_e20432_d_n0;
        locals.var_phi_j0_dep_dn2 = assign24790_e20432_d_n2;
        locals.var_phi_j0_dep_dn4 = assign24790_e20432_d_n4;
        locals.var_phi_j0_dep_dn5 = assign24790_e20432_d_n5;
        locals.var_phi_j0_dep_dn6 = assign24790_e20432_d_n6;
        locals.var_phi_j0_dep_dn7 = assign24790_e20432_d_n7;
        locals.var_phi_j0_dep_dn8 = assign24790_e20432_d_n8;
        locals.var_phi_j0_dep_dn9 = assign24790_e20432_d_n9;
        locals.var_phi_j0_dep_dn10 = assign24790_e20432_d_n10;
        locals.var_phi_j0_dep_dn13 = assign24790_e20432_d_n13;
        locals.var_phi_j0_dep_rv = 0.0;

        let (assign24800_e20447, assign24800_e20447_d_n0, assign24800_e20447_d_n2, assign24800_e20447_d_n4, assign24800_e20447_d_n5, assign24800_e20447_d_n6, assign24800_e20447_d_n7, assign24800_e20447_d_n8, assign24800_e20447_d_n9, assign24800_e20447_d_n10, assign24800_e20447_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard559 == 0.0)) {
        let assign24800_e20441: f64 = (locals.var_c_2esipq_ndepm_inv * locals.var_w_b0);
        let assign24800_e20443: f64 = (assign24800_e20441 * locals.var_w_b0);
        let assign24800_e20445: f64 = (assign24800_e20443 + locals.var_phi_j0_dep);
        (assign24800_e20445, (((((locals.var_c_2esipq_ndepm_inv_dn0 * locals.var_w_b0) + (locals.var_c_2esipq_ndepm_inv * locals.var_w_b0_dn0)) * locals.var_w_b0) + (assign24800_e20441 * locals.var_w_b0_dn0)) + locals.var_phi_j0_dep_dn0), (((((locals.var_c_2esipq_ndepm_inv_dn2 * locals.var_w_b0) + (locals.var_c_2esipq_ndepm_inv * locals.var_w_b0_dn2)) * locals.var_w_b0) + (assign24800_e20441 * locals.var_w_b0_dn2)) + locals.var_phi_j0_dep_dn2), (((((locals.var_c_2esipq_ndepm_inv_dn4 * locals.var_w_b0) + (locals.var_c_2esipq_ndepm_inv * locals.var_w_b0_dn4)) * locals.var_w_b0) + (assign24800_e20441 * locals.var_w_b0_dn4)) + locals.var_phi_j0_dep_dn4), (((((locals.var_c_2esipq_ndepm_inv_dn5 * locals.var_w_b0) + (locals.var_c_2esipq_ndepm_inv * locals.var_w_b0_dn5)) * locals.var_w_b0) + (assign24800_e20441 * locals.var_w_b0_dn5)) + locals.var_phi_j0_dep_dn5), (((((locals.var_c_2esipq_ndepm_inv_dn6 * locals.var_w_b0) + (locals.var_c_2esipq_ndepm_inv * locals.var_w_b0_dn6)) * locals.var_w_b0) + (assign24800_e20441 * locals.var_w_b0_dn6)) + locals.var_phi_j0_dep_dn6), (((((locals.var_c_2esipq_ndepm_inv_dn7 * locals.var_w_b0) + (locals.var_c_2esipq_ndepm_inv * locals.var_w_b0_dn7)) * locals.var_w_b0) + (assign24800_e20441 * locals.var_w_b0_dn7)) + locals.var_phi_j0_dep_dn7), (((((locals.var_c_2esipq_ndepm_inv_dn8 * locals.var_w_b0) + (locals.var_c_2esipq_ndepm_inv * locals.var_w_b0_dn8)) * locals.var_w_b0) + (assign24800_e20441 * locals.var_w_b0_dn8)) + locals.var_phi_j0_dep_dn8), (((((locals.var_c_2esipq_ndepm_inv_dn9 * locals.var_w_b0) + (locals.var_c_2esipq_ndepm_inv * locals.var_w_b0_dn9)) * locals.var_w_b0) + (assign24800_e20441 * locals.var_w_b0_dn9)) + locals.var_phi_j0_dep_dn9), (((((locals.var_c_2esipq_ndepm_inv_dn10 * locals.var_w_b0) + (locals.var_c_2esipq_ndepm_inv * locals.var_w_b0_dn10)) * locals.var_w_b0) + (assign24800_e20441 * locals.var_w_b0_dn10)) + locals.var_phi_j0_dep_dn10), (((((locals.var_c_2esipq_ndepm_inv_dn13 * locals.var_w_b0) + (locals.var_c_2esipq_ndepm_inv * locals.var_w_b0_dn13)) * locals.var_w_b0) + (assign24800_e20441 * locals.var_w_b0_dn13)) + locals.var_phi_j0_dep_dn13),)
    } else {
        (locals.var_phi_b0_dep, locals.var_phi_b0_dep_dn0, locals.var_phi_b0_dep_dn2, locals.var_phi_b0_dep_dn4, locals.var_phi_b0_dep_dn5, locals.var_phi_b0_dep_dn6, locals.var_phi_b0_dep_dn7, locals.var_phi_b0_dep_dn8, locals.var_phi_b0_dep_dn9, locals.var_phi_b0_dep_dn10, locals.var_phi_b0_dep_dn13,)
    }
};
        locals.var_phi_b0_dep = assign24800_e20447;
        locals.var_phi_b0_dep_dn0 = assign24800_e20447_d_n0;
        locals.var_phi_b0_dep_dn2 = assign24800_e20447_d_n2;
        locals.var_phi_b0_dep_dn4 = assign24800_e20447_d_n4;
        locals.var_phi_b0_dep_dn5 = assign24800_e20447_d_n5;
        locals.var_phi_b0_dep_dn6 = assign24800_e20447_d_n6;
        locals.var_phi_b0_dep_dn7 = assign24800_e20447_d_n7;
        locals.var_phi_b0_dep_dn8 = assign24800_e20447_d_n8;
        locals.var_phi_b0_dep_dn9 = assign24800_e20447_d_n9;
        locals.var_phi_b0_dep_dn10 = assign24800_e20447_d_n10;
        locals.var_phi_b0_dep_dn13 = assign24800_e20447_d_n13;
        locals.var_phi_b0_dep_rv = 0.0;

        let (assign24810_e20456, assign24810_e20456_d_n0, assign24810_e20456_d_n2, assign24810_e20456_d_n4, assign24810_e20456_d_n5, assign24810_e20456_d_n6, assign24810_e20456_d_n7, assign24810_e20456_d_n8, assign24810_e20456_d_n9, assign24810_e20456_d_n10, assign24810_e20456_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard559 == 0.0)) {
        (locals.var_phi_j0_dep, locals.var_phi_j0_dep_dn0, locals.var_phi_j0_dep_dn2, locals.var_phi_j0_dep_dn4, locals.var_phi_j0_dep_dn5, locals.var_phi_j0_dep_dn6, locals.var_phi_j0_dep_dn7, locals.var_phi_j0_dep_dn8, locals.var_phi_j0_dep_dn9, locals.var_phi_j0_dep_dn10, locals.var_phi_j0_dep_dn13,)
    } else {
        (locals.var_phi_j0_dep_acc, locals.var_phi_j0_dep_acc_dn0, locals.var_phi_j0_dep_acc_dn2, locals.var_phi_j0_dep_acc_dn4, locals.var_phi_j0_dep_acc_dn5, locals.var_phi_j0_dep_acc_dn6, locals.var_phi_j0_dep_acc_dn7, locals.var_phi_j0_dep_acc_dn8, locals.var_phi_j0_dep_acc_dn9, locals.var_phi_j0_dep_acc_dn10, locals.var_phi_j0_dep_acc_dn13,)
    }
};
        locals.var_phi_j0_dep_acc = assign24810_e20456;
        locals.var_phi_j0_dep_acc_dn0 = assign24810_e20456_d_n0;
        locals.var_phi_j0_dep_acc_dn2 = assign24810_e20456_d_n2;
        locals.var_phi_j0_dep_acc_dn4 = assign24810_e20456_d_n4;
        locals.var_phi_j0_dep_acc_dn5 = assign24810_e20456_d_n5;
        locals.var_phi_j0_dep_acc_dn6 = assign24810_e20456_d_n6;
        locals.var_phi_j0_dep_acc_dn7 = assign24810_e20456_d_n7;
        locals.var_phi_j0_dep_acc_dn8 = assign24810_e20456_d_n8;
        locals.var_phi_j0_dep_acc_dn9 = assign24810_e20456_d_n9;
        locals.var_phi_j0_dep_acc_dn10 = assign24810_e20456_d_n10;
        locals.var_phi_j0_dep_acc_dn13 = assign24810_e20456_d_n13;
        locals.var_phi_j0_dep_acc_rv = 0.0;

        let assign24820_e20459: f64 = if locals.var_vgp > locals.var_vgp0 { 1.0 } else { 0.0 };
        locals.var_guard576 = assign24820_e20459;
        locals.var_guard576_rv = 0.0;

        let (assign24830_e20470,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard559 == 0.0)) && (locals.var_guard576 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_depmode,)
    }
};
        locals.var_depmode = assign24830_e20470;
        locals.var_depmode_rv = 0.0;

        let (assign24840_e20482,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard559 == 0.0)) && (locals.var_guard576 == 0.0)) {
        (2.0,)
    } else {
        (locals.var_depmode,)
    }
};
        locals.var_depmode = assign24840_e20482;
        locals.var_depmode_rv = 0.0;

        let (assign24850_e20495, assign24850_e20495_d_n0, assign24850_e20495_d_n2, assign24850_e20495_d_n4, assign24850_e20495_d_n5, assign24850_e20495_d_n6, assign24850_e20495_d_n7, assign24850_e20495_d_n8, assign24850_e20495_d_n9, assign24850_e20495_d_n10, assign24850_e20495_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) {
        let assign24850_e20489: f64 = (-locals.var_pb2n);
        let assign24850_e20491: f64 = (assign24850_e20489 + locals.var_vbscl__blk435);
        let assign24850_e20492: f64 = (locals.var_psbmax - assign24850_e20491);
        let assign24850_e20493: f64 = (locals.var_c_2esi_q_ndepm * assign24850_e20492);
        (assign24850_e20493, ((locals.var_c_2esi_q_ndepm_dn0 * assign24850_e20492) + (locals.var_c_2esi_q_ndepm * (locals.var_psbmax_dn0 - ((-locals.var_pb2n_dn0) + locals.var_vbscl__blk435_dn0)))), ((locals.var_c_2esi_q_ndepm_dn2 * assign24850_e20492) + (locals.var_c_2esi_q_ndepm * (locals.var_psbmax_dn2 - ((-locals.var_pb2n_dn2) + locals.var_vbscl__blk435_dn2)))), ((locals.var_c_2esi_q_ndepm_dn4 * assign24850_e20492) + (locals.var_c_2esi_q_ndepm * (locals.var_psbmax_dn4 - ((-locals.var_pb2n_dn4) + locals.var_vbscl__blk435_dn4)))), ((locals.var_c_2esi_q_ndepm_dn5 * assign24850_e20492) + (locals.var_c_2esi_q_ndepm * (locals.var_psbmax_dn5 - ((-locals.var_pb2n_dn5) + locals.var_vbscl__blk435_dn5)))), ((locals.var_c_2esi_q_ndepm_dn6 * assign24850_e20492) + (locals.var_c_2esi_q_ndepm * (locals.var_psbmax_dn6 - ((-locals.var_pb2n_dn6) + locals.var_vbscl__blk435_dn6)))), ((locals.var_c_2esi_q_ndepm_dn7 * assign24850_e20492) + (locals.var_c_2esi_q_ndepm * (locals.var_psbmax_dn7 - ((-locals.var_pb2n_dn7) + locals.var_vbscl__blk435_dn7)))), ((locals.var_c_2esi_q_ndepm_dn8 * assign24850_e20492) + (locals.var_c_2esi_q_ndepm * (locals.var_psbmax_dn8 - ((-locals.var_pb2n_dn8) + locals.var_vbscl__blk435_dn8)))), ((locals.var_c_2esi_q_ndepm_dn9 * assign24850_e20492) + (locals.var_c_2esi_q_ndepm * (locals.var_psbmax_dn9 - ((-locals.var_pb2n_dn9) + locals.var_vbscl__blk435_dn9)))), ((locals.var_c_2esi_q_ndepm_dn10 * assign24850_e20492) + (locals.var_c_2esi_q_ndepm * (locals.var_psbmax_dn10 - ((-locals.var_pb2n_dn10) + locals.var_vbscl__blk435_dn10)))), ((locals.var_c_2esi_q_ndepm_dn13 * assign24850_e20492) + (locals.var_c_2esi_q_ndepm * (locals.var_psbmax_dn13 - ((-locals.var_pb2n_dn13) + locals.var_vbscl__blk435_dn13)))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign24850_e20495;
        locals.var_t1_dn0 = assign24850_e20495_d_n0;
        locals.var_t1_dn2 = assign24850_e20495_d_n2;
        locals.var_t1_dn4 = assign24850_e20495_d_n4;
        locals.var_t1_dn5 = assign24850_e20495_d_n5;
        locals.var_t1_dn6 = assign24850_e20495_d_n6;
        locals.var_t1_dn7 = assign24850_e20495_d_n7;
        locals.var_t1_dn8 = assign24850_e20495_d_n8;
        locals.var_t1_dn9 = assign24850_e20495_d_n9;
        locals.var_t1_dn10 = assign24850_e20495_d_n10;
        locals.var_t1_dn13 = assign24850_e20495_d_n13;
        locals.var_t1_rv = 0.0;

        let assign24860_e20498: f64 = if locals.var_t1 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard577 = assign24860_e20498;
        locals.var_guard577_rv = 0.0;

        let (assign24870_e20514, assign24870_e20514_d_n0, assign24870_e20514_d_n2, assign24870_e20514_d_n4, assign24870_e20514_d_n5, assign24870_e20514_d_n6, assign24870_e20514_d_n7, assign24870_e20514_d_n8, assign24870_e20514_d_n9, assign24870_e20514_d_n10, assign24870_e20514_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard577 != 0.0)) {
        let assign24870_e20505: f64 = (-locals.var_pb2n);
        let assign24870_e20507: f64 = (assign24870_e20505 + locals.var_vbscl__blk435);
        let assign24870_e20509: f64 = (locals.var_t1).sqrt();
        let assign24870_e20511: f64 = (assign24870_e20509 / locals.var_cox);
        let assign24870_e20512: f64 = (assign24870_e20507 - assign24870_e20511);
        (assign24870_e20512, (((-locals.var_pb2n_dn0) + locals.var_vbscl__blk435_dn0) - ((((locals.var_t1_dn0 / (2.0 * assign24870_e20509)) * locals.var_cox) - (assign24870_e20509 * locals.var_cox_dn0)) / (locals.var_cox * locals.var_cox))), (((-locals.var_pb2n_dn2) + locals.var_vbscl__blk435_dn2) - ((((locals.var_t1_dn2 / (2.0 * assign24870_e20509)) * locals.var_cox) - (assign24870_e20509 * locals.var_cox_dn2)) / (locals.var_cox * locals.var_cox))), (((-locals.var_pb2n_dn4) + locals.var_vbscl__blk435_dn4) - ((((locals.var_t1_dn4 / (2.0 * assign24870_e20509)) * locals.var_cox) - (assign24870_e20509 * locals.var_cox_dn4)) / (locals.var_cox * locals.var_cox))), (((-locals.var_pb2n_dn5) + locals.var_vbscl__blk435_dn5) - ((((locals.var_t1_dn5 / (2.0 * assign24870_e20509)) * locals.var_cox) - (assign24870_e20509 * locals.var_cox_dn5)) / (locals.var_cox * locals.var_cox))), (((-locals.var_pb2n_dn6) + locals.var_vbscl__blk435_dn6) - ((((locals.var_t1_dn6 / (2.0 * assign24870_e20509)) * locals.var_cox) - (assign24870_e20509 * locals.var_cox_dn6)) / (locals.var_cox * locals.var_cox))), (((-locals.var_pb2n_dn7) + locals.var_vbscl__blk435_dn7) - ((((locals.var_t1_dn7 / (2.0 * assign24870_e20509)) * locals.var_cox) - (assign24870_e20509 * locals.var_cox_dn7)) / (locals.var_cox * locals.var_cox))), (((-locals.var_pb2n_dn8) + locals.var_vbscl__blk435_dn8) - ((((locals.var_t1_dn8 / (2.0 * assign24870_e20509)) * locals.var_cox) - (assign24870_e20509 * locals.var_cox_dn8)) / (locals.var_cox * locals.var_cox))), (((-locals.var_pb2n_dn9) + locals.var_vbscl__blk435_dn9) - ((((locals.var_t1_dn9 / (2.0 * assign24870_e20509)) * locals.var_cox) - (assign24870_e20509 * locals.var_cox_dn9)) / (locals.var_cox * locals.var_cox))), (((-locals.var_pb2n_dn10) + locals.var_vbscl__blk435_dn10) - ((((locals.var_t1_dn10 / (2.0 * assign24870_e20509)) * locals.var_cox) - (assign24870_e20509 * locals.var_cox_dn10)) / (locals.var_cox * locals.var_cox))), (((-locals.var_pb2n_dn13) + locals.var_vbscl__blk435_dn13) - ((((locals.var_t1_dn13 / (2.0 * assign24870_e20509)) * locals.var_cox) - (assign24870_e20509 * locals.var_cox_dn13)) / (locals.var_cox * locals.var_cox))),)
    } else {
        (locals.var_vthn, locals.var_vthn_dn0, locals.var_vthn_dn2, locals.var_vthn_dn4, locals.var_vthn_dn5, locals.var_vthn_dn6, locals.var_vthn_dn7, locals.var_vthn_dn8, locals.var_vthn_dn9, locals.var_vthn_dn10, locals.var_vthn_dn13,)
    }
};
        locals.var_vthn = assign24870_e20514;
        locals.var_vthn_dn0 = assign24870_e20514_d_n0;
        locals.var_vthn_dn2 = assign24870_e20514_d_n2;
        locals.var_vthn_dn4 = assign24870_e20514_d_n4;
        locals.var_vthn_dn5 = assign24870_e20514_d_n5;
        locals.var_vthn_dn6 = assign24870_e20514_d_n6;
        locals.var_vthn_dn7 = assign24870_e20514_d_n7;
        locals.var_vthn_dn8 = assign24870_e20514_d_n8;
        locals.var_vthn_dn9 = assign24870_e20514_d_n9;
        locals.var_vthn_dn10 = assign24870_e20514_d_n10;
        locals.var_vthn_dn13 = assign24870_e20514_d_n13;
        locals.var_vthn_rv = 0.0;

        let (assign24880_e20526, assign24880_e20526_d_n0, assign24880_e20526_d_n2, assign24880_e20526_d_n4, assign24880_e20526_d_n5, assign24880_e20526_d_n6, assign24880_e20526_d_n7, assign24880_e20526_d_n8, assign24880_e20526_d_n9, assign24880_e20526_d_n10, assign24880_e20526_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard577 == 0.0)) {
        let assign24880_e20522: f64 = (-locals.var_pb2n);
        let assign24880_e20524: f64 = (assign24880_e20522 + locals.var_vbscl__blk435);
        (assign24880_e20524, ((-locals.var_pb2n_dn0) + locals.var_vbscl__blk435_dn0), ((-locals.var_pb2n_dn2) + locals.var_vbscl__blk435_dn2), ((-locals.var_pb2n_dn4) + locals.var_vbscl__blk435_dn4), ((-locals.var_pb2n_dn5) + locals.var_vbscl__blk435_dn5), ((-locals.var_pb2n_dn6) + locals.var_vbscl__blk435_dn6), ((-locals.var_pb2n_dn7) + locals.var_vbscl__blk435_dn7), ((-locals.var_pb2n_dn8) + locals.var_vbscl__blk435_dn8), ((-locals.var_pb2n_dn9) + locals.var_vbscl__blk435_dn9), ((-locals.var_pb2n_dn10) + locals.var_vbscl__blk435_dn10), ((-locals.var_pb2n_dn13) + locals.var_vbscl__blk435_dn13),)
    } else {
        (locals.var_vthn, locals.var_vthn_dn0, locals.var_vthn_dn2, locals.var_vthn_dn4, locals.var_vthn_dn5, locals.var_vthn_dn6, locals.var_vthn_dn7, locals.var_vthn_dn8, locals.var_vthn_dn9, locals.var_vthn_dn10, locals.var_vthn_dn13,)
    }
};
        locals.var_vthn = assign24880_e20526;
        locals.var_vthn_dn0 = assign24880_e20526_d_n0;
        locals.var_vthn_dn2 = assign24880_e20526_d_n2;
        locals.var_vthn_dn4 = assign24880_e20526_d_n4;
        locals.var_vthn_dn5 = assign24880_e20526_d_n5;
        locals.var_vthn_dn6 = assign24880_e20526_d_n6;
        locals.var_vthn_dn7 = assign24880_e20526_d_n7;
        locals.var_vthn_dn8 = assign24880_e20526_d_n8;
        locals.var_vthn_dn9 = assign24880_e20526_d_n9;
        locals.var_vthn_dn10 = assign24880_e20526_d_n10;
        locals.var_vthn_dn13 = assign24880_e20526_d_n13;
        locals.var_vthn_rv = 0.0;

        let assign24890_e20529: f64 = if locals.var_vgp > locals.var_vgp0 { 1.0 } else { 0.0 };
        locals.var_guard578 = assign24890_e20529;
        locals.var_guard578_rv = 0.0;

        let (assign24900_e20537, assign24900_e20537_d_n0, assign24900_e20537_d_n2, assign24900_e20537_d_n4, assign24900_e20537_d_n5, assign24900_e20537_d_n6, assign24900_e20537_d_n7, assign24900_e20537_d_n8, assign24900_e20537_d_n9, assign24900_e20537_d_n10, assign24900_e20537_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard578 != 0.0)) {
        (locals.var_phi_j0_dep_acc, locals.var_phi_j0_dep_acc_dn0, locals.var_phi_j0_dep_acc_dn2, locals.var_phi_j0_dep_acc_dn4, locals.var_phi_j0_dep_acc_dn5, locals.var_phi_j0_dep_acc_dn6, locals.var_phi_j0_dep_acc_dn7, locals.var_phi_j0_dep_acc_dn8, locals.var_phi_j0_dep_acc_dn9, locals.var_phi_j0_dep_acc_dn10, locals.var_phi_j0_dep_acc_dn13,)
    } else {
        (locals.var_phi_j0_dep, locals.var_phi_j0_dep_dn0, locals.var_phi_j0_dep_dn2, locals.var_phi_j0_dep_dn4, locals.var_phi_j0_dep_dn5, locals.var_phi_j0_dep_dn6, locals.var_phi_j0_dep_dn7, locals.var_phi_j0_dep_dn8, locals.var_phi_j0_dep_dn9, locals.var_phi_j0_dep_dn10, locals.var_phi_j0_dep_dn13,)
    }
};
        locals.var_phi_j0_dep = assign24900_e20537;
        locals.var_phi_j0_dep_dn0 = assign24900_e20537_d_n0;
        locals.var_phi_j0_dep_dn2 = assign24900_e20537_d_n2;
        locals.var_phi_j0_dep_dn4 = assign24900_e20537_d_n4;
        locals.var_phi_j0_dep_dn5 = assign24900_e20537_d_n5;
        locals.var_phi_j0_dep_dn6 = assign24900_e20537_d_n6;
        locals.var_phi_j0_dep_dn7 = assign24900_e20537_d_n7;
        locals.var_phi_j0_dep_dn8 = assign24900_e20537_d_n8;
        locals.var_phi_j0_dep_dn9 = assign24900_e20537_d_n9;
        locals.var_phi_j0_dep_dn10 = assign24900_e20537_d_n10;
        locals.var_phi_j0_dep_dn13 = assign24900_e20537_d_n13;
        locals.var_phi_j0_dep_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_68(
        locals: &mut StampLocals,
    ) {
        let (assign24910_e20545, assign24910_e20545_d_n0, assign24910_e20545_d_n2, assign24910_e20545_d_n4, assign24910_e20545_d_n5, assign24910_e20545_d_n6, assign24910_e20545_d_n7, assign24910_e20545_d_n8, assign24910_e20545_d_n9, assign24910_e20545_d_n10, assign24910_e20545_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard578 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_phi_b0_dep, locals.var_phi_b0_dep_dn0, locals.var_phi_b0_dep_dn2, locals.var_phi_b0_dep_dn4, locals.var_phi_b0_dep_dn5, locals.var_phi_b0_dep_dn6, locals.var_phi_b0_dep_dn7, locals.var_phi_b0_dep_dn8, locals.var_phi_b0_dep_dn9, locals.var_phi_b0_dep_dn10, locals.var_phi_b0_dep_dn13,)
    }
};
        locals.var_phi_b0_dep = assign24910_e20545;
        locals.var_phi_b0_dep_dn0 = assign24910_e20545_d_n0;
        locals.var_phi_b0_dep_dn2 = assign24910_e20545_d_n2;
        locals.var_phi_b0_dep_dn4 = assign24910_e20545_d_n4;
        locals.var_phi_b0_dep_dn5 = assign24910_e20545_d_n5;
        locals.var_phi_b0_dep_dn6 = assign24910_e20545_d_n6;
        locals.var_phi_b0_dep_dn7 = assign24910_e20545_d_n7;
        locals.var_phi_b0_dep_dn8 = assign24910_e20545_d_n8;
        locals.var_phi_b0_dep_dn9 = assign24910_e20545_d_n9;
        locals.var_phi_b0_dep_dn10 = assign24910_e20545_d_n10;
        locals.var_phi_b0_dep_dn13 = assign24910_e20545_d_n13;
        locals.var_phi_b0_dep_rv = 0.0;

        let (assign24920_e20566, assign24920_e20566_d_n0, assign24920_e20566_d_n2, assign24920_e20566_d_n4, assign24920_e20566_d_n5, assign24920_e20566_d_n6, assign24920_e20566_d_n7, assign24920_e20566_d_n8, assign24920_e20566_d_n9, assign24920_e20566_d_n10, assign24920_e20566_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard578 != 0.0)) {
        let assign24920_e20553: f64 = (locals.var_afact * locals.var_vgp);
        let assign24920_e20555: f64 = (assign24920_e20553 * locals.var_vgp);
        let assign24920_e20556: f64 = (assign24920_e20555).ln();
        let assign24920_e20560: f64 = (2.0 / locals.var_vgp);
        let assign24920_e20561: f64 = (locals.var_beta + assign24920_e20560);
        let assign24920_e20562: f64 = (assign24920_e20556 / assign24920_e20561);
        let assign24920_e20564: f64 = (assign24920_e20562 + locals.var_phi_b0_dep);
        (assign24920_e20564, (((((((((locals.var_afact_dn0 * locals.var_vgp) + (locals.var_afact * locals.var_vgp_dn0)) * locals.var_vgp) + (assign24920_e20553 * locals.var_vgp_dn0)) / assign24920_e20555) * assign24920_e20561) - (assign24920_e20556 * (locals.var_beta_dn0 + (-((2.0 * locals.var_vgp_dn0) / (locals.var_vgp * locals.var_vgp)))))) / (assign24920_e20561 * assign24920_e20561)) + locals.var_phi_b0_dep_dn0), (((((((((locals.var_afact_dn2 * locals.var_vgp) + (locals.var_afact * locals.var_vgp_dn2)) * locals.var_vgp) + (assign24920_e20553 * locals.var_vgp_dn2)) / assign24920_e20555) * assign24920_e20561) - (assign24920_e20556 * (locals.var_beta_dn2 + (-((2.0 * locals.var_vgp_dn2) / (locals.var_vgp * locals.var_vgp)))))) / (assign24920_e20561 * assign24920_e20561)) + locals.var_phi_b0_dep_dn2), (((((((((locals.var_afact_dn4 * locals.var_vgp) + (locals.var_afact * locals.var_vgp_dn4)) * locals.var_vgp) + (assign24920_e20553 * locals.var_vgp_dn4)) / assign24920_e20555) * assign24920_e20561) - (assign24920_e20556 * (locals.var_beta_dn4 + (-((2.0 * locals.var_vgp_dn4) / (locals.var_vgp * locals.var_vgp)))))) / (assign24920_e20561 * assign24920_e20561)) + locals.var_phi_b0_dep_dn4), (((((((((locals.var_afact_dn5 * locals.var_vgp) + (locals.var_afact * locals.var_vgp_dn5)) * locals.var_vgp) + (assign24920_e20553 * locals.var_vgp_dn5)) / assign24920_e20555) * assign24920_e20561) - (assign24920_e20556 * (locals.var_beta_dn5 + (-((2.0 * locals.var_vgp_dn5) / (locals.var_vgp * locals.var_vgp)))))) / (assign24920_e20561 * assign24920_e20561)) + locals.var_phi_b0_dep_dn5), (((((((((locals.var_afact_dn6 * locals.var_vgp) + (locals.var_afact * locals.var_vgp_dn6)) * locals.var_vgp) + (assign24920_e20553 * locals.var_vgp_dn6)) / assign24920_e20555) * assign24920_e20561) - (assign24920_e20556 * (locals.var_beta_dn6 + (-((2.0 * locals.var_vgp_dn6) / (locals.var_vgp * locals.var_vgp)))))) / (assign24920_e20561 * assign24920_e20561)) + locals.var_phi_b0_dep_dn6), (((((((((locals.var_afact_dn7 * locals.var_vgp) + (locals.var_afact * locals.var_vgp_dn7)) * locals.var_vgp) + (assign24920_e20553 * locals.var_vgp_dn7)) / assign24920_e20555) * assign24920_e20561) - (assign24920_e20556 * (locals.var_beta_dn7 + (-((2.0 * locals.var_vgp_dn7) / (locals.var_vgp * locals.var_vgp)))))) / (assign24920_e20561 * assign24920_e20561)) + locals.var_phi_b0_dep_dn7), (((((((((locals.var_afact_dn8 * locals.var_vgp) + (locals.var_afact * locals.var_vgp_dn8)) * locals.var_vgp) + (assign24920_e20553 * locals.var_vgp_dn8)) / assign24920_e20555) * assign24920_e20561) - (assign24920_e20556 * (locals.var_beta_dn8 + (-((2.0 * locals.var_vgp_dn8) / (locals.var_vgp * locals.var_vgp)))))) / (assign24920_e20561 * assign24920_e20561)) + locals.var_phi_b0_dep_dn8), (((((((((locals.var_afact_dn9 * locals.var_vgp) + (locals.var_afact * locals.var_vgp_dn9)) * locals.var_vgp) + (assign24920_e20553 * locals.var_vgp_dn9)) / assign24920_e20555) * assign24920_e20561) - (assign24920_e20556 * (locals.var_beta_dn9 + (-((2.0 * locals.var_vgp_dn9) / (locals.var_vgp * locals.var_vgp)))))) / (assign24920_e20561 * assign24920_e20561)) + locals.var_phi_b0_dep_dn9), (((((((((locals.var_afact_dn10 * locals.var_vgp) + (locals.var_afact * locals.var_vgp_dn10)) * locals.var_vgp) + (assign24920_e20553 * locals.var_vgp_dn10)) / assign24920_e20555) * assign24920_e20561) - (assign24920_e20556 * (locals.var_beta_dn10 + (-((2.0 * locals.var_vgp_dn10) / (locals.var_vgp * locals.var_vgp)))))) / (assign24920_e20561 * assign24920_e20561)) + locals.var_phi_b0_dep_dn10), (((((((((locals.var_afact_dn13 * locals.var_vgp) + (locals.var_afact * locals.var_vgp_dn13)) * locals.var_vgp) + (assign24920_e20553 * locals.var_vgp_dn13)) / assign24920_e20555) * assign24920_e20561) - (assign24920_e20556 * (locals.var_beta_dn13 + (-((2.0 * locals.var_vgp_dn13) / (locals.var_vgp * locals.var_vgp)))))) / (assign24920_e20561 * assign24920_e20561)) + locals.var_phi_b0_dep_dn13),)
    } else {
        (locals.var_phi_s0_dep_ini, locals.var_phi_s0_dep_ini_dn0, locals.var_phi_s0_dep_ini_dn2, locals.var_phi_s0_dep_ini_dn4, locals.var_phi_s0_dep_ini_dn5, locals.var_phi_s0_dep_ini_dn6, locals.var_phi_s0_dep_ini_dn7, locals.var_phi_s0_dep_ini_dn8, locals.var_phi_s0_dep_ini_dn9, locals.var_phi_s0_dep_ini_dn10, locals.var_phi_s0_dep_ini_dn13,)
    }
};
        locals.var_phi_s0_dep_ini = assign24920_e20566;
        locals.var_phi_s0_dep_ini_dn0 = assign24920_e20566_d_n0;
        locals.var_phi_s0_dep_ini_dn2 = assign24920_e20566_d_n2;
        locals.var_phi_s0_dep_ini_dn4 = assign24920_e20566_d_n4;
        locals.var_phi_s0_dep_ini_dn5 = assign24920_e20566_d_n5;
        locals.var_phi_s0_dep_ini_dn6 = assign24920_e20566_d_n6;
        locals.var_phi_s0_dep_ini_dn7 = assign24920_e20566_d_n7;
        locals.var_phi_s0_dep_ini_dn8 = assign24920_e20566_d_n8;
        locals.var_phi_s0_dep_ini_dn9 = assign24920_e20566_d_n9;
        locals.var_phi_s0_dep_ini_dn10 = assign24920_e20566_d_n10;
        locals.var_phi_s0_dep_ini_dn13 = assign24920_e20566_d_n13;
        locals.var_phi_s0_dep_ini_rv = 0.0;

        let assign24930_e20570: f64 = (locals.var_vds_maxb0 + locals.var_ps_conv23);
        let assign24930_e20571: f64 = if locals.var_phi_s0_dep_ini < assign24930_e20570 { 1.0 } else { 0.0 };
        locals.var_guard579 = assign24930_e20571;
        locals.var_guard579_rv = 0.0;

        let (assign24940_e20583, assign24940_e20583_d_n0, assign24940_e20583_d_n2, assign24940_e20583_d_n4, assign24940_e20583_d_n5, assign24940_e20583_d_n6, assign24940_e20583_d_n7, assign24940_e20583_d_n8, assign24940_e20583_d_n9, assign24940_e20583_d_n10, assign24940_e20583_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard578 != 0.0)) && (locals.var_guard579 != 0.0)) {
        let assign24940_e20581: f64 = (locals.var_vds_maxb0 + locals.var_ps_conv23);
        (assign24940_e20581, locals.var_vds_maxb0_dn0, locals.var_vds_maxb0_dn2, locals.var_vds_maxb0_dn4, locals.var_vds_maxb0_dn5, locals.var_vds_maxb0_dn6, locals.var_vds_maxb0_dn7, locals.var_vds_maxb0_dn8, locals.var_vds_maxb0_dn9, locals.var_vds_maxb0_dn10, locals.var_vds_maxb0_dn13,)
    } else {
        (locals.var_phi_s0_dep_ini, locals.var_phi_s0_dep_ini_dn0, locals.var_phi_s0_dep_ini_dn2, locals.var_phi_s0_dep_ini_dn4, locals.var_phi_s0_dep_ini_dn5, locals.var_phi_s0_dep_ini_dn6, locals.var_phi_s0_dep_ini_dn7, locals.var_phi_s0_dep_ini_dn8, locals.var_phi_s0_dep_ini_dn9, locals.var_phi_s0_dep_ini_dn10, locals.var_phi_s0_dep_ini_dn13,)
    }
};
        locals.var_phi_s0_dep_ini = assign24940_e20583;
        locals.var_phi_s0_dep_ini_dn0 = assign24940_e20583_d_n0;
        locals.var_phi_s0_dep_ini_dn2 = assign24940_e20583_d_n2;
        locals.var_phi_s0_dep_ini_dn4 = assign24940_e20583_d_n4;
        locals.var_phi_s0_dep_ini_dn5 = assign24940_e20583_d_n5;
        locals.var_phi_s0_dep_ini_dn6 = assign24940_e20583_d_n6;
        locals.var_phi_s0_dep_ini_dn7 = assign24940_e20583_d_n7;
        locals.var_phi_s0_dep_ini_dn8 = assign24940_e20583_d_n8;
        locals.var_phi_s0_dep_ini_dn9 = assign24940_e20583_d_n9;
        locals.var_phi_s0_dep_ini_dn10 = assign24940_e20583_d_n10;
        locals.var_phi_s0_dep_ini_dn13 = assign24940_e20583_d_n13;
        locals.var_phi_s0_dep_ini_rv = 0.0;

        let assign24950_e20586: f64 = if locals.var_vgp > locals.var_vgp1 { 1.0 } else { 0.0 };
        locals.var_guard580 = assign24950_e20586;
        locals.var_guard580_rv = 0.0;

        let (assign24960_e20597, assign24960_e20597_d_n0, assign24960_e20597_d_n2, assign24960_e20597_d_n4, assign24960_e20597_d_n5, assign24960_e20597_d_n6, assign24960_e20597_d_n7, assign24960_e20597_d_n8, assign24960_e20597_d_n9, assign24960_e20597_d_n10, assign24960_e20597_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard578 == 0.0)) && (locals.var_guard580 != 0.0)) {
        (locals.var_phi_s0_dep, locals.var_phi_s0_dep_dn0, locals.var_phi_s0_dep_dn2, locals.var_phi_s0_dep_dn4, locals.var_phi_s0_dep_dn5, locals.var_phi_s0_dep_dn6, locals.var_phi_s0_dep_dn7, locals.var_phi_s0_dep_dn8, locals.var_phi_s0_dep_dn9, locals.var_phi_s0_dep_dn10, locals.var_phi_s0_dep_dn13,)
    } else {
        (locals.var_phi_s0_dep_ini, locals.var_phi_s0_dep_ini_dn0, locals.var_phi_s0_dep_ini_dn2, locals.var_phi_s0_dep_ini_dn4, locals.var_phi_s0_dep_ini_dn5, locals.var_phi_s0_dep_ini_dn6, locals.var_phi_s0_dep_ini_dn7, locals.var_phi_s0_dep_ini_dn8, locals.var_phi_s0_dep_ini_dn9, locals.var_phi_s0_dep_ini_dn10, locals.var_phi_s0_dep_ini_dn13,)
    }
};
        locals.var_phi_s0_dep_ini = assign24960_e20597;
        locals.var_phi_s0_dep_ini_dn0 = assign24960_e20597_d_n0;
        locals.var_phi_s0_dep_ini_dn2 = assign24960_e20597_d_n2;
        locals.var_phi_s0_dep_ini_dn4 = assign24960_e20597_d_n4;
        locals.var_phi_s0_dep_ini_dn5 = assign24960_e20597_d_n5;
        locals.var_phi_s0_dep_ini_dn6 = assign24960_e20597_d_n6;
        locals.var_phi_s0_dep_ini_dn7 = assign24960_e20597_d_n7;
        locals.var_phi_s0_dep_ini_dn8 = assign24960_e20597_d_n8;
        locals.var_phi_s0_dep_ini_dn9 = assign24960_e20597_d_n9;
        locals.var_phi_s0_dep_ini_dn10 = assign24960_e20597_d_n10;
        locals.var_phi_s0_dep_ini_dn13 = assign24960_e20597_d_n13;
        locals.var_phi_s0_dep_ini_rv = 0.0;

        let assign24970_e20600: f64 = if locals.var_vgp > locals.var_vthn { 1.0 } else { 0.0 };
        locals.var_guard581 = assign24970_e20600;
        locals.var_guard581_rv = 0.0;

        let (assign24980_e20621, assign24980_e20621_d_n0, assign24980_e20621_d_n2, assign24980_e20621_d_n4, assign24980_e20621_d_n5, assign24980_e20621_d_n6, assign24980_e20621_d_n7, assign24980_e20621_d_n8, assign24980_e20621_d_n9, assign24980_e20621_d_n10, assign24980_e20621_d_n13,) = {
    if (((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard578 == 0.0)) && (locals.var_guard580 == 0.0)) && (locals.var_guard581 != 0.0)) {
        let assign24980_e20613: f64 = (-2.0);
        let assign24980_e20615: f64 = (assign24980_e20613 * locals.var_afact);
        let assign24980_e20617: f64 = (assign24980_e20615 * locals.var_vgp);
        let assign24980_e20619: f64 = (assign24980_e20617 + locals.var_beta);
        (assign24980_e20619, ((((assign24980_e20613 * locals.var_afact_dn0) * locals.var_vgp) + (assign24980_e20615 * locals.var_vgp_dn0)) + locals.var_beta_dn0), ((((assign24980_e20613 * locals.var_afact_dn2) * locals.var_vgp) + (assign24980_e20615 * locals.var_vgp_dn2)) + locals.var_beta_dn2), ((((assign24980_e20613 * locals.var_afact_dn4) * locals.var_vgp) + (assign24980_e20615 * locals.var_vgp_dn4)) + locals.var_beta_dn4), ((((assign24980_e20613 * locals.var_afact_dn5) * locals.var_vgp) + (assign24980_e20615 * locals.var_vgp_dn5)) + locals.var_beta_dn5), ((((assign24980_e20613 * locals.var_afact_dn6) * locals.var_vgp) + (assign24980_e20615 * locals.var_vgp_dn6)) + locals.var_beta_dn6), ((((assign24980_e20613 * locals.var_afact_dn7) * locals.var_vgp) + (assign24980_e20615 * locals.var_vgp_dn7)) + locals.var_beta_dn7), ((((assign24980_e20613 * locals.var_afact_dn8) * locals.var_vgp) + (assign24980_e20615 * locals.var_vgp_dn8)) + locals.var_beta_dn8), ((((assign24980_e20613 * locals.var_afact_dn9) * locals.var_vgp) + (assign24980_e20615 * locals.var_vgp_dn9)) + locals.var_beta_dn9), ((((assign24980_e20613 * locals.var_afact_dn10) * locals.var_vgp) + (assign24980_e20615 * locals.var_vgp_dn10)) + locals.var_beta_dn10), ((((assign24980_e20613 * locals.var_afact_dn13) * locals.var_vgp) + (assign24980_e20615 * locals.var_vgp_dn13)) + locals.var_beta_dn13),)
    } else {
        (locals.var_bfact, locals.var_bfact_dn0, locals.var_bfact_dn2, locals.var_bfact_dn4, locals.var_bfact_dn5, locals.var_bfact_dn6, locals.var_bfact_dn7, locals.var_bfact_dn8, locals.var_bfact_dn9, locals.var_bfact_dn10, locals.var_bfact_dn13,)
    }
};
        locals.var_bfact = assign24980_e20621;
        locals.var_bfact_dn0 = assign24980_e20621_d_n0;
        locals.var_bfact_dn2 = assign24980_e20621_d_n2;
        locals.var_bfact_dn4 = assign24980_e20621_d_n4;
        locals.var_bfact_dn5 = assign24980_e20621_d_n5;
        locals.var_bfact_dn6 = assign24980_e20621_d_n6;
        locals.var_bfact_dn7 = assign24980_e20621_d_n7;
        locals.var_bfact_dn8 = assign24980_e20621_d_n8;
        locals.var_bfact_dn9 = assign24980_e20621_d_n9;
        locals.var_bfact_dn10 = assign24980_e20621_d_n10;
        locals.var_bfact_dn13 = assign24980_e20621_d_n13;
        locals.var_bfact_rv = 0.0;

        let (assign24990_e20643, assign24990_e20643_d_n0, assign24990_e20643_d_n2, assign24990_e20643_d_n4, assign24990_e20643_d_n5, assign24990_e20643_d_n6, assign24990_e20643_d_n7, assign24990_e20643_d_n8, assign24990_e20643_d_n9, assign24990_e20643_d_n10, assign24990_e20643_d_n13,) = {
    if (((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard578 == 0.0)) && (locals.var_guard580 == 0.0)) && (locals.var_guard581 != 0.0)) {
        let assign24990_e20635: f64 = (locals.var_afact * locals.var_vgp);
        let assign24990_e20637: f64 = (assign24990_e20635 * locals.var_vgp);
        let assign24990_e20640: f64 = (locals.var_beta * locals.var_phi_b0_dep);
        let assign24990_e20641: f64 = (assign24990_e20637 - assign24990_e20640);
        (assign24990_e20641, (((((locals.var_afact_dn0 * locals.var_vgp) + (locals.var_afact * locals.var_vgp_dn0)) * locals.var_vgp) + (assign24990_e20635 * locals.var_vgp_dn0)) - ((locals.var_beta_dn0 * locals.var_phi_b0_dep) + (locals.var_beta * locals.var_phi_b0_dep_dn0))), (((((locals.var_afact_dn2 * locals.var_vgp) + (locals.var_afact * locals.var_vgp_dn2)) * locals.var_vgp) + (assign24990_e20635 * locals.var_vgp_dn2)) - ((locals.var_beta_dn2 * locals.var_phi_b0_dep) + (locals.var_beta * locals.var_phi_b0_dep_dn2))), (((((locals.var_afact_dn4 * locals.var_vgp) + (locals.var_afact * locals.var_vgp_dn4)) * locals.var_vgp) + (assign24990_e20635 * locals.var_vgp_dn4)) - ((locals.var_beta_dn4 * locals.var_phi_b0_dep) + (locals.var_beta * locals.var_phi_b0_dep_dn4))), (((((locals.var_afact_dn5 * locals.var_vgp) + (locals.var_afact * locals.var_vgp_dn5)) * locals.var_vgp) + (assign24990_e20635 * locals.var_vgp_dn5)) - ((locals.var_beta_dn5 * locals.var_phi_b0_dep) + (locals.var_beta * locals.var_phi_b0_dep_dn5))), (((((locals.var_afact_dn6 * locals.var_vgp) + (locals.var_afact * locals.var_vgp_dn6)) * locals.var_vgp) + (assign24990_e20635 * locals.var_vgp_dn6)) - ((locals.var_beta_dn6 * locals.var_phi_b0_dep) + (locals.var_beta * locals.var_phi_b0_dep_dn6))), (((((locals.var_afact_dn7 * locals.var_vgp) + (locals.var_afact * locals.var_vgp_dn7)) * locals.var_vgp) + (assign24990_e20635 * locals.var_vgp_dn7)) - ((locals.var_beta_dn7 * locals.var_phi_b0_dep) + (locals.var_beta * locals.var_phi_b0_dep_dn7))), (((((locals.var_afact_dn8 * locals.var_vgp) + (locals.var_afact * locals.var_vgp_dn8)) * locals.var_vgp) + (assign24990_e20635 * locals.var_vgp_dn8)) - ((locals.var_beta_dn8 * locals.var_phi_b0_dep) + (locals.var_beta * locals.var_phi_b0_dep_dn8))), (((((locals.var_afact_dn9 * locals.var_vgp) + (locals.var_afact * locals.var_vgp_dn9)) * locals.var_vgp) + (assign24990_e20635 * locals.var_vgp_dn9)) - ((locals.var_beta_dn9 * locals.var_phi_b0_dep) + (locals.var_beta * locals.var_phi_b0_dep_dn9))), (((((locals.var_afact_dn10 * locals.var_vgp) + (locals.var_afact * locals.var_vgp_dn10)) * locals.var_vgp) + (assign24990_e20635 * locals.var_vgp_dn10)) - ((locals.var_beta_dn10 * locals.var_phi_b0_dep) + (locals.var_beta * locals.var_phi_b0_dep_dn10))), (((((locals.var_afact_dn13 * locals.var_vgp) + (locals.var_afact * locals.var_vgp_dn13)) * locals.var_vgp) + (assign24990_e20635 * locals.var_vgp_dn13)) - ((locals.var_beta_dn13 * locals.var_phi_b0_dep) + (locals.var_beta * locals.var_phi_b0_dep_dn13))),)
    } else {
        (locals.var_cfact, locals.var_cfact_dn0, locals.var_cfact_dn2, locals.var_cfact_dn4, locals.var_cfact_dn5, locals.var_cfact_dn6, locals.var_cfact_dn7, locals.var_cfact_dn8, locals.var_cfact_dn9, locals.var_cfact_dn10, locals.var_cfact_dn13,)
    }
};
        locals.var_cfact = assign24990_e20643;
        locals.var_cfact_dn0 = assign24990_e20643_d_n0;
        locals.var_cfact_dn2 = assign24990_e20643_d_n2;
        locals.var_cfact_dn4 = assign24990_e20643_d_n4;
        locals.var_cfact_dn5 = assign24990_e20643_d_n5;
        locals.var_cfact_dn6 = assign24990_e20643_d_n6;
        locals.var_cfact_dn7 = assign24990_e20643_d_n7;
        locals.var_cfact_dn8 = assign24990_e20643_d_n8;
        locals.var_cfact_dn9 = assign24990_e20643_d_n9;
        locals.var_cfact_dn10 = assign24990_e20643_d_n10;
        locals.var_cfact_dn13 = assign24990_e20643_d_n13;
        locals.var_cfact_rv = 0.0;

        let (assign25000_e20657, assign25000_e20657_d_n0, assign25000_e20657_d_n2, assign25000_e20657_d_n4, assign25000_e20657_d_n5, assign25000_e20657_d_n6, assign25000_e20657_d_n7, assign25000_e20657_d_n8, assign25000_e20657_d_n9, assign25000_e20657_d_n10, assign25000_e20657_d_n13,) = {
    if (((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard578 == 0.0)) && (locals.var_guard580 == 0.0)) && (locals.var_guard581 != 0.0)) {
        (locals.var_phi_b0_dep, locals.var_phi_b0_dep_dn0, locals.var_phi_b0_dep_dn2, locals.var_phi_b0_dep_dn4, locals.var_phi_b0_dep_dn5, locals.var_phi_b0_dep_dn6, locals.var_phi_b0_dep_dn7, locals.var_phi_b0_dep_dn8, locals.var_phi_b0_dep_dn9, locals.var_phi_b0_dep_dn10, locals.var_phi_b0_dep_dn13,)
    } else {
        (locals.var_phi_b0_dep_old, locals.var_phi_b0_dep_old_dn0, locals.var_phi_b0_dep_old_dn2, locals.var_phi_b0_dep_old_dn4, locals.var_phi_b0_dep_old_dn5, locals.var_phi_b0_dep_old_dn6, locals.var_phi_b0_dep_old_dn7, locals.var_phi_b0_dep_old_dn8, locals.var_phi_b0_dep_old_dn9, locals.var_phi_b0_dep_old_dn10, locals.var_phi_b0_dep_old_dn13,)
    }
};
        locals.var_phi_b0_dep_old = assign25000_e20657;
        locals.var_phi_b0_dep_old_dn0 = assign25000_e20657_d_n0;
        locals.var_phi_b0_dep_old_dn2 = assign25000_e20657_d_n2;
        locals.var_phi_b0_dep_old_dn4 = assign25000_e20657_d_n4;
        locals.var_phi_b0_dep_old_dn5 = assign25000_e20657_d_n5;
        locals.var_phi_b0_dep_old_dn6 = assign25000_e20657_d_n6;
        locals.var_phi_b0_dep_old_dn7 = assign25000_e20657_d_n7;
        locals.var_phi_b0_dep_old_dn8 = assign25000_e20657_d_n8;
        locals.var_phi_b0_dep_old_dn9 = assign25000_e20657_d_n9;
        locals.var_phi_b0_dep_old_dn10 = assign25000_e20657_d_n10;
        locals.var_phi_b0_dep_old_dn13 = assign25000_e20657_d_n13;
        locals.var_phi_b0_dep_old_rv = 0.0;

        let (assign25010_e20687, assign25010_e20687_d_n0, assign25010_e20687_d_n2, assign25010_e20687_d_n4, assign25010_e20687_d_n5, assign25010_e20687_d_n6, assign25010_e20687_d_n7, assign25010_e20687_d_n8, assign25010_e20687_d_n9, assign25010_e20687_d_n10, assign25010_e20687_d_n13,) = {
    if (((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard578 == 0.0)) && (locals.var_guard580 == 0.0)) && (locals.var_guard581 != 0.0)) {
        let assign25010_e20670: f64 = (-locals.var_bfact);
        let assign25010_e20673: f64 = (locals.var_bfact * locals.var_bfact);
        let assign25010_e20676: f64 = (4.0 * locals.var_afact);
        let assign25010_e20678: f64 = (assign25010_e20676 * locals.var_cfact);
        let assign25010_e20679: f64 = (assign25010_e20673 - assign25010_e20678);
        let assign25010_e20680: f64 = (assign25010_e20679).sqrt();
        let assign25010_e20681: f64 = (assign25010_e20670 + assign25010_e20680);
        let assign25010_e20683: f64 = (assign25010_e20681 / 2.0);
        let assign25010_e20685: f64 = (assign25010_e20683 / locals.var_afact);
        (assign25010_e20685, ((((((-locals.var_bfact_dn0) + ((((locals.var_bfact_dn0 * locals.var_bfact) + (locals.var_bfact * locals.var_bfact_dn0)) - (((4.0 * locals.var_afact_dn0) * locals.var_cfact) + (assign25010_e20676 * locals.var_cfact_dn0))) / (2.0 * assign25010_e20680))) / 2.0) * locals.var_afact) - (assign25010_e20683 * locals.var_afact_dn0)) / (locals.var_afact * locals.var_afact)), ((((((-locals.var_bfact_dn2) + ((((locals.var_bfact_dn2 * locals.var_bfact) + (locals.var_bfact * locals.var_bfact_dn2)) - (((4.0 * locals.var_afact_dn2) * locals.var_cfact) + (assign25010_e20676 * locals.var_cfact_dn2))) / (2.0 * assign25010_e20680))) / 2.0) * locals.var_afact) - (assign25010_e20683 * locals.var_afact_dn2)) / (locals.var_afact * locals.var_afact)), ((((((-locals.var_bfact_dn4) + ((((locals.var_bfact_dn4 * locals.var_bfact) + (locals.var_bfact * locals.var_bfact_dn4)) - (((4.0 * locals.var_afact_dn4) * locals.var_cfact) + (assign25010_e20676 * locals.var_cfact_dn4))) / (2.0 * assign25010_e20680))) / 2.0) * locals.var_afact) - (assign25010_e20683 * locals.var_afact_dn4)) / (locals.var_afact * locals.var_afact)), ((((((-locals.var_bfact_dn5) + ((((locals.var_bfact_dn5 * locals.var_bfact) + (locals.var_bfact * locals.var_bfact_dn5)) - (((4.0 * locals.var_afact_dn5) * locals.var_cfact) + (assign25010_e20676 * locals.var_cfact_dn5))) / (2.0 * assign25010_e20680))) / 2.0) * locals.var_afact) - (assign25010_e20683 * locals.var_afact_dn5)) / (locals.var_afact * locals.var_afact)), ((((((-locals.var_bfact_dn6) + ((((locals.var_bfact_dn6 * locals.var_bfact) + (locals.var_bfact * locals.var_bfact_dn6)) - (((4.0 * locals.var_afact_dn6) * locals.var_cfact) + (assign25010_e20676 * locals.var_cfact_dn6))) / (2.0 * assign25010_e20680))) / 2.0) * locals.var_afact) - (assign25010_e20683 * locals.var_afact_dn6)) / (locals.var_afact * locals.var_afact)), ((((((-locals.var_bfact_dn7) + ((((locals.var_bfact_dn7 * locals.var_bfact) + (locals.var_bfact * locals.var_bfact_dn7)) - (((4.0 * locals.var_afact_dn7) * locals.var_cfact) + (assign25010_e20676 * locals.var_cfact_dn7))) / (2.0 * assign25010_e20680))) / 2.0) * locals.var_afact) - (assign25010_e20683 * locals.var_afact_dn7)) / (locals.var_afact * locals.var_afact)), ((((((-locals.var_bfact_dn8) + ((((locals.var_bfact_dn8 * locals.var_bfact) + (locals.var_bfact * locals.var_bfact_dn8)) - (((4.0 * locals.var_afact_dn8) * locals.var_cfact) + (assign25010_e20676 * locals.var_cfact_dn8))) / (2.0 * assign25010_e20680))) / 2.0) * locals.var_afact) - (assign25010_e20683 * locals.var_afact_dn8)) / (locals.var_afact * locals.var_afact)), ((((((-locals.var_bfact_dn9) + ((((locals.var_bfact_dn9 * locals.var_bfact) + (locals.var_bfact * locals.var_bfact_dn9)) - (((4.0 * locals.var_afact_dn9) * locals.var_cfact) + (assign25010_e20676 * locals.var_cfact_dn9))) / (2.0 * assign25010_e20680))) / 2.0) * locals.var_afact) - (assign25010_e20683 * locals.var_afact_dn9)) / (locals.var_afact * locals.var_afact)), ((((((-locals.var_bfact_dn10) + ((((locals.var_bfact_dn10 * locals.var_bfact) + (locals.var_bfact * locals.var_bfact_dn10)) - (((4.0 * locals.var_afact_dn10) * locals.var_cfact) + (assign25010_e20676 * locals.var_cfact_dn10))) / (2.0 * assign25010_e20680))) / 2.0) * locals.var_afact) - (assign25010_e20683 * locals.var_afact_dn10)) / (locals.var_afact * locals.var_afact)), ((((((-locals.var_bfact_dn13) + ((((locals.var_bfact_dn13 * locals.var_bfact) + (locals.var_bfact * locals.var_bfact_dn13)) - (((4.0 * locals.var_afact_dn13) * locals.var_cfact) + (assign25010_e20676 * locals.var_cfact_dn13))) / (2.0 * assign25010_e20680))) / 2.0) * locals.var_afact) - (assign25010_e20683 * locals.var_afact_dn13)) / (locals.var_afact * locals.var_afact)),)
    } else {
        (locals.var_phi_s0_dep_ini, locals.var_phi_s0_dep_ini_dn0, locals.var_phi_s0_dep_ini_dn2, locals.var_phi_s0_dep_ini_dn4, locals.var_phi_s0_dep_ini_dn5, locals.var_phi_s0_dep_ini_dn6, locals.var_phi_s0_dep_ini_dn7, locals.var_phi_s0_dep_ini_dn8, locals.var_phi_s0_dep_ini_dn9, locals.var_phi_s0_dep_ini_dn10, locals.var_phi_s0_dep_ini_dn13,)
    }
};
        locals.var_phi_s0_dep_ini = assign25010_e20687;
        locals.var_phi_s0_dep_ini_dn0 = assign25010_e20687_d_n0;
        locals.var_phi_s0_dep_ini_dn2 = assign25010_e20687_d_n2;
        locals.var_phi_s0_dep_ini_dn4 = assign25010_e20687_d_n4;
        locals.var_phi_s0_dep_ini_dn5 = assign25010_e20687_d_n5;
        locals.var_phi_s0_dep_ini_dn6 = assign25010_e20687_d_n6;
        locals.var_phi_s0_dep_ini_dn7 = assign25010_e20687_d_n7;
        locals.var_phi_s0_dep_ini_dn8 = assign25010_e20687_d_n8;
        locals.var_phi_s0_dep_ini_dn9 = assign25010_e20687_d_n9;
        locals.var_phi_s0_dep_ini_dn10 = assign25010_e20687_d_n10;
        locals.var_phi_s0_dep_ini_dn13 = assign25010_e20687_d_n13;
        locals.var_phi_s0_dep_ini_rv = 0.0;

        let assign25020_e20691: f64 = (locals.var_psbmax - locals.var_ps_conv3);
        let assign25020_e20692: f64 = if locals.var_phi_s0_dep_ini > assign25020_e20691 { 1.0 } else { 0.0 };
        locals.var_guard582 = assign25020_e20692;
        locals.var_guard582_rv = 0.0;

        let (assign25030_e20710, assign25030_e20710_d_n0, assign25030_e20710_d_n2, assign25030_e20710_d_n4, assign25030_e20710_d_n5, assign25030_e20710_d_n6, assign25030_e20710_d_n7, assign25030_e20710_d_n8, assign25030_e20710_d_n9, assign25030_e20710_d_n10, assign25030_e20710_d_n13,) = {
    if ((((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard578 == 0.0)) && (locals.var_guard580 == 0.0)) && (locals.var_guard581 != 0.0)) && (locals.var_guard582 != 0.0)) {
        let assign25030_e20708: f64 = (locals.var_psbmax - locals.var_ps_conv3);
        (assign25030_e20708, locals.var_psbmax_dn0, locals.var_psbmax_dn2, locals.var_psbmax_dn4, locals.var_psbmax_dn5, locals.var_psbmax_dn6, locals.var_psbmax_dn7, locals.var_psbmax_dn8, locals.var_psbmax_dn9, locals.var_psbmax_dn10, locals.var_psbmax_dn13,)
    } else {
        (locals.var_phi_s0_dep_ini, locals.var_phi_s0_dep_ini_dn0, locals.var_phi_s0_dep_ini_dn2, locals.var_phi_s0_dep_ini_dn4, locals.var_phi_s0_dep_ini_dn5, locals.var_phi_s0_dep_ini_dn6, locals.var_phi_s0_dep_ini_dn7, locals.var_phi_s0_dep_ini_dn8, locals.var_phi_s0_dep_ini_dn9, locals.var_phi_s0_dep_ini_dn10, locals.var_phi_s0_dep_ini_dn13,)
    }
};
        locals.var_phi_s0_dep_ini = assign25030_e20710;
        locals.var_phi_s0_dep_ini_dn0 = assign25030_e20710_d_n0;
        locals.var_phi_s0_dep_ini_dn2 = assign25030_e20710_d_n2;
        locals.var_phi_s0_dep_ini_dn4 = assign25030_e20710_d_n4;
        locals.var_phi_s0_dep_ini_dn5 = assign25030_e20710_d_n5;
        locals.var_phi_s0_dep_ini_dn6 = assign25030_e20710_d_n6;
        locals.var_phi_s0_dep_ini_dn7 = assign25030_e20710_d_n7;
        locals.var_phi_s0_dep_ini_dn8 = assign25030_e20710_d_n8;
        locals.var_phi_s0_dep_ini_dn9 = assign25030_e20710_d_n9;
        locals.var_phi_s0_dep_ini_dn10 = assign25030_e20710_d_n10;
        locals.var_phi_s0_dep_ini_dn13 = assign25030_e20710_d_n13;
        locals.var_phi_s0_dep_ini_rv = 0.0;

        let (assign25040_e20729, assign25040_e20729_d_n0, assign25040_e20729_d_n2, assign25040_e20729_d_n4, assign25040_e20729_d_n5, assign25040_e20729_d_n6, assign25040_e20729_d_n7, assign25040_e20729_d_n8, assign25040_e20729_d_n9, assign25040_e20729_d_n10, assign25040_e20729_d_n13,) = {
    if (((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard578 == 0.0)) && (locals.var_guard580 == 0.0)) && (locals.var_guard581 != 0.0)) {
        let assign25040_e20725: f64 = (locals.var_phi_b0_dep - locals.var_phi_s0_dep_ini);
        let assign25040_e20726: f64 = (locals.var_c_2esipq_ndepm * assign25040_e20725);
        let assign25040_e20727: f64 = (assign25040_e20726).sqrt();
        (assign25040_e20727, (((locals.var_c_2esipq_ndepm_dn0 * assign25040_e20725) + (locals.var_c_2esipq_ndepm * (locals.var_phi_b0_dep_dn0 - locals.var_phi_s0_dep_ini_dn0))) / (2.0 * assign25040_e20727)), (((locals.var_c_2esipq_ndepm_dn2 * assign25040_e20725) + (locals.var_c_2esipq_ndepm * (locals.var_phi_b0_dep_dn2 - locals.var_phi_s0_dep_ini_dn2))) / (2.0 * assign25040_e20727)), (((locals.var_c_2esipq_ndepm_dn4 * assign25040_e20725) + (locals.var_c_2esipq_ndepm * (locals.var_phi_b0_dep_dn4 - locals.var_phi_s0_dep_ini_dn4))) / (2.0 * assign25040_e20727)), (((locals.var_c_2esipq_ndepm_dn5 * assign25040_e20725) + (locals.var_c_2esipq_ndepm * (locals.var_phi_b0_dep_dn5 - locals.var_phi_s0_dep_ini_dn5))) / (2.0 * assign25040_e20727)), (((locals.var_c_2esipq_ndepm_dn6 * assign25040_e20725) + (locals.var_c_2esipq_ndepm * (locals.var_phi_b0_dep_dn6 - locals.var_phi_s0_dep_ini_dn6))) / (2.0 * assign25040_e20727)), (((locals.var_c_2esipq_ndepm_dn7 * assign25040_e20725) + (locals.var_c_2esipq_ndepm * (locals.var_phi_b0_dep_dn7 - locals.var_phi_s0_dep_ini_dn7))) / (2.0 * assign25040_e20727)), (((locals.var_c_2esipq_ndepm_dn8 * assign25040_e20725) + (locals.var_c_2esipq_ndepm * (locals.var_phi_b0_dep_dn8 - locals.var_phi_s0_dep_ini_dn8))) / (2.0 * assign25040_e20727)), (((locals.var_c_2esipq_ndepm_dn9 * assign25040_e20725) + (locals.var_c_2esipq_ndepm * (locals.var_phi_b0_dep_dn9 - locals.var_phi_s0_dep_ini_dn9))) / (2.0 * assign25040_e20727)), (((locals.var_c_2esipq_ndepm_dn10 * assign25040_e20725) + (locals.var_c_2esipq_ndepm * (locals.var_phi_b0_dep_dn10 - locals.var_phi_s0_dep_ini_dn10))) / (2.0 * assign25040_e20727)), (((locals.var_c_2esipq_ndepm_dn13 * assign25040_e20725) + (locals.var_c_2esipq_ndepm * (locals.var_phi_b0_dep_dn13 - locals.var_phi_s0_dep_ini_dn13))) / (2.0 * assign25040_e20727)),)
    } else {
        (locals.var_w_s0, locals.var_w_s0_dn0, locals.var_w_s0_dn2, locals.var_w_s0_dn4, locals.var_w_s0_dn5, locals.var_w_s0_dn6, locals.var_w_s0_dn7, locals.var_w_s0_dn8, locals.var_w_s0_dn9, locals.var_w_s0_dn10, locals.var_w_s0_dn13,)
    }
};
        locals.var_w_s0 = assign25040_e20729;
        locals.var_w_s0_dn0 = assign25040_e20729_d_n0;
        locals.var_w_s0_dn2 = assign25040_e20729_d_n2;
        locals.var_w_s0_dn4 = assign25040_e20729_d_n4;
        locals.var_w_s0_dn5 = assign25040_e20729_d_n5;
        locals.var_w_s0_dn6 = assign25040_e20729_d_n6;
        locals.var_w_s0_dn7 = assign25040_e20729_d_n7;
        locals.var_w_s0_dn8 = assign25040_e20729_d_n8;
        locals.var_w_s0_dn9 = assign25040_e20729_d_n9;
        locals.var_w_s0_dn10 = assign25040_e20729_d_n10;
        locals.var_w_s0_dn13 = assign25040_e20729_d_n13;
        locals.var_w_s0_rv = 0.0;

        let (assign25050_e20748, assign25050_e20748_d_n0, assign25050_e20748_d_n2, assign25050_e20748_d_n4, assign25050_e20748_d_n5, assign25050_e20748_d_n6, assign25050_e20748_d_n7, assign25050_e20748_d_n8, assign25050_e20748_d_n9, assign25050_e20748_d_n10, assign25050_e20748_d_n13,) = {
    if (((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard578 == 0.0)) && (locals.var_guard580 == 0.0)) && (locals.var_guard581 != 0.0)) {
        let assign25050_e20744: f64 = (locals.var_phi_b0_dep - locals.var_phi_j0_dep);
        let assign25050_e20745: f64 = (locals.var_c_2esipq_ndepm * assign25050_e20744);
        let assign25050_e20746: f64 = (assign25050_e20745).sqrt();
        (assign25050_e20746, (((locals.var_c_2esipq_ndepm_dn0 * assign25050_e20744) + (locals.var_c_2esipq_ndepm * (locals.var_phi_b0_dep_dn0 - locals.var_phi_j0_dep_dn0))) / (2.0 * assign25050_e20746)), (((locals.var_c_2esipq_ndepm_dn2 * assign25050_e20744) + (locals.var_c_2esipq_ndepm * (locals.var_phi_b0_dep_dn2 - locals.var_phi_j0_dep_dn2))) / (2.0 * assign25050_e20746)), (((locals.var_c_2esipq_ndepm_dn4 * assign25050_e20744) + (locals.var_c_2esipq_ndepm * (locals.var_phi_b0_dep_dn4 - locals.var_phi_j0_dep_dn4))) / (2.0 * assign25050_e20746)), (((locals.var_c_2esipq_ndepm_dn5 * assign25050_e20744) + (locals.var_c_2esipq_ndepm * (locals.var_phi_b0_dep_dn5 - locals.var_phi_j0_dep_dn5))) / (2.0 * assign25050_e20746)), (((locals.var_c_2esipq_ndepm_dn6 * assign25050_e20744) + (locals.var_c_2esipq_ndepm * (locals.var_phi_b0_dep_dn6 - locals.var_phi_j0_dep_dn6))) / (2.0 * assign25050_e20746)), (((locals.var_c_2esipq_ndepm_dn7 * assign25050_e20744) + (locals.var_c_2esipq_ndepm * (locals.var_phi_b0_dep_dn7 - locals.var_phi_j0_dep_dn7))) / (2.0 * assign25050_e20746)), (((locals.var_c_2esipq_ndepm_dn8 * assign25050_e20744) + (locals.var_c_2esipq_ndepm * (locals.var_phi_b0_dep_dn8 - locals.var_phi_j0_dep_dn8))) / (2.0 * assign25050_e20746)), (((locals.var_c_2esipq_ndepm_dn9 * assign25050_e20744) + (locals.var_c_2esipq_ndepm * (locals.var_phi_b0_dep_dn9 - locals.var_phi_j0_dep_dn9))) / (2.0 * assign25050_e20746)), (((locals.var_c_2esipq_ndepm_dn10 * assign25050_e20744) + (locals.var_c_2esipq_ndepm * (locals.var_phi_b0_dep_dn10 - locals.var_phi_j0_dep_dn10))) / (2.0 * assign25050_e20746)), (((locals.var_c_2esipq_ndepm_dn13 * assign25050_e20744) + (locals.var_c_2esipq_ndepm * (locals.var_phi_b0_dep_dn13 - locals.var_phi_j0_dep_dn13))) / (2.0 * assign25050_e20746)),)
    } else {
        (locals.var_w_b0, locals.var_w_b0_dn0, locals.var_w_b0_dn2, locals.var_w_b0_dn4, locals.var_w_b0_dn5, locals.var_w_b0_dn6, locals.var_w_b0_dn7, locals.var_w_b0_dn8, locals.var_w_b0_dn9, locals.var_w_b0_dn10, locals.var_w_b0_dn13,)
    }
};
        locals.var_w_b0 = assign25050_e20748;
        locals.var_w_b0_dn0 = assign25050_e20748_d_n0;
        locals.var_w_b0_dn2 = assign25050_e20748_d_n2;
        locals.var_w_b0_dn4 = assign25050_e20748_d_n4;
        locals.var_w_b0_dn5 = assign25050_e20748_d_n5;
        locals.var_w_b0_dn6 = assign25050_e20748_d_n6;
        locals.var_w_b0_dn7 = assign25050_e20748_d_n7;
        locals.var_w_b0_dn8 = assign25050_e20748_d_n8;
        locals.var_w_b0_dn9 = assign25050_e20748_d_n9;
        locals.var_w_b0_dn10 = assign25050_e20748_d_n10;
        locals.var_w_b0_dn13 = assign25050_e20748_d_n13;
        locals.var_w_b0_rv = 0.0;

        let assign25060_e20751: f64 = (locals.var_w_s0 + locals.var_w_b0);
        let assign25060_e20753: f64 = if assign25060_e20751 > locals.var_uc_depthn { 1.0 } else { 0.0 };
        locals.var_guard583 = assign25060_e20753;
        locals.var_guard583_rv = 0.0;

        let (assign25070_e20769,) = {
    if ((((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard578 == 0.0)) && (locals.var_guard580 == 0.0)) && (locals.var_guard581 != 0.0)) && (locals.var_guard583 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_lp_s0,)
    }
};
        locals.var_lp_s0 = assign25070_e20769;
        locals.var_lp_s0_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_69(
        locals: &mut StampLocals,
    ) {
        let mut assign25080_loop_guard: usize = 0;
        while {
            let assign25080_cond_e20786: f64 = (150.0 + 1.0);
            let assign25080_cond_e20788: f64 = if (((((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard578 == 0.0)) && (locals.var_guard580 == 0.0)) && (locals.var_guard581 != 0.0)) && (locals.var_guard583 != 0.0)) && (locals.var_lp_s0 <= assign25080_cond_e20786)) { 1.0 } else { 0.0 };
            assign25080_cond_e20788 != 0.0
        } {
            assign25080_loop_guard += 1;
            assert!(assign25080_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign25080_body0_e20808, assign25080_body0_e20808_d_n0, assign25080_body0_e20808_d_n2, assign25080_body0_e20808_d_n4, assign25080_body0_e20808_d_n5, assign25080_body0_e20808_d_n6, assign25080_body0_e20808_d_n7, assign25080_body0_e20808_d_n8, assign25080_body0_e20808_d_n9, assign25080_body0_e20808_d_n10, assign25080_body0_e20808_d_n13,) = {
    if ((((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard578 == 0.0)) && (locals.var_guard580 == 0.0)) && (locals.var_guard581 != 0.0)) && (locals.var_guard583 != 0.0)) {
        let assign25080_body0_e20804: f64 = (locals.var_w_s0 + locals.var_w_b0);
        let assign25080_body0_e20806: f64 = (assign25080_body0_e20804 - locals.var_uc_depthn);
        (assign25080_body0_e20806, ((locals.var_w_s0_dn0 + locals.var_w_b0_dn0) - locals.var_uc_depthn_dn0), ((locals.var_w_s0_dn2 + locals.var_w_b0_dn2) - locals.var_uc_depthn_dn2), ((locals.var_w_s0_dn4 + locals.var_w_b0_dn4) - locals.var_uc_depthn_dn4), ((locals.var_w_s0_dn5 + locals.var_w_b0_dn5) - locals.var_uc_depthn_dn5), ((locals.var_w_s0_dn6 + locals.var_w_b0_dn6) - locals.var_uc_depthn_dn6), ((locals.var_w_s0_dn7 + locals.var_w_b0_dn7) - locals.var_uc_depthn_dn7), ((locals.var_w_s0_dn8 + locals.var_w_b0_dn8) - locals.var_uc_depthn_dn8), ((locals.var_w_s0_dn9 + locals.var_w_b0_dn9) - locals.var_uc_depthn_dn9), ((locals.var_w_s0_dn10 + locals.var_w_b0_dn10) - locals.var_uc_depthn_dn10), ((locals.var_w_s0_dn13 + locals.var_w_b0_dn13) - locals.var_uc_depthn_dn13),)
    } else {
        (locals.var_y0, locals.var_y0_dn0, locals.var_y0_dn2, locals.var_y0_dn4, locals.var_y0_dn5, locals.var_y0_dn6, locals.var_y0_dn7, locals.var_y0_dn8, locals.var_y0_dn9, locals.var_y0_dn10, locals.var_y0_dn13,)
    }
};
            locals.var_y0 = assign25080_body0_e20808;
            locals.var_y0_dn0 = assign25080_body0_e20808_d_n0;
            locals.var_y0_dn2 = assign25080_body0_e20808_d_n2;
            locals.var_y0_dn4 = assign25080_body0_e20808_d_n4;
            locals.var_y0_dn5 = assign25080_body0_e20808_d_n5;
            locals.var_y0_dn6 = assign25080_body0_e20808_d_n6;
            locals.var_y0_dn7 = assign25080_body0_e20808_d_n7;
            locals.var_y0_dn8 = assign25080_body0_e20808_d_n8;
            locals.var_y0_dn9 = assign25080_body0_e20808_d_n9;
            locals.var_y0_dn10 = assign25080_body0_e20808_d_n10;
            locals.var_y0_dn13 = assign25080_body0_e20808_d_n13;
            locals.var_y0_rv = 0.0;
            let (assign25080_body1_e20842, assign25080_body1_e20842_d_n0, assign25080_body1_e20842_d_n2, assign25080_body1_e20842_d_n4, assign25080_body1_e20842_d_n5, assign25080_body1_e20842_d_n6, assign25080_body1_e20842_d_n7, assign25080_body1_e20842_d_n8, assign25080_body1_e20842_d_n9, assign25080_body1_e20842_d_n10, assign25080_body1_e20842_d_n13,) = {
    if ((((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard578 == 0.0)) && (locals.var_guard580 == 0.0)) && (locals.var_guard581 != 0.0)) && (locals.var_guard583 != 0.0)) {
        let assign25080_body1_e20824: f64 = (1.034943e-10 / locals.var_q_ndepm);
        let assign25080_body1_e20826: f64 = (assign25080_body1_e20824 / locals.var_w_s0);
        let assign25080_body1_e20829: f64 = (1.034943e-10 / locals.var_q_ndepm);
        let assign25080_body1_e20834: f64 = (1.0 + locals.var_ndepmpnsub);
        let assign25080_body1_e20835: f64 = (locals.var_ndepmpnsub / assign25080_body1_e20834);
        let assign25080_body1_e20836: f64 = (1.0 - assign25080_body1_e20835);
        let assign25080_body1_e20837: f64 = (assign25080_body1_e20829 * assign25080_body1_e20836);
        let assign25080_body1_e20839: f64 = (assign25080_body1_e20837 / locals.var_w_b0);
        let assign25080_body1_e20840: f64 = (assign25080_body1_e20826 + assign25080_body1_e20839);
        (assign25080_body1_e20840, (((((-((1.034943e-10 * locals.var_q_ndepm_dn0) / (locals.var_q_ndepm * locals.var_q_ndepm))) * locals.var_w_s0) - (assign25080_body1_e20824 * locals.var_w_s0_dn0)) / (locals.var_w_s0 * locals.var_w_s0)) + ((((((-((1.034943e-10 * locals.var_q_ndepm_dn0) / (locals.var_q_ndepm * locals.var_q_ndepm))) * assign25080_body1_e20836) + (assign25080_body1_e20829 * (-(((locals.var_ndepmpnsub_dn0 * assign25080_body1_e20834) - (locals.var_ndepmpnsub * locals.var_ndepmpnsub_dn0)) / (assign25080_body1_e20834 * assign25080_body1_e20834))))) * locals.var_w_b0) - (assign25080_body1_e20837 * locals.var_w_b0_dn0)) / (locals.var_w_b0 * locals.var_w_b0))), (((((-((1.034943e-10 * locals.var_q_ndepm_dn2) / (locals.var_q_ndepm * locals.var_q_ndepm))) * locals.var_w_s0) - (assign25080_body1_e20824 * locals.var_w_s0_dn2)) / (locals.var_w_s0 * locals.var_w_s0)) + ((((((-((1.034943e-10 * locals.var_q_ndepm_dn2) / (locals.var_q_ndepm * locals.var_q_ndepm))) * assign25080_body1_e20836) + (assign25080_body1_e20829 * (-(((locals.var_ndepmpnsub_dn2 * assign25080_body1_e20834) - (locals.var_ndepmpnsub * locals.var_ndepmpnsub_dn2)) / (assign25080_body1_e20834 * assign25080_body1_e20834))))) * locals.var_w_b0) - (assign25080_body1_e20837 * locals.var_w_b0_dn2)) / (locals.var_w_b0 * locals.var_w_b0))), (((((-((1.034943e-10 * locals.var_q_ndepm_dn4) / (locals.var_q_ndepm * locals.var_q_ndepm))) * locals.var_w_s0) - (assign25080_body1_e20824 * locals.var_w_s0_dn4)) / (locals.var_w_s0 * locals.var_w_s0)) + ((((((-((1.034943e-10 * locals.var_q_ndepm_dn4) / (locals.var_q_ndepm * locals.var_q_ndepm))) * assign25080_body1_e20836) + (assign25080_body1_e20829 * (-(((locals.var_ndepmpnsub_dn4 * assign25080_body1_e20834) - (locals.var_ndepmpnsub * locals.var_ndepmpnsub_dn4)) / (assign25080_body1_e20834 * assign25080_body1_e20834))))) * locals.var_w_b0) - (assign25080_body1_e20837 * locals.var_w_b0_dn4)) / (locals.var_w_b0 * locals.var_w_b0))), (((((-((1.034943e-10 * locals.var_q_ndepm_dn5) / (locals.var_q_ndepm * locals.var_q_ndepm))) * locals.var_w_s0) - (assign25080_body1_e20824 * locals.var_w_s0_dn5)) / (locals.var_w_s0 * locals.var_w_s0)) + ((((((-((1.034943e-10 * locals.var_q_ndepm_dn5) / (locals.var_q_ndepm * locals.var_q_ndepm))) * assign25080_body1_e20836) + (assign25080_body1_e20829 * (-(((locals.var_ndepmpnsub_dn5 * assign25080_body1_e20834) - (locals.var_ndepmpnsub * locals.var_ndepmpnsub_dn5)) / (assign25080_body1_e20834 * assign25080_body1_e20834))))) * locals.var_w_b0) - (assign25080_body1_e20837 * locals.var_w_b0_dn5)) / (locals.var_w_b0 * locals.var_w_b0))), (((((-((1.034943e-10 * locals.var_q_ndepm_dn6) / (locals.var_q_ndepm * locals.var_q_ndepm))) * locals.var_w_s0) - (assign25080_body1_e20824 * locals.var_w_s0_dn6)) / (locals.var_w_s0 * locals.var_w_s0)) + ((((((-((1.034943e-10 * locals.var_q_ndepm_dn6) / (locals.var_q_ndepm * locals.var_q_ndepm))) * assign25080_body1_e20836) + (assign25080_body1_e20829 * (-(((locals.var_ndepmpnsub_dn6 * assign25080_body1_e20834) - (locals.var_ndepmpnsub * locals.var_ndepmpnsub_dn6)) / (assign25080_body1_e20834 * assign25080_body1_e20834))))) * locals.var_w_b0) - (assign25080_body1_e20837 * locals.var_w_b0_dn6)) / (locals.var_w_b0 * locals.var_w_b0))), (((((-((1.034943e-10 * locals.var_q_ndepm_dn7) / (locals.var_q_ndepm * locals.var_q_ndepm))) * locals.var_w_s0) - (assign25080_body1_e20824 * locals.var_w_s0_dn7)) / (locals.var_w_s0 * locals.var_w_s0)) + ((((((-((1.034943e-10 * locals.var_q_ndepm_dn7) / (locals.var_q_ndepm * locals.var_q_ndepm))) * assign25080_body1_e20836) + (assign25080_body1_e20829 * (-(((locals.var_ndepmpnsub_dn7 * assign25080_body1_e20834) - (locals.var_ndepmpnsub * locals.var_ndepmpnsub_dn7)) / (assign25080_body1_e20834 * assign25080_body1_e20834))))) * locals.var_w_b0) - (assign25080_body1_e20837 * locals.var_w_b0_dn7)) / (locals.var_w_b0 * locals.var_w_b0))), (((((-((1.034943e-10 * locals.var_q_ndepm_dn8) / (locals.var_q_ndepm * locals.var_q_ndepm))) * locals.var_w_s0) - (assign25080_body1_e20824 * locals.var_w_s0_dn8)) / (locals.var_w_s0 * locals.var_w_s0)) + ((((((-((1.034943e-10 * locals.var_q_ndepm_dn8) / (locals.var_q_ndepm * locals.var_q_ndepm))) * assign25080_body1_e20836) + (assign25080_body1_e20829 * (-(((locals.var_ndepmpnsub_dn8 * assign25080_body1_e20834) - (locals.var_ndepmpnsub * locals.var_ndepmpnsub_dn8)) / (assign25080_body1_e20834 * assign25080_body1_e20834))))) * locals.var_w_b0) - (assign25080_body1_e20837 * locals.var_w_b0_dn8)) / (locals.var_w_b0 * locals.var_w_b0))), (((((-((1.034943e-10 * locals.var_q_ndepm_dn9) / (locals.var_q_ndepm * locals.var_q_ndepm))) * locals.var_w_s0) - (assign25080_body1_e20824 * locals.var_w_s0_dn9)) / (locals.var_w_s0 * locals.var_w_s0)) + ((((((-((1.034943e-10 * locals.var_q_ndepm_dn9) / (locals.var_q_ndepm * locals.var_q_ndepm))) * assign25080_body1_e20836) + (assign25080_body1_e20829 * (-(((locals.var_ndepmpnsub_dn9 * assign25080_body1_e20834) - (locals.var_ndepmpnsub * locals.var_ndepmpnsub_dn9)) / (assign25080_body1_e20834 * assign25080_body1_e20834))))) * locals.var_w_b0) - (assign25080_body1_e20837 * locals.var_w_b0_dn9)) / (locals.var_w_b0 * locals.var_w_b0))), (((((-((1.034943e-10 * locals.var_q_ndepm_dn10) / (locals.var_q_ndepm * locals.var_q_ndepm))) * locals.var_w_s0) - (assign25080_body1_e20824 * locals.var_w_s0_dn10)) / (locals.var_w_s0 * locals.var_w_s0)) + ((((((-((1.034943e-10 * locals.var_q_ndepm_dn10) / (locals.var_q_ndepm * locals.var_q_ndepm))) * assign25080_body1_e20836) + (assign25080_body1_e20829 * (-(((locals.var_ndepmpnsub_dn10 * assign25080_body1_e20834) - (locals.var_ndepmpnsub * locals.var_ndepmpnsub_dn10)) / (assign25080_body1_e20834 * assign25080_body1_e20834))))) * locals.var_w_b0) - (assign25080_body1_e20837 * locals.var_w_b0_dn10)) / (locals.var_w_b0 * locals.var_w_b0))), (((((-((1.034943e-10 * locals.var_q_ndepm_dn13) / (locals.var_q_ndepm * locals.var_q_ndepm))) * locals.var_w_s0) - (assign25080_body1_e20824 * locals.var_w_s0_dn13)) / (locals.var_w_s0 * locals.var_w_s0)) + ((((((-((1.034943e-10 * locals.var_q_ndepm_dn13) / (locals.var_q_ndepm * locals.var_q_ndepm))) * assign25080_body1_e20836) + (assign25080_body1_e20829 * (-(((locals.var_ndepmpnsub_dn13 * assign25080_body1_e20834) - (locals.var_ndepmpnsub * locals.var_ndepmpnsub_dn13)) / (assign25080_body1_e20834 * assign25080_body1_e20834))))) * locals.var_w_b0) - (assign25080_body1_e20837 * locals.var_w_b0_dn13)) / (locals.var_w_b0 * locals.var_w_b0))),)
    } else {
        (locals.var_dydpsm, locals.var_dydpsm_dn0, locals.var_dydpsm_dn2, locals.var_dydpsm_dn4, locals.var_dydpsm_dn5, locals.var_dydpsm_dn6, locals.var_dydpsm_dn7, locals.var_dydpsm_dn8, locals.var_dydpsm_dn9, locals.var_dydpsm_dn10, locals.var_dydpsm_dn13,)
    }
};
            locals.var_dydpsm = assign25080_body1_e20842;
            locals.var_dydpsm_dn0 = assign25080_body1_e20842_d_n0;
            locals.var_dydpsm_dn2 = assign25080_body1_e20842_d_n2;
            locals.var_dydpsm_dn4 = assign25080_body1_e20842_d_n4;
            locals.var_dydpsm_dn5 = assign25080_body1_e20842_d_n5;
            locals.var_dydpsm_dn6 = assign25080_body1_e20842_d_n6;
            locals.var_dydpsm_dn7 = assign25080_body1_e20842_d_n7;
            locals.var_dydpsm_dn8 = assign25080_body1_e20842_d_n8;
            locals.var_dydpsm_dn9 = assign25080_body1_e20842_d_n9;
            locals.var_dydpsm_dn10 = assign25080_body1_e20842_d_n10;
            locals.var_dydpsm_dn13 = assign25080_body1_e20842_d_n13;
            locals.var_dydpsm_rv = 0.0;
            let assign25080_body2_e20845: f64 = (locals.var_y0 / locals.var_dydpsm);
            let assign25080_body2_e20846: f64 = (assign25080_body2_e20845).abs();
            let assign25080_body2_e20848: f64 = if assign25080_body2_e20846 > 0.5 { 1.0 } else { 0.0 };
            locals.var_guard584 = assign25080_body2_e20848;
            locals.var_guard584_rv = 0.0;
            let (assign25080_body3_e20878, assign25080_body3_e20878_d_n0, assign25080_body3_e20878_d_n2, assign25080_body3_e20878_d_n4, assign25080_body3_e20878_d_n5, assign25080_body3_e20878_d_n6, assign25080_body3_e20878_d_n7, assign25080_body3_e20878_d_n8, assign25080_body3_e20878_d_n9, assign25080_body3_e20878_d_n10, assign25080_body3_e20878_d_n13,) = {
    if (((((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard578 == 0.0)) && (locals.var_guard580 == 0.0)) && (locals.var_guard581 != 0.0)) && (locals.var_guard583 != 0.0)) && (locals.var_guard584 != 0.0)) {
        let assign25080_body3_e20868: f64 = (locals.var_y0 / locals.var_dydpsm);
        let (assign25080_body3_e20874,) = {
            if (assign25080_body3_e20868 >= 0.0) {
                (1.0,)
            } else {
                let assign25080_body3_e20873: f64 = (-1.0);
                (assign25080_body3_e20873,)
            }
        };
        let assign25080_body3_e20875: f64 = (0.5 * assign25080_body3_e20874);
        let assign25080_body3_e20876: f64 = (locals.var_phi_b0_dep - assign25080_body3_e20875);
        (assign25080_body3_e20876, locals.var_phi_b0_dep_dn0, locals.var_phi_b0_dep_dn2, locals.var_phi_b0_dep_dn4, locals.var_phi_b0_dep_dn5, locals.var_phi_b0_dep_dn6, locals.var_phi_b0_dep_dn7, locals.var_phi_b0_dep_dn8, locals.var_phi_b0_dep_dn9, locals.var_phi_b0_dep_dn10, locals.var_phi_b0_dep_dn13,)
    } else {
        (locals.var_phi_b0_dep, locals.var_phi_b0_dep_dn0, locals.var_phi_b0_dep_dn2, locals.var_phi_b0_dep_dn4, locals.var_phi_b0_dep_dn5, locals.var_phi_b0_dep_dn6, locals.var_phi_b0_dep_dn7, locals.var_phi_b0_dep_dn8, locals.var_phi_b0_dep_dn9, locals.var_phi_b0_dep_dn10, locals.var_phi_b0_dep_dn13,)
    }
};
            locals.var_phi_b0_dep = assign25080_body3_e20878;
            locals.var_phi_b0_dep_dn0 = assign25080_body3_e20878_d_n0;
            locals.var_phi_b0_dep_dn2 = assign25080_body3_e20878_d_n2;
            locals.var_phi_b0_dep_dn4 = assign25080_body3_e20878_d_n4;
            locals.var_phi_b0_dep_dn5 = assign25080_body3_e20878_d_n5;
            locals.var_phi_b0_dep_dn6 = assign25080_body3_e20878_d_n6;
            locals.var_phi_b0_dep_dn7 = assign25080_body3_e20878_d_n7;
            locals.var_phi_b0_dep_dn8 = assign25080_body3_e20878_d_n8;
            locals.var_phi_b0_dep_dn9 = assign25080_body3_e20878_d_n9;
            locals.var_phi_b0_dep_dn10 = assign25080_body3_e20878_d_n10;
            locals.var_phi_b0_dep_dn13 = assign25080_body3_e20878_d_n13;
            locals.var_phi_b0_dep_rv = 0.0;
            let (assign25080_body4_e20901, assign25080_body4_e20901_d_n0, assign25080_body4_e20901_d_n2, assign25080_body4_e20901_d_n4, assign25080_body4_e20901_d_n5, assign25080_body4_e20901_d_n6, assign25080_body4_e20901_d_n7, assign25080_body4_e20901_d_n8, assign25080_body4_e20901_d_n9, assign25080_body4_e20901_d_n10, assign25080_body4_e20901_d_n13,) = {
    if (((((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard578 == 0.0)) && (locals.var_guard580 == 0.0)) && (locals.var_guard581 != 0.0)) && (locals.var_guard583 != 0.0)) && (locals.var_guard584 == 0.0)) {
        let assign25080_body4_e20898: f64 = (locals.var_y0 / locals.var_dydpsm);
        let assign25080_body4_e20899: f64 = (locals.var_phi_b0_dep - assign25080_body4_e20898);
        (assign25080_body4_e20899, (locals.var_phi_b0_dep_dn0 - (((locals.var_y0_dn0 * locals.var_dydpsm) - (locals.var_y0 * locals.var_dydpsm_dn0)) / (locals.var_dydpsm * locals.var_dydpsm))), (locals.var_phi_b0_dep_dn2 - (((locals.var_y0_dn2 * locals.var_dydpsm) - (locals.var_y0 * locals.var_dydpsm_dn2)) / (locals.var_dydpsm * locals.var_dydpsm))), (locals.var_phi_b0_dep_dn4 - (((locals.var_y0_dn4 * locals.var_dydpsm) - (locals.var_y0 * locals.var_dydpsm_dn4)) / (locals.var_dydpsm * locals.var_dydpsm))), (locals.var_phi_b0_dep_dn5 - (((locals.var_y0_dn5 * locals.var_dydpsm) - (locals.var_y0 * locals.var_dydpsm_dn5)) / (locals.var_dydpsm * locals.var_dydpsm))), (locals.var_phi_b0_dep_dn6 - (((locals.var_y0_dn6 * locals.var_dydpsm) - (locals.var_y0 * locals.var_dydpsm_dn6)) / (locals.var_dydpsm * locals.var_dydpsm))), (locals.var_phi_b0_dep_dn7 - (((locals.var_y0_dn7 * locals.var_dydpsm) - (locals.var_y0 * locals.var_dydpsm_dn7)) / (locals.var_dydpsm * locals.var_dydpsm))), (locals.var_phi_b0_dep_dn8 - (((locals.var_y0_dn8 * locals.var_dydpsm) - (locals.var_y0 * locals.var_dydpsm_dn8)) / (locals.var_dydpsm * locals.var_dydpsm))), (locals.var_phi_b0_dep_dn9 - (((locals.var_y0_dn9 * locals.var_dydpsm) - (locals.var_y0 * locals.var_dydpsm_dn9)) / (locals.var_dydpsm * locals.var_dydpsm))), (locals.var_phi_b0_dep_dn10 - (((locals.var_y0_dn10 * locals.var_dydpsm) - (locals.var_y0 * locals.var_dydpsm_dn10)) / (locals.var_dydpsm * locals.var_dydpsm))), (locals.var_phi_b0_dep_dn13 - (((locals.var_y0_dn13 * locals.var_dydpsm) - (locals.var_y0 * locals.var_dydpsm_dn13)) / (locals.var_dydpsm * locals.var_dydpsm))),)
    } else {
        (locals.var_phi_b0_dep, locals.var_phi_b0_dep_dn0, locals.var_phi_b0_dep_dn2, locals.var_phi_b0_dep_dn4, locals.var_phi_b0_dep_dn5, locals.var_phi_b0_dep_dn6, locals.var_phi_b0_dep_dn7, locals.var_phi_b0_dep_dn8, locals.var_phi_b0_dep_dn9, locals.var_phi_b0_dep_dn10, locals.var_phi_b0_dep_dn13,)
    }
};
            locals.var_phi_b0_dep = assign25080_body4_e20901;
            locals.var_phi_b0_dep_dn0 = assign25080_body4_e20901_d_n0;
            locals.var_phi_b0_dep_dn2 = assign25080_body4_e20901_d_n2;
            locals.var_phi_b0_dep_dn4 = assign25080_body4_e20901_d_n4;
            locals.var_phi_b0_dep_dn5 = assign25080_body4_e20901_d_n5;
            locals.var_phi_b0_dep_dn6 = assign25080_body4_e20901_d_n6;
            locals.var_phi_b0_dep_dn7 = assign25080_body4_e20901_d_n7;
            locals.var_phi_b0_dep_dn8 = assign25080_body4_e20901_d_n8;
            locals.var_phi_b0_dep_dn9 = assign25080_body4_e20901_d_n9;
            locals.var_phi_b0_dep_dn10 = assign25080_body4_e20901_d_n10;
            locals.var_phi_b0_dep_dn13 = assign25080_body4_e20901_d_n13;
            locals.var_phi_b0_dep_rv = 0.0;
            let assign25080_body5_e20904: f64 = (locals.var_phi_b0_dep - locals.var_vbscl__blk435);
            let assign25080_body5_e20906: f64 = (assign25080_body5_e20904 + locals.var_vbi_dep);
            let assign25080_body5_e20909: f64 = (10.0 * 2.220446049250313e-16);
            let assign25080_body5_e20910: f64 = if assign25080_body5_e20906 < assign25080_body5_e20909 { 1.0 } else { 0.0 };
            locals.var_guard585 = assign25080_body5_e20910;
            locals.var_guard585_rv = 0.0;
            let (assign25080_body6_e20934, assign25080_body6_e20934_d_n0, assign25080_body6_e20934_d_n2, assign25080_body6_e20934_d_n4, assign25080_body6_e20934_d_n5, assign25080_body6_e20934_d_n6, assign25080_body6_e20934_d_n7, assign25080_body6_e20934_d_n8, assign25080_body6_e20934_d_n9, assign25080_body6_e20934_d_n10, assign25080_body6_e20934_d_n13,) = {
    if (((((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard578 == 0.0)) && (locals.var_guard580 == 0.0)) && (locals.var_guard581 != 0.0)) && (locals.var_guard583 != 0.0)) && (locals.var_guard585 != 0.0)) {
        let assign25080_body6_e20928: f64 = (locals.var_vbscl__blk435 - locals.var_vbi_dep);
        let assign25080_body6_e20931: f64 = (10.0 * 2.220446049250313e-16);
        let assign25080_body6_e20932: f64 = (assign25080_body6_e20928 + assign25080_body6_e20931);
        (assign25080_body6_e20932, (locals.var_vbscl__blk435_dn0 - locals.var_vbi_dep_dn0), (locals.var_vbscl__blk435_dn2 - locals.var_vbi_dep_dn2), (locals.var_vbscl__blk435_dn4 - locals.var_vbi_dep_dn4), (locals.var_vbscl__blk435_dn5 - locals.var_vbi_dep_dn5), (locals.var_vbscl__blk435_dn6 - locals.var_vbi_dep_dn6), (locals.var_vbscl__blk435_dn7 - locals.var_vbi_dep_dn7), (locals.var_vbscl__blk435_dn8 - locals.var_vbi_dep_dn8), (locals.var_vbscl__blk435_dn9 - locals.var_vbi_dep_dn9), (locals.var_vbscl__blk435_dn10 - locals.var_vbi_dep_dn10), (locals.var_vbscl__blk435_dn13 - locals.var_vbi_dep_dn13),)
    } else {
        (locals.var_phi_b0_dep, locals.var_phi_b0_dep_dn0, locals.var_phi_b0_dep_dn2, locals.var_phi_b0_dep_dn4, locals.var_phi_b0_dep_dn5, locals.var_phi_b0_dep_dn6, locals.var_phi_b0_dep_dn7, locals.var_phi_b0_dep_dn8, locals.var_phi_b0_dep_dn9, locals.var_phi_b0_dep_dn10, locals.var_phi_b0_dep_dn13,)
    }
};
            locals.var_phi_b0_dep = assign25080_body6_e20934;
            locals.var_phi_b0_dep_dn0 = assign25080_body6_e20934_d_n0;
            locals.var_phi_b0_dep_dn2 = assign25080_body6_e20934_d_n2;
            locals.var_phi_b0_dep_dn4 = assign25080_body6_e20934_d_n4;
            locals.var_phi_b0_dep_dn5 = assign25080_body6_e20934_d_n5;
            locals.var_phi_b0_dep_dn6 = assign25080_body6_e20934_d_n6;
            locals.var_phi_b0_dep_dn7 = assign25080_body6_e20934_d_n7;
            locals.var_phi_b0_dep_dn8 = assign25080_body6_e20934_d_n8;
            locals.var_phi_b0_dep_dn9 = assign25080_body6_e20934_d_n9;
            locals.var_phi_b0_dep_dn10 = assign25080_body6_e20934_d_n10;
            locals.var_phi_b0_dep_dn13 = assign25080_body6_e20934_d_n13;
            locals.var_phi_b0_dep_rv = 0.0;
            let (assign25080_body7_e20958, assign25080_body7_e20958_d_n0, assign25080_body7_e20958_d_n2, assign25080_body7_e20958_d_n4, assign25080_body7_e20958_d_n5, assign25080_body7_e20958_d_n6, assign25080_body7_e20958_d_n7, assign25080_body7_e20958_d_n8, assign25080_body7_e20958_d_n9, assign25080_body7_e20958_d_n10, assign25080_body7_e20958_d_n13,) = {
    if ((((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard578 == 0.0)) && (locals.var_guard580 == 0.0)) && (locals.var_guard581 != 0.0)) && (locals.var_guard583 != 0.0)) {
        let assign25080_body7_e20950: f64 = (locals.var_afact * locals.var_vgp);
        let assign25080_body7_e20952: f64 = (assign25080_body7_e20950 * locals.var_vgp);
        let assign25080_body7_e20955: f64 = (locals.var_beta * locals.var_phi_b0_dep);
        let assign25080_body7_e20956: f64 = (assign25080_body7_e20952 - assign25080_body7_e20955);
        (assign25080_body7_e20956, (((((locals.var_afact_dn0 * locals.var_vgp) + (locals.var_afact * locals.var_vgp_dn0)) * locals.var_vgp) + (assign25080_body7_e20950 * locals.var_vgp_dn0)) - ((locals.var_beta_dn0 * locals.var_phi_b0_dep) + (locals.var_beta * locals.var_phi_b0_dep_dn0))), (((((locals.var_afact_dn2 * locals.var_vgp) + (locals.var_afact * locals.var_vgp_dn2)) * locals.var_vgp) + (assign25080_body7_e20950 * locals.var_vgp_dn2)) - ((locals.var_beta_dn2 * locals.var_phi_b0_dep) + (locals.var_beta * locals.var_phi_b0_dep_dn2))), (((((locals.var_afact_dn4 * locals.var_vgp) + (locals.var_afact * locals.var_vgp_dn4)) * locals.var_vgp) + (assign25080_body7_e20950 * locals.var_vgp_dn4)) - ((locals.var_beta_dn4 * locals.var_phi_b0_dep) + (locals.var_beta * locals.var_phi_b0_dep_dn4))), (((((locals.var_afact_dn5 * locals.var_vgp) + (locals.var_afact * locals.var_vgp_dn5)) * locals.var_vgp) + (assign25080_body7_e20950 * locals.var_vgp_dn5)) - ((locals.var_beta_dn5 * locals.var_phi_b0_dep) + (locals.var_beta * locals.var_phi_b0_dep_dn5))), (((((locals.var_afact_dn6 * locals.var_vgp) + (locals.var_afact * locals.var_vgp_dn6)) * locals.var_vgp) + (assign25080_body7_e20950 * locals.var_vgp_dn6)) - ((locals.var_beta_dn6 * locals.var_phi_b0_dep) + (locals.var_beta * locals.var_phi_b0_dep_dn6))), (((((locals.var_afact_dn7 * locals.var_vgp) + (locals.var_afact * locals.var_vgp_dn7)) * locals.var_vgp) + (assign25080_body7_e20950 * locals.var_vgp_dn7)) - ((locals.var_beta_dn7 * locals.var_phi_b0_dep) + (locals.var_beta * locals.var_phi_b0_dep_dn7))), (((((locals.var_afact_dn8 * locals.var_vgp) + (locals.var_afact * locals.var_vgp_dn8)) * locals.var_vgp) + (assign25080_body7_e20950 * locals.var_vgp_dn8)) - ((locals.var_beta_dn8 * locals.var_phi_b0_dep) + (locals.var_beta * locals.var_phi_b0_dep_dn8))), (((((locals.var_afact_dn9 * locals.var_vgp) + (locals.var_afact * locals.var_vgp_dn9)) * locals.var_vgp) + (assign25080_body7_e20950 * locals.var_vgp_dn9)) - ((locals.var_beta_dn9 * locals.var_phi_b0_dep) + (locals.var_beta * locals.var_phi_b0_dep_dn9))), (((((locals.var_afact_dn10 * locals.var_vgp) + (locals.var_afact * locals.var_vgp_dn10)) * locals.var_vgp) + (assign25080_body7_e20950 * locals.var_vgp_dn10)) - ((locals.var_beta_dn10 * locals.var_phi_b0_dep) + (locals.var_beta * locals.var_phi_b0_dep_dn10))), (((((locals.var_afact_dn13 * locals.var_vgp) + (locals.var_afact * locals.var_vgp_dn13)) * locals.var_vgp) + (assign25080_body7_e20950 * locals.var_vgp_dn13)) - ((locals.var_beta_dn13 * locals.var_phi_b0_dep) + (locals.var_beta * locals.var_phi_b0_dep_dn13))),)
    } else {
        (locals.var_cfact, locals.var_cfact_dn0, locals.var_cfact_dn2, locals.var_cfact_dn4, locals.var_cfact_dn5, locals.var_cfact_dn6, locals.var_cfact_dn7, locals.var_cfact_dn8, locals.var_cfact_dn9, locals.var_cfact_dn10, locals.var_cfact_dn13,)
    }
};
            locals.var_cfact = assign25080_body7_e20958;
            locals.var_cfact_dn0 = assign25080_body7_e20958_d_n0;
            locals.var_cfact_dn2 = assign25080_body7_e20958_d_n2;
            locals.var_cfact_dn4 = assign25080_body7_e20958_d_n4;
            locals.var_cfact_dn5 = assign25080_body7_e20958_d_n5;
            locals.var_cfact_dn6 = assign25080_body7_e20958_d_n6;
            locals.var_cfact_dn7 = assign25080_body7_e20958_d_n7;
            locals.var_cfact_dn8 = assign25080_body7_e20958_d_n8;
            locals.var_cfact_dn9 = assign25080_body7_e20958_d_n9;
            locals.var_cfact_dn10 = assign25080_body7_e20958_d_n10;
            locals.var_cfact_dn13 = assign25080_body7_e20958_d_n13;
            locals.var_cfact_rv = 0.0;
            let (assign25080_body8_e20982, assign25080_body8_e20982_d_n0, assign25080_body8_e20982_d_n2, assign25080_body8_e20982_d_n4, assign25080_body8_e20982_d_n5, assign25080_body8_e20982_d_n6, assign25080_body8_e20982_d_n7, assign25080_body8_e20982_d_n8, assign25080_body8_e20982_d_n9, assign25080_body8_e20982_d_n10, assign25080_body8_e20982_d_n13,) = {
    if ((((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard578 == 0.0)) && (locals.var_guard580 == 0.0)) && (locals.var_guard581 != 0.0)) && (locals.var_guard583 != 0.0)) {
        let assign25080_body8_e20974: f64 = (locals.var_bfact * locals.var_bfact);
        let assign25080_body8_e20977: f64 = (4.0 * locals.var_afact);
        let assign25080_body8_e20979: f64 = (assign25080_body8_e20977 * locals.var_cfact);
        let assign25080_body8_e20980: f64 = (assign25080_body8_e20974 - assign25080_body8_e20979);
        (assign25080_body8_e20980, (((locals.var_bfact_dn0 * locals.var_bfact) + (locals.var_bfact * locals.var_bfact_dn0)) - (((4.0 * locals.var_afact_dn0) * locals.var_cfact) + (assign25080_body8_e20977 * locals.var_cfact_dn0))), (((locals.var_bfact_dn2 * locals.var_bfact) + (locals.var_bfact * locals.var_bfact_dn2)) - (((4.0 * locals.var_afact_dn2) * locals.var_cfact) + (assign25080_body8_e20977 * locals.var_cfact_dn2))), (((locals.var_bfact_dn4 * locals.var_bfact) + (locals.var_bfact * locals.var_bfact_dn4)) - (((4.0 * locals.var_afact_dn4) * locals.var_cfact) + (assign25080_body8_e20977 * locals.var_cfact_dn4))), (((locals.var_bfact_dn5 * locals.var_bfact) + (locals.var_bfact * locals.var_bfact_dn5)) - (((4.0 * locals.var_afact_dn5) * locals.var_cfact) + (assign25080_body8_e20977 * locals.var_cfact_dn5))), (((locals.var_bfact_dn6 * locals.var_bfact) + (locals.var_bfact * locals.var_bfact_dn6)) - (((4.0 * locals.var_afact_dn6) * locals.var_cfact) + (assign25080_body8_e20977 * locals.var_cfact_dn6))), (((locals.var_bfact_dn7 * locals.var_bfact) + (locals.var_bfact * locals.var_bfact_dn7)) - (((4.0 * locals.var_afact_dn7) * locals.var_cfact) + (assign25080_body8_e20977 * locals.var_cfact_dn7))), (((locals.var_bfact_dn8 * locals.var_bfact) + (locals.var_bfact * locals.var_bfact_dn8)) - (((4.0 * locals.var_afact_dn8) * locals.var_cfact) + (assign25080_body8_e20977 * locals.var_cfact_dn8))), (((locals.var_bfact_dn9 * locals.var_bfact) + (locals.var_bfact * locals.var_bfact_dn9)) - (((4.0 * locals.var_afact_dn9) * locals.var_cfact) + (assign25080_body8_e20977 * locals.var_cfact_dn9))), (((locals.var_bfact_dn10 * locals.var_bfact) + (locals.var_bfact * locals.var_bfact_dn10)) - (((4.0 * locals.var_afact_dn10) * locals.var_cfact) + (assign25080_body8_e20977 * locals.var_cfact_dn10))), (((locals.var_bfact_dn13 * locals.var_bfact) + (locals.var_bfact * locals.var_bfact_dn13)) - (((4.0 * locals.var_afact_dn13) * locals.var_cfact) + (assign25080_body8_e20977 * locals.var_cfact_dn13))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
            locals.var_t1 = assign25080_body8_e20982;
            locals.var_t1_dn0 = assign25080_body8_e20982_d_n0;
            locals.var_t1_dn2 = assign25080_body8_e20982_d_n2;
            locals.var_t1_dn4 = assign25080_body8_e20982_d_n4;
            locals.var_t1_dn5 = assign25080_body8_e20982_d_n5;
            locals.var_t1_dn6 = assign25080_body8_e20982_d_n6;
            locals.var_t1_dn7 = assign25080_body8_e20982_d_n7;
            locals.var_t1_dn8 = assign25080_body8_e20982_d_n8;
            locals.var_t1_dn9 = assign25080_body8_e20982_d_n9;
            locals.var_t1_dn10 = assign25080_body8_e20982_d_n10;
            locals.var_t1_dn13 = assign25080_body8_e20982_d_n13;
            locals.var_t1_rv = 0.0;
            let assign25080_body9_e20985: f64 = if locals.var_t1 > 0.0 { 1.0 } else { 0.0 };
            locals.var_guard586 = assign25080_body9_e20985;
            locals.var_guard586_rv = 0.0;
            let (assign25080_body10_e21011, assign25080_body10_e21011_d_n0, assign25080_body10_e21011_d_n2, assign25080_body10_e21011_d_n4, assign25080_body10_e21011_d_n5, assign25080_body10_e21011_d_n6, assign25080_body10_e21011_d_n7, assign25080_body10_e21011_d_n8, assign25080_body10_e21011_d_n9, assign25080_body10_e21011_d_n10, assign25080_body10_e21011_d_n13,) = {
    if (((((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard578 == 0.0)) && (locals.var_guard580 == 0.0)) && (locals.var_guard581 != 0.0)) && (locals.var_guard583 != 0.0)) && (locals.var_guard586 != 0.0)) {
        let assign25080_body10_e21002: f64 = (-locals.var_bfact);
        let assign25080_body10_e21004: f64 = (locals.var_t1).sqrt();
        let assign25080_body10_e21005: f64 = (assign25080_body10_e21002 + assign25080_body10_e21004);
        let assign25080_body10_e21007: f64 = (assign25080_body10_e21005 / 2.0);
        let assign25080_body10_e21009: f64 = (assign25080_body10_e21007 / locals.var_afact);
        (assign25080_body10_e21009, ((((((-locals.var_bfact_dn0) + (locals.var_t1_dn0 / (2.0 * assign25080_body10_e21004))) / 2.0) * locals.var_afact) - (assign25080_body10_e21007 * locals.var_afact_dn0)) / (locals.var_afact * locals.var_afact)), ((((((-locals.var_bfact_dn2) + (locals.var_t1_dn2 / (2.0 * assign25080_body10_e21004))) / 2.0) * locals.var_afact) - (assign25080_body10_e21007 * locals.var_afact_dn2)) / (locals.var_afact * locals.var_afact)), ((((((-locals.var_bfact_dn4) + (locals.var_t1_dn4 / (2.0 * assign25080_body10_e21004))) / 2.0) * locals.var_afact) - (assign25080_body10_e21007 * locals.var_afact_dn4)) / (locals.var_afact * locals.var_afact)), ((((((-locals.var_bfact_dn5) + (locals.var_t1_dn5 / (2.0 * assign25080_body10_e21004))) / 2.0) * locals.var_afact) - (assign25080_body10_e21007 * locals.var_afact_dn5)) / (locals.var_afact * locals.var_afact)), ((((((-locals.var_bfact_dn6) + (locals.var_t1_dn6 / (2.0 * assign25080_body10_e21004))) / 2.0) * locals.var_afact) - (assign25080_body10_e21007 * locals.var_afact_dn6)) / (locals.var_afact * locals.var_afact)), ((((((-locals.var_bfact_dn7) + (locals.var_t1_dn7 / (2.0 * assign25080_body10_e21004))) / 2.0) * locals.var_afact) - (assign25080_body10_e21007 * locals.var_afact_dn7)) / (locals.var_afact * locals.var_afact)), ((((((-locals.var_bfact_dn8) + (locals.var_t1_dn8 / (2.0 * assign25080_body10_e21004))) / 2.0) * locals.var_afact) - (assign25080_body10_e21007 * locals.var_afact_dn8)) / (locals.var_afact * locals.var_afact)), ((((((-locals.var_bfact_dn9) + (locals.var_t1_dn9 / (2.0 * assign25080_body10_e21004))) / 2.0) * locals.var_afact) - (assign25080_body10_e21007 * locals.var_afact_dn9)) / (locals.var_afact * locals.var_afact)), ((((((-locals.var_bfact_dn10) + (locals.var_t1_dn10 / (2.0 * assign25080_body10_e21004))) / 2.0) * locals.var_afact) - (assign25080_body10_e21007 * locals.var_afact_dn10)) / (locals.var_afact * locals.var_afact)), ((((((-locals.var_bfact_dn13) + (locals.var_t1_dn13 / (2.0 * assign25080_body10_e21004))) / 2.0) * locals.var_afact) - (assign25080_body10_e21007 * locals.var_afact_dn13)) / (locals.var_afact * locals.var_afact)),)
    } else {
        (locals.var_phi_s0_dep_ini, locals.var_phi_s0_dep_ini_dn0, locals.var_phi_s0_dep_ini_dn2, locals.var_phi_s0_dep_ini_dn4, locals.var_phi_s0_dep_ini_dn5, locals.var_phi_s0_dep_ini_dn6, locals.var_phi_s0_dep_ini_dn7, locals.var_phi_s0_dep_ini_dn8, locals.var_phi_s0_dep_ini_dn9, locals.var_phi_s0_dep_ini_dn10, locals.var_phi_s0_dep_ini_dn13,)
    }
};
            locals.var_phi_s0_dep_ini = assign25080_body10_e21011;
            locals.var_phi_s0_dep_ini_dn0 = assign25080_body10_e21011_d_n0;
            locals.var_phi_s0_dep_ini_dn2 = assign25080_body10_e21011_d_n2;
            locals.var_phi_s0_dep_ini_dn4 = assign25080_body10_e21011_d_n4;
            locals.var_phi_s0_dep_ini_dn5 = assign25080_body10_e21011_d_n5;
            locals.var_phi_s0_dep_ini_dn6 = assign25080_body10_e21011_d_n6;
            locals.var_phi_s0_dep_ini_dn7 = assign25080_body10_e21011_d_n7;
            locals.var_phi_s0_dep_ini_dn8 = assign25080_body10_e21011_d_n8;
            locals.var_phi_s0_dep_ini_dn9 = assign25080_body10_e21011_d_n9;
            locals.var_phi_s0_dep_ini_dn10 = assign25080_body10_e21011_d_n10;
            locals.var_phi_s0_dep_ini_dn13 = assign25080_body10_e21011_d_n13;
            locals.var_phi_s0_dep_ini_rv = 0.0;
            let (assign25080_body11_e21035, assign25080_body11_e21035_d_n0, assign25080_body11_e21035_d_n2, assign25080_body11_e21035_d_n4, assign25080_body11_e21035_d_n5, assign25080_body11_e21035_d_n6, assign25080_body11_e21035_d_n7, assign25080_body11_e21035_d_n8, assign25080_body11_e21035_d_n9, assign25080_body11_e21035_d_n10, assign25080_body11_e21035_d_n13,) = {
    if (((((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard578 == 0.0)) && (locals.var_guard580 == 0.0)) && (locals.var_guard581 != 0.0)) && (locals.var_guard583 != 0.0)) && (locals.var_guard586 == 0.0)) {
        let assign25080_body11_e21029: f64 = (-locals.var_bfact);
        let assign25080_body11_e21031: f64 = (assign25080_body11_e21029 / 2.0);
        let assign25080_body11_e21033: f64 = (assign25080_body11_e21031 / locals.var_afact);
        (assign25080_body11_e21033, (((((-locals.var_bfact_dn0) / 2.0) * locals.var_afact) - (assign25080_body11_e21031 * locals.var_afact_dn0)) / (locals.var_afact * locals.var_afact)), (((((-locals.var_bfact_dn2) / 2.0) * locals.var_afact) - (assign25080_body11_e21031 * locals.var_afact_dn2)) / (locals.var_afact * locals.var_afact)), (((((-locals.var_bfact_dn4) / 2.0) * locals.var_afact) - (assign25080_body11_e21031 * locals.var_afact_dn4)) / (locals.var_afact * locals.var_afact)), (((((-locals.var_bfact_dn5) / 2.0) * locals.var_afact) - (assign25080_body11_e21031 * locals.var_afact_dn5)) / (locals.var_afact * locals.var_afact)), (((((-locals.var_bfact_dn6) / 2.0) * locals.var_afact) - (assign25080_body11_e21031 * locals.var_afact_dn6)) / (locals.var_afact * locals.var_afact)), (((((-locals.var_bfact_dn7) / 2.0) * locals.var_afact) - (assign25080_body11_e21031 * locals.var_afact_dn7)) / (locals.var_afact * locals.var_afact)), (((((-locals.var_bfact_dn8) / 2.0) * locals.var_afact) - (assign25080_body11_e21031 * locals.var_afact_dn8)) / (locals.var_afact * locals.var_afact)), (((((-locals.var_bfact_dn9) / 2.0) * locals.var_afact) - (assign25080_body11_e21031 * locals.var_afact_dn9)) / (locals.var_afact * locals.var_afact)), (((((-locals.var_bfact_dn10) / 2.0) * locals.var_afact) - (assign25080_body11_e21031 * locals.var_afact_dn10)) / (locals.var_afact * locals.var_afact)), (((((-locals.var_bfact_dn13) / 2.0) * locals.var_afact) - (assign25080_body11_e21031 * locals.var_afact_dn13)) / (locals.var_afact * locals.var_afact)),)
    } else {
        (locals.var_phi_s0_dep_ini, locals.var_phi_s0_dep_ini_dn0, locals.var_phi_s0_dep_ini_dn2, locals.var_phi_s0_dep_ini_dn4, locals.var_phi_s0_dep_ini_dn5, locals.var_phi_s0_dep_ini_dn6, locals.var_phi_s0_dep_ini_dn7, locals.var_phi_s0_dep_ini_dn8, locals.var_phi_s0_dep_ini_dn9, locals.var_phi_s0_dep_ini_dn10, locals.var_phi_s0_dep_ini_dn13,)
    }
};
            locals.var_phi_s0_dep_ini = assign25080_body11_e21035;
            locals.var_phi_s0_dep_ini_dn0 = assign25080_body11_e21035_d_n0;
            locals.var_phi_s0_dep_ini_dn2 = assign25080_body11_e21035_d_n2;
            locals.var_phi_s0_dep_ini_dn4 = assign25080_body11_e21035_d_n4;
            locals.var_phi_s0_dep_ini_dn5 = assign25080_body11_e21035_d_n5;
            locals.var_phi_s0_dep_ini_dn6 = assign25080_body11_e21035_d_n6;
            locals.var_phi_s0_dep_ini_dn7 = assign25080_body11_e21035_d_n7;
            locals.var_phi_s0_dep_ini_dn8 = assign25080_body11_e21035_d_n8;
            locals.var_phi_s0_dep_ini_dn9 = assign25080_body11_e21035_d_n9;
            locals.var_phi_s0_dep_ini_dn10 = assign25080_body11_e21035_d_n10;
            locals.var_phi_s0_dep_ini_dn13 = assign25080_body11_e21035_d_n13;
            locals.var_phi_s0_dep_ini_rv = 0.0;
            let assign25080_body12_e21038: f64 = if locals.var_phi_s0_dep_ini > locals.var_psbmax { 1.0 } else { 0.0 };
            locals.var_guard587 = assign25080_body12_e21038;
            locals.var_guard587_rv = 0.0;
            let (assign25080_body13_e21056, assign25080_body13_e21056_d_n0, assign25080_body13_e21056_d_n2, assign25080_body13_e21056_d_n4, assign25080_body13_e21056_d_n5, assign25080_body13_e21056_d_n6, assign25080_body13_e21056_d_n7, assign25080_body13_e21056_d_n8, assign25080_body13_e21056_d_n9, assign25080_body13_e21056_d_n10, assign25080_body13_e21056_d_n13,) = {
    if (((((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard578 == 0.0)) && (locals.var_guard580 == 0.0)) && (locals.var_guard581 != 0.0)) && (locals.var_guard583 != 0.0)) && (locals.var_guard587 != 0.0)) {
        (locals.var_psbmax, locals.var_psbmax_dn0, locals.var_psbmax_dn2, locals.var_psbmax_dn4, locals.var_psbmax_dn5, locals.var_psbmax_dn6, locals.var_psbmax_dn7, locals.var_psbmax_dn8, locals.var_psbmax_dn9, locals.var_psbmax_dn10, locals.var_psbmax_dn13,)
    } else {
        (locals.var_phi_s0_dep_ini, locals.var_phi_s0_dep_ini_dn0, locals.var_phi_s0_dep_ini_dn2, locals.var_phi_s0_dep_ini_dn4, locals.var_phi_s0_dep_ini_dn5, locals.var_phi_s0_dep_ini_dn6, locals.var_phi_s0_dep_ini_dn7, locals.var_phi_s0_dep_ini_dn8, locals.var_phi_s0_dep_ini_dn9, locals.var_phi_s0_dep_ini_dn10, locals.var_phi_s0_dep_ini_dn13,)
    }
};
            locals.var_phi_s0_dep_ini = assign25080_body13_e21056;
            locals.var_phi_s0_dep_ini_dn0 = assign25080_body13_e21056_d_n0;
            locals.var_phi_s0_dep_ini_dn2 = assign25080_body13_e21056_d_n2;
            locals.var_phi_s0_dep_ini_dn4 = assign25080_body13_e21056_d_n4;
            locals.var_phi_s0_dep_ini_dn5 = assign25080_body13_e21056_d_n5;
            locals.var_phi_s0_dep_ini_dn6 = assign25080_body13_e21056_d_n6;
            locals.var_phi_s0_dep_ini_dn7 = assign25080_body13_e21056_d_n7;
            locals.var_phi_s0_dep_ini_dn8 = assign25080_body13_e21056_d_n8;
            locals.var_phi_s0_dep_ini_dn9 = assign25080_body13_e21056_d_n9;
            locals.var_phi_s0_dep_ini_dn10 = assign25080_body13_e21056_d_n10;
            locals.var_phi_s0_dep_ini_dn13 = assign25080_body13_e21056_d_n13;
            locals.var_phi_s0_dep_ini_rv = 0.0;
            let assign25080_body14_e21059: f64 = if locals.var_phi_s0_dep_ini > locals.var_phi_b0_dep { 1.0 } else { 0.0 };
            locals.var_guard588 = assign25080_body14_e21059;
            locals.var_guard588_rv = 0.0;
            let (assign25080_body15_e21079, assign25080_body15_e21079_d_n0, assign25080_body15_e21079_d_n2, assign25080_body15_e21079_d_n4, assign25080_body15_e21079_d_n5, assign25080_body15_e21079_d_n6, assign25080_body15_e21079_d_n7, assign25080_body15_e21079_d_n8, assign25080_body15_e21079_d_n9, assign25080_body15_e21079_d_n10, assign25080_body15_e21079_d_n13,) = {
    if (((((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard578 == 0.0)) && (locals.var_guard580 == 0.0)) && (locals.var_guard581 != 0.0)) && (locals.var_guard583 != 0.0)) && (locals.var_guard588 != 0.0)) {
        let assign25080_body15_e21077: f64 = (locals.var_phi_b0_dep - locals.var_ps_conv23);
        (assign25080_body15_e21077, locals.var_phi_b0_dep_dn0, locals.var_phi_b0_dep_dn2, locals.var_phi_b0_dep_dn4, locals.var_phi_b0_dep_dn5, locals.var_phi_b0_dep_dn6, locals.var_phi_b0_dep_dn7, locals.var_phi_b0_dep_dn8, locals.var_phi_b0_dep_dn9, locals.var_phi_b0_dep_dn10, locals.var_phi_b0_dep_dn13,)
    } else {
        (locals.var_phi_s0_dep_ini, locals.var_phi_s0_dep_ini_dn0, locals.var_phi_s0_dep_ini_dn2, locals.var_phi_s0_dep_ini_dn4, locals.var_phi_s0_dep_ini_dn5, locals.var_phi_s0_dep_ini_dn6, locals.var_phi_s0_dep_ini_dn7, locals.var_phi_s0_dep_ini_dn8, locals.var_phi_s0_dep_ini_dn9, locals.var_phi_s0_dep_ini_dn10, locals.var_phi_s0_dep_ini_dn13,)
    }
};
            locals.var_phi_s0_dep_ini = assign25080_body15_e21079;
            locals.var_phi_s0_dep_ini_dn0 = assign25080_body15_e21079_d_n0;
            locals.var_phi_s0_dep_ini_dn2 = assign25080_body15_e21079_d_n2;
            locals.var_phi_s0_dep_ini_dn4 = assign25080_body15_e21079_d_n4;
            locals.var_phi_s0_dep_ini_dn5 = assign25080_body15_e21079_d_n5;
            locals.var_phi_s0_dep_ini_dn6 = assign25080_body15_e21079_d_n6;
            locals.var_phi_s0_dep_ini_dn7 = assign25080_body15_e21079_d_n7;
            locals.var_phi_s0_dep_ini_dn8 = assign25080_body15_e21079_d_n8;
            locals.var_phi_s0_dep_ini_dn9 = assign25080_body15_e21079_d_n9;
            locals.var_phi_s0_dep_ini_dn10 = assign25080_body15_e21079_d_n10;
            locals.var_phi_s0_dep_ini_dn13 = assign25080_body15_e21079_d_n13;
            locals.var_phi_s0_dep_ini_rv = 0.0;
            let (assign25080_body16_e21099,) = {
    if (((((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard578 == 0.0)) && (locals.var_guard580 == 0.0)) && (locals.var_guard581 != 0.0)) && (locals.var_guard583 != 0.0)) && (locals.var_guard588 != 0.0)) {
        let assign25080_body16_e21097: f64 = (150.0 + 1.0);
        (assign25080_body16_e21097,)
    } else {
        (locals.var_lp_s0,)
    }
};
            locals.var_lp_s0 = assign25080_body16_e21099;
            locals.var_lp_s0_rv = 0.0;
            let (assign25080_body17_e21120, assign25080_body17_e21120_d_n0, assign25080_body17_e21120_d_n2, assign25080_body17_e21120_d_n4, assign25080_body17_e21120_d_n5, assign25080_body17_e21120_d_n6, assign25080_body17_e21120_d_n7, assign25080_body17_e21120_d_n8, assign25080_body17_e21120_d_n9, assign25080_body17_e21120_d_n10, assign25080_body17_e21120_d_n13,) = {
    if ((((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard578 == 0.0)) && (locals.var_guard580 == 0.0)) && (locals.var_guard581 != 0.0)) && (locals.var_guard583 != 0.0)) {
        let assign25080_body17_e21116: f64 = (locals.var_phi_b0_dep - locals.var_phi_s0_dep_ini);
        let assign25080_body17_e21117: f64 = (locals.var_c_2esipq_ndepm * assign25080_body17_e21116);
        let assign25080_body17_e21118: f64 = (assign25080_body17_e21117).sqrt();
        (assign25080_body17_e21118, (((locals.var_c_2esipq_ndepm_dn0 * assign25080_body17_e21116) + (locals.var_c_2esipq_ndepm * (locals.var_phi_b0_dep_dn0 - locals.var_phi_s0_dep_ini_dn0))) / (2.0 * assign25080_body17_e21118)), (((locals.var_c_2esipq_ndepm_dn2 * assign25080_body17_e21116) + (locals.var_c_2esipq_ndepm * (locals.var_phi_b0_dep_dn2 - locals.var_phi_s0_dep_ini_dn2))) / (2.0 * assign25080_body17_e21118)), (((locals.var_c_2esipq_ndepm_dn4 * assign25080_body17_e21116) + (locals.var_c_2esipq_ndepm * (locals.var_phi_b0_dep_dn4 - locals.var_phi_s0_dep_ini_dn4))) / (2.0 * assign25080_body17_e21118)), (((locals.var_c_2esipq_ndepm_dn5 * assign25080_body17_e21116) + (locals.var_c_2esipq_ndepm * (locals.var_phi_b0_dep_dn5 - locals.var_phi_s0_dep_ini_dn5))) / (2.0 * assign25080_body17_e21118)), (((locals.var_c_2esipq_ndepm_dn6 * assign25080_body17_e21116) + (locals.var_c_2esipq_ndepm * (locals.var_phi_b0_dep_dn6 - locals.var_phi_s0_dep_ini_dn6))) / (2.0 * assign25080_body17_e21118)), (((locals.var_c_2esipq_ndepm_dn7 * assign25080_body17_e21116) + (locals.var_c_2esipq_ndepm * (locals.var_phi_b0_dep_dn7 - locals.var_phi_s0_dep_ini_dn7))) / (2.0 * assign25080_body17_e21118)), (((locals.var_c_2esipq_ndepm_dn8 * assign25080_body17_e21116) + (locals.var_c_2esipq_ndepm * (locals.var_phi_b0_dep_dn8 - locals.var_phi_s0_dep_ini_dn8))) / (2.0 * assign25080_body17_e21118)), (((locals.var_c_2esipq_ndepm_dn9 * assign25080_body17_e21116) + (locals.var_c_2esipq_ndepm * (locals.var_phi_b0_dep_dn9 - locals.var_phi_s0_dep_ini_dn9))) / (2.0 * assign25080_body17_e21118)), (((locals.var_c_2esipq_ndepm_dn10 * assign25080_body17_e21116) + (locals.var_c_2esipq_ndepm * (locals.var_phi_b0_dep_dn10 - locals.var_phi_s0_dep_ini_dn10))) / (2.0 * assign25080_body17_e21118)), (((locals.var_c_2esipq_ndepm_dn13 * assign25080_body17_e21116) + (locals.var_c_2esipq_ndepm * (locals.var_phi_b0_dep_dn13 - locals.var_phi_s0_dep_ini_dn13))) / (2.0 * assign25080_body17_e21118)),)
    } else {
        (locals.var_w_s0, locals.var_w_s0_dn0, locals.var_w_s0_dn2, locals.var_w_s0_dn4, locals.var_w_s0_dn5, locals.var_w_s0_dn6, locals.var_w_s0_dn7, locals.var_w_s0_dn8, locals.var_w_s0_dn9, locals.var_w_s0_dn10, locals.var_w_s0_dn13,)
    }
};
            locals.var_w_s0 = assign25080_body17_e21120;
            locals.var_w_s0_dn0 = assign25080_body17_e21120_d_n0;
            locals.var_w_s0_dn2 = assign25080_body17_e21120_d_n2;
            locals.var_w_s0_dn4 = assign25080_body17_e21120_d_n4;
            locals.var_w_s0_dn5 = assign25080_body17_e21120_d_n5;
            locals.var_w_s0_dn6 = assign25080_body17_e21120_d_n6;
            locals.var_w_s0_dn7 = assign25080_body17_e21120_d_n7;
            locals.var_w_s0_dn8 = assign25080_body17_e21120_d_n8;
            locals.var_w_s0_dn9 = assign25080_body17_e21120_d_n9;
            locals.var_w_s0_dn10 = assign25080_body17_e21120_d_n10;
            locals.var_w_s0_dn13 = assign25080_body17_e21120_d_n13;
            locals.var_w_s0_rv = 0.0;
            let (assign25080_body18_e21146, assign25080_body18_e21146_d_n0, assign25080_body18_e21146_d_n2, assign25080_body18_e21146_d_n4, assign25080_body18_e21146_d_n5, assign25080_body18_e21146_d_n6, assign25080_body18_e21146_d_n7, assign25080_body18_e21146_d_n8, assign25080_body18_e21146_d_n9, assign25080_body18_e21146_d_n10, assign25080_body18_e21146_d_n13,) = {
    if ((((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard578 == 0.0)) && (locals.var_guard580 == 0.0)) && (locals.var_guard581 != 0.0)) && (locals.var_guard583 != 0.0)) {
        let assign25080_body18_e21136: f64 = (locals.var_ndepmpnsub * locals.var_phi_b0_dep);
        let assign25080_body18_e21138: f64 = (assign25080_body18_e21136 + locals.var_vbscl__blk435);
        let assign25080_body18_e21140: f64 = (assign25080_body18_e21138 - locals.var_vbi_dep);
        let assign25080_body18_e21143: f64 = (1.0 + locals.var_ndepmpnsub);
        let assign25080_body18_e21144: f64 = (assign25080_body18_e21140 / assign25080_body18_e21143);
        (assign25080_body18_e21144, (((((((locals.var_ndepmpnsub_dn0 * locals.var_phi_b0_dep) + (locals.var_ndepmpnsub * locals.var_phi_b0_dep_dn0)) + locals.var_vbscl__blk435_dn0) - locals.var_vbi_dep_dn0) * assign25080_body18_e21143) - (assign25080_body18_e21140 * locals.var_ndepmpnsub_dn0)) / (assign25080_body18_e21143 * assign25080_body18_e21143)), (((((((locals.var_ndepmpnsub_dn2 * locals.var_phi_b0_dep) + (locals.var_ndepmpnsub * locals.var_phi_b0_dep_dn2)) + locals.var_vbscl__blk435_dn2) - locals.var_vbi_dep_dn2) * assign25080_body18_e21143) - (assign25080_body18_e21140 * locals.var_ndepmpnsub_dn2)) / (assign25080_body18_e21143 * assign25080_body18_e21143)), (((((((locals.var_ndepmpnsub_dn4 * locals.var_phi_b0_dep) + (locals.var_ndepmpnsub * locals.var_phi_b0_dep_dn4)) + locals.var_vbscl__blk435_dn4) - locals.var_vbi_dep_dn4) * assign25080_body18_e21143) - (assign25080_body18_e21140 * locals.var_ndepmpnsub_dn4)) / (assign25080_body18_e21143 * assign25080_body18_e21143)), (((((((locals.var_ndepmpnsub_dn5 * locals.var_phi_b0_dep) + (locals.var_ndepmpnsub * locals.var_phi_b0_dep_dn5)) + locals.var_vbscl__blk435_dn5) - locals.var_vbi_dep_dn5) * assign25080_body18_e21143) - (assign25080_body18_e21140 * locals.var_ndepmpnsub_dn5)) / (assign25080_body18_e21143 * assign25080_body18_e21143)), (((((((locals.var_ndepmpnsub_dn6 * locals.var_phi_b0_dep) + (locals.var_ndepmpnsub * locals.var_phi_b0_dep_dn6)) + locals.var_vbscl__blk435_dn6) - locals.var_vbi_dep_dn6) * assign25080_body18_e21143) - (assign25080_body18_e21140 * locals.var_ndepmpnsub_dn6)) / (assign25080_body18_e21143 * assign25080_body18_e21143)), (((((((locals.var_ndepmpnsub_dn7 * locals.var_phi_b0_dep) + (locals.var_ndepmpnsub * locals.var_phi_b0_dep_dn7)) + locals.var_vbscl__blk435_dn7) - locals.var_vbi_dep_dn7) * assign25080_body18_e21143) - (assign25080_body18_e21140 * locals.var_ndepmpnsub_dn7)) / (assign25080_body18_e21143 * assign25080_body18_e21143)), (((((((locals.var_ndepmpnsub_dn8 * locals.var_phi_b0_dep) + (locals.var_ndepmpnsub * locals.var_phi_b0_dep_dn8)) + locals.var_vbscl__blk435_dn8) - locals.var_vbi_dep_dn8) * assign25080_body18_e21143) - (assign25080_body18_e21140 * locals.var_ndepmpnsub_dn8)) / (assign25080_body18_e21143 * assign25080_body18_e21143)), (((((((locals.var_ndepmpnsub_dn9 * locals.var_phi_b0_dep) + (locals.var_ndepmpnsub * locals.var_phi_b0_dep_dn9)) + locals.var_vbscl__blk435_dn9) - locals.var_vbi_dep_dn9) * assign25080_body18_e21143) - (assign25080_body18_e21140 * locals.var_ndepmpnsub_dn9)) / (assign25080_body18_e21143 * assign25080_body18_e21143)), (((((((locals.var_ndepmpnsub_dn10 * locals.var_phi_b0_dep) + (locals.var_ndepmpnsub * locals.var_phi_b0_dep_dn10)) + locals.var_vbscl__blk435_dn10) - locals.var_vbi_dep_dn10) * assign25080_body18_e21143) - (assign25080_body18_e21140 * locals.var_ndepmpnsub_dn10)) / (assign25080_body18_e21143 * assign25080_body18_e21143)), (((((((locals.var_ndepmpnsub_dn13 * locals.var_phi_b0_dep) + (locals.var_ndepmpnsub * locals.var_phi_b0_dep_dn13)) + locals.var_vbscl__blk435_dn13) - locals.var_vbi_dep_dn13) * assign25080_body18_e21143) - (assign25080_body18_e21140 * locals.var_ndepmpnsub_dn13)) / (assign25080_body18_e21143 * assign25080_body18_e21143)),)
    } else {
        (locals.var_phi_j0_dep, locals.var_phi_j0_dep_dn0, locals.var_phi_j0_dep_dn2, locals.var_phi_j0_dep_dn4, locals.var_phi_j0_dep_dn5, locals.var_phi_j0_dep_dn6, locals.var_phi_j0_dep_dn7, locals.var_phi_j0_dep_dn8, locals.var_phi_j0_dep_dn9, locals.var_phi_j0_dep_dn10, locals.var_phi_j0_dep_dn13,)
    }
};
            locals.var_phi_j0_dep = assign25080_body18_e21146;
            locals.var_phi_j0_dep_dn0 = assign25080_body18_e21146_d_n0;
            locals.var_phi_j0_dep_dn2 = assign25080_body18_e21146_d_n2;
            locals.var_phi_j0_dep_dn4 = assign25080_body18_e21146_d_n4;
            locals.var_phi_j0_dep_dn5 = assign25080_body18_e21146_d_n5;
            locals.var_phi_j0_dep_dn6 = assign25080_body18_e21146_d_n6;
            locals.var_phi_j0_dep_dn7 = assign25080_body18_e21146_d_n7;
            locals.var_phi_j0_dep_dn8 = assign25080_body18_e21146_d_n8;
            locals.var_phi_j0_dep_dn9 = assign25080_body18_e21146_d_n9;
            locals.var_phi_j0_dep_dn10 = assign25080_body18_e21146_d_n10;
            locals.var_phi_j0_dep_dn13 = assign25080_body18_e21146_d_n13;
            locals.var_phi_j0_dep_rv = 0.0;
            let (assign25080_body19_e21167, assign25080_body19_e21167_d_n0, assign25080_body19_e21167_d_n2, assign25080_body19_e21167_d_n4, assign25080_body19_e21167_d_n5, assign25080_body19_e21167_d_n6, assign25080_body19_e21167_d_n7, assign25080_body19_e21167_d_n8, assign25080_body19_e21167_d_n9, assign25080_body19_e21167_d_n10, assign25080_body19_e21167_d_n13,) = {
    if ((((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard578 == 0.0)) && (locals.var_guard580 == 0.0)) && (locals.var_guard581 != 0.0)) && (locals.var_guard583 != 0.0)) {
        let assign25080_body19_e21163: f64 = (locals.var_phi_b0_dep - locals.var_phi_j0_dep);
        let assign25080_body19_e21164: f64 = (locals.var_c_2esipq_ndepm * assign25080_body19_e21163);
        let assign25080_body19_e21165: f64 = (assign25080_body19_e21164).sqrt();
        (assign25080_body19_e21165, (((locals.var_c_2esipq_ndepm_dn0 * assign25080_body19_e21163) + (locals.var_c_2esipq_ndepm * (locals.var_phi_b0_dep_dn0 - locals.var_phi_j0_dep_dn0))) / (2.0 * assign25080_body19_e21165)), (((locals.var_c_2esipq_ndepm_dn2 * assign25080_body19_e21163) + (locals.var_c_2esipq_ndepm * (locals.var_phi_b0_dep_dn2 - locals.var_phi_j0_dep_dn2))) / (2.0 * assign25080_body19_e21165)), (((locals.var_c_2esipq_ndepm_dn4 * assign25080_body19_e21163) + (locals.var_c_2esipq_ndepm * (locals.var_phi_b0_dep_dn4 - locals.var_phi_j0_dep_dn4))) / (2.0 * assign25080_body19_e21165)), (((locals.var_c_2esipq_ndepm_dn5 * assign25080_body19_e21163) + (locals.var_c_2esipq_ndepm * (locals.var_phi_b0_dep_dn5 - locals.var_phi_j0_dep_dn5))) / (2.0 * assign25080_body19_e21165)), (((locals.var_c_2esipq_ndepm_dn6 * assign25080_body19_e21163) + (locals.var_c_2esipq_ndepm * (locals.var_phi_b0_dep_dn6 - locals.var_phi_j0_dep_dn6))) / (2.0 * assign25080_body19_e21165)), (((locals.var_c_2esipq_ndepm_dn7 * assign25080_body19_e21163) + (locals.var_c_2esipq_ndepm * (locals.var_phi_b0_dep_dn7 - locals.var_phi_j0_dep_dn7))) / (2.0 * assign25080_body19_e21165)), (((locals.var_c_2esipq_ndepm_dn8 * assign25080_body19_e21163) + (locals.var_c_2esipq_ndepm * (locals.var_phi_b0_dep_dn8 - locals.var_phi_j0_dep_dn8))) / (2.0 * assign25080_body19_e21165)), (((locals.var_c_2esipq_ndepm_dn9 * assign25080_body19_e21163) + (locals.var_c_2esipq_ndepm * (locals.var_phi_b0_dep_dn9 - locals.var_phi_j0_dep_dn9))) / (2.0 * assign25080_body19_e21165)), (((locals.var_c_2esipq_ndepm_dn10 * assign25080_body19_e21163) + (locals.var_c_2esipq_ndepm * (locals.var_phi_b0_dep_dn10 - locals.var_phi_j0_dep_dn10))) / (2.0 * assign25080_body19_e21165)), (((locals.var_c_2esipq_ndepm_dn13 * assign25080_body19_e21163) + (locals.var_c_2esipq_ndepm * (locals.var_phi_b0_dep_dn13 - locals.var_phi_j0_dep_dn13))) / (2.0 * assign25080_body19_e21165)),)
    } else {
        (locals.var_w_b0, locals.var_w_b0_dn0, locals.var_w_b0_dn2, locals.var_w_b0_dn4, locals.var_w_b0_dn5, locals.var_w_b0_dn6, locals.var_w_b0_dn7, locals.var_w_b0_dn8, locals.var_w_b0_dn9, locals.var_w_b0_dn10, locals.var_w_b0_dn13,)
    }
};
            locals.var_w_b0 = assign25080_body19_e21167;
            locals.var_w_b0_dn0 = assign25080_body19_e21167_d_n0;
            locals.var_w_b0_dn2 = assign25080_body19_e21167_d_n2;
            locals.var_w_b0_dn4 = assign25080_body19_e21167_d_n4;
            locals.var_w_b0_dn5 = assign25080_body19_e21167_d_n5;
            locals.var_w_b0_dn6 = assign25080_body19_e21167_d_n6;
            locals.var_w_b0_dn7 = assign25080_body19_e21167_d_n7;
            locals.var_w_b0_dn8 = assign25080_body19_e21167_d_n8;
            locals.var_w_b0_dn9 = assign25080_body19_e21167_d_n9;
            locals.var_w_b0_dn10 = assign25080_body19_e21167_d_n10;
            locals.var_w_b0_dn13 = assign25080_body19_e21167_d_n13;
            locals.var_w_b0_rv = 0.0;
            let assign25080_body20_e21170: f64 = (locals.var_phi_b0_dep - locals.var_phi_b0_dep_old);
            let assign25080_body20_e21171: f64 = (assign25080_body20_e21170).abs();
            let assign25080_body20_e21173: f64 = if assign25080_body20_e21171 <= 1e-8 { 1.0 } else { 0.0 };
            locals.var_guard589 = assign25080_body20_e21173;
            locals.var_guard589_rv = 0.0;
            let (assign25080_body21_e21193,) = {
    if (((((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard578 == 0.0)) && (locals.var_guard580 == 0.0)) && (locals.var_guard581 != 0.0)) && (locals.var_guard583 != 0.0)) && (locals.var_guard589 != 0.0)) {
        let assign25080_body21_e21191: f64 = (150.0 + 1.0);
        (assign25080_body21_e21191,)
    } else {
        (locals.var_lp_s0,)
    }
};
            locals.var_lp_s0 = assign25080_body21_e21193;
            locals.var_lp_s0_rv = 0.0;
            let (assign25080_body22_e21209, assign25080_body22_e21209_d_n0, assign25080_body22_e21209_d_n2, assign25080_body22_e21209_d_n4, assign25080_body22_e21209_d_n5, assign25080_body22_e21209_d_n6, assign25080_body22_e21209_d_n7, assign25080_body22_e21209_d_n8, assign25080_body22_e21209_d_n9, assign25080_body22_e21209_d_n10, assign25080_body22_e21209_d_n13,) = {
    if ((((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard578 == 0.0)) && (locals.var_guard580 == 0.0)) && (locals.var_guard581 != 0.0)) && (locals.var_guard583 != 0.0)) {
        (locals.var_phi_b0_dep, locals.var_phi_b0_dep_dn0, locals.var_phi_b0_dep_dn2, locals.var_phi_b0_dep_dn4, locals.var_phi_b0_dep_dn5, locals.var_phi_b0_dep_dn6, locals.var_phi_b0_dep_dn7, locals.var_phi_b0_dep_dn8, locals.var_phi_b0_dep_dn9, locals.var_phi_b0_dep_dn10, locals.var_phi_b0_dep_dn13,)
    } else {
        (locals.var_phi_b0_dep_old, locals.var_phi_b0_dep_old_dn0, locals.var_phi_b0_dep_old_dn2, locals.var_phi_b0_dep_old_dn4, locals.var_phi_b0_dep_old_dn5, locals.var_phi_b0_dep_old_dn6, locals.var_phi_b0_dep_old_dn7, locals.var_phi_b0_dep_old_dn8, locals.var_phi_b0_dep_old_dn9, locals.var_phi_b0_dep_old_dn10, locals.var_phi_b0_dep_old_dn13,)
    }
};
            locals.var_phi_b0_dep_old = assign25080_body22_e21209;
            locals.var_phi_b0_dep_old_dn0 = assign25080_body22_e21209_d_n0;
            locals.var_phi_b0_dep_old_dn2 = assign25080_body22_e21209_d_n2;
            locals.var_phi_b0_dep_old_dn4 = assign25080_body22_e21209_d_n4;
            locals.var_phi_b0_dep_old_dn5 = assign25080_body22_e21209_d_n5;
            locals.var_phi_b0_dep_old_dn6 = assign25080_body22_e21209_d_n6;
            locals.var_phi_b0_dep_old_dn7 = assign25080_body22_e21209_d_n7;
            locals.var_phi_b0_dep_old_dn8 = assign25080_body22_e21209_d_n8;
            locals.var_phi_b0_dep_old_dn9 = assign25080_body22_e21209_d_n9;
            locals.var_phi_b0_dep_old_dn10 = assign25080_body22_e21209_d_n10;
            locals.var_phi_b0_dep_old_dn13 = assign25080_body22_e21209_d_n13;
            locals.var_phi_b0_dep_old_rv = 0.0;
            let (assign25080_body23_e21227,) = {
    if ((((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard578 == 0.0)) && (locals.var_guard580 == 0.0)) && (locals.var_guard581 != 0.0)) && (locals.var_guard583 != 0.0)) {
        let assign25080_body23_e21225: f64 = (locals.var_lp_s0 + 1.0);
        (assign25080_body23_e21225,)
    } else {
        (locals.var_lp_s0,)
    }
};
            locals.var_lp_s0 = assign25080_body23_e21227;
            locals.var_lp_s0_rv = 0.0;
        }

        let (assign25090_e21247, assign25090_e21247_d_n0, assign25090_e21247_d_n2, assign25090_e21247_d_n4, assign25090_e21247_d_n5, assign25090_e21247_d_n6, assign25090_e21247_d_n7, assign25090_e21247_d_n8, assign25090_e21247_d_n9, assign25090_e21247_d_n10, assign25090_e21247_d_n13,) = {
    if (((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard578 == 0.0)) && (locals.var_guard580 == 0.0)) && (locals.var_guard581 == 0.0)) {
        let assign25090_e21243: f64 = (locals.var_beta * locals.var_vbscl__blk435);
        let assign25090_e21244: f64 = (assign25090_e21243).exp();
        let assign25090_e21245: f64 = (locals.var_afact2 / assign25090_e21244);
        (assign25090_e21245, (((locals.var_afact2_dn0 * assign25090_e21244) - (locals.var_afact2 * (assign25090_e21244 * ((locals.var_beta_dn0 * locals.var_vbscl__blk435) + (locals.var_beta * locals.var_vbscl__blk435_dn0))))) / (assign25090_e21244 * assign25090_e21244)), (((locals.var_afact2_dn2 * assign25090_e21244) - (locals.var_afact2 * (assign25090_e21244 * ((locals.var_beta_dn2 * locals.var_vbscl__blk435) + (locals.var_beta * locals.var_vbscl__blk435_dn2))))) / (assign25090_e21244 * assign25090_e21244)), (((locals.var_afact2_dn4 * assign25090_e21244) - (locals.var_afact2 * (assign25090_e21244 * ((locals.var_beta_dn4 * locals.var_vbscl__blk435) + (locals.var_beta * locals.var_vbscl__blk435_dn4))))) / (assign25090_e21244 * assign25090_e21244)), (((locals.var_afact2_dn5 * assign25090_e21244) - (locals.var_afact2 * (assign25090_e21244 * ((locals.var_beta_dn5 * locals.var_vbscl__blk435) + (locals.var_beta * locals.var_vbscl__blk435_dn5))))) / (assign25090_e21244 * assign25090_e21244)), (((locals.var_afact2_dn6 * assign25090_e21244) - (locals.var_afact2 * (assign25090_e21244 * ((locals.var_beta_dn6 * locals.var_vbscl__blk435) + (locals.var_beta * locals.var_vbscl__blk435_dn6))))) / (assign25090_e21244 * assign25090_e21244)), (((locals.var_afact2_dn7 * assign25090_e21244) - (locals.var_afact2 * (assign25090_e21244 * ((locals.var_beta_dn7 * locals.var_vbscl__blk435) + (locals.var_beta * locals.var_vbscl__blk435_dn7))))) / (assign25090_e21244 * assign25090_e21244)), (((locals.var_afact2_dn8 * assign25090_e21244) - (locals.var_afact2 * (assign25090_e21244 * ((locals.var_beta_dn8 * locals.var_vbscl__blk435) + (locals.var_beta * locals.var_vbscl__blk435_dn8))))) / (assign25090_e21244 * assign25090_e21244)), (((locals.var_afact2_dn9 * assign25090_e21244) - (locals.var_afact2 * (assign25090_e21244 * ((locals.var_beta_dn9 * locals.var_vbscl__blk435) + (locals.var_beta * locals.var_vbscl__blk435_dn9))))) / (assign25090_e21244 * assign25090_e21244)), (((locals.var_afact2_dn10 * assign25090_e21244) - (locals.var_afact2 * (assign25090_e21244 * ((locals.var_beta_dn10 * locals.var_vbscl__blk435) + (locals.var_beta * locals.var_vbscl__blk435_dn10))))) / (assign25090_e21244 * assign25090_e21244)), (((locals.var_afact2_dn13 * assign25090_e21244) - (locals.var_afact2 * (assign25090_e21244 * ((locals.var_beta_dn13 * locals.var_vbscl__blk435) + (locals.var_beta * locals.var_vbscl__blk435_dn13))))) / (assign25090_e21244 * assign25090_e21244)),)
    } else {
        (locals.var_afact3, locals.var_afact3_dn0, locals.var_afact3_dn2, locals.var_afact3_dn4, locals.var_afact3_dn5, locals.var_afact3_dn6, locals.var_afact3_dn7, locals.var_afact3_dn8, locals.var_afact3_dn9, locals.var_afact3_dn10, locals.var_afact3_dn13,)
    }
};
        locals.var_afact3 = assign25090_e21247;
        locals.var_afact3_dn0 = assign25090_e21247_d_n0;
        locals.var_afact3_dn2 = assign25090_e21247_d_n2;
        locals.var_afact3_dn4 = assign25090_e21247_d_n4;
        locals.var_afact3_dn5 = assign25090_e21247_d_n5;
        locals.var_afact3_dn6 = assign25090_e21247_d_n6;
        locals.var_afact3_dn7 = assign25090_e21247_d_n7;
        locals.var_afact3_dn8 = assign25090_e21247_d_n8;
        locals.var_afact3_dn9 = assign25090_e21247_d_n9;
        locals.var_afact3_dn10 = assign25090_e21247_d_n10;
        locals.var_afact3_dn13 = assign25090_e21247_d_n13;
        locals.var_afact3_rv = 0.0;

        let (assign25100_e21262, assign25100_e21262_d_n0, assign25100_e21262_d_n2, assign25100_e21262_d_n4, assign25100_e21262_d_n5, assign25100_e21262_d_n6, assign25100_e21262_d_n7, assign25100_e21262_d_n8, assign25100_e21262_d_n9, assign25100_e21262_d_n10, assign25100_e21262_d_n13,) = {
    if (((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard578 == 0.0)) && (locals.var_guard580 == 0.0)) && (locals.var_guard581 == 0.0)) {
        (locals.var_phi_b0_dep, locals.var_phi_b0_dep_dn0, locals.var_phi_b0_dep_dn2, locals.var_phi_b0_dep_dn4, locals.var_phi_b0_dep_dn5, locals.var_phi_b0_dep_dn6, locals.var_phi_b0_dep_dn7, locals.var_phi_b0_dep_dn8, locals.var_phi_b0_dep_dn9, locals.var_phi_b0_dep_dn10, locals.var_phi_b0_dep_dn13,)
    } else {
        (locals.var_phi_b0_dep_old, locals.var_phi_b0_dep_old_dn0, locals.var_phi_b0_dep_old_dn2, locals.var_phi_b0_dep_old_dn4, locals.var_phi_b0_dep_old_dn5, locals.var_phi_b0_dep_old_dn6, locals.var_phi_b0_dep_old_dn7, locals.var_phi_b0_dep_old_dn8, locals.var_phi_b0_dep_old_dn9, locals.var_phi_b0_dep_old_dn10, locals.var_phi_b0_dep_old_dn13,)
    }
};
        locals.var_phi_b0_dep_old = assign25100_e21262;
        locals.var_phi_b0_dep_old_dn0 = assign25100_e21262_d_n0;
        locals.var_phi_b0_dep_old_dn2 = assign25100_e21262_d_n2;
        locals.var_phi_b0_dep_old_dn4 = assign25100_e21262_d_n4;
        locals.var_phi_b0_dep_old_dn5 = assign25100_e21262_d_n5;
        locals.var_phi_b0_dep_old_dn6 = assign25100_e21262_d_n6;
        locals.var_phi_b0_dep_old_dn7 = assign25100_e21262_d_n7;
        locals.var_phi_b0_dep_old_dn8 = assign25100_e21262_d_n8;
        locals.var_phi_b0_dep_old_dn9 = assign25100_e21262_d_n9;
        locals.var_phi_b0_dep_old_dn10 = assign25100_e21262_d_n10;
        locals.var_phi_b0_dep_old_dn13 = assign25100_e21262_d_n13;
        locals.var_phi_b0_dep_old_rv = 0.0;

        let (assign25110_e21289, assign25110_e21289_d_n0, assign25110_e21289_d_n2, assign25110_e21289_d_n4, assign25110_e21289_d_n5, assign25110_e21289_d_n6, assign25110_e21289_d_n7, assign25110_e21289_d_n8, assign25110_e21289_d_n9, assign25110_e21289_d_n10, assign25110_e21289_d_n13,) = {
    if (((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard578 == 0.0)) && (locals.var_guard580 == 0.0)) && (locals.var_guard581 == 0.0)) {
        let assign25110_e21277: f64 = (locals.var_afact3 * locals.var_vgp);
        let assign25110_e21279: f64 = (assign25110_e21277 * locals.var_vgp);
        let assign25110_e21280: f64 = (assign25110_e21279).ln();
        let assign25110_e21282: f64 = (-locals.var_beta);
        let assign25110_e21285: f64 = (2.0 / locals.var_vgp);
        let assign25110_e21286: f64 = (assign25110_e21282 + assign25110_e21285);
        let assign25110_e21287: f64 = (assign25110_e21280 / assign25110_e21286);
        (assign25110_e21287, ((((((((locals.var_afact3_dn0 * locals.var_vgp) + (locals.var_afact3 * locals.var_vgp_dn0)) * locals.var_vgp) + (assign25110_e21277 * locals.var_vgp_dn0)) / assign25110_e21279) * assign25110_e21286) - (assign25110_e21280 * ((-locals.var_beta_dn0) + (-((2.0 * locals.var_vgp_dn0) / (locals.var_vgp * locals.var_vgp)))))) / (assign25110_e21286 * assign25110_e21286)), ((((((((locals.var_afact3_dn2 * locals.var_vgp) + (locals.var_afact3 * locals.var_vgp_dn2)) * locals.var_vgp) + (assign25110_e21277 * locals.var_vgp_dn2)) / assign25110_e21279) * assign25110_e21286) - (assign25110_e21280 * ((-locals.var_beta_dn2) + (-((2.0 * locals.var_vgp_dn2) / (locals.var_vgp * locals.var_vgp)))))) / (assign25110_e21286 * assign25110_e21286)), ((((((((locals.var_afact3_dn4 * locals.var_vgp) + (locals.var_afact3 * locals.var_vgp_dn4)) * locals.var_vgp) + (assign25110_e21277 * locals.var_vgp_dn4)) / assign25110_e21279) * assign25110_e21286) - (assign25110_e21280 * ((-locals.var_beta_dn4) + (-((2.0 * locals.var_vgp_dn4) / (locals.var_vgp * locals.var_vgp)))))) / (assign25110_e21286 * assign25110_e21286)), ((((((((locals.var_afact3_dn5 * locals.var_vgp) + (locals.var_afact3 * locals.var_vgp_dn5)) * locals.var_vgp) + (assign25110_e21277 * locals.var_vgp_dn5)) / assign25110_e21279) * assign25110_e21286) - (assign25110_e21280 * ((-locals.var_beta_dn5) + (-((2.0 * locals.var_vgp_dn5) / (locals.var_vgp * locals.var_vgp)))))) / (assign25110_e21286 * assign25110_e21286)), ((((((((locals.var_afact3_dn6 * locals.var_vgp) + (locals.var_afact3 * locals.var_vgp_dn6)) * locals.var_vgp) + (assign25110_e21277 * locals.var_vgp_dn6)) / assign25110_e21279) * assign25110_e21286) - (assign25110_e21280 * ((-locals.var_beta_dn6) + (-((2.0 * locals.var_vgp_dn6) / (locals.var_vgp * locals.var_vgp)))))) / (assign25110_e21286 * assign25110_e21286)), ((((((((locals.var_afact3_dn7 * locals.var_vgp) + (locals.var_afact3 * locals.var_vgp_dn7)) * locals.var_vgp) + (assign25110_e21277 * locals.var_vgp_dn7)) / assign25110_e21279) * assign25110_e21286) - (assign25110_e21280 * ((-locals.var_beta_dn7) + (-((2.0 * locals.var_vgp_dn7) / (locals.var_vgp * locals.var_vgp)))))) / (assign25110_e21286 * assign25110_e21286)), ((((((((locals.var_afact3_dn8 * locals.var_vgp) + (locals.var_afact3 * locals.var_vgp_dn8)) * locals.var_vgp) + (assign25110_e21277 * locals.var_vgp_dn8)) / assign25110_e21279) * assign25110_e21286) - (assign25110_e21280 * ((-locals.var_beta_dn8) + (-((2.0 * locals.var_vgp_dn8) / (locals.var_vgp * locals.var_vgp)))))) / (assign25110_e21286 * assign25110_e21286)), ((((((((locals.var_afact3_dn9 * locals.var_vgp) + (locals.var_afact3 * locals.var_vgp_dn9)) * locals.var_vgp) + (assign25110_e21277 * locals.var_vgp_dn9)) / assign25110_e21279) * assign25110_e21286) - (assign25110_e21280 * ((-locals.var_beta_dn9) + (-((2.0 * locals.var_vgp_dn9) / (locals.var_vgp * locals.var_vgp)))))) / (assign25110_e21286 * assign25110_e21286)), ((((((((locals.var_afact3_dn10 * locals.var_vgp) + (locals.var_afact3 * locals.var_vgp_dn10)) * locals.var_vgp) + (assign25110_e21277 * locals.var_vgp_dn10)) / assign25110_e21279) * assign25110_e21286) - (assign25110_e21280 * ((-locals.var_beta_dn10) + (-((2.0 * locals.var_vgp_dn10) / (locals.var_vgp * locals.var_vgp)))))) / (assign25110_e21286 * assign25110_e21286)), ((((((((locals.var_afact3_dn13 * locals.var_vgp) + (locals.var_afact3 * locals.var_vgp_dn13)) * locals.var_vgp) + (assign25110_e21277 * locals.var_vgp_dn13)) / assign25110_e21279) * assign25110_e21286) - (assign25110_e21280 * ((-locals.var_beta_dn13) + (-((2.0 * locals.var_vgp_dn13) / (locals.var_vgp * locals.var_vgp)))))) / (assign25110_e21286 * assign25110_e21286)),)
    } else {
        (locals.var_phi_s0_dep_ini, locals.var_phi_s0_dep_ini_dn0, locals.var_phi_s0_dep_ini_dn2, locals.var_phi_s0_dep_ini_dn4, locals.var_phi_s0_dep_ini_dn5, locals.var_phi_s0_dep_ini_dn6, locals.var_phi_s0_dep_ini_dn7, locals.var_phi_s0_dep_ini_dn8, locals.var_phi_s0_dep_ini_dn9, locals.var_phi_s0_dep_ini_dn10, locals.var_phi_s0_dep_ini_dn13,)
    }
};
        locals.var_phi_s0_dep_ini = assign25110_e21289;
        locals.var_phi_s0_dep_ini_dn0 = assign25110_e21289_d_n0;
        locals.var_phi_s0_dep_ini_dn2 = assign25110_e21289_d_n2;
        locals.var_phi_s0_dep_ini_dn4 = assign25110_e21289_d_n4;
        locals.var_phi_s0_dep_ini_dn5 = assign25110_e21289_d_n5;
        locals.var_phi_s0_dep_ini_dn6 = assign25110_e21289_d_n6;
        locals.var_phi_s0_dep_ini_dn7 = assign25110_e21289_d_n7;
        locals.var_phi_s0_dep_ini_dn8 = assign25110_e21289_d_n8;
        locals.var_phi_s0_dep_ini_dn9 = assign25110_e21289_d_n9;
        locals.var_phi_s0_dep_ini_dn10 = assign25110_e21289_d_n10;
        locals.var_phi_s0_dep_ini_dn13 = assign25110_e21289_d_n13;
        locals.var_phi_s0_dep_ini_rv = 0.0;

        let (assign25120_e21309, assign25120_e21309_d_n0, assign25120_e21309_d_n2, assign25120_e21309_d_n4, assign25120_e21309_d_n5, assign25120_e21309_d_n6, assign25120_e21309_d_n7, assign25120_e21309_d_n8, assign25120_e21309_d_n9, assign25120_e21309_d_n10, assign25120_e21309_d_n13,) = {
    if (((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard578 == 0.0)) && (locals.var_guard580 == 0.0)) && (locals.var_guard581 == 0.0)) {
        let assign25120_e21305: f64 = (locals.var_phi_b0_dep - locals.var_phi_s0_dep_ini);
        let assign25120_e21306: f64 = (locals.var_c_2esipq_ndepm * assign25120_e21305);
        let assign25120_e21307: f64 = (assign25120_e21306).sqrt();
        (assign25120_e21307, (((locals.var_c_2esipq_ndepm_dn0 * assign25120_e21305) + (locals.var_c_2esipq_ndepm * (locals.var_phi_b0_dep_dn0 - locals.var_phi_s0_dep_ini_dn0))) / (2.0 * assign25120_e21307)), (((locals.var_c_2esipq_ndepm_dn2 * assign25120_e21305) + (locals.var_c_2esipq_ndepm * (locals.var_phi_b0_dep_dn2 - locals.var_phi_s0_dep_ini_dn2))) / (2.0 * assign25120_e21307)), (((locals.var_c_2esipq_ndepm_dn4 * assign25120_e21305) + (locals.var_c_2esipq_ndepm * (locals.var_phi_b0_dep_dn4 - locals.var_phi_s0_dep_ini_dn4))) / (2.0 * assign25120_e21307)), (((locals.var_c_2esipq_ndepm_dn5 * assign25120_e21305) + (locals.var_c_2esipq_ndepm * (locals.var_phi_b0_dep_dn5 - locals.var_phi_s0_dep_ini_dn5))) / (2.0 * assign25120_e21307)), (((locals.var_c_2esipq_ndepm_dn6 * assign25120_e21305) + (locals.var_c_2esipq_ndepm * (locals.var_phi_b0_dep_dn6 - locals.var_phi_s0_dep_ini_dn6))) / (2.0 * assign25120_e21307)), (((locals.var_c_2esipq_ndepm_dn7 * assign25120_e21305) + (locals.var_c_2esipq_ndepm * (locals.var_phi_b0_dep_dn7 - locals.var_phi_s0_dep_ini_dn7))) / (2.0 * assign25120_e21307)), (((locals.var_c_2esipq_ndepm_dn8 * assign25120_e21305) + (locals.var_c_2esipq_ndepm * (locals.var_phi_b0_dep_dn8 - locals.var_phi_s0_dep_ini_dn8))) / (2.0 * assign25120_e21307)), (((locals.var_c_2esipq_ndepm_dn9 * assign25120_e21305) + (locals.var_c_2esipq_ndepm * (locals.var_phi_b0_dep_dn9 - locals.var_phi_s0_dep_ini_dn9))) / (2.0 * assign25120_e21307)), (((locals.var_c_2esipq_ndepm_dn10 * assign25120_e21305) + (locals.var_c_2esipq_ndepm * (locals.var_phi_b0_dep_dn10 - locals.var_phi_s0_dep_ini_dn10))) / (2.0 * assign25120_e21307)), (((locals.var_c_2esipq_ndepm_dn13 * assign25120_e21305) + (locals.var_c_2esipq_ndepm * (locals.var_phi_b0_dep_dn13 - locals.var_phi_s0_dep_ini_dn13))) / (2.0 * assign25120_e21307)),)
    } else {
        (locals.var_w_s0, locals.var_w_s0_dn0, locals.var_w_s0_dn2, locals.var_w_s0_dn4, locals.var_w_s0_dn5, locals.var_w_s0_dn6, locals.var_w_s0_dn7, locals.var_w_s0_dn8, locals.var_w_s0_dn9, locals.var_w_s0_dn10, locals.var_w_s0_dn13,)
    }
};
        locals.var_w_s0 = assign25120_e21309;
        locals.var_w_s0_dn0 = assign25120_e21309_d_n0;
        locals.var_w_s0_dn2 = assign25120_e21309_d_n2;
        locals.var_w_s0_dn4 = assign25120_e21309_d_n4;
        locals.var_w_s0_dn5 = assign25120_e21309_d_n5;
        locals.var_w_s0_dn6 = assign25120_e21309_d_n6;
        locals.var_w_s0_dn7 = assign25120_e21309_d_n7;
        locals.var_w_s0_dn8 = assign25120_e21309_d_n8;
        locals.var_w_s0_dn9 = assign25120_e21309_d_n9;
        locals.var_w_s0_dn10 = assign25120_e21309_d_n10;
        locals.var_w_s0_dn13 = assign25120_e21309_d_n13;
        locals.var_w_s0_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_70(
        locals: &mut StampLocals,
    ) {
        let (assign25130_e21329, assign25130_e21329_d_n0, assign25130_e21329_d_n2, assign25130_e21329_d_n4, assign25130_e21329_d_n5, assign25130_e21329_d_n6, assign25130_e21329_d_n7, assign25130_e21329_d_n8, assign25130_e21329_d_n9, assign25130_e21329_d_n10, assign25130_e21329_d_n13,) = {
    if (((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard578 == 0.0)) && (locals.var_guard580 == 0.0)) && (locals.var_guard581 == 0.0)) {
        let assign25130_e21325: f64 = (locals.var_phi_b0_dep - locals.var_phi_j0_dep);
        let assign25130_e21326: f64 = (locals.var_c_2esipq_ndepm * assign25130_e21325);
        let assign25130_e21327: f64 = (assign25130_e21326).sqrt();
        (assign25130_e21327, (((locals.var_c_2esipq_ndepm_dn0 * assign25130_e21325) + (locals.var_c_2esipq_ndepm * (locals.var_phi_b0_dep_dn0 - locals.var_phi_j0_dep_dn0))) / (2.0 * assign25130_e21327)), (((locals.var_c_2esipq_ndepm_dn2 * assign25130_e21325) + (locals.var_c_2esipq_ndepm * (locals.var_phi_b0_dep_dn2 - locals.var_phi_j0_dep_dn2))) / (2.0 * assign25130_e21327)), (((locals.var_c_2esipq_ndepm_dn4 * assign25130_e21325) + (locals.var_c_2esipq_ndepm * (locals.var_phi_b0_dep_dn4 - locals.var_phi_j0_dep_dn4))) / (2.0 * assign25130_e21327)), (((locals.var_c_2esipq_ndepm_dn5 * assign25130_e21325) + (locals.var_c_2esipq_ndepm * (locals.var_phi_b0_dep_dn5 - locals.var_phi_j0_dep_dn5))) / (2.0 * assign25130_e21327)), (((locals.var_c_2esipq_ndepm_dn6 * assign25130_e21325) + (locals.var_c_2esipq_ndepm * (locals.var_phi_b0_dep_dn6 - locals.var_phi_j0_dep_dn6))) / (2.0 * assign25130_e21327)), (((locals.var_c_2esipq_ndepm_dn7 * assign25130_e21325) + (locals.var_c_2esipq_ndepm * (locals.var_phi_b0_dep_dn7 - locals.var_phi_j0_dep_dn7))) / (2.0 * assign25130_e21327)), (((locals.var_c_2esipq_ndepm_dn8 * assign25130_e21325) + (locals.var_c_2esipq_ndepm * (locals.var_phi_b0_dep_dn8 - locals.var_phi_j0_dep_dn8))) / (2.0 * assign25130_e21327)), (((locals.var_c_2esipq_ndepm_dn9 * assign25130_e21325) + (locals.var_c_2esipq_ndepm * (locals.var_phi_b0_dep_dn9 - locals.var_phi_j0_dep_dn9))) / (2.0 * assign25130_e21327)), (((locals.var_c_2esipq_ndepm_dn10 * assign25130_e21325) + (locals.var_c_2esipq_ndepm * (locals.var_phi_b0_dep_dn10 - locals.var_phi_j0_dep_dn10))) / (2.0 * assign25130_e21327)), (((locals.var_c_2esipq_ndepm_dn13 * assign25130_e21325) + (locals.var_c_2esipq_ndepm * (locals.var_phi_b0_dep_dn13 - locals.var_phi_j0_dep_dn13))) / (2.0 * assign25130_e21327)),)
    } else {
        (locals.var_w_b0, locals.var_w_b0_dn0, locals.var_w_b0_dn2, locals.var_w_b0_dn4, locals.var_w_b0_dn5, locals.var_w_b0_dn6, locals.var_w_b0_dn7, locals.var_w_b0_dn8, locals.var_w_b0_dn9, locals.var_w_b0_dn10, locals.var_w_b0_dn13,)
    }
};
        locals.var_w_b0 = assign25130_e21329;
        locals.var_w_b0_dn0 = assign25130_e21329_d_n0;
        locals.var_w_b0_dn2 = assign25130_e21329_d_n2;
        locals.var_w_b0_dn4 = assign25130_e21329_d_n4;
        locals.var_w_b0_dn5 = assign25130_e21329_d_n5;
        locals.var_w_b0_dn6 = assign25130_e21329_d_n6;
        locals.var_w_b0_dn7 = assign25130_e21329_d_n7;
        locals.var_w_b0_dn8 = assign25130_e21329_d_n8;
        locals.var_w_b0_dn9 = assign25130_e21329_d_n9;
        locals.var_w_b0_dn10 = assign25130_e21329_d_n10;
        locals.var_w_b0_dn13 = assign25130_e21329_d_n13;
        locals.var_w_b0_rv = 0.0;

        let assign25140_e21332: f64 = (locals.var_w_s0 + locals.var_w_b0);
        let assign25140_e21334: f64 = if assign25140_e21332 > locals.var_uc_depthn { 1.0 } else { 0.0 };
        locals.var_guard590 = assign25140_e21334;
        locals.var_guard590_rv = 0.0;

        let (assign25150_e21351,) = {
    if ((((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard578 == 0.0)) && (locals.var_guard580 == 0.0)) && (locals.var_guard581 == 0.0)) && (locals.var_guard590 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_lp_s0,)
    }
};
        locals.var_lp_s0 = assign25150_e21351;
        locals.var_lp_s0_rv = 0.0;

        let mut assign25160_loop_guard: usize = 0;
        while {
            let assign25160_cond_e21369: f64 = (locals.var_lp_s0_max + 1.0);
            let assign25160_cond_e21371: f64 = if (((((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard578 == 0.0)) && (locals.var_guard580 == 0.0)) && (locals.var_guard581 == 0.0)) && (locals.var_guard590 != 0.0)) && (locals.var_lp_s0 <= assign25160_cond_e21369)) { 1.0 } else { 0.0 };
            assign25160_cond_e21371 != 0.0
        } {
            assign25160_loop_guard += 1;
            assert!(assign25160_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign25160_body0_e21392, assign25160_body0_e21392_d_n0, assign25160_body0_e21392_d_n2, assign25160_body0_e21392_d_n4, assign25160_body0_e21392_d_n5, assign25160_body0_e21392_d_n6, assign25160_body0_e21392_d_n7, assign25160_body0_e21392_d_n8, assign25160_body0_e21392_d_n9, assign25160_body0_e21392_d_n10, assign25160_body0_e21392_d_n13,) = {
    if ((((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard578 == 0.0)) && (locals.var_guard580 == 0.0)) && (locals.var_guard581 == 0.0)) && (locals.var_guard590 != 0.0)) {
        let assign25160_body0_e21388: f64 = (locals.var_w_s0 + locals.var_w_b0);
        let assign25160_body0_e21390: f64 = (assign25160_body0_e21388 - locals.var_uc_depthn);
        (assign25160_body0_e21390, ((locals.var_w_s0_dn0 + locals.var_w_b0_dn0) - locals.var_uc_depthn_dn0), ((locals.var_w_s0_dn2 + locals.var_w_b0_dn2) - locals.var_uc_depthn_dn2), ((locals.var_w_s0_dn4 + locals.var_w_b0_dn4) - locals.var_uc_depthn_dn4), ((locals.var_w_s0_dn5 + locals.var_w_b0_dn5) - locals.var_uc_depthn_dn5), ((locals.var_w_s0_dn6 + locals.var_w_b0_dn6) - locals.var_uc_depthn_dn6), ((locals.var_w_s0_dn7 + locals.var_w_b0_dn7) - locals.var_uc_depthn_dn7), ((locals.var_w_s0_dn8 + locals.var_w_b0_dn8) - locals.var_uc_depthn_dn8), ((locals.var_w_s0_dn9 + locals.var_w_b0_dn9) - locals.var_uc_depthn_dn9), ((locals.var_w_s0_dn10 + locals.var_w_b0_dn10) - locals.var_uc_depthn_dn10), ((locals.var_w_s0_dn13 + locals.var_w_b0_dn13) - locals.var_uc_depthn_dn13),)
    } else {
        (locals.var_y0, locals.var_y0_dn0, locals.var_y0_dn2, locals.var_y0_dn4, locals.var_y0_dn5, locals.var_y0_dn6, locals.var_y0_dn7, locals.var_y0_dn8, locals.var_y0_dn9, locals.var_y0_dn10, locals.var_y0_dn13,)
    }
};
            locals.var_y0 = assign25160_body0_e21392;
            locals.var_y0_dn0 = assign25160_body0_e21392_d_n0;
            locals.var_y0_dn2 = assign25160_body0_e21392_d_n2;
            locals.var_y0_dn4 = assign25160_body0_e21392_d_n4;
            locals.var_y0_dn5 = assign25160_body0_e21392_d_n5;
            locals.var_y0_dn6 = assign25160_body0_e21392_d_n6;
            locals.var_y0_dn7 = assign25160_body0_e21392_d_n7;
            locals.var_y0_dn8 = assign25160_body0_e21392_d_n8;
            locals.var_y0_dn9 = assign25160_body0_e21392_d_n9;
            locals.var_y0_dn10 = assign25160_body0_e21392_d_n10;
            locals.var_y0_dn13 = assign25160_body0_e21392_d_n13;
            locals.var_y0_rv = 0.0;
            let (assign25160_body1_e21427, assign25160_body1_e21427_d_n0, assign25160_body1_e21427_d_n2, assign25160_body1_e21427_d_n4, assign25160_body1_e21427_d_n5, assign25160_body1_e21427_d_n6, assign25160_body1_e21427_d_n7, assign25160_body1_e21427_d_n8, assign25160_body1_e21427_d_n9, assign25160_body1_e21427_d_n10, assign25160_body1_e21427_d_n13,) = {
    if ((((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard578 == 0.0)) && (locals.var_guard580 == 0.0)) && (locals.var_guard581 == 0.0)) && (locals.var_guard590 != 0.0)) {
        let assign25160_body1_e21409: f64 = (1.034943e-10 / locals.var_q_ndepm);
        let assign25160_body1_e21411: f64 = (assign25160_body1_e21409 / locals.var_w_s0);
        let assign25160_body1_e21414: f64 = (1.034943e-10 / locals.var_q_ndepm);
        let assign25160_body1_e21419: f64 = (1.0 + locals.var_ndepmpnsub);
        let assign25160_body1_e21420: f64 = (locals.var_ndepmpnsub / assign25160_body1_e21419);
        let assign25160_body1_e21421: f64 = (1.0 - assign25160_body1_e21420);
        let assign25160_body1_e21422: f64 = (assign25160_body1_e21414 * assign25160_body1_e21421);
        let assign25160_body1_e21424: f64 = (assign25160_body1_e21422 / locals.var_w_b0);
        let assign25160_body1_e21425: f64 = (assign25160_body1_e21411 + assign25160_body1_e21424);
        (assign25160_body1_e21425, (((((-((1.034943e-10 * locals.var_q_ndepm_dn0) / (locals.var_q_ndepm * locals.var_q_ndepm))) * locals.var_w_s0) - (assign25160_body1_e21409 * locals.var_w_s0_dn0)) / (locals.var_w_s0 * locals.var_w_s0)) + ((((((-((1.034943e-10 * locals.var_q_ndepm_dn0) / (locals.var_q_ndepm * locals.var_q_ndepm))) * assign25160_body1_e21421) + (assign25160_body1_e21414 * (-(((locals.var_ndepmpnsub_dn0 * assign25160_body1_e21419) - (locals.var_ndepmpnsub * locals.var_ndepmpnsub_dn0)) / (assign25160_body1_e21419 * assign25160_body1_e21419))))) * locals.var_w_b0) - (assign25160_body1_e21422 * locals.var_w_b0_dn0)) / (locals.var_w_b0 * locals.var_w_b0))), (((((-((1.034943e-10 * locals.var_q_ndepm_dn2) / (locals.var_q_ndepm * locals.var_q_ndepm))) * locals.var_w_s0) - (assign25160_body1_e21409 * locals.var_w_s0_dn2)) / (locals.var_w_s0 * locals.var_w_s0)) + ((((((-((1.034943e-10 * locals.var_q_ndepm_dn2) / (locals.var_q_ndepm * locals.var_q_ndepm))) * assign25160_body1_e21421) + (assign25160_body1_e21414 * (-(((locals.var_ndepmpnsub_dn2 * assign25160_body1_e21419) - (locals.var_ndepmpnsub * locals.var_ndepmpnsub_dn2)) / (assign25160_body1_e21419 * assign25160_body1_e21419))))) * locals.var_w_b0) - (assign25160_body1_e21422 * locals.var_w_b0_dn2)) / (locals.var_w_b0 * locals.var_w_b0))), (((((-((1.034943e-10 * locals.var_q_ndepm_dn4) / (locals.var_q_ndepm * locals.var_q_ndepm))) * locals.var_w_s0) - (assign25160_body1_e21409 * locals.var_w_s0_dn4)) / (locals.var_w_s0 * locals.var_w_s0)) + ((((((-((1.034943e-10 * locals.var_q_ndepm_dn4) / (locals.var_q_ndepm * locals.var_q_ndepm))) * assign25160_body1_e21421) + (assign25160_body1_e21414 * (-(((locals.var_ndepmpnsub_dn4 * assign25160_body1_e21419) - (locals.var_ndepmpnsub * locals.var_ndepmpnsub_dn4)) / (assign25160_body1_e21419 * assign25160_body1_e21419))))) * locals.var_w_b0) - (assign25160_body1_e21422 * locals.var_w_b0_dn4)) / (locals.var_w_b0 * locals.var_w_b0))), (((((-((1.034943e-10 * locals.var_q_ndepm_dn5) / (locals.var_q_ndepm * locals.var_q_ndepm))) * locals.var_w_s0) - (assign25160_body1_e21409 * locals.var_w_s0_dn5)) / (locals.var_w_s0 * locals.var_w_s0)) + ((((((-((1.034943e-10 * locals.var_q_ndepm_dn5) / (locals.var_q_ndepm * locals.var_q_ndepm))) * assign25160_body1_e21421) + (assign25160_body1_e21414 * (-(((locals.var_ndepmpnsub_dn5 * assign25160_body1_e21419) - (locals.var_ndepmpnsub * locals.var_ndepmpnsub_dn5)) / (assign25160_body1_e21419 * assign25160_body1_e21419))))) * locals.var_w_b0) - (assign25160_body1_e21422 * locals.var_w_b0_dn5)) / (locals.var_w_b0 * locals.var_w_b0))), (((((-((1.034943e-10 * locals.var_q_ndepm_dn6) / (locals.var_q_ndepm * locals.var_q_ndepm))) * locals.var_w_s0) - (assign25160_body1_e21409 * locals.var_w_s0_dn6)) / (locals.var_w_s0 * locals.var_w_s0)) + ((((((-((1.034943e-10 * locals.var_q_ndepm_dn6) / (locals.var_q_ndepm * locals.var_q_ndepm))) * assign25160_body1_e21421) + (assign25160_body1_e21414 * (-(((locals.var_ndepmpnsub_dn6 * assign25160_body1_e21419) - (locals.var_ndepmpnsub * locals.var_ndepmpnsub_dn6)) / (assign25160_body1_e21419 * assign25160_body1_e21419))))) * locals.var_w_b0) - (assign25160_body1_e21422 * locals.var_w_b0_dn6)) / (locals.var_w_b0 * locals.var_w_b0))), (((((-((1.034943e-10 * locals.var_q_ndepm_dn7) / (locals.var_q_ndepm * locals.var_q_ndepm))) * locals.var_w_s0) - (assign25160_body1_e21409 * locals.var_w_s0_dn7)) / (locals.var_w_s0 * locals.var_w_s0)) + ((((((-((1.034943e-10 * locals.var_q_ndepm_dn7) / (locals.var_q_ndepm * locals.var_q_ndepm))) * assign25160_body1_e21421) + (assign25160_body1_e21414 * (-(((locals.var_ndepmpnsub_dn7 * assign25160_body1_e21419) - (locals.var_ndepmpnsub * locals.var_ndepmpnsub_dn7)) / (assign25160_body1_e21419 * assign25160_body1_e21419))))) * locals.var_w_b0) - (assign25160_body1_e21422 * locals.var_w_b0_dn7)) / (locals.var_w_b0 * locals.var_w_b0))), (((((-((1.034943e-10 * locals.var_q_ndepm_dn8) / (locals.var_q_ndepm * locals.var_q_ndepm))) * locals.var_w_s0) - (assign25160_body1_e21409 * locals.var_w_s0_dn8)) / (locals.var_w_s0 * locals.var_w_s0)) + ((((((-((1.034943e-10 * locals.var_q_ndepm_dn8) / (locals.var_q_ndepm * locals.var_q_ndepm))) * assign25160_body1_e21421) + (assign25160_body1_e21414 * (-(((locals.var_ndepmpnsub_dn8 * assign25160_body1_e21419) - (locals.var_ndepmpnsub * locals.var_ndepmpnsub_dn8)) / (assign25160_body1_e21419 * assign25160_body1_e21419))))) * locals.var_w_b0) - (assign25160_body1_e21422 * locals.var_w_b0_dn8)) / (locals.var_w_b0 * locals.var_w_b0))), (((((-((1.034943e-10 * locals.var_q_ndepm_dn9) / (locals.var_q_ndepm * locals.var_q_ndepm))) * locals.var_w_s0) - (assign25160_body1_e21409 * locals.var_w_s0_dn9)) / (locals.var_w_s0 * locals.var_w_s0)) + ((((((-((1.034943e-10 * locals.var_q_ndepm_dn9) / (locals.var_q_ndepm * locals.var_q_ndepm))) * assign25160_body1_e21421) + (assign25160_body1_e21414 * (-(((locals.var_ndepmpnsub_dn9 * assign25160_body1_e21419) - (locals.var_ndepmpnsub * locals.var_ndepmpnsub_dn9)) / (assign25160_body1_e21419 * assign25160_body1_e21419))))) * locals.var_w_b0) - (assign25160_body1_e21422 * locals.var_w_b0_dn9)) / (locals.var_w_b0 * locals.var_w_b0))), (((((-((1.034943e-10 * locals.var_q_ndepm_dn10) / (locals.var_q_ndepm * locals.var_q_ndepm))) * locals.var_w_s0) - (assign25160_body1_e21409 * locals.var_w_s0_dn10)) / (locals.var_w_s0 * locals.var_w_s0)) + ((((((-((1.034943e-10 * locals.var_q_ndepm_dn10) / (locals.var_q_ndepm * locals.var_q_ndepm))) * assign25160_body1_e21421) + (assign25160_body1_e21414 * (-(((locals.var_ndepmpnsub_dn10 * assign25160_body1_e21419) - (locals.var_ndepmpnsub * locals.var_ndepmpnsub_dn10)) / (assign25160_body1_e21419 * assign25160_body1_e21419))))) * locals.var_w_b0) - (assign25160_body1_e21422 * locals.var_w_b0_dn10)) / (locals.var_w_b0 * locals.var_w_b0))), (((((-((1.034943e-10 * locals.var_q_ndepm_dn13) / (locals.var_q_ndepm * locals.var_q_ndepm))) * locals.var_w_s0) - (assign25160_body1_e21409 * locals.var_w_s0_dn13)) / (locals.var_w_s0 * locals.var_w_s0)) + ((((((-((1.034943e-10 * locals.var_q_ndepm_dn13) / (locals.var_q_ndepm * locals.var_q_ndepm))) * assign25160_body1_e21421) + (assign25160_body1_e21414 * (-(((locals.var_ndepmpnsub_dn13 * assign25160_body1_e21419) - (locals.var_ndepmpnsub * locals.var_ndepmpnsub_dn13)) / (assign25160_body1_e21419 * assign25160_body1_e21419))))) * locals.var_w_b0) - (assign25160_body1_e21422 * locals.var_w_b0_dn13)) / (locals.var_w_b0 * locals.var_w_b0))),)
    } else {
        (locals.var_dydpsm, locals.var_dydpsm_dn0, locals.var_dydpsm_dn2, locals.var_dydpsm_dn4, locals.var_dydpsm_dn5, locals.var_dydpsm_dn6, locals.var_dydpsm_dn7, locals.var_dydpsm_dn8, locals.var_dydpsm_dn9, locals.var_dydpsm_dn10, locals.var_dydpsm_dn13,)
    }
};
            locals.var_dydpsm = assign25160_body1_e21427;
            locals.var_dydpsm_dn0 = assign25160_body1_e21427_d_n0;
            locals.var_dydpsm_dn2 = assign25160_body1_e21427_d_n2;
            locals.var_dydpsm_dn4 = assign25160_body1_e21427_d_n4;
            locals.var_dydpsm_dn5 = assign25160_body1_e21427_d_n5;
            locals.var_dydpsm_dn6 = assign25160_body1_e21427_d_n6;
            locals.var_dydpsm_dn7 = assign25160_body1_e21427_d_n7;
            locals.var_dydpsm_dn8 = assign25160_body1_e21427_d_n8;
            locals.var_dydpsm_dn9 = assign25160_body1_e21427_d_n9;
            locals.var_dydpsm_dn10 = assign25160_body1_e21427_d_n10;
            locals.var_dydpsm_dn13 = assign25160_body1_e21427_d_n13;
            locals.var_dydpsm_rv = 0.0;
            let assign25160_body2_e21430: f64 = (locals.var_y0 / locals.var_dydpsm);
            let assign25160_body2_e21431: f64 = (assign25160_body2_e21430).abs();
            let assign25160_body2_e21433: f64 = if assign25160_body2_e21431 > 0.5 { 1.0 } else { 0.0 };
            locals.var_guard591 = assign25160_body2_e21433;
            locals.var_guard591_rv = 0.0;
            let (assign25160_body3_e21464, assign25160_body3_e21464_d_n0, assign25160_body3_e21464_d_n2, assign25160_body3_e21464_d_n4, assign25160_body3_e21464_d_n5, assign25160_body3_e21464_d_n6, assign25160_body3_e21464_d_n7, assign25160_body3_e21464_d_n8, assign25160_body3_e21464_d_n9, assign25160_body3_e21464_d_n10, assign25160_body3_e21464_d_n13,) = {
    if (((((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard578 == 0.0)) && (locals.var_guard580 == 0.0)) && (locals.var_guard581 == 0.0)) && (locals.var_guard590 != 0.0)) && (locals.var_guard591 != 0.0)) {
        let assign25160_body3_e21454: f64 = (locals.var_y0 / locals.var_dydpsm);
        let (assign25160_body3_e21460,) = {
            if (assign25160_body3_e21454 >= 0.0) {
                (1.0,)
            } else {
                let assign25160_body3_e21459: f64 = (-1.0);
                (assign25160_body3_e21459,)
            }
        };
        let assign25160_body3_e21461: f64 = (0.5 * assign25160_body3_e21460);
        let assign25160_body3_e21462: f64 = (locals.var_phi_b0_dep - assign25160_body3_e21461);
        (assign25160_body3_e21462, locals.var_phi_b0_dep_dn0, locals.var_phi_b0_dep_dn2, locals.var_phi_b0_dep_dn4, locals.var_phi_b0_dep_dn5, locals.var_phi_b0_dep_dn6, locals.var_phi_b0_dep_dn7, locals.var_phi_b0_dep_dn8, locals.var_phi_b0_dep_dn9, locals.var_phi_b0_dep_dn10, locals.var_phi_b0_dep_dn13,)
    } else {
        (locals.var_phi_b0_dep, locals.var_phi_b0_dep_dn0, locals.var_phi_b0_dep_dn2, locals.var_phi_b0_dep_dn4, locals.var_phi_b0_dep_dn5, locals.var_phi_b0_dep_dn6, locals.var_phi_b0_dep_dn7, locals.var_phi_b0_dep_dn8, locals.var_phi_b0_dep_dn9, locals.var_phi_b0_dep_dn10, locals.var_phi_b0_dep_dn13,)
    }
};
            locals.var_phi_b0_dep = assign25160_body3_e21464;
            locals.var_phi_b0_dep_dn0 = assign25160_body3_e21464_d_n0;
            locals.var_phi_b0_dep_dn2 = assign25160_body3_e21464_d_n2;
            locals.var_phi_b0_dep_dn4 = assign25160_body3_e21464_d_n4;
            locals.var_phi_b0_dep_dn5 = assign25160_body3_e21464_d_n5;
            locals.var_phi_b0_dep_dn6 = assign25160_body3_e21464_d_n6;
            locals.var_phi_b0_dep_dn7 = assign25160_body3_e21464_d_n7;
            locals.var_phi_b0_dep_dn8 = assign25160_body3_e21464_d_n8;
            locals.var_phi_b0_dep_dn9 = assign25160_body3_e21464_d_n9;
            locals.var_phi_b0_dep_dn10 = assign25160_body3_e21464_d_n10;
            locals.var_phi_b0_dep_dn13 = assign25160_body3_e21464_d_n13;
            locals.var_phi_b0_dep_rv = 0.0;
            let (assign25160_body4_e21488, assign25160_body4_e21488_d_n0, assign25160_body4_e21488_d_n2, assign25160_body4_e21488_d_n4, assign25160_body4_e21488_d_n5, assign25160_body4_e21488_d_n6, assign25160_body4_e21488_d_n7, assign25160_body4_e21488_d_n8, assign25160_body4_e21488_d_n9, assign25160_body4_e21488_d_n10, assign25160_body4_e21488_d_n13,) = {
    if (((((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard578 == 0.0)) && (locals.var_guard580 == 0.0)) && (locals.var_guard581 == 0.0)) && (locals.var_guard590 != 0.0)) && (locals.var_guard591 == 0.0)) {
        let assign25160_body4_e21485: f64 = (locals.var_y0 / locals.var_dydpsm);
        let assign25160_body4_e21486: f64 = (locals.var_phi_b0_dep - assign25160_body4_e21485);
        (assign25160_body4_e21486, (locals.var_phi_b0_dep_dn0 - (((locals.var_y0_dn0 * locals.var_dydpsm) - (locals.var_y0 * locals.var_dydpsm_dn0)) / (locals.var_dydpsm * locals.var_dydpsm))), (locals.var_phi_b0_dep_dn2 - (((locals.var_y0_dn2 * locals.var_dydpsm) - (locals.var_y0 * locals.var_dydpsm_dn2)) / (locals.var_dydpsm * locals.var_dydpsm))), (locals.var_phi_b0_dep_dn4 - (((locals.var_y0_dn4 * locals.var_dydpsm) - (locals.var_y0 * locals.var_dydpsm_dn4)) / (locals.var_dydpsm * locals.var_dydpsm))), (locals.var_phi_b0_dep_dn5 - (((locals.var_y0_dn5 * locals.var_dydpsm) - (locals.var_y0 * locals.var_dydpsm_dn5)) / (locals.var_dydpsm * locals.var_dydpsm))), (locals.var_phi_b0_dep_dn6 - (((locals.var_y0_dn6 * locals.var_dydpsm) - (locals.var_y0 * locals.var_dydpsm_dn6)) / (locals.var_dydpsm * locals.var_dydpsm))), (locals.var_phi_b0_dep_dn7 - (((locals.var_y0_dn7 * locals.var_dydpsm) - (locals.var_y0 * locals.var_dydpsm_dn7)) / (locals.var_dydpsm * locals.var_dydpsm))), (locals.var_phi_b0_dep_dn8 - (((locals.var_y0_dn8 * locals.var_dydpsm) - (locals.var_y0 * locals.var_dydpsm_dn8)) / (locals.var_dydpsm * locals.var_dydpsm))), (locals.var_phi_b0_dep_dn9 - (((locals.var_y0_dn9 * locals.var_dydpsm) - (locals.var_y0 * locals.var_dydpsm_dn9)) / (locals.var_dydpsm * locals.var_dydpsm))), (locals.var_phi_b0_dep_dn10 - (((locals.var_y0_dn10 * locals.var_dydpsm) - (locals.var_y0 * locals.var_dydpsm_dn10)) / (locals.var_dydpsm * locals.var_dydpsm))), (locals.var_phi_b0_dep_dn13 - (((locals.var_y0_dn13 * locals.var_dydpsm) - (locals.var_y0 * locals.var_dydpsm_dn13)) / (locals.var_dydpsm * locals.var_dydpsm))),)
    } else {
        (locals.var_phi_b0_dep, locals.var_phi_b0_dep_dn0, locals.var_phi_b0_dep_dn2, locals.var_phi_b0_dep_dn4, locals.var_phi_b0_dep_dn5, locals.var_phi_b0_dep_dn6, locals.var_phi_b0_dep_dn7, locals.var_phi_b0_dep_dn8, locals.var_phi_b0_dep_dn9, locals.var_phi_b0_dep_dn10, locals.var_phi_b0_dep_dn13,)
    }
};
            locals.var_phi_b0_dep = assign25160_body4_e21488;
            locals.var_phi_b0_dep_dn0 = assign25160_body4_e21488_d_n0;
            locals.var_phi_b0_dep_dn2 = assign25160_body4_e21488_d_n2;
            locals.var_phi_b0_dep_dn4 = assign25160_body4_e21488_d_n4;
            locals.var_phi_b0_dep_dn5 = assign25160_body4_e21488_d_n5;
            locals.var_phi_b0_dep_dn6 = assign25160_body4_e21488_d_n6;
            locals.var_phi_b0_dep_dn7 = assign25160_body4_e21488_d_n7;
            locals.var_phi_b0_dep_dn8 = assign25160_body4_e21488_d_n8;
            locals.var_phi_b0_dep_dn9 = assign25160_body4_e21488_d_n9;
            locals.var_phi_b0_dep_dn10 = assign25160_body4_e21488_d_n10;
            locals.var_phi_b0_dep_dn13 = assign25160_body4_e21488_d_n13;
            locals.var_phi_b0_dep_rv = 0.0;
            let assign25160_body5_e21491: f64 = (locals.var_phi_b0_dep - locals.var_vbscl__blk435);
            let assign25160_body5_e21493: f64 = (assign25160_body5_e21491 + locals.var_vbi_dep);
            let assign25160_body5_e21496: f64 = (10.0 * 2.220446049250313e-16);
            let assign25160_body5_e21497: f64 = if assign25160_body5_e21493 < assign25160_body5_e21496 { 1.0 } else { 0.0 };
            locals.var_guard592 = assign25160_body5_e21497;
            locals.var_guard592_rv = 0.0;
            let (assign25160_body6_e21522, assign25160_body6_e21522_d_n0, assign25160_body6_e21522_d_n2, assign25160_body6_e21522_d_n4, assign25160_body6_e21522_d_n5, assign25160_body6_e21522_d_n6, assign25160_body6_e21522_d_n7, assign25160_body6_e21522_d_n8, assign25160_body6_e21522_d_n9, assign25160_body6_e21522_d_n10, assign25160_body6_e21522_d_n13,) = {
    if (((((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard578 == 0.0)) && (locals.var_guard580 == 0.0)) && (locals.var_guard581 == 0.0)) && (locals.var_guard590 != 0.0)) && (locals.var_guard592 != 0.0)) {
        let assign25160_body6_e21516: f64 = (locals.var_vbscl__blk435 - locals.var_vbi_dep);
        let assign25160_body6_e21519: f64 = (10.0 * 2.220446049250313e-16);
        let assign25160_body6_e21520: f64 = (assign25160_body6_e21516 + assign25160_body6_e21519);
        (assign25160_body6_e21520, (locals.var_vbscl__blk435_dn0 - locals.var_vbi_dep_dn0), (locals.var_vbscl__blk435_dn2 - locals.var_vbi_dep_dn2), (locals.var_vbscl__blk435_dn4 - locals.var_vbi_dep_dn4), (locals.var_vbscl__blk435_dn5 - locals.var_vbi_dep_dn5), (locals.var_vbscl__blk435_dn6 - locals.var_vbi_dep_dn6), (locals.var_vbscl__blk435_dn7 - locals.var_vbi_dep_dn7), (locals.var_vbscl__blk435_dn8 - locals.var_vbi_dep_dn8), (locals.var_vbscl__blk435_dn9 - locals.var_vbi_dep_dn9), (locals.var_vbscl__blk435_dn10 - locals.var_vbi_dep_dn10), (locals.var_vbscl__blk435_dn13 - locals.var_vbi_dep_dn13),)
    } else {
        (locals.var_phi_b0_dep, locals.var_phi_b0_dep_dn0, locals.var_phi_b0_dep_dn2, locals.var_phi_b0_dep_dn4, locals.var_phi_b0_dep_dn5, locals.var_phi_b0_dep_dn6, locals.var_phi_b0_dep_dn7, locals.var_phi_b0_dep_dn8, locals.var_phi_b0_dep_dn9, locals.var_phi_b0_dep_dn10, locals.var_phi_b0_dep_dn13,)
    }
};
            locals.var_phi_b0_dep = assign25160_body6_e21522;
            locals.var_phi_b0_dep_dn0 = assign25160_body6_e21522_d_n0;
            locals.var_phi_b0_dep_dn2 = assign25160_body6_e21522_d_n2;
            locals.var_phi_b0_dep_dn4 = assign25160_body6_e21522_d_n4;
            locals.var_phi_b0_dep_dn5 = assign25160_body6_e21522_d_n5;
            locals.var_phi_b0_dep_dn6 = assign25160_body6_e21522_d_n6;
            locals.var_phi_b0_dep_dn7 = assign25160_body6_e21522_d_n7;
            locals.var_phi_b0_dep_dn8 = assign25160_body6_e21522_d_n8;
            locals.var_phi_b0_dep_dn9 = assign25160_body6_e21522_d_n9;
            locals.var_phi_b0_dep_dn10 = assign25160_body6_e21522_d_n10;
            locals.var_phi_b0_dep_dn13 = assign25160_body6_e21522_d_n13;
            locals.var_phi_b0_dep_rv = 0.0;
            let (assign25160_body7_e21544, assign25160_body7_e21544_d_n0, assign25160_body7_e21544_d_n2, assign25160_body7_e21544_d_n4, assign25160_body7_e21544_d_n5, assign25160_body7_e21544_d_n6, assign25160_body7_e21544_d_n7, assign25160_body7_e21544_d_n8, assign25160_body7_e21544_d_n9, assign25160_body7_e21544_d_n10, assign25160_body7_e21544_d_n13,) = {
    if ((((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard578 == 0.0)) && (locals.var_guard580 == 0.0)) && (locals.var_guard581 == 0.0)) && (locals.var_guard590 != 0.0)) {
        let assign25160_body7_e21540: f64 = (locals.var_phi_b0_dep - locals.var_phi_s0_dep_ini);
        let assign25160_body7_e21541: f64 = (locals.var_c_2esipq_ndepm * assign25160_body7_e21540);
        let assign25160_body7_e21542: f64 = (assign25160_body7_e21541).sqrt();
        (assign25160_body7_e21542, (((locals.var_c_2esipq_ndepm_dn0 * assign25160_body7_e21540) + (locals.var_c_2esipq_ndepm * (locals.var_phi_b0_dep_dn0 - locals.var_phi_s0_dep_ini_dn0))) / (2.0 * assign25160_body7_e21542)), (((locals.var_c_2esipq_ndepm_dn2 * assign25160_body7_e21540) + (locals.var_c_2esipq_ndepm * (locals.var_phi_b0_dep_dn2 - locals.var_phi_s0_dep_ini_dn2))) / (2.0 * assign25160_body7_e21542)), (((locals.var_c_2esipq_ndepm_dn4 * assign25160_body7_e21540) + (locals.var_c_2esipq_ndepm * (locals.var_phi_b0_dep_dn4 - locals.var_phi_s0_dep_ini_dn4))) / (2.0 * assign25160_body7_e21542)), (((locals.var_c_2esipq_ndepm_dn5 * assign25160_body7_e21540) + (locals.var_c_2esipq_ndepm * (locals.var_phi_b0_dep_dn5 - locals.var_phi_s0_dep_ini_dn5))) / (2.0 * assign25160_body7_e21542)), (((locals.var_c_2esipq_ndepm_dn6 * assign25160_body7_e21540) + (locals.var_c_2esipq_ndepm * (locals.var_phi_b0_dep_dn6 - locals.var_phi_s0_dep_ini_dn6))) / (2.0 * assign25160_body7_e21542)), (((locals.var_c_2esipq_ndepm_dn7 * assign25160_body7_e21540) + (locals.var_c_2esipq_ndepm * (locals.var_phi_b0_dep_dn7 - locals.var_phi_s0_dep_ini_dn7))) / (2.0 * assign25160_body7_e21542)), (((locals.var_c_2esipq_ndepm_dn8 * assign25160_body7_e21540) + (locals.var_c_2esipq_ndepm * (locals.var_phi_b0_dep_dn8 - locals.var_phi_s0_dep_ini_dn8))) / (2.0 * assign25160_body7_e21542)), (((locals.var_c_2esipq_ndepm_dn9 * assign25160_body7_e21540) + (locals.var_c_2esipq_ndepm * (locals.var_phi_b0_dep_dn9 - locals.var_phi_s0_dep_ini_dn9))) / (2.0 * assign25160_body7_e21542)), (((locals.var_c_2esipq_ndepm_dn10 * assign25160_body7_e21540) + (locals.var_c_2esipq_ndepm * (locals.var_phi_b0_dep_dn10 - locals.var_phi_s0_dep_ini_dn10))) / (2.0 * assign25160_body7_e21542)), (((locals.var_c_2esipq_ndepm_dn13 * assign25160_body7_e21540) + (locals.var_c_2esipq_ndepm * (locals.var_phi_b0_dep_dn13 - locals.var_phi_s0_dep_ini_dn13))) / (2.0 * assign25160_body7_e21542)),)
    } else {
        (locals.var_w_s0, locals.var_w_s0_dn0, locals.var_w_s0_dn2, locals.var_w_s0_dn4, locals.var_w_s0_dn5, locals.var_w_s0_dn6, locals.var_w_s0_dn7, locals.var_w_s0_dn8, locals.var_w_s0_dn9, locals.var_w_s0_dn10, locals.var_w_s0_dn13,)
    }
};
            locals.var_w_s0 = assign25160_body7_e21544;
            locals.var_w_s0_dn0 = assign25160_body7_e21544_d_n0;
            locals.var_w_s0_dn2 = assign25160_body7_e21544_d_n2;
            locals.var_w_s0_dn4 = assign25160_body7_e21544_d_n4;
            locals.var_w_s0_dn5 = assign25160_body7_e21544_d_n5;
            locals.var_w_s0_dn6 = assign25160_body7_e21544_d_n6;
            locals.var_w_s0_dn7 = assign25160_body7_e21544_d_n7;
            locals.var_w_s0_dn8 = assign25160_body7_e21544_d_n8;
            locals.var_w_s0_dn9 = assign25160_body7_e21544_d_n9;
            locals.var_w_s0_dn10 = assign25160_body7_e21544_d_n10;
            locals.var_w_s0_dn13 = assign25160_body7_e21544_d_n13;
            locals.var_w_s0_rv = 0.0;
            let (assign25160_body8_e21571, assign25160_body8_e21571_d_n0, assign25160_body8_e21571_d_n2, assign25160_body8_e21571_d_n4, assign25160_body8_e21571_d_n5, assign25160_body8_e21571_d_n6, assign25160_body8_e21571_d_n7, assign25160_body8_e21571_d_n8, assign25160_body8_e21571_d_n9, assign25160_body8_e21571_d_n10, assign25160_body8_e21571_d_n13,) = {
    if ((((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard578 == 0.0)) && (locals.var_guard580 == 0.0)) && (locals.var_guard581 == 0.0)) && (locals.var_guard590 != 0.0)) {
        let assign25160_body8_e21561: f64 = (locals.var_ndepmpnsub * locals.var_phi_b0_dep);
        let assign25160_body8_e21563: f64 = (assign25160_body8_e21561 + locals.var_vbscl__blk435);
        let assign25160_body8_e21565: f64 = (assign25160_body8_e21563 - locals.var_vbi_dep);
        let assign25160_body8_e21568: f64 = (1.0 + locals.var_ndepmpnsub);
        let assign25160_body8_e21569: f64 = (assign25160_body8_e21565 / assign25160_body8_e21568);
        (assign25160_body8_e21569, (((((((locals.var_ndepmpnsub_dn0 * locals.var_phi_b0_dep) + (locals.var_ndepmpnsub * locals.var_phi_b0_dep_dn0)) + locals.var_vbscl__blk435_dn0) - locals.var_vbi_dep_dn0) * assign25160_body8_e21568) - (assign25160_body8_e21565 * locals.var_ndepmpnsub_dn0)) / (assign25160_body8_e21568 * assign25160_body8_e21568)), (((((((locals.var_ndepmpnsub_dn2 * locals.var_phi_b0_dep) + (locals.var_ndepmpnsub * locals.var_phi_b0_dep_dn2)) + locals.var_vbscl__blk435_dn2) - locals.var_vbi_dep_dn2) * assign25160_body8_e21568) - (assign25160_body8_e21565 * locals.var_ndepmpnsub_dn2)) / (assign25160_body8_e21568 * assign25160_body8_e21568)), (((((((locals.var_ndepmpnsub_dn4 * locals.var_phi_b0_dep) + (locals.var_ndepmpnsub * locals.var_phi_b0_dep_dn4)) + locals.var_vbscl__blk435_dn4) - locals.var_vbi_dep_dn4) * assign25160_body8_e21568) - (assign25160_body8_e21565 * locals.var_ndepmpnsub_dn4)) / (assign25160_body8_e21568 * assign25160_body8_e21568)), (((((((locals.var_ndepmpnsub_dn5 * locals.var_phi_b0_dep) + (locals.var_ndepmpnsub * locals.var_phi_b0_dep_dn5)) + locals.var_vbscl__blk435_dn5) - locals.var_vbi_dep_dn5) * assign25160_body8_e21568) - (assign25160_body8_e21565 * locals.var_ndepmpnsub_dn5)) / (assign25160_body8_e21568 * assign25160_body8_e21568)), (((((((locals.var_ndepmpnsub_dn6 * locals.var_phi_b0_dep) + (locals.var_ndepmpnsub * locals.var_phi_b0_dep_dn6)) + locals.var_vbscl__blk435_dn6) - locals.var_vbi_dep_dn6) * assign25160_body8_e21568) - (assign25160_body8_e21565 * locals.var_ndepmpnsub_dn6)) / (assign25160_body8_e21568 * assign25160_body8_e21568)), (((((((locals.var_ndepmpnsub_dn7 * locals.var_phi_b0_dep) + (locals.var_ndepmpnsub * locals.var_phi_b0_dep_dn7)) + locals.var_vbscl__blk435_dn7) - locals.var_vbi_dep_dn7) * assign25160_body8_e21568) - (assign25160_body8_e21565 * locals.var_ndepmpnsub_dn7)) / (assign25160_body8_e21568 * assign25160_body8_e21568)), (((((((locals.var_ndepmpnsub_dn8 * locals.var_phi_b0_dep) + (locals.var_ndepmpnsub * locals.var_phi_b0_dep_dn8)) + locals.var_vbscl__blk435_dn8) - locals.var_vbi_dep_dn8) * assign25160_body8_e21568) - (assign25160_body8_e21565 * locals.var_ndepmpnsub_dn8)) / (assign25160_body8_e21568 * assign25160_body8_e21568)), (((((((locals.var_ndepmpnsub_dn9 * locals.var_phi_b0_dep) + (locals.var_ndepmpnsub * locals.var_phi_b0_dep_dn9)) + locals.var_vbscl__blk435_dn9) - locals.var_vbi_dep_dn9) * assign25160_body8_e21568) - (assign25160_body8_e21565 * locals.var_ndepmpnsub_dn9)) / (assign25160_body8_e21568 * assign25160_body8_e21568)), (((((((locals.var_ndepmpnsub_dn10 * locals.var_phi_b0_dep) + (locals.var_ndepmpnsub * locals.var_phi_b0_dep_dn10)) + locals.var_vbscl__blk435_dn10) - locals.var_vbi_dep_dn10) * assign25160_body8_e21568) - (assign25160_body8_e21565 * locals.var_ndepmpnsub_dn10)) / (assign25160_body8_e21568 * assign25160_body8_e21568)), (((((((locals.var_ndepmpnsub_dn13 * locals.var_phi_b0_dep) + (locals.var_ndepmpnsub * locals.var_phi_b0_dep_dn13)) + locals.var_vbscl__blk435_dn13) - locals.var_vbi_dep_dn13) * assign25160_body8_e21568) - (assign25160_body8_e21565 * locals.var_ndepmpnsub_dn13)) / (assign25160_body8_e21568 * assign25160_body8_e21568)),)
    } else {
        (locals.var_phi_j0_dep, locals.var_phi_j0_dep_dn0, locals.var_phi_j0_dep_dn2, locals.var_phi_j0_dep_dn4, locals.var_phi_j0_dep_dn5, locals.var_phi_j0_dep_dn6, locals.var_phi_j0_dep_dn7, locals.var_phi_j0_dep_dn8, locals.var_phi_j0_dep_dn9, locals.var_phi_j0_dep_dn10, locals.var_phi_j0_dep_dn13,)
    }
};
            locals.var_phi_j0_dep = assign25160_body8_e21571;
            locals.var_phi_j0_dep_dn0 = assign25160_body8_e21571_d_n0;
            locals.var_phi_j0_dep_dn2 = assign25160_body8_e21571_d_n2;
            locals.var_phi_j0_dep_dn4 = assign25160_body8_e21571_d_n4;
            locals.var_phi_j0_dep_dn5 = assign25160_body8_e21571_d_n5;
            locals.var_phi_j0_dep_dn6 = assign25160_body8_e21571_d_n6;
            locals.var_phi_j0_dep_dn7 = assign25160_body8_e21571_d_n7;
            locals.var_phi_j0_dep_dn8 = assign25160_body8_e21571_d_n8;
            locals.var_phi_j0_dep_dn9 = assign25160_body8_e21571_d_n9;
            locals.var_phi_j0_dep_dn10 = assign25160_body8_e21571_d_n10;
            locals.var_phi_j0_dep_dn13 = assign25160_body8_e21571_d_n13;
            locals.var_phi_j0_dep_rv = 0.0;
            let (assign25160_body9_e21593, assign25160_body9_e21593_d_n0, assign25160_body9_e21593_d_n2, assign25160_body9_e21593_d_n4, assign25160_body9_e21593_d_n5, assign25160_body9_e21593_d_n6, assign25160_body9_e21593_d_n7, assign25160_body9_e21593_d_n8, assign25160_body9_e21593_d_n9, assign25160_body9_e21593_d_n10, assign25160_body9_e21593_d_n13,) = {
    if ((((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard578 == 0.0)) && (locals.var_guard580 == 0.0)) && (locals.var_guard581 == 0.0)) && (locals.var_guard590 != 0.0)) {
        let assign25160_body9_e21589: f64 = (locals.var_phi_b0_dep - locals.var_phi_j0_dep);
        let assign25160_body9_e21590: f64 = (locals.var_c_2esipq_ndepm * assign25160_body9_e21589);
        let assign25160_body9_e21591: f64 = (assign25160_body9_e21590).sqrt();
        (assign25160_body9_e21591, (((locals.var_c_2esipq_ndepm_dn0 * assign25160_body9_e21589) + (locals.var_c_2esipq_ndepm * (locals.var_phi_b0_dep_dn0 - locals.var_phi_j0_dep_dn0))) / (2.0 * assign25160_body9_e21591)), (((locals.var_c_2esipq_ndepm_dn2 * assign25160_body9_e21589) + (locals.var_c_2esipq_ndepm * (locals.var_phi_b0_dep_dn2 - locals.var_phi_j0_dep_dn2))) / (2.0 * assign25160_body9_e21591)), (((locals.var_c_2esipq_ndepm_dn4 * assign25160_body9_e21589) + (locals.var_c_2esipq_ndepm * (locals.var_phi_b0_dep_dn4 - locals.var_phi_j0_dep_dn4))) / (2.0 * assign25160_body9_e21591)), (((locals.var_c_2esipq_ndepm_dn5 * assign25160_body9_e21589) + (locals.var_c_2esipq_ndepm * (locals.var_phi_b0_dep_dn5 - locals.var_phi_j0_dep_dn5))) / (2.0 * assign25160_body9_e21591)), (((locals.var_c_2esipq_ndepm_dn6 * assign25160_body9_e21589) + (locals.var_c_2esipq_ndepm * (locals.var_phi_b0_dep_dn6 - locals.var_phi_j0_dep_dn6))) / (2.0 * assign25160_body9_e21591)), (((locals.var_c_2esipq_ndepm_dn7 * assign25160_body9_e21589) + (locals.var_c_2esipq_ndepm * (locals.var_phi_b0_dep_dn7 - locals.var_phi_j0_dep_dn7))) / (2.0 * assign25160_body9_e21591)), (((locals.var_c_2esipq_ndepm_dn8 * assign25160_body9_e21589) + (locals.var_c_2esipq_ndepm * (locals.var_phi_b0_dep_dn8 - locals.var_phi_j0_dep_dn8))) / (2.0 * assign25160_body9_e21591)), (((locals.var_c_2esipq_ndepm_dn9 * assign25160_body9_e21589) + (locals.var_c_2esipq_ndepm * (locals.var_phi_b0_dep_dn9 - locals.var_phi_j0_dep_dn9))) / (2.0 * assign25160_body9_e21591)), (((locals.var_c_2esipq_ndepm_dn10 * assign25160_body9_e21589) + (locals.var_c_2esipq_ndepm * (locals.var_phi_b0_dep_dn10 - locals.var_phi_j0_dep_dn10))) / (2.0 * assign25160_body9_e21591)), (((locals.var_c_2esipq_ndepm_dn13 * assign25160_body9_e21589) + (locals.var_c_2esipq_ndepm * (locals.var_phi_b0_dep_dn13 - locals.var_phi_j0_dep_dn13))) / (2.0 * assign25160_body9_e21591)),)
    } else {
        (locals.var_w_b0, locals.var_w_b0_dn0, locals.var_w_b0_dn2, locals.var_w_b0_dn4, locals.var_w_b0_dn5, locals.var_w_b0_dn6, locals.var_w_b0_dn7, locals.var_w_b0_dn8, locals.var_w_b0_dn9, locals.var_w_b0_dn10, locals.var_w_b0_dn13,)
    }
};
            locals.var_w_b0 = assign25160_body9_e21593;
            locals.var_w_b0_dn0 = assign25160_body9_e21593_d_n0;
            locals.var_w_b0_dn2 = assign25160_body9_e21593_d_n2;
            locals.var_w_b0_dn4 = assign25160_body9_e21593_d_n4;
            locals.var_w_b0_dn5 = assign25160_body9_e21593_d_n5;
            locals.var_w_b0_dn6 = assign25160_body9_e21593_d_n6;
            locals.var_w_b0_dn7 = assign25160_body9_e21593_d_n7;
            locals.var_w_b0_dn8 = assign25160_body9_e21593_d_n8;
            locals.var_w_b0_dn9 = assign25160_body9_e21593_d_n9;
            locals.var_w_b0_dn10 = assign25160_body9_e21593_d_n10;
            locals.var_w_b0_dn13 = assign25160_body9_e21593_d_n13;
            locals.var_w_b0_rv = 0.0;
            let assign25160_body10_e21596: f64 = (locals.var_phi_b0_dep - locals.var_phi_b0_dep_old);
            let assign25160_body10_e21597: f64 = (assign25160_body10_e21596).abs();
            let assign25160_body10_e21599: f64 = if assign25160_body10_e21597 <= 1e-5 { 1.0 } else { 0.0 };
            locals.var_guard593 = assign25160_body10_e21599;
            locals.var_guard593_rv = 0.0;
            let (assign25160_body11_e21620,) = {
    if (((((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard578 == 0.0)) && (locals.var_guard580 == 0.0)) && (locals.var_guard581 == 0.0)) && (locals.var_guard590 != 0.0)) && (locals.var_guard593 != 0.0)) {
        let assign25160_body11_e21618: f64 = (locals.var_lp_s0_max + 1.0);
        (assign25160_body11_e21618,)
    } else {
        (locals.var_lp_s0,)
    }
};
            locals.var_lp_s0 = assign25160_body11_e21620;
            locals.var_lp_s0_rv = 0.0;
            let (assign25160_body12_e21637, assign25160_body12_e21637_d_n0, assign25160_body12_e21637_d_n2, assign25160_body12_e21637_d_n4, assign25160_body12_e21637_d_n5, assign25160_body12_e21637_d_n6, assign25160_body12_e21637_d_n7, assign25160_body12_e21637_d_n8, assign25160_body12_e21637_d_n9, assign25160_body12_e21637_d_n10, assign25160_body12_e21637_d_n13,) = {
    if ((((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard578 == 0.0)) && (locals.var_guard580 == 0.0)) && (locals.var_guard581 == 0.0)) && (locals.var_guard590 != 0.0)) {
        (locals.var_phi_b0_dep, locals.var_phi_b0_dep_dn0, locals.var_phi_b0_dep_dn2, locals.var_phi_b0_dep_dn4, locals.var_phi_b0_dep_dn5, locals.var_phi_b0_dep_dn6, locals.var_phi_b0_dep_dn7, locals.var_phi_b0_dep_dn8, locals.var_phi_b0_dep_dn9, locals.var_phi_b0_dep_dn10, locals.var_phi_b0_dep_dn13,)
    } else {
        (locals.var_phi_b0_dep_old, locals.var_phi_b0_dep_old_dn0, locals.var_phi_b0_dep_old_dn2, locals.var_phi_b0_dep_old_dn4, locals.var_phi_b0_dep_old_dn5, locals.var_phi_b0_dep_old_dn6, locals.var_phi_b0_dep_old_dn7, locals.var_phi_b0_dep_old_dn8, locals.var_phi_b0_dep_old_dn9, locals.var_phi_b0_dep_old_dn10, locals.var_phi_b0_dep_old_dn13,)
    }
};
            locals.var_phi_b0_dep_old = assign25160_body12_e21637;
            locals.var_phi_b0_dep_old_dn0 = assign25160_body12_e21637_d_n0;
            locals.var_phi_b0_dep_old_dn2 = assign25160_body12_e21637_d_n2;
            locals.var_phi_b0_dep_old_dn4 = assign25160_body12_e21637_d_n4;
            locals.var_phi_b0_dep_old_dn5 = assign25160_body12_e21637_d_n5;
            locals.var_phi_b0_dep_old_dn6 = assign25160_body12_e21637_d_n6;
            locals.var_phi_b0_dep_old_dn7 = assign25160_body12_e21637_d_n7;
            locals.var_phi_b0_dep_old_dn8 = assign25160_body12_e21637_d_n8;
            locals.var_phi_b0_dep_old_dn9 = assign25160_body12_e21637_d_n9;
            locals.var_phi_b0_dep_old_dn10 = assign25160_body12_e21637_d_n10;
            locals.var_phi_b0_dep_old_dn13 = assign25160_body12_e21637_d_n13;
            locals.var_phi_b0_dep_old_rv = 0.0;
            let (assign25160_body13_e21656,) = {
    if ((((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard578 == 0.0)) && (locals.var_guard580 == 0.0)) && (locals.var_guard581 == 0.0)) && (locals.var_guard590 != 0.0)) {
        let assign25160_body13_e21654: f64 = (locals.var_lp_s0 + 1.0);
        (assign25160_body13_e21654,)
    } else {
        (locals.var_lp_s0,)
    }
};
            locals.var_lp_s0 = assign25160_body13_e21656;
            locals.var_lp_s0_rv = 0.0;
        }

        let (assign25170_e21662, assign25170_e21662_d_n0, assign25170_e21662_d_n2, assign25170_e21662_d_n4, assign25170_e21662_d_n5, assign25170_e21662_d_n6, assign25170_e21662_d_n7, assign25170_e21662_d_n8, assign25170_e21662_d_n9, assign25170_e21662_d_n10, assign25170_e21662_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) {
        (locals.var_phi_b0_dep, locals.var_phi_b0_dep_dn0, locals.var_phi_b0_dep_dn2, locals.var_phi_b0_dep_dn4, locals.var_phi_b0_dep_dn5, locals.var_phi_b0_dep_dn6, locals.var_phi_b0_dep_dn7, locals.var_phi_b0_dep_dn8, locals.var_phi_b0_dep_dn9, locals.var_phi_b0_dep_dn10, locals.var_phi_b0_dep_dn13,)
    } else {
        (locals.var_phi_b0_dep_ini, locals.var_phi_b0_dep_ini_dn0, locals.var_phi_b0_dep_ini_dn2, locals.var_phi_b0_dep_ini_dn4, locals.var_phi_b0_dep_ini_dn5, locals.var_phi_b0_dep_ini_dn6, locals.var_phi_b0_dep_ini_dn7, locals.var_phi_b0_dep_ini_dn8, locals.var_phi_b0_dep_ini_dn9, locals.var_phi_b0_dep_ini_dn10, locals.var_phi_b0_dep_ini_dn13,)
    }
};
        locals.var_phi_b0_dep_ini = assign25170_e21662;
        locals.var_phi_b0_dep_ini_dn0 = assign25170_e21662_d_n0;
        locals.var_phi_b0_dep_ini_dn2 = assign25170_e21662_d_n2;
        locals.var_phi_b0_dep_ini_dn4 = assign25170_e21662_d_n4;
        locals.var_phi_b0_dep_ini_dn5 = assign25170_e21662_d_n5;
        locals.var_phi_b0_dep_ini_dn6 = assign25170_e21662_d_n6;
        locals.var_phi_b0_dep_ini_dn7 = assign25170_e21662_d_n7;
        locals.var_phi_b0_dep_ini_dn8 = assign25170_e21662_d_n8;
        locals.var_phi_b0_dep_ini_dn9 = assign25170_e21662_d_n9;
        locals.var_phi_b0_dep_ini_dn10 = assign25170_e21662_d_n10;
        locals.var_phi_b0_dep_ini_dn13 = assign25170_e21662_d_n13;
        locals.var_phi_b0_dep_ini_rv = 0.0;

        let (assign25180_e21668,) = {
    if ((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) {
        (0.12,)
    } else {
        (locals.var_sm_delta,)
    }
};
        locals.var_sm_delta = assign25180_e21668;
        locals.var_sm_delta_rv = 0.0;

        let (assign25190_e21674,) = {
    if ((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_flg_conv,)
    }
};
        locals.var_flg_conv = assign25190_e21674;
        locals.var_flg_conv_rv = 0.0;

        let (assign25200_e21680, assign25200_e21680_d_n0, assign25200_e21680_d_n2, assign25200_e21680_d_n4, assign25200_e21680_d_n5, assign25200_e21680_d_n6, assign25200_e21680_d_n7, assign25200_e21680_d_n8, assign25200_e21680_d_n9, assign25200_e21680_d_n10, assign25200_e21680_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) {
        (locals.var_phi_s0_dep_ini, locals.var_phi_s0_dep_ini_dn0, locals.var_phi_s0_dep_ini_dn2, locals.var_phi_s0_dep_ini_dn4, locals.var_phi_s0_dep_ini_dn5, locals.var_phi_s0_dep_ini_dn6, locals.var_phi_s0_dep_ini_dn7, locals.var_phi_s0_dep_ini_dn8, locals.var_phi_s0_dep_ini_dn9, locals.var_phi_s0_dep_ini_dn10, locals.var_phi_s0_dep_ini_dn13,)
    } else {
        (locals.var_phi_s0_dep, locals.var_phi_s0_dep_dn0, locals.var_phi_s0_dep_dn2, locals.var_phi_s0_dep_dn4, locals.var_phi_s0_dep_dn5, locals.var_phi_s0_dep_dn6, locals.var_phi_s0_dep_dn7, locals.var_phi_s0_dep_dn8, locals.var_phi_s0_dep_dn9, locals.var_phi_s0_dep_dn10, locals.var_phi_s0_dep_dn13,)
    }
};
        locals.var_phi_s0_dep = assign25200_e21680;
        locals.var_phi_s0_dep_dn0 = assign25200_e21680_d_n0;
        locals.var_phi_s0_dep_dn2 = assign25200_e21680_d_n2;
        locals.var_phi_s0_dep_dn4 = assign25200_e21680_d_n4;
        locals.var_phi_s0_dep_dn5 = assign25200_e21680_d_n5;
        locals.var_phi_s0_dep_dn6 = assign25200_e21680_d_n6;
        locals.var_phi_s0_dep_dn7 = assign25200_e21680_d_n7;
        locals.var_phi_s0_dep_dn8 = assign25200_e21680_d_n8;
        locals.var_phi_s0_dep_dn9 = assign25200_e21680_d_n9;
        locals.var_phi_s0_dep_dn10 = assign25200_e21680_d_n10;
        locals.var_phi_s0_dep_dn13 = assign25200_e21680_d_n13;
        locals.var_phi_s0_dep_rv = 0.0;

        let (assign25210_e21686, assign25210_e21686_d_n0, assign25210_e21686_d_n2, assign25210_e21686_d_n4, assign25210_e21686_d_n5, assign25210_e21686_d_n6, assign25210_e21686_d_n7, assign25210_e21686_d_n8, assign25210_e21686_d_n9, assign25210_e21686_d_n10, assign25210_e21686_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) {
        (locals.var_phi_b0_dep_ini, locals.var_phi_b0_dep_ini_dn0, locals.var_phi_b0_dep_ini_dn2, locals.var_phi_b0_dep_ini_dn4, locals.var_phi_b0_dep_ini_dn5, locals.var_phi_b0_dep_ini_dn6, locals.var_phi_b0_dep_ini_dn7, locals.var_phi_b0_dep_ini_dn8, locals.var_phi_b0_dep_ini_dn9, locals.var_phi_b0_dep_ini_dn10, locals.var_phi_b0_dep_ini_dn13,)
    } else {
        (locals.var_phi_b0_dep, locals.var_phi_b0_dep_dn0, locals.var_phi_b0_dep_dn2, locals.var_phi_b0_dep_dn4, locals.var_phi_b0_dep_dn5, locals.var_phi_b0_dep_dn6, locals.var_phi_b0_dep_dn7, locals.var_phi_b0_dep_dn8, locals.var_phi_b0_dep_dn9, locals.var_phi_b0_dep_dn10, locals.var_phi_b0_dep_dn13,)
    }
};
        locals.var_phi_b0_dep = assign25210_e21686;
        locals.var_phi_b0_dep_dn0 = assign25210_e21686_d_n0;
        locals.var_phi_b0_dep_dn2 = assign25210_e21686_d_n2;
        locals.var_phi_b0_dep_dn4 = assign25210_e21686_d_n4;
        locals.var_phi_b0_dep_dn5 = assign25210_e21686_d_n5;
        locals.var_phi_b0_dep_dn6 = assign25210_e21686_d_n6;
        locals.var_phi_b0_dep_dn7 = assign25210_e21686_d_n7;
        locals.var_phi_b0_dep_dn8 = assign25210_e21686_d_n8;
        locals.var_phi_b0_dep_dn9 = assign25210_e21686_d_n9;
        locals.var_phi_b0_dep_dn10 = assign25210_e21686_d_n10;
        locals.var_phi_b0_dep_dn13 = assign25210_e21686_d_n13;
        locals.var_phi_b0_dep_rv = 0.0;

        let (assign25220_e21692, assign25220_e21692_d_n0, assign25220_e21692_d_n2, assign25220_e21692_d_n4, assign25220_e21692_d_n5, assign25220_e21692_d_n6, assign25220_e21692_d_n7, assign25220_e21692_d_n8, assign25220_e21692_d_n9, assign25220_e21692_d_n10, assign25220_e21692_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) {
        (locals.var_phi_s0_dep, locals.var_phi_s0_dep_dn0, locals.var_phi_s0_dep_dn2, locals.var_phi_s0_dep_dn4, locals.var_phi_s0_dep_dn5, locals.var_phi_s0_dep_dn6, locals.var_phi_s0_dep_dn7, locals.var_phi_s0_dep_dn8, locals.var_phi_s0_dep_dn9, locals.var_phi_s0_dep_dn10, locals.var_phi_s0_dep_dn13,)
    } else {
        (locals.var_phi_s0_dep_old, locals.var_phi_s0_dep_old_dn0, locals.var_phi_s0_dep_old_dn2, locals.var_phi_s0_dep_old_dn4, locals.var_phi_s0_dep_old_dn5, locals.var_phi_s0_dep_old_dn6, locals.var_phi_s0_dep_old_dn7, locals.var_phi_s0_dep_old_dn8, locals.var_phi_s0_dep_old_dn9, locals.var_phi_s0_dep_old_dn10, locals.var_phi_s0_dep_old_dn13,)
    }
};
        locals.var_phi_s0_dep_old = assign25220_e21692;
        locals.var_phi_s0_dep_old_dn0 = assign25220_e21692_d_n0;
        locals.var_phi_s0_dep_old_dn2 = assign25220_e21692_d_n2;
        locals.var_phi_s0_dep_old_dn4 = assign25220_e21692_d_n4;
        locals.var_phi_s0_dep_old_dn5 = assign25220_e21692_d_n5;
        locals.var_phi_s0_dep_old_dn6 = assign25220_e21692_d_n6;
        locals.var_phi_s0_dep_old_dn7 = assign25220_e21692_d_n7;
        locals.var_phi_s0_dep_old_dn8 = assign25220_e21692_d_n8;
        locals.var_phi_s0_dep_old_dn9 = assign25220_e21692_d_n9;
        locals.var_phi_s0_dep_old_dn10 = assign25220_e21692_d_n10;
        locals.var_phi_s0_dep_old_dn13 = assign25220_e21692_d_n13;
        locals.var_phi_s0_dep_old_rv = 0.0;

        let (assign25230_e21698, assign25230_e21698_d_n0, assign25230_e21698_d_n2, assign25230_e21698_d_n4, assign25230_e21698_d_n5, assign25230_e21698_d_n6, assign25230_e21698_d_n7, assign25230_e21698_d_n8, assign25230_e21698_d_n9, assign25230_e21698_d_n10, assign25230_e21698_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) {
        (locals.var_phi_b0_dep, locals.var_phi_b0_dep_dn0, locals.var_phi_b0_dep_dn2, locals.var_phi_b0_dep_dn4, locals.var_phi_b0_dep_dn5, locals.var_phi_b0_dep_dn6, locals.var_phi_b0_dep_dn7, locals.var_phi_b0_dep_dn8, locals.var_phi_b0_dep_dn9, locals.var_phi_b0_dep_dn10, locals.var_phi_b0_dep_dn13,)
    } else {
        (locals.var_phi_b0_dep_old, locals.var_phi_b0_dep_old_dn0, locals.var_phi_b0_dep_old_dn2, locals.var_phi_b0_dep_old_dn4, locals.var_phi_b0_dep_old_dn5, locals.var_phi_b0_dep_old_dn6, locals.var_phi_b0_dep_old_dn7, locals.var_phi_b0_dep_old_dn8, locals.var_phi_b0_dep_old_dn9, locals.var_phi_b0_dep_old_dn10, locals.var_phi_b0_dep_old_dn13,)
    }
};
        locals.var_phi_b0_dep_old = assign25230_e21698;
        locals.var_phi_b0_dep_old_dn0 = assign25230_e21698_d_n0;
        locals.var_phi_b0_dep_old_dn2 = assign25230_e21698_d_n2;
        locals.var_phi_b0_dep_old_dn4 = assign25230_e21698_d_n4;
        locals.var_phi_b0_dep_old_dn5 = assign25230_e21698_d_n5;
        locals.var_phi_b0_dep_old_dn6 = assign25230_e21698_d_n6;
        locals.var_phi_b0_dep_old_dn7 = assign25230_e21698_d_n7;
        locals.var_phi_b0_dep_old_dn8 = assign25230_e21698_d_n8;
        locals.var_phi_b0_dep_old_dn9 = assign25230_e21698_d_n9;
        locals.var_phi_b0_dep_old_dn10 = assign25230_e21698_d_n10;
        locals.var_phi_b0_dep_old_dn13 = assign25230_e21698_d_n13;
        locals.var_phi_b0_dep_old_rv = 0.0;

        let (assign25240_e21704,) = {
    if ((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_lp_s0,)
    }
};
        locals.var_lp_s0 = assign25240_e21704;
        locals.var_lp_s0_rv = 0.0;

    }
}
