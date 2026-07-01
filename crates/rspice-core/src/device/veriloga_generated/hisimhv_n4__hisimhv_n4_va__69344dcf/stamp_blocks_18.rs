#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_transient_block_288(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign82720_e125810, assign82720_e125810_d_n0, assign82720_e125810_d_n2, assign82720_e125810_d_n4, assign82720_e125810_d_n5, assign82720_e125810_d_n6, assign82720_e125810_d_n7, assign82720_e125810_d_n8, assign82720_e125810_d_n9, assign82720_e125810_d_n10, assign82720_e125810_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1911 != 0.0)) && (locals.var_guard1913 != 0.0)) && (locals.var_guard1914 != 0.0)) {
        let assign82720_e125805: f64 = (0.2 * locals.var_chi_b);
        let assign82720_e125806: f64 = (locals.var_chi_b - assign82720_e125805);
        let assign82720_e125808: f64 = (assign82720_e125806 + locals.var_tmf0);
        (assign82720_e125808, ((locals.var_chi_b_dn0 - (0.2 * locals.var_chi_b_dn0)) + locals.var_tmf0_dn0), ((locals.var_chi_b_dn2 - (0.2 * locals.var_chi_b_dn2)) + locals.var_tmf0_dn2), ((locals.var_chi_b_dn4 - (0.2 * locals.var_chi_b_dn4)) + locals.var_tmf0_dn4), ((locals.var_chi_b_dn5 - (0.2 * locals.var_chi_b_dn5)) + locals.var_tmf0_dn5), ((locals.var_chi_b_dn6 - (0.2 * locals.var_chi_b_dn6)) + locals.var_tmf0_dn6), ((locals.var_chi_b_dn7 - (0.2 * locals.var_chi_b_dn7)) + locals.var_tmf0_dn7), ((locals.var_chi_b_dn8 - (0.2 * locals.var_chi_b_dn8)) + locals.var_tmf0_dn8), ((locals.var_chi_b_dn9 - (0.2 * locals.var_chi_b_dn9)) + locals.var_tmf0_dn9), ((locals.var_chi_b_dn10 - (0.2 * locals.var_chi_b_dn10)) + locals.var_tmf0_dn10), ((locals.var_chi_b_dn13 - (0.2 * locals.var_chi_b_dn13)) + locals.var_tmf0_dn13),)
    } else {
        (locals.var_chi, locals.var_chi_dn0, locals.var_chi_dn2, locals.var_chi_dn4, locals.var_chi_dn5, locals.var_chi_dn6, locals.var_chi_dn7, locals.var_chi_dn8, locals.var_chi_dn9, locals.var_chi_dn10, locals.var_chi_dn13,)
    }
};
        locals.var_chi = assign82720_e125810;
        locals.var_chi_dn0 = assign82720_e125810_d_n0;
        locals.var_chi_dn2 = assign82720_e125810_d_n2;
        locals.var_chi_dn4 = assign82720_e125810_d_n4;
        locals.var_chi_dn5 = assign82720_e125810_d_n5;
        locals.var_chi_dn6 = assign82720_e125810_d_n6;
        locals.var_chi_dn7 = assign82720_e125810_d_n7;
        locals.var_chi_dn8 = assign82720_e125810_d_n8;
        locals.var_chi_dn9 = assign82720_e125810_d_n9;
        locals.var_chi_dn10 = assign82720_e125810_d_n10;
        locals.var_chi_dn13 = assign82720_e125810_d_n13;

        let (assign82730_e125823, assign82730_e125823_d_n0, assign82730_e125823_d_n2, assign82730_e125823_d_n4, assign82730_e125823_d_n5, assign82730_e125823_d_n6, assign82730_e125823_d_n7, assign82730_e125823_d_n8, assign82730_e125823_d_n9, assign82730_e125823_d_n10, assign82730_e125823_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1911 != 0.0)) && (locals.var_guard1913 != 0.0)) && (locals.var_guard1914 != 0.0)) {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign82730_e125823;
        locals.var_t1_dn0 = assign82730_e125823_d_n0;
        locals.var_t1_dn2 = assign82730_e125823_d_n2;
        locals.var_t1_dn4 = assign82730_e125823_d_n4;
        locals.var_t1_dn5 = assign82730_e125823_d_n5;
        locals.var_t1_dn6 = assign82730_e125823_d_n6;
        locals.var_t1_dn7 = assign82730_e125823_d_n7;
        locals.var_t1_dn8 = assign82730_e125823_d_n8;
        locals.var_t1_dn9 = assign82730_e125823_d_n9;
        locals.var_t1_dn10 = assign82730_e125823_d_n10;
        locals.var_t1_dn13 = assign82730_e125823_d_n13;

        let (assign82740_e125837, assign82740_e125837_d_n0, assign82740_e125837_d_n2, assign82740_e125837_d_n4, assign82740_e125837_d_n5, assign82740_e125837_d_n6, assign82740_e125837_d_n7, assign82740_e125837_d_n8, assign82740_e125837_d_n9, assign82740_e125837_d_n10, assign82740_e125837_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1911 != 0.0)) && (locals.var_guard1913 != 0.0)) && (locals.var_guard1914 == 0.0)) {
        (locals.var_chi_a, locals.var_chi_a_dn0, locals.var_chi_a_dn2, locals.var_chi_a_dn4, locals.var_chi_a_dn5, locals.var_chi_a_dn6, locals.var_chi_a_dn7, locals.var_chi_a_dn8, locals.var_chi_a_dn9, locals.var_chi_a_dn10, locals.var_chi_a_dn13,)
    } else {
        (locals.var_chi, locals.var_chi_dn0, locals.var_chi_dn2, locals.var_chi_dn4, locals.var_chi_dn5, locals.var_chi_dn6, locals.var_chi_dn7, locals.var_chi_dn8, locals.var_chi_dn9, locals.var_chi_dn10, locals.var_chi_dn13,)
    }
};
        locals.var_chi = assign82740_e125837;
        locals.var_chi_dn0 = assign82740_e125837_d_n0;
        locals.var_chi_dn2 = assign82740_e125837_d_n2;
        locals.var_chi_dn4 = assign82740_e125837_d_n4;
        locals.var_chi_dn5 = assign82740_e125837_d_n5;
        locals.var_chi_dn6 = assign82740_e125837_d_n6;
        locals.var_chi_dn7 = assign82740_e125837_d_n7;
        locals.var_chi_dn8 = assign82740_e125837_d_n8;
        locals.var_chi_dn9 = assign82740_e125837_d_n9;
        locals.var_chi_dn10 = assign82740_e125837_d_n10;
        locals.var_chi_dn13 = assign82740_e125837_d_n13;

        let (assign82750_e125851, assign82750_e125851_d_n0, assign82750_e125851_d_n2, assign82750_e125851_d_n4, assign82750_e125851_d_n5, assign82750_e125851_d_n6, assign82750_e125851_d_n7, assign82750_e125851_d_n8, assign82750_e125851_d_n9, assign82750_e125851_d_n10, assign82750_e125851_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1911 != 0.0)) && (locals.var_guard1913 != 0.0)) && (locals.var_guard1914 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign82750_e125851;
        locals.var_t1_dn0 = assign82750_e125851_d_n0;
        locals.var_t1_dn2 = assign82750_e125851_d_n2;
        locals.var_t1_dn4 = assign82750_e125851_d_n4;
        locals.var_t1_dn5 = assign82750_e125851_d_n5;
        locals.var_t1_dn6 = assign82750_e125851_d_n6;
        locals.var_t1_dn7 = assign82750_e125851_d_n7;
        locals.var_t1_dn8 = assign82750_e125851_d_n8;
        locals.var_t1_dn9 = assign82750_e125851_d_n9;
        locals.var_t1_dn10 = assign82750_e125851_d_n10;
        locals.var_t1_dn13 = assign82750_e125851_d_n13;

        let (assign82760_e125868, assign82760_e125868_d_n0, assign82760_e125868_d_n2, assign82760_e125868_d_n4, assign82760_e125868_d_n5, assign82760_e125868_d_n6, assign82760_e125868_d_n7, assign82760_e125868_d_n8, assign82760_e125868_d_n9, assign82760_e125868_d_n10, assign82760_e125868_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1911 != 0.0)) && (locals.var_guard1913 == 0.0)) {
        let (assign82760_e125866, assign82760_e125866_d_n0, assign82760_e125866_d_n2, assign82760_e125866_d_n4, assign82760_e125866_d_n5, assign82760_e125866_d_n6, assign82760_e125866_d_n7, assign82760_e125866_d_n8, assign82760_e125866_d_n9, assign82760_e125866_d_n10, assign82760_e125866_d_n13,) = {
            if (locals.var_chi_a <= locals.var_chi_b) {
                (locals.var_chi_a, locals.var_chi_a_dn0, locals.var_chi_a_dn2, locals.var_chi_a_dn4, locals.var_chi_a_dn5, locals.var_chi_a_dn6, locals.var_chi_a_dn7, locals.var_chi_a_dn8, locals.var_chi_a_dn9, locals.var_chi_a_dn10, locals.var_chi_a_dn13,)
            } else {
                (locals.var_chi_b, locals.var_chi_b_dn0, locals.var_chi_b_dn2, locals.var_chi_b_dn4, locals.var_chi_b_dn5, locals.var_chi_b_dn6, locals.var_chi_b_dn7, locals.var_chi_b_dn8, locals.var_chi_b_dn9, locals.var_chi_b_dn10, locals.var_chi_b_dn13,)
            }
        };
        (assign82760_e125866, assign82760_e125866_d_n0, assign82760_e125866_d_n2, assign82760_e125866_d_n4, assign82760_e125866_d_n5, assign82760_e125866_d_n6, assign82760_e125866_d_n7, assign82760_e125866_d_n8, assign82760_e125866_d_n9, assign82760_e125866_d_n10, assign82760_e125866_d_n13,)
    } else {
        (locals.var_chi, locals.var_chi_dn0, locals.var_chi_dn2, locals.var_chi_dn4, locals.var_chi_dn5, locals.var_chi_dn6, locals.var_chi_dn7, locals.var_chi_dn8, locals.var_chi_dn9, locals.var_chi_dn10, locals.var_chi_dn13,)
    }
};
        locals.var_chi = assign82760_e125868;
        locals.var_chi_dn0 = assign82760_e125868_d_n0;
        locals.var_chi_dn2 = assign82760_e125868_d_n2;
        locals.var_chi_dn4 = assign82760_e125868_d_n4;
        locals.var_chi_dn5 = assign82760_e125868_d_n5;
        locals.var_chi_dn6 = assign82760_e125868_d_n6;
        locals.var_chi_dn7 = assign82760_e125868_d_n7;
        locals.var_chi_dn8 = assign82760_e125868_d_n8;
        locals.var_chi_dn9 = assign82760_e125868_d_n9;
        locals.var_chi_dn10 = assign82760_e125868_d_n10;
        locals.var_chi_dn13 = assign82760_e125868_d_n13;

        let assign82770_e125871: f64 = if p.p33 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1920 = assign82770_e125871;

        let (assign82780_e125884, assign82780_e125884_d_n0, assign82780_e125884_d_n2, assign82780_e125884_d_n4, assign82780_e125884_d_n5, assign82780_e125884_d_n6, assign82780_e125884_d_n7, assign82780_e125884_d_n8, assign82780_e125884_d_n9, assign82780_e125884_d_n10, assign82780_e125884_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1920 != 0.0)) {
        let assign82780_e125880: f64 = (locals.var_chi * locals.var_beta_inv);
        let assign82780_e125882: f64 = (assign82780_e125880 - locals.var_vxbgmtcl);
        (assign82780_e125882, (((locals.var_chi_dn0 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn0)) - locals.var_vxbgmtcl_dn0), (((locals.var_chi_dn2 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn2)) - locals.var_vxbgmtcl_dn2), (((locals.var_chi_dn4 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn4)) - locals.var_vxbgmtcl_dn4), (((locals.var_chi_dn5 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn5)) - locals.var_vxbgmtcl_dn5), (((locals.var_chi_dn6 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn6)) - locals.var_vxbgmtcl_dn6), (((locals.var_chi_dn7 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn7)) - locals.var_vxbgmtcl_dn7), (((locals.var_chi_dn8 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn8)) - locals.var_vxbgmtcl_dn8), (((locals.var_chi_dn9 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn9)) - locals.var_vxbgmtcl_dn9), (((locals.var_chi_dn10 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn10)) - locals.var_vxbgmtcl_dn10), (((locals.var_chi_dn13 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn13)) - locals.var_vxbgmtcl_dn13),)
    } else {
        (locals.var_ps0ld, locals.var_ps0ld_dn0, locals.var_ps0ld_dn2, locals.var_ps0ld_dn4, locals.var_ps0ld_dn5, locals.var_ps0ld_dn6, locals.var_ps0ld_dn7, locals.var_ps0ld_dn8, locals.var_ps0ld_dn9, locals.var_ps0ld_dn10, locals.var_ps0ld_dn13,)
    }
};
        locals.var_ps0ld = assign82780_e125884;
        locals.var_ps0ld_dn0 = assign82780_e125884_d_n0;
        locals.var_ps0ld_dn2 = assign82780_e125884_d_n2;
        locals.var_ps0ld_dn4 = assign82780_e125884_d_n4;
        locals.var_ps0ld_dn5 = assign82780_e125884_d_n5;
        locals.var_ps0ld_dn6 = assign82780_e125884_d_n6;
        locals.var_ps0ld_dn7 = assign82780_e125884_d_n7;
        locals.var_ps0ld_dn8 = assign82780_e125884_d_n8;
        locals.var_ps0ld_dn9 = assign82780_e125884_d_n9;
        locals.var_ps0ld_dn10 = assign82780_e125884_d_n10;
        locals.var_ps0ld_dn13 = assign82780_e125884_d_n13;

        let assign82790_e125887: f64 = if locals.var_wdep_func > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1921 = assign82790_e125887;

        let (assign82800_e125900, assign82800_e125900_d_n0, assign82800_e125900_d_n2, assign82800_e125900_d_n4, assign82800_e125900_d_n5, assign82800_e125900_d_n6, assign82800_e125900_d_n7, assign82800_e125900_d_n8, assign82800_e125900_d_n9, assign82800_e125900_d_n10, assign82800_e125900_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1920 != 0.0)) && (locals.var_guard1921 != 0.0)) {
        let assign82800_e125898: f64 = (p.p334 - locals.var_wdep_func);
        (assign82800_e125898, (-locals.var_wdep_func_dn0), (-locals.var_wdep_func_dn2), (-locals.var_wdep_func_dn4), (-locals.var_wdep_func_dn5), (-locals.var_wdep_func_dn6), (-locals.var_wdep_func_dn7), (-locals.var_wdep_func_dn8), (-locals.var_wdep_func_dn9), (-locals.var_wdep_func_dn10), (-locals.var_wdep_func_dn13),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign82800_e125900;
        locals.var_t2_dn0 = assign82800_e125900_d_n0;
        locals.var_t2_dn2 = assign82800_e125900_d_n2;
        locals.var_t2_dn4 = assign82800_e125900_d_n4;
        locals.var_t2_dn5 = assign82800_e125900_d_n5;
        locals.var_t2_dn6 = assign82800_e125900_d_n6;
        locals.var_t2_dn7 = assign82800_e125900_d_n7;
        locals.var_t2_dn8 = assign82800_e125900_d_n8;
        locals.var_t2_dn9 = assign82800_e125900_d_n9;
        locals.var_t2_dn10 = assign82800_e125900_d_n10;
        locals.var_t2_dn13 = assign82800_e125900_d_n13;

        let (assign82810_e125925, assign82810_e125925_d_n0, assign82810_e125925_d_n2, assign82810_e125925_d_n4, assign82810_e125925_d_n5, assign82810_e125925_d_n6, assign82810_e125925_d_n7, assign82810_e125925_d_n8, assign82810_e125925_d_n9, assign82810_e125925_d_n10, assign82810_e125925_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1920 != 0.0)) && (locals.var_guard1921 == 0.0)) {
        let assign82810_e125912: f64 = (locals.var_vdsi + p.p137);
        let assign82810_e125915: f64 = (locals.var_vdsi + p.p137);
        let assign82810_e125916: f64 = (assign82810_e125912 * assign82810_e125915);
        let assign82810_e125919: f64 = (4.0 * 0.1);
        let assign82810_e125921: f64 = (assign82810_e125919 * 0.1);
        let assign82810_e125922: f64 = (assign82810_e125916 + assign82810_e125921);
        let assign82810_e125923: f64 = (assign82810_e125922).sqrt();
        (assign82810_e125923, 0.0, 0.0, 0.0, (((locals.var_vdsi_dn5 * assign82810_e125915) + (assign82810_e125912 * locals.var_vdsi_dn5)) / (2.0 * assign82810_e125923)), 0.0, (((locals.var_vdsi_dn7 * assign82810_e125915) + (assign82810_e125912 * locals.var_vdsi_dn7)) / (2.0 * assign82810_e125923)), 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign82810_e125925;
        locals.var_tmf2_dn0 = assign82810_e125925_d_n0;
        locals.var_tmf2_dn2 = assign82810_e125925_d_n2;
        locals.var_tmf2_dn4 = assign82810_e125925_d_n4;
        locals.var_tmf2_dn5 = assign82810_e125925_d_n5;
        locals.var_tmf2_dn6 = assign82810_e125925_d_n6;
        locals.var_tmf2_dn7 = assign82810_e125925_d_n7;
        locals.var_tmf2_dn8 = assign82810_e125925_d_n8;
        locals.var_tmf2_dn9 = assign82810_e125925_d_n9;
        locals.var_tmf2_dn10 = assign82810_e125925_d_n10;
        locals.var_tmf2_dn13 = assign82810_e125925_d_n13;

        let (assign82820_e125945, assign82820_e125945_d_n0, assign82820_e125945_d_n2, assign82820_e125945_d_n4, assign82820_e125945_d_n5, assign82820_e125945_d_n6, assign82820_e125945_d_n7, assign82820_e125945_d_n8, assign82820_e125945_d_n9, assign82820_e125945_d_n10, assign82820_e125945_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1920 != 0.0)) && (locals.var_guard1921 == 0.0)) {
        let assign82820_e125939: f64 = (locals.var_vdsi + p.p137);
        let assign82820_e125941: f64 = (assign82820_e125939 / locals.var_tmf2);
        let assign82820_e125942: f64 = (1.0 + assign82820_e125941);
        let assign82820_e125943: f64 = (0.5 * assign82820_e125942);
        (assign82820_e125943, (0.5 * (-((assign82820_e125939 * locals.var_tmf2_dn0) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((assign82820_e125939 * locals.var_tmf2_dn2) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((assign82820_e125939 * locals.var_tmf2_dn4) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (((locals.var_vdsi_dn5 * locals.var_tmf2) - (assign82820_e125939 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (-((assign82820_e125939 * locals.var_tmf2_dn6) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (((locals.var_vdsi_dn7 * locals.var_tmf2) - (assign82820_e125939 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (-((assign82820_e125939 * locals.var_tmf2_dn8) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((assign82820_e125939 * locals.var_tmf2_dn9) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((assign82820_e125939 * locals.var_tmf2_dn10) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((assign82820_e125939 * locals.var_tmf2_dn13) / (locals.var_tmf2 * locals.var_tmf2)))),)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn13,)
    }
};
        locals.var_t9 = assign82820_e125945;
        locals.var_t9_dn0 = assign82820_e125945_d_n0;
        locals.var_t9_dn2 = assign82820_e125945_d_n2;
        locals.var_t9_dn4 = assign82820_e125945_d_n4;
        locals.var_t9_dn5 = assign82820_e125945_d_n5;
        locals.var_t9_dn6 = assign82820_e125945_d_n6;
        locals.var_t9_dn7 = assign82820_e125945_d_n7;
        locals.var_t9_dn8 = assign82820_e125945_d_n8;
        locals.var_t9_dn9 = assign82820_e125945_d_n9;
        locals.var_t9_dn10 = assign82820_e125945_d_n10;
        locals.var_t9_dn13 = assign82820_e125945_d_n13;

        let (assign82830_e125963, assign82830_e125963_d_n0, assign82830_e125963_d_n2, assign82830_e125963_d_n4, assign82830_e125963_d_n5, assign82830_e125963_d_n6, assign82830_e125963_d_n7, assign82830_e125963_d_n8, assign82830_e125963_d_n9, assign82830_e125963_d_n10, assign82830_e125963_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1920 != 0.0)) && (locals.var_guard1921 == 0.0)) {
        let assign82830_e125958: f64 = (locals.var_vdsi + p.p137);
        let assign82830_e125960: f64 = (assign82830_e125958 + locals.var_tmf2);
        let assign82830_e125961: f64 = (0.5 * assign82830_e125960);
        (assign82830_e125961, (0.5 * locals.var_tmf2_dn0), (0.5 * locals.var_tmf2_dn2), (0.5 * locals.var_tmf2_dn4), (0.5 * (locals.var_vdsi_dn5 + locals.var_tmf2_dn5)), (0.5 * locals.var_tmf2_dn6), (0.5 * (locals.var_vdsi_dn7 + locals.var_tmf2_dn7)), (0.5 * locals.var_tmf2_dn8), (0.5 * locals.var_tmf2_dn9), (0.5 * locals.var_tmf2_dn10), (0.5 * locals.var_tmf2_dn13),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign82830_e125963;
        locals.var_t2_dn0 = assign82830_e125963_d_n0;
        locals.var_t2_dn2 = assign82830_e125963_d_n2;
        locals.var_t2_dn4 = assign82830_e125963_d_n4;
        locals.var_t2_dn5 = assign82830_e125963_d_n5;
        locals.var_t2_dn6 = assign82830_e125963_d_n6;
        locals.var_t2_dn7 = assign82830_e125963_d_n7;
        locals.var_t2_dn8 = assign82830_e125963_d_n8;
        locals.var_t2_dn9 = assign82830_e125963_d_n9;
        locals.var_t2_dn10 = assign82830_e125963_d_n10;
        locals.var_t2_dn13 = assign82830_e125963_d_n13;

        let assign82840_e125966: f64 = if locals.var_t2 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1922 = assign82840_e125966;

        let (assign82850_e125980, assign82850_e125980_d_n0, assign82850_e125980_d_n2, assign82850_e125980_d_n4, assign82850_e125980_d_n5, assign82850_e125980_d_n6, assign82850_e125980_d_n7, assign82850_e125980_d_n8, assign82850_e125980_d_n9, assign82850_e125980_d_n10, assign82850_e125980_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1920 != 0.0)) && (locals.var_guard1921 == 0.0)) && (locals.var_guard1922 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign82850_e125980;
        locals.var_t2_dn0 = assign82850_e125980_d_n0;
        locals.var_t2_dn2 = assign82850_e125980_d_n2;
        locals.var_t2_dn4 = assign82850_e125980_d_n4;
        locals.var_t2_dn5 = assign82850_e125980_d_n5;
        locals.var_t2_dn6 = assign82850_e125980_d_n6;
        locals.var_t2_dn7 = assign82850_e125980_d_n7;
        locals.var_t2_dn8 = assign82850_e125980_d_n8;
        locals.var_t2_dn9 = assign82850_e125980_d_n9;
        locals.var_t2_dn10 = assign82850_e125980_d_n10;
        locals.var_t2_dn13 = assign82850_e125980_d_n13;

        let (assign82860_e125994, assign82860_e125994_d_n0, assign82860_e125994_d_n2, assign82860_e125994_d_n4, assign82860_e125994_d_n5, assign82860_e125994_d_n6, assign82860_e125994_d_n7, assign82860_e125994_d_n8, assign82860_e125994_d_n9, assign82860_e125994_d_n10, assign82860_e125994_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1920 != 0.0)) && (locals.var_guard1921 == 0.0)) && (locals.var_guard1922 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn13,)
    }
};
        locals.var_t9 = assign82860_e125994;
        locals.var_t9_dn0 = assign82860_e125994_d_n0;
        locals.var_t9_dn2 = assign82860_e125994_d_n2;
        locals.var_t9_dn4 = assign82860_e125994_d_n4;
        locals.var_t9_dn5 = assign82860_e125994_d_n5;
        locals.var_t9_dn6 = assign82860_e125994_d_n6;
        locals.var_t9_dn7 = assign82860_e125994_d_n7;
        locals.var_t9_dn8 = assign82860_e125994_d_n8;
        locals.var_t9_dn9 = assign82860_e125994_d_n9;
        locals.var_t9_dn10 = assign82860_e125994_d_n10;
        locals.var_t9_dn13 = assign82860_e125994_d_n13;

        let (assign82870_e126011, assign82870_e126011_d_n0, assign82870_e126011_d_n2, assign82870_e126011_d_n4, assign82870_e126011_d_n5, assign82870_e126011_d_n6, assign82870_e126011_d_n7, assign82870_e126011_d_n8, assign82870_e126011_d_n9, assign82870_e126011_d_n10, assign82870_e126011_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1920 != 0.0)) && (locals.var_guard1921 == 0.0)) {
        let assign82870_e126006: f64 = (locals.var_kjunc * locals.var_t2);
        let assign82870_e126007: f64 = (assign82870_e126006).sqrt();
        let assign82870_e126009: f64 = (assign82870_e126007 * p.p432);
        (assign82870_e126009, ((((locals.var_kjunc_dn0 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn0)) / (2.0 * assign82870_e126007)) * p.p432), ((((locals.var_kjunc_dn2 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn2)) / (2.0 * assign82870_e126007)) * p.p432), ((((locals.var_kjunc_dn4 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn4)) / (2.0 * assign82870_e126007)) * p.p432), ((((locals.var_kjunc_dn5 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn5)) / (2.0 * assign82870_e126007)) * p.p432), ((((locals.var_kjunc_dn6 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn6)) / (2.0 * assign82870_e126007)) * p.p432), ((((locals.var_kjunc_dn7 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn7)) / (2.0 * assign82870_e126007)) * p.p432), ((((locals.var_kjunc_dn8 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn8)) / (2.0 * assign82870_e126007)) * p.p432), ((((locals.var_kjunc_dn9 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn9)) / (2.0 * assign82870_e126007)) * p.p432), ((((locals.var_kjunc_dn10 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn10)) / (2.0 * assign82870_e126007)) * p.p432), ((((locals.var_kjunc_dn13 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn13)) / (2.0 * assign82870_e126007)) * p.p432),)
    } else {
        (locals.var_wjunc0, locals.var_wjunc0_dn0, locals.var_wjunc0_dn2, locals.var_wjunc0_dn4, locals.var_wjunc0_dn5, locals.var_wjunc0_dn6, locals.var_wjunc0_dn7, locals.var_wjunc0_dn8, locals.var_wjunc0_dn9, locals.var_wjunc0_dn10, locals.var_wjunc0_dn13,)
    }
};
        locals.var_wjunc0 = assign82870_e126011;
        locals.var_wjunc0_dn0 = assign82870_e126011_d_n0;
        locals.var_wjunc0_dn2 = assign82870_e126011_d_n2;
        locals.var_wjunc0_dn4 = assign82870_e126011_d_n4;
        locals.var_wjunc0_dn5 = assign82870_e126011_d_n5;
        locals.var_wjunc0_dn6 = assign82870_e126011_d_n6;
        locals.var_wjunc0_dn7 = assign82870_e126011_d_n7;
        locals.var_wjunc0_dn8 = assign82870_e126011_d_n8;
        locals.var_wjunc0_dn9 = assign82870_e126011_d_n9;
        locals.var_wjunc0_dn10 = assign82870_e126011_d_n10;
        locals.var_wjunc0_dn13 = assign82870_e126011_d_n13;

        let (assign82880_e126025, assign82880_e126025_d_n0, assign82880_e126025_d_n2, assign82880_e126025_d_n4, assign82880_e126025_d_n5, assign82880_e126025_d_n6, assign82880_e126025_d_n7, assign82880_e126025_d_n8, assign82880_e126025_d_n9, assign82880_e126025_d_n10, assign82880_e126025_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1920 != 0.0)) && (locals.var_guard1921 == 0.0)) {
        let assign82880_e126023: f64 = (p.p334 - locals.var_wjunc0);
        (assign82880_e126023, (-locals.var_wjunc0_dn0), (-locals.var_wjunc0_dn2), (-locals.var_wjunc0_dn4), (-locals.var_wjunc0_dn5), (-locals.var_wjunc0_dn6), (-locals.var_wjunc0_dn7), (-locals.var_wjunc0_dn8), (-locals.var_wjunc0_dn9), (-locals.var_wjunc0_dn10), (-locals.var_wjunc0_dn13),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign82880_e126025;
        locals.var_t2_dn0 = assign82880_e126025_d_n0;
        locals.var_t2_dn2 = assign82880_e126025_d_n2;
        locals.var_t2_dn4 = assign82880_e126025_d_n4;
        locals.var_t2_dn5 = assign82880_e126025_d_n5;
        locals.var_t2_dn6 = assign82880_e126025_d_n6;
        locals.var_t2_dn7 = assign82880_e126025_d_n7;
        locals.var_t2_dn8 = assign82880_e126025_d_n8;
        locals.var_t2_dn9 = assign82880_e126025_d_n9;
        locals.var_t2_dn10 = assign82880_e126025_d_n10;
        locals.var_t2_dn13 = assign82880_e126025_d_n13;

        let (assign82890_e126047, assign82890_e126047_d_n0, assign82890_e126047_d_n2, assign82890_e126047_d_n4, assign82890_e126047_d_n5, assign82890_e126047_d_n6, assign82890_e126047_d_n7, assign82890_e126047_d_n8, assign82890_e126047_d_n9, assign82890_e126047_d_n10, assign82890_e126047_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1920 != 0.0)) {
        let assign82890_e126034: f64 = (locals.var_t2 * locals.var_t2);
        let assign82890_e126038: f64 = (p.p334 * 0.01);
        let assign82890_e126039: f64 = (4.0 * assign82890_e126038);
        let assign82890_e126042: f64 = (p.p334 * 0.01);
        let assign82890_e126043: f64 = (assign82890_e126039 * assign82890_e126042);
        let assign82890_e126044: f64 = (assign82890_e126034 + assign82890_e126043);
        let assign82890_e126045: f64 = (assign82890_e126044).sqrt();
        (assign82890_e126045, (((locals.var_t2_dn0 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn0)) / (2.0 * assign82890_e126045)), (((locals.var_t2_dn2 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn2)) / (2.0 * assign82890_e126045)), (((locals.var_t2_dn4 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn4)) / (2.0 * assign82890_e126045)), (((locals.var_t2_dn5 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn5)) / (2.0 * assign82890_e126045)), (((locals.var_t2_dn6 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn6)) / (2.0 * assign82890_e126045)), (((locals.var_t2_dn7 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn7)) / (2.0 * assign82890_e126045)), (((locals.var_t2_dn8 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn8)) / (2.0 * assign82890_e126045)), (((locals.var_t2_dn9 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn9)) / (2.0 * assign82890_e126045)), (((locals.var_t2_dn10 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn10)) / (2.0 * assign82890_e126045)), (((locals.var_t2_dn13 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn13)) / (2.0 * assign82890_e126045)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign82890_e126047;
        locals.var_tmf2_dn0 = assign82890_e126047_d_n0;
        locals.var_tmf2_dn2 = assign82890_e126047_d_n2;
        locals.var_tmf2_dn4 = assign82890_e126047_d_n4;
        locals.var_tmf2_dn5 = assign82890_e126047_d_n5;
        locals.var_tmf2_dn6 = assign82890_e126047_d_n6;
        locals.var_tmf2_dn7 = assign82890_e126047_d_n7;
        locals.var_tmf2_dn8 = assign82890_e126047_d_n8;
        locals.var_tmf2_dn9 = assign82890_e126047_d_n9;
        locals.var_tmf2_dn10 = assign82890_e126047_d_n10;
        locals.var_tmf2_dn13 = assign82890_e126047_d_n13;

        let (assign82900_e126062, assign82900_e126062_d_n0, assign82900_e126062_d_n2, assign82900_e126062_d_n4, assign82900_e126062_d_n5, assign82900_e126062_d_n6, assign82900_e126062_d_n7, assign82900_e126062_d_n8, assign82900_e126062_d_n9, assign82900_e126062_d_n10, assign82900_e126062_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1920 != 0.0)) {
        let assign82900_e126058: f64 = (locals.var_t2 / locals.var_tmf2);
        let assign82900_e126059: f64 = (1.0 + assign82900_e126058);
        let assign82900_e126060: f64 = (0.5 * assign82900_e126059);
        (assign82900_e126060, (0.5 * (((locals.var_t2_dn0 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn2 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn4 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn5 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn6 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn7 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn8 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn9 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn10 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn13 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn13)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn13,)
    }
};
        locals.var_t9 = assign82900_e126062;
        locals.var_t9_dn0 = assign82900_e126062_d_n0;
        locals.var_t9_dn2 = assign82900_e126062_d_n2;
        locals.var_t9_dn4 = assign82900_e126062_d_n4;
        locals.var_t9_dn5 = assign82900_e126062_d_n5;
        locals.var_t9_dn6 = assign82900_e126062_d_n6;
        locals.var_t9_dn7 = assign82900_e126062_d_n7;
        locals.var_t9_dn8 = assign82900_e126062_d_n8;
        locals.var_t9_dn9 = assign82900_e126062_d_n9;
        locals.var_t9_dn10 = assign82900_e126062_d_n10;
        locals.var_t9_dn13 = assign82900_e126062_d_n13;

        let (assign82910_e126075, assign82910_e126075_d_n0, assign82910_e126075_d_n2, assign82910_e126075_d_n4, assign82910_e126075_d_n5, assign82910_e126075_d_n6, assign82910_e126075_d_n7, assign82910_e126075_d_n8, assign82910_e126075_d_n9, assign82910_e126075_d_n10, assign82910_e126075_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1920 != 0.0)) {
        let assign82910_e126072: f64 = (locals.var_t2 + locals.var_tmf2);
        let assign82910_e126073: f64 = (0.5 * assign82910_e126072);
        (assign82910_e126073, (0.5 * (locals.var_t2_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_t2_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_t2_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_t2_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_t2_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_t2_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_t2_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_t2_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_t2_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_t2_dn13 + locals.var_tmf2_dn13)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign82910_e126075;
        locals.var_t2_dn0 = assign82910_e126075_d_n0;
        locals.var_t2_dn2 = assign82910_e126075_d_n2;
        locals.var_t2_dn4 = assign82910_e126075_d_n4;
        locals.var_t2_dn5 = assign82910_e126075_d_n5;
        locals.var_t2_dn6 = assign82910_e126075_d_n6;
        locals.var_t2_dn7 = assign82910_e126075_d_n7;
        locals.var_t2_dn8 = assign82910_e126075_d_n8;
        locals.var_t2_dn9 = assign82910_e126075_d_n9;
        locals.var_t2_dn10 = assign82910_e126075_d_n10;
        locals.var_t2_dn13 = assign82910_e126075_d_n13;

        let assign82920_e126078: f64 = if locals.var_t2 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1923 = assign82920_e126078;

        let (assign82930_e126089, assign82930_e126089_d_n0, assign82930_e126089_d_n2, assign82930_e126089_d_n4, assign82930_e126089_d_n5, assign82930_e126089_d_n6, assign82930_e126089_d_n7, assign82930_e126089_d_n8, assign82930_e126089_d_n9, assign82930_e126089_d_n10, assign82930_e126089_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1920 != 0.0)) && (locals.var_guard1923 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign82930_e126089;
        locals.var_t2_dn0 = assign82930_e126089_d_n0;
        locals.var_t2_dn2 = assign82930_e126089_d_n2;
        locals.var_t2_dn4 = assign82930_e126089_d_n4;
        locals.var_t2_dn5 = assign82930_e126089_d_n5;
        locals.var_t2_dn6 = assign82930_e126089_d_n6;
        locals.var_t2_dn7 = assign82930_e126089_d_n7;
        locals.var_t2_dn8 = assign82930_e126089_d_n8;
        locals.var_t2_dn9 = assign82930_e126089_d_n9;
        locals.var_t2_dn10 = assign82930_e126089_d_n10;
        locals.var_t2_dn13 = assign82930_e126089_d_n13;

        let (assign82940_e126100, assign82940_e126100_d_n0, assign82940_e126100_d_n2, assign82940_e126100_d_n4, assign82940_e126100_d_n5, assign82940_e126100_d_n6, assign82940_e126100_d_n7, assign82940_e126100_d_n8, assign82940_e126100_d_n9, assign82940_e126100_d_n10, assign82940_e126100_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1920 != 0.0)) && (locals.var_guard1923 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn13,)
    }
};
        locals.var_t9 = assign82940_e126100;
        locals.var_t9_dn0 = assign82940_e126100_d_n0;
        locals.var_t9_dn2 = assign82940_e126100_d_n2;
        locals.var_t9_dn4 = assign82940_e126100_d_n4;
        locals.var_t9_dn5 = assign82940_e126100_d_n5;
        locals.var_t9_dn6 = assign82940_e126100_d_n6;
        locals.var_t9_dn7 = assign82940_e126100_d_n7;
        locals.var_t9_dn8 = assign82940_e126100_d_n8;
        locals.var_t9_dn9 = assign82940_e126100_d_n9;
        locals.var_t9_dn10 = assign82940_e126100_d_n10;
        locals.var_t9_dn13 = assign82940_e126100_d_n13;

        let (assign82950_e126109, assign82950_e126109_d_n0, assign82950_e126109_d_n2, assign82950_e126109_d_n4, assign82950_e126109_d_n5, assign82950_e126109_d_n6, assign82950_e126109_d_n7, assign82950_e126109_d_n8, assign82950_e126109_d_n9, assign82950_e126109_d_n10, assign82950_e126109_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1920 != 0.0)) {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    } else {
        (locals.var_ddriftldc, locals.var_ddriftldc_dn0, locals.var_ddriftldc_dn2, locals.var_ddriftldc_dn4, locals.var_ddriftldc_dn5, locals.var_ddriftldc_dn6, locals.var_ddriftldc_dn7, locals.var_ddriftldc_dn8, locals.var_ddriftldc_dn9, locals.var_ddriftldc_dn10, locals.var_ddriftldc_dn13,)
    }
};
        locals.var_ddriftldc = assign82950_e126109;
        locals.var_ddriftldc_dn0 = assign82950_e126109_d_n0;
        locals.var_ddriftldc_dn2 = assign82950_e126109_d_n2;
        locals.var_ddriftldc_dn4 = assign82950_e126109_d_n4;
        locals.var_ddriftldc_dn5 = assign82950_e126109_d_n5;
        locals.var_ddriftldc_dn6 = assign82950_e126109_d_n6;
        locals.var_ddriftldc_dn7 = assign82950_e126109_d_n7;
        locals.var_ddriftldc_dn8 = assign82950_e126109_d_n8;
        locals.var_ddriftldc_dn9 = assign82950_e126109_d_n9;
        locals.var_ddriftldc_dn10 = assign82950_e126109_d_n10;
        locals.var_ddriftldc_dn13 = assign82950_e126109_d_n13;

        let (assign82960_e126126, assign82960_e126126_d_n0, assign82960_e126126_d_n2, assign82960_e126126_d_n4, assign82960_e126126_d_n5, assign82960_e126126_d_n6, assign82960_e126126_d_n7, assign82960_e126126_d_n8, assign82960_e126126_d_n9, assign82960_e126126_d_n10, assign82960_e126126_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1920 != 0.0)) {
        let assign82960_e126118: f64 = (locals.var_q_nsubld__blk1883 * locals.var_ddriftldc);
        let assign82960_e126120: f64 = (assign82960_e126118 * locals.var_ddriftldc);
        let assign82960_e126122: f64 = (assign82960_e126120 / 2.0);
        let assign82960_e126124: f64 = (assign82960_e126122 / 1.034943e-10);
        (assign82960_e126124, (((((locals.var_q_nsubld__blk1883 * locals.var_ddriftldc_dn0) * locals.var_ddriftldc) + (assign82960_e126118 * locals.var_ddriftldc_dn0)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk1883 * locals.var_ddriftldc_dn2) * locals.var_ddriftldc) + (assign82960_e126118 * locals.var_ddriftldc_dn2)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk1883 * locals.var_ddriftldc_dn4) * locals.var_ddriftldc) + (assign82960_e126118 * locals.var_ddriftldc_dn4)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk1883 * locals.var_ddriftldc_dn5) * locals.var_ddriftldc) + (assign82960_e126118 * locals.var_ddriftldc_dn5)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk1883 * locals.var_ddriftldc_dn6) * locals.var_ddriftldc) + (assign82960_e126118 * locals.var_ddriftldc_dn6)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk1883 * locals.var_ddriftldc_dn7) * locals.var_ddriftldc) + (assign82960_e126118 * locals.var_ddriftldc_dn7)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk1883 * locals.var_ddriftldc_dn8) * locals.var_ddriftldc) + (assign82960_e126118 * locals.var_ddriftldc_dn8)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk1883 * locals.var_ddriftldc_dn9) * locals.var_ddriftldc) + (assign82960_e126118 * locals.var_ddriftldc_dn9)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk1883 * locals.var_ddriftldc_dn10) * locals.var_ddriftldc) + (assign82960_e126118 * locals.var_ddriftldc_dn10)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk1883 * locals.var_ddriftldc_dn13) * locals.var_ddriftldc) + (assign82960_e126118 * locals.var_ddriftldc_dn13)) / 2.0) / 1.034943e-10),)
    } else {
        (locals.var_dphi_sb, locals.var_dphi_sb_dn0, locals.var_dphi_sb_dn2, locals.var_dphi_sb_dn4, locals.var_dphi_sb_dn5, locals.var_dphi_sb_dn6, locals.var_dphi_sb_dn7, locals.var_dphi_sb_dn8, locals.var_dphi_sb_dn9, locals.var_dphi_sb_dn10, locals.var_dphi_sb_dn13,)
    }
};
        locals.var_dphi_sb = assign82960_e126126;
        locals.var_dphi_sb_dn0 = assign82960_e126126_d_n0;
        locals.var_dphi_sb_dn2 = assign82960_e126126_d_n2;
        locals.var_dphi_sb_dn4 = assign82960_e126126_d_n4;
        locals.var_dphi_sb_dn5 = assign82960_e126126_d_n5;
        locals.var_dphi_sb_dn6 = assign82960_e126126_d_n6;
        locals.var_dphi_sb_dn7 = assign82960_e126126_d_n7;
        locals.var_dphi_sb_dn8 = assign82960_e126126_d_n8;
        locals.var_dphi_sb_dn9 = assign82960_e126126_d_n9;
        locals.var_dphi_sb_dn10 = assign82960_e126126_d_n10;
        locals.var_dphi_sb_dn13 = assign82960_e126126_d_n13;

        let (assign82970_e126140, assign82970_e126140_d_n0, assign82970_e126140_d_n2, assign82970_e126140_d_n4, assign82970_e126140_d_n5, assign82970_e126140_d_n6, assign82970_e126140_d_n7, assign82970_e126140_d_n8, assign82970_e126140_d_n9, assign82970_e126140_d_n10, assign82970_e126140_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1920 != 0.0)) {
        let assign82970_e126135: f64 = (2.0 * locals.var_beta);
        let assign82970_e126137: f64 = (assign82970_e126135 * locals.var_dphi_sb);
        let assign82970_e126138: f64 = (assign82970_e126137).sqrt();
        (assign82970_e126138, ((((2.0 * locals.var_beta_dn0) * locals.var_dphi_sb) + (assign82970_e126135 * locals.var_dphi_sb_dn0)) / (2.0 * assign82970_e126138)), ((((2.0 * locals.var_beta_dn2) * locals.var_dphi_sb) + (assign82970_e126135 * locals.var_dphi_sb_dn2)) / (2.0 * assign82970_e126138)), ((((2.0 * locals.var_beta_dn4) * locals.var_dphi_sb) + (assign82970_e126135 * locals.var_dphi_sb_dn4)) / (2.0 * assign82970_e126138)), ((((2.0 * locals.var_beta_dn5) * locals.var_dphi_sb) + (assign82970_e126135 * locals.var_dphi_sb_dn5)) / (2.0 * assign82970_e126138)), ((((2.0 * locals.var_beta_dn6) * locals.var_dphi_sb) + (assign82970_e126135 * locals.var_dphi_sb_dn6)) / (2.0 * assign82970_e126138)), ((((2.0 * locals.var_beta_dn7) * locals.var_dphi_sb) + (assign82970_e126135 * locals.var_dphi_sb_dn7)) / (2.0 * assign82970_e126138)), ((((2.0 * locals.var_beta_dn8) * locals.var_dphi_sb) + (assign82970_e126135 * locals.var_dphi_sb_dn8)) / (2.0 * assign82970_e126138)), ((((2.0 * locals.var_beta_dn9) * locals.var_dphi_sb) + (assign82970_e126135 * locals.var_dphi_sb_dn9)) / (2.0 * assign82970_e126138)), ((((2.0 * locals.var_beta_dn10) * locals.var_dphi_sb) + (assign82970_e126135 * locals.var_dphi_sb_dn10)) / (2.0 * assign82970_e126138)), ((((2.0 * locals.var_beta_dn13) * locals.var_dphi_sb) + (assign82970_e126135 * locals.var_dphi_sb_dn13)) / (2.0 * assign82970_e126138)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign82970_e126140;
        locals.var_t0_dn0 = assign82970_e126140_d_n0;
        locals.var_t0_dn2 = assign82970_e126140_d_n2;
        locals.var_t0_dn4 = assign82970_e126140_d_n4;
        locals.var_t0_dn5 = assign82970_e126140_d_n5;
        locals.var_t0_dn6 = assign82970_e126140_d_n6;
        locals.var_t0_dn7 = assign82970_e126140_d_n7;
        locals.var_t0_dn8 = assign82970_e126140_d_n8;
        locals.var_t0_dn9 = assign82970_e126140_d_n9;
        locals.var_t0_dn10 = assign82970_e126140_d_n10;
        locals.var_t0_dn13 = assign82970_e126140_d_n13;

        let (assign82980_e126156, assign82980_e126156_d_n0, assign82980_e126156_d_n2, assign82980_e126156_d_n4, assign82980_e126156_d_n5, assign82980_e126156_d_n6, assign82980_e126156_d_n7, assign82980_e126156_d_n8, assign82980_e126156_d_n9, assign82980_e126156_d_n10, assign82980_e126156_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1920 != 0.0)) {
        let assign82980_e126148: f64 = { let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign82980_e126150: f64 = (-locals.var_t0);
        let assign82980_e126151: f64 = { let limited_exp_arg = assign82980_e126150; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign82980_e126152: f64 = (assign82980_e126148 + assign82980_e126151);
        let assign82980_e126154: f64 = (assign82980_e126152 / 2.0);
        (assign82980_e126154, ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn0) + ({ let limited_exp_arg = assign82980_e126150; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn0))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn2) + ({ let limited_exp_arg = assign82980_e126150; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn2))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn4) + ({ let limited_exp_arg = assign82980_e126150; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn4))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn5) + ({ let limited_exp_arg = assign82980_e126150; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn5))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn6) + ({ let limited_exp_arg = assign82980_e126150; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn6))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn7) + ({ let limited_exp_arg = assign82980_e126150; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn7))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn8) + ({ let limited_exp_arg = assign82980_e126150; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn8))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn9) + ({ let limited_exp_arg = assign82980_e126150; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn9))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn10) + ({ let limited_exp_arg = assign82980_e126150; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn10))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn13) + ({ let limited_exp_arg = assign82980_e126150; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn13))) / 2.0),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign82980_e126156;
        locals.var_t1_dn0 = assign82980_e126156_d_n0;
        locals.var_t1_dn2 = assign82980_e126156_d_n2;
        locals.var_t1_dn4 = assign82980_e126156_d_n4;
        locals.var_t1_dn5 = assign82980_e126156_d_n5;
        locals.var_t1_dn6 = assign82980_e126156_d_n6;
        locals.var_t1_dn7 = assign82980_e126156_d_n7;
        locals.var_t1_dn8 = assign82980_e126156_d_n8;
        locals.var_t1_dn9 = assign82980_e126156_d_n9;
        locals.var_t1_dn10 = assign82980_e126156_d_n10;
        locals.var_t1_dn13 = assign82980_e126156_d_n13;

    }

    pub(super) fn stamp_transient_block_289(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign82990_e126168, assign82990_e126168_d_n0, assign82990_e126168_d_n2, assign82990_e126168_d_n4, assign82990_e126168_d_n5, assign82990_e126168_d_n6, assign82990_e126168_d_n7, assign82990_e126168_d_n8, assign82990_e126168_d_n9, assign82990_e126168_d_n10, assign82990_e126168_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1920 != 0.0)) {
        let assign82990_e126164: f64 = (locals.var_t1).ln();
        let assign82990_e126166: f64 = (assign82990_e126164 / locals.var_dphi_sb);
        (assign82990_e126166, ((((locals.var_t1_dn0 / locals.var_t1) * locals.var_dphi_sb) - (assign82990_e126164 * locals.var_dphi_sb_dn0)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn2 / locals.var_t1) * locals.var_dphi_sb) - (assign82990_e126164 * locals.var_dphi_sb_dn2)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn4 / locals.var_t1) * locals.var_dphi_sb) - (assign82990_e126164 * locals.var_dphi_sb_dn4)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn5 / locals.var_t1) * locals.var_dphi_sb) - (assign82990_e126164 * locals.var_dphi_sb_dn5)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn6 / locals.var_t1) * locals.var_dphi_sb) - (assign82990_e126164 * locals.var_dphi_sb_dn6)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn7 / locals.var_t1) * locals.var_dphi_sb) - (assign82990_e126164 * locals.var_dphi_sb_dn7)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn8 / locals.var_t1) * locals.var_dphi_sb) - (assign82990_e126164 * locals.var_dphi_sb_dn8)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn9 / locals.var_t1) * locals.var_dphi_sb) - (assign82990_e126164 * locals.var_dphi_sb_dn9)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn10 / locals.var_t1) * locals.var_dphi_sb) - (assign82990_e126164 * locals.var_dphi_sb_dn10)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn13 / locals.var_t1) * locals.var_dphi_sb) - (assign82990_e126164 * locals.var_dphi_sb_dn13)) / (locals.var_dphi_sb * locals.var_dphi_sb)),)
    } else {
        (locals.var_c_sb, locals.var_c_sb_dn0, locals.var_c_sb_dn2, locals.var_c_sb_dn4, locals.var_c_sb_dn5, locals.var_c_sb_dn6, locals.var_c_sb_dn7, locals.var_c_sb_dn8, locals.var_c_sb_dn9, locals.var_c_sb_dn10, locals.var_c_sb_dn13,)
    }
};
        locals.var_c_sb = assign82990_e126168;
        locals.var_c_sb_dn0 = assign82990_e126168_d_n0;
        locals.var_c_sb_dn2 = assign82990_e126168_d_n2;
        locals.var_c_sb_dn4 = assign82990_e126168_d_n4;
        locals.var_c_sb_dn5 = assign82990_e126168_d_n5;
        locals.var_c_sb_dn6 = assign82990_e126168_d_n6;
        locals.var_c_sb_dn7 = assign82990_e126168_d_n7;
        locals.var_c_sb_dn8 = assign82990_e126168_d_n8;
        locals.var_c_sb_dn9 = assign82990_e126168_d_n9;
        locals.var_c_sb_dn10 = assign82990_e126168_d_n10;
        locals.var_c_sb_dn13 = assign82990_e126168_d_n13;

        let (assign83000_e126179, assign83000_e126179_d_n0, assign83000_e126179_d_n2, assign83000_e126179_d_n4, assign83000_e126179_d_n5, assign83000_e126179_d_n6, assign83000_e126179_d_n7, assign83000_e126179_d_n8, assign83000_e126179_d_n9, assign83000_e126179_d_n10, assign83000_e126179_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1920 != 0.0)) {
        let assign83000_e126177: f64 = (locals.var_ps0ld + locals.var_vxbgmtcl);
        (assign83000_e126177, (locals.var_ps0ld_dn0 + locals.var_vxbgmtcl_dn0), (locals.var_ps0ld_dn2 + locals.var_vxbgmtcl_dn2), (locals.var_ps0ld_dn4 + locals.var_vxbgmtcl_dn4), (locals.var_ps0ld_dn5 + locals.var_vxbgmtcl_dn5), (locals.var_ps0ld_dn6 + locals.var_vxbgmtcl_dn6), (locals.var_ps0ld_dn7 + locals.var_vxbgmtcl_dn7), (locals.var_ps0ld_dn8 + locals.var_vxbgmtcl_dn8), (locals.var_ps0ld_dn9 + locals.var_vxbgmtcl_dn9), (locals.var_ps0ld_dn10 + locals.var_vxbgmtcl_dn10), (locals.var_ps0ld_dn13 + locals.var_vxbgmtcl_dn13),)
    } else {
        (locals.var_ps0ld_vxb, locals.var_ps0ld_vxb_dn0, locals.var_ps0ld_vxb_dn2, locals.var_ps0ld_vxb_dn4, locals.var_ps0ld_vxb_dn5, locals.var_ps0ld_vxb_dn6, locals.var_ps0ld_vxb_dn7, locals.var_ps0ld_vxb_dn8, locals.var_ps0ld_vxb_dn9, locals.var_ps0ld_vxb_dn10, locals.var_ps0ld_vxb_dn13,)
    }
};
        locals.var_ps0ld_vxb = assign83000_e126179;
        locals.var_ps0ld_vxb_dn0 = assign83000_e126179_d_n0;
        locals.var_ps0ld_vxb_dn2 = assign83000_e126179_d_n2;
        locals.var_ps0ld_vxb_dn4 = assign83000_e126179_d_n4;
        locals.var_ps0ld_vxb_dn5 = assign83000_e126179_d_n5;
        locals.var_ps0ld_vxb_dn6 = assign83000_e126179_d_n6;
        locals.var_ps0ld_vxb_dn7 = assign83000_e126179_d_n7;
        locals.var_ps0ld_vxb_dn8 = assign83000_e126179_d_n8;
        locals.var_ps0ld_vxb_dn9 = assign83000_e126179_d_n9;
        locals.var_ps0ld_vxb_dn10 = assign83000_e126179_d_n10;
        locals.var_ps0ld_vxb_dn13 = assign83000_e126179_d_n13;

        let (assign83010_e126192, assign83010_e126192_d_n0, assign83010_e126192_d_n2, assign83010_e126192_d_n4, assign83010_e126192_d_n5, assign83010_e126192_d_n6, assign83010_e126192_d_n7, assign83010_e126192_d_n8, assign83010_e126192_d_n9, assign83010_e126192_d_n10, assign83010_e126192_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1920 != 0.0)) {
        let assign83010_e126189: f64 = (locals.var_ps0ld_vxb - locals.var_dphi_sb);
        let assign83010_e126190: f64 = (locals.var_c_sb * assign83010_e126189);
        (assign83010_e126190, ((locals.var_c_sb_dn0 * assign83010_e126189) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn0 - locals.var_dphi_sb_dn0))), ((locals.var_c_sb_dn2 * assign83010_e126189) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn2 - locals.var_dphi_sb_dn2))), ((locals.var_c_sb_dn4 * assign83010_e126189) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn4 - locals.var_dphi_sb_dn4))), ((locals.var_c_sb_dn5 * assign83010_e126189) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn5 - locals.var_dphi_sb_dn5))), ((locals.var_c_sb_dn6 * assign83010_e126189) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn6 - locals.var_dphi_sb_dn6))), ((locals.var_c_sb_dn7 * assign83010_e126189) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn7 - locals.var_dphi_sb_dn7))), ((locals.var_c_sb_dn8 * assign83010_e126189) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn8 - locals.var_dphi_sb_dn8))), ((locals.var_c_sb_dn9 * assign83010_e126189) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn9 - locals.var_dphi_sb_dn9))), ((locals.var_c_sb_dn10 * assign83010_e126189) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn10 - locals.var_dphi_sb_dn10))), ((locals.var_c_sb_dn13 * assign83010_e126189) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn13 - locals.var_dphi_sb_dn13))),)
    } else {
        (locals.var_ty, locals.var_ty_dn0, locals.var_ty_dn2, locals.var_ty_dn4, locals.var_ty_dn5, locals.var_ty_dn6, locals.var_ty_dn7, locals.var_ty_dn8, locals.var_ty_dn9, locals.var_ty_dn10, locals.var_ty_dn13,)
    }
};
        locals.var_ty = assign83010_e126192;
        locals.var_ty_dn0 = assign83010_e126192_d_n0;
        locals.var_ty_dn2 = assign83010_e126192_d_n2;
        locals.var_ty_dn4 = assign83010_e126192_d_n4;
        locals.var_ty_dn5 = assign83010_e126192_d_n5;
        locals.var_ty_dn6 = assign83010_e126192_d_n6;
        locals.var_ty_dn7 = assign83010_e126192_d_n7;
        locals.var_ty_dn8 = assign83010_e126192_d_n8;
        locals.var_ty_dn9 = assign83010_e126192_d_n9;
        locals.var_ty_dn10 = assign83010_e126192_d_n10;
        locals.var_ty_dn13 = assign83010_e126192_d_n13;

        let assign83020_e126195: f64 = if locals.var_ty < 60.0 { 1.0 } else { 0.0 };
        locals.var_guard1924 = assign83020_e126195;

        let (assign83030_e126207, assign83030_e126207_d_n0, assign83030_e126207_d_n2, assign83030_e126207_d_n4, assign83030_e126207_d_n5, assign83030_e126207_d_n6, assign83030_e126207_d_n7, assign83030_e126207_d_n8, assign83030_e126207_d_n9, assign83030_e126207_d_n10, assign83030_e126207_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1920 != 0.0)) && (locals.var_guard1924 != 0.0)) {
        let assign83030_e126205: f64 = (locals.var_ty).exp();
        (assign83030_e126205, (assign83030_e126205 * locals.var_ty_dn0), (assign83030_e126205 * locals.var_ty_dn2), (assign83030_e126205 * locals.var_ty_dn4), (assign83030_e126205 * locals.var_ty_dn5), (assign83030_e126205 * locals.var_ty_dn6), (assign83030_e126205 * locals.var_ty_dn7), (assign83030_e126205 * locals.var_ty_dn8), (assign83030_e126205 * locals.var_ty_dn9), (assign83030_e126205 * locals.var_ty_dn10), (assign83030_e126205 * locals.var_ty_dn13),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign83030_e126207;
        locals.var_t1_dn0 = assign83030_e126207_d_n0;
        locals.var_t1_dn2 = assign83030_e126207_d_n2;
        locals.var_t1_dn4 = assign83030_e126207_d_n4;
        locals.var_t1_dn5 = assign83030_e126207_d_n5;
        locals.var_t1_dn6 = assign83030_e126207_d_n6;
        locals.var_t1_dn7 = assign83030_e126207_d_n7;
        locals.var_t1_dn8 = assign83030_e126207_d_n8;
        locals.var_t1_dn9 = assign83030_e126207_d_n9;
        locals.var_t1_dn10 = assign83030_e126207_d_n10;
        locals.var_t1_dn13 = assign83030_e126207_d_n13;

        let (assign83040_e126222, assign83040_e126222_d_n0, assign83040_e126222_d_n2, assign83040_e126222_d_n4, assign83040_e126222_d_n5, assign83040_e126222_d_n6, assign83040_e126222_d_n7, assign83040_e126222_d_n8, assign83040_e126222_d_n9, assign83040_e126222_d_n10, assign83040_e126222_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1920 != 0.0)) && (locals.var_guard1924 != 0.0)) {
        let assign83040_e126217: f64 = (-locals.var_c_sb);
        let assign83040_e126219: f64 = (assign83040_e126217 * locals.var_dphi_sb);
        let assign83040_e126220: f64 = (assign83040_e126219).exp();
        (assign83040_e126220, (assign83040_e126220 * (((-locals.var_c_sb_dn0) * locals.var_dphi_sb) + (assign83040_e126217 * locals.var_dphi_sb_dn0))), (assign83040_e126220 * (((-locals.var_c_sb_dn2) * locals.var_dphi_sb) + (assign83040_e126217 * locals.var_dphi_sb_dn2))), (assign83040_e126220 * (((-locals.var_c_sb_dn4) * locals.var_dphi_sb) + (assign83040_e126217 * locals.var_dphi_sb_dn4))), (assign83040_e126220 * (((-locals.var_c_sb_dn5) * locals.var_dphi_sb) + (assign83040_e126217 * locals.var_dphi_sb_dn5))), (assign83040_e126220 * (((-locals.var_c_sb_dn6) * locals.var_dphi_sb) + (assign83040_e126217 * locals.var_dphi_sb_dn6))), (assign83040_e126220 * (((-locals.var_c_sb_dn7) * locals.var_dphi_sb) + (assign83040_e126217 * locals.var_dphi_sb_dn7))), (assign83040_e126220 * (((-locals.var_c_sb_dn8) * locals.var_dphi_sb) + (assign83040_e126217 * locals.var_dphi_sb_dn8))), (assign83040_e126220 * (((-locals.var_c_sb_dn9) * locals.var_dphi_sb) + (assign83040_e126217 * locals.var_dphi_sb_dn9))), (assign83040_e126220 * (((-locals.var_c_sb_dn10) * locals.var_dphi_sb) + (assign83040_e126217 * locals.var_dphi_sb_dn10))), (assign83040_e126220 * (((-locals.var_c_sb_dn13) * locals.var_dphi_sb) + (assign83040_e126217 * locals.var_dphi_sb_dn13))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign83040_e126222;
        locals.var_t0_dn0 = assign83040_e126222_d_n0;
        locals.var_t0_dn2 = assign83040_e126222_d_n2;
        locals.var_t0_dn4 = assign83040_e126222_d_n4;
        locals.var_t0_dn5 = assign83040_e126222_d_n5;
        locals.var_t0_dn6 = assign83040_e126222_d_n6;
        locals.var_t0_dn7 = assign83040_e126222_d_n7;
        locals.var_t0_dn8 = assign83040_e126222_d_n8;
        locals.var_t0_dn9 = assign83040_e126222_d_n9;
        locals.var_t0_dn10 = assign83040_e126222_d_n10;
        locals.var_t0_dn13 = assign83040_e126222_d_n13;

        let (assign83050_e126235, assign83050_e126235_d_n0, assign83050_e126235_d_n2, assign83050_e126235_d_n4, assign83050_e126235_d_n5, assign83050_e126235_d_n6, assign83050_e126235_d_n7, assign83050_e126235_d_n8, assign83050_e126235_d_n9, assign83050_e126235_d_n10, assign83050_e126235_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1920 != 0.0)) && (locals.var_guard1924 != 0.0)) {
        let assign83050_e126233: f64 = (locals.var_t1 - locals.var_t0);
        (assign83050_e126233, (locals.var_t1_dn0 - locals.var_t0_dn0), (locals.var_t1_dn2 - locals.var_t0_dn2), (locals.var_t1_dn4 - locals.var_t0_dn4), (locals.var_t1_dn5 - locals.var_t0_dn5), (locals.var_t1_dn6 - locals.var_t0_dn6), (locals.var_t1_dn7 - locals.var_t0_dn7), (locals.var_t1_dn8 - locals.var_t0_dn8), (locals.var_t1_dn9 - locals.var_t0_dn9), (locals.var_t1_dn10 - locals.var_t0_dn10), (locals.var_t1_dn13 - locals.var_t0_dn13),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign83050_e126235;
        locals.var_t2_dn0 = assign83050_e126235_d_n0;
        locals.var_t2_dn2 = assign83050_e126235_d_n2;
        locals.var_t2_dn4 = assign83050_e126235_d_n4;
        locals.var_t2_dn5 = assign83050_e126235_d_n5;
        locals.var_t2_dn6 = assign83050_e126235_d_n6;
        locals.var_t2_dn7 = assign83050_e126235_d_n7;
        locals.var_t2_dn8 = assign83050_e126235_d_n8;
        locals.var_t2_dn9 = assign83050_e126235_d_n9;
        locals.var_t2_dn10 = assign83050_e126235_d_n10;
        locals.var_t2_dn13 = assign83050_e126235_d_n13;

        let (assign83060_e126251, assign83060_e126251_d_n0, assign83060_e126251_d_n2, assign83060_e126251_d_n4, assign83060_e126251_d_n5, assign83060_e126251_d_n6, assign83060_e126251_d_n7, assign83060_e126251_d_n8, assign83060_e126251_d_n9, assign83060_e126251_d_n10, assign83060_e126251_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1920 != 0.0)) && (locals.var_guard1924 != 0.0)) {
        let assign83060_e126246: f64 = (1.0 + locals.var_t2);
        let assign83060_e126247: f64 = (assign83060_e126246).ln();
        let assign83060_e126249: f64 = (assign83060_e126247 / locals.var_c_sb);
        (assign83060_e126249, ((((locals.var_t2_dn0 / assign83060_e126246) * locals.var_c_sb) - (assign83060_e126247 * locals.var_c_sb_dn0)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn2 / assign83060_e126246) * locals.var_c_sb) - (assign83060_e126247 * locals.var_c_sb_dn2)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn4 / assign83060_e126246) * locals.var_c_sb) - (assign83060_e126247 * locals.var_c_sb_dn4)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn5 / assign83060_e126246) * locals.var_c_sb) - (assign83060_e126247 * locals.var_c_sb_dn5)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn6 / assign83060_e126246) * locals.var_c_sb) - (assign83060_e126247 * locals.var_c_sb_dn6)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn7 / assign83060_e126246) * locals.var_c_sb) - (assign83060_e126247 * locals.var_c_sb_dn7)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn8 / assign83060_e126246) * locals.var_c_sb) - (assign83060_e126247 * locals.var_c_sb_dn8)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn9 / assign83060_e126246) * locals.var_c_sb) - (assign83060_e126247 * locals.var_c_sb_dn9)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn10 / assign83060_e126246) * locals.var_c_sb) - (assign83060_e126247 * locals.var_c_sb_dn10)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn13 / assign83060_e126246) * locals.var_c_sb) - (assign83060_e126247 * locals.var_c_sb_dn13)) / (locals.var_c_sb * locals.var_c_sb)),)
    } else {
        (locals.var_phi_b, locals.var_phi_b_dn0, locals.var_phi_b_dn2, locals.var_phi_b_dn4, locals.var_phi_b_dn5, locals.var_phi_b_dn6, locals.var_phi_b_dn7, locals.var_phi_b_dn8, locals.var_phi_b_dn9, locals.var_phi_b_dn10, locals.var_phi_b_dn13,)
    }
};
        locals.var_phi_b = assign83060_e126251;
        locals.var_phi_b_dn0 = assign83060_e126251_d_n0;
        locals.var_phi_b_dn2 = assign83060_e126251_d_n2;
        locals.var_phi_b_dn4 = assign83060_e126251_d_n4;
        locals.var_phi_b_dn5 = assign83060_e126251_d_n5;
        locals.var_phi_b_dn6 = assign83060_e126251_d_n6;
        locals.var_phi_b_dn7 = assign83060_e126251_d_n7;
        locals.var_phi_b_dn8 = assign83060_e126251_d_n8;
        locals.var_phi_b_dn9 = assign83060_e126251_d_n9;
        locals.var_phi_b_dn10 = assign83060_e126251_d_n10;
        locals.var_phi_b_dn13 = assign83060_e126251_d_n13;

        let (assign83070_e126265, assign83070_e126265_d_n0, assign83070_e126265_d_n2, assign83070_e126265_d_n4, assign83070_e126265_d_n5, assign83070_e126265_d_n6, assign83070_e126265_d_n7, assign83070_e126265_d_n8, assign83070_e126265_d_n9, assign83070_e126265_d_n10, assign83070_e126265_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1920 != 0.0)) && (locals.var_guard1924 == 0.0)) {
        let assign83070_e126263: f64 = (locals.var_ps0ld_vxb - locals.var_dphi_sb);
        (assign83070_e126263, (locals.var_ps0ld_vxb_dn0 - locals.var_dphi_sb_dn0), (locals.var_ps0ld_vxb_dn2 - locals.var_dphi_sb_dn2), (locals.var_ps0ld_vxb_dn4 - locals.var_dphi_sb_dn4), (locals.var_ps0ld_vxb_dn5 - locals.var_dphi_sb_dn5), (locals.var_ps0ld_vxb_dn6 - locals.var_dphi_sb_dn6), (locals.var_ps0ld_vxb_dn7 - locals.var_dphi_sb_dn7), (locals.var_ps0ld_vxb_dn8 - locals.var_dphi_sb_dn8), (locals.var_ps0ld_vxb_dn9 - locals.var_dphi_sb_dn9), (locals.var_ps0ld_vxb_dn10 - locals.var_dphi_sb_dn10), (locals.var_ps0ld_vxb_dn13 - locals.var_dphi_sb_dn13),)
    } else {
        (locals.var_phi_b, locals.var_phi_b_dn0, locals.var_phi_b_dn2, locals.var_phi_b_dn4, locals.var_phi_b_dn5, locals.var_phi_b_dn6, locals.var_phi_b_dn7, locals.var_phi_b_dn8, locals.var_phi_b_dn9, locals.var_phi_b_dn10, locals.var_phi_b_dn13,)
    }
};
        locals.var_phi_b = assign83070_e126265;
        locals.var_phi_b_dn0 = assign83070_e126265_d_n0;
        locals.var_phi_b_dn2 = assign83070_e126265_d_n2;
        locals.var_phi_b_dn4 = assign83070_e126265_d_n4;
        locals.var_phi_b_dn5 = assign83070_e126265_d_n5;
        locals.var_phi_b_dn6 = assign83070_e126265_d_n6;
        locals.var_phi_b_dn7 = assign83070_e126265_d_n7;
        locals.var_phi_b_dn8 = assign83070_e126265_d_n8;
        locals.var_phi_b_dn9 = assign83070_e126265_d_n9;
        locals.var_phi_b_dn10 = assign83070_e126265_d_n10;
        locals.var_phi_b_dn13 = assign83070_e126265_d_n13;

        let (assign83080_e126276, assign83080_e126276_d_n0, assign83080_e126276_d_n2, assign83080_e126276_d_n4, assign83080_e126276_d_n5, assign83080_e126276_d_n6, assign83080_e126276_d_n7, assign83080_e126276_d_n8, assign83080_e126276_d_n9, assign83080_e126276_d_n10, assign83080_e126276_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1920 != 0.0)) {
        let assign83080_e126274: f64 = (locals.var_beta * locals.var_phi_b);
        (assign83080_e126274, ((locals.var_beta_dn0 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn0)), ((locals.var_beta_dn2 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn2)), ((locals.var_beta_dn4 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn4)), ((locals.var_beta_dn5 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn5)), ((locals.var_beta_dn6 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn6)), ((locals.var_beta_dn7 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn7)), ((locals.var_beta_dn8 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn8)), ((locals.var_beta_dn9 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn9)), ((locals.var_beta_dn10 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn10)), ((locals.var_beta_dn13 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn13)),)
    } else {
        (locals.var_chib, locals.var_chib_dn0, locals.var_chib_dn2, locals.var_chib_dn4, locals.var_chib_dn5, locals.var_chib_dn6, locals.var_chib_dn7, locals.var_chib_dn8, locals.var_chib_dn9, locals.var_chib_dn10, locals.var_chib_dn13,)
    }
};
        locals.var_chib = assign83080_e126276;
        locals.var_chib_dn0 = assign83080_e126276_d_n0;
        locals.var_chib_dn2 = assign83080_e126276_d_n2;
        locals.var_chib_dn4 = assign83080_e126276_d_n4;
        locals.var_chib_dn5 = assign83080_e126276_d_n5;
        locals.var_chib_dn6 = assign83080_e126276_d_n6;
        locals.var_chib_dn7 = assign83080_e126276_d_n7;
        locals.var_chib_dn8 = assign83080_e126276_d_n8;
        locals.var_chib_dn9 = assign83080_e126276_d_n9;
        locals.var_chib_dn10 = assign83080_e126276_d_n10;
        locals.var_chib_dn13 = assign83080_e126276_d_n13;

        let assign83090_e126280: f64 = (locals.var_chi / 100.0);
        let assign83090_e126285: f64 = if ((locals.var_chib > assign83090_e126280) && (locals.var_chib > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1925 = assign83090_e126285;

        let (assign83100_e126298,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1920 != 0.0)) && (locals.var_guard1925 != 0.0)) {
        let assign83100_e126296: f64 = (locals.var_flg_fd_mode__blk1889 + 1.0);
        (assign83100_e126296,)
    } else {
        (locals.var_flg_fd_mode__blk1889,)
    }
};
        locals.var_flg_fd_mode__blk1889 = assign83100_e126298;

        let (assign83110_e126309, assign83110_e126309_d_n0, assign83110_e126309_d_n2, assign83110_e126309_d_n4, assign83110_e126309_d_n5, assign83110_e126309_d_n6, assign83110_e126309_d_n7, assign83110_e126309_d_n8, assign83110_e126309_d_n9, assign83110_e126309_d_n10, assign83110_e126309_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1920 != 0.0)) && (locals.var_guard1925 != 0.0)) {
        (locals.var_chi_1, locals.var_chi_1_dn0, locals.var_chi_1_dn2, locals.var_chi_1_dn4, locals.var_chi_1_dn5, locals.var_chi_1_dn6, locals.var_chi_1_dn7, locals.var_chi_1_dn8, locals.var_chi_1_dn9, locals.var_chi_1_dn10, locals.var_chi_1_dn13,)
    } else {
        (locals.var_chi, locals.var_chi_dn0, locals.var_chi_dn2, locals.var_chi_dn4, locals.var_chi_dn5, locals.var_chi_dn6, locals.var_chi_dn7, locals.var_chi_dn8, locals.var_chi_dn9, locals.var_chi_dn10, locals.var_chi_dn13,)
    }
};
        locals.var_chi = assign83110_e126309;
        locals.var_chi_dn0 = assign83110_e126309_d_n0;
        locals.var_chi_dn2 = assign83110_e126309_d_n2;
        locals.var_chi_dn4 = assign83110_e126309_d_n4;
        locals.var_chi_dn5 = assign83110_e126309_d_n5;
        locals.var_chi_dn6 = assign83110_e126309_d_n6;
        locals.var_chi_dn7 = assign83110_e126309_d_n7;
        locals.var_chi_dn8 = assign83110_e126309_d_n8;
        locals.var_chi_dn9 = assign83110_e126309_d_n9;
        locals.var_chi_dn10 = assign83110_e126309_d_n10;
        locals.var_chi_dn13 = assign83110_e126309_d_n13;

        let (assign83120_e126320, assign83120_e126320_d_n0, assign83120_e126320_d_n2, assign83120_e126320_d_n4, assign83120_e126320_d_n5, assign83120_e126320_d_n6, assign83120_e126320_d_n7, assign83120_e126320_d_n8, assign83120_e126320_d_n9, assign83120_e126320_d_n10, assign83120_e126320_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) {
        let assign83120_e126316: f64 = (locals.var_chi * locals.var_beta_inv);
        let assign83120_e126318: f64 = (assign83120_e126316 - locals.var_vxbgmtcl);
        (assign83120_e126318, (((locals.var_chi_dn0 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn0)) - locals.var_vxbgmtcl_dn0), (((locals.var_chi_dn2 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn2)) - locals.var_vxbgmtcl_dn2), (((locals.var_chi_dn4 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn4)) - locals.var_vxbgmtcl_dn4), (((locals.var_chi_dn5 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn5)) - locals.var_vxbgmtcl_dn5), (((locals.var_chi_dn6 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn6)) - locals.var_vxbgmtcl_dn6), (((locals.var_chi_dn7 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn7)) - locals.var_vxbgmtcl_dn7), (((locals.var_chi_dn8 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn8)) - locals.var_vxbgmtcl_dn8), (((locals.var_chi_dn9 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn9)) - locals.var_vxbgmtcl_dn9), (((locals.var_chi_dn10 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn10)) - locals.var_vxbgmtcl_dn10), (((locals.var_chi_dn13 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn13)) - locals.var_vxbgmtcl_dn13),)
    } else {
        (locals.var_ps0ld, locals.var_ps0ld_dn0, locals.var_ps0ld_dn2, locals.var_ps0ld_dn4, locals.var_ps0ld_dn5, locals.var_ps0ld_dn6, locals.var_ps0ld_dn7, locals.var_ps0ld_dn8, locals.var_ps0ld_dn9, locals.var_ps0ld_dn10, locals.var_ps0ld_dn13,)
    }
};
        locals.var_ps0ld = assign83120_e126320;
        locals.var_ps0ld_dn0 = assign83120_e126320_d_n0;
        locals.var_ps0ld_dn2 = assign83120_e126320_d_n2;
        locals.var_ps0ld_dn4 = assign83120_e126320_d_n4;
        locals.var_ps0ld_dn5 = assign83120_e126320_d_n5;
        locals.var_ps0ld_dn6 = assign83120_e126320_d_n6;
        locals.var_ps0ld_dn7 = assign83120_e126320_d_n7;
        locals.var_ps0ld_dn8 = assign83120_e126320_d_n8;
        locals.var_ps0ld_dn9 = assign83120_e126320_d_n9;
        locals.var_ps0ld_dn10 = assign83120_e126320_d_n10;
        locals.var_ps0ld_dn13 = assign83120_e126320_d_n13;

        let assign83130_e126322: f64 = (locals.var_chi).abs();
        let assign83130_e126324: f64 = if assign83130_e126322 > 1e-6 { 1.0 } else { 0.0 };
        locals.var_guard1926 = assign83130_e126324;

        let (assign83140_e126339, assign83140_e126339_d_n0, assign83140_e126339_d_n2, assign83140_e126339_d_n4, assign83140_e126339_d_n5, assign83140_e126339_d_n6, assign83140_e126339_d_n7, assign83140_e126339_d_n8, assign83140_e126339_d_n9, assign83140_e126339_d_n10, assign83140_e126339_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1926 != 0.0)) {
        let assign83140_e126333: f64 = (locals.var_chi - 1.0);
        let assign83140_e126335: f64 = (-locals.var_chi);
        let assign83140_e126336: f64 = (assign83140_e126335).exp();
        let assign83140_e126337: f64 = (assign83140_e126333 + assign83140_e126336);
        (assign83140_e126337, (locals.var_chi_dn0 + (assign83140_e126336 * (-locals.var_chi_dn0))), (locals.var_chi_dn2 + (assign83140_e126336 * (-locals.var_chi_dn2))), (locals.var_chi_dn4 + (assign83140_e126336 * (-locals.var_chi_dn4))), (locals.var_chi_dn5 + (assign83140_e126336 * (-locals.var_chi_dn5))), (locals.var_chi_dn6 + (assign83140_e126336 * (-locals.var_chi_dn6))), (locals.var_chi_dn7 + (assign83140_e126336 * (-locals.var_chi_dn7))), (locals.var_chi_dn8 + (assign83140_e126336 * (-locals.var_chi_dn8))), (locals.var_chi_dn9 + (assign83140_e126336 * (-locals.var_chi_dn9))), (locals.var_chi_dn10 + (assign83140_e126336 * (-locals.var_chi_dn10))), (locals.var_chi_dn13 + (assign83140_e126336 * (-locals.var_chi_dn13))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign83140_e126339;
        locals.var_t1_dn0 = assign83140_e126339_d_n0;
        locals.var_t1_dn2 = assign83140_e126339_d_n2;
        locals.var_t1_dn4 = assign83140_e126339_d_n4;
        locals.var_t1_dn5 = assign83140_e126339_d_n5;
        locals.var_t1_dn6 = assign83140_e126339_d_n6;
        locals.var_t1_dn7 = assign83140_e126339_d_n7;
        locals.var_t1_dn8 = assign83140_e126339_d_n8;
        locals.var_t1_dn9 = assign83140_e126339_d_n9;
        locals.var_t1_dn10 = assign83140_e126339_d_n10;
        locals.var_t1_dn13 = assign83140_e126339_d_n13;

        let (assign83150_e126349, assign83150_e126349_d_n0, assign83150_e126349_d_n2, assign83150_e126349_d_n4, assign83150_e126349_d_n5, assign83150_e126349_d_n6, assign83150_e126349_d_n7, assign83150_e126349_d_n8, assign83150_e126349_d_n9, assign83150_e126349_d_n10, assign83150_e126349_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1926 != 0.0)) {
        let assign83150_e126347: f64 = (locals.var_t1).sqrt();
        (assign83150_e126347, (locals.var_t1_dn0 / (2.0 * assign83150_e126347)), (locals.var_t1_dn2 / (2.0 * assign83150_e126347)), (locals.var_t1_dn4 / (2.0 * assign83150_e126347)), (locals.var_t1_dn5 / (2.0 * assign83150_e126347)), (locals.var_t1_dn6 / (2.0 * assign83150_e126347)), (locals.var_t1_dn7 / (2.0 * assign83150_e126347)), (locals.var_t1_dn8 / (2.0 * assign83150_e126347)), (locals.var_t1_dn9 / (2.0 * assign83150_e126347)), (locals.var_t1_dn10 / (2.0 * assign83150_e126347)), (locals.var_t1_dn13 / (2.0 * assign83150_e126347)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign83150_e126349;
        locals.var_t2_dn0 = assign83150_e126349_d_n0;
        locals.var_t2_dn2 = assign83150_e126349_d_n2;
        locals.var_t2_dn4 = assign83150_e126349_d_n4;
        locals.var_t2_dn5 = assign83150_e126349_d_n5;
        locals.var_t2_dn6 = assign83150_e126349_d_n6;
        locals.var_t2_dn7 = assign83150_e126349_d_n7;
        locals.var_t2_dn8 = assign83150_e126349_d_n8;
        locals.var_t2_dn9 = assign83150_e126349_d_n9;
        locals.var_t2_dn10 = assign83150_e126349_d_n10;
        locals.var_t2_dn13 = assign83150_e126349_d_n13;

        let (assign83170_e126380, assign83170_e126380_d_n0, assign83170_e126380_d_n2, assign83170_e126380_d_n4, assign83170_e126380_d_n5, assign83170_e126380_d_n6, assign83170_e126380_d_n7, assign83170_e126380_d_n8, assign83170_e126380_d_n9, assign83170_e126380_d_n10, assign83170_e126380_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1926 == 0.0)) {
        let assign83170_e126371: f64 = (0.7071067811865475 * locals.var_chi);
        let assign83170_e126375: f64 = (locals.var_chi * 0.3333333333333333);
        let assign83170_e126376: f64 = (1.0 - assign83170_e126375);
        let assign83170_e126377: f64 = (assign83170_e126376).sqrt();
        let assign83170_e126378: f64 = (assign83170_e126371 * assign83170_e126377);
        (assign83170_e126378, (((0.7071067811865475 * locals.var_chi_dn0) * assign83170_e126377) + (assign83170_e126371 * ((-(locals.var_chi_dn0 * 0.3333333333333333)) / (2.0 * assign83170_e126377)))), (((0.7071067811865475 * locals.var_chi_dn2) * assign83170_e126377) + (assign83170_e126371 * ((-(locals.var_chi_dn2 * 0.3333333333333333)) / (2.0 * assign83170_e126377)))), (((0.7071067811865475 * locals.var_chi_dn4) * assign83170_e126377) + (assign83170_e126371 * ((-(locals.var_chi_dn4 * 0.3333333333333333)) / (2.0 * assign83170_e126377)))), (((0.7071067811865475 * locals.var_chi_dn5) * assign83170_e126377) + (assign83170_e126371 * ((-(locals.var_chi_dn5 * 0.3333333333333333)) / (2.0 * assign83170_e126377)))), (((0.7071067811865475 * locals.var_chi_dn6) * assign83170_e126377) + (assign83170_e126371 * ((-(locals.var_chi_dn6 * 0.3333333333333333)) / (2.0 * assign83170_e126377)))), (((0.7071067811865475 * locals.var_chi_dn7) * assign83170_e126377) + (assign83170_e126371 * ((-(locals.var_chi_dn7 * 0.3333333333333333)) / (2.0 * assign83170_e126377)))), (((0.7071067811865475 * locals.var_chi_dn8) * assign83170_e126377) + (assign83170_e126371 * ((-(locals.var_chi_dn8 * 0.3333333333333333)) / (2.0 * assign83170_e126377)))), (((0.7071067811865475 * locals.var_chi_dn9) * assign83170_e126377) + (assign83170_e126371 * ((-(locals.var_chi_dn9 * 0.3333333333333333)) / (2.0 * assign83170_e126377)))), (((0.7071067811865475 * locals.var_chi_dn10) * assign83170_e126377) + (assign83170_e126371 * ((-(locals.var_chi_dn10 * 0.3333333333333333)) / (2.0 * assign83170_e126377)))), (((0.7071067811865475 * locals.var_chi_dn13) * assign83170_e126377) + (assign83170_e126371 * ((-(locals.var_chi_dn13 * 0.3333333333333333)) / (2.0 * assign83170_e126377)))),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign83170_e126380;
        locals.var_t2_dn0 = assign83170_e126380_d_n0;
        locals.var_t2_dn2 = assign83170_e126380_d_n2;
        locals.var_t2_dn4 = assign83170_e126380_d_n4;
        locals.var_t2_dn5 = assign83170_e126380_d_n5;
        locals.var_t2_dn6 = assign83170_e126380_d_n6;
        locals.var_t2_dn7 = assign83170_e126380_d_n7;
        locals.var_t2_dn8 = assign83170_e126380_d_n8;
        locals.var_t2_dn9 = assign83170_e126380_d_n9;
        locals.var_t2_dn10 = assign83170_e126380_d_n10;
        locals.var_t2_dn13 = assign83170_e126380_d_n13;

        let (assign83180_e126389, assign83180_e126389_d_n0, assign83180_e126389_d_n2, assign83180_e126389_d_n4, assign83180_e126389_d_n5, assign83180_e126389_d_n6, assign83180_e126389_d_n7, assign83180_e126389_d_n8, assign83180_e126389_d_n9, assign83180_e126389_d_n10, assign83180_e126389_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) {
        let assign83180_e126387: f64 = (locals.var_cnst0over_func * locals.var_t2);
        (assign83180_e126387, ((locals.var_cnst0over_func_dn0 * locals.var_t2) + (locals.var_cnst0over_func * locals.var_t2_dn0)), ((locals.var_cnst0over_func_dn2 * locals.var_t2) + (locals.var_cnst0over_func * locals.var_t2_dn2)), ((locals.var_cnst0over_func_dn4 * locals.var_t2) + (locals.var_cnst0over_func * locals.var_t2_dn4)), ((locals.var_cnst0over_func_dn5 * locals.var_t2) + (locals.var_cnst0over_func * locals.var_t2_dn5)), ((locals.var_cnst0over_func_dn6 * locals.var_t2) + (locals.var_cnst0over_func * locals.var_t2_dn6)), ((locals.var_cnst0over_func_dn7 * locals.var_t2) + (locals.var_cnst0over_func * locals.var_t2_dn7)), ((locals.var_cnst0over_func_dn8 * locals.var_t2) + (locals.var_cnst0over_func * locals.var_t2_dn8)), ((locals.var_cnst0over_func_dn9 * locals.var_t2) + (locals.var_cnst0over_func * locals.var_t2_dn9)), ((locals.var_cnst0over_func_dn10 * locals.var_t2) + (locals.var_cnst0over_func * locals.var_t2_dn10)), ((locals.var_cnst0over_func_dn13 * locals.var_t2) + (locals.var_cnst0over_func * locals.var_t2_dn13)),)
    } else {
        (locals.var_qbuld, locals.var_qbuld_dn0, locals.var_qbuld_dn2, locals.var_qbuld_dn4, locals.var_qbuld_dn5, locals.var_qbuld_dn6, locals.var_qbuld_dn7, locals.var_qbuld_dn8, locals.var_qbuld_dn9, locals.var_qbuld_dn10, locals.var_qbuld_dn13,)
    }
};
        locals.var_qbuld = assign83180_e126389;
        locals.var_qbuld_dn0 = assign83180_e126389_d_n0;
        locals.var_qbuld_dn2 = assign83180_e126389_d_n2;
        locals.var_qbuld_dn4 = assign83180_e126389_d_n4;
        locals.var_qbuld_dn5 = assign83180_e126389_d_n5;
        locals.var_qbuld_dn6 = assign83180_e126389_d_n6;
        locals.var_qbuld_dn7 = assign83180_e126389_d_n7;
        locals.var_qbuld_dn8 = assign83180_e126389_d_n8;
        locals.var_qbuld_dn9 = assign83180_e126389_d_n9;
        locals.var_qbuld_dn10 = assign83180_e126389_d_n10;
        locals.var_qbuld_dn13 = assign83180_e126389_d_n13;

        let (assign83190_e126400, assign83190_e126400_d_n0, assign83190_e126400_d_n2, assign83190_e126400_d_n4, assign83190_e126400_d_n5, assign83190_e126400_d_n6, assign83190_e126400_d_n7, assign83190_e126400_d_n8, assign83190_e126400_d_n9, assign83190_e126400_d_n10, assign83190_e126400_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) {
        let assign83190_e126397: f64 = (locals.var_vgpld - locals.var_ps0ld);
        let assign83190_e126398: f64 = (locals.var_cox0_func * assign83190_e126397);
        (assign83190_e126398, (locals.var_cox0_func * (-locals.var_ps0ld_dn0)), (locals.var_cox0_func * (locals.var_vgpld_dn2 - locals.var_ps0ld_dn2)), (locals.var_cox0_func * (-locals.var_ps0ld_dn4)), (locals.var_cox0_func * (-locals.var_ps0ld_dn5)), (locals.var_cox0_func * (locals.var_vgpld_dn6 - locals.var_ps0ld_dn6)), (locals.var_cox0_func * (locals.var_vgpld_dn7 - locals.var_ps0ld_dn7)), (locals.var_cox0_func * (locals.var_vgpld_dn8 - locals.var_ps0ld_dn8)), (locals.var_cox0_func * (-locals.var_ps0ld_dn9)), (locals.var_cox0_func * (-locals.var_ps0ld_dn10)), (locals.var_cox0_func * (-locals.var_ps0ld_dn13)),)
    } else {
        (locals.var_qsuld, locals.var_qsuld_dn0, locals.var_qsuld_dn2, locals.var_qsuld_dn4, locals.var_qsuld_dn5, locals.var_qsuld_dn6, locals.var_qsuld_dn7, locals.var_qsuld_dn8, locals.var_qsuld_dn9, locals.var_qsuld_dn10, locals.var_qsuld_dn13,)
    }
};
        locals.var_qsuld = assign83190_e126400;
        locals.var_qsuld_dn0 = assign83190_e126400_d_n0;
        locals.var_qsuld_dn2 = assign83190_e126400_d_n2;
        locals.var_qsuld_dn4 = assign83190_e126400_d_n4;
        locals.var_qsuld_dn5 = assign83190_e126400_d_n5;
        locals.var_qsuld_dn6 = assign83190_e126400_d_n6;
        locals.var_qsuld_dn7 = assign83190_e126400_d_n7;
        locals.var_qsuld_dn8 = assign83190_e126400_d_n8;
        locals.var_qsuld_dn9 = assign83190_e126400_d_n9;
        locals.var_qsuld_dn10 = assign83190_e126400_d_n10;
        locals.var_qsuld_dn13 = assign83190_e126400_d_n13;

        let (assign83200_e126409, assign83200_e126409_d_n0, assign83200_e126409_d_n2, assign83200_e126409_d_n4, assign83200_e126409_d_n5, assign83200_e126409_d_n6, assign83200_e126409_d_n7, assign83200_e126409_d_n8, assign83200_e126409_d_n9, assign83200_e126409_d_n10, assign83200_e126409_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) {
        let assign83200_e126407: f64 = (locals.var_qbuld / locals.var_q_nsubld__blk1883);
        (assign83200_e126407, (locals.var_qbuld_dn0 / locals.var_q_nsubld__blk1883), (locals.var_qbuld_dn2 / locals.var_q_nsubld__blk1883), (locals.var_qbuld_dn4 / locals.var_q_nsubld__blk1883), (locals.var_qbuld_dn5 / locals.var_q_nsubld__blk1883), (locals.var_qbuld_dn6 / locals.var_q_nsubld__blk1883), (locals.var_qbuld_dn7 / locals.var_q_nsubld__blk1883), (locals.var_qbuld_dn8 / locals.var_q_nsubld__blk1883), (locals.var_qbuld_dn9 / locals.var_q_nsubld__blk1883), (locals.var_qbuld_dn10 / locals.var_q_nsubld__blk1883), (locals.var_qbuld_dn13 / locals.var_q_nsubld__blk1883),)
    } else {
        (locals.var_wdld0__blk1927, locals.var_wdld0__blk1927_dn0, locals.var_wdld0__blk1927_dn2, locals.var_wdld0__blk1927_dn4, locals.var_wdld0__blk1927_dn5, locals.var_wdld0__blk1927_dn6, locals.var_wdld0__blk1927_dn7, locals.var_wdld0__blk1927_dn8, locals.var_wdld0__blk1927_dn9, locals.var_wdld0__blk1927_dn10, locals.var_wdld0__blk1927_dn13,)
    }
};
        locals.var_wdld0__blk1927 = assign83200_e126409;
        locals.var_wdld0__blk1927_dn0 = assign83200_e126409_d_n0;
        locals.var_wdld0__blk1927_dn2 = assign83200_e126409_d_n2;
        locals.var_wdld0__blk1927_dn4 = assign83200_e126409_d_n4;
        locals.var_wdld0__blk1927_dn5 = assign83200_e126409_d_n5;
        locals.var_wdld0__blk1927_dn6 = assign83200_e126409_d_n6;
        locals.var_wdld0__blk1927_dn7 = assign83200_e126409_d_n7;
        locals.var_wdld0__blk1927_dn8 = assign83200_e126409_d_n8;
        locals.var_wdld0__blk1927_dn9 = assign83200_e126409_d_n9;
        locals.var_wdld0__blk1927_dn10 = assign83200_e126409_d_n10;
        locals.var_wdld0__blk1927_dn13 = assign83200_e126409_d_n13;

        let assign83210_e126412: f64 = if p.p33 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard1929 = assign83210_e126412;

        let assign83220_e126417: f64 = (locals.var_ddriftldc * 0.1);
        let assign83220_e126418: f64 = (locals.var_ddriftldc - assign83220_e126417);
        let assign83220_e126422: f64 = (locals.var_ddriftldc * 0.1);
        let assign83220_e126425: f64 = if ((locals.var_wdld0__blk1927 > assign83220_e126418) && (assign83220_e126422 >= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1930 = assign83220_e126425;

        let (assign83230_e126442, assign83230_e126442_d_n0, assign83230_e126442_d_n2, assign83230_e126442_d_n4, assign83230_e126442_d_n5, assign83230_e126442_d_n6, assign83230_e126442_d_n7, assign83230_e126442_d_n8, assign83230_e126442_d_n9, assign83230_e126442_d_n10, assign83230_e126442_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1929 != 0.0)) && (locals.var_guard1930 != 0.0)) {
        let assign83230_e126436: f64 = (locals.var_wdld0__blk1927 - locals.var_ddriftldc);
        let assign83230_e126439: f64 = (locals.var_ddriftldc * 0.1);
        let assign83230_e126440: f64 = (assign83230_e126436 + assign83230_e126439);
        (assign83230_e126440, ((locals.var_wdld0__blk1927_dn0 - locals.var_ddriftldc_dn0) + (locals.var_ddriftldc_dn0 * 0.1)), ((locals.var_wdld0__blk1927_dn2 - locals.var_ddriftldc_dn2) + (locals.var_ddriftldc_dn2 * 0.1)), ((locals.var_wdld0__blk1927_dn4 - locals.var_ddriftldc_dn4) + (locals.var_ddriftldc_dn4 * 0.1)), ((locals.var_wdld0__blk1927_dn5 - locals.var_ddriftldc_dn5) + (locals.var_ddriftldc_dn5 * 0.1)), ((locals.var_wdld0__blk1927_dn6 - locals.var_ddriftldc_dn6) + (locals.var_ddriftldc_dn6 * 0.1)), ((locals.var_wdld0__blk1927_dn7 - locals.var_ddriftldc_dn7) + (locals.var_ddriftldc_dn7 * 0.1)), ((locals.var_wdld0__blk1927_dn8 - locals.var_ddriftldc_dn8) + (locals.var_ddriftldc_dn8 * 0.1)), ((locals.var_wdld0__blk1927_dn9 - locals.var_ddriftldc_dn9) + (locals.var_ddriftldc_dn9 * 0.1)), ((locals.var_wdld0__blk1927_dn10 - locals.var_ddriftldc_dn10) + (locals.var_ddriftldc_dn10 * 0.1)), ((locals.var_wdld0__blk1927_dn13 - locals.var_ddriftldc_dn13) + (locals.var_ddriftldc_dn13 * 0.1)),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13,)
    }
};
        locals.var_tmf1 = assign83230_e126442;
        locals.var_tmf1_dn0 = assign83230_e126442_d_n0;
        locals.var_tmf1_dn2 = assign83230_e126442_d_n2;
        locals.var_tmf1_dn4 = assign83230_e126442_d_n4;
        locals.var_tmf1_dn5 = assign83230_e126442_d_n5;
        locals.var_tmf1_dn6 = assign83230_e126442_d_n6;
        locals.var_tmf1_dn7 = assign83230_e126442_d_n7;
        locals.var_tmf1_dn8 = assign83230_e126442_d_n8;
        locals.var_tmf1_dn9 = assign83230_e126442_d_n9;
        locals.var_tmf1_dn10 = assign83230_e126442_d_n10;
        locals.var_tmf1_dn13 = assign83230_e126442_d_n13;

        let (assign83240_e126455, assign83240_e126455_d_n0, assign83240_e126455_d_n2, assign83240_e126455_d_n4, assign83240_e126455_d_n5, assign83240_e126455_d_n6, assign83240_e126455_d_n7, assign83240_e126455_d_n8, assign83240_e126455_d_n9, assign83240_e126455_d_n10, assign83240_e126455_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1929 != 0.0)) && (locals.var_guard1930 != 0.0)) {
        let assign83240_e126453: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign83240_e126453, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)), ((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)), ((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn13 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn13)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn4, locals.var_x2_dn5, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn8, locals.var_x2_dn9, locals.var_x2_dn10, locals.var_x2_dn13,)
    }
};
        locals.var_x2 = assign83240_e126455;
        locals.var_x2_dn0 = assign83240_e126455_d_n0;
        locals.var_x2_dn2 = assign83240_e126455_d_n2;
        locals.var_x2_dn4 = assign83240_e126455_d_n4;
        locals.var_x2_dn5 = assign83240_e126455_d_n5;
        locals.var_x2_dn6 = assign83240_e126455_d_n6;
        locals.var_x2_dn7 = assign83240_e126455_d_n7;
        locals.var_x2_dn8 = assign83240_e126455_d_n8;
        locals.var_x2_dn9 = assign83240_e126455_d_n9;
        locals.var_x2_dn10 = assign83240_e126455_d_n10;
        locals.var_x2_dn13 = assign83240_e126455_d_n13;

        let (assign83250_e126472, assign83250_e126472_d_n0, assign83250_e126472_d_n2, assign83250_e126472_d_n4, assign83250_e126472_d_n5, assign83250_e126472_d_n6, assign83250_e126472_d_n7, assign83250_e126472_d_n8, assign83250_e126472_d_n9, assign83250_e126472_d_n10, assign83250_e126472_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1929 != 0.0)) && (locals.var_guard1930 != 0.0)) {
        let assign83250_e126466: f64 = (locals.var_ddriftldc * 0.1);
        let assign83250_e126469: f64 = (locals.var_ddriftldc * 0.1);
        let assign83250_e126470: f64 = (assign83250_e126466 * assign83250_e126469);
        (assign83250_e126470, (((locals.var_ddriftldc_dn0 * 0.1) * assign83250_e126469) + (assign83250_e126466 * (locals.var_ddriftldc_dn0 * 0.1))), (((locals.var_ddriftldc_dn2 * 0.1) * assign83250_e126469) + (assign83250_e126466 * (locals.var_ddriftldc_dn2 * 0.1))), (((locals.var_ddriftldc_dn4 * 0.1) * assign83250_e126469) + (assign83250_e126466 * (locals.var_ddriftldc_dn4 * 0.1))), (((locals.var_ddriftldc_dn5 * 0.1) * assign83250_e126469) + (assign83250_e126466 * (locals.var_ddriftldc_dn5 * 0.1))), (((locals.var_ddriftldc_dn6 * 0.1) * assign83250_e126469) + (assign83250_e126466 * (locals.var_ddriftldc_dn6 * 0.1))), (((locals.var_ddriftldc_dn7 * 0.1) * assign83250_e126469) + (assign83250_e126466 * (locals.var_ddriftldc_dn7 * 0.1))), (((locals.var_ddriftldc_dn8 * 0.1) * assign83250_e126469) + (assign83250_e126466 * (locals.var_ddriftldc_dn8 * 0.1))), (((locals.var_ddriftldc_dn9 * 0.1) * assign83250_e126469) + (assign83250_e126466 * (locals.var_ddriftldc_dn9 * 0.1))), (((locals.var_ddriftldc_dn10 * 0.1) * assign83250_e126469) + (assign83250_e126466 * (locals.var_ddriftldc_dn10 * 0.1))), (((locals.var_ddriftldc_dn13 * 0.1) * assign83250_e126469) + (assign83250_e126466 * (locals.var_ddriftldc_dn13 * 0.1))),)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn4, locals.var_xmax2_dn5, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn8, locals.var_xmax2_dn9, locals.var_xmax2_dn10, locals.var_xmax2_dn13,)
    }
};
        locals.var_xmax2 = assign83250_e126472;
        locals.var_xmax2_dn0 = assign83250_e126472_d_n0;
        locals.var_xmax2_dn2 = assign83250_e126472_d_n2;
        locals.var_xmax2_dn4 = assign83250_e126472_d_n4;
        locals.var_xmax2_dn5 = assign83250_e126472_d_n5;
        locals.var_xmax2_dn6 = assign83250_e126472_d_n6;
        locals.var_xmax2_dn7 = assign83250_e126472_d_n7;
        locals.var_xmax2_dn8 = assign83250_e126472_d_n8;
        locals.var_xmax2_dn9 = assign83250_e126472_d_n9;
        locals.var_xmax2_dn10 = assign83250_e126472_d_n10;
        locals.var_xmax2_dn13 = assign83250_e126472_d_n13;

        let (assign83260_e126483, assign83260_e126483_d_n0, assign83260_e126483_d_n2, assign83260_e126483_d_n4, assign83260_e126483_d_n5, assign83260_e126483_d_n6, assign83260_e126483_d_n7, assign83260_e126483_d_n8, assign83260_e126483_d_n9, assign83260_e126483_d_n10, assign83260_e126483_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1929 != 0.0)) && (locals.var_guard1930 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
        locals.var_xp = assign83260_e126483;
        locals.var_xp_dn0 = assign83260_e126483_d_n0;
        locals.var_xp_dn2 = assign83260_e126483_d_n2;
        locals.var_xp_dn4 = assign83260_e126483_d_n4;
        locals.var_xp_dn5 = assign83260_e126483_d_n5;
        locals.var_xp_dn6 = assign83260_e126483_d_n6;
        locals.var_xp_dn7 = assign83260_e126483_d_n7;
        locals.var_xp_dn8 = assign83260_e126483_d_n8;
        locals.var_xp_dn9 = assign83260_e126483_d_n9;
        locals.var_xp_dn10 = assign83260_e126483_d_n10;
        locals.var_xp_dn13 = assign83260_e126483_d_n13;

        let (assign83270_e126494, assign83270_e126494_d_n0, assign83270_e126494_d_n2, assign83270_e126494_d_n4, assign83270_e126494_d_n5, assign83270_e126494_d_n6, assign83270_e126494_d_n7, assign83270_e126494_d_n8, assign83270_e126494_d_n9, assign83270_e126494_d_n10, assign83270_e126494_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1929 != 0.0)) && (locals.var_guard1930 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
        locals.var_xmp = assign83270_e126494;
        locals.var_xmp_dn0 = assign83270_e126494_d_n0;
        locals.var_xmp_dn2 = assign83270_e126494_d_n2;
        locals.var_xmp_dn4 = assign83270_e126494_d_n4;
        locals.var_xmp_dn5 = assign83270_e126494_d_n5;
        locals.var_xmp_dn6 = assign83270_e126494_d_n6;
        locals.var_xmp_dn7 = assign83270_e126494_d_n7;
        locals.var_xmp_dn8 = assign83270_e126494_d_n8;
        locals.var_xmp_dn9 = assign83270_e126494_d_n9;
        locals.var_xmp_dn10 = assign83270_e126494_d_n10;
        locals.var_xmp_dn13 = assign83270_e126494_d_n13;

        let (assign83280_e126505,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1929 != 0.0)) && (locals.var_guard1930 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign83280_e126505;

        let (assign83290_e126516,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1929 != 0.0)) && (locals.var_guard1930 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign83290_e126516;

    }

    pub(super) fn stamp_transient_block_290(
        locals: &mut StampLocals,
    ) {
        let (assign83300_e126527, assign83300_e126527_d_n0, assign83300_e126527_d_n2, assign83300_e126527_d_n4, assign83300_e126527_d_n5, assign83300_e126527_d_n6, assign83300_e126527_d_n7, assign83300_e126527_d_n8, assign83300_e126527_d_n9, assign83300_e126527_d_n10, assign83300_e126527_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1929 != 0.0)) && (locals.var_guard1930 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    }
};
        locals.var_arg = assign83300_e126527;
        locals.var_arg_dn0 = assign83300_e126527_d_n0;
        locals.var_arg_dn2 = assign83300_e126527_d_n2;
        locals.var_arg_dn4 = assign83300_e126527_d_n4;
        locals.var_arg_dn5 = assign83300_e126527_d_n5;
        locals.var_arg_dn6 = assign83300_e126527_d_n6;
        locals.var_arg_dn7 = assign83300_e126527_d_n7;
        locals.var_arg_dn8 = assign83300_e126527_d_n8;
        locals.var_arg_dn9 = assign83300_e126527_d_n9;
        locals.var_arg_dn10 = assign83300_e126527_d_n10;
        locals.var_arg_dn13 = assign83300_e126527_d_n13;

        let (assign83310_e126538, assign83310_e126538_d_n0, assign83310_e126538_d_n2, assign83310_e126538_d_n4, assign83310_e126538_d_n5, assign83310_e126538_d_n6, assign83310_e126538_d_n7, assign83310_e126538_d_n8, assign83310_e126538_d_n9, assign83310_e126538_d_n10, assign83310_e126538_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1929 != 0.0)) && (locals.var_guard1930 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign83310_e126538;
        locals.var_dnm_dn0 = assign83310_e126538_d_n0;
        locals.var_dnm_dn2 = assign83310_e126538_d_n2;
        locals.var_dnm_dn4 = assign83310_e126538_d_n4;
        locals.var_dnm_dn5 = assign83310_e126538_d_n5;
        locals.var_dnm_dn6 = assign83310_e126538_d_n6;
        locals.var_dnm_dn7 = assign83310_e126538_d_n7;
        locals.var_dnm_dn8 = assign83310_e126538_d_n8;
        locals.var_dnm_dn9 = assign83310_e126538_d_n9;
        locals.var_dnm_dn10 = assign83310_e126538_d_n10;
        locals.var_dnm_dn13 = assign83310_e126538_d_n13;

        let (assign83320_e126551, assign83320_e126551_d_n0, assign83320_e126551_d_n2, assign83320_e126551_d_n4, assign83320_e126551_d_n5, assign83320_e126551_d_n6, assign83320_e126551_d_n7, assign83320_e126551_d_n8, assign83320_e126551_d_n9, assign83320_e126551_d_n10, assign83320_e126551_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1929 != 0.0)) && (locals.var_guard1930 != 0.0)) {
        let assign83320_e126549: f64 = (locals.var_xp * locals.var_x2);
        (assign83320_e126549, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn13 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn13)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
        locals.var_xp = assign83320_e126551;
        locals.var_xp_dn0 = assign83320_e126551_d_n0;
        locals.var_xp_dn2 = assign83320_e126551_d_n2;
        locals.var_xp_dn4 = assign83320_e126551_d_n4;
        locals.var_xp_dn5 = assign83320_e126551_d_n5;
        locals.var_xp_dn6 = assign83320_e126551_d_n6;
        locals.var_xp_dn7 = assign83320_e126551_d_n7;
        locals.var_xp_dn8 = assign83320_e126551_d_n8;
        locals.var_xp_dn9 = assign83320_e126551_d_n9;
        locals.var_xp_dn10 = assign83320_e126551_d_n10;
        locals.var_xp_dn13 = assign83320_e126551_d_n13;

        let (assign83330_e126564, assign83330_e126564_d_n0, assign83330_e126564_d_n2, assign83330_e126564_d_n4, assign83330_e126564_d_n5, assign83330_e126564_d_n6, assign83330_e126564_d_n7, assign83330_e126564_d_n8, assign83330_e126564_d_n9, assign83330_e126564_d_n10, assign83330_e126564_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1929 != 0.0)) && (locals.var_guard1930 != 0.0)) {
        let assign83330_e126562: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign83330_e126562, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn13 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn13)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
        locals.var_xmp = assign83330_e126564;
        locals.var_xmp_dn0 = assign83330_e126564_d_n0;
        locals.var_xmp_dn2 = assign83330_e126564_d_n2;
        locals.var_xmp_dn4 = assign83330_e126564_d_n4;
        locals.var_xmp_dn5 = assign83330_e126564_d_n5;
        locals.var_xmp_dn6 = assign83330_e126564_d_n6;
        locals.var_xmp_dn7 = assign83330_e126564_d_n7;
        locals.var_xmp_dn8 = assign83330_e126564_d_n8;
        locals.var_xmp_dn9 = assign83330_e126564_d_n9;
        locals.var_xmp_dn10 = assign83330_e126564_d_n10;
        locals.var_xmp_dn13 = assign83330_e126564_d_n13;

        let (assign83340_e126577, assign83340_e126577_d_n0, assign83340_e126577_d_n2, assign83340_e126577_d_n4, assign83340_e126577_d_n5, assign83340_e126577_d_n6, assign83340_e126577_d_n7, assign83340_e126577_d_n8, assign83340_e126577_d_n9, assign83340_e126577_d_n10, assign83340_e126577_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1929 != 0.0)) && (locals.var_guard1930 != 0.0)) {
        let assign83340_e126575: f64 = (locals.var_xp * locals.var_x2);
        (assign83340_e126575, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn13 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn13)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
        locals.var_xp = assign83340_e126577;
        locals.var_xp_dn0 = assign83340_e126577_d_n0;
        locals.var_xp_dn2 = assign83340_e126577_d_n2;
        locals.var_xp_dn4 = assign83340_e126577_d_n4;
        locals.var_xp_dn5 = assign83340_e126577_d_n5;
        locals.var_xp_dn6 = assign83340_e126577_d_n6;
        locals.var_xp_dn7 = assign83340_e126577_d_n7;
        locals.var_xp_dn8 = assign83340_e126577_d_n8;
        locals.var_xp_dn9 = assign83340_e126577_d_n9;
        locals.var_xp_dn10 = assign83340_e126577_d_n10;
        locals.var_xp_dn13 = assign83340_e126577_d_n13;

        let (assign83350_e126590, assign83350_e126590_d_n0, assign83350_e126590_d_n2, assign83350_e126590_d_n4, assign83350_e126590_d_n5, assign83350_e126590_d_n6, assign83350_e126590_d_n7, assign83350_e126590_d_n8, assign83350_e126590_d_n9, assign83350_e126590_d_n10, assign83350_e126590_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1929 != 0.0)) && (locals.var_guard1930 != 0.0)) {
        let assign83350_e126588: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign83350_e126588, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn13 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn13)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
        locals.var_xmp = assign83350_e126590;
        locals.var_xmp_dn0 = assign83350_e126590_d_n0;
        locals.var_xmp_dn2 = assign83350_e126590_d_n2;
        locals.var_xmp_dn4 = assign83350_e126590_d_n4;
        locals.var_xmp_dn5 = assign83350_e126590_d_n5;
        locals.var_xmp_dn6 = assign83350_e126590_d_n6;
        locals.var_xmp_dn7 = assign83350_e126590_d_n7;
        locals.var_xmp_dn8 = assign83350_e126590_d_n8;
        locals.var_xmp_dn9 = assign83350_e126590_d_n9;
        locals.var_xmp_dn10 = assign83350_e126590_d_n10;
        locals.var_xmp_dn13 = assign83350_e126590_d_n13;

        let (assign83360_e126603, assign83360_e126603_d_n0, assign83360_e126603_d_n2, assign83360_e126603_d_n4, assign83360_e126603_d_n5, assign83360_e126603_d_n6, assign83360_e126603_d_n7, assign83360_e126603_d_n8, assign83360_e126603_d_n9, assign83360_e126603_d_n10, assign83360_e126603_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1929 != 0.0)) && (locals.var_guard1930 != 0.0)) {
        let assign83360_e126601: f64 = (locals.var_xp + locals.var_xmp);
        (assign83360_e126601, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn4 + locals.var_xmp_dn4), (locals.var_xp_dn5 + locals.var_xmp_dn5), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn8 + locals.var_xmp_dn8), (locals.var_xp_dn9 + locals.var_xmp_dn9), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn13 + locals.var_xmp_dn13),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    }
};
        locals.var_arg = assign83360_e126603;
        locals.var_arg_dn0 = assign83360_e126603_d_n0;
        locals.var_arg_dn2 = assign83360_e126603_d_n2;
        locals.var_arg_dn4 = assign83360_e126603_d_n4;
        locals.var_arg_dn5 = assign83360_e126603_d_n5;
        locals.var_arg_dn6 = assign83360_e126603_d_n6;
        locals.var_arg_dn7 = assign83360_e126603_d_n7;
        locals.var_arg_dn8 = assign83360_e126603_d_n8;
        locals.var_arg_dn9 = assign83360_e126603_d_n9;
        locals.var_arg_dn10 = assign83360_e126603_d_n10;
        locals.var_arg_dn13 = assign83360_e126603_d_n13;

        let (assign83370_e126614, assign83370_e126614_d_n0, assign83370_e126614_d_n2, assign83370_e126614_d_n4, assign83370_e126614_d_n5, assign83370_e126614_d_n6, assign83370_e126614_d_n7, assign83370_e126614_d_n8, assign83370_e126614_d_n9, assign83370_e126614_d_n10, assign83370_e126614_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1929 != 0.0)) && (locals.var_guard1930 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign83370_e126614;
        locals.var_dnm_dn0 = assign83370_e126614_d_n0;
        locals.var_dnm_dn2 = assign83370_e126614_d_n2;
        locals.var_dnm_dn4 = assign83370_e126614_d_n4;
        locals.var_dnm_dn5 = assign83370_e126614_d_n5;
        locals.var_dnm_dn6 = assign83370_e126614_d_n6;
        locals.var_dnm_dn7 = assign83370_e126614_d_n7;
        locals.var_dnm_dn8 = assign83370_e126614_d_n8;
        locals.var_dnm_dn9 = assign83370_e126614_d_n9;
        locals.var_dnm_dn10 = assign83370_e126614_d_n10;
        locals.var_dnm_dn13 = assign83370_e126614_d_n13;

        let assign83380_e126629: f64 = if ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard1931 = assign83380_e126629;

        let assign83390_e126632: f64 = if 2.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1932 = assign83390_e126632;

        let (assign83400_e126647,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1929 != 0.0)) && (locals.var_guard1930 != 0.0)) && (locals.var_guard1931 != 0.0)) && (locals.var_guard1932 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign83400_e126647;

        let assign83410_e126650: f64 = if 2.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard1933 = assign83410_e126650;

        let (assign83420_e126668,) = {
    if (((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1929 != 0.0)) && (locals.var_guard1930 != 0.0)) && (locals.var_guard1931 != 0.0)) && (locals.var_guard1932 == 0.0)) && (locals.var_guard1933 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign83420_e126668;

        let assign83430_e126671: f64 = if 2.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard1934 = assign83430_e126671;

        let (assign83440_e126692,) = {
    if ((((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1929 != 0.0)) && (locals.var_guard1930 != 0.0)) && (locals.var_guard1931 != 0.0)) && (locals.var_guard1932 == 0.0)) && (locals.var_guard1933 == 0.0)) && (locals.var_guard1934 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign83440_e126692;

        let assign83450_e126695: f64 = if 2.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard1935 = assign83450_e126695;

        let (assign83460_e126719,) = {
    if (((((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1929 != 0.0)) && (locals.var_guard1930 != 0.0)) && (locals.var_guard1931 != 0.0)) && (locals.var_guard1932 == 0.0)) && (locals.var_guard1933 == 0.0)) && (locals.var_guard1934 == 0.0)) && (locals.var_guard1935 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign83460_e126719;

        let (assign83470_e126732,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1929 != 0.0)) && (locals.var_guard1930 != 0.0)) && (locals.var_guard1931 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign83470_e126732;

        let mut assign83480_loop_guard: usize = 0;
        while {
            let assign83480_cond_e126746: f64 = if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1929 != 0.0)) && (locals.var_guard1930 != 0.0)) && (locals.var_guard1931 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign83480_cond_e126746 != 0.0
        } {
            assign83480_loop_guard += 1;
            assert!(assign83480_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign83480_body0_e126760, assign83480_body0_e126760_d_n0, assign83480_body0_e126760_d_n2, assign83480_body0_e126760_d_n4, assign83480_body0_e126760_d_n5, assign83480_body0_e126760_d_n6, assign83480_body0_e126760_d_n7, assign83480_body0_e126760_d_n8, assign83480_body0_e126760_d_n9, assign83480_body0_e126760_d_n10, assign83480_body0_e126760_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1929 != 0.0)) && (locals.var_guard1930 != 0.0)) && (locals.var_guard1931 != 0.0)) {
        let assign83480_body0_e126758: f64 = (locals.var_dnm).sqrt();
        (assign83480_body0_e126758, (locals.var_dnm_dn0 / (2.0 * assign83480_body0_e126758)), (locals.var_dnm_dn2 / (2.0 * assign83480_body0_e126758)), (locals.var_dnm_dn4 / (2.0 * assign83480_body0_e126758)), (locals.var_dnm_dn5 / (2.0 * assign83480_body0_e126758)), (locals.var_dnm_dn6 / (2.0 * assign83480_body0_e126758)), (locals.var_dnm_dn7 / (2.0 * assign83480_body0_e126758)), (locals.var_dnm_dn8 / (2.0 * assign83480_body0_e126758)), (locals.var_dnm_dn9 / (2.0 * assign83480_body0_e126758)), (locals.var_dnm_dn10 / (2.0 * assign83480_body0_e126758)), (locals.var_dnm_dn13 / (2.0 * assign83480_body0_e126758)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
            locals.var_dnm = assign83480_body0_e126760;
            locals.var_dnm_dn0 = assign83480_body0_e126760_d_n0;
            locals.var_dnm_dn2 = assign83480_body0_e126760_d_n2;
            locals.var_dnm_dn4 = assign83480_body0_e126760_d_n4;
            locals.var_dnm_dn5 = assign83480_body0_e126760_d_n5;
            locals.var_dnm_dn6 = assign83480_body0_e126760_d_n6;
            locals.var_dnm_dn7 = assign83480_body0_e126760_d_n7;
            locals.var_dnm_dn8 = assign83480_body0_e126760_d_n8;
            locals.var_dnm_dn9 = assign83480_body0_e126760_d_n9;
            locals.var_dnm_dn10 = assign83480_body0_e126760_d_n10;
            locals.var_dnm_dn13 = assign83480_body0_e126760_d_n13;
            let (assign83480_body1_e126775,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1929 != 0.0)) && (locals.var_guard1930 != 0.0)) && (locals.var_guard1931 != 0.0)) {
        let assign83480_body1_e126773: f64 = (locals.var_m0 + 1.0);
        (assign83480_body1_e126773,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign83480_body1_e126775;
        }

        let (assign83490_e126800, assign83490_e126800_d_n0, assign83490_e126800_d_n2, assign83490_e126800_d_n4, assign83490_e126800_d_n5, assign83490_e126800_d_n6, assign83490_e126800_d_n7, assign83490_e126800_d_n8, assign83490_e126800_d_n9, assign83490_e126800_d_n10, assign83490_e126800_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1929 != 0.0)) && (locals.var_guard1930 != 0.0)) && (locals.var_guard1931 == 0.0)) {
        let (assign83490_e126798, assign83490_e126798_d_n0, assign83490_e126798_d_n2, assign83490_e126798_d_n4, assign83490_e126798_d_n5, assign83490_e126798_d_n6, assign83490_e126798_d_n7, assign83490_e126798_d_n8, assign83490_e126798_d_n9, assign83490_e126798_d_n10, assign83490_e126798_d_n13,) = {
            if (locals.var_dnm == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign83490_e126795: f64 = (2.0 * 2.0);
                let assign83490_e126796: f64 = (1.0 / assign83490_e126795);
                let assign83490_e126797: f64 = (locals.var_dnm).powf(assign83490_e126796);
                (assign83490_e126797, if 0.0 == 0.0 && ((assign83490_e126796) as f64).is_finite() && ((assign83490_e126796) as f64).fract() == 0.0 { if assign83490_e126796 == 0.0 { 0.0 } else { (assign83490_e126796 * ((locals.var_dnm).powf(assign83490_e126796 - 1.0) * locals.var_dnm_dn0)) } } else { (assign83490_e126797 * (assign83490_e126796 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign83490_e126796) as f64).is_finite() && ((assign83490_e126796) as f64).fract() == 0.0 { if assign83490_e126796 == 0.0 { 0.0 } else { (assign83490_e126796 * ((locals.var_dnm).powf(assign83490_e126796 - 1.0) * locals.var_dnm_dn2)) } } else { (assign83490_e126797 * (assign83490_e126796 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign83490_e126796) as f64).is_finite() && ((assign83490_e126796) as f64).fract() == 0.0 { if assign83490_e126796 == 0.0 { 0.0 } else { (assign83490_e126796 * ((locals.var_dnm).powf(assign83490_e126796 - 1.0) * locals.var_dnm_dn4)) } } else { (assign83490_e126797 * (assign83490_e126796 * (locals.var_dnm_dn4 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign83490_e126796) as f64).is_finite() && ((assign83490_e126796) as f64).fract() == 0.0 { if assign83490_e126796 == 0.0 { 0.0 } else { (assign83490_e126796 * ((locals.var_dnm).powf(assign83490_e126796 - 1.0) * locals.var_dnm_dn5)) } } else { (assign83490_e126797 * (assign83490_e126796 * (locals.var_dnm_dn5 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign83490_e126796) as f64).is_finite() && ((assign83490_e126796) as f64).fract() == 0.0 { if assign83490_e126796 == 0.0 { 0.0 } else { (assign83490_e126796 * ((locals.var_dnm).powf(assign83490_e126796 - 1.0) * locals.var_dnm_dn6)) } } else { (assign83490_e126797 * (assign83490_e126796 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign83490_e126796) as f64).is_finite() && ((assign83490_e126796) as f64).fract() == 0.0 { if assign83490_e126796 == 0.0 { 0.0 } else { (assign83490_e126796 * ((locals.var_dnm).powf(assign83490_e126796 - 1.0) * locals.var_dnm_dn7)) } } else { (assign83490_e126797 * (assign83490_e126796 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign83490_e126796) as f64).is_finite() && ((assign83490_e126796) as f64).fract() == 0.0 { if assign83490_e126796 == 0.0 { 0.0 } else { (assign83490_e126796 * ((locals.var_dnm).powf(assign83490_e126796 - 1.0) * locals.var_dnm_dn8)) } } else { (assign83490_e126797 * (assign83490_e126796 * (locals.var_dnm_dn8 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign83490_e126796) as f64).is_finite() && ((assign83490_e126796) as f64).fract() == 0.0 { if assign83490_e126796 == 0.0 { 0.0 } else { (assign83490_e126796 * ((locals.var_dnm).powf(assign83490_e126796 - 1.0) * locals.var_dnm_dn9)) } } else { (assign83490_e126797 * (assign83490_e126796 * (locals.var_dnm_dn9 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign83490_e126796) as f64).is_finite() && ((assign83490_e126796) as f64).fract() == 0.0 { if assign83490_e126796 == 0.0 { 0.0 } else { (assign83490_e126796 * ((locals.var_dnm).powf(assign83490_e126796 - 1.0) * locals.var_dnm_dn10)) } } else { (assign83490_e126797 * (assign83490_e126796 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign83490_e126796) as f64).is_finite() && ((assign83490_e126796) as f64).fract() == 0.0 { if assign83490_e126796 == 0.0 { 0.0 } else { (assign83490_e126796 * ((locals.var_dnm).powf(assign83490_e126796 - 1.0) * locals.var_dnm_dn13)) } } else { (assign83490_e126797 * (assign83490_e126796 * (locals.var_dnm_dn13 / locals.var_dnm))) },)
            }
        };
        (assign83490_e126798, assign83490_e126798_d_n0, assign83490_e126798_d_n2, assign83490_e126798_d_n4, assign83490_e126798_d_n5, assign83490_e126798_d_n6, assign83490_e126798_d_n7, assign83490_e126798_d_n8, assign83490_e126798_d_n9, assign83490_e126798_d_n10, assign83490_e126798_d_n13,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign83490_e126800;
        locals.var_dnm_dn0 = assign83490_e126800_d_n0;
        locals.var_dnm_dn2 = assign83490_e126800_d_n2;
        locals.var_dnm_dn4 = assign83490_e126800_d_n4;
        locals.var_dnm_dn5 = assign83490_e126800_d_n5;
        locals.var_dnm_dn6 = assign83490_e126800_d_n6;
        locals.var_dnm_dn7 = assign83490_e126800_d_n7;
        locals.var_dnm_dn8 = assign83490_e126800_d_n8;
        locals.var_dnm_dn9 = assign83490_e126800_d_n9;
        locals.var_dnm_dn10 = assign83490_e126800_d_n10;
        locals.var_dnm_dn13 = assign83490_e126800_d_n13;

        let (assign83500_e126813, assign83500_e126813_d_n0, assign83500_e126813_d_n2, assign83500_e126813_d_n4, assign83500_e126813_d_n5, assign83500_e126813_d_n6, assign83500_e126813_d_n7, assign83500_e126813_d_n8, assign83500_e126813_d_n9, assign83500_e126813_d_n10, assign83500_e126813_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1929 != 0.0)) && (locals.var_guard1930 != 0.0)) {
        let assign83500_e126811: f64 = (1.0 / locals.var_dnm);
        (assign83500_e126811, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn4 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn5 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn8 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn9 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn13 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign83500_e126813;
        locals.var_dnm_dn0 = assign83500_e126813_d_n0;
        locals.var_dnm_dn2 = assign83500_e126813_d_n2;
        locals.var_dnm_dn4 = assign83500_e126813_d_n4;
        locals.var_dnm_dn5 = assign83500_e126813_d_n5;
        locals.var_dnm_dn6 = assign83500_e126813_d_n6;
        locals.var_dnm_dn7 = assign83500_e126813_d_n7;
        locals.var_dnm_dn8 = assign83500_e126813_d_n8;
        locals.var_dnm_dn9 = assign83500_e126813_d_n9;
        locals.var_dnm_dn10 = assign83500_e126813_d_n10;
        locals.var_dnm_dn13 = assign83500_e126813_d_n13;

        let (assign83510_e126830, assign83510_e126830_d_n0, assign83510_e126830_d_n2, assign83510_e126830_d_n4, assign83510_e126830_d_n5, assign83510_e126830_d_n6, assign83510_e126830_d_n7, assign83510_e126830_d_n8, assign83510_e126830_d_n9, assign83510_e126830_d_n10, assign83510_e126830_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1929 != 0.0)) && (locals.var_guard1930 != 0.0)) {
        let assign83510_e126825: f64 = (locals.var_ddriftldc * 0.1);
        let assign83510_e126826: f64 = (locals.var_tmf1 * assign83510_e126825);
        let assign83510_e126828: f64 = (assign83510_e126826 * locals.var_dnm);
        (assign83510_e126828, ((((locals.var_tmf1_dn0 * assign83510_e126825) + (locals.var_tmf1 * (locals.var_ddriftldc_dn0 * 0.1))) * locals.var_dnm) + (assign83510_e126826 * locals.var_dnm_dn0)), ((((locals.var_tmf1_dn2 * assign83510_e126825) + (locals.var_tmf1 * (locals.var_ddriftldc_dn2 * 0.1))) * locals.var_dnm) + (assign83510_e126826 * locals.var_dnm_dn2)), ((((locals.var_tmf1_dn4 * assign83510_e126825) + (locals.var_tmf1 * (locals.var_ddriftldc_dn4 * 0.1))) * locals.var_dnm) + (assign83510_e126826 * locals.var_dnm_dn4)), ((((locals.var_tmf1_dn5 * assign83510_e126825) + (locals.var_tmf1 * (locals.var_ddriftldc_dn5 * 0.1))) * locals.var_dnm) + (assign83510_e126826 * locals.var_dnm_dn5)), ((((locals.var_tmf1_dn6 * assign83510_e126825) + (locals.var_tmf1 * (locals.var_ddriftldc_dn6 * 0.1))) * locals.var_dnm) + (assign83510_e126826 * locals.var_dnm_dn6)), ((((locals.var_tmf1_dn7 * assign83510_e126825) + (locals.var_tmf1 * (locals.var_ddriftldc_dn7 * 0.1))) * locals.var_dnm) + (assign83510_e126826 * locals.var_dnm_dn7)), ((((locals.var_tmf1_dn8 * assign83510_e126825) + (locals.var_tmf1 * (locals.var_ddriftldc_dn8 * 0.1))) * locals.var_dnm) + (assign83510_e126826 * locals.var_dnm_dn8)), ((((locals.var_tmf1_dn9 * assign83510_e126825) + (locals.var_tmf1 * (locals.var_ddriftldc_dn9 * 0.1))) * locals.var_dnm) + (assign83510_e126826 * locals.var_dnm_dn9)), ((((locals.var_tmf1_dn10 * assign83510_e126825) + (locals.var_tmf1 * (locals.var_ddriftldc_dn10 * 0.1))) * locals.var_dnm) + (assign83510_e126826 * locals.var_dnm_dn10)), ((((locals.var_tmf1_dn13 * assign83510_e126825) + (locals.var_tmf1 * (locals.var_ddriftldc_dn13 * 0.1))) * locals.var_dnm) + (assign83510_e126826 * locals.var_dnm_dn13)),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn13,)
    }
};
        locals.var_tmf0 = assign83510_e126830;
        locals.var_tmf0_dn0 = assign83510_e126830_d_n0;
        locals.var_tmf0_dn2 = assign83510_e126830_d_n2;
        locals.var_tmf0_dn4 = assign83510_e126830_d_n4;
        locals.var_tmf0_dn5 = assign83510_e126830_d_n5;
        locals.var_tmf0_dn6 = assign83510_e126830_d_n6;
        locals.var_tmf0_dn7 = assign83510_e126830_d_n7;
        locals.var_tmf0_dn8 = assign83510_e126830_d_n8;
        locals.var_tmf0_dn9 = assign83510_e126830_d_n9;
        locals.var_tmf0_dn10 = assign83510_e126830_d_n10;
        locals.var_tmf0_dn13 = assign83510_e126830_d_n13;

        let (assign83520_e126849, assign83520_e126849_d_n0, assign83520_e126849_d_n2, assign83520_e126849_d_n4, assign83520_e126849_d_n5, assign83520_e126849_d_n6, assign83520_e126849_d_n7, assign83520_e126849_d_n8, assign83520_e126849_d_n9, assign83520_e126849_d_n10, assign83520_e126849_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1929 != 0.0)) && (locals.var_guard1930 != 0.0)) {
        let assign83520_e126841: f64 = (locals.var_ddriftldc * 0.1);
        let assign83520_e126843: f64 = (assign83520_e126841 * locals.var_xmp);
        let assign83520_e126845: f64 = (assign83520_e126843 * locals.var_dnm);
        let assign83520_e126847: f64 = (assign83520_e126845 / locals.var_arg);
        (assign83520_e126847, ((((((((locals.var_ddriftldc_dn0 * 0.1) * locals.var_xmp) + (assign83520_e126841 * locals.var_xmp_dn0)) * locals.var_dnm) + (assign83520_e126843 * locals.var_dnm_dn0)) * locals.var_arg) - (assign83520_e126845 * locals.var_arg_dn0)) / (locals.var_arg * locals.var_arg)), ((((((((locals.var_ddriftldc_dn2 * 0.1) * locals.var_xmp) + (assign83520_e126841 * locals.var_xmp_dn2)) * locals.var_dnm) + (assign83520_e126843 * locals.var_dnm_dn2)) * locals.var_arg) - (assign83520_e126845 * locals.var_arg_dn2)) / (locals.var_arg * locals.var_arg)), ((((((((locals.var_ddriftldc_dn4 * 0.1) * locals.var_xmp) + (assign83520_e126841 * locals.var_xmp_dn4)) * locals.var_dnm) + (assign83520_e126843 * locals.var_dnm_dn4)) * locals.var_arg) - (assign83520_e126845 * locals.var_arg_dn4)) / (locals.var_arg * locals.var_arg)), ((((((((locals.var_ddriftldc_dn5 * 0.1) * locals.var_xmp) + (assign83520_e126841 * locals.var_xmp_dn5)) * locals.var_dnm) + (assign83520_e126843 * locals.var_dnm_dn5)) * locals.var_arg) - (assign83520_e126845 * locals.var_arg_dn5)) / (locals.var_arg * locals.var_arg)), ((((((((locals.var_ddriftldc_dn6 * 0.1) * locals.var_xmp) + (assign83520_e126841 * locals.var_xmp_dn6)) * locals.var_dnm) + (assign83520_e126843 * locals.var_dnm_dn6)) * locals.var_arg) - (assign83520_e126845 * locals.var_arg_dn6)) / (locals.var_arg * locals.var_arg)), ((((((((locals.var_ddriftldc_dn7 * 0.1) * locals.var_xmp) + (assign83520_e126841 * locals.var_xmp_dn7)) * locals.var_dnm) + (assign83520_e126843 * locals.var_dnm_dn7)) * locals.var_arg) - (assign83520_e126845 * locals.var_arg_dn7)) / (locals.var_arg * locals.var_arg)), ((((((((locals.var_ddriftldc_dn8 * 0.1) * locals.var_xmp) + (assign83520_e126841 * locals.var_xmp_dn8)) * locals.var_dnm) + (assign83520_e126843 * locals.var_dnm_dn8)) * locals.var_arg) - (assign83520_e126845 * locals.var_arg_dn8)) / (locals.var_arg * locals.var_arg)), ((((((((locals.var_ddriftldc_dn9 * 0.1) * locals.var_xmp) + (assign83520_e126841 * locals.var_xmp_dn9)) * locals.var_dnm) + (assign83520_e126843 * locals.var_dnm_dn9)) * locals.var_arg) - (assign83520_e126845 * locals.var_arg_dn9)) / (locals.var_arg * locals.var_arg)), ((((((((locals.var_ddriftldc_dn10 * 0.1) * locals.var_xmp) + (assign83520_e126841 * locals.var_xmp_dn10)) * locals.var_dnm) + (assign83520_e126843 * locals.var_dnm_dn10)) * locals.var_arg) - (assign83520_e126845 * locals.var_arg_dn10)) / (locals.var_arg * locals.var_arg)), ((((((((locals.var_ddriftldc_dn13 * 0.1) * locals.var_xmp) + (assign83520_e126841 * locals.var_xmp_dn13)) * locals.var_dnm) + (assign83520_e126843 * locals.var_dnm_dn13)) * locals.var_arg) - (assign83520_e126845 * locals.var_arg_dn13)) / (locals.var_arg * locals.var_arg)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign83520_e126849;
        locals.var_t0_dn0 = assign83520_e126849_d_n0;
        locals.var_t0_dn2 = assign83520_e126849_d_n2;
        locals.var_t0_dn4 = assign83520_e126849_d_n4;
        locals.var_t0_dn5 = assign83520_e126849_d_n5;
        locals.var_t0_dn6 = assign83520_e126849_d_n6;
        locals.var_t0_dn7 = assign83520_e126849_d_n7;
        locals.var_t0_dn8 = assign83520_e126849_d_n8;
        locals.var_t0_dn9 = assign83520_e126849_d_n9;
        locals.var_t0_dn10 = assign83520_e126849_d_n10;
        locals.var_t0_dn13 = assign83520_e126849_d_n13;

        let (assign83530_e126866, assign83530_e126866_d_n0, assign83530_e126866_d_n2, assign83530_e126866_d_n4, assign83530_e126866_d_n5, assign83530_e126866_d_n6, assign83530_e126866_d_n7, assign83530_e126866_d_n8, assign83530_e126866_d_n9, assign83530_e126866_d_n10, assign83530_e126866_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1929 != 0.0)) && (locals.var_guard1930 != 0.0)) {
        let assign83530_e126861: f64 = (locals.var_ddriftldc * 0.1);
        let assign83530_e126862: f64 = (locals.var_ddriftldc - assign83530_e126861);
        let assign83530_e126864: f64 = (assign83530_e126862 + locals.var_tmf0);
        (assign83530_e126864, ((locals.var_ddriftldc_dn0 - (locals.var_ddriftldc_dn0 * 0.1)) + locals.var_tmf0_dn0), ((locals.var_ddriftldc_dn2 - (locals.var_ddriftldc_dn2 * 0.1)) + locals.var_tmf0_dn2), ((locals.var_ddriftldc_dn4 - (locals.var_ddriftldc_dn4 * 0.1)) + locals.var_tmf0_dn4), ((locals.var_ddriftldc_dn5 - (locals.var_ddriftldc_dn5 * 0.1)) + locals.var_tmf0_dn5), ((locals.var_ddriftldc_dn6 - (locals.var_ddriftldc_dn6 * 0.1)) + locals.var_tmf0_dn6), ((locals.var_ddriftldc_dn7 - (locals.var_ddriftldc_dn7 * 0.1)) + locals.var_tmf0_dn7), ((locals.var_ddriftldc_dn8 - (locals.var_ddriftldc_dn8 * 0.1)) + locals.var_tmf0_dn8), ((locals.var_ddriftldc_dn9 - (locals.var_ddriftldc_dn9 * 0.1)) + locals.var_tmf0_dn9), ((locals.var_ddriftldc_dn10 - (locals.var_ddriftldc_dn10 * 0.1)) + locals.var_tmf0_dn10), ((locals.var_ddriftldc_dn13 - (locals.var_ddriftldc_dn13 * 0.1)) + locals.var_tmf0_dn13),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign83530_e126866;
        locals.var_t1_dn0 = assign83530_e126866_d_n0;
        locals.var_t1_dn2 = assign83530_e126866_d_n2;
        locals.var_t1_dn4 = assign83530_e126866_d_n4;
        locals.var_t1_dn5 = assign83530_e126866_d_n5;
        locals.var_t1_dn6 = assign83530_e126866_d_n6;
        locals.var_t1_dn7 = assign83530_e126866_d_n7;
        locals.var_t1_dn8 = assign83530_e126866_d_n8;
        locals.var_t1_dn9 = assign83530_e126866_d_n9;
        locals.var_t1_dn10 = assign83530_e126866_d_n10;
        locals.var_t1_dn13 = assign83530_e126866_d_n13;

        let (assign83540_e126877, assign83540_e126877_d_n0, assign83540_e126877_d_n2, assign83540_e126877_d_n4, assign83540_e126877_d_n5, assign83540_e126877_d_n6, assign83540_e126877_d_n7, assign83540_e126877_d_n8, assign83540_e126877_d_n9, assign83540_e126877_d_n10, assign83540_e126877_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1929 != 0.0)) && (locals.var_guard1930 != 0.0)) {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign83540_e126877;
        locals.var_t0_dn0 = assign83540_e126877_d_n0;
        locals.var_t0_dn2 = assign83540_e126877_d_n2;
        locals.var_t0_dn4 = assign83540_e126877_d_n4;
        locals.var_t0_dn5 = assign83540_e126877_d_n5;
        locals.var_t0_dn6 = assign83540_e126877_d_n6;
        locals.var_t0_dn7 = assign83540_e126877_d_n7;
        locals.var_t0_dn8 = assign83540_e126877_d_n8;
        locals.var_t0_dn9 = assign83540_e126877_d_n9;
        locals.var_t0_dn10 = assign83540_e126877_d_n10;
        locals.var_t0_dn13 = assign83540_e126877_d_n13;

        let (assign83550_e126889, assign83550_e126889_d_n0, assign83550_e126889_d_n2, assign83550_e126889_d_n4, assign83550_e126889_d_n5, assign83550_e126889_d_n6, assign83550_e126889_d_n7, assign83550_e126889_d_n8, assign83550_e126889_d_n9, assign83550_e126889_d_n10, assign83550_e126889_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1929 != 0.0)) && (locals.var_guard1930 == 0.0)) {
        (locals.var_wdld0__blk1927, locals.var_wdld0__blk1927_dn0, locals.var_wdld0__blk1927_dn2, locals.var_wdld0__blk1927_dn4, locals.var_wdld0__blk1927_dn5, locals.var_wdld0__blk1927_dn6, locals.var_wdld0__blk1927_dn7, locals.var_wdld0__blk1927_dn8, locals.var_wdld0__blk1927_dn9, locals.var_wdld0__blk1927_dn10, locals.var_wdld0__blk1927_dn13,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign83550_e126889;
        locals.var_t1_dn0 = assign83550_e126889_d_n0;
        locals.var_t1_dn2 = assign83550_e126889_d_n2;
        locals.var_t1_dn4 = assign83550_e126889_d_n4;
        locals.var_t1_dn5 = assign83550_e126889_d_n5;
        locals.var_t1_dn6 = assign83550_e126889_d_n6;
        locals.var_t1_dn7 = assign83550_e126889_d_n7;
        locals.var_t1_dn8 = assign83550_e126889_d_n8;
        locals.var_t1_dn9 = assign83550_e126889_d_n9;
        locals.var_t1_dn10 = assign83550_e126889_d_n10;
        locals.var_t1_dn13 = assign83550_e126889_d_n13;

        let (assign83560_e126901, assign83560_e126901_d_n0, assign83560_e126901_d_n2, assign83560_e126901_d_n4, assign83560_e126901_d_n5, assign83560_e126901_d_n6, assign83560_e126901_d_n7, assign83560_e126901_d_n8, assign83560_e126901_d_n9, assign83560_e126901_d_n10, assign83560_e126901_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1929 != 0.0)) && (locals.var_guard1930 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign83560_e126901;
        locals.var_t0_dn0 = assign83560_e126901_d_n0;
        locals.var_t0_dn2 = assign83560_e126901_d_n2;
        locals.var_t0_dn4 = assign83560_e126901_d_n4;
        locals.var_t0_dn5 = assign83560_e126901_d_n5;
        locals.var_t0_dn6 = assign83560_e126901_d_n6;
        locals.var_t0_dn7 = assign83560_e126901_d_n7;
        locals.var_t0_dn8 = assign83560_e126901_d_n8;
        locals.var_t0_dn9 = assign83560_e126901_d_n9;
        locals.var_t0_dn10 = assign83560_e126901_d_n10;
        locals.var_t0_dn13 = assign83560_e126901_d_n13;

        let assign83570_e126904: f64 = if locals.var_t0 < 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1936 = assign83570_e126904;

        let (assign83580_e126917,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1929 != 0.0)) && (locals.var_guard1936 != 0.0)) {
        let assign83580_e126915: f64 = (locals.var_flg_fd_mode__blk1889 + 2.0);
        (assign83580_e126915,)
    } else {
        (locals.var_flg_fd_mode__blk1889,)
    }
};
        locals.var_flg_fd_mode__blk1889 = assign83580_e126917;

        let (assign83590_e126932, assign83590_e126932_d_n0, assign83590_e126932_d_n2, assign83590_e126932_d_n4, assign83590_e126932_d_n5, assign83590_e126932_d_n6, assign83590_e126932_d_n7, assign83590_e126932_d_n8, assign83590_e126932_d_n9, assign83590_e126932_d_n10, assign83590_e126932_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1929 == 0.0)) {
        let (assign83590_e126930, assign83590_e126930_d_n0, assign83590_e126930_d_n2, assign83590_e126930_d_n4, assign83590_e126930_d_n5, assign83590_e126930_d_n6, assign83590_e126930_d_n7, assign83590_e126930_d_n8, assign83590_e126930_d_n9, assign83590_e126930_d_n10, assign83590_e126930_d_n13,) = {
            if (locals.var_wdld0__blk1927 <= locals.var_ddriftldc) {
                (locals.var_wdld0__blk1927, locals.var_wdld0__blk1927_dn0, locals.var_wdld0__blk1927_dn2, locals.var_wdld0__blk1927_dn4, locals.var_wdld0__blk1927_dn5, locals.var_wdld0__blk1927_dn6, locals.var_wdld0__blk1927_dn7, locals.var_wdld0__blk1927_dn8, locals.var_wdld0__blk1927_dn9, locals.var_wdld0__blk1927_dn10, locals.var_wdld0__blk1927_dn13,)
            } else {
                (locals.var_ddriftldc, locals.var_ddriftldc_dn0, locals.var_ddriftldc_dn2, locals.var_ddriftldc_dn4, locals.var_ddriftldc_dn5, locals.var_ddriftldc_dn6, locals.var_ddriftldc_dn7, locals.var_ddriftldc_dn8, locals.var_ddriftldc_dn9, locals.var_ddriftldc_dn10, locals.var_ddriftldc_dn13,)
            }
        };
        (assign83590_e126930, assign83590_e126930_d_n0, assign83590_e126930_d_n2, assign83590_e126930_d_n4, assign83590_e126930_d_n5, assign83590_e126930_d_n6, assign83590_e126930_d_n7, assign83590_e126930_d_n8, assign83590_e126930_d_n9, assign83590_e126930_d_n10, assign83590_e126930_d_n13,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign83590_e126932;
        locals.var_t1_dn0 = assign83590_e126932_d_n0;
        locals.var_t1_dn2 = assign83590_e126932_d_n2;
        locals.var_t1_dn4 = assign83590_e126932_d_n4;
        locals.var_t1_dn5 = assign83590_e126932_d_n5;
        locals.var_t1_dn6 = assign83590_e126932_d_n6;
        locals.var_t1_dn7 = assign83590_e126932_d_n7;
        locals.var_t1_dn8 = assign83590_e126932_d_n8;
        locals.var_t1_dn9 = assign83590_e126932_d_n9;
        locals.var_t1_dn10 = assign83590_e126932_d_n10;
        locals.var_t1_dn13 = assign83590_e126932_d_n13;

        let assign83600_e126935: f64 = if locals.var_wdld0__blk1927 >= locals.var_ddriftldc { 1.0 } else { 0.0 };
        locals.var_guard1937 = assign83600_e126935;

        let (assign83610_e126949,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1929 == 0.0)) && (locals.var_guard1937 != 0.0)) {
        let assign83610_e126947: f64 = (locals.var_flg_fd_mode__blk1889 + 2.0);
        (assign83610_e126947,)
    } else {
        (locals.var_flg_fd_mode__blk1889,)
    }
};
        locals.var_flg_fd_mode__blk1889 = assign83610_e126949;

        let assign83620_e126952: f64 = if locals.var_flg_fd_mode__blk1889 >= 2.0 { 1.0 } else { 0.0 };
        locals.var_guard1938 = assign83620_e126952;

        let (assign83630_e126961, assign83630_e126961_d_n0, assign83630_e126961_d_n2, assign83630_e126961_d_n4, assign83630_e126961_d_n5, assign83630_e126961_d_n6, assign83630_e126961_d_n7, assign83630_e126961_d_n8, assign83630_e126961_d_n9, assign83630_e126961_d_n10, assign83630_e126961_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1938 != 0.0)) {
        (locals.var_ps0ld, locals.var_ps0ld_dn0, locals.var_ps0ld_dn2, locals.var_ps0ld_dn4, locals.var_ps0ld_dn5, locals.var_ps0ld_dn6, locals.var_ps0ld_dn7, locals.var_ps0ld_dn8, locals.var_ps0ld_dn9, locals.var_ps0ld_dn10, locals.var_ps0ld_dn13,)
    } else {
        (locals.var_ps0ld_bef1__blk1928, locals.var_ps0ld_bef1__blk1928_dn0, locals.var_ps0ld_bef1__blk1928_dn2, locals.var_ps0ld_bef1__blk1928_dn4, locals.var_ps0ld_bef1__blk1928_dn5, locals.var_ps0ld_bef1__blk1928_dn6, locals.var_ps0ld_bef1__blk1928_dn7, locals.var_ps0ld_bef1__blk1928_dn8, locals.var_ps0ld_bef1__blk1928_dn9, locals.var_ps0ld_bef1__blk1928_dn10, locals.var_ps0ld_bef1__blk1928_dn13,)
    }
};
        locals.var_ps0ld_bef1__blk1928 = assign83630_e126961;
        locals.var_ps0ld_bef1__blk1928_dn0 = assign83630_e126961_d_n0;
        locals.var_ps0ld_bef1__blk1928_dn2 = assign83630_e126961_d_n2;
        locals.var_ps0ld_bef1__blk1928_dn4 = assign83630_e126961_d_n4;
        locals.var_ps0ld_bef1__blk1928_dn5 = assign83630_e126961_d_n5;
        locals.var_ps0ld_bef1__blk1928_dn6 = assign83630_e126961_d_n6;
        locals.var_ps0ld_bef1__blk1928_dn7 = assign83630_e126961_d_n7;
        locals.var_ps0ld_bef1__blk1928_dn8 = assign83630_e126961_d_n8;
        locals.var_ps0ld_bef1__blk1928_dn9 = assign83630_e126961_d_n9;
        locals.var_ps0ld_bef1__blk1928_dn10 = assign83630_e126961_d_n10;
        locals.var_ps0ld_bef1__blk1928_dn13 = assign83630_e126961_d_n13;

    }

    pub(super) fn stamp_transient_block_291(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign83640_e126972, assign83640_e126972_d_n0, assign83640_e126972_d_n2, assign83640_e126972_d_n4, assign83640_e126972_d_n5, assign83640_e126972_d_n6, assign83640_e126972_d_n7, assign83640_e126972_d_n8, assign83640_e126972_d_n9, assign83640_e126972_d_n10, assign83640_e126972_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1938 != 0.0)) {
        let assign83640_e126970: f64 = (locals.var_t1 * locals.var_q_nsubld__blk1883);
        (assign83640_e126970, (locals.var_t1_dn0 * locals.var_q_nsubld__blk1883), (locals.var_t1_dn2 * locals.var_q_nsubld__blk1883), (locals.var_t1_dn4 * locals.var_q_nsubld__blk1883), (locals.var_t1_dn5 * locals.var_q_nsubld__blk1883), (locals.var_t1_dn6 * locals.var_q_nsubld__blk1883), (locals.var_t1_dn7 * locals.var_q_nsubld__blk1883), (locals.var_t1_dn8 * locals.var_q_nsubld__blk1883), (locals.var_t1_dn9 * locals.var_q_nsubld__blk1883), (locals.var_t1_dn10 * locals.var_q_nsubld__blk1883), (locals.var_t1_dn13 * locals.var_q_nsubld__blk1883),)
    } else {
        (locals.var_qbuld, locals.var_qbuld_dn0, locals.var_qbuld_dn2, locals.var_qbuld_dn4, locals.var_qbuld_dn5, locals.var_qbuld_dn6, locals.var_qbuld_dn7, locals.var_qbuld_dn8, locals.var_qbuld_dn9, locals.var_qbuld_dn10, locals.var_qbuld_dn13,)
    }
};
        locals.var_qbuld = assign83640_e126972;
        locals.var_qbuld_dn0 = assign83640_e126972_d_n0;
        locals.var_qbuld_dn2 = assign83640_e126972_d_n2;
        locals.var_qbuld_dn4 = assign83640_e126972_d_n4;
        locals.var_qbuld_dn5 = assign83640_e126972_d_n5;
        locals.var_qbuld_dn6 = assign83640_e126972_d_n6;
        locals.var_qbuld_dn7 = assign83640_e126972_d_n7;
        locals.var_qbuld_dn8 = assign83640_e126972_d_n8;
        locals.var_qbuld_dn9 = assign83640_e126972_d_n9;
        locals.var_qbuld_dn10 = assign83640_e126972_d_n10;
        locals.var_qbuld_dn13 = assign83640_e126972_d_n13;

        let (assign83650_e126985, assign83650_e126985_d_n0, assign83650_e126985_d_n2, assign83650_e126985_d_n4, assign83650_e126985_d_n5, assign83650_e126985_d_n6, assign83650_e126985_d_n7, assign83650_e126985_d_n8, assign83650_e126985_d_n9, assign83650_e126985_d_n10, assign83650_e126985_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1938 != 0.0)) {
        let assign83650_e126982: f64 = (locals.var_qbuld / locals.var_cox0_func);
        let assign83650_e126983: f64 = (locals.var_vgpld - assign83650_e126982);
        (assign83650_e126983, (-(locals.var_qbuld_dn0 / locals.var_cox0_func)), (locals.var_vgpld_dn2 - (locals.var_qbuld_dn2 / locals.var_cox0_func)), (-(locals.var_qbuld_dn4 / locals.var_cox0_func)), (-(locals.var_qbuld_dn5 / locals.var_cox0_func)), (locals.var_vgpld_dn6 - (locals.var_qbuld_dn6 / locals.var_cox0_func)), (locals.var_vgpld_dn7 - (locals.var_qbuld_dn7 / locals.var_cox0_func)), (locals.var_vgpld_dn8 - (locals.var_qbuld_dn8 / locals.var_cox0_func)), (-(locals.var_qbuld_dn9 / locals.var_cox0_func)), (-(locals.var_qbuld_dn10 / locals.var_cox0_func)), (-(locals.var_qbuld_dn13 / locals.var_cox0_func)),)
    } else {
        (locals.var_ps0ld, locals.var_ps0ld_dn0, locals.var_ps0ld_dn2, locals.var_ps0ld_dn4, locals.var_ps0ld_dn5, locals.var_ps0ld_dn6, locals.var_ps0ld_dn7, locals.var_ps0ld_dn8, locals.var_ps0ld_dn9, locals.var_ps0ld_dn10, locals.var_ps0ld_dn13,)
    }
};
        locals.var_ps0ld = assign83650_e126985;
        locals.var_ps0ld_dn0 = assign83650_e126985_d_n0;
        locals.var_ps0ld_dn2 = assign83650_e126985_d_n2;
        locals.var_ps0ld_dn4 = assign83650_e126985_d_n4;
        locals.var_ps0ld_dn5 = assign83650_e126985_d_n5;
        locals.var_ps0ld_dn6 = assign83650_e126985_d_n6;
        locals.var_ps0ld_dn7 = assign83650_e126985_d_n7;
        locals.var_ps0ld_dn8 = assign83650_e126985_d_n8;
        locals.var_ps0ld_dn9 = assign83650_e126985_d_n9;
        locals.var_ps0ld_dn10 = assign83650_e126985_d_n10;
        locals.var_ps0ld_dn13 = assign83650_e126985_d_n13;

        let assign83660_e126988: f64 = if p.p33 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard1939 = assign83660_e126988;

        let assign83670_e126992: f64 = (locals.var_ps0ld_bef1__blk1928 - 0.1);
        let assign83670_e126997: f64 = if ((locals.var_ps0ld > assign83670_e126992) && (0.1 >= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1940 = assign83670_e126997;

        let (assign83680_e127014, assign83680_e127014_d_n0, assign83680_e127014_d_n2, assign83680_e127014_d_n4, assign83680_e127014_d_n5, assign83680_e127014_d_n6, assign83680_e127014_d_n7, assign83680_e127014_d_n8, assign83680_e127014_d_n9, assign83680_e127014_d_n10, assign83680_e127014_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1938 != 0.0)) && (locals.var_guard1939 != 0.0)) && (locals.var_guard1940 != 0.0)) {
        let assign83680_e127010: f64 = (locals.var_ps0ld - locals.var_ps0ld_bef1__blk1928);
        let assign83680_e127012: f64 = (assign83680_e127010 + 0.1);
        (assign83680_e127012, (locals.var_ps0ld_dn0 - locals.var_ps0ld_bef1__blk1928_dn0), (locals.var_ps0ld_dn2 - locals.var_ps0ld_bef1__blk1928_dn2), (locals.var_ps0ld_dn4 - locals.var_ps0ld_bef1__blk1928_dn4), (locals.var_ps0ld_dn5 - locals.var_ps0ld_bef1__blk1928_dn5), (locals.var_ps0ld_dn6 - locals.var_ps0ld_bef1__blk1928_dn6), (locals.var_ps0ld_dn7 - locals.var_ps0ld_bef1__blk1928_dn7), (locals.var_ps0ld_dn8 - locals.var_ps0ld_bef1__blk1928_dn8), (locals.var_ps0ld_dn9 - locals.var_ps0ld_bef1__blk1928_dn9), (locals.var_ps0ld_dn10 - locals.var_ps0ld_bef1__blk1928_dn10), (locals.var_ps0ld_dn13 - locals.var_ps0ld_bef1__blk1928_dn13),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13,)
    }
};
        locals.var_tmf1 = assign83680_e127014;
        locals.var_tmf1_dn0 = assign83680_e127014_d_n0;
        locals.var_tmf1_dn2 = assign83680_e127014_d_n2;
        locals.var_tmf1_dn4 = assign83680_e127014_d_n4;
        locals.var_tmf1_dn5 = assign83680_e127014_d_n5;
        locals.var_tmf1_dn6 = assign83680_e127014_d_n6;
        locals.var_tmf1_dn7 = assign83680_e127014_d_n7;
        locals.var_tmf1_dn8 = assign83680_e127014_d_n8;
        locals.var_tmf1_dn9 = assign83680_e127014_d_n9;
        locals.var_tmf1_dn10 = assign83680_e127014_d_n10;
        locals.var_tmf1_dn13 = assign83680_e127014_d_n13;

        let (assign83690_e127029, assign83690_e127029_d_n0, assign83690_e127029_d_n2, assign83690_e127029_d_n4, assign83690_e127029_d_n5, assign83690_e127029_d_n6, assign83690_e127029_d_n7, assign83690_e127029_d_n8, assign83690_e127029_d_n9, assign83690_e127029_d_n10, assign83690_e127029_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1938 != 0.0)) && (locals.var_guard1939 != 0.0)) && (locals.var_guard1940 != 0.0)) {
        let assign83690_e127027: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign83690_e127027, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)), ((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)), ((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn13 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn13)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn4, locals.var_x2_dn5, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn8, locals.var_x2_dn9, locals.var_x2_dn10, locals.var_x2_dn13,)
    }
};
        locals.var_x2 = assign83690_e127029;
        locals.var_x2_dn0 = assign83690_e127029_d_n0;
        locals.var_x2_dn2 = assign83690_e127029_d_n2;
        locals.var_x2_dn4 = assign83690_e127029_d_n4;
        locals.var_x2_dn5 = assign83690_e127029_d_n5;
        locals.var_x2_dn6 = assign83690_e127029_d_n6;
        locals.var_x2_dn7 = assign83690_e127029_d_n7;
        locals.var_x2_dn8 = assign83690_e127029_d_n8;
        locals.var_x2_dn9 = assign83690_e127029_d_n9;
        locals.var_x2_dn10 = assign83690_e127029_d_n10;
        locals.var_x2_dn13 = assign83690_e127029_d_n13;

        let (assign83700_e127044, assign83700_e127044_d_n0, assign83700_e127044_d_n2, assign83700_e127044_d_n4, assign83700_e127044_d_n5, assign83700_e127044_d_n6, assign83700_e127044_d_n7, assign83700_e127044_d_n8, assign83700_e127044_d_n9, assign83700_e127044_d_n10, assign83700_e127044_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1938 != 0.0)) && (locals.var_guard1939 != 0.0)) && (locals.var_guard1940 != 0.0)) {
        let assign83700_e127042: f64 = (0.1 * 0.1);
        (assign83700_e127042, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn4, locals.var_xmax2_dn5, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn8, locals.var_xmax2_dn9, locals.var_xmax2_dn10, locals.var_xmax2_dn13,)
    }
};
        locals.var_xmax2 = assign83700_e127044;
        locals.var_xmax2_dn0 = assign83700_e127044_d_n0;
        locals.var_xmax2_dn2 = assign83700_e127044_d_n2;
        locals.var_xmax2_dn4 = assign83700_e127044_d_n4;
        locals.var_xmax2_dn5 = assign83700_e127044_d_n5;
        locals.var_xmax2_dn6 = assign83700_e127044_d_n6;
        locals.var_xmax2_dn7 = assign83700_e127044_d_n7;
        locals.var_xmax2_dn8 = assign83700_e127044_d_n8;
        locals.var_xmax2_dn9 = assign83700_e127044_d_n9;
        locals.var_xmax2_dn10 = assign83700_e127044_d_n10;
        locals.var_xmax2_dn13 = assign83700_e127044_d_n13;

        let (assign83710_e127057, assign83710_e127057_d_n0, assign83710_e127057_d_n2, assign83710_e127057_d_n4, assign83710_e127057_d_n5, assign83710_e127057_d_n6, assign83710_e127057_d_n7, assign83710_e127057_d_n8, assign83710_e127057_d_n9, assign83710_e127057_d_n10, assign83710_e127057_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1938 != 0.0)) && (locals.var_guard1939 != 0.0)) && (locals.var_guard1940 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
        locals.var_xp = assign83710_e127057;
        locals.var_xp_dn0 = assign83710_e127057_d_n0;
        locals.var_xp_dn2 = assign83710_e127057_d_n2;
        locals.var_xp_dn4 = assign83710_e127057_d_n4;
        locals.var_xp_dn5 = assign83710_e127057_d_n5;
        locals.var_xp_dn6 = assign83710_e127057_d_n6;
        locals.var_xp_dn7 = assign83710_e127057_d_n7;
        locals.var_xp_dn8 = assign83710_e127057_d_n8;
        locals.var_xp_dn9 = assign83710_e127057_d_n9;
        locals.var_xp_dn10 = assign83710_e127057_d_n10;
        locals.var_xp_dn13 = assign83710_e127057_d_n13;

        let (assign83720_e127070, assign83720_e127070_d_n0, assign83720_e127070_d_n2, assign83720_e127070_d_n4, assign83720_e127070_d_n5, assign83720_e127070_d_n6, assign83720_e127070_d_n7, assign83720_e127070_d_n8, assign83720_e127070_d_n9, assign83720_e127070_d_n10, assign83720_e127070_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1938 != 0.0)) && (locals.var_guard1939 != 0.0)) && (locals.var_guard1940 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
        locals.var_xmp = assign83720_e127070;
        locals.var_xmp_dn0 = assign83720_e127070_d_n0;
        locals.var_xmp_dn2 = assign83720_e127070_d_n2;
        locals.var_xmp_dn4 = assign83720_e127070_d_n4;
        locals.var_xmp_dn5 = assign83720_e127070_d_n5;
        locals.var_xmp_dn6 = assign83720_e127070_d_n6;
        locals.var_xmp_dn7 = assign83720_e127070_d_n7;
        locals.var_xmp_dn8 = assign83720_e127070_d_n8;
        locals.var_xmp_dn9 = assign83720_e127070_d_n9;
        locals.var_xmp_dn10 = assign83720_e127070_d_n10;
        locals.var_xmp_dn13 = assign83720_e127070_d_n13;

        let (assign83730_e127083,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1938 != 0.0)) && (locals.var_guard1939 != 0.0)) && (locals.var_guard1940 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign83730_e127083;

        let (assign83740_e127096,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1938 != 0.0)) && (locals.var_guard1939 != 0.0)) && (locals.var_guard1940 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign83740_e127096;

        let (assign83750_e127109, assign83750_e127109_d_n0, assign83750_e127109_d_n2, assign83750_e127109_d_n4, assign83750_e127109_d_n5, assign83750_e127109_d_n6, assign83750_e127109_d_n7, assign83750_e127109_d_n8, assign83750_e127109_d_n9, assign83750_e127109_d_n10, assign83750_e127109_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1938 != 0.0)) && (locals.var_guard1939 != 0.0)) && (locals.var_guard1940 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    }
};
        locals.var_arg = assign83750_e127109;
        locals.var_arg_dn0 = assign83750_e127109_d_n0;
        locals.var_arg_dn2 = assign83750_e127109_d_n2;
        locals.var_arg_dn4 = assign83750_e127109_d_n4;
        locals.var_arg_dn5 = assign83750_e127109_d_n5;
        locals.var_arg_dn6 = assign83750_e127109_d_n6;
        locals.var_arg_dn7 = assign83750_e127109_d_n7;
        locals.var_arg_dn8 = assign83750_e127109_d_n8;
        locals.var_arg_dn9 = assign83750_e127109_d_n9;
        locals.var_arg_dn10 = assign83750_e127109_d_n10;
        locals.var_arg_dn13 = assign83750_e127109_d_n13;

        let (assign83760_e127122, assign83760_e127122_d_n0, assign83760_e127122_d_n2, assign83760_e127122_d_n4, assign83760_e127122_d_n5, assign83760_e127122_d_n6, assign83760_e127122_d_n7, assign83760_e127122_d_n8, assign83760_e127122_d_n9, assign83760_e127122_d_n10, assign83760_e127122_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1938 != 0.0)) && (locals.var_guard1939 != 0.0)) && (locals.var_guard1940 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign83760_e127122;
        locals.var_dnm_dn0 = assign83760_e127122_d_n0;
        locals.var_dnm_dn2 = assign83760_e127122_d_n2;
        locals.var_dnm_dn4 = assign83760_e127122_d_n4;
        locals.var_dnm_dn5 = assign83760_e127122_d_n5;
        locals.var_dnm_dn6 = assign83760_e127122_d_n6;
        locals.var_dnm_dn7 = assign83760_e127122_d_n7;
        locals.var_dnm_dn8 = assign83760_e127122_d_n8;
        locals.var_dnm_dn9 = assign83760_e127122_d_n9;
        locals.var_dnm_dn10 = assign83760_e127122_d_n10;
        locals.var_dnm_dn13 = assign83760_e127122_d_n13;

        let (assign83770_e127137, assign83770_e127137_d_n0, assign83770_e127137_d_n2, assign83770_e127137_d_n4, assign83770_e127137_d_n5, assign83770_e127137_d_n6, assign83770_e127137_d_n7, assign83770_e127137_d_n8, assign83770_e127137_d_n9, assign83770_e127137_d_n10, assign83770_e127137_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1938 != 0.0)) && (locals.var_guard1939 != 0.0)) && (locals.var_guard1940 != 0.0)) {
        let assign83770_e127135: f64 = (locals.var_xp * locals.var_x2);
        (assign83770_e127135, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn13 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn13)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
        locals.var_xp = assign83770_e127137;
        locals.var_xp_dn0 = assign83770_e127137_d_n0;
        locals.var_xp_dn2 = assign83770_e127137_d_n2;
        locals.var_xp_dn4 = assign83770_e127137_d_n4;
        locals.var_xp_dn5 = assign83770_e127137_d_n5;
        locals.var_xp_dn6 = assign83770_e127137_d_n6;
        locals.var_xp_dn7 = assign83770_e127137_d_n7;
        locals.var_xp_dn8 = assign83770_e127137_d_n8;
        locals.var_xp_dn9 = assign83770_e127137_d_n9;
        locals.var_xp_dn10 = assign83770_e127137_d_n10;
        locals.var_xp_dn13 = assign83770_e127137_d_n13;

        let (assign83780_e127152, assign83780_e127152_d_n0, assign83780_e127152_d_n2, assign83780_e127152_d_n4, assign83780_e127152_d_n5, assign83780_e127152_d_n6, assign83780_e127152_d_n7, assign83780_e127152_d_n8, assign83780_e127152_d_n9, assign83780_e127152_d_n10, assign83780_e127152_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1938 != 0.0)) && (locals.var_guard1939 != 0.0)) && (locals.var_guard1940 != 0.0)) {
        let assign83780_e127150: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign83780_e127150, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn13 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn13)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
        locals.var_xmp = assign83780_e127152;
        locals.var_xmp_dn0 = assign83780_e127152_d_n0;
        locals.var_xmp_dn2 = assign83780_e127152_d_n2;
        locals.var_xmp_dn4 = assign83780_e127152_d_n4;
        locals.var_xmp_dn5 = assign83780_e127152_d_n5;
        locals.var_xmp_dn6 = assign83780_e127152_d_n6;
        locals.var_xmp_dn7 = assign83780_e127152_d_n7;
        locals.var_xmp_dn8 = assign83780_e127152_d_n8;
        locals.var_xmp_dn9 = assign83780_e127152_d_n9;
        locals.var_xmp_dn10 = assign83780_e127152_d_n10;
        locals.var_xmp_dn13 = assign83780_e127152_d_n13;

        let (assign83790_e127167, assign83790_e127167_d_n0, assign83790_e127167_d_n2, assign83790_e127167_d_n4, assign83790_e127167_d_n5, assign83790_e127167_d_n6, assign83790_e127167_d_n7, assign83790_e127167_d_n8, assign83790_e127167_d_n9, assign83790_e127167_d_n10, assign83790_e127167_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1938 != 0.0)) && (locals.var_guard1939 != 0.0)) && (locals.var_guard1940 != 0.0)) {
        let assign83790_e127165: f64 = (locals.var_xp * locals.var_x2);
        (assign83790_e127165, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn13 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn13)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
        locals.var_xp = assign83790_e127167;
        locals.var_xp_dn0 = assign83790_e127167_d_n0;
        locals.var_xp_dn2 = assign83790_e127167_d_n2;
        locals.var_xp_dn4 = assign83790_e127167_d_n4;
        locals.var_xp_dn5 = assign83790_e127167_d_n5;
        locals.var_xp_dn6 = assign83790_e127167_d_n6;
        locals.var_xp_dn7 = assign83790_e127167_d_n7;
        locals.var_xp_dn8 = assign83790_e127167_d_n8;
        locals.var_xp_dn9 = assign83790_e127167_d_n9;
        locals.var_xp_dn10 = assign83790_e127167_d_n10;
        locals.var_xp_dn13 = assign83790_e127167_d_n13;

        let (assign83800_e127182, assign83800_e127182_d_n0, assign83800_e127182_d_n2, assign83800_e127182_d_n4, assign83800_e127182_d_n5, assign83800_e127182_d_n6, assign83800_e127182_d_n7, assign83800_e127182_d_n8, assign83800_e127182_d_n9, assign83800_e127182_d_n10, assign83800_e127182_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1938 != 0.0)) && (locals.var_guard1939 != 0.0)) && (locals.var_guard1940 != 0.0)) {
        let assign83800_e127180: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign83800_e127180, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn13 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn13)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
        locals.var_xmp = assign83800_e127182;
        locals.var_xmp_dn0 = assign83800_e127182_d_n0;
        locals.var_xmp_dn2 = assign83800_e127182_d_n2;
        locals.var_xmp_dn4 = assign83800_e127182_d_n4;
        locals.var_xmp_dn5 = assign83800_e127182_d_n5;
        locals.var_xmp_dn6 = assign83800_e127182_d_n6;
        locals.var_xmp_dn7 = assign83800_e127182_d_n7;
        locals.var_xmp_dn8 = assign83800_e127182_d_n8;
        locals.var_xmp_dn9 = assign83800_e127182_d_n9;
        locals.var_xmp_dn10 = assign83800_e127182_d_n10;
        locals.var_xmp_dn13 = assign83800_e127182_d_n13;

        let (assign83810_e127197, assign83810_e127197_d_n0, assign83810_e127197_d_n2, assign83810_e127197_d_n4, assign83810_e127197_d_n5, assign83810_e127197_d_n6, assign83810_e127197_d_n7, assign83810_e127197_d_n8, assign83810_e127197_d_n9, assign83810_e127197_d_n10, assign83810_e127197_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1938 != 0.0)) && (locals.var_guard1939 != 0.0)) && (locals.var_guard1940 != 0.0)) {
        let assign83810_e127195: f64 = (locals.var_xp + locals.var_xmp);
        (assign83810_e127195, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn4 + locals.var_xmp_dn4), (locals.var_xp_dn5 + locals.var_xmp_dn5), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn8 + locals.var_xmp_dn8), (locals.var_xp_dn9 + locals.var_xmp_dn9), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn13 + locals.var_xmp_dn13),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    }
};
        locals.var_arg = assign83810_e127197;
        locals.var_arg_dn0 = assign83810_e127197_d_n0;
        locals.var_arg_dn2 = assign83810_e127197_d_n2;
        locals.var_arg_dn4 = assign83810_e127197_d_n4;
        locals.var_arg_dn5 = assign83810_e127197_d_n5;
        locals.var_arg_dn6 = assign83810_e127197_d_n6;
        locals.var_arg_dn7 = assign83810_e127197_d_n7;
        locals.var_arg_dn8 = assign83810_e127197_d_n8;
        locals.var_arg_dn9 = assign83810_e127197_d_n9;
        locals.var_arg_dn10 = assign83810_e127197_d_n10;
        locals.var_arg_dn13 = assign83810_e127197_d_n13;

        let (assign83820_e127210, assign83820_e127210_d_n0, assign83820_e127210_d_n2, assign83820_e127210_d_n4, assign83820_e127210_d_n5, assign83820_e127210_d_n6, assign83820_e127210_d_n7, assign83820_e127210_d_n8, assign83820_e127210_d_n9, assign83820_e127210_d_n10, assign83820_e127210_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1938 != 0.0)) && (locals.var_guard1939 != 0.0)) && (locals.var_guard1940 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign83820_e127210;
        locals.var_dnm_dn0 = assign83820_e127210_d_n0;
        locals.var_dnm_dn2 = assign83820_e127210_d_n2;
        locals.var_dnm_dn4 = assign83820_e127210_d_n4;
        locals.var_dnm_dn5 = assign83820_e127210_d_n5;
        locals.var_dnm_dn6 = assign83820_e127210_d_n6;
        locals.var_dnm_dn7 = assign83820_e127210_d_n7;
        locals.var_dnm_dn8 = assign83820_e127210_d_n8;
        locals.var_dnm_dn9 = assign83820_e127210_d_n9;
        locals.var_dnm_dn10 = assign83820_e127210_d_n10;
        locals.var_dnm_dn13 = assign83820_e127210_d_n13;

        let assign83830_e127225: f64 = if ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard1941 = assign83830_e127225;

        let assign83840_e127228: f64 = if 2.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1942 = assign83840_e127228;

        let (assign83850_e127245,) = {
    if (((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1938 != 0.0)) && (locals.var_guard1939 != 0.0)) && (locals.var_guard1940 != 0.0)) && (locals.var_guard1941 != 0.0)) && (locals.var_guard1942 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign83850_e127245;

        let assign83860_e127248: f64 = if 2.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard1943 = assign83860_e127248;

        let (assign83870_e127268,) = {
    if ((((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1938 != 0.0)) && (locals.var_guard1939 != 0.0)) && (locals.var_guard1940 != 0.0)) && (locals.var_guard1941 != 0.0)) && (locals.var_guard1942 == 0.0)) && (locals.var_guard1943 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign83870_e127268;

        let assign83880_e127271: f64 = if 2.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard1944 = assign83880_e127271;

        let (assign83890_e127294,) = {
    if (((((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1938 != 0.0)) && (locals.var_guard1939 != 0.0)) && (locals.var_guard1940 != 0.0)) && (locals.var_guard1941 != 0.0)) && (locals.var_guard1942 == 0.0)) && (locals.var_guard1943 == 0.0)) && (locals.var_guard1944 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign83890_e127294;

        let assign83900_e127297: f64 = if 2.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard1945 = assign83900_e127297;

        let (assign83910_e127323,) = {
    if ((((((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1938 != 0.0)) && (locals.var_guard1939 != 0.0)) && (locals.var_guard1940 != 0.0)) && (locals.var_guard1941 != 0.0)) && (locals.var_guard1942 == 0.0)) && (locals.var_guard1943 == 0.0)) && (locals.var_guard1944 == 0.0)) && (locals.var_guard1945 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign83910_e127323;

        let (assign83920_e127338,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1938 != 0.0)) && (locals.var_guard1939 != 0.0)) && (locals.var_guard1940 != 0.0)) && (locals.var_guard1941 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign83920_e127338;

        let mut assign83930_loop_guard: usize = 0;
        while {
            let assign83930_cond_e127354: f64 = if (((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1938 != 0.0)) && (locals.var_guard1939 != 0.0)) && (locals.var_guard1940 != 0.0)) && (locals.var_guard1941 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign83930_cond_e127354 != 0.0
        } {
            assign83930_loop_guard += 1;
            assert!(assign83930_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign83930_body0_e127370, assign83930_body0_e127370_d_n0, assign83930_body0_e127370_d_n2, assign83930_body0_e127370_d_n4, assign83930_body0_e127370_d_n5, assign83930_body0_e127370_d_n6, assign83930_body0_e127370_d_n7, assign83930_body0_e127370_d_n8, assign83930_body0_e127370_d_n9, assign83930_body0_e127370_d_n10, assign83930_body0_e127370_d_n13,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1938 != 0.0)) && (locals.var_guard1939 != 0.0)) && (locals.var_guard1940 != 0.0)) && (locals.var_guard1941 != 0.0)) {
        let assign83930_body0_e127368: f64 = (locals.var_dnm).sqrt();
        (assign83930_body0_e127368, (locals.var_dnm_dn0 / (2.0 * assign83930_body0_e127368)), (locals.var_dnm_dn2 / (2.0 * assign83930_body0_e127368)), (locals.var_dnm_dn4 / (2.0 * assign83930_body0_e127368)), (locals.var_dnm_dn5 / (2.0 * assign83930_body0_e127368)), (locals.var_dnm_dn6 / (2.0 * assign83930_body0_e127368)), (locals.var_dnm_dn7 / (2.0 * assign83930_body0_e127368)), (locals.var_dnm_dn8 / (2.0 * assign83930_body0_e127368)), (locals.var_dnm_dn9 / (2.0 * assign83930_body0_e127368)), (locals.var_dnm_dn10 / (2.0 * assign83930_body0_e127368)), (locals.var_dnm_dn13 / (2.0 * assign83930_body0_e127368)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
            locals.var_dnm = assign83930_body0_e127370;
            locals.var_dnm_dn0 = assign83930_body0_e127370_d_n0;
            locals.var_dnm_dn2 = assign83930_body0_e127370_d_n2;
            locals.var_dnm_dn4 = assign83930_body0_e127370_d_n4;
            locals.var_dnm_dn5 = assign83930_body0_e127370_d_n5;
            locals.var_dnm_dn6 = assign83930_body0_e127370_d_n6;
            locals.var_dnm_dn7 = assign83930_body0_e127370_d_n7;
            locals.var_dnm_dn8 = assign83930_body0_e127370_d_n8;
            locals.var_dnm_dn9 = assign83930_body0_e127370_d_n9;
            locals.var_dnm_dn10 = assign83930_body0_e127370_d_n10;
            locals.var_dnm_dn13 = assign83930_body0_e127370_d_n13;
            let (assign83930_body1_e127387,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1938 != 0.0)) && (locals.var_guard1939 != 0.0)) && (locals.var_guard1940 != 0.0)) && (locals.var_guard1941 != 0.0)) {
        let assign83930_body1_e127385: f64 = (locals.var_m0 + 1.0);
        (assign83930_body1_e127385,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign83930_body1_e127387;
        }

        let (assign83940_e127414, assign83940_e127414_d_n0, assign83940_e127414_d_n2, assign83940_e127414_d_n4, assign83940_e127414_d_n5, assign83940_e127414_d_n6, assign83940_e127414_d_n7, assign83940_e127414_d_n8, assign83940_e127414_d_n9, assign83940_e127414_d_n10, assign83940_e127414_d_n13,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1938 != 0.0)) && (locals.var_guard1939 != 0.0)) && (locals.var_guard1940 != 0.0)) && (locals.var_guard1941 == 0.0)) {
        let (assign83940_e127412, assign83940_e127412_d_n0, assign83940_e127412_d_n2, assign83940_e127412_d_n4, assign83940_e127412_d_n5, assign83940_e127412_d_n6, assign83940_e127412_d_n7, assign83940_e127412_d_n8, assign83940_e127412_d_n9, assign83940_e127412_d_n10, assign83940_e127412_d_n13,) = {
            if (locals.var_dnm == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign83940_e127409: f64 = (2.0 * 2.0);
                let assign83940_e127410: f64 = (1.0 / assign83940_e127409);
                let assign83940_e127411: f64 = (locals.var_dnm).powf(assign83940_e127410);
                (assign83940_e127411, if 0.0 == 0.0 && ((assign83940_e127410) as f64).is_finite() && ((assign83940_e127410) as f64).fract() == 0.0 { if assign83940_e127410 == 0.0 { 0.0 } else { (assign83940_e127410 * ((locals.var_dnm).powf(assign83940_e127410 - 1.0) * locals.var_dnm_dn0)) } } else { (assign83940_e127411 * (assign83940_e127410 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign83940_e127410) as f64).is_finite() && ((assign83940_e127410) as f64).fract() == 0.0 { if assign83940_e127410 == 0.0 { 0.0 } else { (assign83940_e127410 * ((locals.var_dnm).powf(assign83940_e127410 - 1.0) * locals.var_dnm_dn2)) } } else { (assign83940_e127411 * (assign83940_e127410 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign83940_e127410) as f64).is_finite() && ((assign83940_e127410) as f64).fract() == 0.0 { if assign83940_e127410 == 0.0 { 0.0 } else { (assign83940_e127410 * ((locals.var_dnm).powf(assign83940_e127410 - 1.0) * locals.var_dnm_dn4)) } } else { (assign83940_e127411 * (assign83940_e127410 * (locals.var_dnm_dn4 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign83940_e127410) as f64).is_finite() && ((assign83940_e127410) as f64).fract() == 0.0 { if assign83940_e127410 == 0.0 { 0.0 } else { (assign83940_e127410 * ((locals.var_dnm).powf(assign83940_e127410 - 1.0) * locals.var_dnm_dn5)) } } else { (assign83940_e127411 * (assign83940_e127410 * (locals.var_dnm_dn5 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign83940_e127410) as f64).is_finite() && ((assign83940_e127410) as f64).fract() == 0.0 { if assign83940_e127410 == 0.0 { 0.0 } else { (assign83940_e127410 * ((locals.var_dnm).powf(assign83940_e127410 - 1.0) * locals.var_dnm_dn6)) } } else { (assign83940_e127411 * (assign83940_e127410 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign83940_e127410) as f64).is_finite() && ((assign83940_e127410) as f64).fract() == 0.0 { if assign83940_e127410 == 0.0 { 0.0 } else { (assign83940_e127410 * ((locals.var_dnm).powf(assign83940_e127410 - 1.0) * locals.var_dnm_dn7)) } } else { (assign83940_e127411 * (assign83940_e127410 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign83940_e127410) as f64).is_finite() && ((assign83940_e127410) as f64).fract() == 0.0 { if assign83940_e127410 == 0.0 { 0.0 } else { (assign83940_e127410 * ((locals.var_dnm).powf(assign83940_e127410 - 1.0) * locals.var_dnm_dn8)) } } else { (assign83940_e127411 * (assign83940_e127410 * (locals.var_dnm_dn8 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign83940_e127410) as f64).is_finite() && ((assign83940_e127410) as f64).fract() == 0.0 { if assign83940_e127410 == 0.0 { 0.0 } else { (assign83940_e127410 * ((locals.var_dnm).powf(assign83940_e127410 - 1.0) * locals.var_dnm_dn9)) } } else { (assign83940_e127411 * (assign83940_e127410 * (locals.var_dnm_dn9 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign83940_e127410) as f64).is_finite() && ((assign83940_e127410) as f64).fract() == 0.0 { if assign83940_e127410 == 0.0 { 0.0 } else { (assign83940_e127410 * ((locals.var_dnm).powf(assign83940_e127410 - 1.0) * locals.var_dnm_dn10)) } } else { (assign83940_e127411 * (assign83940_e127410 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign83940_e127410) as f64).is_finite() && ((assign83940_e127410) as f64).fract() == 0.0 { if assign83940_e127410 == 0.0 { 0.0 } else { (assign83940_e127410 * ((locals.var_dnm).powf(assign83940_e127410 - 1.0) * locals.var_dnm_dn13)) } } else { (assign83940_e127411 * (assign83940_e127410 * (locals.var_dnm_dn13 / locals.var_dnm))) },)
            }
        };
        (assign83940_e127412, assign83940_e127412_d_n0, assign83940_e127412_d_n2, assign83940_e127412_d_n4, assign83940_e127412_d_n5, assign83940_e127412_d_n6, assign83940_e127412_d_n7, assign83940_e127412_d_n8, assign83940_e127412_d_n9, assign83940_e127412_d_n10, assign83940_e127412_d_n13,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign83940_e127414;
        locals.var_dnm_dn0 = assign83940_e127414_d_n0;
        locals.var_dnm_dn2 = assign83940_e127414_d_n2;
        locals.var_dnm_dn4 = assign83940_e127414_d_n4;
        locals.var_dnm_dn5 = assign83940_e127414_d_n5;
        locals.var_dnm_dn6 = assign83940_e127414_d_n6;
        locals.var_dnm_dn7 = assign83940_e127414_d_n7;
        locals.var_dnm_dn8 = assign83940_e127414_d_n8;
        locals.var_dnm_dn9 = assign83940_e127414_d_n9;
        locals.var_dnm_dn10 = assign83940_e127414_d_n10;
        locals.var_dnm_dn13 = assign83940_e127414_d_n13;

        let (assign83950_e127429, assign83950_e127429_d_n0, assign83950_e127429_d_n2, assign83950_e127429_d_n4, assign83950_e127429_d_n5, assign83950_e127429_d_n6, assign83950_e127429_d_n7, assign83950_e127429_d_n8, assign83950_e127429_d_n9, assign83950_e127429_d_n10, assign83950_e127429_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1938 != 0.0)) && (locals.var_guard1939 != 0.0)) && (locals.var_guard1940 != 0.0)) {
        let assign83950_e127427: f64 = (1.0 / locals.var_dnm);
        (assign83950_e127427, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn4 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn5 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn8 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn9 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn13 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign83950_e127429;
        locals.var_dnm_dn0 = assign83950_e127429_d_n0;
        locals.var_dnm_dn2 = assign83950_e127429_d_n2;
        locals.var_dnm_dn4 = assign83950_e127429_d_n4;
        locals.var_dnm_dn5 = assign83950_e127429_d_n5;
        locals.var_dnm_dn6 = assign83950_e127429_d_n6;
        locals.var_dnm_dn7 = assign83950_e127429_d_n7;
        locals.var_dnm_dn8 = assign83950_e127429_d_n8;
        locals.var_dnm_dn9 = assign83950_e127429_d_n9;
        locals.var_dnm_dn10 = assign83950_e127429_d_n10;
        locals.var_dnm_dn13 = assign83950_e127429_d_n13;

        let (assign83960_e127446, assign83960_e127446_d_n0, assign83960_e127446_d_n2, assign83960_e127446_d_n4, assign83960_e127446_d_n5, assign83960_e127446_d_n6, assign83960_e127446_d_n7, assign83960_e127446_d_n8, assign83960_e127446_d_n9, assign83960_e127446_d_n10, assign83960_e127446_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1938 != 0.0)) && (locals.var_guard1939 != 0.0)) && (locals.var_guard1940 != 0.0)) {
        let assign83960_e127442: f64 = (locals.var_tmf1 * 0.1);
        let assign83960_e127444: f64 = (assign83960_e127442 * locals.var_dnm);
        (assign83960_e127444, (((locals.var_tmf1_dn0 * 0.1) * locals.var_dnm) + (assign83960_e127442 * locals.var_dnm_dn0)), (((locals.var_tmf1_dn2 * 0.1) * locals.var_dnm) + (assign83960_e127442 * locals.var_dnm_dn2)), (((locals.var_tmf1_dn4 * 0.1) * locals.var_dnm) + (assign83960_e127442 * locals.var_dnm_dn4)), (((locals.var_tmf1_dn5 * 0.1) * locals.var_dnm) + (assign83960_e127442 * locals.var_dnm_dn5)), (((locals.var_tmf1_dn6 * 0.1) * locals.var_dnm) + (assign83960_e127442 * locals.var_dnm_dn6)), (((locals.var_tmf1_dn7 * 0.1) * locals.var_dnm) + (assign83960_e127442 * locals.var_dnm_dn7)), (((locals.var_tmf1_dn8 * 0.1) * locals.var_dnm) + (assign83960_e127442 * locals.var_dnm_dn8)), (((locals.var_tmf1_dn9 * 0.1) * locals.var_dnm) + (assign83960_e127442 * locals.var_dnm_dn9)), (((locals.var_tmf1_dn10 * 0.1) * locals.var_dnm) + (assign83960_e127442 * locals.var_dnm_dn10)), (((locals.var_tmf1_dn13 * 0.1) * locals.var_dnm) + (assign83960_e127442 * locals.var_dnm_dn13)),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn13,)
    }
};
        locals.var_tmf0 = assign83960_e127446;
        locals.var_tmf0_dn0 = assign83960_e127446_d_n0;
        locals.var_tmf0_dn2 = assign83960_e127446_d_n2;
        locals.var_tmf0_dn4 = assign83960_e127446_d_n4;
        locals.var_tmf0_dn5 = assign83960_e127446_d_n5;
        locals.var_tmf0_dn6 = assign83960_e127446_d_n6;
        locals.var_tmf0_dn7 = assign83960_e127446_d_n7;
        locals.var_tmf0_dn8 = assign83960_e127446_d_n8;
        locals.var_tmf0_dn9 = assign83960_e127446_d_n9;
        locals.var_tmf0_dn10 = assign83960_e127446_d_n10;
        locals.var_tmf0_dn13 = assign83960_e127446_d_n13;

        let (assign83970_e127465, assign83970_e127465_d_n0, assign83970_e127465_d_n2, assign83970_e127465_d_n4, assign83970_e127465_d_n5, assign83970_e127465_d_n6, assign83970_e127465_d_n7, assign83970_e127465_d_n8, assign83970_e127465_d_n9, assign83970_e127465_d_n10, assign83970_e127465_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1938 != 0.0)) && (locals.var_guard1939 != 0.0)) && (locals.var_guard1940 != 0.0)) {
        let assign83970_e127459: f64 = (0.1 * locals.var_xmp);
        let assign83970_e127461: f64 = (assign83970_e127459 * locals.var_dnm);
        let assign83970_e127463: f64 = (assign83970_e127461 / locals.var_arg);
        (assign83970_e127463, ((((((0.1 * locals.var_xmp_dn0) * locals.var_dnm) + (assign83970_e127459 * locals.var_dnm_dn0)) * locals.var_arg) - (assign83970_e127461 * locals.var_arg_dn0)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn2) * locals.var_dnm) + (assign83970_e127459 * locals.var_dnm_dn2)) * locals.var_arg) - (assign83970_e127461 * locals.var_arg_dn2)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn4) * locals.var_dnm) + (assign83970_e127459 * locals.var_dnm_dn4)) * locals.var_arg) - (assign83970_e127461 * locals.var_arg_dn4)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn5) * locals.var_dnm) + (assign83970_e127459 * locals.var_dnm_dn5)) * locals.var_arg) - (assign83970_e127461 * locals.var_arg_dn5)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn6) * locals.var_dnm) + (assign83970_e127459 * locals.var_dnm_dn6)) * locals.var_arg) - (assign83970_e127461 * locals.var_arg_dn6)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn7) * locals.var_dnm) + (assign83970_e127459 * locals.var_dnm_dn7)) * locals.var_arg) - (assign83970_e127461 * locals.var_arg_dn7)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn8) * locals.var_dnm) + (assign83970_e127459 * locals.var_dnm_dn8)) * locals.var_arg) - (assign83970_e127461 * locals.var_arg_dn8)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn9) * locals.var_dnm) + (assign83970_e127459 * locals.var_dnm_dn9)) * locals.var_arg) - (assign83970_e127461 * locals.var_arg_dn9)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn10) * locals.var_dnm) + (assign83970_e127459 * locals.var_dnm_dn10)) * locals.var_arg) - (assign83970_e127461 * locals.var_arg_dn10)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn13) * locals.var_dnm) + (assign83970_e127459 * locals.var_dnm_dn13)) * locals.var_arg) - (assign83970_e127461 * locals.var_arg_dn13)) / (locals.var_arg * locals.var_arg)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign83970_e127465;
        locals.var_t0_dn0 = assign83970_e127465_d_n0;
        locals.var_t0_dn2 = assign83970_e127465_d_n2;
        locals.var_t0_dn4 = assign83970_e127465_d_n4;
        locals.var_t0_dn5 = assign83970_e127465_d_n5;
        locals.var_t0_dn6 = assign83970_e127465_d_n6;
        locals.var_t0_dn7 = assign83970_e127465_d_n7;
        locals.var_t0_dn8 = assign83970_e127465_d_n8;
        locals.var_t0_dn9 = assign83970_e127465_d_n9;
        locals.var_t0_dn10 = assign83970_e127465_d_n10;
        locals.var_t0_dn13 = assign83970_e127465_d_n13;

    }

    pub(super) fn stamp_transient_block_292(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign83980_e127482, assign83980_e127482_d_n0, assign83980_e127482_d_n2, assign83980_e127482_d_n4, assign83980_e127482_d_n5, assign83980_e127482_d_n6, assign83980_e127482_d_n7, assign83980_e127482_d_n8, assign83980_e127482_d_n9, assign83980_e127482_d_n10, assign83980_e127482_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1938 != 0.0)) && (locals.var_guard1939 != 0.0)) && (locals.var_guard1940 != 0.0)) {
        let assign83980_e127478: f64 = (locals.var_ps0ld_bef1__blk1928 - 0.1);
        let assign83980_e127480: f64 = (assign83980_e127478 + locals.var_tmf0);
        (assign83980_e127480, (locals.var_ps0ld_bef1__blk1928_dn0 + locals.var_tmf0_dn0), (locals.var_ps0ld_bef1__blk1928_dn2 + locals.var_tmf0_dn2), (locals.var_ps0ld_bef1__blk1928_dn4 + locals.var_tmf0_dn4), (locals.var_ps0ld_bef1__blk1928_dn5 + locals.var_tmf0_dn5), (locals.var_ps0ld_bef1__blk1928_dn6 + locals.var_tmf0_dn6), (locals.var_ps0ld_bef1__blk1928_dn7 + locals.var_tmf0_dn7), (locals.var_ps0ld_bef1__blk1928_dn8 + locals.var_tmf0_dn8), (locals.var_ps0ld_bef1__blk1928_dn9 + locals.var_tmf0_dn9), (locals.var_ps0ld_bef1__blk1928_dn10 + locals.var_tmf0_dn10), (locals.var_ps0ld_bef1__blk1928_dn13 + locals.var_tmf0_dn13),)
    } else {
        (locals.var_ps0ld, locals.var_ps0ld_dn0, locals.var_ps0ld_dn2, locals.var_ps0ld_dn4, locals.var_ps0ld_dn5, locals.var_ps0ld_dn6, locals.var_ps0ld_dn7, locals.var_ps0ld_dn8, locals.var_ps0ld_dn9, locals.var_ps0ld_dn10, locals.var_ps0ld_dn13,)
    }
};
        locals.var_ps0ld = assign83980_e127482;
        locals.var_ps0ld_dn0 = assign83980_e127482_d_n0;
        locals.var_ps0ld_dn2 = assign83980_e127482_d_n2;
        locals.var_ps0ld_dn4 = assign83980_e127482_d_n4;
        locals.var_ps0ld_dn5 = assign83980_e127482_d_n5;
        locals.var_ps0ld_dn6 = assign83980_e127482_d_n6;
        locals.var_ps0ld_dn7 = assign83980_e127482_d_n7;
        locals.var_ps0ld_dn8 = assign83980_e127482_d_n8;
        locals.var_ps0ld_dn9 = assign83980_e127482_d_n9;
        locals.var_ps0ld_dn10 = assign83980_e127482_d_n10;
        locals.var_ps0ld_dn13 = assign83980_e127482_d_n13;

        let (assign83990_e127495, assign83990_e127495_d_n0, assign83990_e127495_d_n2, assign83990_e127495_d_n4, assign83990_e127495_d_n5, assign83990_e127495_d_n6, assign83990_e127495_d_n7, assign83990_e127495_d_n8, assign83990_e127495_d_n9, assign83990_e127495_d_n10, assign83990_e127495_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1938 != 0.0)) && (locals.var_guard1939 != 0.0)) && (locals.var_guard1940 != 0.0)) {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign83990_e127495;
        locals.var_t0_dn0 = assign83990_e127495_d_n0;
        locals.var_t0_dn2 = assign83990_e127495_d_n2;
        locals.var_t0_dn4 = assign83990_e127495_d_n4;
        locals.var_t0_dn5 = assign83990_e127495_d_n5;
        locals.var_t0_dn6 = assign83990_e127495_d_n6;
        locals.var_t0_dn7 = assign83990_e127495_d_n7;
        locals.var_t0_dn8 = assign83990_e127495_d_n8;
        locals.var_t0_dn9 = assign83990_e127495_d_n9;
        locals.var_t0_dn10 = assign83990_e127495_d_n10;
        locals.var_t0_dn13 = assign83990_e127495_d_n13;

        let (assign84000_e127509, assign84000_e127509_d_n0, assign84000_e127509_d_n2, assign84000_e127509_d_n4, assign84000_e127509_d_n5, assign84000_e127509_d_n6, assign84000_e127509_d_n7, assign84000_e127509_d_n8, assign84000_e127509_d_n9, assign84000_e127509_d_n10, assign84000_e127509_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1938 != 0.0)) && (locals.var_guard1939 != 0.0)) && (locals.var_guard1940 == 0.0)) {
        (locals.var_ps0ld, locals.var_ps0ld_dn0, locals.var_ps0ld_dn2, locals.var_ps0ld_dn4, locals.var_ps0ld_dn5, locals.var_ps0ld_dn6, locals.var_ps0ld_dn7, locals.var_ps0ld_dn8, locals.var_ps0ld_dn9, locals.var_ps0ld_dn10, locals.var_ps0ld_dn13,)
    } else {
        (locals.var_ps0ld, locals.var_ps0ld_dn0, locals.var_ps0ld_dn2, locals.var_ps0ld_dn4, locals.var_ps0ld_dn5, locals.var_ps0ld_dn6, locals.var_ps0ld_dn7, locals.var_ps0ld_dn8, locals.var_ps0ld_dn9, locals.var_ps0ld_dn10, locals.var_ps0ld_dn13,)
    }
};
        locals.var_ps0ld = assign84000_e127509;
        locals.var_ps0ld_dn0 = assign84000_e127509_d_n0;
        locals.var_ps0ld_dn2 = assign84000_e127509_d_n2;
        locals.var_ps0ld_dn4 = assign84000_e127509_d_n4;
        locals.var_ps0ld_dn5 = assign84000_e127509_d_n5;
        locals.var_ps0ld_dn6 = assign84000_e127509_d_n6;
        locals.var_ps0ld_dn7 = assign84000_e127509_d_n7;
        locals.var_ps0ld_dn8 = assign84000_e127509_d_n8;
        locals.var_ps0ld_dn9 = assign84000_e127509_d_n9;
        locals.var_ps0ld_dn10 = assign84000_e127509_d_n10;
        locals.var_ps0ld_dn13 = assign84000_e127509_d_n13;

        let (assign84010_e127523, assign84010_e127523_d_n0, assign84010_e127523_d_n2, assign84010_e127523_d_n4, assign84010_e127523_d_n5, assign84010_e127523_d_n6, assign84010_e127523_d_n7, assign84010_e127523_d_n8, assign84010_e127523_d_n9, assign84010_e127523_d_n10, assign84010_e127523_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1938 != 0.0)) && (locals.var_guard1939 != 0.0)) && (locals.var_guard1940 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign84010_e127523;
        locals.var_t0_dn0 = assign84010_e127523_d_n0;
        locals.var_t0_dn2 = assign84010_e127523_d_n2;
        locals.var_t0_dn4 = assign84010_e127523_d_n4;
        locals.var_t0_dn5 = assign84010_e127523_d_n5;
        locals.var_t0_dn6 = assign84010_e127523_d_n6;
        locals.var_t0_dn7 = assign84010_e127523_d_n7;
        locals.var_t0_dn8 = assign84010_e127523_d_n8;
        locals.var_t0_dn9 = assign84010_e127523_d_n9;
        locals.var_t0_dn10 = assign84010_e127523_d_n10;
        locals.var_t0_dn13 = assign84010_e127523_d_n13;

        let (assign84020_e127540, assign84020_e127540_d_n0, assign84020_e127540_d_n2, assign84020_e127540_d_n4, assign84020_e127540_d_n5, assign84020_e127540_d_n6, assign84020_e127540_d_n7, assign84020_e127540_d_n8, assign84020_e127540_d_n9, assign84020_e127540_d_n10, assign84020_e127540_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1938 != 0.0)) && (locals.var_guard1939 == 0.0)) {
        let (assign84020_e127538, assign84020_e127538_d_n0, assign84020_e127538_d_n2, assign84020_e127538_d_n4, assign84020_e127538_d_n5, assign84020_e127538_d_n6, assign84020_e127538_d_n7, assign84020_e127538_d_n8, assign84020_e127538_d_n9, assign84020_e127538_d_n10, assign84020_e127538_d_n13,) = {
            if (locals.var_ps0ld <= locals.var_ps0ld_bef1__blk1928) {
                (locals.var_ps0ld, locals.var_ps0ld_dn0, locals.var_ps0ld_dn2, locals.var_ps0ld_dn4, locals.var_ps0ld_dn5, locals.var_ps0ld_dn6, locals.var_ps0ld_dn7, locals.var_ps0ld_dn8, locals.var_ps0ld_dn9, locals.var_ps0ld_dn10, locals.var_ps0ld_dn13,)
            } else {
                (locals.var_ps0ld_bef1__blk1928, locals.var_ps0ld_bef1__blk1928_dn0, locals.var_ps0ld_bef1__blk1928_dn2, locals.var_ps0ld_bef1__blk1928_dn4, locals.var_ps0ld_bef1__blk1928_dn5, locals.var_ps0ld_bef1__blk1928_dn6, locals.var_ps0ld_bef1__blk1928_dn7, locals.var_ps0ld_bef1__blk1928_dn8, locals.var_ps0ld_bef1__blk1928_dn9, locals.var_ps0ld_bef1__blk1928_dn10, locals.var_ps0ld_bef1__blk1928_dn13,)
            }
        };
        (assign84020_e127538, assign84020_e127538_d_n0, assign84020_e127538_d_n2, assign84020_e127538_d_n4, assign84020_e127538_d_n5, assign84020_e127538_d_n6, assign84020_e127538_d_n7, assign84020_e127538_d_n8, assign84020_e127538_d_n9, assign84020_e127538_d_n10, assign84020_e127538_d_n13,)
    } else {
        (locals.var_ps0ld, locals.var_ps0ld_dn0, locals.var_ps0ld_dn2, locals.var_ps0ld_dn4, locals.var_ps0ld_dn5, locals.var_ps0ld_dn6, locals.var_ps0ld_dn7, locals.var_ps0ld_dn8, locals.var_ps0ld_dn9, locals.var_ps0ld_dn10, locals.var_ps0ld_dn13,)
    }
};
        locals.var_ps0ld = assign84020_e127540;
        locals.var_ps0ld_dn0 = assign84020_e127540_d_n0;
        locals.var_ps0ld_dn2 = assign84020_e127540_d_n2;
        locals.var_ps0ld_dn4 = assign84020_e127540_d_n4;
        locals.var_ps0ld_dn5 = assign84020_e127540_d_n5;
        locals.var_ps0ld_dn6 = assign84020_e127540_d_n6;
        locals.var_ps0ld_dn7 = assign84020_e127540_d_n7;
        locals.var_ps0ld_dn8 = assign84020_e127540_d_n8;
        locals.var_ps0ld_dn9 = assign84020_e127540_d_n9;
        locals.var_ps0ld_dn10 = assign84020_e127540_d_n10;
        locals.var_ps0ld_dn13 = assign84020_e127540_d_n13;

        let (assign84030_e127547, assign84030_e127547_d_n0, assign84030_e127547_d_n2, assign84030_e127547_d_n4, assign84030_e127547_d_n5, assign84030_e127547_d_n6, assign84030_e127547_d_n7, assign84030_e127547_d_n8, assign84030_e127547_d_n9, assign84030_e127547_d_n10, assign84030_e127547_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) {
        (locals.var_ps0ld, locals.var_ps0ld_dn0, locals.var_ps0ld_dn2, locals.var_ps0ld_dn4, locals.var_ps0ld_dn5, locals.var_ps0ld_dn6, locals.var_ps0ld_dn7, locals.var_ps0ld_dn8, locals.var_ps0ld_dn9, locals.var_ps0ld_dn10, locals.var_ps0ld_dn13,)
    } else {
        (locals.var_ps0ld_ini__blk1890, locals.var_ps0ld_ini__blk1890_dn0, locals.var_ps0ld_ini__blk1890_dn2, locals.var_ps0ld_ini__blk1890_dn4, locals.var_ps0ld_ini__blk1890_dn5, locals.var_ps0ld_ini__blk1890_dn6, locals.var_ps0ld_ini__blk1890_dn7, locals.var_ps0ld_ini__blk1890_dn8, locals.var_ps0ld_ini__blk1890_dn9, locals.var_ps0ld_ini__blk1890_dn10, locals.var_ps0ld_ini__blk1890_dn13,)
    }
};
        locals.var_ps0ld_ini__blk1890 = assign84030_e127547;
        locals.var_ps0ld_ini__blk1890_dn0 = assign84030_e127547_d_n0;
        locals.var_ps0ld_ini__blk1890_dn2 = assign84030_e127547_d_n2;
        locals.var_ps0ld_ini__blk1890_dn4 = assign84030_e127547_d_n4;
        locals.var_ps0ld_ini__blk1890_dn5 = assign84030_e127547_d_n5;
        locals.var_ps0ld_ini__blk1890_dn6 = assign84030_e127547_d_n6;
        locals.var_ps0ld_ini__blk1890_dn7 = assign84030_e127547_d_n7;
        locals.var_ps0ld_ini__blk1890_dn8 = assign84030_e127547_d_n8;
        locals.var_ps0ld_ini__blk1890_dn9 = assign84030_e127547_d_n9;
        locals.var_ps0ld_ini__blk1890_dn10 = assign84030_e127547_d_n10;
        locals.var_ps0ld_ini__blk1890_dn13 = assign84030_e127547_d_n13;

        let assign84040_e127550: f64 = if p.p33 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1946 = assign84040_e127550;

        let (assign84050_e127559,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1946 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_flg_conv,)
    }
};
        locals.var_flg_conv = assign84050_e127559;

        let (assign84060_e127575, assign84060_e127575_d_n0, assign84060_e127575_d_n2, assign84060_e127575_d_n4, assign84060_e127575_d_n5, assign84060_e127575_d_n6, assign84060_e127575_d_n7, assign84060_e127575_d_n8, assign84060_e127575_d_n9, assign84060_e127575_d_n10, assign84060_e127575_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1946 != 0.0)) {
        let assign84060_e127569: f64 = (1.034943e-10 / locals.var_q_nsubld__blk1883);
        let assign84060_e127571: f64 = (assign84060_e127569 * locals.var_beta_inv);
        let assign84060_e127572: f64 = (2.0 * assign84060_e127571);
        let assign84060_e127573: f64 = (assign84060_e127572).sqrt();
        (assign84060_e127573, ((2.0 * (assign84060_e127569 * locals.var_beta_inv_dn0)) / (2.0 * assign84060_e127573)), ((2.0 * (assign84060_e127569 * locals.var_beta_inv_dn2)) / (2.0 * assign84060_e127573)), ((2.0 * (assign84060_e127569 * locals.var_beta_inv_dn4)) / (2.0 * assign84060_e127573)), ((2.0 * (assign84060_e127569 * locals.var_beta_inv_dn5)) / (2.0 * assign84060_e127573)), ((2.0 * (assign84060_e127569 * locals.var_beta_inv_dn6)) / (2.0 * assign84060_e127573)), ((2.0 * (assign84060_e127569 * locals.var_beta_inv_dn7)) / (2.0 * assign84060_e127573)), ((2.0 * (assign84060_e127569 * locals.var_beta_inv_dn8)) / (2.0 * assign84060_e127573)), ((2.0 * (assign84060_e127569 * locals.var_beta_inv_dn9)) / (2.0 * assign84060_e127573)), ((2.0 * (assign84060_e127569 * locals.var_beta_inv_dn10)) / (2.0 * assign84060_e127573)), ((2.0 * (assign84060_e127569 * locals.var_beta_inv_dn13)) / (2.0 * assign84060_e127573)),)
    } else {
        (locals.var_c_w_ld, locals.var_c_w_ld_dn0, locals.var_c_w_ld_dn2, locals.var_c_w_ld_dn4, locals.var_c_w_ld_dn5, locals.var_c_w_ld_dn6, locals.var_c_w_ld_dn7, locals.var_c_w_ld_dn8, locals.var_c_w_ld_dn9, locals.var_c_w_ld_dn10, locals.var_c_w_ld_dn13,)
    }
};
        locals.var_c_w_ld = assign84060_e127575;
        locals.var_c_w_ld_dn0 = assign84060_e127575_d_n0;
        locals.var_c_w_ld_dn2 = assign84060_e127575_d_n2;
        locals.var_c_w_ld_dn4 = assign84060_e127575_d_n4;
        locals.var_c_w_ld_dn5 = assign84060_e127575_d_n5;
        locals.var_c_w_ld_dn6 = assign84060_e127575_d_n6;
        locals.var_c_w_ld_dn7 = assign84060_e127575_d_n7;
        locals.var_c_w_ld_dn8 = assign84060_e127575_d_n8;
        locals.var_c_w_ld_dn9 = assign84060_e127575_d_n9;
        locals.var_c_w_ld_dn10 = assign84060_e127575_d_n10;
        locals.var_c_w_ld_dn13 = assign84060_e127575_d_n13;

        let assign84070_e127578: f64 = if locals.var_wdep_func > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1947 = assign84070_e127578;

        let (assign84080_e127591, assign84080_e127591_d_n0, assign84080_e127591_d_n2, assign84080_e127591_d_n4, assign84080_e127591_d_n5, assign84080_e127591_d_n6, assign84080_e127591_d_n7, assign84080_e127591_d_n8, assign84080_e127591_d_n9, assign84080_e127591_d_n10, assign84080_e127591_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1946 != 0.0)) && (locals.var_guard1947 != 0.0)) {
        let assign84080_e127589: f64 = (p.p334 - locals.var_wdep_func);
        (assign84080_e127589, (-locals.var_wdep_func_dn0), (-locals.var_wdep_func_dn2), (-locals.var_wdep_func_dn4), (-locals.var_wdep_func_dn5), (-locals.var_wdep_func_dn6), (-locals.var_wdep_func_dn7), (-locals.var_wdep_func_dn8), (-locals.var_wdep_func_dn9), (-locals.var_wdep_func_dn10), (-locals.var_wdep_func_dn13),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign84080_e127591;
        locals.var_t2_dn0 = assign84080_e127591_d_n0;
        locals.var_t2_dn2 = assign84080_e127591_d_n2;
        locals.var_t2_dn4 = assign84080_e127591_d_n4;
        locals.var_t2_dn5 = assign84080_e127591_d_n5;
        locals.var_t2_dn6 = assign84080_e127591_d_n6;
        locals.var_t2_dn7 = assign84080_e127591_d_n7;
        locals.var_t2_dn8 = assign84080_e127591_d_n8;
        locals.var_t2_dn9 = assign84080_e127591_d_n9;
        locals.var_t2_dn10 = assign84080_e127591_d_n10;
        locals.var_t2_dn13 = assign84080_e127591_d_n13;

        let (assign84090_e127616, assign84090_e127616_d_n0, assign84090_e127616_d_n2, assign84090_e127616_d_n4, assign84090_e127616_d_n5, assign84090_e127616_d_n6, assign84090_e127616_d_n7, assign84090_e127616_d_n8, assign84090_e127616_d_n9, assign84090_e127616_d_n10, assign84090_e127616_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1946 != 0.0)) && (locals.var_guard1947 == 0.0)) {
        let assign84090_e127603: f64 = (locals.var_vdsi + p.p137);
        let assign84090_e127606: f64 = (locals.var_vdsi + p.p137);
        let assign84090_e127607: f64 = (assign84090_e127603 * assign84090_e127606);
        let assign84090_e127610: f64 = (4.0 * 0.1);
        let assign84090_e127612: f64 = (assign84090_e127610 * 0.1);
        let assign84090_e127613: f64 = (assign84090_e127607 + assign84090_e127612);
        let assign84090_e127614: f64 = (assign84090_e127613).sqrt();
        (assign84090_e127614, 0.0, 0.0, 0.0, (((locals.var_vdsi_dn5 * assign84090_e127606) + (assign84090_e127603 * locals.var_vdsi_dn5)) / (2.0 * assign84090_e127614)), 0.0, (((locals.var_vdsi_dn7 * assign84090_e127606) + (assign84090_e127603 * locals.var_vdsi_dn7)) / (2.0 * assign84090_e127614)), 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign84090_e127616;
        locals.var_tmf2_dn0 = assign84090_e127616_d_n0;
        locals.var_tmf2_dn2 = assign84090_e127616_d_n2;
        locals.var_tmf2_dn4 = assign84090_e127616_d_n4;
        locals.var_tmf2_dn5 = assign84090_e127616_d_n5;
        locals.var_tmf2_dn6 = assign84090_e127616_d_n6;
        locals.var_tmf2_dn7 = assign84090_e127616_d_n7;
        locals.var_tmf2_dn8 = assign84090_e127616_d_n8;
        locals.var_tmf2_dn9 = assign84090_e127616_d_n9;
        locals.var_tmf2_dn10 = assign84090_e127616_d_n10;
        locals.var_tmf2_dn13 = assign84090_e127616_d_n13;

        let (assign84100_e127636, assign84100_e127636_d_n0, assign84100_e127636_d_n2, assign84100_e127636_d_n4, assign84100_e127636_d_n5, assign84100_e127636_d_n6, assign84100_e127636_d_n7, assign84100_e127636_d_n8, assign84100_e127636_d_n9, assign84100_e127636_d_n10, assign84100_e127636_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1946 != 0.0)) && (locals.var_guard1947 == 0.0)) {
        let assign84100_e127630: f64 = (locals.var_vdsi + p.p137);
        let assign84100_e127632: f64 = (assign84100_e127630 / locals.var_tmf2);
        let assign84100_e127633: f64 = (1.0 + assign84100_e127632);
        let assign84100_e127634: f64 = (0.5 * assign84100_e127633);
        (assign84100_e127634, (0.5 * (-((assign84100_e127630 * locals.var_tmf2_dn0) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((assign84100_e127630 * locals.var_tmf2_dn2) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((assign84100_e127630 * locals.var_tmf2_dn4) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (((locals.var_vdsi_dn5 * locals.var_tmf2) - (assign84100_e127630 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (-((assign84100_e127630 * locals.var_tmf2_dn6) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (((locals.var_vdsi_dn7 * locals.var_tmf2) - (assign84100_e127630 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (-((assign84100_e127630 * locals.var_tmf2_dn8) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((assign84100_e127630 * locals.var_tmf2_dn9) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((assign84100_e127630 * locals.var_tmf2_dn10) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((assign84100_e127630 * locals.var_tmf2_dn13) / (locals.var_tmf2 * locals.var_tmf2)))),)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn13,)
    }
};
        locals.var_t9 = assign84100_e127636;
        locals.var_t9_dn0 = assign84100_e127636_d_n0;
        locals.var_t9_dn2 = assign84100_e127636_d_n2;
        locals.var_t9_dn4 = assign84100_e127636_d_n4;
        locals.var_t9_dn5 = assign84100_e127636_d_n5;
        locals.var_t9_dn6 = assign84100_e127636_d_n6;
        locals.var_t9_dn7 = assign84100_e127636_d_n7;
        locals.var_t9_dn8 = assign84100_e127636_d_n8;
        locals.var_t9_dn9 = assign84100_e127636_d_n9;
        locals.var_t9_dn10 = assign84100_e127636_d_n10;
        locals.var_t9_dn13 = assign84100_e127636_d_n13;

        let (assign84110_e127654, assign84110_e127654_d_n0, assign84110_e127654_d_n2, assign84110_e127654_d_n4, assign84110_e127654_d_n5, assign84110_e127654_d_n6, assign84110_e127654_d_n7, assign84110_e127654_d_n8, assign84110_e127654_d_n9, assign84110_e127654_d_n10, assign84110_e127654_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1946 != 0.0)) && (locals.var_guard1947 == 0.0)) {
        let assign84110_e127649: f64 = (locals.var_vdsi + p.p137);
        let assign84110_e127651: f64 = (assign84110_e127649 + locals.var_tmf2);
        let assign84110_e127652: f64 = (0.5 * assign84110_e127651);
        (assign84110_e127652, (0.5 * locals.var_tmf2_dn0), (0.5 * locals.var_tmf2_dn2), (0.5 * locals.var_tmf2_dn4), (0.5 * (locals.var_vdsi_dn5 + locals.var_tmf2_dn5)), (0.5 * locals.var_tmf2_dn6), (0.5 * (locals.var_vdsi_dn7 + locals.var_tmf2_dn7)), (0.5 * locals.var_tmf2_dn8), (0.5 * locals.var_tmf2_dn9), (0.5 * locals.var_tmf2_dn10), (0.5 * locals.var_tmf2_dn13),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign84110_e127654;
        locals.var_t2_dn0 = assign84110_e127654_d_n0;
        locals.var_t2_dn2 = assign84110_e127654_d_n2;
        locals.var_t2_dn4 = assign84110_e127654_d_n4;
        locals.var_t2_dn5 = assign84110_e127654_d_n5;
        locals.var_t2_dn6 = assign84110_e127654_d_n6;
        locals.var_t2_dn7 = assign84110_e127654_d_n7;
        locals.var_t2_dn8 = assign84110_e127654_d_n8;
        locals.var_t2_dn9 = assign84110_e127654_d_n9;
        locals.var_t2_dn10 = assign84110_e127654_d_n10;
        locals.var_t2_dn13 = assign84110_e127654_d_n13;

        let assign84120_e127657: f64 = if locals.var_t2 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1948 = assign84120_e127657;

        let (assign84130_e127671, assign84130_e127671_d_n0, assign84130_e127671_d_n2, assign84130_e127671_d_n4, assign84130_e127671_d_n5, assign84130_e127671_d_n6, assign84130_e127671_d_n7, assign84130_e127671_d_n8, assign84130_e127671_d_n9, assign84130_e127671_d_n10, assign84130_e127671_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1946 != 0.0)) && (locals.var_guard1947 == 0.0)) && (locals.var_guard1948 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign84130_e127671;
        locals.var_t2_dn0 = assign84130_e127671_d_n0;
        locals.var_t2_dn2 = assign84130_e127671_d_n2;
        locals.var_t2_dn4 = assign84130_e127671_d_n4;
        locals.var_t2_dn5 = assign84130_e127671_d_n5;
        locals.var_t2_dn6 = assign84130_e127671_d_n6;
        locals.var_t2_dn7 = assign84130_e127671_d_n7;
        locals.var_t2_dn8 = assign84130_e127671_d_n8;
        locals.var_t2_dn9 = assign84130_e127671_d_n9;
        locals.var_t2_dn10 = assign84130_e127671_d_n10;
        locals.var_t2_dn13 = assign84130_e127671_d_n13;

        let (assign84140_e127685, assign84140_e127685_d_n0, assign84140_e127685_d_n2, assign84140_e127685_d_n4, assign84140_e127685_d_n5, assign84140_e127685_d_n6, assign84140_e127685_d_n7, assign84140_e127685_d_n8, assign84140_e127685_d_n9, assign84140_e127685_d_n10, assign84140_e127685_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1946 != 0.0)) && (locals.var_guard1947 == 0.0)) && (locals.var_guard1948 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn13,)
    }
};
        locals.var_t9 = assign84140_e127685;
        locals.var_t9_dn0 = assign84140_e127685_d_n0;
        locals.var_t9_dn2 = assign84140_e127685_d_n2;
        locals.var_t9_dn4 = assign84140_e127685_d_n4;
        locals.var_t9_dn5 = assign84140_e127685_d_n5;
        locals.var_t9_dn6 = assign84140_e127685_d_n6;
        locals.var_t9_dn7 = assign84140_e127685_d_n7;
        locals.var_t9_dn8 = assign84140_e127685_d_n8;
        locals.var_t9_dn9 = assign84140_e127685_d_n9;
        locals.var_t9_dn10 = assign84140_e127685_d_n10;
        locals.var_t9_dn13 = assign84140_e127685_d_n13;

        let (assign84150_e127702, assign84150_e127702_d_n0, assign84150_e127702_d_n2, assign84150_e127702_d_n4, assign84150_e127702_d_n5, assign84150_e127702_d_n6, assign84150_e127702_d_n7, assign84150_e127702_d_n8, assign84150_e127702_d_n9, assign84150_e127702_d_n10, assign84150_e127702_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1946 != 0.0)) && (locals.var_guard1947 == 0.0)) {
        let assign84150_e127697: f64 = (locals.var_kjunc * locals.var_t2);
        let assign84150_e127698: f64 = (assign84150_e127697).sqrt();
        let assign84150_e127700: f64 = (assign84150_e127698 * p.p432);
        (assign84150_e127700, ((((locals.var_kjunc_dn0 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn0)) / (2.0 * assign84150_e127698)) * p.p432), ((((locals.var_kjunc_dn2 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn2)) / (2.0 * assign84150_e127698)) * p.p432), ((((locals.var_kjunc_dn4 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn4)) / (2.0 * assign84150_e127698)) * p.p432), ((((locals.var_kjunc_dn5 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn5)) / (2.0 * assign84150_e127698)) * p.p432), ((((locals.var_kjunc_dn6 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn6)) / (2.0 * assign84150_e127698)) * p.p432), ((((locals.var_kjunc_dn7 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn7)) / (2.0 * assign84150_e127698)) * p.p432), ((((locals.var_kjunc_dn8 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn8)) / (2.0 * assign84150_e127698)) * p.p432), ((((locals.var_kjunc_dn9 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn9)) / (2.0 * assign84150_e127698)) * p.p432), ((((locals.var_kjunc_dn10 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn10)) / (2.0 * assign84150_e127698)) * p.p432), ((((locals.var_kjunc_dn13 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn13)) / (2.0 * assign84150_e127698)) * p.p432),)
    } else {
        (locals.var_wjunc0, locals.var_wjunc0_dn0, locals.var_wjunc0_dn2, locals.var_wjunc0_dn4, locals.var_wjunc0_dn5, locals.var_wjunc0_dn6, locals.var_wjunc0_dn7, locals.var_wjunc0_dn8, locals.var_wjunc0_dn9, locals.var_wjunc0_dn10, locals.var_wjunc0_dn13,)
    }
};
        locals.var_wjunc0 = assign84150_e127702;
        locals.var_wjunc0_dn0 = assign84150_e127702_d_n0;
        locals.var_wjunc0_dn2 = assign84150_e127702_d_n2;
        locals.var_wjunc0_dn4 = assign84150_e127702_d_n4;
        locals.var_wjunc0_dn5 = assign84150_e127702_d_n5;
        locals.var_wjunc0_dn6 = assign84150_e127702_d_n6;
        locals.var_wjunc0_dn7 = assign84150_e127702_d_n7;
        locals.var_wjunc0_dn8 = assign84150_e127702_d_n8;
        locals.var_wjunc0_dn9 = assign84150_e127702_d_n9;
        locals.var_wjunc0_dn10 = assign84150_e127702_d_n10;
        locals.var_wjunc0_dn13 = assign84150_e127702_d_n13;

        let (assign84160_e127716, assign84160_e127716_d_n0, assign84160_e127716_d_n2, assign84160_e127716_d_n4, assign84160_e127716_d_n5, assign84160_e127716_d_n6, assign84160_e127716_d_n7, assign84160_e127716_d_n8, assign84160_e127716_d_n9, assign84160_e127716_d_n10, assign84160_e127716_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1946 != 0.0)) && (locals.var_guard1947 == 0.0)) {
        let assign84160_e127714: f64 = (p.p334 - locals.var_wjunc0);
        (assign84160_e127714, (-locals.var_wjunc0_dn0), (-locals.var_wjunc0_dn2), (-locals.var_wjunc0_dn4), (-locals.var_wjunc0_dn5), (-locals.var_wjunc0_dn6), (-locals.var_wjunc0_dn7), (-locals.var_wjunc0_dn8), (-locals.var_wjunc0_dn9), (-locals.var_wjunc0_dn10), (-locals.var_wjunc0_dn13),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign84160_e127716;
        locals.var_t2_dn0 = assign84160_e127716_d_n0;
        locals.var_t2_dn2 = assign84160_e127716_d_n2;
        locals.var_t2_dn4 = assign84160_e127716_d_n4;
        locals.var_t2_dn5 = assign84160_e127716_d_n5;
        locals.var_t2_dn6 = assign84160_e127716_d_n6;
        locals.var_t2_dn7 = assign84160_e127716_d_n7;
        locals.var_t2_dn8 = assign84160_e127716_d_n8;
        locals.var_t2_dn9 = assign84160_e127716_d_n9;
        locals.var_t2_dn10 = assign84160_e127716_d_n10;
        locals.var_t2_dn13 = assign84160_e127716_d_n13;

        let (assign84170_e127738, assign84170_e127738_d_n0, assign84170_e127738_d_n2, assign84170_e127738_d_n4, assign84170_e127738_d_n5, assign84170_e127738_d_n6, assign84170_e127738_d_n7, assign84170_e127738_d_n8, assign84170_e127738_d_n9, assign84170_e127738_d_n10, assign84170_e127738_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1946 != 0.0)) {
        let assign84170_e127725: f64 = (locals.var_t2 * locals.var_t2);
        let assign84170_e127729: f64 = (p.p334 * 0.01);
        let assign84170_e127730: f64 = (4.0 * assign84170_e127729);
        let assign84170_e127733: f64 = (p.p334 * 0.01);
        let assign84170_e127734: f64 = (assign84170_e127730 * assign84170_e127733);
        let assign84170_e127735: f64 = (assign84170_e127725 + assign84170_e127734);
        let assign84170_e127736: f64 = (assign84170_e127735).sqrt();
        (assign84170_e127736, (((locals.var_t2_dn0 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn0)) / (2.0 * assign84170_e127736)), (((locals.var_t2_dn2 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn2)) / (2.0 * assign84170_e127736)), (((locals.var_t2_dn4 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn4)) / (2.0 * assign84170_e127736)), (((locals.var_t2_dn5 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn5)) / (2.0 * assign84170_e127736)), (((locals.var_t2_dn6 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn6)) / (2.0 * assign84170_e127736)), (((locals.var_t2_dn7 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn7)) / (2.0 * assign84170_e127736)), (((locals.var_t2_dn8 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn8)) / (2.0 * assign84170_e127736)), (((locals.var_t2_dn9 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn9)) / (2.0 * assign84170_e127736)), (((locals.var_t2_dn10 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn10)) / (2.0 * assign84170_e127736)), (((locals.var_t2_dn13 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn13)) / (2.0 * assign84170_e127736)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign84170_e127738;
        locals.var_tmf2_dn0 = assign84170_e127738_d_n0;
        locals.var_tmf2_dn2 = assign84170_e127738_d_n2;
        locals.var_tmf2_dn4 = assign84170_e127738_d_n4;
        locals.var_tmf2_dn5 = assign84170_e127738_d_n5;
        locals.var_tmf2_dn6 = assign84170_e127738_d_n6;
        locals.var_tmf2_dn7 = assign84170_e127738_d_n7;
        locals.var_tmf2_dn8 = assign84170_e127738_d_n8;
        locals.var_tmf2_dn9 = assign84170_e127738_d_n9;
        locals.var_tmf2_dn10 = assign84170_e127738_d_n10;
        locals.var_tmf2_dn13 = assign84170_e127738_d_n13;

        let (assign84180_e127753, assign84180_e127753_d_n0, assign84180_e127753_d_n2, assign84180_e127753_d_n4, assign84180_e127753_d_n5, assign84180_e127753_d_n6, assign84180_e127753_d_n7, assign84180_e127753_d_n8, assign84180_e127753_d_n9, assign84180_e127753_d_n10, assign84180_e127753_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1946 != 0.0)) {
        let assign84180_e127749: f64 = (locals.var_t2 / locals.var_tmf2);
        let assign84180_e127750: f64 = (1.0 + assign84180_e127749);
        let assign84180_e127751: f64 = (0.5 * assign84180_e127750);
        (assign84180_e127751, (0.5 * (((locals.var_t2_dn0 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn2 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn4 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn5 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn6 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn7 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn8 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn9 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn10 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn13 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn13)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn13,)
    }
};
        locals.var_t9 = assign84180_e127753;
        locals.var_t9_dn0 = assign84180_e127753_d_n0;
        locals.var_t9_dn2 = assign84180_e127753_d_n2;
        locals.var_t9_dn4 = assign84180_e127753_d_n4;
        locals.var_t9_dn5 = assign84180_e127753_d_n5;
        locals.var_t9_dn6 = assign84180_e127753_d_n6;
        locals.var_t9_dn7 = assign84180_e127753_d_n7;
        locals.var_t9_dn8 = assign84180_e127753_d_n8;
        locals.var_t9_dn9 = assign84180_e127753_d_n9;
        locals.var_t9_dn10 = assign84180_e127753_d_n10;
        locals.var_t9_dn13 = assign84180_e127753_d_n13;

        let (assign84190_e127766, assign84190_e127766_d_n0, assign84190_e127766_d_n2, assign84190_e127766_d_n4, assign84190_e127766_d_n5, assign84190_e127766_d_n6, assign84190_e127766_d_n7, assign84190_e127766_d_n8, assign84190_e127766_d_n9, assign84190_e127766_d_n10, assign84190_e127766_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1946 != 0.0)) {
        let assign84190_e127763: f64 = (locals.var_t2 + locals.var_tmf2);
        let assign84190_e127764: f64 = (0.5 * assign84190_e127763);
        (assign84190_e127764, (0.5 * (locals.var_t2_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_t2_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_t2_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_t2_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_t2_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_t2_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_t2_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_t2_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_t2_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_t2_dn13 + locals.var_tmf2_dn13)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign84190_e127766;
        locals.var_t2_dn0 = assign84190_e127766_d_n0;
        locals.var_t2_dn2 = assign84190_e127766_d_n2;
        locals.var_t2_dn4 = assign84190_e127766_d_n4;
        locals.var_t2_dn5 = assign84190_e127766_d_n5;
        locals.var_t2_dn6 = assign84190_e127766_d_n6;
        locals.var_t2_dn7 = assign84190_e127766_d_n7;
        locals.var_t2_dn8 = assign84190_e127766_d_n8;
        locals.var_t2_dn9 = assign84190_e127766_d_n9;
        locals.var_t2_dn10 = assign84190_e127766_d_n10;
        locals.var_t2_dn13 = assign84190_e127766_d_n13;

        let assign84200_e127769: f64 = if locals.var_t2 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1949 = assign84200_e127769;

        let (assign84210_e127780, assign84210_e127780_d_n0, assign84210_e127780_d_n2, assign84210_e127780_d_n4, assign84210_e127780_d_n5, assign84210_e127780_d_n6, assign84210_e127780_d_n7, assign84210_e127780_d_n8, assign84210_e127780_d_n9, assign84210_e127780_d_n10, assign84210_e127780_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1946 != 0.0)) && (locals.var_guard1949 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign84210_e127780;
        locals.var_t2_dn0 = assign84210_e127780_d_n0;
        locals.var_t2_dn2 = assign84210_e127780_d_n2;
        locals.var_t2_dn4 = assign84210_e127780_d_n4;
        locals.var_t2_dn5 = assign84210_e127780_d_n5;
        locals.var_t2_dn6 = assign84210_e127780_d_n6;
        locals.var_t2_dn7 = assign84210_e127780_d_n7;
        locals.var_t2_dn8 = assign84210_e127780_d_n8;
        locals.var_t2_dn9 = assign84210_e127780_d_n9;
        locals.var_t2_dn10 = assign84210_e127780_d_n10;
        locals.var_t2_dn13 = assign84210_e127780_d_n13;

        let (assign84220_e127791, assign84220_e127791_d_n0, assign84220_e127791_d_n2, assign84220_e127791_d_n4, assign84220_e127791_d_n5, assign84220_e127791_d_n6, assign84220_e127791_d_n7, assign84220_e127791_d_n8, assign84220_e127791_d_n9, assign84220_e127791_d_n10, assign84220_e127791_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1946 != 0.0)) && (locals.var_guard1949 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn13,)
    }
};
        locals.var_t9 = assign84220_e127791;
        locals.var_t9_dn0 = assign84220_e127791_d_n0;
        locals.var_t9_dn2 = assign84220_e127791_d_n2;
        locals.var_t9_dn4 = assign84220_e127791_d_n4;
        locals.var_t9_dn5 = assign84220_e127791_d_n5;
        locals.var_t9_dn6 = assign84220_e127791_d_n6;
        locals.var_t9_dn7 = assign84220_e127791_d_n7;
        locals.var_t9_dn8 = assign84220_e127791_d_n8;
        locals.var_t9_dn9 = assign84220_e127791_d_n9;
        locals.var_t9_dn10 = assign84220_e127791_d_n10;
        locals.var_t9_dn13 = assign84220_e127791_d_n13;

        let (assign84230_e127800, assign84230_e127800_d_n0, assign84230_e127800_d_n2, assign84230_e127800_d_n4, assign84230_e127800_d_n5, assign84230_e127800_d_n6, assign84230_e127800_d_n7, assign84230_e127800_d_n8, assign84230_e127800_d_n9, assign84230_e127800_d_n10, assign84230_e127800_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1946 != 0.0)) {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    } else {
        (locals.var_ddriftldc, locals.var_ddriftldc_dn0, locals.var_ddriftldc_dn2, locals.var_ddriftldc_dn4, locals.var_ddriftldc_dn5, locals.var_ddriftldc_dn6, locals.var_ddriftldc_dn7, locals.var_ddriftldc_dn8, locals.var_ddriftldc_dn9, locals.var_ddriftldc_dn10, locals.var_ddriftldc_dn13,)
    }
};
        locals.var_ddriftldc = assign84230_e127800;
        locals.var_ddriftldc_dn0 = assign84230_e127800_d_n0;
        locals.var_ddriftldc_dn2 = assign84230_e127800_d_n2;
        locals.var_ddriftldc_dn4 = assign84230_e127800_d_n4;
        locals.var_ddriftldc_dn5 = assign84230_e127800_d_n5;
        locals.var_ddriftldc_dn6 = assign84230_e127800_d_n6;
        locals.var_ddriftldc_dn7 = assign84230_e127800_d_n7;
        locals.var_ddriftldc_dn8 = assign84230_e127800_d_n8;
        locals.var_ddriftldc_dn9 = assign84230_e127800_d_n9;
        locals.var_ddriftldc_dn10 = assign84230_e127800_d_n10;
        locals.var_ddriftldc_dn13 = assign84230_e127800_d_n13;

        let (assign84240_e127817, assign84240_e127817_d_n0, assign84240_e127817_d_n2, assign84240_e127817_d_n4, assign84240_e127817_d_n5, assign84240_e127817_d_n6, assign84240_e127817_d_n7, assign84240_e127817_d_n8, assign84240_e127817_d_n9, assign84240_e127817_d_n10, assign84240_e127817_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1946 != 0.0)) {
        let assign84240_e127809: f64 = (locals.var_q_nsubld__blk1883 * locals.var_ddriftldc);
        let assign84240_e127811: f64 = (assign84240_e127809 * locals.var_ddriftldc);
        let assign84240_e127813: f64 = (assign84240_e127811 / 2.0);
        let assign84240_e127815: f64 = (assign84240_e127813 / 1.034943e-10);
        (assign84240_e127815, (((((locals.var_q_nsubld__blk1883 * locals.var_ddriftldc_dn0) * locals.var_ddriftldc) + (assign84240_e127809 * locals.var_ddriftldc_dn0)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk1883 * locals.var_ddriftldc_dn2) * locals.var_ddriftldc) + (assign84240_e127809 * locals.var_ddriftldc_dn2)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk1883 * locals.var_ddriftldc_dn4) * locals.var_ddriftldc) + (assign84240_e127809 * locals.var_ddriftldc_dn4)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk1883 * locals.var_ddriftldc_dn5) * locals.var_ddriftldc) + (assign84240_e127809 * locals.var_ddriftldc_dn5)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk1883 * locals.var_ddriftldc_dn6) * locals.var_ddriftldc) + (assign84240_e127809 * locals.var_ddriftldc_dn6)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk1883 * locals.var_ddriftldc_dn7) * locals.var_ddriftldc) + (assign84240_e127809 * locals.var_ddriftldc_dn7)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk1883 * locals.var_ddriftldc_dn8) * locals.var_ddriftldc) + (assign84240_e127809 * locals.var_ddriftldc_dn8)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk1883 * locals.var_ddriftldc_dn9) * locals.var_ddriftldc) + (assign84240_e127809 * locals.var_ddriftldc_dn9)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk1883 * locals.var_ddriftldc_dn10) * locals.var_ddriftldc) + (assign84240_e127809 * locals.var_ddriftldc_dn10)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk1883 * locals.var_ddriftldc_dn13) * locals.var_ddriftldc) + (assign84240_e127809 * locals.var_ddriftldc_dn13)) / 2.0) / 1.034943e-10),)
    } else {
        (locals.var_dphi_sb, locals.var_dphi_sb_dn0, locals.var_dphi_sb_dn2, locals.var_dphi_sb_dn4, locals.var_dphi_sb_dn5, locals.var_dphi_sb_dn6, locals.var_dphi_sb_dn7, locals.var_dphi_sb_dn8, locals.var_dphi_sb_dn9, locals.var_dphi_sb_dn10, locals.var_dphi_sb_dn13,)
    }
};
        locals.var_dphi_sb = assign84240_e127817;
        locals.var_dphi_sb_dn0 = assign84240_e127817_d_n0;
        locals.var_dphi_sb_dn2 = assign84240_e127817_d_n2;
        locals.var_dphi_sb_dn4 = assign84240_e127817_d_n4;
        locals.var_dphi_sb_dn5 = assign84240_e127817_d_n5;
        locals.var_dphi_sb_dn6 = assign84240_e127817_d_n6;
        locals.var_dphi_sb_dn7 = assign84240_e127817_d_n7;
        locals.var_dphi_sb_dn8 = assign84240_e127817_d_n8;
        locals.var_dphi_sb_dn9 = assign84240_e127817_d_n9;
        locals.var_dphi_sb_dn10 = assign84240_e127817_d_n10;
        locals.var_dphi_sb_dn13 = assign84240_e127817_d_n13;

        let (assign84250_e127831, assign84250_e127831_d_n0, assign84250_e127831_d_n2, assign84250_e127831_d_n4, assign84250_e127831_d_n5, assign84250_e127831_d_n6, assign84250_e127831_d_n7, assign84250_e127831_d_n8, assign84250_e127831_d_n9, assign84250_e127831_d_n10, assign84250_e127831_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1946 != 0.0)) {
        let assign84250_e127826: f64 = (2.0 * locals.var_beta);
        let assign84250_e127828: f64 = (assign84250_e127826 * locals.var_dphi_sb);
        let assign84250_e127829: f64 = (assign84250_e127828).sqrt();
        (assign84250_e127829, ((((2.0 * locals.var_beta_dn0) * locals.var_dphi_sb) + (assign84250_e127826 * locals.var_dphi_sb_dn0)) / (2.0 * assign84250_e127829)), ((((2.0 * locals.var_beta_dn2) * locals.var_dphi_sb) + (assign84250_e127826 * locals.var_dphi_sb_dn2)) / (2.0 * assign84250_e127829)), ((((2.0 * locals.var_beta_dn4) * locals.var_dphi_sb) + (assign84250_e127826 * locals.var_dphi_sb_dn4)) / (2.0 * assign84250_e127829)), ((((2.0 * locals.var_beta_dn5) * locals.var_dphi_sb) + (assign84250_e127826 * locals.var_dphi_sb_dn5)) / (2.0 * assign84250_e127829)), ((((2.0 * locals.var_beta_dn6) * locals.var_dphi_sb) + (assign84250_e127826 * locals.var_dphi_sb_dn6)) / (2.0 * assign84250_e127829)), ((((2.0 * locals.var_beta_dn7) * locals.var_dphi_sb) + (assign84250_e127826 * locals.var_dphi_sb_dn7)) / (2.0 * assign84250_e127829)), ((((2.0 * locals.var_beta_dn8) * locals.var_dphi_sb) + (assign84250_e127826 * locals.var_dphi_sb_dn8)) / (2.0 * assign84250_e127829)), ((((2.0 * locals.var_beta_dn9) * locals.var_dphi_sb) + (assign84250_e127826 * locals.var_dphi_sb_dn9)) / (2.0 * assign84250_e127829)), ((((2.0 * locals.var_beta_dn10) * locals.var_dphi_sb) + (assign84250_e127826 * locals.var_dphi_sb_dn10)) / (2.0 * assign84250_e127829)), ((((2.0 * locals.var_beta_dn13) * locals.var_dphi_sb) + (assign84250_e127826 * locals.var_dphi_sb_dn13)) / (2.0 * assign84250_e127829)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign84250_e127831;
        locals.var_t0_dn0 = assign84250_e127831_d_n0;
        locals.var_t0_dn2 = assign84250_e127831_d_n2;
        locals.var_t0_dn4 = assign84250_e127831_d_n4;
        locals.var_t0_dn5 = assign84250_e127831_d_n5;
        locals.var_t0_dn6 = assign84250_e127831_d_n6;
        locals.var_t0_dn7 = assign84250_e127831_d_n7;
        locals.var_t0_dn8 = assign84250_e127831_d_n8;
        locals.var_t0_dn9 = assign84250_e127831_d_n9;
        locals.var_t0_dn10 = assign84250_e127831_d_n10;
        locals.var_t0_dn13 = assign84250_e127831_d_n13;

    }

    pub(super) fn stamp_transient_block_293(
        locals: &mut StampLocals,
    ) {
        let (assign84260_e127847, assign84260_e127847_d_n0, assign84260_e127847_d_n2, assign84260_e127847_d_n4, assign84260_e127847_d_n5, assign84260_e127847_d_n6, assign84260_e127847_d_n7, assign84260_e127847_d_n8, assign84260_e127847_d_n9, assign84260_e127847_d_n10, assign84260_e127847_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1946 != 0.0)) {
        let assign84260_e127839: f64 = { let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign84260_e127841: f64 = (-locals.var_t0);
        let assign84260_e127842: f64 = { let limited_exp_arg = assign84260_e127841; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign84260_e127843: f64 = (assign84260_e127839 + assign84260_e127842);
        let assign84260_e127845: f64 = (assign84260_e127843 / 2.0);
        (assign84260_e127845, ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn0) + ({ let limited_exp_arg = assign84260_e127841; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn0))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn2) + ({ let limited_exp_arg = assign84260_e127841; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn2))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn4) + ({ let limited_exp_arg = assign84260_e127841; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn4))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn5) + ({ let limited_exp_arg = assign84260_e127841; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn5))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn6) + ({ let limited_exp_arg = assign84260_e127841; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn6))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn7) + ({ let limited_exp_arg = assign84260_e127841; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn7))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn8) + ({ let limited_exp_arg = assign84260_e127841; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn8))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn9) + ({ let limited_exp_arg = assign84260_e127841; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn9))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn10) + ({ let limited_exp_arg = assign84260_e127841; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn10))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn13) + ({ let limited_exp_arg = assign84260_e127841; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn13))) / 2.0),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign84260_e127847;
        locals.var_t1_dn0 = assign84260_e127847_d_n0;
        locals.var_t1_dn2 = assign84260_e127847_d_n2;
        locals.var_t1_dn4 = assign84260_e127847_d_n4;
        locals.var_t1_dn5 = assign84260_e127847_d_n5;
        locals.var_t1_dn6 = assign84260_e127847_d_n6;
        locals.var_t1_dn7 = assign84260_e127847_d_n7;
        locals.var_t1_dn8 = assign84260_e127847_d_n8;
        locals.var_t1_dn9 = assign84260_e127847_d_n9;
        locals.var_t1_dn10 = assign84260_e127847_d_n10;
        locals.var_t1_dn13 = assign84260_e127847_d_n13;

        let (assign84270_e127859, assign84270_e127859_d_n0, assign84270_e127859_d_n2, assign84270_e127859_d_n4, assign84270_e127859_d_n5, assign84270_e127859_d_n6, assign84270_e127859_d_n7, assign84270_e127859_d_n8, assign84270_e127859_d_n9, assign84270_e127859_d_n10, assign84270_e127859_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1946 != 0.0)) {
        let assign84270_e127855: f64 = (locals.var_t1).ln();
        let assign84270_e127857: f64 = (assign84270_e127855 / locals.var_dphi_sb);
        (assign84270_e127857, ((((locals.var_t1_dn0 / locals.var_t1) * locals.var_dphi_sb) - (assign84270_e127855 * locals.var_dphi_sb_dn0)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn2 / locals.var_t1) * locals.var_dphi_sb) - (assign84270_e127855 * locals.var_dphi_sb_dn2)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn4 / locals.var_t1) * locals.var_dphi_sb) - (assign84270_e127855 * locals.var_dphi_sb_dn4)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn5 / locals.var_t1) * locals.var_dphi_sb) - (assign84270_e127855 * locals.var_dphi_sb_dn5)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn6 / locals.var_t1) * locals.var_dphi_sb) - (assign84270_e127855 * locals.var_dphi_sb_dn6)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn7 / locals.var_t1) * locals.var_dphi_sb) - (assign84270_e127855 * locals.var_dphi_sb_dn7)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn8 / locals.var_t1) * locals.var_dphi_sb) - (assign84270_e127855 * locals.var_dphi_sb_dn8)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn9 / locals.var_t1) * locals.var_dphi_sb) - (assign84270_e127855 * locals.var_dphi_sb_dn9)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn10 / locals.var_t1) * locals.var_dphi_sb) - (assign84270_e127855 * locals.var_dphi_sb_dn10)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn13 / locals.var_t1) * locals.var_dphi_sb) - (assign84270_e127855 * locals.var_dphi_sb_dn13)) / (locals.var_dphi_sb * locals.var_dphi_sb)),)
    } else {
        (locals.var_c_sb, locals.var_c_sb_dn0, locals.var_c_sb_dn2, locals.var_c_sb_dn4, locals.var_c_sb_dn5, locals.var_c_sb_dn6, locals.var_c_sb_dn7, locals.var_c_sb_dn8, locals.var_c_sb_dn9, locals.var_c_sb_dn10, locals.var_c_sb_dn13,)
    }
};
        locals.var_c_sb = assign84270_e127859;
        locals.var_c_sb_dn0 = assign84270_e127859_d_n0;
        locals.var_c_sb_dn2 = assign84270_e127859_d_n2;
        locals.var_c_sb_dn4 = assign84270_e127859_d_n4;
        locals.var_c_sb_dn5 = assign84270_e127859_d_n5;
        locals.var_c_sb_dn6 = assign84270_e127859_d_n6;
        locals.var_c_sb_dn7 = assign84270_e127859_d_n7;
        locals.var_c_sb_dn8 = assign84270_e127859_d_n8;
        locals.var_c_sb_dn9 = assign84270_e127859_d_n9;
        locals.var_c_sb_dn10 = assign84270_e127859_d_n10;
        locals.var_c_sb_dn13 = assign84270_e127859_d_n13;

        let (assign84280_e127868,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1946 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_lp_s0,)
    }
};
        locals.var_lp_s0 = assign84280_e127868;

    }

    pub(super) fn stamp_transient_block_294(
        locals: &mut StampLocals,
    ) {
        let mut assign84290_loop_guard: usize = 0;
        while {
            let assign84290_cond_e127878: f64 = (locals.var_lp_s0_max + 1.0);
            let assign84290_cond_e127880: f64 = if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1946 != 0.0)) && (locals.var_lp_s0 <= assign84290_cond_e127878)) { 1.0 } else { 0.0 };
            assign84290_cond_e127880 != 0.0
        } {
            assign84290_loop_guard += 1;
            assert!(assign84290_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign84290_body3_e127916, assign84290_body3_e127916_d_n0, assign84290_body3_e127916_d_n2, assign84290_body3_e127916_d_n4, assign84290_body3_e127916_d_n5, assign84290_body3_e127916_d_n6, assign84290_body3_e127916_d_n7, assign84290_body3_e127916_d_n8, assign84290_body3_e127916_d_n9, assign84290_body3_e127916_d_n10, assign84290_body3_e127916_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1946 != 0.0)) {
        let assign84290_body3_e127914: f64 = (locals.var_ps0ld + locals.var_vxbgmtcl);
        (assign84290_body3_e127914, (locals.var_ps0ld_dn0 + locals.var_vxbgmtcl_dn0), (locals.var_ps0ld_dn2 + locals.var_vxbgmtcl_dn2), (locals.var_ps0ld_dn4 + locals.var_vxbgmtcl_dn4), (locals.var_ps0ld_dn5 + locals.var_vxbgmtcl_dn5), (locals.var_ps0ld_dn6 + locals.var_vxbgmtcl_dn6), (locals.var_ps0ld_dn7 + locals.var_vxbgmtcl_dn7), (locals.var_ps0ld_dn8 + locals.var_vxbgmtcl_dn8), (locals.var_ps0ld_dn9 + locals.var_vxbgmtcl_dn9), (locals.var_ps0ld_dn10 + locals.var_vxbgmtcl_dn10), (locals.var_ps0ld_dn13 + locals.var_vxbgmtcl_dn13),)
    } else {
        (locals.var_ps0ld_vxb, locals.var_ps0ld_vxb_dn0, locals.var_ps0ld_vxb_dn2, locals.var_ps0ld_vxb_dn4, locals.var_ps0ld_vxb_dn5, locals.var_ps0ld_vxb_dn6, locals.var_ps0ld_vxb_dn7, locals.var_ps0ld_vxb_dn8, locals.var_ps0ld_vxb_dn9, locals.var_ps0ld_vxb_dn10, locals.var_ps0ld_vxb_dn13,)
    }
};
            locals.var_ps0ld_vxb = assign84290_body3_e127916;
            locals.var_ps0ld_vxb_dn0 = assign84290_body3_e127916_d_n0;
            locals.var_ps0ld_vxb_dn2 = assign84290_body3_e127916_d_n2;
            locals.var_ps0ld_vxb_dn4 = assign84290_body3_e127916_d_n4;
            locals.var_ps0ld_vxb_dn5 = assign84290_body3_e127916_d_n5;
            locals.var_ps0ld_vxb_dn6 = assign84290_body3_e127916_d_n6;
            locals.var_ps0ld_vxb_dn7 = assign84290_body3_e127916_d_n7;
            locals.var_ps0ld_vxb_dn8 = assign84290_body3_e127916_d_n8;
            locals.var_ps0ld_vxb_dn9 = assign84290_body3_e127916_d_n9;
            locals.var_ps0ld_vxb_dn10 = assign84290_body3_e127916_d_n10;
            locals.var_ps0ld_vxb_dn13 = assign84290_body3_e127916_d_n13;
            let (assign84290_body4_e127927, assign84290_body4_e127927_d_n0, assign84290_body4_e127927_d_n2, assign84290_body4_e127927_d_n4, assign84290_body4_e127927_d_n5, assign84290_body4_e127927_d_n6, assign84290_body4_e127927_d_n7, assign84290_body4_e127927_d_n8, assign84290_body4_e127927_d_n9, assign84290_body4_e127927_d_n10, assign84290_body4_e127927_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1946 != 0.0)) {
        let assign84290_body4_e127925: f64 = (locals.var_beta * locals.var_ps0ld_vxb);
        (assign84290_body4_e127925, ((locals.var_beta_dn0 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn0)), ((locals.var_beta_dn2 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn2)), ((locals.var_beta_dn4 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn4)), ((locals.var_beta_dn5 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn5)), ((locals.var_beta_dn6 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn6)), ((locals.var_beta_dn7 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn7)), ((locals.var_beta_dn8 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn8)), ((locals.var_beta_dn9 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn9)), ((locals.var_beta_dn10 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn10)), ((locals.var_beta_dn13 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn13)),)
    } else {
        (locals.var_chi, locals.var_chi_dn0, locals.var_chi_dn2, locals.var_chi_dn4, locals.var_chi_dn5, locals.var_chi_dn6, locals.var_chi_dn7, locals.var_chi_dn8, locals.var_chi_dn9, locals.var_chi_dn10, locals.var_chi_dn13,)
    }
};
            locals.var_chi = assign84290_body4_e127927;
            locals.var_chi_dn0 = assign84290_body4_e127927_d_n0;
            locals.var_chi_dn2 = assign84290_body4_e127927_d_n2;
            locals.var_chi_dn4 = assign84290_body4_e127927_d_n4;
            locals.var_chi_dn5 = assign84290_body4_e127927_d_n5;
            locals.var_chi_dn6 = assign84290_body4_e127927_d_n6;
            locals.var_chi_dn7 = assign84290_body4_e127927_d_n7;
            locals.var_chi_dn8 = assign84290_body4_e127927_d_n8;
            locals.var_chi_dn9 = assign84290_body4_e127927_d_n9;
            locals.var_chi_dn10 = assign84290_body4_e127927_d_n10;
            locals.var_chi_dn13 = assign84290_body4_e127927_d_n13;
            let (assign84290_body5_e127940, assign84290_body5_e127940_d_n0, assign84290_body5_e127940_d_n2, assign84290_body5_e127940_d_n4, assign84290_body5_e127940_d_n5, assign84290_body5_e127940_d_n6, assign84290_body5_e127940_d_n7, assign84290_body5_e127940_d_n8, assign84290_body5_e127940_d_n9, assign84290_body5_e127940_d_n10, assign84290_body5_e127940_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1946 != 0.0)) {
        let assign84290_body5_e127937: f64 = (locals.var_ps0ld_vxb - locals.var_dphi_sb);
        let assign84290_body5_e127938: f64 = (locals.var_c_sb * assign84290_body5_e127937);
        (assign84290_body5_e127938, ((locals.var_c_sb_dn0 * assign84290_body5_e127937) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn0 - locals.var_dphi_sb_dn0))), ((locals.var_c_sb_dn2 * assign84290_body5_e127937) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn2 - locals.var_dphi_sb_dn2))), ((locals.var_c_sb_dn4 * assign84290_body5_e127937) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn4 - locals.var_dphi_sb_dn4))), ((locals.var_c_sb_dn5 * assign84290_body5_e127937) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn5 - locals.var_dphi_sb_dn5))), ((locals.var_c_sb_dn6 * assign84290_body5_e127937) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn6 - locals.var_dphi_sb_dn6))), ((locals.var_c_sb_dn7 * assign84290_body5_e127937) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn7 - locals.var_dphi_sb_dn7))), ((locals.var_c_sb_dn8 * assign84290_body5_e127937) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn8 - locals.var_dphi_sb_dn8))), ((locals.var_c_sb_dn9 * assign84290_body5_e127937) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn9 - locals.var_dphi_sb_dn9))), ((locals.var_c_sb_dn10 * assign84290_body5_e127937) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn10 - locals.var_dphi_sb_dn10))), ((locals.var_c_sb_dn13 * assign84290_body5_e127937) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn13 - locals.var_dphi_sb_dn13))),)
    } else {
        (locals.var_ty, locals.var_ty_dn0, locals.var_ty_dn2, locals.var_ty_dn4, locals.var_ty_dn5, locals.var_ty_dn6, locals.var_ty_dn7, locals.var_ty_dn8, locals.var_ty_dn9, locals.var_ty_dn10, locals.var_ty_dn13,)
    }
};
            locals.var_ty = assign84290_body5_e127940;
            locals.var_ty_dn0 = assign84290_body5_e127940_d_n0;
            locals.var_ty_dn2 = assign84290_body5_e127940_d_n2;
            locals.var_ty_dn4 = assign84290_body5_e127940_d_n4;
            locals.var_ty_dn5 = assign84290_body5_e127940_d_n5;
            locals.var_ty_dn6 = assign84290_body5_e127940_d_n6;
            locals.var_ty_dn7 = assign84290_body5_e127940_d_n7;
            locals.var_ty_dn8 = assign84290_body5_e127940_d_n8;
            locals.var_ty_dn9 = assign84290_body5_e127940_d_n9;
            locals.var_ty_dn10 = assign84290_body5_e127940_d_n10;
            locals.var_ty_dn13 = assign84290_body5_e127940_d_n13;
            let assign84290_body6_e127943: f64 = if locals.var_ty < 60.0 { 1.0 } else { 0.0 };
            locals.var_guard1951 = assign84290_body6_e127943;
            let (assign84290_body7_e127955, assign84290_body7_e127955_d_n0, assign84290_body7_e127955_d_n2, assign84290_body7_e127955_d_n4, assign84290_body7_e127955_d_n5, assign84290_body7_e127955_d_n6, assign84290_body7_e127955_d_n7, assign84290_body7_e127955_d_n8, assign84290_body7_e127955_d_n9, assign84290_body7_e127955_d_n10, assign84290_body7_e127955_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1946 != 0.0)) && (locals.var_guard1951 != 0.0)) {
        let assign84290_body7_e127953: f64 = (locals.var_ty).exp();
        (assign84290_body7_e127953, (assign84290_body7_e127953 * locals.var_ty_dn0), (assign84290_body7_e127953 * locals.var_ty_dn2), (assign84290_body7_e127953 * locals.var_ty_dn4), (assign84290_body7_e127953 * locals.var_ty_dn5), (assign84290_body7_e127953 * locals.var_ty_dn6), (assign84290_body7_e127953 * locals.var_ty_dn7), (assign84290_body7_e127953 * locals.var_ty_dn8), (assign84290_body7_e127953 * locals.var_ty_dn9), (assign84290_body7_e127953 * locals.var_ty_dn10), (assign84290_body7_e127953 * locals.var_ty_dn13),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
            locals.var_t1 = assign84290_body7_e127955;
            locals.var_t1_dn0 = assign84290_body7_e127955_d_n0;
            locals.var_t1_dn2 = assign84290_body7_e127955_d_n2;
            locals.var_t1_dn4 = assign84290_body7_e127955_d_n4;
            locals.var_t1_dn5 = assign84290_body7_e127955_d_n5;
            locals.var_t1_dn6 = assign84290_body7_e127955_d_n6;
            locals.var_t1_dn7 = assign84290_body7_e127955_d_n7;
            locals.var_t1_dn8 = assign84290_body7_e127955_d_n8;
            locals.var_t1_dn9 = assign84290_body7_e127955_d_n9;
            locals.var_t1_dn10 = assign84290_body7_e127955_d_n10;
            locals.var_t1_dn13 = assign84290_body7_e127955_d_n13;
            let (assign84290_body8_e127970, assign84290_body8_e127970_d_n0, assign84290_body8_e127970_d_n2, assign84290_body8_e127970_d_n4, assign84290_body8_e127970_d_n5, assign84290_body8_e127970_d_n6, assign84290_body8_e127970_d_n7, assign84290_body8_e127970_d_n8, assign84290_body8_e127970_d_n9, assign84290_body8_e127970_d_n10, assign84290_body8_e127970_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1946 != 0.0)) && (locals.var_guard1951 != 0.0)) {
        let assign84290_body8_e127965: f64 = (-locals.var_c_sb);
        let assign84290_body8_e127967: f64 = (assign84290_body8_e127965 * locals.var_dphi_sb);
        let assign84290_body8_e127968: f64 = (assign84290_body8_e127967).exp();
        (assign84290_body8_e127968, (assign84290_body8_e127968 * (((-locals.var_c_sb_dn0) * locals.var_dphi_sb) + (assign84290_body8_e127965 * locals.var_dphi_sb_dn0))), (assign84290_body8_e127968 * (((-locals.var_c_sb_dn2) * locals.var_dphi_sb) + (assign84290_body8_e127965 * locals.var_dphi_sb_dn2))), (assign84290_body8_e127968 * (((-locals.var_c_sb_dn4) * locals.var_dphi_sb) + (assign84290_body8_e127965 * locals.var_dphi_sb_dn4))), (assign84290_body8_e127968 * (((-locals.var_c_sb_dn5) * locals.var_dphi_sb) + (assign84290_body8_e127965 * locals.var_dphi_sb_dn5))), (assign84290_body8_e127968 * (((-locals.var_c_sb_dn6) * locals.var_dphi_sb) + (assign84290_body8_e127965 * locals.var_dphi_sb_dn6))), (assign84290_body8_e127968 * (((-locals.var_c_sb_dn7) * locals.var_dphi_sb) + (assign84290_body8_e127965 * locals.var_dphi_sb_dn7))), (assign84290_body8_e127968 * (((-locals.var_c_sb_dn8) * locals.var_dphi_sb) + (assign84290_body8_e127965 * locals.var_dphi_sb_dn8))), (assign84290_body8_e127968 * (((-locals.var_c_sb_dn9) * locals.var_dphi_sb) + (assign84290_body8_e127965 * locals.var_dphi_sb_dn9))), (assign84290_body8_e127968 * (((-locals.var_c_sb_dn10) * locals.var_dphi_sb) + (assign84290_body8_e127965 * locals.var_dphi_sb_dn10))), (assign84290_body8_e127968 * (((-locals.var_c_sb_dn13) * locals.var_dphi_sb) + (assign84290_body8_e127965 * locals.var_dphi_sb_dn13))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
            locals.var_t0 = assign84290_body8_e127970;
            locals.var_t0_dn0 = assign84290_body8_e127970_d_n0;
            locals.var_t0_dn2 = assign84290_body8_e127970_d_n2;
            locals.var_t0_dn4 = assign84290_body8_e127970_d_n4;
            locals.var_t0_dn5 = assign84290_body8_e127970_d_n5;
            locals.var_t0_dn6 = assign84290_body8_e127970_d_n6;
            locals.var_t0_dn7 = assign84290_body8_e127970_d_n7;
            locals.var_t0_dn8 = assign84290_body8_e127970_d_n8;
            locals.var_t0_dn9 = assign84290_body8_e127970_d_n9;
            locals.var_t0_dn10 = assign84290_body8_e127970_d_n10;
            locals.var_t0_dn13 = assign84290_body8_e127970_d_n13;
            let (assign84290_body9_e127983, assign84290_body9_e127983_d_n0, assign84290_body9_e127983_d_n2, assign84290_body9_e127983_d_n4, assign84290_body9_e127983_d_n5, assign84290_body9_e127983_d_n6, assign84290_body9_e127983_d_n7, assign84290_body9_e127983_d_n8, assign84290_body9_e127983_d_n9, assign84290_body9_e127983_d_n10, assign84290_body9_e127983_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1946 != 0.0)) && (locals.var_guard1951 != 0.0)) {
        let assign84290_body9_e127981: f64 = (locals.var_t1 - locals.var_t0);
        (assign84290_body9_e127981, (locals.var_t1_dn0 - locals.var_t0_dn0), (locals.var_t1_dn2 - locals.var_t0_dn2), (locals.var_t1_dn4 - locals.var_t0_dn4), (locals.var_t1_dn5 - locals.var_t0_dn5), (locals.var_t1_dn6 - locals.var_t0_dn6), (locals.var_t1_dn7 - locals.var_t0_dn7), (locals.var_t1_dn8 - locals.var_t0_dn8), (locals.var_t1_dn9 - locals.var_t0_dn9), (locals.var_t1_dn10 - locals.var_t0_dn10), (locals.var_t1_dn13 - locals.var_t0_dn13),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
            locals.var_t2 = assign84290_body9_e127983;
            locals.var_t2_dn0 = assign84290_body9_e127983_d_n0;
            locals.var_t2_dn2 = assign84290_body9_e127983_d_n2;
            locals.var_t2_dn4 = assign84290_body9_e127983_d_n4;
            locals.var_t2_dn5 = assign84290_body9_e127983_d_n5;
            locals.var_t2_dn6 = assign84290_body9_e127983_d_n6;
            locals.var_t2_dn7 = assign84290_body9_e127983_d_n7;
            locals.var_t2_dn8 = assign84290_body9_e127983_d_n8;
            locals.var_t2_dn9 = assign84290_body9_e127983_d_n9;
            locals.var_t2_dn10 = assign84290_body9_e127983_d_n10;
            locals.var_t2_dn13 = assign84290_body9_e127983_d_n13;
            let (assign84290_body10_e127999, assign84290_body10_e127999_d_n0, assign84290_body10_e127999_d_n2, assign84290_body10_e127999_d_n4, assign84290_body10_e127999_d_n5, assign84290_body10_e127999_d_n6, assign84290_body10_e127999_d_n7, assign84290_body10_e127999_d_n8, assign84290_body10_e127999_d_n9, assign84290_body10_e127999_d_n10, assign84290_body10_e127999_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1946 != 0.0)) && (locals.var_guard1951 != 0.0)) {
        let assign84290_body10_e127994: f64 = (1.0 + locals.var_t2);
        let assign84290_body10_e127995: f64 = (assign84290_body10_e127994).ln();
        let assign84290_body10_e127997: f64 = (assign84290_body10_e127995 / locals.var_c_sb);
        (assign84290_body10_e127997, ((((locals.var_t2_dn0 / assign84290_body10_e127994) * locals.var_c_sb) - (assign84290_body10_e127995 * locals.var_c_sb_dn0)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn2 / assign84290_body10_e127994) * locals.var_c_sb) - (assign84290_body10_e127995 * locals.var_c_sb_dn2)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn4 / assign84290_body10_e127994) * locals.var_c_sb) - (assign84290_body10_e127995 * locals.var_c_sb_dn4)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn5 / assign84290_body10_e127994) * locals.var_c_sb) - (assign84290_body10_e127995 * locals.var_c_sb_dn5)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn6 / assign84290_body10_e127994) * locals.var_c_sb) - (assign84290_body10_e127995 * locals.var_c_sb_dn6)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn7 / assign84290_body10_e127994) * locals.var_c_sb) - (assign84290_body10_e127995 * locals.var_c_sb_dn7)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn8 / assign84290_body10_e127994) * locals.var_c_sb) - (assign84290_body10_e127995 * locals.var_c_sb_dn8)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn9 / assign84290_body10_e127994) * locals.var_c_sb) - (assign84290_body10_e127995 * locals.var_c_sb_dn9)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn10 / assign84290_body10_e127994) * locals.var_c_sb) - (assign84290_body10_e127995 * locals.var_c_sb_dn10)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn13 / assign84290_body10_e127994) * locals.var_c_sb) - (assign84290_body10_e127995 * locals.var_c_sb_dn13)) / (locals.var_c_sb * locals.var_c_sb)),)
    } else {
        (locals.var_phi_b, locals.var_phi_b_dn0, locals.var_phi_b_dn2, locals.var_phi_b_dn4, locals.var_phi_b_dn5, locals.var_phi_b_dn6, locals.var_phi_b_dn7, locals.var_phi_b_dn8, locals.var_phi_b_dn9, locals.var_phi_b_dn10, locals.var_phi_b_dn13,)
    }
};
            locals.var_phi_b = assign84290_body10_e127999;
            locals.var_phi_b_dn0 = assign84290_body10_e127999_d_n0;
            locals.var_phi_b_dn2 = assign84290_body10_e127999_d_n2;
            locals.var_phi_b_dn4 = assign84290_body10_e127999_d_n4;
            locals.var_phi_b_dn5 = assign84290_body10_e127999_d_n5;
            locals.var_phi_b_dn6 = assign84290_body10_e127999_d_n6;
            locals.var_phi_b_dn7 = assign84290_body10_e127999_d_n7;
            locals.var_phi_b_dn8 = assign84290_body10_e127999_d_n8;
            locals.var_phi_b_dn9 = assign84290_body10_e127999_d_n9;
            locals.var_phi_b_dn10 = assign84290_body10_e127999_d_n10;
            locals.var_phi_b_dn13 = assign84290_body10_e127999_d_n13;
            let (assign84290_body11_e128014, assign84290_body11_e128014_d_n0, assign84290_body11_e128014_d_n2, assign84290_body11_e128014_d_n4, assign84290_body11_e128014_d_n5, assign84290_body11_e128014_d_n6, assign84290_body11_e128014_d_n7, assign84290_body11_e128014_d_n8, assign84290_body11_e128014_d_n9, assign84290_body11_e128014_d_n10, assign84290_body11_e128014_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1946 != 0.0)) && (locals.var_guard1951 != 0.0)) {
        let assign84290_body11_e128011: f64 = (1.0 + locals.var_t2);
        let assign84290_body11_e128012: f64 = (locals.var_t1 / assign84290_body11_e128011);
        (assign84290_body11_e128012, (((locals.var_t1_dn0 * assign84290_body11_e128011) - (locals.var_t1 * locals.var_t2_dn0)) / (assign84290_body11_e128011 * assign84290_body11_e128011)), (((locals.var_t1_dn2 * assign84290_body11_e128011) - (locals.var_t1 * locals.var_t2_dn2)) / (assign84290_body11_e128011 * assign84290_body11_e128011)), (((locals.var_t1_dn4 * assign84290_body11_e128011) - (locals.var_t1 * locals.var_t2_dn4)) / (assign84290_body11_e128011 * assign84290_body11_e128011)), (((locals.var_t1_dn5 * assign84290_body11_e128011) - (locals.var_t1 * locals.var_t2_dn5)) / (assign84290_body11_e128011 * assign84290_body11_e128011)), (((locals.var_t1_dn6 * assign84290_body11_e128011) - (locals.var_t1 * locals.var_t2_dn6)) / (assign84290_body11_e128011 * assign84290_body11_e128011)), (((locals.var_t1_dn7 * assign84290_body11_e128011) - (locals.var_t1 * locals.var_t2_dn7)) / (assign84290_body11_e128011 * assign84290_body11_e128011)), (((locals.var_t1_dn8 * assign84290_body11_e128011) - (locals.var_t1 * locals.var_t2_dn8)) / (assign84290_body11_e128011 * assign84290_body11_e128011)), (((locals.var_t1_dn9 * assign84290_body11_e128011) - (locals.var_t1 * locals.var_t2_dn9)) / (assign84290_body11_e128011 * assign84290_body11_e128011)), (((locals.var_t1_dn10 * assign84290_body11_e128011) - (locals.var_t1 * locals.var_t2_dn10)) / (assign84290_body11_e128011 * assign84290_body11_e128011)), (((locals.var_t1_dn13 * assign84290_body11_e128011) - (locals.var_t1 * locals.var_t2_dn13)) / (assign84290_body11_e128011 * assign84290_body11_e128011)),)
    } else {
        (locals.var_phi_b_dpss, locals.var_phi_b_dpss_dn0, locals.var_phi_b_dpss_dn2, locals.var_phi_b_dpss_dn4, locals.var_phi_b_dpss_dn5, locals.var_phi_b_dpss_dn6, locals.var_phi_b_dpss_dn7, locals.var_phi_b_dpss_dn8, locals.var_phi_b_dpss_dn9, locals.var_phi_b_dpss_dn10, locals.var_phi_b_dpss_dn13,)
    }
};
            locals.var_phi_b_dpss = assign84290_body11_e128014;
            locals.var_phi_b_dpss_dn0 = assign84290_body11_e128014_d_n0;
            locals.var_phi_b_dpss_dn2 = assign84290_body11_e128014_d_n2;
            locals.var_phi_b_dpss_dn4 = assign84290_body11_e128014_d_n4;
            locals.var_phi_b_dpss_dn5 = assign84290_body11_e128014_d_n5;
            locals.var_phi_b_dpss_dn6 = assign84290_body11_e128014_d_n6;
            locals.var_phi_b_dpss_dn7 = assign84290_body11_e128014_d_n7;
            locals.var_phi_b_dpss_dn8 = assign84290_body11_e128014_d_n8;
            locals.var_phi_b_dpss_dn9 = assign84290_body11_e128014_d_n9;
            locals.var_phi_b_dpss_dn10 = assign84290_body11_e128014_d_n10;
            locals.var_phi_b_dpss_dn13 = assign84290_body11_e128014_d_n13;
            let (assign84290_body13_e128042, assign84290_body13_e128042_d_n0, assign84290_body13_e128042_d_n2, assign84290_body13_e128042_d_n4, assign84290_body13_e128042_d_n5, assign84290_body13_e128042_d_n6, assign84290_body13_e128042_d_n7, assign84290_body13_e128042_d_n8, assign84290_body13_e128042_d_n9, assign84290_body13_e128042_d_n10, assign84290_body13_e128042_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1946 != 0.0)) && (locals.var_guard1951 == 0.0)) {
        let assign84290_body13_e128040: f64 = (locals.var_ps0ld_vxb - locals.var_dphi_sb);
        (assign84290_body13_e128040, (locals.var_ps0ld_vxb_dn0 - locals.var_dphi_sb_dn0), (locals.var_ps0ld_vxb_dn2 - locals.var_dphi_sb_dn2), (locals.var_ps0ld_vxb_dn4 - locals.var_dphi_sb_dn4), (locals.var_ps0ld_vxb_dn5 - locals.var_dphi_sb_dn5), (locals.var_ps0ld_vxb_dn6 - locals.var_dphi_sb_dn6), (locals.var_ps0ld_vxb_dn7 - locals.var_dphi_sb_dn7), (locals.var_ps0ld_vxb_dn8 - locals.var_dphi_sb_dn8), (locals.var_ps0ld_vxb_dn9 - locals.var_dphi_sb_dn9), (locals.var_ps0ld_vxb_dn10 - locals.var_dphi_sb_dn10), (locals.var_ps0ld_vxb_dn13 - locals.var_dphi_sb_dn13),)
    } else {
        (locals.var_phi_b, locals.var_phi_b_dn0, locals.var_phi_b_dn2, locals.var_phi_b_dn4, locals.var_phi_b_dn5, locals.var_phi_b_dn6, locals.var_phi_b_dn7, locals.var_phi_b_dn8, locals.var_phi_b_dn9, locals.var_phi_b_dn10, locals.var_phi_b_dn13,)
    }
};
            locals.var_phi_b = assign84290_body13_e128042;
            locals.var_phi_b_dn0 = assign84290_body13_e128042_d_n0;
            locals.var_phi_b_dn2 = assign84290_body13_e128042_d_n2;
            locals.var_phi_b_dn4 = assign84290_body13_e128042_d_n4;
            locals.var_phi_b_dn5 = assign84290_body13_e128042_d_n5;
            locals.var_phi_b_dn6 = assign84290_body13_e128042_d_n6;
            locals.var_phi_b_dn7 = assign84290_body13_e128042_d_n7;
            locals.var_phi_b_dn8 = assign84290_body13_e128042_d_n8;
            locals.var_phi_b_dn9 = assign84290_body13_e128042_d_n9;
            locals.var_phi_b_dn10 = assign84290_body13_e128042_d_n10;
            locals.var_phi_b_dn13 = assign84290_body13_e128042_d_n13;
            let (assign84290_body14_e128054, assign84290_body14_e128054_d_n0, assign84290_body14_e128054_d_n2, assign84290_body14_e128054_d_n4, assign84290_body14_e128054_d_n5, assign84290_body14_e128054_d_n6, assign84290_body14_e128054_d_n7, assign84290_body14_e128054_d_n8, assign84290_body14_e128054_d_n9, assign84290_body14_e128054_d_n10, assign84290_body14_e128054_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1946 != 0.0)) && (locals.var_guard1951 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_phi_b_dpss, locals.var_phi_b_dpss_dn0, locals.var_phi_b_dpss_dn2, locals.var_phi_b_dpss_dn4, locals.var_phi_b_dpss_dn5, locals.var_phi_b_dpss_dn6, locals.var_phi_b_dpss_dn7, locals.var_phi_b_dpss_dn8, locals.var_phi_b_dpss_dn9, locals.var_phi_b_dpss_dn10, locals.var_phi_b_dpss_dn13,)
    }
};
            locals.var_phi_b_dpss = assign84290_body14_e128054;
            locals.var_phi_b_dpss_dn0 = assign84290_body14_e128054_d_n0;
            locals.var_phi_b_dpss_dn2 = assign84290_body14_e128054_d_n2;
            locals.var_phi_b_dpss_dn4 = assign84290_body14_e128054_d_n4;
            locals.var_phi_b_dpss_dn5 = assign84290_body14_e128054_d_n5;
            locals.var_phi_b_dpss_dn6 = assign84290_body14_e128054_d_n6;
            locals.var_phi_b_dpss_dn7 = assign84290_body14_e128054_d_n7;
            locals.var_phi_b_dpss_dn8 = assign84290_body14_e128054_d_n8;
            locals.var_phi_b_dpss_dn9 = assign84290_body14_e128054_d_n9;
            locals.var_phi_b_dpss_dn10 = assign84290_body14_e128054_d_n10;
            locals.var_phi_b_dpss_dn13 = assign84290_body14_e128054_d_n13;
            let (assign84290_body15_e128065, assign84290_body15_e128065_d_n0, assign84290_body15_e128065_d_n2, assign84290_body15_e128065_d_n4, assign84290_body15_e128065_d_n5, assign84290_body15_e128065_d_n6, assign84290_body15_e128065_d_n7, assign84290_body15_e128065_d_n8, assign84290_body15_e128065_d_n9, assign84290_body15_e128065_d_n10, assign84290_body15_e128065_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1946 != 0.0)) {
        let assign84290_body15_e128063: f64 = (locals.var_beta * locals.var_phi_b);
        (assign84290_body15_e128063, ((locals.var_beta_dn0 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn0)), ((locals.var_beta_dn2 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn2)), ((locals.var_beta_dn4 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn4)), ((locals.var_beta_dn5 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn5)), ((locals.var_beta_dn6 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn6)), ((locals.var_beta_dn7 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn7)), ((locals.var_beta_dn8 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn8)), ((locals.var_beta_dn9 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn9)), ((locals.var_beta_dn10 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn10)), ((locals.var_beta_dn13 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn13)),)
    } else {
        (locals.var_chib, locals.var_chib_dn0, locals.var_chib_dn2, locals.var_chib_dn4, locals.var_chib_dn5, locals.var_chib_dn6, locals.var_chib_dn7, locals.var_chib_dn8, locals.var_chib_dn9, locals.var_chib_dn10, locals.var_chib_dn13,)
    }
};
            locals.var_chib = assign84290_body15_e128065;
            locals.var_chib_dn0 = assign84290_body15_e128065_d_n0;
            locals.var_chib_dn2 = assign84290_body15_e128065_d_n2;
            locals.var_chib_dn4 = assign84290_body15_e128065_d_n4;
            locals.var_chib_dn5 = assign84290_body15_e128065_d_n5;
            locals.var_chib_dn6 = assign84290_body15_e128065_d_n6;
            locals.var_chib_dn7 = assign84290_body15_e128065_d_n7;
            locals.var_chib_dn8 = assign84290_body15_e128065_d_n8;
            locals.var_chib_dn9 = assign84290_body15_e128065_d_n9;
            locals.var_chib_dn10 = assign84290_body15_e128065_d_n10;
            locals.var_chib_dn13 = assign84290_body15_e128065_d_n13;
            let assign84290_body16_e128068: f64 = if locals.var_chi < 0.0 { 1.0 } else { 0.0 };
            locals.var_guard1952 = assign84290_body16_e128068;
            let (assign84290_body18_e128093, assign84290_body18_e128093_d_n0, assign84290_body18_e128093_d_n2, assign84290_body18_e128093_d_n4, assign84290_body18_e128093_d_n5, assign84290_body18_e128093_d_n6, assign84290_body18_e128093_d_n7, assign84290_body18_e128093_d_n8, assign84290_body18_e128093_d_n9, assign84290_body18_e128093_d_n10, assign84290_body18_e128093_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1946 != 0.0)) && (locals.var_guard1952 != 0.0)) {
        let assign84290_body18_e128091: f64 = (-0.7071067811865475);
        (assign84290_body18_e128091, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
            locals.var_t0 = assign84290_body18_e128093;
            locals.var_t0_dn0 = assign84290_body18_e128093_d_n0;
            locals.var_t0_dn2 = assign84290_body18_e128093_d_n2;
            locals.var_t0_dn4 = assign84290_body18_e128093_d_n4;
            locals.var_t0_dn5 = assign84290_body18_e128093_d_n5;
            locals.var_t0_dn6 = assign84290_body18_e128093_d_n6;
            locals.var_t0_dn7 = assign84290_body18_e128093_d_n7;
            locals.var_t0_dn8 = assign84290_body18_e128093_d_n8;
            locals.var_t0_dn9 = assign84290_body18_e128093_d_n9;
            locals.var_t0_dn10 = assign84290_body18_e128093_d_n10;
            locals.var_t0_dn13 = assign84290_body18_e128093_d_n13;
            let (assign84290_body19_e128106, assign84290_body19_e128106_d_n0, assign84290_body19_e128106_d_n2, assign84290_body19_e128106_d_n4, assign84290_body19_e128106_d_n5, assign84290_body19_e128106_d_n6, assign84290_body19_e128106_d_n7, assign84290_body19_e128106_d_n8, assign84290_body19_e128106_d_n9, assign84290_body19_e128106_d_n10, assign84290_body19_e128106_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1946 != 0.0)) && (locals.var_guard1952 != 0.0)) {
        let assign84290_body19_e128104: f64 = (locals.var_chi * locals.var_t0);
        (assign84290_body19_e128104, ((locals.var_chi_dn0 * locals.var_t0) + (locals.var_chi * locals.var_t0_dn0)), ((locals.var_chi_dn2 * locals.var_t0) + (locals.var_chi * locals.var_t0_dn2)), ((locals.var_chi_dn4 * locals.var_t0) + (locals.var_chi * locals.var_t0_dn4)), ((locals.var_chi_dn5 * locals.var_t0) + (locals.var_chi * locals.var_t0_dn5)), ((locals.var_chi_dn6 * locals.var_t0) + (locals.var_chi * locals.var_t0_dn6)), ((locals.var_chi_dn7 * locals.var_t0) + (locals.var_chi * locals.var_t0_dn7)), ((locals.var_chi_dn8 * locals.var_t0) + (locals.var_chi * locals.var_t0_dn8)), ((locals.var_chi_dn9 * locals.var_t0) + (locals.var_chi * locals.var_t0_dn9)), ((locals.var_chi_dn10 * locals.var_t0) + (locals.var_chi * locals.var_t0_dn10)), ((locals.var_chi_dn13 * locals.var_t0) + (locals.var_chi * locals.var_t0_dn13)),)
    } else {
        (locals.var_fb, locals.var_fb_dn0, locals.var_fb_dn2, locals.var_fb_dn4, locals.var_fb_dn5, locals.var_fb_dn6, locals.var_fb_dn7, locals.var_fb_dn8, locals.var_fb_dn9, locals.var_fb_dn10, locals.var_fb_dn13,)
    }
};
            locals.var_fb = assign84290_body19_e128106;
            locals.var_fb_dn0 = assign84290_body19_e128106_d_n0;
            locals.var_fb_dn2 = assign84290_body19_e128106_d_n2;
            locals.var_fb_dn4 = assign84290_body19_e128106_d_n4;
            locals.var_fb_dn5 = assign84290_body19_e128106_d_n5;
            locals.var_fb_dn6 = assign84290_body19_e128106_d_n6;
            locals.var_fb_dn7 = assign84290_body19_e128106_d_n7;
            locals.var_fb_dn8 = assign84290_body19_e128106_d_n8;
            locals.var_fb_dn9 = assign84290_body19_e128106_d_n9;
            locals.var_fb_dn10 = assign84290_body19_e128106_d_n10;
            locals.var_fb_dn13 = assign84290_body19_e128106_d_n13;
            let (assign84290_body20_e128119, assign84290_body20_e128119_d_n0, assign84290_body20_e128119_d_n2, assign84290_body20_e128119_d_n4, assign84290_body20_e128119_d_n5, assign84290_body20_e128119_d_n6, assign84290_body20_e128119_d_n7, assign84290_body20_e128119_d_n8, assign84290_body20_e128119_d_n9, assign84290_body20_e128119_d_n10, assign84290_body20_e128119_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1946 != 0.0)) && (locals.var_guard1952 != 0.0)) {
        let assign84290_body20_e128117: f64 = (locals.var_beta * locals.var_t0);
        (assign84290_body20_e128117, ((locals.var_beta_dn0 * locals.var_t0) + (locals.var_beta * locals.var_t0_dn0)), ((locals.var_beta_dn2 * locals.var_t0) + (locals.var_beta * locals.var_t0_dn2)), ((locals.var_beta_dn4 * locals.var_t0) + (locals.var_beta * locals.var_t0_dn4)), ((locals.var_beta_dn5 * locals.var_t0) + (locals.var_beta * locals.var_t0_dn5)), ((locals.var_beta_dn6 * locals.var_t0) + (locals.var_beta * locals.var_t0_dn6)), ((locals.var_beta_dn7 * locals.var_t0) + (locals.var_beta * locals.var_t0_dn7)), ((locals.var_beta_dn8 * locals.var_t0) + (locals.var_beta * locals.var_t0_dn8)), ((locals.var_beta_dn9 * locals.var_t0) + (locals.var_beta * locals.var_t0_dn9)), ((locals.var_beta_dn10 * locals.var_t0) + (locals.var_beta * locals.var_t0_dn10)), ((locals.var_beta_dn13 * locals.var_t0) + (locals.var_beta * locals.var_t0_dn13)),)
    } else {
        (locals.var_fb_dpss, locals.var_fb_dpss_dn0, locals.var_fb_dpss_dn2, locals.var_fb_dpss_dn4, locals.var_fb_dpss_dn5, locals.var_fb_dpss_dn6, locals.var_fb_dpss_dn7, locals.var_fb_dpss_dn8, locals.var_fb_dpss_dn9, locals.var_fb_dpss_dn10, locals.var_fb_dpss_dn13,)
    }
};
            locals.var_fb_dpss = assign84290_body20_e128119;
            locals.var_fb_dpss_dn0 = assign84290_body20_e128119_d_n0;
            locals.var_fb_dpss_dn2 = assign84290_body20_e128119_d_n2;
            locals.var_fb_dpss_dn4 = assign84290_body20_e128119_d_n4;
            locals.var_fb_dpss_dn5 = assign84290_body20_e128119_d_n5;
            locals.var_fb_dpss_dn6 = assign84290_body20_e128119_d_n6;
            locals.var_fb_dpss_dn7 = assign84290_body20_e128119_d_n7;
            locals.var_fb_dpss_dn8 = assign84290_body20_e128119_d_n8;
            locals.var_fb_dpss_dn9 = assign84290_body20_e128119_d_n9;
            locals.var_fb_dpss_dn10 = assign84290_body20_e128119_d_n10;
            locals.var_fb_dpss_dn13 = assign84290_body20_e128119_d_n13;
            let assign84290_body21_e128122: f64 = if locals.var_chi < 1e-6 { 1.0 } else { 0.0 };
            locals.var_guard1953 = assign84290_body21_e128122;
            let (assign84290_body23_e128174, assign84290_body23_e128174_d_n0, assign84290_body23_e128174_d_n2, assign84290_body23_e128174_d_n4, assign84290_body23_e128174_d_n5, assign84290_body23_e128174_d_n6, assign84290_body23_e128174_d_n7, assign84290_body23_e128174_d_n8, assign84290_body23_e128174_d_n9, assign84290_body23_e128174_d_n10, assign84290_body23_e128174_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1946 != 0.0)) && (locals.var_guard1952 == 0.0)) && (locals.var_guard1953 != 0.0)) {
        let assign84290_body23_e128152: f64 = (locals.var_chi * locals.var_chi);
        let assign84290_body23_e128154: f64 = (assign84290_body23_e128152 / 2.0);
        let assign84290_body23_e128158: f64 = (locals.var_chi / 3.0);
        let assign84290_body23_e128162: f64 = (locals.var_chi / 4.0);
        let assign84290_body23_e128166: f64 = (locals.var_chi / 5.0);
        let assign84290_body23_e128167: f64 = (1.0 - assign84290_body23_e128166);
        let assign84290_body23_e128168: f64 = (assign84290_body23_e128162 * assign84290_body23_e128167);
        let assign84290_body23_e128169: f64 = (1.0 - assign84290_body23_e128168);
        let assign84290_body23_e128170: f64 = (assign84290_body23_e128158 * assign84290_body23_e128169);
        let assign84290_body23_e128171: f64 = (1.0 - assign84290_body23_e128170);
        let assign84290_body23_e128172: f64 = (assign84290_body23_e128154 * assign84290_body23_e128171);
        (assign84290_body23_e128172, (((((locals.var_chi_dn0 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn0)) / 2.0) * assign84290_body23_e128171) + (assign84290_body23_e128154 * (-(((locals.var_chi_dn0 / 3.0) * assign84290_body23_e128169) + (assign84290_body23_e128158 * (-(((locals.var_chi_dn0 / 4.0) * assign84290_body23_e128167) + (assign84290_body23_e128162 * (-(locals.var_chi_dn0 / 5.0)))))))))), (((((locals.var_chi_dn2 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn2)) / 2.0) * assign84290_body23_e128171) + (assign84290_body23_e128154 * (-(((locals.var_chi_dn2 / 3.0) * assign84290_body23_e128169) + (assign84290_body23_e128158 * (-(((locals.var_chi_dn2 / 4.0) * assign84290_body23_e128167) + (assign84290_body23_e128162 * (-(locals.var_chi_dn2 / 5.0)))))))))), (((((locals.var_chi_dn4 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn4)) / 2.0) * assign84290_body23_e128171) + (assign84290_body23_e128154 * (-(((locals.var_chi_dn4 / 3.0) * assign84290_body23_e128169) + (assign84290_body23_e128158 * (-(((locals.var_chi_dn4 / 4.0) * assign84290_body23_e128167) + (assign84290_body23_e128162 * (-(locals.var_chi_dn4 / 5.0)))))))))), (((((locals.var_chi_dn5 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn5)) / 2.0) * assign84290_body23_e128171) + (assign84290_body23_e128154 * (-(((locals.var_chi_dn5 / 3.0) * assign84290_body23_e128169) + (assign84290_body23_e128158 * (-(((locals.var_chi_dn5 / 4.0) * assign84290_body23_e128167) + (assign84290_body23_e128162 * (-(locals.var_chi_dn5 / 5.0)))))))))), (((((locals.var_chi_dn6 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn6)) / 2.0) * assign84290_body23_e128171) + (assign84290_body23_e128154 * (-(((locals.var_chi_dn6 / 3.0) * assign84290_body23_e128169) + (assign84290_body23_e128158 * (-(((locals.var_chi_dn6 / 4.0) * assign84290_body23_e128167) + (assign84290_body23_e128162 * (-(locals.var_chi_dn6 / 5.0)))))))))), (((((locals.var_chi_dn7 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn7)) / 2.0) * assign84290_body23_e128171) + (assign84290_body23_e128154 * (-(((locals.var_chi_dn7 / 3.0) * assign84290_body23_e128169) + (assign84290_body23_e128158 * (-(((locals.var_chi_dn7 / 4.0) * assign84290_body23_e128167) + (assign84290_body23_e128162 * (-(locals.var_chi_dn7 / 5.0)))))))))), (((((locals.var_chi_dn8 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn8)) / 2.0) * assign84290_body23_e128171) + (assign84290_body23_e128154 * (-(((locals.var_chi_dn8 / 3.0) * assign84290_body23_e128169) + (assign84290_body23_e128158 * (-(((locals.var_chi_dn8 / 4.0) * assign84290_body23_e128167) + (assign84290_body23_e128162 * (-(locals.var_chi_dn8 / 5.0)))))))))), (((((locals.var_chi_dn9 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn9)) / 2.0) * assign84290_body23_e128171) + (assign84290_body23_e128154 * (-(((locals.var_chi_dn9 / 3.0) * assign84290_body23_e128169) + (assign84290_body23_e128158 * (-(((locals.var_chi_dn9 / 4.0) * assign84290_body23_e128167) + (assign84290_body23_e128162 * (-(locals.var_chi_dn9 / 5.0)))))))))), (((((locals.var_chi_dn10 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn10)) / 2.0) * assign84290_body23_e128171) + (assign84290_body23_e128154 * (-(((locals.var_chi_dn10 / 3.0) * assign84290_body23_e128169) + (assign84290_body23_e128158 * (-(((locals.var_chi_dn10 / 4.0) * assign84290_body23_e128167) + (assign84290_body23_e128162 * (-(locals.var_chi_dn10 / 5.0)))))))))), (((((locals.var_chi_dn13 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn13)) / 2.0) * assign84290_body23_e128171) + (assign84290_body23_e128154 * (-(((locals.var_chi_dn13 / 3.0) * assign84290_body23_e128169) + (assign84290_body23_e128158 * (-(((locals.var_chi_dn13 / 4.0) * assign84290_body23_e128167) + (assign84290_body23_e128162 * (-(locals.var_chi_dn13 / 5.0)))))))))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
            locals.var_t0 = assign84290_body23_e128174;
            locals.var_t0_dn0 = assign84290_body23_e128174_d_n0;
            locals.var_t0_dn2 = assign84290_body23_e128174_d_n2;
            locals.var_t0_dn4 = assign84290_body23_e128174_d_n4;
            locals.var_t0_dn5 = assign84290_body23_e128174_d_n5;
            locals.var_t0_dn6 = assign84290_body23_e128174_d_n6;
            locals.var_t0_dn7 = assign84290_body23_e128174_d_n7;
            locals.var_t0_dn8 = assign84290_body23_e128174_d_n8;
            locals.var_t0_dn9 = assign84290_body23_e128174_d_n9;
            locals.var_t0_dn10 = assign84290_body23_e128174_d_n10;
            locals.var_t0_dn13 = assign84290_body23_e128174_d_n13;
            let (assign84290_body24_e128206, assign84290_body24_e128206_d_n0, assign84290_body24_e128206_d_n2, assign84290_body24_e128206_d_n4, assign84290_body24_e128206_d_n5, assign84290_body24_e128206_d_n6, assign84290_body24_e128206_d_n7, assign84290_body24_e128206_d_n8, assign84290_body24_e128206_d_n9, assign84290_body24_e128206_d_n10, assign84290_body24_e128206_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1946 != 0.0)) && (locals.var_guard1952 == 0.0)) && (locals.var_guard1953 != 0.0)) {
        let assign84290_body24_e128190: f64 = (locals.var_chi / 2.0);
        let assign84290_body24_e128194: f64 = (locals.var_chi / 3.0);
        let assign84290_body24_e128198: f64 = (locals.var_chi / 4.0);
        let assign84290_body24_e128199: f64 = (1.0 - assign84290_body24_e128198);
        let assign84290_body24_e128200: f64 = (assign84290_body24_e128194 * assign84290_body24_e128199);
        let assign84290_body24_e128201: f64 = (1.0 - assign84290_body24_e128200);
        let assign84290_body24_e128202: f64 = (assign84290_body24_e128190 * assign84290_body24_e128201);
        let assign84290_body24_e128203: f64 = (1.0 - assign84290_body24_e128202);
        let assign84290_body24_e128204: f64 = (locals.var_chi * assign84290_body24_e128203);
        (assign84290_body24_e128204, ((locals.var_chi_dn0 * assign84290_body24_e128203) + (locals.var_chi * (-(((locals.var_chi_dn0 / 2.0) * assign84290_body24_e128201) + (assign84290_body24_e128190 * (-(((locals.var_chi_dn0 / 3.0) * assign84290_body24_e128199) + (assign84290_body24_e128194 * (-(locals.var_chi_dn0 / 4.0)))))))))), ((locals.var_chi_dn2 * assign84290_body24_e128203) + (locals.var_chi * (-(((locals.var_chi_dn2 / 2.0) * assign84290_body24_e128201) + (assign84290_body24_e128190 * (-(((locals.var_chi_dn2 / 3.0) * assign84290_body24_e128199) + (assign84290_body24_e128194 * (-(locals.var_chi_dn2 / 4.0)))))))))), ((locals.var_chi_dn4 * assign84290_body24_e128203) + (locals.var_chi * (-(((locals.var_chi_dn4 / 2.0) * assign84290_body24_e128201) + (assign84290_body24_e128190 * (-(((locals.var_chi_dn4 / 3.0) * assign84290_body24_e128199) + (assign84290_body24_e128194 * (-(locals.var_chi_dn4 / 4.0)))))))))), ((locals.var_chi_dn5 * assign84290_body24_e128203) + (locals.var_chi * (-(((locals.var_chi_dn5 / 2.0) * assign84290_body24_e128201) + (assign84290_body24_e128190 * (-(((locals.var_chi_dn5 / 3.0) * assign84290_body24_e128199) + (assign84290_body24_e128194 * (-(locals.var_chi_dn5 / 4.0)))))))))), ((locals.var_chi_dn6 * assign84290_body24_e128203) + (locals.var_chi * (-(((locals.var_chi_dn6 / 2.0) * assign84290_body24_e128201) + (assign84290_body24_e128190 * (-(((locals.var_chi_dn6 / 3.0) * assign84290_body24_e128199) + (assign84290_body24_e128194 * (-(locals.var_chi_dn6 / 4.0)))))))))), ((locals.var_chi_dn7 * assign84290_body24_e128203) + (locals.var_chi * (-(((locals.var_chi_dn7 / 2.0) * assign84290_body24_e128201) + (assign84290_body24_e128190 * (-(((locals.var_chi_dn7 / 3.0) * assign84290_body24_e128199) + (assign84290_body24_e128194 * (-(locals.var_chi_dn7 / 4.0)))))))))), ((locals.var_chi_dn8 * assign84290_body24_e128203) + (locals.var_chi * (-(((locals.var_chi_dn8 / 2.0) * assign84290_body24_e128201) + (assign84290_body24_e128190 * (-(((locals.var_chi_dn8 / 3.0) * assign84290_body24_e128199) + (assign84290_body24_e128194 * (-(locals.var_chi_dn8 / 4.0)))))))))), ((locals.var_chi_dn9 * assign84290_body24_e128203) + (locals.var_chi * (-(((locals.var_chi_dn9 / 2.0) * assign84290_body24_e128201) + (assign84290_body24_e128190 * (-(((locals.var_chi_dn9 / 3.0) * assign84290_body24_e128199) + (assign84290_body24_e128194 * (-(locals.var_chi_dn9 / 4.0)))))))))), ((locals.var_chi_dn10 * assign84290_body24_e128203) + (locals.var_chi * (-(((locals.var_chi_dn10 / 2.0) * assign84290_body24_e128201) + (assign84290_body24_e128190 * (-(((locals.var_chi_dn10 / 3.0) * assign84290_body24_e128199) + (assign84290_body24_e128194 * (-(locals.var_chi_dn10 / 4.0)))))))))), ((locals.var_chi_dn13 * assign84290_body24_e128203) + (locals.var_chi * (-(((locals.var_chi_dn13 / 2.0) * assign84290_body24_e128201) + (assign84290_body24_e128190 * (-(((locals.var_chi_dn13 / 3.0) * assign84290_body24_e128199) + (assign84290_body24_e128194 * (-(locals.var_chi_dn13 / 4.0)))))))))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
            locals.var_t1 = assign84290_body24_e128206;
            locals.var_t1_dn0 = assign84290_body24_e128206_d_n0;
            locals.var_t1_dn2 = assign84290_body24_e128206_d_n2;
            locals.var_t1_dn4 = assign84290_body24_e128206_d_n4;
            locals.var_t1_dn5 = assign84290_body24_e128206_d_n5;
            locals.var_t1_dn6 = assign84290_body24_e128206_d_n6;
            locals.var_t1_dn7 = assign84290_body24_e128206_d_n7;
            locals.var_t1_dn8 = assign84290_body24_e128206_d_n8;
            locals.var_t1_dn9 = assign84290_body24_e128206_d_n9;
            locals.var_t1_dn10 = assign84290_body24_e128206_d_n10;
            locals.var_t1_dn13 = assign84290_body24_e128206_d_n13;
            let (assign84290_body25_e128242, assign84290_body25_e128242_d_n0, assign84290_body25_e128242_d_n2, assign84290_body25_e128242_d_n4, assign84290_body25_e128242_d_n5, assign84290_body25_e128242_d_n6, assign84290_body25_e128242_d_n7, assign84290_body25_e128242_d_n8, assign84290_body25_e128242_d_n9, assign84290_body25_e128242_d_n10, assign84290_body25_e128242_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1946 != 0.0)) && (locals.var_guard1952 == 0.0)) && (locals.var_guard1953 != 0.0)) {
        let assign84290_body25_e128220: f64 = (locals.var_chib * locals.var_chib);
        let assign84290_body25_e128222: f64 = (assign84290_body25_e128220 / 2.0);
        let assign84290_body25_e128226: f64 = (locals.var_chib / 3.0);
        let assign84290_body25_e128230: f64 = (locals.var_chib / 4.0);
        let assign84290_body25_e128234: f64 = (locals.var_chib / 5.0);
        let assign84290_body25_e128235: f64 = (1.0 - assign84290_body25_e128234);
        let assign84290_body25_e128236: f64 = (assign84290_body25_e128230 * assign84290_body25_e128235);
        let assign84290_body25_e128237: f64 = (1.0 - assign84290_body25_e128236);
        let assign84290_body25_e128238: f64 = (assign84290_body25_e128226 * assign84290_body25_e128237);
        let assign84290_body25_e128239: f64 = (1.0 - assign84290_body25_e128238);
        let assign84290_body25_e128240: f64 = (assign84290_body25_e128222 * assign84290_body25_e128239);
        (assign84290_body25_e128240, (((((locals.var_chib_dn0 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn0)) / 2.0) * assign84290_body25_e128239) + (assign84290_body25_e128222 * (-(((locals.var_chib_dn0 / 3.0) * assign84290_body25_e128237) + (assign84290_body25_e128226 * (-(((locals.var_chib_dn0 / 4.0) * assign84290_body25_e128235) + (assign84290_body25_e128230 * (-(locals.var_chib_dn0 / 5.0)))))))))), (((((locals.var_chib_dn2 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn2)) / 2.0) * assign84290_body25_e128239) + (assign84290_body25_e128222 * (-(((locals.var_chib_dn2 / 3.0) * assign84290_body25_e128237) + (assign84290_body25_e128226 * (-(((locals.var_chib_dn2 / 4.0) * assign84290_body25_e128235) + (assign84290_body25_e128230 * (-(locals.var_chib_dn2 / 5.0)))))))))), (((((locals.var_chib_dn4 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn4)) / 2.0) * assign84290_body25_e128239) + (assign84290_body25_e128222 * (-(((locals.var_chib_dn4 / 3.0) * assign84290_body25_e128237) + (assign84290_body25_e128226 * (-(((locals.var_chib_dn4 / 4.0) * assign84290_body25_e128235) + (assign84290_body25_e128230 * (-(locals.var_chib_dn4 / 5.0)))))))))), (((((locals.var_chib_dn5 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn5)) / 2.0) * assign84290_body25_e128239) + (assign84290_body25_e128222 * (-(((locals.var_chib_dn5 / 3.0) * assign84290_body25_e128237) + (assign84290_body25_e128226 * (-(((locals.var_chib_dn5 / 4.0) * assign84290_body25_e128235) + (assign84290_body25_e128230 * (-(locals.var_chib_dn5 / 5.0)))))))))), (((((locals.var_chib_dn6 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn6)) / 2.0) * assign84290_body25_e128239) + (assign84290_body25_e128222 * (-(((locals.var_chib_dn6 / 3.0) * assign84290_body25_e128237) + (assign84290_body25_e128226 * (-(((locals.var_chib_dn6 / 4.0) * assign84290_body25_e128235) + (assign84290_body25_e128230 * (-(locals.var_chib_dn6 / 5.0)))))))))), (((((locals.var_chib_dn7 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn7)) / 2.0) * assign84290_body25_e128239) + (assign84290_body25_e128222 * (-(((locals.var_chib_dn7 / 3.0) * assign84290_body25_e128237) + (assign84290_body25_e128226 * (-(((locals.var_chib_dn7 / 4.0) * assign84290_body25_e128235) + (assign84290_body25_e128230 * (-(locals.var_chib_dn7 / 5.0)))))))))), (((((locals.var_chib_dn8 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn8)) / 2.0) * assign84290_body25_e128239) + (assign84290_body25_e128222 * (-(((locals.var_chib_dn8 / 3.0) * assign84290_body25_e128237) + (assign84290_body25_e128226 * (-(((locals.var_chib_dn8 / 4.0) * assign84290_body25_e128235) + (assign84290_body25_e128230 * (-(locals.var_chib_dn8 / 5.0)))))))))), (((((locals.var_chib_dn9 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn9)) / 2.0) * assign84290_body25_e128239) + (assign84290_body25_e128222 * (-(((locals.var_chib_dn9 / 3.0) * assign84290_body25_e128237) + (assign84290_body25_e128226 * (-(((locals.var_chib_dn9 / 4.0) * assign84290_body25_e128235) + (assign84290_body25_e128230 * (-(locals.var_chib_dn9 / 5.0)))))))))), (((((locals.var_chib_dn10 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn10)) / 2.0) * assign84290_body25_e128239) + (assign84290_body25_e128222 * (-(((locals.var_chib_dn10 / 3.0) * assign84290_body25_e128237) + (assign84290_body25_e128226 * (-(((locals.var_chib_dn10 / 4.0) * assign84290_body25_e128235) + (assign84290_body25_e128230 * (-(locals.var_chib_dn10 / 5.0)))))))))), (((((locals.var_chib_dn13 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn13)) / 2.0) * assign84290_body25_e128239) + (assign84290_body25_e128222 * (-(((locals.var_chib_dn13 / 3.0) * assign84290_body25_e128237) + (assign84290_body25_e128226 * (-(((locals.var_chib_dn13 / 4.0) * assign84290_body25_e128235) + (assign84290_body25_e128230 * (-(locals.var_chib_dn13 / 5.0)))))))))),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
            locals.var_t2 = assign84290_body25_e128242;
            locals.var_t2_dn0 = assign84290_body25_e128242_d_n0;
            locals.var_t2_dn2 = assign84290_body25_e128242_d_n2;
            locals.var_t2_dn4 = assign84290_body25_e128242_d_n4;
            locals.var_t2_dn5 = assign84290_body25_e128242_d_n5;
            locals.var_t2_dn6 = assign84290_body25_e128242_d_n6;
            locals.var_t2_dn7 = assign84290_body25_e128242_d_n7;
            locals.var_t2_dn8 = assign84290_body25_e128242_d_n8;
            locals.var_t2_dn9 = assign84290_body25_e128242_d_n9;
            locals.var_t2_dn10 = assign84290_body25_e128242_d_n10;
            locals.var_t2_dn13 = assign84290_body25_e128242_d_n13;
            let (assign84290_body26_e128274, assign84290_body26_e128274_d_n0, assign84290_body26_e128274_d_n2, assign84290_body26_e128274_d_n4, assign84290_body26_e128274_d_n5, assign84290_body26_e128274_d_n6, assign84290_body26_e128274_d_n7, assign84290_body26_e128274_d_n8, assign84290_body26_e128274_d_n9, assign84290_body26_e128274_d_n10, assign84290_body26_e128274_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1946 != 0.0)) && (locals.var_guard1952 == 0.0)) && (locals.var_guard1953 != 0.0)) {
        let assign84290_body26_e128258: f64 = (locals.var_chib / 2.0);
        let assign84290_body26_e128262: f64 = (locals.var_chib / 3.0);
        let assign84290_body26_e128266: f64 = (locals.var_chib / 4.0);
        let assign84290_body26_e128267: f64 = (1.0 - assign84290_body26_e128266);
        let assign84290_body26_e128268: f64 = (assign84290_body26_e128262 * assign84290_body26_e128267);
        let assign84290_body26_e128269: f64 = (1.0 - assign84290_body26_e128268);
        let assign84290_body26_e128270: f64 = (assign84290_body26_e128258 * assign84290_body26_e128269);
        let assign84290_body26_e128271: f64 = (1.0 - assign84290_body26_e128270);
        let assign84290_body26_e128272: f64 = (locals.var_chib * assign84290_body26_e128271);
        (assign84290_body26_e128272, ((locals.var_chib_dn0 * assign84290_body26_e128271) + (locals.var_chib * (-(((locals.var_chib_dn0 / 2.0) * assign84290_body26_e128269) + (assign84290_body26_e128258 * (-(((locals.var_chib_dn0 / 3.0) * assign84290_body26_e128267) + (assign84290_body26_e128262 * (-(locals.var_chib_dn0 / 4.0)))))))))), ((locals.var_chib_dn2 * assign84290_body26_e128271) + (locals.var_chib * (-(((locals.var_chib_dn2 / 2.0) * assign84290_body26_e128269) + (assign84290_body26_e128258 * (-(((locals.var_chib_dn2 / 3.0) * assign84290_body26_e128267) + (assign84290_body26_e128262 * (-(locals.var_chib_dn2 / 4.0)))))))))), ((locals.var_chib_dn4 * assign84290_body26_e128271) + (locals.var_chib * (-(((locals.var_chib_dn4 / 2.0) * assign84290_body26_e128269) + (assign84290_body26_e128258 * (-(((locals.var_chib_dn4 / 3.0) * assign84290_body26_e128267) + (assign84290_body26_e128262 * (-(locals.var_chib_dn4 / 4.0)))))))))), ((locals.var_chib_dn5 * assign84290_body26_e128271) + (locals.var_chib * (-(((locals.var_chib_dn5 / 2.0) * assign84290_body26_e128269) + (assign84290_body26_e128258 * (-(((locals.var_chib_dn5 / 3.0) * assign84290_body26_e128267) + (assign84290_body26_e128262 * (-(locals.var_chib_dn5 / 4.0)))))))))), ((locals.var_chib_dn6 * assign84290_body26_e128271) + (locals.var_chib * (-(((locals.var_chib_dn6 / 2.0) * assign84290_body26_e128269) + (assign84290_body26_e128258 * (-(((locals.var_chib_dn6 / 3.0) * assign84290_body26_e128267) + (assign84290_body26_e128262 * (-(locals.var_chib_dn6 / 4.0)))))))))), ((locals.var_chib_dn7 * assign84290_body26_e128271) + (locals.var_chib * (-(((locals.var_chib_dn7 / 2.0) * assign84290_body26_e128269) + (assign84290_body26_e128258 * (-(((locals.var_chib_dn7 / 3.0) * assign84290_body26_e128267) + (assign84290_body26_e128262 * (-(locals.var_chib_dn7 / 4.0)))))))))), ((locals.var_chib_dn8 * assign84290_body26_e128271) + (locals.var_chib * (-(((locals.var_chib_dn8 / 2.0) * assign84290_body26_e128269) + (assign84290_body26_e128258 * (-(((locals.var_chib_dn8 / 3.0) * assign84290_body26_e128267) + (assign84290_body26_e128262 * (-(locals.var_chib_dn8 / 4.0)))))))))), ((locals.var_chib_dn9 * assign84290_body26_e128271) + (locals.var_chib * (-(((locals.var_chib_dn9 / 2.0) * assign84290_body26_e128269) + (assign84290_body26_e128258 * (-(((locals.var_chib_dn9 / 3.0) * assign84290_body26_e128267) + (assign84290_body26_e128262 * (-(locals.var_chib_dn9 / 4.0)))))))))), ((locals.var_chib_dn10 * assign84290_body26_e128271) + (locals.var_chib * (-(((locals.var_chib_dn10 / 2.0) * assign84290_body26_e128269) + (assign84290_body26_e128258 * (-(((locals.var_chib_dn10 / 3.0) * assign84290_body26_e128267) + (assign84290_body26_e128262 * (-(locals.var_chib_dn10 / 4.0)))))))))), ((locals.var_chib_dn13 * assign84290_body26_e128271) + (locals.var_chib * (-(((locals.var_chib_dn13 / 2.0) * assign84290_body26_e128269) + (assign84290_body26_e128258 * (-(((locals.var_chib_dn13 / 3.0) * assign84290_body26_e128267) + (assign84290_body26_e128262 * (-(locals.var_chib_dn13 / 4.0)))))))))),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn13,)
    }
};
            locals.var_t3 = assign84290_body26_e128274;
            locals.var_t3_dn0 = assign84290_body26_e128274_d_n0;
            locals.var_t3_dn2 = assign84290_body26_e128274_d_n2;
            locals.var_t3_dn4 = assign84290_body26_e128274_d_n4;
            locals.var_t3_dn5 = assign84290_body26_e128274_d_n5;
            locals.var_t3_dn6 = assign84290_body26_e128274_d_n6;
            locals.var_t3_dn7 = assign84290_body26_e128274_d_n7;
            locals.var_t3_dn8 = assign84290_body26_e128274_d_n8;
            locals.var_t3_dn9 = assign84290_body26_e128274_d_n9;
            locals.var_t3_dn10 = assign84290_body26_e128274_d_n10;
            locals.var_t3_dn13 = assign84290_body26_e128274_d_n13;
            let (assign84290_body27_e128290, assign84290_body27_e128290_d_n0, assign84290_body27_e128290_d_n2, assign84290_body27_e128290_d_n4, assign84290_body27_e128290_d_n5, assign84290_body27_e128290_d_n6, assign84290_body27_e128290_d_n7, assign84290_body27_e128290_d_n8, assign84290_body27_e128290_d_n9, assign84290_body27_e128290_d_n10, assign84290_body27_e128290_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1946 != 0.0)) && (locals.var_guard1952 == 0.0)) && (locals.var_guard1953 != 0.0)) {
        let assign84290_body27_e128288: f64 = (locals.var_t0 - locals.var_t2);
        (assign84290_body27_e128288, (locals.var_t0_dn0 - locals.var_t2_dn0), (locals.var_t0_dn2 - locals.var_t2_dn2), (locals.var_t0_dn4 - locals.var_t2_dn4), (locals.var_t0_dn5 - locals.var_t2_dn5), (locals.var_t0_dn6 - locals.var_t2_dn6), (locals.var_t0_dn7 - locals.var_t2_dn7), (locals.var_t0_dn8 - locals.var_t2_dn8), (locals.var_t0_dn9 - locals.var_t2_dn9), (locals.var_t0_dn10 - locals.var_t2_dn10), (locals.var_t0_dn13 - locals.var_t2_dn13),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn13,)
    }
};
            locals.var_t4 = assign84290_body27_e128290;
            locals.var_t4_dn0 = assign84290_body27_e128290_d_n0;
            locals.var_t4_dn2 = assign84290_body27_e128290_d_n2;
            locals.var_t4_dn4 = assign84290_body27_e128290_d_n4;
            locals.var_t4_dn5 = assign84290_body27_e128290_d_n5;
            locals.var_t4_dn6 = assign84290_body27_e128290_d_n6;
            locals.var_t4_dn7 = assign84290_body27_e128290_d_n7;
            locals.var_t4_dn8 = assign84290_body27_e128290_d_n8;
            locals.var_t4_dn9 = assign84290_body27_e128290_d_n9;
            locals.var_t4_dn10 = assign84290_body27_e128290_d_n10;
            locals.var_t4_dn13 = assign84290_body27_e128290_d_n13;
            let assign84290_body28_e128293: f64 = if locals.var_t4 > 0.0 { 1.0 } else { 0.0 };
            locals.var_guard1954 = assign84290_body28_e128293;
            let (assign84290_body29_e128310, assign84290_body29_e128310_d_n0, assign84290_body29_e128310_d_n2, assign84290_body29_e128310_d_n4, assign84290_body29_e128310_d_n5, assign84290_body29_e128310_d_n6, assign84290_body29_e128310_d_n7, assign84290_body29_e128310_d_n8, assign84290_body29_e128310_d_n9, assign84290_body29_e128310_d_n10, assign84290_body29_e128310_d_n13,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1946 != 0.0)) && (locals.var_guard1952 == 0.0)) && (locals.var_guard1953 != 0.0)) && (locals.var_guard1954 != 0.0)) {
        let assign84290_body29_e128308: f64 = (locals.var_t4).sqrt();
        (assign84290_body29_e128308, (locals.var_t4_dn0 / (2.0 * assign84290_body29_e128308)), (locals.var_t4_dn2 / (2.0 * assign84290_body29_e128308)), (locals.var_t4_dn4 / (2.0 * assign84290_body29_e128308)), (locals.var_t4_dn5 / (2.0 * assign84290_body29_e128308)), (locals.var_t4_dn6 / (2.0 * assign84290_body29_e128308)), (locals.var_t4_dn7 / (2.0 * assign84290_body29_e128308)), (locals.var_t4_dn8 / (2.0 * assign84290_body29_e128308)), (locals.var_t4_dn9 / (2.0 * assign84290_body29_e128308)), (locals.var_t4_dn10 / (2.0 * assign84290_body29_e128308)), (locals.var_t4_dn13 / (2.0 * assign84290_body29_e128308)),)
    } else {
        (locals.var_fb, locals.var_fb_dn0, locals.var_fb_dn2, locals.var_fb_dn4, locals.var_fb_dn5, locals.var_fb_dn6, locals.var_fb_dn7, locals.var_fb_dn8, locals.var_fb_dn9, locals.var_fb_dn10, locals.var_fb_dn13,)
    }
};
            locals.var_fb = assign84290_body29_e128310;
            locals.var_fb_dn0 = assign84290_body29_e128310_d_n0;
            locals.var_fb_dn2 = assign84290_body29_e128310_d_n2;
            locals.var_fb_dn4 = assign84290_body29_e128310_d_n4;
            locals.var_fb_dn5 = assign84290_body29_e128310_d_n5;
            locals.var_fb_dn6 = assign84290_body29_e128310_d_n6;
            locals.var_fb_dn7 = assign84290_body29_e128310_d_n7;
            locals.var_fb_dn8 = assign84290_body29_e128310_d_n8;
            locals.var_fb_dn9 = assign84290_body29_e128310_d_n9;
            locals.var_fb_dn10 = assign84290_body29_e128310_d_n10;
            locals.var_fb_dn13 = assign84290_body29_e128310_d_n13;
            let (assign84290_body30_e128336, assign84290_body30_e128336_d_n0, assign84290_body30_e128336_d_n2, assign84290_body30_e128336_d_n4, assign84290_body30_e128336_d_n5, assign84290_body30_e128336_d_n6, assign84290_body30_e128336_d_n7, assign84290_body30_e128336_d_n8, assign84290_body30_e128336_d_n9, assign84290_body30_e128336_d_n10, assign84290_body30_e128336_d_n13,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1946 != 0.0)) && (locals.var_guard1952 == 0.0)) && (locals.var_guard1953 != 0.0)) && (locals.var_guard1954 != 0.0)) {
        let assign84290_body30_e128326: f64 = (locals.var_beta * 0.5);
        let assign84290_body30_e128330: f64 = (locals.var_phi_b_dpss * locals.var_t3);
        let assign84290_body30_e128331: f64 = (locals.var_t1 - assign84290_body30_e128330);
        let assign84290_body30_e128332: f64 = (assign84290_body30_e128326 * assign84290_body30_e128331);
        let assign84290_body30_e128334: f64 = (assign84290_body30_e128332 / locals.var_fb);
        (assign84290_body30_e128334, ((((((locals.var_beta_dn0 * 0.5) * assign84290_body30_e128331) + (assign84290_body30_e128326 * (locals.var_t1_dn0 - ((locals.var_phi_b_dpss_dn0 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn0))))) * locals.var_fb) - (assign84290_body30_e128332 * locals.var_fb_dn0)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn2 * 0.5) * assign84290_body30_e128331) + (assign84290_body30_e128326 * (locals.var_t1_dn2 - ((locals.var_phi_b_dpss_dn2 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn2))))) * locals.var_fb) - (assign84290_body30_e128332 * locals.var_fb_dn2)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn4 * 0.5) * assign84290_body30_e128331) + (assign84290_body30_e128326 * (locals.var_t1_dn4 - ((locals.var_phi_b_dpss_dn4 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn4))))) * locals.var_fb) - (assign84290_body30_e128332 * locals.var_fb_dn4)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn5 * 0.5) * assign84290_body30_e128331) + (assign84290_body30_e128326 * (locals.var_t1_dn5 - ((locals.var_phi_b_dpss_dn5 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn5))))) * locals.var_fb) - (assign84290_body30_e128332 * locals.var_fb_dn5)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn6 * 0.5) * assign84290_body30_e128331) + (assign84290_body30_e128326 * (locals.var_t1_dn6 - ((locals.var_phi_b_dpss_dn6 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn6))))) * locals.var_fb) - (assign84290_body30_e128332 * locals.var_fb_dn6)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn7 * 0.5) * assign84290_body30_e128331) + (assign84290_body30_e128326 * (locals.var_t1_dn7 - ((locals.var_phi_b_dpss_dn7 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn7))))) * locals.var_fb) - (assign84290_body30_e128332 * locals.var_fb_dn7)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn8 * 0.5) * assign84290_body30_e128331) + (assign84290_body30_e128326 * (locals.var_t1_dn8 - ((locals.var_phi_b_dpss_dn8 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn8))))) * locals.var_fb) - (assign84290_body30_e128332 * locals.var_fb_dn8)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn9 * 0.5) * assign84290_body30_e128331) + (assign84290_body30_e128326 * (locals.var_t1_dn9 - ((locals.var_phi_b_dpss_dn9 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn9))))) * locals.var_fb) - (assign84290_body30_e128332 * locals.var_fb_dn9)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn10 * 0.5) * assign84290_body30_e128331) + (assign84290_body30_e128326 * (locals.var_t1_dn10 - ((locals.var_phi_b_dpss_dn10 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn10))))) * locals.var_fb) - (assign84290_body30_e128332 * locals.var_fb_dn10)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn13 * 0.5) * assign84290_body30_e128331) + (assign84290_body30_e128326 * (locals.var_t1_dn13 - ((locals.var_phi_b_dpss_dn13 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn13))))) * locals.var_fb) - (assign84290_body30_e128332 * locals.var_fb_dn13)) / (locals.var_fb * locals.var_fb)),)
    } else {
        (locals.var_fb_dpss, locals.var_fb_dpss_dn0, locals.var_fb_dpss_dn2, locals.var_fb_dpss_dn4, locals.var_fb_dpss_dn5, locals.var_fb_dpss_dn6, locals.var_fb_dpss_dn7, locals.var_fb_dpss_dn8, locals.var_fb_dpss_dn9, locals.var_fb_dpss_dn10, locals.var_fb_dpss_dn13,)
    }
};
            locals.var_fb_dpss = assign84290_body30_e128336;
            locals.var_fb_dpss_dn0 = assign84290_body30_e128336_d_n0;
            locals.var_fb_dpss_dn2 = assign84290_body30_e128336_d_n2;
            locals.var_fb_dpss_dn4 = assign84290_body30_e128336_d_n4;
            locals.var_fb_dpss_dn5 = assign84290_body30_e128336_d_n5;
            locals.var_fb_dpss_dn6 = assign84290_body30_e128336_d_n6;
            locals.var_fb_dpss_dn7 = assign84290_body30_e128336_d_n7;
            locals.var_fb_dpss_dn8 = assign84290_body30_e128336_d_n8;
            locals.var_fb_dpss_dn9 = assign84290_body30_e128336_d_n9;
            locals.var_fb_dpss_dn10 = assign84290_body30_e128336_d_n10;
            locals.var_fb_dpss_dn13 = assign84290_body30_e128336_d_n13;
            let (assign84290_body32_e128372, assign84290_body32_e128372_d_n0, assign84290_body32_e128372_d_n2, assign84290_body32_e128372_d_n4, assign84290_body32_e128372_d_n5, assign84290_body32_e128372_d_n6, assign84290_body32_e128372_d_n7, assign84290_body32_e128372_d_n8, assign84290_body32_e128372_d_n9, assign84290_body32_e128372_d_n10, assign84290_body32_e128372_d_n13,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1946 != 0.0)) && (locals.var_guard1952 == 0.0)) && (locals.var_guard1953 != 0.0)) && (locals.var_guard1954 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fb, locals.var_fb_dn0, locals.var_fb_dn2, locals.var_fb_dn4, locals.var_fb_dn5, locals.var_fb_dn6, locals.var_fb_dn7, locals.var_fb_dn8, locals.var_fb_dn9, locals.var_fb_dn10, locals.var_fb_dn13,)
    }
};
            locals.var_fb = assign84290_body32_e128372;
            locals.var_fb_dn0 = assign84290_body32_e128372_d_n0;
            locals.var_fb_dn2 = assign84290_body32_e128372_d_n2;
            locals.var_fb_dn4 = assign84290_body32_e128372_d_n4;
            locals.var_fb_dn5 = assign84290_body32_e128372_d_n5;
            locals.var_fb_dn6 = assign84290_body32_e128372_d_n6;
            locals.var_fb_dn7 = assign84290_body32_e128372_d_n7;
            locals.var_fb_dn8 = assign84290_body32_e128372_d_n8;
            locals.var_fb_dn9 = assign84290_body32_e128372_d_n9;
            locals.var_fb_dn10 = assign84290_body32_e128372_d_n10;
            locals.var_fb_dn13 = assign84290_body32_e128372_d_n13;
            let (assign84290_body33_e128389, assign84290_body33_e128389_d_n0, assign84290_body33_e128389_d_n2, assign84290_body33_e128389_d_n4, assign84290_body33_e128389_d_n5, assign84290_body33_e128389_d_n6, assign84290_body33_e128389_d_n7, assign84290_body33_e128389_d_n8, assign84290_body33_e128389_d_n9, assign84290_body33_e128389_d_n10, assign84290_body33_e128389_d_n13,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1946 != 0.0)) && (locals.var_guard1952 == 0.0)) && (locals.var_guard1953 != 0.0)) && (locals.var_guard1954 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fb_dpss, locals.var_fb_dpss_dn0, locals.var_fb_dpss_dn2, locals.var_fb_dpss_dn4, locals.var_fb_dpss_dn5, locals.var_fb_dpss_dn6, locals.var_fb_dpss_dn7, locals.var_fb_dpss_dn8, locals.var_fb_dpss_dn9, locals.var_fb_dpss_dn10, locals.var_fb_dpss_dn13,)
    }
};
            locals.var_fb_dpss = assign84290_body33_e128389;
            locals.var_fb_dpss_dn0 = assign84290_body33_e128389_d_n0;
            locals.var_fb_dpss_dn2 = assign84290_body33_e128389_d_n2;
            locals.var_fb_dpss_dn4 = assign84290_body33_e128389_d_n4;
            locals.var_fb_dpss_dn5 = assign84290_body33_e128389_d_n5;
            locals.var_fb_dpss_dn6 = assign84290_body33_e128389_d_n6;
            locals.var_fb_dpss_dn7 = assign84290_body33_e128389_d_n7;
            locals.var_fb_dpss_dn8 = assign84290_body33_e128389_d_n8;
            locals.var_fb_dpss_dn9 = assign84290_body33_e128389_d_n9;
            locals.var_fb_dpss_dn10 = assign84290_body33_e128389_d_n10;
            locals.var_fb_dpss_dn13 = assign84290_body33_e128389_d_n13;
            let (assign84290_body34_e128406, assign84290_body34_e128406_d_n0, assign84290_body34_e128406_d_n2, assign84290_body34_e128406_d_n4, assign84290_body34_e128406_d_n5, assign84290_body34_e128406_d_n6, assign84290_body34_e128406_d_n7, assign84290_body34_e128406_d_n8, assign84290_body34_e128406_d_n9, assign84290_body34_e128406_d_n10, assign84290_body34_e128406_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1946 != 0.0)) && (locals.var_guard1952 == 0.0)) && (locals.var_guard1953 == 0.0)) {
        let assign84290_body34_e128403: f64 = (-locals.var_chi);
        let assign84290_body34_e128404: f64 = (assign84290_body34_e128403).exp();
        (assign84290_body34_e128404, (assign84290_body34_e128404 * (-locals.var_chi_dn0)), (assign84290_body34_e128404 * (-locals.var_chi_dn2)), (assign84290_body34_e128404 * (-locals.var_chi_dn4)), (assign84290_body34_e128404 * (-locals.var_chi_dn5)), (assign84290_body34_e128404 * (-locals.var_chi_dn6)), (assign84290_body34_e128404 * (-locals.var_chi_dn7)), (assign84290_body34_e128404 * (-locals.var_chi_dn8)), (assign84290_body34_e128404 * (-locals.var_chi_dn9)), (assign84290_body34_e128404 * (-locals.var_chi_dn10)), (assign84290_body34_e128404 * (-locals.var_chi_dn13)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
            locals.var_t0 = assign84290_body34_e128406;
            locals.var_t0_dn0 = assign84290_body34_e128406_d_n0;
            locals.var_t0_dn2 = assign84290_body34_e128406_d_n2;
            locals.var_t0_dn4 = assign84290_body34_e128406_d_n4;
            locals.var_t0_dn5 = assign84290_body34_e128406_d_n5;
            locals.var_t0_dn6 = assign84290_body34_e128406_d_n6;
            locals.var_t0_dn7 = assign84290_body34_e128406_d_n7;
            locals.var_t0_dn8 = assign84290_body34_e128406_d_n8;
            locals.var_t0_dn9 = assign84290_body34_e128406_d_n9;
            locals.var_t0_dn10 = assign84290_body34_e128406_d_n10;
            locals.var_t0_dn13 = assign84290_body34_e128406_d_n13;
            let (assign84290_body35_e128423, assign84290_body35_e128423_d_n0, assign84290_body35_e128423_d_n2, assign84290_body35_e128423_d_n4, assign84290_body35_e128423_d_n5, assign84290_body35_e128423_d_n6, assign84290_body35_e128423_d_n7, assign84290_body35_e128423_d_n8, assign84290_body35_e128423_d_n9, assign84290_body35_e128423_d_n10, assign84290_body35_e128423_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1946 != 0.0)) && (locals.var_guard1952 == 0.0)) && (locals.var_guard1953 == 0.0)) {
        let assign84290_body35_e128420: f64 = (-locals.var_chib);
        let assign84290_body35_e128421: f64 = (assign84290_body35_e128420).exp();
        (assign84290_body35_e128421, (assign84290_body35_e128421 * (-locals.var_chib_dn0)), (assign84290_body35_e128421 * (-locals.var_chib_dn2)), (assign84290_body35_e128421 * (-locals.var_chib_dn4)), (assign84290_body35_e128421 * (-locals.var_chib_dn5)), (assign84290_body35_e128421 * (-locals.var_chib_dn6)), (assign84290_body35_e128421 * (-locals.var_chib_dn7)), (assign84290_body35_e128421 * (-locals.var_chib_dn8)), (assign84290_body35_e128421 * (-locals.var_chib_dn9)), (assign84290_body35_e128421 * (-locals.var_chib_dn10)), (assign84290_body35_e128421 * (-locals.var_chib_dn13)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
            locals.var_t1 = assign84290_body35_e128423;
            locals.var_t1_dn0 = assign84290_body35_e128423_d_n0;
            locals.var_t1_dn2 = assign84290_body35_e128423_d_n2;
            locals.var_t1_dn4 = assign84290_body35_e128423_d_n4;
            locals.var_t1_dn5 = assign84290_body35_e128423_d_n5;
            locals.var_t1_dn6 = assign84290_body35_e128423_d_n6;
            locals.var_t1_dn7 = assign84290_body35_e128423_d_n7;
            locals.var_t1_dn8 = assign84290_body35_e128423_d_n8;
            locals.var_t1_dn9 = assign84290_body35_e128423_d_n9;
            locals.var_t1_dn10 = assign84290_body35_e128423_d_n10;
            locals.var_t1_dn13 = assign84290_body35_e128423_d_n13;
            let (assign84290_body36_e128444, assign84290_body36_e128444_d_n0, assign84290_body36_e128444_d_n2, assign84290_body36_e128444_d_n4, assign84290_body36_e128444_d_n5, assign84290_body36_e128444_d_n6, assign84290_body36_e128444_d_n7, assign84290_body36_e128444_d_n8, assign84290_body36_e128444_d_n9, assign84290_body36_e128444_d_n10, assign84290_body36_e128444_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1946 != 0.0)) && (locals.var_guard1952 == 0.0)) && (locals.var_guard1953 == 0.0)) {
        let assign84290_body36_e128438: f64 = (locals.var_chi - locals.var_chib);
        let assign84290_body36_e128441: f64 = (locals.var_t0 - locals.var_t1);
        let assign84290_body36_e128442: f64 = (assign84290_body36_e128438 + assign84290_body36_e128441);
        (assign84290_body36_e128442, ((locals.var_chi_dn0 - locals.var_chib_dn0) + (locals.var_t0_dn0 - locals.var_t1_dn0)), ((locals.var_chi_dn2 - locals.var_chib_dn2) + (locals.var_t0_dn2 - locals.var_t1_dn2)), ((locals.var_chi_dn4 - locals.var_chib_dn4) + (locals.var_t0_dn4 - locals.var_t1_dn4)), ((locals.var_chi_dn5 - locals.var_chib_dn5) + (locals.var_t0_dn5 - locals.var_t1_dn5)), ((locals.var_chi_dn6 - locals.var_chib_dn6) + (locals.var_t0_dn6 - locals.var_t1_dn6)), ((locals.var_chi_dn7 - locals.var_chib_dn7) + (locals.var_t0_dn7 - locals.var_t1_dn7)), ((locals.var_chi_dn8 - locals.var_chib_dn8) + (locals.var_t0_dn8 - locals.var_t1_dn8)), ((locals.var_chi_dn9 - locals.var_chib_dn9) + (locals.var_t0_dn9 - locals.var_t1_dn9)), ((locals.var_chi_dn10 - locals.var_chib_dn10) + (locals.var_t0_dn10 - locals.var_t1_dn10)), ((locals.var_chi_dn13 - locals.var_chib_dn13) + (locals.var_t0_dn13 - locals.var_t1_dn13)),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn13,)
    }
};
            locals.var_t4 = assign84290_body36_e128444;
            locals.var_t4_dn0 = assign84290_body36_e128444_d_n0;
            locals.var_t4_dn2 = assign84290_body36_e128444_d_n2;
            locals.var_t4_dn4 = assign84290_body36_e128444_d_n4;
            locals.var_t4_dn5 = assign84290_body36_e128444_d_n5;
            locals.var_t4_dn6 = assign84290_body36_e128444_d_n6;
            locals.var_t4_dn7 = assign84290_body36_e128444_d_n7;
            locals.var_t4_dn8 = assign84290_body36_e128444_d_n8;
            locals.var_t4_dn9 = assign84290_body36_e128444_d_n9;
            locals.var_t4_dn10 = assign84290_body36_e128444_d_n10;
            locals.var_t4_dn13 = assign84290_body36_e128444_d_n13;
            let assign84290_body37_e128447: f64 = if locals.var_t4 > 0.0 { 1.0 } else { 0.0 };
            locals.var_guard1955 = assign84290_body37_e128447;
            let (assign84290_body38_e128465, assign84290_body38_e128465_d_n0, assign84290_body38_e128465_d_n2, assign84290_body38_e128465_d_n4, assign84290_body38_e128465_d_n5, assign84290_body38_e128465_d_n6, assign84290_body38_e128465_d_n7, assign84290_body38_e128465_d_n8, assign84290_body38_e128465_d_n9, assign84290_body38_e128465_d_n10, assign84290_body38_e128465_d_n13,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1946 != 0.0)) && (locals.var_guard1952 == 0.0)) && (locals.var_guard1953 == 0.0)) && (locals.var_guard1955 != 0.0)) {
        let assign84290_body38_e128463: f64 = (locals.var_t4).sqrt();
        (assign84290_body38_e128463, (locals.var_t4_dn0 / (2.0 * assign84290_body38_e128463)), (locals.var_t4_dn2 / (2.0 * assign84290_body38_e128463)), (locals.var_t4_dn4 / (2.0 * assign84290_body38_e128463)), (locals.var_t4_dn5 / (2.0 * assign84290_body38_e128463)), (locals.var_t4_dn6 / (2.0 * assign84290_body38_e128463)), (locals.var_t4_dn7 / (2.0 * assign84290_body38_e128463)), (locals.var_t4_dn8 / (2.0 * assign84290_body38_e128463)), (locals.var_t4_dn9 / (2.0 * assign84290_body38_e128463)), (locals.var_t4_dn10 / (2.0 * assign84290_body38_e128463)), (locals.var_t4_dn13 / (2.0 * assign84290_body38_e128463)),)
    } else {
        (locals.var_fb, locals.var_fb_dn0, locals.var_fb_dn2, locals.var_fb_dn4, locals.var_fb_dn5, locals.var_fb_dn6, locals.var_fb_dn7, locals.var_fb_dn8, locals.var_fb_dn9, locals.var_fb_dn10, locals.var_fb_dn13,)
    }
};
            locals.var_fb = assign84290_body38_e128465;
            locals.var_fb_dn0 = assign84290_body38_e128465_d_n0;
            locals.var_fb_dn2 = assign84290_body38_e128465_d_n2;
            locals.var_fb_dn4 = assign84290_body38_e128465_d_n4;
            locals.var_fb_dn5 = assign84290_body38_e128465_d_n5;
            locals.var_fb_dn6 = assign84290_body38_e128465_d_n6;
            locals.var_fb_dn7 = assign84290_body38_e128465_d_n7;
            locals.var_fb_dn8 = assign84290_body38_e128465_d_n8;
            locals.var_fb_dn9 = assign84290_body38_e128465_d_n9;
            locals.var_fb_dn10 = assign84290_body38_e128465_d_n10;
            locals.var_fb_dn13 = assign84290_body38_e128465_d_n13;
            let (assign84290_body39_e128496, assign84290_body39_e128496_d_n0, assign84290_body39_e128496_d_n2, assign84290_body39_e128496_d_n4, assign84290_body39_e128496_d_n5, assign84290_body39_e128496_d_n6, assign84290_body39_e128496_d_n7, assign84290_body39_e128496_d_n8, assign84290_body39_e128496_d_n9, assign84290_body39_e128496_d_n10, assign84290_body39_e128496_d_n13,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1946 != 0.0)) && (locals.var_guard1952 == 0.0)) && (locals.var_guard1953 == 0.0)) && (locals.var_guard1955 != 0.0)) {
        let assign84290_body39_e128482: f64 = (locals.var_beta * 0.5);
        let assign84290_body39_e128485: f64 = (1.0 - locals.var_t0);
        let assign84290_body39_e128489: f64 = (1.0 - locals.var_t1);
        let assign84290_body39_e128490: f64 = (locals.var_phi_b_dpss * assign84290_body39_e128489);
        let assign84290_body39_e128491: f64 = (assign84290_body39_e128485 - assign84290_body39_e128490);
        let assign84290_body39_e128492: f64 = (assign84290_body39_e128482 * assign84290_body39_e128491);
        let assign84290_body39_e128494: f64 = (assign84290_body39_e128492 / locals.var_fb);
        (assign84290_body39_e128494, ((((((locals.var_beta_dn0 * 0.5) * assign84290_body39_e128491) + (assign84290_body39_e128482 * ((-locals.var_t0_dn0) - ((locals.var_phi_b_dpss_dn0 * assign84290_body39_e128489) + (locals.var_phi_b_dpss * (-locals.var_t1_dn0)))))) * locals.var_fb) - (assign84290_body39_e128492 * locals.var_fb_dn0)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn2 * 0.5) * assign84290_body39_e128491) + (assign84290_body39_e128482 * ((-locals.var_t0_dn2) - ((locals.var_phi_b_dpss_dn2 * assign84290_body39_e128489) + (locals.var_phi_b_dpss * (-locals.var_t1_dn2)))))) * locals.var_fb) - (assign84290_body39_e128492 * locals.var_fb_dn2)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn4 * 0.5) * assign84290_body39_e128491) + (assign84290_body39_e128482 * ((-locals.var_t0_dn4) - ((locals.var_phi_b_dpss_dn4 * assign84290_body39_e128489) + (locals.var_phi_b_dpss * (-locals.var_t1_dn4)))))) * locals.var_fb) - (assign84290_body39_e128492 * locals.var_fb_dn4)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn5 * 0.5) * assign84290_body39_e128491) + (assign84290_body39_e128482 * ((-locals.var_t0_dn5) - ((locals.var_phi_b_dpss_dn5 * assign84290_body39_e128489) + (locals.var_phi_b_dpss * (-locals.var_t1_dn5)))))) * locals.var_fb) - (assign84290_body39_e128492 * locals.var_fb_dn5)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn6 * 0.5) * assign84290_body39_e128491) + (assign84290_body39_e128482 * ((-locals.var_t0_dn6) - ((locals.var_phi_b_dpss_dn6 * assign84290_body39_e128489) + (locals.var_phi_b_dpss * (-locals.var_t1_dn6)))))) * locals.var_fb) - (assign84290_body39_e128492 * locals.var_fb_dn6)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn7 * 0.5) * assign84290_body39_e128491) + (assign84290_body39_e128482 * ((-locals.var_t0_dn7) - ((locals.var_phi_b_dpss_dn7 * assign84290_body39_e128489) + (locals.var_phi_b_dpss * (-locals.var_t1_dn7)))))) * locals.var_fb) - (assign84290_body39_e128492 * locals.var_fb_dn7)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn8 * 0.5) * assign84290_body39_e128491) + (assign84290_body39_e128482 * ((-locals.var_t0_dn8) - ((locals.var_phi_b_dpss_dn8 * assign84290_body39_e128489) + (locals.var_phi_b_dpss * (-locals.var_t1_dn8)))))) * locals.var_fb) - (assign84290_body39_e128492 * locals.var_fb_dn8)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn9 * 0.5) * assign84290_body39_e128491) + (assign84290_body39_e128482 * ((-locals.var_t0_dn9) - ((locals.var_phi_b_dpss_dn9 * assign84290_body39_e128489) + (locals.var_phi_b_dpss * (-locals.var_t1_dn9)))))) * locals.var_fb) - (assign84290_body39_e128492 * locals.var_fb_dn9)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn10 * 0.5) * assign84290_body39_e128491) + (assign84290_body39_e128482 * ((-locals.var_t0_dn10) - ((locals.var_phi_b_dpss_dn10 * assign84290_body39_e128489) + (locals.var_phi_b_dpss * (-locals.var_t1_dn10)))))) * locals.var_fb) - (assign84290_body39_e128492 * locals.var_fb_dn10)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn13 * 0.5) * assign84290_body39_e128491) + (assign84290_body39_e128482 * ((-locals.var_t0_dn13) - ((locals.var_phi_b_dpss_dn13 * assign84290_body39_e128489) + (locals.var_phi_b_dpss * (-locals.var_t1_dn13)))))) * locals.var_fb) - (assign84290_body39_e128492 * locals.var_fb_dn13)) / (locals.var_fb * locals.var_fb)),)
    } else {
        (locals.var_fb_dpss, locals.var_fb_dpss_dn0, locals.var_fb_dpss_dn2, locals.var_fb_dpss_dn4, locals.var_fb_dpss_dn5, locals.var_fb_dpss_dn6, locals.var_fb_dpss_dn7, locals.var_fb_dpss_dn8, locals.var_fb_dpss_dn9, locals.var_fb_dpss_dn10, locals.var_fb_dpss_dn13,)
    }
};
            locals.var_fb_dpss = assign84290_body39_e128496;
            locals.var_fb_dpss_dn0 = assign84290_body39_e128496_d_n0;
            locals.var_fb_dpss_dn2 = assign84290_body39_e128496_d_n2;
            locals.var_fb_dpss_dn4 = assign84290_body39_e128496_d_n4;
            locals.var_fb_dpss_dn5 = assign84290_body39_e128496_d_n5;
            locals.var_fb_dpss_dn6 = assign84290_body39_e128496_d_n6;
            locals.var_fb_dpss_dn7 = assign84290_body39_e128496_d_n7;
            locals.var_fb_dpss_dn8 = assign84290_body39_e128496_d_n8;
            locals.var_fb_dpss_dn9 = assign84290_body39_e128496_d_n9;
            locals.var_fb_dpss_dn10 = assign84290_body39_e128496_d_n10;
            locals.var_fb_dpss_dn13 = assign84290_body39_e128496_d_n13;
            let (assign84290_body41_e128534, assign84290_body41_e128534_d_n0, assign84290_body41_e128534_d_n2, assign84290_body41_e128534_d_n4, assign84290_body41_e128534_d_n5, assign84290_body41_e128534_d_n6, assign84290_body41_e128534_d_n7, assign84290_body41_e128534_d_n8, assign84290_body41_e128534_d_n9, assign84290_body41_e128534_d_n10, assign84290_body41_e128534_d_n13,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1946 != 0.0)) && (locals.var_guard1952 == 0.0)) && (locals.var_guard1953 == 0.0)) && (locals.var_guard1955 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fb, locals.var_fb_dn0, locals.var_fb_dn2, locals.var_fb_dn4, locals.var_fb_dn5, locals.var_fb_dn6, locals.var_fb_dn7, locals.var_fb_dn8, locals.var_fb_dn9, locals.var_fb_dn10, locals.var_fb_dn13,)
    }
};
            locals.var_fb = assign84290_body41_e128534;
            locals.var_fb_dn0 = assign84290_body41_e128534_d_n0;
            locals.var_fb_dn2 = assign84290_body41_e128534_d_n2;
            locals.var_fb_dn4 = assign84290_body41_e128534_d_n4;
            locals.var_fb_dn5 = assign84290_body41_e128534_d_n5;
            locals.var_fb_dn6 = assign84290_body41_e128534_d_n6;
            locals.var_fb_dn7 = assign84290_body41_e128534_d_n7;
            locals.var_fb_dn8 = assign84290_body41_e128534_d_n8;
            locals.var_fb_dn9 = assign84290_body41_e128534_d_n9;
            locals.var_fb_dn10 = assign84290_body41_e128534_d_n10;
            locals.var_fb_dn13 = assign84290_body41_e128534_d_n13;
            let (assign84290_body42_e128552, assign84290_body42_e128552_d_n0, assign84290_body42_e128552_d_n2, assign84290_body42_e128552_d_n4, assign84290_body42_e128552_d_n5, assign84290_body42_e128552_d_n6, assign84290_body42_e128552_d_n7, assign84290_body42_e128552_d_n8, assign84290_body42_e128552_d_n9, assign84290_body42_e128552_d_n10, assign84290_body42_e128552_d_n13,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1946 != 0.0)) && (locals.var_guard1952 == 0.0)) && (locals.var_guard1953 == 0.0)) && (locals.var_guard1955 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fb_dpss, locals.var_fb_dpss_dn0, locals.var_fb_dpss_dn2, locals.var_fb_dpss_dn4, locals.var_fb_dpss_dn5, locals.var_fb_dpss_dn6, locals.var_fb_dpss_dn7, locals.var_fb_dpss_dn8, locals.var_fb_dpss_dn9, locals.var_fb_dpss_dn10, locals.var_fb_dpss_dn13,)
    }
};
            locals.var_fb_dpss = assign84290_body42_e128552;
            locals.var_fb_dpss_dn0 = assign84290_body42_e128552_d_n0;
            locals.var_fb_dpss_dn2 = assign84290_body42_e128552_d_n2;
            locals.var_fb_dpss_dn4 = assign84290_body42_e128552_d_n4;
            locals.var_fb_dpss_dn5 = assign84290_body42_e128552_d_n5;
            locals.var_fb_dpss_dn6 = assign84290_body42_e128552_d_n6;
            locals.var_fb_dpss_dn7 = assign84290_body42_e128552_d_n7;
            locals.var_fb_dpss_dn8 = assign84290_body42_e128552_d_n8;
            locals.var_fb_dpss_dn9 = assign84290_body42_e128552_d_n9;
            locals.var_fb_dpss_dn10 = assign84290_body42_e128552_d_n10;
            locals.var_fb_dpss_dn13 = assign84290_body42_e128552_d_n13;
            let assign84290_body43_e128555: f64 = if locals.var_chi < 0.0 { 1.0 } else { 0.0 };
            locals.var_guard1956 = assign84290_body43_e128555;
            let (assign84290_body45_e128579, assign84290_body45_e128579_d_n0, assign84290_body45_e128579_d_n2, assign84290_body45_e128579_d_n4, assign84290_body45_e128579_d_n5, assign84290_body45_e128579_d_n6, assign84290_body45_e128579_d_n7, assign84290_body45_e128579_d_n8, assign84290_body45_e128579_d_n9, assign84290_body45_e128579_d_n10, assign84290_body45_e128579_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1946 != 0.0)) && (locals.var_guard1956 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fs01, locals.var_fs01_dn0, locals.var_fs01_dn2, locals.var_fs01_dn4, locals.var_fs01_dn5, locals.var_fs01_dn6, locals.var_fs01_dn7, locals.var_fs01_dn8, locals.var_fs01_dn9, locals.var_fs01_dn10, locals.var_fs01_dn13,)
    }
};
            locals.var_fs01 = assign84290_body45_e128579;
            locals.var_fs01_dn0 = assign84290_body45_e128579_d_n0;
            locals.var_fs01_dn2 = assign84290_body45_e128579_d_n2;
            locals.var_fs01_dn4 = assign84290_body45_e128579_d_n4;
            locals.var_fs01_dn5 = assign84290_body45_e128579_d_n5;
            locals.var_fs01_dn6 = assign84290_body45_e128579_d_n6;
            locals.var_fs01_dn7 = assign84290_body45_e128579_d_n7;
            locals.var_fs01_dn8 = assign84290_body45_e128579_d_n8;
            locals.var_fs01_dn9 = assign84290_body45_e128579_d_n9;
            locals.var_fs01_dn10 = assign84290_body45_e128579_d_n10;
            locals.var_fs01_dn13 = assign84290_body45_e128579_d_n13;
            let (assign84290_body46_e128590, assign84290_body46_e128590_d_n0, assign84290_body46_e128590_d_n2, assign84290_body46_e128590_d_n4, assign84290_body46_e128590_d_n5, assign84290_body46_e128590_d_n6, assign84290_body46_e128590_d_n7, assign84290_body46_e128590_d_n8, assign84290_body46_e128590_d_n9, assign84290_body46_e128590_d_n10, assign84290_body46_e128590_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1946 != 0.0)) && (locals.var_guard1956 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fs01_dps0, locals.var_fs01_dps0_dn0, locals.var_fs01_dps0_dn2, locals.var_fs01_dps0_dn4, locals.var_fs01_dps0_dn5, locals.var_fs01_dps0_dn6, locals.var_fs01_dps0_dn7, locals.var_fs01_dps0_dn8, locals.var_fs01_dps0_dn9, locals.var_fs01_dps0_dn10, locals.var_fs01_dps0_dn13,)
    }
};
            locals.var_fs01_dps0 = assign84290_body46_e128590;
            locals.var_fs01_dps0_dn0 = assign84290_body46_e128590_d_n0;
            locals.var_fs01_dps0_dn2 = assign84290_body46_e128590_d_n2;
            locals.var_fs01_dps0_dn4 = assign84290_body46_e128590_d_n4;
            locals.var_fs01_dps0_dn5 = assign84290_body46_e128590_d_n5;
            locals.var_fs01_dps0_dn6 = assign84290_body46_e128590_d_n6;
            locals.var_fs01_dps0_dn7 = assign84290_body46_e128590_d_n7;
            locals.var_fs01_dps0_dn8 = assign84290_body46_e128590_d_n8;
            locals.var_fs01_dps0_dn9 = assign84290_body46_e128590_d_n9;
            locals.var_fs01_dps0_dn10 = assign84290_body46_e128590_d_n10;
            locals.var_fs01_dps0_dn13 = assign84290_body46_e128590_d_n13;
            let (assign84290_body47_e128602, assign84290_body47_e128602_d_n0, assign84290_body47_e128602_d_n2, assign84290_body47_e128602_d_n4, assign84290_body47_e128602_d_n5, assign84290_body47_e128602_d_n6, assign84290_body47_e128602_d_n7, assign84290_body47_e128602_d_n8, assign84290_body47_e128602_d_n9, assign84290_body47_e128602_d_n10, assign84290_body47_e128602_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1946 != 0.0)) && (locals.var_guard1956 != 0.0)) {
        let assign84290_body47_e128600: f64 = (-locals.var_fb);
        (assign84290_body47_e128600, (-locals.var_fb_dn0), (-locals.var_fb_dn2), (-locals.var_fb_dn4), (-locals.var_fb_dn5), (-locals.var_fb_dn6), (-locals.var_fb_dn7), (-locals.var_fb_dn8), (-locals.var_fb_dn9), (-locals.var_fb_dn10), (-locals.var_fb_dn13),)
    } else {
        (locals.var_fs02, locals.var_fs02_dn0, locals.var_fs02_dn2, locals.var_fs02_dn4, locals.var_fs02_dn5, locals.var_fs02_dn6, locals.var_fs02_dn7, locals.var_fs02_dn8, locals.var_fs02_dn9, locals.var_fs02_dn10, locals.var_fs02_dn13,)
    }
};
            locals.var_fs02 = assign84290_body47_e128602;
            locals.var_fs02_dn0 = assign84290_body47_e128602_d_n0;
            locals.var_fs02_dn2 = assign84290_body47_e128602_d_n2;
            locals.var_fs02_dn4 = assign84290_body47_e128602_d_n4;
            locals.var_fs02_dn5 = assign84290_body47_e128602_d_n5;
            locals.var_fs02_dn6 = assign84290_body47_e128602_d_n6;
            locals.var_fs02_dn7 = assign84290_body47_e128602_d_n7;
            locals.var_fs02_dn8 = assign84290_body47_e128602_d_n8;
            locals.var_fs02_dn9 = assign84290_body47_e128602_d_n9;
            locals.var_fs02_dn10 = assign84290_body47_e128602_d_n10;
            locals.var_fs02_dn13 = assign84290_body47_e128602_d_n13;
            let (assign84290_body48_e128614, assign84290_body48_e128614_d_n0, assign84290_body48_e128614_d_n2, assign84290_body48_e128614_d_n4, assign84290_body48_e128614_d_n5, assign84290_body48_e128614_d_n6, assign84290_body48_e128614_d_n7, assign84290_body48_e128614_d_n8, assign84290_body48_e128614_d_n9, assign84290_body48_e128614_d_n10, assign84290_body48_e128614_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1946 != 0.0)) && (locals.var_guard1956 != 0.0)) {
        let assign84290_body48_e128612: f64 = (-locals.var_fb_dpss);
        (assign84290_body48_e128612, (-locals.var_fb_dpss_dn0), (-locals.var_fb_dpss_dn2), (-locals.var_fb_dpss_dn4), (-locals.var_fb_dpss_dn5), (-locals.var_fb_dpss_dn6), (-locals.var_fb_dpss_dn7), (-locals.var_fb_dpss_dn8), (-locals.var_fb_dpss_dn9), (-locals.var_fb_dpss_dn10), (-locals.var_fb_dpss_dn13),)
    } else {
        (locals.var_fs02_dps0, locals.var_fs02_dps0_dn0, locals.var_fs02_dps0_dn2, locals.var_fs02_dps0_dn4, locals.var_fs02_dps0_dn5, locals.var_fs02_dps0_dn6, locals.var_fs02_dps0_dn7, locals.var_fs02_dps0_dn8, locals.var_fs02_dps0_dn9, locals.var_fs02_dps0_dn10, locals.var_fs02_dps0_dn13,)
    }
};
            locals.var_fs02_dps0 = assign84290_body48_e128614;
            locals.var_fs02_dps0_dn0 = assign84290_body48_e128614_d_n0;
            locals.var_fs02_dps0_dn2 = assign84290_body48_e128614_d_n2;
            locals.var_fs02_dps0_dn4 = assign84290_body48_e128614_d_n4;
            locals.var_fs02_dps0_dn5 = assign84290_body48_e128614_d_n5;
            locals.var_fs02_dps0_dn6 = assign84290_body48_e128614_d_n6;
            locals.var_fs02_dps0_dn7 = assign84290_body48_e128614_d_n7;
            locals.var_fs02_dps0_dn8 = assign84290_body48_e128614_d_n8;
            locals.var_fs02_dps0_dn9 = assign84290_body48_e128614_d_n9;
            locals.var_fs02_dps0_dn10 = assign84290_body48_e128614_d_n10;
            locals.var_fs02_dps0_dn13 = assign84290_body48_e128614_d_n13;
            let assign84290_body49_e128617: f64 = if locals.var_chi < 60.0 { 1.0 } else { 0.0 };
            locals.var_guard1957 = assign84290_body49_e128617;
            let assign84290_body50_e128620: f64 = if locals.var_chi < 5e-5 { 1.0 } else { 0.0 };
            locals.var_guard1958 = assign84290_body50_e128620;
            let (assign84290_body51_e128658, assign84290_body51_e128658_d_n0, assign84290_body51_e128658_d_n2, assign84290_body51_e128658_d_n4, assign84290_body51_e128658_d_n5, assign84290_body51_e128658_d_n6, assign84290_body51_e128658_d_n7, assign84290_body51_e128658_d_n8, assign84290_body51_e128658_d_n9, assign84290_body51_e128658_d_n10, assign84290_body51_e128658_d_n13,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1946 != 0.0)) && (locals.var_guard1956 == 0.0)) && (locals.var_guard1957 != 0.0)) && (locals.var_guard1958 != 0.0)) {
        let assign84290_body51_e128636: f64 = (locals.var_chi * locals.var_chi);
        let assign84290_body51_e128638: f64 = (assign84290_body51_e128636 / 2.0);
        let assign84290_body51_e128642: f64 = (locals.var_chi / 3.0);
        let assign84290_body51_e128646: f64 = (locals.var_chi / 4.0);
        let assign84290_body51_e128650: f64 = (locals.var_chi / 5.0);
        let assign84290_body51_e128651: f64 = (1.0 + assign84290_body51_e128650);
        let assign84290_body51_e128652: f64 = (assign84290_body51_e128646 * assign84290_body51_e128651);
        let assign84290_body51_e128653: f64 = (1.0 + assign84290_body51_e128652);
        let assign84290_body51_e128654: f64 = (assign84290_body51_e128642 * assign84290_body51_e128653);
        let assign84290_body51_e128655: f64 = (1.0 + assign84290_body51_e128654);
        let assign84290_body51_e128656: f64 = (assign84290_body51_e128638 * assign84290_body51_e128655);
        (assign84290_body51_e128656, (((((locals.var_chi_dn0 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn0)) / 2.0) * assign84290_body51_e128655) + (assign84290_body51_e128638 * (((locals.var_chi_dn0 / 3.0) * assign84290_body51_e128653) + (assign84290_body51_e128642 * (((locals.var_chi_dn0 / 4.0) * assign84290_body51_e128651) + (assign84290_body51_e128646 * (locals.var_chi_dn0 / 5.0))))))), (((((locals.var_chi_dn2 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn2)) / 2.0) * assign84290_body51_e128655) + (assign84290_body51_e128638 * (((locals.var_chi_dn2 / 3.0) * assign84290_body51_e128653) + (assign84290_body51_e128642 * (((locals.var_chi_dn2 / 4.0) * assign84290_body51_e128651) + (assign84290_body51_e128646 * (locals.var_chi_dn2 / 5.0))))))), (((((locals.var_chi_dn4 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn4)) / 2.0) * assign84290_body51_e128655) + (assign84290_body51_e128638 * (((locals.var_chi_dn4 / 3.0) * assign84290_body51_e128653) + (assign84290_body51_e128642 * (((locals.var_chi_dn4 / 4.0) * assign84290_body51_e128651) + (assign84290_body51_e128646 * (locals.var_chi_dn4 / 5.0))))))), (((((locals.var_chi_dn5 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn5)) / 2.0) * assign84290_body51_e128655) + (assign84290_body51_e128638 * (((locals.var_chi_dn5 / 3.0) * assign84290_body51_e128653) + (assign84290_body51_e128642 * (((locals.var_chi_dn5 / 4.0) * assign84290_body51_e128651) + (assign84290_body51_e128646 * (locals.var_chi_dn5 / 5.0))))))), (((((locals.var_chi_dn6 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn6)) / 2.0) * assign84290_body51_e128655) + (assign84290_body51_e128638 * (((locals.var_chi_dn6 / 3.0) * assign84290_body51_e128653) + (assign84290_body51_e128642 * (((locals.var_chi_dn6 / 4.0) * assign84290_body51_e128651) + (assign84290_body51_e128646 * (locals.var_chi_dn6 / 5.0))))))), (((((locals.var_chi_dn7 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn7)) / 2.0) * assign84290_body51_e128655) + (assign84290_body51_e128638 * (((locals.var_chi_dn7 / 3.0) * assign84290_body51_e128653) + (assign84290_body51_e128642 * (((locals.var_chi_dn7 / 4.0) * assign84290_body51_e128651) + (assign84290_body51_e128646 * (locals.var_chi_dn7 / 5.0))))))), (((((locals.var_chi_dn8 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn8)) / 2.0) * assign84290_body51_e128655) + (assign84290_body51_e128638 * (((locals.var_chi_dn8 / 3.0) * assign84290_body51_e128653) + (assign84290_body51_e128642 * (((locals.var_chi_dn8 / 4.0) * assign84290_body51_e128651) + (assign84290_body51_e128646 * (locals.var_chi_dn8 / 5.0))))))), (((((locals.var_chi_dn9 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn9)) / 2.0) * assign84290_body51_e128655) + (assign84290_body51_e128638 * (((locals.var_chi_dn9 / 3.0) * assign84290_body51_e128653) + (assign84290_body51_e128642 * (((locals.var_chi_dn9 / 4.0) * assign84290_body51_e128651) + (assign84290_body51_e128646 * (locals.var_chi_dn9 / 5.0))))))), (((((locals.var_chi_dn10 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn10)) / 2.0) * assign84290_body51_e128655) + (assign84290_body51_e128638 * (((locals.var_chi_dn10 / 3.0) * assign84290_body51_e128653) + (assign84290_body51_e128642 * (((locals.var_chi_dn10 / 4.0) * assign84290_body51_e128651) + (assign84290_body51_e128646 * (locals.var_chi_dn10 / 5.0))))))), (((((locals.var_chi_dn13 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn13)) / 2.0) * assign84290_body51_e128655) + (assign84290_body51_e128638 * (((locals.var_chi_dn13 / 3.0) * assign84290_body51_e128653) + (assign84290_body51_e128642 * (((locals.var_chi_dn13 / 4.0) * assign84290_body51_e128651) + (assign84290_body51_e128646 * (locals.var_chi_dn13 / 5.0))))))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
            locals.var_t0 = assign84290_body51_e128658;
            locals.var_t0_dn0 = assign84290_body51_e128658_d_n0;
            locals.var_t0_dn2 = assign84290_body51_e128658_d_n2;
            locals.var_t0_dn4 = assign84290_body51_e128658_d_n4;
            locals.var_t0_dn5 = assign84290_body51_e128658_d_n5;
            locals.var_t0_dn6 = assign84290_body51_e128658_d_n6;
            locals.var_t0_dn7 = assign84290_body51_e128658_d_n7;
            locals.var_t0_dn8 = assign84290_body51_e128658_d_n8;
            locals.var_t0_dn9 = assign84290_body51_e128658_d_n9;
            locals.var_t0_dn10 = assign84290_body51_e128658_d_n10;
            locals.var_t0_dn13 = assign84290_body51_e128658_d_n13;
            let (assign84290_body52_e128692, assign84290_body52_e128692_d_n0, assign84290_body52_e128692_d_n2, assign84290_body52_e128692_d_n4, assign84290_body52_e128692_d_n5, assign84290_body52_e128692_d_n6, assign84290_body52_e128692_d_n7, assign84290_body52_e128692_d_n8, assign84290_body52_e128692_d_n9, assign84290_body52_e128692_d_n10, assign84290_body52_e128692_d_n13,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1946 != 0.0)) && (locals.var_guard1956 == 0.0)) && (locals.var_guard1957 != 0.0)) && (locals.var_guard1958 != 0.0)) {
        let assign84290_body52_e128676: f64 = (locals.var_chi / 2.0);
        let assign84290_body52_e128680: f64 = (locals.var_chi / 3.0);
        let assign84290_body52_e128684: f64 = (locals.var_chi / 4.0);
        let assign84290_body52_e128685: f64 = (1.0 + assign84290_body52_e128684);
        let assign84290_body52_e128686: f64 = (assign84290_body52_e128680 * assign84290_body52_e128685);
        let assign84290_body52_e128687: f64 = (1.0 + assign84290_body52_e128686);
        let assign84290_body52_e128688: f64 = (assign84290_body52_e128676 * assign84290_body52_e128687);
        let assign84290_body52_e128689: f64 = (1.0 + assign84290_body52_e128688);
        let assign84290_body52_e128690: f64 = (locals.var_chi * assign84290_body52_e128689);
        (assign84290_body52_e128690, ((locals.var_chi_dn0 * assign84290_body52_e128689) + (locals.var_chi * (((locals.var_chi_dn0 / 2.0) * assign84290_body52_e128687) + (assign84290_body52_e128676 * (((locals.var_chi_dn0 / 3.0) * assign84290_body52_e128685) + (assign84290_body52_e128680 * (locals.var_chi_dn0 / 4.0))))))), ((locals.var_chi_dn2 * assign84290_body52_e128689) + (locals.var_chi * (((locals.var_chi_dn2 / 2.0) * assign84290_body52_e128687) + (assign84290_body52_e128676 * (((locals.var_chi_dn2 / 3.0) * assign84290_body52_e128685) + (assign84290_body52_e128680 * (locals.var_chi_dn2 / 4.0))))))), ((locals.var_chi_dn4 * assign84290_body52_e128689) + (locals.var_chi * (((locals.var_chi_dn4 / 2.0) * assign84290_body52_e128687) + (assign84290_body52_e128676 * (((locals.var_chi_dn4 / 3.0) * assign84290_body52_e128685) + (assign84290_body52_e128680 * (locals.var_chi_dn4 / 4.0))))))), ((locals.var_chi_dn5 * assign84290_body52_e128689) + (locals.var_chi * (((locals.var_chi_dn5 / 2.0) * assign84290_body52_e128687) + (assign84290_body52_e128676 * (((locals.var_chi_dn5 / 3.0) * assign84290_body52_e128685) + (assign84290_body52_e128680 * (locals.var_chi_dn5 / 4.0))))))), ((locals.var_chi_dn6 * assign84290_body52_e128689) + (locals.var_chi * (((locals.var_chi_dn6 / 2.0) * assign84290_body52_e128687) + (assign84290_body52_e128676 * (((locals.var_chi_dn6 / 3.0) * assign84290_body52_e128685) + (assign84290_body52_e128680 * (locals.var_chi_dn6 / 4.0))))))), ((locals.var_chi_dn7 * assign84290_body52_e128689) + (locals.var_chi * (((locals.var_chi_dn7 / 2.0) * assign84290_body52_e128687) + (assign84290_body52_e128676 * (((locals.var_chi_dn7 / 3.0) * assign84290_body52_e128685) + (assign84290_body52_e128680 * (locals.var_chi_dn7 / 4.0))))))), ((locals.var_chi_dn8 * assign84290_body52_e128689) + (locals.var_chi * (((locals.var_chi_dn8 / 2.0) * assign84290_body52_e128687) + (assign84290_body52_e128676 * (((locals.var_chi_dn8 / 3.0) * assign84290_body52_e128685) + (assign84290_body52_e128680 * (locals.var_chi_dn8 / 4.0))))))), ((locals.var_chi_dn9 * assign84290_body52_e128689) + (locals.var_chi * (((locals.var_chi_dn9 / 2.0) * assign84290_body52_e128687) + (assign84290_body52_e128676 * (((locals.var_chi_dn9 / 3.0) * assign84290_body52_e128685) + (assign84290_body52_e128680 * (locals.var_chi_dn9 / 4.0))))))), ((locals.var_chi_dn10 * assign84290_body52_e128689) + (locals.var_chi * (((locals.var_chi_dn10 / 2.0) * assign84290_body52_e128687) + (assign84290_body52_e128676 * (((locals.var_chi_dn10 / 3.0) * assign84290_body52_e128685) + (assign84290_body52_e128680 * (locals.var_chi_dn10 / 4.0))))))), ((locals.var_chi_dn13 * assign84290_body52_e128689) + (locals.var_chi * (((locals.var_chi_dn13 / 2.0) * assign84290_body52_e128687) + (assign84290_body52_e128676 * (((locals.var_chi_dn13 / 3.0) * assign84290_body52_e128685) + (assign84290_body52_e128680 * (locals.var_chi_dn13 / 4.0))))))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
            locals.var_t1 = assign84290_body52_e128692;
            locals.var_t1_dn0 = assign84290_body52_e128692_d_n0;
            locals.var_t1_dn2 = assign84290_body52_e128692_d_n2;
            locals.var_t1_dn4 = assign84290_body52_e128692_d_n4;
            locals.var_t1_dn5 = assign84290_body52_e128692_d_n5;
            locals.var_t1_dn6 = assign84290_body52_e128692_d_n6;
            locals.var_t1_dn7 = assign84290_body52_e128692_d_n7;
            locals.var_t1_dn8 = assign84290_body52_e128692_d_n8;
            locals.var_t1_dn9 = assign84290_body52_e128692_d_n9;
            locals.var_t1_dn10 = assign84290_body52_e128692_d_n10;
            locals.var_t1_dn13 = assign84290_body52_e128692_d_n13;
            let (assign84290_body53_e128710, assign84290_body53_e128710_d_n0, assign84290_body53_e128710_d_n2, assign84290_body53_e128710_d_n4, assign84290_body53_e128710_d_n5, assign84290_body53_e128710_d_n6, assign84290_body53_e128710_d_n7, assign84290_body53_e128710_d_n8, assign84290_body53_e128710_d_n9, assign84290_body53_e128710_d_n10, assign84290_body53_e128710_d_n13,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1946 != 0.0)) && (locals.var_guard1956 == 0.0)) && (locals.var_guard1957 != 0.0)) && (locals.var_guard1958 != 0.0)) {
        let assign84290_body53_e128708: f64 = (locals.var_cfs1 * locals.var_t0);
        (assign84290_body53_e128708, ((locals.var_cfs1_dn0 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn0)), ((locals.var_cfs1_dn2 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn2)), ((locals.var_cfs1_dn4 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn4)), ((locals.var_cfs1_dn5 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn5)), ((locals.var_cfs1_dn6 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn6)), ((locals.var_cfs1_dn7 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn7)), ((locals.var_cfs1_dn8 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn8)), ((locals.var_cfs1_dn9 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn9)), ((locals.var_cfs1_dn10 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn10)), ((locals.var_cfs1_dn13 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn13)),)
    } else {
        (locals.var_fs01, locals.var_fs01_dn0, locals.var_fs01_dn2, locals.var_fs01_dn4, locals.var_fs01_dn5, locals.var_fs01_dn6, locals.var_fs01_dn7, locals.var_fs01_dn8, locals.var_fs01_dn9, locals.var_fs01_dn10, locals.var_fs01_dn13,)
    }
};
            locals.var_fs01 = assign84290_body53_e128710;
            locals.var_fs01_dn0 = assign84290_body53_e128710_d_n0;
            locals.var_fs01_dn2 = assign84290_body53_e128710_d_n2;
            locals.var_fs01_dn4 = assign84290_body53_e128710_d_n4;
            locals.var_fs01_dn5 = assign84290_body53_e128710_d_n5;
            locals.var_fs01_dn6 = assign84290_body53_e128710_d_n6;
            locals.var_fs01_dn7 = assign84290_body53_e128710_d_n7;
            locals.var_fs01_dn8 = assign84290_body53_e128710_d_n8;
            locals.var_fs01_dn9 = assign84290_body53_e128710_d_n9;
            locals.var_fs01_dn10 = assign84290_body53_e128710_d_n10;
            locals.var_fs01_dn13 = assign84290_body53_e128710_d_n13;
            let (assign84290_body54_e128730, assign84290_body54_e128730_d_n0, assign84290_body54_e128730_d_n2, assign84290_body54_e128730_d_n4, assign84290_body54_e128730_d_n5, assign84290_body54_e128730_d_n6, assign84290_body54_e128730_d_n7, assign84290_body54_e128730_d_n8, assign84290_body54_e128730_d_n9, assign84290_body54_e128730_d_n10, assign84290_body54_e128730_d_n13,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1946 != 0.0)) && (locals.var_guard1956 == 0.0)) && (locals.var_guard1957 != 0.0)) && (locals.var_guard1958 != 0.0)) {
        let assign84290_body54_e128726: f64 = (locals.var_cfs1 * locals.var_t1);
        let assign84290_body54_e128728: f64 = (assign84290_body54_e128726 * locals.var_beta);
        (assign84290_body54_e128728, ((((locals.var_cfs1_dn0 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn0)) * locals.var_beta) + (assign84290_body54_e128726 * locals.var_beta_dn0)), ((((locals.var_cfs1_dn2 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn2)) * locals.var_beta) + (assign84290_body54_e128726 * locals.var_beta_dn2)), ((((locals.var_cfs1_dn4 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn4)) * locals.var_beta) + (assign84290_body54_e128726 * locals.var_beta_dn4)), ((((locals.var_cfs1_dn5 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn5)) * locals.var_beta) + (assign84290_body54_e128726 * locals.var_beta_dn5)), ((((locals.var_cfs1_dn6 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn6)) * locals.var_beta) + (assign84290_body54_e128726 * locals.var_beta_dn6)), ((((locals.var_cfs1_dn7 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn7)) * locals.var_beta) + (assign84290_body54_e128726 * locals.var_beta_dn7)), ((((locals.var_cfs1_dn8 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn8)) * locals.var_beta) + (assign84290_body54_e128726 * locals.var_beta_dn8)), ((((locals.var_cfs1_dn9 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn9)) * locals.var_beta) + (assign84290_body54_e128726 * locals.var_beta_dn9)), ((((locals.var_cfs1_dn10 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn10)) * locals.var_beta) + (assign84290_body54_e128726 * locals.var_beta_dn10)), ((((locals.var_cfs1_dn13 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn13)) * locals.var_beta) + (assign84290_body54_e128726 * locals.var_beta_dn13)),)
    } else {
        (locals.var_fs01_dps0, locals.var_fs01_dps0_dn0, locals.var_fs01_dps0_dn2, locals.var_fs01_dps0_dn4, locals.var_fs01_dps0_dn5, locals.var_fs01_dps0_dn6, locals.var_fs01_dps0_dn7, locals.var_fs01_dps0_dn8, locals.var_fs01_dps0_dn9, locals.var_fs01_dps0_dn10, locals.var_fs01_dps0_dn13,)
    }
};
            locals.var_fs01_dps0 = assign84290_body54_e128730;
            locals.var_fs01_dps0_dn0 = assign84290_body54_e128730_d_n0;
            locals.var_fs01_dps0_dn2 = assign84290_body54_e128730_d_n2;
            locals.var_fs01_dps0_dn4 = assign84290_body54_e128730_d_n4;
            locals.var_fs01_dps0_dn5 = assign84290_body54_e128730_d_n5;
            locals.var_fs01_dps0_dn6 = assign84290_body54_e128730_d_n6;
            locals.var_fs01_dps0_dn7 = assign84290_body54_e128730_d_n7;
            locals.var_fs01_dps0_dn8 = assign84290_body54_e128730_d_n8;
            locals.var_fs01_dps0_dn9 = assign84290_body54_e128730_d_n9;
            locals.var_fs01_dps0_dn10 = assign84290_body54_e128730_d_n10;
            locals.var_fs01_dps0_dn13 = assign84290_body54_e128730_d_n13;
            let (assign84290_body55_e128748, assign84290_body55_e128748_d_n0, assign84290_body55_e128748_d_n2, assign84290_body55_e128748_d_n4, assign84290_body55_e128748_d_n5, assign84290_body55_e128748_d_n6, assign84290_body55_e128748_d_n7, assign84290_body55_e128748_d_n8, assign84290_body55_e128748_d_n9, assign84290_body55_e128748_d_n10, assign84290_body55_e128748_d_n13,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1946 != 0.0)) && (locals.var_guard1956 == 0.0)) && (locals.var_guard1957 != 0.0)) && (locals.var_guard1958 == 0.0)) {
        let assign84290_body55_e128746: f64 = (locals.var_chi).exp();
        (assign84290_body55_e128746, (assign84290_body55_e128746 * locals.var_chi_dn0), (assign84290_body55_e128746 * locals.var_chi_dn2), (assign84290_body55_e128746 * locals.var_chi_dn4), (assign84290_body55_e128746 * locals.var_chi_dn5), (assign84290_body55_e128746 * locals.var_chi_dn6), (assign84290_body55_e128746 * locals.var_chi_dn7), (assign84290_body55_e128746 * locals.var_chi_dn8), (assign84290_body55_e128746 * locals.var_chi_dn9), (assign84290_body55_e128746 * locals.var_chi_dn10), (assign84290_body55_e128746 * locals.var_chi_dn13),)
    } else {
        (locals.var_exp_chi, locals.var_exp_chi_dn0, locals.var_exp_chi_dn2, locals.var_exp_chi_dn4, locals.var_exp_chi_dn5, locals.var_exp_chi_dn6, locals.var_exp_chi_dn7, locals.var_exp_chi_dn8, locals.var_exp_chi_dn9, locals.var_exp_chi_dn10, locals.var_exp_chi_dn13,)
    }
};
            locals.var_exp_chi = assign84290_body55_e128748;
            locals.var_exp_chi_dn0 = assign84290_body55_e128748_d_n0;
            locals.var_exp_chi_dn2 = assign84290_body55_e128748_d_n2;
            locals.var_exp_chi_dn4 = assign84290_body55_e128748_d_n4;
            locals.var_exp_chi_dn5 = assign84290_body55_e128748_d_n5;
            locals.var_exp_chi_dn6 = assign84290_body55_e128748_d_n6;
            locals.var_exp_chi_dn7 = assign84290_body55_e128748_d_n7;
            locals.var_exp_chi_dn8 = assign84290_body55_e128748_d_n8;
            locals.var_exp_chi_dn9 = assign84290_body55_e128748_d_n9;
            locals.var_exp_chi_dn10 = assign84290_body55_e128748_d_n10;
            locals.var_exp_chi_dn13 = assign84290_body55_e128748_d_n13;
            let (assign84290_body56_e128767, assign84290_body56_e128767_d_n0, assign84290_body56_e128767_d_n2, assign84290_body56_e128767_d_n4, assign84290_body56_e128767_d_n5, assign84290_body56_e128767_d_n6, assign84290_body56_e128767_d_n7, assign84290_body56_e128767_d_n8, assign84290_body56_e128767_d_n9, assign84290_body56_e128767_d_n10, assign84290_body56_e128767_d_n13,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1946 != 0.0)) && (locals.var_guard1956 == 0.0)) && (locals.var_guard1957 != 0.0)) && (locals.var_guard1958 == 0.0)) {
        let assign84290_body56_e128765: f64 = (locals.var_exp_chi - 1.0);
        (assign84290_body56_e128765, locals.var_exp_chi_dn0, locals.var_exp_chi_dn2, locals.var_exp_chi_dn4, locals.var_exp_chi_dn5, locals.var_exp_chi_dn6, locals.var_exp_chi_dn7, locals.var_exp_chi_dn8, locals.var_exp_chi_dn9, locals.var_exp_chi_dn10, locals.var_exp_chi_dn13,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
            locals.var_t1 = assign84290_body56_e128767;
            locals.var_t1_dn0 = assign84290_body56_e128767_d_n0;
            locals.var_t1_dn2 = assign84290_body56_e128767_d_n2;
            locals.var_t1_dn4 = assign84290_body56_e128767_d_n4;
            locals.var_t1_dn5 = assign84290_body56_e128767_d_n5;
            locals.var_t1_dn6 = assign84290_body56_e128767_d_n6;
            locals.var_t1_dn7 = assign84290_body56_e128767_d_n7;
            locals.var_t1_dn8 = assign84290_body56_e128767_d_n8;
            locals.var_t1_dn9 = assign84290_body56_e128767_d_n9;
            locals.var_t1_dn10 = assign84290_body56_e128767_d_n10;
            locals.var_t1_dn13 = assign84290_body56_e128767_d_n13;
            let (assign84290_body57_e128788, assign84290_body57_e128788_d_n0, assign84290_body57_e128788_d_n2, assign84290_body57_e128788_d_n4, assign84290_body57_e128788_d_n5, assign84290_body57_e128788_d_n6, assign84290_body57_e128788_d_n7, assign84290_body57_e128788_d_n8, assign84290_body57_e128788_d_n9, assign84290_body57_e128788_d_n10, assign84290_body57_e128788_d_n13,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1946 != 0.0)) && (locals.var_guard1956 == 0.0)) && (locals.var_guard1957 != 0.0)) && (locals.var_guard1958 == 0.0)) {
        let assign84290_body57_e128785: f64 = (locals.var_t1 - locals.var_chi);
        let assign84290_body57_e128786: f64 = (locals.var_cfs1 * assign84290_body57_e128785);
        (assign84290_body57_e128786, ((locals.var_cfs1_dn0 * assign84290_body57_e128785) + (locals.var_cfs1 * (locals.var_t1_dn0 - locals.var_chi_dn0))), ((locals.var_cfs1_dn2 * assign84290_body57_e128785) + (locals.var_cfs1 * (locals.var_t1_dn2 - locals.var_chi_dn2))), ((locals.var_cfs1_dn4 * assign84290_body57_e128785) + (locals.var_cfs1 * (locals.var_t1_dn4 - locals.var_chi_dn4))), ((locals.var_cfs1_dn5 * assign84290_body57_e128785) + (locals.var_cfs1 * (locals.var_t1_dn5 - locals.var_chi_dn5))), ((locals.var_cfs1_dn6 * assign84290_body57_e128785) + (locals.var_cfs1 * (locals.var_t1_dn6 - locals.var_chi_dn6))), ((locals.var_cfs1_dn7 * assign84290_body57_e128785) + (locals.var_cfs1 * (locals.var_t1_dn7 - locals.var_chi_dn7))), ((locals.var_cfs1_dn8 * assign84290_body57_e128785) + (locals.var_cfs1 * (locals.var_t1_dn8 - locals.var_chi_dn8))), ((locals.var_cfs1_dn9 * assign84290_body57_e128785) + (locals.var_cfs1 * (locals.var_t1_dn9 - locals.var_chi_dn9))), ((locals.var_cfs1_dn10 * assign84290_body57_e128785) + (locals.var_cfs1 * (locals.var_t1_dn10 - locals.var_chi_dn10))), ((locals.var_cfs1_dn13 * assign84290_body57_e128785) + (locals.var_cfs1 * (locals.var_t1_dn13 - locals.var_chi_dn13))),)
    } else {
        (locals.var_fs01, locals.var_fs01_dn0, locals.var_fs01_dn2, locals.var_fs01_dn4, locals.var_fs01_dn5, locals.var_fs01_dn6, locals.var_fs01_dn7, locals.var_fs01_dn8, locals.var_fs01_dn9, locals.var_fs01_dn10, locals.var_fs01_dn13,)
    }
};
            locals.var_fs01 = assign84290_body57_e128788;
            locals.var_fs01_dn0 = assign84290_body57_e128788_d_n0;
            locals.var_fs01_dn2 = assign84290_body57_e128788_d_n2;
            locals.var_fs01_dn4 = assign84290_body57_e128788_d_n4;
            locals.var_fs01_dn5 = assign84290_body57_e128788_d_n5;
            locals.var_fs01_dn6 = assign84290_body57_e128788_d_n6;
            locals.var_fs01_dn7 = assign84290_body57_e128788_d_n7;
            locals.var_fs01_dn8 = assign84290_body57_e128788_d_n8;
            locals.var_fs01_dn9 = assign84290_body57_e128788_d_n9;
            locals.var_fs01_dn10 = assign84290_body57_e128788_d_n10;
            locals.var_fs01_dn13 = assign84290_body57_e128788_d_n13;
            let (assign84290_body58_e128809, assign84290_body58_e128809_d_n0, assign84290_body58_e128809_d_n2, assign84290_body58_e128809_d_n4, assign84290_body58_e128809_d_n5, assign84290_body58_e128809_d_n6, assign84290_body58_e128809_d_n7, assign84290_body58_e128809_d_n8, assign84290_body58_e128809_d_n9, assign84290_body58_e128809_d_n10, assign84290_body58_e128809_d_n13,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1946 != 0.0)) && (locals.var_guard1956 == 0.0)) && (locals.var_guard1957 != 0.0)) && (locals.var_guard1958 == 0.0)) {
        let assign84290_body58_e128805: f64 = (locals.var_cfs1 * locals.var_beta);
        let assign84290_body58_e128807: f64 = (assign84290_body58_e128805 * locals.var_t1);
        (assign84290_body58_e128807, ((((locals.var_cfs1_dn0 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn0)) * locals.var_t1) + (assign84290_body58_e128805 * locals.var_t1_dn0)), ((((locals.var_cfs1_dn2 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn2)) * locals.var_t1) + (assign84290_body58_e128805 * locals.var_t1_dn2)), ((((locals.var_cfs1_dn4 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn4)) * locals.var_t1) + (assign84290_body58_e128805 * locals.var_t1_dn4)), ((((locals.var_cfs1_dn5 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn5)) * locals.var_t1) + (assign84290_body58_e128805 * locals.var_t1_dn5)), ((((locals.var_cfs1_dn6 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn6)) * locals.var_t1) + (assign84290_body58_e128805 * locals.var_t1_dn6)), ((((locals.var_cfs1_dn7 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn7)) * locals.var_t1) + (assign84290_body58_e128805 * locals.var_t1_dn7)), ((((locals.var_cfs1_dn8 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn8)) * locals.var_t1) + (assign84290_body58_e128805 * locals.var_t1_dn8)), ((((locals.var_cfs1_dn9 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn9)) * locals.var_t1) + (assign84290_body58_e128805 * locals.var_t1_dn9)), ((((locals.var_cfs1_dn10 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn10)) * locals.var_t1) + (assign84290_body58_e128805 * locals.var_t1_dn10)), ((((locals.var_cfs1_dn13 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn13)) * locals.var_t1) + (assign84290_body58_e128805 * locals.var_t1_dn13)),)
    } else {
        (locals.var_fs01_dps0, locals.var_fs01_dps0_dn0, locals.var_fs01_dps0_dn2, locals.var_fs01_dps0_dn4, locals.var_fs01_dps0_dn5, locals.var_fs01_dps0_dn6, locals.var_fs01_dps0_dn7, locals.var_fs01_dps0_dn8, locals.var_fs01_dps0_dn9, locals.var_fs01_dps0_dn10, locals.var_fs01_dps0_dn13,)
    }
};
            locals.var_fs01_dps0 = assign84290_body58_e128809;
            locals.var_fs01_dps0_dn0 = assign84290_body58_e128809_d_n0;
            locals.var_fs01_dps0_dn2 = assign84290_body58_e128809_d_n2;
            locals.var_fs01_dps0_dn4 = assign84290_body58_e128809_d_n4;
            locals.var_fs01_dps0_dn5 = assign84290_body58_e128809_d_n5;
            locals.var_fs01_dps0_dn6 = assign84290_body58_e128809_d_n6;
            locals.var_fs01_dps0_dn7 = assign84290_body58_e128809_d_n7;
            locals.var_fs01_dps0_dn8 = assign84290_body58_e128809_d_n8;
            locals.var_fs01_dps0_dn9 = assign84290_body58_e128809_d_n9;
            locals.var_fs01_dps0_dn10 = assign84290_body58_e128809_d_n10;
            locals.var_fs01_dps0_dn13 = assign84290_body58_e128809_d_n13;
            let (assign84290_body60_e128844, assign84290_body60_e128844_d_n0, assign84290_body60_e128844_d_n2, assign84290_body60_e128844_d_n4, assign84290_body60_e128844_d_n5, assign84290_body60_e128844_d_n6, assign84290_body60_e128844_d_n7, assign84290_body60_e128844_d_n8, assign84290_body60_e128844_d_n9, assign84290_body60_e128844_d_n10, assign84290_body60_e128844_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1946 != 0.0)) && (locals.var_guard1956 == 0.0)) && (locals.var_guard1957 == 0.0)) {
        let assign84290_body60_e128841: f64 = (locals.var_beta * locals.var_ps0ld);
        let assign84290_body60_e128842: f64 = (assign84290_body60_e128841).exp();
        (assign84290_body60_e128842, (assign84290_body60_e128842 * ((locals.var_beta_dn0 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn0))), (assign84290_body60_e128842 * ((locals.var_beta_dn2 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn2))), (assign84290_body60_e128842 * ((locals.var_beta_dn4 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn4))), (assign84290_body60_e128842 * ((locals.var_beta_dn5 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn5))), (assign84290_body60_e128842 * ((locals.var_beta_dn6 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn6))), (assign84290_body60_e128842 * ((locals.var_beta_dn7 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn7))), (assign84290_body60_e128842 * ((locals.var_beta_dn8 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn8))), (assign84290_body60_e128842 * ((locals.var_beta_dn9 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn9))), (assign84290_body60_e128842 * ((locals.var_beta_dn10 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn10))), (assign84290_body60_e128842 * ((locals.var_beta_dn13 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn13))),)
    } else {
        (locals.var_exp_bps0, locals.var_exp_bps0_dn0, locals.var_exp_bps0_dn2, locals.var_exp_bps0_dn4, locals.var_exp_bps0_dn5, locals.var_exp_bps0_dn6, locals.var_exp_bps0_dn7, locals.var_exp_bps0_dn8, locals.var_exp_bps0_dn9, locals.var_exp_bps0_dn10, locals.var_exp_bps0_dn13,)
    }
};
            locals.var_exp_bps0 = assign84290_body60_e128844;
            locals.var_exp_bps0_dn0 = assign84290_body60_e128844_d_n0;
            locals.var_exp_bps0_dn2 = assign84290_body60_e128844_d_n2;
            locals.var_exp_bps0_dn4 = assign84290_body60_e128844_d_n4;
            locals.var_exp_bps0_dn5 = assign84290_body60_e128844_d_n5;
            locals.var_exp_bps0_dn6 = assign84290_body60_e128844_d_n6;
            locals.var_exp_bps0_dn7 = assign84290_body60_e128844_d_n7;
            locals.var_exp_bps0_dn8 = assign84290_body60_e128844_d_n8;
            locals.var_exp_bps0_dn9 = assign84290_body60_e128844_d_n9;
            locals.var_exp_bps0_dn10 = assign84290_body60_e128844_d_n10;
            locals.var_exp_bps0_dn13 = assign84290_body60_e128844_d_n13;
            let (assign84290_body61_e128867, assign84290_body61_e128867_d_n0, assign84290_body61_e128867_d_n2, assign84290_body61_e128867_d_n4, assign84290_body61_e128867_d_n5, assign84290_body61_e128867_d_n6, assign84290_body61_e128867_d_n7, assign84290_body61_e128867_d_n8, assign84290_body61_e128867_d_n9, assign84290_body61_e128867_d_n10, assign84290_body61_e128867_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1946 != 0.0)) && (locals.var_guard1956 == 0.0)) && (locals.var_guard1957 == 0.0)) {
        let assign84290_body61_e128862: f64 = (locals.var_chi + 1.0);
        let assign84290_body61_e128863: f64 = (locals.var_exp_bvbs * assign84290_body61_e128862);
        let assign84290_body61_e128864: f64 = (locals.var_exp_bps0 - assign84290_body61_e128863);
        let assign84290_body61_e128865: f64 = (locals.var_cnst1over * assign84290_body61_e128864);
        (assign84290_body61_e128865, ((locals.var_cnst1over_dn0 * assign84290_body61_e128864) + (locals.var_cnst1over * (locals.var_exp_bps0_dn0 - ((locals.var_exp_bvbs_dn0 * assign84290_body61_e128862) + (locals.var_exp_bvbs * locals.var_chi_dn0))))), ((locals.var_cnst1over_dn2 * assign84290_body61_e128864) + (locals.var_cnst1over * (locals.var_exp_bps0_dn2 - ((locals.var_exp_bvbs_dn2 * assign84290_body61_e128862) + (locals.var_exp_bvbs * locals.var_chi_dn2))))), ((locals.var_cnst1over_dn4 * assign84290_body61_e128864) + (locals.var_cnst1over * (locals.var_exp_bps0_dn4 - ((locals.var_exp_bvbs_dn4 * assign84290_body61_e128862) + (locals.var_exp_bvbs * locals.var_chi_dn4))))), ((locals.var_cnst1over_dn5 * assign84290_body61_e128864) + (locals.var_cnst1over * (locals.var_exp_bps0_dn5 - ((locals.var_exp_bvbs_dn5 * assign84290_body61_e128862) + (locals.var_exp_bvbs * locals.var_chi_dn5))))), ((locals.var_cnst1over_dn6 * assign84290_body61_e128864) + (locals.var_cnst1over * (locals.var_exp_bps0_dn6 - ((locals.var_exp_bvbs_dn6 * assign84290_body61_e128862) + (locals.var_exp_bvbs * locals.var_chi_dn6))))), ((locals.var_cnst1over_dn7 * assign84290_body61_e128864) + (locals.var_cnst1over * (locals.var_exp_bps0_dn7 - ((locals.var_exp_bvbs_dn7 * assign84290_body61_e128862) + (locals.var_exp_bvbs * locals.var_chi_dn7))))), ((locals.var_cnst1over_dn8 * assign84290_body61_e128864) + (locals.var_cnst1over * (locals.var_exp_bps0_dn8 - ((locals.var_exp_bvbs_dn8 * assign84290_body61_e128862) + (locals.var_exp_bvbs * locals.var_chi_dn8))))), ((locals.var_cnst1over_dn9 * assign84290_body61_e128864) + (locals.var_cnst1over * (locals.var_exp_bps0_dn9 - ((locals.var_exp_bvbs_dn9 * assign84290_body61_e128862) + (locals.var_exp_bvbs * locals.var_chi_dn9))))), ((locals.var_cnst1over_dn10 * assign84290_body61_e128864) + (locals.var_cnst1over * (locals.var_exp_bps0_dn10 - ((locals.var_exp_bvbs_dn10 * assign84290_body61_e128862) + (locals.var_exp_bvbs * locals.var_chi_dn10))))), ((locals.var_cnst1over_dn13 * assign84290_body61_e128864) + (locals.var_cnst1over * (locals.var_exp_bps0_dn13 - ((locals.var_exp_bvbs_dn13 * assign84290_body61_e128862) + (locals.var_exp_bvbs * locals.var_chi_dn13))))),)
    } else {
        (locals.var_fs01, locals.var_fs01_dn0, locals.var_fs01_dn2, locals.var_fs01_dn4, locals.var_fs01_dn5, locals.var_fs01_dn6, locals.var_fs01_dn7, locals.var_fs01_dn8, locals.var_fs01_dn9, locals.var_fs01_dn10, locals.var_fs01_dn13,)
    }
};
            locals.var_fs01 = assign84290_body61_e128867;
            locals.var_fs01_dn0 = assign84290_body61_e128867_d_n0;
            locals.var_fs01_dn2 = assign84290_body61_e128867_d_n2;
            locals.var_fs01_dn4 = assign84290_body61_e128867_d_n4;
            locals.var_fs01_dn5 = assign84290_body61_e128867_d_n5;
            locals.var_fs01_dn6 = assign84290_body61_e128867_d_n6;
            locals.var_fs01_dn7 = assign84290_body61_e128867_d_n7;
            locals.var_fs01_dn8 = assign84290_body61_e128867_d_n8;
            locals.var_fs01_dn9 = assign84290_body61_e128867_d_n9;
            locals.var_fs01_dn10 = assign84290_body61_e128867_d_n10;
            locals.var_fs01_dn13 = assign84290_body61_e128867_d_n13;
            let (assign84290_body62_e128888, assign84290_body62_e128888_d_n0, assign84290_body62_e128888_d_n2, assign84290_body62_e128888_d_n4, assign84290_body62_e128888_d_n5, assign84290_body62_e128888_d_n6, assign84290_body62_e128888_d_n7, assign84290_body62_e128888_d_n8, assign84290_body62_e128888_d_n9, assign84290_body62_e128888_d_n10, assign84290_body62_e128888_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1946 != 0.0)) && (locals.var_guard1956 == 0.0)) && (locals.var_guard1957 == 0.0)) {
        let assign84290_body62_e128882: f64 = (locals.var_cnst1over * locals.var_beta);
        let assign84290_body62_e128885: f64 = (locals.var_exp_bps0 - locals.var_exp_bvbs);
        let assign84290_body62_e128886: f64 = (assign84290_body62_e128882 * assign84290_body62_e128885);
        (assign84290_body62_e128886, ((((locals.var_cnst1over_dn0 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn0)) * assign84290_body62_e128885) + (assign84290_body62_e128882 * (locals.var_exp_bps0_dn0 - locals.var_exp_bvbs_dn0))), ((((locals.var_cnst1over_dn2 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn2)) * assign84290_body62_e128885) + (assign84290_body62_e128882 * (locals.var_exp_bps0_dn2 - locals.var_exp_bvbs_dn2))), ((((locals.var_cnst1over_dn4 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn4)) * assign84290_body62_e128885) + (assign84290_body62_e128882 * (locals.var_exp_bps0_dn4 - locals.var_exp_bvbs_dn4))), ((((locals.var_cnst1over_dn5 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn5)) * assign84290_body62_e128885) + (assign84290_body62_e128882 * (locals.var_exp_bps0_dn5 - locals.var_exp_bvbs_dn5))), ((((locals.var_cnst1over_dn6 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn6)) * assign84290_body62_e128885) + (assign84290_body62_e128882 * (locals.var_exp_bps0_dn6 - locals.var_exp_bvbs_dn6))), ((((locals.var_cnst1over_dn7 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn7)) * assign84290_body62_e128885) + (assign84290_body62_e128882 * (locals.var_exp_bps0_dn7 - locals.var_exp_bvbs_dn7))), ((((locals.var_cnst1over_dn8 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn8)) * assign84290_body62_e128885) + (assign84290_body62_e128882 * (locals.var_exp_bps0_dn8 - locals.var_exp_bvbs_dn8))), ((((locals.var_cnst1over_dn9 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn9)) * assign84290_body62_e128885) + (assign84290_body62_e128882 * (locals.var_exp_bps0_dn9 - locals.var_exp_bvbs_dn9))), ((((locals.var_cnst1over_dn10 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn10)) * assign84290_body62_e128885) + (assign84290_body62_e128882 * (locals.var_exp_bps0_dn10 - locals.var_exp_bvbs_dn10))), ((((locals.var_cnst1over_dn13 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn13)) * assign84290_body62_e128885) + (assign84290_body62_e128882 * (locals.var_exp_bps0_dn13 - locals.var_exp_bvbs_dn13))),)
    } else {
        (locals.var_fs01_dps0, locals.var_fs01_dps0_dn0, locals.var_fs01_dps0_dn2, locals.var_fs01_dps0_dn4, locals.var_fs01_dps0_dn5, locals.var_fs01_dps0_dn6, locals.var_fs01_dps0_dn7, locals.var_fs01_dps0_dn8, locals.var_fs01_dps0_dn9, locals.var_fs01_dps0_dn10, locals.var_fs01_dps0_dn13,)
    }
};
            locals.var_fs01_dps0 = assign84290_body62_e128888;
            locals.var_fs01_dps0_dn0 = assign84290_body62_e128888_d_n0;
            locals.var_fs01_dps0_dn2 = assign84290_body62_e128888_d_n2;
            locals.var_fs01_dps0_dn4 = assign84290_body62_e128888_d_n4;
            locals.var_fs01_dps0_dn5 = assign84290_body62_e128888_d_n5;
            locals.var_fs01_dps0_dn6 = assign84290_body62_e128888_d_n6;
            locals.var_fs01_dps0_dn7 = assign84290_body62_e128888_d_n7;
            locals.var_fs01_dps0_dn8 = assign84290_body62_e128888_d_n8;
            locals.var_fs01_dps0_dn9 = assign84290_body62_e128888_d_n9;
            locals.var_fs01_dps0_dn10 = assign84290_body62_e128888_d_n10;
            locals.var_fs01_dps0_dn13 = assign84290_body62_e128888_d_n13;
            let assign84290_body63_e128891: f64 = if locals.var_fs01 > 0.0 { 1.0 } else { 0.0 };
            locals.var_guard1959 = assign84290_body63_e128891;
            let (assign84290_body64_e128910, assign84290_body64_e128910_d_n0, assign84290_body64_e128910_d_n2, assign84290_body64_e128910_d_n4, assign84290_body64_e128910_d_n5, assign84290_body64_e128910_d_n6, assign84290_body64_e128910_d_n7, assign84290_body64_e128910_d_n8, assign84290_body64_e128910_d_n9, assign84290_body64_e128910_d_n10, assign84290_body64_e128910_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1946 != 0.0)) && (locals.var_guard1956 == 0.0)) && (locals.var_guard1959 != 0.0)) {
        let assign84290_body64_e128905: f64 = (locals.var_fb * locals.var_fb);
        let assign84290_body64_e128907: f64 = (assign84290_body64_e128905 + locals.var_fs01);
        let assign84290_body64_e128908: f64 = (assign84290_body64_e128907).sqrt();
        (assign84290_body64_e128908, ((((locals.var_fb_dn0 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn0)) + locals.var_fs01_dn0) / (2.0 * assign84290_body64_e128908)), ((((locals.var_fb_dn2 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn2)) + locals.var_fs01_dn2) / (2.0 * assign84290_body64_e128908)), ((((locals.var_fb_dn4 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn4)) + locals.var_fs01_dn4) / (2.0 * assign84290_body64_e128908)), ((((locals.var_fb_dn5 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn5)) + locals.var_fs01_dn5) / (2.0 * assign84290_body64_e128908)), ((((locals.var_fb_dn6 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn6)) + locals.var_fs01_dn6) / (2.0 * assign84290_body64_e128908)), ((((locals.var_fb_dn7 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn7)) + locals.var_fs01_dn7) / (2.0 * assign84290_body64_e128908)), ((((locals.var_fb_dn8 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn8)) + locals.var_fs01_dn8) / (2.0 * assign84290_body64_e128908)), ((((locals.var_fb_dn9 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn9)) + locals.var_fs01_dn9) / (2.0 * assign84290_body64_e128908)), ((((locals.var_fb_dn10 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn10)) + locals.var_fs01_dn10) / (2.0 * assign84290_body64_e128908)), ((((locals.var_fb_dn13 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn13)) + locals.var_fs01_dn13) / (2.0 * assign84290_body64_e128908)),)
    } else {
        (locals.var_fs02, locals.var_fs02_dn0, locals.var_fs02_dn2, locals.var_fs02_dn4, locals.var_fs02_dn5, locals.var_fs02_dn6, locals.var_fs02_dn7, locals.var_fs02_dn8, locals.var_fs02_dn9, locals.var_fs02_dn10, locals.var_fs02_dn13,)
    }
};
            locals.var_fs02 = assign84290_body64_e128910;
            locals.var_fs02_dn0 = assign84290_body64_e128910_d_n0;
            locals.var_fs02_dn2 = assign84290_body64_e128910_d_n2;
            locals.var_fs02_dn4 = assign84290_body64_e128910_d_n4;
            locals.var_fs02_dn5 = assign84290_body64_e128910_d_n5;
            locals.var_fs02_dn6 = assign84290_body64_e128910_d_n6;
            locals.var_fs02_dn7 = assign84290_body64_e128910_d_n7;
            locals.var_fs02_dn8 = assign84290_body64_e128910_d_n8;
            locals.var_fs02_dn9 = assign84290_body64_e128910_d_n9;
            locals.var_fs02_dn10 = assign84290_body64_e128910_d_n10;
            locals.var_fs02_dn13 = assign84290_body64_e128910_d_n13;
            let (assign84290_body65_e128934, assign84290_body65_e128934_d_n0, assign84290_body65_e128934_d_n2, assign84290_body65_e128934_d_n4, assign84290_body65_e128934_d_n5, assign84290_body65_e128934_d_n6, assign84290_body65_e128934_d_n7, assign84290_body65_e128934_d_n8, assign84290_body65_e128934_d_n9, assign84290_body65_e128934_d_n10, assign84290_body65_e128934_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1946 != 0.0)) && (locals.var_guard1956 == 0.0)) && (locals.var_guard1959 != 0.0)) {
        let assign84290_body65_e128925: f64 = (2.0 * locals.var_fb_dpss);
        let assign84290_body65_e128927: f64 = (assign84290_body65_e128925 * locals.var_fb);
        let assign84290_body65_e128929: f64 = (assign84290_body65_e128927 + locals.var_fs01_dps0);
        let assign84290_body65_e128930: f64 = (0.5 * assign84290_body65_e128929);
        let assign84290_body65_e128932: f64 = (assign84290_body65_e128930 / locals.var_fs02);
        (assign84290_body65_e128932, ((((0.5 * ((((2.0 * locals.var_fb_dpss_dn0) * locals.var_fb) + (assign84290_body65_e128925 * locals.var_fb_dn0)) + locals.var_fs01_dps0_dn0)) * locals.var_fs02) - (assign84290_body65_e128930 * locals.var_fs02_dn0)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * ((((2.0 * locals.var_fb_dpss_dn2) * locals.var_fb) + (assign84290_body65_e128925 * locals.var_fb_dn2)) + locals.var_fs01_dps0_dn2)) * locals.var_fs02) - (assign84290_body65_e128930 * locals.var_fs02_dn2)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * ((((2.0 * locals.var_fb_dpss_dn4) * locals.var_fb) + (assign84290_body65_e128925 * locals.var_fb_dn4)) + locals.var_fs01_dps0_dn4)) * locals.var_fs02) - (assign84290_body65_e128930 * locals.var_fs02_dn4)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * ((((2.0 * locals.var_fb_dpss_dn5) * locals.var_fb) + (assign84290_body65_e128925 * locals.var_fb_dn5)) + locals.var_fs01_dps0_dn5)) * locals.var_fs02) - (assign84290_body65_e128930 * locals.var_fs02_dn5)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * ((((2.0 * locals.var_fb_dpss_dn6) * locals.var_fb) + (assign84290_body65_e128925 * locals.var_fb_dn6)) + locals.var_fs01_dps0_dn6)) * locals.var_fs02) - (assign84290_body65_e128930 * locals.var_fs02_dn6)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * ((((2.0 * locals.var_fb_dpss_dn7) * locals.var_fb) + (assign84290_body65_e128925 * locals.var_fb_dn7)) + locals.var_fs01_dps0_dn7)) * locals.var_fs02) - (assign84290_body65_e128930 * locals.var_fs02_dn7)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * ((((2.0 * locals.var_fb_dpss_dn8) * locals.var_fb) + (assign84290_body65_e128925 * locals.var_fb_dn8)) + locals.var_fs01_dps0_dn8)) * locals.var_fs02) - (assign84290_body65_e128930 * locals.var_fs02_dn8)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * ((((2.0 * locals.var_fb_dpss_dn9) * locals.var_fb) + (assign84290_body65_e128925 * locals.var_fb_dn9)) + locals.var_fs01_dps0_dn9)) * locals.var_fs02) - (assign84290_body65_e128930 * locals.var_fs02_dn9)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * ((((2.0 * locals.var_fb_dpss_dn10) * locals.var_fb) + (assign84290_body65_e128925 * locals.var_fb_dn10)) + locals.var_fs01_dps0_dn10)) * locals.var_fs02) - (assign84290_body65_e128930 * locals.var_fs02_dn10)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * ((((2.0 * locals.var_fb_dpss_dn13) * locals.var_fb) + (assign84290_body65_e128925 * locals.var_fb_dn13)) + locals.var_fs01_dps0_dn13)) * locals.var_fs02) - (assign84290_body65_e128930 * locals.var_fs02_dn13)) / (locals.var_fs02 * locals.var_fs02)),)
    } else {
        (locals.var_fs02_dps0, locals.var_fs02_dps0_dn0, locals.var_fs02_dps0_dn2, locals.var_fs02_dps0_dn4, locals.var_fs02_dps0_dn5, locals.var_fs02_dps0_dn6, locals.var_fs02_dps0_dn7, locals.var_fs02_dps0_dn8, locals.var_fs02_dps0_dn9, locals.var_fs02_dps0_dn10, locals.var_fs02_dps0_dn13,)
    }
};
            locals.var_fs02_dps0 = assign84290_body65_e128934;
            locals.var_fs02_dps0_dn0 = assign84290_body65_e128934_d_n0;
            locals.var_fs02_dps0_dn2 = assign84290_body65_e128934_d_n2;
            locals.var_fs02_dps0_dn4 = assign84290_body65_e128934_d_n4;
            locals.var_fs02_dps0_dn5 = assign84290_body65_e128934_d_n5;
            locals.var_fs02_dps0_dn6 = assign84290_body65_e128934_d_n6;
            locals.var_fs02_dps0_dn7 = assign84290_body65_e128934_d_n7;
            locals.var_fs02_dps0_dn8 = assign84290_body65_e128934_d_n8;
            locals.var_fs02_dps0_dn9 = assign84290_body65_e128934_d_n9;
            locals.var_fs02_dps0_dn10 = assign84290_body65_e128934_d_n10;
            locals.var_fs02_dps0_dn13 = assign84290_body65_e128934_d_n13;
            let (assign84290_body67_e128966, assign84290_body67_e128966_d_n0, assign84290_body67_e128966_d_n2, assign84290_body67_e128966_d_n4, assign84290_body67_e128966_d_n5, assign84290_body67_e128966_d_n6, assign84290_body67_e128966_d_n7, assign84290_body67_e128966_d_n8, assign84290_body67_e128966_d_n9, assign84290_body67_e128966_d_n10, assign84290_body67_e128966_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1946 != 0.0)) && (locals.var_guard1956 == 0.0)) && (locals.var_guard1959 == 0.0)) {
        (locals.var_fb, locals.var_fb_dn0, locals.var_fb_dn2, locals.var_fb_dn4, locals.var_fb_dn5, locals.var_fb_dn6, locals.var_fb_dn7, locals.var_fb_dn8, locals.var_fb_dn9, locals.var_fb_dn10, locals.var_fb_dn13,)
    } else {
        (locals.var_fs02, locals.var_fs02_dn0, locals.var_fs02_dn2, locals.var_fs02_dn4, locals.var_fs02_dn5, locals.var_fs02_dn6, locals.var_fs02_dn7, locals.var_fs02_dn8, locals.var_fs02_dn9, locals.var_fs02_dn10, locals.var_fs02_dn13,)
    }
};
            locals.var_fs02 = assign84290_body67_e128966;
            locals.var_fs02_dn0 = assign84290_body67_e128966_d_n0;
            locals.var_fs02_dn2 = assign84290_body67_e128966_d_n2;
            locals.var_fs02_dn4 = assign84290_body67_e128966_d_n4;
            locals.var_fs02_dn5 = assign84290_body67_e128966_d_n5;
            locals.var_fs02_dn6 = assign84290_body67_e128966_d_n6;
            locals.var_fs02_dn7 = assign84290_body67_e128966_d_n7;
            locals.var_fs02_dn8 = assign84290_body67_e128966_d_n8;
            locals.var_fs02_dn9 = assign84290_body67_e128966_d_n9;
            locals.var_fs02_dn10 = assign84290_body67_e128966_d_n10;
            locals.var_fs02_dn13 = assign84290_body67_e128966_d_n13;
            let (assign84290_body68_e128981, assign84290_body68_e128981_d_n0, assign84290_body68_e128981_d_n2, assign84290_body68_e128981_d_n4, assign84290_body68_e128981_d_n5, assign84290_body68_e128981_d_n6, assign84290_body68_e128981_d_n7, assign84290_body68_e128981_d_n8, assign84290_body68_e128981_d_n9, assign84290_body68_e128981_d_n10, assign84290_body68_e128981_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1946 != 0.0)) && (locals.var_guard1956 == 0.0)) && (locals.var_guard1959 == 0.0)) {
        (locals.var_fb_dpss, locals.var_fb_dpss_dn0, locals.var_fb_dpss_dn2, locals.var_fb_dpss_dn4, locals.var_fb_dpss_dn5, locals.var_fb_dpss_dn6, locals.var_fb_dpss_dn7, locals.var_fb_dpss_dn8, locals.var_fb_dpss_dn9, locals.var_fb_dpss_dn10, locals.var_fb_dpss_dn13,)
    } else {
        (locals.var_fs02_dps0, locals.var_fs02_dps0_dn0, locals.var_fs02_dps0_dn2, locals.var_fs02_dps0_dn4, locals.var_fs02_dps0_dn5, locals.var_fs02_dps0_dn6, locals.var_fs02_dps0_dn7, locals.var_fs02_dps0_dn8, locals.var_fs02_dps0_dn9, locals.var_fs02_dps0_dn10, locals.var_fs02_dps0_dn13,)
    }
};
            locals.var_fs02_dps0 = assign84290_body68_e128981;
            locals.var_fs02_dps0_dn0 = assign84290_body68_e128981_d_n0;
            locals.var_fs02_dps0_dn2 = assign84290_body68_e128981_d_n2;
            locals.var_fs02_dps0_dn4 = assign84290_body68_e128981_d_n4;
            locals.var_fs02_dps0_dn5 = assign84290_body68_e128981_d_n5;
            locals.var_fs02_dps0_dn6 = assign84290_body68_e128981_d_n6;
            locals.var_fs02_dps0_dn7 = assign84290_body68_e128981_d_n7;
            locals.var_fs02_dps0_dn8 = assign84290_body68_e128981_d_n8;
            locals.var_fs02_dps0_dn9 = assign84290_body68_e128981_d_n9;
            locals.var_fs02_dps0_dn10 = assign84290_body68_e128981_d_n10;
            locals.var_fs02_dps0_dn13 = assign84290_body68_e128981_d_n13;
            let (assign84290_body69_e128997, assign84290_body69_e128997_d_n0, assign84290_body69_e128997_d_n2, assign84290_body69_e128997_d_n4, assign84290_body69_e128997_d_n5, assign84290_body69_e128997_d_n6, assign84290_body69_e128997_d_n7, assign84290_body69_e128997_d_n8, assign84290_body69_e128997_d_n9, assign84290_body69_e128997_d_n10, assign84290_body69_e128997_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1946 != 0.0)) {
        let assign84290_body69_e128989: f64 = (-locals.var_vgpld);
        let assign84290_body69_e128991: f64 = (assign84290_body69_e128989 + locals.var_ps0ld);
        let assign84290_body69_e128994: f64 = (locals.var_fac1 * locals.var_fs02);
        let assign84290_body69_e128995: f64 = (assign84290_body69_e128991 + assign84290_body69_e128994);
        (assign84290_body69_e128995, (locals.var_ps0ld_dn0 + ((locals.var_fac1_dn0 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn0))), (((-locals.var_vgpld_dn2) + locals.var_ps0ld_dn2) + ((locals.var_fac1_dn2 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn2))), (locals.var_ps0ld_dn4 + ((locals.var_fac1_dn4 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn4))), (locals.var_ps0ld_dn5 + ((locals.var_fac1_dn5 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn5))), (((-locals.var_vgpld_dn6) + locals.var_ps0ld_dn6) + ((locals.var_fac1_dn6 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn6))), (((-locals.var_vgpld_dn7) + locals.var_ps0ld_dn7) + ((locals.var_fac1_dn7 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn7))), (((-locals.var_vgpld_dn8) + locals.var_ps0ld_dn8) + ((locals.var_fac1_dn8 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn8))), (locals.var_ps0ld_dn9 + ((locals.var_fac1_dn9 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn9))), (locals.var_ps0ld_dn10 + ((locals.var_fac1_dn10 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn10))), (locals.var_ps0ld_dn13 + ((locals.var_fac1_dn13 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn13))),)
    } else {
        (locals.var_fs0, locals.var_fs0_dn0, locals.var_fs0_dn2, locals.var_fs0_dn4, locals.var_fs0_dn5, locals.var_fs0_dn6, locals.var_fs0_dn7, locals.var_fs0_dn8, locals.var_fs0_dn9, locals.var_fs0_dn10, locals.var_fs0_dn13,)
    }
};
            locals.var_fs0 = assign84290_body69_e128997;
            locals.var_fs0_dn0 = assign84290_body69_e128997_d_n0;
            locals.var_fs0_dn2 = assign84290_body69_e128997_d_n2;
            locals.var_fs0_dn4 = assign84290_body69_e128997_d_n4;
            locals.var_fs0_dn5 = assign84290_body69_e128997_d_n5;
            locals.var_fs0_dn6 = assign84290_body69_e128997_d_n6;
            locals.var_fs0_dn7 = assign84290_body69_e128997_d_n7;
            locals.var_fs0_dn8 = assign84290_body69_e128997_d_n8;
            locals.var_fs0_dn9 = assign84290_body69_e128997_d_n9;
            locals.var_fs0_dn10 = assign84290_body69_e128997_d_n10;
            locals.var_fs0_dn13 = assign84290_body69_e128997_d_n13;
            let (assign84290_body70_e129010, assign84290_body70_e129010_d_n0, assign84290_body70_e129010_d_n2, assign84290_body70_e129010_d_n4, assign84290_body70_e129010_d_n5, assign84290_body70_e129010_d_n6, assign84290_body70_e129010_d_n7, assign84290_body70_e129010_d_n8, assign84290_body70_e129010_d_n9, assign84290_body70_e129010_d_n10, assign84290_body70_e129010_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1946 != 0.0)) {
        let assign84290_body70_e129007: f64 = (locals.var_fac1 * locals.var_fs02_dps0);
        let assign84290_body70_e129008: f64 = (1.0 + assign84290_body70_e129007);
        (assign84290_body70_e129008, ((locals.var_fac1_dn0 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn0)), ((locals.var_fac1_dn2 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn2)), ((locals.var_fac1_dn4 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn4)), ((locals.var_fac1_dn5 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn5)), ((locals.var_fac1_dn6 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn6)), ((locals.var_fac1_dn7 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn7)), ((locals.var_fac1_dn8 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn8)), ((locals.var_fac1_dn9 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn9)), ((locals.var_fac1_dn10 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn10)), ((locals.var_fac1_dn13 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn13)),)
    } else {
        (locals.var_fs0_dps0, locals.var_fs0_dps0_dn0, locals.var_fs0_dps0_dn2, locals.var_fs0_dps0_dn4, locals.var_fs0_dps0_dn5, locals.var_fs0_dps0_dn6, locals.var_fs0_dps0_dn7, locals.var_fs0_dps0_dn8, locals.var_fs0_dps0_dn9, locals.var_fs0_dps0_dn10, locals.var_fs0_dps0_dn13,)
    }
};
            locals.var_fs0_dps0 = assign84290_body70_e129010;
            locals.var_fs0_dps0_dn0 = assign84290_body70_e129010_d_n0;
            locals.var_fs0_dps0_dn2 = assign84290_body70_e129010_d_n2;
            locals.var_fs0_dps0_dn4 = assign84290_body70_e129010_d_n4;
            locals.var_fs0_dps0_dn5 = assign84290_body70_e129010_d_n5;
            locals.var_fs0_dps0_dn6 = assign84290_body70_e129010_d_n6;
            locals.var_fs0_dps0_dn7 = assign84290_body70_e129010_d_n7;
            locals.var_fs0_dps0_dn8 = assign84290_body70_e129010_d_n8;
            locals.var_fs0_dps0_dn9 = assign84290_body70_e129010_d_n9;
            locals.var_fs0_dps0_dn10 = assign84290_body70_e129010_d_n10;
            locals.var_fs0_dps0_dn13 = assign84290_body70_e129010_d_n13;
            let assign84290_body71_e129013: f64 = if locals.var_flg_conv == 1.0 { 1.0 } else { 0.0 };
            locals.var_guard1960 = assign84290_body71_e129013;
            let (assign84290_body72_e129026,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1946 != 0.0)) && (locals.var_guard1960 != 0.0)) {
        let assign84290_body72_e129024: f64 = (locals.var_lp_s0_max + 1.0);
        (assign84290_body72_e129024,)
    } else {
        (locals.var_lp_s0,)
    }
};
            locals.var_lp_s0 = assign84290_body72_e129026;
            let (assign84290_body73_e129041, assign84290_body73_e129041_d_n0, assign84290_body73_e129041_d_n2, assign84290_body73_e129041_d_n4, assign84290_body73_e129041_d_n5, assign84290_body73_e129041_d_n6, assign84290_body73_e129041_d_n7, assign84290_body73_e129041_d_n8, assign84290_body73_e129041_d_n9, assign84290_body73_e129041_d_n10, assign84290_body73_e129041_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1946 != 0.0)) && (locals.var_guard1960 == 0.0)) {
        let assign84290_body73_e129037: f64 = (-locals.var_fs0);
        let assign84290_body73_e129039: f64 = (assign84290_body73_e129037 / locals.var_fs0_dps0);
        (assign84290_body73_e129039, ((((-locals.var_fs0_dn0) * locals.var_fs0_dps0) - (assign84290_body73_e129037 * locals.var_fs0_dps0_dn0)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn2) * locals.var_fs0_dps0) - (assign84290_body73_e129037 * locals.var_fs0_dps0_dn2)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn4) * locals.var_fs0_dps0) - (assign84290_body73_e129037 * locals.var_fs0_dps0_dn4)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn5) * locals.var_fs0_dps0) - (assign84290_body73_e129037 * locals.var_fs0_dps0_dn5)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn6) * locals.var_fs0_dps0) - (assign84290_body73_e129037 * locals.var_fs0_dps0_dn6)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn7) * locals.var_fs0_dps0) - (assign84290_body73_e129037 * locals.var_fs0_dps0_dn7)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn8) * locals.var_fs0_dps0) - (assign84290_body73_e129037 * locals.var_fs0_dps0_dn8)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn9) * locals.var_fs0_dps0) - (assign84290_body73_e129037 * locals.var_fs0_dps0_dn9)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn10) * locals.var_fs0_dps0) - (assign84290_body73_e129037 * locals.var_fs0_dps0_dn10)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn13) * locals.var_fs0_dps0) - (assign84290_body73_e129037 * locals.var_fs0_dps0_dn13)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)),)
    } else {
        (locals.var_dps0, locals.var_dps0_dn0, locals.var_dps0_dn2, locals.var_dps0_dn4, locals.var_dps0_dn5, locals.var_dps0_dn6, locals.var_dps0_dn7, locals.var_dps0_dn8, locals.var_dps0_dn9, locals.var_dps0_dn10, locals.var_dps0_dn13,)
    }
};
            locals.var_dps0 = assign84290_body73_e129041;
            locals.var_dps0_dn0 = assign84290_body73_e129041_d_n0;
            locals.var_dps0_dn2 = assign84290_body73_e129041_d_n2;
            locals.var_dps0_dn4 = assign84290_body73_e129041_d_n4;
            locals.var_dps0_dn5 = assign84290_body73_e129041_d_n5;
            locals.var_dps0_dn6 = assign84290_body73_e129041_d_n6;
            locals.var_dps0_dn7 = assign84290_body73_e129041_d_n7;
            locals.var_dps0_dn8 = assign84290_body73_e129041_d_n8;
            locals.var_dps0_dn9 = assign84290_body73_e129041_d_n9;
            locals.var_dps0_dn10 = assign84290_body73_e129041_d_n10;
            locals.var_dps0_dn13 = assign84290_body73_e129041_d_n13;
            let (assign84290_body74_e129066, assign84290_body74_e129066_d_n0, assign84290_body74_e129066_d_n2, assign84290_body74_e129066_d_n4, assign84290_body74_e129066_d_n5, assign84290_body74_e129066_d_n6, assign84290_body74_e129066_d_n7, assign84290_body74_e129066_d_n8, assign84290_body74_e129066_d_n9, assign84290_body74_e129066_d_n10, assign84290_body74_e129066_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1946 != 0.0)) && (locals.var_guard1960 == 0.0)) {
        let assign84290_body74_e129053: f64 = (0.5 * 0.1);
        let assign84290_body74_e129057: f64 = (locals.var_ps0ld).abs();
        let (assign84290_body74_e129062, assign84290_body74_e129062_d_n0, assign84290_body74_e129062_d_n2, assign84290_body74_e129062_d_n4, assign84290_body74_e129062_d_n5, assign84290_body74_e129062_d_n6, assign84290_body74_e129062_d_n7, assign84290_body74_e129062_d_n8, assign84290_body74_e129062_d_n9, assign84290_body74_e129062_d_n10, assign84290_body74_e129062_d_n13,) = {
            if (1.0 >= assign84290_body74_e129057) {
                (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign84290_body74_e129061: f64 = (locals.var_ps0ld).abs();
                (assign84290_body74_e129061, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn0 } else { (-locals.var_ps0ld_dn0) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn2 } else { (-locals.var_ps0ld_dn2) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn4 } else { (-locals.var_ps0ld_dn4) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn5 } else { (-locals.var_ps0ld_dn5) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn6 } else { (-locals.var_ps0ld_dn6) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn7 } else { (-locals.var_ps0ld_dn7) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn8 } else { (-locals.var_ps0ld_dn8) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn9 } else { (-locals.var_ps0ld_dn9) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn10 } else { (-locals.var_ps0ld_dn10) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn13 } else { (-locals.var_ps0ld_dn13) },)
            }
        };
        let assign84290_body74_e129063: f64 = (1.0 + assign84290_body74_e129062);
        let assign84290_body74_e129064: f64 = (assign84290_body74_e129053 * assign84290_body74_e129063);
        (assign84290_body74_e129064, (assign84290_body74_e129053 * assign84290_body74_e129062_d_n0), (assign84290_body74_e129053 * assign84290_body74_e129062_d_n2), (assign84290_body74_e129053 * assign84290_body74_e129062_d_n4), (assign84290_body74_e129053 * assign84290_body74_e129062_d_n5), (assign84290_body74_e129053 * assign84290_body74_e129062_d_n6), (assign84290_body74_e129053 * assign84290_body74_e129062_d_n7), (assign84290_body74_e129053 * assign84290_body74_e129062_d_n8), (assign84290_body74_e129053 * assign84290_body74_e129062_d_n9), (assign84290_body74_e129053 * assign84290_body74_e129062_d_n10), (assign84290_body74_e129053 * assign84290_body74_e129062_d_n13),)
    } else {
        (locals.var_dplim, locals.var_dplim_dn0, locals.var_dplim_dn2, locals.var_dplim_dn4, locals.var_dplim_dn5, locals.var_dplim_dn6, locals.var_dplim_dn7, locals.var_dplim_dn8, locals.var_dplim_dn9, locals.var_dplim_dn10, locals.var_dplim_dn13,)
    }
};
            locals.var_dplim = assign84290_body74_e129066;
            locals.var_dplim_dn0 = assign84290_body74_e129066_d_n0;
            locals.var_dplim_dn2 = assign84290_body74_e129066_d_n2;
            locals.var_dplim_dn4 = assign84290_body74_e129066_d_n4;
            locals.var_dplim_dn5 = assign84290_body74_e129066_d_n5;
            locals.var_dplim_dn6 = assign84290_body74_e129066_d_n6;
            locals.var_dplim_dn7 = assign84290_body74_e129066_d_n7;
            locals.var_dplim_dn8 = assign84290_body74_e129066_d_n8;
            locals.var_dplim_dn9 = assign84290_body74_e129066_d_n9;
            locals.var_dplim_dn10 = assign84290_body74_e129066_d_n10;
            locals.var_dplim_dn13 = assign84290_body74_e129066_d_n13;
            let assign84290_body75_e129068: f64 = (locals.var_dps0).abs();
            let assign84290_body75_e129070: f64 = if assign84290_body75_e129068 > locals.var_dplim { 1.0 } else { 0.0 };
            locals.var_guard1961 = assign84290_body75_e129070;
            let (assign84290_body76_e129092, assign84290_body76_e129092_d_n0, assign84290_body76_e129092_d_n2, assign84290_body76_e129092_d_n4, assign84290_body76_e129092_d_n5, assign84290_body76_e129092_d_n6, assign84290_body76_e129092_d_n7, assign84290_body76_e129092_d_n8, assign84290_body76_e129092_d_n9, assign84290_body76_e129092_d_n10, assign84290_body76_e129092_d_n13,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1946 != 0.0)) && (locals.var_guard1960 == 0.0)) && (locals.var_guard1961 != 0.0)) {
        let (assign84290_body76_e129089,) = {
            if (locals.var_dps0 >= 0.0) {
                (1.0,)
            } else {
                let assign84290_body76_e129088: f64 = (-1.0);
                (assign84290_body76_e129088,)
            }
        };
        let assign84290_body76_e129090: f64 = (locals.var_dplim * assign84290_body76_e129089);
        (assign84290_body76_e129090, (locals.var_dplim_dn0 * assign84290_body76_e129089), (locals.var_dplim_dn2 * assign84290_body76_e129089), (locals.var_dplim_dn4 * assign84290_body76_e129089), (locals.var_dplim_dn5 * assign84290_body76_e129089), (locals.var_dplim_dn6 * assign84290_body76_e129089), (locals.var_dplim_dn7 * assign84290_body76_e129089), (locals.var_dplim_dn8 * assign84290_body76_e129089), (locals.var_dplim_dn9 * assign84290_body76_e129089), (locals.var_dplim_dn10 * assign84290_body76_e129089), (locals.var_dplim_dn13 * assign84290_body76_e129089),)
    } else {
        (locals.var_dps0, locals.var_dps0_dn0, locals.var_dps0_dn2, locals.var_dps0_dn4, locals.var_dps0_dn5, locals.var_dps0_dn6, locals.var_dps0_dn7, locals.var_dps0_dn8, locals.var_dps0_dn9, locals.var_dps0_dn10, locals.var_dps0_dn13,)
    }
};
            locals.var_dps0 = assign84290_body76_e129092;
            locals.var_dps0_dn0 = assign84290_body76_e129092_d_n0;
            locals.var_dps0_dn2 = assign84290_body76_e129092_d_n2;
            locals.var_dps0_dn4 = assign84290_body76_e129092_d_n4;
            locals.var_dps0_dn5 = assign84290_body76_e129092_d_n5;
            locals.var_dps0_dn6 = assign84290_body76_e129092_d_n6;
            locals.var_dps0_dn7 = assign84290_body76_e129092_d_n7;
            locals.var_dps0_dn8 = assign84290_body76_e129092_d_n8;
            locals.var_dps0_dn9 = assign84290_body76_e129092_d_n9;
            locals.var_dps0_dn10 = assign84290_body76_e129092_d_n10;
            locals.var_dps0_dn13 = assign84290_body76_e129092_d_n13;
            let (assign84290_body77_e129106, assign84290_body77_e129106_d_n0, assign84290_body77_e129106_d_n2, assign84290_body77_e129106_d_n4, assign84290_body77_e129106_d_n5, assign84290_body77_e129106_d_n6, assign84290_body77_e129106_d_n7, assign84290_body77_e129106_d_n8, assign84290_body77_e129106_d_n9, assign84290_body77_e129106_d_n10, assign84290_body77_e129106_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1946 != 0.0)) && (locals.var_guard1960 == 0.0)) {
        let assign84290_body77_e129104: f64 = (locals.var_ps0ld + locals.var_dps0);
        (assign84290_body77_e129104, (locals.var_ps0ld_dn0 + locals.var_dps0_dn0), (locals.var_ps0ld_dn2 + locals.var_dps0_dn2), (locals.var_ps0ld_dn4 + locals.var_dps0_dn4), (locals.var_ps0ld_dn5 + locals.var_dps0_dn5), (locals.var_ps0ld_dn6 + locals.var_dps0_dn6), (locals.var_ps0ld_dn7 + locals.var_dps0_dn7), (locals.var_ps0ld_dn8 + locals.var_dps0_dn8), (locals.var_ps0ld_dn9 + locals.var_dps0_dn9), (locals.var_ps0ld_dn10 + locals.var_dps0_dn10), (locals.var_ps0ld_dn13 + locals.var_dps0_dn13),)
    } else {
        (locals.var_ps0ld, locals.var_ps0ld_dn0, locals.var_ps0ld_dn2, locals.var_ps0ld_dn4, locals.var_ps0ld_dn5, locals.var_ps0ld_dn6, locals.var_ps0ld_dn7, locals.var_ps0ld_dn8, locals.var_ps0ld_dn9, locals.var_ps0ld_dn10, locals.var_ps0ld_dn13,)
    }
};
            locals.var_ps0ld = assign84290_body77_e129106;
            locals.var_ps0ld_dn0 = assign84290_body77_e129106_d_n0;
            locals.var_ps0ld_dn2 = assign84290_body77_e129106_d_n2;
            locals.var_ps0ld_dn4 = assign84290_body77_e129106_d_n4;
            locals.var_ps0ld_dn5 = assign84290_body77_e129106_d_n5;
            locals.var_ps0ld_dn6 = assign84290_body77_e129106_d_n6;
            locals.var_ps0ld_dn7 = assign84290_body77_e129106_d_n7;
            locals.var_ps0ld_dn8 = assign84290_body77_e129106_d_n8;
            locals.var_ps0ld_dn9 = assign84290_body77_e129106_d_n9;
            locals.var_ps0ld_dn10 = assign84290_body77_e129106_d_n10;
            locals.var_ps0ld_dn13 = assign84290_body77_e129106_d_n13;
            let assign84290_body78_e129108: f64 = (locals.var_dps0).abs();
            let assign84290_body78_e129112: f64 = (locals.var_fs0).abs();
            let assign84290_body78_e129115: f64 = if ((assign84290_body78_e129108 <= 1e-12) && (assign84290_body78_e129112 <= 1e-8)) { 1.0 } else { 0.0 };
            locals.var_guard1962 = assign84290_body78_e129115;
            let (assign84290_body79_e129129,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1946 != 0.0)) && (locals.var_guard1960 == 0.0)) && (locals.var_guard1962 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_flg_conv,)
    }
};
            locals.var_flg_conv = assign84290_body79_e129129;
            let (assign84290_body80_e129140,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1946 != 0.0)) {
        let assign84290_body80_e129138: f64 = (locals.var_lp_s0 + 1.0);
        (assign84290_body80_e129138,)
    } else {
        (locals.var_lp_s0,)
    }
};
            locals.var_lp_s0 = assign84290_body80_e129140;
        }

    }

    pub(super) fn stamp_transient_block_295(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign84310_e129154, assign84310_e129154_d_n0, assign84310_e129154_d_n2, assign84310_e129154_d_n4, assign84310_e129154_d_n5, assign84310_e129154_d_n6, assign84310_e129154_d_n7, assign84310_e129154_d_n8, assign84310_e129154_d_n9, assign84310_e129154_d_n10, assign84310_e129154_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1946 != 0.0)) {
        let assign84310_e129152: f64 = (locals.var_c_w_ld * locals.var_fb);
        (assign84310_e129152, ((locals.var_c_w_ld_dn0 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn0)), ((locals.var_c_w_ld_dn2 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn2)), ((locals.var_c_w_ld_dn4 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn4)), ((locals.var_c_w_ld_dn5 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn5)), ((locals.var_c_w_ld_dn6 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn6)), ((locals.var_c_w_ld_dn7 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn7)), ((locals.var_c_w_ld_dn8 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn8)), ((locals.var_c_w_ld_dn9 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn9)), ((locals.var_c_w_ld_dn10 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn10)), ((locals.var_c_w_ld_dn13 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn13)),)
    } else {
        (locals.var_wdld__blk1881, locals.var_wdld__blk1881_dn0, locals.var_wdld__blk1881_dn2, locals.var_wdld__blk1881_dn4, locals.var_wdld__blk1881_dn5, locals.var_wdld__blk1881_dn6, locals.var_wdld__blk1881_dn7, locals.var_wdld__blk1881_dn8, locals.var_wdld__blk1881_dn9, locals.var_wdld__blk1881_dn10, locals.var_wdld__blk1881_dn13,)
    }
};
        locals.var_wdld__blk1881 = assign84310_e129154;
        locals.var_wdld__blk1881_dn0 = assign84310_e129154_d_n0;
        locals.var_wdld__blk1881_dn2 = assign84310_e129154_d_n2;
        locals.var_wdld__blk1881_dn4 = assign84310_e129154_d_n4;
        locals.var_wdld__blk1881_dn5 = assign84310_e129154_d_n5;
        locals.var_wdld__blk1881_dn6 = assign84310_e129154_d_n6;
        locals.var_wdld__blk1881_dn7 = assign84310_e129154_d_n7;
        locals.var_wdld__blk1881_dn8 = assign84310_e129154_d_n8;
        locals.var_wdld__blk1881_dn9 = assign84310_e129154_d_n9;
        locals.var_wdld__blk1881_dn10 = assign84310_e129154_d_n10;
        locals.var_wdld__blk1881_dn13 = assign84310_e129154_d_n13;

        let (assign84320_e129165, assign84320_e129165_d_n0, assign84320_e129165_d_n2, assign84320_e129165_d_n4, assign84320_e129165_d_n5, assign84320_e129165_d_n6, assign84320_e129165_d_n7, assign84320_e129165_d_n8, assign84320_e129165_d_n9, assign84320_e129165_d_n10, assign84320_e129165_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1946 != 0.0)) {
        let assign84320_e129163: f64 = (locals.var_q_nsubld__blk1883 * locals.var_wdld__blk1881);
        (assign84320_e129163, (locals.var_q_nsubld__blk1883 * locals.var_wdld__blk1881_dn0), (locals.var_q_nsubld__blk1883 * locals.var_wdld__blk1881_dn2), (locals.var_q_nsubld__blk1883 * locals.var_wdld__blk1881_dn4), (locals.var_q_nsubld__blk1883 * locals.var_wdld__blk1881_dn5), (locals.var_q_nsubld__blk1883 * locals.var_wdld__blk1881_dn6), (locals.var_q_nsubld__blk1883 * locals.var_wdld__blk1881_dn7), (locals.var_q_nsubld__blk1883 * locals.var_wdld__blk1881_dn8), (locals.var_q_nsubld__blk1883 * locals.var_wdld__blk1881_dn9), (locals.var_q_nsubld__blk1883 * locals.var_wdld__blk1881_dn10), (locals.var_q_nsubld__blk1883 * locals.var_wdld__blk1881_dn13),)
    } else {
        (locals.var_q_dep_ld__blk1882, locals.var_q_dep_ld__blk1882_dn0, locals.var_q_dep_ld__blk1882_dn2, locals.var_q_dep_ld__blk1882_dn4, locals.var_q_dep_ld__blk1882_dn5, locals.var_q_dep_ld__blk1882_dn6, locals.var_q_dep_ld__blk1882_dn7, locals.var_q_dep_ld__blk1882_dn8, locals.var_q_dep_ld__blk1882_dn9, locals.var_q_dep_ld__blk1882_dn10, locals.var_q_dep_ld__blk1882_dn13,)
    }
};
        locals.var_q_dep_ld__blk1882 = assign84320_e129165;
        locals.var_q_dep_ld__blk1882_dn0 = assign84320_e129165_d_n0;
        locals.var_q_dep_ld__blk1882_dn2 = assign84320_e129165_d_n2;
        locals.var_q_dep_ld__blk1882_dn4 = assign84320_e129165_d_n4;
        locals.var_q_dep_ld__blk1882_dn5 = assign84320_e129165_d_n5;
        locals.var_q_dep_ld__blk1882_dn6 = assign84320_e129165_d_n6;
        locals.var_q_dep_ld__blk1882_dn7 = assign84320_e129165_d_n7;
        locals.var_q_dep_ld__blk1882_dn8 = assign84320_e129165_d_n8;
        locals.var_q_dep_ld__blk1882_dn9 = assign84320_e129165_d_n9;
        locals.var_q_dep_ld__blk1882_dn10 = assign84320_e129165_d_n10;
        locals.var_q_dep_ld__blk1882_dn13 = assign84320_e129165_d_n13;

        let (assign84330_e129180, assign84330_e129180_d_n0, assign84330_e129180_d_n2, assign84330_e129180_d_n4, assign84330_e129180_d_n5, assign84330_e129180_d_n6, assign84330_e129180_d_n7, assign84330_e129180_d_n8, assign84330_e129180_d_n9, assign84330_e129180_d_n10, assign84330_e129180_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1946 != 0.0)) {
        let assign84330_e129174: f64 = (locals.var_q_dep_ld__blk1882 / locals.var_cnst0over_func);
        let assign84330_e129177: f64 = (10.0 * 2.220446049250313e-16);
        let assign84330_e129178: f64 = (assign84330_e129174 + assign84330_e129177);
        (assign84330_e129178, (((locals.var_q_dep_ld__blk1882_dn0 * locals.var_cnst0over_func) - (locals.var_q_dep_ld__blk1882 * locals.var_cnst0over_func_dn0)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)), (((locals.var_q_dep_ld__blk1882_dn2 * locals.var_cnst0over_func) - (locals.var_q_dep_ld__blk1882 * locals.var_cnst0over_func_dn2)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)), (((locals.var_q_dep_ld__blk1882_dn4 * locals.var_cnst0over_func) - (locals.var_q_dep_ld__blk1882 * locals.var_cnst0over_func_dn4)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)), (((locals.var_q_dep_ld__blk1882_dn5 * locals.var_cnst0over_func) - (locals.var_q_dep_ld__blk1882 * locals.var_cnst0over_func_dn5)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)), (((locals.var_q_dep_ld__blk1882_dn6 * locals.var_cnst0over_func) - (locals.var_q_dep_ld__blk1882 * locals.var_cnst0over_func_dn6)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)), (((locals.var_q_dep_ld__blk1882_dn7 * locals.var_cnst0over_func) - (locals.var_q_dep_ld__blk1882 * locals.var_cnst0over_func_dn7)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)), (((locals.var_q_dep_ld__blk1882_dn8 * locals.var_cnst0over_func) - (locals.var_q_dep_ld__blk1882 * locals.var_cnst0over_func_dn8)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)), (((locals.var_q_dep_ld__blk1882_dn9 * locals.var_cnst0over_func) - (locals.var_q_dep_ld__blk1882 * locals.var_cnst0over_func_dn9)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)), (((locals.var_q_dep_ld__blk1882_dn10 * locals.var_cnst0over_func) - (locals.var_q_dep_ld__blk1882 * locals.var_cnst0over_func_dn10)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)), (((locals.var_q_dep_ld__blk1882_dn13 * locals.var_cnst0over_func) - (locals.var_q_dep_ld__blk1882 * locals.var_cnst0over_func_dn13)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)),)
    } else {
        (locals.var_xi0p12, locals.var_xi0p12_dn0, locals.var_xi0p12_dn2, locals.var_xi0p12_dn4, locals.var_xi0p12_dn5, locals.var_xi0p12_dn6, locals.var_xi0p12_dn7, locals.var_xi0p12_dn8, locals.var_xi0p12_dn9, locals.var_xi0p12_dn10, locals.var_xi0p12_dn13,)
    }
};
        locals.var_xi0p12 = assign84330_e129180;
        locals.var_xi0p12_dn0 = assign84330_e129180_d_n0;
        locals.var_xi0p12_dn2 = assign84330_e129180_d_n2;
        locals.var_xi0p12_dn4 = assign84330_e129180_d_n4;
        locals.var_xi0p12_dn5 = assign84330_e129180_d_n5;
        locals.var_xi0p12_dn6 = assign84330_e129180_d_n6;
        locals.var_xi0p12_dn7 = assign84330_e129180_d_n7;
        locals.var_xi0p12_dn8 = assign84330_e129180_d_n8;
        locals.var_xi0p12_dn9 = assign84330_e129180_d_n9;
        locals.var_xi0p12_dn10 = assign84330_e129180_d_n10;
        locals.var_xi0p12_dn13 = assign84330_e129180_d_n13;

        let (assign84340_e129191, assign84340_e129191_d_n0, assign84340_e129191_d_n2, assign84340_e129191_d_n4, assign84340_e129191_d_n5, assign84340_e129191_d_n6, assign84340_e129191_d_n7, assign84340_e129191_d_n8, assign84340_e129191_d_n9, assign84340_e129191_d_n10, assign84340_e129191_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1946 != 0.0)) {
        let assign84340_e129189: f64 = (locals.var_cnst0over_func * locals.var_xi0p12);
        (assign84340_e129189, ((locals.var_cnst0over_func_dn0 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn0)), ((locals.var_cnst0over_func_dn2 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn2)), ((locals.var_cnst0over_func_dn4 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn4)), ((locals.var_cnst0over_func_dn5 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn5)), ((locals.var_cnst0over_func_dn6 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn6)), ((locals.var_cnst0over_func_dn7 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn7)), ((locals.var_cnst0over_func_dn8 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn8)), ((locals.var_cnst0over_func_dn9 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn9)), ((locals.var_cnst0over_func_dn10 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn10)), ((locals.var_cnst0over_func_dn13 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn13)),)
    } else {
        (locals.var_qbuld, locals.var_qbuld_dn0, locals.var_qbuld_dn2, locals.var_qbuld_dn4, locals.var_qbuld_dn5, locals.var_qbuld_dn6, locals.var_qbuld_dn7, locals.var_qbuld_dn8, locals.var_qbuld_dn9, locals.var_qbuld_dn10, locals.var_qbuld_dn13,)
    }
};
        locals.var_qbuld = assign84340_e129191;
        locals.var_qbuld_dn0 = assign84340_e129191_d_n0;
        locals.var_qbuld_dn2 = assign84340_e129191_d_n2;
        locals.var_qbuld_dn4 = assign84340_e129191_d_n4;
        locals.var_qbuld_dn5 = assign84340_e129191_d_n5;
        locals.var_qbuld_dn6 = assign84340_e129191_d_n6;
        locals.var_qbuld_dn7 = assign84340_e129191_d_n7;
        locals.var_qbuld_dn8 = assign84340_e129191_d_n8;
        locals.var_qbuld_dn9 = assign84340_e129191_d_n9;
        locals.var_qbuld_dn10 = assign84340_e129191_d_n10;
        locals.var_qbuld_dn13 = assign84340_e129191_d_n13;

        let (assign84350_e129204, assign84350_e129204_d_n0, assign84350_e129204_d_n2, assign84350_e129204_d_n4, assign84350_e129204_d_n5, assign84350_e129204_d_n6, assign84350_e129204_d_n7, assign84350_e129204_d_n8, assign84350_e129204_d_n9, assign84350_e129204_d_n10, assign84350_e129204_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1946 != 0.0)) {
        let assign84350_e129201: f64 = (locals.var_fs02 + locals.var_xi0p12);
        let assign84350_e129202: f64 = (1.0 / assign84350_e129201);
        (assign84350_e129202, (-((locals.var_fs02_dn0 + locals.var_xi0p12_dn0) / (assign84350_e129201 * assign84350_e129201))), (-((locals.var_fs02_dn2 + locals.var_xi0p12_dn2) / (assign84350_e129201 * assign84350_e129201))), (-((locals.var_fs02_dn4 + locals.var_xi0p12_dn4) / (assign84350_e129201 * assign84350_e129201))), (-((locals.var_fs02_dn5 + locals.var_xi0p12_dn5) / (assign84350_e129201 * assign84350_e129201))), (-((locals.var_fs02_dn6 + locals.var_xi0p12_dn6) / (assign84350_e129201 * assign84350_e129201))), (-((locals.var_fs02_dn7 + locals.var_xi0p12_dn7) / (assign84350_e129201 * assign84350_e129201))), (-((locals.var_fs02_dn8 + locals.var_xi0p12_dn8) / (assign84350_e129201 * assign84350_e129201))), (-((locals.var_fs02_dn9 + locals.var_xi0p12_dn9) / (assign84350_e129201 * assign84350_e129201))), (-((locals.var_fs02_dn10 + locals.var_xi0p12_dn10) / (assign84350_e129201 * assign84350_e129201))), (-((locals.var_fs02_dn13 + locals.var_xi0p12_dn13) / (assign84350_e129201 * assign84350_e129201))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign84350_e129204;
        locals.var_t1_dn0 = assign84350_e129204_d_n0;
        locals.var_t1_dn2 = assign84350_e129204_d_n2;
        locals.var_t1_dn4 = assign84350_e129204_d_n4;
        locals.var_t1_dn5 = assign84350_e129204_d_n5;
        locals.var_t1_dn6 = assign84350_e129204_d_n6;
        locals.var_t1_dn7 = assign84350_e129204_d_n7;
        locals.var_t1_dn8 = assign84350_e129204_d_n8;
        locals.var_t1_dn9 = assign84350_e129204_d_n9;
        locals.var_t1_dn10 = assign84350_e129204_d_n10;
        locals.var_t1_dn13 = assign84350_e129204_d_n13;

        let (assign84360_e129217, assign84360_e129217_d_n0, assign84360_e129217_d_n2, assign84360_e129217_d_n4, assign84360_e129217_d_n5, assign84360_e129217_d_n6, assign84360_e129217_d_n7, assign84360_e129217_d_n8, assign84360_e129217_d_n9, assign84360_e129217_d_n10, assign84360_e129217_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1946 != 0.0)) {
        let assign84360_e129213: f64 = (locals.var_cnst0over_func * locals.var_fs01);
        let assign84360_e129215: f64 = (assign84360_e129213 * locals.var_t1);
        (assign84360_e129215, ((((locals.var_cnst0over_func_dn0 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn0)) * locals.var_t1) + (assign84360_e129213 * locals.var_t1_dn0)), ((((locals.var_cnst0over_func_dn2 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn2)) * locals.var_t1) + (assign84360_e129213 * locals.var_t1_dn2)), ((((locals.var_cnst0over_func_dn4 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn4)) * locals.var_t1) + (assign84360_e129213 * locals.var_t1_dn4)), ((((locals.var_cnst0over_func_dn5 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn5)) * locals.var_t1) + (assign84360_e129213 * locals.var_t1_dn5)), ((((locals.var_cnst0over_func_dn6 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn6)) * locals.var_t1) + (assign84360_e129213 * locals.var_t1_dn6)), ((((locals.var_cnst0over_func_dn7 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn7)) * locals.var_t1) + (assign84360_e129213 * locals.var_t1_dn7)), ((((locals.var_cnst0over_func_dn8 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn8)) * locals.var_t1) + (assign84360_e129213 * locals.var_t1_dn8)), ((((locals.var_cnst0over_func_dn9 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn9)) * locals.var_t1) + (assign84360_e129213 * locals.var_t1_dn9)), ((((locals.var_cnst0over_func_dn10 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn10)) * locals.var_t1) + (assign84360_e129213 * locals.var_t1_dn10)), ((((locals.var_cnst0over_func_dn13 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn13)) * locals.var_t1) + (assign84360_e129213 * locals.var_t1_dn13)),)
    } else {
        (locals.var_qiuld, locals.var_qiuld_dn0, locals.var_qiuld_dn2, locals.var_qiuld_dn4, locals.var_qiuld_dn5, locals.var_qiuld_dn6, locals.var_qiuld_dn7, locals.var_qiuld_dn8, locals.var_qiuld_dn9, locals.var_qiuld_dn10, locals.var_qiuld_dn13,)
    }
};
        locals.var_qiuld = assign84360_e129217;
        locals.var_qiuld_dn0 = assign84360_e129217_d_n0;
        locals.var_qiuld_dn2 = assign84360_e129217_d_n2;
        locals.var_qiuld_dn4 = assign84360_e129217_d_n4;
        locals.var_qiuld_dn5 = assign84360_e129217_d_n5;
        locals.var_qiuld_dn6 = assign84360_e129217_d_n6;
        locals.var_qiuld_dn7 = assign84360_e129217_d_n7;
        locals.var_qiuld_dn8 = assign84360_e129217_d_n8;
        locals.var_qiuld_dn9 = assign84360_e129217_d_n9;
        locals.var_qiuld_dn10 = assign84360_e129217_d_n10;
        locals.var_qiuld_dn13 = assign84360_e129217_d_n13;

        let (assign84370_e129228, assign84370_e129228_d_n0, assign84370_e129228_d_n2, assign84370_e129228_d_n4, assign84370_e129228_d_n5, assign84370_e129228_d_n6, assign84370_e129228_d_n7, assign84370_e129228_d_n8, assign84370_e129228_d_n9, assign84370_e129228_d_n10, assign84370_e129228_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1907 == 0.0)) && (locals.var_guard1946 != 0.0)) {
        let assign84370_e129226: f64 = (locals.var_qbuld + locals.var_qiuld);
        (assign84370_e129226, (locals.var_qbuld_dn0 + locals.var_qiuld_dn0), (locals.var_qbuld_dn2 + locals.var_qiuld_dn2), (locals.var_qbuld_dn4 + locals.var_qiuld_dn4), (locals.var_qbuld_dn5 + locals.var_qiuld_dn5), (locals.var_qbuld_dn6 + locals.var_qiuld_dn6), (locals.var_qbuld_dn7 + locals.var_qiuld_dn7), (locals.var_qbuld_dn8 + locals.var_qiuld_dn8), (locals.var_qbuld_dn9 + locals.var_qiuld_dn9), (locals.var_qbuld_dn10 + locals.var_qiuld_dn10), (locals.var_qbuld_dn13 + locals.var_qiuld_dn13),)
    } else {
        (locals.var_qsuld, locals.var_qsuld_dn0, locals.var_qsuld_dn2, locals.var_qsuld_dn4, locals.var_qsuld_dn5, locals.var_qsuld_dn6, locals.var_qsuld_dn7, locals.var_qsuld_dn8, locals.var_qsuld_dn9, locals.var_qsuld_dn10, locals.var_qsuld_dn13,)
    }
};
        locals.var_qsuld = assign84370_e129228;
        locals.var_qsuld_dn0 = assign84370_e129228_d_n0;
        locals.var_qsuld_dn2 = assign84370_e129228_d_n2;
        locals.var_qsuld_dn4 = assign84370_e129228_d_n4;
        locals.var_qsuld_dn5 = assign84370_e129228_d_n5;
        locals.var_qsuld_dn6 = assign84370_e129228_d_n6;
        locals.var_qsuld_dn7 = assign84370_e129228_d_n7;
        locals.var_qsuld_dn8 = assign84370_e129228_d_n8;
        locals.var_qsuld_dn9 = assign84370_e129228_d_n9;
        locals.var_qsuld_dn10 = assign84370_e129228_d_n10;
        locals.var_qsuld_dn13 = assign84370_e129228_d_n13;

        let assign84380_e129231: f64 = if p.p33 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard1964 = assign84380_e129231;

        let (assign84390_e129241, assign84390_e129241_d_n0, assign84390_e129241_d_n2, assign84390_e129241_d_n4, assign84390_e129241_d_n5, assign84390_e129241_d_n6, assign84390_e129241_d_n7, assign84390_e129241_d_n8, assign84390_e129241_d_n9, assign84390_e129241_d_n10, assign84390_e129241_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1964 != 0.0)) {
        let assign84390_e129237: f64 = (-locals.var_vxbgmtcl);
        let assign84390_e129238: f64 = (locals.var_beta * assign84390_e129237);
        let assign84390_e129239: f64 = (assign84390_e129238).exp();
        (assign84390_e129239, (assign84390_e129239 * ((locals.var_beta_dn0 * assign84390_e129237) + (locals.var_beta * (-locals.var_vxbgmtcl_dn0)))), (assign84390_e129239 * ((locals.var_beta_dn2 * assign84390_e129237) + (locals.var_beta * (-locals.var_vxbgmtcl_dn2)))), (assign84390_e129239 * ((locals.var_beta_dn4 * assign84390_e129237) + (locals.var_beta * (-locals.var_vxbgmtcl_dn4)))), (assign84390_e129239 * ((locals.var_beta_dn5 * assign84390_e129237) + (locals.var_beta * (-locals.var_vxbgmtcl_dn5)))), (assign84390_e129239 * ((locals.var_beta_dn6 * assign84390_e129237) + (locals.var_beta * (-locals.var_vxbgmtcl_dn6)))), (assign84390_e129239 * ((locals.var_beta_dn7 * assign84390_e129237) + (locals.var_beta * (-locals.var_vxbgmtcl_dn7)))), (assign84390_e129239 * ((locals.var_beta_dn8 * assign84390_e129237) + (locals.var_beta * (-locals.var_vxbgmtcl_dn8)))), (assign84390_e129239 * ((locals.var_beta_dn9 * assign84390_e129237) + (locals.var_beta * (-locals.var_vxbgmtcl_dn9)))), (assign84390_e129239 * ((locals.var_beta_dn10 * assign84390_e129237) + (locals.var_beta * (-locals.var_vxbgmtcl_dn10)))), (assign84390_e129239 * ((locals.var_beta_dn13 * assign84390_e129237) + (locals.var_beta * (-locals.var_vxbgmtcl_dn13)))),)
    } else {
        (locals.var_exp_bvbs, locals.var_exp_bvbs_dn0, locals.var_exp_bvbs_dn2, locals.var_exp_bvbs_dn4, locals.var_exp_bvbs_dn5, locals.var_exp_bvbs_dn6, locals.var_exp_bvbs_dn7, locals.var_exp_bvbs_dn8, locals.var_exp_bvbs_dn9, locals.var_exp_bvbs_dn10, locals.var_exp_bvbs_dn13,)
    }
};
        locals.var_exp_bvbs = assign84390_e129241;
        locals.var_exp_bvbs_dn0 = assign84390_e129241_d_n0;
        locals.var_exp_bvbs_dn2 = assign84390_e129241_d_n2;
        locals.var_exp_bvbs_dn4 = assign84390_e129241_d_n4;
        locals.var_exp_bvbs_dn5 = assign84390_e129241_d_n5;
        locals.var_exp_bvbs_dn6 = assign84390_e129241_d_n6;
        locals.var_exp_bvbs_dn7 = assign84390_e129241_d_n7;
        locals.var_exp_bvbs_dn8 = assign84390_e129241_d_n8;
        locals.var_exp_bvbs_dn9 = assign84390_e129241_d_n9;
        locals.var_exp_bvbs_dn10 = assign84390_e129241_d_n10;
        locals.var_exp_bvbs_dn13 = assign84390_e129241_d_n13;

        let (assign84400_e129249, assign84400_e129249_d_n0, assign84400_e129249_d_n2, assign84400_e129249_d_n4, assign84400_e129249_d_n5, assign84400_e129249_d_n6, assign84400_e129249_d_n7, assign84400_e129249_d_n8, assign84400_e129249_d_n9, assign84400_e129249_d_n10, assign84400_e129249_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1964 != 0.0)) {
        let assign84400_e129247: f64 = (locals.var_nin / locals.var_nover_func);
        (assign84400_e129247, (locals.var_nin_dn0 / locals.var_nover_func), (locals.var_nin_dn2 / locals.var_nover_func), (locals.var_nin_dn4 / locals.var_nover_func), (locals.var_nin_dn5 / locals.var_nover_func), (locals.var_nin_dn6 / locals.var_nover_func), (locals.var_nin_dn7 / locals.var_nover_func), (locals.var_nin_dn8 / locals.var_nover_func), (locals.var_nin_dn9 / locals.var_nover_func), (locals.var_nin_dn10 / locals.var_nover_func), (locals.var_nin_dn13 / locals.var_nover_func),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign84400_e129249;
        locals.var_t0_dn0 = assign84400_e129249_d_n0;
        locals.var_t0_dn2 = assign84400_e129249_d_n2;
        locals.var_t0_dn4 = assign84400_e129249_d_n4;
        locals.var_t0_dn5 = assign84400_e129249_d_n5;
        locals.var_t0_dn6 = assign84400_e129249_d_n6;
        locals.var_t0_dn7 = assign84400_e129249_d_n7;
        locals.var_t0_dn8 = assign84400_e129249_d_n8;
        locals.var_t0_dn9 = assign84400_e129249_d_n9;
        locals.var_t0_dn10 = assign84400_e129249_d_n10;
        locals.var_t0_dn13 = assign84400_e129249_d_n13;

        let (assign84410_e129257, assign84410_e129257_d_n0, assign84410_e129257_d_n2, assign84410_e129257_d_n4, assign84410_e129257_d_n5, assign84410_e129257_d_n6, assign84410_e129257_d_n7, assign84410_e129257_d_n8, assign84410_e129257_d_n9, assign84410_e129257_d_n10, assign84410_e129257_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1964 != 0.0)) {
        let assign84410_e129255: f64 = (locals.var_t0 * locals.var_t0);
        (assign84410_e129255, ((locals.var_t0_dn0 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn0)), ((locals.var_t0_dn2 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn2)), ((locals.var_t0_dn4 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn4)), ((locals.var_t0_dn5 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn5)), ((locals.var_t0_dn6 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn6)), ((locals.var_t0_dn7 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn7)), ((locals.var_t0_dn8 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn8)), ((locals.var_t0_dn9 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn9)), ((locals.var_t0_dn10 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn10)), ((locals.var_t0_dn13 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn13)),)
    } else {
        (locals.var_cnst1over, locals.var_cnst1over_dn0, locals.var_cnst1over_dn2, locals.var_cnst1over_dn4, locals.var_cnst1over_dn5, locals.var_cnst1over_dn6, locals.var_cnst1over_dn7, locals.var_cnst1over_dn8, locals.var_cnst1over_dn9, locals.var_cnst1over_dn10, locals.var_cnst1over_dn13,)
    }
};
        locals.var_cnst1over = assign84410_e129257;
        locals.var_cnst1over_dn0 = assign84410_e129257_d_n0;
        locals.var_cnst1over_dn2 = assign84410_e129257_d_n2;
        locals.var_cnst1over_dn4 = assign84410_e129257_d_n4;
        locals.var_cnst1over_dn5 = assign84410_e129257_d_n5;
        locals.var_cnst1over_dn6 = assign84410_e129257_d_n6;
        locals.var_cnst1over_dn7 = assign84410_e129257_d_n7;
        locals.var_cnst1over_dn8 = assign84410_e129257_d_n8;
        locals.var_cnst1over_dn9 = assign84410_e129257_d_n9;
        locals.var_cnst1over_dn10 = assign84410_e129257_d_n10;
        locals.var_cnst1over_dn13 = assign84410_e129257_d_n13;

        let (assign84420_e129265, assign84420_e129265_d_n0, assign84420_e129265_d_n2, assign84420_e129265_d_n4, assign84420_e129265_d_n5, assign84420_e129265_d_n6, assign84420_e129265_d_n7, assign84420_e129265_d_n8, assign84420_e129265_d_n9, assign84420_e129265_d_n10, assign84420_e129265_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1964 != 0.0)) {
        let assign84420_e129263: f64 = (locals.var_cnst1over * locals.var_exp_bvbs);
        (assign84420_e129263, ((locals.var_cnst1over_dn0 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn0)), ((locals.var_cnst1over_dn2 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn2)), ((locals.var_cnst1over_dn4 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn4)), ((locals.var_cnst1over_dn5 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn5)), ((locals.var_cnst1over_dn6 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn6)), ((locals.var_cnst1over_dn7 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn7)), ((locals.var_cnst1over_dn8 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn8)), ((locals.var_cnst1over_dn9 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn9)), ((locals.var_cnst1over_dn10 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn10)), ((locals.var_cnst1over_dn13 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn13)),)
    } else {
        (locals.var_cfs1, locals.var_cfs1_dn0, locals.var_cfs1_dn2, locals.var_cfs1_dn4, locals.var_cfs1_dn5, locals.var_cfs1_dn6, locals.var_cfs1_dn7, locals.var_cfs1_dn8, locals.var_cfs1_dn9, locals.var_cfs1_dn10, locals.var_cfs1_dn13,)
    }
};
        locals.var_cfs1 = assign84420_e129265;
        locals.var_cfs1_dn0 = assign84420_e129265_d_n0;
        locals.var_cfs1_dn2 = assign84420_e129265_d_n2;
        locals.var_cfs1_dn4 = assign84420_e129265_d_n4;
        locals.var_cfs1_dn5 = assign84420_e129265_d_n5;
        locals.var_cfs1_dn6 = assign84420_e129265_d_n6;
        locals.var_cfs1_dn7 = assign84420_e129265_d_n7;
        locals.var_cfs1_dn8 = assign84420_e129265_d_n8;
        locals.var_cfs1_dn9 = assign84420_e129265_d_n9;
        locals.var_cfs1_dn10 = assign84420_e129265_d_n10;
        locals.var_cfs1_dn13 = assign84420_e129265_d_n13;

        let (assign84430_e129271, assign84430_e129271_d_n0, assign84430_e129271_d_n2, assign84430_e129271_d_n4, assign84430_e129271_d_n5, assign84430_e129271_d_n6, assign84430_e129271_d_n7, assign84430_e129271_d_n8, assign84430_e129271_d_n9, assign84430_e129271_d_n10, assign84430_e129271_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1964 != 0.0)) {
        (locals.var_ps0ld_ini__blk1890, locals.var_ps0ld_ini__blk1890_dn0, locals.var_ps0ld_ini__blk1890_dn2, locals.var_ps0ld_ini__blk1890_dn4, locals.var_ps0ld_ini__blk1890_dn5, locals.var_ps0ld_ini__blk1890_dn6, locals.var_ps0ld_ini__blk1890_dn7, locals.var_ps0ld_ini__blk1890_dn8, locals.var_ps0ld_ini__blk1890_dn9, locals.var_ps0ld_ini__blk1890_dn10, locals.var_ps0ld_ini__blk1890_dn13,)
    } else {
        (locals.var_ps0ld, locals.var_ps0ld_dn0, locals.var_ps0ld_dn2, locals.var_ps0ld_dn4, locals.var_ps0ld_dn5, locals.var_ps0ld_dn6, locals.var_ps0ld_dn7, locals.var_ps0ld_dn8, locals.var_ps0ld_dn9, locals.var_ps0ld_dn10, locals.var_ps0ld_dn13,)
    }
};
        locals.var_ps0ld = assign84430_e129271;
        locals.var_ps0ld_dn0 = assign84430_e129271_d_n0;
        locals.var_ps0ld_dn2 = assign84430_e129271_d_n2;
        locals.var_ps0ld_dn4 = assign84430_e129271_d_n4;
        locals.var_ps0ld_dn5 = assign84430_e129271_d_n5;
        locals.var_ps0ld_dn6 = assign84430_e129271_d_n6;
        locals.var_ps0ld_dn7 = assign84430_e129271_d_n7;
        locals.var_ps0ld_dn8 = assign84430_e129271_d_n8;
        locals.var_ps0ld_dn9 = assign84430_e129271_d_n9;
        locals.var_ps0ld_dn10 = assign84430_e129271_d_n10;
        locals.var_ps0ld_dn13 = assign84430_e129271_d_n13;

        let (assign84440_e129277,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1964 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_flg_conv,)
    }
};
        locals.var_flg_conv = assign84440_e129277;

        let (assign84450_e129290, assign84450_e129290_d_n0, assign84450_e129290_d_n2, assign84450_e129290_d_n4, assign84450_e129290_d_n5, assign84450_e129290_d_n6, assign84450_e129290_d_n7, assign84450_e129290_d_n8, assign84450_e129290_d_n9, assign84450_e129290_d_n10, assign84450_e129290_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1964 != 0.0)) {
        let assign84450_e129284: f64 = (1.034943e-10 / locals.var_q_nsubld__blk1883);
        let assign84450_e129286: f64 = (assign84450_e129284 * locals.var_beta_inv);
        let assign84450_e129287: f64 = (2.0 * assign84450_e129286);
        let assign84450_e129288: f64 = (assign84450_e129287).sqrt();
        (assign84450_e129288, ((2.0 * (assign84450_e129284 * locals.var_beta_inv_dn0)) / (2.0 * assign84450_e129288)), ((2.0 * (assign84450_e129284 * locals.var_beta_inv_dn2)) / (2.0 * assign84450_e129288)), ((2.0 * (assign84450_e129284 * locals.var_beta_inv_dn4)) / (2.0 * assign84450_e129288)), ((2.0 * (assign84450_e129284 * locals.var_beta_inv_dn5)) / (2.0 * assign84450_e129288)), ((2.0 * (assign84450_e129284 * locals.var_beta_inv_dn6)) / (2.0 * assign84450_e129288)), ((2.0 * (assign84450_e129284 * locals.var_beta_inv_dn7)) / (2.0 * assign84450_e129288)), ((2.0 * (assign84450_e129284 * locals.var_beta_inv_dn8)) / (2.0 * assign84450_e129288)), ((2.0 * (assign84450_e129284 * locals.var_beta_inv_dn9)) / (2.0 * assign84450_e129288)), ((2.0 * (assign84450_e129284 * locals.var_beta_inv_dn10)) / (2.0 * assign84450_e129288)), ((2.0 * (assign84450_e129284 * locals.var_beta_inv_dn13)) / (2.0 * assign84450_e129288)),)
    } else {
        (locals.var_c_w_ld, locals.var_c_w_ld_dn0, locals.var_c_w_ld_dn2, locals.var_c_w_ld_dn4, locals.var_c_w_ld_dn5, locals.var_c_w_ld_dn6, locals.var_c_w_ld_dn7, locals.var_c_w_ld_dn8, locals.var_c_w_ld_dn9, locals.var_c_w_ld_dn10, locals.var_c_w_ld_dn13,)
    }
};
        locals.var_c_w_ld = assign84450_e129290;
        locals.var_c_w_ld_dn0 = assign84450_e129290_d_n0;
        locals.var_c_w_ld_dn2 = assign84450_e129290_d_n2;
        locals.var_c_w_ld_dn4 = assign84450_e129290_d_n4;
        locals.var_c_w_ld_dn5 = assign84450_e129290_d_n5;
        locals.var_c_w_ld_dn6 = assign84450_e129290_d_n6;
        locals.var_c_w_ld_dn7 = assign84450_e129290_d_n7;
        locals.var_c_w_ld_dn8 = assign84450_e129290_d_n8;
        locals.var_c_w_ld_dn9 = assign84450_e129290_d_n9;
        locals.var_c_w_ld_dn10 = assign84450_e129290_d_n10;
        locals.var_c_w_ld_dn13 = assign84450_e129290_d_n13;

        let assign84460_e129293: f64 = if locals.var_wdep_func > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1965 = assign84460_e129293;

        let (assign84470_e129303, assign84470_e129303_d_n0, assign84470_e129303_d_n2, assign84470_e129303_d_n4, assign84470_e129303_d_n5, assign84470_e129303_d_n6, assign84470_e129303_d_n7, assign84470_e129303_d_n8, assign84470_e129303_d_n9, assign84470_e129303_d_n10, assign84470_e129303_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1964 != 0.0)) && (locals.var_guard1965 != 0.0)) {
        let assign84470_e129301: f64 = (p.p334 - locals.var_wdep_func);
        (assign84470_e129301, (-locals.var_wdep_func_dn0), (-locals.var_wdep_func_dn2), (-locals.var_wdep_func_dn4), (-locals.var_wdep_func_dn5), (-locals.var_wdep_func_dn6), (-locals.var_wdep_func_dn7), (-locals.var_wdep_func_dn8), (-locals.var_wdep_func_dn9), (-locals.var_wdep_func_dn10), (-locals.var_wdep_func_dn13),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign84470_e129303;
        locals.var_t2_dn0 = assign84470_e129303_d_n0;
        locals.var_t2_dn2 = assign84470_e129303_d_n2;
        locals.var_t2_dn4 = assign84470_e129303_d_n4;
        locals.var_t2_dn5 = assign84470_e129303_d_n5;
        locals.var_t2_dn6 = assign84470_e129303_d_n6;
        locals.var_t2_dn7 = assign84470_e129303_d_n7;
        locals.var_t2_dn8 = assign84470_e129303_d_n8;
        locals.var_t2_dn9 = assign84470_e129303_d_n9;
        locals.var_t2_dn10 = assign84470_e129303_d_n10;
        locals.var_t2_dn13 = assign84470_e129303_d_n13;

        let (assign84480_e129325, assign84480_e129325_d_n0, assign84480_e129325_d_n2, assign84480_e129325_d_n4, assign84480_e129325_d_n5, assign84480_e129325_d_n6, assign84480_e129325_d_n7, assign84480_e129325_d_n8, assign84480_e129325_d_n9, assign84480_e129325_d_n10, assign84480_e129325_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1964 != 0.0)) && (locals.var_guard1965 == 0.0)) {
        let assign84480_e129312: f64 = (locals.var_vdsi + p.p137);
        let assign84480_e129315: f64 = (locals.var_vdsi + p.p137);
        let assign84480_e129316: f64 = (assign84480_e129312 * assign84480_e129315);
        let assign84480_e129319: f64 = (4.0 * 0.1);
        let assign84480_e129321: f64 = (assign84480_e129319 * 0.1);
        let assign84480_e129322: f64 = (assign84480_e129316 + assign84480_e129321);
        let assign84480_e129323: f64 = (assign84480_e129322).sqrt();
        (assign84480_e129323, 0.0, 0.0, 0.0, (((locals.var_vdsi_dn5 * assign84480_e129315) + (assign84480_e129312 * locals.var_vdsi_dn5)) / (2.0 * assign84480_e129323)), 0.0, (((locals.var_vdsi_dn7 * assign84480_e129315) + (assign84480_e129312 * locals.var_vdsi_dn7)) / (2.0 * assign84480_e129323)), 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign84480_e129325;
        locals.var_tmf2_dn0 = assign84480_e129325_d_n0;
        locals.var_tmf2_dn2 = assign84480_e129325_d_n2;
        locals.var_tmf2_dn4 = assign84480_e129325_d_n4;
        locals.var_tmf2_dn5 = assign84480_e129325_d_n5;
        locals.var_tmf2_dn6 = assign84480_e129325_d_n6;
        locals.var_tmf2_dn7 = assign84480_e129325_d_n7;
        locals.var_tmf2_dn8 = assign84480_e129325_d_n8;
        locals.var_tmf2_dn9 = assign84480_e129325_d_n9;
        locals.var_tmf2_dn10 = assign84480_e129325_d_n10;
        locals.var_tmf2_dn13 = assign84480_e129325_d_n13;

        let (assign84490_e129342, assign84490_e129342_d_n0, assign84490_e129342_d_n2, assign84490_e129342_d_n4, assign84490_e129342_d_n5, assign84490_e129342_d_n6, assign84490_e129342_d_n7, assign84490_e129342_d_n8, assign84490_e129342_d_n9, assign84490_e129342_d_n10, assign84490_e129342_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1964 != 0.0)) && (locals.var_guard1965 == 0.0)) {
        let assign84490_e129336: f64 = (locals.var_vdsi + p.p137);
        let assign84490_e129338: f64 = (assign84490_e129336 / locals.var_tmf2);
        let assign84490_e129339: f64 = (1.0 + assign84490_e129338);
        let assign84490_e129340: f64 = (0.5 * assign84490_e129339);
        (assign84490_e129340, (0.5 * (-((assign84490_e129336 * locals.var_tmf2_dn0) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((assign84490_e129336 * locals.var_tmf2_dn2) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((assign84490_e129336 * locals.var_tmf2_dn4) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (((locals.var_vdsi_dn5 * locals.var_tmf2) - (assign84490_e129336 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (-((assign84490_e129336 * locals.var_tmf2_dn6) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (((locals.var_vdsi_dn7 * locals.var_tmf2) - (assign84490_e129336 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (-((assign84490_e129336 * locals.var_tmf2_dn8) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((assign84490_e129336 * locals.var_tmf2_dn9) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((assign84490_e129336 * locals.var_tmf2_dn10) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((assign84490_e129336 * locals.var_tmf2_dn13) / (locals.var_tmf2 * locals.var_tmf2)))),)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn13,)
    }
};
        locals.var_t9 = assign84490_e129342;
        locals.var_t9_dn0 = assign84490_e129342_d_n0;
        locals.var_t9_dn2 = assign84490_e129342_d_n2;
        locals.var_t9_dn4 = assign84490_e129342_d_n4;
        locals.var_t9_dn5 = assign84490_e129342_d_n5;
        locals.var_t9_dn6 = assign84490_e129342_d_n6;
        locals.var_t9_dn7 = assign84490_e129342_d_n7;
        locals.var_t9_dn8 = assign84490_e129342_d_n8;
        locals.var_t9_dn9 = assign84490_e129342_d_n9;
        locals.var_t9_dn10 = assign84490_e129342_d_n10;
        locals.var_t9_dn13 = assign84490_e129342_d_n13;

        let (assign84500_e129357, assign84500_e129357_d_n0, assign84500_e129357_d_n2, assign84500_e129357_d_n4, assign84500_e129357_d_n5, assign84500_e129357_d_n6, assign84500_e129357_d_n7, assign84500_e129357_d_n8, assign84500_e129357_d_n9, assign84500_e129357_d_n10, assign84500_e129357_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1964 != 0.0)) && (locals.var_guard1965 == 0.0)) {
        let assign84500_e129352: f64 = (locals.var_vdsi + p.p137);
        let assign84500_e129354: f64 = (assign84500_e129352 + locals.var_tmf2);
        let assign84500_e129355: f64 = (0.5 * assign84500_e129354);
        (assign84500_e129355, (0.5 * locals.var_tmf2_dn0), (0.5 * locals.var_tmf2_dn2), (0.5 * locals.var_tmf2_dn4), (0.5 * (locals.var_vdsi_dn5 + locals.var_tmf2_dn5)), (0.5 * locals.var_tmf2_dn6), (0.5 * (locals.var_vdsi_dn7 + locals.var_tmf2_dn7)), (0.5 * locals.var_tmf2_dn8), (0.5 * locals.var_tmf2_dn9), (0.5 * locals.var_tmf2_dn10), (0.5 * locals.var_tmf2_dn13),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign84500_e129357;
        locals.var_t2_dn0 = assign84500_e129357_d_n0;
        locals.var_t2_dn2 = assign84500_e129357_d_n2;
        locals.var_t2_dn4 = assign84500_e129357_d_n4;
        locals.var_t2_dn5 = assign84500_e129357_d_n5;
        locals.var_t2_dn6 = assign84500_e129357_d_n6;
        locals.var_t2_dn7 = assign84500_e129357_d_n7;
        locals.var_t2_dn8 = assign84500_e129357_d_n8;
        locals.var_t2_dn9 = assign84500_e129357_d_n9;
        locals.var_t2_dn10 = assign84500_e129357_d_n10;
        locals.var_t2_dn13 = assign84500_e129357_d_n13;

        let assign84510_e129360: f64 = if locals.var_t2 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1966 = assign84510_e129360;

        let (assign84520_e129371, assign84520_e129371_d_n0, assign84520_e129371_d_n2, assign84520_e129371_d_n4, assign84520_e129371_d_n5, assign84520_e129371_d_n6, assign84520_e129371_d_n7, assign84520_e129371_d_n8, assign84520_e129371_d_n9, assign84520_e129371_d_n10, assign84520_e129371_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1964 != 0.0)) && (locals.var_guard1965 == 0.0)) && (locals.var_guard1966 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign84520_e129371;
        locals.var_t2_dn0 = assign84520_e129371_d_n0;
        locals.var_t2_dn2 = assign84520_e129371_d_n2;
        locals.var_t2_dn4 = assign84520_e129371_d_n4;
        locals.var_t2_dn5 = assign84520_e129371_d_n5;
        locals.var_t2_dn6 = assign84520_e129371_d_n6;
        locals.var_t2_dn7 = assign84520_e129371_d_n7;
        locals.var_t2_dn8 = assign84520_e129371_d_n8;
        locals.var_t2_dn9 = assign84520_e129371_d_n9;
        locals.var_t2_dn10 = assign84520_e129371_d_n10;
        locals.var_t2_dn13 = assign84520_e129371_d_n13;

        let (assign84530_e129382, assign84530_e129382_d_n0, assign84530_e129382_d_n2, assign84530_e129382_d_n4, assign84530_e129382_d_n5, assign84530_e129382_d_n6, assign84530_e129382_d_n7, assign84530_e129382_d_n8, assign84530_e129382_d_n9, assign84530_e129382_d_n10, assign84530_e129382_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1964 != 0.0)) && (locals.var_guard1965 == 0.0)) && (locals.var_guard1966 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn13,)
    }
};
        locals.var_t9 = assign84530_e129382;
        locals.var_t9_dn0 = assign84530_e129382_d_n0;
        locals.var_t9_dn2 = assign84530_e129382_d_n2;
        locals.var_t9_dn4 = assign84530_e129382_d_n4;
        locals.var_t9_dn5 = assign84530_e129382_d_n5;
        locals.var_t9_dn6 = assign84530_e129382_d_n6;
        locals.var_t9_dn7 = assign84530_e129382_d_n7;
        locals.var_t9_dn8 = assign84530_e129382_d_n8;
        locals.var_t9_dn9 = assign84530_e129382_d_n9;
        locals.var_t9_dn10 = assign84530_e129382_d_n10;
        locals.var_t9_dn13 = assign84530_e129382_d_n13;

        let (assign84540_e129396, assign84540_e129396_d_n0, assign84540_e129396_d_n2, assign84540_e129396_d_n4, assign84540_e129396_d_n5, assign84540_e129396_d_n6, assign84540_e129396_d_n7, assign84540_e129396_d_n8, assign84540_e129396_d_n9, assign84540_e129396_d_n10, assign84540_e129396_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1964 != 0.0)) && (locals.var_guard1965 == 0.0)) {
        let assign84540_e129391: f64 = (locals.var_kjunc * locals.var_t2);
        let assign84540_e129392: f64 = (assign84540_e129391).sqrt();
        let assign84540_e129394: f64 = (assign84540_e129392 * p.p432);
        (assign84540_e129394, ((((locals.var_kjunc_dn0 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn0)) / (2.0 * assign84540_e129392)) * p.p432), ((((locals.var_kjunc_dn2 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn2)) / (2.0 * assign84540_e129392)) * p.p432), ((((locals.var_kjunc_dn4 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn4)) / (2.0 * assign84540_e129392)) * p.p432), ((((locals.var_kjunc_dn5 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn5)) / (2.0 * assign84540_e129392)) * p.p432), ((((locals.var_kjunc_dn6 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn6)) / (2.0 * assign84540_e129392)) * p.p432), ((((locals.var_kjunc_dn7 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn7)) / (2.0 * assign84540_e129392)) * p.p432), ((((locals.var_kjunc_dn8 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn8)) / (2.0 * assign84540_e129392)) * p.p432), ((((locals.var_kjunc_dn9 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn9)) / (2.0 * assign84540_e129392)) * p.p432), ((((locals.var_kjunc_dn10 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn10)) / (2.0 * assign84540_e129392)) * p.p432), ((((locals.var_kjunc_dn13 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn13)) / (2.0 * assign84540_e129392)) * p.p432),)
    } else {
        (locals.var_wjunc0, locals.var_wjunc0_dn0, locals.var_wjunc0_dn2, locals.var_wjunc0_dn4, locals.var_wjunc0_dn5, locals.var_wjunc0_dn6, locals.var_wjunc0_dn7, locals.var_wjunc0_dn8, locals.var_wjunc0_dn9, locals.var_wjunc0_dn10, locals.var_wjunc0_dn13,)
    }
};
        locals.var_wjunc0 = assign84540_e129396;
        locals.var_wjunc0_dn0 = assign84540_e129396_d_n0;
        locals.var_wjunc0_dn2 = assign84540_e129396_d_n2;
        locals.var_wjunc0_dn4 = assign84540_e129396_d_n4;
        locals.var_wjunc0_dn5 = assign84540_e129396_d_n5;
        locals.var_wjunc0_dn6 = assign84540_e129396_d_n6;
        locals.var_wjunc0_dn7 = assign84540_e129396_d_n7;
        locals.var_wjunc0_dn8 = assign84540_e129396_d_n8;
        locals.var_wjunc0_dn9 = assign84540_e129396_d_n9;
        locals.var_wjunc0_dn10 = assign84540_e129396_d_n10;
        locals.var_wjunc0_dn13 = assign84540_e129396_d_n13;

        let (assign84550_e129407, assign84550_e129407_d_n0, assign84550_e129407_d_n2, assign84550_e129407_d_n4, assign84550_e129407_d_n5, assign84550_e129407_d_n6, assign84550_e129407_d_n7, assign84550_e129407_d_n8, assign84550_e129407_d_n9, assign84550_e129407_d_n10, assign84550_e129407_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1964 != 0.0)) && (locals.var_guard1965 == 0.0)) {
        let assign84550_e129405: f64 = (p.p334 - locals.var_wjunc0);
        (assign84550_e129405, (-locals.var_wjunc0_dn0), (-locals.var_wjunc0_dn2), (-locals.var_wjunc0_dn4), (-locals.var_wjunc0_dn5), (-locals.var_wjunc0_dn6), (-locals.var_wjunc0_dn7), (-locals.var_wjunc0_dn8), (-locals.var_wjunc0_dn9), (-locals.var_wjunc0_dn10), (-locals.var_wjunc0_dn13),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign84550_e129407;
        locals.var_t2_dn0 = assign84550_e129407_d_n0;
        locals.var_t2_dn2 = assign84550_e129407_d_n2;
        locals.var_t2_dn4 = assign84550_e129407_d_n4;
        locals.var_t2_dn5 = assign84550_e129407_d_n5;
        locals.var_t2_dn6 = assign84550_e129407_d_n6;
        locals.var_t2_dn7 = assign84550_e129407_d_n7;
        locals.var_t2_dn8 = assign84550_e129407_d_n8;
        locals.var_t2_dn9 = assign84550_e129407_d_n9;
        locals.var_t2_dn10 = assign84550_e129407_d_n10;
        locals.var_t2_dn13 = assign84550_e129407_d_n13;

        let (assign84560_e129426, assign84560_e129426_d_n0, assign84560_e129426_d_n2, assign84560_e129426_d_n4, assign84560_e129426_d_n5, assign84560_e129426_d_n6, assign84560_e129426_d_n7, assign84560_e129426_d_n8, assign84560_e129426_d_n9, assign84560_e129426_d_n10, assign84560_e129426_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1964 != 0.0)) {
        let assign84560_e129413: f64 = (locals.var_t2 * locals.var_t2);
        let assign84560_e129417: f64 = (p.p334 * 0.01);
        let assign84560_e129418: f64 = (4.0 * assign84560_e129417);
        let assign84560_e129421: f64 = (p.p334 * 0.01);
        let assign84560_e129422: f64 = (assign84560_e129418 * assign84560_e129421);
        let assign84560_e129423: f64 = (assign84560_e129413 + assign84560_e129422);
        let assign84560_e129424: f64 = (assign84560_e129423).sqrt();
        (assign84560_e129424, (((locals.var_t2_dn0 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn0)) / (2.0 * assign84560_e129424)), (((locals.var_t2_dn2 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn2)) / (2.0 * assign84560_e129424)), (((locals.var_t2_dn4 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn4)) / (2.0 * assign84560_e129424)), (((locals.var_t2_dn5 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn5)) / (2.0 * assign84560_e129424)), (((locals.var_t2_dn6 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn6)) / (2.0 * assign84560_e129424)), (((locals.var_t2_dn7 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn7)) / (2.0 * assign84560_e129424)), (((locals.var_t2_dn8 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn8)) / (2.0 * assign84560_e129424)), (((locals.var_t2_dn9 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn9)) / (2.0 * assign84560_e129424)), (((locals.var_t2_dn10 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn10)) / (2.0 * assign84560_e129424)), (((locals.var_t2_dn13 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn13)) / (2.0 * assign84560_e129424)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign84560_e129426;
        locals.var_tmf2_dn0 = assign84560_e129426_d_n0;
        locals.var_tmf2_dn2 = assign84560_e129426_d_n2;
        locals.var_tmf2_dn4 = assign84560_e129426_d_n4;
        locals.var_tmf2_dn5 = assign84560_e129426_d_n5;
        locals.var_tmf2_dn6 = assign84560_e129426_d_n6;
        locals.var_tmf2_dn7 = assign84560_e129426_d_n7;
        locals.var_tmf2_dn8 = assign84560_e129426_d_n8;
        locals.var_tmf2_dn9 = assign84560_e129426_d_n9;
        locals.var_tmf2_dn10 = assign84560_e129426_d_n10;
        locals.var_tmf2_dn13 = assign84560_e129426_d_n13;

        let (assign84570_e129438, assign84570_e129438_d_n0, assign84570_e129438_d_n2, assign84570_e129438_d_n4, assign84570_e129438_d_n5, assign84570_e129438_d_n6, assign84570_e129438_d_n7, assign84570_e129438_d_n8, assign84570_e129438_d_n9, assign84570_e129438_d_n10, assign84570_e129438_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1964 != 0.0)) {
        let assign84570_e129434: f64 = (locals.var_t2 / locals.var_tmf2);
        let assign84570_e129435: f64 = (1.0 + assign84570_e129434);
        let assign84570_e129436: f64 = (0.5 * assign84570_e129435);
        (assign84570_e129436, (0.5 * (((locals.var_t2_dn0 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn2 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn4 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn5 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn6 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn7 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn8 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn9 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn10 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn13 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn13)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn13,)
    }
};
        locals.var_t9 = assign84570_e129438;
        locals.var_t9_dn0 = assign84570_e129438_d_n0;
        locals.var_t9_dn2 = assign84570_e129438_d_n2;
        locals.var_t9_dn4 = assign84570_e129438_d_n4;
        locals.var_t9_dn5 = assign84570_e129438_d_n5;
        locals.var_t9_dn6 = assign84570_e129438_d_n6;
        locals.var_t9_dn7 = assign84570_e129438_d_n7;
        locals.var_t9_dn8 = assign84570_e129438_d_n8;
        locals.var_t9_dn9 = assign84570_e129438_d_n9;
        locals.var_t9_dn10 = assign84570_e129438_d_n10;
        locals.var_t9_dn13 = assign84570_e129438_d_n13;

    }

    pub(super) fn stamp_transient_block_296(
        locals: &mut StampLocals,
    ) {
        let (assign84580_e129448, assign84580_e129448_d_n0, assign84580_e129448_d_n2, assign84580_e129448_d_n4, assign84580_e129448_d_n5, assign84580_e129448_d_n6, assign84580_e129448_d_n7, assign84580_e129448_d_n8, assign84580_e129448_d_n9, assign84580_e129448_d_n10, assign84580_e129448_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1964 != 0.0)) {
        let assign84580_e129445: f64 = (locals.var_t2 + locals.var_tmf2);
        let assign84580_e129446: f64 = (0.5 * assign84580_e129445);
        (assign84580_e129446, (0.5 * (locals.var_t2_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_t2_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_t2_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_t2_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_t2_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_t2_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_t2_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_t2_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_t2_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_t2_dn13 + locals.var_tmf2_dn13)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign84580_e129448;
        locals.var_t2_dn0 = assign84580_e129448_d_n0;
        locals.var_t2_dn2 = assign84580_e129448_d_n2;
        locals.var_t2_dn4 = assign84580_e129448_d_n4;
        locals.var_t2_dn5 = assign84580_e129448_d_n5;
        locals.var_t2_dn6 = assign84580_e129448_d_n6;
        locals.var_t2_dn7 = assign84580_e129448_d_n7;
        locals.var_t2_dn8 = assign84580_e129448_d_n8;
        locals.var_t2_dn9 = assign84580_e129448_d_n9;
        locals.var_t2_dn10 = assign84580_e129448_d_n10;
        locals.var_t2_dn13 = assign84580_e129448_d_n13;

        let assign84590_e129451: f64 = if locals.var_t2 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1967 = assign84590_e129451;

        let (assign84600_e129459, assign84600_e129459_d_n0, assign84600_e129459_d_n2, assign84600_e129459_d_n4, assign84600_e129459_d_n5, assign84600_e129459_d_n6, assign84600_e129459_d_n7, assign84600_e129459_d_n8, assign84600_e129459_d_n9, assign84600_e129459_d_n10, assign84600_e129459_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1964 != 0.0)) && (locals.var_guard1967 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign84600_e129459;
        locals.var_t2_dn0 = assign84600_e129459_d_n0;
        locals.var_t2_dn2 = assign84600_e129459_d_n2;
        locals.var_t2_dn4 = assign84600_e129459_d_n4;
        locals.var_t2_dn5 = assign84600_e129459_d_n5;
        locals.var_t2_dn6 = assign84600_e129459_d_n6;
        locals.var_t2_dn7 = assign84600_e129459_d_n7;
        locals.var_t2_dn8 = assign84600_e129459_d_n8;
        locals.var_t2_dn9 = assign84600_e129459_d_n9;
        locals.var_t2_dn10 = assign84600_e129459_d_n10;
        locals.var_t2_dn13 = assign84600_e129459_d_n13;

        let (assign84610_e129467, assign84610_e129467_d_n0, assign84610_e129467_d_n2, assign84610_e129467_d_n4, assign84610_e129467_d_n5, assign84610_e129467_d_n6, assign84610_e129467_d_n7, assign84610_e129467_d_n8, assign84610_e129467_d_n9, assign84610_e129467_d_n10, assign84610_e129467_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1964 != 0.0)) && (locals.var_guard1967 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn13,)
    }
};
        locals.var_t9 = assign84610_e129467;
        locals.var_t9_dn0 = assign84610_e129467_d_n0;
        locals.var_t9_dn2 = assign84610_e129467_d_n2;
        locals.var_t9_dn4 = assign84610_e129467_d_n4;
        locals.var_t9_dn5 = assign84610_e129467_d_n5;
        locals.var_t9_dn6 = assign84610_e129467_d_n6;
        locals.var_t9_dn7 = assign84610_e129467_d_n7;
        locals.var_t9_dn8 = assign84610_e129467_d_n8;
        locals.var_t9_dn9 = assign84610_e129467_d_n9;
        locals.var_t9_dn10 = assign84610_e129467_d_n10;
        locals.var_t9_dn13 = assign84610_e129467_d_n13;

        let (assign84620_e129473, assign84620_e129473_d_n0, assign84620_e129473_d_n2, assign84620_e129473_d_n4, assign84620_e129473_d_n5, assign84620_e129473_d_n6, assign84620_e129473_d_n7, assign84620_e129473_d_n8, assign84620_e129473_d_n9, assign84620_e129473_d_n10, assign84620_e129473_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1964 != 0.0)) {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    } else {
        (locals.var_ddriftldc, locals.var_ddriftldc_dn0, locals.var_ddriftldc_dn2, locals.var_ddriftldc_dn4, locals.var_ddriftldc_dn5, locals.var_ddriftldc_dn6, locals.var_ddriftldc_dn7, locals.var_ddriftldc_dn8, locals.var_ddriftldc_dn9, locals.var_ddriftldc_dn10, locals.var_ddriftldc_dn13,)
    }
};
        locals.var_ddriftldc = assign84620_e129473;
        locals.var_ddriftldc_dn0 = assign84620_e129473_d_n0;
        locals.var_ddriftldc_dn2 = assign84620_e129473_d_n2;
        locals.var_ddriftldc_dn4 = assign84620_e129473_d_n4;
        locals.var_ddriftldc_dn5 = assign84620_e129473_d_n5;
        locals.var_ddriftldc_dn6 = assign84620_e129473_d_n6;
        locals.var_ddriftldc_dn7 = assign84620_e129473_d_n7;
        locals.var_ddriftldc_dn8 = assign84620_e129473_d_n8;
        locals.var_ddriftldc_dn9 = assign84620_e129473_d_n9;
        locals.var_ddriftldc_dn10 = assign84620_e129473_d_n10;
        locals.var_ddriftldc_dn13 = assign84620_e129473_d_n13;

        let (assign84630_e129487, assign84630_e129487_d_n0, assign84630_e129487_d_n2, assign84630_e129487_d_n4, assign84630_e129487_d_n5, assign84630_e129487_d_n6, assign84630_e129487_d_n7, assign84630_e129487_d_n8, assign84630_e129487_d_n9, assign84630_e129487_d_n10, assign84630_e129487_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1964 != 0.0)) {
        let assign84630_e129479: f64 = (locals.var_q_nsubld__blk1883 * locals.var_ddriftldc);
        let assign84630_e129481: f64 = (assign84630_e129479 * locals.var_ddriftldc);
        let assign84630_e129483: f64 = (assign84630_e129481 / 2.0);
        let assign84630_e129485: f64 = (assign84630_e129483 / 1.034943e-10);
        (assign84630_e129485, (((((locals.var_q_nsubld__blk1883 * locals.var_ddriftldc_dn0) * locals.var_ddriftldc) + (assign84630_e129479 * locals.var_ddriftldc_dn0)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk1883 * locals.var_ddriftldc_dn2) * locals.var_ddriftldc) + (assign84630_e129479 * locals.var_ddriftldc_dn2)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk1883 * locals.var_ddriftldc_dn4) * locals.var_ddriftldc) + (assign84630_e129479 * locals.var_ddriftldc_dn4)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk1883 * locals.var_ddriftldc_dn5) * locals.var_ddriftldc) + (assign84630_e129479 * locals.var_ddriftldc_dn5)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk1883 * locals.var_ddriftldc_dn6) * locals.var_ddriftldc) + (assign84630_e129479 * locals.var_ddriftldc_dn6)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk1883 * locals.var_ddriftldc_dn7) * locals.var_ddriftldc) + (assign84630_e129479 * locals.var_ddriftldc_dn7)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk1883 * locals.var_ddriftldc_dn8) * locals.var_ddriftldc) + (assign84630_e129479 * locals.var_ddriftldc_dn8)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk1883 * locals.var_ddriftldc_dn9) * locals.var_ddriftldc) + (assign84630_e129479 * locals.var_ddriftldc_dn9)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk1883 * locals.var_ddriftldc_dn10) * locals.var_ddriftldc) + (assign84630_e129479 * locals.var_ddriftldc_dn10)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk1883 * locals.var_ddriftldc_dn13) * locals.var_ddriftldc) + (assign84630_e129479 * locals.var_ddriftldc_dn13)) / 2.0) / 1.034943e-10),)
    } else {
        (locals.var_dphi_sb, locals.var_dphi_sb_dn0, locals.var_dphi_sb_dn2, locals.var_dphi_sb_dn4, locals.var_dphi_sb_dn5, locals.var_dphi_sb_dn6, locals.var_dphi_sb_dn7, locals.var_dphi_sb_dn8, locals.var_dphi_sb_dn9, locals.var_dphi_sb_dn10, locals.var_dphi_sb_dn13,)
    }
};
        locals.var_dphi_sb = assign84630_e129487;
        locals.var_dphi_sb_dn0 = assign84630_e129487_d_n0;
        locals.var_dphi_sb_dn2 = assign84630_e129487_d_n2;
        locals.var_dphi_sb_dn4 = assign84630_e129487_d_n4;
        locals.var_dphi_sb_dn5 = assign84630_e129487_d_n5;
        locals.var_dphi_sb_dn6 = assign84630_e129487_d_n6;
        locals.var_dphi_sb_dn7 = assign84630_e129487_d_n7;
        locals.var_dphi_sb_dn8 = assign84630_e129487_d_n8;
        locals.var_dphi_sb_dn9 = assign84630_e129487_d_n9;
        locals.var_dphi_sb_dn10 = assign84630_e129487_d_n10;
        locals.var_dphi_sb_dn13 = assign84630_e129487_d_n13;

        let (assign84640_e129498, assign84640_e129498_d_n0, assign84640_e129498_d_n2, assign84640_e129498_d_n4, assign84640_e129498_d_n5, assign84640_e129498_d_n6, assign84640_e129498_d_n7, assign84640_e129498_d_n8, assign84640_e129498_d_n9, assign84640_e129498_d_n10, assign84640_e129498_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1964 != 0.0)) {
        let assign84640_e129493: f64 = (2.0 * locals.var_beta);
        let assign84640_e129495: f64 = (assign84640_e129493 * locals.var_dphi_sb);
        let assign84640_e129496: f64 = (assign84640_e129495).sqrt();
        (assign84640_e129496, ((((2.0 * locals.var_beta_dn0) * locals.var_dphi_sb) + (assign84640_e129493 * locals.var_dphi_sb_dn0)) / (2.0 * assign84640_e129496)), ((((2.0 * locals.var_beta_dn2) * locals.var_dphi_sb) + (assign84640_e129493 * locals.var_dphi_sb_dn2)) / (2.0 * assign84640_e129496)), ((((2.0 * locals.var_beta_dn4) * locals.var_dphi_sb) + (assign84640_e129493 * locals.var_dphi_sb_dn4)) / (2.0 * assign84640_e129496)), ((((2.0 * locals.var_beta_dn5) * locals.var_dphi_sb) + (assign84640_e129493 * locals.var_dphi_sb_dn5)) / (2.0 * assign84640_e129496)), ((((2.0 * locals.var_beta_dn6) * locals.var_dphi_sb) + (assign84640_e129493 * locals.var_dphi_sb_dn6)) / (2.0 * assign84640_e129496)), ((((2.0 * locals.var_beta_dn7) * locals.var_dphi_sb) + (assign84640_e129493 * locals.var_dphi_sb_dn7)) / (2.0 * assign84640_e129496)), ((((2.0 * locals.var_beta_dn8) * locals.var_dphi_sb) + (assign84640_e129493 * locals.var_dphi_sb_dn8)) / (2.0 * assign84640_e129496)), ((((2.0 * locals.var_beta_dn9) * locals.var_dphi_sb) + (assign84640_e129493 * locals.var_dphi_sb_dn9)) / (2.0 * assign84640_e129496)), ((((2.0 * locals.var_beta_dn10) * locals.var_dphi_sb) + (assign84640_e129493 * locals.var_dphi_sb_dn10)) / (2.0 * assign84640_e129496)), ((((2.0 * locals.var_beta_dn13) * locals.var_dphi_sb) + (assign84640_e129493 * locals.var_dphi_sb_dn13)) / (2.0 * assign84640_e129496)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign84640_e129498;
        locals.var_t0_dn0 = assign84640_e129498_d_n0;
        locals.var_t0_dn2 = assign84640_e129498_d_n2;
        locals.var_t0_dn4 = assign84640_e129498_d_n4;
        locals.var_t0_dn5 = assign84640_e129498_d_n5;
        locals.var_t0_dn6 = assign84640_e129498_d_n6;
        locals.var_t0_dn7 = assign84640_e129498_d_n7;
        locals.var_t0_dn8 = assign84640_e129498_d_n8;
        locals.var_t0_dn9 = assign84640_e129498_d_n9;
        locals.var_t0_dn10 = assign84640_e129498_d_n10;
        locals.var_t0_dn13 = assign84640_e129498_d_n13;

        let (assign84650_e129511, assign84650_e129511_d_n0, assign84650_e129511_d_n2, assign84650_e129511_d_n4, assign84650_e129511_d_n5, assign84650_e129511_d_n6, assign84650_e129511_d_n7, assign84650_e129511_d_n8, assign84650_e129511_d_n9, assign84650_e129511_d_n10, assign84650_e129511_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1964 != 0.0)) {
        let assign84650_e129503: f64 = { let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign84650_e129505: f64 = (-locals.var_t0);
        let assign84650_e129506: f64 = { let limited_exp_arg = assign84650_e129505; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign84650_e129507: f64 = (assign84650_e129503 + assign84650_e129506);
        let assign84650_e129509: f64 = (assign84650_e129507 / 2.0);
        (assign84650_e129509, ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn0) + ({ let limited_exp_arg = assign84650_e129505; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn0))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn2) + ({ let limited_exp_arg = assign84650_e129505; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn2))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn4) + ({ let limited_exp_arg = assign84650_e129505; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn4))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn5) + ({ let limited_exp_arg = assign84650_e129505; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn5))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn6) + ({ let limited_exp_arg = assign84650_e129505; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn6))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn7) + ({ let limited_exp_arg = assign84650_e129505; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn7))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn8) + ({ let limited_exp_arg = assign84650_e129505; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn8))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn9) + ({ let limited_exp_arg = assign84650_e129505; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn9))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn10) + ({ let limited_exp_arg = assign84650_e129505; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn10))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn13) + ({ let limited_exp_arg = assign84650_e129505; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn13))) / 2.0),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign84650_e129511;
        locals.var_t1_dn0 = assign84650_e129511_d_n0;
        locals.var_t1_dn2 = assign84650_e129511_d_n2;
        locals.var_t1_dn4 = assign84650_e129511_d_n4;
        locals.var_t1_dn5 = assign84650_e129511_d_n5;
        locals.var_t1_dn6 = assign84650_e129511_d_n6;
        locals.var_t1_dn7 = assign84650_e129511_d_n7;
        locals.var_t1_dn8 = assign84650_e129511_d_n8;
        locals.var_t1_dn9 = assign84650_e129511_d_n9;
        locals.var_t1_dn10 = assign84650_e129511_d_n10;
        locals.var_t1_dn13 = assign84650_e129511_d_n13;

        let (assign84660_e129520, assign84660_e129520_d_n0, assign84660_e129520_d_n2, assign84660_e129520_d_n4, assign84660_e129520_d_n5, assign84660_e129520_d_n6, assign84660_e129520_d_n7, assign84660_e129520_d_n8, assign84660_e129520_d_n9, assign84660_e129520_d_n10, assign84660_e129520_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1964 != 0.0)) {
        let assign84660_e129516: f64 = (locals.var_t1).ln();
        let assign84660_e129518: f64 = (assign84660_e129516 / locals.var_dphi_sb);
        (assign84660_e129518, ((((locals.var_t1_dn0 / locals.var_t1) * locals.var_dphi_sb) - (assign84660_e129516 * locals.var_dphi_sb_dn0)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn2 / locals.var_t1) * locals.var_dphi_sb) - (assign84660_e129516 * locals.var_dphi_sb_dn2)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn4 / locals.var_t1) * locals.var_dphi_sb) - (assign84660_e129516 * locals.var_dphi_sb_dn4)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn5 / locals.var_t1) * locals.var_dphi_sb) - (assign84660_e129516 * locals.var_dphi_sb_dn5)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn6 / locals.var_t1) * locals.var_dphi_sb) - (assign84660_e129516 * locals.var_dphi_sb_dn6)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn7 / locals.var_t1) * locals.var_dphi_sb) - (assign84660_e129516 * locals.var_dphi_sb_dn7)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn8 / locals.var_t1) * locals.var_dphi_sb) - (assign84660_e129516 * locals.var_dphi_sb_dn8)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn9 / locals.var_t1) * locals.var_dphi_sb) - (assign84660_e129516 * locals.var_dphi_sb_dn9)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn10 / locals.var_t1) * locals.var_dphi_sb) - (assign84660_e129516 * locals.var_dphi_sb_dn10)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn13 / locals.var_t1) * locals.var_dphi_sb) - (assign84660_e129516 * locals.var_dphi_sb_dn13)) / (locals.var_dphi_sb * locals.var_dphi_sb)),)
    } else {
        (locals.var_c_sb, locals.var_c_sb_dn0, locals.var_c_sb_dn2, locals.var_c_sb_dn4, locals.var_c_sb_dn5, locals.var_c_sb_dn6, locals.var_c_sb_dn7, locals.var_c_sb_dn8, locals.var_c_sb_dn9, locals.var_c_sb_dn10, locals.var_c_sb_dn13,)
    }
};
        locals.var_c_sb = assign84660_e129520;
        locals.var_c_sb_dn0 = assign84660_e129520_d_n0;
        locals.var_c_sb_dn2 = assign84660_e129520_d_n2;
        locals.var_c_sb_dn4 = assign84660_e129520_d_n4;
        locals.var_c_sb_dn5 = assign84660_e129520_d_n5;
        locals.var_c_sb_dn6 = assign84660_e129520_d_n6;
        locals.var_c_sb_dn7 = assign84660_e129520_d_n7;
        locals.var_c_sb_dn8 = assign84660_e129520_d_n8;
        locals.var_c_sb_dn9 = assign84660_e129520_d_n9;
        locals.var_c_sb_dn10 = assign84660_e129520_d_n10;
        locals.var_c_sb_dn13 = assign84660_e129520_d_n13;

        let (assign84670_e129526,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1964 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_lp_s0,)
    }
};
        locals.var_lp_s0 = assign84670_e129526;

    }

    pub(super) fn stamp_transient_block_297(
        locals: &mut StampLocals,
    ) {
        let mut assign84680_loop_guard: usize = 0;
        while {
            let assign84680_cond_e129533: f64 = (locals.var_lp_s0_max + 1.0);
            let assign84680_cond_e129535: f64 = if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1964 != 0.0)) && (locals.var_lp_s0 <= assign84680_cond_e129533)) { 1.0 } else { 0.0 };
            assign84680_cond_e129535 != 0.0
        } {
            assign84680_loop_guard += 1;
            assert!(assign84680_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign84680_body3_e129562, assign84680_body3_e129562_d_n0, assign84680_body3_e129562_d_n2, assign84680_body3_e129562_d_n4, assign84680_body3_e129562_d_n5, assign84680_body3_e129562_d_n6, assign84680_body3_e129562_d_n7, assign84680_body3_e129562_d_n8, assign84680_body3_e129562_d_n9, assign84680_body3_e129562_d_n10, assign84680_body3_e129562_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1964 != 0.0)) {
        let assign84680_body3_e129560: f64 = (locals.var_ps0ld + locals.var_vxbgmtcl);
        (assign84680_body3_e129560, (locals.var_ps0ld_dn0 + locals.var_vxbgmtcl_dn0), (locals.var_ps0ld_dn2 + locals.var_vxbgmtcl_dn2), (locals.var_ps0ld_dn4 + locals.var_vxbgmtcl_dn4), (locals.var_ps0ld_dn5 + locals.var_vxbgmtcl_dn5), (locals.var_ps0ld_dn6 + locals.var_vxbgmtcl_dn6), (locals.var_ps0ld_dn7 + locals.var_vxbgmtcl_dn7), (locals.var_ps0ld_dn8 + locals.var_vxbgmtcl_dn8), (locals.var_ps0ld_dn9 + locals.var_vxbgmtcl_dn9), (locals.var_ps0ld_dn10 + locals.var_vxbgmtcl_dn10), (locals.var_ps0ld_dn13 + locals.var_vxbgmtcl_dn13),)
    } else {
        (locals.var_ps0ld_vxb, locals.var_ps0ld_vxb_dn0, locals.var_ps0ld_vxb_dn2, locals.var_ps0ld_vxb_dn4, locals.var_ps0ld_vxb_dn5, locals.var_ps0ld_vxb_dn6, locals.var_ps0ld_vxb_dn7, locals.var_ps0ld_vxb_dn8, locals.var_ps0ld_vxb_dn9, locals.var_ps0ld_vxb_dn10, locals.var_ps0ld_vxb_dn13,)
    }
};
            locals.var_ps0ld_vxb = assign84680_body3_e129562;
            locals.var_ps0ld_vxb_dn0 = assign84680_body3_e129562_d_n0;
            locals.var_ps0ld_vxb_dn2 = assign84680_body3_e129562_d_n2;
            locals.var_ps0ld_vxb_dn4 = assign84680_body3_e129562_d_n4;
            locals.var_ps0ld_vxb_dn5 = assign84680_body3_e129562_d_n5;
            locals.var_ps0ld_vxb_dn6 = assign84680_body3_e129562_d_n6;
            locals.var_ps0ld_vxb_dn7 = assign84680_body3_e129562_d_n7;
            locals.var_ps0ld_vxb_dn8 = assign84680_body3_e129562_d_n8;
            locals.var_ps0ld_vxb_dn9 = assign84680_body3_e129562_d_n9;
            locals.var_ps0ld_vxb_dn10 = assign84680_body3_e129562_d_n10;
            locals.var_ps0ld_vxb_dn13 = assign84680_body3_e129562_d_n13;
            let (assign84680_body4_e129570, assign84680_body4_e129570_d_n0, assign84680_body4_e129570_d_n2, assign84680_body4_e129570_d_n4, assign84680_body4_e129570_d_n5, assign84680_body4_e129570_d_n6, assign84680_body4_e129570_d_n7, assign84680_body4_e129570_d_n8, assign84680_body4_e129570_d_n9, assign84680_body4_e129570_d_n10, assign84680_body4_e129570_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1964 != 0.0)) {
        let assign84680_body4_e129568: f64 = (locals.var_beta * locals.var_ps0ld_vxb);
        (assign84680_body4_e129568, ((locals.var_beta_dn0 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn0)), ((locals.var_beta_dn2 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn2)), ((locals.var_beta_dn4 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn4)), ((locals.var_beta_dn5 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn5)), ((locals.var_beta_dn6 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn6)), ((locals.var_beta_dn7 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn7)), ((locals.var_beta_dn8 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn8)), ((locals.var_beta_dn9 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn9)), ((locals.var_beta_dn10 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn10)), ((locals.var_beta_dn13 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn13)),)
    } else {
        (locals.var_chi, locals.var_chi_dn0, locals.var_chi_dn2, locals.var_chi_dn4, locals.var_chi_dn5, locals.var_chi_dn6, locals.var_chi_dn7, locals.var_chi_dn8, locals.var_chi_dn9, locals.var_chi_dn10, locals.var_chi_dn13,)
    }
};
            locals.var_chi = assign84680_body4_e129570;
            locals.var_chi_dn0 = assign84680_body4_e129570_d_n0;
            locals.var_chi_dn2 = assign84680_body4_e129570_d_n2;
            locals.var_chi_dn4 = assign84680_body4_e129570_d_n4;
            locals.var_chi_dn5 = assign84680_body4_e129570_d_n5;
            locals.var_chi_dn6 = assign84680_body4_e129570_d_n6;
            locals.var_chi_dn7 = assign84680_body4_e129570_d_n7;
            locals.var_chi_dn8 = assign84680_body4_e129570_d_n8;
            locals.var_chi_dn9 = assign84680_body4_e129570_d_n9;
            locals.var_chi_dn10 = assign84680_body4_e129570_d_n10;
            locals.var_chi_dn13 = assign84680_body4_e129570_d_n13;
            let (assign84680_body5_e129580, assign84680_body5_e129580_d_n0, assign84680_body5_e129580_d_n2, assign84680_body5_e129580_d_n4, assign84680_body5_e129580_d_n5, assign84680_body5_e129580_d_n6, assign84680_body5_e129580_d_n7, assign84680_body5_e129580_d_n8, assign84680_body5_e129580_d_n9, assign84680_body5_e129580_d_n10, assign84680_body5_e129580_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1964 != 0.0)) {
        let assign84680_body5_e129577: f64 = (locals.var_ps0ld_vxb - locals.var_dphi_sb);
        let assign84680_body5_e129578: f64 = (locals.var_c_sb * assign84680_body5_e129577);
        (assign84680_body5_e129578, ((locals.var_c_sb_dn0 * assign84680_body5_e129577) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn0 - locals.var_dphi_sb_dn0))), ((locals.var_c_sb_dn2 * assign84680_body5_e129577) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn2 - locals.var_dphi_sb_dn2))), ((locals.var_c_sb_dn4 * assign84680_body5_e129577) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn4 - locals.var_dphi_sb_dn4))), ((locals.var_c_sb_dn5 * assign84680_body5_e129577) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn5 - locals.var_dphi_sb_dn5))), ((locals.var_c_sb_dn6 * assign84680_body5_e129577) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn6 - locals.var_dphi_sb_dn6))), ((locals.var_c_sb_dn7 * assign84680_body5_e129577) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn7 - locals.var_dphi_sb_dn7))), ((locals.var_c_sb_dn8 * assign84680_body5_e129577) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn8 - locals.var_dphi_sb_dn8))), ((locals.var_c_sb_dn9 * assign84680_body5_e129577) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn9 - locals.var_dphi_sb_dn9))), ((locals.var_c_sb_dn10 * assign84680_body5_e129577) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn10 - locals.var_dphi_sb_dn10))), ((locals.var_c_sb_dn13 * assign84680_body5_e129577) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn13 - locals.var_dphi_sb_dn13))),)
    } else {
        (locals.var_ty, locals.var_ty_dn0, locals.var_ty_dn2, locals.var_ty_dn4, locals.var_ty_dn5, locals.var_ty_dn6, locals.var_ty_dn7, locals.var_ty_dn8, locals.var_ty_dn9, locals.var_ty_dn10, locals.var_ty_dn13,)
    }
};
            locals.var_ty = assign84680_body5_e129580;
            locals.var_ty_dn0 = assign84680_body5_e129580_d_n0;
            locals.var_ty_dn2 = assign84680_body5_e129580_d_n2;
            locals.var_ty_dn4 = assign84680_body5_e129580_d_n4;
            locals.var_ty_dn5 = assign84680_body5_e129580_d_n5;
            locals.var_ty_dn6 = assign84680_body5_e129580_d_n6;
            locals.var_ty_dn7 = assign84680_body5_e129580_d_n7;
            locals.var_ty_dn8 = assign84680_body5_e129580_d_n8;
            locals.var_ty_dn9 = assign84680_body5_e129580_d_n9;
            locals.var_ty_dn10 = assign84680_body5_e129580_d_n10;
            locals.var_ty_dn13 = assign84680_body5_e129580_d_n13;
            let assign84680_body6_e129583: f64 = if locals.var_ty < 60.0 { 1.0 } else { 0.0 };
            locals.var_guard1969 = assign84680_body6_e129583;
            let (assign84680_body7_e129592, assign84680_body7_e129592_d_n0, assign84680_body7_e129592_d_n2, assign84680_body7_e129592_d_n4, assign84680_body7_e129592_d_n5, assign84680_body7_e129592_d_n6, assign84680_body7_e129592_d_n7, assign84680_body7_e129592_d_n8, assign84680_body7_e129592_d_n9, assign84680_body7_e129592_d_n10, assign84680_body7_e129592_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1964 != 0.0)) && (locals.var_guard1969 != 0.0)) {
        let assign84680_body7_e129590: f64 = (locals.var_ty).exp();
        (assign84680_body7_e129590, (assign84680_body7_e129590 * locals.var_ty_dn0), (assign84680_body7_e129590 * locals.var_ty_dn2), (assign84680_body7_e129590 * locals.var_ty_dn4), (assign84680_body7_e129590 * locals.var_ty_dn5), (assign84680_body7_e129590 * locals.var_ty_dn6), (assign84680_body7_e129590 * locals.var_ty_dn7), (assign84680_body7_e129590 * locals.var_ty_dn8), (assign84680_body7_e129590 * locals.var_ty_dn9), (assign84680_body7_e129590 * locals.var_ty_dn10), (assign84680_body7_e129590 * locals.var_ty_dn13),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
            locals.var_t1 = assign84680_body7_e129592;
            locals.var_t1_dn0 = assign84680_body7_e129592_d_n0;
            locals.var_t1_dn2 = assign84680_body7_e129592_d_n2;
            locals.var_t1_dn4 = assign84680_body7_e129592_d_n4;
            locals.var_t1_dn5 = assign84680_body7_e129592_d_n5;
            locals.var_t1_dn6 = assign84680_body7_e129592_d_n6;
            locals.var_t1_dn7 = assign84680_body7_e129592_d_n7;
            locals.var_t1_dn8 = assign84680_body7_e129592_d_n8;
            locals.var_t1_dn9 = assign84680_body7_e129592_d_n9;
            locals.var_t1_dn10 = assign84680_body7_e129592_d_n10;
            locals.var_t1_dn13 = assign84680_body7_e129592_d_n13;
            let (assign84680_body8_e129604, assign84680_body8_e129604_d_n0, assign84680_body8_e129604_d_n2, assign84680_body8_e129604_d_n4, assign84680_body8_e129604_d_n5, assign84680_body8_e129604_d_n6, assign84680_body8_e129604_d_n7, assign84680_body8_e129604_d_n8, assign84680_body8_e129604_d_n9, assign84680_body8_e129604_d_n10, assign84680_body8_e129604_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1964 != 0.0)) && (locals.var_guard1969 != 0.0)) {
        let assign84680_body8_e129599: f64 = (-locals.var_c_sb);
        let assign84680_body8_e129601: f64 = (assign84680_body8_e129599 * locals.var_dphi_sb);
        let assign84680_body8_e129602: f64 = (assign84680_body8_e129601).exp();
        (assign84680_body8_e129602, (assign84680_body8_e129602 * (((-locals.var_c_sb_dn0) * locals.var_dphi_sb) + (assign84680_body8_e129599 * locals.var_dphi_sb_dn0))), (assign84680_body8_e129602 * (((-locals.var_c_sb_dn2) * locals.var_dphi_sb) + (assign84680_body8_e129599 * locals.var_dphi_sb_dn2))), (assign84680_body8_e129602 * (((-locals.var_c_sb_dn4) * locals.var_dphi_sb) + (assign84680_body8_e129599 * locals.var_dphi_sb_dn4))), (assign84680_body8_e129602 * (((-locals.var_c_sb_dn5) * locals.var_dphi_sb) + (assign84680_body8_e129599 * locals.var_dphi_sb_dn5))), (assign84680_body8_e129602 * (((-locals.var_c_sb_dn6) * locals.var_dphi_sb) + (assign84680_body8_e129599 * locals.var_dphi_sb_dn6))), (assign84680_body8_e129602 * (((-locals.var_c_sb_dn7) * locals.var_dphi_sb) + (assign84680_body8_e129599 * locals.var_dphi_sb_dn7))), (assign84680_body8_e129602 * (((-locals.var_c_sb_dn8) * locals.var_dphi_sb) + (assign84680_body8_e129599 * locals.var_dphi_sb_dn8))), (assign84680_body8_e129602 * (((-locals.var_c_sb_dn9) * locals.var_dphi_sb) + (assign84680_body8_e129599 * locals.var_dphi_sb_dn9))), (assign84680_body8_e129602 * (((-locals.var_c_sb_dn10) * locals.var_dphi_sb) + (assign84680_body8_e129599 * locals.var_dphi_sb_dn10))), (assign84680_body8_e129602 * (((-locals.var_c_sb_dn13) * locals.var_dphi_sb) + (assign84680_body8_e129599 * locals.var_dphi_sb_dn13))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
            locals.var_t0 = assign84680_body8_e129604;
            locals.var_t0_dn0 = assign84680_body8_e129604_d_n0;
            locals.var_t0_dn2 = assign84680_body8_e129604_d_n2;
            locals.var_t0_dn4 = assign84680_body8_e129604_d_n4;
            locals.var_t0_dn5 = assign84680_body8_e129604_d_n5;
            locals.var_t0_dn6 = assign84680_body8_e129604_d_n6;
            locals.var_t0_dn7 = assign84680_body8_e129604_d_n7;
            locals.var_t0_dn8 = assign84680_body8_e129604_d_n8;
            locals.var_t0_dn9 = assign84680_body8_e129604_d_n9;
            locals.var_t0_dn10 = assign84680_body8_e129604_d_n10;
            locals.var_t0_dn13 = assign84680_body8_e129604_d_n13;
            let (assign84680_body9_e129614, assign84680_body9_e129614_d_n0, assign84680_body9_e129614_d_n2, assign84680_body9_e129614_d_n4, assign84680_body9_e129614_d_n5, assign84680_body9_e129614_d_n6, assign84680_body9_e129614_d_n7, assign84680_body9_e129614_d_n8, assign84680_body9_e129614_d_n9, assign84680_body9_e129614_d_n10, assign84680_body9_e129614_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1964 != 0.0)) && (locals.var_guard1969 != 0.0)) {
        let assign84680_body9_e129612: f64 = (locals.var_t1 - locals.var_t0);
        (assign84680_body9_e129612, (locals.var_t1_dn0 - locals.var_t0_dn0), (locals.var_t1_dn2 - locals.var_t0_dn2), (locals.var_t1_dn4 - locals.var_t0_dn4), (locals.var_t1_dn5 - locals.var_t0_dn5), (locals.var_t1_dn6 - locals.var_t0_dn6), (locals.var_t1_dn7 - locals.var_t0_dn7), (locals.var_t1_dn8 - locals.var_t0_dn8), (locals.var_t1_dn9 - locals.var_t0_dn9), (locals.var_t1_dn10 - locals.var_t0_dn10), (locals.var_t1_dn13 - locals.var_t0_dn13),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
            locals.var_t2 = assign84680_body9_e129614;
            locals.var_t2_dn0 = assign84680_body9_e129614_d_n0;
            locals.var_t2_dn2 = assign84680_body9_e129614_d_n2;
            locals.var_t2_dn4 = assign84680_body9_e129614_d_n4;
            locals.var_t2_dn5 = assign84680_body9_e129614_d_n5;
            locals.var_t2_dn6 = assign84680_body9_e129614_d_n6;
            locals.var_t2_dn7 = assign84680_body9_e129614_d_n7;
            locals.var_t2_dn8 = assign84680_body9_e129614_d_n8;
            locals.var_t2_dn9 = assign84680_body9_e129614_d_n9;
            locals.var_t2_dn10 = assign84680_body9_e129614_d_n10;
            locals.var_t2_dn13 = assign84680_body9_e129614_d_n13;
            let (assign84680_body10_e129627, assign84680_body10_e129627_d_n0, assign84680_body10_e129627_d_n2, assign84680_body10_e129627_d_n4, assign84680_body10_e129627_d_n5, assign84680_body10_e129627_d_n6, assign84680_body10_e129627_d_n7, assign84680_body10_e129627_d_n8, assign84680_body10_e129627_d_n9, assign84680_body10_e129627_d_n10, assign84680_body10_e129627_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1964 != 0.0)) && (locals.var_guard1969 != 0.0)) {
        let assign84680_body10_e129622: f64 = (1.0 + locals.var_t2);
        let assign84680_body10_e129623: f64 = (assign84680_body10_e129622).ln();
        let assign84680_body10_e129625: f64 = (assign84680_body10_e129623 / locals.var_c_sb);
        (assign84680_body10_e129625, ((((locals.var_t2_dn0 / assign84680_body10_e129622) * locals.var_c_sb) - (assign84680_body10_e129623 * locals.var_c_sb_dn0)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn2 / assign84680_body10_e129622) * locals.var_c_sb) - (assign84680_body10_e129623 * locals.var_c_sb_dn2)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn4 / assign84680_body10_e129622) * locals.var_c_sb) - (assign84680_body10_e129623 * locals.var_c_sb_dn4)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn5 / assign84680_body10_e129622) * locals.var_c_sb) - (assign84680_body10_e129623 * locals.var_c_sb_dn5)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn6 / assign84680_body10_e129622) * locals.var_c_sb) - (assign84680_body10_e129623 * locals.var_c_sb_dn6)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn7 / assign84680_body10_e129622) * locals.var_c_sb) - (assign84680_body10_e129623 * locals.var_c_sb_dn7)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn8 / assign84680_body10_e129622) * locals.var_c_sb) - (assign84680_body10_e129623 * locals.var_c_sb_dn8)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn9 / assign84680_body10_e129622) * locals.var_c_sb) - (assign84680_body10_e129623 * locals.var_c_sb_dn9)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn10 / assign84680_body10_e129622) * locals.var_c_sb) - (assign84680_body10_e129623 * locals.var_c_sb_dn10)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn13 / assign84680_body10_e129622) * locals.var_c_sb) - (assign84680_body10_e129623 * locals.var_c_sb_dn13)) / (locals.var_c_sb * locals.var_c_sb)),)
    } else {
        (locals.var_phi_b, locals.var_phi_b_dn0, locals.var_phi_b_dn2, locals.var_phi_b_dn4, locals.var_phi_b_dn5, locals.var_phi_b_dn6, locals.var_phi_b_dn7, locals.var_phi_b_dn8, locals.var_phi_b_dn9, locals.var_phi_b_dn10, locals.var_phi_b_dn13,)
    }
};
            locals.var_phi_b = assign84680_body10_e129627;
            locals.var_phi_b_dn0 = assign84680_body10_e129627_d_n0;
            locals.var_phi_b_dn2 = assign84680_body10_e129627_d_n2;
            locals.var_phi_b_dn4 = assign84680_body10_e129627_d_n4;
            locals.var_phi_b_dn5 = assign84680_body10_e129627_d_n5;
            locals.var_phi_b_dn6 = assign84680_body10_e129627_d_n6;
            locals.var_phi_b_dn7 = assign84680_body10_e129627_d_n7;
            locals.var_phi_b_dn8 = assign84680_body10_e129627_d_n8;
            locals.var_phi_b_dn9 = assign84680_body10_e129627_d_n9;
            locals.var_phi_b_dn10 = assign84680_body10_e129627_d_n10;
            locals.var_phi_b_dn13 = assign84680_body10_e129627_d_n13;
            let (assign84680_body11_e129639, assign84680_body11_e129639_d_n0, assign84680_body11_e129639_d_n2, assign84680_body11_e129639_d_n4, assign84680_body11_e129639_d_n5, assign84680_body11_e129639_d_n6, assign84680_body11_e129639_d_n7, assign84680_body11_e129639_d_n8, assign84680_body11_e129639_d_n9, assign84680_body11_e129639_d_n10, assign84680_body11_e129639_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1964 != 0.0)) && (locals.var_guard1969 != 0.0)) {
        let assign84680_body11_e129636: f64 = (1.0 + locals.var_t2);
        let assign84680_body11_e129637: f64 = (locals.var_t1 / assign84680_body11_e129636);
        (assign84680_body11_e129637, (((locals.var_t1_dn0 * assign84680_body11_e129636) - (locals.var_t1 * locals.var_t2_dn0)) / (assign84680_body11_e129636 * assign84680_body11_e129636)), (((locals.var_t1_dn2 * assign84680_body11_e129636) - (locals.var_t1 * locals.var_t2_dn2)) / (assign84680_body11_e129636 * assign84680_body11_e129636)), (((locals.var_t1_dn4 * assign84680_body11_e129636) - (locals.var_t1 * locals.var_t2_dn4)) / (assign84680_body11_e129636 * assign84680_body11_e129636)), (((locals.var_t1_dn5 * assign84680_body11_e129636) - (locals.var_t1 * locals.var_t2_dn5)) / (assign84680_body11_e129636 * assign84680_body11_e129636)), (((locals.var_t1_dn6 * assign84680_body11_e129636) - (locals.var_t1 * locals.var_t2_dn6)) / (assign84680_body11_e129636 * assign84680_body11_e129636)), (((locals.var_t1_dn7 * assign84680_body11_e129636) - (locals.var_t1 * locals.var_t2_dn7)) / (assign84680_body11_e129636 * assign84680_body11_e129636)), (((locals.var_t1_dn8 * assign84680_body11_e129636) - (locals.var_t1 * locals.var_t2_dn8)) / (assign84680_body11_e129636 * assign84680_body11_e129636)), (((locals.var_t1_dn9 * assign84680_body11_e129636) - (locals.var_t1 * locals.var_t2_dn9)) / (assign84680_body11_e129636 * assign84680_body11_e129636)), (((locals.var_t1_dn10 * assign84680_body11_e129636) - (locals.var_t1 * locals.var_t2_dn10)) / (assign84680_body11_e129636 * assign84680_body11_e129636)), (((locals.var_t1_dn13 * assign84680_body11_e129636) - (locals.var_t1 * locals.var_t2_dn13)) / (assign84680_body11_e129636 * assign84680_body11_e129636)),)
    } else {
        (locals.var_phi_b_dpss, locals.var_phi_b_dpss_dn0, locals.var_phi_b_dpss_dn2, locals.var_phi_b_dpss_dn4, locals.var_phi_b_dpss_dn5, locals.var_phi_b_dpss_dn6, locals.var_phi_b_dpss_dn7, locals.var_phi_b_dpss_dn8, locals.var_phi_b_dpss_dn9, locals.var_phi_b_dpss_dn10, locals.var_phi_b_dpss_dn13,)
    }
};
            locals.var_phi_b_dpss = assign84680_body11_e129639;
            locals.var_phi_b_dpss_dn0 = assign84680_body11_e129639_d_n0;
            locals.var_phi_b_dpss_dn2 = assign84680_body11_e129639_d_n2;
            locals.var_phi_b_dpss_dn4 = assign84680_body11_e129639_d_n4;
            locals.var_phi_b_dpss_dn5 = assign84680_body11_e129639_d_n5;
            locals.var_phi_b_dpss_dn6 = assign84680_body11_e129639_d_n6;
            locals.var_phi_b_dpss_dn7 = assign84680_body11_e129639_d_n7;
            locals.var_phi_b_dpss_dn8 = assign84680_body11_e129639_d_n8;
            locals.var_phi_b_dpss_dn9 = assign84680_body11_e129639_d_n9;
            locals.var_phi_b_dpss_dn10 = assign84680_body11_e129639_d_n10;
            locals.var_phi_b_dpss_dn13 = assign84680_body11_e129639_d_n13;
            let (assign84680_body12_e129650, assign84680_body12_e129650_d_n0, assign84680_body12_e129650_d_n2, assign84680_body12_e129650_d_n4, assign84680_body12_e129650_d_n5, assign84680_body12_e129650_d_n6, assign84680_body12_e129650_d_n7, assign84680_body12_e129650_d_n8, assign84680_body12_e129650_d_n9, assign84680_body12_e129650_d_n10, assign84680_body12_e129650_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1964 != 0.0)) && (locals.var_guard1969 == 0.0)) {
        let assign84680_body12_e129648: f64 = (locals.var_ps0ld_vxb - locals.var_dphi_sb);
        (assign84680_body12_e129648, (locals.var_ps0ld_vxb_dn0 - locals.var_dphi_sb_dn0), (locals.var_ps0ld_vxb_dn2 - locals.var_dphi_sb_dn2), (locals.var_ps0ld_vxb_dn4 - locals.var_dphi_sb_dn4), (locals.var_ps0ld_vxb_dn5 - locals.var_dphi_sb_dn5), (locals.var_ps0ld_vxb_dn6 - locals.var_dphi_sb_dn6), (locals.var_ps0ld_vxb_dn7 - locals.var_dphi_sb_dn7), (locals.var_ps0ld_vxb_dn8 - locals.var_dphi_sb_dn8), (locals.var_ps0ld_vxb_dn9 - locals.var_dphi_sb_dn9), (locals.var_ps0ld_vxb_dn10 - locals.var_dphi_sb_dn10), (locals.var_ps0ld_vxb_dn13 - locals.var_dphi_sb_dn13),)
    } else {
        (locals.var_phi_b, locals.var_phi_b_dn0, locals.var_phi_b_dn2, locals.var_phi_b_dn4, locals.var_phi_b_dn5, locals.var_phi_b_dn6, locals.var_phi_b_dn7, locals.var_phi_b_dn8, locals.var_phi_b_dn9, locals.var_phi_b_dn10, locals.var_phi_b_dn13,)
    }
};
            locals.var_phi_b = assign84680_body12_e129650;
            locals.var_phi_b_dn0 = assign84680_body12_e129650_d_n0;
            locals.var_phi_b_dn2 = assign84680_body12_e129650_d_n2;
            locals.var_phi_b_dn4 = assign84680_body12_e129650_d_n4;
            locals.var_phi_b_dn5 = assign84680_body12_e129650_d_n5;
            locals.var_phi_b_dn6 = assign84680_body12_e129650_d_n6;
            locals.var_phi_b_dn7 = assign84680_body12_e129650_d_n7;
            locals.var_phi_b_dn8 = assign84680_body12_e129650_d_n8;
            locals.var_phi_b_dn9 = assign84680_body12_e129650_d_n9;
            locals.var_phi_b_dn10 = assign84680_body12_e129650_d_n10;
            locals.var_phi_b_dn13 = assign84680_body12_e129650_d_n13;
            let (assign84680_body13_e129659, assign84680_body13_e129659_d_n0, assign84680_body13_e129659_d_n2, assign84680_body13_e129659_d_n4, assign84680_body13_e129659_d_n5, assign84680_body13_e129659_d_n6, assign84680_body13_e129659_d_n7, assign84680_body13_e129659_d_n8, assign84680_body13_e129659_d_n9, assign84680_body13_e129659_d_n10, assign84680_body13_e129659_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1964 != 0.0)) && (locals.var_guard1969 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_phi_b_dpss, locals.var_phi_b_dpss_dn0, locals.var_phi_b_dpss_dn2, locals.var_phi_b_dpss_dn4, locals.var_phi_b_dpss_dn5, locals.var_phi_b_dpss_dn6, locals.var_phi_b_dpss_dn7, locals.var_phi_b_dpss_dn8, locals.var_phi_b_dpss_dn9, locals.var_phi_b_dpss_dn10, locals.var_phi_b_dpss_dn13,)
    }
};
            locals.var_phi_b_dpss = assign84680_body13_e129659;
            locals.var_phi_b_dpss_dn0 = assign84680_body13_e129659_d_n0;
            locals.var_phi_b_dpss_dn2 = assign84680_body13_e129659_d_n2;
            locals.var_phi_b_dpss_dn4 = assign84680_body13_e129659_d_n4;
            locals.var_phi_b_dpss_dn5 = assign84680_body13_e129659_d_n5;
            locals.var_phi_b_dpss_dn6 = assign84680_body13_e129659_d_n6;
            locals.var_phi_b_dpss_dn7 = assign84680_body13_e129659_d_n7;
            locals.var_phi_b_dpss_dn8 = assign84680_body13_e129659_d_n8;
            locals.var_phi_b_dpss_dn9 = assign84680_body13_e129659_d_n9;
            locals.var_phi_b_dpss_dn10 = assign84680_body13_e129659_d_n10;
            locals.var_phi_b_dpss_dn13 = assign84680_body13_e129659_d_n13;
            let (assign84680_body14_e129667, assign84680_body14_e129667_d_n0, assign84680_body14_e129667_d_n2, assign84680_body14_e129667_d_n4, assign84680_body14_e129667_d_n5, assign84680_body14_e129667_d_n6, assign84680_body14_e129667_d_n7, assign84680_body14_e129667_d_n8, assign84680_body14_e129667_d_n9, assign84680_body14_e129667_d_n10, assign84680_body14_e129667_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1964 != 0.0)) {
        let assign84680_body14_e129665: f64 = (locals.var_beta * locals.var_phi_b);
        (assign84680_body14_e129665, ((locals.var_beta_dn0 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn0)), ((locals.var_beta_dn2 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn2)), ((locals.var_beta_dn4 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn4)), ((locals.var_beta_dn5 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn5)), ((locals.var_beta_dn6 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn6)), ((locals.var_beta_dn7 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn7)), ((locals.var_beta_dn8 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn8)), ((locals.var_beta_dn9 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn9)), ((locals.var_beta_dn10 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn10)), ((locals.var_beta_dn13 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn13)),)
    } else {
        (locals.var_chib, locals.var_chib_dn0, locals.var_chib_dn2, locals.var_chib_dn4, locals.var_chib_dn5, locals.var_chib_dn6, locals.var_chib_dn7, locals.var_chib_dn8, locals.var_chib_dn9, locals.var_chib_dn10, locals.var_chib_dn13,)
    }
};
            locals.var_chib = assign84680_body14_e129667;
            locals.var_chib_dn0 = assign84680_body14_e129667_d_n0;
            locals.var_chib_dn2 = assign84680_body14_e129667_d_n2;
            locals.var_chib_dn4 = assign84680_body14_e129667_d_n4;
            locals.var_chib_dn5 = assign84680_body14_e129667_d_n5;
            locals.var_chib_dn6 = assign84680_body14_e129667_d_n6;
            locals.var_chib_dn7 = assign84680_body14_e129667_d_n7;
            locals.var_chib_dn8 = assign84680_body14_e129667_d_n8;
            locals.var_chib_dn9 = assign84680_body14_e129667_d_n9;
            locals.var_chib_dn10 = assign84680_body14_e129667_d_n10;
            locals.var_chib_dn13 = assign84680_body14_e129667_d_n13;
            let assign84680_body15_e129669: f64 = (locals.var_chi).abs();
            let assign84680_body15_e129671: f64 = if assign84680_body15_e129669 < 1e-6 { 1.0 } else { 0.0 };
            locals.var_guard1970 = assign84680_body15_e129671;
            let (assign84680_body17_e129717, assign84680_body17_e129717_d_n0, assign84680_body17_e129717_d_n2, assign84680_body17_e129717_d_n4, assign84680_body17_e129717_d_n5, assign84680_body17_e129717_d_n6, assign84680_body17_e129717_d_n7, assign84680_body17_e129717_d_n8, assign84680_body17_e129717_d_n9, assign84680_body17_e129717_d_n10, assign84680_body17_e129717_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1964 != 0.0)) && (locals.var_guard1970 != 0.0)) {
        let assign84680_body17_e129695: f64 = (locals.var_chi * locals.var_chi);
        let assign84680_body17_e129697: f64 = (assign84680_body17_e129695 / 2.0);
        let assign84680_body17_e129701: f64 = (locals.var_chi / 3.0);
        let assign84680_body17_e129705: f64 = (locals.var_chi / 4.0);
        let assign84680_body17_e129709: f64 = (locals.var_chi / 5.0);
        let assign84680_body17_e129710: f64 = (1.0 - assign84680_body17_e129709);
        let assign84680_body17_e129711: f64 = (assign84680_body17_e129705 * assign84680_body17_e129710);
        let assign84680_body17_e129712: f64 = (1.0 - assign84680_body17_e129711);
        let assign84680_body17_e129713: f64 = (assign84680_body17_e129701 * assign84680_body17_e129712);
        let assign84680_body17_e129714: f64 = (1.0 - assign84680_body17_e129713);
        let assign84680_body17_e129715: f64 = (assign84680_body17_e129697 * assign84680_body17_e129714);
        (assign84680_body17_e129715, (((((locals.var_chi_dn0 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn0)) / 2.0) * assign84680_body17_e129714) + (assign84680_body17_e129697 * (-(((locals.var_chi_dn0 / 3.0) * assign84680_body17_e129712) + (assign84680_body17_e129701 * (-(((locals.var_chi_dn0 / 4.0) * assign84680_body17_e129710) + (assign84680_body17_e129705 * (-(locals.var_chi_dn0 / 5.0)))))))))), (((((locals.var_chi_dn2 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn2)) / 2.0) * assign84680_body17_e129714) + (assign84680_body17_e129697 * (-(((locals.var_chi_dn2 / 3.0) * assign84680_body17_e129712) + (assign84680_body17_e129701 * (-(((locals.var_chi_dn2 / 4.0) * assign84680_body17_e129710) + (assign84680_body17_e129705 * (-(locals.var_chi_dn2 / 5.0)))))))))), (((((locals.var_chi_dn4 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn4)) / 2.0) * assign84680_body17_e129714) + (assign84680_body17_e129697 * (-(((locals.var_chi_dn4 / 3.0) * assign84680_body17_e129712) + (assign84680_body17_e129701 * (-(((locals.var_chi_dn4 / 4.0) * assign84680_body17_e129710) + (assign84680_body17_e129705 * (-(locals.var_chi_dn4 / 5.0)))))))))), (((((locals.var_chi_dn5 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn5)) / 2.0) * assign84680_body17_e129714) + (assign84680_body17_e129697 * (-(((locals.var_chi_dn5 / 3.0) * assign84680_body17_e129712) + (assign84680_body17_e129701 * (-(((locals.var_chi_dn5 / 4.0) * assign84680_body17_e129710) + (assign84680_body17_e129705 * (-(locals.var_chi_dn5 / 5.0)))))))))), (((((locals.var_chi_dn6 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn6)) / 2.0) * assign84680_body17_e129714) + (assign84680_body17_e129697 * (-(((locals.var_chi_dn6 / 3.0) * assign84680_body17_e129712) + (assign84680_body17_e129701 * (-(((locals.var_chi_dn6 / 4.0) * assign84680_body17_e129710) + (assign84680_body17_e129705 * (-(locals.var_chi_dn6 / 5.0)))))))))), (((((locals.var_chi_dn7 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn7)) / 2.0) * assign84680_body17_e129714) + (assign84680_body17_e129697 * (-(((locals.var_chi_dn7 / 3.0) * assign84680_body17_e129712) + (assign84680_body17_e129701 * (-(((locals.var_chi_dn7 / 4.0) * assign84680_body17_e129710) + (assign84680_body17_e129705 * (-(locals.var_chi_dn7 / 5.0)))))))))), (((((locals.var_chi_dn8 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn8)) / 2.0) * assign84680_body17_e129714) + (assign84680_body17_e129697 * (-(((locals.var_chi_dn8 / 3.0) * assign84680_body17_e129712) + (assign84680_body17_e129701 * (-(((locals.var_chi_dn8 / 4.0) * assign84680_body17_e129710) + (assign84680_body17_e129705 * (-(locals.var_chi_dn8 / 5.0)))))))))), (((((locals.var_chi_dn9 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn9)) / 2.0) * assign84680_body17_e129714) + (assign84680_body17_e129697 * (-(((locals.var_chi_dn9 / 3.0) * assign84680_body17_e129712) + (assign84680_body17_e129701 * (-(((locals.var_chi_dn9 / 4.0) * assign84680_body17_e129710) + (assign84680_body17_e129705 * (-(locals.var_chi_dn9 / 5.0)))))))))), (((((locals.var_chi_dn10 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn10)) / 2.0) * assign84680_body17_e129714) + (assign84680_body17_e129697 * (-(((locals.var_chi_dn10 / 3.0) * assign84680_body17_e129712) + (assign84680_body17_e129701 * (-(((locals.var_chi_dn10 / 4.0) * assign84680_body17_e129710) + (assign84680_body17_e129705 * (-(locals.var_chi_dn10 / 5.0)))))))))), (((((locals.var_chi_dn13 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn13)) / 2.0) * assign84680_body17_e129714) + (assign84680_body17_e129697 * (-(((locals.var_chi_dn13 / 3.0) * assign84680_body17_e129712) + (assign84680_body17_e129701 * (-(((locals.var_chi_dn13 / 4.0) * assign84680_body17_e129710) + (assign84680_body17_e129705 * (-(locals.var_chi_dn13 / 5.0)))))))))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
            locals.var_t0 = assign84680_body17_e129717;
            locals.var_t0_dn0 = assign84680_body17_e129717_d_n0;
            locals.var_t0_dn2 = assign84680_body17_e129717_d_n2;
            locals.var_t0_dn4 = assign84680_body17_e129717_d_n4;
            locals.var_t0_dn5 = assign84680_body17_e129717_d_n5;
            locals.var_t0_dn6 = assign84680_body17_e129717_d_n6;
            locals.var_t0_dn7 = assign84680_body17_e129717_d_n7;
            locals.var_t0_dn8 = assign84680_body17_e129717_d_n8;
            locals.var_t0_dn9 = assign84680_body17_e129717_d_n9;
            locals.var_t0_dn10 = assign84680_body17_e129717_d_n10;
            locals.var_t0_dn13 = assign84680_body17_e129717_d_n13;
            let (assign84680_body18_e129743, assign84680_body18_e129743_d_n0, assign84680_body18_e129743_d_n2, assign84680_body18_e129743_d_n4, assign84680_body18_e129743_d_n5, assign84680_body18_e129743_d_n6, assign84680_body18_e129743_d_n7, assign84680_body18_e129743_d_n8, assign84680_body18_e129743_d_n9, assign84680_body18_e129743_d_n10, assign84680_body18_e129743_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1964 != 0.0)) && (locals.var_guard1970 != 0.0)) {
        let assign84680_body18_e129727: f64 = (locals.var_chi / 2.0);
        let assign84680_body18_e129731: f64 = (locals.var_chi / 3.0);
        let assign84680_body18_e129735: f64 = (locals.var_chi / 4.0);
        let assign84680_body18_e129736: f64 = (1.0 - assign84680_body18_e129735);
        let assign84680_body18_e129737: f64 = (assign84680_body18_e129731 * assign84680_body18_e129736);
        let assign84680_body18_e129738: f64 = (1.0 - assign84680_body18_e129737);
        let assign84680_body18_e129739: f64 = (assign84680_body18_e129727 * assign84680_body18_e129738);
        let assign84680_body18_e129740: f64 = (1.0 - assign84680_body18_e129739);
        let assign84680_body18_e129741: f64 = (locals.var_chi * assign84680_body18_e129740);
        (assign84680_body18_e129741, ((locals.var_chi_dn0 * assign84680_body18_e129740) + (locals.var_chi * (-(((locals.var_chi_dn0 / 2.0) * assign84680_body18_e129738) + (assign84680_body18_e129727 * (-(((locals.var_chi_dn0 / 3.0) * assign84680_body18_e129736) + (assign84680_body18_e129731 * (-(locals.var_chi_dn0 / 4.0)))))))))), ((locals.var_chi_dn2 * assign84680_body18_e129740) + (locals.var_chi * (-(((locals.var_chi_dn2 / 2.0) * assign84680_body18_e129738) + (assign84680_body18_e129727 * (-(((locals.var_chi_dn2 / 3.0) * assign84680_body18_e129736) + (assign84680_body18_e129731 * (-(locals.var_chi_dn2 / 4.0)))))))))), ((locals.var_chi_dn4 * assign84680_body18_e129740) + (locals.var_chi * (-(((locals.var_chi_dn4 / 2.0) * assign84680_body18_e129738) + (assign84680_body18_e129727 * (-(((locals.var_chi_dn4 / 3.0) * assign84680_body18_e129736) + (assign84680_body18_e129731 * (-(locals.var_chi_dn4 / 4.0)))))))))), ((locals.var_chi_dn5 * assign84680_body18_e129740) + (locals.var_chi * (-(((locals.var_chi_dn5 / 2.0) * assign84680_body18_e129738) + (assign84680_body18_e129727 * (-(((locals.var_chi_dn5 / 3.0) * assign84680_body18_e129736) + (assign84680_body18_e129731 * (-(locals.var_chi_dn5 / 4.0)))))))))), ((locals.var_chi_dn6 * assign84680_body18_e129740) + (locals.var_chi * (-(((locals.var_chi_dn6 / 2.0) * assign84680_body18_e129738) + (assign84680_body18_e129727 * (-(((locals.var_chi_dn6 / 3.0) * assign84680_body18_e129736) + (assign84680_body18_e129731 * (-(locals.var_chi_dn6 / 4.0)))))))))), ((locals.var_chi_dn7 * assign84680_body18_e129740) + (locals.var_chi * (-(((locals.var_chi_dn7 / 2.0) * assign84680_body18_e129738) + (assign84680_body18_e129727 * (-(((locals.var_chi_dn7 / 3.0) * assign84680_body18_e129736) + (assign84680_body18_e129731 * (-(locals.var_chi_dn7 / 4.0)))))))))), ((locals.var_chi_dn8 * assign84680_body18_e129740) + (locals.var_chi * (-(((locals.var_chi_dn8 / 2.0) * assign84680_body18_e129738) + (assign84680_body18_e129727 * (-(((locals.var_chi_dn8 / 3.0) * assign84680_body18_e129736) + (assign84680_body18_e129731 * (-(locals.var_chi_dn8 / 4.0)))))))))), ((locals.var_chi_dn9 * assign84680_body18_e129740) + (locals.var_chi * (-(((locals.var_chi_dn9 / 2.0) * assign84680_body18_e129738) + (assign84680_body18_e129727 * (-(((locals.var_chi_dn9 / 3.0) * assign84680_body18_e129736) + (assign84680_body18_e129731 * (-(locals.var_chi_dn9 / 4.0)))))))))), ((locals.var_chi_dn10 * assign84680_body18_e129740) + (locals.var_chi * (-(((locals.var_chi_dn10 / 2.0) * assign84680_body18_e129738) + (assign84680_body18_e129727 * (-(((locals.var_chi_dn10 / 3.0) * assign84680_body18_e129736) + (assign84680_body18_e129731 * (-(locals.var_chi_dn10 / 4.0)))))))))), ((locals.var_chi_dn13 * assign84680_body18_e129740) + (locals.var_chi * (-(((locals.var_chi_dn13 / 2.0) * assign84680_body18_e129738) + (assign84680_body18_e129727 * (-(((locals.var_chi_dn13 / 3.0) * assign84680_body18_e129736) + (assign84680_body18_e129731 * (-(locals.var_chi_dn13 / 4.0)))))))))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
            locals.var_t1 = assign84680_body18_e129743;
            locals.var_t1_dn0 = assign84680_body18_e129743_d_n0;
            locals.var_t1_dn2 = assign84680_body18_e129743_d_n2;
            locals.var_t1_dn4 = assign84680_body18_e129743_d_n4;
            locals.var_t1_dn5 = assign84680_body18_e129743_d_n5;
            locals.var_t1_dn6 = assign84680_body18_e129743_d_n6;
            locals.var_t1_dn7 = assign84680_body18_e129743_d_n7;
            locals.var_t1_dn8 = assign84680_body18_e129743_d_n8;
            locals.var_t1_dn9 = assign84680_body18_e129743_d_n9;
            locals.var_t1_dn10 = assign84680_body18_e129743_d_n10;
            locals.var_t1_dn13 = assign84680_body18_e129743_d_n13;
            let (assign84680_body19_e129773, assign84680_body19_e129773_d_n0, assign84680_body19_e129773_d_n2, assign84680_body19_e129773_d_n4, assign84680_body19_e129773_d_n5, assign84680_body19_e129773_d_n6, assign84680_body19_e129773_d_n7, assign84680_body19_e129773_d_n8, assign84680_body19_e129773_d_n9, assign84680_body19_e129773_d_n10, assign84680_body19_e129773_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1964 != 0.0)) && (locals.var_guard1970 != 0.0)) {
        let assign84680_body19_e129751: f64 = (locals.var_chib * locals.var_chib);
        let assign84680_body19_e129753: f64 = (assign84680_body19_e129751 / 2.0);
        let assign84680_body19_e129757: f64 = (locals.var_chib / 3.0);
        let assign84680_body19_e129761: f64 = (locals.var_chib / 4.0);
        let assign84680_body19_e129765: f64 = (locals.var_chib / 5.0);
        let assign84680_body19_e129766: f64 = (1.0 - assign84680_body19_e129765);
        let assign84680_body19_e129767: f64 = (assign84680_body19_e129761 * assign84680_body19_e129766);
        let assign84680_body19_e129768: f64 = (1.0 - assign84680_body19_e129767);
        let assign84680_body19_e129769: f64 = (assign84680_body19_e129757 * assign84680_body19_e129768);
        let assign84680_body19_e129770: f64 = (1.0 - assign84680_body19_e129769);
        let assign84680_body19_e129771: f64 = (assign84680_body19_e129753 * assign84680_body19_e129770);
        (assign84680_body19_e129771, (((((locals.var_chib_dn0 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn0)) / 2.0) * assign84680_body19_e129770) + (assign84680_body19_e129753 * (-(((locals.var_chib_dn0 / 3.0) * assign84680_body19_e129768) + (assign84680_body19_e129757 * (-(((locals.var_chib_dn0 / 4.0) * assign84680_body19_e129766) + (assign84680_body19_e129761 * (-(locals.var_chib_dn0 / 5.0)))))))))), (((((locals.var_chib_dn2 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn2)) / 2.0) * assign84680_body19_e129770) + (assign84680_body19_e129753 * (-(((locals.var_chib_dn2 / 3.0) * assign84680_body19_e129768) + (assign84680_body19_e129757 * (-(((locals.var_chib_dn2 / 4.0) * assign84680_body19_e129766) + (assign84680_body19_e129761 * (-(locals.var_chib_dn2 / 5.0)))))))))), (((((locals.var_chib_dn4 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn4)) / 2.0) * assign84680_body19_e129770) + (assign84680_body19_e129753 * (-(((locals.var_chib_dn4 / 3.0) * assign84680_body19_e129768) + (assign84680_body19_e129757 * (-(((locals.var_chib_dn4 / 4.0) * assign84680_body19_e129766) + (assign84680_body19_e129761 * (-(locals.var_chib_dn4 / 5.0)))))))))), (((((locals.var_chib_dn5 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn5)) / 2.0) * assign84680_body19_e129770) + (assign84680_body19_e129753 * (-(((locals.var_chib_dn5 / 3.0) * assign84680_body19_e129768) + (assign84680_body19_e129757 * (-(((locals.var_chib_dn5 / 4.0) * assign84680_body19_e129766) + (assign84680_body19_e129761 * (-(locals.var_chib_dn5 / 5.0)))))))))), (((((locals.var_chib_dn6 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn6)) / 2.0) * assign84680_body19_e129770) + (assign84680_body19_e129753 * (-(((locals.var_chib_dn6 / 3.0) * assign84680_body19_e129768) + (assign84680_body19_e129757 * (-(((locals.var_chib_dn6 / 4.0) * assign84680_body19_e129766) + (assign84680_body19_e129761 * (-(locals.var_chib_dn6 / 5.0)))))))))), (((((locals.var_chib_dn7 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn7)) / 2.0) * assign84680_body19_e129770) + (assign84680_body19_e129753 * (-(((locals.var_chib_dn7 / 3.0) * assign84680_body19_e129768) + (assign84680_body19_e129757 * (-(((locals.var_chib_dn7 / 4.0) * assign84680_body19_e129766) + (assign84680_body19_e129761 * (-(locals.var_chib_dn7 / 5.0)))))))))), (((((locals.var_chib_dn8 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn8)) / 2.0) * assign84680_body19_e129770) + (assign84680_body19_e129753 * (-(((locals.var_chib_dn8 / 3.0) * assign84680_body19_e129768) + (assign84680_body19_e129757 * (-(((locals.var_chib_dn8 / 4.0) * assign84680_body19_e129766) + (assign84680_body19_e129761 * (-(locals.var_chib_dn8 / 5.0)))))))))), (((((locals.var_chib_dn9 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn9)) / 2.0) * assign84680_body19_e129770) + (assign84680_body19_e129753 * (-(((locals.var_chib_dn9 / 3.0) * assign84680_body19_e129768) + (assign84680_body19_e129757 * (-(((locals.var_chib_dn9 / 4.0) * assign84680_body19_e129766) + (assign84680_body19_e129761 * (-(locals.var_chib_dn9 / 5.0)))))))))), (((((locals.var_chib_dn10 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn10)) / 2.0) * assign84680_body19_e129770) + (assign84680_body19_e129753 * (-(((locals.var_chib_dn10 / 3.0) * assign84680_body19_e129768) + (assign84680_body19_e129757 * (-(((locals.var_chib_dn10 / 4.0) * assign84680_body19_e129766) + (assign84680_body19_e129761 * (-(locals.var_chib_dn10 / 5.0)))))))))), (((((locals.var_chib_dn13 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn13)) / 2.0) * assign84680_body19_e129770) + (assign84680_body19_e129753 * (-(((locals.var_chib_dn13 / 3.0) * assign84680_body19_e129768) + (assign84680_body19_e129757 * (-(((locals.var_chib_dn13 / 4.0) * assign84680_body19_e129766) + (assign84680_body19_e129761 * (-(locals.var_chib_dn13 / 5.0)))))))))),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
            locals.var_t2 = assign84680_body19_e129773;
            locals.var_t2_dn0 = assign84680_body19_e129773_d_n0;
            locals.var_t2_dn2 = assign84680_body19_e129773_d_n2;
            locals.var_t2_dn4 = assign84680_body19_e129773_d_n4;
            locals.var_t2_dn5 = assign84680_body19_e129773_d_n5;
            locals.var_t2_dn6 = assign84680_body19_e129773_d_n6;
            locals.var_t2_dn7 = assign84680_body19_e129773_d_n7;
            locals.var_t2_dn8 = assign84680_body19_e129773_d_n8;
            locals.var_t2_dn9 = assign84680_body19_e129773_d_n9;
            locals.var_t2_dn10 = assign84680_body19_e129773_d_n10;
            locals.var_t2_dn13 = assign84680_body19_e129773_d_n13;
            let (assign84680_body20_e129799, assign84680_body20_e129799_d_n0, assign84680_body20_e129799_d_n2, assign84680_body20_e129799_d_n4, assign84680_body20_e129799_d_n5, assign84680_body20_e129799_d_n6, assign84680_body20_e129799_d_n7, assign84680_body20_e129799_d_n8, assign84680_body20_e129799_d_n9, assign84680_body20_e129799_d_n10, assign84680_body20_e129799_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1964 != 0.0)) && (locals.var_guard1970 != 0.0)) {
        let assign84680_body20_e129783: f64 = (locals.var_chib / 2.0);
        let assign84680_body20_e129787: f64 = (locals.var_chib / 3.0);
        let assign84680_body20_e129791: f64 = (locals.var_chib / 4.0);
        let assign84680_body20_e129792: f64 = (1.0 - assign84680_body20_e129791);
        let assign84680_body20_e129793: f64 = (assign84680_body20_e129787 * assign84680_body20_e129792);
        let assign84680_body20_e129794: f64 = (1.0 - assign84680_body20_e129793);
        let assign84680_body20_e129795: f64 = (assign84680_body20_e129783 * assign84680_body20_e129794);
        let assign84680_body20_e129796: f64 = (1.0 - assign84680_body20_e129795);
        let assign84680_body20_e129797: f64 = (locals.var_chib * assign84680_body20_e129796);
        (assign84680_body20_e129797, ((locals.var_chib_dn0 * assign84680_body20_e129796) + (locals.var_chib * (-(((locals.var_chib_dn0 / 2.0) * assign84680_body20_e129794) + (assign84680_body20_e129783 * (-(((locals.var_chib_dn0 / 3.0) * assign84680_body20_e129792) + (assign84680_body20_e129787 * (-(locals.var_chib_dn0 / 4.0)))))))))), ((locals.var_chib_dn2 * assign84680_body20_e129796) + (locals.var_chib * (-(((locals.var_chib_dn2 / 2.0) * assign84680_body20_e129794) + (assign84680_body20_e129783 * (-(((locals.var_chib_dn2 / 3.0) * assign84680_body20_e129792) + (assign84680_body20_e129787 * (-(locals.var_chib_dn2 / 4.0)))))))))), ((locals.var_chib_dn4 * assign84680_body20_e129796) + (locals.var_chib * (-(((locals.var_chib_dn4 / 2.0) * assign84680_body20_e129794) + (assign84680_body20_e129783 * (-(((locals.var_chib_dn4 / 3.0) * assign84680_body20_e129792) + (assign84680_body20_e129787 * (-(locals.var_chib_dn4 / 4.0)))))))))), ((locals.var_chib_dn5 * assign84680_body20_e129796) + (locals.var_chib * (-(((locals.var_chib_dn5 / 2.0) * assign84680_body20_e129794) + (assign84680_body20_e129783 * (-(((locals.var_chib_dn5 / 3.0) * assign84680_body20_e129792) + (assign84680_body20_e129787 * (-(locals.var_chib_dn5 / 4.0)))))))))), ((locals.var_chib_dn6 * assign84680_body20_e129796) + (locals.var_chib * (-(((locals.var_chib_dn6 / 2.0) * assign84680_body20_e129794) + (assign84680_body20_e129783 * (-(((locals.var_chib_dn6 / 3.0) * assign84680_body20_e129792) + (assign84680_body20_e129787 * (-(locals.var_chib_dn6 / 4.0)))))))))), ((locals.var_chib_dn7 * assign84680_body20_e129796) + (locals.var_chib * (-(((locals.var_chib_dn7 / 2.0) * assign84680_body20_e129794) + (assign84680_body20_e129783 * (-(((locals.var_chib_dn7 / 3.0) * assign84680_body20_e129792) + (assign84680_body20_e129787 * (-(locals.var_chib_dn7 / 4.0)))))))))), ((locals.var_chib_dn8 * assign84680_body20_e129796) + (locals.var_chib * (-(((locals.var_chib_dn8 / 2.0) * assign84680_body20_e129794) + (assign84680_body20_e129783 * (-(((locals.var_chib_dn8 / 3.0) * assign84680_body20_e129792) + (assign84680_body20_e129787 * (-(locals.var_chib_dn8 / 4.0)))))))))), ((locals.var_chib_dn9 * assign84680_body20_e129796) + (locals.var_chib * (-(((locals.var_chib_dn9 / 2.0) * assign84680_body20_e129794) + (assign84680_body20_e129783 * (-(((locals.var_chib_dn9 / 3.0) * assign84680_body20_e129792) + (assign84680_body20_e129787 * (-(locals.var_chib_dn9 / 4.0)))))))))), ((locals.var_chib_dn10 * assign84680_body20_e129796) + (locals.var_chib * (-(((locals.var_chib_dn10 / 2.0) * assign84680_body20_e129794) + (assign84680_body20_e129783 * (-(((locals.var_chib_dn10 / 3.0) * assign84680_body20_e129792) + (assign84680_body20_e129787 * (-(locals.var_chib_dn10 / 4.0)))))))))), ((locals.var_chib_dn13 * assign84680_body20_e129796) + (locals.var_chib * (-(((locals.var_chib_dn13 / 2.0) * assign84680_body20_e129794) + (assign84680_body20_e129783 * (-(((locals.var_chib_dn13 / 3.0) * assign84680_body20_e129792) + (assign84680_body20_e129787 * (-(locals.var_chib_dn13 / 4.0)))))))))),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn13,)
    }
};
            locals.var_t3 = assign84680_body20_e129799;
            locals.var_t3_dn0 = assign84680_body20_e129799_d_n0;
            locals.var_t3_dn2 = assign84680_body20_e129799_d_n2;
            locals.var_t3_dn4 = assign84680_body20_e129799_d_n4;
            locals.var_t3_dn5 = assign84680_body20_e129799_d_n5;
            locals.var_t3_dn6 = assign84680_body20_e129799_d_n6;
            locals.var_t3_dn7 = assign84680_body20_e129799_d_n7;
            locals.var_t3_dn8 = assign84680_body20_e129799_d_n8;
            locals.var_t3_dn9 = assign84680_body20_e129799_d_n9;
            locals.var_t3_dn10 = assign84680_body20_e129799_d_n10;
            locals.var_t3_dn13 = assign84680_body20_e129799_d_n13;
            let (assign84680_body21_e129809, assign84680_body21_e129809_d_n0, assign84680_body21_e129809_d_n2, assign84680_body21_e129809_d_n4, assign84680_body21_e129809_d_n5, assign84680_body21_e129809_d_n6, assign84680_body21_e129809_d_n7, assign84680_body21_e129809_d_n8, assign84680_body21_e129809_d_n9, assign84680_body21_e129809_d_n10, assign84680_body21_e129809_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1964 != 0.0)) && (locals.var_guard1970 != 0.0)) {
        let assign84680_body21_e129807: f64 = (locals.var_t0 - locals.var_t2);
        (assign84680_body21_e129807, (locals.var_t0_dn0 - locals.var_t2_dn0), (locals.var_t0_dn2 - locals.var_t2_dn2), (locals.var_t0_dn4 - locals.var_t2_dn4), (locals.var_t0_dn5 - locals.var_t2_dn5), (locals.var_t0_dn6 - locals.var_t2_dn6), (locals.var_t0_dn7 - locals.var_t2_dn7), (locals.var_t0_dn8 - locals.var_t2_dn8), (locals.var_t0_dn9 - locals.var_t2_dn9), (locals.var_t0_dn10 - locals.var_t2_dn10), (locals.var_t0_dn13 - locals.var_t2_dn13),)
    } else {
        (locals.var_fbsq__blk1891, locals.var_fbsq__blk1891_dn0, locals.var_fbsq__blk1891_dn2, locals.var_fbsq__blk1891_dn4, locals.var_fbsq__blk1891_dn5, locals.var_fbsq__blk1891_dn6, locals.var_fbsq__blk1891_dn7, locals.var_fbsq__blk1891_dn8, locals.var_fbsq__blk1891_dn9, locals.var_fbsq__blk1891_dn10, locals.var_fbsq__blk1891_dn13,)
    }
};
            locals.var_fbsq__blk1891 = assign84680_body21_e129809;
            locals.var_fbsq__blk1891_dn0 = assign84680_body21_e129809_d_n0;
            locals.var_fbsq__blk1891_dn2 = assign84680_body21_e129809_d_n2;
            locals.var_fbsq__blk1891_dn4 = assign84680_body21_e129809_d_n4;
            locals.var_fbsq__blk1891_dn5 = assign84680_body21_e129809_d_n5;
            locals.var_fbsq__blk1891_dn6 = assign84680_body21_e129809_d_n6;
            locals.var_fbsq__blk1891_dn7 = assign84680_body21_e129809_d_n7;
            locals.var_fbsq__blk1891_dn8 = assign84680_body21_e129809_d_n8;
            locals.var_fbsq__blk1891_dn9 = assign84680_body21_e129809_d_n9;
            locals.var_fbsq__blk1891_dn10 = assign84680_body21_e129809_d_n10;
            locals.var_fbsq__blk1891_dn13 = assign84680_body21_e129809_d_n13;
            let (assign84680_body22_e129823, assign84680_body22_e129823_d_n0, assign84680_body22_e129823_d_n2, assign84680_body22_e129823_d_n4, assign84680_body22_e129823_d_n5, assign84680_body22_e129823_d_n6, assign84680_body22_e129823_d_n7, assign84680_body22_e129823_d_n8, assign84680_body22_e129823_d_n9, assign84680_body22_e129823_d_n10, assign84680_body22_e129823_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1964 != 0.0)) && (locals.var_guard1970 != 0.0)) {
        let assign84680_body22_e129819: f64 = (locals.var_phi_b_dpss * locals.var_t3);
        let assign84680_body22_e129820: f64 = (locals.var_t1 - assign84680_body22_e129819);
        let assign84680_body22_e129821: f64 = (locals.var_beta * assign84680_body22_e129820);
        (assign84680_body22_e129821, ((locals.var_beta_dn0 * assign84680_body22_e129820) + (locals.var_beta * (locals.var_t1_dn0 - ((locals.var_phi_b_dpss_dn0 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn0))))), ((locals.var_beta_dn2 * assign84680_body22_e129820) + (locals.var_beta * (locals.var_t1_dn2 - ((locals.var_phi_b_dpss_dn2 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn2))))), ((locals.var_beta_dn4 * assign84680_body22_e129820) + (locals.var_beta * (locals.var_t1_dn4 - ((locals.var_phi_b_dpss_dn4 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn4))))), ((locals.var_beta_dn5 * assign84680_body22_e129820) + (locals.var_beta * (locals.var_t1_dn5 - ((locals.var_phi_b_dpss_dn5 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn5))))), ((locals.var_beta_dn6 * assign84680_body22_e129820) + (locals.var_beta * (locals.var_t1_dn6 - ((locals.var_phi_b_dpss_dn6 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn6))))), ((locals.var_beta_dn7 * assign84680_body22_e129820) + (locals.var_beta * (locals.var_t1_dn7 - ((locals.var_phi_b_dpss_dn7 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn7))))), ((locals.var_beta_dn8 * assign84680_body22_e129820) + (locals.var_beta * (locals.var_t1_dn8 - ((locals.var_phi_b_dpss_dn8 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn8))))), ((locals.var_beta_dn9 * assign84680_body22_e129820) + (locals.var_beta * (locals.var_t1_dn9 - ((locals.var_phi_b_dpss_dn9 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn9))))), ((locals.var_beta_dn10 * assign84680_body22_e129820) + (locals.var_beta * (locals.var_t1_dn10 - ((locals.var_phi_b_dpss_dn10 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn10))))), ((locals.var_beta_dn13 * assign84680_body22_e129820) + (locals.var_beta * (locals.var_t1_dn13 - ((locals.var_phi_b_dpss_dn13 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn13))))),)
    } else {
        (locals.var_fbsq_dpss__blk1892, locals.var_fbsq_dpss__blk1892_dn0, locals.var_fbsq_dpss__blk1892_dn2, locals.var_fbsq_dpss__blk1892_dn4, locals.var_fbsq_dpss__blk1892_dn5, locals.var_fbsq_dpss__blk1892_dn6, locals.var_fbsq_dpss__blk1892_dn7, locals.var_fbsq_dpss__blk1892_dn8, locals.var_fbsq_dpss__blk1892_dn9, locals.var_fbsq_dpss__blk1892_dn10, locals.var_fbsq_dpss__blk1892_dn13,)
    }
};
            locals.var_fbsq_dpss__blk1892 = assign84680_body22_e129823;
            locals.var_fbsq_dpss__blk1892_dn0 = assign84680_body22_e129823_d_n0;
            locals.var_fbsq_dpss__blk1892_dn2 = assign84680_body22_e129823_d_n2;
            locals.var_fbsq_dpss__blk1892_dn4 = assign84680_body22_e129823_d_n4;
            locals.var_fbsq_dpss__blk1892_dn5 = assign84680_body22_e129823_d_n5;
            locals.var_fbsq_dpss__blk1892_dn6 = assign84680_body22_e129823_d_n6;
            locals.var_fbsq_dpss__blk1892_dn7 = assign84680_body22_e129823_d_n7;
            locals.var_fbsq_dpss__blk1892_dn8 = assign84680_body22_e129823_d_n8;
            locals.var_fbsq_dpss__blk1892_dn9 = assign84680_body22_e129823_d_n9;
            locals.var_fbsq_dpss__blk1892_dn10 = assign84680_body22_e129823_d_n10;
            locals.var_fbsq_dpss__blk1892_dn13 = assign84680_body22_e129823_d_n13;
            let (assign84680_body24_e129851, assign84680_body24_e129851_d_n0, assign84680_body24_e129851_d_n2, assign84680_body24_e129851_d_n4, assign84680_body24_e129851_d_n5, assign84680_body24_e129851_d_n6, assign84680_body24_e129851_d_n7, assign84680_body24_e129851_d_n8, assign84680_body24_e129851_d_n9, assign84680_body24_e129851_d_n10, assign84680_body24_e129851_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1964 != 0.0)) && (locals.var_guard1970 == 0.0)) {
        let assign84680_body24_e129848: f64 = (-locals.var_chi);
        let assign84680_body24_e129849: f64 = (assign84680_body24_e129848).exp();
        (assign84680_body24_e129849, (assign84680_body24_e129849 * (-locals.var_chi_dn0)), (assign84680_body24_e129849 * (-locals.var_chi_dn2)), (assign84680_body24_e129849 * (-locals.var_chi_dn4)), (assign84680_body24_e129849 * (-locals.var_chi_dn5)), (assign84680_body24_e129849 * (-locals.var_chi_dn6)), (assign84680_body24_e129849 * (-locals.var_chi_dn7)), (assign84680_body24_e129849 * (-locals.var_chi_dn8)), (assign84680_body24_e129849 * (-locals.var_chi_dn9)), (assign84680_body24_e129849 * (-locals.var_chi_dn10)), (assign84680_body24_e129849 * (-locals.var_chi_dn13)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
            locals.var_t0 = assign84680_body24_e129851;
            locals.var_t0_dn0 = assign84680_body24_e129851_d_n0;
            locals.var_t0_dn2 = assign84680_body24_e129851_d_n2;
            locals.var_t0_dn4 = assign84680_body24_e129851_d_n4;
            locals.var_t0_dn5 = assign84680_body24_e129851_d_n5;
            locals.var_t0_dn6 = assign84680_body24_e129851_d_n6;
            locals.var_t0_dn7 = assign84680_body24_e129851_d_n7;
            locals.var_t0_dn8 = assign84680_body24_e129851_d_n8;
            locals.var_t0_dn9 = assign84680_body24_e129851_d_n9;
            locals.var_t0_dn10 = assign84680_body24_e129851_d_n10;
            locals.var_t0_dn13 = assign84680_body24_e129851_d_n13;
            let (assign84680_body25_e129862, assign84680_body25_e129862_d_n0, assign84680_body25_e129862_d_n2, assign84680_body25_e129862_d_n4, assign84680_body25_e129862_d_n5, assign84680_body25_e129862_d_n6, assign84680_body25_e129862_d_n7, assign84680_body25_e129862_d_n8, assign84680_body25_e129862_d_n9, assign84680_body25_e129862_d_n10, assign84680_body25_e129862_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1964 != 0.0)) && (locals.var_guard1970 == 0.0)) {
        let assign84680_body25_e129859: f64 = (-locals.var_chib);
        let assign84680_body25_e129860: f64 = (assign84680_body25_e129859).exp();
        (assign84680_body25_e129860, (assign84680_body25_e129860 * (-locals.var_chib_dn0)), (assign84680_body25_e129860 * (-locals.var_chib_dn2)), (assign84680_body25_e129860 * (-locals.var_chib_dn4)), (assign84680_body25_e129860 * (-locals.var_chib_dn5)), (assign84680_body25_e129860 * (-locals.var_chib_dn6)), (assign84680_body25_e129860 * (-locals.var_chib_dn7)), (assign84680_body25_e129860 * (-locals.var_chib_dn8)), (assign84680_body25_e129860 * (-locals.var_chib_dn9)), (assign84680_body25_e129860 * (-locals.var_chib_dn10)), (assign84680_body25_e129860 * (-locals.var_chib_dn13)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
            locals.var_t1 = assign84680_body25_e129862;
            locals.var_t1_dn0 = assign84680_body25_e129862_d_n0;
            locals.var_t1_dn2 = assign84680_body25_e129862_d_n2;
            locals.var_t1_dn4 = assign84680_body25_e129862_d_n4;
            locals.var_t1_dn5 = assign84680_body25_e129862_d_n5;
            locals.var_t1_dn6 = assign84680_body25_e129862_d_n6;
            locals.var_t1_dn7 = assign84680_body25_e129862_d_n7;
            locals.var_t1_dn8 = assign84680_body25_e129862_d_n8;
            locals.var_t1_dn9 = assign84680_body25_e129862_d_n9;
            locals.var_t1_dn10 = assign84680_body25_e129862_d_n10;
            locals.var_t1_dn13 = assign84680_body25_e129862_d_n13;
            let (assign84680_body26_e129877, assign84680_body26_e129877_d_n0, assign84680_body26_e129877_d_n2, assign84680_body26_e129877_d_n4, assign84680_body26_e129877_d_n5, assign84680_body26_e129877_d_n6, assign84680_body26_e129877_d_n7, assign84680_body26_e129877_d_n8, assign84680_body26_e129877_d_n9, assign84680_body26_e129877_d_n10, assign84680_body26_e129877_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1964 != 0.0)) && (locals.var_guard1970 == 0.0)) {
        let assign84680_body26_e129871: f64 = (locals.var_chi - locals.var_chib);
        let assign84680_body26_e129874: f64 = (locals.var_t0 - locals.var_t1);
        let assign84680_body26_e129875: f64 = (assign84680_body26_e129871 + assign84680_body26_e129874);
        (assign84680_body26_e129875, ((locals.var_chi_dn0 - locals.var_chib_dn0) + (locals.var_t0_dn0 - locals.var_t1_dn0)), ((locals.var_chi_dn2 - locals.var_chib_dn2) + (locals.var_t0_dn2 - locals.var_t1_dn2)), ((locals.var_chi_dn4 - locals.var_chib_dn4) + (locals.var_t0_dn4 - locals.var_t1_dn4)), ((locals.var_chi_dn5 - locals.var_chib_dn5) + (locals.var_t0_dn5 - locals.var_t1_dn5)), ((locals.var_chi_dn6 - locals.var_chib_dn6) + (locals.var_t0_dn6 - locals.var_t1_dn6)), ((locals.var_chi_dn7 - locals.var_chib_dn7) + (locals.var_t0_dn7 - locals.var_t1_dn7)), ((locals.var_chi_dn8 - locals.var_chib_dn8) + (locals.var_t0_dn8 - locals.var_t1_dn8)), ((locals.var_chi_dn9 - locals.var_chib_dn9) + (locals.var_t0_dn9 - locals.var_t1_dn9)), ((locals.var_chi_dn10 - locals.var_chib_dn10) + (locals.var_t0_dn10 - locals.var_t1_dn10)), ((locals.var_chi_dn13 - locals.var_chib_dn13) + (locals.var_t0_dn13 - locals.var_t1_dn13)),)
    } else {
        (locals.var_fbsq__blk1891, locals.var_fbsq__blk1891_dn0, locals.var_fbsq__blk1891_dn2, locals.var_fbsq__blk1891_dn4, locals.var_fbsq__blk1891_dn5, locals.var_fbsq__blk1891_dn6, locals.var_fbsq__blk1891_dn7, locals.var_fbsq__blk1891_dn8, locals.var_fbsq__blk1891_dn9, locals.var_fbsq__blk1891_dn10, locals.var_fbsq__blk1891_dn13,)
    }
};
            locals.var_fbsq__blk1891 = assign84680_body26_e129877;
            locals.var_fbsq__blk1891_dn0 = assign84680_body26_e129877_d_n0;
            locals.var_fbsq__blk1891_dn2 = assign84680_body26_e129877_d_n2;
            locals.var_fbsq__blk1891_dn4 = assign84680_body26_e129877_d_n4;
            locals.var_fbsq__blk1891_dn5 = assign84680_body26_e129877_d_n5;
            locals.var_fbsq__blk1891_dn6 = assign84680_body26_e129877_d_n6;
            locals.var_fbsq__blk1891_dn7 = assign84680_body26_e129877_d_n7;
            locals.var_fbsq__blk1891_dn8 = assign84680_body26_e129877_d_n8;
            locals.var_fbsq__blk1891_dn9 = assign84680_body26_e129877_d_n9;
            locals.var_fbsq__blk1891_dn10 = assign84680_body26_e129877_d_n10;
            locals.var_fbsq__blk1891_dn13 = assign84680_body26_e129877_d_n13;
            let (assign84680_body27_e129896, assign84680_body27_e129896_d_n0, assign84680_body27_e129896_d_n2, assign84680_body27_e129896_d_n4, assign84680_body27_e129896_d_n5, assign84680_body27_e129896_d_n6, assign84680_body27_e129896_d_n7, assign84680_body27_e129896_d_n8, assign84680_body27_e129896_d_n9, assign84680_body27_e129896_d_n10, assign84680_body27_e129896_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1964 != 0.0)) && (locals.var_guard1970 == 0.0)) {
        let assign84680_body27_e129887: f64 = (1.0 - locals.var_t0);
        let assign84680_body27_e129891: f64 = (1.0 - locals.var_t1);
        let assign84680_body27_e129892: f64 = (locals.var_phi_b_dpss * assign84680_body27_e129891);
        let assign84680_body27_e129893: f64 = (assign84680_body27_e129887 - assign84680_body27_e129892);
        let assign84680_body27_e129894: f64 = (locals.var_beta * assign84680_body27_e129893);
        (assign84680_body27_e129894, ((locals.var_beta_dn0 * assign84680_body27_e129893) + (locals.var_beta * ((-locals.var_t0_dn0) - ((locals.var_phi_b_dpss_dn0 * assign84680_body27_e129891) + (locals.var_phi_b_dpss * (-locals.var_t1_dn0)))))), ((locals.var_beta_dn2 * assign84680_body27_e129893) + (locals.var_beta * ((-locals.var_t0_dn2) - ((locals.var_phi_b_dpss_dn2 * assign84680_body27_e129891) + (locals.var_phi_b_dpss * (-locals.var_t1_dn2)))))), ((locals.var_beta_dn4 * assign84680_body27_e129893) + (locals.var_beta * ((-locals.var_t0_dn4) - ((locals.var_phi_b_dpss_dn4 * assign84680_body27_e129891) + (locals.var_phi_b_dpss * (-locals.var_t1_dn4)))))), ((locals.var_beta_dn5 * assign84680_body27_e129893) + (locals.var_beta * ((-locals.var_t0_dn5) - ((locals.var_phi_b_dpss_dn5 * assign84680_body27_e129891) + (locals.var_phi_b_dpss * (-locals.var_t1_dn5)))))), ((locals.var_beta_dn6 * assign84680_body27_e129893) + (locals.var_beta * ((-locals.var_t0_dn6) - ((locals.var_phi_b_dpss_dn6 * assign84680_body27_e129891) + (locals.var_phi_b_dpss * (-locals.var_t1_dn6)))))), ((locals.var_beta_dn7 * assign84680_body27_e129893) + (locals.var_beta * ((-locals.var_t0_dn7) - ((locals.var_phi_b_dpss_dn7 * assign84680_body27_e129891) + (locals.var_phi_b_dpss * (-locals.var_t1_dn7)))))), ((locals.var_beta_dn8 * assign84680_body27_e129893) + (locals.var_beta * ((-locals.var_t0_dn8) - ((locals.var_phi_b_dpss_dn8 * assign84680_body27_e129891) + (locals.var_phi_b_dpss * (-locals.var_t1_dn8)))))), ((locals.var_beta_dn9 * assign84680_body27_e129893) + (locals.var_beta * ((-locals.var_t0_dn9) - ((locals.var_phi_b_dpss_dn9 * assign84680_body27_e129891) + (locals.var_phi_b_dpss * (-locals.var_t1_dn9)))))), ((locals.var_beta_dn10 * assign84680_body27_e129893) + (locals.var_beta * ((-locals.var_t0_dn10) - ((locals.var_phi_b_dpss_dn10 * assign84680_body27_e129891) + (locals.var_phi_b_dpss * (-locals.var_t1_dn10)))))), ((locals.var_beta_dn13 * assign84680_body27_e129893) + (locals.var_beta * ((-locals.var_t0_dn13) - ((locals.var_phi_b_dpss_dn13 * assign84680_body27_e129891) + (locals.var_phi_b_dpss * (-locals.var_t1_dn13)))))),)
    } else {
        (locals.var_fbsq_dpss__blk1892, locals.var_fbsq_dpss__blk1892_dn0, locals.var_fbsq_dpss__blk1892_dn2, locals.var_fbsq_dpss__blk1892_dn4, locals.var_fbsq_dpss__blk1892_dn5, locals.var_fbsq_dpss__blk1892_dn6, locals.var_fbsq_dpss__blk1892_dn7, locals.var_fbsq_dpss__blk1892_dn8, locals.var_fbsq_dpss__blk1892_dn9, locals.var_fbsq_dpss__blk1892_dn10, locals.var_fbsq_dpss__blk1892_dn13,)
    }
};
            locals.var_fbsq_dpss__blk1892 = assign84680_body27_e129896;
            locals.var_fbsq_dpss__blk1892_dn0 = assign84680_body27_e129896_d_n0;
            locals.var_fbsq_dpss__blk1892_dn2 = assign84680_body27_e129896_d_n2;
            locals.var_fbsq_dpss__blk1892_dn4 = assign84680_body27_e129896_d_n4;
            locals.var_fbsq_dpss__blk1892_dn5 = assign84680_body27_e129896_d_n5;
            locals.var_fbsq_dpss__blk1892_dn6 = assign84680_body27_e129896_d_n6;
            locals.var_fbsq_dpss__blk1892_dn7 = assign84680_body27_e129896_d_n7;
            locals.var_fbsq_dpss__blk1892_dn8 = assign84680_body27_e129896_d_n8;
            locals.var_fbsq_dpss__blk1892_dn9 = assign84680_body27_e129896_d_n9;
            locals.var_fbsq_dpss__blk1892_dn10 = assign84680_body27_e129896_d_n10;
            locals.var_fbsq_dpss__blk1892_dn13 = assign84680_body27_e129896_d_n13;
            let assign84680_body28_e129898: f64 = (locals.var_chi).abs();
            let assign84680_body28_e129900: f64 = if assign84680_body28_e129898 < 5e-5 { 1.0 } else { 0.0 };
            locals.var_guard1971 = assign84680_body28_e129900;
            let (assign84680_body29_e129930, assign84680_body29_e129930_d_n0, assign84680_body29_e129930_d_n2, assign84680_body29_e129930_d_n4, assign84680_body29_e129930_d_n5, assign84680_body29_e129930_d_n6, assign84680_body29_e129930_d_n7, assign84680_body29_e129930_d_n8, assign84680_body29_e129930_d_n9, assign84680_body29_e129930_d_n10, assign84680_body29_e129930_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1964 != 0.0)) && (locals.var_guard1971 != 0.0)) {
        let assign84680_body29_e129908: f64 = (locals.var_chi * locals.var_chi);
        let assign84680_body29_e129910: f64 = (assign84680_body29_e129908 / 2.0);
        let assign84680_body29_e129914: f64 = (locals.var_chi / 3.0);
        let assign84680_body29_e129918: f64 = (locals.var_chi / 4.0);
        let assign84680_body29_e129922: f64 = (locals.var_chi / 5.0);
        let assign84680_body29_e129923: f64 = (1.0 + assign84680_body29_e129922);
        let assign84680_body29_e129924: f64 = (assign84680_body29_e129918 * assign84680_body29_e129923);
        let assign84680_body29_e129925: f64 = (1.0 + assign84680_body29_e129924);
        let assign84680_body29_e129926: f64 = (assign84680_body29_e129914 * assign84680_body29_e129925);
        let assign84680_body29_e129927: f64 = (1.0 + assign84680_body29_e129926);
        let assign84680_body29_e129928: f64 = (assign84680_body29_e129910 * assign84680_body29_e129927);
        (assign84680_body29_e129928, (((((locals.var_chi_dn0 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn0)) / 2.0) * assign84680_body29_e129927) + (assign84680_body29_e129910 * (((locals.var_chi_dn0 / 3.0) * assign84680_body29_e129925) + (assign84680_body29_e129914 * (((locals.var_chi_dn0 / 4.0) * assign84680_body29_e129923) + (assign84680_body29_e129918 * (locals.var_chi_dn0 / 5.0))))))), (((((locals.var_chi_dn2 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn2)) / 2.0) * assign84680_body29_e129927) + (assign84680_body29_e129910 * (((locals.var_chi_dn2 / 3.0) * assign84680_body29_e129925) + (assign84680_body29_e129914 * (((locals.var_chi_dn2 / 4.0) * assign84680_body29_e129923) + (assign84680_body29_e129918 * (locals.var_chi_dn2 / 5.0))))))), (((((locals.var_chi_dn4 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn4)) / 2.0) * assign84680_body29_e129927) + (assign84680_body29_e129910 * (((locals.var_chi_dn4 / 3.0) * assign84680_body29_e129925) + (assign84680_body29_e129914 * (((locals.var_chi_dn4 / 4.0) * assign84680_body29_e129923) + (assign84680_body29_e129918 * (locals.var_chi_dn4 / 5.0))))))), (((((locals.var_chi_dn5 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn5)) / 2.0) * assign84680_body29_e129927) + (assign84680_body29_e129910 * (((locals.var_chi_dn5 / 3.0) * assign84680_body29_e129925) + (assign84680_body29_e129914 * (((locals.var_chi_dn5 / 4.0) * assign84680_body29_e129923) + (assign84680_body29_e129918 * (locals.var_chi_dn5 / 5.0))))))), (((((locals.var_chi_dn6 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn6)) / 2.0) * assign84680_body29_e129927) + (assign84680_body29_e129910 * (((locals.var_chi_dn6 / 3.0) * assign84680_body29_e129925) + (assign84680_body29_e129914 * (((locals.var_chi_dn6 / 4.0) * assign84680_body29_e129923) + (assign84680_body29_e129918 * (locals.var_chi_dn6 / 5.0))))))), (((((locals.var_chi_dn7 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn7)) / 2.0) * assign84680_body29_e129927) + (assign84680_body29_e129910 * (((locals.var_chi_dn7 / 3.0) * assign84680_body29_e129925) + (assign84680_body29_e129914 * (((locals.var_chi_dn7 / 4.0) * assign84680_body29_e129923) + (assign84680_body29_e129918 * (locals.var_chi_dn7 / 5.0))))))), (((((locals.var_chi_dn8 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn8)) / 2.0) * assign84680_body29_e129927) + (assign84680_body29_e129910 * (((locals.var_chi_dn8 / 3.0) * assign84680_body29_e129925) + (assign84680_body29_e129914 * (((locals.var_chi_dn8 / 4.0) * assign84680_body29_e129923) + (assign84680_body29_e129918 * (locals.var_chi_dn8 / 5.0))))))), (((((locals.var_chi_dn9 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn9)) / 2.0) * assign84680_body29_e129927) + (assign84680_body29_e129910 * (((locals.var_chi_dn9 / 3.0) * assign84680_body29_e129925) + (assign84680_body29_e129914 * (((locals.var_chi_dn9 / 4.0) * assign84680_body29_e129923) + (assign84680_body29_e129918 * (locals.var_chi_dn9 / 5.0))))))), (((((locals.var_chi_dn10 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn10)) / 2.0) * assign84680_body29_e129927) + (assign84680_body29_e129910 * (((locals.var_chi_dn10 / 3.0) * assign84680_body29_e129925) + (assign84680_body29_e129914 * (((locals.var_chi_dn10 / 4.0) * assign84680_body29_e129923) + (assign84680_body29_e129918 * (locals.var_chi_dn10 / 5.0))))))), (((((locals.var_chi_dn13 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn13)) / 2.0) * assign84680_body29_e129927) + (assign84680_body29_e129910 * (((locals.var_chi_dn13 / 3.0) * assign84680_body29_e129925) + (assign84680_body29_e129914 * (((locals.var_chi_dn13 / 4.0) * assign84680_body29_e129923) + (assign84680_body29_e129918 * (locals.var_chi_dn13 / 5.0))))))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
            locals.var_t0 = assign84680_body29_e129930;
            locals.var_t0_dn0 = assign84680_body29_e129930_d_n0;
            locals.var_t0_dn2 = assign84680_body29_e129930_d_n2;
            locals.var_t0_dn4 = assign84680_body29_e129930_d_n4;
            locals.var_t0_dn5 = assign84680_body29_e129930_d_n5;
            locals.var_t0_dn6 = assign84680_body29_e129930_d_n6;
            locals.var_t0_dn7 = assign84680_body29_e129930_d_n7;
            locals.var_t0_dn8 = assign84680_body29_e129930_d_n8;
            locals.var_t0_dn9 = assign84680_body29_e129930_d_n9;
            locals.var_t0_dn10 = assign84680_body29_e129930_d_n10;
            locals.var_t0_dn13 = assign84680_body29_e129930_d_n13;
            let (assign84680_body30_e129956, assign84680_body30_e129956_d_n0, assign84680_body30_e129956_d_n2, assign84680_body30_e129956_d_n4, assign84680_body30_e129956_d_n5, assign84680_body30_e129956_d_n6, assign84680_body30_e129956_d_n7, assign84680_body30_e129956_d_n8, assign84680_body30_e129956_d_n9, assign84680_body30_e129956_d_n10, assign84680_body30_e129956_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1964 != 0.0)) && (locals.var_guard1971 != 0.0)) {
        let assign84680_body30_e129940: f64 = (locals.var_chi / 2.0);
        let assign84680_body30_e129944: f64 = (locals.var_chi / 3.0);
        let assign84680_body30_e129948: f64 = (locals.var_chi / 4.0);
        let assign84680_body30_e129949: f64 = (1.0 + assign84680_body30_e129948);
        let assign84680_body30_e129950: f64 = (assign84680_body30_e129944 * assign84680_body30_e129949);
        let assign84680_body30_e129951: f64 = (1.0 + assign84680_body30_e129950);
        let assign84680_body30_e129952: f64 = (assign84680_body30_e129940 * assign84680_body30_e129951);
        let assign84680_body30_e129953: f64 = (1.0 + assign84680_body30_e129952);
        let assign84680_body30_e129954: f64 = (locals.var_chi * assign84680_body30_e129953);
        (assign84680_body30_e129954, ((locals.var_chi_dn0 * assign84680_body30_e129953) + (locals.var_chi * (((locals.var_chi_dn0 / 2.0) * assign84680_body30_e129951) + (assign84680_body30_e129940 * (((locals.var_chi_dn0 / 3.0) * assign84680_body30_e129949) + (assign84680_body30_e129944 * (locals.var_chi_dn0 / 4.0))))))), ((locals.var_chi_dn2 * assign84680_body30_e129953) + (locals.var_chi * (((locals.var_chi_dn2 / 2.0) * assign84680_body30_e129951) + (assign84680_body30_e129940 * (((locals.var_chi_dn2 / 3.0) * assign84680_body30_e129949) + (assign84680_body30_e129944 * (locals.var_chi_dn2 / 4.0))))))), ((locals.var_chi_dn4 * assign84680_body30_e129953) + (locals.var_chi * (((locals.var_chi_dn4 / 2.0) * assign84680_body30_e129951) + (assign84680_body30_e129940 * (((locals.var_chi_dn4 / 3.0) * assign84680_body30_e129949) + (assign84680_body30_e129944 * (locals.var_chi_dn4 / 4.0))))))), ((locals.var_chi_dn5 * assign84680_body30_e129953) + (locals.var_chi * (((locals.var_chi_dn5 / 2.0) * assign84680_body30_e129951) + (assign84680_body30_e129940 * (((locals.var_chi_dn5 / 3.0) * assign84680_body30_e129949) + (assign84680_body30_e129944 * (locals.var_chi_dn5 / 4.0))))))), ((locals.var_chi_dn6 * assign84680_body30_e129953) + (locals.var_chi * (((locals.var_chi_dn6 / 2.0) * assign84680_body30_e129951) + (assign84680_body30_e129940 * (((locals.var_chi_dn6 / 3.0) * assign84680_body30_e129949) + (assign84680_body30_e129944 * (locals.var_chi_dn6 / 4.0))))))), ((locals.var_chi_dn7 * assign84680_body30_e129953) + (locals.var_chi * (((locals.var_chi_dn7 / 2.0) * assign84680_body30_e129951) + (assign84680_body30_e129940 * (((locals.var_chi_dn7 / 3.0) * assign84680_body30_e129949) + (assign84680_body30_e129944 * (locals.var_chi_dn7 / 4.0))))))), ((locals.var_chi_dn8 * assign84680_body30_e129953) + (locals.var_chi * (((locals.var_chi_dn8 / 2.0) * assign84680_body30_e129951) + (assign84680_body30_e129940 * (((locals.var_chi_dn8 / 3.0) * assign84680_body30_e129949) + (assign84680_body30_e129944 * (locals.var_chi_dn8 / 4.0))))))), ((locals.var_chi_dn9 * assign84680_body30_e129953) + (locals.var_chi * (((locals.var_chi_dn9 / 2.0) * assign84680_body30_e129951) + (assign84680_body30_e129940 * (((locals.var_chi_dn9 / 3.0) * assign84680_body30_e129949) + (assign84680_body30_e129944 * (locals.var_chi_dn9 / 4.0))))))), ((locals.var_chi_dn10 * assign84680_body30_e129953) + (locals.var_chi * (((locals.var_chi_dn10 / 2.0) * assign84680_body30_e129951) + (assign84680_body30_e129940 * (((locals.var_chi_dn10 / 3.0) * assign84680_body30_e129949) + (assign84680_body30_e129944 * (locals.var_chi_dn10 / 4.0))))))), ((locals.var_chi_dn13 * assign84680_body30_e129953) + (locals.var_chi * (((locals.var_chi_dn13 / 2.0) * assign84680_body30_e129951) + (assign84680_body30_e129940 * (((locals.var_chi_dn13 / 3.0) * assign84680_body30_e129949) + (assign84680_body30_e129944 * (locals.var_chi_dn13 / 4.0))))))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
            locals.var_t1 = assign84680_body30_e129956;
            locals.var_t1_dn0 = assign84680_body30_e129956_d_n0;
            locals.var_t1_dn2 = assign84680_body30_e129956_d_n2;
            locals.var_t1_dn4 = assign84680_body30_e129956_d_n4;
            locals.var_t1_dn5 = assign84680_body30_e129956_d_n5;
            locals.var_t1_dn6 = assign84680_body30_e129956_d_n6;
            locals.var_t1_dn7 = assign84680_body30_e129956_d_n7;
            locals.var_t1_dn8 = assign84680_body30_e129956_d_n8;
            locals.var_t1_dn9 = assign84680_body30_e129956_d_n9;
            locals.var_t1_dn10 = assign84680_body30_e129956_d_n10;
            locals.var_t1_dn13 = assign84680_body30_e129956_d_n13;
            let (assign84680_body31_e129966, assign84680_body31_e129966_d_n0, assign84680_body31_e129966_d_n2, assign84680_body31_e129966_d_n4, assign84680_body31_e129966_d_n5, assign84680_body31_e129966_d_n6, assign84680_body31_e129966_d_n7, assign84680_body31_e129966_d_n8, assign84680_body31_e129966_d_n9, assign84680_body31_e129966_d_n10, assign84680_body31_e129966_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1964 != 0.0)) && (locals.var_guard1971 != 0.0)) {
        let assign84680_body31_e129964: f64 = (locals.var_cfs1 * locals.var_t0);
        (assign84680_body31_e129964, ((locals.var_cfs1_dn0 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn0)), ((locals.var_cfs1_dn2 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn2)), ((locals.var_cfs1_dn4 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn4)), ((locals.var_cfs1_dn5 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn5)), ((locals.var_cfs1_dn6 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn6)), ((locals.var_cfs1_dn7 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn7)), ((locals.var_cfs1_dn8 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn8)), ((locals.var_cfs1_dn9 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn9)), ((locals.var_cfs1_dn10 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn10)), ((locals.var_cfs1_dn13 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn13)),)
    } else {
        (locals.var_fs01, locals.var_fs01_dn0, locals.var_fs01_dn2, locals.var_fs01_dn4, locals.var_fs01_dn5, locals.var_fs01_dn6, locals.var_fs01_dn7, locals.var_fs01_dn8, locals.var_fs01_dn9, locals.var_fs01_dn10, locals.var_fs01_dn13,)
    }
};
            locals.var_fs01 = assign84680_body31_e129966;
            locals.var_fs01_dn0 = assign84680_body31_e129966_d_n0;
            locals.var_fs01_dn2 = assign84680_body31_e129966_d_n2;
            locals.var_fs01_dn4 = assign84680_body31_e129966_d_n4;
            locals.var_fs01_dn5 = assign84680_body31_e129966_d_n5;
            locals.var_fs01_dn6 = assign84680_body31_e129966_d_n6;
            locals.var_fs01_dn7 = assign84680_body31_e129966_d_n7;
            locals.var_fs01_dn8 = assign84680_body31_e129966_d_n8;
            locals.var_fs01_dn9 = assign84680_body31_e129966_d_n9;
            locals.var_fs01_dn10 = assign84680_body31_e129966_d_n10;
            locals.var_fs01_dn13 = assign84680_body31_e129966_d_n13;
            let (assign84680_body32_e129978, assign84680_body32_e129978_d_n0, assign84680_body32_e129978_d_n2, assign84680_body32_e129978_d_n4, assign84680_body32_e129978_d_n5, assign84680_body32_e129978_d_n6, assign84680_body32_e129978_d_n7, assign84680_body32_e129978_d_n8, assign84680_body32_e129978_d_n9, assign84680_body32_e129978_d_n10, assign84680_body32_e129978_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1964 != 0.0)) && (locals.var_guard1971 != 0.0)) {
        let assign84680_body32_e129974: f64 = (locals.var_cfs1 * locals.var_t1);
        let assign84680_body32_e129976: f64 = (assign84680_body32_e129974 * locals.var_beta);
        (assign84680_body32_e129976, ((((locals.var_cfs1_dn0 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn0)) * locals.var_beta) + (assign84680_body32_e129974 * locals.var_beta_dn0)), ((((locals.var_cfs1_dn2 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn2)) * locals.var_beta) + (assign84680_body32_e129974 * locals.var_beta_dn2)), ((((locals.var_cfs1_dn4 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn4)) * locals.var_beta) + (assign84680_body32_e129974 * locals.var_beta_dn4)), ((((locals.var_cfs1_dn5 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn5)) * locals.var_beta) + (assign84680_body32_e129974 * locals.var_beta_dn5)), ((((locals.var_cfs1_dn6 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn6)) * locals.var_beta) + (assign84680_body32_e129974 * locals.var_beta_dn6)), ((((locals.var_cfs1_dn7 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn7)) * locals.var_beta) + (assign84680_body32_e129974 * locals.var_beta_dn7)), ((((locals.var_cfs1_dn8 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn8)) * locals.var_beta) + (assign84680_body32_e129974 * locals.var_beta_dn8)), ((((locals.var_cfs1_dn9 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn9)) * locals.var_beta) + (assign84680_body32_e129974 * locals.var_beta_dn9)), ((((locals.var_cfs1_dn10 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn10)) * locals.var_beta) + (assign84680_body32_e129974 * locals.var_beta_dn10)), ((((locals.var_cfs1_dn13 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn13)) * locals.var_beta) + (assign84680_body32_e129974 * locals.var_beta_dn13)),)
    } else {
        (locals.var_fs01_dps0, locals.var_fs01_dps0_dn0, locals.var_fs01_dps0_dn2, locals.var_fs01_dps0_dn4, locals.var_fs01_dps0_dn5, locals.var_fs01_dps0_dn6, locals.var_fs01_dps0_dn7, locals.var_fs01_dps0_dn8, locals.var_fs01_dps0_dn9, locals.var_fs01_dps0_dn10, locals.var_fs01_dps0_dn13,)
    }
};
            locals.var_fs01_dps0 = assign84680_body32_e129978;
            locals.var_fs01_dps0_dn0 = assign84680_body32_e129978_d_n0;
            locals.var_fs01_dps0_dn2 = assign84680_body32_e129978_d_n2;
            locals.var_fs01_dps0_dn4 = assign84680_body32_e129978_d_n4;
            locals.var_fs01_dps0_dn5 = assign84680_body32_e129978_d_n5;
            locals.var_fs01_dps0_dn6 = assign84680_body32_e129978_d_n6;
            locals.var_fs01_dps0_dn7 = assign84680_body32_e129978_d_n7;
            locals.var_fs01_dps0_dn8 = assign84680_body32_e129978_d_n8;
            locals.var_fs01_dps0_dn9 = assign84680_body32_e129978_d_n9;
            locals.var_fs01_dps0_dn10 = assign84680_body32_e129978_d_n10;
            locals.var_fs01_dps0_dn13 = assign84680_body32_e129978_d_n13;
            let assign84680_body33_e129980: f64 = (locals.var_chi).abs();
            let assign84680_body33_e129982: f64 = if assign84680_body33_e129980 < 60.0 { 1.0 } else { 0.0 };
            locals.var_guard1972 = assign84680_body33_e129982;
            let (assign84680_body35_e130013, assign84680_body35_e130013_d_n0, assign84680_body35_e130013_d_n2, assign84680_body35_e130013_d_n4, assign84680_body35_e130013_d_n5, assign84680_body35_e130013_d_n6, assign84680_body35_e130013_d_n7, assign84680_body35_e130013_d_n8, assign84680_body35_e130013_d_n9, assign84680_body35_e130013_d_n10, assign84680_body35_e130013_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1964 != 0.0)) && (locals.var_guard1971 == 0.0)) && (locals.var_guard1972 != 0.0)) {
        let assign84680_body35_e130011: f64 = (locals.var_chi).exp();
        (assign84680_body35_e130011, (assign84680_body35_e130011 * locals.var_chi_dn0), (assign84680_body35_e130011 * locals.var_chi_dn2), (assign84680_body35_e130011 * locals.var_chi_dn4), (assign84680_body35_e130011 * locals.var_chi_dn5), (assign84680_body35_e130011 * locals.var_chi_dn6), (assign84680_body35_e130011 * locals.var_chi_dn7), (assign84680_body35_e130011 * locals.var_chi_dn8), (assign84680_body35_e130011 * locals.var_chi_dn9), (assign84680_body35_e130011 * locals.var_chi_dn10), (assign84680_body35_e130011 * locals.var_chi_dn13),)
    } else {
        (locals.var_exp_chi, locals.var_exp_chi_dn0, locals.var_exp_chi_dn2, locals.var_exp_chi_dn4, locals.var_exp_chi_dn5, locals.var_exp_chi_dn6, locals.var_exp_chi_dn7, locals.var_exp_chi_dn8, locals.var_exp_chi_dn9, locals.var_exp_chi_dn10, locals.var_exp_chi_dn13,)
    }
};
            locals.var_exp_chi = assign84680_body35_e130013;
            locals.var_exp_chi_dn0 = assign84680_body35_e130013_d_n0;
            locals.var_exp_chi_dn2 = assign84680_body35_e130013_d_n2;
            locals.var_exp_chi_dn4 = assign84680_body35_e130013_d_n4;
            locals.var_exp_chi_dn5 = assign84680_body35_e130013_d_n5;
            locals.var_exp_chi_dn6 = assign84680_body35_e130013_d_n6;
            locals.var_exp_chi_dn7 = assign84680_body35_e130013_d_n7;
            locals.var_exp_chi_dn8 = assign84680_body35_e130013_d_n8;
            locals.var_exp_chi_dn9 = assign84680_body35_e130013_d_n9;
            locals.var_exp_chi_dn10 = assign84680_body35_e130013_d_n10;
            locals.var_exp_chi_dn13 = assign84680_body35_e130013_d_n13;
            let (assign84680_body36_e130026, assign84680_body36_e130026_d_n0, assign84680_body36_e130026_d_n2, assign84680_body36_e130026_d_n4, assign84680_body36_e130026_d_n5, assign84680_body36_e130026_d_n6, assign84680_body36_e130026_d_n7, assign84680_body36_e130026_d_n8, assign84680_body36_e130026_d_n9, assign84680_body36_e130026_d_n10, assign84680_body36_e130026_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1964 != 0.0)) && (locals.var_guard1971 == 0.0)) && (locals.var_guard1972 != 0.0)) {
        let assign84680_body36_e130024: f64 = (locals.var_exp_chi - 1.0);
        (assign84680_body36_e130024, locals.var_exp_chi_dn0, locals.var_exp_chi_dn2, locals.var_exp_chi_dn4, locals.var_exp_chi_dn5, locals.var_exp_chi_dn6, locals.var_exp_chi_dn7, locals.var_exp_chi_dn8, locals.var_exp_chi_dn9, locals.var_exp_chi_dn10, locals.var_exp_chi_dn13,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
            locals.var_t1 = assign84680_body36_e130026;
            locals.var_t1_dn0 = assign84680_body36_e130026_d_n0;
            locals.var_t1_dn2 = assign84680_body36_e130026_d_n2;
            locals.var_t1_dn4 = assign84680_body36_e130026_d_n4;
            locals.var_t1_dn5 = assign84680_body36_e130026_d_n5;
            locals.var_t1_dn6 = assign84680_body36_e130026_d_n6;
            locals.var_t1_dn7 = assign84680_body36_e130026_d_n7;
            locals.var_t1_dn8 = assign84680_body36_e130026_d_n8;
            locals.var_t1_dn9 = assign84680_body36_e130026_d_n9;
            locals.var_t1_dn10 = assign84680_body36_e130026_d_n10;
            locals.var_t1_dn13 = assign84680_body36_e130026_d_n13;
            let (assign84680_body37_e130041, assign84680_body37_e130041_d_n0, assign84680_body37_e130041_d_n2, assign84680_body37_e130041_d_n4, assign84680_body37_e130041_d_n5, assign84680_body37_e130041_d_n6, assign84680_body37_e130041_d_n7, assign84680_body37_e130041_d_n8, assign84680_body37_e130041_d_n9, assign84680_body37_e130041_d_n10, assign84680_body37_e130041_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1964 != 0.0)) && (locals.var_guard1971 == 0.0)) && (locals.var_guard1972 != 0.0)) {
        let assign84680_body37_e130038: f64 = (locals.var_t1 - locals.var_chi);
        let assign84680_body37_e130039: f64 = (locals.var_cfs1 * assign84680_body37_e130038);
        (assign84680_body37_e130039, ((locals.var_cfs1_dn0 * assign84680_body37_e130038) + (locals.var_cfs1 * (locals.var_t1_dn0 - locals.var_chi_dn0))), ((locals.var_cfs1_dn2 * assign84680_body37_e130038) + (locals.var_cfs1 * (locals.var_t1_dn2 - locals.var_chi_dn2))), ((locals.var_cfs1_dn4 * assign84680_body37_e130038) + (locals.var_cfs1 * (locals.var_t1_dn4 - locals.var_chi_dn4))), ((locals.var_cfs1_dn5 * assign84680_body37_e130038) + (locals.var_cfs1 * (locals.var_t1_dn5 - locals.var_chi_dn5))), ((locals.var_cfs1_dn6 * assign84680_body37_e130038) + (locals.var_cfs1 * (locals.var_t1_dn6 - locals.var_chi_dn6))), ((locals.var_cfs1_dn7 * assign84680_body37_e130038) + (locals.var_cfs1 * (locals.var_t1_dn7 - locals.var_chi_dn7))), ((locals.var_cfs1_dn8 * assign84680_body37_e130038) + (locals.var_cfs1 * (locals.var_t1_dn8 - locals.var_chi_dn8))), ((locals.var_cfs1_dn9 * assign84680_body37_e130038) + (locals.var_cfs1 * (locals.var_t1_dn9 - locals.var_chi_dn9))), ((locals.var_cfs1_dn10 * assign84680_body37_e130038) + (locals.var_cfs1 * (locals.var_t1_dn10 - locals.var_chi_dn10))), ((locals.var_cfs1_dn13 * assign84680_body37_e130038) + (locals.var_cfs1 * (locals.var_t1_dn13 - locals.var_chi_dn13))),)
    } else {
        (locals.var_fs01, locals.var_fs01_dn0, locals.var_fs01_dn2, locals.var_fs01_dn4, locals.var_fs01_dn5, locals.var_fs01_dn6, locals.var_fs01_dn7, locals.var_fs01_dn8, locals.var_fs01_dn9, locals.var_fs01_dn10, locals.var_fs01_dn13,)
    }
};
            locals.var_fs01 = assign84680_body37_e130041;
            locals.var_fs01_dn0 = assign84680_body37_e130041_d_n0;
            locals.var_fs01_dn2 = assign84680_body37_e130041_d_n2;
            locals.var_fs01_dn4 = assign84680_body37_e130041_d_n4;
            locals.var_fs01_dn5 = assign84680_body37_e130041_d_n5;
            locals.var_fs01_dn6 = assign84680_body37_e130041_d_n6;
            locals.var_fs01_dn7 = assign84680_body37_e130041_d_n7;
            locals.var_fs01_dn8 = assign84680_body37_e130041_d_n8;
            locals.var_fs01_dn9 = assign84680_body37_e130041_d_n9;
            locals.var_fs01_dn10 = assign84680_body37_e130041_d_n10;
            locals.var_fs01_dn13 = assign84680_body37_e130041_d_n13;
            let (assign84680_body38_e130056, assign84680_body38_e130056_d_n0, assign84680_body38_e130056_d_n2, assign84680_body38_e130056_d_n4, assign84680_body38_e130056_d_n5, assign84680_body38_e130056_d_n6, assign84680_body38_e130056_d_n7, assign84680_body38_e130056_d_n8, assign84680_body38_e130056_d_n9, assign84680_body38_e130056_d_n10, assign84680_body38_e130056_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1964 != 0.0)) && (locals.var_guard1971 == 0.0)) && (locals.var_guard1972 != 0.0)) {
        let assign84680_body38_e130052: f64 = (locals.var_cfs1 * locals.var_beta);
        let assign84680_body38_e130054: f64 = (assign84680_body38_e130052 * locals.var_t1);
        (assign84680_body38_e130054, ((((locals.var_cfs1_dn0 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn0)) * locals.var_t1) + (assign84680_body38_e130052 * locals.var_t1_dn0)), ((((locals.var_cfs1_dn2 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn2)) * locals.var_t1) + (assign84680_body38_e130052 * locals.var_t1_dn2)), ((((locals.var_cfs1_dn4 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn4)) * locals.var_t1) + (assign84680_body38_e130052 * locals.var_t1_dn4)), ((((locals.var_cfs1_dn5 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn5)) * locals.var_t1) + (assign84680_body38_e130052 * locals.var_t1_dn5)), ((((locals.var_cfs1_dn6 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn6)) * locals.var_t1) + (assign84680_body38_e130052 * locals.var_t1_dn6)), ((((locals.var_cfs1_dn7 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn7)) * locals.var_t1) + (assign84680_body38_e130052 * locals.var_t1_dn7)), ((((locals.var_cfs1_dn8 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn8)) * locals.var_t1) + (assign84680_body38_e130052 * locals.var_t1_dn8)), ((((locals.var_cfs1_dn9 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn9)) * locals.var_t1) + (assign84680_body38_e130052 * locals.var_t1_dn9)), ((((locals.var_cfs1_dn10 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn10)) * locals.var_t1) + (assign84680_body38_e130052 * locals.var_t1_dn10)), ((((locals.var_cfs1_dn13 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn13)) * locals.var_t1) + (assign84680_body38_e130052 * locals.var_t1_dn13)),)
    } else {
        (locals.var_fs01_dps0, locals.var_fs01_dps0_dn0, locals.var_fs01_dps0_dn2, locals.var_fs01_dps0_dn4, locals.var_fs01_dps0_dn5, locals.var_fs01_dps0_dn6, locals.var_fs01_dps0_dn7, locals.var_fs01_dps0_dn8, locals.var_fs01_dps0_dn9, locals.var_fs01_dps0_dn10, locals.var_fs01_dps0_dn13,)
    }
};
            locals.var_fs01_dps0 = assign84680_body38_e130056;
            locals.var_fs01_dps0_dn0 = assign84680_body38_e130056_d_n0;
            locals.var_fs01_dps0_dn2 = assign84680_body38_e130056_d_n2;
            locals.var_fs01_dps0_dn4 = assign84680_body38_e130056_d_n4;
            locals.var_fs01_dps0_dn5 = assign84680_body38_e130056_d_n5;
            locals.var_fs01_dps0_dn6 = assign84680_body38_e130056_d_n6;
            locals.var_fs01_dps0_dn7 = assign84680_body38_e130056_d_n7;
            locals.var_fs01_dps0_dn8 = assign84680_body38_e130056_d_n8;
            locals.var_fs01_dps0_dn9 = assign84680_body38_e130056_d_n9;
            locals.var_fs01_dps0_dn10 = assign84680_body38_e130056_d_n10;
            locals.var_fs01_dps0_dn13 = assign84680_body38_e130056_d_n13;
            let (assign84680_body40_e130091, assign84680_body40_e130091_d_n0, assign84680_body40_e130091_d_n2, assign84680_body40_e130091_d_n4, assign84680_body40_e130091_d_n5, assign84680_body40_e130091_d_n6, assign84680_body40_e130091_d_n7, assign84680_body40_e130091_d_n8, assign84680_body40_e130091_d_n9, assign84680_body40_e130091_d_n10, assign84680_body40_e130091_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1964 != 0.0)) && (locals.var_guard1971 == 0.0)) && (locals.var_guard1972 == 0.0)) {
        let assign84680_body40_e130088: f64 = (locals.var_beta * locals.var_ps0ld);
        let assign84680_body40_e130089: f64 = (assign84680_body40_e130088).exp();
        (assign84680_body40_e130089, (assign84680_body40_e130089 * ((locals.var_beta_dn0 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn0))), (assign84680_body40_e130089 * ((locals.var_beta_dn2 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn2))), (assign84680_body40_e130089 * ((locals.var_beta_dn4 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn4))), (assign84680_body40_e130089 * ((locals.var_beta_dn5 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn5))), (assign84680_body40_e130089 * ((locals.var_beta_dn6 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn6))), (assign84680_body40_e130089 * ((locals.var_beta_dn7 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn7))), (assign84680_body40_e130089 * ((locals.var_beta_dn8 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn8))), (assign84680_body40_e130089 * ((locals.var_beta_dn9 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn9))), (assign84680_body40_e130089 * ((locals.var_beta_dn10 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn10))), (assign84680_body40_e130089 * ((locals.var_beta_dn13 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn13))),)
    } else {
        (locals.var_exp_bps0, locals.var_exp_bps0_dn0, locals.var_exp_bps0_dn2, locals.var_exp_bps0_dn4, locals.var_exp_bps0_dn5, locals.var_exp_bps0_dn6, locals.var_exp_bps0_dn7, locals.var_exp_bps0_dn8, locals.var_exp_bps0_dn9, locals.var_exp_bps0_dn10, locals.var_exp_bps0_dn13,)
    }
};
            locals.var_exp_bps0 = assign84680_body40_e130091;
            locals.var_exp_bps0_dn0 = assign84680_body40_e130091_d_n0;
            locals.var_exp_bps0_dn2 = assign84680_body40_e130091_d_n2;
            locals.var_exp_bps0_dn4 = assign84680_body40_e130091_d_n4;
            locals.var_exp_bps0_dn5 = assign84680_body40_e130091_d_n5;
            locals.var_exp_bps0_dn6 = assign84680_body40_e130091_d_n6;
            locals.var_exp_bps0_dn7 = assign84680_body40_e130091_d_n7;
            locals.var_exp_bps0_dn8 = assign84680_body40_e130091_d_n8;
            locals.var_exp_bps0_dn9 = assign84680_body40_e130091_d_n9;
            locals.var_exp_bps0_dn10 = assign84680_body40_e130091_d_n10;
            locals.var_exp_bps0_dn13 = assign84680_body40_e130091_d_n13;
            let (assign84680_body41_e130111, assign84680_body41_e130111_d_n0, assign84680_body41_e130111_d_n2, assign84680_body41_e130111_d_n4, assign84680_body41_e130111_d_n5, assign84680_body41_e130111_d_n6, assign84680_body41_e130111_d_n7, assign84680_body41_e130111_d_n8, assign84680_body41_e130111_d_n9, assign84680_body41_e130111_d_n10, assign84680_body41_e130111_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1964 != 0.0)) && (locals.var_guard1971 == 0.0)) && (locals.var_guard1972 == 0.0)) {
        let assign84680_body41_e130106: f64 = (locals.var_chi + 1.0);
        let assign84680_body41_e130107: f64 = (locals.var_exp_bvbs * assign84680_body41_e130106);
        let assign84680_body41_e130108: f64 = (locals.var_exp_bps0 - assign84680_body41_e130107);
        let assign84680_body41_e130109: f64 = (locals.var_cnst1over * assign84680_body41_e130108);
        (assign84680_body41_e130109, ((locals.var_cnst1over_dn0 * assign84680_body41_e130108) + (locals.var_cnst1over * (locals.var_exp_bps0_dn0 - ((locals.var_exp_bvbs_dn0 * assign84680_body41_e130106) + (locals.var_exp_bvbs * locals.var_chi_dn0))))), ((locals.var_cnst1over_dn2 * assign84680_body41_e130108) + (locals.var_cnst1over * (locals.var_exp_bps0_dn2 - ((locals.var_exp_bvbs_dn2 * assign84680_body41_e130106) + (locals.var_exp_bvbs * locals.var_chi_dn2))))), ((locals.var_cnst1over_dn4 * assign84680_body41_e130108) + (locals.var_cnst1over * (locals.var_exp_bps0_dn4 - ((locals.var_exp_bvbs_dn4 * assign84680_body41_e130106) + (locals.var_exp_bvbs * locals.var_chi_dn4))))), ((locals.var_cnst1over_dn5 * assign84680_body41_e130108) + (locals.var_cnst1over * (locals.var_exp_bps0_dn5 - ((locals.var_exp_bvbs_dn5 * assign84680_body41_e130106) + (locals.var_exp_bvbs * locals.var_chi_dn5))))), ((locals.var_cnst1over_dn6 * assign84680_body41_e130108) + (locals.var_cnst1over * (locals.var_exp_bps0_dn6 - ((locals.var_exp_bvbs_dn6 * assign84680_body41_e130106) + (locals.var_exp_bvbs * locals.var_chi_dn6))))), ((locals.var_cnst1over_dn7 * assign84680_body41_e130108) + (locals.var_cnst1over * (locals.var_exp_bps0_dn7 - ((locals.var_exp_bvbs_dn7 * assign84680_body41_e130106) + (locals.var_exp_bvbs * locals.var_chi_dn7))))), ((locals.var_cnst1over_dn8 * assign84680_body41_e130108) + (locals.var_cnst1over * (locals.var_exp_bps0_dn8 - ((locals.var_exp_bvbs_dn8 * assign84680_body41_e130106) + (locals.var_exp_bvbs * locals.var_chi_dn8))))), ((locals.var_cnst1over_dn9 * assign84680_body41_e130108) + (locals.var_cnst1over * (locals.var_exp_bps0_dn9 - ((locals.var_exp_bvbs_dn9 * assign84680_body41_e130106) + (locals.var_exp_bvbs * locals.var_chi_dn9))))), ((locals.var_cnst1over_dn10 * assign84680_body41_e130108) + (locals.var_cnst1over * (locals.var_exp_bps0_dn10 - ((locals.var_exp_bvbs_dn10 * assign84680_body41_e130106) + (locals.var_exp_bvbs * locals.var_chi_dn10))))), ((locals.var_cnst1over_dn13 * assign84680_body41_e130108) + (locals.var_cnst1over * (locals.var_exp_bps0_dn13 - ((locals.var_exp_bvbs_dn13 * assign84680_body41_e130106) + (locals.var_exp_bvbs * locals.var_chi_dn13))))),)
    } else {
        (locals.var_fs01, locals.var_fs01_dn0, locals.var_fs01_dn2, locals.var_fs01_dn4, locals.var_fs01_dn5, locals.var_fs01_dn6, locals.var_fs01_dn7, locals.var_fs01_dn8, locals.var_fs01_dn9, locals.var_fs01_dn10, locals.var_fs01_dn13,)
    }
};
            locals.var_fs01 = assign84680_body41_e130111;
            locals.var_fs01_dn0 = assign84680_body41_e130111_d_n0;
            locals.var_fs01_dn2 = assign84680_body41_e130111_d_n2;
            locals.var_fs01_dn4 = assign84680_body41_e130111_d_n4;
            locals.var_fs01_dn5 = assign84680_body41_e130111_d_n5;
            locals.var_fs01_dn6 = assign84680_body41_e130111_d_n6;
            locals.var_fs01_dn7 = assign84680_body41_e130111_d_n7;
            locals.var_fs01_dn8 = assign84680_body41_e130111_d_n8;
            locals.var_fs01_dn9 = assign84680_body41_e130111_d_n9;
            locals.var_fs01_dn10 = assign84680_body41_e130111_d_n10;
            locals.var_fs01_dn13 = assign84680_body41_e130111_d_n13;
            let (assign84680_body42_e130129, assign84680_body42_e130129_d_n0, assign84680_body42_e130129_d_n2, assign84680_body42_e130129_d_n4, assign84680_body42_e130129_d_n5, assign84680_body42_e130129_d_n6, assign84680_body42_e130129_d_n7, assign84680_body42_e130129_d_n8, assign84680_body42_e130129_d_n9, assign84680_body42_e130129_d_n10, assign84680_body42_e130129_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1964 != 0.0)) && (locals.var_guard1971 == 0.0)) && (locals.var_guard1972 == 0.0)) {
        let assign84680_body42_e130123: f64 = (locals.var_cnst1over * locals.var_beta);
        let assign84680_body42_e130126: f64 = (locals.var_exp_bps0 - locals.var_exp_bvbs);
        let assign84680_body42_e130127: f64 = (assign84680_body42_e130123 * assign84680_body42_e130126);
        (assign84680_body42_e130127, ((((locals.var_cnst1over_dn0 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn0)) * assign84680_body42_e130126) + (assign84680_body42_e130123 * (locals.var_exp_bps0_dn0 - locals.var_exp_bvbs_dn0))), ((((locals.var_cnst1over_dn2 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn2)) * assign84680_body42_e130126) + (assign84680_body42_e130123 * (locals.var_exp_bps0_dn2 - locals.var_exp_bvbs_dn2))), ((((locals.var_cnst1over_dn4 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn4)) * assign84680_body42_e130126) + (assign84680_body42_e130123 * (locals.var_exp_bps0_dn4 - locals.var_exp_bvbs_dn4))), ((((locals.var_cnst1over_dn5 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn5)) * assign84680_body42_e130126) + (assign84680_body42_e130123 * (locals.var_exp_bps0_dn5 - locals.var_exp_bvbs_dn5))), ((((locals.var_cnst1over_dn6 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn6)) * assign84680_body42_e130126) + (assign84680_body42_e130123 * (locals.var_exp_bps0_dn6 - locals.var_exp_bvbs_dn6))), ((((locals.var_cnst1over_dn7 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn7)) * assign84680_body42_e130126) + (assign84680_body42_e130123 * (locals.var_exp_bps0_dn7 - locals.var_exp_bvbs_dn7))), ((((locals.var_cnst1over_dn8 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn8)) * assign84680_body42_e130126) + (assign84680_body42_e130123 * (locals.var_exp_bps0_dn8 - locals.var_exp_bvbs_dn8))), ((((locals.var_cnst1over_dn9 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn9)) * assign84680_body42_e130126) + (assign84680_body42_e130123 * (locals.var_exp_bps0_dn9 - locals.var_exp_bvbs_dn9))), ((((locals.var_cnst1over_dn10 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn10)) * assign84680_body42_e130126) + (assign84680_body42_e130123 * (locals.var_exp_bps0_dn10 - locals.var_exp_bvbs_dn10))), ((((locals.var_cnst1over_dn13 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn13)) * assign84680_body42_e130126) + (assign84680_body42_e130123 * (locals.var_exp_bps0_dn13 - locals.var_exp_bvbs_dn13))),)
    } else {
        (locals.var_fs01_dps0, locals.var_fs01_dps0_dn0, locals.var_fs01_dps0_dn2, locals.var_fs01_dps0_dn4, locals.var_fs01_dps0_dn5, locals.var_fs01_dps0_dn6, locals.var_fs01_dps0_dn7, locals.var_fs01_dps0_dn8, locals.var_fs01_dps0_dn9, locals.var_fs01_dps0_dn10, locals.var_fs01_dps0_dn13,)
    }
};
            locals.var_fs01_dps0 = assign84680_body42_e130129;
            locals.var_fs01_dps0_dn0 = assign84680_body42_e130129_d_n0;
            locals.var_fs01_dps0_dn2 = assign84680_body42_e130129_d_n2;
            locals.var_fs01_dps0_dn4 = assign84680_body42_e130129_d_n4;
            locals.var_fs01_dps0_dn5 = assign84680_body42_e130129_d_n5;
            locals.var_fs01_dps0_dn6 = assign84680_body42_e130129_d_n6;
            locals.var_fs01_dps0_dn7 = assign84680_body42_e130129_d_n7;
            locals.var_fs01_dps0_dn8 = assign84680_body42_e130129_d_n8;
            locals.var_fs01_dps0_dn9 = assign84680_body42_e130129_d_n9;
            locals.var_fs01_dps0_dn10 = assign84680_body42_e130129_d_n10;
            locals.var_fs01_dps0_dn13 = assign84680_body42_e130129_d_n13;
            let assign84680_body43_e130132: f64 = if locals.var_fs01 > 0.0 { 1.0 } else { 0.0 };
            locals.var_guard1973 = assign84680_body43_e130132;
            let (assign84680_body44_e130143, assign84680_body44_e130143_d_n0, assign84680_body44_e130143_d_n2, assign84680_body44_e130143_d_n4, assign84680_body44_e130143_d_n5, assign84680_body44_e130143_d_n6, assign84680_body44_e130143_d_n7, assign84680_body44_e130143_d_n8, assign84680_body44_e130143_d_n9, assign84680_body44_e130143_d_n10, assign84680_body44_e130143_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1964 != 0.0)) && (locals.var_guard1973 != 0.0)) {
        let assign84680_body44_e130140: f64 = (locals.var_fbsq__blk1891 + locals.var_fs01);
        let assign84680_body44_e130141: f64 = (assign84680_body44_e130140).sqrt();
        (assign84680_body44_e130141, ((locals.var_fbsq__blk1891_dn0 + locals.var_fs01_dn0) / (2.0 * assign84680_body44_e130141)), ((locals.var_fbsq__blk1891_dn2 + locals.var_fs01_dn2) / (2.0 * assign84680_body44_e130141)), ((locals.var_fbsq__blk1891_dn4 + locals.var_fs01_dn4) / (2.0 * assign84680_body44_e130141)), ((locals.var_fbsq__blk1891_dn5 + locals.var_fs01_dn5) / (2.0 * assign84680_body44_e130141)), ((locals.var_fbsq__blk1891_dn6 + locals.var_fs01_dn6) / (2.0 * assign84680_body44_e130141)), ((locals.var_fbsq__blk1891_dn7 + locals.var_fs01_dn7) / (2.0 * assign84680_body44_e130141)), ((locals.var_fbsq__blk1891_dn8 + locals.var_fs01_dn8) / (2.0 * assign84680_body44_e130141)), ((locals.var_fbsq__blk1891_dn9 + locals.var_fs01_dn9) / (2.0 * assign84680_body44_e130141)), ((locals.var_fbsq__blk1891_dn10 + locals.var_fs01_dn10) / (2.0 * assign84680_body44_e130141)), ((locals.var_fbsq__blk1891_dn13 + locals.var_fs01_dn13) / (2.0 * assign84680_body44_e130141)),)
    } else {
        (locals.var_fs02, locals.var_fs02_dn0, locals.var_fs02_dn2, locals.var_fs02_dn4, locals.var_fs02_dn5, locals.var_fs02_dn6, locals.var_fs02_dn7, locals.var_fs02_dn8, locals.var_fs02_dn9, locals.var_fs02_dn10, locals.var_fs02_dn13,)
    }
};
            locals.var_fs02 = assign84680_body44_e130143;
            locals.var_fs02_dn0 = assign84680_body44_e130143_d_n0;
            locals.var_fs02_dn2 = assign84680_body44_e130143_d_n2;
            locals.var_fs02_dn4 = assign84680_body44_e130143_d_n4;
            locals.var_fs02_dn5 = assign84680_body44_e130143_d_n5;
            locals.var_fs02_dn6 = assign84680_body44_e130143_d_n6;
            locals.var_fs02_dn7 = assign84680_body44_e130143_d_n7;
            locals.var_fs02_dn8 = assign84680_body44_e130143_d_n8;
            locals.var_fs02_dn9 = assign84680_body44_e130143_d_n9;
            locals.var_fs02_dn10 = assign84680_body44_e130143_d_n10;
            locals.var_fs02_dn13 = assign84680_body44_e130143_d_n13;
            let (assign84680_body45_e130157, assign84680_body45_e130157_d_n0, assign84680_body45_e130157_d_n2, assign84680_body45_e130157_d_n4, assign84680_body45_e130157_d_n5, assign84680_body45_e130157_d_n6, assign84680_body45_e130157_d_n7, assign84680_body45_e130157_d_n8, assign84680_body45_e130157_d_n9, assign84680_body45_e130157_d_n10, assign84680_body45_e130157_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1964 != 0.0)) && (locals.var_guard1973 != 0.0)) {
        let assign84680_body45_e130152: f64 = (locals.var_fbsq_dpss__blk1892 + locals.var_fs01_dps0);
        let assign84680_body45_e130153: f64 = (0.5 * assign84680_body45_e130152);
        let assign84680_body45_e130155: f64 = (assign84680_body45_e130153 / locals.var_fs02);
        (assign84680_body45_e130155, ((((0.5 * (locals.var_fbsq_dpss__blk1892_dn0 + locals.var_fs01_dps0_dn0)) * locals.var_fs02) - (assign84680_body45_e130153 * locals.var_fs02_dn0)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * (locals.var_fbsq_dpss__blk1892_dn2 + locals.var_fs01_dps0_dn2)) * locals.var_fs02) - (assign84680_body45_e130153 * locals.var_fs02_dn2)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * (locals.var_fbsq_dpss__blk1892_dn4 + locals.var_fs01_dps0_dn4)) * locals.var_fs02) - (assign84680_body45_e130153 * locals.var_fs02_dn4)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * (locals.var_fbsq_dpss__blk1892_dn5 + locals.var_fs01_dps0_dn5)) * locals.var_fs02) - (assign84680_body45_e130153 * locals.var_fs02_dn5)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * (locals.var_fbsq_dpss__blk1892_dn6 + locals.var_fs01_dps0_dn6)) * locals.var_fs02) - (assign84680_body45_e130153 * locals.var_fs02_dn6)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * (locals.var_fbsq_dpss__blk1892_dn7 + locals.var_fs01_dps0_dn7)) * locals.var_fs02) - (assign84680_body45_e130153 * locals.var_fs02_dn7)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * (locals.var_fbsq_dpss__blk1892_dn8 + locals.var_fs01_dps0_dn8)) * locals.var_fs02) - (assign84680_body45_e130153 * locals.var_fs02_dn8)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * (locals.var_fbsq_dpss__blk1892_dn9 + locals.var_fs01_dps0_dn9)) * locals.var_fs02) - (assign84680_body45_e130153 * locals.var_fs02_dn9)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * (locals.var_fbsq_dpss__blk1892_dn10 + locals.var_fs01_dps0_dn10)) * locals.var_fs02) - (assign84680_body45_e130153 * locals.var_fs02_dn10)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * (locals.var_fbsq_dpss__blk1892_dn13 + locals.var_fs01_dps0_dn13)) * locals.var_fs02) - (assign84680_body45_e130153 * locals.var_fs02_dn13)) / (locals.var_fs02 * locals.var_fs02)),)
    } else {
        (locals.var_fs02_dps0, locals.var_fs02_dps0_dn0, locals.var_fs02_dps0_dn2, locals.var_fs02_dps0_dn4, locals.var_fs02_dps0_dn5, locals.var_fs02_dps0_dn6, locals.var_fs02_dps0_dn7, locals.var_fs02_dps0_dn8, locals.var_fs02_dps0_dn9, locals.var_fs02_dps0_dn10, locals.var_fs02_dps0_dn13,)
    }
};
            locals.var_fs02_dps0 = assign84680_body45_e130157;
            locals.var_fs02_dps0_dn0 = assign84680_body45_e130157_d_n0;
            locals.var_fs02_dps0_dn2 = assign84680_body45_e130157_d_n2;
            locals.var_fs02_dps0_dn4 = assign84680_body45_e130157_d_n4;
            locals.var_fs02_dps0_dn5 = assign84680_body45_e130157_d_n5;
            locals.var_fs02_dps0_dn6 = assign84680_body45_e130157_d_n6;
            locals.var_fs02_dps0_dn7 = assign84680_body45_e130157_d_n7;
            locals.var_fs02_dps0_dn8 = assign84680_body45_e130157_d_n8;
            locals.var_fs02_dps0_dn9 = assign84680_body45_e130157_d_n9;
            locals.var_fs02_dps0_dn10 = assign84680_body45_e130157_d_n10;
            locals.var_fs02_dps0_dn13 = assign84680_body45_e130157_d_n13;
            let assign84680_body46_e130160: f64 = if locals.var_fbsq__blk1891 > 0.0 { 1.0 } else { 0.0 };
            locals.var_guard1974 = assign84680_body46_e130160;
            let (assign84680_body47_e130172, assign84680_body47_e130172_d_n0, assign84680_body47_e130172_d_n2, assign84680_body47_e130172_d_n4, assign84680_body47_e130172_d_n5, assign84680_body47_e130172_d_n6, assign84680_body47_e130172_d_n7, assign84680_body47_e130172_d_n8, assign84680_body47_e130172_d_n9, assign84680_body47_e130172_d_n10, assign84680_body47_e130172_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1964 != 0.0)) && (locals.var_guard1973 == 0.0)) && (locals.var_guard1974 != 0.0)) {
        let assign84680_body47_e130170: f64 = (locals.var_fbsq__blk1891).sqrt();
        (assign84680_body47_e130170, (locals.var_fbsq__blk1891_dn0 / (2.0 * assign84680_body47_e130170)), (locals.var_fbsq__blk1891_dn2 / (2.0 * assign84680_body47_e130170)), (locals.var_fbsq__blk1891_dn4 / (2.0 * assign84680_body47_e130170)), (locals.var_fbsq__blk1891_dn5 / (2.0 * assign84680_body47_e130170)), (locals.var_fbsq__blk1891_dn6 / (2.0 * assign84680_body47_e130170)), (locals.var_fbsq__blk1891_dn7 / (2.0 * assign84680_body47_e130170)), (locals.var_fbsq__blk1891_dn8 / (2.0 * assign84680_body47_e130170)), (locals.var_fbsq__blk1891_dn9 / (2.0 * assign84680_body47_e130170)), (locals.var_fbsq__blk1891_dn10 / (2.0 * assign84680_body47_e130170)), (locals.var_fbsq__blk1891_dn13 / (2.0 * assign84680_body47_e130170)),)
    } else {
        (locals.var_fs02, locals.var_fs02_dn0, locals.var_fs02_dn2, locals.var_fs02_dn4, locals.var_fs02_dn5, locals.var_fs02_dn6, locals.var_fs02_dn7, locals.var_fs02_dn8, locals.var_fs02_dn9, locals.var_fs02_dn10, locals.var_fs02_dn13,)
    }
};
            locals.var_fs02 = assign84680_body47_e130172;
            locals.var_fs02_dn0 = assign84680_body47_e130172_d_n0;
            locals.var_fs02_dn2 = assign84680_body47_e130172_d_n2;
            locals.var_fs02_dn4 = assign84680_body47_e130172_d_n4;
            locals.var_fs02_dn5 = assign84680_body47_e130172_d_n5;
            locals.var_fs02_dn6 = assign84680_body47_e130172_d_n6;
            locals.var_fs02_dn7 = assign84680_body47_e130172_d_n7;
            locals.var_fs02_dn8 = assign84680_body47_e130172_d_n8;
            locals.var_fs02_dn9 = assign84680_body47_e130172_d_n9;
            locals.var_fs02_dn10 = assign84680_body47_e130172_d_n10;
            locals.var_fs02_dn13 = assign84680_body47_e130172_d_n13;
            let (assign84680_body48_e130187, assign84680_body48_e130187_d_n0, assign84680_body48_e130187_d_n2, assign84680_body48_e130187_d_n4, assign84680_body48_e130187_d_n5, assign84680_body48_e130187_d_n6, assign84680_body48_e130187_d_n7, assign84680_body48_e130187_d_n8, assign84680_body48_e130187_d_n9, assign84680_body48_e130187_d_n10, assign84680_body48_e130187_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1964 != 0.0)) && (locals.var_guard1973 == 0.0)) && (locals.var_guard1974 != 0.0)) {
        let assign84680_body48_e130183: f64 = (0.5 * locals.var_fbsq_dpss__blk1892);
        let assign84680_body48_e130185: f64 = (assign84680_body48_e130183 / locals.var_fs02);
        (assign84680_body48_e130185, ((((0.5 * locals.var_fbsq_dpss__blk1892_dn0) * locals.var_fs02) - (assign84680_body48_e130183 * locals.var_fs02_dn0)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * locals.var_fbsq_dpss__blk1892_dn2) * locals.var_fs02) - (assign84680_body48_e130183 * locals.var_fs02_dn2)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * locals.var_fbsq_dpss__blk1892_dn4) * locals.var_fs02) - (assign84680_body48_e130183 * locals.var_fs02_dn4)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * locals.var_fbsq_dpss__blk1892_dn5) * locals.var_fs02) - (assign84680_body48_e130183 * locals.var_fs02_dn5)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * locals.var_fbsq_dpss__blk1892_dn6) * locals.var_fs02) - (assign84680_body48_e130183 * locals.var_fs02_dn6)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * locals.var_fbsq_dpss__blk1892_dn7) * locals.var_fs02) - (assign84680_body48_e130183 * locals.var_fs02_dn7)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * locals.var_fbsq_dpss__blk1892_dn8) * locals.var_fs02) - (assign84680_body48_e130183 * locals.var_fs02_dn8)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * locals.var_fbsq_dpss__blk1892_dn9) * locals.var_fs02) - (assign84680_body48_e130183 * locals.var_fs02_dn9)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * locals.var_fbsq_dpss__blk1892_dn10) * locals.var_fs02) - (assign84680_body48_e130183 * locals.var_fs02_dn10)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * locals.var_fbsq_dpss__blk1892_dn13) * locals.var_fs02) - (assign84680_body48_e130183 * locals.var_fs02_dn13)) / (locals.var_fs02 * locals.var_fs02)),)
    } else {
        (locals.var_fs02_dps0, locals.var_fs02_dps0_dn0, locals.var_fs02_dps0_dn2, locals.var_fs02_dps0_dn4, locals.var_fs02_dps0_dn5, locals.var_fs02_dps0_dn6, locals.var_fs02_dps0_dn7, locals.var_fs02_dps0_dn8, locals.var_fs02_dps0_dn9, locals.var_fs02_dps0_dn10, locals.var_fs02_dps0_dn13,)
    }
};
            locals.var_fs02_dps0 = assign84680_body48_e130187;
            locals.var_fs02_dps0_dn0 = assign84680_body48_e130187_d_n0;
            locals.var_fs02_dps0_dn2 = assign84680_body48_e130187_d_n2;
            locals.var_fs02_dps0_dn4 = assign84680_body48_e130187_d_n4;
            locals.var_fs02_dps0_dn5 = assign84680_body48_e130187_d_n5;
            locals.var_fs02_dps0_dn6 = assign84680_body48_e130187_d_n6;
            locals.var_fs02_dps0_dn7 = assign84680_body48_e130187_d_n7;
            locals.var_fs02_dps0_dn8 = assign84680_body48_e130187_d_n8;
            locals.var_fs02_dps0_dn9 = assign84680_body48_e130187_d_n9;
            locals.var_fs02_dps0_dn10 = assign84680_body48_e130187_d_n10;
            locals.var_fs02_dps0_dn13 = assign84680_body48_e130187_d_n13;
            let (assign84680_body49_e130199, assign84680_body49_e130199_d_n0, assign84680_body49_e130199_d_n2, assign84680_body49_e130199_d_n4, assign84680_body49_e130199_d_n5, assign84680_body49_e130199_d_n6, assign84680_body49_e130199_d_n7, assign84680_body49_e130199_d_n8, assign84680_body49_e130199_d_n9, assign84680_body49_e130199_d_n10, assign84680_body49_e130199_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1964 != 0.0)) && (locals.var_guard1973 == 0.0)) && (locals.var_guard1974 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fs02, locals.var_fs02_dn0, locals.var_fs02_dn2, locals.var_fs02_dn4, locals.var_fs02_dn5, locals.var_fs02_dn6, locals.var_fs02_dn7, locals.var_fs02_dn8, locals.var_fs02_dn9, locals.var_fs02_dn10, locals.var_fs02_dn13,)
    }
};
            locals.var_fs02 = assign84680_body49_e130199;
            locals.var_fs02_dn0 = assign84680_body49_e130199_d_n0;
            locals.var_fs02_dn2 = assign84680_body49_e130199_d_n2;
            locals.var_fs02_dn4 = assign84680_body49_e130199_d_n4;
            locals.var_fs02_dn5 = assign84680_body49_e130199_d_n5;
            locals.var_fs02_dn6 = assign84680_body49_e130199_d_n6;
            locals.var_fs02_dn7 = assign84680_body49_e130199_d_n7;
            locals.var_fs02_dn8 = assign84680_body49_e130199_d_n8;
            locals.var_fs02_dn9 = assign84680_body49_e130199_d_n9;
            locals.var_fs02_dn10 = assign84680_body49_e130199_d_n10;
            locals.var_fs02_dn13 = assign84680_body49_e130199_d_n13;
            let (assign84680_body50_e130211, assign84680_body50_e130211_d_n0, assign84680_body50_e130211_d_n2, assign84680_body50_e130211_d_n4, assign84680_body50_e130211_d_n5, assign84680_body50_e130211_d_n6, assign84680_body50_e130211_d_n7, assign84680_body50_e130211_d_n8, assign84680_body50_e130211_d_n9, assign84680_body50_e130211_d_n10, assign84680_body50_e130211_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1964 != 0.0)) && (locals.var_guard1973 == 0.0)) && (locals.var_guard1974 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fs02_dps0, locals.var_fs02_dps0_dn0, locals.var_fs02_dps0_dn2, locals.var_fs02_dps0_dn4, locals.var_fs02_dps0_dn5, locals.var_fs02_dps0_dn6, locals.var_fs02_dps0_dn7, locals.var_fs02_dps0_dn8, locals.var_fs02_dps0_dn9, locals.var_fs02_dps0_dn10, locals.var_fs02_dps0_dn13,)
    }
};
            locals.var_fs02_dps0 = assign84680_body50_e130211;
            locals.var_fs02_dps0_dn0 = assign84680_body50_e130211_d_n0;
            locals.var_fs02_dps0_dn2 = assign84680_body50_e130211_d_n2;
            locals.var_fs02_dps0_dn4 = assign84680_body50_e130211_d_n4;
            locals.var_fs02_dps0_dn5 = assign84680_body50_e130211_d_n5;
            locals.var_fs02_dps0_dn6 = assign84680_body50_e130211_d_n6;
            locals.var_fs02_dps0_dn7 = assign84680_body50_e130211_d_n7;
            locals.var_fs02_dps0_dn8 = assign84680_body50_e130211_d_n8;
            locals.var_fs02_dps0_dn9 = assign84680_body50_e130211_d_n9;
            locals.var_fs02_dps0_dn10 = assign84680_body50_e130211_d_n10;
            locals.var_fs02_dps0_dn13 = assign84680_body50_e130211_d_n13;
            let (assign84680_body51_e130225, assign84680_body51_e130225_d_n0, assign84680_body51_e130225_d_n2, assign84680_body51_e130225_d_n4, assign84680_body51_e130225_d_n5, assign84680_body51_e130225_d_n6, assign84680_body51_e130225_d_n7, assign84680_body51_e130225_d_n8, assign84680_body51_e130225_d_n9, assign84680_body51_e130225_d_n10, assign84680_body51_e130225_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1964 != 0.0)) {
        let (assign84680_body51_e130221,) = {
            if (locals.var_chi >= 0.0) {
                (1.0,)
            } else {
                let assign84680_body51_e130220: f64 = (-1.0);
                (assign84680_body51_e130220,)
            }
        };
        let assign84680_body51_e130223: f64 = (assign84680_body51_e130221 * locals.var_fs02);
        (assign84680_body51_e130223, (assign84680_body51_e130221 * locals.var_fs02_dn0), (assign84680_body51_e130221 * locals.var_fs02_dn2), (assign84680_body51_e130221 * locals.var_fs02_dn4), (assign84680_body51_e130221 * locals.var_fs02_dn5), (assign84680_body51_e130221 * locals.var_fs02_dn6), (assign84680_body51_e130221 * locals.var_fs02_dn7), (assign84680_body51_e130221 * locals.var_fs02_dn8), (assign84680_body51_e130221 * locals.var_fs02_dn9), (assign84680_body51_e130221 * locals.var_fs02_dn10), (assign84680_body51_e130221 * locals.var_fs02_dn13),)
    } else {
        (locals.var_fs02, locals.var_fs02_dn0, locals.var_fs02_dn2, locals.var_fs02_dn4, locals.var_fs02_dn5, locals.var_fs02_dn6, locals.var_fs02_dn7, locals.var_fs02_dn8, locals.var_fs02_dn9, locals.var_fs02_dn10, locals.var_fs02_dn13,)
    }
};
            locals.var_fs02 = assign84680_body51_e130225;
            locals.var_fs02_dn0 = assign84680_body51_e130225_d_n0;
            locals.var_fs02_dn2 = assign84680_body51_e130225_d_n2;
            locals.var_fs02_dn4 = assign84680_body51_e130225_d_n4;
            locals.var_fs02_dn5 = assign84680_body51_e130225_d_n5;
            locals.var_fs02_dn6 = assign84680_body51_e130225_d_n6;
            locals.var_fs02_dn7 = assign84680_body51_e130225_d_n7;
            locals.var_fs02_dn8 = assign84680_body51_e130225_d_n8;
            locals.var_fs02_dn9 = assign84680_body51_e130225_d_n9;
            locals.var_fs02_dn10 = assign84680_body51_e130225_d_n10;
            locals.var_fs02_dn13 = assign84680_body51_e130225_d_n13;
            let (assign84680_body52_e130239, assign84680_body52_e130239_d_n0, assign84680_body52_e130239_d_n2, assign84680_body52_e130239_d_n4, assign84680_body52_e130239_d_n5, assign84680_body52_e130239_d_n6, assign84680_body52_e130239_d_n7, assign84680_body52_e130239_d_n8, assign84680_body52_e130239_d_n9, assign84680_body52_e130239_d_n10, assign84680_body52_e130239_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1964 != 0.0)) {
        let (assign84680_body52_e130235,) = {
            if (locals.var_chi >= 0.0) {
                (1.0,)
            } else {
                let assign84680_body52_e130234: f64 = (-1.0);
                (assign84680_body52_e130234,)
            }
        };
        let assign84680_body52_e130237: f64 = (assign84680_body52_e130235 * locals.var_fs02_dps0);
        (assign84680_body52_e130237, (assign84680_body52_e130235 * locals.var_fs02_dps0_dn0), (assign84680_body52_e130235 * locals.var_fs02_dps0_dn2), (assign84680_body52_e130235 * locals.var_fs02_dps0_dn4), (assign84680_body52_e130235 * locals.var_fs02_dps0_dn5), (assign84680_body52_e130235 * locals.var_fs02_dps0_dn6), (assign84680_body52_e130235 * locals.var_fs02_dps0_dn7), (assign84680_body52_e130235 * locals.var_fs02_dps0_dn8), (assign84680_body52_e130235 * locals.var_fs02_dps0_dn9), (assign84680_body52_e130235 * locals.var_fs02_dps0_dn10), (assign84680_body52_e130235 * locals.var_fs02_dps0_dn13),)
    } else {
        (locals.var_fs02_dps0, locals.var_fs02_dps0_dn0, locals.var_fs02_dps0_dn2, locals.var_fs02_dps0_dn4, locals.var_fs02_dps0_dn5, locals.var_fs02_dps0_dn6, locals.var_fs02_dps0_dn7, locals.var_fs02_dps0_dn8, locals.var_fs02_dps0_dn9, locals.var_fs02_dps0_dn10, locals.var_fs02_dps0_dn13,)
    }
};
            locals.var_fs02_dps0 = assign84680_body52_e130239;
            locals.var_fs02_dps0_dn0 = assign84680_body52_e130239_d_n0;
            locals.var_fs02_dps0_dn2 = assign84680_body52_e130239_d_n2;
            locals.var_fs02_dps0_dn4 = assign84680_body52_e130239_d_n4;
            locals.var_fs02_dps0_dn5 = assign84680_body52_e130239_d_n5;
            locals.var_fs02_dps0_dn6 = assign84680_body52_e130239_d_n6;
            locals.var_fs02_dps0_dn7 = assign84680_body52_e130239_d_n7;
            locals.var_fs02_dps0_dn8 = assign84680_body52_e130239_d_n8;
            locals.var_fs02_dps0_dn9 = assign84680_body52_e130239_d_n9;
            locals.var_fs02_dps0_dn10 = assign84680_body52_e130239_d_n10;
            locals.var_fs02_dps0_dn13 = assign84680_body52_e130239_d_n13;
            let (assign84680_body53_e130252, assign84680_body53_e130252_d_n0, assign84680_body53_e130252_d_n2, assign84680_body53_e130252_d_n4, assign84680_body53_e130252_d_n5, assign84680_body53_e130252_d_n6, assign84680_body53_e130252_d_n7, assign84680_body53_e130252_d_n8, assign84680_body53_e130252_d_n9, assign84680_body53_e130252_d_n10, assign84680_body53_e130252_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1964 != 0.0)) {
        let assign84680_body53_e130244: f64 = (-locals.var_vgpld);
        let assign84680_body53_e130246: f64 = (assign84680_body53_e130244 + locals.var_ps0ld);
        let assign84680_body53_e130249: f64 = (locals.var_fac1 * locals.var_fs02);
        let assign84680_body53_e130250: f64 = (assign84680_body53_e130246 + assign84680_body53_e130249);
        (assign84680_body53_e130250, (locals.var_ps0ld_dn0 + ((locals.var_fac1_dn0 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn0))), (((-locals.var_vgpld_dn2) + locals.var_ps0ld_dn2) + ((locals.var_fac1_dn2 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn2))), (locals.var_ps0ld_dn4 + ((locals.var_fac1_dn4 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn4))), (locals.var_ps0ld_dn5 + ((locals.var_fac1_dn5 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn5))), (((-locals.var_vgpld_dn6) + locals.var_ps0ld_dn6) + ((locals.var_fac1_dn6 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn6))), (((-locals.var_vgpld_dn7) + locals.var_ps0ld_dn7) + ((locals.var_fac1_dn7 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn7))), (((-locals.var_vgpld_dn8) + locals.var_ps0ld_dn8) + ((locals.var_fac1_dn8 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn8))), (locals.var_ps0ld_dn9 + ((locals.var_fac1_dn9 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn9))), (locals.var_ps0ld_dn10 + ((locals.var_fac1_dn10 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn10))), (locals.var_ps0ld_dn13 + ((locals.var_fac1_dn13 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn13))),)
    } else {
        (locals.var_fs0, locals.var_fs0_dn0, locals.var_fs0_dn2, locals.var_fs0_dn4, locals.var_fs0_dn5, locals.var_fs0_dn6, locals.var_fs0_dn7, locals.var_fs0_dn8, locals.var_fs0_dn9, locals.var_fs0_dn10, locals.var_fs0_dn13,)
    }
};
            locals.var_fs0 = assign84680_body53_e130252;
            locals.var_fs0_dn0 = assign84680_body53_e130252_d_n0;
            locals.var_fs0_dn2 = assign84680_body53_e130252_d_n2;
            locals.var_fs0_dn4 = assign84680_body53_e130252_d_n4;
            locals.var_fs0_dn5 = assign84680_body53_e130252_d_n5;
            locals.var_fs0_dn6 = assign84680_body53_e130252_d_n6;
            locals.var_fs0_dn7 = assign84680_body53_e130252_d_n7;
            locals.var_fs0_dn8 = assign84680_body53_e130252_d_n8;
            locals.var_fs0_dn9 = assign84680_body53_e130252_d_n9;
            locals.var_fs0_dn10 = assign84680_body53_e130252_d_n10;
            locals.var_fs0_dn13 = assign84680_body53_e130252_d_n13;
            let (assign84680_body54_e130262, assign84680_body54_e130262_d_n0, assign84680_body54_e130262_d_n2, assign84680_body54_e130262_d_n4, assign84680_body54_e130262_d_n5, assign84680_body54_e130262_d_n6, assign84680_body54_e130262_d_n7, assign84680_body54_e130262_d_n8, assign84680_body54_e130262_d_n9, assign84680_body54_e130262_d_n10, assign84680_body54_e130262_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1964 != 0.0)) {
        let assign84680_body54_e130259: f64 = (locals.var_fac1 * locals.var_fs02_dps0);
        let assign84680_body54_e130260: f64 = (1.0 + assign84680_body54_e130259);
        (assign84680_body54_e130260, ((locals.var_fac1_dn0 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn0)), ((locals.var_fac1_dn2 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn2)), ((locals.var_fac1_dn4 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn4)), ((locals.var_fac1_dn5 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn5)), ((locals.var_fac1_dn6 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn6)), ((locals.var_fac1_dn7 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn7)), ((locals.var_fac1_dn8 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn8)), ((locals.var_fac1_dn9 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn9)), ((locals.var_fac1_dn10 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn10)), ((locals.var_fac1_dn13 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn13)),)
    } else {
        (locals.var_fs0_dps0, locals.var_fs0_dps0_dn0, locals.var_fs0_dps0_dn2, locals.var_fs0_dps0_dn4, locals.var_fs0_dps0_dn5, locals.var_fs0_dps0_dn6, locals.var_fs0_dps0_dn7, locals.var_fs0_dps0_dn8, locals.var_fs0_dps0_dn9, locals.var_fs0_dps0_dn10, locals.var_fs0_dps0_dn13,)
    }
};
            locals.var_fs0_dps0 = assign84680_body54_e130262;
            locals.var_fs0_dps0_dn0 = assign84680_body54_e130262_d_n0;
            locals.var_fs0_dps0_dn2 = assign84680_body54_e130262_d_n2;
            locals.var_fs0_dps0_dn4 = assign84680_body54_e130262_d_n4;
            locals.var_fs0_dps0_dn5 = assign84680_body54_e130262_d_n5;
            locals.var_fs0_dps0_dn6 = assign84680_body54_e130262_d_n6;
            locals.var_fs0_dps0_dn7 = assign84680_body54_e130262_d_n7;
            locals.var_fs0_dps0_dn8 = assign84680_body54_e130262_d_n8;
            locals.var_fs0_dps0_dn9 = assign84680_body54_e130262_d_n9;
            locals.var_fs0_dps0_dn10 = assign84680_body54_e130262_d_n10;
            locals.var_fs0_dps0_dn13 = assign84680_body54_e130262_d_n13;
            let assign84680_body55_e130265: f64 = if locals.var_flg_conv > 0.0 { 1.0 } else { 0.0 };
            locals.var_guard1975 = assign84680_body55_e130265;
            let (assign84680_body56_e130275,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1964 != 0.0)) && (locals.var_guard1975 != 0.0)) {
        let assign84680_body56_e130273: f64 = (locals.var_lp_s0_max + 1.0);
        (assign84680_body56_e130273,)
    } else {
        (locals.var_lp_s0,)
    }
};
            locals.var_lp_s0 = assign84680_body56_e130275;
            let (assign84680_body57_e130287, assign84680_body57_e130287_d_n0, assign84680_body57_e130287_d_n2, assign84680_body57_e130287_d_n4, assign84680_body57_e130287_d_n5, assign84680_body57_e130287_d_n6, assign84680_body57_e130287_d_n7, assign84680_body57_e130287_d_n8, assign84680_body57_e130287_d_n9, assign84680_body57_e130287_d_n10, assign84680_body57_e130287_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1964 != 0.0)) && (locals.var_guard1975 == 0.0)) {
        let assign84680_body57_e130283: f64 = (-locals.var_fs0);
        let assign84680_body57_e130285: f64 = (assign84680_body57_e130283 / locals.var_fs0_dps0);
        (assign84680_body57_e130285, ((((-locals.var_fs0_dn0) * locals.var_fs0_dps0) - (assign84680_body57_e130283 * locals.var_fs0_dps0_dn0)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn2) * locals.var_fs0_dps0) - (assign84680_body57_e130283 * locals.var_fs0_dps0_dn2)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn4) * locals.var_fs0_dps0) - (assign84680_body57_e130283 * locals.var_fs0_dps0_dn4)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn5) * locals.var_fs0_dps0) - (assign84680_body57_e130283 * locals.var_fs0_dps0_dn5)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn6) * locals.var_fs0_dps0) - (assign84680_body57_e130283 * locals.var_fs0_dps0_dn6)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn7) * locals.var_fs0_dps0) - (assign84680_body57_e130283 * locals.var_fs0_dps0_dn7)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn8) * locals.var_fs0_dps0) - (assign84680_body57_e130283 * locals.var_fs0_dps0_dn8)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn9) * locals.var_fs0_dps0) - (assign84680_body57_e130283 * locals.var_fs0_dps0_dn9)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn10) * locals.var_fs0_dps0) - (assign84680_body57_e130283 * locals.var_fs0_dps0_dn10)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn13) * locals.var_fs0_dps0) - (assign84680_body57_e130283 * locals.var_fs0_dps0_dn13)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)),)
    } else {
        (locals.var_dps0, locals.var_dps0_dn0, locals.var_dps0_dn2, locals.var_dps0_dn4, locals.var_dps0_dn5, locals.var_dps0_dn6, locals.var_dps0_dn7, locals.var_dps0_dn8, locals.var_dps0_dn9, locals.var_dps0_dn10, locals.var_dps0_dn13,)
    }
};
            locals.var_dps0 = assign84680_body57_e130287;
            locals.var_dps0_dn0 = assign84680_body57_e130287_d_n0;
            locals.var_dps0_dn2 = assign84680_body57_e130287_d_n2;
            locals.var_dps0_dn4 = assign84680_body57_e130287_d_n4;
            locals.var_dps0_dn5 = assign84680_body57_e130287_d_n5;
            locals.var_dps0_dn6 = assign84680_body57_e130287_d_n6;
            locals.var_dps0_dn7 = assign84680_body57_e130287_d_n7;
            locals.var_dps0_dn8 = assign84680_body57_e130287_d_n8;
            locals.var_dps0_dn9 = assign84680_body57_e130287_d_n9;
            locals.var_dps0_dn10 = assign84680_body57_e130287_d_n10;
            locals.var_dps0_dn13 = assign84680_body57_e130287_d_n13;
            let (assign84680_body58_e130309, assign84680_body58_e130309_d_n0, assign84680_body58_e130309_d_n2, assign84680_body58_e130309_d_n4, assign84680_body58_e130309_d_n5, assign84680_body58_e130309_d_n6, assign84680_body58_e130309_d_n7, assign84680_body58_e130309_d_n8, assign84680_body58_e130309_d_n9, assign84680_body58_e130309_d_n10, assign84680_body58_e130309_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1964 != 0.0)) && (locals.var_guard1975 == 0.0)) {
        let assign84680_body58_e130296: f64 = (0.5 * 0.1);
        let assign84680_body58_e130300: f64 = (locals.var_ps0ld).abs();
        let (assign84680_body58_e130305, assign84680_body58_e130305_d_n0, assign84680_body58_e130305_d_n2, assign84680_body58_e130305_d_n4, assign84680_body58_e130305_d_n5, assign84680_body58_e130305_d_n6, assign84680_body58_e130305_d_n7, assign84680_body58_e130305_d_n8, assign84680_body58_e130305_d_n9, assign84680_body58_e130305_d_n10, assign84680_body58_e130305_d_n13,) = {
            if (1.0 >= assign84680_body58_e130300) {
                (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign84680_body58_e130304: f64 = (locals.var_ps0ld).abs();
                (assign84680_body58_e130304, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn0 } else { (-locals.var_ps0ld_dn0) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn2 } else { (-locals.var_ps0ld_dn2) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn4 } else { (-locals.var_ps0ld_dn4) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn5 } else { (-locals.var_ps0ld_dn5) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn6 } else { (-locals.var_ps0ld_dn6) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn7 } else { (-locals.var_ps0ld_dn7) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn8 } else { (-locals.var_ps0ld_dn8) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn9 } else { (-locals.var_ps0ld_dn9) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn10 } else { (-locals.var_ps0ld_dn10) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn13 } else { (-locals.var_ps0ld_dn13) },)
            }
        };
        let assign84680_body58_e130306: f64 = (1.0 + assign84680_body58_e130305);
        let assign84680_body58_e130307: f64 = (assign84680_body58_e130296 * assign84680_body58_e130306);
        (assign84680_body58_e130307, (assign84680_body58_e130296 * assign84680_body58_e130305_d_n0), (assign84680_body58_e130296 * assign84680_body58_e130305_d_n2), (assign84680_body58_e130296 * assign84680_body58_e130305_d_n4), (assign84680_body58_e130296 * assign84680_body58_e130305_d_n5), (assign84680_body58_e130296 * assign84680_body58_e130305_d_n6), (assign84680_body58_e130296 * assign84680_body58_e130305_d_n7), (assign84680_body58_e130296 * assign84680_body58_e130305_d_n8), (assign84680_body58_e130296 * assign84680_body58_e130305_d_n9), (assign84680_body58_e130296 * assign84680_body58_e130305_d_n10), (assign84680_body58_e130296 * assign84680_body58_e130305_d_n13),)
    } else {
        (locals.var_dplim, locals.var_dplim_dn0, locals.var_dplim_dn2, locals.var_dplim_dn4, locals.var_dplim_dn5, locals.var_dplim_dn6, locals.var_dplim_dn7, locals.var_dplim_dn8, locals.var_dplim_dn9, locals.var_dplim_dn10, locals.var_dplim_dn13,)
    }
};
            locals.var_dplim = assign84680_body58_e130309;
            locals.var_dplim_dn0 = assign84680_body58_e130309_d_n0;
            locals.var_dplim_dn2 = assign84680_body58_e130309_d_n2;
            locals.var_dplim_dn4 = assign84680_body58_e130309_d_n4;
            locals.var_dplim_dn5 = assign84680_body58_e130309_d_n5;
            locals.var_dplim_dn6 = assign84680_body58_e130309_d_n6;
            locals.var_dplim_dn7 = assign84680_body58_e130309_d_n7;
            locals.var_dplim_dn8 = assign84680_body58_e130309_d_n8;
            locals.var_dplim_dn9 = assign84680_body58_e130309_d_n9;
            locals.var_dplim_dn10 = assign84680_body58_e130309_d_n10;
            locals.var_dplim_dn13 = assign84680_body58_e130309_d_n13;
            let assign84680_body59_e130311: f64 = (locals.var_dps0).abs();
            let assign84680_body59_e130313: f64 = if assign84680_body59_e130311 > locals.var_dplim { 1.0 } else { 0.0 };
            locals.var_guard1976 = assign84680_body59_e130313;
            let (assign84680_body60_e130332, assign84680_body60_e130332_d_n0, assign84680_body60_e130332_d_n2, assign84680_body60_e130332_d_n4, assign84680_body60_e130332_d_n5, assign84680_body60_e130332_d_n6, assign84680_body60_e130332_d_n7, assign84680_body60_e130332_d_n8, assign84680_body60_e130332_d_n9, assign84680_body60_e130332_d_n10, assign84680_body60_e130332_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1964 != 0.0)) && (locals.var_guard1975 == 0.0)) && (locals.var_guard1976 != 0.0)) {
        let (assign84680_body60_e130329,) = {
            if (locals.var_dps0 >= 0.0) {
                (1.0,)
            } else {
                let assign84680_body60_e130328: f64 = (-1.0);
                (assign84680_body60_e130328,)
            }
        };
        let assign84680_body60_e130330: f64 = (locals.var_dplim * assign84680_body60_e130329);
        (assign84680_body60_e130330, (locals.var_dplim_dn0 * assign84680_body60_e130329), (locals.var_dplim_dn2 * assign84680_body60_e130329), (locals.var_dplim_dn4 * assign84680_body60_e130329), (locals.var_dplim_dn5 * assign84680_body60_e130329), (locals.var_dplim_dn6 * assign84680_body60_e130329), (locals.var_dplim_dn7 * assign84680_body60_e130329), (locals.var_dplim_dn8 * assign84680_body60_e130329), (locals.var_dplim_dn9 * assign84680_body60_e130329), (locals.var_dplim_dn10 * assign84680_body60_e130329), (locals.var_dplim_dn13 * assign84680_body60_e130329),)
    } else {
        (locals.var_dps0, locals.var_dps0_dn0, locals.var_dps0_dn2, locals.var_dps0_dn4, locals.var_dps0_dn5, locals.var_dps0_dn6, locals.var_dps0_dn7, locals.var_dps0_dn8, locals.var_dps0_dn9, locals.var_dps0_dn10, locals.var_dps0_dn13,)
    }
};
            locals.var_dps0 = assign84680_body60_e130332;
            locals.var_dps0_dn0 = assign84680_body60_e130332_d_n0;
            locals.var_dps0_dn2 = assign84680_body60_e130332_d_n2;
            locals.var_dps0_dn4 = assign84680_body60_e130332_d_n4;
            locals.var_dps0_dn5 = assign84680_body60_e130332_d_n5;
            locals.var_dps0_dn6 = assign84680_body60_e130332_d_n6;
            locals.var_dps0_dn7 = assign84680_body60_e130332_d_n7;
            locals.var_dps0_dn8 = assign84680_body60_e130332_d_n8;
            locals.var_dps0_dn9 = assign84680_body60_e130332_d_n9;
            locals.var_dps0_dn10 = assign84680_body60_e130332_d_n10;
            locals.var_dps0_dn13 = assign84680_body60_e130332_d_n13;
            let (assign84680_body61_e130343, assign84680_body61_e130343_d_n0, assign84680_body61_e130343_d_n2, assign84680_body61_e130343_d_n4, assign84680_body61_e130343_d_n5, assign84680_body61_e130343_d_n6, assign84680_body61_e130343_d_n7, assign84680_body61_e130343_d_n8, assign84680_body61_e130343_d_n9, assign84680_body61_e130343_d_n10, assign84680_body61_e130343_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1964 != 0.0)) && (locals.var_guard1975 == 0.0)) {
        let assign84680_body61_e130341: f64 = (locals.var_ps0ld + locals.var_dps0);
        (assign84680_body61_e130341, (locals.var_ps0ld_dn0 + locals.var_dps0_dn0), (locals.var_ps0ld_dn2 + locals.var_dps0_dn2), (locals.var_ps0ld_dn4 + locals.var_dps0_dn4), (locals.var_ps0ld_dn5 + locals.var_dps0_dn5), (locals.var_ps0ld_dn6 + locals.var_dps0_dn6), (locals.var_ps0ld_dn7 + locals.var_dps0_dn7), (locals.var_ps0ld_dn8 + locals.var_dps0_dn8), (locals.var_ps0ld_dn9 + locals.var_dps0_dn9), (locals.var_ps0ld_dn10 + locals.var_dps0_dn10), (locals.var_ps0ld_dn13 + locals.var_dps0_dn13),)
    } else {
        (locals.var_ps0ld, locals.var_ps0ld_dn0, locals.var_ps0ld_dn2, locals.var_ps0ld_dn4, locals.var_ps0ld_dn5, locals.var_ps0ld_dn6, locals.var_ps0ld_dn7, locals.var_ps0ld_dn8, locals.var_ps0ld_dn9, locals.var_ps0ld_dn10, locals.var_ps0ld_dn13,)
    }
};
            locals.var_ps0ld = assign84680_body61_e130343;
            locals.var_ps0ld_dn0 = assign84680_body61_e130343_d_n0;
            locals.var_ps0ld_dn2 = assign84680_body61_e130343_d_n2;
            locals.var_ps0ld_dn4 = assign84680_body61_e130343_d_n4;
            locals.var_ps0ld_dn5 = assign84680_body61_e130343_d_n5;
            locals.var_ps0ld_dn6 = assign84680_body61_e130343_d_n6;
            locals.var_ps0ld_dn7 = assign84680_body61_e130343_d_n7;
            locals.var_ps0ld_dn8 = assign84680_body61_e130343_d_n8;
            locals.var_ps0ld_dn9 = assign84680_body61_e130343_d_n9;
            locals.var_ps0ld_dn10 = assign84680_body61_e130343_d_n10;
            locals.var_ps0ld_dn13 = assign84680_body61_e130343_d_n13;
            let assign84680_body62_e130345: f64 = (locals.var_dps0).abs();
            let assign84680_body62_e130349: f64 = (locals.var_fs0).abs();
            let assign84680_body62_e130352: f64 = if ((assign84680_body62_e130345 <= 1e-12) && (assign84680_body62_e130349 <= 1e-8)) { 1.0 } else { 0.0 };
            locals.var_guard1977 = assign84680_body62_e130352;
            let (assign84680_body63_e130365,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1964 != 0.0)) && (locals.var_guard1975 == 0.0)) && (locals.var_guard1977 != 0.0)) {
        let assign84680_body63_e130363: f64 = (locals.var_flg_conv + 2.0);
        (assign84680_body63_e130363,)
    } else {
        (locals.var_flg_conv,)
    }
};
            locals.var_flg_conv = assign84680_body63_e130365;
            let (assign84680_body64_e130373,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1964 != 0.0)) {
        let assign84680_body64_e130371: f64 = (locals.var_lp_s0 + 1.0);
        (assign84680_body64_e130371,)
    } else {
        (locals.var_lp_s0,)
    }
};
            locals.var_lp_s0 = assign84680_body64_e130373;
        }

    }

    pub(super) fn stamp_transient_block_298(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign84700_e130396, assign84700_e130396_d_n0, assign84700_e130396_d_n2, assign84700_e130396_d_n4, assign84700_e130396_d_n5, assign84700_e130396_d_n6, assign84700_e130396_d_n7, assign84700_e130396_d_n8, assign84700_e130396_d_n9, assign84700_e130396_d_n10, assign84700_e130396_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1964 != 0.0)) {
        let (assign84700_e130394, assign84700_e130394_d_n0, assign84700_e130394_d_n2, assign84700_e130394_d_n4, assign84700_e130394_d_n5, assign84700_e130394_d_n6, assign84700_e130394_d_n7, assign84700_e130394_d_n8, assign84700_e130394_d_n9, assign84700_e130394_d_n10, assign84700_e130394_d_n13,) = {
            if (locals.var_fbsq__blk1891 >= 0.0) {
                let (assign84700_e130389,) = {
                    if (locals.var_chi >= 0.0) {
                        (1.0,)
                    } else {
                        let assign84700_e130388: f64 = (-1.0);
                        (assign84700_e130388,)
                    }
                };
                let assign84700_e130391: f64 = (locals.var_fbsq__blk1891).sqrt();
                let assign84700_e130392: f64 = (assign84700_e130389 * assign84700_e130391);
                (assign84700_e130392, (assign84700_e130389 * (locals.var_fbsq__blk1891_dn0 / (2.0 * assign84700_e130391))), (assign84700_e130389 * (locals.var_fbsq__blk1891_dn2 / (2.0 * assign84700_e130391))), (assign84700_e130389 * (locals.var_fbsq__blk1891_dn4 / (2.0 * assign84700_e130391))), (assign84700_e130389 * (locals.var_fbsq__blk1891_dn5 / (2.0 * assign84700_e130391))), (assign84700_e130389 * (locals.var_fbsq__blk1891_dn6 / (2.0 * assign84700_e130391))), (assign84700_e130389 * (locals.var_fbsq__blk1891_dn7 / (2.0 * assign84700_e130391))), (assign84700_e130389 * (locals.var_fbsq__blk1891_dn8 / (2.0 * assign84700_e130391))), (assign84700_e130389 * (locals.var_fbsq__blk1891_dn9 / (2.0 * assign84700_e130391))), (assign84700_e130389 * (locals.var_fbsq__blk1891_dn10 / (2.0 * assign84700_e130391))), (assign84700_e130389 * (locals.var_fbsq__blk1891_dn13 / (2.0 * assign84700_e130391))),)
            } else {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        (assign84700_e130394, assign84700_e130394_d_n0, assign84700_e130394_d_n2, assign84700_e130394_d_n4, assign84700_e130394_d_n5, assign84700_e130394_d_n6, assign84700_e130394_d_n7, assign84700_e130394_d_n8, assign84700_e130394_d_n9, assign84700_e130394_d_n10, assign84700_e130394_d_n13,)
    } else {
        (locals.var_fb, locals.var_fb_dn0, locals.var_fb_dn2, locals.var_fb_dn4, locals.var_fb_dn5, locals.var_fb_dn6, locals.var_fb_dn7, locals.var_fb_dn8, locals.var_fb_dn9, locals.var_fb_dn10, locals.var_fb_dn13,)
    }
};
        locals.var_fb = assign84700_e130396;
        locals.var_fb_dn0 = assign84700_e130396_d_n0;
        locals.var_fb_dn2 = assign84700_e130396_d_n2;
        locals.var_fb_dn4 = assign84700_e130396_d_n4;
        locals.var_fb_dn5 = assign84700_e130396_d_n5;
        locals.var_fb_dn6 = assign84700_e130396_d_n6;
        locals.var_fb_dn7 = assign84700_e130396_d_n7;
        locals.var_fb_dn8 = assign84700_e130396_d_n8;
        locals.var_fb_dn9 = assign84700_e130396_d_n9;
        locals.var_fb_dn10 = assign84700_e130396_d_n10;
        locals.var_fb_dn13 = assign84700_e130396_d_n13;

        let (assign84710_e130404, assign84710_e130404_d_n0, assign84710_e130404_d_n2, assign84710_e130404_d_n4, assign84710_e130404_d_n5, assign84710_e130404_d_n6, assign84710_e130404_d_n7, assign84710_e130404_d_n8, assign84710_e130404_d_n9, assign84710_e130404_d_n10, assign84710_e130404_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1964 != 0.0)) {
        let assign84710_e130402: f64 = (locals.var_c_w_ld * locals.var_fb);
        (assign84710_e130402, ((locals.var_c_w_ld_dn0 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn0)), ((locals.var_c_w_ld_dn2 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn2)), ((locals.var_c_w_ld_dn4 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn4)), ((locals.var_c_w_ld_dn5 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn5)), ((locals.var_c_w_ld_dn6 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn6)), ((locals.var_c_w_ld_dn7 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn7)), ((locals.var_c_w_ld_dn8 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn8)), ((locals.var_c_w_ld_dn9 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn9)), ((locals.var_c_w_ld_dn10 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn10)), ((locals.var_c_w_ld_dn13 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn13)),)
    } else {
        (locals.var_wdld__blk1881, locals.var_wdld__blk1881_dn0, locals.var_wdld__blk1881_dn2, locals.var_wdld__blk1881_dn4, locals.var_wdld__blk1881_dn5, locals.var_wdld__blk1881_dn6, locals.var_wdld__blk1881_dn7, locals.var_wdld__blk1881_dn8, locals.var_wdld__blk1881_dn9, locals.var_wdld__blk1881_dn10, locals.var_wdld__blk1881_dn13,)
    }
};
        locals.var_wdld__blk1881 = assign84710_e130404;
        locals.var_wdld__blk1881_dn0 = assign84710_e130404_d_n0;
        locals.var_wdld__blk1881_dn2 = assign84710_e130404_d_n2;
        locals.var_wdld__blk1881_dn4 = assign84710_e130404_d_n4;
        locals.var_wdld__blk1881_dn5 = assign84710_e130404_d_n5;
        locals.var_wdld__blk1881_dn6 = assign84710_e130404_d_n6;
        locals.var_wdld__blk1881_dn7 = assign84710_e130404_d_n7;
        locals.var_wdld__blk1881_dn8 = assign84710_e130404_d_n8;
        locals.var_wdld__blk1881_dn9 = assign84710_e130404_d_n9;
        locals.var_wdld__blk1881_dn10 = assign84710_e130404_d_n10;
        locals.var_wdld__blk1881_dn13 = assign84710_e130404_d_n13;

        let (assign84720_e130412, assign84720_e130412_d_n0, assign84720_e130412_d_n2, assign84720_e130412_d_n4, assign84720_e130412_d_n5, assign84720_e130412_d_n6, assign84720_e130412_d_n7, assign84720_e130412_d_n8, assign84720_e130412_d_n9, assign84720_e130412_d_n10, assign84720_e130412_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1964 != 0.0)) {
        let assign84720_e130410: f64 = (locals.var_q_nsubld__blk1883 * locals.var_wdld__blk1881);
        (assign84720_e130410, (locals.var_q_nsubld__blk1883 * locals.var_wdld__blk1881_dn0), (locals.var_q_nsubld__blk1883 * locals.var_wdld__blk1881_dn2), (locals.var_q_nsubld__blk1883 * locals.var_wdld__blk1881_dn4), (locals.var_q_nsubld__blk1883 * locals.var_wdld__blk1881_dn5), (locals.var_q_nsubld__blk1883 * locals.var_wdld__blk1881_dn6), (locals.var_q_nsubld__blk1883 * locals.var_wdld__blk1881_dn7), (locals.var_q_nsubld__blk1883 * locals.var_wdld__blk1881_dn8), (locals.var_q_nsubld__blk1883 * locals.var_wdld__blk1881_dn9), (locals.var_q_nsubld__blk1883 * locals.var_wdld__blk1881_dn10), (locals.var_q_nsubld__blk1883 * locals.var_wdld__blk1881_dn13),)
    } else {
        (locals.var_q_dep_ld__blk1882, locals.var_q_dep_ld__blk1882_dn0, locals.var_q_dep_ld__blk1882_dn2, locals.var_q_dep_ld__blk1882_dn4, locals.var_q_dep_ld__blk1882_dn5, locals.var_q_dep_ld__blk1882_dn6, locals.var_q_dep_ld__blk1882_dn7, locals.var_q_dep_ld__blk1882_dn8, locals.var_q_dep_ld__blk1882_dn9, locals.var_q_dep_ld__blk1882_dn10, locals.var_q_dep_ld__blk1882_dn13,)
    }
};
        locals.var_q_dep_ld__blk1882 = assign84720_e130412;
        locals.var_q_dep_ld__blk1882_dn0 = assign84720_e130412_d_n0;
        locals.var_q_dep_ld__blk1882_dn2 = assign84720_e130412_d_n2;
        locals.var_q_dep_ld__blk1882_dn4 = assign84720_e130412_d_n4;
        locals.var_q_dep_ld__blk1882_dn5 = assign84720_e130412_d_n5;
        locals.var_q_dep_ld__blk1882_dn6 = assign84720_e130412_d_n6;
        locals.var_q_dep_ld__blk1882_dn7 = assign84720_e130412_d_n7;
        locals.var_q_dep_ld__blk1882_dn8 = assign84720_e130412_d_n8;
        locals.var_q_dep_ld__blk1882_dn9 = assign84720_e130412_d_n9;
        locals.var_q_dep_ld__blk1882_dn10 = assign84720_e130412_d_n10;
        locals.var_q_dep_ld__blk1882_dn13 = assign84720_e130412_d_n13;

        let (assign84730_e130424, assign84730_e130424_d_n0, assign84730_e130424_d_n2, assign84730_e130424_d_n4, assign84730_e130424_d_n5, assign84730_e130424_d_n6, assign84730_e130424_d_n7, assign84730_e130424_d_n8, assign84730_e130424_d_n9, assign84730_e130424_d_n10, assign84730_e130424_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1964 != 0.0)) {
        let assign84730_e130418: f64 = (locals.var_q_dep_ld__blk1882 / locals.var_cnst0over_func);
        let assign84730_e130421: f64 = (10.0 * 2.220446049250313e-16);
        let assign84730_e130422: f64 = (assign84730_e130418 + assign84730_e130421);
        (assign84730_e130422, (((locals.var_q_dep_ld__blk1882_dn0 * locals.var_cnst0over_func) - (locals.var_q_dep_ld__blk1882 * locals.var_cnst0over_func_dn0)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)), (((locals.var_q_dep_ld__blk1882_dn2 * locals.var_cnst0over_func) - (locals.var_q_dep_ld__blk1882 * locals.var_cnst0over_func_dn2)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)), (((locals.var_q_dep_ld__blk1882_dn4 * locals.var_cnst0over_func) - (locals.var_q_dep_ld__blk1882 * locals.var_cnst0over_func_dn4)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)), (((locals.var_q_dep_ld__blk1882_dn5 * locals.var_cnst0over_func) - (locals.var_q_dep_ld__blk1882 * locals.var_cnst0over_func_dn5)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)), (((locals.var_q_dep_ld__blk1882_dn6 * locals.var_cnst0over_func) - (locals.var_q_dep_ld__blk1882 * locals.var_cnst0over_func_dn6)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)), (((locals.var_q_dep_ld__blk1882_dn7 * locals.var_cnst0over_func) - (locals.var_q_dep_ld__blk1882 * locals.var_cnst0over_func_dn7)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)), (((locals.var_q_dep_ld__blk1882_dn8 * locals.var_cnst0over_func) - (locals.var_q_dep_ld__blk1882 * locals.var_cnst0over_func_dn8)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)), (((locals.var_q_dep_ld__blk1882_dn9 * locals.var_cnst0over_func) - (locals.var_q_dep_ld__blk1882 * locals.var_cnst0over_func_dn9)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)), (((locals.var_q_dep_ld__blk1882_dn10 * locals.var_cnst0over_func) - (locals.var_q_dep_ld__blk1882 * locals.var_cnst0over_func_dn10)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)), (((locals.var_q_dep_ld__blk1882_dn13 * locals.var_cnst0over_func) - (locals.var_q_dep_ld__blk1882 * locals.var_cnst0over_func_dn13)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)),)
    } else {
        (locals.var_xi0p12, locals.var_xi0p12_dn0, locals.var_xi0p12_dn2, locals.var_xi0p12_dn4, locals.var_xi0p12_dn5, locals.var_xi0p12_dn6, locals.var_xi0p12_dn7, locals.var_xi0p12_dn8, locals.var_xi0p12_dn9, locals.var_xi0p12_dn10, locals.var_xi0p12_dn13,)
    }
};
        locals.var_xi0p12 = assign84730_e130424;
        locals.var_xi0p12_dn0 = assign84730_e130424_d_n0;
        locals.var_xi0p12_dn2 = assign84730_e130424_d_n2;
        locals.var_xi0p12_dn4 = assign84730_e130424_d_n4;
        locals.var_xi0p12_dn5 = assign84730_e130424_d_n5;
        locals.var_xi0p12_dn6 = assign84730_e130424_d_n6;
        locals.var_xi0p12_dn7 = assign84730_e130424_d_n7;
        locals.var_xi0p12_dn8 = assign84730_e130424_d_n8;
        locals.var_xi0p12_dn9 = assign84730_e130424_d_n9;
        locals.var_xi0p12_dn10 = assign84730_e130424_d_n10;
        locals.var_xi0p12_dn13 = assign84730_e130424_d_n13;

        let (assign84740_e130432, assign84740_e130432_d_n0, assign84740_e130432_d_n2, assign84740_e130432_d_n4, assign84740_e130432_d_n5, assign84740_e130432_d_n6, assign84740_e130432_d_n7, assign84740_e130432_d_n8, assign84740_e130432_d_n9, assign84740_e130432_d_n10, assign84740_e130432_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1964 != 0.0)) {
        let assign84740_e130430: f64 = (locals.var_cnst0over_func * locals.var_xi0p12);
        (assign84740_e130430, ((locals.var_cnst0over_func_dn0 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn0)), ((locals.var_cnst0over_func_dn2 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn2)), ((locals.var_cnst0over_func_dn4 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn4)), ((locals.var_cnst0over_func_dn5 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn5)), ((locals.var_cnst0over_func_dn6 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn6)), ((locals.var_cnst0over_func_dn7 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn7)), ((locals.var_cnst0over_func_dn8 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn8)), ((locals.var_cnst0over_func_dn9 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn9)), ((locals.var_cnst0over_func_dn10 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn10)), ((locals.var_cnst0over_func_dn13 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn13)),)
    } else {
        (locals.var_qbuld, locals.var_qbuld_dn0, locals.var_qbuld_dn2, locals.var_qbuld_dn4, locals.var_qbuld_dn5, locals.var_qbuld_dn6, locals.var_qbuld_dn7, locals.var_qbuld_dn8, locals.var_qbuld_dn9, locals.var_qbuld_dn10, locals.var_qbuld_dn13,)
    }
};
        locals.var_qbuld = assign84740_e130432;
        locals.var_qbuld_dn0 = assign84740_e130432_d_n0;
        locals.var_qbuld_dn2 = assign84740_e130432_d_n2;
        locals.var_qbuld_dn4 = assign84740_e130432_d_n4;
        locals.var_qbuld_dn5 = assign84740_e130432_d_n5;
        locals.var_qbuld_dn6 = assign84740_e130432_d_n6;
        locals.var_qbuld_dn7 = assign84740_e130432_d_n7;
        locals.var_qbuld_dn8 = assign84740_e130432_d_n8;
        locals.var_qbuld_dn9 = assign84740_e130432_d_n9;
        locals.var_qbuld_dn10 = assign84740_e130432_d_n10;
        locals.var_qbuld_dn13 = assign84740_e130432_d_n13;

        let (assign84750_e130442, assign84750_e130442_d_n0, assign84750_e130442_d_n2, assign84750_e130442_d_n4, assign84750_e130442_d_n5, assign84750_e130442_d_n6, assign84750_e130442_d_n7, assign84750_e130442_d_n8, assign84750_e130442_d_n9, assign84750_e130442_d_n10, assign84750_e130442_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1964 != 0.0)) {
        let assign84750_e130439: f64 = (locals.var_fs02 + locals.var_xi0p12);
        let assign84750_e130440: f64 = (1.0 / assign84750_e130439);
        (assign84750_e130440, (-((locals.var_fs02_dn0 + locals.var_xi0p12_dn0) / (assign84750_e130439 * assign84750_e130439))), (-((locals.var_fs02_dn2 + locals.var_xi0p12_dn2) / (assign84750_e130439 * assign84750_e130439))), (-((locals.var_fs02_dn4 + locals.var_xi0p12_dn4) / (assign84750_e130439 * assign84750_e130439))), (-((locals.var_fs02_dn5 + locals.var_xi0p12_dn5) / (assign84750_e130439 * assign84750_e130439))), (-((locals.var_fs02_dn6 + locals.var_xi0p12_dn6) / (assign84750_e130439 * assign84750_e130439))), (-((locals.var_fs02_dn7 + locals.var_xi0p12_dn7) / (assign84750_e130439 * assign84750_e130439))), (-((locals.var_fs02_dn8 + locals.var_xi0p12_dn8) / (assign84750_e130439 * assign84750_e130439))), (-((locals.var_fs02_dn9 + locals.var_xi0p12_dn9) / (assign84750_e130439 * assign84750_e130439))), (-((locals.var_fs02_dn10 + locals.var_xi0p12_dn10) / (assign84750_e130439 * assign84750_e130439))), (-((locals.var_fs02_dn13 + locals.var_xi0p12_dn13) / (assign84750_e130439 * assign84750_e130439))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign84750_e130442;
        locals.var_t1_dn0 = assign84750_e130442_d_n0;
        locals.var_t1_dn2 = assign84750_e130442_d_n2;
        locals.var_t1_dn4 = assign84750_e130442_d_n4;
        locals.var_t1_dn5 = assign84750_e130442_d_n5;
        locals.var_t1_dn6 = assign84750_e130442_d_n6;
        locals.var_t1_dn7 = assign84750_e130442_d_n7;
        locals.var_t1_dn8 = assign84750_e130442_d_n8;
        locals.var_t1_dn9 = assign84750_e130442_d_n9;
        locals.var_t1_dn10 = assign84750_e130442_d_n10;
        locals.var_t1_dn13 = assign84750_e130442_d_n13;

        let (assign84760_e130452, assign84760_e130452_d_n0, assign84760_e130452_d_n2, assign84760_e130452_d_n4, assign84760_e130452_d_n5, assign84760_e130452_d_n6, assign84760_e130452_d_n7, assign84760_e130452_d_n8, assign84760_e130452_d_n9, assign84760_e130452_d_n10, assign84760_e130452_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1964 != 0.0)) {
        let assign84760_e130448: f64 = (locals.var_cnst0over_func * locals.var_fs01);
        let assign84760_e130450: f64 = (assign84760_e130448 * locals.var_t1);
        (assign84760_e130450, ((((locals.var_cnst0over_func_dn0 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn0)) * locals.var_t1) + (assign84760_e130448 * locals.var_t1_dn0)), ((((locals.var_cnst0over_func_dn2 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn2)) * locals.var_t1) + (assign84760_e130448 * locals.var_t1_dn2)), ((((locals.var_cnst0over_func_dn4 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn4)) * locals.var_t1) + (assign84760_e130448 * locals.var_t1_dn4)), ((((locals.var_cnst0over_func_dn5 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn5)) * locals.var_t1) + (assign84760_e130448 * locals.var_t1_dn5)), ((((locals.var_cnst0over_func_dn6 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn6)) * locals.var_t1) + (assign84760_e130448 * locals.var_t1_dn6)), ((((locals.var_cnst0over_func_dn7 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn7)) * locals.var_t1) + (assign84760_e130448 * locals.var_t1_dn7)), ((((locals.var_cnst0over_func_dn8 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn8)) * locals.var_t1) + (assign84760_e130448 * locals.var_t1_dn8)), ((((locals.var_cnst0over_func_dn9 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn9)) * locals.var_t1) + (assign84760_e130448 * locals.var_t1_dn9)), ((((locals.var_cnst0over_func_dn10 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn10)) * locals.var_t1) + (assign84760_e130448 * locals.var_t1_dn10)), ((((locals.var_cnst0over_func_dn13 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn13)) * locals.var_t1) + (assign84760_e130448 * locals.var_t1_dn13)),)
    } else {
        (locals.var_qiuld, locals.var_qiuld_dn0, locals.var_qiuld_dn2, locals.var_qiuld_dn4, locals.var_qiuld_dn5, locals.var_qiuld_dn6, locals.var_qiuld_dn7, locals.var_qiuld_dn8, locals.var_qiuld_dn9, locals.var_qiuld_dn10, locals.var_qiuld_dn13,)
    }
};
        locals.var_qiuld = assign84760_e130452;
        locals.var_qiuld_dn0 = assign84760_e130452_d_n0;
        locals.var_qiuld_dn2 = assign84760_e130452_d_n2;
        locals.var_qiuld_dn4 = assign84760_e130452_d_n4;
        locals.var_qiuld_dn5 = assign84760_e130452_d_n5;
        locals.var_qiuld_dn6 = assign84760_e130452_d_n6;
        locals.var_qiuld_dn7 = assign84760_e130452_d_n7;
        locals.var_qiuld_dn8 = assign84760_e130452_d_n8;
        locals.var_qiuld_dn9 = assign84760_e130452_d_n9;
        locals.var_qiuld_dn10 = assign84760_e130452_d_n10;
        locals.var_qiuld_dn13 = assign84760_e130452_d_n13;

        let (assign84770_e130460, assign84770_e130460_d_n0, assign84770_e130460_d_n2, assign84770_e130460_d_n4, assign84770_e130460_d_n5, assign84770_e130460_d_n6, assign84770_e130460_d_n7, assign84770_e130460_d_n8, assign84770_e130460_d_n9, assign84770_e130460_d_n10, assign84770_e130460_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1964 != 0.0)) {
        let assign84770_e130458: f64 = (locals.var_qbuld + locals.var_qiuld);
        (assign84770_e130458, (locals.var_qbuld_dn0 + locals.var_qiuld_dn0), (locals.var_qbuld_dn2 + locals.var_qiuld_dn2), (locals.var_qbuld_dn4 + locals.var_qiuld_dn4), (locals.var_qbuld_dn5 + locals.var_qiuld_dn5), (locals.var_qbuld_dn6 + locals.var_qiuld_dn6), (locals.var_qbuld_dn7 + locals.var_qiuld_dn7), (locals.var_qbuld_dn8 + locals.var_qiuld_dn8), (locals.var_qbuld_dn9 + locals.var_qiuld_dn9), (locals.var_qbuld_dn10 + locals.var_qiuld_dn10), (locals.var_qbuld_dn13 + locals.var_qiuld_dn13),)
    } else {
        (locals.var_qsuld, locals.var_qsuld_dn0, locals.var_qsuld_dn2, locals.var_qsuld_dn4, locals.var_qsuld_dn5, locals.var_qsuld_dn6, locals.var_qsuld_dn7, locals.var_qsuld_dn8, locals.var_qsuld_dn9, locals.var_qsuld_dn10, locals.var_qsuld_dn13,)
    }
};
        locals.var_qsuld = assign84770_e130460;
        locals.var_qsuld_dn0 = assign84770_e130460_d_n0;
        locals.var_qsuld_dn2 = assign84770_e130460_d_n2;
        locals.var_qsuld_dn4 = assign84770_e130460_d_n4;
        locals.var_qsuld_dn5 = assign84770_e130460_d_n5;
        locals.var_qsuld_dn6 = assign84770_e130460_d_n6;
        locals.var_qsuld_dn7 = assign84770_e130460_d_n7;
        locals.var_qsuld_dn8 = assign84770_e130460_d_n8;
        locals.var_qsuld_dn9 = assign84770_e130460_d_n9;
        locals.var_qsuld_dn10 = assign84770_e130460_d_n10;
        locals.var_qsuld_dn13 = assign84770_e130460_d_n13;

        let (assign84780_e130466, assign84780_e130466_d_n0, assign84780_e130466_d_n2, assign84780_e130466_d_n4, assign84780_e130466_d_n5, assign84780_e130466_d_n6, assign84780_e130466_d_n7, assign84780_e130466_d_n8, assign84780_e130466_d_n9, assign84780_e130466_d_n10, assign84780_e130466_d_n13,) = {
    if (locals.var_flg_calcqover != 0.0) {
        let assign84780_e130464: f64 = (locals.var_qsuld - locals.var_qbuld);
        (assign84780_e130464, (locals.var_qsuld_dn0 - locals.var_qbuld_dn0), (locals.var_qsuld_dn2 - locals.var_qbuld_dn2), (locals.var_qsuld_dn4 - locals.var_qbuld_dn4), (locals.var_qsuld_dn5 - locals.var_qbuld_dn5), (locals.var_qsuld_dn6 - locals.var_qbuld_dn6), (locals.var_qsuld_dn7 - locals.var_qbuld_dn7), (locals.var_qsuld_dn8 - locals.var_qbuld_dn8), (locals.var_qsuld_dn9 - locals.var_qbuld_dn9), (locals.var_qsuld_dn10 - locals.var_qbuld_dn10), (locals.var_qsuld_dn13 - locals.var_qbuld_dn13),)
    } else {
        (locals.var_qiuld, locals.var_qiuld_dn0, locals.var_qiuld_dn2, locals.var_qiuld_dn4, locals.var_qiuld_dn5, locals.var_qiuld_dn6, locals.var_qiuld_dn7, locals.var_qiuld_dn8, locals.var_qiuld_dn9, locals.var_qiuld_dn10, locals.var_qiuld_dn13,)
    }
};
        locals.var_qiuld = assign84780_e130466;
        locals.var_qiuld_dn0 = assign84780_e130466_d_n0;
        locals.var_qiuld_dn2 = assign84780_e130466_d_n2;
        locals.var_qiuld_dn4 = assign84780_e130466_d_n4;
        locals.var_qiuld_dn5 = assign84780_e130466_d_n5;
        locals.var_qiuld_dn6 = assign84780_e130466_d_n6;
        locals.var_qiuld_dn7 = assign84780_e130466_d_n7;
        locals.var_qiuld_dn8 = assign84780_e130466_d_n8;
        locals.var_qiuld_dn9 = assign84780_e130466_d_n9;
        locals.var_qiuld_dn10 = assign84780_e130466_d_n10;
        locals.var_qiuld_dn13 = assign84780_e130466_d_n13;

        let assign84790_e130469: f64 = if locals.var_lover_func < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1979 = assign84790_e130469;

        let (assign84800_e130476, assign84800_e130476_d_n0, assign84800_e130476_d_n2, assign84800_e130476_d_n4, assign84800_e130476_d_n5, assign84800_e130476_d_n6, assign84800_e130476_d_n7, assign84800_e130476_d_n8, assign84800_e130476_d_n9, assign84800_e130476_d_n10, assign84800_e130476_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1979 != 0.0)) {
        let assign84800_e130474: f64 = (-locals.var_lover_func);
        (assign84800_e130474, (-locals.var_lover_func_dn0), (-locals.var_lover_func_dn2), (-locals.var_lover_func_dn4), (-locals.var_lover_func_dn5), (-locals.var_lover_func_dn6), (-locals.var_lover_func_dn7), (-locals.var_lover_func_dn8), (-locals.var_lover_func_dn9), (-locals.var_lover_func_dn10), (-locals.var_lover_func_dn13),)
    } else {
        (locals.var_lover_func, locals.var_lover_func_dn0, locals.var_lover_func_dn2, locals.var_lover_func_dn4, locals.var_lover_func_dn5, locals.var_lover_func_dn6, locals.var_lover_func_dn7, locals.var_lover_func_dn8, locals.var_lover_func_dn9, locals.var_lover_func_dn10, locals.var_lover_func_dn13,)
    }
};
        locals.var_lover_func = assign84800_e130476;
        locals.var_lover_func_dn0 = assign84800_e130476_d_n0;
        locals.var_lover_func_dn2 = assign84800_e130476_d_n2;
        locals.var_lover_func_dn4 = assign84800_e130476_d_n4;
        locals.var_lover_func_dn5 = assign84800_e130476_d_n5;
        locals.var_lover_func_dn6 = assign84800_e130476_d_n6;
        locals.var_lover_func_dn7 = assign84800_e130476_d_n7;
        locals.var_lover_func_dn8 = assign84800_e130476_d_n8;
        locals.var_lover_func_dn9 = assign84800_e130476_d_n9;
        locals.var_lover_func_dn10 = assign84800_e130476_d_n10;
        locals.var_lover_func_dn13 = assign84800_e130476_d_n13;

        let assign84810_e130479: f64 = if p.p55 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1980 = assign84810_e130479;

        let assign84820_e130482: f64 = if p.p50 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1981 = assign84820_e130482;

        let (assign84830_e130493, assign84830_e130493_d_n0, assign84830_e130493_d_n2, assign84830_e130493_d_n4, assign84830_e130493_d_n5, assign84830_e130493_d_n6, assign84830_e130493_d_n7, assign84830_e130493_d_n8, assign84830_e130493_d_n9, assign84830_e130493_d_n10, assign84830_e130493_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1979 != 0.0)) && (locals.var_guard1980 != 0.0)) && (locals.var_guard1981 != 0.0)) {
        let assign84830_e130491: f64 = (-locals.var_ps0ld);
        (assign84830_e130491, (-locals.var_ps0ld_dn0), (-locals.var_ps0ld_dn2), (-locals.var_ps0ld_dn4), (-locals.var_ps0ld_dn5), (-locals.var_ps0ld_dn6), (-locals.var_ps0ld_dn7), (-locals.var_ps0ld_dn8), (-locals.var_ps0ld_dn9), (-locals.var_ps0ld_dn10), (-locals.var_ps0ld_dn13),)
    } else {
        (locals.var_vx__blk1884, locals.var_vx__blk1884_dn0, locals.var_vx__blk1884_dn2, locals.var_vx__blk1884_dn4, locals.var_vx__blk1884_dn5, locals.var_vx__blk1884_dn6, locals.var_vx__blk1884_dn7, locals.var_vx__blk1884_dn8, locals.var_vx__blk1884_dn9, locals.var_vx__blk1884_dn10, locals.var_vx__blk1884_dn13,)
    }
};
        locals.var_vx__blk1884 = assign84830_e130493;
        locals.var_vx__blk1884_dn0 = assign84830_e130493_d_n0;
        locals.var_vx__blk1884_dn2 = assign84830_e130493_d_n2;
        locals.var_vx__blk1884_dn4 = assign84830_e130493_d_n4;
        locals.var_vx__blk1884_dn5 = assign84830_e130493_d_n5;
        locals.var_vx__blk1884_dn6 = assign84830_e130493_d_n6;
        locals.var_vx__blk1884_dn7 = assign84830_e130493_d_n7;
        locals.var_vx__blk1884_dn8 = assign84830_e130493_d_n8;
        locals.var_vx__blk1884_dn9 = assign84830_e130493_d_n9;
        locals.var_vx__blk1884_dn10 = assign84830_e130493_d_n10;
        locals.var_vx__blk1884_dn13 = assign84830_e130493_d_n13;

        let (assign84840_e130504, assign84840_e130504_d_n0, assign84840_e130504_d_n2, assign84840_e130504_d_n4, assign84840_e130504_d_n5, assign84840_e130504_d_n6, assign84840_e130504_d_n7, assign84840_e130504_d_n8, assign84840_e130504_d_n9, assign84840_e130504_d_n10, assign84840_e130504_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1979 != 0.0)) && (locals.var_guard1980 != 0.0)) && (locals.var_guard1981 == 0.0)) {
        (locals.var_vxbgmt, locals.var_vxbgmt_dn0, locals.var_vxbgmt_dn2, locals.var_vxbgmt_dn4, locals.var_vxbgmt_dn5, locals.var_vxbgmt_dn6, locals.var_vxbgmt_dn7, locals.var_vxbgmt_dn8, locals.var_vxbgmt_dn9, locals.var_vxbgmt_dn10, locals.var_vxbgmt_dn13,)
    } else {
        (locals.var_vx__blk1884, locals.var_vx__blk1884_dn0, locals.var_vx__blk1884_dn2, locals.var_vx__blk1884_dn4, locals.var_vx__blk1884_dn5, locals.var_vx__blk1884_dn6, locals.var_vx__blk1884_dn7, locals.var_vx__blk1884_dn8, locals.var_vx__blk1884_dn9, locals.var_vx__blk1884_dn10, locals.var_vx__blk1884_dn13,)
    }
};
        locals.var_vx__blk1884 = assign84840_e130504;
        locals.var_vx__blk1884_dn0 = assign84840_e130504_d_n0;
        locals.var_vx__blk1884_dn2 = assign84840_e130504_d_n2;
        locals.var_vx__blk1884_dn4 = assign84840_e130504_d_n4;
        locals.var_vx__blk1884_dn5 = assign84840_e130504_d_n5;
        locals.var_vx__blk1884_dn6 = assign84840_e130504_d_n6;
        locals.var_vx__blk1884_dn7 = assign84840_e130504_d_n7;
        locals.var_vx__blk1884_dn8 = assign84840_e130504_d_n8;
        locals.var_vx__blk1884_dn9 = assign84840_e130504_d_n9;
        locals.var_vx__blk1884_dn10 = assign84840_e130504_d_n10;
        locals.var_vx__blk1884_dn13 = assign84840_e130504_d_n13;

        let (assign84850_e130525, assign84850_e130525_d_n0, assign84850_e130525_d_n2, assign84850_e130525_d_n4, assign84850_e130525_d_n5, assign84850_e130525_d_n6, assign84850_e130525_d_n7, assign84850_e130525_d_n8, assign84850_e130525_d_n9, assign84850_e130525_d_n10, assign84850_e130525_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1979 != 0.0)) && (locals.var_guard1980 != 0.0)) {
        let assign84850_e130512: f64 = (locals.var_vx__blk1884 + p.p137);
        let assign84850_e130515: f64 = (locals.var_vx__blk1884 + p.p137);
        let assign84850_e130516: f64 = (assign84850_e130512 * assign84850_e130515);
        let assign84850_e130519: f64 = (4.0 * 0.1);
        let assign84850_e130521: f64 = (assign84850_e130519 * 0.1);
        let assign84850_e130522: f64 = (assign84850_e130516 + assign84850_e130521);
        let assign84850_e130523: f64 = (assign84850_e130522).sqrt();
        (assign84850_e130523, (((locals.var_vx__blk1884_dn0 * assign84850_e130515) + (assign84850_e130512 * locals.var_vx__blk1884_dn0)) / (2.0 * assign84850_e130523)), (((locals.var_vx__blk1884_dn2 * assign84850_e130515) + (assign84850_e130512 * locals.var_vx__blk1884_dn2)) / (2.0 * assign84850_e130523)), (((locals.var_vx__blk1884_dn4 * assign84850_e130515) + (assign84850_e130512 * locals.var_vx__blk1884_dn4)) / (2.0 * assign84850_e130523)), (((locals.var_vx__blk1884_dn5 * assign84850_e130515) + (assign84850_e130512 * locals.var_vx__blk1884_dn5)) / (2.0 * assign84850_e130523)), (((locals.var_vx__blk1884_dn6 * assign84850_e130515) + (assign84850_e130512 * locals.var_vx__blk1884_dn6)) / (2.0 * assign84850_e130523)), (((locals.var_vx__blk1884_dn7 * assign84850_e130515) + (assign84850_e130512 * locals.var_vx__blk1884_dn7)) / (2.0 * assign84850_e130523)), (((locals.var_vx__blk1884_dn8 * assign84850_e130515) + (assign84850_e130512 * locals.var_vx__blk1884_dn8)) / (2.0 * assign84850_e130523)), (((locals.var_vx__blk1884_dn9 * assign84850_e130515) + (assign84850_e130512 * locals.var_vx__blk1884_dn9)) / (2.0 * assign84850_e130523)), (((locals.var_vx__blk1884_dn10 * assign84850_e130515) + (assign84850_e130512 * locals.var_vx__blk1884_dn10)) / (2.0 * assign84850_e130523)), (((locals.var_vx__blk1884_dn13 * assign84850_e130515) + (assign84850_e130512 * locals.var_vx__blk1884_dn13)) / (2.0 * assign84850_e130523)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign84850_e130525;
        locals.var_tmf2_dn0 = assign84850_e130525_d_n0;
        locals.var_tmf2_dn2 = assign84850_e130525_d_n2;
        locals.var_tmf2_dn4 = assign84850_e130525_d_n4;
        locals.var_tmf2_dn5 = assign84850_e130525_d_n5;
        locals.var_tmf2_dn6 = assign84850_e130525_d_n6;
        locals.var_tmf2_dn7 = assign84850_e130525_d_n7;
        locals.var_tmf2_dn8 = assign84850_e130525_d_n8;
        locals.var_tmf2_dn9 = assign84850_e130525_d_n9;
        locals.var_tmf2_dn10 = assign84850_e130525_d_n10;
        locals.var_tmf2_dn13 = assign84850_e130525_d_n13;

        let (assign84860_e130541, assign84860_e130541_d_n0, assign84860_e130541_d_n2, assign84860_e130541_d_n4, assign84860_e130541_d_n5, assign84860_e130541_d_n6, assign84860_e130541_d_n7, assign84860_e130541_d_n8, assign84860_e130541_d_n9, assign84860_e130541_d_n10, assign84860_e130541_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1979 != 0.0)) && (locals.var_guard1980 != 0.0)) {
        let assign84860_e130535: f64 = (locals.var_vx__blk1884 + p.p137);
        let assign84860_e130537: f64 = (assign84860_e130535 / locals.var_tmf2);
        let assign84860_e130538: f64 = (1.0 + assign84860_e130537);
        let assign84860_e130539: f64 = (0.5 * assign84860_e130538);
        (assign84860_e130539, (0.5 * (((locals.var_vx__blk1884_dn0 * locals.var_tmf2) - (assign84860_e130535 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vx__blk1884_dn2 * locals.var_tmf2) - (assign84860_e130535 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vx__blk1884_dn4 * locals.var_tmf2) - (assign84860_e130535 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vx__blk1884_dn5 * locals.var_tmf2) - (assign84860_e130535 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vx__blk1884_dn6 * locals.var_tmf2) - (assign84860_e130535 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vx__blk1884_dn7 * locals.var_tmf2) - (assign84860_e130535 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vx__blk1884_dn8 * locals.var_tmf2) - (assign84860_e130535 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vx__blk1884_dn9 * locals.var_tmf2) - (assign84860_e130535 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vx__blk1884_dn10 * locals.var_tmf2) - (assign84860_e130535 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vx__blk1884_dn13 * locals.var_tmf2) - (assign84860_e130535 * locals.var_tmf2_dn13)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn13,)
    }
};
        locals.var_t9 = assign84860_e130541;
        locals.var_t9_dn0 = assign84860_e130541_d_n0;
        locals.var_t9_dn2 = assign84860_e130541_d_n2;
        locals.var_t9_dn4 = assign84860_e130541_d_n4;
        locals.var_t9_dn5 = assign84860_e130541_d_n5;
        locals.var_t9_dn6 = assign84860_e130541_d_n6;
        locals.var_t9_dn7 = assign84860_e130541_d_n7;
        locals.var_t9_dn8 = assign84860_e130541_d_n8;
        locals.var_t9_dn9 = assign84860_e130541_d_n9;
        locals.var_t9_dn10 = assign84860_e130541_d_n10;
        locals.var_t9_dn13 = assign84860_e130541_d_n13;

        let (assign84870_e130555, assign84870_e130555_d_n0, assign84870_e130555_d_n2, assign84870_e130555_d_n4, assign84870_e130555_d_n5, assign84870_e130555_d_n6, assign84870_e130555_d_n7, assign84870_e130555_d_n8, assign84870_e130555_d_n9, assign84870_e130555_d_n10, assign84870_e130555_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1979 != 0.0)) && (locals.var_guard1980 != 0.0)) {
        let assign84870_e130550: f64 = (locals.var_vx__blk1884 + p.p137);
        let assign84870_e130552: f64 = (assign84870_e130550 + locals.var_tmf2);
        let assign84870_e130553: f64 = (0.5 * assign84870_e130552);
        (assign84870_e130553, (0.5 * (locals.var_vx__blk1884_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_vx__blk1884_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_vx__blk1884_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_vx__blk1884_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_vx__blk1884_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_vx__blk1884_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_vx__blk1884_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_vx__blk1884_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_vx__blk1884_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_vx__blk1884_dn13 + locals.var_tmf2_dn13)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign84870_e130555;
        locals.var_t2_dn0 = assign84870_e130555_d_n0;
        locals.var_t2_dn2 = assign84870_e130555_d_n2;
        locals.var_t2_dn4 = assign84870_e130555_d_n4;
        locals.var_t2_dn5 = assign84870_e130555_d_n5;
        locals.var_t2_dn6 = assign84870_e130555_d_n6;
        locals.var_t2_dn7 = assign84870_e130555_d_n7;
        locals.var_t2_dn8 = assign84870_e130555_d_n8;
        locals.var_t2_dn9 = assign84870_e130555_d_n9;
        locals.var_t2_dn10 = assign84870_e130555_d_n10;
        locals.var_t2_dn13 = assign84870_e130555_d_n13;

        let assign84880_e130558: f64 = if locals.var_t2 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1982 = assign84880_e130558;

        let (assign84890_e130568, assign84890_e130568_d_n0, assign84890_e130568_d_n2, assign84890_e130568_d_n4, assign84890_e130568_d_n5, assign84890_e130568_d_n6, assign84890_e130568_d_n7, assign84890_e130568_d_n8, assign84890_e130568_d_n9, assign84890_e130568_d_n10, assign84890_e130568_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1979 != 0.0)) && (locals.var_guard1980 != 0.0)) && (locals.var_guard1982 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign84890_e130568;
        locals.var_t2_dn0 = assign84890_e130568_d_n0;
        locals.var_t2_dn2 = assign84890_e130568_d_n2;
        locals.var_t2_dn4 = assign84890_e130568_d_n4;
        locals.var_t2_dn5 = assign84890_e130568_d_n5;
        locals.var_t2_dn6 = assign84890_e130568_d_n6;
        locals.var_t2_dn7 = assign84890_e130568_d_n7;
        locals.var_t2_dn8 = assign84890_e130568_d_n8;
        locals.var_t2_dn9 = assign84890_e130568_d_n9;
        locals.var_t2_dn10 = assign84890_e130568_d_n10;
        locals.var_t2_dn13 = assign84890_e130568_d_n13;

        let (assign84900_e130578, assign84900_e130578_d_n0, assign84900_e130578_d_n2, assign84900_e130578_d_n4, assign84900_e130578_d_n5, assign84900_e130578_d_n6, assign84900_e130578_d_n7, assign84900_e130578_d_n8, assign84900_e130578_d_n9, assign84900_e130578_d_n10, assign84900_e130578_d_n13,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1979 != 0.0)) && (locals.var_guard1980 != 0.0)) && (locals.var_guard1982 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn13,)
    }
};
        locals.var_t9 = assign84900_e130578;
        locals.var_t9_dn0 = assign84900_e130578_d_n0;
        locals.var_t9_dn2 = assign84900_e130578_d_n2;
        locals.var_t9_dn4 = assign84900_e130578_d_n4;
        locals.var_t9_dn5 = assign84900_e130578_d_n5;
        locals.var_t9_dn6 = assign84900_e130578_d_n6;
        locals.var_t9_dn7 = assign84900_e130578_d_n7;
        locals.var_t9_dn8 = assign84900_e130578_d_n8;
        locals.var_t9_dn9 = assign84900_e130578_d_n9;
        locals.var_t9_dn10 = assign84900_e130578_d_n10;
        locals.var_t9_dn13 = assign84900_e130578_d_n13;

        let (assign84910_e130591, assign84910_e130591_d_n0, assign84910_e130591_d_n2, assign84910_e130591_d_n4, assign84910_e130591_d_n5, assign84910_e130591_d_n6, assign84910_e130591_d_n7, assign84910_e130591_d_n8, assign84910_e130591_d_n9, assign84910_e130591_d_n10, assign84910_e130591_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1979 != 0.0)) && (locals.var_guard1980 != 0.0)) {
        let assign84910_e130586: f64 = (locals.var_kjunc * locals.var_t2);
        let assign84910_e130587: f64 = (assign84910_e130586).sqrt();
        let assign84910_e130589: f64 = (assign84910_e130587 * p.p432);
        (assign84910_e130589, ((((locals.var_kjunc_dn0 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn0)) / (2.0 * assign84910_e130587)) * p.p432), ((((locals.var_kjunc_dn2 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn2)) / (2.0 * assign84910_e130587)) * p.p432), ((((locals.var_kjunc_dn4 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn4)) / (2.0 * assign84910_e130587)) * p.p432), ((((locals.var_kjunc_dn5 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn5)) / (2.0 * assign84910_e130587)) * p.p432), ((((locals.var_kjunc_dn6 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn6)) / (2.0 * assign84910_e130587)) * p.p432), ((((locals.var_kjunc_dn7 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn7)) / (2.0 * assign84910_e130587)) * p.p432), ((((locals.var_kjunc_dn8 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn8)) / (2.0 * assign84910_e130587)) * p.p432), ((((locals.var_kjunc_dn9 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn9)) / (2.0 * assign84910_e130587)) * p.p432), ((((locals.var_kjunc_dn10 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn10)) / (2.0 * assign84910_e130587)) * p.p432), ((((locals.var_kjunc_dn13 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn13)) / (2.0 * assign84910_e130587)) * p.p432),)
    } else {
        (locals.var_wjunc0, locals.var_wjunc0_dn0, locals.var_wjunc0_dn2, locals.var_wjunc0_dn4, locals.var_wjunc0_dn5, locals.var_wjunc0_dn6, locals.var_wjunc0_dn7, locals.var_wjunc0_dn8, locals.var_wjunc0_dn9, locals.var_wjunc0_dn10, locals.var_wjunc0_dn13,)
    }
};
        locals.var_wjunc0 = assign84910_e130591;
        locals.var_wjunc0_dn0 = assign84910_e130591_d_n0;
        locals.var_wjunc0_dn2 = assign84910_e130591_d_n2;
        locals.var_wjunc0_dn4 = assign84910_e130591_d_n4;
        locals.var_wjunc0_dn5 = assign84910_e130591_d_n5;
        locals.var_wjunc0_dn6 = assign84910_e130591_d_n6;
        locals.var_wjunc0_dn7 = assign84910_e130591_d_n7;
        locals.var_wjunc0_dn8 = assign84910_e130591_d_n8;
        locals.var_wjunc0_dn9 = assign84910_e130591_d_n9;
        locals.var_wjunc0_dn10 = assign84910_e130591_d_n10;
        locals.var_wjunc0_dn13 = assign84910_e130591_d_n13;

        let (assign84920_e130605, assign84920_e130605_d_n0, assign84920_e130605_d_n2, assign84920_e130605_d_n4, assign84920_e130605_d_n5, assign84920_e130605_d_n6, assign84920_e130605_d_n7, assign84920_e130605_d_n8, assign84920_e130605_d_n9, assign84920_e130605_d_n10, assign84920_e130605_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1979 != 0.0)) && (locals.var_guard1980 != 0.0)) {
        let assign84920_e130599: f64 = (locals.var_lover_func - locals.var_wjunc0);
        let assign84920_e130602: f64 = (0.1 * locals.var_lover_func);
        let assign84920_e130603: f64 = (assign84920_e130599 - assign84920_e130602);
        (assign84920_e130603, ((locals.var_lover_func_dn0 - locals.var_wjunc0_dn0) - (0.1 * locals.var_lover_func_dn0)), ((locals.var_lover_func_dn2 - locals.var_wjunc0_dn2) - (0.1 * locals.var_lover_func_dn2)), ((locals.var_lover_func_dn4 - locals.var_wjunc0_dn4) - (0.1 * locals.var_lover_func_dn4)), ((locals.var_lover_func_dn5 - locals.var_wjunc0_dn5) - (0.1 * locals.var_lover_func_dn5)), ((locals.var_lover_func_dn6 - locals.var_wjunc0_dn6) - (0.1 * locals.var_lover_func_dn6)), ((locals.var_lover_func_dn7 - locals.var_wjunc0_dn7) - (0.1 * locals.var_lover_func_dn7)), ((locals.var_lover_func_dn8 - locals.var_wjunc0_dn8) - (0.1 * locals.var_lover_func_dn8)), ((locals.var_lover_func_dn9 - locals.var_wjunc0_dn9) - (0.1 * locals.var_lover_func_dn9)), ((locals.var_lover_func_dn10 - locals.var_wjunc0_dn10) - (0.1 * locals.var_lover_func_dn10)), ((locals.var_lover_func_dn13 - locals.var_wjunc0_dn13) - (0.1 * locals.var_lover_func_dn13)),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13,)
    }
};
        locals.var_tmf1 = assign84920_e130605;
        locals.var_tmf1_dn0 = assign84920_e130605_d_n0;
        locals.var_tmf1_dn2 = assign84920_e130605_d_n2;
        locals.var_tmf1_dn4 = assign84920_e130605_d_n4;
        locals.var_tmf1_dn5 = assign84920_e130605_d_n5;
        locals.var_tmf1_dn6 = assign84920_e130605_d_n6;
        locals.var_tmf1_dn7 = assign84920_e130605_d_n7;
        locals.var_tmf1_dn8 = assign84920_e130605_d_n8;
        locals.var_tmf1_dn9 = assign84920_e130605_d_n9;
        locals.var_tmf1_dn10 = assign84920_e130605_d_n10;
        locals.var_tmf1_dn13 = assign84920_e130605_d_n13;

        let (assign84930_e130619, assign84930_e130619_d_n0, assign84930_e130619_d_n2, assign84930_e130619_d_n4, assign84930_e130619_d_n5, assign84930_e130619_d_n6, assign84930_e130619_d_n7, assign84930_e130619_d_n8, assign84930_e130619_d_n9, assign84930_e130619_d_n10, assign84930_e130619_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1979 != 0.0)) && (locals.var_guard1980 != 0.0)) {
        let assign84930_e130613: f64 = (4.0 * locals.var_lover_func);
        let assign84930_e130616: f64 = (0.1 * locals.var_lover_func);
        let assign84930_e130617: f64 = (assign84930_e130613 * assign84930_e130616);
        (assign84930_e130617, (((4.0 * locals.var_lover_func_dn0) * assign84930_e130616) + (assign84930_e130613 * (0.1 * locals.var_lover_func_dn0))), (((4.0 * locals.var_lover_func_dn2) * assign84930_e130616) + (assign84930_e130613 * (0.1 * locals.var_lover_func_dn2))), (((4.0 * locals.var_lover_func_dn4) * assign84930_e130616) + (assign84930_e130613 * (0.1 * locals.var_lover_func_dn4))), (((4.0 * locals.var_lover_func_dn5) * assign84930_e130616) + (assign84930_e130613 * (0.1 * locals.var_lover_func_dn5))), (((4.0 * locals.var_lover_func_dn6) * assign84930_e130616) + (assign84930_e130613 * (0.1 * locals.var_lover_func_dn6))), (((4.0 * locals.var_lover_func_dn7) * assign84930_e130616) + (assign84930_e130613 * (0.1 * locals.var_lover_func_dn7))), (((4.0 * locals.var_lover_func_dn8) * assign84930_e130616) + (assign84930_e130613 * (0.1 * locals.var_lover_func_dn8))), (((4.0 * locals.var_lover_func_dn9) * assign84930_e130616) + (assign84930_e130613 * (0.1 * locals.var_lover_func_dn9))), (((4.0 * locals.var_lover_func_dn10) * assign84930_e130616) + (assign84930_e130613 * (0.1 * locals.var_lover_func_dn10))), (((4.0 * locals.var_lover_func_dn13) * assign84930_e130616) + (assign84930_e130613 * (0.1 * locals.var_lover_func_dn13))),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign84930_e130619;
        locals.var_tmf2_dn0 = assign84930_e130619_d_n0;
        locals.var_tmf2_dn2 = assign84930_e130619_d_n2;
        locals.var_tmf2_dn4 = assign84930_e130619_d_n4;
        locals.var_tmf2_dn5 = assign84930_e130619_d_n5;
        locals.var_tmf2_dn6 = assign84930_e130619_d_n6;
        locals.var_tmf2_dn7 = assign84930_e130619_d_n7;
        locals.var_tmf2_dn8 = assign84930_e130619_d_n8;
        locals.var_tmf2_dn9 = assign84930_e130619_d_n9;
        locals.var_tmf2_dn10 = assign84930_e130619_d_n10;
        locals.var_tmf2_dn13 = assign84930_e130619_d_n13;

        let (assign84940_e130633, assign84940_e130633_d_n0, assign84940_e130633_d_n2, assign84940_e130633_d_n4, assign84940_e130633_d_n5, assign84940_e130633_d_n6, assign84940_e130633_d_n7, assign84940_e130633_d_n8, assign84940_e130633_d_n9, assign84940_e130633_d_n10, assign84940_e130633_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1979 != 0.0)) && (locals.var_guard1980 != 0.0)) {
        let (assign84940_e130631, assign84940_e130631_d_n0, assign84940_e130631_d_n2, assign84940_e130631_d_n4, assign84940_e130631_d_n5, assign84940_e130631_d_n6, assign84940_e130631_d_n7, assign84940_e130631_d_n8, assign84940_e130631_d_n9, assign84940_e130631_d_n10, assign84940_e130631_d_n13,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
            } else {
                let assign84940_e130630: f64 = (-locals.var_tmf2);
                (assign84940_e130630, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn13),)
            }
        };
        (assign84940_e130631, assign84940_e130631_d_n0, assign84940_e130631_d_n2, assign84940_e130631_d_n4, assign84940_e130631_d_n5, assign84940_e130631_d_n6, assign84940_e130631_d_n7, assign84940_e130631_d_n8, assign84940_e130631_d_n9, assign84940_e130631_d_n10, assign84940_e130631_d_n13,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign84940_e130633;
        locals.var_tmf2_dn0 = assign84940_e130633_d_n0;
        locals.var_tmf2_dn2 = assign84940_e130633_d_n2;
        locals.var_tmf2_dn4 = assign84940_e130633_d_n4;
        locals.var_tmf2_dn5 = assign84940_e130633_d_n5;
        locals.var_tmf2_dn6 = assign84940_e130633_d_n6;
        locals.var_tmf2_dn7 = assign84940_e130633_d_n7;
        locals.var_tmf2_dn8 = assign84940_e130633_d_n8;
        locals.var_tmf2_dn9 = assign84940_e130633_d_n9;
        locals.var_tmf2_dn10 = assign84940_e130633_d_n10;
        locals.var_tmf2_dn13 = assign84940_e130633_d_n13;

        let (assign84950_e130646, assign84950_e130646_d_n0, assign84950_e130646_d_n2, assign84950_e130646_d_n4, assign84950_e130646_d_n5, assign84950_e130646_d_n6, assign84950_e130646_d_n7, assign84950_e130646_d_n8, assign84950_e130646_d_n9, assign84950_e130646_d_n10, assign84950_e130646_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1979 != 0.0)) && (locals.var_guard1980 != 0.0)) {
        let assign84950_e130641: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign84950_e130643: f64 = (assign84950_e130641 + locals.var_tmf2);
        let assign84950_e130644: f64 = (assign84950_e130643).sqrt();
        (assign84950_e130644, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign84950_e130644)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign84950_e130644)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign84950_e130644)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign84950_e130644)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign84950_e130644)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign84950_e130644)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign84950_e130644)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign84950_e130644)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign84950_e130644)), ((((locals.var_tmf1_dn13 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn13)) + locals.var_tmf2_dn13) / (2.0 * assign84950_e130644)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign84950_e130646;
        locals.var_tmf2_dn0 = assign84950_e130646_d_n0;
        locals.var_tmf2_dn2 = assign84950_e130646_d_n2;
        locals.var_tmf2_dn4 = assign84950_e130646_d_n4;
        locals.var_tmf2_dn5 = assign84950_e130646_d_n5;
        locals.var_tmf2_dn6 = assign84950_e130646_d_n6;
        locals.var_tmf2_dn7 = assign84950_e130646_d_n7;
        locals.var_tmf2_dn8 = assign84950_e130646_d_n8;
        locals.var_tmf2_dn9 = assign84950_e130646_d_n9;
        locals.var_tmf2_dn10 = assign84950_e130646_d_n10;
        locals.var_tmf2_dn13 = assign84950_e130646_d_n13;

    }

    pub(super) fn stamp_transient_block_299(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign84960_e130660, assign84960_e130660_d_n0, assign84960_e130660_d_n2, assign84960_e130660_d_n4, assign84960_e130660_d_n5, assign84960_e130660_d_n6, assign84960_e130660_d_n7, assign84960_e130660_d_n8, assign84960_e130660_d_n9, assign84960_e130660_d_n10, assign84960_e130660_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1979 != 0.0)) && (locals.var_guard1980 != 0.0)) {
        let assign84960_e130656: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign84960_e130657: f64 = (1.0 + assign84960_e130656);
        let assign84960_e130658: f64 = (0.5 * assign84960_e130657);
        (assign84960_e130658, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn13 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn13)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign84960_e130660;
        locals.var_t0_dn0 = assign84960_e130660_d_n0;
        locals.var_t0_dn2 = assign84960_e130660_d_n2;
        locals.var_t0_dn4 = assign84960_e130660_d_n4;
        locals.var_t0_dn5 = assign84960_e130660_d_n5;
        locals.var_t0_dn6 = assign84960_e130660_d_n6;
        locals.var_t0_dn7 = assign84960_e130660_d_n7;
        locals.var_t0_dn8 = assign84960_e130660_d_n8;
        locals.var_t0_dn9 = assign84960_e130660_d_n9;
        locals.var_t0_dn10 = assign84960_e130660_d_n10;
        locals.var_t0_dn13 = assign84960_e130660_d_n13;

        let (assign84970_e130674, assign84970_e130674_d_n0, assign84970_e130674_d_n2, assign84970_e130674_d_n4, assign84970_e130674_d_n5, assign84970_e130674_d_n6, assign84970_e130674_d_n7, assign84970_e130674_d_n8, assign84970_e130674_d_n9, assign84970_e130674_d_n10, assign84970_e130674_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1979 != 0.0)) && (locals.var_guard1980 != 0.0)) {
        let assign84970_e130670: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign84970_e130671: f64 = (0.5 * assign84970_e130670);
        let assign84970_e130672: f64 = (locals.var_lover_func - assign84970_e130671);
        (assign84970_e130672, (locals.var_lover_func_dn0 - (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (locals.var_lover_func_dn2 - (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), (locals.var_lover_func_dn4 - (0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4))), (locals.var_lover_func_dn5 - (0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5))), (locals.var_lover_func_dn6 - (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), (locals.var_lover_func_dn7 - (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))), (locals.var_lover_func_dn8 - (0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8))), (locals.var_lover_func_dn9 - (0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9))), (locals.var_lover_func_dn10 - (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), (locals.var_lover_func_dn13 - (0.5 * (locals.var_tmf1_dn13 + locals.var_tmf2_dn13))),)
    } else {
        (locals.var_wjuncld, locals.var_wjuncld_dn0, locals.var_wjuncld_dn2, locals.var_wjuncld_dn4, locals.var_wjuncld_dn5, locals.var_wjuncld_dn6, locals.var_wjuncld_dn7, locals.var_wjuncld_dn8, locals.var_wjuncld_dn9, locals.var_wjuncld_dn10, locals.var_wjuncld_dn13,)
    }
};
        locals.var_wjuncld = assign84970_e130674;
        locals.var_wjuncld_dn0 = assign84970_e130674_d_n0;
        locals.var_wjuncld_dn2 = assign84970_e130674_d_n2;
        locals.var_wjuncld_dn4 = assign84970_e130674_d_n4;
        locals.var_wjuncld_dn5 = assign84970_e130674_d_n5;
        locals.var_wjuncld_dn6 = assign84970_e130674_d_n6;
        locals.var_wjuncld_dn7 = assign84970_e130674_d_n7;
        locals.var_wjuncld_dn8 = assign84970_e130674_d_n8;
        locals.var_wjuncld_dn9 = assign84970_e130674_d_n9;
        locals.var_wjuncld_dn10 = assign84970_e130674_d_n10;
        locals.var_wjuncld_dn13 = assign84970_e130674_d_n13;

        let (assign84980_e130684, assign84980_e130684_d_n0, assign84980_e130684_d_n2, assign84980_e130684_d_n4, assign84980_e130684_d_n5, assign84980_e130684_d_n6, assign84980_e130684_d_n7, assign84980_e130684_d_n8, assign84980_e130684_d_n9, assign84980_e130684_d_n10, assign84980_e130684_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1979 != 0.0)) && (locals.var_guard1980 != 0.0)) {
        let assign84980_e130682: f64 = (locals.var_lover_func - locals.var_wjuncld);
        (assign84980_e130682, (locals.var_lover_func_dn0 - locals.var_wjuncld_dn0), (locals.var_lover_func_dn2 - locals.var_wjuncld_dn2), (locals.var_lover_func_dn4 - locals.var_wjuncld_dn4), (locals.var_lover_func_dn5 - locals.var_wjuncld_dn5), (locals.var_lover_func_dn6 - locals.var_wjuncld_dn6), (locals.var_lover_func_dn7 - locals.var_wjuncld_dn7), (locals.var_lover_func_dn8 - locals.var_wjuncld_dn8), (locals.var_lover_func_dn9 - locals.var_wjuncld_dn9), (locals.var_lover_func_dn10 - locals.var_wjuncld_dn10), (locals.var_lover_func_dn13 - locals.var_wjuncld_dn13),)
    } else {
        (locals.var_lover_func, locals.var_lover_func_dn0, locals.var_lover_func_dn2, locals.var_lover_func_dn4, locals.var_lover_func_dn5, locals.var_lover_func_dn6, locals.var_lover_func_dn7, locals.var_lover_func_dn8, locals.var_lover_func_dn9, locals.var_lover_func_dn10, locals.var_lover_func_dn13,)
    }
};
        locals.var_lover_func = assign84980_e130684;
        locals.var_lover_func_dn0 = assign84980_e130684_d_n0;
        locals.var_lover_func_dn2 = assign84980_e130684_d_n2;
        locals.var_lover_func_dn4 = assign84980_e130684_d_n4;
        locals.var_lover_func_dn5 = assign84980_e130684_d_n5;
        locals.var_lover_func_dn6 = assign84980_e130684_d_n6;
        locals.var_lover_func_dn7 = assign84980_e130684_d_n7;
        locals.var_lover_func_dn8 = assign84980_e130684_d_n8;
        locals.var_lover_func_dn9 = assign84980_e130684_d_n9;
        locals.var_lover_func_dn10 = assign84980_e130684_d_n10;
        locals.var_lover_func_dn13 = assign84980_e130684_d_n13;

        let assign84990_e130687: f64 = if 3.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1983 = assign84990_e130687;

        let assign85000_e130690: f64 = if 3.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard1984 = assign85000_e130690;

        let assign85010_e130693: f64 = if 3.0 == 3.0 { 1.0 } else { 0.0 };
        locals.var_guard1985 = assign85010_e130693;

        let assign85020_e130696: f64 = if 3.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard1986 = assign85020_e130696;

        let assign85030_e130699: f64 = if p.p55 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1987 = assign85030_e130699;

        let (assign85040_e130709, assign85040_e130709_d_n0, assign85040_e130709_d_n2, assign85040_e130709_d_n4, assign85040_e130709_d_n5, assign85040_e130709_d_n6, assign85040_e130709_d_n7, assign85040_e130709_d_n8, assign85040_e130709_d_n9, assign85040_e130709_d_n10, assign85040_e130709_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1983 != 0.0)) && (locals.var_guard1987 != 0.0)) {
        let assign85040_e130707: f64 = (locals.var_weffcv_nf * locals.var_lover_func);
        (assign85040_e130707, (locals.var_weffcv_nf * locals.var_lover_func_dn0), (locals.var_weffcv_nf * locals.var_lover_func_dn2), (locals.var_weffcv_nf * locals.var_lover_func_dn4), (locals.var_weffcv_nf * locals.var_lover_func_dn5), (locals.var_weffcv_nf * locals.var_lover_func_dn6), (locals.var_weffcv_nf * locals.var_lover_func_dn7), (locals.var_weffcv_nf * locals.var_lover_func_dn8), (locals.var_weffcv_nf * locals.var_lover_func_dn9), (locals.var_weffcv_nf * locals.var_lover_func_dn10), (locals.var_weffcv_nf * locals.var_lover_func_dn13),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn13,)
    }
};
        locals.var_t4 = assign85040_e130709;
        locals.var_t4_dn0 = assign85040_e130709_d_n0;
        locals.var_t4_dn2 = assign85040_e130709_d_n2;
        locals.var_t4_dn4 = assign85040_e130709_d_n4;
        locals.var_t4_dn5 = assign85040_e130709_d_n5;
        locals.var_t4_dn6 = assign85040_e130709_d_n6;
        locals.var_t4_dn7 = assign85040_e130709_d_n7;
        locals.var_t4_dn8 = assign85040_e130709_d_n8;
        locals.var_t4_dn9 = assign85040_e130709_d_n9;
        locals.var_t4_dn10 = assign85040_e130709_d_n10;
        locals.var_t4_dn13 = assign85040_e130709_d_n13;

        let (assign85050_e130724, assign85050_e130724_d_n0, assign85050_e130724_d_n2, assign85050_e130724_d_n4, assign85050_e130724_d_n5, assign85050_e130724_d_n6, assign85050_e130724_d_n7, assign85050_e130724_d_n8, assign85050_e130724_d_n9, assign85050_e130724_d_n10, assign85050_e130724_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1983 != 0.0)) && (locals.var_guard1987 == 0.0)) {
        let assign85050_e130718: f64 = (locals.var_weffcv_nf * locals.var_lover_func);
        let assign85050_e130721: f64 = (1.0 - locals.var_uc_cvdsover);
        let assign85050_e130722: f64 = (assign85050_e130718 * assign85050_e130721);
        (assign85050_e130722, ((locals.var_weffcv_nf * locals.var_lover_func_dn0) * assign85050_e130721), ((locals.var_weffcv_nf * locals.var_lover_func_dn2) * assign85050_e130721), ((locals.var_weffcv_nf * locals.var_lover_func_dn4) * assign85050_e130721), ((locals.var_weffcv_nf * locals.var_lover_func_dn5) * assign85050_e130721), ((locals.var_weffcv_nf * locals.var_lover_func_dn6) * assign85050_e130721), ((locals.var_weffcv_nf * locals.var_lover_func_dn7) * assign85050_e130721), ((locals.var_weffcv_nf * locals.var_lover_func_dn8) * assign85050_e130721), ((locals.var_weffcv_nf * locals.var_lover_func_dn9) * assign85050_e130721), ((locals.var_weffcv_nf * locals.var_lover_func_dn10) * assign85050_e130721), ((locals.var_weffcv_nf * locals.var_lover_func_dn13) * assign85050_e130721),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn13,)
    }
};
        locals.var_t4 = assign85050_e130724;
        locals.var_t4_dn0 = assign85050_e130724_d_n0;
        locals.var_t4_dn2 = assign85050_e130724_d_n2;
        locals.var_t4_dn4 = assign85050_e130724_d_n4;
        locals.var_t4_dn5 = assign85050_e130724_d_n5;
        locals.var_t4_dn6 = assign85050_e130724_d_n6;
        locals.var_t4_dn7 = assign85050_e130724_d_n7;
        locals.var_t4_dn8 = assign85050_e130724_d_n8;
        locals.var_t4_dn9 = assign85050_e130724_d_n9;
        locals.var_t4_dn10 = assign85050_e130724_d_n10;
        locals.var_t4_dn13 = assign85050_e130724_d_n13;

        let (assign85060_e130732, assign85060_e130732_d_n0, assign85060_e130732_d_n2, assign85060_e130732_d_n4, assign85060_e130732_d_n5, assign85060_e130732_d_n6, assign85060_e130732_d_n7, assign85060_e130732_d_n8, assign85060_e130732_d_n9, assign85060_e130732_d_n10, assign85060_e130732_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1983 != 0.0)) {
        let assign85060_e130730: f64 = (locals.var_t4 * locals.var_qsuld);
        (assign85060_e130730, ((locals.var_t4_dn0 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn0)), ((locals.var_t4_dn2 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn2)), ((locals.var_t4_dn4 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn4)), ((locals.var_t4_dn5 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn5)), ((locals.var_t4_dn6 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn6)), ((locals.var_t4_dn7 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn7)), ((locals.var_t4_dn8 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn8)), ((locals.var_t4_dn9 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn9)), ((locals.var_t4_dn10 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn10)), ((locals.var_t4_dn13 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn13)),)
    } else {
        (locals.var_qovs, locals.var_qovs_dn0, locals.var_qovs_dn2, locals.var_qovs_dn4, locals.var_qovs_dn5, locals.var_qovs_dn6, locals.var_qovs_dn7, locals.var_qovs_dn8, locals.var_qovs_dn9, locals.var_qovs_dn10, locals.var_qovs_dn13,)
    }
};
        locals.var_qovs = assign85060_e130732;
        locals.var_qovs_dn0 = assign85060_e130732_d_n0;
        locals.var_qovs_dn2 = assign85060_e130732_d_n2;
        locals.var_qovs_dn4 = assign85060_e130732_d_n4;
        locals.var_qovs_dn5 = assign85060_e130732_d_n5;
        locals.var_qovs_dn6 = assign85060_e130732_d_n6;
        locals.var_qovs_dn7 = assign85060_e130732_d_n7;
        locals.var_qovs_dn8 = assign85060_e130732_d_n8;
        locals.var_qovs_dn9 = assign85060_e130732_d_n9;
        locals.var_qovs_dn10 = assign85060_e130732_d_n10;
        locals.var_qovs_dn13 = assign85060_e130732_d_n13;

        let (assign85070_e130740, assign85070_e130740_d_n0, assign85070_e130740_d_n2, assign85070_e130740_d_n4, assign85070_e130740_d_n5, assign85070_e130740_d_n6, assign85070_e130740_d_n7, assign85070_e130740_d_n8, assign85070_e130740_d_n9, assign85070_e130740_d_n10, assign85070_e130740_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1983 != 0.0)) {
        let assign85070_e130738: f64 = (locals.var_t4 * locals.var_qbuld);
        (assign85070_e130738, ((locals.var_t4_dn0 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn0)), ((locals.var_t4_dn2 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn2)), ((locals.var_t4_dn4 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn4)), ((locals.var_t4_dn5 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn5)), ((locals.var_t4_dn6 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn6)), ((locals.var_t4_dn7 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn7)), ((locals.var_t4_dn8 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn8)), ((locals.var_t4_dn9 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn9)), ((locals.var_t4_dn10 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn10)), ((locals.var_t4_dn13 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn13)),)
    } else {
        (locals.var_qbsld, locals.var_qbsld_dn0, locals.var_qbsld_dn2, locals.var_qbsld_dn4, locals.var_qbsld_dn5, locals.var_qbsld_dn6, locals.var_qbsld_dn7, locals.var_qbsld_dn8, locals.var_qbsld_dn9, locals.var_qbsld_dn10, locals.var_qbsld_dn13,)
    }
};
        locals.var_qbsld = assign85070_e130740;
        locals.var_qbsld_dn0 = assign85070_e130740_d_n0;
        locals.var_qbsld_dn2 = assign85070_e130740_d_n2;
        locals.var_qbsld_dn4 = assign85070_e130740_d_n4;
        locals.var_qbsld_dn5 = assign85070_e130740_d_n5;
        locals.var_qbsld_dn6 = assign85070_e130740_d_n6;
        locals.var_qbsld_dn7 = assign85070_e130740_d_n7;
        locals.var_qbsld_dn8 = assign85070_e130740_d_n8;
        locals.var_qbsld_dn9 = assign85070_e130740_d_n9;
        locals.var_qbsld_dn10 = assign85070_e130740_d_n10;
        locals.var_qbsld_dn13 = assign85070_e130740_d_n13;

        let (assign85100_e130765, assign85100_e130765_d_n0, assign85100_e130765_d_n2, assign85100_e130765_d_n4, assign85100_e130765_d_n5, assign85100_e130765_d_n6, assign85100_e130765_d_n7, assign85100_e130765_d_n8, assign85100_e130765_d_n9, assign85100_e130765_d_n10, assign85100_e130765_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && ((locals.var_guard1984 != 0.0) && (locals.var_guard1983 == 0.0))) {
        let assign85100_e130761: f64 = (locals.var_weffcv_nf * locals.var_lover_func);
        let assign85100_e130763: f64 = (assign85100_e130761 * locals.var_uc_cvdsover);
        (assign85100_e130763, ((locals.var_weffcv_nf * locals.var_lover_func_dn0) * locals.var_uc_cvdsover), ((locals.var_weffcv_nf * locals.var_lover_func_dn2) * locals.var_uc_cvdsover), ((locals.var_weffcv_nf * locals.var_lover_func_dn4) * locals.var_uc_cvdsover), ((locals.var_weffcv_nf * locals.var_lover_func_dn5) * locals.var_uc_cvdsover), ((locals.var_weffcv_nf * locals.var_lover_func_dn6) * locals.var_uc_cvdsover), ((locals.var_weffcv_nf * locals.var_lover_func_dn7) * locals.var_uc_cvdsover), ((locals.var_weffcv_nf * locals.var_lover_func_dn8) * locals.var_uc_cvdsover), ((locals.var_weffcv_nf * locals.var_lover_func_dn9) * locals.var_uc_cvdsover), ((locals.var_weffcv_nf * locals.var_lover_func_dn10) * locals.var_uc_cvdsover), ((locals.var_weffcv_nf * locals.var_lover_func_dn13) * locals.var_uc_cvdsover),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn13,)
    }
};
        locals.var_t4 = assign85100_e130765;
        locals.var_t4_dn0 = assign85100_e130765_d_n0;
        locals.var_t4_dn2 = assign85100_e130765_d_n2;
        locals.var_t4_dn4 = assign85100_e130765_d_n4;
        locals.var_t4_dn5 = assign85100_e130765_d_n5;
        locals.var_t4_dn6 = assign85100_e130765_d_n6;
        locals.var_t4_dn7 = assign85100_e130765_d_n7;
        locals.var_t4_dn8 = assign85100_e130765_d_n8;
        locals.var_t4_dn9 = assign85100_e130765_d_n9;
        locals.var_t4_dn10 = assign85100_e130765_d_n10;
        locals.var_t4_dn13 = assign85100_e130765_d_n13;

        let (assign85110_e130776, assign85110_e130776_d_n0, assign85110_e130776_d_n2, assign85110_e130776_d_n4, assign85110_e130776_d_n5, assign85110_e130776_d_n6, assign85110_e130776_d_n7, assign85110_e130776_d_n8, assign85110_e130776_d_n9, assign85110_e130776_d_n10, assign85110_e130776_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && ((locals.var_guard1984 != 0.0) && (locals.var_guard1983 == 0.0))) {
        let assign85110_e130774: f64 = (locals.var_t4 * locals.var_qsuld);
        (assign85110_e130774, ((locals.var_t4_dn0 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn0)), ((locals.var_t4_dn2 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn2)), ((locals.var_t4_dn4 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn4)), ((locals.var_t4_dn5 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn5)), ((locals.var_t4_dn6 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn6)), ((locals.var_t4_dn7 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn7)), ((locals.var_t4_dn8 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn8)), ((locals.var_t4_dn9 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn9)), ((locals.var_t4_dn10 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn10)), ((locals.var_t4_dn13 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn13)),)
    } else {
        (locals.var_qovsext, locals.var_qovsext_dn0, locals.var_qovsext_dn2, locals.var_qovsext_dn4, locals.var_qovsext_dn5, locals.var_qovsext_dn6, locals.var_qovsext_dn7, locals.var_qovsext_dn8, locals.var_qovsext_dn9, locals.var_qovsext_dn10, locals.var_qovsext_dn13,)
    }
};
        locals.var_qovsext = assign85110_e130776;
        locals.var_qovsext_dn0 = assign85110_e130776_d_n0;
        locals.var_qovsext_dn2 = assign85110_e130776_d_n2;
        locals.var_qovsext_dn4 = assign85110_e130776_d_n4;
        locals.var_qovsext_dn5 = assign85110_e130776_d_n5;
        locals.var_qovsext_dn6 = assign85110_e130776_d_n6;
        locals.var_qovsext_dn7 = assign85110_e130776_d_n7;
        locals.var_qovsext_dn8 = assign85110_e130776_d_n8;
        locals.var_qovsext_dn9 = assign85110_e130776_d_n9;
        locals.var_qovsext_dn10 = assign85110_e130776_d_n10;
        locals.var_qovsext_dn13 = assign85110_e130776_d_n13;

        let (assign85120_e130787, assign85120_e130787_d_n0, assign85120_e130787_d_n2, assign85120_e130787_d_n4, assign85120_e130787_d_n5, assign85120_e130787_d_n6, assign85120_e130787_d_n7, assign85120_e130787_d_n8, assign85120_e130787_d_n9, assign85120_e130787_d_n10, assign85120_e130787_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && ((locals.var_guard1984 != 0.0) && (locals.var_guard1983 == 0.0))) {
        let assign85120_e130785: f64 = (locals.var_t4 * locals.var_qbuld);
        (assign85120_e130785, ((locals.var_t4_dn0 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn0)), ((locals.var_t4_dn2 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn2)), ((locals.var_t4_dn4 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn4)), ((locals.var_t4_dn5 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn5)), ((locals.var_t4_dn6 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn6)), ((locals.var_t4_dn7 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn7)), ((locals.var_t4_dn8 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn8)), ((locals.var_t4_dn9 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn9)), ((locals.var_t4_dn10 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn10)), ((locals.var_t4_dn13 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn13)),)
    } else {
        (locals.var_qbsldext, locals.var_qbsldext_dn0, locals.var_qbsldext_dn2, locals.var_qbsldext_dn4, locals.var_qbsldext_dn5, locals.var_qbsldext_dn6, locals.var_qbsldext_dn7, locals.var_qbsldext_dn8, locals.var_qbsldext_dn9, locals.var_qbsldext_dn10, locals.var_qbsldext_dn13,)
    }
};
        locals.var_qbsldext = assign85120_e130787;
        locals.var_qbsldext_dn0 = assign85120_e130787_d_n0;
        locals.var_qbsldext_dn2 = assign85120_e130787_d_n2;
        locals.var_qbsldext_dn4 = assign85120_e130787_d_n4;
        locals.var_qbsldext_dn5 = assign85120_e130787_d_n5;
        locals.var_qbsldext_dn6 = assign85120_e130787_d_n6;
        locals.var_qbsldext_dn7 = assign85120_e130787_d_n7;
        locals.var_qbsldext_dn8 = assign85120_e130787_d_n8;
        locals.var_qbsldext_dn9 = assign85120_e130787_d_n9;
        locals.var_qbsldext_dn10 = assign85120_e130787_d_n10;
        locals.var_qbsldext_dn13 = assign85120_e130787_d_n13;

        let assign85130_e130790: f64 = if p.p55 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1988 = assign85130_e130790;

        let (assign85140_e130805, assign85140_e130805_d_n0, assign85140_e130805_d_n2, assign85140_e130805_d_n4, assign85140_e130805_d_n5, assign85140_e130805_d_n6, assign85140_e130805_d_n7, assign85140_e130805_d_n8, assign85140_e130805_d_n9, assign85140_e130805_d_n10, assign85140_e130805_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && ((locals.var_guard1985 != 0.0) && (!((locals.var_guard1983 != 0.0) || (locals.var_guard1984 != 0.0))))) && (locals.var_guard1988 != 0.0)) {
        let assign85140_e130803: f64 = (locals.var_weffcv_nf * locals.var_lover_func);
        (assign85140_e130803, (locals.var_weffcv_nf * locals.var_lover_func_dn0), (locals.var_weffcv_nf * locals.var_lover_func_dn2), (locals.var_weffcv_nf * locals.var_lover_func_dn4), (locals.var_weffcv_nf * locals.var_lover_func_dn5), (locals.var_weffcv_nf * locals.var_lover_func_dn6), (locals.var_weffcv_nf * locals.var_lover_func_dn7), (locals.var_weffcv_nf * locals.var_lover_func_dn8), (locals.var_weffcv_nf * locals.var_lover_func_dn9), (locals.var_weffcv_nf * locals.var_lover_func_dn10), (locals.var_weffcv_nf * locals.var_lover_func_dn13),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn13,)
    }
};
        locals.var_t4 = assign85140_e130805;
        locals.var_t4_dn0 = assign85140_e130805_d_n0;
        locals.var_t4_dn2 = assign85140_e130805_d_n2;
        locals.var_t4_dn4 = assign85140_e130805_d_n4;
        locals.var_t4_dn5 = assign85140_e130805_d_n5;
        locals.var_t4_dn6 = assign85140_e130805_d_n6;
        locals.var_t4_dn7 = assign85140_e130805_d_n7;
        locals.var_t4_dn8 = assign85140_e130805_d_n8;
        locals.var_t4_dn9 = assign85140_e130805_d_n9;
        locals.var_t4_dn10 = assign85140_e130805_d_n10;
        locals.var_t4_dn13 = assign85140_e130805_d_n13;

        let (assign85150_e130825, assign85150_e130825_d_n0, assign85150_e130825_d_n2, assign85150_e130825_d_n4, assign85150_e130825_d_n5, assign85150_e130825_d_n6, assign85150_e130825_d_n7, assign85150_e130825_d_n8, assign85150_e130825_d_n9, assign85150_e130825_d_n10, assign85150_e130825_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && ((locals.var_guard1985 != 0.0) && (!((locals.var_guard1983 != 0.0) || (locals.var_guard1984 != 0.0))))) && (locals.var_guard1988 == 0.0)) {
        let assign85150_e130819: f64 = (locals.var_weffcv_nf * locals.var_lover_func);
        let assign85150_e130822: f64 = (1.0 - locals.var_uc_cvdsover);
        let assign85150_e130823: f64 = (assign85150_e130819 * assign85150_e130822);
        (assign85150_e130823, ((locals.var_weffcv_nf * locals.var_lover_func_dn0) * assign85150_e130822), ((locals.var_weffcv_nf * locals.var_lover_func_dn2) * assign85150_e130822), ((locals.var_weffcv_nf * locals.var_lover_func_dn4) * assign85150_e130822), ((locals.var_weffcv_nf * locals.var_lover_func_dn5) * assign85150_e130822), ((locals.var_weffcv_nf * locals.var_lover_func_dn6) * assign85150_e130822), ((locals.var_weffcv_nf * locals.var_lover_func_dn7) * assign85150_e130822), ((locals.var_weffcv_nf * locals.var_lover_func_dn8) * assign85150_e130822), ((locals.var_weffcv_nf * locals.var_lover_func_dn9) * assign85150_e130822), ((locals.var_weffcv_nf * locals.var_lover_func_dn10) * assign85150_e130822), ((locals.var_weffcv_nf * locals.var_lover_func_dn13) * assign85150_e130822),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn13,)
    }
};
        locals.var_t4 = assign85150_e130825;
        locals.var_t4_dn0 = assign85150_e130825_d_n0;
        locals.var_t4_dn2 = assign85150_e130825_d_n2;
        locals.var_t4_dn4 = assign85150_e130825_d_n4;
        locals.var_t4_dn5 = assign85150_e130825_d_n5;
        locals.var_t4_dn6 = assign85150_e130825_d_n6;
        locals.var_t4_dn7 = assign85150_e130825_d_n7;
        locals.var_t4_dn8 = assign85150_e130825_d_n8;
        locals.var_t4_dn9 = assign85150_e130825_d_n9;
        locals.var_t4_dn10 = assign85150_e130825_d_n10;
        locals.var_t4_dn13 = assign85150_e130825_d_n13;

        let (assign85160_e130836, assign85160_e130836_d_n0, assign85160_e130836_d_n2, assign85160_e130836_d_n4, assign85160_e130836_d_n5, assign85160_e130836_d_n6, assign85160_e130836_d_n7, assign85160_e130836_d_n8, assign85160_e130836_d_n9, assign85160_e130836_d_n10, assign85160_e130836_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && ((locals.var_guard1985 != 0.0) && (!((locals.var_guard1983 != 0.0) || (locals.var_guard1984 != 0.0))))) {
        (locals.var_ps0ld, locals.var_ps0ld_dn0, locals.var_ps0ld_dn2, locals.var_ps0ld_dn4, locals.var_ps0ld_dn5, locals.var_ps0ld_dn6, locals.var_ps0ld_dn7, locals.var_ps0ld_dn8, locals.var_ps0ld_dn9, locals.var_ps0ld_dn10, locals.var_ps0ld_dn13,)
    } else {
        (locals.var_rd_ps0ld, locals.var_rd_ps0ld_dn0, locals.var_rd_ps0ld_dn2, locals.var_rd_ps0ld_dn4, locals.var_rd_ps0ld_dn5, locals.var_rd_ps0ld_dn6, locals.var_rd_ps0ld_dn7, locals.var_rd_ps0ld_dn8, locals.var_rd_ps0ld_dn9, locals.var_rd_ps0ld_dn10, locals.var_rd_ps0ld_dn13,)
    }
};
        locals.var_rd_ps0ld = assign85160_e130836;
        locals.var_rd_ps0ld_dn0 = assign85160_e130836_d_n0;
        locals.var_rd_ps0ld_dn2 = assign85160_e130836_d_n2;
        locals.var_rd_ps0ld_dn4 = assign85160_e130836_d_n4;
        locals.var_rd_ps0ld_dn5 = assign85160_e130836_d_n5;
        locals.var_rd_ps0ld_dn6 = assign85160_e130836_d_n6;
        locals.var_rd_ps0ld_dn7 = assign85160_e130836_d_n7;
        locals.var_rd_ps0ld_dn8 = assign85160_e130836_d_n8;
        locals.var_rd_ps0ld_dn9 = assign85160_e130836_d_n9;
        locals.var_rd_ps0ld_dn10 = assign85160_e130836_d_n10;
        locals.var_rd_ps0ld_dn13 = assign85160_e130836_d_n13;

        let assign85170_e130839: f64 = if p.p430 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1989 = assign85170_e130839;

        let (assign85180_e130852, assign85180_e130852_d_n0, assign85180_e130852_d_n2, assign85180_e130852_d_n4, assign85180_e130852_d_n5, assign85180_e130852_d_n6, assign85180_e130852_d_n7, assign85180_e130852_d_n8, assign85180_e130852_d_n9, assign85180_e130852_d_n10, assign85180_e130852_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && ((locals.var_guard1985 != 0.0) && (!((locals.var_guard1983 != 0.0) || (locals.var_guard1984 != 0.0))))) && (locals.var_guard1989 != 0.0)) {
        (locals.var_qbuld, locals.var_qbuld_dn0, locals.var_qbuld_dn2, locals.var_qbuld_dn4, locals.var_qbuld_dn5, locals.var_qbuld_dn6, locals.var_qbuld_dn7, locals.var_qbuld_dn8, locals.var_qbuld_dn9, locals.var_qbuld_dn10, locals.var_qbuld_dn13,)
    } else {
        (locals.var_rd_qbuld, locals.var_rd_qbuld_dn0, locals.var_rd_qbuld_dn2, locals.var_rd_qbuld_dn4, locals.var_rd_qbuld_dn5, locals.var_rd_qbuld_dn6, locals.var_rd_qbuld_dn7, locals.var_rd_qbuld_dn8, locals.var_rd_qbuld_dn9, locals.var_rd_qbuld_dn10, locals.var_rd_qbuld_dn13,)
    }
};
        locals.var_rd_qbuld = assign85180_e130852;
        locals.var_rd_qbuld_dn0 = assign85180_e130852_d_n0;
        locals.var_rd_qbuld_dn2 = assign85180_e130852_d_n2;
        locals.var_rd_qbuld_dn4 = assign85180_e130852_d_n4;
        locals.var_rd_qbuld_dn5 = assign85180_e130852_d_n5;
        locals.var_rd_qbuld_dn6 = assign85180_e130852_d_n6;
        locals.var_rd_qbuld_dn7 = assign85180_e130852_d_n7;
        locals.var_rd_qbuld_dn8 = assign85180_e130852_d_n8;
        locals.var_rd_qbuld_dn9 = assign85180_e130852_d_n9;
        locals.var_rd_qbuld_dn10 = assign85180_e130852_d_n10;
        locals.var_rd_qbuld_dn13 = assign85180_e130852_d_n13;

        let (assign85190_e130865, assign85190_e130865_d_n0, assign85190_e130865_d_n2, assign85190_e130865_d_n4, assign85190_e130865_d_n5, assign85190_e130865_d_n6, assign85190_e130865_d_n7, assign85190_e130865_d_n8, assign85190_e130865_d_n9, assign85190_e130865_d_n10, assign85190_e130865_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && ((locals.var_guard1985 != 0.0) && (!((locals.var_guard1983 != 0.0) || (locals.var_guard1984 != 0.0))))) {
        let assign85190_e130863: f64 = (locals.var_t4 * locals.var_qsuld);
        (assign85190_e130863, ((locals.var_t4_dn0 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn0)), ((locals.var_t4_dn2 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn2)), ((locals.var_t4_dn4 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn4)), ((locals.var_t4_dn5 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn5)), ((locals.var_t4_dn6 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn6)), ((locals.var_t4_dn7 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn7)), ((locals.var_t4_dn8 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn8)), ((locals.var_t4_dn9 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn9)), ((locals.var_t4_dn10 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn10)), ((locals.var_t4_dn13 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn13)),)
    } else {
        (locals.var_qovd, locals.var_qovd_dn0, locals.var_qovd_dn2, locals.var_qovd_dn4, locals.var_qovd_dn5, locals.var_qovd_dn6, locals.var_qovd_dn7, locals.var_qovd_dn8, locals.var_qovd_dn9, locals.var_qovd_dn10, locals.var_qovd_dn13,)
    }
};
        locals.var_qovd = assign85190_e130865;
        locals.var_qovd_dn0 = assign85190_e130865_d_n0;
        locals.var_qovd_dn2 = assign85190_e130865_d_n2;
        locals.var_qovd_dn4 = assign85190_e130865_d_n4;
        locals.var_qovd_dn5 = assign85190_e130865_d_n5;
        locals.var_qovd_dn6 = assign85190_e130865_d_n6;
        locals.var_qovd_dn7 = assign85190_e130865_d_n7;
        locals.var_qovd_dn8 = assign85190_e130865_d_n8;
        locals.var_qovd_dn9 = assign85190_e130865_d_n9;
        locals.var_qovd_dn10 = assign85190_e130865_d_n10;
        locals.var_qovd_dn13 = assign85190_e130865_d_n13;

        let (assign85200_e130878, assign85200_e130878_d_n0, assign85200_e130878_d_n2, assign85200_e130878_d_n4, assign85200_e130878_d_n5, assign85200_e130878_d_n6, assign85200_e130878_d_n7, assign85200_e130878_d_n8, assign85200_e130878_d_n9, assign85200_e130878_d_n10, assign85200_e130878_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && ((locals.var_guard1985 != 0.0) && (!((locals.var_guard1983 != 0.0) || (locals.var_guard1984 != 0.0))))) {
        let assign85200_e130876: f64 = (locals.var_t4 * locals.var_qbuld);
        (assign85200_e130876, ((locals.var_t4_dn0 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn0)), ((locals.var_t4_dn2 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn2)), ((locals.var_t4_dn4 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn4)), ((locals.var_t4_dn5 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn5)), ((locals.var_t4_dn6 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn6)), ((locals.var_t4_dn7 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn7)), ((locals.var_t4_dn8 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn8)), ((locals.var_t4_dn9 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn9)), ((locals.var_t4_dn10 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn10)), ((locals.var_t4_dn13 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn13)),)
    } else {
        (locals.var_qbdld, locals.var_qbdld_dn0, locals.var_qbdld_dn2, locals.var_qbdld_dn4, locals.var_qbdld_dn5, locals.var_qbdld_dn6, locals.var_qbdld_dn7, locals.var_qbdld_dn8, locals.var_qbdld_dn9, locals.var_qbdld_dn10, locals.var_qbdld_dn13,)
    }
};
        locals.var_qbdld = assign85200_e130878;
        locals.var_qbdld_dn0 = assign85200_e130878_d_n0;
        locals.var_qbdld_dn2 = assign85200_e130878_d_n2;
        locals.var_qbdld_dn4 = assign85200_e130878_d_n4;
        locals.var_qbdld_dn5 = assign85200_e130878_d_n5;
        locals.var_qbdld_dn6 = assign85200_e130878_d_n6;
        locals.var_qbdld_dn7 = assign85200_e130878_d_n7;
        locals.var_qbdld_dn8 = assign85200_e130878_d_n8;
        locals.var_qbdld_dn9 = assign85200_e130878_d_n9;
        locals.var_qbdld_dn10 = assign85200_e130878_d_n10;
        locals.var_qbdld_dn13 = assign85200_e130878_d_n13;

        let (assign85210_e130889, assign85210_e130889_d_n0, assign85210_e130889_d_n2, assign85210_e130889_d_n4, assign85210_e130889_d_n5, assign85210_e130889_d_n6, assign85210_e130889_d_n7, assign85210_e130889_d_n8, assign85210_e130889_d_n9, assign85210_e130889_d_n10, assign85210_e130889_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && ((locals.var_guard1985 != 0.0) && (!((locals.var_guard1983 != 0.0) || (locals.var_guard1984 != 0.0))))) {
        (locals.var_qbdld, locals.var_qbdld_dn0, locals.var_qbdld_dn2, locals.var_qbdld_dn4, locals.var_qbdld_dn5, locals.var_qbdld_dn6, locals.var_qbdld_dn7, locals.var_qbdld_dn8, locals.var_qbdld_dn9, locals.var_qbdld_dn10, locals.var_qbdld_dn13,)
    } else {
        (locals.var_qbd_qs, locals.var_qbd_qs_dn0, locals.var_qbd_qs_dn2, locals.var_qbd_qs_dn4, locals.var_qbd_qs_dn5, locals.var_qbd_qs_dn6, locals.var_qbd_qs_dn7, locals.var_qbd_qs_dn8, locals.var_qbd_qs_dn9, locals.var_qbd_qs_dn10, locals.var_qbd_qs_dn13,)
    }
};
        locals.var_qbd_qs = assign85210_e130889;
        locals.var_qbd_qs_dn0 = assign85210_e130889_d_n0;
        locals.var_qbd_qs_dn2 = assign85210_e130889_d_n2;
        locals.var_qbd_qs_dn4 = assign85210_e130889_d_n4;
        locals.var_qbd_qs_dn5 = assign85210_e130889_d_n5;
        locals.var_qbd_qs_dn6 = assign85210_e130889_d_n6;
        locals.var_qbd_qs_dn7 = assign85210_e130889_d_n7;
        locals.var_qbd_qs_dn8 = assign85210_e130889_d_n8;
        locals.var_qbd_qs_dn9 = assign85210_e130889_d_n9;
        locals.var_qbd_qs_dn10 = assign85210_e130889_d_n10;
        locals.var_qbd_qs_dn13 = assign85210_e130889_d_n13;

        let (assign85220_e130906, assign85220_e130906_d_n0, assign85220_e130906_d_n2, assign85220_e130906_d_n4, assign85220_e130906_d_n5, assign85220_e130906_d_n6, assign85220_e130906_d_n7, assign85220_e130906_d_n8, assign85220_e130906_d_n9, assign85220_e130906_d_n10, assign85220_e130906_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && ((locals.var_guard1986 != 0.0) && (!(((locals.var_guard1983 != 0.0) || (locals.var_guard1984 != 0.0)) || (locals.var_guard1985 != 0.0))))) {
        let assign85220_e130902: f64 = (locals.var_weffcv_nf * locals.var_lover_func);
        let assign85220_e130904: f64 = (assign85220_e130902 * locals.var_uc_cvdsover);
        (assign85220_e130904, ((locals.var_weffcv_nf * locals.var_lover_func_dn0) * locals.var_uc_cvdsover), ((locals.var_weffcv_nf * locals.var_lover_func_dn2) * locals.var_uc_cvdsover), ((locals.var_weffcv_nf * locals.var_lover_func_dn4) * locals.var_uc_cvdsover), ((locals.var_weffcv_nf * locals.var_lover_func_dn5) * locals.var_uc_cvdsover), ((locals.var_weffcv_nf * locals.var_lover_func_dn6) * locals.var_uc_cvdsover), ((locals.var_weffcv_nf * locals.var_lover_func_dn7) * locals.var_uc_cvdsover), ((locals.var_weffcv_nf * locals.var_lover_func_dn8) * locals.var_uc_cvdsover), ((locals.var_weffcv_nf * locals.var_lover_func_dn9) * locals.var_uc_cvdsover), ((locals.var_weffcv_nf * locals.var_lover_func_dn10) * locals.var_uc_cvdsover), ((locals.var_weffcv_nf * locals.var_lover_func_dn13) * locals.var_uc_cvdsover),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn13,)
    }
};
        locals.var_t4 = assign85220_e130906;
        locals.var_t4_dn0 = assign85220_e130906_d_n0;
        locals.var_t4_dn2 = assign85220_e130906_d_n2;
        locals.var_t4_dn4 = assign85220_e130906_d_n4;
        locals.var_t4_dn5 = assign85220_e130906_d_n5;
        locals.var_t4_dn6 = assign85220_e130906_d_n6;
        locals.var_t4_dn7 = assign85220_e130906_d_n7;
        locals.var_t4_dn8 = assign85220_e130906_d_n8;
        locals.var_t4_dn9 = assign85220_e130906_d_n9;
        locals.var_t4_dn10 = assign85220_e130906_d_n10;
        locals.var_t4_dn13 = assign85220_e130906_d_n13;

        let (assign85230_e130921, assign85230_e130921_d_n0, assign85230_e130921_d_n2, assign85230_e130921_d_n4, assign85230_e130921_d_n5, assign85230_e130921_d_n6, assign85230_e130921_d_n7, assign85230_e130921_d_n8, assign85230_e130921_d_n9, assign85230_e130921_d_n10, assign85230_e130921_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && ((locals.var_guard1986 != 0.0) && (!(((locals.var_guard1983 != 0.0) || (locals.var_guard1984 != 0.0)) || (locals.var_guard1985 != 0.0))))) {
        let assign85230_e130919: f64 = (locals.var_t4 * locals.var_qsuld);
        (assign85230_e130919, ((locals.var_t4_dn0 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn0)), ((locals.var_t4_dn2 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn2)), ((locals.var_t4_dn4 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn4)), ((locals.var_t4_dn5 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn5)), ((locals.var_t4_dn6 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn6)), ((locals.var_t4_dn7 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn7)), ((locals.var_t4_dn8 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn8)), ((locals.var_t4_dn9 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn9)), ((locals.var_t4_dn10 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn10)), ((locals.var_t4_dn13 * locals.var_qsuld) + (locals.var_t4 * locals.var_qsuld_dn13)),)
    } else {
        (locals.var_qovdext, locals.var_qovdext_dn0, locals.var_qovdext_dn2, locals.var_qovdext_dn4, locals.var_qovdext_dn5, locals.var_qovdext_dn6, locals.var_qovdext_dn7, locals.var_qovdext_dn8, locals.var_qovdext_dn9, locals.var_qovdext_dn10, locals.var_qovdext_dn13,)
    }
};
        locals.var_qovdext = assign85230_e130921;
        locals.var_qovdext_dn0 = assign85230_e130921_d_n0;
        locals.var_qovdext_dn2 = assign85230_e130921_d_n2;
        locals.var_qovdext_dn4 = assign85230_e130921_d_n4;
        locals.var_qovdext_dn5 = assign85230_e130921_d_n5;
        locals.var_qovdext_dn6 = assign85230_e130921_d_n6;
        locals.var_qovdext_dn7 = assign85230_e130921_d_n7;
        locals.var_qovdext_dn8 = assign85230_e130921_d_n8;
        locals.var_qovdext_dn9 = assign85230_e130921_d_n9;
        locals.var_qovdext_dn10 = assign85230_e130921_d_n10;
        locals.var_qovdext_dn13 = assign85230_e130921_d_n13;

        let (assign85240_e130936, assign85240_e130936_d_n0, assign85240_e130936_d_n2, assign85240_e130936_d_n4, assign85240_e130936_d_n5, assign85240_e130936_d_n6, assign85240_e130936_d_n7, assign85240_e130936_d_n8, assign85240_e130936_d_n9, assign85240_e130936_d_n10, assign85240_e130936_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && ((locals.var_guard1986 != 0.0) && (!(((locals.var_guard1983 != 0.0) || (locals.var_guard1984 != 0.0)) || (locals.var_guard1985 != 0.0))))) {
        let assign85240_e130934: f64 = (locals.var_t4 * locals.var_qbuld);
        (assign85240_e130934, ((locals.var_t4_dn0 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn0)), ((locals.var_t4_dn2 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn2)), ((locals.var_t4_dn4 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn4)), ((locals.var_t4_dn5 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn5)), ((locals.var_t4_dn6 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn6)), ((locals.var_t4_dn7 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn7)), ((locals.var_t4_dn8 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn8)), ((locals.var_t4_dn9 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn9)), ((locals.var_t4_dn10 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn10)), ((locals.var_t4_dn13 * locals.var_qbuld) + (locals.var_t4 * locals.var_qbuld_dn13)),)
    } else {
        (locals.var_qbdldext, locals.var_qbdldext_dn0, locals.var_qbdldext_dn2, locals.var_qbdldext_dn4, locals.var_qbdldext_dn5, locals.var_qbdldext_dn6, locals.var_qbdldext_dn7, locals.var_qbdldext_dn8, locals.var_qbdldext_dn9, locals.var_qbdldext_dn10, locals.var_qbdldext_dn13,)
    }
};
        locals.var_qbdldext = assign85240_e130936;
        locals.var_qbdldext_dn0 = assign85240_e130936_d_n0;
        locals.var_qbdldext_dn2 = assign85240_e130936_d_n2;
        locals.var_qbdldext_dn4 = assign85240_e130936_d_n4;
        locals.var_qbdldext_dn5 = assign85240_e130936_d_n5;
        locals.var_qbdldext_dn6 = assign85240_e130936_d_n6;
        locals.var_qbdldext_dn7 = assign85240_e130936_d_n7;
        locals.var_qbdldext_dn8 = assign85240_e130936_d_n8;
        locals.var_qbdldext_dn9 = assign85240_e130936_d_n9;
        locals.var_qbdldext_dn10 = assign85240_e130936_d_n10;
        locals.var_qbdldext_dn13 = assign85240_e130936_d_n13;

        locals.var_flg_calcqover = 0.0;

        let assign85260_e130940: f64 = if 4.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1990 = assign85260_e130940;

        let assign85270_e130943: f64 = if 4.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard1991 = assign85270_e130943;

        let assign85280_e130946: f64 = if 4.0 == 3.0 { 1.0 } else { 0.0 };
        locals.var_guard1992 = assign85280_e130946;

        let assign85290_e130949: f64 = if 4.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard1993 = assign85290_e130949;

        let assign85300_e130960: f64 = if (((p.p36 == 1.0) && (p.p66 > 0.0)) && (locals.var_uc_novers > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1994 = assign85300_e130960;

        let (assign85310_e130966,) = {
    if ((locals.var_guard1990 != 0.0) && (locals.var_guard1994 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_flg_calcqover,)
    }
};
        locals.var_flg_calcqover = assign85310_e130966;

        let (assign85320_e130972,) = {
    if ((locals.var_guard1990 != 0.0) && (locals.var_guard1994 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_flg_coovlps,)
    }
};
        locals.var_flg_coovlps = assign85320_e130972;

        let (assign85330_e130980, assign85330_e130980_d_n2, assign85330_e130980_d_n6, assign85330_e130980_d_n7, assign85330_e130980_d_n8,) = {
    if ((locals.var_guard1990 != 0.0) && (locals.var_guard1994 != 0.0)) {
        let assign85330_e130978: f64 = (locals.var_vgsi - locals.var_vbsi);
        (assign85330_e130978, 0.0, locals.var_vgsi_dn6, (locals.var_vgsi_dn7 - locals.var_vbsi_dn7), (-locals.var_vbsi_dn8),)
    } else {
        (locals.var_vgbgmt, locals.var_vgbgmt_dn2, locals.var_vgbgmt_dn6, locals.var_vgbgmt_dn7, locals.var_vgbgmt_dn8,)
    }
};
        locals.var_vgbgmt = assign85330_e130980;
        locals.var_vgbgmt_dn2 = assign85330_e130980_d_n2;
        locals.var_vgbgmt_dn6 = assign85330_e130980_d_n6;
        locals.var_vgbgmt_dn7 = assign85330_e130980_d_n7;
        locals.var_vgbgmt_dn8 = assign85330_e130980_d_n8;

        let (assign85340_e130987, assign85340_e130987_d_n0, assign85340_e130987_d_n2, assign85340_e130987_d_n4, assign85340_e130987_d_n5, assign85340_e130987_d_n6, assign85340_e130987_d_n7, assign85340_e130987_d_n8, assign85340_e130987_d_n9, assign85340_e130987_d_n10, assign85340_e130987_d_n13,) = {
    if ((locals.var_guard1990 != 0.0) && (locals.var_guard1994 != 0.0)) {
        let assign85340_e130985: f64 = (-locals.var_vbsi);
        (assign85340_e130985, 0.0, 0.0, 0.0, 0.0, 0.0, (-locals.var_vbsi_dn7), (-locals.var_vbsi_dn8), 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vxbgmt, locals.var_vxbgmt_dn0, locals.var_vxbgmt_dn2, locals.var_vxbgmt_dn4, locals.var_vxbgmt_dn5, locals.var_vxbgmt_dn6, locals.var_vxbgmt_dn7, locals.var_vxbgmt_dn8, locals.var_vxbgmt_dn9, locals.var_vxbgmt_dn10, locals.var_vxbgmt_dn13,)
    }
};
        locals.var_vxbgmt = assign85340_e130987;
        locals.var_vxbgmt_dn0 = assign85340_e130987_d_n0;
        locals.var_vxbgmt_dn2 = assign85340_e130987_d_n2;
        locals.var_vxbgmt_dn4 = assign85340_e130987_d_n4;
        locals.var_vxbgmt_dn5 = assign85340_e130987_d_n5;
        locals.var_vxbgmt_dn6 = assign85340_e130987_d_n6;
        locals.var_vxbgmt_dn7 = assign85340_e130987_d_n7;
        locals.var_vxbgmt_dn8 = assign85340_e130987_d_n8;
        locals.var_vxbgmt_dn9 = assign85340_e130987_d_n9;
        locals.var_vxbgmt_dn10 = assign85340_e130987_d_n10;
        locals.var_vxbgmt_dn13 = assign85340_e130987_d_n13;

        let (assign85350_e130993,) = {
    if ((locals.var_guard1990 != 0.0) && (locals.var_guard1994 != 0.0)) {
        (locals.var_uc_novers,)
    } else {
        (locals.var_nover_func,)
    }
};
        locals.var_nover_func = assign85350_e130993;

    }

    pub(super) fn stamp_transient_block_300(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign85360_e130999, assign85360_e130999_d_n0, assign85360_e130999_d_n2, assign85360_e130999_d_n4, assign85360_e130999_d_n5, assign85360_e130999_d_n6, assign85360_e130999_d_n7, assign85360_e130999_d_n8, assign85360_e130999_d_n9, assign85360_e130999_d_n10, assign85360_e130999_d_n13,) = {
    if ((locals.var_guard1990 != 0.0) && (locals.var_guard1994 != 0.0)) {
        (p.p66, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_lover_func, locals.var_lover_func_dn0, locals.var_lover_func_dn2, locals.var_lover_func_dn4, locals.var_lover_func_dn5, locals.var_lover_func_dn6, locals.var_lover_func_dn7, locals.var_lover_func_dn8, locals.var_lover_func_dn9, locals.var_lover_func_dn10, locals.var_lover_func_dn13,)
    }
};
        locals.var_lover_func = assign85360_e130999;
        locals.var_lover_func_dn0 = assign85360_e130999_d_n0;
        locals.var_lover_func_dn2 = assign85360_e130999_d_n2;
        locals.var_lover_func_dn4 = assign85360_e130999_d_n4;
        locals.var_lover_func_dn5 = assign85360_e130999_d_n5;
        locals.var_lover_func_dn6 = assign85360_e130999_d_n6;
        locals.var_lover_func_dn7 = assign85360_e130999_d_n7;
        locals.var_lover_func_dn8 = assign85360_e130999_d_n8;
        locals.var_lover_func_dn9 = assign85360_e130999_d_n9;
        locals.var_lover_func_dn10 = assign85360_e130999_d_n10;
        locals.var_lover_func_dn13 = assign85360_e130999_d_n13;

        let (assign85370_e131005, assign85370_e131005_d_n0, assign85370_e131005_d_n2, assign85370_e131005_d_n4, assign85370_e131005_d_n5, assign85370_e131005_d_n6, assign85370_e131005_d_n7, assign85370_e131005_d_n8, assign85370_e131005_d_n9, assign85370_e131005_d_n10, assign85370_e131005_d_n13,) = {
    if ((locals.var_guard1990 != 0.0) && (locals.var_guard1994 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_wdep_func, locals.var_wdep_func_dn0, locals.var_wdep_func_dn2, locals.var_wdep_func_dn4, locals.var_wdep_func_dn5, locals.var_wdep_func_dn6, locals.var_wdep_func_dn7, locals.var_wdep_func_dn8, locals.var_wdep_func_dn9, locals.var_wdep_func_dn10, locals.var_wdep_func_dn13,)
    }
};
        locals.var_wdep_func = assign85370_e131005;
        locals.var_wdep_func_dn0 = assign85370_e131005_d_n0;
        locals.var_wdep_func_dn2 = assign85370_e131005_d_n2;
        locals.var_wdep_func_dn4 = assign85370_e131005_d_n4;
        locals.var_wdep_func_dn5 = assign85370_e131005_d_n5;
        locals.var_wdep_func_dn6 = assign85370_e131005_d_n6;
        locals.var_wdep_func_dn7 = assign85370_e131005_d_n7;
        locals.var_wdep_func_dn8 = assign85370_e131005_d_n8;
        locals.var_wdep_func_dn9 = assign85370_e131005_d_n9;
        locals.var_wdep_func_dn10 = assign85370_e131005_d_n10;
        locals.var_wdep_func_dn13 = assign85370_e131005_d_n13;

        let (assign85380_e131011, assign85380_e131011_d_n0, assign85380_e131011_d_n2, assign85380_e131011_d_n4, assign85380_e131011_d_n5, assign85380_e131011_d_n6, assign85380_e131011_d_n7, assign85380_e131011_d_n8, assign85380_e131011_d_n9, assign85380_e131011_d_n10, assign85380_e131011_d_n13,) = {
    if ((locals.var_guard1990 != 0.0) && (locals.var_guard1994 != 0.0)) {
        (locals.var_cnst0overs, locals.var_cnst0overs_dn0, locals.var_cnst0overs_dn2, locals.var_cnst0overs_dn4, locals.var_cnst0overs_dn5, locals.var_cnst0overs_dn6, locals.var_cnst0overs_dn7, locals.var_cnst0overs_dn8, locals.var_cnst0overs_dn9, locals.var_cnst0overs_dn10, locals.var_cnst0overs_dn13,)
    } else {
        (locals.var_cnst0over_func, locals.var_cnst0over_func_dn0, locals.var_cnst0over_func_dn2, locals.var_cnst0over_func_dn4, locals.var_cnst0over_func_dn5, locals.var_cnst0over_func_dn6, locals.var_cnst0over_func_dn7, locals.var_cnst0over_func_dn8, locals.var_cnst0over_func_dn9, locals.var_cnst0over_func_dn10, locals.var_cnst0over_func_dn13,)
    }
};
        locals.var_cnst0over_func = assign85380_e131011;
        locals.var_cnst0over_func_dn0 = assign85380_e131011_d_n0;
        locals.var_cnst0over_func_dn2 = assign85380_e131011_d_n2;
        locals.var_cnst0over_func_dn4 = assign85380_e131011_d_n4;
        locals.var_cnst0over_func_dn5 = assign85380_e131011_d_n5;
        locals.var_cnst0over_func_dn6 = assign85380_e131011_d_n6;
        locals.var_cnst0over_func_dn7 = assign85380_e131011_d_n7;
        locals.var_cnst0over_func_dn8 = assign85380_e131011_d_n8;
        locals.var_cnst0over_func_dn9 = assign85380_e131011_d_n9;
        locals.var_cnst0over_func_dn10 = assign85380_e131011_d_n10;
        locals.var_cnst0over_func_dn13 = assign85380_e131011_d_n13;

        let (assign85390_e131017,) = {
    if ((locals.var_guard1990 != 0.0) && (locals.var_guard1994 != 0.0)) {
        (locals.var_cox0,)
    } else {
        (locals.var_cox0_func,)
    }
};
        locals.var_cox0_func = assign85390_e131017;

        let assign85400_e131036: f64 = if (((((p.p36 == 1.0) && (p.p66 > 0.0)) && (locals.var_uc_novers > 0.0)) && (locals.var_uc_cvdsover != 0.0)) && (p.p55 != 1.0)) { 1.0 } else { 0.0 };
        locals.var_guard1995 = assign85400_e131036;

        let (assign85410_e131045,) = {
    if (((locals.var_guard1991 != 0.0) && (locals.var_guard1990 == 0.0)) && (locals.var_guard1995 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_flg_calcqover,)
    }
};
        locals.var_flg_calcqover = assign85410_e131045;

        let (assign85420_e131056, assign85420_e131056_d_n2, assign85420_e131056_d_n6, assign85420_e131056_d_n7, assign85420_e131056_d_n8,) = {
    if (((locals.var_guard1991 != 0.0) && (locals.var_guard1990 == 0.0)) && (locals.var_guard1995 != 0.0)) {
        let assign85420_e131054: f64 = (locals.var_vgsei - locals.var_vbsei);
        (assign85420_e131054, (locals.var_vgsei_dn2 - locals.var_vbsei_dn2), locals.var_vgsei_dn6, 0.0, (-locals.var_vbsei_dn8),)
    } else {
        (locals.var_vgbgmt, locals.var_vgbgmt_dn2, locals.var_vgbgmt_dn6, locals.var_vgbgmt_dn7, locals.var_vgbgmt_dn8,)
    }
};
        locals.var_vgbgmt = assign85420_e131056;
        locals.var_vgbgmt_dn2 = assign85420_e131056_d_n2;
        locals.var_vgbgmt_dn6 = assign85420_e131056_d_n6;
        locals.var_vgbgmt_dn7 = assign85420_e131056_d_n7;
        locals.var_vgbgmt_dn8 = assign85420_e131056_d_n8;

        let (assign85430_e131066, assign85430_e131066_d_n0, assign85430_e131066_d_n2, assign85430_e131066_d_n4, assign85430_e131066_d_n5, assign85430_e131066_d_n6, assign85430_e131066_d_n7, assign85430_e131066_d_n8, assign85430_e131066_d_n9, assign85430_e131066_d_n10, assign85430_e131066_d_n13,) = {
    if (((locals.var_guard1991 != 0.0) && (locals.var_guard1990 == 0.0)) && (locals.var_guard1995 != 0.0)) {
        let assign85430_e131064: f64 = (-locals.var_vbsei);
        (assign85430_e131064, 0.0, (-locals.var_vbsei_dn2), 0.0, 0.0, 0.0, 0.0, (-locals.var_vbsei_dn8), 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vxbgmt, locals.var_vxbgmt_dn0, locals.var_vxbgmt_dn2, locals.var_vxbgmt_dn4, locals.var_vxbgmt_dn5, locals.var_vxbgmt_dn6, locals.var_vxbgmt_dn7, locals.var_vxbgmt_dn8, locals.var_vxbgmt_dn9, locals.var_vxbgmt_dn10, locals.var_vxbgmt_dn13,)
    }
};
        locals.var_vxbgmt = assign85430_e131066;
        locals.var_vxbgmt_dn0 = assign85430_e131066_d_n0;
        locals.var_vxbgmt_dn2 = assign85430_e131066_d_n2;
        locals.var_vxbgmt_dn4 = assign85430_e131066_d_n4;
        locals.var_vxbgmt_dn5 = assign85430_e131066_d_n5;
        locals.var_vxbgmt_dn6 = assign85430_e131066_d_n6;
        locals.var_vxbgmt_dn7 = assign85430_e131066_d_n7;
        locals.var_vxbgmt_dn8 = assign85430_e131066_d_n8;
        locals.var_vxbgmt_dn9 = assign85430_e131066_d_n9;
        locals.var_vxbgmt_dn10 = assign85430_e131066_d_n10;
        locals.var_vxbgmt_dn13 = assign85430_e131066_d_n13;

        let assign85440_e131077: f64 = if (((p.p35 == 1.0) && (p.p63 > 0.0)) && (locals.var_uc_nover > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1996 = assign85440_e131077;

        let (assign85450_e131088,) = {
    if (((locals.var_guard1992 != 0.0) && (!((locals.var_guard1990 != 0.0) || (locals.var_guard1991 != 0.0)))) && (locals.var_guard1996 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_flg_calcqover,)
    }
};
        locals.var_flg_calcqover = assign85450_e131088;

        let (assign85460_e131099,) = {
    if (((locals.var_guard1992 != 0.0) && (!((locals.var_guard1990 != 0.0) || (locals.var_guard1991 != 0.0)))) && (locals.var_guard1996 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_flg_coovlp,)
    }
};
        locals.var_flg_coovlp = assign85460_e131099;

        let (assign85470_e131112, assign85470_e131112_d_n2, assign85470_e131112_d_n6, assign85470_e131112_d_n7, assign85470_e131112_d_n8,) = {
    if (((locals.var_guard1992 != 0.0) && (!((locals.var_guard1990 != 0.0) || (locals.var_guard1991 != 0.0)))) && (locals.var_guard1996 != 0.0)) {
        let assign85470_e131110: f64 = (locals.var_vgsi - locals.var_vbsi);
        (assign85470_e131110, 0.0, locals.var_vgsi_dn6, (locals.var_vgsi_dn7 - locals.var_vbsi_dn7), (-locals.var_vbsi_dn8),)
    } else {
        (locals.var_vgbgmt, locals.var_vgbgmt_dn2, locals.var_vgbgmt_dn6, locals.var_vgbgmt_dn7, locals.var_vgbgmt_dn8,)
    }
};
        locals.var_vgbgmt = assign85470_e131112;
        locals.var_vgbgmt_dn2 = assign85470_e131112_d_n2;
        locals.var_vgbgmt_dn6 = assign85470_e131112_d_n6;
        locals.var_vgbgmt_dn7 = assign85470_e131112_d_n7;
        locals.var_vgbgmt_dn8 = assign85470_e131112_d_n8;

        let (assign85480_e131125, assign85480_e131125_d_n0, assign85480_e131125_d_n2, assign85480_e131125_d_n4, assign85480_e131125_d_n5, assign85480_e131125_d_n6, assign85480_e131125_d_n7, assign85480_e131125_d_n8, assign85480_e131125_d_n9, assign85480_e131125_d_n10, assign85480_e131125_d_n13,) = {
    if (((locals.var_guard1992 != 0.0) && (!((locals.var_guard1990 != 0.0) || (locals.var_guard1991 != 0.0)))) && (locals.var_guard1996 != 0.0)) {
        let assign85480_e131123: f64 = (locals.var_vdsi - locals.var_vbsi);
        (assign85480_e131123, 0.0, 0.0, 0.0, locals.var_vdsi_dn5, 0.0, (locals.var_vdsi_dn7 - locals.var_vbsi_dn7), (-locals.var_vbsi_dn8), 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vxbgmt, locals.var_vxbgmt_dn0, locals.var_vxbgmt_dn2, locals.var_vxbgmt_dn4, locals.var_vxbgmt_dn5, locals.var_vxbgmt_dn6, locals.var_vxbgmt_dn7, locals.var_vxbgmt_dn8, locals.var_vxbgmt_dn9, locals.var_vxbgmt_dn10, locals.var_vxbgmt_dn13,)
    }
};
        locals.var_vxbgmt = assign85480_e131125;
        locals.var_vxbgmt_dn0 = assign85480_e131125_d_n0;
        locals.var_vxbgmt_dn2 = assign85480_e131125_d_n2;
        locals.var_vxbgmt_dn4 = assign85480_e131125_d_n4;
        locals.var_vxbgmt_dn5 = assign85480_e131125_d_n5;
        locals.var_vxbgmt_dn6 = assign85480_e131125_d_n6;
        locals.var_vxbgmt_dn7 = assign85480_e131125_d_n7;
        locals.var_vxbgmt_dn8 = assign85480_e131125_d_n8;
        locals.var_vxbgmt_dn9 = assign85480_e131125_d_n9;
        locals.var_vxbgmt_dn10 = assign85480_e131125_d_n10;
        locals.var_vxbgmt_dn13 = assign85480_e131125_d_n13;

        let (assign85490_e131136,) = {
    if (((locals.var_guard1992 != 0.0) && (!((locals.var_guard1990 != 0.0) || (locals.var_guard1991 != 0.0)))) && (locals.var_guard1996 != 0.0)) {
        (locals.var_uc_nover,)
    } else {
        (locals.var_nover_func,)
    }
};
        locals.var_nover_func = assign85490_e131136;

        let (assign85500_e131151, assign85500_e131151_d_n0, assign85500_e131151_d_n2, assign85500_e131151_d_n4, assign85500_e131151_d_n5, assign85500_e131151_d_n6, assign85500_e131151_d_n7, assign85500_e131151_d_n8, assign85500_e131151_d_n9, assign85500_e131151_d_n10, assign85500_e131151_d_n13,) = {
    if (((locals.var_guard1992 != 0.0) && (!((locals.var_guard1990 != 0.0) || (locals.var_guard1991 != 0.0)))) && (locals.var_guard1996 != 0.0)) {
        let assign85500_e131148: f64 = (p.p64 * p.p55);
        let assign85500_e131149: f64 = (p.p63 + assign85500_e131148);
        (assign85500_e131149, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_lover_func, locals.var_lover_func_dn0, locals.var_lover_func_dn2, locals.var_lover_func_dn4, locals.var_lover_func_dn5, locals.var_lover_func_dn6, locals.var_lover_func_dn7, locals.var_lover_func_dn8, locals.var_lover_func_dn9, locals.var_lover_func_dn10, locals.var_lover_func_dn13,)
    }
};
        locals.var_lover_func = assign85500_e131151;
        locals.var_lover_func_dn0 = assign85500_e131151_d_n0;
        locals.var_lover_func_dn2 = assign85500_e131151_d_n2;
        locals.var_lover_func_dn4 = assign85500_e131151_d_n4;
        locals.var_lover_func_dn5 = assign85500_e131151_d_n5;
        locals.var_lover_func_dn6 = assign85500_e131151_d_n6;
        locals.var_lover_func_dn7 = assign85500_e131151_d_n7;
        locals.var_lover_func_dn8 = assign85500_e131151_d_n8;
        locals.var_lover_func_dn9 = assign85500_e131151_d_n9;
        locals.var_lover_func_dn10 = assign85500_e131151_d_n10;
        locals.var_lover_func_dn13 = assign85500_e131151_d_n13;

        let (assign85510_e131162, assign85510_e131162_d_n0, assign85510_e131162_d_n2, assign85510_e131162_d_n4, assign85510_e131162_d_n5, assign85510_e131162_d_n6, assign85510_e131162_d_n7, assign85510_e131162_d_n8, assign85510_e131162_d_n9, assign85510_e131162_d_n10, assign85510_e131162_d_n13,) = {
    if (((locals.var_guard1992 != 0.0) && (!((locals.var_guard1990 != 0.0) || (locals.var_guard1991 != 0.0)))) && (locals.var_guard1996 != 0.0)) {
        (locals.var_wdep, locals.var_wdep_dn0, locals.var_wdep_dn2, locals.var_wdep_dn4, locals.var_wdep_dn5, locals.var_wdep_dn6, locals.var_wdep_dn7, locals.var_wdep_dn8, locals.var_wdep_dn9, locals.var_wdep_dn10, locals.var_wdep_dn13,)
    } else {
        (locals.var_wdep_func, locals.var_wdep_func_dn0, locals.var_wdep_func_dn2, locals.var_wdep_func_dn4, locals.var_wdep_func_dn5, locals.var_wdep_func_dn6, locals.var_wdep_func_dn7, locals.var_wdep_func_dn8, locals.var_wdep_func_dn9, locals.var_wdep_func_dn10, locals.var_wdep_func_dn13,)
    }
};
        locals.var_wdep_func = assign85510_e131162;
        locals.var_wdep_func_dn0 = assign85510_e131162_d_n0;
        locals.var_wdep_func_dn2 = assign85510_e131162_d_n2;
        locals.var_wdep_func_dn4 = assign85510_e131162_d_n4;
        locals.var_wdep_func_dn5 = assign85510_e131162_d_n5;
        locals.var_wdep_func_dn6 = assign85510_e131162_d_n6;
        locals.var_wdep_func_dn7 = assign85510_e131162_d_n7;
        locals.var_wdep_func_dn8 = assign85510_e131162_d_n8;
        locals.var_wdep_func_dn9 = assign85510_e131162_d_n9;
        locals.var_wdep_func_dn10 = assign85510_e131162_d_n10;
        locals.var_wdep_func_dn13 = assign85510_e131162_d_n13;

        let (assign85520_e131173, assign85520_e131173_d_n0, assign85520_e131173_d_n2, assign85520_e131173_d_n4, assign85520_e131173_d_n5, assign85520_e131173_d_n6, assign85520_e131173_d_n7, assign85520_e131173_d_n8, assign85520_e131173_d_n9, assign85520_e131173_d_n10, assign85520_e131173_d_n13,) = {
    if (((locals.var_guard1992 != 0.0) && (!((locals.var_guard1990 != 0.0) || (locals.var_guard1991 != 0.0)))) && (locals.var_guard1996 != 0.0)) {
        (locals.var_cnst0over, locals.var_cnst0over_dn0, locals.var_cnst0over_dn2, locals.var_cnst0over_dn4, locals.var_cnst0over_dn5, locals.var_cnst0over_dn6, locals.var_cnst0over_dn7, locals.var_cnst0over_dn8, locals.var_cnst0over_dn9, locals.var_cnst0over_dn10, locals.var_cnst0over_dn13,)
    } else {
        (locals.var_cnst0over_func, locals.var_cnst0over_func_dn0, locals.var_cnst0over_func_dn2, locals.var_cnst0over_func_dn4, locals.var_cnst0over_func_dn5, locals.var_cnst0over_func_dn6, locals.var_cnst0over_func_dn7, locals.var_cnst0over_func_dn8, locals.var_cnst0over_func_dn9, locals.var_cnst0over_func_dn10, locals.var_cnst0over_func_dn13,)
    }
};
        locals.var_cnst0over_func = assign85520_e131173;
        locals.var_cnst0over_func_dn0 = assign85520_e131173_d_n0;
        locals.var_cnst0over_func_dn2 = assign85520_e131173_d_n2;
        locals.var_cnst0over_func_dn4 = assign85520_e131173_d_n4;
        locals.var_cnst0over_func_dn5 = assign85520_e131173_d_n5;
        locals.var_cnst0over_func_dn6 = assign85520_e131173_d_n6;
        locals.var_cnst0over_func_dn7 = assign85520_e131173_d_n7;
        locals.var_cnst0over_func_dn8 = assign85520_e131173_d_n8;
        locals.var_cnst0over_func_dn9 = assign85520_e131173_d_n9;
        locals.var_cnst0over_func_dn10 = assign85520_e131173_d_n10;
        locals.var_cnst0over_func_dn13 = assign85520_e131173_d_n13;

        let (assign85530_e131184,) = {
    if (((locals.var_guard1992 != 0.0) && (!((locals.var_guard1990 != 0.0) || (locals.var_guard1991 != 0.0)))) && (locals.var_guard1996 != 0.0)) {
        (locals.var_coxb0,)
    } else {
        (locals.var_cox0_func,)
    }
};
        locals.var_cox0_func = assign85530_e131184;

        let (assign85540_e131196, assign85540_e131196_d_n0, assign85540_e131196_d_n2, assign85540_e131196_d_n4, assign85540_e131196_d_n5, assign85540_e131196_d_n6, assign85540_e131196_d_n7, assign85540_e131196_d_n8, assign85540_e131196_d_n9, assign85540_e131196_d_n10, assign85540_e131196_d_n13,) = {
    if (((locals.var_guard1992 != 0.0) && (!((locals.var_guard1990 != 0.0) || (locals.var_guard1991 != 0.0)))) && (locals.var_guard1996 != 0.0)) {
        let assign85540_e131194: f64 = (-locals.var_lover_func);
        (assign85540_e131194, (-locals.var_lover_func_dn0), (-locals.var_lover_func_dn2), (-locals.var_lover_func_dn4), (-locals.var_lover_func_dn5), (-locals.var_lover_func_dn6), (-locals.var_lover_func_dn7), (-locals.var_lover_func_dn8), (-locals.var_lover_func_dn9), (-locals.var_lover_func_dn10), (-locals.var_lover_func_dn13),)
    } else {
        (locals.var_lover_func, locals.var_lover_func_dn0, locals.var_lover_func_dn2, locals.var_lover_func_dn4, locals.var_lover_func_dn5, locals.var_lover_func_dn6, locals.var_lover_func_dn7, locals.var_lover_func_dn8, locals.var_lover_func_dn9, locals.var_lover_func_dn10, locals.var_lover_func_dn13,)
    }
};
        locals.var_lover_func = assign85540_e131196;
        locals.var_lover_func_dn0 = assign85540_e131196_d_n0;
        locals.var_lover_func_dn2 = assign85540_e131196_d_n2;
        locals.var_lover_func_dn4 = assign85540_e131196_d_n4;
        locals.var_lover_func_dn5 = assign85540_e131196_d_n5;
        locals.var_lover_func_dn6 = assign85540_e131196_d_n6;
        locals.var_lover_func_dn7 = assign85540_e131196_d_n7;
        locals.var_lover_func_dn8 = assign85540_e131196_d_n8;
        locals.var_lover_func_dn9 = assign85540_e131196_d_n9;
        locals.var_lover_func_dn10 = assign85540_e131196_d_n10;
        locals.var_lover_func_dn13 = assign85540_e131196_d_n13;

        let assign85550_e131207: f64 = if (((locals.var_lover_func < 0.0) && (p.p432 > 0.0)) && (p.p55 == 1.0)) { 1.0 } else { 0.0 };
        locals.var_guard1997 = assign85550_e131207;

        let (assign85560_e131221, assign85560_e131221_d_n0, assign85560_e131221_d_n2, assign85560_e131221_d_n4, assign85560_e131221_d_n5, assign85560_e131221_d_n6, assign85560_e131221_d_n7, assign85560_e131221_d_n8, assign85560_e131221_d_n9, assign85560_e131221_d_n10, assign85560_e131221_d_n13,) = {
    if ((((locals.var_guard1992 != 0.0) && (!((locals.var_guard1990 != 0.0) || (locals.var_guard1991 != 0.0)))) && (locals.var_guard1996 != 0.0)) && (locals.var_guard1997 != 0.0)) {
        let assign85560_e131219: f64 = (-locals.var_lover_func);
        (assign85560_e131219, (-locals.var_lover_func_dn0), (-locals.var_lover_func_dn2), (-locals.var_lover_func_dn4), (-locals.var_lover_func_dn5), (-locals.var_lover_func_dn6), (-locals.var_lover_func_dn7), (-locals.var_lover_func_dn8), (-locals.var_lover_func_dn9), (-locals.var_lover_func_dn10), (-locals.var_lover_func_dn13),)
    } else {
        (locals.var_lover_func, locals.var_lover_func_dn0, locals.var_lover_func_dn2, locals.var_lover_func_dn4, locals.var_lover_func_dn5, locals.var_lover_func_dn6, locals.var_lover_func_dn7, locals.var_lover_func_dn8, locals.var_lover_func_dn9, locals.var_lover_func_dn10, locals.var_lover_func_dn13,)
    }
};
        locals.var_lover_func = assign85560_e131221;
        locals.var_lover_func_dn0 = assign85560_e131221_d_n0;
        locals.var_lover_func_dn2 = assign85560_e131221_d_n2;
        locals.var_lover_func_dn4 = assign85560_e131221_d_n4;
        locals.var_lover_func_dn5 = assign85560_e131221_d_n5;
        locals.var_lover_func_dn6 = assign85560_e131221_d_n6;
        locals.var_lover_func_dn7 = assign85560_e131221_d_n7;
        locals.var_lover_func_dn8 = assign85560_e131221_d_n8;
        locals.var_lover_func_dn9 = assign85560_e131221_d_n9;
        locals.var_lover_func_dn10 = assign85560_e131221_d_n10;
        locals.var_lover_func_dn13 = assign85560_e131221_d_n13;

        let (assign85570_e131234, assign85570_e131234_d_n0, assign85570_e131234_d_n2, assign85570_e131234_d_n4, assign85570_e131234_d_n5, assign85570_e131234_d_n6, assign85570_e131234_d_n7, assign85570_e131234_d_n8, assign85570_e131234_d_n9, assign85570_e131234_d_n10, assign85570_e131234_d_n13,) = {
    if ((((locals.var_guard1992 != 0.0) && (!((locals.var_guard1990 != 0.0) || (locals.var_guard1991 != 0.0)))) && (locals.var_guard1996 != 0.0)) && (locals.var_guard1997 != 0.0)) {
        (p.p63, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign85570_e131234;
        locals.var_t1_dn0 = assign85570_e131234_d_n0;
        locals.var_t1_dn2 = assign85570_e131234_d_n2;
        locals.var_t1_dn4 = assign85570_e131234_d_n4;
        locals.var_t1_dn5 = assign85570_e131234_d_n5;
        locals.var_t1_dn6 = assign85570_e131234_d_n6;
        locals.var_t1_dn7 = assign85570_e131234_d_n7;
        locals.var_t1_dn8 = assign85570_e131234_d_n8;
        locals.var_t1_dn9 = assign85570_e131234_d_n9;
        locals.var_t1_dn10 = assign85570_e131234_d_n10;
        locals.var_t1_dn13 = assign85570_e131234_d_n13;

        let (assign85580_e131253, assign85580_e131253_d_n0, assign85580_e131253_d_n2, assign85580_e131253_d_n4, assign85580_e131253_d_n5, assign85580_e131253_d_n6, assign85580_e131253_d_n7, assign85580_e131253_d_n8, assign85580_e131253_d_n9, assign85580_e131253_d_n10, assign85580_e131253_d_n13,) = {
    if ((((locals.var_guard1992 != 0.0) && (!((locals.var_guard1990 != 0.0) || (locals.var_guard1991 != 0.0)))) && (locals.var_guard1996 != 0.0)) && (locals.var_guard1997 != 0.0)) {
        let assign85580_e131247: f64 = (locals.var_t1 * locals.var_t1);
        let assign85580_e131249: f64 = (assign85580_e131247 / locals.var_kjunc);
        let assign85580_e131251: f64 = (assign85580_e131249 - p.p137);
        (assign85580_e131251, (((((locals.var_t1_dn0 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn0)) * locals.var_kjunc) - (assign85580_e131247 * locals.var_kjunc_dn0)) / (locals.var_kjunc * locals.var_kjunc)), (((((locals.var_t1_dn2 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn2)) * locals.var_kjunc) - (assign85580_e131247 * locals.var_kjunc_dn2)) / (locals.var_kjunc * locals.var_kjunc)), (((((locals.var_t1_dn4 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn4)) * locals.var_kjunc) - (assign85580_e131247 * locals.var_kjunc_dn4)) / (locals.var_kjunc * locals.var_kjunc)), (((((locals.var_t1_dn5 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn5)) * locals.var_kjunc) - (assign85580_e131247 * locals.var_kjunc_dn5)) / (locals.var_kjunc * locals.var_kjunc)), (((((locals.var_t1_dn6 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn6)) * locals.var_kjunc) - (assign85580_e131247 * locals.var_kjunc_dn6)) / (locals.var_kjunc * locals.var_kjunc)), (((((locals.var_t1_dn7 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn7)) * locals.var_kjunc) - (assign85580_e131247 * locals.var_kjunc_dn7)) / (locals.var_kjunc * locals.var_kjunc)), (((((locals.var_t1_dn8 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn8)) * locals.var_kjunc) - (assign85580_e131247 * locals.var_kjunc_dn8)) / (locals.var_kjunc * locals.var_kjunc)), (((((locals.var_t1_dn9 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn9)) * locals.var_kjunc) - (assign85580_e131247 * locals.var_kjunc_dn9)) / (locals.var_kjunc * locals.var_kjunc)), (((((locals.var_t1_dn10 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn10)) * locals.var_kjunc) - (assign85580_e131247 * locals.var_kjunc_dn10)) / (locals.var_kjunc * locals.var_kjunc)), (((((locals.var_t1_dn13 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn13)) * locals.var_kjunc) - (assign85580_e131247 * locals.var_kjunc_dn13)) / (locals.var_kjunc * locals.var_kjunc)),)
    } else {
        (locals.var_vxb_lim, locals.var_vxb_lim_dn0, locals.var_vxb_lim_dn2, locals.var_vxb_lim_dn4, locals.var_vxb_lim_dn5, locals.var_vxb_lim_dn6, locals.var_vxb_lim_dn7, locals.var_vxb_lim_dn8, locals.var_vxb_lim_dn9, locals.var_vxb_lim_dn10, locals.var_vxb_lim_dn13,)
    }
};
        locals.var_vxb_lim = assign85580_e131253;
        locals.var_vxb_lim_dn0 = assign85580_e131253_d_n0;
        locals.var_vxb_lim_dn2 = assign85580_e131253_d_n2;
        locals.var_vxb_lim_dn4 = assign85580_e131253_d_n4;
        locals.var_vxb_lim_dn5 = assign85580_e131253_d_n5;
        locals.var_vxb_lim_dn6 = assign85580_e131253_d_n6;
        locals.var_vxb_lim_dn7 = assign85580_e131253_d_n7;
        locals.var_vxb_lim_dn8 = assign85580_e131253_d_n8;
        locals.var_vxb_lim_dn9 = assign85580_e131253_d_n9;
        locals.var_vxb_lim_dn10 = assign85580_e131253_d_n10;
        locals.var_vxb_lim_dn13 = assign85580_e131253_d_n13;

        let assign85590_e131256: f64 = if p.p113 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1998 = assign85590_e131256;

        let assign85600_e131263: f64 = if ((locals.var_vxbgmt == 0.0) || (p.p113 <= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1999 = assign85600_e131263;

        let (assign85610_e131280, assign85610_e131280_d_n0, assign85610_e131280_d_n2, assign85610_e131280_d_n4, assign85610_e131280_d_n5, assign85610_e131280_d_n6, assign85610_e131280_d_n7, assign85610_e131280_d_n8, assign85610_e131280_d_n9, assign85610_e131280_d_n10, assign85610_e131280_d_n13,) = {
    if ((((((locals.var_guard1992 != 0.0) && (!((locals.var_guard1990 != 0.0) || (locals.var_guard1991 != 0.0)))) && (locals.var_guard1996 != 0.0)) && (locals.var_guard1997 != 0.0)) && (locals.var_guard1998 != 0.0)) && (locals.var_guard1999 != 0.0)) {
        (locals.var_vxbgmt, locals.var_vxbgmt_dn0, locals.var_vxbgmt_dn2, locals.var_vxbgmt_dn4, locals.var_vxbgmt_dn5, locals.var_vxbgmt_dn6, locals.var_vxbgmt_dn7, locals.var_vxbgmt_dn8, locals.var_vxbgmt_dn9, locals.var_vxbgmt_dn10, locals.var_vxbgmt_dn13,)
    } else {
        (locals.var_vxbgmt, locals.var_vxbgmt_dn0, locals.var_vxbgmt_dn2, locals.var_vxbgmt_dn4, locals.var_vxbgmt_dn5, locals.var_vxbgmt_dn6, locals.var_vxbgmt_dn7, locals.var_vxbgmt_dn8, locals.var_vxbgmt_dn9, locals.var_vxbgmt_dn10, locals.var_vxbgmt_dn13,)
    }
};
        locals.var_vxbgmt = assign85610_e131280;
        locals.var_vxbgmt_dn0 = assign85610_e131280_d_n0;
        locals.var_vxbgmt_dn2 = assign85610_e131280_d_n2;
        locals.var_vxbgmt_dn4 = assign85610_e131280_d_n4;
        locals.var_vxbgmt_dn5 = assign85610_e131280_d_n5;
        locals.var_vxbgmt_dn6 = assign85610_e131280_d_n6;
        locals.var_vxbgmt_dn7 = assign85610_e131280_d_n7;
        locals.var_vxbgmt_dn8 = assign85610_e131280_d_n8;
        locals.var_vxbgmt_dn9 = assign85610_e131280_d_n9;
        locals.var_vxbgmt_dn10 = assign85610_e131280_d_n10;
        locals.var_vxbgmt_dn13 = assign85610_e131280_d_n13;

        let (assign85620_e131304, assign85620_e131304_d_n0, assign85620_e131304_d_n2, assign85620_e131304_d_n4, assign85620_e131304_d_n5, assign85620_e131304_d_n6, assign85620_e131304_d_n7, assign85620_e131304_d_n8, assign85620_e131304_d_n9, assign85620_e131304_d_n10, assign85620_e131304_d_n13,) = {
    if ((((((locals.var_guard1992 != 0.0) && (!((locals.var_guard1990 != 0.0) || (locals.var_guard1991 != 0.0)))) && (locals.var_guard1996 != 0.0)) && (locals.var_guard1997 != 0.0)) && (locals.var_guard1998 != 0.0)) && (locals.var_guard1999 == 0.0)) {
        let (assign85620_e131302,) = {
            if (locals.var_vxbgmt < 0.0) {
                let assign85620_e131300: f64 = (-1.0);
                (assign85620_e131300,)
            } else {
                (1.0,)
            }
        };
        (assign85620_e131302, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf3, locals.var_tmf3_dn0, locals.var_tmf3_dn2, locals.var_tmf3_dn4, locals.var_tmf3_dn5, locals.var_tmf3_dn6, locals.var_tmf3_dn7, locals.var_tmf3_dn8, locals.var_tmf3_dn9, locals.var_tmf3_dn10, locals.var_tmf3_dn13,)
    }
};
        locals.var_tmf3 = assign85620_e131304;
        locals.var_tmf3_dn0 = assign85620_e131304_d_n0;
        locals.var_tmf3_dn2 = assign85620_e131304_d_n2;
        locals.var_tmf3_dn4 = assign85620_e131304_d_n4;
        locals.var_tmf3_dn5 = assign85620_e131304_d_n5;
        locals.var_tmf3_dn6 = assign85620_e131304_d_n6;
        locals.var_tmf3_dn7 = assign85620_e131304_d_n7;
        locals.var_tmf3_dn8 = assign85620_e131304_d_n8;
        locals.var_tmf3_dn9 = assign85620_e131304_d_n9;
        locals.var_tmf3_dn10 = assign85620_e131304_d_n10;
        locals.var_tmf3_dn13 = assign85620_e131304_d_n13;

        let (assign85630_e131324, assign85630_e131324_d_n0, assign85630_e131324_d_n2, assign85630_e131324_d_n4, assign85630_e131324_d_n5, assign85630_e131324_d_n6, assign85630_e131324_d_n7, assign85630_e131324_d_n8, assign85630_e131324_d_n9, assign85630_e131324_d_n10, assign85630_e131324_d_n13,) = {
    if ((((((locals.var_guard1992 != 0.0) && (!((locals.var_guard1990 != 0.0) || (locals.var_guard1991 != 0.0)))) && (locals.var_guard1996 != 0.0)) && (locals.var_guard1997 != 0.0)) && (locals.var_guard1998 != 0.0)) && (locals.var_guard1999 == 0.0)) {
        let assign85630_e131322: f64 = (locals.var_tmf3 * locals.var_vxbgmt);
        (assign85630_e131322, ((locals.var_tmf3_dn0 * locals.var_vxbgmt) + (locals.var_tmf3 * locals.var_vxbgmt_dn0)), ((locals.var_tmf3_dn2 * locals.var_vxbgmt) + (locals.var_tmf3 * locals.var_vxbgmt_dn2)), ((locals.var_tmf3_dn4 * locals.var_vxbgmt) + (locals.var_tmf3 * locals.var_vxbgmt_dn4)), ((locals.var_tmf3_dn5 * locals.var_vxbgmt) + (locals.var_tmf3 * locals.var_vxbgmt_dn5)), ((locals.var_tmf3_dn6 * locals.var_vxbgmt) + (locals.var_tmf3 * locals.var_vxbgmt_dn6)), ((locals.var_tmf3_dn7 * locals.var_vxbgmt) + (locals.var_tmf3 * locals.var_vxbgmt_dn7)), ((locals.var_tmf3_dn8 * locals.var_vxbgmt) + (locals.var_tmf3 * locals.var_vxbgmt_dn8)), ((locals.var_tmf3_dn9 * locals.var_vxbgmt) + (locals.var_tmf3 * locals.var_vxbgmt_dn9)), ((locals.var_tmf3_dn10 * locals.var_vxbgmt) + (locals.var_tmf3 * locals.var_vxbgmt_dn10)), ((locals.var_tmf3_dn13 * locals.var_vxbgmt) + (locals.var_tmf3 * locals.var_vxbgmt_dn13)),)
    } else {
        (locals.var_tmf4, locals.var_tmf4_dn0, locals.var_tmf4_dn2, locals.var_tmf4_dn4, locals.var_tmf4_dn5, locals.var_tmf4_dn6, locals.var_tmf4_dn7, locals.var_tmf4_dn8, locals.var_tmf4_dn9, locals.var_tmf4_dn10, locals.var_tmf4_dn13,)
    }
};
        locals.var_tmf4 = assign85630_e131324;
        locals.var_tmf4_dn0 = assign85630_e131324_d_n0;
        locals.var_tmf4_dn2 = assign85630_e131324_d_n2;
        locals.var_tmf4_dn4 = assign85630_e131324_d_n4;
        locals.var_tmf4_dn5 = assign85630_e131324_d_n5;
        locals.var_tmf4_dn6 = assign85630_e131324_d_n6;
        locals.var_tmf4_dn7 = assign85630_e131324_d_n7;
        locals.var_tmf4_dn8 = assign85630_e131324_d_n8;
        locals.var_tmf4_dn9 = assign85630_e131324_d_n9;
        locals.var_tmf4_dn10 = assign85630_e131324_d_n10;
        locals.var_tmf4_dn13 = assign85630_e131324_d_n13;

        let (assign85640_e131348, assign85640_e131348_d_n0, assign85640_e131348_d_n2, assign85640_e131348_d_n4, assign85640_e131348_d_n5, assign85640_e131348_d_n6, assign85640_e131348_d_n7, assign85640_e131348_d_n8, assign85640_e131348_d_n9, assign85640_e131348_d_n10, assign85640_e131348_d_n13,) = {
    if ((((((locals.var_guard1992 != 0.0) && (!((locals.var_guard1990 != 0.0) || (locals.var_guard1991 != 0.0)))) && (locals.var_guard1996 != 0.0)) && (locals.var_guard1997 != 0.0)) && (locals.var_guard1998 != 0.0)) && (locals.var_guard1999 == 0.0)) {
        let assign85640_e131343: f64 = (locals.var_tmf4 / locals.var_vxb_lim);
        let assign85640_e131345: f64 = (assign85640_e131343).powf(p.p113);
        let assign85640_e131346: f64 = (1.0 + assign85640_e131345);
        (assign85640_e131346, if 0.0 == 0.0 && ((p.p113) as f64).is_finite() && ((p.p113) as f64).fract() == 0.0 { if p.p113 == 0.0 { 0.0 } else { (p.p113 * ((assign85640_e131343).powf(p.p113 - 1.0) * (((locals.var_tmf4_dn0 * locals.var_vxb_lim) - (locals.var_tmf4 * locals.var_vxb_lim_dn0)) / (locals.var_vxb_lim * locals.var_vxb_lim)))) } } else { (assign85640_e131345 * (p.p113 * ((((locals.var_tmf4_dn0 * locals.var_vxb_lim) - (locals.var_tmf4 * locals.var_vxb_lim_dn0)) / (locals.var_vxb_lim * locals.var_vxb_lim)) / assign85640_e131343))) }, if 0.0 == 0.0 && ((p.p113) as f64).is_finite() && ((p.p113) as f64).fract() == 0.0 { if p.p113 == 0.0 { 0.0 } else { (p.p113 * ((assign85640_e131343).powf(p.p113 - 1.0) * (((locals.var_tmf4_dn2 * locals.var_vxb_lim) - (locals.var_tmf4 * locals.var_vxb_lim_dn2)) / (locals.var_vxb_lim * locals.var_vxb_lim)))) } } else { (assign85640_e131345 * (p.p113 * ((((locals.var_tmf4_dn2 * locals.var_vxb_lim) - (locals.var_tmf4 * locals.var_vxb_lim_dn2)) / (locals.var_vxb_lim * locals.var_vxb_lim)) / assign85640_e131343))) }, if 0.0 == 0.0 && ((p.p113) as f64).is_finite() && ((p.p113) as f64).fract() == 0.0 { if p.p113 == 0.0 { 0.0 } else { (p.p113 * ((assign85640_e131343).powf(p.p113 - 1.0) * (((locals.var_tmf4_dn4 * locals.var_vxb_lim) - (locals.var_tmf4 * locals.var_vxb_lim_dn4)) / (locals.var_vxb_lim * locals.var_vxb_lim)))) } } else { (assign85640_e131345 * (p.p113 * ((((locals.var_tmf4_dn4 * locals.var_vxb_lim) - (locals.var_tmf4 * locals.var_vxb_lim_dn4)) / (locals.var_vxb_lim * locals.var_vxb_lim)) / assign85640_e131343))) }, if 0.0 == 0.0 && ((p.p113) as f64).is_finite() && ((p.p113) as f64).fract() == 0.0 { if p.p113 == 0.0 { 0.0 } else { (p.p113 * ((assign85640_e131343).powf(p.p113 - 1.0) * (((locals.var_tmf4_dn5 * locals.var_vxb_lim) - (locals.var_tmf4 * locals.var_vxb_lim_dn5)) / (locals.var_vxb_lim * locals.var_vxb_lim)))) } } else { (assign85640_e131345 * (p.p113 * ((((locals.var_tmf4_dn5 * locals.var_vxb_lim) - (locals.var_tmf4 * locals.var_vxb_lim_dn5)) / (locals.var_vxb_lim * locals.var_vxb_lim)) / assign85640_e131343))) }, if 0.0 == 0.0 && ((p.p113) as f64).is_finite() && ((p.p113) as f64).fract() == 0.0 { if p.p113 == 0.0 { 0.0 } else { (p.p113 * ((assign85640_e131343).powf(p.p113 - 1.0) * (((locals.var_tmf4_dn6 * locals.var_vxb_lim) - (locals.var_tmf4 * locals.var_vxb_lim_dn6)) / (locals.var_vxb_lim * locals.var_vxb_lim)))) } } else { (assign85640_e131345 * (p.p113 * ((((locals.var_tmf4_dn6 * locals.var_vxb_lim) - (locals.var_tmf4 * locals.var_vxb_lim_dn6)) / (locals.var_vxb_lim * locals.var_vxb_lim)) / assign85640_e131343))) }, if 0.0 == 0.0 && ((p.p113) as f64).is_finite() && ((p.p113) as f64).fract() == 0.0 { if p.p113 == 0.0 { 0.0 } else { (p.p113 * ((assign85640_e131343).powf(p.p113 - 1.0) * (((locals.var_tmf4_dn7 * locals.var_vxb_lim) - (locals.var_tmf4 * locals.var_vxb_lim_dn7)) / (locals.var_vxb_lim * locals.var_vxb_lim)))) } } else { (assign85640_e131345 * (p.p113 * ((((locals.var_tmf4_dn7 * locals.var_vxb_lim) - (locals.var_tmf4 * locals.var_vxb_lim_dn7)) / (locals.var_vxb_lim * locals.var_vxb_lim)) / assign85640_e131343))) }, if 0.0 == 0.0 && ((p.p113) as f64).is_finite() && ((p.p113) as f64).fract() == 0.0 { if p.p113 == 0.0 { 0.0 } else { (p.p113 * ((assign85640_e131343).powf(p.p113 - 1.0) * (((locals.var_tmf4_dn8 * locals.var_vxb_lim) - (locals.var_tmf4 * locals.var_vxb_lim_dn8)) / (locals.var_vxb_lim * locals.var_vxb_lim)))) } } else { (assign85640_e131345 * (p.p113 * ((((locals.var_tmf4_dn8 * locals.var_vxb_lim) - (locals.var_tmf4 * locals.var_vxb_lim_dn8)) / (locals.var_vxb_lim * locals.var_vxb_lim)) / assign85640_e131343))) }, if 0.0 == 0.0 && ((p.p113) as f64).is_finite() && ((p.p113) as f64).fract() == 0.0 { if p.p113 == 0.0 { 0.0 } else { (p.p113 * ((assign85640_e131343).powf(p.p113 - 1.0) * (((locals.var_tmf4_dn9 * locals.var_vxb_lim) - (locals.var_tmf4 * locals.var_vxb_lim_dn9)) / (locals.var_vxb_lim * locals.var_vxb_lim)))) } } else { (assign85640_e131345 * (p.p113 * ((((locals.var_tmf4_dn9 * locals.var_vxb_lim) - (locals.var_tmf4 * locals.var_vxb_lim_dn9)) / (locals.var_vxb_lim * locals.var_vxb_lim)) / assign85640_e131343))) }, if 0.0 == 0.0 && ((p.p113) as f64).is_finite() && ((p.p113) as f64).fract() == 0.0 { if p.p113 == 0.0 { 0.0 } else { (p.p113 * ((assign85640_e131343).powf(p.p113 - 1.0) * (((locals.var_tmf4_dn10 * locals.var_vxb_lim) - (locals.var_tmf4 * locals.var_vxb_lim_dn10)) / (locals.var_vxb_lim * locals.var_vxb_lim)))) } } else { (assign85640_e131345 * (p.p113 * ((((locals.var_tmf4_dn10 * locals.var_vxb_lim) - (locals.var_tmf4 * locals.var_vxb_lim_dn10)) / (locals.var_vxb_lim * locals.var_vxb_lim)) / assign85640_e131343))) }, if 0.0 == 0.0 && ((p.p113) as f64).is_finite() && ((p.p113) as f64).fract() == 0.0 { if p.p113 == 0.0 { 0.0 } else { (p.p113 * ((assign85640_e131343).powf(p.p113 - 1.0) * (((locals.var_tmf4_dn13 * locals.var_vxb_lim) - (locals.var_tmf4 * locals.var_vxb_lim_dn13)) / (locals.var_vxb_lim * locals.var_vxb_lim)))) } } else { (assign85640_e131345 * (p.p113 * ((((locals.var_tmf4_dn13 * locals.var_vxb_lim) - (locals.var_tmf4 * locals.var_vxb_lim_dn13)) / (locals.var_vxb_lim * locals.var_vxb_lim)) / assign85640_e131343))) },)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13,)
    }
};
        locals.var_tmf1 = assign85640_e131348;
        locals.var_tmf1_dn0 = assign85640_e131348_d_n0;
        locals.var_tmf1_dn2 = assign85640_e131348_d_n2;
        locals.var_tmf1_dn4 = assign85640_e131348_d_n4;
        locals.var_tmf1_dn5 = assign85640_e131348_d_n5;
        locals.var_tmf1_dn6 = assign85640_e131348_d_n6;
        locals.var_tmf1_dn7 = assign85640_e131348_d_n7;
        locals.var_tmf1_dn8 = assign85640_e131348_d_n8;
        locals.var_tmf1_dn9 = assign85640_e131348_d_n9;
        locals.var_tmf1_dn10 = assign85640_e131348_d_n10;
        locals.var_tmf1_dn13 = assign85640_e131348_d_n13;

        let (assign85650_e131370, assign85650_e131370_d_n0, assign85650_e131370_d_n2, assign85650_e131370_d_n4, assign85650_e131370_d_n5, assign85650_e131370_d_n6, assign85650_e131370_d_n7, assign85650_e131370_d_n8, assign85650_e131370_d_n9, assign85650_e131370_d_n10, assign85650_e131370_d_n13,) = {
    if ((((((locals.var_guard1992 != 0.0) && (!((locals.var_guard1990 != 0.0) || (locals.var_guard1991 != 0.0)))) && (locals.var_guard1996 != 0.0)) && (locals.var_guard1997 != 0.0)) && (locals.var_guard1998 != 0.0)) && (locals.var_guard1999 == 0.0)) {
        let assign85650_e131367: f64 = (1.0 / p.p113);
        let assign85650_e131368: f64 = (locals.var_tmf1).powf(assign85650_e131367);
        (assign85650_e131368, if 0.0 == 0.0 && ((assign85650_e131367) as f64).is_finite() && ((assign85650_e131367) as f64).fract() == 0.0 { if assign85650_e131367 == 0.0 { 0.0 } else { (assign85650_e131367 * ((locals.var_tmf1).powf(assign85650_e131367 - 1.0) * locals.var_tmf1_dn0)) } } else { (assign85650_e131368 * (assign85650_e131367 * (locals.var_tmf1_dn0 / locals.var_tmf1))) }, if 0.0 == 0.0 && ((assign85650_e131367) as f64).is_finite() && ((assign85650_e131367) as f64).fract() == 0.0 { if assign85650_e131367 == 0.0 { 0.0 } else { (assign85650_e131367 * ((locals.var_tmf1).powf(assign85650_e131367 - 1.0) * locals.var_tmf1_dn2)) } } else { (assign85650_e131368 * (assign85650_e131367 * (locals.var_tmf1_dn2 / locals.var_tmf1))) }, if 0.0 == 0.0 && ((assign85650_e131367) as f64).is_finite() && ((assign85650_e131367) as f64).fract() == 0.0 { if assign85650_e131367 == 0.0 { 0.0 } else { (assign85650_e131367 * ((locals.var_tmf1).powf(assign85650_e131367 - 1.0) * locals.var_tmf1_dn4)) } } else { (assign85650_e131368 * (assign85650_e131367 * (locals.var_tmf1_dn4 / locals.var_tmf1))) }, if 0.0 == 0.0 && ((assign85650_e131367) as f64).is_finite() && ((assign85650_e131367) as f64).fract() == 0.0 { if assign85650_e131367 == 0.0 { 0.0 } else { (assign85650_e131367 * ((locals.var_tmf1).powf(assign85650_e131367 - 1.0) * locals.var_tmf1_dn5)) } } else { (assign85650_e131368 * (assign85650_e131367 * (locals.var_tmf1_dn5 / locals.var_tmf1))) }, if 0.0 == 0.0 && ((assign85650_e131367) as f64).is_finite() && ((assign85650_e131367) as f64).fract() == 0.0 { if assign85650_e131367 == 0.0 { 0.0 } else { (assign85650_e131367 * ((locals.var_tmf1).powf(assign85650_e131367 - 1.0) * locals.var_tmf1_dn6)) } } else { (assign85650_e131368 * (assign85650_e131367 * (locals.var_tmf1_dn6 / locals.var_tmf1))) }, if 0.0 == 0.0 && ((assign85650_e131367) as f64).is_finite() && ((assign85650_e131367) as f64).fract() == 0.0 { if assign85650_e131367 == 0.0 { 0.0 } else { (assign85650_e131367 * ((locals.var_tmf1).powf(assign85650_e131367 - 1.0) * locals.var_tmf1_dn7)) } } else { (assign85650_e131368 * (assign85650_e131367 * (locals.var_tmf1_dn7 / locals.var_tmf1))) }, if 0.0 == 0.0 && ((assign85650_e131367) as f64).is_finite() && ((assign85650_e131367) as f64).fract() == 0.0 { if assign85650_e131367 == 0.0 { 0.0 } else { (assign85650_e131367 * ((locals.var_tmf1).powf(assign85650_e131367 - 1.0) * locals.var_tmf1_dn8)) } } else { (assign85650_e131368 * (assign85650_e131367 * (locals.var_tmf1_dn8 / locals.var_tmf1))) }, if 0.0 == 0.0 && ((assign85650_e131367) as f64).is_finite() && ((assign85650_e131367) as f64).fract() == 0.0 { if assign85650_e131367 == 0.0 { 0.0 } else { (assign85650_e131367 * ((locals.var_tmf1).powf(assign85650_e131367 - 1.0) * locals.var_tmf1_dn9)) } } else { (assign85650_e131368 * (assign85650_e131367 * (locals.var_tmf1_dn9 / locals.var_tmf1))) }, if 0.0 == 0.0 && ((assign85650_e131367) as f64).is_finite() && ((assign85650_e131367) as f64).fract() == 0.0 { if assign85650_e131367 == 0.0 { 0.0 } else { (assign85650_e131367 * ((locals.var_tmf1).powf(assign85650_e131367 - 1.0) * locals.var_tmf1_dn10)) } } else { (assign85650_e131368 * (assign85650_e131367 * (locals.var_tmf1_dn10 / locals.var_tmf1))) }, if 0.0 == 0.0 && ((assign85650_e131367) as f64).is_finite() && ((assign85650_e131367) as f64).fract() == 0.0 { if assign85650_e131367 == 0.0 { 0.0 } else { (assign85650_e131367 * ((locals.var_tmf1).powf(assign85650_e131367 - 1.0) * locals.var_tmf1_dn13)) } } else { (assign85650_e131368 * (assign85650_e131367 * (locals.var_tmf1_dn13 / locals.var_tmf1))) },)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign85650_e131370;
        locals.var_tmf2_dn0 = assign85650_e131370_d_n0;
        locals.var_tmf2_dn2 = assign85650_e131370_d_n2;
        locals.var_tmf2_dn4 = assign85650_e131370_d_n4;
        locals.var_tmf2_dn5 = assign85650_e131370_d_n5;
        locals.var_tmf2_dn6 = assign85650_e131370_d_n6;
        locals.var_tmf2_dn7 = assign85650_e131370_d_n7;
        locals.var_tmf2_dn8 = assign85650_e131370_d_n8;
        locals.var_tmf2_dn9 = assign85650_e131370_d_n9;
        locals.var_tmf2_dn10 = assign85650_e131370_d_n10;
        locals.var_tmf2_dn13 = assign85650_e131370_d_n13;

        let (assign85660_e131392, assign85660_e131392_d_n0, assign85660_e131392_d_n2, assign85660_e131392_d_n4, assign85660_e131392_d_n5, assign85660_e131392_d_n6, assign85660_e131392_d_n7, assign85660_e131392_d_n8, assign85660_e131392_d_n9, assign85660_e131392_d_n10, assign85660_e131392_d_n13,) = {
    if ((((((locals.var_guard1992 != 0.0) && (!((locals.var_guard1990 != 0.0) || (locals.var_guard1991 != 0.0)))) && (locals.var_guard1996 != 0.0)) && (locals.var_guard1997 != 0.0)) && (locals.var_guard1998 != 0.0)) && (locals.var_guard1999 == 0.0)) {
        let assign85660_e131388: f64 = (locals.var_tmf3 * locals.var_tmf4);
        let assign85660_e131390: f64 = (assign85660_e131388 / locals.var_tmf2);
        (assign85660_e131390, (((((locals.var_tmf3_dn0 * locals.var_tmf4) + (locals.var_tmf3 * locals.var_tmf4_dn0)) * locals.var_tmf2) - (assign85660_e131388 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2)), (((((locals.var_tmf3_dn2 * locals.var_tmf4) + (locals.var_tmf3 * locals.var_tmf4_dn2)) * locals.var_tmf2) - (assign85660_e131388 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2)), (((((locals.var_tmf3_dn4 * locals.var_tmf4) + (locals.var_tmf3 * locals.var_tmf4_dn4)) * locals.var_tmf2) - (assign85660_e131388 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2)), (((((locals.var_tmf3_dn5 * locals.var_tmf4) + (locals.var_tmf3 * locals.var_tmf4_dn5)) * locals.var_tmf2) - (assign85660_e131388 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2)), (((((locals.var_tmf3_dn6 * locals.var_tmf4) + (locals.var_tmf3 * locals.var_tmf4_dn6)) * locals.var_tmf2) - (assign85660_e131388 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2)), (((((locals.var_tmf3_dn7 * locals.var_tmf4) + (locals.var_tmf3 * locals.var_tmf4_dn7)) * locals.var_tmf2) - (assign85660_e131388 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2)), (((((locals.var_tmf3_dn8 * locals.var_tmf4) + (locals.var_tmf3 * locals.var_tmf4_dn8)) * locals.var_tmf2) - (assign85660_e131388 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2)), (((((locals.var_tmf3_dn9 * locals.var_tmf4) + (locals.var_tmf3 * locals.var_tmf4_dn9)) * locals.var_tmf2) - (assign85660_e131388 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2)), (((((locals.var_tmf3_dn10 * locals.var_tmf4) + (locals.var_tmf3 * locals.var_tmf4_dn10)) * locals.var_tmf2) - (assign85660_e131388 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2)), (((((locals.var_tmf3_dn13 * locals.var_tmf4) + (locals.var_tmf3 * locals.var_tmf4_dn13)) * locals.var_tmf2) - (assign85660_e131388 * locals.var_tmf2_dn13)) / (locals.var_tmf2 * locals.var_tmf2)),)
    } else {
        (locals.var_vxbgmt, locals.var_vxbgmt_dn0, locals.var_vxbgmt_dn2, locals.var_vxbgmt_dn4, locals.var_vxbgmt_dn5, locals.var_vxbgmt_dn6, locals.var_vxbgmt_dn7, locals.var_vxbgmt_dn8, locals.var_vxbgmt_dn9, locals.var_vxbgmt_dn10, locals.var_vxbgmt_dn13,)
    }
};
        locals.var_vxbgmt = assign85660_e131392;
        locals.var_vxbgmt_dn0 = assign85660_e131392_d_n0;
        locals.var_vxbgmt_dn2 = assign85660_e131392_d_n2;
        locals.var_vxbgmt_dn4 = assign85660_e131392_d_n4;
        locals.var_vxbgmt_dn5 = assign85660_e131392_d_n5;
        locals.var_vxbgmt_dn6 = assign85660_e131392_d_n6;
        locals.var_vxbgmt_dn7 = assign85660_e131392_d_n7;
        locals.var_vxbgmt_dn8 = assign85660_e131392_d_n8;
        locals.var_vxbgmt_dn9 = assign85660_e131392_d_n9;
        locals.var_vxbgmt_dn10 = assign85660_e131392_d_n10;
        locals.var_vxbgmt_dn13 = assign85660_e131392_d_n13;

        let (assign85670_e131420, assign85670_e131420_d_n0, assign85670_e131420_d_n2, assign85670_e131420_d_n4, assign85670_e131420_d_n5, assign85670_e131420_d_n6, assign85670_e131420_d_n7, assign85670_e131420_d_n8, assign85670_e131420_d_n9, assign85670_e131420_d_n10, assign85670_e131420_d_n13,) = {
    if (((((locals.var_guard1992 != 0.0) && (!((locals.var_guard1990 != 0.0) || (locals.var_guard1991 != 0.0)))) && (locals.var_guard1996 != 0.0)) && (locals.var_guard1997 != 0.0)) && (locals.var_guard1998 != 0.0)) {
        let assign85670_e131407: f64 = (locals.var_vxbgmt + p.p137);
        let assign85670_e131410: f64 = (locals.var_vxbgmt + p.p137);
        let assign85670_e131411: f64 = (assign85670_e131407 * assign85670_e131410);
        let assign85670_e131414: f64 = (4.0 * 0.1);
        let assign85670_e131416: f64 = (assign85670_e131414 * 0.1);
        let assign85670_e131417: f64 = (assign85670_e131411 + assign85670_e131416);
        let assign85670_e131418: f64 = (assign85670_e131417).sqrt();
        (assign85670_e131418, (((locals.var_vxbgmt_dn0 * assign85670_e131410) + (assign85670_e131407 * locals.var_vxbgmt_dn0)) / (2.0 * assign85670_e131418)), (((locals.var_vxbgmt_dn2 * assign85670_e131410) + (assign85670_e131407 * locals.var_vxbgmt_dn2)) / (2.0 * assign85670_e131418)), (((locals.var_vxbgmt_dn4 * assign85670_e131410) + (assign85670_e131407 * locals.var_vxbgmt_dn4)) / (2.0 * assign85670_e131418)), (((locals.var_vxbgmt_dn5 * assign85670_e131410) + (assign85670_e131407 * locals.var_vxbgmt_dn5)) / (2.0 * assign85670_e131418)), (((locals.var_vxbgmt_dn6 * assign85670_e131410) + (assign85670_e131407 * locals.var_vxbgmt_dn6)) / (2.0 * assign85670_e131418)), (((locals.var_vxbgmt_dn7 * assign85670_e131410) + (assign85670_e131407 * locals.var_vxbgmt_dn7)) / (2.0 * assign85670_e131418)), (((locals.var_vxbgmt_dn8 * assign85670_e131410) + (assign85670_e131407 * locals.var_vxbgmt_dn8)) / (2.0 * assign85670_e131418)), (((locals.var_vxbgmt_dn9 * assign85670_e131410) + (assign85670_e131407 * locals.var_vxbgmt_dn9)) / (2.0 * assign85670_e131418)), (((locals.var_vxbgmt_dn10 * assign85670_e131410) + (assign85670_e131407 * locals.var_vxbgmt_dn10)) / (2.0 * assign85670_e131418)), (((locals.var_vxbgmt_dn13 * assign85670_e131410) + (assign85670_e131407 * locals.var_vxbgmt_dn13)) / (2.0 * assign85670_e131418)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign85670_e131420;
        locals.var_tmf2_dn0 = assign85670_e131420_d_n0;
        locals.var_tmf2_dn2 = assign85670_e131420_d_n2;
        locals.var_tmf2_dn4 = assign85670_e131420_d_n4;
        locals.var_tmf2_dn5 = assign85670_e131420_d_n5;
        locals.var_tmf2_dn6 = assign85670_e131420_d_n6;
        locals.var_tmf2_dn7 = assign85670_e131420_d_n7;
        locals.var_tmf2_dn8 = assign85670_e131420_d_n8;
        locals.var_tmf2_dn9 = assign85670_e131420_d_n9;
        locals.var_tmf2_dn10 = assign85670_e131420_d_n10;
        locals.var_tmf2_dn13 = assign85670_e131420_d_n13;

    }

    pub(super) fn stamp_transient_block_301(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign85680_e131443, assign85680_e131443_d_n0, assign85680_e131443_d_n2, assign85680_e131443_d_n4, assign85680_e131443_d_n5, assign85680_e131443_d_n6, assign85680_e131443_d_n7, assign85680_e131443_d_n8, assign85680_e131443_d_n9, assign85680_e131443_d_n10, assign85680_e131443_d_n13,) = {
    if (((((locals.var_guard1992 != 0.0) && (!((locals.var_guard1990 != 0.0) || (locals.var_guard1991 != 0.0)))) && (locals.var_guard1996 != 0.0)) && (locals.var_guard1997 != 0.0)) && (locals.var_guard1998 != 0.0)) {
        let assign85680_e131437: f64 = (locals.var_vxbgmt + p.p137);
        let assign85680_e131439: f64 = (assign85680_e131437 / locals.var_tmf2);
        let assign85680_e131440: f64 = (1.0 + assign85680_e131439);
        let assign85680_e131441: f64 = (0.5 * assign85680_e131440);
        (assign85680_e131441, (0.5 * (((locals.var_vxbgmt_dn0 * locals.var_tmf2) - (assign85680_e131437 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vxbgmt_dn2 * locals.var_tmf2) - (assign85680_e131437 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vxbgmt_dn4 * locals.var_tmf2) - (assign85680_e131437 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vxbgmt_dn5 * locals.var_tmf2) - (assign85680_e131437 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vxbgmt_dn6 * locals.var_tmf2) - (assign85680_e131437 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vxbgmt_dn7 * locals.var_tmf2) - (assign85680_e131437 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vxbgmt_dn8 * locals.var_tmf2) - (assign85680_e131437 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vxbgmt_dn9 * locals.var_tmf2) - (assign85680_e131437 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vxbgmt_dn10 * locals.var_tmf2) - (assign85680_e131437 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vxbgmt_dn13 * locals.var_tmf2) - (assign85680_e131437 * locals.var_tmf2_dn13)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn13,)
    }
};
        locals.var_t9 = assign85680_e131443;
        locals.var_t9_dn0 = assign85680_e131443_d_n0;
        locals.var_t9_dn2 = assign85680_e131443_d_n2;
        locals.var_t9_dn4 = assign85680_e131443_d_n4;
        locals.var_t9_dn5 = assign85680_e131443_d_n5;
        locals.var_t9_dn6 = assign85680_e131443_d_n6;
        locals.var_t9_dn7 = assign85680_e131443_d_n7;
        locals.var_t9_dn8 = assign85680_e131443_d_n8;
        locals.var_t9_dn9 = assign85680_e131443_d_n9;
        locals.var_t9_dn10 = assign85680_e131443_d_n10;
        locals.var_t9_dn13 = assign85680_e131443_d_n13;

        let (assign85690_e131464, assign85690_e131464_d_n0, assign85690_e131464_d_n2, assign85690_e131464_d_n4, assign85690_e131464_d_n5, assign85690_e131464_d_n6, assign85690_e131464_d_n7, assign85690_e131464_d_n8, assign85690_e131464_d_n9, assign85690_e131464_d_n10, assign85690_e131464_d_n13,) = {
    if (((((locals.var_guard1992 != 0.0) && (!((locals.var_guard1990 != 0.0) || (locals.var_guard1991 != 0.0)))) && (locals.var_guard1996 != 0.0)) && (locals.var_guard1997 != 0.0)) && (locals.var_guard1998 != 0.0)) {
        let assign85690_e131459: f64 = (locals.var_vxbgmt + p.p137);
        let assign85690_e131461: f64 = (assign85690_e131459 + locals.var_tmf2);
        let assign85690_e131462: f64 = (0.5 * assign85690_e131461);
        (assign85690_e131462, (0.5 * (locals.var_vxbgmt_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_vxbgmt_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_vxbgmt_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_vxbgmt_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_vxbgmt_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_vxbgmt_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_vxbgmt_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_vxbgmt_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_vxbgmt_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_vxbgmt_dn13 + locals.var_tmf2_dn13)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign85690_e131464;
        locals.var_t2_dn0 = assign85690_e131464_d_n0;
        locals.var_t2_dn2 = assign85690_e131464_d_n2;
        locals.var_t2_dn4 = assign85690_e131464_d_n4;
        locals.var_t2_dn5 = assign85690_e131464_d_n5;
        locals.var_t2_dn6 = assign85690_e131464_d_n6;
        locals.var_t2_dn7 = assign85690_e131464_d_n7;
        locals.var_t2_dn8 = assign85690_e131464_d_n8;
        locals.var_t2_dn9 = assign85690_e131464_d_n9;
        locals.var_t2_dn10 = assign85690_e131464_d_n10;
        locals.var_t2_dn13 = assign85690_e131464_d_n13;

        let assign85700_e131467: f64 = if locals.var_t2 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2000 = assign85700_e131467;

        let (assign85710_e131484, assign85710_e131484_d_n0, assign85710_e131484_d_n2, assign85710_e131484_d_n4, assign85710_e131484_d_n5, assign85710_e131484_d_n6, assign85710_e131484_d_n7, assign85710_e131484_d_n8, assign85710_e131484_d_n9, assign85710_e131484_d_n10, assign85710_e131484_d_n13,) = {
    if ((((((locals.var_guard1992 != 0.0) && (!((locals.var_guard1990 != 0.0) || (locals.var_guard1991 != 0.0)))) && (locals.var_guard1996 != 0.0)) && (locals.var_guard1997 != 0.0)) && (locals.var_guard1998 != 0.0)) && (locals.var_guard2000 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign85710_e131484;
        locals.var_t2_dn0 = assign85710_e131484_d_n0;
        locals.var_t2_dn2 = assign85710_e131484_d_n2;
        locals.var_t2_dn4 = assign85710_e131484_d_n4;
        locals.var_t2_dn5 = assign85710_e131484_d_n5;
        locals.var_t2_dn6 = assign85710_e131484_d_n6;
        locals.var_t2_dn7 = assign85710_e131484_d_n7;
        locals.var_t2_dn8 = assign85710_e131484_d_n8;
        locals.var_t2_dn9 = assign85710_e131484_d_n9;
        locals.var_t2_dn10 = assign85710_e131484_d_n10;
        locals.var_t2_dn13 = assign85710_e131484_d_n13;

        let (assign85720_e131501, assign85720_e131501_d_n0, assign85720_e131501_d_n2, assign85720_e131501_d_n4, assign85720_e131501_d_n5, assign85720_e131501_d_n6, assign85720_e131501_d_n7, assign85720_e131501_d_n8, assign85720_e131501_d_n9, assign85720_e131501_d_n10, assign85720_e131501_d_n13,) = {
    if ((((((locals.var_guard1992 != 0.0) && (!((locals.var_guard1990 != 0.0) || (locals.var_guard1991 != 0.0)))) && (locals.var_guard1996 != 0.0)) && (locals.var_guard1997 != 0.0)) && (locals.var_guard1998 != 0.0)) && (locals.var_guard2000 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn13,)
    }
};
        locals.var_t9 = assign85720_e131501;
        locals.var_t9_dn0 = assign85720_e131501_d_n0;
        locals.var_t9_dn2 = assign85720_e131501_d_n2;
        locals.var_t9_dn4 = assign85720_e131501_d_n4;
        locals.var_t9_dn5 = assign85720_e131501_d_n5;
        locals.var_t9_dn6 = assign85720_e131501_d_n6;
        locals.var_t9_dn7 = assign85720_e131501_d_n7;
        locals.var_t9_dn8 = assign85720_e131501_d_n8;
        locals.var_t9_dn9 = assign85720_e131501_d_n9;
        locals.var_t9_dn10 = assign85720_e131501_d_n10;
        locals.var_t9_dn13 = assign85720_e131501_d_n13;

        let (assign85730_e131521, assign85730_e131521_d_n0, assign85730_e131521_d_n2, assign85730_e131521_d_n4, assign85730_e131521_d_n5, assign85730_e131521_d_n6, assign85730_e131521_d_n7, assign85730_e131521_d_n8, assign85730_e131521_d_n9, assign85730_e131521_d_n10, assign85730_e131521_d_n13,) = {
    if (((((locals.var_guard1992 != 0.0) && (!((locals.var_guard1990 != 0.0) || (locals.var_guard1991 != 0.0)))) && (locals.var_guard1996 != 0.0)) && (locals.var_guard1997 != 0.0)) && (locals.var_guard1998 != 0.0)) {
        let assign85730_e131516: f64 = (locals.var_kjunc * locals.var_t2);
        let assign85730_e131517: f64 = (assign85730_e131516).sqrt();
        let assign85730_e131519: f64 = (assign85730_e131517 * p.p432);
        (assign85730_e131519, ((((locals.var_kjunc_dn0 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn0)) / (2.0 * assign85730_e131517)) * p.p432), ((((locals.var_kjunc_dn2 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn2)) / (2.0 * assign85730_e131517)) * p.p432), ((((locals.var_kjunc_dn4 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn4)) / (2.0 * assign85730_e131517)) * p.p432), ((((locals.var_kjunc_dn5 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn5)) / (2.0 * assign85730_e131517)) * p.p432), ((((locals.var_kjunc_dn6 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn6)) / (2.0 * assign85730_e131517)) * p.p432), ((((locals.var_kjunc_dn7 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn7)) / (2.0 * assign85730_e131517)) * p.p432), ((((locals.var_kjunc_dn8 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn8)) / (2.0 * assign85730_e131517)) * p.p432), ((((locals.var_kjunc_dn9 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn9)) / (2.0 * assign85730_e131517)) * p.p432), ((((locals.var_kjunc_dn10 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn10)) / (2.0 * assign85730_e131517)) * p.p432), ((((locals.var_kjunc_dn13 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn13)) / (2.0 * assign85730_e131517)) * p.p432),)
    } else {
        (locals.var_wjunc0, locals.var_wjunc0_dn0, locals.var_wjunc0_dn2, locals.var_wjunc0_dn4, locals.var_wjunc0_dn5, locals.var_wjunc0_dn6, locals.var_wjunc0_dn7, locals.var_wjunc0_dn8, locals.var_wjunc0_dn9, locals.var_wjunc0_dn10, locals.var_wjunc0_dn13,)
    }
};
        locals.var_wjunc0 = assign85730_e131521;
        locals.var_wjunc0_dn0 = assign85730_e131521_d_n0;
        locals.var_wjunc0_dn2 = assign85730_e131521_d_n2;
        locals.var_wjunc0_dn4 = assign85730_e131521_d_n4;
        locals.var_wjunc0_dn5 = assign85730_e131521_d_n5;
        locals.var_wjunc0_dn6 = assign85730_e131521_d_n6;
        locals.var_wjunc0_dn7 = assign85730_e131521_d_n7;
        locals.var_wjunc0_dn8 = assign85730_e131521_d_n8;
        locals.var_wjunc0_dn9 = assign85730_e131521_d_n9;
        locals.var_wjunc0_dn10 = assign85730_e131521_d_n10;
        locals.var_wjunc0_dn13 = assign85730_e131521_d_n13;

        let (assign85740_e131538, assign85740_e131538_d_n0, assign85740_e131538_d_n2, assign85740_e131538_d_n4, assign85740_e131538_d_n5, assign85740_e131538_d_n6, assign85740_e131538_d_n7, assign85740_e131538_d_n8, assign85740_e131538_d_n9, assign85740_e131538_d_n10, assign85740_e131538_d_n13,) = {
    if (((((locals.var_guard1992 != 0.0) && (!((locals.var_guard1990 != 0.0) || (locals.var_guard1991 != 0.0)))) && (locals.var_guard1996 != 0.0)) && (locals.var_guard1997 != 0.0)) && (locals.var_guard1998 != 0.0)) {
        let assign85740_e131536: f64 = (locals.var_lover_func - locals.var_wjunc0);
        (assign85740_e131536, (locals.var_lover_func_dn0 - locals.var_wjunc0_dn0), (locals.var_lover_func_dn2 - locals.var_wjunc0_dn2), (locals.var_lover_func_dn4 - locals.var_wjunc0_dn4), (locals.var_lover_func_dn5 - locals.var_wjunc0_dn5), (locals.var_lover_func_dn6 - locals.var_wjunc0_dn6), (locals.var_lover_func_dn7 - locals.var_wjunc0_dn7), (locals.var_lover_func_dn8 - locals.var_wjunc0_dn8), (locals.var_lover_func_dn9 - locals.var_wjunc0_dn9), (locals.var_lover_func_dn10 - locals.var_wjunc0_dn10), (locals.var_lover_func_dn13 - locals.var_wjunc0_dn13),)
    } else {
        (locals.var_lover_func, locals.var_lover_func_dn0, locals.var_lover_func_dn2, locals.var_lover_func_dn4, locals.var_lover_func_dn5, locals.var_lover_func_dn6, locals.var_lover_func_dn7, locals.var_lover_func_dn8, locals.var_lover_func_dn9, locals.var_lover_func_dn10, locals.var_lover_func_dn13,)
    }
};
        locals.var_lover_func = assign85740_e131538;
        locals.var_lover_func_dn0 = assign85740_e131538_d_n0;
        locals.var_lover_func_dn2 = assign85740_e131538_d_n2;
        locals.var_lover_func_dn4 = assign85740_e131538_d_n4;
        locals.var_lover_func_dn5 = assign85740_e131538_d_n5;
        locals.var_lover_func_dn6 = assign85740_e131538_d_n6;
        locals.var_lover_func_dn7 = assign85740_e131538_d_n7;
        locals.var_lover_func_dn8 = assign85740_e131538_d_n8;
        locals.var_lover_func_dn9 = assign85740_e131538_d_n9;
        locals.var_lover_func_dn10 = assign85740_e131538_d_n10;
        locals.var_lover_func_dn13 = assign85740_e131538_d_n13;

        let assign85750_e131557: f64 = if (((((p.p35 == 1.0) && (p.p63 > 0.0)) && (locals.var_uc_nover > 0.0)) && (locals.var_uc_cvdsover != 0.0)) && (p.p55 != 1.0)) { 1.0 } else { 0.0 };
        locals.var_guard2001 = assign85750_e131557;

        let (assign85760_e131570,) = {
    if (((locals.var_guard1993 != 0.0) && (!(((locals.var_guard1990 != 0.0) || (locals.var_guard1991 != 0.0)) || (locals.var_guard1992 != 0.0)))) && (locals.var_guard2001 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_flg_calcqover,)
    }
};
        locals.var_flg_calcqover = assign85760_e131570;

        let (assign85770_e131585, assign85770_e131585_d_n2, assign85770_e131585_d_n6, assign85770_e131585_d_n7, assign85770_e131585_d_n8,) = {
    if (((locals.var_guard1993 != 0.0) && (!(((locals.var_guard1990 != 0.0) || (locals.var_guard1991 != 0.0)) || (locals.var_guard1992 != 0.0)))) && (locals.var_guard2001 != 0.0)) {
        let assign85770_e131583: f64 = (locals.var_vgsei - locals.var_vbsei);
        (assign85770_e131583, (locals.var_vgsei_dn2 - locals.var_vbsei_dn2), locals.var_vgsei_dn6, 0.0, (-locals.var_vbsei_dn8),)
    } else {
        (locals.var_vgbgmt, locals.var_vgbgmt_dn2, locals.var_vgbgmt_dn6, locals.var_vgbgmt_dn7, locals.var_vgbgmt_dn8,)
    }
};
        locals.var_vgbgmt = assign85770_e131585;
        locals.var_vgbgmt_dn2 = assign85770_e131585_d_n2;
        locals.var_vgbgmt_dn6 = assign85770_e131585_d_n6;
        locals.var_vgbgmt_dn7 = assign85770_e131585_d_n7;
        locals.var_vgbgmt_dn8 = assign85770_e131585_d_n8;

        let (assign85780_e131600, assign85780_e131600_d_n0, assign85780_e131600_d_n2, assign85780_e131600_d_n4, assign85780_e131600_d_n5, assign85780_e131600_d_n6, assign85780_e131600_d_n7, assign85780_e131600_d_n8, assign85780_e131600_d_n9, assign85780_e131600_d_n10, assign85780_e131600_d_n13,) = {
    if (((locals.var_guard1993 != 0.0) && (!(((locals.var_guard1990 != 0.0) || (locals.var_guard1991 != 0.0)) || (locals.var_guard1992 != 0.0)))) && (locals.var_guard2001 != 0.0)) {
        let assign85780_e131598: f64 = (locals.var_vdsei - locals.var_vbsei);
        (assign85780_e131598, locals.var_vdsei_dn0, (locals.var_vdsei_dn2 - locals.var_vbsei_dn2), 0.0, 0.0, 0.0, 0.0, (-locals.var_vbsei_dn8), 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vxbgmt, locals.var_vxbgmt_dn0, locals.var_vxbgmt_dn2, locals.var_vxbgmt_dn4, locals.var_vxbgmt_dn5, locals.var_vxbgmt_dn6, locals.var_vxbgmt_dn7, locals.var_vxbgmt_dn8, locals.var_vxbgmt_dn9, locals.var_vxbgmt_dn10, locals.var_vxbgmt_dn13,)
    }
};
        locals.var_vxbgmt = assign85780_e131600;
        locals.var_vxbgmt_dn0 = assign85780_e131600_d_n0;
        locals.var_vxbgmt_dn2 = assign85780_e131600_d_n2;
        locals.var_vxbgmt_dn4 = assign85780_e131600_d_n4;
        locals.var_vxbgmt_dn5 = assign85780_e131600_d_n5;
        locals.var_vxbgmt_dn6 = assign85780_e131600_d_n6;
        locals.var_vxbgmt_dn7 = assign85780_e131600_d_n7;
        locals.var_vxbgmt_dn8 = assign85780_e131600_d_n8;
        locals.var_vxbgmt_dn9 = assign85780_e131600_d_n9;
        locals.var_vxbgmt_dn10 = assign85780_e131600_d_n10;
        locals.var_vxbgmt_dn13 = assign85780_e131600_d_n13;

        let (assign85790_e131604, assign85790_e131604_d_n0, assign85790_e131604_d_n2, assign85790_e131604_d_n4, assign85790_e131604_d_n5, assign85790_e131604_d_n6, assign85790_e131604_d_n7, assign85790_e131604_d_n8, assign85790_e131604_d_n9, assign85790_e131604_d_n10, assign85790_e131604_d_n13,) = {
    if (locals.var_flg_calcqover != 0.0) {
        (0.4, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vbs_bnd_over__blk2009, locals.var_vbs_bnd_over__blk2009_dn0, locals.var_vbs_bnd_over__blk2009_dn2, locals.var_vbs_bnd_over__blk2009_dn4, locals.var_vbs_bnd_over__blk2009_dn5, locals.var_vbs_bnd_over__blk2009_dn6, locals.var_vbs_bnd_over__blk2009_dn7, locals.var_vbs_bnd_over__blk2009_dn8, locals.var_vbs_bnd_over__blk2009_dn9, locals.var_vbs_bnd_over__blk2009_dn10, locals.var_vbs_bnd_over__blk2009_dn13,)
    }
};
        locals.var_vbs_bnd_over__blk2009 = assign85790_e131604;
        locals.var_vbs_bnd_over__blk2009_dn0 = assign85790_e131604_d_n0;
        locals.var_vbs_bnd_over__blk2009_dn2 = assign85790_e131604_d_n2;
        locals.var_vbs_bnd_over__blk2009_dn4 = assign85790_e131604_d_n4;
        locals.var_vbs_bnd_over__blk2009_dn5 = assign85790_e131604_d_n5;
        locals.var_vbs_bnd_over__blk2009_dn6 = assign85790_e131604_d_n6;
        locals.var_vbs_bnd_over__blk2009_dn7 = assign85790_e131604_d_n7;
        locals.var_vbs_bnd_over__blk2009_dn8 = assign85790_e131604_d_n8;
        locals.var_vbs_bnd_over__blk2009_dn9 = assign85790_e131604_d_n9;
        locals.var_vbs_bnd_over__blk2009_dn10 = assign85790_e131604_d_n10;
        locals.var_vbs_bnd_over__blk2009_dn13 = assign85790_e131604_d_n13;

        let (assign85810_e131612,) = {
    if (locals.var_flg_calcqover != 0.0) {
        (0.0,)
    } else {
        (locals.var_flg_fd_mode__blk2010,)
    }
};
        locals.var_flg_fd_mode__blk2010 = assign85810_e131612;

        let (assign85820_e131616, assign85820_e131616_d_n0, assign85820_e131616_d_n2, assign85820_e131616_d_n4, assign85820_e131616_d_n5, assign85820_e131616_d_n6, assign85820_e131616_d_n7, assign85820_e131616_d_n8, assign85820_e131616_d_n9, assign85820_e131616_d_n10, assign85820_e131616_d_n13,) = {
    if (locals.var_flg_calcqover != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fb, locals.var_fb_dn0, locals.var_fb_dn2, locals.var_fb_dn4, locals.var_fb_dn5, locals.var_fb_dn6, locals.var_fb_dn7, locals.var_fb_dn8, locals.var_fb_dn9, locals.var_fb_dn10, locals.var_fb_dn13,)
    }
};
        locals.var_fb = assign85820_e131616;
        locals.var_fb_dn0 = assign85820_e131616_d_n0;
        locals.var_fb_dn2 = assign85820_e131616_d_n2;
        locals.var_fb_dn4 = assign85820_e131616_d_n4;
        locals.var_fb_dn5 = assign85820_e131616_d_n5;
        locals.var_fb_dn6 = assign85820_e131616_d_n6;
        locals.var_fb_dn7 = assign85820_e131616_d_n7;
        locals.var_fb_dn8 = assign85820_e131616_d_n8;
        locals.var_fb_dn9 = assign85820_e131616_d_n9;
        locals.var_fb_dn10 = assign85820_e131616_d_n10;
        locals.var_fb_dn13 = assign85820_e131616_d_n13;

        let (assign85830_e131620, assign85830_e131620_d_n0, assign85830_e131620_d_n2, assign85830_e131620_d_n4, assign85830_e131620_d_n5, assign85830_e131620_d_n6, assign85830_e131620_d_n7, assign85830_e131620_d_n8, assign85830_e131620_d_n9, assign85830_e131620_d_n10, assign85830_e131620_d_n13,) = {
    if (locals.var_flg_calcqover != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fs01, locals.var_fs01_dn0, locals.var_fs01_dn2, locals.var_fs01_dn4, locals.var_fs01_dn5, locals.var_fs01_dn6, locals.var_fs01_dn7, locals.var_fs01_dn8, locals.var_fs01_dn9, locals.var_fs01_dn10, locals.var_fs01_dn13,)
    }
};
        locals.var_fs01 = assign85830_e131620;
        locals.var_fs01_dn0 = assign85830_e131620_d_n0;
        locals.var_fs01_dn2 = assign85830_e131620_d_n2;
        locals.var_fs01_dn4 = assign85830_e131620_d_n4;
        locals.var_fs01_dn5 = assign85830_e131620_d_n5;
        locals.var_fs01_dn6 = assign85830_e131620_d_n6;
        locals.var_fs01_dn7 = assign85830_e131620_d_n7;
        locals.var_fs01_dn8 = assign85830_e131620_d_n8;
        locals.var_fs01_dn9 = assign85830_e131620_d_n9;
        locals.var_fs01_dn10 = assign85830_e131620_d_n10;
        locals.var_fs01_dn13 = assign85830_e131620_d_n13;

        let (assign85840_e131624, assign85840_e131624_d_n0, assign85840_e131624_d_n2, assign85840_e131624_d_n4, assign85840_e131624_d_n5, assign85840_e131624_d_n6, assign85840_e131624_d_n7, assign85840_e131624_d_n8, assign85840_e131624_d_n9, assign85840_e131624_d_n10, assign85840_e131624_d_n13,) = {
    if (locals.var_flg_calcqover != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fs02, locals.var_fs02_dn0, locals.var_fs02_dn2, locals.var_fs02_dn4, locals.var_fs02_dn5, locals.var_fs02_dn6, locals.var_fs02_dn7, locals.var_fs02_dn8, locals.var_fs02_dn9, locals.var_fs02_dn10, locals.var_fs02_dn13,)
    }
};
        locals.var_fs02 = assign85840_e131624;
        locals.var_fs02_dn0 = assign85840_e131624_d_n0;
        locals.var_fs02_dn2 = assign85840_e131624_d_n2;
        locals.var_fs02_dn4 = assign85840_e131624_d_n4;
        locals.var_fs02_dn5 = assign85840_e131624_d_n5;
        locals.var_fs02_dn6 = assign85840_e131624_d_n6;
        locals.var_fs02_dn7 = assign85840_e131624_d_n7;
        locals.var_fs02_dn8 = assign85840_e131624_d_n8;
        locals.var_fs02_dn9 = assign85840_e131624_d_n9;
        locals.var_fs02_dn10 = assign85840_e131624_d_n10;
        locals.var_fs02_dn13 = assign85840_e131624_d_n13;

        let (assign85850_e131628, assign85850_e131628_d_n0, assign85850_e131628_d_n2, assign85850_e131628_d_n4, assign85850_e131628_d_n5, assign85850_e131628_d_n6, assign85850_e131628_d_n7, assign85850_e131628_d_n8, assign85850_e131628_d_n9, assign85850_e131628_d_n10, assign85850_e131628_d_n13,) = {
    if (locals.var_flg_calcqover != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fs0, locals.var_fs0_dn0, locals.var_fs0_dn2, locals.var_fs0_dn4, locals.var_fs0_dn5, locals.var_fs0_dn6, locals.var_fs0_dn7, locals.var_fs0_dn8, locals.var_fs0_dn9, locals.var_fs0_dn10, locals.var_fs0_dn13,)
    }
};
        locals.var_fs0 = assign85850_e131628;
        locals.var_fs0_dn0 = assign85850_e131628_d_n0;
        locals.var_fs0_dn2 = assign85850_e131628_d_n2;
        locals.var_fs0_dn4 = assign85850_e131628_d_n4;
        locals.var_fs0_dn5 = assign85850_e131628_d_n5;
        locals.var_fs0_dn6 = assign85850_e131628_d_n6;
        locals.var_fs0_dn7 = assign85850_e131628_d_n7;
        locals.var_fs0_dn8 = assign85850_e131628_d_n8;
        locals.var_fs0_dn9 = assign85850_e131628_d_n9;
        locals.var_fs0_dn10 = assign85850_e131628_d_n10;
        locals.var_fs0_dn13 = assign85850_e131628_d_n13;

        let (assign85860_e131632, assign85860_e131632_d_n0, assign85860_e131632_d_n2, assign85860_e131632_d_n4, assign85860_e131632_d_n5, assign85860_e131632_d_n6, assign85860_e131632_d_n7, assign85860_e131632_d_n8, assign85860_e131632_d_n9, assign85860_e131632_d_n10, assign85860_e131632_d_n13,) = {
    if (locals.var_flg_calcqover != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dps0, locals.var_dps0_dn0, locals.var_dps0_dn2, locals.var_dps0_dn4, locals.var_dps0_dn5, locals.var_dps0_dn6, locals.var_dps0_dn7, locals.var_dps0_dn8, locals.var_dps0_dn9, locals.var_dps0_dn10, locals.var_dps0_dn13,)
    }
};
        locals.var_dps0 = assign85860_e131632;
        locals.var_dps0_dn0 = assign85860_e131632_d_n0;
        locals.var_dps0_dn2 = assign85860_e131632_d_n2;
        locals.var_dps0_dn4 = assign85860_e131632_d_n4;
        locals.var_dps0_dn5 = assign85860_e131632_d_n5;
        locals.var_dps0_dn6 = assign85860_e131632_d_n6;
        locals.var_dps0_dn7 = assign85860_e131632_d_n7;
        locals.var_dps0_dn8 = assign85860_e131632_d_n8;
        locals.var_dps0_dn9 = assign85860_e131632_d_n9;
        locals.var_dps0_dn10 = assign85860_e131632_d_n10;
        locals.var_dps0_dn13 = assign85860_e131632_d_n13;

        let (assign85870_e131636, assign85870_e131636_d_n0, assign85870_e131636_d_n2, assign85870_e131636_d_n4, assign85870_e131636_d_n5, assign85870_e131636_d_n6, assign85870_e131636_d_n7, assign85870_e131636_d_n8, assign85870_e131636_d_n9, assign85870_e131636_d_n10, assign85870_e131636_d_n13,) = {
    if (locals.var_flg_calcqover != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fs0_dps0, locals.var_fs0_dps0_dn0, locals.var_fs0_dps0_dn2, locals.var_fs0_dps0_dn4, locals.var_fs0_dps0_dn5, locals.var_fs0_dps0_dn6, locals.var_fs0_dps0_dn7, locals.var_fs0_dps0_dn8, locals.var_fs0_dps0_dn9, locals.var_fs0_dps0_dn10, locals.var_fs0_dps0_dn13,)
    }
};
        locals.var_fs0_dps0 = assign85870_e131636;
        locals.var_fs0_dps0_dn0 = assign85870_e131636_d_n0;
        locals.var_fs0_dps0_dn2 = assign85870_e131636_d_n2;
        locals.var_fs0_dps0_dn4 = assign85870_e131636_d_n4;
        locals.var_fs0_dps0_dn5 = assign85870_e131636_d_n5;
        locals.var_fs0_dps0_dn6 = assign85870_e131636_d_n6;
        locals.var_fs0_dps0_dn7 = assign85870_e131636_d_n7;
        locals.var_fs0_dps0_dn8 = assign85870_e131636_d_n8;
        locals.var_fs0_dps0_dn9 = assign85870_e131636_d_n9;
        locals.var_fs0_dps0_dn10 = assign85870_e131636_d_n10;
        locals.var_fs0_dps0_dn13 = assign85870_e131636_d_n13;

        let (assign85880_e131640, assign85880_e131640_d_n0, assign85880_e131640_d_n2, assign85880_e131640_d_n4, assign85880_e131640_d_n5, assign85880_e131640_d_n6, assign85880_e131640_d_n7, assign85880_e131640_d_n8, assign85880_e131640_d_n9, assign85880_e131640_d_n10, assign85880_e131640_d_n13,) = {
    if (locals.var_flg_calcqover != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fs02_dps0, locals.var_fs02_dps0_dn0, locals.var_fs02_dps0_dn2, locals.var_fs02_dps0_dn4, locals.var_fs02_dps0_dn5, locals.var_fs02_dps0_dn6, locals.var_fs02_dps0_dn7, locals.var_fs02_dps0_dn8, locals.var_fs02_dps0_dn9, locals.var_fs02_dps0_dn10, locals.var_fs02_dps0_dn13,)
    }
};
        locals.var_fs02_dps0 = assign85880_e131640;
        locals.var_fs02_dps0_dn0 = assign85880_e131640_d_n0;
        locals.var_fs02_dps0_dn2 = assign85880_e131640_d_n2;
        locals.var_fs02_dps0_dn4 = assign85880_e131640_d_n4;
        locals.var_fs02_dps0_dn5 = assign85880_e131640_d_n5;
        locals.var_fs02_dps0_dn6 = assign85880_e131640_d_n6;
        locals.var_fs02_dps0_dn7 = assign85880_e131640_d_n7;
        locals.var_fs02_dps0_dn8 = assign85880_e131640_d_n8;
        locals.var_fs02_dps0_dn9 = assign85880_e131640_d_n9;
        locals.var_fs02_dps0_dn10 = assign85880_e131640_d_n10;
        locals.var_fs02_dps0_dn13 = assign85880_e131640_d_n13;

        let (assign85890_e131644, assign85890_e131644_d_n0, assign85890_e131644_d_n2, assign85890_e131644_d_n4, assign85890_e131644_d_n5, assign85890_e131644_d_n6, assign85890_e131644_d_n7, assign85890_e131644_d_n8, assign85890_e131644_d_n9, assign85890_e131644_d_n10, assign85890_e131644_d_n13,) = {
    if (locals.var_flg_calcqover != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fb_dpss, locals.var_fb_dpss_dn0, locals.var_fb_dpss_dn2, locals.var_fb_dpss_dn4, locals.var_fb_dpss_dn5, locals.var_fb_dpss_dn6, locals.var_fb_dpss_dn7, locals.var_fb_dpss_dn8, locals.var_fb_dpss_dn9, locals.var_fb_dpss_dn10, locals.var_fb_dpss_dn13,)
    }
};
        locals.var_fb_dpss = assign85890_e131644;
        locals.var_fb_dpss_dn0 = assign85890_e131644_d_n0;
        locals.var_fb_dpss_dn2 = assign85890_e131644_d_n2;
        locals.var_fb_dpss_dn4 = assign85890_e131644_d_n4;
        locals.var_fb_dpss_dn5 = assign85890_e131644_d_n5;
        locals.var_fb_dpss_dn6 = assign85890_e131644_d_n6;
        locals.var_fb_dpss_dn7 = assign85890_e131644_d_n7;
        locals.var_fb_dpss_dn8 = assign85890_e131644_d_n8;
        locals.var_fb_dpss_dn9 = assign85890_e131644_d_n9;
        locals.var_fb_dpss_dn10 = assign85890_e131644_d_n10;
        locals.var_fb_dpss_dn13 = assign85890_e131644_d_n13;

        let (assign85900_e131648, assign85900_e131648_d_n0, assign85900_e131648_d_n2, assign85900_e131648_d_n4, assign85900_e131648_d_n5, assign85900_e131648_d_n6, assign85900_e131648_d_n7, assign85900_e131648_d_n8, assign85900_e131648_d_n9, assign85900_e131648_d_n10, assign85900_e131648_d_n13,) = {
    if (locals.var_flg_calcqover != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fs01_dps0, locals.var_fs01_dps0_dn0, locals.var_fs01_dps0_dn2, locals.var_fs01_dps0_dn4, locals.var_fs01_dps0_dn5, locals.var_fs01_dps0_dn6, locals.var_fs01_dps0_dn7, locals.var_fs01_dps0_dn8, locals.var_fs01_dps0_dn9, locals.var_fs01_dps0_dn10, locals.var_fs01_dps0_dn13,)
    }
};
        locals.var_fs01_dps0 = assign85900_e131648;
        locals.var_fs01_dps0_dn0 = assign85900_e131648_d_n0;
        locals.var_fs01_dps0_dn2 = assign85900_e131648_d_n2;
        locals.var_fs01_dps0_dn4 = assign85900_e131648_d_n4;
        locals.var_fs01_dps0_dn5 = assign85900_e131648_d_n5;
        locals.var_fs01_dps0_dn6 = assign85900_e131648_d_n6;
        locals.var_fs01_dps0_dn7 = assign85900_e131648_d_n7;
        locals.var_fs01_dps0_dn8 = assign85900_e131648_d_n8;
        locals.var_fs01_dps0_dn9 = assign85900_e131648_d_n9;
        locals.var_fs01_dps0_dn10 = assign85900_e131648_d_n10;
        locals.var_fs01_dps0_dn13 = assign85900_e131648_d_n13;

        let (assign85910_e131652, assign85910_e131652_d_n0, assign85910_e131652_d_n2, assign85910_e131652_d_n4, assign85910_e131652_d_n5, assign85910_e131652_d_n6, assign85910_e131652_d_n7, assign85910_e131652_d_n8, assign85910_e131652_d_n9, assign85910_e131652_d_n10, assign85910_e131652_d_n13,) = {
    if (locals.var_flg_calcqover != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_chi_1, locals.var_chi_1_dn0, locals.var_chi_1_dn2, locals.var_chi_1_dn4, locals.var_chi_1_dn5, locals.var_chi_1_dn6, locals.var_chi_1_dn7, locals.var_chi_1_dn8, locals.var_chi_1_dn9, locals.var_chi_1_dn10, locals.var_chi_1_dn13,)
    }
};
        locals.var_chi_1 = assign85910_e131652;
        locals.var_chi_1_dn0 = assign85910_e131652_d_n0;
        locals.var_chi_1_dn2 = assign85910_e131652_d_n2;
        locals.var_chi_1_dn4 = assign85910_e131652_d_n4;
        locals.var_chi_1_dn5 = assign85910_e131652_d_n5;
        locals.var_chi_1_dn6 = assign85910_e131652_d_n6;
        locals.var_chi_1_dn7 = assign85910_e131652_d_n7;
        locals.var_chi_1_dn8 = assign85910_e131652_d_n8;
        locals.var_chi_1_dn9 = assign85910_e131652_d_n9;
        locals.var_chi_1_dn10 = assign85910_e131652_d_n10;
        locals.var_chi_1_dn13 = assign85910_e131652_d_n13;

        let (assign85920_e131656, assign85920_e131656_d_n0, assign85920_e131656_d_n2, assign85920_e131656_d_n4, assign85920_e131656_d_n5, assign85920_e131656_d_n6, assign85920_e131656_d_n7, assign85920_e131656_d_n8, assign85920_e131656_d_n9, assign85920_e131656_d_n10, assign85920_e131656_d_n13,) = {
    if (locals.var_flg_calcqover != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_chi_a, locals.var_chi_a_dn0, locals.var_chi_a_dn2, locals.var_chi_a_dn4, locals.var_chi_a_dn5, locals.var_chi_a_dn6, locals.var_chi_a_dn7, locals.var_chi_a_dn8, locals.var_chi_a_dn9, locals.var_chi_a_dn10, locals.var_chi_a_dn13,)
    }
};
        locals.var_chi_a = assign85920_e131656;
        locals.var_chi_a_dn0 = assign85920_e131656_d_n0;
        locals.var_chi_a_dn2 = assign85920_e131656_d_n2;
        locals.var_chi_a_dn4 = assign85920_e131656_d_n4;
        locals.var_chi_a_dn5 = assign85920_e131656_d_n5;
        locals.var_chi_a_dn6 = assign85920_e131656_d_n6;
        locals.var_chi_a_dn7 = assign85920_e131656_d_n7;
        locals.var_chi_a_dn8 = assign85920_e131656_d_n8;
        locals.var_chi_a_dn9 = assign85920_e131656_d_n9;
        locals.var_chi_a_dn10 = assign85920_e131656_d_n10;
        locals.var_chi_a_dn13 = assign85920_e131656_d_n13;

        let (assign85930_e131660, assign85930_e131660_d_n0, assign85930_e131660_d_n2, assign85930_e131660_d_n4, assign85930_e131660_d_n5, assign85930_e131660_d_n6, assign85930_e131660_d_n7, assign85930_e131660_d_n8, assign85930_e131660_d_n9, assign85930_e131660_d_n10, assign85930_e131660_d_n13,) = {
    if (locals.var_flg_calcqover != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_chi_b, locals.var_chi_b_dn0, locals.var_chi_b_dn2, locals.var_chi_b_dn4, locals.var_chi_b_dn5, locals.var_chi_b_dn6, locals.var_chi_b_dn7, locals.var_chi_b_dn8, locals.var_chi_b_dn9, locals.var_chi_b_dn10, locals.var_chi_b_dn13,)
    }
};
        locals.var_chi_b = assign85930_e131660;
        locals.var_chi_b_dn0 = assign85930_e131660_d_n0;
        locals.var_chi_b_dn2 = assign85930_e131660_d_n2;
        locals.var_chi_b_dn4 = assign85930_e131660_d_n4;
        locals.var_chi_b_dn5 = assign85930_e131660_d_n5;
        locals.var_chi_b_dn6 = assign85930_e131660_d_n6;
        locals.var_chi_b_dn7 = assign85930_e131660_d_n7;
        locals.var_chi_b_dn8 = assign85930_e131660_d_n8;
        locals.var_chi_b_dn9 = assign85930_e131660_d_n9;
        locals.var_chi_b_dn10 = assign85930_e131660_d_n10;
        locals.var_chi_b_dn13 = assign85930_e131660_d_n13;

        let (assign85940_e131665,) = {
    if (locals.var_flg_calcqover != 0.0) {
        let assign85940_e131663: f64 = (-1.0);
        (assign85940_e131663,)
    } else {
        (locals.var_flg_conv,)
    }
};
        locals.var_flg_conv = assign85940_e131665;

        let (assign85950_e131669, assign85950_e131669_d_n0, assign85950_e131669_d_n2, assign85950_e131669_d_n4, assign85950_e131669_d_n5, assign85950_e131669_d_n6, assign85950_e131669_d_n7, assign85950_e131669_d_n8, assign85950_e131669_d_n9, assign85950_e131669_d_n10, assign85950_e131669_d_n13,) = {
    if (locals.var_flg_calcqover != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ps0ld_ini__blk2011, locals.var_ps0ld_ini__blk2011_dn0, locals.var_ps0ld_ini__blk2011_dn2, locals.var_ps0ld_ini__blk2011_dn4, locals.var_ps0ld_ini__blk2011_dn5, locals.var_ps0ld_ini__blk2011_dn6, locals.var_ps0ld_ini__blk2011_dn7, locals.var_ps0ld_ini__blk2011_dn8, locals.var_ps0ld_ini__blk2011_dn9, locals.var_ps0ld_ini__blk2011_dn10, locals.var_ps0ld_ini__blk2011_dn13,)
    }
};
        locals.var_ps0ld_ini__blk2011 = assign85950_e131669;
        locals.var_ps0ld_ini__blk2011_dn0 = assign85950_e131669_d_n0;
        locals.var_ps0ld_ini__blk2011_dn2 = assign85950_e131669_d_n2;
        locals.var_ps0ld_ini__blk2011_dn4 = assign85950_e131669_d_n4;
        locals.var_ps0ld_ini__blk2011_dn5 = assign85950_e131669_d_n5;
        locals.var_ps0ld_ini__blk2011_dn6 = assign85950_e131669_d_n6;
        locals.var_ps0ld_ini__blk2011_dn7 = assign85950_e131669_d_n7;
        locals.var_ps0ld_ini__blk2011_dn8 = assign85950_e131669_d_n8;
        locals.var_ps0ld_ini__blk2011_dn9 = assign85950_e131669_d_n9;
        locals.var_ps0ld_ini__blk2011_dn10 = assign85950_e131669_d_n10;
        locals.var_ps0ld_ini__blk2011_dn13 = assign85950_e131669_d_n13;

        let (assign85960_e131673, assign85960_e131673_d_n0, assign85960_e131673_d_n2, assign85960_e131673_d_n4, assign85960_e131673_d_n5, assign85960_e131673_d_n6, assign85960_e131673_d_n7, assign85960_e131673_d_n8, assign85960_e131673_d_n9, assign85960_e131673_d_n10, assign85960_e131673_d_n13,) = {
    if (locals.var_flg_calcqover != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fbsq__blk2012, locals.var_fbsq__blk2012_dn0, locals.var_fbsq__blk2012_dn2, locals.var_fbsq__blk2012_dn4, locals.var_fbsq__blk2012_dn5, locals.var_fbsq__blk2012_dn6, locals.var_fbsq__blk2012_dn7, locals.var_fbsq__blk2012_dn8, locals.var_fbsq__blk2012_dn9, locals.var_fbsq__blk2012_dn10, locals.var_fbsq__blk2012_dn13,)
    }
};
        locals.var_fbsq__blk2012 = assign85960_e131673;
        locals.var_fbsq__blk2012_dn0 = assign85960_e131673_d_n0;
        locals.var_fbsq__blk2012_dn2 = assign85960_e131673_d_n2;
        locals.var_fbsq__blk2012_dn4 = assign85960_e131673_d_n4;
        locals.var_fbsq__blk2012_dn5 = assign85960_e131673_d_n5;
        locals.var_fbsq__blk2012_dn6 = assign85960_e131673_d_n6;
        locals.var_fbsq__blk2012_dn7 = assign85960_e131673_d_n7;
        locals.var_fbsq__blk2012_dn8 = assign85960_e131673_d_n8;
        locals.var_fbsq__blk2012_dn9 = assign85960_e131673_d_n9;
        locals.var_fbsq__blk2012_dn10 = assign85960_e131673_d_n10;
        locals.var_fbsq__blk2012_dn13 = assign85960_e131673_d_n13;

        let (assign85970_e131684, assign85970_e131684_d_n0, assign85970_e131684_d_n2, assign85970_e131684_d_n4, assign85970_e131684_d_n5, assign85970_e131684_d_n6, assign85970_e131684_d_n7, assign85970_e131684_d_n8, assign85970_e131684_d_n9, assign85970_e131684_d_n10, assign85970_e131684_d_n13,) = {
    if (locals.var_flg_calcqover != 0.0) {
        let assign85970_e131677: f64 = (2.0 * locals.var_beta_inv);
        let assign85970_e131680: f64 = (locals.var_nover_func / locals.var_nin);
        let assign85970_e131681: f64 = (assign85970_e131680).ln();
        let assign85970_e131682: f64 = (assign85970_e131677 * assign85970_e131681);
        (assign85970_e131682, (((2.0 * locals.var_beta_inv_dn0) * assign85970_e131681) + (assign85970_e131677 * ((-((locals.var_nover_func * locals.var_nin_dn0) / (locals.var_nin * locals.var_nin))) / assign85970_e131680))), (((2.0 * locals.var_beta_inv_dn2) * assign85970_e131681) + (assign85970_e131677 * ((-((locals.var_nover_func * locals.var_nin_dn2) / (locals.var_nin * locals.var_nin))) / assign85970_e131680))), (((2.0 * locals.var_beta_inv_dn4) * assign85970_e131681) + (assign85970_e131677 * ((-((locals.var_nover_func * locals.var_nin_dn4) / (locals.var_nin * locals.var_nin))) / assign85970_e131680))), (((2.0 * locals.var_beta_inv_dn5) * assign85970_e131681) + (assign85970_e131677 * ((-((locals.var_nover_func * locals.var_nin_dn5) / (locals.var_nin * locals.var_nin))) / assign85970_e131680))), (((2.0 * locals.var_beta_inv_dn6) * assign85970_e131681) + (assign85970_e131677 * ((-((locals.var_nover_func * locals.var_nin_dn6) / (locals.var_nin * locals.var_nin))) / assign85970_e131680))), (((2.0 * locals.var_beta_inv_dn7) * assign85970_e131681) + (assign85970_e131677 * ((-((locals.var_nover_func * locals.var_nin_dn7) / (locals.var_nin * locals.var_nin))) / assign85970_e131680))), (((2.0 * locals.var_beta_inv_dn8) * assign85970_e131681) + (assign85970_e131677 * ((-((locals.var_nover_func * locals.var_nin_dn8) / (locals.var_nin * locals.var_nin))) / assign85970_e131680))), (((2.0 * locals.var_beta_inv_dn9) * assign85970_e131681) + (assign85970_e131677 * ((-((locals.var_nover_func * locals.var_nin_dn9) / (locals.var_nin * locals.var_nin))) / assign85970_e131680))), (((2.0 * locals.var_beta_inv_dn10) * assign85970_e131681) + (assign85970_e131677 * ((-((locals.var_nover_func * locals.var_nin_dn10) / (locals.var_nin * locals.var_nin))) / assign85970_e131680))), (((2.0 * locals.var_beta_inv_dn13) * assign85970_e131681) + (assign85970_e131677 * ((-((locals.var_nover_func * locals.var_nin_dn13) / (locals.var_nin * locals.var_nin))) / assign85970_e131680))),)
    } else {
        (locals.var_pb2over__blk2007, locals.var_pb2over__blk2007_dn0, locals.var_pb2over__blk2007_dn2, locals.var_pb2over__blk2007_dn4, locals.var_pb2over__blk2007_dn5, locals.var_pb2over__blk2007_dn6, locals.var_pb2over__blk2007_dn7, locals.var_pb2over__blk2007_dn8, locals.var_pb2over__blk2007_dn9, locals.var_pb2over__blk2007_dn10, locals.var_pb2over__blk2007_dn13,)
    }
};
        locals.var_pb2over__blk2007 = assign85970_e131684;
        locals.var_pb2over__blk2007_dn0 = assign85970_e131684_d_n0;
        locals.var_pb2over__blk2007_dn2 = assign85970_e131684_d_n2;
        locals.var_pb2over__blk2007_dn4 = assign85970_e131684_d_n4;
        locals.var_pb2over__blk2007_dn5 = assign85970_e131684_d_n5;
        locals.var_pb2over__blk2007_dn6 = assign85970_e131684_d_n6;
        locals.var_pb2over__blk2007_dn7 = assign85970_e131684_d_n7;
        locals.var_pb2over__blk2007_dn8 = assign85970_e131684_d_n8;
        locals.var_pb2over__blk2007_dn9 = assign85970_e131684_d_n9;
        locals.var_pb2over__blk2007_dn10 = assign85970_e131684_d_n10;
        locals.var_pb2over__blk2007_dn13 = assign85970_e131684_d_n13;

    }

    pub(super) fn stamp_transient_block_302(
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
        locals: &mut StampLocals,
    ) {
        let (assign85980_e131692, assign85980_e131692_d_n0, assign85980_e131692_d_n2, assign85980_e131692_d_n4, assign85980_e131692_d_n5, assign85980_e131692_d_n6, assign85980_e131692_d_n7, assign85980_e131692_d_n8, assign85980_e131692_d_n9, assign85980_e131692_d_n10, assign85980_e131692_d_n13,) = {
    if (locals.var_flg_calcqover != 0.0) {
        let assign85980_e131688: f64 = (0.8 - locals.var_pb2over__blk2007);
        let assign85980_e131690: f64 = (assign85980_e131688 - 0.1);
        (assign85980_e131690, (-locals.var_pb2over__blk2007_dn0), (-locals.var_pb2over__blk2007_dn2), (-locals.var_pb2over__blk2007_dn4), (-locals.var_pb2over__blk2007_dn5), (-locals.var_pb2over__blk2007_dn6), (-locals.var_pb2over__blk2007_dn7), (-locals.var_pb2over__blk2007_dn8), (-locals.var_pb2over__blk2007_dn9), (-locals.var_pb2over__blk2007_dn10), (-locals.var_pb2over__blk2007_dn13),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13,)
    }
};
        locals.var_tmf1 = assign85980_e131692;
        locals.var_tmf1_dn0 = assign85980_e131692_d_n0;
        locals.var_tmf1_dn2 = assign85980_e131692_d_n2;
        locals.var_tmf1_dn4 = assign85980_e131692_d_n4;
        locals.var_tmf1_dn5 = assign85980_e131692_d_n5;
        locals.var_tmf1_dn6 = assign85980_e131692_d_n6;
        locals.var_tmf1_dn7 = assign85980_e131692_d_n7;
        locals.var_tmf1_dn8 = assign85980_e131692_d_n8;
        locals.var_tmf1_dn9 = assign85980_e131692_d_n9;
        locals.var_tmf1_dn10 = assign85980_e131692_d_n10;
        locals.var_tmf1_dn13 = assign85980_e131692_d_n13;

        let (assign85990_e131700, assign85990_e131700_d_n0, assign85990_e131700_d_n2, assign85990_e131700_d_n4, assign85990_e131700_d_n5, assign85990_e131700_d_n6, assign85990_e131700_d_n7, assign85990_e131700_d_n8, assign85990_e131700_d_n9, assign85990_e131700_d_n10, assign85990_e131700_d_n13,) = {
    if (locals.var_flg_calcqover != 0.0) {
        let assign85990_e131696: f64 = (4.0 * 0.8);
        let assign85990_e131698: f64 = (assign85990_e131696 * 0.1);
        (assign85990_e131698, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign85990_e131700;
        locals.var_tmf2_dn0 = assign85990_e131700_d_n0;
        locals.var_tmf2_dn2 = assign85990_e131700_d_n2;
        locals.var_tmf2_dn4 = assign85990_e131700_d_n4;
        locals.var_tmf2_dn5 = assign85990_e131700_d_n5;
        locals.var_tmf2_dn6 = assign85990_e131700_d_n6;
        locals.var_tmf2_dn7 = assign85990_e131700_d_n7;
        locals.var_tmf2_dn8 = assign85990_e131700_d_n8;
        locals.var_tmf2_dn9 = assign85990_e131700_d_n9;
        locals.var_tmf2_dn10 = assign85990_e131700_d_n10;
        locals.var_tmf2_dn13 = assign85990_e131700_d_n13;

        let (assign86000_e131710, assign86000_e131710_d_n0, assign86000_e131710_d_n2, assign86000_e131710_d_n4, assign86000_e131710_d_n5, assign86000_e131710_d_n6, assign86000_e131710_d_n7, assign86000_e131710_d_n8, assign86000_e131710_d_n9, assign86000_e131710_d_n10, assign86000_e131710_d_n13,) = {
    if (locals.var_flg_calcqover != 0.0) {
        let (assign86000_e131708, assign86000_e131708_d_n0, assign86000_e131708_d_n2, assign86000_e131708_d_n4, assign86000_e131708_d_n5, assign86000_e131708_d_n6, assign86000_e131708_d_n7, assign86000_e131708_d_n8, assign86000_e131708_d_n9, assign86000_e131708_d_n10, assign86000_e131708_d_n13,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
            } else {
                let assign86000_e131707: f64 = (-locals.var_tmf2);
                (assign86000_e131707, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn13),)
            }
        };
        (assign86000_e131708, assign86000_e131708_d_n0, assign86000_e131708_d_n2, assign86000_e131708_d_n4, assign86000_e131708_d_n5, assign86000_e131708_d_n6, assign86000_e131708_d_n7, assign86000_e131708_d_n8, assign86000_e131708_d_n9, assign86000_e131708_d_n10, assign86000_e131708_d_n13,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign86000_e131710;
        locals.var_tmf2_dn0 = assign86000_e131710_d_n0;
        locals.var_tmf2_dn2 = assign86000_e131710_d_n2;
        locals.var_tmf2_dn4 = assign86000_e131710_d_n4;
        locals.var_tmf2_dn5 = assign86000_e131710_d_n5;
        locals.var_tmf2_dn6 = assign86000_e131710_d_n6;
        locals.var_tmf2_dn7 = assign86000_e131710_d_n7;
        locals.var_tmf2_dn8 = assign86000_e131710_d_n8;
        locals.var_tmf2_dn9 = assign86000_e131710_d_n9;
        locals.var_tmf2_dn10 = assign86000_e131710_d_n10;
        locals.var_tmf2_dn13 = assign86000_e131710_d_n13;

        let (assign86010_e131719, assign86010_e131719_d_n0, assign86010_e131719_d_n2, assign86010_e131719_d_n4, assign86010_e131719_d_n5, assign86010_e131719_d_n6, assign86010_e131719_d_n7, assign86010_e131719_d_n8, assign86010_e131719_d_n9, assign86010_e131719_d_n10, assign86010_e131719_d_n13,) = {
    if (locals.var_flg_calcqover != 0.0) {
        let assign86010_e131714: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign86010_e131716: f64 = (assign86010_e131714 + locals.var_tmf2);
        let assign86010_e131717: f64 = (assign86010_e131716).sqrt();
        (assign86010_e131717, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign86010_e131717)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign86010_e131717)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign86010_e131717)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign86010_e131717)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign86010_e131717)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign86010_e131717)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign86010_e131717)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign86010_e131717)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign86010_e131717)), ((((locals.var_tmf1_dn13 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn13)) + locals.var_tmf2_dn13) / (2.0 * assign86010_e131717)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign86010_e131719;
        locals.var_tmf2_dn0 = assign86010_e131719_d_n0;
        locals.var_tmf2_dn2 = assign86010_e131719_d_n2;
        locals.var_tmf2_dn4 = assign86010_e131719_d_n4;
        locals.var_tmf2_dn5 = assign86010_e131719_d_n5;
        locals.var_tmf2_dn6 = assign86010_e131719_d_n6;
        locals.var_tmf2_dn7 = assign86010_e131719_d_n7;
        locals.var_tmf2_dn8 = assign86010_e131719_d_n8;
        locals.var_tmf2_dn9 = assign86010_e131719_d_n9;
        locals.var_tmf2_dn10 = assign86010_e131719_d_n10;
        locals.var_tmf2_dn13 = assign86010_e131719_d_n13;

        let (assign86020_e131729, assign86020_e131729_d_n0, assign86020_e131729_d_n2, assign86020_e131729_d_n4, assign86020_e131729_d_n5, assign86020_e131729_d_n6, assign86020_e131729_d_n7, assign86020_e131729_d_n8, assign86020_e131729_d_n9, assign86020_e131729_d_n10, assign86020_e131729_d_n13,) = {
    if (locals.var_flg_calcqover != 0.0) {
        let assign86020_e131725: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign86020_e131726: f64 = (1.0 + assign86020_e131725);
        let assign86020_e131727: f64 = (0.5 * assign86020_e131726);
        (assign86020_e131727, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn13 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn13)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign86020_e131729;
        locals.var_t0_dn0 = assign86020_e131729_d_n0;
        locals.var_t0_dn2 = assign86020_e131729_d_n2;
        locals.var_t0_dn4 = assign86020_e131729_d_n4;
        locals.var_t0_dn5 = assign86020_e131729_d_n5;
        locals.var_t0_dn6 = assign86020_e131729_d_n6;
        locals.var_t0_dn7 = assign86020_e131729_d_n7;
        locals.var_t0_dn8 = assign86020_e131729_d_n8;
        locals.var_t0_dn9 = assign86020_e131729_d_n9;
        locals.var_t0_dn10 = assign86020_e131729_d_n10;
        locals.var_t0_dn13 = assign86020_e131729_d_n13;

        let (assign86030_e131739, assign86030_e131739_d_n0, assign86030_e131739_d_n2, assign86030_e131739_d_n4, assign86030_e131739_d_n5, assign86030_e131739_d_n6, assign86030_e131739_d_n7, assign86030_e131739_d_n8, assign86030_e131739_d_n9, assign86030_e131739_d_n10, assign86030_e131739_d_n13,) = {
    if (locals.var_flg_calcqover != 0.0) {
        let assign86030_e131735: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign86030_e131736: f64 = (0.5 * assign86030_e131735);
        let assign86030_e131737: f64 = (0.8 - assign86030_e131736);
        (assign86030_e131737, (-(0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (-(0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), (-(0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4))), (-(0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5))), (-(0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), (-(0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))), (-(0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8))), (-(0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9))), (-(0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), (-(0.5 * (locals.var_tmf1_dn13 + locals.var_tmf2_dn13))),)
    } else {
        (locals.var_vbs_max_over__blk2008, locals.var_vbs_max_over__blk2008_dn0, locals.var_vbs_max_over__blk2008_dn2, locals.var_vbs_max_over__blk2008_dn4, locals.var_vbs_max_over__blk2008_dn5, locals.var_vbs_max_over__blk2008_dn6, locals.var_vbs_max_over__blk2008_dn7, locals.var_vbs_max_over__blk2008_dn8, locals.var_vbs_max_over__blk2008_dn9, locals.var_vbs_max_over__blk2008_dn10, locals.var_vbs_max_over__blk2008_dn13,)
    }
};
        locals.var_vbs_max_over__blk2008 = assign86030_e131739;
        locals.var_vbs_max_over__blk2008_dn0 = assign86030_e131739_d_n0;
        locals.var_vbs_max_over__blk2008_dn2 = assign86030_e131739_d_n2;
        locals.var_vbs_max_over__blk2008_dn4 = assign86030_e131739_d_n4;
        locals.var_vbs_max_over__blk2008_dn5 = assign86030_e131739_d_n5;
        locals.var_vbs_max_over__blk2008_dn6 = assign86030_e131739_d_n6;
        locals.var_vbs_max_over__blk2008_dn7 = assign86030_e131739_d_n7;
        locals.var_vbs_max_over__blk2008_dn8 = assign86030_e131739_d_n8;
        locals.var_vbs_max_over__blk2008_dn9 = assign86030_e131739_d_n9;
        locals.var_vbs_max_over__blk2008_dn10 = assign86030_e131739_d_n10;
        locals.var_vbs_max_over__blk2008_dn13 = assign86030_e131739_d_n13;

        let assign86040_e131743: f64 = (locals.var_vbs_max_over__blk2008 * 0.5);
        let assign86040_e131744: f64 = if locals.var_vbs_bnd_over__blk2009 > assign86040_e131743 { 1.0 } else { 0.0 };
        locals.var_guard2014 = assign86040_e131744;

        let (assign86050_e131752, assign86050_e131752_d_n0, assign86050_e131752_d_n2, assign86050_e131752_d_n4, assign86050_e131752_d_n5, assign86050_e131752_d_n6, assign86050_e131752_d_n7, assign86050_e131752_d_n8, assign86050_e131752_d_n9, assign86050_e131752_d_n10, assign86050_e131752_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard2014 != 0.0)) {
        let assign86050_e131750: f64 = (0.5 * locals.var_vbs_max_over__blk2008);
        (assign86050_e131750, (0.5 * locals.var_vbs_max_over__blk2008_dn0), (0.5 * locals.var_vbs_max_over__blk2008_dn2), (0.5 * locals.var_vbs_max_over__blk2008_dn4), (0.5 * locals.var_vbs_max_over__blk2008_dn5), (0.5 * locals.var_vbs_max_over__blk2008_dn6), (0.5 * locals.var_vbs_max_over__blk2008_dn7), (0.5 * locals.var_vbs_max_over__blk2008_dn8), (0.5 * locals.var_vbs_max_over__blk2008_dn9), (0.5 * locals.var_vbs_max_over__blk2008_dn10), (0.5 * locals.var_vbs_max_over__blk2008_dn13),)
    } else {
        (locals.var_vbs_bnd_over__blk2009, locals.var_vbs_bnd_over__blk2009_dn0, locals.var_vbs_bnd_over__blk2009_dn2, locals.var_vbs_bnd_over__blk2009_dn4, locals.var_vbs_bnd_over__blk2009_dn5, locals.var_vbs_bnd_over__blk2009_dn6, locals.var_vbs_bnd_over__blk2009_dn7, locals.var_vbs_bnd_over__blk2009_dn8, locals.var_vbs_bnd_over__blk2009_dn9, locals.var_vbs_bnd_over__blk2009_dn10, locals.var_vbs_bnd_over__blk2009_dn13,)
    }
};
        locals.var_vbs_bnd_over__blk2009 = assign86050_e131752;
        locals.var_vbs_bnd_over__blk2009_dn0 = assign86050_e131752_d_n0;
        locals.var_vbs_bnd_over__blk2009_dn2 = assign86050_e131752_d_n2;
        locals.var_vbs_bnd_over__blk2009_dn4 = assign86050_e131752_d_n4;
        locals.var_vbs_bnd_over__blk2009_dn5 = assign86050_e131752_d_n5;
        locals.var_vbs_bnd_over__blk2009_dn6 = assign86050_e131752_d_n6;
        locals.var_vbs_bnd_over__blk2009_dn7 = assign86050_e131752_d_n7;
        locals.var_vbs_bnd_over__blk2009_dn8 = assign86050_e131752_d_n8;
        locals.var_vbs_bnd_over__blk2009_dn9 = assign86050_e131752_d_n9;
        locals.var_vbs_bnd_over__blk2009_dn10 = assign86050_e131752_d_n10;
        locals.var_vbs_bnd_over__blk2009_dn13 = assign86050_e131752_d_n13;

        let assign86060_e131754: f64 = if param_given[338] { 1.0 } else { 0.0 };
        locals.var_guard2015 = assign86060_e131754;

        let (assign86070_e131760, assign86070_e131760_d_n0, assign86070_e131760_d_n2, assign86070_e131760_d_n4, assign86070_e131760_d_n5, assign86070_e131760_d_n6, assign86070_e131760_d_n7, assign86070_e131760_d_n8, assign86070_e131760_d_n9, assign86070_e131760_d_n10, assign86070_e131760_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard2015 != 0.0)) {
        (p.p338, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vbs_max_over__blk2008, locals.var_vbs_max_over__blk2008_dn0, locals.var_vbs_max_over__blk2008_dn2, locals.var_vbs_max_over__blk2008_dn4, locals.var_vbs_max_over__blk2008_dn5, locals.var_vbs_max_over__blk2008_dn6, locals.var_vbs_max_over__blk2008_dn7, locals.var_vbs_max_over__blk2008_dn8, locals.var_vbs_max_over__blk2008_dn9, locals.var_vbs_max_over__blk2008_dn10, locals.var_vbs_max_over__blk2008_dn13,)
    }
};
        locals.var_vbs_max_over__blk2008 = assign86070_e131760;
        locals.var_vbs_max_over__blk2008_dn0 = assign86070_e131760_d_n0;
        locals.var_vbs_max_over__blk2008_dn2 = assign86070_e131760_d_n2;
        locals.var_vbs_max_over__blk2008_dn4 = assign86070_e131760_d_n4;
        locals.var_vbs_max_over__blk2008_dn5 = assign86070_e131760_d_n5;
        locals.var_vbs_max_over__blk2008_dn6 = assign86070_e131760_d_n6;
        locals.var_vbs_max_over__blk2008_dn7 = assign86070_e131760_d_n7;
        locals.var_vbs_max_over__blk2008_dn8 = assign86070_e131760_d_n8;
        locals.var_vbs_max_over__blk2008_dn9 = assign86070_e131760_d_n9;
        locals.var_vbs_max_over__blk2008_dn10 = assign86070_e131760_d_n10;
        locals.var_vbs_max_over__blk2008_dn13 = assign86070_e131760_d_n13;

        let assign86080_e131762: f64 = if param_given[339] { 1.0 } else { 0.0 };
        locals.var_guard2016 = assign86080_e131762;

        let (assign86090_e131768, assign86090_e131768_d_n0, assign86090_e131768_d_n2, assign86090_e131768_d_n4, assign86090_e131768_d_n5, assign86090_e131768_d_n6, assign86090_e131768_d_n7, assign86090_e131768_d_n8, assign86090_e131768_d_n9, assign86090_e131768_d_n10, assign86090_e131768_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard2016 != 0.0)) {
        (p.p339, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vbs_bnd_over__blk2009, locals.var_vbs_bnd_over__blk2009_dn0, locals.var_vbs_bnd_over__blk2009_dn2, locals.var_vbs_bnd_over__blk2009_dn4, locals.var_vbs_bnd_over__blk2009_dn5, locals.var_vbs_bnd_over__blk2009_dn6, locals.var_vbs_bnd_over__blk2009_dn7, locals.var_vbs_bnd_over__blk2009_dn8, locals.var_vbs_bnd_over__blk2009_dn9, locals.var_vbs_bnd_over__blk2009_dn10, locals.var_vbs_bnd_over__blk2009_dn13,)
    }
};
        locals.var_vbs_bnd_over__blk2009 = assign86090_e131768;
        locals.var_vbs_bnd_over__blk2009_dn0 = assign86090_e131768_d_n0;
        locals.var_vbs_bnd_over__blk2009_dn2 = assign86090_e131768_d_n2;
        locals.var_vbs_bnd_over__blk2009_dn4 = assign86090_e131768_d_n4;
        locals.var_vbs_bnd_over__blk2009_dn5 = assign86090_e131768_d_n5;
        locals.var_vbs_bnd_over__blk2009_dn6 = assign86090_e131768_d_n6;
        locals.var_vbs_bnd_over__blk2009_dn7 = assign86090_e131768_d_n7;
        locals.var_vbs_bnd_over__blk2009_dn8 = assign86090_e131768_d_n8;
        locals.var_vbs_bnd_over__blk2009_dn9 = assign86090_e131768_d_n9;
        locals.var_vbs_bnd_over__blk2009_dn10 = assign86090_e131768_d_n10;
        locals.var_vbs_bnd_over__blk2009_dn13 = assign86090_e131768_d_n13;

        let assign86100_e131770: f64 = if param_given[338] { 1.0 } else { 0.0 };
        locals.var_guard2017 = assign86100_e131770;

        let (assign86110_e131781, assign86110_e131781_d_n0, assign86110_e131781_d_n2, assign86110_e131781_d_n4, assign86110_e131781_d_n5, assign86110_e131781_d_n6, assign86110_e131781_d_n7, assign86110_e131781_d_n8, assign86110_e131781_d_n9, assign86110_e131781_d_n10, assign86110_e131781_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2016 == 0.0)) && (locals.var_guard2017 != 0.0)) {
        let assign86110_e131779: f64 = (0.5 * locals.var_vbs_max_over__blk2008);
        (assign86110_e131779, (0.5 * locals.var_vbs_max_over__blk2008_dn0), (0.5 * locals.var_vbs_max_over__blk2008_dn2), (0.5 * locals.var_vbs_max_over__blk2008_dn4), (0.5 * locals.var_vbs_max_over__blk2008_dn5), (0.5 * locals.var_vbs_max_over__blk2008_dn6), (0.5 * locals.var_vbs_max_over__blk2008_dn7), (0.5 * locals.var_vbs_max_over__blk2008_dn8), (0.5 * locals.var_vbs_max_over__blk2008_dn9), (0.5 * locals.var_vbs_max_over__blk2008_dn10), (0.5 * locals.var_vbs_max_over__blk2008_dn13),)
    } else {
        (locals.var_vbs_bnd_over__blk2009, locals.var_vbs_bnd_over__blk2009_dn0, locals.var_vbs_bnd_over__blk2009_dn2, locals.var_vbs_bnd_over__blk2009_dn4, locals.var_vbs_bnd_over__blk2009_dn5, locals.var_vbs_bnd_over__blk2009_dn6, locals.var_vbs_bnd_over__blk2009_dn7, locals.var_vbs_bnd_over__blk2009_dn8, locals.var_vbs_bnd_over__blk2009_dn9, locals.var_vbs_bnd_over__blk2009_dn10, locals.var_vbs_bnd_over__blk2009_dn13,)
    }
};
        locals.var_vbs_bnd_over__blk2009 = assign86110_e131781;
        locals.var_vbs_bnd_over__blk2009_dn0 = assign86110_e131781_d_n0;
        locals.var_vbs_bnd_over__blk2009_dn2 = assign86110_e131781_d_n2;
        locals.var_vbs_bnd_over__blk2009_dn4 = assign86110_e131781_d_n4;
        locals.var_vbs_bnd_over__blk2009_dn5 = assign86110_e131781_d_n5;
        locals.var_vbs_bnd_over__blk2009_dn6 = assign86110_e131781_d_n6;
        locals.var_vbs_bnd_over__blk2009_dn7 = assign86110_e131781_d_n7;
        locals.var_vbs_bnd_over__blk2009_dn8 = assign86110_e131781_d_n8;
        locals.var_vbs_bnd_over__blk2009_dn9 = assign86110_e131781_d_n9;
        locals.var_vbs_bnd_over__blk2009_dn10 = assign86110_e131781_d_n10;
        locals.var_vbs_bnd_over__blk2009_dn13 = assign86110_e131781_d_n13;

        let assign86120_e131785: f64 = (locals.var_vbs_max_over__blk2008 * 0.5);
        let assign86120_e131786: f64 = if locals.var_vbs_bnd_over__blk2009 > assign86120_e131785 { 1.0 } else { 0.0 };
        locals.var_guard2018 = assign86120_e131786;

        let (assign86130_e131794, assign86130_e131794_d_n0, assign86130_e131794_d_n2, assign86130_e131794_d_n4, assign86130_e131794_d_n5, assign86130_e131794_d_n6, assign86130_e131794_d_n7, assign86130_e131794_d_n8, assign86130_e131794_d_n9, assign86130_e131794_d_n10, assign86130_e131794_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard2018 != 0.0)) {
        let assign86130_e131792: f64 = (0.5 * locals.var_vbs_max_over__blk2008);
        (assign86130_e131792, (0.5 * locals.var_vbs_max_over__blk2008_dn0), (0.5 * locals.var_vbs_max_over__blk2008_dn2), (0.5 * locals.var_vbs_max_over__blk2008_dn4), (0.5 * locals.var_vbs_max_over__blk2008_dn5), (0.5 * locals.var_vbs_max_over__blk2008_dn6), (0.5 * locals.var_vbs_max_over__blk2008_dn7), (0.5 * locals.var_vbs_max_over__blk2008_dn8), (0.5 * locals.var_vbs_max_over__blk2008_dn9), (0.5 * locals.var_vbs_max_over__blk2008_dn10), (0.5 * locals.var_vbs_max_over__blk2008_dn13),)
    } else {
        (locals.var_vbs_bnd_over__blk2009, locals.var_vbs_bnd_over__blk2009_dn0, locals.var_vbs_bnd_over__blk2009_dn2, locals.var_vbs_bnd_over__blk2009_dn4, locals.var_vbs_bnd_over__blk2009_dn5, locals.var_vbs_bnd_over__blk2009_dn6, locals.var_vbs_bnd_over__blk2009_dn7, locals.var_vbs_bnd_over__blk2009_dn8, locals.var_vbs_bnd_over__blk2009_dn9, locals.var_vbs_bnd_over__blk2009_dn10, locals.var_vbs_bnd_over__blk2009_dn13,)
    }
};
        locals.var_vbs_bnd_over__blk2009 = assign86130_e131794;
        locals.var_vbs_bnd_over__blk2009_dn0 = assign86130_e131794_d_n0;
        locals.var_vbs_bnd_over__blk2009_dn2 = assign86130_e131794_d_n2;
        locals.var_vbs_bnd_over__blk2009_dn4 = assign86130_e131794_d_n4;
        locals.var_vbs_bnd_over__blk2009_dn5 = assign86130_e131794_d_n5;
        locals.var_vbs_bnd_over__blk2009_dn6 = assign86130_e131794_d_n6;
        locals.var_vbs_bnd_over__blk2009_dn7 = assign86130_e131794_d_n7;
        locals.var_vbs_bnd_over__blk2009_dn8 = assign86130_e131794_d_n8;
        locals.var_vbs_bnd_over__blk2009_dn9 = assign86130_e131794_d_n9;
        locals.var_vbs_bnd_over__blk2009_dn10 = assign86130_e131794_d_n10;
        locals.var_vbs_bnd_over__blk2009_dn13 = assign86130_e131794_d_n13;

        let assign86140_e131797: f64 = if p.p38 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard2019 = assign86140_e131797;

        let (assign86150_e131804, assign86150_e131804_d_n0, assign86150_e131804_d_n2, assign86150_e131804_d_n4, assign86150_e131804_d_n5, assign86150_e131804_d_n6, assign86150_e131804_d_n7, assign86150_e131804_d_n8, assign86150_e131804_d_n9, assign86150_e131804_d_n10, assign86150_e131804_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard2019 != 0.0)) {
        let assign86150_e131802: f64 = (-locals.var_vxbgmt);
        (assign86150_e131802, (-locals.var_vxbgmt_dn0), (-locals.var_vxbgmt_dn2), (-locals.var_vxbgmt_dn4), (-locals.var_vxbgmt_dn5), (-locals.var_vxbgmt_dn6), (-locals.var_vxbgmt_dn7), (-locals.var_vxbgmt_dn8), (-locals.var_vxbgmt_dn9), (-locals.var_vxbgmt_dn10), (-locals.var_vxbgmt_dn13),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign86150_e131804;
        locals.var_t0_dn0 = assign86150_e131804_d_n0;
        locals.var_t0_dn2 = assign86150_e131804_d_n2;
        locals.var_t0_dn4 = assign86150_e131804_d_n4;
        locals.var_t0_dn5 = assign86150_e131804_d_n5;
        locals.var_t0_dn6 = assign86150_e131804_d_n6;
        locals.var_t0_dn7 = assign86150_e131804_d_n7;
        locals.var_t0_dn8 = assign86150_e131804_d_n8;
        locals.var_t0_dn9 = assign86150_e131804_d_n9;
        locals.var_t0_dn10 = assign86150_e131804_d_n10;
        locals.var_t0_dn13 = assign86150_e131804_d_n13;

        let assign86160_e131807: f64 = if locals.var_t0 > locals.var_vbs_bnd_over__blk2009 { 1.0 } else { 0.0 };
        locals.var_guard2020 = assign86160_e131807;

        let (assign86170_e131817, assign86170_e131817_d_n0, assign86170_e131817_d_n2, assign86170_e131817_d_n4, assign86170_e131817_d_n5, assign86170_e131817_d_n6, assign86170_e131817_d_n7, assign86170_e131817_d_n8, assign86170_e131817_d_n9, assign86170_e131817_d_n10, assign86170_e131817_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2019 != 0.0)) && (locals.var_guard2020 != 0.0)) {
        let assign86170_e131815: f64 = (locals.var_t0 - locals.var_vbs_bnd_over__blk2009);
        (assign86170_e131815, (locals.var_t0_dn0 - locals.var_vbs_bnd_over__blk2009_dn0), (locals.var_t0_dn2 - locals.var_vbs_bnd_over__blk2009_dn2), (locals.var_t0_dn4 - locals.var_vbs_bnd_over__blk2009_dn4), (locals.var_t0_dn5 - locals.var_vbs_bnd_over__blk2009_dn5), (locals.var_t0_dn6 - locals.var_vbs_bnd_over__blk2009_dn6), (locals.var_t0_dn7 - locals.var_vbs_bnd_over__blk2009_dn7), (locals.var_t0_dn8 - locals.var_vbs_bnd_over__blk2009_dn8), (locals.var_t0_dn9 - locals.var_vbs_bnd_over__blk2009_dn9), (locals.var_t0_dn10 - locals.var_vbs_bnd_over__blk2009_dn10), (locals.var_t0_dn13 - locals.var_vbs_bnd_over__blk2009_dn13),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign86170_e131817;
        locals.var_t1_dn0 = assign86170_e131817_d_n0;
        locals.var_t1_dn2 = assign86170_e131817_d_n2;
        locals.var_t1_dn4 = assign86170_e131817_d_n4;
        locals.var_t1_dn5 = assign86170_e131817_d_n5;
        locals.var_t1_dn6 = assign86170_e131817_d_n6;
        locals.var_t1_dn7 = assign86170_e131817_d_n7;
        locals.var_t1_dn8 = assign86170_e131817_d_n8;
        locals.var_t1_dn9 = assign86170_e131817_d_n9;
        locals.var_t1_dn10 = assign86170_e131817_d_n10;
        locals.var_t1_dn13 = assign86170_e131817_d_n13;

        let (assign86180_e131827, assign86180_e131827_d_n0, assign86180_e131827_d_n2, assign86180_e131827_d_n4, assign86180_e131827_d_n5, assign86180_e131827_d_n6, assign86180_e131827_d_n7, assign86180_e131827_d_n8, assign86180_e131827_d_n9, assign86180_e131827_d_n10, assign86180_e131827_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2019 != 0.0)) && (locals.var_guard2020 != 0.0)) {
        let assign86180_e131825: f64 = (locals.var_vbs_max_over__blk2008 - locals.var_vbs_bnd_over__blk2009);
        (assign86180_e131825, (locals.var_vbs_max_over__blk2008_dn0 - locals.var_vbs_bnd_over__blk2009_dn0), (locals.var_vbs_max_over__blk2008_dn2 - locals.var_vbs_bnd_over__blk2009_dn2), (locals.var_vbs_max_over__blk2008_dn4 - locals.var_vbs_bnd_over__blk2009_dn4), (locals.var_vbs_max_over__blk2008_dn5 - locals.var_vbs_bnd_over__blk2009_dn5), (locals.var_vbs_max_over__blk2008_dn6 - locals.var_vbs_bnd_over__blk2009_dn6), (locals.var_vbs_max_over__blk2008_dn7 - locals.var_vbs_bnd_over__blk2009_dn7), (locals.var_vbs_max_over__blk2008_dn8 - locals.var_vbs_bnd_over__blk2009_dn8), (locals.var_vbs_max_over__blk2008_dn9 - locals.var_vbs_bnd_over__blk2009_dn9), (locals.var_vbs_max_over__blk2008_dn10 - locals.var_vbs_bnd_over__blk2009_dn10), (locals.var_vbs_max_over__blk2008_dn13 - locals.var_vbs_bnd_over__blk2009_dn13),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign86180_e131827;
        locals.var_t2_dn0 = assign86180_e131827_d_n0;
        locals.var_t2_dn2 = assign86180_e131827_d_n2;
        locals.var_t2_dn4 = assign86180_e131827_d_n4;
        locals.var_t2_dn5 = assign86180_e131827_d_n5;
        locals.var_t2_dn6 = assign86180_e131827_d_n6;
        locals.var_t2_dn7 = assign86180_e131827_d_n7;
        locals.var_t2_dn8 = assign86180_e131827_d_n8;
        locals.var_t2_dn9 = assign86180_e131827_d_n9;
        locals.var_t2_dn10 = assign86180_e131827_d_n10;
        locals.var_t2_dn13 = assign86180_e131827_d_n13;

        let (assign86190_e131837, assign86190_e131837_d_n0, assign86190_e131837_d_n2, assign86190_e131837_d_n4, assign86190_e131837_d_n5, assign86190_e131837_d_n6, assign86190_e131837_d_n7, assign86190_e131837_d_n8, assign86190_e131837_d_n9, assign86190_e131837_d_n10, assign86190_e131837_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2019 != 0.0)) && (locals.var_guard2020 != 0.0)) {
        let assign86190_e131835: f64 = (locals.var_t1 / locals.var_t2);
        (assign86190_e131835, (((locals.var_t1_dn0 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn0)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn2 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn2)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn4 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn4)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn5 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn5)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn6 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn6)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn7 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn7)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn8 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn8)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn9 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn9)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn10 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn10)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn13 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn13)) / (locals.var_t2 * locals.var_t2)),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13,)
    }
};
        locals.var_tmf1 = assign86190_e131837;
        locals.var_tmf1_dn0 = assign86190_e131837_d_n0;
        locals.var_tmf1_dn2 = assign86190_e131837_d_n2;
        locals.var_tmf1_dn4 = assign86190_e131837_d_n4;
        locals.var_tmf1_dn5 = assign86190_e131837_d_n5;
        locals.var_tmf1_dn6 = assign86190_e131837_d_n6;
        locals.var_tmf1_dn7 = assign86190_e131837_d_n7;
        locals.var_tmf1_dn8 = assign86190_e131837_d_n8;
        locals.var_tmf1_dn9 = assign86190_e131837_d_n9;
        locals.var_tmf1_dn10 = assign86190_e131837_d_n10;
        locals.var_tmf1_dn13 = assign86190_e131837_d_n13;

        let (assign86200_e131847, assign86200_e131847_d_n0, assign86200_e131847_d_n2, assign86200_e131847_d_n4, assign86200_e131847_d_n5, assign86200_e131847_d_n6, assign86200_e131847_d_n7, assign86200_e131847_d_n8, assign86200_e131847_d_n9, assign86200_e131847_d_n10, assign86200_e131847_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2019 != 0.0)) && (locals.var_guard2020 != 0.0)) {
        let assign86200_e131845: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign86200_e131845, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)), ((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)), ((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn13 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn13)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign86200_e131847;
        locals.var_tmf2_dn0 = assign86200_e131847_d_n0;
        locals.var_tmf2_dn2 = assign86200_e131847_d_n2;
        locals.var_tmf2_dn4 = assign86200_e131847_d_n4;
        locals.var_tmf2_dn5 = assign86200_e131847_d_n5;
        locals.var_tmf2_dn6 = assign86200_e131847_d_n6;
        locals.var_tmf2_dn7 = assign86200_e131847_d_n7;
        locals.var_tmf2_dn8 = assign86200_e131847_d_n8;
        locals.var_tmf2_dn9 = assign86200_e131847_d_n9;
        locals.var_tmf2_dn10 = assign86200_e131847_d_n10;
        locals.var_tmf2_dn13 = assign86200_e131847_d_n13;

        let (assign86210_e131857, assign86210_e131857_d_n0, assign86210_e131857_d_n2, assign86210_e131857_d_n4, assign86210_e131857_d_n5, assign86210_e131857_d_n6, assign86210_e131857_d_n7, assign86210_e131857_d_n8, assign86210_e131857_d_n9, assign86210_e131857_d_n10, assign86210_e131857_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2019 != 0.0)) && (locals.var_guard2020 != 0.0)) {
        let assign86210_e131855: f64 = (locals.var_tmf2 * locals.var_tmf1);
        (assign86210_e131855, ((locals.var_tmf2_dn0 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn0)), ((locals.var_tmf2_dn2 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn2)), ((locals.var_tmf2_dn4 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn4)), ((locals.var_tmf2_dn5 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn5)), ((locals.var_tmf2_dn6 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn6)), ((locals.var_tmf2_dn7 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn7)), ((locals.var_tmf2_dn8 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn8)), ((locals.var_tmf2_dn9 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn9)), ((locals.var_tmf2_dn10 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn10)), ((locals.var_tmf2_dn13 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn13)),)
    } else {
        (locals.var_tmf3, locals.var_tmf3_dn0, locals.var_tmf3_dn2, locals.var_tmf3_dn4, locals.var_tmf3_dn5, locals.var_tmf3_dn6, locals.var_tmf3_dn7, locals.var_tmf3_dn8, locals.var_tmf3_dn9, locals.var_tmf3_dn10, locals.var_tmf3_dn13,)
    }
};
        locals.var_tmf3 = assign86210_e131857;
        locals.var_tmf3_dn0 = assign86210_e131857_d_n0;
        locals.var_tmf3_dn2 = assign86210_e131857_d_n2;
        locals.var_tmf3_dn4 = assign86210_e131857_d_n4;
        locals.var_tmf3_dn5 = assign86210_e131857_d_n5;
        locals.var_tmf3_dn6 = assign86210_e131857_d_n6;
        locals.var_tmf3_dn7 = assign86210_e131857_d_n7;
        locals.var_tmf3_dn8 = assign86210_e131857_d_n8;
        locals.var_tmf3_dn9 = assign86210_e131857_d_n9;
        locals.var_tmf3_dn10 = assign86210_e131857_d_n10;
        locals.var_tmf3_dn13 = assign86210_e131857_d_n13;

        let (assign86220_e131867, assign86220_e131867_d_n0, assign86220_e131867_d_n2, assign86220_e131867_d_n4, assign86220_e131867_d_n5, assign86220_e131867_d_n6, assign86220_e131867_d_n7, assign86220_e131867_d_n8, assign86220_e131867_d_n9, assign86220_e131867_d_n10, assign86220_e131867_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2019 != 0.0)) && (locals.var_guard2020 != 0.0)) {
        let assign86220_e131865: f64 = (locals.var_tmf2 * locals.var_tmf2);
        (assign86220_e131865, ((locals.var_tmf2_dn0 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn0)), ((locals.var_tmf2_dn2 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn2)), ((locals.var_tmf2_dn4 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn4)), ((locals.var_tmf2_dn5 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn5)), ((locals.var_tmf2_dn6 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn6)), ((locals.var_tmf2_dn7 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn7)), ((locals.var_tmf2_dn8 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn8)), ((locals.var_tmf2_dn9 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn9)), ((locals.var_tmf2_dn10 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn10)), ((locals.var_tmf2_dn13 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn13)),)
    } else {
        (locals.var_tmf4, locals.var_tmf4_dn0, locals.var_tmf4_dn2, locals.var_tmf4_dn4, locals.var_tmf4_dn5, locals.var_tmf4_dn6, locals.var_tmf4_dn7, locals.var_tmf4_dn8, locals.var_tmf4_dn9, locals.var_tmf4_dn10, locals.var_tmf4_dn13,)
    }
};
        locals.var_tmf4 = assign86220_e131867;
        locals.var_tmf4_dn0 = assign86220_e131867_d_n0;
        locals.var_tmf4_dn2 = assign86220_e131867_d_n2;
        locals.var_tmf4_dn4 = assign86220_e131867_d_n4;
        locals.var_tmf4_dn5 = assign86220_e131867_d_n5;
        locals.var_tmf4_dn6 = assign86220_e131867_d_n6;
        locals.var_tmf4_dn7 = assign86220_e131867_d_n7;
        locals.var_tmf4_dn8 = assign86220_e131867_d_n8;
        locals.var_tmf4_dn9 = assign86220_e131867_d_n9;
        locals.var_tmf4_dn10 = assign86220_e131867_d_n10;
        locals.var_tmf4_dn13 = assign86220_e131867_d_n13;

        let (assign86230_e131885, assign86230_e131885_d_n0, assign86230_e131885_d_n2, assign86230_e131885_d_n4, assign86230_e131885_d_n5, assign86230_e131885_d_n6, assign86230_e131885_d_n7, assign86230_e131885_d_n8, assign86230_e131885_d_n9, assign86230_e131885_d_n10, assign86230_e131885_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2019 != 0.0)) && (locals.var_guard2020 != 0.0)) {
        let assign86230_e131876: f64 = (1.0 + locals.var_tmf1);
        let assign86230_e131878: f64 = (assign86230_e131876 + locals.var_tmf2);
        let assign86230_e131880: f64 = (assign86230_e131878 + locals.var_tmf3);
        let assign86230_e131882: f64 = (assign86230_e131880 + locals.var_tmf4);
        let assign86230_e131883: f64 = (1.0 / assign86230_e131882);
        (assign86230_e131883, (-((((locals.var_tmf1_dn0 + locals.var_tmf2_dn0) + locals.var_tmf3_dn0) + locals.var_tmf4_dn0) / (assign86230_e131882 * assign86230_e131882))), (-((((locals.var_tmf1_dn2 + locals.var_tmf2_dn2) + locals.var_tmf3_dn2) + locals.var_tmf4_dn2) / (assign86230_e131882 * assign86230_e131882))), (-((((locals.var_tmf1_dn4 + locals.var_tmf2_dn4) + locals.var_tmf3_dn4) + locals.var_tmf4_dn4) / (assign86230_e131882 * assign86230_e131882))), (-((((locals.var_tmf1_dn5 + locals.var_tmf2_dn5) + locals.var_tmf3_dn5) + locals.var_tmf4_dn5) / (assign86230_e131882 * assign86230_e131882))), (-((((locals.var_tmf1_dn6 + locals.var_tmf2_dn6) + locals.var_tmf3_dn6) + locals.var_tmf4_dn6) / (assign86230_e131882 * assign86230_e131882))), (-((((locals.var_tmf1_dn7 + locals.var_tmf2_dn7) + locals.var_tmf3_dn7) + locals.var_tmf4_dn7) / (assign86230_e131882 * assign86230_e131882))), (-((((locals.var_tmf1_dn8 + locals.var_tmf2_dn8) + locals.var_tmf3_dn8) + locals.var_tmf4_dn8) / (assign86230_e131882 * assign86230_e131882))), (-((((locals.var_tmf1_dn9 + locals.var_tmf2_dn9) + locals.var_tmf3_dn9) + locals.var_tmf4_dn9) / (assign86230_e131882 * assign86230_e131882))), (-((((locals.var_tmf1_dn10 + locals.var_tmf2_dn10) + locals.var_tmf3_dn10) + locals.var_tmf4_dn10) / (assign86230_e131882 * assign86230_e131882))), (-((((locals.var_tmf1_dn13 + locals.var_tmf2_dn13) + locals.var_tmf3_dn13) + locals.var_tmf4_dn13) / (assign86230_e131882 * assign86230_e131882))),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn13,)
    }
};
        locals.var_tmf0 = assign86230_e131885;
        locals.var_tmf0_dn0 = assign86230_e131885_d_n0;
        locals.var_tmf0_dn2 = assign86230_e131885_d_n2;
        locals.var_tmf0_dn4 = assign86230_e131885_d_n4;
        locals.var_tmf0_dn5 = assign86230_e131885_d_n5;
        locals.var_tmf0_dn6 = assign86230_e131885_d_n6;
        locals.var_tmf0_dn7 = assign86230_e131885_d_n7;
        locals.var_tmf0_dn8 = assign86230_e131885_d_n8;
        locals.var_tmf0_dn9 = assign86230_e131885_d_n9;
        locals.var_tmf0_dn10 = assign86230_e131885_d_n10;
        locals.var_tmf0_dn13 = assign86230_e131885_d_n13;

        let (assign86240_e131910, assign86240_e131910_d_n0, assign86240_e131910_d_n2, assign86240_e131910_d_n4, assign86240_e131910_d_n5, assign86240_e131910_d_n6, assign86240_e131910_d_n7, assign86240_e131910_d_n8, assign86240_e131910_d_n9, assign86240_e131910_d_n10, assign86240_e131910_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2019 != 0.0)) && (locals.var_guard2020 != 0.0)) {
        let assign86240_e131894: f64 = (2.0 * locals.var_tmf1);
        let assign86240_e131895: f64 = (1.0 + assign86240_e131894);
        let assign86240_e131898: f64 = (3.0 * locals.var_tmf2);
        let assign86240_e131899: f64 = (assign86240_e131895 + assign86240_e131898);
        let assign86240_e131902: f64 = (4.0 * locals.var_tmf3);
        let assign86240_e131903: f64 = (assign86240_e131899 + assign86240_e131902);
        let assign86240_e131904: f64 = (-assign86240_e131903);
        let assign86240_e131906: f64 = (assign86240_e131904 * locals.var_tmf0);
        let assign86240_e131908: f64 = (assign86240_e131906 * locals.var_tmf0);
        (assign86240_e131908, (((((-(((2.0 * locals.var_tmf1_dn0) + (3.0 * locals.var_tmf2_dn0)) + (4.0 * locals.var_tmf3_dn0))) * locals.var_tmf0) + (assign86240_e131904 * locals.var_tmf0_dn0)) * locals.var_tmf0) + (assign86240_e131906 * locals.var_tmf0_dn0)), (((((-(((2.0 * locals.var_tmf1_dn2) + (3.0 * locals.var_tmf2_dn2)) + (4.0 * locals.var_tmf3_dn2))) * locals.var_tmf0) + (assign86240_e131904 * locals.var_tmf0_dn2)) * locals.var_tmf0) + (assign86240_e131906 * locals.var_tmf0_dn2)), (((((-(((2.0 * locals.var_tmf1_dn4) + (3.0 * locals.var_tmf2_dn4)) + (4.0 * locals.var_tmf3_dn4))) * locals.var_tmf0) + (assign86240_e131904 * locals.var_tmf0_dn4)) * locals.var_tmf0) + (assign86240_e131906 * locals.var_tmf0_dn4)), (((((-(((2.0 * locals.var_tmf1_dn5) + (3.0 * locals.var_tmf2_dn5)) + (4.0 * locals.var_tmf3_dn5))) * locals.var_tmf0) + (assign86240_e131904 * locals.var_tmf0_dn5)) * locals.var_tmf0) + (assign86240_e131906 * locals.var_tmf0_dn5)), (((((-(((2.0 * locals.var_tmf1_dn6) + (3.0 * locals.var_tmf2_dn6)) + (4.0 * locals.var_tmf3_dn6))) * locals.var_tmf0) + (assign86240_e131904 * locals.var_tmf0_dn6)) * locals.var_tmf0) + (assign86240_e131906 * locals.var_tmf0_dn6)), (((((-(((2.0 * locals.var_tmf1_dn7) + (3.0 * locals.var_tmf2_dn7)) + (4.0 * locals.var_tmf3_dn7))) * locals.var_tmf0) + (assign86240_e131904 * locals.var_tmf0_dn7)) * locals.var_tmf0) + (assign86240_e131906 * locals.var_tmf0_dn7)), (((((-(((2.0 * locals.var_tmf1_dn8) + (3.0 * locals.var_tmf2_dn8)) + (4.0 * locals.var_tmf3_dn8))) * locals.var_tmf0) + (assign86240_e131904 * locals.var_tmf0_dn8)) * locals.var_tmf0) + (assign86240_e131906 * locals.var_tmf0_dn8)), (((((-(((2.0 * locals.var_tmf1_dn9) + (3.0 * locals.var_tmf2_dn9)) + (4.0 * locals.var_tmf3_dn9))) * locals.var_tmf0) + (assign86240_e131904 * locals.var_tmf0_dn9)) * locals.var_tmf0) + (assign86240_e131906 * locals.var_tmf0_dn9)), (((((-(((2.0 * locals.var_tmf1_dn10) + (3.0 * locals.var_tmf2_dn10)) + (4.0 * locals.var_tmf3_dn10))) * locals.var_tmf0) + (assign86240_e131904 * locals.var_tmf0_dn10)) * locals.var_tmf0) + (assign86240_e131906 * locals.var_tmf0_dn10)), (((((-(((2.0 * locals.var_tmf1_dn13) + (3.0 * locals.var_tmf2_dn13)) + (4.0 * locals.var_tmf3_dn13))) * locals.var_tmf0) + (assign86240_e131904 * locals.var_tmf0_dn13)) * locals.var_tmf0) + (assign86240_e131906 * locals.var_tmf0_dn13)),)
    } else {
        (locals.var_t11, locals.var_t11_dn0, locals.var_t11_dn2, locals.var_t11_dn4, locals.var_t11_dn5, locals.var_t11_dn6, locals.var_t11_dn7, locals.var_t11_dn8, locals.var_t11_dn9, locals.var_t11_dn10, locals.var_t11_dn13,)
    }
};
        locals.var_t11 = assign86240_e131910;
        locals.var_t11_dn0 = assign86240_e131910_d_n0;
        locals.var_t11_dn2 = assign86240_e131910_d_n2;
        locals.var_t11_dn4 = assign86240_e131910_d_n4;
        locals.var_t11_dn5 = assign86240_e131910_d_n5;
        locals.var_t11_dn6 = assign86240_e131910_d_n6;
        locals.var_t11_dn7 = assign86240_e131910_d_n7;
        locals.var_t11_dn8 = assign86240_e131910_d_n8;
        locals.var_t11_dn9 = assign86240_e131910_d_n9;
        locals.var_t11_dn10 = assign86240_e131910_d_n10;
        locals.var_t11_dn13 = assign86240_e131910_d_n13;

        let (assign86250_e131922, assign86250_e131922_d_n0, assign86250_e131922_d_n2, assign86250_e131922_d_n4, assign86250_e131922_d_n5, assign86250_e131922_d_n6, assign86250_e131922_d_n7, assign86250_e131922_d_n8, assign86250_e131922_d_n9, assign86250_e131922_d_n10, assign86250_e131922_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2019 != 0.0)) && (locals.var_guard2020 != 0.0)) {
        let assign86250_e131919: f64 = (1.0 - locals.var_tmf0);
        let assign86250_e131920: f64 = (locals.var_t2 * assign86250_e131919);
        (assign86250_e131920, ((locals.var_t2_dn0 * assign86250_e131919) + (locals.var_t2 * (-locals.var_tmf0_dn0))), ((locals.var_t2_dn2 * assign86250_e131919) + (locals.var_t2 * (-locals.var_tmf0_dn2))), ((locals.var_t2_dn4 * assign86250_e131919) + (locals.var_t2 * (-locals.var_tmf0_dn4))), ((locals.var_t2_dn5 * assign86250_e131919) + (locals.var_t2 * (-locals.var_tmf0_dn5))), ((locals.var_t2_dn6 * assign86250_e131919) + (locals.var_t2 * (-locals.var_tmf0_dn6))), ((locals.var_t2_dn7 * assign86250_e131919) + (locals.var_t2 * (-locals.var_tmf0_dn7))), ((locals.var_t2_dn8 * assign86250_e131919) + (locals.var_t2 * (-locals.var_tmf0_dn8))), ((locals.var_t2_dn9 * assign86250_e131919) + (locals.var_t2 * (-locals.var_tmf0_dn9))), ((locals.var_t2_dn10 * assign86250_e131919) + (locals.var_t2 * (-locals.var_tmf0_dn10))), ((locals.var_t2_dn13 * assign86250_e131919) + (locals.var_t2 * (-locals.var_tmf0_dn13))),)
    } else {
        (locals.var_ty, locals.var_ty_dn0, locals.var_ty_dn2, locals.var_ty_dn4, locals.var_ty_dn5, locals.var_ty_dn6, locals.var_ty_dn7, locals.var_ty_dn8, locals.var_ty_dn9, locals.var_ty_dn10, locals.var_ty_dn13,)
    }
};
        locals.var_ty = assign86250_e131922;
        locals.var_ty_dn0 = assign86250_e131922_d_n0;
        locals.var_ty_dn2 = assign86250_e131922_d_n2;
        locals.var_ty_dn4 = assign86250_e131922_d_n4;
        locals.var_ty_dn5 = assign86250_e131922_d_n5;
        locals.var_ty_dn6 = assign86250_e131922_d_n6;
        locals.var_ty_dn7 = assign86250_e131922_d_n7;
        locals.var_ty_dn8 = assign86250_e131922_d_n8;
        locals.var_ty_dn9 = assign86250_e131922_d_n9;
        locals.var_ty_dn10 = assign86250_e131922_d_n10;
        locals.var_ty_dn13 = assign86250_e131922_d_n13;

        let (assign86260_e131936, assign86260_e131936_d_n0, assign86260_e131936_d_n2, assign86260_e131936_d_n4, assign86260_e131936_d_n5, assign86260_e131936_d_n6, assign86260_e131936_d_n7, assign86260_e131936_d_n8, assign86260_e131936_d_n9, assign86260_e131936_d_n10, assign86260_e131936_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2019 != 0.0)) && (locals.var_guard2020 != 0.0)) {
        let assign86260_e131930: f64 = (1.0 - locals.var_tmf0);
        let assign86260_e131933: f64 = (locals.var_tmf1 * locals.var_t11);
        let assign86260_e131934: f64 = (assign86260_e131930 + assign86260_e131933);
        (assign86260_e131934, ((-locals.var_tmf0_dn0) + ((locals.var_tmf1_dn0 * locals.var_t11) + (locals.var_tmf1 * locals.var_t11_dn0))), ((-locals.var_tmf0_dn2) + ((locals.var_tmf1_dn2 * locals.var_t11) + (locals.var_tmf1 * locals.var_t11_dn2))), ((-locals.var_tmf0_dn4) + ((locals.var_tmf1_dn4 * locals.var_t11) + (locals.var_tmf1 * locals.var_t11_dn4))), ((-locals.var_tmf0_dn5) + ((locals.var_tmf1_dn5 * locals.var_t11) + (locals.var_tmf1 * locals.var_t11_dn5))), ((-locals.var_tmf0_dn6) + ((locals.var_tmf1_dn6 * locals.var_t11) + (locals.var_tmf1 * locals.var_t11_dn6))), ((-locals.var_tmf0_dn7) + ((locals.var_tmf1_dn7 * locals.var_t11) + (locals.var_tmf1 * locals.var_t11_dn7))), ((-locals.var_tmf0_dn8) + ((locals.var_tmf1_dn8 * locals.var_t11) + (locals.var_tmf1 * locals.var_t11_dn8))), ((-locals.var_tmf0_dn9) + ((locals.var_tmf1_dn9 * locals.var_t11) + (locals.var_tmf1 * locals.var_t11_dn9))), ((-locals.var_tmf0_dn10) + ((locals.var_tmf1_dn10 * locals.var_t11) + (locals.var_tmf1 * locals.var_t11_dn10))), ((-locals.var_tmf0_dn13) + ((locals.var_tmf1_dn13 * locals.var_t11) + (locals.var_tmf1 * locals.var_t11_dn13))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign86260_e131936;
        locals.var_t0_dn0 = assign86260_e131936_d_n0;
        locals.var_t0_dn2 = assign86260_e131936_d_n2;
        locals.var_t0_dn4 = assign86260_e131936_d_n4;
        locals.var_t0_dn5 = assign86260_e131936_d_n5;
        locals.var_t0_dn6 = assign86260_e131936_d_n6;
        locals.var_t0_dn7 = assign86260_e131936_d_n7;
        locals.var_t0_dn8 = assign86260_e131936_d_n8;
        locals.var_t0_dn9 = assign86260_e131936_d_n9;
        locals.var_t0_dn10 = assign86260_e131936_d_n10;
        locals.var_t0_dn13 = assign86260_e131936_d_n13;

        let (assign86270_e131945, assign86270_e131945_d_n0, assign86270_e131945_d_n2, assign86270_e131945_d_n4, assign86270_e131945_d_n5, assign86270_e131945_d_n6, assign86270_e131945_d_n7, assign86270_e131945_d_n8, assign86270_e131945_d_n9, assign86270_e131945_d_n10, assign86270_e131945_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2019 != 0.0)) && (locals.var_guard2020 != 0.0)) {
        let assign86270_e131943: f64 = (-locals.var_t11);
        (assign86270_e131943, (-locals.var_t11_dn0), (-locals.var_t11_dn2), (-locals.var_t11_dn4), (-locals.var_t11_dn5), (-locals.var_t11_dn6), (-locals.var_t11_dn7), (-locals.var_t11_dn8), (-locals.var_t11_dn9), (-locals.var_t11_dn10), (-locals.var_t11_dn13),)
    } else {
        (locals.var_t11, locals.var_t11_dn0, locals.var_t11_dn2, locals.var_t11_dn4, locals.var_t11_dn5, locals.var_t11_dn6, locals.var_t11_dn7, locals.var_t11_dn8, locals.var_t11_dn9, locals.var_t11_dn10, locals.var_t11_dn13,)
    }
};
        locals.var_t11 = assign86270_e131945;
        locals.var_t11_dn0 = assign86270_e131945_d_n0;
        locals.var_t11_dn2 = assign86270_e131945_d_n2;
        locals.var_t11_dn4 = assign86270_e131945_d_n4;
        locals.var_t11_dn5 = assign86270_e131945_d_n5;
        locals.var_t11_dn6 = assign86270_e131945_d_n6;
        locals.var_t11_dn7 = assign86270_e131945_d_n7;
        locals.var_t11_dn8 = assign86270_e131945_d_n8;
        locals.var_t11_dn9 = assign86270_e131945_d_n9;
        locals.var_t11_dn10 = assign86270_e131945_d_n10;
        locals.var_t11_dn13 = assign86270_e131945_d_n13;

    }

    pub(super) fn stamp_transient_block_303(
        locals: &mut StampLocals,
    ) {
        let (assign86280_e131955, assign86280_e131955_d_n0, assign86280_e131955_d_n2, assign86280_e131955_d_n4, assign86280_e131955_d_n5, assign86280_e131955_d_n6, assign86280_e131955_d_n7, assign86280_e131955_d_n8, assign86280_e131955_d_n9, assign86280_e131955_d_n10, assign86280_e131955_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2019 != 0.0)) && (locals.var_guard2020 != 0.0)) {
        let assign86280_e131953: f64 = (locals.var_vbs_bnd_over__blk2009 + locals.var_ty);
        (assign86280_e131953, (locals.var_vbs_bnd_over__blk2009_dn0 + locals.var_ty_dn0), (locals.var_vbs_bnd_over__blk2009_dn2 + locals.var_ty_dn2), (locals.var_vbs_bnd_over__blk2009_dn4 + locals.var_ty_dn4), (locals.var_vbs_bnd_over__blk2009_dn5 + locals.var_ty_dn5), (locals.var_vbs_bnd_over__blk2009_dn6 + locals.var_ty_dn6), (locals.var_vbs_bnd_over__blk2009_dn7 + locals.var_ty_dn7), (locals.var_vbs_bnd_over__blk2009_dn8 + locals.var_ty_dn8), (locals.var_vbs_bnd_over__blk2009_dn9 + locals.var_ty_dn9), (locals.var_vbs_bnd_over__blk2009_dn10 + locals.var_ty_dn10), (locals.var_vbs_bnd_over__blk2009_dn13 + locals.var_ty_dn13),)
    } else {
        (locals.var_t10, locals.var_t10_dn0, locals.var_t10_dn2, locals.var_t10_dn4, locals.var_t10_dn5, locals.var_t10_dn6, locals.var_t10_dn7, locals.var_t10_dn8, locals.var_t10_dn9, locals.var_t10_dn10, locals.var_t10_dn13,)
    }
};
        locals.var_t10 = assign86280_e131955;
        locals.var_t10_dn0 = assign86280_e131955_d_n0;
        locals.var_t10_dn2 = assign86280_e131955_d_n2;
        locals.var_t10_dn4 = assign86280_e131955_d_n4;
        locals.var_t10_dn5 = assign86280_e131955_d_n5;
        locals.var_t10_dn6 = assign86280_e131955_d_n6;
        locals.var_t10_dn7 = assign86280_e131955_d_n7;
        locals.var_t10_dn8 = assign86280_e131955_d_n8;
        locals.var_t10_dn9 = assign86280_e131955_d_n9;
        locals.var_t10_dn10 = assign86280_e131955_d_n10;
        locals.var_t10_dn13 = assign86280_e131955_d_n13;

        let (assign86290_e131964, assign86290_e131964_d_n0, assign86290_e131964_d_n2, assign86290_e131964_d_n4, assign86290_e131964_d_n5, assign86290_e131964_d_n6, assign86290_e131964_d_n7, assign86290_e131964_d_n8, assign86290_e131964_d_n9, assign86290_e131964_d_n10, assign86290_e131964_d_n13,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2019 != 0.0)) && (locals.var_guard2020 == 0.0)) {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    } else {
        (locals.var_t10, locals.var_t10_dn0, locals.var_t10_dn2, locals.var_t10_dn4, locals.var_t10_dn5, locals.var_t10_dn6, locals.var_t10_dn7, locals.var_t10_dn8, locals.var_t10_dn9, locals.var_t10_dn10, locals.var_t10_dn13,)
    }
};
        locals.var_t10 = assign86290_e131964;
        locals.var_t10_dn0 = assign86290_e131964_d_n0;
        locals.var_t10_dn2 = assign86290_e131964_d_n2;
        locals.var_t10_dn4 = assign86290_e131964_d_n4;
        locals.var_t10_dn5 = assign86290_e131964_d_n5;
        locals.var_t10_dn6 = assign86290_e131964_d_n6;
        locals.var_t10_dn7 = assign86290_e131964_d_n7;
        locals.var_t10_dn8 = assign86290_e131964_d_n8;
        locals.var_t10_dn9 = assign86290_e131964_d_n9;
        locals.var_t10_dn10 = assign86290_e131964_d_n10;
        locals.var_t10_dn13 = assign86290_e131964_d_n13;

        let (assign86300_e131971, assign86300_e131971_d_n0, assign86300_e131971_d_n2, assign86300_e131971_d_n4, assign86300_e131971_d_n5, assign86300_e131971_d_n6, assign86300_e131971_d_n7, assign86300_e131971_d_n8, assign86300_e131971_d_n9, assign86300_e131971_d_n10, assign86300_e131971_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard2019 != 0.0)) {
        let assign86300_e131969: f64 = (-locals.var_t10);
        (assign86300_e131969, (-locals.var_t10_dn0), (-locals.var_t10_dn2), (-locals.var_t10_dn4), (-locals.var_t10_dn5), (-locals.var_t10_dn6), (-locals.var_t10_dn7), (-locals.var_t10_dn8), (-locals.var_t10_dn9), (-locals.var_t10_dn10), (-locals.var_t10_dn13),)
    } else {
        (locals.var_vxbgmtcl, locals.var_vxbgmtcl_dn0, locals.var_vxbgmtcl_dn2, locals.var_vxbgmtcl_dn4, locals.var_vxbgmtcl_dn5, locals.var_vxbgmtcl_dn6, locals.var_vxbgmtcl_dn7, locals.var_vxbgmtcl_dn8, locals.var_vxbgmtcl_dn9, locals.var_vxbgmtcl_dn10, locals.var_vxbgmtcl_dn13,)
    }
};
        locals.var_vxbgmtcl = assign86300_e131971;
        locals.var_vxbgmtcl_dn0 = assign86300_e131971_d_n0;
        locals.var_vxbgmtcl_dn2 = assign86300_e131971_d_n2;
        locals.var_vxbgmtcl_dn4 = assign86300_e131971_d_n4;
        locals.var_vxbgmtcl_dn5 = assign86300_e131971_d_n5;
        locals.var_vxbgmtcl_dn6 = assign86300_e131971_d_n6;
        locals.var_vxbgmtcl_dn7 = assign86300_e131971_d_n7;
        locals.var_vxbgmtcl_dn8 = assign86300_e131971_d_n8;
        locals.var_vxbgmtcl_dn9 = assign86300_e131971_d_n9;
        locals.var_vxbgmtcl_dn10 = assign86300_e131971_d_n10;
        locals.var_vxbgmtcl_dn13 = assign86300_e131971_d_n13;

        let (assign86310_e131978, assign86310_e131978_d_n0, assign86310_e131978_d_n2, assign86310_e131978_d_n4, assign86310_e131978_d_n5, assign86310_e131978_d_n6, assign86310_e131978_d_n7, assign86310_e131978_d_n8, assign86310_e131978_d_n9, assign86310_e131978_d_n10, assign86310_e131978_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard2019 == 0.0)) {
        (locals.var_vxbgmt, locals.var_vxbgmt_dn0, locals.var_vxbgmt_dn2, locals.var_vxbgmt_dn4, locals.var_vxbgmt_dn5, locals.var_vxbgmt_dn6, locals.var_vxbgmt_dn7, locals.var_vxbgmt_dn8, locals.var_vxbgmt_dn9, locals.var_vxbgmt_dn10, locals.var_vxbgmt_dn13,)
    } else {
        (locals.var_vxbgmtcl, locals.var_vxbgmtcl_dn0, locals.var_vxbgmtcl_dn2, locals.var_vxbgmtcl_dn4, locals.var_vxbgmtcl_dn5, locals.var_vxbgmtcl_dn6, locals.var_vxbgmtcl_dn7, locals.var_vxbgmtcl_dn8, locals.var_vxbgmtcl_dn9, locals.var_vxbgmtcl_dn10, locals.var_vxbgmtcl_dn13,)
    }
};
        locals.var_vxbgmtcl = assign86310_e131978;
        locals.var_vxbgmtcl_dn0 = assign86310_e131978_d_n0;
        locals.var_vxbgmtcl_dn2 = assign86310_e131978_d_n2;
        locals.var_vxbgmtcl_dn4 = assign86310_e131978_d_n4;
        locals.var_vxbgmtcl_dn5 = assign86310_e131978_d_n5;
        locals.var_vxbgmtcl_dn6 = assign86310_e131978_d_n6;
        locals.var_vxbgmtcl_dn7 = assign86310_e131978_d_n7;
        locals.var_vxbgmtcl_dn8 = assign86310_e131978_d_n8;
        locals.var_vxbgmtcl_dn9 = assign86310_e131978_d_n9;
        locals.var_vxbgmtcl_dn10 = assign86310_e131978_d_n10;
        locals.var_vxbgmtcl_dn13 = assign86310_e131978_d_n13;

        let (assign86320_e131984, assign86320_e131984_d_n0, assign86320_e131984_d_n2, assign86320_e131984_d_n4, assign86320_e131984_d_n5, assign86320_e131984_d_n6, assign86320_e131984_d_n7, assign86320_e131984_d_n8, assign86320_e131984_d_n9, assign86320_e131984_d_n10, assign86320_e131984_d_n13,) = {
    if (locals.var_flg_calcqover != 0.0) {
        let assign86320_e131982: f64 = (locals.var_cnst0over_func / locals.var_cox0_func);
        (assign86320_e131982, (locals.var_cnst0over_func_dn0 / locals.var_cox0_func), (locals.var_cnst0over_func_dn2 / locals.var_cox0_func), (locals.var_cnst0over_func_dn4 / locals.var_cox0_func), (locals.var_cnst0over_func_dn5 / locals.var_cox0_func), (locals.var_cnst0over_func_dn6 / locals.var_cox0_func), (locals.var_cnst0over_func_dn7 / locals.var_cox0_func), (locals.var_cnst0over_func_dn8 / locals.var_cox0_func), (locals.var_cnst0over_func_dn9 / locals.var_cox0_func), (locals.var_cnst0over_func_dn10 / locals.var_cox0_func), (locals.var_cnst0over_func_dn13 / locals.var_cox0_func),)
    } else {
        (locals.var_fac1, locals.var_fac1_dn0, locals.var_fac1_dn2, locals.var_fac1_dn4, locals.var_fac1_dn5, locals.var_fac1_dn6, locals.var_fac1_dn7, locals.var_fac1_dn8, locals.var_fac1_dn9, locals.var_fac1_dn10, locals.var_fac1_dn13,)
    }
};
        locals.var_fac1 = assign86320_e131984;
        locals.var_fac1_dn0 = assign86320_e131984_d_n0;
        locals.var_fac1_dn2 = assign86320_e131984_d_n2;
        locals.var_fac1_dn4 = assign86320_e131984_d_n4;
        locals.var_fac1_dn5 = assign86320_e131984_d_n5;
        locals.var_fac1_dn6 = assign86320_e131984_d_n6;
        locals.var_fac1_dn7 = assign86320_e131984_d_n7;
        locals.var_fac1_dn8 = assign86320_e131984_d_n8;
        locals.var_fac1_dn9 = assign86320_e131984_d_n9;
        locals.var_fac1_dn10 = assign86320_e131984_d_n10;
        locals.var_fac1_dn13 = assign86320_e131984_d_n13;

        let (assign86330_e131990, assign86330_e131990_d_n0, assign86330_e131990_d_n2, assign86330_e131990_d_n4, assign86330_e131990_d_n5, assign86330_e131990_d_n6, assign86330_e131990_d_n7, assign86330_e131990_d_n8, assign86330_e131990_d_n9, assign86330_e131990_d_n10, assign86330_e131990_d_n13,) = {
    if (locals.var_flg_calcqover != 0.0) {
        let assign86330_e131988: f64 = (locals.var_fac1 * locals.var_fac1);
        (assign86330_e131988, ((locals.var_fac1_dn0 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn0)), ((locals.var_fac1_dn2 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn2)), ((locals.var_fac1_dn4 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn4)), ((locals.var_fac1_dn5 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn5)), ((locals.var_fac1_dn6 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn6)), ((locals.var_fac1_dn7 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn7)), ((locals.var_fac1_dn8 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn8)), ((locals.var_fac1_dn9 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn9)), ((locals.var_fac1_dn10 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn10)), ((locals.var_fac1_dn13 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn13)),)
    } else {
        (locals.var_fac1p2, locals.var_fac1p2_dn0, locals.var_fac1p2_dn2, locals.var_fac1p2_dn4, locals.var_fac1p2_dn5, locals.var_fac1p2_dn6, locals.var_fac1p2_dn7, locals.var_fac1p2_dn8, locals.var_fac1p2_dn9, locals.var_fac1p2_dn10, locals.var_fac1p2_dn13,)
    }
};
        locals.var_fac1p2 = assign86330_e131990;
        locals.var_fac1p2_dn0 = assign86330_e131990_d_n0;
        locals.var_fac1p2_dn2 = assign86330_e131990_d_n2;
        locals.var_fac1p2_dn4 = assign86330_e131990_d_n4;
        locals.var_fac1p2_dn5 = assign86330_e131990_d_n5;
        locals.var_fac1p2_dn6 = assign86330_e131990_d_n6;
        locals.var_fac1p2_dn7 = assign86330_e131990_d_n7;
        locals.var_fac1p2_dn8 = assign86330_e131990_d_n8;
        locals.var_fac1p2_dn9 = assign86330_e131990_d_n9;
        locals.var_fac1p2_dn10 = assign86330_e131990_d_n10;
        locals.var_fac1p2_dn13 = assign86330_e131990_d_n13;

        let (assign86340_e131997, assign86340_e131997_d_n2, assign86340_e131997_d_n6, assign86340_e131997_d_n7, assign86340_e131997_d_n8,) = {
    if (locals.var_flg_calcqover != 0.0) {
        let assign86340_e131993: f64 = (-locals.var_vgbgmt);
        let assign86340_e131995: f64 = (assign86340_e131993 + locals.var_uc_vfbover);
        (assign86340_e131995, (-locals.var_vgbgmt_dn2), (-locals.var_vgbgmt_dn6), (-locals.var_vgbgmt_dn7), (-locals.var_vgbgmt_dn8),)
    } else {
        (locals.var_vgpld, locals.var_vgpld_dn2, locals.var_vgpld_dn6, locals.var_vgpld_dn7, locals.var_vgpld_dn8,)
    }
};
        locals.var_vgpld = assign86340_e131997;
        locals.var_vgpld_dn2 = assign86340_e131997_d_n2;
        locals.var_vgpld_dn6 = assign86340_e131997_d_n6;
        locals.var_vgpld_dn7 = assign86340_e131997_d_n7;
        locals.var_vgpld_dn8 = assign86340_e131997_d_n8;

        let (assign86350_e132006,) = {
    if (locals.var_flg_calcqover != 0.0) {
        let assign86350_e132000: f64 = (-locals.var_vxbgmtcl);
        let assign86350_e132003: f64 = (10.0 * 2.220446049250313e-16);
        let assign86350_e132004: f64 = (assign86350_e132000 + assign86350_e132003);
        (assign86350_e132004,)
    } else {
        (locals.var_vgb_fb_ld,)
    }
};
        locals.var_vgb_fb_ld = assign86350_e132006;

        let (assign86360_e132010, assign86360_e132010_d_n0, assign86360_e132010_d_n2, assign86360_e132010_d_n4, assign86360_e132010_d_n5, assign86360_e132010_d_n6, assign86360_e132010_d_n7, assign86360_e132010_d_n8, assign86360_e132010_d_n9, assign86360_e132010_d_n10, assign86360_e132010_d_n13,) = {
    if (locals.var_flg_calcqover != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_q_dep_ld__blk2003, locals.var_q_dep_ld__blk2003_dn0, locals.var_q_dep_ld__blk2003_dn2, locals.var_q_dep_ld__blk2003_dn4, locals.var_q_dep_ld__blk2003_dn5, locals.var_q_dep_ld__blk2003_dn6, locals.var_q_dep_ld__blk2003_dn7, locals.var_q_dep_ld__blk2003_dn8, locals.var_q_dep_ld__blk2003_dn9, locals.var_q_dep_ld__blk2003_dn10, locals.var_q_dep_ld__blk2003_dn13,)
    }
};
        locals.var_q_dep_ld__blk2003 = assign86360_e132010;
        locals.var_q_dep_ld__blk2003_dn0 = assign86360_e132010_d_n0;
        locals.var_q_dep_ld__blk2003_dn2 = assign86360_e132010_d_n2;
        locals.var_q_dep_ld__blk2003_dn4 = assign86360_e132010_d_n4;
        locals.var_q_dep_ld__blk2003_dn5 = assign86360_e132010_d_n5;
        locals.var_q_dep_ld__blk2003_dn6 = assign86360_e132010_d_n6;
        locals.var_q_dep_ld__blk2003_dn7 = assign86360_e132010_d_n7;
        locals.var_q_dep_ld__blk2003_dn8 = assign86360_e132010_d_n8;
        locals.var_q_dep_ld__blk2003_dn9 = assign86360_e132010_d_n9;
        locals.var_q_dep_ld__blk2003_dn10 = assign86360_e132010_d_n10;
        locals.var_q_dep_ld__blk2003_dn13 = assign86360_e132010_d_n13;

        let (assign86370_e132016,) = {
    if (locals.var_flg_calcqover != 0.0) {
        let assign86370_e132014: f64 = (1.6021918e-19 * locals.var_nover_func);
        (assign86370_e132014,)
    } else {
        (locals.var_q_nsubld__blk2004,)
    }
};
        locals.var_q_nsubld__blk2004 = assign86370_e132016;

        let (assign86380_e132022, assign86380_e132022_d_n0, assign86380_e132022_d_n2, assign86380_e132022_d_n4, assign86380_e132022_d_n5, assign86380_e132022_d_n6, assign86380_e132022_d_n7, assign86380_e132022_d_n8, assign86380_e132022_d_n9, assign86380_e132022_d_n10, assign86380_e132022_d_n13,) = {
    if (locals.var_flg_calcqover != 0.0) {
        let assign86380_e132020: f64 = (locals.var_nin / locals.var_nover_func);
        (assign86380_e132020, (locals.var_nin_dn0 / locals.var_nover_func), (locals.var_nin_dn2 / locals.var_nover_func), (locals.var_nin_dn4 / locals.var_nover_func), (locals.var_nin_dn5 / locals.var_nover_func), (locals.var_nin_dn6 / locals.var_nover_func), (locals.var_nin_dn7 / locals.var_nover_func), (locals.var_nin_dn8 / locals.var_nover_func), (locals.var_nin_dn9 / locals.var_nover_func), (locals.var_nin_dn10 / locals.var_nover_func), (locals.var_nin_dn13 / locals.var_nover_func),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign86380_e132022;
        locals.var_t0_dn0 = assign86380_e132022_d_n0;
        locals.var_t0_dn2 = assign86380_e132022_d_n2;
        locals.var_t0_dn4 = assign86380_e132022_d_n4;
        locals.var_t0_dn5 = assign86380_e132022_d_n5;
        locals.var_t0_dn6 = assign86380_e132022_d_n6;
        locals.var_t0_dn7 = assign86380_e132022_d_n7;
        locals.var_t0_dn8 = assign86380_e132022_d_n8;
        locals.var_t0_dn9 = assign86380_e132022_d_n9;
        locals.var_t0_dn10 = assign86380_e132022_d_n10;
        locals.var_t0_dn13 = assign86380_e132022_d_n13;

        let (assign86390_e132028, assign86390_e132028_d_n0, assign86390_e132028_d_n2, assign86390_e132028_d_n4, assign86390_e132028_d_n5, assign86390_e132028_d_n6, assign86390_e132028_d_n7, assign86390_e132028_d_n8, assign86390_e132028_d_n9, assign86390_e132028_d_n10, assign86390_e132028_d_n13,) = {
    if (locals.var_flg_calcqover != 0.0) {
        let assign86390_e132026: f64 = (locals.var_t0 * locals.var_t0);
        (assign86390_e132026, ((locals.var_t0_dn0 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn0)), ((locals.var_t0_dn2 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn2)), ((locals.var_t0_dn4 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn4)), ((locals.var_t0_dn5 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn5)), ((locals.var_t0_dn6 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn6)), ((locals.var_t0_dn7 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn7)), ((locals.var_t0_dn8 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn8)), ((locals.var_t0_dn9 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn9)), ((locals.var_t0_dn10 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn10)), ((locals.var_t0_dn13 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn13)),)
    } else {
        (locals.var_cnst1over, locals.var_cnst1over_dn0, locals.var_cnst1over_dn2, locals.var_cnst1over_dn4, locals.var_cnst1over_dn5, locals.var_cnst1over_dn6, locals.var_cnst1over_dn7, locals.var_cnst1over_dn8, locals.var_cnst1over_dn9, locals.var_cnst1over_dn10, locals.var_cnst1over_dn13,)
    }
};
        locals.var_cnst1over = assign86390_e132028;
        locals.var_cnst1over_dn0 = assign86390_e132028_d_n0;
        locals.var_cnst1over_dn2 = assign86390_e132028_d_n2;
        locals.var_cnst1over_dn4 = assign86390_e132028_d_n4;
        locals.var_cnst1over_dn5 = assign86390_e132028_d_n5;
        locals.var_cnst1over_dn6 = assign86390_e132028_d_n6;
        locals.var_cnst1over_dn7 = assign86390_e132028_d_n7;
        locals.var_cnst1over_dn8 = assign86390_e132028_d_n8;
        locals.var_cnst1over_dn9 = assign86390_e132028_d_n9;
        locals.var_cnst1over_dn10 = assign86390_e132028_d_n10;
        locals.var_cnst1over_dn13 = assign86390_e132028_d_n13;

        let assign86400_e132031: f64 = (-locals.var_vxbgmtcl);
        let assign86400_e132032: f64 = (locals.var_beta * assign86400_e132031);
        let assign86400_e132034: f64 = if assign86400_e132032 >= 500.0 { 1.0 } else { 0.0 };
        locals.var_guard2021 = assign86400_e132034;

        let (assign86410_e132049, assign86410_e132049_d_n0, assign86410_e132049_d_n2, assign86410_e132049_d_n4, assign86410_e132049_d_n5, assign86410_e132049_d_n6, assign86410_e132049_d_n7, assign86410_e132049_d_n8, assign86410_e132049_d_n9, assign86410_e132049_d_n10, assign86410_e132049_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard2021 != 0.0)) {
        let assign86410_e132042: f64 = (-locals.var_vxbgmtcl);
        let assign86410_e132043: f64 = (locals.var_beta * assign86410_e132042);
        let assign86410_e132044: f64 = (1.0 + assign86410_e132043);
        let assign86410_e132046: f64 = (assign86410_e132044 - 500.0);
        let assign86410_e132047: f64 = (1.403592217853e217 * assign86410_e132046);
        (assign86410_e132047, (1.403592217853e217 * ((locals.var_beta_dn0 * assign86410_e132042) + (locals.var_beta * (-locals.var_vxbgmtcl_dn0)))), (1.403592217853e217 * ((locals.var_beta_dn2 * assign86410_e132042) + (locals.var_beta * (-locals.var_vxbgmtcl_dn2)))), (1.403592217853e217 * ((locals.var_beta_dn4 * assign86410_e132042) + (locals.var_beta * (-locals.var_vxbgmtcl_dn4)))), (1.403592217853e217 * ((locals.var_beta_dn5 * assign86410_e132042) + (locals.var_beta * (-locals.var_vxbgmtcl_dn5)))), (1.403592217853e217 * ((locals.var_beta_dn6 * assign86410_e132042) + (locals.var_beta * (-locals.var_vxbgmtcl_dn6)))), (1.403592217853e217 * ((locals.var_beta_dn7 * assign86410_e132042) + (locals.var_beta * (-locals.var_vxbgmtcl_dn7)))), (1.403592217853e217 * ((locals.var_beta_dn8 * assign86410_e132042) + (locals.var_beta * (-locals.var_vxbgmtcl_dn8)))), (1.403592217853e217 * ((locals.var_beta_dn9 * assign86410_e132042) + (locals.var_beta * (-locals.var_vxbgmtcl_dn9)))), (1.403592217853e217 * ((locals.var_beta_dn10 * assign86410_e132042) + (locals.var_beta * (-locals.var_vxbgmtcl_dn10)))), (1.403592217853e217 * ((locals.var_beta_dn13 * assign86410_e132042) + (locals.var_beta * (-locals.var_vxbgmtcl_dn13)))),)
    } else {
        (locals.var_exp_bvbs, locals.var_exp_bvbs_dn0, locals.var_exp_bvbs_dn2, locals.var_exp_bvbs_dn4, locals.var_exp_bvbs_dn5, locals.var_exp_bvbs_dn6, locals.var_exp_bvbs_dn7, locals.var_exp_bvbs_dn8, locals.var_exp_bvbs_dn9, locals.var_exp_bvbs_dn10, locals.var_exp_bvbs_dn13,)
    }
};
        locals.var_exp_bvbs = assign86410_e132049;
        locals.var_exp_bvbs_dn0 = assign86410_e132049_d_n0;
        locals.var_exp_bvbs_dn2 = assign86410_e132049_d_n2;
        locals.var_exp_bvbs_dn4 = assign86410_e132049_d_n4;
        locals.var_exp_bvbs_dn5 = assign86410_e132049_d_n5;
        locals.var_exp_bvbs_dn6 = assign86410_e132049_d_n6;
        locals.var_exp_bvbs_dn7 = assign86410_e132049_d_n7;
        locals.var_exp_bvbs_dn8 = assign86410_e132049_d_n8;
        locals.var_exp_bvbs_dn9 = assign86410_e132049_d_n9;
        locals.var_exp_bvbs_dn10 = assign86410_e132049_d_n10;
        locals.var_exp_bvbs_dn13 = assign86410_e132049_d_n13;

        let (assign86420_e132055, assign86420_e132055_d_n0, assign86420_e132055_d_n2, assign86420_e132055_d_n4, assign86420_e132055_d_n5, assign86420_e132055_d_n6, assign86420_e132055_d_n7, assign86420_e132055_d_n8, assign86420_e132055_d_n9, assign86420_e132055_d_n10, assign86420_e132055_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard2021 != 0.0)) {
        (1.403592217853e217, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign86420_e132055;
        locals.var_t0_dn0 = assign86420_e132055_d_n0;
        locals.var_t0_dn2 = assign86420_e132055_d_n2;
        locals.var_t0_dn4 = assign86420_e132055_d_n4;
        locals.var_t0_dn5 = assign86420_e132055_d_n5;
        locals.var_t0_dn6 = assign86420_e132055_d_n6;
        locals.var_t0_dn7 = assign86420_e132055_d_n7;
        locals.var_t0_dn8 = assign86420_e132055_d_n8;
        locals.var_t0_dn9 = assign86420_e132055_d_n9;
        locals.var_t0_dn10 = assign86420_e132055_d_n10;
        locals.var_t0_dn13 = assign86420_e132055_d_n13;

        let (assign86430_e132065, assign86430_e132065_d_n0, assign86430_e132065_d_n2, assign86430_e132065_d_n4, assign86430_e132065_d_n5, assign86430_e132065_d_n6, assign86430_e132065_d_n7, assign86430_e132065_d_n8, assign86430_e132065_d_n9, assign86430_e132065_d_n10, assign86430_e132065_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard2021 == 0.0)) {
        let assign86430_e132062: f64 = (-locals.var_vxbgmtcl);
        let assign86430_e132063: f64 = (locals.var_beta * assign86430_e132062);
        (assign86430_e132063, ((locals.var_beta_dn0 * assign86430_e132062) + (locals.var_beta * (-locals.var_vxbgmtcl_dn0))), ((locals.var_beta_dn2 * assign86430_e132062) + (locals.var_beta * (-locals.var_vxbgmtcl_dn2))), ((locals.var_beta_dn4 * assign86430_e132062) + (locals.var_beta * (-locals.var_vxbgmtcl_dn4))), ((locals.var_beta_dn5 * assign86430_e132062) + (locals.var_beta * (-locals.var_vxbgmtcl_dn5))), ((locals.var_beta_dn6 * assign86430_e132062) + (locals.var_beta * (-locals.var_vxbgmtcl_dn6))), ((locals.var_beta_dn7 * assign86430_e132062) + (locals.var_beta * (-locals.var_vxbgmtcl_dn7))), ((locals.var_beta_dn8 * assign86430_e132062) + (locals.var_beta * (-locals.var_vxbgmtcl_dn8))), ((locals.var_beta_dn9 * assign86430_e132062) + (locals.var_beta * (-locals.var_vxbgmtcl_dn9))), ((locals.var_beta_dn10 * assign86430_e132062) + (locals.var_beta * (-locals.var_vxbgmtcl_dn10))), ((locals.var_beta_dn13 * assign86430_e132062) + (locals.var_beta * (-locals.var_vxbgmtcl_dn13))),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13,)
    }
};
        locals.var_tmf1 = assign86430_e132065;
        locals.var_tmf1_dn0 = assign86430_e132065_d_n0;
        locals.var_tmf1_dn2 = assign86430_e132065_d_n2;
        locals.var_tmf1_dn4 = assign86430_e132065_d_n4;
        locals.var_tmf1_dn5 = assign86430_e132065_d_n5;
        locals.var_tmf1_dn6 = assign86430_e132065_d_n6;
        locals.var_tmf1_dn7 = assign86430_e132065_d_n7;
        locals.var_tmf1_dn8 = assign86430_e132065_d_n8;
        locals.var_tmf1_dn9 = assign86430_e132065_d_n9;
        locals.var_tmf1_dn10 = assign86430_e132065_d_n10;
        locals.var_tmf1_dn13 = assign86430_e132065_d_n13;

        let (assign86440_e132072, assign86440_e132072_d_n0, assign86440_e132072_d_n2, assign86440_e132072_d_n4, assign86440_e132072_d_n5, assign86440_e132072_d_n6, assign86440_e132072_d_n7, assign86440_e132072_d_n8, assign86440_e132072_d_n9, assign86440_e132072_d_n10, assign86440_e132072_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard2021 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_exp_bvbs, locals.var_exp_bvbs_dn0, locals.var_exp_bvbs_dn2, locals.var_exp_bvbs_dn4, locals.var_exp_bvbs_dn5, locals.var_exp_bvbs_dn6, locals.var_exp_bvbs_dn7, locals.var_exp_bvbs_dn8, locals.var_exp_bvbs_dn9, locals.var_exp_bvbs_dn10, locals.var_exp_bvbs_dn13,)
    }
};
        locals.var_exp_bvbs = assign86440_e132072;
        locals.var_exp_bvbs_dn0 = assign86440_e132072_d_n0;
        locals.var_exp_bvbs_dn2 = assign86440_e132072_d_n2;
        locals.var_exp_bvbs_dn4 = assign86440_e132072_d_n4;
        locals.var_exp_bvbs_dn5 = assign86440_e132072_d_n5;
        locals.var_exp_bvbs_dn6 = assign86440_e132072_d_n6;
        locals.var_exp_bvbs_dn7 = assign86440_e132072_d_n7;
        locals.var_exp_bvbs_dn8 = assign86440_e132072_d_n8;
        locals.var_exp_bvbs_dn9 = assign86440_e132072_d_n9;
        locals.var_exp_bvbs_dn10 = assign86440_e132072_d_n10;
        locals.var_exp_bvbs_dn13 = assign86440_e132072_d_n13;

        let mut assign86450_loop_guard: usize = 0;
        while {
            let assign86450_cond_e132080: f64 = if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard2021 == 0.0)) && (locals.var_tmf1 >= 60.0)) { 1.0 } else { 0.0 };
            assign86450_cond_e132080 != 0.0
        } {
            assign86450_loop_guard += 1;
            assert!(assign86450_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign86450_body0_e132089, assign86450_body0_e132089_d_n0, assign86450_body0_e132089_d_n2, assign86450_body0_e132089_d_n4, assign86450_body0_e132089_d_n5, assign86450_body0_e132089_d_n6, assign86450_body0_e132089_d_n7, assign86450_body0_e132089_d_n8, assign86450_body0_e132089_d_n9, assign86450_body0_e132089_d_n10, assign86450_body0_e132089_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard2021 == 0.0)) {
        let assign86450_body0_e132087: f64 = (locals.var_exp_bvbs * 1.14200738981568e26);
        (assign86450_body0_e132087, (locals.var_exp_bvbs_dn0 * 1.14200738981568e26), (locals.var_exp_bvbs_dn2 * 1.14200738981568e26), (locals.var_exp_bvbs_dn4 * 1.14200738981568e26), (locals.var_exp_bvbs_dn5 * 1.14200738981568e26), (locals.var_exp_bvbs_dn6 * 1.14200738981568e26), (locals.var_exp_bvbs_dn7 * 1.14200738981568e26), (locals.var_exp_bvbs_dn8 * 1.14200738981568e26), (locals.var_exp_bvbs_dn9 * 1.14200738981568e26), (locals.var_exp_bvbs_dn10 * 1.14200738981568e26), (locals.var_exp_bvbs_dn13 * 1.14200738981568e26),)
    } else {
        (locals.var_exp_bvbs, locals.var_exp_bvbs_dn0, locals.var_exp_bvbs_dn2, locals.var_exp_bvbs_dn4, locals.var_exp_bvbs_dn5, locals.var_exp_bvbs_dn6, locals.var_exp_bvbs_dn7, locals.var_exp_bvbs_dn8, locals.var_exp_bvbs_dn9, locals.var_exp_bvbs_dn10, locals.var_exp_bvbs_dn13,)
    }
};
            locals.var_exp_bvbs = assign86450_body0_e132089;
            locals.var_exp_bvbs_dn0 = assign86450_body0_e132089_d_n0;
            locals.var_exp_bvbs_dn2 = assign86450_body0_e132089_d_n2;
            locals.var_exp_bvbs_dn4 = assign86450_body0_e132089_d_n4;
            locals.var_exp_bvbs_dn5 = assign86450_body0_e132089_d_n5;
            locals.var_exp_bvbs_dn6 = assign86450_body0_e132089_d_n6;
            locals.var_exp_bvbs_dn7 = assign86450_body0_e132089_d_n7;
            locals.var_exp_bvbs_dn8 = assign86450_body0_e132089_d_n8;
            locals.var_exp_bvbs_dn9 = assign86450_body0_e132089_d_n9;
            locals.var_exp_bvbs_dn10 = assign86450_body0_e132089_d_n10;
            locals.var_exp_bvbs_dn13 = assign86450_body0_e132089_d_n13;
            let (assign86450_body1_e132098, assign86450_body1_e132098_d_n0, assign86450_body1_e132098_d_n2, assign86450_body1_e132098_d_n4, assign86450_body1_e132098_d_n5, assign86450_body1_e132098_d_n6, assign86450_body1_e132098_d_n7, assign86450_body1_e132098_d_n8, assign86450_body1_e132098_d_n9, assign86450_body1_e132098_d_n10, assign86450_body1_e132098_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard2021 == 0.0)) {
        let assign86450_body1_e132096: f64 = (locals.var_tmf1 - 60.0);
        (assign86450_body1_e132096, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13,)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13,)
    }
};
            locals.var_tmf1 = assign86450_body1_e132098;
            locals.var_tmf1_dn0 = assign86450_body1_e132098_d_n0;
            locals.var_tmf1_dn2 = assign86450_body1_e132098_d_n2;
            locals.var_tmf1_dn4 = assign86450_body1_e132098_d_n4;
            locals.var_tmf1_dn5 = assign86450_body1_e132098_d_n5;
            locals.var_tmf1_dn6 = assign86450_body1_e132098_d_n6;
            locals.var_tmf1_dn7 = assign86450_body1_e132098_d_n7;
            locals.var_tmf1_dn8 = assign86450_body1_e132098_d_n8;
            locals.var_tmf1_dn9 = assign86450_body1_e132098_d_n9;
            locals.var_tmf1_dn10 = assign86450_body1_e132098_d_n10;
            locals.var_tmf1_dn13 = assign86450_body1_e132098_d_n13;
        }

        let (assign86460_e132108, assign86460_e132108_d_n0, assign86460_e132108_d_n2, assign86460_e132108_d_n4, assign86460_e132108_d_n5, assign86460_e132108_d_n6, assign86460_e132108_d_n7, assign86460_e132108_d_n8, assign86460_e132108_d_n9, assign86460_e132108_d_n10, assign86460_e132108_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard2021 == 0.0)) {
        let assign86460_e132105: f64 = (locals.var_tmf1).exp();
        let assign86460_e132106: f64 = (locals.var_exp_bvbs * assign86460_e132105);
        (assign86460_e132106, ((locals.var_exp_bvbs_dn0 * assign86460_e132105) + (locals.var_exp_bvbs * (assign86460_e132105 * locals.var_tmf1_dn0))), ((locals.var_exp_bvbs_dn2 * assign86460_e132105) + (locals.var_exp_bvbs * (assign86460_e132105 * locals.var_tmf1_dn2))), ((locals.var_exp_bvbs_dn4 * assign86460_e132105) + (locals.var_exp_bvbs * (assign86460_e132105 * locals.var_tmf1_dn4))), ((locals.var_exp_bvbs_dn5 * assign86460_e132105) + (locals.var_exp_bvbs * (assign86460_e132105 * locals.var_tmf1_dn5))), ((locals.var_exp_bvbs_dn6 * assign86460_e132105) + (locals.var_exp_bvbs * (assign86460_e132105 * locals.var_tmf1_dn6))), ((locals.var_exp_bvbs_dn7 * assign86460_e132105) + (locals.var_exp_bvbs * (assign86460_e132105 * locals.var_tmf1_dn7))), ((locals.var_exp_bvbs_dn8 * assign86460_e132105) + (locals.var_exp_bvbs * (assign86460_e132105 * locals.var_tmf1_dn8))), ((locals.var_exp_bvbs_dn9 * assign86460_e132105) + (locals.var_exp_bvbs * (assign86460_e132105 * locals.var_tmf1_dn9))), ((locals.var_exp_bvbs_dn10 * assign86460_e132105) + (locals.var_exp_bvbs * (assign86460_e132105 * locals.var_tmf1_dn10))), ((locals.var_exp_bvbs_dn13 * assign86460_e132105) + (locals.var_exp_bvbs * (assign86460_e132105 * locals.var_tmf1_dn13))),)
    } else {
        (locals.var_exp_bvbs, locals.var_exp_bvbs_dn0, locals.var_exp_bvbs_dn2, locals.var_exp_bvbs_dn4, locals.var_exp_bvbs_dn5, locals.var_exp_bvbs_dn6, locals.var_exp_bvbs_dn7, locals.var_exp_bvbs_dn8, locals.var_exp_bvbs_dn9, locals.var_exp_bvbs_dn10, locals.var_exp_bvbs_dn13,)
    }
};
        locals.var_exp_bvbs = assign86460_e132108;
        locals.var_exp_bvbs_dn0 = assign86460_e132108_d_n0;
        locals.var_exp_bvbs_dn2 = assign86460_e132108_d_n2;
        locals.var_exp_bvbs_dn4 = assign86460_e132108_d_n4;
        locals.var_exp_bvbs_dn5 = assign86460_e132108_d_n5;
        locals.var_exp_bvbs_dn6 = assign86460_e132108_d_n6;
        locals.var_exp_bvbs_dn7 = assign86460_e132108_d_n7;
        locals.var_exp_bvbs_dn8 = assign86460_e132108_d_n8;
        locals.var_exp_bvbs_dn9 = assign86460_e132108_d_n9;
        locals.var_exp_bvbs_dn10 = assign86460_e132108_d_n10;
        locals.var_exp_bvbs_dn13 = assign86460_e132108_d_n13;

        let (assign86470_e132115, assign86470_e132115_d_n0, assign86470_e132115_d_n2, assign86470_e132115_d_n4, assign86470_e132115_d_n5, assign86470_e132115_d_n6, assign86470_e132115_d_n7, assign86470_e132115_d_n8, assign86470_e132115_d_n9, assign86470_e132115_d_n10, assign86470_e132115_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard2021 == 0.0)) {
        (locals.var_exp_bvbs, locals.var_exp_bvbs_dn0, locals.var_exp_bvbs_dn2, locals.var_exp_bvbs_dn4, locals.var_exp_bvbs_dn5, locals.var_exp_bvbs_dn6, locals.var_exp_bvbs_dn7, locals.var_exp_bvbs_dn8, locals.var_exp_bvbs_dn9, locals.var_exp_bvbs_dn10, locals.var_exp_bvbs_dn13,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign86470_e132115;
        locals.var_t0_dn0 = assign86470_e132115_d_n0;
        locals.var_t0_dn2 = assign86470_e132115_d_n2;
        locals.var_t0_dn4 = assign86470_e132115_d_n4;
        locals.var_t0_dn5 = assign86470_e132115_d_n5;
        locals.var_t0_dn6 = assign86470_e132115_d_n6;
        locals.var_t0_dn7 = assign86470_e132115_d_n7;
        locals.var_t0_dn8 = assign86470_e132115_d_n8;
        locals.var_t0_dn9 = assign86470_e132115_d_n9;
        locals.var_t0_dn10 = assign86470_e132115_d_n10;
        locals.var_t0_dn13 = assign86470_e132115_d_n13;

        let (assign86480_e132128, assign86480_e132128_d_n0, assign86480_e132128_d_n2, assign86480_e132128_d_n4, assign86480_e132128_d_n5, assign86480_e132128_d_n6, assign86480_e132128_d_n7, assign86480_e132128_d_n8, assign86480_e132128_d_n9, assign86480_e132128_d_n10, assign86480_e132128_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) {
        let assign86480_e132120: f64 = (-locals.var_vgpld);
        let assign86480_e132122: f64 = (assign86480_e132120 * 0.5);
        let assign86480_e132124: f64 = (assign86480_e132122 - 0.5);
        let assign86480_e132126: f64 = (assign86480_e132124 - 1.0);
        (assign86480_e132126, 0.0, ((-locals.var_vgpld_dn2) * 0.5), 0.0, 0.0, ((-locals.var_vgpld_dn6) * 0.5), ((-locals.var_vgpld_dn7) * 0.5), ((-locals.var_vgpld_dn8) * 0.5), 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13,)
    }
};
        locals.var_tmf1 = assign86480_e132128;
        locals.var_tmf1_dn0 = assign86480_e132128_d_n0;
        locals.var_tmf1_dn2 = assign86480_e132128_d_n2;
        locals.var_tmf1_dn4 = assign86480_e132128_d_n4;
        locals.var_tmf1_dn5 = assign86480_e132128_d_n5;
        locals.var_tmf1_dn6 = assign86480_e132128_d_n6;
        locals.var_tmf1_dn7 = assign86480_e132128_d_n7;
        locals.var_tmf1_dn8 = assign86480_e132128_d_n8;
        locals.var_tmf1_dn9 = assign86480_e132128_d_n9;
        locals.var_tmf1_dn10 = assign86480_e132128_d_n10;
        locals.var_tmf1_dn13 = assign86480_e132128_d_n13;

        let (assign86490_e132138, assign86490_e132138_d_n0, assign86490_e132138_d_n2, assign86490_e132138_d_n4, assign86490_e132138_d_n5, assign86490_e132138_d_n6, assign86490_e132138_d_n7, assign86490_e132138_d_n8, assign86490_e132138_d_n9, assign86490_e132138_d_n10, assign86490_e132138_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) {
        let assign86490_e132134: f64 = (4.0 * 0.5);
        let assign86490_e132136: f64 = assign86490_e132134;
        (assign86490_e132136, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign86490_e132138;
        locals.var_tmf2_dn0 = assign86490_e132138_d_n0;
        locals.var_tmf2_dn2 = assign86490_e132138_d_n2;
        locals.var_tmf2_dn4 = assign86490_e132138_d_n4;
        locals.var_tmf2_dn5 = assign86490_e132138_d_n5;
        locals.var_tmf2_dn6 = assign86490_e132138_d_n6;
        locals.var_tmf2_dn7 = assign86490_e132138_d_n7;
        locals.var_tmf2_dn8 = assign86490_e132138_d_n8;
        locals.var_tmf2_dn9 = assign86490_e132138_d_n9;
        locals.var_tmf2_dn10 = assign86490_e132138_d_n10;
        locals.var_tmf2_dn13 = assign86490_e132138_d_n13;

        let (assign86500_e132150, assign86500_e132150_d_n0, assign86500_e132150_d_n2, assign86500_e132150_d_n4, assign86500_e132150_d_n5, assign86500_e132150_d_n6, assign86500_e132150_d_n7, assign86500_e132150_d_n8, assign86500_e132150_d_n9, assign86500_e132150_d_n10, assign86500_e132150_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) {
        let (assign86500_e132148, assign86500_e132148_d_n0, assign86500_e132148_d_n2, assign86500_e132148_d_n4, assign86500_e132148_d_n5, assign86500_e132148_d_n6, assign86500_e132148_d_n7, assign86500_e132148_d_n8, assign86500_e132148_d_n9, assign86500_e132148_d_n10, assign86500_e132148_d_n13,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
            } else {
                let assign86500_e132147: f64 = (-locals.var_tmf2);
                (assign86500_e132147, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn13),)
            }
        };
        (assign86500_e132148, assign86500_e132148_d_n0, assign86500_e132148_d_n2, assign86500_e132148_d_n4, assign86500_e132148_d_n5, assign86500_e132148_d_n6, assign86500_e132148_d_n7, assign86500_e132148_d_n8, assign86500_e132148_d_n9, assign86500_e132148_d_n10, assign86500_e132148_d_n13,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign86500_e132150;
        locals.var_tmf2_dn0 = assign86500_e132150_d_n0;
        locals.var_tmf2_dn2 = assign86500_e132150_d_n2;
        locals.var_tmf2_dn4 = assign86500_e132150_d_n4;
        locals.var_tmf2_dn5 = assign86500_e132150_d_n5;
        locals.var_tmf2_dn6 = assign86500_e132150_d_n6;
        locals.var_tmf2_dn7 = assign86500_e132150_d_n7;
        locals.var_tmf2_dn8 = assign86500_e132150_d_n8;
        locals.var_tmf2_dn9 = assign86500_e132150_d_n9;
        locals.var_tmf2_dn10 = assign86500_e132150_d_n10;
        locals.var_tmf2_dn13 = assign86500_e132150_d_n13;

        let (assign86510_e132161, assign86510_e132161_d_n0, assign86510_e132161_d_n2, assign86510_e132161_d_n4, assign86510_e132161_d_n5, assign86510_e132161_d_n6, assign86510_e132161_d_n7, assign86510_e132161_d_n8, assign86510_e132161_d_n9, assign86510_e132161_d_n10, assign86510_e132161_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) {
        let assign86510_e132156: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign86510_e132158: f64 = (assign86510_e132156 + locals.var_tmf2);
        let assign86510_e132159: f64 = (assign86510_e132158).sqrt();
        (assign86510_e132159, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign86510_e132159)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign86510_e132159)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign86510_e132159)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign86510_e132159)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign86510_e132159)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign86510_e132159)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign86510_e132159)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign86510_e132159)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign86510_e132159)), ((((locals.var_tmf1_dn13 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn13)) + locals.var_tmf2_dn13) / (2.0 * assign86510_e132159)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign86510_e132161;
        locals.var_tmf2_dn0 = assign86510_e132161_d_n0;
        locals.var_tmf2_dn2 = assign86510_e132161_d_n2;
        locals.var_tmf2_dn4 = assign86510_e132161_d_n4;
        locals.var_tmf2_dn5 = assign86510_e132161_d_n5;
        locals.var_tmf2_dn6 = assign86510_e132161_d_n6;
        locals.var_tmf2_dn7 = assign86510_e132161_d_n7;
        locals.var_tmf2_dn8 = assign86510_e132161_d_n8;
        locals.var_tmf2_dn9 = assign86510_e132161_d_n9;
        locals.var_tmf2_dn10 = assign86510_e132161_d_n10;
        locals.var_tmf2_dn13 = assign86510_e132161_d_n13;

        let (assign86520_e132173, assign86520_e132173_d_n0, assign86520_e132173_d_n2, assign86520_e132173_d_n4, assign86520_e132173_d_n5, assign86520_e132173_d_n6, assign86520_e132173_d_n7, assign86520_e132173_d_n8, assign86520_e132173_d_n9, assign86520_e132173_d_n10, assign86520_e132173_d_n13,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) {
        let assign86520_e132169: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign86520_e132170: f64 = (1.0 + assign86520_e132169);
        let assign86520_e132171: f64 = (0.5 * assign86520_e132170);
        (assign86520_e132171, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn13 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn13)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign86520_e132173;
        locals.var_t0_dn0 = assign86520_e132173_d_n0;
        locals.var_t0_dn2 = assign86520_e132173_d_n2;
        locals.var_t0_dn4 = assign86520_e132173_d_n4;
        locals.var_t0_dn5 = assign86520_e132173_d_n5;
        locals.var_t0_dn6 = assign86520_e132173_d_n6;
        locals.var_t0_dn7 = assign86520_e132173_d_n7;
        locals.var_t0_dn8 = assign86520_e132173_d_n8;
        locals.var_t0_dn9 = assign86520_e132173_d_n9;
        locals.var_t0_dn10 = assign86520_e132173_d_n10;
        locals.var_t0_dn13 = assign86520_e132173_d_n13;

    }
}
