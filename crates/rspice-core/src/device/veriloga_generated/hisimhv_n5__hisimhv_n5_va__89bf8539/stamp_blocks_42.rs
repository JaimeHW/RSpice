#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_reactive_block_283(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign74780_e114327, assign74780_e114327_d_n0, assign74780_e114327_d_n2, assign74780_e114327_d_n4, assign74780_e114327_d_n5, assign74780_e114327_d_n6, assign74780_e114327_d_n7, assign74780_e114327_d_n8, assign74780_e114327_d_n9, assign74780_e114327_d_n10, assign74780_e114327_d_n11, assign74780_e114327_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && ((locals.var_guard1746 != 0.0) && (!(((locals.var_guard1743 != 0.0) || (locals.var_guard1744 != 0.0)) || (locals.var_guard1745 != 0.0))))) {
        let assign74780_e114325: f64 = (locals.var_t4 * locals.var_qbuld);
        (assign74780_e114325, ((locals.var_t4_dn0 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn0)), ((locals.var_t4_dn2 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn2)), ((locals.var_t4_dn4 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn4)), ((locals.var_t4_dn5 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn5)), ((locals.var_t4_dn6 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn6)), ((locals.var_t4_dn7 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn7)), ((locals.var_t4_dn8 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn8)), ((locals.var_t4_dn9 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn9)), ((locals.var_t4_dn10 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn10)), ((locals.var_t4_dn11 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn11)), ((locals.var_t4_dn14 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn14)),)
    } else {
        (locals.var_qbdldext, locals.var_qbdldext_dn0, locals.var_qbdldext_dn2, locals.var_qbdldext_dn4, locals.var_qbdldext_dn5, locals.var_qbdldext_dn6, locals.var_qbdldext_dn7, locals.var_qbdldext_dn8, locals.var_qbdldext_dn9, locals.var_qbdldext_dn10, locals.var_qbdldext_dn11, locals.var_qbdldext_dn14,)
    }
};
        locals.var_qbdldext = assign74780_e114327;
        locals.var_qbdldext_dn0 = assign74780_e114327_d_n0;
        locals.var_qbdldext_dn2 = assign74780_e114327_d_n2;
        locals.var_qbdldext_dn4 = assign74780_e114327_d_n4;
        locals.var_qbdldext_dn5 = assign74780_e114327_d_n5;
        locals.var_qbdldext_dn6 = assign74780_e114327_d_n6;
        locals.var_qbdldext_dn7 = assign74780_e114327_d_n7;
        locals.var_qbdldext_dn8 = assign74780_e114327_d_n8;
        locals.var_qbdldext_dn9 = assign74780_e114327_d_n9;
        locals.var_qbdldext_dn10 = assign74780_e114327_d_n10;
        locals.var_qbdldext_dn11 = assign74780_e114327_d_n11;
        locals.var_qbdldext_dn14 = assign74780_e114327_d_n14;
        locals.var_qbdldext_rv = 0.0;

        locals.var_flg_calcqover = 0.0;
        locals.var_flg_calcqover_rv = 0.0;

        let assign74800_e114331: f64 = if 2.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1750 = assign74800_e114331;
        locals.var_guard1750_rv = 0.0;

        let assign74810_e114334: f64 = if 2.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard1751 = assign74810_e114334;
        locals.var_guard1751_rv = 0.0;

        let assign74820_e114337: f64 = if 2.0 == 3.0 { 1.0 } else { 0.0 };
        locals.var_guard1752 = assign74820_e114337;
        locals.var_guard1752_rv = 0.0;

        let assign74830_e114340: f64 = if 2.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard1753 = assign74830_e114340;
        locals.var_guard1753_rv = 0.0;

        let assign74840_e114351: f64 = if (((p.p36 == 1.0) && (p.p66 > 0.0)) && (locals.var_uc_novers > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1754 = assign74840_e114351;
        locals.var_guard1754_rv = 0.0;

        let (assign74850_e114357,) = {
    if ((locals.var_guard1750 != 0.0) && (locals.var_guard1754 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_flg_calcqover,)
    }
};
        locals.var_flg_calcqover = assign74850_e114357;
        locals.var_flg_calcqover_rv = 0.0;

        let (assign74860_e114363,) = {
    if ((locals.var_guard1750 != 0.0) && (locals.var_guard1754 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_flg_coovlps,)
    }
};
        locals.var_flg_coovlps = assign74860_e114363;
        locals.var_flg_coovlps_rv = 0.0;

        let (assign74870_e114371, assign74870_e114371_d_n2, assign74870_e114371_d_n7, assign74870_e114371_d_n8, assign74870_e114371_d_n9,) = {
    if ((locals.var_guard1750 != 0.0) && (locals.var_guard1754 != 0.0)) {
        let assign74870_e114369: f64 = (locals.var_vgsi - locals.var_vbsi);
        (assign74870_e114369, 0.0, locals.var_vgsi_dn7, (locals.var_vgsi_dn8 - locals.var_vbsi_dn8), (-locals.var_vbsi_dn9),)
    } else {
        (locals.var_vgbgmt, locals.var_vgbgmt_dn2, locals.var_vgbgmt_dn7, locals.var_vgbgmt_dn8, locals.var_vgbgmt_dn9,)
    }
};
        locals.var_vgbgmt = assign74870_e114371;
        locals.var_vgbgmt_dn2 = assign74870_e114371_d_n2;
        locals.var_vgbgmt_dn7 = assign74870_e114371_d_n7;
        locals.var_vgbgmt_dn8 = assign74870_e114371_d_n8;
        locals.var_vgbgmt_dn9 = assign74870_e114371_d_n9;
        locals.var_vgbgmt_rv = 0.0;

        let (assign74880_e114378, assign74880_e114378_d_n0, assign74880_e114378_d_n2, assign74880_e114378_d_n4, assign74880_e114378_d_n5, assign74880_e114378_d_n6, assign74880_e114378_d_n7, assign74880_e114378_d_n8, assign74880_e114378_d_n9, assign74880_e114378_d_n10, assign74880_e114378_d_n11, assign74880_e114378_d_n14,) = {
    if ((locals.var_guard1750 != 0.0) && (locals.var_guard1754 != 0.0)) {
        let assign74880_e114376: f64 = (-locals.var_vbsi);
        (assign74880_e114376, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, (-locals.var_vbsi_dn8), (-locals.var_vbsi_dn9), 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vxbgmt, locals.var_vxbgmt_dn0, locals.var_vxbgmt_dn2, locals.var_vxbgmt_dn4, locals.var_vxbgmt_dn5, locals.var_vxbgmt_dn6, locals.var_vxbgmt_dn7, locals.var_vxbgmt_dn8, locals.var_vxbgmt_dn9, locals.var_vxbgmt_dn10, locals.var_vxbgmt_dn11, locals.var_vxbgmt_dn14,)
    }
};
        locals.var_vxbgmt = assign74880_e114378;
        locals.var_vxbgmt_dn0 = assign74880_e114378_d_n0;
        locals.var_vxbgmt_dn2 = assign74880_e114378_d_n2;
        locals.var_vxbgmt_dn4 = assign74880_e114378_d_n4;
        locals.var_vxbgmt_dn5 = assign74880_e114378_d_n5;
        locals.var_vxbgmt_dn6 = assign74880_e114378_d_n6;
        locals.var_vxbgmt_dn7 = assign74880_e114378_d_n7;
        locals.var_vxbgmt_dn8 = assign74880_e114378_d_n8;
        locals.var_vxbgmt_dn9 = assign74880_e114378_d_n9;
        locals.var_vxbgmt_dn10 = assign74880_e114378_d_n10;
        locals.var_vxbgmt_dn11 = assign74880_e114378_d_n11;
        locals.var_vxbgmt_dn14 = assign74880_e114378_d_n14;
        locals.var_vxbgmt_rv = 0.0;

        let (assign74890_e114384,) = {
    if ((locals.var_guard1750 != 0.0) && (locals.var_guard1754 != 0.0)) {
        (locals.var_uc_novers,)
    } else {
        (locals.var_nover_func,)
    }
};
        locals.var_nover_func = assign74890_e114384;
        locals.var_nover_func_rv = 0.0;

        let (assign74900_e114390, assign74900_e114390_d_n0, assign74900_e114390_d_n2, assign74900_e114390_d_n4, assign74900_e114390_d_n5, assign74900_e114390_d_n6, assign74900_e114390_d_n7, assign74900_e114390_d_n8, assign74900_e114390_d_n9, assign74900_e114390_d_n10, assign74900_e114390_d_n11, assign74900_e114390_d_n14,) = {
    if ((locals.var_guard1750 != 0.0) && (locals.var_guard1754 != 0.0)) {
        (p.p66, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_lover_func, locals.var_lover_func_dn0, locals.var_lover_func_dn2, locals.var_lover_func_dn4, locals.var_lover_func_dn5, locals.var_lover_func_dn6, locals.var_lover_func_dn7, locals.var_lover_func_dn8, locals.var_lover_func_dn9, locals.var_lover_func_dn10, locals.var_lover_func_dn11, locals.var_lover_func_dn14,)
    }
};
        locals.var_lover_func = assign74900_e114390;
        locals.var_lover_func_dn0 = assign74900_e114390_d_n0;
        locals.var_lover_func_dn2 = assign74900_e114390_d_n2;
        locals.var_lover_func_dn4 = assign74900_e114390_d_n4;
        locals.var_lover_func_dn5 = assign74900_e114390_d_n5;
        locals.var_lover_func_dn6 = assign74900_e114390_d_n6;
        locals.var_lover_func_dn7 = assign74900_e114390_d_n7;
        locals.var_lover_func_dn8 = assign74900_e114390_d_n8;
        locals.var_lover_func_dn9 = assign74900_e114390_d_n9;
        locals.var_lover_func_dn10 = assign74900_e114390_d_n10;
        locals.var_lover_func_dn11 = assign74900_e114390_d_n11;
        locals.var_lover_func_dn14 = assign74900_e114390_d_n14;
        locals.var_lover_func_rv = 0.0;

        let (assign74910_e114396, assign74910_e114396_d_n0, assign74910_e114396_d_n2, assign74910_e114396_d_n4, assign74910_e114396_d_n5, assign74910_e114396_d_n6, assign74910_e114396_d_n7, assign74910_e114396_d_n8, assign74910_e114396_d_n9, assign74910_e114396_d_n10, assign74910_e114396_d_n11, assign74910_e114396_d_n14,) = {
    if ((locals.var_guard1750 != 0.0) && (locals.var_guard1754 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_wdep_func, locals.var_wdep_func_dn0, locals.var_wdep_func_dn2, locals.var_wdep_func_dn4, locals.var_wdep_func_dn5, locals.var_wdep_func_dn6, locals.var_wdep_func_dn7, locals.var_wdep_func_dn8, locals.var_wdep_func_dn9, locals.var_wdep_func_dn10, locals.var_wdep_func_dn11, locals.var_wdep_func_dn14,)
    }
};
        locals.var_wdep_func = assign74910_e114396;
        locals.var_wdep_func_dn0 = assign74910_e114396_d_n0;
        locals.var_wdep_func_dn2 = assign74910_e114396_d_n2;
        locals.var_wdep_func_dn4 = assign74910_e114396_d_n4;
        locals.var_wdep_func_dn5 = assign74910_e114396_d_n5;
        locals.var_wdep_func_dn6 = assign74910_e114396_d_n6;
        locals.var_wdep_func_dn7 = assign74910_e114396_d_n7;
        locals.var_wdep_func_dn8 = assign74910_e114396_d_n8;
        locals.var_wdep_func_dn9 = assign74910_e114396_d_n9;
        locals.var_wdep_func_dn10 = assign74910_e114396_d_n10;
        locals.var_wdep_func_dn11 = assign74910_e114396_d_n11;
        locals.var_wdep_func_dn14 = assign74910_e114396_d_n14;
        locals.var_wdep_func_rv = 0.0;

        let (assign74920_e114402, assign74920_e114402_d_n0, assign74920_e114402_d_n2, assign74920_e114402_d_n4, assign74920_e114402_d_n5, assign74920_e114402_d_n6, assign74920_e114402_d_n7, assign74920_e114402_d_n8, assign74920_e114402_d_n9, assign74920_e114402_d_n10, assign74920_e114402_d_n11, assign74920_e114402_d_n14,) = {
    if ((locals.var_guard1750 != 0.0) && (locals.var_guard1754 != 0.0)) {
        (locals.var_cnst0overs, locals.var_cnst0overs_dn0, locals.var_cnst0overs_dn2, locals.var_cnst0overs_dn4, locals.var_cnst0overs_dn5, locals.var_cnst0overs_dn6, locals.var_cnst0overs_dn7, locals.var_cnst0overs_dn8, locals.var_cnst0overs_dn9, locals.var_cnst0overs_dn10, locals.var_cnst0overs_dn11, locals.var_cnst0overs_dn14,)
    } else {
        (locals.var_cnst0over_func, locals.var_cnst0over_func_dn0, locals.var_cnst0over_func_dn2, locals.var_cnst0over_func_dn4, locals.var_cnst0over_func_dn5, locals.var_cnst0over_func_dn6, locals.var_cnst0over_func_dn7, locals.var_cnst0over_func_dn8, locals.var_cnst0over_func_dn9, locals.var_cnst0over_func_dn10, locals.var_cnst0over_func_dn11, locals.var_cnst0over_func_dn14,)
    }
};
        locals.var_cnst0over_func = assign74920_e114402;
        locals.var_cnst0over_func_dn0 = assign74920_e114402_d_n0;
        locals.var_cnst0over_func_dn2 = assign74920_e114402_d_n2;
        locals.var_cnst0over_func_dn4 = assign74920_e114402_d_n4;
        locals.var_cnst0over_func_dn5 = assign74920_e114402_d_n5;
        locals.var_cnst0over_func_dn6 = assign74920_e114402_d_n6;
        locals.var_cnst0over_func_dn7 = assign74920_e114402_d_n7;
        locals.var_cnst0over_func_dn8 = assign74920_e114402_d_n8;
        locals.var_cnst0over_func_dn9 = assign74920_e114402_d_n9;
        locals.var_cnst0over_func_dn10 = assign74920_e114402_d_n10;
        locals.var_cnst0over_func_dn11 = assign74920_e114402_d_n11;
        locals.var_cnst0over_func_dn14 = assign74920_e114402_d_n14;
        locals.var_cnst0over_func_rv = 0.0;

        let (assign74930_e114408,) = {
    if ((locals.var_guard1750 != 0.0) && (locals.var_guard1754 != 0.0)) {
        (locals.var_cox0,)
    } else {
        (locals.var_cox0_func,)
    }
};
        locals.var_cox0_func = assign74930_e114408;
        locals.var_cox0_func_rv = 0.0;

        let assign74940_e114427: f64 = if (((((p.p36 == 1.0) && (p.p66 > 0.0)) && (locals.var_uc_novers > 0.0)) && (locals.var_uc_cvdsover != 0.0)) && (p.p55 != 1.0)) { 1.0 } else { 0.0 };
        locals.var_guard1755 = assign74940_e114427;
        locals.var_guard1755_rv = 0.0;

        let (assign74950_e114436,) = {
    if (((locals.var_guard1751 != 0.0) && (locals.var_guard1750 == 0.0)) && (locals.var_guard1755 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_flg_calcqover,)
    }
};
        locals.var_flg_calcqover = assign74950_e114436;
        locals.var_flg_calcqover_rv = 0.0;

        let (assign74960_e114447, assign74960_e114447_d_n2, assign74960_e114447_d_n7, assign74960_e114447_d_n8, assign74960_e114447_d_n9,) = {
    if (((locals.var_guard1751 != 0.0) && (locals.var_guard1750 == 0.0)) && (locals.var_guard1755 != 0.0)) {
        let assign74960_e114445: f64 = (locals.var_vgsei - locals.var_vbsei);
        (assign74960_e114445, (locals.var_vgsei_dn2 - locals.var_vbsei_dn2), locals.var_vgsei_dn7, 0.0, (-locals.var_vbsei_dn9),)
    } else {
        (locals.var_vgbgmt, locals.var_vgbgmt_dn2, locals.var_vgbgmt_dn7, locals.var_vgbgmt_dn8, locals.var_vgbgmt_dn9,)
    }
};
        locals.var_vgbgmt = assign74960_e114447;
        locals.var_vgbgmt_dn2 = assign74960_e114447_d_n2;
        locals.var_vgbgmt_dn7 = assign74960_e114447_d_n7;
        locals.var_vgbgmt_dn8 = assign74960_e114447_d_n8;
        locals.var_vgbgmt_dn9 = assign74960_e114447_d_n9;
        locals.var_vgbgmt_rv = 0.0;

        let (assign74970_e114457, assign74970_e114457_d_n0, assign74970_e114457_d_n2, assign74970_e114457_d_n4, assign74970_e114457_d_n5, assign74970_e114457_d_n6, assign74970_e114457_d_n7, assign74970_e114457_d_n8, assign74970_e114457_d_n9, assign74970_e114457_d_n10, assign74970_e114457_d_n11, assign74970_e114457_d_n14,) = {
    if (((locals.var_guard1751 != 0.0) && (locals.var_guard1750 == 0.0)) && (locals.var_guard1755 != 0.0)) {
        let assign74970_e114455: f64 = (-locals.var_vbsei);
        (assign74970_e114455, 0.0, (-locals.var_vbsei_dn2), 0.0, 0.0, 0.0, 0.0, 0.0, (-locals.var_vbsei_dn9), 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vxbgmt, locals.var_vxbgmt_dn0, locals.var_vxbgmt_dn2, locals.var_vxbgmt_dn4, locals.var_vxbgmt_dn5, locals.var_vxbgmt_dn6, locals.var_vxbgmt_dn7, locals.var_vxbgmt_dn8, locals.var_vxbgmt_dn9, locals.var_vxbgmt_dn10, locals.var_vxbgmt_dn11, locals.var_vxbgmt_dn14,)
    }
};
        locals.var_vxbgmt = assign74970_e114457;
        locals.var_vxbgmt_dn0 = assign74970_e114457_d_n0;
        locals.var_vxbgmt_dn2 = assign74970_e114457_d_n2;
        locals.var_vxbgmt_dn4 = assign74970_e114457_d_n4;
        locals.var_vxbgmt_dn5 = assign74970_e114457_d_n5;
        locals.var_vxbgmt_dn6 = assign74970_e114457_d_n6;
        locals.var_vxbgmt_dn7 = assign74970_e114457_d_n7;
        locals.var_vxbgmt_dn8 = assign74970_e114457_d_n8;
        locals.var_vxbgmt_dn9 = assign74970_e114457_d_n9;
        locals.var_vxbgmt_dn10 = assign74970_e114457_d_n10;
        locals.var_vxbgmt_dn11 = assign74970_e114457_d_n11;
        locals.var_vxbgmt_dn14 = assign74970_e114457_d_n14;
        locals.var_vxbgmt_rv = 0.0;

        let assign74980_e114468: f64 = if (((p.p35 == 1.0) && (p.p63 > 0.0)) && (locals.var_uc_nover > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1756 = assign74980_e114468;
        locals.var_guard1756_rv = 0.0;

        let (assign74990_e114479,) = {
    if (((locals.var_guard1752 != 0.0) && (!((locals.var_guard1750 != 0.0) || (locals.var_guard1751 != 0.0)))) && (locals.var_guard1756 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_flg_calcqover,)
    }
};
        locals.var_flg_calcqover = assign74990_e114479;
        locals.var_flg_calcqover_rv = 0.0;

        let (assign75000_e114490,) = {
    if (((locals.var_guard1752 != 0.0) && (!((locals.var_guard1750 != 0.0) || (locals.var_guard1751 != 0.0)))) && (locals.var_guard1756 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_flg_coovlp,)
    }
};
        locals.var_flg_coovlp = assign75000_e114490;
        locals.var_flg_coovlp_rv = 0.0;

        let (assign75010_e114503, assign75010_e114503_d_n2, assign75010_e114503_d_n7, assign75010_e114503_d_n8, assign75010_e114503_d_n9,) = {
    if (((locals.var_guard1752 != 0.0) && (!((locals.var_guard1750 != 0.0) || (locals.var_guard1751 != 0.0)))) && (locals.var_guard1756 != 0.0)) {
        let assign75010_e114501: f64 = (locals.var_vgsi - locals.var_vbsi);
        (assign75010_e114501, 0.0, locals.var_vgsi_dn7, (locals.var_vgsi_dn8 - locals.var_vbsi_dn8), (-locals.var_vbsi_dn9),)
    } else {
        (locals.var_vgbgmt, locals.var_vgbgmt_dn2, locals.var_vgbgmt_dn7, locals.var_vgbgmt_dn8, locals.var_vgbgmt_dn9,)
    }
};
        locals.var_vgbgmt = assign75010_e114503;
        locals.var_vgbgmt_dn2 = assign75010_e114503_d_n2;
        locals.var_vgbgmt_dn7 = assign75010_e114503_d_n7;
        locals.var_vgbgmt_dn8 = assign75010_e114503_d_n8;
        locals.var_vgbgmt_dn9 = assign75010_e114503_d_n9;
        locals.var_vgbgmt_rv = 0.0;

        let (assign75020_e114516, assign75020_e114516_d_n0, assign75020_e114516_d_n2, assign75020_e114516_d_n4, assign75020_e114516_d_n5, assign75020_e114516_d_n6, assign75020_e114516_d_n7, assign75020_e114516_d_n8, assign75020_e114516_d_n9, assign75020_e114516_d_n10, assign75020_e114516_d_n11, assign75020_e114516_d_n14,) = {
    if (((locals.var_guard1752 != 0.0) && (!((locals.var_guard1750 != 0.0) || (locals.var_guard1751 != 0.0)))) && (locals.var_guard1756 != 0.0)) {
        let assign75020_e114514: f64 = (locals.var_vdsi - locals.var_vbsi);
        (assign75020_e114514, 0.0, 0.0, 0.0, 0.0, locals.var_vdsi_dn6, 0.0, (locals.var_vdsi_dn8 - locals.var_vbsi_dn8), (-locals.var_vbsi_dn9), 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vxbgmt, locals.var_vxbgmt_dn0, locals.var_vxbgmt_dn2, locals.var_vxbgmt_dn4, locals.var_vxbgmt_dn5, locals.var_vxbgmt_dn6, locals.var_vxbgmt_dn7, locals.var_vxbgmt_dn8, locals.var_vxbgmt_dn9, locals.var_vxbgmt_dn10, locals.var_vxbgmt_dn11, locals.var_vxbgmt_dn14,)
    }
};
        locals.var_vxbgmt = assign75020_e114516;
        locals.var_vxbgmt_dn0 = assign75020_e114516_d_n0;
        locals.var_vxbgmt_dn2 = assign75020_e114516_d_n2;
        locals.var_vxbgmt_dn4 = assign75020_e114516_d_n4;
        locals.var_vxbgmt_dn5 = assign75020_e114516_d_n5;
        locals.var_vxbgmt_dn6 = assign75020_e114516_d_n6;
        locals.var_vxbgmt_dn7 = assign75020_e114516_d_n7;
        locals.var_vxbgmt_dn8 = assign75020_e114516_d_n8;
        locals.var_vxbgmt_dn9 = assign75020_e114516_d_n9;
        locals.var_vxbgmt_dn10 = assign75020_e114516_d_n10;
        locals.var_vxbgmt_dn11 = assign75020_e114516_d_n11;
        locals.var_vxbgmt_dn14 = assign75020_e114516_d_n14;
        locals.var_vxbgmt_rv = 0.0;

        let (assign75030_e114527,) = {
    if (((locals.var_guard1752 != 0.0) && (!((locals.var_guard1750 != 0.0) || (locals.var_guard1751 != 0.0)))) && (locals.var_guard1756 != 0.0)) {
        (locals.var_uc_nover,)
    } else {
        (locals.var_nover_func,)
    }
};
        locals.var_nover_func = assign75030_e114527;
        locals.var_nover_func_rv = 0.0;

        let (assign75040_e114542, assign75040_e114542_d_n0, assign75040_e114542_d_n2, assign75040_e114542_d_n4, assign75040_e114542_d_n5, assign75040_e114542_d_n6, assign75040_e114542_d_n7, assign75040_e114542_d_n8, assign75040_e114542_d_n9, assign75040_e114542_d_n10, assign75040_e114542_d_n11, assign75040_e114542_d_n14,) = {
    if (((locals.var_guard1752 != 0.0) && (!((locals.var_guard1750 != 0.0) || (locals.var_guard1751 != 0.0)))) && (locals.var_guard1756 != 0.0)) {
        let assign75040_e114539: f64 = (p.p64 * p.p55);
        let assign75040_e114540: f64 = (p.p63 + assign75040_e114539);
        (assign75040_e114540, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_lover_func, locals.var_lover_func_dn0, locals.var_lover_func_dn2, locals.var_lover_func_dn4, locals.var_lover_func_dn5, locals.var_lover_func_dn6, locals.var_lover_func_dn7, locals.var_lover_func_dn8, locals.var_lover_func_dn9, locals.var_lover_func_dn10, locals.var_lover_func_dn11, locals.var_lover_func_dn14,)
    }
};
        locals.var_lover_func = assign75040_e114542;
        locals.var_lover_func_dn0 = assign75040_e114542_d_n0;
        locals.var_lover_func_dn2 = assign75040_e114542_d_n2;
        locals.var_lover_func_dn4 = assign75040_e114542_d_n4;
        locals.var_lover_func_dn5 = assign75040_e114542_d_n5;
        locals.var_lover_func_dn6 = assign75040_e114542_d_n6;
        locals.var_lover_func_dn7 = assign75040_e114542_d_n7;
        locals.var_lover_func_dn8 = assign75040_e114542_d_n8;
        locals.var_lover_func_dn9 = assign75040_e114542_d_n9;
        locals.var_lover_func_dn10 = assign75040_e114542_d_n10;
        locals.var_lover_func_dn11 = assign75040_e114542_d_n11;
        locals.var_lover_func_dn14 = assign75040_e114542_d_n14;
        locals.var_lover_func_rv = 0.0;

        let (assign75050_e114553, assign75050_e114553_d_n0, assign75050_e114553_d_n2, assign75050_e114553_d_n4, assign75050_e114553_d_n5, assign75050_e114553_d_n6, assign75050_e114553_d_n7, assign75050_e114553_d_n8, assign75050_e114553_d_n9, assign75050_e114553_d_n10, assign75050_e114553_d_n11, assign75050_e114553_d_n14,) = {
    if (((locals.var_guard1752 != 0.0) && (!((locals.var_guard1750 != 0.0) || (locals.var_guard1751 != 0.0)))) && (locals.var_guard1756 != 0.0)) {
        (locals.var_wdep, locals.var_wdep_dn0, locals.var_wdep_dn2, locals.var_wdep_dn4, locals.var_wdep_dn5, locals.var_wdep_dn6, locals.var_wdep_dn7, locals.var_wdep_dn8, locals.var_wdep_dn9, locals.var_wdep_dn10, locals.var_wdep_dn11, locals.var_wdep_dn14,)
    } else {
        (locals.var_wdep_func, locals.var_wdep_func_dn0, locals.var_wdep_func_dn2, locals.var_wdep_func_dn4, locals.var_wdep_func_dn5, locals.var_wdep_func_dn6, locals.var_wdep_func_dn7, locals.var_wdep_func_dn8, locals.var_wdep_func_dn9, locals.var_wdep_func_dn10, locals.var_wdep_func_dn11, locals.var_wdep_func_dn14,)
    }
};
        locals.var_wdep_func = assign75050_e114553;
        locals.var_wdep_func_dn0 = assign75050_e114553_d_n0;
        locals.var_wdep_func_dn2 = assign75050_e114553_d_n2;
        locals.var_wdep_func_dn4 = assign75050_e114553_d_n4;
        locals.var_wdep_func_dn5 = assign75050_e114553_d_n5;
        locals.var_wdep_func_dn6 = assign75050_e114553_d_n6;
        locals.var_wdep_func_dn7 = assign75050_e114553_d_n7;
        locals.var_wdep_func_dn8 = assign75050_e114553_d_n8;
        locals.var_wdep_func_dn9 = assign75050_e114553_d_n9;
        locals.var_wdep_func_dn10 = assign75050_e114553_d_n10;
        locals.var_wdep_func_dn11 = assign75050_e114553_d_n11;
        locals.var_wdep_func_dn14 = assign75050_e114553_d_n14;
        locals.var_wdep_func_rv = 0.0;

        let (assign75060_e114564, assign75060_e114564_d_n0, assign75060_e114564_d_n2, assign75060_e114564_d_n4, assign75060_e114564_d_n5, assign75060_e114564_d_n6, assign75060_e114564_d_n7, assign75060_e114564_d_n8, assign75060_e114564_d_n9, assign75060_e114564_d_n10, assign75060_e114564_d_n11, assign75060_e114564_d_n14,) = {
    if (((locals.var_guard1752 != 0.0) && (!((locals.var_guard1750 != 0.0) || (locals.var_guard1751 != 0.0)))) && (locals.var_guard1756 != 0.0)) {
        (locals.var_cnst0over, locals.var_cnst0over_dn0, locals.var_cnst0over_dn2, locals.var_cnst0over_dn4, locals.var_cnst0over_dn5, locals.var_cnst0over_dn6, locals.var_cnst0over_dn7, locals.var_cnst0over_dn8, locals.var_cnst0over_dn9, locals.var_cnst0over_dn10, locals.var_cnst0over_dn11, locals.var_cnst0over_dn14,)
    } else {
        (locals.var_cnst0over_func, locals.var_cnst0over_func_dn0, locals.var_cnst0over_func_dn2, locals.var_cnst0over_func_dn4, locals.var_cnst0over_func_dn5, locals.var_cnst0over_func_dn6, locals.var_cnst0over_func_dn7, locals.var_cnst0over_func_dn8, locals.var_cnst0over_func_dn9, locals.var_cnst0over_func_dn10, locals.var_cnst0over_func_dn11, locals.var_cnst0over_func_dn14,)
    }
};
        locals.var_cnst0over_func = assign75060_e114564;
        locals.var_cnst0over_func_dn0 = assign75060_e114564_d_n0;
        locals.var_cnst0over_func_dn2 = assign75060_e114564_d_n2;
        locals.var_cnst0over_func_dn4 = assign75060_e114564_d_n4;
        locals.var_cnst0over_func_dn5 = assign75060_e114564_d_n5;
        locals.var_cnst0over_func_dn6 = assign75060_e114564_d_n6;
        locals.var_cnst0over_func_dn7 = assign75060_e114564_d_n7;
        locals.var_cnst0over_func_dn8 = assign75060_e114564_d_n8;
        locals.var_cnst0over_func_dn9 = assign75060_e114564_d_n9;
        locals.var_cnst0over_func_dn10 = assign75060_e114564_d_n10;
        locals.var_cnst0over_func_dn11 = assign75060_e114564_d_n11;
        locals.var_cnst0over_func_dn14 = assign75060_e114564_d_n14;
        locals.var_cnst0over_func_rv = 0.0;

        let (assign75070_e114575,) = {
    if (((locals.var_guard1752 != 0.0) && (!((locals.var_guard1750 != 0.0) || (locals.var_guard1751 != 0.0)))) && (locals.var_guard1756 != 0.0)) {
        (locals.var_coxb0,)
    } else {
        (locals.var_cox0_func,)
    }
};
        locals.var_cox0_func = assign75070_e114575;
        locals.var_cox0_func_rv = 0.0;

        let (assign75080_e114587, assign75080_e114587_d_n0, assign75080_e114587_d_n2, assign75080_e114587_d_n4, assign75080_e114587_d_n5, assign75080_e114587_d_n6, assign75080_e114587_d_n7, assign75080_e114587_d_n8, assign75080_e114587_d_n9, assign75080_e114587_d_n10, assign75080_e114587_d_n11, assign75080_e114587_d_n14,) = {
    if (((locals.var_guard1752 != 0.0) && (!((locals.var_guard1750 != 0.0) || (locals.var_guard1751 != 0.0)))) && (locals.var_guard1756 != 0.0)) {
        let assign75080_e114585: f64 = (-locals.var_lover_func);
        (assign75080_e114585, (-locals.var_lover_func_dn0), (-locals.var_lover_func_dn2), (-locals.var_lover_func_dn4), (-locals.var_lover_func_dn5), (-locals.var_lover_func_dn6), (-locals.var_lover_func_dn7), (-locals.var_lover_func_dn8), (-locals.var_lover_func_dn9), (-locals.var_lover_func_dn10), (-locals.var_lover_func_dn11), (-locals.var_lover_func_dn14),)
    } else {
        (locals.var_lover_func, locals.var_lover_func_dn0, locals.var_lover_func_dn2, locals.var_lover_func_dn4, locals.var_lover_func_dn5, locals.var_lover_func_dn6, locals.var_lover_func_dn7, locals.var_lover_func_dn8, locals.var_lover_func_dn9, locals.var_lover_func_dn10, locals.var_lover_func_dn11, locals.var_lover_func_dn14,)
    }
};
        locals.var_lover_func = assign75080_e114587;
        locals.var_lover_func_dn0 = assign75080_e114587_d_n0;
        locals.var_lover_func_dn2 = assign75080_e114587_d_n2;
        locals.var_lover_func_dn4 = assign75080_e114587_d_n4;
        locals.var_lover_func_dn5 = assign75080_e114587_d_n5;
        locals.var_lover_func_dn6 = assign75080_e114587_d_n6;
        locals.var_lover_func_dn7 = assign75080_e114587_d_n7;
        locals.var_lover_func_dn8 = assign75080_e114587_d_n8;
        locals.var_lover_func_dn9 = assign75080_e114587_d_n9;
        locals.var_lover_func_dn10 = assign75080_e114587_d_n10;
        locals.var_lover_func_dn11 = assign75080_e114587_d_n11;
        locals.var_lover_func_dn14 = assign75080_e114587_d_n14;
        locals.var_lover_func_rv = 0.0;

        let assign75090_e114598: f64 = if (((locals.var_lover_func < 0.0) && (p.p432 > 0.0)) && (p.p55 == 1.0)) { 1.0 } else { 0.0 };
        locals.var_guard1757 = assign75090_e114598;
        locals.var_guard1757_rv = 0.0;

        let (assign75100_e114612, assign75100_e114612_d_n0, assign75100_e114612_d_n2, assign75100_e114612_d_n4, assign75100_e114612_d_n5, assign75100_e114612_d_n6, assign75100_e114612_d_n7, assign75100_e114612_d_n8, assign75100_e114612_d_n9, assign75100_e114612_d_n10, assign75100_e114612_d_n11, assign75100_e114612_d_n14,) = {
    if ((((locals.var_guard1752 != 0.0) && (!((locals.var_guard1750 != 0.0) || (locals.var_guard1751 != 0.0)))) && (locals.var_guard1756 != 0.0)) && (locals.var_guard1757 != 0.0)) {
        let assign75100_e114610: f64 = (-locals.var_lover_func);
        (assign75100_e114610, (-locals.var_lover_func_dn0), (-locals.var_lover_func_dn2), (-locals.var_lover_func_dn4), (-locals.var_lover_func_dn5), (-locals.var_lover_func_dn6), (-locals.var_lover_func_dn7), (-locals.var_lover_func_dn8), (-locals.var_lover_func_dn9), (-locals.var_lover_func_dn10), (-locals.var_lover_func_dn11), (-locals.var_lover_func_dn14),)
    } else {
        (locals.var_lover_func, locals.var_lover_func_dn0, locals.var_lover_func_dn2, locals.var_lover_func_dn4, locals.var_lover_func_dn5, locals.var_lover_func_dn6, locals.var_lover_func_dn7, locals.var_lover_func_dn8, locals.var_lover_func_dn9, locals.var_lover_func_dn10, locals.var_lover_func_dn11, locals.var_lover_func_dn14,)
    }
};
        locals.var_lover_func = assign75100_e114612;
        locals.var_lover_func_dn0 = assign75100_e114612_d_n0;
        locals.var_lover_func_dn2 = assign75100_e114612_d_n2;
        locals.var_lover_func_dn4 = assign75100_e114612_d_n4;
        locals.var_lover_func_dn5 = assign75100_e114612_d_n5;
        locals.var_lover_func_dn6 = assign75100_e114612_d_n6;
        locals.var_lover_func_dn7 = assign75100_e114612_d_n7;
        locals.var_lover_func_dn8 = assign75100_e114612_d_n8;
        locals.var_lover_func_dn9 = assign75100_e114612_d_n9;
        locals.var_lover_func_dn10 = assign75100_e114612_d_n10;
        locals.var_lover_func_dn11 = assign75100_e114612_d_n11;
        locals.var_lover_func_dn14 = assign75100_e114612_d_n14;
        locals.var_lover_func_rv = 0.0;

        let (assign75110_e114625, assign75110_e114625_d_n0, assign75110_e114625_d_n2, assign75110_e114625_d_n4, assign75110_e114625_d_n5, assign75110_e114625_d_n6, assign75110_e114625_d_n7, assign75110_e114625_d_n8, assign75110_e114625_d_n9, assign75110_e114625_d_n10, assign75110_e114625_d_n11, assign75110_e114625_d_n14,) = {
    if ((((locals.var_guard1752 != 0.0) && (!((locals.var_guard1750 != 0.0) || (locals.var_guard1751 != 0.0)))) && (locals.var_guard1756 != 0.0)) && (locals.var_guard1757 != 0.0)) {
        (p.p63, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign75110_e114625;
        locals.var_t1_dn0 = assign75110_e114625_d_n0;
        locals.var_t1_dn2 = assign75110_e114625_d_n2;
        locals.var_t1_dn4 = assign75110_e114625_d_n4;
        locals.var_t1_dn5 = assign75110_e114625_d_n5;
        locals.var_t1_dn6 = assign75110_e114625_d_n6;
        locals.var_t1_dn7 = assign75110_e114625_d_n7;
        locals.var_t1_dn8 = assign75110_e114625_d_n8;
        locals.var_t1_dn9 = assign75110_e114625_d_n9;
        locals.var_t1_dn10 = assign75110_e114625_d_n10;
        locals.var_t1_dn11 = assign75110_e114625_d_n11;
        locals.var_t1_dn14 = assign75110_e114625_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign75120_e114644, assign75120_e114644_d_n0, assign75120_e114644_d_n2, assign75120_e114644_d_n4, assign75120_e114644_d_n5, assign75120_e114644_d_n6, assign75120_e114644_d_n7, assign75120_e114644_d_n8, assign75120_e114644_d_n9, assign75120_e114644_d_n10, assign75120_e114644_d_n11, assign75120_e114644_d_n14,) = {
    if ((((locals.var_guard1752 != 0.0) && (!((locals.var_guard1750 != 0.0) || (locals.var_guard1751 != 0.0)))) && (locals.var_guard1756 != 0.0)) && (locals.var_guard1757 != 0.0)) {
        let assign75120_e114638: f64 = (locals.var_t1 * locals.var_t1);
        let assign75120_e114640: f64 = (assign75120_e114638 / locals.var_kjunc);
        let assign75120_e114642: f64 = (assign75120_e114640 - p.p137);
        (assign75120_e114642, (((((locals.var_t1_dn0 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn0)) * locals.var_kjunc) - (assign75120_e114638 * locals.var_kjunc_dn0)) / (locals.var_kjunc * locals.var_kjunc)), (((((locals.var_t1_dn2 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn2)) * locals.var_kjunc) - (assign75120_e114638 * locals.var_kjunc_dn2)) / (locals.var_kjunc * locals.var_kjunc)), (((((locals.var_t1_dn4 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn4)) * locals.var_kjunc) - (assign75120_e114638 * locals.var_kjunc_dn4)) / (locals.var_kjunc * locals.var_kjunc)), (((((locals.var_t1_dn5 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn5)) * locals.var_kjunc) - (assign75120_e114638 * locals.var_kjunc_dn5)) / (locals.var_kjunc * locals.var_kjunc)), (((((locals.var_t1_dn6 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn6)) * locals.var_kjunc) - (assign75120_e114638 * locals.var_kjunc_dn6)) / (locals.var_kjunc * locals.var_kjunc)), (((((locals.var_t1_dn7 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn7)) * locals.var_kjunc) - (assign75120_e114638 * locals.var_kjunc_dn7)) / (locals.var_kjunc * locals.var_kjunc)), (((((locals.var_t1_dn8 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn8)) * locals.var_kjunc) - (assign75120_e114638 * locals.var_kjunc_dn8)) / (locals.var_kjunc * locals.var_kjunc)), (((((locals.var_t1_dn9 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn9)) * locals.var_kjunc) - (assign75120_e114638 * locals.var_kjunc_dn9)) / (locals.var_kjunc * locals.var_kjunc)), (((((locals.var_t1_dn10 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn10)) * locals.var_kjunc) - (assign75120_e114638 * locals.var_kjunc_dn10)) / (locals.var_kjunc * locals.var_kjunc)), (((((locals.var_t1_dn11 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn11)) * locals.var_kjunc) - (assign75120_e114638 * locals.var_kjunc_dn11)) / (locals.var_kjunc * locals.var_kjunc)), (((((locals.var_t1_dn14 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn14)) * locals.var_kjunc) - (assign75120_e114638 * locals.var_kjunc_dn14)) / (locals.var_kjunc * locals.var_kjunc)),)
    } else {
        (locals.var_vxb_lim, locals.var_vxb_lim_dn0, locals.var_vxb_lim_dn2, locals.var_vxb_lim_dn4, locals.var_vxb_lim_dn5, locals.var_vxb_lim_dn6, locals.var_vxb_lim_dn7, locals.var_vxb_lim_dn8, locals.var_vxb_lim_dn9, locals.var_vxb_lim_dn10, locals.var_vxb_lim_dn11, locals.var_vxb_lim_dn14,)
    }
};
        locals.var_vxb_lim = assign75120_e114644;
        locals.var_vxb_lim_dn0 = assign75120_e114644_d_n0;
        locals.var_vxb_lim_dn2 = assign75120_e114644_d_n2;
        locals.var_vxb_lim_dn4 = assign75120_e114644_d_n4;
        locals.var_vxb_lim_dn5 = assign75120_e114644_d_n5;
        locals.var_vxb_lim_dn6 = assign75120_e114644_d_n6;
        locals.var_vxb_lim_dn7 = assign75120_e114644_d_n7;
        locals.var_vxb_lim_dn8 = assign75120_e114644_d_n8;
        locals.var_vxb_lim_dn9 = assign75120_e114644_d_n9;
        locals.var_vxb_lim_dn10 = assign75120_e114644_d_n10;
        locals.var_vxb_lim_dn11 = assign75120_e114644_d_n11;
        locals.var_vxb_lim_dn14 = assign75120_e114644_d_n14;
        locals.var_vxb_lim_rv = 0.0;

        let assign75130_e114647: f64 = if p.p113 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1758 = assign75130_e114647;
        locals.var_guard1758_rv = 0.0;

        let assign75140_e114654: f64 = if ((locals.var_vxbgmt == 0.0) || (p.p113 <= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1759 = assign75140_e114654;
        locals.var_guard1759_rv = 0.0;

        let (assign75150_e114671, assign75150_e114671_d_n0, assign75150_e114671_d_n2, assign75150_e114671_d_n4, assign75150_e114671_d_n5, assign75150_e114671_d_n6, assign75150_e114671_d_n7, assign75150_e114671_d_n8, assign75150_e114671_d_n9, assign75150_e114671_d_n10, assign75150_e114671_d_n11, assign75150_e114671_d_n14,) = {
    if ((((((locals.var_guard1752 != 0.0) && (!((locals.var_guard1750 != 0.0) || (locals.var_guard1751 != 0.0)))) && (locals.var_guard1756 != 0.0)) && (locals.var_guard1757 != 0.0)) && (locals.var_guard1758 != 0.0)) && (locals.var_guard1759 != 0.0)) {
        (locals.var_vxbgmt, locals.var_vxbgmt_dn0, locals.var_vxbgmt_dn2, locals.var_vxbgmt_dn4, locals.var_vxbgmt_dn5, locals.var_vxbgmt_dn6, locals.var_vxbgmt_dn7, locals.var_vxbgmt_dn8, locals.var_vxbgmt_dn9, locals.var_vxbgmt_dn10, locals.var_vxbgmt_dn11, locals.var_vxbgmt_dn14,)
    } else {
        (locals.var_vxbgmt, locals.var_vxbgmt_dn0, locals.var_vxbgmt_dn2, locals.var_vxbgmt_dn4, locals.var_vxbgmt_dn5, locals.var_vxbgmt_dn6, locals.var_vxbgmt_dn7, locals.var_vxbgmt_dn8, locals.var_vxbgmt_dn9, locals.var_vxbgmt_dn10, locals.var_vxbgmt_dn11, locals.var_vxbgmt_dn14,)
    }
};
        locals.var_vxbgmt = assign75150_e114671;
        locals.var_vxbgmt_dn0 = assign75150_e114671_d_n0;
        locals.var_vxbgmt_dn2 = assign75150_e114671_d_n2;
        locals.var_vxbgmt_dn4 = assign75150_e114671_d_n4;
        locals.var_vxbgmt_dn5 = assign75150_e114671_d_n5;
        locals.var_vxbgmt_dn6 = assign75150_e114671_d_n6;
        locals.var_vxbgmt_dn7 = assign75150_e114671_d_n7;
        locals.var_vxbgmt_dn8 = assign75150_e114671_d_n8;
        locals.var_vxbgmt_dn9 = assign75150_e114671_d_n9;
        locals.var_vxbgmt_dn10 = assign75150_e114671_d_n10;
        locals.var_vxbgmt_dn11 = assign75150_e114671_d_n11;
        locals.var_vxbgmt_dn14 = assign75150_e114671_d_n14;
        locals.var_vxbgmt_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_284(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign75160_e114695, assign75160_e114695_d_n0, assign75160_e114695_d_n2, assign75160_e114695_d_n4, assign75160_e114695_d_n5, assign75160_e114695_d_n6, assign75160_e114695_d_n7, assign75160_e114695_d_n8, assign75160_e114695_d_n9, assign75160_e114695_d_n10, assign75160_e114695_d_n11, assign75160_e114695_d_n14,) = {
    if ((((((locals.var_guard1752 != 0.0) && (!((locals.var_guard1750 != 0.0) || (locals.var_guard1751 != 0.0)))) && (locals.var_guard1756 != 0.0)) && (locals.var_guard1757 != 0.0)) && (locals.var_guard1758 != 0.0)) && (locals.var_guard1759 == 0.0)) {
        let (assign75160_e114693,) = {
            if (locals.var_vxbgmt < 0.0) {
                let assign75160_e114691: f64 = (-1.0);
                (assign75160_e114691,)
            } else {
                (1.0,)
            }
        };
        (assign75160_e114693, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf3, locals.var_tmf3_dn0, locals.var_tmf3_dn2, locals.var_tmf3_dn4, locals.var_tmf3_dn5, locals.var_tmf3_dn6, locals.var_tmf3_dn7, locals.var_tmf3_dn8, locals.var_tmf3_dn9, locals.var_tmf3_dn10, locals.var_tmf3_dn11, locals.var_tmf3_dn14,)
    }
};
        locals.var_tmf3 = assign75160_e114695;
        locals.var_tmf3_dn0 = assign75160_e114695_d_n0;
        locals.var_tmf3_dn2 = assign75160_e114695_d_n2;
        locals.var_tmf3_dn4 = assign75160_e114695_d_n4;
        locals.var_tmf3_dn5 = assign75160_e114695_d_n5;
        locals.var_tmf3_dn6 = assign75160_e114695_d_n6;
        locals.var_tmf3_dn7 = assign75160_e114695_d_n7;
        locals.var_tmf3_dn8 = assign75160_e114695_d_n8;
        locals.var_tmf3_dn9 = assign75160_e114695_d_n9;
        locals.var_tmf3_dn10 = assign75160_e114695_d_n10;
        locals.var_tmf3_dn11 = assign75160_e114695_d_n11;
        locals.var_tmf3_dn14 = assign75160_e114695_d_n14;
        locals.var_tmf3_rv = 0.0;

        let (assign75170_e114715, assign75170_e114715_d_n0, assign75170_e114715_d_n2, assign75170_e114715_d_n4, assign75170_e114715_d_n5, assign75170_e114715_d_n6, assign75170_e114715_d_n7, assign75170_e114715_d_n8, assign75170_e114715_d_n9, assign75170_e114715_d_n10, assign75170_e114715_d_n11, assign75170_e114715_d_n14,) = {
    if ((((((locals.var_guard1752 != 0.0) && (!((locals.var_guard1750 != 0.0) || (locals.var_guard1751 != 0.0)))) && (locals.var_guard1756 != 0.0)) && (locals.var_guard1757 != 0.0)) && (locals.var_guard1758 != 0.0)) && (locals.var_guard1759 == 0.0)) {
        let assign75170_e114713: f64 = (locals.var_tmf3 * locals.var_vxbgmt);
        (assign75170_e114713, ((locals.var_tmf3_dn0 * locals.var_vxbgmt) + (locals.var_tmf3 * locals.var_vxbgmt_dn0)), ((locals.var_tmf3_dn2 * locals.var_vxbgmt) + (locals.var_tmf3 * locals.var_vxbgmt_dn2)), ((locals.var_tmf3_dn4 * locals.var_vxbgmt) + (locals.var_tmf3 * locals.var_vxbgmt_dn4)), ((locals.var_tmf3_dn5 * locals.var_vxbgmt) + (locals.var_tmf3 * locals.var_vxbgmt_dn5)), ((locals.var_tmf3_dn6 * locals.var_vxbgmt) + (locals.var_tmf3 * locals.var_vxbgmt_dn6)), ((locals.var_tmf3_dn7 * locals.var_vxbgmt) + (locals.var_tmf3 * locals.var_vxbgmt_dn7)), ((locals.var_tmf3_dn8 * locals.var_vxbgmt) + (locals.var_tmf3 * locals.var_vxbgmt_dn8)), ((locals.var_tmf3_dn9 * locals.var_vxbgmt) + (locals.var_tmf3 * locals.var_vxbgmt_dn9)), ((locals.var_tmf3_dn10 * locals.var_vxbgmt) + (locals.var_tmf3 * locals.var_vxbgmt_dn10)), ((locals.var_tmf3_dn11 * locals.var_vxbgmt) + (locals.var_tmf3 * locals.var_vxbgmt_dn11)), ((locals.var_tmf3_dn14 * locals.var_vxbgmt) + (locals.var_tmf3 * locals.var_vxbgmt_dn14)),)
    } else {
        (locals.var_tmf4, locals.var_tmf4_dn0, locals.var_tmf4_dn2, locals.var_tmf4_dn4, locals.var_tmf4_dn5, locals.var_tmf4_dn6, locals.var_tmf4_dn7, locals.var_tmf4_dn8, locals.var_tmf4_dn9, locals.var_tmf4_dn10, locals.var_tmf4_dn11, locals.var_tmf4_dn14,)
    }
};
        locals.var_tmf4 = assign75170_e114715;
        locals.var_tmf4_dn0 = assign75170_e114715_d_n0;
        locals.var_tmf4_dn2 = assign75170_e114715_d_n2;
        locals.var_tmf4_dn4 = assign75170_e114715_d_n4;
        locals.var_tmf4_dn5 = assign75170_e114715_d_n5;
        locals.var_tmf4_dn6 = assign75170_e114715_d_n6;
        locals.var_tmf4_dn7 = assign75170_e114715_d_n7;
        locals.var_tmf4_dn8 = assign75170_e114715_d_n8;
        locals.var_tmf4_dn9 = assign75170_e114715_d_n9;
        locals.var_tmf4_dn10 = assign75170_e114715_d_n10;
        locals.var_tmf4_dn11 = assign75170_e114715_d_n11;
        locals.var_tmf4_dn14 = assign75170_e114715_d_n14;
        locals.var_tmf4_rv = 0.0;

        let (assign75180_e114739, assign75180_e114739_d_n0, assign75180_e114739_d_n2, assign75180_e114739_d_n4, assign75180_e114739_d_n5, assign75180_e114739_d_n6, assign75180_e114739_d_n7, assign75180_e114739_d_n8, assign75180_e114739_d_n9, assign75180_e114739_d_n10, assign75180_e114739_d_n11, assign75180_e114739_d_n14,) = {
    if ((((((locals.var_guard1752 != 0.0) && (!((locals.var_guard1750 != 0.0) || (locals.var_guard1751 != 0.0)))) && (locals.var_guard1756 != 0.0)) && (locals.var_guard1757 != 0.0)) && (locals.var_guard1758 != 0.0)) && (locals.var_guard1759 == 0.0)) {
        let assign75180_e114734: f64 = (locals.var_tmf4 / locals.var_vxb_lim);
        let assign75180_e114736: f64 = (assign75180_e114734).powf(p.p113);
        let assign75180_e114737: f64 = (1.0 + assign75180_e114736);
        (assign75180_e114737, if 0.0 == 0.0 && ((p.p113) as f64).is_finite() && ((p.p113) as f64).fract() == 0.0 { if p.p113 == 0.0 { 0.0 } else { (p.p113 * ((assign75180_e114734).powf(p.p113 - 1.0) * (((locals.var_tmf4_dn0 * locals.var_vxb_lim) - (locals.var_tmf4 * locals.var_vxb_lim_dn0)) / (locals.var_vxb_lim * locals.var_vxb_lim)))) } } else { (assign75180_e114736 * (p.p113 * ((((locals.var_tmf4_dn0 * locals.var_vxb_lim) - (locals.var_tmf4 * locals.var_vxb_lim_dn0)) / (locals.var_vxb_lim * locals.var_vxb_lim)) / assign75180_e114734))) }, if 0.0 == 0.0 && ((p.p113) as f64).is_finite() && ((p.p113) as f64).fract() == 0.0 { if p.p113 == 0.0 { 0.0 } else { (p.p113 * ((assign75180_e114734).powf(p.p113 - 1.0) * (((locals.var_tmf4_dn2 * locals.var_vxb_lim) - (locals.var_tmf4 * locals.var_vxb_lim_dn2)) / (locals.var_vxb_lim * locals.var_vxb_lim)))) } } else { (assign75180_e114736 * (p.p113 * ((((locals.var_tmf4_dn2 * locals.var_vxb_lim) - (locals.var_tmf4 * locals.var_vxb_lim_dn2)) / (locals.var_vxb_lim * locals.var_vxb_lim)) / assign75180_e114734))) }, if 0.0 == 0.0 && ((p.p113) as f64).is_finite() && ((p.p113) as f64).fract() == 0.0 { if p.p113 == 0.0 { 0.0 } else { (p.p113 * ((assign75180_e114734).powf(p.p113 - 1.0) * (((locals.var_tmf4_dn4 * locals.var_vxb_lim) - (locals.var_tmf4 * locals.var_vxb_lim_dn4)) / (locals.var_vxb_lim * locals.var_vxb_lim)))) } } else { (assign75180_e114736 * (p.p113 * ((((locals.var_tmf4_dn4 * locals.var_vxb_lim) - (locals.var_tmf4 * locals.var_vxb_lim_dn4)) / (locals.var_vxb_lim * locals.var_vxb_lim)) / assign75180_e114734))) }, if 0.0 == 0.0 && ((p.p113) as f64).is_finite() && ((p.p113) as f64).fract() == 0.0 { if p.p113 == 0.0 { 0.0 } else { (p.p113 * ((assign75180_e114734).powf(p.p113 - 1.0) * (((locals.var_tmf4_dn5 * locals.var_vxb_lim) - (locals.var_tmf4 * locals.var_vxb_lim_dn5)) / (locals.var_vxb_lim * locals.var_vxb_lim)))) } } else { (assign75180_e114736 * (p.p113 * ((((locals.var_tmf4_dn5 * locals.var_vxb_lim) - (locals.var_tmf4 * locals.var_vxb_lim_dn5)) / (locals.var_vxb_lim * locals.var_vxb_lim)) / assign75180_e114734))) }, if 0.0 == 0.0 && ((p.p113) as f64).is_finite() && ((p.p113) as f64).fract() == 0.0 { if p.p113 == 0.0 { 0.0 } else { (p.p113 * ((assign75180_e114734).powf(p.p113 - 1.0) * (((locals.var_tmf4_dn6 * locals.var_vxb_lim) - (locals.var_tmf4 * locals.var_vxb_lim_dn6)) / (locals.var_vxb_lim * locals.var_vxb_lim)))) } } else { (assign75180_e114736 * (p.p113 * ((((locals.var_tmf4_dn6 * locals.var_vxb_lim) - (locals.var_tmf4 * locals.var_vxb_lim_dn6)) / (locals.var_vxb_lim * locals.var_vxb_lim)) / assign75180_e114734))) }, if 0.0 == 0.0 && ((p.p113) as f64).is_finite() && ((p.p113) as f64).fract() == 0.0 { if p.p113 == 0.0 { 0.0 } else { (p.p113 * ((assign75180_e114734).powf(p.p113 - 1.0) * (((locals.var_tmf4_dn7 * locals.var_vxb_lim) - (locals.var_tmf4 * locals.var_vxb_lim_dn7)) / (locals.var_vxb_lim * locals.var_vxb_lim)))) } } else { (assign75180_e114736 * (p.p113 * ((((locals.var_tmf4_dn7 * locals.var_vxb_lim) - (locals.var_tmf4 * locals.var_vxb_lim_dn7)) / (locals.var_vxb_lim * locals.var_vxb_lim)) / assign75180_e114734))) }, if 0.0 == 0.0 && ((p.p113) as f64).is_finite() && ((p.p113) as f64).fract() == 0.0 { if p.p113 == 0.0 { 0.0 } else { (p.p113 * ((assign75180_e114734).powf(p.p113 - 1.0) * (((locals.var_tmf4_dn8 * locals.var_vxb_lim) - (locals.var_tmf4 * locals.var_vxb_lim_dn8)) / (locals.var_vxb_lim * locals.var_vxb_lim)))) } } else { (assign75180_e114736 * (p.p113 * ((((locals.var_tmf4_dn8 * locals.var_vxb_lim) - (locals.var_tmf4 * locals.var_vxb_lim_dn8)) / (locals.var_vxb_lim * locals.var_vxb_lim)) / assign75180_e114734))) }, if 0.0 == 0.0 && ((p.p113) as f64).is_finite() && ((p.p113) as f64).fract() == 0.0 { if p.p113 == 0.0 { 0.0 } else { (p.p113 * ((assign75180_e114734).powf(p.p113 - 1.0) * (((locals.var_tmf4_dn9 * locals.var_vxb_lim) - (locals.var_tmf4 * locals.var_vxb_lim_dn9)) / (locals.var_vxb_lim * locals.var_vxb_lim)))) } } else { (assign75180_e114736 * (p.p113 * ((((locals.var_tmf4_dn9 * locals.var_vxb_lim) - (locals.var_tmf4 * locals.var_vxb_lim_dn9)) / (locals.var_vxb_lim * locals.var_vxb_lim)) / assign75180_e114734))) }, if 0.0 == 0.0 && ((p.p113) as f64).is_finite() && ((p.p113) as f64).fract() == 0.0 { if p.p113 == 0.0 { 0.0 } else { (p.p113 * ((assign75180_e114734).powf(p.p113 - 1.0) * (((locals.var_tmf4_dn10 * locals.var_vxb_lim) - (locals.var_tmf4 * locals.var_vxb_lim_dn10)) / (locals.var_vxb_lim * locals.var_vxb_lim)))) } } else { (assign75180_e114736 * (p.p113 * ((((locals.var_tmf4_dn10 * locals.var_vxb_lim) - (locals.var_tmf4 * locals.var_vxb_lim_dn10)) / (locals.var_vxb_lim * locals.var_vxb_lim)) / assign75180_e114734))) }, if 0.0 == 0.0 && ((p.p113) as f64).is_finite() && ((p.p113) as f64).fract() == 0.0 { if p.p113 == 0.0 { 0.0 } else { (p.p113 * ((assign75180_e114734).powf(p.p113 - 1.0) * (((locals.var_tmf4_dn11 * locals.var_vxb_lim) - (locals.var_tmf4 * locals.var_vxb_lim_dn11)) / (locals.var_vxb_lim * locals.var_vxb_lim)))) } } else { (assign75180_e114736 * (p.p113 * ((((locals.var_tmf4_dn11 * locals.var_vxb_lim) - (locals.var_tmf4 * locals.var_vxb_lim_dn11)) / (locals.var_vxb_lim * locals.var_vxb_lim)) / assign75180_e114734))) }, if 0.0 == 0.0 && ((p.p113) as f64).is_finite() && ((p.p113) as f64).fract() == 0.0 { if p.p113 == 0.0 { 0.0 } else { (p.p113 * ((assign75180_e114734).powf(p.p113 - 1.0) * (((locals.var_tmf4_dn14 * locals.var_vxb_lim) - (locals.var_tmf4 * locals.var_vxb_lim_dn14)) / (locals.var_vxb_lim * locals.var_vxb_lim)))) } } else { (assign75180_e114736 * (p.p113 * ((((locals.var_tmf4_dn14 * locals.var_vxb_lim) - (locals.var_tmf4 * locals.var_vxb_lim_dn14)) / (locals.var_vxb_lim * locals.var_vxb_lim)) / assign75180_e114734))) },)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign75180_e114739;
        locals.var_tmf1_dn0 = assign75180_e114739_d_n0;
        locals.var_tmf1_dn2 = assign75180_e114739_d_n2;
        locals.var_tmf1_dn4 = assign75180_e114739_d_n4;
        locals.var_tmf1_dn5 = assign75180_e114739_d_n5;
        locals.var_tmf1_dn6 = assign75180_e114739_d_n6;
        locals.var_tmf1_dn7 = assign75180_e114739_d_n7;
        locals.var_tmf1_dn8 = assign75180_e114739_d_n8;
        locals.var_tmf1_dn9 = assign75180_e114739_d_n9;
        locals.var_tmf1_dn10 = assign75180_e114739_d_n10;
        locals.var_tmf1_dn11 = assign75180_e114739_d_n11;
        locals.var_tmf1_dn14 = assign75180_e114739_d_n14;
        locals.var_tmf1_rv = 0.0;

        let (assign75190_e114761, assign75190_e114761_d_n0, assign75190_e114761_d_n2, assign75190_e114761_d_n4, assign75190_e114761_d_n5, assign75190_e114761_d_n6, assign75190_e114761_d_n7, assign75190_e114761_d_n8, assign75190_e114761_d_n9, assign75190_e114761_d_n10, assign75190_e114761_d_n11, assign75190_e114761_d_n14,) = {
    if ((((((locals.var_guard1752 != 0.0) && (!((locals.var_guard1750 != 0.0) || (locals.var_guard1751 != 0.0)))) && (locals.var_guard1756 != 0.0)) && (locals.var_guard1757 != 0.0)) && (locals.var_guard1758 != 0.0)) && (locals.var_guard1759 == 0.0)) {
        let assign75190_e114758: f64 = (1.0 / p.p113);
        let assign75190_e114759: f64 = (locals.var_tmf1).powf(assign75190_e114758);
        (assign75190_e114759, if 0.0 == 0.0 && ((assign75190_e114758) as f64).is_finite() && ((assign75190_e114758) as f64).fract() == 0.0 { if assign75190_e114758 == 0.0 { 0.0 } else { (assign75190_e114758 * ((locals.var_tmf1).powf(assign75190_e114758 - 1.0) * locals.var_tmf1_dn0)) } } else { (assign75190_e114759 * (assign75190_e114758 * (locals.var_tmf1_dn0 / locals.var_tmf1))) }, if 0.0 == 0.0 && ((assign75190_e114758) as f64).is_finite() && ((assign75190_e114758) as f64).fract() == 0.0 { if assign75190_e114758 == 0.0 { 0.0 } else { (assign75190_e114758 * ((locals.var_tmf1).powf(assign75190_e114758 - 1.0) * locals.var_tmf1_dn2)) } } else { (assign75190_e114759 * (assign75190_e114758 * (locals.var_tmf1_dn2 / locals.var_tmf1))) }, if 0.0 == 0.0 && ((assign75190_e114758) as f64).is_finite() && ((assign75190_e114758) as f64).fract() == 0.0 { if assign75190_e114758 == 0.0 { 0.0 } else { (assign75190_e114758 * ((locals.var_tmf1).powf(assign75190_e114758 - 1.0) * locals.var_tmf1_dn4)) } } else { (assign75190_e114759 * (assign75190_e114758 * (locals.var_tmf1_dn4 / locals.var_tmf1))) }, if 0.0 == 0.0 && ((assign75190_e114758) as f64).is_finite() && ((assign75190_e114758) as f64).fract() == 0.0 { if assign75190_e114758 == 0.0 { 0.0 } else { (assign75190_e114758 * ((locals.var_tmf1).powf(assign75190_e114758 - 1.0) * locals.var_tmf1_dn5)) } } else { (assign75190_e114759 * (assign75190_e114758 * (locals.var_tmf1_dn5 / locals.var_tmf1))) }, if 0.0 == 0.0 && ((assign75190_e114758) as f64).is_finite() && ((assign75190_e114758) as f64).fract() == 0.0 { if assign75190_e114758 == 0.0 { 0.0 } else { (assign75190_e114758 * ((locals.var_tmf1).powf(assign75190_e114758 - 1.0) * locals.var_tmf1_dn6)) } } else { (assign75190_e114759 * (assign75190_e114758 * (locals.var_tmf1_dn6 / locals.var_tmf1))) }, if 0.0 == 0.0 && ((assign75190_e114758) as f64).is_finite() && ((assign75190_e114758) as f64).fract() == 0.0 { if assign75190_e114758 == 0.0 { 0.0 } else { (assign75190_e114758 * ((locals.var_tmf1).powf(assign75190_e114758 - 1.0) * locals.var_tmf1_dn7)) } } else { (assign75190_e114759 * (assign75190_e114758 * (locals.var_tmf1_dn7 / locals.var_tmf1))) }, if 0.0 == 0.0 && ((assign75190_e114758) as f64).is_finite() && ((assign75190_e114758) as f64).fract() == 0.0 { if assign75190_e114758 == 0.0 { 0.0 } else { (assign75190_e114758 * ((locals.var_tmf1).powf(assign75190_e114758 - 1.0) * locals.var_tmf1_dn8)) } } else { (assign75190_e114759 * (assign75190_e114758 * (locals.var_tmf1_dn8 / locals.var_tmf1))) }, if 0.0 == 0.0 && ((assign75190_e114758) as f64).is_finite() && ((assign75190_e114758) as f64).fract() == 0.0 { if assign75190_e114758 == 0.0 { 0.0 } else { (assign75190_e114758 * ((locals.var_tmf1).powf(assign75190_e114758 - 1.0) * locals.var_tmf1_dn9)) } } else { (assign75190_e114759 * (assign75190_e114758 * (locals.var_tmf1_dn9 / locals.var_tmf1))) }, if 0.0 == 0.0 && ((assign75190_e114758) as f64).is_finite() && ((assign75190_e114758) as f64).fract() == 0.0 { if assign75190_e114758 == 0.0 { 0.0 } else { (assign75190_e114758 * ((locals.var_tmf1).powf(assign75190_e114758 - 1.0) * locals.var_tmf1_dn10)) } } else { (assign75190_e114759 * (assign75190_e114758 * (locals.var_tmf1_dn10 / locals.var_tmf1))) }, if 0.0 == 0.0 && ((assign75190_e114758) as f64).is_finite() && ((assign75190_e114758) as f64).fract() == 0.0 { if assign75190_e114758 == 0.0 { 0.0 } else { (assign75190_e114758 * ((locals.var_tmf1).powf(assign75190_e114758 - 1.0) * locals.var_tmf1_dn11)) } } else { (assign75190_e114759 * (assign75190_e114758 * (locals.var_tmf1_dn11 / locals.var_tmf1))) }, if 0.0 == 0.0 && ((assign75190_e114758) as f64).is_finite() && ((assign75190_e114758) as f64).fract() == 0.0 { if assign75190_e114758 == 0.0 { 0.0 } else { (assign75190_e114758 * ((locals.var_tmf1).powf(assign75190_e114758 - 1.0) * locals.var_tmf1_dn14)) } } else { (assign75190_e114759 * (assign75190_e114758 * (locals.var_tmf1_dn14 / locals.var_tmf1))) },)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign75190_e114761;
        locals.var_tmf2_dn0 = assign75190_e114761_d_n0;
        locals.var_tmf2_dn2 = assign75190_e114761_d_n2;
        locals.var_tmf2_dn4 = assign75190_e114761_d_n4;
        locals.var_tmf2_dn5 = assign75190_e114761_d_n5;
        locals.var_tmf2_dn6 = assign75190_e114761_d_n6;
        locals.var_tmf2_dn7 = assign75190_e114761_d_n7;
        locals.var_tmf2_dn8 = assign75190_e114761_d_n8;
        locals.var_tmf2_dn9 = assign75190_e114761_d_n9;
        locals.var_tmf2_dn10 = assign75190_e114761_d_n10;
        locals.var_tmf2_dn11 = assign75190_e114761_d_n11;
        locals.var_tmf2_dn14 = assign75190_e114761_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign75200_e114783, assign75200_e114783_d_n0, assign75200_e114783_d_n2, assign75200_e114783_d_n4, assign75200_e114783_d_n5, assign75200_e114783_d_n6, assign75200_e114783_d_n7, assign75200_e114783_d_n8, assign75200_e114783_d_n9, assign75200_e114783_d_n10, assign75200_e114783_d_n11, assign75200_e114783_d_n14,) = {
    if ((((((locals.var_guard1752 != 0.0) && (!((locals.var_guard1750 != 0.0) || (locals.var_guard1751 != 0.0)))) && (locals.var_guard1756 != 0.0)) && (locals.var_guard1757 != 0.0)) && (locals.var_guard1758 != 0.0)) && (locals.var_guard1759 == 0.0)) {
        let assign75200_e114779: f64 = (locals.var_tmf3 * locals.var_tmf4);
        let assign75200_e114781: f64 = (assign75200_e114779 / locals.var_tmf2);
        (assign75200_e114781, (((((locals.var_tmf3_dn0 * locals.var_tmf4) + (locals.var_tmf3 * locals.var_tmf4_dn0)) * locals.var_tmf2) - (assign75200_e114779 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2)), (((((locals.var_tmf3_dn2 * locals.var_tmf4) + (locals.var_tmf3 * locals.var_tmf4_dn2)) * locals.var_tmf2) - (assign75200_e114779 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2)), (((((locals.var_tmf3_dn4 * locals.var_tmf4) + (locals.var_tmf3 * locals.var_tmf4_dn4)) * locals.var_tmf2) - (assign75200_e114779 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2)), (((((locals.var_tmf3_dn5 * locals.var_tmf4) + (locals.var_tmf3 * locals.var_tmf4_dn5)) * locals.var_tmf2) - (assign75200_e114779 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2)), (((((locals.var_tmf3_dn6 * locals.var_tmf4) + (locals.var_tmf3 * locals.var_tmf4_dn6)) * locals.var_tmf2) - (assign75200_e114779 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2)), (((((locals.var_tmf3_dn7 * locals.var_tmf4) + (locals.var_tmf3 * locals.var_tmf4_dn7)) * locals.var_tmf2) - (assign75200_e114779 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2)), (((((locals.var_tmf3_dn8 * locals.var_tmf4) + (locals.var_tmf3 * locals.var_tmf4_dn8)) * locals.var_tmf2) - (assign75200_e114779 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2)), (((((locals.var_tmf3_dn9 * locals.var_tmf4) + (locals.var_tmf3 * locals.var_tmf4_dn9)) * locals.var_tmf2) - (assign75200_e114779 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2)), (((((locals.var_tmf3_dn10 * locals.var_tmf4) + (locals.var_tmf3 * locals.var_tmf4_dn10)) * locals.var_tmf2) - (assign75200_e114779 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2)), (((((locals.var_tmf3_dn11 * locals.var_tmf4) + (locals.var_tmf3 * locals.var_tmf4_dn11)) * locals.var_tmf2) - (assign75200_e114779 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2)), (((((locals.var_tmf3_dn14 * locals.var_tmf4) + (locals.var_tmf3 * locals.var_tmf4_dn14)) * locals.var_tmf2) - (assign75200_e114779 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2)),)
    } else {
        (locals.var_vxbgmt, locals.var_vxbgmt_dn0, locals.var_vxbgmt_dn2, locals.var_vxbgmt_dn4, locals.var_vxbgmt_dn5, locals.var_vxbgmt_dn6, locals.var_vxbgmt_dn7, locals.var_vxbgmt_dn8, locals.var_vxbgmt_dn9, locals.var_vxbgmt_dn10, locals.var_vxbgmt_dn11, locals.var_vxbgmt_dn14,)
    }
};
        locals.var_vxbgmt = assign75200_e114783;
        locals.var_vxbgmt_dn0 = assign75200_e114783_d_n0;
        locals.var_vxbgmt_dn2 = assign75200_e114783_d_n2;
        locals.var_vxbgmt_dn4 = assign75200_e114783_d_n4;
        locals.var_vxbgmt_dn5 = assign75200_e114783_d_n5;
        locals.var_vxbgmt_dn6 = assign75200_e114783_d_n6;
        locals.var_vxbgmt_dn7 = assign75200_e114783_d_n7;
        locals.var_vxbgmt_dn8 = assign75200_e114783_d_n8;
        locals.var_vxbgmt_dn9 = assign75200_e114783_d_n9;
        locals.var_vxbgmt_dn10 = assign75200_e114783_d_n10;
        locals.var_vxbgmt_dn11 = assign75200_e114783_d_n11;
        locals.var_vxbgmt_dn14 = assign75200_e114783_d_n14;
        locals.var_vxbgmt_rv = 0.0;

        let (assign75210_e114811, assign75210_e114811_d_n0, assign75210_e114811_d_n2, assign75210_e114811_d_n4, assign75210_e114811_d_n5, assign75210_e114811_d_n6, assign75210_e114811_d_n7, assign75210_e114811_d_n8, assign75210_e114811_d_n9, assign75210_e114811_d_n10, assign75210_e114811_d_n11, assign75210_e114811_d_n14,) = {
    if (((((locals.var_guard1752 != 0.0) && (!((locals.var_guard1750 != 0.0) || (locals.var_guard1751 != 0.0)))) && (locals.var_guard1756 != 0.0)) && (locals.var_guard1757 != 0.0)) && (locals.var_guard1758 != 0.0)) {
        let assign75210_e114798: f64 = (locals.var_vxbgmt + p.p137);
        let assign75210_e114801: f64 = (locals.var_vxbgmt + p.p137);
        let assign75210_e114802: f64 = (assign75210_e114798 * assign75210_e114801);
        let assign75210_e114805: f64 = (4.0 * 0.1);
        let assign75210_e114807: f64 = (assign75210_e114805 * 0.1);
        let assign75210_e114808: f64 = (assign75210_e114802 + assign75210_e114807);
        let assign75210_e114809: f64 = (assign75210_e114808).sqrt();
        (assign75210_e114809, (((locals.var_vxbgmt_dn0 * assign75210_e114801) + (assign75210_e114798 * locals.var_vxbgmt_dn0)) / (2.0 * assign75210_e114809)), (((locals.var_vxbgmt_dn2 * assign75210_e114801) + (assign75210_e114798 * locals.var_vxbgmt_dn2)) / (2.0 * assign75210_e114809)), (((locals.var_vxbgmt_dn4 * assign75210_e114801) + (assign75210_e114798 * locals.var_vxbgmt_dn4)) / (2.0 * assign75210_e114809)), (((locals.var_vxbgmt_dn5 * assign75210_e114801) + (assign75210_e114798 * locals.var_vxbgmt_dn5)) / (2.0 * assign75210_e114809)), (((locals.var_vxbgmt_dn6 * assign75210_e114801) + (assign75210_e114798 * locals.var_vxbgmt_dn6)) / (2.0 * assign75210_e114809)), (((locals.var_vxbgmt_dn7 * assign75210_e114801) + (assign75210_e114798 * locals.var_vxbgmt_dn7)) / (2.0 * assign75210_e114809)), (((locals.var_vxbgmt_dn8 * assign75210_e114801) + (assign75210_e114798 * locals.var_vxbgmt_dn8)) / (2.0 * assign75210_e114809)), (((locals.var_vxbgmt_dn9 * assign75210_e114801) + (assign75210_e114798 * locals.var_vxbgmt_dn9)) / (2.0 * assign75210_e114809)), (((locals.var_vxbgmt_dn10 * assign75210_e114801) + (assign75210_e114798 * locals.var_vxbgmt_dn10)) / (2.0 * assign75210_e114809)), (((locals.var_vxbgmt_dn11 * assign75210_e114801) + (assign75210_e114798 * locals.var_vxbgmt_dn11)) / (2.0 * assign75210_e114809)), (((locals.var_vxbgmt_dn14 * assign75210_e114801) + (assign75210_e114798 * locals.var_vxbgmt_dn14)) / (2.0 * assign75210_e114809)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign75210_e114811;
        locals.var_tmf2_dn0 = assign75210_e114811_d_n0;
        locals.var_tmf2_dn2 = assign75210_e114811_d_n2;
        locals.var_tmf2_dn4 = assign75210_e114811_d_n4;
        locals.var_tmf2_dn5 = assign75210_e114811_d_n5;
        locals.var_tmf2_dn6 = assign75210_e114811_d_n6;
        locals.var_tmf2_dn7 = assign75210_e114811_d_n7;
        locals.var_tmf2_dn8 = assign75210_e114811_d_n8;
        locals.var_tmf2_dn9 = assign75210_e114811_d_n9;
        locals.var_tmf2_dn10 = assign75210_e114811_d_n10;
        locals.var_tmf2_dn11 = assign75210_e114811_d_n11;
        locals.var_tmf2_dn14 = assign75210_e114811_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign75220_e114834, assign75220_e114834_d_n0, assign75220_e114834_d_n2, assign75220_e114834_d_n4, assign75220_e114834_d_n5, assign75220_e114834_d_n6, assign75220_e114834_d_n7, assign75220_e114834_d_n8, assign75220_e114834_d_n9, assign75220_e114834_d_n10, assign75220_e114834_d_n11, assign75220_e114834_d_n14,) = {
    if (((((locals.var_guard1752 != 0.0) && (!((locals.var_guard1750 != 0.0) || (locals.var_guard1751 != 0.0)))) && (locals.var_guard1756 != 0.0)) && (locals.var_guard1757 != 0.0)) && (locals.var_guard1758 != 0.0)) {
        let assign75220_e114828: f64 = (locals.var_vxbgmt + p.p137);
        let assign75220_e114830: f64 = (assign75220_e114828 / locals.var_tmf2);
        let assign75220_e114831: f64 = (1.0 + assign75220_e114830);
        let assign75220_e114832: f64 = (0.5 * assign75220_e114831);
        (assign75220_e114832, (0.5 * (((locals.var_vxbgmt_dn0 * locals.var_tmf2) - (assign75220_e114828 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vxbgmt_dn2 * locals.var_tmf2) - (assign75220_e114828 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vxbgmt_dn4 * locals.var_tmf2) - (assign75220_e114828 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vxbgmt_dn5 * locals.var_tmf2) - (assign75220_e114828 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vxbgmt_dn6 * locals.var_tmf2) - (assign75220_e114828 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vxbgmt_dn7 * locals.var_tmf2) - (assign75220_e114828 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vxbgmt_dn8 * locals.var_tmf2) - (assign75220_e114828 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vxbgmt_dn9 * locals.var_tmf2) - (assign75220_e114828 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vxbgmt_dn10 * locals.var_tmf2) - (assign75220_e114828 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vxbgmt_dn11 * locals.var_tmf2) - (assign75220_e114828 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vxbgmt_dn14 * locals.var_tmf2) - (assign75220_e114828 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn11, locals.var_t9_dn14,)
    }
};
        locals.var_t9 = assign75220_e114834;
        locals.var_t9_dn0 = assign75220_e114834_d_n0;
        locals.var_t9_dn2 = assign75220_e114834_d_n2;
        locals.var_t9_dn4 = assign75220_e114834_d_n4;
        locals.var_t9_dn5 = assign75220_e114834_d_n5;
        locals.var_t9_dn6 = assign75220_e114834_d_n6;
        locals.var_t9_dn7 = assign75220_e114834_d_n7;
        locals.var_t9_dn8 = assign75220_e114834_d_n8;
        locals.var_t9_dn9 = assign75220_e114834_d_n9;
        locals.var_t9_dn10 = assign75220_e114834_d_n10;
        locals.var_t9_dn11 = assign75220_e114834_d_n11;
        locals.var_t9_dn14 = assign75220_e114834_d_n14;
        locals.var_t9_rv = 0.0;

        let (assign75230_e114855, assign75230_e114855_d_n0, assign75230_e114855_d_n2, assign75230_e114855_d_n4, assign75230_e114855_d_n5, assign75230_e114855_d_n6, assign75230_e114855_d_n7, assign75230_e114855_d_n8, assign75230_e114855_d_n9, assign75230_e114855_d_n10, assign75230_e114855_d_n11, assign75230_e114855_d_n14,) = {
    if (((((locals.var_guard1752 != 0.0) && (!((locals.var_guard1750 != 0.0) || (locals.var_guard1751 != 0.0)))) && (locals.var_guard1756 != 0.0)) && (locals.var_guard1757 != 0.0)) && (locals.var_guard1758 != 0.0)) {
        let assign75230_e114850: f64 = (locals.var_vxbgmt + p.p137);
        let assign75230_e114852: f64 = (assign75230_e114850 + locals.var_tmf2);
        let assign75230_e114853: f64 = (0.5 * assign75230_e114852);
        (assign75230_e114853, (0.5 * (locals.var_vxbgmt_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_vxbgmt_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_vxbgmt_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_vxbgmt_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_vxbgmt_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_vxbgmt_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_vxbgmt_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_vxbgmt_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_vxbgmt_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_vxbgmt_dn11 + locals.var_tmf2_dn11)), (0.5 * (locals.var_vxbgmt_dn14 + locals.var_tmf2_dn14)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign75230_e114855;
        locals.var_t2_dn0 = assign75230_e114855_d_n0;
        locals.var_t2_dn2 = assign75230_e114855_d_n2;
        locals.var_t2_dn4 = assign75230_e114855_d_n4;
        locals.var_t2_dn5 = assign75230_e114855_d_n5;
        locals.var_t2_dn6 = assign75230_e114855_d_n6;
        locals.var_t2_dn7 = assign75230_e114855_d_n7;
        locals.var_t2_dn8 = assign75230_e114855_d_n8;
        locals.var_t2_dn9 = assign75230_e114855_d_n9;
        locals.var_t2_dn10 = assign75230_e114855_d_n10;
        locals.var_t2_dn11 = assign75230_e114855_d_n11;
        locals.var_t2_dn14 = assign75230_e114855_d_n14;
        locals.var_t2_rv = 0.0;

        let assign75240_e114858: f64 = if locals.var_t2 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1760 = assign75240_e114858;
        locals.var_guard1760_rv = 0.0;

        let (assign75250_e114875, assign75250_e114875_d_n0, assign75250_e114875_d_n2, assign75250_e114875_d_n4, assign75250_e114875_d_n5, assign75250_e114875_d_n6, assign75250_e114875_d_n7, assign75250_e114875_d_n8, assign75250_e114875_d_n9, assign75250_e114875_d_n10, assign75250_e114875_d_n11, assign75250_e114875_d_n14,) = {
    if ((((((locals.var_guard1752 != 0.0) && (!((locals.var_guard1750 != 0.0) || (locals.var_guard1751 != 0.0)))) && (locals.var_guard1756 != 0.0)) && (locals.var_guard1757 != 0.0)) && (locals.var_guard1758 != 0.0)) && (locals.var_guard1760 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign75250_e114875;
        locals.var_t2_dn0 = assign75250_e114875_d_n0;
        locals.var_t2_dn2 = assign75250_e114875_d_n2;
        locals.var_t2_dn4 = assign75250_e114875_d_n4;
        locals.var_t2_dn5 = assign75250_e114875_d_n5;
        locals.var_t2_dn6 = assign75250_e114875_d_n6;
        locals.var_t2_dn7 = assign75250_e114875_d_n7;
        locals.var_t2_dn8 = assign75250_e114875_d_n8;
        locals.var_t2_dn9 = assign75250_e114875_d_n9;
        locals.var_t2_dn10 = assign75250_e114875_d_n10;
        locals.var_t2_dn11 = assign75250_e114875_d_n11;
        locals.var_t2_dn14 = assign75250_e114875_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign75260_e114892, assign75260_e114892_d_n0, assign75260_e114892_d_n2, assign75260_e114892_d_n4, assign75260_e114892_d_n5, assign75260_e114892_d_n6, assign75260_e114892_d_n7, assign75260_e114892_d_n8, assign75260_e114892_d_n9, assign75260_e114892_d_n10, assign75260_e114892_d_n11, assign75260_e114892_d_n14,) = {
    if ((((((locals.var_guard1752 != 0.0) && (!((locals.var_guard1750 != 0.0) || (locals.var_guard1751 != 0.0)))) && (locals.var_guard1756 != 0.0)) && (locals.var_guard1757 != 0.0)) && (locals.var_guard1758 != 0.0)) && (locals.var_guard1760 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn11, locals.var_t9_dn14,)
    }
};
        locals.var_t9 = assign75260_e114892;
        locals.var_t9_dn0 = assign75260_e114892_d_n0;
        locals.var_t9_dn2 = assign75260_e114892_d_n2;
        locals.var_t9_dn4 = assign75260_e114892_d_n4;
        locals.var_t9_dn5 = assign75260_e114892_d_n5;
        locals.var_t9_dn6 = assign75260_e114892_d_n6;
        locals.var_t9_dn7 = assign75260_e114892_d_n7;
        locals.var_t9_dn8 = assign75260_e114892_d_n8;
        locals.var_t9_dn9 = assign75260_e114892_d_n9;
        locals.var_t9_dn10 = assign75260_e114892_d_n10;
        locals.var_t9_dn11 = assign75260_e114892_d_n11;
        locals.var_t9_dn14 = assign75260_e114892_d_n14;
        locals.var_t9_rv = 0.0;

        let (assign75270_e114912, assign75270_e114912_d_n0, assign75270_e114912_d_n2, assign75270_e114912_d_n4, assign75270_e114912_d_n5, assign75270_e114912_d_n6, assign75270_e114912_d_n7, assign75270_e114912_d_n8, assign75270_e114912_d_n9, assign75270_e114912_d_n10, assign75270_e114912_d_n11, assign75270_e114912_d_n14,) = {
    if (((((locals.var_guard1752 != 0.0) && (!((locals.var_guard1750 != 0.0) || (locals.var_guard1751 != 0.0)))) && (locals.var_guard1756 != 0.0)) && (locals.var_guard1757 != 0.0)) && (locals.var_guard1758 != 0.0)) {
        let assign75270_e114907: f64 = (locals.var_kjunc * locals.var_t2);
        let assign75270_e114908: f64 = (assign75270_e114907).sqrt();
        let assign75270_e114910: f64 = (assign75270_e114908 * p.p432);
        (assign75270_e114910, ((((locals.var_kjunc_dn0 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn0)) / (2.0 * assign75270_e114908)) * p.p432), ((((locals.var_kjunc_dn2 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn2)) / (2.0 * assign75270_e114908)) * p.p432), ((((locals.var_kjunc_dn4 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn4)) / (2.0 * assign75270_e114908)) * p.p432), ((((locals.var_kjunc_dn5 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn5)) / (2.0 * assign75270_e114908)) * p.p432), ((((locals.var_kjunc_dn6 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn6)) / (2.0 * assign75270_e114908)) * p.p432), ((((locals.var_kjunc_dn7 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn7)) / (2.0 * assign75270_e114908)) * p.p432), ((((locals.var_kjunc_dn8 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn8)) / (2.0 * assign75270_e114908)) * p.p432), ((((locals.var_kjunc_dn9 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn9)) / (2.0 * assign75270_e114908)) * p.p432), ((((locals.var_kjunc_dn10 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn10)) / (2.0 * assign75270_e114908)) * p.p432), ((((locals.var_kjunc_dn11 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn11)) / (2.0 * assign75270_e114908)) * p.p432), ((((locals.var_kjunc_dn14 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn14)) / (2.0 * assign75270_e114908)) * p.p432),)
    } else {
        (locals.var_wjunc0, locals.var_wjunc0_dn0, locals.var_wjunc0_dn2, locals.var_wjunc0_dn4, locals.var_wjunc0_dn5, locals.var_wjunc0_dn6, locals.var_wjunc0_dn7, locals.var_wjunc0_dn8, locals.var_wjunc0_dn9, locals.var_wjunc0_dn10, locals.var_wjunc0_dn11, locals.var_wjunc0_dn14,)
    }
};
        locals.var_wjunc0 = assign75270_e114912;
        locals.var_wjunc0_dn0 = assign75270_e114912_d_n0;
        locals.var_wjunc0_dn2 = assign75270_e114912_d_n2;
        locals.var_wjunc0_dn4 = assign75270_e114912_d_n4;
        locals.var_wjunc0_dn5 = assign75270_e114912_d_n5;
        locals.var_wjunc0_dn6 = assign75270_e114912_d_n6;
        locals.var_wjunc0_dn7 = assign75270_e114912_d_n7;
        locals.var_wjunc0_dn8 = assign75270_e114912_d_n8;
        locals.var_wjunc0_dn9 = assign75270_e114912_d_n9;
        locals.var_wjunc0_dn10 = assign75270_e114912_d_n10;
        locals.var_wjunc0_dn11 = assign75270_e114912_d_n11;
        locals.var_wjunc0_dn14 = assign75270_e114912_d_n14;
        locals.var_wjunc0_rv = 0.0;

        let (assign75280_e114929, assign75280_e114929_d_n0, assign75280_e114929_d_n2, assign75280_e114929_d_n4, assign75280_e114929_d_n5, assign75280_e114929_d_n6, assign75280_e114929_d_n7, assign75280_e114929_d_n8, assign75280_e114929_d_n9, assign75280_e114929_d_n10, assign75280_e114929_d_n11, assign75280_e114929_d_n14,) = {
    if (((((locals.var_guard1752 != 0.0) && (!((locals.var_guard1750 != 0.0) || (locals.var_guard1751 != 0.0)))) && (locals.var_guard1756 != 0.0)) && (locals.var_guard1757 != 0.0)) && (locals.var_guard1758 != 0.0)) {
        let assign75280_e114927: f64 = (locals.var_lover_func - locals.var_wjunc0);
        (assign75280_e114927, (locals.var_lover_func_dn0 - locals.var_wjunc0_dn0), (locals.var_lover_func_dn2 - locals.var_wjunc0_dn2), (locals.var_lover_func_dn4 - locals.var_wjunc0_dn4), (locals.var_lover_func_dn5 - locals.var_wjunc0_dn5), (locals.var_lover_func_dn6 - locals.var_wjunc0_dn6), (locals.var_lover_func_dn7 - locals.var_wjunc0_dn7), (locals.var_lover_func_dn8 - locals.var_wjunc0_dn8), (locals.var_lover_func_dn9 - locals.var_wjunc0_dn9), (locals.var_lover_func_dn10 - locals.var_wjunc0_dn10), (locals.var_lover_func_dn11 - locals.var_wjunc0_dn11), (locals.var_lover_func_dn14 - locals.var_wjunc0_dn14),)
    } else {
        (locals.var_lover_func, locals.var_lover_func_dn0, locals.var_lover_func_dn2, locals.var_lover_func_dn4, locals.var_lover_func_dn5, locals.var_lover_func_dn6, locals.var_lover_func_dn7, locals.var_lover_func_dn8, locals.var_lover_func_dn9, locals.var_lover_func_dn10, locals.var_lover_func_dn11, locals.var_lover_func_dn14,)
    }
};
        locals.var_lover_func = assign75280_e114929;
        locals.var_lover_func_dn0 = assign75280_e114929_d_n0;
        locals.var_lover_func_dn2 = assign75280_e114929_d_n2;
        locals.var_lover_func_dn4 = assign75280_e114929_d_n4;
        locals.var_lover_func_dn5 = assign75280_e114929_d_n5;
        locals.var_lover_func_dn6 = assign75280_e114929_d_n6;
        locals.var_lover_func_dn7 = assign75280_e114929_d_n7;
        locals.var_lover_func_dn8 = assign75280_e114929_d_n8;
        locals.var_lover_func_dn9 = assign75280_e114929_d_n9;
        locals.var_lover_func_dn10 = assign75280_e114929_d_n10;
        locals.var_lover_func_dn11 = assign75280_e114929_d_n11;
        locals.var_lover_func_dn14 = assign75280_e114929_d_n14;
        locals.var_lover_func_rv = 0.0;

        let assign75290_e114948: f64 = if (((((p.p35 == 1.0) && (p.p63 > 0.0)) && (locals.var_uc_nover > 0.0)) && (locals.var_uc_cvdsover != 0.0)) && (p.p55 != 1.0)) { 1.0 } else { 0.0 };
        locals.var_guard1761 = assign75290_e114948;
        locals.var_guard1761_rv = 0.0;

        let (assign75300_e114961,) = {
    if (((locals.var_guard1753 != 0.0) && (!(((locals.var_guard1750 != 0.0) || (locals.var_guard1751 != 0.0)) || (locals.var_guard1752 != 0.0)))) && (locals.var_guard1761 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_flg_calcqover,)
    }
};
        locals.var_flg_calcqover = assign75300_e114961;
        locals.var_flg_calcqover_rv = 0.0;

        let (assign75310_e114976, assign75310_e114976_d_n2, assign75310_e114976_d_n7, assign75310_e114976_d_n8, assign75310_e114976_d_n9,) = {
    if (((locals.var_guard1753 != 0.0) && (!(((locals.var_guard1750 != 0.0) || (locals.var_guard1751 != 0.0)) || (locals.var_guard1752 != 0.0)))) && (locals.var_guard1761 != 0.0)) {
        let assign75310_e114974: f64 = (locals.var_vgsei - locals.var_vbsei);
        (assign75310_e114974, (locals.var_vgsei_dn2 - locals.var_vbsei_dn2), locals.var_vgsei_dn7, 0.0, (-locals.var_vbsei_dn9),)
    } else {
        (locals.var_vgbgmt, locals.var_vgbgmt_dn2, locals.var_vgbgmt_dn7, locals.var_vgbgmt_dn8, locals.var_vgbgmt_dn9,)
    }
};
        locals.var_vgbgmt = assign75310_e114976;
        locals.var_vgbgmt_dn2 = assign75310_e114976_d_n2;
        locals.var_vgbgmt_dn7 = assign75310_e114976_d_n7;
        locals.var_vgbgmt_dn8 = assign75310_e114976_d_n8;
        locals.var_vgbgmt_dn9 = assign75310_e114976_d_n9;
        locals.var_vgbgmt_rv = 0.0;

        let (assign75320_e114991, assign75320_e114991_d_n0, assign75320_e114991_d_n2, assign75320_e114991_d_n4, assign75320_e114991_d_n5, assign75320_e114991_d_n6, assign75320_e114991_d_n7, assign75320_e114991_d_n8, assign75320_e114991_d_n9, assign75320_e114991_d_n10, assign75320_e114991_d_n11, assign75320_e114991_d_n14,) = {
    if (((locals.var_guard1753 != 0.0) && (!(((locals.var_guard1750 != 0.0) || (locals.var_guard1751 != 0.0)) || (locals.var_guard1752 != 0.0)))) && (locals.var_guard1761 != 0.0)) {
        let assign75320_e114989: f64 = (locals.var_vdsei - locals.var_vbsei);
        (assign75320_e114989, locals.var_vdsei_dn0, (locals.var_vdsei_dn2 - locals.var_vbsei_dn2), 0.0, 0.0, 0.0, 0.0, 0.0, (-locals.var_vbsei_dn9), 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vxbgmt, locals.var_vxbgmt_dn0, locals.var_vxbgmt_dn2, locals.var_vxbgmt_dn4, locals.var_vxbgmt_dn5, locals.var_vxbgmt_dn6, locals.var_vxbgmt_dn7, locals.var_vxbgmt_dn8, locals.var_vxbgmt_dn9, locals.var_vxbgmt_dn10, locals.var_vxbgmt_dn11, locals.var_vxbgmt_dn14,)
    }
};
        locals.var_vxbgmt = assign75320_e114991;
        locals.var_vxbgmt_dn0 = assign75320_e114991_d_n0;
        locals.var_vxbgmt_dn2 = assign75320_e114991_d_n2;
        locals.var_vxbgmt_dn4 = assign75320_e114991_d_n4;
        locals.var_vxbgmt_dn5 = assign75320_e114991_d_n5;
        locals.var_vxbgmt_dn6 = assign75320_e114991_d_n6;
        locals.var_vxbgmt_dn7 = assign75320_e114991_d_n7;
        locals.var_vxbgmt_dn8 = assign75320_e114991_d_n8;
        locals.var_vxbgmt_dn9 = assign75320_e114991_d_n9;
        locals.var_vxbgmt_dn10 = assign75320_e114991_d_n10;
        locals.var_vxbgmt_dn11 = assign75320_e114991_d_n11;
        locals.var_vxbgmt_dn14 = assign75320_e114991_d_n14;
        locals.var_vxbgmt_rv = 0.0;

        let (assign75330_e114995, assign75330_e114995_d_n0, assign75330_e114995_d_n2, assign75330_e114995_d_n4, assign75330_e114995_d_n5, assign75330_e114995_d_n6, assign75330_e114995_d_n7, assign75330_e114995_d_n8, assign75330_e114995_d_n9, assign75330_e114995_d_n10, assign75330_e114995_d_n11, assign75330_e114995_d_n14,) = {
    if (locals.var_flg_calcqover != 0.0) {
        (0.4, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vbs_bnd_over__blk1769, locals.var_vbs_bnd_over__blk1769_dn0, locals.var_vbs_bnd_over__blk1769_dn2, locals.var_vbs_bnd_over__blk1769_dn4, locals.var_vbs_bnd_over__blk1769_dn5, locals.var_vbs_bnd_over__blk1769_dn6, locals.var_vbs_bnd_over__blk1769_dn7, locals.var_vbs_bnd_over__blk1769_dn8, locals.var_vbs_bnd_over__blk1769_dn9, locals.var_vbs_bnd_over__blk1769_dn10, locals.var_vbs_bnd_over__blk1769_dn11, locals.var_vbs_bnd_over__blk1769_dn14,)
    }
};
        locals.var_vbs_bnd_over__blk1769 = assign75330_e114995;
        locals.var_vbs_bnd_over__blk1769_dn0 = assign75330_e114995_d_n0;
        locals.var_vbs_bnd_over__blk1769_dn2 = assign75330_e114995_d_n2;
        locals.var_vbs_bnd_over__blk1769_dn4 = assign75330_e114995_d_n4;
        locals.var_vbs_bnd_over__blk1769_dn5 = assign75330_e114995_d_n5;
        locals.var_vbs_bnd_over__blk1769_dn6 = assign75330_e114995_d_n6;
        locals.var_vbs_bnd_over__blk1769_dn7 = assign75330_e114995_d_n7;
        locals.var_vbs_bnd_over__blk1769_dn8 = assign75330_e114995_d_n8;
        locals.var_vbs_bnd_over__blk1769_dn9 = assign75330_e114995_d_n9;
        locals.var_vbs_bnd_over__blk1769_dn10 = assign75330_e114995_d_n10;
        locals.var_vbs_bnd_over__blk1769_dn11 = assign75330_e114995_d_n11;
        locals.var_vbs_bnd_over__blk1769_dn14 = assign75330_e114995_d_n14;
        locals.var_vbs_bnd_over__blk1769_rv = 0.0;

        let (assign75350_e115003,) = {
    if (locals.var_flg_calcqover != 0.0) {
        (0.0,)
    } else {
        (locals.var_flg_fd_mode__blk1770,)
    }
};
        locals.var_flg_fd_mode__blk1770 = assign75350_e115003;
        locals.var_flg_fd_mode__blk1770_rv = 0.0;

        let (assign75360_e115007, assign75360_e115007_d_n0, assign75360_e115007_d_n2, assign75360_e115007_d_n4, assign75360_e115007_d_n5, assign75360_e115007_d_n6, assign75360_e115007_d_n7, assign75360_e115007_d_n8, assign75360_e115007_d_n9, assign75360_e115007_d_n10, assign75360_e115007_d_n11, assign75360_e115007_d_n14,) = {
    if (locals.var_flg_calcqover != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fb, locals.var_fb_dn0, locals.var_fb_dn2, locals.var_fb_dn4, locals.var_fb_dn5, locals.var_fb_dn6, locals.var_fb_dn7, locals.var_fb_dn8, locals.var_fb_dn9, locals.var_fb_dn10, locals.var_fb_dn11, locals.var_fb_dn14,)
    }
};
        locals.var_fb = assign75360_e115007;
        locals.var_fb_dn0 = assign75360_e115007_d_n0;
        locals.var_fb_dn2 = assign75360_e115007_d_n2;
        locals.var_fb_dn4 = assign75360_e115007_d_n4;
        locals.var_fb_dn5 = assign75360_e115007_d_n5;
        locals.var_fb_dn6 = assign75360_e115007_d_n6;
        locals.var_fb_dn7 = assign75360_e115007_d_n7;
        locals.var_fb_dn8 = assign75360_e115007_d_n8;
        locals.var_fb_dn9 = assign75360_e115007_d_n9;
        locals.var_fb_dn10 = assign75360_e115007_d_n10;
        locals.var_fb_dn11 = assign75360_e115007_d_n11;
        locals.var_fb_dn14 = assign75360_e115007_d_n14;
        locals.var_fb_rv = 0.0;

        let (assign75370_e115011, assign75370_e115011_d_n0, assign75370_e115011_d_n2, assign75370_e115011_d_n4, assign75370_e115011_d_n5, assign75370_e115011_d_n6, assign75370_e115011_d_n7, assign75370_e115011_d_n8, assign75370_e115011_d_n9, assign75370_e115011_d_n10, assign75370_e115011_d_n11, assign75370_e115011_d_n14,) = {
    if (locals.var_flg_calcqover != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fs01, locals.var_fs01_dn0, locals.var_fs01_dn2, locals.var_fs01_dn4, locals.var_fs01_dn5, locals.var_fs01_dn6, locals.var_fs01_dn7, locals.var_fs01_dn8, locals.var_fs01_dn9, locals.var_fs01_dn10, locals.var_fs01_dn11, locals.var_fs01_dn14,)
    }
};
        locals.var_fs01 = assign75370_e115011;
        locals.var_fs01_dn0 = assign75370_e115011_d_n0;
        locals.var_fs01_dn2 = assign75370_e115011_d_n2;
        locals.var_fs01_dn4 = assign75370_e115011_d_n4;
        locals.var_fs01_dn5 = assign75370_e115011_d_n5;
        locals.var_fs01_dn6 = assign75370_e115011_d_n6;
        locals.var_fs01_dn7 = assign75370_e115011_d_n7;
        locals.var_fs01_dn8 = assign75370_e115011_d_n8;
        locals.var_fs01_dn9 = assign75370_e115011_d_n9;
        locals.var_fs01_dn10 = assign75370_e115011_d_n10;
        locals.var_fs01_dn11 = assign75370_e115011_d_n11;
        locals.var_fs01_dn14 = assign75370_e115011_d_n14;
        locals.var_fs01_rv = 0.0;

        let (assign75380_e115015, assign75380_e115015_d_n0, assign75380_e115015_d_n2, assign75380_e115015_d_n4, assign75380_e115015_d_n5, assign75380_e115015_d_n6, assign75380_e115015_d_n7, assign75380_e115015_d_n8, assign75380_e115015_d_n9, assign75380_e115015_d_n10, assign75380_e115015_d_n11, assign75380_e115015_d_n14,) = {
    if (locals.var_flg_calcqover != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fs02, locals.var_fs02_dn0, locals.var_fs02_dn2, locals.var_fs02_dn4, locals.var_fs02_dn5, locals.var_fs02_dn6, locals.var_fs02_dn7, locals.var_fs02_dn8, locals.var_fs02_dn9, locals.var_fs02_dn10, locals.var_fs02_dn11, locals.var_fs02_dn14,)
    }
};
        locals.var_fs02 = assign75380_e115015;
        locals.var_fs02_dn0 = assign75380_e115015_d_n0;
        locals.var_fs02_dn2 = assign75380_e115015_d_n2;
        locals.var_fs02_dn4 = assign75380_e115015_d_n4;
        locals.var_fs02_dn5 = assign75380_e115015_d_n5;
        locals.var_fs02_dn6 = assign75380_e115015_d_n6;
        locals.var_fs02_dn7 = assign75380_e115015_d_n7;
        locals.var_fs02_dn8 = assign75380_e115015_d_n8;
        locals.var_fs02_dn9 = assign75380_e115015_d_n9;
        locals.var_fs02_dn10 = assign75380_e115015_d_n10;
        locals.var_fs02_dn11 = assign75380_e115015_d_n11;
        locals.var_fs02_dn14 = assign75380_e115015_d_n14;
        locals.var_fs02_rv = 0.0;

        let (assign75390_e115019, assign75390_e115019_d_n0, assign75390_e115019_d_n2, assign75390_e115019_d_n4, assign75390_e115019_d_n5, assign75390_e115019_d_n6, assign75390_e115019_d_n7, assign75390_e115019_d_n8, assign75390_e115019_d_n9, assign75390_e115019_d_n10, assign75390_e115019_d_n11, assign75390_e115019_d_n14,) = {
    if (locals.var_flg_calcqover != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fs0, locals.var_fs0_dn0, locals.var_fs0_dn2, locals.var_fs0_dn4, locals.var_fs0_dn5, locals.var_fs0_dn6, locals.var_fs0_dn7, locals.var_fs0_dn8, locals.var_fs0_dn9, locals.var_fs0_dn10, locals.var_fs0_dn11, locals.var_fs0_dn14,)
    }
};
        locals.var_fs0 = assign75390_e115019;
        locals.var_fs0_dn0 = assign75390_e115019_d_n0;
        locals.var_fs0_dn2 = assign75390_e115019_d_n2;
        locals.var_fs0_dn4 = assign75390_e115019_d_n4;
        locals.var_fs0_dn5 = assign75390_e115019_d_n5;
        locals.var_fs0_dn6 = assign75390_e115019_d_n6;
        locals.var_fs0_dn7 = assign75390_e115019_d_n7;
        locals.var_fs0_dn8 = assign75390_e115019_d_n8;
        locals.var_fs0_dn9 = assign75390_e115019_d_n9;
        locals.var_fs0_dn10 = assign75390_e115019_d_n10;
        locals.var_fs0_dn11 = assign75390_e115019_d_n11;
        locals.var_fs0_dn14 = assign75390_e115019_d_n14;
        locals.var_fs0_rv = 0.0;

        let (assign75400_e115023, assign75400_e115023_d_n0, assign75400_e115023_d_n2, assign75400_e115023_d_n4, assign75400_e115023_d_n5, assign75400_e115023_d_n6, assign75400_e115023_d_n7, assign75400_e115023_d_n8, assign75400_e115023_d_n9, assign75400_e115023_d_n10, assign75400_e115023_d_n11, assign75400_e115023_d_n14,) = {
    if (locals.var_flg_calcqover != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dps0, locals.var_dps0_dn0, locals.var_dps0_dn2, locals.var_dps0_dn4, locals.var_dps0_dn5, locals.var_dps0_dn6, locals.var_dps0_dn7, locals.var_dps0_dn8, locals.var_dps0_dn9, locals.var_dps0_dn10, locals.var_dps0_dn11, locals.var_dps0_dn14,)
    }
};
        locals.var_dps0 = assign75400_e115023;
        locals.var_dps0_dn0 = assign75400_e115023_d_n0;
        locals.var_dps0_dn2 = assign75400_e115023_d_n2;
        locals.var_dps0_dn4 = assign75400_e115023_d_n4;
        locals.var_dps0_dn5 = assign75400_e115023_d_n5;
        locals.var_dps0_dn6 = assign75400_e115023_d_n6;
        locals.var_dps0_dn7 = assign75400_e115023_d_n7;
        locals.var_dps0_dn8 = assign75400_e115023_d_n8;
        locals.var_dps0_dn9 = assign75400_e115023_d_n9;
        locals.var_dps0_dn10 = assign75400_e115023_d_n10;
        locals.var_dps0_dn11 = assign75400_e115023_d_n11;
        locals.var_dps0_dn14 = assign75400_e115023_d_n14;
        locals.var_dps0_rv = 0.0;

        let (assign75410_e115027, assign75410_e115027_d_n0, assign75410_e115027_d_n2, assign75410_e115027_d_n4, assign75410_e115027_d_n5, assign75410_e115027_d_n6, assign75410_e115027_d_n7, assign75410_e115027_d_n8, assign75410_e115027_d_n9, assign75410_e115027_d_n10, assign75410_e115027_d_n11, assign75410_e115027_d_n14,) = {
    if (locals.var_flg_calcqover != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fs0_dps0, locals.var_fs0_dps0_dn0, locals.var_fs0_dps0_dn2, locals.var_fs0_dps0_dn4, locals.var_fs0_dps0_dn5, locals.var_fs0_dps0_dn6, locals.var_fs0_dps0_dn7, locals.var_fs0_dps0_dn8, locals.var_fs0_dps0_dn9, locals.var_fs0_dps0_dn10, locals.var_fs0_dps0_dn11, locals.var_fs0_dps0_dn14,)
    }
};
        locals.var_fs0_dps0 = assign75410_e115027;
        locals.var_fs0_dps0_dn0 = assign75410_e115027_d_n0;
        locals.var_fs0_dps0_dn2 = assign75410_e115027_d_n2;
        locals.var_fs0_dps0_dn4 = assign75410_e115027_d_n4;
        locals.var_fs0_dps0_dn5 = assign75410_e115027_d_n5;
        locals.var_fs0_dps0_dn6 = assign75410_e115027_d_n6;
        locals.var_fs0_dps0_dn7 = assign75410_e115027_d_n7;
        locals.var_fs0_dps0_dn8 = assign75410_e115027_d_n8;
        locals.var_fs0_dps0_dn9 = assign75410_e115027_d_n9;
        locals.var_fs0_dps0_dn10 = assign75410_e115027_d_n10;
        locals.var_fs0_dps0_dn11 = assign75410_e115027_d_n11;
        locals.var_fs0_dps0_dn14 = assign75410_e115027_d_n14;
        locals.var_fs0_dps0_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_285(
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
        locals: &mut StampLocals,
    ) {
        let (assign75420_e115031, assign75420_e115031_d_n0, assign75420_e115031_d_n2, assign75420_e115031_d_n4, assign75420_e115031_d_n5, assign75420_e115031_d_n6, assign75420_e115031_d_n7, assign75420_e115031_d_n8, assign75420_e115031_d_n9, assign75420_e115031_d_n10, assign75420_e115031_d_n11, assign75420_e115031_d_n14,) = {
    if (locals.var_flg_calcqover != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fs02_dps0, locals.var_fs02_dps0_dn0, locals.var_fs02_dps0_dn2, locals.var_fs02_dps0_dn4, locals.var_fs02_dps0_dn5, locals.var_fs02_dps0_dn6, locals.var_fs02_dps0_dn7, locals.var_fs02_dps0_dn8, locals.var_fs02_dps0_dn9, locals.var_fs02_dps0_dn10, locals.var_fs02_dps0_dn11, locals.var_fs02_dps0_dn14,)
    }
};
        locals.var_fs02_dps0 = assign75420_e115031;
        locals.var_fs02_dps0_dn0 = assign75420_e115031_d_n0;
        locals.var_fs02_dps0_dn2 = assign75420_e115031_d_n2;
        locals.var_fs02_dps0_dn4 = assign75420_e115031_d_n4;
        locals.var_fs02_dps0_dn5 = assign75420_e115031_d_n5;
        locals.var_fs02_dps0_dn6 = assign75420_e115031_d_n6;
        locals.var_fs02_dps0_dn7 = assign75420_e115031_d_n7;
        locals.var_fs02_dps0_dn8 = assign75420_e115031_d_n8;
        locals.var_fs02_dps0_dn9 = assign75420_e115031_d_n9;
        locals.var_fs02_dps0_dn10 = assign75420_e115031_d_n10;
        locals.var_fs02_dps0_dn11 = assign75420_e115031_d_n11;
        locals.var_fs02_dps0_dn14 = assign75420_e115031_d_n14;
        locals.var_fs02_dps0_rv = 0.0;

        let (assign75430_e115035, assign75430_e115035_d_n0, assign75430_e115035_d_n2, assign75430_e115035_d_n4, assign75430_e115035_d_n5, assign75430_e115035_d_n6, assign75430_e115035_d_n7, assign75430_e115035_d_n8, assign75430_e115035_d_n9, assign75430_e115035_d_n10, assign75430_e115035_d_n11, assign75430_e115035_d_n14,) = {
    if (locals.var_flg_calcqover != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fb_dpss, locals.var_fb_dpss_dn0, locals.var_fb_dpss_dn2, locals.var_fb_dpss_dn4, locals.var_fb_dpss_dn5, locals.var_fb_dpss_dn6, locals.var_fb_dpss_dn7, locals.var_fb_dpss_dn8, locals.var_fb_dpss_dn9, locals.var_fb_dpss_dn10, locals.var_fb_dpss_dn11, locals.var_fb_dpss_dn14,)
    }
};
        locals.var_fb_dpss = assign75430_e115035;
        locals.var_fb_dpss_dn0 = assign75430_e115035_d_n0;
        locals.var_fb_dpss_dn2 = assign75430_e115035_d_n2;
        locals.var_fb_dpss_dn4 = assign75430_e115035_d_n4;
        locals.var_fb_dpss_dn5 = assign75430_e115035_d_n5;
        locals.var_fb_dpss_dn6 = assign75430_e115035_d_n6;
        locals.var_fb_dpss_dn7 = assign75430_e115035_d_n7;
        locals.var_fb_dpss_dn8 = assign75430_e115035_d_n8;
        locals.var_fb_dpss_dn9 = assign75430_e115035_d_n9;
        locals.var_fb_dpss_dn10 = assign75430_e115035_d_n10;
        locals.var_fb_dpss_dn11 = assign75430_e115035_d_n11;
        locals.var_fb_dpss_dn14 = assign75430_e115035_d_n14;
        locals.var_fb_dpss_rv = 0.0;

        let (assign75440_e115039, assign75440_e115039_d_n0, assign75440_e115039_d_n2, assign75440_e115039_d_n4, assign75440_e115039_d_n5, assign75440_e115039_d_n6, assign75440_e115039_d_n7, assign75440_e115039_d_n8, assign75440_e115039_d_n9, assign75440_e115039_d_n10, assign75440_e115039_d_n11, assign75440_e115039_d_n14,) = {
    if (locals.var_flg_calcqover != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fs01_dps0, locals.var_fs01_dps0_dn0, locals.var_fs01_dps0_dn2, locals.var_fs01_dps0_dn4, locals.var_fs01_dps0_dn5, locals.var_fs01_dps0_dn6, locals.var_fs01_dps0_dn7, locals.var_fs01_dps0_dn8, locals.var_fs01_dps0_dn9, locals.var_fs01_dps0_dn10, locals.var_fs01_dps0_dn11, locals.var_fs01_dps0_dn14,)
    }
};
        locals.var_fs01_dps0 = assign75440_e115039;
        locals.var_fs01_dps0_dn0 = assign75440_e115039_d_n0;
        locals.var_fs01_dps0_dn2 = assign75440_e115039_d_n2;
        locals.var_fs01_dps0_dn4 = assign75440_e115039_d_n4;
        locals.var_fs01_dps0_dn5 = assign75440_e115039_d_n5;
        locals.var_fs01_dps0_dn6 = assign75440_e115039_d_n6;
        locals.var_fs01_dps0_dn7 = assign75440_e115039_d_n7;
        locals.var_fs01_dps0_dn8 = assign75440_e115039_d_n8;
        locals.var_fs01_dps0_dn9 = assign75440_e115039_d_n9;
        locals.var_fs01_dps0_dn10 = assign75440_e115039_d_n10;
        locals.var_fs01_dps0_dn11 = assign75440_e115039_d_n11;
        locals.var_fs01_dps0_dn14 = assign75440_e115039_d_n14;
        locals.var_fs01_dps0_rv = 0.0;

        let (assign75450_e115043, assign75450_e115043_d_n0, assign75450_e115043_d_n2, assign75450_e115043_d_n4, assign75450_e115043_d_n5, assign75450_e115043_d_n6, assign75450_e115043_d_n7, assign75450_e115043_d_n8, assign75450_e115043_d_n9, assign75450_e115043_d_n10, assign75450_e115043_d_n11, assign75450_e115043_d_n14,) = {
    if (locals.var_flg_calcqover != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_chi_1, locals.var_chi_1_dn0, locals.var_chi_1_dn2, locals.var_chi_1_dn4, locals.var_chi_1_dn5, locals.var_chi_1_dn6, locals.var_chi_1_dn7, locals.var_chi_1_dn8, locals.var_chi_1_dn9, locals.var_chi_1_dn10, locals.var_chi_1_dn11, locals.var_chi_1_dn14,)
    }
};
        locals.var_chi_1 = assign75450_e115043;
        locals.var_chi_1_dn0 = assign75450_e115043_d_n0;
        locals.var_chi_1_dn2 = assign75450_e115043_d_n2;
        locals.var_chi_1_dn4 = assign75450_e115043_d_n4;
        locals.var_chi_1_dn5 = assign75450_e115043_d_n5;
        locals.var_chi_1_dn6 = assign75450_e115043_d_n6;
        locals.var_chi_1_dn7 = assign75450_e115043_d_n7;
        locals.var_chi_1_dn8 = assign75450_e115043_d_n8;
        locals.var_chi_1_dn9 = assign75450_e115043_d_n9;
        locals.var_chi_1_dn10 = assign75450_e115043_d_n10;
        locals.var_chi_1_dn11 = assign75450_e115043_d_n11;
        locals.var_chi_1_dn14 = assign75450_e115043_d_n14;
        locals.var_chi_1_rv = 0.0;

        let (assign75460_e115047, assign75460_e115047_d_n0, assign75460_e115047_d_n2, assign75460_e115047_d_n4, assign75460_e115047_d_n5, assign75460_e115047_d_n6, assign75460_e115047_d_n7, assign75460_e115047_d_n8, assign75460_e115047_d_n9, assign75460_e115047_d_n10, assign75460_e115047_d_n11, assign75460_e115047_d_n14,) = {
    if (locals.var_flg_calcqover != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_chi_a, locals.var_chi_a_dn0, locals.var_chi_a_dn2, locals.var_chi_a_dn4, locals.var_chi_a_dn5, locals.var_chi_a_dn6, locals.var_chi_a_dn7, locals.var_chi_a_dn8, locals.var_chi_a_dn9, locals.var_chi_a_dn10, locals.var_chi_a_dn11, locals.var_chi_a_dn14,)
    }
};
        locals.var_chi_a = assign75460_e115047;
        locals.var_chi_a_dn0 = assign75460_e115047_d_n0;
        locals.var_chi_a_dn2 = assign75460_e115047_d_n2;
        locals.var_chi_a_dn4 = assign75460_e115047_d_n4;
        locals.var_chi_a_dn5 = assign75460_e115047_d_n5;
        locals.var_chi_a_dn6 = assign75460_e115047_d_n6;
        locals.var_chi_a_dn7 = assign75460_e115047_d_n7;
        locals.var_chi_a_dn8 = assign75460_e115047_d_n8;
        locals.var_chi_a_dn9 = assign75460_e115047_d_n9;
        locals.var_chi_a_dn10 = assign75460_e115047_d_n10;
        locals.var_chi_a_dn11 = assign75460_e115047_d_n11;
        locals.var_chi_a_dn14 = assign75460_e115047_d_n14;
        locals.var_chi_a_rv = 0.0;

        let (assign75470_e115051, assign75470_e115051_d_n0, assign75470_e115051_d_n2, assign75470_e115051_d_n4, assign75470_e115051_d_n5, assign75470_e115051_d_n6, assign75470_e115051_d_n7, assign75470_e115051_d_n8, assign75470_e115051_d_n9, assign75470_e115051_d_n10, assign75470_e115051_d_n11, assign75470_e115051_d_n14,) = {
    if (locals.var_flg_calcqover != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_chi_b, locals.var_chi_b_dn0, locals.var_chi_b_dn2, locals.var_chi_b_dn4, locals.var_chi_b_dn5, locals.var_chi_b_dn6, locals.var_chi_b_dn7, locals.var_chi_b_dn8, locals.var_chi_b_dn9, locals.var_chi_b_dn10, locals.var_chi_b_dn11, locals.var_chi_b_dn14,)
    }
};
        locals.var_chi_b = assign75470_e115051;
        locals.var_chi_b_dn0 = assign75470_e115051_d_n0;
        locals.var_chi_b_dn2 = assign75470_e115051_d_n2;
        locals.var_chi_b_dn4 = assign75470_e115051_d_n4;
        locals.var_chi_b_dn5 = assign75470_e115051_d_n5;
        locals.var_chi_b_dn6 = assign75470_e115051_d_n6;
        locals.var_chi_b_dn7 = assign75470_e115051_d_n7;
        locals.var_chi_b_dn8 = assign75470_e115051_d_n8;
        locals.var_chi_b_dn9 = assign75470_e115051_d_n9;
        locals.var_chi_b_dn10 = assign75470_e115051_d_n10;
        locals.var_chi_b_dn11 = assign75470_e115051_d_n11;
        locals.var_chi_b_dn14 = assign75470_e115051_d_n14;
        locals.var_chi_b_rv = 0.0;

        let (assign75480_e115056,) = {
    if (locals.var_flg_calcqover != 0.0) {
        let assign75480_e115054: f64 = (-1.0);
        (assign75480_e115054,)
    } else {
        (locals.var_flg_conv,)
    }
};
        locals.var_flg_conv = assign75480_e115056;
        locals.var_flg_conv_rv = 0.0;

        let (assign75490_e115060, assign75490_e115060_d_n0, assign75490_e115060_d_n2, assign75490_e115060_d_n4, assign75490_e115060_d_n5, assign75490_e115060_d_n6, assign75490_e115060_d_n7, assign75490_e115060_d_n8, assign75490_e115060_d_n9, assign75490_e115060_d_n10, assign75490_e115060_d_n11, assign75490_e115060_d_n14,) = {
    if (locals.var_flg_calcqover != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ps0ld_ini__blk1771, locals.var_ps0ld_ini__blk1771_dn0, locals.var_ps0ld_ini__blk1771_dn2, locals.var_ps0ld_ini__blk1771_dn4, locals.var_ps0ld_ini__blk1771_dn5, locals.var_ps0ld_ini__blk1771_dn6, locals.var_ps0ld_ini__blk1771_dn7, locals.var_ps0ld_ini__blk1771_dn8, locals.var_ps0ld_ini__blk1771_dn9, locals.var_ps0ld_ini__blk1771_dn10, locals.var_ps0ld_ini__blk1771_dn11, locals.var_ps0ld_ini__blk1771_dn14,)
    }
};
        locals.var_ps0ld_ini__blk1771 = assign75490_e115060;
        locals.var_ps0ld_ini__blk1771_dn0 = assign75490_e115060_d_n0;
        locals.var_ps0ld_ini__blk1771_dn2 = assign75490_e115060_d_n2;
        locals.var_ps0ld_ini__blk1771_dn4 = assign75490_e115060_d_n4;
        locals.var_ps0ld_ini__blk1771_dn5 = assign75490_e115060_d_n5;
        locals.var_ps0ld_ini__blk1771_dn6 = assign75490_e115060_d_n6;
        locals.var_ps0ld_ini__blk1771_dn7 = assign75490_e115060_d_n7;
        locals.var_ps0ld_ini__blk1771_dn8 = assign75490_e115060_d_n8;
        locals.var_ps0ld_ini__blk1771_dn9 = assign75490_e115060_d_n9;
        locals.var_ps0ld_ini__blk1771_dn10 = assign75490_e115060_d_n10;
        locals.var_ps0ld_ini__blk1771_dn11 = assign75490_e115060_d_n11;
        locals.var_ps0ld_ini__blk1771_dn14 = assign75490_e115060_d_n14;
        locals.var_ps0ld_ini__blk1771_rv = 0.0;

        let (assign75500_e115064, assign75500_e115064_d_n0, assign75500_e115064_d_n2, assign75500_e115064_d_n4, assign75500_e115064_d_n5, assign75500_e115064_d_n6, assign75500_e115064_d_n7, assign75500_e115064_d_n8, assign75500_e115064_d_n9, assign75500_e115064_d_n10, assign75500_e115064_d_n11, assign75500_e115064_d_n14,) = {
    if (locals.var_flg_calcqover != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fbsq__blk1772, locals.var_fbsq__blk1772_dn0, locals.var_fbsq__blk1772_dn2, locals.var_fbsq__blk1772_dn4, locals.var_fbsq__blk1772_dn5, locals.var_fbsq__blk1772_dn6, locals.var_fbsq__blk1772_dn7, locals.var_fbsq__blk1772_dn8, locals.var_fbsq__blk1772_dn9, locals.var_fbsq__blk1772_dn10, locals.var_fbsq__blk1772_dn11, locals.var_fbsq__blk1772_dn14,)
    }
};
        locals.var_fbsq__blk1772 = assign75500_e115064;
        locals.var_fbsq__blk1772_dn0 = assign75500_e115064_d_n0;
        locals.var_fbsq__blk1772_dn2 = assign75500_e115064_d_n2;
        locals.var_fbsq__blk1772_dn4 = assign75500_e115064_d_n4;
        locals.var_fbsq__blk1772_dn5 = assign75500_e115064_d_n5;
        locals.var_fbsq__blk1772_dn6 = assign75500_e115064_d_n6;
        locals.var_fbsq__blk1772_dn7 = assign75500_e115064_d_n7;
        locals.var_fbsq__blk1772_dn8 = assign75500_e115064_d_n8;
        locals.var_fbsq__blk1772_dn9 = assign75500_e115064_d_n9;
        locals.var_fbsq__blk1772_dn10 = assign75500_e115064_d_n10;
        locals.var_fbsq__blk1772_dn11 = assign75500_e115064_d_n11;
        locals.var_fbsq__blk1772_dn14 = assign75500_e115064_d_n14;
        locals.var_fbsq__blk1772_rv = 0.0;

        let (assign75510_e115075, assign75510_e115075_d_n0, assign75510_e115075_d_n2, assign75510_e115075_d_n4, assign75510_e115075_d_n5, assign75510_e115075_d_n6, assign75510_e115075_d_n7, assign75510_e115075_d_n8, assign75510_e115075_d_n9, assign75510_e115075_d_n10, assign75510_e115075_d_n11, assign75510_e115075_d_n14,) = {
    if (locals.var_flg_calcqover != 0.0) {
        let assign75510_e115068: f64 = (2.0 * locals.var_beta_inv);
        let assign75510_e115071: f64 = (locals.var_nover_func / locals.var_nin);
        let assign75510_e115072: f64 = (assign75510_e115071).ln();
        let assign75510_e115073: f64 = (assign75510_e115068 * assign75510_e115072);
        (assign75510_e115073, (((2.0 * locals.var_beta_inv_dn0) * assign75510_e115072) + (assign75510_e115068 * ((-((locals.var_nover_func * locals.var_nin_dn0) / (locals.var_nin * locals.var_nin))) / assign75510_e115071))), (((2.0 * locals.var_beta_inv_dn2) * assign75510_e115072) + (assign75510_e115068 * ((-((locals.var_nover_func * locals.var_nin_dn2) / (locals.var_nin * locals.var_nin))) / assign75510_e115071))), (((2.0 * locals.var_beta_inv_dn4) * assign75510_e115072) + (assign75510_e115068 * ((-((locals.var_nover_func * locals.var_nin_dn4) / (locals.var_nin * locals.var_nin))) / assign75510_e115071))), (((2.0 * locals.var_beta_inv_dn5) * assign75510_e115072) + (assign75510_e115068 * ((-((locals.var_nover_func * locals.var_nin_dn5) / (locals.var_nin * locals.var_nin))) / assign75510_e115071))), (((2.0 * locals.var_beta_inv_dn6) * assign75510_e115072) + (assign75510_e115068 * ((-((locals.var_nover_func * locals.var_nin_dn6) / (locals.var_nin * locals.var_nin))) / assign75510_e115071))), (((2.0 * locals.var_beta_inv_dn7) * assign75510_e115072) + (assign75510_e115068 * ((-((locals.var_nover_func * locals.var_nin_dn7) / (locals.var_nin * locals.var_nin))) / assign75510_e115071))), (((2.0 * locals.var_beta_inv_dn8) * assign75510_e115072) + (assign75510_e115068 * ((-((locals.var_nover_func * locals.var_nin_dn8) / (locals.var_nin * locals.var_nin))) / assign75510_e115071))), (((2.0 * locals.var_beta_inv_dn9) * assign75510_e115072) + (assign75510_e115068 * ((-((locals.var_nover_func * locals.var_nin_dn9) / (locals.var_nin * locals.var_nin))) / assign75510_e115071))), (((2.0 * locals.var_beta_inv_dn10) * assign75510_e115072) + (assign75510_e115068 * ((-((locals.var_nover_func * locals.var_nin_dn10) / (locals.var_nin * locals.var_nin))) / assign75510_e115071))), (((2.0 * locals.var_beta_inv_dn11) * assign75510_e115072) + (assign75510_e115068 * ((-((locals.var_nover_func * locals.var_nin_dn11) / (locals.var_nin * locals.var_nin))) / assign75510_e115071))), (((2.0 * locals.var_beta_inv_dn14) * assign75510_e115072) + (assign75510_e115068 * ((-((locals.var_nover_func * locals.var_nin_dn14) / (locals.var_nin * locals.var_nin))) / assign75510_e115071))),)
    } else {
        (locals.var_pb2over__blk1767, locals.var_pb2over__blk1767_dn0, locals.var_pb2over__blk1767_dn2, locals.var_pb2over__blk1767_dn4, locals.var_pb2over__blk1767_dn5, locals.var_pb2over__blk1767_dn6, locals.var_pb2over__blk1767_dn7, locals.var_pb2over__blk1767_dn8, locals.var_pb2over__blk1767_dn9, locals.var_pb2over__blk1767_dn10, locals.var_pb2over__blk1767_dn11, locals.var_pb2over__blk1767_dn14,)
    }
};
        locals.var_pb2over__blk1767 = assign75510_e115075;
        locals.var_pb2over__blk1767_dn0 = assign75510_e115075_d_n0;
        locals.var_pb2over__blk1767_dn2 = assign75510_e115075_d_n2;
        locals.var_pb2over__blk1767_dn4 = assign75510_e115075_d_n4;
        locals.var_pb2over__blk1767_dn5 = assign75510_e115075_d_n5;
        locals.var_pb2over__blk1767_dn6 = assign75510_e115075_d_n6;
        locals.var_pb2over__blk1767_dn7 = assign75510_e115075_d_n7;
        locals.var_pb2over__blk1767_dn8 = assign75510_e115075_d_n8;
        locals.var_pb2over__blk1767_dn9 = assign75510_e115075_d_n9;
        locals.var_pb2over__blk1767_dn10 = assign75510_e115075_d_n10;
        locals.var_pb2over__blk1767_dn11 = assign75510_e115075_d_n11;
        locals.var_pb2over__blk1767_dn14 = assign75510_e115075_d_n14;
        locals.var_pb2over__blk1767_rv = 0.0;

        let (assign75520_e115083, assign75520_e115083_d_n0, assign75520_e115083_d_n2, assign75520_e115083_d_n4, assign75520_e115083_d_n5, assign75520_e115083_d_n6, assign75520_e115083_d_n7, assign75520_e115083_d_n8, assign75520_e115083_d_n9, assign75520_e115083_d_n10, assign75520_e115083_d_n11, assign75520_e115083_d_n14,) = {
    if (locals.var_flg_calcqover != 0.0) {
        let assign75520_e115079: f64 = (0.8 - locals.var_pb2over__blk1767);
        let assign75520_e115081: f64 = (assign75520_e115079 - 0.1);
        (assign75520_e115081, (-locals.var_pb2over__blk1767_dn0), (-locals.var_pb2over__blk1767_dn2), (-locals.var_pb2over__blk1767_dn4), (-locals.var_pb2over__blk1767_dn5), (-locals.var_pb2over__blk1767_dn6), (-locals.var_pb2over__blk1767_dn7), (-locals.var_pb2over__blk1767_dn8), (-locals.var_pb2over__blk1767_dn9), (-locals.var_pb2over__blk1767_dn10), (-locals.var_pb2over__blk1767_dn11), (-locals.var_pb2over__blk1767_dn14),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign75520_e115083;
        locals.var_tmf1_dn0 = assign75520_e115083_d_n0;
        locals.var_tmf1_dn2 = assign75520_e115083_d_n2;
        locals.var_tmf1_dn4 = assign75520_e115083_d_n4;
        locals.var_tmf1_dn5 = assign75520_e115083_d_n5;
        locals.var_tmf1_dn6 = assign75520_e115083_d_n6;
        locals.var_tmf1_dn7 = assign75520_e115083_d_n7;
        locals.var_tmf1_dn8 = assign75520_e115083_d_n8;
        locals.var_tmf1_dn9 = assign75520_e115083_d_n9;
        locals.var_tmf1_dn10 = assign75520_e115083_d_n10;
        locals.var_tmf1_dn11 = assign75520_e115083_d_n11;
        locals.var_tmf1_dn14 = assign75520_e115083_d_n14;
        locals.var_tmf1_rv = 0.0;

        let (assign75530_e115091, assign75530_e115091_d_n0, assign75530_e115091_d_n2, assign75530_e115091_d_n4, assign75530_e115091_d_n5, assign75530_e115091_d_n6, assign75530_e115091_d_n7, assign75530_e115091_d_n8, assign75530_e115091_d_n9, assign75530_e115091_d_n10, assign75530_e115091_d_n11, assign75530_e115091_d_n14,) = {
    if (locals.var_flg_calcqover != 0.0) {
        let assign75530_e115087: f64 = (4.0 * 0.8);
        let assign75530_e115089: f64 = (assign75530_e115087 * 0.1);
        (assign75530_e115089, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign75530_e115091;
        locals.var_tmf2_dn0 = assign75530_e115091_d_n0;
        locals.var_tmf2_dn2 = assign75530_e115091_d_n2;
        locals.var_tmf2_dn4 = assign75530_e115091_d_n4;
        locals.var_tmf2_dn5 = assign75530_e115091_d_n5;
        locals.var_tmf2_dn6 = assign75530_e115091_d_n6;
        locals.var_tmf2_dn7 = assign75530_e115091_d_n7;
        locals.var_tmf2_dn8 = assign75530_e115091_d_n8;
        locals.var_tmf2_dn9 = assign75530_e115091_d_n9;
        locals.var_tmf2_dn10 = assign75530_e115091_d_n10;
        locals.var_tmf2_dn11 = assign75530_e115091_d_n11;
        locals.var_tmf2_dn14 = assign75530_e115091_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign75540_e115101, assign75540_e115101_d_n0, assign75540_e115101_d_n2, assign75540_e115101_d_n4, assign75540_e115101_d_n5, assign75540_e115101_d_n6, assign75540_e115101_d_n7, assign75540_e115101_d_n8, assign75540_e115101_d_n9, assign75540_e115101_d_n10, assign75540_e115101_d_n11, assign75540_e115101_d_n14,) = {
    if (locals.var_flg_calcqover != 0.0) {
        let (assign75540_e115099, assign75540_e115099_d_n0, assign75540_e115099_d_n2, assign75540_e115099_d_n4, assign75540_e115099_d_n5, assign75540_e115099_d_n6, assign75540_e115099_d_n7, assign75540_e115099_d_n8, assign75540_e115099_d_n9, assign75540_e115099_d_n10, assign75540_e115099_d_n11, assign75540_e115099_d_n14,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
            } else {
                let assign75540_e115098: f64 = (-locals.var_tmf2);
                (assign75540_e115098, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn14),)
            }
        };
        (assign75540_e115099, assign75540_e115099_d_n0, assign75540_e115099_d_n2, assign75540_e115099_d_n4, assign75540_e115099_d_n5, assign75540_e115099_d_n6, assign75540_e115099_d_n7, assign75540_e115099_d_n8, assign75540_e115099_d_n9, assign75540_e115099_d_n10, assign75540_e115099_d_n11, assign75540_e115099_d_n14,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign75540_e115101;
        locals.var_tmf2_dn0 = assign75540_e115101_d_n0;
        locals.var_tmf2_dn2 = assign75540_e115101_d_n2;
        locals.var_tmf2_dn4 = assign75540_e115101_d_n4;
        locals.var_tmf2_dn5 = assign75540_e115101_d_n5;
        locals.var_tmf2_dn6 = assign75540_e115101_d_n6;
        locals.var_tmf2_dn7 = assign75540_e115101_d_n7;
        locals.var_tmf2_dn8 = assign75540_e115101_d_n8;
        locals.var_tmf2_dn9 = assign75540_e115101_d_n9;
        locals.var_tmf2_dn10 = assign75540_e115101_d_n10;
        locals.var_tmf2_dn11 = assign75540_e115101_d_n11;
        locals.var_tmf2_dn14 = assign75540_e115101_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign75550_e115110, assign75550_e115110_d_n0, assign75550_e115110_d_n2, assign75550_e115110_d_n4, assign75550_e115110_d_n5, assign75550_e115110_d_n6, assign75550_e115110_d_n7, assign75550_e115110_d_n8, assign75550_e115110_d_n9, assign75550_e115110_d_n10, assign75550_e115110_d_n11, assign75550_e115110_d_n14,) = {
    if (locals.var_flg_calcqover != 0.0) {
        let assign75550_e115105: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign75550_e115107: f64 = (assign75550_e115105 + locals.var_tmf2);
        let assign75550_e115108: f64 = (assign75550_e115107).sqrt();
        (assign75550_e115108, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign75550_e115108)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign75550_e115108)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign75550_e115108)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign75550_e115108)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign75550_e115108)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign75550_e115108)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign75550_e115108)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign75550_e115108)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign75550_e115108)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign75550_e115108)), ((((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)) + locals.var_tmf2_dn14) / (2.0 * assign75550_e115108)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign75550_e115110;
        locals.var_tmf2_dn0 = assign75550_e115110_d_n0;
        locals.var_tmf2_dn2 = assign75550_e115110_d_n2;
        locals.var_tmf2_dn4 = assign75550_e115110_d_n4;
        locals.var_tmf2_dn5 = assign75550_e115110_d_n5;
        locals.var_tmf2_dn6 = assign75550_e115110_d_n6;
        locals.var_tmf2_dn7 = assign75550_e115110_d_n7;
        locals.var_tmf2_dn8 = assign75550_e115110_d_n8;
        locals.var_tmf2_dn9 = assign75550_e115110_d_n9;
        locals.var_tmf2_dn10 = assign75550_e115110_d_n10;
        locals.var_tmf2_dn11 = assign75550_e115110_d_n11;
        locals.var_tmf2_dn14 = assign75550_e115110_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign75560_e115120, assign75560_e115120_d_n0, assign75560_e115120_d_n2, assign75560_e115120_d_n4, assign75560_e115120_d_n5, assign75560_e115120_d_n6, assign75560_e115120_d_n7, assign75560_e115120_d_n8, assign75560_e115120_d_n9, assign75560_e115120_d_n10, assign75560_e115120_d_n11, assign75560_e115120_d_n14,) = {
    if (locals.var_flg_calcqover != 0.0) {
        let assign75560_e115116: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign75560_e115117: f64 = (1.0 + assign75560_e115116);
        let assign75560_e115118: f64 = (0.5 * assign75560_e115117);
        (assign75560_e115118, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn11 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn14 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign75560_e115120;
        locals.var_t0_dn0 = assign75560_e115120_d_n0;
        locals.var_t0_dn2 = assign75560_e115120_d_n2;
        locals.var_t0_dn4 = assign75560_e115120_d_n4;
        locals.var_t0_dn5 = assign75560_e115120_d_n5;
        locals.var_t0_dn6 = assign75560_e115120_d_n6;
        locals.var_t0_dn7 = assign75560_e115120_d_n7;
        locals.var_t0_dn8 = assign75560_e115120_d_n8;
        locals.var_t0_dn9 = assign75560_e115120_d_n9;
        locals.var_t0_dn10 = assign75560_e115120_d_n10;
        locals.var_t0_dn11 = assign75560_e115120_d_n11;
        locals.var_t0_dn14 = assign75560_e115120_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign75570_e115130, assign75570_e115130_d_n0, assign75570_e115130_d_n2, assign75570_e115130_d_n4, assign75570_e115130_d_n5, assign75570_e115130_d_n6, assign75570_e115130_d_n7, assign75570_e115130_d_n8, assign75570_e115130_d_n9, assign75570_e115130_d_n10, assign75570_e115130_d_n11, assign75570_e115130_d_n14,) = {
    if (locals.var_flg_calcqover != 0.0) {
        let assign75570_e115126: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign75570_e115127: f64 = (0.5 * assign75570_e115126);
        let assign75570_e115128: f64 = (0.8 - assign75570_e115127);
        (assign75570_e115128, (-(0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (-(0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), (-(0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4))), (-(0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5))), (-(0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), (-(0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))), (-(0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8))), (-(0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9))), (-(0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), (-(0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11))), (-(0.5 * (locals.var_tmf1_dn14 + locals.var_tmf2_dn14))),)
    } else {
        (locals.var_vbs_max_over__blk1768, locals.var_vbs_max_over__blk1768_dn0, locals.var_vbs_max_over__blk1768_dn2, locals.var_vbs_max_over__blk1768_dn4, locals.var_vbs_max_over__blk1768_dn5, locals.var_vbs_max_over__blk1768_dn6, locals.var_vbs_max_over__blk1768_dn7, locals.var_vbs_max_over__blk1768_dn8, locals.var_vbs_max_over__blk1768_dn9, locals.var_vbs_max_over__blk1768_dn10, locals.var_vbs_max_over__blk1768_dn11, locals.var_vbs_max_over__blk1768_dn14,)
    }
};
        locals.var_vbs_max_over__blk1768 = assign75570_e115130;
        locals.var_vbs_max_over__blk1768_dn0 = assign75570_e115130_d_n0;
        locals.var_vbs_max_over__blk1768_dn2 = assign75570_e115130_d_n2;
        locals.var_vbs_max_over__blk1768_dn4 = assign75570_e115130_d_n4;
        locals.var_vbs_max_over__blk1768_dn5 = assign75570_e115130_d_n5;
        locals.var_vbs_max_over__blk1768_dn6 = assign75570_e115130_d_n6;
        locals.var_vbs_max_over__blk1768_dn7 = assign75570_e115130_d_n7;
        locals.var_vbs_max_over__blk1768_dn8 = assign75570_e115130_d_n8;
        locals.var_vbs_max_over__blk1768_dn9 = assign75570_e115130_d_n9;
        locals.var_vbs_max_over__blk1768_dn10 = assign75570_e115130_d_n10;
        locals.var_vbs_max_over__blk1768_dn11 = assign75570_e115130_d_n11;
        locals.var_vbs_max_over__blk1768_dn14 = assign75570_e115130_d_n14;
        locals.var_vbs_max_over__blk1768_rv = 0.0;

        let assign75580_e115134: f64 = (locals.var_vbs_max_over__blk1768 * 0.5);
        let assign75580_e115135: f64 = if locals.var_vbs_bnd_over__blk1769 > assign75580_e115134 { 1.0 } else { 0.0 };
        locals.var_guard1774 = assign75580_e115135;
        locals.var_guard1774_rv = 0.0;

        let (assign75590_e115143, assign75590_e115143_d_n0, assign75590_e115143_d_n2, assign75590_e115143_d_n4, assign75590_e115143_d_n5, assign75590_e115143_d_n6, assign75590_e115143_d_n7, assign75590_e115143_d_n8, assign75590_e115143_d_n9, assign75590_e115143_d_n10, assign75590_e115143_d_n11, assign75590_e115143_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1774 != 0.0)) {
        let assign75590_e115141: f64 = (0.5 * locals.var_vbs_max_over__blk1768);
        (assign75590_e115141, (0.5 * locals.var_vbs_max_over__blk1768_dn0), (0.5 * locals.var_vbs_max_over__blk1768_dn2), (0.5 * locals.var_vbs_max_over__blk1768_dn4), (0.5 * locals.var_vbs_max_over__blk1768_dn5), (0.5 * locals.var_vbs_max_over__blk1768_dn6), (0.5 * locals.var_vbs_max_over__blk1768_dn7), (0.5 * locals.var_vbs_max_over__blk1768_dn8), (0.5 * locals.var_vbs_max_over__blk1768_dn9), (0.5 * locals.var_vbs_max_over__blk1768_dn10), (0.5 * locals.var_vbs_max_over__blk1768_dn11), (0.5 * locals.var_vbs_max_over__blk1768_dn14),)
    } else {
        (locals.var_vbs_bnd_over__blk1769, locals.var_vbs_bnd_over__blk1769_dn0, locals.var_vbs_bnd_over__blk1769_dn2, locals.var_vbs_bnd_over__blk1769_dn4, locals.var_vbs_bnd_over__blk1769_dn5, locals.var_vbs_bnd_over__blk1769_dn6, locals.var_vbs_bnd_over__blk1769_dn7, locals.var_vbs_bnd_over__blk1769_dn8, locals.var_vbs_bnd_over__blk1769_dn9, locals.var_vbs_bnd_over__blk1769_dn10, locals.var_vbs_bnd_over__blk1769_dn11, locals.var_vbs_bnd_over__blk1769_dn14,)
    }
};
        locals.var_vbs_bnd_over__blk1769 = assign75590_e115143;
        locals.var_vbs_bnd_over__blk1769_dn0 = assign75590_e115143_d_n0;
        locals.var_vbs_bnd_over__blk1769_dn2 = assign75590_e115143_d_n2;
        locals.var_vbs_bnd_over__blk1769_dn4 = assign75590_e115143_d_n4;
        locals.var_vbs_bnd_over__blk1769_dn5 = assign75590_e115143_d_n5;
        locals.var_vbs_bnd_over__blk1769_dn6 = assign75590_e115143_d_n6;
        locals.var_vbs_bnd_over__blk1769_dn7 = assign75590_e115143_d_n7;
        locals.var_vbs_bnd_over__blk1769_dn8 = assign75590_e115143_d_n8;
        locals.var_vbs_bnd_over__blk1769_dn9 = assign75590_e115143_d_n9;
        locals.var_vbs_bnd_over__blk1769_dn10 = assign75590_e115143_d_n10;
        locals.var_vbs_bnd_over__blk1769_dn11 = assign75590_e115143_d_n11;
        locals.var_vbs_bnd_over__blk1769_dn14 = assign75590_e115143_d_n14;
        locals.var_vbs_bnd_over__blk1769_rv = 0.0;

        let assign75600_e115145: f64 = if param_given[338] { 1.0 } else { 0.0 };
        locals.var_guard1775 = assign75600_e115145;
        locals.var_guard1775_rv = 0.0;

        let (assign75610_e115151, assign75610_e115151_d_n0, assign75610_e115151_d_n2, assign75610_e115151_d_n4, assign75610_e115151_d_n5, assign75610_e115151_d_n6, assign75610_e115151_d_n7, assign75610_e115151_d_n8, assign75610_e115151_d_n9, assign75610_e115151_d_n10, assign75610_e115151_d_n11, assign75610_e115151_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1775 != 0.0)) {
        (p.p338, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vbs_max_over__blk1768, locals.var_vbs_max_over__blk1768_dn0, locals.var_vbs_max_over__blk1768_dn2, locals.var_vbs_max_over__blk1768_dn4, locals.var_vbs_max_over__blk1768_dn5, locals.var_vbs_max_over__blk1768_dn6, locals.var_vbs_max_over__blk1768_dn7, locals.var_vbs_max_over__blk1768_dn8, locals.var_vbs_max_over__blk1768_dn9, locals.var_vbs_max_over__blk1768_dn10, locals.var_vbs_max_over__blk1768_dn11, locals.var_vbs_max_over__blk1768_dn14,)
    }
};
        locals.var_vbs_max_over__blk1768 = assign75610_e115151;
        locals.var_vbs_max_over__blk1768_dn0 = assign75610_e115151_d_n0;
        locals.var_vbs_max_over__blk1768_dn2 = assign75610_e115151_d_n2;
        locals.var_vbs_max_over__blk1768_dn4 = assign75610_e115151_d_n4;
        locals.var_vbs_max_over__blk1768_dn5 = assign75610_e115151_d_n5;
        locals.var_vbs_max_over__blk1768_dn6 = assign75610_e115151_d_n6;
        locals.var_vbs_max_over__blk1768_dn7 = assign75610_e115151_d_n7;
        locals.var_vbs_max_over__blk1768_dn8 = assign75610_e115151_d_n8;
        locals.var_vbs_max_over__blk1768_dn9 = assign75610_e115151_d_n9;
        locals.var_vbs_max_over__blk1768_dn10 = assign75610_e115151_d_n10;
        locals.var_vbs_max_over__blk1768_dn11 = assign75610_e115151_d_n11;
        locals.var_vbs_max_over__blk1768_dn14 = assign75610_e115151_d_n14;
        locals.var_vbs_max_over__blk1768_rv = 0.0;

        let assign75620_e115153: f64 = if param_given[339] { 1.0 } else { 0.0 };
        locals.var_guard1776 = assign75620_e115153;
        locals.var_guard1776_rv = 0.0;

        let (assign75630_e115159, assign75630_e115159_d_n0, assign75630_e115159_d_n2, assign75630_e115159_d_n4, assign75630_e115159_d_n5, assign75630_e115159_d_n6, assign75630_e115159_d_n7, assign75630_e115159_d_n8, assign75630_e115159_d_n9, assign75630_e115159_d_n10, assign75630_e115159_d_n11, assign75630_e115159_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1776 != 0.0)) {
        (p.p339, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vbs_bnd_over__blk1769, locals.var_vbs_bnd_over__blk1769_dn0, locals.var_vbs_bnd_over__blk1769_dn2, locals.var_vbs_bnd_over__blk1769_dn4, locals.var_vbs_bnd_over__blk1769_dn5, locals.var_vbs_bnd_over__blk1769_dn6, locals.var_vbs_bnd_over__blk1769_dn7, locals.var_vbs_bnd_over__blk1769_dn8, locals.var_vbs_bnd_over__blk1769_dn9, locals.var_vbs_bnd_over__blk1769_dn10, locals.var_vbs_bnd_over__blk1769_dn11, locals.var_vbs_bnd_over__blk1769_dn14,)
    }
};
        locals.var_vbs_bnd_over__blk1769 = assign75630_e115159;
        locals.var_vbs_bnd_over__blk1769_dn0 = assign75630_e115159_d_n0;
        locals.var_vbs_bnd_over__blk1769_dn2 = assign75630_e115159_d_n2;
        locals.var_vbs_bnd_over__blk1769_dn4 = assign75630_e115159_d_n4;
        locals.var_vbs_bnd_over__blk1769_dn5 = assign75630_e115159_d_n5;
        locals.var_vbs_bnd_over__blk1769_dn6 = assign75630_e115159_d_n6;
        locals.var_vbs_bnd_over__blk1769_dn7 = assign75630_e115159_d_n7;
        locals.var_vbs_bnd_over__blk1769_dn8 = assign75630_e115159_d_n8;
        locals.var_vbs_bnd_over__blk1769_dn9 = assign75630_e115159_d_n9;
        locals.var_vbs_bnd_over__blk1769_dn10 = assign75630_e115159_d_n10;
        locals.var_vbs_bnd_over__blk1769_dn11 = assign75630_e115159_d_n11;
        locals.var_vbs_bnd_over__blk1769_dn14 = assign75630_e115159_d_n14;
        locals.var_vbs_bnd_over__blk1769_rv = 0.0;

        let assign75640_e115161: f64 = if param_given[338] { 1.0 } else { 0.0 };
        locals.var_guard1777 = assign75640_e115161;
        locals.var_guard1777_rv = 0.0;

        let (assign75650_e115172, assign75650_e115172_d_n0, assign75650_e115172_d_n2, assign75650_e115172_d_n4, assign75650_e115172_d_n5, assign75650_e115172_d_n6, assign75650_e115172_d_n7, assign75650_e115172_d_n8, assign75650_e115172_d_n9, assign75650_e115172_d_n10, assign75650_e115172_d_n11, assign75650_e115172_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1776 == 0.0)) && (locals.var_guard1777 != 0.0)) {
        let assign75650_e115170: f64 = (0.5 * locals.var_vbs_max_over__blk1768);
        (assign75650_e115170, (0.5 * locals.var_vbs_max_over__blk1768_dn0), (0.5 * locals.var_vbs_max_over__blk1768_dn2), (0.5 * locals.var_vbs_max_over__blk1768_dn4), (0.5 * locals.var_vbs_max_over__blk1768_dn5), (0.5 * locals.var_vbs_max_over__blk1768_dn6), (0.5 * locals.var_vbs_max_over__blk1768_dn7), (0.5 * locals.var_vbs_max_over__blk1768_dn8), (0.5 * locals.var_vbs_max_over__blk1768_dn9), (0.5 * locals.var_vbs_max_over__blk1768_dn10), (0.5 * locals.var_vbs_max_over__blk1768_dn11), (0.5 * locals.var_vbs_max_over__blk1768_dn14),)
    } else {
        (locals.var_vbs_bnd_over__blk1769, locals.var_vbs_bnd_over__blk1769_dn0, locals.var_vbs_bnd_over__blk1769_dn2, locals.var_vbs_bnd_over__blk1769_dn4, locals.var_vbs_bnd_over__blk1769_dn5, locals.var_vbs_bnd_over__blk1769_dn6, locals.var_vbs_bnd_over__blk1769_dn7, locals.var_vbs_bnd_over__blk1769_dn8, locals.var_vbs_bnd_over__blk1769_dn9, locals.var_vbs_bnd_over__blk1769_dn10, locals.var_vbs_bnd_over__blk1769_dn11, locals.var_vbs_bnd_over__blk1769_dn14,)
    }
};
        locals.var_vbs_bnd_over__blk1769 = assign75650_e115172;
        locals.var_vbs_bnd_over__blk1769_dn0 = assign75650_e115172_d_n0;
        locals.var_vbs_bnd_over__blk1769_dn2 = assign75650_e115172_d_n2;
        locals.var_vbs_bnd_over__blk1769_dn4 = assign75650_e115172_d_n4;
        locals.var_vbs_bnd_over__blk1769_dn5 = assign75650_e115172_d_n5;
        locals.var_vbs_bnd_over__blk1769_dn6 = assign75650_e115172_d_n6;
        locals.var_vbs_bnd_over__blk1769_dn7 = assign75650_e115172_d_n7;
        locals.var_vbs_bnd_over__blk1769_dn8 = assign75650_e115172_d_n8;
        locals.var_vbs_bnd_over__blk1769_dn9 = assign75650_e115172_d_n9;
        locals.var_vbs_bnd_over__blk1769_dn10 = assign75650_e115172_d_n10;
        locals.var_vbs_bnd_over__blk1769_dn11 = assign75650_e115172_d_n11;
        locals.var_vbs_bnd_over__blk1769_dn14 = assign75650_e115172_d_n14;
        locals.var_vbs_bnd_over__blk1769_rv = 0.0;

        let assign75660_e115176: f64 = (locals.var_vbs_max_over__blk1768 * 0.5);
        let assign75660_e115177: f64 = if locals.var_vbs_bnd_over__blk1769 > assign75660_e115176 { 1.0 } else { 0.0 };
        locals.var_guard1778 = assign75660_e115177;
        locals.var_guard1778_rv = 0.0;

        let (assign75670_e115185, assign75670_e115185_d_n0, assign75670_e115185_d_n2, assign75670_e115185_d_n4, assign75670_e115185_d_n5, assign75670_e115185_d_n6, assign75670_e115185_d_n7, assign75670_e115185_d_n8, assign75670_e115185_d_n9, assign75670_e115185_d_n10, assign75670_e115185_d_n11, assign75670_e115185_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1778 != 0.0)) {
        let assign75670_e115183: f64 = (0.5 * locals.var_vbs_max_over__blk1768);
        (assign75670_e115183, (0.5 * locals.var_vbs_max_over__blk1768_dn0), (0.5 * locals.var_vbs_max_over__blk1768_dn2), (0.5 * locals.var_vbs_max_over__blk1768_dn4), (0.5 * locals.var_vbs_max_over__blk1768_dn5), (0.5 * locals.var_vbs_max_over__blk1768_dn6), (0.5 * locals.var_vbs_max_over__blk1768_dn7), (0.5 * locals.var_vbs_max_over__blk1768_dn8), (0.5 * locals.var_vbs_max_over__blk1768_dn9), (0.5 * locals.var_vbs_max_over__blk1768_dn10), (0.5 * locals.var_vbs_max_over__blk1768_dn11), (0.5 * locals.var_vbs_max_over__blk1768_dn14),)
    } else {
        (locals.var_vbs_bnd_over__blk1769, locals.var_vbs_bnd_over__blk1769_dn0, locals.var_vbs_bnd_over__blk1769_dn2, locals.var_vbs_bnd_over__blk1769_dn4, locals.var_vbs_bnd_over__blk1769_dn5, locals.var_vbs_bnd_over__blk1769_dn6, locals.var_vbs_bnd_over__blk1769_dn7, locals.var_vbs_bnd_over__blk1769_dn8, locals.var_vbs_bnd_over__blk1769_dn9, locals.var_vbs_bnd_over__blk1769_dn10, locals.var_vbs_bnd_over__blk1769_dn11, locals.var_vbs_bnd_over__blk1769_dn14,)
    }
};
        locals.var_vbs_bnd_over__blk1769 = assign75670_e115185;
        locals.var_vbs_bnd_over__blk1769_dn0 = assign75670_e115185_d_n0;
        locals.var_vbs_bnd_over__blk1769_dn2 = assign75670_e115185_d_n2;
        locals.var_vbs_bnd_over__blk1769_dn4 = assign75670_e115185_d_n4;
        locals.var_vbs_bnd_over__blk1769_dn5 = assign75670_e115185_d_n5;
        locals.var_vbs_bnd_over__blk1769_dn6 = assign75670_e115185_d_n6;
        locals.var_vbs_bnd_over__blk1769_dn7 = assign75670_e115185_d_n7;
        locals.var_vbs_bnd_over__blk1769_dn8 = assign75670_e115185_d_n8;
        locals.var_vbs_bnd_over__blk1769_dn9 = assign75670_e115185_d_n9;
        locals.var_vbs_bnd_over__blk1769_dn10 = assign75670_e115185_d_n10;
        locals.var_vbs_bnd_over__blk1769_dn11 = assign75670_e115185_d_n11;
        locals.var_vbs_bnd_over__blk1769_dn14 = assign75670_e115185_d_n14;
        locals.var_vbs_bnd_over__blk1769_rv = 0.0;

        let assign75680_e115188: f64 = if p.p38 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1779 = assign75680_e115188;
        locals.var_guard1779_rv = 0.0;

        let (assign75690_e115195, assign75690_e115195_d_n0, assign75690_e115195_d_n2, assign75690_e115195_d_n4, assign75690_e115195_d_n5, assign75690_e115195_d_n6, assign75690_e115195_d_n7, assign75690_e115195_d_n8, assign75690_e115195_d_n9, assign75690_e115195_d_n10, assign75690_e115195_d_n11, assign75690_e115195_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1779 != 0.0)) {
        let assign75690_e115193: f64 = (-locals.var_vxbgmt);
        (assign75690_e115193, (-locals.var_vxbgmt_dn0), (-locals.var_vxbgmt_dn2), (-locals.var_vxbgmt_dn4), (-locals.var_vxbgmt_dn5), (-locals.var_vxbgmt_dn6), (-locals.var_vxbgmt_dn7), (-locals.var_vxbgmt_dn8), (-locals.var_vxbgmt_dn9), (-locals.var_vxbgmt_dn10), (-locals.var_vxbgmt_dn11), (-locals.var_vxbgmt_dn14),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign75690_e115195;
        locals.var_t0_dn0 = assign75690_e115195_d_n0;
        locals.var_t0_dn2 = assign75690_e115195_d_n2;
        locals.var_t0_dn4 = assign75690_e115195_d_n4;
        locals.var_t0_dn5 = assign75690_e115195_d_n5;
        locals.var_t0_dn6 = assign75690_e115195_d_n6;
        locals.var_t0_dn7 = assign75690_e115195_d_n7;
        locals.var_t0_dn8 = assign75690_e115195_d_n8;
        locals.var_t0_dn9 = assign75690_e115195_d_n9;
        locals.var_t0_dn10 = assign75690_e115195_d_n10;
        locals.var_t0_dn11 = assign75690_e115195_d_n11;
        locals.var_t0_dn14 = assign75690_e115195_d_n14;
        locals.var_t0_rv = 0.0;

        let assign75700_e115198: f64 = if locals.var_t0 > locals.var_vbs_bnd_over__blk1769 { 1.0 } else { 0.0 };
        locals.var_guard1780 = assign75700_e115198;
        locals.var_guard1780_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_286(
        locals: &mut StampLocals,
    ) {
        let (assign75710_e115208, assign75710_e115208_d_n0, assign75710_e115208_d_n2, assign75710_e115208_d_n4, assign75710_e115208_d_n5, assign75710_e115208_d_n6, assign75710_e115208_d_n7, assign75710_e115208_d_n8, assign75710_e115208_d_n9, assign75710_e115208_d_n10, assign75710_e115208_d_n11, assign75710_e115208_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1779 != 0.0)) && (locals.var_guard1780 != 0.0)) {
        let assign75710_e115206: f64 = (locals.var_t0 - locals.var_vbs_bnd_over__blk1769);
        (assign75710_e115206, (locals.var_t0_dn0 - locals.var_vbs_bnd_over__blk1769_dn0), (locals.var_t0_dn2 - locals.var_vbs_bnd_over__blk1769_dn2), (locals.var_t0_dn4 - locals.var_vbs_bnd_over__blk1769_dn4), (locals.var_t0_dn5 - locals.var_vbs_bnd_over__blk1769_dn5), (locals.var_t0_dn6 - locals.var_vbs_bnd_over__blk1769_dn6), (locals.var_t0_dn7 - locals.var_vbs_bnd_over__blk1769_dn7), (locals.var_t0_dn8 - locals.var_vbs_bnd_over__blk1769_dn8), (locals.var_t0_dn9 - locals.var_vbs_bnd_over__blk1769_dn9), (locals.var_t0_dn10 - locals.var_vbs_bnd_over__blk1769_dn10), (locals.var_t0_dn11 - locals.var_vbs_bnd_over__blk1769_dn11), (locals.var_t0_dn14 - locals.var_vbs_bnd_over__blk1769_dn14),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign75710_e115208;
        locals.var_t1_dn0 = assign75710_e115208_d_n0;
        locals.var_t1_dn2 = assign75710_e115208_d_n2;
        locals.var_t1_dn4 = assign75710_e115208_d_n4;
        locals.var_t1_dn5 = assign75710_e115208_d_n5;
        locals.var_t1_dn6 = assign75710_e115208_d_n6;
        locals.var_t1_dn7 = assign75710_e115208_d_n7;
        locals.var_t1_dn8 = assign75710_e115208_d_n8;
        locals.var_t1_dn9 = assign75710_e115208_d_n9;
        locals.var_t1_dn10 = assign75710_e115208_d_n10;
        locals.var_t1_dn11 = assign75710_e115208_d_n11;
        locals.var_t1_dn14 = assign75710_e115208_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign75720_e115218, assign75720_e115218_d_n0, assign75720_e115218_d_n2, assign75720_e115218_d_n4, assign75720_e115218_d_n5, assign75720_e115218_d_n6, assign75720_e115218_d_n7, assign75720_e115218_d_n8, assign75720_e115218_d_n9, assign75720_e115218_d_n10, assign75720_e115218_d_n11, assign75720_e115218_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1779 != 0.0)) && (locals.var_guard1780 != 0.0)) {
        let assign75720_e115216: f64 = (locals.var_vbs_max_over__blk1768 - locals.var_vbs_bnd_over__blk1769);
        (assign75720_e115216, (locals.var_vbs_max_over__blk1768_dn0 - locals.var_vbs_bnd_over__blk1769_dn0), (locals.var_vbs_max_over__blk1768_dn2 - locals.var_vbs_bnd_over__blk1769_dn2), (locals.var_vbs_max_over__blk1768_dn4 - locals.var_vbs_bnd_over__blk1769_dn4), (locals.var_vbs_max_over__blk1768_dn5 - locals.var_vbs_bnd_over__blk1769_dn5), (locals.var_vbs_max_over__blk1768_dn6 - locals.var_vbs_bnd_over__blk1769_dn6), (locals.var_vbs_max_over__blk1768_dn7 - locals.var_vbs_bnd_over__blk1769_dn7), (locals.var_vbs_max_over__blk1768_dn8 - locals.var_vbs_bnd_over__blk1769_dn8), (locals.var_vbs_max_over__blk1768_dn9 - locals.var_vbs_bnd_over__blk1769_dn9), (locals.var_vbs_max_over__blk1768_dn10 - locals.var_vbs_bnd_over__blk1769_dn10), (locals.var_vbs_max_over__blk1768_dn11 - locals.var_vbs_bnd_over__blk1769_dn11), (locals.var_vbs_max_over__blk1768_dn14 - locals.var_vbs_bnd_over__blk1769_dn14),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign75720_e115218;
        locals.var_t2_dn0 = assign75720_e115218_d_n0;
        locals.var_t2_dn2 = assign75720_e115218_d_n2;
        locals.var_t2_dn4 = assign75720_e115218_d_n4;
        locals.var_t2_dn5 = assign75720_e115218_d_n5;
        locals.var_t2_dn6 = assign75720_e115218_d_n6;
        locals.var_t2_dn7 = assign75720_e115218_d_n7;
        locals.var_t2_dn8 = assign75720_e115218_d_n8;
        locals.var_t2_dn9 = assign75720_e115218_d_n9;
        locals.var_t2_dn10 = assign75720_e115218_d_n10;
        locals.var_t2_dn11 = assign75720_e115218_d_n11;
        locals.var_t2_dn14 = assign75720_e115218_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign75730_e115228, assign75730_e115228_d_n0, assign75730_e115228_d_n2, assign75730_e115228_d_n4, assign75730_e115228_d_n5, assign75730_e115228_d_n6, assign75730_e115228_d_n7, assign75730_e115228_d_n8, assign75730_e115228_d_n9, assign75730_e115228_d_n10, assign75730_e115228_d_n11, assign75730_e115228_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1779 != 0.0)) && (locals.var_guard1780 != 0.0)) {
        let assign75730_e115226: f64 = (locals.var_t1 / locals.var_t2);
        (assign75730_e115226, (((locals.var_t1_dn0 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn0)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn2 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn2)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn4 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn4)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn5 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn5)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn6 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn6)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn7 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn7)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn8 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn8)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn9 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn9)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn10 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn10)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn11 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn11)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn14 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn14)) / (locals.var_t2 * locals.var_t2)),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign75730_e115228;
        locals.var_tmf1_dn0 = assign75730_e115228_d_n0;
        locals.var_tmf1_dn2 = assign75730_e115228_d_n2;
        locals.var_tmf1_dn4 = assign75730_e115228_d_n4;
        locals.var_tmf1_dn5 = assign75730_e115228_d_n5;
        locals.var_tmf1_dn6 = assign75730_e115228_d_n6;
        locals.var_tmf1_dn7 = assign75730_e115228_d_n7;
        locals.var_tmf1_dn8 = assign75730_e115228_d_n8;
        locals.var_tmf1_dn9 = assign75730_e115228_d_n9;
        locals.var_tmf1_dn10 = assign75730_e115228_d_n10;
        locals.var_tmf1_dn11 = assign75730_e115228_d_n11;
        locals.var_tmf1_dn14 = assign75730_e115228_d_n14;
        locals.var_tmf1_rv = 0.0;

        let (assign75740_e115238, assign75740_e115238_d_n0, assign75740_e115238_d_n2, assign75740_e115238_d_n4, assign75740_e115238_d_n5, assign75740_e115238_d_n6, assign75740_e115238_d_n7, assign75740_e115238_d_n8, assign75740_e115238_d_n9, assign75740_e115238_d_n10, assign75740_e115238_d_n11, assign75740_e115238_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1779 != 0.0)) && (locals.var_guard1780 != 0.0)) {
        let assign75740_e115236: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign75740_e115236, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)), ((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)), ((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)), ((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign75740_e115238;
        locals.var_tmf2_dn0 = assign75740_e115238_d_n0;
        locals.var_tmf2_dn2 = assign75740_e115238_d_n2;
        locals.var_tmf2_dn4 = assign75740_e115238_d_n4;
        locals.var_tmf2_dn5 = assign75740_e115238_d_n5;
        locals.var_tmf2_dn6 = assign75740_e115238_d_n6;
        locals.var_tmf2_dn7 = assign75740_e115238_d_n7;
        locals.var_tmf2_dn8 = assign75740_e115238_d_n8;
        locals.var_tmf2_dn9 = assign75740_e115238_d_n9;
        locals.var_tmf2_dn10 = assign75740_e115238_d_n10;
        locals.var_tmf2_dn11 = assign75740_e115238_d_n11;
        locals.var_tmf2_dn14 = assign75740_e115238_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign75750_e115248, assign75750_e115248_d_n0, assign75750_e115248_d_n2, assign75750_e115248_d_n4, assign75750_e115248_d_n5, assign75750_e115248_d_n6, assign75750_e115248_d_n7, assign75750_e115248_d_n8, assign75750_e115248_d_n9, assign75750_e115248_d_n10, assign75750_e115248_d_n11, assign75750_e115248_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1779 != 0.0)) && (locals.var_guard1780 != 0.0)) {
        let assign75750_e115246: f64 = (locals.var_tmf2 * locals.var_tmf1);
        (assign75750_e115246, ((locals.var_tmf2_dn0 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn0)), ((locals.var_tmf2_dn2 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn2)), ((locals.var_tmf2_dn4 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn4)), ((locals.var_tmf2_dn5 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn5)), ((locals.var_tmf2_dn6 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn6)), ((locals.var_tmf2_dn7 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn7)), ((locals.var_tmf2_dn8 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn8)), ((locals.var_tmf2_dn9 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn9)), ((locals.var_tmf2_dn10 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn10)), ((locals.var_tmf2_dn11 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn11)), ((locals.var_tmf2_dn14 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn14)),)
    } else {
        (locals.var_tmf3, locals.var_tmf3_dn0, locals.var_tmf3_dn2, locals.var_tmf3_dn4, locals.var_tmf3_dn5, locals.var_tmf3_dn6, locals.var_tmf3_dn7, locals.var_tmf3_dn8, locals.var_tmf3_dn9, locals.var_tmf3_dn10, locals.var_tmf3_dn11, locals.var_tmf3_dn14,)
    }
};
        locals.var_tmf3 = assign75750_e115248;
        locals.var_tmf3_dn0 = assign75750_e115248_d_n0;
        locals.var_tmf3_dn2 = assign75750_e115248_d_n2;
        locals.var_tmf3_dn4 = assign75750_e115248_d_n4;
        locals.var_tmf3_dn5 = assign75750_e115248_d_n5;
        locals.var_tmf3_dn6 = assign75750_e115248_d_n6;
        locals.var_tmf3_dn7 = assign75750_e115248_d_n7;
        locals.var_tmf3_dn8 = assign75750_e115248_d_n8;
        locals.var_tmf3_dn9 = assign75750_e115248_d_n9;
        locals.var_tmf3_dn10 = assign75750_e115248_d_n10;
        locals.var_tmf3_dn11 = assign75750_e115248_d_n11;
        locals.var_tmf3_dn14 = assign75750_e115248_d_n14;
        locals.var_tmf3_rv = 0.0;

        let (assign75760_e115258, assign75760_e115258_d_n0, assign75760_e115258_d_n2, assign75760_e115258_d_n4, assign75760_e115258_d_n5, assign75760_e115258_d_n6, assign75760_e115258_d_n7, assign75760_e115258_d_n8, assign75760_e115258_d_n9, assign75760_e115258_d_n10, assign75760_e115258_d_n11, assign75760_e115258_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1779 != 0.0)) && (locals.var_guard1780 != 0.0)) {
        let assign75760_e115256: f64 = (locals.var_tmf2 * locals.var_tmf2);
        (assign75760_e115256, ((locals.var_tmf2_dn0 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn0)), ((locals.var_tmf2_dn2 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn2)), ((locals.var_tmf2_dn4 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn4)), ((locals.var_tmf2_dn5 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn5)), ((locals.var_tmf2_dn6 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn6)), ((locals.var_tmf2_dn7 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn7)), ((locals.var_tmf2_dn8 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn8)), ((locals.var_tmf2_dn9 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn9)), ((locals.var_tmf2_dn10 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn10)), ((locals.var_tmf2_dn11 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn11)), ((locals.var_tmf2_dn14 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn14)),)
    } else {
        (locals.var_tmf4, locals.var_tmf4_dn0, locals.var_tmf4_dn2, locals.var_tmf4_dn4, locals.var_tmf4_dn5, locals.var_tmf4_dn6, locals.var_tmf4_dn7, locals.var_tmf4_dn8, locals.var_tmf4_dn9, locals.var_tmf4_dn10, locals.var_tmf4_dn11, locals.var_tmf4_dn14,)
    }
};
        locals.var_tmf4 = assign75760_e115258;
        locals.var_tmf4_dn0 = assign75760_e115258_d_n0;
        locals.var_tmf4_dn2 = assign75760_e115258_d_n2;
        locals.var_tmf4_dn4 = assign75760_e115258_d_n4;
        locals.var_tmf4_dn5 = assign75760_e115258_d_n5;
        locals.var_tmf4_dn6 = assign75760_e115258_d_n6;
        locals.var_tmf4_dn7 = assign75760_e115258_d_n7;
        locals.var_tmf4_dn8 = assign75760_e115258_d_n8;
        locals.var_tmf4_dn9 = assign75760_e115258_d_n9;
        locals.var_tmf4_dn10 = assign75760_e115258_d_n10;
        locals.var_tmf4_dn11 = assign75760_e115258_d_n11;
        locals.var_tmf4_dn14 = assign75760_e115258_d_n14;
        locals.var_tmf4_rv = 0.0;

        let (assign75770_e115276, assign75770_e115276_d_n0, assign75770_e115276_d_n2, assign75770_e115276_d_n4, assign75770_e115276_d_n5, assign75770_e115276_d_n6, assign75770_e115276_d_n7, assign75770_e115276_d_n8, assign75770_e115276_d_n9, assign75770_e115276_d_n10, assign75770_e115276_d_n11, assign75770_e115276_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1779 != 0.0)) && (locals.var_guard1780 != 0.0)) {
        let assign75770_e115267: f64 = (1.0 + locals.var_tmf1);
        let assign75770_e115269: f64 = (assign75770_e115267 + locals.var_tmf2);
        let assign75770_e115271: f64 = (assign75770_e115269 + locals.var_tmf3);
        let assign75770_e115273: f64 = (assign75770_e115271 + locals.var_tmf4);
        let assign75770_e115274: f64 = (1.0 / assign75770_e115273);
        (assign75770_e115274, (-((((locals.var_tmf1_dn0 + locals.var_tmf2_dn0) + locals.var_tmf3_dn0) + locals.var_tmf4_dn0) / (assign75770_e115273 * assign75770_e115273))), (-((((locals.var_tmf1_dn2 + locals.var_tmf2_dn2) + locals.var_tmf3_dn2) + locals.var_tmf4_dn2) / (assign75770_e115273 * assign75770_e115273))), (-((((locals.var_tmf1_dn4 + locals.var_tmf2_dn4) + locals.var_tmf3_dn4) + locals.var_tmf4_dn4) / (assign75770_e115273 * assign75770_e115273))), (-((((locals.var_tmf1_dn5 + locals.var_tmf2_dn5) + locals.var_tmf3_dn5) + locals.var_tmf4_dn5) / (assign75770_e115273 * assign75770_e115273))), (-((((locals.var_tmf1_dn6 + locals.var_tmf2_dn6) + locals.var_tmf3_dn6) + locals.var_tmf4_dn6) / (assign75770_e115273 * assign75770_e115273))), (-((((locals.var_tmf1_dn7 + locals.var_tmf2_dn7) + locals.var_tmf3_dn7) + locals.var_tmf4_dn7) / (assign75770_e115273 * assign75770_e115273))), (-((((locals.var_tmf1_dn8 + locals.var_tmf2_dn8) + locals.var_tmf3_dn8) + locals.var_tmf4_dn8) / (assign75770_e115273 * assign75770_e115273))), (-((((locals.var_tmf1_dn9 + locals.var_tmf2_dn9) + locals.var_tmf3_dn9) + locals.var_tmf4_dn9) / (assign75770_e115273 * assign75770_e115273))), (-((((locals.var_tmf1_dn10 + locals.var_tmf2_dn10) + locals.var_tmf3_dn10) + locals.var_tmf4_dn10) / (assign75770_e115273 * assign75770_e115273))), (-((((locals.var_tmf1_dn11 + locals.var_tmf2_dn11) + locals.var_tmf3_dn11) + locals.var_tmf4_dn11) / (assign75770_e115273 * assign75770_e115273))), (-((((locals.var_tmf1_dn14 + locals.var_tmf2_dn14) + locals.var_tmf3_dn14) + locals.var_tmf4_dn14) / (assign75770_e115273 * assign75770_e115273))),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn11, locals.var_tmf0_dn14,)
    }
};
        locals.var_tmf0 = assign75770_e115276;
        locals.var_tmf0_dn0 = assign75770_e115276_d_n0;
        locals.var_tmf0_dn2 = assign75770_e115276_d_n2;
        locals.var_tmf0_dn4 = assign75770_e115276_d_n4;
        locals.var_tmf0_dn5 = assign75770_e115276_d_n5;
        locals.var_tmf0_dn6 = assign75770_e115276_d_n6;
        locals.var_tmf0_dn7 = assign75770_e115276_d_n7;
        locals.var_tmf0_dn8 = assign75770_e115276_d_n8;
        locals.var_tmf0_dn9 = assign75770_e115276_d_n9;
        locals.var_tmf0_dn10 = assign75770_e115276_d_n10;
        locals.var_tmf0_dn11 = assign75770_e115276_d_n11;
        locals.var_tmf0_dn14 = assign75770_e115276_d_n14;
        locals.var_tmf0_rv = 0.0;

        let (assign75780_e115301, assign75780_e115301_d_n0, assign75780_e115301_d_n2, assign75780_e115301_d_n4, assign75780_e115301_d_n5, assign75780_e115301_d_n6, assign75780_e115301_d_n7, assign75780_e115301_d_n8, assign75780_e115301_d_n9, assign75780_e115301_d_n10, assign75780_e115301_d_n11, assign75780_e115301_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1779 != 0.0)) && (locals.var_guard1780 != 0.0)) {
        let assign75780_e115285: f64 = (2.0 * locals.var_tmf1);
        let assign75780_e115286: f64 = (1.0 + assign75780_e115285);
        let assign75780_e115289: f64 = (3.0 * locals.var_tmf2);
        let assign75780_e115290: f64 = (assign75780_e115286 + assign75780_e115289);
        let assign75780_e115293: f64 = (4.0 * locals.var_tmf3);
        let assign75780_e115294: f64 = (assign75780_e115290 + assign75780_e115293);
        let assign75780_e115295: f64 = (-assign75780_e115294);
        let assign75780_e115297: f64 = (assign75780_e115295 * locals.var_tmf0);
        let assign75780_e115299: f64 = (assign75780_e115297 * locals.var_tmf0);
        (assign75780_e115299, (((((-(((2.0 * locals.var_tmf1_dn0) + (3.0 * locals.var_tmf2_dn0)) + (4.0 * locals.var_tmf3_dn0))) * locals.var_tmf0) + (assign75780_e115295 * locals.var_tmf0_dn0)) * locals.var_tmf0) + (assign75780_e115297 * locals.var_tmf0_dn0)), (((((-(((2.0 * locals.var_tmf1_dn2) + (3.0 * locals.var_tmf2_dn2)) + (4.0 * locals.var_tmf3_dn2))) * locals.var_tmf0) + (assign75780_e115295 * locals.var_tmf0_dn2)) * locals.var_tmf0) + (assign75780_e115297 * locals.var_tmf0_dn2)), (((((-(((2.0 * locals.var_tmf1_dn4) + (3.0 * locals.var_tmf2_dn4)) + (4.0 * locals.var_tmf3_dn4))) * locals.var_tmf0) + (assign75780_e115295 * locals.var_tmf0_dn4)) * locals.var_tmf0) + (assign75780_e115297 * locals.var_tmf0_dn4)), (((((-(((2.0 * locals.var_tmf1_dn5) + (3.0 * locals.var_tmf2_dn5)) + (4.0 * locals.var_tmf3_dn5))) * locals.var_tmf0) + (assign75780_e115295 * locals.var_tmf0_dn5)) * locals.var_tmf0) + (assign75780_e115297 * locals.var_tmf0_dn5)), (((((-(((2.0 * locals.var_tmf1_dn6) + (3.0 * locals.var_tmf2_dn6)) + (4.0 * locals.var_tmf3_dn6))) * locals.var_tmf0) + (assign75780_e115295 * locals.var_tmf0_dn6)) * locals.var_tmf0) + (assign75780_e115297 * locals.var_tmf0_dn6)), (((((-(((2.0 * locals.var_tmf1_dn7) + (3.0 * locals.var_tmf2_dn7)) + (4.0 * locals.var_tmf3_dn7))) * locals.var_tmf0) + (assign75780_e115295 * locals.var_tmf0_dn7)) * locals.var_tmf0) + (assign75780_e115297 * locals.var_tmf0_dn7)), (((((-(((2.0 * locals.var_tmf1_dn8) + (3.0 * locals.var_tmf2_dn8)) + (4.0 * locals.var_tmf3_dn8))) * locals.var_tmf0) + (assign75780_e115295 * locals.var_tmf0_dn8)) * locals.var_tmf0) + (assign75780_e115297 * locals.var_tmf0_dn8)), (((((-(((2.0 * locals.var_tmf1_dn9) + (3.0 * locals.var_tmf2_dn9)) + (4.0 * locals.var_tmf3_dn9))) * locals.var_tmf0) + (assign75780_e115295 * locals.var_tmf0_dn9)) * locals.var_tmf0) + (assign75780_e115297 * locals.var_tmf0_dn9)), (((((-(((2.0 * locals.var_tmf1_dn10) + (3.0 * locals.var_tmf2_dn10)) + (4.0 * locals.var_tmf3_dn10))) * locals.var_tmf0) + (assign75780_e115295 * locals.var_tmf0_dn10)) * locals.var_tmf0) + (assign75780_e115297 * locals.var_tmf0_dn10)), (((((-(((2.0 * locals.var_tmf1_dn11) + (3.0 * locals.var_tmf2_dn11)) + (4.0 * locals.var_tmf3_dn11))) * locals.var_tmf0) + (assign75780_e115295 * locals.var_tmf0_dn11)) * locals.var_tmf0) + (assign75780_e115297 * locals.var_tmf0_dn11)), (((((-(((2.0 * locals.var_tmf1_dn14) + (3.0 * locals.var_tmf2_dn14)) + (4.0 * locals.var_tmf3_dn14))) * locals.var_tmf0) + (assign75780_e115295 * locals.var_tmf0_dn14)) * locals.var_tmf0) + (assign75780_e115297 * locals.var_tmf0_dn14)),)
    } else {
        (locals.var_t11, locals.var_t11_dn0, locals.var_t11_dn2, locals.var_t11_dn4, locals.var_t11_dn5, locals.var_t11_dn6, locals.var_t11_dn7, locals.var_t11_dn8, locals.var_t11_dn9, locals.var_t11_dn10, locals.var_t11_dn11, locals.var_t11_dn14,)
    }
};
        locals.var_t11 = assign75780_e115301;
        locals.var_t11_dn0 = assign75780_e115301_d_n0;
        locals.var_t11_dn2 = assign75780_e115301_d_n2;
        locals.var_t11_dn4 = assign75780_e115301_d_n4;
        locals.var_t11_dn5 = assign75780_e115301_d_n5;
        locals.var_t11_dn6 = assign75780_e115301_d_n6;
        locals.var_t11_dn7 = assign75780_e115301_d_n7;
        locals.var_t11_dn8 = assign75780_e115301_d_n8;
        locals.var_t11_dn9 = assign75780_e115301_d_n9;
        locals.var_t11_dn10 = assign75780_e115301_d_n10;
        locals.var_t11_dn11 = assign75780_e115301_d_n11;
        locals.var_t11_dn14 = assign75780_e115301_d_n14;
        locals.var_t11_rv = 0.0;

        let (assign75790_e115313, assign75790_e115313_d_n0, assign75790_e115313_d_n2, assign75790_e115313_d_n4, assign75790_e115313_d_n5, assign75790_e115313_d_n6, assign75790_e115313_d_n7, assign75790_e115313_d_n8, assign75790_e115313_d_n9, assign75790_e115313_d_n10, assign75790_e115313_d_n11, assign75790_e115313_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1779 != 0.0)) && (locals.var_guard1780 != 0.0)) {
        let assign75790_e115310: f64 = (1.0 - locals.var_tmf0);
        let assign75790_e115311: f64 = (locals.var_t2 * assign75790_e115310);
        (assign75790_e115311, ((locals.var_t2_dn0 * assign75790_e115310) + (locals.var_t2 * (-locals.var_tmf0_dn0))), ((locals.var_t2_dn2 * assign75790_e115310) + (locals.var_t2 * (-locals.var_tmf0_dn2))), ((locals.var_t2_dn4 * assign75790_e115310) + (locals.var_t2 * (-locals.var_tmf0_dn4))), ((locals.var_t2_dn5 * assign75790_e115310) + (locals.var_t2 * (-locals.var_tmf0_dn5))), ((locals.var_t2_dn6 * assign75790_e115310) + (locals.var_t2 * (-locals.var_tmf0_dn6))), ((locals.var_t2_dn7 * assign75790_e115310) + (locals.var_t2 * (-locals.var_tmf0_dn7))), ((locals.var_t2_dn8 * assign75790_e115310) + (locals.var_t2 * (-locals.var_tmf0_dn8))), ((locals.var_t2_dn9 * assign75790_e115310) + (locals.var_t2 * (-locals.var_tmf0_dn9))), ((locals.var_t2_dn10 * assign75790_e115310) + (locals.var_t2 * (-locals.var_tmf0_dn10))), ((locals.var_t2_dn11 * assign75790_e115310) + (locals.var_t2 * (-locals.var_tmf0_dn11))), ((locals.var_t2_dn14 * assign75790_e115310) + (locals.var_t2 * (-locals.var_tmf0_dn14))),)
    } else {
        (locals.var_ty, locals.var_ty_dn0, locals.var_ty_dn2, locals.var_ty_dn4, locals.var_ty_dn5, locals.var_ty_dn6, locals.var_ty_dn7, locals.var_ty_dn8, locals.var_ty_dn9, locals.var_ty_dn10, locals.var_ty_dn11, locals.var_ty_dn14,)
    }
};
        locals.var_ty = assign75790_e115313;
        locals.var_ty_dn0 = assign75790_e115313_d_n0;
        locals.var_ty_dn2 = assign75790_e115313_d_n2;
        locals.var_ty_dn4 = assign75790_e115313_d_n4;
        locals.var_ty_dn5 = assign75790_e115313_d_n5;
        locals.var_ty_dn6 = assign75790_e115313_d_n6;
        locals.var_ty_dn7 = assign75790_e115313_d_n7;
        locals.var_ty_dn8 = assign75790_e115313_d_n8;
        locals.var_ty_dn9 = assign75790_e115313_d_n9;
        locals.var_ty_dn10 = assign75790_e115313_d_n10;
        locals.var_ty_dn11 = assign75790_e115313_d_n11;
        locals.var_ty_dn14 = assign75790_e115313_d_n14;
        locals.var_ty_rv = 0.0;

        let (assign75800_e115327, assign75800_e115327_d_n0, assign75800_e115327_d_n2, assign75800_e115327_d_n4, assign75800_e115327_d_n5, assign75800_e115327_d_n6, assign75800_e115327_d_n7, assign75800_e115327_d_n8, assign75800_e115327_d_n9, assign75800_e115327_d_n10, assign75800_e115327_d_n11, assign75800_e115327_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1779 != 0.0)) && (locals.var_guard1780 != 0.0)) {
        let assign75800_e115321: f64 = (1.0 - locals.var_tmf0);
        let assign75800_e115324: f64 = (locals.var_tmf1 * locals.var_t11);
        let assign75800_e115325: f64 = (assign75800_e115321 + assign75800_e115324);
        (assign75800_e115325, ((-locals.var_tmf0_dn0) + ((locals.var_tmf1_dn0 * locals.var_t11) + (locals.var_tmf1 * locals.var_t11_dn0))), ((-locals.var_tmf0_dn2) + ((locals.var_tmf1_dn2 * locals.var_t11) + (locals.var_tmf1 * locals.var_t11_dn2))), ((-locals.var_tmf0_dn4) + ((locals.var_tmf1_dn4 * locals.var_t11) + (locals.var_tmf1 * locals.var_t11_dn4))), ((-locals.var_tmf0_dn5) + ((locals.var_tmf1_dn5 * locals.var_t11) + (locals.var_tmf1 * locals.var_t11_dn5))), ((-locals.var_tmf0_dn6) + ((locals.var_tmf1_dn6 * locals.var_t11) + (locals.var_tmf1 * locals.var_t11_dn6))), ((-locals.var_tmf0_dn7) + ((locals.var_tmf1_dn7 * locals.var_t11) + (locals.var_tmf1 * locals.var_t11_dn7))), ((-locals.var_tmf0_dn8) + ((locals.var_tmf1_dn8 * locals.var_t11) + (locals.var_tmf1 * locals.var_t11_dn8))), ((-locals.var_tmf0_dn9) + ((locals.var_tmf1_dn9 * locals.var_t11) + (locals.var_tmf1 * locals.var_t11_dn9))), ((-locals.var_tmf0_dn10) + ((locals.var_tmf1_dn10 * locals.var_t11) + (locals.var_tmf1 * locals.var_t11_dn10))), ((-locals.var_tmf0_dn11) + ((locals.var_tmf1_dn11 * locals.var_t11) + (locals.var_tmf1 * locals.var_t11_dn11))), ((-locals.var_tmf0_dn14) + ((locals.var_tmf1_dn14 * locals.var_t11) + (locals.var_tmf1 * locals.var_t11_dn14))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign75800_e115327;
        locals.var_t0_dn0 = assign75800_e115327_d_n0;
        locals.var_t0_dn2 = assign75800_e115327_d_n2;
        locals.var_t0_dn4 = assign75800_e115327_d_n4;
        locals.var_t0_dn5 = assign75800_e115327_d_n5;
        locals.var_t0_dn6 = assign75800_e115327_d_n6;
        locals.var_t0_dn7 = assign75800_e115327_d_n7;
        locals.var_t0_dn8 = assign75800_e115327_d_n8;
        locals.var_t0_dn9 = assign75800_e115327_d_n9;
        locals.var_t0_dn10 = assign75800_e115327_d_n10;
        locals.var_t0_dn11 = assign75800_e115327_d_n11;
        locals.var_t0_dn14 = assign75800_e115327_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign75810_e115336, assign75810_e115336_d_n0, assign75810_e115336_d_n2, assign75810_e115336_d_n4, assign75810_e115336_d_n5, assign75810_e115336_d_n6, assign75810_e115336_d_n7, assign75810_e115336_d_n8, assign75810_e115336_d_n9, assign75810_e115336_d_n10, assign75810_e115336_d_n11, assign75810_e115336_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1779 != 0.0)) && (locals.var_guard1780 != 0.0)) {
        let assign75810_e115334: f64 = (-locals.var_t11);
        (assign75810_e115334, (-locals.var_t11_dn0), (-locals.var_t11_dn2), (-locals.var_t11_dn4), (-locals.var_t11_dn5), (-locals.var_t11_dn6), (-locals.var_t11_dn7), (-locals.var_t11_dn8), (-locals.var_t11_dn9), (-locals.var_t11_dn10), (-locals.var_t11_dn11), (-locals.var_t11_dn14),)
    } else {
        (locals.var_t11, locals.var_t11_dn0, locals.var_t11_dn2, locals.var_t11_dn4, locals.var_t11_dn5, locals.var_t11_dn6, locals.var_t11_dn7, locals.var_t11_dn8, locals.var_t11_dn9, locals.var_t11_dn10, locals.var_t11_dn11, locals.var_t11_dn14,)
    }
};
        locals.var_t11 = assign75810_e115336;
        locals.var_t11_dn0 = assign75810_e115336_d_n0;
        locals.var_t11_dn2 = assign75810_e115336_d_n2;
        locals.var_t11_dn4 = assign75810_e115336_d_n4;
        locals.var_t11_dn5 = assign75810_e115336_d_n5;
        locals.var_t11_dn6 = assign75810_e115336_d_n6;
        locals.var_t11_dn7 = assign75810_e115336_d_n7;
        locals.var_t11_dn8 = assign75810_e115336_d_n8;
        locals.var_t11_dn9 = assign75810_e115336_d_n9;
        locals.var_t11_dn10 = assign75810_e115336_d_n10;
        locals.var_t11_dn11 = assign75810_e115336_d_n11;
        locals.var_t11_dn14 = assign75810_e115336_d_n14;
        locals.var_t11_rv = 0.0;

        let (assign75820_e115346, assign75820_e115346_d_n0, assign75820_e115346_d_n2, assign75820_e115346_d_n4, assign75820_e115346_d_n5, assign75820_e115346_d_n6, assign75820_e115346_d_n7, assign75820_e115346_d_n8, assign75820_e115346_d_n9, assign75820_e115346_d_n10, assign75820_e115346_d_n11, assign75820_e115346_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1779 != 0.0)) && (locals.var_guard1780 != 0.0)) {
        let assign75820_e115344: f64 = (locals.var_vbs_bnd_over__blk1769 + locals.var_ty);
        (assign75820_e115344, (locals.var_vbs_bnd_over__blk1769_dn0 + locals.var_ty_dn0), (locals.var_vbs_bnd_over__blk1769_dn2 + locals.var_ty_dn2), (locals.var_vbs_bnd_over__blk1769_dn4 + locals.var_ty_dn4), (locals.var_vbs_bnd_over__blk1769_dn5 + locals.var_ty_dn5), (locals.var_vbs_bnd_over__blk1769_dn6 + locals.var_ty_dn6), (locals.var_vbs_bnd_over__blk1769_dn7 + locals.var_ty_dn7), (locals.var_vbs_bnd_over__blk1769_dn8 + locals.var_ty_dn8), (locals.var_vbs_bnd_over__blk1769_dn9 + locals.var_ty_dn9), (locals.var_vbs_bnd_over__blk1769_dn10 + locals.var_ty_dn10), (locals.var_vbs_bnd_over__blk1769_dn11 + locals.var_ty_dn11), (locals.var_vbs_bnd_over__blk1769_dn14 + locals.var_ty_dn14),)
    } else {
        (locals.var_t10, locals.var_t10_dn0, locals.var_t10_dn2, locals.var_t10_dn4, locals.var_t10_dn5, locals.var_t10_dn6, locals.var_t10_dn7, locals.var_t10_dn8, locals.var_t10_dn9, locals.var_t10_dn10, locals.var_t10_dn11, locals.var_t10_dn14,)
    }
};
        locals.var_t10 = assign75820_e115346;
        locals.var_t10_dn0 = assign75820_e115346_d_n0;
        locals.var_t10_dn2 = assign75820_e115346_d_n2;
        locals.var_t10_dn4 = assign75820_e115346_d_n4;
        locals.var_t10_dn5 = assign75820_e115346_d_n5;
        locals.var_t10_dn6 = assign75820_e115346_d_n6;
        locals.var_t10_dn7 = assign75820_e115346_d_n7;
        locals.var_t10_dn8 = assign75820_e115346_d_n8;
        locals.var_t10_dn9 = assign75820_e115346_d_n9;
        locals.var_t10_dn10 = assign75820_e115346_d_n10;
        locals.var_t10_dn11 = assign75820_e115346_d_n11;
        locals.var_t10_dn14 = assign75820_e115346_d_n14;
        locals.var_t10_rv = 0.0;

        let (assign75830_e115355, assign75830_e115355_d_n0, assign75830_e115355_d_n2, assign75830_e115355_d_n4, assign75830_e115355_d_n5, assign75830_e115355_d_n6, assign75830_e115355_d_n7, assign75830_e115355_d_n8, assign75830_e115355_d_n9, assign75830_e115355_d_n10, assign75830_e115355_d_n11, assign75830_e115355_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1779 != 0.0)) && (locals.var_guard1780 == 0.0)) {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    } else {
        (locals.var_t10, locals.var_t10_dn0, locals.var_t10_dn2, locals.var_t10_dn4, locals.var_t10_dn5, locals.var_t10_dn6, locals.var_t10_dn7, locals.var_t10_dn8, locals.var_t10_dn9, locals.var_t10_dn10, locals.var_t10_dn11, locals.var_t10_dn14,)
    }
};
        locals.var_t10 = assign75830_e115355;
        locals.var_t10_dn0 = assign75830_e115355_d_n0;
        locals.var_t10_dn2 = assign75830_e115355_d_n2;
        locals.var_t10_dn4 = assign75830_e115355_d_n4;
        locals.var_t10_dn5 = assign75830_e115355_d_n5;
        locals.var_t10_dn6 = assign75830_e115355_d_n6;
        locals.var_t10_dn7 = assign75830_e115355_d_n7;
        locals.var_t10_dn8 = assign75830_e115355_d_n8;
        locals.var_t10_dn9 = assign75830_e115355_d_n9;
        locals.var_t10_dn10 = assign75830_e115355_d_n10;
        locals.var_t10_dn11 = assign75830_e115355_d_n11;
        locals.var_t10_dn14 = assign75830_e115355_d_n14;
        locals.var_t10_rv = 0.0;

        let (assign75840_e115362, assign75840_e115362_d_n0, assign75840_e115362_d_n2, assign75840_e115362_d_n4, assign75840_e115362_d_n5, assign75840_e115362_d_n6, assign75840_e115362_d_n7, assign75840_e115362_d_n8, assign75840_e115362_d_n9, assign75840_e115362_d_n10, assign75840_e115362_d_n11, assign75840_e115362_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1779 != 0.0)) {
        let assign75840_e115360: f64 = (-locals.var_t10);
        (assign75840_e115360, (-locals.var_t10_dn0), (-locals.var_t10_dn2), (-locals.var_t10_dn4), (-locals.var_t10_dn5), (-locals.var_t10_dn6), (-locals.var_t10_dn7), (-locals.var_t10_dn8), (-locals.var_t10_dn9), (-locals.var_t10_dn10), (-locals.var_t10_dn11), (-locals.var_t10_dn14),)
    } else {
        (locals.var_vxbgmtcl, locals.var_vxbgmtcl_dn0, locals.var_vxbgmtcl_dn2, locals.var_vxbgmtcl_dn4, locals.var_vxbgmtcl_dn5, locals.var_vxbgmtcl_dn6, locals.var_vxbgmtcl_dn7, locals.var_vxbgmtcl_dn8, locals.var_vxbgmtcl_dn9, locals.var_vxbgmtcl_dn10, locals.var_vxbgmtcl_dn11, locals.var_vxbgmtcl_dn14,)
    }
};
        locals.var_vxbgmtcl = assign75840_e115362;
        locals.var_vxbgmtcl_dn0 = assign75840_e115362_d_n0;
        locals.var_vxbgmtcl_dn2 = assign75840_e115362_d_n2;
        locals.var_vxbgmtcl_dn4 = assign75840_e115362_d_n4;
        locals.var_vxbgmtcl_dn5 = assign75840_e115362_d_n5;
        locals.var_vxbgmtcl_dn6 = assign75840_e115362_d_n6;
        locals.var_vxbgmtcl_dn7 = assign75840_e115362_d_n7;
        locals.var_vxbgmtcl_dn8 = assign75840_e115362_d_n8;
        locals.var_vxbgmtcl_dn9 = assign75840_e115362_d_n9;
        locals.var_vxbgmtcl_dn10 = assign75840_e115362_d_n10;
        locals.var_vxbgmtcl_dn11 = assign75840_e115362_d_n11;
        locals.var_vxbgmtcl_dn14 = assign75840_e115362_d_n14;
        locals.var_vxbgmtcl_rv = 0.0;

        let (assign75850_e115369, assign75850_e115369_d_n0, assign75850_e115369_d_n2, assign75850_e115369_d_n4, assign75850_e115369_d_n5, assign75850_e115369_d_n6, assign75850_e115369_d_n7, assign75850_e115369_d_n8, assign75850_e115369_d_n9, assign75850_e115369_d_n10, assign75850_e115369_d_n11, assign75850_e115369_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1779 == 0.0)) {
        (locals.var_vxbgmt, locals.var_vxbgmt_dn0, locals.var_vxbgmt_dn2, locals.var_vxbgmt_dn4, locals.var_vxbgmt_dn5, locals.var_vxbgmt_dn6, locals.var_vxbgmt_dn7, locals.var_vxbgmt_dn8, locals.var_vxbgmt_dn9, locals.var_vxbgmt_dn10, locals.var_vxbgmt_dn11, locals.var_vxbgmt_dn14,)
    } else {
        (locals.var_vxbgmtcl, locals.var_vxbgmtcl_dn0, locals.var_vxbgmtcl_dn2, locals.var_vxbgmtcl_dn4, locals.var_vxbgmtcl_dn5, locals.var_vxbgmtcl_dn6, locals.var_vxbgmtcl_dn7, locals.var_vxbgmtcl_dn8, locals.var_vxbgmtcl_dn9, locals.var_vxbgmtcl_dn10, locals.var_vxbgmtcl_dn11, locals.var_vxbgmtcl_dn14,)
    }
};
        locals.var_vxbgmtcl = assign75850_e115369;
        locals.var_vxbgmtcl_dn0 = assign75850_e115369_d_n0;
        locals.var_vxbgmtcl_dn2 = assign75850_e115369_d_n2;
        locals.var_vxbgmtcl_dn4 = assign75850_e115369_d_n4;
        locals.var_vxbgmtcl_dn5 = assign75850_e115369_d_n5;
        locals.var_vxbgmtcl_dn6 = assign75850_e115369_d_n6;
        locals.var_vxbgmtcl_dn7 = assign75850_e115369_d_n7;
        locals.var_vxbgmtcl_dn8 = assign75850_e115369_d_n8;
        locals.var_vxbgmtcl_dn9 = assign75850_e115369_d_n9;
        locals.var_vxbgmtcl_dn10 = assign75850_e115369_d_n10;
        locals.var_vxbgmtcl_dn11 = assign75850_e115369_d_n11;
        locals.var_vxbgmtcl_dn14 = assign75850_e115369_d_n14;
        locals.var_vxbgmtcl_rv = 0.0;

        let (assign75860_e115375, assign75860_e115375_d_n0, assign75860_e115375_d_n2, assign75860_e115375_d_n4, assign75860_e115375_d_n5, assign75860_e115375_d_n6, assign75860_e115375_d_n7, assign75860_e115375_d_n8, assign75860_e115375_d_n9, assign75860_e115375_d_n10, assign75860_e115375_d_n11, assign75860_e115375_d_n14,) = {
    if (locals.var_flg_calcqover != 0.0) {
        let assign75860_e115373: f64 = (locals.var_cnst0over_func / locals.var_cox0_func);
        (assign75860_e115373, (locals.var_cnst0over_func_dn0 / locals.var_cox0_func), (locals.var_cnst0over_func_dn2 / locals.var_cox0_func), (locals.var_cnst0over_func_dn4 / locals.var_cox0_func), (locals.var_cnst0over_func_dn5 / locals.var_cox0_func), (locals.var_cnst0over_func_dn6 / locals.var_cox0_func), (locals.var_cnst0over_func_dn7 / locals.var_cox0_func), (locals.var_cnst0over_func_dn8 / locals.var_cox0_func), (locals.var_cnst0over_func_dn9 / locals.var_cox0_func), (locals.var_cnst0over_func_dn10 / locals.var_cox0_func), (locals.var_cnst0over_func_dn11 / locals.var_cox0_func), (locals.var_cnst0over_func_dn14 / locals.var_cox0_func),)
    } else {
        (locals.var_fac1, locals.var_fac1_dn0, locals.var_fac1_dn2, locals.var_fac1_dn4, locals.var_fac1_dn5, locals.var_fac1_dn6, locals.var_fac1_dn7, locals.var_fac1_dn8, locals.var_fac1_dn9, locals.var_fac1_dn10, locals.var_fac1_dn11, locals.var_fac1_dn14,)
    }
};
        locals.var_fac1 = assign75860_e115375;
        locals.var_fac1_dn0 = assign75860_e115375_d_n0;
        locals.var_fac1_dn2 = assign75860_e115375_d_n2;
        locals.var_fac1_dn4 = assign75860_e115375_d_n4;
        locals.var_fac1_dn5 = assign75860_e115375_d_n5;
        locals.var_fac1_dn6 = assign75860_e115375_d_n6;
        locals.var_fac1_dn7 = assign75860_e115375_d_n7;
        locals.var_fac1_dn8 = assign75860_e115375_d_n8;
        locals.var_fac1_dn9 = assign75860_e115375_d_n9;
        locals.var_fac1_dn10 = assign75860_e115375_d_n10;
        locals.var_fac1_dn11 = assign75860_e115375_d_n11;
        locals.var_fac1_dn14 = assign75860_e115375_d_n14;
        locals.var_fac1_rv = 0.0;

        let (assign75870_e115381, assign75870_e115381_d_n0, assign75870_e115381_d_n2, assign75870_e115381_d_n4, assign75870_e115381_d_n5, assign75870_e115381_d_n6, assign75870_e115381_d_n7, assign75870_e115381_d_n8, assign75870_e115381_d_n9, assign75870_e115381_d_n10, assign75870_e115381_d_n11, assign75870_e115381_d_n14,) = {
    if (locals.var_flg_calcqover != 0.0) {
        let assign75870_e115379: f64 = (locals.var_fac1 * locals.var_fac1);
        (assign75870_e115379, ((locals.var_fac1_dn0 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn0)), ((locals.var_fac1_dn2 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn2)), ((locals.var_fac1_dn4 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn4)), ((locals.var_fac1_dn5 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn5)), ((locals.var_fac1_dn6 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn6)), ((locals.var_fac1_dn7 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn7)), ((locals.var_fac1_dn8 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn8)), ((locals.var_fac1_dn9 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn9)), ((locals.var_fac1_dn10 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn10)), ((locals.var_fac1_dn11 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn11)), ((locals.var_fac1_dn14 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn14)),)
    } else {
        (locals.var_fac1p2, locals.var_fac1p2_dn0, locals.var_fac1p2_dn2, locals.var_fac1p2_dn4, locals.var_fac1p2_dn5, locals.var_fac1p2_dn6, locals.var_fac1p2_dn7, locals.var_fac1p2_dn8, locals.var_fac1p2_dn9, locals.var_fac1p2_dn10, locals.var_fac1p2_dn11, locals.var_fac1p2_dn14,)
    }
};
        locals.var_fac1p2 = assign75870_e115381;
        locals.var_fac1p2_dn0 = assign75870_e115381_d_n0;
        locals.var_fac1p2_dn2 = assign75870_e115381_d_n2;
        locals.var_fac1p2_dn4 = assign75870_e115381_d_n4;
        locals.var_fac1p2_dn5 = assign75870_e115381_d_n5;
        locals.var_fac1p2_dn6 = assign75870_e115381_d_n6;
        locals.var_fac1p2_dn7 = assign75870_e115381_d_n7;
        locals.var_fac1p2_dn8 = assign75870_e115381_d_n8;
        locals.var_fac1p2_dn9 = assign75870_e115381_d_n9;
        locals.var_fac1p2_dn10 = assign75870_e115381_d_n10;
        locals.var_fac1p2_dn11 = assign75870_e115381_d_n11;
        locals.var_fac1p2_dn14 = assign75870_e115381_d_n14;
        locals.var_fac1p2_rv = 0.0;

        let (assign75880_e115388, assign75880_e115388_d_n2, assign75880_e115388_d_n7, assign75880_e115388_d_n8, assign75880_e115388_d_n9,) = {
    if (locals.var_flg_calcqover != 0.0) {
        let assign75880_e115384: f64 = (-locals.var_vgbgmt);
        let assign75880_e115386: f64 = (assign75880_e115384 + locals.var_uc_vfbover);
        (assign75880_e115386, (-locals.var_vgbgmt_dn2), (-locals.var_vgbgmt_dn7), (-locals.var_vgbgmt_dn8), (-locals.var_vgbgmt_dn9),)
    } else {
        (locals.var_vgpld, locals.var_vgpld_dn2, locals.var_vgpld_dn7, locals.var_vgpld_dn8, locals.var_vgpld_dn9,)
    }
};
        locals.var_vgpld = assign75880_e115388;
        locals.var_vgpld_dn2 = assign75880_e115388_d_n2;
        locals.var_vgpld_dn7 = assign75880_e115388_d_n7;
        locals.var_vgpld_dn8 = assign75880_e115388_d_n8;
        locals.var_vgpld_dn9 = assign75880_e115388_d_n9;
        locals.var_vgpld_rv = 0.0;

        let (assign75890_e115397, assign75890_e115397_d_n0, assign75890_e115397_d_n2, assign75890_e115397_d_n4, assign75890_e115397_d_n5, assign75890_e115397_d_n6, assign75890_e115397_d_n7, assign75890_e115397_d_n8, assign75890_e115397_d_n9, assign75890_e115397_d_n10, assign75890_e115397_d_n11, assign75890_e115397_d_n14,) = {
    if (locals.var_flg_calcqover != 0.0) {
        let assign75890_e115391: f64 = (-locals.var_vxbgmtcl);
        let assign75890_e115394: f64 = (10.0 * 2.220446049250313e-16);
        let assign75890_e115395: f64 = (assign75890_e115391 + assign75890_e115394);
        (assign75890_e115395, (-locals.var_vxbgmtcl_dn0), (-locals.var_vxbgmtcl_dn2), (-locals.var_vxbgmtcl_dn4), (-locals.var_vxbgmtcl_dn5), (-locals.var_vxbgmtcl_dn6), (-locals.var_vxbgmtcl_dn7), (-locals.var_vxbgmtcl_dn8), (-locals.var_vxbgmtcl_dn9), (-locals.var_vxbgmtcl_dn10), (-locals.var_vxbgmtcl_dn11), (-locals.var_vxbgmtcl_dn14),)
    } else {
        (locals.var_vgb_fb_ld, locals.var_vgb_fb_ld_dn0, locals.var_vgb_fb_ld_dn2, locals.var_vgb_fb_ld_dn4, locals.var_vgb_fb_ld_dn5, locals.var_vgb_fb_ld_dn6, locals.var_vgb_fb_ld_dn7, locals.var_vgb_fb_ld_dn8, locals.var_vgb_fb_ld_dn9, locals.var_vgb_fb_ld_dn10, locals.var_vgb_fb_ld_dn11, locals.var_vgb_fb_ld_dn14,)
    }
};
        locals.var_vgb_fb_ld = assign75890_e115397;
        locals.var_vgb_fb_ld_dn0 = assign75890_e115397_d_n0;
        locals.var_vgb_fb_ld_dn2 = assign75890_e115397_d_n2;
        locals.var_vgb_fb_ld_dn4 = assign75890_e115397_d_n4;
        locals.var_vgb_fb_ld_dn5 = assign75890_e115397_d_n5;
        locals.var_vgb_fb_ld_dn6 = assign75890_e115397_d_n6;
        locals.var_vgb_fb_ld_dn7 = assign75890_e115397_d_n7;
        locals.var_vgb_fb_ld_dn8 = assign75890_e115397_d_n8;
        locals.var_vgb_fb_ld_dn9 = assign75890_e115397_d_n9;
        locals.var_vgb_fb_ld_dn10 = assign75890_e115397_d_n10;
        locals.var_vgb_fb_ld_dn11 = assign75890_e115397_d_n11;
        locals.var_vgb_fb_ld_dn14 = assign75890_e115397_d_n14;
        locals.var_vgb_fb_ld_rv = 0.0;

        let (assign75900_e115401, assign75900_e115401_d_n0, assign75900_e115401_d_n2, assign75900_e115401_d_n4, assign75900_e115401_d_n5, assign75900_e115401_d_n6, assign75900_e115401_d_n7, assign75900_e115401_d_n8, assign75900_e115401_d_n9, assign75900_e115401_d_n10, assign75900_e115401_d_n11, assign75900_e115401_d_n14,) = {
    if (locals.var_flg_calcqover != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_q_dep_ld__blk1763, locals.var_q_dep_ld__blk1763_dn0, locals.var_q_dep_ld__blk1763_dn2, locals.var_q_dep_ld__blk1763_dn4, locals.var_q_dep_ld__blk1763_dn5, locals.var_q_dep_ld__blk1763_dn6, locals.var_q_dep_ld__blk1763_dn7, locals.var_q_dep_ld__blk1763_dn8, locals.var_q_dep_ld__blk1763_dn9, locals.var_q_dep_ld__blk1763_dn10, locals.var_q_dep_ld__blk1763_dn11, locals.var_q_dep_ld__blk1763_dn14,)
    }
};
        locals.var_q_dep_ld__blk1763 = assign75900_e115401;
        locals.var_q_dep_ld__blk1763_dn0 = assign75900_e115401_d_n0;
        locals.var_q_dep_ld__blk1763_dn2 = assign75900_e115401_d_n2;
        locals.var_q_dep_ld__blk1763_dn4 = assign75900_e115401_d_n4;
        locals.var_q_dep_ld__blk1763_dn5 = assign75900_e115401_d_n5;
        locals.var_q_dep_ld__blk1763_dn6 = assign75900_e115401_d_n6;
        locals.var_q_dep_ld__blk1763_dn7 = assign75900_e115401_d_n7;
        locals.var_q_dep_ld__blk1763_dn8 = assign75900_e115401_d_n8;
        locals.var_q_dep_ld__blk1763_dn9 = assign75900_e115401_d_n9;
        locals.var_q_dep_ld__blk1763_dn10 = assign75900_e115401_d_n10;
        locals.var_q_dep_ld__blk1763_dn11 = assign75900_e115401_d_n11;
        locals.var_q_dep_ld__blk1763_dn14 = assign75900_e115401_d_n14;
        locals.var_q_dep_ld__blk1763_rv = 0.0;

        let (assign75910_e115407,) = {
    if (locals.var_flg_calcqover != 0.0) {
        let assign75910_e115405: f64 = (1.6021918e-19 * locals.var_nover_func);
        (assign75910_e115405,)
    } else {
        (locals.var_q_nsubld__blk1764,)
    }
};
        locals.var_q_nsubld__blk1764 = assign75910_e115407;
        locals.var_q_nsubld__blk1764_rv = 0.0;

        let (assign75920_e115413, assign75920_e115413_d_n0, assign75920_e115413_d_n2, assign75920_e115413_d_n4, assign75920_e115413_d_n5, assign75920_e115413_d_n6, assign75920_e115413_d_n7, assign75920_e115413_d_n8, assign75920_e115413_d_n9, assign75920_e115413_d_n10, assign75920_e115413_d_n11, assign75920_e115413_d_n14,) = {
    if (locals.var_flg_calcqover != 0.0) {
        let assign75920_e115411: f64 = (locals.var_nin / locals.var_nover_func);
        (assign75920_e115411, (locals.var_nin_dn0 / locals.var_nover_func), (locals.var_nin_dn2 / locals.var_nover_func), (locals.var_nin_dn4 / locals.var_nover_func), (locals.var_nin_dn5 / locals.var_nover_func), (locals.var_nin_dn6 / locals.var_nover_func), (locals.var_nin_dn7 / locals.var_nover_func), (locals.var_nin_dn8 / locals.var_nover_func), (locals.var_nin_dn9 / locals.var_nover_func), (locals.var_nin_dn10 / locals.var_nover_func), (locals.var_nin_dn11 / locals.var_nover_func), (locals.var_nin_dn14 / locals.var_nover_func),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign75920_e115413;
        locals.var_t0_dn0 = assign75920_e115413_d_n0;
        locals.var_t0_dn2 = assign75920_e115413_d_n2;
        locals.var_t0_dn4 = assign75920_e115413_d_n4;
        locals.var_t0_dn5 = assign75920_e115413_d_n5;
        locals.var_t0_dn6 = assign75920_e115413_d_n6;
        locals.var_t0_dn7 = assign75920_e115413_d_n7;
        locals.var_t0_dn8 = assign75920_e115413_d_n8;
        locals.var_t0_dn9 = assign75920_e115413_d_n9;
        locals.var_t0_dn10 = assign75920_e115413_d_n10;
        locals.var_t0_dn11 = assign75920_e115413_d_n11;
        locals.var_t0_dn14 = assign75920_e115413_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign75930_e115419, assign75930_e115419_d_n0, assign75930_e115419_d_n2, assign75930_e115419_d_n4, assign75930_e115419_d_n5, assign75930_e115419_d_n6, assign75930_e115419_d_n7, assign75930_e115419_d_n8, assign75930_e115419_d_n9, assign75930_e115419_d_n10, assign75930_e115419_d_n11, assign75930_e115419_d_n14,) = {
    if (locals.var_flg_calcqover != 0.0) {
        let assign75930_e115417: f64 = (locals.var_t0 * locals.var_t0);
        (assign75930_e115417, ((locals.var_t0_dn0 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn0)), ((locals.var_t0_dn2 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn2)), ((locals.var_t0_dn4 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn4)), ((locals.var_t0_dn5 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn5)), ((locals.var_t0_dn6 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn6)), ((locals.var_t0_dn7 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn7)), ((locals.var_t0_dn8 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn8)), ((locals.var_t0_dn9 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn9)), ((locals.var_t0_dn10 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn10)), ((locals.var_t0_dn11 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn11)), ((locals.var_t0_dn14 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn14)),)
    } else {
        (locals.var_cnst1over, locals.var_cnst1over_dn0, locals.var_cnst1over_dn2, locals.var_cnst1over_dn4, locals.var_cnst1over_dn5, locals.var_cnst1over_dn6, locals.var_cnst1over_dn7, locals.var_cnst1over_dn8, locals.var_cnst1over_dn9, locals.var_cnst1over_dn10, locals.var_cnst1over_dn11, locals.var_cnst1over_dn14,)
    }
};
        locals.var_cnst1over = assign75930_e115419;
        locals.var_cnst1over_dn0 = assign75930_e115419_d_n0;
        locals.var_cnst1over_dn2 = assign75930_e115419_d_n2;
        locals.var_cnst1over_dn4 = assign75930_e115419_d_n4;
        locals.var_cnst1over_dn5 = assign75930_e115419_d_n5;
        locals.var_cnst1over_dn6 = assign75930_e115419_d_n6;
        locals.var_cnst1over_dn7 = assign75930_e115419_d_n7;
        locals.var_cnst1over_dn8 = assign75930_e115419_d_n8;
        locals.var_cnst1over_dn9 = assign75930_e115419_d_n9;
        locals.var_cnst1over_dn10 = assign75930_e115419_d_n10;
        locals.var_cnst1over_dn11 = assign75930_e115419_d_n11;
        locals.var_cnst1over_dn14 = assign75930_e115419_d_n14;
        locals.var_cnst1over_rv = 0.0;

        let assign75940_e115422: f64 = (-locals.var_vxbgmtcl);
        let assign75940_e115423: f64 = (locals.var_beta * assign75940_e115422);
        let assign75940_e115425: f64 = if assign75940_e115423 >= 500.0 { 1.0 } else { 0.0 };
        locals.var_guard1781 = assign75940_e115425;
        locals.var_guard1781_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_287(
        locals: &mut StampLocals,
    ) {
        let (assign75950_e115440, assign75950_e115440_d_n0, assign75950_e115440_d_n2, assign75950_e115440_d_n4, assign75950_e115440_d_n5, assign75950_e115440_d_n6, assign75950_e115440_d_n7, assign75950_e115440_d_n8, assign75950_e115440_d_n9, assign75950_e115440_d_n10, assign75950_e115440_d_n11, assign75950_e115440_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1781 != 0.0)) {
        let assign75950_e115433: f64 = (-locals.var_vxbgmtcl);
        let assign75950_e115434: f64 = (locals.var_beta * assign75950_e115433);
        let assign75950_e115435: f64 = (1.0 + assign75950_e115434);
        let assign75950_e115437: f64 = (assign75950_e115435 - 500.0);
        let assign75950_e115438: f64 = (1.403592217853e217 * assign75950_e115437);
        (assign75950_e115438, (1.403592217853e217 * ((locals.var_beta_dn0 * assign75950_e115433) + (locals.var_beta * (-locals.var_vxbgmtcl_dn0)))), (1.403592217853e217 * ((locals.var_beta_dn2 * assign75950_e115433) + (locals.var_beta * (-locals.var_vxbgmtcl_dn2)))), (1.403592217853e217 * ((locals.var_beta_dn4 * assign75950_e115433) + (locals.var_beta * (-locals.var_vxbgmtcl_dn4)))), (1.403592217853e217 * ((locals.var_beta_dn5 * assign75950_e115433) + (locals.var_beta * (-locals.var_vxbgmtcl_dn5)))), (1.403592217853e217 * ((locals.var_beta_dn6 * assign75950_e115433) + (locals.var_beta * (-locals.var_vxbgmtcl_dn6)))), (1.403592217853e217 * ((locals.var_beta_dn7 * assign75950_e115433) + (locals.var_beta * (-locals.var_vxbgmtcl_dn7)))), (1.403592217853e217 * ((locals.var_beta_dn8 * assign75950_e115433) + (locals.var_beta * (-locals.var_vxbgmtcl_dn8)))), (1.403592217853e217 * ((locals.var_beta_dn9 * assign75950_e115433) + (locals.var_beta * (-locals.var_vxbgmtcl_dn9)))), (1.403592217853e217 * ((locals.var_beta_dn10 * assign75950_e115433) + (locals.var_beta * (-locals.var_vxbgmtcl_dn10)))), (1.403592217853e217 * ((locals.var_beta_dn11 * assign75950_e115433) + (locals.var_beta * (-locals.var_vxbgmtcl_dn11)))), (1.403592217853e217 * ((locals.var_beta_dn14 * assign75950_e115433) + (locals.var_beta * (-locals.var_vxbgmtcl_dn14)))),)
    } else {
        (locals.var_exp_bvbs, locals.var_exp_bvbs_dn0, locals.var_exp_bvbs_dn2, locals.var_exp_bvbs_dn4, locals.var_exp_bvbs_dn5, locals.var_exp_bvbs_dn6, locals.var_exp_bvbs_dn7, locals.var_exp_bvbs_dn8, locals.var_exp_bvbs_dn9, locals.var_exp_bvbs_dn10, locals.var_exp_bvbs_dn11, locals.var_exp_bvbs_dn14,)
    }
};
        locals.var_exp_bvbs = assign75950_e115440;
        locals.var_exp_bvbs_dn0 = assign75950_e115440_d_n0;
        locals.var_exp_bvbs_dn2 = assign75950_e115440_d_n2;
        locals.var_exp_bvbs_dn4 = assign75950_e115440_d_n4;
        locals.var_exp_bvbs_dn5 = assign75950_e115440_d_n5;
        locals.var_exp_bvbs_dn6 = assign75950_e115440_d_n6;
        locals.var_exp_bvbs_dn7 = assign75950_e115440_d_n7;
        locals.var_exp_bvbs_dn8 = assign75950_e115440_d_n8;
        locals.var_exp_bvbs_dn9 = assign75950_e115440_d_n9;
        locals.var_exp_bvbs_dn10 = assign75950_e115440_d_n10;
        locals.var_exp_bvbs_dn11 = assign75950_e115440_d_n11;
        locals.var_exp_bvbs_dn14 = assign75950_e115440_d_n14;
        locals.var_exp_bvbs_rv = 0.0;

        let (assign75960_e115446, assign75960_e115446_d_n0, assign75960_e115446_d_n2, assign75960_e115446_d_n4, assign75960_e115446_d_n5, assign75960_e115446_d_n6, assign75960_e115446_d_n7, assign75960_e115446_d_n8, assign75960_e115446_d_n9, assign75960_e115446_d_n10, assign75960_e115446_d_n11, assign75960_e115446_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1781 != 0.0)) {
        (1.403592217853e217, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign75960_e115446;
        locals.var_t0_dn0 = assign75960_e115446_d_n0;
        locals.var_t0_dn2 = assign75960_e115446_d_n2;
        locals.var_t0_dn4 = assign75960_e115446_d_n4;
        locals.var_t0_dn5 = assign75960_e115446_d_n5;
        locals.var_t0_dn6 = assign75960_e115446_d_n6;
        locals.var_t0_dn7 = assign75960_e115446_d_n7;
        locals.var_t0_dn8 = assign75960_e115446_d_n8;
        locals.var_t0_dn9 = assign75960_e115446_d_n9;
        locals.var_t0_dn10 = assign75960_e115446_d_n10;
        locals.var_t0_dn11 = assign75960_e115446_d_n11;
        locals.var_t0_dn14 = assign75960_e115446_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign75970_e115456, assign75970_e115456_d_n0, assign75970_e115456_d_n2, assign75970_e115456_d_n4, assign75970_e115456_d_n5, assign75970_e115456_d_n6, assign75970_e115456_d_n7, assign75970_e115456_d_n8, assign75970_e115456_d_n9, assign75970_e115456_d_n10, assign75970_e115456_d_n11, assign75970_e115456_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1781 == 0.0)) {
        let assign75970_e115453: f64 = (-locals.var_vxbgmtcl);
        let assign75970_e115454: f64 = (locals.var_beta * assign75970_e115453);
        (assign75970_e115454, ((locals.var_beta_dn0 * assign75970_e115453) + (locals.var_beta * (-locals.var_vxbgmtcl_dn0))), ((locals.var_beta_dn2 * assign75970_e115453) + (locals.var_beta * (-locals.var_vxbgmtcl_dn2))), ((locals.var_beta_dn4 * assign75970_e115453) + (locals.var_beta * (-locals.var_vxbgmtcl_dn4))), ((locals.var_beta_dn5 * assign75970_e115453) + (locals.var_beta * (-locals.var_vxbgmtcl_dn5))), ((locals.var_beta_dn6 * assign75970_e115453) + (locals.var_beta * (-locals.var_vxbgmtcl_dn6))), ((locals.var_beta_dn7 * assign75970_e115453) + (locals.var_beta * (-locals.var_vxbgmtcl_dn7))), ((locals.var_beta_dn8 * assign75970_e115453) + (locals.var_beta * (-locals.var_vxbgmtcl_dn8))), ((locals.var_beta_dn9 * assign75970_e115453) + (locals.var_beta * (-locals.var_vxbgmtcl_dn9))), ((locals.var_beta_dn10 * assign75970_e115453) + (locals.var_beta * (-locals.var_vxbgmtcl_dn10))), ((locals.var_beta_dn11 * assign75970_e115453) + (locals.var_beta * (-locals.var_vxbgmtcl_dn11))), ((locals.var_beta_dn14 * assign75970_e115453) + (locals.var_beta * (-locals.var_vxbgmtcl_dn14))),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign75970_e115456;
        locals.var_tmf1_dn0 = assign75970_e115456_d_n0;
        locals.var_tmf1_dn2 = assign75970_e115456_d_n2;
        locals.var_tmf1_dn4 = assign75970_e115456_d_n4;
        locals.var_tmf1_dn5 = assign75970_e115456_d_n5;
        locals.var_tmf1_dn6 = assign75970_e115456_d_n6;
        locals.var_tmf1_dn7 = assign75970_e115456_d_n7;
        locals.var_tmf1_dn8 = assign75970_e115456_d_n8;
        locals.var_tmf1_dn9 = assign75970_e115456_d_n9;
        locals.var_tmf1_dn10 = assign75970_e115456_d_n10;
        locals.var_tmf1_dn11 = assign75970_e115456_d_n11;
        locals.var_tmf1_dn14 = assign75970_e115456_d_n14;
        locals.var_tmf1_rv = 0.0;

        let (assign75980_e115463, assign75980_e115463_d_n0, assign75980_e115463_d_n2, assign75980_e115463_d_n4, assign75980_e115463_d_n5, assign75980_e115463_d_n6, assign75980_e115463_d_n7, assign75980_e115463_d_n8, assign75980_e115463_d_n9, assign75980_e115463_d_n10, assign75980_e115463_d_n11, assign75980_e115463_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1781 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_exp_bvbs, locals.var_exp_bvbs_dn0, locals.var_exp_bvbs_dn2, locals.var_exp_bvbs_dn4, locals.var_exp_bvbs_dn5, locals.var_exp_bvbs_dn6, locals.var_exp_bvbs_dn7, locals.var_exp_bvbs_dn8, locals.var_exp_bvbs_dn9, locals.var_exp_bvbs_dn10, locals.var_exp_bvbs_dn11, locals.var_exp_bvbs_dn14,)
    }
};
        locals.var_exp_bvbs = assign75980_e115463;
        locals.var_exp_bvbs_dn0 = assign75980_e115463_d_n0;
        locals.var_exp_bvbs_dn2 = assign75980_e115463_d_n2;
        locals.var_exp_bvbs_dn4 = assign75980_e115463_d_n4;
        locals.var_exp_bvbs_dn5 = assign75980_e115463_d_n5;
        locals.var_exp_bvbs_dn6 = assign75980_e115463_d_n6;
        locals.var_exp_bvbs_dn7 = assign75980_e115463_d_n7;
        locals.var_exp_bvbs_dn8 = assign75980_e115463_d_n8;
        locals.var_exp_bvbs_dn9 = assign75980_e115463_d_n9;
        locals.var_exp_bvbs_dn10 = assign75980_e115463_d_n10;
        locals.var_exp_bvbs_dn11 = assign75980_e115463_d_n11;
        locals.var_exp_bvbs_dn14 = assign75980_e115463_d_n14;
        locals.var_exp_bvbs_rv = 0.0;

        let mut assign75990_loop_guard: usize = 0;
        while {
            let assign75990_cond_e115471: f64 = if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1781 == 0.0)) && (locals.var_tmf1 >= 60.0)) { 1.0 } else { 0.0 };
            assign75990_cond_e115471 != 0.0
        } {
            assign75990_loop_guard += 1;
            assert!(assign75990_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign75990_body0_e115480, assign75990_body0_e115480_d_n0, assign75990_body0_e115480_d_n2, assign75990_body0_e115480_d_n4, assign75990_body0_e115480_d_n5, assign75990_body0_e115480_d_n6, assign75990_body0_e115480_d_n7, assign75990_body0_e115480_d_n8, assign75990_body0_e115480_d_n9, assign75990_body0_e115480_d_n10, assign75990_body0_e115480_d_n11, assign75990_body0_e115480_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1781 == 0.0)) {
        let assign75990_body0_e115478: f64 = (locals.var_exp_bvbs * 1.14200738981568e26);
        (assign75990_body0_e115478, (locals.var_exp_bvbs_dn0 * 1.14200738981568e26), (locals.var_exp_bvbs_dn2 * 1.14200738981568e26), (locals.var_exp_bvbs_dn4 * 1.14200738981568e26), (locals.var_exp_bvbs_dn5 * 1.14200738981568e26), (locals.var_exp_bvbs_dn6 * 1.14200738981568e26), (locals.var_exp_bvbs_dn7 * 1.14200738981568e26), (locals.var_exp_bvbs_dn8 * 1.14200738981568e26), (locals.var_exp_bvbs_dn9 * 1.14200738981568e26), (locals.var_exp_bvbs_dn10 * 1.14200738981568e26), (locals.var_exp_bvbs_dn11 * 1.14200738981568e26), (locals.var_exp_bvbs_dn14 * 1.14200738981568e26),)
    } else {
        (locals.var_exp_bvbs, locals.var_exp_bvbs_dn0, locals.var_exp_bvbs_dn2, locals.var_exp_bvbs_dn4, locals.var_exp_bvbs_dn5, locals.var_exp_bvbs_dn6, locals.var_exp_bvbs_dn7, locals.var_exp_bvbs_dn8, locals.var_exp_bvbs_dn9, locals.var_exp_bvbs_dn10, locals.var_exp_bvbs_dn11, locals.var_exp_bvbs_dn14,)
    }
};
            locals.var_exp_bvbs = assign75990_body0_e115480;
            locals.var_exp_bvbs_dn0 = assign75990_body0_e115480_d_n0;
            locals.var_exp_bvbs_dn2 = assign75990_body0_e115480_d_n2;
            locals.var_exp_bvbs_dn4 = assign75990_body0_e115480_d_n4;
            locals.var_exp_bvbs_dn5 = assign75990_body0_e115480_d_n5;
            locals.var_exp_bvbs_dn6 = assign75990_body0_e115480_d_n6;
            locals.var_exp_bvbs_dn7 = assign75990_body0_e115480_d_n7;
            locals.var_exp_bvbs_dn8 = assign75990_body0_e115480_d_n8;
            locals.var_exp_bvbs_dn9 = assign75990_body0_e115480_d_n9;
            locals.var_exp_bvbs_dn10 = assign75990_body0_e115480_d_n10;
            locals.var_exp_bvbs_dn11 = assign75990_body0_e115480_d_n11;
            locals.var_exp_bvbs_dn14 = assign75990_body0_e115480_d_n14;
            locals.var_exp_bvbs_rv = 0.0;
            let (assign75990_body1_e115489, assign75990_body1_e115489_d_n0, assign75990_body1_e115489_d_n2, assign75990_body1_e115489_d_n4, assign75990_body1_e115489_d_n5, assign75990_body1_e115489_d_n6, assign75990_body1_e115489_d_n7, assign75990_body1_e115489_d_n8, assign75990_body1_e115489_d_n9, assign75990_body1_e115489_d_n10, assign75990_body1_e115489_d_n11, assign75990_body1_e115489_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1781 == 0.0)) {
        let assign75990_body1_e115487: f64 = (locals.var_tmf1 - 60.0);
        (assign75990_body1_e115487, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
            locals.var_tmf1 = assign75990_body1_e115489;
            locals.var_tmf1_dn0 = assign75990_body1_e115489_d_n0;
            locals.var_tmf1_dn2 = assign75990_body1_e115489_d_n2;
            locals.var_tmf1_dn4 = assign75990_body1_e115489_d_n4;
            locals.var_tmf1_dn5 = assign75990_body1_e115489_d_n5;
            locals.var_tmf1_dn6 = assign75990_body1_e115489_d_n6;
            locals.var_tmf1_dn7 = assign75990_body1_e115489_d_n7;
            locals.var_tmf1_dn8 = assign75990_body1_e115489_d_n8;
            locals.var_tmf1_dn9 = assign75990_body1_e115489_d_n9;
            locals.var_tmf1_dn10 = assign75990_body1_e115489_d_n10;
            locals.var_tmf1_dn11 = assign75990_body1_e115489_d_n11;
            locals.var_tmf1_dn14 = assign75990_body1_e115489_d_n14;
            locals.var_tmf1_rv = 0.0;
        }

        let (assign76000_e115499, assign76000_e115499_d_n0, assign76000_e115499_d_n2, assign76000_e115499_d_n4, assign76000_e115499_d_n5, assign76000_e115499_d_n6, assign76000_e115499_d_n7, assign76000_e115499_d_n8, assign76000_e115499_d_n9, assign76000_e115499_d_n10, assign76000_e115499_d_n11, assign76000_e115499_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1781 == 0.0)) {
        let assign76000_e115496: f64 = (locals.var_tmf1).exp();
        let assign76000_e115497: f64 = (locals.var_exp_bvbs * assign76000_e115496);
        (assign76000_e115497, ((locals.var_exp_bvbs_dn0 * assign76000_e115496) + (locals.var_exp_bvbs * (assign76000_e115496 * locals.var_tmf1_dn0))), ((locals.var_exp_bvbs_dn2 * assign76000_e115496) + (locals.var_exp_bvbs * (assign76000_e115496 * locals.var_tmf1_dn2))), ((locals.var_exp_bvbs_dn4 * assign76000_e115496) + (locals.var_exp_bvbs * (assign76000_e115496 * locals.var_tmf1_dn4))), ((locals.var_exp_bvbs_dn5 * assign76000_e115496) + (locals.var_exp_bvbs * (assign76000_e115496 * locals.var_tmf1_dn5))), ((locals.var_exp_bvbs_dn6 * assign76000_e115496) + (locals.var_exp_bvbs * (assign76000_e115496 * locals.var_tmf1_dn6))), ((locals.var_exp_bvbs_dn7 * assign76000_e115496) + (locals.var_exp_bvbs * (assign76000_e115496 * locals.var_tmf1_dn7))), ((locals.var_exp_bvbs_dn8 * assign76000_e115496) + (locals.var_exp_bvbs * (assign76000_e115496 * locals.var_tmf1_dn8))), ((locals.var_exp_bvbs_dn9 * assign76000_e115496) + (locals.var_exp_bvbs * (assign76000_e115496 * locals.var_tmf1_dn9))), ((locals.var_exp_bvbs_dn10 * assign76000_e115496) + (locals.var_exp_bvbs * (assign76000_e115496 * locals.var_tmf1_dn10))), ((locals.var_exp_bvbs_dn11 * assign76000_e115496) + (locals.var_exp_bvbs * (assign76000_e115496 * locals.var_tmf1_dn11))), ((locals.var_exp_bvbs_dn14 * assign76000_e115496) + (locals.var_exp_bvbs * (assign76000_e115496 * locals.var_tmf1_dn14))),)
    } else {
        (locals.var_exp_bvbs, locals.var_exp_bvbs_dn0, locals.var_exp_bvbs_dn2, locals.var_exp_bvbs_dn4, locals.var_exp_bvbs_dn5, locals.var_exp_bvbs_dn6, locals.var_exp_bvbs_dn7, locals.var_exp_bvbs_dn8, locals.var_exp_bvbs_dn9, locals.var_exp_bvbs_dn10, locals.var_exp_bvbs_dn11, locals.var_exp_bvbs_dn14,)
    }
};
        locals.var_exp_bvbs = assign76000_e115499;
        locals.var_exp_bvbs_dn0 = assign76000_e115499_d_n0;
        locals.var_exp_bvbs_dn2 = assign76000_e115499_d_n2;
        locals.var_exp_bvbs_dn4 = assign76000_e115499_d_n4;
        locals.var_exp_bvbs_dn5 = assign76000_e115499_d_n5;
        locals.var_exp_bvbs_dn6 = assign76000_e115499_d_n6;
        locals.var_exp_bvbs_dn7 = assign76000_e115499_d_n7;
        locals.var_exp_bvbs_dn8 = assign76000_e115499_d_n8;
        locals.var_exp_bvbs_dn9 = assign76000_e115499_d_n9;
        locals.var_exp_bvbs_dn10 = assign76000_e115499_d_n10;
        locals.var_exp_bvbs_dn11 = assign76000_e115499_d_n11;
        locals.var_exp_bvbs_dn14 = assign76000_e115499_d_n14;
        locals.var_exp_bvbs_rv = 0.0;

        let (assign76010_e115506, assign76010_e115506_d_n0, assign76010_e115506_d_n2, assign76010_e115506_d_n4, assign76010_e115506_d_n5, assign76010_e115506_d_n6, assign76010_e115506_d_n7, assign76010_e115506_d_n8, assign76010_e115506_d_n9, assign76010_e115506_d_n10, assign76010_e115506_d_n11, assign76010_e115506_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1781 == 0.0)) {
        (locals.var_exp_bvbs, locals.var_exp_bvbs_dn0, locals.var_exp_bvbs_dn2, locals.var_exp_bvbs_dn4, locals.var_exp_bvbs_dn5, locals.var_exp_bvbs_dn6, locals.var_exp_bvbs_dn7, locals.var_exp_bvbs_dn8, locals.var_exp_bvbs_dn9, locals.var_exp_bvbs_dn10, locals.var_exp_bvbs_dn11, locals.var_exp_bvbs_dn14,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign76010_e115506;
        locals.var_t0_dn0 = assign76010_e115506_d_n0;
        locals.var_t0_dn2 = assign76010_e115506_d_n2;
        locals.var_t0_dn4 = assign76010_e115506_d_n4;
        locals.var_t0_dn5 = assign76010_e115506_d_n5;
        locals.var_t0_dn6 = assign76010_e115506_d_n6;
        locals.var_t0_dn7 = assign76010_e115506_d_n7;
        locals.var_t0_dn8 = assign76010_e115506_d_n8;
        locals.var_t0_dn9 = assign76010_e115506_d_n9;
        locals.var_t0_dn10 = assign76010_e115506_d_n10;
        locals.var_t0_dn11 = assign76010_e115506_d_n11;
        locals.var_t0_dn14 = assign76010_e115506_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign76020_e115519, assign76020_e115519_d_n0, assign76020_e115519_d_n2, assign76020_e115519_d_n4, assign76020_e115519_d_n5, assign76020_e115519_d_n6, assign76020_e115519_d_n7, assign76020_e115519_d_n8, assign76020_e115519_d_n9, assign76020_e115519_d_n10, assign76020_e115519_d_n11, assign76020_e115519_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) {
        let assign76020_e115511: f64 = (-locals.var_vgpld);
        let assign76020_e115513: f64 = (assign76020_e115511 * 0.5);
        let assign76020_e115515: f64 = (assign76020_e115513 - 0.5);
        let assign76020_e115517: f64 = (assign76020_e115515 - 1.0);
        (assign76020_e115517, 0.0, ((-locals.var_vgpld_dn2) * 0.5), 0.0, 0.0, 0.0, ((-locals.var_vgpld_dn7) * 0.5), ((-locals.var_vgpld_dn8) * 0.5), ((-locals.var_vgpld_dn9) * 0.5), 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign76020_e115519;
        locals.var_tmf1_dn0 = assign76020_e115519_d_n0;
        locals.var_tmf1_dn2 = assign76020_e115519_d_n2;
        locals.var_tmf1_dn4 = assign76020_e115519_d_n4;
        locals.var_tmf1_dn5 = assign76020_e115519_d_n5;
        locals.var_tmf1_dn6 = assign76020_e115519_d_n6;
        locals.var_tmf1_dn7 = assign76020_e115519_d_n7;
        locals.var_tmf1_dn8 = assign76020_e115519_d_n8;
        locals.var_tmf1_dn9 = assign76020_e115519_d_n9;
        locals.var_tmf1_dn10 = assign76020_e115519_d_n10;
        locals.var_tmf1_dn11 = assign76020_e115519_d_n11;
        locals.var_tmf1_dn14 = assign76020_e115519_d_n14;
        locals.var_tmf1_rv = 0.0;

        let (assign76030_e115529, assign76030_e115529_d_n0, assign76030_e115529_d_n2, assign76030_e115529_d_n4, assign76030_e115529_d_n5, assign76030_e115529_d_n6, assign76030_e115529_d_n7, assign76030_e115529_d_n8, assign76030_e115529_d_n9, assign76030_e115529_d_n10, assign76030_e115529_d_n11, assign76030_e115529_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) {
        let assign76030_e115525: f64 = (4.0 * 0.5);
        let assign76030_e115527: f64 = assign76030_e115525;
        (assign76030_e115527, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign76030_e115529;
        locals.var_tmf2_dn0 = assign76030_e115529_d_n0;
        locals.var_tmf2_dn2 = assign76030_e115529_d_n2;
        locals.var_tmf2_dn4 = assign76030_e115529_d_n4;
        locals.var_tmf2_dn5 = assign76030_e115529_d_n5;
        locals.var_tmf2_dn6 = assign76030_e115529_d_n6;
        locals.var_tmf2_dn7 = assign76030_e115529_d_n7;
        locals.var_tmf2_dn8 = assign76030_e115529_d_n8;
        locals.var_tmf2_dn9 = assign76030_e115529_d_n9;
        locals.var_tmf2_dn10 = assign76030_e115529_d_n10;
        locals.var_tmf2_dn11 = assign76030_e115529_d_n11;
        locals.var_tmf2_dn14 = assign76030_e115529_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign76040_e115541, assign76040_e115541_d_n0, assign76040_e115541_d_n2, assign76040_e115541_d_n4, assign76040_e115541_d_n5, assign76040_e115541_d_n6, assign76040_e115541_d_n7, assign76040_e115541_d_n8, assign76040_e115541_d_n9, assign76040_e115541_d_n10, assign76040_e115541_d_n11, assign76040_e115541_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) {
        let (assign76040_e115539, assign76040_e115539_d_n0, assign76040_e115539_d_n2, assign76040_e115539_d_n4, assign76040_e115539_d_n5, assign76040_e115539_d_n6, assign76040_e115539_d_n7, assign76040_e115539_d_n8, assign76040_e115539_d_n9, assign76040_e115539_d_n10, assign76040_e115539_d_n11, assign76040_e115539_d_n14,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
            } else {
                let assign76040_e115538: f64 = (-locals.var_tmf2);
                (assign76040_e115538, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn14),)
            }
        };
        (assign76040_e115539, assign76040_e115539_d_n0, assign76040_e115539_d_n2, assign76040_e115539_d_n4, assign76040_e115539_d_n5, assign76040_e115539_d_n6, assign76040_e115539_d_n7, assign76040_e115539_d_n8, assign76040_e115539_d_n9, assign76040_e115539_d_n10, assign76040_e115539_d_n11, assign76040_e115539_d_n14,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign76040_e115541;
        locals.var_tmf2_dn0 = assign76040_e115541_d_n0;
        locals.var_tmf2_dn2 = assign76040_e115541_d_n2;
        locals.var_tmf2_dn4 = assign76040_e115541_d_n4;
        locals.var_tmf2_dn5 = assign76040_e115541_d_n5;
        locals.var_tmf2_dn6 = assign76040_e115541_d_n6;
        locals.var_tmf2_dn7 = assign76040_e115541_d_n7;
        locals.var_tmf2_dn8 = assign76040_e115541_d_n8;
        locals.var_tmf2_dn9 = assign76040_e115541_d_n9;
        locals.var_tmf2_dn10 = assign76040_e115541_d_n10;
        locals.var_tmf2_dn11 = assign76040_e115541_d_n11;
        locals.var_tmf2_dn14 = assign76040_e115541_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign76050_e115552, assign76050_e115552_d_n0, assign76050_e115552_d_n2, assign76050_e115552_d_n4, assign76050_e115552_d_n5, assign76050_e115552_d_n6, assign76050_e115552_d_n7, assign76050_e115552_d_n8, assign76050_e115552_d_n9, assign76050_e115552_d_n10, assign76050_e115552_d_n11, assign76050_e115552_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) {
        let assign76050_e115547: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign76050_e115549: f64 = (assign76050_e115547 + locals.var_tmf2);
        let assign76050_e115550: f64 = (assign76050_e115549).sqrt();
        (assign76050_e115550, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign76050_e115550)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign76050_e115550)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign76050_e115550)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign76050_e115550)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign76050_e115550)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign76050_e115550)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign76050_e115550)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign76050_e115550)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign76050_e115550)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign76050_e115550)), ((((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)) + locals.var_tmf2_dn14) / (2.0 * assign76050_e115550)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign76050_e115552;
        locals.var_tmf2_dn0 = assign76050_e115552_d_n0;
        locals.var_tmf2_dn2 = assign76050_e115552_d_n2;
        locals.var_tmf2_dn4 = assign76050_e115552_d_n4;
        locals.var_tmf2_dn5 = assign76050_e115552_d_n5;
        locals.var_tmf2_dn6 = assign76050_e115552_d_n6;
        locals.var_tmf2_dn7 = assign76050_e115552_d_n7;
        locals.var_tmf2_dn8 = assign76050_e115552_d_n8;
        locals.var_tmf2_dn9 = assign76050_e115552_d_n9;
        locals.var_tmf2_dn10 = assign76050_e115552_d_n10;
        locals.var_tmf2_dn11 = assign76050_e115552_d_n11;
        locals.var_tmf2_dn14 = assign76050_e115552_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign76060_e115564, assign76060_e115564_d_n0, assign76060_e115564_d_n2, assign76060_e115564_d_n4, assign76060_e115564_d_n5, assign76060_e115564_d_n6, assign76060_e115564_d_n7, assign76060_e115564_d_n8, assign76060_e115564_d_n9, assign76060_e115564_d_n10, assign76060_e115564_d_n11, assign76060_e115564_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) {
        let assign76060_e115560: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign76060_e115561: f64 = (1.0 + assign76060_e115560);
        let assign76060_e115562: f64 = (0.5 * assign76060_e115561);
        (assign76060_e115562, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn11 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn14 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign76060_e115564;
        locals.var_t0_dn0 = assign76060_e115564_d_n0;
        locals.var_t0_dn2 = assign76060_e115564_d_n2;
        locals.var_t0_dn4 = assign76060_e115564_d_n4;
        locals.var_t0_dn5 = assign76060_e115564_d_n5;
        locals.var_t0_dn6 = assign76060_e115564_d_n6;
        locals.var_t0_dn7 = assign76060_e115564_d_n7;
        locals.var_t0_dn8 = assign76060_e115564_d_n8;
        locals.var_t0_dn9 = assign76060_e115564_d_n9;
        locals.var_t0_dn10 = assign76060_e115564_d_n10;
        locals.var_t0_dn11 = assign76060_e115564_d_n11;
        locals.var_t0_dn14 = assign76060_e115564_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign76070_e115576, assign76070_e115576_d_n0, assign76070_e115576_d_n2, assign76070_e115576_d_n4, assign76070_e115576_d_n5, assign76070_e115576_d_n6, assign76070_e115576_d_n7, assign76070_e115576_d_n8, assign76070_e115576_d_n9, assign76070_e115576_d_n10, assign76070_e115576_d_n11, assign76070_e115576_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) {
        let assign76070_e115572: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign76070_e115573: f64 = (0.5 * assign76070_e115572);
        let assign76070_e115574: f64 = (0.5 + assign76070_e115573);
        (assign76070_e115574, (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11)), (0.5 * (locals.var_tmf1_dn14 + locals.var_tmf2_dn14)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign76070_e115576;
        locals.var_t1_dn0 = assign76070_e115576_d_n0;
        locals.var_t1_dn2 = assign76070_e115576_d_n2;
        locals.var_t1_dn4 = assign76070_e115576_d_n4;
        locals.var_t1_dn5 = assign76070_e115576_d_n5;
        locals.var_t1_dn6 = assign76070_e115576_d_n6;
        locals.var_t1_dn7 = assign76070_e115576_d_n7;
        locals.var_t1_dn8 = assign76070_e115576_d_n8;
        locals.var_t1_dn9 = assign76070_e115576_d_n9;
        locals.var_t1_dn10 = assign76070_e115576_d_n10;
        locals.var_t1_dn11 = assign76070_e115576_d_n11;
        locals.var_t1_dn14 = assign76070_e115576_d_n14;
        locals.var_t1_rv = 0.0;

        let assign76080_e115579: f64 = (locals.var_vgpld + locals.var_vxbgmtcl);
        let assign76080_e115582: f64 = (-locals.var_t1);
        let assign76080_e115587: f64 = if ((assign76080_e115579 > assign76080_e115582) && (locals.var_t1 >= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1782 = assign76080_e115587;
        locals.var_guard1782_rv = 0.0;

        let (assign76090_e115601, assign76090_e115601_d_n0, assign76090_e115601_d_n2, assign76090_e115601_d_n4, assign76090_e115601_d_n5, assign76090_e115601_d_n6, assign76090_e115601_d_n7, assign76090_e115601_d_n8, assign76090_e115601_d_n9, assign76090_e115601_d_n10, assign76090_e115601_d_n11, assign76090_e115601_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1782 != 0.0)) {
        let assign76090_e115595: f64 = (locals.var_vgpld + locals.var_vxbgmtcl);
        let assign76090_e115597: f64 = assign76090_e115595;
        let assign76090_e115599: f64 = (assign76090_e115597 + locals.var_t1);
        (assign76090_e115599, (locals.var_vxbgmtcl_dn0 + locals.var_t1_dn0), ((locals.var_vgpld_dn2 + locals.var_vxbgmtcl_dn2) + locals.var_t1_dn2), (locals.var_vxbgmtcl_dn4 + locals.var_t1_dn4), (locals.var_vxbgmtcl_dn5 + locals.var_t1_dn5), (locals.var_vxbgmtcl_dn6 + locals.var_t1_dn6), ((locals.var_vgpld_dn7 + locals.var_vxbgmtcl_dn7) + locals.var_t1_dn7), ((locals.var_vgpld_dn8 + locals.var_vxbgmtcl_dn8) + locals.var_t1_dn8), ((locals.var_vgpld_dn9 + locals.var_vxbgmtcl_dn9) + locals.var_t1_dn9), (locals.var_vxbgmtcl_dn10 + locals.var_t1_dn10), (locals.var_vxbgmtcl_dn11 + locals.var_t1_dn11), (locals.var_vxbgmtcl_dn14 + locals.var_t1_dn14),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign76090_e115601;
        locals.var_tmf1_dn0 = assign76090_e115601_d_n0;
        locals.var_tmf1_dn2 = assign76090_e115601_d_n2;
        locals.var_tmf1_dn4 = assign76090_e115601_d_n4;
        locals.var_tmf1_dn5 = assign76090_e115601_d_n5;
        locals.var_tmf1_dn6 = assign76090_e115601_d_n6;
        locals.var_tmf1_dn7 = assign76090_e115601_d_n7;
        locals.var_tmf1_dn8 = assign76090_e115601_d_n8;
        locals.var_tmf1_dn9 = assign76090_e115601_d_n9;
        locals.var_tmf1_dn10 = assign76090_e115601_d_n10;
        locals.var_tmf1_dn11 = assign76090_e115601_d_n11;
        locals.var_tmf1_dn14 = assign76090_e115601_d_n14;
        locals.var_tmf1_rv = 0.0;

        let (assign76100_e115611, assign76100_e115611_d_n0, assign76100_e115611_d_n2, assign76100_e115611_d_n4, assign76100_e115611_d_n5, assign76100_e115611_d_n6, assign76100_e115611_d_n7, assign76100_e115611_d_n8, assign76100_e115611_d_n9, assign76100_e115611_d_n10, assign76100_e115611_d_n11, assign76100_e115611_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1782 != 0.0)) {
        let assign76100_e115609: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign76100_e115609, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)), ((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)), ((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)), ((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn4, locals.var_x2_dn5, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn8, locals.var_x2_dn9, locals.var_x2_dn10, locals.var_x2_dn11, locals.var_x2_dn14,)
    }
};
        locals.var_x2 = assign76100_e115611;
        locals.var_x2_dn0 = assign76100_e115611_d_n0;
        locals.var_x2_dn2 = assign76100_e115611_d_n2;
        locals.var_x2_dn4 = assign76100_e115611_d_n4;
        locals.var_x2_dn5 = assign76100_e115611_d_n5;
        locals.var_x2_dn6 = assign76100_e115611_d_n6;
        locals.var_x2_dn7 = assign76100_e115611_d_n7;
        locals.var_x2_dn8 = assign76100_e115611_d_n8;
        locals.var_x2_dn9 = assign76100_e115611_d_n9;
        locals.var_x2_dn10 = assign76100_e115611_d_n10;
        locals.var_x2_dn11 = assign76100_e115611_d_n11;
        locals.var_x2_dn14 = assign76100_e115611_d_n14;
        locals.var_x2_rv = 0.0;

        let (assign76110_e115621, assign76110_e115621_d_n0, assign76110_e115621_d_n2, assign76110_e115621_d_n4, assign76110_e115621_d_n5, assign76110_e115621_d_n6, assign76110_e115621_d_n7, assign76110_e115621_d_n8, assign76110_e115621_d_n9, assign76110_e115621_d_n10, assign76110_e115621_d_n11, assign76110_e115621_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1782 != 0.0)) {
        let assign76110_e115619: f64 = (locals.var_t1 * locals.var_t1);
        (assign76110_e115619, ((locals.var_t1_dn0 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn0)), ((locals.var_t1_dn2 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn2)), ((locals.var_t1_dn4 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn4)), ((locals.var_t1_dn5 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn5)), ((locals.var_t1_dn6 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn6)), ((locals.var_t1_dn7 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn7)), ((locals.var_t1_dn8 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn8)), ((locals.var_t1_dn9 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn9)), ((locals.var_t1_dn10 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn10)), ((locals.var_t1_dn11 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn11)), ((locals.var_t1_dn14 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn14)),)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn4, locals.var_xmax2_dn5, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn8, locals.var_xmax2_dn9, locals.var_xmax2_dn10, locals.var_xmax2_dn11, locals.var_xmax2_dn14,)
    }
};
        locals.var_xmax2 = assign76110_e115621;
        locals.var_xmax2_dn0 = assign76110_e115621_d_n0;
        locals.var_xmax2_dn2 = assign76110_e115621_d_n2;
        locals.var_xmax2_dn4 = assign76110_e115621_d_n4;
        locals.var_xmax2_dn5 = assign76110_e115621_d_n5;
        locals.var_xmax2_dn6 = assign76110_e115621_d_n6;
        locals.var_xmax2_dn7 = assign76110_e115621_d_n7;
        locals.var_xmax2_dn8 = assign76110_e115621_d_n8;
        locals.var_xmax2_dn9 = assign76110_e115621_d_n9;
        locals.var_xmax2_dn10 = assign76110_e115621_d_n10;
        locals.var_xmax2_dn11 = assign76110_e115621_d_n11;
        locals.var_xmax2_dn14 = assign76110_e115621_d_n14;
        locals.var_xmax2_rv = 0.0;

        let (assign76120_e115629, assign76120_e115629_d_n0, assign76120_e115629_d_n2, assign76120_e115629_d_n4, assign76120_e115629_d_n5, assign76120_e115629_d_n6, assign76120_e115629_d_n7, assign76120_e115629_d_n8, assign76120_e115629_d_n9, assign76120_e115629_d_n10, assign76120_e115629_d_n11, assign76120_e115629_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1782 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign76120_e115629;
        locals.var_xp_dn0 = assign76120_e115629_d_n0;
        locals.var_xp_dn2 = assign76120_e115629_d_n2;
        locals.var_xp_dn4 = assign76120_e115629_d_n4;
        locals.var_xp_dn5 = assign76120_e115629_d_n5;
        locals.var_xp_dn6 = assign76120_e115629_d_n6;
        locals.var_xp_dn7 = assign76120_e115629_d_n7;
        locals.var_xp_dn8 = assign76120_e115629_d_n8;
        locals.var_xp_dn9 = assign76120_e115629_d_n9;
        locals.var_xp_dn10 = assign76120_e115629_d_n10;
        locals.var_xp_dn11 = assign76120_e115629_d_n11;
        locals.var_xp_dn14 = assign76120_e115629_d_n14;
        locals.var_xp_rv = 0.0;

        let (assign76130_e115637, assign76130_e115637_d_n0, assign76130_e115637_d_n2, assign76130_e115637_d_n4, assign76130_e115637_d_n5, assign76130_e115637_d_n6, assign76130_e115637_d_n7, assign76130_e115637_d_n8, assign76130_e115637_d_n9, assign76130_e115637_d_n10, assign76130_e115637_d_n11, assign76130_e115637_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1782 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign76130_e115637;
        locals.var_xmp_dn0 = assign76130_e115637_d_n0;
        locals.var_xmp_dn2 = assign76130_e115637_d_n2;
        locals.var_xmp_dn4 = assign76130_e115637_d_n4;
        locals.var_xmp_dn5 = assign76130_e115637_d_n5;
        locals.var_xmp_dn6 = assign76130_e115637_d_n6;
        locals.var_xmp_dn7 = assign76130_e115637_d_n7;
        locals.var_xmp_dn8 = assign76130_e115637_d_n8;
        locals.var_xmp_dn9 = assign76130_e115637_d_n9;
        locals.var_xmp_dn10 = assign76130_e115637_d_n10;
        locals.var_xmp_dn11 = assign76130_e115637_d_n11;
        locals.var_xmp_dn14 = assign76130_e115637_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign76140_e115645,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1782 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign76140_e115645;
        locals.var_m0_rv = 0.0;

        let (assign76150_e115653,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1782 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign76150_e115653;
        locals.var_mm_rv = 0.0;

        let (assign76160_e115661, assign76160_e115661_d_n0, assign76160_e115661_d_n2, assign76160_e115661_d_n4, assign76160_e115661_d_n5, assign76160_e115661_d_n6, assign76160_e115661_d_n7, assign76160_e115661_d_n8, assign76160_e115661_d_n9, assign76160_e115661_d_n10, assign76160_e115661_d_n11, assign76160_e115661_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1782 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign76160_e115661;
        locals.var_arg_dn0 = assign76160_e115661_d_n0;
        locals.var_arg_dn2 = assign76160_e115661_d_n2;
        locals.var_arg_dn4 = assign76160_e115661_d_n4;
        locals.var_arg_dn5 = assign76160_e115661_d_n5;
        locals.var_arg_dn6 = assign76160_e115661_d_n6;
        locals.var_arg_dn7 = assign76160_e115661_d_n7;
        locals.var_arg_dn8 = assign76160_e115661_d_n8;
        locals.var_arg_dn9 = assign76160_e115661_d_n9;
        locals.var_arg_dn10 = assign76160_e115661_d_n10;
        locals.var_arg_dn11 = assign76160_e115661_d_n11;
        locals.var_arg_dn14 = assign76160_e115661_d_n14;
        locals.var_arg_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_288(
        locals: &mut StampLocals,
    ) {
        let (assign76170_e115669, assign76170_e115669_d_n0, assign76170_e115669_d_n2, assign76170_e115669_d_n4, assign76170_e115669_d_n5, assign76170_e115669_d_n6, assign76170_e115669_d_n7, assign76170_e115669_d_n8, assign76170_e115669_d_n9, assign76170_e115669_d_n10, assign76170_e115669_d_n11, assign76170_e115669_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1782 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign76170_e115669;
        locals.var_dnm_dn0 = assign76170_e115669_d_n0;
        locals.var_dnm_dn2 = assign76170_e115669_d_n2;
        locals.var_dnm_dn4 = assign76170_e115669_d_n4;
        locals.var_dnm_dn5 = assign76170_e115669_d_n5;
        locals.var_dnm_dn6 = assign76170_e115669_d_n6;
        locals.var_dnm_dn7 = assign76170_e115669_d_n7;
        locals.var_dnm_dn8 = assign76170_e115669_d_n8;
        locals.var_dnm_dn9 = assign76170_e115669_d_n9;
        locals.var_dnm_dn10 = assign76170_e115669_d_n10;
        locals.var_dnm_dn11 = assign76170_e115669_d_n11;
        locals.var_dnm_dn14 = assign76170_e115669_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign76180_e115679, assign76180_e115679_d_n0, assign76180_e115679_d_n2, assign76180_e115679_d_n4, assign76180_e115679_d_n5, assign76180_e115679_d_n6, assign76180_e115679_d_n7, assign76180_e115679_d_n8, assign76180_e115679_d_n9, assign76180_e115679_d_n10, assign76180_e115679_d_n11, assign76180_e115679_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1782 != 0.0)) {
        let assign76180_e115677: f64 = (locals.var_xp * locals.var_x2);
        (assign76180_e115677, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign76180_e115679;
        locals.var_xp_dn0 = assign76180_e115679_d_n0;
        locals.var_xp_dn2 = assign76180_e115679_d_n2;
        locals.var_xp_dn4 = assign76180_e115679_d_n4;
        locals.var_xp_dn5 = assign76180_e115679_d_n5;
        locals.var_xp_dn6 = assign76180_e115679_d_n6;
        locals.var_xp_dn7 = assign76180_e115679_d_n7;
        locals.var_xp_dn8 = assign76180_e115679_d_n8;
        locals.var_xp_dn9 = assign76180_e115679_d_n9;
        locals.var_xp_dn10 = assign76180_e115679_d_n10;
        locals.var_xp_dn11 = assign76180_e115679_d_n11;
        locals.var_xp_dn14 = assign76180_e115679_d_n14;
        locals.var_xp_rv = 0.0;

        let (assign76190_e115689, assign76190_e115689_d_n0, assign76190_e115689_d_n2, assign76190_e115689_d_n4, assign76190_e115689_d_n5, assign76190_e115689_d_n6, assign76190_e115689_d_n7, assign76190_e115689_d_n8, assign76190_e115689_d_n9, assign76190_e115689_d_n10, assign76190_e115689_d_n11, assign76190_e115689_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1782 != 0.0)) {
        let assign76190_e115687: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign76190_e115687, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign76190_e115689;
        locals.var_xmp_dn0 = assign76190_e115689_d_n0;
        locals.var_xmp_dn2 = assign76190_e115689_d_n2;
        locals.var_xmp_dn4 = assign76190_e115689_d_n4;
        locals.var_xmp_dn5 = assign76190_e115689_d_n5;
        locals.var_xmp_dn6 = assign76190_e115689_d_n6;
        locals.var_xmp_dn7 = assign76190_e115689_d_n7;
        locals.var_xmp_dn8 = assign76190_e115689_d_n8;
        locals.var_xmp_dn9 = assign76190_e115689_d_n9;
        locals.var_xmp_dn10 = assign76190_e115689_d_n10;
        locals.var_xmp_dn11 = assign76190_e115689_d_n11;
        locals.var_xmp_dn14 = assign76190_e115689_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign76200_e115699, assign76200_e115699_d_n0, assign76200_e115699_d_n2, assign76200_e115699_d_n4, assign76200_e115699_d_n5, assign76200_e115699_d_n6, assign76200_e115699_d_n7, assign76200_e115699_d_n8, assign76200_e115699_d_n9, assign76200_e115699_d_n10, assign76200_e115699_d_n11, assign76200_e115699_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1782 != 0.0)) {
        let assign76200_e115697: f64 = (locals.var_xp + locals.var_xmp);
        (assign76200_e115697, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn4 + locals.var_xmp_dn4), (locals.var_xp_dn5 + locals.var_xmp_dn5), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn8 + locals.var_xmp_dn8), (locals.var_xp_dn9 + locals.var_xmp_dn9), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn11 + locals.var_xmp_dn11), (locals.var_xp_dn14 + locals.var_xmp_dn14),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign76200_e115699;
        locals.var_arg_dn0 = assign76200_e115699_d_n0;
        locals.var_arg_dn2 = assign76200_e115699_d_n2;
        locals.var_arg_dn4 = assign76200_e115699_d_n4;
        locals.var_arg_dn5 = assign76200_e115699_d_n5;
        locals.var_arg_dn6 = assign76200_e115699_d_n6;
        locals.var_arg_dn7 = assign76200_e115699_d_n7;
        locals.var_arg_dn8 = assign76200_e115699_d_n8;
        locals.var_arg_dn9 = assign76200_e115699_d_n9;
        locals.var_arg_dn10 = assign76200_e115699_d_n10;
        locals.var_arg_dn11 = assign76200_e115699_d_n11;
        locals.var_arg_dn14 = assign76200_e115699_d_n14;
        locals.var_arg_rv = 0.0;

        let (assign76210_e115707, assign76210_e115707_d_n0, assign76210_e115707_d_n2, assign76210_e115707_d_n4, assign76210_e115707_d_n5, assign76210_e115707_d_n6, assign76210_e115707_d_n7, assign76210_e115707_d_n8, assign76210_e115707_d_n9, assign76210_e115707_d_n10, assign76210_e115707_d_n11, assign76210_e115707_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1782 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign76210_e115707;
        locals.var_dnm_dn0 = assign76210_e115707_d_n0;
        locals.var_dnm_dn2 = assign76210_e115707_d_n2;
        locals.var_dnm_dn4 = assign76210_e115707_d_n4;
        locals.var_dnm_dn5 = assign76210_e115707_d_n5;
        locals.var_dnm_dn6 = assign76210_e115707_d_n6;
        locals.var_dnm_dn7 = assign76210_e115707_d_n7;
        locals.var_dnm_dn8 = assign76210_e115707_d_n8;
        locals.var_dnm_dn9 = assign76210_e115707_d_n9;
        locals.var_dnm_dn10 = assign76210_e115707_d_n10;
        locals.var_dnm_dn11 = assign76210_e115707_d_n11;
        locals.var_dnm_dn14 = assign76210_e115707_d_n14;
        locals.var_dnm_rv = 0.0;

        let assign76220_e115722: f64 = if ((((1.0 == 1.0) || (1.0 == 2.0)) || (1.0 == 4.0)) || (1.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard1783 = assign76220_e115722;
        locals.var_guard1783_rv = 0.0;

        let assign76230_e115725: f64 = if 1.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1784 = assign76230_e115725;
        locals.var_guard1784_rv = 0.0;

        let (assign76240_e115737,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1782 != 0.0)) && (locals.var_guard1783 != 0.0)) && (locals.var_guard1784 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign76240_e115737;
        locals.var_mm_rv = 0.0;

        let assign76250_e115740: f64 = if 1.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard1785 = assign76250_e115740;
        locals.var_guard1785_rv = 0.0;

        let (assign76260_e115755,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1782 != 0.0)) && (locals.var_guard1783 != 0.0)) && (locals.var_guard1784 == 0.0)) && (locals.var_guard1785 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign76260_e115755;
        locals.var_mm_rv = 0.0;

        let assign76270_e115758: f64 = if 1.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard1786 = assign76270_e115758;
        locals.var_guard1786_rv = 0.0;

        let (assign76280_e115776,) = {
    if (((((((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1782 != 0.0)) && (locals.var_guard1783 != 0.0)) && (locals.var_guard1784 == 0.0)) && (locals.var_guard1785 == 0.0)) && (locals.var_guard1786 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign76280_e115776;
        locals.var_mm_rv = 0.0;

        let assign76290_e115779: f64 = if 1.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard1787 = assign76290_e115779;
        locals.var_guard1787_rv = 0.0;

        let (assign76300_e115800,) = {
    if ((((((((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1782 != 0.0)) && (locals.var_guard1783 != 0.0)) && (locals.var_guard1784 == 0.0)) && (locals.var_guard1785 == 0.0)) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1787 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign76300_e115800;
        locals.var_mm_rv = 0.0;

        let (assign76310_e115810,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1782 != 0.0)) && (locals.var_guard1783 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign76310_e115810;
        locals.var_m0_rv = 0.0;

        let mut assign76320_loop_guard: usize = 0;
        while {
            let assign76320_cond_e115821: f64 = if (((((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1782 != 0.0)) && (locals.var_guard1783 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign76320_cond_e115821 != 0.0
        } {
            assign76320_loop_guard += 1;
            assert!(assign76320_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign76320_body0_e115832, assign76320_body0_e115832_d_n0, assign76320_body0_e115832_d_n2, assign76320_body0_e115832_d_n4, assign76320_body0_e115832_d_n5, assign76320_body0_e115832_d_n6, assign76320_body0_e115832_d_n7, assign76320_body0_e115832_d_n8, assign76320_body0_e115832_d_n9, assign76320_body0_e115832_d_n10, assign76320_body0_e115832_d_n11, assign76320_body0_e115832_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1782 != 0.0)) && (locals.var_guard1783 != 0.0)) {
        let assign76320_body0_e115830: f64 = (locals.var_dnm).sqrt();
        (assign76320_body0_e115830, (locals.var_dnm_dn0 / (2.0 * assign76320_body0_e115830)), (locals.var_dnm_dn2 / (2.0 * assign76320_body0_e115830)), (locals.var_dnm_dn4 / (2.0 * assign76320_body0_e115830)), (locals.var_dnm_dn5 / (2.0 * assign76320_body0_e115830)), (locals.var_dnm_dn6 / (2.0 * assign76320_body0_e115830)), (locals.var_dnm_dn7 / (2.0 * assign76320_body0_e115830)), (locals.var_dnm_dn8 / (2.0 * assign76320_body0_e115830)), (locals.var_dnm_dn9 / (2.0 * assign76320_body0_e115830)), (locals.var_dnm_dn10 / (2.0 * assign76320_body0_e115830)), (locals.var_dnm_dn11 / (2.0 * assign76320_body0_e115830)), (locals.var_dnm_dn14 / (2.0 * assign76320_body0_e115830)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
            locals.var_dnm = assign76320_body0_e115832;
            locals.var_dnm_dn0 = assign76320_body0_e115832_d_n0;
            locals.var_dnm_dn2 = assign76320_body0_e115832_d_n2;
            locals.var_dnm_dn4 = assign76320_body0_e115832_d_n4;
            locals.var_dnm_dn5 = assign76320_body0_e115832_d_n5;
            locals.var_dnm_dn6 = assign76320_body0_e115832_d_n6;
            locals.var_dnm_dn7 = assign76320_body0_e115832_d_n7;
            locals.var_dnm_dn8 = assign76320_body0_e115832_d_n8;
            locals.var_dnm_dn9 = assign76320_body0_e115832_d_n9;
            locals.var_dnm_dn10 = assign76320_body0_e115832_d_n10;
            locals.var_dnm_dn11 = assign76320_body0_e115832_d_n11;
            locals.var_dnm_dn14 = assign76320_body0_e115832_d_n14;
            locals.var_dnm_rv = 0.0;
            let (assign76320_body1_e115844,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1782 != 0.0)) && (locals.var_guard1783 != 0.0)) {
        let assign76320_body1_e115842: f64 = (locals.var_m0 + 1.0);
        (assign76320_body1_e115842,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign76320_body1_e115844;
            locals.var_m0_rv = 0.0;
        }

        let (assign76330_e115866, assign76330_e115866_d_n0, assign76330_e115866_d_n2, assign76330_e115866_d_n4, assign76330_e115866_d_n5, assign76330_e115866_d_n6, assign76330_e115866_d_n7, assign76330_e115866_d_n8, assign76330_e115866_d_n9, assign76330_e115866_d_n10, assign76330_e115866_d_n11, assign76330_e115866_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1782 != 0.0)) && (locals.var_guard1783 == 0.0)) {
        let (assign76330_e115864, assign76330_e115864_d_n0, assign76330_e115864_d_n2, assign76330_e115864_d_n4, assign76330_e115864_d_n5, assign76330_e115864_d_n6, assign76330_e115864_d_n7, assign76330_e115864_d_n8, assign76330_e115864_d_n9, assign76330_e115864_d_n10, assign76330_e115864_d_n11, assign76330_e115864_d_n14,) = {
            if (locals.var_dnm == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign76330_e115861: f64 = 2.0;
                let assign76330_e115862: f64 = (1.0 / assign76330_e115861);
                let assign76330_e115863: f64 = (locals.var_dnm).powf(assign76330_e115862);
                (assign76330_e115863, if 0.0 == 0.0 && ((assign76330_e115862) as f64).is_finite() && ((assign76330_e115862) as f64).fract() == 0.0 { if assign76330_e115862 == 0.0 { 0.0 } else { (assign76330_e115862 * ((locals.var_dnm).powf(assign76330_e115862 - 1.0) * locals.var_dnm_dn0)) } } else { (assign76330_e115863 * (assign76330_e115862 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign76330_e115862) as f64).is_finite() && ((assign76330_e115862) as f64).fract() == 0.0 { if assign76330_e115862 == 0.0 { 0.0 } else { (assign76330_e115862 * ((locals.var_dnm).powf(assign76330_e115862 - 1.0) * locals.var_dnm_dn2)) } } else { (assign76330_e115863 * (assign76330_e115862 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign76330_e115862) as f64).is_finite() && ((assign76330_e115862) as f64).fract() == 0.0 { if assign76330_e115862 == 0.0 { 0.0 } else { (assign76330_e115862 * ((locals.var_dnm).powf(assign76330_e115862 - 1.0) * locals.var_dnm_dn4)) } } else { (assign76330_e115863 * (assign76330_e115862 * (locals.var_dnm_dn4 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign76330_e115862) as f64).is_finite() && ((assign76330_e115862) as f64).fract() == 0.0 { if assign76330_e115862 == 0.0 { 0.0 } else { (assign76330_e115862 * ((locals.var_dnm).powf(assign76330_e115862 - 1.0) * locals.var_dnm_dn5)) } } else { (assign76330_e115863 * (assign76330_e115862 * (locals.var_dnm_dn5 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign76330_e115862) as f64).is_finite() && ((assign76330_e115862) as f64).fract() == 0.0 { if assign76330_e115862 == 0.0 { 0.0 } else { (assign76330_e115862 * ((locals.var_dnm).powf(assign76330_e115862 - 1.0) * locals.var_dnm_dn6)) } } else { (assign76330_e115863 * (assign76330_e115862 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign76330_e115862) as f64).is_finite() && ((assign76330_e115862) as f64).fract() == 0.0 { if assign76330_e115862 == 0.0 { 0.0 } else { (assign76330_e115862 * ((locals.var_dnm).powf(assign76330_e115862 - 1.0) * locals.var_dnm_dn7)) } } else { (assign76330_e115863 * (assign76330_e115862 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign76330_e115862) as f64).is_finite() && ((assign76330_e115862) as f64).fract() == 0.0 { if assign76330_e115862 == 0.0 { 0.0 } else { (assign76330_e115862 * ((locals.var_dnm).powf(assign76330_e115862 - 1.0) * locals.var_dnm_dn8)) } } else { (assign76330_e115863 * (assign76330_e115862 * (locals.var_dnm_dn8 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign76330_e115862) as f64).is_finite() && ((assign76330_e115862) as f64).fract() == 0.0 { if assign76330_e115862 == 0.0 { 0.0 } else { (assign76330_e115862 * ((locals.var_dnm).powf(assign76330_e115862 - 1.0) * locals.var_dnm_dn9)) } } else { (assign76330_e115863 * (assign76330_e115862 * (locals.var_dnm_dn9 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign76330_e115862) as f64).is_finite() && ((assign76330_e115862) as f64).fract() == 0.0 { if assign76330_e115862 == 0.0 { 0.0 } else { (assign76330_e115862 * ((locals.var_dnm).powf(assign76330_e115862 - 1.0) * locals.var_dnm_dn10)) } } else { (assign76330_e115863 * (assign76330_e115862 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign76330_e115862) as f64).is_finite() && ((assign76330_e115862) as f64).fract() == 0.0 { if assign76330_e115862 == 0.0 { 0.0 } else { (assign76330_e115862 * ((locals.var_dnm).powf(assign76330_e115862 - 1.0) * locals.var_dnm_dn11)) } } else { (assign76330_e115863 * (assign76330_e115862 * (locals.var_dnm_dn11 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign76330_e115862) as f64).is_finite() && ((assign76330_e115862) as f64).fract() == 0.0 { if assign76330_e115862 == 0.0 { 0.0 } else { (assign76330_e115862 * ((locals.var_dnm).powf(assign76330_e115862 - 1.0) * locals.var_dnm_dn14)) } } else { (assign76330_e115863 * (assign76330_e115862 * (locals.var_dnm_dn14 / locals.var_dnm))) },)
            }
        };
        (assign76330_e115864, assign76330_e115864_d_n0, assign76330_e115864_d_n2, assign76330_e115864_d_n4, assign76330_e115864_d_n5, assign76330_e115864_d_n6, assign76330_e115864_d_n7, assign76330_e115864_d_n8, assign76330_e115864_d_n9, assign76330_e115864_d_n10, assign76330_e115864_d_n11, assign76330_e115864_d_n14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign76330_e115866;
        locals.var_dnm_dn0 = assign76330_e115866_d_n0;
        locals.var_dnm_dn2 = assign76330_e115866_d_n2;
        locals.var_dnm_dn4 = assign76330_e115866_d_n4;
        locals.var_dnm_dn5 = assign76330_e115866_d_n5;
        locals.var_dnm_dn6 = assign76330_e115866_d_n6;
        locals.var_dnm_dn7 = assign76330_e115866_d_n7;
        locals.var_dnm_dn8 = assign76330_e115866_d_n8;
        locals.var_dnm_dn9 = assign76330_e115866_d_n9;
        locals.var_dnm_dn10 = assign76330_e115866_d_n10;
        locals.var_dnm_dn11 = assign76330_e115866_d_n11;
        locals.var_dnm_dn14 = assign76330_e115866_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign76340_e115876, assign76340_e115876_d_n0, assign76340_e115876_d_n2, assign76340_e115876_d_n4, assign76340_e115876_d_n5, assign76340_e115876_d_n6, assign76340_e115876_d_n7, assign76340_e115876_d_n8, assign76340_e115876_d_n9, assign76340_e115876_d_n10, assign76340_e115876_d_n11, assign76340_e115876_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1782 != 0.0)) {
        let assign76340_e115874: f64 = (1.0 / locals.var_dnm);
        (assign76340_e115874, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn4 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn5 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn8 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn9 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn11 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn14 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign76340_e115876;
        locals.var_dnm_dn0 = assign76340_e115876_d_n0;
        locals.var_dnm_dn2 = assign76340_e115876_d_n2;
        locals.var_dnm_dn4 = assign76340_e115876_d_n4;
        locals.var_dnm_dn5 = assign76340_e115876_d_n5;
        locals.var_dnm_dn6 = assign76340_e115876_d_n6;
        locals.var_dnm_dn7 = assign76340_e115876_d_n7;
        locals.var_dnm_dn8 = assign76340_e115876_d_n8;
        locals.var_dnm_dn9 = assign76340_e115876_d_n9;
        locals.var_dnm_dn10 = assign76340_e115876_d_n10;
        locals.var_dnm_dn11 = assign76340_e115876_d_n11;
        locals.var_dnm_dn14 = assign76340_e115876_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign76350_e115888, assign76350_e115888_d_n0, assign76350_e115888_d_n2, assign76350_e115888_d_n4, assign76350_e115888_d_n5, assign76350_e115888_d_n6, assign76350_e115888_d_n7, assign76350_e115888_d_n8, assign76350_e115888_d_n9, assign76350_e115888_d_n10, assign76350_e115888_d_n11, assign76350_e115888_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1782 != 0.0)) {
        let assign76350_e115884: f64 = (locals.var_tmf1 * locals.var_t1);
        let assign76350_e115886: f64 = (assign76350_e115884 * locals.var_dnm);
        (assign76350_e115886, ((((locals.var_tmf1_dn0 * locals.var_t1) + (locals.var_tmf1 * locals.var_t1_dn0)) * locals.var_dnm) + (assign76350_e115884 * locals.var_dnm_dn0)), ((((locals.var_tmf1_dn2 * locals.var_t1) + (locals.var_tmf1 * locals.var_t1_dn2)) * locals.var_dnm) + (assign76350_e115884 * locals.var_dnm_dn2)), ((((locals.var_tmf1_dn4 * locals.var_t1) + (locals.var_tmf1 * locals.var_t1_dn4)) * locals.var_dnm) + (assign76350_e115884 * locals.var_dnm_dn4)), ((((locals.var_tmf1_dn5 * locals.var_t1) + (locals.var_tmf1 * locals.var_t1_dn5)) * locals.var_dnm) + (assign76350_e115884 * locals.var_dnm_dn5)), ((((locals.var_tmf1_dn6 * locals.var_t1) + (locals.var_tmf1 * locals.var_t1_dn6)) * locals.var_dnm) + (assign76350_e115884 * locals.var_dnm_dn6)), ((((locals.var_tmf1_dn7 * locals.var_t1) + (locals.var_tmf1 * locals.var_t1_dn7)) * locals.var_dnm) + (assign76350_e115884 * locals.var_dnm_dn7)), ((((locals.var_tmf1_dn8 * locals.var_t1) + (locals.var_tmf1 * locals.var_t1_dn8)) * locals.var_dnm) + (assign76350_e115884 * locals.var_dnm_dn8)), ((((locals.var_tmf1_dn9 * locals.var_t1) + (locals.var_tmf1 * locals.var_t1_dn9)) * locals.var_dnm) + (assign76350_e115884 * locals.var_dnm_dn9)), ((((locals.var_tmf1_dn10 * locals.var_t1) + (locals.var_tmf1 * locals.var_t1_dn10)) * locals.var_dnm) + (assign76350_e115884 * locals.var_dnm_dn10)), ((((locals.var_tmf1_dn11 * locals.var_t1) + (locals.var_tmf1 * locals.var_t1_dn11)) * locals.var_dnm) + (assign76350_e115884 * locals.var_dnm_dn11)), ((((locals.var_tmf1_dn14 * locals.var_t1) + (locals.var_tmf1 * locals.var_t1_dn14)) * locals.var_dnm) + (assign76350_e115884 * locals.var_dnm_dn14)),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn11, locals.var_tmf0_dn14,)
    }
};
        locals.var_tmf0 = assign76350_e115888;
        locals.var_tmf0_dn0 = assign76350_e115888_d_n0;
        locals.var_tmf0_dn2 = assign76350_e115888_d_n2;
        locals.var_tmf0_dn4 = assign76350_e115888_d_n4;
        locals.var_tmf0_dn5 = assign76350_e115888_d_n5;
        locals.var_tmf0_dn6 = assign76350_e115888_d_n6;
        locals.var_tmf0_dn7 = assign76350_e115888_d_n7;
        locals.var_tmf0_dn8 = assign76350_e115888_d_n8;
        locals.var_tmf0_dn9 = assign76350_e115888_d_n9;
        locals.var_tmf0_dn10 = assign76350_e115888_d_n10;
        locals.var_tmf0_dn11 = assign76350_e115888_d_n11;
        locals.var_tmf0_dn14 = assign76350_e115888_d_n14;
        locals.var_tmf0_rv = 0.0;

        let (assign76360_e115902, assign76360_e115902_d_n0, assign76360_e115902_d_n2, assign76360_e115902_d_n4, assign76360_e115902_d_n5, assign76360_e115902_d_n6, assign76360_e115902_d_n7, assign76360_e115902_d_n8, assign76360_e115902_d_n9, assign76360_e115902_d_n10, assign76360_e115902_d_n11, assign76360_e115902_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1782 != 0.0)) {
        let assign76360_e115896: f64 = (locals.var_t1 * locals.var_xmp);
        let assign76360_e115898: f64 = (assign76360_e115896 * locals.var_dnm);
        let assign76360_e115900: f64 = (assign76360_e115898 / locals.var_arg);
        (assign76360_e115900, (((((((locals.var_t1_dn0 * locals.var_xmp) + (locals.var_t1 * locals.var_xmp_dn0)) * locals.var_dnm) + (assign76360_e115896 * locals.var_dnm_dn0)) * locals.var_arg) - (assign76360_e115898 * locals.var_arg_dn0)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t1_dn2 * locals.var_xmp) + (locals.var_t1 * locals.var_xmp_dn2)) * locals.var_dnm) + (assign76360_e115896 * locals.var_dnm_dn2)) * locals.var_arg) - (assign76360_e115898 * locals.var_arg_dn2)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t1_dn4 * locals.var_xmp) + (locals.var_t1 * locals.var_xmp_dn4)) * locals.var_dnm) + (assign76360_e115896 * locals.var_dnm_dn4)) * locals.var_arg) - (assign76360_e115898 * locals.var_arg_dn4)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t1_dn5 * locals.var_xmp) + (locals.var_t1 * locals.var_xmp_dn5)) * locals.var_dnm) + (assign76360_e115896 * locals.var_dnm_dn5)) * locals.var_arg) - (assign76360_e115898 * locals.var_arg_dn5)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t1_dn6 * locals.var_xmp) + (locals.var_t1 * locals.var_xmp_dn6)) * locals.var_dnm) + (assign76360_e115896 * locals.var_dnm_dn6)) * locals.var_arg) - (assign76360_e115898 * locals.var_arg_dn6)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t1_dn7 * locals.var_xmp) + (locals.var_t1 * locals.var_xmp_dn7)) * locals.var_dnm) + (assign76360_e115896 * locals.var_dnm_dn7)) * locals.var_arg) - (assign76360_e115898 * locals.var_arg_dn7)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t1_dn8 * locals.var_xmp) + (locals.var_t1 * locals.var_xmp_dn8)) * locals.var_dnm) + (assign76360_e115896 * locals.var_dnm_dn8)) * locals.var_arg) - (assign76360_e115898 * locals.var_arg_dn8)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t1_dn9 * locals.var_xmp) + (locals.var_t1 * locals.var_xmp_dn9)) * locals.var_dnm) + (assign76360_e115896 * locals.var_dnm_dn9)) * locals.var_arg) - (assign76360_e115898 * locals.var_arg_dn9)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t1_dn10 * locals.var_xmp) + (locals.var_t1 * locals.var_xmp_dn10)) * locals.var_dnm) + (assign76360_e115896 * locals.var_dnm_dn10)) * locals.var_arg) - (assign76360_e115898 * locals.var_arg_dn10)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t1_dn11 * locals.var_xmp) + (locals.var_t1 * locals.var_xmp_dn11)) * locals.var_dnm) + (assign76360_e115896 * locals.var_dnm_dn11)) * locals.var_arg) - (assign76360_e115898 * locals.var_arg_dn11)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t1_dn14 * locals.var_xmp) + (locals.var_t1 * locals.var_xmp_dn14)) * locals.var_dnm) + (assign76360_e115896 * locals.var_dnm_dn14)) * locals.var_arg) - (assign76360_e115898 * locals.var_arg_dn14)) / (locals.var_arg * locals.var_arg)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign76360_e115902;
        locals.var_t0_dn0 = assign76360_e115902_d_n0;
        locals.var_t0_dn2 = assign76360_e115902_d_n2;
        locals.var_t0_dn4 = assign76360_e115902_d_n4;
        locals.var_t0_dn5 = assign76360_e115902_d_n5;
        locals.var_t0_dn6 = assign76360_e115902_d_n6;
        locals.var_t0_dn7 = assign76360_e115902_d_n7;
        locals.var_t0_dn8 = assign76360_e115902_d_n8;
        locals.var_t0_dn9 = assign76360_e115902_d_n9;
        locals.var_t0_dn10 = assign76360_e115902_d_n10;
        locals.var_t0_dn11 = assign76360_e115902_d_n11;
        locals.var_t0_dn14 = assign76360_e115902_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign76370_e115914, assign76370_e115914_d_n0, assign76370_e115914_d_n2, assign76370_e115914_d_n4, assign76370_e115914_d_n5, assign76370_e115914_d_n6, assign76370_e115914_d_n7, assign76370_e115914_d_n8, assign76370_e115914_d_n9, assign76370_e115914_d_n10, assign76370_e115914_d_n11, assign76370_e115914_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1782 != 0.0)) {
        let assign76370_e115910: f64 = (-locals.var_t1);
        let assign76370_e115912: f64 = (assign76370_e115910 + locals.var_tmf0);
        (assign76370_e115912, ((-locals.var_t1_dn0) + locals.var_tmf0_dn0), ((-locals.var_t1_dn2) + locals.var_tmf0_dn2), ((-locals.var_t1_dn4) + locals.var_tmf0_dn4), ((-locals.var_t1_dn5) + locals.var_tmf0_dn5), ((-locals.var_t1_dn6) + locals.var_tmf0_dn6), ((-locals.var_t1_dn7) + locals.var_tmf0_dn7), ((-locals.var_t1_dn8) + locals.var_tmf0_dn8), ((-locals.var_t1_dn9) + locals.var_tmf0_dn9), ((-locals.var_t1_dn10) + locals.var_tmf0_dn10), ((-locals.var_t1_dn11) + locals.var_tmf0_dn11), ((-locals.var_t1_dn14) + locals.var_tmf0_dn14),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign76370_e115914;
        locals.var_t1_dn0 = assign76370_e115914_d_n0;
        locals.var_t1_dn2 = assign76370_e115914_d_n2;
        locals.var_t1_dn4 = assign76370_e115914_d_n4;
        locals.var_t1_dn5 = assign76370_e115914_d_n5;
        locals.var_t1_dn6 = assign76370_e115914_d_n6;
        locals.var_t1_dn7 = assign76370_e115914_d_n7;
        locals.var_t1_dn8 = assign76370_e115914_d_n8;
        locals.var_t1_dn9 = assign76370_e115914_d_n9;
        locals.var_t1_dn10 = assign76370_e115914_d_n10;
        locals.var_t1_dn11 = assign76370_e115914_d_n11;
        locals.var_t1_dn14 = assign76370_e115914_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign76380_e115922, assign76380_e115922_d_n0, assign76380_e115922_d_n2, assign76380_e115922_d_n4, assign76380_e115922_d_n5, assign76380_e115922_d_n6, assign76380_e115922_d_n7, assign76380_e115922_d_n8, assign76380_e115922_d_n9, assign76380_e115922_d_n10, assign76380_e115922_d_n11, assign76380_e115922_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1782 != 0.0)) {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign76380_e115922;
        locals.var_t0_dn0 = assign76380_e115922_d_n0;
        locals.var_t0_dn2 = assign76380_e115922_d_n2;
        locals.var_t0_dn4 = assign76380_e115922_d_n4;
        locals.var_t0_dn5 = assign76380_e115922_d_n5;
        locals.var_t0_dn6 = assign76380_e115922_d_n6;
        locals.var_t0_dn7 = assign76380_e115922_d_n7;
        locals.var_t0_dn8 = assign76380_e115922_d_n8;
        locals.var_t0_dn9 = assign76380_e115922_d_n9;
        locals.var_t0_dn10 = assign76380_e115922_d_n10;
        locals.var_t0_dn11 = assign76380_e115922_d_n11;
        locals.var_t0_dn14 = assign76380_e115922_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign76390_e115933, assign76390_e115933_d_n0, assign76390_e115933_d_n2, assign76390_e115933_d_n4, assign76390_e115933_d_n5, assign76390_e115933_d_n6, assign76390_e115933_d_n7, assign76390_e115933_d_n8, assign76390_e115933_d_n9, assign76390_e115933_d_n10, assign76390_e115933_d_n11, assign76390_e115933_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1782 == 0.0)) {
        let assign76390_e115931: f64 = (locals.var_vgpld + locals.var_vxbgmtcl);
        (assign76390_e115931, locals.var_vxbgmtcl_dn0, (locals.var_vgpld_dn2 + locals.var_vxbgmtcl_dn2), locals.var_vxbgmtcl_dn4, locals.var_vxbgmtcl_dn5, locals.var_vxbgmtcl_dn6, (locals.var_vgpld_dn7 + locals.var_vxbgmtcl_dn7), (locals.var_vgpld_dn8 + locals.var_vxbgmtcl_dn8), (locals.var_vgpld_dn9 + locals.var_vxbgmtcl_dn9), locals.var_vxbgmtcl_dn10, locals.var_vxbgmtcl_dn11, locals.var_vxbgmtcl_dn14,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign76390_e115933;
        locals.var_t1_dn0 = assign76390_e115933_d_n0;
        locals.var_t1_dn2 = assign76390_e115933_d_n2;
        locals.var_t1_dn4 = assign76390_e115933_d_n4;
        locals.var_t1_dn5 = assign76390_e115933_d_n5;
        locals.var_t1_dn6 = assign76390_e115933_d_n6;
        locals.var_t1_dn7 = assign76390_e115933_d_n7;
        locals.var_t1_dn8 = assign76390_e115933_d_n8;
        locals.var_t1_dn9 = assign76390_e115933_d_n9;
        locals.var_t1_dn10 = assign76390_e115933_d_n10;
        locals.var_t1_dn11 = assign76390_e115933_d_n11;
        locals.var_t1_dn14 = assign76390_e115933_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign76400_e115942, assign76400_e115942_d_n0, assign76400_e115942_d_n2, assign76400_e115942_d_n4, assign76400_e115942_d_n5, assign76400_e115942_d_n6, assign76400_e115942_d_n7, assign76400_e115942_d_n8, assign76400_e115942_d_n9, assign76400_e115942_d_n10, assign76400_e115942_d_n11, assign76400_e115942_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1782 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign76400_e115942;
        locals.var_t0_dn0 = assign76400_e115942_d_n0;
        locals.var_t0_dn2 = assign76400_e115942_d_n2;
        locals.var_t0_dn4 = assign76400_e115942_d_n4;
        locals.var_t0_dn5 = assign76400_e115942_d_n5;
        locals.var_t0_dn6 = assign76400_e115942_d_n6;
        locals.var_t0_dn7 = assign76400_e115942_d_n7;
        locals.var_t0_dn8 = assign76400_e115942_d_n8;
        locals.var_t0_dn9 = assign76400_e115942_d_n9;
        locals.var_t0_dn10 = assign76400_e115942_d_n10;
        locals.var_t0_dn11 = assign76400_e115942_d_n11;
        locals.var_t0_dn14 = assign76400_e115942_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign76410_e115950, assign76410_e115950_d_n0, assign76410_e115950_d_n2, assign76410_e115950_d_n4, assign76410_e115950_d_n5, assign76410_e115950_d_n6, assign76410_e115950_d_n7, assign76410_e115950_d_n8, assign76410_e115950_d_n9, assign76410_e115950_d_n10, assign76410_e115950_d_n11, assign76410_e115950_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) {
        let assign76410_e115948: f64 = (locals.var_t1 - locals.var_vgpld);
        (assign76410_e115948, locals.var_t1_dn0, (locals.var_t1_dn2 - locals.var_vgpld_dn2), locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, (locals.var_t1_dn7 - locals.var_vgpld_dn7), (locals.var_t1_dn8 - locals.var_vgpld_dn8), (locals.var_t1_dn9 - locals.var_vgpld_dn9), locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    } else {
        (locals.var_vxbgmtcl, locals.var_vxbgmtcl_dn0, locals.var_vxbgmtcl_dn2, locals.var_vxbgmtcl_dn4, locals.var_vxbgmtcl_dn5, locals.var_vxbgmtcl_dn6, locals.var_vxbgmtcl_dn7, locals.var_vxbgmtcl_dn8, locals.var_vxbgmtcl_dn9, locals.var_vxbgmtcl_dn10, locals.var_vxbgmtcl_dn11, locals.var_vxbgmtcl_dn14,)
    }
};
        locals.var_vxbgmtcl = assign76410_e115950;
        locals.var_vxbgmtcl_dn0 = assign76410_e115950_d_n0;
        locals.var_vxbgmtcl_dn2 = assign76410_e115950_d_n2;
        locals.var_vxbgmtcl_dn4 = assign76410_e115950_d_n4;
        locals.var_vxbgmtcl_dn5 = assign76410_e115950_d_n5;
        locals.var_vxbgmtcl_dn6 = assign76410_e115950_d_n6;
        locals.var_vxbgmtcl_dn7 = assign76410_e115950_d_n7;
        locals.var_vxbgmtcl_dn8 = assign76410_e115950_d_n8;
        locals.var_vxbgmtcl_dn9 = assign76410_e115950_d_n9;
        locals.var_vxbgmtcl_dn10 = assign76410_e115950_d_n10;
        locals.var_vxbgmtcl_dn11 = assign76410_e115950_d_n11;
        locals.var_vxbgmtcl_dn14 = assign76410_e115950_d_n14;
        locals.var_vxbgmtcl_rv = 0.0;

        let (assign76420_e115961, assign76420_e115961_d_n0, assign76420_e115961_d_n2, assign76420_e115961_d_n4, assign76420_e115961_d_n5, assign76420_e115961_d_n6, assign76420_e115961_d_n7, assign76420_e115961_d_n8, assign76420_e115961_d_n9, assign76420_e115961_d_n10, assign76420_e115961_d_n11, assign76420_e115961_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) {
        let assign76420_e115955: f64 = (-locals.var_vxbgmtcl);
        let assign76420_e115958: f64 = (10.0 * 2.220446049250313e-16);
        let assign76420_e115959: f64 = (assign76420_e115955 + assign76420_e115958);
        (assign76420_e115959, (-locals.var_vxbgmtcl_dn0), (-locals.var_vxbgmtcl_dn2), (-locals.var_vxbgmtcl_dn4), (-locals.var_vxbgmtcl_dn5), (-locals.var_vxbgmtcl_dn6), (-locals.var_vxbgmtcl_dn7), (-locals.var_vxbgmtcl_dn8), (-locals.var_vxbgmtcl_dn9), (-locals.var_vxbgmtcl_dn10), (-locals.var_vxbgmtcl_dn11), (-locals.var_vxbgmtcl_dn14),)
    } else {
        (locals.var_vgb_fb_ld, locals.var_vgb_fb_ld_dn0, locals.var_vgb_fb_ld_dn2, locals.var_vgb_fb_ld_dn4, locals.var_vgb_fb_ld_dn5, locals.var_vgb_fb_ld_dn6, locals.var_vgb_fb_ld_dn7, locals.var_vgb_fb_ld_dn8, locals.var_vgb_fb_ld_dn9, locals.var_vgb_fb_ld_dn10, locals.var_vgb_fb_ld_dn11, locals.var_vgb_fb_ld_dn14,)
    }
};
        locals.var_vgb_fb_ld = assign76420_e115961;
        locals.var_vgb_fb_ld_dn0 = assign76420_e115961_d_n0;
        locals.var_vgb_fb_ld_dn2 = assign76420_e115961_d_n2;
        locals.var_vgb_fb_ld_dn4 = assign76420_e115961_d_n4;
        locals.var_vgb_fb_ld_dn5 = assign76420_e115961_d_n5;
        locals.var_vgb_fb_ld_dn6 = assign76420_e115961_d_n6;
        locals.var_vgb_fb_ld_dn7 = assign76420_e115961_d_n7;
        locals.var_vgb_fb_ld_dn8 = assign76420_e115961_d_n8;
        locals.var_vgb_fb_ld_dn9 = assign76420_e115961_d_n9;
        locals.var_vgb_fb_ld_dn10 = assign76420_e115961_d_n10;
        locals.var_vgb_fb_ld_dn11 = assign76420_e115961_d_n11;
        locals.var_vgb_fb_ld_dn14 = assign76420_e115961_d_n14;
        locals.var_vgb_fb_ld_rv = 0.0;

        let assign76430_e115964: f64 = if locals.var_vgpld < locals.var_vgb_fb_ld { 1.0 } else { 0.0 };
        locals.var_guard1788 = assign76430_e115964;
        locals.var_guard1788_rv = 0.0;

        let (assign76450_e115985, assign76450_e115985_d_n0, assign76450_e115985_d_n2, assign76450_e115985_d_n4, assign76450_e115985_d_n5, assign76450_e115985_d_n6, assign76450_e115985_d_n7, assign76450_e115985_d_n8, assign76450_e115985_d_n9, assign76450_e115985_d_n10, assign76450_e115985_d_n11, assign76450_e115985_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1788 != 0.0)) {
        let assign76450_e115977: f64 = (2.0 * locals.var_beta_inv);
        let assign76450_e115979: f64 = (-locals.var_vgs_min);
        let assign76450_e115981: f64 = (assign76450_e115979 / locals.var_fac1);
        let assign76450_e115982: f64 = (assign76450_e115981).ln();
        let assign76450_e115983: f64 = (assign76450_e115977 * assign76450_e115982);
        (assign76450_e115983, (((2.0 * locals.var_beta_inv_dn0) * assign76450_e115982) + (assign76450_e115977 * ((-((assign76450_e115979 * locals.var_fac1_dn0) / (locals.var_fac1 * locals.var_fac1))) / assign76450_e115981))), (((2.0 * locals.var_beta_inv_dn2) * assign76450_e115982) + (assign76450_e115977 * ((-((assign76450_e115979 * locals.var_fac1_dn2) / (locals.var_fac1 * locals.var_fac1))) / assign76450_e115981))), (((2.0 * locals.var_beta_inv_dn4) * assign76450_e115982) + (assign76450_e115977 * ((-((assign76450_e115979 * locals.var_fac1_dn4) / (locals.var_fac1 * locals.var_fac1))) / assign76450_e115981))), (((2.0 * locals.var_beta_inv_dn5) * assign76450_e115982) + (assign76450_e115977 * ((-((assign76450_e115979 * locals.var_fac1_dn5) / (locals.var_fac1 * locals.var_fac1))) / assign76450_e115981))), (((2.0 * locals.var_beta_inv_dn6) * assign76450_e115982) + (assign76450_e115977 * ((-((assign76450_e115979 * locals.var_fac1_dn6) / (locals.var_fac1 * locals.var_fac1))) / assign76450_e115981))), (((2.0 * locals.var_beta_inv_dn7) * assign76450_e115982) + (assign76450_e115977 * ((-((assign76450_e115979 * locals.var_fac1_dn7) / (locals.var_fac1 * locals.var_fac1))) / assign76450_e115981))), (((2.0 * locals.var_beta_inv_dn8) * assign76450_e115982) + (assign76450_e115977 * ((-((assign76450_e115979 * locals.var_fac1_dn8) / (locals.var_fac1 * locals.var_fac1))) / assign76450_e115981))), (((2.0 * locals.var_beta_inv_dn9) * assign76450_e115982) + (assign76450_e115977 * ((-((assign76450_e115979 * locals.var_fac1_dn9) / (locals.var_fac1 * locals.var_fac1))) / assign76450_e115981))), (((2.0 * locals.var_beta_inv_dn10) * assign76450_e115982) + (assign76450_e115977 * ((-((assign76450_e115979 * locals.var_fac1_dn10) / (locals.var_fac1 * locals.var_fac1))) / assign76450_e115981))), (((2.0 * locals.var_beta_inv_dn11) * assign76450_e115982) + (assign76450_e115977 * ((-((assign76450_e115979 * locals.var_fac1_dn11) / (locals.var_fac1 * locals.var_fac1))) / assign76450_e115981))), (((2.0 * locals.var_beta_inv_dn14) * assign76450_e115982) + (assign76450_e115977 * ((-((assign76450_e115979 * locals.var_fac1_dn14) / (locals.var_fac1 * locals.var_fac1))) / assign76450_e115981))),)
    } else {
        (locals.var_ps0_min, locals.var_ps0_min_dn0, locals.var_ps0_min_dn2, locals.var_ps0_min_dn4, locals.var_ps0_min_dn5, locals.var_ps0_min_dn6, locals.var_ps0_min_dn7, locals.var_ps0_min_dn8, locals.var_ps0_min_dn9, locals.var_ps0_min_dn10, locals.var_ps0_min_dn11, locals.var_ps0_min_dn14,)
    }
};
        locals.var_ps0_min = assign76450_e115985;
        locals.var_ps0_min_dn0 = assign76450_e115985_d_n0;
        locals.var_ps0_min_dn2 = assign76450_e115985_d_n2;
        locals.var_ps0_min_dn4 = assign76450_e115985_d_n4;
        locals.var_ps0_min_dn5 = assign76450_e115985_d_n5;
        locals.var_ps0_min_dn6 = assign76450_e115985_d_n6;
        locals.var_ps0_min_dn7 = assign76450_e115985_d_n7;
        locals.var_ps0_min_dn8 = assign76450_e115985_d_n8;
        locals.var_ps0_min_dn9 = assign76450_e115985_d_n9;
        locals.var_ps0_min_dn10 = assign76450_e115985_d_n10;
        locals.var_ps0_min_dn11 = assign76450_e115985_d_n11;
        locals.var_ps0_min_dn14 = assign76450_e115985_d_n14;
        locals.var_ps0_min_rv = 0.0;

        let (assign76460_e115995, assign76460_e115995_d_n0, assign76460_e115995_d_n2, assign76460_e115995_d_n4, assign76460_e115995_d_n5, assign76460_e115995_d_n6, assign76460_e115995_d_n7, assign76460_e115995_d_n8, assign76460_e115995_d_n9, assign76460_e115995_d_n10, assign76460_e115995_d_n11, assign76460_e115995_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1788 != 0.0)) {
        let assign76460_e115992: f64 = (locals.var_vgpld + locals.var_vxbgmtcl);
        let assign76460_e115993: f64 = (locals.var_beta * assign76460_e115992);
        (assign76460_e115993, ((locals.var_beta_dn0 * assign76460_e115992) + (locals.var_beta * locals.var_vxbgmtcl_dn0)), ((locals.var_beta_dn2 * assign76460_e115992) + (locals.var_beta * (locals.var_vgpld_dn2 + locals.var_vxbgmtcl_dn2))), ((locals.var_beta_dn4 * assign76460_e115992) + (locals.var_beta * locals.var_vxbgmtcl_dn4)), ((locals.var_beta_dn5 * assign76460_e115992) + (locals.var_beta * locals.var_vxbgmtcl_dn5)), ((locals.var_beta_dn6 * assign76460_e115992) + (locals.var_beta * locals.var_vxbgmtcl_dn6)), ((locals.var_beta_dn7 * assign76460_e115992) + (locals.var_beta * (locals.var_vgpld_dn7 + locals.var_vxbgmtcl_dn7))), ((locals.var_beta_dn8 * assign76460_e115992) + (locals.var_beta * (locals.var_vgpld_dn8 + locals.var_vxbgmtcl_dn8))), ((locals.var_beta_dn9 * assign76460_e115992) + (locals.var_beta * (locals.var_vgpld_dn9 + locals.var_vxbgmtcl_dn9))), ((locals.var_beta_dn10 * assign76460_e115992) + (locals.var_beta * locals.var_vxbgmtcl_dn10)), ((locals.var_beta_dn11 * assign76460_e115992) + (locals.var_beta * locals.var_vxbgmtcl_dn11)), ((locals.var_beta_dn14 * assign76460_e115992) + (locals.var_beta * locals.var_vxbgmtcl_dn14)),)
    } else {
        (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn4, locals.var_tx_dn5, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn8, locals.var_tx_dn9, locals.var_tx_dn10, locals.var_tx_dn11, locals.var_tx_dn14,)
    }
};
        locals.var_tx = assign76460_e115995;
        locals.var_tx_dn0 = assign76460_e115995_d_n0;
        locals.var_tx_dn2 = assign76460_e115995_d_n2;
        locals.var_tx_dn4 = assign76460_e115995_d_n4;
        locals.var_tx_dn5 = assign76460_e115995_d_n5;
        locals.var_tx_dn6 = assign76460_e115995_d_n6;
        locals.var_tx_dn7 = assign76460_e115995_d_n7;
        locals.var_tx_dn8 = assign76460_e115995_d_n8;
        locals.var_tx_dn9 = assign76460_e115995_d_n9;
        locals.var_tx_dn10 = assign76460_e115995_d_n10;
        locals.var_tx_dn11 = assign76460_e115995_d_n11;
        locals.var_tx_dn14 = assign76460_e115995_d_n14;
        locals.var_tx_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_289(
        locals: &mut StampLocals,
    ) {
        let (assign76470_e116005, assign76470_e116005_d_n0, assign76470_e116005_d_n2, assign76470_e116005_d_n4, assign76470_e116005_d_n5, assign76470_e116005_d_n6, assign76470_e116005_d_n7, assign76470_e116005_d_n8, assign76470_e116005_d_n9, assign76470_e116005_d_n10, assign76470_e116005_d_n11, assign76470_e116005_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1788 != 0.0)) {
        let assign76470_e116002: f64 = (locals.var_beta * locals.var_cnst0over_func);
        let assign76470_e116003: f64 = (1.0 / assign76470_e116002);
        (assign76470_e116003, (-(((locals.var_beta_dn0 * locals.var_cnst0over_func) + (locals.var_beta * locals.var_cnst0over_func_dn0)) / (assign76470_e116002 * assign76470_e116002))), (-(((locals.var_beta_dn2 * locals.var_cnst0over_func) + (locals.var_beta * locals.var_cnst0over_func_dn2)) / (assign76470_e116002 * assign76470_e116002))), (-(((locals.var_beta_dn4 * locals.var_cnst0over_func) + (locals.var_beta * locals.var_cnst0over_func_dn4)) / (assign76470_e116002 * assign76470_e116002))), (-(((locals.var_beta_dn5 * locals.var_cnst0over_func) + (locals.var_beta * locals.var_cnst0over_func_dn5)) / (assign76470_e116002 * assign76470_e116002))), (-(((locals.var_beta_dn6 * locals.var_cnst0over_func) + (locals.var_beta * locals.var_cnst0over_func_dn6)) / (assign76470_e116002 * assign76470_e116002))), (-(((locals.var_beta_dn7 * locals.var_cnst0over_func) + (locals.var_beta * locals.var_cnst0over_func_dn7)) / (assign76470_e116002 * assign76470_e116002))), (-(((locals.var_beta_dn8 * locals.var_cnst0over_func) + (locals.var_beta * locals.var_cnst0over_func_dn8)) / (assign76470_e116002 * assign76470_e116002))), (-(((locals.var_beta_dn9 * locals.var_cnst0over_func) + (locals.var_beta * locals.var_cnst0over_func_dn9)) / (assign76470_e116002 * assign76470_e116002))), (-(((locals.var_beta_dn10 * locals.var_cnst0over_func) + (locals.var_beta * locals.var_cnst0over_func_dn10)) / (assign76470_e116002 * assign76470_e116002))), (-(((locals.var_beta_dn11 * locals.var_cnst0over_func) + (locals.var_beta * locals.var_cnst0over_func_dn11)) / (assign76470_e116002 * assign76470_e116002))), (-(((locals.var_beta_dn14 * locals.var_cnst0over_func) + (locals.var_beta * locals.var_cnst0over_func_dn14)) / (assign76470_e116002 * assign76470_e116002))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign76470_e116005;
        locals.var_t1_dn0 = assign76470_e116005_d_n0;
        locals.var_t1_dn2 = assign76470_e116005_d_n2;
        locals.var_t1_dn4 = assign76470_e116005_d_n4;
        locals.var_t1_dn5 = assign76470_e116005_d_n5;
        locals.var_t1_dn6 = assign76470_e116005_d_n6;
        locals.var_t1_dn7 = assign76470_e116005_d_n7;
        locals.var_t1_dn8 = assign76470_e116005_d_n8;
        locals.var_t1_dn9 = assign76470_e116005_d_n9;
        locals.var_t1_dn10 = assign76470_e116005_d_n10;
        locals.var_t1_dn11 = assign76470_e116005_d_n11;
        locals.var_t1_dn14 = assign76470_e116005_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign76480_e116013, assign76480_e116013_d_n0, assign76480_e116013_d_n2, assign76480_e116013_d_n4, assign76480_e116013_d_n5, assign76480_e116013_d_n6, assign76480_e116013_d_n7, assign76480_e116013_d_n8, assign76480_e116013_d_n9, assign76480_e116013_d_n10, assign76480_e116013_d_n11, assign76480_e116013_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1788 != 0.0)) {
        let assign76480_e116011: f64 = (locals.var_t1 * locals.var_cox0_func);
        (assign76480_e116011, (locals.var_t1_dn0 * locals.var_cox0_func), (locals.var_t1_dn2 * locals.var_cox0_func), (locals.var_t1_dn4 * locals.var_cox0_func), (locals.var_t1_dn5 * locals.var_cox0_func), (locals.var_t1_dn6 * locals.var_cox0_func), (locals.var_t1_dn7 * locals.var_cox0_func), (locals.var_t1_dn8 * locals.var_cox0_func), (locals.var_t1_dn9 * locals.var_cox0_func), (locals.var_t1_dn10 * locals.var_cox0_func), (locals.var_t1_dn11 * locals.var_cox0_func), (locals.var_t1_dn14 * locals.var_cox0_func),)
    } else {
        (locals.var_ty, locals.var_ty_dn0, locals.var_ty_dn2, locals.var_ty_dn4, locals.var_ty_dn5, locals.var_ty_dn6, locals.var_ty_dn7, locals.var_ty_dn8, locals.var_ty_dn9, locals.var_ty_dn10, locals.var_ty_dn11, locals.var_ty_dn14,)
    }
};
        locals.var_ty = assign76480_e116013;
        locals.var_ty_dn0 = assign76480_e116013_d_n0;
        locals.var_ty_dn2 = assign76480_e116013_d_n2;
        locals.var_ty_dn4 = assign76480_e116013_d_n4;
        locals.var_ty_dn5 = assign76480_e116013_d_n5;
        locals.var_ty_dn6 = assign76480_e116013_d_n6;
        locals.var_ty_dn7 = assign76480_e116013_d_n7;
        locals.var_ty_dn8 = assign76480_e116013_d_n8;
        locals.var_ty_dn9 = assign76480_e116013_d_n9;
        locals.var_ty_dn10 = assign76480_e116013_d_n10;
        locals.var_ty_dn11 = assign76480_e116013_d_n11;
        locals.var_ty_dn14 = assign76480_e116013_d_n14;
        locals.var_ty_rv = 0.0;

        let (assign76490_e116025, assign76490_e116025_d_n0, assign76490_e116025_d_n2, assign76490_e116025_d_n4, assign76490_e116025_d_n5, assign76490_e116025_d_n6, assign76490_e116025_d_n7, assign76490_e116025_d_n8, assign76490_e116025_d_n9, assign76490_e116025_d_n10, assign76490_e116025_d_n11, assign76490_e116025_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1788 != 0.0)) {
        let assign76490_e116020: f64 = (3.0 * 1.414213562373095);
        let assign76490_e116022: f64 = (assign76490_e116020 * locals.var_ty);
        let assign76490_e116023: f64 = (2.0 + assign76490_e116022);
        (assign76490_e116023, (assign76490_e116020 * locals.var_ty_dn0), (assign76490_e116020 * locals.var_ty_dn2), (assign76490_e116020 * locals.var_ty_dn4), (assign76490_e116020 * locals.var_ty_dn5), (assign76490_e116020 * locals.var_ty_dn6), (assign76490_e116020 * locals.var_ty_dn7), (assign76490_e116020 * locals.var_ty_dn8), (assign76490_e116020 * locals.var_ty_dn9), (assign76490_e116020 * locals.var_ty_dn10), (assign76490_e116020 * locals.var_ty_dn11), (assign76490_e116020 * locals.var_ty_dn14),)
    } else {
        (locals.var_ac41, locals.var_ac41_dn0, locals.var_ac41_dn2, locals.var_ac41_dn4, locals.var_ac41_dn5, locals.var_ac41_dn6, locals.var_ac41_dn7, locals.var_ac41_dn8, locals.var_ac41_dn9, locals.var_ac41_dn10, locals.var_ac41_dn11, locals.var_ac41_dn14,)
    }
};
        locals.var_ac41 = assign76490_e116025;
        locals.var_ac41_dn0 = assign76490_e116025_d_n0;
        locals.var_ac41_dn2 = assign76490_e116025_d_n2;
        locals.var_ac41_dn4 = assign76490_e116025_d_n4;
        locals.var_ac41_dn5 = assign76490_e116025_d_n5;
        locals.var_ac41_dn6 = assign76490_e116025_d_n6;
        locals.var_ac41_dn7 = assign76490_e116025_d_n7;
        locals.var_ac41_dn8 = assign76490_e116025_d_n8;
        locals.var_ac41_dn9 = assign76490_e116025_d_n9;
        locals.var_ac41_dn10 = assign76490_e116025_d_n10;
        locals.var_ac41_dn11 = assign76490_e116025_d_n11;
        locals.var_ac41_dn14 = assign76490_e116025_d_n14;
        locals.var_ac41_rv = 0.0;

        let (assign76500_e116037, assign76500_e116037_d_n0, assign76500_e116037_d_n2, assign76500_e116037_d_n4, assign76500_e116037_d_n5, assign76500_e116037_d_n6, assign76500_e116037_d_n7, assign76500_e116037_d_n8, assign76500_e116037_d_n9, assign76500_e116037_d_n10, assign76500_e116037_d_n11, assign76500_e116037_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1788 != 0.0)) {
        let assign76500_e116031: f64 = (8.0 * locals.var_ac41);
        let assign76500_e116033: f64 = (assign76500_e116031 * locals.var_ac41);
        let assign76500_e116035: f64 = (assign76500_e116033 * locals.var_ac41);
        (assign76500_e116035, (((((8.0 * locals.var_ac41_dn0) * locals.var_ac41) + (assign76500_e116031 * locals.var_ac41_dn0)) * locals.var_ac41) + (assign76500_e116033 * locals.var_ac41_dn0)), (((((8.0 * locals.var_ac41_dn2) * locals.var_ac41) + (assign76500_e116031 * locals.var_ac41_dn2)) * locals.var_ac41) + (assign76500_e116033 * locals.var_ac41_dn2)), (((((8.0 * locals.var_ac41_dn4) * locals.var_ac41) + (assign76500_e116031 * locals.var_ac41_dn4)) * locals.var_ac41) + (assign76500_e116033 * locals.var_ac41_dn4)), (((((8.0 * locals.var_ac41_dn5) * locals.var_ac41) + (assign76500_e116031 * locals.var_ac41_dn5)) * locals.var_ac41) + (assign76500_e116033 * locals.var_ac41_dn5)), (((((8.0 * locals.var_ac41_dn6) * locals.var_ac41) + (assign76500_e116031 * locals.var_ac41_dn6)) * locals.var_ac41) + (assign76500_e116033 * locals.var_ac41_dn6)), (((((8.0 * locals.var_ac41_dn7) * locals.var_ac41) + (assign76500_e116031 * locals.var_ac41_dn7)) * locals.var_ac41) + (assign76500_e116033 * locals.var_ac41_dn7)), (((((8.0 * locals.var_ac41_dn8) * locals.var_ac41) + (assign76500_e116031 * locals.var_ac41_dn8)) * locals.var_ac41) + (assign76500_e116033 * locals.var_ac41_dn8)), (((((8.0 * locals.var_ac41_dn9) * locals.var_ac41) + (assign76500_e116031 * locals.var_ac41_dn9)) * locals.var_ac41) + (assign76500_e116033 * locals.var_ac41_dn9)), (((((8.0 * locals.var_ac41_dn10) * locals.var_ac41) + (assign76500_e116031 * locals.var_ac41_dn10)) * locals.var_ac41) + (assign76500_e116033 * locals.var_ac41_dn10)), (((((8.0 * locals.var_ac41_dn11) * locals.var_ac41) + (assign76500_e116031 * locals.var_ac41_dn11)) * locals.var_ac41) + (assign76500_e116033 * locals.var_ac41_dn11)), (((((8.0 * locals.var_ac41_dn14) * locals.var_ac41) + (assign76500_e116031 * locals.var_ac41_dn14)) * locals.var_ac41) + (assign76500_e116033 * locals.var_ac41_dn14)),)
    } else {
        (locals.var_ac4, locals.var_ac4_dn0, locals.var_ac4_dn2, locals.var_ac4_dn4, locals.var_ac4_dn5, locals.var_ac4_dn6, locals.var_ac4_dn7, locals.var_ac4_dn8, locals.var_ac4_dn9, locals.var_ac4_dn10, locals.var_ac4_dn11, locals.var_ac4_dn14,)
    }
};
        locals.var_ac4 = assign76500_e116037;
        locals.var_ac4_dn0 = assign76500_e116037_d_n0;
        locals.var_ac4_dn2 = assign76500_e116037_d_n2;
        locals.var_ac4_dn4 = assign76500_e116037_d_n4;
        locals.var_ac4_dn5 = assign76500_e116037_d_n5;
        locals.var_ac4_dn6 = assign76500_e116037_d_n6;
        locals.var_ac4_dn7 = assign76500_e116037_d_n7;
        locals.var_ac4_dn8 = assign76500_e116037_d_n8;
        locals.var_ac4_dn9 = assign76500_e116037_d_n9;
        locals.var_ac4_dn10 = assign76500_e116037_d_n10;
        locals.var_ac4_dn11 = assign76500_e116037_d_n11;
        locals.var_ac4_dn14 = assign76500_e116037_d_n14;
        locals.var_ac4_rv = 0.0;

        let (assign76510_e116053, assign76510_e116053_d_n0, assign76510_e116053_d_n2, assign76510_e116053_d_n4, assign76510_e116053_d_n5, assign76510_e116053_d_n6, assign76510_e116053_d_n7, assign76510_e116053_d_n8, assign76510_e116053_d_n9, assign76510_e116053_d_n10, assign76510_e116053_d_n11, assign76510_e116053_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1788 != 0.0)) {
        let assign76510_e116043: f64 = (7.0 * 1.414213562373095);
        let assign76510_e116046: f64 = (9.0 * locals.var_ty);
        let assign76510_e116049: f64 = (locals.var_tx - 2.0);
        let assign76510_e116050: f64 = (assign76510_e116046 * assign76510_e116049);
        let assign76510_e116051: f64 = (assign76510_e116043 - assign76510_e116050);
        (assign76510_e116051, (-(((9.0 * locals.var_ty_dn0) * assign76510_e116049) + (assign76510_e116046 * locals.var_tx_dn0))), (-(((9.0 * locals.var_ty_dn2) * assign76510_e116049) + (assign76510_e116046 * locals.var_tx_dn2))), (-(((9.0 * locals.var_ty_dn4) * assign76510_e116049) + (assign76510_e116046 * locals.var_tx_dn4))), (-(((9.0 * locals.var_ty_dn5) * assign76510_e116049) + (assign76510_e116046 * locals.var_tx_dn5))), (-(((9.0 * locals.var_ty_dn6) * assign76510_e116049) + (assign76510_e116046 * locals.var_tx_dn6))), (-(((9.0 * locals.var_ty_dn7) * assign76510_e116049) + (assign76510_e116046 * locals.var_tx_dn7))), (-(((9.0 * locals.var_ty_dn8) * assign76510_e116049) + (assign76510_e116046 * locals.var_tx_dn8))), (-(((9.0 * locals.var_ty_dn9) * assign76510_e116049) + (assign76510_e116046 * locals.var_tx_dn9))), (-(((9.0 * locals.var_ty_dn10) * assign76510_e116049) + (assign76510_e116046 * locals.var_tx_dn10))), (-(((9.0 * locals.var_ty_dn11) * assign76510_e116049) + (assign76510_e116046 * locals.var_tx_dn11))), (-(((9.0 * locals.var_ty_dn14) * assign76510_e116049) + (assign76510_e116046 * locals.var_tx_dn14))),)
    } else {
        (locals.var_ac31, locals.var_ac31_dn0, locals.var_ac31_dn2, locals.var_ac31_dn4, locals.var_ac31_dn5, locals.var_ac31_dn6, locals.var_ac31_dn7, locals.var_ac31_dn8, locals.var_ac31_dn9, locals.var_ac31_dn10, locals.var_ac31_dn11, locals.var_ac31_dn14,)
    }
};
        locals.var_ac31 = assign76510_e116053;
        locals.var_ac31_dn0 = assign76510_e116053_d_n0;
        locals.var_ac31_dn2 = assign76510_e116053_d_n2;
        locals.var_ac31_dn4 = assign76510_e116053_d_n4;
        locals.var_ac31_dn5 = assign76510_e116053_d_n5;
        locals.var_ac31_dn6 = assign76510_e116053_d_n6;
        locals.var_ac31_dn7 = assign76510_e116053_d_n7;
        locals.var_ac31_dn8 = assign76510_e116053_d_n8;
        locals.var_ac31_dn9 = assign76510_e116053_d_n9;
        locals.var_ac31_dn10 = assign76510_e116053_d_n10;
        locals.var_ac31_dn11 = assign76510_e116053_d_n11;
        locals.var_ac31_dn14 = assign76510_e116053_d_n14;
        locals.var_ac31_rv = 0.0;

        let (assign76520_e116061, assign76520_e116061_d_n0, assign76520_e116061_d_n2, assign76520_e116061_d_n4, assign76520_e116061_d_n5, assign76520_e116061_d_n6, assign76520_e116061_d_n7, assign76520_e116061_d_n8, assign76520_e116061_d_n9, assign76520_e116061_d_n10, assign76520_e116061_d_n11, assign76520_e116061_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1788 != 0.0)) {
        let assign76520_e116059: f64 = (locals.var_ac31 * locals.var_ac31);
        (assign76520_e116059, ((locals.var_ac31_dn0 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn0)), ((locals.var_ac31_dn2 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn2)), ((locals.var_ac31_dn4 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn4)), ((locals.var_ac31_dn5 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn5)), ((locals.var_ac31_dn6 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn6)), ((locals.var_ac31_dn7 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn7)), ((locals.var_ac31_dn8 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn8)), ((locals.var_ac31_dn9 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn9)), ((locals.var_ac31_dn10 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn10)), ((locals.var_ac31_dn11 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn11)), ((locals.var_ac31_dn14 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn14)),)
    } else {
        (locals.var_ac3, locals.var_ac3_dn0, locals.var_ac3_dn2, locals.var_ac3_dn4, locals.var_ac3_dn5, locals.var_ac3_dn6, locals.var_ac3_dn7, locals.var_ac3_dn8, locals.var_ac3_dn9, locals.var_ac3_dn10, locals.var_ac3_dn11, locals.var_ac3_dn14,)
    }
};
        locals.var_ac3 = assign76520_e116061;
        locals.var_ac3_dn0 = assign76520_e116061_d_n0;
        locals.var_ac3_dn2 = assign76520_e116061_d_n2;
        locals.var_ac3_dn4 = assign76520_e116061_d_n4;
        locals.var_ac3_dn5 = assign76520_e116061_d_n5;
        locals.var_ac3_dn6 = assign76520_e116061_d_n6;
        locals.var_ac3_dn7 = assign76520_e116061_d_n7;
        locals.var_ac3_dn8 = assign76520_e116061_d_n8;
        locals.var_ac3_dn9 = assign76520_e116061_d_n9;
        locals.var_ac3_dn10 = assign76520_e116061_d_n10;
        locals.var_ac3_dn11 = assign76520_e116061_d_n11;
        locals.var_ac3_dn14 = assign76520_e116061_d_n14;
        locals.var_ac3_rv = 0.0;

        let assign76530_e116065: f64 = (locals.var_ac3 * 1e-8);
        let assign76530_e116066: f64 = if locals.var_ac4 < assign76530_e116065 { 1.0 } else { 0.0 };
        locals.var_guard1789 = assign76530_e116066;
        locals.var_guard1789_rv = 0.0;

        let (assign76550_e116087, assign76550_e116087_d_n0, assign76550_e116087_d_n2, assign76550_e116087_d_n4, assign76550_e116087_d_n5, assign76550_e116087_d_n6, assign76550_e116087_d_n7, assign76550_e116087_d_n8, assign76550_e116087_d_n9, assign76550_e116087_d_n10, assign76550_e116087_d_n11, assign76550_e116087_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1788 != 0.0)) && (locals.var_guard1789 != 0.0)) {
        let assign76550_e116083: f64 = (0.5 * locals.var_ac4);
        let assign76550_e116085: f64 = (assign76550_e116083 / locals.var_ac31);
        (assign76550_e116085, ((((0.5 * locals.var_ac4_dn0) * locals.var_ac31) - (assign76550_e116083 * locals.var_ac31_dn0)) / (locals.var_ac31 * locals.var_ac31)), ((((0.5 * locals.var_ac4_dn2) * locals.var_ac31) - (assign76550_e116083 * locals.var_ac31_dn2)) / (locals.var_ac31 * locals.var_ac31)), ((((0.5 * locals.var_ac4_dn4) * locals.var_ac31) - (assign76550_e116083 * locals.var_ac31_dn4)) / (locals.var_ac31 * locals.var_ac31)), ((((0.5 * locals.var_ac4_dn5) * locals.var_ac31) - (assign76550_e116083 * locals.var_ac31_dn5)) / (locals.var_ac31 * locals.var_ac31)), ((((0.5 * locals.var_ac4_dn6) * locals.var_ac31) - (assign76550_e116083 * locals.var_ac31_dn6)) / (locals.var_ac31 * locals.var_ac31)), ((((0.5 * locals.var_ac4_dn7) * locals.var_ac31) - (assign76550_e116083 * locals.var_ac31_dn7)) / (locals.var_ac31 * locals.var_ac31)), ((((0.5 * locals.var_ac4_dn8) * locals.var_ac31) - (assign76550_e116083 * locals.var_ac31_dn8)) / (locals.var_ac31 * locals.var_ac31)), ((((0.5 * locals.var_ac4_dn9) * locals.var_ac31) - (assign76550_e116083 * locals.var_ac31_dn9)) / (locals.var_ac31 * locals.var_ac31)), ((((0.5 * locals.var_ac4_dn10) * locals.var_ac31) - (assign76550_e116083 * locals.var_ac31_dn10)) / (locals.var_ac31 * locals.var_ac31)), ((((0.5 * locals.var_ac4_dn11) * locals.var_ac31) - (assign76550_e116083 * locals.var_ac31_dn11)) / (locals.var_ac31 * locals.var_ac31)), ((((0.5 * locals.var_ac4_dn14) * locals.var_ac31) - (assign76550_e116083 * locals.var_ac31_dn14)) / (locals.var_ac31 * locals.var_ac31)),)
    } else {
        (locals.var_ac1, locals.var_ac1_dn0, locals.var_ac1_dn2, locals.var_ac1_dn4, locals.var_ac1_dn5, locals.var_ac1_dn6, locals.var_ac1_dn7, locals.var_ac1_dn8, locals.var_ac1_dn9, locals.var_ac1_dn10, locals.var_ac1_dn11, locals.var_ac1_dn14,)
    }
};
        locals.var_ac1 = assign76550_e116087;
        locals.var_ac1_dn0 = assign76550_e116087_d_n0;
        locals.var_ac1_dn2 = assign76550_e116087_d_n2;
        locals.var_ac1_dn4 = assign76550_e116087_d_n4;
        locals.var_ac1_dn5 = assign76550_e116087_d_n5;
        locals.var_ac1_dn6 = assign76550_e116087_d_n6;
        locals.var_ac1_dn7 = assign76550_e116087_d_n7;
        locals.var_ac1_dn8 = assign76550_e116087_d_n8;
        locals.var_ac1_dn9 = assign76550_e116087_d_n9;
        locals.var_ac1_dn10 = assign76550_e116087_d_n10;
        locals.var_ac1_dn11 = assign76550_e116087_d_n11;
        locals.var_ac1_dn14 = assign76550_e116087_d_n14;
        locals.var_ac1_rv = 0.0;

        let (assign76560_e116099, assign76560_e116099_d_n0, assign76560_e116099_d_n2, assign76560_e116099_d_n4, assign76560_e116099_d_n5, assign76560_e116099_d_n6, assign76560_e116099_d_n7, assign76560_e116099_d_n8, assign76560_e116099_d_n9, assign76560_e116099_d_n10, assign76560_e116099_d_n11, assign76560_e116099_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1788 != 0.0)) && (locals.var_guard1789 == 0.0)) {
        let assign76560_e116096: f64 = (locals.var_ac4 + locals.var_ac3);
        let assign76560_e116097: f64 = (assign76560_e116096).sqrt();
        (assign76560_e116097, ((locals.var_ac4_dn0 + locals.var_ac3_dn0) / (2.0 * assign76560_e116097)), ((locals.var_ac4_dn2 + locals.var_ac3_dn2) / (2.0 * assign76560_e116097)), ((locals.var_ac4_dn4 + locals.var_ac3_dn4) / (2.0 * assign76560_e116097)), ((locals.var_ac4_dn5 + locals.var_ac3_dn5) / (2.0 * assign76560_e116097)), ((locals.var_ac4_dn6 + locals.var_ac3_dn6) / (2.0 * assign76560_e116097)), ((locals.var_ac4_dn7 + locals.var_ac3_dn7) / (2.0 * assign76560_e116097)), ((locals.var_ac4_dn8 + locals.var_ac3_dn8) / (2.0 * assign76560_e116097)), ((locals.var_ac4_dn9 + locals.var_ac3_dn9) / (2.0 * assign76560_e116097)), ((locals.var_ac4_dn10 + locals.var_ac3_dn10) / (2.0 * assign76560_e116097)), ((locals.var_ac4_dn11 + locals.var_ac3_dn11) / (2.0 * assign76560_e116097)), ((locals.var_ac4_dn14 + locals.var_ac3_dn14) / (2.0 * assign76560_e116097)),)
    } else {
        (locals.var_ac2, locals.var_ac2_dn0, locals.var_ac2_dn2, locals.var_ac2_dn4, locals.var_ac2_dn5, locals.var_ac2_dn6, locals.var_ac2_dn7, locals.var_ac2_dn8, locals.var_ac2_dn9, locals.var_ac2_dn10, locals.var_ac2_dn11, locals.var_ac2_dn14,)
    }
};
        locals.var_ac2 = assign76560_e116099;
        locals.var_ac2_dn0 = assign76560_e116099_d_n0;
        locals.var_ac2_dn2 = assign76560_e116099_d_n2;
        locals.var_ac2_dn4 = assign76560_e116099_d_n4;
        locals.var_ac2_dn5 = assign76560_e116099_d_n5;
        locals.var_ac2_dn6 = assign76560_e116099_d_n6;
        locals.var_ac2_dn7 = assign76560_e116099_d_n7;
        locals.var_ac2_dn8 = assign76560_e116099_d_n8;
        locals.var_ac2_dn9 = assign76560_e116099_d_n9;
        locals.var_ac2_dn10 = assign76560_e116099_d_n10;
        locals.var_ac2_dn11 = assign76560_e116099_d_n11;
        locals.var_ac2_dn14 = assign76560_e116099_d_n14;
        locals.var_ac2_rv = 0.0;

        let (assign76570_e116111, assign76570_e116111_d_n0, assign76570_e116111_d_n2, assign76570_e116111_d_n4, assign76570_e116111_d_n5, assign76570_e116111_d_n6, assign76570_e116111_d_n7, assign76570_e116111_d_n8, assign76570_e116111_d_n9, assign76570_e116111_d_n10, assign76570_e116111_d_n11, assign76570_e116111_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1788 != 0.0)) && (locals.var_guard1789 == 0.0)) {
        let assign76570_e116107: f64 = (-locals.var_ac31);
        let assign76570_e116109: f64 = (assign76570_e116107 + locals.var_ac2);
        (assign76570_e116109, ((-locals.var_ac31_dn0) + locals.var_ac2_dn0), ((-locals.var_ac31_dn2) + locals.var_ac2_dn2), ((-locals.var_ac31_dn4) + locals.var_ac2_dn4), ((-locals.var_ac31_dn5) + locals.var_ac2_dn5), ((-locals.var_ac31_dn6) + locals.var_ac2_dn6), ((-locals.var_ac31_dn7) + locals.var_ac2_dn7), ((-locals.var_ac31_dn8) + locals.var_ac2_dn8), ((-locals.var_ac31_dn9) + locals.var_ac2_dn9), ((-locals.var_ac31_dn10) + locals.var_ac2_dn10), ((-locals.var_ac31_dn11) + locals.var_ac2_dn11), ((-locals.var_ac31_dn14) + locals.var_ac2_dn14),)
    } else {
        (locals.var_ac1, locals.var_ac1_dn0, locals.var_ac1_dn2, locals.var_ac1_dn4, locals.var_ac1_dn5, locals.var_ac1_dn6, locals.var_ac1_dn7, locals.var_ac1_dn8, locals.var_ac1_dn9, locals.var_ac1_dn10, locals.var_ac1_dn11, locals.var_ac1_dn14,)
    }
};
        locals.var_ac1 = assign76570_e116111;
        locals.var_ac1_dn0 = assign76570_e116111_d_n0;
        locals.var_ac1_dn2 = assign76570_e116111_d_n2;
        locals.var_ac1_dn4 = assign76570_e116111_d_n4;
        locals.var_ac1_dn5 = assign76570_e116111_d_n5;
        locals.var_ac1_dn6 = assign76570_e116111_d_n6;
        locals.var_ac1_dn7 = assign76570_e116111_d_n7;
        locals.var_ac1_dn8 = assign76570_e116111_d_n8;
        locals.var_ac1_dn9 = assign76570_e116111_d_n9;
        locals.var_ac1_dn10 = assign76570_e116111_d_n10;
        locals.var_ac1_dn11 = assign76570_e116111_d_n11;
        locals.var_ac1_dn14 = assign76570_e116111_d_n14;
        locals.var_ac1_rv = 0.0;

        let (assign76580_e116119, assign76580_e116119_d_n0, assign76580_e116119_d_n2, assign76580_e116119_d_n4, assign76580_e116119_d_n5, assign76580_e116119_d_n6, assign76580_e116119_d_n7, assign76580_e116119_d_n8, assign76580_e116119_d_n9, assign76580_e116119_d_n10, assign76580_e116119_d_n11, assign76580_e116119_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1788 != 0.0)) {
        let assign76580_e116117: f64 = (locals.var_ac1).powf(0.3333333333333333);
        (assign76580_e116117, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn0)) } } else { (assign76580_e116117 * (0.3333333333333333 * (locals.var_ac1_dn0 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn2)) } } else { (assign76580_e116117 * (0.3333333333333333 * (locals.var_ac1_dn2 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn4)) } } else { (assign76580_e116117 * (0.3333333333333333 * (locals.var_ac1_dn4 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn5)) } } else { (assign76580_e116117 * (0.3333333333333333 * (locals.var_ac1_dn5 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn6)) } } else { (assign76580_e116117 * (0.3333333333333333 * (locals.var_ac1_dn6 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn7)) } } else { (assign76580_e116117 * (0.3333333333333333 * (locals.var_ac1_dn7 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn8)) } } else { (assign76580_e116117 * (0.3333333333333333 * (locals.var_ac1_dn8 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn9)) } } else { (assign76580_e116117 * (0.3333333333333333 * (locals.var_ac1_dn9 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn10)) } } else { (assign76580_e116117 * (0.3333333333333333 * (locals.var_ac1_dn10 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn11)) } } else { (assign76580_e116117 * (0.3333333333333333 * (locals.var_ac1_dn11 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn14)) } } else { (assign76580_e116117 * (0.3333333333333333 * (locals.var_ac1_dn14 / locals.var_ac1))) },)
    } else {
        (locals.var_acd, locals.var_acd_dn0, locals.var_acd_dn2, locals.var_acd_dn4, locals.var_acd_dn5, locals.var_acd_dn6, locals.var_acd_dn7, locals.var_acd_dn8, locals.var_acd_dn9, locals.var_acd_dn10, locals.var_acd_dn11, locals.var_acd_dn14,)
    }
};
        locals.var_acd = assign76580_e116119;
        locals.var_acd_dn0 = assign76580_e116119_d_n0;
        locals.var_acd_dn2 = assign76580_e116119_d_n2;
        locals.var_acd_dn4 = assign76580_e116119_d_n4;
        locals.var_acd_dn5 = assign76580_e116119_d_n5;
        locals.var_acd_dn6 = assign76580_e116119_d_n6;
        locals.var_acd_dn7 = assign76580_e116119_d_n7;
        locals.var_acd_dn8 = assign76580_e116119_d_n8;
        locals.var_acd_dn9 = assign76580_e116119_d_n9;
        locals.var_acd_dn10 = assign76580_e116119_d_n10;
        locals.var_acd_dn11 = assign76580_e116119_d_n11;
        locals.var_acd_dn14 = assign76580_e116119_d_n14;
        locals.var_acd_rv = 0.0;

        let (assign76590_e116142, assign76590_e116142_d_n0, assign76590_e116142_d_n2, assign76590_e116142_d_n4, assign76590_e116142_d_n5, assign76590_e116142_d_n6, assign76590_e116142_d_n7, assign76590_e116142_d_n8, assign76590_e116142_d_n9, assign76590_e116142_d_n10, assign76590_e116142_d_n11, assign76590_e116142_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1788 != 0.0)) {
        let assign76590_e116124: f64 = (-4.0);
        let assign76590_e116126: f64 = (assign76590_e116124 * 1.414213562373095);
        let assign76590_e116129: f64 = (12.0 * locals.var_ty);
        let assign76590_e116130: f64 = (assign76590_e116126 - assign76590_e116129);
        let assign76590_e116133: f64 = (2.0 * locals.var_acd);
        let assign76590_e116134: f64 = (assign76590_e116130 + assign76590_e116133);
        let assign76590_e116137: f64 = (1.414213562373095 * locals.var_acd);
        let assign76590_e116139: f64 = (assign76590_e116137 * locals.var_acd);
        let assign76590_e116140: f64 = (assign76590_e116134 + assign76590_e116139);
        (assign76590_e116140, (((-(12.0 * locals.var_ty_dn0)) + (2.0 * locals.var_acd_dn0)) + (((1.414213562373095 * locals.var_acd_dn0) * locals.var_acd) + (assign76590_e116137 * locals.var_acd_dn0))), (((-(12.0 * locals.var_ty_dn2)) + (2.0 * locals.var_acd_dn2)) + (((1.414213562373095 * locals.var_acd_dn2) * locals.var_acd) + (assign76590_e116137 * locals.var_acd_dn2))), (((-(12.0 * locals.var_ty_dn4)) + (2.0 * locals.var_acd_dn4)) + (((1.414213562373095 * locals.var_acd_dn4) * locals.var_acd) + (assign76590_e116137 * locals.var_acd_dn4))), (((-(12.0 * locals.var_ty_dn5)) + (2.0 * locals.var_acd_dn5)) + (((1.414213562373095 * locals.var_acd_dn5) * locals.var_acd) + (assign76590_e116137 * locals.var_acd_dn5))), (((-(12.0 * locals.var_ty_dn6)) + (2.0 * locals.var_acd_dn6)) + (((1.414213562373095 * locals.var_acd_dn6) * locals.var_acd) + (assign76590_e116137 * locals.var_acd_dn6))), (((-(12.0 * locals.var_ty_dn7)) + (2.0 * locals.var_acd_dn7)) + (((1.414213562373095 * locals.var_acd_dn7) * locals.var_acd) + (assign76590_e116137 * locals.var_acd_dn7))), (((-(12.0 * locals.var_ty_dn8)) + (2.0 * locals.var_acd_dn8)) + (((1.414213562373095 * locals.var_acd_dn8) * locals.var_acd) + (assign76590_e116137 * locals.var_acd_dn8))), (((-(12.0 * locals.var_ty_dn9)) + (2.0 * locals.var_acd_dn9)) + (((1.414213562373095 * locals.var_acd_dn9) * locals.var_acd) + (assign76590_e116137 * locals.var_acd_dn9))), (((-(12.0 * locals.var_ty_dn10)) + (2.0 * locals.var_acd_dn10)) + (((1.414213562373095 * locals.var_acd_dn10) * locals.var_acd) + (assign76590_e116137 * locals.var_acd_dn10))), (((-(12.0 * locals.var_ty_dn11)) + (2.0 * locals.var_acd_dn11)) + (((1.414213562373095 * locals.var_acd_dn11) * locals.var_acd) + (assign76590_e116137 * locals.var_acd_dn11))), (((-(12.0 * locals.var_ty_dn14)) + (2.0 * locals.var_acd_dn14)) + (((1.414213562373095 * locals.var_acd_dn14) * locals.var_acd) + (assign76590_e116137 * locals.var_acd_dn14))),)
    } else {
        (locals.var_acn, locals.var_acn_dn0, locals.var_acn_dn2, locals.var_acn_dn4, locals.var_acn_dn5, locals.var_acn_dn6, locals.var_acn_dn7, locals.var_acn_dn8, locals.var_acn_dn9, locals.var_acn_dn10, locals.var_acn_dn11, locals.var_acn_dn14,)
    }
};
        locals.var_acn = assign76590_e116142;
        locals.var_acn_dn0 = assign76590_e116142_d_n0;
        locals.var_acn_dn2 = assign76590_e116142_d_n2;
        locals.var_acn_dn4 = assign76590_e116142_d_n4;
        locals.var_acn_dn5 = assign76590_e116142_d_n5;
        locals.var_acn_dn6 = assign76590_e116142_d_n6;
        locals.var_acn_dn7 = assign76590_e116142_d_n7;
        locals.var_acn_dn8 = assign76590_e116142_d_n8;
        locals.var_acn_dn9 = assign76590_e116142_d_n9;
        locals.var_acn_dn10 = assign76590_e116142_d_n10;
        locals.var_acn_dn11 = assign76590_e116142_d_n11;
        locals.var_acn_dn14 = assign76590_e116142_d_n14;
        locals.var_acn_rv = 0.0;

        let (assign76600_e116150, assign76600_e116150_d_n0, assign76600_e116150_d_n2, assign76600_e116150_d_n4, assign76600_e116150_d_n5, assign76600_e116150_d_n6, assign76600_e116150_d_n7, assign76600_e116150_d_n8, assign76600_e116150_d_n9, assign76600_e116150_d_n10, assign76600_e116150_d_n11, assign76600_e116150_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1788 != 0.0)) {
        let assign76600_e116148: f64 = (locals.var_acn / locals.var_acd);
        (assign76600_e116148, (((locals.var_acn_dn0 * locals.var_acd) - (locals.var_acn * locals.var_acd_dn0)) / (locals.var_acd * locals.var_acd)), (((locals.var_acn_dn2 * locals.var_acd) - (locals.var_acn * locals.var_acd_dn2)) / (locals.var_acd * locals.var_acd)), (((locals.var_acn_dn4 * locals.var_acd) - (locals.var_acn * locals.var_acd_dn4)) / (locals.var_acd * locals.var_acd)), (((locals.var_acn_dn5 * locals.var_acd) - (locals.var_acn * locals.var_acd_dn5)) / (locals.var_acd * locals.var_acd)), (((locals.var_acn_dn6 * locals.var_acd) - (locals.var_acn * locals.var_acd_dn6)) / (locals.var_acd * locals.var_acd)), (((locals.var_acn_dn7 * locals.var_acd) - (locals.var_acn * locals.var_acd_dn7)) / (locals.var_acd * locals.var_acd)), (((locals.var_acn_dn8 * locals.var_acd) - (locals.var_acn * locals.var_acd_dn8)) / (locals.var_acd * locals.var_acd)), (((locals.var_acn_dn9 * locals.var_acd) - (locals.var_acn * locals.var_acd_dn9)) / (locals.var_acd * locals.var_acd)), (((locals.var_acn_dn10 * locals.var_acd) - (locals.var_acn * locals.var_acd_dn10)) / (locals.var_acd * locals.var_acd)), (((locals.var_acn_dn11 * locals.var_acd) - (locals.var_acn * locals.var_acd_dn11)) / (locals.var_acd * locals.var_acd)), (((locals.var_acn_dn14 * locals.var_acd) - (locals.var_acn * locals.var_acd_dn14)) / (locals.var_acd * locals.var_acd)),)
    } else {
        (locals.var_chi, locals.var_chi_dn0, locals.var_chi_dn2, locals.var_chi_dn4, locals.var_chi_dn5, locals.var_chi_dn6, locals.var_chi_dn7, locals.var_chi_dn8, locals.var_chi_dn9, locals.var_chi_dn10, locals.var_chi_dn11, locals.var_chi_dn14,)
    }
};
        locals.var_chi = assign76600_e116150;
        locals.var_chi_dn0 = assign76600_e116150_d_n0;
        locals.var_chi_dn2 = assign76600_e116150_d_n2;
        locals.var_chi_dn4 = assign76600_e116150_d_n4;
        locals.var_chi_dn5 = assign76600_e116150_d_n5;
        locals.var_chi_dn6 = assign76600_e116150_d_n6;
        locals.var_chi_dn7 = assign76600_e116150_d_n7;
        locals.var_chi_dn8 = assign76600_e116150_d_n8;
        locals.var_chi_dn9 = assign76600_e116150_d_n9;
        locals.var_chi_dn10 = assign76600_e116150_d_n10;
        locals.var_chi_dn11 = assign76600_e116150_d_n11;
        locals.var_chi_dn14 = assign76600_e116150_d_n14;
        locals.var_chi_rv = 0.0;

        let (assign76610_e116158, assign76610_e116158_d_n0, assign76610_e116158_d_n2, assign76610_e116158_d_n4, assign76610_e116158_d_n5, assign76610_e116158_d_n6, assign76610_e116158_d_n7, assign76610_e116158_d_n8, assign76610_e116158_d_n9, assign76610_e116158_d_n10, assign76610_e116158_d_n11, assign76610_e116158_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1788 != 0.0)) {
        let assign76610_e116156: f64 = (locals.var_chi * locals.var_beta_inv);
        (assign76610_e116156, ((locals.var_chi_dn0 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn0)), ((locals.var_chi_dn2 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn2)), ((locals.var_chi_dn4 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn4)), ((locals.var_chi_dn5 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn5)), ((locals.var_chi_dn6 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn6)), ((locals.var_chi_dn7 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn7)), ((locals.var_chi_dn8 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn8)), ((locals.var_chi_dn9 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn9)), ((locals.var_chi_dn10 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn10)), ((locals.var_chi_dn11 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn11)), ((locals.var_chi_dn14 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn14)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign76610_e116158;
        locals.var_t1_dn0 = assign76610_e116158_d_n0;
        locals.var_t1_dn2 = assign76610_e116158_d_n2;
        locals.var_t1_dn4 = assign76610_e116158_d_n4;
        locals.var_t1_dn5 = assign76610_e116158_d_n5;
        locals.var_t1_dn6 = assign76610_e116158_d_n6;
        locals.var_t1_dn7 = assign76610_e116158_d_n7;
        locals.var_t1_dn8 = assign76610_e116158_d_n8;
        locals.var_t1_dn9 = assign76610_e116158_d_n9;
        locals.var_t1_dn10 = assign76610_e116158_d_n10;
        locals.var_t1_dn11 = assign76610_e116158_d_n11;
        locals.var_t1_dn14 = assign76610_e116158_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign76620_e116166, assign76620_e116166_d_n0, assign76620_e116166_d_n2, assign76620_e116166_d_n4, assign76620_e116166_d_n5, assign76620_e116166_d_n6, assign76620_e116166_d_n7, assign76620_e116166_d_n8, assign76620_e116166_d_n9, assign76620_e116166_d_n10, assign76620_e116166_d_n11, assign76620_e116166_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1788 != 0.0)) {
        let assign76620_e116164: f64 = (locals.var_t1 / locals.var_ps0_min);
        (assign76620_e116164, (((locals.var_t1_dn0 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn0)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn2 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn2)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn4 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn4)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn5 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn5)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn6 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn6)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn7 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn7)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn8 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn8)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn9 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn9)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn10 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn10)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn11 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn11)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn14 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn14)) / (locals.var_ps0_min * locals.var_ps0_min)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign76620_e116166;
        locals.var_t2_dn0 = assign76620_e116166_d_n0;
        locals.var_t2_dn2 = assign76620_e116166_d_n2;
        locals.var_t2_dn4 = assign76620_e116166_d_n4;
        locals.var_t2_dn5 = assign76620_e116166_d_n5;
        locals.var_t2_dn6 = assign76620_e116166_d_n6;
        locals.var_t2_dn7 = assign76620_e116166_d_n7;
        locals.var_t2_dn8 = assign76620_e116166_d_n8;
        locals.var_t2_dn9 = assign76620_e116166_d_n9;
        locals.var_t2_dn10 = assign76620_e116166_d_n10;
        locals.var_t2_dn11 = assign76620_e116166_d_n11;
        locals.var_t2_dn14 = assign76620_e116166_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign76630_e116177, assign76630_e116177_d_n0, assign76630_e116177_d_n2, assign76630_e116177_d_n4, assign76630_e116177_d_n5, assign76630_e116177_d_n6, assign76630_e116177_d_n7, assign76630_e116177_d_n8, assign76630_e116177_d_n9, assign76630_e116177_d_n10, assign76630_e116177_d_n11, assign76630_e116177_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1788 != 0.0)) {
        let assign76630_e116173: f64 = (locals.var_t2 * locals.var_t2);
        let assign76630_e116174: f64 = (1.0 + assign76630_e116173);
        let assign76630_e116175: f64 = (assign76630_e116174).sqrt();
        (assign76630_e116175, (((locals.var_t2_dn0 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn0)) / (2.0 * assign76630_e116175)), (((locals.var_t2_dn2 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn2)) / (2.0 * assign76630_e116175)), (((locals.var_t2_dn4 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn4)) / (2.0 * assign76630_e116175)), (((locals.var_t2_dn5 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn5)) / (2.0 * assign76630_e116175)), (((locals.var_t2_dn6 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn6)) / (2.0 * assign76630_e116175)), (((locals.var_t2_dn7 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn7)) / (2.0 * assign76630_e116175)), (((locals.var_t2_dn8 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn8)) / (2.0 * assign76630_e116175)), (((locals.var_t2_dn9 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn9)) / (2.0 * assign76630_e116175)), (((locals.var_t2_dn10 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn10)) / (2.0 * assign76630_e116175)), (((locals.var_t2_dn11 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn11)) / (2.0 * assign76630_e116175)), (((locals.var_t2_dn14 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn14)) / (2.0 * assign76630_e116175)),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign76630_e116177;
        locals.var_t3_dn0 = assign76630_e116177_d_n0;
        locals.var_t3_dn2 = assign76630_e116177_d_n2;
        locals.var_t3_dn4 = assign76630_e116177_d_n4;
        locals.var_t3_dn5 = assign76630_e116177_d_n5;
        locals.var_t3_dn6 = assign76630_e116177_d_n6;
        locals.var_t3_dn7 = assign76630_e116177_d_n7;
        locals.var_t3_dn8 = assign76630_e116177_d_n8;
        locals.var_t3_dn9 = assign76630_e116177_d_n9;
        locals.var_t3_dn10 = assign76630_e116177_d_n10;
        locals.var_t3_dn11 = assign76630_e116177_d_n11;
        locals.var_t3_dn14 = assign76630_e116177_d_n14;
        locals.var_t3_rv = 0.0;

        let (assign76640_e116187, assign76640_e116187_d_n0, assign76640_e116187_d_n2, assign76640_e116187_d_n4, assign76640_e116187_d_n5, assign76640_e116187_d_n6, assign76640_e116187_d_n7, assign76640_e116187_d_n8, assign76640_e116187_d_n9, assign76640_e116187_d_n10, assign76640_e116187_d_n11, assign76640_e116187_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1788 != 0.0)) {
        let assign76640_e116183: f64 = (locals.var_t1 / locals.var_t3);
        let assign76640_e116185: f64 = (assign76640_e116183 - locals.var_vxbgmtcl);
        (assign76640_e116185, ((((locals.var_t1_dn0 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn0)) / (locals.var_t3 * locals.var_t3)) - locals.var_vxbgmtcl_dn0), ((((locals.var_t1_dn2 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn2)) / (locals.var_t3 * locals.var_t3)) - locals.var_vxbgmtcl_dn2), ((((locals.var_t1_dn4 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn4)) / (locals.var_t3 * locals.var_t3)) - locals.var_vxbgmtcl_dn4), ((((locals.var_t1_dn5 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn5)) / (locals.var_t3 * locals.var_t3)) - locals.var_vxbgmtcl_dn5), ((((locals.var_t1_dn6 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn6)) / (locals.var_t3 * locals.var_t3)) - locals.var_vxbgmtcl_dn6), ((((locals.var_t1_dn7 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn7)) / (locals.var_t3 * locals.var_t3)) - locals.var_vxbgmtcl_dn7), ((((locals.var_t1_dn8 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn8)) / (locals.var_t3 * locals.var_t3)) - locals.var_vxbgmtcl_dn8), ((((locals.var_t1_dn9 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn9)) / (locals.var_t3 * locals.var_t3)) - locals.var_vxbgmtcl_dn9), ((((locals.var_t1_dn10 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn10)) / (locals.var_t3 * locals.var_t3)) - locals.var_vxbgmtcl_dn10), ((((locals.var_t1_dn11 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn11)) / (locals.var_t3 * locals.var_t3)) - locals.var_vxbgmtcl_dn11), ((((locals.var_t1_dn14 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn14)) / (locals.var_t3 * locals.var_t3)) - locals.var_vxbgmtcl_dn14),)
    } else {
        (locals.var_ps0ld, locals.var_ps0ld_dn0, locals.var_ps0ld_dn2, locals.var_ps0ld_dn4, locals.var_ps0ld_dn5, locals.var_ps0ld_dn6, locals.var_ps0ld_dn7, locals.var_ps0ld_dn8, locals.var_ps0ld_dn9, locals.var_ps0ld_dn10, locals.var_ps0ld_dn11, locals.var_ps0ld_dn14,)
    }
};
        locals.var_ps0ld = assign76640_e116187;
        locals.var_ps0ld_dn0 = assign76640_e116187_d_n0;
        locals.var_ps0ld_dn2 = assign76640_e116187_d_n2;
        locals.var_ps0ld_dn4 = assign76640_e116187_d_n4;
        locals.var_ps0ld_dn5 = assign76640_e116187_d_n5;
        locals.var_ps0ld_dn6 = assign76640_e116187_d_n6;
        locals.var_ps0ld_dn7 = assign76640_e116187_d_n7;
        locals.var_ps0ld_dn8 = assign76640_e116187_d_n8;
        locals.var_ps0ld_dn9 = assign76640_e116187_d_n9;
        locals.var_ps0ld_dn10 = assign76640_e116187_d_n10;
        locals.var_ps0ld_dn11 = assign76640_e116187_d_n11;
        locals.var_ps0ld_dn14 = assign76640_e116187_d_n14;
        locals.var_ps0ld_rv = 0.0;

        let (assign76650_e116195, assign76650_e116195_d_n0, assign76650_e116195_d_n2, assign76650_e116195_d_n4, assign76650_e116195_d_n5, assign76650_e116195_d_n6, assign76650_e116195_d_n7, assign76650_e116195_d_n8, assign76650_e116195_d_n9, assign76650_e116195_d_n10, assign76650_e116195_d_n11, assign76650_e116195_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1788 != 0.0)) {
        let assign76650_e116193: f64 = (locals.var_vgpld - locals.var_ps0ld);
        (assign76650_e116193, (-locals.var_ps0ld_dn0), (locals.var_vgpld_dn2 - locals.var_ps0ld_dn2), (-locals.var_ps0ld_dn4), (-locals.var_ps0ld_dn5), (-locals.var_ps0ld_dn6), (locals.var_vgpld_dn7 - locals.var_ps0ld_dn7), (locals.var_vgpld_dn8 - locals.var_ps0ld_dn8), (locals.var_vgpld_dn9 - locals.var_ps0ld_dn9), (-locals.var_ps0ld_dn10), (-locals.var_ps0ld_dn11), (-locals.var_ps0ld_dn14),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign76650_e116195;
        locals.var_t2_dn0 = assign76650_e116195_d_n0;
        locals.var_t2_dn2 = assign76650_e116195_d_n2;
        locals.var_t2_dn4 = assign76650_e116195_d_n4;
        locals.var_t2_dn5 = assign76650_e116195_d_n5;
        locals.var_t2_dn6 = assign76650_e116195_d_n6;
        locals.var_t2_dn7 = assign76650_e116195_d_n7;
        locals.var_t2_dn8 = assign76650_e116195_d_n8;
        locals.var_t2_dn9 = assign76650_e116195_d_n9;
        locals.var_t2_dn10 = assign76650_e116195_d_n10;
        locals.var_t2_dn11 = assign76650_e116195_d_n11;
        locals.var_t2_dn14 = assign76650_e116195_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign76660_e116203, assign76660_e116203_d_n0, assign76660_e116203_d_n2, assign76660_e116203_d_n4, assign76660_e116203_d_n5, assign76660_e116203_d_n6, assign76660_e116203_d_n7, assign76660_e116203_d_n8, assign76660_e116203_d_n9, assign76660_e116203_d_n10, assign76660_e116203_d_n11, assign76660_e116203_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1788 != 0.0)) {
        let assign76660_e116201: f64 = (locals.var_cox0_func * locals.var_t2);
        (assign76660_e116201, (locals.var_cox0_func * locals.var_t2_dn0), (locals.var_cox0_func * locals.var_t2_dn2), (locals.var_cox0_func * locals.var_t2_dn4), (locals.var_cox0_func * locals.var_t2_dn5), (locals.var_cox0_func * locals.var_t2_dn6), (locals.var_cox0_func * locals.var_t2_dn7), (locals.var_cox0_func * locals.var_t2_dn8), (locals.var_cox0_func * locals.var_t2_dn9), (locals.var_cox0_func * locals.var_t2_dn10), (locals.var_cox0_func * locals.var_t2_dn11), (locals.var_cox0_func * locals.var_t2_dn14),)
    } else {
        (locals.var_qsuld, locals.var_qsuld_dn0, locals.var_qsuld_dn2, locals.var_qsuld_dn4, locals.var_qsuld_dn5, locals.var_qsuld_dn6, locals.var_qsuld_dn7, locals.var_qsuld_dn8, locals.var_qsuld_dn9, locals.var_qsuld_dn10, locals.var_qsuld_dn11, locals.var_qsuld_dn14,)
    }
};
        locals.var_qsuld = assign76660_e116203;
        locals.var_qsuld_dn0 = assign76660_e116203_d_n0;
        locals.var_qsuld_dn2 = assign76660_e116203_d_n2;
        locals.var_qsuld_dn4 = assign76660_e116203_d_n4;
        locals.var_qsuld_dn5 = assign76660_e116203_d_n5;
        locals.var_qsuld_dn6 = assign76660_e116203_d_n6;
        locals.var_qsuld_dn7 = assign76660_e116203_d_n7;
        locals.var_qsuld_dn8 = assign76660_e116203_d_n8;
        locals.var_qsuld_dn9 = assign76660_e116203_d_n9;
        locals.var_qsuld_dn10 = assign76660_e116203_d_n10;
        locals.var_qsuld_dn11 = assign76660_e116203_d_n11;
        locals.var_qsuld_dn14 = assign76660_e116203_d_n14;
        locals.var_qsuld_rv = 0.0;

        let (assign76670_e116209, assign76670_e116209_d_n0, assign76670_e116209_d_n2, assign76670_e116209_d_n4, assign76670_e116209_d_n5, assign76670_e116209_d_n6, assign76670_e116209_d_n7, assign76670_e116209_d_n8, assign76670_e116209_d_n9, assign76670_e116209_d_n10, assign76670_e116209_d_n11, assign76670_e116209_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1788 != 0.0)) {
        (locals.var_qsuld, locals.var_qsuld_dn0, locals.var_qsuld_dn2, locals.var_qsuld_dn4, locals.var_qsuld_dn5, locals.var_qsuld_dn6, locals.var_qsuld_dn7, locals.var_qsuld_dn8, locals.var_qsuld_dn9, locals.var_qsuld_dn10, locals.var_qsuld_dn11, locals.var_qsuld_dn14,)
    } else {
        (locals.var_qbuld, locals.var_qbuld_dn0, locals.var_qbuld_dn2, locals.var_qbuld_dn4, locals.var_qbuld_dn5, locals.var_qbuld_dn6, locals.var_qbuld_dn7, locals.var_qbuld_dn8, locals.var_qbuld_dn9, locals.var_qbuld_dn10, locals.var_qbuld_dn11, locals.var_qbuld_dn14,)
    }
};
        locals.var_qbuld = assign76670_e116209;
        locals.var_qbuld_dn0 = assign76670_e116209_d_n0;
        locals.var_qbuld_dn2 = assign76670_e116209_d_n2;
        locals.var_qbuld_dn4 = assign76670_e116209_d_n4;
        locals.var_qbuld_dn5 = assign76670_e116209_d_n5;
        locals.var_qbuld_dn6 = assign76670_e116209_d_n6;
        locals.var_qbuld_dn7 = assign76670_e116209_d_n7;
        locals.var_qbuld_dn8 = assign76670_e116209_d_n8;
        locals.var_qbuld_dn9 = assign76670_e116209_d_n9;
        locals.var_qbuld_dn10 = assign76670_e116209_d_n10;
        locals.var_qbuld_dn11 = assign76670_e116209_d_n11;
        locals.var_qbuld_dn14 = assign76670_e116209_d_n14;
        locals.var_qbuld_rv = 0.0;

        let (assign76680_e116215, assign76680_e116215_d_n0, assign76680_e116215_d_n2, assign76680_e116215_d_n4, assign76680_e116215_d_n5, assign76680_e116215_d_n6, assign76680_e116215_d_n7, assign76680_e116215_d_n8, assign76680_e116215_d_n9, assign76680_e116215_d_n10, assign76680_e116215_d_n11, assign76680_e116215_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1788 != 0.0)) {
        (locals.var_ps0ld, locals.var_ps0ld_dn0, locals.var_ps0ld_dn2, locals.var_ps0ld_dn4, locals.var_ps0ld_dn5, locals.var_ps0ld_dn6, locals.var_ps0ld_dn7, locals.var_ps0ld_dn8, locals.var_ps0ld_dn9, locals.var_ps0ld_dn10, locals.var_ps0ld_dn11, locals.var_ps0ld_dn14,)
    } else {
        (locals.var_ps0ld_ini__blk1771, locals.var_ps0ld_ini__blk1771_dn0, locals.var_ps0ld_ini__blk1771_dn2, locals.var_ps0ld_ini__blk1771_dn4, locals.var_ps0ld_ini__blk1771_dn5, locals.var_ps0ld_ini__blk1771_dn6, locals.var_ps0ld_ini__blk1771_dn7, locals.var_ps0ld_ini__blk1771_dn8, locals.var_ps0ld_ini__blk1771_dn9, locals.var_ps0ld_ini__blk1771_dn10, locals.var_ps0ld_ini__blk1771_dn11, locals.var_ps0ld_ini__blk1771_dn14,)
    }
};
        locals.var_ps0ld_ini__blk1771 = assign76680_e116215;
        locals.var_ps0ld_ini__blk1771_dn0 = assign76680_e116215_d_n0;
        locals.var_ps0ld_ini__blk1771_dn2 = assign76680_e116215_d_n2;
        locals.var_ps0ld_ini__blk1771_dn4 = assign76680_e116215_d_n4;
        locals.var_ps0ld_ini__blk1771_dn5 = assign76680_e116215_d_n5;
        locals.var_ps0ld_ini__blk1771_dn6 = assign76680_e116215_d_n6;
        locals.var_ps0ld_ini__blk1771_dn7 = assign76680_e116215_d_n7;
        locals.var_ps0ld_ini__blk1771_dn8 = assign76680_e116215_d_n8;
        locals.var_ps0ld_ini__blk1771_dn9 = assign76680_e116215_d_n9;
        locals.var_ps0ld_ini__blk1771_dn10 = assign76680_e116215_d_n10;
        locals.var_ps0ld_ini__blk1771_dn11 = assign76680_e116215_d_n11;
        locals.var_ps0ld_ini__blk1771_dn14 = assign76680_e116215_d_n14;
        locals.var_ps0ld_ini__blk1771_rv = 0.0;

        let assign76690_e116219: f64 = (locals.var_vgpld + locals.var_vxbgmtcl);
        let assign76690_e116220: f64 = (locals.var_beta * assign76690_e116219);
        let assign76690_e116224: f64 = (10.0 * 2.220446049250313e-16);
        let assign76690_e116226: f64 = (assign76690_e116224 - 1.0);
        let assign76690_e116228: f64 = (assign76690_e116226 * locals.var_fac1p2);
        let assign76690_e116230: f64 = (assign76690_e116228 * locals.var_beta2);
        let assign76690_e116232: f64 = (assign76690_e116230 / 4.0);
        let assign76690_e116233: f64 = (1.0 + assign76690_e116232);
        let assign76690_e116234: f64 = if assign76690_e116220 < assign76690_e116233 { 1.0 } else { 0.0 };
        locals.var_guard1790 = assign76690_e116234;
        locals.var_guard1790_rv = 0.0;

        let (assign76700_e116249, assign76700_e116249_d_n0, assign76700_e116249_d_n2, assign76700_e116249_d_n4, assign76700_e116249_d_n5, assign76700_e116249_d_n6, assign76700_e116249_d_n7, assign76700_e116249_d_n8, assign76700_e116249_d_n9, assign76700_e116249_d_n10, assign76700_e116249_d_n11, assign76700_e116249_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1788 == 0.0)) && (locals.var_guard1790 != 0.0)) {
        let assign76700_e116244: f64 = (locals.var_fac1p2 * locals.var_beta);
        let assign76700_e116246: f64 = (assign76700_e116244 / 2.0);
        let assign76700_e116247: f64 = (locals.var_vgpld + assign76700_e116246);
        (assign76700_e116247, (((locals.var_fac1p2_dn0 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn0)) / 2.0), (locals.var_vgpld_dn2 + (((locals.var_fac1p2_dn2 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn2)) / 2.0)), (((locals.var_fac1p2_dn4 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn4)) / 2.0), (((locals.var_fac1p2_dn5 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn5)) / 2.0), (((locals.var_fac1p2_dn6 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn6)) / 2.0), (locals.var_vgpld_dn7 + (((locals.var_fac1p2_dn7 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn7)) / 2.0)), (locals.var_vgpld_dn8 + (((locals.var_fac1p2_dn8 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn8)) / 2.0)), (locals.var_vgpld_dn9 + (((locals.var_fac1p2_dn9 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn9)) / 2.0)), (((locals.var_fac1p2_dn10 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn10)) / 2.0), (((locals.var_fac1p2_dn11 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn11)) / 2.0), (((locals.var_fac1p2_dn14 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn14)) / 2.0),)
    } else {
        (locals.var_ps0_inia, locals.var_ps0_inia_dn0, locals.var_ps0_inia_dn2, locals.var_ps0_inia_dn4, locals.var_ps0_inia_dn5, locals.var_ps0_inia_dn6, locals.var_ps0_inia_dn7, locals.var_ps0_inia_dn8, locals.var_ps0_inia_dn9, locals.var_ps0_inia_dn10, locals.var_ps0_inia_dn11, locals.var_ps0_inia_dn14,)
    }
};
        locals.var_ps0_inia = assign76700_e116249;
        locals.var_ps0_inia_dn0 = assign76700_e116249_d_n0;
        locals.var_ps0_inia_dn2 = assign76700_e116249_d_n2;
        locals.var_ps0_inia_dn4 = assign76700_e116249_d_n4;
        locals.var_ps0_inia_dn5 = assign76700_e116249_d_n5;
        locals.var_ps0_inia_dn6 = assign76700_e116249_d_n6;
        locals.var_ps0_inia_dn7 = assign76700_e116249_d_n7;
        locals.var_ps0_inia_dn8 = assign76700_e116249_d_n8;
        locals.var_ps0_inia_dn9 = assign76700_e116249_d_n9;
        locals.var_ps0_inia_dn10 = assign76700_e116249_d_n10;
        locals.var_ps0_inia_dn11 = assign76700_e116249_d_n11;
        locals.var_ps0_inia_dn14 = assign76700_e116249_d_n14;
        locals.var_ps0_inia_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_290(
        locals: &mut StampLocals,
    ) {
        let (assign76710_e116273, assign76710_e116273_d_n0, assign76710_e116273_d_n2, assign76710_e116273_d_n4, assign76710_e116273_d_n5, assign76710_e116273_d_n6, assign76710_e116273_d_n7, assign76710_e116273_d_n8, assign76710_e116273_d_n9, assign76710_e116273_d_n10, assign76710_e116273_d_n11, assign76710_e116273_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1788 == 0.0)) && (locals.var_guard1790 == 0.0)) {
        let assign76710_e116262: f64 = (locals.var_vgpld + locals.var_vxbgmtcl);
        let assign76710_e116263: f64 = (locals.var_beta * assign76710_e116262);
        let assign76710_e116265: f64 = (assign76710_e116263 - 1.0);
        let assign76710_e116266: f64 = (4.0 * assign76710_e116265);
        let assign76710_e116269: f64 = (locals.var_fac1p2 * locals.var_beta2);
        let assign76710_e116270: f64 = (assign76710_e116266 / assign76710_e116269);
        let assign76710_e116271: f64 = (1.0 + assign76710_e116270);
        (assign76710_e116271, ((((4.0 * ((locals.var_beta_dn0 * assign76710_e116262) + (locals.var_beta * locals.var_vxbgmtcl_dn0))) * assign76710_e116269) - (assign76710_e116266 * ((locals.var_fac1p2_dn0 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn0)))) / (assign76710_e116269 * assign76710_e116269)), ((((4.0 * ((locals.var_beta_dn2 * assign76710_e116262) + (locals.var_beta * (locals.var_vgpld_dn2 + locals.var_vxbgmtcl_dn2)))) * assign76710_e116269) - (assign76710_e116266 * ((locals.var_fac1p2_dn2 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn2)))) / (assign76710_e116269 * assign76710_e116269)), ((((4.0 * ((locals.var_beta_dn4 * assign76710_e116262) + (locals.var_beta * locals.var_vxbgmtcl_dn4))) * assign76710_e116269) - (assign76710_e116266 * ((locals.var_fac1p2_dn4 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn4)))) / (assign76710_e116269 * assign76710_e116269)), ((((4.0 * ((locals.var_beta_dn5 * assign76710_e116262) + (locals.var_beta * locals.var_vxbgmtcl_dn5))) * assign76710_e116269) - (assign76710_e116266 * ((locals.var_fac1p2_dn5 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn5)))) / (assign76710_e116269 * assign76710_e116269)), ((((4.0 * ((locals.var_beta_dn6 * assign76710_e116262) + (locals.var_beta * locals.var_vxbgmtcl_dn6))) * assign76710_e116269) - (assign76710_e116266 * ((locals.var_fac1p2_dn6 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn6)))) / (assign76710_e116269 * assign76710_e116269)), ((((4.0 * ((locals.var_beta_dn7 * assign76710_e116262) + (locals.var_beta * (locals.var_vgpld_dn7 + locals.var_vxbgmtcl_dn7)))) * assign76710_e116269) - (assign76710_e116266 * ((locals.var_fac1p2_dn7 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn7)))) / (assign76710_e116269 * assign76710_e116269)), ((((4.0 * ((locals.var_beta_dn8 * assign76710_e116262) + (locals.var_beta * (locals.var_vgpld_dn8 + locals.var_vxbgmtcl_dn8)))) * assign76710_e116269) - (assign76710_e116266 * ((locals.var_fac1p2_dn8 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn8)))) / (assign76710_e116269 * assign76710_e116269)), ((((4.0 * ((locals.var_beta_dn9 * assign76710_e116262) + (locals.var_beta * (locals.var_vgpld_dn9 + locals.var_vxbgmtcl_dn9)))) * assign76710_e116269) - (assign76710_e116266 * ((locals.var_fac1p2_dn9 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn9)))) / (assign76710_e116269 * assign76710_e116269)), ((((4.0 * ((locals.var_beta_dn10 * assign76710_e116262) + (locals.var_beta * locals.var_vxbgmtcl_dn10))) * assign76710_e116269) - (assign76710_e116266 * ((locals.var_fac1p2_dn10 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn10)))) / (assign76710_e116269 * assign76710_e116269)), ((((4.0 * ((locals.var_beta_dn11 * assign76710_e116262) + (locals.var_beta * locals.var_vxbgmtcl_dn11))) * assign76710_e116269) - (assign76710_e116266 * ((locals.var_fac1p2_dn11 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn11)))) / (assign76710_e116269 * assign76710_e116269)), ((((4.0 * ((locals.var_beta_dn14 * assign76710_e116262) + (locals.var_beta * locals.var_vxbgmtcl_dn14))) * assign76710_e116269) - (assign76710_e116266 * ((locals.var_fac1p2_dn14 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn14)))) / (assign76710_e116269 * assign76710_e116269)),)
    } else {
        (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn4, locals.var_tx_dn5, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn8, locals.var_tx_dn9, locals.var_tx_dn10, locals.var_tx_dn11, locals.var_tx_dn14,)
    }
};
        locals.var_tx = assign76710_e116273;
        locals.var_tx_dn0 = assign76710_e116273_d_n0;
        locals.var_tx_dn2 = assign76710_e116273_d_n2;
        locals.var_tx_dn4 = assign76710_e116273_d_n4;
        locals.var_tx_dn5 = assign76710_e116273_d_n5;
        locals.var_tx_dn6 = assign76710_e116273_d_n6;
        locals.var_tx_dn7 = assign76710_e116273_d_n7;
        locals.var_tx_dn8 = assign76710_e116273_d_n8;
        locals.var_tx_dn9 = assign76710_e116273_d_n9;
        locals.var_tx_dn10 = assign76710_e116273_d_n10;
        locals.var_tx_dn11 = assign76710_e116273_d_n11;
        locals.var_tx_dn14 = assign76710_e116273_d_n14;
        locals.var_tx_rv = 0.0;

        let (assign76720_e116294, assign76720_e116294_d_n0, assign76720_e116294_d_n2, assign76720_e116294_d_n4, assign76720_e116294_d_n5, assign76720_e116294_d_n6, assign76720_e116294_d_n7, assign76720_e116294_d_n8, assign76720_e116294_d_n9, assign76720_e116294_d_n10, assign76720_e116294_d_n11, assign76720_e116294_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1788 == 0.0)) && (locals.var_guard1790 == 0.0)) {
        let assign76720_e116284: f64 = (locals.var_fac1p2 * locals.var_beta);
        let assign76720_e116286: f64 = (assign76720_e116284 / 2.0);
        let assign76720_e116289: f64 = (locals.var_tx).sqrt();
        let assign76720_e116290: f64 = (1.0 - assign76720_e116289);
        let assign76720_e116291: f64 = (assign76720_e116286 * assign76720_e116290);
        let assign76720_e116292: f64 = (locals.var_vgpld + assign76720_e116291);
        (assign76720_e116292, (((((locals.var_fac1p2_dn0 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn0)) / 2.0) * assign76720_e116290) + (assign76720_e116286 * (-(locals.var_tx_dn0 / (2.0 * assign76720_e116289))))), (locals.var_vgpld_dn2 + (((((locals.var_fac1p2_dn2 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn2)) / 2.0) * assign76720_e116290) + (assign76720_e116286 * (-(locals.var_tx_dn2 / (2.0 * assign76720_e116289)))))), (((((locals.var_fac1p2_dn4 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn4)) / 2.0) * assign76720_e116290) + (assign76720_e116286 * (-(locals.var_tx_dn4 / (2.0 * assign76720_e116289))))), (((((locals.var_fac1p2_dn5 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn5)) / 2.0) * assign76720_e116290) + (assign76720_e116286 * (-(locals.var_tx_dn5 / (2.0 * assign76720_e116289))))), (((((locals.var_fac1p2_dn6 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn6)) / 2.0) * assign76720_e116290) + (assign76720_e116286 * (-(locals.var_tx_dn6 / (2.0 * assign76720_e116289))))), (locals.var_vgpld_dn7 + (((((locals.var_fac1p2_dn7 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn7)) / 2.0) * assign76720_e116290) + (assign76720_e116286 * (-(locals.var_tx_dn7 / (2.0 * assign76720_e116289)))))), (locals.var_vgpld_dn8 + (((((locals.var_fac1p2_dn8 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn8)) / 2.0) * assign76720_e116290) + (assign76720_e116286 * (-(locals.var_tx_dn8 / (2.0 * assign76720_e116289)))))), (locals.var_vgpld_dn9 + (((((locals.var_fac1p2_dn9 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn9)) / 2.0) * assign76720_e116290) + (assign76720_e116286 * (-(locals.var_tx_dn9 / (2.0 * assign76720_e116289)))))), (((((locals.var_fac1p2_dn10 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn10)) / 2.0) * assign76720_e116290) + (assign76720_e116286 * (-(locals.var_tx_dn10 / (2.0 * assign76720_e116289))))), (((((locals.var_fac1p2_dn11 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn11)) / 2.0) * assign76720_e116290) + (assign76720_e116286 * (-(locals.var_tx_dn11 / (2.0 * assign76720_e116289))))), (((((locals.var_fac1p2_dn14 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn14)) / 2.0) * assign76720_e116290) + (assign76720_e116286 * (-(locals.var_tx_dn14 / (2.0 * assign76720_e116289))))),)
    } else {
        (locals.var_ps0_inia, locals.var_ps0_inia_dn0, locals.var_ps0_inia_dn2, locals.var_ps0_inia_dn4, locals.var_ps0_inia_dn5, locals.var_ps0_inia_dn6, locals.var_ps0_inia_dn7, locals.var_ps0_inia_dn8, locals.var_ps0_inia_dn9, locals.var_ps0_inia_dn10, locals.var_ps0_inia_dn11, locals.var_ps0_inia_dn14,)
    }
};
        locals.var_ps0_inia = assign76720_e116294;
        locals.var_ps0_inia_dn0 = assign76720_e116294_d_n0;
        locals.var_ps0_inia_dn2 = assign76720_e116294_d_n2;
        locals.var_ps0_inia_dn4 = assign76720_e116294_d_n4;
        locals.var_ps0_inia_dn5 = assign76720_e116294_d_n5;
        locals.var_ps0_inia_dn6 = assign76720_e116294_d_n6;
        locals.var_ps0_inia_dn7 = assign76720_e116294_d_n7;
        locals.var_ps0_inia_dn8 = assign76720_e116294_d_n8;
        locals.var_ps0_inia_dn9 = assign76720_e116294_d_n9;
        locals.var_ps0_inia_dn10 = assign76720_e116294_d_n10;
        locals.var_ps0_inia_dn11 = assign76720_e116294_d_n11;
        locals.var_ps0_inia_dn14 = assign76720_e116294_d_n14;
        locals.var_ps0_inia_rv = 0.0;

        let (assign76730_e116305, assign76730_e116305_d_n0, assign76730_e116305_d_n2, assign76730_e116305_d_n4, assign76730_e116305_d_n5, assign76730_e116305_d_n6, assign76730_e116305_d_n7, assign76730_e116305_d_n8, assign76730_e116305_d_n9, assign76730_e116305_d_n10, assign76730_e116305_d_n11, assign76730_e116305_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1788 == 0.0)) {
        let assign76730_e116302: f64 = (locals.var_ps0_inia + locals.var_vxbgmtcl);
        let assign76730_e116303: f64 = (locals.var_beta * assign76730_e116302);
        (assign76730_e116303, ((locals.var_beta_dn0 * assign76730_e116302) + (locals.var_beta * (locals.var_ps0_inia_dn0 + locals.var_vxbgmtcl_dn0))), ((locals.var_beta_dn2 * assign76730_e116302) + (locals.var_beta * (locals.var_ps0_inia_dn2 + locals.var_vxbgmtcl_dn2))), ((locals.var_beta_dn4 * assign76730_e116302) + (locals.var_beta * (locals.var_ps0_inia_dn4 + locals.var_vxbgmtcl_dn4))), ((locals.var_beta_dn5 * assign76730_e116302) + (locals.var_beta * (locals.var_ps0_inia_dn5 + locals.var_vxbgmtcl_dn5))), ((locals.var_beta_dn6 * assign76730_e116302) + (locals.var_beta * (locals.var_ps0_inia_dn6 + locals.var_vxbgmtcl_dn6))), ((locals.var_beta_dn7 * assign76730_e116302) + (locals.var_beta * (locals.var_ps0_inia_dn7 + locals.var_vxbgmtcl_dn7))), ((locals.var_beta_dn8 * assign76730_e116302) + (locals.var_beta * (locals.var_ps0_inia_dn8 + locals.var_vxbgmtcl_dn8))), ((locals.var_beta_dn9 * assign76730_e116302) + (locals.var_beta * (locals.var_ps0_inia_dn9 + locals.var_vxbgmtcl_dn9))), ((locals.var_beta_dn10 * assign76730_e116302) + (locals.var_beta * (locals.var_ps0_inia_dn10 + locals.var_vxbgmtcl_dn10))), ((locals.var_beta_dn11 * assign76730_e116302) + (locals.var_beta * (locals.var_ps0_inia_dn11 + locals.var_vxbgmtcl_dn11))), ((locals.var_beta_dn14 * assign76730_e116302) + (locals.var_beta * (locals.var_ps0_inia_dn14 + locals.var_vxbgmtcl_dn14))),)
    } else {
        (locals.var_chi, locals.var_chi_dn0, locals.var_chi_dn2, locals.var_chi_dn4, locals.var_chi_dn5, locals.var_chi_dn6, locals.var_chi_dn7, locals.var_chi_dn8, locals.var_chi_dn9, locals.var_chi_dn10, locals.var_chi_dn11, locals.var_chi_dn14,)
    }
};
        locals.var_chi = assign76730_e116305;
        locals.var_chi_dn0 = assign76730_e116305_d_n0;
        locals.var_chi_dn2 = assign76730_e116305_d_n2;
        locals.var_chi_dn4 = assign76730_e116305_d_n4;
        locals.var_chi_dn5 = assign76730_e116305_d_n5;
        locals.var_chi_dn6 = assign76730_e116305_d_n6;
        locals.var_chi_dn7 = assign76730_e116305_d_n7;
        locals.var_chi_dn8 = assign76730_e116305_d_n8;
        locals.var_chi_dn9 = assign76730_e116305_d_n9;
        locals.var_chi_dn10 = assign76730_e116305_d_n10;
        locals.var_chi_dn11 = assign76730_e116305_d_n11;
        locals.var_chi_dn14 = assign76730_e116305_d_n14;
        locals.var_chi_rv = 0.0;

        let assign76740_e116308: f64 = if locals.var_chi >= 3.0 { 1.0 } else { 0.0 };
        locals.var_guard1791 = assign76740_e116308;
        locals.var_guard1791_rv = 0.0;

        let (assign76760_e116328, assign76760_e116328_d_n0, assign76760_e116328_d_n2, assign76760_e116328_d_n4, assign76760_e116328_d_n5, assign76760_e116328_d_n6, assign76760_e116328_d_n7, assign76760_e116328_d_n8, assign76760_e116328_d_n9, assign76760_e116328_d_n10, assign76760_e116328_d_n11, assign76760_e116328_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1788 == 0.0)) && (locals.var_guard1791 != 0.0)) {
        let assign76760_e116325: f64 = (-locals.var_chi);
        let assign76760_e116326: f64 = (assign76760_e116325).exp();
        (assign76760_e116326, (assign76760_e116326 * (-locals.var_chi_dn0)), (assign76760_e116326 * (-locals.var_chi_dn2)), (assign76760_e116326 * (-locals.var_chi_dn4)), (assign76760_e116326 * (-locals.var_chi_dn5)), (assign76760_e116326 * (-locals.var_chi_dn6)), (assign76760_e116326 * (-locals.var_chi_dn7)), (assign76760_e116326 * (-locals.var_chi_dn8)), (assign76760_e116326 * (-locals.var_chi_dn9)), (assign76760_e116326 * (-locals.var_chi_dn10)), (assign76760_e116326 * (-locals.var_chi_dn11)), (assign76760_e116326 * (-locals.var_chi_dn14)),)
    } else {
        (locals.var_ty, locals.var_ty_dn0, locals.var_ty_dn2, locals.var_ty_dn4, locals.var_ty_dn5, locals.var_ty_dn6, locals.var_ty_dn7, locals.var_ty_dn8, locals.var_ty_dn9, locals.var_ty_dn10, locals.var_ty_dn11, locals.var_ty_dn14,)
    }
};
        locals.var_ty = assign76760_e116328;
        locals.var_ty_dn0 = assign76760_e116328_d_n0;
        locals.var_ty_dn2 = assign76760_e116328_d_n2;
        locals.var_ty_dn4 = assign76760_e116328_d_n4;
        locals.var_ty_dn5 = assign76760_e116328_d_n5;
        locals.var_ty_dn6 = assign76760_e116328_d_n6;
        locals.var_ty_dn7 = assign76760_e116328_d_n7;
        locals.var_ty_dn8 = assign76760_e116328_d_n8;
        locals.var_ty_dn9 = assign76760_e116328_d_n9;
        locals.var_ty_dn10 = assign76760_e116328_d_n10;
        locals.var_ty_dn11 = assign76760_e116328_d_n11;
        locals.var_ty_dn14 = assign76760_e116328_d_n14;
        locals.var_ty_rv = 0.0;

        let (assign76770_e116353, assign76770_e116353_d_n0, assign76770_e116353_d_n2, assign76770_e116353_d_n4, assign76770_e116353_d_n5, assign76770_e116353_d_n6, assign76770_e116353_d_n7, assign76770_e116353_d_n8, assign76770_e116353_d_n9, assign76770_e116353_d_n10, assign76770_e116353_d_n11, assign76770_e116353_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1788 == 0.0)) && (locals.var_guard1791 != 0.0)) {
        let assign76770_e116340: f64 = (locals.var_vgpld + locals.var_vxbgmtcl);
        let assign76770_e116341: f64 = (locals.var_beta * assign76770_e116340);
        let assign76770_e116343: f64 = (assign76770_e116341 - 1.0);
        let assign76770_e116345: f64 = (assign76770_e116343 + locals.var_ty);
        let assign76770_e116346: f64 = (4.0 * assign76770_e116345);
        let assign76770_e116349: f64 = (locals.var_fac1p2 * locals.var_beta2);
        let assign76770_e116350: f64 = (assign76770_e116346 / assign76770_e116349);
        let assign76770_e116351: f64 = (1.0 + assign76770_e116350);
        (assign76770_e116351, ((((4.0 * (((locals.var_beta_dn0 * assign76770_e116340) + (locals.var_beta * locals.var_vxbgmtcl_dn0)) + locals.var_ty_dn0)) * assign76770_e116349) - (assign76770_e116346 * ((locals.var_fac1p2_dn0 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn0)))) / (assign76770_e116349 * assign76770_e116349)), ((((4.0 * (((locals.var_beta_dn2 * assign76770_e116340) + (locals.var_beta * (locals.var_vgpld_dn2 + locals.var_vxbgmtcl_dn2))) + locals.var_ty_dn2)) * assign76770_e116349) - (assign76770_e116346 * ((locals.var_fac1p2_dn2 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn2)))) / (assign76770_e116349 * assign76770_e116349)), ((((4.0 * (((locals.var_beta_dn4 * assign76770_e116340) + (locals.var_beta * locals.var_vxbgmtcl_dn4)) + locals.var_ty_dn4)) * assign76770_e116349) - (assign76770_e116346 * ((locals.var_fac1p2_dn4 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn4)))) / (assign76770_e116349 * assign76770_e116349)), ((((4.0 * (((locals.var_beta_dn5 * assign76770_e116340) + (locals.var_beta * locals.var_vxbgmtcl_dn5)) + locals.var_ty_dn5)) * assign76770_e116349) - (assign76770_e116346 * ((locals.var_fac1p2_dn5 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn5)))) / (assign76770_e116349 * assign76770_e116349)), ((((4.0 * (((locals.var_beta_dn6 * assign76770_e116340) + (locals.var_beta * locals.var_vxbgmtcl_dn6)) + locals.var_ty_dn6)) * assign76770_e116349) - (assign76770_e116346 * ((locals.var_fac1p2_dn6 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn6)))) / (assign76770_e116349 * assign76770_e116349)), ((((4.0 * (((locals.var_beta_dn7 * assign76770_e116340) + (locals.var_beta * (locals.var_vgpld_dn7 + locals.var_vxbgmtcl_dn7))) + locals.var_ty_dn7)) * assign76770_e116349) - (assign76770_e116346 * ((locals.var_fac1p2_dn7 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn7)))) / (assign76770_e116349 * assign76770_e116349)), ((((4.0 * (((locals.var_beta_dn8 * assign76770_e116340) + (locals.var_beta * (locals.var_vgpld_dn8 + locals.var_vxbgmtcl_dn8))) + locals.var_ty_dn8)) * assign76770_e116349) - (assign76770_e116346 * ((locals.var_fac1p2_dn8 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn8)))) / (assign76770_e116349 * assign76770_e116349)), ((((4.0 * (((locals.var_beta_dn9 * assign76770_e116340) + (locals.var_beta * (locals.var_vgpld_dn9 + locals.var_vxbgmtcl_dn9))) + locals.var_ty_dn9)) * assign76770_e116349) - (assign76770_e116346 * ((locals.var_fac1p2_dn9 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn9)))) / (assign76770_e116349 * assign76770_e116349)), ((((4.0 * (((locals.var_beta_dn10 * assign76770_e116340) + (locals.var_beta * locals.var_vxbgmtcl_dn10)) + locals.var_ty_dn10)) * assign76770_e116349) - (assign76770_e116346 * ((locals.var_fac1p2_dn10 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn10)))) / (assign76770_e116349 * assign76770_e116349)), ((((4.0 * (((locals.var_beta_dn11 * assign76770_e116340) + (locals.var_beta * locals.var_vxbgmtcl_dn11)) + locals.var_ty_dn11)) * assign76770_e116349) - (assign76770_e116346 * ((locals.var_fac1p2_dn11 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn11)))) / (assign76770_e116349 * assign76770_e116349)), ((((4.0 * (((locals.var_beta_dn14 * assign76770_e116340) + (locals.var_beta * locals.var_vxbgmtcl_dn14)) + locals.var_ty_dn14)) * assign76770_e116349) - (assign76770_e116346 * ((locals.var_fac1p2_dn14 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn14)))) / (assign76770_e116349 * assign76770_e116349)),)
    } else {
        (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn4, locals.var_tx_dn5, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn8, locals.var_tx_dn9, locals.var_tx_dn10, locals.var_tx_dn11, locals.var_tx_dn14,)
    }
};
        locals.var_tx = assign76770_e116353;
        locals.var_tx_dn0 = assign76770_e116353_d_n0;
        locals.var_tx_dn2 = assign76770_e116353_d_n2;
        locals.var_tx_dn4 = assign76770_e116353_d_n4;
        locals.var_tx_dn5 = assign76770_e116353_d_n5;
        locals.var_tx_dn6 = assign76770_e116353_d_n6;
        locals.var_tx_dn7 = assign76770_e116353_d_n7;
        locals.var_tx_dn8 = assign76770_e116353_d_n8;
        locals.var_tx_dn9 = assign76770_e116353_d_n9;
        locals.var_tx_dn10 = assign76770_e116353_d_n10;
        locals.var_tx_dn11 = assign76770_e116353_d_n11;
        locals.var_tx_dn14 = assign76770_e116353_d_n14;
        locals.var_tx_rv = 0.0;

        let (assign76780_e116373, assign76780_e116373_d_n0, assign76780_e116373_d_n2, assign76780_e116373_d_n4, assign76780_e116373_d_n5, assign76780_e116373_d_n6, assign76780_e116373_d_n7, assign76780_e116373_d_n8, assign76780_e116373_d_n9, assign76780_e116373_d_n10, assign76780_e116373_d_n11, assign76780_e116373_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1788 == 0.0)) && (locals.var_guard1791 != 0.0)) {
        let assign76780_e116363: f64 = (locals.var_fac1p2 * locals.var_beta);
        let assign76780_e116365: f64 = (assign76780_e116363 / 2.0);
        let assign76780_e116368: f64 = (locals.var_tx).sqrt();
        let assign76780_e116369: f64 = (1.0 - assign76780_e116368);
        let assign76780_e116370: f64 = (assign76780_e116365 * assign76780_e116369);
        let assign76780_e116371: f64 = (locals.var_vgpld + assign76780_e116370);
        (assign76780_e116371, (((((locals.var_fac1p2_dn0 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn0)) / 2.0) * assign76780_e116369) + (assign76780_e116365 * (-(locals.var_tx_dn0 / (2.0 * assign76780_e116368))))), (locals.var_vgpld_dn2 + (((((locals.var_fac1p2_dn2 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn2)) / 2.0) * assign76780_e116369) + (assign76780_e116365 * (-(locals.var_tx_dn2 / (2.0 * assign76780_e116368)))))), (((((locals.var_fac1p2_dn4 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn4)) / 2.0) * assign76780_e116369) + (assign76780_e116365 * (-(locals.var_tx_dn4 / (2.0 * assign76780_e116368))))), (((((locals.var_fac1p2_dn5 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn5)) / 2.0) * assign76780_e116369) + (assign76780_e116365 * (-(locals.var_tx_dn5 / (2.0 * assign76780_e116368))))), (((((locals.var_fac1p2_dn6 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn6)) / 2.0) * assign76780_e116369) + (assign76780_e116365 * (-(locals.var_tx_dn6 / (2.0 * assign76780_e116368))))), (locals.var_vgpld_dn7 + (((((locals.var_fac1p2_dn7 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn7)) / 2.0) * assign76780_e116369) + (assign76780_e116365 * (-(locals.var_tx_dn7 / (2.0 * assign76780_e116368)))))), (locals.var_vgpld_dn8 + (((((locals.var_fac1p2_dn8 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn8)) / 2.0) * assign76780_e116369) + (assign76780_e116365 * (-(locals.var_tx_dn8 / (2.0 * assign76780_e116368)))))), (locals.var_vgpld_dn9 + (((((locals.var_fac1p2_dn9 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn9)) / 2.0) * assign76780_e116369) + (assign76780_e116365 * (-(locals.var_tx_dn9 / (2.0 * assign76780_e116368)))))), (((((locals.var_fac1p2_dn10 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn10)) / 2.0) * assign76780_e116369) + (assign76780_e116365 * (-(locals.var_tx_dn10 / (2.0 * assign76780_e116368))))), (((((locals.var_fac1p2_dn11 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn11)) / 2.0) * assign76780_e116369) + (assign76780_e116365 * (-(locals.var_tx_dn11 / (2.0 * assign76780_e116368))))), (((((locals.var_fac1p2_dn14 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn14)) / 2.0) * assign76780_e116369) + (assign76780_e116365 * (-(locals.var_tx_dn14 / (2.0 * assign76780_e116368))))),)
    } else {
        (locals.var_ps0_inia, locals.var_ps0_inia_dn0, locals.var_ps0_inia_dn2, locals.var_ps0_inia_dn4, locals.var_ps0_inia_dn5, locals.var_ps0_inia_dn6, locals.var_ps0_inia_dn7, locals.var_ps0_inia_dn8, locals.var_ps0_inia_dn9, locals.var_ps0_inia_dn10, locals.var_ps0_inia_dn11, locals.var_ps0_inia_dn14,)
    }
};
        locals.var_ps0_inia = assign76780_e116373;
        locals.var_ps0_inia_dn0 = assign76780_e116373_d_n0;
        locals.var_ps0_inia_dn2 = assign76780_e116373_d_n2;
        locals.var_ps0_inia_dn4 = assign76780_e116373_d_n4;
        locals.var_ps0_inia_dn5 = assign76780_e116373_d_n5;
        locals.var_ps0_inia_dn6 = assign76780_e116373_d_n6;
        locals.var_ps0_inia_dn7 = assign76780_e116373_d_n7;
        locals.var_ps0_inia_dn8 = assign76780_e116373_d_n8;
        locals.var_ps0_inia_dn9 = assign76780_e116373_d_n9;
        locals.var_ps0_inia_dn10 = assign76780_e116373_d_n10;
        locals.var_ps0_inia_dn11 = assign76780_e116373_d_n11;
        locals.var_ps0_inia_dn14 = assign76780_e116373_d_n14;
        locals.var_ps0_inia_rv = 0.0;

        let (assign76790_e116386, assign76790_e116386_d_n0, assign76790_e116386_d_n2, assign76790_e116386_d_n4, assign76790_e116386_d_n5, assign76790_e116386_d_n6, assign76790_e116386_d_n7, assign76790_e116386_d_n8, assign76790_e116386_d_n9, assign76790_e116386_d_n10, assign76790_e116386_d_n11, assign76790_e116386_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1788 == 0.0)) && (locals.var_guard1791 != 0.0)) {
        let assign76790_e116383: f64 = (locals.var_ps0_inia + locals.var_vxbgmtcl);
        let assign76790_e116384: f64 = (locals.var_beta * assign76790_e116383);
        (assign76790_e116384, ((locals.var_beta_dn0 * assign76790_e116383) + (locals.var_beta * (locals.var_ps0_inia_dn0 + locals.var_vxbgmtcl_dn0))), ((locals.var_beta_dn2 * assign76790_e116383) + (locals.var_beta * (locals.var_ps0_inia_dn2 + locals.var_vxbgmtcl_dn2))), ((locals.var_beta_dn4 * assign76790_e116383) + (locals.var_beta * (locals.var_ps0_inia_dn4 + locals.var_vxbgmtcl_dn4))), ((locals.var_beta_dn5 * assign76790_e116383) + (locals.var_beta * (locals.var_ps0_inia_dn5 + locals.var_vxbgmtcl_dn5))), ((locals.var_beta_dn6 * assign76790_e116383) + (locals.var_beta * (locals.var_ps0_inia_dn6 + locals.var_vxbgmtcl_dn6))), ((locals.var_beta_dn7 * assign76790_e116383) + (locals.var_beta * (locals.var_ps0_inia_dn7 + locals.var_vxbgmtcl_dn7))), ((locals.var_beta_dn8 * assign76790_e116383) + (locals.var_beta * (locals.var_ps0_inia_dn8 + locals.var_vxbgmtcl_dn8))), ((locals.var_beta_dn9 * assign76790_e116383) + (locals.var_beta * (locals.var_ps0_inia_dn9 + locals.var_vxbgmtcl_dn9))), ((locals.var_beta_dn10 * assign76790_e116383) + (locals.var_beta * (locals.var_ps0_inia_dn10 + locals.var_vxbgmtcl_dn10))), ((locals.var_beta_dn11 * assign76790_e116383) + (locals.var_beta * (locals.var_ps0_inia_dn11 + locals.var_vxbgmtcl_dn11))), ((locals.var_beta_dn14 * assign76790_e116383) + (locals.var_beta * (locals.var_ps0_inia_dn14 + locals.var_vxbgmtcl_dn14))),)
    } else {
        (locals.var_chi, locals.var_chi_dn0, locals.var_chi_dn2, locals.var_chi_dn4, locals.var_chi_dn5, locals.var_chi_dn6, locals.var_chi_dn7, locals.var_chi_dn8, locals.var_chi_dn9, locals.var_chi_dn10, locals.var_chi_dn11, locals.var_chi_dn14,)
    }
};
        locals.var_chi = assign76790_e116386;
        locals.var_chi_dn0 = assign76790_e116386_d_n0;
        locals.var_chi_dn2 = assign76790_e116386_d_n2;
        locals.var_chi_dn4 = assign76790_e116386_d_n4;
        locals.var_chi_dn5 = assign76790_e116386_d_n5;
        locals.var_chi_dn6 = assign76790_e116386_d_n6;
        locals.var_chi_dn7 = assign76790_e116386_d_n7;
        locals.var_chi_dn8 = assign76790_e116386_d_n8;
        locals.var_chi_dn9 = assign76790_e116386_d_n9;
        locals.var_chi_dn10 = assign76790_e116386_d_n10;
        locals.var_chi_dn11 = assign76790_e116386_d_n11;
        locals.var_chi_dn14 = assign76790_e116386_d_n14;
        locals.var_chi_rv = 0.0;

        let (assign76800_e116397, assign76800_e116397_d_n0, assign76800_e116397_d_n2, assign76800_e116397_d_n4, assign76800_e116397_d_n5, assign76800_e116397_d_n6, assign76800_e116397_d_n7, assign76800_e116397_d_n8, assign76800_e116397_d_n9, assign76800_e116397_d_n10, assign76800_e116397_d_n11, assign76800_e116397_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1788 == 0.0)) && (locals.var_guard1791 != 0.0)) {
        let assign76800_e116394: f64 = (-locals.var_chi);
        let assign76800_e116395: f64 = (assign76800_e116394).exp();
        (assign76800_e116395, (assign76800_e116395 * (-locals.var_chi_dn0)), (assign76800_e116395 * (-locals.var_chi_dn2)), (assign76800_e116395 * (-locals.var_chi_dn4)), (assign76800_e116395 * (-locals.var_chi_dn5)), (assign76800_e116395 * (-locals.var_chi_dn6)), (assign76800_e116395 * (-locals.var_chi_dn7)), (assign76800_e116395 * (-locals.var_chi_dn8)), (assign76800_e116395 * (-locals.var_chi_dn9)), (assign76800_e116395 * (-locals.var_chi_dn10)), (assign76800_e116395 * (-locals.var_chi_dn11)), (assign76800_e116395 * (-locals.var_chi_dn14)),)
    } else {
        (locals.var_ty, locals.var_ty_dn0, locals.var_ty_dn2, locals.var_ty_dn4, locals.var_ty_dn5, locals.var_ty_dn6, locals.var_ty_dn7, locals.var_ty_dn8, locals.var_ty_dn9, locals.var_ty_dn10, locals.var_ty_dn11, locals.var_ty_dn14,)
    }
};
        locals.var_ty = assign76800_e116397;
        locals.var_ty_dn0 = assign76800_e116397_d_n0;
        locals.var_ty_dn2 = assign76800_e116397_d_n2;
        locals.var_ty_dn4 = assign76800_e116397_d_n4;
        locals.var_ty_dn5 = assign76800_e116397_d_n5;
        locals.var_ty_dn6 = assign76800_e116397_d_n6;
        locals.var_ty_dn7 = assign76800_e116397_d_n7;
        locals.var_ty_dn8 = assign76800_e116397_d_n8;
        locals.var_ty_dn9 = assign76800_e116397_d_n9;
        locals.var_ty_dn10 = assign76800_e116397_d_n10;
        locals.var_ty_dn11 = assign76800_e116397_d_n11;
        locals.var_ty_dn14 = assign76800_e116397_d_n14;
        locals.var_ty_rv = 0.0;

        let (assign76810_e116422, assign76810_e116422_d_n0, assign76810_e116422_d_n2, assign76810_e116422_d_n4, assign76810_e116422_d_n5, assign76810_e116422_d_n6, assign76810_e116422_d_n7, assign76810_e116422_d_n8, assign76810_e116422_d_n9, assign76810_e116422_d_n10, assign76810_e116422_d_n11, assign76810_e116422_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1788 == 0.0)) && (locals.var_guard1791 != 0.0)) {
        let assign76810_e116409: f64 = (locals.var_vgpld + locals.var_vxbgmtcl);
        let assign76810_e116410: f64 = (locals.var_beta * assign76810_e116409);
        let assign76810_e116412: f64 = (assign76810_e116410 - 1.0);
        let assign76810_e116414: f64 = (assign76810_e116412 + locals.var_ty);
        let assign76810_e116415: f64 = (4.0 * assign76810_e116414);
        let assign76810_e116418: f64 = (locals.var_fac1p2 * locals.var_beta2);
        let assign76810_e116419: f64 = (assign76810_e116415 / assign76810_e116418);
        let assign76810_e116420: f64 = (1.0 + assign76810_e116419);
        (assign76810_e116420, ((((4.0 * (((locals.var_beta_dn0 * assign76810_e116409) + (locals.var_beta * locals.var_vxbgmtcl_dn0)) + locals.var_ty_dn0)) * assign76810_e116418) - (assign76810_e116415 * ((locals.var_fac1p2_dn0 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn0)))) / (assign76810_e116418 * assign76810_e116418)), ((((4.0 * (((locals.var_beta_dn2 * assign76810_e116409) + (locals.var_beta * (locals.var_vgpld_dn2 + locals.var_vxbgmtcl_dn2))) + locals.var_ty_dn2)) * assign76810_e116418) - (assign76810_e116415 * ((locals.var_fac1p2_dn2 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn2)))) / (assign76810_e116418 * assign76810_e116418)), ((((4.0 * (((locals.var_beta_dn4 * assign76810_e116409) + (locals.var_beta * locals.var_vxbgmtcl_dn4)) + locals.var_ty_dn4)) * assign76810_e116418) - (assign76810_e116415 * ((locals.var_fac1p2_dn4 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn4)))) / (assign76810_e116418 * assign76810_e116418)), ((((4.0 * (((locals.var_beta_dn5 * assign76810_e116409) + (locals.var_beta * locals.var_vxbgmtcl_dn5)) + locals.var_ty_dn5)) * assign76810_e116418) - (assign76810_e116415 * ((locals.var_fac1p2_dn5 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn5)))) / (assign76810_e116418 * assign76810_e116418)), ((((4.0 * (((locals.var_beta_dn6 * assign76810_e116409) + (locals.var_beta * locals.var_vxbgmtcl_dn6)) + locals.var_ty_dn6)) * assign76810_e116418) - (assign76810_e116415 * ((locals.var_fac1p2_dn6 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn6)))) / (assign76810_e116418 * assign76810_e116418)), ((((4.0 * (((locals.var_beta_dn7 * assign76810_e116409) + (locals.var_beta * (locals.var_vgpld_dn7 + locals.var_vxbgmtcl_dn7))) + locals.var_ty_dn7)) * assign76810_e116418) - (assign76810_e116415 * ((locals.var_fac1p2_dn7 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn7)))) / (assign76810_e116418 * assign76810_e116418)), ((((4.0 * (((locals.var_beta_dn8 * assign76810_e116409) + (locals.var_beta * (locals.var_vgpld_dn8 + locals.var_vxbgmtcl_dn8))) + locals.var_ty_dn8)) * assign76810_e116418) - (assign76810_e116415 * ((locals.var_fac1p2_dn8 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn8)))) / (assign76810_e116418 * assign76810_e116418)), ((((4.0 * (((locals.var_beta_dn9 * assign76810_e116409) + (locals.var_beta * (locals.var_vgpld_dn9 + locals.var_vxbgmtcl_dn9))) + locals.var_ty_dn9)) * assign76810_e116418) - (assign76810_e116415 * ((locals.var_fac1p2_dn9 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn9)))) / (assign76810_e116418 * assign76810_e116418)), ((((4.0 * (((locals.var_beta_dn10 * assign76810_e116409) + (locals.var_beta * locals.var_vxbgmtcl_dn10)) + locals.var_ty_dn10)) * assign76810_e116418) - (assign76810_e116415 * ((locals.var_fac1p2_dn10 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn10)))) / (assign76810_e116418 * assign76810_e116418)), ((((4.0 * (((locals.var_beta_dn11 * assign76810_e116409) + (locals.var_beta * locals.var_vxbgmtcl_dn11)) + locals.var_ty_dn11)) * assign76810_e116418) - (assign76810_e116415 * ((locals.var_fac1p2_dn11 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn11)))) / (assign76810_e116418 * assign76810_e116418)), ((((4.0 * (((locals.var_beta_dn14 * assign76810_e116409) + (locals.var_beta * locals.var_vxbgmtcl_dn14)) + locals.var_ty_dn14)) * assign76810_e116418) - (assign76810_e116415 * ((locals.var_fac1p2_dn14 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn14)))) / (assign76810_e116418 * assign76810_e116418)),)
    } else {
        (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn4, locals.var_tx_dn5, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn8, locals.var_tx_dn9, locals.var_tx_dn10, locals.var_tx_dn11, locals.var_tx_dn14,)
    }
};
        locals.var_tx = assign76810_e116422;
        locals.var_tx_dn0 = assign76810_e116422_d_n0;
        locals.var_tx_dn2 = assign76810_e116422_d_n2;
        locals.var_tx_dn4 = assign76810_e116422_d_n4;
        locals.var_tx_dn5 = assign76810_e116422_d_n5;
        locals.var_tx_dn6 = assign76810_e116422_d_n6;
        locals.var_tx_dn7 = assign76810_e116422_d_n7;
        locals.var_tx_dn8 = assign76810_e116422_d_n8;
        locals.var_tx_dn9 = assign76810_e116422_d_n9;
        locals.var_tx_dn10 = assign76810_e116422_d_n10;
        locals.var_tx_dn11 = assign76810_e116422_d_n11;
        locals.var_tx_dn14 = assign76810_e116422_d_n14;
        locals.var_tx_rv = 0.0;

        let (assign76820_e116442, assign76820_e116442_d_n0, assign76820_e116442_d_n2, assign76820_e116442_d_n4, assign76820_e116442_d_n5, assign76820_e116442_d_n6, assign76820_e116442_d_n7, assign76820_e116442_d_n8, assign76820_e116442_d_n9, assign76820_e116442_d_n10, assign76820_e116442_d_n11, assign76820_e116442_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1788 == 0.0)) && (locals.var_guard1791 != 0.0)) {
        let assign76820_e116432: f64 = (locals.var_fac1p2 * locals.var_beta);
        let assign76820_e116434: f64 = (assign76820_e116432 / 2.0);
        let assign76820_e116437: f64 = (locals.var_tx).sqrt();
        let assign76820_e116438: f64 = (1.0 - assign76820_e116437);
        let assign76820_e116439: f64 = (assign76820_e116434 * assign76820_e116438);
        let assign76820_e116440: f64 = (locals.var_vgpld + assign76820_e116439);
        (assign76820_e116440, (((((locals.var_fac1p2_dn0 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn0)) / 2.0) * assign76820_e116438) + (assign76820_e116434 * (-(locals.var_tx_dn0 / (2.0 * assign76820_e116437))))), (locals.var_vgpld_dn2 + (((((locals.var_fac1p2_dn2 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn2)) / 2.0) * assign76820_e116438) + (assign76820_e116434 * (-(locals.var_tx_dn2 / (2.0 * assign76820_e116437)))))), (((((locals.var_fac1p2_dn4 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn4)) / 2.0) * assign76820_e116438) + (assign76820_e116434 * (-(locals.var_tx_dn4 / (2.0 * assign76820_e116437))))), (((((locals.var_fac1p2_dn5 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn5)) / 2.0) * assign76820_e116438) + (assign76820_e116434 * (-(locals.var_tx_dn5 / (2.0 * assign76820_e116437))))), (((((locals.var_fac1p2_dn6 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn6)) / 2.0) * assign76820_e116438) + (assign76820_e116434 * (-(locals.var_tx_dn6 / (2.0 * assign76820_e116437))))), (locals.var_vgpld_dn7 + (((((locals.var_fac1p2_dn7 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn7)) / 2.0) * assign76820_e116438) + (assign76820_e116434 * (-(locals.var_tx_dn7 / (2.0 * assign76820_e116437)))))), (locals.var_vgpld_dn8 + (((((locals.var_fac1p2_dn8 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn8)) / 2.0) * assign76820_e116438) + (assign76820_e116434 * (-(locals.var_tx_dn8 / (2.0 * assign76820_e116437)))))), (locals.var_vgpld_dn9 + (((((locals.var_fac1p2_dn9 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn9)) / 2.0) * assign76820_e116438) + (assign76820_e116434 * (-(locals.var_tx_dn9 / (2.0 * assign76820_e116437)))))), (((((locals.var_fac1p2_dn10 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn10)) / 2.0) * assign76820_e116438) + (assign76820_e116434 * (-(locals.var_tx_dn10 / (2.0 * assign76820_e116437))))), (((((locals.var_fac1p2_dn11 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn11)) / 2.0) * assign76820_e116438) + (assign76820_e116434 * (-(locals.var_tx_dn11 / (2.0 * assign76820_e116437))))), (((((locals.var_fac1p2_dn14 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn14)) / 2.0) * assign76820_e116438) + (assign76820_e116434 * (-(locals.var_tx_dn14 / (2.0 * assign76820_e116437))))),)
    } else {
        (locals.var_ps0_inia, locals.var_ps0_inia_dn0, locals.var_ps0_inia_dn2, locals.var_ps0_inia_dn4, locals.var_ps0_inia_dn5, locals.var_ps0_inia_dn6, locals.var_ps0_inia_dn7, locals.var_ps0_inia_dn8, locals.var_ps0_inia_dn9, locals.var_ps0_inia_dn10, locals.var_ps0_inia_dn11, locals.var_ps0_inia_dn14,)
    }
};
        locals.var_ps0_inia = assign76820_e116442;
        locals.var_ps0_inia_dn0 = assign76820_e116442_d_n0;
        locals.var_ps0_inia_dn2 = assign76820_e116442_d_n2;
        locals.var_ps0_inia_dn4 = assign76820_e116442_d_n4;
        locals.var_ps0_inia_dn5 = assign76820_e116442_d_n5;
        locals.var_ps0_inia_dn6 = assign76820_e116442_d_n6;
        locals.var_ps0_inia_dn7 = assign76820_e116442_d_n7;
        locals.var_ps0_inia_dn8 = assign76820_e116442_d_n8;
        locals.var_ps0_inia_dn9 = assign76820_e116442_d_n9;
        locals.var_ps0_inia_dn10 = assign76820_e116442_d_n10;
        locals.var_ps0_inia_dn11 = assign76820_e116442_d_n11;
        locals.var_ps0_inia_dn14 = assign76820_e116442_d_n14;
        locals.var_ps0_inia_rv = 0.0;

        let (assign76830_e116455, assign76830_e116455_d_n0, assign76830_e116455_d_n2, assign76830_e116455_d_n4, assign76830_e116455_d_n5, assign76830_e116455_d_n6, assign76830_e116455_d_n7, assign76830_e116455_d_n8, assign76830_e116455_d_n9, assign76830_e116455_d_n10, assign76830_e116455_d_n11, assign76830_e116455_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1788 == 0.0)) && (locals.var_guard1791 != 0.0)) {
        let assign76830_e116452: f64 = (locals.var_ps0_inia + locals.var_vxbgmtcl);
        let assign76830_e116453: f64 = (locals.var_beta * assign76830_e116452);
        (assign76830_e116453, ((locals.var_beta_dn0 * assign76830_e116452) + (locals.var_beta * (locals.var_ps0_inia_dn0 + locals.var_vxbgmtcl_dn0))), ((locals.var_beta_dn2 * assign76830_e116452) + (locals.var_beta * (locals.var_ps0_inia_dn2 + locals.var_vxbgmtcl_dn2))), ((locals.var_beta_dn4 * assign76830_e116452) + (locals.var_beta * (locals.var_ps0_inia_dn4 + locals.var_vxbgmtcl_dn4))), ((locals.var_beta_dn5 * assign76830_e116452) + (locals.var_beta * (locals.var_ps0_inia_dn5 + locals.var_vxbgmtcl_dn5))), ((locals.var_beta_dn6 * assign76830_e116452) + (locals.var_beta * (locals.var_ps0_inia_dn6 + locals.var_vxbgmtcl_dn6))), ((locals.var_beta_dn7 * assign76830_e116452) + (locals.var_beta * (locals.var_ps0_inia_dn7 + locals.var_vxbgmtcl_dn7))), ((locals.var_beta_dn8 * assign76830_e116452) + (locals.var_beta * (locals.var_ps0_inia_dn8 + locals.var_vxbgmtcl_dn8))), ((locals.var_beta_dn9 * assign76830_e116452) + (locals.var_beta * (locals.var_ps0_inia_dn9 + locals.var_vxbgmtcl_dn9))), ((locals.var_beta_dn10 * assign76830_e116452) + (locals.var_beta * (locals.var_ps0_inia_dn10 + locals.var_vxbgmtcl_dn10))), ((locals.var_beta_dn11 * assign76830_e116452) + (locals.var_beta * (locals.var_ps0_inia_dn11 + locals.var_vxbgmtcl_dn11))), ((locals.var_beta_dn14 * assign76830_e116452) + (locals.var_beta * (locals.var_ps0_inia_dn14 + locals.var_vxbgmtcl_dn14))),)
    } else {
        (locals.var_chi, locals.var_chi_dn0, locals.var_chi_dn2, locals.var_chi_dn4, locals.var_chi_dn5, locals.var_chi_dn6, locals.var_chi_dn7, locals.var_chi_dn8, locals.var_chi_dn9, locals.var_chi_dn10, locals.var_chi_dn11, locals.var_chi_dn14,)
    }
};
        locals.var_chi = assign76830_e116455;
        locals.var_chi_dn0 = assign76830_e116455_d_n0;
        locals.var_chi_dn2 = assign76830_e116455_d_n2;
        locals.var_chi_dn4 = assign76830_e116455_d_n4;
        locals.var_chi_dn5 = assign76830_e116455_d_n5;
        locals.var_chi_dn6 = assign76830_e116455_d_n6;
        locals.var_chi_dn7 = assign76830_e116455_d_n7;
        locals.var_chi_dn8 = assign76830_e116455_d_n8;
        locals.var_chi_dn9 = assign76830_e116455_d_n9;
        locals.var_chi_dn10 = assign76830_e116455_d_n10;
        locals.var_chi_dn11 = assign76830_e116455_d_n11;
        locals.var_chi_dn14 = assign76830_e116455_d_n14;
        locals.var_chi_rv = 0.0;

        let (assign76850_e116497,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1788 == 0.0)) && (locals.var_guard1791 == 0.0)) {
        let assign76850_e116476: f64 = (2.0_f64).sqrt();
        let assign76850_e116477: f64 = (9.0 * assign76850_e116476);
        let assign76850_e116478: f64 = (1.0 / assign76850_e116477);
        let assign76850_e116482: f64 = (-3.0);
        let assign76850_e116483: f64 = (assign76850_e116482).exp();
        let assign76850_e116484: f64 = (7.0 * assign76850_e116483);
        let assign76850_e116485: f64 = (5.0 + assign76850_e116484);
        let assign76850_e116489: f64 = (-3.0);
        let assign76850_e116490: f64 = (assign76850_e116489).exp();
        let assign76850_e116491: f64 = (2.0 + assign76850_e116490);
        let assign76850_e116492: f64 = (assign76850_e116491).sqrt();
        let assign76850_e116493: f64 = (54.0 * assign76850_e116492);
        let assign76850_e116494: f64 = (assign76850_e116485 / assign76850_e116493);
        let assign76850_e116495: f64 = (assign76850_e116478 - assign76850_e116494);
        (assign76850_e116495,)
    } else {
        (locals.var_ta,)
    }
};
        locals.var_ta = assign76850_e116497;
        locals.var_ta_rv = 0.0;

        let (assign76860_e116525,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1788 == 0.0)) && (locals.var_guard1791 == 0.0)) {
        let assign76860_e116507: f64 = (-3.0);
        let assign76860_e116508: f64 = (assign76860_e116507).exp();
        let assign76860_e116509: f64 = (1.0 + assign76860_e116508);
        let assign76860_e116513: f64 = (-3.0);
        let assign76860_e116514: f64 = (assign76860_e116513).exp();
        let assign76860_e116515: f64 = (2.0 + assign76860_e116514);
        let assign76860_e116516: f64 = (assign76860_e116515).sqrt();
        let assign76860_e116517: f64 = (2.0 * assign76860_e116516);
        let assign76860_e116518: f64 = (assign76860_e116509 / assign76860_e116517);
        let assign76860_e116520: f64 = (2.0_f64).sqrt();
        let assign76860_e116522: f64 = (assign76860_e116520 / 3.0);
        let assign76860_e116523: f64 = (assign76860_e116518 - assign76860_e116522);
        (assign76860_e116523,)
    } else {
        (locals.var_tb,)
    }
};
        locals.var_tb = assign76860_e116525;
        locals.var_tb_rv = 0.0;

        let (assign76870_e116544, assign76870_e116544_d_n0, assign76870_e116544_d_n2, assign76870_e116544_d_n4, assign76870_e116544_d_n5, assign76870_e116544_d_n6, assign76870_e116544_d_n7, assign76870_e116544_d_n8, assign76870_e116544_d_n9, assign76870_e116544_d_n10, assign76870_e116544_d_n11, assign76870_e116544_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1788 == 0.0)) && (locals.var_guard1791 == 0.0)) {
        let assign76870_e116535: f64 = (2.0_f64).sqrt();
        let assign76870_e116536: f64 = (1.0 / assign76870_e116535);
        let assign76870_e116540: f64 = (locals.var_beta * locals.var_fac1);
        let assign76870_e116541: f64 = (1.0 / assign76870_e116540);
        let assign76870_e116542: f64 = (assign76870_e116536 + assign76870_e116541);
        (assign76870_e116542, (-(((locals.var_beta_dn0 * locals.var_fac1) + (locals.var_beta * locals.var_fac1_dn0)) / (assign76870_e116540 * assign76870_e116540))), (-(((locals.var_beta_dn2 * locals.var_fac1) + (locals.var_beta * locals.var_fac1_dn2)) / (assign76870_e116540 * assign76870_e116540))), (-(((locals.var_beta_dn4 * locals.var_fac1) + (locals.var_beta * locals.var_fac1_dn4)) / (assign76870_e116540 * assign76870_e116540))), (-(((locals.var_beta_dn5 * locals.var_fac1) + (locals.var_beta * locals.var_fac1_dn5)) / (assign76870_e116540 * assign76870_e116540))), (-(((locals.var_beta_dn6 * locals.var_fac1) + (locals.var_beta * locals.var_fac1_dn6)) / (assign76870_e116540 * assign76870_e116540))), (-(((locals.var_beta_dn7 * locals.var_fac1) + (locals.var_beta * locals.var_fac1_dn7)) / (assign76870_e116540 * assign76870_e116540))), (-(((locals.var_beta_dn8 * locals.var_fac1) + (locals.var_beta * locals.var_fac1_dn8)) / (assign76870_e116540 * assign76870_e116540))), (-(((locals.var_beta_dn9 * locals.var_fac1) + (locals.var_beta * locals.var_fac1_dn9)) / (assign76870_e116540 * assign76870_e116540))), (-(((locals.var_beta_dn10 * locals.var_fac1) + (locals.var_beta * locals.var_fac1_dn10)) / (assign76870_e116540 * assign76870_e116540))), (-(((locals.var_beta_dn11 * locals.var_fac1) + (locals.var_beta * locals.var_fac1_dn11)) / (assign76870_e116540 * assign76870_e116540))), (-(((locals.var_beta_dn14 * locals.var_fac1) + (locals.var_beta * locals.var_fac1_dn14)) / (assign76870_e116540 * assign76870_e116540))),)
    } else {
        (locals.var_tc, locals.var_tc_dn0, locals.var_tc_dn2, locals.var_tc_dn4, locals.var_tc_dn5, locals.var_tc_dn6, locals.var_tc_dn7, locals.var_tc_dn8, locals.var_tc_dn9, locals.var_tc_dn10, locals.var_tc_dn11, locals.var_tc_dn14,)
    }
};
        locals.var_tc = assign76870_e116544;
        locals.var_tc_dn0 = assign76870_e116544_d_n0;
        locals.var_tc_dn2 = assign76870_e116544_d_n2;
        locals.var_tc_dn4 = assign76870_e116544_d_n4;
        locals.var_tc_dn5 = assign76870_e116544_d_n5;
        locals.var_tc_dn6 = assign76870_e116544_d_n6;
        locals.var_tc_dn7 = assign76870_e116544_d_n7;
        locals.var_tc_dn8 = assign76870_e116544_d_n8;
        locals.var_tc_dn9 = assign76870_e116544_d_n9;
        locals.var_tc_dn10 = assign76870_e116544_d_n10;
        locals.var_tc_dn11 = assign76870_e116544_d_n11;
        locals.var_tc_dn14 = assign76870_e116544_d_n14;
        locals.var_tc_rv = 0.0;

        let (assign76880_e116559, assign76880_e116559_d_n0, assign76880_e116559_d_n2, assign76880_e116559_d_n4, assign76880_e116559_d_n5, assign76880_e116559_d_n6, assign76880_e116559_d_n7, assign76880_e116559_d_n8, assign76880_e116559_d_n9, assign76880_e116559_d_n10, assign76880_e116559_d_n11, assign76880_e116559_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1788 == 0.0)) && (locals.var_guard1791 == 0.0)) {
        let assign76880_e116554: f64 = (locals.var_vgpld + locals.var_vxbgmtcl);
        let assign76880_e116555: f64 = (-assign76880_e116554);
        let assign76880_e116557: f64 = (assign76880_e116555 / locals.var_fac1);
        (assign76880_e116557, ((((-locals.var_vxbgmtcl_dn0) * locals.var_fac1) - (assign76880_e116555 * locals.var_fac1_dn0)) / (locals.var_fac1 * locals.var_fac1)), ((((-(locals.var_vgpld_dn2 + locals.var_vxbgmtcl_dn2)) * locals.var_fac1) - (assign76880_e116555 * locals.var_fac1_dn2)) / (locals.var_fac1 * locals.var_fac1)), ((((-locals.var_vxbgmtcl_dn4) * locals.var_fac1) - (assign76880_e116555 * locals.var_fac1_dn4)) / (locals.var_fac1 * locals.var_fac1)), ((((-locals.var_vxbgmtcl_dn5) * locals.var_fac1) - (assign76880_e116555 * locals.var_fac1_dn5)) / (locals.var_fac1 * locals.var_fac1)), ((((-locals.var_vxbgmtcl_dn6) * locals.var_fac1) - (assign76880_e116555 * locals.var_fac1_dn6)) / (locals.var_fac1 * locals.var_fac1)), ((((-(locals.var_vgpld_dn7 + locals.var_vxbgmtcl_dn7)) * locals.var_fac1) - (assign76880_e116555 * locals.var_fac1_dn7)) / (locals.var_fac1 * locals.var_fac1)), ((((-(locals.var_vgpld_dn8 + locals.var_vxbgmtcl_dn8)) * locals.var_fac1) - (assign76880_e116555 * locals.var_fac1_dn8)) / (locals.var_fac1 * locals.var_fac1)), ((((-(locals.var_vgpld_dn9 + locals.var_vxbgmtcl_dn9)) * locals.var_fac1) - (assign76880_e116555 * locals.var_fac1_dn9)) / (locals.var_fac1 * locals.var_fac1)), ((((-locals.var_vxbgmtcl_dn10) * locals.var_fac1) - (assign76880_e116555 * locals.var_fac1_dn10)) / (locals.var_fac1 * locals.var_fac1)), ((((-locals.var_vxbgmtcl_dn11) * locals.var_fac1) - (assign76880_e116555 * locals.var_fac1_dn11)) / (locals.var_fac1 * locals.var_fac1)), ((((-locals.var_vxbgmtcl_dn14) * locals.var_fac1) - (assign76880_e116555 * locals.var_fac1_dn14)) / (locals.var_fac1 * locals.var_fac1)),)
    } else {
        (locals.var_td, locals.var_td_dn0, locals.var_td_dn2, locals.var_td_dn4, locals.var_td_dn5, locals.var_td_dn6, locals.var_td_dn7, locals.var_td_dn8, locals.var_td_dn9, locals.var_td_dn10, locals.var_td_dn11, locals.var_td_dn14,)
    }
};
        locals.var_td = assign76880_e116559;
        locals.var_td_dn0 = assign76880_e116559_d_n0;
        locals.var_td_dn2 = assign76880_e116559_d_n2;
        locals.var_td_dn4 = assign76880_e116559_d_n4;
        locals.var_td_dn5 = assign76880_e116559_d_n5;
        locals.var_td_dn6 = assign76880_e116559_d_n6;
        locals.var_td_dn7 = assign76880_e116559_d_n7;
        locals.var_td_dn8 = assign76880_e116559_d_n8;
        locals.var_td_dn9 = assign76880_e116559_d_n9;
        locals.var_td_dn10 = assign76880_e116559_d_n10;
        locals.var_td_dn11 = assign76880_e116559_d_n11;
        locals.var_td_dn14 = assign76880_e116559_d_n14;
        locals.var_td_rv = 0.0;

        let (assign76890_e116597, assign76890_e116597_d_n0, assign76890_e116597_d_n2, assign76890_e116597_d_n4, assign76890_e116597_d_n5, assign76890_e116597_d_n6, assign76890_e116597_d_n7, assign76890_e116597_d_n8, assign76890_e116597_d_n9, assign76890_e116597_d_n10, assign76890_e116597_d_n11, assign76890_e116597_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1788 == 0.0)) && (locals.var_guard1791 == 0.0)) {
        let assign76890_e116569: f64 = (locals.var_tb * locals.var_tb);
        let assign76890_e116571: f64 = (assign76890_e116569 * locals.var_tb);
        let assign76890_e116574: f64 = (27.0 * locals.var_ta);
        let assign76890_e116576: f64 = (assign76890_e116574 * locals.var_ta);
        let assign76890_e116578: f64 = (assign76890_e116576 * locals.var_ta);
        let assign76890_e116579: f64 = (assign76890_e116571 / assign76890_e116578);
        let assign76890_e116582: f64 = (locals.var_tb * locals.var_tc);
        let assign76890_e116585: f64 = (6.0 * locals.var_ta);
        let assign76890_e116587: f64 = (assign76890_e116585 * locals.var_ta);
        let assign76890_e116588: f64 = (assign76890_e116582 / assign76890_e116587);
        let assign76890_e116589: f64 = (assign76890_e116579 - assign76890_e116588);
        let assign76890_e116593: f64 = (2.0 * locals.var_ta);
        let assign76890_e116594: f64 = (locals.var_td / assign76890_e116593);
        let assign76890_e116595: f64 = (assign76890_e116589 + assign76890_e116594);
        (assign76890_e116595, ((-((locals.var_tb * locals.var_tc_dn0) / assign76890_e116587)) + (locals.var_td_dn0 / assign76890_e116593)), ((-((locals.var_tb * locals.var_tc_dn2) / assign76890_e116587)) + (locals.var_td_dn2 / assign76890_e116593)), ((-((locals.var_tb * locals.var_tc_dn4) / assign76890_e116587)) + (locals.var_td_dn4 / assign76890_e116593)), ((-((locals.var_tb * locals.var_tc_dn5) / assign76890_e116587)) + (locals.var_td_dn5 / assign76890_e116593)), ((-((locals.var_tb * locals.var_tc_dn6) / assign76890_e116587)) + (locals.var_td_dn6 / assign76890_e116593)), ((-((locals.var_tb * locals.var_tc_dn7) / assign76890_e116587)) + (locals.var_td_dn7 / assign76890_e116593)), ((-((locals.var_tb * locals.var_tc_dn8) / assign76890_e116587)) + (locals.var_td_dn8 / assign76890_e116593)), ((-((locals.var_tb * locals.var_tc_dn9) / assign76890_e116587)) + (locals.var_td_dn9 / assign76890_e116593)), ((-((locals.var_tb * locals.var_tc_dn10) / assign76890_e116587)) + (locals.var_td_dn10 / assign76890_e116593)), ((-((locals.var_tb * locals.var_tc_dn11) / assign76890_e116587)) + (locals.var_td_dn11 / assign76890_e116593)), ((-((locals.var_tb * locals.var_tc_dn14) / assign76890_e116587)) + (locals.var_td_dn14 / assign76890_e116593)),)
    } else {
        (locals.var_tq, locals.var_tq_dn0, locals.var_tq_dn2, locals.var_tq_dn4, locals.var_tq_dn5, locals.var_tq_dn6, locals.var_tq_dn7, locals.var_tq_dn8, locals.var_tq_dn9, locals.var_tq_dn10, locals.var_tq_dn11, locals.var_tq_dn14,)
    }
};
        locals.var_tq = assign76890_e116597;
        locals.var_tq_dn0 = assign76890_e116597_d_n0;
        locals.var_tq_dn2 = assign76890_e116597_d_n2;
        locals.var_tq_dn4 = assign76890_e116597_d_n4;
        locals.var_tq_dn5 = assign76890_e116597_d_n5;
        locals.var_tq_dn6 = assign76890_e116597_d_n6;
        locals.var_tq_dn7 = assign76890_e116597_d_n7;
        locals.var_tq_dn8 = assign76890_e116597_d_n8;
        locals.var_tq_dn9 = assign76890_e116597_d_n9;
        locals.var_tq_dn10 = assign76890_e116597_d_n10;
        locals.var_tq_dn11 = assign76890_e116597_d_n11;
        locals.var_tq_dn14 = assign76890_e116597_d_n14;
        locals.var_tq_rv = 0.0;

        let (assign76900_e116621, assign76900_e116621_d_n0, assign76900_e116621_d_n2, assign76900_e116621_d_n4, assign76900_e116621_d_n5, assign76900_e116621_d_n6, assign76900_e116621_d_n7, assign76900_e116621_d_n8, assign76900_e116621_d_n9, assign76900_e116621_d_n10, assign76900_e116621_d_n11, assign76900_e116621_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1788 == 0.0)) && (locals.var_guard1791 == 0.0)) {
        let assign76900_e116607: f64 = (3.0 * locals.var_ta);
        let assign76900_e116609: f64 = (assign76900_e116607 * locals.var_tc);
        let assign76900_e116612: f64 = (locals.var_tb * locals.var_tb);
        let assign76900_e116613: f64 = (assign76900_e116609 - assign76900_e116612);
        let assign76900_e116616: f64 = (9.0 * locals.var_ta);
        let assign76900_e116618: f64 = (assign76900_e116616 * locals.var_ta);
        let assign76900_e116619: f64 = (assign76900_e116613 / assign76900_e116618);
        (assign76900_e116619, ((assign76900_e116607 * locals.var_tc_dn0) / assign76900_e116618), ((assign76900_e116607 * locals.var_tc_dn2) / assign76900_e116618), ((assign76900_e116607 * locals.var_tc_dn4) / assign76900_e116618), ((assign76900_e116607 * locals.var_tc_dn5) / assign76900_e116618), ((assign76900_e116607 * locals.var_tc_dn6) / assign76900_e116618), ((assign76900_e116607 * locals.var_tc_dn7) / assign76900_e116618), ((assign76900_e116607 * locals.var_tc_dn8) / assign76900_e116618), ((assign76900_e116607 * locals.var_tc_dn9) / assign76900_e116618), ((assign76900_e116607 * locals.var_tc_dn10) / assign76900_e116618), ((assign76900_e116607 * locals.var_tc_dn11) / assign76900_e116618), ((assign76900_e116607 * locals.var_tc_dn14) / assign76900_e116618),)
    } else {
        (locals.var_tp, locals.var_tp_dn0, locals.var_tp_dn2, locals.var_tp_dn4, locals.var_tp_dn5, locals.var_tp_dn6, locals.var_tp_dn7, locals.var_tp_dn8, locals.var_tp_dn9, locals.var_tp_dn10, locals.var_tp_dn11, locals.var_tp_dn14,)
    }
};
        locals.var_tp = assign76900_e116621;
        locals.var_tp_dn0 = assign76900_e116621_d_n0;
        locals.var_tp_dn2 = assign76900_e116621_d_n2;
        locals.var_tp_dn4 = assign76900_e116621_d_n4;
        locals.var_tp_dn5 = assign76900_e116621_d_n5;
        locals.var_tp_dn6 = assign76900_e116621_d_n6;
        locals.var_tp_dn7 = assign76900_e116621_d_n7;
        locals.var_tp_dn8 = assign76900_e116621_d_n8;
        locals.var_tp_dn9 = assign76900_e116621_d_n9;
        locals.var_tp_dn10 = assign76900_e116621_d_n10;
        locals.var_tp_dn11 = assign76900_e116621_d_n11;
        locals.var_tp_dn14 = assign76900_e116621_d_n14;
        locals.var_tp_rv = 0.0;

        let (assign76910_e116640, assign76910_e116640_d_n0, assign76910_e116640_d_n2, assign76910_e116640_d_n4, assign76910_e116640_d_n5, assign76910_e116640_d_n6, assign76910_e116640_d_n7, assign76910_e116640_d_n8, assign76910_e116640_d_n9, assign76910_e116640_d_n10, assign76910_e116640_d_n11, assign76910_e116640_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1788 == 0.0)) && (locals.var_guard1791 == 0.0)) {
        let assign76910_e116631: f64 = (locals.var_tq * locals.var_tq);
        let assign76910_e116634: f64 = (locals.var_tp * locals.var_tp);
        let assign76910_e116636: f64 = (assign76910_e116634 * locals.var_tp);
        let assign76910_e116637: f64 = (assign76910_e116631 + assign76910_e116636);
        let assign76910_e116638: f64 = (assign76910_e116637).sqrt();
        (assign76910_e116638, ((((locals.var_tq_dn0 * locals.var_tq) + (locals.var_tq * locals.var_tq_dn0)) + ((((locals.var_tp_dn0 * locals.var_tp) + (locals.var_tp * locals.var_tp_dn0)) * locals.var_tp) + (assign76910_e116634 * locals.var_tp_dn0))) / (2.0 * assign76910_e116638)), ((((locals.var_tq_dn2 * locals.var_tq) + (locals.var_tq * locals.var_tq_dn2)) + ((((locals.var_tp_dn2 * locals.var_tp) + (locals.var_tp * locals.var_tp_dn2)) * locals.var_tp) + (assign76910_e116634 * locals.var_tp_dn2))) / (2.0 * assign76910_e116638)), ((((locals.var_tq_dn4 * locals.var_tq) + (locals.var_tq * locals.var_tq_dn4)) + ((((locals.var_tp_dn4 * locals.var_tp) + (locals.var_tp * locals.var_tp_dn4)) * locals.var_tp) + (assign76910_e116634 * locals.var_tp_dn4))) / (2.0 * assign76910_e116638)), ((((locals.var_tq_dn5 * locals.var_tq) + (locals.var_tq * locals.var_tq_dn5)) + ((((locals.var_tp_dn5 * locals.var_tp) + (locals.var_tp * locals.var_tp_dn5)) * locals.var_tp) + (assign76910_e116634 * locals.var_tp_dn5))) / (2.0 * assign76910_e116638)), ((((locals.var_tq_dn6 * locals.var_tq) + (locals.var_tq * locals.var_tq_dn6)) + ((((locals.var_tp_dn6 * locals.var_tp) + (locals.var_tp * locals.var_tp_dn6)) * locals.var_tp) + (assign76910_e116634 * locals.var_tp_dn6))) / (2.0 * assign76910_e116638)), ((((locals.var_tq_dn7 * locals.var_tq) + (locals.var_tq * locals.var_tq_dn7)) + ((((locals.var_tp_dn7 * locals.var_tp) + (locals.var_tp * locals.var_tp_dn7)) * locals.var_tp) + (assign76910_e116634 * locals.var_tp_dn7))) / (2.0 * assign76910_e116638)), ((((locals.var_tq_dn8 * locals.var_tq) + (locals.var_tq * locals.var_tq_dn8)) + ((((locals.var_tp_dn8 * locals.var_tp) + (locals.var_tp * locals.var_tp_dn8)) * locals.var_tp) + (assign76910_e116634 * locals.var_tp_dn8))) / (2.0 * assign76910_e116638)), ((((locals.var_tq_dn9 * locals.var_tq) + (locals.var_tq * locals.var_tq_dn9)) + ((((locals.var_tp_dn9 * locals.var_tp) + (locals.var_tp * locals.var_tp_dn9)) * locals.var_tp) + (assign76910_e116634 * locals.var_tp_dn9))) / (2.0 * assign76910_e116638)), ((((locals.var_tq_dn10 * locals.var_tq) + (locals.var_tq * locals.var_tq_dn10)) + ((((locals.var_tp_dn10 * locals.var_tp) + (locals.var_tp * locals.var_tp_dn10)) * locals.var_tp) + (assign76910_e116634 * locals.var_tp_dn10))) / (2.0 * assign76910_e116638)), ((((locals.var_tq_dn11 * locals.var_tq) + (locals.var_tq * locals.var_tq_dn11)) + ((((locals.var_tp_dn11 * locals.var_tp) + (locals.var_tp * locals.var_tp_dn11)) * locals.var_tp) + (assign76910_e116634 * locals.var_tp_dn11))) / (2.0 * assign76910_e116638)), ((((locals.var_tq_dn14 * locals.var_tq) + (locals.var_tq * locals.var_tq_dn14)) + ((((locals.var_tp_dn14 * locals.var_tp) + (locals.var_tp * locals.var_tp_dn14)) * locals.var_tp) + (assign76910_e116634 * locals.var_tp_dn14))) / (2.0 * assign76910_e116638)),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign76910_e116640;
        locals.var_t5_dn0 = assign76910_e116640_d_n0;
        locals.var_t5_dn2 = assign76910_e116640_d_n2;
        locals.var_t5_dn4 = assign76910_e116640_d_n4;
        locals.var_t5_dn5 = assign76910_e116640_d_n5;
        locals.var_t5_dn6 = assign76910_e116640_d_n6;
        locals.var_t5_dn7 = assign76910_e116640_d_n7;
        locals.var_t5_dn8 = assign76910_e116640_d_n8;
        locals.var_t5_dn9 = assign76910_e116640_d_n9;
        locals.var_t5_dn10 = assign76910_e116640_d_n10;
        locals.var_t5_dn11 = assign76910_e116640_d_n11;
        locals.var_t5_dn14 = assign76910_e116640_d_n14;
        locals.var_t5_rv = 0.0;

        let (assign76920_e116655, assign76920_e116655_d_n0, assign76920_e116655_d_n2, assign76920_e116655_d_n4, assign76920_e116655_d_n5, assign76920_e116655_d_n6, assign76920_e116655_d_n7, assign76920_e116655_d_n8, assign76920_e116655_d_n9, assign76920_e116655_d_n10, assign76920_e116655_d_n11, assign76920_e116655_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1788 == 0.0)) && (locals.var_guard1791 == 0.0)) {
        let assign76920_e116649: f64 = (-locals.var_tq);
        let assign76920_e116651: f64 = (assign76920_e116649 + locals.var_t5);
        let assign76920_e116653: f64 = (assign76920_e116651).powf(0.3333333333333333);
        (assign76920_e116653, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign76920_e116651).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq_dn0) + locals.var_t5_dn0))) } } else { (assign76920_e116653 * (0.3333333333333333 * (((-locals.var_tq_dn0) + locals.var_t5_dn0) / assign76920_e116651))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign76920_e116651).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq_dn2) + locals.var_t5_dn2))) } } else { (assign76920_e116653 * (0.3333333333333333 * (((-locals.var_tq_dn2) + locals.var_t5_dn2) / assign76920_e116651))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign76920_e116651).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq_dn4) + locals.var_t5_dn4))) } } else { (assign76920_e116653 * (0.3333333333333333 * (((-locals.var_tq_dn4) + locals.var_t5_dn4) / assign76920_e116651))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign76920_e116651).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq_dn5) + locals.var_t5_dn5))) } } else { (assign76920_e116653 * (0.3333333333333333 * (((-locals.var_tq_dn5) + locals.var_t5_dn5) / assign76920_e116651))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign76920_e116651).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq_dn6) + locals.var_t5_dn6))) } } else { (assign76920_e116653 * (0.3333333333333333 * (((-locals.var_tq_dn6) + locals.var_t5_dn6) / assign76920_e116651))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign76920_e116651).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq_dn7) + locals.var_t5_dn7))) } } else { (assign76920_e116653 * (0.3333333333333333 * (((-locals.var_tq_dn7) + locals.var_t5_dn7) / assign76920_e116651))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign76920_e116651).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq_dn8) + locals.var_t5_dn8))) } } else { (assign76920_e116653 * (0.3333333333333333 * (((-locals.var_tq_dn8) + locals.var_t5_dn8) / assign76920_e116651))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign76920_e116651).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq_dn9) + locals.var_t5_dn9))) } } else { (assign76920_e116653 * (0.3333333333333333 * (((-locals.var_tq_dn9) + locals.var_t5_dn9) / assign76920_e116651))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign76920_e116651).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq_dn10) + locals.var_t5_dn10))) } } else { (assign76920_e116653 * (0.3333333333333333 * (((-locals.var_tq_dn10) + locals.var_t5_dn10) / assign76920_e116651))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign76920_e116651).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq_dn11) + locals.var_t5_dn11))) } } else { (assign76920_e116653 * (0.3333333333333333 * (((-locals.var_tq_dn11) + locals.var_t5_dn11) / assign76920_e116651))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign76920_e116651).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq_dn14) + locals.var_t5_dn14))) } } else { (assign76920_e116653 * (0.3333333333333333 * (((-locals.var_tq_dn14) + locals.var_t5_dn14) / assign76920_e116651))) },)
    } else {
        (locals.var_tu, locals.var_tu_dn0, locals.var_tu_dn2, locals.var_tu_dn4, locals.var_tu_dn5, locals.var_tu_dn6, locals.var_tu_dn7, locals.var_tu_dn8, locals.var_tu_dn9, locals.var_tu_dn10, locals.var_tu_dn11, locals.var_tu_dn14,)
    }
};
        locals.var_tu = assign76920_e116655;
        locals.var_tu_dn0 = assign76920_e116655_d_n0;
        locals.var_tu_dn2 = assign76920_e116655_d_n2;
        locals.var_tu_dn4 = assign76920_e116655_d_n4;
        locals.var_tu_dn5 = assign76920_e116655_d_n5;
        locals.var_tu_dn6 = assign76920_e116655_d_n6;
        locals.var_tu_dn7 = assign76920_e116655_d_n7;
        locals.var_tu_dn8 = assign76920_e116655_d_n8;
        locals.var_tu_dn9 = assign76920_e116655_d_n9;
        locals.var_tu_dn10 = assign76920_e116655_d_n10;
        locals.var_tu_dn11 = assign76920_e116655_d_n11;
        locals.var_tu_dn14 = assign76920_e116655_d_n14;
        locals.var_tu_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_291(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign76930_e116670, assign76930_e116670_d_n0, assign76930_e116670_d_n2, assign76930_e116670_d_n4, assign76930_e116670_d_n5, assign76930_e116670_d_n6, assign76930_e116670_d_n7, assign76930_e116670_d_n8, assign76930_e116670_d_n9, assign76930_e116670_d_n10, assign76930_e116670_d_n11, assign76930_e116670_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1788 == 0.0)) && (locals.var_guard1791 == 0.0)) {
        let assign76930_e116665: f64 = (locals.var_tq + locals.var_t5);
        let assign76930_e116667: f64 = (assign76930_e116665).powf(0.3333333333333333);
        let assign76930_e116668: f64 = (-assign76930_e116667);
        (assign76930_e116668, (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign76930_e116665).powf(0.3333333333333333 - 1.0) * (locals.var_tq_dn0 + locals.var_t5_dn0))) } } else { (assign76930_e116667 * (0.3333333333333333 * ((locals.var_tq_dn0 + locals.var_t5_dn0) / assign76930_e116665))) }), (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign76930_e116665).powf(0.3333333333333333 - 1.0) * (locals.var_tq_dn2 + locals.var_t5_dn2))) } } else { (assign76930_e116667 * (0.3333333333333333 * ((locals.var_tq_dn2 + locals.var_t5_dn2) / assign76930_e116665))) }), (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign76930_e116665).powf(0.3333333333333333 - 1.0) * (locals.var_tq_dn4 + locals.var_t5_dn4))) } } else { (assign76930_e116667 * (0.3333333333333333 * ((locals.var_tq_dn4 + locals.var_t5_dn4) / assign76930_e116665))) }), (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign76930_e116665).powf(0.3333333333333333 - 1.0) * (locals.var_tq_dn5 + locals.var_t5_dn5))) } } else { (assign76930_e116667 * (0.3333333333333333 * ((locals.var_tq_dn5 + locals.var_t5_dn5) / assign76930_e116665))) }), (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign76930_e116665).powf(0.3333333333333333 - 1.0) * (locals.var_tq_dn6 + locals.var_t5_dn6))) } } else { (assign76930_e116667 * (0.3333333333333333 * ((locals.var_tq_dn6 + locals.var_t5_dn6) / assign76930_e116665))) }), (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign76930_e116665).powf(0.3333333333333333 - 1.0) * (locals.var_tq_dn7 + locals.var_t5_dn7))) } } else { (assign76930_e116667 * (0.3333333333333333 * ((locals.var_tq_dn7 + locals.var_t5_dn7) / assign76930_e116665))) }), (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign76930_e116665).powf(0.3333333333333333 - 1.0) * (locals.var_tq_dn8 + locals.var_t5_dn8))) } } else { (assign76930_e116667 * (0.3333333333333333 * ((locals.var_tq_dn8 + locals.var_t5_dn8) / assign76930_e116665))) }), (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign76930_e116665).powf(0.3333333333333333 - 1.0) * (locals.var_tq_dn9 + locals.var_t5_dn9))) } } else { (assign76930_e116667 * (0.3333333333333333 * ((locals.var_tq_dn9 + locals.var_t5_dn9) / assign76930_e116665))) }), (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign76930_e116665).powf(0.3333333333333333 - 1.0) * (locals.var_tq_dn10 + locals.var_t5_dn10))) } } else { (assign76930_e116667 * (0.3333333333333333 * ((locals.var_tq_dn10 + locals.var_t5_dn10) / assign76930_e116665))) }), (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign76930_e116665).powf(0.3333333333333333 - 1.0) * (locals.var_tq_dn11 + locals.var_t5_dn11))) } } else { (assign76930_e116667 * (0.3333333333333333 * ((locals.var_tq_dn11 + locals.var_t5_dn11) / assign76930_e116665))) }), (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign76930_e116665).powf(0.3333333333333333 - 1.0) * (locals.var_tq_dn14 + locals.var_t5_dn14))) } } else { (assign76930_e116667 * (0.3333333333333333 * ((locals.var_tq_dn14 + locals.var_t5_dn14) / assign76930_e116665))) }),)
    } else {
        (locals.var_tv, locals.var_tv_dn0, locals.var_tv_dn2, locals.var_tv_dn4, locals.var_tv_dn5, locals.var_tv_dn6, locals.var_tv_dn7, locals.var_tv_dn8, locals.var_tv_dn9, locals.var_tv_dn10, locals.var_tv_dn11, locals.var_tv_dn14,)
    }
};
        locals.var_tv = assign76930_e116670;
        locals.var_tv_dn0 = assign76930_e116670_d_n0;
        locals.var_tv_dn2 = assign76930_e116670_d_n2;
        locals.var_tv_dn4 = assign76930_e116670_d_n4;
        locals.var_tv_dn5 = assign76930_e116670_d_n5;
        locals.var_tv_dn6 = assign76930_e116670_d_n6;
        locals.var_tv_dn7 = assign76930_e116670_d_n7;
        locals.var_tv_dn8 = assign76930_e116670_d_n8;
        locals.var_tv_dn9 = assign76930_e116670_d_n9;
        locals.var_tv_dn10 = assign76930_e116670_d_n10;
        locals.var_tv_dn11 = assign76930_e116670_d_n11;
        locals.var_tv_dn14 = assign76930_e116670_d_n14;
        locals.var_tv_rv = 0.0;

        let (assign76940_e116688, assign76940_e116688_d_n0, assign76940_e116688_d_n2, assign76940_e116688_d_n4, assign76940_e116688_d_n5, assign76940_e116688_d_n6, assign76940_e116688_d_n7, assign76940_e116688_d_n8, assign76940_e116688_d_n9, assign76940_e116688_d_n10, assign76940_e116688_d_n11, assign76940_e116688_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1788 == 0.0)) && (locals.var_guard1791 == 0.0)) {
        let assign76940_e116680: f64 = (locals.var_tu + locals.var_tv);
        let assign76940_e116684: f64 = (3.0 * locals.var_ta);
        let assign76940_e116685: f64 = (locals.var_tb / assign76940_e116684);
        let assign76940_e116686: f64 = (assign76940_e116680 - assign76940_e116685);
        (assign76940_e116686, (locals.var_tu_dn0 + locals.var_tv_dn0), (locals.var_tu_dn2 + locals.var_tv_dn2), (locals.var_tu_dn4 + locals.var_tv_dn4), (locals.var_tu_dn5 + locals.var_tv_dn5), (locals.var_tu_dn6 + locals.var_tv_dn6), (locals.var_tu_dn7 + locals.var_tv_dn7), (locals.var_tu_dn8 + locals.var_tv_dn8), (locals.var_tu_dn9 + locals.var_tv_dn9), (locals.var_tu_dn10 + locals.var_tv_dn10), (locals.var_tu_dn11 + locals.var_tv_dn11), (locals.var_tu_dn14 + locals.var_tv_dn14),)
    } else {
        (locals.var_chi, locals.var_chi_dn0, locals.var_chi_dn2, locals.var_chi_dn4, locals.var_chi_dn5, locals.var_chi_dn6, locals.var_chi_dn7, locals.var_chi_dn8, locals.var_chi_dn9, locals.var_chi_dn10, locals.var_chi_dn11, locals.var_chi_dn14,)
    }
};
        locals.var_chi = assign76940_e116688;
        locals.var_chi_dn0 = assign76940_e116688_d_n0;
        locals.var_chi_dn2 = assign76940_e116688_d_n2;
        locals.var_chi_dn4 = assign76940_e116688_d_n4;
        locals.var_chi_dn5 = assign76940_e116688_d_n5;
        locals.var_chi_dn6 = assign76940_e116688_d_n6;
        locals.var_chi_dn7 = assign76940_e116688_d_n7;
        locals.var_chi_dn8 = assign76940_e116688_d_n8;
        locals.var_chi_dn9 = assign76940_e116688_d_n9;
        locals.var_chi_dn10 = assign76940_e116688_d_n10;
        locals.var_chi_dn11 = assign76940_e116688_d_n11;
        locals.var_chi_dn14 = assign76940_e116688_d_n14;
        locals.var_chi_rv = 0.0;

        let (assign76950_e116702, assign76950_e116702_d_n0, assign76950_e116702_d_n2, assign76950_e116702_d_n4, assign76950_e116702_d_n5, assign76950_e116702_d_n6, assign76950_e116702_d_n7, assign76950_e116702_d_n8, assign76950_e116702_d_n9, assign76950_e116702_d_n10, assign76950_e116702_d_n11, assign76950_e116702_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1788 == 0.0)) && (locals.var_guard1791 == 0.0)) {
        let assign76950_e116698: f64 = (locals.var_chi * locals.var_beta_inv);
        let assign76950_e116700: f64 = (assign76950_e116698 - locals.var_vxbgmtcl);
        (assign76950_e116700, (((locals.var_chi_dn0 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn0)) - locals.var_vxbgmtcl_dn0), (((locals.var_chi_dn2 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn2)) - locals.var_vxbgmtcl_dn2), (((locals.var_chi_dn4 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn4)) - locals.var_vxbgmtcl_dn4), (((locals.var_chi_dn5 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn5)) - locals.var_vxbgmtcl_dn5), (((locals.var_chi_dn6 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn6)) - locals.var_vxbgmtcl_dn6), (((locals.var_chi_dn7 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn7)) - locals.var_vxbgmtcl_dn7), (((locals.var_chi_dn8 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn8)) - locals.var_vxbgmtcl_dn8), (((locals.var_chi_dn9 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn9)) - locals.var_vxbgmtcl_dn9), (((locals.var_chi_dn10 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn10)) - locals.var_vxbgmtcl_dn10), (((locals.var_chi_dn11 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn11)) - locals.var_vxbgmtcl_dn11), (((locals.var_chi_dn14 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn14)) - locals.var_vxbgmtcl_dn14),)
    } else {
        (locals.var_ps0_inia, locals.var_ps0_inia_dn0, locals.var_ps0_inia_dn2, locals.var_ps0_inia_dn4, locals.var_ps0_inia_dn5, locals.var_ps0_inia_dn6, locals.var_ps0_inia_dn7, locals.var_ps0_inia_dn8, locals.var_ps0_inia_dn9, locals.var_ps0_inia_dn10, locals.var_ps0_inia_dn11, locals.var_ps0_inia_dn14,)
    }
};
        locals.var_ps0_inia = assign76950_e116702;
        locals.var_ps0_inia_dn0 = assign76950_e116702_d_n0;
        locals.var_ps0_inia_dn2 = assign76950_e116702_d_n2;
        locals.var_ps0_inia_dn4 = assign76950_e116702_d_n4;
        locals.var_ps0_inia_dn5 = assign76950_e116702_d_n5;
        locals.var_ps0_inia_dn6 = assign76950_e116702_d_n6;
        locals.var_ps0_inia_dn7 = assign76950_e116702_d_n7;
        locals.var_ps0_inia_dn8 = assign76950_e116702_d_n8;
        locals.var_ps0_inia_dn9 = assign76950_e116702_d_n9;
        locals.var_ps0_inia_dn10 = assign76950_e116702_d_n10;
        locals.var_ps0_inia_dn11 = assign76950_e116702_d_n11;
        locals.var_ps0_inia_dn14 = assign76950_e116702_d_n14;
        locals.var_ps0_inia_rv = 0.0;

        let assign76960_e116705: f64 = if p.p33 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1792 = assign76960_e116705;
        locals.var_guard1792_rv = 0.0;

        let (assign76970_e116718, assign76970_e116718_d_n0, assign76970_e116718_d_n2, assign76970_e116718_d_n4, assign76970_e116718_d_n5, assign76970_e116718_d_n6, assign76970_e116718_d_n7, assign76970_e116718_d_n8, assign76970_e116718_d_n9, assign76970_e116718_d_n10, assign76970_e116718_d_n11, assign76970_e116718_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1788 == 0.0)) && (locals.var_guard1792 != 0.0)) {
        let assign76970_e116714: f64 = (locals.var_vgpld + locals.var_vxbgmtcl);
        let assign76970_e116716: f64 = (assign76970_e116714 + 0.1);
        (assign76970_e116716, locals.var_vxbgmtcl_dn0, (locals.var_vgpld_dn2 + locals.var_vxbgmtcl_dn2), locals.var_vxbgmtcl_dn4, locals.var_vxbgmtcl_dn5, locals.var_vxbgmtcl_dn6, (locals.var_vgpld_dn7 + locals.var_vxbgmtcl_dn7), (locals.var_vgpld_dn8 + locals.var_vxbgmtcl_dn8), (locals.var_vgpld_dn9 + locals.var_vxbgmtcl_dn9), locals.var_vxbgmtcl_dn10, locals.var_vxbgmtcl_dn11, locals.var_vxbgmtcl_dn14,)
    } else {
        (locals.var_vgpld_shift, locals.var_vgpld_shift_dn0, locals.var_vgpld_shift_dn2, locals.var_vgpld_shift_dn4, locals.var_vgpld_shift_dn5, locals.var_vgpld_shift_dn6, locals.var_vgpld_shift_dn7, locals.var_vgpld_shift_dn8, locals.var_vgpld_shift_dn9, locals.var_vgpld_shift_dn10, locals.var_vgpld_shift_dn11, locals.var_vgpld_shift_dn14,)
    }
};
        locals.var_vgpld_shift = assign76970_e116718;
        locals.var_vgpld_shift_dn0 = assign76970_e116718_d_n0;
        locals.var_vgpld_shift_dn2 = assign76970_e116718_d_n2;
        locals.var_vgpld_shift_dn4 = assign76970_e116718_d_n4;
        locals.var_vgpld_shift_dn5 = assign76970_e116718_d_n5;
        locals.var_vgpld_shift_dn6 = assign76970_e116718_d_n6;
        locals.var_vgpld_shift_dn7 = assign76970_e116718_d_n7;
        locals.var_vgpld_shift_dn8 = assign76970_e116718_d_n8;
        locals.var_vgpld_shift_dn9 = assign76970_e116718_d_n9;
        locals.var_vgpld_shift_dn10 = assign76970_e116718_d_n10;
        locals.var_vgpld_shift_dn11 = assign76970_e116718_d_n11;
        locals.var_vgpld_shift_dn14 = assign76970_e116718_d_n14;
        locals.var_vgpld_shift_rv = 0.0;

        let (assign76980_e116729, assign76980_e116729_d_n0, assign76980_e116729_d_n2, assign76980_e116729_d_n4, assign76980_e116729_d_n5, assign76980_e116729_d_n6, assign76980_e116729_d_n7, assign76980_e116729_d_n8, assign76980_e116729_d_n9, assign76980_e116729_d_n10, assign76980_e116729_d_n11, assign76980_e116729_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1788 == 0.0)) && (locals.var_guard1792 != 0.0)) {
        let assign76980_e116727: f64 = (locals.var_cnst1over * locals.var_exp_bvbs);
        (assign76980_e116727, ((locals.var_cnst1over_dn0 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn0)), ((locals.var_cnst1over_dn2 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn2)), ((locals.var_cnst1over_dn4 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn4)), ((locals.var_cnst1over_dn5 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn5)), ((locals.var_cnst1over_dn6 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn6)), ((locals.var_cnst1over_dn7 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn7)), ((locals.var_cnst1over_dn8 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn8)), ((locals.var_cnst1over_dn9 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn9)), ((locals.var_cnst1over_dn10 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn10)), ((locals.var_cnst1over_dn11 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn11)), ((locals.var_cnst1over_dn14 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn14)),)
    } else {
        (locals.var_cfs1, locals.var_cfs1_dn0, locals.var_cfs1_dn2, locals.var_cfs1_dn4, locals.var_cfs1_dn5, locals.var_cfs1_dn6, locals.var_cfs1_dn7, locals.var_cfs1_dn8, locals.var_cfs1_dn9, locals.var_cfs1_dn10, locals.var_cfs1_dn11, locals.var_cfs1_dn14,)
    }
};
        locals.var_cfs1 = assign76980_e116729;
        locals.var_cfs1_dn0 = assign76980_e116729_d_n0;
        locals.var_cfs1_dn2 = assign76980_e116729_d_n2;
        locals.var_cfs1_dn4 = assign76980_e116729_d_n4;
        locals.var_cfs1_dn5 = assign76980_e116729_d_n5;
        locals.var_cfs1_dn6 = assign76980_e116729_d_n6;
        locals.var_cfs1_dn7 = assign76980_e116729_d_n7;
        locals.var_cfs1_dn8 = assign76980_e116729_d_n8;
        locals.var_cfs1_dn9 = assign76980_e116729_d_n9;
        locals.var_cfs1_dn10 = assign76980_e116729_d_n10;
        locals.var_cfs1_dn11 = assign76980_e116729_d_n11;
        locals.var_cfs1_dn14 = assign76980_e116729_d_n14;
        locals.var_cfs1_rv = 0.0;

        let (assign76990_e116740, assign76990_e116740_d_n0, assign76990_e116740_d_n2, assign76990_e116740_d_n4, assign76990_e116740_d_n5, assign76990_e116740_d_n6, assign76990_e116740_d_n7, assign76990_e116740_d_n8, assign76990_e116740_d_n9, assign76990_e116740_d_n10, assign76990_e116740_d_n11, assign76990_e116740_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1788 == 0.0)) && (locals.var_guard1792 != 0.0)) {
        let assign76990_e116738: f64 = (locals.var_cnst1over * locals.var_exp_bvbs);
        (assign76990_e116738, ((locals.var_cnst1over_dn0 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn0)), ((locals.var_cnst1over_dn2 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn2)), ((locals.var_cnst1over_dn4 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn4)), ((locals.var_cnst1over_dn5 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn5)), ((locals.var_cnst1over_dn6 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn6)), ((locals.var_cnst1over_dn7 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn7)), ((locals.var_cnst1over_dn8 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn8)), ((locals.var_cnst1over_dn9 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn9)), ((locals.var_cnst1over_dn10 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn10)), ((locals.var_cnst1over_dn11 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn11)), ((locals.var_cnst1over_dn14 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn14)),)
    } else {
        (locals.var_gammachi, locals.var_gammachi_dn0, locals.var_gammachi_dn2, locals.var_gammachi_dn4, locals.var_gammachi_dn5, locals.var_gammachi_dn6, locals.var_gammachi_dn7, locals.var_gammachi_dn8, locals.var_gammachi_dn9, locals.var_gammachi_dn10, locals.var_gammachi_dn11, locals.var_gammachi_dn14,)
    }
};
        locals.var_gammachi = assign76990_e116740;
        locals.var_gammachi_dn0 = assign76990_e116740_d_n0;
        locals.var_gammachi_dn2 = assign76990_e116740_d_n2;
        locals.var_gammachi_dn4 = assign76990_e116740_d_n4;
        locals.var_gammachi_dn5 = assign76990_e116740_d_n5;
        locals.var_gammachi_dn6 = assign76990_e116740_d_n6;
        locals.var_gammachi_dn7 = assign76990_e116740_d_n7;
        locals.var_gammachi_dn8 = assign76990_e116740_d_n8;
        locals.var_gammachi_dn9 = assign76990_e116740_d_n9;
        locals.var_gammachi_dn10 = assign76990_e116740_d_n10;
        locals.var_gammachi_dn11 = assign76990_e116740_d_n11;
        locals.var_gammachi_dn14 = assign76990_e116740_d_n14;
        locals.var_gammachi_rv = 0.0;

        let (assign77000_e116751, assign77000_e116751_d_n0, assign77000_e116751_d_n2, assign77000_e116751_d_n4, assign77000_e116751_d_n5, assign77000_e116751_d_n6, assign77000_e116751_d_n7, assign77000_e116751_d_n8, assign77000_e116751_d_n9, assign77000_e116751_d_n10, assign77000_e116751_d_n11, assign77000_e116751_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1788 == 0.0)) && (locals.var_guard1792 != 0.0)) {
        let assign77000_e116749: f64 = (locals.var_beta2 * locals.var_fac1p2);
        (assign77000_e116749, ((locals.var_beta2_dn0 * locals.var_fac1p2) + (locals.var_beta2 * locals.var_fac1p2_dn0)), ((locals.var_beta2_dn2 * locals.var_fac1p2) + (locals.var_beta2 * locals.var_fac1p2_dn2)), ((locals.var_beta2_dn4 * locals.var_fac1p2) + (locals.var_beta2 * locals.var_fac1p2_dn4)), ((locals.var_beta2_dn5 * locals.var_fac1p2) + (locals.var_beta2 * locals.var_fac1p2_dn5)), ((locals.var_beta2_dn6 * locals.var_fac1p2) + (locals.var_beta2 * locals.var_fac1p2_dn6)), ((locals.var_beta2_dn7 * locals.var_fac1p2) + (locals.var_beta2 * locals.var_fac1p2_dn7)), ((locals.var_beta2_dn8 * locals.var_fac1p2) + (locals.var_beta2 * locals.var_fac1p2_dn8)), ((locals.var_beta2_dn9 * locals.var_fac1p2) + (locals.var_beta2 * locals.var_fac1p2_dn9)), ((locals.var_beta2_dn10 * locals.var_fac1p2) + (locals.var_beta2 * locals.var_fac1p2_dn10)), ((locals.var_beta2_dn11 * locals.var_fac1p2) + (locals.var_beta2 * locals.var_fac1p2_dn11)), ((locals.var_beta2_dn14 * locals.var_fac1p2) + (locals.var_beta2 * locals.var_fac1p2_dn14)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign77000_e116751;
        locals.var_t0_dn0 = assign77000_e116751_d_n0;
        locals.var_t0_dn2 = assign77000_e116751_d_n2;
        locals.var_t0_dn4 = assign77000_e116751_d_n4;
        locals.var_t0_dn5 = assign77000_e116751_d_n5;
        locals.var_t0_dn6 = assign77000_e116751_d_n6;
        locals.var_t0_dn7 = assign77000_e116751_d_n7;
        locals.var_t0_dn8 = assign77000_e116751_d_n8;
        locals.var_t0_dn9 = assign77000_e116751_d_n9;
        locals.var_t0_dn10 = assign77000_e116751_d_n10;
        locals.var_t0_dn11 = assign77000_e116751_d_n11;
        locals.var_t0_dn14 = assign77000_e116751_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign77010_e116762, assign77010_e116762_d_n0, assign77010_e116762_d_n2, assign77010_e116762_d_n4, assign77010_e116762_d_n5, assign77010_e116762_d_n6, assign77010_e116762_d_n7, assign77010_e116762_d_n8, assign77010_e116762_d_n9, assign77010_e116762_d_n10, assign77010_e116762_d_n11, assign77010_e116762_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1788 == 0.0)) && (locals.var_guard1792 != 0.0)) {
        let assign77010_e116760: f64 = (locals.var_beta * locals.var_vgpld_shift);
        (assign77010_e116760, ((locals.var_beta_dn0 * locals.var_vgpld_shift) + (locals.var_beta * locals.var_vgpld_shift_dn0)), ((locals.var_beta_dn2 * locals.var_vgpld_shift) + (locals.var_beta * locals.var_vgpld_shift_dn2)), ((locals.var_beta_dn4 * locals.var_vgpld_shift) + (locals.var_beta * locals.var_vgpld_shift_dn4)), ((locals.var_beta_dn5 * locals.var_vgpld_shift) + (locals.var_beta * locals.var_vgpld_shift_dn5)), ((locals.var_beta_dn6 * locals.var_vgpld_shift) + (locals.var_beta * locals.var_vgpld_shift_dn6)), ((locals.var_beta_dn7 * locals.var_vgpld_shift) + (locals.var_beta * locals.var_vgpld_shift_dn7)), ((locals.var_beta_dn8 * locals.var_vgpld_shift) + (locals.var_beta * locals.var_vgpld_shift_dn8)), ((locals.var_beta_dn9 * locals.var_vgpld_shift) + (locals.var_beta * locals.var_vgpld_shift_dn9)), ((locals.var_beta_dn10 * locals.var_vgpld_shift) + (locals.var_beta * locals.var_vgpld_shift_dn10)), ((locals.var_beta_dn11 * locals.var_vgpld_shift) + (locals.var_beta * locals.var_vgpld_shift_dn11)), ((locals.var_beta_dn14 * locals.var_vgpld_shift) + (locals.var_beta * locals.var_vgpld_shift_dn14)),)
    } else {
        (locals.var_psi, locals.var_psi_dn0, locals.var_psi_dn2, locals.var_psi_dn4, locals.var_psi_dn5, locals.var_psi_dn6, locals.var_psi_dn7, locals.var_psi_dn8, locals.var_psi_dn9, locals.var_psi_dn10, locals.var_psi_dn11, locals.var_psi_dn14,)
    }
};
        locals.var_psi = assign77010_e116762;
        locals.var_psi_dn0 = assign77010_e116762_d_n0;
        locals.var_psi_dn2 = assign77010_e116762_d_n2;
        locals.var_psi_dn4 = assign77010_e116762_d_n4;
        locals.var_psi_dn5 = assign77010_e116762_d_n5;
        locals.var_psi_dn6 = assign77010_e116762_d_n6;
        locals.var_psi_dn7 = assign77010_e116762_d_n7;
        locals.var_psi_dn8 = assign77010_e116762_d_n8;
        locals.var_psi_dn9 = assign77010_e116762_d_n9;
        locals.var_psi_dn10 = assign77010_e116762_d_n10;
        locals.var_psi_dn11 = assign77010_e116762_d_n11;
        locals.var_psi_dn14 = assign77010_e116762_d_n14;
        locals.var_psi_rv = 0.0;

        let (assign77020_e116787, assign77020_e116787_d_n0, assign77020_e116787_d_n2, assign77020_e116787_d_n4, assign77020_e116787_d_n5, assign77020_e116787_d_n6, assign77020_e116787_d_n7, assign77020_e116787_d_n8, assign77020_e116787_d_n9, assign77020_e116787_d_n10, assign77020_e116787_d_n11, assign77020_e116787_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1788 == 0.0)) && (locals.var_guard1792 != 0.0)) {
        let assign77020_e116771: f64 = (locals.var_gammachi * locals.var_t0);
        let assign77020_e116774: f64 = (locals.var_psi * locals.var_psi);
        let assign77020_e116775: f64 = (assign77020_e116771 + assign77020_e116774);
        let assign77020_e116776: f64 = (assign77020_e116775).ln();
        let assign77020_e116779: f64 = (locals.var_cnst1over * locals.var_t0);
        let assign77020_e116780: f64 = (assign77020_e116779).ln();
        let assign77020_e116781: f64 = (assign77020_e116776 - assign77020_e116780);
        let assign77020_e116784: f64 = (locals.var_beta * locals.var_vxbgmtcl);
        let assign77020_e116785: f64 = (assign77020_e116781 + assign77020_e116784);
        (assign77020_e116785, ((((((locals.var_gammachi_dn0 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn0)) + ((locals.var_psi_dn0 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn0))) / assign77020_e116775) - (((locals.var_cnst1over_dn0 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn0)) / assign77020_e116779)) + ((locals.var_beta_dn0 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn0))), ((((((locals.var_gammachi_dn2 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn2)) + ((locals.var_psi_dn2 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn2))) / assign77020_e116775) - (((locals.var_cnst1over_dn2 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn2)) / assign77020_e116779)) + ((locals.var_beta_dn2 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn2))), ((((((locals.var_gammachi_dn4 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn4)) + ((locals.var_psi_dn4 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn4))) / assign77020_e116775) - (((locals.var_cnst1over_dn4 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn4)) / assign77020_e116779)) + ((locals.var_beta_dn4 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn4))), ((((((locals.var_gammachi_dn5 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn5)) + ((locals.var_psi_dn5 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn5))) / assign77020_e116775) - (((locals.var_cnst1over_dn5 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn5)) / assign77020_e116779)) + ((locals.var_beta_dn5 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn5))), ((((((locals.var_gammachi_dn6 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn6)) + ((locals.var_psi_dn6 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn6))) / assign77020_e116775) - (((locals.var_cnst1over_dn6 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn6)) / assign77020_e116779)) + ((locals.var_beta_dn6 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn6))), ((((((locals.var_gammachi_dn7 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn7)) + ((locals.var_psi_dn7 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn7))) / assign77020_e116775) - (((locals.var_cnst1over_dn7 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn7)) / assign77020_e116779)) + ((locals.var_beta_dn7 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn7))), ((((((locals.var_gammachi_dn8 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn8)) + ((locals.var_psi_dn8 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn8))) / assign77020_e116775) - (((locals.var_cnst1over_dn8 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn8)) / assign77020_e116779)) + ((locals.var_beta_dn8 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn8))), ((((((locals.var_gammachi_dn9 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn9)) + ((locals.var_psi_dn9 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn9))) / assign77020_e116775) - (((locals.var_cnst1over_dn9 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn9)) / assign77020_e116779)) + ((locals.var_beta_dn9 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn9))), ((((((locals.var_gammachi_dn10 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn10)) + ((locals.var_psi_dn10 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn10))) / assign77020_e116775) - (((locals.var_cnst1over_dn10 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn10)) / assign77020_e116779)) + ((locals.var_beta_dn10 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn10))), ((((((locals.var_gammachi_dn11 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn11)) + ((locals.var_psi_dn11 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn11))) / assign77020_e116775) - (((locals.var_cnst1over_dn11 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn11)) / assign77020_e116779)) + ((locals.var_beta_dn11 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn11))), ((((((locals.var_gammachi_dn14 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn14)) + ((locals.var_psi_dn14 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn14))) / assign77020_e116775) - (((locals.var_cnst1over_dn14 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn14)) / assign77020_e116779)) + ((locals.var_beta_dn14 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn14))),)
    } else {
        (locals.var_chi_1, locals.var_chi_1_dn0, locals.var_chi_1_dn2, locals.var_chi_1_dn4, locals.var_chi_1_dn5, locals.var_chi_1_dn6, locals.var_chi_1_dn7, locals.var_chi_1_dn8, locals.var_chi_1_dn9, locals.var_chi_1_dn10, locals.var_chi_1_dn11, locals.var_chi_1_dn14,)
    }
};
        locals.var_chi_1 = assign77020_e116787;
        locals.var_chi_1_dn0 = assign77020_e116787_d_n0;
        locals.var_chi_1_dn2 = assign77020_e116787_d_n2;
        locals.var_chi_1_dn4 = assign77020_e116787_d_n4;
        locals.var_chi_1_dn5 = assign77020_e116787_d_n5;
        locals.var_chi_1_dn6 = assign77020_e116787_d_n6;
        locals.var_chi_1_dn7 = assign77020_e116787_d_n7;
        locals.var_chi_1_dn8 = assign77020_e116787_d_n8;
        locals.var_chi_1_dn9 = assign77020_e116787_d_n9;
        locals.var_chi_1_dn10 = assign77020_e116787_d_n10;
        locals.var_chi_1_dn11 = assign77020_e116787_d_n11;
        locals.var_chi_1_dn14 = assign77020_e116787_d_n14;
        locals.var_chi_1_rv = 0.0;

        let assign77030_e116790: f64 = if p.p33 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard1793 = assign77030_e116790;
        locals.var_guard1793_rv = 0.0;

        let (assign77040_e116805, assign77040_e116805_d_n0, assign77040_e116805_d_n2, assign77040_e116805_d_n4, assign77040_e116805_d_n5, assign77040_e116805_d_n6, assign77040_e116805_d_n7, assign77040_e116805_d_n8, assign77040_e116805_d_n9, assign77040_e116805_d_n10, assign77040_e116805_d_n11, assign77040_e116805_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1788 == 0.0)) && (locals.var_guard1792 != 0.0)) && (locals.var_guard1793 != 0.0)) {
        let assign77040_e116801: f64 = (locals.var_psi - locals.var_chi_1);
        let assign77040_e116803: f64 = (assign77040_e116801 - 1.0);
        (assign77040_e116803, (locals.var_psi_dn0 - locals.var_chi_1_dn0), (locals.var_psi_dn2 - locals.var_chi_1_dn2), (locals.var_psi_dn4 - locals.var_chi_1_dn4), (locals.var_psi_dn5 - locals.var_chi_1_dn5), (locals.var_psi_dn6 - locals.var_chi_1_dn6), (locals.var_psi_dn7 - locals.var_chi_1_dn7), (locals.var_psi_dn8 - locals.var_chi_1_dn8), (locals.var_psi_dn9 - locals.var_chi_1_dn9), (locals.var_psi_dn10 - locals.var_chi_1_dn10), (locals.var_psi_dn11 - locals.var_chi_1_dn11), (locals.var_psi_dn14 - locals.var_chi_1_dn14),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign77040_e116805;
        locals.var_tmf1_dn0 = assign77040_e116805_d_n0;
        locals.var_tmf1_dn2 = assign77040_e116805_d_n2;
        locals.var_tmf1_dn4 = assign77040_e116805_d_n4;
        locals.var_tmf1_dn5 = assign77040_e116805_d_n5;
        locals.var_tmf1_dn6 = assign77040_e116805_d_n6;
        locals.var_tmf1_dn7 = assign77040_e116805_d_n7;
        locals.var_tmf1_dn8 = assign77040_e116805_d_n8;
        locals.var_tmf1_dn9 = assign77040_e116805_d_n9;
        locals.var_tmf1_dn10 = assign77040_e116805_d_n10;
        locals.var_tmf1_dn11 = assign77040_e116805_d_n11;
        locals.var_tmf1_dn14 = assign77040_e116805_d_n14;
        locals.var_tmf1_rv = 0.0;

        let (assign77050_e116820, assign77050_e116820_d_n0, assign77050_e116820_d_n2, assign77050_e116820_d_n4, assign77050_e116820_d_n5, assign77050_e116820_d_n6, assign77050_e116820_d_n7, assign77050_e116820_d_n8, assign77050_e116820_d_n9, assign77050_e116820_d_n10, assign77050_e116820_d_n11, assign77050_e116820_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1788 == 0.0)) && (locals.var_guard1792 != 0.0)) && (locals.var_guard1793 != 0.0)) {
        let assign77050_e116816: f64 = (4.0 * locals.var_psi);
        let assign77050_e116818: f64 = assign77050_e116816;
        (assign77050_e116818, (4.0 * locals.var_psi_dn0), (4.0 * locals.var_psi_dn2), (4.0 * locals.var_psi_dn4), (4.0 * locals.var_psi_dn5), (4.0 * locals.var_psi_dn6), (4.0 * locals.var_psi_dn7), (4.0 * locals.var_psi_dn8), (4.0 * locals.var_psi_dn9), (4.0 * locals.var_psi_dn10), (4.0 * locals.var_psi_dn11), (4.0 * locals.var_psi_dn14),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign77050_e116820;
        locals.var_tmf2_dn0 = assign77050_e116820_d_n0;
        locals.var_tmf2_dn2 = assign77050_e116820_d_n2;
        locals.var_tmf2_dn4 = assign77050_e116820_d_n4;
        locals.var_tmf2_dn5 = assign77050_e116820_d_n5;
        locals.var_tmf2_dn6 = assign77050_e116820_d_n6;
        locals.var_tmf2_dn7 = assign77050_e116820_d_n7;
        locals.var_tmf2_dn8 = assign77050_e116820_d_n8;
        locals.var_tmf2_dn9 = assign77050_e116820_d_n9;
        locals.var_tmf2_dn10 = assign77050_e116820_d_n10;
        locals.var_tmf2_dn11 = assign77050_e116820_d_n11;
        locals.var_tmf2_dn14 = assign77050_e116820_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign77060_e116837, assign77060_e116837_d_n0, assign77060_e116837_d_n2, assign77060_e116837_d_n4, assign77060_e116837_d_n5, assign77060_e116837_d_n6, assign77060_e116837_d_n7, assign77060_e116837_d_n8, assign77060_e116837_d_n9, assign77060_e116837_d_n10, assign77060_e116837_d_n11, assign77060_e116837_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1788 == 0.0)) && (locals.var_guard1792 != 0.0)) && (locals.var_guard1793 != 0.0)) {
        let (assign77060_e116835, assign77060_e116835_d_n0, assign77060_e116835_d_n2, assign77060_e116835_d_n4, assign77060_e116835_d_n5, assign77060_e116835_d_n6, assign77060_e116835_d_n7, assign77060_e116835_d_n8, assign77060_e116835_d_n9, assign77060_e116835_d_n10, assign77060_e116835_d_n11, assign77060_e116835_d_n14,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
            } else {
                let assign77060_e116834: f64 = (-locals.var_tmf2);
                (assign77060_e116834, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn14),)
            }
        };
        (assign77060_e116835, assign77060_e116835_d_n0, assign77060_e116835_d_n2, assign77060_e116835_d_n4, assign77060_e116835_d_n5, assign77060_e116835_d_n6, assign77060_e116835_d_n7, assign77060_e116835_d_n8, assign77060_e116835_d_n9, assign77060_e116835_d_n10, assign77060_e116835_d_n11, assign77060_e116835_d_n14,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign77060_e116837;
        locals.var_tmf2_dn0 = assign77060_e116837_d_n0;
        locals.var_tmf2_dn2 = assign77060_e116837_d_n2;
        locals.var_tmf2_dn4 = assign77060_e116837_d_n4;
        locals.var_tmf2_dn5 = assign77060_e116837_d_n5;
        locals.var_tmf2_dn6 = assign77060_e116837_d_n6;
        locals.var_tmf2_dn7 = assign77060_e116837_d_n7;
        locals.var_tmf2_dn8 = assign77060_e116837_d_n8;
        locals.var_tmf2_dn9 = assign77060_e116837_d_n9;
        locals.var_tmf2_dn10 = assign77060_e116837_d_n10;
        locals.var_tmf2_dn11 = assign77060_e116837_d_n11;
        locals.var_tmf2_dn14 = assign77060_e116837_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign77070_e116853, assign77070_e116853_d_n0, assign77070_e116853_d_n2, assign77070_e116853_d_n4, assign77070_e116853_d_n5, assign77070_e116853_d_n6, assign77070_e116853_d_n7, assign77070_e116853_d_n8, assign77070_e116853_d_n9, assign77070_e116853_d_n10, assign77070_e116853_d_n11, assign77070_e116853_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1788 == 0.0)) && (locals.var_guard1792 != 0.0)) && (locals.var_guard1793 != 0.0)) {
        let assign77070_e116848: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign77070_e116850: f64 = (assign77070_e116848 + locals.var_tmf2);
        let assign77070_e116851: f64 = (assign77070_e116850).sqrt();
        (assign77070_e116851, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign77070_e116851)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign77070_e116851)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign77070_e116851)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign77070_e116851)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign77070_e116851)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign77070_e116851)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign77070_e116851)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign77070_e116851)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign77070_e116851)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign77070_e116851)), ((((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)) + locals.var_tmf2_dn14) / (2.0 * assign77070_e116851)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign77070_e116853;
        locals.var_tmf2_dn0 = assign77070_e116853_d_n0;
        locals.var_tmf2_dn2 = assign77070_e116853_d_n2;
        locals.var_tmf2_dn4 = assign77070_e116853_d_n4;
        locals.var_tmf2_dn5 = assign77070_e116853_d_n5;
        locals.var_tmf2_dn6 = assign77070_e116853_d_n6;
        locals.var_tmf2_dn7 = assign77070_e116853_d_n7;
        locals.var_tmf2_dn8 = assign77070_e116853_d_n8;
        locals.var_tmf2_dn9 = assign77070_e116853_d_n9;
        locals.var_tmf2_dn10 = assign77070_e116853_d_n10;
        locals.var_tmf2_dn11 = assign77070_e116853_d_n11;
        locals.var_tmf2_dn14 = assign77070_e116853_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign77080_e116870, assign77080_e116870_d_n0, assign77080_e116870_d_n2, assign77080_e116870_d_n4, assign77080_e116870_d_n5, assign77080_e116870_d_n6, assign77080_e116870_d_n7, assign77080_e116870_d_n8, assign77080_e116870_d_n9, assign77080_e116870_d_n10, assign77080_e116870_d_n11, assign77080_e116870_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1788 == 0.0)) && (locals.var_guard1792 != 0.0)) && (locals.var_guard1793 != 0.0)) {
        let assign77080_e116866: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign77080_e116867: f64 = (1.0 + assign77080_e116866);
        let assign77080_e116868: f64 = (0.5 * assign77080_e116867);
        (assign77080_e116868, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn11 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn14 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign77080_e116870;
        locals.var_t1_dn0 = assign77080_e116870_d_n0;
        locals.var_t1_dn2 = assign77080_e116870_d_n2;
        locals.var_t1_dn4 = assign77080_e116870_d_n4;
        locals.var_t1_dn5 = assign77080_e116870_d_n5;
        locals.var_t1_dn6 = assign77080_e116870_d_n6;
        locals.var_t1_dn7 = assign77080_e116870_d_n7;
        locals.var_t1_dn8 = assign77080_e116870_d_n8;
        locals.var_t1_dn9 = assign77080_e116870_d_n9;
        locals.var_t1_dn10 = assign77080_e116870_d_n10;
        locals.var_t1_dn11 = assign77080_e116870_d_n11;
        locals.var_t1_dn14 = assign77080_e116870_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign77090_e116887, assign77090_e116887_d_n0, assign77090_e116887_d_n2, assign77090_e116887_d_n4, assign77090_e116887_d_n5, assign77090_e116887_d_n6, assign77090_e116887_d_n7, assign77090_e116887_d_n8, assign77090_e116887_d_n9, assign77090_e116887_d_n10, assign77090_e116887_d_n11, assign77090_e116887_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1788 == 0.0)) && (locals.var_guard1792 != 0.0)) && (locals.var_guard1793 != 0.0)) {
        let assign77090_e116883: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign77090_e116884: f64 = (0.5 * assign77090_e116883);
        let assign77090_e116885: f64 = (locals.var_psi - assign77090_e116884);
        (assign77090_e116885, (locals.var_psi_dn0 - (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (locals.var_psi_dn2 - (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), (locals.var_psi_dn4 - (0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4))), (locals.var_psi_dn5 - (0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5))), (locals.var_psi_dn6 - (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), (locals.var_psi_dn7 - (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))), (locals.var_psi_dn8 - (0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8))), (locals.var_psi_dn9 - (0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9))), (locals.var_psi_dn10 - (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), (locals.var_psi_dn11 - (0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11))), (locals.var_psi_dn14 - (0.5 * (locals.var_tmf1_dn14 + locals.var_tmf2_dn14))),)
    } else {
        (locals.var_chi_1, locals.var_chi_1_dn0, locals.var_chi_1_dn2, locals.var_chi_1_dn4, locals.var_chi_1_dn5, locals.var_chi_1_dn6, locals.var_chi_1_dn7, locals.var_chi_1_dn8, locals.var_chi_1_dn9, locals.var_chi_1_dn10, locals.var_chi_1_dn11, locals.var_chi_1_dn14,)
    }
};
        locals.var_chi_1 = assign77090_e116887;
        locals.var_chi_1_dn0 = assign77090_e116887_d_n0;
        locals.var_chi_1_dn2 = assign77090_e116887_d_n2;
        locals.var_chi_1_dn4 = assign77090_e116887_d_n4;
        locals.var_chi_1_dn5 = assign77090_e116887_d_n5;
        locals.var_chi_1_dn6 = assign77090_e116887_d_n6;
        locals.var_chi_1_dn7 = assign77090_e116887_d_n7;
        locals.var_chi_1_dn8 = assign77090_e116887_d_n8;
        locals.var_chi_1_dn9 = assign77090_e116887_d_n9;
        locals.var_chi_1_dn10 = assign77090_e116887_d_n10;
        locals.var_chi_1_dn11 = assign77090_e116887_d_n11;
        locals.var_chi_1_dn14 = assign77090_e116887_d_n14;
        locals.var_chi_1_rv = 0.0;

        let (assign77100_e116904, assign77100_e116904_d_n0, assign77100_e116904_d_n2, assign77100_e116904_d_n4, assign77100_e116904_d_n5, assign77100_e116904_d_n6, assign77100_e116904_d_n7, assign77100_e116904_d_n8, assign77100_e116904_d_n9, assign77100_e116904_d_n10, assign77100_e116904_d_n11, assign77100_e116904_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1788 == 0.0)) && (locals.var_guard1792 != 0.0)) && (locals.var_guard1793 == 0.0)) {
        let (assign77100_e116902, assign77100_e116902_d_n0, assign77100_e116902_d_n2, assign77100_e116902_d_n4, assign77100_e116902_d_n5, assign77100_e116902_d_n6, assign77100_e116902_d_n7, assign77100_e116902_d_n8, assign77100_e116902_d_n9, assign77100_e116902_d_n10, assign77100_e116902_d_n11, assign77100_e116902_d_n14,) = {
            if (locals.var_chi_1 <= locals.var_psi) {
                (locals.var_chi_1, locals.var_chi_1_dn0, locals.var_chi_1_dn2, locals.var_chi_1_dn4, locals.var_chi_1_dn5, locals.var_chi_1_dn6, locals.var_chi_1_dn7, locals.var_chi_1_dn8, locals.var_chi_1_dn9, locals.var_chi_1_dn10, locals.var_chi_1_dn11, locals.var_chi_1_dn14,)
            } else {
                (locals.var_psi, locals.var_psi_dn0, locals.var_psi_dn2, locals.var_psi_dn4, locals.var_psi_dn5, locals.var_psi_dn6, locals.var_psi_dn7, locals.var_psi_dn8, locals.var_psi_dn9, locals.var_psi_dn10, locals.var_psi_dn11, locals.var_psi_dn14,)
            }
        };
        (assign77100_e116902, assign77100_e116902_d_n0, assign77100_e116902_d_n2, assign77100_e116902_d_n4, assign77100_e116902_d_n5, assign77100_e116902_d_n6, assign77100_e116902_d_n7, assign77100_e116902_d_n8, assign77100_e116902_d_n9, assign77100_e116902_d_n10, assign77100_e116902_d_n11, assign77100_e116902_d_n14,)
    } else {
        (locals.var_chi_1, locals.var_chi_1_dn0, locals.var_chi_1_dn2, locals.var_chi_1_dn4, locals.var_chi_1_dn5, locals.var_chi_1_dn6, locals.var_chi_1_dn7, locals.var_chi_1_dn8, locals.var_chi_1_dn9, locals.var_chi_1_dn10, locals.var_chi_1_dn11, locals.var_chi_1_dn14,)
    }
};
        locals.var_chi_1 = assign77100_e116904;
        locals.var_chi_1_dn0 = assign77100_e116904_d_n0;
        locals.var_chi_1_dn2 = assign77100_e116904_d_n2;
        locals.var_chi_1_dn4 = assign77100_e116904_d_n4;
        locals.var_chi_1_dn5 = assign77100_e116904_d_n5;
        locals.var_chi_1_dn6 = assign77100_e116904_d_n6;
        locals.var_chi_1_dn7 = assign77100_e116904_d_n7;
        locals.var_chi_1_dn8 = assign77100_e116904_d_n8;
        locals.var_chi_1_dn9 = assign77100_e116904_d_n9;
        locals.var_chi_1_dn10 = assign77100_e116904_d_n10;
        locals.var_chi_1_dn11 = assign77100_e116904_d_n11;
        locals.var_chi_1_dn14 = assign77100_e116904_d_n14;
        locals.var_chi_1_rv = 0.0;

        let (assign77110_e116918, assign77110_e116918_d_n0, assign77110_e116918_d_n2, assign77110_e116918_d_n4, assign77110_e116918_d_n5, assign77110_e116918_d_n6, assign77110_e116918_d_n7, assign77110_e116918_d_n8, assign77110_e116918_d_n9, assign77110_e116918_d_n10, assign77110_e116918_d_n11, assign77110_e116918_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1788 == 0.0)) && (locals.var_guard1792 != 0.0)) {
        let (assign77110_e116916, assign77110_e116916_d_n0, assign77110_e116916_d_n2, assign77110_e116916_d_n4, assign77110_e116916_d_n5, assign77110_e116916_d_n6, assign77110_e116916_d_n7, assign77110_e116916_d_n8, assign77110_e116916_d_n9, assign77110_e116916_d_n10, assign77110_e116916_d_n11, assign77110_e116916_d_n14,) = {
            if (locals.var_chi_1 >= 0.0) {
                (locals.var_chi_1, locals.var_chi_1_dn0, locals.var_chi_1_dn2, locals.var_chi_1_dn4, locals.var_chi_1_dn5, locals.var_chi_1_dn6, locals.var_chi_1_dn7, locals.var_chi_1_dn8, locals.var_chi_1_dn9, locals.var_chi_1_dn10, locals.var_chi_1_dn11, locals.var_chi_1_dn14,)
            } else {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        (assign77110_e116916, assign77110_e116916_d_n0, assign77110_e116916_d_n2, assign77110_e116916_d_n4, assign77110_e116916_d_n5, assign77110_e116916_d_n6, assign77110_e116916_d_n7, assign77110_e116916_d_n8, assign77110_e116916_d_n9, assign77110_e116916_d_n10, assign77110_e116916_d_n11, assign77110_e116916_d_n14,)
    } else {
        (locals.var_chi_1, locals.var_chi_1_dn0, locals.var_chi_1_dn2, locals.var_chi_1_dn4, locals.var_chi_1_dn5, locals.var_chi_1_dn6, locals.var_chi_1_dn7, locals.var_chi_1_dn8, locals.var_chi_1_dn9, locals.var_chi_1_dn10, locals.var_chi_1_dn11, locals.var_chi_1_dn14,)
    }
};
        locals.var_chi_1 = assign77110_e116918;
        locals.var_chi_1_dn0 = assign77110_e116918_d_n0;
        locals.var_chi_1_dn2 = assign77110_e116918_d_n2;
        locals.var_chi_1_dn4 = assign77110_e116918_d_n4;
        locals.var_chi_1_dn5 = assign77110_e116918_d_n5;
        locals.var_chi_1_dn6 = assign77110_e116918_d_n6;
        locals.var_chi_1_dn7 = assign77110_e116918_d_n7;
        locals.var_chi_1_dn8 = assign77110_e116918_d_n8;
        locals.var_chi_1_dn9 = assign77110_e116918_d_n9;
        locals.var_chi_1_dn10 = assign77110_e116918_d_n10;
        locals.var_chi_1_dn11 = assign77110_e116918_d_n11;
        locals.var_chi_1_dn14 = assign77110_e116918_d_n14;
        locals.var_chi_1_rv = 0.0;

        let (assign77120_e116929, assign77120_e116929_d_n0, assign77120_e116929_d_n2, assign77120_e116929_d_n4, assign77120_e116929_d_n5, assign77120_e116929_d_n6, assign77120_e116929_d_n7, assign77120_e116929_d_n8, assign77120_e116929_d_n9, assign77120_e116929_d_n10, assign77120_e116929_d_n11, assign77120_e116929_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1788 == 0.0)) && (locals.var_guard1792 != 0.0)) {
        let assign77120_e116927: f64 = (locals.var_psi - locals.var_chi_1);
        (assign77120_e116927, (locals.var_psi_dn0 - locals.var_chi_1_dn0), (locals.var_psi_dn2 - locals.var_chi_1_dn2), (locals.var_psi_dn4 - locals.var_chi_1_dn4), (locals.var_psi_dn5 - locals.var_chi_1_dn5), (locals.var_psi_dn6 - locals.var_chi_1_dn6), (locals.var_psi_dn7 - locals.var_chi_1_dn7), (locals.var_psi_dn8 - locals.var_chi_1_dn8), (locals.var_psi_dn9 - locals.var_chi_1_dn9), (locals.var_psi_dn10 - locals.var_chi_1_dn10), (locals.var_psi_dn11 - locals.var_chi_1_dn11), (locals.var_psi_dn14 - locals.var_chi_1_dn14),)
    } else {
        (locals.var_psi, locals.var_psi_dn0, locals.var_psi_dn2, locals.var_psi_dn4, locals.var_psi_dn5, locals.var_psi_dn6, locals.var_psi_dn7, locals.var_psi_dn8, locals.var_psi_dn9, locals.var_psi_dn10, locals.var_psi_dn11, locals.var_psi_dn14,)
    }
};
        locals.var_psi = assign77120_e116929;
        locals.var_psi_dn0 = assign77120_e116929_d_n0;
        locals.var_psi_dn2 = assign77120_e116929_d_n2;
        locals.var_psi_dn4 = assign77120_e116929_d_n4;
        locals.var_psi_dn5 = assign77120_e116929_d_n5;
        locals.var_psi_dn6 = assign77120_e116929_d_n6;
        locals.var_psi_dn7 = assign77120_e116929_d_n7;
        locals.var_psi_dn8 = assign77120_e116929_d_n8;
        locals.var_psi_dn9 = assign77120_e116929_d_n9;
        locals.var_psi_dn10 = assign77120_e116929_d_n10;
        locals.var_psi_dn11 = assign77120_e116929_d_n11;
        locals.var_psi_dn14 = assign77120_e116929_d_n14;
        locals.var_psi_rv = 0.0;

        let (assign77130_e116942, assign77130_e116942_d_n0, assign77130_e116942_d_n2, assign77130_e116942_d_n4, assign77130_e116942_d_n5, assign77130_e116942_d_n6, assign77130_e116942_d_n7, assign77130_e116942_d_n8, assign77130_e116942_d_n9, assign77130_e116942_d_n10, assign77130_e116942_d_n11, assign77130_e116942_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1788 == 0.0)) && (locals.var_guard1792 != 0.0)) {
        let assign77130_e116939: f64 = (locals.var_beta * 0.1);
        let assign77130_e116940: f64 = (locals.var_psi + assign77130_e116939);
        (assign77130_e116940, (locals.var_psi_dn0 + (locals.var_beta_dn0 * 0.1)), (locals.var_psi_dn2 + (locals.var_beta_dn2 * 0.1)), (locals.var_psi_dn4 + (locals.var_beta_dn4 * 0.1)), (locals.var_psi_dn5 + (locals.var_beta_dn5 * 0.1)), (locals.var_psi_dn6 + (locals.var_beta_dn6 * 0.1)), (locals.var_psi_dn7 + (locals.var_beta_dn7 * 0.1)), (locals.var_psi_dn8 + (locals.var_beta_dn8 * 0.1)), (locals.var_psi_dn9 + (locals.var_beta_dn9 * 0.1)), (locals.var_psi_dn10 + (locals.var_beta_dn10 * 0.1)), (locals.var_psi_dn11 + (locals.var_beta_dn11 * 0.1)), (locals.var_psi_dn14 + (locals.var_beta_dn14 * 0.1)),)
    } else {
        (locals.var_psi, locals.var_psi_dn0, locals.var_psi_dn2, locals.var_psi_dn4, locals.var_psi_dn5, locals.var_psi_dn6, locals.var_psi_dn7, locals.var_psi_dn8, locals.var_psi_dn9, locals.var_psi_dn10, locals.var_psi_dn11, locals.var_psi_dn14,)
    }
};
        locals.var_psi = assign77130_e116942;
        locals.var_psi_dn0 = assign77130_e116942_d_n0;
        locals.var_psi_dn2 = assign77130_e116942_d_n2;
        locals.var_psi_dn4 = assign77130_e116942_d_n4;
        locals.var_psi_dn5 = assign77130_e116942_d_n5;
        locals.var_psi_dn6 = assign77130_e116942_d_n6;
        locals.var_psi_dn7 = assign77130_e116942_d_n7;
        locals.var_psi_dn8 = assign77130_e116942_d_n8;
        locals.var_psi_dn9 = assign77130_e116942_d_n9;
        locals.var_psi_dn10 = assign77130_e116942_d_n10;
        locals.var_psi_dn11 = assign77130_e116942_d_n11;
        locals.var_psi_dn14 = assign77130_e116942_d_n14;
        locals.var_psi_rv = 0.0;

        let (assign77140_e116963, assign77140_e116963_d_n0, assign77140_e116963_d_n2, assign77140_e116963_d_n4, assign77140_e116963_d_n5, assign77140_e116963_d_n6, assign77140_e116963_d_n7, assign77140_e116963_d_n8, assign77140_e116963_d_n9, assign77140_e116963_d_n10, assign77140_e116963_d_n11, assign77140_e116963_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1788 == 0.0)) && (locals.var_guard1792 != 0.0)) {
        let assign77140_e116951: f64 = (locals.var_gammachi * locals.var_t0);
        let assign77140_e116954: f64 = (locals.var_psi * locals.var_psi);
        let assign77140_e116955: f64 = (assign77140_e116951 + assign77140_e116954);
        let assign77140_e116956: f64 = (assign77140_e116955).ln();
        let assign77140_e116959: f64 = (locals.var_cnst1over * locals.var_t0);
        let assign77140_e116960: f64 = (assign77140_e116959).ln();
        let assign77140_e116961: f64 = (assign77140_e116956 - assign77140_e116960);
        (assign77140_e116961, (((((locals.var_gammachi_dn0 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn0)) + ((locals.var_psi_dn0 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn0))) / assign77140_e116955) - (((locals.var_cnst1over_dn0 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn0)) / assign77140_e116959)), (((((locals.var_gammachi_dn2 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn2)) + ((locals.var_psi_dn2 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn2))) / assign77140_e116955) - (((locals.var_cnst1over_dn2 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn2)) / assign77140_e116959)), (((((locals.var_gammachi_dn4 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn4)) + ((locals.var_psi_dn4 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn4))) / assign77140_e116955) - (((locals.var_cnst1over_dn4 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn4)) / assign77140_e116959)), (((((locals.var_gammachi_dn5 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn5)) + ((locals.var_psi_dn5 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn5))) / assign77140_e116955) - (((locals.var_cnst1over_dn5 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn5)) / assign77140_e116959)), (((((locals.var_gammachi_dn6 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn6)) + ((locals.var_psi_dn6 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn6))) / assign77140_e116955) - (((locals.var_cnst1over_dn6 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn6)) / assign77140_e116959)), (((((locals.var_gammachi_dn7 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn7)) + ((locals.var_psi_dn7 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn7))) / assign77140_e116955) - (((locals.var_cnst1over_dn7 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn7)) / assign77140_e116959)), (((((locals.var_gammachi_dn8 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn8)) + ((locals.var_psi_dn8 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn8))) / assign77140_e116955) - (((locals.var_cnst1over_dn8 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn8)) / assign77140_e116959)), (((((locals.var_gammachi_dn9 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn9)) + ((locals.var_psi_dn9 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn9))) / assign77140_e116955) - (((locals.var_cnst1over_dn9 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn9)) / assign77140_e116959)), (((((locals.var_gammachi_dn10 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn10)) + ((locals.var_psi_dn10 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn10))) / assign77140_e116955) - (((locals.var_cnst1over_dn10 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn10)) / assign77140_e116959)), (((((locals.var_gammachi_dn11 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn11)) + ((locals.var_psi_dn11 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn11))) / assign77140_e116955) - (((locals.var_cnst1over_dn11 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn11)) / assign77140_e116959)), (((((locals.var_gammachi_dn14 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn14)) + ((locals.var_psi_dn14 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn14))) / assign77140_e116955) - (((locals.var_cnst1over_dn14 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn14)) / assign77140_e116959)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign77140_e116963;
        locals.var_t1_dn0 = assign77140_e116963_d_n0;
        locals.var_t1_dn2 = assign77140_e116963_d_n2;
        locals.var_t1_dn4 = assign77140_e116963_d_n4;
        locals.var_t1_dn5 = assign77140_e116963_d_n5;
        locals.var_t1_dn6 = assign77140_e116963_d_n6;
        locals.var_t1_dn7 = assign77140_e116963_d_n7;
        locals.var_t1_dn8 = assign77140_e116963_d_n8;
        locals.var_t1_dn9 = assign77140_e116963_d_n9;
        locals.var_t1_dn10 = assign77140_e116963_d_n10;
        locals.var_t1_dn11 = assign77140_e116963_d_n11;
        locals.var_t1_dn14 = assign77140_e116963_d_n14;
        locals.var_t1_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_292(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign77150_e116976, assign77150_e116976_d_n0, assign77150_e116976_d_n2, assign77150_e116976_d_n4, assign77150_e116976_d_n5, assign77150_e116976_d_n6, assign77150_e116976_d_n7, assign77150_e116976_d_n8, assign77150_e116976_d_n9, assign77150_e116976_d_n10, assign77150_e116976_d_n11, assign77150_e116976_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1788 == 0.0)) && (locals.var_guard1792 != 0.0)) {
        let assign77150_e116973: f64 = (locals.var_beta * locals.var_vxbgmtcl);
        let assign77150_e116974: f64 = (locals.var_t1 + assign77150_e116973);
        (assign77150_e116974, (locals.var_t1_dn0 + ((locals.var_beta_dn0 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn0))), (locals.var_t1_dn2 + ((locals.var_beta_dn2 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn2))), (locals.var_t1_dn4 + ((locals.var_beta_dn4 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn4))), (locals.var_t1_dn5 + ((locals.var_beta_dn5 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn5))), (locals.var_t1_dn6 + ((locals.var_beta_dn6 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn6))), (locals.var_t1_dn7 + ((locals.var_beta_dn7 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn7))), (locals.var_t1_dn8 + ((locals.var_beta_dn8 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn8))), (locals.var_t1_dn9 + ((locals.var_beta_dn9 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn9))), (locals.var_t1_dn10 + ((locals.var_beta_dn10 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn10))), (locals.var_t1_dn11 + ((locals.var_beta_dn11 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn11))), (locals.var_t1_dn14 + ((locals.var_beta_dn14 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn14))),)
    } else {
        (locals.var_chi_b, locals.var_chi_b_dn0, locals.var_chi_b_dn2, locals.var_chi_b_dn4, locals.var_chi_b_dn5, locals.var_chi_b_dn6, locals.var_chi_b_dn7, locals.var_chi_b_dn8, locals.var_chi_b_dn9, locals.var_chi_b_dn10, locals.var_chi_b_dn11, locals.var_chi_b_dn14,)
    }
};
        locals.var_chi_b = assign77150_e116976;
        locals.var_chi_b_dn0 = assign77150_e116976_d_n0;
        locals.var_chi_b_dn2 = assign77150_e116976_d_n2;
        locals.var_chi_b_dn4 = assign77150_e116976_d_n4;
        locals.var_chi_b_dn5 = assign77150_e116976_d_n5;
        locals.var_chi_b_dn6 = assign77150_e116976_d_n6;
        locals.var_chi_b_dn7 = assign77150_e116976_d_n7;
        locals.var_chi_b_dn8 = assign77150_e116976_d_n8;
        locals.var_chi_b_dn9 = assign77150_e116976_d_n9;
        locals.var_chi_b_dn10 = assign77150_e116976_d_n10;
        locals.var_chi_b_dn11 = assign77150_e116976_d_n11;
        locals.var_chi_b_dn14 = assign77150_e116976_d_n14;
        locals.var_chi_b_rv = 0.0;

        let (assign77160_e116990, assign77160_e116990_d_n0, assign77160_e116990_d_n2, assign77160_e116990_d_n4, assign77160_e116990_d_n5, assign77160_e116990_d_n6, assign77160_e116990_d_n7, assign77160_e116990_d_n8, assign77160_e116990_d_n9, assign77160_e116990_d_n10, assign77160_e116990_d_n11, assign77160_e116990_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1788 == 0.0)) && (locals.var_guard1792 != 0.0)) {
        let (assign77160_e116988, assign77160_e116988_d_n0, assign77160_e116988_d_n2, assign77160_e116988_d_n4, assign77160_e116988_d_n5, assign77160_e116988_d_n6, assign77160_e116988_d_n7, assign77160_e116988_d_n8, assign77160_e116988_d_n9, assign77160_e116988_d_n10, assign77160_e116988_d_n11, assign77160_e116988_d_n14,) = {
            if (locals.var_chi_b >= 0.0) {
                (locals.var_chi_b, locals.var_chi_b_dn0, locals.var_chi_b_dn2, locals.var_chi_b_dn4, locals.var_chi_b_dn5, locals.var_chi_b_dn6, locals.var_chi_b_dn7, locals.var_chi_b_dn8, locals.var_chi_b_dn9, locals.var_chi_b_dn10, locals.var_chi_b_dn11, locals.var_chi_b_dn14,)
            } else {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        (assign77160_e116988, assign77160_e116988_d_n0, assign77160_e116988_d_n2, assign77160_e116988_d_n4, assign77160_e116988_d_n5, assign77160_e116988_d_n6, assign77160_e116988_d_n7, assign77160_e116988_d_n8, assign77160_e116988_d_n9, assign77160_e116988_d_n10, assign77160_e116988_d_n11, assign77160_e116988_d_n14,)
    } else {
        (locals.var_chi_b, locals.var_chi_b_dn0, locals.var_chi_b_dn2, locals.var_chi_b_dn4, locals.var_chi_b_dn5, locals.var_chi_b_dn6, locals.var_chi_b_dn7, locals.var_chi_b_dn8, locals.var_chi_b_dn9, locals.var_chi_b_dn10, locals.var_chi_b_dn11, locals.var_chi_b_dn14,)
    }
};
        locals.var_chi_b = assign77160_e116990;
        locals.var_chi_b_dn0 = assign77160_e116990_d_n0;
        locals.var_chi_b_dn2 = assign77160_e116990_d_n2;
        locals.var_chi_b_dn4 = assign77160_e116990_d_n4;
        locals.var_chi_b_dn5 = assign77160_e116990_d_n5;
        locals.var_chi_b_dn6 = assign77160_e116990_d_n6;
        locals.var_chi_b_dn7 = assign77160_e116990_d_n7;
        locals.var_chi_b_dn8 = assign77160_e116990_d_n8;
        locals.var_chi_b_dn9 = assign77160_e116990_d_n9;
        locals.var_chi_b_dn10 = assign77160_e116990_d_n10;
        locals.var_chi_b_dn11 = assign77160_e116990_d_n11;
        locals.var_chi_b_dn14 = assign77160_e116990_d_n14;
        locals.var_chi_b_rv = 0.0;

        let (assign77170_e116999, assign77170_e116999_d_n0, assign77170_e116999_d_n2, assign77170_e116999_d_n4, assign77170_e116999_d_n5, assign77170_e116999_d_n6, assign77170_e116999_d_n7, assign77170_e116999_d_n8, assign77170_e116999_d_n9, assign77170_e116999_d_n10, assign77170_e116999_d_n11, assign77170_e116999_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1788 == 0.0)) && (locals.var_guard1792 != 0.0)) {
        (locals.var_chi, locals.var_chi_dn0, locals.var_chi_dn2, locals.var_chi_dn4, locals.var_chi_dn5, locals.var_chi_dn6, locals.var_chi_dn7, locals.var_chi_dn8, locals.var_chi_dn9, locals.var_chi_dn10, locals.var_chi_dn11, locals.var_chi_dn14,)
    } else {
        (locals.var_chi_a, locals.var_chi_a_dn0, locals.var_chi_a_dn2, locals.var_chi_a_dn4, locals.var_chi_a_dn5, locals.var_chi_a_dn6, locals.var_chi_a_dn7, locals.var_chi_a_dn8, locals.var_chi_a_dn9, locals.var_chi_a_dn10, locals.var_chi_a_dn11, locals.var_chi_a_dn14,)
    }
};
        locals.var_chi_a = assign77170_e116999;
        locals.var_chi_a_dn0 = assign77170_e116999_d_n0;
        locals.var_chi_a_dn2 = assign77170_e116999_d_n2;
        locals.var_chi_a_dn4 = assign77170_e116999_d_n4;
        locals.var_chi_a_dn5 = assign77170_e116999_d_n5;
        locals.var_chi_a_dn6 = assign77170_e116999_d_n6;
        locals.var_chi_a_dn7 = assign77170_e116999_d_n7;
        locals.var_chi_a_dn8 = assign77170_e116999_d_n8;
        locals.var_chi_a_dn9 = assign77170_e116999_d_n9;
        locals.var_chi_a_dn10 = assign77170_e116999_d_n10;
        locals.var_chi_a_dn11 = assign77170_e116999_d_n11;
        locals.var_chi_a_dn14 = assign77170_e116999_d_n14;
        locals.var_chi_a_rv = 0.0;

        let assign77180_e117002: f64 = if p.p33 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard1794 = assign77180_e117002;
        locals.var_guard1794_rv = 0.0;

        let assign77190_e117007: f64 = (0.2 * locals.var_chi_b);
        let assign77190_e117008: f64 = (locals.var_chi_b - assign77190_e117007);
        let assign77190_e117012: f64 = (0.2 * locals.var_chi_b);
        let assign77190_e117015: f64 = if ((locals.var_chi_a > assign77190_e117008) && (assign77190_e117012 >= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1795 = assign77190_e117015;
        locals.var_guard1795_rv = 0.0;

        let (assign77200_e117034, assign77200_e117034_d_n0, assign77200_e117034_d_n2, assign77200_e117034_d_n4, assign77200_e117034_d_n5, assign77200_e117034_d_n6, assign77200_e117034_d_n7, assign77200_e117034_d_n8, assign77200_e117034_d_n9, assign77200_e117034_d_n10, assign77200_e117034_d_n11, assign77200_e117034_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1788 == 0.0)) && (locals.var_guard1792 != 0.0)) && (locals.var_guard1794 != 0.0)) && (locals.var_guard1795 != 0.0)) {
        let assign77200_e117028: f64 = (locals.var_chi_a - locals.var_chi_b);
        let assign77200_e117031: f64 = (0.2 * locals.var_chi_b);
        let assign77200_e117032: f64 = (assign77200_e117028 + assign77200_e117031);
        (assign77200_e117032, ((locals.var_chi_a_dn0 - locals.var_chi_b_dn0) + (0.2 * locals.var_chi_b_dn0)), ((locals.var_chi_a_dn2 - locals.var_chi_b_dn2) + (0.2 * locals.var_chi_b_dn2)), ((locals.var_chi_a_dn4 - locals.var_chi_b_dn4) + (0.2 * locals.var_chi_b_dn4)), ((locals.var_chi_a_dn5 - locals.var_chi_b_dn5) + (0.2 * locals.var_chi_b_dn5)), ((locals.var_chi_a_dn6 - locals.var_chi_b_dn6) + (0.2 * locals.var_chi_b_dn6)), ((locals.var_chi_a_dn7 - locals.var_chi_b_dn7) + (0.2 * locals.var_chi_b_dn7)), ((locals.var_chi_a_dn8 - locals.var_chi_b_dn8) + (0.2 * locals.var_chi_b_dn8)), ((locals.var_chi_a_dn9 - locals.var_chi_b_dn9) + (0.2 * locals.var_chi_b_dn9)), ((locals.var_chi_a_dn10 - locals.var_chi_b_dn10) + (0.2 * locals.var_chi_b_dn10)), ((locals.var_chi_a_dn11 - locals.var_chi_b_dn11) + (0.2 * locals.var_chi_b_dn11)), ((locals.var_chi_a_dn14 - locals.var_chi_b_dn14) + (0.2 * locals.var_chi_b_dn14)),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign77200_e117034;
        locals.var_tmf1_dn0 = assign77200_e117034_d_n0;
        locals.var_tmf1_dn2 = assign77200_e117034_d_n2;
        locals.var_tmf1_dn4 = assign77200_e117034_d_n4;
        locals.var_tmf1_dn5 = assign77200_e117034_d_n5;
        locals.var_tmf1_dn6 = assign77200_e117034_d_n6;
        locals.var_tmf1_dn7 = assign77200_e117034_d_n7;
        locals.var_tmf1_dn8 = assign77200_e117034_d_n8;
        locals.var_tmf1_dn9 = assign77200_e117034_d_n9;
        locals.var_tmf1_dn10 = assign77200_e117034_d_n10;
        locals.var_tmf1_dn11 = assign77200_e117034_d_n11;
        locals.var_tmf1_dn14 = assign77200_e117034_d_n14;
        locals.var_tmf1_rv = 0.0;

        let (assign77210_e117049, assign77210_e117049_d_n0, assign77210_e117049_d_n2, assign77210_e117049_d_n4, assign77210_e117049_d_n5, assign77210_e117049_d_n6, assign77210_e117049_d_n7, assign77210_e117049_d_n8, assign77210_e117049_d_n9, assign77210_e117049_d_n10, assign77210_e117049_d_n11, assign77210_e117049_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1788 == 0.0)) && (locals.var_guard1792 != 0.0)) && (locals.var_guard1794 != 0.0)) && (locals.var_guard1795 != 0.0)) {
        let assign77210_e117047: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign77210_e117047, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)), ((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)), ((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)), ((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn4, locals.var_x2_dn5, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn8, locals.var_x2_dn9, locals.var_x2_dn10, locals.var_x2_dn11, locals.var_x2_dn14,)
    }
};
        locals.var_x2 = assign77210_e117049;
        locals.var_x2_dn0 = assign77210_e117049_d_n0;
        locals.var_x2_dn2 = assign77210_e117049_d_n2;
        locals.var_x2_dn4 = assign77210_e117049_d_n4;
        locals.var_x2_dn5 = assign77210_e117049_d_n5;
        locals.var_x2_dn6 = assign77210_e117049_d_n6;
        locals.var_x2_dn7 = assign77210_e117049_d_n7;
        locals.var_x2_dn8 = assign77210_e117049_d_n8;
        locals.var_x2_dn9 = assign77210_e117049_d_n9;
        locals.var_x2_dn10 = assign77210_e117049_d_n10;
        locals.var_x2_dn11 = assign77210_e117049_d_n11;
        locals.var_x2_dn14 = assign77210_e117049_d_n14;
        locals.var_x2_rv = 0.0;

        let (assign77220_e117068, assign77220_e117068_d_n0, assign77220_e117068_d_n2, assign77220_e117068_d_n4, assign77220_e117068_d_n5, assign77220_e117068_d_n6, assign77220_e117068_d_n7, assign77220_e117068_d_n8, assign77220_e117068_d_n9, assign77220_e117068_d_n10, assign77220_e117068_d_n11, assign77220_e117068_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1788 == 0.0)) && (locals.var_guard1792 != 0.0)) && (locals.var_guard1794 != 0.0)) && (locals.var_guard1795 != 0.0)) {
        let assign77220_e117062: f64 = (0.2 * locals.var_chi_b);
        let assign77220_e117065: f64 = (0.2 * locals.var_chi_b);
        let assign77220_e117066: f64 = (assign77220_e117062 * assign77220_e117065);
        (assign77220_e117066, (((0.2 * locals.var_chi_b_dn0) * assign77220_e117065) + (assign77220_e117062 * (0.2 * locals.var_chi_b_dn0))), (((0.2 * locals.var_chi_b_dn2) * assign77220_e117065) + (assign77220_e117062 * (0.2 * locals.var_chi_b_dn2))), (((0.2 * locals.var_chi_b_dn4) * assign77220_e117065) + (assign77220_e117062 * (0.2 * locals.var_chi_b_dn4))), (((0.2 * locals.var_chi_b_dn5) * assign77220_e117065) + (assign77220_e117062 * (0.2 * locals.var_chi_b_dn5))), (((0.2 * locals.var_chi_b_dn6) * assign77220_e117065) + (assign77220_e117062 * (0.2 * locals.var_chi_b_dn6))), (((0.2 * locals.var_chi_b_dn7) * assign77220_e117065) + (assign77220_e117062 * (0.2 * locals.var_chi_b_dn7))), (((0.2 * locals.var_chi_b_dn8) * assign77220_e117065) + (assign77220_e117062 * (0.2 * locals.var_chi_b_dn8))), (((0.2 * locals.var_chi_b_dn9) * assign77220_e117065) + (assign77220_e117062 * (0.2 * locals.var_chi_b_dn9))), (((0.2 * locals.var_chi_b_dn10) * assign77220_e117065) + (assign77220_e117062 * (0.2 * locals.var_chi_b_dn10))), (((0.2 * locals.var_chi_b_dn11) * assign77220_e117065) + (assign77220_e117062 * (0.2 * locals.var_chi_b_dn11))), (((0.2 * locals.var_chi_b_dn14) * assign77220_e117065) + (assign77220_e117062 * (0.2 * locals.var_chi_b_dn14))),)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn4, locals.var_xmax2_dn5, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn8, locals.var_xmax2_dn9, locals.var_xmax2_dn10, locals.var_xmax2_dn11, locals.var_xmax2_dn14,)
    }
};
        locals.var_xmax2 = assign77220_e117068;
        locals.var_xmax2_dn0 = assign77220_e117068_d_n0;
        locals.var_xmax2_dn2 = assign77220_e117068_d_n2;
        locals.var_xmax2_dn4 = assign77220_e117068_d_n4;
        locals.var_xmax2_dn5 = assign77220_e117068_d_n5;
        locals.var_xmax2_dn6 = assign77220_e117068_d_n6;
        locals.var_xmax2_dn7 = assign77220_e117068_d_n7;
        locals.var_xmax2_dn8 = assign77220_e117068_d_n8;
        locals.var_xmax2_dn9 = assign77220_e117068_d_n9;
        locals.var_xmax2_dn10 = assign77220_e117068_d_n10;
        locals.var_xmax2_dn11 = assign77220_e117068_d_n11;
        locals.var_xmax2_dn14 = assign77220_e117068_d_n14;
        locals.var_xmax2_rv = 0.0;

        let (assign77230_e117081, assign77230_e117081_d_n0, assign77230_e117081_d_n2, assign77230_e117081_d_n4, assign77230_e117081_d_n5, assign77230_e117081_d_n6, assign77230_e117081_d_n7, assign77230_e117081_d_n8, assign77230_e117081_d_n9, assign77230_e117081_d_n10, assign77230_e117081_d_n11, assign77230_e117081_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1788 == 0.0)) && (locals.var_guard1792 != 0.0)) && (locals.var_guard1794 != 0.0)) && (locals.var_guard1795 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign77230_e117081;
        locals.var_xp_dn0 = assign77230_e117081_d_n0;
        locals.var_xp_dn2 = assign77230_e117081_d_n2;
        locals.var_xp_dn4 = assign77230_e117081_d_n4;
        locals.var_xp_dn5 = assign77230_e117081_d_n5;
        locals.var_xp_dn6 = assign77230_e117081_d_n6;
        locals.var_xp_dn7 = assign77230_e117081_d_n7;
        locals.var_xp_dn8 = assign77230_e117081_d_n8;
        locals.var_xp_dn9 = assign77230_e117081_d_n9;
        locals.var_xp_dn10 = assign77230_e117081_d_n10;
        locals.var_xp_dn11 = assign77230_e117081_d_n11;
        locals.var_xp_dn14 = assign77230_e117081_d_n14;
        locals.var_xp_rv = 0.0;

        let (assign77240_e117094, assign77240_e117094_d_n0, assign77240_e117094_d_n2, assign77240_e117094_d_n4, assign77240_e117094_d_n5, assign77240_e117094_d_n6, assign77240_e117094_d_n7, assign77240_e117094_d_n8, assign77240_e117094_d_n9, assign77240_e117094_d_n10, assign77240_e117094_d_n11, assign77240_e117094_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1788 == 0.0)) && (locals.var_guard1792 != 0.0)) && (locals.var_guard1794 != 0.0)) && (locals.var_guard1795 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign77240_e117094;
        locals.var_xmp_dn0 = assign77240_e117094_d_n0;
        locals.var_xmp_dn2 = assign77240_e117094_d_n2;
        locals.var_xmp_dn4 = assign77240_e117094_d_n4;
        locals.var_xmp_dn5 = assign77240_e117094_d_n5;
        locals.var_xmp_dn6 = assign77240_e117094_d_n6;
        locals.var_xmp_dn7 = assign77240_e117094_d_n7;
        locals.var_xmp_dn8 = assign77240_e117094_d_n8;
        locals.var_xmp_dn9 = assign77240_e117094_d_n9;
        locals.var_xmp_dn10 = assign77240_e117094_d_n10;
        locals.var_xmp_dn11 = assign77240_e117094_d_n11;
        locals.var_xmp_dn14 = assign77240_e117094_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign77250_e117107,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1788 == 0.0)) && (locals.var_guard1792 != 0.0)) && (locals.var_guard1794 != 0.0)) && (locals.var_guard1795 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign77250_e117107;
        locals.var_m0_rv = 0.0;

        let (assign77260_e117120,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1788 == 0.0)) && (locals.var_guard1792 != 0.0)) && (locals.var_guard1794 != 0.0)) && (locals.var_guard1795 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign77260_e117120;
        locals.var_mm_rv = 0.0;

        let (assign77270_e117133, assign77270_e117133_d_n0, assign77270_e117133_d_n2, assign77270_e117133_d_n4, assign77270_e117133_d_n5, assign77270_e117133_d_n6, assign77270_e117133_d_n7, assign77270_e117133_d_n8, assign77270_e117133_d_n9, assign77270_e117133_d_n10, assign77270_e117133_d_n11, assign77270_e117133_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1788 == 0.0)) && (locals.var_guard1792 != 0.0)) && (locals.var_guard1794 != 0.0)) && (locals.var_guard1795 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign77270_e117133;
        locals.var_arg_dn0 = assign77270_e117133_d_n0;
        locals.var_arg_dn2 = assign77270_e117133_d_n2;
        locals.var_arg_dn4 = assign77270_e117133_d_n4;
        locals.var_arg_dn5 = assign77270_e117133_d_n5;
        locals.var_arg_dn6 = assign77270_e117133_d_n6;
        locals.var_arg_dn7 = assign77270_e117133_d_n7;
        locals.var_arg_dn8 = assign77270_e117133_d_n8;
        locals.var_arg_dn9 = assign77270_e117133_d_n9;
        locals.var_arg_dn10 = assign77270_e117133_d_n10;
        locals.var_arg_dn11 = assign77270_e117133_d_n11;
        locals.var_arg_dn14 = assign77270_e117133_d_n14;
        locals.var_arg_rv = 0.0;

        let (assign77280_e117146, assign77280_e117146_d_n0, assign77280_e117146_d_n2, assign77280_e117146_d_n4, assign77280_e117146_d_n5, assign77280_e117146_d_n6, assign77280_e117146_d_n7, assign77280_e117146_d_n8, assign77280_e117146_d_n9, assign77280_e117146_d_n10, assign77280_e117146_d_n11, assign77280_e117146_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1788 == 0.0)) && (locals.var_guard1792 != 0.0)) && (locals.var_guard1794 != 0.0)) && (locals.var_guard1795 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign77280_e117146;
        locals.var_dnm_dn0 = assign77280_e117146_d_n0;
        locals.var_dnm_dn2 = assign77280_e117146_d_n2;
        locals.var_dnm_dn4 = assign77280_e117146_d_n4;
        locals.var_dnm_dn5 = assign77280_e117146_d_n5;
        locals.var_dnm_dn6 = assign77280_e117146_d_n6;
        locals.var_dnm_dn7 = assign77280_e117146_d_n7;
        locals.var_dnm_dn8 = assign77280_e117146_d_n8;
        locals.var_dnm_dn9 = assign77280_e117146_d_n9;
        locals.var_dnm_dn10 = assign77280_e117146_d_n10;
        locals.var_dnm_dn11 = assign77280_e117146_d_n11;
        locals.var_dnm_dn14 = assign77280_e117146_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign77290_e117161, assign77290_e117161_d_n0, assign77290_e117161_d_n2, assign77290_e117161_d_n4, assign77290_e117161_d_n5, assign77290_e117161_d_n6, assign77290_e117161_d_n7, assign77290_e117161_d_n8, assign77290_e117161_d_n9, assign77290_e117161_d_n10, assign77290_e117161_d_n11, assign77290_e117161_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1788 == 0.0)) && (locals.var_guard1792 != 0.0)) && (locals.var_guard1794 != 0.0)) && (locals.var_guard1795 != 0.0)) {
        let assign77290_e117159: f64 = (locals.var_xp * locals.var_x2);
        (assign77290_e117159, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign77290_e117161;
        locals.var_xp_dn0 = assign77290_e117161_d_n0;
        locals.var_xp_dn2 = assign77290_e117161_d_n2;
        locals.var_xp_dn4 = assign77290_e117161_d_n4;
        locals.var_xp_dn5 = assign77290_e117161_d_n5;
        locals.var_xp_dn6 = assign77290_e117161_d_n6;
        locals.var_xp_dn7 = assign77290_e117161_d_n7;
        locals.var_xp_dn8 = assign77290_e117161_d_n8;
        locals.var_xp_dn9 = assign77290_e117161_d_n9;
        locals.var_xp_dn10 = assign77290_e117161_d_n10;
        locals.var_xp_dn11 = assign77290_e117161_d_n11;
        locals.var_xp_dn14 = assign77290_e117161_d_n14;
        locals.var_xp_rv = 0.0;

        let (assign77300_e117176, assign77300_e117176_d_n0, assign77300_e117176_d_n2, assign77300_e117176_d_n4, assign77300_e117176_d_n5, assign77300_e117176_d_n6, assign77300_e117176_d_n7, assign77300_e117176_d_n8, assign77300_e117176_d_n9, assign77300_e117176_d_n10, assign77300_e117176_d_n11, assign77300_e117176_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1788 == 0.0)) && (locals.var_guard1792 != 0.0)) && (locals.var_guard1794 != 0.0)) && (locals.var_guard1795 != 0.0)) {
        let assign77300_e117174: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign77300_e117174, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign77300_e117176;
        locals.var_xmp_dn0 = assign77300_e117176_d_n0;
        locals.var_xmp_dn2 = assign77300_e117176_d_n2;
        locals.var_xmp_dn4 = assign77300_e117176_d_n4;
        locals.var_xmp_dn5 = assign77300_e117176_d_n5;
        locals.var_xmp_dn6 = assign77300_e117176_d_n6;
        locals.var_xmp_dn7 = assign77300_e117176_d_n7;
        locals.var_xmp_dn8 = assign77300_e117176_d_n8;
        locals.var_xmp_dn9 = assign77300_e117176_d_n9;
        locals.var_xmp_dn10 = assign77300_e117176_d_n10;
        locals.var_xmp_dn11 = assign77300_e117176_d_n11;
        locals.var_xmp_dn14 = assign77300_e117176_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign77310_e117191, assign77310_e117191_d_n0, assign77310_e117191_d_n2, assign77310_e117191_d_n4, assign77310_e117191_d_n5, assign77310_e117191_d_n6, assign77310_e117191_d_n7, assign77310_e117191_d_n8, assign77310_e117191_d_n9, assign77310_e117191_d_n10, assign77310_e117191_d_n11, assign77310_e117191_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1788 == 0.0)) && (locals.var_guard1792 != 0.0)) && (locals.var_guard1794 != 0.0)) && (locals.var_guard1795 != 0.0)) {
        let assign77310_e117189: f64 = (locals.var_xp * locals.var_x2);
        (assign77310_e117189, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign77310_e117191;
        locals.var_xp_dn0 = assign77310_e117191_d_n0;
        locals.var_xp_dn2 = assign77310_e117191_d_n2;
        locals.var_xp_dn4 = assign77310_e117191_d_n4;
        locals.var_xp_dn5 = assign77310_e117191_d_n5;
        locals.var_xp_dn6 = assign77310_e117191_d_n6;
        locals.var_xp_dn7 = assign77310_e117191_d_n7;
        locals.var_xp_dn8 = assign77310_e117191_d_n8;
        locals.var_xp_dn9 = assign77310_e117191_d_n9;
        locals.var_xp_dn10 = assign77310_e117191_d_n10;
        locals.var_xp_dn11 = assign77310_e117191_d_n11;
        locals.var_xp_dn14 = assign77310_e117191_d_n14;
        locals.var_xp_rv = 0.0;

        let (assign77320_e117206, assign77320_e117206_d_n0, assign77320_e117206_d_n2, assign77320_e117206_d_n4, assign77320_e117206_d_n5, assign77320_e117206_d_n6, assign77320_e117206_d_n7, assign77320_e117206_d_n8, assign77320_e117206_d_n9, assign77320_e117206_d_n10, assign77320_e117206_d_n11, assign77320_e117206_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1788 == 0.0)) && (locals.var_guard1792 != 0.0)) && (locals.var_guard1794 != 0.0)) && (locals.var_guard1795 != 0.0)) {
        let assign77320_e117204: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign77320_e117204, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign77320_e117206;
        locals.var_xmp_dn0 = assign77320_e117206_d_n0;
        locals.var_xmp_dn2 = assign77320_e117206_d_n2;
        locals.var_xmp_dn4 = assign77320_e117206_d_n4;
        locals.var_xmp_dn5 = assign77320_e117206_d_n5;
        locals.var_xmp_dn6 = assign77320_e117206_d_n6;
        locals.var_xmp_dn7 = assign77320_e117206_d_n7;
        locals.var_xmp_dn8 = assign77320_e117206_d_n8;
        locals.var_xmp_dn9 = assign77320_e117206_d_n9;
        locals.var_xmp_dn10 = assign77320_e117206_d_n10;
        locals.var_xmp_dn11 = assign77320_e117206_d_n11;
        locals.var_xmp_dn14 = assign77320_e117206_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign77330_e117221, assign77330_e117221_d_n0, assign77330_e117221_d_n2, assign77330_e117221_d_n4, assign77330_e117221_d_n5, assign77330_e117221_d_n6, assign77330_e117221_d_n7, assign77330_e117221_d_n8, assign77330_e117221_d_n9, assign77330_e117221_d_n10, assign77330_e117221_d_n11, assign77330_e117221_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1788 == 0.0)) && (locals.var_guard1792 != 0.0)) && (locals.var_guard1794 != 0.0)) && (locals.var_guard1795 != 0.0)) {
        let assign77330_e117219: f64 = (locals.var_xp + locals.var_xmp);
        (assign77330_e117219, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn4 + locals.var_xmp_dn4), (locals.var_xp_dn5 + locals.var_xmp_dn5), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn8 + locals.var_xmp_dn8), (locals.var_xp_dn9 + locals.var_xmp_dn9), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn11 + locals.var_xmp_dn11), (locals.var_xp_dn14 + locals.var_xmp_dn14),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign77330_e117221;
        locals.var_arg_dn0 = assign77330_e117221_d_n0;
        locals.var_arg_dn2 = assign77330_e117221_d_n2;
        locals.var_arg_dn4 = assign77330_e117221_d_n4;
        locals.var_arg_dn5 = assign77330_e117221_d_n5;
        locals.var_arg_dn6 = assign77330_e117221_d_n6;
        locals.var_arg_dn7 = assign77330_e117221_d_n7;
        locals.var_arg_dn8 = assign77330_e117221_d_n8;
        locals.var_arg_dn9 = assign77330_e117221_d_n9;
        locals.var_arg_dn10 = assign77330_e117221_d_n10;
        locals.var_arg_dn11 = assign77330_e117221_d_n11;
        locals.var_arg_dn14 = assign77330_e117221_d_n14;
        locals.var_arg_rv = 0.0;

        let (assign77340_e117234, assign77340_e117234_d_n0, assign77340_e117234_d_n2, assign77340_e117234_d_n4, assign77340_e117234_d_n5, assign77340_e117234_d_n6, assign77340_e117234_d_n7, assign77340_e117234_d_n8, assign77340_e117234_d_n9, assign77340_e117234_d_n10, assign77340_e117234_d_n11, assign77340_e117234_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1788 == 0.0)) && (locals.var_guard1792 != 0.0)) && (locals.var_guard1794 != 0.0)) && (locals.var_guard1795 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign77340_e117234;
        locals.var_dnm_dn0 = assign77340_e117234_d_n0;
        locals.var_dnm_dn2 = assign77340_e117234_d_n2;
        locals.var_dnm_dn4 = assign77340_e117234_d_n4;
        locals.var_dnm_dn5 = assign77340_e117234_d_n5;
        locals.var_dnm_dn6 = assign77340_e117234_d_n6;
        locals.var_dnm_dn7 = assign77340_e117234_d_n7;
        locals.var_dnm_dn8 = assign77340_e117234_d_n8;
        locals.var_dnm_dn9 = assign77340_e117234_d_n9;
        locals.var_dnm_dn10 = assign77340_e117234_d_n10;
        locals.var_dnm_dn11 = assign77340_e117234_d_n11;
        locals.var_dnm_dn14 = assign77340_e117234_d_n14;
        locals.var_dnm_rv = 0.0;

        let assign77350_e117249: f64 = if ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard1796 = assign77350_e117249;
        locals.var_guard1796_rv = 0.0;

        let assign77360_e117252: f64 = if 2.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1797 = assign77360_e117252;
        locals.var_guard1797_rv = 0.0;

        let (assign77370_e117269,) = {
    if (((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1788 == 0.0)) && (locals.var_guard1792 != 0.0)) && (locals.var_guard1794 != 0.0)) && (locals.var_guard1795 != 0.0)) && (locals.var_guard1796 != 0.0)) && (locals.var_guard1797 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign77370_e117269;
        locals.var_mm_rv = 0.0;

        let assign77380_e117272: f64 = if 2.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard1798 = assign77380_e117272;
        locals.var_guard1798_rv = 0.0;

        let (assign77390_e117292,) = {
    if ((((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1788 == 0.0)) && (locals.var_guard1792 != 0.0)) && (locals.var_guard1794 != 0.0)) && (locals.var_guard1795 != 0.0)) && (locals.var_guard1796 != 0.0)) && (locals.var_guard1797 == 0.0)) && (locals.var_guard1798 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign77390_e117292;
        locals.var_mm_rv = 0.0;

        let assign77400_e117295: f64 = if 2.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard1799 = assign77400_e117295;
        locals.var_guard1799_rv = 0.0;

        let (assign77410_e117318,) = {
    if (((((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1788 == 0.0)) && (locals.var_guard1792 != 0.0)) && (locals.var_guard1794 != 0.0)) && (locals.var_guard1795 != 0.0)) && (locals.var_guard1796 != 0.0)) && (locals.var_guard1797 == 0.0)) && (locals.var_guard1798 == 0.0)) && (locals.var_guard1799 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign77410_e117318;
        locals.var_mm_rv = 0.0;

        let assign77420_e117321: f64 = if 2.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard1800 = assign77420_e117321;
        locals.var_guard1800_rv = 0.0;

        let (assign77430_e117347,) = {
    if ((((((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1788 == 0.0)) && (locals.var_guard1792 != 0.0)) && (locals.var_guard1794 != 0.0)) && (locals.var_guard1795 != 0.0)) && (locals.var_guard1796 != 0.0)) && (locals.var_guard1797 == 0.0)) && (locals.var_guard1798 == 0.0)) && (locals.var_guard1799 == 0.0)) && (locals.var_guard1800 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign77430_e117347;
        locals.var_mm_rv = 0.0;

        let (assign77440_e117362,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1788 == 0.0)) && (locals.var_guard1792 != 0.0)) && (locals.var_guard1794 != 0.0)) && (locals.var_guard1795 != 0.0)) && (locals.var_guard1796 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign77440_e117362;
        locals.var_m0_rv = 0.0;

        let mut assign77450_loop_guard: usize = 0;
        while {
            let assign77450_cond_e117378: f64 = if (((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1788 == 0.0)) && (locals.var_guard1792 != 0.0)) && (locals.var_guard1794 != 0.0)) && (locals.var_guard1795 != 0.0)) && (locals.var_guard1796 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign77450_cond_e117378 != 0.0
        } {
            assign77450_loop_guard += 1;
            assert!(assign77450_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign77450_body0_e117394, assign77450_body0_e117394_d_n0, assign77450_body0_e117394_d_n2, assign77450_body0_e117394_d_n4, assign77450_body0_e117394_d_n5, assign77450_body0_e117394_d_n6, assign77450_body0_e117394_d_n7, assign77450_body0_e117394_d_n8, assign77450_body0_e117394_d_n9, assign77450_body0_e117394_d_n10, assign77450_body0_e117394_d_n11, assign77450_body0_e117394_d_n14,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1788 == 0.0)) && (locals.var_guard1792 != 0.0)) && (locals.var_guard1794 != 0.0)) && (locals.var_guard1795 != 0.0)) && (locals.var_guard1796 != 0.0)) {
        let assign77450_body0_e117392: f64 = (locals.var_dnm).sqrt();
        (assign77450_body0_e117392, (locals.var_dnm_dn0 / (2.0 * assign77450_body0_e117392)), (locals.var_dnm_dn2 / (2.0 * assign77450_body0_e117392)), (locals.var_dnm_dn4 / (2.0 * assign77450_body0_e117392)), (locals.var_dnm_dn5 / (2.0 * assign77450_body0_e117392)), (locals.var_dnm_dn6 / (2.0 * assign77450_body0_e117392)), (locals.var_dnm_dn7 / (2.0 * assign77450_body0_e117392)), (locals.var_dnm_dn8 / (2.0 * assign77450_body0_e117392)), (locals.var_dnm_dn9 / (2.0 * assign77450_body0_e117392)), (locals.var_dnm_dn10 / (2.0 * assign77450_body0_e117392)), (locals.var_dnm_dn11 / (2.0 * assign77450_body0_e117392)), (locals.var_dnm_dn14 / (2.0 * assign77450_body0_e117392)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
            locals.var_dnm = assign77450_body0_e117394;
            locals.var_dnm_dn0 = assign77450_body0_e117394_d_n0;
            locals.var_dnm_dn2 = assign77450_body0_e117394_d_n2;
            locals.var_dnm_dn4 = assign77450_body0_e117394_d_n4;
            locals.var_dnm_dn5 = assign77450_body0_e117394_d_n5;
            locals.var_dnm_dn6 = assign77450_body0_e117394_d_n6;
            locals.var_dnm_dn7 = assign77450_body0_e117394_d_n7;
            locals.var_dnm_dn8 = assign77450_body0_e117394_d_n8;
            locals.var_dnm_dn9 = assign77450_body0_e117394_d_n9;
            locals.var_dnm_dn10 = assign77450_body0_e117394_d_n10;
            locals.var_dnm_dn11 = assign77450_body0_e117394_d_n11;
            locals.var_dnm_dn14 = assign77450_body0_e117394_d_n14;
            locals.var_dnm_rv = 0.0;
            let (assign77450_body1_e117411,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1788 == 0.0)) && (locals.var_guard1792 != 0.0)) && (locals.var_guard1794 != 0.0)) && (locals.var_guard1795 != 0.0)) && (locals.var_guard1796 != 0.0)) {
        let assign77450_body1_e117409: f64 = (locals.var_m0 + 1.0);
        (assign77450_body1_e117409,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign77450_body1_e117411;
            locals.var_m0_rv = 0.0;
        }

    }

    pub(super) fn stamp_reactive_block_293(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign77460_e117438, assign77460_e117438_d_n0, assign77460_e117438_d_n2, assign77460_e117438_d_n4, assign77460_e117438_d_n5, assign77460_e117438_d_n6, assign77460_e117438_d_n7, assign77460_e117438_d_n8, assign77460_e117438_d_n9, assign77460_e117438_d_n10, assign77460_e117438_d_n11, assign77460_e117438_d_n14,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1788 == 0.0)) && (locals.var_guard1792 != 0.0)) && (locals.var_guard1794 != 0.0)) && (locals.var_guard1795 != 0.0)) && (locals.var_guard1796 == 0.0)) {
        let (assign77460_e117436, assign77460_e117436_d_n0, assign77460_e117436_d_n2, assign77460_e117436_d_n4, assign77460_e117436_d_n5, assign77460_e117436_d_n6, assign77460_e117436_d_n7, assign77460_e117436_d_n8, assign77460_e117436_d_n9, assign77460_e117436_d_n10, assign77460_e117436_d_n11, assign77460_e117436_d_n14,) = {
            if (locals.var_dnm == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign77460_e117433: f64 = (2.0 * 2.0);
                let assign77460_e117434: f64 = (1.0 / assign77460_e117433);
                let assign77460_e117435: f64 = (locals.var_dnm).powf(assign77460_e117434);
                (assign77460_e117435, if 0.0 == 0.0 && ((assign77460_e117434) as f64).is_finite() && ((assign77460_e117434) as f64).fract() == 0.0 { if assign77460_e117434 == 0.0 { 0.0 } else { (assign77460_e117434 * ((locals.var_dnm).powf(assign77460_e117434 - 1.0) * locals.var_dnm_dn0)) } } else { (assign77460_e117435 * (assign77460_e117434 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign77460_e117434) as f64).is_finite() && ((assign77460_e117434) as f64).fract() == 0.0 { if assign77460_e117434 == 0.0 { 0.0 } else { (assign77460_e117434 * ((locals.var_dnm).powf(assign77460_e117434 - 1.0) * locals.var_dnm_dn2)) } } else { (assign77460_e117435 * (assign77460_e117434 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign77460_e117434) as f64).is_finite() && ((assign77460_e117434) as f64).fract() == 0.0 { if assign77460_e117434 == 0.0 { 0.0 } else { (assign77460_e117434 * ((locals.var_dnm).powf(assign77460_e117434 - 1.0) * locals.var_dnm_dn4)) } } else { (assign77460_e117435 * (assign77460_e117434 * (locals.var_dnm_dn4 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign77460_e117434) as f64).is_finite() && ((assign77460_e117434) as f64).fract() == 0.0 { if assign77460_e117434 == 0.0 { 0.0 } else { (assign77460_e117434 * ((locals.var_dnm).powf(assign77460_e117434 - 1.0) * locals.var_dnm_dn5)) } } else { (assign77460_e117435 * (assign77460_e117434 * (locals.var_dnm_dn5 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign77460_e117434) as f64).is_finite() && ((assign77460_e117434) as f64).fract() == 0.0 { if assign77460_e117434 == 0.0 { 0.0 } else { (assign77460_e117434 * ((locals.var_dnm).powf(assign77460_e117434 - 1.0) * locals.var_dnm_dn6)) } } else { (assign77460_e117435 * (assign77460_e117434 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign77460_e117434) as f64).is_finite() && ((assign77460_e117434) as f64).fract() == 0.0 { if assign77460_e117434 == 0.0 { 0.0 } else { (assign77460_e117434 * ((locals.var_dnm).powf(assign77460_e117434 - 1.0) * locals.var_dnm_dn7)) } } else { (assign77460_e117435 * (assign77460_e117434 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign77460_e117434) as f64).is_finite() && ((assign77460_e117434) as f64).fract() == 0.0 { if assign77460_e117434 == 0.0 { 0.0 } else { (assign77460_e117434 * ((locals.var_dnm).powf(assign77460_e117434 - 1.0) * locals.var_dnm_dn8)) } } else { (assign77460_e117435 * (assign77460_e117434 * (locals.var_dnm_dn8 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign77460_e117434) as f64).is_finite() && ((assign77460_e117434) as f64).fract() == 0.0 { if assign77460_e117434 == 0.0 { 0.0 } else { (assign77460_e117434 * ((locals.var_dnm).powf(assign77460_e117434 - 1.0) * locals.var_dnm_dn9)) } } else { (assign77460_e117435 * (assign77460_e117434 * (locals.var_dnm_dn9 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign77460_e117434) as f64).is_finite() && ((assign77460_e117434) as f64).fract() == 0.0 { if assign77460_e117434 == 0.0 { 0.0 } else { (assign77460_e117434 * ((locals.var_dnm).powf(assign77460_e117434 - 1.0) * locals.var_dnm_dn10)) } } else { (assign77460_e117435 * (assign77460_e117434 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign77460_e117434) as f64).is_finite() && ((assign77460_e117434) as f64).fract() == 0.0 { if assign77460_e117434 == 0.0 { 0.0 } else { (assign77460_e117434 * ((locals.var_dnm).powf(assign77460_e117434 - 1.0) * locals.var_dnm_dn11)) } } else { (assign77460_e117435 * (assign77460_e117434 * (locals.var_dnm_dn11 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign77460_e117434) as f64).is_finite() && ((assign77460_e117434) as f64).fract() == 0.0 { if assign77460_e117434 == 0.0 { 0.0 } else { (assign77460_e117434 * ((locals.var_dnm).powf(assign77460_e117434 - 1.0) * locals.var_dnm_dn14)) } } else { (assign77460_e117435 * (assign77460_e117434 * (locals.var_dnm_dn14 / locals.var_dnm))) },)
            }
        };
        (assign77460_e117436, assign77460_e117436_d_n0, assign77460_e117436_d_n2, assign77460_e117436_d_n4, assign77460_e117436_d_n5, assign77460_e117436_d_n6, assign77460_e117436_d_n7, assign77460_e117436_d_n8, assign77460_e117436_d_n9, assign77460_e117436_d_n10, assign77460_e117436_d_n11, assign77460_e117436_d_n14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign77460_e117438;
        locals.var_dnm_dn0 = assign77460_e117438_d_n0;
        locals.var_dnm_dn2 = assign77460_e117438_d_n2;
        locals.var_dnm_dn4 = assign77460_e117438_d_n4;
        locals.var_dnm_dn5 = assign77460_e117438_d_n5;
        locals.var_dnm_dn6 = assign77460_e117438_d_n6;
        locals.var_dnm_dn7 = assign77460_e117438_d_n7;
        locals.var_dnm_dn8 = assign77460_e117438_d_n8;
        locals.var_dnm_dn9 = assign77460_e117438_d_n9;
        locals.var_dnm_dn10 = assign77460_e117438_d_n10;
        locals.var_dnm_dn11 = assign77460_e117438_d_n11;
        locals.var_dnm_dn14 = assign77460_e117438_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign77470_e117453, assign77470_e117453_d_n0, assign77470_e117453_d_n2, assign77470_e117453_d_n4, assign77470_e117453_d_n5, assign77470_e117453_d_n6, assign77470_e117453_d_n7, assign77470_e117453_d_n8, assign77470_e117453_d_n9, assign77470_e117453_d_n10, assign77470_e117453_d_n11, assign77470_e117453_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1788 == 0.0)) && (locals.var_guard1792 != 0.0)) && (locals.var_guard1794 != 0.0)) && (locals.var_guard1795 != 0.0)) {
        let assign77470_e117451: f64 = (1.0 / locals.var_dnm);
        (assign77470_e117451, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn4 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn5 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn8 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn9 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn11 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn14 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign77470_e117453;
        locals.var_dnm_dn0 = assign77470_e117453_d_n0;
        locals.var_dnm_dn2 = assign77470_e117453_d_n2;
        locals.var_dnm_dn4 = assign77470_e117453_d_n4;
        locals.var_dnm_dn5 = assign77470_e117453_d_n5;
        locals.var_dnm_dn6 = assign77470_e117453_d_n6;
        locals.var_dnm_dn7 = assign77470_e117453_d_n7;
        locals.var_dnm_dn8 = assign77470_e117453_d_n8;
        locals.var_dnm_dn9 = assign77470_e117453_d_n9;
        locals.var_dnm_dn10 = assign77470_e117453_d_n10;
        locals.var_dnm_dn11 = assign77470_e117453_d_n11;
        locals.var_dnm_dn14 = assign77470_e117453_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign77480_e117472, assign77480_e117472_d_n0, assign77480_e117472_d_n2, assign77480_e117472_d_n4, assign77480_e117472_d_n5, assign77480_e117472_d_n6, assign77480_e117472_d_n7, assign77480_e117472_d_n8, assign77480_e117472_d_n9, assign77480_e117472_d_n10, assign77480_e117472_d_n11, assign77480_e117472_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1788 == 0.0)) && (locals.var_guard1792 != 0.0)) && (locals.var_guard1794 != 0.0)) && (locals.var_guard1795 != 0.0)) {
        let assign77480_e117467: f64 = (0.2 * locals.var_chi_b);
        let assign77480_e117468: f64 = (locals.var_tmf1 * assign77480_e117467);
        let assign77480_e117470: f64 = (assign77480_e117468 * locals.var_dnm);
        (assign77480_e117470, ((((locals.var_tmf1_dn0 * assign77480_e117467) + (locals.var_tmf1 * (0.2 * locals.var_chi_b_dn0))) * locals.var_dnm) + (assign77480_e117468 * locals.var_dnm_dn0)), ((((locals.var_tmf1_dn2 * assign77480_e117467) + (locals.var_tmf1 * (0.2 * locals.var_chi_b_dn2))) * locals.var_dnm) + (assign77480_e117468 * locals.var_dnm_dn2)), ((((locals.var_tmf1_dn4 * assign77480_e117467) + (locals.var_tmf1 * (0.2 * locals.var_chi_b_dn4))) * locals.var_dnm) + (assign77480_e117468 * locals.var_dnm_dn4)), ((((locals.var_tmf1_dn5 * assign77480_e117467) + (locals.var_tmf1 * (0.2 * locals.var_chi_b_dn5))) * locals.var_dnm) + (assign77480_e117468 * locals.var_dnm_dn5)), ((((locals.var_tmf1_dn6 * assign77480_e117467) + (locals.var_tmf1 * (0.2 * locals.var_chi_b_dn6))) * locals.var_dnm) + (assign77480_e117468 * locals.var_dnm_dn6)), ((((locals.var_tmf1_dn7 * assign77480_e117467) + (locals.var_tmf1 * (0.2 * locals.var_chi_b_dn7))) * locals.var_dnm) + (assign77480_e117468 * locals.var_dnm_dn7)), ((((locals.var_tmf1_dn8 * assign77480_e117467) + (locals.var_tmf1 * (0.2 * locals.var_chi_b_dn8))) * locals.var_dnm) + (assign77480_e117468 * locals.var_dnm_dn8)), ((((locals.var_tmf1_dn9 * assign77480_e117467) + (locals.var_tmf1 * (0.2 * locals.var_chi_b_dn9))) * locals.var_dnm) + (assign77480_e117468 * locals.var_dnm_dn9)), ((((locals.var_tmf1_dn10 * assign77480_e117467) + (locals.var_tmf1 * (0.2 * locals.var_chi_b_dn10))) * locals.var_dnm) + (assign77480_e117468 * locals.var_dnm_dn10)), ((((locals.var_tmf1_dn11 * assign77480_e117467) + (locals.var_tmf1 * (0.2 * locals.var_chi_b_dn11))) * locals.var_dnm) + (assign77480_e117468 * locals.var_dnm_dn11)), ((((locals.var_tmf1_dn14 * assign77480_e117467) + (locals.var_tmf1 * (0.2 * locals.var_chi_b_dn14))) * locals.var_dnm) + (assign77480_e117468 * locals.var_dnm_dn14)),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn11, locals.var_tmf0_dn14,)
    }
};
        locals.var_tmf0 = assign77480_e117472;
        locals.var_tmf0_dn0 = assign77480_e117472_d_n0;
        locals.var_tmf0_dn2 = assign77480_e117472_d_n2;
        locals.var_tmf0_dn4 = assign77480_e117472_d_n4;
        locals.var_tmf0_dn5 = assign77480_e117472_d_n5;
        locals.var_tmf0_dn6 = assign77480_e117472_d_n6;
        locals.var_tmf0_dn7 = assign77480_e117472_d_n7;
        locals.var_tmf0_dn8 = assign77480_e117472_d_n8;
        locals.var_tmf0_dn9 = assign77480_e117472_d_n9;
        locals.var_tmf0_dn10 = assign77480_e117472_d_n10;
        locals.var_tmf0_dn11 = assign77480_e117472_d_n11;
        locals.var_tmf0_dn14 = assign77480_e117472_d_n14;
        locals.var_tmf0_rv = 0.0;

        let (assign77490_e117493, assign77490_e117493_d_n0, assign77490_e117493_d_n2, assign77490_e117493_d_n4, assign77490_e117493_d_n5, assign77490_e117493_d_n6, assign77490_e117493_d_n7, assign77490_e117493_d_n8, assign77490_e117493_d_n9, assign77490_e117493_d_n10, assign77490_e117493_d_n11, assign77490_e117493_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1788 == 0.0)) && (locals.var_guard1792 != 0.0)) && (locals.var_guard1794 != 0.0)) && (locals.var_guard1795 != 0.0)) {
        let assign77490_e117485: f64 = (0.2 * locals.var_chi_b);
        let assign77490_e117487: f64 = (assign77490_e117485 * locals.var_xmp);
        let assign77490_e117489: f64 = (assign77490_e117487 * locals.var_dnm);
        let assign77490_e117491: f64 = (assign77490_e117489 / locals.var_arg);
        (assign77490_e117491, ((((((((0.2 * locals.var_chi_b_dn0) * locals.var_xmp) + (assign77490_e117485 * locals.var_xmp_dn0)) * locals.var_dnm) + (assign77490_e117487 * locals.var_dnm_dn0)) * locals.var_arg) - (assign77490_e117489 * locals.var_arg_dn0)) / (locals.var_arg * locals.var_arg)), ((((((((0.2 * locals.var_chi_b_dn2) * locals.var_xmp) + (assign77490_e117485 * locals.var_xmp_dn2)) * locals.var_dnm) + (assign77490_e117487 * locals.var_dnm_dn2)) * locals.var_arg) - (assign77490_e117489 * locals.var_arg_dn2)) / (locals.var_arg * locals.var_arg)), ((((((((0.2 * locals.var_chi_b_dn4) * locals.var_xmp) + (assign77490_e117485 * locals.var_xmp_dn4)) * locals.var_dnm) + (assign77490_e117487 * locals.var_dnm_dn4)) * locals.var_arg) - (assign77490_e117489 * locals.var_arg_dn4)) / (locals.var_arg * locals.var_arg)), ((((((((0.2 * locals.var_chi_b_dn5) * locals.var_xmp) + (assign77490_e117485 * locals.var_xmp_dn5)) * locals.var_dnm) + (assign77490_e117487 * locals.var_dnm_dn5)) * locals.var_arg) - (assign77490_e117489 * locals.var_arg_dn5)) / (locals.var_arg * locals.var_arg)), ((((((((0.2 * locals.var_chi_b_dn6) * locals.var_xmp) + (assign77490_e117485 * locals.var_xmp_dn6)) * locals.var_dnm) + (assign77490_e117487 * locals.var_dnm_dn6)) * locals.var_arg) - (assign77490_e117489 * locals.var_arg_dn6)) / (locals.var_arg * locals.var_arg)), ((((((((0.2 * locals.var_chi_b_dn7) * locals.var_xmp) + (assign77490_e117485 * locals.var_xmp_dn7)) * locals.var_dnm) + (assign77490_e117487 * locals.var_dnm_dn7)) * locals.var_arg) - (assign77490_e117489 * locals.var_arg_dn7)) / (locals.var_arg * locals.var_arg)), ((((((((0.2 * locals.var_chi_b_dn8) * locals.var_xmp) + (assign77490_e117485 * locals.var_xmp_dn8)) * locals.var_dnm) + (assign77490_e117487 * locals.var_dnm_dn8)) * locals.var_arg) - (assign77490_e117489 * locals.var_arg_dn8)) / (locals.var_arg * locals.var_arg)), ((((((((0.2 * locals.var_chi_b_dn9) * locals.var_xmp) + (assign77490_e117485 * locals.var_xmp_dn9)) * locals.var_dnm) + (assign77490_e117487 * locals.var_dnm_dn9)) * locals.var_arg) - (assign77490_e117489 * locals.var_arg_dn9)) / (locals.var_arg * locals.var_arg)), ((((((((0.2 * locals.var_chi_b_dn10) * locals.var_xmp) + (assign77490_e117485 * locals.var_xmp_dn10)) * locals.var_dnm) + (assign77490_e117487 * locals.var_dnm_dn10)) * locals.var_arg) - (assign77490_e117489 * locals.var_arg_dn10)) / (locals.var_arg * locals.var_arg)), ((((((((0.2 * locals.var_chi_b_dn11) * locals.var_xmp) + (assign77490_e117485 * locals.var_xmp_dn11)) * locals.var_dnm) + (assign77490_e117487 * locals.var_dnm_dn11)) * locals.var_arg) - (assign77490_e117489 * locals.var_arg_dn11)) / (locals.var_arg * locals.var_arg)), ((((((((0.2 * locals.var_chi_b_dn14) * locals.var_xmp) + (assign77490_e117485 * locals.var_xmp_dn14)) * locals.var_dnm) + (assign77490_e117487 * locals.var_dnm_dn14)) * locals.var_arg) - (assign77490_e117489 * locals.var_arg_dn14)) / (locals.var_arg * locals.var_arg)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign77490_e117493;
        locals.var_t1_dn0 = assign77490_e117493_d_n0;
        locals.var_t1_dn2 = assign77490_e117493_d_n2;
        locals.var_t1_dn4 = assign77490_e117493_d_n4;
        locals.var_t1_dn5 = assign77490_e117493_d_n5;
        locals.var_t1_dn6 = assign77490_e117493_d_n6;
        locals.var_t1_dn7 = assign77490_e117493_d_n7;
        locals.var_t1_dn8 = assign77490_e117493_d_n8;
        locals.var_t1_dn9 = assign77490_e117493_d_n9;
        locals.var_t1_dn10 = assign77490_e117493_d_n10;
        locals.var_t1_dn11 = assign77490_e117493_d_n11;
        locals.var_t1_dn14 = assign77490_e117493_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign77500_e117512, assign77500_e117512_d_n0, assign77500_e117512_d_n2, assign77500_e117512_d_n4, assign77500_e117512_d_n5, assign77500_e117512_d_n6, assign77500_e117512_d_n7, assign77500_e117512_d_n8, assign77500_e117512_d_n9, assign77500_e117512_d_n10, assign77500_e117512_d_n11, assign77500_e117512_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1788 == 0.0)) && (locals.var_guard1792 != 0.0)) && (locals.var_guard1794 != 0.0)) && (locals.var_guard1795 != 0.0)) {
        let assign77500_e117507: f64 = (0.2 * locals.var_chi_b);
        let assign77500_e117508: f64 = (locals.var_chi_b - assign77500_e117507);
        let assign77500_e117510: f64 = (assign77500_e117508 + locals.var_tmf0);
        (assign77500_e117510, ((locals.var_chi_b_dn0 - (0.2 * locals.var_chi_b_dn0)) + locals.var_tmf0_dn0), ((locals.var_chi_b_dn2 - (0.2 * locals.var_chi_b_dn2)) + locals.var_tmf0_dn2), ((locals.var_chi_b_dn4 - (0.2 * locals.var_chi_b_dn4)) + locals.var_tmf0_dn4), ((locals.var_chi_b_dn5 - (0.2 * locals.var_chi_b_dn5)) + locals.var_tmf0_dn5), ((locals.var_chi_b_dn6 - (0.2 * locals.var_chi_b_dn6)) + locals.var_tmf0_dn6), ((locals.var_chi_b_dn7 - (0.2 * locals.var_chi_b_dn7)) + locals.var_tmf0_dn7), ((locals.var_chi_b_dn8 - (0.2 * locals.var_chi_b_dn8)) + locals.var_tmf0_dn8), ((locals.var_chi_b_dn9 - (0.2 * locals.var_chi_b_dn9)) + locals.var_tmf0_dn9), ((locals.var_chi_b_dn10 - (0.2 * locals.var_chi_b_dn10)) + locals.var_tmf0_dn10), ((locals.var_chi_b_dn11 - (0.2 * locals.var_chi_b_dn11)) + locals.var_tmf0_dn11), ((locals.var_chi_b_dn14 - (0.2 * locals.var_chi_b_dn14)) + locals.var_tmf0_dn14),)
    } else {
        (locals.var_chi, locals.var_chi_dn0, locals.var_chi_dn2, locals.var_chi_dn4, locals.var_chi_dn5, locals.var_chi_dn6, locals.var_chi_dn7, locals.var_chi_dn8, locals.var_chi_dn9, locals.var_chi_dn10, locals.var_chi_dn11, locals.var_chi_dn14,)
    }
};
        locals.var_chi = assign77500_e117512;
        locals.var_chi_dn0 = assign77500_e117512_d_n0;
        locals.var_chi_dn2 = assign77500_e117512_d_n2;
        locals.var_chi_dn4 = assign77500_e117512_d_n4;
        locals.var_chi_dn5 = assign77500_e117512_d_n5;
        locals.var_chi_dn6 = assign77500_e117512_d_n6;
        locals.var_chi_dn7 = assign77500_e117512_d_n7;
        locals.var_chi_dn8 = assign77500_e117512_d_n8;
        locals.var_chi_dn9 = assign77500_e117512_d_n9;
        locals.var_chi_dn10 = assign77500_e117512_d_n10;
        locals.var_chi_dn11 = assign77500_e117512_d_n11;
        locals.var_chi_dn14 = assign77500_e117512_d_n14;
        locals.var_chi_rv = 0.0;

        let (assign77510_e117525, assign77510_e117525_d_n0, assign77510_e117525_d_n2, assign77510_e117525_d_n4, assign77510_e117525_d_n5, assign77510_e117525_d_n6, assign77510_e117525_d_n7, assign77510_e117525_d_n8, assign77510_e117525_d_n9, assign77510_e117525_d_n10, assign77510_e117525_d_n11, assign77510_e117525_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1788 == 0.0)) && (locals.var_guard1792 != 0.0)) && (locals.var_guard1794 != 0.0)) && (locals.var_guard1795 != 0.0)) {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign77510_e117525;
        locals.var_t1_dn0 = assign77510_e117525_d_n0;
        locals.var_t1_dn2 = assign77510_e117525_d_n2;
        locals.var_t1_dn4 = assign77510_e117525_d_n4;
        locals.var_t1_dn5 = assign77510_e117525_d_n5;
        locals.var_t1_dn6 = assign77510_e117525_d_n6;
        locals.var_t1_dn7 = assign77510_e117525_d_n7;
        locals.var_t1_dn8 = assign77510_e117525_d_n8;
        locals.var_t1_dn9 = assign77510_e117525_d_n9;
        locals.var_t1_dn10 = assign77510_e117525_d_n10;
        locals.var_t1_dn11 = assign77510_e117525_d_n11;
        locals.var_t1_dn14 = assign77510_e117525_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign77520_e117539, assign77520_e117539_d_n0, assign77520_e117539_d_n2, assign77520_e117539_d_n4, assign77520_e117539_d_n5, assign77520_e117539_d_n6, assign77520_e117539_d_n7, assign77520_e117539_d_n8, assign77520_e117539_d_n9, assign77520_e117539_d_n10, assign77520_e117539_d_n11, assign77520_e117539_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1788 == 0.0)) && (locals.var_guard1792 != 0.0)) && (locals.var_guard1794 != 0.0)) && (locals.var_guard1795 == 0.0)) {
        (locals.var_chi_a, locals.var_chi_a_dn0, locals.var_chi_a_dn2, locals.var_chi_a_dn4, locals.var_chi_a_dn5, locals.var_chi_a_dn6, locals.var_chi_a_dn7, locals.var_chi_a_dn8, locals.var_chi_a_dn9, locals.var_chi_a_dn10, locals.var_chi_a_dn11, locals.var_chi_a_dn14,)
    } else {
        (locals.var_chi, locals.var_chi_dn0, locals.var_chi_dn2, locals.var_chi_dn4, locals.var_chi_dn5, locals.var_chi_dn6, locals.var_chi_dn7, locals.var_chi_dn8, locals.var_chi_dn9, locals.var_chi_dn10, locals.var_chi_dn11, locals.var_chi_dn14,)
    }
};
        locals.var_chi = assign77520_e117539;
        locals.var_chi_dn0 = assign77520_e117539_d_n0;
        locals.var_chi_dn2 = assign77520_e117539_d_n2;
        locals.var_chi_dn4 = assign77520_e117539_d_n4;
        locals.var_chi_dn5 = assign77520_e117539_d_n5;
        locals.var_chi_dn6 = assign77520_e117539_d_n6;
        locals.var_chi_dn7 = assign77520_e117539_d_n7;
        locals.var_chi_dn8 = assign77520_e117539_d_n8;
        locals.var_chi_dn9 = assign77520_e117539_d_n9;
        locals.var_chi_dn10 = assign77520_e117539_d_n10;
        locals.var_chi_dn11 = assign77520_e117539_d_n11;
        locals.var_chi_dn14 = assign77520_e117539_d_n14;
        locals.var_chi_rv = 0.0;

        let (assign77530_e117553, assign77530_e117553_d_n0, assign77530_e117553_d_n2, assign77530_e117553_d_n4, assign77530_e117553_d_n5, assign77530_e117553_d_n6, assign77530_e117553_d_n7, assign77530_e117553_d_n8, assign77530_e117553_d_n9, assign77530_e117553_d_n10, assign77530_e117553_d_n11, assign77530_e117553_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1788 == 0.0)) && (locals.var_guard1792 != 0.0)) && (locals.var_guard1794 != 0.0)) && (locals.var_guard1795 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign77530_e117553;
        locals.var_t1_dn0 = assign77530_e117553_d_n0;
        locals.var_t1_dn2 = assign77530_e117553_d_n2;
        locals.var_t1_dn4 = assign77530_e117553_d_n4;
        locals.var_t1_dn5 = assign77530_e117553_d_n5;
        locals.var_t1_dn6 = assign77530_e117553_d_n6;
        locals.var_t1_dn7 = assign77530_e117553_d_n7;
        locals.var_t1_dn8 = assign77530_e117553_d_n8;
        locals.var_t1_dn9 = assign77530_e117553_d_n9;
        locals.var_t1_dn10 = assign77530_e117553_d_n10;
        locals.var_t1_dn11 = assign77530_e117553_d_n11;
        locals.var_t1_dn14 = assign77530_e117553_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign77540_e117570, assign77540_e117570_d_n0, assign77540_e117570_d_n2, assign77540_e117570_d_n4, assign77540_e117570_d_n5, assign77540_e117570_d_n6, assign77540_e117570_d_n7, assign77540_e117570_d_n8, assign77540_e117570_d_n9, assign77540_e117570_d_n10, assign77540_e117570_d_n11, assign77540_e117570_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1788 == 0.0)) && (locals.var_guard1792 != 0.0)) && (locals.var_guard1794 == 0.0)) {
        let (assign77540_e117568, assign77540_e117568_d_n0, assign77540_e117568_d_n2, assign77540_e117568_d_n4, assign77540_e117568_d_n5, assign77540_e117568_d_n6, assign77540_e117568_d_n7, assign77540_e117568_d_n8, assign77540_e117568_d_n9, assign77540_e117568_d_n10, assign77540_e117568_d_n11, assign77540_e117568_d_n14,) = {
            if (locals.var_chi_a <= locals.var_chi_b) {
                (locals.var_chi_a, locals.var_chi_a_dn0, locals.var_chi_a_dn2, locals.var_chi_a_dn4, locals.var_chi_a_dn5, locals.var_chi_a_dn6, locals.var_chi_a_dn7, locals.var_chi_a_dn8, locals.var_chi_a_dn9, locals.var_chi_a_dn10, locals.var_chi_a_dn11, locals.var_chi_a_dn14,)
            } else {
                (locals.var_chi_b, locals.var_chi_b_dn0, locals.var_chi_b_dn2, locals.var_chi_b_dn4, locals.var_chi_b_dn5, locals.var_chi_b_dn6, locals.var_chi_b_dn7, locals.var_chi_b_dn8, locals.var_chi_b_dn9, locals.var_chi_b_dn10, locals.var_chi_b_dn11, locals.var_chi_b_dn14,)
            }
        };
        (assign77540_e117568, assign77540_e117568_d_n0, assign77540_e117568_d_n2, assign77540_e117568_d_n4, assign77540_e117568_d_n5, assign77540_e117568_d_n6, assign77540_e117568_d_n7, assign77540_e117568_d_n8, assign77540_e117568_d_n9, assign77540_e117568_d_n10, assign77540_e117568_d_n11, assign77540_e117568_d_n14,)
    } else {
        (locals.var_chi, locals.var_chi_dn0, locals.var_chi_dn2, locals.var_chi_dn4, locals.var_chi_dn5, locals.var_chi_dn6, locals.var_chi_dn7, locals.var_chi_dn8, locals.var_chi_dn9, locals.var_chi_dn10, locals.var_chi_dn11, locals.var_chi_dn14,)
    }
};
        locals.var_chi = assign77540_e117570;
        locals.var_chi_dn0 = assign77540_e117570_d_n0;
        locals.var_chi_dn2 = assign77540_e117570_d_n2;
        locals.var_chi_dn4 = assign77540_e117570_d_n4;
        locals.var_chi_dn5 = assign77540_e117570_d_n5;
        locals.var_chi_dn6 = assign77540_e117570_d_n6;
        locals.var_chi_dn7 = assign77540_e117570_d_n7;
        locals.var_chi_dn8 = assign77540_e117570_d_n8;
        locals.var_chi_dn9 = assign77540_e117570_d_n9;
        locals.var_chi_dn10 = assign77540_e117570_d_n10;
        locals.var_chi_dn11 = assign77540_e117570_d_n11;
        locals.var_chi_dn14 = assign77540_e117570_d_n14;
        locals.var_chi_rv = 0.0;

        let assign77550_e117573: f64 = if p.p33 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1801 = assign77550_e117573;
        locals.var_guard1801_rv = 0.0;

        let (assign77560_e117586, assign77560_e117586_d_n0, assign77560_e117586_d_n2, assign77560_e117586_d_n4, assign77560_e117586_d_n5, assign77560_e117586_d_n6, assign77560_e117586_d_n7, assign77560_e117586_d_n8, assign77560_e117586_d_n9, assign77560_e117586_d_n10, assign77560_e117586_d_n11, assign77560_e117586_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1788 == 0.0)) && (locals.var_guard1801 != 0.0)) {
        let assign77560_e117582: f64 = (locals.var_chi * locals.var_beta_inv);
        let assign77560_e117584: f64 = (assign77560_e117582 - locals.var_vxbgmtcl);
        (assign77560_e117584, (((locals.var_chi_dn0 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn0)) - locals.var_vxbgmtcl_dn0), (((locals.var_chi_dn2 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn2)) - locals.var_vxbgmtcl_dn2), (((locals.var_chi_dn4 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn4)) - locals.var_vxbgmtcl_dn4), (((locals.var_chi_dn5 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn5)) - locals.var_vxbgmtcl_dn5), (((locals.var_chi_dn6 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn6)) - locals.var_vxbgmtcl_dn6), (((locals.var_chi_dn7 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn7)) - locals.var_vxbgmtcl_dn7), (((locals.var_chi_dn8 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn8)) - locals.var_vxbgmtcl_dn8), (((locals.var_chi_dn9 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn9)) - locals.var_vxbgmtcl_dn9), (((locals.var_chi_dn10 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn10)) - locals.var_vxbgmtcl_dn10), (((locals.var_chi_dn11 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn11)) - locals.var_vxbgmtcl_dn11), (((locals.var_chi_dn14 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn14)) - locals.var_vxbgmtcl_dn14),)
    } else {
        (locals.var_ps0ld, locals.var_ps0ld_dn0, locals.var_ps0ld_dn2, locals.var_ps0ld_dn4, locals.var_ps0ld_dn5, locals.var_ps0ld_dn6, locals.var_ps0ld_dn7, locals.var_ps0ld_dn8, locals.var_ps0ld_dn9, locals.var_ps0ld_dn10, locals.var_ps0ld_dn11, locals.var_ps0ld_dn14,)
    }
};
        locals.var_ps0ld = assign77560_e117586;
        locals.var_ps0ld_dn0 = assign77560_e117586_d_n0;
        locals.var_ps0ld_dn2 = assign77560_e117586_d_n2;
        locals.var_ps0ld_dn4 = assign77560_e117586_d_n4;
        locals.var_ps0ld_dn5 = assign77560_e117586_d_n5;
        locals.var_ps0ld_dn6 = assign77560_e117586_d_n6;
        locals.var_ps0ld_dn7 = assign77560_e117586_d_n7;
        locals.var_ps0ld_dn8 = assign77560_e117586_d_n8;
        locals.var_ps0ld_dn9 = assign77560_e117586_d_n9;
        locals.var_ps0ld_dn10 = assign77560_e117586_d_n10;
        locals.var_ps0ld_dn11 = assign77560_e117586_d_n11;
        locals.var_ps0ld_dn14 = assign77560_e117586_d_n14;
        locals.var_ps0ld_rv = 0.0;

        let assign77570_e117589: f64 = if locals.var_wdep_func > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1802 = assign77570_e117589;
        locals.var_guard1802_rv = 0.0;

        let (assign77580_e117602, assign77580_e117602_d_n0, assign77580_e117602_d_n2, assign77580_e117602_d_n4, assign77580_e117602_d_n5, assign77580_e117602_d_n6, assign77580_e117602_d_n7, assign77580_e117602_d_n8, assign77580_e117602_d_n9, assign77580_e117602_d_n10, assign77580_e117602_d_n11, assign77580_e117602_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1788 == 0.0)) && (locals.var_guard1801 != 0.0)) && (locals.var_guard1802 != 0.0)) {
        let assign77580_e117600: f64 = (p.p334 - locals.var_wdep_func);
        (assign77580_e117600, (-locals.var_wdep_func_dn0), (-locals.var_wdep_func_dn2), (-locals.var_wdep_func_dn4), (-locals.var_wdep_func_dn5), (-locals.var_wdep_func_dn6), (-locals.var_wdep_func_dn7), (-locals.var_wdep_func_dn8), (-locals.var_wdep_func_dn9), (-locals.var_wdep_func_dn10), (-locals.var_wdep_func_dn11), (-locals.var_wdep_func_dn14),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign77580_e117602;
        locals.var_t2_dn0 = assign77580_e117602_d_n0;
        locals.var_t2_dn2 = assign77580_e117602_d_n2;
        locals.var_t2_dn4 = assign77580_e117602_d_n4;
        locals.var_t2_dn5 = assign77580_e117602_d_n5;
        locals.var_t2_dn6 = assign77580_e117602_d_n6;
        locals.var_t2_dn7 = assign77580_e117602_d_n7;
        locals.var_t2_dn8 = assign77580_e117602_d_n8;
        locals.var_t2_dn9 = assign77580_e117602_d_n9;
        locals.var_t2_dn10 = assign77580_e117602_d_n10;
        locals.var_t2_dn11 = assign77580_e117602_d_n11;
        locals.var_t2_dn14 = assign77580_e117602_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign77590_e117627, assign77590_e117627_d_n0, assign77590_e117627_d_n2, assign77590_e117627_d_n4, assign77590_e117627_d_n5, assign77590_e117627_d_n6, assign77590_e117627_d_n7, assign77590_e117627_d_n8, assign77590_e117627_d_n9, assign77590_e117627_d_n10, assign77590_e117627_d_n11, assign77590_e117627_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1788 == 0.0)) && (locals.var_guard1801 != 0.0)) && (locals.var_guard1802 == 0.0)) {
        let assign77590_e117614: f64 = (locals.var_vdsi + p.p137);
        let assign77590_e117617: f64 = (locals.var_vdsi + p.p137);
        let assign77590_e117618: f64 = (assign77590_e117614 * assign77590_e117617);
        let assign77590_e117621: f64 = (4.0 * 0.1);
        let assign77590_e117623: f64 = (assign77590_e117621 * 0.1);
        let assign77590_e117624: f64 = (assign77590_e117618 + assign77590_e117623);
        let assign77590_e117625: f64 = (assign77590_e117624).sqrt();
        (assign77590_e117625, 0.0, 0.0, 0.0, 0.0, (((locals.var_vdsi_dn6 * assign77590_e117617) + (assign77590_e117614 * locals.var_vdsi_dn6)) / (2.0 * assign77590_e117625)), 0.0, (((locals.var_vdsi_dn8 * assign77590_e117617) + (assign77590_e117614 * locals.var_vdsi_dn8)) / (2.0 * assign77590_e117625)), 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign77590_e117627;
        locals.var_tmf2_dn0 = assign77590_e117627_d_n0;
        locals.var_tmf2_dn2 = assign77590_e117627_d_n2;
        locals.var_tmf2_dn4 = assign77590_e117627_d_n4;
        locals.var_tmf2_dn5 = assign77590_e117627_d_n5;
        locals.var_tmf2_dn6 = assign77590_e117627_d_n6;
        locals.var_tmf2_dn7 = assign77590_e117627_d_n7;
        locals.var_tmf2_dn8 = assign77590_e117627_d_n8;
        locals.var_tmf2_dn9 = assign77590_e117627_d_n9;
        locals.var_tmf2_dn10 = assign77590_e117627_d_n10;
        locals.var_tmf2_dn11 = assign77590_e117627_d_n11;
        locals.var_tmf2_dn14 = assign77590_e117627_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign77600_e117647, assign77600_e117647_d_n0, assign77600_e117647_d_n2, assign77600_e117647_d_n4, assign77600_e117647_d_n5, assign77600_e117647_d_n6, assign77600_e117647_d_n7, assign77600_e117647_d_n8, assign77600_e117647_d_n9, assign77600_e117647_d_n10, assign77600_e117647_d_n11, assign77600_e117647_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1788 == 0.0)) && (locals.var_guard1801 != 0.0)) && (locals.var_guard1802 == 0.0)) {
        let assign77600_e117641: f64 = (locals.var_vdsi + p.p137);
        let assign77600_e117643: f64 = (assign77600_e117641 / locals.var_tmf2);
        let assign77600_e117644: f64 = (1.0 + assign77600_e117643);
        let assign77600_e117645: f64 = (0.5 * assign77600_e117644);
        (assign77600_e117645, (0.5 * (-((assign77600_e117641 * locals.var_tmf2_dn0) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((assign77600_e117641 * locals.var_tmf2_dn2) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((assign77600_e117641 * locals.var_tmf2_dn4) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((assign77600_e117641 * locals.var_tmf2_dn5) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (((locals.var_vdsi_dn6 * locals.var_tmf2) - (assign77600_e117641 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (-((assign77600_e117641 * locals.var_tmf2_dn7) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (((locals.var_vdsi_dn8 * locals.var_tmf2) - (assign77600_e117641 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (-((assign77600_e117641 * locals.var_tmf2_dn9) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((assign77600_e117641 * locals.var_tmf2_dn10) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((assign77600_e117641 * locals.var_tmf2_dn11) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((assign77600_e117641 * locals.var_tmf2_dn14) / (locals.var_tmf2 * locals.var_tmf2)))),)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn11, locals.var_t9_dn14,)
    }
};
        locals.var_t9 = assign77600_e117647;
        locals.var_t9_dn0 = assign77600_e117647_d_n0;
        locals.var_t9_dn2 = assign77600_e117647_d_n2;
        locals.var_t9_dn4 = assign77600_e117647_d_n4;
        locals.var_t9_dn5 = assign77600_e117647_d_n5;
        locals.var_t9_dn6 = assign77600_e117647_d_n6;
        locals.var_t9_dn7 = assign77600_e117647_d_n7;
        locals.var_t9_dn8 = assign77600_e117647_d_n8;
        locals.var_t9_dn9 = assign77600_e117647_d_n9;
        locals.var_t9_dn10 = assign77600_e117647_d_n10;
        locals.var_t9_dn11 = assign77600_e117647_d_n11;
        locals.var_t9_dn14 = assign77600_e117647_d_n14;
        locals.var_t9_rv = 0.0;

        let (assign77610_e117665, assign77610_e117665_d_n0, assign77610_e117665_d_n2, assign77610_e117665_d_n4, assign77610_e117665_d_n5, assign77610_e117665_d_n6, assign77610_e117665_d_n7, assign77610_e117665_d_n8, assign77610_e117665_d_n9, assign77610_e117665_d_n10, assign77610_e117665_d_n11, assign77610_e117665_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1788 == 0.0)) && (locals.var_guard1801 != 0.0)) && (locals.var_guard1802 == 0.0)) {
        let assign77610_e117660: f64 = (locals.var_vdsi + p.p137);
        let assign77610_e117662: f64 = (assign77610_e117660 + locals.var_tmf2);
        let assign77610_e117663: f64 = (0.5 * assign77610_e117662);
        (assign77610_e117663, (0.5 * locals.var_tmf2_dn0), (0.5 * locals.var_tmf2_dn2), (0.5 * locals.var_tmf2_dn4), (0.5 * locals.var_tmf2_dn5), (0.5 * (locals.var_vdsi_dn6 + locals.var_tmf2_dn6)), (0.5 * locals.var_tmf2_dn7), (0.5 * (locals.var_vdsi_dn8 + locals.var_tmf2_dn8)), (0.5 * locals.var_tmf2_dn9), (0.5 * locals.var_tmf2_dn10), (0.5 * locals.var_tmf2_dn11), (0.5 * locals.var_tmf2_dn14),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign77610_e117665;
        locals.var_t2_dn0 = assign77610_e117665_d_n0;
        locals.var_t2_dn2 = assign77610_e117665_d_n2;
        locals.var_t2_dn4 = assign77610_e117665_d_n4;
        locals.var_t2_dn5 = assign77610_e117665_d_n5;
        locals.var_t2_dn6 = assign77610_e117665_d_n6;
        locals.var_t2_dn7 = assign77610_e117665_d_n7;
        locals.var_t2_dn8 = assign77610_e117665_d_n8;
        locals.var_t2_dn9 = assign77610_e117665_d_n9;
        locals.var_t2_dn10 = assign77610_e117665_d_n10;
        locals.var_t2_dn11 = assign77610_e117665_d_n11;
        locals.var_t2_dn14 = assign77610_e117665_d_n14;
        locals.var_t2_rv = 0.0;

        let assign77620_e117668: f64 = if locals.var_t2 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1803 = assign77620_e117668;
        locals.var_guard1803_rv = 0.0;

        let (assign77630_e117682, assign77630_e117682_d_n0, assign77630_e117682_d_n2, assign77630_e117682_d_n4, assign77630_e117682_d_n5, assign77630_e117682_d_n6, assign77630_e117682_d_n7, assign77630_e117682_d_n8, assign77630_e117682_d_n9, assign77630_e117682_d_n10, assign77630_e117682_d_n11, assign77630_e117682_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1788 == 0.0)) && (locals.var_guard1801 != 0.0)) && (locals.var_guard1802 == 0.0)) && (locals.var_guard1803 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign77630_e117682;
        locals.var_t2_dn0 = assign77630_e117682_d_n0;
        locals.var_t2_dn2 = assign77630_e117682_d_n2;
        locals.var_t2_dn4 = assign77630_e117682_d_n4;
        locals.var_t2_dn5 = assign77630_e117682_d_n5;
        locals.var_t2_dn6 = assign77630_e117682_d_n6;
        locals.var_t2_dn7 = assign77630_e117682_d_n7;
        locals.var_t2_dn8 = assign77630_e117682_d_n8;
        locals.var_t2_dn9 = assign77630_e117682_d_n9;
        locals.var_t2_dn10 = assign77630_e117682_d_n10;
        locals.var_t2_dn11 = assign77630_e117682_d_n11;
        locals.var_t2_dn14 = assign77630_e117682_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign77640_e117696, assign77640_e117696_d_n0, assign77640_e117696_d_n2, assign77640_e117696_d_n4, assign77640_e117696_d_n5, assign77640_e117696_d_n6, assign77640_e117696_d_n7, assign77640_e117696_d_n8, assign77640_e117696_d_n9, assign77640_e117696_d_n10, assign77640_e117696_d_n11, assign77640_e117696_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1788 == 0.0)) && (locals.var_guard1801 != 0.0)) && (locals.var_guard1802 == 0.0)) && (locals.var_guard1803 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn11, locals.var_t9_dn14,)
    }
};
        locals.var_t9 = assign77640_e117696;
        locals.var_t9_dn0 = assign77640_e117696_d_n0;
        locals.var_t9_dn2 = assign77640_e117696_d_n2;
        locals.var_t9_dn4 = assign77640_e117696_d_n4;
        locals.var_t9_dn5 = assign77640_e117696_d_n5;
        locals.var_t9_dn6 = assign77640_e117696_d_n6;
        locals.var_t9_dn7 = assign77640_e117696_d_n7;
        locals.var_t9_dn8 = assign77640_e117696_d_n8;
        locals.var_t9_dn9 = assign77640_e117696_d_n9;
        locals.var_t9_dn10 = assign77640_e117696_d_n10;
        locals.var_t9_dn11 = assign77640_e117696_d_n11;
        locals.var_t9_dn14 = assign77640_e117696_d_n14;
        locals.var_t9_rv = 0.0;

        let (assign77650_e117713, assign77650_e117713_d_n0, assign77650_e117713_d_n2, assign77650_e117713_d_n4, assign77650_e117713_d_n5, assign77650_e117713_d_n6, assign77650_e117713_d_n7, assign77650_e117713_d_n8, assign77650_e117713_d_n9, assign77650_e117713_d_n10, assign77650_e117713_d_n11, assign77650_e117713_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1788 == 0.0)) && (locals.var_guard1801 != 0.0)) && (locals.var_guard1802 == 0.0)) {
        let assign77650_e117708: f64 = (locals.var_kjunc * locals.var_t2);
        let assign77650_e117709: f64 = (assign77650_e117708).sqrt();
        let assign77650_e117711: f64 = (assign77650_e117709 * p.p432);
        (assign77650_e117711, ((((locals.var_kjunc_dn0 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn0)) / (2.0 * assign77650_e117709)) * p.p432), ((((locals.var_kjunc_dn2 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn2)) / (2.0 * assign77650_e117709)) * p.p432), ((((locals.var_kjunc_dn4 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn4)) / (2.0 * assign77650_e117709)) * p.p432), ((((locals.var_kjunc_dn5 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn5)) / (2.0 * assign77650_e117709)) * p.p432), ((((locals.var_kjunc_dn6 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn6)) / (2.0 * assign77650_e117709)) * p.p432), ((((locals.var_kjunc_dn7 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn7)) / (2.0 * assign77650_e117709)) * p.p432), ((((locals.var_kjunc_dn8 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn8)) / (2.0 * assign77650_e117709)) * p.p432), ((((locals.var_kjunc_dn9 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn9)) / (2.0 * assign77650_e117709)) * p.p432), ((((locals.var_kjunc_dn10 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn10)) / (2.0 * assign77650_e117709)) * p.p432), ((((locals.var_kjunc_dn11 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn11)) / (2.0 * assign77650_e117709)) * p.p432), ((((locals.var_kjunc_dn14 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn14)) / (2.0 * assign77650_e117709)) * p.p432),)
    } else {
        (locals.var_wjunc0, locals.var_wjunc0_dn0, locals.var_wjunc0_dn2, locals.var_wjunc0_dn4, locals.var_wjunc0_dn5, locals.var_wjunc0_dn6, locals.var_wjunc0_dn7, locals.var_wjunc0_dn8, locals.var_wjunc0_dn9, locals.var_wjunc0_dn10, locals.var_wjunc0_dn11, locals.var_wjunc0_dn14,)
    }
};
        locals.var_wjunc0 = assign77650_e117713;
        locals.var_wjunc0_dn0 = assign77650_e117713_d_n0;
        locals.var_wjunc0_dn2 = assign77650_e117713_d_n2;
        locals.var_wjunc0_dn4 = assign77650_e117713_d_n4;
        locals.var_wjunc0_dn5 = assign77650_e117713_d_n5;
        locals.var_wjunc0_dn6 = assign77650_e117713_d_n6;
        locals.var_wjunc0_dn7 = assign77650_e117713_d_n7;
        locals.var_wjunc0_dn8 = assign77650_e117713_d_n8;
        locals.var_wjunc0_dn9 = assign77650_e117713_d_n9;
        locals.var_wjunc0_dn10 = assign77650_e117713_d_n10;
        locals.var_wjunc0_dn11 = assign77650_e117713_d_n11;
        locals.var_wjunc0_dn14 = assign77650_e117713_d_n14;
        locals.var_wjunc0_rv = 0.0;

        let (assign77660_e117727, assign77660_e117727_d_n0, assign77660_e117727_d_n2, assign77660_e117727_d_n4, assign77660_e117727_d_n5, assign77660_e117727_d_n6, assign77660_e117727_d_n7, assign77660_e117727_d_n8, assign77660_e117727_d_n9, assign77660_e117727_d_n10, assign77660_e117727_d_n11, assign77660_e117727_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1788 == 0.0)) && (locals.var_guard1801 != 0.0)) && (locals.var_guard1802 == 0.0)) {
        let assign77660_e117725: f64 = (p.p334 - locals.var_wjunc0);
        (assign77660_e117725, (-locals.var_wjunc0_dn0), (-locals.var_wjunc0_dn2), (-locals.var_wjunc0_dn4), (-locals.var_wjunc0_dn5), (-locals.var_wjunc0_dn6), (-locals.var_wjunc0_dn7), (-locals.var_wjunc0_dn8), (-locals.var_wjunc0_dn9), (-locals.var_wjunc0_dn10), (-locals.var_wjunc0_dn11), (-locals.var_wjunc0_dn14),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign77660_e117727;
        locals.var_t2_dn0 = assign77660_e117727_d_n0;
        locals.var_t2_dn2 = assign77660_e117727_d_n2;
        locals.var_t2_dn4 = assign77660_e117727_d_n4;
        locals.var_t2_dn5 = assign77660_e117727_d_n5;
        locals.var_t2_dn6 = assign77660_e117727_d_n6;
        locals.var_t2_dn7 = assign77660_e117727_d_n7;
        locals.var_t2_dn8 = assign77660_e117727_d_n8;
        locals.var_t2_dn9 = assign77660_e117727_d_n9;
        locals.var_t2_dn10 = assign77660_e117727_d_n10;
        locals.var_t2_dn11 = assign77660_e117727_d_n11;
        locals.var_t2_dn14 = assign77660_e117727_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign77670_e117749, assign77670_e117749_d_n0, assign77670_e117749_d_n2, assign77670_e117749_d_n4, assign77670_e117749_d_n5, assign77670_e117749_d_n6, assign77670_e117749_d_n7, assign77670_e117749_d_n8, assign77670_e117749_d_n9, assign77670_e117749_d_n10, assign77670_e117749_d_n11, assign77670_e117749_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1788 == 0.0)) && (locals.var_guard1801 != 0.0)) {
        let assign77670_e117736: f64 = (locals.var_t2 * locals.var_t2);
        let assign77670_e117740: f64 = (p.p334 * 0.01);
        let assign77670_e117741: f64 = (4.0 * assign77670_e117740);
        let assign77670_e117744: f64 = (p.p334 * 0.01);
        let assign77670_e117745: f64 = (assign77670_e117741 * assign77670_e117744);
        let assign77670_e117746: f64 = (assign77670_e117736 + assign77670_e117745);
        let assign77670_e117747: f64 = (assign77670_e117746).sqrt();
        (assign77670_e117747, (((locals.var_t2_dn0 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn0)) / (2.0 * assign77670_e117747)), (((locals.var_t2_dn2 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn2)) / (2.0 * assign77670_e117747)), (((locals.var_t2_dn4 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn4)) / (2.0 * assign77670_e117747)), (((locals.var_t2_dn5 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn5)) / (2.0 * assign77670_e117747)), (((locals.var_t2_dn6 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn6)) / (2.0 * assign77670_e117747)), (((locals.var_t2_dn7 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn7)) / (2.0 * assign77670_e117747)), (((locals.var_t2_dn8 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn8)) / (2.0 * assign77670_e117747)), (((locals.var_t2_dn9 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn9)) / (2.0 * assign77670_e117747)), (((locals.var_t2_dn10 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn10)) / (2.0 * assign77670_e117747)), (((locals.var_t2_dn11 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn11)) / (2.0 * assign77670_e117747)), (((locals.var_t2_dn14 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn14)) / (2.0 * assign77670_e117747)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign77670_e117749;
        locals.var_tmf2_dn0 = assign77670_e117749_d_n0;
        locals.var_tmf2_dn2 = assign77670_e117749_d_n2;
        locals.var_tmf2_dn4 = assign77670_e117749_d_n4;
        locals.var_tmf2_dn5 = assign77670_e117749_d_n5;
        locals.var_tmf2_dn6 = assign77670_e117749_d_n6;
        locals.var_tmf2_dn7 = assign77670_e117749_d_n7;
        locals.var_tmf2_dn8 = assign77670_e117749_d_n8;
        locals.var_tmf2_dn9 = assign77670_e117749_d_n9;
        locals.var_tmf2_dn10 = assign77670_e117749_d_n10;
        locals.var_tmf2_dn11 = assign77670_e117749_d_n11;
        locals.var_tmf2_dn14 = assign77670_e117749_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign77680_e117764, assign77680_e117764_d_n0, assign77680_e117764_d_n2, assign77680_e117764_d_n4, assign77680_e117764_d_n5, assign77680_e117764_d_n6, assign77680_e117764_d_n7, assign77680_e117764_d_n8, assign77680_e117764_d_n9, assign77680_e117764_d_n10, assign77680_e117764_d_n11, assign77680_e117764_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1788 == 0.0)) && (locals.var_guard1801 != 0.0)) {
        let assign77680_e117760: f64 = (locals.var_t2 / locals.var_tmf2);
        let assign77680_e117761: f64 = (1.0 + assign77680_e117760);
        let assign77680_e117762: f64 = (0.5 * assign77680_e117761);
        (assign77680_e117762, (0.5 * (((locals.var_t2_dn0 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn2 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn4 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn5 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn6 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn7 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn8 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn9 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn10 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn11 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn14 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn11, locals.var_t9_dn14,)
    }
};
        locals.var_t9 = assign77680_e117764;
        locals.var_t9_dn0 = assign77680_e117764_d_n0;
        locals.var_t9_dn2 = assign77680_e117764_d_n2;
        locals.var_t9_dn4 = assign77680_e117764_d_n4;
        locals.var_t9_dn5 = assign77680_e117764_d_n5;
        locals.var_t9_dn6 = assign77680_e117764_d_n6;
        locals.var_t9_dn7 = assign77680_e117764_d_n7;
        locals.var_t9_dn8 = assign77680_e117764_d_n8;
        locals.var_t9_dn9 = assign77680_e117764_d_n9;
        locals.var_t9_dn10 = assign77680_e117764_d_n10;
        locals.var_t9_dn11 = assign77680_e117764_d_n11;
        locals.var_t9_dn14 = assign77680_e117764_d_n14;
        locals.var_t9_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_294(
        locals: &mut StampLocals,
    ) {
        let (assign77690_e117777, assign77690_e117777_d_n0, assign77690_e117777_d_n2, assign77690_e117777_d_n4, assign77690_e117777_d_n5, assign77690_e117777_d_n6, assign77690_e117777_d_n7, assign77690_e117777_d_n8, assign77690_e117777_d_n9, assign77690_e117777_d_n10, assign77690_e117777_d_n11, assign77690_e117777_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1788 == 0.0)) && (locals.var_guard1801 != 0.0)) {
        let assign77690_e117774: f64 = (locals.var_t2 + locals.var_tmf2);
        let assign77690_e117775: f64 = (0.5 * assign77690_e117774);
        (assign77690_e117775, (0.5 * (locals.var_t2_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_t2_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_t2_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_t2_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_t2_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_t2_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_t2_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_t2_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_t2_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_t2_dn11 + locals.var_tmf2_dn11)), (0.5 * (locals.var_t2_dn14 + locals.var_tmf2_dn14)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign77690_e117777;
        locals.var_t2_dn0 = assign77690_e117777_d_n0;
        locals.var_t2_dn2 = assign77690_e117777_d_n2;
        locals.var_t2_dn4 = assign77690_e117777_d_n4;
        locals.var_t2_dn5 = assign77690_e117777_d_n5;
        locals.var_t2_dn6 = assign77690_e117777_d_n6;
        locals.var_t2_dn7 = assign77690_e117777_d_n7;
        locals.var_t2_dn8 = assign77690_e117777_d_n8;
        locals.var_t2_dn9 = assign77690_e117777_d_n9;
        locals.var_t2_dn10 = assign77690_e117777_d_n10;
        locals.var_t2_dn11 = assign77690_e117777_d_n11;
        locals.var_t2_dn14 = assign77690_e117777_d_n14;
        locals.var_t2_rv = 0.0;

        let assign77700_e117780: f64 = if locals.var_t2 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1804 = assign77700_e117780;
        locals.var_guard1804_rv = 0.0;

        let (assign77710_e117791, assign77710_e117791_d_n0, assign77710_e117791_d_n2, assign77710_e117791_d_n4, assign77710_e117791_d_n5, assign77710_e117791_d_n6, assign77710_e117791_d_n7, assign77710_e117791_d_n8, assign77710_e117791_d_n9, assign77710_e117791_d_n10, assign77710_e117791_d_n11, assign77710_e117791_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1788 == 0.0)) && (locals.var_guard1801 != 0.0)) && (locals.var_guard1804 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign77710_e117791;
        locals.var_t2_dn0 = assign77710_e117791_d_n0;
        locals.var_t2_dn2 = assign77710_e117791_d_n2;
        locals.var_t2_dn4 = assign77710_e117791_d_n4;
        locals.var_t2_dn5 = assign77710_e117791_d_n5;
        locals.var_t2_dn6 = assign77710_e117791_d_n6;
        locals.var_t2_dn7 = assign77710_e117791_d_n7;
        locals.var_t2_dn8 = assign77710_e117791_d_n8;
        locals.var_t2_dn9 = assign77710_e117791_d_n9;
        locals.var_t2_dn10 = assign77710_e117791_d_n10;
        locals.var_t2_dn11 = assign77710_e117791_d_n11;
        locals.var_t2_dn14 = assign77710_e117791_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign77720_e117802, assign77720_e117802_d_n0, assign77720_e117802_d_n2, assign77720_e117802_d_n4, assign77720_e117802_d_n5, assign77720_e117802_d_n6, assign77720_e117802_d_n7, assign77720_e117802_d_n8, assign77720_e117802_d_n9, assign77720_e117802_d_n10, assign77720_e117802_d_n11, assign77720_e117802_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1788 == 0.0)) && (locals.var_guard1801 != 0.0)) && (locals.var_guard1804 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn11, locals.var_t9_dn14,)
    }
};
        locals.var_t9 = assign77720_e117802;
        locals.var_t9_dn0 = assign77720_e117802_d_n0;
        locals.var_t9_dn2 = assign77720_e117802_d_n2;
        locals.var_t9_dn4 = assign77720_e117802_d_n4;
        locals.var_t9_dn5 = assign77720_e117802_d_n5;
        locals.var_t9_dn6 = assign77720_e117802_d_n6;
        locals.var_t9_dn7 = assign77720_e117802_d_n7;
        locals.var_t9_dn8 = assign77720_e117802_d_n8;
        locals.var_t9_dn9 = assign77720_e117802_d_n9;
        locals.var_t9_dn10 = assign77720_e117802_d_n10;
        locals.var_t9_dn11 = assign77720_e117802_d_n11;
        locals.var_t9_dn14 = assign77720_e117802_d_n14;
        locals.var_t9_rv = 0.0;

        let (assign77730_e117811, assign77730_e117811_d_n0, assign77730_e117811_d_n2, assign77730_e117811_d_n4, assign77730_e117811_d_n5, assign77730_e117811_d_n6, assign77730_e117811_d_n7, assign77730_e117811_d_n8, assign77730_e117811_d_n9, assign77730_e117811_d_n10, assign77730_e117811_d_n11, assign77730_e117811_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1788 == 0.0)) && (locals.var_guard1801 != 0.0)) {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    } else {
        (locals.var_ddriftldc, locals.var_ddriftldc_dn0, locals.var_ddriftldc_dn2, locals.var_ddriftldc_dn4, locals.var_ddriftldc_dn5, locals.var_ddriftldc_dn6, locals.var_ddriftldc_dn7, locals.var_ddriftldc_dn8, locals.var_ddriftldc_dn9, locals.var_ddriftldc_dn10, locals.var_ddriftldc_dn11, locals.var_ddriftldc_dn14,)
    }
};
        locals.var_ddriftldc = assign77730_e117811;
        locals.var_ddriftldc_dn0 = assign77730_e117811_d_n0;
        locals.var_ddriftldc_dn2 = assign77730_e117811_d_n2;
        locals.var_ddriftldc_dn4 = assign77730_e117811_d_n4;
        locals.var_ddriftldc_dn5 = assign77730_e117811_d_n5;
        locals.var_ddriftldc_dn6 = assign77730_e117811_d_n6;
        locals.var_ddriftldc_dn7 = assign77730_e117811_d_n7;
        locals.var_ddriftldc_dn8 = assign77730_e117811_d_n8;
        locals.var_ddriftldc_dn9 = assign77730_e117811_d_n9;
        locals.var_ddriftldc_dn10 = assign77730_e117811_d_n10;
        locals.var_ddriftldc_dn11 = assign77730_e117811_d_n11;
        locals.var_ddriftldc_dn14 = assign77730_e117811_d_n14;
        locals.var_ddriftldc_rv = 0.0;

        let (assign77740_e117828, assign77740_e117828_d_n0, assign77740_e117828_d_n2, assign77740_e117828_d_n4, assign77740_e117828_d_n5, assign77740_e117828_d_n6, assign77740_e117828_d_n7, assign77740_e117828_d_n8, assign77740_e117828_d_n9, assign77740_e117828_d_n10, assign77740_e117828_d_n11, assign77740_e117828_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1788 == 0.0)) && (locals.var_guard1801 != 0.0)) {
        let assign77740_e117820: f64 = (locals.var_q_nsubld__blk1764 * locals.var_ddriftldc);
        let assign77740_e117822: f64 = (assign77740_e117820 * locals.var_ddriftldc);
        let assign77740_e117824: f64 = (assign77740_e117822 / 2.0);
        let assign77740_e117826: f64 = (assign77740_e117824 / 1.034943e-10);
        (assign77740_e117826, (((((locals.var_q_nsubld__blk1764 * locals.var_ddriftldc_dn0) * locals.var_ddriftldc) + (assign77740_e117820 * locals.var_ddriftldc_dn0)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk1764 * locals.var_ddriftldc_dn2) * locals.var_ddriftldc) + (assign77740_e117820 * locals.var_ddriftldc_dn2)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk1764 * locals.var_ddriftldc_dn4) * locals.var_ddriftldc) + (assign77740_e117820 * locals.var_ddriftldc_dn4)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk1764 * locals.var_ddriftldc_dn5) * locals.var_ddriftldc) + (assign77740_e117820 * locals.var_ddriftldc_dn5)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk1764 * locals.var_ddriftldc_dn6) * locals.var_ddriftldc) + (assign77740_e117820 * locals.var_ddriftldc_dn6)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk1764 * locals.var_ddriftldc_dn7) * locals.var_ddriftldc) + (assign77740_e117820 * locals.var_ddriftldc_dn7)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk1764 * locals.var_ddriftldc_dn8) * locals.var_ddriftldc) + (assign77740_e117820 * locals.var_ddriftldc_dn8)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk1764 * locals.var_ddriftldc_dn9) * locals.var_ddriftldc) + (assign77740_e117820 * locals.var_ddriftldc_dn9)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk1764 * locals.var_ddriftldc_dn10) * locals.var_ddriftldc) + (assign77740_e117820 * locals.var_ddriftldc_dn10)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk1764 * locals.var_ddriftldc_dn11) * locals.var_ddriftldc) + (assign77740_e117820 * locals.var_ddriftldc_dn11)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk1764 * locals.var_ddriftldc_dn14) * locals.var_ddriftldc) + (assign77740_e117820 * locals.var_ddriftldc_dn14)) / 2.0) / 1.034943e-10),)
    } else {
        (locals.var_dphi_sb, locals.var_dphi_sb_dn0, locals.var_dphi_sb_dn2, locals.var_dphi_sb_dn4, locals.var_dphi_sb_dn5, locals.var_dphi_sb_dn6, locals.var_dphi_sb_dn7, locals.var_dphi_sb_dn8, locals.var_dphi_sb_dn9, locals.var_dphi_sb_dn10, locals.var_dphi_sb_dn11, locals.var_dphi_sb_dn14,)
    }
};
        locals.var_dphi_sb = assign77740_e117828;
        locals.var_dphi_sb_dn0 = assign77740_e117828_d_n0;
        locals.var_dphi_sb_dn2 = assign77740_e117828_d_n2;
        locals.var_dphi_sb_dn4 = assign77740_e117828_d_n4;
        locals.var_dphi_sb_dn5 = assign77740_e117828_d_n5;
        locals.var_dphi_sb_dn6 = assign77740_e117828_d_n6;
        locals.var_dphi_sb_dn7 = assign77740_e117828_d_n7;
        locals.var_dphi_sb_dn8 = assign77740_e117828_d_n8;
        locals.var_dphi_sb_dn9 = assign77740_e117828_d_n9;
        locals.var_dphi_sb_dn10 = assign77740_e117828_d_n10;
        locals.var_dphi_sb_dn11 = assign77740_e117828_d_n11;
        locals.var_dphi_sb_dn14 = assign77740_e117828_d_n14;
        locals.var_dphi_sb_rv = 0.0;

        let (assign77750_e117842, assign77750_e117842_d_n0, assign77750_e117842_d_n2, assign77750_e117842_d_n4, assign77750_e117842_d_n5, assign77750_e117842_d_n6, assign77750_e117842_d_n7, assign77750_e117842_d_n8, assign77750_e117842_d_n9, assign77750_e117842_d_n10, assign77750_e117842_d_n11, assign77750_e117842_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1788 == 0.0)) && (locals.var_guard1801 != 0.0)) {
        let assign77750_e117837: f64 = (2.0 * locals.var_beta);
        let assign77750_e117839: f64 = (assign77750_e117837 * locals.var_dphi_sb);
        let assign77750_e117840: f64 = (assign77750_e117839).sqrt();
        (assign77750_e117840, ((((2.0 * locals.var_beta_dn0) * locals.var_dphi_sb) + (assign77750_e117837 * locals.var_dphi_sb_dn0)) / (2.0 * assign77750_e117840)), ((((2.0 * locals.var_beta_dn2) * locals.var_dphi_sb) + (assign77750_e117837 * locals.var_dphi_sb_dn2)) / (2.0 * assign77750_e117840)), ((((2.0 * locals.var_beta_dn4) * locals.var_dphi_sb) + (assign77750_e117837 * locals.var_dphi_sb_dn4)) / (2.0 * assign77750_e117840)), ((((2.0 * locals.var_beta_dn5) * locals.var_dphi_sb) + (assign77750_e117837 * locals.var_dphi_sb_dn5)) / (2.0 * assign77750_e117840)), ((((2.0 * locals.var_beta_dn6) * locals.var_dphi_sb) + (assign77750_e117837 * locals.var_dphi_sb_dn6)) / (2.0 * assign77750_e117840)), ((((2.0 * locals.var_beta_dn7) * locals.var_dphi_sb) + (assign77750_e117837 * locals.var_dphi_sb_dn7)) / (2.0 * assign77750_e117840)), ((((2.0 * locals.var_beta_dn8) * locals.var_dphi_sb) + (assign77750_e117837 * locals.var_dphi_sb_dn8)) / (2.0 * assign77750_e117840)), ((((2.0 * locals.var_beta_dn9) * locals.var_dphi_sb) + (assign77750_e117837 * locals.var_dphi_sb_dn9)) / (2.0 * assign77750_e117840)), ((((2.0 * locals.var_beta_dn10) * locals.var_dphi_sb) + (assign77750_e117837 * locals.var_dphi_sb_dn10)) / (2.0 * assign77750_e117840)), ((((2.0 * locals.var_beta_dn11) * locals.var_dphi_sb) + (assign77750_e117837 * locals.var_dphi_sb_dn11)) / (2.0 * assign77750_e117840)), ((((2.0 * locals.var_beta_dn14) * locals.var_dphi_sb) + (assign77750_e117837 * locals.var_dphi_sb_dn14)) / (2.0 * assign77750_e117840)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign77750_e117842;
        locals.var_t0_dn0 = assign77750_e117842_d_n0;
        locals.var_t0_dn2 = assign77750_e117842_d_n2;
        locals.var_t0_dn4 = assign77750_e117842_d_n4;
        locals.var_t0_dn5 = assign77750_e117842_d_n5;
        locals.var_t0_dn6 = assign77750_e117842_d_n6;
        locals.var_t0_dn7 = assign77750_e117842_d_n7;
        locals.var_t0_dn8 = assign77750_e117842_d_n8;
        locals.var_t0_dn9 = assign77750_e117842_d_n9;
        locals.var_t0_dn10 = assign77750_e117842_d_n10;
        locals.var_t0_dn11 = assign77750_e117842_d_n11;
        locals.var_t0_dn14 = assign77750_e117842_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign77760_e117858, assign77760_e117858_d_n0, assign77760_e117858_d_n2, assign77760_e117858_d_n4, assign77760_e117858_d_n5, assign77760_e117858_d_n6, assign77760_e117858_d_n7, assign77760_e117858_d_n8, assign77760_e117858_d_n9, assign77760_e117858_d_n10, assign77760_e117858_d_n11, assign77760_e117858_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1788 == 0.0)) && (locals.var_guard1801 != 0.0)) {
        let assign77760_e117850: f64 = { let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign77760_e117852: f64 = (-locals.var_t0);
        let assign77760_e117853: f64 = { let limited_exp_arg = assign77760_e117852; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign77760_e117854: f64 = (assign77760_e117850 + assign77760_e117853);
        let assign77760_e117856: f64 = (assign77760_e117854 / 2.0);
        (assign77760_e117856, ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn0) + ({ let limited_exp_arg = assign77760_e117852; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn0))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn2) + ({ let limited_exp_arg = assign77760_e117852; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn2))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn4) + ({ let limited_exp_arg = assign77760_e117852; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn4))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn5) + ({ let limited_exp_arg = assign77760_e117852; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn5))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn6) + ({ let limited_exp_arg = assign77760_e117852; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn6))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn7) + ({ let limited_exp_arg = assign77760_e117852; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn7))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn8) + ({ let limited_exp_arg = assign77760_e117852; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn8))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn9) + ({ let limited_exp_arg = assign77760_e117852; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn9))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn10) + ({ let limited_exp_arg = assign77760_e117852; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn10))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn11) + ({ let limited_exp_arg = assign77760_e117852; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn11))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn14) + ({ let limited_exp_arg = assign77760_e117852; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn14))) / 2.0),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign77760_e117858;
        locals.var_t1_dn0 = assign77760_e117858_d_n0;
        locals.var_t1_dn2 = assign77760_e117858_d_n2;
        locals.var_t1_dn4 = assign77760_e117858_d_n4;
        locals.var_t1_dn5 = assign77760_e117858_d_n5;
        locals.var_t1_dn6 = assign77760_e117858_d_n6;
        locals.var_t1_dn7 = assign77760_e117858_d_n7;
        locals.var_t1_dn8 = assign77760_e117858_d_n8;
        locals.var_t1_dn9 = assign77760_e117858_d_n9;
        locals.var_t1_dn10 = assign77760_e117858_d_n10;
        locals.var_t1_dn11 = assign77760_e117858_d_n11;
        locals.var_t1_dn14 = assign77760_e117858_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign77770_e117870, assign77770_e117870_d_n0, assign77770_e117870_d_n2, assign77770_e117870_d_n4, assign77770_e117870_d_n5, assign77770_e117870_d_n6, assign77770_e117870_d_n7, assign77770_e117870_d_n8, assign77770_e117870_d_n9, assign77770_e117870_d_n10, assign77770_e117870_d_n11, assign77770_e117870_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1788 == 0.0)) && (locals.var_guard1801 != 0.0)) {
        let assign77770_e117866: f64 = (locals.var_t1).ln();
        let assign77770_e117868: f64 = (assign77770_e117866 / locals.var_dphi_sb);
        (assign77770_e117868, ((((locals.var_t1_dn0 / locals.var_t1) * locals.var_dphi_sb) - (assign77770_e117866 * locals.var_dphi_sb_dn0)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn2 / locals.var_t1) * locals.var_dphi_sb) - (assign77770_e117866 * locals.var_dphi_sb_dn2)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn4 / locals.var_t1) * locals.var_dphi_sb) - (assign77770_e117866 * locals.var_dphi_sb_dn4)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn5 / locals.var_t1) * locals.var_dphi_sb) - (assign77770_e117866 * locals.var_dphi_sb_dn5)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn6 / locals.var_t1) * locals.var_dphi_sb) - (assign77770_e117866 * locals.var_dphi_sb_dn6)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn7 / locals.var_t1) * locals.var_dphi_sb) - (assign77770_e117866 * locals.var_dphi_sb_dn7)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn8 / locals.var_t1) * locals.var_dphi_sb) - (assign77770_e117866 * locals.var_dphi_sb_dn8)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn9 / locals.var_t1) * locals.var_dphi_sb) - (assign77770_e117866 * locals.var_dphi_sb_dn9)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn10 / locals.var_t1) * locals.var_dphi_sb) - (assign77770_e117866 * locals.var_dphi_sb_dn10)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn11 / locals.var_t1) * locals.var_dphi_sb) - (assign77770_e117866 * locals.var_dphi_sb_dn11)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn14 / locals.var_t1) * locals.var_dphi_sb) - (assign77770_e117866 * locals.var_dphi_sb_dn14)) / (locals.var_dphi_sb * locals.var_dphi_sb)),)
    } else {
        (locals.var_c_sb, locals.var_c_sb_dn0, locals.var_c_sb_dn2, locals.var_c_sb_dn4, locals.var_c_sb_dn5, locals.var_c_sb_dn6, locals.var_c_sb_dn7, locals.var_c_sb_dn8, locals.var_c_sb_dn9, locals.var_c_sb_dn10, locals.var_c_sb_dn11, locals.var_c_sb_dn14,)
    }
};
        locals.var_c_sb = assign77770_e117870;
        locals.var_c_sb_dn0 = assign77770_e117870_d_n0;
        locals.var_c_sb_dn2 = assign77770_e117870_d_n2;
        locals.var_c_sb_dn4 = assign77770_e117870_d_n4;
        locals.var_c_sb_dn5 = assign77770_e117870_d_n5;
        locals.var_c_sb_dn6 = assign77770_e117870_d_n6;
        locals.var_c_sb_dn7 = assign77770_e117870_d_n7;
        locals.var_c_sb_dn8 = assign77770_e117870_d_n8;
        locals.var_c_sb_dn9 = assign77770_e117870_d_n9;
        locals.var_c_sb_dn10 = assign77770_e117870_d_n10;
        locals.var_c_sb_dn11 = assign77770_e117870_d_n11;
        locals.var_c_sb_dn14 = assign77770_e117870_d_n14;
        locals.var_c_sb_rv = 0.0;

        let (assign77780_e117881, assign77780_e117881_d_n0, assign77780_e117881_d_n2, assign77780_e117881_d_n4, assign77780_e117881_d_n5, assign77780_e117881_d_n6, assign77780_e117881_d_n7, assign77780_e117881_d_n8, assign77780_e117881_d_n9, assign77780_e117881_d_n10, assign77780_e117881_d_n11, assign77780_e117881_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1788 == 0.0)) && (locals.var_guard1801 != 0.0)) {
        let assign77780_e117879: f64 = (locals.var_ps0ld + locals.var_vxbgmtcl);
        (assign77780_e117879, (locals.var_ps0ld_dn0 + locals.var_vxbgmtcl_dn0), (locals.var_ps0ld_dn2 + locals.var_vxbgmtcl_dn2), (locals.var_ps0ld_dn4 + locals.var_vxbgmtcl_dn4), (locals.var_ps0ld_dn5 + locals.var_vxbgmtcl_dn5), (locals.var_ps0ld_dn6 + locals.var_vxbgmtcl_dn6), (locals.var_ps0ld_dn7 + locals.var_vxbgmtcl_dn7), (locals.var_ps0ld_dn8 + locals.var_vxbgmtcl_dn8), (locals.var_ps0ld_dn9 + locals.var_vxbgmtcl_dn9), (locals.var_ps0ld_dn10 + locals.var_vxbgmtcl_dn10), (locals.var_ps0ld_dn11 + locals.var_vxbgmtcl_dn11), (locals.var_ps0ld_dn14 + locals.var_vxbgmtcl_dn14),)
    } else {
        (locals.var_ps0ld_vxb, locals.var_ps0ld_vxb_dn0, locals.var_ps0ld_vxb_dn2, locals.var_ps0ld_vxb_dn4, locals.var_ps0ld_vxb_dn5, locals.var_ps0ld_vxb_dn6, locals.var_ps0ld_vxb_dn7, locals.var_ps0ld_vxb_dn8, locals.var_ps0ld_vxb_dn9, locals.var_ps0ld_vxb_dn10, locals.var_ps0ld_vxb_dn11, locals.var_ps0ld_vxb_dn14,)
    }
};
        locals.var_ps0ld_vxb = assign77780_e117881;
        locals.var_ps0ld_vxb_dn0 = assign77780_e117881_d_n0;
        locals.var_ps0ld_vxb_dn2 = assign77780_e117881_d_n2;
        locals.var_ps0ld_vxb_dn4 = assign77780_e117881_d_n4;
        locals.var_ps0ld_vxb_dn5 = assign77780_e117881_d_n5;
        locals.var_ps0ld_vxb_dn6 = assign77780_e117881_d_n6;
        locals.var_ps0ld_vxb_dn7 = assign77780_e117881_d_n7;
        locals.var_ps0ld_vxb_dn8 = assign77780_e117881_d_n8;
        locals.var_ps0ld_vxb_dn9 = assign77780_e117881_d_n9;
        locals.var_ps0ld_vxb_dn10 = assign77780_e117881_d_n10;
        locals.var_ps0ld_vxb_dn11 = assign77780_e117881_d_n11;
        locals.var_ps0ld_vxb_dn14 = assign77780_e117881_d_n14;
        locals.var_ps0ld_vxb_rv = 0.0;

        let (assign77790_e117894, assign77790_e117894_d_n0, assign77790_e117894_d_n2, assign77790_e117894_d_n4, assign77790_e117894_d_n5, assign77790_e117894_d_n6, assign77790_e117894_d_n7, assign77790_e117894_d_n8, assign77790_e117894_d_n9, assign77790_e117894_d_n10, assign77790_e117894_d_n11, assign77790_e117894_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1788 == 0.0)) && (locals.var_guard1801 != 0.0)) {
        let assign77790_e117891: f64 = (locals.var_ps0ld_vxb - locals.var_dphi_sb);
        let assign77790_e117892: f64 = (locals.var_c_sb * assign77790_e117891);
        (assign77790_e117892, ((locals.var_c_sb_dn0 * assign77790_e117891) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn0 - locals.var_dphi_sb_dn0))), ((locals.var_c_sb_dn2 * assign77790_e117891) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn2 - locals.var_dphi_sb_dn2))), ((locals.var_c_sb_dn4 * assign77790_e117891) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn4 - locals.var_dphi_sb_dn4))), ((locals.var_c_sb_dn5 * assign77790_e117891) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn5 - locals.var_dphi_sb_dn5))), ((locals.var_c_sb_dn6 * assign77790_e117891) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn6 - locals.var_dphi_sb_dn6))), ((locals.var_c_sb_dn7 * assign77790_e117891) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn7 - locals.var_dphi_sb_dn7))), ((locals.var_c_sb_dn8 * assign77790_e117891) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn8 - locals.var_dphi_sb_dn8))), ((locals.var_c_sb_dn9 * assign77790_e117891) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn9 - locals.var_dphi_sb_dn9))), ((locals.var_c_sb_dn10 * assign77790_e117891) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn10 - locals.var_dphi_sb_dn10))), ((locals.var_c_sb_dn11 * assign77790_e117891) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn11 - locals.var_dphi_sb_dn11))), ((locals.var_c_sb_dn14 * assign77790_e117891) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn14 - locals.var_dphi_sb_dn14))),)
    } else {
        (locals.var_ty, locals.var_ty_dn0, locals.var_ty_dn2, locals.var_ty_dn4, locals.var_ty_dn5, locals.var_ty_dn6, locals.var_ty_dn7, locals.var_ty_dn8, locals.var_ty_dn9, locals.var_ty_dn10, locals.var_ty_dn11, locals.var_ty_dn14,)
    }
};
        locals.var_ty = assign77790_e117894;
        locals.var_ty_dn0 = assign77790_e117894_d_n0;
        locals.var_ty_dn2 = assign77790_e117894_d_n2;
        locals.var_ty_dn4 = assign77790_e117894_d_n4;
        locals.var_ty_dn5 = assign77790_e117894_d_n5;
        locals.var_ty_dn6 = assign77790_e117894_d_n6;
        locals.var_ty_dn7 = assign77790_e117894_d_n7;
        locals.var_ty_dn8 = assign77790_e117894_d_n8;
        locals.var_ty_dn9 = assign77790_e117894_d_n9;
        locals.var_ty_dn10 = assign77790_e117894_d_n10;
        locals.var_ty_dn11 = assign77790_e117894_d_n11;
        locals.var_ty_dn14 = assign77790_e117894_d_n14;
        locals.var_ty_rv = 0.0;

        let assign77800_e117897: f64 = if locals.var_ty < 60.0 { 1.0 } else { 0.0 };
        locals.var_guard1805 = assign77800_e117897;
        locals.var_guard1805_rv = 0.0;

        let (assign77810_e117909, assign77810_e117909_d_n0, assign77810_e117909_d_n2, assign77810_e117909_d_n4, assign77810_e117909_d_n5, assign77810_e117909_d_n6, assign77810_e117909_d_n7, assign77810_e117909_d_n8, assign77810_e117909_d_n9, assign77810_e117909_d_n10, assign77810_e117909_d_n11, assign77810_e117909_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1788 == 0.0)) && (locals.var_guard1801 != 0.0)) && (locals.var_guard1805 != 0.0)) {
        let assign77810_e117907: f64 = (locals.var_ty).exp();
        (assign77810_e117907, (assign77810_e117907 * locals.var_ty_dn0), (assign77810_e117907 * locals.var_ty_dn2), (assign77810_e117907 * locals.var_ty_dn4), (assign77810_e117907 * locals.var_ty_dn5), (assign77810_e117907 * locals.var_ty_dn6), (assign77810_e117907 * locals.var_ty_dn7), (assign77810_e117907 * locals.var_ty_dn8), (assign77810_e117907 * locals.var_ty_dn9), (assign77810_e117907 * locals.var_ty_dn10), (assign77810_e117907 * locals.var_ty_dn11), (assign77810_e117907 * locals.var_ty_dn14),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign77810_e117909;
        locals.var_t1_dn0 = assign77810_e117909_d_n0;
        locals.var_t1_dn2 = assign77810_e117909_d_n2;
        locals.var_t1_dn4 = assign77810_e117909_d_n4;
        locals.var_t1_dn5 = assign77810_e117909_d_n5;
        locals.var_t1_dn6 = assign77810_e117909_d_n6;
        locals.var_t1_dn7 = assign77810_e117909_d_n7;
        locals.var_t1_dn8 = assign77810_e117909_d_n8;
        locals.var_t1_dn9 = assign77810_e117909_d_n9;
        locals.var_t1_dn10 = assign77810_e117909_d_n10;
        locals.var_t1_dn11 = assign77810_e117909_d_n11;
        locals.var_t1_dn14 = assign77810_e117909_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign77820_e117924, assign77820_e117924_d_n0, assign77820_e117924_d_n2, assign77820_e117924_d_n4, assign77820_e117924_d_n5, assign77820_e117924_d_n6, assign77820_e117924_d_n7, assign77820_e117924_d_n8, assign77820_e117924_d_n9, assign77820_e117924_d_n10, assign77820_e117924_d_n11, assign77820_e117924_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1788 == 0.0)) && (locals.var_guard1801 != 0.0)) && (locals.var_guard1805 != 0.0)) {
        let assign77820_e117919: f64 = (-locals.var_c_sb);
        let assign77820_e117921: f64 = (assign77820_e117919 * locals.var_dphi_sb);
        let assign77820_e117922: f64 = (assign77820_e117921).exp();
        (assign77820_e117922, (assign77820_e117922 * (((-locals.var_c_sb_dn0) * locals.var_dphi_sb) + (assign77820_e117919 * locals.var_dphi_sb_dn0))), (assign77820_e117922 * (((-locals.var_c_sb_dn2) * locals.var_dphi_sb) + (assign77820_e117919 * locals.var_dphi_sb_dn2))), (assign77820_e117922 * (((-locals.var_c_sb_dn4) * locals.var_dphi_sb) + (assign77820_e117919 * locals.var_dphi_sb_dn4))), (assign77820_e117922 * (((-locals.var_c_sb_dn5) * locals.var_dphi_sb) + (assign77820_e117919 * locals.var_dphi_sb_dn5))), (assign77820_e117922 * (((-locals.var_c_sb_dn6) * locals.var_dphi_sb) + (assign77820_e117919 * locals.var_dphi_sb_dn6))), (assign77820_e117922 * (((-locals.var_c_sb_dn7) * locals.var_dphi_sb) + (assign77820_e117919 * locals.var_dphi_sb_dn7))), (assign77820_e117922 * (((-locals.var_c_sb_dn8) * locals.var_dphi_sb) + (assign77820_e117919 * locals.var_dphi_sb_dn8))), (assign77820_e117922 * (((-locals.var_c_sb_dn9) * locals.var_dphi_sb) + (assign77820_e117919 * locals.var_dphi_sb_dn9))), (assign77820_e117922 * (((-locals.var_c_sb_dn10) * locals.var_dphi_sb) + (assign77820_e117919 * locals.var_dphi_sb_dn10))), (assign77820_e117922 * (((-locals.var_c_sb_dn11) * locals.var_dphi_sb) + (assign77820_e117919 * locals.var_dphi_sb_dn11))), (assign77820_e117922 * (((-locals.var_c_sb_dn14) * locals.var_dphi_sb) + (assign77820_e117919 * locals.var_dphi_sb_dn14))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign77820_e117924;
        locals.var_t0_dn0 = assign77820_e117924_d_n0;
        locals.var_t0_dn2 = assign77820_e117924_d_n2;
        locals.var_t0_dn4 = assign77820_e117924_d_n4;
        locals.var_t0_dn5 = assign77820_e117924_d_n5;
        locals.var_t0_dn6 = assign77820_e117924_d_n6;
        locals.var_t0_dn7 = assign77820_e117924_d_n7;
        locals.var_t0_dn8 = assign77820_e117924_d_n8;
        locals.var_t0_dn9 = assign77820_e117924_d_n9;
        locals.var_t0_dn10 = assign77820_e117924_d_n10;
        locals.var_t0_dn11 = assign77820_e117924_d_n11;
        locals.var_t0_dn14 = assign77820_e117924_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign77830_e117937, assign77830_e117937_d_n0, assign77830_e117937_d_n2, assign77830_e117937_d_n4, assign77830_e117937_d_n5, assign77830_e117937_d_n6, assign77830_e117937_d_n7, assign77830_e117937_d_n8, assign77830_e117937_d_n9, assign77830_e117937_d_n10, assign77830_e117937_d_n11, assign77830_e117937_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1788 == 0.0)) && (locals.var_guard1801 != 0.0)) && (locals.var_guard1805 != 0.0)) {
        let assign77830_e117935: f64 = (locals.var_t1 - locals.var_t0);
        (assign77830_e117935, (locals.var_t1_dn0 - locals.var_t0_dn0), (locals.var_t1_dn2 - locals.var_t0_dn2), (locals.var_t1_dn4 - locals.var_t0_dn4), (locals.var_t1_dn5 - locals.var_t0_dn5), (locals.var_t1_dn6 - locals.var_t0_dn6), (locals.var_t1_dn7 - locals.var_t0_dn7), (locals.var_t1_dn8 - locals.var_t0_dn8), (locals.var_t1_dn9 - locals.var_t0_dn9), (locals.var_t1_dn10 - locals.var_t0_dn10), (locals.var_t1_dn11 - locals.var_t0_dn11), (locals.var_t1_dn14 - locals.var_t0_dn14),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign77830_e117937;
        locals.var_t2_dn0 = assign77830_e117937_d_n0;
        locals.var_t2_dn2 = assign77830_e117937_d_n2;
        locals.var_t2_dn4 = assign77830_e117937_d_n4;
        locals.var_t2_dn5 = assign77830_e117937_d_n5;
        locals.var_t2_dn6 = assign77830_e117937_d_n6;
        locals.var_t2_dn7 = assign77830_e117937_d_n7;
        locals.var_t2_dn8 = assign77830_e117937_d_n8;
        locals.var_t2_dn9 = assign77830_e117937_d_n9;
        locals.var_t2_dn10 = assign77830_e117937_d_n10;
        locals.var_t2_dn11 = assign77830_e117937_d_n11;
        locals.var_t2_dn14 = assign77830_e117937_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign77840_e117953, assign77840_e117953_d_n0, assign77840_e117953_d_n2, assign77840_e117953_d_n4, assign77840_e117953_d_n5, assign77840_e117953_d_n6, assign77840_e117953_d_n7, assign77840_e117953_d_n8, assign77840_e117953_d_n9, assign77840_e117953_d_n10, assign77840_e117953_d_n11, assign77840_e117953_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1788 == 0.0)) && (locals.var_guard1801 != 0.0)) && (locals.var_guard1805 != 0.0)) {
        let assign77840_e117948: f64 = (1.0 + locals.var_t2);
        let assign77840_e117949: f64 = (assign77840_e117948).ln();
        let assign77840_e117951: f64 = (assign77840_e117949 / locals.var_c_sb);
        (assign77840_e117951, ((((locals.var_t2_dn0 / assign77840_e117948) * locals.var_c_sb) - (assign77840_e117949 * locals.var_c_sb_dn0)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn2 / assign77840_e117948) * locals.var_c_sb) - (assign77840_e117949 * locals.var_c_sb_dn2)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn4 / assign77840_e117948) * locals.var_c_sb) - (assign77840_e117949 * locals.var_c_sb_dn4)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn5 / assign77840_e117948) * locals.var_c_sb) - (assign77840_e117949 * locals.var_c_sb_dn5)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn6 / assign77840_e117948) * locals.var_c_sb) - (assign77840_e117949 * locals.var_c_sb_dn6)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn7 / assign77840_e117948) * locals.var_c_sb) - (assign77840_e117949 * locals.var_c_sb_dn7)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn8 / assign77840_e117948) * locals.var_c_sb) - (assign77840_e117949 * locals.var_c_sb_dn8)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn9 / assign77840_e117948) * locals.var_c_sb) - (assign77840_e117949 * locals.var_c_sb_dn9)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn10 / assign77840_e117948) * locals.var_c_sb) - (assign77840_e117949 * locals.var_c_sb_dn10)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn11 / assign77840_e117948) * locals.var_c_sb) - (assign77840_e117949 * locals.var_c_sb_dn11)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn14 / assign77840_e117948) * locals.var_c_sb) - (assign77840_e117949 * locals.var_c_sb_dn14)) / (locals.var_c_sb * locals.var_c_sb)),)
    } else {
        (locals.var_phi_b, locals.var_phi_b_dn0, locals.var_phi_b_dn2, locals.var_phi_b_dn4, locals.var_phi_b_dn5, locals.var_phi_b_dn6, locals.var_phi_b_dn7, locals.var_phi_b_dn8, locals.var_phi_b_dn9, locals.var_phi_b_dn10, locals.var_phi_b_dn11, locals.var_phi_b_dn14,)
    }
};
        locals.var_phi_b = assign77840_e117953;
        locals.var_phi_b_dn0 = assign77840_e117953_d_n0;
        locals.var_phi_b_dn2 = assign77840_e117953_d_n2;
        locals.var_phi_b_dn4 = assign77840_e117953_d_n4;
        locals.var_phi_b_dn5 = assign77840_e117953_d_n5;
        locals.var_phi_b_dn6 = assign77840_e117953_d_n6;
        locals.var_phi_b_dn7 = assign77840_e117953_d_n7;
        locals.var_phi_b_dn8 = assign77840_e117953_d_n8;
        locals.var_phi_b_dn9 = assign77840_e117953_d_n9;
        locals.var_phi_b_dn10 = assign77840_e117953_d_n10;
        locals.var_phi_b_dn11 = assign77840_e117953_d_n11;
        locals.var_phi_b_dn14 = assign77840_e117953_d_n14;
        locals.var_phi_b_rv = 0.0;

        let (assign77850_e117967, assign77850_e117967_d_n0, assign77850_e117967_d_n2, assign77850_e117967_d_n4, assign77850_e117967_d_n5, assign77850_e117967_d_n6, assign77850_e117967_d_n7, assign77850_e117967_d_n8, assign77850_e117967_d_n9, assign77850_e117967_d_n10, assign77850_e117967_d_n11, assign77850_e117967_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1788 == 0.0)) && (locals.var_guard1801 != 0.0)) && (locals.var_guard1805 == 0.0)) {
        let assign77850_e117965: f64 = (locals.var_ps0ld_vxb - locals.var_dphi_sb);
        (assign77850_e117965, (locals.var_ps0ld_vxb_dn0 - locals.var_dphi_sb_dn0), (locals.var_ps0ld_vxb_dn2 - locals.var_dphi_sb_dn2), (locals.var_ps0ld_vxb_dn4 - locals.var_dphi_sb_dn4), (locals.var_ps0ld_vxb_dn5 - locals.var_dphi_sb_dn5), (locals.var_ps0ld_vxb_dn6 - locals.var_dphi_sb_dn6), (locals.var_ps0ld_vxb_dn7 - locals.var_dphi_sb_dn7), (locals.var_ps0ld_vxb_dn8 - locals.var_dphi_sb_dn8), (locals.var_ps0ld_vxb_dn9 - locals.var_dphi_sb_dn9), (locals.var_ps0ld_vxb_dn10 - locals.var_dphi_sb_dn10), (locals.var_ps0ld_vxb_dn11 - locals.var_dphi_sb_dn11), (locals.var_ps0ld_vxb_dn14 - locals.var_dphi_sb_dn14),)
    } else {
        (locals.var_phi_b, locals.var_phi_b_dn0, locals.var_phi_b_dn2, locals.var_phi_b_dn4, locals.var_phi_b_dn5, locals.var_phi_b_dn6, locals.var_phi_b_dn7, locals.var_phi_b_dn8, locals.var_phi_b_dn9, locals.var_phi_b_dn10, locals.var_phi_b_dn11, locals.var_phi_b_dn14,)
    }
};
        locals.var_phi_b = assign77850_e117967;
        locals.var_phi_b_dn0 = assign77850_e117967_d_n0;
        locals.var_phi_b_dn2 = assign77850_e117967_d_n2;
        locals.var_phi_b_dn4 = assign77850_e117967_d_n4;
        locals.var_phi_b_dn5 = assign77850_e117967_d_n5;
        locals.var_phi_b_dn6 = assign77850_e117967_d_n6;
        locals.var_phi_b_dn7 = assign77850_e117967_d_n7;
        locals.var_phi_b_dn8 = assign77850_e117967_d_n8;
        locals.var_phi_b_dn9 = assign77850_e117967_d_n9;
        locals.var_phi_b_dn10 = assign77850_e117967_d_n10;
        locals.var_phi_b_dn11 = assign77850_e117967_d_n11;
        locals.var_phi_b_dn14 = assign77850_e117967_d_n14;
        locals.var_phi_b_rv = 0.0;

        let (assign77860_e117978, assign77860_e117978_d_n0, assign77860_e117978_d_n2, assign77860_e117978_d_n4, assign77860_e117978_d_n5, assign77860_e117978_d_n6, assign77860_e117978_d_n7, assign77860_e117978_d_n8, assign77860_e117978_d_n9, assign77860_e117978_d_n10, assign77860_e117978_d_n11, assign77860_e117978_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1788 == 0.0)) && (locals.var_guard1801 != 0.0)) {
        let assign77860_e117976: f64 = (locals.var_beta * locals.var_phi_b);
        (assign77860_e117976, ((locals.var_beta_dn0 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn0)), ((locals.var_beta_dn2 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn2)), ((locals.var_beta_dn4 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn4)), ((locals.var_beta_dn5 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn5)), ((locals.var_beta_dn6 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn6)), ((locals.var_beta_dn7 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn7)), ((locals.var_beta_dn8 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn8)), ((locals.var_beta_dn9 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn9)), ((locals.var_beta_dn10 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn10)), ((locals.var_beta_dn11 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn11)), ((locals.var_beta_dn14 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn14)),)
    } else {
        (locals.var_chib, locals.var_chib_dn0, locals.var_chib_dn2, locals.var_chib_dn4, locals.var_chib_dn5, locals.var_chib_dn6, locals.var_chib_dn7, locals.var_chib_dn8, locals.var_chib_dn9, locals.var_chib_dn10, locals.var_chib_dn11, locals.var_chib_dn14,)
    }
};
        locals.var_chib = assign77860_e117978;
        locals.var_chib_dn0 = assign77860_e117978_d_n0;
        locals.var_chib_dn2 = assign77860_e117978_d_n2;
        locals.var_chib_dn4 = assign77860_e117978_d_n4;
        locals.var_chib_dn5 = assign77860_e117978_d_n5;
        locals.var_chib_dn6 = assign77860_e117978_d_n6;
        locals.var_chib_dn7 = assign77860_e117978_d_n7;
        locals.var_chib_dn8 = assign77860_e117978_d_n8;
        locals.var_chib_dn9 = assign77860_e117978_d_n9;
        locals.var_chib_dn10 = assign77860_e117978_d_n10;
        locals.var_chib_dn11 = assign77860_e117978_d_n11;
        locals.var_chib_dn14 = assign77860_e117978_d_n14;
        locals.var_chib_rv = 0.0;

        let assign77870_e117982: f64 = (locals.var_chi / 100.0);
        let assign77870_e117987: f64 = if ((locals.var_chib > assign77870_e117982) && (locals.var_chib > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1806 = assign77870_e117987;
        locals.var_guard1806_rv = 0.0;

        let (assign77880_e118000,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1788 == 0.0)) && (locals.var_guard1801 != 0.0)) && (locals.var_guard1806 != 0.0)) {
        let assign77880_e117998: f64 = (locals.var_flg_fd_mode__blk1770 + 1.0);
        (assign77880_e117998,)
    } else {
        (locals.var_flg_fd_mode__blk1770,)
    }
};
        locals.var_flg_fd_mode__blk1770 = assign77880_e118000;
        locals.var_flg_fd_mode__blk1770_rv = 0.0;

        let (assign77890_e118011, assign77890_e118011_d_n0, assign77890_e118011_d_n2, assign77890_e118011_d_n4, assign77890_e118011_d_n5, assign77890_e118011_d_n6, assign77890_e118011_d_n7, assign77890_e118011_d_n8, assign77890_e118011_d_n9, assign77890_e118011_d_n10, assign77890_e118011_d_n11, assign77890_e118011_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1788 == 0.0)) && (locals.var_guard1801 != 0.0)) && (locals.var_guard1806 != 0.0)) {
        (locals.var_chi_1, locals.var_chi_1_dn0, locals.var_chi_1_dn2, locals.var_chi_1_dn4, locals.var_chi_1_dn5, locals.var_chi_1_dn6, locals.var_chi_1_dn7, locals.var_chi_1_dn8, locals.var_chi_1_dn9, locals.var_chi_1_dn10, locals.var_chi_1_dn11, locals.var_chi_1_dn14,)
    } else {
        (locals.var_chi, locals.var_chi_dn0, locals.var_chi_dn2, locals.var_chi_dn4, locals.var_chi_dn5, locals.var_chi_dn6, locals.var_chi_dn7, locals.var_chi_dn8, locals.var_chi_dn9, locals.var_chi_dn10, locals.var_chi_dn11, locals.var_chi_dn14,)
    }
};
        locals.var_chi = assign77890_e118011;
        locals.var_chi_dn0 = assign77890_e118011_d_n0;
        locals.var_chi_dn2 = assign77890_e118011_d_n2;
        locals.var_chi_dn4 = assign77890_e118011_d_n4;
        locals.var_chi_dn5 = assign77890_e118011_d_n5;
        locals.var_chi_dn6 = assign77890_e118011_d_n6;
        locals.var_chi_dn7 = assign77890_e118011_d_n7;
        locals.var_chi_dn8 = assign77890_e118011_d_n8;
        locals.var_chi_dn9 = assign77890_e118011_d_n9;
        locals.var_chi_dn10 = assign77890_e118011_d_n10;
        locals.var_chi_dn11 = assign77890_e118011_d_n11;
        locals.var_chi_dn14 = assign77890_e118011_d_n14;
        locals.var_chi_rv = 0.0;

        let (assign77900_e118022, assign77900_e118022_d_n0, assign77900_e118022_d_n2, assign77900_e118022_d_n4, assign77900_e118022_d_n5, assign77900_e118022_d_n6, assign77900_e118022_d_n7, assign77900_e118022_d_n8, assign77900_e118022_d_n9, assign77900_e118022_d_n10, assign77900_e118022_d_n11, assign77900_e118022_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1788 == 0.0)) {
        let assign77900_e118018: f64 = (locals.var_chi * locals.var_beta_inv);
        let assign77900_e118020: f64 = (assign77900_e118018 - locals.var_vxbgmtcl);
        (assign77900_e118020, (((locals.var_chi_dn0 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn0)) - locals.var_vxbgmtcl_dn0), (((locals.var_chi_dn2 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn2)) - locals.var_vxbgmtcl_dn2), (((locals.var_chi_dn4 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn4)) - locals.var_vxbgmtcl_dn4), (((locals.var_chi_dn5 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn5)) - locals.var_vxbgmtcl_dn5), (((locals.var_chi_dn6 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn6)) - locals.var_vxbgmtcl_dn6), (((locals.var_chi_dn7 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn7)) - locals.var_vxbgmtcl_dn7), (((locals.var_chi_dn8 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn8)) - locals.var_vxbgmtcl_dn8), (((locals.var_chi_dn9 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn9)) - locals.var_vxbgmtcl_dn9), (((locals.var_chi_dn10 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn10)) - locals.var_vxbgmtcl_dn10), (((locals.var_chi_dn11 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn11)) - locals.var_vxbgmtcl_dn11), (((locals.var_chi_dn14 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn14)) - locals.var_vxbgmtcl_dn14),)
    } else {
        (locals.var_ps0ld, locals.var_ps0ld_dn0, locals.var_ps0ld_dn2, locals.var_ps0ld_dn4, locals.var_ps0ld_dn5, locals.var_ps0ld_dn6, locals.var_ps0ld_dn7, locals.var_ps0ld_dn8, locals.var_ps0ld_dn9, locals.var_ps0ld_dn10, locals.var_ps0ld_dn11, locals.var_ps0ld_dn14,)
    }
};
        locals.var_ps0ld = assign77900_e118022;
        locals.var_ps0ld_dn0 = assign77900_e118022_d_n0;
        locals.var_ps0ld_dn2 = assign77900_e118022_d_n2;
        locals.var_ps0ld_dn4 = assign77900_e118022_d_n4;
        locals.var_ps0ld_dn5 = assign77900_e118022_d_n5;
        locals.var_ps0ld_dn6 = assign77900_e118022_d_n6;
        locals.var_ps0ld_dn7 = assign77900_e118022_d_n7;
        locals.var_ps0ld_dn8 = assign77900_e118022_d_n8;
        locals.var_ps0ld_dn9 = assign77900_e118022_d_n9;
        locals.var_ps0ld_dn10 = assign77900_e118022_d_n10;
        locals.var_ps0ld_dn11 = assign77900_e118022_d_n11;
        locals.var_ps0ld_dn14 = assign77900_e118022_d_n14;
        locals.var_ps0ld_rv = 0.0;

        let assign77910_e118024: f64 = (locals.var_chi).abs();
        let assign77910_e118026: f64 = if assign77910_e118024 > 1e-6 { 1.0 } else { 0.0 };
        locals.var_guard1807 = assign77910_e118026;
        locals.var_guard1807_rv = 0.0;

        let (assign77920_e118041, assign77920_e118041_d_n0, assign77920_e118041_d_n2, assign77920_e118041_d_n4, assign77920_e118041_d_n5, assign77920_e118041_d_n6, assign77920_e118041_d_n7, assign77920_e118041_d_n8, assign77920_e118041_d_n9, assign77920_e118041_d_n10, assign77920_e118041_d_n11, assign77920_e118041_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1788 == 0.0)) && (locals.var_guard1807 != 0.0)) {
        let assign77920_e118035: f64 = (locals.var_chi - 1.0);
        let assign77920_e118037: f64 = (-locals.var_chi);
        let assign77920_e118038: f64 = (assign77920_e118037).exp();
        let assign77920_e118039: f64 = (assign77920_e118035 + assign77920_e118038);
        (assign77920_e118039, (locals.var_chi_dn0 + (assign77920_e118038 * (-locals.var_chi_dn0))), (locals.var_chi_dn2 + (assign77920_e118038 * (-locals.var_chi_dn2))), (locals.var_chi_dn4 + (assign77920_e118038 * (-locals.var_chi_dn4))), (locals.var_chi_dn5 + (assign77920_e118038 * (-locals.var_chi_dn5))), (locals.var_chi_dn6 + (assign77920_e118038 * (-locals.var_chi_dn6))), (locals.var_chi_dn7 + (assign77920_e118038 * (-locals.var_chi_dn7))), (locals.var_chi_dn8 + (assign77920_e118038 * (-locals.var_chi_dn8))), (locals.var_chi_dn9 + (assign77920_e118038 * (-locals.var_chi_dn9))), (locals.var_chi_dn10 + (assign77920_e118038 * (-locals.var_chi_dn10))), (locals.var_chi_dn11 + (assign77920_e118038 * (-locals.var_chi_dn11))), (locals.var_chi_dn14 + (assign77920_e118038 * (-locals.var_chi_dn14))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign77920_e118041;
        locals.var_t1_dn0 = assign77920_e118041_d_n0;
        locals.var_t1_dn2 = assign77920_e118041_d_n2;
        locals.var_t1_dn4 = assign77920_e118041_d_n4;
        locals.var_t1_dn5 = assign77920_e118041_d_n5;
        locals.var_t1_dn6 = assign77920_e118041_d_n6;
        locals.var_t1_dn7 = assign77920_e118041_d_n7;
        locals.var_t1_dn8 = assign77920_e118041_d_n8;
        locals.var_t1_dn9 = assign77920_e118041_d_n9;
        locals.var_t1_dn10 = assign77920_e118041_d_n10;
        locals.var_t1_dn11 = assign77920_e118041_d_n11;
        locals.var_t1_dn14 = assign77920_e118041_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign77930_e118051, assign77930_e118051_d_n0, assign77930_e118051_d_n2, assign77930_e118051_d_n4, assign77930_e118051_d_n5, assign77930_e118051_d_n6, assign77930_e118051_d_n7, assign77930_e118051_d_n8, assign77930_e118051_d_n9, assign77930_e118051_d_n10, assign77930_e118051_d_n11, assign77930_e118051_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1788 == 0.0)) && (locals.var_guard1807 != 0.0)) {
        let assign77930_e118049: f64 = (locals.var_t1).sqrt();
        (assign77930_e118049, (locals.var_t1_dn0 / (2.0 * assign77930_e118049)), (locals.var_t1_dn2 / (2.0 * assign77930_e118049)), (locals.var_t1_dn4 / (2.0 * assign77930_e118049)), (locals.var_t1_dn5 / (2.0 * assign77930_e118049)), (locals.var_t1_dn6 / (2.0 * assign77930_e118049)), (locals.var_t1_dn7 / (2.0 * assign77930_e118049)), (locals.var_t1_dn8 / (2.0 * assign77930_e118049)), (locals.var_t1_dn9 / (2.0 * assign77930_e118049)), (locals.var_t1_dn10 / (2.0 * assign77930_e118049)), (locals.var_t1_dn11 / (2.0 * assign77930_e118049)), (locals.var_t1_dn14 / (2.0 * assign77930_e118049)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign77930_e118051;
        locals.var_t2_dn0 = assign77930_e118051_d_n0;
        locals.var_t2_dn2 = assign77930_e118051_d_n2;
        locals.var_t2_dn4 = assign77930_e118051_d_n4;
        locals.var_t2_dn5 = assign77930_e118051_d_n5;
        locals.var_t2_dn6 = assign77930_e118051_d_n6;
        locals.var_t2_dn7 = assign77930_e118051_d_n7;
        locals.var_t2_dn8 = assign77930_e118051_d_n8;
        locals.var_t2_dn9 = assign77930_e118051_d_n9;
        locals.var_t2_dn10 = assign77930_e118051_d_n10;
        locals.var_t2_dn11 = assign77930_e118051_d_n11;
        locals.var_t2_dn14 = assign77930_e118051_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign77950_e118082, assign77950_e118082_d_n0, assign77950_e118082_d_n2, assign77950_e118082_d_n4, assign77950_e118082_d_n5, assign77950_e118082_d_n6, assign77950_e118082_d_n7, assign77950_e118082_d_n8, assign77950_e118082_d_n9, assign77950_e118082_d_n10, assign77950_e118082_d_n11, assign77950_e118082_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1788 == 0.0)) && (locals.var_guard1807 == 0.0)) {
        let assign77950_e118073: f64 = (0.7071067811865475 * locals.var_chi);
        let assign77950_e118077: f64 = (locals.var_chi * 0.3333333333333333);
        let assign77950_e118078: f64 = (1.0 - assign77950_e118077);
        let assign77950_e118079: f64 = (assign77950_e118078).sqrt();
        let assign77950_e118080: f64 = (assign77950_e118073 * assign77950_e118079);
        (assign77950_e118080, (((0.7071067811865475 * locals.var_chi_dn0) * assign77950_e118079) + (assign77950_e118073 * ((-(locals.var_chi_dn0 * 0.3333333333333333)) / (2.0 * assign77950_e118079)))), (((0.7071067811865475 * locals.var_chi_dn2) * assign77950_e118079) + (assign77950_e118073 * ((-(locals.var_chi_dn2 * 0.3333333333333333)) / (2.0 * assign77950_e118079)))), (((0.7071067811865475 * locals.var_chi_dn4) * assign77950_e118079) + (assign77950_e118073 * ((-(locals.var_chi_dn4 * 0.3333333333333333)) / (2.0 * assign77950_e118079)))), (((0.7071067811865475 * locals.var_chi_dn5) * assign77950_e118079) + (assign77950_e118073 * ((-(locals.var_chi_dn5 * 0.3333333333333333)) / (2.0 * assign77950_e118079)))), (((0.7071067811865475 * locals.var_chi_dn6) * assign77950_e118079) + (assign77950_e118073 * ((-(locals.var_chi_dn6 * 0.3333333333333333)) / (2.0 * assign77950_e118079)))), (((0.7071067811865475 * locals.var_chi_dn7) * assign77950_e118079) + (assign77950_e118073 * ((-(locals.var_chi_dn7 * 0.3333333333333333)) / (2.0 * assign77950_e118079)))), (((0.7071067811865475 * locals.var_chi_dn8) * assign77950_e118079) + (assign77950_e118073 * ((-(locals.var_chi_dn8 * 0.3333333333333333)) / (2.0 * assign77950_e118079)))), (((0.7071067811865475 * locals.var_chi_dn9) * assign77950_e118079) + (assign77950_e118073 * ((-(locals.var_chi_dn9 * 0.3333333333333333)) / (2.0 * assign77950_e118079)))), (((0.7071067811865475 * locals.var_chi_dn10) * assign77950_e118079) + (assign77950_e118073 * ((-(locals.var_chi_dn10 * 0.3333333333333333)) / (2.0 * assign77950_e118079)))), (((0.7071067811865475 * locals.var_chi_dn11) * assign77950_e118079) + (assign77950_e118073 * ((-(locals.var_chi_dn11 * 0.3333333333333333)) / (2.0 * assign77950_e118079)))), (((0.7071067811865475 * locals.var_chi_dn14) * assign77950_e118079) + (assign77950_e118073 * ((-(locals.var_chi_dn14 * 0.3333333333333333)) / (2.0 * assign77950_e118079)))),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign77950_e118082;
        locals.var_t2_dn0 = assign77950_e118082_d_n0;
        locals.var_t2_dn2 = assign77950_e118082_d_n2;
        locals.var_t2_dn4 = assign77950_e118082_d_n4;
        locals.var_t2_dn5 = assign77950_e118082_d_n5;
        locals.var_t2_dn6 = assign77950_e118082_d_n6;
        locals.var_t2_dn7 = assign77950_e118082_d_n7;
        locals.var_t2_dn8 = assign77950_e118082_d_n8;
        locals.var_t2_dn9 = assign77950_e118082_d_n9;
        locals.var_t2_dn10 = assign77950_e118082_d_n10;
        locals.var_t2_dn11 = assign77950_e118082_d_n11;
        locals.var_t2_dn14 = assign77950_e118082_d_n14;
        locals.var_t2_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_295(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign77960_e118091, assign77960_e118091_d_n0, assign77960_e118091_d_n2, assign77960_e118091_d_n4, assign77960_e118091_d_n5, assign77960_e118091_d_n6, assign77960_e118091_d_n7, assign77960_e118091_d_n8, assign77960_e118091_d_n9, assign77960_e118091_d_n10, assign77960_e118091_d_n11, assign77960_e118091_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1788 == 0.0)) {
        let assign77960_e118089: f64 = (locals.var_cnst0over_func * locals.var_t2);
        (assign77960_e118089, ((locals.var_cnst0over_func_dn0 * locals.var_t2) + (locals.var_cnst0over_func * locals.var_t2_dn0)), ((locals.var_cnst0over_func_dn2 * locals.var_t2) + (locals.var_cnst0over_func * locals.var_t2_dn2)), ((locals.var_cnst0over_func_dn4 * locals.var_t2) + (locals.var_cnst0over_func * locals.var_t2_dn4)), ((locals.var_cnst0over_func_dn5 * locals.var_t2) + (locals.var_cnst0over_func * locals.var_t2_dn5)), ((locals.var_cnst0over_func_dn6 * locals.var_t2) + (locals.var_cnst0over_func * locals.var_t2_dn6)), ((locals.var_cnst0over_func_dn7 * locals.var_t2) + (locals.var_cnst0over_func * locals.var_t2_dn7)), ((locals.var_cnst0over_func_dn8 * locals.var_t2) + (locals.var_cnst0over_func * locals.var_t2_dn8)), ((locals.var_cnst0over_func_dn9 * locals.var_t2) + (locals.var_cnst0over_func * locals.var_t2_dn9)), ((locals.var_cnst0over_func_dn10 * locals.var_t2) + (locals.var_cnst0over_func * locals.var_t2_dn10)), ((locals.var_cnst0over_func_dn11 * locals.var_t2) + (locals.var_cnst0over_func * locals.var_t2_dn11)), ((locals.var_cnst0over_func_dn14 * locals.var_t2) + (locals.var_cnst0over_func * locals.var_t2_dn14)),)
    } else {
        (locals.var_qbuld, locals.var_qbuld_dn0, locals.var_qbuld_dn2, locals.var_qbuld_dn4, locals.var_qbuld_dn5, locals.var_qbuld_dn6, locals.var_qbuld_dn7, locals.var_qbuld_dn8, locals.var_qbuld_dn9, locals.var_qbuld_dn10, locals.var_qbuld_dn11, locals.var_qbuld_dn14,)
    }
};
        locals.var_qbuld = assign77960_e118091;
        locals.var_qbuld_dn0 = assign77960_e118091_d_n0;
        locals.var_qbuld_dn2 = assign77960_e118091_d_n2;
        locals.var_qbuld_dn4 = assign77960_e118091_d_n4;
        locals.var_qbuld_dn5 = assign77960_e118091_d_n5;
        locals.var_qbuld_dn6 = assign77960_e118091_d_n6;
        locals.var_qbuld_dn7 = assign77960_e118091_d_n7;
        locals.var_qbuld_dn8 = assign77960_e118091_d_n8;
        locals.var_qbuld_dn9 = assign77960_e118091_d_n9;
        locals.var_qbuld_dn10 = assign77960_e118091_d_n10;
        locals.var_qbuld_dn11 = assign77960_e118091_d_n11;
        locals.var_qbuld_dn14 = assign77960_e118091_d_n14;
        locals.var_qbuld_rv = 0.0;

        let (assign77970_e118102, assign77970_e118102_d_n0, assign77970_e118102_d_n2, assign77970_e118102_d_n4, assign77970_e118102_d_n5, assign77970_e118102_d_n6, assign77970_e118102_d_n7, assign77970_e118102_d_n8, assign77970_e118102_d_n9, assign77970_e118102_d_n10, assign77970_e118102_d_n11, assign77970_e118102_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1788 == 0.0)) {
        let assign77970_e118099: f64 = (locals.var_vgpld - locals.var_ps0ld);
        let assign77970_e118100: f64 = (locals.var_cox0_func * assign77970_e118099);
        (assign77970_e118100, (locals.var_cox0_func * (-locals.var_ps0ld_dn0)), (locals.var_cox0_func * (locals.var_vgpld_dn2 - locals.var_ps0ld_dn2)), (locals.var_cox0_func * (-locals.var_ps0ld_dn4)), (locals.var_cox0_func * (-locals.var_ps0ld_dn5)), (locals.var_cox0_func * (-locals.var_ps0ld_dn6)), (locals.var_cox0_func * (locals.var_vgpld_dn7 - locals.var_ps0ld_dn7)), (locals.var_cox0_func * (locals.var_vgpld_dn8 - locals.var_ps0ld_dn8)), (locals.var_cox0_func * (locals.var_vgpld_dn9 - locals.var_ps0ld_dn9)), (locals.var_cox0_func * (-locals.var_ps0ld_dn10)), (locals.var_cox0_func * (-locals.var_ps0ld_dn11)), (locals.var_cox0_func * (-locals.var_ps0ld_dn14)),)
    } else {
        (locals.var_qsuld, locals.var_qsuld_dn0, locals.var_qsuld_dn2, locals.var_qsuld_dn4, locals.var_qsuld_dn5, locals.var_qsuld_dn6, locals.var_qsuld_dn7, locals.var_qsuld_dn8, locals.var_qsuld_dn9, locals.var_qsuld_dn10, locals.var_qsuld_dn11, locals.var_qsuld_dn14,)
    }
};
        locals.var_qsuld = assign77970_e118102;
        locals.var_qsuld_dn0 = assign77970_e118102_d_n0;
        locals.var_qsuld_dn2 = assign77970_e118102_d_n2;
        locals.var_qsuld_dn4 = assign77970_e118102_d_n4;
        locals.var_qsuld_dn5 = assign77970_e118102_d_n5;
        locals.var_qsuld_dn6 = assign77970_e118102_d_n6;
        locals.var_qsuld_dn7 = assign77970_e118102_d_n7;
        locals.var_qsuld_dn8 = assign77970_e118102_d_n8;
        locals.var_qsuld_dn9 = assign77970_e118102_d_n9;
        locals.var_qsuld_dn10 = assign77970_e118102_d_n10;
        locals.var_qsuld_dn11 = assign77970_e118102_d_n11;
        locals.var_qsuld_dn14 = assign77970_e118102_d_n14;
        locals.var_qsuld_rv = 0.0;

        let (assign77980_e118111, assign77980_e118111_d_n0, assign77980_e118111_d_n2, assign77980_e118111_d_n4, assign77980_e118111_d_n5, assign77980_e118111_d_n6, assign77980_e118111_d_n7, assign77980_e118111_d_n8, assign77980_e118111_d_n9, assign77980_e118111_d_n10, assign77980_e118111_d_n11, assign77980_e118111_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1788 == 0.0)) {
        let assign77980_e118109: f64 = (locals.var_qbuld / locals.var_q_nsubld__blk1764);
        (assign77980_e118109, (locals.var_qbuld_dn0 / locals.var_q_nsubld__blk1764), (locals.var_qbuld_dn2 / locals.var_q_nsubld__blk1764), (locals.var_qbuld_dn4 / locals.var_q_nsubld__blk1764), (locals.var_qbuld_dn5 / locals.var_q_nsubld__blk1764), (locals.var_qbuld_dn6 / locals.var_q_nsubld__blk1764), (locals.var_qbuld_dn7 / locals.var_q_nsubld__blk1764), (locals.var_qbuld_dn8 / locals.var_q_nsubld__blk1764), (locals.var_qbuld_dn9 / locals.var_q_nsubld__blk1764), (locals.var_qbuld_dn10 / locals.var_q_nsubld__blk1764), (locals.var_qbuld_dn11 / locals.var_q_nsubld__blk1764), (locals.var_qbuld_dn14 / locals.var_q_nsubld__blk1764),)
    } else {
        (locals.var_wdld0__blk1808, locals.var_wdld0__blk1808_dn0, locals.var_wdld0__blk1808_dn2, locals.var_wdld0__blk1808_dn4, locals.var_wdld0__blk1808_dn5, locals.var_wdld0__blk1808_dn6, locals.var_wdld0__blk1808_dn7, locals.var_wdld0__blk1808_dn8, locals.var_wdld0__blk1808_dn9, locals.var_wdld0__blk1808_dn10, locals.var_wdld0__blk1808_dn11, locals.var_wdld0__blk1808_dn14,)
    }
};
        locals.var_wdld0__blk1808 = assign77980_e118111;
        locals.var_wdld0__blk1808_dn0 = assign77980_e118111_d_n0;
        locals.var_wdld0__blk1808_dn2 = assign77980_e118111_d_n2;
        locals.var_wdld0__blk1808_dn4 = assign77980_e118111_d_n4;
        locals.var_wdld0__blk1808_dn5 = assign77980_e118111_d_n5;
        locals.var_wdld0__blk1808_dn6 = assign77980_e118111_d_n6;
        locals.var_wdld0__blk1808_dn7 = assign77980_e118111_d_n7;
        locals.var_wdld0__blk1808_dn8 = assign77980_e118111_d_n8;
        locals.var_wdld0__blk1808_dn9 = assign77980_e118111_d_n9;
        locals.var_wdld0__blk1808_dn10 = assign77980_e118111_d_n10;
        locals.var_wdld0__blk1808_dn11 = assign77980_e118111_d_n11;
        locals.var_wdld0__blk1808_dn14 = assign77980_e118111_d_n14;
        locals.var_wdld0__blk1808_rv = 0.0;

        let assign77990_e118114: f64 = if p.p33 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard1810 = assign77990_e118114;
        locals.var_guard1810_rv = 0.0;

        let assign78000_e118119: f64 = (locals.var_ddriftldc * 0.1);
        let assign78000_e118120: f64 = (locals.var_ddriftldc - assign78000_e118119);
        let assign78000_e118124: f64 = (locals.var_ddriftldc * 0.1);
        let assign78000_e118127: f64 = if ((locals.var_wdld0__blk1808 > assign78000_e118120) && (assign78000_e118124 >= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1811 = assign78000_e118127;
        locals.var_guard1811_rv = 0.0;

        let (assign78010_e118144, assign78010_e118144_d_n0, assign78010_e118144_d_n2, assign78010_e118144_d_n4, assign78010_e118144_d_n5, assign78010_e118144_d_n6, assign78010_e118144_d_n7, assign78010_e118144_d_n8, assign78010_e118144_d_n9, assign78010_e118144_d_n10, assign78010_e118144_d_n11, assign78010_e118144_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1788 == 0.0)) && (locals.var_guard1810 != 0.0)) && (locals.var_guard1811 != 0.0)) {
        let assign78010_e118138: f64 = (locals.var_wdld0__blk1808 - locals.var_ddriftldc);
        let assign78010_e118141: f64 = (locals.var_ddriftldc * 0.1);
        let assign78010_e118142: f64 = (assign78010_e118138 + assign78010_e118141);
        (assign78010_e118142, ((locals.var_wdld0__blk1808_dn0 - locals.var_ddriftldc_dn0) + (locals.var_ddriftldc_dn0 * 0.1)), ((locals.var_wdld0__blk1808_dn2 - locals.var_ddriftldc_dn2) + (locals.var_ddriftldc_dn2 * 0.1)), ((locals.var_wdld0__blk1808_dn4 - locals.var_ddriftldc_dn4) + (locals.var_ddriftldc_dn4 * 0.1)), ((locals.var_wdld0__blk1808_dn5 - locals.var_ddriftldc_dn5) + (locals.var_ddriftldc_dn5 * 0.1)), ((locals.var_wdld0__blk1808_dn6 - locals.var_ddriftldc_dn6) + (locals.var_ddriftldc_dn6 * 0.1)), ((locals.var_wdld0__blk1808_dn7 - locals.var_ddriftldc_dn7) + (locals.var_ddriftldc_dn7 * 0.1)), ((locals.var_wdld0__blk1808_dn8 - locals.var_ddriftldc_dn8) + (locals.var_ddriftldc_dn8 * 0.1)), ((locals.var_wdld0__blk1808_dn9 - locals.var_ddriftldc_dn9) + (locals.var_ddriftldc_dn9 * 0.1)), ((locals.var_wdld0__blk1808_dn10 - locals.var_ddriftldc_dn10) + (locals.var_ddriftldc_dn10 * 0.1)), ((locals.var_wdld0__blk1808_dn11 - locals.var_ddriftldc_dn11) + (locals.var_ddriftldc_dn11 * 0.1)), ((locals.var_wdld0__blk1808_dn14 - locals.var_ddriftldc_dn14) + (locals.var_ddriftldc_dn14 * 0.1)),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign78010_e118144;
        locals.var_tmf1_dn0 = assign78010_e118144_d_n0;
        locals.var_tmf1_dn2 = assign78010_e118144_d_n2;
        locals.var_tmf1_dn4 = assign78010_e118144_d_n4;
        locals.var_tmf1_dn5 = assign78010_e118144_d_n5;
        locals.var_tmf1_dn6 = assign78010_e118144_d_n6;
        locals.var_tmf1_dn7 = assign78010_e118144_d_n7;
        locals.var_tmf1_dn8 = assign78010_e118144_d_n8;
        locals.var_tmf1_dn9 = assign78010_e118144_d_n9;
        locals.var_tmf1_dn10 = assign78010_e118144_d_n10;
        locals.var_tmf1_dn11 = assign78010_e118144_d_n11;
        locals.var_tmf1_dn14 = assign78010_e118144_d_n14;
        locals.var_tmf1_rv = 0.0;

        let (assign78020_e118157, assign78020_e118157_d_n0, assign78020_e118157_d_n2, assign78020_e118157_d_n4, assign78020_e118157_d_n5, assign78020_e118157_d_n6, assign78020_e118157_d_n7, assign78020_e118157_d_n8, assign78020_e118157_d_n9, assign78020_e118157_d_n10, assign78020_e118157_d_n11, assign78020_e118157_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1788 == 0.0)) && (locals.var_guard1810 != 0.0)) && (locals.var_guard1811 != 0.0)) {
        let assign78020_e118155: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign78020_e118155, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)), ((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)), ((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)), ((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn4, locals.var_x2_dn5, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn8, locals.var_x2_dn9, locals.var_x2_dn10, locals.var_x2_dn11, locals.var_x2_dn14,)
    }
};
        locals.var_x2 = assign78020_e118157;
        locals.var_x2_dn0 = assign78020_e118157_d_n0;
        locals.var_x2_dn2 = assign78020_e118157_d_n2;
        locals.var_x2_dn4 = assign78020_e118157_d_n4;
        locals.var_x2_dn5 = assign78020_e118157_d_n5;
        locals.var_x2_dn6 = assign78020_e118157_d_n6;
        locals.var_x2_dn7 = assign78020_e118157_d_n7;
        locals.var_x2_dn8 = assign78020_e118157_d_n8;
        locals.var_x2_dn9 = assign78020_e118157_d_n9;
        locals.var_x2_dn10 = assign78020_e118157_d_n10;
        locals.var_x2_dn11 = assign78020_e118157_d_n11;
        locals.var_x2_dn14 = assign78020_e118157_d_n14;
        locals.var_x2_rv = 0.0;

        let (assign78030_e118174, assign78030_e118174_d_n0, assign78030_e118174_d_n2, assign78030_e118174_d_n4, assign78030_e118174_d_n5, assign78030_e118174_d_n6, assign78030_e118174_d_n7, assign78030_e118174_d_n8, assign78030_e118174_d_n9, assign78030_e118174_d_n10, assign78030_e118174_d_n11, assign78030_e118174_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1788 == 0.0)) && (locals.var_guard1810 != 0.0)) && (locals.var_guard1811 != 0.0)) {
        let assign78030_e118168: f64 = (locals.var_ddriftldc * 0.1);
        let assign78030_e118171: f64 = (locals.var_ddriftldc * 0.1);
        let assign78030_e118172: f64 = (assign78030_e118168 * assign78030_e118171);
        (assign78030_e118172, (((locals.var_ddriftldc_dn0 * 0.1) * assign78030_e118171) + (assign78030_e118168 * (locals.var_ddriftldc_dn0 * 0.1))), (((locals.var_ddriftldc_dn2 * 0.1) * assign78030_e118171) + (assign78030_e118168 * (locals.var_ddriftldc_dn2 * 0.1))), (((locals.var_ddriftldc_dn4 * 0.1) * assign78030_e118171) + (assign78030_e118168 * (locals.var_ddriftldc_dn4 * 0.1))), (((locals.var_ddriftldc_dn5 * 0.1) * assign78030_e118171) + (assign78030_e118168 * (locals.var_ddriftldc_dn5 * 0.1))), (((locals.var_ddriftldc_dn6 * 0.1) * assign78030_e118171) + (assign78030_e118168 * (locals.var_ddriftldc_dn6 * 0.1))), (((locals.var_ddriftldc_dn7 * 0.1) * assign78030_e118171) + (assign78030_e118168 * (locals.var_ddriftldc_dn7 * 0.1))), (((locals.var_ddriftldc_dn8 * 0.1) * assign78030_e118171) + (assign78030_e118168 * (locals.var_ddriftldc_dn8 * 0.1))), (((locals.var_ddriftldc_dn9 * 0.1) * assign78030_e118171) + (assign78030_e118168 * (locals.var_ddriftldc_dn9 * 0.1))), (((locals.var_ddriftldc_dn10 * 0.1) * assign78030_e118171) + (assign78030_e118168 * (locals.var_ddriftldc_dn10 * 0.1))), (((locals.var_ddriftldc_dn11 * 0.1) * assign78030_e118171) + (assign78030_e118168 * (locals.var_ddriftldc_dn11 * 0.1))), (((locals.var_ddriftldc_dn14 * 0.1) * assign78030_e118171) + (assign78030_e118168 * (locals.var_ddriftldc_dn14 * 0.1))),)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn4, locals.var_xmax2_dn5, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn8, locals.var_xmax2_dn9, locals.var_xmax2_dn10, locals.var_xmax2_dn11, locals.var_xmax2_dn14,)
    }
};
        locals.var_xmax2 = assign78030_e118174;
        locals.var_xmax2_dn0 = assign78030_e118174_d_n0;
        locals.var_xmax2_dn2 = assign78030_e118174_d_n2;
        locals.var_xmax2_dn4 = assign78030_e118174_d_n4;
        locals.var_xmax2_dn5 = assign78030_e118174_d_n5;
        locals.var_xmax2_dn6 = assign78030_e118174_d_n6;
        locals.var_xmax2_dn7 = assign78030_e118174_d_n7;
        locals.var_xmax2_dn8 = assign78030_e118174_d_n8;
        locals.var_xmax2_dn9 = assign78030_e118174_d_n9;
        locals.var_xmax2_dn10 = assign78030_e118174_d_n10;
        locals.var_xmax2_dn11 = assign78030_e118174_d_n11;
        locals.var_xmax2_dn14 = assign78030_e118174_d_n14;
        locals.var_xmax2_rv = 0.0;

        let (assign78040_e118185, assign78040_e118185_d_n0, assign78040_e118185_d_n2, assign78040_e118185_d_n4, assign78040_e118185_d_n5, assign78040_e118185_d_n6, assign78040_e118185_d_n7, assign78040_e118185_d_n8, assign78040_e118185_d_n9, assign78040_e118185_d_n10, assign78040_e118185_d_n11, assign78040_e118185_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1788 == 0.0)) && (locals.var_guard1810 != 0.0)) && (locals.var_guard1811 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign78040_e118185;
        locals.var_xp_dn0 = assign78040_e118185_d_n0;
        locals.var_xp_dn2 = assign78040_e118185_d_n2;
        locals.var_xp_dn4 = assign78040_e118185_d_n4;
        locals.var_xp_dn5 = assign78040_e118185_d_n5;
        locals.var_xp_dn6 = assign78040_e118185_d_n6;
        locals.var_xp_dn7 = assign78040_e118185_d_n7;
        locals.var_xp_dn8 = assign78040_e118185_d_n8;
        locals.var_xp_dn9 = assign78040_e118185_d_n9;
        locals.var_xp_dn10 = assign78040_e118185_d_n10;
        locals.var_xp_dn11 = assign78040_e118185_d_n11;
        locals.var_xp_dn14 = assign78040_e118185_d_n14;
        locals.var_xp_rv = 0.0;

        let (assign78050_e118196, assign78050_e118196_d_n0, assign78050_e118196_d_n2, assign78050_e118196_d_n4, assign78050_e118196_d_n5, assign78050_e118196_d_n6, assign78050_e118196_d_n7, assign78050_e118196_d_n8, assign78050_e118196_d_n9, assign78050_e118196_d_n10, assign78050_e118196_d_n11, assign78050_e118196_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1788 == 0.0)) && (locals.var_guard1810 != 0.0)) && (locals.var_guard1811 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign78050_e118196;
        locals.var_xmp_dn0 = assign78050_e118196_d_n0;
        locals.var_xmp_dn2 = assign78050_e118196_d_n2;
        locals.var_xmp_dn4 = assign78050_e118196_d_n4;
        locals.var_xmp_dn5 = assign78050_e118196_d_n5;
        locals.var_xmp_dn6 = assign78050_e118196_d_n6;
        locals.var_xmp_dn7 = assign78050_e118196_d_n7;
        locals.var_xmp_dn8 = assign78050_e118196_d_n8;
        locals.var_xmp_dn9 = assign78050_e118196_d_n9;
        locals.var_xmp_dn10 = assign78050_e118196_d_n10;
        locals.var_xmp_dn11 = assign78050_e118196_d_n11;
        locals.var_xmp_dn14 = assign78050_e118196_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign78060_e118207,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1788 == 0.0)) && (locals.var_guard1810 != 0.0)) && (locals.var_guard1811 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign78060_e118207;
        locals.var_m0_rv = 0.0;

        let (assign78070_e118218,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1788 == 0.0)) && (locals.var_guard1810 != 0.0)) && (locals.var_guard1811 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign78070_e118218;
        locals.var_mm_rv = 0.0;

        let (assign78080_e118229, assign78080_e118229_d_n0, assign78080_e118229_d_n2, assign78080_e118229_d_n4, assign78080_e118229_d_n5, assign78080_e118229_d_n6, assign78080_e118229_d_n7, assign78080_e118229_d_n8, assign78080_e118229_d_n9, assign78080_e118229_d_n10, assign78080_e118229_d_n11, assign78080_e118229_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1788 == 0.0)) && (locals.var_guard1810 != 0.0)) && (locals.var_guard1811 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign78080_e118229;
        locals.var_arg_dn0 = assign78080_e118229_d_n0;
        locals.var_arg_dn2 = assign78080_e118229_d_n2;
        locals.var_arg_dn4 = assign78080_e118229_d_n4;
        locals.var_arg_dn5 = assign78080_e118229_d_n5;
        locals.var_arg_dn6 = assign78080_e118229_d_n6;
        locals.var_arg_dn7 = assign78080_e118229_d_n7;
        locals.var_arg_dn8 = assign78080_e118229_d_n8;
        locals.var_arg_dn9 = assign78080_e118229_d_n9;
        locals.var_arg_dn10 = assign78080_e118229_d_n10;
        locals.var_arg_dn11 = assign78080_e118229_d_n11;
        locals.var_arg_dn14 = assign78080_e118229_d_n14;
        locals.var_arg_rv = 0.0;

        let (assign78090_e118240, assign78090_e118240_d_n0, assign78090_e118240_d_n2, assign78090_e118240_d_n4, assign78090_e118240_d_n5, assign78090_e118240_d_n6, assign78090_e118240_d_n7, assign78090_e118240_d_n8, assign78090_e118240_d_n9, assign78090_e118240_d_n10, assign78090_e118240_d_n11, assign78090_e118240_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1788 == 0.0)) && (locals.var_guard1810 != 0.0)) && (locals.var_guard1811 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign78090_e118240;
        locals.var_dnm_dn0 = assign78090_e118240_d_n0;
        locals.var_dnm_dn2 = assign78090_e118240_d_n2;
        locals.var_dnm_dn4 = assign78090_e118240_d_n4;
        locals.var_dnm_dn5 = assign78090_e118240_d_n5;
        locals.var_dnm_dn6 = assign78090_e118240_d_n6;
        locals.var_dnm_dn7 = assign78090_e118240_d_n7;
        locals.var_dnm_dn8 = assign78090_e118240_d_n8;
        locals.var_dnm_dn9 = assign78090_e118240_d_n9;
        locals.var_dnm_dn10 = assign78090_e118240_d_n10;
        locals.var_dnm_dn11 = assign78090_e118240_d_n11;
        locals.var_dnm_dn14 = assign78090_e118240_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign78100_e118253, assign78100_e118253_d_n0, assign78100_e118253_d_n2, assign78100_e118253_d_n4, assign78100_e118253_d_n5, assign78100_e118253_d_n6, assign78100_e118253_d_n7, assign78100_e118253_d_n8, assign78100_e118253_d_n9, assign78100_e118253_d_n10, assign78100_e118253_d_n11, assign78100_e118253_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1788 == 0.0)) && (locals.var_guard1810 != 0.0)) && (locals.var_guard1811 != 0.0)) {
        let assign78100_e118251: f64 = (locals.var_xp * locals.var_x2);
        (assign78100_e118251, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign78100_e118253;
        locals.var_xp_dn0 = assign78100_e118253_d_n0;
        locals.var_xp_dn2 = assign78100_e118253_d_n2;
        locals.var_xp_dn4 = assign78100_e118253_d_n4;
        locals.var_xp_dn5 = assign78100_e118253_d_n5;
        locals.var_xp_dn6 = assign78100_e118253_d_n6;
        locals.var_xp_dn7 = assign78100_e118253_d_n7;
        locals.var_xp_dn8 = assign78100_e118253_d_n8;
        locals.var_xp_dn9 = assign78100_e118253_d_n9;
        locals.var_xp_dn10 = assign78100_e118253_d_n10;
        locals.var_xp_dn11 = assign78100_e118253_d_n11;
        locals.var_xp_dn14 = assign78100_e118253_d_n14;
        locals.var_xp_rv = 0.0;

        let (assign78110_e118266, assign78110_e118266_d_n0, assign78110_e118266_d_n2, assign78110_e118266_d_n4, assign78110_e118266_d_n5, assign78110_e118266_d_n6, assign78110_e118266_d_n7, assign78110_e118266_d_n8, assign78110_e118266_d_n9, assign78110_e118266_d_n10, assign78110_e118266_d_n11, assign78110_e118266_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1788 == 0.0)) && (locals.var_guard1810 != 0.0)) && (locals.var_guard1811 != 0.0)) {
        let assign78110_e118264: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign78110_e118264, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign78110_e118266;
        locals.var_xmp_dn0 = assign78110_e118266_d_n0;
        locals.var_xmp_dn2 = assign78110_e118266_d_n2;
        locals.var_xmp_dn4 = assign78110_e118266_d_n4;
        locals.var_xmp_dn5 = assign78110_e118266_d_n5;
        locals.var_xmp_dn6 = assign78110_e118266_d_n6;
        locals.var_xmp_dn7 = assign78110_e118266_d_n7;
        locals.var_xmp_dn8 = assign78110_e118266_d_n8;
        locals.var_xmp_dn9 = assign78110_e118266_d_n9;
        locals.var_xmp_dn10 = assign78110_e118266_d_n10;
        locals.var_xmp_dn11 = assign78110_e118266_d_n11;
        locals.var_xmp_dn14 = assign78110_e118266_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign78120_e118279, assign78120_e118279_d_n0, assign78120_e118279_d_n2, assign78120_e118279_d_n4, assign78120_e118279_d_n5, assign78120_e118279_d_n6, assign78120_e118279_d_n7, assign78120_e118279_d_n8, assign78120_e118279_d_n9, assign78120_e118279_d_n10, assign78120_e118279_d_n11, assign78120_e118279_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1788 == 0.0)) && (locals.var_guard1810 != 0.0)) && (locals.var_guard1811 != 0.0)) {
        let assign78120_e118277: f64 = (locals.var_xp * locals.var_x2);
        (assign78120_e118277, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign78120_e118279;
        locals.var_xp_dn0 = assign78120_e118279_d_n0;
        locals.var_xp_dn2 = assign78120_e118279_d_n2;
        locals.var_xp_dn4 = assign78120_e118279_d_n4;
        locals.var_xp_dn5 = assign78120_e118279_d_n5;
        locals.var_xp_dn6 = assign78120_e118279_d_n6;
        locals.var_xp_dn7 = assign78120_e118279_d_n7;
        locals.var_xp_dn8 = assign78120_e118279_d_n8;
        locals.var_xp_dn9 = assign78120_e118279_d_n9;
        locals.var_xp_dn10 = assign78120_e118279_d_n10;
        locals.var_xp_dn11 = assign78120_e118279_d_n11;
        locals.var_xp_dn14 = assign78120_e118279_d_n14;
        locals.var_xp_rv = 0.0;

        let (assign78130_e118292, assign78130_e118292_d_n0, assign78130_e118292_d_n2, assign78130_e118292_d_n4, assign78130_e118292_d_n5, assign78130_e118292_d_n6, assign78130_e118292_d_n7, assign78130_e118292_d_n8, assign78130_e118292_d_n9, assign78130_e118292_d_n10, assign78130_e118292_d_n11, assign78130_e118292_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1788 == 0.0)) && (locals.var_guard1810 != 0.0)) && (locals.var_guard1811 != 0.0)) {
        let assign78130_e118290: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign78130_e118290, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign78130_e118292;
        locals.var_xmp_dn0 = assign78130_e118292_d_n0;
        locals.var_xmp_dn2 = assign78130_e118292_d_n2;
        locals.var_xmp_dn4 = assign78130_e118292_d_n4;
        locals.var_xmp_dn5 = assign78130_e118292_d_n5;
        locals.var_xmp_dn6 = assign78130_e118292_d_n6;
        locals.var_xmp_dn7 = assign78130_e118292_d_n7;
        locals.var_xmp_dn8 = assign78130_e118292_d_n8;
        locals.var_xmp_dn9 = assign78130_e118292_d_n9;
        locals.var_xmp_dn10 = assign78130_e118292_d_n10;
        locals.var_xmp_dn11 = assign78130_e118292_d_n11;
        locals.var_xmp_dn14 = assign78130_e118292_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign78140_e118305, assign78140_e118305_d_n0, assign78140_e118305_d_n2, assign78140_e118305_d_n4, assign78140_e118305_d_n5, assign78140_e118305_d_n6, assign78140_e118305_d_n7, assign78140_e118305_d_n8, assign78140_e118305_d_n9, assign78140_e118305_d_n10, assign78140_e118305_d_n11, assign78140_e118305_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1788 == 0.0)) && (locals.var_guard1810 != 0.0)) && (locals.var_guard1811 != 0.0)) {
        let assign78140_e118303: f64 = (locals.var_xp + locals.var_xmp);
        (assign78140_e118303, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn4 + locals.var_xmp_dn4), (locals.var_xp_dn5 + locals.var_xmp_dn5), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn8 + locals.var_xmp_dn8), (locals.var_xp_dn9 + locals.var_xmp_dn9), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn11 + locals.var_xmp_dn11), (locals.var_xp_dn14 + locals.var_xmp_dn14),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign78140_e118305;
        locals.var_arg_dn0 = assign78140_e118305_d_n0;
        locals.var_arg_dn2 = assign78140_e118305_d_n2;
        locals.var_arg_dn4 = assign78140_e118305_d_n4;
        locals.var_arg_dn5 = assign78140_e118305_d_n5;
        locals.var_arg_dn6 = assign78140_e118305_d_n6;
        locals.var_arg_dn7 = assign78140_e118305_d_n7;
        locals.var_arg_dn8 = assign78140_e118305_d_n8;
        locals.var_arg_dn9 = assign78140_e118305_d_n9;
        locals.var_arg_dn10 = assign78140_e118305_d_n10;
        locals.var_arg_dn11 = assign78140_e118305_d_n11;
        locals.var_arg_dn14 = assign78140_e118305_d_n14;
        locals.var_arg_rv = 0.0;

        let (assign78150_e118316, assign78150_e118316_d_n0, assign78150_e118316_d_n2, assign78150_e118316_d_n4, assign78150_e118316_d_n5, assign78150_e118316_d_n6, assign78150_e118316_d_n7, assign78150_e118316_d_n8, assign78150_e118316_d_n9, assign78150_e118316_d_n10, assign78150_e118316_d_n11, assign78150_e118316_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1788 == 0.0)) && (locals.var_guard1810 != 0.0)) && (locals.var_guard1811 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign78150_e118316;
        locals.var_dnm_dn0 = assign78150_e118316_d_n0;
        locals.var_dnm_dn2 = assign78150_e118316_d_n2;
        locals.var_dnm_dn4 = assign78150_e118316_d_n4;
        locals.var_dnm_dn5 = assign78150_e118316_d_n5;
        locals.var_dnm_dn6 = assign78150_e118316_d_n6;
        locals.var_dnm_dn7 = assign78150_e118316_d_n7;
        locals.var_dnm_dn8 = assign78150_e118316_d_n8;
        locals.var_dnm_dn9 = assign78150_e118316_d_n9;
        locals.var_dnm_dn10 = assign78150_e118316_d_n10;
        locals.var_dnm_dn11 = assign78150_e118316_d_n11;
        locals.var_dnm_dn14 = assign78150_e118316_d_n14;
        locals.var_dnm_rv = 0.0;

        let assign78160_e118331: f64 = if ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard1812 = assign78160_e118331;
        locals.var_guard1812_rv = 0.0;

        let assign78170_e118334: f64 = if 2.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1813 = assign78170_e118334;
        locals.var_guard1813_rv = 0.0;

        let (assign78180_e118349,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1788 == 0.0)) && (locals.var_guard1810 != 0.0)) && (locals.var_guard1811 != 0.0)) && (locals.var_guard1812 != 0.0)) && (locals.var_guard1813 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign78180_e118349;
        locals.var_mm_rv = 0.0;

        let assign78190_e118352: f64 = if 2.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard1814 = assign78190_e118352;
        locals.var_guard1814_rv = 0.0;

        let (assign78200_e118370,) = {
    if (((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1788 == 0.0)) && (locals.var_guard1810 != 0.0)) && (locals.var_guard1811 != 0.0)) && (locals.var_guard1812 != 0.0)) && (locals.var_guard1813 == 0.0)) && (locals.var_guard1814 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign78200_e118370;
        locals.var_mm_rv = 0.0;

        let assign78210_e118373: f64 = if 2.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard1815 = assign78210_e118373;
        locals.var_guard1815_rv = 0.0;

        let (assign78220_e118394,) = {
    if ((((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1788 == 0.0)) && (locals.var_guard1810 != 0.0)) && (locals.var_guard1811 != 0.0)) && (locals.var_guard1812 != 0.0)) && (locals.var_guard1813 == 0.0)) && (locals.var_guard1814 == 0.0)) && (locals.var_guard1815 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign78220_e118394;
        locals.var_mm_rv = 0.0;

        let assign78230_e118397: f64 = if 2.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard1816 = assign78230_e118397;
        locals.var_guard1816_rv = 0.0;

        let (assign78240_e118421,) = {
    if (((((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1788 == 0.0)) && (locals.var_guard1810 != 0.0)) && (locals.var_guard1811 != 0.0)) && (locals.var_guard1812 != 0.0)) && (locals.var_guard1813 == 0.0)) && (locals.var_guard1814 == 0.0)) && (locals.var_guard1815 == 0.0)) && (locals.var_guard1816 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign78240_e118421;
        locals.var_mm_rv = 0.0;

        let (assign78250_e118434,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1788 == 0.0)) && (locals.var_guard1810 != 0.0)) && (locals.var_guard1811 != 0.0)) && (locals.var_guard1812 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign78250_e118434;
        locals.var_m0_rv = 0.0;

        let mut assign78260_loop_guard: usize = 0;
        while {
            let assign78260_cond_e118448: f64 = if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1788 == 0.0)) && (locals.var_guard1810 != 0.0)) && (locals.var_guard1811 != 0.0)) && (locals.var_guard1812 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign78260_cond_e118448 != 0.0
        } {
            assign78260_loop_guard += 1;
            assert!(assign78260_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign78260_body0_e118462, assign78260_body0_e118462_d_n0, assign78260_body0_e118462_d_n2, assign78260_body0_e118462_d_n4, assign78260_body0_e118462_d_n5, assign78260_body0_e118462_d_n6, assign78260_body0_e118462_d_n7, assign78260_body0_e118462_d_n8, assign78260_body0_e118462_d_n9, assign78260_body0_e118462_d_n10, assign78260_body0_e118462_d_n11, assign78260_body0_e118462_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1788 == 0.0)) && (locals.var_guard1810 != 0.0)) && (locals.var_guard1811 != 0.0)) && (locals.var_guard1812 != 0.0)) {
        let assign78260_body0_e118460: f64 = (locals.var_dnm).sqrt();
        (assign78260_body0_e118460, (locals.var_dnm_dn0 / (2.0 * assign78260_body0_e118460)), (locals.var_dnm_dn2 / (2.0 * assign78260_body0_e118460)), (locals.var_dnm_dn4 / (2.0 * assign78260_body0_e118460)), (locals.var_dnm_dn5 / (2.0 * assign78260_body0_e118460)), (locals.var_dnm_dn6 / (2.0 * assign78260_body0_e118460)), (locals.var_dnm_dn7 / (2.0 * assign78260_body0_e118460)), (locals.var_dnm_dn8 / (2.0 * assign78260_body0_e118460)), (locals.var_dnm_dn9 / (2.0 * assign78260_body0_e118460)), (locals.var_dnm_dn10 / (2.0 * assign78260_body0_e118460)), (locals.var_dnm_dn11 / (2.0 * assign78260_body0_e118460)), (locals.var_dnm_dn14 / (2.0 * assign78260_body0_e118460)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
            locals.var_dnm = assign78260_body0_e118462;
            locals.var_dnm_dn0 = assign78260_body0_e118462_d_n0;
            locals.var_dnm_dn2 = assign78260_body0_e118462_d_n2;
            locals.var_dnm_dn4 = assign78260_body0_e118462_d_n4;
            locals.var_dnm_dn5 = assign78260_body0_e118462_d_n5;
            locals.var_dnm_dn6 = assign78260_body0_e118462_d_n6;
            locals.var_dnm_dn7 = assign78260_body0_e118462_d_n7;
            locals.var_dnm_dn8 = assign78260_body0_e118462_d_n8;
            locals.var_dnm_dn9 = assign78260_body0_e118462_d_n9;
            locals.var_dnm_dn10 = assign78260_body0_e118462_d_n10;
            locals.var_dnm_dn11 = assign78260_body0_e118462_d_n11;
            locals.var_dnm_dn14 = assign78260_body0_e118462_d_n14;
            locals.var_dnm_rv = 0.0;
            let (assign78260_body1_e118477,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1788 == 0.0)) && (locals.var_guard1810 != 0.0)) && (locals.var_guard1811 != 0.0)) && (locals.var_guard1812 != 0.0)) {
        let assign78260_body1_e118475: f64 = (locals.var_m0 + 1.0);
        (assign78260_body1_e118475,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign78260_body1_e118477;
            locals.var_m0_rv = 0.0;
        }

    }

    pub(super) fn stamp_reactive_block_296(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign78270_e118502, assign78270_e118502_d_n0, assign78270_e118502_d_n2, assign78270_e118502_d_n4, assign78270_e118502_d_n5, assign78270_e118502_d_n6, assign78270_e118502_d_n7, assign78270_e118502_d_n8, assign78270_e118502_d_n9, assign78270_e118502_d_n10, assign78270_e118502_d_n11, assign78270_e118502_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1788 == 0.0)) && (locals.var_guard1810 != 0.0)) && (locals.var_guard1811 != 0.0)) && (locals.var_guard1812 == 0.0)) {
        let (assign78270_e118500, assign78270_e118500_d_n0, assign78270_e118500_d_n2, assign78270_e118500_d_n4, assign78270_e118500_d_n5, assign78270_e118500_d_n6, assign78270_e118500_d_n7, assign78270_e118500_d_n8, assign78270_e118500_d_n9, assign78270_e118500_d_n10, assign78270_e118500_d_n11, assign78270_e118500_d_n14,) = {
            if (locals.var_dnm == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign78270_e118497: f64 = (2.0 * 2.0);
                let assign78270_e118498: f64 = (1.0 / assign78270_e118497);
                let assign78270_e118499: f64 = (locals.var_dnm).powf(assign78270_e118498);
                (assign78270_e118499, if 0.0 == 0.0 && ((assign78270_e118498) as f64).is_finite() && ((assign78270_e118498) as f64).fract() == 0.0 { if assign78270_e118498 == 0.0 { 0.0 } else { (assign78270_e118498 * ((locals.var_dnm).powf(assign78270_e118498 - 1.0) * locals.var_dnm_dn0)) } } else { (assign78270_e118499 * (assign78270_e118498 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign78270_e118498) as f64).is_finite() && ((assign78270_e118498) as f64).fract() == 0.0 { if assign78270_e118498 == 0.0 { 0.0 } else { (assign78270_e118498 * ((locals.var_dnm).powf(assign78270_e118498 - 1.0) * locals.var_dnm_dn2)) } } else { (assign78270_e118499 * (assign78270_e118498 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign78270_e118498) as f64).is_finite() && ((assign78270_e118498) as f64).fract() == 0.0 { if assign78270_e118498 == 0.0 { 0.0 } else { (assign78270_e118498 * ((locals.var_dnm).powf(assign78270_e118498 - 1.0) * locals.var_dnm_dn4)) } } else { (assign78270_e118499 * (assign78270_e118498 * (locals.var_dnm_dn4 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign78270_e118498) as f64).is_finite() && ((assign78270_e118498) as f64).fract() == 0.0 { if assign78270_e118498 == 0.0 { 0.0 } else { (assign78270_e118498 * ((locals.var_dnm).powf(assign78270_e118498 - 1.0) * locals.var_dnm_dn5)) } } else { (assign78270_e118499 * (assign78270_e118498 * (locals.var_dnm_dn5 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign78270_e118498) as f64).is_finite() && ((assign78270_e118498) as f64).fract() == 0.0 { if assign78270_e118498 == 0.0 { 0.0 } else { (assign78270_e118498 * ((locals.var_dnm).powf(assign78270_e118498 - 1.0) * locals.var_dnm_dn6)) } } else { (assign78270_e118499 * (assign78270_e118498 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign78270_e118498) as f64).is_finite() && ((assign78270_e118498) as f64).fract() == 0.0 { if assign78270_e118498 == 0.0 { 0.0 } else { (assign78270_e118498 * ((locals.var_dnm).powf(assign78270_e118498 - 1.0) * locals.var_dnm_dn7)) } } else { (assign78270_e118499 * (assign78270_e118498 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign78270_e118498) as f64).is_finite() && ((assign78270_e118498) as f64).fract() == 0.0 { if assign78270_e118498 == 0.0 { 0.0 } else { (assign78270_e118498 * ((locals.var_dnm).powf(assign78270_e118498 - 1.0) * locals.var_dnm_dn8)) } } else { (assign78270_e118499 * (assign78270_e118498 * (locals.var_dnm_dn8 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign78270_e118498) as f64).is_finite() && ((assign78270_e118498) as f64).fract() == 0.0 { if assign78270_e118498 == 0.0 { 0.0 } else { (assign78270_e118498 * ((locals.var_dnm).powf(assign78270_e118498 - 1.0) * locals.var_dnm_dn9)) } } else { (assign78270_e118499 * (assign78270_e118498 * (locals.var_dnm_dn9 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign78270_e118498) as f64).is_finite() && ((assign78270_e118498) as f64).fract() == 0.0 { if assign78270_e118498 == 0.0 { 0.0 } else { (assign78270_e118498 * ((locals.var_dnm).powf(assign78270_e118498 - 1.0) * locals.var_dnm_dn10)) } } else { (assign78270_e118499 * (assign78270_e118498 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign78270_e118498) as f64).is_finite() && ((assign78270_e118498) as f64).fract() == 0.0 { if assign78270_e118498 == 0.0 { 0.0 } else { (assign78270_e118498 * ((locals.var_dnm).powf(assign78270_e118498 - 1.0) * locals.var_dnm_dn11)) } } else { (assign78270_e118499 * (assign78270_e118498 * (locals.var_dnm_dn11 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign78270_e118498) as f64).is_finite() && ((assign78270_e118498) as f64).fract() == 0.0 { if assign78270_e118498 == 0.0 { 0.0 } else { (assign78270_e118498 * ((locals.var_dnm).powf(assign78270_e118498 - 1.0) * locals.var_dnm_dn14)) } } else { (assign78270_e118499 * (assign78270_e118498 * (locals.var_dnm_dn14 / locals.var_dnm))) },)
            }
        };
        (assign78270_e118500, assign78270_e118500_d_n0, assign78270_e118500_d_n2, assign78270_e118500_d_n4, assign78270_e118500_d_n5, assign78270_e118500_d_n6, assign78270_e118500_d_n7, assign78270_e118500_d_n8, assign78270_e118500_d_n9, assign78270_e118500_d_n10, assign78270_e118500_d_n11, assign78270_e118500_d_n14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign78270_e118502;
        locals.var_dnm_dn0 = assign78270_e118502_d_n0;
        locals.var_dnm_dn2 = assign78270_e118502_d_n2;
        locals.var_dnm_dn4 = assign78270_e118502_d_n4;
        locals.var_dnm_dn5 = assign78270_e118502_d_n5;
        locals.var_dnm_dn6 = assign78270_e118502_d_n6;
        locals.var_dnm_dn7 = assign78270_e118502_d_n7;
        locals.var_dnm_dn8 = assign78270_e118502_d_n8;
        locals.var_dnm_dn9 = assign78270_e118502_d_n9;
        locals.var_dnm_dn10 = assign78270_e118502_d_n10;
        locals.var_dnm_dn11 = assign78270_e118502_d_n11;
        locals.var_dnm_dn14 = assign78270_e118502_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign78280_e118515, assign78280_e118515_d_n0, assign78280_e118515_d_n2, assign78280_e118515_d_n4, assign78280_e118515_d_n5, assign78280_e118515_d_n6, assign78280_e118515_d_n7, assign78280_e118515_d_n8, assign78280_e118515_d_n9, assign78280_e118515_d_n10, assign78280_e118515_d_n11, assign78280_e118515_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1788 == 0.0)) && (locals.var_guard1810 != 0.0)) && (locals.var_guard1811 != 0.0)) {
        let assign78280_e118513: f64 = (1.0 / locals.var_dnm);
        (assign78280_e118513, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn4 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn5 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn8 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn9 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn11 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn14 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign78280_e118515;
        locals.var_dnm_dn0 = assign78280_e118515_d_n0;
        locals.var_dnm_dn2 = assign78280_e118515_d_n2;
        locals.var_dnm_dn4 = assign78280_e118515_d_n4;
        locals.var_dnm_dn5 = assign78280_e118515_d_n5;
        locals.var_dnm_dn6 = assign78280_e118515_d_n6;
        locals.var_dnm_dn7 = assign78280_e118515_d_n7;
        locals.var_dnm_dn8 = assign78280_e118515_d_n8;
        locals.var_dnm_dn9 = assign78280_e118515_d_n9;
        locals.var_dnm_dn10 = assign78280_e118515_d_n10;
        locals.var_dnm_dn11 = assign78280_e118515_d_n11;
        locals.var_dnm_dn14 = assign78280_e118515_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign78290_e118532, assign78290_e118532_d_n0, assign78290_e118532_d_n2, assign78290_e118532_d_n4, assign78290_e118532_d_n5, assign78290_e118532_d_n6, assign78290_e118532_d_n7, assign78290_e118532_d_n8, assign78290_e118532_d_n9, assign78290_e118532_d_n10, assign78290_e118532_d_n11, assign78290_e118532_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1788 == 0.0)) && (locals.var_guard1810 != 0.0)) && (locals.var_guard1811 != 0.0)) {
        let assign78290_e118527: f64 = (locals.var_ddriftldc * 0.1);
        let assign78290_e118528: f64 = (locals.var_tmf1 * assign78290_e118527);
        let assign78290_e118530: f64 = (assign78290_e118528 * locals.var_dnm);
        (assign78290_e118530, ((((locals.var_tmf1_dn0 * assign78290_e118527) + (locals.var_tmf1 * (locals.var_ddriftldc_dn0 * 0.1))) * locals.var_dnm) + (assign78290_e118528 * locals.var_dnm_dn0)), ((((locals.var_tmf1_dn2 * assign78290_e118527) + (locals.var_tmf1 * (locals.var_ddriftldc_dn2 * 0.1))) * locals.var_dnm) + (assign78290_e118528 * locals.var_dnm_dn2)), ((((locals.var_tmf1_dn4 * assign78290_e118527) + (locals.var_tmf1 * (locals.var_ddriftldc_dn4 * 0.1))) * locals.var_dnm) + (assign78290_e118528 * locals.var_dnm_dn4)), ((((locals.var_tmf1_dn5 * assign78290_e118527) + (locals.var_tmf1 * (locals.var_ddriftldc_dn5 * 0.1))) * locals.var_dnm) + (assign78290_e118528 * locals.var_dnm_dn5)), ((((locals.var_tmf1_dn6 * assign78290_e118527) + (locals.var_tmf1 * (locals.var_ddriftldc_dn6 * 0.1))) * locals.var_dnm) + (assign78290_e118528 * locals.var_dnm_dn6)), ((((locals.var_tmf1_dn7 * assign78290_e118527) + (locals.var_tmf1 * (locals.var_ddriftldc_dn7 * 0.1))) * locals.var_dnm) + (assign78290_e118528 * locals.var_dnm_dn7)), ((((locals.var_tmf1_dn8 * assign78290_e118527) + (locals.var_tmf1 * (locals.var_ddriftldc_dn8 * 0.1))) * locals.var_dnm) + (assign78290_e118528 * locals.var_dnm_dn8)), ((((locals.var_tmf1_dn9 * assign78290_e118527) + (locals.var_tmf1 * (locals.var_ddriftldc_dn9 * 0.1))) * locals.var_dnm) + (assign78290_e118528 * locals.var_dnm_dn9)), ((((locals.var_tmf1_dn10 * assign78290_e118527) + (locals.var_tmf1 * (locals.var_ddriftldc_dn10 * 0.1))) * locals.var_dnm) + (assign78290_e118528 * locals.var_dnm_dn10)), ((((locals.var_tmf1_dn11 * assign78290_e118527) + (locals.var_tmf1 * (locals.var_ddriftldc_dn11 * 0.1))) * locals.var_dnm) + (assign78290_e118528 * locals.var_dnm_dn11)), ((((locals.var_tmf1_dn14 * assign78290_e118527) + (locals.var_tmf1 * (locals.var_ddriftldc_dn14 * 0.1))) * locals.var_dnm) + (assign78290_e118528 * locals.var_dnm_dn14)),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn11, locals.var_tmf0_dn14,)
    }
};
        locals.var_tmf0 = assign78290_e118532;
        locals.var_tmf0_dn0 = assign78290_e118532_d_n0;
        locals.var_tmf0_dn2 = assign78290_e118532_d_n2;
        locals.var_tmf0_dn4 = assign78290_e118532_d_n4;
        locals.var_tmf0_dn5 = assign78290_e118532_d_n5;
        locals.var_tmf0_dn6 = assign78290_e118532_d_n6;
        locals.var_tmf0_dn7 = assign78290_e118532_d_n7;
        locals.var_tmf0_dn8 = assign78290_e118532_d_n8;
        locals.var_tmf0_dn9 = assign78290_e118532_d_n9;
        locals.var_tmf0_dn10 = assign78290_e118532_d_n10;
        locals.var_tmf0_dn11 = assign78290_e118532_d_n11;
        locals.var_tmf0_dn14 = assign78290_e118532_d_n14;
        locals.var_tmf0_rv = 0.0;

        let (assign78300_e118551, assign78300_e118551_d_n0, assign78300_e118551_d_n2, assign78300_e118551_d_n4, assign78300_e118551_d_n5, assign78300_e118551_d_n6, assign78300_e118551_d_n7, assign78300_e118551_d_n8, assign78300_e118551_d_n9, assign78300_e118551_d_n10, assign78300_e118551_d_n11, assign78300_e118551_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1788 == 0.0)) && (locals.var_guard1810 != 0.0)) && (locals.var_guard1811 != 0.0)) {
        let assign78300_e118543: f64 = (locals.var_ddriftldc * 0.1);
        let assign78300_e118545: f64 = (assign78300_e118543 * locals.var_xmp);
        let assign78300_e118547: f64 = (assign78300_e118545 * locals.var_dnm);
        let assign78300_e118549: f64 = (assign78300_e118547 / locals.var_arg);
        (assign78300_e118549, ((((((((locals.var_ddriftldc_dn0 * 0.1) * locals.var_xmp) + (assign78300_e118543 * locals.var_xmp_dn0)) * locals.var_dnm) + (assign78300_e118545 * locals.var_dnm_dn0)) * locals.var_arg) - (assign78300_e118547 * locals.var_arg_dn0)) / (locals.var_arg * locals.var_arg)), ((((((((locals.var_ddriftldc_dn2 * 0.1) * locals.var_xmp) + (assign78300_e118543 * locals.var_xmp_dn2)) * locals.var_dnm) + (assign78300_e118545 * locals.var_dnm_dn2)) * locals.var_arg) - (assign78300_e118547 * locals.var_arg_dn2)) / (locals.var_arg * locals.var_arg)), ((((((((locals.var_ddriftldc_dn4 * 0.1) * locals.var_xmp) + (assign78300_e118543 * locals.var_xmp_dn4)) * locals.var_dnm) + (assign78300_e118545 * locals.var_dnm_dn4)) * locals.var_arg) - (assign78300_e118547 * locals.var_arg_dn4)) / (locals.var_arg * locals.var_arg)), ((((((((locals.var_ddriftldc_dn5 * 0.1) * locals.var_xmp) + (assign78300_e118543 * locals.var_xmp_dn5)) * locals.var_dnm) + (assign78300_e118545 * locals.var_dnm_dn5)) * locals.var_arg) - (assign78300_e118547 * locals.var_arg_dn5)) / (locals.var_arg * locals.var_arg)), ((((((((locals.var_ddriftldc_dn6 * 0.1) * locals.var_xmp) + (assign78300_e118543 * locals.var_xmp_dn6)) * locals.var_dnm) + (assign78300_e118545 * locals.var_dnm_dn6)) * locals.var_arg) - (assign78300_e118547 * locals.var_arg_dn6)) / (locals.var_arg * locals.var_arg)), ((((((((locals.var_ddriftldc_dn7 * 0.1) * locals.var_xmp) + (assign78300_e118543 * locals.var_xmp_dn7)) * locals.var_dnm) + (assign78300_e118545 * locals.var_dnm_dn7)) * locals.var_arg) - (assign78300_e118547 * locals.var_arg_dn7)) / (locals.var_arg * locals.var_arg)), ((((((((locals.var_ddriftldc_dn8 * 0.1) * locals.var_xmp) + (assign78300_e118543 * locals.var_xmp_dn8)) * locals.var_dnm) + (assign78300_e118545 * locals.var_dnm_dn8)) * locals.var_arg) - (assign78300_e118547 * locals.var_arg_dn8)) / (locals.var_arg * locals.var_arg)), ((((((((locals.var_ddriftldc_dn9 * 0.1) * locals.var_xmp) + (assign78300_e118543 * locals.var_xmp_dn9)) * locals.var_dnm) + (assign78300_e118545 * locals.var_dnm_dn9)) * locals.var_arg) - (assign78300_e118547 * locals.var_arg_dn9)) / (locals.var_arg * locals.var_arg)), ((((((((locals.var_ddriftldc_dn10 * 0.1) * locals.var_xmp) + (assign78300_e118543 * locals.var_xmp_dn10)) * locals.var_dnm) + (assign78300_e118545 * locals.var_dnm_dn10)) * locals.var_arg) - (assign78300_e118547 * locals.var_arg_dn10)) / (locals.var_arg * locals.var_arg)), ((((((((locals.var_ddriftldc_dn11 * 0.1) * locals.var_xmp) + (assign78300_e118543 * locals.var_xmp_dn11)) * locals.var_dnm) + (assign78300_e118545 * locals.var_dnm_dn11)) * locals.var_arg) - (assign78300_e118547 * locals.var_arg_dn11)) / (locals.var_arg * locals.var_arg)), ((((((((locals.var_ddriftldc_dn14 * 0.1) * locals.var_xmp) + (assign78300_e118543 * locals.var_xmp_dn14)) * locals.var_dnm) + (assign78300_e118545 * locals.var_dnm_dn14)) * locals.var_arg) - (assign78300_e118547 * locals.var_arg_dn14)) / (locals.var_arg * locals.var_arg)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign78300_e118551;
        locals.var_t0_dn0 = assign78300_e118551_d_n0;
        locals.var_t0_dn2 = assign78300_e118551_d_n2;
        locals.var_t0_dn4 = assign78300_e118551_d_n4;
        locals.var_t0_dn5 = assign78300_e118551_d_n5;
        locals.var_t0_dn6 = assign78300_e118551_d_n6;
        locals.var_t0_dn7 = assign78300_e118551_d_n7;
        locals.var_t0_dn8 = assign78300_e118551_d_n8;
        locals.var_t0_dn9 = assign78300_e118551_d_n9;
        locals.var_t0_dn10 = assign78300_e118551_d_n10;
        locals.var_t0_dn11 = assign78300_e118551_d_n11;
        locals.var_t0_dn14 = assign78300_e118551_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign78310_e118568, assign78310_e118568_d_n0, assign78310_e118568_d_n2, assign78310_e118568_d_n4, assign78310_e118568_d_n5, assign78310_e118568_d_n6, assign78310_e118568_d_n7, assign78310_e118568_d_n8, assign78310_e118568_d_n9, assign78310_e118568_d_n10, assign78310_e118568_d_n11, assign78310_e118568_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1788 == 0.0)) && (locals.var_guard1810 != 0.0)) && (locals.var_guard1811 != 0.0)) {
        let assign78310_e118563: f64 = (locals.var_ddriftldc * 0.1);
        let assign78310_e118564: f64 = (locals.var_ddriftldc - assign78310_e118563);
        let assign78310_e118566: f64 = (assign78310_e118564 + locals.var_tmf0);
        (assign78310_e118566, ((locals.var_ddriftldc_dn0 - (locals.var_ddriftldc_dn0 * 0.1)) + locals.var_tmf0_dn0), ((locals.var_ddriftldc_dn2 - (locals.var_ddriftldc_dn2 * 0.1)) + locals.var_tmf0_dn2), ((locals.var_ddriftldc_dn4 - (locals.var_ddriftldc_dn4 * 0.1)) + locals.var_tmf0_dn4), ((locals.var_ddriftldc_dn5 - (locals.var_ddriftldc_dn5 * 0.1)) + locals.var_tmf0_dn5), ((locals.var_ddriftldc_dn6 - (locals.var_ddriftldc_dn6 * 0.1)) + locals.var_tmf0_dn6), ((locals.var_ddriftldc_dn7 - (locals.var_ddriftldc_dn7 * 0.1)) + locals.var_tmf0_dn7), ((locals.var_ddriftldc_dn8 - (locals.var_ddriftldc_dn8 * 0.1)) + locals.var_tmf0_dn8), ((locals.var_ddriftldc_dn9 - (locals.var_ddriftldc_dn9 * 0.1)) + locals.var_tmf0_dn9), ((locals.var_ddriftldc_dn10 - (locals.var_ddriftldc_dn10 * 0.1)) + locals.var_tmf0_dn10), ((locals.var_ddriftldc_dn11 - (locals.var_ddriftldc_dn11 * 0.1)) + locals.var_tmf0_dn11), ((locals.var_ddriftldc_dn14 - (locals.var_ddriftldc_dn14 * 0.1)) + locals.var_tmf0_dn14),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign78310_e118568;
        locals.var_t1_dn0 = assign78310_e118568_d_n0;
        locals.var_t1_dn2 = assign78310_e118568_d_n2;
        locals.var_t1_dn4 = assign78310_e118568_d_n4;
        locals.var_t1_dn5 = assign78310_e118568_d_n5;
        locals.var_t1_dn6 = assign78310_e118568_d_n6;
        locals.var_t1_dn7 = assign78310_e118568_d_n7;
        locals.var_t1_dn8 = assign78310_e118568_d_n8;
        locals.var_t1_dn9 = assign78310_e118568_d_n9;
        locals.var_t1_dn10 = assign78310_e118568_d_n10;
        locals.var_t1_dn11 = assign78310_e118568_d_n11;
        locals.var_t1_dn14 = assign78310_e118568_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign78320_e118579, assign78320_e118579_d_n0, assign78320_e118579_d_n2, assign78320_e118579_d_n4, assign78320_e118579_d_n5, assign78320_e118579_d_n6, assign78320_e118579_d_n7, assign78320_e118579_d_n8, assign78320_e118579_d_n9, assign78320_e118579_d_n10, assign78320_e118579_d_n11, assign78320_e118579_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1788 == 0.0)) && (locals.var_guard1810 != 0.0)) && (locals.var_guard1811 != 0.0)) {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign78320_e118579;
        locals.var_t0_dn0 = assign78320_e118579_d_n0;
        locals.var_t0_dn2 = assign78320_e118579_d_n2;
        locals.var_t0_dn4 = assign78320_e118579_d_n4;
        locals.var_t0_dn5 = assign78320_e118579_d_n5;
        locals.var_t0_dn6 = assign78320_e118579_d_n6;
        locals.var_t0_dn7 = assign78320_e118579_d_n7;
        locals.var_t0_dn8 = assign78320_e118579_d_n8;
        locals.var_t0_dn9 = assign78320_e118579_d_n9;
        locals.var_t0_dn10 = assign78320_e118579_d_n10;
        locals.var_t0_dn11 = assign78320_e118579_d_n11;
        locals.var_t0_dn14 = assign78320_e118579_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign78330_e118591, assign78330_e118591_d_n0, assign78330_e118591_d_n2, assign78330_e118591_d_n4, assign78330_e118591_d_n5, assign78330_e118591_d_n6, assign78330_e118591_d_n7, assign78330_e118591_d_n8, assign78330_e118591_d_n9, assign78330_e118591_d_n10, assign78330_e118591_d_n11, assign78330_e118591_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1788 == 0.0)) && (locals.var_guard1810 != 0.0)) && (locals.var_guard1811 == 0.0)) {
        (locals.var_wdld0__blk1808, locals.var_wdld0__blk1808_dn0, locals.var_wdld0__blk1808_dn2, locals.var_wdld0__blk1808_dn4, locals.var_wdld0__blk1808_dn5, locals.var_wdld0__blk1808_dn6, locals.var_wdld0__blk1808_dn7, locals.var_wdld0__blk1808_dn8, locals.var_wdld0__blk1808_dn9, locals.var_wdld0__blk1808_dn10, locals.var_wdld0__blk1808_dn11, locals.var_wdld0__blk1808_dn14,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign78330_e118591;
        locals.var_t1_dn0 = assign78330_e118591_d_n0;
        locals.var_t1_dn2 = assign78330_e118591_d_n2;
        locals.var_t1_dn4 = assign78330_e118591_d_n4;
        locals.var_t1_dn5 = assign78330_e118591_d_n5;
        locals.var_t1_dn6 = assign78330_e118591_d_n6;
        locals.var_t1_dn7 = assign78330_e118591_d_n7;
        locals.var_t1_dn8 = assign78330_e118591_d_n8;
        locals.var_t1_dn9 = assign78330_e118591_d_n9;
        locals.var_t1_dn10 = assign78330_e118591_d_n10;
        locals.var_t1_dn11 = assign78330_e118591_d_n11;
        locals.var_t1_dn14 = assign78330_e118591_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign78340_e118603, assign78340_e118603_d_n0, assign78340_e118603_d_n2, assign78340_e118603_d_n4, assign78340_e118603_d_n5, assign78340_e118603_d_n6, assign78340_e118603_d_n7, assign78340_e118603_d_n8, assign78340_e118603_d_n9, assign78340_e118603_d_n10, assign78340_e118603_d_n11, assign78340_e118603_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1788 == 0.0)) && (locals.var_guard1810 != 0.0)) && (locals.var_guard1811 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign78340_e118603;
        locals.var_t0_dn0 = assign78340_e118603_d_n0;
        locals.var_t0_dn2 = assign78340_e118603_d_n2;
        locals.var_t0_dn4 = assign78340_e118603_d_n4;
        locals.var_t0_dn5 = assign78340_e118603_d_n5;
        locals.var_t0_dn6 = assign78340_e118603_d_n6;
        locals.var_t0_dn7 = assign78340_e118603_d_n7;
        locals.var_t0_dn8 = assign78340_e118603_d_n8;
        locals.var_t0_dn9 = assign78340_e118603_d_n9;
        locals.var_t0_dn10 = assign78340_e118603_d_n10;
        locals.var_t0_dn11 = assign78340_e118603_d_n11;
        locals.var_t0_dn14 = assign78340_e118603_d_n14;
        locals.var_t0_rv = 0.0;

        let assign78350_e118606: f64 = if locals.var_t0 < 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1817 = assign78350_e118606;
        locals.var_guard1817_rv = 0.0;

        let (assign78360_e118619,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1788 == 0.0)) && (locals.var_guard1810 != 0.0)) && (locals.var_guard1817 != 0.0)) {
        let assign78360_e118617: f64 = (locals.var_flg_fd_mode__blk1770 + 2.0);
        (assign78360_e118617,)
    } else {
        (locals.var_flg_fd_mode__blk1770,)
    }
};
        locals.var_flg_fd_mode__blk1770 = assign78360_e118619;
        locals.var_flg_fd_mode__blk1770_rv = 0.0;

        let (assign78370_e118634, assign78370_e118634_d_n0, assign78370_e118634_d_n2, assign78370_e118634_d_n4, assign78370_e118634_d_n5, assign78370_e118634_d_n6, assign78370_e118634_d_n7, assign78370_e118634_d_n8, assign78370_e118634_d_n9, assign78370_e118634_d_n10, assign78370_e118634_d_n11, assign78370_e118634_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1788 == 0.0)) && (locals.var_guard1810 == 0.0)) {
        let (assign78370_e118632, assign78370_e118632_d_n0, assign78370_e118632_d_n2, assign78370_e118632_d_n4, assign78370_e118632_d_n5, assign78370_e118632_d_n6, assign78370_e118632_d_n7, assign78370_e118632_d_n8, assign78370_e118632_d_n9, assign78370_e118632_d_n10, assign78370_e118632_d_n11, assign78370_e118632_d_n14,) = {
            if (locals.var_wdld0__blk1808 <= locals.var_ddriftldc) {
                (locals.var_wdld0__blk1808, locals.var_wdld0__blk1808_dn0, locals.var_wdld0__blk1808_dn2, locals.var_wdld0__blk1808_dn4, locals.var_wdld0__blk1808_dn5, locals.var_wdld0__blk1808_dn6, locals.var_wdld0__blk1808_dn7, locals.var_wdld0__blk1808_dn8, locals.var_wdld0__blk1808_dn9, locals.var_wdld0__blk1808_dn10, locals.var_wdld0__blk1808_dn11, locals.var_wdld0__blk1808_dn14,)
            } else {
                (locals.var_ddriftldc, locals.var_ddriftldc_dn0, locals.var_ddriftldc_dn2, locals.var_ddriftldc_dn4, locals.var_ddriftldc_dn5, locals.var_ddriftldc_dn6, locals.var_ddriftldc_dn7, locals.var_ddriftldc_dn8, locals.var_ddriftldc_dn9, locals.var_ddriftldc_dn10, locals.var_ddriftldc_dn11, locals.var_ddriftldc_dn14,)
            }
        };
        (assign78370_e118632, assign78370_e118632_d_n0, assign78370_e118632_d_n2, assign78370_e118632_d_n4, assign78370_e118632_d_n5, assign78370_e118632_d_n6, assign78370_e118632_d_n7, assign78370_e118632_d_n8, assign78370_e118632_d_n9, assign78370_e118632_d_n10, assign78370_e118632_d_n11, assign78370_e118632_d_n14,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign78370_e118634;
        locals.var_t1_dn0 = assign78370_e118634_d_n0;
        locals.var_t1_dn2 = assign78370_e118634_d_n2;
        locals.var_t1_dn4 = assign78370_e118634_d_n4;
        locals.var_t1_dn5 = assign78370_e118634_d_n5;
        locals.var_t1_dn6 = assign78370_e118634_d_n6;
        locals.var_t1_dn7 = assign78370_e118634_d_n7;
        locals.var_t1_dn8 = assign78370_e118634_d_n8;
        locals.var_t1_dn9 = assign78370_e118634_d_n9;
        locals.var_t1_dn10 = assign78370_e118634_d_n10;
        locals.var_t1_dn11 = assign78370_e118634_d_n11;
        locals.var_t1_dn14 = assign78370_e118634_d_n14;
        locals.var_t1_rv = 0.0;

        let assign78380_e118637: f64 = if locals.var_wdld0__blk1808 >= locals.var_ddriftldc { 1.0 } else { 0.0 };
        locals.var_guard1818 = assign78380_e118637;
        locals.var_guard1818_rv = 0.0;

        let (assign78390_e118651,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1788 == 0.0)) && (locals.var_guard1810 == 0.0)) && (locals.var_guard1818 != 0.0)) {
        let assign78390_e118649: f64 = (locals.var_flg_fd_mode__blk1770 + 2.0);
        (assign78390_e118649,)
    } else {
        (locals.var_flg_fd_mode__blk1770,)
    }
};
        locals.var_flg_fd_mode__blk1770 = assign78390_e118651;
        locals.var_flg_fd_mode__blk1770_rv = 0.0;

        let assign78400_e118654: f64 = if locals.var_flg_fd_mode__blk1770 >= 2.0 { 1.0 } else { 0.0 };
        locals.var_guard1819 = assign78400_e118654;
        locals.var_guard1819_rv = 0.0;

        let (assign78410_e118663, assign78410_e118663_d_n0, assign78410_e118663_d_n2, assign78410_e118663_d_n4, assign78410_e118663_d_n5, assign78410_e118663_d_n6, assign78410_e118663_d_n7, assign78410_e118663_d_n8, assign78410_e118663_d_n9, assign78410_e118663_d_n10, assign78410_e118663_d_n11, assign78410_e118663_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1788 == 0.0)) && (locals.var_guard1819 != 0.0)) {
        (locals.var_ps0ld, locals.var_ps0ld_dn0, locals.var_ps0ld_dn2, locals.var_ps0ld_dn4, locals.var_ps0ld_dn5, locals.var_ps0ld_dn6, locals.var_ps0ld_dn7, locals.var_ps0ld_dn8, locals.var_ps0ld_dn9, locals.var_ps0ld_dn10, locals.var_ps0ld_dn11, locals.var_ps0ld_dn14,)
    } else {
        (locals.var_ps0ld_bef1__blk1809, locals.var_ps0ld_bef1__blk1809_dn0, locals.var_ps0ld_bef1__blk1809_dn2, locals.var_ps0ld_bef1__blk1809_dn4, locals.var_ps0ld_bef1__blk1809_dn5, locals.var_ps0ld_bef1__blk1809_dn6, locals.var_ps0ld_bef1__blk1809_dn7, locals.var_ps0ld_bef1__blk1809_dn8, locals.var_ps0ld_bef1__blk1809_dn9, locals.var_ps0ld_bef1__blk1809_dn10, locals.var_ps0ld_bef1__blk1809_dn11, locals.var_ps0ld_bef1__blk1809_dn14,)
    }
};
        locals.var_ps0ld_bef1__blk1809 = assign78410_e118663;
        locals.var_ps0ld_bef1__blk1809_dn0 = assign78410_e118663_d_n0;
        locals.var_ps0ld_bef1__blk1809_dn2 = assign78410_e118663_d_n2;
        locals.var_ps0ld_bef1__blk1809_dn4 = assign78410_e118663_d_n4;
        locals.var_ps0ld_bef1__blk1809_dn5 = assign78410_e118663_d_n5;
        locals.var_ps0ld_bef1__blk1809_dn6 = assign78410_e118663_d_n6;
        locals.var_ps0ld_bef1__blk1809_dn7 = assign78410_e118663_d_n7;
        locals.var_ps0ld_bef1__blk1809_dn8 = assign78410_e118663_d_n8;
        locals.var_ps0ld_bef1__blk1809_dn9 = assign78410_e118663_d_n9;
        locals.var_ps0ld_bef1__blk1809_dn10 = assign78410_e118663_d_n10;
        locals.var_ps0ld_bef1__blk1809_dn11 = assign78410_e118663_d_n11;
        locals.var_ps0ld_bef1__blk1809_dn14 = assign78410_e118663_d_n14;
        locals.var_ps0ld_bef1__blk1809_rv = 0.0;

        let (assign78420_e118674, assign78420_e118674_d_n0, assign78420_e118674_d_n2, assign78420_e118674_d_n4, assign78420_e118674_d_n5, assign78420_e118674_d_n6, assign78420_e118674_d_n7, assign78420_e118674_d_n8, assign78420_e118674_d_n9, assign78420_e118674_d_n10, assign78420_e118674_d_n11, assign78420_e118674_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1788 == 0.0)) && (locals.var_guard1819 != 0.0)) {
        let assign78420_e118672: f64 = (locals.var_t1 * locals.var_q_nsubld__blk1764);
        (assign78420_e118672, (locals.var_t1_dn0 * locals.var_q_nsubld__blk1764), (locals.var_t1_dn2 * locals.var_q_nsubld__blk1764), (locals.var_t1_dn4 * locals.var_q_nsubld__blk1764), (locals.var_t1_dn5 * locals.var_q_nsubld__blk1764), (locals.var_t1_dn6 * locals.var_q_nsubld__blk1764), (locals.var_t1_dn7 * locals.var_q_nsubld__blk1764), (locals.var_t1_dn8 * locals.var_q_nsubld__blk1764), (locals.var_t1_dn9 * locals.var_q_nsubld__blk1764), (locals.var_t1_dn10 * locals.var_q_nsubld__blk1764), (locals.var_t1_dn11 * locals.var_q_nsubld__blk1764), (locals.var_t1_dn14 * locals.var_q_nsubld__blk1764),)
    } else {
        (locals.var_qbuld, locals.var_qbuld_dn0, locals.var_qbuld_dn2, locals.var_qbuld_dn4, locals.var_qbuld_dn5, locals.var_qbuld_dn6, locals.var_qbuld_dn7, locals.var_qbuld_dn8, locals.var_qbuld_dn9, locals.var_qbuld_dn10, locals.var_qbuld_dn11, locals.var_qbuld_dn14,)
    }
};
        locals.var_qbuld = assign78420_e118674;
        locals.var_qbuld_dn0 = assign78420_e118674_d_n0;
        locals.var_qbuld_dn2 = assign78420_e118674_d_n2;
        locals.var_qbuld_dn4 = assign78420_e118674_d_n4;
        locals.var_qbuld_dn5 = assign78420_e118674_d_n5;
        locals.var_qbuld_dn6 = assign78420_e118674_d_n6;
        locals.var_qbuld_dn7 = assign78420_e118674_d_n7;
        locals.var_qbuld_dn8 = assign78420_e118674_d_n8;
        locals.var_qbuld_dn9 = assign78420_e118674_d_n9;
        locals.var_qbuld_dn10 = assign78420_e118674_d_n10;
        locals.var_qbuld_dn11 = assign78420_e118674_d_n11;
        locals.var_qbuld_dn14 = assign78420_e118674_d_n14;
        locals.var_qbuld_rv = 0.0;

        let (assign78430_e118687, assign78430_e118687_d_n0, assign78430_e118687_d_n2, assign78430_e118687_d_n4, assign78430_e118687_d_n5, assign78430_e118687_d_n6, assign78430_e118687_d_n7, assign78430_e118687_d_n8, assign78430_e118687_d_n9, assign78430_e118687_d_n10, assign78430_e118687_d_n11, assign78430_e118687_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1788 == 0.0)) && (locals.var_guard1819 != 0.0)) {
        let assign78430_e118684: f64 = (locals.var_qbuld / locals.var_cox0_func);
        let assign78430_e118685: f64 = (locals.var_vgpld - assign78430_e118684);
        (assign78430_e118685, (-(locals.var_qbuld_dn0 / locals.var_cox0_func)), (locals.var_vgpld_dn2 - (locals.var_qbuld_dn2 / locals.var_cox0_func)), (-(locals.var_qbuld_dn4 / locals.var_cox0_func)), (-(locals.var_qbuld_dn5 / locals.var_cox0_func)), (-(locals.var_qbuld_dn6 / locals.var_cox0_func)), (locals.var_vgpld_dn7 - (locals.var_qbuld_dn7 / locals.var_cox0_func)), (locals.var_vgpld_dn8 - (locals.var_qbuld_dn8 / locals.var_cox0_func)), (locals.var_vgpld_dn9 - (locals.var_qbuld_dn9 / locals.var_cox0_func)), (-(locals.var_qbuld_dn10 / locals.var_cox0_func)), (-(locals.var_qbuld_dn11 / locals.var_cox0_func)), (-(locals.var_qbuld_dn14 / locals.var_cox0_func)),)
    } else {
        (locals.var_ps0ld, locals.var_ps0ld_dn0, locals.var_ps0ld_dn2, locals.var_ps0ld_dn4, locals.var_ps0ld_dn5, locals.var_ps0ld_dn6, locals.var_ps0ld_dn7, locals.var_ps0ld_dn8, locals.var_ps0ld_dn9, locals.var_ps0ld_dn10, locals.var_ps0ld_dn11, locals.var_ps0ld_dn14,)
    }
};
        locals.var_ps0ld = assign78430_e118687;
        locals.var_ps0ld_dn0 = assign78430_e118687_d_n0;
        locals.var_ps0ld_dn2 = assign78430_e118687_d_n2;
        locals.var_ps0ld_dn4 = assign78430_e118687_d_n4;
        locals.var_ps0ld_dn5 = assign78430_e118687_d_n5;
        locals.var_ps0ld_dn6 = assign78430_e118687_d_n6;
        locals.var_ps0ld_dn7 = assign78430_e118687_d_n7;
        locals.var_ps0ld_dn8 = assign78430_e118687_d_n8;
        locals.var_ps0ld_dn9 = assign78430_e118687_d_n9;
        locals.var_ps0ld_dn10 = assign78430_e118687_d_n10;
        locals.var_ps0ld_dn11 = assign78430_e118687_d_n11;
        locals.var_ps0ld_dn14 = assign78430_e118687_d_n14;
        locals.var_ps0ld_rv = 0.0;

        let assign78440_e118690: f64 = if p.p33 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard1820 = assign78440_e118690;
        locals.var_guard1820_rv = 0.0;

        let assign78450_e118694: f64 = (locals.var_ps0ld_bef1__blk1809 - 0.1);
        let assign78450_e118699: f64 = if ((locals.var_ps0ld > assign78450_e118694) && (0.1 >= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1821 = assign78450_e118699;
        locals.var_guard1821_rv = 0.0;

        let (assign78460_e118716, assign78460_e118716_d_n0, assign78460_e118716_d_n2, assign78460_e118716_d_n4, assign78460_e118716_d_n5, assign78460_e118716_d_n6, assign78460_e118716_d_n7, assign78460_e118716_d_n8, assign78460_e118716_d_n9, assign78460_e118716_d_n10, assign78460_e118716_d_n11, assign78460_e118716_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1788 == 0.0)) && (locals.var_guard1819 != 0.0)) && (locals.var_guard1820 != 0.0)) && (locals.var_guard1821 != 0.0)) {
        let assign78460_e118712: f64 = (locals.var_ps0ld - locals.var_ps0ld_bef1__blk1809);
        let assign78460_e118714: f64 = (assign78460_e118712 + 0.1);
        (assign78460_e118714, (locals.var_ps0ld_dn0 - locals.var_ps0ld_bef1__blk1809_dn0), (locals.var_ps0ld_dn2 - locals.var_ps0ld_bef1__blk1809_dn2), (locals.var_ps0ld_dn4 - locals.var_ps0ld_bef1__blk1809_dn4), (locals.var_ps0ld_dn5 - locals.var_ps0ld_bef1__blk1809_dn5), (locals.var_ps0ld_dn6 - locals.var_ps0ld_bef1__blk1809_dn6), (locals.var_ps0ld_dn7 - locals.var_ps0ld_bef1__blk1809_dn7), (locals.var_ps0ld_dn8 - locals.var_ps0ld_bef1__blk1809_dn8), (locals.var_ps0ld_dn9 - locals.var_ps0ld_bef1__blk1809_dn9), (locals.var_ps0ld_dn10 - locals.var_ps0ld_bef1__blk1809_dn10), (locals.var_ps0ld_dn11 - locals.var_ps0ld_bef1__blk1809_dn11), (locals.var_ps0ld_dn14 - locals.var_ps0ld_bef1__blk1809_dn14),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign78460_e118716;
        locals.var_tmf1_dn0 = assign78460_e118716_d_n0;
        locals.var_tmf1_dn2 = assign78460_e118716_d_n2;
        locals.var_tmf1_dn4 = assign78460_e118716_d_n4;
        locals.var_tmf1_dn5 = assign78460_e118716_d_n5;
        locals.var_tmf1_dn6 = assign78460_e118716_d_n6;
        locals.var_tmf1_dn7 = assign78460_e118716_d_n7;
        locals.var_tmf1_dn8 = assign78460_e118716_d_n8;
        locals.var_tmf1_dn9 = assign78460_e118716_d_n9;
        locals.var_tmf1_dn10 = assign78460_e118716_d_n10;
        locals.var_tmf1_dn11 = assign78460_e118716_d_n11;
        locals.var_tmf1_dn14 = assign78460_e118716_d_n14;
        locals.var_tmf1_rv = 0.0;

        let (assign78470_e118731, assign78470_e118731_d_n0, assign78470_e118731_d_n2, assign78470_e118731_d_n4, assign78470_e118731_d_n5, assign78470_e118731_d_n6, assign78470_e118731_d_n7, assign78470_e118731_d_n8, assign78470_e118731_d_n9, assign78470_e118731_d_n10, assign78470_e118731_d_n11, assign78470_e118731_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1788 == 0.0)) && (locals.var_guard1819 != 0.0)) && (locals.var_guard1820 != 0.0)) && (locals.var_guard1821 != 0.0)) {
        let assign78470_e118729: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign78470_e118729, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)), ((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)), ((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)), ((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn4, locals.var_x2_dn5, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn8, locals.var_x2_dn9, locals.var_x2_dn10, locals.var_x2_dn11, locals.var_x2_dn14,)
    }
};
        locals.var_x2 = assign78470_e118731;
        locals.var_x2_dn0 = assign78470_e118731_d_n0;
        locals.var_x2_dn2 = assign78470_e118731_d_n2;
        locals.var_x2_dn4 = assign78470_e118731_d_n4;
        locals.var_x2_dn5 = assign78470_e118731_d_n5;
        locals.var_x2_dn6 = assign78470_e118731_d_n6;
        locals.var_x2_dn7 = assign78470_e118731_d_n7;
        locals.var_x2_dn8 = assign78470_e118731_d_n8;
        locals.var_x2_dn9 = assign78470_e118731_d_n9;
        locals.var_x2_dn10 = assign78470_e118731_d_n10;
        locals.var_x2_dn11 = assign78470_e118731_d_n11;
        locals.var_x2_dn14 = assign78470_e118731_d_n14;
        locals.var_x2_rv = 0.0;

        let (assign78480_e118746, assign78480_e118746_d_n0, assign78480_e118746_d_n2, assign78480_e118746_d_n4, assign78480_e118746_d_n5, assign78480_e118746_d_n6, assign78480_e118746_d_n7, assign78480_e118746_d_n8, assign78480_e118746_d_n9, assign78480_e118746_d_n10, assign78480_e118746_d_n11, assign78480_e118746_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1788 == 0.0)) && (locals.var_guard1819 != 0.0)) && (locals.var_guard1820 != 0.0)) && (locals.var_guard1821 != 0.0)) {
        let assign78480_e118744: f64 = (0.1 * 0.1);
        (assign78480_e118744, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn4, locals.var_xmax2_dn5, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn8, locals.var_xmax2_dn9, locals.var_xmax2_dn10, locals.var_xmax2_dn11, locals.var_xmax2_dn14,)
    }
};
        locals.var_xmax2 = assign78480_e118746;
        locals.var_xmax2_dn0 = assign78480_e118746_d_n0;
        locals.var_xmax2_dn2 = assign78480_e118746_d_n2;
        locals.var_xmax2_dn4 = assign78480_e118746_d_n4;
        locals.var_xmax2_dn5 = assign78480_e118746_d_n5;
        locals.var_xmax2_dn6 = assign78480_e118746_d_n6;
        locals.var_xmax2_dn7 = assign78480_e118746_d_n7;
        locals.var_xmax2_dn8 = assign78480_e118746_d_n8;
        locals.var_xmax2_dn9 = assign78480_e118746_d_n9;
        locals.var_xmax2_dn10 = assign78480_e118746_d_n10;
        locals.var_xmax2_dn11 = assign78480_e118746_d_n11;
        locals.var_xmax2_dn14 = assign78480_e118746_d_n14;
        locals.var_xmax2_rv = 0.0;

        let (assign78490_e118759, assign78490_e118759_d_n0, assign78490_e118759_d_n2, assign78490_e118759_d_n4, assign78490_e118759_d_n5, assign78490_e118759_d_n6, assign78490_e118759_d_n7, assign78490_e118759_d_n8, assign78490_e118759_d_n9, assign78490_e118759_d_n10, assign78490_e118759_d_n11, assign78490_e118759_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1788 == 0.0)) && (locals.var_guard1819 != 0.0)) && (locals.var_guard1820 != 0.0)) && (locals.var_guard1821 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign78490_e118759;
        locals.var_xp_dn0 = assign78490_e118759_d_n0;
        locals.var_xp_dn2 = assign78490_e118759_d_n2;
        locals.var_xp_dn4 = assign78490_e118759_d_n4;
        locals.var_xp_dn5 = assign78490_e118759_d_n5;
        locals.var_xp_dn6 = assign78490_e118759_d_n6;
        locals.var_xp_dn7 = assign78490_e118759_d_n7;
        locals.var_xp_dn8 = assign78490_e118759_d_n8;
        locals.var_xp_dn9 = assign78490_e118759_d_n9;
        locals.var_xp_dn10 = assign78490_e118759_d_n10;
        locals.var_xp_dn11 = assign78490_e118759_d_n11;
        locals.var_xp_dn14 = assign78490_e118759_d_n14;
        locals.var_xp_rv = 0.0;

        let (assign78500_e118772, assign78500_e118772_d_n0, assign78500_e118772_d_n2, assign78500_e118772_d_n4, assign78500_e118772_d_n5, assign78500_e118772_d_n6, assign78500_e118772_d_n7, assign78500_e118772_d_n8, assign78500_e118772_d_n9, assign78500_e118772_d_n10, assign78500_e118772_d_n11, assign78500_e118772_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1788 == 0.0)) && (locals.var_guard1819 != 0.0)) && (locals.var_guard1820 != 0.0)) && (locals.var_guard1821 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign78500_e118772;
        locals.var_xmp_dn0 = assign78500_e118772_d_n0;
        locals.var_xmp_dn2 = assign78500_e118772_d_n2;
        locals.var_xmp_dn4 = assign78500_e118772_d_n4;
        locals.var_xmp_dn5 = assign78500_e118772_d_n5;
        locals.var_xmp_dn6 = assign78500_e118772_d_n6;
        locals.var_xmp_dn7 = assign78500_e118772_d_n7;
        locals.var_xmp_dn8 = assign78500_e118772_d_n8;
        locals.var_xmp_dn9 = assign78500_e118772_d_n9;
        locals.var_xmp_dn10 = assign78500_e118772_d_n10;
        locals.var_xmp_dn11 = assign78500_e118772_d_n11;
        locals.var_xmp_dn14 = assign78500_e118772_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign78510_e118785,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1788 == 0.0)) && (locals.var_guard1819 != 0.0)) && (locals.var_guard1820 != 0.0)) && (locals.var_guard1821 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign78510_e118785;
        locals.var_m0_rv = 0.0;

        let (assign78520_e118798,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1788 == 0.0)) && (locals.var_guard1819 != 0.0)) && (locals.var_guard1820 != 0.0)) && (locals.var_guard1821 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign78520_e118798;
        locals.var_mm_rv = 0.0;

        let (assign78530_e118811, assign78530_e118811_d_n0, assign78530_e118811_d_n2, assign78530_e118811_d_n4, assign78530_e118811_d_n5, assign78530_e118811_d_n6, assign78530_e118811_d_n7, assign78530_e118811_d_n8, assign78530_e118811_d_n9, assign78530_e118811_d_n10, assign78530_e118811_d_n11, assign78530_e118811_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1788 == 0.0)) && (locals.var_guard1819 != 0.0)) && (locals.var_guard1820 != 0.0)) && (locals.var_guard1821 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign78530_e118811;
        locals.var_arg_dn0 = assign78530_e118811_d_n0;
        locals.var_arg_dn2 = assign78530_e118811_d_n2;
        locals.var_arg_dn4 = assign78530_e118811_d_n4;
        locals.var_arg_dn5 = assign78530_e118811_d_n5;
        locals.var_arg_dn6 = assign78530_e118811_d_n6;
        locals.var_arg_dn7 = assign78530_e118811_d_n7;
        locals.var_arg_dn8 = assign78530_e118811_d_n8;
        locals.var_arg_dn9 = assign78530_e118811_d_n9;
        locals.var_arg_dn10 = assign78530_e118811_d_n10;
        locals.var_arg_dn11 = assign78530_e118811_d_n11;
        locals.var_arg_dn14 = assign78530_e118811_d_n14;
        locals.var_arg_rv = 0.0;

        let (assign78540_e118824, assign78540_e118824_d_n0, assign78540_e118824_d_n2, assign78540_e118824_d_n4, assign78540_e118824_d_n5, assign78540_e118824_d_n6, assign78540_e118824_d_n7, assign78540_e118824_d_n8, assign78540_e118824_d_n9, assign78540_e118824_d_n10, assign78540_e118824_d_n11, assign78540_e118824_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1788 == 0.0)) && (locals.var_guard1819 != 0.0)) && (locals.var_guard1820 != 0.0)) && (locals.var_guard1821 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign78540_e118824;
        locals.var_dnm_dn0 = assign78540_e118824_d_n0;
        locals.var_dnm_dn2 = assign78540_e118824_d_n2;
        locals.var_dnm_dn4 = assign78540_e118824_d_n4;
        locals.var_dnm_dn5 = assign78540_e118824_d_n5;
        locals.var_dnm_dn6 = assign78540_e118824_d_n6;
        locals.var_dnm_dn7 = assign78540_e118824_d_n7;
        locals.var_dnm_dn8 = assign78540_e118824_d_n8;
        locals.var_dnm_dn9 = assign78540_e118824_d_n9;
        locals.var_dnm_dn10 = assign78540_e118824_d_n10;
        locals.var_dnm_dn11 = assign78540_e118824_d_n11;
        locals.var_dnm_dn14 = assign78540_e118824_d_n14;
        locals.var_dnm_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_297(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign78550_e118839, assign78550_e118839_d_n0, assign78550_e118839_d_n2, assign78550_e118839_d_n4, assign78550_e118839_d_n5, assign78550_e118839_d_n6, assign78550_e118839_d_n7, assign78550_e118839_d_n8, assign78550_e118839_d_n9, assign78550_e118839_d_n10, assign78550_e118839_d_n11, assign78550_e118839_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1788 == 0.0)) && (locals.var_guard1819 != 0.0)) && (locals.var_guard1820 != 0.0)) && (locals.var_guard1821 != 0.0)) {
        let assign78550_e118837: f64 = (locals.var_xp * locals.var_x2);
        (assign78550_e118837, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign78550_e118839;
        locals.var_xp_dn0 = assign78550_e118839_d_n0;
        locals.var_xp_dn2 = assign78550_e118839_d_n2;
        locals.var_xp_dn4 = assign78550_e118839_d_n4;
        locals.var_xp_dn5 = assign78550_e118839_d_n5;
        locals.var_xp_dn6 = assign78550_e118839_d_n6;
        locals.var_xp_dn7 = assign78550_e118839_d_n7;
        locals.var_xp_dn8 = assign78550_e118839_d_n8;
        locals.var_xp_dn9 = assign78550_e118839_d_n9;
        locals.var_xp_dn10 = assign78550_e118839_d_n10;
        locals.var_xp_dn11 = assign78550_e118839_d_n11;
        locals.var_xp_dn14 = assign78550_e118839_d_n14;
        locals.var_xp_rv = 0.0;

        let (assign78560_e118854, assign78560_e118854_d_n0, assign78560_e118854_d_n2, assign78560_e118854_d_n4, assign78560_e118854_d_n5, assign78560_e118854_d_n6, assign78560_e118854_d_n7, assign78560_e118854_d_n8, assign78560_e118854_d_n9, assign78560_e118854_d_n10, assign78560_e118854_d_n11, assign78560_e118854_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1788 == 0.0)) && (locals.var_guard1819 != 0.0)) && (locals.var_guard1820 != 0.0)) && (locals.var_guard1821 != 0.0)) {
        let assign78560_e118852: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign78560_e118852, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign78560_e118854;
        locals.var_xmp_dn0 = assign78560_e118854_d_n0;
        locals.var_xmp_dn2 = assign78560_e118854_d_n2;
        locals.var_xmp_dn4 = assign78560_e118854_d_n4;
        locals.var_xmp_dn5 = assign78560_e118854_d_n5;
        locals.var_xmp_dn6 = assign78560_e118854_d_n6;
        locals.var_xmp_dn7 = assign78560_e118854_d_n7;
        locals.var_xmp_dn8 = assign78560_e118854_d_n8;
        locals.var_xmp_dn9 = assign78560_e118854_d_n9;
        locals.var_xmp_dn10 = assign78560_e118854_d_n10;
        locals.var_xmp_dn11 = assign78560_e118854_d_n11;
        locals.var_xmp_dn14 = assign78560_e118854_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign78570_e118869, assign78570_e118869_d_n0, assign78570_e118869_d_n2, assign78570_e118869_d_n4, assign78570_e118869_d_n5, assign78570_e118869_d_n6, assign78570_e118869_d_n7, assign78570_e118869_d_n8, assign78570_e118869_d_n9, assign78570_e118869_d_n10, assign78570_e118869_d_n11, assign78570_e118869_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1788 == 0.0)) && (locals.var_guard1819 != 0.0)) && (locals.var_guard1820 != 0.0)) && (locals.var_guard1821 != 0.0)) {
        let assign78570_e118867: f64 = (locals.var_xp * locals.var_x2);
        (assign78570_e118867, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign78570_e118869;
        locals.var_xp_dn0 = assign78570_e118869_d_n0;
        locals.var_xp_dn2 = assign78570_e118869_d_n2;
        locals.var_xp_dn4 = assign78570_e118869_d_n4;
        locals.var_xp_dn5 = assign78570_e118869_d_n5;
        locals.var_xp_dn6 = assign78570_e118869_d_n6;
        locals.var_xp_dn7 = assign78570_e118869_d_n7;
        locals.var_xp_dn8 = assign78570_e118869_d_n8;
        locals.var_xp_dn9 = assign78570_e118869_d_n9;
        locals.var_xp_dn10 = assign78570_e118869_d_n10;
        locals.var_xp_dn11 = assign78570_e118869_d_n11;
        locals.var_xp_dn14 = assign78570_e118869_d_n14;
        locals.var_xp_rv = 0.0;

        let (assign78580_e118884, assign78580_e118884_d_n0, assign78580_e118884_d_n2, assign78580_e118884_d_n4, assign78580_e118884_d_n5, assign78580_e118884_d_n6, assign78580_e118884_d_n7, assign78580_e118884_d_n8, assign78580_e118884_d_n9, assign78580_e118884_d_n10, assign78580_e118884_d_n11, assign78580_e118884_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1788 == 0.0)) && (locals.var_guard1819 != 0.0)) && (locals.var_guard1820 != 0.0)) && (locals.var_guard1821 != 0.0)) {
        let assign78580_e118882: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign78580_e118882, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign78580_e118884;
        locals.var_xmp_dn0 = assign78580_e118884_d_n0;
        locals.var_xmp_dn2 = assign78580_e118884_d_n2;
        locals.var_xmp_dn4 = assign78580_e118884_d_n4;
        locals.var_xmp_dn5 = assign78580_e118884_d_n5;
        locals.var_xmp_dn6 = assign78580_e118884_d_n6;
        locals.var_xmp_dn7 = assign78580_e118884_d_n7;
        locals.var_xmp_dn8 = assign78580_e118884_d_n8;
        locals.var_xmp_dn9 = assign78580_e118884_d_n9;
        locals.var_xmp_dn10 = assign78580_e118884_d_n10;
        locals.var_xmp_dn11 = assign78580_e118884_d_n11;
        locals.var_xmp_dn14 = assign78580_e118884_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign78590_e118899, assign78590_e118899_d_n0, assign78590_e118899_d_n2, assign78590_e118899_d_n4, assign78590_e118899_d_n5, assign78590_e118899_d_n6, assign78590_e118899_d_n7, assign78590_e118899_d_n8, assign78590_e118899_d_n9, assign78590_e118899_d_n10, assign78590_e118899_d_n11, assign78590_e118899_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1788 == 0.0)) && (locals.var_guard1819 != 0.0)) && (locals.var_guard1820 != 0.0)) && (locals.var_guard1821 != 0.0)) {
        let assign78590_e118897: f64 = (locals.var_xp + locals.var_xmp);
        (assign78590_e118897, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn4 + locals.var_xmp_dn4), (locals.var_xp_dn5 + locals.var_xmp_dn5), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn8 + locals.var_xmp_dn8), (locals.var_xp_dn9 + locals.var_xmp_dn9), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn11 + locals.var_xmp_dn11), (locals.var_xp_dn14 + locals.var_xmp_dn14),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign78590_e118899;
        locals.var_arg_dn0 = assign78590_e118899_d_n0;
        locals.var_arg_dn2 = assign78590_e118899_d_n2;
        locals.var_arg_dn4 = assign78590_e118899_d_n4;
        locals.var_arg_dn5 = assign78590_e118899_d_n5;
        locals.var_arg_dn6 = assign78590_e118899_d_n6;
        locals.var_arg_dn7 = assign78590_e118899_d_n7;
        locals.var_arg_dn8 = assign78590_e118899_d_n8;
        locals.var_arg_dn9 = assign78590_e118899_d_n9;
        locals.var_arg_dn10 = assign78590_e118899_d_n10;
        locals.var_arg_dn11 = assign78590_e118899_d_n11;
        locals.var_arg_dn14 = assign78590_e118899_d_n14;
        locals.var_arg_rv = 0.0;

        let (assign78600_e118912, assign78600_e118912_d_n0, assign78600_e118912_d_n2, assign78600_e118912_d_n4, assign78600_e118912_d_n5, assign78600_e118912_d_n6, assign78600_e118912_d_n7, assign78600_e118912_d_n8, assign78600_e118912_d_n9, assign78600_e118912_d_n10, assign78600_e118912_d_n11, assign78600_e118912_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1788 == 0.0)) && (locals.var_guard1819 != 0.0)) && (locals.var_guard1820 != 0.0)) && (locals.var_guard1821 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign78600_e118912;
        locals.var_dnm_dn0 = assign78600_e118912_d_n0;
        locals.var_dnm_dn2 = assign78600_e118912_d_n2;
        locals.var_dnm_dn4 = assign78600_e118912_d_n4;
        locals.var_dnm_dn5 = assign78600_e118912_d_n5;
        locals.var_dnm_dn6 = assign78600_e118912_d_n6;
        locals.var_dnm_dn7 = assign78600_e118912_d_n7;
        locals.var_dnm_dn8 = assign78600_e118912_d_n8;
        locals.var_dnm_dn9 = assign78600_e118912_d_n9;
        locals.var_dnm_dn10 = assign78600_e118912_d_n10;
        locals.var_dnm_dn11 = assign78600_e118912_d_n11;
        locals.var_dnm_dn14 = assign78600_e118912_d_n14;
        locals.var_dnm_rv = 0.0;

        let assign78610_e118927: f64 = if ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard1822 = assign78610_e118927;
        locals.var_guard1822_rv = 0.0;

        let assign78620_e118930: f64 = if 2.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1823 = assign78620_e118930;
        locals.var_guard1823_rv = 0.0;

        let (assign78630_e118947,) = {
    if (((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1788 == 0.0)) && (locals.var_guard1819 != 0.0)) && (locals.var_guard1820 != 0.0)) && (locals.var_guard1821 != 0.0)) && (locals.var_guard1822 != 0.0)) && (locals.var_guard1823 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign78630_e118947;
        locals.var_mm_rv = 0.0;

        let assign78640_e118950: f64 = if 2.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard1824 = assign78640_e118950;
        locals.var_guard1824_rv = 0.0;

        let (assign78650_e118970,) = {
    if ((((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1788 == 0.0)) && (locals.var_guard1819 != 0.0)) && (locals.var_guard1820 != 0.0)) && (locals.var_guard1821 != 0.0)) && (locals.var_guard1822 != 0.0)) && (locals.var_guard1823 == 0.0)) && (locals.var_guard1824 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign78650_e118970;
        locals.var_mm_rv = 0.0;

        let assign78660_e118973: f64 = if 2.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard1825 = assign78660_e118973;
        locals.var_guard1825_rv = 0.0;

        let (assign78670_e118996,) = {
    if (((((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1788 == 0.0)) && (locals.var_guard1819 != 0.0)) && (locals.var_guard1820 != 0.0)) && (locals.var_guard1821 != 0.0)) && (locals.var_guard1822 != 0.0)) && (locals.var_guard1823 == 0.0)) && (locals.var_guard1824 == 0.0)) && (locals.var_guard1825 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign78670_e118996;
        locals.var_mm_rv = 0.0;

        let assign78680_e118999: f64 = if 2.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard1826 = assign78680_e118999;
        locals.var_guard1826_rv = 0.0;

        let (assign78690_e119025,) = {
    if ((((((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1788 == 0.0)) && (locals.var_guard1819 != 0.0)) && (locals.var_guard1820 != 0.0)) && (locals.var_guard1821 != 0.0)) && (locals.var_guard1822 != 0.0)) && (locals.var_guard1823 == 0.0)) && (locals.var_guard1824 == 0.0)) && (locals.var_guard1825 == 0.0)) && (locals.var_guard1826 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign78690_e119025;
        locals.var_mm_rv = 0.0;

        let (assign78700_e119040,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1788 == 0.0)) && (locals.var_guard1819 != 0.0)) && (locals.var_guard1820 != 0.0)) && (locals.var_guard1821 != 0.0)) && (locals.var_guard1822 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign78700_e119040;
        locals.var_m0_rv = 0.0;

        let mut assign78710_loop_guard: usize = 0;
        while {
            let assign78710_cond_e119056: f64 = if (((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1788 == 0.0)) && (locals.var_guard1819 != 0.0)) && (locals.var_guard1820 != 0.0)) && (locals.var_guard1821 != 0.0)) && (locals.var_guard1822 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign78710_cond_e119056 != 0.0
        } {
            assign78710_loop_guard += 1;
            assert!(assign78710_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign78710_body0_e119072, assign78710_body0_e119072_d_n0, assign78710_body0_e119072_d_n2, assign78710_body0_e119072_d_n4, assign78710_body0_e119072_d_n5, assign78710_body0_e119072_d_n6, assign78710_body0_e119072_d_n7, assign78710_body0_e119072_d_n8, assign78710_body0_e119072_d_n9, assign78710_body0_e119072_d_n10, assign78710_body0_e119072_d_n11, assign78710_body0_e119072_d_n14,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1788 == 0.0)) && (locals.var_guard1819 != 0.0)) && (locals.var_guard1820 != 0.0)) && (locals.var_guard1821 != 0.0)) && (locals.var_guard1822 != 0.0)) {
        let assign78710_body0_e119070: f64 = (locals.var_dnm).sqrt();
        (assign78710_body0_e119070, (locals.var_dnm_dn0 / (2.0 * assign78710_body0_e119070)), (locals.var_dnm_dn2 / (2.0 * assign78710_body0_e119070)), (locals.var_dnm_dn4 / (2.0 * assign78710_body0_e119070)), (locals.var_dnm_dn5 / (2.0 * assign78710_body0_e119070)), (locals.var_dnm_dn6 / (2.0 * assign78710_body0_e119070)), (locals.var_dnm_dn7 / (2.0 * assign78710_body0_e119070)), (locals.var_dnm_dn8 / (2.0 * assign78710_body0_e119070)), (locals.var_dnm_dn9 / (2.0 * assign78710_body0_e119070)), (locals.var_dnm_dn10 / (2.0 * assign78710_body0_e119070)), (locals.var_dnm_dn11 / (2.0 * assign78710_body0_e119070)), (locals.var_dnm_dn14 / (2.0 * assign78710_body0_e119070)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
            locals.var_dnm = assign78710_body0_e119072;
            locals.var_dnm_dn0 = assign78710_body0_e119072_d_n0;
            locals.var_dnm_dn2 = assign78710_body0_e119072_d_n2;
            locals.var_dnm_dn4 = assign78710_body0_e119072_d_n4;
            locals.var_dnm_dn5 = assign78710_body0_e119072_d_n5;
            locals.var_dnm_dn6 = assign78710_body0_e119072_d_n6;
            locals.var_dnm_dn7 = assign78710_body0_e119072_d_n7;
            locals.var_dnm_dn8 = assign78710_body0_e119072_d_n8;
            locals.var_dnm_dn9 = assign78710_body0_e119072_d_n9;
            locals.var_dnm_dn10 = assign78710_body0_e119072_d_n10;
            locals.var_dnm_dn11 = assign78710_body0_e119072_d_n11;
            locals.var_dnm_dn14 = assign78710_body0_e119072_d_n14;
            locals.var_dnm_rv = 0.0;
            let (assign78710_body1_e119089,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1788 == 0.0)) && (locals.var_guard1819 != 0.0)) && (locals.var_guard1820 != 0.0)) && (locals.var_guard1821 != 0.0)) && (locals.var_guard1822 != 0.0)) {
        let assign78710_body1_e119087: f64 = (locals.var_m0 + 1.0);
        (assign78710_body1_e119087,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign78710_body1_e119089;
            locals.var_m0_rv = 0.0;
        }

        let (assign78720_e119116, assign78720_e119116_d_n0, assign78720_e119116_d_n2, assign78720_e119116_d_n4, assign78720_e119116_d_n5, assign78720_e119116_d_n6, assign78720_e119116_d_n7, assign78720_e119116_d_n8, assign78720_e119116_d_n9, assign78720_e119116_d_n10, assign78720_e119116_d_n11, assign78720_e119116_d_n14,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1788 == 0.0)) && (locals.var_guard1819 != 0.0)) && (locals.var_guard1820 != 0.0)) && (locals.var_guard1821 != 0.0)) && (locals.var_guard1822 == 0.0)) {
        let (assign78720_e119114, assign78720_e119114_d_n0, assign78720_e119114_d_n2, assign78720_e119114_d_n4, assign78720_e119114_d_n5, assign78720_e119114_d_n6, assign78720_e119114_d_n7, assign78720_e119114_d_n8, assign78720_e119114_d_n9, assign78720_e119114_d_n10, assign78720_e119114_d_n11, assign78720_e119114_d_n14,) = {
            if (locals.var_dnm == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign78720_e119111: f64 = (2.0 * 2.0);
                let assign78720_e119112: f64 = (1.0 / assign78720_e119111);
                let assign78720_e119113: f64 = (locals.var_dnm).powf(assign78720_e119112);
                (assign78720_e119113, if 0.0 == 0.0 && ((assign78720_e119112) as f64).is_finite() && ((assign78720_e119112) as f64).fract() == 0.0 { if assign78720_e119112 == 0.0 { 0.0 } else { (assign78720_e119112 * ((locals.var_dnm).powf(assign78720_e119112 - 1.0) * locals.var_dnm_dn0)) } } else { (assign78720_e119113 * (assign78720_e119112 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign78720_e119112) as f64).is_finite() && ((assign78720_e119112) as f64).fract() == 0.0 { if assign78720_e119112 == 0.0 { 0.0 } else { (assign78720_e119112 * ((locals.var_dnm).powf(assign78720_e119112 - 1.0) * locals.var_dnm_dn2)) } } else { (assign78720_e119113 * (assign78720_e119112 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign78720_e119112) as f64).is_finite() && ((assign78720_e119112) as f64).fract() == 0.0 { if assign78720_e119112 == 0.0 { 0.0 } else { (assign78720_e119112 * ((locals.var_dnm).powf(assign78720_e119112 - 1.0) * locals.var_dnm_dn4)) } } else { (assign78720_e119113 * (assign78720_e119112 * (locals.var_dnm_dn4 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign78720_e119112) as f64).is_finite() && ((assign78720_e119112) as f64).fract() == 0.0 { if assign78720_e119112 == 0.0 { 0.0 } else { (assign78720_e119112 * ((locals.var_dnm).powf(assign78720_e119112 - 1.0) * locals.var_dnm_dn5)) } } else { (assign78720_e119113 * (assign78720_e119112 * (locals.var_dnm_dn5 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign78720_e119112) as f64).is_finite() && ((assign78720_e119112) as f64).fract() == 0.0 { if assign78720_e119112 == 0.0 { 0.0 } else { (assign78720_e119112 * ((locals.var_dnm).powf(assign78720_e119112 - 1.0) * locals.var_dnm_dn6)) } } else { (assign78720_e119113 * (assign78720_e119112 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign78720_e119112) as f64).is_finite() && ((assign78720_e119112) as f64).fract() == 0.0 { if assign78720_e119112 == 0.0 { 0.0 } else { (assign78720_e119112 * ((locals.var_dnm).powf(assign78720_e119112 - 1.0) * locals.var_dnm_dn7)) } } else { (assign78720_e119113 * (assign78720_e119112 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign78720_e119112) as f64).is_finite() && ((assign78720_e119112) as f64).fract() == 0.0 { if assign78720_e119112 == 0.0 { 0.0 } else { (assign78720_e119112 * ((locals.var_dnm).powf(assign78720_e119112 - 1.0) * locals.var_dnm_dn8)) } } else { (assign78720_e119113 * (assign78720_e119112 * (locals.var_dnm_dn8 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign78720_e119112) as f64).is_finite() && ((assign78720_e119112) as f64).fract() == 0.0 { if assign78720_e119112 == 0.0 { 0.0 } else { (assign78720_e119112 * ((locals.var_dnm).powf(assign78720_e119112 - 1.0) * locals.var_dnm_dn9)) } } else { (assign78720_e119113 * (assign78720_e119112 * (locals.var_dnm_dn9 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign78720_e119112) as f64).is_finite() && ((assign78720_e119112) as f64).fract() == 0.0 { if assign78720_e119112 == 0.0 { 0.0 } else { (assign78720_e119112 * ((locals.var_dnm).powf(assign78720_e119112 - 1.0) * locals.var_dnm_dn10)) } } else { (assign78720_e119113 * (assign78720_e119112 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign78720_e119112) as f64).is_finite() && ((assign78720_e119112) as f64).fract() == 0.0 { if assign78720_e119112 == 0.0 { 0.0 } else { (assign78720_e119112 * ((locals.var_dnm).powf(assign78720_e119112 - 1.0) * locals.var_dnm_dn11)) } } else { (assign78720_e119113 * (assign78720_e119112 * (locals.var_dnm_dn11 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign78720_e119112) as f64).is_finite() && ((assign78720_e119112) as f64).fract() == 0.0 { if assign78720_e119112 == 0.0 { 0.0 } else { (assign78720_e119112 * ((locals.var_dnm).powf(assign78720_e119112 - 1.0) * locals.var_dnm_dn14)) } } else { (assign78720_e119113 * (assign78720_e119112 * (locals.var_dnm_dn14 / locals.var_dnm))) },)
            }
        };
        (assign78720_e119114, assign78720_e119114_d_n0, assign78720_e119114_d_n2, assign78720_e119114_d_n4, assign78720_e119114_d_n5, assign78720_e119114_d_n6, assign78720_e119114_d_n7, assign78720_e119114_d_n8, assign78720_e119114_d_n9, assign78720_e119114_d_n10, assign78720_e119114_d_n11, assign78720_e119114_d_n14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign78720_e119116;
        locals.var_dnm_dn0 = assign78720_e119116_d_n0;
        locals.var_dnm_dn2 = assign78720_e119116_d_n2;
        locals.var_dnm_dn4 = assign78720_e119116_d_n4;
        locals.var_dnm_dn5 = assign78720_e119116_d_n5;
        locals.var_dnm_dn6 = assign78720_e119116_d_n6;
        locals.var_dnm_dn7 = assign78720_e119116_d_n7;
        locals.var_dnm_dn8 = assign78720_e119116_d_n8;
        locals.var_dnm_dn9 = assign78720_e119116_d_n9;
        locals.var_dnm_dn10 = assign78720_e119116_d_n10;
        locals.var_dnm_dn11 = assign78720_e119116_d_n11;
        locals.var_dnm_dn14 = assign78720_e119116_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign78730_e119131, assign78730_e119131_d_n0, assign78730_e119131_d_n2, assign78730_e119131_d_n4, assign78730_e119131_d_n5, assign78730_e119131_d_n6, assign78730_e119131_d_n7, assign78730_e119131_d_n8, assign78730_e119131_d_n9, assign78730_e119131_d_n10, assign78730_e119131_d_n11, assign78730_e119131_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1788 == 0.0)) && (locals.var_guard1819 != 0.0)) && (locals.var_guard1820 != 0.0)) && (locals.var_guard1821 != 0.0)) {
        let assign78730_e119129: f64 = (1.0 / locals.var_dnm);
        (assign78730_e119129, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn4 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn5 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn8 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn9 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn11 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn14 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign78730_e119131;
        locals.var_dnm_dn0 = assign78730_e119131_d_n0;
        locals.var_dnm_dn2 = assign78730_e119131_d_n2;
        locals.var_dnm_dn4 = assign78730_e119131_d_n4;
        locals.var_dnm_dn5 = assign78730_e119131_d_n5;
        locals.var_dnm_dn6 = assign78730_e119131_d_n6;
        locals.var_dnm_dn7 = assign78730_e119131_d_n7;
        locals.var_dnm_dn8 = assign78730_e119131_d_n8;
        locals.var_dnm_dn9 = assign78730_e119131_d_n9;
        locals.var_dnm_dn10 = assign78730_e119131_d_n10;
        locals.var_dnm_dn11 = assign78730_e119131_d_n11;
        locals.var_dnm_dn14 = assign78730_e119131_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign78740_e119148, assign78740_e119148_d_n0, assign78740_e119148_d_n2, assign78740_e119148_d_n4, assign78740_e119148_d_n5, assign78740_e119148_d_n6, assign78740_e119148_d_n7, assign78740_e119148_d_n8, assign78740_e119148_d_n9, assign78740_e119148_d_n10, assign78740_e119148_d_n11, assign78740_e119148_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1788 == 0.0)) && (locals.var_guard1819 != 0.0)) && (locals.var_guard1820 != 0.0)) && (locals.var_guard1821 != 0.0)) {
        let assign78740_e119144: f64 = (locals.var_tmf1 * 0.1);
        let assign78740_e119146: f64 = (assign78740_e119144 * locals.var_dnm);
        (assign78740_e119146, (((locals.var_tmf1_dn0 * 0.1) * locals.var_dnm) + (assign78740_e119144 * locals.var_dnm_dn0)), (((locals.var_tmf1_dn2 * 0.1) * locals.var_dnm) + (assign78740_e119144 * locals.var_dnm_dn2)), (((locals.var_tmf1_dn4 * 0.1) * locals.var_dnm) + (assign78740_e119144 * locals.var_dnm_dn4)), (((locals.var_tmf1_dn5 * 0.1) * locals.var_dnm) + (assign78740_e119144 * locals.var_dnm_dn5)), (((locals.var_tmf1_dn6 * 0.1) * locals.var_dnm) + (assign78740_e119144 * locals.var_dnm_dn6)), (((locals.var_tmf1_dn7 * 0.1) * locals.var_dnm) + (assign78740_e119144 * locals.var_dnm_dn7)), (((locals.var_tmf1_dn8 * 0.1) * locals.var_dnm) + (assign78740_e119144 * locals.var_dnm_dn8)), (((locals.var_tmf1_dn9 * 0.1) * locals.var_dnm) + (assign78740_e119144 * locals.var_dnm_dn9)), (((locals.var_tmf1_dn10 * 0.1) * locals.var_dnm) + (assign78740_e119144 * locals.var_dnm_dn10)), (((locals.var_tmf1_dn11 * 0.1) * locals.var_dnm) + (assign78740_e119144 * locals.var_dnm_dn11)), (((locals.var_tmf1_dn14 * 0.1) * locals.var_dnm) + (assign78740_e119144 * locals.var_dnm_dn14)),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn11, locals.var_tmf0_dn14,)
    }
};
        locals.var_tmf0 = assign78740_e119148;
        locals.var_tmf0_dn0 = assign78740_e119148_d_n0;
        locals.var_tmf0_dn2 = assign78740_e119148_d_n2;
        locals.var_tmf0_dn4 = assign78740_e119148_d_n4;
        locals.var_tmf0_dn5 = assign78740_e119148_d_n5;
        locals.var_tmf0_dn6 = assign78740_e119148_d_n6;
        locals.var_tmf0_dn7 = assign78740_e119148_d_n7;
        locals.var_tmf0_dn8 = assign78740_e119148_d_n8;
        locals.var_tmf0_dn9 = assign78740_e119148_d_n9;
        locals.var_tmf0_dn10 = assign78740_e119148_d_n10;
        locals.var_tmf0_dn11 = assign78740_e119148_d_n11;
        locals.var_tmf0_dn14 = assign78740_e119148_d_n14;
        locals.var_tmf0_rv = 0.0;

        let (assign78750_e119167, assign78750_e119167_d_n0, assign78750_e119167_d_n2, assign78750_e119167_d_n4, assign78750_e119167_d_n5, assign78750_e119167_d_n6, assign78750_e119167_d_n7, assign78750_e119167_d_n8, assign78750_e119167_d_n9, assign78750_e119167_d_n10, assign78750_e119167_d_n11, assign78750_e119167_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1788 == 0.0)) && (locals.var_guard1819 != 0.0)) && (locals.var_guard1820 != 0.0)) && (locals.var_guard1821 != 0.0)) {
        let assign78750_e119161: f64 = (0.1 * locals.var_xmp);
        let assign78750_e119163: f64 = (assign78750_e119161 * locals.var_dnm);
        let assign78750_e119165: f64 = (assign78750_e119163 / locals.var_arg);
        (assign78750_e119165, ((((((0.1 * locals.var_xmp_dn0) * locals.var_dnm) + (assign78750_e119161 * locals.var_dnm_dn0)) * locals.var_arg) - (assign78750_e119163 * locals.var_arg_dn0)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn2) * locals.var_dnm) + (assign78750_e119161 * locals.var_dnm_dn2)) * locals.var_arg) - (assign78750_e119163 * locals.var_arg_dn2)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn4) * locals.var_dnm) + (assign78750_e119161 * locals.var_dnm_dn4)) * locals.var_arg) - (assign78750_e119163 * locals.var_arg_dn4)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn5) * locals.var_dnm) + (assign78750_e119161 * locals.var_dnm_dn5)) * locals.var_arg) - (assign78750_e119163 * locals.var_arg_dn5)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn6) * locals.var_dnm) + (assign78750_e119161 * locals.var_dnm_dn6)) * locals.var_arg) - (assign78750_e119163 * locals.var_arg_dn6)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn7) * locals.var_dnm) + (assign78750_e119161 * locals.var_dnm_dn7)) * locals.var_arg) - (assign78750_e119163 * locals.var_arg_dn7)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn8) * locals.var_dnm) + (assign78750_e119161 * locals.var_dnm_dn8)) * locals.var_arg) - (assign78750_e119163 * locals.var_arg_dn8)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn9) * locals.var_dnm) + (assign78750_e119161 * locals.var_dnm_dn9)) * locals.var_arg) - (assign78750_e119163 * locals.var_arg_dn9)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn10) * locals.var_dnm) + (assign78750_e119161 * locals.var_dnm_dn10)) * locals.var_arg) - (assign78750_e119163 * locals.var_arg_dn10)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn11) * locals.var_dnm) + (assign78750_e119161 * locals.var_dnm_dn11)) * locals.var_arg) - (assign78750_e119163 * locals.var_arg_dn11)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn14) * locals.var_dnm) + (assign78750_e119161 * locals.var_dnm_dn14)) * locals.var_arg) - (assign78750_e119163 * locals.var_arg_dn14)) / (locals.var_arg * locals.var_arg)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign78750_e119167;
        locals.var_t0_dn0 = assign78750_e119167_d_n0;
        locals.var_t0_dn2 = assign78750_e119167_d_n2;
        locals.var_t0_dn4 = assign78750_e119167_d_n4;
        locals.var_t0_dn5 = assign78750_e119167_d_n5;
        locals.var_t0_dn6 = assign78750_e119167_d_n6;
        locals.var_t0_dn7 = assign78750_e119167_d_n7;
        locals.var_t0_dn8 = assign78750_e119167_d_n8;
        locals.var_t0_dn9 = assign78750_e119167_d_n9;
        locals.var_t0_dn10 = assign78750_e119167_d_n10;
        locals.var_t0_dn11 = assign78750_e119167_d_n11;
        locals.var_t0_dn14 = assign78750_e119167_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign78760_e119184, assign78760_e119184_d_n0, assign78760_e119184_d_n2, assign78760_e119184_d_n4, assign78760_e119184_d_n5, assign78760_e119184_d_n6, assign78760_e119184_d_n7, assign78760_e119184_d_n8, assign78760_e119184_d_n9, assign78760_e119184_d_n10, assign78760_e119184_d_n11, assign78760_e119184_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1788 == 0.0)) && (locals.var_guard1819 != 0.0)) && (locals.var_guard1820 != 0.0)) && (locals.var_guard1821 != 0.0)) {
        let assign78760_e119180: f64 = (locals.var_ps0ld_bef1__blk1809 - 0.1);
        let assign78760_e119182: f64 = (assign78760_e119180 + locals.var_tmf0);
        (assign78760_e119182, (locals.var_ps0ld_bef1__blk1809_dn0 + locals.var_tmf0_dn0), (locals.var_ps0ld_bef1__blk1809_dn2 + locals.var_tmf0_dn2), (locals.var_ps0ld_bef1__blk1809_dn4 + locals.var_tmf0_dn4), (locals.var_ps0ld_bef1__blk1809_dn5 + locals.var_tmf0_dn5), (locals.var_ps0ld_bef1__blk1809_dn6 + locals.var_tmf0_dn6), (locals.var_ps0ld_bef1__blk1809_dn7 + locals.var_tmf0_dn7), (locals.var_ps0ld_bef1__blk1809_dn8 + locals.var_tmf0_dn8), (locals.var_ps0ld_bef1__blk1809_dn9 + locals.var_tmf0_dn9), (locals.var_ps0ld_bef1__blk1809_dn10 + locals.var_tmf0_dn10), (locals.var_ps0ld_bef1__blk1809_dn11 + locals.var_tmf0_dn11), (locals.var_ps0ld_bef1__blk1809_dn14 + locals.var_tmf0_dn14),)
    } else {
        (locals.var_ps0ld, locals.var_ps0ld_dn0, locals.var_ps0ld_dn2, locals.var_ps0ld_dn4, locals.var_ps0ld_dn5, locals.var_ps0ld_dn6, locals.var_ps0ld_dn7, locals.var_ps0ld_dn8, locals.var_ps0ld_dn9, locals.var_ps0ld_dn10, locals.var_ps0ld_dn11, locals.var_ps0ld_dn14,)
    }
};
        locals.var_ps0ld = assign78760_e119184;
        locals.var_ps0ld_dn0 = assign78760_e119184_d_n0;
        locals.var_ps0ld_dn2 = assign78760_e119184_d_n2;
        locals.var_ps0ld_dn4 = assign78760_e119184_d_n4;
        locals.var_ps0ld_dn5 = assign78760_e119184_d_n5;
        locals.var_ps0ld_dn6 = assign78760_e119184_d_n6;
        locals.var_ps0ld_dn7 = assign78760_e119184_d_n7;
        locals.var_ps0ld_dn8 = assign78760_e119184_d_n8;
        locals.var_ps0ld_dn9 = assign78760_e119184_d_n9;
        locals.var_ps0ld_dn10 = assign78760_e119184_d_n10;
        locals.var_ps0ld_dn11 = assign78760_e119184_d_n11;
        locals.var_ps0ld_dn14 = assign78760_e119184_d_n14;
        locals.var_ps0ld_rv = 0.0;

        let (assign78770_e119197, assign78770_e119197_d_n0, assign78770_e119197_d_n2, assign78770_e119197_d_n4, assign78770_e119197_d_n5, assign78770_e119197_d_n6, assign78770_e119197_d_n7, assign78770_e119197_d_n8, assign78770_e119197_d_n9, assign78770_e119197_d_n10, assign78770_e119197_d_n11, assign78770_e119197_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1788 == 0.0)) && (locals.var_guard1819 != 0.0)) && (locals.var_guard1820 != 0.0)) && (locals.var_guard1821 != 0.0)) {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign78770_e119197;
        locals.var_t0_dn0 = assign78770_e119197_d_n0;
        locals.var_t0_dn2 = assign78770_e119197_d_n2;
        locals.var_t0_dn4 = assign78770_e119197_d_n4;
        locals.var_t0_dn5 = assign78770_e119197_d_n5;
        locals.var_t0_dn6 = assign78770_e119197_d_n6;
        locals.var_t0_dn7 = assign78770_e119197_d_n7;
        locals.var_t0_dn8 = assign78770_e119197_d_n8;
        locals.var_t0_dn9 = assign78770_e119197_d_n9;
        locals.var_t0_dn10 = assign78770_e119197_d_n10;
        locals.var_t0_dn11 = assign78770_e119197_d_n11;
        locals.var_t0_dn14 = assign78770_e119197_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign78780_e119211, assign78780_e119211_d_n0, assign78780_e119211_d_n2, assign78780_e119211_d_n4, assign78780_e119211_d_n5, assign78780_e119211_d_n6, assign78780_e119211_d_n7, assign78780_e119211_d_n8, assign78780_e119211_d_n9, assign78780_e119211_d_n10, assign78780_e119211_d_n11, assign78780_e119211_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1788 == 0.0)) && (locals.var_guard1819 != 0.0)) && (locals.var_guard1820 != 0.0)) && (locals.var_guard1821 == 0.0)) {
        (locals.var_ps0ld, locals.var_ps0ld_dn0, locals.var_ps0ld_dn2, locals.var_ps0ld_dn4, locals.var_ps0ld_dn5, locals.var_ps0ld_dn6, locals.var_ps0ld_dn7, locals.var_ps0ld_dn8, locals.var_ps0ld_dn9, locals.var_ps0ld_dn10, locals.var_ps0ld_dn11, locals.var_ps0ld_dn14,)
    } else {
        (locals.var_ps0ld, locals.var_ps0ld_dn0, locals.var_ps0ld_dn2, locals.var_ps0ld_dn4, locals.var_ps0ld_dn5, locals.var_ps0ld_dn6, locals.var_ps0ld_dn7, locals.var_ps0ld_dn8, locals.var_ps0ld_dn9, locals.var_ps0ld_dn10, locals.var_ps0ld_dn11, locals.var_ps0ld_dn14,)
    }
};
        locals.var_ps0ld = assign78780_e119211;
        locals.var_ps0ld_dn0 = assign78780_e119211_d_n0;
        locals.var_ps0ld_dn2 = assign78780_e119211_d_n2;
        locals.var_ps0ld_dn4 = assign78780_e119211_d_n4;
        locals.var_ps0ld_dn5 = assign78780_e119211_d_n5;
        locals.var_ps0ld_dn6 = assign78780_e119211_d_n6;
        locals.var_ps0ld_dn7 = assign78780_e119211_d_n7;
        locals.var_ps0ld_dn8 = assign78780_e119211_d_n8;
        locals.var_ps0ld_dn9 = assign78780_e119211_d_n9;
        locals.var_ps0ld_dn10 = assign78780_e119211_d_n10;
        locals.var_ps0ld_dn11 = assign78780_e119211_d_n11;
        locals.var_ps0ld_dn14 = assign78780_e119211_d_n14;
        locals.var_ps0ld_rv = 0.0;

        let (assign78790_e119225, assign78790_e119225_d_n0, assign78790_e119225_d_n2, assign78790_e119225_d_n4, assign78790_e119225_d_n5, assign78790_e119225_d_n6, assign78790_e119225_d_n7, assign78790_e119225_d_n8, assign78790_e119225_d_n9, assign78790_e119225_d_n10, assign78790_e119225_d_n11, assign78790_e119225_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1788 == 0.0)) && (locals.var_guard1819 != 0.0)) && (locals.var_guard1820 != 0.0)) && (locals.var_guard1821 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign78790_e119225;
        locals.var_t0_dn0 = assign78790_e119225_d_n0;
        locals.var_t0_dn2 = assign78790_e119225_d_n2;
        locals.var_t0_dn4 = assign78790_e119225_d_n4;
        locals.var_t0_dn5 = assign78790_e119225_d_n5;
        locals.var_t0_dn6 = assign78790_e119225_d_n6;
        locals.var_t0_dn7 = assign78790_e119225_d_n7;
        locals.var_t0_dn8 = assign78790_e119225_d_n8;
        locals.var_t0_dn9 = assign78790_e119225_d_n9;
        locals.var_t0_dn10 = assign78790_e119225_d_n10;
        locals.var_t0_dn11 = assign78790_e119225_d_n11;
        locals.var_t0_dn14 = assign78790_e119225_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign78800_e119242, assign78800_e119242_d_n0, assign78800_e119242_d_n2, assign78800_e119242_d_n4, assign78800_e119242_d_n5, assign78800_e119242_d_n6, assign78800_e119242_d_n7, assign78800_e119242_d_n8, assign78800_e119242_d_n9, assign78800_e119242_d_n10, assign78800_e119242_d_n11, assign78800_e119242_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1788 == 0.0)) && (locals.var_guard1819 != 0.0)) && (locals.var_guard1820 == 0.0)) {
        let (assign78800_e119240, assign78800_e119240_d_n0, assign78800_e119240_d_n2, assign78800_e119240_d_n4, assign78800_e119240_d_n5, assign78800_e119240_d_n6, assign78800_e119240_d_n7, assign78800_e119240_d_n8, assign78800_e119240_d_n9, assign78800_e119240_d_n10, assign78800_e119240_d_n11, assign78800_e119240_d_n14,) = {
            if (locals.var_ps0ld <= locals.var_ps0ld_bef1__blk1809) {
                (locals.var_ps0ld, locals.var_ps0ld_dn0, locals.var_ps0ld_dn2, locals.var_ps0ld_dn4, locals.var_ps0ld_dn5, locals.var_ps0ld_dn6, locals.var_ps0ld_dn7, locals.var_ps0ld_dn8, locals.var_ps0ld_dn9, locals.var_ps0ld_dn10, locals.var_ps0ld_dn11, locals.var_ps0ld_dn14,)
            } else {
                (locals.var_ps0ld_bef1__blk1809, locals.var_ps0ld_bef1__blk1809_dn0, locals.var_ps0ld_bef1__blk1809_dn2, locals.var_ps0ld_bef1__blk1809_dn4, locals.var_ps0ld_bef1__blk1809_dn5, locals.var_ps0ld_bef1__blk1809_dn6, locals.var_ps0ld_bef1__blk1809_dn7, locals.var_ps0ld_bef1__blk1809_dn8, locals.var_ps0ld_bef1__blk1809_dn9, locals.var_ps0ld_bef1__blk1809_dn10, locals.var_ps0ld_bef1__blk1809_dn11, locals.var_ps0ld_bef1__blk1809_dn14,)
            }
        };
        (assign78800_e119240, assign78800_e119240_d_n0, assign78800_e119240_d_n2, assign78800_e119240_d_n4, assign78800_e119240_d_n5, assign78800_e119240_d_n6, assign78800_e119240_d_n7, assign78800_e119240_d_n8, assign78800_e119240_d_n9, assign78800_e119240_d_n10, assign78800_e119240_d_n11, assign78800_e119240_d_n14,)
    } else {
        (locals.var_ps0ld, locals.var_ps0ld_dn0, locals.var_ps0ld_dn2, locals.var_ps0ld_dn4, locals.var_ps0ld_dn5, locals.var_ps0ld_dn6, locals.var_ps0ld_dn7, locals.var_ps0ld_dn8, locals.var_ps0ld_dn9, locals.var_ps0ld_dn10, locals.var_ps0ld_dn11, locals.var_ps0ld_dn14,)
    }
};
        locals.var_ps0ld = assign78800_e119242;
        locals.var_ps0ld_dn0 = assign78800_e119242_d_n0;
        locals.var_ps0ld_dn2 = assign78800_e119242_d_n2;
        locals.var_ps0ld_dn4 = assign78800_e119242_d_n4;
        locals.var_ps0ld_dn5 = assign78800_e119242_d_n5;
        locals.var_ps0ld_dn6 = assign78800_e119242_d_n6;
        locals.var_ps0ld_dn7 = assign78800_e119242_d_n7;
        locals.var_ps0ld_dn8 = assign78800_e119242_d_n8;
        locals.var_ps0ld_dn9 = assign78800_e119242_d_n9;
        locals.var_ps0ld_dn10 = assign78800_e119242_d_n10;
        locals.var_ps0ld_dn11 = assign78800_e119242_d_n11;
        locals.var_ps0ld_dn14 = assign78800_e119242_d_n14;
        locals.var_ps0ld_rv = 0.0;

        let (assign78810_e119249, assign78810_e119249_d_n0, assign78810_e119249_d_n2, assign78810_e119249_d_n4, assign78810_e119249_d_n5, assign78810_e119249_d_n6, assign78810_e119249_d_n7, assign78810_e119249_d_n8, assign78810_e119249_d_n9, assign78810_e119249_d_n10, assign78810_e119249_d_n11, assign78810_e119249_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1788 == 0.0)) {
        (locals.var_ps0ld, locals.var_ps0ld_dn0, locals.var_ps0ld_dn2, locals.var_ps0ld_dn4, locals.var_ps0ld_dn5, locals.var_ps0ld_dn6, locals.var_ps0ld_dn7, locals.var_ps0ld_dn8, locals.var_ps0ld_dn9, locals.var_ps0ld_dn10, locals.var_ps0ld_dn11, locals.var_ps0ld_dn14,)
    } else {
        (locals.var_ps0ld_ini__blk1771, locals.var_ps0ld_ini__blk1771_dn0, locals.var_ps0ld_ini__blk1771_dn2, locals.var_ps0ld_ini__blk1771_dn4, locals.var_ps0ld_ini__blk1771_dn5, locals.var_ps0ld_ini__blk1771_dn6, locals.var_ps0ld_ini__blk1771_dn7, locals.var_ps0ld_ini__blk1771_dn8, locals.var_ps0ld_ini__blk1771_dn9, locals.var_ps0ld_ini__blk1771_dn10, locals.var_ps0ld_ini__blk1771_dn11, locals.var_ps0ld_ini__blk1771_dn14,)
    }
};
        locals.var_ps0ld_ini__blk1771 = assign78810_e119249;
        locals.var_ps0ld_ini__blk1771_dn0 = assign78810_e119249_d_n0;
        locals.var_ps0ld_ini__blk1771_dn2 = assign78810_e119249_d_n2;
        locals.var_ps0ld_ini__blk1771_dn4 = assign78810_e119249_d_n4;
        locals.var_ps0ld_ini__blk1771_dn5 = assign78810_e119249_d_n5;
        locals.var_ps0ld_ini__blk1771_dn6 = assign78810_e119249_d_n6;
        locals.var_ps0ld_ini__blk1771_dn7 = assign78810_e119249_d_n7;
        locals.var_ps0ld_ini__blk1771_dn8 = assign78810_e119249_d_n8;
        locals.var_ps0ld_ini__blk1771_dn9 = assign78810_e119249_d_n9;
        locals.var_ps0ld_ini__blk1771_dn10 = assign78810_e119249_d_n10;
        locals.var_ps0ld_ini__blk1771_dn11 = assign78810_e119249_d_n11;
        locals.var_ps0ld_ini__blk1771_dn14 = assign78810_e119249_d_n14;
        locals.var_ps0ld_ini__blk1771_rv = 0.0;

        let assign78820_e119252: f64 = if p.p33 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1827 = assign78820_e119252;
        locals.var_guard1827_rv = 0.0;

        let (assign78830_e119261,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1788 == 0.0)) && (locals.var_guard1827 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_flg_conv,)
    }
};
        locals.var_flg_conv = assign78830_e119261;
        locals.var_flg_conv_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_298(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign78840_e119277, assign78840_e119277_d_n0, assign78840_e119277_d_n2, assign78840_e119277_d_n4, assign78840_e119277_d_n5, assign78840_e119277_d_n6, assign78840_e119277_d_n7, assign78840_e119277_d_n8, assign78840_e119277_d_n9, assign78840_e119277_d_n10, assign78840_e119277_d_n11, assign78840_e119277_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1788 == 0.0)) && (locals.var_guard1827 != 0.0)) {
        let assign78840_e119271: f64 = (1.034943e-10 / locals.var_q_nsubld__blk1764);
        let assign78840_e119273: f64 = (assign78840_e119271 * locals.var_beta_inv);
        let assign78840_e119274: f64 = (2.0 * assign78840_e119273);
        let assign78840_e119275: f64 = (assign78840_e119274).sqrt();
        (assign78840_e119275, ((2.0 * (assign78840_e119271 * locals.var_beta_inv_dn0)) / (2.0 * assign78840_e119275)), ((2.0 * (assign78840_e119271 * locals.var_beta_inv_dn2)) / (2.0 * assign78840_e119275)), ((2.0 * (assign78840_e119271 * locals.var_beta_inv_dn4)) / (2.0 * assign78840_e119275)), ((2.0 * (assign78840_e119271 * locals.var_beta_inv_dn5)) / (2.0 * assign78840_e119275)), ((2.0 * (assign78840_e119271 * locals.var_beta_inv_dn6)) / (2.0 * assign78840_e119275)), ((2.0 * (assign78840_e119271 * locals.var_beta_inv_dn7)) / (2.0 * assign78840_e119275)), ((2.0 * (assign78840_e119271 * locals.var_beta_inv_dn8)) / (2.0 * assign78840_e119275)), ((2.0 * (assign78840_e119271 * locals.var_beta_inv_dn9)) / (2.0 * assign78840_e119275)), ((2.0 * (assign78840_e119271 * locals.var_beta_inv_dn10)) / (2.0 * assign78840_e119275)), ((2.0 * (assign78840_e119271 * locals.var_beta_inv_dn11)) / (2.0 * assign78840_e119275)), ((2.0 * (assign78840_e119271 * locals.var_beta_inv_dn14)) / (2.0 * assign78840_e119275)),)
    } else {
        (locals.var_c_w_ld, locals.var_c_w_ld_dn0, locals.var_c_w_ld_dn2, locals.var_c_w_ld_dn4, locals.var_c_w_ld_dn5, locals.var_c_w_ld_dn6, locals.var_c_w_ld_dn7, locals.var_c_w_ld_dn8, locals.var_c_w_ld_dn9, locals.var_c_w_ld_dn10, locals.var_c_w_ld_dn11, locals.var_c_w_ld_dn14,)
    }
};
        locals.var_c_w_ld = assign78840_e119277;
        locals.var_c_w_ld_dn0 = assign78840_e119277_d_n0;
        locals.var_c_w_ld_dn2 = assign78840_e119277_d_n2;
        locals.var_c_w_ld_dn4 = assign78840_e119277_d_n4;
        locals.var_c_w_ld_dn5 = assign78840_e119277_d_n5;
        locals.var_c_w_ld_dn6 = assign78840_e119277_d_n6;
        locals.var_c_w_ld_dn7 = assign78840_e119277_d_n7;
        locals.var_c_w_ld_dn8 = assign78840_e119277_d_n8;
        locals.var_c_w_ld_dn9 = assign78840_e119277_d_n9;
        locals.var_c_w_ld_dn10 = assign78840_e119277_d_n10;
        locals.var_c_w_ld_dn11 = assign78840_e119277_d_n11;
        locals.var_c_w_ld_dn14 = assign78840_e119277_d_n14;
        locals.var_c_w_ld_rv = 0.0;

        let assign78850_e119280: f64 = if locals.var_wdep_func > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1828 = assign78850_e119280;
        locals.var_guard1828_rv = 0.0;

        let (assign78860_e119293, assign78860_e119293_d_n0, assign78860_e119293_d_n2, assign78860_e119293_d_n4, assign78860_e119293_d_n5, assign78860_e119293_d_n6, assign78860_e119293_d_n7, assign78860_e119293_d_n8, assign78860_e119293_d_n9, assign78860_e119293_d_n10, assign78860_e119293_d_n11, assign78860_e119293_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1788 == 0.0)) && (locals.var_guard1827 != 0.0)) && (locals.var_guard1828 != 0.0)) {
        let assign78860_e119291: f64 = (p.p334 - locals.var_wdep_func);
        (assign78860_e119291, (-locals.var_wdep_func_dn0), (-locals.var_wdep_func_dn2), (-locals.var_wdep_func_dn4), (-locals.var_wdep_func_dn5), (-locals.var_wdep_func_dn6), (-locals.var_wdep_func_dn7), (-locals.var_wdep_func_dn8), (-locals.var_wdep_func_dn9), (-locals.var_wdep_func_dn10), (-locals.var_wdep_func_dn11), (-locals.var_wdep_func_dn14),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign78860_e119293;
        locals.var_t2_dn0 = assign78860_e119293_d_n0;
        locals.var_t2_dn2 = assign78860_e119293_d_n2;
        locals.var_t2_dn4 = assign78860_e119293_d_n4;
        locals.var_t2_dn5 = assign78860_e119293_d_n5;
        locals.var_t2_dn6 = assign78860_e119293_d_n6;
        locals.var_t2_dn7 = assign78860_e119293_d_n7;
        locals.var_t2_dn8 = assign78860_e119293_d_n8;
        locals.var_t2_dn9 = assign78860_e119293_d_n9;
        locals.var_t2_dn10 = assign78860_e119293_d_n10;
        locals.var_t2_dn11 = assign78860_e119293_d_n11;
        locals.var_t2_dn14 = assign78860_e119293_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign78870_e119318, assign78870_e119318_d_n0, assign78870_e119318_d_n2, assign78870_e119318_d_n4, assign78870_e119318_d_n5, assign78870_e119318_d_n6, assign78870_e119318_d_n7, assign78870_e119318_d_n8, assign78870_e119318_d_n9, assign78870_e119318_d_n10, assign78870_e119318_d_n11, assign78870_e119318_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1788 == 0.0)) && (locals.var_guard1827 != 0.0)) && (locals.var_guard1828 == 0.0)) {
        let assign78870_e119305: f64 = (locals.var_vdsi + p.p137);
        let assign78870_e119308: f64 = (locals.var_vdsi + p.p137);
        let assign78870_e119309: f64 = (assign78870_e119305 * assign78870_e119308);
        let assign78870_e119312: f64 = (4.0 * 0.1);
        let assign78870_e119314: f64 = (assign78870_e119312 * 0.1);
        let assign78870_e119315: f64 = (assign78870_e119309 + assign78870_e119314);
        let assign78870_e119316: f64 = (assign78870_e119315).sqrt();
        (assign78870_e119316, 0.0, 0.0, 0.0, 0.0, (((locals.var_vdsi_dn6 * assign78870_e119308) + (assign78870_e119305 * locals.var_vdsi_dn6)) / (2.0 * assign78870_e119316)), 0.0, (((locals.var_vdsi_dn8 * assign78870_e119308) + (assign78870_e119305 * locals.var_vdsi_dn8)) / (2.0 * assign78870_e119316)), 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign78870_e119318;
        locals.var_tmf2_dn0 = assign78870_e119318_d_n0;
        locals.var_tmf2_dn2 = assign78870_e119318_d_n2;
        locals.var_tmf2_dn4 = assign78870_e119318_d_n4;
        locals.var_tmf2_dn5 = assign78870_e119318_d_n5;
        locals.var_tmf2_dn6 = assign78870_e119318_d_n6;
        locals.var_tmf2_dn7 = assign78870_e119318_d_n7;
        locals.var_tmf2_dn8 = assign78870_e119318_d_n8;
        locals.var_tmf2_dn9 = assign78870_e119318_d_n9;
        locals.var_tmf2_dn10 = assign78870_e119318_d_n10;
        locals.var_tmf2_dn11 = assign78870_e119318_d_n11;
        locals.var_tmf2_dn14 = assign78870_e119318_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign78880_e119338, assign78880_e119338_d_n0, assign78880_e119338_d_n2, assign78880_e119338_d_n4, assign78880_e119338_d_n5, assign78880_e119338_d_n6, assign78880_e119338_d_n7, assign78880_e119338_d_n8, assign78880_e119338_d_n9, assign78880_e119338_d_n10, assign78880_e119338_d_n11, assign78880_e119338_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1788 == 0.0)) && (locals.var_guard1827 != 0.0)) && (locals.var_guard1828 == 0.0)) {
        let assign78880_e119332: f64 = (locals.var_vdsi + p.p137);
        let assign78880_e119334: f64 = (assign78880_e119332 / locals.var_tmf2);
        let assign78880_e119335: f64 = (1.0 + assign78880_e119334);
        let assign78880_e119336: f64 = (0.5 * assign78880_e119335);
        (assign78880_e119336, (0.5 * (-((assign78880_e119332 * locals.var_tmf2_dn0) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((assign78880_e119332 * locals.var_tmf2_dn2) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((assign78880_e119332 * locals.var_tmf2_dn4) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((assign78880_e119332 * locals.var_tmf2_dn5) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (((locals.var_vdsi_dn6 * locals.var_tmf2) - (assign78880_e119332 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (-((assign78880_e119332 * locals.var_tmf2_dn7) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (((locals.var_vdsi_dn8 * locals.var_tmf2) - (assign78880_e119332 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (-((assign78880_e119332 * locals.var_tmf2_dn9) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((assign78880_e119332 * locals.var_tmf2_dn10) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((assign78880_e119332 * locals.var_tmf2_dn11) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((assign78880_e119332 * locals.var_tmf2_dn14) / (locals.var_tmf2 * locals.var_tmf2)))),)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn11, locals.var_t9_dn14,)
    }
};
        locals.var_t9 = assign78880_e119338;
        locals.var_t9_dn0 = assign78880_e119338_d_n0;
        locals.var_t9_dn2 = assign78880_e119338_d_n2;
        locals.var_t9_dn4 = assign78880_e119338_d_n4;
        locals.var_t9_dn5 = assign78880_e119338_d_n5;
        locals.var_t9_dn6 = assign78880_e119338_d_n6;
        locals.var_t9_dn7 = assign78880_e119338_d_n7;
        locals.var_t9_dn8 = assign78880_e119338_d_n8;
        locals.var_t9_dn9 = assign78880_e119338_d_n9;
        locals.var_t9_dn10 = assign78880_e119338_d_n10;
        locals.var_t9_dn11 = assign78880_e119338_d_n11;
        locals.var_t9_dn14 = assign78880_e119338_d_n14;
        locals.var_t9_rv = 0.0;

        let (assign78890_e119356, assign78890_e119356_d_n0, assign78890_e119356_d_n2, assign78890_e119356_d_n4, assign78890_e119356_d_n5, assign78890_e119356_d_n6, assign78890_e119356_d_n7, assign78890_e119356_d_n8, assign78890_e119356_d_n9, assign78890_e119356_d_n10, assign78890_e119356_d_n11, assign78890_e119356_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1788 == 0.0)) && (locals.var_guard1827 != 0.0)) && (locals.var_guard1828 == 0.0)) {
        let assign78890_e119351: f64 = (locals.var_vdsi + p.p137);
        let assign78890_e119353: f64 = (assign78890_e119351 + locals.var_tmf2);
        let assign78890_e119354: f64 = (0.5 * assign78890_e119353);
        (assign78890_e119354, (0.5 * locals.var_tmf2_dn0), (0.5 * locals.var_tmf2_dn2), (0.5 * locals.var_tmf2_dn4), (0.5 * locals.var_tmf2_dn5), (0.5 * (locals.var_vdsi_dn6 + locals.var_tmf2_dn6)), (0.5 * locals.var_tmf2_dn7), (0.5 * (locals.var_vdsi_dn8 + locals.var_tmf2_dn8)), (0.5 * locals.var_tmf2_dn9), (0.5 * locals.var_tmf2_dn10), (0.5 * locals.var_tmf2_dn11), (0.5 * locals.var_tmf2_dn14),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign78890_e119356;
        locals.var_t2_dn0 = assign78890_e119356_d_n0;
        locals.var_t2_dn2 = assign78890_e119356_d_n2;
        locals.var_t2_dn4 = assign78890_e119356_d_n4;
        locals.var_t2_dn5 = assign78890_e119356_d_n5;
        locals.var_t2_dn6 = assign78890_e119356_d_n6;
        locals.var_t2_dn7 = assign78890_e119356_d_n7;
        locals.var_t2_dn8 = assign78890_e119356_d_n8;
        locals.var_t2_dn9 = assign78890_e119356_d_n9;
        locals.var_t2_dn10 = assign78890_e119356_d_n10;
        locals.var_t2_dn11 = assign78890_e119356_d_n11;
        locals.var_t2_dn14 = assign78890_e119356_d_n14;
        locals.var_t2_rv = 0.0;

        let assign78900_e119359: f64 = if locals.var_t2 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1829 = assign78900_e119359;
        locals.var_guard1829_rv = 0.0;

        let (assign78910_e119373, assign78910_e119373_d_n0, assign78910_e119373_d_n2, assign78910_e119373_d_n4, assign78910_e119373_d_n5, assign78910_e119373_d_n6, assign78910_e119373_d_n7, assign78910_e119373_d_n8, assign78910_e119373_d_n9, assign78910_e119373_d_n10, assign78910_e119373_d_n11, assign78910_e119373_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1788 == 0.0)) && (locals.var_guard1827 != 0.0)) && (locals.var_guard1828 == 0.0)) && (locals.var_guard1829 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign78910_e119373;
        locals.var_t2_dn0 = assign78910_e119373_d_n0;
        locals.var_t2_dn2 = assign78910_e119373_d_n2;
        locals.var_t2_dn4 = assign78910_e119373_d_n4;
        locals.var_t2_dn5 = assign78910_e119373_d_n5;
        locals.var_t2_dn6 = assign78910_e119373_d_n6;
        locals.var_t2_dn7 = assign78910_e119373_d_n7;
        locals.var_t2_dn8 = assign78910_e119373_d_n8;
        locals.var_t2_dn9 = assign78910_e119373_d_n9;
        locals.var_t2_dn10 = assign78910_e119373_d_n10;
        locals.var_t2_dn11 = assign78910_e119373_d_n11;
        locals.var_t2_dn14 = assign78910_e119373_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign78920_e119387, assign78920_e119387_d_n0, assign78920_e119387_d_n2, assign78920_e119387_d_n4, assign78920_e119387_d_n5, assign78920_e119387_d_n6, assign78920_e119387_d_n7, assign78920_e119387_d_n8, assign78920_e119387_d_n9, assign78920_e119387_d_n10, assign78920_e119387_d_n11, assign78920_e119387_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1788 == 0.0)) && (locals.var_guard1827 != 0.0)) && (locals.var_guard1828 == 0.0)) && (locals.var_guard1829 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn11, locals.var_t9_dn14,)
    }
};
        locals.var_t9 = assign78920_e119387;
        locals.var_t9_dn0 = assign78920_e119387_d_n0;
        locals.var_t9_dn2 = assign78920_e119387_d_n2;
        locals.var_t9_dn4 = assign78920_e119387_d_n4;
        locals.var_t9_dn5 = assign78920_e119387_d_n5;
        locals.var_t9_dn6 = assign78920_e119387_d_n6;
        locals.var_t9_dn7 = assign78920_e119387_d_n7;
        locals.var_t9_dn8 = assign78920_e119387_d_n8;
        locals.var_t9_dn9 = assign78920_e119387_d_n9;
        locals.var_t9_dn10 = assign78920_e119387_d_n10;
        locals.var_t9_dn11 = assign78920_e119387_d_n11;
        locals.var_t9_dn14 = assign78920_e119387_d_n14;
        locals.var_t9_rv = 0.0;

        let (assign78930_e119404, assign78930_e119404_d_n0, assign78930_e119404_d_n2, assign78930_e119404_d_n4, assign78930_e119404_d_n5, assign78930_e119404_d_n6, assign78930_e119404_d_n7, assign78930_e119404_d_n8, assign78930_e119404_d_n9, assign78930_e119404_d_n10, assign78930_e119404_d_n11, assign78930_e119404_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1788 == 0.0)) && (locals.var_guard1827 != 0.0)) && (locals.var_guard1828 == 0.0)) {
        let assign78930_e119399: f64 = (locals.var_kjunc * locals.var_t2);
        let assign78930_e119400: f64 = (assign78930_e119399).sqrt();
        let assign78930_e119402: f64 = (assign78930_e119400 * p.p432);
        (assign78930_e119402, ((((locals.var_kjunc_dn0 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn0)) / (2.0 * assign78930_e119400)) * p.p432), ((((locals.var_kjunc_dn2 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn2)) / (2.0 * assign78930_e119400)) * p.p432), ((((locals.var_kjunc_dn4 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn4)) / (2.0 * assign78930_e119400)) * p.p432), ((((locals.var_kjunc_dn5 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn5)) / (2.0 * assign78930_e119400)) * p.p432), ((((locals.var_kjunc_dn6 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn6)) / (2.0 * assign78930_e119400)) * p.p432), ((((locals.var_kjunc_dn7 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn7)) / (2.0 * assign78930_e119400)) * p.p432), ((((locals.var_kjunc_dn8 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn8)) / (2.0 * assign78930_e119400)) * p.p432), ((((locals.var_kjunc_dn9 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn9)) / (2.0 * assign78930_e119400)) * p.p432), ((((locals.var_kjunc_dn10 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn10)) / (2.0 * assign78930_e119400)) * p.p432), ((((locals.var_kjunc_dn11 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn11)) / (2.0 * assign78930_e119400)) * p.p432), ((((locals.var_kjunc_dn14 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn14)) / (2.0 * assign78930_e119400)) * p.p432),)
    } else {
        (locals.var_wjunc0, locals.var_wjunc0_dn0, locals.var_wjunc0_dn2, locals.var_wjunc0_dn4, locals.var_wjunc0_dn5, locals.var_wjunc0_dn6, locals.var_wjunc0_dn7, locals.var_wjunc0_dn8, locals.var_wjunc0_dn9, locals.var_wjunc0_dn10, locals.var_wjunc0_dn11, locals.var_wjunc0_dn14,)
    }
};
        locals.var_wjunc0 = assign78930_e119404;
        locals.var_wjunc0_dn0 = assign78930_e119404_d_n0;
        locals.var_wjunc0_dn2 = assign78930_e119404_d_n2;
        locals.var_wjunc0_dn4 = assign78930_e119404_d_n4;
        locals.var_wjunc0_dn5 = assign78930_e119404_d_n5;
        locals.var_wjunc0_dn6 = assign78930_e119404_d_n6;
        locals.var_wjunc0_dn7 = assign78930_e119404_d_n7;
        locals.var_wjunc0_dn8 = assign78930_e119404_d_n8;
        locals.var_wjunc0_dn9 = assign78930_e119404_d_n9;
        locals.var_wjunc0_dn10 = assign78930_e119404_d_n10;
        locals.var_wjunc0_dn11 = assign78930_e119404_d_n11;
        locals.var_wjunc0_dn14 = assign78930_e119404_d_n14;
        locals.var_wjunc0_rv = 0.0;

        let (assign78940_e119418, assign78940_e119418_d_n0, assign78940_e119418_d_n2, assign78940_e119418_d_n4, assign78940_e119418_d_n5, assign78940_e119418_d_n6, assign78940_e119418_d_n7, assign78940_e119418_d_n8, assign78940_e119418_d_n9, assign78940_e119418_d_n10, assign78940_e119418_d_n11, assign78940_e119418_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1788 == 0.0)) && (locals.var_guard1827 != 0.0)) && (locals.var_guard1828 == 0.0)) {
        let assign78940_e119416: f64 = (p.p334 - locals.var_wjunc0);
        (assign78940_e119416, (-locals.var_wjunc0_dn0), (-locals.var_wjunc0_dn2), (-locals.var_wjunc0_dn4), (-locals.var_wjunc0_dn5), (-locals.var_wjunc0_dn6), (-locals.var_wjunc0_dn7), (-locals.var_wjunc0_dn8), (-locals.var_wjunc0_dn9), (-locals.var_wjunc0_dn10), (-locals.var_wjunc0_dn11), (-locals.var_wjunc0_dn14),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign78940_e119418;
        locals.var_t2_dn0 = assign78940_e119418_d_n0;
        locals.var_t2_dn2 = assign78940_e119418_d_n2;
        locals.var_t2_dn4 = assign78940_e119418_d_n4;
        locals.var_t2_dn5 = assign78940_e119418_d_n5;
        locals.var_t2_dn6 = assign78940_e119418_d_n6;
        locals.var_t2_dn7 = assign78940_e119418_d_n7;
        locals.var_t2_dn8 = assign78940_e119418_d_n8;
        locals.var_t2_dn9 = assign78940_e119418_d_n9;
        locals.var_t2_dn10 = assign78940_e119418_d_n10;
        locals.var_t2_dn11 = assign78940_e119418_d_n11;
        locals.var_t2_dn14 = assign78940_e119418_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign78950_e119440, assign78950_e119440_d_n0, assign78950_e119440_d_n2, assign78950_e119440_d_n4, assign78950_e119440_d_n5, assign78950_e119440_d_n6, assign78950_e119440_d_n7, assign78950_e119440_d_n8, assign78950_e119440_d_n9, assign78950_e119440_d_n10, assign78950_e119440_d_n11, assign78950_e119440_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1788 == 0.0)) && (locals.var_guard1827 != 0.0)) {
        let assign78950_e119427: f64 = (locals.var_t2 * locals.var_t2);
        let assign78950_e119431: f64 = (p.p334 * 0.01);
        let assign78950_e119432: f64 = (4.0 * assign78950_e119431);
        let assign78950_e119435: f64 = (p.p334 * 0.01);
        let assign78950_e119436: f64 = (assign78950_e119432 * assign78950_e119435);
        let assign78950_e119437: f64 = (assign78950_e119427 + assign78950_e119436);
        let assign78950_e119438: f64 = (assign78950_e119437).sqrt();
        (assign78950_e119438, (((locals.var_t2_dn0 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn0)) / (2.0 * assign78950_e119438)), (((locals.var_t2_dn2 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn2)) / (2.0 * assign78950_e119438)), (((locals.var_t2_dn4 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn4)) / (2.0 * assign78950_e119438)), (((locals.var_t2_dn5 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn5)) / (2.0 * assign78950_e119438)), (((locals.var_t2_dn6 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn6)) / (2.0 * assign78950_e119438)), (((locals.var_t2_dn7 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn7)) / (2.0 * assign78950_e119438)), (((locals.var_t2_dn8 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn8)) / (2.0 * assign78950_e119438)), (((locals.var_t2_dn9 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn9)) / (2.0 * assign78950_e119438)), (((locals.var_t2_dn10 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn10)) / (2.0 * assign78950_e119438)), (((locals.var_t2_dn11 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn11)) / (2.0 * assign78950_e119438)), (((locals.var_t2_dn14 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn14)) / (2.0 * assign78950_e119438)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign78950_e119440;
        locals.var_tmf2_dn0 = assign78950_e119440_d_n0;
        locals.var_tmf2_dn2 = assign78950_e119440_d_n2;
        locals.var_tmf2_dn4 = assign78950_e119440_d_n4;
        locals.var_tmf2_dn5 = assign78950_e119440_d_n5;
        locals.var_tmf2_dn6 = assign78950_e119440_d_n6;
        locals.var_tmf2_dn7 = assign78950_e119440_d_n7;
        locals.var_tmf2_dn8 = assign78950_e119440_d_n8;
        locals.var_tmf2_dn9 = assign78950_e119440_d_n9;
        locals.var_tmf2_dn10 = assign78950_e119440_d_n10;
        locals.var_tmf2_dn11 = assign78950_e119440_d_n11;
        locals.var_tmf2_dn14 = assign78950_e119440_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign78960_e119455, assign78960_e119455_d_n0, assign78960_e119455_d_n2, assign78960_e119455_d_n4, assign78960_e119455_d_n5, assign78960_e119455_d_n6, assign78960_e119455_d_n7, assign78960_e119455_d_n8, assign78960_e119455_d_n9, assign78960_e119455_d_n10, assign78960_e119455_d_n11, assign78960_e119455_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1788 == 0.0)) && (locals.var_guard1827 != 0.0)) {
        let assign78960_e119451: f64 = (locals.var_t2 / locals.var_tmf2);
        let assign78960_e119452: f64 = (1.0 + assign78960_e119451);
        let assign78960_e119453: f64 = (0.5 * assign78960_e119452);
        (assign78960_e119453, (0.5 * (((locals.var_t2_dn0 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn2 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn4 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn5 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn6 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn7 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn8 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn9 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn10 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn11 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn14 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn11, locals.var_t9_dn14,)
    }
};
        locals.var_t9 = assign78960_e119455;
        locals.var_t9_dn0 = assign78960_e119455_d_n0;
        locals.var_t9_dn2 = assign78960_e119455_d_n2;
        locals.var_t9_dn4 = assign78960_e119455_d_n4;
        locals.var_t9_dn5 = assign78960_e119455_d_n5;
        locals.var_t9_dn6 = assign78960_e119455_d_n6;
        locals.var_t9_dn7 = assign78960_e119455_d_n7;
        locals.var_t9_dn8 = assign78960_e119455_d_n8;
        locals.var_t9_dn9 = assign78960_e119455_d_n9;
        locals.var_t9_dn10 = assign78960_e119455_d_n10;
        locals.var_t9_dn11 = assign78960_e119455_d_n11;
        locals.var_t9_dn14 = assign78960_e119455_d_n14;
        locals.var_t9_rv = 0.0;

        let (assign78970_e119468, assign78970_e119468_d_n0, assign78970_e119468_d_n2, assign78970_e119468_d_n4, assign78970_e119468_d_n5, assign78970_e119468_d_n6, assign78970_e119468_d_n7, assign78970_e119468_d_n8, assign78970_e119468_d_n9, assign78970_e119468_d_n10, assign78970_e119468_d_n11, assign78970_e119468_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1788 == 0.0)) && (locals.var_guard1827 != 0.0)) {
        let assign78970_e119465: f64 = (locals.var_t2 + locals.var_tmf2);
        let assign78970_e119466: f64 = (0.5 * assign78970_e119465);
        (assign78970_e119466, (0.5 * (locals.var_t2_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_t2_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_t2_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_t2_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_t2_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_t2_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_t2_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_t2_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_t2_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_t2_dn11 + locals.var_tmf2_dn11)), (0.5 * (locals.var_t2_dn14 + locals.var_tmf2_dn14)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign78970_e119468;
        locals.var_t2_dn0 = assign78970_e119468_d_n0;
        locals.var_t2_dn2 = assign78970_e119468_d_n2;
        locals.var_t2_dn4 = assign78970_e119468_d_n4;
        locals.var_t2_dn5 = assign78970_e119468_d_n5;
        locals.var_t2_dn6 = assign78970_e119468_d_n6;
        locals.var_t2_dn7 = assign78970_e119468_d_n7;
        locals.var_t2_dn8 = assign78970_e119468_d_n8;
        locals.var_t2_dn9 = assign78970_e119468_d_n9;
        locals.var_t2_dn10 = assign78970_e119468_d_n10;
        locals.var_t2_dn11 = assign78970_e119468_d_n11;
        locals.var_t2_dn14 = assign78970_e119468_d_n14;
        locals.var_t2_rv = 0.0;

        let assign78980_e119471: f64 = if locals.var_t2 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1830 = assign78980_e119471;
        locals.var_guard1830_rv = 0.0;

        let (assign78990_e119482, assign78990_e119482_d_n0, assign78990_e119482_d_n2, assign78990_e119482_d_n4, assign78990_e119482_d_n5, assign78990_e119482_d_n6, assign78990_e119482_d_n7, assign78990_e119482_d_n8, assign78990_e119482_d_n9, assign78990_e119482_d_n10, assign78990_e119482_d_n11, assign78990_e119482_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1788 == 0.0)) && (locals.var_guard1827 != 0.0)) && (locals.var_guard1830 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign78990_e119482;
        locals.var_t2_dn0 = assign78990_e119482_d_n0;
        locals.var_t2_dn2 = assign78990_e119482_d_n2;
        locals.var_t2_dn4 = assign78990_e119482_d_n4;
        locals.var_t2_dn5 = assign78990_e119482_d_n5;
        locals.var_t2_dn6 = assign78990_e119482_d_n6;
        locals.var_t2_dn7 = assign78990_e119482_d_n7;
        locals.var_t2_dn8 = assign78990_e119482_d_n8;
        locals.var_t2_dn9 = assign78990_e119482_d_n9;
        locals.var_t2_dn10 = assign78990_e119482_d_n10;
        locals.var_t2_dn11 = assign78990_e119482_d_n11;
        locals.var_t2_dn14 = assign78990_e119482_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign79000_e119493, assign79000_e119493_d_n0, assign79000_e119493_d_n2, assign79000_e119493_d_n4, assign79000_e119493_d_n5, assign79000_e119493_d_n6, assign79000_e119493_d_n7, assign79000_e119493_d_n8, assign79000_e119493_d_n9, assign79000_e119493_d_n10, assign79000_e119493_d_n11, assign79000_e119493_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1788 == 0.0)) && (locals.var_guard1827 != 0.0)) && (locals.var_guard1830 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn11, locals.var_t9_dn14,)
    }
};
        locals.var_t9 = assign79000_e119493;
        locals.var_t9_dn0 = assign79000_e119493_d_n0;
        locals.var_t9_dn2 = assign79000_e119493_d_n2;
        locals.var_t9_dn4 = assign79000_e119493_d_n4;
        locals.var_t9_dn5 = assign79000_e119493_d_n5;
        locals.var_t9_dn6 = assign79000_e119493_d_n6;
        locals.var_t9_dn7 = assign79000_e119493_d_n7;
        locals.var_t9_dn8 = assign79000_e119493_d_n8;
        locals.var_t9_dn9 = assign79000_e119493_d_n9;
        locals.var_t9_dn10 = assign79000_e119493_d_n10;
        locals.var_t9_dn11 = assign79000_e119493_d_n11;
        locals.var_t9_dn14 = assign79000_e119493_d_n14;
        locals.var_t9_rv = 0.0;

        let (assign79010_e119502, assign79010_e119502_d_n0, assign79010_e119502_d_n2, assign79010_e119502_d_n4, assign79010_e119502_d_n5, assign79010_e119502_d_n6, assign79010_e119502_d_n7, assign79010_e119502_d_n8, assign79010_e119502_d_n9, assign79010_e119502_d_n10, assign79010_e119502_d_n11, assign79010_e119502_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1788 == 0.0)) && (locals.var_guard1827 != 0.0)) {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    } else {
        (locals.var_ddriftldc, locals.var_ddriftldc_dn0, locals.var_ddriftldc_dn2, locals.var_ddriftldc_dn4, locals.var_ddriftldc_dn5, locals.var_ddriftldc_dn6, locals.var_ddriftldc_dn7, locals.var_ddriftldc_dn8, locals.var_ddriftldc_dn9, locals.var_ddriftldc_dn10, locals.var_ddriftldc_dn11, locals.var_ddriftldc_dn14,)
    }
};
        locals.var_ddriftldc = assign79010_e119502;
        locals.var_ddriftldc_dn0 = assign79010_e119502_d_n0;
        locals.var_ddriftldc_dn2 = assign79010_e119502_d_n2;
        locals.var_ddriftldc_dn4 = assign79010_e119502_d_n4;
        locals.var_ddriftldc_dn5 = assign79010_e119502_d_n5;
        locals.var_ddriftldc_dn6 = assign79010_e119502_d_n6;
        locals.var_ddriftldc_dn7 = assign79010_e119502_d_n7;
        locals.var_ddriftldc_dn8 = assign79010_e119502_d_n8;
        locals.var_ddriftldc_dn9 = assign79010_e119502_d_n9;
        locals.var_ddriftldc_dn10 = assign79010_e119502_d_n10;
        locals.var_ddriftldc_dn11 = assign79010_e119502_d_n11;
        locals.var_ddriftldc_dn14 = assign79010_e119502_d_n14;
        locals.var_ddriftldc_rv = 0.0;

        let (assign79020_e119519, assign79020_e119519_d_n0, assign79020_e119519_d_n2, assign79020_e119519_d_n4, assign79020_e119519_d_n5, assign79020_e119519_d_n6, assign79020_e119519_d_n7, assign79020_e119519_d_n8, assign79020_e119519_d_n9, assign79020_e119519_d_n10, assign79020_e119519_d_n11, assign79020_e119519_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1788 == 0.0)) && (locals.var_guard1827 != 0.0)) {
        let assign79020_e119511: f64 = (locals.var_q_nsubld__blk1764 * locals.var_ddriftldc);
        let assign79020_e119513: f64 = (assign79020_e119511 * locals.var_ddriftldc);
        let assign79020_e119515: f64 = (assign79020_e119513 / 2.0);
        let assign79020_e119517: f64 = (assign79020_e119515 / 1.034943e-10);
        (assign79020_e119517, (((((locals.var_q_nsubld__blk1764 * locals.var_ddriftldc_dn0) * locals.var_ddriftldc) + (assign79020_e119511 * locals.var_ddriftldc_dn0)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk1764 * locals.var_ddriftldc_dn2) * locals.var_ddriftldc) + (assign79020_e119511 * locals.var_ddriftldc_dn2)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk1764 * locals.var_ddriftldc_dn4) * locals.var_ddriftldc) + (assign79020_e119511 * locals.var_ddriftldc_dn4)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk1764 * locals.var_ddriftldc_dn5) * locals.var_ddriftldc) + (assign79020_e119511 * locals.var_ddriftldc_dn5)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk1764 * locals.var_ddriftldc_dn6) * locals.var_ddriftldc) + (assign79020_e119511 * locals.var_ddriftldc_dn6)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk1764 * locals.var_ddriftldc_dn7) * locals.var_ddriftldc) + (assign79020_e119511 * locals.var_ddriftldc_dn7)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk1764 * locals.var_ddriftldc_dn8) * locals.var_ddriftldc) + (assign79020_e119511 * locals.var_ddriftldc_dn8)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk1764 * locals.var_ddriftldc_dn9) * locals.var_ddriftldc) + (assign79020_e119511 * locals.var_ddriftldc_dn9)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk1764 * locals.var_ddriftldc_dn10) * locals.var_ddriftldc) + (assign79020_e119511 * locals.var_ddriftldc_dn10)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk1764 * locals.var_ddriftldc_dn11) * locals.var_ddriftldc) + (assign79020_e119511 * locals.var_ddriftldc_dn11)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk1764 * locals.var_ddriftldc_dn14) * locals.var_ddriftldc) + (assign79020_e119511 * locals.var_ddriftldc_dn14)) / 2.0) / 1.034943e-10),)
    } else {
        (locals.var_dphi_sb, locals.var_dphi_sb_dn0, locals.var_dphi_sb_dn2, locals.var_dphi_sb_dn4, locals.var_dphi_sb_dn5, locals.var_dphi_sb_dn6, locals.var_dphi_sb_dn7, locals.var_dphi_sb_dn8, locals.var_dphi_sb_dn9, locals.var_dphi_sb_dn10, locals.var_dphi_sb_dn11, locals.var_dphi_sb_dn14,)
    }
};
        locals.var_dphi_sb = assign79020_e119519;
        locals.var_dphi_sb_dn0 = assign79020_e119519_d_n0;
        locals.var_dphi_sb_dn2 = assign79020_e119519_d_n2;
        locals.var_dphi_sb_dn4 = assign79020_e119519_d_n4;
        locals.var_dphi_sb_dn5 = assign79020_e119519_d_n5;
        locals.var_dphi_sb_dn6 = assign79020_e119519_d_n6;
        locals.var_dphi_sb_dn7 = assign79020_e119519_d_n7;
        locals.var_dphi_sb_dn8 = assign79020_e119519_d_n8;
        locals.var_dphi_sb_dn9 = assign79020_e119519_d_n9;
        locals.var_dphi_sb_dn10 = assign79020_e119519_d_n10;
        locals.var_dphi_sb_dn11 = assign79020_e119519_d_n11;
        locals.var_dphi_sb_dn14 = assign79020_e119519_d_n14;
        locals.var_dphi_sb_rv = 0.0;

        let (assign79030_e119533, assign79030_e119533_d_n0, assign79030_e119533_d_n2, assign79030_e119533_d_n4, assign79030_e119533_d_n5, assign79030_e119533_d_n6, assign79030_e119533_d_n7, assign79030_e119533_d_n8, assign79030_e119533_d_n9, assign79030_e119533_d_n10, assign79030_e119533_d_n11, assign79030_e119533_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1788 == 0.0)) && (locals.var_guard1827 != 0.0)) {
        let assign79030_e119528: f64 = (2.0 * locals.var_beta);
        let assign79030_e119530: f64 = (assign79030_e119528 * locals.var_dphi_sb);
        let assign79030_e119531: f64 = (assign79030_e119530).sqrt();
        (assign79030_e119531, ((((2.0 * locals.var_beta_dn0) * locals.var_dphi_sb) + (assign79030_e119528 * locals.var_dphi_sb_dn0)) / (2.0 * assign79030_e119531)), ((((2.0 * locals.var_beta_dn2) * locals.var_dphi_sb) + (assign79030_e119528 * locals.var_dphi_sb_dn2)) / (2.0 * assign79030_e119531)), ((((2.0 * locals.var_beta_dn4) * locals.var_dphi_sb) + (assign79030_e119528 * locals.var_dphi_sb_dn4)) / (2.0 * assign79030_e119531)), ((((2.0 * locals.var_beta_dn5) * locals.var_dphi_sb) + (assign79030_e119528 * locals.var_dphi_sb_dn5)) / (2.0 * assign79030_e119531)), ((((2.0 * locals.var_beta_dn6) * locals.var_dphi_sb) + (assign79030_e119528 * locals.var_dphi_sb_dn6)) / (2.0 * assign79030_e119531)), ((((2.0 * locals.var_beta_dn7) * locals.var_dphi_sb) + (assign79030_e119528 * locals.var_dphi_sb_dn7)) / (2.0 * assign79030_e119531)), ((((2.0 * locals.var_beta_dn8) * locals.var_dphi_sb) + (assign79030_e119528 * locals.var_dphi_sb_dn8)) / (2.0 * assign79030_e119531)), ((((2.0 * locals.var_beta_dn9) * locals.var_dphi_sb) + (assign79030_e119528 * locals.var_dphi_sb_dn9)) / (2.0 * assign79030_e119531)), ((((2.0 * locals.var_beta_dn10) * locals.var_dphi_sb) + (assign79030_e119528 * locals.var_dphi_sb_dn10)) / (2.0 * assign79030_e119531)), ((((2.0 * locals.var_beta_dn11) * locals.var_dphi_sb) + (assign79030_e119528 * locals.var_dphi_sb_dn11)) / (2.0 * assign79030_e119531)), ((((2.0 * locals.var_beta_dn14) * locals.var_dphi_sb) + (assign79030_e119528 * locals.var_dphi_sb_dn14)) / (2.0 * assign79030_e119531)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign79030_e119533;
        locals.var_t0_dn0 = assign79030_e119533_d_n0;
        locals.var_t0_dn2 = assign79030_e119533_d_n2;
        locals.var_t0_dn4 = assign79030_e119533_d_n4;
        locals.var_t0_dn5 = assign79030_e119533_d_n5;
        locals.var_t0_dn6 = assign79030_e119533_d_n6;
        locals.var_t0_dn7 = assign79030_e119533_d_n7;
        locals.var_t0_dn8 = assign79030_e119533_d_n8;
        locals.var_t0_dn9 = assign79030_e119533_d_n9;
        locals.var_t0_dn10 = assign79030_e119533_d_n10;
        locals.var_t0_dn11 = assign79030_e119533_d_n11;
        locals.var_t0_dn14 = assign79030_e119533_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign79040_e119549, assign79040_e119549_d_n0, assign79040_e119549_d_n2, assign79040_e119549_d_n4, assign79040_e119549_d_n5, assign79040_e119549_d_n6, assign79040_e119549_d_n7, assign79040_e119549_d_n8, assign79040_e119549_d_n9, assign79040_e119549_d_n10, assign79040_e119549_d_n11, assign79040_e119549_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1788 == 0.0)) && (locals.var_guard1827 != 0.0)) {
        let assign79040_e119541: f64 = { let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign79040_e119543: f64 = (-locals.var_t0);
        let assign79040_e119544: f64 = { let limited_exp_arg = assign79040_e119543; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign79040_e119545: f64 = (assign79040_e119541 + assign79040_e119544);
        let assign79040_e119547: f64 = (assign79040_e119545 / 2.0);
        (assign79040_e119547, ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn0) + ({ let limited_exp_arg = assign79040_e119543; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn0))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn2) + ({ let limited_exp_arg = assign79040_e119543; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn2))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn4) + ({ let limited_exp_arg = assign79040_e119543; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn4))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn5) + ({ let limited_exp_arg = assign79040_e119543; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn5))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn6) + ({ let limited_exp_arg = assign79040_e119543; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn6))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn7) + ({ let limited_exp_arg = assign79040_e119543; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn7))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn8) + ({ let limited_exp_arg = assign79040_e119543; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn8))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn9) + ({ let limited_exp_arg = assign79040_e119543; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn9))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn10) + ({ let limited_exp_arg = assign79040_e119543; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn10))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn11) + ({ let limited_exp_arg = assign79040_e119543; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn11))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn14) + ({ let limited_exp_arg = assign79040_e119543; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn14))) / 2.0),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign79040_e119549;
        locals.var_t1_dn0 = assign79040_e119549_d_n0;
        locals.var_t1_dn2 = assign79040_e119549_d_n2;
        locals.var_t1_dn4 = assign79040_e119549_d_n4;
        locals.var_t1_dn5 = assign79040_e119549_d_n5;
        locals.var_t1_dn6 = assign79040_e119549_d_n6;
        locals.var_t1_dn7 = assign79040_e119549_d_n7;
        locals.var_t1_dn8 = assign79040_e119549_d_n8;
        locals.var_t1_dn9 = assign79040_e119549_d_n9;
        locals.var_t1_dn10 = assign79040_e119549_d_n10;
        locals.var_t1_dn11 = assign79040_e119549_d_n11;
        locals.var_t1_dn14 = assign79040_e119549_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign79050_e119561, assign79050_e119561_d_n0, assign79050_e119561_d_n2, assign79050_e119561_d_n4, assign79050_e119561_d_n5, assign79050_e119561_d_n6, assign79050_e119561_d_n7, assign79050_e119561_d_n8, assign79050_e119561_d_n9, assign79050_e119561_d_n10, assign79050_e119561_d_n11, assign79050_e119561_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1788 == 0.0)) && (locals.var_guard1827 != 0.0)) {
        let assign79050_e119557: f64 = (locals.var_t1).ln();
        let assign79050_e119559: f64 = (assign79050_e119557 / locals.var_dphi_sb);
        (assign79050_e119559, ((((locals.var_t1_dn0 / locals.var_t1) * locals.var_dphi_sb) - (assign79050_e119557 * locals.var_dphi_sb_dn0)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn2 / locals.var_t1) * locals.var_dphi_sb) - (assign79050_e119557 * locals.var_dphi_sb_dn2)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn4 / locals.var_t1) * locals.var_dphi_sb) - (assign79050_e119557 * locals.var_dphi_sb_dn4)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn5 / locals.var_t1) * locals.var_dphi_sb) - (assign79050_e119557 * locals.var_dphi_sb_dn5)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn6 / locals.var_t1) * locals.var_dphi_sb) - (assign79050_e119557 * locals.var_dphi_sb_dn6)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn7 / locals.var_t1) * locals.var_dphi_sb) - (assign79050_e119557 * locals.var_dphi_sb_dn7)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn8 / locals.var_t1) * locals.var_dphi_sb) - (assign79050_e119557 * locals.var_dphi_sb_dn8)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn9 / locals.var_t1) * locals.var_dphi_sb) - (assign79050_e119557 * locals.var_dphi_sb_dn9)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn10 / locals.var_t1) * locals.var_dphi_sb) - (assign79050_e119557 * locals.var_dphi_sb_dn10)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn11 / locals.var_t1) * locals.var_dphi_sb) - (assign79050_e119557 * locals.var_dphi_sb_dn11)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn14 / locals.var_t1) * locals.var_dphi_sb) - (assign79050_e119557 * locals.var_dphi_sb_dn14)) / (locals.var_dphi_sb * locals.var_dphi_sb)),)
    } else {
        (locals.var_c_sb, locals.var_c_sb_dn0, locals.var_c_sb_dn2, locals.var_c_sb_dn4, locals.var_c_sb_dn5, locals.var_c_sb_dn6, locals.var_c_sb_dn7, locals.var_c_sb_dn8, locals.var_c_sb_dn9, locals.var_c_sb_dn10, locals.var_c_sb_dn11, locals.var_c_sb_dn14,)
    }
};
        locals.var_c_sb = assign79050_e119561;
        locals.var_c_sb_dn0 = assign79050_e119561_d_n0;
        locals.var_c_sb_dn2 = assign79050_e119561_d_n2;
        locals.var_c_sb_dn4 = assign79050_e119561_d_n4;
        locals.var_c_sb_dn5 = assign79050_e119561_d_n5;
        locals.var_c_sb_dn6 = assign79050_e119561_d_n6;
        locals.var_c_sb_dn7 = assign79050_e119561_d_n7;
        locals.var_c_sb_dn8 = assign79050_e119561_d_n8;
        locals.var_c_sb_dn9 = assign79050_e119561_d_n9;
        locals.var_c_sb_dn10 = assign79050_e119561_d_n10;
        locals.var_c_sb_dn11 = assign79050_e119561_d_n11;
        locals.var_c_sb_dn14 = assign79050_e119561_d_n14;
        locals.var_c_sb_rv = 0.0;

        let (assign79060_e119570,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1788 == 0.0)) && (locals.var_guard1827 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_lp_s0,)
    }
};
        locals.var_lp_s0 = assign79060_e119570;
        locals.var_lp_s0_rv = 0.0;

    }
}
