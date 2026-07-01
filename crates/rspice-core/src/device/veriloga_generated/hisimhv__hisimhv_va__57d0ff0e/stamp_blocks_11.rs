#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_transient_block_176(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign51770_e78332, assign51770_e78332_d_n0, assign51770_e78332_d_n2, assign51770_e78332_d_n4, assign51770_e78332_d_n5, assign51770_e78332_d_n6, assign51770_e78332_d_n7, assign51770_e78332_d_n8, assign51770_e78332_d_n9, assign51770_e78332_d_n10, assign51770_e78332_d_n11, assign51770_e78332_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1301 == 0.0)) {
        let assign51770_e78327: f64 = (locals.var_t7 * locals.var_t7);
        let assign51770_e78329: f64 = (assign51770_e78327 + locals.var_t8);
        let assign51770_e78330: f64 = (assign51770_e78329).sqrt();
        (assign51770_e78330, ((((locals.var_t7_dn0 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn0)) + locals.var_t8_dn0) / (2.0 * assign51770_e78330)), ((((locals.var_t7_dn2 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn2)) + locals.var_t8_dn2) / (2.0 * assign51770_e78330)), ((((locals.var_t7_dn4 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn4)) + locals.var_t8_dn4) / (2.0 * assign51770_e78330)), ((((locals.var_t7_dn5 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn5)) + locals.var_t8_dn5) / (2.0 * assign51770_e78330)), ((((locals.var_t7_dn6 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn6)) + locals.var_t8_dn6) / (2.0 * assign51770_e78330)), ((((locals.var_t7_dn7 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn7)) + locals.var_t8_dn7) / (2.0 * assign51770_e78330)), ((((locals.var_t7_dn8 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn8)) + locals.var_t8_dn8) / (2.0 * assign51770_e78330)), ((((locals.var_t7_dn9 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn9)) + locals.var_t8_dn9) / (2.0 * assign51770_e78330)), ((((locals.var_t7_dn10 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn10)) + locals.var_t8_dn10) / (2.0 * assign51770_e78330)), ((((locals.var_t7_dn11 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn11)) + locals.var_t8_dn11) / (2.0 * assign51770_e78330)), ((((locals.var_t7_dn14 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn14)) + locals.var_t8_dn14) / (2.0 * assign51770_e78330)),)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn11, locals.var_t9_dn14,)
    }
};
        locals.var_t9 = assign51770_e78332;
        locals.var_t9_dn0 = assign51770_e78332_d_n0;
        locals.var_t9_dn2 = assign51770_e78332_d_n2;
        locals.var_t9_dn4 = assign51770_e78332_d_n4;
        locals.var_t9_dn5 = assign51770_e78332_d_n5;
        locals.var_t9_dn6 = assign51770_e78332_d_n6;
        locals.var_t9_dn7 = assign51770_e78332_d_n7;
        locals.var_t9_dn8 = assign51770_e78332_d_n8;
        locals.var_t9_dn9 = assign51770_e78332_d_n9;
        locals.var_t9_dn10 = assign51770_e78332_d_n10;
        locals.var_t9_dn11 = assign51770_e78332_d_n11;
        locals.var_t9_dn14 = assign51770_e78332_d_n14;

        let (assign51780_e78351, assign51780_e78351_d_n0, assign51780_e78351_d_n2, assign51780_e78351_d_n4, assign51780_e78351_d_n5, assign51780_e78351_d_n6, assign51780_e78351_d_n7, assign51780_e78351_d_n8, assign51780_e78351_d_n9, assign51780_e78351_d_n10, assign51780_e78351_d_n11, assign51780_e78351_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1301 == 0.0)) {
        let assign51780_e78346: f64 = (-locals.var_t7);
        let assign51780_e78348: f64 = (assign51780_e78346 + locals.var_t9);
        let assign51780_e78349: f64 = (0.5 * assign51780_e78348);
        (assign51780_e78349, (0.5 * ((-locals.var_t7_dn0) + locals.var_t9_dn0)), (0.5 * ((-locals.var_t7_dn2) + locals.var_t9_dn2)), (0.5 * ((-locals.var_t7_dn4) + locals.var_t9_dn4)), (0.5 * ((-locals.var_t7_dn5) + locals.var_t9_dn5)), (0.5 * ((-locals.var_t7_dn6) + locals.var_t9_dn6)), (0.5 * ((-locals.var_t7_dn7) + locals.var_t9_dn7)), (0.5 * ((-locals.var_t7_dn8) + locals.var_t9_dn8)), (0.5 * ((-locals.var_t7_dn9) + locals.var_t9_dn9)), (0.5 * ((-locals.var_t7_dn10) + locals.var_t9_dn10)), (0.5 * ((-locals.var_t7_dn11) + locals.var_t9_dn11)), (0.5 * ((-locals.var_t7_dn14) + locals.var_t9_dn14)),)
    } else {
        (locals.var_lred, locals.var_lred_dn0, locals.var_lred_dn2, locals.var_lred_dn4, locals.var_lred_dn5, locals.var_lred_dn6, locals.var_lred_dn7, locals.var_lred_dn8, locals.var_lred_dn9, locals.var_lred_dn10, locals.var_lred_dn11, locals.var_lred_dn14,)
    }
};
        locals.var_lred = assign51780_e78351;
        locals.var_lred_dn0 = assign51780_e78351_d_n0;
        locals.var_lred_dn2 = assign51780_e78351_d_n2;
        locals.var_lred_dn4 = assign51780_e78351_d_n4;
        locals.var_lred_dn5 = assign51780_e78351_d_n5;
        locals.var_lred_dn6 = assign51780_e78351_d_n6;
        locals.var_lred_dn7 = assign51780_e78351_d_n7;
        locals.var_lred_dn8 = assign51780_e78351_d_n8;
        locals.var_lred_dn9 = assign51780_e78351_d_n9;
        locals.var_lred_dn10 = assign51780_e78351_d_n10;
        locals.var_lred_dn11 = assign51780_e78351_d_n11;
        locals.var_lred_dn14 = assign51780_e78351_d_n14;

        let (assign51790_e78365, assign51790_e78365_d_n0, assign51790_e78365_d_n2, assign51790_e78365_d_n4, assign51790_e78365_d_n5, assign51790_e78365_d_n6, assign51790_e78365_d_n7, assign51790_e78365_d_n8, assign51790_e78365_d_n9, assign51790_e78365_d_n10, assign51790_e78365_d_n11, assign51790_e78365_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1301 == 0.0)) {
        (locals.var_lred, locals.var_lred_dn0, locals.var_lred_dn2, locals.var_lred_dn4, locals.var_lred_dn5, locals.var_lred_dn6, locals.var_lred_dn7, locals.var_lred_dn8, locals.var_lred_dn9, locals.var_lred_dn10, locals.var_lred_dn11, locals.var_lred_dn14,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign51790_e78365;
        locals.var_t1_dn0 = assign51790_e78365_d_n0;
        locals.var_t1_dn2 = assign51790_e78365_d_n2;
        locals.var_t1_dn4 = assign51790_e78365_d_n4;
        locals.var_t1_dn5 = assign51790_e78365_d_n5;
        locals.var_t1_dn6 = assign51790_e78365_d_n6;
        locals.var_t1_dn7 = assign51790_e78365_d_n7;
        locals.var_t1_dn8 = assign51790_e78365_d_n8;
        locals.var_t1_dn9 = assign51790_e78365_d_n9;
        locals.var_t1_dn10 = assign51790_e78365_d_n10;
        locals.var_t1_dn11 = assign51790_e78365_d_n11;
        locals.var_t1_dn14 = assign51790_e78365_d_n14;

        let (assign51800_e78381, assign51800_e78381_d_n0, assign51800_e78381_d_n2, assign51800_e78381_d_n4, assign51800_e78381_d_n5, assign51800_e78381_d_n6, assign51800_e78381_d_n7, assign51800_e78381_d_n8, assign51800_e78381_d_n9, assign51800_e78381_d_n10, assign51800_e78381_d_n11, assign51800_e78381_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1301 == 0.0)) {
        let assign51800_e78379: f64 = (locals.var_fmdvds * locals.var_t1);
        (assign51800_e78379, ((locals.var_fmdvds_dn0 * locals.var_t1) + (locals.var_fmdvds * locals.var_t1_dn0)), ((locals.var_fmdvds_dn2 * locals.var_t1) + (locals.var_fmdvds * locals.var_t1_dn2)), ((locals.var_fmdvds_dn4 * locals.var_t1) + (locals.var_fmdvds * locals.var_t1_dn4)), ((locals.var_fmdvds_dn5 * locals.var_t1) + (locals.var_fmdvds * locals.var_t1_dn5)), ((locals.var_fmdvds_dn6 * locals.var_t1) + (locals.var_fmdvds * locals.var_t1_dn6)), ((locals.var_fmdvds_dn7 * locals.var_t1) + (locals.var_fmdvds * locals.var_t1_dn7)), ((locals.var_fmdvds_dn8 * locals.var_t1) + (locals.var_fmdvds * locals.var_t1_dn8)), ((locals.var_fmdvds_dn9 * locals.var_t1) + (locals.var_fmdvds * locals.var_t1_dn9)), ((locals.var_fmdvds_dn10 * locals.var_t1) + (locals.var_fmdvds * locals.var_t1_dn10)), ((locals.var_fmdvds_dn11 * locals.var_t1) + (locals.var_fmdvds * locals.var_t1_dn11)), ((locals.var_fmdvds_dn14 * locals.var_t1) + (locals.var_fmdvds * locals.var_t1_dn14)),)
    } else {
        (locals.var_lred, locals.var_lred_dn0, locals.var_lred_dn2, locals.var_lred_dn4, locals.var_lred_dn5, locals.var_lred_dn6, locals.var_lred_dn7, locals.var_lred_dn8, locals.var_lred_dn9, locals.var_lred_dn10, locals.var_lred_dn11, locals.var_lred_dn14,)
    }
};
        locals.var_lred = assign51800_e78381;
        locals.var_lred_dn0 = assign51800_e78381_d_n0;
        locals.var_lred_dn2 = assign51800_e78381_d_n2;
        locals.var_lred_dn4 = assign51800_e78381_d_n4;
        locals.var_lred_dn5 = assign51800_e78381_d_n5;
        locals.var_lred_dn6 = assign51800_e78381_d_n6;
        locals.var_lred_dn7 = assign51800_e78381_d_n7;
        locals.var_lred_dn8 = assign51800_e78381_d_n8;
        locals.var_lred_dn9 = assign51800_e78381_d_n9;
        locals.var_lred_dn10 = assign51800_e78381_d_n10;
        locals.var_lred_dn11 = assign51800_e78381_d_n11;
        locals.var_lred_dn14 = assign51800_e78381_d_n14;

        let (assign51810_e78394, assign51810_e78394_d_n0, assign51810_e78394_d_n2, assign51810_e78394_d_n4, assign51810_e78394_d_n5, assign51810_e78394_d_n6, assign51810_e78394_d_n7, assign51810_e78394_d_n8, assign51810_e78394_d_n9, assign51810_e78394_d_n10, assign51810_e78394_d_n11, assign51810_e78394_d_n14,) = {
    if ((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) {
        let assign51810_e78392: f64 = (locals.var_lred * locals.var_clmmod);
        (assign51810_e78392, (locals.var_lred_dn0 * locals.var_clmmod), (locals.var_lred_dn2 * locals.var_clmmod), (locals.var_lred_dn4 * locals.var_clmmod), (locals.var_lred_dn5 * locals.var_clmmod), (locals.var_lred_dn6 * locals.var_clmmod), (locals.var_lred_dn7 * locals.var_clmmod), (locals.var_lred_dn8 * locals.var_clmmod), (locals.var_lred_dn9 * locals.var_clmmod), (locals.var_lred_dn10 * locals.var_clmmod), (locals.var_lred_dn11 * locals.var_clmmod), (locals.var_lred_dn14 * locals.var_clmmod),)
    } else {
        (locals.var_lred, locals.var_lred_dn0, locals.var_lred_dn2, locals.var_lred_dn4, locals.var_lred_dn5, locals.var_lred_dn6, locals.var_lred_dn7, locals.var_lred_dn8, locals.var_lred_dn9, locals.var_lred_dn10, locals.var_lred_dn11, locals.var_lred_dn14,)
    }
};
        locals.var_lred = assign51810_e78394;
        locals.var_lred_dn0 = assign51810_e78394_d_n0;
        locals.var_lred_dn2 = assign51810_e78394_d_n2;
        locals.var_lred_dn4 = assign51810_e78394_d_n4;
        locals.var_lred_dn5 = assign51810_e78394_d_n5;
        locals.var_lred_dn6 = assign51810_e78394_d_n6;
        locals.var_lred_dn7 = assign51810_e78394_d_n7;
        locals.var_lred_dn8 = assign51810_e78394_d_n8;
        locals.var_lred_dn9 = assign51810_e78394_d_n9;
        locals.var_lred_dn10 = assign51810_e78394_d_n10;
        locals.var_lred_dn11 = assign51810_e78394_d_n11;
        locals.var_lred_dn14 = assign51810_e78394_d_n14;

        let (assign51820_e78407, assign51820_e78407_d_n0, assign51820_e78407_d_n2, assign51820_e78407_d_n4, assign51820_e78407_d_n5, assign51820_e78407_d_n6, assign51820_e78407_d_n7, assign51820_e78407_d_n8, assign51820_e78407_d_n9, assign51820_e78407_d_n10, assign51820_e78407_d_n11, assign51820_e78407_d_n14,) = {
    if ((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) {
        let assign51820_e78405: f64 = (locals.var_lch - locals.var_lred);
        (assign51820_e78405, (locals.var_lch_dn0 - locals.var_lred_dn0), (locals.var_lch_dn2 - locals.var_lred_dn2), (locals.var_lch_dn4 - locals.var_lred_dn4), (locals.var_lch_dn5 - locals.var_lred_dn5), (locals.var_lch_dn6 - locals.var_lred_dn6), (locals.var_lch_dn7 - locals.var_lred_dn7), (locals.var_lch_dn8 - locals.var_lred_dn8), (locals.var_lch_dn9 - locals.var_lred_dn9), (locals.var_lch_dn10 - locals.var_lred_dn10), (locals.var_lch_dn11 - locals.var_lred_dn11), (locals.var_lch_dn14 - locals.var_lred_dn14),)
    } else {
        (locals.var_lch, locals.var_lch_dn0, locals.var_lch_dn2, locals.var_lch_dn4, locals.var_lch_dn5, locals.var_lch_dn6, locals.var_lch_dn7, locals.var_lch_dn8, locals.var_lch_dn9, locals.var_lch_dn10, locals.var_lch_dn11, locals.var_lch_dn14,)
    }
};
        locals.var_lch = assign51820_e78407;
        locals.var_lch_dn0 = assign51820_e78407_d_n0;
        locals.var_lch_dn2 = assign51820_e78407_d_n2;
        locals.var_lch_dn4 = assign51820_e78407_d_n4;
        locals.var_lch_dn5 = assign51820_e78407_d_n5;
        locals.var_lch_dn6 = assign51820_e78407_d_n6;
        locals.var_lch_dn7 = assign51820_e78407_d_n7;
        locals.var_lch_dn8 = assign51820_e78407_d_n8;
        locals.var_lch_dn9 = assign51820_e78407_d_n9;
        locals.var_lch_dn10 = assign51820_e78407_d_n10;
        locals.var_lch_dn11 = assign51820_e78407_d_n11;
        locals.var_lch_dn14 = assign51820_e78407_d_n14;

        let (assign51830_e78420, assign51830_e78420_d_n0, assign51830_e78420_d_n2, assign51830_e78420_d_n4, assign51830_e78420_d_n5, assign51830_e78420_d_n6, assign51830_e78420_d_n7, assign51830_e78420_d_n8, assign51830_e78420_d_n9, assign51830_e78420_d_n10, assign51830_e78420_d_n11, assign51830_e78420_d_n14,) = {
    if ((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) {
        let assign51830_e78418: f64 = (locals.var_ninv_o_esi / 100.0);
        (assign51830_e78418, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign51830_e78420;
        locals.var_t2_dn0 = assign51830_e78420_d_n0;
        locals.var_t2_dn2 = assign51830_e78420_d_n2;
        locals.var_t2_dn4 = assign51830_e78420_d_n4;
        locals.var_t2_dn5 = assign51830_e78420_d_n5;
        locals.var_t2_dn6 = assign51830_e78420_d_n6;
        locals.var_t2_dn7 = assign51830_e78420_d_n7;
        locals.var_t2_dn8 = assign51830_e78420_d_n8;
        locals.var_t2_dn9 = assign51830_e78420_d_n9;
        locals.var_t2_dn10 = assign51830_e78420_d_n10;
        locals.var_t2_dn11 = assign51830_e78420_d_n11;
        locals.var_t2_dn14 = assign51830_e78420_d_n14;

        let (assign51840_e78431, assign51840_e78431_d_n0, assign51840_e78431_d_n2, assign51840_e78431_d_n4, assign51840_e78431_d_n5, assign51840_e78431_d_n6, assign51840_e78431_d_n7, assign51840_e78431_d_n8, assign51840_e78431_d_n9, assign51840_e78431_d_n10, assign51840_e78431_d_n11, assign51840_e78431_d_n14,) = {
    if ((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) {
        (locals.var_ninvde, locals.var_ninvde_dn0, locals.var_ninvde_dn2, locals.var_ninvde_dn4, locals.var_ninvde_dn5, locals.var_ninvde_dn6, locals.var_ninvde_dn7, locals.var_ninvde_dn8, locals.var_ninvde_dn9, locals.var_ninvde_dn10, locals.var_ninvde_dn11, locals.var_ninvde_dn14,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign51840_e78431;
        locals.var_t0_dn0 = assign51840_e78431_d_n0;
        locals.var_t0_dn2 = assign51840_e78431_d_n2;
        locals.var_t0_dn4 = assign51840_e78431_d_n4;
        locals.var_t0_dn5 = assign51840_e78431_d_n5;
        locals.var_t0_dn6 = assign51840_e78431_d_n6;
        locals.var_t0_dn7 = assign51840_e78431_d_n7;
        locals.var_t0_dn8 = assign51840_e78431_d_n8;
        locals.var_t0_dn9 = assign51840_e78431_d_n9;
        locals.var_t0_dn10 = assign51840_e78431_d_n10;
        locals.var_t0_dn11 = assign51840_e78431_d_n11;
        locals.var_t0_dn14 = assign51840_e78431_d_n14;

        let (assign51850_e78450, assign51850_e78450_d_n0, assign51850_e78450_d_n2, assign51850_e78450_d_n4, assign51850_e78450_d_n5, assign51850_e78450_d_n6, assign51850_e78450_d_n7, assign51850_e78450_d_n8, assign51850_e78450_d_n9, assign51850_e78450_d_n10, assign51850_e78450_d_n11, assign51850_e78450_d_n14,) = {
    if ((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) {
        let assign51850_e78442: f64 = (locals.var_pds * locals.var_pds);
        let assign51850_e78444: f64 = (assign51850_e78442 + p.p262);
        let assign51850_e78445: f64 = (assign51850_e78444).sqrt();
        let assign51850_e78447: f64 = (p.p262).sqrt();
        let assign51850_e78448: f64 = (assign51850_e78445 - assign51850_e78447);
        (assign51850_e78448, (((locals.var_pds_dn0 * locals.var_pds) + (locals.var_pds * locals.var_pds_dn0)) / (2.0 * assign51850_e78445)), (((locals.var_pds_dn2 * locals.var_pds) + (locals.var_pds * locals.var_pds_dn2)) / (2.0 * assign51850_e78445)), (((locals.var_pds_dn4 * locals.var_pds) + (locals.var_pds * locals.var_pds_dn4)) / (2.0 * assign51850_e78445)), (((locals.var_pds_dn5 * locals.var_pds) + (locals.var_pds * locals.var_pds_dn5)) / (2.0 * assign51850_e78445)), (((locals.var_pds_dn6 * locals.var_pds) + (locals.var_pds * locals.var_pds_dn6)) / (2.0 * assign51850_e78445)), (((locals.var_pds_dn7 * locals.var_pds) + (locals.var_pds * locals.var_pds_dn7)) / (2.0 * assign51850_e78445)), (((locals.var_pds_dn8 * locals.var_pds) + (locals.var_pds * locals.var_pds_dn8)) / (2.0 * assign51850_e78445)), (((locals.var_pds_dn9 * locals.var_pds) + (locals.var_pds * locals.var_pds_dn9)) / (2.0 * assign51850_e78445)), (((locals.var_pds_dn10 * locals.var_pds) + (locals.var_pds * locals.var_pds_dn10)) / (2.0 * assign51850_e78445)), (((locals.var_pds_dn11 * locals.var_pds) + (locals.var_pds * locals.var_pds_dn11)) / (2.0 * assign51850_e78445)), (((locals.var_pds_dn14 * locals.var_pds) + (locals.var_pds * locals.var_pds_dn14)) / (2.0 * assign51850_e78445)),)
    } else {
        (locals.var_pdsz, locals.var_pdsz_dn0, locals.var_pdsz_dn2, locals.var_pdsz_dn4, locals.var_pdsz_dn5, locals.var_pdsz_dn6, locals.var_pdsz_dn7, locals.var_pdsz_dn8, locals.var_pdsz_dn9, locals.var_pdsz_dn10, locals.var_pdsz_dn11, locals.var_pdsz_dn14,)
    }
};
        locals.var_pdsz = assign51850_e78450;
        locals.var_pdsz_dn0 = assign51850_e78450_d_n0;
        locals.var_pdsz_dn2 = assign51850_e78450_d_n2;
        locals.var_pdsz_dn4 = assign51850_e78450_d_n4;
        locals.var_pdsz_dn5 = assign51850_e78450_d_n5;
        locals.var_pdsz_dn6 = assign51850_e78450_d_n6;
        locals.var_pdsz_dn7 = assign51850_e78450_d_n7;
        locals.var_pdsz_dn8 = assign51850_e78450_d_n8;
        locals.var_pdsz_dn9 = assign51850_e78450_d_n9;
        locals.var_pdsz_dn10 = assign51850_e78450_d_n10;
        locals.var_pdsz_dn11 = assign51850_e78450_d_n11;
        locals.var_pdsz_dn14 = assign51850_e78450_d_n14;

        let (assign51860_e78465, assign51860_e78465_d_n0, assign51860_e78465_d_n2, assign51860_e78465_d_n4, assign51860_e78465_d_n5, assign51860_e78465_d_n6, assign51860_e78465_d_n7, assign51860_e78465_d_n8, assign51860_e78465_d_n9, assign51860_e78465_d_n10, assign51860_e78465_d_n11, assign51860_e78465_d_n14,) = {
    if ((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) {
        let assign51860_e78462: f64 = (locals.var_pdsz * locals.var_t0);
        let assign51860_e78463: f64 = (1.0 + assign51860_e78462);
        (assign51860_e78463, ((locals.var_pdsz_dn0 * locals.var_t0) + (locals.var_pdsz * locals.var_t0_dn0)), ((locals.var_pdsz_dn2 * locals.var_t0) + (locals.var_pdsz * locals.var_t0_dn2)), ((locals.var_pdsz_dn4 * locals.var_t0) + (locals.var_pdsz * locals.var_t0_dn4)), ((locals.var_pdsz_dn5 * locals.var_t0) + (locals.var_pdsz * locals.var_t0_dn5)), ((locals.var_pdsz_dn6 * locals.var_t0) + (locals.var_pdsz * locals.var_t0_dn6)), ((locals.var_pdsz_dn7 * locals.var_t0) + (locals.var_pdsz * locals.var_t0_dn7)), ((locals.var_pdsz_dn8 * locals.var_t0) + (locals.var_pdsz * locals.var_t0_dn8)), ((locals.var_pdsz_dn9 * locals.var_t0) + (locals.var_pdsz * locals.var_t0_dn9)), ((locals.var_pdsz_dn10 * locals.var_t0) + (locals.var_pdsz * locals.var_t0_dn10)), ((locals.var_pdsz_dn11 * locals.var_t0) + (locals.var_pdsz * locals.var_t0_dn11)), ((locals.var_pdsz_dn14 * locals.var_t0) + (locals.var_pdsz * locals.var_t0_dn14)),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign51860_e78465;
        locals.var_t4_dn0 = assign51860_e78465_d_n0;
        locals.var_t4_dn2 = assign51860_e78465_d_n2;
        locals.var_t4_dn4 = assign51860_e78465_d_n4;
        locals.var_t4_dn5 = assign51860_e78465_d_n5;
        locals.var_t4_dn6 = assign51860_e78465_d_n6;
        locals.var_t4_dn7 = assign51860_e78465_d_n7;
        locals.var_t4_dn8 = assign51860_e78465_d_n8;
        locals.var_t4_dn9 = assign51860_e78465_d_n9;
        locals.var_t4_dn10 = assign51860_e78465_d_n10;
        locals.var_t4_dn11 = assign51860_e78465_d_n11;
        locals.var_t4_dn14 = assign51860_e78465_d_n14;

        let (assign51870_e78478, assign51870_e78478_d_n0, assign51870_e78478_d_n2, assign51870_e78478_d_n4, assign51870_e78478_d_n5, assign51870_e78478_d_n6, assign51870_e78478_d_n7, assign51870_e78478_d_n8, assign51870_e78478_d_n9, assign51870_e78478_d_n10, assign51870_e78478_d_n11, assign51870_e78478_d_n14,) = {
    if ((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) {
        let assign51870_e78476: f64 = (locals.var_t2 * locals.var_qn0);
        (assign51870_e78476, ((locals.var_t2_dn0 * locals.var_qn0) + (locals.var_t2 * locals.var_qn0_dn0)), ((locals.var_t2_dn2 * locals.var_qn0) + (locals.var_t2 * locals.var_qn0_dn2)), ((locals.var_t2_dn4 * locals.var_qn0) + (locals.var_t2 * locals.var_qn0_dn4)), ((locals.var_t2_dn5 * locals.var_qn0) + (locals.var_t2 * locals.var_qn0_dn5)), ((locals.var_t2_dn6 * locals.var_qn0) + (locals.var_t2 * locals.var_qn0_dn6)), ((locals.var_t2_dn7 * locals.var_qn0) + (locals.var_t2 * locals.var_qn0_dn7)), ((locals.var_t2_dn8 * locals.var_qn0) + (locals.var_t2 * locals.var_qn0_dn8)), ((locals.var_t2_dn9 * locals.var_qn0) + (locals.var_t2 * locals.var_qn0_dn9)), ((locals.var_t2_dn10 * locals.var_qn0) + (locals.var_t2 * locals.var_qn0_dn10)), ((locals.var_t2_dn11 * locals.var_qn0) + (locals.var_t2 * locals.var_qn0_dn11)), ((locals.var_t2_dn14 * locals.var_qn0) + (locals.var_t2 * locals.var_qn0_dn14)),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign51870_e78478;
        locals.var_t5_dn0 = assign51870_e78478_d_n0;
        locals.var_t5_dn2 = assign51870_e78478_d_n2;
        locals.var_t5_dn4 = assign51870_e78478_d_n4;
        locals.var_t5_dn5 = assign51870_e78478_d_n5;
        locals.var_t5_dn6 = assign51870_e78478_d_n6;
        locals.var_t5_dn7 = assign51870_e78478_d_n7;
        locals.var_t5_dn8 = assign51870_e78478_d_n8;
        locals.var_t5_dn9 = assign51870_e78478_d_n9;
        locals.var_t5_dn10 = assign51870_e78478_d_n10;
        locals.var_t5_dn11 = assign51870_e78478_d_n11;
        locals.var_t5_dn14 = assign51870_e78478_d_n14;

        let (assign51880_e78491, assign51880_e78491_d_n0, assign51880_e78491_d_n2, assign51880_e78491_d_n4, assign51880_e78491_d_n5, assign51880_e78491_d_n6, assign51880_e78491_d_n7, assign51880_e78491_d_n8, assign51880_e78491_d_n9, assign51880_e78491_d_n10, assign51880_e78491_d_n11, assign51880_e78491_d_n14,) = {
    if ((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) {
        let assign51880_e78489: f64 = (locals.var_t5 / locals.var_t4);
        (assign51880_e78489, (((locals.var_t5_dn0 * locals.var_t4) - (locals.var_t5 * locals.var_t4_dn0)) / (locals.var_t4 * locals.var_t4)), (((locals.var_t5_dn2 * locals.var_t4) - (locals.var_t5 * locals.var_t4_dn2)) / (locals.var_t4 * locals.var_t4)), (((locals.var_t5_dn4 * locals.var_t4) - (locals.var_t5 * locals.var_t4_dn4)) / (locals.var_t4 * locals.var_t4)), (((locals.var_t5_dn5 * locals.var_t4) - (locals.var_t5 * locals.var_t4_dn5)) / (locals.var_t4 * locals.var_t4)), (((locals.var_t5_dn6 * locals.var_t4) - (locals.var_t5 * locals.var_t4_dn6)) / (locals.var_t4 * locals.var_t4)), (((locals.var_t5_dn7 * locals.var_t4) - (locals.var_t5 * locals.var_t4_dn7)) / (locals.var_t4 * locals.var_t4)), (((locals.var_t5_dn8 * locals.var_t4) - (locals.var_t5 * locals.var_t4_dn8)) / (locals.var_t4 * locals.var_t4)), (((locals.var_t5_dn9 * locals.var_t4) - (locals.var_t5 * locals.var_t4_dn9)) / (locals.var_t4 * locals.var_t4)), (((locals.var_t5_dn10 * locals.var_t4) - (locals.var_t5 * locals.var_t4_dn10)) / (locals.var_t4 * locals.var_t4)), (((locals.var_t5_dn11 * locals.var_t4) - (locals.var_t5 * locals.var_t4_dn11)) / (locals.var_t4 * locals.var_t4)), (((locals.var_t5_dn14 * locals.var_t4) - (locals.var_t5 * locals.var_t4_dn14)) / (locals.var_t4 * locals.var_t4)),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign51880_e78491;
        locals.var_t3_dn0 = assign51880_e78491_d_n0;
        locals.var_t3_dn2 = assign51880_e78491_d_n2;
        locals.var_t3_dn4 = assign51880_e78491_d_n4;
        locals.var_t3_dn5 = assign51880_e78491_d_n5;
        locals.var_t3_dn6 = assign51880_e78491_d_n6;
        locals.var_t3_dn7 = assign51880_e78491_d_n7;
        locals.var_t3_dn8 = assign51880_e78491_d_n8;
        locals.var_t3_dn9 = assign51880_e78491_d_n9;
        locals.var_t3_dn10 = assign51880_e78491_d_n10;
        locals.var_t3_dn11 = assign51880_e78491_d_n11;
        locals.var_t3_dn14 = assign51880_e78491_d_n14;

        let (assign51890_e78502, assign51890_e78502_d_n0, assign51890_e78502_d_n2, assign51890_e78502_d_n4, assign51890_e78502_d_n5, assign51890_e78502_d_n6, assign51890_e78502_d_n7, assign51890_e78502_d_n8, assign51890_e78502_d_n9, assign51890_e78502_d_n10, assign51890_e78502_d_n11, assign51890_e78502_d_n14,) = {
    if ((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    } else {
        (locals.var_eeff, locals.var_eeff_dn0, locals.var_eeff_dn2, locals.var_eeff_dn4, locals.var_eeff_dn5, locals.var_eeff_dn6, locals.var_eeff_dn7, locals.var_eeff_dn8, locals.var_eeff_dn9, locals.var_eeff_dn10, locals.var_eeff_dn11, locals.var_eeff_dn14,)
    }
};
        locals.var_eeff = assign51890_e78502;
        locals.var_eeff_dn0 = assign51890_e78502_d_n0;
        locals.var_eeff_dn2 = assign51890_e78502_d_n2;
        locals.var_eeff_dn4 = assign51890_e78502_d_n4;
        locals.var_eeff_dn5 = assign51890_e78502_d_n5;
        locals.var_eeff_dn6 = assign51890_e78502_d_n6;
        locals.var_eeff_dn7 = assign51890_e78502_d_n7;
        locals.var_eeff_dn8 = assign51890_e78502_d_n8;
        locals.var_eeff_dn9 = assign51890_e78502_d_n9;
        locals.var_eeff_dn10 = assign51890_e78502_d_n10;
        locals.var_eeff_dn11 = assign51890_e78502_d_n11;
        locals.var_eeff_dn14 = assign51890_e78502_d_n14;

        let (assign51900_e78520, assign51900_e78520_d_n0, assign51900_e78520_d_n2, assign51900_e78520_d_n4, assign51900_e78520_d_n5, assign51900_e78520_d_n6, assign51900_e78520_d_n7, assign51900_e78520_d_n8, assign51900_e78520_d_n9, assign51900_e78520_d_n10, assign51900_e78520_d_n11, assign51900_e78520_d_n14,) = {
    if ((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) {
        let (assign51900_e78518, assign51900_e78518_d_n0, assign51900_e78518_d_n2, assign51900_e78518_d_n4, assign51900_e78518_d_n5, assign51900_e78518_d_n6, assign51900_e78518_d_n7, assign51900_e78518_d_n8, assign51900_e78518_d_n9, assign51900_e78518_d_n10, assign51900_e78518_d_n11, assign51900_e78518_d_n14,) = {
            if (locals.var_eeff == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign51900_e78517: f64 = (locals.var_eeff).powf(p.p160);
                (assign51900_e78517, if 0.0 == 0.0 && ((p.p160) as f64).is_finite() && ((p.p160) as f64).fract() == 0.0 { if p.p160 == 0.0 { 0.0 } else { (p.p160 * ((locals.var_eeff).powf(p.p160 - 1.0) * locals.var_eeff_dn0)) } } else { (assign51900_e78517 * (p.p160 * (locals.var_eeff_dn0 / locals.var_eeff))) }, if 0.0 == 0.0 && ((p.p160) as f64).is_finite() && ((p.p160) as f64).fract() == 0.0 { if p.p160 == 0.0 { 0.0 } else { (p.p160 * ((locals.var_eeff).powf(p.p160 - 1.0) * locals.var_eeff_dn2)) } } else { (assign51900_e78517 * (p.p160 * (locals.var_eeff_dn2 / locals.var_eeff))) }, if 0.0 == 0.0 && ((p.p160) as f64).is_finite() && ((p.p160) as f64).fract() == 0.0 { if p.p160 == 0.0 { 0.0 } else { (p.p160 * ((locals.var_eeff).powf(p.p160 - 1.0) * locals.var_eeff_dn4)) } } else { (assign51900_e78517 * (p.p160 * (locals.var_eeff_dn4 / locals.var_eeff))) }, if 0.0 == 0.0 && ((p.p160) as f64).is_finite() && ((p.p160) as f64).fract() == 0.0 { if p.p160 == 0.0 { 0.0 } else { (p.p160 * ((locals.var_eeff).powf(p.p160 - 1.0) * locals.var_eeff_dn5)) } } else { (assign51900_e78517 * (p.p160 * (locals.var_eeff_dn5 / locals.var_eeff))) }, if 0.0 == 0.0 && ((p.p160) as f64).is_finite() && ((p.p160) as f64).fract() == 0.0 { if p.p160 == 0.0 { 0.0 } else { (p.p160 * ((locals.var_eeff).powf(p.p160 - 1.0) * locals.var_eeff_dn6)) } } else { (assign51900_e78517 * (p.p160 * (locals.var_eeff_dn6 / locals.var_eeff))) }, if 0.0 == 0.0 && ((p.p160) as f64).is_finite() && ((p.p160) as f64).fract() == 0.0 { if p.p160 == 0.0 { 0.0 } else { (p.p160 * ((locals.var_eeff).powf(p.p160 - 1.0) * locals.var_eeff_dn7)) } } else { (assign51900_e78517 * (p.p160 * (locals.var_eeff_dn7 / locals.var_eeff))) }, if 0.0 == 0.0 && ((p.p160) as f64).is_finite() && ((p.p160) as f64).fract() == 0.0 { if p.p160 == 0.0 { 0.0 } else { (p.p160 * ((locals.var_eeff).powf(p.p160 - 1.0) * locals.var_eeff_dn8)) } } else { (assign51900_e78517 * (p.p160 * (locals.var_eeff_dn8 / locals.var_eeff))) }, if 0.0 == 0.0 && ((p.p160) as f64).is_finite() && ((p.p160) as f64).fract() == 0.0 { if p.p160 == 0.0 { 0.0 } else { (p.p160 * ((locals.var_eeff).powf(p.p160 - 1.0) * locals.var_eeff_dn9)) } } else { (assign51900_e78517 * (p.p160 * (locals.var_eeff_dn9 / locals.var_eeff))) }, if 0.0 == 0.0 && ((p.p160) as f64).is_finite() && ((p.p160) as f64).fract() == 0.0 { if p.p160 == 0.0 { 0.0 } else { (p.p160 * ((locals.var_eeff).powf(p.p160 - 1.0) * locals.var_eeff_dn10)) } } else { (assign51900_e78517 * (p.p160 * (locals.var_eeff_dn10 / locals.var_eeff))) }, if 0.0 == 0.0 && ((p.p160) as f64).is_finite() && ((p.p160) as f64).fract() == 0.0 { if p.p160 == 0.0 { 0.0 } else { (p.p160 * ((locals.var_eeff).powf(p.p160 - 1.0) * locals.var_eeff_dn11)) } } else { (assign51900_e78517 * (p.p160 * (locals.var_eeff_dn11 / locals.var_eeff))) }, if 0.0 == 0.0 && ((p.p160) as f64).is_finite() && ((p.p160) as f64).fract() == 0.0 { if p.p160 == 0.0 { 0.0 } else { (p.p160 * ((locals.var_eeff).powf(p.p160 - 1.0) * locals.var_eeff_dn14)) } } else { (assign51900_e78517 * (p.p160 * (locals.var_eeff_dn14 / locals.var_eeff))) },)
            }
        };
        (assign51900_e78518, assign51900_e78518_d_n0, assign51900_e78518_d_n2, assign51900_e78518_d_n4, assign51900_e78518_d_n5, assign51900_e78518_d_n6, assign51900_e78518_d_n7, assign51900_e78518_d_n8, assign51900_e78518_d_n9, assign51900_e78518_d_n10, assign51900_e78518_d_n11, assign51900_e78518_d_n14,)
    } else {
        (locals.var_t8, locals.var_t8_dn0, locals.var_t8_dn2, locals.var_t8_dn4, locals.var_t8_dn5, locals.var_t8_dn6, locals.var_t8_dn7, locals.var_t8_dn8, locals.var_t8_dn9, locals.var_t8_dn10, locals.var_t8_dn11, locals.var_t8_dn14,)
    }
};
        locals.var_t8 = assign51900_e78520;
        locals.var_t8_dn0 = assign51900_e78520_d_n0;
        locals.var_t8_dn2 = assign51900_e78520_d_n2;
        locals.var_t8_dn4 = assign51900_e78520_d_n4;
        locals.var_t8_dn5 = assign51900_e78520_d_n5;
        locals.var_t8_dn6 = assign51900_e78520_d_n6;
        locals.var_t8_dn7 = assign51900_e78520_d_n7;
        locals.var_t8_dn8 = assign51900_e78520_d_n8;
        locals.var_t8_dn9 = assign51900_e78520_d_n9;
        locals.var_t8_dn10 = assign51900_e78520_d_n10;
        locals.var_t8_dn11 = assign51900_e78520_d_n11;
        locals.var_t8_dn14 = assign51900_e78520_d_n14;

        let (assign51910_e78538, assign51910_e78538_d_n0, assign51910_e78538_d_n2, assign51910_e78538_d_n4, assign51910_e78538_d_n5, assign51910_e78538_d_n6, assign51910_e78538_d_n7, assign51910_e78538_d_n8, assign51910_e78538_d_n9, assign51910_e78538_d_n10, assign51910_e78538_d_n11, assign51910_e78538_d_n14,) = {
    if ((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) {
        let (assign51910_e78536, assign51910_e78536_d_n0, assign51910_e78536_d_n2, assign51910_e78536_d_n4, assign51910_e78536_d_n5, assign51910_e78536_d_n6, assign51910_e78536_d_n7, assign51910_e78536_d_n8, assign51910_e78536_d_n9, assign51910_e78536_d_n10, assign51910_e78536_d_n11, assign51910_e78536_d_n14,) = {
            if (locals.var_eeff == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign51910_e78535: f64 = (locals.var_eeff).powf(locals.var_muesr);
                (assign51910_e78535, if 0.0 == 0.0 && ((locals.var_muesr) as f64).is_finite() && ((locals.var_muesr) as f64).fract() == 0.0 { if locals.var_muesr == 0.0 { 0.0 } else { (locals.var_muesr * ((locals.var_eeff).powf(locals.var_muesr - 1.0) * locals.var_eeff_dn0)) } } else { (assign51910_e78535 * (locals.var_muesr * (locals.var_eeff_dn0 / locals.var_eeff))) }, if 0.0 == 0.0 && ((locals.var_muesr) as f64).is_finite() && ((locals.var_muesr) as f64).fract() == 0.0 { if locals.var_muesr == 0.0 { 0.0 } else { (locals.var_muesr * ((locals.var_eeff).powf(locals.var_muesr - 1.0) * locals.var_eeff_dn2)) } } else { (assign51910_e78535 * (locals.var_muesr * (locals.var_eeff_dn2 / locals.var_eeff))) }, if 0.0 == 0.0 && ((locals.var_muesr) as f64).is_finite() && ((locals.var_muesr) as f64).fract() == 0.0 { if locals.var_muesr == 0.0 { 0.0 } else { (locals.var_muesr * ((locals.var_eeff).powf(locals.var_muesr - 1.0) * locals.var_eeff_dn4)) } } else { (assign51910_e78535 * (locals.var_muesr * (locals.var_eeff_dn4 / locals.var_eeff))) }, if 0.0 == 0.0 && ((locals.var_muesr) as f64).is_finite() && ((locals.var_muesr) as f64).fract() == 0.0 { if locals.var_muesr == 0.0 { 0.0 } else { (locals.var_muesr * ((locals.var_eeff).powf(locals.var_muesr - 1.0) * locals.var_eeff_dn5)) } } else { (assign51910_e78535 * (locals.var_muesr * (locals.var_eeff_dn5 / locals.var_eeff))) }, if 0.0 == 0.0 && ((locals.var_muesr) as f64).is_finite() && ((locals.var_muesr) as f64).fract() == 0.0 { if locals.var_muesr == 0.0 { 0.0 } else { (locals.var_muesr * ((locals.var_eeff).powf(locals.var_muesr - 1.0) * locals.var_eeff_dn6)) } } else { (assign51910_e78535 * (locals.var_muesr * (locals.var_eeff_dn6 / locals.var_eeff))) }, if 0.0 == 0.0 && ((locals.var_muesr) as f64).is_finite() && ((locals.var_muesr) as f64).fract() == 0.0 { if locals.var_muesr == 0.0 { 0.0 } else { (locals.var_muesr * ((locals.var_eeff).powf(locals.var_muesr - 1.0) * locals.var_eeff_dn7)) } } else { (assign51910_e78535 * (locals.var_muesr * (locals.var_eeff_dn7 / locals.var_eeff))) }, if 0.0 == 0.0 && ((locals.var_muesr) as f64).is_finite() && ((locals.var_muesr) as f64).fract() == 0.0 { if locals.var_muesr == 0.0 { 0.0 } else { (locals.var_muesr * ((locals.var_eeff).powf(locals.var_muesr - 1.0) * locals.var_eeff_dn8)) } } else { (assign51910_e78535 * (locals.var_muesr * (locals.var_eeff_dn8 / locals.var_eeff))) }, if 0.0 == 0.0 && ((locals.var_muesr) as f64).is_finite() && ((locals.var_muesr) as f64).fract() == 0.0 { if locals.var_muesr == 0.0 { 0.0 } else { (locals.var_muesr * ((locals.var_eeff).powf(locals.var_muesr - 1.0) * locals.var_eeff_dn9)) } } else { (assign51910_e78535 * (locals.var_muesr * (locals.var_eeff_dn9 / locals.var_eeff))) }, if 0.0 == 0.0 && ((locals.var_muesr) as f64).is_finite() && ((locals.var_muesr) as f64).fract() == 0.0 { if locals.var_muesr == 0.0 { 0.0 } else { (locals.var_muesr * ((locals.var_eeff).powf(locals.var_muesr - 1.0) * locals.var_eeff_dn10)) } } else { (assign51910_e78535 * (locals.var_muesr * (locals.var_eeff_dn10 / locals.var_eeff))) }, if 0.0 == 0.0 && ((locals.var_muesr) as f64).is_finite() && ((locals.var_muesr) as f64).fract() == 0.0 { if locals.var_muesr == 0.0 { 0.0 } else { (locals.var_muesr * ((locals.var_eeff).powf(locals.var_muesr - 1.0) * locals.var_eeff_dn11)) } } else { (assign51910_e78535 * (locals.var_muesr * (locals.var_eeff_dn11 / locals.var_eeff))) }, if 0.0 == 0.0 && ((locals.var_muesr) as f64).is_finite() && ((locals.var_muesr) as f64).fract() == 0.0 { if locals.var_muesr == 0.0 { 0.0 } else { (locals.var_muesr * ((locals.var_eeff).powf(locals.var_muesr - 1.0) * locals.var_eeff_dn14)) } } else { (assign51910_e78535 * (locals.var_muesr * (locals.var_eeff_dn14 / locals.var_eeff))) },)
            }
        };
        (assign51910_e78536, assign51910_e78536_d_n0, assign51910_e78536_d_n2, assign51910_e78536_d_n4, assign51910_e78536_d_n5, assign51910_e78536_d_n6, assign51910_e78536_d_n7, assign51910_e78536_d_n8, assign51910_e78536_d_n9, assign51910_e78536_d_n10, assign51910_e78536_d_n11, assign51910_e78536_d_n14,)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn14,)
    }
};
        locals.var_t6 = assign51910_e78538;
        locals.var_t6_dn0 = assign51910_e78538_d_n0;
        locals.var_t6_dn2 = assign51910_e78538_d_n2;
        locals.var_t6_dn4 = assign51910_e78538_d_n4;
        locals.var_t6_dn5 = assign51910_e78538_d_n5;
        locals.var_t6_dn6 = assign51910_e78538_d_n6;
        locals.var_t6_dn7 = assign51910_e78538_d_n7;
        locals.var_t6_dn8 = assign51910_e78538_d_n8;
        locals.var_t6_dn9 = assign51910_e78538_d_n9;
        locals.var_t6_dn10 = assign51910_e78538_d_n10;
        locals.var_t6_dn11 = assign51910_e78538_d_n11;
        locals.var_t6_dn14 = assign51910_e78538_d_n14;

        let (assign51920_e78551, assign51920_e78551_d_n0, assign51920_e78551_d_n2, assign51920_e78551_d_n4, assign51920_e78551_d_n5, assign51920_e78551_d_n6, assign51920_e78551_d_n7, assign51920_e78551_d_n8, assign51920_e78551_d_n9, assign51920_e78551_d_n10, assign51920_e78551_d_n11, assign51920_e78551_d_n14,) = {
    if ((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) {
        let assign51920_e78549: f64 = (1.6021918e-19 * 10000.0);
        (assign51920_e78549, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn11, locals.var_t9_dn14,)
    }
};
        locals.var_t9 = assign51920_e78551;
        locals.var_t9_dn0 = assign51920_e78551_d_n0;
        locals.var_t9_dn2 = assign51920_e78551_d_n2;
        locals.var_t9_dn4 = assign51920_e78551_d_n4;
        locals.var_t9_dn5 = assign51920_e78551_d_n5;
        locals.var_t9_dn6 = assign51920_e78551_d_n6;
        locals.var_t9_dn7 = assign51920_e78551_d_n7;
        locals.var_t9_dn8 = assign51920_e78551_d_n8;
        locals.var_t9_dn9 = assign51920_e78551_d_n9;
        locals.var_t9_dn10 = assign51920_e78551_d_n10;
        locals.var_t9_dn11 = assign51920_e78551_d_n11;
        locals.var_t9_dn14 = assign51920_e78551_d_n14;

        let (assign51930_e78564, assign51930_e78564_d_n0, assign51930_e78564_d_n2, assign51930_e78564_d_n4, assign51930_e78564_d_n5, assign51930_e78564_d_n6, assign51930_e78564_d_n7, assign51930_e78564_d_n8, assign51930_e78564_d_n9, assign51930_e78564_d_n10, assign51930_e78564_d_n11, assign51930_e78564_d_n14,) = {
    if ((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) {
        let assign51930_e78562: f64 = (locals.var_qn0 / locals.var_t9);
        (assign51930_e78562, (((locals.var_qn0_dn0 * locals.var_t9) - (locals.var_qn0 * locals.var_t9_dn0)) / (locals.var_t9 * locals.var_t9)), (((locals.var_qn0_dn2 * locals.var_t9) - (locals.var_qn0 * locals.var_t9_dn2)) / (locals.var_t9 * locals.var_t9)), (((locals.var_qn0_dn4 * locals.var_t9) - (locals.var_qn0 * locals.var_t9_dn4)) / (locals.var_t9 * locals.var_t9)), (((locals.var_qn0_dn5 * locals.var_t9) - (locals.var_qn0 * locals.var_t9_dn5)) / (locals.var_t9 * locals.var_t9)), (((locals.var_qn0_dn6 * locals.var_t9) - (locals.var_qn0 * locals.var_t9_dn6)) / (locals.var_t9 * locals.var_t9)), (((locals.var_qn0_dn7 * locals.var_t9) - (locals.var_qn0 * locals.var_t9_dn7)) / (locals.var_t9 * locals.var_t9)), (((locals.var_qn0_dn8 * locals.var_t9) - (locals.var_qn0 * locals.var_t9_dn8)) / (locals.var_t9 * locals.var_t9)), (((locals.var_qn0_dn9 * locals.var_t9) - (locals.var_qn0 * locals.var_t9_dn9)) / (locals.var_t9 * locals.var_t9)), (((locals.var_qn0_dn10 * locals.var_t9) - (locals.var_qn0 * locals.var_t9_dn10)) / (locals.var_t9 * locals.var_t9)), (((locals.var_qn0_dn11 * locals.var_t9) - (locals.var_qn0 * locals.var_t9_dn11)) / (locals.var_t9 * locals.var_t9)), (((locals.var_qn0_dn14 * locals.var_t9) - (locals.var_qn0 * locals.var_t9_dn14)) / (locals.var_t9 * locals.var_t9)),)
    } else {
        (locals.var_rns, locals.var_rns_dn0, locals.var_rns_dn2, locals.var_rns_dn4, locals.var_rns_dn5, locals.var_rns_dn6, locals.var_rns_dn7, locals.var_rns_dn8, locals.var_rns_dn9, locals.var_rns_dn10, locals.var_rns_dn11, locals.var_rns_dn14,)
    }
};
        locals.var_rns = assign51930_e78564;
        locals.var_rns_dn0 = assign51930_e78564_d_n0;
        locals.var_rns_dn2 = assign51930_e78564_d_n2;
        locals.var_rns_dn4 = assign51930_e78564_d_n4;
        locals.var_rns_dn5 = assign51930_e78564_d_n5;
        locals.var_rns_dn6 = assign51930_e78564_d_n6;
        locals.var_rns_dn7 = assign51930_e78564_d_n7;
        locals.var_rns_dn8 = assign51930_e78564_d_n8;
        locals.var_rns_dn9 = assign51930_e78564_d_n9;
        locals.var_rns_dn10 = assign51930_e78564_d_n10;
        locals.var_rns_dn11 = assign51930_e78564_d_n11;
        locals.var_rns_dn14 = assign51930_e78564_d_n14;

        let (assign51940_e78575, assign51940_e78575_d_n0, assign51940_e78575_d_n2, assign51940_e78575_d_n4, assign51940_e78575_d_n5, assign51940_e78575_d_n6, assign51940_e78575_d_n7, assign51940_e78575_d_n8, assign51940_e78575_d_n9, assign51940_e78575_d_n10, assign51940_e78575_d_n11, assign51940_e78575_d_n14,) = {
    if ((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) {
        (locals.var_uc_muecb0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign51940_e78575;
        locals.var_t2_dn0 = assign51940_e78575_d_n0;
        locals.var_t2_dn2 = assign51940_e78575_d_n2;
        locals.var_t2_dn4 = assign51940_e78575_d_n4;
        locals.var_t2_dn5 = assign51940_e78575_d_n5;
        locals.var_t2_dn6 = assign51940_e78575_d_n6;
        locals.var_t2_dn7 = assign51940_e78575_d_n7;
        locals.var_t2_dn8 = assign51940_e78575_d_n8;
        locals.var_t2_dn9 = assign51940_e78575_d_n9;
        locals.var_t2_dn10 = assign51940_e78575_d_n10;
        locals.var_t2_dn11 = assign51940_e78575_d_n11;
        locals.var_t2_dn14 = assign51940_e78575_d_n14;

        let (assign51950_e78610, assign51950_e78610_d_n0, assign51950_e78610_d_n2, assign51950_e78610_d_n4, assign51950_e78610_d_n5, assign51950_e78610_d_n6, assign51950_e78610_d_n7, assign51950_e78610_d_n8, assign51950_e78610_d_n9, assign51950_e78610_d_n10, assign51950_e78610_d_n11, assign51950_e78610_d_n14,) = {
    if ((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) {
        let assign51950_e78586: f64 = 1.0;
        let assign51950_e78590: f64 = (locals.var_uc_muecb1 * locals.var_t4);
        let assign51950_e78592: f64 = (assign51950_e78590 * locals.var_rns);
        let assign51950_e78594: f64 = (assign51950_e78592 / 100000000000.0);
        let assign51950_e78595: f64 = (locals.var_t2 + assign51950_e78594);
        let assign51950_e78596: f64 = (assign51950_e78586 / assign51950_e78595);
        let assign51950_e78599: f64 = locals.var_mphn0;
        let assign51950_e78601: f64 = (assign51950_e78599 * locals.var_t8);
        let assign51950_e78602: f64 = (assign51950_e78596 + assign51950_e78601);
        let assign51950_e78605: f64 = locals.var_t6;
        let assign51950_e78607: f64 = (assign51950_e78605 / locals.var_uc_muesr1);
        let assign51950_e78608: f64 = (assign51950_e78602 + assign51950_e78607);
        (assign51950_e78608, (((-((assign51950_e78586 * (locals.var_t2_dn0 + ((((locals.var_uc_muecb1 * locals.var_t4_dn0) * locals.var_rns) + (assign51950_e78590 * locals.var_rns_dn0)) / 100000000000.0))) / (assign51950_e78595 * assign51950_e78595))) + ((locals.var_mphn0_dn0 * locals.var_t8) + (assign51950_e78599 * locals.var_t8_dn0))) + (locals.var_t6_dn0 / locals.var_uc_muesr1)), (((-((assign51950_e78586 * (locals.var_t2_dn2 + ((((locals.var_uc_muecb1 * locals.var_t4_dn2) * locals.var_rns) + (assign51950_e78590 * locals.var_rns_dn2)) / 100000000000.0))) / (assign51950_e78595 * assign51950_e78595))) + ((locals.var_mphn0_dn2 * locals.var_t8) + (assign51950_e78599 * locals.var_t8_dn2))) + (locals.var_t6_dn2 / locals.var_uc_muesr1)), (((-((assign51950_e78586 * (locals.var_t2_dn4 + ((((locals.var_uc_muecb1 * locals.var_t4_dn4) * locals.var_rns) + (assign51950_e78590 * locals.var_rns_dn4)) / 100000000000.0))) / (assign51950_e78595 * assign51950_e78595))) + ((locals.var_mphn0_dn4 * locals.var_t8) + (assign51950_e78599 * locals.var_t8_dn4))) + (locals.var_t6_dn4 / locals.var_uc_muesr1)), (((-((assign51950_e78586 * (locals.var_t2_dn5 + ((((locals.var_uc_muecb1 * locals.var_t4_dn5) * locals.var_rns) + (assign51950_e78590 * locals.var_rns_dn5)) / 100000000000.0))) / (assign51950_e78595 * assign51950_e78595))) + ((locals.var_mphn0_dn5 * locals.var_t8) + (assign51950_e78599 * locals.var_t8_dn5))) + (locals.var_t6_dn5 / locals.var_uc_muesr1)), (((-((assign51950_e78586 * (locals.var_t2_dn6 + ((((locals.var_uc_muecb1 * locals.var_t4_dn6) * locals.var_rns) + (assign51950_e78590 * locals.var_rns_dn6)) / 100000000000.0))) / (assign51950_e78595 * assign51950_e78595))) + ((locals.var_mphn0_dn6 * locals.var_t8) + (assign51950_e78599 * locals.var_t8_dn6))) + (locals.var_t6_dn6 / locals.var_uc_muesr1)), (((-((assign51950_e78586 * (locals.var_t2_dn7 + ((((locals.var_uc_muecb1 * locals.var_t4_dn7) * locals.var_rns) + (assign51950_e78590 * locals.var_rns_dn7)) / 100000000000.0))) / (assign51950_e78595 * assign51950_e78595))) + ((locals.var_mphn0_dn7 * locals.var_t8) + (assign51950_e78599 * locals.var_t8_dn7))) + (locals.var_t6_dn7 / locals.var_uc_muesr1)), (((-((assign51950_e78586 * (locals.var_t2_dn8 + ((((locals.var_uc_muecb1 * locals.var_t4_dn8) * locals.var_rns) + (assign51950_e78590 * locals.var_rns_dn8)) / 100000000000.0))) / (assign51950_e78595 * assign51950_e78595))) + ((locals.var_mphn0_dn8 * locals.var_t8) + (assign51950_e78599 * locals.var_t8_dn8))) + (locals.var_t6_dn8 / locals.var_uc_muesr1)), (((-((assign51950_e78586 * (locals.var_t2_dn9 + ((((locals.var_uc_muecb1 * locals.var_t4_dn9) * locals.var_rns) + (assign51950_e78590 * locals.var_rns_dn9)) / 100000000000.0))) / (assign51950_e78595 * assign51950_e78595))) + ((locals.var_mphn0_dn9 * locals.var_t8) + (assign51950_e78599 * locals.var_t8_dn9))) + (locals.var_t6_dn9 / locals.var_uc_muesr1)), (((-((assign51950_e78586 * (locals.var_t2_dn10 + ((((locals.var_uc_muecb1 * locals.var_t4_dn10) * locals.var_rns) + (assign51950_e78590 * locals.var_rns_dn10)) / 100000000000.0))) / (assign51950_e78595 * assign51950_e78595))) + ((locals.var_mphn0_dn10 * locals.var_t8) + (assign51950_e78599 * locals.var_t8_dn10))) + (locals.var_t6_dn10 / locals.var_uc_muesr1)), (((-((assign51950_e78586 * (locals.var_t2_dn11 + ((((locals.var_uc_muecb1 * locals.var_t4_dn11) * locals.var_rns) + (assign51950_e78590 * locals.var_rns_dn11)) / 100000000000.0))) / (assign51950_e78595 * assign51950_e78595))) + ((locals.var_mphn0_dn11 * locals.var_t8) + (assign51950_e78599 * locals.var_t8_dn11))) + (locals.var_t6_dn11 / locals.var_uc_muesr1)), (((-((assign51950_e78586 * (locals.var_t2_dn14 + ((((locals.var_uc_muecb1 * locals.var_t4_dn14) * locals.var_rns) + (assign51950_e78590 * locals.var_rns_dn14)) / 100000000000.0))) / (assign51950_e78595 * assign51950_e78595))) + ((locals.var_mphn0_dn14 * locals.var_t8) + (assign51950_e78599 * locals.var_t8_dn14))) + (locals.var_t6_dn14 / locals.var_uc_muesr1)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign51950_e78610;
        locals.var_t1_dn0 = assign51950_e78610_d_n0;
        locals.var_t1_dn2 = assign51950_e78610_d_n2;
        locals.var_t1_dn4 = assign51950_e78610_d_n4;
        locals.var_t1_dn5 = assign51950_e78610_d_n5;
        locals.var_t1_dn6 = assign51950_e78610_d_n6;
        locals.var_t1_dn7 = assign51950_e78610_d_n7;
        locals.var_t1_dn8 = assign51950_e78610_d_n8;
        locals.var_t1_dn9 = assign51950_e78610_d_n9;
        locals.var_t1_dn10 = assign51950_e78610_d_n10;
        locals.var_t1_dn11 = assign51950_e78610_d_n11;
        locals.var_t1_dn14 = assign51950_e78610_d_n14;

        let (assign51960_e78623, assign51960_e78623_d_n0, assign51960_e78623_d_n2, assign51960_e78623_d_n4, assign51960_e78623_d_n5, assign51960_e78623_d_n6, assign51960_e78623_d_n7, assign51960_e78623_d_n8, assign51960_e78623_d_n9, assign51960_e78623_d_n10, assign51960_e78623_d_n11, assign51960_e78623_d_n14,) = {
    if ((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) {
        let assign51960_e78621: f64 = (1.0 / locals.var_t1);
        (assign51960_e78621, (-(locals.var_t1_dn0 / (locals.var_t1 * locals.var_t1))), (-(locals.var_t1_dn2 / (locals.var_t1 * locals.var_t1))), (-(locals.var_t1_dn4 / (locals.var_t1 * locals.var_t1))), (-(locals.var_t1_dn5 / (locals.var_t1 * locals.var_t1))), (-(locals.var_t1_dn6 / (locals.var_t1 * locals.var_t1))), (-(locals.var_t1_dn7 / (locals.var_t1 * locals.var_t1))), (-(locals.var_t1_dn8 / (locals.var_t1 * locals.var_t1))), (-(locals.var_t1_dn9 / (locals.var_t1 * locals.var_t1))), (-(locals.var_t1_dn10 / (locals.var_t1 * locals.var_t1))), (-(locals.var_t1_dn11 / (locals.var_t1 * locals.var_t1))), (-(locals.var_t1_dn14 / (locals.var_t1 * locals.var_t1))),)
    } else {
        (locals.var_muun, locals.var_muun_dn0, locals.var_muun_dn2, locals.var_muun_dn4, locals.var_muun_dn5, locals.var_muun_dn6, locals.var_muun_dn7, locals.var_muun_dn8, locals.var_muun_dn9, locals.var_muun_dn10, locals.var_muun_dn11, locals.var_muun_dn14,)
    }
};
        locals.var_muun = assign51960_e78623;
        locals.var_muun_dn0 = assign51960_e78623_d_n0;
        locals.var_muun_dn2 = assign51960_e78623_d_n2;
        locals.var_muun_dn4 = assign51960_e78623_d_n4;
        locals.var_muun_dn5 = assign51960_e78623_d_n5;
        locals.var_muun_dn6 = assign51960_e78623_d_n6;
        locals.var_muun_dn7 = assign51960_e78623_d_n7;
        locals.var_muun_dn8 = assign51960_e78623_d_n8;
        locals.var_muun_dn9 = assign51960_e78623_d_n9;
        locals.var_muun_dn10 = assign51960_e78623_d_n10;
        locals.var_muun_dn11 = assign51960_e78623_d_n11;
        locals.var_muun_dn14 = assign51960_e78623_d_n14;

        let (assign51970_e78636, assign51970_e78636_d_n0, assign51970_e78636_d_n2, assign51970_e78636_d_n4, assign51970_e78636_d_n5, assign51970_e78636_d_n6, assign51970_e78636_d_n7, assign51970_e78636_d_n8, assign51970_e78636_d_n9, assign51970_e78636_d_n10, assign51970_e78636_d_n11, assign51970_e78636_d_n14,) = {
    if ((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) {
        let assign51970_e78634: f64 = (locals.var_muun / 10000.0);
        (assign51970_e78634, (locals.var_muun_dn0 / 10000.0), (locals.var_muun_dn2 / 10000.0), (locals.var_muun_dn4 / 10000.0), (locals.var_muun_dn5 / 10000.0), (locals.var_muun_dn6 / 10000.0), (locals.var_muun_dn7 / 10000.0), (locals.var_muun_dn8 / 10000.0), (locals.var_muun_dn9 / 10000.0), (locals.var_muun_dn10 / 10000.0), (locals.var_muun_dn11 / 10000.0), (locals.var_muun_dn14 / 10000.0),)
    } else {
        (locals.var_muun, locals.var_muun_dn0, locals.var_muun_dn2, locals.var_muun_dn4, locals.var_muun_dn5, locals.var_muun_dn6, locals.var_muun_dn7, locals.var_muun_dn8, locals.var_muun_dn9, locals.var_muun_dn10, locals.var_muun_dn11, locals.var_muun_dn14,)
    }
};
        locals.var_muun = assign51970_e78636;
        locals.var_muun_dn0 = assign51970_e78636_d_n0;
        locals.var_muun_dn2 = assign51970_e78636_d_n2;
        locals.var_muun_dn4 = assign51970_e78636_d_n4;
        locals.var_muun_dn5 = assign51970_e78636_d_n5;
        locals.var_muun_dn6 = assign51970_e78636_d_n6;
        locals.var_muun_dn7 = assign51970_e78636_d_n7;
        locals.var_muun_dn8 = assign51970_e78636_d_n8;
        locals.var_muun_dn9 = assign51970_e78636_d_n9;
        locals.var_muun_dn10 = assign51970_e78636_d_n10;
        locals.var_muun_dn11 = assign51970_e78636_d_n11;
        locals.var_muun_dn14 = assign51970_e78636_d_n14;

        let (assign51980_e78653, assign51980_e78653_d_n0, assign51980_e78653_d_n2, assign51980_e78653_d_n4, assign51980_e78653_d_n5, assign51980_e78653_d_n6, assign51980_e78653_d_n7, assign51980_e78653_d_n8, assign51980_e78653_d_n9, assign51980_e78653_d_n10, assign51980_e78653_d_n11, assign51980_e78653_d_n14,) = {
    if ((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) {
        let assign51980_e78648: f64 = (locals.var_qn0 + 1e-25);
        let assign51980_e78649: f64 = (locals.var_beta * assign51980_e78648);
        let assign51980_e78651: f64 = (assign51980_e78649 * locals.var_lch);
        (assign51980_e78651, ((((locals.var_beta_dn0 * assign51980_e78648) + (locals.var_beta * locals.var_qn0_dn0)) * locals.var_lch) + (assign51980_e78649 * locals.var_lch_dn0)), ((((locals.var_beta_dn2 * assign51980_e78648) + (locals.var_beta * locals.var_qn0_dn2)) * locals.var_lch) + (assign51980_e78649 * locals.var_lch_dn2)), ((((locals.var_beta_dn4 * assign51980_e78648) + (locals.var_beta * locals.var_qn0_dn4)) * locals.var_lch) + (assign51980_e78649 * locals.var_lch_dn4)), ((((locals.var_beta_dn5 * assign51980_e78648) + (locals.var_beta * locals.var_qn0_dn5)) * locals.var_lch) + (assign51980_e78649 * locals.var_lch_dn5)), ((((locals.var_beta_dn6 * assign51980_e78648) + (locals.var_beta * locals.var_qn0_dn6)) * locals.var_lch) + (assign51980_e78649 * locals.var_lch_dn6)), ((((locals.var_beta_dn7 * assign51980_e78648) + (locals.var_beta * locals.var_qn0_dn7)) * locals.var_lch) + (assign51980_e78649 * locals.var_lch_dn7)), ((((locals.var_beta_dn8 * assign51980_e78648) + (locals.var_beta * locals.var_qn0_dn8)) * locals.var_lch) + (assign51980_e78649 * locals.var_lch_dn8)), ((((locals.var_beta_dn9 * assign51980_e78648) + (locals.var_beta * locals.var_qn0_dn9)) * locals.var_lch) + (assign51980_e78649 * locals.var_lch_dn9)), ((((locals.var_beta_dn10 * assign51980_e78648) + (locals.var_beta * locals.var_qn0_dn10)) * locals.var_lch) + (assign51980_e78649 * locals.var_lch_dn10)), ((((locals.var_beta_dn11 * assign51980_e78648) + (locals.var_beta * locals.var_qn0_dn11)) * locals.var_lch) + (assign51980_e78649 * locals.var_lch_dn11)), ((((locals.var_beta_dn14 * assign51980_e78648) + (locals.var_beta * locals.var_qn0_dn14)) * locals.var_lch) + (assign51980_e78649 * locals.var_lch_dn14)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign51980_e78653;
        locals.var_t2_dn0 = assign51980_e78653_d_n0;
        locals.var_t2_dn2 = assign51980_e78653_d_n2;
        locals.var_t2_dn4 = assign51980_e78653_d_n4;
        locals.var_t2_dn5 = assign51980_e78653_d_n5;
        locals.var_t2_dn6 = assign51980_e78653_d_n6;
        locals.var_t2_dn7 = assign51980_e78653_d_n7;
        locals.var_t2_dn8 = assign51980_e78653_d_n8;
        locals.var_t2_dn9 = assign51980_e78653_d_n9;
        locals.var_t2_dn10 = assign51980_e78653_d_n10;
        locals.var_t2_dn11 = assign51980_e78653_d_n11;
        locals.var_t2_dn14 = assign51980_e78653_d_n14;

    }

    pub(super) fn stamp_transient_block_177(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign51990_e78666, assign51990_e78666_d_n0, assign51990_e78666_d_n2, assign51990_e78666_d_n4, assign51990_e78666_d_n5, assign51990_e78666_d_n6, assign51990_e78666_d_n7, assign51990_e78666_d_n8, assign51990_e78666_d_n9, assign51990_e78666_d_n10, assign51990_e78666_d_n11, assign51990_e78666_d_n14,) = {
    if ((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) {
        let assign51990_e78664: f64 = (1.0 / locals.var_t2);
        (assign51990_e78664, (-(locals.var_t2_dn0 / (locals.var_t2 * locals.var_t2))), (-(locals.var_t2_dn2 / (locals.var_t2 * locals.var_t2))), (-(locals.var_t2_dn4 / (locals.var_t2 * locals.var_t2))), (-(locals.var_t2_dn5 / (locals.var_t2 * locals.var_t2))), (-(locals.var_t2_dn6 / (locals.var_t2 * locals.var_t2))), (-(locals.var_t2_dn7 / (locals.var_t2 * locals.var_t2))), (-(locals.var_t2_dn8 / (locals.var_t2 * locals.var_t2))), (-(locals.var_t2_dn9 / (locals.var_t2 * locals.var_t2))), (-(locals.var_t2_dn10 / (locals.var_t2 * locals.var_t2))), (-(locals.var_t2_dn11 / (locals.var_t2 * locals.var_t2))), (-(locals.var_t2_dn14 / (locals.var_t2 * locals.var_t2))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign51990_e78666;
        locals.var_t1_dn0 = assign51990_e78666_d_n0;
        locals.var_t1_dn2 = assign51990_e78666_d_n2;
        locals.var_t1_dn4 = assign51990_e78666_d_n4;
        locals.var_t1_dn5 = assign51990_e78666_d_n5;
        locals.var_t1_dn6 = assign51990_e78666_d_n6;
        locals.var_t1_dn7 = assign51990_e78666_d_n7;
        locals.var_t1_dn8 = assign51990_e78666_d_n8;
        locals.var_t1_dn9 = assign51990_e78666_d_n9;
        locals.var_t1_dn10 = assign51990_e78666_d_n10;
        locals.var_t1_dn11 = assign51990_e78666_d_n11;
        locals.var_t1_dn14 = assign51990_e78666_d_n14;

        let (assign52000_e78679, assign52000_e78679_d_n0, assign52000_e78679_d_n2, assign52000_e78679_d_n4, assign52000_e78679_d_n5, assign52000_e78679_d_n6, assign52000_e78679_d_n7, assign52000_e78679_d_n8, assign52000_e78679_d_n9, assign52000_e78679_d_n10, assign52000_e78679_d_n11, assign52000_e78679_d_n14,) = {
    if ((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) {
        let assign52000_e78677: f64 = (locals.var_idd * locals.var_t1);
        (assign52000_e78677, ((locals.var_idd_dn0 * locals.var_t1) + (locals.var_idd * locals.var_t1_dn0)), ((locals.var_idd_dn2 * locals.var_t1) + (locals.var_idd * locals.var_t1_dn2)), ((locals.var_idd_dn4 * locals.var_t1) + (locals.var_idd * locals.var_t1_dn4)), ((locals.var_idd_dn5 * locals.var_t1) + (locals.var_idd * locals.var_t1_dn5)), ((locals.var_idd_dn6 * locals.var_t1) + (locals.var_idd * locals.var_t1_dn6)), ((locals.var_idd_dn7 * locals.var_t1) + (locals.var_idd * locals.var_t1_dn7)), ((locals.var_idd_dn8 * locals.var_t1) + (locals.var_idd * locals.var_t1_dn8)), ((locals.var_idd_dn9 * locals.var_t1) + (locals.var_idd * locals.var_t1_dn9)), ((locals.var_idd_dn10 * locals.var_t1) + (locals.var_idd * locals.var_t1_dn10)), ((locals.var_idd_dn11 * locals.var_t1) + (locals.var_idd * locals.var_t1_dn11)), ((locals.var_idd_dn14 * locals.var_t1) + (locals.var_idd * locals.var_t1_dn14)),)
    } else {
        (locals.var_ty, locals.var_ty_dn0, locals.var_ty_dn2, locals.var_ty_dn4, locals.var_ty_dn5, locals.var_ty_dn6, locals.var_ty_dn7, locals.var_ty_dn8, locals.var_ty_dn9, locals.var_ty_dn10, locals.var_ty_dn11, locals.var_ty_dn14,)
    }
};
        locals.var_ty = assign52000_e78679;
        locals.var_ty_dn0 = assign52000_e78679_d_n0;
        locals.var_ty_dn2 = assign52000_e78679_d_n2;
        locals.var_ty_dn4 = assign52000_e78679_d_n4;
        locals.var_ty_dn5 = assign52000_e78679_d_n5;
        locals.var_ty_dn6 = assign52000_e78679_d_n6;
        locals.var_ty_dn7 = assign52000_e78679_d_n7;
        locals.var_ty_dn8 = assign52000_e78679_d_n8;
        locals.var_ty_dn9 = assign52000_e78679_d_n9;
        locals.var_ty_dn10 = assign52000_e78679_d_n10;
        locals.var_ty_dn11 = assign52000_e78679_d_n11;
        locals.var_ty_dn14 = assign52000_e78679_d_n14;

        let (assign52010_e78694, assign52010_e78694_d_n0, assign52010_e78694_d_n2, assign52010_e78694_d_n4, assign52010_e78694_d_n5, assign52010_e78694_d_n6, assign52010_e78694_d_n7, assign52010_e78694_d_n8, assign52010_e78694_d_n9, assign52010_e78694_d_n10, assign52010_e78694_d_n11, assign52010_e78694_d_n14,) = {
    if ((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) {
        let assign52010_e78690: f64 = (0.2 * locals.var_vmaxe);
        let assign52010_e78692: f64 = (assign52010_e78690 / locals.var_muun);
        (assign52010_e78692, ((((0.2 * locals.var_vmaxe_dn0) * locals.var_muun) - (assign52010_e78690 * locals.var_muun_dn0)) / (locals.var_muun * locals.var_muun)), ((((0.2 * locals.var_vmaxe_dn2) * locals.var_muun) - (assign52010_e78690 * locals.var_muun_dn2)) / (locals.var_muun * locals.var_muun)), ((((0.2 * locals.var_vmaxe_dn4) * locals.var_muun) - (assign52010_e78690 * locals.var_muun_dn4)) / (locals.var_muun * locals.var_muun)), ((((0.2 * locals.var_vmaxe_dn5) * locals.var_muun) - (assign52010_e78690 * locals.var_muun_dn5)) / (locals.var_muun * locals.var_muun)), ((((0.2 * locals.var_vmaxe_dn6) * locals.var_muun) - (assign52010_e78690 * locals.var_muun_dn6)) / (locals.var_muun * locals.var_muun)), ((((0.2 * locals.var_vmaxe_dn7) * locals.var_muun) - (assign52010_e78690 * locals.var_muun_dn7)) / (locals.var_muun * locals.var_muun)), ((((0.2 * locals.var_vmaxe_dn8) * locals.var_muun) - (assign52010_e78690 * locals.var_muun_dn8)) / (locals.var_muun * locals.var_muun)), ((((0.2 * locals.var_vmaxe_dn9) * locals.var_muun) - (assign52010_e78690 * locals.var_muun_dn9)) / (locals.var_muun * locals.var_muun)), ((((0.2 * locals.var_vmaxe_dn10) * locals.var_muun) - (assign52010_e78690 * locals.var_muun_dn10)) / (locals.var_muun * locals.var_muun)), ((((0.2 * locals.var_vmaxe_dn11) * locals.var_muun) - (assign52010_e78690 * locals.var_muun_dn11)) / (locals.var_muun * locals.var_muun)), ((((0.2 * locals.var_vmaxe_dn14) * locals.var_muun) - (assign52010_e78690 * locals.var_muun_dn14)) / (locals.var_muun * locals.var_muun)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign52010_e78694;
        locals.var_t2_dn0 = assign52010_e78694_d_n0;
        locals.var_t2_dn2 = assign52010_e78694_d_n2;
        locals.var_t2_dn4 = assign52010_e78694_d_n4;
        locals.var_t2_dn5 = assign52010_e78694_d_n5;
        locals.var_t2_dn6 = assign52010_e78694_d_n6;
        locals.var_t2_dn7 = assign52010_e78694_d_n7;
        locals.var_t2_dn8 = assign52010_e78694_d_n8;
        locals.var_t2_dn9 = assign52010_e78694_d_n9;
        locals.var_t2_dn10 = assign52010_e78694_d_n10;
        locals.var_t2_dn11 = assign52010_e78694_d_n11;
        locals.var_t2_dn14 = assign52010_e78694_d_n14;

        let (assign52020_e78712, assign52020_e78712_d_n0, assign52020_e78712_d_n2, assign52020_e78712_d_n4, assign52020_e78712_d_n5, assign52020_e78712_d_n6, assign52020_e78712_d_n7, assign52020_e78712_d_n8, assign52020_e78712_d_n9, assign52020_e78712_d_n10, assign52020_e78712_d_n11, assign52020_e78712_d_n14,) = {
    if ((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) {
        let assign52020_e78705: f64 = (locals.var_ty * locals.var_ty);
        let assign52020_e78708: f64 = (locals.var_t2 * locals.var_t2);
        let assign52020_e78709: f64 = (assign52020_e78705 + assign52020_e78708);
        let assign52020_e78710: f64 = (assign52020_e78709).sqrt();
        (assign52020_e78710, ((((locals.var_ty_dn0 * locals.var_ty) + (locals.var_ty * locals.var_ty_dn0)) + ((locals.var_t2_dn0 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn0))) / (2.0 * assign52020_e78710)), ((((locals.var_ty_dn2 * locals.var_ty) + (locals.var_ty * locals.var_ty_dn2)) + ((locals.var_t2_dn2 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn2))) / (2.0 * assign52020_e78710)), ((((locals.var_ty_dn4 * locals.var_ty) + (locals.var_ty * locals.var_ty_dn4)) + ((locals.var_t2_dn4 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn4))) / (2.0 * assign52020_e78710)), ((((locals.var_ty_dn5 * locals.var_ty) + (locals.var_ty * locals.var_ty_dn5)) + ((locals.var_t2_dn5 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn5))) / (2.0 * assign52020_e78710)), ((((locals.var_ty_dn6 * locals.var_ty) + (locals.var_ty * locals.var_ty_dn6)) + ((locals.var_t2_dn6 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn6))) / (2.0 * assign52020_e78710)), ((((locals.var_ty_dn7 * locals.var_ty) + (locals.var_ty * locals.var_ty_dn7)) + ((locals.var_t2_dn7 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn7))) / (2.0 * assign52020_e78710)), ((((locals.var_ty_dn8 * locals.var_ty) + (locals.var_ty * locals.var_ty_dn8)) + ((locals.var_t2_dn8 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn8))) / (2.0 * assign52020_e78710)), ((((locals.var_ty_dn9 * locals.var_ty) + (locals.var_ty * locals.var_ty_dn9)) + ((locals.var_t2_dn9 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn9))) / (2.0 * assign52020_e78710)), ((((locals.var_ty_dn10 * locals.var_ty) + (locals.var_ty * locals.var_ty_dn10)) + ((locals.var_t2_dn10 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn10))) / (2.0 * assign52020_e78710)), ((((locals.var_ty_dn11 * locals.var_ty) + (locals.var_ty * locals.var_ty_dn11)) + ((locals.var_t2_dn11 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn11))) / (2.0 * assign52020_e78710)), ((((locals.var_ty_dn14 * locals.var_ty) + (locals.var_ty * locals.var_ty_dn14)) + ((locals.var_t2_dn14 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn14))) / (2.0 * assign52020_e78710)),)
    } else {
        (locals.var_ey, locals.var_ey_dn0, locals.var_ey_dn2, locals.var_ey_dn4, locals.var_ey_dn5, locals.var_ey_dn6, locals.var_ey_dn7, locals.var_ey_dn8, locals.var_ey_dn9, locals.var_ey_dn10, locals.var_ey_dn11, locals.var_ey_dn14,)
    }
};
        locals.var_ey = assign52020_e78712;
        locals.var_ey_dn0 = assign52020_e78712_d_n0;
        locals.var_ey_dn2 = assign52020_e78712_d_n2;
        locals.var_ey_dn4 = assign52020_e78712_d_n4;
        locals.var_ey_dn5 = assign52020_e78712_d_n5;
        locals.var_ey_dn6 = assign52020_e78712_d_n6;
        locals.var_ey_dn7 = assign52020_e78712_d_n7;
        locals.var_ey_dn8 = assign52020_e78712_d_n8;
        locals.var_ey_dn9 = assign52020_e78712_d_n9;
        locals.var_ey_dn10 = assign52020_e78712_d_n10;
        locals.var_ey_dn11 = assign52020_e78712_d_n11;
        locals.var_ey_dn14 = assign52020_e78712_d_n14;

        let (assign52030_e78725, assign52030_e78725_d_n0, assign52030_e78725_d_n2, assign52030_e78725_d_n4, assign52030_e78725_d_n5, assign52030_e78725_d_n6, assign52030_e78725_d_n7, assign52030_e78725_d_n8, assign52030_e78725_d_n9, assign52030_e78725_d_n10, assign52030_e78725_d_n11, assign52030_e78725_d_n14,) = {
    if ((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) {
        let assign52030_e78723: f64 = (1.0 / locals.var_ey);
        (assign52030_e78723, (-(locals.var_ey_dn0 / (locals.var_ey * locals.var_ey))), (-(locals.var_ey_dn2 / (locals.var_ey * locals.var_ey))), (-(locals.var_ey_dn4 / (locals.var_ey * locals.var_ey))), (-(locals.var_ey_dn5 / (locals.var_ey * locals.var_ey))), (-(locals.var_ey_dn6 / (locals.var_ey * locals.var_ey))), (-(locals.var_ey_dn7 / (locals.var_ey * locals.var_ey))), (-(locals.var_ey_dn8 / (locals.var_ey * locals.var_ey))), (-(locals.var_ey_dn9 / (locals.var_ey * locals.var_ey))), (-(locals.var_ey_dn10 / (locals.var_ey * locals.var_ey))), (-(locals.var_ey_dn11 / (locals.var_ey * locals.var_ey))), (-(locals.var_ey_dn14 / (locals.var_ey * locals.var_ey))),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign52030_e78725;
        locals.var_t4_dn0 = assign52030_e78725_d_n0;
        locals.var_t4_dn2 = assign52030_e78725_d_n2;
        locals.var_t4_dn4 = assign52030_e78725_d_n4;
        locals.var_t4_dn5 = assign52030_e78725_d_n5;
        locals.var_t4_dn6 = assign52030_e78725_d_n6;
        locals.var_t4_dn7 = assign52030_e78725_d_n7;
        locals.var_t4_dn8 = assign52030_e78725_d_n8;
        locals.var_t4_dn9 = assign52030_e78725_d_n9;
        locals.var_t4_dn10 = assign52030_e78725_d_n10;
        locals.var_t4_dn11 = assign52030_e78725_d_n11;
        locals.var_t4_dn14 = assign52030_e78725_d_n14;

        let (assign52040_e78738, assign52040_e78738_d_n0, assign52040_e78738_d_n2, assign52040_e78738_d_n4, assign52040_e78738_d_n5, assign52040_e78738_d_n6, assign52040_e78738_d_n7, assign52040_e78738_d_n8, assign52040_e78738_d_n9, assign52040_e78738_d_n10, assign52040_e78738_d_n11, assign52040_e78738_d_n14,) = {
    if ((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) {
        let assign52040_e78736: f64 = (locals.var_muun * locals.var_ey);
        (assign52040_e78736, ((locals.var_muun_dn0 * locals.var_ey) + (locals.var_muun * locals.var_ey_dn0)), ((locals.var_muun_dn2 * locals.var_ey) + (locals.var_muun * locals.var_ey_dn2)), ((locals.var_muun_dn4 * locals.var_ey) + (locals.var_muun * locals.var_ey_dn4)), ((locals.var_muun_dn5 * locals.var_ey) + (locals.var_muun * locals.var_ey_dn5)), ((locals.var_muun_dn6 * locals.var_ey) + (locals.var_muun * locals.var_ey_dn6)), ((locals.var_muun_dn7 * locals.var_ey) + (locals.var_muun * locals.var_ey_dn7)), ((locals.var_muun_dn8 * locals.var_ey) + (locals.var_muun * locals.var_ey_dn8)), ((locals.var_muun_dn9 * locals.var_ey) + (locals.var_muun * locals.var_ey_dn9)), ((locals.var_muun_dn10 * locals.var_ey) + (locals.var_muun * locals.var_ey_dn10)), ((locals.var_muun_dn11 * locals.var_ey) + (locals.var_muun * locals.var_ey_dn11)), ((locals.var_muun_dn14 * locals.var_ey) + (locals.var_muun * locals.var_ey_dn14)),)
    } else {
        (locals.var_em, locals.var_em_dn0, locals.var_em_dn2, locals.var_em_dn4, locals.var_em_dn5, locals.var_em_dn6, locals.var_em_dn7, locals.var_em_dn8, locals.var_em_dn9, locals.var_em_dn10, locals.var_em_dn11, locals.var_em_dn14,)
    }
};
        locals.var_em = assign52040_e78738;
        locals.var_em_dn0 = assign52040_e78738_d_n0;
        locals.var_em_dn2 = assign52040_e78738_d_n2;
        locals.var_em_dn4 = assign52040_e78738_d_n4;
        locals.var_em_dn5 = assign52040_e78738_d_n5;
        locals.var_em_dn6 = assign52040_e78738_d_n6;
        locals.var_em_dn7 = assign52040_e78738_d_n7;
        locals.var_em_dn8 = assign52040_e78738_d_n8;
        locals.var_em_dn9 = assign52040_e78738_d_n9;
        locals.var_em_dn10 = assign52040_e78738_d_n10;
        locals.var_em_dn11 = assign52040_e78738_d_n11;
        locals.var_em_dn14 = assign52040_e78738_d_n14;

        let (assign52050_e78751, assign52050_e78751_d_n0, assign52050_e78751_d_n2, assign52050_e78751_d_n4, assign52050_e78751_d_n5, assign52050_e78751_d_n6, assign52050_e78751_d_n7, assign52050_e78751_d_n8, assign52050_e78751_d_n9, assign52050_e78751_d_n10, assign52050_e78751_d_n11, assign52050_e78751_d_n14,) = {
    if ((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) {
        let assign52050_e78749: f64 = (locals.var_em / locals.var_vmaxe);
        (assign52050_e78749, (((locals.var_em_dn0 * locals.var_vmaxe) - (locals.var_em * locals.var_vmaxe_dn0)) / (locals.var_vmaxe * locals.var_vmaxe)), (((locals.var_em_dn2 * locals.var_vmaxe) - (locals.var_em * locals.var_vmaxe_dn2)) / (locals.var_vmaxe * locals.var_vmaxe)), (((locals.var_em_dn4 * locals.var_vmaxe) - (locals.var_em * locals.var_vmaxe_dn4)) / (locals.var_vmaxe * locals.var_vmaxe)), (((locals.var_em_dn5 * locals.var_vmaxe) - (locals.var_em * locals.var_vmaxe_dn5)) / (locals.var_vmaxe * locals.var_vmaxe)), (((locals.var_em_dn6 * locals.var_vmaxe) - (locals.var_em * locals.var_vmaxe_dn6)) / (locals.var_vmaxe * locals.var_vmaxe)), (((locals.var_em_dn7 * locals.var_vmaxe) - (locals.var_em * locals.var_vmaxe_dn7)) / (locals.var_vmaxe * locals.var_vmaxe)), (((locals.var_em_dn8 * locals.var_vmaxe) - (locals.var_em * locals.var_vmaxe_dn8)) / (locals.var_vmaxe * locals.var_vmaxe)), (((locals.var_em_dn9 * locals.var_vmaxe) - (locals.var_em * locals.var_vmaxe_dn9)) / (locals.var_vmaxe * locals.var_vmaxe)), (((locals.var_em_dn10 * locals.var_vmaxe) - (locals.var_em * locals.var_vmaxe_dn10)) / (locals.var_vmaxe * locals.var_vmaxe)), (((locals.var_em_dn11 * locals.var_vmaxe) - (locals.var_em * locals.var_vmaxe_dn11)) / (locals.var_vmaxe * locals.var_vmaxe)), (((locals.var_em_dn14 * locals.var_vmaxe) - (locals.var_em * locals.var_vmaxe_dn14)) / (locals.var_vmaxe * locals.var_vmaxe)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign52050_e78751;
        locals.var_t1_dn0 = assign52050_e78751_d_n0;
        locals.var_t1_dn2 = assign52050_e78751_d_n2;
        locals.var_t1_dn4 = assign52050_e78751_d_n4;
        locals.var_t1_dn5 = assign52050_e78751_d_n5;
        locals.var_t1_dn6 = assign52050_e78751_d_n6;
        locals.var_t1_dn7 = assign52050_e78751_d_n7;
        locals.var_t1_dn8 = assign52050_e78751_d_n8;
        locals.var_t1_dn9 = assign52050_e78751_d_n9;
        locals.var_t1_dn10 = assign52050_e78751_d_n10;
        locals.var_t1_dn11 = assign52050_e78751_d_n11;
        locals.var_t1_dn14 = assign52050_e78751_d_n14;

        let assign52060_e78755: f64 = (10.0 * 2.220446049250313e-16);
        let assign52060_e78756: f64 = (1.0 - assign52060_e78755);
        let assign52060_e78763: f64 = (10.0 * 2.220446049250313e-16);
        let assign52060_e78764: f64 = (1.0 + assign52060_e78763);
        let assign52060_e78766: f64 = if ((assign52060_e78756 <= p.p178) && (p.p178 <= assign52060_e78764)) { 1.0 } else { 0.0 };
        locals.var_guard1320 = assign52060_e78766;

        let (assign52070_e78779, assign52070_e78779_d_n0, assign52070_e78779_d_n2, assign52070_e78779_d_n4, assign52070_e78779_d_n5, assign52070_e78779_d_n6, assign52070_e78779_d_n7, assign52070_e78779_d_n8, assign52070_e78779_d_n9, assign52070_e78779_d_n10, assign52070_e78779_d_n11, assign52070_e78779_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1320 != 0.0)) {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign52070_e78779;
        locals.var_t2_dn0 = assign52070_e78779_d_n0;
        locals.var_t2_dn2 = assign52070_e78779_d_n2;
        locals.var_t2_dn4 = assign52070_e78779_d_n4;
        locals.var_t2_dn5 = assign52070_e78779_d_n5;
        locals.var_t2_dn6 = assign52070_e78779_d_n6;
        locals.var_t2_dn7 = assign52070_e78779_d_n7;
        locals.var_t2_dn8 = assign52070_e78779_d_n8;
        locals.var_t2_dn9 = assign52070_e78779_d_n9;
        locals.var_t2_dn10 = assign52070_e78779_d_n10;
        locals.var_t2_dn11 = assign52070_e78779_d_n11;
        locals.var_t2_dn14 = assign52070_e78779_d_n14;

        let assign52080_e78783: f64 = (10.0 * 2.220446049250313e-16);
        let assign52080_e78784: f64 = (2.0 - assign52080_e78783);
        let assign52080_e78791: f64 = (10.0 * 2.220446049250313e-16);
        let assign52080_e78792: f64 = (2.0 + assign52080_e78791);
        let assign52080_e78794: f64 = if ((assign52080_e78784 <= p.p178) && (p.p178 <= assign52080_e78792)) { 1.0 } else { 0.0 };
        locals.var_guard1321 = assign52080_e78794;

        let (assign52090_e78812, assign52090_e78812_d_n0, assign52090_e78812_d_n2, assign52090_e78812_d_n4, assign52090_e78812_d_n5, assign52090_e78812_d_n6, assign52090_e78812_d_n7, assign52090_e78812_d_n8, assign52090_e78812_d_n9, assign52090_e78812_d_n10, assign52090_e78812_d_n11, assign52090_e78812_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1320 == 0.0)) && (locals.var_guard1321 != 0.0)) {
        let assign52090_e78810: f64 = (locals.var_t1 * locals.var_t1);
        (assign52090_e78810, ((locals.var_t1_dn0 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn0)), ((locals.var_t1_dn2 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn2)), ((locals.var_t1_dn4 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn4)), ((locals.var_t1_dn5 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn5)), ((locals.var_t1_dn6 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn6)), ((locals.var_t1_dn7 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn7)), ((locals.var_t1_dn8 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn8)), ((locals.var_t1_dn9 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn9)), ((locals.var_t1_dn10 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn10)), ((locals.var_t1_dn11 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn11)), ((locals.var_t1_dn14 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn14)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign52090_e78812;
        locals.var_t2_dn0 = assign52090_e78812_d_n0;
        locals.var_t2_dn2 = assign52090_e78812_d_n2;
        locals.var_t2_dn4 = assign52090_e78812_d_n4;
        locals.var_t2_dn5 = assign52090_e78812_d_n5;
        locals.var_t2_dn6 = assign52090_e78812_d_n6;
        locals.var_t2_dn7 = assign52090_e78812_d_n7;
        locals.var_t2_dn8 = assign52090_e78812_d_n8;
        locals.var_t2_dn9 = assign52090_e78812_d_n9;
        locals.var_t2_dn10 = assign52090_e78812_d_n10;
        locals.var_t2_dn11 = assign52090_e78812_d_n11;
        locals.var_t2_dn14 = assign52090_e78812_d_n14;

        let (assign52100_e78836, assign52100_e78836_d_n0, assign52100_e78836_d_n2, assign52100_e78836_d_n4, assign52100_e78836_d_n5, assign52100_e78836_d_n6, assign52100_e78836_d_n7, assign52100_e78836_d_n8, assign52100_e78836_d_n9, assign52100_e78836_d_n10, assign52100_e78836_d_n11, assign52100_e78836_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1320 == 0.0)) && (locals.var_guard1321 == 0.0)) {
        let (assign52100_e78834, assign52100_e78834_d_n0, assign52100_e78834_d_n2, assign52100_e78834_d_n4, assign52100_e78834_d_n5, assign52100_e78834_d_n6, assign52100_e78834_d_n7, assign52100_e78834_d_n8, assign52100_e78834_d_n9, assign52100_e78834_d_n10, assign52100_e78834_d_n11, assign52100_e78834_d_n14,) = {
            if (locals.var_t1 == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign52100_e78833: f64 = (locals.var_t1).powf(p.p178);
                (assign52100_e78833, if 0.0 == 0.0 && ((p.p178) as f64).is_finite() && ((p.p178) as f64).fract() == 0.0 { if p.p178 == 0.0 { 0.0 } else { (p.p178 * ((locals.var_t1).powf(p.p178 - 1.0) * locals.var_t1_dn0)) } } else { (assign52100_e78833 * (p.p178 * (locals.var_t1_dn0 / locals.var_t1))) }, if 0.0 == 0.0 && ((p.p178) as f64).is_finite() && ((p.p178) as f64).fract() == 0.0 { if p.p178 == 0.0 { 0.0 } else { (p.p178 * ((locals.var_t1).powf(p.p178 - 1.0) * locals.var_t1_dn2)) } } else { (assign52100_e78833 * (p.p178 * (locals.var_t1_dn2 / locals.var_t1))) }, if 0.0 == 0.0 && ((p.p178) as f64).is_finite() && ((p.p178) as f64).fract() == 0.0 { if p.p178 == 0.0 { 0.0 } else { (p.p178 * ((locals.var_t1).powf(p.p178 - 1.0) * locals.var_t1_dn4)) } } else { (assign52100_e78833 * (p.p178 * (locals.var_t1_dn4 / locals.var_t1))) }, if 0.0 == 0.0 && ((p.p178) as f64).is_finite() && ((p.p178) as f64).fract() == 0.0 { if p.p178 == 0.0 { 0.0 } else { (p.p178 * ((locals.var_t1).powf(p.p178 - 1.0) * locals.var_t1_dn5)) } } else { (assign52100_e78833 * (p.p178 * (locals.var_t1_dn5 / locals.var_t1))) }, if 0.0 == 0.0 && ((p.p178) as f64).is_finite() && ((p.p178) as f64).fract() == 0.0 { if p.p178 == 0.0 { 0.0 } else { (p.p178 * ((locals.var_t1).powf(p.p178 - 1.0) * locals.var_t1_dn6)) } } else { (assign52100_e78833 * (p.p178 * (locals.var_t1_dn6 / locals.var_t1))) }, if 0.0 == 0.0 && ((p.p178) as f64).is_finite() && ((p.p178) as f64).fract() == 0.0 { if p.p178 == 0.0 { 0.0 } else { (p.p178 * ((locals.var_t1).powf(p.p178 - 1.0) * locals.var_t1_dn7)) } } else { (assign52100_e78833 * (p.p178 * (locals.var_t1_dn7 / locals.var_t1))) }, if 0.0 == 0.0 && ((p.p178) as f64).is_finite() && ((p.p178) as f64).fract() == 0.0 { if p.p178 == 0.0 { 0.0 } else { (p.p178 * ((locals.var_t1).powf(p.p178 - 1.0) * locals.var_t1_dn8)) } } else { (assign52100_e78833 * (p.p178 * (locals.var_t1_dn8 / locals.var_t1))) }, if 0.0 == 0.0 && ((p.p178) as f64).is_finite() && ((p.p178) as f64).fract() == 0.0 { if p.p178 == 0.0 { 0.0 } else { (p.p178 * ((locals.var_t1).powf(p.p178 - 1.0) * locals.var_t1_dn9)) } } else { (assign52100_e78833 * (p.p178 * (locals.var_t1_dn9 / locals.var_t1))) }, if 0.0 == 0.0 && ((p.p178) as f64).is_finite() && ((p.p178) as f64).fract() == 0.0 { if p.p178 == 0.0 { 0.0 } else { (p.p178 * ((locals.var_t1).powf(p.p178 - 1.0) * locals.var_t1_dn10)) } } else { (assign52100_e78833 * (p.p178 * (locals.var_t1_dn10 / locals.var_t1))) }, if 0.0 == 0.0 && ((p.p178) as f64).is_finite() && ((p.p178) as f64).fract() == 0.0 { if p.p178 == 0.0 { 0.0 } else { (p.p178 * ((locals.var_t1).powf(p.p178 - 1.0) * locals.var_t1_dn11)) } } else { (assign52100_e78833 * (p.p178 * (locals.var_t1_dn11 / locals.var_t1))) }, if 0.0 == 0.0 && ((p.p178) as f64).is_finite() && ((p.p178) as f64).fract() == 0.0 { if p.p178 == 0.0 { 0.0 } else { (p.p178 * ((locals.var_t1).powf(p.p178 - 1.0) * locals.var_t1_dn14)) } } else { (assign52100_e78833 * (p.p178 * (locals.var_t1_dn14 / locals.var_t1))) },)
            }
        };
        (assign52100_e78834, assign52100_e78834_d_n0, assign52100_e78834_d_n2, assign52100_e78834_d_n4, assign52100_e78834_d_n5, assign52100_e78834_d_n6, assign52100_e78834_d_n7, assign52100_e78834_d_n8, assign52100_e78834_d_n9, assign52100_e78834_d_n10, assign52100_e78834_d_n11, assign52100_e78834_d_n14,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign52100_e78836;
        locals.var_t2_dn0 = assign52100_e78836_d_n0;
        locals.var_t2_dn2 = assign52100_e78836_d_n2;
        locals.var_t2_dn4 = assign52100_e78836_d_n4;
        locals.var_t2_dn5 = assign52100_e78836_d_n5;
        locals.var_t2_dn6 = assign52100_e78836_d_n6;
        locals.var_t2_dn7 = assign52100_e78836_d_n7;
        locals.var_t2_dn8 = assign52100_e78836_d_n8;
        locals.var_t2_dn9 = assign52100_e78836_d_n9;
        locals.var_t2_dn10 = assign52100_e78836_d_n10;
        locals.var_t2_dn11 = assign52100_e78836_d_n11;
        locals.var_t2_dn14 = assign52100_e78836_d_n14;

        let (assign52110_e78849, assign52110_e78849_d_n0, assign52110_e78849_d_n2, assign52110_e78849_d_n4, assign52110_e78849_d_n5, assign52110_e78849_d_n6, assign52110_e78849_d_n7, assign52110_e78849_d_n8, assign52110_e78849_d_n9, assign52110_e78849_d_n10, assign52110_e78849_d_n11, assign52110_e78849_d_n14,) = {
    if ((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) {
        let assign52110_e78847: f64 = (1.0 + locals.var_t2);
        (assign52110_e78847, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign52110_e78849;
        locals.var_t4_dn0 = assign52110_e78849_d_n0;
        locals.var_t4_dn2 = assign52110_e78849_d_n2;
        locals.var_t4_dn4 = assign52110_e78849_d_n4;
        locals.var_t4_dn5 = assign52110_e78849_d_n5;
        locals.var_t4_dn6 = assign52110_e78849_d_n6;
        locals.var_t4_dn7 = assign52110_e78849_d_n7;
        locals.var_t4_dn8 = assign52110_e78849_d_n8;
        locals.var_t4_dn9 = assign52110_e78849_d_n9;
        locals.var_t4_dn10 = assign52110_e78849_d_n10;
        locals.var_t4_dn11 = assign52110_e78849_d_n11;
        locals.var_t4_dn14 = assign52110_e78849_d_n14;

        let assign52120_e78853: f64 = (10.0 * 2.220446049250313e-16);
        let assign52120_e78854: f64 = (1.0 - assign52120_e78853);
        let assign52120_e78861: f64 = (10.0 * 2.220446049250313e-16);
        let assign52120_e78862: f64 = (1.0 + assign52120_e78861);
        let assign52120_e78864: f64 = if ((assign52120_e78854 <= p.p178) && (p.p178 <= assign52120_e78862)) { 1.0 } else { 0.0 };
        locals.var_guard1322 = assign52120_e78864;

        let (assign52130_e78879, assign52130_e78879_d_n0, assign52130_e78879_d_n2, assign52130_e78879_d_n4, assign52130_e78879_d_n5, assign52130_e78879_d_n6, assign52130_e78879_d_n7, assign52130_e78879_d_n8, assign52130_e78879_d_n9, assign52130_e78879_d_n10, assign52130_e78879_d_n11, assign52130_e78879_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1322 != 0.0)) {
        let assign52130_e78877: f64 = (1.0 / locals.var_t4);
        (assign52130_e78877, (-(locals.var_t4_dn0 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn2 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn4 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn5 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn6 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn7 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn8 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn9 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn10 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn11 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn14 / (locals.var_t4 * locals.var_t4))),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign52130_e78879;
        locals.var_t5_dn0 = assign52130_e78879_d_n0;
        locals.var_t5_dn2 = assign52130_e78879_d_n2;
        locals.var_t5_dn4 = assign52130_e78879_d_n4;
        locals.var_t5_dn5 = assign52130_e78879_d_n5;
        locals.var_t5_dn6 = assign52130_e78879_d_n6;
        locals.var_t5_dn7 = assign52130_e78879_d_n7;
        locals.var_t5_dn8 = assign52130_e78879_d_n8;
        locals.var_t5_dn9 = assign52130_e78879_d_n9;
        locals.var_t5_dn10 = assign52130_e78879_d_n10;
        locals.var_t5_dn11 = assign52130_e78879_d_n11;
        locals.var_t5_dn14 = assign52130_e78879_d_n14;

        let assign52140_e78883: f64 = (10.0 * 2.220446049250313e-16);
        let assign52140_e78884: f64 = (2.0 - assign52140_e78883);
        let assign52140_e78891: f64 = (10.0 * 2.220446049250313e-16);
        let assign52140_e78892: f64 = (2.0 + assign52140_e78891);
        let assign52140_e78894: f64 = if ((assign52140_e78884 <= p.p178) && (p.p178 <= assign52140_e78892)) { 1.0 } else { 0.0 };
        locals.var_guard1323 = assign52140_e78894;

        let (assign52150_e78913, assign52150_e78913_d_n0, assign52150_e78913_d_n2, assign52150_e78913_d_n4, assign52150_e78913_d_n5, assign52150_e78913_d_n6, assign52150_e78913_d_n7, assign52150_e78913_d_n8, assign52150_e78913_d_n9, assign52150_e78913_d_n10, assign52150_e78913_d_n11, assign52150_e78913_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1322 == 0.0)) && (locals.var_guard1323 != 0.0)) {
        let assign52150_e78910: f64 = (locals.var_t4).sqrt();
        let assign52150_e78911: f64 = (1.0 / assign52150_e78910);
        (assign52150_e78911, (-((locals.var_t4_dn0 / (2.0 * assign52150_e78910)) / (assign52150_e78910 * assign52150_e78910))), (-((locals.var_t4_dn2 / (2.0 * assign52150_e78910)) / (assign52150_e78910 * assign52150_e78910))), (-((locals.var_t4_dn4 / (2.0 * assign52150_e78910)) / (assign52150_e78910 * assign52150_e78910))), (-((locals.var_t4_dn5 / (2.0 * assign52150_e78910)) / (assign52150_e78910 * assign52150_e78910))), (-((locals.var_t4_dn6 / (2.0 * assign52150_e78910)) / (assign52150_e78910 * assign52150_e78910))), (-((locals.var_t4_dn7 / (2.0 * assign52150_e78910)) / (assign52150_e78910 * assign52150_e78910))), (-((locals.var_t4_dn8 / (2.0 * assign52150_e78910)) / (assign52150_e78910 * assign52150_e78910))), (-((locals.var_t4_dn9 / (2.0 * assign52150_e78910)) / (assign52150_e78910 * assign52150_e78910))), (-((locals.var_t4_dn10 / (2.0 * assign52150_e78910)) / (assign52150_e78910 * assign52150_e78910))), (-((locals.var_t4_dn11 / (2.0 * assign52150_e78910)) / (assign52150_e78910 * assign52150_e78910))), (-((locals.var_t4_dn14 / (2.0 * assign52150_e78910)) / (assign52150_e78910 * assign52150_e78910))),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign52150_e78913;
        locals.var_t5_dn0 = assign52150_e78913_d_n0;
        locals.var_t5_dn2 = assign52150_e78913_d_n2;
        locals.var_t5_dn4 = assign52150_e78913_d_n4;
        locals.var_t5_dn5 = assign52150_e78913_d_n5;
        locals.var_t5_dn6 = assign52150_e78913_d_n6;
        locals.var_t5_dn7 = assign52150_e78913_d_n7;
        locals.var_t5_dn8 = assign52150_e78913_d_n8;
        locals.var_t5_dn9 = assign52150_e78913_d_n9;
        locals.var_t5_dn10 = assign52150_e78913_d_n10;
        locals.var_t5_dn11 = assign52150_e78913_d_n11;
        locals.var_t5_dn14 = assign52150_e78913_d_n14;

        let (assign52160_e78940, assign52160_e78940_d_n0, assign52160_e78940_d_n2, assign52160_e78940_d_n4, assign52160_e78940_d_n5, assign52160_e78940_d_n6, assign52160_e78940_d_n7, assign52160_e78940_d_n8, assign52160_e78940_d_n9, assign52160_e78940_d_n10, assign52160_e78940_d_n11, assign52160_e78940_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1322 == 0.0)) && (locals.var_guard1323 == 0.0)) {
        let (assign52160_e78938, assign52160_e78938_d_n0, assign52160_e78938_d_n2, assign52160_e78938_d_n4, assign52160_e78938_d_n5, assign52160_e78938_d_n6, assign52160_e78938_d_n7, assign52160_e78938_d_n8, assign52160_e78938_d_n9, assign52160_e78938_d_n10, assign52160_e78938_d_n11, assign52160_e78938_d_n14,) = {
            if (locals.var_t4 == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign52160_e78934: f64 = (-1.0);
                let assign52160_e78936: f64 = (assign52160_e78934 / p.p178);
                let assign52160_e78937: f64 = (locals.var_t4).powf(assign52160_e78936);
                (assign52160_e78937, if 0.0 == 0.0 && ((assign52160_e78936) as f64).is_finite() && ((assign52160_e78936) as f64).fract() == 0.0 { if assign52160_e78936 == 0.0 { 0.0 } else { (assign52160_e78936 * ((locals.var_t4).powf(assign52160_e78936 - 1.0) * locals.var_t4_dn0)) } } else { (assign52160_e78937 * (assign52160_e78936 * (locals.var_t4_dn0 / locals.var_t4))) }, if 0.0 == 0.0 && ((assign52160_e78936) as f64).is_finite() && ((assign52160_e78936) as f64).fract() == 0.0 { if assign52160_e78936 == 0.0 { 0.0 } else { (assign52160_e78936 * ((locals.var_t4).powf(assign52160_e78936 - 1.0) * locals.var_t4_dn2)) } } else { (assign52160_e78937 * (assign52160_e78936 * (locals.var_t4_dn2 / locals.var_t4))) }, if 0.0 == 0.0 && ((assign52160_e78936) as f64).is_finite() && ((assign52160_e78936) as f64).fract() == 0.0 { if assign52160_e78936 == 0.0 { 0.0 } else { (assign52160_e78936 * ((locals.var_t4).powf(assign52160_e78936 - 1.0) * locals.var_t4_dn4)) } } else { (assign52160_e78937 * (assign52160_e78936 * (locals.var_t4_dn4 / locals.var_t4))) }, if 0.0 == 0.0 && ((assign52160_e78936) as f64).is_finite() && ((assign52160_e78936) as f64).fract() == 0.0 { if assign52160_e78936 == 0.0 { 0.0 } else { (assign52160_e78936 * ((locals.var_t4).powf(assign52160_e78936 - 1.0) * locals.var_t4_dn5)) } } else { (assign52160_e78937 * (assign52160_e78936 * (locals.var_t4_dn5 / locals.var_t4))) }, if 0.0 == 0.0 && ((assign52160_e78936) as f64).is_finite() && ((assign52160_e78936) as f64).fract() == 0.0 { if assign52160_e78936 == 0.0 { 0.0 } else { (assign52160_e78936 * ((locals.var_t4).powf(assign52160_e78936 - 1.0) * locals.var_t4_dn6)) } } else { (assign52160_e78937 * (assign52160_e78936 * (locals.var_t4_dn6 / locals.var_t4))) }, if 0.0 == 0.0 && ((assign52160_e78936) as f64).is_finite() && ((assign52160_e78936) as f64).fract() == 0.0 { if assign52160_e78936 == 0.0 { 0.0 } else { (assign52160_e78936 * ((locals.var_t4).powf(assign52160_e78936 - 1.0) * locals.var_t4_dn7)) } } else { (assign52160_e78937 * (assign52160_e78936 * (locals.var_t4_dn7 / locals.var_t4))) }, if 0.0 == 0.0 && ((assign52160_e78936) as f64).is_finite() && ((assign52160_e78936) as f64).fract() == 0.0 { if assign52160_e78936 == 0.0 { 0.0 } else { (assign52160_e78936 * ((locals.var_t4).powf(assign52160_e78936 - 1.0) * locals.var_t4_dn8)) } } else { (assign52160_e78937 * (assign52160_e78936 * (locals.var_t4_dn8 / locals.var_t4))) }, if 0.0 == 0.0 && ((assign52160_e78936) as f64).is_finite() && ((assign52160_e78936) as f64).fract() == 0.0 { if assign52160_e78936 == 0.0 { 0.0 } else { (assign52160_e78936 * ((locals.var_t4).powf(assign52160_e78936 - 1.0) * locals.var_t4_dn9)) } } else { (assign52160_e78937 * (assign52160_e78936 * (locals.var_t4_dn9 / locals.var_t4))) }, if 0.0 == 0.0 && ((assign52160_e78936) as f64).is_finite() && ((assign52160_e78936) as f64).fract() == 0.0 { if assign52160_e78936 == 0.0 { 0.0 } else { (assign52160_e78936 * ((locals.var_t4).powf(assign52160_e78936 - 1.0) * locals.var_t4_dn10)) } } else { (assign52160_e78937 * (assign52160_e78936 * (locals.var_t4_dn10 / locals.var_t4))) }, if 0.0 == 0.0 && ((assign52160_e78936) as f64).is_finite() && ((assign52160_e78936) as f64).fract() == 0.0 { if assign52160_e78936 == 0.0 { 0.0 } else { (assign52160_e78936 * ((locals.var_t4).powf(assign52160_e78936 - 1.0) * locals.var_t4_dn11)) } } else { (assign52160_e78937 * (assign52160_e78936 * (locals.var_t4_dn11 / locals.var_t4))) }, if 0.0 == 0.0 && ((assign52160_e78936) as f64).is_finite() && ((assign52160_e78936) as f64).fract() == 0.0 { if assign52160_e78936 == 0.0 { 0.0 } else { (assign52160_e78936 * ((locals.var_t4).powf(assign52160_e78936 - 1.0) * locals.var_t4_dn14)) } } else { (assign52160_e78937 * (assign52160_e78936 * (locals.var_t4_dn14 / locals.var_t4))) },)
            }
        };
        (assign52160_e78938, assign52160_e78938_d_n0, assign52160_e78938_d_n2, assign52160_e78938_d_n4, assign52160_e78938_d_n5, assign52160_e78938_d_n6, assign52160_e78938_d_n7, assign52160_e78938_d_n8, assign52160_e78938_d_n9, assign52160_e78938_d_n10, assign52160_e78938_d_n11, assign52160_e78938_d_n14,)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign52160_e78940;
        locals.var_t5_dn0 = assign52160_e78940_d_n0;
        locals.var_t5_dn2 = assign52160_e78940_d_n2;
        locals.var_t5_dn4 = assign52160_e78940_d_n4;
        locals.var_t5_dn5 = assign52160_e78940_d_n5;
        locals.var_t5_dn6 = assign52160_e78940_d_n6;
        locals.var_t5_dn7 = assign52160_e78940_d_n7;
        locals.var_t5_dn8 = assign52160_e78940_d_n8;
        locals.var_t5_dn9 = assign52160_e78940_d_n9;
        locals.var_t5_dn10 = assign52160_e78940_d_n10;
        locals.var_t5_dn11 = assign52160_e78940_d_n11;
        locals.var_t5_dn14 = assign52160_e78940_d_n14;

        let (assign52170_e78953, assign52170_e78953_d_n0, assign52170_e78953_d_n2, assign52170_e78953_d_n4, assign52170_e78953_d_n5, assign52170_e78953_d_n6, assign52170_e78953_d_n7, assign52170_e78953_d_n8, assign52170_e78953_d_n9, assign52170_e78953_d_n10, assign52170_e78953_d_n11, assign52170_e78953_d_n14,) = {
    if ((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) {
        let assign52170_e78951: f64 = (locals.var_muun * locals.var_t5);
        (assign52170_e78951, ((locals.var_muun_dn0 * locals.var_t5) + (locals.var_muun * locals.var_t5_dn0)), ((locals.var_muun_dn2 * locals.var_t5) + (locals.var_muun * locals.var_t5_dn2)), ((locals.var_muun_dn4 * locals.var_t5) + (locals.var_muun * locals.var_t5_dn4)), ((locals.var_muun_dn5 * locals.var_t5) + (locals.var_muun * locals.var_t5_dn5)), ((locals.var_muun_dn6 * locals.var_t5) + (locals.var_muun * locals.var_t5_dn6)), ((locals.var_muun_dn7 * locals.var_t5) + (locals.var_muun * locals.var_t5_dn7)), ((locals.var_muun_dn8 * locals.var_t5) + (locals.var_muun * locals.var_t5_dn8)), ((locals.var_muun_dn9 * locals.var_t5) + (locals.var_muun * locals.var_t5_dn9)), ((locals.var_muun_dn10 * locals.var_t5) + (locals.var_muun * locals.var_t5_dn10)), ((locals.var_muun_dn11 * locals.var_t5) + (locals.var_muun * locals.var_t5_dn11)), ((locals.var_muun_dn14 * locals.var_t5) + (locals.var_muun * locals.var_t5_dn14)),)
    } else {
        (locals.var_mu, locals.var_mu_dn0, locals.var_mu_dn2, locals.var_mu_dn4, locals.var_mu_dn5, locals.var_mu_dn6, locals.var_mu_dn7, locals.var_mu_dn8, locals.var_mu_dn9, locals.var_mu_dn10, locals.var_mu_dn11, locals.var_mu_dn14,)
    }
};
        locals.var_mu = assign52170_e78953;
        locals.var_mu_dn0 = assign52170_e78953_d_n0;
        locals.var_mu_dn2 = assign52170_e78953_d_n2;
        locals.var_mu_dn4 = assign52170_e78953_d_n4;
        locals.var_mu_dn5 = assign52170_e78953_d_n5;
        locals.var_mu_dn6 = assign52170_e78953_d_n6;
        locals.var_mu_dn7 = assign52170_e78953_d_n7;
        locals.var_mu_dn8 = assign52170_e78953_d_n8;
        locals.var_mu_dn9 = assign52170_e78953_d_n9;
        locals.var_mu_dn10 = assign52170_e78953_d_n10;
        locals.var_mu_dn11 = assign52170_e78953_d_n11;
        locals.var_mu_dn14 = assign52170_e78953_d_n14;

        let (assign52180_e78964, assign52180_e78964_d_n0, assign52180_e78964_d_n2, assign52180_e78964_d_n4, assign52180_e78964_d_n5, assign52180_e78964_d_n6, assign52180_e78964_d_n7, assign52180_e78964_d_n8, assign52180_e78964_d_n9, assign52180_e78964_d_n10, assign52180_e78964_d_n11, assign52180_e78964_d_n14,) = {
    if ((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) {
        (locals.var_mu, locals.var_mu_dn0, locals.var_mu_dn2, locals.var_mu_dn4, locals.var_mu_dn5, locals.var_mu_dn6, locals.var_mu_dn7, locals.var_mu_dn8, locals.var_mu_dn9, locals.var_mu_dn10, locals.var_mu_dn11, locals.var_mu_dn14,)
    } else {
        (locals.var_mu_acc, locals.var_mu_acc_dn0, locals.var_mu_acc_dn2, locals.var_mu_acc_dn4, locals.var_mu_acc_dn5, locals.var_mu_acc_dn6, locals.var_mu_acc_dn7, locals.var_mu_acc_dn8, locals.var_mu_acc_dn9, locals.var_mu_acc_dn10, locals.var_mu_acc_dn11, locals.var_mu_acc_dn14,)
    }
};
        locals.var_mu_acc = assign52180_e78964;
        locals.var_mu_acc_dn0 = assign52180_e78964_d_n0;
        locals.var_mu_acc_dn2 = assign52180_e78964_d_n2;
        locals.var_mu_acc_dn4 = assign52180_e78964_d_n4;
        locals.var_mu_acc_dn5 = assign52180_e78964_d_n5;
        locals.var_mu_acc_dn6 = assign52180_e78964_d_n6;
        locals.var_mu_acc_dn7 = assign52180_e78964_d_n7;
        locals.var_mu_acc_dn8 = assign52180_e78964_d_n8;
        locals.var_mu_acc_dn9 = assign52180_e78964_d_n9;
        locals.var_mu_acc_dn10 = assign52180_e78964_d_n10;
        locals.var_mu_acc_dn11 = assign52180_e78964_d_n11;
        locals.var_mu_acc_dn14 = assign52180_e78964_d_n14;

        let (assign52190_e78975, assign52190_e78975_d_n0, assign52190_e78975_d_n2, assign52190_e78975_d_n4, assign52190_e78975_d_n5, assign52190_e78975_d_n6, assign52190_e78975_d_n7, assign52190_e78975_d_n8, assign52190_e78975_d_n9, assign52190_e78975_d_n10, assign52190_e78975_d_n11, assign52190_e78975_d_n14,) = {
    if ((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) {
        (locals.var_ey, locals.var_ey_dn0, locals.var_ey_dn2, locals.var_ey_dn4, locals.var_ey_dn5, locals.var_ey_dn6, locals.var_ey_dn7, locals.var_ey_dn8, locals.var_ey_dn9, locals.var_ey_dn10, locals.var_ey_dn11, locals.var_ey_dn14,)
    } else {
        (locals.var_ey_acc__blk1120, locals.var_ey_acc__blk1120_dn0, locals.var_ey_acc__blk1120_dn2, locals.var_ey_acc__blk1120_dn4, locals.var_ey_acc__blk1120_dn5, locals.var_ey_acc__blk1120_dn6, locals.var_ey_acc__blk1120_dn7, locals.var_ey_acc__blk1120_dn8, locals.var_ey_acc__blk1120_dn9, locals.var_ey_acc__blk1120_dn10, locals.var_ey_acc__blk1120_dn11, locals.var_ey_acc__blk1120_dn14,)
    }
};
        locals.var_ey_acc__blk1120 = assign52190_e78975;
        locals.var_ey_acc__blk1120_dn0 = assign52190_e78975_d_n0;
        locals.var_ey_acc__blk1120_dn2 = assign52190_e78975_d_n2;
        locals.var_ey_acc__blk1120_dn4 = assign52190_e78975_d_n4;
        locals.var_ey_acc__blk1120_dn5 = assign52190_e78975_d_n5;
        locals.var_ey_acc__blk1120_dn6 = assign52190_e78975_d_n6;
        locals.var_ey_acc__blk1120_dn7 = assign52190_e78975_d_n7;
        locals.var_ey_acc__blk1120_dn8 = assign52190_e78975_d_n8;
        locals.var_ey_acc__blk1120_dn9 = assign52190_e78975_d_n9;
        locals.var_ey_acc__blk1120_dn10 = assign52190_e78975_d_n10;
        locals.var_ey_acc__blk1120_dn11 = assign52190_e78975_d_n11;
        locals.var_ey_acc__blk1120_dn14 = assign52190_e78975_d_n14;

        let (assign52200_e78986, assign52200_e78986_d_n0, assign52200_e78986_d_n2, assign52200_e78986_d_n4, assign52200_e78986_d_n5, assign52200_e78986_d_n6, assign52200_e78986_d_n7, assign52200_e78986_d_n8, assign52200_e78986_d_n9, assign52200_e78986_d_n10, assign52200_e78986_d_n11, assign52200_e78986_d_n14,) = {
    if ((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vgp_ws, locals.var_vgp_ws_dn0, locals.var_vgp_ws_dn2, locals.var_vgp_ws_dn4, locals.var_vgp_ws_dn5, locals.var_vgp_ws_dn6, locals.var_vgp_ws_dn7, locals.var_vgp_ws_dn8, locals.var_vgp_ws_dn9, locals.var_vgp_ws_dn10, locals.var_vgp_ws_dn11, locals.var_vgp_ws_dn14,)
    }
};
        locals.var_vgp_ws = assign52200_e78986;
        locals.var_vgp_ws_dn0 = assign52200_e78986_d_n0;
        locals.var_vgp_ws_dn2 = assign52200_e78986_d_n2;
        locals.var_vgp_ws_dn4 = assign52200_e78986_d_n4;
        locals.var_vgp_ws_dn5 = assign52200_e78986_d_n5;
        locals.var_vgp_ws_dn6 = assign52200_e78986_d_n6;
        locals.var_vgp_ws_dn7 = assign52200_e78986_d_n7;
        locals.var_vgp_ws_dn8 = assign52200_e78986_d_n8;
        locals.var_vgp_ws_dn9 = assign52200_e78986_d_n9;
        locals.var_vgp_ws_dn10 = assign52200_e78986_d_n10;
        locals.var_vgp_ws_dn11 = assign52200_e78986_d_n11;
        locals.var_vgp_ws_dn14 = assign52200_e78986_d_n14;

        let (assign52210_e78997, assign52210_e78997_d_n0, assign52210_e78997_d_n2, assign52210_e78997_d_n4, assign52210_e78997_d_n5, assign52210_e78997_d_n6, assign52210_e78997_d_n7, assign52210_e78997_d_n8, assign52210_e78997_d_n9, assign52210_e78997_d_n10, assign52210_e78997_d_n11, assign52210_e78997_d_n14,) = {
    if ((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_w_res_leak, locals.var_w_res_leak_dn0, locals.var_w_res_leak_dn2, locals.var_w_res_leak_dn4, locals.var_w_res_leak_dn5, locals.var_w_res_leak_dn6, locals.var_w_res_leak_dn7, locals.var_w_res_leak_dn8, locals.var_w_res_leak_dn9, locals.var_w_res_leak_dn10, locals.var_w_res_leak_dn11, locals.var_w_res_leak_dn14,)
    }
};
        locals.var_w_res_leak = assign52210_e78997;
        locals.var_w_res_leak_dn0 = assign52210_e78997_d_n0;
        locals.var_w_res_leak_dn2 = assign52210_e78997_d_n2;
        locals.var_w_res_leak_dn4 = assign52210_e78997_d_n4;
        locals.var_w_res_leak_dn5 = assign52210_e78997_d_n5;
        locals.var_w_res_leak_dn6 = assign52210_e78997_d_n6;
        locals.var_w_res_leak_dn7 = assign52210_e78997_d_n7;
        locals.var_w_res_leak_dn8 = assign52210_e78997_d_n8;
        locals.var_w_res_leak_dn9 = assign52210_e78997_d_n9;
        locals.var_w_res_leak_dn10 = assign52210_e78997_d_n10;
        locals.var_w_res_leak_dn11 = assign52210_e78997_d_n11;
        locals.var_w_res_leak_dn14 = assign52210_e78997_d_n14;

        let (assign52220_e79008, assign52220_e79008_d_n0, assign52220_e79008_d_n2, assign52220_e79008_d_n4, assign52220_e79008_d_n5, assign52220_e79008_d_n6, assign52220_e79008_d_n7, assign52220_e79008_d_n8, assign52220_e79008_d_n9, assign52220_e79008_d_n10, assign52220_e79008_d_n11, assign52220_e79008_d_n14,) = {
    if ((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_w_res, locals.var_w_res_dn0, locals.var_w_res_dn2, locals.var_w_res_dn4, locals.var_w_res_dn5, locals.var_w_res_dn6, locals.var_w_res_dn7, locals.var_w_res_dn8, locals.var_w_res_dn9, locals.var_w_res_dn10, locals.var_w_res_dn11, locals.var_w_res_dn14,)
    }
};
        locals.var_w_res = assign52220_e79008;
        locals.var_w_res_dn0 = assign52220_e79008_d_n0;
        locals.var_w_res_dn2 = assign52220_e79008_d_n2;
        locals.var_w_res_dn4 = assign52220_e79008_d_n4;
        locals.var_w_res_dn5 = assign52220_e79008_d_n5;
        locals.var_w_res_dn6 = assign52220_e79008_d_n6;
        locals.var_w_res_dn7 = assign52220_e79008_d_n7;
        locals.var_w_res_dn8 = assign52220_e79008_d_n8;
        locals.var_w_res_dn9 = assign52220_e79008_d_n9;
        locals.var_w_res_dn10 = assign52220_e79008_d_n10;
        locals.var_w_res_dn11 = assign52220_e79008_d_n11;
        locals.var_w_res_dn14 = assign52220_e79008_d_n14;

        let (assign52230_e79019, assign52230_e79019_d_n0, assign52230_e79019_d_n2, assign52230_e79019_d_n4, assign52230_e79019_d_n5, assign52230_e79019_d_n6, assign52230_e79019_d_n7, assign52230_e79019_d_n8, assign52230_e79019_d_n9, assign52230_e79019_d_n10, assign52230_e79019_d_n11, assign52230_e79019_d_n14,) = {
    if ((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ws__blk1151, locals.var_ws__blk1151_dn0, locals.var_ws__blk1151_dn2, locals.var_ws__blk1151_dn4, locals.var_ws__blk1151_dn5, locals.var_ws__blk1151_dn6, locals.var_ws__blk1151_dn7, locals.var_ws__blk1151_dn8, locals.var_ws__blk1151_dn9, locals.var_ws__blk1151_dn10, locals.var_ws__blk1151_dn11, locals.var_ws__blk1151_dn14,)
    }
};
        locals.var_ws__blk1151 = assign52230_e79019;
        locals.var_ws__blk1151_dn0 = assign52230_e79019_d_n0;
        locals.var_ws__blk1151_dn2 = assign52230_e79019_d_n2;
        locals.var_ws__blk1151_dn4 = assign52230_e79019_d_n4;
        locals.var_ws__blk1151_dn5 = assign52230_e79019_d_n5;
        locals.var_ws__blk1151_dn6 = assign52230_e79019_d_n6;
        locals.var_ws__blk1151_dn7 = assign52230_e79019_d_n7;
        locals.var_ws__blk1151_dn8 = assign52230_e79019_d_n8;
        locals.var_ws__blk1151_dn9 = assign52230_e79019_d_n9;
        locals.var_ws__blk1151_dn10 = assign52230_e79019_d_n10;
        locals.var_ws__blk1151_dn11 = assign52230_e79019_d_n11;
        locals.var_ws__blk1151_dn14 = assign52230_e79019_d_n14;

        let (assign52240_e79030, assign52240_e79030_d_n0, assign52240_e79030_d_n2, assign52240_e79030_d_n4, assign52240_e79030_d_n5, assign52240_e79030_d_n6, assign52240_e79030_d_n7, assign52240_e79030_d_n8, assign52240_e79030_d_n9, assign52240_e79030_d_n10, assign52240_e79030_d_n11, assign52240_e79030_d_n14,) = {
    if ((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_q_s0__blk1326, locals.var_q_s0__blk1326_dn0, locals.var_q_s0__blk1326_dn2, locals.var_q_s0__blk1326_dn4, locals.var_q_s0__blk1326_dn5, locals.var_q_s0__blk1326_dn6, locals.var_q_s0__blk1326_dn7, locals.var_q_s0__blk1326_dn8, locals.var_q_s0__blk1326_dn9, locals.var_q_s0__blk1326_dn10, locals.var_q_s0__blk1326_dn11, locals.var_q_s0__blk1326_dn14,)
    }
};
        locals.var_q_s0__blk1326 = assign52240_e79030;
        locals.var_q_s0__blk1326_dn0 = assign52240_e79030_d_n0;
        locals.var_q_s0__blk1326_dn2 = assign52240_e79030_d_n2;
        locals.var_q_s0__blk1326_dn4 = assign52240_e79030_d_n4;
        locals.var_q_s0__blk1326_dn5 = assign52240_e79030_d_n5;
        locals.var_q_s0__blk1326_dn6 = assign52240_e79030_d_n6;
        locals.var_q_s0__blk1326_dn7 = assign52240_e79030_d_n7;
        locals.var_q_s0__blk1326_dn8 = assign52240_e79030_d_n8;
        locals.var_q_s0__blk1326_dn9 = assign52240_e79030_d_n9;
        locals.var_q_s0__blk1326_dn10 = assign52240_e79030_d_n10;
        locals.var_q_s0__blk1326_dn11 = assign52240_e79030_d_n11;
        locals.var_q_s0__blk1326_dn14 = assign52240_e79030_d_n14;

    }

    pub(super) fn stamp_transient_block_178(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign52250_e79047, assign52250_e79047_d_n0, assign52250_e79047_d_n2, assign52250_e79047_d_n4, assign52250_e79047_d_n5, assign52250_e79047_d_n6, assign52250_e79047_d_n7, assign52250_e79047_d_n8, assign52250_e79047_d_n9, assign52250_e79047_d_n10, assign52250_e79047_d_n11, assign52250_e79047_d_n14,) = {
    if ((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) {
        let assign52250_e79041: f64 = (locals.var_vgsz__blk444 - locals.var_vfb);
        let assign52250_e79043: f64 = (assign52250_e79041 + locals.var_dvth);
        let assign52250_e79045: f64 = (assign52250_e79043 - locals.var_dppg);
        (assign52250_e79045, ((locals.var_vgsz__blk444_dn0 + locals.var_dvth_dn0) - locals.var_dppg_dn0), ((locals.var_vgsz__blk444_dn2 + locals.var_dvth_dn2) - locals.var_dppg_dn2), ((locals.var_vgsz__blk444_dn4 + locals.var_dvth_dn4) - locals.var_dppg_dn4), ((locals.var_vgsz__blk444_dn5 + locals.var_dvth_dn5) - locals.var_dppg_dn5), ((locals.var_vgsz__blk444_dn6 + locals.var_dvth_dn6) - locals.var_dppg_dn6), ((locals.var_vgsz__blk444_dn7 + locals.var_dvth_dn7) - locals.var_dppg_dn7), ((locals.var_vgsz__blk444_dn8 + locals.var_dvth_dn8) - locals.var_dppg_dn8), ((locals.var_vgsz__blk444_dn9 + locals.var_dvth_dn9) - locals.var_dppg_dn9), ((locals.var_vgsz__blk444_dn10 + locals.var_dvth_dn10) - locals.var_dppg_dn10), ((locals.var_vgsz__blk444_dn11 + locals.var_dvth_dn11) - locals.var_dppg_dn11), ((locals.var_vgsz__blk444_dn14 + locals.var_dvth_dn14) - locals.var_dppg_dn14),)
    } else {
        (locals.var_vgpz, locals.var_vgpz_dn0, locals.var_vgpz_dn2, locals.var_vgpz_dn4, locals.var_vgpz_dn5, locals.var_vgpz_dn6, locals.var_vgpz_dn7, locals.var_vgpz_dn8, locals.var_vgpz_dn9, locals.var_vgpz_dn10, locals.var_vgpz_dn11, locals.var_vgpz_dn14,)
    }
};
        locals.var_vgpz = assign52250_e79047;
        locals.var_vgpz_dn0 = assign52250_e79047_d_n0;
        locals.var_vgpz_dn2 = assign52250_e79047_d_n2;
        locals.var_vgpz_dn4 = assign52250_e79047_d_n4;
        locals.var_vgpz_dn5 = assign52250_e79047_d_n5;
        locals.var_vgpz_dn6 = assign52250_e79047_d_n6;
        locals.var_vgpz_dn7 = assign52250_e79047_d_n7;
        locals.var_vgpz_dn8 = assign52250_e79047_d_n8;
        locals.var_vgpz_dn9 = assign52250_e79047_d_n9;
        locals.var_vgpz_dn10 = assign52250_e79047_d_n10;
        locals.var_vgpz_dn11 = assign52250_e79047_d_n11;
        locals.var_vgpz_dn14 = assign52250_e79047_d_n14;

        let assign52260_e79050: f64 = if 0.0 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1330 = assign52260_e79050;

        let (assign52270_e79065, assign52270_e79065_d_n0, assign52270_e79065_d_n2, assign52270_e79065_d_n4, assign52270_e79065_d_n5, assign52270_e79065_d_n6, assign52270_e79065_d_n7, assign52270_e79065_d_n8, assign52270_e79065_d_n9, assign52270_e79065_d_n10, assign52270_e79065_d_n11, assign52270_e79065_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1330 != 0.0)) {
        let assign52270_e79063: f64 = (locals.var_vgpz - p.p393);
        (assign52270_e79063, locals.var_vgpz_dn0, locals.var_vgpz_dn2, locals.var_vgpz_dn4, locals.var_vgpz_dn5, locals.var_vgpz_dn6, locals.var_vgpz_dn7, locals.var_vgpz_dn8, locals.var_vgpz_dn9, locals.var_vgpz_dn10, locals.var_vgpz_dn11, locals.var_vgpz_dn14,)
    } else {
        (locals.var_vgp_res__blk1149, locals.var_vgp_res__blk1149_dn0, locals.var_vgp_res__blk1149_dn2, locals.var_vgp_res__blk1149_dn4, locals.var_vgp_res__blk1149_dn5, locals.var_vgp_res__blk1149_dn6, locals.var_vgp_res__blk1149_dn7, locals.var_vgp_res__blk1149_dn8, locals.var_vgp_res__blk1149_dn9, locals.var_vgp_res__blk1149_dn10, locals.var_vgp_res__blk1149_dn11, locals.var_vgp_res__blk1149_dn14,)
    }
};
        locals.var_vgp_res__blk1149 = assign52270_e79065;
        locals.var_vgp_res__blk1149_dn0 = assign52270_e79065_d_n0;
        locals.var_vgp_res__blk1149_dn2 = assign52270_e79065_d_n2;
        locals.var_vgp_res__blk1149_dn4 = assign52270_e79065_d_n4;
        locals.var_vgp_res__blk1149_dn5 = assign52270_e79065_d_n5;
        locals.var_vgp_res__blk1149_dn6 = assign52270_e79065_d_n6;
        locals.var_vgp_res__blk1149_dn7 = assign52270_e79065_d_n7;
        locals.var_vgp_res__blk1149_dn8 = assign52270_e79065_d_n8;
        locals.var_vgp_res__blk1149_dn9 = assign52270_e79065_d_n9;
        locals.var_vgp_res__blk1149_dn10 = assign52270_e79065_d_n10;
        locals.var_vgp_res__blk1149_dn11 = assign52270_e79065_d_n11;
        locals.var_vgp_res__blk1149_dn14 = assign52270_e79065_d_n14;

        let assign52280_e79068: f64 = if 0.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1331 = assign52280_e79068;

        let (assign52290_e79088, assign52290_e79088_d_n0, assign52290_e79088_d_n2, assign52290_e79088_d_n4, assign52290_e79088_d_n5, assign52290_e79088_d_n6, assign52290_e79088_d_n7, assign52290_e79088_d_n8, assign52290_e79088_d_n9, assign52290_e79088_d_n10, assign52290_e79088_d_n11, assign52290_e79088_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1331 != 0.0)) {
        let assign52290_e79084: f64 = (locals.var_vgsz__blk444 - locals.var_vfb);
        let assign52290_e79086: f64 = (assign52290_e79084 - p.p393);
        (assign52290_e79086, locals.var_vgsz__blk444_dn0, locals.var_vgsz__blk444_dn2, locals.var_vgsz__blk444_dn4, locals.var_vgsz__blk444_dn5, locals.var_vgsz__blk444_dn6, locals.var_vgsz__blk444_dn7, locals.var_vgsz__blk444_dn8, locals.var_vgsz__blk444_dn9, locals.var_vgsz__blk444_dn10, locals.var_vgsz__blk444_dn11, locals.var_vgsz__blk444_dn14,)
    } else {
        (locals.var_vgp_res__blk1149, locals.var_vgp_res__blk1149_dn0, locals.var_vgp_res__blk1149_dn2, locals.var_vgp_res__blk1149_dn4, locals.var_vgp_res__blk1149_dn5, locals.var_vgp_res__blk1149_dn6, locals.var_vgp_res__blk1149_dn7, locals.var_vgp_res__blk1149_dn8, locals.var_vgp_res__blk1149_dn9, locals.var_vgp_res__blk1149_dn10, locals.var_vgp_res__blk1149_dn11, locals.var_vgp_res__blk1149_dn14,)
    }
};
        locals.var_vgp_res__blk1149 = assign52290_e79088;
        locals.var_vgp_res__blk1149_dn0 = assign52290_e79088_d_n0;
        locals.var_vgp_res__blk1149_dn2 = assign52290_e79088_d_n2;
        locals.var_vgp_res__blk1149_dn4 = assign52290_e79088_d_n4;
        locals.var_vgp_res__blk1149_dn5 = assign52290_e79088_d_n5;
        locals.var_vgp_res__blk1149_dn6 = assign52290_e79088_d_n6;
        locals.var_vgp_res__blk1149_dn7 = assign52290_e79088_d_n7;
        locals.var_vgp_res__blk1149_dn8 = assign52290_e79088_d_n8;
        locals.var_vgp_res__blk1149_dn9 = assign52290_e79088_d_n9;
        locals.var_vgp_res__blk1149_dn10 = assign52290_e79088_d_n10;
        locals.var_vgp_res__blk1149_dn11 = assign52290_e79088_d_n11;
        locals.var_vgp_res__blk1149_dn14 = assign52290_e79088_d_n14;

        let (assign52300_e79107, assign52300_e79107_d_n0, assign52300_e79107_d_n2, assign52300_e79107_d_n4, assign52300_e79107_d_n5, assign52300_e79107_d_n6, assign52300_e79107_d_n7, assign52300_e79107_d_n8, assign52300_e79107_d_n9, assign52300_e79107_d_n10, assign52300_e79107_d_n11, assign52300_e79107_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1331 == 0.0)) {
        let assign52300_e79105: f64 = (locals.var_vgp - p.p393);
        (assign52300_e79105, locals.var_vgp_dn0, locals.var_vgp_dn2, locals.var_vgp_dn4, locals.var_vgp_dn5, locals.var_vgp_dn6, locals.var_vgp_dn7, locals.var_vgp_dn8, locals.var_vgp_dn9, locals.var_vgp_dn10, locals.var_vgp_dn11, locals.var_vgp_dn14,)
    } else {
        (locals.var_vgp_res__blk1149, locals.var_vgp_res__blk1149_dn0, locals.var_vgp_res__blk1149_dn2, locals.var_vgp_res__blk1149_dn4, locals.var_vgp_res__blk1149_dn5, locals.var_vgp_res__blk1149_dn6, locals.var_vgp_res__blk1149_dn7, locals.var_vgp_res__blk1149_dn8, locals.var_vgp_res__blk1149_dn9, locals.var_vgp_res__blk1149_dn10, locals.var_vgp_res__blk1149_dn11, locals.var_vgp_res__blk1149_dn14,)
    }
};
        locals.var_vgp_res__blk1149 = assign52300_e79107;
        locals.var_vgp_res__blk1149_dn0 = assign52300_e79107_d_n0;
        locals.var_vgp_res__blk1149_dn2 = assign52300_e79107_d_n2;
        locals.var_vgp_res__blk1149_dn4 = assign52300_e79107_d_n4;
        locals.var_vgp_res__blk1149_dn5 = assign52300_e79107_d_n5;
        locals.var_vgp_res__blk1149_dn6 = assign52300_e79107_d_n6;
        locals.var_vgp_res__blk1149_dn7 = assign52300_e79107_d_n7;
        locals.var_vgp_res__blk1149_dn8 = assign52300_e79107_d_n8;
        locals.var_vgp_res__blk1149_dn9 = assign52300_e79107_d_n9;
        locals.var_vgp_res__blk1149_dn10 = assign52300_e79107_d_n10;
        locals.var_vgp_res__blk1149_dn11 = assign52300_e79107_d_n11;
        locals.var_vgp_res__blk1149_dn14 = assign52300_e79107_d_n14;

        let assign52310_e79109: f64 = (locals.var_tnp__blk1152).abs();
        let assign52310_e79111: f64 = if assign52310_e79109 <= 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1332 = assign52310_e79111;

        let (assign52320_e79124, assign52320_e79124_d_n0, assign52320_e79124_d_n2, assign52320_e79124_d_n4, assign52320_e79124_d_n5, assign52320_e79124_d_n6, assign52320_e79124_d_n7, assign52320_e79124_d_n8, assign52320_e79124_d_n9, assign52320_e79124_d_n10, assign52320_e79124_d_n11, assign52320_e79124_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ps0_res, locals.var_ps0_res_dn0, locals.var_ps0_res_dn2, locals.var_ps0_res_dn4, locals.var_ps0_res_dn5, locals.var_ps0_res_dn6, locals.var_ps0_res_dn7, locals.var_ps0_res_dn8, locals.var_ps0_res_dn9, locals.var_ps0_res_dn10, locals.var_ps0_res_dn11, locals.var_ps0_res_dn14,)
    }
};
        locals.var_ps0_res = assign52320_e79124;
        locals.var_ps0_res_dn0 = assign52320_e79124_d_n0;
        locals.var_ps0_res_dn2 = assign52320_e79124_d_n2;
        locals.var_ps0_res_dn4 = assign52320_e79124_d_n4;
        locals.var_ps0_res_dn5 = assign52320_e79124_d_n5;
        locals.var_ps0_res_dn6 = assign52320_e79124_d_n6;
        locals.var_ps0_res_dn7 = assign52320_e79124_d_n7;
        locals.var_ps0_res_dn8 = assign52320_e79124_d_n8;
        locals.var_ps0_res_dn9 = assign52320_e79124_d_n9;
        locals.var_ps0_res_dn10 = assign52320_e79124_d_n10;
        locals.var_ps0_res_dn11 = assign52320_e79124_d_n11;
        locals.var_ps0_res_dn14 = assign52320_e79124_d_n14;

        let (assign52330_e79138, assign52330_e79138_d_n0, assign52330_e79138_d_n2, assign52330_e79138_d_n4, assign52330_e79138_d_n5, assign52330_e79138_d_n6, assign52330_e79138_d_n7, assign52330_e79138_d_n8, assign52330_e79138_d_n9, assign52330_e79138_d_n10, assign52330_e79138_d_n11, assign52330_e79138_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) {
        (locals.var_ps0, locals.var_ps0_dn0, locals.var_ps0_dn2, locals.var_ps0_dn4, locals.var_ps0_dn5, locals.var_ps0_dn6, locals.var_ps0_dn7, locals.var_ps0_dn8, locals.var_ps0_dn9, locals.var_ps0_dn10, locals.var_ps0_dn11, locals.var_ps0_dn14,)
    } else {
        (locals.var_ps0dep, locals.var_ps0dep_dn0, locals.var_ps0dep_dn2, locals.var_ps0dep_dn4, locals.var_ps0dep_dn5, locals.var_ps0dep_dn6, locals.var_ps0dep_dn7, locals.var_ps0dep_dn8, locals.var_ps0dep_dn9, locals.var_ps0dep_dn10, locals.var_ps0dep_dn11, locals.var_ps0dep_dn14,)
    }
};
        locals.var_ps0dep = assign52330_e79138;
        locals.var_ps0dep_dn0 = assign52330_e79138_d_n0;
        locals.var_ps0dep_dn2 = assign52330_e79138_d_n2;
        locals.var_ps0dep_dn4 = assign52330_e79138_d_n4;
        locals.var_ps0dep_dn5 = assign52330_e79138_d_n5;
        locals.var_ps0dep_dn6 = assign52330_e79138_d_n6;
        locals.var_ps0dep_dn7 = assign52330_e79138_d_n7;
        locals.var_ps0dep_dn8 = assign52330_e79138_d_n8;
        locals.var_ps0dep_dn9 = assign52330_e79138_d_n9;
        locals.var_ps0dep_dn10 = assign52330_e79138_d_n10;
        locals.var_ps0dep_dn11 = assign52330_e79138_d_n11;
        locals.var_ps0dep_dn14 = assign52330_e79138_d_n14;

        let (assign52340_e79154, assign52340_e79154_d_n0, assign52340_e79154_d_n2, assign52340_e79154_d_n4, assign52340_e79154_d_n5, assign52340_e79154_d_n6, assign52340_e79154_d_n7, assign52340_e79154_d_n8, assign52340_e79154_d_n9, assign52340_e79154_d_n10, assign52340_e79154_d_n11, assign52340_e79154_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) {
        let assign52340_e79152: f64 = (p.p399 * locals.var_vbsc__blk1121);
        (assign52340_e79152, (p.p399 * locals.var_vbsc__blk1121_dn0), (p.p399 * locals.var_vbsc__blk1121_dn2), (p.p399 * locals.var_vbsc__blk1121_dn4), (p.p399 * locals.var_vbsc__blk1121_dn5), (p.p399 * locals.var_vbsc__blk1121_dn6), (p.p399 * locals.var_vbsc__blk1121_dn7), (p.p399 * locals.var_vbsc__blk1121_dn8), (p.p399 * locals.var_vbsc__blk1121_dn9), (p.p399 * locals.var_vbsc__blk1121_dn10), (p.p399 * locals.var_vbsc__blk1121_dn11), (p.p399 * locals.var_vbsc__blk1121_dn14),)
    } else {
        (locals.var_depvbs, locals.var_depvbs_dn0, locals.var_depvbs_dn2, locals.var_depvbs_dn4, locals.var_depvbs_dn5, locals.var_depvbs_dn6, locals.var_depvbs_dn7, locals.var_depvbs_dn8, locals.var_depvbs_dn9, locals.var_depvbs_dn10, locals.var_depvbs_dn11, locals.var_depvbs_dn14,)
    }
};
        locals.var_depvbs = assign52340_e79154;
        locals.var_depvbs_dn0 = assign52340_e79154_d_n0;
        locals.var_depvbs_dn2 = assign52340_e79154_d_n2;
        locals.var_depvbs_dn4 = assign52340_e79154_d_n4;
        locals.var_depvbs_dn5 = assign52340_e79154_d_n5;
        locals.var_depvbs_dn6 = assign52340_e79154_d_n6;
        locals.var_depvbs_dn7 = assign52340_e79154_d_n7;
        locals.var_depvbs_dn8 = assign52340_e79154_d_n8;
        locals.var_depvbs_dn9 = assign52340_e79154_d_n9;
        locals.var_depvbs_dn10 = assign52340_e79154_d_n10;
        locals.var_depvbs_dn11 = assign52340_e79154_d_n11;
        locals.var_depvbs_dn14 = assign52340_e79154_d_n14;

        let (assign52350_e79172,) = {
    if (((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) {
        let assign52350_e79168: f64 = (locals.var_vfb + p.p393);
        let assign52350_e79170: f64 = (assign52350_e79168 - 3.0);
        (assign52350_e79170,)
    } else {
        (locals.var_vgp_leak,)
    }
};
        locals.var_vgp_leak = assign52350_e79172;

        let assign52360_e79175: f64 = if 1.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1333 = assign52360_e79175;

        let (assign52370_e79193, assign52370_e79193_d_n0, assign52370_e79193_d_n2, assign52370_e79193_d_n4, assign52370_e79193_d_n5, assign52370_e79193_d_n6, assign52370_e79193_d_n7, assign52370_e79193_d_n8, assign52370_e79193_d_n9, assign52370_e79193_d_n10, assign52370_e79193_d_n11, assign52370_e79193_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_guard1333 != 0.0)) {
        let assign52370_e79191: f64 = (p.p399 * locals.var_vbsc__blk1121);
        (assign52370_e79191, (p.p399 * locals.var_vbsc__blk1121_dn0), (p.p399 * locals.var_vbsc__blk1121_dn2), (p.p399 * locals.var_vbsc__blk1121_dn4), (p.p399 * locals.var_vbsc__blk1121_dn5), (p.p399 * locals.var_vbsc__blk1121_dn6), (p.p399 * locals.var_vbsc__blk1121_dn7), (p.p399 * locals.var_vbsc__blk1121_dn8), (p.p399 * locals.var_vbsc__blk1121_dn9), (p.p399 * locals.var_vbsc__blk1121_dn10), (p.p399 * locals.var_vbsc__blk1121_dn11), (p.p399 * locals.var_vbsc__blk1121_dn14),)
    } else {
        (locals.var_depvbs, locals.var_depvbs_dn0, locals.var_depvbs_dn2, locals.var_depvbs_dn4, locals.var_depvbs_dn5, locals.var_depvbs_dn6, locals.var_depvbs_dn7, locals.var_depvbs_dn8, locals.var_depvbs_dn9, locals.var_depvbs_dn10, locals.var_depvbs_dn11, locals.var_depvbs_dn14,)
    }
};
        locals.var_depvbs = assign52370_e79193;
        locals.var_depvbs_dn0 = assign52370_e79193_d_n0;
        locals.var_depvbs_dn2 = assign52370_e79193_d_n2;
        locals.var_depvbs_dn4 = assign52370_e79193_d_n4;
        locals.var_depvbs_dn5 = assign52370_e79193_d_n5;
        locals.var_depvbs_dn6 = assign52370_e79193_d_n6;
        locals.var_depvbs_dn7 = assign52370_e79193_d_n7;
        locals.var_depvbs_dn8 = assign52370_e79193_d_n8;
        locals.var_depvbs_dn9 = assign52370_e79193_d_n9;
        locals.var_depvbs_dn10 = assign52370_e79193_d_n10;
        locals.var_depvbs_dn11 = assign52370_e79193_d_n11;
        locals.var_depvbs_dn14 = assign52370_e79193_d_n14;

        let (assign52380_e79211, assign52380_e79211_d_n0, assign52380_e79211_d_n2, assign52380_e79211_d_n4, assign52380_e79211_d_n5, assign52380_e79211_d_n6, assign52380_e79211_d_n7, assign52380_e79211_d_n8, assign52380_e79211_d_n9, assign52380_e79211_d_n10, assign52380_e79211_d_n11, assign52380_e79211_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_guard1333 != 0.0)) {
        let assign52380_e79209: f64 = (locals.var_depvbs - 1.0);
        (assign52380_e79209, locals.var_depvbs_dn0, locals.var_depvbs_dn2, locals.var_depvbs_dn4, locals.var_depvbs_dn5, locals.var_depvbs_dn6, locals.var_depvbs_dn7, locals.var_depvbs_dn8, locals.var_depvbs_dn9, locals.var_depvbs_dn10, locals.var_depvbs_dn11, locals.var_depvbs_dn14,)
    } else {
        (locals.var_ps0dep, locals.var_ps0dep_dn0, locals.var_ps0dep_dn2, locals.var_ps0dep_dn4, locals.var_ps0dep_dn5, locals.var_ps0dep_dn6, locals.var_ps0dep_dn7, locals.var_ps0dep_dn8, locals.var_ps0dep_dn9, locals.var_ps0dep_dn10, locals.var_ps0dep_dn11, locals.var_ps0dep_dn14,)
    }
};
        locals.var_ps0dep = assign52380_e79211;
        locals.var_ps0dep_dn0 = assign52380_e79211_d_n0;
        locals.var_ps0dep_dn2 = assign52380_e79211_d_n2;
        locals.var_ps0dep_dn4 = assign52380_e79211_d_n4;
        locals.var_ps0dep_dn5 = assign52380_e79211_d_n5;
        locals.var_ps0dep_dn6 = assign52380_e79211_d_n6;
        locals.var_ps0dep_dn7 = assign52380_e79211_d_n7;
        locals.var_ps0dep_dn8 = assign52380_e79211_d_n8;
        locals.var_ps0dep_dn9 = assign52380_e79211_d_n9;
        locals.var_ps0dep_dn10 = assign52380_e79211_d_n10;
        locals.var_ps0dep_dn11 = assign52380_e79211_d_n11;
        locals.var_ps0dep_dn14 = assign52380_e79211_d_n14;

        let (assign52390_e79227, assign52390_e79227_d_n0, assign52390_e79227_d_n2, assign52390_e79227_d_n4, assign52390_e79227_d_n5, assign52390_e79227_d_n6, assign52390_e79227_d_n7, assign52390_e79227_d_n8, assign52390_e79227_d_n9, assign52390_e79227_d_n10, assign52390_e79227_d_n11, assign52390_e79227_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_guard1333 != 0.0)) {
        (locals.var_vgp_leak, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vgp_ws, locals.var_vgp_ws_dn0, locals.var_vgp_ws_dn2, locals.var_vgp_ws_dn4, locals.var_vgp_ws_dn5, locals.var_vgp_ws_dn6, locals.var_vgp_ws_dn7, locals.var_vgp_ws_dn8, locals.var_vgp_ws_dn9, locals.var_vgp_ws_dn10, locals.var_vgp_ws_dn11, locals.var_vgp_ws_dn14,)
    }
};
        locals.var_vgp_ws = assign52390_e79227;
        locals.var_vgp_ws_dn0 = assign52390_e79227_d_n0;
        locals.var_vgp_ws_dn2 = assign52390_e79227_d_n2;
        locals.var_vgp_ws_dn4 = assign52390_e79227_d_n4;
        locals.var_vgp_ws_dn5 = assign52390_e79227_d_n5;
        locals.var_vgp_ws_dn6 = assign52390_e79227_d_n6;
        locals.var_vgp_ws_dn7 = assign52390_e79227_d_n7;
        locals.var_vgp_ws_dn8 = assign52390_e79227_d_n8;
        locals.var_vgp_ws_dn9 = assign52390_e79227_d_n9;
        locals.var_vgp_ws_dn10 = assign52390_e79227_d_n10;
        locals.var_vgp_ws_dn11 = assign52390_e79227_d_n11;
        locals.var_vgp_ws_dn14 = assign52390_e79227_d_n14;

        let (assign52400_e79243, assign52400_e79243_d_n0, assign52400_e79243_d_n2, assign52400_e79243_d_n4, assign52400_e79243_d_n5, assign52400_e79243_d_n6, assign52400_e79243_d_n7, assign52400_e79243_d_n8, assign52400_e79243_d_n9, assign52400_e79243_d_n10, assign52400_e79243_d_n11, assign52400_e79243_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_guard1333 != 0.0)) {
        (locals.var_vgp_leak, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vgp_res_raw, locals.var_vgp_res_raw_dn0, locals.var_vgp_res_raw_dn2, locals.var_vgp_res_raw_dn4, locals.var_vgp_res_raw_dn5, locals.var_vgp_res_raw_dn6, locals.var_vgp_res_raw_dn7, locals.var_vgp_res_raw_dn8, locals.var_vgp_res_raw_dn9, locals.var_vgp_res_raw_dn10, locals.var_vgp_res_raw_dn11, locals.var_vgp_res_raw_dn14,)
    }
};
        locals.var_vgp_res_raw = assign52400_e79243;
        locals.var_vgp_res_raw_dn0 = assign52400_e79243_d_n0;
        locals.var_vgp_res_raw_dn2 = assign52400_e79243_d_n2;
        locals.var_vgp_res_raw_dn4 = assign52400_e79243_d_n4;
        locals.var_vgp_res_raw_dn5 = assign52400_e79243_d_n5;
        locals.var_vgp_res_raw_dn6 = assign52400_e79243_d_n6;
        locals.var_vgp_res_raw_dn7 = assign52400_e79243_d_n7;
        locals.var_vgp_res_raw_dn8 = assign52400_e79243_d_n8;
        locals.var_vgp_res_raw_dn9 = assign52400_e79243_d_n9;
        locals.var_vgp_res_raw_dn10 = assign52400_e79243_d_n10;
        locals.var_vgp_res_raw_dn11 = assign52400_e79243_d_n11;
        locals.var_vgp_res_raw_dn14 = assign52400_e79243_d_n14;

        let (assign52410_e79264, assign52410_e79264_d_n0, assign52410_e79264_d_n2, assign52410_e79264_d_n4, assign52410_e79264_d_n5, assign52410_e79264_d_n6, assign52410_e79264_d_n7, assign52410_e79264_d_n8, assign52410_e79264_d_n9, assign52410_e79264_d_n10, assign52410_e79264_d_n11, assign52410_e79264_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_guard1333 == 0.0)) {
        let assign52410_e79260: f64 = (p.p399 * locals.var_vbsc__blk1121);
        let assign52410_e79262: f64 = (assign52410_e79260 - 0.1);
        (assign52410_e79262, (p.p399 * locals.var_vbsc__blk1121_dn0), (p.p399 * locals.var_vbsc__blk1121_dn2), (p.p399 * locals.var_vbsc__blk1121_dn4), (p.p399 * locals.var_vbsc__blk1121_dn5), (p.p399 * locals.var_vbsc__blk1121_dn6), (p.p399 * locals.var_vbsc__blk1121_dn7), (p.p399 * locals.var_vbsc__blk1121_dn8), (p.p399 * locals.var_vbsc__blk1121_dn9), (p.p399 * locals.var_vbsc__blk1121_dn10), (p.p399 * locals.var_vbsc__blk1121_dn11), (p.p399 * locals.var_vbsc__blk1121_dn14),)
    } else {
        (locals.var_depvbs, locals.var_depvbs_dn0, locals.var_depvbs_dn2, locals.var_depvbs_dn4, locals.var_depvbs_dn5, locals.var_depvbs_dn6, locals.var_depvbs_dn7, locals.var_depvbs_dn8, locals.var_depvbs_dn9, locals.var_depvbs_dn10, locals.var_depvbs_dn11, locals.var_depvbs_dn14,)
    }
};
        locals.var_depvbs = assign52410_e79264;
        locals.var_depvbs_dn0 = assign52410_e79264_d_n0;
        locals.var_depvbs_dn2 = assign52410_e79264_d_n2;
        locals.var_depvbs_dn4 = assign52410_e79264_d_n4;
        locals.var_depvbs_dn5 = assign52410_e79264_d_n5;
        locals.var_depvbs_dn6 = assign52410_e79264_d_n6;
        locals.var_depvbs_dn7 = assign52410_e79264_d_n7;
        locals.var_depvbs_dn8 = assign52410_e79264_d_n8;
        locals.var_depvbs_dn9 = assign52410_e79264_d_n9;
        locals.var_depvbs_dn10 = assign52410_e79264_d_n10;
        locals.var_depvbs_dn11 = assign52410_e79264_d_n11;
        locals.var_depvbs_dn14 = assign52410_e79264_d_n14;

        let (assign52420_e79281, assign52420_e79281_d_n0, assign52420_e79281_d_n2, assign52420_e79281_d_n4, assign52420_e79281_d_n5, assign52420_e79281_d_n6, assign52420_e79281_d_n7, assign52420_e79281_d_n8, assign52420_e79281_d_n9, assign52420_e79281_d_n10, assign52420_e79281_d_n11, assign52420_e79281_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_guard1333 == 0.0)) {
        (locals.var_ps0, locals.var_ps0_dn0, locals.var_ps0_dn2, locals.var_ps0_dn4, locals.var_ps0_dn5, locals.var_ps0_dn6, locals.var_ps0_dn7, locals.var_ps0_dn8, locals.var_ps0_dn9, locals.var_ps0_dn10, locals.var_ps0_dn11, locals.var_ps0_dn14,)
    } else {
        (locals.var_ps0dep, locals.var_ps0dep_dn0, locals.var_ps0dep_dn2, locals.var_ps0dep_dn4, locals.var_ps0dep_dn5, locals.var_ps0dep_dn6, locals.var_ps0dep_dn7, locals.var_ps0dep_dn8, locals.var_ps0dep_dn9, locals.var_ps0dep_dn10, locals.var_ps0dep_dn11, locals.var_ps0dep_dn14,)
    }
};
        locals.var_ps0dep = assign52420_e79281;
        locals.var_ps0dep_dn0 = assign52420_e79281_d_n0;
        locals.var_ps0dep_dn2 = assign52420_e79281_d_n2;
        locals.var_ps0dep_dn4 = assign52420_e79281_d_n4;
        locals.var_ps0dep_dn5 = assign52420_e79281_d_n5;
        locals.var_ps0dep_dn6 = assign52420_e79281_d_n6;
        locals.var_ps0dep_dn7 = assign52420_e79281_d_n7;
        locals.var_ps0dep_dn8 = assign52420_e79281_d_n8;
        locals.var_ps0dep_dn9 = assign52420_e79281_d_n9;
        locals.var_ps0dep_dn10 = assign52420_e79281_d_n10;
        locals.var_ps0dep_dn11 = assign52420_e79281_d_n11;
        locals.var_ps0dep_dn14 = assign52420_e79281_d_n14;

        let (assign52430_e79298, assign52430_e79298_d_n0, assign52430_e79298_d_n2, assign52430_e79298_d_n4, assign52430_e79298_d_n5, assign52430_e79298_d_n6, assign52430_e79298_d_n7, assign52430_e79298_d_n8, assign52430_e79298_d_n9, assign52430_e79298_d_n10, assign52430_e79298_d_n11, assign52430_e79298_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_guard1333 == 0.0)) {
        (locals.var_vgp_res__blk1149, locals.var_vgp_res__blk1149_dn0, locals.var_vgp_res__blk1149_dn2, locals.var_vgp_res__blk1149_dn4, locals.var_vgp_res__blk1149_dn5, locals.var_vgp_res__blk1149_dn6, locals.var_vgp_res__blk1149_dn7, locals.var_vgp_res__blk1149_dn8, locals.var_vgp_res__blk1149_dn9, locals.var_vgp_res__blk1149_dn10, locals.var_vgp_res__blk1149_dn11, locals.var_vgp_res__blk1149_dn14,)
    } else {
        (locals.var_vgp_ws, locals.var_vgp_ws_dn0, locals.var_vgp_ws_dn2, locals.var_vgp_ws_dn4, locals.var_vgp_ws_dn5, locals.var_vgp_ws_dn6, locals.var_vgp_ws_dn7, locals.var_vgp_ws_dn8, locals.var_vgp_ws_dn9, locals.var_vgp_ws_dn10, locals.var_vgp_ws_dn11, locals.var_vgp_ws_dn14,)
    }
};
        locals.var_vgp_ws = assign52430_e79298;
        locals.var_vgp_ws_dn0 = assign52430_e79298_d_n0;
        locals.var_vgp_ws_dn2 = assign52430_e79298_d_n2;
        locals.var_vgp_ws_dn4 = assign52430_e79298_d_n4;
        locals.var_vgp_ws_dn5 = assign52430_e79298_d_n5;
        locals.var_vgp_ws_dn6 = assign52430_e79298_d_n6;
        locals.var_vgp_ws_dn7 = assign52430_e79298_d_n7;
        locals.var_vgp_ws_dn8 = assign52430_e79298_d_n8;
        locals.var_vgp_ws_dn9 = assign52430_e79298_d_n9;
        locals.var_vgp_ws_dn10 = assign52430_e79298_d_n10;
        locals.var_vgp_ws_dn11 = assign52430_e79298_d_n11;
        locals.var_vgp_ws_dn14 = assign52430_e79298_d_n14;

        let (assign52440_e79315, assign52440_e79315_d_n0, assign52440_e79315_d_n2, assign52440_e79315_d_n4, assign52440_e79315_d_n5, assign52440_e79315_d_n6, assign52440_e79315_d_n7, assign52440_e79315_d_n8, assign52440_e79315_d_n9, assign52440_e79315_d_n10, assign52440_e79315_d_n11, assign52440_e79315_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_guard1333 == 0.0)) {
        (locals.var_vgp_res__blk1149, locals.var_vgp_res__blk1149_dn0, locals.var_vgp_res__blk1149_dn2, locals.var_vgp_res__blk1149_dn4, locals.var_vgp_res__blk1149_dn5, locals.var_vgp_res__blk1149_dn6, locals.var_vgp_res__blk1149_dn7, locals.var_vgp_res__blk1149_dn8, locals.var_vgp_res__blk1149_dn9, locals.var_vgp_res__blk1149_dn10, locals.var_vgp_res__blk1149_dn11, locals.var_vgp_res__blk1149_dn14,)
    } else {
        (locals.var_vgp_res_raw, locals.var_vgp_res_raw_dn0, locals.var_vgp_res_raw_dn2, locals.var_vgp_res_raw_dn4, locals.var_vgp_res_raw_dn5, locals.var_vgp_res_raw_dn6, locals.var_vgp_res_raw_dn7, locals.var_vgp_res_raw_dn8, locals.var_vgp_res_raw_dn9, locals.var_vgp_res_raw_dn10, locals.var_vgp_res_raw_dn11, locals.var_vgp_res_raw_dn14,)
    }
};
        locals.var_vgp_res_raw = assign52440_e79315;
        locals.var_vgp_res_raw_dn0 = assign52440_e79315_d_n0;
        locals.var_vgp_res_raw_dn2 = assign52440_e79315_d_n2;
        locals.var_vgp_res_raw_dn4 = assign52440_e79315_d_n4;
        locals.var_vgp_res_raw_dn5 = assign52440_e79315_d_n5;
        locals.var_vgp_res_raw_dn6 = assign52440_e79315_d_n6;
        locals.var_vgp_res_raw_dn7 = assign52440_e79315_d_n7;
        locals.var_vgp_res_raw_dn8 = assign52440_e79315_d_n8;
        locals.var_vgp_res_raw_dn9 = assign52440_e79315_d_n9;
        locals.var_vgp_res_raw_dn10 = assign52440_e79315_d_n10;
        locals.var_vgp_res_raw_dn11 = assign52440_e79315_d_n11;
        locals.var_vgp_res_raw_dn14 = assign52440_e79315_d_n14;

        let (assign52450_e79329,) = {
    if (((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) {
        (0.0,)
    } else {
        (locals.var_flg_conv,)
    }
};
        locals.var_flg_conv = assign52450_e79329;

        let (assign52460_e79343,) = {
    if (((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) {
        (1.0,)
    } else {
        (locals.var_lp_s0,)
    }
};
        locals.var_lp_s0 = assign52460_e79343;

    }

    pub(super) fn stamp_transient_block_179(
        locals: &mut StampLocals,
    ) {
        let mut assign52470_loop_guard: usize = 0;
        while {
            let assign52470_cond_e79358: f64 = if ((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_lp_s0 <= 150.0)) { 1.0 } else { 0.0 };
            assign52470_cond_e79358 != 0.0
        } {
            assign52470_loop_guard += 1;
            assert!(assign52470_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign52470_body0_e79374, assign52470_body0_e79374_d_n0, assign52470_body0_e79374_d_n2, assign52470_body0_e79374_d_n4, assign52470_body0_e79374_d_n5, assign52470_body0_e79374_d_n6, assign52470_body0_e79374_d_n7, assign52470_body0_e79374_d_n8, assign52470_body0_e79374_d_n9, assign52470_body0_e79374_d_n10, assign52470_body0_e79374_d_n11, assign52470_body0_e79374_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) {
        let assign52470_body0_e79372: f64 = (locals.var_beta * locals.var_ps0dep);
        (assign52470_body0_e79372, ((locals.var_beta_dn0 * locals.var_ps0dep) + (locals.var_beta * locals.var_ps0dep_dn0)), ((locals.var_beta_dn2 * locals.var_ps0dep) + (locals.var_beta * locals.var_ps0dep_dn2)), ((locals.var_beta_dn4 * locals.var_ps0dep) + (locals.var_beta * locals.var_ps0dep_dn4)), ((locals.var_beta_dn5 * locals.var_ps0dep) + (locals.var_beta * locals.var_ps0dep_dn5)), ((locals.var_beta_dn6 * locals.var_ps0dep) + (locals.var_beta * locals.var_ps0dep_dn6)), ((locals.var_beta_dn7 * locals.var_ps0dep) + (locals.var_beta * locals.var_ps0dep_dn7)), ((locals.var_beta_dn8 * locals.var_ps0dep) + (locals.var_beta * locals.var_ps0dep_dn8)), ((locals.var_beta_dn9 * locals.var_ps0dep) + (locals.var_beta * locals.var_ps0dep_dn9)), ((locals.var_beta_dn10 * locals.var_ps0dep) + (locals.var_beta * locals.var_ps0dep_dn10)), ((locals.var_beta_dn11 * locals.var_ps0dep) + (locals.var_beta * locals.var_ps0dep_dn11)), ((locals.var_beta_dn14 * locals.var_ps0dep) + (locals.var_beta * locals.var_ps0dep_dn14)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
            locals.var_t1 = assign52470_body0_e79374;
            locals.var_t1_dn0 = assign52470_body0_e79374_d_n0;
            locals.var_t1_dn2 = assign52470_body0_e79374_d_n2;
            locals.var_t1_dn4 = assign52470_body0_e79374_d_n4;
            locals.var_t1_dn5 = assign52470_body0_e79374_d_n5;
            locals.var_t1_dn6 = assign52470_body0_e79374_d_n6;
            locals.var_t1_dn7 = assign52470_body0_e79374_d_n7;
            locals.var_t1_dn8 = assign52470_body0_e79374_d_n8;
            locals.var_t1_dn9 = assign52470_body0_e79374_d_n9;
            locals.var_t1_dn10 = assign52470_body0_e79374_d_n10;
            locals.var_t1_dn11 = assign52470_body0_e79374_d_n11;
            locals.var_t1_dn14 = assign52470_body0_e79374_d_n14;
            let (assign52470_body1_e79389, assign52470_body1_e79389_d_n0, assign52470_body1_e79389_d_n2, assign52470_body1_e79389_d_n4, assign52470_body1_e79389_d_n5, assign52470_body1_e79389_d_n6, assign52470_body1_e79389_d_n7, assign52470_body1_e79389_d_n8, assign52470_body1_e79389_d_n9, assign52470_body1_e79389_d_n10, assign52470_body1_e79389_d_n11, assign52470_body1_e79389_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) {
        let assign52470_body1_e79387: f64 = (locals.var_t1).exp();
        (assign52470_body1_e79387, (assign52470_body1_e79387 * locals.var_t1_dn0), (assign52470_body1_e79387 * locals.var_t1_dn2), (assign52470_body1_e79387 * locals.var_t1_dn4), (assign52470_body1_e79387 * locals.var_t1_dn5), (assign52470_body1_e79387 * locals.var_t1_dn6), (assign52470_body1_e79387 * locals.var_t1_dn7), (assign52470_body1_e79387 * locals.var_t1_dn8), (assign52470_body1_e79387 * locals.var_t1_dn9), (assign52470_body1_e79387 * locals.var_t1_dn10), (assign52470_body1_e79387 * locals.var_t1_dn11), (assign52470_body1_e79387 * locals.var_t1_dn14),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
            locals.var_t2 = assign52470_body1_e79389;
            locals.var_t2_dn0 = assign52470_body1_e79389_d_n0;
            locals.var_t2_dn2 = assign52470_body1_e79389_d_n2;
            locals.var_t2_dn4 = assign52470_body1_e79389_d_n4;
            locals.var_t2_dn5 = assign52470_body1_e79389_d_n5;
            locals.var_t2_dn6 = assign52470_body1_e79389_d_n6;
            locals.var_t2_dn7 = assign52470_body1_e79389_d_n7;
            locals.var_t2_dn8 = assign52470_body1_e79389_d_n8;
            locals.var_t2_dn9 = assign52470_body1_e79389_d_n9;
            locals.var_t2_dn10 = assign52470_body1_e79389_d_n10;
            locals.var_t2_dn11 = assign52470_body1_e79389_d_n11;
            locals.var_t2_dn14 = assign52470_body1_e79389_d_n14;
            let assign52470_body2_e79392: f64 = if locals.var_ps0dep >= 0.0 { 1.0 } else { 0.0 };
            locals.var_guard1334 = assign52470_body2_e79392;
            let (assign52470_body3_e79418, assign52470_body3_e79418_d_n0, assign52470_body3_e79418_d_n2, assign52470_body3_e79418_d_n4, assign52470_body3_e79418_d_n5, assign52470_body3_e79418_d_n6, assign52470_body3_e79418_d_n7, assign52470_body3_e79418_d_n8, assign52470_body3_e79418_d_n9, assign52470_body3_e79418_d_n10, assign52470_body3_e79418_d_n11, assign52470_body3_e79418_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_guard1334 != 0.0)) {
        let assign52470_body3_e79407: f64 = (-locals.var_cnst0);
        let assign52470_body3_e79410: f64 = (locals.var_t2 - 1.0);
        let assign52470_body3_e79412: f64 = (assign52470_body3_e79410 - locals.var_t1);
        let assign52470_body3_e79414: f64 = (assign52470_body3_e79412 + 1e-15);
        let assign52470_body3_e79415: f64 = (assign52470_body3_e79414).sqrt();
        let assign52470_body3_e79416: f64 = (assign52470_body3_e79407 * assign52470_body3_e79415);
        (assign52470_body3_e79416, (((-locals.var_cnst0_dn0) * assign52470_body3_e79415) + (assign52470_body3_e79407 * ((locals.var_t2_dn0 - locals.var_t1_dn0) / (2.0 * assign52470_body3_e79415)))), (((-locals.var_cnst0_dn2) * assign52470_body3_e79415) + (assign52470_body3_e79407 * ((locals.var_t2_dn2 - locals.var_t1_dn2) / (2.0 * assign52470_body3_e79415)))), (((-locals.var_cnst0_dn4) * assign52470_body3_e79415) + (assign52470_body3_e79407 * ((locals.var_t2_dn4 - locals.var_t1_dn4) / (2.0 * assign52470_body3_e79415)))), (((-locals.var_cnst0_dn5) * assign52470_body3_e79415) + (assign52470_body3_e79407 * ((locals.var_t2_dn5 - locals.var_t1_dn5) / (2.0 * assign52470_body3_e79415)))), (((-locals.var_cnst0_dn6) * assign52470_body3_e79415) + (assign52470_body3_e79407 * ((locals.var_t2_dn6 - locals.var_t1_dn6) / (2.0 * assign52470_body3_e79415)))), (((-locals.var_cnst0_dn7) * assign52470_body3_e79415) + (assign52470_body3_e79407 * ((locals.var_t2_dn7 - locals.var_t1_dn7) / (2.0 * assign52470_body3_e79415)))), (((-locals.var_cnst0_dn8) * assign52470_body3_e79415) + (assign52470_body3_e79407 * ((locals.var_t2_dn8 - locals.var_t1_dn8) / (2.0 * assign52470_body3_e79415)))), (((-locals.var_cnst0_dn9) * assign52470_body3_e79415) + (assign52470_body3_e79407 * ((locals.var_t2_dn9 - locals.var_t1_dn9) / (2.0 * assign52470_body3_e79415)))), (((-locals.var_cnst0_dn10) * assign52470_body3_e79415) + (assign52470_body3_e79407 * ((locals.var_t2_dn10 - locals.var_t1_dn10) / (2.0 * assign52470_body3_e79415)))), (((-locals.var_cnst0_dn11) * assign52470_body3_e79415) + (assign52470_body3_e79407 * ((locals.var_t2_dn11 - locals.var_t1_dn11) / (2.0 * assign52470_body3_e79415)))), (((-locals.var_cnst0_dn14) * assign52470_body3_e79415) + (assign52470_body3_e79407 * ((locals.var_t2_dn14 - locals.var_t1_dn14) / (2.0 * assign52470_body3_e79415)))),)
    } else {
        (locals.var_q_s0__blk1326, locals.var_q_s0__blk1326_dn0, locals.var_q_s0__blk1326_dn2, locals.var_q_s0__blk1326_dn4, locals.var_q_s0__blk1326_dn5, locals.var_q_s0__blk1326_dn6, locals.var_q_s0__blk1326_dn7, locals.var_q_s0__blk1326_dn8, locals.var_q_s0__blk1326_dn9, locals.var_q_s0__blk1326_dn10, locals.var_q_s0__blk1326_dn11, locals.var_q_s0__blk1326_dn14,)
    }
};
            locals.var_q_s0__blk1326 = assign52470_body3_e79418;
            locals.var_q_s0__blk1326_dn0 = assign52470_body3_e79418_d_n0;
            locals.var_q_s0__blk1326_dn2 = assign52470_body3_e79418_d_n2;
            locals.var_q_s0__blk1326_dn4 = assign52470_body3_e79418_d_n4;
            locals.var_q_s0__blk1326_dn5 = assign52470_body3_e79418_d_n5;
            locals.var_q_s0__blk1326_dn6 = assign52470_body3_e79418_d_n6;
            locals.var_q_s0__blk1326_dn7 = assign52470_body3_e79418_d_n7;
            locals.var_q_s0__blk1326_dn8 = assign52470_body3_e79418_d_n8;
            locals.var_q_s0__blk1326_dn9 = assign52470_body3_e79418_d_n9;
            locals.var_q_s0__blk1326_dn10 = assign52470_body3_e79418_d_n10;
            locals.var_q_s0__blk1326_dn11 = assign52470_body3_e79418_d_n11;
            locals.var_q_s0__blk1326_dn14 = assign52470_body3_e79418_d_n14;
            let (assign52470_body4_e79446, assign52470_body4_e79446_d_n0, assign52470_body4_e79446_d_n2, assign52470_body4_e79446_d_n4, assign52470_body4_e79446_d_n5, assign52470_body4_e79446_d_n6, assign52470_body4_e79446_d_n7, assign52470_body4_e79446_d_n8, assign52470_body4_e79446_d_n9, assign52470_body4_e79446_d_n10, assign52470_body4_e79446_d_n11, assign52470_body4_e79446_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_guard1334 != 0.0)) {
        let assign52470_body4_e79434: f64 = (0.5 * locals.var_cnst0);
        let assign52470_body4_e79436: f64 = (assign52470_body4_e79434 * locals.var_cnst0);
        let assign52470_body4_e79438: f64 = (assign52470_body4_e79436 / locals.var_q_s0__blk1326);
        let assign52470_body4_e79441: f64 = (locals.var_beta * locals.var_t2);
        let assign52470_body4_e79443: f64 = (assign52470_body4_e79441 - locals.var_beta);
        let assign52470_body4_e79444: f64 = (assign52470_body4_e79438 * assign52470_body4_e79443);
        (assign52470_body4_e79444, ((((((((0.5 * locals.var_cnst0_dn0) * locals.var_cnst0) + (assign52470_body4_e79434 * locals.var_cnst0_dn0)) * locals.var_q_s0__blk1326) - (assign52470_body4_e79436 * locals.var_q_s0__blk1326_dn0)) / (locals.var_q_s0__blk1326 * locals.var_q_s0__blk1326)) * assign52470_body4_e79443) + (assign52470_body4_e79438 * (((locals.var_beta_dn0 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn0)) - locals.var_beta_dn0))), ((((((((0.5 * locals.var_cnst0_dn2) * locals.var_cnst0) + (assign52470_body4_e79434 * locals.var_cnst0_dn2)) * locals.var_q_s0__blk1326) - (assign52470_body4_e79436 * locals.var_q_s0__blk1326_dn2)) / (locals.var_q_s0__blk1326 * locals.var_q_s0__blk1326)) * assign52470_body4_e79443) + (assign52470_body4_e79438 * (((locals.var_beta_dn2 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn2)) - locals.var_beta_dn2))), ((((((((0.5 * locals.var_cnst0_dn4) * locals.var_cnst0) + (assign52470_body4_e79434 * locals.var_cnst0_dn4)) * locals.var_q_s0__blk1326) - (assign52470_body4_e79436 * locals.var_q_s0__blk1326_dn4)) / (locals.var_q_s0__blk1326 * locals.var_q_s0__blk1326)) * assign52470_body4_e79443) + (assign52470_body4_e79438 * (((locals.var_beta_dn4 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn4)) - locals.var_beta_dn4))), ((((((((0.5 * locals.var_cnst0_dn5) * locals.var_cnst0) + (assign52470_body4_e79434 * locals.var_cnst0_dn5)) * locals.var_q_s0__blk1326) - (assign52470_body4_e79436 * locals.var_q_s0__blk1326_dn5)) / (locals.var_q_s0__blk1326 * locals.var_q_s0__blk1326)) * assign52470_body4_e79443) + (assign52470_body4_e79438 * (((locals.var_beta_dn5 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn5)) - locals.var_beta_dn5))), ((((((((0.5 * locals.var_cnst0_dn6) * locals.var_cnst0) + (assign52470_body4_e79434 * locals.var_cnst0_dn6)) * locals.var_q_s0__blk1326) - (assign52470_body4_e79436 * locals.var_q_s0__blk1326_dn6)) / (locals.var_q_s0__blk1326 * locals.var_q_s0__blk1326)) * assign52470_body4_e79443) + (assign52470_body4_e79438 * (((locals.var_beta_dn6 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn6)) - locals.var_beta_dn6))), ((((((((0.5 * locals.var_cnst0_dn7) * locals.var_cnst0) + (assign52470_body4_e79434 * locals.var_cnst0_dn7)) * locals.var_q_s0__blk1326) - (assign52470_body4_e79436 * locals.var_q_s0__blk1326_dn7)) / (locals.var_q_s0__blk1326 * locals.var_q_s0__blk1326)) * assign52470_body4_e79443) + (assign52470_body4_e79438 * (((locals.var_beta_dn7 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn7)) - locals.var_beta_dn7))), ((((((((0.5 * locals.var_cnst0_dn8) * locals.var_cnst0) + (assign52470_body4_e79434 * locals.var_cnst0_dn8)) * locals.var_q_s0__blk1326) - (assign52470_body4_e79436 * locals.var_q_s0__blk1326_dn8)) / (locals.var_q_s0__blk1326 * locals.var_q_s0__blk1326)) * assign52470_body4_e79443) + (assign52470_body4_e79438 * (((locals.var_beta_dn8 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn8)) - locals.var_beta_dn8))), ((((((((0.5 * locals.var_cnst0_dn9) * locals.var_cnst0) + (assign52470_body4_e79434 * locals.var_cnst0_dn9)) * locals.var_q_s0__blk1326) - (assign52470_body4_e79436 * locals.var_q_s0__blk1326_dn9)) / (locals.var_q_s0__blk1326 * locals.var_q_s0__blk1326)) * assign52470_body4_e79443) + (assign52470_body4_e79438 * (((locals.var_beta_dn9 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn9)) - locals.var_beta_dn9))), ((((((((0.5 * locals.var_cnst0_dn10) * locals.var_cnst0) + (assign52470_body4_e79434 * locals.var_cnst0_dn10)) * locals.var_q_s0__blk1326) - (assign52470_body4_e79436 * locals.var_q_s0__blk1326_dn10)) / (locals.var_q_s0__blk1326 * locals.var_q_s0__blk1326)) * assign52470_body4_e79443) + (assign52470_body4_e79438 * (((locals.var_beta_dn10 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn10)) - locals.var_beta_dn10))), ((((((((0.5 * locals.var_cnst0_dn11) * locals.var_cnst0) + (assign52470_body4_e79434 * locals.var_cnst0_dn11)) * locals.var_q_s0__blk1326) - (assign52470_body4_e79436 * locals.var_q_s0__blk1326_dn11)) / (locals.var_q_s0__blk1326 * locals.var_q_s0__blk1326)) * assign52470_body4_e79443) + (assign52470_body4_e79438 * (((locals.var_beta_dn11 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn11)) - locals.var_beta_dn11))), ((((((((0.5 * locals.var_cnst0_dn14) * locals.var_cnst0) + (assign52470_body4_e79434 * locals.var_cnst0_dn14)) * locals.var_q_s0__blk1326) - (assign52470_body4_e79436 * locals.var_q_s0__blk1326_dn14)) / (locals.var_q_s0__blk1326 * locals.var_q_s0__blk1326)) * assign52470_body4_e79443) + (assign52470_body4_e79438 * (((locals.var_beta_dn14 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn14)) - locals.var_beta_dn14))),)
    } else {
        (locals.var_q_s0_dps__blk1129, locals.var_q_s0_dps__blk1129_dn0, locals.var_q_s0_dps__blk1129_dn2, locals.var_q_s0_dps__blk1129_dn4, locals.var_q_s0_dps__blk1129_dn5, locals.var_q_s0_dps__blk1129_dn6, locals.var_q_s0_dps__blk1129_dn7, locals.var_q_s0_dps__blk1129_dn8, locals.var_q_s0_dps__blk1129_dn9, locals.var_q_s0_dps__blk1129_dn10, locals.var_q_s0_dps__blk1129_dn11, locals.var_q_s0_dps__blk1129_dn14,)
    }
};
            locals.var_q_s0_dps__blk1129 = assign52470_body4_e79446;
            locals.var_q_s0_dps__blk1129_dn0 = assign52470_body4_e79446_d_n0;
            locals.var_q_s0_dps__blk1129_dn2 = assign52470_body4_e79446_d_n2;
            locals.var_q_s0_dps__blk1129_dn4 = assign52470_body4_e79446_d_n4;
            locals.var_q_s0_dps__blk1129_dn5 = assign52470_body4_e79446_d_n5;
            locals.var_q_s0_dps__blk1129_dn6 = assign52470_body4_e79446_d_n6;
            locals.var_q_s0_dps__blk1129_dn7 = assign52470_body4_e79446_d_n7;
            locals.var_q_s0_dps__blk1129_dn8 = assign52470_body4_e79446_d_n8;
            locals.var_q_s0_dps__blk1129_dn9 = assign52470_body4_e79446_d_n9;
            locals.var_q_s0_dps__blk1129_dn10 = assign52470_body4_e79446_d_n10;
            locals.var_q_s0_dps__blk1129_dn11 = assign52470_body4_e79446_d_n11;
            locals.var_q_s0_dps__blk1129_dn14 = assign52470_body4_e79446_d_n14;
            let (assign52470_body5_e79469, assign52470_body5_e79469_d_n0, assign52470_body5_e79469_d_n2, assign52470_body5_e79469_d_n4, assign52470_body5_e79469_d_n5, assign52470_body5_e79469_d_n6, assign52470_body5_e79469_d_n7, assign52470_body5_e79469_d_n8, assign52470_body5_e79469_d_n9, assign52470_body5_e79469_d_n10, assign52470_body5_e79469_d_n11, assign52470_body5_e79469_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_guard1334 == 0.0)) {
        let assign52470_body5_e79462: f64 = (-locals.var_beta);
        let assign52470_body5_e79465: f64 = (locals.var_ps0dep - locals.var_depvbs);
        let assign52470_body5_e79466: f64 = (assign52470_body5_e79462 * assign52470_body5_e79465);
        let assign52470_body5_e79467: f64 = (assign52470_body5_e79466).exp();
        (assign52470_body5_e79467, (assign52470_body5_e79467 * (((-locals.var_beta_dn0) * assign52470_body5_e79465) + (assign52470_body5_e79462 * (locals.var_ps0dep_dn0 - locals.var_depvbs_dn0)))), (assign52470_body5_e79467 * (((-locals.var_beta_dn2) * assign52470_body5_e79465) + (assign52470_body5_e79462 * (locals.var_ps0dep_dn2 - locals.var_depvbs_dn2)))), (assign52470_body5_e79467 * (((-locals.var_beta_dn4) * assign52470_body5_e79465) + (assign52470_body5_e79462 * (locals.var_ps0dep_dn4 - locals.var_depvbs_dn4)))), (assign52470_body5_e79467 * (((-locals.var_beta_dn5) * assign52470_body5_e79465) + (assign52470_body5_e79462 * (locals.var_ps0dep_dn5 - locals.var_depvbs_dn5)))), (assign52470_body5_e79467 * (((-locals.var_beta_dn6) * assign52470_body5_e79465) + (assign52470_body5_e79462 * (locals.var_ps0dep_dn6 - locals.var_depvbs_dn6)))), (assign52470_body5_e79467 * (((-locals.var_beta_dn7) * assign52470_body5_e79465) + (assign52470_body5_e79462 * (locals.var_ps0dep_dn7 - locals.var_depvbs_dn7)))), (assign52470_body5_e79467 * (((-locals.var_beta_dn8) * assign52470_body5_e79465) + (assign52470_body5_e79462 * (locals.var_ps0dep_dn8 - locals.var_depvbs_dn8)))), (assign52470_body5_e79467 * (((-locals.var_beta_dn9) * assign52470_body5_e79465) + (assign52470_body5_e79462 * (locals.var_ps0dep_dn9 - locals.var_depvbs_dn9)))), (assign52470_body5_e79467 * (((-locals.var_beta_dn10) * assign52470_body5_e79465) + (assign52470_body5_e79462 * (locals.var_ps0dep_dn10 - locals.var_depvbs_dn10)))), (assign52470_body5_e79467 * (((-locals.var_beta_dn11) * assign52470_body5_e79465) + (assign52470_body5_e79462 * (locals.var_ps0dep_dn11 - locals.var_depvbs_dn11)))), (assign52470_body5_e79467 * (((-locals.var_beta_dn14) * assign52470_body5_e79465) + (assign52470_body5_e79462 * (locals.var_ps0dep_dn14 - locals.var_depvbs_dn14)))),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
            locals.var_t3 = assign52470_body5_e79469;
            locals.var_t3_dn0 = assign52470_body5_e79469_d_n0;
            locals.var_t3_dn2 = assign52470_body5_e79469_d_n2;
            locals.var_t3_dn4 = assign52470_body5_e79469_d_n4;
            locals.var_t3_dn5 = assign52470_body5_e79469_d_n5;
            locals.var_t3_dn6 = assign52470_body5_e79469_d_n6;
            locals.var_t3_dn7 = assign52470_body5_e79469_d_n7;
            locals.var_t3_dn8 = assign52470_body5_e79469_d_n8;
            locals.var_t3_dn9 = assign52470_body5_e79469_d_n9;
            locals.var_t3_dn10 = assign52470_body5_e79469_d_n10;
            locals.var_t3_dn11 = assign52470_body5_e79469_d_n11;
            locals.var_t3_dn14 = assign52470_body5_e79469_d_n14;
            let (assign52470_body6_e79489, assign52470_body6_e79489_d_n0, assign52470_body6_e79489_d_n2, assign52470_body6_e79489_d_n4, assign52470_body6_e79489_d_n5, assign52470_body6_e79489_d_n6, assign52470_body6_e79489_d_n7, assign52470_body6_e79489_d_n8, assign52470_body6_e79489_d_n9, assign52470_body6_e79489_d_n10, assign52470_body6_e79489_d_n11, assign52470_body6_e79489_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_guard1334 == 0.0)) {
        let assign52470_body6_e79486: f64 = (locals.var_beta * locals.var_depvbs);
        let assign52470_body6_e79487: f64 = (assign52470_body6_e79486).exp();
        (assign52470_body6_e79487, (assign52470_body6_e79487 * ((locals.var_beta_dn0 * locals.var_depvbs) + (locals.var_beta * locals.var_depvbs_dn0))), (assign52470_body6_e79487 * ((locals.var_beta_dn2 * locals.var_depvbs) + (locals.var_beta * locals.var_depvbs_dn2))), (assign52470_body6_e79487 * ((locals.var_beta_dn4 * locals.var_depvbs) + (locals.var_beta * locals.var_depvbs_dn4))), (assign52470_body6_e79487 * ((locals.var_beta_dn5 * locals.var_depvbs) + (locals.var_beta * locals.var_depvbs_dn5))), (assign52470_body6_e79487 * ((locals.var_beta_dn6 * locals.var_depvbs) + (locals.var_beta * locals.var_depvbs_dn6))), (assign52470_body6_e79487 * ((locals.var_beta_dn7 * locals.var_depvbs) + (locals.var_beta * locals.var_depvbs_dn7))), (assign52470_body6_e79487 * ((locals.var_beta_dn8 * locals.var_depvbs) + (locals.var_beta * locals.var_depvbs_dn8))), (assign52470_body6_e79487 * ((locals.var_beta_dn9 * locals.var_depvbs) + (locals.var_beta * locals.var_depvbs_dn9))), (assign52470_body6_e79487 * ((locals.var_beta_dn10 * locals.var_depvbs) + (locals.var_beta * locals.var_depvbs_dn10))), (assign52470_body6_e79487 * ((locals.var_beta_dn11 * locals.var_depvbs) + (locals.var_beta * locals.var_depvbs_dn11))), (assign52470_body6_e79487 * ((locals.var_beta_dn14 * locals.var_depvbs) + (locals.var_beta * locals.var_depvbs_dn14))),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
            locals.var_t4 = assign52470_body6_e79489;
            locals.var_t4_dn0 = assign52470_body6_e79489_d_n0;
            locals.var_t4_dn2 = assign52470_body6_e79489_d_n2;
            locals.var_t4_dn4 = assign52470_body6_e79489_d_n4;
            locals.var_t4_dn5 = assign52470_body6_e79489_d_n5;
            locals.var_t4_dn6 = assign52470_body6_e79489_d_n6;
            locals.var_t4_dn7 = assign52470_body6_e79489_d_n7;
            locals.var_t4_dn8 = assign52470_body6_e79489_d_n8;
            locals.var_t4_dn9 = assign52470_body6_e79489_d_n9;
            locals.var_t4_dn10 = assign52470_body6_e79489_d_n10;
            locals.var_t4_dn11 = assign52470_body6_e79489_d_n11;
            locals.var_t4_dn14 = assign52470_body6_e79489_d_n14;
            let (assign52470_body7_e79521, assign52470_body7_e79521_d_n0, assign52470_body7_e79521_d_n2, assign52470_body7_e79521_d_n4, assign52470_body7_e79521_d_n5, assign52470_body7_e79521_d_n6, assign52470_body7_e79521_d_n7, assign52470_body7_e79521_d_n8, assign52470_body7_e79521_d_n9, assign52470_body7_e79521_d_n10, assign52470_body7_e79521_d_n11, assign52470_body7_e79521_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_guard1334 == 0.0)) {
        let assign52470_body7_e79507: f64 = (locals.var_t2 - 1.0);
        let assign52470_body7_e79509: f64 = (assign52470_body7_e79507 - locals.var_t1);
        let assign52470_body7_e79513: f64 = (locals.var_t3 - locals.var_t4);
        let assign52470_body7_e79514: f64 = (locals.var_cnst1 * assign52470_body7_e79513);
        let assign52470_body7_e79515: f64 = (assign52470_body7_e79509 + assign52470_body7_e79514);
        let assign52470_body7_e79517: f64 = (assign52470_body7_e79515 + 1e-15);
        let assign52470_body7_e79518: f64 = (assign52470_body7_e79517).sqrt();
        let assign52470_body7_e79519: f64 = (locals.var_cnst0 * assign52470_body7_e79518);
        (assign52470_body7_e79519, ((locals.var_cnst0_dn0 * assign52470_body7_e79518) + (locals.var_cnst0 * (((locals.var_t2_dn0 - locals.var_t1_dn0) + ((locals.var_cnst1_dn0 * assign52470_body7_e79513) + (locals.var_cnst1 * (locals.var_t3_dn0 - locals.var_t4_dn0)))) / (2.0 * assign52470_body7_e79518)))), ((locals.var_cnst0_dn2 * assign52470_body7_e79518) + (locals.var_cnst0 * (((locals.var_t2_dn2 - locals.var_t1_dn2) + ((locals.var_cnst1_dn2 * assign52470_body7_e79513) + (locals.var_cnst1 * (locals.var_t3_dn2 - locals.var_t4_dn2)))) / (2.0 * assign52470_body7_e79518)))), ((locals.var_cnst0_dn4 * assign52470_body7_e79518) + (locals.var_cnst0 * (((locals.var_t2_dn4 - locals.var_t1_dn4) + ((locals.var_cnst1_dn4 * assign52470_body7_e79513) + (locals.var_cnst1 * (locals.var_t3_dn4 - locals.var_t4_dn4)))) / (2.0 * assign52470_body7_e79518)))), ((locals.var_cnst0_dn5 * assign52470_body7_e79518) + (locals.var_cnst0 * (((locals.var_t2_dn5 - locals.var_t1_dn5) + ((locals.var_cnst1_dn5 * assign52470_body7_e79513) + (locals.var_cnst1 * (locals.var_t3_dn5 - locals.var_t4_dn5)))) / (2.0 * assign52470_body7_e79518)))), ((locals.var_cnst0_dn6 * assign52470_body7_e79518) + (locals.var_cnst0 * (((locals.var_t2_dn6 - locals.var_t1_dn6) + ((locals.var_cnst1_dn6 * assign52470_body7_e79513) + (locals.var_cnst1 * (locals.var_t3_dn6 - locals.var_t4_dn6)))) / (2.0 * assign52470_body7_e79518)))), ((locals.var_cnst0_dn7 * assign52470_body7_e79518) + (locals.var_cnst0 * (((locals.var_t2_dn7 - locals.var_t1_dn7) + ((locals.var_cnst1_dn7 * assign52470_body7_e79513) + (locals.var_cnst1 * (locals.var_t3_dn7 - locals.var_t4_dn7)))) / (2.0 * assign52470_body7_e79518)))), ((locals.var_cnst0_dn8 * assign52470_body7_e79518) + (locals.var_cnst0 * (((locals.var_t2_dn8 - locals.var_t1_dn8) + ((locals.var_cnst1_dn8 * assign52470_body7_e79513) + (locals.var_cnst1 * (locals.var_t3_dn8 - locals.var_t4_dn8)))) / (2.0 * assign52470_body7_e79518)))), ((locals.var_cnst0_dn9 * assign52470_body7_e79518) + (locals.var_cnst0 * (((locals.var_t2_dn9 - locals.var_t1_dn9) + ((locals.var_cnst1_dn9 * assign52470_body7_e79513) + (locals.var_cnst1 * (locals.var_t3_dn9 - locals.var_t4_dn9)))) / (2.0 * assign52470_body7_e79518)))), ((locals.var_cnst0_dn10 * assign52470_body7_e79518) + (locals.var_cnst0 * (((locals.var_t2_dn10 - locals.var_t1_dn10) + ((locals.var_cnst1_dn10 * assign52470_body7_e79513) + (locals.var_cnst1 * (locals.var_t3_dn10 - locals.var_t4_dn10)))) / (2.0 * assign52470_body7_e79518)))), ((locals.var_cnst0_dn11 * assign52470_body7_e79518) + (locals.var_cnst0 * (((locals.var_t2_dn11 - locals.var_t1_dn11) + ((locals.var_cnst1_dn11 * assign52470_body7_e79513) + (locals.var_cnst1 * (locals.var_t3_dn11 - locals.var_t4_dn11)))) / (2.0 * assign52470_body7_e79518)))), ((locals.var_cnst0_dn14 * assign52470_body7_e79518) + (locals.var_cnst0 * (((locals.var_t2_dn14 - locals.var_t1_dn14) + ((locals.var_cnst1_dn14 * assign52470_body7_e79513) + (locals.var_cnst1 * (locals.var_t3_dn14 - locals.var_t4_dn14)))) / (2.0 * assign52470_body7_e79518)))),)
    } else {
        (locals.var_q_s0__blk1326, locals.var_q_s0__blk1326_dn0, locals.var_q_s0__blk1326_dn2, locals.var_q_s0__blk1326_dn4, locals.var_q_s0__blk1326_dn5, locals.var_q_s0__blk1326_dn6, locals.var_q_s0__blk1326_dn7, locals.var_q_s0__blk1326_dn8, locals.var_q_s0__blk1326_dn9, locals.var_q_s0__blk1326_dn10, locals.var_q_s0__blk1326_dn11, locals.var_q_s0__blk1326_dn14,)
    }
};
            locals.var_q_s0__blk1326 = assign52470_body7_e79521;
            locals.var_q_s0__blk1326_dn0 = assign52470_body7_e79521_d_n0;
            locals.var_q_s0__blk1326_dn2 = assign52470_body7_e79521_d_n2;
            locals.var_q_s0__blk1326_dn4 = assign52470_body7_e79521_d_n4;
            locals.var_q_s0__blk1326_dn5 = assign52470_body7_e79521_d_n5;
            locals.var_q_s0__blk1326_dn6 = assign52470_body7_e79521_d_n6;
            locals.var_q_s0__blk1326_dn7 = assign52470_body7_e79521_d_n7;
            locals.var_q_s0__blk1326_dn8 = assign52470_body7_e79521_d_n8;
            locals.var_q_s0__blk1326_dn9 = assign52470_body7_e79521_d_n9;
            locals.var_q_s0__blk1326_dn10 = assign52470_body7_e79521_d_n10;
            locals.var_q_s0__blk1326_dn11 = assign52470_body7_e79521_d_n11;
            locals.var_q_s0__blk1326_dn14 = assign52470_body7_e79521_d_n14;
            let (assign52470_body8_e79544, assign52470_body8_e79544_d_n0, assign52470_body8_e79544_d_n2, assign52470_body8_e79544_d_n4, assign52470_body8_e79544_d_n5, assign52470_body8_e79544_d_n6, assign52470_body8_e79544_d_n7, assign52470_body8_e79544_d_n8, assign52470_body8_e79544_d_n9, assign52470_body8_e79544_d_n10, assign52470_body8_e79544_d_n11, assign52470_body8_e79544_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_guard1334 == 0.0)) {
        let assign52470_body8_e79538: f64 = (0.5 * locals.var_cnst0);
        let assign52470_body8_e79540: f64 = (assign52470_body8_e79538 * locals.var_cnst0);
        let assign52470_body8_e79542: f64 = (assign52470_body8_e79540 / locals.var_q_s0__blk1326);
        (assign52470_body8_e79542, ((((((0.5 * locals.var_cnst0_dn0) * locals.var_cnst0) + (assign52470_body8_e79538 * locals.var_cnst0_dn0)) * locals.var_q_s0__blk1326) - (assign52470_body8_e79540 * locals.var_q_s0__blk1326_dn0)) / (locals.var_q_s0__blk1326 * locals.var_q_s0__blk1326)), ((((((0.5 * locals.var_cnst0_dn2) * locals.var_cnst0) + (assign52470_body8_e79538 * locals.var_cnst0_dn2)) * locals.var_q_s0__blk1326) - (assign52470_body8_e79540 * locals.var_q_s0__blk1326_dn2)) / (locals.var_q_s0__blk1326 * locals.var_q_s0__blk1326)), ((((((0.5 * locals.var_cnst0_dn4) * locals.var_cnst0) + (assign52470_body8_e79538 * locals.var_cnst0_dn4)) * locals.var_q_s0__blk1326) - (assign52470_body8_e79540 * locals.var_q_s0__blk1326_dn4)) / (locals.var_q_s0__blk1326 * locals.var_q_s0__blk1326)), ((((((0.5 * locals.var_cnst0_dn5) * locals.var_cnst0) + (assign52470_body8_e79538 * locals.var_cnst0_dn5)) * locals.var_q_s0__blk1326) - (assign52470_body8_e79540 * locals.var_q_s0__blk1326_dn5)) / (locals.var_q_s0__blk1326 * locals.var_q_s0__blk1326)), ((((((0.5 * locals.var_cnst0_dn6) * locals.var_cnst0) + (assign52470_body8_e79538 * locals.var_cnst0_dn6)) * locals.var_q_s0__blk1326) - (assign52470_body8_e79540 * locals.var_q_s0__blk1326_dn6)) / (locals.var_q_s0__blk1326 * locals.var_q_s0__blk1326)), ((((((0.5 * locals.var_cnst0_dn7) * locals.var_cnst0) + (assign52470_body8_e79538 * locals.var_cnst0_dn7)) * locals.var_q_s0__blk1326) - (assign52470_body8_e79540 * locals.var_q_s0__blk1326_dn7)) / (locals.var_q_s0__blk1326 * locals.var_q_s0__blk1326)), ((((((0.5 * locals.var_cnst0_dn8) * locals.var_cnst0) + (assign52470_body8_e79538 * locals.var_cnst0_dn8)) * locals.var_q_s0__blk1326) - (assign52470_body8_e79540 * locals.var_q_s0__blk1326_dn8)) / (locals.var_q_s0__blk1326 * locals.var_q_s0__blk1326)), ((((((0.5 * locals.var_cnst0_dn9) * locals.var_cnst0) + (assign52470_body8_e79538 * locals.var_cnst0_dn9)) * locals.var_q_s0__blk1326) - (assign52470_body8_e79540 * locals.var_q_s0__blk1326_dn9)) / (locals.var_q_s0__blk1326 * locals.var_q_s0__blk1326)), ((((((0.5 * locals.var_cnst0_dn10) * locals.var_cnst0) + (assign52470_body8_e79538 * locals.var_cnst0_dn10)) * locals.var_q_s0__blk1326) - (assign52470_body8_e79540 * locals.var_q_s0__blk1326_dn10)) / (locals.var_q_s0__blk1326 * locals.var_q_s0__blk1326)), ((((((0.5 * locals.var_cnst0_dn11) * locals.var_cnst0) + (assign52470_body8_e79538 * locals.var_cnst0_dn11)) * locals.var_q_s0__blk1326) - (assign52470_body8_e79540 * locals.var_q_s0__blk1326_dn11)) / (locals.var_q_s0__blk1326 * locals.var_q_s0__blk1326)), ((((((0.5 * locals.var_cnst0_dn14) * locals.var_cnst0) + (assign52470_body8_e79538 * locals.var_cnst0_dn14)) * locals.var_q_s0__blk1326) - (assign52470_body8_e79540 * locals.var_q_s0__blk1326_dn14)) / (locals.var_q_s0__blk1326 * locals.var_q_s0__blk1326)),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn14,)
    }
};
            locals.var_t5 = assign52470_body8_e79544;
            locals.var_t5_dn0 = assign52470_body8_e79544_d_n0;
            locals.var_t5_dn2 = assign52470_body8_e79544_d_n2;
            locals.var_t5_dn4 = assign52470_body8_e79544_d_n4;
            locals.var_t5_dn5 = assign52470_body8_e79544_d_n5;
            locals.var_t5_dn6 = assign52470_body8_e79544_d_n6;
            locals.var_t5_dn7 = assign52470_body8_e79544_d_n7;
            locals.var_t5_dn8 = assign52470_body8_e79544_d_n8;
            locals.var_t5_dn9 = assign52470_body8_e79544_d_n9;
            locals.var_t5_dn10 = assign52470_body8_e79544_d_n10;
            locals.var_t5_dn11 = assign52470_body8_e79544_d_n11;
            locals.var_t5_dn14 = assign52470_body8_e79544_d_n14;
            let (assign52470_body9_e79574, assign52470_body9_e79574_d_n0, assign52470_body9_e79574_d_n2, assign52470_body9_e79574_d_n4, assign52470_body9_e79574_d_n5, assign52470_body9_e79574_d_n6, assign52470_body9_e79574_d_n7, assign52470_body9_e79574_d_n8, assign52470_body9_e79574_d_n9, assign52470_body9_e79574_d_n10, assign52470_body9_e79574_d_n11, assign52470_body9_e79574_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_guard1334 == 0.0)) {
        let assign52470_body9_e79562: f64 = (locals.var_beta * locals.var_t2);
        let assign52470_body9_e79564: f64 = (assign52470_body9_e79562 - locals.var_beta);
        let assign52470_body9_e79567: f64 = (-locals.var_beta);
        let assign52470_body9_e79569: f64 = (assign52470_body9_e79567 * locals.var_t3);
        let assign52470_body9_e79570: f64 = (locals.var_cnst1 * assign52470_body9_e79569);
        let assign52470_body9_e79571: f64 = (assign52470_body9_e79564 + assign52470_body9_e79570);
        let assign52470_body9_e79572: f64 = (locals.var_t5 * assign52470_body9_e79571);
        (assign52470_body9_e79572, ((locals.var_t5_dn0 * assign52470_body9_e79571) + (locals.var_t5 * ((((locals.var_beta_dn0 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn0)) - locals.var_beta_dn0) + ((locals.var_cnst1_dn0 * assign52470_body9_e79569) + (locals.var_cnst1 * (((-locals.var_beta_dn0) * locals.var_t3) + (assign52470_body9_e79567 * locals.var_t3_dn0))))))), ((locals.var_t5_dn2 * assign52470_body9_e79571) + (locals.var_t5 * ((((locals.var_beta_dn2 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn2)) - locals.var_beta_dn2) + ((locals.var_cnst1_dn2 * assign52470_body9_e79569) + (locals.var_cnst1 * (((-locals.var_beta_dn2) * locals.var_t3) + (assign52470_body9_e79567 * locals.var_t3_dn2))))))), ((locals.var_t5_dn4 * assign52470_body9_e79571) + (locals.var_t5 * ((((locals.var_beta_dn4 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn4)) - locals.var_beta_dn4) + ((locals.var_cnst1_dn4 * assign52470_body9_e79569) + (locals.var_cnst1 * (((-locals.var_beta_dn4) * locals.var_t3) + (assign52470_body9_e79567 * locals.var_t3_dn4))))))), ((locals.var_t5_dn5 * assign52470_body9_e79571) + (locals.var_t5 * ((((locals.var_beta_dn5 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn5)) - locals.var_beta_dn5) + ((locals.var_cnst1_dn5 * assign52470_body9_e79569) + (locals.var_cnst1 * (((-locals.var_beta_dn5) * locals.var_t3) + (assign52470_body9_e79567 * locals.var_t3_dn5))))))), ((locals.var_t5_dn6 * assign52470_body9_e79571) + (locals.var_t5 * ((((locals.var_beta_dn6 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn6)) - locals.var_beta_dn6) + ((locals.var_cnst1_dn6 * assign52470_body9_e79569) + (locals.var_cnst1 * (((-locals.var_beta_dn6) * locals.var_t3) + (assign52470_body9_e79567 * locals.var_t3_dn6))))))), ((locals.var_t5_dn7 * assign52470_body9_e79571) + (locals.var_t5 * ((((locals.var_beta_dn7 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn7)) - locals.var_beta_dn7) + ((locals.var_cnst1_dn7 * assign52470_body9_e79569) + (locals.var_cnst1 * (((-locals.var_beta_dn7) * locals.var_t3) + (assign52470_body9_e79567 * locals.var_t3_dn7))))))), ((locals.var_t5_dn8 * assign52470_body9_e79571) + (locals.var_t5 * ((((locals.var_beta_dn8 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn8)) - locals.var_beta_dn8) + ((locals.var_cnst1_dn8 * assign52470_body9_e79569) + (locals.var_cnst1 * (((-locals.var_beta_dn8) * locals.var_t3) + (assign52470_body9_e79567 * locals.var_t3_dn8))))))), ((locals.var_t5_dn9 * assign52470_body9_e79571) + (locals.var_t5 * ((((locals.var_beta_dn9 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn9)) - locals.var_beta_dn9) + ((locals.var_cnst1_dn9 * assign52470_body9_e79569) + (locals.var_cnst1 * (((-locals.var_beta_dn9) * locals.var_t3) + (assign52470_body9_e79567 * locals.var_t3_dn9))))))), ((locals.var_t5_dn10 * assign52470_body9_e79571) + (locals.var_t5 * ((((locals.var_beta_dn10 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn10)) - locals.var_beta_dn10) + ((locals.var_cnst1_dn10 * assign52470_body9_e79569) + (locals.var_cnst1 * (((-locals.var_beta_dn10) * locals.var_t3) + (assign52470_body9_e79567 * locals.var_t3_dn10))))))), ((locals.var_t5_dn11 * assign52470_body9_e79571) + (locals.var_t5 * ((((locals.var_beta_dn11 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn11)) - locals.var_beta_dn11) + ((locals.var_cnst1_dn11 * assign52470_body9_e79569) + (locals.var_cnst1 * (((-locals.var_beta_dn11) * locals.var_t3) + (assign52470_body9_e79567 * locals.var_t3_dn11))))))), ((locals.var_t5_dn14 * assign52470_body9_e79571) + (locals.var_t5 * ((((locals.var_beta_dn14 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn14)) - locals.var_beta_dn14) + ((locals.var_cnst1_dn14 * assign52470_body9_e79569) + (locals.var_cnst1 * (((-locals.var_beta_dn14) * locals.var_t3) + (assign52470_body9_e79567 * locals.var_t3_dn14))))))),)
    } else {
        (locals.var_q_s0_dps__blk1129, locals.var_q_s0_dps__blk1129_dn0, locals.var_q_s0_dps__blk1129_dn2, locals.var_q_s0_dps__blk1129_dn4, locals.var_q_s0_dps__blk1129_dn5, locals.var_q_s0_dps__blk1129_dn6, locals.var_q_s0_dps__blk1129_dn7, locals.var_q_s0_dps__blk1129_dn8, locals.var_q_s0_dps__blk1129_dn9, locals.var_q_s0_dps__blk1129_dn10, locals.var_q_s0_dps__blk1129_dn11, locals.var_q_s0_dps__blk1129_dn14,)
    }
};
            locals.var_q_s0_dps__blk1129 = assign52470_body9_e79574;
            locals.var_q_s0_dps__blk1129_dn0 = assign52470_body9_e79574_d_n0;
            locals.var_q_s0_dps__blk1129_dn2 = assign52470_body9_e79574_d_n2;
            locals.var_q_s0_dps__blk1129_dn4 = assign52470_body9_e79574_d_n4;
            locals.var_q_s0_dps__blk1129_dn5 = assign52470_body9_e79574_d_n5;
            locals.var_q_s0_dps__blk1129_dn6 = assign52470_body9_e79574_d_n6;
            locals.var_q_s0_dps__blk1129_dn7 = assign52470_body9_e79574_d_n7;
            locals.var_q_s0_dps__blk1129_dn8 = assign52470_body9_e79574_d_n8;
            locals.var_q_s0_dps__blk1129_dn9 = assign52470_body9_e79574_d_n9;
            locals.var_q_s0_dps__blk1129_dn10 = assign52470_body9_e79574_d_n10;
            locals.var_q_s0_dps__blk1129_dn11 = assign52470_body9_e79574_d_n11;
            locals.var_q_s0_dps__blk1129_dn14 = assign52470_body9_e79574_d_n14;
            let (assign52470_body10_e79592,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_flg_conv != 0.0)) {
        let assign52470_body10_e79590: f64 = (150.0 + 1.0);
        (assign52470_body10_e79590,)
    } else {
        (locals.var_lp_s0,)
    }
};
            locals.var_lp_s0 = assign52470_body10_e79592;
            let (assign52470_body11_e79615, assign52470_body11_e79615_d_n0, assign52470_body11_e79615_d_n2, assign52470_body11_e79615_d_n4, assign52470_body11_e79615_d_n5, assign52470_body11_e79615_d_n6, assign52470_body11_e79615_d_n7, assign52470_body11_e79615_d_n8, assign52470_body11_e79615_d_n9, assign52470_body11_e79615_d_n10, assign52470_body11_e79615_d_n11, assign52470_body11_e79615_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_flg_conv == 0.0)) {
        let assign52470_body11_e79610: f64 = (locals.var_vgp_ws - locals.var_ps0dep);
        let assign52470_body11_e79611: f64 = (locals.var_cox * assign52470_body11_e79610);
        let assign52470_body11_e79613: f64 = (assign52470_body11_e79611 + locals.var_q_s0__blk1326);
        (assign52470_body11_e79613, (((locals.var_cox_dn0 * assign52470_body11_e79610) + (locals.var_cox * (locals.var_vgp_ws_dn0 - locals.var_ps0dep_dn0))) + locals.var_q_s0__blk1326_dn0), (((locals.var_cox_dn2 * assign52470_body11_e79610) + (locals.var_cox * (locals.var_vgp_ws_dn2 - locals.var_ps0dep_dn2))) + locals.var_q_s0__blk1326_dn2), (((locals.var_cox_dn4 * assign52470_body11_e79610) + (locals.var_cox * (locals.var_vgp_ws_dn4 - locals.var_ps0dep_dn4))) + locals.var_q_s0__blk1326_dn4), (((locals.var_cox_dn5 * assign52470_body11_e79610) + (locals.var_cox * (locals.var_vgp_ws_dn5 - locals.var_ps0dep_dn5))) + locals.var_q_s0__blk1326_dn5), (((locals.var_cox_dn6 * assign52470_body11_e79610) + (locals.var_cox * (locals.var_vgp_ws_dn6 - locals.var_ps0dep_dn6))) + locals.var_q_s0__blk1326_dn6), (((locals.var_cox_dn7 * assign52470_body11_e79610) + (locals.var_cox * (locals.var_vgp_ws_dn7 - locals.var_ps0dep_dn7))) + locals.var_q_s0__blk1326_dn7), (((locals.var_cox_dn8 * assign52470_body11_e79610) + (locals.var_cox * (locals.var_vgp_ws_dn8 - locals.var_ps0dep_dn8))) + locals.var_q_s0__blk1326_dn8), (((locals.var_cox_dn9 * assign52470_body11_e79610) + (locals.var_cox * (locals.var_vgp_ws_dn9 - locals.var_ps0dep_dn9))) + locals.var_q_s0__blk1326_dn9), (((locals.var_cox_dn10 * assign52470_body11_e79610) + (locals.var_cox * (locals.var_vgp_ws_dn10 - locals.var_ps0dep_dn10))) + locals.var_q_s0__blk1326_dn10), (((locals.var_cox_dn11 * assign52470_body11_e79610) + (locals.var_cox * (locals.var_vgp_ws_dn11 - locals.var_ps0dep_dn11))) + locals.var_q_s0__blk1326_dn11), (((locals.var_cox_dn14 * assign52470_body11_e79610) + (locals.var_cox * (locals.var_vgp_ws_dn14 - locals.var_ps0dep_dn14))) + locals.var_q_s0__blk1326_dn14),)
    } else {
        (locals.var_pf1__blk1104, locals.var_pf1__blk1104_dn0, locals.var_pf1__blk1104_dn2, locals.var_pf1__blk1104_dn4, locals.var_pf1__blk1104_dn5, locals.var_pf1__blk1104_dn6, locals.var_pf1__blk1104_dn7, locals.var_pf1__blk1104_dn8, locals.var_pf1__blk1104_dn9, locals.var_pf1__blk1104_dn10, locals.var_pf1__blk1104_dn11, locals.var_pf1__blk1104_dn14,)
    }
};
            locals.var_pf1__blk1104 = assign52470_body11_e79615;
            locals.var_pf1__blk1104_dn0 = assign52470_body11_e79615_d_n0;
            locals.var_pf1__blk1104_dn2 = assign52470_body11_e79615_d_n2;
            locals.var_pf1__blk1104_dn4 = assign52470_body11_e79615_d_n4;
            locals.var_pf1__blk1104_dn5 = assign52470_body11_e79615_d_n5;
            locals.var_pf1__blk1104_dn6 = assign52470_body11_e79615_d_n6;
            locals.var_pf1__blk1104_dn7 = assign52470_body11_e79615_d_n7;
            locals.var_pf1__blk1104_dn8 = assign52470_body11_e79615_d_n8;
            locals.var_pf1__blk1104_dn9 = assign52470_body11_e79615_d_n9;
            locals.var_pf1__blk1104_dn10 = assign52470_body11_e79615_d_n10;
            locals.var_pf1__blk1104_dn11 = assign52470_body11_e79615_d_n11;
            locals.var_pf1__blk1104_dn14 = assign52470_body11_e79615_d_n14;
            let (assign52470_body12_e79635, assign52470_body12_e79635_d_n0, assign52470_body12_e79635_d_n2, assign52470_body12_e79635_d_n4, assign52470_body12_e79635_d_n5, assign52470_body12_e79635_d_n6, assign52470_body12_e79635_d_n7, assign52470_body12_e79635_d_n8, assign52470_body12_e79635_d_n9, assign52470_body12_e79635_d_n10, assign52470_body12_e79635_d_n11, assign52470_body12_e79635_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_flg_conv == 0.0)) {
        let assign52470_body12_e79631: f64 = (-locals.var_cox);
        let assign52470_body12_e79633: f64 = (assign52470_body12_e79631 + locals.var_q_s0_dps__blk1129);
        (assign52470_body12_e79633, ((-locals.var_cox_dn0) + locals.var_q_s0_dps__blk1129_dn0), ((-locals.var_cox_dn2) + locals.var_q_s0_dps__blk1129_dn2), ((-locals.var_cox_dn4) + locals.var_q_s0_dps__blk1129_dn4), ((-locals.var_cox_dn5) + locals.var_q_s0_dps__blk1129_dn5), ((-locals.var_cox_dn6) + locals.var_q_s0_dps__blk1129_dn6), ((-locals.var_cox_dn7) + locals.var_q_s0_dps__blk1129_dn7), ((-locals.var_cox_dn8) + locals.var_q_s0_dps__blk1129_dn8), ((-locals.var_cox_dn9) + locals.var_q_s0_dps__blk1129_dn9), ((-locals.var_cox_dn10) + locals.var_q_s0_dps__blk1129_dn10), ((-locals.var_cox_dn11) + locals.var_q_s0_dps__blk1129_dn11), ((-locals.var_cox_dn14) + locals.var_q_s0_dps__blk1129_dn14),)
    } else {
        (locals.var_pf11__blk1105, locals.var_pf11__blk1105_dn0, locals.var_pf11__blk1105_dn2, locals.var_pf11__blk1105_dn4, locals.var_pf11__blk1105_dn5, locals.var_pf11__blk1105_dn6, locals.var_pf11__blk1105_dn7, locals.var_pf11__blk1105_dn8, locals.var_pf11__blk1105_dn9, locals.var_pf11__blk1105_dn10, locals.var_pf11__blk1105_dn11, locals.var_pf11__blk1105_dn14,)
    }
};
            locals.var_pf11__blk1105 = assign52470_body12_e79635;
            locals.var_pf11__blk1105_dn0 = assign52470_body12_e79635_d_n0;
            locals.var_pf11__blk1105_dn2 = assign52470_body12_e79635_d_n2;
            locals.var_pf11__blk1105_dn4 = assign52470_body12_e79635_d_n4;
            locals.var_pf11__blk1105_dn5 = assign52470_body12_e79635_d_n5;
            locals.var_pf11__blk1105_dn6 = assign52470_body12_e79635_d_n6;
            locals.var_pf11__blk1105_dn7 = assign52470_body12_e79635_d_n7;
            locals.var_pf11__blk1105_dn8 = assign52470_body12_e79635_d_n8;
            locals.var_pf11__blk1105_dn9 = assign52470_body12_e79635_d_n9;
            locals.var_pf11__blk1105_dn10 = assign52470_body12_e79635_d_n10;
            locals.var_pf11__blk1105_dn11 = assign52470_body12_e79635_d_n11;
            locals.var_pf11__blk1105_dn14 = assign52470_body12_e79635_d_n14;
            let (assign52470_body13_e79655, assign52470_body13_e79655_d_n0, assign52470_body13_e79655_d_n2, assign52470_body13_e79655_d_n4, assign52470_body13_e79655_d_n5, assign52470_body13_e79655_d_n6, assign52470_body13_e79655_d_n7, assign52470_body13_e79655_d_n8, assign52470_body13_e79655_d_n9, assign52470_body13_e79655_d_n10, assign52470_body13_e79655_d_n11, assign52470_body13_e79655_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_flg_conv == 0.0)) {
        let assign52470_body13_e79651: f64 = (-locals.var_pf1__blk1104);
        let assign52470_body13_e79653: f64 = (assign52470_body13_e79651 / locals.var_pf11__blk1105);
        (assign52470_body13_e79653, ((((-locals.var_pf1__blk1104_dn0) * locals.var_pf11__blk1105) - (assign52470_body13_e79651 * locals.var_pf11__blk1105_dn0)) / (locals.var_pf11__blk1105 * locals.var_pf11__blk1105)), ((((-locals.var_pf1__blk1104_dn2) * locals.var_pf11__blk1105) - (assign52470_body13_e79651 * locals.var_pf11__blk1105_dn2)) / (locals.var_pf11__blk1105 * locals.var_pf11__blk1105)), ((((-locals.var_pf1__blk1104_dn4) * locals.var_pf11__blk1105) - (assign52470_body13_e79651 * locals.var_pf11__blk1105_dn4)) / (locals.var_pf11__blk1105 * locals.var_pf11__blk1105)), ((((-locals.var_pf1__blk1104_dn5) * locals.var_pf11__blk1105) - (assign52470_body13_e79651 * locals.var_pf11__blk1105_dn5)) / (locals.var_pf11__blk1105 * locals.var_pf11__blk1105)), ((((-locals.var_pf1__blk1104_dn6) * locals.var_pf11__blk1105) - (assign52470_body13_e79651 * locals.var_pf11__blk1105_dn6)) / (locals.var_pf11__blk1105 * locals.var_pf11__blk1105)), ((((-locals.var_pf1__blk1104_dn7) * locals.var_pf11__blk1105) - (assign52470_body13_e79651 * locals.var_pf11__blk1105_dn7)) / (locals.var_pf11__blk1105 * locals.var_pf11__blk1105)), ((((-locals.var_pf1__blk1104_dn8) * locals.var_pf11__blk1105) - (assign52470_body13_e79651 * locals.var_pf11__blk1105_dn8)) / (locals.var_pf11__blk1105 * locals.var_pf11__blk1105)), ((((-locals.var_pf1__blk1104_dn9) * locals.var_pf11__blk1105) - (assign52470_body13_e79651 * locals.var_pf11__blk1105_dn9)) / (locals.var_pf11__blk1105 * locals.var_pf11__blk1105)), ((((-locals.var_pf1__blk1104_dn10) * locals.var_pf11__blk1105) - (assign52470_body13_e79651 * locals.var_pf11__blk1105_dn10)) / (locals.var_pf11__blk1105 * locals.var_pf11__blk1105)), ((((-locals.var_pf1__blk1104_dn11) * locals.var_pf11__blk1105) - (assign52470_body13_e79651 * locals.var_pf11__blk1105_dn11)) / (locals.var_pf11__blk1105 * locals.var_pf11__blk1105)), ((((-locals.var_pf1__blk1104_dn14) * locals.var_pf11__blk1105) - (assign52470_body13_e79651 * locals.var_pf11__blk1105_dn14)) / (locals.var_pf11__blk1105 * locals.var_pf11__blk1105)),)
    } else {
        (locals.var_dps__blk1116, locals.var_dps__blk1116_dn0, locals.var_dps__blk1116_dn2, locals.var_dps__blk1116_dn4, locals.var_dps__blk1116_dn5, locals.var_dps__blk1116_dn6, locals.var_dps__blk1116_dn7, locals.var_dps__blk1116_dn8, locals.var_dps__blk1116_dn9, locals.var_dps__blk1116_dn10, locals.var_dps__blk1116_dn11, locals.var_dps__blk1116_dn14,)
    }
};
            locals.var_dps__blk1116 = assign52470_body13_e79655;
            locals.var_dps__blk1116_dn0 = assign52470_body13_e79655_d_n0;
            locals.var_dps__blk1116_dn2 = assign52470_body13_e79655_d_n2;
            locals.var_dps__blk1116_dn4 = assign52470_body13_e79655_d_n4;
            locals.var_dps__blk1116_dn5 = assign52470_body13_e79655_d_n5;
            locals.var_dps__blk1116_dn6 = assign52470_body13_e79655_d_n6;
            locals.var_dps__blk1116_dn7 = assign52470_body13_e79655_d_n7;
            locals.var_dps__blk1116_dn8 = assign52470_body13_e79655_d_n8;
            locals.var_dps__blk1116_dn9 = assign52470_body13_e79655_d_n9;
            locals.var_dps__blk1116_dn10 = assign52470_body13_e79655_d_n10;
            locals.var_dps__blk1116_dn11 = assign52470_body13_e79655_d_n11;
            locals.var_dps__blk1116_dn14 = assign52470_body13_e79655_d_n14;
            let assign52470_body14_e79657: f64 = (locals.var_dps__blk1116).abs();
            let assign52470_body14_e79660: f64 = (1e-10 * 100.0);
            let assign52470_body14_e79661: f64 = if assign52470_body14_e79657 < assign52470_body14_e79660 { 1.0 } else { 0.0 };
            locals.var_guard1335 = assign52470_body14_e79661;
            let (assign52470_body15_e79680,) = {
    if (((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_flg_conv == 0.0)) && (locals.var_guard1335 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_flg_conv,)
    }
};
            locals.var_flg_conv = assign52470_body15_e79680;
            let assign52470_body16_e79683: f64 = if locals.var_dps__blk1116 > 0.1 { 1.0 } else { 0.0 };
            locals.var_guard1336 = assign52470_body16_e79683;
            let (assign52470_body17_e79705, assign52470_body17_e79705_d_n0, assign52470_body17_e79705_d_n2, assign52470_body17_e79705_d_n4, assign52470_body17_e79705_d_n5, assign52470_body17_e79705_d_n6, assign52470_body17_e79705_d_n7, assign52470_body17_e79705_d_n8, assign52470_body17_e79705_d_n9, assign52470_body17_e79705_d_n10, assign52470_body17_e79705_d_n11, assign52470_body17_e79705_d_n14,) = {
    if ((((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_flg_conv == 0.0)) && (locals.var_guard1335 == 0.0)) && (locals.var_guard1336 != 0.0)) {
        (0.1, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dps__blk1116, locals.var_dps__blk1116_dn0, locals.var_dps__blk1116_dn2, locals.var_dps__blk1116_dn4, locals.var_dps__blk1116_dn5, locals.var_dps__blk1116_dn6, locals.var_dps__blk1116_dn7, locals.var_dps__blk1116_dn8, locals.var_dps__blk1116_dn9, locals.var_dps__blk1116_dn10, locals.var_dps__blk1116_dn11, locals.var_dps__blk1116_dn14,)
    }
};
            locals.var_dps__blk1116 = assign52470_body17_e79705;
            locals.var_dps__blk1116_dn0 = assign52470_body17_e79705_d_n0;
            locals.var_dps__blk1116_dn2 = assign52470_body17_e79705_d_n2;
            locals.var_dps__blk1116_dn4 = assign52470_body17_e79705_d_n4;
            locals.var_dps__blk1116_dn5 = assign52470_body17_e79705_d_n5;
            locals.var_dps__blk1116_dn6 = assign52470_body17_e79705_d_n6;
            locals.var_dps__blk1116_dn7 = assign52470_body17_e79705_d_n7;
            locals.var_dps__blk1116_dn8 = assign52470_body17_e79705_d_n8;
            locals.var_dps__blk1116_dn9 = assign52470_body17_e79705_d_n9;
            locals.var_dps__blk1116_dn10 = assign52470_body17_e79705_d_n10;
            locals.var_dps__blk1116_dn11 = assign52470_body17_e79705_d_n11;
            locals.var_dps__blk1116_dn14 = assign52470_body17_e79705_d_n14;
            let assign52470_body18_e79708: f64 = (-0.1);
            let assign52470_body18_e79709: f64 = if locals.var_dps__blk1116 < assign52470_body18_e79708 { 1.0 } else { 0.0 };
            locals.var_guard1337 = assign52470_body18_e79709;
            let (assign52470_body19_e79735, assign52470_body19_e79735_d_n0, assign52470_body19_e79735_d_n2, assign52470_body19_e79735_d_n4, assign52470_body19_e79735_d_n5, assign52470_body19_e79735_d_n6, assign52470_body19_e79735_d_n7, assign52470_body19_e79735_d_n8, assign52470_body19_e79735_d_n9, assign52470_body19_e79735_d_n10, assign52470_body19_e79735_d_n11, assign52470_body19_e79735_d_n14,) = {
    if (((((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_flg_conv == 0.0)) && (locals.var_guard1335 == 0.0)) && (locals.var_guard1336 == 0.0)) && (locals.var_guard1337 != 0.0)) {
        let assign52470_body19_e79733: f64 = (-0.1);
        (assign52470_body19_e79733, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dps__blk1116, locals.var_dps__blk1116_dn0, locals.var_dps__blk1116_dn2, locals.var_dps__blk1116_dn4, locals.var_dps__blk1116_dn5, locals.var_dps__blk1116_dn6, locals.var_dps__blk1116_dn7, locals.var_dps__blk1116_dn8, locals.var_dps__blk1116_dn9, locals.var_dps__blk1116_dn10, locals.var_dps__blk1116_dn11, locals.var_dps__blk1116_dn14,)
    }
};
            locals.var_dps__blk1116 = assign52470_body19_e79735;
            locals.var_dps__blk1116_dn0 = assign52470_body19_e79735_d_n0;
            locals.var_dps__blk1116_dn2 = assign52470_body19_e79735_d_n2;
            locals.var_dps__blk1116_dn4 = assign52470_body19_e79735_d_n4;
            locals.var_dps__blk1116_dn5 = assign52470_body19_e79735_d_n5;
            locals.var_dps__blk1116_dn6 = assign52470_body19_e79735_d_n6;
            locals.var_dps__blk1116_dn7 = assign52470_body19_e79735_d_n7;
            locals.var_dps__blk1116_dn8 = assign52470_body19_e79735_d_n8;
            locals.var_dps__blk1116_dn9 = assign52470_body19_e79735_d_n9;
            locals.var_dps__blk1116_dn10 = assign52470_body19_e79735_d_n10;
            locals.var_dps__blk1116_dn11 = assign52470_body19_e79735_d_n11;
            locals.var_dps__blk1116_dn14 = assign52470_body19_e79735_d_n14;
            let (assign52470_body20_e79754, assign52470_body20_e79754_d_n0, assign52470_body20_e79754_d_n2, assign52470_body20_e79754_d_n4, assign52470_body20_e79754_d_n5, assign52470_body20_e79754_d_n6, assign52470_body20_e79754_d_n7, assign52470_body20_e79754_d_n8, assign52470_body20_e79754_d_n9, assign52470_body20_e79754_d_n10, assign52470_body20_e79754_d_n11, assign52470_body20_e79754_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_flg_conv == 0.0)) {
        let assign52470_body20_e79752: f64 = (locals.var_ps0dep + locals.var_dps__blk1116);
        (assign52470_body20_e79752, (locals.var_ps0dep_dn0 + locals.var_dps__blk1116_dn0), (locals.var_ps0dep_dn2 + locals.var_dps__blk1116_dn2), (locals.var_ps0dep_dn4 + locals.var_dps__blk1116_dn4), (locals.var_ps0dep_dn5 + locals.var_dps__blk1116_dn5), (locals.var_ps0dep_dn6 + locals.var_dps__blk1116_dn6), (locals.var_ps0dep_dn7 + locals.var_dps__blk1116_dn7), (locals.var_ps0dep_dn8 + locals.var_dps__blk1116_dn8), (locals.var_ps0dep_dn9 + locals.var_dps__blk1116_dn9), (locals.var_ps0dep_dn10 + locals.var_dps__blk1116_dn10), (locals.var_ps0dep_dn11 + locals.var_dps__blk1116_dn11), (locals.var_ps0dep_dn14 + locals.var_dps__blk1116_dn14),)
    } else {
        (locals.var_ps0dep, locals.var_ps0dep_dn0, locals.var_ps0dep_dn2, locals.var_ps0dep_dn4, locals.var_ps0dep_dn5, locals.var_ps0dep_dn6, locals.var_ps0dep_dn7, locals.var_ps0dep_dn8, locals.var_ps0dep_dn9, locals.var_ps0dep_dn10, locals.var_ps0dep_dn11, locals.var_ps0dep_dn14,)
    }
};
            locals.var_ps0dep = assign52470_body20_e79754;
            locals.var_ps0dep_dn0 = assign52470_body20_e79754_d_n0;
            locals.var_ps0dep_dn2 = assign52470_body20_e79754_d_n2;
            locals.var_ps0dep_dn4 = assign52470_body20_e79754_d_n4;
            locals.var_ps0dep_dn5 = assign52470_body20_e79754_d_n5;
            locals.var_ps0dep_dn6 = assign52470_body20_e79754_d_n6;
            locals.var_ps0dep_dn7 = assign52470_body20_e79754_d_n7;
            locals.var_ps0dep_dn8 = assign52470_body20_e79754_d_n8;
            locals.var_ps0dep_dn9 = assign52470_body20_e79754_d_n9;
            locals.var_ps0dep_dn10 = assign52470_body20_e79754_d_n10;
            locals.var_ps0dep_dn11 = assign52470_body20_e79754_d_n11;
            locals.var_ps0dep_dn14 = assign52470_body20_e79754_d_n14;
            let (assign52470_body21_e79770,) = {
    if (((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) {
        let assign52470_body21_e79768: f64 = (locals.var_lp_s0 + 1.0);
        (assign52470_body21_e79768,)
    } else {
        (locals.var_lp_s0,)
    }
};
            locals.var_lp_s0 = assign52470_body21_e79770;
        }

        let assign52490_e79776: f64 = if 1.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1339 = assign52490_e79776;

        let (assign52500_e79792, assign52500_e79792_d_n0, assign52500_e79792_d_n2, assign52500_e79792_d_n4, assign52500_e79792_d_n5, assign52500_e79792_d_n6, assign52500_e79792_d_n7, assign52500_e79792_d_n8, assign52500_e79792_d_n9, assign52500_e79792_d_n10, assign52500_e79792_d_n11, assign52500_e79792_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_guard1339 != 0.0)) {
        (locals.var_ps0dep, locals.var_ps0dep_dn0, locals.var_ps0dep_dn2, locals.var_ps0dep_dn4, locals.var_ps0dep_dn5, locals.var_ps0dep_dn6, locals.var_ps0dep_dn7, locals.var_ps0dep_dn8, locals.var_ps0dep_dn9, locals.var_ps0dep_dn10, locals.var_ps0dep_dn11, locals.var_ps0dep_dn14,)
    } else {
        (locals.var_ps0dep0, locals.var_ps0dep0_dn0, locals.var_ps0dep0_dn2, locals.var_ps0dep0_dn4, locals.var_ps0dep0_dn5, locals.var_ps0dep0_dn6, locals.var_ps0dep0_dn7, locals.var_ps0dep0_dn8, locals.var_ps0dep0_dn9, locals.var_ps0dep0_dn10, locals.var_ps0dep0_dn11, locals.var_ps0dep0_dn14,)
    }
};
        locals.var_ps0dep0 = assign52500_e79792;
        locals.var_ps0dep0_dn0 = assign52500_e79792_d_n0;
        locals.var_ps0dep0_dn2 = assign52500_e79792_d_n2;
        locals.var_ps0dep0_dn4 = assign52500_e79792_d_n4;
        locals.var_ps0dep0_dn5 = assign52500_e79792_d_n5;
        locals.var_ps0dep0_dn6 = assign52500_e79792_d_n6;
        locals.var_ps0dep0_dn7 = assign52500_e79792_d_n7;
        locals.var_ps0dep0_dn8 = assign52500_e79792_d_n8;
        locals.var_ps0dep0_dn9 = assign52500_e79792_d_n9;
        locals.var_ps0dep0_dn10 = assign52500_e79792_d_n10;
        locals.var_ps0dep0_dn11 = assign52500_e79792_d_n11;
        locals.var_ps0dep0_dn14 = assign52500_e79792_d_n14;

        let assign52510_e79796: f64 = (locals.var_ps0dep0 + 0.2);
        let assign52510_e79801: f64 = if ((locals.var_ps0dep < assign52510_e79796) && (0.2 >= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1340 = assign52510_e79801;

        let (assign52520_e79824, assign52520_e79824_d_n0, assign52520_e79824_d_n2, assign52520_e79824_d_n4, assign52520_e79824_d_n5, assign52520_e79824_d_n6, assign52520_e79824_d_n7, assign52520_e79824_d_n8, assign52520_e79824_d_n9, assign52520_e79824_d_n10, assign52520_e79824_d_n11, assign52520_e79824_d_n14,) = {
    if (((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_guard1339 == 0.0)) && (locals.var_guard1340 != 0.0)) {
        let assign52520_e79820: f64 = (locals.var_ps0dep0 + 0.2);
        let assign52520_e79822: f64 = (assign52520_e79820 - locals.var_ps0dep);
        (assign52520_e79822, (locals.var_ps0dep0_dn0 - locals.var_ps0dep_dn0), (locals.var_ps0dep0_dn2 - locals.var_ps0dep_dn2), (locals.var_ps0dep0_dn4 - locals.var_ps0dep_dn4), (locals.var_ps0dep0_dn5 - locals.var_ps0dep_dn5), (locals.var_ps0dep0_dn6 - locals.var_ps0dep_dn6), (locals.var_ps0dep0_dn7 - locals.var_ps0dep_dn7), (locals.var_ps0dep0_dn8 - locals.var_ps0dep_dn8), (locals.var_ps0dep0_dn9 - locals.var_ps0dep_dn9), (locals.var_ps0dep0_dn10 - locals.var_ps0dep_dn10), (locals.var_ps0dep0_dn11 - locals.var_ps0dep_dn11), (locals.var_ps0dep0_dn14 - locals.var_ps0dep_dn14),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign52520_e79824;
        locals.var_tmf1_dn0 = assign52520_e79824_d_n0;
        locals.var_tmf1_dn2 = assign52520_e79824_d_n2;
        locals.var_tmf1_dn4 = assign52520_e79824_d_n4;
        locals.var_tmf1_dn5 = assign52520_e79824_d_n5;
        locals.var_tmf1_dn6 = assign52520_e79824_d_n6;
        locals.var_tmf1_dn7 = assign52520_e79824_d_n7;
        locals.var_tmf1_dn8 = assign52520_e79824_d_n8;
        locals.var_tmf1_dn9 = assign52520_e79824_d_n9;
        locals.var_tmf1_dn10 = assign52520_e79824_d_n10;
        locals.var_tmf1_dn11 = assign52520_e79824_d_n11;
        locals.var_tmf1_dn14 = assign52520_e79824_d_n14;

        let (assign52530_e79845, assign52530_e79845_d_n0, assign52530_e79845_d_n2, assign52530_e79845_d_n4, assign52530_e79845_d_n5, assign52530_e79845_d_n6, assign52530_e79845_d_n7, assign52530_e79845_d_n8, assign52530_e79845_d_n9, assign52530_e79845_d_n10, assign52530_e79845_d_n11, assign52530_e79845_d_n14,) = {
    if (((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_guard1339 == 0.0)) && (locals.var_guard1340 != 0.0)) {
        let assign52530_e79843: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign52530_e79843, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)), ((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)), ((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)), ((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn4, locals.var_x2_dn5, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn8, locals.var_x2_dn9, locals.var_x2_dn10, locals.var_x2_dn11, locals.var_x2_dn14,)
    }
};
        locals.var_x2 = assign52530_e79845;
        locals.var_x2_dn0 = assign52530_e79845_d_n0;
        locals.var_x2_dn2 = assign52530_e79845_d_n2;
        locals.var_x2_dn4 = assign52530_e79845_d_n4;
        locals.var_x2_dn5 = assign52530_e79845_d_n5;
        locals.var_x2_dn6 = assign52530_e79845_d_n6;
        locals.var_x2_dn7 = assign52530_e79845_d_n7;
        locals.var_x2_dn8 = assign52530_e79845_d_n8;
        locals.var_x2_dn9 = assign52530_e79845_d_n9;
        locals.var_x2_dn10 = assign52530_e79845_d_n10;
        locals.var_x2_dn11 = assign52530_e79845_d_n11;
        locals.var_x2_dn14 = assign52530_e79845_d_n14;

        let (assign52540_e79866, assign52540_e79866_d_n0, assign52540_e79866_d_n2, assign52540_e79866_d_n4, assign52540_e79866_d_n5, assign52540_e79866_d_n6, assign52540_e79866_d_n7, assign52540_e79866_d_n8, assign52540_e79866_d_n9, assign52540_e79866_d_n10, assign52540_e79866_d_n11, assign52540_e79866_d_n14,) = {
    if (((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_guard1339 == 0.0)) && (locals.var_guard1340 != 0.0)) {
        let assign52540_e79864: f64 = (0.2 * 0.2);
        (assign52540_e79864, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn4, locals.var_xmax2_dn5, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn8, locals.var_xmax2_dn9, locals.var_xmax2_dn10, locals.var_xmax2_dn11, locals.var_xmax2_dn14,)
    }
};
        locals.var_xmax2 = assign52540_e79866;
        locals.var_xmax2_dn0 = assign52540_e79866_d_n0;
        locals.var_xmax2_dn2 = assign52540_e79866_d_n2;
        locals.var_xmax2_dn4 = assign52540_e79866_d_n4;
        locals.var_xmax2_dn5 = assign52540_e79866_d_n5;
        locals.var_xmax2_dn6 = assign52540_e79866_d_n6;
        locals.var_xmax2_dn7 = assign52540_e79866_d_n7;
        locals.var_xmax2_dn8 = assign52540_e79866_d_n8;
        locals.var_xmax2_dn9 = assign52540_e79866_d_n9;
        locals.var_xmax2_dn10 = assign52540_e79866_d_n10;
        locals.var_xmax2_dn11 = assign52540_e79866_d_n11;
        locals.var_xmax2_dn14 = assign52540_e79866_d_n14;

        let (assign52550_e79885, assign52550_e79885_d_n0, assign52550_e79885_d_n2, assign52550_e79885_d_n4, assign52550_e79885_d_n5, assign52550_e79885_d_n6, assign52550_e79885_d_n7, assign52550_e79885_d_n8, assign52550_e79885_d_n9, assign52550_e79885_d_n10, assign52550_e79885_d_n11, assign52550_e79885_d_n14,) = {
    if (((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_guard1339 == 0.0)) && (locals.var_guard1340 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign52550_e79885;
        locals.var_xp_dn0 = assign52550_e79885_d_n0;
        locals.var_xp_dn2 = assign52550_e79885_d_n2;
        locals.var_xp_dn4 = assign52550_e79885_d_n4;
        locals.var_xp_dn5 = assign52550_e79885_d_n5;
        locals.var_xp_dn6 = assign52550_e79885_d_n6;
        locals.var_xp_dn7 = assign52550_e79885_d_n7;
        locals.var_xp_dn8 = assign52550_e79885_d_n8;
        locals.var_xp_dn9 = assign52550_e79885_d_n9;
        locals.var_xp_dn10 = assign52550_e79885_d_n10;
        locals.var_xp_dn11 = assign52550_e79885_d_n11;
        locals.var_xp_dn14 = assign52550_e79885_d_n14;

        let (assign52560_e79904, assign52560_e79904_d_n0, assign52560_e79904_d_n2, assign52560_e79904_d_n4, assign52560_e79904_d_n5, assign52560_e79904_d_n6, assign52560_e79904_d_n7, assign52560_e79904_d_n8, assign52560_e79904_d_n9, assign52560_e79904_d_n10, assign52560_e79904_d_n11, assign52560_e79904_d_n14,) = {
    if (((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_guard1339 == 0.0)) && (locals.var_guard1340 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign52560_e79904;
        locals.var_xmp_dn0 = assign52560_e79904_d_n0;
        locals.var_xmp_dn2 = assign52560_e79904_d_n2;
        locals.var_xmp_dn4 = assign52560_e79904_d_n4;
        locals.var_xmp_dn5 = assign52560_e79904_d_n5;
        locals.var_xmp_dn6 = assign52560_e79904_d_n6;
        locals.var_xmp_dn7 = assign52560_e79904_d_n7;
        locals.var_xmp_dn8 = assign52560_e79904_d_n8;
        locals.var_xmp_dn9 = assign52560_e79904_d_n9;
        locals.var_xmp_dn10 = assign52560_e79904_d_n10;
        locals.var_xmp_dn11 = assign52560_e79904_d_n11;
        locals.var_xmp_dn14 = assign52560_e79904_d_n14;

    }

    pub(super) fn stamp_transient_block_180(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign52570_e79923,) = {
    if (((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_guard1339 == 0.0)) && (locals.var_guard1340 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign52570_e79923;

        let (assign52580_e79942,) = {
    if (((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_guard1339 == 0.0)) && (locals.var_guard1340 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign52580_e79942;

        let (assign52590_e79961, assign52590_e79961_d_n0, assign52590_e79961_d_n2, assign52590_e79961_d_n4, assign52590_e79961_d_n5, assign52590_e79961_d_n6, assign52590_e79961_d_n7, assign52590_e79961_d_n8, assign52590_e79961_d_n9, assign52590_e79961_d_n10, assign52590_e79961_d_n11, assign52590_e79961_d_n14,) = {
    if (((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_guard1339 == 0.0)) && (locals.var_guard1340 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign52590_e79961;
        locals.var_arg_dn0 = assign52590_e79961_d_n0;
        locals.var_arg_dn2 = assign52590_e79961_d_n2;
        locals.var_arg_dn4 = assign52590_e79961_d_n4;
        locals.var_arg_dn5 = assign52590_e79961_d_n5;
        locals.var_arg_dn6 = assign52590_e79961_d_n6;
        locals.var_arg_dn7 = assign52590_e79961_d_n7;
        locals.var_arg_dn8 = assign52590_e79961_d_n8;
        locals.var_arg_dn9 = assign52590_e79961_d_n9;
        locals.var_arg_dn10 = assign52590_e79961_d_n10;
        locals.var_arg_dn11 = assign52590_e79961_d_n11;
        locals.var_arg_dn14 = assign52590_e79961_d_n14;

        let (assign52600_e79980, assign52600_e79980_d_n0, assign52600_e79980_d_n2, assign52600_e79980_d_n4, assign52600_e79980_d_n5, assign52600_e79980_d_n6, assign52600_e79980_d_n7, assign52600_e79980_d_n8, assign52600_e79980_d_n9, assign52600_e79980_d_n10, assign52600_e79980_d_n11, assign52600_e79980_d_n14,) = {
    if (((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_guard1339 == 0.0)) && (locals.var_guard1340 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign52600_e79980;
        locals.var_dnm_dn0 = assign52600_e79980_d_n0;
        locals.var_dnm_dn2 = assign52600_e79980_d_n2;
        locals.var_dnm_dn4 = assign52600_e79980_d_n4;
        locals.var_dnm_dn5 = assign52600_e79980_d_n5;
        locals.var_dnm_dn6 = assign52600_e79980_d_n6;
        locals.var_dnm_dn7 = assign52600_e79980_d_n7;
        locals.var_dnm_dn8 = assign52600_e79980_d_n8;
        locals.var_dnm_dn9 = assign52600_e79980_d_n9;
        locals.var_dnm_dn10 = assign52600_e79980_d_n10;
        locals.var_dnm_dn11 = assign52600_e79980_d_n11;
        locals.var_dnm_dn14 = assign52600_e79980_d_n14;

        let (assign52610_e80001, assign52610_e80001_d_n0, assign52610_e80001_d_n2, assign52610_e80001_d_n4, assign52610_e80001_d_n5, assign52610_e80001_d_n6, assign52610_e80001_d_n7, assign52610_e80001_d_n8, assign52610_e80001_d_n9, assign52610_e80001_d_n10, assign52610_e80001_d_n11, assign52610_e80001_d_n14,) = {
    if (((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_guard1339 == 0.0)) && (locals.var_guard1340 != 0.0)) {
        let assign52610_e79999: f64 = (locals.var_xp * locals.var_x2);
        (assign52610_e79999, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign52610_e80001;
        locals.var_xp_dn0 = assign52610_e80001_d_n0;
        locals.var_xp_dn2 = assign52610_e80001_d_n2;
        locals.var_xp_dn4 = assign52610_e80001_d_n4;
        locals.var_xp_dn5 = assign52610_e80001_d_n5;
        locals.var_xp_dn6 = assign52610_e80001_d_n6;
        locals.var_xp_dn7 = assign52610_e80001_d_n7;
        locals.var_xp_dn8 = assign52610_e80001_d_n8;
        locals.var_xp_dn9 = assign52610_e80001_d_n9;
        locals.var_xp_dn10 = assign52610_e80001_d_n10;
        locals.var_xp_dn11 = assign52610_e80001_d_n11;
        locals.var_xp_dn14 = assign52610_e80001_d_n14;

        let (assign52620_e80022, assign52620_e80022_d_n0, assign52620_e80022_d_n2, assign52620_e80022_d_n4, assign52620_e80022_d_n5, assign52620_e80022_d_n6, assign52620_e80022_d_n7, assign52620_e80022_d_n8, assign52620_e80022_d_n9, assign52620_e80022_d_n10, assign52620_e80022_d_n11, assign52620_e80022_d_n14,) = {
    if (((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_guard1339 == 0.0)) && (locals.var_guard1340 != 0.0)) {
        let assign52620_e80020: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign52620_e80020, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign52620_e80022;
        locals.var_xmp_dn0 = assign52620_e80022_d_n0;
        locals.var_xmp_dn2 = assign52620_e80022_d_n2;
        locals.var_xmp_dn4 = assign52620_e80022_d_n4;
        locals.var_xmp_dn5 = assign52620_e80022_d_n5;
        locals.var_xmp_dn6 = assign52620_e80022_d_n6;
        locals.var_xmp_dn7 = assign52620_e80022_d_n7;
        locals.var_xmp_dn8 = assign52620_e80022_d_n8;
        locals.var_xmp_dn9 = assign52620_e80022_d_n9;
        locals.var_xmp_dn10 = assign52620_e80022_d_n10;
        locals.var_xmp_dn11 = assign52620_e80022_d_n11;
        locals.var_xmp_dn14 = assign52620_e80022_d_n14;

        let (assign52630_e80043, assign52630_e80043_d_n0, assign52630_e80043_d_n2, assign52630_e80043_d_n4, assign52630_e80043_d_n5, assign52630_e80043_d_n6, assign52630_e80043_d_n7, assign52630_e80043_d_n8, assign52630_e80043_d_n9, assign52630_e80043_d_n10, assign52630_e80043_d_n11, assign52630_e80043_d_n14,) = {
    if (((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_guard1339 == 0.0)) && (locals.var_guard1340 != 0.0)) {
        let assign52630_e80041: f64 = (locals.var_xp * locals.var_x2);
        (assign52630_e80041, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign52630_e80043;
        locals.var_xp_dn0 = assign52630_e80043_d_n0;
        locals.var_xp_dn2 = assign52630_e80043_d_n2;
        locals.var_xp_dn4 = assign52630_e80043_d_n4;
        locals.var_xp_dn5 = assign52630_e80043_d_n5;
        locals.var_xp_dn6 = assign52630_e80043_d_n6;
        locals.var_xp_dn7 = assign52630_e80043_d_n7;
        locals.var_xp_dn8 = assign52630_e80043_d_n8;
        locals.var_xp_dn9 = assign52630_e80043_d_n9;
        locals.var_xp_dn10 = assign52630_e80043_d_n10;
        locals.var_xp_dn11 = assign52630_e80043_d_n11;
        locals.var_xp_dn14 = assign52630_e80043_d_n14;

        let (assign52640_e80064, assign52640_e80064_d_n0, assign52640_e80064_d_n2, assign52640_e80064_d_n4, assign52640_e80064_d_n5, assign52640_e80064_d_n6, assign52640_e80064_d_n7, assign52640_e80064_d_n8, assign52640_e80064_d_n9, assign52640_e80064_d_n10, assign52640_e80064_d_n11, assign52640_e80064_d_n14,) = {
    if (((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_guard1339 == 0.0)) && (locals.var_guard1340 != 0.0)) {
        let assign52640_e80062: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign52640_e80062, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign52640_e80064;
        locals.var_xmp_dn0 = assign52640_e80064_d_n0;
        locals.var_xmp_dn2 = assign52640_e80064_d_n2;
        locals.var_xmp_dn4 = assign52640_e80064_d_n4;
        locals.var_xmp_dn5 = assign52640_e80064_d_n5;
        locals.var_xmp_dn6 = assign52640_e80064_d_n6;
        locals.var_xmp_dn7 = assign52640_e80064_d_n7;
        locals.var_xmp_dn8 = assign52640_e80064_d_n8;
        locals.var_xmp_dn9 = assign52640_e80064_d_n9;
        locals.var_xmp_dn10 = assign52640_e80064_d_n10;
        locals.var_xmp_dn11 = assign52640_e80064_d_n11;
        locals.var_xmp_dn14 = assign52640_e80064_d_n14;

        let (assign52650_e80085, assign52650_e80085_d_n0, assign52650_e80085_d_n2, assign52650_e80085_d_n4, assign52650_e80085_d_n5, assign52650_e80085_d_n6, assign52650_e80085_d_n7, assign52650_e80085_d_n8, assign52650_e80085_d_n9, assign52650_e80085_d_n10, assign52650_e80085_d_n11, assign52650_e80085_d_n14,) = {
    if (((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_guard1339 == 0.0)) && (locals.var_guard1340 != 0.0)) {
        let assign52650_e80083: f64 = (locals.var_xp + locals.var_xmp);
        (assign52650_e80083, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn4 + locals.var_xmp_dn4), (locals.var_xp_dn5 + locals.var_xmp_dn5), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn8 + locals.var_xmp_dn8), (locals.var_xp_dn9 + locals.var_xmp_dn9), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn11 + locals.var_xmp_dn11), (locals.var_xp_dn14 + locals.var_xmp_dn14),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign52650_e80085;
        locals.var_arg_dn0 = assign52650_e80085_d_n0;
        locals.var_arg_dn2 = assign52650_e80085_d_n2;
        locals.var_arg_dn4 = assign52650_e80085_d_n4;
        locals.var_arg_dn5 = assign52650_e80085_d_n5;
        locals.var_arg_dn6 = assign52650_e80085_d_n6;
        locals.var_arg_dn7 = assign52650_e80085_d_n7;
        locals.var_arg_dn8 = assign52650_e80085_d_n8;
        locals.var_arg_dn9 = assign52650_e80085_d_n9;
        locals.var_arg_dn10 = assign52650_e80085_d_n10;
        locals.var_arg_dn11 = assign52650_e80085_d_n11;
        locals.var_arg_dn14 = assign52650_e80085_d_n14;

        let (assign52660_e80104, assign52660_e80104_d_n0, assign52660_e80104_d_n2, assign52660_e80104_d_n4, assign52660_e80104_d_n5, assign52660_e80104_d_n6, assign52660_e80104_d_n7, assign52660_e80104_d_n8, assign52660_e80104_d_n9, assign52660_e80104_d_n10, assign52660_e80104_d_n11, assign52660_e80104_d_n14,) = {
    if (((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_guard1339 == 0.0)) && (locals.var_guard1340 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign52660_e80104;
        locals.var_dnm_dn0 = assign52660_e80104_d_n0;
        locals.var_dnm_dn2 = assign52660_e80104_d_n2;
        locals.var_dnm_dn4 = assign52660_e80104_d_n4;
        locals.var_dnm_dn5 = assign52660_e80104_d_n5;
        locals.var_dnm_dn6 = assign52660_e80104_d_n6;
        locals.var_dnm_dn7 = assign52660_e80104_d_n7;
        locals.var_dnm_dn8 = assign52660_e80104_d_n8;
        locals.var_dnm_dn9 = assign52660_e80104_d_n9;
        locals.var_dnm_dn10 = assign52660_e80104_d_n10;
        locals.var_dnm_dn11 = assign52660_e80104_d_n11;
        locals.var_dnm_dn14 = assign52660_e80104_d_n14;

        let assign52670_e80119: f64 = if ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard1341 = assign52670_e80119;

        let assign52680_e80122: f64 = if 2.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1342 = assign52680_e80122;

        let (assign52690_e80145,) = {
    if (((((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_guard1339 == 0.0)) && (locals.var_guard1340 != 0.0)) && (locals.var_guard1341 != 0.0)) && (locals.var_guard1342 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign52690_e80145;

        let assign52700_e80148: f64 = if 2.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard1343 = assign52700_e80148;

        let (assign52710_e80174,) = {
    if ((((((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_guard1339 == 0.0)) && (locals.var_guard1340 != 0.0)) && (locals.var_guard1341 != 0.0)) && (locals.var_guard1342 == 0.0)) && (locals.var_guard1343 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign52710_e80174;

        let assign52720_e80177: f64 = if 2.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard1344 = assign52720_e80177;

        let (assign52730_e80206,) = {
    if (((((((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_guard1339 == 0.0)) && (locals.var_guard1340 != 0.0)) && (locals.var_guard1341 != 0.0)) && (locals.var_guard1342 == 0.0)) && (locals.var_guard1343 == 0.0)) && (locals.var_guard1344 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign52730_e80206;

        let assign52740_e80209: f64 = if 2.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard1345 = assign52740_e80209;

        let (assign52750_e80241,) = {
    if ((((((((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_guard1339 == 0.0)) && (locals.var_guard1340 != 0.0)) && (locals.var_guard1341 != 0.0)) && (locals.var_guard1342 == 0.0)) && (locals.var_guard1343 == 0.0)) && (locals.var_guard1344 == 0.0)) && (locals.var_guard1345 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign52750_e80241;

        let (assign52760_e80262,) = {
    if ((((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_guard1339 == 0.0)) && (locals.var_guard1340 != 0.0)) && (locals.var_guard1341 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign52760_e80262;

        let mut assign52770_loop_guard: usize = 0;
        while {
            let assign52770_cond_e80284: f64 = if (((((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_guard1339 == 0.0)) && (locals.var_guard1340 != 0.0)) && (locals.var_guard1341 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign52770_cond_e80284 != 0.0
        } {
            assign52770_loop_guard += 1;
            assert!(assign52770_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign52770_body0_e80306, assign52770_body0_e80306_d_n0, assign52770_body0_e80306_d_n2, assign52770_body0_e80306_d_n4, assign52770_body0_e80306_d_n5, assign52770_body0_e80306_d_n6, assign52770_body0_e80306_d_n7, assign52770_body0_e80306_d_n8, assign52770_body0_e80306_d_n9, assign52770_body0_e80306_d_n10, assign52770_body0_e80306_d_n11, assign52770_body0_e80306_d_n14,) = {
    if ((((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_guard1339 == 0.0)) && (locals.var_guard1340 != 0.0)) && (locals.var_guard1341 != 0.0)) {
        let assign52770_body0_e80304: f64 = (locals.var_dnm).sqrt();
        (assign52770_body0_e80304, (locals.var_dnm_dn0 / (2.0 * assign52770_body0_e80304)), (locals.var_dnm_dn2 / (2.0 * assign52770_body0_e80304)), (locals.var_dnm_dn4 / (2.0 * assign52770_body0_e80304)), (locals.var_dnm_dn5 / (2.0 * assign52770_body0_e80304)), (locals.var_dnm_dn6 / (2.0 * assign52770_body0_e80304)), (locals.var_dnm_dn7 / (2.0 * assign52770_body0_e80304)), (locals.var_dnm_dn8 / (2.0 * assign52770_body0_e80304)), (locals.var_dnm_dn9 / (2.0 * assign52770_body0_e80304)), (locals.var_dnm_dn10 / (2.0 * assign52770_body0_e80304)), (locals.var_dnm_dn11 / (2.0 * assign52770_body0_e80304)), (locals.var_dnm_dn14 / (2.0 * assign52770_body0_e80304)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
            locals.var_dnm = assign52770_body0_e80306;
            locals.var_dnm_dn0 = assign52770_body0_e80306_d_n0;
            locals.var_dnm_dn2 = assign52770_body0_e80306_d_n2;
            locals.var_dnm_dn4 = assign52770_body0_e80306_d_n4;
            locals.var_dnm_dn5 = assign52770_body0_e80306_d_n5;
            locals.var_dnm_dn6 = assign52770_body0_e80306_d_n6;
            locals.var_dnm_dn7 = assign52770_body0_e80306_d_n7;
            locals.var_dnm_dn8 = assign52770_body0_e80306_d_n8;
            locals.var_dnm_dn9 = assign52770_body0_e80306_d_n9;
            locals.var_dnm_dn10 = assign52770_body0_e80306_d_n10;
            locals.var_dnm_dn11 = assign52770_body0_e80306_d_n11;
            locals.var_dnm_dn14 = assign52770_body0_e80306_d_n14;
            let (assign52770_body1_e80329,) = {
    if ((((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_guard1339 == 0.0)) && (locals.var_guard1340 != 0.0)) && (locals.var_guard1341 != 0.0)) {
        let assign52770_body1_e80327: f64 = (locals.var_m0 + 1.0);
        (assign52770_body1_e80327,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign52770_body1_e80329;
        }

        let (assign52780_e80362, assign52780_e80362_d_n0, assign52780_e80362_d_n2, assign52780_e80362_d_n4, assign52780_e80362_d_n5, assign52780_e80362_d_n6, assign52780_e80362_d_n7, assign52780_e80362_d_n8, assign52780_e80362_d_n9, assign52780_e80362_d_n10, assign52780_e80362_d_n11, assign52780_e80362_d_n14,) = {
    if ((((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_guard1339 == 0.0)) && (locals.var_guard1340 != 0.0)) && (locals.var_guard1341 == 0.0)) {
        let (assign52780_e80360, assign52780_e80360_d_n0, assign52780_e80360_d_n2, assign52780_e80360_d_n4, assign52780_e80360_d_n5, assign52780_e80360_d_n6, assign52780_e80360_d_n7, assign52780_e80360_d_n8, assign52780_e80360_d_n9, assign52780_e80360_d_n10, assign52780_e80360_d_n11, assign52780_e80360_d_n14,) = {
            if (locals.var_dnm == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign52780_e80357: f64 = (2.0 * 2.0);
                let assign52780_e80358: f64 = (1.0 / assign52780_e80357);
                let assign52780_e80359: f64 = (locals.var_dnm).powf(assign52780_e80358);
                (assign52780_e80359, if 0.0 == 0.0 && ((assign52780_e80358) as f64).is_finite() && ((assign52780_e80358) as f64).fract() == 0.0 { if assign52780_e80358 == 0.0 { 0.0 } else { (assign52780_e80358 * ((locals.var_dnm).powf(assign52780_e80358 - 1.0) * locals.var_dnm_dn0)) } } else { (assign52780_e80359 * (assign52780_e80358 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign52780_e80358) as f64).is_finite() && ((assign52780_e80358) as f64).fract() == 0.0 { if assign52780_e80358 == 0.0 { 0.0 } else { (assign52780_e80358 * ((locals.var_dnm).powf(assign52780_e80358 - 1.0) * locals.var_dnm_dn2)) } } else { (assign52780_e80359 * (assign52780_e80358 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign52780_e80358) as f64).is_finite() && ((assign52780_e80358) as f64).fract() == 0.0 { if assign52780_e80358 == 0.0 { 0.0 } else { (assign52780_e80358 * ((locals.var_dnm).powf(assign52780_e80358 - 1.0) * locals.var_dnm_dn4)) } } else { (assign52780_e80359 * (assign52780_e80358 * (locals.var_dnm_dn4 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign52780_e80358) as f64).is_finite() && ((assign52780_e80358) as f64).fract() == 0.0 { if assign52780_e80358 == 0.0 { 0.0 } else { (assign52780_e80358 * ((locals.var_dnm).powf(assign52780_e80358 - 1.0) * locals.var_dnm_dn5)) } } else { (assign52780_e80359 * (assign52780_e80358 * (locals.var_dnm_dn5 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign52780_e80358) as f64).is_finite() && ((assign52780_e80358) as f64).fract() == 0.0 { if assign52780_e80358 == 0.0 { 0.0 } else { (assign52780_e80358 * ((locals.var_dnm).powf(assign52780_e80358 - 1.0) * locals.var_dnm_dn6)) } } else { (assign52780_e80359 * (assign52780_e80358 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign52780_e80358) as f64).is_finite() && ((assign52780_e80358) as f64).fract() == 0.0 { if assign52780_e80358 == 0.0 { 0.0 } else { (assign52780_e80358 * ((locals.var_dnm).powf(assign52780_e80358 - 1.0) * locals.var_dnm_dn7)) } } else { (assign52780_e80359 * (assign52780_e80358 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign52780_e80358) as f64).is_finite() && ((assign52780_e80358) as f64).fract() == 0.0 { if assign52780_e80358 == 0.0 { 0.0 } else { (assign52780_e80358 * ((locals.var_dnm).powf(assign52780_e80358 - 1.0) * locals.var_dnm_dn8)) } } else { (assign52780_e80359 * (assign52780_e80358 * (locals.var_dnm_dn8 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign52780_e80358) as f64).is_finite() && ((assign52780_e80358) as f64).fract() == 0.0 { if assign52780_e80358 == 0.0 { 0.0 } else { (assign52780_e80358 * ((locals.var_dnm).powf(assign52780_e80358 - 1.0) * locals.var_dnm_dn9)) } } else { (assign52780_e80359 * (assign52780_e80358 * (locals.var_dnm_dn9 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign52780_e80358) as f64).is_finite() && ((assign52780_e80358) as f64).fract() == 0.0 { if assign52780_e80358 == 0.0 { 0.0 } else { (assign52780_e80358 * ((locals.var_dnm).powf(assign52780_e80358 - 1.0) * locals.var_dnm_dn10)) } } else { (assign52780_e80359 * (assign52780_e80358 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign52780_e80358) as f64).is_finite() && ((assign52780_e80358) as f64).fract() == 0.0 { if assign52780_e80358 == 0.0 { 0.0 } else { (assign52780_e80358 * ((locals.var_dnm).powf(assign52780_e80358 - 1.0) * locals.var_dnm_dn11)) } } else { (assign52780_e80359 * (assign52780_e80358 * (locals.var_dnm_dn11 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign52780_e80358) as f64).is_finite() && ((assign52780_e80358) as f64).fract() == 0.0 { if assign52780_e80358 == 0.0 { 0.0 } else { (assign52780_e80358 * ((locals.var_dnm).powf(assign52780_e80358 - 1.0) * locals.var_dnm_dn14)) } } else { (assign52780_e80359 * (assign52780_e80358 * (locals.var_dnm_dn14 / locals.var_dnm))) },)
            }
        };
        (assign52780_e80360, assign52780_e80360_d_n0, assign52780_e80360_d_n2, assign52780_e80360_d_n4, assign52780_e80360_d_n5, assign52780_e80360_d_n6, assign52780_e80360_d_n7, assign52780_e80360_d_n8, assign52780_e80360_d_n9, assign52780_e80360_d_n10, assign52780_e80360_d_n11, assign52780_e80360_d_n14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign52780_e80362;
        locals.var_dnm_dn0 = assign52780_e80362_d_n0;
        locals.var_dnm_dn2 = assign52780_e80362_d_n2;
        locals.var_dnm_dn4 = assign52780_e80362_d_n4;
        locals.var_dnm_dn5 = assign52780_e80362_d_n5;
        locals.var_dnm_dn6 = assign52780_e80362_d_n6;
        locals.var_dnm_dn7 = assign52780_e80362_d_n7;
        locals.var_dnm_dn8 = assign52780_e80362_d_n8;
        locals.var_dnm_dn9 = assign52780_e80362_d_n9;
        locals.var_dnm_dn10 = assign52780_e80362_d_n10;
        locals.var_dnm_dn11 = assign52780_e80362_d_n11;
        locals.var_dnm_dn14 = assign52780_e80362_d_n14;

        let (assign52790_e80383, assign52790_e80383_d_n0, assign52790_e80383_d_n2, assign52790_e80383_d_n4, assign52790_e80383_d_n5, assign52790_e80383_d_n6, assign52790_e80383_d_n7, assign52790_e80383_d_n8, assign52790_e80383_d_n9, assign52790_e80383_d_n10, assign52790_e80383_d_n11, assign52790_e80383_d_n14,) = {
    if (((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_guard1339 == 0.0)) && (locals.var_guard1340 != 0.0)) {
        let assign52790_e80381: f64 = (1.0 / locals.var_dnm);
        (assign52790_e80381, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn4 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn5 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn8 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn9 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn11 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn14 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign52790_e80383;
        locals.var_dnm_dn0 = assign52790_e80383_d_n0;
        locals.var_dnm_dn2 = assign52790_e80383_d_n2;
        locals.var_dnm_dn4 = assign52790_e80383_d_n4;
        locals.var_dnm_dn5 = assign52790_e80383_d_n5;
        locals.var_dnm_dn6 = assign52790_e80383_d_n6;
        locals.var_dnm_dn7 = assign52790_e80383_d_n7;
        locals.var_dnm_dn8 = assign52790_e80383_d_n8;
        locals.var_dnm_dn9 = assign52790_e80383_d_n9;
        locals.var_dnm_dn10 = assign52790_e80383_d_n10;
        locals.var_dnm_dn11 = assign52790_e80383_d_n11;
        locals.var_dnm_dn14 = assign52790_e80383_d_n14;

        let (assign52800_e80406, assign52800_e80406_d_n0, assign52800_e80406_d_n2, assign52800_e80406_d_n4, assign52800_e80406_d_n5, assign52800_e80406_d_n6, assign52800_e80406_d_n7, assign52800_e80406_d_n8, assign52800_e80406_d_n9, assign52800_e80406_d_n10, assign52800_e80406_d_n11, assign52800_e80406_d_n14,) = {
    if (((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_guard1339 == 0.0)) && (locals.var_guard1340 != 0.0)) {
        let assign52800_e80402: f64 = (locals.var_tmf1 * 0.2);
        let assign52800_e80404: f64 = (assign52800_e80402 * locals.var_dnm);
        (assign52800_e80404, (((locals.var_tmf1_dn0 * 0.2) * locals.var_dnm) + (assign52800_e80402 * locals.var_dnm_dn0)), (((locals.var_tmf1_dn2 * 0.2) * locals.var_dnm) + (assign52800_e80402 * locals.var_dnm_dn2)), (((locals.var_tmf1_dn4 * 0.2) * locals.var_dnm) + (assign52800_e80402 * locals.var_dnm_dn4)), (((locals.var_tmf1_dn5 * 0.2) * locals.var_dnm) + (assign52800_e80402 * locals.var_dnm_dn5)), (((locals.var_tmf1_dn6 * 0.2) * locals.var_dnm) + (assign52800_e80402 * locals.var_dnm_dn6)), (((locals.var_tmf1_dn7 * 0.2) * locals.var_dnm) + (assign52800_e80402 * locals.var_dnm_dn7)), (((locals.var_tmf1_dn8 * 0.2) * locals.var_dnm) + (assign52800_e80402 * locals.var_dnm_dn8)), (((locals.var_tmf1_dn9 * 0.2) * locals.var_dnm) + (assign52800_e80402 * locals.var_dnm_dn9)), (((locals.var_tmf1_dn10 * 0.2) * locals.var_dnm) + (assign52800_e80402 * locals.var_dnm_dn10)), (((locals.var_tmf1_dn11 * 0.2) * locals.var_dnm) + (assign52800_e80402 * locals.var_dnm_dn11)), (((locals.var_tmf1_dn14 * 0.2) * locals.var_dnm) + (assign52800_e80402 * locals.var_dnm_dn14)),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn11, locals.var_tmf0_dn14,)
    }
};
        locals.var_tmf0 = assign52800_e80406;
        locals.var_tmf0_dn0 = assign52800_e80406_d_n0;
        locals.var_tmf0_dn2 = assign52800_e80406_d_n2;
        locals.var_tmf0_dn4 = assign52800_e80406_d_n4;
        locals.var_tmf0_dn5 = assign52800_e80406_d_n5;
        locals.var_tmf0_dn6 = assign52800_e80406_d_n6;
        locals.var_tmf0_dn7 = assign52800_e80406_d_n7;
        locals.var_tmf0_dn8 = assign52800_e80406_d_n8;
        locals.var_tmf0_dn9 = assign52800_e80406_d_n9;
        locals.var_tmf0_dn10 = assign52800_e80406_d_n10;
        locals.var_tmf0_dn11 = assign52800_e80406_d_n11;
        locals.var_tmf0_dn14 = assign52800_e80406_d_n14;

        let (assign52810_e80431, assign52810_e80431_d_n0, assign52810_e80431_d_n2, assign52810_e80431_d_n4, assign52810_e80431_d_n5, assign52810_e80431_d_n6, assign52810_e80431_d_n7, assign52810_e80431_d_n8, assign52810_e80431_d_n9, assign52810_e80431_d_n10, assign52810_e80431_d_n11, assign52810_e80431_d_n14,) = {
    if (((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_guard1339 == 0.0)) && (locals.var_guard1340 != 0.0)) {
        let assign52810_e80425: f64 = (0.2 * locals.var_xmp);
        let assign52810_e80427: f64 = (assign52810_e80425 * locals.var_dnm);
        let assign52810_e80429: f64 = (assign52810_e80427 / locals.var_arg);
        (assign52810_e80429, ((((((0.2 * locals.var_xmp_dn0) * locals.var_dnm) + (assign52810_e80425 * locals.var_dnm_dn0)) * locals.var_arg) - (assign52810_e80427 * locals.var_arg_dn0)) / (locals.var_arg * locals.var_arg)), ((((((0.2 * locals.var_xmp_dn2) * locals.var_dnm) + (assign52810_e80425 * locals.var_dnm_dn2)) * locals.var_arg) - (assign52810_e80427 * locals.var_arg_dn2)) / (locals.var_arg * locals.var_arg)), ((((((0.2 * locals.var_xmp_dn4) * locals.var_dnm) + (assign52810_e80425 * locals.var_dnm_dn4)) * locals.var_arg) - (assign52810_e80427 * locals.var_arg_dn4)) / (locals.var_arg * locals.var_arg)), ((((((0.2 * locals.var_xmp_dn5) * locals.var_dnm) + (assign52810_e80425 * locals.var_dnm_dn5)) * locals.var_arg) - (assign52810_e80427 * locals.var_arg_dn5)) / (locals.var_arg * locals.var_arg)), ((((((0.2 * locals.var_xmp_dn6) * locals.var_dnm) + (assign52810_e80425 * locals.var_dnm_dn6)) * locals.var_arg) - (assign52810_e80427 * locals.var_arg_dn6)) / (locals.var_arg * locals.var_arg)), ((((((0.2 * locals.var_xmp_dn7) * locals.var_dnm) + (assign52810_e80425 * locals.var_dnm_dn7)) * locals.var_arg) - (assign52810_e80427 * locals.var_arg_dn7)) / (locals.var_arg * locals.var_arg)), ((((((0.2 * locals.var_xmp_dn8) * locals.var_dnm) + (assign52810_e80425 * locals.var_dnm_dn8)) * locals.var_arg) - (assign52810_e80427 * locals.var_arg_dn8)) / (locals.var_arg * locals.var_arg)), ((((((0.2 * locals.var_xmp_dn9) * locals.var_dnm) + (assign52810_e80425 * locals.var_dnm_dn9)) * locals.var_arg) - (assign52810_e80427 * locals.var_arg_dn9)) / (locals.var_arg * locals.var_arg)), ((((((0.2 * locals.var_xmp_dn10) * locals.var_dnm) + (assign52810_e80425 * locals.var_dnm_dn10)) * locals.var_arg) - (assign52810_e80427 * locals.var_arg_dn10)) / (locals.var_arg * locals.var_arg)), ((((((0.2 * locals.var_xmp_dn11) * locals.var_dnm) + (assign52810_e80425 * locals.var_dnm_dn11)) * locals.var_arg) - (assign52810_e80427 * locals.var_arg_dn11)) / (locals.var_arg * locals.var_arg)), ((((((0.2 * locals.var_xmp_dn14) * locals.var_dnm) + (assign52810_e80425 * locals.var_dnm_dn14)) * locals.var_arg) - (assign52810_e80427 * locals.var_arg_dn14)) / (locals.var_arg * locals.var_arg)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign52810_e80431;
        locals.var_t0_dn0 = assign52810_e80431_d_n0;
        locals.var_t0_dn2 = assign52810_e80431_d_n2;
        locals.var_t0_dn4 = assign52810_e80431_d_n4;
        locals.var_t0_dn5 = assign52810_e80431_d_n5;
        locals.var_t0_dn6 = assign52810_e80431_d_n6;
        locals.var_t0_dn7 = assign52810_e80431_d_n7;
        locals.var_t0_dn8 = assign52810_e80431_d_n8;
        locals.var_t0_dn9 = assign52810_e80431_d_n9;
        locals.var_t0_dn10 = assign52810_e80431_d_n10;
        locals.var_t0_dn11 = assign52810_e80431_d_n11;
        locals.var_t0_dn14 = assign52810_e80431_d_n14;

        let (assign52820_e80454, assign52820_e80454_d_n0, assign52820_e80454_d_n2, assign52820_e80454_d_n4, assign52820_e80454_d_n5, assign52820_e80454_d_n6, assign52820_e80454_d_n7, assign52820_e80454_d_n8, assign52820_e80454_d_n9, assign52820_e80454_d_n10, assign52820_e80454_d_n11, assign52820_e80454_d_n14,) = {
    if (((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_guard1339 == 0.0)) && (locals.var_guard1340 != 0.0)) {
        let assign52820_e80450: f64 = (locals.var_ps0dep0 + 0.2);
        let assign52820_e80452: f64 = (assign52820_e80450 - locals.var_tmf0);
        (assign52820_e80452, (locals.var_ps0dep0_dn0 - locals.var_tmf0_dn0), (locals.var_ps0dep0_dn2 - locals.var_tmf0_dn2), (locals.var_ps0dep0_dn4 - locals.var_tmf0_dn4), (locals.var_ps0dep0_dn5 - locals.var_tmf0_dn5), (locals.var_ps0dep0_dn6 - locals.var_tmf0_dn6), (locals.var_ps0dep0_dn7 - locals.var_tmf0_dn7), (locals.var_ps0dep0_dn8 - locals.var_tmf0_dn8), (locals.var_ps0dep0_dn9 - locals.var_tmf0_dn9), (locals.var_ps0dep0_dn10 - locals.var_tmf0_dn10), (locals.var_ps0dep0_dn11 - locals.var_tmf0_dn11), (locals.var_ps0dep0_dn14 - locals.var_tmf0_dn14),)
    } else {
        (locals.var_ps0dep, locals.var_ps0dep_dn0, locals.var_ps0dep_dn2, locals.var_ps0dep_dn4, locals.var_ps0dep_dn5, locals.var_ps0dep_dn6, locals.var_ps0dep_dn7, locals.var_ps0dep_dn8, locals.var_ps0dep_dn9, locals.var_ps0dep_dn10, locals.var_ps0dep_dn11, locals.var_ps0dep_dn14,)
    }
};
        locals.var_ps0dep = assign52820_e80454;
        locals.var_ps0dep_dn0 = assign52820_e80454_d_n0;
        locals.var_ps0dep_dn2 = assign52820_e80454_d_n2;
        locals.var_ps0dep_dn4 = assign52820_e80454_d_n4;
        locals.var_ps0dep_dn5 = assign52820_e80454_d_n5;
        locals.var_ps0dep_dn6 = assign52820_e80454_d_n6;
        locals.var_ps0dep_dn7 = assign52820_e80454_d_n7;
        locals.var_ps0dep_dn8 = assign52820_e80454_d_n8;
        locals.var_ps0dep_dn9 = assign52820_e80454_d_n9;
        locals.var_ps0dep_dn10 = assign52820_e80454_d_n10;
        locals.var_ps0dep_dn11 = assign52820_e80454_d_n11;
        locals.var_ps0dep_dn14 = assign52820_e80454_d_n14;

        let (assign52830_e80473, assign52830_e80473_d_n0, assign52830_e80473_d_n2, assign52830_e80473_d_n4, assign52830_e80473_d_n5, assign52830_e80473_d_n6, assign52830_e80473_d_n7, assign52830_e80473_d_n8, assign52830_e80473_d_n9, assign52830_e80473_d_n10, assign52830_e80473_d_n11, assign52830_e80473_d_n14,) = {
    if (((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_guard1339 == 0.0)) && (locals.var_guard1340 != 0.0)) {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign52830_e80473;
        locals.var_t0_dn0 = assign52830_e80473_d_n0;
        locals.var_t0_dn2 = assign52830_e80473_d_n2;
        locals.var_t0_dn4 = assign52830_e80473_d_n4;
        locals.var_t0_dn5 = assign52830_e80473_d_n5;
        locals.var_t0_dn6 = assign52830_e80473_d_n6;
        locals.var_t0_dn7 = assign52830_e80473_d_n7;
        locals.var_t0_dn8 = assign52830_e80473_d_n8;
        locals.var_t0_dn9 = assign52830_e80473_d_n9;
        locals.var_t0_dn10 = assign52830_e80473_d_n10;
        locals.var_t0_dn11 = assign52830_e80473_d_n11;
        locals.var_t0_dn14 = assign52830_e80473_d_n14;

        let (assign52840_e80493, assign52840_e80493_d_n0, assign52840_e80493_d_n2, assign52840_e80493_d_n4, assign52840_e80493_d_n5, assign52840_e80493_d_n6, assign52840_e80493_d_n7, assign52840_e80493_d_n8, assign52840_e80493_d_n9, assign52840_e80493_d_n10, assign52840_e80493_d_n11, assign52840_e80493_d_n14,) = {
    if (((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_guard1339 == 0.0)) && (locals.var_guard1340 == 0.0)) {
        (locals.var_ps0dep, locals.var_ps0dep_dn0, locals.var_ps0dep_dn2, locals.var_ps0dep_dn4, locals.var_ps0dep_dn5, locals.var_ps0dep_dn6, locals.var_ps0dep_dn7, locals.var_ps0dep_dn8, locals.var_ps0dep_dn9, locals.var_ps0dep_dn10, locals.var_ps0dep_dn11, locals.var_ps0dep_dn14,)
    } else {
        (locals.var_ps0dep, locals.var_ps0dep_dn0, locals.var_ps0dep_dn2, locals.var_ps0dep_dn4, locals.var_ps0dep_dn5, locals.var_ps0dep_dn6, locals.var_ps0dep_dn7, locals.var_ps0dep_dn8, locals.var_ps0dep_dn9, locals.var_ps0dep_dn10, locals.var_ps0dep_dn11, locals.var_ps0dep_dn14,)
    }
};
        locals.var_ps0dep = assign52840_e80493;
        locals.var_ps0dep_dn0 = assign52840_e80493_d_n0;
        locals.var_ps0dep_dn2 = assign52840_e80493_d_n2;
        locals.var_ps0dep_dn4 = assign52840_e80493_d_n4;
        locals.var_ps0dep_dn5 = assign52840_e80493_d_n5;
        locals.var_ps0dep_dn6 = assign52840_e80493_d_n6;
        locals.var_ps0dep_dn7 = assign52840_e80493_d_n7;
        locals.var_ps0dep_dn8 = assign52840_e80493_d_n8;
        locals.var_ps0dep_dn9 = assign52840_e80493_d_n9;
        locals.var_ps0dep_dn10 = assign52840_e80493_d_n10;
        locals.var_ps0dep_dn11 = assign52840_e80493_d_n11;
        locals.var_ps0dep_dn14 = assign52840_e80493_d_n14;

        let (assign52850_e80513, assign52850_e80513_d_n0, assign52850_e80513_d_n2, assign52850_e80513_d_n4, assign52850_e80513_d_n5, assign52850_e80513_d_n6, assign52850_e80513_d_n7, assign52850_e80513_d_n8, assign52850_e80513_d_n9, assign52850_e80513_d_n10, assign52850_e80513_d_n11, assign52850_e80513_d_n14,) = {
    if (((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_guard1339 == 0.0)) && (locals.var_guard1340 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign52850_e80513;
        locals.var_t0_dn0 = assign52850_e80513_d_n0;
        locals.var_t0_dn2 = assign52850_e80513_d_n2;
        locals.var_t0_dn4 = assign52850_e80513_d_n4;
        locals.var_t0_dn5 = assign52850_e80513_d_n5;
        locals.var_t0_dn6 = assign52850_e80513_d_n6;
        locals.var_t0_dn7 = assign52850_e80513_d_n7;
        locals.var_t0_dn8 = assign52850_e80513_d_n8;
        locals.var_t0_dn9 = assign52850_e80513_d_n9;
        locals.var_t0_dn10 = assign52850_e80513_d_n10;
        locals.var_t0_dn11 = assign52850_e80513_d_n11;
        locals.var_t0_dn14 = assign52850_e80513_d_n14;

        let (assign52860_e80527, assign52860_e80527_d_n0, assign52860_e80527_d_n2, assign52860_e80527_d_n4, assign52860_e80527_d_n5, assign52860_e80527_d_n6, assign52860_e80527_d_n7, assign52860_e80527_d_n8, assign52860_e80527_d_n9, assign52860_e80527_d_n10, assign52860_e80527_d_n11, assign52860_e80527_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) {
        (locals.var_ps0dep, locals.var_ps0dep_dn0, locals.var_ps0dep_dn2, locals.var_ps0dep_dn4, locals.var_ps0dep_dn5, locals.var_ps0dep_dn6, locals.var_ps0dep_dn7, locals.var_ps0dep_dn8, locals.var_ps0dep_dn9, locals.var_ps0dep_dn10, locals.var_ps0dep_dn11, locals.var_ps0dep_dn14,)
    } else {
        (locals.var_ps0_res, locals.var_ps0_res_dn0, locals.var_ps0_res_dn2, locals.var_ps0_res_dn4, locals.var_ps0_res_dn5, locals.var_ps0_res_dn6, locals.var_ps0_res_dn7, locals.var_ps0_res_dn8, locals.var_ps0_res_dn9, locals.var_ps0_res_dn10, locals.var_ps0_res_dn11, locals.var_ps0_res_dn14,)
    }
};
        locals.var_ps0_res = assign52860_e80527;
        locals.var_ps0_res_dn0 = assign52860_e80527_d_n0;
        locals.var_ps0_res_dn2 = assign52860_e80527_d_n2;
        locals.var_ps0_res_dn4 = assign52860_e80527_d_n4;
        locals.var_ps0_res_dn5 = assign52860_e80527_d_n5;
        locals.var_ps0_res_dn6 = assign52860_e80527_d_n6;
        locals.var_ps0_res_dn7 = assign52860_e80527_d_n7;
        locals.var_ps0_res_dn8 = assign52860_e80527_d_n8;
        locals.var_ps0_res_dn9 = assign52860_e80527_d_n9;
        locals.var_ps0_res_dn10 = assign52860_e80527_d_n10;
        locals.var_ps0_res_dn11 = assign52860_e80527_d_n11;
        locals.var_ps0_res_dn14 = assign52860_e80527_d_n14;

        let (assign52870_e80546,) = {
    if (((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) {
        let (assign52870_e80544,) = {
            if (1e-6 >= p.p407) {
                (1e-6,)
            } else {
                (p.p407,)
            }
        };
        (assign52870_e80544,)
    } else {
        (locals.var_vgpdep_dlt__blk1146,)
    }
};
        locals.var_vgpdep_dlt__blk1146 = assign52870_e80546;

        let assign52880_e80550: f64 = (-locals.var_vgpdep_dlt__blk1146);
        let assign52880_e80555: f64 = if ((locals.var_ps0_res > assign52880_e80550) && (locals.var_vgpdep_dlt__blk1146 >= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1346 = assign52880_e80555;

    }

    pub(super) fn stamp_transient_block_181(
        locals: &mut StampLocals,
    ) {
        let (assign52890_e80575, assign52890_e80575_d_n0, assign52890_e80575_d_n2, assign52890_e80575_d_n4, assign52890_e80575_d_n5, assign52890_e80575_d_n6, assign52890_e80575_d_n7, assign52890_e80575_d_n8, assign52890_e80575_d_n9, assign52890_e80575_d_n10, assign52890_e80575_d_n11, assign52890_e80575_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_guard1346 != 0.0)) {
        let assign52890_e80571: f64 = locals.var_ps0_res;
        let assign52890_e80573: f64 = (assign52890_e80571 + locals.var_vgpdep_dlt__blk1146);
        (assign52890_e80573, locals.var_ps0_res_dn0, locals.var_ps0_res_dn2, locals.var_ps0_res_dn4, locals.var_ps0_res_dn5, locals.var_ps0_res_dn6, locals.var_ps0_res_dn7, locals.var_ps0_res_dn8, locals.var_ps0_res_dn9, locals.var_ps0_res_dn10, locals.var_ps0_res_dn11, locals.var_ps0_res_dn14,)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign52890_e80575;
        locals.var_tmf1_dn0 = assign52890_e80575_d_n0;
        locals.var_tmf1_dn2 = assign52890_e80575_d_n2;
        locals.var_tmf1_dn4 = assign52890_e80575_d_n4;
        locals.var_tmf1_dn5 = assign52890_e80575_d_n5;
        locals.var_tmf1_dn6 = assign52890_e80575_d_n6;
        locals.var_tmf1_dn7 = assign52890_e80575_d_n7;
        locals.var_tmf1_dn8 = assign52890_e80575_d_n8;
        locals.var_tmf1_dn9 = assign52890_e80575_d_n9;
        locals.var_tmf1_dn10 = assign52890_e80575_d_n10;
        locals.var_tmf1_dn11 = assign52890_e80575_d_n11;
        locals.var_tmf1_dn14 = assign52890_e80575_d_n14;

        let (assign52900_e80593, assign52900_e80593_d_n0, assign52900_e80593_d_n2, assign52900_e80593_d_n4, assign52900_e80593_d_n5, assign52900_e80593_d_n6, assign52900_e80593_d_n7, assign52900_e80593_d_n8, assign52900_e80593_d_n9, assign52900_e80593_d_n10, assign52900_e80593_d_n11, assign52900_e80593_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_guard1346 != 0.0)) {
        let assign52900_e80591: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign52900_e80591, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)), ((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)), ((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)), ((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn4, locals.var_x2_dn5, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn8, locals.var_x2_dn9, locals.var_x2_dn10, locals.var_x2_dn11, locals.var_x2_dn14,)
    }
};
        locals.var_x2 = assign52900_e80593;
        locals.var_x2_dn0 = assign52900_e80593_d_n0;
        locals.var_x2_dn2 = assign52900_e80593_d_n2;
        locals.var_x2_dn4 = assign52900_e80593_d_n4;
        locals.var_x2_dn5 = assign52900_e80593_d_n5;
        locals.var_x2_dn6 = assign52900_e80593_d_n6;
        locals.var_x2_dn7 = assign52900_e80593_d_n7;
        locals.var_x2_dn8 = assign52900_e80593_d_n8;
        locals.var_x2_dn9 = assign52900_e80593_d_n9;
        locals.var_x2_dn10 = assign52900_e80593_d_n10;
        locals.var_x2_dn11 = assign52900_e80593_d_n11;
        locals.var_x2_dn14 = assign52900_e80593_d_n14;

        let (assign52910_e80611, assign52910_e80611_d_n0, assign52910_e80611_d_n2, assign52910_e80611_d_n4, assign52910_e80611_d_n5, assign52910_e80611_d_n6, assign52910_e80611_d_n7, assign52910_e80611_d_n8, assign52910_e80611_d_n9, assign52910_e80611_d_n10, assign52910_e80611_d_n11, assign52910_e80611_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_guard1346 != 0.0)) {
        let assign52910_e80609: f64 = (locals.var_vgpdep_dlt__blk1146 * locals.var_vgpdep_dlt__blk1146);
        (assign52910_e80609, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn4, locals.var_xmax2_dn5, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn8, locals.var_xmax2_dn9, locals.var_xmax2_dn10, locals.var_xmax2_dn11, locals.var_xmax2_dn14,)
    }
};
        locals.var_xmax2 = assign52910_e80611;
        locals.var_xmax2_dn0 = assign52910_e80611_d_n0;
        locals.var_xmax2_dn2 = assign52910_e80611_d_n2;
        locals.var_xmax2_dn4 = assign52910_e80611_d_n4;
        locals.var_xmax2_dn5 = assign52910_e80611_d_n5;
        locals.var_xmax2_dn6 = assign52910_e80611_d_n6;
        locals.var_xmax2_dn7 = assign52910_e80611_d_n7;
        locals.var_xmax2_dn8 = assign52910_e80611_d_n8;
        locals.var_xmax2_dn9 = assign52910_e80611_d_n9;
        locals.var_xmax2_dn10 = assign52910_e80611_d_n10;
        locals.var_xmax2_dn11 = assign52910_e80611_d_n11;
        locals.var_xmax2_dn14 = assign52910_e80611_d_n14;

        let (assign52920_e80627, assign52920_e80627_d_n0, assign52920_e80627_d_n2, assign52920_e80627_d_n4, assign52920_e80627_d_n5, assign52920_e80627_d_n6, assign52920_e80627_d_n7, assign52920_e80627_d_n8, assign52920_e80627_d_n9, assign52920_e80627_d_n10, assign52920_e80627_d_n11, assign52920_e80627_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_guard1346 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign52920_e80627;
        locals.var_xp_dn0 = assign52920_e80627_d_n0;
        locals.var_xp_dn2 = assign52920_e80627_d_n2;
        locals.var_xp_dn4 = assign52920_e80627_d_n4;
        locals.var_xp_dn5 = assign52920_e80627_d_n5;
        locals.var_xp_dn6 = assign52920_e80627_d_n6;
        locals.var_xp_dn7 = assign52920_e80627_d_n7;
        locals.var_xp_dn8 = assign52920_e80627_d_n8;
        locals.var_xp_dn9 = assign52920_e80627_d_n9;
        locals.var_xp_dn10 = assign52920_e80627_d_n10;
        locals.var_xp_dn11 = assign52920_e80627_d_n11;
        locals.var_xp_dn14 = assign52920_e80627_d_n14;

        let (assign52930_e80643, assign52930_e80643_d_n0, assign52930_e80643_d_n2, assign52930_e80643_d_n4, assign52930_e80643_d_n5, assign52930_e80643_d_n6, assign52930_e80643_d_n7, assign52930_e80643_d_n8, assign52930_e80643_d_n9, assign52930_e80643_d_n10, assign52930_e80643_d_n11, assign52930_e80643_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_guard1346 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign52930_e80643;
        locals.var_xmp_dn0 = assign52930_e80643_d_n0;
        locals.var_xmp_dn2 = assign52930_e80643_d_n2;
        locals.var_xmp_dn4 = assign52930_e80643_d_n4;
        locals.var_xmp_dn5 = assign52930_e80643_d_n5;
        locals.var_xmp_dn6 = assign52930_e80643_d_n6;
        locals.var_xmp_dn7 = assign52930_e80643_d_n7;
        locals.var_xmp_dn8 = assign52930_e80643_d_n8;
        locals.var_xmp_dn9 = assign52930_e80643_d_n9;
        locals.var_xmp_dn10 = assign52930_e80643_d_n10;
        locals.var_xmp_dn11 = assign52930_e80643_d_n11;
        locals.var_xmp_dn14 = assign52930_e80643_d_n14;

        let (assign52940_e80659,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_guard1346 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign52940_e80659;

        let (assign52950_e80675,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_guard1346 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign52950_e80675;

        let (assign52960_e80691, assign52960_e80691_d_n0, assign52960_e80691_d_n2, assign52960_e80691_d_n4, assign52960_e80691_d_n5, assign52960_e80691_d_n6, assign52960_e80691_d_n7, assign52960_e80691_d_n8, assign52960_e80691_d_n9, assign52960_e80691_d_n10, assign52960_e80691_d_n11, assign52960_e80691_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_guard1346 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign52960_e80691;
        locals.var_arg_dn0 = assign52960_e80691_d_n0;
        locals.var_arg_dn2 = assign52960_e80691_d_n2;
        locals.var_arg_dn4 = assign52960_e80691_d_n4;
        locals.var_arg_dn5 = assign52960_e80691_d_n5;
        locals.var_arg_dn6 = assign52960_e80691_d_n6;
        locals.var_arg_dn7 = assign52960_e80691_d_n7;
        locals.var_arg_dn8 = assign52960_e80691_d_n8;
        locals.var_arg_dn9 = assign52960_e80691_d_n9;
        locals.var_arg_dn10 = assign52960_e80691_d_n10;
        locals.var_arg_dn11 = assign52960_e80691_d_n11;
        locals.var_arg_dn14 = assign52960_e80691_d_n14;

        let (assign52970_e80707, assign52970_e80707_d_n0, assign52970_e80707_d_n2, assign52970_e80707_d_n4, assign52970_e80707_d_n5, assign52970_e80707_d_n6, assign52970_e80707_d_n7, assign52970_e80707_d_n8, assign52970_e80707_d_n9, assign52970_e80707_d_n10, assign52970_e80707_d_n11, assign52970_e80707_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_guard1346 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign52970_e80707;
        locals.var_dnm_dn0 = assign52970_e80707_d_n0;
        locals.var_dnm_dn2 = assign52970_e80707_d_n2;
        locals.var_dnm_dn4 = assign52970_e80707_d_n4;
        locals.var_dnm_dn5 = assign52970_e80707_d_n5;
        locals.var_dnm_dn6 = assign52970_e80707_d_n6;
        locals.var_dnm_dn7 = assign52970_e80707_d_n7;
        locals.var_dnm_dn8 = assign52970_e80707_d_n8;
        locals.var_dnm_dn9 = assign52970_e80707_d_n9;
        locals.var_dnm_dn10 = assign52970_e80707_d_n10;
        locals.var_dnm_dn11 = assign52970_e80707_d_n11;
        locals.var_dnm_dn14 = assign52970_e80707_d_n14;

        let (assign52980_e80723,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_guard1346 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign52980_e80723;

        let mut assign52990_loop_guard: usize = 0;
        while {
            let assign52990_cond_e80740: f64 = if (((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_guard1346 != 0.0)) && (locals.var_m0 < locals.var_vgpdep_pw__blk1147)) { 1.0 } else { 0.0 };
            assign52990_cond_e80740 != 0.0
        } {
            assign52990_loop_guard += 1;
            assert!(assign52990_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign52990_body0_e80758, assign52990_body0_e80758_d_n0, assign52990_body0_e80758_d_n2, assign52990_body0_e80758_d_n4, assign52990_body0_e80758_d_n5, assign52990_body0_e80758_d_n6, assign52990_body0_e80758_d_n7, assign52990_body0_e80758_d_n8, assign52990_body0_e80758_d_n9, assign52990_body0_e80758_d_n10, assign52990_body0_e80758_d_n11, assign52990_body0_e80758_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_guard1346 != 0.0)) {
        let assign52990_body0_e80756: f64 = (locals.var_xp * locals.var_x2);
        (assign52990_body0_e80756, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
            locals.var_xp = assign52990_body0_e80758;
            locals.var_xp_dn0 = assign52990_body0_e80758_d_n0;
            locals.var_xp_dn2 = assign52990_body0_e80758_d_n2;
            locals.var_xp_dn4 = assign52990_body0_e80758_d_n4;
            locals.var_xp_dn5 = assign52990_body0_e80758_d_n5;
            locals.var_xp_dn6 = assign52990_body0_e80758_d_n6;
            locals.var_xp_dn7 = assign52990_body0_e80758_d_n7;
            locals.var_xp_dn8 = assign52990_body0_e80758_d_n8;
            locals.var_xp_dn9 = assign52990_body0_e80758_d_n9;
            locals.var_xp_dn10 = assign52990_body0_e80758_d_n10;
            locals.var_xp_dn11 = assign52990_body0_e80758_d_n11;
            locals.var_xp_dn14 = assign52990_body0_e80758_d_n14;
            let (assign52990_body1_e80776, assign52990_body1_e80776_d_n0, assign52990_body1_e80776_d_n2, assign52990_body1_e80776_d_n4, assign52990_body1_e80776_d_n5, assign52990_body1_e80776_d_n6, assign52990_body1_e80776_d_n7, assign52990_body1_e80776_d_n8, assign52990_body1_e80776_d_n9, assign52990_body1_e80776_d_n10, assign52990_body1_e80776_d_n11, assign52990_body1_e80776_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_guard1346 != 0.0)) {
        let assign52990_body1_e80774: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign52990_body1_e80774, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
            locals.var_xmp = assign52990_body1_e80776;
            locals.var_xmp_dn0 = assign52990_body1_e80776_d_n0;
            locals.var_xmp_dn2 = assign52990_body1_e80776_d_n2;
            locals.var_xmp_dn4 = assign52990_body1_e80776_d_n4;
            locals.var_xmp_dn5 = assign52990_body1_e80776_d_n5;
            locals.var_xmp_dn6 = assign52990_body1_e80776_d_n6;
            locals.var_xmp_dn7 = assign52990_body1_e80776_d_n7;
            locals.var_xmp_dn8 = assign52990_body1_e80776_d_n8;
            locals.var_xmp_dn9 = assign52990_body1_e80776_d_n9;
            locals.var_xmp_dn10 = assign52990_body1_e80776_d_n10;
            locals.var_xmp_dn11 = assign52990_body1_e80776_d_n11;
            locals.var_xmp_dn14 = assign52990_body1_e80776_d_n14;
            let (assign52990_body2_e80794,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_guard1346 != 0.0)) {
        let assign52990_body2_e80792: f64 = (locals.var_m0 + 1.0);
        (assign52990_body2_e80792,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign52990_body2_e80794;
        }

        let (assign53000_e80812, assign53000_e80812_d_n0, assign53000_e80812_d_n2, assign53000_e80812_d_n4, assign53000_e80812_d_n5, assign53000_e80812_d_n6, assign53000_e80812_d_n7, assign53000_e80812_d_n8, assign53000_e80812_d_n9, assign53000_e80812_d_n10, assign53000_e80812_d_n11, assign53000_e80812_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_guard1346 != 0.0)) {
        let assign53000_e80810: f64 = (locals.var_xp + locals.var_xmp);
        (assign53000_e80810, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn4 + locals.var_xmp_dn4), (locals.var_xp_dn5 + locals.var_xmp_dn5), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn8 + locals.var_xmp_dn8), (locals.var_xp_dn9 + locals.var_xmp_dn9), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn11 + locals.var_xmp_dn11), (locals.var_xp_dn14 + locals.var_xmp_dn14),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign53000_e80812;
        locals.var_arg_dn0 = assign53000_e80812_d_n0;
        locals.var_arg_dn2 = assign53000_e80812_d_n2;
        locals.var_arg_dn4 = assign53000_e80812_d_n4;
        locals.var_arg_dn5 = assign53000_e80812_d_n5;
        locals.var_arg_dn6 = assign53000_e80812_d_n6;
        locals.var_arg_dn7 = assign53000_e80812_d_n7;
        locals.var_arg_dn8 = assign53000_e80812_d_n8;
        locals.var_arg_dn9 = assign53000_e80812_d_n9;
        locals.var_arg_dn10 = assign53000_e80812_d_n10;
        locals.var_arg_dn11 = assign53000_e80812_d_n11;
        locals.var_arg_dn14 = assign53000_e80812_d_n14;

        let (assign53010_e80828, assign53010_e80828_d_n0, assign53010_e80828_d_n2, assign53010_e80828_d_n4, assign53010_e80828_d_n5, assign53010_e80828_d_n6, assign53010_e80828_d_n7, assign53010_e80828_d_n8, assign53010_e80828_d_n9, assign53010_e80828_d_n10, assign53010_e80828_d_n11, assign53010_e80828_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_guard1346 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign53010_e80828;
        locals.var_dnm_dn0 = assign53010_e80828_d_n0;
        locals.var_dnm_dn2 = assign53010_e80828_d_n2;
        locals.var_dnm_dn4 = assign53010_e80828_d_n4;
        locals.var_dnm_dn5 = assign53010_e80828_d_n5;
        locals.var_dnm_dn6 = assign53010_e80828_d_n6;
        locals.var_dnm_dn7 = assign53010_e80828_d_n7;
        locals.var_dnm_dn8 = assign53010_e80828_d_n8;
        locals.var_dnm_dn9 = assign53010_e80828_d_n9;
        locals.var_dnm_dn10 = assign53010_e80828_d_n10;
        locals.var_dnm_dn11 = assign53010_e80828_d_n11;
        locals.var_dnm_dn14 = assign53010_e80828_d_n14;

        let assign53020_e80843: f64 = if ((((locals.var_vgpdep_pw__blk1147 == 1.0) || (locals.var_vgpdep_pw__blk1147 == 2.0)) || (locals.var_vgpdep_pw__blk1147 == 4.0)) || (locals.var_vgpdep_pw__blk1147 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard1347 = assign53020_e80843;

        let assign53030_e80846: f64 = if locals.var_vgpdep_pw__blk1147 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1348 = assign53030_e80846;

        let (assign53040_e80866,) = {
    if ((((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_guard1346 != 0.0)) && (locals.var_guard1347 != 0.0)) && (locals.var_guard1348 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign53040_e80866;

        let assign53050_e80869: f64 = if locals.var_vgpdep_pw__blk1147 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard1349 = assign53050_e80869;

        let (assign53060_e80892,) = {
    if (((((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_guard1346 != 0.0)) && (locals.var_guard1347 != 0.0)) && (locals.var_guard1348 == 0.0)) && (locals.var_guard1349 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign53060_e80892;

        let assign53070_e80895: f64 = if locals.var_vgpdep_pw__blk1147 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard1350 = assign53070_e80895;

        let (assign53080_e80921,) = {
    if ((((((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_guard1346 != 0.0)) && (locals.var_guard1347 != 0.0)) && (locals.var_guard1348 == 0.0)) && (locals.var_guard1349 == 0.0)) && (locals.var_guard1350 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign53080_e80921;

        let assign53090_e80924: f64 = if locals.var_vgpdep_pw__blk1147 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard1351 = assign53090_e80924;

        let (assign53100_e80953,) = {
    if (((((((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_guard1346 != 0.0)) && (locals.var_guard1347 != 0.0)) && (locals.var_guard1348 == 0.0)) && (locals.var_guard1349 == 0.0)) && (locals.var_guard1350 == 0.0)) && (locals.var_guard1351 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign53100_e80953;

        let (assign53110_e80971,) = {
    if (((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_guard1346 != 0.0)) && (locals.var_guard1347 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign53110_e80971;

        let mut assign53120_loop_guard: usize = 0;
        while {
            let assign53120_cond_e80990: f64 = if ((((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_guard1346 != 0.0)) && (locals.var_guard1347 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign53120_cond_e80990 != 0.0
        } {
            assign53120_loop_guard += 1;
            assert!(assign53120_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign53120_body0_e81009, assign53120_body0_e81009_d_n0, assign53120_body0_e81009_d_n2, assign53120_body0_e81009_d_n4, assign53120_body0_e81009_d_n5, assign53120_body0_e81009_d_n6, assign53120_body0_e81009_d_n7, assign53120_body0_e81009_d_n8, assign53120_body0_e81009_d_n9, assign53120_body0_e81009_d_n10, assign53120_body0_e81009_d_n11, assign53120_body0_e81009_d_n14,) = {
    if (((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_guard1346 != 0.0)) && (locals.var_guard1347 != 0.0)) {
        let assign53120_body0_e81007: f64 = (locals.var_dnm).sqrt();
        (assign53120_body0_e81007, (locals.var_dnm_dn0 / (2.0 * assign53120_body0_e81007)), (locals.var_dnm_dn2 / (2.0 * assign53120_body0_e81007)), (locals.var_dnm_dn4 / (2.0 * assign53120_body0_e81007)), (locals.var_dnm_dn5 / (2.0 * assign53120_body0_e81007)), (locals.var_dnm_dn6 / (2.0 * assign53120_body0_e81007)), (locals.var_dnm_dn7 / (2.0 * assign53120_body0_e81007)), (locals.var_dnm_dn8 / (2.0 * assign53120_body0_e81007)), (locals.var_dnm_dn9 / (2.0 * assign53120_body0_e81007)), (locals.var_dnm_dn10 / (2.0 * assign53120_body0_e81007)), (locals.var_dnm_dn11 / (2.0 * assign53120_body0_e81007)), (locals.var_dnm_dn14 / (2.0 * assign53120_body0_e81007)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
            locals.var_dnm = assign53120_body0_e81009;
            locals.var_dnm_dn0 = assign53120_body0_e81009_d_n0;
            locals.var_dnm_dn2 = assign53120_body0_e81009_d_n2;
            locals.var_dnm_dn4 = assign53120_body0_e81009_d_n4;
            locals.var_dnm_dn5 = assign53120_body0_e81009_d_n5;
            locals.var_dnm_dn6 = assign53120_body0_e81009_d_n6;
            locals.var_dnm_dn7 = assign53120_body0_e81009_d_n7;
            locals.var_dnm_dn8 = assign53120_body0_e81009_d_n8;
            locals.var_dnm_dn9 = assign53120_body0_e81009_d_n9;
            locals.var_dnm_dn10 = assign53120_body0_e81009_d_n10;
            locals.var_dnm_dn11 = assign53120_body0_e81009_d_n11;
            locals.var_dnm_dn14 = assign53120_body0_e81009_d_n14;
            let (assign53120_body1_e81029,) = {
    if (((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_guard1346 != 0.0)) && (locals.var_guard1347 != 0.0)) {
        let assign53120_body1_e81027: f64 = (locals.var_m0 + 1.0);
        (assign53120_body1_e81027,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign53120_body1_e81029;
        }

        let (assign53130_e81059, assign53130_e81059_d_n0, assign53130_e81059_d_n2, assign53130_e81059_d_n4, assign53130_e81059_d_n5, assign53130_e81059_d_n6, assign53130_e81059_d_n7, assign53130_e81059_d_n8, assign53130_e81059_d_n9, assign53130_e81059_d_n10, assign53130_e81059_d_n11, assign53130_e81059_d_n14,) = {
    if (((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_guard1346 != 0.0)) && (locals.var_guard1347 == 0.0)) {
        let (assign53130_e81057, assign53130_e81057_d_n0, assign53130_e81057_d_n2, assign53130_e81057_d_n4, assign53130_e81057_d_n5, assign53130_e81057_d_n6, assign53130_e81057_d_n7, assign53130_e81057_d_n8, assign53130_e81057_d_n9, assign53130_e81057_d_n10, assign53130_e81057_d_n11, assign53130_e81057_d_n14,) = {
            if (locals.var_dnm == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign53130_e81054: f64 = (2.0 * locals.var_vgpdep_pw__blk1147);
                let assign53130_e81055: f64 = (1.0 / assign53130_e81054);
                let assign53130_e81056: f64 = (locals.var_dnm).powf(assign53130_e81055);
                (assign53130_e81056, if 0.0 == 0.0 && ((assign53130_e81055) as f64).is_finite() && ((assign53130_e81055) as f64).fract() == 0.0 { if assign53130_e81055 == 0.0 { 0.0 } else { (assign53130_e81055 * ((locals.var_dnm).powf(assign53130_e81055 - 1.0) * locals.var_dnm_dn0)) } } else { (assign53130_e81056 * (assign53130_e81055 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign53130_e81055) as f64).is_finite() && ((assign53130_e81055) as f64).fract() == 0.0 { if assign53130_e81055 == 0.0 { 0.0 } else { (assign53130_e81055 * ((locals.var_dnm).powf(assign53130_e81055 - 1.0) * locals.var_dnm_dn2)) } } else { (assign53130_e81056 * (assign53130_e81055 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign53130_e81055) as f64).is_finite() && ((assign53130_e81055) as f64).fract() == 0.0 { if assign53130_e81055 == 0.0 { 0.0 } else { (assign53130_e81055 * ((locals.var_dnm).powf(assign53130_e81055 - 1.0) * locals.var_dnm_dn4)) } } else { (assign53130_e81056 * (assign53130_e81055 * (locals.var_dnm_dn4 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign53130_e81055) as f64).is_finite() && ((assign53130_e81055) as f64).fract() == 0.0 { if assign53130_e81055 == 0.0 { 0.0 } else { (assign53130_e81055 * ((locals.var_dnm).powf(assign53130_e81055 - 1.0) * locals.var_dnm_dn5)) } } else { (assign53130_e81056 * (assign53130_e81055 * (locals.var_dnm_dn5 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign53130_e81055) as f64).is_finite() && ((assign53130_e81055) as f64).fract() == 0.0 { if assign53130_e81055 == 0.0 { 0.0 } else { (assign53130_e81055 * ((locals.var_dnm).powf(assign53130_e81055 - 1.0) * locals.var_dnm_dn6)) } } else { (assign53130_e81056 * (assign53130_e81055 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign53130_e81055) as f64).is_finite() && ((assign53130_e81055) as f64).fract() == 0.0 { if assign53130_e81055 == 0.0 { 0.0 } else { (assign53130_e81055 * ((locals.var_dnm).powf(assign53130_e81055 - 1.0) * locals.var_dnm_dn7)) } } else { (assign53130_e81056 * (assign53130_e81055 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign53130_e81055) as f64).is_finite() && ((assign53130_e81055) as f64).fract() == 0.0 { if assign53130_e81055 == 0.0 { 0.0 } else { (assign53130_e81055 * ((locals.var_dnm).powf(assign53130_e81055 - 1.0) * locals.var_dnm_dn8)) } } else { (assign53130_e81056 * (assign53130_e81055 * (locals.var_dnm_dn8 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign53130_e81055) as f64).is_finite() && ((assign53130_e81055) as f64).fract() == 0.0 { if assign53130_e81055 == 0.0 { 0.0 } else { (assign53130_e81055 * ((locals.var_dnm).powf(assign53130_e81055 - 1.0) * locals.var_dnm_dn9)) } } else { (assign53130_e81056 * (assign53130_e81055 * (locals.var_dnm_dn9 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign53130_e81055) as f64).is_finite() && ((assign53130_e81055) as f64).fract() == 0.0 { if assign53130_e81055 == 0.0 { 0.0 } else { (assign53130_e81055 * ((locals.var_dnm).powf(assign53130_e81055 - 1.0) * locals.var_dnm_dn10)) } } else { (assign53130_e81056 * (assign53130_e81055 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign53130_e81055) as f64).is_finite() && ((assign53130_e81055) as f64).fract() == 0.0 { if assign53130_e81055 == 0.0 { 0.0 } else { (assign53130_e81055 * ((locals.var_dnm).powf(assign53130_e81055 - 1.0) * locals.var_dnm_dn11)) } } else { (assign53130_e81056 * (assign53130_e81055 * (locals.var_dnm_dn11 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign53130_e81055) as f64).is_finite() && ((assign53130_e81055) as f64).fract() == 0.0 { if assign53130_e81055 == 0.0 { 0.0 } else { (assign53130_e81055 * ((locals.var_dnm).powf(assign53130_e81055 - 1.0) * locals.var_dnm_dn14)) } } else { (assign53130_e81056 * (assign53130_e81055 * (locals.var_dnm_dn14 / locals.var_dnm))) },)
            }
        };
        (assign53130_e81057, assign53130_e81057_d_n0, assign53130_e81057_d_n2, assign53130_e81057_d_n4, assign53130_e81057_d_n5, assign53130_e81057_d_n6, assign53130_e81057_d_n7, assign53130_e81057_d_n8, assign53130_e81057_d_n9, assign53130_e81057_d_n10, assign53130_e81057_d_n11, assign53130_e81057_d_n14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign53130_e81059;
        locals.var_dnm_dn0 = assign53130_e81059_d_n0;
        locals.var_dnm_dn2 = assign53130_e81059_d_n2;
        locals.var_dnm_dn4 = assign53130_e81059_d_n4;
        locals.var_dnm_dn5 = assign53130_e81059_d_n5;
        locals.var_dnm_dn6 = assign53130_e81059_d_n6;
        locals.var_dnm_dn7 = assign53130_e81059_d_n7;
        locals.var_dnm_dn8 = assign53130_e81059_d_n8;
        locals.var_dnm_dn9 = assign53130_e81059_d_n9;
        locals.var_dnm_dn10 = assign53130_e81059_d_n10;
        locals.var_dnm_dn11 = assign53130_e81059_d_n11;
        locals.var_dnm_dn14 = assign53130_e81059_d_n14;

        let (assign53140_e81077, assign53140_e81077_d_n0, assign53140_e81077_d_n2, assign53140_e81077_d_n4, assign53140_e81077_d_n5, assign53140_e81077_d_n6, assign53140_e81077_d_n7, assign53140_e81077_d_n8, assign53140_e81077_d_n9, assign53140_e81077_d_n10, assign53140_e81077_d_n11, assign53140_e81077_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_guard1346 != 0.0)) {
        let assign53140_e81075: f64 = (1.0 / locals.var_dnm);
        (assign53140_e81075, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn4 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn5 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn8 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn9 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn11 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn14 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign53140_e81077;
        locals.var_dnm_dn0 = assign53140_e81077_d_n0;
        locals.var_dnm_dn2 = assign53140_e81077_d_n2;
        locals.var_dnm_dn4 = assign53140_e81077_d_n4;
        locals.var_dnm_dn5 = assign53140_e81077_d_n5;
        locals.var_dnm_dn6 = assign53140_e81077_d_n6;
        locals.var_dnm_dn7 = assign53140_e81077_d_n7;
        locals.var_dnm_dn8 = assign53140_e81077_d_n8;
        locals.var_dnm_dn9 = assign53140_e81077_d_n9;
        locals.var_dnm_dn10 = assign53140_e81077_d_n10;
        locals.var_dnm_dn11 = assign53140_e81077_d_n11;
        locals.var_dnm_dn14 = assign53140_e81077_d_n14;

        let (assign53150_e81097, assign53150_e81097_d_n0, assign53150_e81097_d_n2, assign53150_e81097_d_n4, assign53150_e81097_d_n5, assign53150_e81097_d_n6, assign53150_e81097_d_n7, assign53150_e81097_d_n8, assign53150_e81097_d_n9, assign53150_e81097_d_n10, assign53150_e81097_d_n11, assign53150_e81097_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_guard1346 != 0.0)) {
        let assign53150_e81093: f64 = (locals.var_tmf1 * locals.var_vgpdep_dlt__blk1146);
        let assign53150_e81095: f64 = (assign53150_e81093 * locals.var_dnm);
        (assign53150_e81095, (((locals.var_tmf1_dn0 * locals.var_vgpdep_dlt__blk1146) * locals.var_dnm) + (assign53150_e81093 * locals.var_dnm_dn0)), (((locals.var_tmf1_dn2 * locals.var_vgpdep_dlt__blk1146) * locals.var_dnm) + (assign53150_e81093 * locals.var_dnm_dn2)), (((locals.var_tmf1_dn4 * locals.var_vgpdep_dlt__blk1146) * locals.var_dnm) + (assign53150_e81093 * locals.var_dnm_dn4)), (((locals.var_tmf1_dn5 * locals.var_vgpdep_dlt__blk1146) * locals.var_dnm) + (assign53150_e81093 * locals.var_dnm_dn5)), (((locals.var_tmf1_dn6 * locals.var_vgpdep_dlt__blk1146) * locals.var_dnm) + (assign53150_e81093 * locals.var_dnm_dn6)), (((locals.var_tmf1_dn7 * locals.var_vgpdep_dlt__blk1146) * locals.var_dnm) + (assign53150_e81093 * locals.var_dnm_dn7)), (((locals.var_tmf1_dn8 * locals.var_vgpdep_dlt__blk1146) * locals.var_dnm) + (assign53150_e81093 * locals.var_dnm_dn8)), (((locals.var_tmf1_dn9 * locals.var_vgpdep_dlt__blk1146) * locals.var_dnm) + (assign53150_e81093 * locals.var_dnm_dn9)), (((locals.var_tmf1_dn10 * locals.var_vgpdep_dlt__blk1146) * locals.var_dnm) + (assign53150_e81093 * locals.var_dnm_dn10)), (((locals.var_tmf1_dn11 * locals.var_vgpdep_dlt__blk1146) * locals.var_dnm) + (assign53150_e81093 * locals.var_dnm_dn11)), (((locals.var_tmf1_dn14 * locals.var_vgpdep_dlt__blk1146) * locals.var_dnm) + (assign53150_e81093 * locals.var_dnm_dn14)),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn11, locals.var_tmf0_dn14,)
    }
};
        locals.var_tmf0 = assign53150_e81097;
        locals.var_tmf0_dn0 = assign53150_e81097_d_n0;
        locals.var_tmf0_dn2 = assign53150_e81097_d_n2;
        locals.var_tmf0_dn4 = assign53150_e81097_d_n4;
        locals.var_tmf0_dn5 = assign53150_e81097_d_n5;
        locals.var_tmf0_dn6 = assign53150_e81097_d_n6;
        locals.var_tmf0_dn7 = assign53150_e81097_d_n7;
        locals.var_tmf0_dn8 = assign53150_e81097_d_n8;
        locals.var_tmf0_dn9 = assign53150_e81097_d_n9;
        locals.var_tmf0_dn10 = assign53150_e81097_d_n10;
        locals.var_tmf0_dn11 = assign53150_e81097_d_n11;
        locals.var_tmf0_dn14 = assign53150_e81097_d_n14;

        let (assign53160_e81119, assign53160_e81119_d_n0, assign53160_e81119_d_n2, assign53160_e81119_d_n4, assign53160_e81119_d_n5, assign53160_e81119_d_n6, assign53160_e81119_d_n7, assign53160_e81119_d_n8, assign53160_e81119_d_n9, assign53160_e81119_d_n10, assign53160_e81119_d_n11, assign53160_e81119_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_guard1346 != 0.0)) {
        let assign53160_e81113: f64 = (locals.var_vgpdep_dlt__blk1146 * locals.var_xmp);
        let assign53160_e81115: f64 = (assign53160_e81113 * locals.var_dnm);
        let assign53160_e81117: f64 = (assign53160_e81115 / locals.var_arg);
        (assign53160_e81117, ((((((locals.var_vgpdep_dlt__blk1146 * locals.var_xmp_dn0) * locals.var_dnm) + (assign53160_e81113 * locals.var_dnm_dn0)) * locals.var_arg) - (assign53160_e81115 * locals.var_arg_dn0)) / (locals.var_arg * locals.var_arg)), ((((((locals.var_vgpdep_dlt__blk1146 * locals.var_xmp_dn2) * locals.var_dnm) + (assign53160_e81113 * locals.var_dnm_dn2)) * locals.var_arg) - (assign53160_e81115 * locals.var_arg_dn2)) / (locals.var_arg * locals.var_arg)), ((((((locals.var_vgpdep_dlt__blk1146 * locals.var_xmp_dn4) * locals.var_dnm) + (assign53160_e81113 * locals.var_dnm_dn4)) * locals.var_arg) - (assign53160_e81115 * locals.var_arg_dn4)) / (locals.var_arg * locals.var_arg)), ((((((locals.var_vgpdep_dlt__blk1146 * locals.var_xmp_dn5) * locals.var_dnm) + (assign53160_e81113 * locals.var_dnm_dn5)) * locals.var_arg) - (assign53160_e81115 * locals.var_arg_dn5)) / (locals.var_arg * locals.var_arg)), ((((((locals.var_vgpdep_dlt__blk1146 * locals.var_xmp_dn6) * locals.var_dnm) + (assign53160_e81113 * locals.var_dnm_dn6)) * locals.var_arg) - (assign53160_e81115 * locals.var_arg_dn6)) / (locals.var_arg * locals.var_arg)), ((((((locals.var_vgpdep_dlt__blk1146 * locals.var_xmp_dn7) * locals.var_dnm) + (assign53160_e81113 * locals.var_dnm_dn7)) * locals.var_arg) - (assign53160_e81115 * locals.var_arg_dn7)) / (locals.var_arg * locals.var_arg)), ((((((locals.var_vgpdep_dlt__blk1146 * locals.var_xmp_dn8) * locals.var_dnm) + (assign53160_e81113 * locals.var_dnm_dn8)) * locals.var_arg) - (assign53160_e81115 * locals.var_arg_dn8)) / (locals.var_arg * locals.var_arg)), ((((((locals.var_vgpdep_dlt__blk1146 * locals.var_xmp_dn9) * locals.var_dnm) + (assign53160_e81113 * locals.var_dnm_dn9)) * locals.var_arg) - (assign53160_e81115 * locals.var_arg_dn9)) / (locals.var_arg * locals.var_arg)), ((((((locals.var_vgpdep_dlt__blk1146 * locals.var_xmp_dn10) * locals.var_dnm) + (assign53160_e81113 * locals.var_dnm_dn10)) * locals.var_arg) - (assign53160_e81115 * locals.var_arg_dn10)) / (locals.var_arg * locals.var_arg)), ((((((locals.var_vgpdep_dlt__blk1146 * locals.var_xmp_dn11) * locals.var_dnm) + (assign53160_e81113 * locals.var_dnm_dn11)) * locals.var_arg) - (assign53160_e81115 * locals.var_arg_dn11)) / (locals.var_arg * locals.var_arg)), ((((((locals.var_vgpdep_dlt__blk1146 * locals.var_xmp_dn14) * locals.var_dnm) + (assign53160_e81113 * locals.var_dnm_dn14)) * locals.var_arg) - (assign53160_e81115 * locals.var_arg_dn14)) / (locals.var_arg * locals.var_arg)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign53160_e81119;
        locals.var_t0_dn0 = assign53160_e81119_d_n0;
        locals.var_t0_dn2 = assign53160_e81119_d_n2;
        locals.var_t0_dn4 = assign53160_e81119_d_n4;
        locals.var_t0_dn5 = assign53160_e81119_d_n5;
        locals.var_t0_dn6 = assign53160_e81119_d_n6;
        locals.var_t0_dn7 = assign53160_e81119_d_n7;
        locals.var_t0_dn8 = assign53160_e81119_d_n8;
        locals.var_t0_dn9 = assign53160_e81119_d_n9;
        locals.var_t0_dn10 = assign53160_e81119_d_n10;
        locals.var_t0_dn11 = assign53160_e81119_d_n11;
        locals.var_t0_dn14 = assign53160_e81119_d_n14;

        let (assign53170_e81139, assign53170_e81139_d_n0, assign53170_e81139_d_n2, assign53170_e81139_d_n4, assign53170_e81139_d_n5, assign53170_e81139_d_n6, assign53170_e81139_d_n7, assign53170_e81139_d_n8, assign53170_e81139_d_n9, assign53170_e81139_d_n10, assign53170_e81139_d_n11, assign53170_e81139_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_guard1346 != 0.0)) {
        let assign53170_e81135: f64 = (-locals.var_vgpdep_dlt__blk1146);
        let assign53170_e81137: f64 = (assign53170_e81135 + locals.var_tmf0);
        (assign53170_e81137, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn11, locals.var_tmf0_dn14,)
    } else {
        (locals.var_ps0dep, locals.var_ps0dep_dn0, locals.var_ps0dep_dn2, locals.var_ps0dep_dn4, locals.var_ps0dep_dn5, locals.var_ps0dep_dn6, locals.var_ps0dep_dn7, locals.var_ps0dep_dn8, locals.var_ps0dep_dn9, locals.var_ps0dep_dn10, locals.var_ps0dep_dn11, locals.var_ps0dep_dn14,)
    }
};
        locals.var_ps0dep = assign53170_e81139;
        locals.var_ps0dep_dn0 = assign53170_e81139_d_n0;
        locals.var_ps0dep_dn2 = assign53170_e81139_d_n2;
        locals.var_ps0dep_dn4 = assign53170_e81139_d_n4;
        locals.var_ps0dep_dn5 = assign53170_e81139_d_n5;
        locals.var_ps0dep_dn6 = assign53170_e81139_d_n6;
        locals.var_ps0dep_dn7 = assign53170_e81139_d_n7;
        locals.var_ps0dep_dn8 = assign53170_e81139_d_n8;
        locals.var_ps0dep_dn9 = assign53170_e81139_d_n9;
        locals.var_ps0dep_dn10 = assign53170_e81139_d_n10;
        locals.var_ps0dep_dn11 = assign53170_e81139_d_n11;
        locals.var_ps0dep_dn14 = assign53170_e81139_d_n14;

        let (assign53180_e81155, assign53180_e81155_d_n0, assign53180_e81155_d_n2, assign53180_e81155_d_n4, assign53180_e81155_d_n5, assign53180_e81155_d_n6, assign53180_e81155_d_n7, assign53180_e81155_d_n8, assign53180_e81155_d_n9, assign53180_e81155_d_n10, assign53180_e81155_d_n11, assign53180_e81155_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_guard1346 != 0.0)) {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign53180_e81155;
        locals.var_t0_dn0 = assign53180_e81155_d_n0;
        locals.var_t0_dn2 = assign53180_e81155_d_n2;
        locals.var_t0_dn4 = assign53180_e81155_d_n4;
        locals.var_t0_dn5 = assign53180_e81155_d_n5;
        locals.var_t0_dn6 = assign53180_e81155_d_n6;
        locals.var_t0_dn7 = assign53180_e81155_d_n7;
        locals.var_t0_dn8 = assign53180_e81155_d_n8;
        locals.var_t0_dn9 = assign53180_e81155_d_n9;
        locals.var_t0_dn10 = assign53180_e81155_d_n10;
        locals.var_t0_dn11 = assign53180_e81155_d_n11;
        locals.var_t0_dn14 = assign53180_e81155_d_n14;

    }

    pub(super) fn stamp_transient_block_182(
        locals: &mut StampLocals,
    ) {
        let (assign53190_e81172, assign53190_e81172_d_n0, assign53190_e81172_d_n2, assign53190_e81172_d_n4, assign53190_e81172_d_n5, assign53190_e81172_d_n6, assign53190_e81172_d_n7, assign53190_e81172_d_n8, assign53190_e81172_d_n9, assign53190_e81172_d_n10, assign53190_e81172_d_n11, assign53190_e81172_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_guard1346 == 0.0)) {
        (locals.var_ps0_res, locals.var_ps0_res_dn0, locals.var_ps0_res_dn2, locals.var_ps0_res_dn4, locals.var_ps0_res_dn5, locals.var_ps0_res_dn6, locals.var_ps0_res_dn7, locals.var_ps0_res_dn8, locals.var_ps0_res_dn9, locals.var_ps0_res_dn10, locals.var_ps0_res_dn11, locals.var_ps0_res_dn14,)
    } else {
        (locals.var_ps0dep, locals.var_ps0dep_dn0, locals.var_ps0dep_dn2, locals.var_ps0dep_dn4, locals.var_ps0dep_dn5, locals.var_ps0dep_dn6, locals.var_ps0dep_dn7, locals.var_ps0dep_dn8, locals.var_ps0dep_dn9, locals.var_ps0dep_dn10, locals.var_ps0dep_dn11, locals.var_ps0dep_dn14,)
    }
};
        locals.var_ps0dep = assign53190_e81172;
        locals.var_ps0dep_dn0 = assign53190_e81172_d_n0;
        locals.var_ps0dep_dn2 = assign53190_e81172_d_n2;
        locals.var_ps0dep_dn4 = assign53190_e81172_d_n4;
        locals.var_ps0dep_dn5 = assign53190_e81172_d_n5;
        locals.var_ps0dep_dn6 = assign53190_e81172_d_n6;
        locals.var_ps0dep_dn7 = assign53190_e81172_d_n7;
        locals.var_ps0dep_dn8 = assign53190_e81172_d_n8;
        locals.var_ps0dep_dn9 = assign53190_e81172_d_n9;
        locals.var_ps0dep_dn10 = assign53190_e81172_d_n10;
        locals.var_ps0dep_dn11 = assign53190_e81172_d_n11;
        locals.var_ps0dep_dn14 = assign53190_e81172_d_n14;

        let (assign53200_e81189, assign53200_e81189_d_n0, assign53200_e81189_d_n2, assign53200_e81189_d_n4, assign53200_e81189_d_n5, assign53200_e81189_d_n6, assign53200_e81189_d_n7, assign53200_e81189_d_n8, assign53200_e81189_d_n9, assign53200_e81189_d_n10, assign53200_e81189_d_n11, assign53200_e81189_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_guard1346 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign53200_e81189;
        locals.var_t0_dn0 = assign53200_e81189_d_n0;
        locals.var_t0_dn2 = assign53200_e81189_d_n2;
        locals.var_t0_dn4 = assign53200_e81189_d_n4;
        locals.var_t0_dn5 = assign53200_e81189_d_n5;
        locals.var_t0_dn6 = assign53200_e81189_d_n6;
        locals.var_t0_dn7 = assign53200_e81189_d_n7;
        locals.var_t0_dn8 = assign53200_e81189_d_n8;
        locals.var_t0_dn9 = assign53200_e81189_d_n9;
        locals.var_t0_dn10 = assign53200_e81189_d_n10;
        locals.var_t0_dn11 = assign53200_e81189_d_n11;
        locals.var_t0_dn14 = assign53200_e81189_d_n14;

        let (assign53210_e81204, assign53210_e81204_d_n0, assign53210_e81204_d_n2, assign53210_e81204_d_n4, assign53210_e81204_d_n5, assign53210_e81204_d_n6, assign53210_e81204_d_n7, assign53210_e81204_d_n8, assign53210_e81204_d_n9, assign53210_e81204_d_n10, assign53210_e81204_d_n11, assign53210_e81204_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) {
        let assign53210_e81202: f64 = (-locals.var_ps0dep);
        (assign53210_e81202, (-locals.var_ps0dep_dn0), (-locals.var_ps0dep_dn2), (-locals.var_ps0dep_dn4), (-locals.var_ps0dep_dn5), (-locals.var_ps0dep_dn6), (-locals.var_ps0dep_dn7), (-locals.var_ps0dep_dn8), (-locals.var_ps0dep_dn9), (-locals.var_ps0dep_dn10), (-locals.var_ps0dep_dn11), (-locals.var_ps0dep_dn14),)
    } else {
        (locals.var_ps0dep, locals.var_ps0dep_dn0, locals.var_ps0dep_dn2, locals.var_ps0dep_dn4, locals.var_ps0dep_dn5, locals.var_ps0dep_dn6, locals.var_ps0dep_dn7, locals.var_ps0dep_dn8, locals.var_ps0dep_dn9, locals.var_ps0dep_dn10, locals.var_ps0dep_dn11, locals.var_ps0dep_dn14,)
    }
};
        locals.var_ps0dep = assign53210_e81204;
        locals.var_ps0dep_dn0 = assign53210_e81204_d_n0;
        locals.var_ps0dep_dn2 = assign53210_e81204_d_n2;
        locals.var_ps0dep_dn4 = assign53210_e81204_d_n4;
        locals.var_ps0dep_dn5 = assign53210_e81204_d_n5;
        locals.var_ps0dep_dn6 = assign53210_e81204_d_n6;
        locals.var_ps0dep_dn7 = assign53210_e81204_d_n7;
        locals.var_ps0dep_dn8 = assign53210_e81204_d_n8;
        locals.var_ps0dep_dn9 = assign53210_e81204_d_n9;
        locals.var_ps0dep_dn10 = assign53210_e81204_d_n10;
        locals.var_ps0dep_dn11 = assign53210_e81204_d_n11;
        locals.var_ps0dep_dn14 = assign53210_e81204_d_n14;

        let (assign53220_e81226, assign53220_e81226_d_n0, assign53220_e81226_d_n2, assign53220_e81226_d_n4, assign53220_e81226_d_n5, assign53220_e81226_d_n6, assign53220_e81226_d_n7, assign53220_e81226_d_n8, assign53220_e81226_d_n9, assign53220_e81226_d_n10, assign53220_e81226_d_n11, assign53220_e81226_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) {
        let assign53220_e81218: f64 = (locals.var_q_ndepm__blk1137 * locals.var_tnp__blk1152);
        let assign53220_e81220: f64 = (assign53220_e81218 * locals.var_tnp__blk1152);
        let assign53220_e81222: f64 = (assign53220_e81220 / 2.0);
        let assign53220_e81224: f64 = (assign53220_e81222 / 1.034943e-10);
        (assign53220_e81224, ((((((locals.var_q_ndepm__blk1137_dn0 * locals.var_tnp__blk1152) + (locals.var_q_ndepm__blk1137 * locals.var_tnp__blk1152_dn0)) * locals.var_tnp__blk1152) + (assign53220_e81218 * locals.var_tnp__blk1152_dn0)) / 2.0) / 1.034943e-10), ((((((locals.var_q_ndepm__blk1137_dn2 * locals.var_tnp__blk1152) + (locals.var_q_ndepm__blk1137 * locals.var_tnp__blk1152_dn2)) * locals.var_tnp__blk1152) + (assign53220_e81218 * locals.var_tnp__blk1152_dn2)) / 2.0) / 1.034943e-10), ((((((locals.var_q_ndepm__blk1137_dn4 * locals.var_tnp__blk1152) + (locals.var_q_ndepm__blk1137 * locals.var_tnp__blk1152_dn4)) * locals.var_tnp__blk1152) + (assign53220_e81218 * locals.var_tnp__blk1152_dn4)) / 2.0) / 1.034943e-10), ((((((locals.var_q_ndepm__blk1137_dn5 * locals.var_tnp__blk1152) + (locals.var_q_ndepm__blk1137 * locals.var_tnp__blk1152_dn5)) * locals.var_tnp__blk1152) + (assign53220_e81218 * locals.var_tnp__blk1152_dn5)) / 2.0) / 1.034943e-10), ((((((locals.var_q_ndepm__blk1137_dn6 * locals.var_tnp__blk1152) + (locals.var_q_ndepm__blk1137 * locals.var_tnp__blk1152_dn6)) * locals.var_tnp__blk1152) + (assign53220_e81218 * locals.var_tnp__blk1152_dn6)) / 2.0) / 1.034943e-10), ((((((locals.var_q_ndepm__blk1137_dn7 * locals.var_tnp__blk1152) + (locals.var_q_ndepm__blk1137 * locals.var_tnp__blk1152_dn7)) * locals.var_tnp__blk1152) + (assign53220_e81218 * locals.var_tnp__blk1152_dn7)) / 2.0) / 1.034943e-10), ((((((locals.var_q_ndepm__blk1137_dn8 * locals.var_tnp__blk1152) + (locals.var_q_ndepm__blk1137 * locals.var_tnp__blk1152_dn8)) * locals.var_tnp__blk1152) + (assign53220_e81218 * locals.var_tnp__blk1152_dn8)) / 2.0) / 1.034943e-10), ((((((locals.var_q_ndepm__blk1137_dn9 * locals.var_tnp__blk1152) + (locals.var_q_ndepm__blk1137 * locals.var_tnp__blk1152_dn9)) * locals.var_tnp__blk1152) + (assign53220_e81218 * locals.var_tnp__blk1152_dn9)) / 2.0) / 1.034943e-10), ((((((locals.var_q_ndepm__blk1137_dn10 * locals.var_tnp__blk1152) + (locals.var_q_ndepm__blk1137 * locals.var_tnp__blk1152_dn10)) * locals.var_tnp__blk1152) + (assign53220_e81218 * locals.var_tnp__blk1152_dn10)) / 2.0) / 1.034943e-10), ((((((locals.var_q_ndepm__blk1137_dn11 * locals.var_tnp__blk1152) + (locals.var_q_ndepm__blk1137 * locals.var_tnp__blk1152_dn11)) * locals.var_tnp__blk1152) + (assign53220_e81218 * locals.var_tnp__blk1152_dn11)) / 2.0) / 1.034943e-10), ((((((locals.var_q_ndepm__blk1137_dn14 * locals.var_tnp__blk1152) + (locals.var_q_ndepm__blk1137 * locals.var_tnp__blk1152_dn14)) * locals.var_tnp__blk1152) + (assign53220_e81218 * locals.var_tnp__blk1152_dn14)) / 2.0) / 1.034943e-10),)
    } else {
        (locals.var_dphi_sb__blk1324, locals.var_dphi_sb__blk1324_dn0, locals.var_dphi_sb__blk1324_dn2, locals.var_dphi_sb__blk1324_dn4, locals.var_dphi_sb__blk1324_dn5, locals.var_dphi_sb__blk1324_dn6, locals.var_dphi_sb__blk1324_dn7, locals.var_dphi_sb__blk1324_dn8, locals.var_dphi_sb__blk1324_dn9, locals.var_dphi_sb__blk1324_dn10, locals.var_dphi_sb__blk1324_dn11, locals.var_dphi_sb__blk1324_dn14,)
    }
};
        locals.var_dphi_sb__blk1324 = assign53220_e81226;
        locals.var_dphi_sb__blk1324_dn0 = assign53220_e81226_d_n0;
        locals.var_dphi_sb__blk1324_dn2 = assign53220_e81226_d_n2;
        locals.var_dphi_sb__blk1324_dn4 = assign53220_e81226_d_n4;
        locals.var_dphi_sb__blk1324_dn5 = assign53220_e81226_d_n5;
        locals.var_dphi_sb__blk1324_dn6 = assign53220_e81226_d_n6;
        locals.var_dphi_sb__blk1324_dn7 = assign53220_e81226_d_n7;
        locals.var_dphi_sb__blk1324_dn8 = assign53220_e81226_d_n8;
        locals.var_dphi_sb__blk1324_dn9 = assign53220_e81226_d_n9;
        locals.var_dphi_sb__blk1324_dn10 = assign53220_e81226_d_n10;
        locals.var_dphi_sb__blk1324_dn11 = assign53220_e81226_d_n11;
        locals.var_dphi_sb__blk1324_dn14 = assign53220_e81226_d_n14;

        let (assign53230_e81247, assign53230_e81247_d_n0, assign53230_e81247_d_n2, assign53230_e81247_d_n4, assign53230_e81247_d_n5, assign53230_e81247_d_n6, assign53230_e81247_d_n7, assign53230_e81247_d_n8, assign53230_e81247_d_n9, assign53230_e81247_d_n10, assign53230_e81247_d_n11, assign53230_e81247_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) {
        let assign53230_e81241: f64 = (2.0 * locals.var_beta);
        let assign53230_e81243: f64 = (assign53230_e81241 * locals.var_dphi_sb__blk1324);
        let assign53230_e81244: f64 = (assign53230_e81243).sqrt();
        let assign53230_e81245: f64 = (locals.var_wdepsubsl * assign53230_e81244);
        (assign53230_e81245, ((locals.var_wdepsubsl_dn0 * assign53230_e81244) + (locals.var_wdepsubsl * ((((2.0 * locals.var_beta_dn0) * locals.var_dphi_sb__blk1324) + (assign53230_e81241 * locals.var_dphi_sb__blk1324_dn0)) / (2.0 * assign53230_e81244)))), ((locals.var_wdepsubsl_dn2 * assign53230_e81244) + (locals.var_wdepsubsl * ((((2.0 * locals.var_beta_dn2) * locals.var_dphi_sb__blk1324) + (assign53230_e81241 * locals.var_dphi_sb__blk1324_dn2)) / (2.0 * assign53230_e81244)))), ((locals.var_wdepsubsl_dn4 * assign53230_e81244) + (locals.var_wdepsubsl * ((((2.0 * locals.var_beta_dn4) * locals.var_dphi_sb__blk1324) + (assign53230_e81241 * locals.var_dphi_sb__blk1324_dn4)) / (2.0 * assign53230_e81244)))), ((locals.var_wdepsubsl_dn5 * assign53230_e81244) + (locals.var_wdepsubsl * ((((2.0 * locals.var_beta_dn5) * locals.var_dphi_sb__blk1324) + (assign53230_e81241 * locals.var_dphi_sb__blk1324_dn5)) / (2.0 * assign53230_e81244)))), ((locals.var_wdepsubsl_dn6 * assign53230_e81244) + (locals.var_wdepsubsl * ((((2.0 * locals.var_beta_dn6) * locals.var_dphi_sb__blk1324) + (assign53230_e81241 * locals.var_dphi_sb__blk1324_dn6)) / (2.0 * assign53230_e81244)))), ((locals.var_wdepsubsl_dn7 * assign53230_e81244) + (locals.var_wdepsubsl * ((((2.0 * locals.var_beta_dn7) * locals.var_dphi_sb__blk1324) + (assign53230_e81241 * locals.var_dphi_sb__blk1324_dn7)) / (2.0 * assign53230_e81244)))), ((locals.var_wdepsubsl_dn8 * assign53230_e81244) + (locals.var_wdepsubsl * ((((2.0 * locals.var_beta_dn8) * locals.var_dphi_sb__blk1324) + (assign53230_e81241 * locals.var_dphi_sb__blk1324_dn8)) / (2.0 * assign53230_e81244)))), ((locals.var_wdepsubsl_dn9 * assign53230_e81244) + (locals.var_wdepsubsl * ((((2.0 * locals.var_beta_dn9) * locals.var_dphi_sb__blk1324) + (assign53230_e81241 * locals.var_dphi_sb__blk1324_dn9)) / (2.0 * assign53230_e81244)))), ((locals.var_wdepsubsl_dn10 * assign53230_e81244) + (locals.var_wdepsubsl * ((((2.0 * locals.var_beta_dn10) * locals.var_dphi_sb__blk1324) + (assign53230_e81241 * locals.var_dphi_sb__blk1324_dn10)) / (2.0 * assign53230_e81244)))), ((locals.var_wdepsubsl_dn11 * assign53230_e81244) + (locals.var_wdepsubsl * ((((2.0 * locals.var_beta_dn11) * locals.var_dphi_sb__blk1324) + (assign53230_e81241 * locals.var_dphi_sb__blk1324_dn11)) / (2.0 * assign53230_e81244)))), ((locals.var_wdepsubsl_dn14 * assign53230_e81244) + (locals.var_wdepsubsl * ((((2.0 * locals.var_beta_dn14) * locals.var_dphi_sb__blk1324) + (assign53230_e81241 * locals.var_dphi_sb__blk1324_dn14)) / (2.0 * assign53230_e81244)))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign53230_e81247;
        locals.var_t0_dn0 = assign53230_e81247_d_n0;
        locals.var_t0_dn2 = assign53230_e81247_d_n2;
        locals.var_t0_dn4 = assign53230_e81247_d_n4;
        locals.var_t0_dn5 = assign53230_e81247_d_n5;
        locals.var_t0_dn6 = assign53230_e81247_d_n6;
        locals.var_t0_dn7 = assign53230_e81247_d_n7;
        locals.var_t0_dn8 = assign53230_e81247_d_n8;
        locals.var_t0_dn9 = assign53230_e81247_d_n9;
        locals.var_t0_dn10 = assign53230_e81247_d_n10;
        locals.var_t0_dn11 = assign53230_e81247_d_n11;
        locals.var_t0_dn14 = assign53230_e81247_d_n14;

        let (assign53240_e81268, assign53240_e81268_d_n0, assign53240_e81268_d_n2, assign53240_e81268_d_n4, assign53240_e81268_d_n5, assign53240_e81268_d_n6, assign53240_e81268_d_n7, assign53240_e81268_d_n8, assign53240_e81268_d_n9, assign53240_e81268_d_n10, assign53240_e81268_d_n11, assign53240_e81268_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) {
        let assign53240_e81260: f64 = (locals.var_t0).exp();
        let assign53240_e81262: f64 = (-locals.var_t0);
        let assign53240_e81263: f64 = (assign53240_e81262).exp();
        let assign53240_e81264: f64 = (assign53240_e81260 + assign53240_e81263);
        let assign53240_e81266: f64 = (assign53240_e81264 / 2.0);
        (assign53240_e81266, (((assign53240_e81260 * locals.var_t0_dn0) + (assign53240_e81263 * (-locals.var_t0_dn0))) / 2.0), (((assign53240_e81260 * locals.var_t0_dn2) + (assign53240_e81263 * (-locals.var_t0_dn2))) / 2.0), (((assign53240_e81260 * locals.var_t0_dn4) + (assign53240_e81263 * (-locals.var_t0_dn4))) / 2.0), (((assign53240_e81260 * locals.var_t0_dn5) + (assign53240_e81263 * (-locals.var_t0_dn5))) / 2.0), (((assign53240_e81260 * locals.var_t0_dn6) + (assign53240_e81263 * (-locals.var_t0_dn6))) / 2.0), (((assign53240_e81260 * locals.var_t0_dn7) + (assign53240_e81263 * (-locals.var_t0_dn7))) / 2.0), (((assign53240_e81260 * locals.var_t0_dn8) + (assign53240_e81263 * (-locals.var_t0_dn8))) / 2.0), (((assign53240_e81260 * locals.var_t0_dn9) + (assign53240_e81263 * (-locals.var_t0_dn9))) / 2.0), (((assign53240_e81260 * locals.var_t0_dn10) + (assign53240_e81263 * (-locals.var_t0_dn10))) / 2.0), (((assign53240_e81260 * locals.var_t0_dn11) + (assign53240_e81263 * (-locals.var_t0_dn11))) / 2.0), (((assign53240_e81260 * locals.var_t0_dn14) + (assign53240_e81263 * (-locals.var_t0_dn14))) / 2.0),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign53240_e81268;
        locals.var_t1_dn0 = assign53240_e81268_d_n0;
        locals.var_t1_dn2 = assign53240_e81268_d_n2;
        locals.var_t1_dn4 = assign53240_e81268_d_n4;
        locals.var_t1_dn5 = assign53240_e81268_d_n5;
        locals.var_t1_dn6 = assign53240_e81268_d_n6;
        locals.var_t1_dn7 = assign53240_e81268_d_n7;
        locals.var_t1_dn8 = assign53240_e81268_d_n8;
        locals.var_t1_dn9 = assign53240_e81268_d_n9;
        locals.var_t1_dn10 = assign53240_e81268_d_n10;
        locals.var_t1_dn11 = assign53240_e81268_d_n11;
        locals.var_t1_dn14 = assign53240_e81268_d_n14;

        let assign53250_e81270: f64 = (locals.var_t0).abs();
        let assign53250_e81272: f64 = if assign53250_e81270 > 0.0001 { 1.0 } else { 0.0 };
        locals.var_guard1352 = assign53250_e81272;

        let (assign53260_e81291, assign53260_e81291_d_n0, assign53260_e81291_d_n2, assign53260_e81291_d_n4, assign53260_e81291_d_n5, assign53260_e81291_d_n6, assign53260_e81291_d_n7, assign53260_e81291_d_n8, assign53260_e81291_d_n9, assign53260_e81291_d_n10, assign53260_e81291_d_n11, assign53260_e81291_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_guard1352 != 0.0)) {
        let assign53260_e81287: f64 = (locals.var_t1).ln();
        let assign53260_e81289: f64 = (assign53260_e81287 / locals.var_dphi_sb__blk1324);
        (assign53260_e81289, ((((locals.var_t1_dn0 / locals.var_t1) * locals.var_dphi_sb__blk1324) - (assign53260_e81287 * locals.var_dphi_sb__blk1324_dn0)) / (locals.var_dphi_sb__blk1324 * locals.var_dphi_sb__blk1324)), ((((locals.var_t1_dn2 / locals.var_t1) * locals.var_dphi_sb__blk1324) - (assign53260_e81287 * locals.var_dphi_sb__blk1324_dn2)) / (locals.var_dphi_sb__blk1324 * locals.var_dphi_sb__blk1324)), ((((locals.var_t1_dn4 / locals.var_t1) * locals.var_dphi_sb__blk1324) - (assign53260_e81287 * locals.var_dphi_sb__blk1324_dn4)) / (locals.var_dphi_sb__blk1324 * locals.var_dphi_sb__blk1324)), ((((locals.var_t1_dn5 / locals.var_t1) * locals.var_dphi_sb__blk1324) - (assign53260_e81287 * locals.var_dphi_sb__blk1324_dn5)) / (locals.var_dphi_sb__blk1324 * locals.var_dphi_sb__blk1324)), ((((locals.var_t1_dn6 / locals.var_t1) * locals.var_dphi_sb__blk1324) - (assign53260_e81287 * locals.var_dphi_sb__blk1324_dn6)) / (locals.var_dphi_sb__blk1324 * locals.var_dphi_sb__blk1324)), ((((locals.var_t1_dn7 / locals.var_t1) * locals.var_dphi_sb__blk1324) - (assign53260_e81287 * locals.var_dphi_sb__blk1324_dn7)) / (locals.var_dphi_sb__blk1324 * locals.var_dphi_sb__blk1324)), ((((locals.var_t1_dn8 / locals.var_t1) * locals.var_dphi_sb__blk1324) - (assign53260_e81287 * locals.var_dphi_sb__blk1324_dn8)) / (locals.var_dphi_sb__blk1324 * locals.var_dphi_sb__blk1324)), ((((locals.var_t1_dn9 / locals.var_t1) * locals.var_dphi_sb__blk1324) - (assign53260_e81287 * locals.var_dphi_sb__blk1324_dn9)) / (locals.var_dphi_sb__blk1324 * locals.var_dphi_sb__blk1324)), ((((locals.var_t1_dn10 / locals.var_t1) * locals.var_dphi_sb__blk1324) - (assign53260_e81287 * locals.var_dphi_sb__blk1324_dn10)) / (locals.var_dphi_sb__blk1324 * locals.var_dphi_sb__blk1324)), ((((locals.var_t1_dn11 / locals.var_t1) * locals.var_dphi_sb__blk1324) - (assign53260_e81287 * locals.var_dphi_sb__blk1324_dn11)) / (locals.var_dphi_sb__blk1324 * locals.var_dphi_sb__blk1324)), ((((locals.var_t1_dn14 / locals.var_t1) * locals.var_dphi_sb__blk1324) - (assign53260_e81287 * locals.var_dphi_sb__blk1324_dn14)) / (locals.var_dphi_sb__blk1324 * locals.var_dphi_sb__blk1324)),)
    } else {
        (locals.var_c_sb__blk1325, locals.var_c_sb__blk1325_dn0, locals.var_c_sb__blk1325_dn2, locals.var_c_sb__blk1325_dn4, locals.var_c_sb__blk1325_dn5, locals.var_c_sb__blk1325_dn6, locals.var_c_sb__blk1325_dn7, locals.var_c_sb__blk1325_dn8, locals.var_c_sb__blk1325_dn9, locals.var_c_sb__blk1325_dn10, locals.var_c_sb__blk1325_dn11, locals.var_c_sb__blk1325_dn14,)
    }
};
        locals.var_c_sb__blk1325 = assign53260_e81291;
        locals.var_c_sb__blk1325_dn0 = assign53260_e81291_d_n0;
        locals.var_c_sb__blk1325_dn2 = assign53260_e81291_d_n2;
        locals.var_c_sb__blk1325_dn4 = assign53260_e81291_d_n4;
        locals.var_c_sb__blk1325_dn5 = assign53260_e81291_d_n5;
        locals.var_c_sb__blk1325_dn6 = assign53260_e81291_d_n6;
        locals.var_c_sb__blk1325_dn7 = assign53260_e81291_d_n7;
        locals.var_c_sb__blk1325_dn8 = assign53260_e81291_d_n8;
        locals.var_c_sb__blk1325_dn9 = assign53260_e81291_d_n9;
        locals.var_c_sb__blk1325_dn10 = assign53260_e81291_d_n10;
        locals.var_c_sb__blk1325_dn11 = assign53260_e81291_d_n11;
        locals.var_c_sb__blk1325_dn14 = assign53260_e81291_d_n14;

        let (assign53270_e81320, assign53270_e81320_d_n0, assign53270_e81320_d_n2, assign53270_e81320_d_n4, assign53270_e81320_d_n5, assign53270_e81320_d_n6, assign53270_e81320_d_n7, assign53270_e81320_d_n8, assign53270_e81320_d_n9, assign53270_e81320_d_n10, assign53270_e81320_d_n11, assign53270_e81320_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_guard1352 == 0.0)) {
        let assign53270_e81308: f64 = (locals.var_wdepsubsl * locals.var_wdepsubsl);
        let assign53270_e81310: f64 = (assign53270_e81308 * locals.var_beta);
        let assign53270_e81314: f64 = (0.1666666666666667 * locals.var_t0);
        let assign53270_e81316: f64 = (assign53270_e81314 * locals.var_t0);
        let assign53270_e81317: f64 = (1.0 - assign53270_e81316);
        let assign53270_e81318: f64 = (assign53270_e81310 * assign53270_e81317);
        (assign53270_e81318, ((((((locals.var_wdepsubsl_dn0 * locals.var_wdepsubsl) + (locals.var_wdepsubsl * locals.var_wdepsubsl_dn0)) * locals.var_beta) + (assign53270_e81308 * locals.var_beta_dn0)) * assign53270_e81317) + (assign53270_e81310 * (-(((0.1666666666666667 * locals.var_t0_dn0) * locals.var_t0) + (assign53270_e81314 * locals.var_t0_dn0))))), ((((((locals.var_wdepsubsl_dn2 * locals.var_wdepsubsl) + (locals.var_wdepsubsl * locals.var_wdepsubsl_dn2)) * locals.var_beta) + (assign53270_e81308 * locals.var_beta_dn2)) * assign53270_e81317) + (assign53270_e81310 * (-(((0.1666666666666667 * locals.var_t0_dn2) * locals.var_t0) + (assign53270_e81314 * locals.var_t0_dn2))))), ((((((locals.var_wdepsubsl_dn4 * locals.var_wdepsubsl) + (locals.var_wdepsubsl * locals.var_wdepsubsl_dn4)) * locals.var_beta) + (assign53270_e81308 * locals.var_beta_dn4)) * assign53270_e81317) + (assign53270_e81310 * (-(((0.1666666666666667 * locals.var_t0_dn4) * locals.var_t0) + (assign53270_e81314 * locals.var_t0_dn4))))), ((((((locals.var_wdepsubsl_dn5 * locals.var_wdepsubsl) + (locals.var_wdepsubsl * locals.var_wdepsubsl_dn5)) * locals.var_beta) + (assign53270_e81308 * locals.var_beta_dn5)) * assign53270_e81317) + (assign53270_e81310 * (-(((0.1666666666666667 * locals.var_t0_dn5) * locals.var_t0) + (assign53270_e81314 * locals.var_t0_dn5))))), ((((((locals.var_wdepsubsl_dn6 * locals.var_wdepsubsl) + (locals.var_wdepsubsl * locals.var_wdepsubsl_dn6)) * locals.var_beta) + (assign53270_e81308 * locals.var_beta_dn6)) * assign53270_e81317) + (assign53270_e81310 * (-(((0.1666666666666667 * locals.var_t0_dn6) * locals.var_t0) + (assign53270_e81314 * locals.var_t0_dn6))))), ((((((locals.var_wdepsubsl_dn7 * locals.var_wdepsubsl) + (locals.var_wdepsubsl * locals.var_wdepsubsl_dn7)) * locals.var_beta) + (assign53270_e81308 * locals.var_beta_dn7)) * assign53270_e81317) + (assign53270_e81310 * (-(((0.1666666666666667 * locals.var_t0_dn7) * locals.var_t0) + (assign53270_e81314 * locals.var_t0_dn7))))), ((((((locals.var_wdepsubsl_dn8 * locals.var_wdepsubsl) + (locals.var_wdepsubsl * locals.var_wdepsubsl_dn8)) * locals.var_beta) + (assign53270_e81308 * locals.var_beta_dn8)) * assign53270_e81317) + (assign53270_e81310 * (-(((0.1666666666666667 * locals.var_t0_dn8) * locals.var_t0) + (assign53270_e81314 * locals.var_t0_dn8))))), ((((((locals.var_wdepsubsl_dn9 * locals.var_wdepsubsl) + (locals.var_wdepsubsl * locals.var_wdepsubsl_dn9)) * locals.var_beta) + (assign53270_e81308 * locals.var_beta_dn9)) * assign53270_e81317) + (assign53270_e81310 * (-(((0.1666666666666667 * locals.var_t0_dn9) * locals.var_t0) + (assign53270_e81314 * locals.var_t0_dn9))))), ((((((locals.var_wdepsubsl_dn10 * locals.var_wdepsubsl) + (locals.var_wdepsubsl * locals.var_wdepsubsl_dn10)) * locals.var_beta) + (assign53270_e81308 * locals.var_beta_dn10)) * assign53270_e81317) + (assign53270_e81310 * (-(((0.1666666666666667 * locals.var_t0_dn10) * locals.var_t0) + (assign53270_e81314 * locals.var_t0_dn10))))), ((((((locals.var_wdepsubsl_dn11 * locals.var_wdepsubsl) + (locals.var_wdepsubsl * locals.var_wdepsubsl_dn11)) * locals.var_beta) + (assign53270_e81308 * locals.var_beta_dn11)) * assign53270_e81317) + (assign53270_e81310 * (-(((0.1666666666666667 * locals.var_t0_dn11) * locals.var_t0) + (assign53270_e81314 * locals.var_t0_dn11))))), ((((((locals.var_wdepsubsl_dn14 * locals.var_wdepsubsl) + (locals.var_wdepsubsl * locals.var_wdepsubsl_dn14)) * locals.var_beta) + (assign53270_e81308 * locals.var_beta_dn14)) * assign53270_e81317) + (assign53270_e81310 * (-(((0.1666666666666667 * locals.var_t0_dn14) * locals.var_t0) + (assign53270_e81314 * locals.var_t0_dn14))))),)
    } else {
        (locals.var_c_sb__blk1325, locals.var_c_sb__blk1325_dn0, locals.var_c_sb__blk1325_dn2, locals.var_c_sb__blk1325_dn4, locals.var_c_sb__blk1325_dn5, locals.var_c_sb__blk1325_dn6, locals.var_c_sb__blk1325_dn7, locals.var_c_sb__blk1325_dn8, locals.var_c_sb__blk1325_dn9, locals.var_c_sb__blk1325_dn10, locals.var_c_sb__blk1325_dn11, locals.var_c_sb__blk1325_dn14,)
    }
};
        locals.var_c_sb__blk1325 = assign53270_e81320;
        locals.var_c_sb__blk1325_dn0 = assign53270_e81320_d_n0;
        locals.var_c_sb__blk1325_dn2 = assign53270_e81320_d_n2;
        locals.var_c_sb__blk1325_dn4 = assign53270_e81320_d_n4;
        locals.var_c_sb__blk1325_dn5 = assign53270_e81320_d_n5;
        locals.var_c_sb__blk1325_dn6 = assign53270_e81320_d_n6;
        locals.var_c_sb__blk1325_dn7 = assign53270_e81320_d_n7;
        locals.var_c_sb__blk1325_dn8 = assign53270_e81320_d_n8;
        locals.var_c_sb__blk1325_dn9 = assign53270_e81320_d_n9;
        locals.var_c_sb__blk1325_dn10 = assign53270_e81320_d_n10;
        locals.var_c_sb__blk1325_dn11 = assign53270_e81320_d_n11;
        locals.var_c_sb__blk1325_dn14 = assign53270_e81320_d_n14;

        let (assign53280_e81336, assign53280_e81336_d_n0, assign53280_e81336_d_n2, assign53280_e81336_d_n4, assign53280_e81336_d_n5, assign53280_e81336_d_n6, assign53280_e81336_d_n7, assign53280_e81336_d_n8, assign53280_e81336_d_n9, assign53280_e81336_d_n10, assign53280_e81336_d_n11, assign53280_e81336_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) {
        let assign53280_e81334: f64 = (locals.var_c_sb__blk1325 * locals.var_ps0dep);
        (assign53280_e81334, ((locals.var_c_sb__blk1325_dn0 * locals.var_ps0dep) + (locals.var_c_sb__blk1325 * locals.var_ps0dep_dn0)), ((locals.var_c_sb__blk1325_dn2 * locals.var_ps0dep) + (locals.var_c_sb__blk1325 * locals.var_ps0dep_dn2)), ((locals.var_c_sb__blk1325_dn4 * locals.var_ps0dep) + (locals.var_c_sb__blk1325 * locals.var_ps0dep_dn4)), ((locals.var_c_sb__blk1325_dn5 * locals.var_ps0dep) + (locals.var_c_sb__blk1325 * locals.var_ps0dep_dn5)), ((locals.var_c_sb__blk1325_dn6 * locals.var_ps0dep) + (locals.var_c_sb__blk1325 * locals.var_ps0dep_dn6)), ((locals.var_c_sb__blk1325_dn7 * locals.var_ps0dep) + (locals.var_c_sb__blk1325 * locals.var_ps0dep_dn7)), ((locals.var_c_sb__blk1325_dn8 * locals.var_ps0dep) + (locals.var_c_sb__blk1325 * locals.var_ps0dep_dn8)), ((locals.var_c_sb__blk1325_dn9 * locals.var_ps0dep) + (locals.var_c_sb__blk1325 * locals.var_ps0dep_dn9)), ((locals.var_c_sb__blk1325_dn10 * locals.var_ps0dep) + (locals.var_c_sb__blk1325 * locals.var_ps0dep_dn10)), ((locals.var_c_sb__blk1325_dn11 * locals.var_ps0dep) + (locals.var_c_sb__blk1325 * locals.var_ps0dep_dn11)), ((locals.var_c_sb__blk1325_dn14 * locals.var_ps0dep) + (locals.var_c_sb__blk1325 * locals.var_ps0dep_dn14)),)
    } else {
        (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn4, locals.var_tx_dn5, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn8, locals.var_tx_dn9, locals.var_tx_dn10, locals.var_tx_dn11, locals.var_tx_dn14,)
    }
};
        locals.var_tx = assign53280_e81336;
        locals.var_tx_dn0 = assign53280_e81336_d_n0;
        locals.var_tx_dn2 = assign53280_e81336_d_n2;
        locals.var_tx_dn4 = assign53280_e81336_d_n4;
        locals.var_tx_dn5 = assign53280_e81336_d_n5;
        locals.var_tx_dn6 = assign53280_e81336_d_n6;
        locals.var_tx_dn7 = assign53280_e81336_d_n7;
        locals.var_tx_dn8 = assign53280_e81336_d_n8;
        locals.var_tx_dn9 = assign53280_e81336_d_n9;
        locals.var_tx_dn10 = assign53280_e81336_d_n10;
        locals.var_tx_dn11 = assign53280_e81336_d_n11;
        locals.var_tx_dn14 = assign53280_e81336_d_n14;

        let assign53290_e81339: f64 = if locals.var_tx > 500.0 { 1.0 } else { 0.0 };
        locals.var_guard1353 = assign53290_e81339;

        let (assign53300_e81357, assign53300_e81357_d_n0, assign53300_e81357_d_n2, assign53300_e81357_d_n4, assign53300_e81357_d_n5, assign53300_e81357_d_n6, assign53300_e81357_d_n7, assign53300_e81357_d_n8, assign53300_e81357_d_n9, assign53300_e81357_d_n10, assign53300_e81357_d_n11, assign53300_e81357_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_guard1353 != 0.0)) {
        let assign53300_e81355: f64 = (locals.var_ps0dep - locals.var_dphi_sb__blk1324);
        (assign53300_e81355, (locals.var_ps0dep_dn0 - locals.var_dphi_sb__blk1324_dn0), (locals.var_ps0dep_dn2 - locals.var_dphi_sb__blk1324_dn2), (locals.var_ps0dep_dn4 - locals.var_dphi_sb__blk1324_dn4), (locals.var_ps0dep_dn5 - locals.var_dphi_sb__blk1324_dn5), (locals.var_ps0dep_dn6 - locals.var_dphi_sb__blk1324_dn6), (locals.var_ps0dep_dn7 - locals.var_dphi_sb__blk1324_dn7), (locals.var_ps0dep_dn8 - locals.var_dphi_sb__blk1324_dn8), (locals.var_ps0dep_dn9 - locals.var_dphi_sb__blk1324_dn9), (locals.var_ps0dep_dn10 - locals.var_dphi_sb__blk1324_dn10), (locals.var_ps0dep_dn11 - locals.var_dphi_sb__blk1324_dn11), (locals.var_ps0dep_dn14 - locals.var_dphi_sb__blk1324_dn14),)
    } else {
        (locals.var_pb0dep__blk1169, locals.var_pb0dep__blk1169_dn0, locals.var_pb0dep__blk1169_dn2, locals.var_pb0dep__blk1169_dn4, locals.var_pb0dep__blk1169_dn5, locals.var_pb0dep__blk1169_dn6, locals.var_pb0dep__blk1169_dn7, locals.var_pb0dep__blk1169_dn8, locals.var_pb0dep__blk1169_dn9, locals.var_pb0dep__blk1169_dn10, locals.var_pb0dep__blk1169_dn11, locals.var_pb0dep__blk1169_dn14,)
    }
};
        locals.var_pb0dep__blk1169 = assign53300_e81357;
        locals.var_pb0dep__blk1169_dn0 = assign53300_e81357_d_n0;
        locals.var_pb0dep__blk1169_dn2 = assign53300_e81357_d_n2;
        locals.var_pb0dep__blk1169_dn4 = assign53300_e81357_d_n4;
        locals.var_pb0dep__blk1169_dn5 = assign53300_e81357_d_n5;
        locals.var_pb0dep__blk1169_dn6 = assign53300_e81357_d_n6;
        locals.var_pb0dep__blk1169_dn7 = assign53300_e81357_d_n7;
        locals.var_pb0dep__blk1169_dn8 = assign53300_e81357_d_n8;
        locals.var_pb0dep__blk1169_dn9 = assign53300_e81357_d_n9;
        locals.var_pb0dep__blk1169_dn10 = assign53300_e81357_d_n10;
        locals.var_pb0dep__blk1169_dn11 = assign53300_e81357_d_n11;
        locals.var_pb0dep__blk1169_dn14 = assign53300_e81357_d_n14;

        let (assign53310_e81378, assign53310_e81378_d_n0, assign53310_e81378_d_n2, assign53310_e81378_d_n4, assign53310_e81378_d_n5, assign53310_e81378_d_n6, assign53310_e81378_d_n7, assign53310_e81378_d_n8, assign53310_e81378_d_n9, assign53310_e81378_d_n10, assign53310_e81378_d_n11, assign53310_e81378_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_guard1353 == 0.0)) {
        let assign53310_e81373: f64 = (-locals.var_c_sb__blk1325);
        let assign53310_e81375: f64 = (assign53310_e81373 * locals.var_dphi_sb__blk1324);
        let assign53310_e81376: f64 = (assign53310_e81375).exp();
        (assign53310_e81376, (assign53310_e81376 * (((-locals.var_c_sb__blk1325_dn0) * locals.var_dphi_sb__blk1324) + (assign53310_e81373 * locals.var_dphi_sb__blk1324_dn0))), (assign53310_e81376 * (((-locals.var_c_sb__blk1325_dn2) * locals.var_dphi_sb__blk1324) + (assign53310_e81373 * locals.var_dphi_sb__blk1324_dn2))), (assign53310_e81376 * (((-locals.var_c_sb__blk1325_dn4) * locals.var_dphi_sb__blk1324) + (assign53310_e81373 * locals.var_dphi_sb__blk1324_dn4))), (assign53310_e81376 * (((-locals.var_c_sb__blk1325_dn5) * locals.var_dphi_sb__blk1324) + (assign53310_e81373 * locals.var_dphi_sb__blk1324_dn5))), (assign53310_e81376 * (((-locals.var_c_sb__blk1325_dn6) * locals.var_dphi_sb__blk1324) + (assign53310_e81373 * locals.var_dphi_sb__blk1324_dn6))), (assign53310_e81376 * (((-locals.var_c_sb__blk1325_dn7) * locals.var_dphi_sb__blk1324) + (assign53310_e81373 * locals.var_dphi_sb__blk1324_dn7))), (assign53310_e81376 * (((-locals.var_c_sb__blk1325_dn8) * locals.var_dphi_sb__blk1324) + (assign53310_e81373 * locals.var_dphi_sb__blk1324_dn8))), (assign53310_e81376 * (((-locals.var_c_sb__blk1325_dn9) * locals.var_dphi_sb__blk1324) + (assign53310_e81373 * locals.var_dphi_sb__blk1324_dn9))), (assign53310_e81376 * (((-locals.var_c_sb__blk1325_dn10) * locals.var_dphi_sb__blk1324) + (assign53310_e81373 * locals.var_dphi_sb__blk1324_dn10))), (assign53310_e81376 * (((-locals.var_c_sb__blk1325_dn11) * locals.var_dphi_sb__blk1324) + (assign53310_e81373 * locals.var_dphi_sb__blk1324_dn11))), (assign53310_e81376 * (((-locals.var_c_sb__blk1325_dn14) * locals.var_dphi_sb__blk1324) + (assign53310_e81373 * locals.var_dphi_sb__blk1324_dn14))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign53310_e81378;
        locals.var_t0_dn0 = assign53310_e81378_d_n0;
        locals.var_t0_dn2 = assign53310_e81378_d_n2;
        locals.var_t0_dn4 = assign53310_e81378_d_n4;
        locals.var_t0_dn5 = assign53310_e81378_d_n5;
        locals.var_t0_dn6 = assign53310_e81378_d_n6;
        locals.var_t0_dn7 = assign53310_e81378_d_n7;
        locals.var_t0_dn8 = assign53310_e81378_d_n8;
        locals.var_t0_dn9 = assign53310_e81378_d_n9;
        locals.var_t0_dn10 = assign53310_e81378_d_n10;
        locals.var_t0_dn11 = assign53310_e81378_d_n11;
        locals.var_t0_dn14 = assign53310_e81378_d_n14;

        let assign53320_e81380: f64 = (locals.var_tx).abs();
        let assign53320_e81382: f64 = if assign53320_e81380 > 1e-8 { 1.0 } else { 0.0 };
        locals.var_guard1354 = assign53320_e81382;

        let assign53330_e81385: f64 = if locals.var_tx >= 500.0 { 1.0 } else { 0.0 };
        locals.var_guard1355 = assign53330_e81385;

        let (assign53340_e81412, assign53340_e81412_d_n0, assign53340_e81412_d_n2, assign53340_e81412_d_n4, assign53340_e81412_d_n5, assign53340_e81412_d_n6, assign53340_e81412_d_n7, assign53340_e81412_d_n8, assign53340_e81412_d_n9, assign53340_e81412_d_n10, assign53340_e81412_d_n11, assign53340_e81412_d_n14,) = {
    if ((((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_guard1353 == 0.0)) && (locals.var_guard1354 != 0.0)) && (locals.var_guard1355 != 0.0)) {
        let assign53340_e81407: f64 = (1.0 + locals.var_tx);
        let assign53340_e81409: f64 = (assign53340_e81407 - 500.0);
        let assign53340_e81410: f64 = (1.403592217853e217 * assign53340_e81409);
        (assign53340_e81410, (1.403592217853e217 * locals.var_tx_dn0), (1.403592217853e217 * locals.var_tx_dn2), (1.403592217853e217 * locals.var_tx_dn4), (1.403592217853e217 * locals.var_tx_dn5), (1.403592217853e217 * locals.var_tx_dn6), (1.403592217853e217 * locals.var_tx_dn7), (1.403592217853e217 * locals.var_tx_dn8), (1.403592217853e217 * locals.var_tx_dn9), (1.403592217853e217 * locals.var_tx_dn10), (1.403592217853e217 * locals.var_tx_dn11), (1.403592217853e217 * locals.var_tx_dn14),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign53340_e81412;
        locals.var_t1_dn0 = assign53340_e81412_d_n0;
        locals.var_t1_dn2 = assign53340_e81412_d_n2;
        locals.var_t1_dn4 = assign53340_e81412_d_n4;
        locals.var_t1_dn5 = assign53340_e81412_d_n5;
        locals.var_t1_dn6 = assign53340_e81412_d_n6;
        locals.var_t1_dn7 = assign53340_e81412_d_n7;
        locals.var_t1_dn8 = assign53340_e81412_d_n8;
        locals.var_t1_dn9 = assign53340_e81412_d_n9;
        locals.var_t1_dn10 = assign53340_e81412_d_n10;
        locals.var_t1_dn11 = assign53340_e81412_d_n11;
        locals.var_t1_dn14 = assign53340_e81412_d_n14;

        let (assign53350_e81433, assign53350_e81433_d_n0, assign53350_e81433_d_n2, assign53350_e81433_d_n4, assign53350_e81433_d_n5, assign53350_e81433_d_n6, assign53350_e81433_d_n7, assign53350_e81433_d_n8, assign53350_e81433_d_n9, assign53350_e81433_d_n10, assign53350_e81433_d_n11, assign53350_e81433_d_n14,) = {
    if ((((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_guard1353 == 0.0)) && (locals.var_guard1354 != 0.0)) && (locals.var_guard1355 != 0.0)) {
        (1.403592217853e217, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign53350_e81433;
        locals.var_t3_dn0 = assign53350_e81433_d_n0;
        locals.var_t3_dn2 = assign53350_e81433_d_n2;
        locals.var_t3_dn4 = assign53350_e81433_d_n4;
        locals.var_t3_dn5 = assign53350_e81433_d_n5;
        locals.var_t3_dn6 = assign53350_e81433_d_n6;
        locals.var_t3_dn7 = assign53350_e81433_d_n7;
        locals.var_t3_dn8 = assign53350_e81433_d_n8;
        locals.var_t3_dn9 = assign53350_e81433_d_n9;
        locals.var_t3_dn10 = assign53350_e81433_d_n10;
        locals.var_t3_dn11 = assign53350_e81433_d_n11;
        locals.var_t3_dn14 = assign53350_e81433_d_n14;

        let (assign53360_e81455, assign53360_e81455_d_n0, assign53360_e81455_d_n2, assign53360_e81455_d_n4, assign53360_e81455_d_n5, assign53360_e81455_d_n6, assign53360_e81455_d_n7, assign53360_e81455_d_n8, assign53360_e81455_d_n9, assign53360_e81455_d_n10, assign53360_e81455_d_n11, assign53360_e81455_d_n14,) = {
    if ((((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_guard1353 == 0.0)) && (locals.var_guard1354 != 0.0)) && (locals.var_guard1355 == 0.0)) {
        (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn4, locals.var_tx_dn5, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn8, locals.var_tx_dn9, locals.var_tx_dn10, locals.var_tx_dn11, locals.var_tx_dn14,)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign53360_e81455;
        locals.var_tmf1_dn0 = assign53360_e81455_d_n0;
        locals.var_tmf1_dn2 = assign53360_e81455_d_n2;
        locals.var_tmf1_dn4 = assign53360_e81455_d_n4;
        locals.var_tmf1_dn5 = assign53360_e81455_d_n5;
        locals.var_tmf1_dn6 = assign53360_e81455_d_n6;
        locals.var_tmf1_dn7 = assign53360_e81455_d_n7;
        locals.var_tmf1_dn8 = assign53360_e81455_d_n8;
        locals.var_tmf1_dn9 = assign53360_e81455_d_n9;
        locals.var_tmf1_dn10 = assign53360_e81455_d_n10;
        locals.var_tmf1_dn11 = assign53360_e81455_d_n11;
        locals.var_tmf1_dn14 = assign53360_e81455_d_n14;

        let (assign53370_e81477, assign53370_e81477_d_n0, assign53370_e81477_d_n2, assign53370_e81477_d_n4, assign53370_e81477_d_n5, assign53370_e81477_d_n6, assign53370_e81477_d_n7, assign53370_e81477_d_n8, assign53370_e81477_d_n9, assign53370_e81477_d_n10, assign53370_e81477_d_n11, assign53370_e81477_d_n14,) = {
    if ((((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_guard1353 == 0.0)) && (locals.var_guard1354 != 0.0)) && (locals.var_guard1355 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign53370_e81477;
        locals.var_t1_dn0 = assign53370_e81477_d_n0;
        locals.var_t1_dn2 = assign53370_e81477_d_n2;
        locals.var_t1_dn4 = assign53370_e81477_d_n4;
        locals.var_t1_dn5 = assign53370_e81477_d_n5;
        locals.var_t1_dn6 = assign53370_e81477_d_n6;
        locals.var_t1_dn7 = assign53370_e81477_d_n7;
        locals.var_t1_dn8 = assign53370_e81477_d_n8;
        locals.var_t1_dn9 = assign53370_e81477_d_n9;
        locals.var_t1_dn10 = assign53370_e81477_d_n10;
        locals.var_t1_dn11 = assign53370_e81477_d_n11;
        locals.var_t1_dn14 = assign53370_e81477_d_n14;

        let mut assign53380_loop_guard: usize = 0;
        while {
            let assign53380_cond_e81500: f64 = if (((((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_guard1353 == 0.0)) && (locals.var_guard1354 != 0.0)) && (locals.var_guard1355 == 0.0)) && (locals.var_tmf1 >= 60.0)) { 1.0 } else { 0.0 };
            assign53380_cond_e81500 != 0.0
        } {
            assign53380_loop_guard += 1;
            assert!(assign53380_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign53380_body0_e81524, assign53380_body0_e81524_d_n0, assign53380_body0_e81524_d_n2, assign53380_body0_e81524_d_n4, assign53380_body0_e81524_d_n5, assign53380_body0_e81524_d_n6, assign53380_body0_e81524_d_n7, assign53380_body0_e81524_d_n8, assign53380_body0_e81524_d_n9, assign53380_body0_e81524_d_n10, assign53380_body0_e81524_d_n11, assign53380_body0_e81524_d_n14,) = {
    if ((((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_guard1353 == 0.0)) && (locals.var_guard1354 != 0.0)) && (locals.var_guard1355 == 0.0)) {
        let assign53380_body0_e81522: f64 = (locals.var_t1 * 1.14200738981568e26);
        (assign53380_body0_e81522, (locals.var_t1_dn0 * 1.14200738981568e26), (locals.var_t1_dn2 * 1.14200738981568e26), (locals.var_t1_dn4 * 1.14200738981568e26), (locals.var_t1_dn5 * 1.14200738981568e26), (locals.var_t1_dn6 * 1.14200738981568e26), (locals.var_t1_dn7 * 1.14200738981568e26), (locals.var_t1_dn8 * 1.14200738981568e26), (locals.var_t1_dn9 * 1.14200738981568e26), (locals.var_t1_dn10 * 1.14200738981568e26), (locals.var_t1_dn11 * 1.14200738981568e26), (locals.var_t1_dn14 * 1.14200738981568e26),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
            locals.var_t1 = assign53380_body0_e81524;
            locals.var_t1_dn0 = assign53380_body0_e81524_d_n0;
            locals.var_t1_dn2 = assign53380_body0_e81524_d_n2;
            locals.var_t1_dn4 = assign53380_body0_e81524_d_n4;
            locals.var_t1_dn5 = assign53380_body0_e81524_d_n5;
            locals.var_t1_dn6 = assign53380_body0_e81524_d_n6;
            locals.var_t1_dn7 = assign53380_body0_e81524_d_n7;
            locals.var_t1_dn8 = assign53380_body0_e81524_d_n8;
            locals.var_t1_dn9 = assign53380_body0_e81524_d_n9;
            locals.var_t1_dn10 = assign53380_body0_e81524_d_n10;
            locals.var_t1_dn11 = assign53380_body0_e81524_d_n11;
            locals.var_t1_dn14 = assign53380_body0_e81524_d_n14;
            let (assign53380_body1_e81548, assign53380_body1_e81548_d_n0, assign53380_body1_e81548_d_n2, assign53380_body1_e81548_d_n4, assign53380_body1_e81548_d_n5, assign53380_body1_e81548_d_n6, assign53380_body1_e81548_d_n7, assign53380_body1_e81548_d_n8, assign53380_body1_e81548_d_n9, assign53380_body1_e81548_d_n10, assign53380_body1_e81548_d_n11, assign53380_body1_e81548_d_n14,) = {
    if ((((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_guard1353 == 0.0)) && (locals.var_guard1354 != 0.0)) && (locals.var_guard1355 == 0.0)) {
        let assign53380_body1_e81546: f64 = (locals.var_tmf1 - 60.0);
        (assign53380_body1_e81546, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
            locals.var_tmf1 = assign53380_body1_e81548;
            locals.var_tmf1_dn0 = assign53380_body1_e81548_d_n0;
            locals.var_tmf1_dn2 = assign53380_body1_e81548_d_n2;
            locals.var_tmf1_dn4 = assign53380_body1_e81548_d_n4;
            locals.var_tmf1_dn5 = assign53380_body1_e81548_d_n5;
            locals.var_tmf1_dn6 = assign53380_body1_e81548_d_n6;
            locals.var_tmf1_dn7 = assign53380_body1_e81548_d_n7;
            locals.var_tmf1_dn8 = assign53380_body1_e81548_d_n8;
            locals.var_tmf1_dn9 = assign53380_body1_e81548_d_n9;
            locals.var_tmf1_dn10 = assign53380_body1_e81548_d_n10;
            locals.var_tmf1_dn11 = assign53380_body1_e81548_d_n11;
            locals.var_tmf1_dn14 = assign53380_body1_e81548_d_n14;
        }

        let (assign53390_e81573, assign53390_e81573_d_n0, assign53390_e81573_d_n2, assign53390_e81573_d_n4, assign53390_e81573_d_n5, assign53390_e81573_d_n6, assign53390_e81573_d_n7, assign53390_e81573_d_n8, assign53390_e81573_d_n9, assign53390_e81573_d_n10, assign53390_e81573_d_n11, assign53390_e81573_d_n14,) = {
    if ((((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_guard1353 == 0.0)) && (locals.var_guard1354 != 0.0)) && (locals.var_guard1355 == 0.0)) {
        let assign53390_e81570: f64 = (locals.var_tmf1).exp();
        let assign53390_e81571: f64 = (locals.var_t1 * assign53390_e81570);
        (assign53390_e81571, ((locals.var_t1_dn0 * assign53390_e81570) + (locals.var_t1 * (assign53390_e81570 * locals.var_tmf1_dn0))), ((locals.var_t1_dn2 * assign53390_e81570) + (locals.var_t1 * (assign53390_e81570 * locals.var_tmf1_dn2))), ((locals.var_t1_dn4 * assign53390_e81570) + (locals.var_t1 * (assign53390_e81570 * locals.var_tmf1_dn4))), ((locals.var_t1_dn5 * assign53390_e81570) + (locals.var_t1 * (assign53390_e81570 * locals.var_tmf1_dn5))), ((locals.var_t1_dn6 * assign53390_e81570) + (locals.var_t1 * (assign53390_e81570 * locals.var_tmf1_dn6))), ((locals.var_t1_dn7 * assign53390_e81570) + (locals.var_t1 * (assign53390_e81570 * locals.var_tmf1_dn7))), ((locals.var_t1_dn8 * assign53390_e81570) + (locals.var_t1 * (assign53390_e81570 * locals.var_tmf1_dn8))), ((locals.var_t1_dn9 * assign53390_e81570) + (locals.var_t1 * (assign53390_e81570 * locals.var_tmf1_dn9))), ((locals.var_t1_dn10 * assign53390_e81570) + (locals.var_t1 * (assign53390_e81570 * locals.var_tmf1_dn10))), ((locals.var_t1_dn11 * assign53390_e81570) + (locals.var_t1 * (assign53390_e81570 * locals.var_tmf1_dn11))), ((locals.var_t1_dn14 * assign53390_e81570) + (locals.var_t1 * (assign53390_e81570 * locals.var_tmf1_dn14))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign53390_e81573;
        locals.var_t1_dn0 = assign53390_e81573_d_n0;
        locals.var_t1_dn2 = assign53390_e81573_d_n2;
        locals.var_t1_dn4 = assign53390_e81573_d_n4;
        locals.var_t1_dn5 = assign53390_e81573_d_n5;
        locals.var_t1_dn6 = assign53390_e81573_d_n6;
        locals.var_t1_dn7 = assign53390_e81573_d_n7;
        locals.var_t1_dn8 = assign53390_e81573_d_n8;
        locals.var_t1_dn9 = assign53390_e81573_d_n9;
        locals.var_t1_dn10 = assign53390_e81573_d_n10;
        locals.var_t1_dn11 = assign53390_e81573_d_n11;
        locals.var_t1_dn14 = assign53390_e81573_d_n14;

        let (assign53400_e81595, assign53400_e81595_d_n0, assign53400_e81595_d_n2, assign53400_e81595_d_n4, assign53400_e81595_d_n5, assign53400_e81595_d_n6, assign53400_e81595_d_n7, assign53400_e81595_d_n8, assign53400_e81595_d_n9, assign53400_e81595_d_n10, assign53400_e81595_d_n11, assign53400_e81595_d_n14,) = {
    if ((((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_guard1353 == 0.0)) && (locals.var_guard1354 != 0.0)) && (locals.var_guard1355 == 0.0)) {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign53400_e81595;
        locals.var_t3_dn0 = assign53400_e81595_d_n0;
        locals.var_t3_dn2 = assign53400_e81595_d_n2;
        locals.var_t3_dn4 = assign53400_e81595_d_n4;
        locals.var_t3_dn5 = assign53400_e81595_d_n5;
        locals.var_t3_dn6 = assign53400_e81595_d_n6;
        locals.var_t3_dn7 = assign53400_e81595_d_n7;
        locals.var_t3_dn8 = assign53400_e81595_d_n8;
        locals.var_t3_dn9 = assign53400_e81595_d_n9;
        locals.var_t3_dn10 = assign53400_e81595_d_n10;
        locals.var_t3_dn11 = assign53400_e81595_d_n11;
        locals.var_t3_dn14 = assign53400_e81595_d_n14;

        let (assign53410_e81616, assign53410_e81616_d_n0, assign53410_e81616_d_n2, assign53410_e81616_d_n4, assign53410_e81616_d_n5, assign53410_e81616_d_n6, assign53410_e81616_d_n7, assign53410_e81616_d_n8, assign53410_e81616_d_n9, assign53410_e81616_d_n10, assign53410_e81616_d_n11, assign53410_e81616_d_n14,) = {
    if (((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_guard1353 == 0.0)) && (locals.var_guard1354 != 0.0)) {
        let assign53410_e81614: f64 = (locals.var_t1 * locals.var_t0);
        (assign53410_e81614, ((locals.var_t1_dn0 * locals.var_t0) + (locals.var_t1 * locals.var_t0_dn0)), ((locals.var_t1_dn2 * locals.var_t0) + (locals.var_t1 * locals.var_t0_dn2)), ((locals.var_t1_dn4 * locals.var_t0) + (locals.var_t1 * locals.var_t0_dn4)), ((locals.var_t1_dn5 * locals.var_t0) + (locals.var_t1 * locals.var_t0_dn5)), ((locals.var_t1_dn6 * locals.var_t0) + (locals.var_t1 * locals.var_t0_dn6)), ((locals.var_t1_dn7 * locals.var_t0) + (locals.var_t1 * locals.var_t0_dn7)), ((locals.var_t1_dn8 * locals.var_t0) + (locals.var_t1 * locals.var_t0_dn8)), ((locals.var_t1_dn9 * locals.var_t0) + (locals.var_t1 * locals.var_t0_dn9)), ((locals.var_t1_dn10 * locals.var_t0) + (locals.var_t1 * locals.var_t0_dn10)), ((locals.var_t1_dn11 * locals.var_t0) + (locals.var_t1 * locals.var_t0_dn11)), ((locals.var_t1_dn14 * locals.var_t0) + (locals.var_t1 * locals.var_t0_dn14)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign53410_e81616;
        locals.var_t1_dn0 = assign53410_e81616_d_n0;
        locals.var_t1_dn2 = assign53410_e81616_d_n2;
        locals.var_t1_dn4 = assign53410_e81616_d_n4;
        locals.var_t1_dn5 = assign53410_e81616_d_n5;
        locals.var_t1_dn6 = assign53410_e81616_d_n6;
        locals.var_t1_dn7 = assign53410_e81616_d_n7;
        locals.var_t1_dn8 = assign53410_e81616_d_n8;
        locals.var_t1_dn9 = assign53410_e81616_d_n9;
        locals.var_t1_dn10 = assign53410_e81616_d_n10;
        locals.var_t1_dn11 = assign53410_e81616_d_n11;
        locals.var_t1_dn14 = assign53410_e81616_d_n14;

        let (assign53420_e81637, assign53420_e81637_d_n0, assign53420_e81637_d_n2, assign53420_e81637_d_n4, assign53420_e81637_d_n5, assign53420_e81637_d_n6, assign53420_e81637_d_n7, assign53420_e81637_d_n8, assign53420_e81637_d_n9, assign53420_e81637_d_n10, assign53420_e81637_d_n11, assign53420_e81637_d_n14,) = {
    if (((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_guard1353 == 0.0)) && (locals.var_guard1354 != 0.0)) {
        let assign53420_e81635: f64 = (locals.var_t1 - locals.var_t0);
        (assign53420_e81635, (locals.var_t1_dn0 - locals.var_t0_dn0), (locals.var_t1_dn2 - locals.var_t0_dn2), (locals.var_t1_dn4 - locals.var_t0_dn4), (locals.var_t1_dn5 - locals.var_t0_dn5), (locals.var_t1_dn6 - locals.var_t0_dn6), (locals.var_t1_dn7 - locals.var_t0_dn7), (locals.var_t1_dn8 - locals.var_t0_dn8), (locals.var_t1_dn9 - locals.var_t0_dn9), (locals.var_t1_dn10 - locals.var_t0_dn10), (locals.var_t1_dn11 - locals.var_t0_dn11), (locals.var_t1_dn14 - locals.var_t0_dn14),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign53420_e81637;
        locals.var_t2_dn0 = assign53420_e81637_d_n0;
        locals.var_t2_dn2 = assign53420_e81637_d_n2;
        locals.var_t2_dn4 = assign53420_e81637_d_n4;
        locals.var_t2_dn5 = assign53420_e81637_d_n5;
        locals.var_t2_dn6 = assign53420_e81637_d_n6;
        locals.var_t2_dn7 = assign53420_e81637_d_n7;
        locals.var_t2_dn8 = assign53420_e81637_d_n8;
        locals.var_t2_dn9 = assign53420_e81637_d_n9;
        locals.var_t2_dn10 = assign53420_e81637_d_n10;
        locals.var_t2_dn11 = assign53420_e81637_d_n11;
        locals.var_t2_dn14 = assign53420_e81637_d_n14;

        let (assign53430_e81661, assign53430_e81661_d_n0, assign53430_e81661_d_n2, assign53430_e81661_d_n4, assign53430_e81661_d_n5, assign53430_e81661_d_n6, assign53430_e81661_d_n7, assign53430_e81661_d_n8, assign53430_e81661_d_n9, assign53430_e81661_d_n10, assign53430_e81661_d_n11, assign53430_e81661_d_n14,) = {
    if (((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_guard1353 == 0.0)) && (locals.var_guard1354 == 0.0)) {
        let assign53430_e81657: f64 = (1.0 + locals.var_tx);
        let assign53430_e81659: f64 = (assign53430_e81657 * locals.var_t0);
        (assign53430_e81659, ((locals.var_tx_dn0 * locals.var_t0) + (assign53430_e81657 * locals.var_t0_dn0)), ((locals.var_tx_dn2 * locals.var_t0) + (assign53430_e81657 * locals.var_t0_dn2)), ((locals.var_tx_dn4 * locals.var_t0) + (assign53430_e81657 * locals.var_t0_dn4)), ((locals.var_tx_dn5 * locals.var_t0) + (assign53430_e81657 * locals.var_t0_dn5)), ((locals.var_tx_dn6 * locals.var_t0) + (assign53430_e81657 * locals.var_t0_dn6)), ((locals.var_tx_dn7 * locals.var_t0) + (assign53430_e81657 * locals.var_t0_dn7)), ((locals.var_tx_dn8 * locals.var_t0) + (assign53430_e81657 * locals.var_t0_dn8)), ((locals.var_tx_dn9 * locals.var_t0) + (assign53430_e81657 * locals.var_t0_dn9)), ((locals.var_tx_dn10 * locals.var_t0) + (assign53430_e81657 * locals.var_t0_dn10)), ((locals.var_tx_dn11 * locals.var_t0) + (assign53430_e81657 * locals.var_t0_dn11)), ((locals.var_tx_dn14 * locals.var_t0) + (assign53430_e81657 * locals.var_t0_dn14)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign53430_e81661;
        locals.var_t1_dn0 = assign53430_e81661_d_n0;
        locals.var_t1_dn2 = assign53430_e81661_d_n2;
        locals.var_t1_dn4 = assign53430_e81661_d_n4;
        locals.var_t1_dn5 = assign53430_e81661_d_n5;
        locals.var_t1_dn6 = assign53430_e81661_d_n6;
        locals.var_t1_dn7 = assign53430_e81661_d_n7;
        locals.var_t1_dn8 = assign53430_e81661_d_n8;
        locals.var_t1_dn9 = assign53430_e81661_d_n9;
        locals.var_t1_dn10 = assign53430_e81661_d_n10;
        locals.var_t1_dn11 = assign53430_e81661_d_n11;
        locals.var_t1_dn14 = assign53430_e81661_d_n14;

    }

    pub(super) fn stamp_transient_block_183(
        locals: &mut StampLocals,
    ) {
        let (assign53440_e81689, assign53440_e81689_d_n0, assign53440_e81689_d_n2, assign53440_e81689_d_n4, assign53440_e81689_d_n5, assign53440_e81689_d_n6, assign53440_e81689_d_n7, assign53440_e81689_d_n8, assign53440_e81689_d_n9, assign53440_e81689_d_n10, assign53440_e81689_d_n11, assign53440_e81689_d_n14,) = {
    if (((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_guard1353 == 0.0)) && (locals.var_guard1354 == 0.0)) {
        let assign53440_e81683: f64 = (locals.var_tx / 2.0);
        let assign53440_e81684: f64 = (1.0 + assign53440_e81683);
        let assign53440_e81685: f64 = (locals.var_tx * assign53440_e81684);
        let assign53440_e81687: f64 = (assign53440_e81685 * locals.var_t0);
        (assign53440_e81687, ((((locals.var_tx_dn0 * assign53440_e81684) + (locals.var_tx * (locals.var_tx_dn0 / 2.0))) * locals.var_t0) + (assign53440_e81685 * locals.var_t0_dn0)), ((((locals.var_tx_dn2 * assign53440_e81684) + (locals.var_tx * (locals.var_tx_dn2 / 2.0))) * locals.var_t0) + (assign53440_e81685 * locals.var_t0_dn2)), ((((locals.var_tx_dn4 * assign53440_e81684) + (locals.var_tx * (locals.var_tx_dn4 / 2.0))) * locals.var_t0) + (assign53440_e81685 * locals.var_t0_dn4)), ((((locals.var_tx_dn5 * assign53440_e81684) + (locals.var_tx * (locals.var_tx_dn5 / 2.0))) * locals.var_t0) + (assign53440_e81685 * locals.var_t0_dn5)), ((((locals.var_tx_dn6 * assign53440_e81684) + (locals.var_tx * (locals.var_tx_dn6 / 2.0))) * locals.var_t0) + (assign53440_e81685 * locals.var_t0_dn6)), ((((locals.var_tx_dn7 * assign53440_e81684) + (locals.var_tx * (locals.var_tx_dn7 / 2.0))) * locals.var_t0) + (assign53440_e81685 * locals.var_t0_dn7)), ((((locals.var_tx_dn8 * assign53440_e81684) + (locals.var_tx * (locals.var_tx_dn8 / 2.0))) * locals.var_t0) + (assign53440_e81685 * locals.var_t0_dn8)), ((((locals.var_tx_dn9 * assign53440_e81684) + (locals.var_tx * (locals.var_tx_dn9 / 2.0))) * locals.var_t0) + (assign53440_e81685 * locals.var_t0_dn9)), ((((locals.var_tx_dn10 * assign53440_e81684) + (locals.var_tx * (locals.var_tx_dn10 / 2.0))) * locals.var_t0) + (assign53440_e81685 * locals.var_t0_dn10)), ((((locals.var_tx_dn11 * assign53440_e81684) + (locals.var_tx * (locals.var_tx_dn11 / 2.0))) * locals.var_t0) + (assign53440_e81685 * locals.var_t0_dn11)), ((((locals.var_tx_dn14 * assign53440_e81684) + (locals.var_tx * (locals.var_tx_dn14 / 2.0))) * locals.var_t0) + (assign53440_e81685 * locals.var_t0_dn14)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign53440_e81689;
        locals.var_t2_dn0 = assign53440_e81689_d_n0;
        locals.var_t2_dn2 = assign53440_e81689_d_n2;
        locals.var_t2_dn4 = assign53440_e81689_d_n4;
        locals.var_t2_dn5 = assign53440_e81689_d_n5;
        locals.var_t2_dn6 = assign53440_e81689_d_n6;
        locals.var_t2_dn7 = assign53440_e81689_d_n7;
        locals.var_t2_dn8 = assign53440_e81689_d_n8;
        locals.var_t2_dn9 = assign53440_e81689_d_n9;
        locals.var_t2_dn10 = assign53440_e81689_d_n10;
        locals.var_t2_dn11 = assign53440_e81689_d_n11;
        locals.var_t2_dn14 = assign53440_e81689_d_n14;

        let assign53450_e81691: f64 = (locals.var_t2).abs();
        let assign53450_e81693: f64 = if assign53450_e81691 > 1e-8 { 1.0 } else { 0.0 };
        locals.var_guard1356 = assign53450_e81693;

        let (assign53460_e81717, assign53460_e81717_d_n0, assign53460_e81717_d_n2, assign53460_e81717_d_n4, assign53460_e81717_d_n5, assign53460_e81717_d_n6, assign53460_e81717_d_n7, assign53460_e81717_d_n8, assign53460_e81717_d_n9, assign53460_e81717_d_n10, assign53460_e81717_d_n11, assign53460_e81717_d_n14,) = {
    if (((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_guard1353 == 0.0)) && (locals.var_guard1356 != 0.0)) {
        let assign53460_e81712: f64 = (1.0 + locals.var_t2);
        let assign53460_e81713: f64 = (assign53460_e81712).ln();
        let assign53460_e81715: f64 = (assign53460_e81713 / locals.var_c_sb__blk1325);
        (assign53460_e81715, ((((locals.var_t2_dn0 / assign53460_e81712) * locals.var_c_sb__blk1325) - (assign53460_e81713 * locals.var_c_sb__blk1325_dn0)) / (locals.var_c_sb__blk1325 * locals.var_c_sb__blk1325)), ((((locals.var_t2_dn2 / assign53460_e81712) * locals.var_c_sb__blk1325) - (assign53460_e81713 * locals.var_c_sb__blk1325_dn2)) / (locals.var_c_sb__blk1325 * locals.var_c_sb__blk1325)), ((((locals.var_t2_dn4 / assign53460_e81712) * locals.var_c_sb__blk1325) - (assign53460_e81713 * locals.var_c_sb__blk1325_dn4)) / (locals.var_c_sb__blk1325 * locals.var_c_sb__blk1325)), ((((locals.var_t2_dn5 / assign53460_e81712) * locals.var_c_sb__blk1325) - (assign53460_e81713 * locals.var_c_sb__blk1325_dn5)) / (locals.var_c_sb__blk1325 * locals.var_c_sb__blk1325)), ((((locals.var_t2_dn6 / assign53460_e81712) * locals.var_c_sb__blk1325) - (assign53460_e81713 * locals.var_c_sb__blk1325_dn6)) / (locals.var_c_sb__blk1325 * locals.var_c_sb__blk1325)), ((((locals.var_t2_dn7 / assign53460_e81712) * locals.var_c_sb__blk1325) - (assign53460_e81713 * locals.var_c_sb__blk1325_dn7)) / (locals.var_c_sb__blk1325 * locals.var_c_sb__blk1325)), ((((locals.var_t2_dn8 / assign53460_e81712) * locals.var_c_sb__blk1325) - (assign53460_e81713 * locals.var_c_sb__blk1325_dn8)) / (locals.var_c_sb__blk1325 * locals.var_c_sb__blk1325)), ((((locals.var_t2_dn9 / assign53460_e81712) * locals.var_c_sb__blk1325) - (assign53460_e81713 * locals.var_c_sb__blk1325_dn9)) / (locals.var_c_sb__blk1325 * locals.var_c_sb__blk1325)), ((((locals.var_t2_dn10 / assign53460_e81712) * locals.var_c_sb__blk1325) - (assign53460_e81713 * locals.var_c_sb__blk1325_dn10)) / (locals.var_c_sb__blk1325 * locals.var_c_sb__blk1325)), ((((locals.var_t2_dn11 / assign53460_e81712) * locals.var_c_sb__blk1325) - (assign53460_e81713 * locals.var_c_sb__blk1325_dn11)) / (locals.var_c_sb__blk1325 * locals.var_c_sb__blk1325)), ((((locals.var_t2_dn14 / assign53460_e81712) * locals.var_c_sb__blk1325) - (assign53460_e81713 * locals.var_c_sb__blk1325_dn14)) / (locals.var_c_sb__blk1325 * locals.var_c_sb__blk1325)),)
    } else {
        (locals.var_pb0dep__blk1169, locals.var_pb0dep__blk1169_dn0, locals.var_pb0dep__blk1169_dn2, locals.var_pb0dep__blk1169_dn4, locals.var_pb0dep__blk1169_dn5, locals.var_pb0dep__blk1169_dn6, locals.var_pb0dep__blk1169_dn7, locals.var_pb0dep__blk1169_dn8, locals.var_pb0dep__blk1169_dn9, locals.var_pb0dep__blk1169_dn10, locals.var_pb0dep__blk1169_dn11, locals.var_pb0dep__blk1169_dn14,)
    }
};
        locals.var_pb0dep__blk1169 = assign53460_e81717;
        locals.var_pb0dep__blk1169_dn0 = assign53460_e81717_d_n0;
        locals.var_pb0dep__blk1169_dn2 = assign53460_e81717_d_n2;
        locals.var_pb0dep__blk1169_dn4 = assign53460_e81717_d_n4;
        locals.var_pb0dep__blk1169_dn5 = assign53460_e81717_d_n5;
        locals.var_pb0dep__blk1169_dn6 = assign53460_e81717_d_n6;
        locals.var_pb0dep__blk1169_dn7 = assign53460_e81717_d_n7;
        locals.var_pb0dep__blk1169_dn8 = assign53460_e81717_d_n8;
        locals.var_pb0dep__blk1169_dn9 = assign53460_e81717_d_n9;
        locals.var_pb0dep__blk1169_dn10 = assign53460_e81717_d_n10;
        locals.var_pb0dep__blk1169_dn11 = assign53460_e81717_d_n11;
        locals.var_pb0dep__blk1169_dn14 = assign53460_e81717_d_n14;

        let (assign53470_e81739, assign53470_e81739_d_n0, assign53470_e81739_d_n2, assign53470_e81739_d_n4, assign53470_e81739_d_n5, assign53470_e81739_d_n6, assign53470_e81739_d_n7, assign53470_e81739_d_n8, assign53470_e81739_d_n9, assign53470_e81739_d_n10, assign53470_e81739_d_n11, assign53470_e81739_d_n14,) = {
    if (((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_guard1353 == 0.0)) && (locals.var_guard1356 == 0.0)) {
        let assign53470_e81737: f64 = (locals.var_t2 / locals.var_c_sb__blk1325);
        (assign53470_e81737, (((locals.var_t2_dn0 * locals.var_c_sb__blk1325) - (locals.var_t2 * locals.var_c_sb__blk1325_dn0)) / (locals.var_c_sb__blk1325 * locals.var_c_sb__blk1325)), (((locals.var_t2_dn2 * locals.var_c_sb__blk1325) - (locals.var_t2 * locals.var_c_sb__blk1325_dn2)) / (locals.var_c_sb__blk1325 * locals.var_c_sb__blk1325)), (((locals.var_t2_dn4 * locals.var_c_sb__blk1325) - (locals.var_t2 * locals.var_c_sb__blk1325_dn4)) / (locals.var_c_sb__blk1325 * locals.var_c_sb__blk1325)), (((locals.var_t2_dn5 * locals.var_c_sb__blk1325) - (locals.var_t2 * locals.var_c_sb__blk1325_dn5)) / (locals.var_c_sb__blk1325 * locals.var_c_sb__blk1325)), (((locals.var_t2_dn6 * locals.var_c_sb__blk1325) - (locals.var_t2 * locals.var_c_sb__blk1325_dn6)) / (locals.var_c_sb__blk1325 * locals.var_c_sb__blk1325)), (((locals.var_t2_dn7 * locals.var_c_sb__blk1325) - (locals.var_t2 * locals.var_c_sb__blk1325_dn7)) / (locals.var_c_sb__blk1325 * locals.var_c_sb__blk1325)), (((locals.var_t2_dn8 * locals.var_c_sb__blk1325) - (locals.var_t2 * locals.var_c_sb__blk1325_dn8)) / (locals.var_c_sb__blk1325 * locals.var_c_sb__blk1325)), (((locals.var_t2_dn9 * locals.var_c_sb__blk1325) - (locals.var_t2 * locals.var_c_sb__blk1325_dn9)) / (locals.var_c_sb__blk1325 * locals.var_c_sb__blk1325)), (((locals.var_t2_dn10 * locals.var_c_sb__blk1325) - (locals.var_t2 * locals.var_c_sb__blk1325_dn10)) / (locals.var_c_sb__blk1325 * locals.var_c_sb__blk1325)), (((locals.var_t2_dn11 * locals.var_c_sb__blk1325) - (locals.var_t2 * locals.var_c_sb__blk1325_dn11)) / (locals.var_c_sb__blk1325 * locals.var_c_sb__blk1325)), (((locals.var_t2_dn14 * locals.var_c_sb__blk1325) - (locals.var_t2 * locals.var_c_sb__blk1325_dn14)) / (locals.var_c_sb__blk1325 * locals.var_c_sb__blk1325)),)
    } else {
        (locals.var_pb0dep__blk1169, locals.var_pb0dep__blk1169_dn0, locals.var_pb0dep__blk1169_dn2, locals.var_pb0dep__blk1169_dn4, locals.var_pb0dep__blk1169_dn5, locals.var_pb0dep__blk1169_dn6, locals.var_pb0dep__blk1169_dn7, locals.var_pb0dep__blk1169_dn8, locals.var_pb0dep__blk1169_dn9, locals.var_pb0dep__blk1169_dn10, locals.var_pb0dep__blk1169_dn11, locals.var_pb0dep__blk1169_dn14,)
    }
};
        locals.var_pb0dep__blk1169 = assign53470_e81739;
        locals.var_pb0dep__blk1169_dn0 = assign53470_e81739_d_n0;
        locals.var_pb0dep__blk1169_dn2 = assign53470_e81739_d_n2;
        locals.var_pb0dep__blk1169_dn4 = assign53470_e81739_d_n4;
        locals.var_pb0dep__blk1169_dn5 = assign53470_e81739_d_n5;
        locals.var_pb0dep__blk1169_dn6 = assign53470_e81739_d_n6;
        locals.var_pb0dep__blk1169_dn7 = assign53470_e81739_d_n7;
        locals.var_pb0dep__blk1169_dn8 = assign53470_e81739_d_n8;
        locals.var_pb0dep__blk1169_dn9 = assign53470_e81739_d_n9;
        locals.var_pb0dep__blk1169_dn10 = assign53470_e81739_d_n10;
        locals.var_pb0dep__blk1169_dn11 = assign53470_e81739_d_n11;
        locals.var_pb0dep__blk1169_dn14 = assign53470_e81739_d_n14;

        let (assign53480_e81755, assign53480_e81755_d_n0, assign53480_e81755_d_n2, assign53480_e81755_d_n4, assign53480_e81755_d_n5, assign53480_e81755_d_n6, assign53480_e81755_d_n7, assign53480_e81755_d_n8, assign53480_e81755_d_n9, assign53480_e81755_d_n10, assign53480_e81755_d_n11, assign53480_e81755_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) {
        let assign53480_e81753: f64 = (locals.var_ps0dep - locals.var_pb0dep__blk1169);
        (assign53480_e81753, (locals.var_ps0dep_dn0 - locals.var_pb0dep__blk1169_dn0), (locals.var_ps0dep_dn2 - locals.var_pb0dep__blk1169_dn2), (locals.var_ps0dep_dn4 - locals.var_pb0dep__blk1169_dn4), (locals.var_ps0dep_dn5 - locals.var_pb0dep__blk1169_dn5), (locals.var_ps0dep_dn6 - locals.var_pb0dep__blk1169_dn6), (locals.var_ps0dep_dn7 - locals.var_pb0dep__blk1169_dn7), (locals.var_ps0dep_dn8 - locals.var_pb0dep__blk1169_dn8), (locals.var_ps0dep_dn9 - locals.var_pb0dep__blk1169_dn9), (locals.var_ps0dep_dn10 - locals.var_pb0dep__blk1169_dn10), (locals.var_ps0dep_dn11 - locals.var_pb0dep__blk1169_dn11), (locals.var_ps0dep_dn14 - locals.var_pb0dep__blk1169_dn14),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign53480_e81755;
        locals.var_t2_dn0 = assign53480_e81755_d_n0;
        locals.var_t2_dn2 = assign53480_e81755_d_n2;
        locals.var_t2_dn4 = assign53480_e81755_d_n4;
        locals.var_t2_dn5 = assign53480_e81755_d_n5;
        locals.var_t2_dn6 = assign53480_e81755_d_n6;
        locals.var_t2_dn7 = assign53480_e81755_d_n7;
        locals.var_t2_dn8 = assign53480_e81755_d_n8;
        locals.var_t2_dn9 = assign53480_e81755_d_n9;
        locals.var_t2_dn10 = assign53480_e81755_d_n10;
        locals.var_t2_dn11 = assign53480_e81755_d_n11;
        locals.var_t2_dn14 = assign53480_e81755_d_n14;

        let assign53490_e81758: f64 = if 0.0 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1357 = assign53490_e81758;

        let (assign53500_e81787, assign53500_e81787_d_n0, assign53500_e81787_d_n2, assign53500_e81787_d_n4, assign53500_e81787_d_n5, assign53500_e81787_d_n6, assign53500_e81787_d_n7, assign53500_e81787_d_n8, assign53500_e81787_d_n9, assign53500_e81787_d_n10, assign53500_e81787_d_n11, assign53500_e81787_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_guard1357 != 0.0)) {
        let (assign53500_e81785, assign53500_e81785_d_n0, assign53500_e81785_d_n2, assign53500_e81785_d_n4, assign53500_e81785_d_n5, assign53500_e81785_d_n6, assign53500_e81785_d_n7, assign53500_e81785_d_n8, assign53500_e81785_d_n9, assign53500_e81785_d_n10, assign53500_e81785_d_n11, assign53500_e81785_d_n14,) = {
            if (locals.var_t2 < 0.0) {
                let assign53500_e81776: f64 = (-locals.var_c_2esipq_ndepm__blk1140);
                let assign53500_e81778: f64 = (assign53500_e81776 * locals.var_t2);
                let assign53500_e81779: f64 = (assign53500_e81778).sqrt();
                let assign53500_e81780: f64 = (-assign53500_e81779);
                (assign53500_e81780, (-((((-locals.var_c_2esipq_ndepm__blk1140_dn0) * locals.var_t2) + (assign53500_e81776 * locals.var_t2_dn0)) / (2.0 * assign53500_e81779))), (-((((-locals.var_c_2esipq_ndepm__blk1140_dn2) * locals.var_t2) + (assign53500_e81776 * locals.var_t2_dn2)) / (2.0 * assign53500_e81779))), (-((((-locals.var_c_2esipq_ndepm__blk1140_dn4) * locals.var_t2) + (assign53500_e81776 * locals.var_t2_dn4)) / (2.0 * assign53500_e81779))), (-((((-locals.var_c_2esipq_ndepm__blk1140_dn5) * locals.var_t2) + (assign53500_e81776 * locals.var_t2_dn5)) / (2.0 * assign53500_e81779))), (-((((-locals.var_c_2esipq_ndepm__blk1140_dn6) * locals.var_t2) + (assign53500_e81776 * locals.var_t2_dn6)) / (2.0 * assign53500_e81779))), (-((((-locals.var_c_2esipq_ndepm__blk1140_dn7) * locals.var_t2) + (assign53500_e81776 * locals.var_t2_dn7)) / (2.0 * assign53500_e81779))), (-((((-locals.var_c_2esipq_ndepm__blk1140_dn8) * locals.var_t2) + (assign53500_e81776 * locals.var_t2_dn8)) / (2.0 * assign53500_e81779))), (-((((-locals.var_c_2esipq_ndepm__blk1140_dn9) * locals.var_t2) + (assign53500_e81776 * locals.var_t2_dn9)) / (2.0 * assign53500_e81779))), (-((((-locals.var_c_2esipq_ndepm__blk1140_dn10) * locals.var_t2) + (assign53500_e81776 * locals.var_t2_dn10)) / (2.0 * assign53500_e81779))), (-((((-locals.var_c_2esipq_ndepm__blk1140_dn11) * locals.var_t2) + (assign53500_e81776 * locals.var_t2_dn11)) / (2.0 * assign53500_e81779))), (-((((-locals.var_c_2esipq_ndepm__blk1140_dn14) * locals.var_t2) + (assign53500_e81776 * locals.var_t2_dn14)) / (2.0 * assign53500_e81779))),)
            } else {
                let assign53500_e81783: f64 = (locals.var_c_2esipq_ndepm__blk1140 * locals.var_t2);
                let assign53500_e81784: f64 = (assign53500_e81783).sqrt();
                (assign53500_e81784, (((locals.var_c_2esipq_ndepm__blk1140_dn0 * locals.var_t2) + (locals.var_c_2esipq_ndepm__blk1140 * locals.var_t2_dn0)) / (2.0 * assign53500_e81784)), (((locals.var_c_2esipq_ndepm__blk1140_dn2 * locals.var_t2) + (locals.var_c_2esipq_ndepm__blk1140 * locals.var_t2_dn2)) / (2.0 * assign53500_e81784)), (((locals.var_c_2esipq_ndepm__blk1140_dn4 * locals.var_t2) + (locals.var_c_2esipq_ndepm__blk1140 * locals.var_t2_dn4)) / (2.0 * assign53500_e81784)), (((locals.var_c_2esipq_ndepm__blk1140_dn5 * locals.var_t2) + (locals.var_c_2esipq_ndepm__blk1140 * locals.var_t2_dn5)) / (2.0 * assign53500_e81784)), (((locals.var_c_2esipq_ndepm__blk1140_dn6 * locals.var_t2) + (locals.var_c_2esipq_ndepm__blk1140 * locals.var_t2_dn6)) / (2.0 * assign53500_e81784)), (((locals.var_c_2esipq_ndepm__blk1140_dn7 * locals.var_t2) + (locals.var_c_2esipq_ndepm__blk1140 * locals.var_t2_dn7)) / (2.0 * assign53500_e81784)), (((locals.var_c_2esipq_ndepm__blk1140_dn8 * locals.var_t2) + (locals.var_c_2esipq_ndepm__blk1140 * locals.var_t2_dn8)) / (2.0 * assign53500_e81784)), (((locals.var_c_2esipq_ndepm__blk1140_dn9 * locals.var_t2) + (locals.var_c_2esipq_ndepm__blk1140 * locals.var_t2_dn9)) / (2.0 * assign53500_e81784)), (((locals.var_c_2esipq_ndepm__blk1140_dn10 * locals.var_t2) + (locals.var_c_2esipq_ndepm__blk1140 * locals.var_t2_dn10)) / (2.0 * assign53500_e81784)), (((locals.var_c_2esipq_ndepm__blk1140_dn11 * locals.var_t2) + (locals.var_c_2esipq_ndepm__blk1140 * locals.var_t2_dn11)) / (2.0 * assign53500_e81784)), (((locals.var_c_2esipq_ndepm__blk1140_dn14 * locals.var_t2) + (locals.var_c_2esipq_ndepm__blk1140 * locals.var_t2_dn14)) / (2.0 * assign53500_e81784)),)
            }
        };
        (assign53500_e81785, assign53500_e81785_d_n0, assign53500_e81785_d_n2, assign53500_e81785_d_n4, assign53500_e81785_d_n5, assign53500_e81785_d_n6, assign53500_e81785_d_n7, assign53500_e81785_d_n8, assign53500_e81785_d_n9, assign53500_e81785_d_n10, assign53500_e81785_d_n11, assign53500_e81785_d_n14,)
    } else {
        (locals.var_ws__blk1151, locals.var_ws__blk1151_dn0, locals.var_ws__blk1151_dn2, locals.var_ws__blk1151_dn4, locals.var_ws__blk1151_dn5, locals.var_ws__blk1151_dn6, locals.var_ws__blk1151_dn7, locals.var_ws__blk1151_dn8, locals.var_ws__blk1151_dn9, locals.var_ws__blk1151_dn10, locals.var_ws__blk1151_dn11, locals.var_ws__blk1151_dn14,)
    }
};
        locals.var_ws__blk1151 = assign53500_e81787;
        locals.var_ws__blk1151_dn0 = assign53500_e81787_d_n0;
        locals.var_ws__blk1151_dn2 = assign53500_e81787_d_n2;
        locals.var_ws__blk1151_dn4 = assign53500_e81787_d_n4;
        locals.var_ws__blk1151_dn5 = assign53500_e81787_d_n5;
        locals.var_ws__blk1151_dn6 = assign53500_e81787_d_n6;
        locals.var_ws__blk1151_dn7 = assign53500_e81787_d_n7;
        locals.var_ws__blk1151_dn8 = assign53500_e81787_d_n8;
        locals.var_ws__blk1151_dn9 = assign53500_e81787_d_n9;
        locals.var_ws__blk1151_dn10 = assign53500_e81787_d_n10;
        locals.var_ws__blk1151_dn11 = assign53500_e81787_d_n11;
        locals.var_ws__blk1151_dn14 = assign53500_e81787_d_n14;

        let assign53510_e81790: f64 = if locals.var_t2 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1358 = assign53510_e81790;

        let (assign53520_e81811, assign53520_e81811_d_n0, assign53520_e81811_d_n2, assign53520_e81811_d_n4, assign53520_e81811_d_n5, assign53520_e81811_d_n6, assign53520_e81811_d_n7, assign53520_e81811_d_n8, assign53520_e81811_d_n9, assign53520_e81811_d_n10, assign53520_e81811_d_n11, assign53520_e81811_d_n14,) = {
    if (((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_guard1357 == 0.0)) && (locals.var_guard1358 != 0.0)) {
        let assign53520_e81809: f64 = (locals.var_beta * locals.var_t2);
        (assign53520_e81809, ((locals.var_beta_dn0 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn0)), ((locals.var_beta_dn2 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn2)), ((locals.var_beta_dn4 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn4)), ((locals.var_beta_dn5 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn5)), ((locals.var_beta_dn6 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn6)), ((locals.var_beta_dn7 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn7)), ((locals.var_beta_dn8 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn8)), ((locals.var_beta_dn9 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn9)), ((locals.var_beta_dn10 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn10)), ((locals.var_beta_dn11 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn11)), ((locals.var_beta_dn14 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn14)),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign53520_e81811;
        locals.var_t3_dn0 = assign53520_e81811_d_n0;
        locals.var_t3_dn2 = assign53520_e81811_d_n2;
        locals.var_t3_dn4 = assign53520_e81811_d_n4;
        locals.var_t3_dn5 = assign53520_e81811_d_n5;
        locals.var_t3_dn6 = assign53520_e81811_d_n6;
        locals.var_t3_dn7 = assign53520_e81811_d_n7;
        locals.var_t3_dn8 = assign53520_e81811_d_n8;
        locals.var_t3_dn9 = assign53520_e81811_d_n9;
        locals.var_t3_dn10 = assign53520_e81811_d_n10;
        locals.var_t3_dn11 = assign53520_e81811_d_n11;
        locals.var_t3_dn14 = assign53520_e81811_d_n14;

        let (assign53530_e81841, assign53530_e81841_d_n0, assign53530_e81841_d_n2, assign53530_e81841_d_n4, assign53530_e81841_d_n5, assign53530_e81841_d_n6, assign53530_e81841_d_n7, assign53530_e81841_d_n8, assign53530_e81841_d_n9, assign53530_e81841_d_n10, assign53530_e81841_d_n11, assign53530_e81841_d_n14,) = {
    if (((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_guard1357 == 0.0)) && (locals.var_guard1358 != 0.0)) {
        let assign53530_e81830: f64 = (locals.var_c_2esipq_ndepm__blk1140 * locals.var_beta_inv);
        let assign53530_e81832: f64 = (locals.var_t3).exp();
        let assign53530_e81834: f64 = (assign53530_e81832 - locals.var_t3);
        let assign53530_e81836: f64 = (assign53530_e81834 - 1.0);
        let assign53530_e81837: f64 = (assign53530_e81830 * assign53530_e81836);
        let assign53530_e81838: f64 = (assign53530_e81837).sqrt();
        let assign53530_e81839: f64 = (-assign53530_e81838);
        (assign53530_e81839, (-(((((locals.var_c_2esipq_ndepm__blk1140_dn0 * locals.var_beta_inv) + (locals.var_c_2esipq_ndepm__blk1140 * locals.var_beta_inv_dn0)) * assign53530_e81836) + (assign53530_e81830 * ((assign53530_e81832 * locals.var_t3_dn0) - locals.var_t3_dn0))) / (2.0 * assign53530_e81838))), (-(((((locals.var_c_2esipq_ndepm__blk1140_dn2 * locals.var_beta_inv) + (locals.var_c_2esipq_ndepm__blk1140 * locals.var_beta_inv_dn2)) * assign53530_e81836) + (assign53530_e81830 * ((assign53530_e81832 * locals.var_t3_dn2) - locals.var_t3_dn2))) / (2.0 * assign53530_e81838))), (-(((((locals.var_c_2esipq_ndepm__blk1140_dn4 * locals.var_beta_inv) + (locals.var_c_2esipq_ndepm__blk1140 * locals.var_beta_inv_dn4)) * assign53530_e81836) + (assign53530_e81830 * ((assign53530_e81832 * locals.var_t3_dn4) - locals.var_t3_dn4))) / (2.0 * assign53530_e81838))), (-(((((locals.var_c_2esipq_ndepm__blk1140_dn5 * locals.var_beta_inv) + (locals.var_c_2esipq_ndepm__blk1140 * locals.var_beta_inv_dn5)) * assign53530_e81836) + (assign53530_e81830 * ((assign53530_e81832 * locals.var_t3_dn5) - locals.var_t3_dn5))) / (2.0 * assign53530_e81838))), (-(((((locals.var_c_2esipq_ndepm__blk1140_dn6 * locals.var_beta_inv) + (locals.var_c_2esipq_ndepm__blk1140 * locals.var_beta_inv_dn6)) * assign53530_e81836) + (assign53530_e81830 * ((assign53530_e81832 * locals.var_t3_dn6) - locals.var_t3_dn6))) / (2.0 * assign53530_e81838))), (-(((((locals.var_c_2esipq_ndepm__blk1140_dn7 * locals.var_beta_inv) + (locals.var_c_2esipq_ndepm__blk1140 * locals.var_beta_inv_dn7)) * assign53530_e81836) + (assign53530_e81830 * ((assign53530_e81832 * locals.var_t3_dn7) - locals.var_t3_dn7))) / (2.0 * assign53530_e81838))), (-(((((locals.var_c_2esipq_ndepm__blk1140_dn8 * locals.var_beta_inv) + (locals.var_c_2esipq_ndepm__blk1140 * locals.var_beta_inv_dn8)) * assign53530_e81836) + (assign53530_e81830 * ((assign53530_e81832 * locals.var_t3_dn8) - locals.var_t3_dn8))) / (2.0 * assign53530_e81838))), (-(((((locals.var_c_2esipq_ndepm__blk1140_dn9 * locals.var_beta_inv) + (locals.var_c_2esipq_ndepm__blk1140 * locals.var_beta_inv_dn9)) * assign53530_e81836) + (assign53530_e81830 * ((assign53530_e81832 * locals.var_t3_dn9) - locals.var_t3_dn9))) / (2.0 * assign53530_e81838))), (-(((((locals.var_c_2esipq_ndepm__blk1140_dn10 * locals.var_beta_inv) + (locals.var_c_2esipq_ndepm__blk1140 * locals.var_beta_inv_dn10)) * assign53530_e81836) + (assign53530_e81830 * ((assign53530_e81832 * locals.var_t3_dn10) - locals.var_t3_dn10))) / (2.0 * assign53530_e81838))), (-(((((locals.var_c_2esipq_ndepm__blk1140_dn11 * locals.var_beta_inv) + (locals.var_c_2esipq_ndepm__blk1140 * locals.var_beta_inv_dn11)) * assign53530_e81836) + (assign53530_e81830 * ((assign53530_e81832 * locals.var_t3_dn11) - locals.var_t3_dn11))) / (2.0 * assign53530_e81838))), (-(((((locals.var_c_2esipq_ndepm__blk1140_dn14 * locals.var_beta_inv) + (locals.var_c_2esipq_ndepm__blk1140 * locals.var_beta_inv_dn14)) * assign53530_e81836) + (assign53530_e81830 * ((assign53530_e81832 * locals.var_t3_dn14) - locals.var_t3_dn14))) / (2.0 * assign53530_e81838))),)
    } else {
        (locals.var_ws__blk1151, locals.var_ws__blk1151_dn0, locals.var_ws__blk1151_dn2, locals.var_ws__blk1151_dn4, locals.var_ws__blk1151_dn5, locals.var_ws__blk1151_dn6, locals.var_ws__blk1151_dn7, locals.var_ws__blk1151_dn8, locals.var_ws__blk1151_dn9, locals.var_ws__blk1151_dn10, locals.var_ws__blk1151_dn11, locals.var_ws__blk1151_dn14,)
    }
};
        locals.var_ws__blk1151 = assign53530_e81841;
        locals.var_ws__blk1151_dn0 = assign53530_e81841_d_n0;
        locals.var_ws__blk1151_dn2 = assign53530_e81841_d_n2;
        locals.var_ws__blk1151_dn4 = assign53530_e81841_d_n4;
        locals.var_ws__blk1151_dn5 = assign53530_e81841_d_n5;
        locals.var_ws__blk1151_dn6 = assign53530_e81841_d_n6;
        locals.var_ws__blk1151_dn7 = assign53530_e81841_d_n7;
        locals.var_ws__blk1151_dn8 = assign53530_e81841_d_n8;
        locals.var_ws__blk1151_dn9 = assign53530_e81841_d_n9;
        locals.var_ws__blk1151_dn10 = assign53530_e81841_d_n10;
        locals.var_ws__blk1151_dn11 = assign53530_e81841_d_n11;
        locals.var_ws__blk1151_dn14 = assign53530_e81841_d_n14;

        let (assign53540_e81864, assign53540_e81864_d_n0, assign53540_e81864_d_n2, assign53540_e81864_d_n4, assign53540_e81864_d_n5, assign53540_e81864_d_n6, assign53540_e81864_d_n7, assign53540_e81864_d_n8, assign53540_e81864_d_n9, assign53540_e81864_d_n10, assign53540_e81864_d_n11, assign53540_e81864_d_n14,) = {
    if (((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_guard1357 == 0.0)) && (locals.var_guard1358 == 0.0)) {
        let assign53540_e81860: f64 = (-locals.var_beta);
        let assign53540_e81862: f64 = (assign53540_e81860 * locals.var_t2);
        (assign53540_e81862, (((-locals.var_beta_dn0) * locals.var_t2) + (assign53540_e81860 * locals.var_t2_dn0)), (((-locals.var_beta_dn2) * locals.var_t2) + (assign53540_e81860 * locals.var_t2_dn2)), (((-locals.var_beta_dn4) * locals.var_t2) + (assign53540_e81860 * locals.var_t2_dn4)), (((-locals.var_beta_dn5) * locals.var_t2) + (assign53540_e81860 * locals.var_t2_dn5)), (((-locals.var_beta_dn6) * locals.var_t2) + (assign53540_e81860 * locals.var_t2_dn6)), (((-locals.var_beta_dn7) * locals.var_t2) + (assign53540_e81860 * locals.var_t2_dn7)), (((-locals.var_beta_dn8) * locals.var_t2) + (assign53540_e81860 * locals.var_t2_dn8)), (((-locals.var_beta_dn9) * locals.var_t2) + (assign53540_e81860 * locals.var_t2_dn9)), (((-locals.var_beta_dn10) * locals.var_t2) + (assign53540_e81860 * locals.var_t2_dn10)), (((-locals.var_beta_dn11) * locals.var_t2) + (assign53540_e81860 * locals.var_t2_dn11)), (((-locals.var_beta_dn14) * locals.var_t2) + (assign53540_e81860 * locals.var_t2_dn14)),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign53540_e81864;
        locals.var_t3_dn0 = assign53540_e81864_d_n0;
        locals.var_t3_dn2 = assign53540_e81864_d_n2;
        locals.var_t3_dn4 = assign53540_e81864_d_n4;
        locals.var_t3_dn5 = assign53540_e81864_d_n5;
        locals.var_t3_dn6 = assign53540_e81864_d_n6;
        locals.var_t3_dn7 = assign53540_e81864_d_n7;
        locals.var_t3_dn8 = assign53540_e81864_d_n8;
        locals.var_t3_dn9 = assign53540_e81864_d_n9;
        locals.var_t3_dn10 = assign53540_e81864_d_n10;
        locals.var_t3_dn11 = assign53540_e81864_d_n11;
        locals.var_t3_dn14 = assign53540_e81864_d_n14;

        let (assign53550_e81894, assign53550_e81894_d_n0, assign53550_e81894_d_n2, assign53550_e81894_d_n4, assign53550_e81894_d_n5, assign53550_e81894_d_n6, assign53550_e81894_d_n7, assign53550_e81894_d_n8, assign53550_e81894_d_n9, assign53550_e81894_d_n10, assign53550_e81894_d_n11, assign53550_e81894_d_n14,) = {
    if (((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_guard1357 == 0.0)) && (locals.var_guard1358 == 0.0)) {
        let assign53550_e81884: f64 = (locals.var_c_2esipq_ndepm__blk1140 * locals.var_beta_inv);
        let assign53550_e81886: f64 = (locals.var_t3).exp();
        let assign53550_e81888: f64 = (assign53550_e81886 - locals.var_t3);
        let assign53550_e81890: f64 = (assign53550_e81888 - 1.0);
        let assign53550_e81891: f64 = (assign53550_e81884 * assign53550_e81890);
        let assign53550_e81892: f64 = (assign53550_e81891).sqrt();
        (assign53550_e81892, (((((locals.var_c_2esipq_ndepm__blk1140_dn0 * locals.var_beta_inv) + (locals.var_c_2esipq_ndepm__blk1140 * locals.var_beta_inv_dn0)) * assign53550_e81890) + (assign53550_e81884 * ((assign53550_e81886 * locals.var_t3_dn0) - locals.var_t3_dn0))) / (2.0 * assign53550_e81892)), (((((locals.var_c_2esipq_ndepm__blk1140_dn2 * locals.var_beta_inv) + (locals.var_c_2esipq_ndepm__blk1140 * locals.var_beta_inv_dn2)) * assign53550_e81890) + (assign53550_e81884 * ((assign53550_e81886 * locals.var_t3_dn2) - locals.var_t3_dn2))) / (2.0 * assign53550_e81892)), (((((locals.var_c_2esipq_ndepm__blk1140_dn4 * locals.var_beta_inv) + (locals.var_c_2esipq_ndepm__blk1140 * locals.var_beta_inv_dn4)) * assign53550_e81890) + (assign53550_e81884 * ((assign53550_e81886 * locals.var_t3_dn4) - locals.var_t3_dn4))) / (2.0 * assign53550_e81892)), (((((locals.var_c_2esipq_ndepm__blk1140_dn5 * locals.var_beta_inv) + (locals.var_c_2esipq_ndepm__blk1140 * locals.var_beta_inv_dn5)) * assign53550_e81890) + (assign53550_e81884 * ((assign53550_e81886 * locals.var_t3_dn5) - locals.var_t3_dn5))) / (2.0 * assign53550_e81892)), (((((locals.var_c_2esipq_ndepm__blk1140_dn6 * locals.var_beta_inv) + (locals.var_c_2esipq_ndepm__blk1140 * locals.var_beta_inv_dn6)) * assign53550_e81890) + (assign53550_e81884 * ((assign53550_e81886 * locals.var_t3_dn6) - locals.var_t3_dn6))) / (2.0 * assign53550_e81892)), (((((locals.var_c_2esipq_ndepm__blk1140_dn7 * locals.var_beta_inv) + (locals.var_c_2esipq_ndepm__blk1140 * locals.var_beta_inv_dn7)) * assign53550_e81890) + (assign53550_e81884 * ((assign53550_e81886 * locals.var_t3_dn7) - locals.var_t3_dn7))) / (2.0 * assign53550_e81892)), (((((locals.var_c_2esipq_ndepm__blk1140_dn8 * locals.var_beta_inv) + (locals.var_c_2esipq_ndepm__blk1140 * locals.var_beta_inv_dn8)) * assign53550_e81890) + (assign53550_e81884 * ((assign53550_e81886 * locals.var_t3_dn8) - locals.var_t3_dn8))) / (2.0 * assign53550_e81892)), (((((locals.var_c_2esipq_ndepm__blk1140_dn9 * locals.var_beta_inv) + (locals.var_c_2esipq_ndepm__blk1140 * locals.var_beta_inv_dn9)) * assign53550_e81890) + (assign53550_e81884 * ((assign53550_e81886 * locals.var_t3_dn9) - locals.var_t3_dn9))) / (2.0 * assign53550_e81892)), (((((locals.var_c_2esipq_ndepm__blk1140_dn10 * locals.var_beta_inv) + (locals.var_c_2esipq_ndepm__blk1140 * locals.var_beta_inv_dn10)) * assign53550_e81890) + (assign53550_e81884 * ((assign53550_e81886 * locals.var_t3_dn10) - locals.var_t3_dn10))) / (2.0 * assign53550_e81892)), (((((locals.var_c_2esipq_ndepm__blk1140_dn11 * locals.var_beta_inv) + (locals.var_c_2esipq_ndepm__blk1140 * locals.var_beta_inv_dn11)) * assign53550_e81890) + (assign53550_e81884 * ((assign53550_e81886 * locals.var_t3_dn11) - locals.var_t3_dn11))) / (2.0 * assign53550_e81892)), (((((locals.var_c_2esipq_ndepm__blk1140_dn14 * locals.var_beta_inv) + (locals.var_c_2esipq_ndepm__blk1140 * locals.var_beta_inv_dn14)) * assign53550_e81890) + (assign53550_e81884 * ((assign53550_e81886 * locals.var_t3_dn14) - locals.var_t3_dn14))) / (2.0 * assign53550_e81892)),)
    } else {
        (locals.var_ws__blk1151, locals.var_ws__blk1151_dn0, locals.var_ws__blk1151_dn2, locals.var_ws__blk1151_dn4, locals.var_ws__blk1151_dn5, locals.var_ws__blk1151_dn6, locals.var_ws__blk1151_dn7, locals.var_ws__blk1151_dn8, locals.var_ws__blk1151_dn9, locals.var_ws__blk1151_dn10, locals.var_ws__blk1151_dn11, locals.var_ws__blk1151_dn14,)
    }
};
        locals.var_ws__blk1151 = assign53550_e81894;
        locals.var_ws__blk1151_dn0 = assign53550_e81894_d_n0;
        locals.var_ws__blk1151_dn2 = assign53550_e81894_d_n2;
        locals.var_ws__blk1151_dn4 = assign53550_e81894_d_n4;
        locals.var_ws__blk1151_dn5 = assign53550_e81894_d_n5;
        locals.var_ws__blk1151_dn6 = assign53550_e81894_d_n6;
        locals.var_ws__blk1151_dn7 = assign53550_e81894_d_n7;
        locals.var_ws__blk1151_dn8 = assign53550_e81894_d_n8;
        locals.var_ws__blk1151_dn9 = assign53550_e81894_d_n9;
        locals.var_ws__blk1151_dn10 = assign53550_e81894_d_n10;
        locals.var_ws__blk1151_dn11 = assign53550_e81894_d_n11;
        locals.var_ws__blk1151_dn14 = assign53550_e81894_d_n14;

        let (assign53560_e81910, assign53560_e81910_d_n0, assign53560_e81910_d_n2, assign53560_e81910_d_n4, assign53560_e81910_d_n5, assign53560_e81910_d_n6, assign53560_e81910_d_n7, assign53560_e81910_d_n8, assign53560_e81910_d_n9, assign53560_e81910_d_n10, assign53560_e81910_d_n11, assign53560_e81910_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) {
        let assign53560_e81908: f64 = (locals.var_tnp__blk1152 - locals.var_ws__blk1151);
        (assign53560_e81908, (locals.var_tnp__blk1152_dn0 - locals.var_ws__blk1151_dn0), (locals.var_tnp__blk1152_dn2 - locals.var_ws__blk1151_dn2), (locals.var_tnp__blk1152_dn4 - locals.var_ws__blk1151_dn4), (locals.var_tnp__blk1152_dn5 - locals.var_ws__blk1151_dn5), (locals.var_tnp__blk1152_dn6 - locals.var_ws__blk1151_dn6), (locals.var_tnp__blk1152_dn7 - locals.var_ws__blk1151_dn7), (locals.var_tnp__blk1152_dn8 - locals.var_ws__blk1151_dn8), (locals.var_tnp__blk1152_dn9 - locals.var_ws__blk1151_dn9), (locals.var_tnp__blk1152_dn10 - locals.var_ws__blk1151_dn10), (locals.var_tnp__blk1152_dn11 - locals.var_ws__blk1151_dn11), (locals.var_tnp__blk1152_dn14 - locals.var_ws__blk1151_dn14),)
    } else {
        (locals.var_w_res, locals.var_w_res_dn0, locals.var_w_res_dn2, locals.var_w_res_dn4, locals.var_w_res_dn5, locals.var_w_res_dn6, locals.var_w_res_dn7, locals.var_w_res_dn8, locals.var_w_res_dn9, locals.var_w_res_dn10, locals.var_w_res_dn11, locals.var_w_res_dn14,)
    }
};
        locals.var_w_res = assign53560_e81910;
        locals.var_w_res_dn0 = assign53560_e81910_d_n0;
        locals.var_w_res_dn2 = assign53560_e81910_d_n2;
        locals.var_w_res_dn4 = assign53560_e81910_d_n4;
        locals.var_w_res_dn5 = assign53560_e81910_d_n5;
        locals.var_w_res_dn6 = assign53560_e81910_d_n6;
        locals.var_w_res_dn7 = assign53560_e81910_d_n7;
        locals.var_w_res_dn8 = assign53560_e81910_d_n8;
        locals.var_w_res_dn9 = assign53560_e81910_d_n9;
        locals.var_w_res_dn10 = assign53560_e81910_d_n10;
        locals.var_w_res_dn11 = assign53560_e81910_d_n11;
        locals.var_w_res_dn14 = assign53560_e81910_d_n14;

        let assign53570_e81914: f64 = 1e-16;
        let assign53570_e81919: f64 = if ((locals.var_w_res < assign53570_e81914) && (1e-16 >= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1359 = assign53570_e81919;

        let (assign53580_e81939, assign53580_e81939_d_n0, assign53580_e81939_d_n2, assign53580_e81939_d_n4, assign53580_e81939_d_n5, assign53580_e81939_d_n6, assign53580_e81939_d_n7, assign53580_e81939_d_n8, assign53580_e81939_d_n9, assign53580_e81939_d_n10, assign53580_e81939_d_n11, assign53580_e81939_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_guard1359 != 0.0)) {
        let assign53580_e81935: f64 = 1e-16;
        let assign53580_e81937: f64 = (assign53580_e81935 - locals.var_w_res);
        (assign53580_e81937, (-locals.var_w_res_dn0), (-locals.var_w_res_dn2), (-locals.var_w_res_dn4), (-locals.var_w_res_dn5), (-locals.var_w_res_dn6), (-locals.var_w_res_dn7), (-locals.var_w_res_dn8), (-locals.var_w_res_dn9), (-locals.var_w_res_dn10), (-locals.var_w_res_dn11), (-locals.var_w_res_dn14),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign53580_e81939;
        locals.var_tmf1_dn0 = assign53580_e81939_d_n0;
        locals.var_tmf1_dn2 = assign53580_e81939_d_n2;
        locals.var_tmf1_dn4 = assign53580_e81939_d_n4;
        locals.var_tmf1_dn5 = assign53580_e81939_d_n5;
        locals.var_tmf1_dn6 = assign53580_e81939_d_n6;
        locals.var_tmf1_dn7 = assign53580_e81939_d_n7;
        locals.var_tmf1_dn8 = assign53580_e81939_d_n8;
        locals.var_tmf1_dn9 = assign53580_e81939_d_n9;
        locals.var_tmf1_dn10 = assign53580_e81939_d_n10;
        locals.var_tmf1_dn11 = assign53580_e81939_d_n11;
        locals.var_tmf1_dn14 = assign53580_e81939_d_n14;

        let (assign53590_e81957, assign53590_e81957_d_n0, assign53590_e81957_d_n2, assign53590_e81957_d_n4, assign53590_e81957_d_n5, assign53590_e81957_d_n6, assign53590_e81957_d_n7, assign53590_e81957_d_n8, assign53590_e81957_d_n9, assign53590_e81957_d_n10, assign53590_e81957_d_n11, assign53590_e81957_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_guard1359 != 0.0)) {
        let assign53590_e81955: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign53590_e81955, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)), ((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)), ((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)), ((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn4, locals.var_x2_dn5, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn8, locals.var_x2_dn9, locals.var_x2_dn10, locals.var_x2_dn11, locals.var_x2_dn14,)
    }
};
        locals.var_x2 = assign53590_e81957;
        locals.var_x2_dn0 = assign53590_e81957_d_n0;
        locals.var_x2_dn2 = assign53590_e81957_d_n2;
        locals.var_x2_dn4 = assign53590_e81957_d_n4;
        locals.var_x2_dn5 = assign53590_e81957_d_n5;
        locals.var_x2_dn6 = assign53590_e81957_d_n6;
        locals.var_x2_dn7 = assign53590_e81957_d_n7;
        locals.var_x2_dn8 = assign53590_e81957_d_n8;
        locals.var_x2_dn9 = assign53590_e81957_d_n9;
        locals.var_x2_dn10 = assign53590_e81957_d_n10;
        locals.var_x2_dn11 = assign53590_e81957_d_n11;
        locals.var_x2_dn14 = assign53590_e81957_d_n14;

        let (assign53600_e81975, assign53600_e81975_d_n0, assign53600_e81975_d_n2, assign53600_e81975_d_n4, assign53600_e81975_d_n5, assign53600_e81975_d_n6, assign53600_e81975_d_n7, assign53600_e81975_d_n8, assign53600_e81975_d_n9, assign53600_e81975_d_n10, assign53600_e81975_d_n11, assign53600_e81975_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_guard1359 != 0.0)) {
        let assign53600_e81973: f64 = (1e-16 * 1e-16);
        (assign53600_e81973, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn4, locals.var_xmax2_dn5, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn8, locals.var_xmax2_dn9, locals.var_xmax2_dn10, locals.var_xmax2_dn11, locals.var_xmax2_dn14,)
    }
};
        locals.var_xmax2 = assign53600_e81975;
        locals.var_xmax2_dn0 = assign53600_e81975_d_n0;
        locals.var_xmax2_dn2 = assign53600_e81975_d_n2;
        locals.var_xmax2_dn4 = assign53600_e81975_d_n4;
        locals.var_xmax2_dn5 = assign53600_e81975_d_n5;
        locals.var_xmax2_dn6 = assign53600_e81975_d_n6;
        locals.var_xmax2_dn7 = assign53600_e81975_d_n7;
        locals.var_xmax2_dn8 = assign53600_e81975_d_n8;
        locals.var_xmax2_dn9 = assign53600_e81975_d_n9;
        locals.var_xmax2_dn10 = assign53600_e81975_d_n10;
        locals.var_xmax2_dn11 = assign53600_e81975_d_n11;
        locals.var_xmax2_dn14 = assign53600_e81975_d_n14;

        let (assign53610_e81991, assign53610_e81991_d_n0, assign53610_e81991_d_n2, assign53610_e81991_d_n4, assign53610_e81991_d_n5, assign53610_e81991_d_n6, assign53610_e81991_d_n7, assign53610_e81991_d_n8, assign53610_e81991_d_n9, assign53610_e81991_d_n10, assign53610_e81991_d_n11, assign53610_e81991_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_guard1359 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign53610_e81991;
        locals.var_xp_dn0 = assign53610_e81991_d_n0;
        locals.var_xp_dn2 = assign53610_e81991_d_n2;
        locals.var_xp_dn4 = assign53610_e81991_d_n4;
        locals.var_xp_dn5 = assign53610_e81991_d_n5;
        locals.var_xp_dn6 = assign53610_e81991_d_n6;
        locals.var_xp_dn7 = assign53610_e81991_d_n7;
        locals.var_xp_dn8 = assign53610_e81991_d_n8;
        locals.var_xp_dn9 = assign53610_e81991_d_n9;
        locals.var_xp_dn10 = assign53610_e81991_d_n10;
        locals.var_xp_dn11 = assign53610_e81991_d_n11;
        locals.var_xp_dn14 = assign53610_e81991_d_n14;

        let (assign53620_e82007, assign53620_e82007_d_n0, assign53620_e82007_d_n2, assign53620_e82007_d_n4, assign53620_e82007_d_n5, assign53620_e82007_d_n6, assign53620_e82007_d_n7, assign53620_e82007_d_n8, assign53620_e82007_d_n9, assign53620_e82007_d_n10, assign53620_e82007_d_n11, assign53620_e82007_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_guard1359 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign53620_e82007;
        locals.var_xmp_dn0 = assign53620_e82007_d_n0;
        locals.var_xmp_dn2 = assign53620_e82007_d_n2;
        locals.var_xmp_dn4 = assign53620_e82007_d_n4;
        locals.var_xmp_dn5 = assign53620_e82007_d_n5;
        locals.var_xmp_dn6 = assign53620_e82007_d_n6;
        locals.var_xmp_dn7 = assign53620_e82007_d_n7;
        locals.var_xmp_dn8 = assign53620_e82007_d_n8;
        locals.var_xmp_dn9 = assign53620_e82007_d_n9;
        locals.var_xmp_dn10 = assign53620_e82007_d_n10;
        locals.var_xmp_dn11 = assign53620_e82007_d_n11;
        locals.var_xmp_dn14 = assign53620_e82007_d_n14;

        let (assign53630_e82023,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_guard1359 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign53630_e82023;

        let (assign53640_e82039,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_guard1359 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign53640_e82039;

        let (assign53650_e82055, assign53650_e82055_d_n0, assign53650_e82055_d_n2, assign53650_e82055_d_n4, assign53650_e82055_d_n5, assign53650_e82055_d_n6, assign53650_e82055_d_n7, assign53650_e82055_d_n8, assign53650_e82055_d_n9, assign53650_e82055_d_n10, assign53650_e82055_d_n11, assign53650_e82055_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_guard1359 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign53650_e82055;
        locals.var_arg_dn0 = assign53650_e82055_d_n0;
        locals.var_arg_dn2 = assign53650_e82055_d_n2;
        locals.var_arg_dn4 = assign53650_e82055_d_n4;
        locals.var_arg_dn5 = assign53650_e82055_d_n5;
        locals.var_arg_dn6 = assign53650_e82055_d_n6;
        locals.var_arg_dn7 = assign53650_e82055_d_n7;
        locals.var_arg_dn8 = assign53650_e82055_d_n8;
        locals.var_arg_dn9 = assign53650_e82055_d_n9;
        locals.var_arg_dn10 = assign53650_e82055_d_n10;
        locals.var_arg_dn11 = assign53650_e82055_d_n11;
        locals.var_arg_dn14 = assign53650_e82055_d_n14;

        let (assign53660_e82071, assign53660_e82071_d_n0, assign53660_e82071_d_n2, assign53660_e82071_d_n4, assign53660_e82071_d_n5, assign53660_e82071_d_n6, assign53660_e82071_d_n7, assign53660_e82071_d_n8, assign53660_e82071_d_n9, assign53660_e82071_d_n10, assign53660_e82071_d_n11, assign53660_e82071_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_guard1359 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign53660_e82071;
        locals.var_dnm_dn0 = assign53660_e82071_d_n0;
        locals.var_dnm_dn2 = assign53660_e82071_d_n2;
        locals.var_dnm_dn4 = assign53660_e82071_d_n4;
        locals.var_dnm_dn5 = assign53660_e82071_d_n5;
        locals.var_dnm_dn6 = assign53660_e82071_d_n6;
        locals.var_dnm_dn7 = assign53660_e82071_d_n7;
        locals.var_dnm_dn8 = assign53660_e82071_d_n8;
        locals.var_dnm_dn9 = assign53660_e82071_d_n9;
        locals.var_dnm_dn10 = assign53660_e82071_d_n10;
        locals.var_dnm_dn11 = assign53660_e82071_d_n11;
        locals.var_dnm_dn14 = assign53660_e82071_d_n14;

        let (assign53670_e82089, assign53670_e82089_d_n0, assign53670_e82089_d_n2, assign53670_e82089_d_n4, assign53670_e82089_d_n5, assign53670_e82089_d_n6, assign53670_e82089_d_n7, assign53670_e82089_d_n8, assign53670_e82089_d_n9, assign53670_e82089_d_n10, assign53670_e82089_d_n11, assign53670_e82089_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_guard1359 != 0.0)) {
        let assign53670_e82087: f64 = (locals.var_xp * locals.var_x2);
        (assign53670_e82087, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign53670_e82089;
        locals.var_xp_dn0 = assign53670_e82089_d_n0;
        locals.var_xp_dn2 = assign53670_e82089_d_n2;
        locals.var_xp_dn4 = assign53670_e82089_d_n4;
        locals.var_xp_dn5 = assign53670_e82089_d_n5;
        locals.var_xp_dn6 = assign53670_e82089_d_n6;
        locals.var_xp_dn7 = assign53670_e82089_d_n7;
        locals.var_xp_dn8 = assign53670_e82089_d_n8;
        locals.var_xp_dn9 = assign53670_e82089_d_n9;
        locals.var_xp_dn10 = assign53670_e82089_d_n10;
        locals.var_xp_dn11 = assign53670_e82089_d_n11;
        locals.var_xp_dn14 = assign53670_e82089_d_n14;

        let (assign53680_e82107, assign53680_e82107_d_n0, assign53680_e82107_d_n2, assign53680_e82107_d_n4, assign53680_e82107_d_n5, assign53680_e82107_d_n6, assign53680_e82107_d_n7, assign53680_e82107_d_n8, assign53680_e82107_d_n9, assign53680_e82107_d_n10, assign53680_e82107_d_n11, assign53680_e82107_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_guard1359 != 0.0)) {
        let assign53680_e82105: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign53680_e82105, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign53680_e82107;
        locals.var_xmp_dn0 = assign53680_e82107_d_n0;
        locals.var_xmp_dn2 = assign53680_e82107_d_n2;
        locals.var_xmp_dn4 = assign53680_e82107_d_n4;
        locals.var_xmp_dn5 = assign53680_e82107_d_n5;
        locals.var_xmp_dn6 = assign53680_e82107_d_n6;
        locals.var_xmp_dn7 = assign53680_e82107_d_n7;
        locals.var_xmp_dn8 = assign53680_e82107_d_n8;
        locals.var_xmp_dn9 = assign53680_e82107_d_n9;
        locals.var_xmp_dn10 = assign53680_e82107_d_n10;
        locals.var_xmp_dn11 = assign53680_e82107_d_n11;
        locals.var_xmp_dn14 = assign53680_e82107_d_n14;

        let (assign53690_e82125, assign53690_e82125_d_n0, assign53690_e82125_d_n2, assign53690_e82125_d_n4, assign53690_e82125_d_n5, assign53690_e82125_d_n6, assign53690_e82125_d_n7, assign53690_e82125_d_n8, assign53690_e82125_d_n9, assign53690_e82125_d_n10, assign53690_e82125_d_n11, assign53690_e82125_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_guard1359 != 0.0)) {
        let assign53690_e82123: f64 = (locals.var_xp * locals.var_x2);
        (assign53690_e82123, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign53690_e82125;
        locals.var_xp_dn0 = assign53690_e82125_d_n0;
        locals.var_xp_dn2 = assign53690_e82125_d_n2;
        locals.var_xp_dn4 = assign53690_e82125_d_n4;
        locals.var_xp_dn5 = assign53690_e82125_d_n5;
        locals.var_xp_dn6 = assign53690_e82125_d_n6;
        locals.var_xp_dn7 = assign53690_e82125_d_n7;
        locals.var_xp_dn8 = assign53690_e82125_d_n8;
        locals.var_xp_dn9 = assign53690_e82125_d_n9;
        locals.var_xp_dn10 = assign53690_e82125_d_n10;
        locals.var_xp_dn11 = assign53690_e82125_d_n11;
        locals.var_xp_dn14 = assign53690_e82125_d_n14;

        let (assign53700_e82143, assign53700_e82143_d_n0, assign53700_e82143_d_n2, assign53700_e82143_d_n4, assign53700_e82143_d_n5, assign53700_e82143_d_n6, assign53700_e82143_d_n7, assign53700_e82143_d_n8, assign53700_e82143_d_n9, assign53700_e82143_d_n10, assign53700_e82143_d_n11, assign53700_e82143_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_guard1359 != 0.0)) {
        let assign53700_e82141: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign53700_e82141, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign53700_e82143;
        locals.var_xmp_dn0 = assign53700_e82143_d_n0;
        locals.var_xmp_dn2 = assign53700_e82143_d_n2;
        locals.var_xmp_dn4 = assign53700_e82143_d_n4;
        locals.var_xmp_dn5 = assign53700_e82143_d_n5;
        locals.var_xmp_dn6 = assign53700_e82143_d_n6;
        locals.var_xmp_dn7 = assign53700_e82143_d_n7;
        locals.var_xmp_dn8 = assign53700_e82143_d_n8;
        locals.var_xmp_dn9 = assign53700_e82143_d_n9;
        locals.var_xmp_dn10 = assign53700_e82143_d_n10;
        locals.var_xmp_dn11 = assign53700_e82143_d_n11;
        locals.var_xmp_dn14 = assign53700_e82143_d_n14;

    }

    pub(super) fn stamp_transient_block_184(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign53710_e82161, assign53710_e82161_d_n0, assign53710_e82161_d_n2, assign53710_e82161_d_n4, assign53710_e82161_d_n5, assign53710_e82161_d_n6, assign53710_e82161_d_n7, assign53710_e82161_d_n8, assign53710_e82161_d_n9, assign53710_e82161_d_n10, assign53710_e82161_d_n11, assign53710_e82161_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_guard1359 != 0.0)) {
        let assign53710_e82159: f64 = (locals.var_xp + locals.var_xmp);
        (assign53710_e82159, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn4 + locals.var_xmp_dn4), (locals.var_xp_dn5 + locals.var_xmp_dn5), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn8 + locals.var_xmp_dn8), (locals.var_xp_dn9 + locals.var_xmp_dn9), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn11 + locals.var_xmp_dn11), (locals.var_xp_dn14 + locals.var_xmp_dn14),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign53710_e82161;
        locals.var_arg_dn0 = assign53710_e82161_d_n0;
        locals.var_arg_dn2 = assign53710_e82161_d_n2;
        locals.var_arg_dn4 = assign53710_e82161_d_n4;
        locals.var_arg_dn5 = assign53710_e82161_d_n5;
        locals.var_arg_dn6 = assign53710_e82161_d_n6;
        locals.var_arg_dn7 = assign53710_e82161_d_n7;
        locals.var_arg_dn8 = assign53710_e82161_d_n8;
        locals.var_arg_dn9 = assign53710_e82161_d_n9;
        locals.var_arg_dn10 = assign53710_e82161_d_n10;
        locals.var_arg_dn11 = assign53710_e82161_d_n11;
        locals.var_arg_dn14 = assign53710_e82161_d_n14;

        let (assign53720_e82177, assign53720_e82177_d_n0, assign53720_e82177_d_n2, assign53720_e82177_d_n4, assign53720_e82177_d_n5, assign53720_e82177_d_n6, assign53720_e82177_d_n7, assign53720_e82177_d_n8, assign53720_e82177_d_n9, assign53720_e82177_d_n10, assign53720_e82177_d_n11, assign53720_e82177_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_guard1359 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign53720_e82177;
        locals.var_dnm_dn0 = assign53720_e82177_d_n0;
        locals.var_dnm_dn2 = assign53720_e82177_d_n2;
        locals.var_dnm_dn4 = assign53720_e82177_d_n4;
        locals.var_dnm_dn5 = assign53720_e82177_d_n5;
        locals.var_dnm_dn6 = assign53720_e82177_d_n6;
        locals.var_dnm_dn7 = assign53720_e82177_d_n7;
        locals.var_dnm_dn8 = assign53720_e82177_d_n8;
        locals.var_dnm_dn9 = assign53720_e82177_d_n9;
        locals.var_dnm_dn10 = assign53720_e82177_d_n10;
        locals.var_dnm_dn11 = assign53720_e82177_d_n11;
        locals.var_dnm_dn14 = assign53720_e82177_d_n14;

        let assign53730_e82192: f64 = if ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard1360 = assign53730_e82192;

        let assign53740_e82195: f64 = if 2.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1361 = assign53740_e82195;

        let (assign53750_e82215,) = {
    if ((((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_guard1359 != 0.0)) && (locals.var_guard1360 != 0.0)) && (locals.var_guard1361 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign53750_e82215;

        let assign53760_e82218: f64 = if 2.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard1362 = assign53760_e82218;

        let (assign53770_e82241,) = {
    if (((((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_guard1359 != 0.0)) && (locals.var_guard1360 != 0.0)) && (locals.var_guard1361 == 0.0)) && (locals.var_guard1362 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign53770_e82241;

        let assign53780_e82244: f64 = if 2.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard1363 = assign53780_e82244;

        let (assign53790_e82270,) = {
    if ((((((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_guard1359 != 0.0)) && (locals.var_guard1360 != 0.0)) && (locals.var_guard1361 == 0.0)) && (locals.var_guard1362 == 0.0)) && (locals.var_guard1363 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign53790_e82270;

        let assign53800_e82273: f64 = if 2.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard1364 = assign53800_e82273;

        let (assign53810_e82302,) = {
    if (((((((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_guard1359 != 0.0)) && (locals.var_guard1360 != 0.0)) && (locals.var_guard1361 == 0.0)) && (locals.var_guard1362 == 0.0)) && (locals.var_guard1363 == 0.0)) && (locals.var_guard1364 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign53810_e82302;

        let (assign53820_e82320,) = {
    if (((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_guard1359 != 0.0)) && (locals.var_guard1360 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign53820_e82320;

        let mut assign53830_loop_guard: usize = 0;
        while {
            let assign53830_cond_e82339: f64 = if ((((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_guard1359 != 0.0)) && (locals.var_guard1360 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign53830_cond_e82339 != 0.0
        } {
            assign53830_loop_guard += 1;
            assert!(assign53830_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign53830_body0_e82358, assign53830_body0_e82358_d_n0, assign53830_body0_e82358_d_n2, assign53830_body0_e82358_d_n4, assign53830_body0_e82358_d_n5, assign53830_body0_e82358_d_n6, assign53830_body0_e82358_d_n7, assign53830_body0_e82358_d_n8, assign53830_body0_e82358_d_n9, assign53830_body0_e82358_d_n10, assign53830_body0_e82358_d_n11, assign53830_body0_e82358_d_n14,) = {
    if (((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_guard1359 != 0.0)) && (locals.var_guard1360 != 0.0)) {
        let assign53830_body0_e82356: f64 = (locals.var_dnm).sqrt();
        (assign53830_body0_e82356, (locals.var_dnm_dn0 / (2.0 * assign53830_body0_e82356)), (locals.var_dnm_dn2 / (2.0 * assign53830_body0_e82356)), (locals.var_dnm_dn4 / (2.0 * assign53830_body0_e82356)), (locals.var_dnm_dn5 / (2.0 * assign53830_body0_e82356)), (locals.var_dnm_dn6 / (2.0 * assign53830_body0_e82356)), (locals.var_dnm_dn7 / (2.0 * assign53830_body0_e82356)), (locals.var_dnm_dn8 / (2.0 * assign53830_body0_e82356)), (locals.var_dnm_dn9 / (2.0 * assign53830_body0_e82356)), (locals.var_dnm_dn10 / (2.0 * assign53830_body0_e82356)), (locals.var_dnm_dn11 / (2.0 * assign53830_body0_e82356)), (locals.var_dnm_dn14 / (2.0 * assign53830_body0_e82356)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
            locals.var_dnm = assign53830_body0_e82358;
            locals.var_dnm_dn0 = assign53830_body0_e82358_d_n0;
            locals.var_dnm_dn2 = assign53830_body0_e82358_d_n2;
            locals.var_dnm_dn4 = assign53830_body0_e82358_d_n4;
            locals.var_dnm_dn5 = assign53830_body0_e82358_d_n5;
            locals.var_dnm_dn6 = assign53830_body0_e82358_d_n6;
            locals.var_dnm_dn7 = assign53830_body0_e82358_d_n7;
            locals.var_dnm_dn8 = assign53830_body0_e82358_d_n8;
            locals.var_dnm_dn9 = assign53830_body0_e82358_d_n9;
            locals.var_dnm_dn10 = assign53830_body0_e82358_d_n10;
            locals.var_dnm_dn11 = assign53830_body0_e82358_d_n11;
            locals.var_dnm_dn14 = assign53830_body0_e82358_d_n14;
            let (assign53830_body1_e82378,) = {
    if (((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_guard1359 != 0.0)) && (locals.var_guard1360 != 0.0)) {
        let assign53830_body1_e82376: f64 = (locals.var_m0 + 1.0);
        (assign53830_body1_e82376,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign53830_body1_e82378;
        }

        let (assign53840_e82408, assign53840_e82408_d_n0, assign53840_e82408_d_n2, assign53840_e82408_d_n4, assign53840_e82408_d_n5, assign53840_e82408_d_n6, assign53840_e82408_d_n7, assign53840_e82408_d_n8, assign53840_e82408_d_n9, assign53840_e82408_d_n10, assign53840_e82408_d_n11, assign53840_e82408_d_n14,) = {
    if (((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_guard1359 != 0.0)) && (locals.var_guard1360 == 0.0)) {
        let (assign53840_e82406, assign53840_e82406_d_n0, assign53840_e82406_d_n2, assign53840_e82406_d_n4, assign53840_e82406_d_n5, assign53840_e82406_d_n6, assign53840_e82406_d_n7, assign53840_e82406_d_n8, assign53840_e82406_d_n9, assign53840_e82406_d_n10, assign53840_e82406_d_n11, assign53840_e82406_d_n14,) = {
            if (locals.var_dnm == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign53840_e82403: f64 = (2.0 * 2.0);
                let assign53840_e82404: f64 = (1.0 / assign53840_e82403);
                let assign53840_e82405: f64 = (locals.var_dnm).powf(assign53840_e82404);
                (assign53840_e82405, if 0.0 == 0.0 && ((assign53840_e82404) as f64).is_finite() && ((assign53840_e82404) as f64).fract() == 0.0 { if assign53840_e82404 == 0.0 { 0.0 } else { (assign53840_e82404 * ((locals.var_dnm).powf(assign53840_e82404 - 1.0) * locals.var_dnm_dn0)) } } else { (assign53840_e82405 * (assign53840_e82404 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign53840_e82404) as f64).is_finite() && ((assign53840_e82404) as f64).fract() == 0.0 { if assign53840_e82404 == 0.0 { 0.0 } else { (assign53840_e82404 * ((locals.var_dnm).powf(assign53840_e82404 - 1.0) * locals.var_dnm_dn2)) } } else { (assign53840_e82405 * (assign53840_e82404 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign53840_e82404) as f64).is_finite() && ((assign53840_e82404) as f64).fract() == 0.0 { if assign53840_e82404 == 0.0 { 0.0 } else { (assign53840_e82404 * ((locals.var_dnm).powf(assign53840_e82404 - 1.0) * locals.var_dnm_dn4)) } } else { (assign53840_e82405 * (assign53840_e82404 * (locals.var_dnm_dn4 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign53840_e82404) as f64).is_finite() && ((assign53840_e82404) as f64).fract() == 0.0 { if assign53840_e82404 == 0.0 { 0.0 } else { (assign53840_e82404 * ((locals.var_dnm).powf(assign53840_e82404 - 1.0) * locals.var_dnm_dn5)) } } else { (assign53840_e82405 * (assign53840_e82404 * (locals.var_dnm_dn5 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign53840_e82404) as f64).is_finite() && ((assign53840_e82404) as f64).fract() == 0.0 { if assign53840_e82404 == 0.0 { 0.0 } else { (assign53840_e82404 * ((locals.var_dnm).powf(assign53840_e82404 - 1.0) * locals.var_dnm_dn6)) } } else { (assign53840_e82405 * (assign53840_e82404 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign53840_e82404) as f64).is_finite() && ((assign53840_e82404) as f64).fract() == 0.0 { if assign53840_e82404 == 0.0 { 0.0 } else { (assign53840_e82404 * ((locals.var_dnm).powf(assign53840_e82404 - 1.0) * locals.var_dnm_dn7)) } } else { (assign53840_e82405 * (assign53840_e82404 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign53840_e82404) as f64).is_finite() && ((assign53840_e82404) as f64).fract() == 0.0 { if assign53840_e82404 == 0.0 { 0.0 } else { (assign53840_e82404 * ((locals.var_dnm).powf(assign53840_e82404 - 1.0) * locals.var_dnm_dn8)) } } else { (assign53840_e82405 * (assign53840_e82404 * (locals.var_dnm_dn8 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign53840_e82404) as f64).is_finite() && ((assign53840_e82404) as f64).fract() == 0.0 { if assign53840_e82404 == 0.0 { 0.0 } else { (assign53840_e82404 * ((locals.var_dnm).powf(assign53840_e82404 - 1.0) * locals.var_dnm_dn9)) } } else { (assign53840_e82405 * (assign53840_e82404 * (locals.var_dnm_dn9 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign53840_e82404) as f64).is_finite() && ((assign53840_e82404) as f64).fract() == 0.0 { if assign53840_e82404 == 0.0 { 0.0 } else { (assign53840_e82404 * ((locals.var_dnm).powf(assign53840_e82404 - 1.0) * locals.var_dnm_dn10)) } } else { (assign53840_e82405 * (assign53840_e82404 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign53840_e82404) as f64).is_finite() && ((assign53840_e82404) as f64).fract() == 0.0 { if assign53840_e82404 == 0.0 { 0.0 } else { (assign53840_e82404 * ((locals.var_dnm).powf(assign53840_e82404 - 1.0) * locals.var_dnm_dn11)) } } else { (assign53840_e82405 * (assign53840_e82404 * (locals.var_dnm_dn11 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign53840_e82404) as f64).is_finite() && ((assign53840_e82404) as f64).fract() == 0.0 { if assign53840_e82404 == 0.0 { 0.0 } else { (assign53840_e82404 * ((locals.var_dnm).powf(assign53840_e82404 - 1.0) * locals.var_dnm_dn14)) } } else { (assign53840_e82405 * (assign53840_e82404 * (locals.var_dnm_dn14 / locals.var_dnm))) },)
            }
        };
        (assign53840_e82406, assign53840_e82406_d_n0, assign53840_e82406_d_n2, assign53840_e82406_d_n4, assign53840_e82406_d_n5, assign53840_e82406_d_n6, assign53840_e82406_d_n7, assign53840_e82406_d_n8, assign53840_e82406_d_n9, assign53840_e82406_d_n10, assign53840_e82406_d_n11, assign53840_e82406_d_n14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign53840_e82408;
        locals.var_dnm_dn0 = assign53840_e82408_d_n0;
        locals.var_dnm_dn2 = assign53840_e82408_d_n2;
        locals.var_dnm_dn4 = assign53840_e82408_d_n4;
        locals.var_dnm_dn5 = assign53840_e82408_d_n5;
        locals.var_dnm_dn6 = assign53840_e82408_d_n6;
        locals.var_dnm_dn7 = assign53840_e82408_d_n7;
        locals.var_dnm_dn8 = assign53840_e82408_d_n8;
        locals.var_dnm_dn9 = assign53840_e82408_d_n9;
        locals.var_dnm_dn10 = assign53840_e82408_d_n10;
        locals.var_dnm_dn11 = assign53840_e82408_d_n11;
        locals.var_dnm_dn14 = assign53840_e82408_d_n14;

        let (assign53850_e82426, assign53850_e82426_d_n0, assign53850_e82426_d_n2, assign53850_e82426_d_n4, assign53850_e82426_d_n5, assign53850_e82426_d_n6, assign53850_e82426_d_n7, assign53850_e82426_d_n8, assign53850_e82426_d_n9, assign53850_e82426_d_n10, assign53850_e82426_d_n11, assign53850_e82426_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_guard1359 != 0.0)) {
        let assign53850_e82424: f64 = (1.0 / locals.var_dnm);
        (assign53850_e82424, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn4 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn5 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn8 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn9 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn11 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn14 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign53850_e82426;
        locals.var_dnm_dn0 = assign53850_e82426_d_n0;
        locals.var_dnm_dn2 = assign53850_e82426_d_n2;
        locals.var_dnm_dn4 = assign53850_e82426_d_n4;
        locals.var_dnm_dn5 = assign53850_e82426_d_n5;
        locals.var_dnm_dn6 = assign53850_e82426_d_n6;
        locals.var_dnm_dn7 = assign53850_e82426_d_n7;
        locals.var_dnm_dn8 = assign53850_e82426_d_n8;
        locals.var_dnm_dn9 = assign53850_e82426_d_n9;
        locals.var_dnm_dn10 = assign53850_e82426_d_n10;
        locals.var_dnm_dn11 = assign53850_e82426_d_n11;
        locals.var_dnm_dn14 = assign53850_e82426_d_n14;

        let (assign53860_e82446, assign53860_e82446_d_n0, assign53860_e82446_d_n2, assign53860_e82446_d_n4, assign53860_e82446_d_n5, assign53860_e82446_d_n6, assign53860_e82446_d_n7, assign53860_e82446_d_n8, assign53860_e82446_d_n9, assign53860_e82446_d_n10, assign53860_e82446_d_n11, assign53860_e82446_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_guard1359 != 0.0)) {
        let assign53860_e82442: f64 = (locals.var_tmf1 * 1e-16);
        let assign53860_e82444: f64 = (assign53860_e82442 * locals.var_dnm);
        (assign53860_e82444, (((locals.var_tmf1_dn0 * 1e-16) * locals.var_dnm) + (assign53860_e82442 * locals.var_dnm_dn0)), (((locals.var_tmf1_dn2 * 1e-16) * locals.var_dnm) + (assign53860_e82442 * locals.var_dnm_dn2)), (((locals.var_tmf1_dn4 * 1e-16) * locals.var_dnm) + (assign53860_e82442 * locals.var_dnm_dn4)), (((locals.var_tmf1_dn5 * 1e-16) * locals.var_dnm) + (assign53860_e82442 * locals.var_dnm_dn5)), (((locals.var_tmf1_dn6 * 1e-16) * locals.var_dnm) + (assign53860_e82442 * locals.var_dnm_dn6)), (((locals.var_tmf1_dn7 * 1e-16) * locals.var_dnm) + (assign53860_e82442 * locals.var_dnm_dn7)), (((locals.var_tmf1_dn8 * 1e-16) * locals.var_dnm) + (assign53860_e82442 * locals.var_dnm_dn8)), (((locals.var_tmf1_dn9 * 1e-16) * locals.var_dnm) + (assign53860_e82442 * locals.var_dnm_dn9)), (((locals.var_tmf1_dn10 * 1e-16) * locals.var_dnm) + (assign53860_e82442 * locals.var_dnm_dn10)), (((locals.var_tmf1_dn11 * 1e-16) * locals.var_dnm) + (assign53860_e82442 * locals.var_dnm_dn11)), (((locals.var_tmf1_dn14 * 1e-16) * locals.var_dnm) + (assign53860_e82442 * locals.var_dnm_dn14)),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn11, locals.var_tmf0_dn14,)
    }
};
        locals.var_tmf0 = assign53860_e82446;
        locals.var_tmf0_dn0 = assign53860_e82446_d_n0;
        locals.var_tmf0_dn2 = assign53860_e82446_d_n2;
        locals.var_tmf0_dn4 = assign53860_e82446_d_n4;
        locals.var_tmf0_dn5 = assign53860_e82446_d_n5;
        locals.var_tmf0_dn6 = assign53860_e82446_d_n6;
        locals.var_tmf0_dn7 = assign53860_e82446_d_n7;
        locals.var_tmf0_dn8 = assign53860_e82446_d_n8;
        locals.var_tmf0_dn9 = assign53860_e82446_d_n9;
        locals.var_tmf0_dn10 = assign53860_e82446_d_n10;
        locals.var_tmf0_dn11 = assign53860_e82446_d_n11;
        locals.var_tmf0_dn14 = assign53860_e82446_d_n14;

        let (assign53870_e82468, assign53870_e82468_d_n0, assign53870_e82468_d_n2, assign53870_e82468_d_n4, assign53870_e82468_d_n5, assign53870_e82468_d_n6, assign53870_e82468_d_n7, assign53870_e82468_d_n8, assign53870_e82468_d_n9, assign53870_e82468_d_n10, assign53870_e82468_d_n11, assign53870_e82468_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_guard1359 != 0.0)) {
        let assign53870_e82462: f64 = (1e-16 * locals.var_xmp);
        let assign53870_e82464: f64 = (assign53870_e82462 * locals.var_dnm);
        let assign53870_e82466: f64 = (assign53870_e82464 / locals.var_arg);
        (assign53870_e82466, ((((((1e-16 * locals.var_xmp_dn0) * locals.var_dnm) + (assign53870_e82462 * locals.var_dnm_dn0)) * locals.var_arg) - (assign53870_e82464 * locals.var_arg_dn0)) / (locals.var_arg * locals.var_arg)), ((((((1e-16 * locals.var_xmp_dn2) * locals.var_dnm) + (assign53870_e82462 * locals.var_dnm_dn2)) * locals.var_arg) - (assign53870_e82464 * locals.var_arg_dn2)) / (locals.var_arg * locals.var_arg)), ((((((1e-16 * locals.var_xmp_dn4) * locals.var_dnm) + (assign53870_e82462 * locals.var_dnm_dn4)) * locals.var_arg) - (assign53870_e82464 * locals.var_arg_dn4)) / (locals.var_arg * locals.var_arg)), ((((((1e-16 * locals.var_xmp_dn5) * locals.var_dnm) + (assign53870_e82462 * locals.var_dnm_dn5)) * locals.var_arg) - (assign53870_e82464 * locals.var_arg_dn5)) / (locals.var_arg * locals.var_arg)), ((((((1e-16 * locals.var_xmp_dn6) * locals.var_dnm) + (assign53870_e82462 * locals.var_dnm_dn6)) * locals.var_arg) - (assign53870_e82464 * locals.var_arg_dn6)) / (locals.var_arg * locals.var_arg)), ((((((1e-16 * locals.var_xmp_dn7) * locals.var_dnm) + (assign53870_e82462 * locals.var_dnm_dn7)) * locals.var_arg) - (assign53870_e82464 * locals.var_arg_dn7)) / (locals.var_arg * locals.var_arg)), ((((((1e-16 * locals.var_xmp_dn8) * locals.var_dnm) + (assign53870_e82462 * locals.var_dnm_dn8)) * locals.var_arg) - (assign53870_e82464 * locals.var_arg_dn8)) / (locals.var_arg * locals.var_arg)), ((((((1e-16 * locals.var_xmp_dn9) * locals.var_dnm) + (assign53870_e82462 * locals.var_dnm_dn9)) * locals.var_arg) - (assign53870_e82464 * locals.var_arg_dn9)) / (locals.var_arg * locals.var_arg)), ((((((1e-16 * locals.var_xmp_dn10) * locals.var_dnm) + (assign53870_e82462 * locals.var_dnm_dn10)) * locals.var_arg) - (assign53870_e82464 * locals.var_arg_dn10)) / (locals.var_arg * locals.var_arg)), ((((((1e-16 * locals.var_xmp_dn11) * locals.var_dnm) + (assign53870_e82462 * locals.var_dnm_dn11)) * locals.var_arg) - (assign53870_e82464 * locals.var_arg_dn11)) / (locals.var_arg * locals.var_arg)), ((((((1e-16 * locals.var_xmp_dn14) * locals.var_dnm) + (assign53870_e82462 * locals.var_dnm_dn14)) * locals.var_arg) - (assign53870_e82464 * locals.var_arg_dn14)) / (locals.var_arg * locals.var_arg)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign53870_e82468;
        locals.var_t0_dn0 = assign53870_e82468_d_n0;
        locals.var_t0_dn2 = assign53870_e82468_d_n2;
        locals.var_t0_dn4 = assign53870_e82468_d_n4;
        locals.var_t0_dn5 = assign53870_e82468_d_n5;
        locals.var_t0_dn6 = assign53870_e82468_d_n6;
        locals.var_t0_dn7 = assign53870_e82468_d_n7;
        locals.var_t0_dn8 = assign53870_e82468_d_n8;
        locals.var_t0_dn9 = assign53870_e82468_d_n9;
        locals.var_t0_dn10 = assign53870_e82468_d_n10;
        locals.var_t0_dn11 = assign53870_e82468_d_n11;
        locals.var_t0_dn14 = assign53870_e82468_d_n14;

        let (assign53880_e82488, assign53880_e82488_d_n0, assign53880_e82488_d_n2, assign53880_e82488_d_n4, assign53880_e82488_d_n5, assign53880_e82488_d_n6, assign53880_e82488_d_n7, assign53880_e82488_d_n8, assign53880_e82488_d_n9, assign53880_e82488_d_n10, assign53880_e82488_d_n11, assign53880_e82488_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_guard1359 != 0.0)) {
        let assign53880_e82484: f64 = 1e-16;
        let assign53880_e82486: f64 = (assign53880_e82484 - locals.var_tmf0);
        (assign53880_e82486, (-locals.var_tmf0_dn0), (-locals.var_tmf0_dn2), (-locals.var_tmf0_dn4), (-locals.var_tmf0_dn5), (-locals.var_tmf0_dn6), (-locals.var_tmf0_dn7), (-locals.var_tmf0_dn8), (-locals.var_tmf0_dn9), (-locals.var_tmf0_dn10), (-locals.var_tmf0_dn11), (-locals.var_tmf0_dn14),)
    } else {
        (locals.var_w_res, locals.var_w_res_dn0, locals.var_w_res_dn2, locals.var_w_res_dn4, locals.var_w_res_dn5, locals.var_w_res_dn6, locals.var_w_res_dn7, locals.var_w_res_dn8, locals.var_w_res_dn9, locals.var_w_res_dn10, locals.var_w_res_dn11, locals.var_w_res_dn14,)
    }
};
        locals.var_w_res = assign53880_e82488;
        locals.var_w_res_dn0 = assign53880_e82488_d_n0;
        locals.var_w_res_dn2 = assign53880_e82488_d_n2;
        locals.var_w_res_dn4 = assign53880_e82488_d_n4;
        locals.var_w_res_dn5 = assign53880_e82488_d_n5;
        locals.var_w_res_dn6 = assign53880_e82488_d_n6;
        locals.var_w_res_dn7 = assign53880_e82488_d_n7;
        locals.var_w_res_dn8 = assign53880_e82488_d_n8;
        locals.var_w_res_dn9 = assign53880_e82488_d_n9;
        locals.var_w_res_dn10 = assign53880_e82488_d_n10;
        locals.var_w_res_dn11 = assign53880_e82488_d_n11;
        locals.var_w_res_dn14 = assign53880_e82488_d_n14;

        let (assign53890_e82504, assign53890_e82504_d_n0, assign53890_e82504_d_n2, assign53890_e82504_d_n4, assign53890_e82504_d_n5, assign53890_e82504_d_n6, assign53890_e82504_d_n7, assign53890_e82504_d_n8, assign53890_e82504_d_n9, assign53890_e82504_d_n10, assign53890_e82504_d_n11, assign53890_e82504_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_guard1359 != 0.0)) {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign53890_e82504;
        locals.var_t0_dn0 = assign53890_e82504_d_n0;
        locals.var_t0_dn2 = assign53890_e82504_d_n2;
        locals.var_t0_dn4 = assign53890_e82504_d_n4;
        locals.var_t0_dn5 = assign53890_e82504_d_n5;
        locals.var_t0_dn6 = assign53890_e82504_d_n6;
        locals.var_t0_dn7 = assign53890_e82504_d_n7;
        locals.var_t0_dn8 = assign53890_e82504_d_n8;
        locals.var_t0_dn9 = assign53890_e82504_d_n9;
        locals.var_t0_dn10 = assign53890_e82504_d_n10;
        locals.var_t0_dn11 = assign53890_e82504_d_n11;
        locals.var_t0_dn14 = assign53890_e82504_d_n14;

        let (assign53900_e82521, assign53900_e82521_d_n0, assign53900_e82521_d_n2, assign53900_e82521_d_n4, assign53900_e82521_d_n5, assign53900_e82521_d_n6, assign53900_e82521_d_n7, assign53900_e82521_d_n8, assign53900_e82521_d_n9, assign53900_e82521_d_n10, assign53900_e82521_d_n11, assign53900_e82521_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_guard1359 == 0.0)) {
        (locals.var_w_res, locals.var_w_res_dn0, locals.var_w_res_dn2, locals.var_w_res_dn4, locals.var_w_res_dn5, locals.var_w_res_dn6, locals.var_w_res_dn7, locals.var_w_res_dn8, locals.var_w_res_dn9, locals.var_w_res_dn10, locals.var_w_res_dn11, locals.var_w_res_dn14,)
    } else {
        (locals.var_w_res, locals.var_w_res_dn0, locals.var_w_res_dn2, locals.var_w_res_dn4, locals.var_w_res_dn5, locals.var_w_res_dn6, locals.var_w_res_dn7, locals.var_w_res_dn8, locals.var_w_res_dn9, locals.var_w_res_dn10, locals.var_w_res_dn11, locals.var_w_res_dn14,)
    }
};
        locals.var_w_res = assign53900_e82521;
        locals.var_w_res_dn0 = assign53900_e82521_d_n0;
        locals.var_w_res_dn2 = assign53900_e82521_d_n2;
        locals.var_w_res_dn4 = assign53900_e82521_d_n4;
        locals.var_w_res_dn5 = assign53900_e82521_d_n5;
        locals.var_w_res_dn6 = assign53900_e82521_d_n6;
        locals.var_w_res_dn7 = assign53900_e82521_d_n7;
        locals.var_w_res_dn8 = assign53900_e82521_d_n8;
        locals.var_w_res_dn9 = assign53900_e82521_d_n9;
        locals.var_w_res_dn10 = assign53900_e82521_d_n10;
        locals.var_w_res_dn11 = assign53900_e82521_d_n11;
        locals.var_w_res_dn14 = assign53900_e82521_d_n14;

        let (assign53910_e82538, assign53910_e82538_d_n0, assign53910_e82538_d_n2, assign53910_e82538_d_n4, assign53910_e82538_d_n5, assign53910_e82538_d_n6, assign53910_e82538_d_n7, assign53910_e82538_d_n8, assign53910_e82538_d_n9, assign53910_e82538_d_n10, assign53910_e82538_d_n11, assign53910_e82538_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_guard1359 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign53910_e82538;
        locals.var_t0_dn0 = assign53910_e82538_d_n0;
        locals.var_t0_dn2 = assign53910_e82538_d_n2;
        locals.var_t0_dn4 = assign53910_e82538_d_n4;
        locals.var_t0_dn5 = assign53910_e82538_d_n5;
        locals.var_t0_dn6 = assign53910_e82538_d_n6;
        locals.var_t0_dn7 = assign53910_e82538_d_n7;
        locals.var_t0_dn8 = assign53910_e82538_d_n8;
        locals.var_t0_dn9 = assign53910_e82538_d_n9;
        locals.var_t0_dn10 = assign53910_e82538_d_n10;
        locals.var_t0_dn11 = assign53910_e82538_d_n11;
        locals.var_t0_dn14 = assign53910_e82538_d_n14;

        let assign53920_e82541: f64 = if 1.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1365 = assign53920_e82541;

        let (assign53930_e82557, assign53930_e82557_d_n0, assign53930_e82557_d_n2, assign53930_e82557_d_n4, assign53930_e82557_d_n5, assign53930_e82557_d_n6, assign53930_e82557_d_n7, assign53930_e82557_d_n8, assign53930_e82557_d_n9, assign53930_e82557_d_n10, assign53930_e82557_d_n11, assign53930_e82557_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_guard1365 != 0.0)) {
        (locals.var_w_res, locals.var_w_res_dn0, locals.var_w_res_dn2, locals.var_w_res_dn4, locals.var_w_res_dn5, locals.var_w_res_dn6, locals.var_w_res_dn7, locals.var_w_res_dn8, locals.var_w_res_dn9, locals.var_w_res_dn10, locals.var_w_res_dn11, locals.var_w_res_dn14,)
    } else {
        (locals.var_w_res_leak, locals.var_w_res_leak_dn0, locals.var_w_res_leak_dn2, locals.var_w_res_leak_dn4, locals.var_w_res_leak_dn5, locals.var_w_res_leak_dn6, locals.var_w_res_leak_dn7, locals.var_w_res_leak_dn8, locals.var_w_res_leak_dn9, locals.var_w_res_leak_dn10, locals.var_w_res_leak_dn11, locals.var_w_res_leak_dn14,)
    }
};
        locals.var_w_res_leak = assign53930_e82557;
        locals.var_w_res_leak_dn0 = assign53930_e82557_d_n0;
        locals.var_w_res_leak_dn2 = assign53930_e82557_d_n2;
        locals.var_w_res_leak_dn4 = assign53930_e82557_d_n4;
        locals.var_w_res_leak_dn5 = assign53930_e82557_d_n5;
        locals.var_w_res_leak_dn6 = assign53930_e82557_d_n6;
        locals.var_w_res_leak_dn7 = assign53930_e82557_d_n7;
        locals.var_w_res_leak_dn8 = assign53930_e82557_d_n8;
        locals.var_w_res_leak_dn9 = assign53930_e82557_d_n9;
        locals.var_w_res_leak_dn10 = assign53930_e82557_d_n10;
        locals.var_w_res_leak_dn11 = assign53930_e82557_d_n11;
        locals.var_w_res_leak_dn14 = assign53930_e82557_d_n14;

        let assign53940_e82560: f64 = if 2.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1366 = assign53940_e82560;

        let (assign53950_e82578, assign53950_e82578_d_n0, assign53950_e82578_d_n2, assign53950_e82578_d_n4, assign53950_e82578_d_n5, assign53950_e82578_d_n6, assign53950_e82578_d_n7, assign53950_e82578_d_n8, assign53950_e82578_d_n9, assign53950_e82578_d_n10, assign53950_e82578_d_n11, assign53950_e82578_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_guard1366 != 0.0)) {
        let assign53950_e82576: f64 = (p.p399 * locals.var_vbsc__blk1121);
        (assign53950_e82576, (p.p399 * locals.var_vbsc__blk1121_dn0), (p.p399 * locals.var_vbsc__blk1121_dn2), (p.p399 * locals.var_vbsc__blk1121_dn4), (p.p399 * locals.var_vbsc__blk1121_dn5), (p.p399 * locals.var_vbsc__blk1121_dn6), (p.p399 * locals.var_vbsc__blk1121_dn7), (p.p399 * locals.var_vbsc__blk1121_dn8), (p.p399 * locals.var_vbsc__blk1121_dn9), (p.p399 * locals.var_vbsc__blk1121_dn10), (p.p399 * locals.var_vbsc__blk1121_dn11), (p.p399 * locals.var_vbsc__blk1121_dn14),)
    } else {
        (locals.var_depvbs, locals.var_depvbs_dn0, locals.var_depvbs_dn2, locals.var_depvbs_dn4, locals.var_depvbs_dn5, locals.var_depvbs_dn6, locals.var_depvbs_dn7, locals.var_depvbs_dn8, locals.var_depvbs_dn9, locals.var_depvbs_dn10, locals.var_depvbs_dn11, locals.var_depvbs_dn14,)
    }
};
        locals.var_depvbs = assign53950_e82578;
        locals.var_depvbs_dn0 = assign53950_e82578_d_n0;
        locals.var_depvbs_dn2 = assign53950_e82578_d_n2;
        locals.var_depvbs_dn4 = assign53950_e82578_d_n4;
        locals.var_depvbs_dn5 = assign53950_e82578_d_n5;
        locals.var_depvbs_dn6 = assign53950_e82578_d_n6;
        locals.var_depvbs_dn7 = assign53950_e82578_d_n7;
        locals.var_depvbs_dn8 = assign53950_e82578_d_n8;
        locals.var_depvbs_dn9 = assign53950_e82578_d_n9;
        locals.var_depvbs_dn10 = assign53950_e82578_d_n10;
        locals.var_depvbs_dn11 = assign53950_e82578_d_n11;
        locals.var_depvbs_dn14 = assign53950_e82578_d_n14;

        let (assign53960_e82596, assign53960_e82596_d_n0, assign53960_e82596_d_n2, assign53960_e82596_d_n4, assign53960_e82596_d_n5, assign53960_e82596_d_n6, assign53960_e82596_d_n7, assign53960_e82596_d_n8, assign53960_e82596_d_n9, assign53960_e82596_d_n10, assign53960_e82596_d_n11, assign53960_e82596_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_guard1366 != 0.0)) {
        let assign53960_e82594: f64 = (locals.var_depvbs - 1.0);
        (assign53960_e82594, locals.var_depvbs_dn0, locals.var_depvbs_dn2, locals.var_depvbs_dn4, locals.var_depvbs_dn5, locals.var_depvbs_dn6, locals.var_depvbs_dn7, locals.var_depvbs_dn8, locals.var_depvbs_dn9, locals.var_depvbs_dn10, locals.var_depvbs_dn11, locals.var_depvbs_dn14,)
    } else {
        (locals.var_ps0dep, locals.var_ps0dep_dn0, locals.var_ps0dep_dn2, locals.var_ps0dep_dn4, locals.var_ps0dep_dn5, locals.var_ps0dep_dn6, locals.var_ps0dep_dn7, locals.var_ps0dep_dn8, locals.var_ps0dep_dn9, locals.var_ps0dep_dn10, locals.var_ps0dep_dn11, locals.var_ps0dep_dn14,)
    }
};
        locals.var_ps0dep = assign53960_e82596;
        locals.var_ps0dep_dn0 = assign53960_e82596_d_n0;
        locals.var_ps0dep_dn2 = assign53960_e82596_d_n2;
        locals.var_ps0dep_dn4 = assign53960_e82596_d_n4;
        locals.var_ps0dep_dn5 = assign53960_e82596_d_n5;
        locals.var_ps0dep_dn6 = assign53960_e82596_d_n6;
        locals.var_ps0dep_dn7 = assign53960_e82596_d_n7;
        locals.var_ps0dep_dn8 = assign53960_e82596_d_n8;
        locals.var_ps0dep_dn9 = assign53960_e82596_d_n9;
        locals.var_ps0dep_dn10 = assign53960_e82596_d_n10;
        locals.var_ps0dep_dn11 = assign53960_e82596_d_n11;
        locals.var_ps0dep_dn14 = assign53960_e82596_d_n14;

        let (assign53970_e82612, assign53970_e82612_d_n0, assign53970_e82612_d_n2, assign53970_e82612_d_n4, assign53970_e82612_d_n5, assign53970_e82612_d_n6, assign53970_e82612_d_n7, assign53970_e82612_d_n8, assign53970_e82612_d_n9, assign53970_e82612_d_n10, assign53970_e82612_d_n11, assign53970_e82612_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_guard1366 != 0.0)) {
        (locals.var_vgp_leak, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vgp_ws, locals.var_vgp_ws_dn0, locals.var_vgp_ws_dn2, locals.var_vgp_ws_dn4, locals.var_vgp_ws_dn5, locals.var_vgp_ws_dn6, locals.var_vgp_ws_dn7, locals.var_vgp_ws_dn8, locals.var_vgp_ws_dn9, locals.var_vgp_ws_dn10, locals.var_vgp_ws_dn11, locals.var_vgp_ws_dn14,)
    }
};
        locals.var_vgp_ws = assign53970_e82612;
        locals.var_vgp_ws_dn0 = assign53970_e82612_d_n0;
        locals.var_vgp_ws_dn2 = assign53970_e82612_d_n2;
        locals.var_vgp_ws_dn4 = assign53970_e82612_d_n4;
        locals.var_vgp_ws_dn5 = assign53970_e82612_d_n5;
        locals.var_vgp_ws_dn6 = assign53970_e82612_d_n6;
        locals.var_vgp_ws_dn7 = assign53970_e82612_d_n7;
        locals.var_vgp_ws_dn8 = assign53970_e82612_d_n8;
        locals.var_vgp_ws_dn9 = assign53970_e82612_d_n9;
        locals.var_vgp_ws_dn10 = assign53970_e82612_d_n10;
        locals.var_vgp_ws_dn11 = assign53970_e82612_d_n11;
        locals.var_vgp_ws_dn14 = assign53970_e82612_d_n14;

        let (assign53980_e82628, assign53980_e82628_d_n0, assign53980_e82628_d_n2, assign53980_e82628_d_n4, assign53980_e82628_d_n5, assign53980_e82628_d_n6, assign53980_e82628_d_n7, assign53980_e82628_d_n8, assign53980_e82628_d_n9, assign53980_e82628_d_n10, assign53980_e82628_d_n11, assign53980_e82628_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_guard1366 != 0.0)) {
        (locals.var_vgp_leak, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vgp_res_raw, locals.var_vgp_res_raw_dn0, locals.var_vgp_res_raw_dn2, locals.var_vgp_res_raw_dn4, locals.var_vgp_res_raw_dn5, locals.var_vgp_res_raw_dn6, locals.var_vgp_res_raw_dn7, locals.var_vgp_res_raw_dn8, locals.var_vgp_res_raw_dn9, locals.var_vgp_res_raw_dn10, locals.var_vgp_res_raw_dn11, locals.var_vgp_res_raw_dn14,)
    }
};
        locals.var_vgp_res_raw = assign53980_e82628;
        locals.var_vgp_res_raw_dn0 = assign53980_e82628_d_n0;
        locals.var_vgp_res_raw_dn2 = assign53980_e82628_d_n2;
        locals.var_vgp_res_raw_dn4 = assign53980_e82628_d_n4;
        locals.var_vgp_res_raw_dn5 = assign53980_e82628_d_n5;
        locals.var_vgp_res_raw_dn6 = assign53980_e82628_d_n6;
        locals.var_vgp_res_raw_dn7 = assign53980_e82628_d_n7;
        locals.var_vgp_res_raw_dn8 = assign53980_e82628_d_n8;
        locals.var_vgp_res_raw_dn9 = assign53980_e82628_d_n9;
        locals.var_vgp_res_raw_dn10 = assign53980_e82628_d_n10;
        locals.var_vgp_res_raw_dn11 = assign53980_e82628_d_n11;
        locals.var_vgp_res_raw_dn14 = assign53980_e82628_d_n14;

        let (assign53990_e82649, assign53990_e82649_d_n0, assign53990_e82649_d_n2, assign53990_e82649_d_n4, assign53990_e82649_d_n5, assign53990_e82649_d_n6, assign53990_e82649_d_n7, assign53990_e82649_d_n8, assign53990_e82649_d_n9, assign53990_e82649_d_n10, assign53990_e82649_d_n11, assign53990_e82649_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_guard1366 == 0.0)) {
        let assign53990_e82645: f64 = (p.p399 * locals.var_vbsc__blk1121);
        let assign53990_e82647: f64 = (assign53990_e82645 - 0.1);
        (assign53990_e82647, (p.p399 * locals.var_vbsc__blk1121_dn0), (p.p399 * locals.var_vbsc__blk1121_dn2), (p.p399 * locals.var_vbsc__blk1121_dn4), (p.p399 * locals.var_vbsc__blk1121_dn5), (p.p399 * locals.var_vbsc__blk1121_dn6), (p.p399 * locals.var_vbsc__blk1121_dn7), (p.p399 * locals.var_vbsc__blk1121_dn8), (p.p399 * locals.var_vbsc__blk1121_dn9), (p.p399 * locals.var_vbsc__blk1121_dn10), (p.p399 * locals.var_vbsc__blk1121_dn11), (p.p399 * locals.var_vbsc__blk1121_dn14),)
    } else {
        (locals.var_depvbs, locals.var_depvbs_dn0, locals.var_depvbs_dn2, locals.var_depvbs_dn4, locals.var_depvbs_dn5, locals.var_depvbs_dn6, locals.var_depvbs_dn7, locals.var_depvbs_dn8, locals.var_depvbs_dn9, locals.var_depvbs_dn10, locals.var_depvbs_dn11, locals.var_depvbs_dn14,)
    }
};
        locals.var_depvbs = assign53990_e82649;
        locals.var_depvbs_dn0 = assign53990_e82649_d_n0;
        locals.var_depvbs_dn2 = assign53990_e82649_d_n2;
        locals.var_depvbs_dn4 = assign53990_e82649_d_n4;
        locals.var_depvbs_dn5 = assign53990_e82649_d_n5;
        locals.var_depvbs_dn6 = assign53990_e82649_d_n6;
        locals.var_depvbs_dn7 = assign53990_e82649_d_n7;
        locals.var_depvbs_dn8 = assign53990_e82649_d_n8;
        locals.var_depvbs_dn9 = assign53990_e82649_d_n9;
        locals.var_depvbs_dn10 = assign53990_e82649_d_n10;
        locals.var_depvbs_dn11 = assign53990_e82649_d_n11;
        locals.var_depvbs_dn14 = assign53990_e82649_d_n14;

        let (assign54000_e82666, assign54000_e82666_d_n0, assign54000_e82666_d_n2, assign54000_e82666_d_n4, assign54000_e82666_d_n5, assign54000_e82666_d_n6, assign54000_e82666_d_n7, assign54000_e82666_d_n8, assign54000_e82666_d_n9, assign54000_e82666_d_n10, assign54000_e82666_d_n11, assign54000_e82666_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_guard1366 == 0.0)) {
        (locals.var_ps0, locals.var_ps0_dn0, locals.var_ps0_dn2, locals.var_ps0_dn4, locals.var_ps0_dn5, locals.var_ps0_dn6, locals.var_ps0_dn7, locals.var_ps0_dn8, locals.var_ps0_dn9, locals.var_ps0_dn10, locals.var_ps0_dn11, locals.var_ps0_dn14,)
    } else {
        (locals.var_ps0dep, locals.var_ps0dep_dn0, locals.var_ps0dep_dn2, locals.var_ps0dep_dn4, locals.var_ps0dep_dn5, locals.var_ps0dep_dn6, locals.var_ps0dep_dn7, locals.var_ps0dep_dn8, locals.var_ps0dep_dn9, locals.var_ps0dep_dn10, locals.var_ps0dep_dn11, locals.var_ps0dep_dn14,)
    }
};
        locals.var_ps0dep = assign54000_e82666;
        locals.var_ps0dep_dn0 = assign54000_e82666_d_n0;
        locals.var_ps0dep_dn2 = assign54000_e82666_d_n2;
        locals.var_ps0dep_dn4 = assign54000_e82666_d_n4;
        locals.var_ps0dep_dn5 = assign54000_e82666_d_n5;
        locals.var_ps0dep_dn6 = assign54000_e82666_d_n6;
        locals.var_ps0dep_dn7 = assign54000_e82666_d_n7;
        locals.var_ps0dep_dn8 = assign54000_e82666_d_n8;
        locals.var_ps0dep_dn9 = assign54000_e82666_d_n9;
        locals.var_ps0dep_dn10 = assign54000_e82666_d_n10;
        locals.var_ps0dep_dn11 = assign54000_e82666_d_n11;
        locals.var_ps0dep_dn14 = assign54000_e82666_d_n14;

        let (assign54010_e82683, assign54010_e82683_d_n0, assign54010_e82683_d_n2, assign54010_e82683_d_n4, assign54010_e82683_d_n5, assign54010_e82683_d_n6, assign54010_e82683_d_n7, assign54010_e82683_d_n8, assign54010_e82683_d_n9, assign54010_e82683_d_n10, assign54010_e82683_d_n11, assign54010_e82683_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_guard1366 == 0.0)) {
        (locals.var_vgp_res__blk1149, locals.var_vgp_res__blk1149_dn0, locals.var_vgp_res__blk1149_dn2, locals.var_vgp_res__blk1149_dn4, locals.var_vgp_res__blk1149_dn5, locals.var_vgp_res__blk1149_dn6, locals.var_vgp_res__blk1149_dn7, locals.var_vgp_res__blk1149_dn8, locals.var_vgp_res__blk1149_dn9, locals.var_vgp_res__blk1149_dn10, locals.var_vgp_res__blk1149_dn11, locals.var_vgp_res__blk1149_dn14,)
    } else {
        (locals.var_vgp_ws, locals.var_vgp_ws_dn0, locals.var_vgp_ws_dn2, locals.var_vgp_ws_dn4, locals.var_vgp_ws_dn5, locals.var_vgp_ws_dn6, locals.var_vgp_ws_dn7, locals.var_vgp_ws_dn8, locals.var_vgp_ws_dn9, locals.var_vgp_ws_dn10, locals.var_vgp_ws_dn11, locals.var_vgp_ws_dn14,)
    }
};
        locals.var_vgp_ws = assign54010_e82683;
        locals.var_vgp_ws_dn0 = assign54010_e82683_d_n0;
        locals.var_vgp_ws_dn2 = assign54010_e82683_d_n2;
        locals.var_vgp_ws_dn4 = assign54010_e82683_d_n4;
        locals.var_vgp_ws_dn5 = assign54010_e82683_d_n5;
        locals.var_vgp_ws_dn6 = assign54010_e82683_d_n6;
        locals.var_vgp_ws_dn7 = assign54010_e82683_d_n7;
        locals.var_vgp_ws_dn8 = assign54010_e82683_d_n8;
        locals.var_vgp_ws_dn9 = assign54010_e82683_d_n9;
        locals.var_vgp_ws_dn10 = assign54010_e82683_d_n10;
        locals.var_vgp_ws_dn11 = assign54010_e82683_d_n11;
        locals.var_vgp_ws_dn14 = assign54010_e82683_d_n14;

        let (assign54020_e82700, assign54020_e82700_d_n0, assign54020_e82700_d_n2, assign54020_e82700_d_n4, assign54020_e82700_d_n5, assign54020_e82700_d_n6, assign54020_e82700_d_n7, assign54020_e82700_d_n8, assign54020_e82700_d_n9, assign54020_e82700_d_n10, assign54020_e82700_d_n11, assign54020_e82700_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_guard1366 == 0.0)) {
        (locals.var_vgp_res__blk1149, locals.var_vgp_res__blk1149_dn0, locals.var_vgp_res__blk1149_dn2, locals.var_vgp_res__blk1149_dn4, locals.var_vgp_res__blk1149_dn5, locals.var_vgp_res__blk1149_dn6, locals.var_vgp_res__blk1149_dn7, locals.var_vgp_res__blk1149_dn8, locals.var_vgp_res__blk1149_dn9, locals.var_vgp_res__blk1149_dn10, locals.var_vgp_res__blk1149_dn11, locals.var_vgp_res__blk1149_dn14,)
    } else {
        (locals.var_vgp_res_raw, locals.var_vgp_res_raw_dn0, locals.var_vgp_res_raw_dn2, locals.var_vgp_res_raw_dn4, locals.var_vgp_res_raw_dn5, locals.var_vgp_res_raw_dn6, locals.var_vgp_res_raw_dn7, locals.var_vgp_res_raw_dn8, locals.var_vgp_res_raw_dn9, locals.var_vgp_res_raw_dn10, locals.var_vgp_res_raw_dn11, locals.var_vgp_res_raw_dn14,)
    }
};
        locals.var_vgp_res_raw = assign54020_e82700;
        locals.var_vgp_res_raw_dn0 = assign54020_e82700_d_n0;
        locals.var_vgp_res_raw_dn2 = assign54020_e82700_d_n2;
        locals.var_vgp_res_raw_dn4 = assign54020_e82700_d_n4;
        locals.var_vgp_res_raw_dn5 = assign54020_e82700_d_n5;
        locals.var_vgp_res_raw_dn6 = assign54020_e82700_d_n6;
        locals.var_vgp_res_raw_dn7 = assign54020_e82700_d_n7;
        locals.var_vgp_res_raw_dn8 = assign54020_e82700_d_n8;
        locals.var_vgp_res_raw_dn9 = assign54020_e82700_d_n9;
        locals.var_vgp_res_raw_dn10 = assign54020_e82700_d_n10;
        locals.var_vgp_res_raw_dn11 = assign54020_e82700_d_n11;
        locals.var_vgp_res_raw_dn14 = assign54020_e82700_d_n14;

    }

    pub(super) fn stamp_transient_block_185(
        locals: &mut StampLocals,
    ) {
        let (assign54030_e82714,) = {
    if (((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) {
        (0.0,)
    } else {
        (locals.var_flg_conv,)
    }
};
        locals.var_flg_conv = assign54030_e82714;

        let (assign54040_e82728,) = {
    if (((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) {
        (1.0,)
    } else {
        (locals.var_lp_s0,)
    }
};
        locals.var_lp_s0 = assign54040_e82728;

        let mut assign54050_loop_guard: usize = 0;
        while {
            let assign54050_cond_e82743: f64 = if ((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_lp_s0 <= 150.0)) { 1.0 } else { 0.0 };
            assign54050_cond_e82743 != 0.0
        } {
            assign54050_loop_guard += 1;
            assert!(assign54050_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign54050_body0_e82759, assign54050_body0_e82759_d_n0, assign54050_body0_e82759_d_n2, assign54050_body0_e82759_d_n4, assign54050_body0_e82759_d_n5, assign54050_body0_e82759_d_n6, assign54050_body0_e82759_d_n7, assign54050_body0_e82759_d_n8, assign54050_body0_e82759_d_n9, assign54050_body0_e82759_d_n10, assign54050_body0_e82759_d_n11, assign54050_body0_e82759_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) {
        let assign54050_body0_e82757: f64 = (locals.var_beta * locals.var_ps0dep);
        (assign54050_body0_e82757, ((locals.var_beta_dn0 * locals.var_ps0dep) + (locals.var_beta * locals.var_ps0dep_dn0)), ((locals.var_beta_dn2 * locals.var_ps0dep) + (locals.var_beta * locals.var_ps0dep_dn2)), ((locals.var_beta_dn4 * locals.var_ps0dep) + (locals.var_beta * locals.var_ps0dep_dn4)), ((locals.var_beta_dn5 * locals.var_ps0dep) + (locals.var_beta * locals.var_ps0dep_dn5)), ((locals.var_beta_dn6 * locals.var_ps0dep) + (locals.var_beta * locals.var_ps0dep_dn6)), ((locals.var_beta_dn7 * locals.var_ps0dep) + (locals.var_beta * locals.var_ps0dep_dn7)), ((locals.var_beta_dn8 * locals.var_ps0dep) + (locals.var_beta * locals.var_ps0dep_dn8)), ((locals.var_beta_dn9 * locals.var_ps0dep) + (locals.var_beta * locals.var_ps0dep_dn9)), ((locals.var_beta_dn10 * locals.var_ps0dep) + (locals.var_beta * locals.var_ps0dep_dn10)), ((locals.var_beta_dn11 * locals.var_ps0dep) + (locals.var_beta * locals.var_ps0dep_dn11)), ((locals.var_beta_dn14 * locals.var_ps0dep) + (locals.var_beta * locals.var_ps0dep_dn14)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
            locals.var_t1 = assign54050_body0_e82759;
            locals.var_t1_dn0 = assign54050_body0_e82759_d_n0;
            locals.var_t1_dn2 = assign54050_body0_e82759_d_n2;
            locals.var_t1_dn4 = assign54050_body0_e82759_d_n4;
            locals.var_t1_dn5 = assign54050_body0_e82759_d_n5;
            locals.var_t1_dn6 = assign54050_body0_e82759_d_n6;
            locals.var_t1_dn7 = assign54050_body0_e82759_d_n7;
            locals.var_t1_dn8 = assign54050_body0_e82759_d_n8;
            locals.var_t1_dn9 = assign54050_body0_e82759_d_n9;
            locals.var_t1_dn10 = assign54050_body0_e82759_d_n10;
            locals.var_t1_dn11 = assign54050_body0_e82759_d_n11;
            locals.var_t1_dn14 = assign54050_body0_e82759_d_n14;
            let (assign54050_body1_e82774, assign54050_body1_e82774_d_n0, assign54050_body1_e82774_d_n2, assign54050_body1_e82774_d_n4, assign54050_body1_e82774_d_n5, assign54050_body1_e82774_d_n6, assign54050_body1_e82774_d_n7, assign54050_body1_e82774_d_n8, assign54050_body1_e82774_d_n9, assign54050_body1_e82774_d_n10, assign54050_body1_e82774_d_n11, assign54050_body1_e82774_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) {
        let assign54050_body1_e82772: f64 = (locals.var_t1).exp();
        (assign54050_body1_e82772, (assign54050_body1_e82772 * locals.var_t1_dn0), (assign54050_body1_e82772 * locals.var_t1_dn2), (assign54050_body1_e82772 * locals.var_t1_dn4), (assign54050_body1_e82772 * locals.var_t1_dn5), (assign54050_body1_e82772 * locals.var_t1_dn6), (assign54050_body1_e82772 * locals.var_t1_dn7), (assign54050_body1_e82772 * locals.var_t1_dn8), (assign54050_body1_e82772 * locals.var_t1_dn9), (assign54050_body1_e82772 * locals.var_t1_dn10), (assign54050_body1_e82772 * locals.var_t1_dn11), (assign54050_body1_e82772 * locals.var_t1_dn14),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
            locals.var_t2 = assign54050_body1_e82774;
            locals.var_t2_dn0 = assign54050_body1_e82774_d_n0;
            locals.var_t2_dn2 = assign54050_body1_e82774_d_n2;
            locals.var_t2_dn4 = assign54050_body1_e82774_d_n4;
            locals.var_t2_dn5 = assign54050_body1_e82774_d_n5;
            locals.var_t2_dn6 = assign54050_body1_e82774_d_n6;
            locals.var_t2_dn7 = assign54050_body1_e82774_d_n7;
            locals.var_t2_dn8 = assign54050_body1_e82774_d_n8;
            locals.var_t2_dn9 = assign54050_body1_e82774_d_n9;
            locals.var_t2_dn10 = assign54050_body1_e82774_d_n10;
            locals.var_t2_dn11 = assign54050_body1_e82774_d_n11;
            locals.var_t2_dn14 = assign54050_body1_e82774_d_n14;
            let assign54050_body2_e82777: f64 = if locals.var_ps0dep >= 0.0 { 1.0 } else { 0.0 };
            locals.var_guard1367 = assign54050_body2_e82777;
            let (assign54050_body3_e82803, assign54050_body3_e82803_d_n0, assign54050_body3_e82803_d_n2, assign54050_body3_e82803_d_n4, assign54050_body3_e82803_d_n5, assign54050_body3_e82803_d_n6, assign54050_body3_e82803_d_n7, assign54050_body3_e82803_d_n8, assign54050_body3_e82803_d_n9, assign54050_body3_e82803_d_n10, assign54050_body3_e82803_d_n11, assign54050_body3_e82803_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_guard1367 != 0.0)) {
        let assign54050_body3_e82792: f64 = (-locals.var_cnst0);
        let assign54050_body3_e82795: f64 = (locals.var_t2 - 1.0);
        let assign54050_body3_e82797: f64 = (assign54050_body3_e82795 - locals.var_t1);
        let assign54050_body3_e82799: f64 = (assign54050_body3_e82797 + 1e-15);
        let assign54050_body3_e82800: f64 = (assign54050_body3_e82799).sqrt();
        let assign54050_body3_e82801: f64 = (assign54050_body3_e82792 * assign54050_body3_e82800);
        (assign54050_body3_e82801, (((-locals.var_cnst0_dn0) * assign54050_body3_e82800) + (assign54050_body3_e82792 * ((locals.var_t2_dn0 - locals.var_t1_dn0) / (2.0 * assign54050_body3_e82800)))), (((-locals.var_cnst0_dn2) * assign54050_body3_e82800) + (assign54050_body3_e82792 * ((locals.var_t2_dn2 - locals.var_t1_dn2) / (2.0 * assign54050_body3_e82800)))), (((-locals.var_cnst0_dn4) * assign54050_body3_e82800) + (assign54050_body3_e82792 * ((locals.var_t2_dn4 - locals.var_t1_dn4) / (2.0 * assign54050_body3_e82800)))), (((-locals.var_cnst0_dn5) * assign54050_body3_e82800) + (assign54050_body3_e82792 * ((locals.var_t2_dn5 - locals.var_t1_dn5) / (2.0 * assign54050_body3_e82800)))), (((-locals.var_cnst0_dn6) * assign54050_body3_e82800) + (assign54050_body3_e82792 * ((locals.var_t2_dn6 - locals.var_t1_dn6) / (2.0 * assign54050_body3_e82800)))), (((-locals.var_cnst0_dn7) * assign54050_body3_e82800) + (assign54050_body3_e82792 * ((locals.var_t2_dn7 - locals.var_t1_dn7) / (2.0 * assign54050_body3_e82800)))), (((-locals.var_cnst0_dn8) * assign54050_body3_e82800) + (assign54050_body3_e82792 * ((locals.var_t2_dn8 - locals.var_t1_dn8) / (2.0 * assign54050_body3_e82800)))), (((-locals.var_cnst0_dn9) * assign54050_body3_e82800) + (assign54050_body3_e82792 * ((locals.var_t2_dn9 - locals.var_t1_dn9) / (2.0 * assign54050_body3_e82800)))), (((-locals.var_cnst0_dn10) * assign54050_body3_e82800) + (assign54050_body3_e82792 * ((locals.var_t2_dn10 - locals.var_t1_dn10) / (2.0 * assign54050_body3_e82800)))), (((-locals.var_cnst0_dn11) * assign54050_body3_e82800) + (assign54050_body3_e82792 * ((locals.var_t2_dn11 - locals.var_t1_dn11) / (2.0 * assign54050_body3_e82800)))), (((-locals.var_cnst0_dn14) * assign54050_body3_e82800) + (assign54050_body3_e82792 * ((locals.var_t2_dn14 - locals.var_t1_dn14) / (2.0 * assign54050_body3_e82800)))),)
    } else {
        (locals.var_q_s0__blk1326, locals.var_q_s0__blk1326_dn0, locals.var_q_s0__blk1326_dn2, locals.var_q_s0__blk1326_dn4, locals.var_q_s0__blk1326_dn5, locals.var_q_s0__blk1326_dn6, locals.var_q_s0__blk1326_dn7, locals.var_q_s0__blk1326_dn8, locals.var_q_s0__blk1326_dn9, locals.var_q_s0__blk1326_dn10, locals.var_q_s0__blk1326_dn11, locals.var_q_s0__blk1326_dn14,)
    }
};
            locals.var_q_s0__blk1326 = assign54050_body3_e82803;
            locals.var_q_s0__blk1326_dn0 = assign54050_body3_e82803_d_n0;
            locals.var_q_s0__blk1326_dn2 = assign54050_body3_e82803_d_n2;
            locals.var_q_s0__blk1326_dn4 = assign54050_body3_e82803_d_n4;
            locals.var_q_s0__blk1326_dn5 = assign54050_body3_e82803_d_n5;
            locals.var_q_s0__blk1326_dn6 = assign54050_body3_e82803_d_n6;
            locals.var_q_s0__blk1326_dn7 = assign54050_body3_e82803_d_n7;
            locals.var_q_s0__blk1326_dn8 = assign54050_body3_e82803_d_n8;
            locals.var_q_s0__blk1326_dn9 = assign54050_body3_e82803_d_n9;
            locals.var_q_s0__blk1326_dn10 = assign54050_body3_e82803_d_n10;
            locals.var_q_s0__blk1326_dn11 = assign54050_body3_e82803_d_n11;
            locals.var_q_s0__blk1326_dn14 = assign54050_body3_e82803_d_n14;
            let (assign54050_body4_e82831, assign54050_body4_e82831_d_n0, assign54050_body4_e82831_d_n2, assign54050_body4_e82831_d_n4, assign54050_body4_e82831_d_n5, assign54050_body4_e82831_d_n6, assign54050_body4_e82831_d_n7, assign54050_body4_e82831_d_n8, assign54050_body4_e82831_d_n9, assign54050_body4_e82831_d_n10, assign54050_body4_e82831_d_n11, assign54050_body4_e82831_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_guard1367 != 0.0)) {
        let assign54050_body4_e82819: f64 = (0.5 * locals.var_cnst0);
        let assign54050_body4_e82821: f64 = (assign54050_body4_e82819 * locals.var_cnst0);
        let assign54050_body4_e82823: f64 = (assign54050_body4_e82821 / locals.var_q_s0__blk1326);
        let assign54050_body4_e82826: f64 = (locals.var_beta * locals.var_t2);
        let assign54050_body4_e82828: f64 = (assign54050_body4_e82826 - locals.var_beta);
        let assign54050_body4_e82829: f64 = (assign54050_body4_e82823 * assign54050_body4_e82828);
        (assign54050_body4_e82829, ((((((((0.5 * locals.var_cnst0_dn0) * locals.var_cnst0) + (assign54050_body4_e82819 * locals.var_cnst0_dn0)) * locals.var_q_s0__blk1326) - (assign54050_body4_e82821 * locals.var_q_s0__blk1326_dn0)) / (locals.var_q_s0__blk1326 * locals.var_q_s0__blk1326)) * assign54050_body4_e82828) + (assign54050_body4_e82823 * (((locals.var_beta_dn0 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn0)) - locals.var_beta_dn0))), ((((((((0.5 * locals.var_cnst0_dn2) * locals.var_cnst0) + (assign54050_body4_e82819 * locals.var_cnst0_dn2)) * locals.var_q_s0__blk1326) - (assign54050_body4_e82821 * locals.var_q_s0__blk1326_dn2)) / (locals.var_q_s0__blk1326 * locals.var_q_s0__blk1326)) * assign54050_body4_e82828) + (assign54050_body4_e82823 * (((locals.var_beta_dn2 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn2)) - locals.var_beta_dn2))), ((((((((0.5 * locals.var_cnst0_dn4) * locals.var_cnst0) + (assign54050_body4_e82819 * locals.var_cnst0_dn4)) * locals.var_q_s0__blk1326) - (assign54050_body4_e82821 * locals.var_q_s0__blk1326_dn4)) / (locals.var_q_s0__blk1326 * locals.var_q_s0__blk1326)) * assign54050_body4_e82828) + (assign54050_body4_e82823 * (((locals.var_beta_dn4 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn4)) - locals.var_beta_dn4))), ((((((((0.5 * locals.var_cnst0_dn5) * locals.var_cnst0) + (assign54050_body4_e82819 * locals.var_cnst0_dn5)) * locals.var_q_s0__blk1326) - (assign54050_body4_e82821 * locals.var_q_s0__blk1326_dn5)) / (locals.var_q_s0__blk1326 * locals.var_q_s0__blk1326)) * assign54050_body4_e82828) + (assign54050_body4_e82823 * (((locals.var_beta_dn5 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn5)) - locals.var_beta_dn5))), ((((((((0.5 * locals.var_cnst0_dn6) * locals.var_cnst0) + (assign54050_body4_e82819 * locals.var_cnst0_dn6)) * locals.var_q_s0__blk1326) - (assign54050_body4_e82821 * locals.var_q_s0__blk1326_dn6)) / (locals.var_q_s0__blk1326 * locals.var_q_s0__blk1326)) * assign54050_body4_e82828) + (assign54050_body4_e82823 * (((locals.var_beta_dn6 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn6)) - locals.var_beta_dn6))), ((((((((0.5 * locals.var_cnst0_dn7) * locals.var_cnst0) + (assign54050_body4_e82819 * locals.var_cnst0_dn7)) * locals.var_q_s0__blk1326) - (assign54050_body4_e82821 * locals.var_q_s0__blk1326_dn7)) / (locals.var_q_s0__blk1326 * locals.var_q_s0__blk1326)) * assign54050_body4_e82828) + (assign54050_body4_e82823 * (((locals.var_beta_dn7 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn7)) - locals.var_beta_dn7))), ((((((((0.5 * locals.var_cnst0_dn8) * locals.var_cnst0) + (assign54050_body4_e82819 * locals.var_cnst0_dn8)) * locals.var_q_s0__blk1326) - (assign54050_body4_e82821 * locals.var_q_s0__blk1326_dn8)) / (locals.var_q_s0__blk1326 * locals.var_q_s0__blk1326)) * assign54050_body4_e82828) + (assign54050_body4_e82823 * (((locals.var_beta_dn8 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn8)) - locals.var_beta_dn8))), ((((((((0.5 * locals.var_cnst0_dn9) * locals.var_cnst0) + (assign54050_body4_e82819 * locals.var_cnst0_dn9)) * locals.var_q_s0__blk1326) - (assign54050_body4_e82821 * locals.var_q_s0__blk1326_dn9)) / (locals.var_q_s0__blk1326 * locals.var_q_s0__blk1326)) * assign54050_body4_e82828) + (assign54050_body4_e82823 * (((locals.var_beta_dn9 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn9)) - locals.var_beta_dn9))), ((((((((0.5 * locals.var_cnst0_dn10) * locals.var_cnst0) + (assign54050_body4_e82819 * locals.var_cnst0_dn10)) * locals.var_q_s0__blk1326) - (assign54050_body4_e82821 * locals.var_q_s0__blk1326_dn10)) / (locals.var_q_s0__blk1326 * locals.var_q_s0__blk1326)) * assign54050_body4_e82828) + (assign54050_body4_e82823 * (((locals.var_beta_dn10 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn10)) - locals.var_beta_dn10))), ((((((((0.5 * locals.var_cnst0_dn11) * locals.var_cnst0) + (assign54050_body4_e82819 * locals.var_cnst0_dn11)) * locals.var_q_s0__blk1326) - (assign54050_body4_e82821 * locals.var_q_s0__blk1326_dn11)) / (locals.var_q_s0__blk1326 * locals.var_q_s0__blk1326)) * assign54050_body4_e82828) + (assign54050_body4_e82823 * (((locals.var_beta_dn11 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn11)) - locals.var_beta_dn11))), ((((((((0.5 * locals.var_cnst0_dn14) * locals.var_cnst0) + (assign54050_body4_e82819 * locals.var_cnst0_dn14)) * locals.var_q_s0__blk1326) - (assign54050_body4_e82821 * locals.var_q_s0__blk1326_dn14)) / (locals.var_q_s0__blk1326 * locals.var_q_s0__blk1326)) * assign54050_body4_e82828) + (assign54050_body4_e82823 * (((locals.var_beta_dn14 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn14)) - locals.var_beta_dn14))),)
    } else {
        (locals.var_q_s0_dps__blk1129, locals.var_q_s0_dps__blk1129_dn0, locals.var_q_s0_dps__blk1129_dn2, locals.var_q_s0_dps__blk1129_dn4, locals.var_q_s0_dps__blk1129_dn5, locals.var_q_s0_dps__blk1129_dn6, locals.var_q_s0_dps__blk1129_dn7, locals.var_q_s0_dps__blk1129_dn8, locals.var_q_s0_dps__blk1129_dn9, locals.var_q_s0_dps__blk1129_dn10, locals.var_q_s0_dps__blk1129_dn11, locals.var_q_s0_dps__blk1129_dn14,)
    }
};
            locals.var_q_s0_dps__blk1129 = assign54050_body4_e82831;
            locals.var_q_s0_dps__blk1129_dn0 = assign54050_body4_e82831_d_n0;
            locals.var_q_s0_dps__blk1129_dn2 = assign54050_body4_e82831_d_n2;
            locals.var_q_s0_dps__blk1129_dn4 = assign54050_body4_e82831_d_n4;
            locals.var_q_s0_dps__blk1129_dn5 = assign54050_body4_e82831_d_n5;
            locals.var_q_s0_dps__blk1129_dn6 = assign54050_body4_e82831_d_n6;
            locals.var_q_s0_dps__blk1129_dn7 = assign54050_body4_e82831_d_n7;
            locals.var_q_s0_dps__blk1129_dn8 = assign54050_body4_e82831_d_n8;
            locals.var_q_s0_dps__blk1129_dn9 = assign54050_body4_e82831_d_n9;
            locals.var_q_s0_dps__blk1129_dn10 = assign54050_body4_e82831_d_n10;
            locals.var_q_s0_dps__blk1129_dn11 = assign54050_body4_e82831_d_n11;
            locals.var_q_s0_dps__blk1129_dn14 = assign54050_body4_e82831_d_n14;
            let (assign54050_body5_e82854, assign54050_body5_e82854_d_n0, assign54050_body5_e82854_d_n2, assign54050_body5_e82854_d_n4, assign54050_body5_e82854_d_n5, assign54050_body5_e82854_d_n6, assign54050_body5_e82854_d_n7, assign54050_body5_e82854_d_n8, assign54050_body5_e82854_d_n9, assign54050_body5_e82854_d_n10, assign54050_body5_e82854_d_n11, assign54050_body5_e82854_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_guard1367 == 0.0)) {
        let assign54050_body5_e82847: f64 = (-locals.var_beta);
        let assign54050_body5_e82850: f64 = (locals.var_ps0dep - locals.var_depvbs);
        let assign54050_body5_e82851: f64 = (assign54050_body5_e82847 * assign54050_body5_e82850);
        let assign54050_body5_e82852: f64 = (assign54050_body5_e82851).exp();
        (assign54050_body5_e82852, (assign54050_body5_e82852 * (((-locals.var_beta_dn0) * assign54050_body5_e82850) + (assign54050_body5_e82847 * (locals.var_ps0dep_dn0 - locals.var_depvbs_dn0)))), (assign54050_body5_e82852 * (((-locals.var_beta_dn2) * assign54050_body5_e82850) + (assign54050_body5_e82847 * (locals.var_ps0dep_dn2 - locals.var_depvbs_dn2)))), (assign54050_body5_e82852 * (((-locals.var_beta_dn4) * assign54050_body5_e82850) + (assign54050_body5_e82847 * (locals.var_ps0dep_dn4 - locals.var_depvbs_dn4)))), (assign54050_body5_e82852 * (((-locals.var_beta_dn5) * assign54050_body5_e82850) + (assign54050_body5_e82847 * (locals.var_ps0dep_dn5 - locals.var_depvbs_dn5)))), (assign54050_body5_e82852 * (((-locals.var_beta_dn6) * assign54050_body5_e82850) + (assign54050_body5_e82847 * (locals.var_ps0dep_dn6 - locals.var_depvbs_dn6)))), (assign54050_body5_e82852 * (((-locals.var_beta_dn7) * assign54050_body5_e82850) + (assign54050_body5_e82847 * (locals.var_ps0dep_dn7 - locals.var_depvbs_dn7)))), (assign54050_body5_e82852 * (((-locals.var_beta_dn8) * assign54050_body5_e82850) + (assign54050_body5_e82847 * (locals.var_ps0dep_dn8 - locals.var_depvbs_dn8)))), (assign54050_body5_e82852 * (((-locals.var_beta_dn9) * assign54050_body5_e82850) + (assign54050_body5_e82847 * (locals.var_ps0dep_dn9 - locals.var_depvbs_dn9)))), (assign54050_body5_e82852 * (((-locals.var_beta_dn10) * assign54050_body5_e82850) + (assign54050_body5_e82847 * (locals.var_ps0dep_dn10 - locals.var_depvbs_dn10)))), (assign54050_body5_e82852 * (((-locals.var_beta_dn11) * assign54050_body5_e82850) + (assign54050_body5_e82847 * (locals.var_ps0dep_dn11 - locals.var_depvbs_dn11)))), (assign54050_body5_e82852 * (((-locals.var_beta_dn14) * assign54050_body5_e82850) + (assign54050_body5_e82847 * (locals.var_ps0dep_dn14 - locals.var_depvbs_dn14)))),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
            locals.var_t3 = assign54050_body5_e82854;
            locals.var_t3_dn0 = assign54050_body5_e82854_d_n0;
            locals.var_t3_dn2 = assign54050_body5_e82854_d_n2;
            locals.var_t3_dn4 = assign54050_body5_e82854_d_n4;
            locals.var_t3_dn5 = assign54050_body5_e82854_d_n5;
            locals.var_t3_dn6 = assign54050_body5_e82854_d_n6;
            locals.var_t3_dn7 = assign54050_body5_e82854_d_n7;
            locals.var_t3_dn8 = assign54050_body5_e82854_d_n8;
            locals.var_t3_dn9 = assign54050_body5_e82854_d_n9;
            locals.var_t3_dn10 = assign54050_body5_e82854_d_n10;
            locals.var_t3_dn11 = assign54050_body5_e82854_d_n11;
            locals.var_t3_dn14 = assign54050_body5_e82854_d_n14;
            let (assign54050_body6_e82874, assign54050_body6_e82874_d_n0, assign54050_body6_e82874_d_n2, assign54050_body6_e82874_d_n4, assign54050_body6_e82874_d_n5, assign54050_body6_e82874_d_n6, assign54050_body6_e82874_d_n7, assign54050_body6_e82874_d_n8, assign54050_body6_e82874_d_n9, assign54050_body6_e82874_d_n10, assign54050_body6_e82874_d_n11, assign54050_body6_e82874_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_guard1367 == 0.0)) {
        let assign54050_body6_e82871: f64 = (locals.var_beta * locals.var_depvbs);
        let assign54050_body6_e82872: f64 = (assign54050_body6_e82871).exp();
        (assign54050_body6_e82872, (assign54050_body6_e82872 * ((locals.var_beta_dn0 * locals.var_depvbs) + (locals.var_beta * locals.var_depvbs_dn0))), (assign54050_body6_e82872 * ((locals.var_beta_dn2 * locals.var_depvbs) + (locals.var_beta * locals.var_depvbs_dn2))), (assign54050_body6_e82872 * ((locals.var_beta_dn4 * locals.var_depvbs) + (locals.var_beta * locals.var_depvbs_dn4))), (assign54050_body6_e82872 * ((locals.var_beta_dn5 * locals.var_depvbs) + (locals.var_beta * locals.var_depvbs_dn5))), (assign54050_body6_e82872 * ((locals.var_beta_dn6 * locals.var_depvbs) + (locals.var_beta * locals.var_depvbs_dn6))), (assign54050_body6_e82872 * ((locals.var_beta_dn7 * locals.var_depvbs) + (locals.var_beta * locals.var_depvbs_dn7))), (assign54050_body6_e82872 * ((locals.var_beta_dn8 * locals.var_depvbs) + (locals.var_beta * locals.var_depvbs_dn8))), (assign54050_body6_e82872 * ((locals.var_beta_dn9 * locals.var_depvbs) + (locals.var_beta * locals.var_depvbs_dn9))), (assign54050_body6_e82872 * ((locals.var_beta_dn10 * locals.var_depvbs) + (locals.var_beta * locals.var_depvbs_dn10))), (assign54050_body6_e82872 * ((locals.var_beta_dn11 * locals.var_depvbs) + (locals.var_beta * locals.var_depvbs_dn11))), (assign54050_body6_e82872 * ((locals.var_beta_dn14 * locals.var_depvbs) + (locals.var_beta * locals.var_depvbs_dn14))),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
            locals.var_t4 = assign54050_body6_e82874;
            locals.var_t4_dn0 = assign54050_body6_e82874_d_n0;
            locals.var_t4_dn2 = assign54050_body6_e82874_d_n2;
            locals.var_t4_dn4 = assign54050_body6_e82874_d_n4;
            locals.var_t4_dn5 = assign54050_body6_e82874_d_n5;
            locals.var_t4_dn6 = assign54050_body6_e82874_d_n6;
            locals.var_t4_dn7 = assign54050_body6_e82874_d_n7;
            locals.var_t4_dn8 = assign54050_body6_e82874_d_n8;
            locals.var_t4_dn9 = assign54050_body6_e82874_d_n9;
            locals.var_t4_dn10 = assign54050_body6_e82874_d_n10;
            locals.var_t4_dn11 = assign54050_body6_e82874_d_n11;
            locals.var_t4_dn14 = assign54050_body6_e82874_d_n14;
            let (assign54050_body7_e82906, assign54050_body7_e82906_d_n0, assign54050_body7_e82906_d_n2, assign54050_body7_e82906_d_n4, assign54050_body7_e82906_d_n5, assign54050_body7_e82906_d_n6, assign54050_body7_e82906_d_n7, assign54050_body7_e82906_d_n8, assign54050_body7_e82906_d_n9, assign54050_body7_e82906_d_n10, assign54050_body7_e82906_d_n11, assign54050_body7_e82906_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_guard1367 == 0.0)) {
        let assign54050_body7_e82892: f64 = (locals.var_t2 - 1.0);
        let assign54050_body7_e82894: f64 = (assign54050_body7_e82892 - locals.var_t1);
        let assign54050_body7_e82898: f64 = (locals.var_t3 - locals.var_t4);
        let assign54050_body7_e82899: f64 = (locals.var_cnst1 * assign54050_body7_e82898);
        let assign54050_body7_e82900: f64 = (assign54050_body7_e82894 + assign54050_body7_e82899);
        let assign54050_body7_e82902: f64 = (assign54050_body7_e82900 + 1e-15);
        let assign54050_body7_e82903: f64 = (assign54050_body7_e82902).sqrt();
        let assign54050_body7_e82904: f64 = (locals.var_cnst0 * assign54050_body7_e82903);
        (assign54050_body7_e82904, ((locals.var_cnst0_dn0 * assign54050_body7_e82903) + (locals.var_cnst0 * (((locals.var_t2_dn0 - locals.var_t1_dn0) + ((locals.var_cnst1_dn0 * assign54050_body7_e82898) + (locals.var_cnst1 * (locals.var_t3_dn0 - locals.var_t4_dn0)))) / (2.0 * assign54050_body7_e82903)))), ((locals.var_cnst0_dn2 * assign54050_body7_e82903) + (locals.var_cnst0 * (((locals.var_t2_dn2 - locals.var_t1_dn2) + ((locals.var_cnst1_dn2 * assign54050_body7_e82898) + (locals.var_cnst1 * (locals.var_t3_dn2 - locals.var_t4_dn2)))) / (2.0 * assign54050_body7_e82903)))), ((locals.var_cnst0_dn4 * assign54050_body7_e82903) + (locals.var_cnst0 * (((locals.var_t2_dn4 - locals.var_t1_dn4) + ((locals.var_cnst1_dn4 * assign54050_body7_e82898) + (locals.var_cnst1 * (locals.var_t3_dn4 - locals.var_t4_dn4)))) / (2.0 * assign54050_body7_e82903)))), ((locals.var_cnst0_dn5 * assign54050_body7_e82903) + (locals.var_cnst0 * (((locals.var_t2_dn5 - locals.var_t1_dn5) + ((locals.var_cnst1_dn5 * assign54050_body7_e82898) + (locals.var_cnst1 * (locals.var_t3_dn5 - locals.var_t4_dn5)))) / (2.0 * assign54050_body7_e82903)))), ((locals.var_cnst0_dn6 * assign54050_body7_e82903) + (locals.var_cnst0 * (((locals.var_t2_dn6 - locals.var_t1_dn6) + ((locals.var_cnst1_dn6 * assign54050_body7_e82898) + (locals.var_cnst1 * (locals.var_t3_dn6 - locals.var_t4_dn6)))) / (2.0 * assign54050_body7_e82903)))), ((locals.var_cnst0_dn7 * assign54050_body7_e82903) + (locals.var_cnst0 * (((locals.var_t2_dn7 - locals.var_t1_dn7) + ((locals.var_cnst1_dn7 * assign54050_body7_e82898) + (locals.var_cnst1 * (locals.var_t3_dn7 - locals.var_t4_dn7)))) / (2.0 * assign54050_body7_e82903)))), ((locals.var_cnst0_dn8 * assign54050_body7_e82903) + (locals.var_cnst0 * (((locals.var_t2_dn8 - locals.var_t1_dn8) + ((locals.var_cnst1_dn8 * assign54050_body7_e82898) + (locals.var_cnst1 * (locals.var_t3_dn8 - locals.var_t4_dn8)))) / (2.0 * assign54050_body7_e82903)))), ((locals.var_cnst0_dn9 * assign54050_body7_e82903) + (locals.var_cnst0 * (((locals.var_t2_dn9 - locals.var_t1_dn9) + ((locals.var_cnst1_dn9 * assign54050_body7_e82898) + (locals.var_cnst1 * (locals.var_t3_dn9 - locals.var_t4_dn9)))) / (2.0 * assign54050_body7_e82903)))), ((locals.var_cnst0_dn10 * assign54050_body7_e82903) + (locals.var_cnst0 * (((locals.var_t2_dn10 - locals.var_t1_dn10) + ((locals.var_cnst1_dn10 * assign54050_body7_e82898) + (locals.var_cnst1 * (locals.var_t3_dn10 - locals.var_t4_dn10)))) / (2.0 * assign54050_body7_e82903)))), ((locals.var_cnst0_dn11 * assign54050_body7_e82903) + (locals.var_cnst0 * (((locals.var_t2_dn11 - locals.var_t1_dn11) + ((locals.var_cnst1_dn11 * assign54050_body7_e82898) + (locals.var_cnst1 * (locals.var_t3_dn11 - locals.var_t4_dn11)))) / (2.0 * assign54050_body7_e82903)))), ((locals.var_cnst0_dn14 * assign54050_body7_e82903) + (locals.var_cnst0 * (((locals.var_t2_dn14 - locals.var_t1_dn14) + ((locals.var_cnst1_dn14 * assign54050_body7_e82898) + (locals.var_cnst1 * (locals.var_t3_dn14 - locals.var_t4_dn14)))) / (2.0 * assign54050_body7_e82903)))),)
    } else {
        (locals.var_q_s0__blk1326, locals.var_q_s0__blk1326_dn0, locals.var_q_s0__blk1326_dn2, locals.var_q_s0__blk1326_dn4, locals.var_q_s0__blk1326_dn5, locals.var_q_s0__blk1326_dn6, locals.var_q_s0__blk1326_dn7, locals.var_q_s0__blk1326_dn8, locals.var_q_s0__blk1326_dn9, locals.var_q_s0__blk1326_dn10, locals.var_q_s0__blk1326_dn11, locals.var_q_s0__blk1326_dn14,)
    }
};
            locals.var_q_s0__blk1326 = assign54050_body7_e82906;
            locals.var_q_s0__blk1326_dn0 = assign54050_body7_e82906_d_n0;
            locals.var_q_s0__blk1326_dn2 = assign54050_body7_e82906_d_n2;
            locals.var_q_s0__blk1326_dn4 = assign54050_body7_e82906_d_n4;
            locals.var_q_s0__blk1326_dn5 = assign54050_body7_e82906_d_n5;
            locals.var_q_s0__blk1326_dn6 = assign54050_body7_e82906_d_n6;
            locals.var_q_s0__blk1326_dn7 = assign54050_body7_e82906_d_n7;
            locals.var_q_s0__blk1326_dn8 = assign54050_body7_e82906_d_n8;
            locals.var_q_s0__blk1326_dn9 = assign54050_body7_e82906_d_n9;
            locals.var_q_s0__blk1326_dn10 = assign54050_body7_e82906_d_n10;
            locals.var_q_s0__blk1326_dn11 = assign54050_body7_e82906_d_n11;
            locals.var_q_s0__blk1326_dn14 = assign54050_body7_e82906_d_n14;
            let (assign54050_body8_e82929, assign54050_body8_e82929_d_n0, assign54050_body8_e82929_d_n2, assign54050_body8_e82929_d_n4, assign54050_body8_e82929_d_n5, assign54050_body8_e82929_d_n6, assign54050_body8_e82929_d_n7, assign54050_body8_e82929_d_n8, assign54050_body8_e82929_d_n9, assign54050_body8_e82929_d_n10, assign54050_body8_e82929_d_n11, assign54050_body8_e82929_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_guard1367 == 0.0)) {
        let assign54050_body8_e82923: f64 = (0.5 * locals.var_cnst0);
        let assign54050_body8_e82925: f64 = (assign54050_body8_e82923 * locals.var_cnst0);
        let assign54050_body8_e82927: f64 = (assign54050_body8_e82925 / locals.var_q_s0__blk1326);
        (assign54050_body8_e82927, ((((((0.5 * locals.var_cnst0_dn0) * locals.var_cnst0) + (assign54050_body8_e82923 * locals.var_cnst0_dn0)) * locals.var_q_s0__blk1326) - (assign54050_body8_e82925 * locals.var_q_s0__blk1326_dn0)) / (locals.var_q_s0__blk1326 * locals.var_q_s0__blk1326)), ((((((0.5 * locals.var_cnst0_dn2) * locals.var_cnst0) + (assign54050_body8_e82923 * locals.var_cnst0_dn2)) * locals.var_q_s0__blk1326) - (assign54050_body8_e82925 * locals.var_q_s0__blk1326_dn2)) / (locals.var_q_s0__blk1326 * locals.var_q_s0__blk1326)), ((((((0.5 * locals.var_cnst0_dn4) * locals.var_cnst0) + (assign54050_body8_e82923 * locals.var_cnst0_dn4)) * locals.var_q_s0__blk1326) - (assign54050_body8_e82925 * locals.var_q_s0__blk1326_dn4)) / (locals.var_q_s0__blk1326 * locals.var_q_s0__blk1326)), ((((((0.5 * locals.var_cnst0_dn5) * locals.var_cnst0) + (assign54050_body8_e82923 * locals.var_cnst0_dn5)) * locals.var_q_s0__blk1326) - (assign54050_body8_e82925 * locals.var_q_s0__blk1326_dn5)) / (locals.var_q_s0__blk1326 * locals.var_q_s0__blk1326)), ((((((0.5 * locals.var_cnst0_dn6) * locals.var_cnst0) + (assign54050_body8_e82923 * locals.var_cnst0_dn6)) * locals.var_q_s0__blk1326) - (assign54050_body8_e82925 * locals.var_q_s0__blk1326_dn6)) / (locals.var_q_s0__blk1326 * locals.var_q_s0__blk1326)), ((((((0.5 * locals.var_cnst0_dn7) * locals.var_cnst0) + (assign54050_body8_e82923 * locals.var_cnst0_dn7)) * locals.var_q_s0__blk1326) - (assign54050_body8_e82925 * locals.var_q_s0__blk1326_dn7)) / (locals.var_q_s0__blk1326 * locals.var_q_s0__blk1326)), ((((((0.5 * locals.var_cnst0_dn8) * locals.var_cnst0) + (assign54050_body8_e82923 * locals.var_cnst0_dn8)) * locals.var_q_s0__blk1326) - (assign54050_body8_e82925 * locals.var_q_s0__blk1326_dn8)) / (locals.var_q_s0__blk1326 * locals.var_q_s0__blk1326)), ((((((0.5 * locals.var_cnst0_dn9) * locals.var_cnst0) + (assign54050_body8_e82923 * locals.var_cnst0_dn9)) * locals.var_q_s0__blk1326) - (assign54050_body8_e82925 * locals.var_q_s0__blk1326_dn9)) / (locals.var_q_s0__blk1326 * locals.var_q_s0__blk1326)), ((((((0.5 * locals.var_cnst0_dn10) * locals.var_cnst0) + (assign54050_body8_e82923 * locals.var_cnst0_dn10)) * locals.var_q_s0__blk1326) - (assign54050_body8_e82925 * locals.var_q_s0__blk1326_dn10)) / (locals.var_q_s0__blk1326 * locals.var_q_s0__blk1326)), ((((((0.5 * locals.var_cnst0_dn11) * locals.var_cnst0) + (assign54050_body8_e82923 * locals.var_cnst0_dn11)) * locals.var_q_s0__blk1326) - (assign54050_body8_e82925 * locals.var_q_s0__blk1326_dn11)) / (locals.var_q_s0__blk1326 * locals.var_q_s0__blk1326)), ((((((0.5 * locals.var_cnst0_dn14) * locals.var_cnst0) + (assign54050_body8_e82923 * locals.var_cnst0_dn14)) * locals.var_q_s0__blk1326) - (assign54050_body8_e82925 * locals.var_q_s0__blk1326_dn14)) / (locals.var_q_s0__blk1326 * locals.var_q_s0__blk1326)),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn14,)
    }
};
            locals.var_t5 = assign54050_body8_e82929;
            locals.var_t5_dn0 = assign54050_body8_e82929_d_n0;
            locals.var_t5_dn2 = assign54050_body8_e82929_d_n2;
            locals.var_t5_dn4 = assign54050_body8_e82929_d_n4;
            locals.var_t5_dn5 = assign54050_body8_e82929_d_n5;
            locals.var_t5_dn6 = assign54050_body8_e82929_d_n6;
            locals.var_t5_dn7 = assign54050_body8_e82929_d_n7;
            locals.var_t5_dn8 = assign54050_body8_e82929_d_n8;
            locals.var_t5_dn9 = assign54050_body8_e82929_d_n9;
            locals.var_t5_dn10 = assign54050_body8_e82929_d_n10;
            locals.var_t5_dn11 = assign54050_body8_e82929_d_n11;
            locals.var_t5_dn14 = assign54050_body8_e82929_d_n14;
            let (assign54050_body9_e82959, assign54050_body9_e82959_d_n0, assign54050_body9_e82959_d_n2, assign54050_body9_e82959_d_n4, assign54050_body9_e82959_d_n5, assign54050_body9_e82959_d_n6, assign54050_body9_e82959_d_n7, assign54050_body9_e82959_d_n8, assign54050_body9_e82959_d_n9, assign54050_body9_e82959_d_n10, assign54050_body9_e82959_d_n11, assign54050_body9_e82959_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_guard1367 == 0.0)) {
        let assign54050_body9_e82947: f64 = (locals.var_beta * locals.var_t2);
        let assign54050_body9_e82949: f64 = (assign54050_body9_e82947 - locals.var_beta);
        let assign54050_body9_e82952: f64 = (-locals.var_beta);
        let assign54050_body9_e82954: f64 = (assign54050_body9_e82952 * locals.var_t3);
        let assign54050_body9_e82955: f64 = (locals.var_cnst1 * assign54050_body9_e82954);
        let assign54050_body9_e82956: f64 = (assign54050_body9_e82949 + assign54050_body9_e82955);
        let assign54050_body9_e82957: f64 = (locals.var_t5 * assign54050_body9_e82956);
        (assign54050_body9_e82957, ((locals.var_t5_dn0 * assign54050_body9_e82956) + (locals.var_t5 * ((((locals.var_beta_dn0 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn0)) - locals.var_beta_dn0) + ((locals.var_cnst1_dn0 * assign54050_body9_e82954) + (locals.var_cnst1 * (((-locals.var_beta_dn0) * locals.var_t3) + (assign54050_body9_e82952 * locals.var_t3_dn0))))))), ((locals.var_t5_dn2 * assign54050_body9_e82956) + (locals.var_t5 * ((((locals.var_beta_dn2 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn2)) - locals.var_beta_dn2) + ((locals.var_cnst1_dn2 * assign54050_body9_e82954) + (locals.var_cnst1 * (((-locals.var_beta_dn2) * locals.var_t3) + (assign54050_body9_e82952 * locals.var_t3_dn2))))))), ((locals.var_t5_dn4 * assign54050_body9_e82956) + (locals.var_t5 * ((((locals.var_beta_dn4 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn4)) - locals.var_beta_dn4) + ((locals.var_cnst1_dn4 * assign54050_body9_e82954) + (locals.var_cnst1 * (((-locals.var_beta_dn4) * locals.var_t3) + (assign54050_body9_e82952 * locals.var_t3_dn4))))))), ((locals.var_t5_dn5 * assign54050_body9_e82956) + (locals.var_t5 * ((((locals.var_beta_dn5 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn5)) - locals.var_beta_dn5) + ((locals.var_cnst1_dn5 * assign54050_body9_e82954) + (locals.var_cnst1 * (((-locals.var_beta_dn5) * locals.var_t3) + (assign54050_body9_e82952 * locals.var_t3_dn5))))))), ((locals.var_t5_dn6 * assign54050_body9_e82956) + (locals.var_t5 * ((((locals.var_beta_dn6 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn6)) - locals.var_beta_dn6) + ((locals.var_cnst1_dn6 * assign54050_body9_e82954) + (locals.var_cnst1 * (((-locals.var_beta_dn6) * locals.var_t3) + (assign54050_body9_e82952 * locals.var_t3_dn6))))))), ((locals.var_t5_dn7 * assign54050_body9_e82956) + (locals.var_t5 * ((((locals.var_beta_dn7 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn7)) - locals.var_beta_dn7) + ((locals.var_cnst1_dn7 * assign54050_body9_e82954) + (locals.var_cnst1 * (((-locals.var_beta_dn7) * locals.var_t3) + (assign54050_body9_e82952 * locals.var_t3_dn7))))))), ((locals.var_t5_dn8 * assign54050_body9_e82956) + (locals.var_t5 * ((((locals.var_beta_dn8 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn8)) - locals.var_beta_dn8) + ((locals.var_cnst1_dn8 * assign54050_body9_e82954) + (locals.var_cnst1 * (((-locals.var_beta_dn8) * locals.var_t3) + (assign54050_body9_e82952 * locals.var_t3_dn8))))))), ((locals.var_t5_dn9 * assign54050_body9_e82956) + (locals.var_t5 * ((((locals.var_beta_dn9 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn9)) - locals.var_beta_dn9) + ((locals.var_cnst1_dn9 * assign54050_body9_e82954) + (locals.var_cnst1 * (((-locals.var_beta_dn9) * locals.var_t3) + (assign54050_body9_e82952 * locals.var_t3_dn9))))))), ((locals.var_t5_dn10 * assign54050_body9_e82956) + (locals.var_t5 * ((((locals.var_beta_dn10 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn10)) - locals.var_beta_dn10) + ((locals.var_cnst1_dn10 * assign54050_body9_e82954) + (locals.var_cnst1 * (((-locals.var_beta_dn10) * locals.var_t3) + (assign54050_body9_e82952 * locals.var_t3_dn10))))))), ((locals.var_t5_dn11 * assign54050_body9_e82956) + (locals.var_t5 * ((((locals.var_beta_dn11 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn11)) - locals.var_beta_dn11) + ((locals.var_cnst1_dn11 * assign54050_body9_e82954) + (locals.var_cnst1 * (((-locals.var_beta_dn11) * locals.var_t3) + (assign54050_body9_e82952 * locals.var_t3_dn11))))))), ((locals.var_t5_dn14 * assign54050_body9_e82956) + (locals.var_t5 * ((((locals.var_beta_dn14 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn14)) - locals.var_beta_dn14) + ((locals.var_cnst1_dn14 * assign54050_body9_e82954) + (locals.var_cnst1 * (((-locals.var_beta_dn14) * locals.var_t3) + (assign54050_body9_e82952 * locals.var_t3_dn14))))))),)
    } else {
        (locals.var_q_s0_dps__blk1129, locals.var_q_s0_dps__blk1129_dn0, locals.var_q_s0_dps__blk1129_dn2, locals.var_q_s0_dps__blk1129_dn4, locals.var_q_s0_dps__blk1129_dn5, locals.var_q_s0_dps__blk1129_dn6, locals.var_q_s0_dps__blk1129_dn7, locals.var_q_s0_dps__blk1129_dn8, locals.var_q_s0_dps__blk1129_dn9, locals.var_q_s0_dps__blk1129_dn10, locals.var_q_s0_dps__blk1129_dn11, locals.var_q_s0_dps__blk1129_dn14,)
    }
};
            locals.var_q_s0_dps__blk1129 = assign54050_body9_e82959;
            locals.var_q_s0_dps__blk1129_dn0 = assign54050_body9_e82959_d_n0;
            locals.var_q_s0_dps__blk1129_dn2 = assign54050_body9_e82959_d_n2;
            locals.var_q_s0_dps__blk1129_dn4 = assign54050_body9_e82959_d_n4;
            locals.var_q_s0_dps__blk1129_dn5 = assign54050_body9_e82959_d_n5;
            locals.var_q_s0_dps__blk1129_dn6 = assign54050_body9_e82959_d_n6;
            locals.var_q_s0_dps__blk1129_dn7 = assign54050_body9_e82959_d_n7;
            locals.var_q_s0_dps__blk1129_dn8 = assign54050_body9_e82959_d_n8;
            locals.var_q_s0_dps__blk1129_dn9 = assign54050_body9_e82959_d_n9;
            locals.var_q_s0_dps__blk1129_dn10 = assign54050_body9_e82959_d_n10;
            locals.var_q_s0_dps__blk1129_dn11 = assign54050_body9_e82959_d_n11;
            locals.var_q_s0_dps__blk1129_dn14 = assign54050_body9_e82959_d_n14;
            let (assign54050_body10_e82977,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_flg_conv != 0.0)) {
        let assign54050_body10_e82975: f64 = (150.0 + 1.0);
        (assign54050_body10_e82975,)
    } else {
        (locals.var_lp_s0,)
    }
};
            locals.var_lp_s0 = assign54050_body10_e82977;
            let (assign54050_body11_e83000, assign54050_body11_e83000_d_n0, assign54050_body11_e83000_d_n2, assign54050_body11_e83000_d_n4, assign54050_body11_e83000_d_n5, assign54050_body11_e83000_d_n6, assign54050_body11_e83000_d_n7, assign54050_body11_e83000_d_n8, assign54050_body11_e83000_d_n9, assign54050_body11_e83000_d_n10, assign54050_body11_e83000_d_n11, assign54050_body11_e83000_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_flg_conv == 0.0)) {
        let assign54050_body11_e82995: f64 = (locals.var_vgp_ws - locals.var_ps0dep);
        let assign54050_body11_e82996: f64 = (locals.var_cox * assign54050_body11_e82995);
        let assign54050_body11_e82998: f64 = (assign54050_body11_e82996 + locals.var_q_s0__blk1326);
        (assign54050_body11_e82998, (((locals.var_cox_dn0 * assign54050_body11_e82995) + (locals.var_cox * (locals.var_vgp_ws_dn0 - locals.var_ps0dep_dn0))) + locals.var_q_s0__blk1326_dn0), (((locals.var_cox_dn2 * assign54050_body11_e82995) + (locals.var_cox * (locals.var_vgp_ws_dn2 - locals.var_ps0dep_dn2))) + locals.var_q_s0__blk1326_dn2), (((locals.var_cox_dn4 * assign54050_body11_e82995) + (locals.var_cox * (locals.var_vgp_ws_dn4 - locals.var_ps0dep_dn4))) + locals.var_q_s0__blk1326_dn4), (((locals.var_cox_dn5 * assign54050_body11_e82995) + (locals.var_cox * (locals.var_vgp_ws_dn5 - locals.var_ps0dep_dn5))) + locals.var_q_s0__blk1326_dn5), (((locals.var_cox_dn6 * assign54050_body11_e82995) + (locals.var_cox * (locals.var_vgp_ws_dn6 - locals.var_ps0dep_dn6))) + locals.var_q_s0__blk1326_dn6), (((locals.var_cox_dn7 * assign54050_body11_e82995) + (locals.var_cox * (locals.var_vgp_ws_dn7 - locals.var_ps0dep_dn7))) + locals.var_q_s0__blk1326_dn7), (((locals.var_cox_dn8 * assign54050_body11_e82995) + (locals.var_cox * (locals.var_vgp_ws_dn8 - locals.var_ps0dep_dn8))) + locals.var_q_s0__blk1326_dn8), (((locals.var_cox_dn9 * assign54050_body11_e82995) + (locals.var_cox * (locals.var_vgp_ws_dn9 - locals.var_ps0dep_dn9))) + locals.var_q_s0__blk1326_dn9), (((locals.var_cox_dn10 * assign54050_body11_e82995) + (locals.var_cox * (locals.var_vgp_ws_dn10 - locals.var_ps0dep_dn10))) + locals.var_q_s0__blk1326_dn10), (((locals.var_cox_dn11 * assign54050_body11_e82995) + (locals.var_cox * (locals.var_vgp_ws_dn11 - locals.var_ps0dep_dn11))) + locals.var_q_s0__blk1326_dn11), (((locals.var_cox_dn14 * assign54050_body11_e82995) + (locals.var_cox * (locals.var_vgp_ws_dn14 - locals.var_ps0dep_dn14))) + locals.var_q_s0__blk1326_dn14),)
    } else {
        (locals.var_pf1__blk1104, locals.var_pf1__blk1104_dn0, locals.var_pf1__blk1104_dn2, locals.var_pf1__blk1104_dn4, locals.var_pf1__blk1104_dn5, locals.var_pf1__blk1104_dn6, locals.var_pf1__blk1104_dn7, locals.var_pf1__blk1104_dn8, locals.var_pf1__blk1104_dn9, locals.var_pf1__blk1104_dn10, locals.var_pf1__blk1104_dn11, locals.var_pf1__blk1104_dn14,)
    }
};
            locals.var_pf1__blk1104 = assign54050_body11_e83000;
            locals.var_pf1__blk1104_dn0 = assign54050_body11_e83000_d_n0;
            locals.var_pf1__blk1104_dn2 = assign54050_body11_e83000_d_n2;
            locals.var_pf1__blk1104_dn4 = assign54050_body11_e83000_d_n4;
            locals.var_pf1__blk1104_dn5 = assign54050_body11_e83000_d_n5;
            locals.var_pf1__blk1104_dn6 = assign54050_body11_e83000_d_n6;
            locals.var_pf1__blk1104_dn7 = assign54050_body11_e83000_d_n7;
            locals.var_pf1__blk1104_dn8 = assign54050_body11_e83000_d_n8;
            locals.var_pf1__blk1104_dn9 = assign54050_body11_e83000_d_n9;
            locals.var_pf1__blk1104_dn10 = assign54050_body11_e83000_d_n10;
            locals.var_pf1__blk1104_dn11 = assign54050_body11_e83000_d_n11;
            locals.var_pf1__blk1104_dn14 = assign54050_body11_e83000_d_n14;
            let (assign54050_body12_e83020, assign54050_body12_e83020_d_n0, assign54050_body12_e83020_d_n2, assign54050_body12_e83020_d_n4, assign54050_body12_e83020_d_n5, assign54050_body12_e83020_d_n6, assign54050_body12_e83020_d_n7, assign54050_body12_e83020_d_n8, assign54050_body12_e83020_d_n9, assign54050_body12_e83020_d_n10, assign54050_body12_e83020_d_n11, assign54050_body12_e83020_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_flg_conv == 0.0)) {
        let assign54050_body12_e83016: f64 = (-locals.var_cox);
        let assign54050_body12_e83018: f64 = (assign54050_body12_e83016 + locals.var_q_s0_dps__blk1129);
        (assign54050_body12_e83018, ((-locals.var_cox_dn0) + locals.var_q_s0_dps__blk1129_dn0), ((-locals.var_cox_dn2) + locals.var_q_s0_dps__blk1129_dn2), ((-locals.var_cox_dn4) + locals.var_q_s0_dps__blk1129_dn4), ((-locals.var_cox_dn5) + locals.var_q_s0_dps__blk1129_dn5), ((-locals.var_cox_dn6) + locals.var_q_s0_dps__blk1129_dn6), ((-locals.var_cox_dn7) + locals.var_q_s0_dps__blk1129_dn7), ((-locals.var_cox_dn8) + locals.var_q_s0_dps__blk1129_dn8), ((-locals.var_cox_dn9) + locals.var_q_s0_dps__blk1129_dn9), ((-locals.var_cox_dn10) + locals.var_q_s0_dps__blk1129_dn10), ((-locals.var_cox_dn11) + locals.var_q_s0_dps__blk1129_dn11), ((-locals.var_cox_dn14) + locals.var_q_s0_dps__blk1129_dn14),)
    } else {
        (locals.var_pf11__blk1105, locals.var_pf11__blk1105_dn0, locals.var_pf11__blk1105_dn2, locals.var_pf11__blk1105_dn4, locals.var_pf11__blk1105_dn5, locals.var_pf11__blk1105_dn6, locals.var_pf11__blk1105_dn7, locals.var_pf11__blk1105_dn8, locals.var_pf11__blk1105_dn9, locals.var_pf11__blk1105_dn10, locals.var_pf11__blk1105_dn11, locals.var_pf11__blk1105_dn14,)
    }
};
            locals.var_pf11__blk1105 = assign54050_body12_e83020;
            locals.var_pf11__blk1105_dn0 = assign54050_body12_e83020_d_n0;
            locals.var_pf11__blk1105_dn2 = assign54050_body12_e83020_d_n2;
            locals.var_pf11__blk1105_dn4 = assign54050_body12_e83020_d_n4;
            locals.var_pf11__blk1105_dn5 = assign54050_body12_e83020_d_n5;
            locals.var_pf11__blk1105_dn6 = assign54050_body12_e83020_d_n6;
            locals.var_pf11__blk1105_dn7 = assign54050_body12_e83020_d_n7;
            locals.var_pf11__blk1105_dn8 = assign54050_body12_e83020_d_n8;
            locals.var_pf11__blk1105_dn9 = assign54050_body12_e83020_d_n9;
            locals.var_pf11__blk1105_dn10 = assign54050_body12_e83020_d_n10;
            locals.var_pf11__blk1105_dn11 = assign54050_body12_e83020_d_n11;
            locals.var_pf11__blk1105_dn14 = assign54050_body12_e83020_d_n14;
            let (assign54050_body13_e83040, assign54050_body13_e83040_d_n0, assign54050_body13_e83040_d_n2, assign54050_body13_e83040_d_n4, assign54050_body13_e83040_d_n5, assign54050_body13_e83040_d_n6, assign54050_body13_e83040_d_n7, assign54050_body13_e83040_d_n8, assign54050_body13_e83040_d_n9, assign54050_body13_e83040_d_n10, assign54050_body13_e83040_d_n11, assign54050_body13_e83040_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_flg_conv == 0.0)) {
        let assign54050_body13_e83036: f64 = (-locals.var_pf1__blk1104);
        let assign54050_body13_e83038: f64 = (assign54050_body13_e83036 / locals.var_pf11__blk1105);
        (assign54050_body13_e83038, ((((-locals.var_pf1__blk1104_dn0) * locals.var_pf11__blk1105) - (assign54050_body13_e83036 * locals.var_pf11__blk1105_dn0)) / (locals.var_pf11__blk1105 * locals.var_pf11__blk1105)), ((((-locals.var_pf1__blk1104_dn2) * locals.var_pf11__blk1105) - (assign54050_body13_e83036 * locals.var_pf11__blk1105_dn2)) / (locals.var_pf11__blk1105 * locals.var_pf11__blk1105)), ((((-locals.var_pf1__blk1104_dn4) * locals.var_pf11__blk1105) - (assign54050_body13_e83036 * locals.var_pf11__blk1105_dn4)) / (locals.var_pf11__blk1105 * locals.var_pf11__blk1105)), ((((-locals.var_pf1__blk1104_dn5) * locals.var_pf11__blk1105) - (assign54050_body13_e83036 * locals.var_pf11__blk1105_dn5)) / (locals.var_pf11__blk1105 * locals.var_pf11__blk1105)), ((((-locals.var_pf1__blk1104_dn6) * locals.var_pf11__blk1105) - (assign54050_body13_e83036 * locals.var_pf11__blk1105_dn6)) / (locals.var_pf11__blk1105 * locals.var_pf11__blk1105)), ((((-locals.var_pf1__blk1104_dn7) * locals.var_pf11__blk1105) - (assign54050_body13_e83036 * locals.var_pf11__blk1105_dn7)) / (locals.var_pf11__blk1105 * locals.var_pf11__blk1105)), ((((-locals.var_pf1__blk1104_dn8) * locals.var_pf11__blk1105) - (assign54050_body13_e83036 * locals.var_pf11__blk1105_dn8)) / (locals.var_pf11__blk1105 * locals.var_pf11__blk1105)), ((((-locals.var_pf1__blk1104_dn9) * locals.var_pf11__blk1105) - (assign54050_body13_e83036 * locals.var_pf11__blk1105_dn9)) / (locals.var_pf11__blk1105 * locals.var_pf11__blk1105)), ((((-locals.var_pf1__blk1104_dn10) * locals.var_pf11__blk1105) - (assign54050_body13_e83036 * locals.var_pf11__blk1105_dn10)) / (locals.var_pf11__blk1105 * locals.var_pf11__blk1105)), ((((-locals.var_pf1__blk1104_dn11) * locals.var_pf11__blk1105) - (assign54050_body13_e83036 * locals.var_pf11__blk1105_dn11)) / (locals.var_pf11__blk1105 * locals.var_pf11__blk1105)), ((((-locals.var_pf1__blk1104_dn14) * locals.var_pf11__blk1105) - (assign54050_body13_e83036 * locals.var_pf11__blk1105_dn14)) / (locals.var_pf11__blk1105 * locals.var_pf11__blk1105)),)
    } else {
        (locals.var_dps__blk1116, locals.var_dps__blk1116_dn0, locals.var_dps__blk1116_dn2, locals.var_dps__blk1116_dn4, locals.var_dps__blk1116_dn5, locals.var_dps__blk1116_dn6, locals.var_dps__blk1116_dn7, locals.var_dps__blk1116_dn8, locals.var_dps__blk1116_dn9, locals.var_dps__blk1116_dn10, locals.var_dps__blk1116_dn11, locals.var_dps__blk1116_dn14,)
    }
};
            locals.var_dps__blk1116 = assign54050_body13_e83040;
            locals.var_dps__blk1116_dn0 = assign54050_body13_e83040_d_n0;
            locals.var_dps__blk1116_dn2 = assign54050_body13_e83040_d_n2;
            locals.var_dps__blk1116_dn4 = assign54050_body13_e83040_d_n4;
            locals.var_dps__blk1116_dn5 = assign54050_body13_e83040_d_n5;
            locals.var_dps__blk1116_dn6 = assign54050_body13_e83040_d_n6;
            locals.var_dps__blk1116_dn7 = assign54050_body13_e83040_d_n7;
            locals.var_dps__blk1116_dn8 = assign54050_body13_e83040_d_n8;
            locals.var_dps__blk1116_dn9 = assign54050_body13_e83040_d_n9;
            locals.var_dps__blk1116_dn10 = assign54050_body13_e83040_d_n10;
            locals.var_dps__blk1116_dn11 = assign54050_body13_e83040_d_n11;
            locals.var_dps__blk1116_dn14 = assign54050_body13_e83040_d_n14;
            let assign54050_body14_e83042: f64 = (locals.var_dps__blk1116).abs();
            let assign54050_body14_e83045: f64 = (1e-10 * 100.0);
            let assign54050_body14_e83046: f64 = if assign54050_body14_e83042 < assign54050_body14_e83045 { 1.0 } else { 0.0 };
            locals.var_guard1368 = assign54050_body14_e83046;
            let (assign54050_body15_e83065,) = {
    if (((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_flg_conv == 0.0)) && (locals.var_guard1368 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_flg_conv,)
    }
};
            locals.var_flg_conv = assign54050_body15_e83065;
            let assign54050_body16_e83068: f64 = if locals.var_dps__blk1116 > 0.1 { 1.0 } else { 0.0 };
            locals.var_guard1369 = assign54050_body16_e83068;
            let (assign54050_body17_e83090, assign54050_body17_e83090_d_n0, assign54050_body17_e83090_d_n2, assign54050_body17_e83090_d_n4, assign54050_body17_e83090_d_n5, assign54050_body17_e83090_d_n6, assign54050_body17_e83090_d_n7, assign54050_body17_e83090_d_n8, assign54050_body17_e83090_d_n9, assign54050_body17_e83090_d_n10, assign54050_body17_e83090_d_n11, assign54050_body17_e83090_d_n14,) = {
    if ((((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_flg_conv == 0.0)) && (locals.var_guard1368 == 0.0)) && (locals.var_guard1369 != 0.0)) {
        (0.1, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dps__blk1116, locals.var_dps__blk1116_dn0, locals.var_dps__blk1116_dn2, locals.var_dps__blk1116_dn4, locals.var_dps__blk1116_dn5, locals.var_dps__blk1116_dn6, locals.var_dps__blk1116_dn7, locals.var_dps__blk1116_dn8, locals.var_dps__blk1116_dn9, locals.var_dps__blk1116_dn10, locals.var_dps__blk1116_dn11, locals.var_dps__blk1116_dn14,)
    }
};
            locals.var_dps__blk1116 = assign54050_body17_e83090;
            locals.var_dps__blk1116_dn0 = assign54050_body17_e83090_d_n0;
            locals.var_dps__blk1116_dn2 = assign54050_body17_e83090_d_n2;
            locals.var_dps__blk1116_dn4 = assign54050_body17_e83090_d_n4;
            locals.var_dps__blk1116_dn5 = assign54050_body17_e83090_d_n5;
            locals.var_dps__blk1116_dn6 = assign54050_body17_e83090_d_n6;
            locals.var_dps__blk1116_dn7 = assign54050_body17_e83090_d_n7;
            locals.var_dps__blk1116_dn8 = assign54050_body17_e83090_d_n8;
            locals.var_dps__blk1116_dn9 = assign54050_body17_e83090_d_n9;
            locals.var_dps__blk1116_dn10 = assign54050_body17_e83090_d_n10;
            locals.var_dps__blk1116_dn11 = assign54050_body17_e83090_d_n11;
            locals.var_dps__blk1116_dn14 = assign54050_body17_e83090_d_n14;
            let assign54050_body18_e83093: f64 = (-0.1);
            let assign54050_body18_e83094: f64 = if locals.var_dps__blk1116 < assign54050_body18_e83093 { 1.0 } else { 0.0 };
            locals.var_guard1370 = assign54050_body18_e83094;
            let (assign54050_body19_e83120, assign54050_body19_e83120_d_n0, assign54050_body19_e83120_d_n2, assign54050_body19_e83120_d_n4, assign54050_body19_e83120_d_n5, assign54050_body19_e83120_d_n6, assign54050_body19_e83120_d_n7, assign54050_body19_e83120_d_n8, assign54050_body19_e83120_d_n9, assign54050_body19_e83120_d_n10, assign54050_body19_e83120_d_n11, assign54050_body19_e83120_d_n14,) = {
    if (((((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_flg_conv == 0.0)) && (locals.var_guard1368 == 0.0)) && (locals.var_guard1369 == 0.0)) && (locals.var_guard1370 != 0.0)) {
        let assign54050_body19_e83118: f64 = (-0.1);
        (assign54050_body19_e83118, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dps__blk1116, locals.var_dps__blk1116_dn0, locals.var_dps__blk1116_dn2, locals.var_dps__blk1116_dn4, locals.var_dps__blk1116_dn5, locals.var_dps__blk1116_dn6, locals.var_dps__blk1116_dn7, locals.var_dps__blk1116_dn8, locals.var_dps__blk1116_dn9, locals.var_dps__blk1116_dn10, locals.var_dps__blk1116_dn11, locals.var_dps__blk1116_dn14,)
    }
};
            locals.var_dps__blk1116 = assign54050_body19_e83120;
            locals.var_dps__blk1116_dn0 = assign54050_body19_e83120_d_n0;
            locals.var_dps__blk1116_dn2 = assign54050_body19_e83120_d_n2;
            locals.var_dps__blk1116_dn4 = assign54050_body19_e83120_d_n4;
            locals.var_dps__blk1116_dn5 = assign54050_body19_e83120_d_n5;
            locals.var_dps__blk1116_dn6 = assign54050_body19_e83120_d_n6;
            locals.var_dps__blk1116_dn7 = assign54050_body19_e83120_d_n7;
            locals.var_dps__blk1116_dn8 = assign54050_body19_e83120_d_n8;
            locals.var_dps__blk1116_dn9 = assign54050_body19_e83120_d_n9;
            locals.var_dps__blk1116_dn10 = assign54050_body19_e83120_d_n10;
            locals.var_dps__blk1116_dn11 = assign54050_body19_e83120_d_n11;
            locals.var_dps__blk1116_dn14 = assign54050_body19_e83120_d_n14;
            let (assign54050_body20_e83139, assign54050_body20_e83139_d_n0, assign54050_body20_e83139_d_n2, assign54050_body20_e83139_d_n4, assign54050_body20_e83139_d_n5, assign54050_body20_e83139_d_n6, assign54050_body20_e83139_d_n7, assign54050_body20_e83139_d_n8, assign54050_body20_e83139_d_n9, assign54050_body20_e83139_d_n10, assign54050_body20_e83139_d_n11, assign54050_body20_e83139_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_flg_conv == 0.0)) {
        let assign54050_body20_e83137: f64 = (locals.var_ps0dep + locals.var_dps__blk1116);
        (assign54050_body20_e83137, (locals.var_ps0dep_dn0 + locals.var_dps__blk1116_dn0), (locals.var_ps0dep_dn2 + locals.var_dps__blk1116_dn2), (locals.var_ps0dep_dn4 + locals.var_dps__blk1116_dn4), (locals.var_ps0dep_dn5 + locals.var_dps__blk1116_dn5), (locals.var_ps0dep_dn6 + locals.var_dps__blk1116_dn6), (locals.var_ps0dep_dn7 + locals.var_dps__blk1116_dn7), (locals.var_ps0dep_dn8 + locals.var_dps__blk1116_dn8), (locals.var_ps0dep_dn9 + locals.var_dps__blk1116_dn9), (locals.var_ps0dep_dn10 + locals.var_dps__blk1116_dn10), (locals.var_ps0dep_dn11 + locals.var_dps__blk1116_dn11), (locals.var_ps0dep_dn14 + locals.var_dps__blk1116_dn14),)
    } else {
        (locals.var_ps0dep, locals.var_ps0dep_dn0, locals.var_ps0dep_dn2, locals.var_ps0dep_dn4, locals.var_ps0dep_dn5, locals.var_ps0dep_dn6, locals.var_ps0dep_dn7, locals.var_ps0dep_dn8, locals.var_ps0dep_dn9, locals.var_ps0dep_dn10, locals.var_ps0dep_dn11, locals.var_ps0dep_dn14,)
    }
};
            locals.var_ps0dep = assign54050_body20_e83139;
            locals.var_ps0dep_dn0 = assign54050_body20_e83139_d_n0;
            locals.var_ps0dep_dn2 = assign54050_body20_e83139_d_n2;
            locals.var_ps0dep_dn4 = assign54050_body20_e83139_d_n4;
            locals.var_ps0dep_dn5 = assign54050_body20_e83139_d_n5;
            locals.var_ps0dep_dn6 = assign54050_body20_e83139_d_n6;
            locals.var_ps0dep_dn7 = assign54050_body20_e83139_d_n7;
            locals.var_ps0dep_dn8 = assign54050_body20_e83139_d_n8;
            locals.var_ps0dep_dn9 = assign54050_body20_e83139_d_n9;
            locals.var_ps0dep_dn10 = assign54050_body20_e83139_d_n10;
            locals.var_ps0dep_dn11 = assign54050_body20_e83139_d_n11;
            locals.var_ps0dep_dn14 = assign54050_body20_e83139_d_n14;
            let (assign54050_body21_e83155,) = {
    if (((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) {
        let assign54050_body21_e83153: f64 = (locals.var_lp_s0 + 1.0);
        (assign54050_body21_e83153,)
    } else {
        (locals.var_lp_s0,)
    }
};
            locals.var_lp_s0 = assign54050_body21_e83155;
        }

        let assign54070_e83161: f64 = if 2.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1372 = assign54070_e83161;

        let (assign54080_e83177, assign54080_e83177_d_n0, assign54080_e83177_d_n2, assign54080_e83177_d_n4, assign54080_e83177_d_n5, assign54080_e83177_d_n6, assign54080_e83177_d_n7, assign54080_e83177_d_n8, assign54080_e83177_d_n9, assign54080_e83177_d_n10, assign54080_e83177_d_n11, assign54080_e83177_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_guard1372 != 0.0)) {
        (locals.var_ps0dep, locals.var_ps0dep_dn0, locals.var_ps0dep_dn2, locals.var_ps0dep_dn4, locals.var_ps0dep_dn5, locals.var_ps0dep_dn6, locals.var_ps0dep_dn7, locals.var_ps0dep_dn8, locals.var_ps0dep_dn9, locals.var_ps0dep_dn10, locals.var_ps0dep_dn11, locals.var_ps0dep_dn14,)
    } else {
        (locals.var_ps0dep0, locals.var_ps0dep0_dn0, locals.var_ps0dep0_dn2, locals.var_ps0dep0_dn4, locals.var_ps0dep0_dn5, locals.var_ps0dep0_dn6, locals.var_ps0dep0_dn7, locals.var_ps0dep0_dn8, locals.var_ps0dep0_dn9, locals.var_ps0dep0_dn10, locals.var_ps0dep0_dn11, locals.var_ps0dep0_dn14,)
    }
};
        locals.var_ps0dep0 = assign54080_e83177;
        locals.var_ps0dep0_dn0 = assign54080_e83177_d_n0;
        locals.var_ps0dep0_dn2 = assign54080_e83177_d_n2;
        locals.var_ps0dep0_dn4 = assign54080_e83177_d_n4;
        locals.var_ps0dep0_dn5 = assign54080_e83177_d_n5;
        locals.var_ps0dep0_dn6 = assign54080_e83177_d_n6;
        locals.var_ps0dep0_dn7 = assign54080_e83177_d_n7;
        locals.var_ps0dep0_dn8 = assign54080_e83177_d_n8;
        locals.var_ps0dep0_dn9 = assign54080_e83177_d_n9;
        locals.var_ps0dep0_dn10 = assign54080_e83177_d_n10;
        locals.var_ps0dep0_dn11 = assign54080_e83177_d_n11;
        locals.var_ps0dep0_dn14 = assign54080_e83177_d_n14;

        let assign54090_e83181: f64 = (locals.var_ps0dep0 + 0.2);
        let assign54090_e83186: f64 = if ((locals.var_ps0dep < assign54090_e83181) && (0.2 >= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1373 = assign54090_e83186;

        let (assign54100_e83209, assign54100_e83209_d_n0, assign54100_e83209_d_n2, assign54100_e83209_d_n4, assign54100_e83209_d_n5, assign54100_e83209_d_n6, assign54100_e83209_d_n7, assign54100_e83209_d_n8, assign54100_e83209_d_n9, assign54100_e83209_d_n10, assign54100_e83209_d_n11, assign54100_e83209_d_n14,) = {
    if (((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_guard1372 == 0.0)) && (locals.var_guard1373 != 0.0)) {
        let assign54100_e83205: f64 = (locals.var_ps0dep0 + 0.2);
        let assign54100_e83207: f64 = (assign54100_e83205 - locals.var_ps0dep);
        (assign54100_e83207, (locals.var_ps0dep0_dn0 - locals.var_ps0dep_dn0), (locals.var_ps0dep0_dn2 - locals.var_ps0dep_dn2), (locals.var_ps0dep0_dn4 - locals.var_ps0dep_dn4), (locals.var_ps0dep0_dn5 - locals.var_ps0dep_dn5), (locals.var_ps0dep0_dn6 - locals.var_ps0dep_dn6), (locals.var_ps0dep0_dn7 - locals.var_ps0dep_dn7), (locals.var_ps0dep0_dn8 - locals.var_ps0dep_dn8), (locals.var_ps0dep0_dn9 - locals.var_ps0dep_dn9), (locals.var_ps0dep0_dn10 - locals.var_ps0dep_dn10), (locals.var_ps0dep0_dn11 - locals.var_ps0dep_dn11), (locals.var_ps0dep0_dn14 - locals.var_ps0dep_dn14),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign54100_e83209;
        locals.var_tmf1_dn0 = assign54100_e83209_d_n0;
        locals.var_tmf1_dn2 = assign54100_e83209_d_n2;
        locals.var_tmf1_dn4 = assign54100_e83209_d_n4;
        locals.var_tmf1_dn5 = assign54100_e83209_d_n5;
        locals.var_tmf1_dn6 = assign54100_e83209_d_n6;
        locals.var_tmf1_dn7 = assign54100_e83209_d_n7;
        locals.var_tmf1_dn8 = assign54100_e83209_d_n8;
        locals.var_tmf1_dn9 = assign54100_e83209_d_n9;
        locals.var_tmf1_dn10 = assign54100_e83209_d_n10;
        locals.var_tmf1_dn11 = assign54100_e83209_d_n11;
        locals.var_tmf1_dn14 = assign54100_e83209_d_n14;

        let (assign54110_e83230, assign54110_e83230_d_n0, assign54110_e83230_d_n2, assign54110_e83230_d_n4, assign54110_e83230_d_n5, assign54110_e83230_d_n6, assign54110_e83230_d_n7, assign54110_e83230_d_n8, assign54110_e83230_d_n9, assign54110_e83230_d_n10, assign54110_e83230_d_n11, assign54110_e83230_d_n14,) = {
    if (((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_guard1372 == 0.0)) && (locals.var_guard1373 != 0.0)) {
        let assign54110_e83228: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign54110_e83228, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)), ((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)), ((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)), ((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn4, locals.var_x2_dn5, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn8, locals.var_x2_dn9, locals.var_x2_dn10, locals.var_x2_dn11, locals.var_x2_dn14,)
    }
};
        locals.var_x2 = assign54110_e83230;
        locals.var_x2_dn0 = assign54110_e83230_d_n0;
        locals.var_x2_dn2 = assign54110_e83230_d_n2;
        locals.var_x2_dn4 = assign54110_e83230_d_n4;
        locals.var_x2_dn5 = assign54110_e83230_d_n5;
        locals.var_x2_dn6 = assign54110_e83230_d_n6;
        locals.var_x2_dn7 = assign54110_e83230_d_n7;
        locals.var_x2_dn8 = assign54110_e83230_d_n8;
        locals.var_x2_dn9 = assign54110_e83230_d_n9;
        locals.var_x2_dn10 = assign54110_e83230_d_n10;
        locals.var_x2_dn11 = assign54110_e83230_d_n11;
        locals.var_x2_dn14 = assign54110_e83230_d_n14;

        let (assign54120_e83251, assign54120_e83251_d_n0, assign54120_e83251_d_n2, assign54120_e83251_d_n4, assign54120_e83251_d_n5, assign54120_e83251_d_n6, assign54120_e83251_d_n7, assign54120_e83251_d_n8, assign54120_e83251_d_n9, assign54120_e83251_d_n10, assign54120_e83251_d_n11, assign54120_e83251_d_n14,) = {
    if (((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_guard1372 == 0.0)) && (locals.var_guard1373 != 0.0)) {
        let assign54120_e83249: f64 = (0.2 * 0.2);
        (assign54120_e83249, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn4, locals.var_xmax2_dn5, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn8, locals.var_xmax2_dn9, locals.var_xmax2_dn10, locals.var_xmax2_dn11, locals.var_xmax2_dn14,)
    }
};
        locals.var_xmax2 = assign54120_e83251;
        locals.var_xmax2_dn0 = assign54120_e83251_d_n0;
        locals.var_xmax2_dn2 = assign54120_e83251_d_n2;
        locals.var_xmax2_dn4 = assign54120_e83251_d_n4;
        locals.var_xmax2_dn5 = assign54120_e83251_d_n5;
        locals.var_xmax2_dn6 = assign54120_e83251_d_n6;
        locals.var_xmax2_dn7 = assign54120_e83251_d_n7;
        locals.var_xmax2_dn8 = assign54120_e83251_d_n8;
        locals.var_xmax2_dn9 = assign54120_e83251_d_n9;
        locals.var_xmax2_dn10 = assign54120_e83251_d_n10;
        locals.var_xmax2_dn11 = assign54120_e83251_d_n11;
        locals.var_xmax2_dn14 = assign54120_e83251_d_n14;

        let (assign54130_e83270, assign54130_e83270_d_n0, assign54130_e83270_d_n2, assign54130_e83270_d_n4, assign54130_e83270_d_n5, assign54130_e83270_d_n6, assign54130_e83270_d_n7, assign54130_e83270_d_n8, assign54130_e83270_d_n9, assign54130_e83270_d_n10, assign54130_e83270_d_n11, assign54130_e83270_d_n14,) = {
    if (((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_guard1372 == 0.0)) && (locals.var_guard1373 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign54130_e83270;
        locals.var_xp_dn0 = assign54130_e83270_d_n0;
        locals.var_xp_dn2 = assign54130_e83270_d_n2;
        locals.var_xp_dn4 = assign54130_e83270_d_n4;
        locals.var_xp_dn5 = assign54130_e83270_d_n5;
        locals.var_xp_dn6 = assign54130_e83270_d_n6;
        locals.var_xp_dn7 = assign54130_e83270_d_n7;
        locals.var_xp_dn8 = assign54130_e83270_d_n8;
        locals.var_xp_dn9 = assign54130_e83270_d_n9;
        locals.var_xp_dn10 = assign54130_e83270_d_n10;
        locals.var_xp_dn11 = assign54130_e83270_d_n11;
        locals.var_xp_dn14 = assign54130_e83270_d_n14;

    }

    pub(super) fn stamp_transient_block_186(
        locals: &mut StampLocals,
    ) {
        let (assign54140_e83289, assign54140_e83289_d_n0, assign54140_e83289_d_n2, assign54140_e83289_d_n4, assign54140_e83289_d_n5, assign54140_e83289_d_n6, assign54140_e83289_d_n7, assign54140_e83289_d_n8, assign54140_e83289_d_n9, assign54140_e83289_d_n10, assign54140_e83289_d_n11, assign54140_e83289_d_n14,) = {
    if (((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_guard1372 == 0.0)) && (locals.var_guard1373 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign54140_e83289;
        locals.var_xmp_dn0 = assign54140_e83289_d_n0;
        locals.var_xmp_dn2 = assign54140_e83289_d_n2;
        locals.var_xmp_dn4 = assign54140_e83289_d_n4;
        locals.var_xmp_dn5 = assign54140_e83289_d_n5;
        locals.var_xmp_dn6 = assign54140_e83289_d_n6;
        locals.var_xmp_dn7 = assign54140_e83289_d_n7;
        locals.var_xmp_dn8 = assign54140_e83289_d_n8;
        locals.var_xmp_dn9 = assign54140_e83289_d_n9;
        locals.var_xmp_dn10 = assign54140_e83289_d_n10;
        locals.var_xmp_dn11 = assign54140_e83289_d_n11;
        locals.var_xmp_dn14 = assign54140_e83289_d_n14;

        let (assign54150_e83308,) = {
    if (((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_guard1372 == 0.0)) && (locals.var_guard1373 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign54150_e83308;

        let (assign54160_e83327,) = {
    if (((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_guard1372 == 0.0)) && (locals.var_guard1373 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign54160_e83327;

        let (assign54170_e83346, assign54170_e83346_d_n0, assign54170_e83346_d_n2, assign54170_e83346_d_n4, assign54170_e83346_d_n5, assign54170_e83346_d_n6, assign54170_e83346_d_n7, assign54170_e83346_d_n8, assign54170_e83346_d_n9, assign54170_e83346_d_n10, assign54170_e83346_d_n11, assign54170_e83346_d_n14,) = {
    if (((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_guard1372 == 0.0)) && (locals.var_guard1373 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign54170_e83346;
        locals.var_arg_dn0 = assign54170_e83346_d_n0;
        locals.var_arg_dn2 = assign54170_e83346_d_n2;
        locals.var_arg_dn4 = assign54170_e83346_d_n4;
        locals.var_arg_dn5 = assign54170_e83346_d_n5;
        locals.var_arg_dn6 = assign54170_e83346_d_n6;
        locals.var_arg_dn7 = assign54170_e83346_d_n7;
        locals.var_arg_dn8 = assign54170_e83346_d_n8;
        locals.var_arg_dn9 = assign54170_e83346_d_n9;
        locals.var_arg_dn10 = assign54170_e83346_d_n10;
        locals.var_arg_dn11 = assign54170_e83346_d_n11;
        locals.var_arg_dn14 = assign54170_e83346_d_n14;

        let (assign54180_e83365, assign54180_e83365_d_n0, assign54180_e83365_d_n2, assign54180_e83365_d_n4, assign54180_e83365_d_n5, assign54180_e83365_d_n6, assign54180_e83365_d_n7, assign54180_e83365_d_n8, assign54180_e83365_d_n9, assign54180_e83365_d_n10, assign54180_e83365_d_n11, assign54180_e83365_d_n14,) = {
    if (((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_guard1372 == 0.0)) && (locals.var_guard1373 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign54180_e83365;
        locals.var_dnm_dn0 = assign54180_e83365_d_n0;
        locals.var_dnm_dn2 = assign54180_e83365_d_n2;
        locals.var_dnm_dn4 = assign54180_e83365_d_n4;
        locals.var_dnm_dn5 = assign54180_e83365_d_n5;
        locals.var_dnm_dn6 = assign54180_e83365_d_n6;
        locals.var_dnm_dn7 = assign54180_e83365_d_n7;
        locals.var_dnm_dn8 = assign54180_e83365_d_n8;
        locals.var_dnm_dn9 = assign54180_e83365_d_n9;
        locals.var_dnm_dn10 = assign54180_e83365_d_n10;
        locals.var_dnm_dn11 = assign54180_e83365_d_n11;
        locals.var_dnm_dn14 = assign54180_e83365_d_n14;

        let (assign54190_e83386, assign54190_e83386_d_n0, assign54190_e83386_d_n2, assign54190_e83386_d_n4, assign54190_e83386_d_n5, assign54190_e83386_d_n6, assign54190_e83386_d_n7, assign54190_e83386_d_n8, assign54190_e83386_d_n9, assign54190_e83386_d_n10, assign54190_e83386_d_n11, assign54190_e83386_d_n14,) = {
    if (((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_guard1372 == 0.0)) && (locals.var_guard1373 != 0.0)) {
        let assign54190_e83384: f64 = (locals.var_xp * locals.var_x2);
        (assign54190_e83384, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign54190_e83386;
        locals.var_xp_dn0 = assign54190_e83386_d_n0;
        locals.var_xp_dn2 = assign54190_e83386_d_n2;
        locals.var_xp_dn4 = assign54190_e83386_d_n4;
        locals.var_xp_dn5 = assign54190_e83386_d_n5;
        locals.var_xp_dn6 = assign54190_e83386_d_n6;
        locals.var_xp_dn7 = assign54190_e83386_d_n7;
        locals.var_xp_dn8 = assign54190_e83386_d_n8;
        locals.var_xp_dn9 = assign54190_e83386_d_n9;
        locals.var_xp_dn10 = assign54190_e83386_d_n10;
        locals.var_xp_dn11 = assign54190_e83386_d_n11;
        locals.var_xp_dn14 = assign54190_e83386_d_n14;

        let (assign54200_e83407, assign54200_e83407_d_n0, assign54200_e83407_d_n2, assign54200_e83407_d_n4, assign54200_e83407_d_n5, assign54200_e83407_d_n6, assign54200_e83407_d_n7, assign54200_e83407_d_n8, assign54200_e83407_d_n9, assign54200_e83407_d_n10, assign54200_e83407_d_n11, assign54200_e83407_d_n14,) = {
    if (((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_guard1372 == 0.0)) && (locals.var_guard1373 != 0.0)) {
        let assign54200_e83405: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign54200_e83405, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign54200_e83407;
        locals.var_xmp_dn0 = assign54200_e83407_d_n0;
        locals.var_xmp_dn2 = assign54200_e83407_d_n2;
        locals.var_xmp_dn4 = assign54200_e83407_d_n4;
        locals.var_xmp_dn5 = assign54200_e83407_d_n5;
        locals.var_xmp_dn6 = assign54200_e83407_d_n6;
        locals.var_xmp_dn7 = assign54200_e83407_d_n7;
        locals.var_xmp_dn8 = assign54200_e83407_d_n8;
        locals.var_xmp_dn9 = assign54200_e83407_d_n9;
        locals.var_xmp_dn10 = assign54200_e83407_d_n10;
        locals.var_xmp_dn11 = assign54200_e83407_d_n11;
        locals.var_xmp_dn14 = assign54200_e83407_d_n14;

        let (assign54210_e83428, assign54210_e83428_d_n0, assign54210_e83428_d_n2, assign54210_e83428_d_n4, assign54210_e83428_d_n5, assign54210_e83428_d_n6, assign54210_e83428_d_n7, assign54210_e83428_d_n8, assign54210_e83428_d_n9, assign54210_e83428_d_n10, assign54210_e83428_d_n11, assign54210_e83428_d_n14,) = {
    if (((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_guard1372 == 0.0)) && (locals.var_guard1373 != 0.0)) {
        let assign54210_e83426: f64 = (locals.var_xp * locals.var_x2);
        (assign54210_e83426, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign54210_e83428;
        locals.var_xp_dn0 = assign54210_e83428_d_n0;
        locals.var_xp_dn2 = assign54210_e83428_d_n2;
        locals.var_xp_dn4 = assign54210_e83428_d_n4;
        locals.var_xp_dn5 = assign54210_e83428_d_n5;
        locals.var_xp_dn6 = assign54210_e83428_d_n6;
        locals.var_xp_dn7 = assign54210_e83428_d_n7;
        locals.var_xp_dn8 = assign54210_e83428_d_n8;
        locals.var_xp_dn9 = assign54210_e83428_d_n9;
        locals.var_xp_dn10 = assign54210_e83428_d_n10;
        locals.var_xp_dn11 = assign54210_e83428_d_n11;
        locals.var_xp_dn14 = assign54210_e83428_d_n14;

        let (assign54220_e83449, assign54220_e83449_d_n0, assign54220_e83449_d_n2, assign54220_e83449_d_n4, assign54220_e83449_d_n5, assign54220_e83449_d_n6, assign54220_e83449_d_n7, assign54220_e83449_d_n8, assign54220_e83449_d_n9, assign54220_e83449_d_n10, assign54220_e83449_d_n11, assign54220_e83449_d_n14,) = {
    if (((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_guard1372 == 0.0)) && (locals.var_guard1373 != 0.0)) {
        let assign54220_e83447: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign54220_e83447, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign54220_e83449;
        locals.var_xmp_dn0 = assign54220_e83449_d_n0;
        locals.var_xmp_dn2 = assign54220_e83449_d_n2;
        locals.var_xmp_dn4 = assign54220_e83449_d_n4;
        locals.var_xmp_dn5 = assign54220_e83449_d_n5;
        locals.var_xmp_dn6 = assign54220_e83449_d_n6;
        locals.var_xmp_dn7 = assign54220_e83449_d_n7;
        locals.var_xmp_dn8 = assign54220_e83449_d_n8;
        locals.var_xmp_dn9 = assign54220_e83449_d_n9;
        locals.var_xmp_dn10 = assign54220_e83449_d_n10;
        locals.var_xmp_dn11 = assign54220_e83449_d_n11;
        locals.var_xmp_dn14 = assign54220_e83449_d_n14;

        let (assign54230_e83470, assign54230_e83470_d_n0, assign54230_e83470_d_n2, assign54230_e83470_d_n4, assign54230_e83470_d_n5, assign54230_e83470_d_n6, assign54230_e83470_d_n7, assign54230_e83470_d_n8, assign54230_e83470_d_n9, assign54230_e83470_d_n10, assign54230_e83470_d_n11, assign54230_e83470_d_n14,) = {
    if (((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_guard1372 == 0.0)) && (locals.var_guard1373 != 0.0)) {
        let assign54230_e83468: f64 = (locals.var_xp + locals.var_xmp);
        (assign54230_e83468, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn4 + locals.var_xmp_dn4), (locals.var_xp_dn5 + locals.var_xmp_dn5), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn8 + locals.var_xmp_dn8), (locals.var_xp_dn9 + locals.var_xmp_dn9), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn11 + locals.var_xmp_dn11), (locals.var_xp_dn14 + locals.var_xmp_dn14),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign54230_e83470;
        locals.var_arg_dn0 = assign54230_e83470_d_n0;
        locals.var_arg_dn2 = assign54230_e83470_d_n2;
        locals.var_arg_dn4 = assign54230_e83470_d_n4;
        locals.var_arg_dn5 = assign54230_e83470_d_n5;
        locals.var_arg_dn6 = assign54230_e83470_d_n6;
        locals.var_arg_dn7 = assign54230_e83470_d_n7;
        locals.var_arg_dn8 = assign54230_e83470_d_n8;
        locals.var_arg_dn9 = assign54230_e83470_d_n9;
        locals.var_arg_dn10 = assign54230_e83470_d_n10;
        locals.var_arg_dn11 = assign54230_e83470_d_n11;
        locals.var_arg_dn14 = assign54230_e83470_d_n14;

        let (assign54240_e83489, assign54240_e83489_d_n0, assign54240_e83489_d_n2, assign54240_e83489_d_n4, assign54240_e83489_d_n5, assign54240_e83489_d_n6, assign54240_e83489_d_n7, assign54240_e83489_d_n8, assign54240_e83489_d_n9, assign54240_e83489_d_n10, assign54240_e83489_d_n11, assign54240_e83489_d_n14,) = {
    if (((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_guard1372 == 0.0)) && (locals.var_guard1373 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign54240_e83489;
        locals.var_dnm_dn0 = assign54240_e83489_d_n0;
        locals.var_dnm_dn2 = assign54240_e83489_d_n2;
        locals.var_dnm_dn4 = assign54240_e83489_d_n4;
        locals.var_dnm_dn5 = assign54240_e83489_d_n5;
        locals.var_dnm_dn6 = assign54240_e83489_d_n6;
        locals.var_dnm_dn7 = assign54240_e83489_d_n7;
        locals.var_dnm_dn8 = assign54240_e83489_d_n8;
        locals.var_dnm_dn9 = assign54240_e83489_d_n9;
        locals.var_dnm_dn10 = assign54240_e83489_d_n10;
        locals.var_dnm_dn11 = assign54240_e83489_d_n11;
        locals.var_dnm_dn14 = assign54240_e83489_d_n14;

        let assign54250_e83504: f64 = if ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard1374 = assign54250_e83504;

        let assign54260_e83507: f64 = if 2.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1375 = assign54260_e83507;

        let (assign54270_e83530,) = {
    if (((((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_guard1372 == 0.0)) && (locals.var_guard1373 != 0.0)) && (locals.var_guard1374 != 0.0)) && (locals.var_guard1375 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign54270_e83530;

        let assign54280_e83533: f64 = if 2.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard1376 = assign54280_e83533;

        let (assign54290_e83559,) = {
    if ((((((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_guard1372 == 0.0)) && (locals.var_guard1373 != 0.0)) && (locals.var_guard1374 != 0.0)) && (locals.var_guard1375 == 0.0)) && (locals.var_guard1376 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign54290_e83559;

        let assign54300_e83562: f64 = if 2.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard1377 = assign54300_e83562;

        let (assign54310_e83591,) = {
    if (((((((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_guard1372 == 0.0)) && (locals.var_guard1373 != 0.0)) && (locals.var_guard1374 != 0.0)) && (locals.var_guard1375 == 0.0)) && (locals.var_guard1376 == 0.0)) && (locals.var_guard1377 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign54310_e83591;

        let assign54320_e83594: f64 = if 2.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard1378 = assign54320_e83594;

        let (assign54330_e83626,) = {
    if ((((((((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_guard1372 == 0.0)) && (locals.var_guard1373 != 0.0)) && (locals.var_guard1374 != 0.0)) && (locals.var_guard1375 == 0.0)) && (locals.var_guard1376 == 0.0)) && (locals.var_guard1377 == 0.0)) && (locals.var_guard1378 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign54330_e83626;

        let (assign54340_e83647,) = {
    if ((((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_guard1372 == 0.0)) && (locals.var_guard1373 != 0.0)) && (locals.var_guard1374 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign54340_e83647;

        let mut assign54350_loop_guard: usize = 0;
        while {
            let assign54350_cond_e83669: f64 = if (((((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_guard1372 == 0.0)) && (locals.var_guard1373 != 0.0)) && (locals.var_guard1374 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign54350_cond_e83669 != 0.0
        } {
            assign54350_loop_guard += 1;
            assert!(assign54350_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign54350_body0_e83691, assign54350_body0_e83691_d_n0, assign54350_body0_e83691_d_n2, assign54350_body0_e83691_d_n4, assign54350_body0_e83691_d_n5, assign54350_body0_e83691_d_n6, assign54350_body0_e83691_d_n7, assign54350_body0_e83691_d_n8, assign54350_body0_e83691_d_n9, assign54350_body0_e83691_d_n10, assign54350_body0_e83691_d_n11, assign54350_body0_e83691_d_n14,) = {
    if ((((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_guard1372 == 0.0)) && (locals.var_guard1373 != 0.0)) && (locals.var_guard1374 != 0.0)) {
        let assign54350_body0_e83689: f64 = (locals.var_dnm).sqrt();
        (assign54350_body0_e83689, (locals.var_dnm_dn0 / (2.0 * assign54350_body0_e83689)), (locals.var_dnm_dn2 / (2.0 * assign54350_body0_e83689)), (locals.var_dnm_dn4 / (2.0 * assign54350_body0_e83689)), (locals.var_dnm_dn5 / (2.0 * assign54350_body0_e83689)), (locals.var_dnm_dn6 / (2.0 * assign54350_body0_e83689)), (locals.var_dnm_dn7 / (2.0 * assign54350_body0_e83689)), (locals.var_dnm_dn8 / (2.0 * assign54350_body0_e83689)), (locals.var_dnm_dn9 / (2.0 * assign54350_body0_e83689)), (locals.var_dnm_dn10 / (2.0 * assign54350_body0_e83689)), (locals.var_dnm_dn11 / (2.0 * assign54350_body0_e83689)), (locals.var_dnm_dn14 / (2.0 * assign54350_body0_e83689)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
            locals.var_dnm = assign54350_body0_e83691;
            locals.var_dnm_dn0 = assign54350_body0_e83691_d_n0;
            locals.var_dnm_dn2 = assign54350_body0_e83691_d_n2;
            locals.var_dnm_dn4 = assign54350_body0_e83691_d_n4;
            locals.var_dnm_dn5 = assign54350_body0_e83691_d_n5;
            locals.var_dnm_dn6 = assign54350_body0_e83691_d_n6;
            locals.var_dnm_dn7 = assign54350_body0_e83691_d_n7;
            locals.var_dnm_dn8 = assign54350_body0_e83691_d_n8;
            locals.var_dnm_dn9 = assign54350_body0_e83691_d_n9;
            locals.var_dnm_dn10 = assign54350_body0_e83691_d_n10;
            locals.var_dnm_dn11 = assign54350_body0_e83691_d_n11;
            locals.var_dnm_dn14 = assign54350_body0_e83691_d_n14;
            let (assign54350_body1_e83714,) = {
    if ((((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_guard1372 == 0.0)) && (locals.var_guard1373 != 0.0)) && (locals.var_guard1374 != 0.0)) {
        let assign54350_body1_e83712: f64 = (locals.var_m0 + 1.0);
        (assign54350_body1_e83712,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign54350_body1_e83714;
        }

        let (assign54360_e83747, assign54360_e83747_d_n0, assign54360_e83747_d_n2, assign54360_e83747_d_n4, assign54360_e83747_d_n5, assign54360_e83747_d_n6, assign54360_e83747_d_n7, assign54360_e83747_d_n8, assign54360_e83747_d_n9, assign54360_e83747_d_n10, assign54360_e83747_d_n11, assign54360_e83747_d_n14,) = {
    if ((((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_guard1372 == 0.0)) && (locals.var_guard1373 != 0.0)) && (locals.var_guard1374 == 0.0)) {
        let (assign54360_e83745, assign54360_e83745_d_n0, assign54360_e83745_d_n2, assign54360_e83745_d_n4, assign54360_e83745_d_n5, assign54360_e83745_d_n6, assign54360_e83745_d_n7, assign54360_e83745_d_n8, assign54360_e83745_d_n9, assign54360_e83745_d_n10, assign54360_e83745_d_n11, assign54360_e83745_d_n14,) = {
            if (locals.var_dnm == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign54360_e83742: f64 = (2.0 * 2.0);
                let assign54360_e83743: f64 = (1.0 / assign54360_e83742);
                let assign54360_e83744: f64 = (locals.var_dnm).powf(assign54360_e83743);
                (assign54360_e83744, if 0.0 == 0.0 && ((assign54360_e83743) as f64).is_finite() && ((assign54360_e83743) as f64).fract() == 0.0 { if assign54360_e83743 == 0.0 { 0.0 } else { (assign54360_e83743 * ((locals.var_dnm).powf(assign54360_e83743 - 1.0) * locals.var_dnm_dn0)) } } else { (assign54360_e83744 * (assign54360_e83743 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign54360_e83743) as f64).is_finite() && ((assign54360_e83743) as f64).fract() == 0.0 { if assign54360_e83743 == 0.0 { 0.0 } else { (assign54360_e83743 * ((locals.var_dnm).powf(assign54360_e83743 - 1.0) * locals.var_dnm_dn2)) } } else { (assign54360_e83744 * (assign54360_e83743 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign54360_e83743) as f64).is_finite() && ((assign54360_e83743) as f64).fract() == 0.0 { if assign54360_e83743 == 0.0 { 0.0 } else { (assign54360_e83743 * ((locals.var_dnm).powf(assign54360_e83743 - 1.0) * locals.var_dnm_dn4)) } } else { (assign54360_e83744 * (assign54360_e83743 * (locals.var_dnm_dn4 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign54360_e83743) as f64).is_finite() && ((assign54360_e83743) as f64).fract() == 0.0 { if assign54360_e83743 == 0.0 { 0.0 } else { (assign54360_e83743 * ((locals.var_dnm).powf(assign54360_e83743 - 1.0) * locals.var_dnm_dn5)) } } else { (assign54360_e83744 * (assign54360_e83743 * (locals.var_dnm_dn5 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign54360_e83743) as f64).is_finite() && ((assign54360_e83743) as f64).fract() == 0.0 { if assign54360_e83743 == 0.0 { 0.0 } else { (assign54360_e83743 * ((locals.var_dnm).powf(assign54360_e83743 - 1.0) * locals.var_dnm_dn6)) } } else { (assign54360_e83744 * (assign54360_e83743 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign54360_e83743) as f64).is_finite() && ((assign54360_e83743) as f64).fract() == 0.0 { if assign54360_e83743 == 0.0 { 0.0 } else { (assign54360_e83743 * ((locals.var_dnm).powf(assign54360_e83743 - 1.0) * locals.var_dnm_dn7)) } } else { (assign54360_e83744 * (assign54360_e83743 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign54360_e83743) as f64).is_finite() && ((assign54360_e83743) as f64).fract() == 0.0 { if assign54360_e83743 == 0.0 { 0.0 } else { (assign54360_e83743 * ((locals.var_dnm).powf(assign54360_e83743 - 1.0) * locals.var_dnm_dn8)) } } else { (assign54360_e83744 * (assign54360_e83743 * (locals.var_dnm_dn8 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign54360_e83743) as f64).is_finite() && ((assign54360_e83743) as f64).fract() == 0.0 { if assign54360_e83743 == 0.0 { 0.0 } else { (assign54360_e83743 * ((locals.var_dnm).powf(assign54360_e83743 - 1.0) * locals.var_dnm_dn9)) } } else { (assign54360_e83744 * (assign54360_e83743 * (locals.var_dnm_dn9 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign54360_e83743) as f64).is_finite() && ((assign54360_e83743) as f64).fract() == 0.0 { if assign54360_e83743 == 0.0 { 0.0 } else { (assign54360_e83743 * ((locals.var_dnm).powf(assign54360_e83743 - 1.0) * locals.var_dnm_dn10)) } } else { (assign54360_e83744 * (assign54360_e83743 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign54360_e83743) as f64).is_finite() && ((assign54360_e83743) as f64).fract() == 0.0 { if assign54360_e83743 == 0.0 { 0.0 } else { (assign54360_e83743 * ((locals.var_dnm).powf(assign54360_e83743 - 1.0) * locals.var_dnm_dn11)) } } else { (assign54360_e83744 * (assign54360_e83743 * (locals.var_dnm_dn11 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign54360_e83743) as f64).is_finite() && ((assign54360_e83743) as f64).fract() == 0.0 { if assign54360_e83743 == 0.0 { 0.0 } else { (assign54360_e83743 * ((locals.var_dnm).powf(assign54360_e83743 - 1.0) * locals.var_dnm_dn14)) } } else { (assign54360_e83744 * (assign54360_e83743 * (locals.var_dnm_dn14 / locals.var_dnm))) },)
            }
        };
        (assign54360_e83745, assign54360_e83745_d_n0, assign54360_e83745_d_n2, assign54360_e83745_d_n4, assign54360_e83745_d_n5, assign54360_e83745_d_n6, assign54360_e83745_d_n7, assign54360_e83745_d_n8, assign54360_e83745_d_n9, assign54360_e83745_d_n10, assign54360_e83745_d_n11, assign54360_e83745_d_n14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign54360_e83747;
        locals.var_dnm_dn0 = assign54360_e83747_d_n0;
        locals.var_dnm_dn2 = assign54360_e83747_d_n2;
        locals.var_dnm_dn4 = assign54360_e83747_d_n4;
        locals.var_dnm_dn5 = assign54360_e83747_d_n5;
        locals.var_dnm_dn6 = assign54360_e83747_d_n6;
        locals.var_dnm_dn7 = assign54360_e83747_d_n7;
        locals.var_dnm_dn8 = assign54360_e83747_d_n8;
        locals.var_dnm_dn9 = assign54360_e83747_d_n9;
        locals.var_dnm_dn10 = assign54360_e83747_d_n10;
        locals.var_dnm_dn11 = assign54360_e83747_d_n11;
        locals.var_dnm_dn14 = assign54360_e83747_d_n14;

        let (assign54370_e83768, assign54370_e83768_d_n0, assign54370_e83768_d_n2, assign54370_e83768_d_n4, assign54370_e83768_d_n5, assign54370_e83768_d_n6, assign54370_e83768_d_n7, assign54370_e83768_d_n8, assign54370_e83768_d_n9, assign54370_e83768_d_n10, assign54370_e83768_d_n11, assign54370_e83768_d_n14,) = {
    if (((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_guard1372 == 0.0)) && (locals.var_guard1373 != 0.0)) {
        let assign54370_e83766: f64 = (1.0 / locals.var_dnm);
        (assign54370_e83766, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn4 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn5 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn8 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn9 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn11 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn14 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign54370_e83768;
        locals.var_dnm_dn0 = assign54370_e83768_d_n0;
        locals.var_dnm_dn2 = assign54370_e83768_d_n2;
        locals.var_dnm_dn4 = assign54370_e83768_d_n4;
        locals.var_dnm_dn5 = assign54370_e83768_d_n5;
        locals.var_dnm_dn6 = assign54370_e83768_d_n6;
        locals.var_dnm_dn7 = assign54370_e83768_d_n7;
        locals.var_dnm_dn8 = assign54370_e83768_d_n8;
        locals.var_dnm_dn9 = assign54370_e83768_d_n9;
        locals.var_dnm_dn10 = assign54370_e83768_d_n10;
        locals.var_dnm_dn11 = assign54370_e83768_d_n11;
        locals.var_dnm_dn14 = assign54370_e83768_d_n14;

        let (assign54380_e83791, assign54380_e83791_d_n0, assign54380_e83791_d_n2, assign54380_e83791_d_n4, assign54380_e83791_d_n5, assign54380_e83791_d_n6, assign54380_e83791_d_n7, assign54380_e83791_d_n8, assign54380_e83791_d_n9, assign54380_e83791_d_n10, assign54380_e83791_d_n11, assign54380_e83791_d_n14,) = {
    if (((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_guard1372 == 0.0)) && (locals.var_guard1373 != 0.0)) {
        let assign54380_e83787: f64 = (locals.var_tmf1 * 0.2);
        let assign54380_e83789: f64 = (assign54380_e83787 * locals.var_dnm);
        (assign54380_e83789, (((locals.var_tmf1_dn0 * 0.2) * locals.var_dnm) + (assign54380_e83787 * locals.var_dnm_dn0)), (((locals.var_tmf1_dn2 * 0.2) * locals.var_dnm) + (assign54380_e83787 * locals.var_dnm_dn2)), (((locals.var_tmf1_dn4 * 0.2) * locals.var_dnm) + (assign54380_e83787 * locals.var_dnm_dn4)), (((locals.var_tmf1_dn5 * 0.2) * locals.var_dnm) + (assign54380_e83787 * locals.var_dnm_dn5)), (((locals.var_tmf1_dn6 * 0.2) * locals.var_dnm) + (assign54380_e83787 * locals.var_dnm_dn6)), (((locals.var_tmf1_dn7 * 0.2) * locals.var_dnm) + (assign54380_e83787 * locals.var_dnm_dn7)), (((locals.var_tmf1_dn8 * 0.2) * locals.var_dnm) + (assign54380_e83787 * locals.var_dnm_dn8)), (((locals.var_tmf1_dn9 * 0.2) * locals.var_dnm) + (assign54380_e83787 * locals.var_dnm_dn9)), (((locals.var_tmf1_dn10 * 0.2) * locals.var_dnm) + (assign54380_e83787 * locals.var_dnm_dn10)), (((locals.var_tmf1_dn11 * 0.2) * locals.var_dnm) + (assign54380_e83787 * locals.var_dnm_dn11)), (((locals.var_tmf1_dn14 * 0.2) * locals.var_dnm) + (assign54380_e83787 * locals.var_dnm_dn14)),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn11, locals.var_tmf0_dn14,)
    }
};
        locals.var_tmf0 = assign54380_e83791;
        locals.var_tmf0_dn0 = assign54380_e83791_d_n0;
        locals.var_tmf0_dn2 = assign54380_e83791_d_n2;
        locals.var_tmf0_dn4 = assign54380_e83791_d_n4;
        locals.var_tmf0_dn5 = assign54380_e83791_d_n5;
        locals.var_tmf0_dn6 = assign54380_e83791_d_n6;
        locals.var_tmf0_dn7 = assign54380_e83791_d_n7;
        locals.var_tmf0_dn8 = assign54380_e83791_d_n8;
        locals.var_tmf0_dn9 = assign54380_e83791_d_n9;
        locals.var_tmf0_dn10 = assign54380_e83791_d_n10;
        locals.var_tmf0_dn11 = assign54380_e83791_d_n11;
        locals.var_tmf0_dn14 = assign54380_e83791_d_n14;

        let (assign54390_e83816, assign54390_e83816_d_n0, assign54390_e83816_d_n2, assign54390_e83816_d_n4, assign54390_e83816_d_n5, assign54390_e83816_d_n6, assign54390_e83816_d_n7, assign54390_e83816_d_n8, assign54390_e83816_d_n9, assign54390_e83816_d_n10, assign54390_e83816_d_n11, assign54390_e83816_d_n14,) = {
    if (((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_guard1372 == 0.0)) && (locals.var_guard1373 != 0.0)) {
        let assign54390_e83810: f64 = (0.2 * locals.var_xmp);
        let assign54390_e83812: f64 = (assign54390_e83810 * locals.var_dnm);
        let assign54390_e83814: f64 = (assign54390_e83812 / locals.var_arg);
        (assign54390_e83814, ((((((0.2 * locals.var_xmp_dn0) * locals.var_dnm) + (assign54390_e83810 * locals.var_dnm_dn0)) * locals.var_arg) - (assign54390_e83812 * locals.var_arg_dn0)) / (locals.var_arg * locals.var_arg)), ((((((0.2 * locals.var_xmp_dn2) * locals.var_dnm) + (assign54390_e83810 * locals.var_dnm_dn2)) * locals.var_arg) - (assign54390_e83812 * locals.var_arg_dn2)) / (locals.var_arg * locals.var_arg)), ((((((0.2 * locals.var_xmp_dn4) * locals.var_dnm) + (assign54390_e83810 * locals.var_dnm_dn4)) * locals.var_arg) - (assign54390_e83812 * locals.var_arg_dn4)) / (locals.var_arg * locals.var_arg)), ((((((0.2 * locals.var_xmp_dn5) * locals.var_dnm) + (assign54390_e83810 * locals.var_dnm_dn5)) * locals.var_arg) - (assign54390_e83812 * locals.var_arg_dn5)) / (locals.var_arg * locals.var_arg)), ((((((0.2 * locals.var_xmp_dn6) * locals.var_dnm) + (assign54390_e83810 * locals.var_dnm_dn6)) * locals.var_arg) - (assign54390_e83812 * locals.var_arg_dn6)) / (locals.var_arg * locals.var_arg)), ((((((0.2 * locals.var_xmp_dn7) * locals.var_dnm) + (assign54390_e83810 * locals.var_dnm_dn7)) * locals.var_arg) - (assign54390_e83812 * locals.var_arg_dn7)) / (locals.var_arg * locals.var_arg)), ((((((0.2 * locals.var_xmp_dn8) * locals.var_dnm) + (assign54390_e83810 * locals.var_dnm_dn8)) * locals.var_arg) - (assign54390_e83812 * locals.var_arg_dn8)) / (locals.var_arg * locals.var_arg)), ((((((0.2 * locals.var_xmp_dn9) * locals.var_dnm) + (assign54390_e83810 * locals.var_dnm_dn9)) * locals.var_arg) - (assign54390_e83812 * locals.var_arg_dn9)) / (locals.var_arg * locals.var_arg)), ((((((0.2 * locals.var_xmp_dn10) * locals.var_dnm) + (assign54390_e83810 * locals.var_dnm_dn10)) * locals.var_arg) - (assign54390_e83812 * locals.var_arg_dn10)) / (locals.var_arg * locals.var_arg)), ((((((0.2 * locals.var_xmp_dn11) * locals.var_dnm) + (assign54390_e83810 * locals.var_dnm_dn11)) * locals.var_arg) - (assign54390_e83812 * locals.var_arg_dn11)) / (locals.var_arg * locals.var_arg)), ((((((0.2 * locals.var_xmp_dn14) * locals.var_dnm) + (assign54390_e83810 * locals.var_dnm_dn14)) * locals.var_arg) - (assign54390_e83812 * locals.var_arg_dn14)) / (locals.var_arg * locals.var_arg)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign54390_e83816;
        locals.var_t0_dn0 = assign54390_e83816_d_n0;
        locals.var_t0_dn2 = assign54390_e83816_d_n2;
        locals.var_t0_dn4 = assign54390_e83816_d_n4;
        locals.var_t0_dn5 = assign54390_e83816_d_n5;
        locals.var_t0_dn6 = assign54390_e83816_d_n6;
        locals.var_t0_dn7 = assign54390_e83816_d_n7;
        locals.var_t0_dn8 = assign54390_e83816_d_n8;
        locals.var_t0_dn9 = assign54390_e83816_d_n9;
        locals.var_t0_dn10 = assign54390_e83816_d_n10;
        locals.var_t0_dn11 = assign54390_e83816_d_n11;
        locals.var_t0_dn14 = assign54390_e83816_d_n14;

        let (assign54400_e83839, assign54400_e83839_d_n0, assign54400_e83839_d_n2, assign54400_e83839_d_n4, assign54400_e83839_d_n5, assign54400_e83839_d_n6, assign54400_e83839_d_n7, assign54400_e83839_d_n8, assign54400_e83839_d_n9, assign54400_e83839_d_n10, assign54400_e83839_d_n11, assign54400_e83839_d_n14,) = {
    if (((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_guard1372 == 0.0)) && (locals.var_guard1373 != 0.0)) {
        let assign54400_e83835: f64 = (locals.var_ps0dep0 + 0.2);
        let assign54400_e83837: f64 = (assign54400_e83835 - locals.var_tmf0);
        (assign54400_e83837, (locals.var_ps0dep0_dn0 - locals.var_tmf0_dn0), (locals.var_ps0dep0_dn2 - locals.var_tmf0_dn2), (locals.var_ps0dep0_dn4 - locals.var_tmf0_dn4), (locals.var_ps0dep0_dn5 - locals.var_tmf0_dn5), (locals.var_ps0dep0_dn6 - locals.var_tmf0_dn6), (locals.var_ps0dep0_dn7 - locals.var_tmf0_dn7), (locals.var_ps0dep0_dn8 - locals.var_tmf0_dn8), (locals.var_ps0dep0_dn9 - locals.var_tmf0_dn9), (locals.var_ps0dep0_dn10 - locals.var_tmf0_dn10), (locals.var_ps0dep0_dn11 - locals.var_tmf0_dn11), (locals.var_ps0dep0_dn14 - locals.var_tmf0_dn14),)
    } else {
        (locals.var_ps0dep, locals.var_ps0dep_dn0, locals.var_ps0dep_dn2, locals.var_ps0dep_dn4, locals.var_ps0dep_dn5, locals.var_ps0dep_dn6, locals.var_ps0dep_dn7, locals.var_ps0dep_dn8, locals.var_ps0dep_dn9, locals.var_ps0dep_dn10, locals.var_ps0dep_dn11, locals.var_ps0dep_dn14,)
    }
};
        locals.var_ps0dep = assign54400_e83839;
        locals.var_ps0dep_dn0 = assign54400_e83839_d_n0;
        locals.var_ps0dep_dn2 = assign54400_e83839_d_n2;
        locals.var_ps0dep_dn4 = assign54400_e83839_d_n4;
        locals.var_ps0dep_dn5 = assign54400_e83839_d_n5;
        locals.var_ps0dep_dn6 = assign54400_e83839_d_n6;
        locals.var_ps0dep_dn7 = assign54400_e83839_d_n7;
        locals.var_ps0dep_dn8 = assign54400_e83839_d_n8;
        locals.var_ps0dep_dn9 = assign54400_e83839_d_n9;
        locals.var_ps0dep_dn10 = assign54400_e83839_d_n10;
        locals.var_ps0dep_dn11 = assign54400_e83839_d_n11;
        locals.var_ps0dep_dn14 = assign54400_e83839_d_n14;

        let (assign54410_e83858, assign54410_e83858_d_n0, assign54410_e83858_d_n2, assign54410_e83858_d_n4, assign54410_e83858_d_n5, assign54410_e83858_d_n6, assign54410_e83858_d_n7, assign54410_e83858_d_n8, assign54410_e83858_d_n9, assign54410_e83858_d_n10, assign54410_e83858_d_n11, assign54410_e83858_d_n14,) = {
    if (((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_guard1372 == 0.0)) && (locals.var_guard1373 != 0.0)) {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign54410_e83858;
        locals.var_t0_dn0 = assign54410_e83858_d_n0;
        locals.var_t0_dn2 = assign54410_e83858_d_n2;
        locals.var_t0_dn4 = assign54410_e83858_d_n4;
        locals.var_t0_dn5 = assign54410_e83858_d_n5;
        locals.var_t0_dn6 = assign54410_e83858_d_n6;
        locals.var_t0_dn7 = assign54410_e83858_d_n7;
        locals.var_t0_dn8 = assign54410_e83858_d_n8;
        locals.var_t0_dn9 = assign54410_e83858_d_n9;
        locals.var_t0_dn10 = assign54410_e83858_d_n10;
        locals.var_t0_dn11 = assign54410_e83858_d_n11;
        locals.var_t0_dn14 = assign54410_e83858_d_n14;

        let (assign54420_e83878, assign54420_e83878_d_n0, assign54420_e83878_d_n2, assign54420_e83878_d_n4, assign54420_e83878_d_n5, assign54420_e83878_d_n6, assign54420_e83878_d_n7, assign54420_e83878_d_n8, assign54420_e83878_d_n9, assign54420_e83878_d_n10, assign54420_e83878_d_n11, assign54420_e83878_d_n14,) = {
    if (((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_guard1372 == 0.0)) && (locals.var_guard1373 == 0.0)) {
        (locals.var_ps0dep, locals.var_ps0dep_dn0, locals.var_ps0dep_dn2, locals.var_ps0dep_dn4, locals.var_ps0dep_dn5, locals.var_ps0dep_dn6, locals.var_ps0dep_dn7, locals.var_ps0dep_dn8, locals.var_ps0dep_dn9, locals.var_ps0dep_dn10, locals.var_ps0dep_dn11, locals.var_ps0dep_dn14,)
    } else {
        (locals.var_ps0dep, locals.var_ps0dep_dn0, locals.var_ps0dep_dn2, locals.var_ps0dep_dn4, locals.var_ps0dep_dn5, locals.var_ps0dep_dn6, locals.var_ps0dep_dn7, locals.var_ps0dep_dn8, locals.var_ps0dep_dn9, locals.var_ps0dep_dn10, locals.var_ps0dep_dn11, locals.var_ps0dep_dn14,)
    }
};
        locals.var_ps0dep = assign54420_e83878;
        locals.var_ps0dep_dn0 = assign54420_e83878_d_n0;
        locals.var_ps0dep_dn2 = assign54420_e83878_d_n2;
        locals.var_ps0dep_dn4 = assign54420_e83878_d_n4;
        locals.var_ps0dep_dn5 = assign54420_e83878_d_n5;
        locals.var_ps0dep_dn6 = assign54420_e83878_d_n6;
        locals.var_ps0dep_dn7 = assign54420_e83878_d_n7;
        locals.var_ps0dep_dn8 = assign54420_e83878_d_n8;
        locals.var_ps0dep_dn9 = assign54420_e83878_d_n9;
        locals.var_ps0dep_dn10 = assign54420_e83878_d_n10;
        locals.var_ps0dep_dn11 = assign54420_e83878_d_n11;
        locals.var_ps0dep_dn14 = assign54420_e83878_d_n14;

        let (assign54430_e83898, assign54430_e83898_d_n0, assign54430_e83898_d_n2, assign54430_e83898_d_n4, assign54430_e83898_d_n5, assign54430_e83898_d_n6, assign54430_e83898_d_n7, assign54430_e83898_d_n8, assign54430_e83898_d_n9, assign54430_e83898_d_n10, assign54430_e83898_d_n11, assign54430_e83898_d_n14,) = {
    if (((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_guard1372 == 0.0)) && (locals.var_guard1373 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign54430_e83898;
        locals.var_t0_dn0 = assign54430_e83898_d_n0;
        locals.var_t0_dn2 = assign54430_e83898_d_n2;
        locals.var_t0_dn4 = assign54430_e83898_d_n4;
        locals.var_t0_dn5 = assign54430_e83898_d_n5;
        locals.var_t0_dn6 = assign54430_e83898_d_n6;
        locals.var_t0_dn7 = assign54430_e83898_d_n7;
        locals.var_t0_dn8 = assign54430_e83898_d_n8;
        locals.var_t0_dn9 = assign54430_e83898_d_n9;
        locals.var_t0_dn10 = assign54430_e83898_d_n10;
        locals.var_t0_dn11 = assign54430_e83898_d_n11;
        locals.var_t0_dn14 = assign54430_e83898_d_n14;

        let (assign54440_e83912, assign54440_e83912_d_n0, assign54440_e83912_d_n2, assign54440_e83912_d_n4, assign54440_e83912_d_n5, assign54440_e83912_d_n6, assign54440_e83912_d_n7, assign54440_e83912_d_n8, assign54440_e83912_d_n9, assign54440_e83912_d_n10, assign54440_e83912_d_n11, assign54440_e83912_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) {
        (locals.var_ps0dep, locals.var_ps0dep_dn0, locals.var_ps0dep_dn2, locals.var_ps0dep_dn4, locals.var_ps0dep_dn5, locals.var_ps0dep_dn6, locals.var_ps0dep_dn7, locals.var_ps0dep_dn8, locals.var_ps0dep_dn9, locals.var_ps0dep_dn10, locals.var_ps0dep_dn11, locals.var_ps0dep_dn14,)
    } else {
        (locals.var_ps0_res, locals.var_ps0_res_dn0, locals.var_ps0_res_dn2, locals.var_ps0_res_dn4, locals.var_ps0_res_dn5, locals.var_ps0_res_dn6, locals.var_ps0_res_dn7, locals.var_ps0_res_dn8, locals.var_ps0_res_dn9, locals.var_ps0_res_dn10, locals.var_ps0_res_dn11, locals.var_ps0_res_dn14,)
    }
};
        locals.var_ps0_res = assign54440_e83912;
        locals.var_ps0_res_dn0 = assign54440_e83912_d_n0;
        locals.var_ps0_res_dn2 = assign54440_e83912_d_n2;
        locals.var_ps0_res_dn4 = assign54440_e83912_d_n4;
        locals.var_ps0_res_dn5 = assign54440_e83912_d_n5;
        locals.var_ps0_res_dn6 = assign54440_e83912_d_n6;
        locals.var_ps0_res_dn7 = assign54440_e83912_d_n7;
        locals.var_ps0_res_dn8 = assign54440_e83912_d_n8;
        locals.var_ps0_res_dn9 = assign54440_e83912_d_n9;
        locals.var_ps0_res_dn10 = assign54440_e83912_d_n10;
        locals.var_ps0_res_dn11 = assign54440_e83912_d_n11;
        locals.var_ps0_res_dn14 = assign54440_e83912_d_n14;

    }

    pub(super) fn stamp_transient_block_187(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign54450_e83931,) = {
    if (((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) {
        let (assign54450_e83929,) = {
            if (1e-6 >= p.p407) {
                (1e-6,)
            } else {
                (p.p407,)
            }
        };
        (assign54450_e83929,)
    } else {
        (locals.var_vgpdep_dlt__blk1146,)
    }
};
        locals.var_vgpdep_dlt__blk1146 = assign54450_e83931;

        let assign54460_e83935: f64 = (-locals.var_vgpdep_dlt__blk1146);
        let assign54460_e83940: f64 = if ((locals.var_ps0_res > assign54460_e83935) && (locals.var_vgpdep_dlt__blk1146 >= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1379 = assign54460_e83940;

        let (assign54470_e83960, assign54470_e83960_d_n0, assign54470_e83960_d_n2, assign54470_e83960_d_n4, assign54470_e83960_d_n5, assign54470_e83960_d_n6, assign54470_e83960_d_n7, assign54470_e83960_d_n8, assign54470_e83960_d_n9, assign54470_e83960_d_n10, assign54470_e83960_d_n11, assign54470_e83960_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_guard1379 != 0.0)) {
        let assign54470_e83956: f64 = locals.var_ps0_res;
        let assign54470_e83958: f64 = (assign54470_e83956 + locals.var_vgpdep_dlt__blk1146);
        (assign54470_e83958, locals.var_ps0_res_dn0, locals.var_ps0_res_dn2, locals.var_ps0_res_dn4, locals.var_ps0_res_dn5, locals.var_ps0_res_dn6, locals.var_ps0_res_dn7, locals.var_ps0_res_dn8, locals.var_ps0_res_dn9, locals.var_ps0_res_dn10, locals.var_ps0_res_dn11, locals.var_ps0_res_dn14,)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign54470_e83960;
        locals.var_tmf1_dn0 = assign54470_e83960_d_n0;
        locals.var_tmf1_dn2 = assign54470_e83960_d_n2;
        locals.var_tmf1_dn4 = assign54470_e83960_d_n4;
        locals.var_tmf1_dn5 = assign54470_e83960_d_n5;
        locals.var_tmf1_dn6 = assign54470_e83960_d_n6;
        locals.var_tmf1_dn7 = assign54470_e83960_d_n7;
        locals.var_tmf1_dn8 = assign54470_e83960_d_n8;
        locals.var_tmf1_dn9 = assign54470_e83960_d_n9;
        locals.var_tmf1_dn10 = assign54470_e83960_d_n10;
        locals.var_tmf1_dn11 = assign54470_e83960_d_n11;
        locals.var_tmf1_dn14 = assign54470_e83960_d_n14;

        let (assign54480_e83978, assign54480_e83978_d_n0, assign54480_e83978_d_n2, assign54480_e83978_d_n4, assign54480_e83978_d_n5, assign54480_e83978_d_n6, assign54480_e83978_d_n7, assign54480_e83978_d_n8, assign54480_e83978_d_n9, assign54480_e83978_d_n10, assign54480_e83978_d_n11, assign54480_e83978_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_guard1379 != 0.0)) {
        let assign54480_e83976: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign54480_e83976, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)), ((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)), ((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)), ((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn4, locals.var_x2_dn5, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn8, locals.var_x2_dn9, locals.var_x2_dn10, locals.var_x2_dn11, locals.var_x2_dn14,)
    }
};
        locals.var_x2 = assign54480_e83978;
        locals.var_x2_dn0 = assign54480_e83978_d_n0;
        locals.var_x2_dn2 = assign54480_e83978_d_n2;
        locals.var_x2_dn4 = assign54480_e83978_d_n4;
        locals.var_x2_dn5 = assign54480_e83978_d_n5;
        locals.var_x2_dn6 = assign54480_e83978_d_n6;
        locals.var_x2_dn7 = assign54480_e83978_d_n7;
        locals.var_x2_dn8 = assign54480_e83978_d_n8;
        locals.var_x2_dn9 = assign54480_e83978_d_n9;
        locals.var_x2_dn10 = assign54480_e83978_d_n10;
        locals.var_x2_dn11 = assign54480_e83978_d_n11;
        locals.var_x2_dn14 = assign54480_e83978_d_n14;

        let (assign54490_e83996, assign54490_e83996_d_n0, assign54490_e83996_d_n2, assign54490_e83996_d_n4, assign54490_e83996_d_n5, assign54490_e83996_d_n6, assign54490_e83996_d_n7, assign54490_e83996_d_n8, assign54490_e83996_d_n9, assign54490_e83996_d_n10, assign54490_e83996_d_n11, assign54490_e83996_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_guard1379 != 0.0)) {
        let assign54490_e83994: f64 = (locals.var_vgpdep_dlt__blk1146 * locals.var_vgpdep_dlt__blk1146);
        (assign54490_e83994, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn4, locals.var_xmax2_dn5, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn8, locals.var_xmax2_dn9, locals.var_xmax2_dn10, locals.var_xmax2_dn11, locals.var_xmax2_dn14,)
    }
};
        locals.var_xmax2 = assign54490_e83996;
        locals.var_xmax2_dn0 = assign54490_e83996_d_n0;
        locals.var_xmax2_dn2 = assign54490_e83996_d_n2;
        locals.var_xmax2_dn4 = assign54490_e83996_d_n4;
        locals.var_xmax2_dn5 = assign54490_e83996_d_n5;
        locals.var_xmax2_dn6 = assign54490_e83996_d_n6;
        locals.var_xmax2_dn7 = assign54490_e83996_d_n7;
        locals.var_xmax2_dn8 = assign54490_e83996_d_n8;
        locals.var_xmax2_dn9 = assign54490_e83996_d_n9;
        locals.var_xmax2_dn10 = assign54490_e83996_d_n10;
        locals.var_xmax2_dn11 = assign54490_e83996_d_n11;
        locals.var_xmax2_dn14 = assign54490_e83996_d_n14;

        let (assign54500_e84012, assign54500_e84012_d_n0, assign54500_e84012_d_n2, assign54500_e84012_d_n4, assign54500_e84012_d_n5, assign54500_e84012_d_n6, assign54500_e84012_d_n7, assign54500_e84012_d_n8, assign54500_e84012_d_n9, assign54500_e84012_d_n10, assign54500_e84012_d_n11, assign54500_e84012_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_guard1379 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign54500_e84012;
        locals.var_xp_dn0 = assign54500_e84012_d_n0;
        locals.var_xp_dn2 = assign54500_e84012_d_n2;
        locals.var_xp_dn4 = assign54500_e84012_d_n4;
        locals.var_xp_dn5 = assign54500_e84012_d_n5;
        locals.var_xp_dn6 = assign54500_e84012_d_n6;
        locals.var_xp_dn7 = assign54500_e84012_d_n7;
        locals.var_xp_dn8 = assign54500_e84012_d_n8;
        locals.var_xp_dn9 = assign54500_e84012_d_n9;
        locals.var_xp_dn10 = assign54500_e84012_d_n10;
        locals.var_xp_dn11 = assign54500_e84012_d_n11;
        locals.var_xp_dn14 = assign54500_e84012_d_n14;

        let (assign54510_e84028, assign54510_e84028_d_n0, assign54510_e84028_d_n2, assign54510_e84028_d_n4, assign54510_e84028_d_n5, assign54510_e84028_d_n6, assign54510_e84028_d_n7, assign54510_e84028_d_n8, assign54510_e84028_d_n9, assign54510_e84028_d_n10, assign54510_e84028_d_n11, assign54510_e84028_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_guard1379 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign54510_e84028;
        locals.var_xmp_dn0 = assign54510_e84028_d_n0;
        locals.var_xmp_dn2 = assign54510_e84028_d_n2;
        locals.var_xmp_dn4 = assign54510_e84028_d_n4;
        locals.var_xmp_dn5 = assign54510_e84028_d_n5;
        locals.var_xmp_dn6 = assign54510_e84028_d_n6;
        locals.var_xmp_dn7 = assign54510_e84028_d_n7;
        locals.var_xmp_dn8 = assign54510_e84028_d_n8;
        locals.var_xmp_dn9 = assign54510_e84028_d_n9;
        locals.var_xmp_dn10 = assign54510_e84028_d_n10;
        locals.var_xmp_dn11 = assign54510_e84028_d_n11;
        locals.var_xmp_dn14 = assign54510_e84028_d_n14;

        let (assign54520_e84044,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_guard1379 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign54520_e84044;

        let (assign54530_e84060,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_guard1379 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign54530_e84060;

        let (assign54540_e84076, assign54540_e84076_d_n0, assign54540_e84076_d_n2, assign54540_e84076_d_n4, assign54540_e84076_d_n5, assign54540_e84076_d_n6, assign54540_e84076_d_n7, assign54540_e84076_d_n8, assign54540_e84076_d_n9, assign54540_e84076_d_n10, assign54540_e84076_d_n11, assign54540_e84076_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_guard1379 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign54540_e84076;
        locals.var_arg_dn0 = assign54540_e84076_d_n0;
        locals.var_arg_dn2 = assign54540_e84076_d_n2;
        locals.var_arg_dn4 = assign54540_e84076_d_n4;
        locals.var_arg_dn5 = assign54540_e84076_d_n5;
        locals.var_arg_dn6 = assign54540_e84076_d_n6;
        locals.var_arg_dn7 = assign54540_e84076_d_n7;
        locals.var_arg_dn8 = assign54540_e84076_d_n8;
        locals.var_arg_dn9 = assign54540_e84076_d_n9;
        locals.var_arg_dn10 = assign54540_e84076_d_n10;
        locals.var_arg_dn11 = assign54540_e84076_d_n11;
        locals.var_arg_dn14 = assign54540_e84076_d_n14;

        let (assign54550_e84092, assign54550_e84092_d_n0, assign54550_e84092_d_n2, assign54550_e84092_d_n4, assign54550_e84092_d_n5, assign54550_e84092_d_n6, assign54550_e84092_d_n7, assign54550_e84092_d_n8, assign54550_e84092_d_n9, assign54550_e84092_d_n10, assign54550_e84092_d_n11, assign54550_e84092_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_guard1379 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign54550_e84092;
        locals.var_dnm_dn0 = assign54550_e84092_d_n0;
        locals.var_dnm_dn2 = assign54550_e84092_d_n2;
        locals.var_dnm_dn4 = assign54550_e84092_d_n4;
        locals.var_dnm_dn5 = assign54550_e84092_d_n5;
        locals.var_dnm_dn6 = assign54550_e84092_d_n6;
        locals.var_dnm_dn7 = assign54550_e84092_d_n7;
        locals.var_dnm_dn8 = assign54550_e84092_d_n8;
        locals.var_dnm_dn9 = assign54550_e84092_d_n9;
        locals.var_dnm_dn10 = assign54550_e84092_d_n10;
        locals.var_dnm_dn11 = assign54550_e84092_d_n11;
        locals.var_dnm_dn14 = assign54550_e84092_d_n14;

        let (assign54560_e84108,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_guard1379 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign54560_e84108;

        let mut assign54570_loop_guard: usize = 0;
        while {
            let assign54570_cond_e84125: f64 = if (((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_guard1379 != 0.0)) && (locals.var_m0 < locals.var_vgpdep_pw__blk1147)) { 1.0 } else { 0.0 };
            assign54570_cond_e84125 != 0.0
        } {
            assign54570_loop_guard += 1;
            assert!(assign54570_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign54570_body0_e84143, assign54570_body0_e84143_d_n0, assign54570_body0_e84143_d_n2, assign54570_body0_e84143_d_n4, assign54570_body0_e84143_d_n5, assign54570_body0_e84143_d_n6, assign54570_body0_e84143_d_n7, assign54570_body0_e84143_d_n8, assign54570_body0_e84143_d_n9, assign54570_body0_e84143_d_n10, assign54570_body0_e84143_d_n11, assign54570_body0_e84143_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_guard1379 != 0.0)) {
        let assign54570_body0_e84141: f64 = (locals.var_xp * locals.var_x2);
        (assign54570_body0_e84141, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
            locals.var_xp = assign54570_body0_e84143;
            locals.var_xp_dn0 = assign54570_body0_e84143_d_n0;
            locals.var_xp_dn2 = assign54570_body0_e84143_d_n2;
            locals.var_xp_dn4 = assign54570_body0_e84143_d_n4;
            locals.var_xp_dn5 = assign54570_body0_e84143_d_n5;
            locals.var_xp_dn6 = assign54570_body0_e84143_d_n6;
            locals.var_xp_dn7 = assign54570_body0_e84143_d_n7;
            locals.var_xp_dn8 = assign54570_body0_e84143_d_n8;
            locals.var_xp_dn9 = assign54570_body0_e84143_d_n9;
            locals.var_xp_dn10 = assign54570_body0_e84143_d_n10;
            locals.var_xp_dn11 = assign54570_body0_e84143_d_n11;
            locals.var_xp_dn14 = assign54570_body0_e84143_d_n14;
            let (assign54570_body1_e84161, assign54570_body1_e84161_d_n0, assign54570_body1_e84161_d_n2, assign54570_body1_e84161_d_n4, assign54570_body1_e84161_d_n5, assign54570_body1_e84161_d_n6, assign54570_body1_e84161_d_n7, assign54570_body1_e84161_d_n8, assign54570_body1_e84161_d_n9, assign54570_body1_e84161_d_n10, assign54570_body1_e84161_d_n11, assign54570_body1_e84161_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_guard1379 != 0.0)) {
        let assign54570_body1_e84159: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign54570_body1_e84159, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
            locals.var_xmp = assign54570_body1_e84161;
            locals.var_xmp_dn0 = assign54570_body1_e84161_d_n0;
            locals.var_xmp_dn2 = assign54570_body1_e84161_d_n2;
            locals.var_xmp_dn4 = assign54570_body1_e84161_d_n4;
            locals.var_xmp_dn5 = assign54570_body1_e84161_d_n5;
            locals.var_xmp_dn6 = assign54570_body1_e84161_d_n6;
            locals.var_xmp_dn7 = assign54570_body1_e84161_d_n7;
            locals.var_xmp_dn8 = assign54570_body1_e84161_d_n8;
            locals.var_xmp_dn9 = assign54570_body1_e84161_d_n9;
            locals.var_xmp_dn10 = assign54570_body1_e84161_d_n10;
            locals.var_xmp_dn11 = assign54570_body1_e84161_d_n11;
            locals.var_xmp_dn14 = assign54570_body1_e84161_d_n14;
            let (assign54570_body2_e84179,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_guard1379 != 0.0)) {
        let assign54570_body2_e84177: f64 = (locals.var_m0 + 1.0);
        (assign54570_body2_e84177,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign54570_body2_e84179;
        }

        let (assign54580_e84197, assign54580_e84197_d_n0, assign54580_e84197_d_n2, assign54580_e84197_d_n4, assign54580_e84197_d_n5, assign54580_e84197_d_n6, assign54580_e84197_d_n7, assign54580_e84197_d_n8, assign54580_e84197_d_n9, assign54580_e84197_d_n10, assign54580_e84197_d_n11, assign54580_e84197_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_guard1379 != 0.0)) {
        let assign54580_e84195: f64 = (locals.var_xp + locals.var_xmp);
        (assign54580_e84195, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn4 + locals.var_xmp_dn4), (locals.var_xp_dn5 + locals.var_xmp_dn5), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn8 + locals.var_xmp_dn8), (locals.var_xp_dn9 + locals.var_xmp_dn9), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn11 + locals.var_xmp_dn11), (locals.var_xp_dn14 + locals.var_xmp_dn14),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign54580_e84197;
        locals.var_arg_dn0 = assign54580_e84197_d_n0;
        locals.var_arg_dn2 = assign54580_e84197_d_n2;
        locals.var_arg_dn4 = assign54580_e84197_d_n4;
        locals.var_arg_dn5 = assign54580_e84197_d_n5;
        locals.var_arg_dn6 = assign54580_e84197_d_n6;
        locals.var_arg_dn7 = assign54580_e84197_d_n7;
        locals.var_arg_dn8 = assign54580_e84197_d_n8;
        locals.var_arg_dn9 = assign54580_e84197_d_n9;
        locals.var_arg_dn10 = assign54580_e84197_d_n10;
        locals.var_arg_dn11 = assign54580_e84197_d_n11;
        locals.var_arg_dn14 = assign54580_e84197_d_n14;

        let (assign54590_e84213, assign54590_e84213_d_n0, assign54590_e84213_d_n2, assign54590_e84213_d_n4, assign54590_e84213_d_n5, assign54590_e84213_d_n6, assign54590_e84213_d_n7, assign54590_e84213_d_n8, assign54590_e84213_d_n9, assign54590_e84213_d_n10, assign54590_e84213_d_n11, assign54590_e84213_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_guard1379 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign54590_e84213;
        locals.var_dnm_dn0 = assign54590_e84213_d_n0;
        locals.var_dnm_dn2 = assign54590_e84213_d_n2;
        locals.var_dnm_dn4 = assign54590_e84213_d_n4;
        locals.var_dnm_dn5 = assign54590_e84213_d_n5;
        locals.var_dnm_dn6 = assign54590_e84213_d_n6;
        locals.var_dnm_dn7 = assign54590_e84213_d_n7;
        locals.var_dnm_dn8 = assign54590_e84213_d_n8;
        locals.var_dnm_dn9 = assign54590_e84213_d_n9;
        locals.var_dnm_dn10 = assign54590_e84213_d_n10;
        locals.var_dnm_dn11 = assign54590_e84213_d_n11;
        locals.var_dnm_dn14 = assign54590_e84213_d_n14;

        let assign54600_e84228: f64 = if ((((locals.var_vgpdep_pw__blk1147 == 1.0) || (locals.var_vgpdep_pw__blk1147 == 2.0)) || (locals.var_vgpdep_pw__blk1147 == 4.0)) || (locals.var_vgpdep_pw__blk1147 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard1380 = assign54600_e84228;

        let assign54610_e84231: f64 = if locals.var_vgpdep_pw__blk1147 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1381 = assign54610_e84231;

        let (assign54620_e84251,) = {
    if ((((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_guard1379 != 0.0)) && (locals.var_guard1380 != 0.0)) && (locals.var_guard1381 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign54620_e84251;

        let assign54630_e84254: f64 = if locals.var_vgpdep_pw__blk1147 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard1382 = assign54630_e84254;

        let (assign54640_e84277,) = {
    if (((((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_guard1379 != 0.0)) && (locals.var_guard1380 != 0.0)) && (locals.var_guard1381 == 0.0)) && (locals.var_guard1382 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign54640_e84277;

        let assign54650_e84280: f64 = if locals.var_vgpdep_pw__blk1147 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard1383 = assign54650_e84280;

        let (assign54660_e84306,) = {
    if ((((((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_guard1379 != 0.0)) && (locals.var_guard1380 != 0.0)) && (locals.var_guard1381 == 0.0)) && (locals.var_guard1382 == 0.0)) && (locals.var_guard1383 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign54660_e84306;

        let assign54670_e84309: f64 = if locals.var_vgpdep_pw__blk1147 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard1384 = assign54670_e84309;

        let (assign54680_e84338,) = {
    if (((((((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_guard1379 != 0.0)) && (locals.var_guard1380 != 0.0)) && (locals.var_guard1381 == 0.0)) && (locals.var_guard1382 == 0.0)) && (locals.var_guard1383 == 0.0)) && (locals.var_guard1384 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign54680_e84338;

        let (assign54690_e84356,) = {
    if (((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_guard1379 != 0.0)) && (locals.var_guard1380 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign54690_e84356;

        let mut assign54700_loop_guard: usize = 0;
        while {
            let assign54700_cond_e84375: f64 = if ((((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_guard1379 != 0.0)) && (locals.var_guard1380 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign54700_cond_e84375 != 0.0
        } {
            assign54700_loop_guard += 1;
            assert!(assign54700_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign54700_body0_e84394, assign54700_body0_e84394_d_n0, assign54700_body0_e84394_d_n2, assign54700_body0_e84394_d_n4, assign54700_body0_e84394_d_n5, assign54700_body0_e84394_d_n6, assign54700_body0_e84394_d_n7, assign54700_body0_e84394_d_n8, assign54700_body0_e84394_d_n9, assign54700_body0_e84394_d_n10, assign54700_body0_e84394_d_n11, assign54700_body0_e84394_d_n14,) = {
    if (((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_guard1379 != 0.0)) && (locals.var_guard1380 != 0.0)) {
        let assign54700_body0_e84392: f64 = (locals.var_dnm).sqrt();
        (assign54700_body0_e84392, (locals.var_dnm_dn0 / (2.0 * assign54700_body0_e84392)), (locals.var_dnm_dn2 / (2.0 * assign54700_body0_e84392)), (locals.var_dnm_dn4 / (2.0 * assign54700_body0_e84392)), (locals.var_dnm_dn5 / (2.0 * assign54700_body0_e84392)), (locals.var_dnm_dn6 / (2.0 * assign54700_body0_e84392)), (locals.var_dnm_dn7 / (2.0 * assign54700_body0_e84392)), (locals.var_dnm_dn8 / (2.0 * assign54700_body0_e84392)), (locals.var_dnm_dn9 / (2.0 * assign54700_body0_e84392)), (locals.var_dnm_dn10 / (2.0 * assign54700_body0_e84392)), (locals.var_dnm_dn11 / (2.0 * assign54700_body0_e84392)), (locals.var_dnm_dn14 / (2.0 * assign54700_body0_e84392)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
            locals.var_dnm = assign54700_body0_e84394;
            locals.var_dnm_dn0 = assign54700_body0_e84394_d_n0;
            locals.var_dnm_dn2 = assign54700_body0_e84394_d_n2;
            locals.var_dnm_dn4 = assign54700_body0_e84394_d_n4;
            locals.var_dnm_dn5 = assign54700_body0_e84394_d_n5;
            locals.var_dnm_dn6 = assign54700_body0_e84394_d_n6;
            locals.var_dnm_dn7 = assign54700_body0_e84394_d_n7;
            locals.var_dnm_dn8 = assign54700_body0_e84394_d_n8;
            locals.var_dnm_dn9 = assign54700_body0_e84394_d_n9;
            locals.var_dnm_dn10 = assign54700_body0_e84394_d_n10;
            locals.var_dnm_dn11 = assign54700_body0_e84394_d_n11;
            locals.var_dnm_dn14 = assign54700_body0_e84394_d_n14;
            let (assign54700_body1_e84414,) = {
    if (((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_guard1379 != 0.0)) && (locals.var_guard1380 != 0.0)) {
        let assign54700_body1_e84412: f64 = (locals.var_m0 + 1.0);
        (assign54700_body1_e84412,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign54700_body1_e84414;
        }

        let (assign54710_e84444, assign54710_e84444_d_n0, assign54710_e84444_d_n2, assign54710_e84444_d_n4, assign54710_e84444_d_n5, assign54710_e84444_d_n6, assign54710_e84444_d_n7, assign54710_e84444_d_n8, assign54710_e84444_d_n9, assign54710_e84444_d_n10, assign54710_e84444_d_n11, assign54710_e84444_d_n14,) = {
    if (((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_guard1379 != 0.0)) && (locals.var_guard1380 == 0.0)) {
        let (assign54710_e84442, assign54710_e84442_d_n0, assign54710_e84442_d_n2, assign54710_e84442_d_n4, assign54710_e84442_d_n5, assign54710_e84442_d_n6, assign54710_e84442_d_n7, assign54710_e84442_d_n8, assign54710_e84442_d_n9, assign54710_e84442_d_n10, assign54710_e84442_d_n11, assign54710_e84442_d_n14,) = {
            if (locals.var_dnm == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign54710_e84439: f64 = (2.0 * locals.var_vgpdep_pw__blk1147);
                let assign54710_e84440: f64 = (1.0 / assign54710_e84439);
                let assign54710_e84441: f64 = (locals.var_dnm).powf(assign54710_e84440);
                (assign54710_e84441, if 0.0 == 0.0 && ((assign54710_e84440) as f64).is_finite() && ((assign54710_e84440) as f64).fract() == 0.0 { if assign54710_e84440 == 0.0 { 0.0 } else { (assign54710_e84440 * ((locals.var_dnm).powf(assign54710_e84440 - 1.0) * locals.var_dnm_dn0)) } } else { (assign54710_e84441 * (assign54710_e84440 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign54710_e84440) as f64).is_finite() && ((assign54710_e84440) as f64).fract() == 0.0 { if assign54710_e84440 == 0.0 { 0.0 } else { (assign54710_e84440 * ((locals.var_dnm).powf(assign54710_e84440 - 1.0) * locals.var_dnm_dn2)) } } else { (assign54710_e84441 * (assign54710_e84440 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign54710_e84440) as f64).is_finite() && ((assign54710_e84440) as f64).fract() == 0.0 { if assign54710_e84440 == 0.0 { 0.0 } else { (assign54710_e84440 * ((locals.var_dnm).powf(assign54710_e84440 - 1.0) * locals.var_dnm_dn4)) } } else { (assign54710_e84441 * (assign54710_e84440 * (locals.var_dnm_dn4 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign54710_e84440) as f64).is_finite() && ((assign54710_e84440) as f64).fract() == 0.0 { if assign54710_e84440 == 0.0 { 0.0 } else { (assign54710_e84440 * ((locals.var_dnm).powf(assign54710_e84440 - 1.0) * locals.var_dnm_dn5)) } } else { (assign54710_e84441 * (assign54710_e84440 * (locals.var_dnm_dn5 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign54710_e84440) as f64).is_finite() && ((assign54710_e84440) as f64).fract() == 0.0 { if assign54710_e84440 == 0.0 { 0.0 } else { (assign54710_e84440 * ((locals.var_dnm).powf(assign54710_e84440 - 1.0) * locals.var_dnm_dn6)) } } else { (assign54710_e84441 * (assign54710_e84440 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign54710_e84440) as f64).is_finite() && ((assign54710_e84440) as f64).fract() == 0.0 { if assign54710_e84440 == 0.0 { 0.0 } else { (assign54710_e84440 * ((locals.var_dnm).powf(assign54710_e84440 - 1.0) * locals.var_dnm_dn7)) } } else { (assign54710_e84441 * (assign54710_e84440 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign54710_e84440) as f64).is_finite() && ((assign54710_e84440) as f64).fract() == 0.0 { if assign54710_e84440 == 0.0 { 0.0 } else { (assign54710_e84440 * ((locals.var_dnm).powf(assign54710_e84440 - 1.0) * locals.var_dnm_dn8)) } } else { (assign54710_e84441 * (assign54710_e84440 * (locals.var_dnm_dn8 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign54710_e84440) as f64).is_finite() && ((assign54710_e84440) as f64).fract() == 0.0 { if assign54710_e84440 == 0.0 { 0.0 } else { (assign54710_e84440 * ((locals.var_dnm).powf(assign54710_e84440 - 1.0) * locals.var_dnm_dn9)) } } else { (assign54710_e84441 * (assign54710_e84440 * (locals.var_dnm_dn9 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign54710_e84440) as f64).is_finite() && ((assign54710_e84440) as f64).fract() == 0.0 { if assign54710_e84440 == 0.0 { 0.0 } else { (assign54710_e84440 * ((locals.var_dnm).powf(assign54710_e84440 - 1.0) * locals.var_dnm_dn10)) } } else { (assign54710_e84441 * (assign54710_e84440 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign54710_e84440) as f64).is_finite() && ((assign54710_e84440) as f64).fract() == 0.0 { if assign54710_e84440 == 0.0 { 0.0 } else { (assign54710_e84440 * ((locals.var_dnm).powf(assign54710_e84440 - 1.0) * locals.var_dnm_dn11)) } } else { (assign54710_e84441 * (assign54710_e84440 * (locals.var_dnm_dn11 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign54710_e84440) as f64).is_finite() && ((assign54710_e84440) as f64).fract() == 0.0 { if assign54710_e84440 == 0.0 { 0.0 } else { (assign54710_e84440 * ((locals.var_dnm).powf(assign54710_e84440 - 1.0) * locals.var_dnm_dn14)) } } else { (assign54710_e84441 * (assign54710_e84440 * (locals.var_dnm_dn14 / locals.var_dnm))) },)
            }
        };
        (assign54710_e84442, assign54710_e84442_d_n0, assign54710_e84442_d_n2, assign54710_e84442_d_n4, assign54710_e84442_d_n5, assign54710_e84442_d_n6, assign54710_e84442_d_n7, assign54710_e84442_d_n8, assign54710_e84442_d_n9, assign54710_e84442_d_n10, assign54710_e84442_d_n11, assign54710_e84442_d_n14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign54710_e84444;
        locals.var_dnm_dn0 = assign54710_e84444_d_n0;
        locals.var_dnm_dn2 = assign54710_e84444_d_n2;
        locals.var_dnm_dn4 = assign54710_e84444_d_n4;
        locals.var_dnm_dn5 = assign54710_e84444_d_n5;
        locals.var_dnm_dn6 = assign54710_e84444_d_n6;
        locals.var_dnm_dn7 = assign54710_e84444_d_n7;
        locals.var_dnm_dn8 = assign54710_e84444_d_n8;
        locals.var_dnm_dn9 = assign54710_e84444_d_n9;
        locals.var_dnm_dn10 = assign54710_e84444_d_n10;
        locals.var_dnm_dn11 = assign54710_e84444_d_n11;
        locals.var_dnm_dn14 = assign54710_e84444_d_n14;

        let (assign54720_e84462, assign54720_e84462_d_n0, assign54720_e84462_d_n2, assign54720_e84462_d_n4, assign54720_e84462_d_n5, assign54720_e84462_d_n6, assign54720_e84462_d_n7, assign54720_e84462_d_n8, assign54720_e84462_d_n9, assign54720_e84462_d_n10, assign54720_e84462_d_n11, assign54720_e84462_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_guard1379 != 0.0)) {
        let assign54720_e84460: f64 = (1.0 / locals.var_dnm);
        (assign54720_e84460, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn4 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn5 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn8 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn9 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn11 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn14 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign54720_e84462;
        locals.var_dnm_dn0 = assign54720_e84462_d_n0;
        locals.var_dnm_dn2 = assign54720_e84462_d_n2;
        locals.var_dnm_dn4 = assign54720_e84462_d_n4;
        locals.var_dnm_dn5 = assign54720_e84462_d_n5;
        locals.var_dnm_dn6 = assign54720_e84462_d_n6;
        locals.var_dnm_dn7 = assign54720_e84462_d_n7;
        locals.var_dnm_dn8 = assign54720_e84462_d_n8;
        locals.var_dnm_dn9 = assign54720_e84462_d_n9;
        locals.var_dnm_dn10 = assign54720_e84462_d_n10;
        locals.var_dnm_dn11 = assign54720_e84462_d_n11;
        locals.var_dnm_dn14 = assign54720_e84462_d_n14;

        let (assign54730_e84482, assign54730_e84482_d_n0, assign54730_e84482_d_n2, assign54730_e84482_d_n4, assign54730_e84482_d_n5, assign54730_e84482_d_n6, assign54730_e84482_d_n7, assign54730_e84482_d_n8, assign54730_e84482_d_n9, assign54730_e84482_d_n10, assign54730_e84482_d_n11, assign54730_e84482_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_guard1379 != 0.0)) {
        let assign54730_e84478: f64 = (locals.var_tmf1 * locals.var_vgpdep_dlt__blk1146);
        let assign54730_e84480: f64 = (assign54730_e84478 * locals.var_dnm);
        (assign54730_e84480, (((locals.var_tmf1_dn0 * locals.var_vgpdep_dlt__blk1146) * locals.var_dnm) + (assign54730_e84478 * locals.var_dnm_dn0)), (((locals.var_tmf1_dn2 * locals.var_vgpdep_dlt__blk1146) * locals.var_dnm) + (assign54730_e84478 * locals.var_dnm_dn2)), (((locals.var_tmf1_dn4 * locals.var_vgpdep_dlt__blk1146) * locals.var_dnm) + (assign54730_e84478 * locals.var_dnm_dn4)), (((locals.var_tmf1_dn5 * locals.var_vgpdep_dlt__blk1146) * locals.var_dnm) + (assign54730_e84478 * locals.var_dnm_dn5)), (((locals.var_tmf1_dn6 * locals.var_vgpdep_dlt__blk1146) * locals.var_dnm) + (assign54730_e84478 * locals.var_dnm_dn6)), (((locals.var_tmf1_dn7 * locals.var_vgpdep_dlt__blk1146) * locals.var_dnm) + (assign54730_e84478 * locals.var_dnm_dn7)), (((locals.var_tmf1_dn8 * locals.var_vgpdep_dlt__blk1146) * locals.var_dnm) + (assign54730_e84478 * locals.var_dnm_dn8)), (((locals.var_tmf1_dn9 * locals.var_vgpdep_dlt__blk1146) * locals.var_dnm) + (assign54730_e84478 * locals.var_dnm_dn9)), (((locals.var_tmf1_dn10 * locals.var_vgpdep_dlt__blk1146) * locals.var_dnm) + (assign54730_e84478 * locals.var_dnm_dn10)), (((locals.var_tmf1_dn11 * locals.var_vgpdep_dlt__blk1146) * locals.var_dnm) + (assign54730_e84478 * locals.var_dnm_dn11)), (((locals.var_tmf1_dn14 * locals.var_vgpdep_dlt__blk1146) * locals.var_dnm) + (assign54730_e84478 * locals.var_dnm_dn14)),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn11, locals.var_tmf0_dn14,)
    }
};
        locals.var_tmf0 = assign54730_e84482;
        locals.var_tmf0_dn0 = assign54730_e84482_d_n0;
        locals.var_tmf0_dn2 = assign54730_e84482_d_n2;
        locals.var_tmf0_dn4 = assign54730_e84482_d_n4;
        locals.var_tmf0_dn5 = assign54730_e84482_d_n5;
        locals.var_tmf0_dn6 = assign54730_e84482_d_n6;
        locals.var_tmf0_dn7 = assign54730_e84482_d_n7;
        locals.var_tmf0_dn8 = assign54730_e84482_d_n8;
        locals.var_tmf0_dn9 = assign54730_e84482_d_n9;
        locals.var_tmf0_dn10 = assign54730_e84482_d_n10;
        locals.var_tmf0_dn11 = assign54730_e84482_d_n11;
        locals.var_tmf0_dn14 = assign54730_e84482_d_n14;

        let (assign54740_e84504, assign54740_e84504_d_n0, assign54740_e84504_d_n2, assign54740_e84504_d_n4, assign54740_e84504_d_n5, assign54740_e84504_d_n6, assign54740_e84504_d_n7, assign54740_e84504_d_n8, assign54740_e84504_d_n9, assign54740_e84504_d_n10, assign54740_e84504_d_n11, assign54740_e84504_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_guard1379 != 0.0)) {
        let assign54740_e84498: f64 = (locals.var_vgpdep_dlt__blk1146 * locals.var_xmp);
        let assign54740_e84500: f64 = (assign54740_e84498 * locals.var_dnm);
        let assign54740_e84502: f64 = (assign54740_e84500 / locals.var_arg);
        (assign54740_e84502, ((((((locals.var_vgpdep_dlt__blk1146 * locals.var_xmp_dn0) * locals.var_dnm) + (assign54740_e84498 * locals.var_dnm_dn0)) * locals.var_arg) - (assign54740_e84500 * locals.var_arg_dn0)) / (locals.var_arg * locals.var_arg)), ((((((locals.var_vgpdep_dlt__blk1146 * locals.var_xmp_dn2) * locals.var_dnm) + (assign54740_e84498 * locals.var_dnm_dn2)) * locals.var_arg) - (assign54740_e84500 * locals.var_arg_dn2)) / (locals.var_arg * locals.var_arg)), ((((((locals.var_vgpdep_dlt__blk1146 * locals.var_xmp_dn4) * locals.var_dnm) + (assign54740_e84498 * locals.var_dnm_dn4)) * locals.var_arg) - (assign54740_e84500 * locals.var_arg_dn4)) / (locals.var_arg * locals.var_arg)), ((((((locals.var_vgpdep_dlt__blk1146 * locals.var_xmp_dn5) * locals.var_dnm) + (assign54740_e84498 * locals.var_dnm_dn5)) * locals.var_arg) - (assign54740_e84500 * locals.var_arg_dn5)) / (locals.var_arg * locals.var_arg)), ((((((locals.var_vgpdep_dlt__blk1146 * locals.var_xmp_dn6) * locals.var_dnm) + (assign54740_e84498 * locals.var_dnm_dn6)) * locals.var_arg) - (assign54740_e84500 * locals.var_arg_dn6)) / (locals.var_arg * locals.var_arg)), ((((((locals.var_vgpdep_dlt__blk1146 * locals.var_xmp_dn7) * locals.var_dnm) + (assign54740_e84498 * locals.var_dnm_dn7)) * locals.var_arg) - (assign54740_e84500 * locals.var_arg_dn7)) / (locals.var_arg * locals.var_arg)), ((((((locals.var_vgpdep_dlt__blk1146 * locals.var_xmp_dn8) * locals.var_dnm) + (assign54740_e84498 * locals.var_dnm_dn8)) * locals.var_arg) - (assign54740_e84500 * locals.var_arg_dn8)) / (locals.var_arg * locals.var_arg)), ((((((locals.var_vgpdep_dlt__blk1146 * locals.var_xmp_dn9) * locals.var_dnm) + (assign54740_e84498 * locals.var_dnm_dn9)) * locals.var_arg) - (assign54740_e84500 * locals.var_arg_dn9)) / (locals.var_arg * locals.var_arg)), ((((((locals.var_vgpdep_dlt__blk1146 * locals.var_xmp_dn10) * locals.var_dnm) + (assign54740_e84498 * locals.var_dnm_dn10)) * locals.var_arg) - (assign54740_e84500 * locals.var_arg_dn10)) / (locals.var_arg * locals.var_arg)), ((((((locals.var_vgpdep_dlt__blk1146 * locals.var_xmp_dn11) * locals.var_dnm) + (assign54740_e84498 * locals.var_dnm_dn11)) * locals.var_arg) - (assign54740_e84500 * locals.var_arg_dn11)) / (locals.var_arg * locals.var_arg)), ((((((locals.var_vgpdep_dlt__blk1146 * locals.var_xmp_dn14) * locals.var_dnm) + (assign54740_e84498 * locals.var_dnm_dn14)) * locals.var_arg) - (assign54740_e84500 * locals.var_arg_dn14)) / (locals.var_arg * locals.var_arg)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign54740_e84504;
        locals.var_t0_dn0 = assign54740_e84504_d_n0;
        locals.var_t0_dn2 = assign54740_e84504_d_n2;
        locals.var_t0_dn4 = assign54740_e84504_d_n4;
        locals.var_t0_dn5 = assign54740_e84504_d_n5;
        locals.var_t0_dn6 = assign54740_e84504_d_n6;
        locals.var_t0_dn7 = assign54740_e84504_d_n7;
        locals.var_t0_dn8 = assign54740_e84504_d_n8;
        locals.var_t0_dn9 = assign54740_e84504_d_n9;
        locals.var_t0_dn10 = assign54740_e84504_d_n10;
        locals.var_t0_dn11 = assign54740_e84504_d_n11;
        locals.var_t0_dn14 = assign54740_e84504_d_n14;

        let (assign54750_e84524, assign54750_e84524_d_n0, assign54750_e84524_d_n2, assign54750_e84524_d_n4, assign54750_e84524_d_n5, assign54750_e84524_d_n6, assign54750_e84524_d_n7, assign54750_e84524_d_n8, assign54750_e84524_d_n9, assign54750_e84524_d_n10, assign54750_e84524_d_n11, assign54750_e84524_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_guard1379 != 0.0)) {
        let assign54750_e84520: f64 = (-locals.var_vgpdep_dlt__blk1146);
        let assign54750_e84522: f64 = (assign54750_e84520 + locals.var_tmf0);
        (assign54750_e84522, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn11, locals.var_tmf0_dn14,)
    } else {
        (locals.var_ps0dep, locals.var_ps0dep_dn0, locals.var_ps0dep_dn2, locals.var_ps0dep_dn4, locals.var_ps0dep_dn5, locals.var_ps0dep_dn6, locals.var_ps0dep_dn7, locals.var_ps0dep_dn8, locals.var_ps0dep_dn9, locals.var_ps0dep_dn10, locals.var_ps0dep_dn11, locals.var_ps0dep_dn14,)
    }
};
        locals.var_ps0dep = assign54750_e84524;
        locals.var_ps0dep_dn0 = assign54750_e84524_d_n0;
        locals.var_ps0dep_dn2 = assign54750_e84524_d_n2;
        locals.var_ps0dep_dn4 = assign54750_e84524_d_n4;
        locals.var_ps0dep_dn5 = assign54750_e84524_d_n5;
        locals.var_ps0dep_dn6 = assign54750_e84524_d_n6;
        locals.var_ps0dep_dn7 = assign54750_e84524_d_n7;
        locals.var_ps0dep_dn8 = assign54750_e84524_d_n8;
        locals.var_ps0dep_dn9 = assign54750_e84524_d_n9;
        locals.var_ps0dep_dn10 = assign54750_e84524_d_n10;
        locals.var_ps0dep_dn11 = assign54750_e84524_d_n11;
        locals.var_ps0dep_dn14 = assign54750_e84524_d_n14;

    }

    pub(super) fn stamp_transient_block_188(
        locals: &mut StampLocals,
    ) {
        let (assign54760_e84540, assign54760_e84540_d_n0, assign54760_e84540_d_n2, assign54760_e84540_d_n4, assign54760_e84540_d_n5, assign54760_e84540_d_n6, assign54760_e84540_d_n7, assign54760_e84540_d_n8, assign54760_e84540_d_n9, assign54760_e84540_d_n10, assign54760_e84540_d_n11, assign54760_e84540_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_guard1379 != 0.0)) {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign54760_e84540;
        locals.var_t0_dn0 = assign54760_e84540_d_n0;
        locals.var_t0_dn2 = assign54760_e84540_d_n2;
        locals.var_t0_dn4 = assign54760_e84540_d_n4;
        locals.var_t0_dn5 = assign54760_e84540_d_n5;
        locals.var_t0_dn6 = assign54760_e84540_d_n6;
        locals.var_t0_dn7 = assign54760_e84540_d_n7;
        locals.var_t0_dn8 = assign54760_e84540_d_n8;
        locals.var_t0_dn9 = assign54760_e84540_d_n9;
        locals.var_t0_dn10 = assign54760_e84540_d_n10;
        locals.var_t0_dn11 = assign54760_e84540_d_n11;
        locals.var_t0_dn14 = assign54760_e84540_d_n14;

        let (assign54770_e84557, assign54770_e84557_d_n0, assign54770_e84557_d_n2, assign54770_e84557_d_n4, assign54770_e84557_d_n5, assign54770_e84557_d_n6, assign54770_e84557_d_n7, assign54770_e84557_d_n8, assign54770_e84557_d_n9, assign54770_e84557_d_n10, assign54770_e84557_d_n11, assign54770_e84557_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_guard1379 == 0.0)) {
        (locals.var_ps0_res, locals.var_ps0_res_dn0, locals.var_ps0_res_dn2, locals.var_ps0_res_dn4, locals.var_ps0_res_dn5, locals.var_ps0_res_dn6, locals.var_ps0_res_dn7, locals.var_ps0_res_dn8, locals.var_ps0_res_dn9, locals.var_ps0_res_dn10, locals.var_ps0_res_dn11, locals.var_ps0_res_dn14,)
    } else {
        (locals.var_ps0dep, locals.var_ps0dep_dn0, locals.var_ps0dep_dn2, locals.var_ps0dep_dn4, locals.var_ps0dep_dn5, locals.var_ps0dep_dn6, locals.var_ps0dep_dn7, locals.var_ps0dep_dn8, locals.var_ps0dep_dn9, locals.var_ps0dep_dn10, locals.var_ps0dep_dn11, locals.var_ps0dep_dn14,)
    }
};
        locals.var_ps0dep = assign54770_e84557;
        locals.var_ps0dep_dn0 = assign54770_e84557_d_n0;
        locals.var_ps0dep_dn2 = assign54770_e84557_d_n2;
        locals.var_ps0dep_dn4 = assign54770_e84557_d_n4;
        locals.var_ps0dep_dn5 = assign54770_e84557_d_n5;
        locals.var_ps0dep_dn6 = assign54770_e84557_d_n6;
        locals.var_ps0dep_dn7 = assign54770_e84557_d_n7;
        locals.var_ps0dep_dn8 = assign54770_e84557_d_n8;
        locals.var_ps0dep_dn9 = assign54770_e84557_d_n9;
        locals.var_ps0dep_dn10 = assign54770_e84557_d_n10;
        locals.var_ps0dep_dn11 = assign54770_e84557_d_n11;
        locals.var_ps0dep_dn14 = assign54770_e84557_d_n14;

        let (assign54780_e84574, assign54780_e84574_d_n0, assign54780_e84574_d_n2, assign54780_e84574_d_n4, assign54780_e84574_d_n5, assign54780_e84574_d_n6, assign54780_e84574_d_n7, assign54780_e84574_d_n8, assign54780_e84574_d_n9, assign54780_e84574_d_n10, assign54780_e84574_d_n11, assign54780_e84574_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_guard1379 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign54780_e84574;
        locals.var_t0_dn0 = assign54780_e84574_d_n0;
        locals.var_t0_dn2 = assign54780_e84574_d_n2;
        locals.var_t0_dn4 = assign54780_e84574_d_n4;
        locals.var_t0_dn5 = assign54780_e84574_d_n5;
        locals.var_t0_dn6 = assign54780_e84574_d_n6;
        locals.var_t0_dn7 = assign54780_e84574_d_n7;
        locals.var_t0_dn8 = assign54780_e84574_d_n8;
        locals.var_t0_dn9 = assign54780_e84574_d_n9;
        locals.var_t0_dn10 = assign54780_e84574_d_n10;
        locals.var_t0_dn11 = assign54780_e84574_d_n11;
        locals.var_t0_dn14 = assign54780_e84574_d_n14;

        let (assign54790_e84589, assign54790_e84589_d_n0, assign54790_e84589_d_n2, assign54790_e84589_d_n4, assign54790_e84589_d_n5, assign54790_e84589_d_n6, assign54790_e84589_d_n7, assign54790_e84589_d_n8, assign54790_e84589_d_n9, assign54790_e84589_d_n10, assign54790_e84589_d_n11, assign54790_e84589_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) {
        let assign54790_e84587: f64 = (-locals.var_ps0dep);
        (assign54790_e84587, (-locals.var_ps0dep_dn0), (-locals.var_ps0dep_dn2), (-locals.var_ps0dep_dn4), (-locals.var_ps0dep_dn5), (-locals.var_ps0dep_dn6), (-locals.var_ps0dep_dn7), (-locals.var_ps0dep_dn8), (-locals.var_ps0dep_dn9), (-locals.var_ps0dep_dn10), (-locals.var_ps0dep_dn11), (-locals.var_ps0dep_dn14),)
    } else {
        (locals.var_ps0dep, locals.var_ps0dep_dn0, locals.var_ps0dep_dn2, locals.var_ps0dep_dn4, locals.var_ps0dep_dn5, locals.var_ps0dep_dn6, locals.var_ps0dep_dn7, locals.var_ps0dep_dn8, locals.var_ps0dep_dn9, locals.var_ps0dep_dn10, locals.var_ps0dep_dn11, locals.var_ps0dep_dn14,)
    }
};
        locals.var_ps0dep = assign54790_e84589;
        locals.var_ps0dep_dn0 = assign54790_e84589_d_n0;
        locals.var_ps0dep_dn2 = assign54790_e84589_d_n2;
        locals.var_ps0dep_dn4 = assign54790_e84589_d_n4;
        locals.var_ps0dep_dn5 = assign54790_e84589_d_n5;
        locals.var_ps0dep_dn6 = assign54790_e84589_d_n6;
        locals.var_ps0dep_dn7 = assign54790_e84589_d_n7;
        locals.var_ps0dep_dn8 = assign54790_e84589_d_n8;
        locals.var_ps0dep_dn9 = assign54790_e84589_d_n9;
        locals.var_ps0dep_dn10 = assign54790_e84589_d_n10;
        locals.var_ps0dep_dn11 = assign54790_e84589_d_n11;
        locals.var_ps0dep_dn14 = assign54790_e84589_d_n14;

        let (assign54800_e84611, assign54800_e84611_d_n0, assign54800_e84611_d_n2, assign54800_e84611_d_n4, assign54800_e84611_d_n5, assign54800_e84611_d_n6, assign54800_e84611_d_n7, assign54800_e84611_d_n8, assign54800_e84611_d_n9, assign54800_e84611_d_n10, assign54800_e84611_d_n11, assign54800_e84611_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) {
        let assign54800_e84603: f64 = (locals.var_q_ndepm__blk1137 * locals.var_tnp__blk1152);
        let assign54800_e84605: f64 = (assign54800_e84603 * locals.var_tnp__blk1152);
        let assign54800_e84607: f64 = (assign54800_e84605 / 2.0);
        let assign54800_e84609: f64 = (assign54800_e84607 / 1.034943e-10);
        (assign54800_e84609, ((((((locals.var_q_ndepm__blk1137_dn0 * locals.var_tnp__blk1152) + (locals.var_q_ndepm__blk1137 * locals.var_tnp__blk1152_dn0)) * locals.var_tnp__blk1152) + (assign54800_e84603 * locals.var_tnp__blk1152_dn0)) / 2.0) / 1.034943e-10), ((((((locals.var_q_ndepm__blk1137_dn2 * locals.var_tnp__blk1152) + (locals.var_q_ndepm__blk1137 * locals.var_tnp__blk1152_dn2)) * locals.var_tnp__blk1152) + (assign54800_e84603 * locals.var_tnp__blk1152_dn2)) / 2.0) / 1.034943e-10), ((((((locals.var_q_ndepm__blk1137_dn4 * locals.var_tnp__blk1152) + (locals.var_q_ndepm__blk1137 * locals.var_tnp__blk1152_dn4)) * locals.var_tnp__blk1152) + (assign54800_e84603 * locals.var_tnp__blk1152_dn4)) / 2.0) / 1.034943e-10), ((((((locals.var_q_ndepm__blk1137_dn5 * locals.var_tnp__blk1152) + (locals.var_q_ndepm__blk1137 * locals.var_tnp__blk1152_dn5)) * locals.var_tnp__blk1152) + (assign54800_e84603 * locals.var_tnp__blk1152_dn5)) / 2.0) / 1.034943e-10), ((((((locals.var_q_ndepm__blk1137_dn6 * locals.var_tnp__blk1152) + (locals.var_q_ndepm__blk1137 * locals.var_tnp__blk1152_dn6)) * locals.var_tnp__blk1152) + (assign54800_e84603 * locals.var_tnp__blk1152_dn6)) / 2.0) / 1.034943e-10), ((((((locals.var_q_ndepm__blk1137_dn7 * locals.var_tnp__blk1152) + (locals.var_q_ndepm__blk1137 * locals.var_tnp__blk1152_dn7)) * locals.var_tnp__blk1152) + (assign54800_e84603 * locals.var_tnp__blk1152_dn7)) / 2.0) / 1.034943e-10), ((((((locals.var_q_ndepm__blk1137_dn8 * locals.var_tnp__blk1152) + (locals.var_q_ndepm__blk1137 * locals.var_tnp__blk1152_dn8)) * locals.var_tnp__blk1152) + (assign54800_e84603 * locals.var_tnp__blk1152_dn8)) / 2.0) / 1.034943e-10), ((((((locals.var_q_ndepm__blk1137_dn9 * locals.var_tnp__blk1152) + (locals.var_q_ndepm__blk1137 * locals.var_tnp__blk1152_dn9)) * locals.var_tnp__blk1152) + (assign54800_e84603 * locals.var_tnp__blk1152_dn9)) / 2.0) / 1.034943e-10), ((((((locals.var_q_ndepm__blk1137_dn10 * locals.var_tnp__blk1152) + (locals.var_q_ndepm__blk1137 * locals.var_tnp__blk1152_dn10)) * locals.var_tnp__blk1152) + (assign54800_e84603 * locals.var_tnp__blk1152_dn10)) / 2.0) / 1.034943e-10), ((((((locals.var_q_ndepm__blk1137_dn11 * locals.var_tnp__blk1152) + (locals.var_q_ndepm__blk1137 * locals.var_tnp__blk1152_dn11)) * locals.var_tnp__blk1152) + (assign54800_e84603 * locals.var_tnp__blk1152_dn11)) / 2.0) / 1.034943e-10), ((((((locals.var_q_ndepm__blk1137_dn14 * locals.var_tnp__blk1152) + (locals.var_q_ndepm__blk1137 * locals.var_tnp__blk1152_dn14)) * locals.var_tnp__blk1152) + (assign54800_e84603 * locals.var_tnp__blk1152_dn14)) / 2.0) / 1.034943e-10),)
    } else {
        (locals.var_dphi_sb__blk1324, locals.var_dphi_sb__blk1324_dn0, locals.var_dphi_sb__blk1324_dn2, locals.var_dphi_sb__blk1324_dn4, locals.var_dphi_sb__blk1324_dn5, locals.var_dphi_sb__blk1324_dn6, locals.var_dphi_sb__blk1324_dn7, locals.var_dphi_sb__blk1324_dn8, locals.var_dphi_sb__blk1324_dn9, locals.var_dphi_sb__blk1324_dn10, locals.var_dphi_sb__blk1324_dn11, locals.var_dphi_sb__blk1324_dn14,)
    }
};
        locals.var_dphi_sb__blk1324 = assign54800_e84611;
        locals.var_dphi_sb__blk1324_dn0 = assign54800_e84611_d_n0;
        locals.var_dphi_sb__blk1324_dn2 = assign54800_e84611_d_n2;
        locals.var_dphi_sb__blk1324_dn4 = assign54800_e84611_d_n4;
        locals.var_dphi_sb__blk1324_dn5 = assign54800_e84611_d_n5;
        locals.var_dphi_sb__blk1324_dn6 = assign54800_e84611_d_n6;
        locals.var_dphi_sb__blk1324_dn7 = assign54800_e84611_d_n7;
        locals.var_dphi_sb__blk1324_dn8 = assign54800_e84611_d_n8;
        locals.var_dphi_sb__blk1324_dn9 = assign54800_e84611_d_n9;
        locals.var_dphi_sb__blk1324_dn10 = assign54800_e84611_d_n10;
        locals.var_dphi_sb__blk1324_dn11 = assign54800_e84611_d_n11;
        locals.var_dphi_sb__blk1324_dn14 = assign54800_e84611_d_n14;

        let (assign54810_e84632, assign54810_e84632_d_n0, assign54810_e84632_d_n2, assign54810_e84632_d_n4, assign54810_e84632_d_n5, assign54810_e84632_d_n6, assign54810_e84632_d_n7, assign54810_e84632_d_n8, assign54810_e84632_d_n9, assign54810_e84632_d_n10, assign54810_e84632_d_n11, assign54810_e84632_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) {
        let assign54810_e84626: f64 = (2.0 * locals.var_beta);
        let assign54810_e84628: f64 = (assign54810_e84626 * locals.var_dphi_sb__blk1324);
        let assign54810_e84629: f64 = (assign54810_e84628).sqrt();
        let assign54810_e84630: f64 = (locals.var_wdepsubsl * assign54810_e84629);
        (assign54810_e84630, ((locals.var_wdepsubsl_dn0 * assign54810_e84629) + (locals.var_wdepsubsl * ((((2.0 * locals.var_beta_dn0) * locals.var_dphi_sb__blk1324) + (assign54810_e84626 * locals.var_dphi_sb__blk1324_dn0)) / (2.0 * assign54810_e84629)))), ((locals.var_wdepsubsl_dn2 * assign54810_e84629) + (locals.var_wdepsubsl * ((((2.0 * locals.var_beta_dn2) * locals.var_dphi_sb__blk1324) + (assign54810_e84626 * locals.var_dphi_sb__blk1324_dn2)) / (2.0 * assign54810_e84629)))), ((locals.var_wdepsubsl_dn4 * assign54810_e84629) + (locals.var_wdepsubsl * ((((2.0 * locals.var_beta_dn4) * locals.var_dphi_sb__blk1324) + (assign54810_e84626 * locals.var_dphi_sb__blk1324_dn4)) / (2.0 * assign54810_e84629)))), ((locals.var_wdepsubsl_dn5 * assign54810_e84629) + (locals.var_wdepsubsl * ((((2.0 * locals.var_beta_dn5) * locals.var_dphi_sb__blk1324) + (assign54810_e84626 * locals.var_dphi_sb__blk1324_dn5)) / (2.0 * assign54810_e84629)))), ((locals.var_wdepsubsl_dn6 * assign54810_e84629) + (locals.var_wdepsubsl * ((((2.0 * locals.var_beta_dn6) * locals.var_dphi_sb__blk1324) + (assign54810_e84626 * locals.var_dphi_sb__blk1324_dn6)) / (2.0 * assign54810_e84629)))), ((locals.var_wdepsubsl_dn7 * assign54810_e84629) + (locals.var_wdepsubsl * ((((2.0 * locals.var_beta_dn7) * locals.var_dphi_sb__blk1324) + (assign54810_e84626 * locals.var_dphi_sb__blk1324_dn7)) / (2.0 * assign54810_e84629)))), ((locals.var_wdepsubsl_dn8 * assign54810_e84629) + (locals.var_wdepsubsl * ((((2.0 * locals.var_beta_dn8) * locals.var_dphi_sb__blk1324) + (assign54810_e84626 * locals.var_dphi_sb__blk1324_dn8)) / (2.0 * assign54810_e84629)))), ((locals.var_wdepsubsl_dn9 * assign54810_e84629) + (locals.var_wdepsubsl * ((((2.0 * locals.var_beta_dn9) * locals.var_dphi_sb__blk1324) + (assign54810_e84626 * locals.var_dphi_sb__blk1324_dn9)) / (2.0 * assign54810_e84629)))), ((locals.var_wdepsubsl_dn10 * assign54810_e84629) + (locals.var_wdepsubsl * ((((2.0 * locals.var_beta_dn10) * locals.var_dphi_sb__blk1324) + (assign54810_e84626 * locals.var_dphi_sb__blk1324_dn10)) / (2.0 * assign54810_e84629)))), ((locals.var_wdepsubsl_dn11 * assign54810_e84629) + (locals.var_wdepsubsl * ((((2.0 * locals.var_beta_dn11) * locals.var_dphi_sb__blk1324) + (assign54810_e84626 * locals.var_dphi_sb__blk1324_dn11)) / (2.0 * assign54810_e84629)))), ((locals.var_wdepsubsl_dn14 * assign54810_e84629) + (locals.var_wdepsubsl * ((((2.0 * locals.var_beta_dn14) * locals.var_dphi_sb__blk1324) + (assign54810_e84626 * locals.var_dphi_sb__blk1324_dn14)) / (2.0 * assign54810_e84629)))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign54810_e84632;
        locals.var_t0_dn0 = assign54810_e84632_d_n0;
        locals.var_t0_dn2 = assign54810_e84632_d_n2;
        locals.var_t0_dn4 = assign54810_e84632_d_n4;
        locals.var_t0_dn5 = assign54810_e84632_d_n5;
        locals.var_t0_dn6 = assign54810_e84632_d_n6;
        locals.var_t0_dn7 = assign54810_e84632_d_n7;
        locals.var_t0_dn8 = assign54810_e84632_d_n8;
        locals.var_t0_dn9 = assign54810_e84632_d_n9;
        locals.var_t0_dn10 = assign54810_e84632_d_n10;
        locals.var_t0_dn11 = assign54810_e84632_d_n11;
        locals.var_t0_dn14 = assign54810_e84632_d_n14;

        let (assign54820_e84653, assign54820_e84653_d_n0, assign54820_e84653_d_n2, assign54820_e84653_d_n4, assign54820_e84653_d_n5, assign54820_e84653_d_n6, assign54820_e84653_d_n7, assign54820_e84653_d_n8, assign54820_e84653_d_n9, assign54820_e84653_d_n10, assign54820_e84653_d_n11, assign54820_e84653_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) {
        let assign54820_e84645: f64 = (locals.var_t0).exp();
        let assign54820_e84647: f64 = (-locals.var_t0);
        let assign54820_e84648: f64 = (assign54820_e84647).exp();
        let assign54820_e84649: f64 = (assign54820_e84645 + assign54820_e84648);
        let assign54820_e84651: f64 = (assign54820_e84649 / 2.0);
        (assign54820_e84651, (((assign54820_e84645 * locals.var_t0_dn0) + (assign54820_e84648 * (-locals.var_t0_dn0))) / 2.0), (((assign54820_e84645 * locals.var_t0_dn2) + (assign54820_e84648 * (-locals.var_t0_dn2))) / 2.0), (((assign54820_e84645 * locals.var_t0_dn4) + (assign54820_e84648 * (-locals.var_t0_dn4))) / 2.0), (((assign54820_e84645 * locals.var_t0_dn5) + (assign54820_e84648 * (-locals.var_t0_dn5))) / 2.0), (((assign54820_e84645 * locals.var_t0_dn6) + (assign54820_e84648 * (-locals.var_t0_dn6))) / 2.0), (((assign54820_e84645 * locals.var_t0_dn7) + (assign54820_e84648 * (-locals.var_t0_dn7))) / 2.0), (((assign54820_e84645 * locals.var_t0_dn8) + (assign54820_e84648 * (-locals.var_t0_dn8))) / 2.0), (((assign54820_e84645 * locals.var_t0_dn9) + (assign54820_e84648 * (-locals.var_t0_dn9))) / 2.0), (((assign54820_e84645 * locals.var_t0_dn10) + (assign54820_e84648 * (-locals.var_t0_dn10))) / 2.0), (((assign54820_e84645 * locals.var_t0_dn11) + (assign54820_e84648 * (-locals.var_t0_dn11))) / 2.0), (((assign54820_e84645 * locals.var_t0_dn14) + (assign54820_e84648 * (-locals.var_t0_dn14))) / 2.0),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign54820_e84653;
        locals.var_t1_dn0 = assign54820_e84653_d_n0;
        locals.var_t1_dn2 = assign54820_e84653_d_n2;
        locals.var_t1_dn4 = assign54820_e84653_d_n4;
        locals.var_t1_dn5 = assign54820_e84653_d_n5;
        locals.var_t1_dn6 = assign54820_e84653_d_n6;
        locals.var_t1_dn7 = assign54820_e84653_d_n7;
        locals.var_t1_dn8 = assign54820_e84653_d_n8;
        locals.var_t1_dn9 = assign54820_e84653_d_n9;
        locals.var_t1_dn10 = assign54820_e84653_d_n10;
        locals.var_t1_dn11 = assign54820_e84653_d_n11;
        locals.var_t1_dn14 = assign54820_e84653_d_n14;

        let assign54830_e84655: f64 = (locals.var_t0).abs();
        let assign54830_e84657: f64 = if assign54830_e84655 > 0.0001 { 1.0 } else { 0.0 };
        locals.var_guard1385 = assign54830_e84657;

        let (assign54840_e84676, assign54840_e84676_d_n0, assign54840_e84676_d_n2, assign54840_e84676_d_n4, assign54840_e84676_d_n5, assign54840_e84676_d_n6, assign54840_e84676_d_n7, assign54840_e84676_d_n8, assign54840_e84676_d_n9, assign54840_e84676_d_n10, assign54840_e84676_d_n11, assign54840_e84676_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_guard1385 != 0.0)) {
        let assign54840_e84672: f64 = (locals.var_t1).ln();
        let assign54840_e84674: f64 = (assign54840_e84672 / locals.var_dphi_sb__blk1324);
        (assign54840_e84674, ((((locals.var_t1_dn0 / locals.var_t1) * locals.var_dphi_sb__blk1324) - (assign54840_e84672 * locals.var_dphi_sb__blk1324_dn0)) / (locals.var_dphi_sb__blk1324 * locals.var_dphi_sb__blk1324)), ((((locals.var_t1_dn2 / locals.var_t1) * locals.var_dphi_sb__blk1324) - (assign54840_e84672 * locals.var_dphi_sb__blk1324_dn2)) / (locals.var_dphi_sb__blk1324 * locals.var_dphi_sb__blk1324)), ((((locals.var_t1_dn4 / locals.var_t1) * locals.var_dphi_sb__blk1324) - (assign54840_e84672 * locals.var_dphi_sb__blk1324_dn4)) / (locals.var_dphi_sb__blk1324 * locals.var_dphi_sb__blk1324)), ((((locals.var_t1_dn5 / locals.var_t1) * locals.var_dphi_sb__blk1324) - (assign54840_e84672 * locals.var_dphi_sb__blk1324_dn5)) / (locals.var_dphi_sb__blk1324 * locals.var_dphi_sb__blk1324)), ((((locals.var_t1_dn6 / locals.var_t1) * locals.var_dphi_sb__blk1324) - (assign54840_e84672 * locals.var_dphi_sb__blk1324_dn6)) / (locals.var_dphi_sb__blk1324 * locals.var_dphi_sb__blk1324)), ((((locals.var_t1_dn7 / locals.var_t1) * locals.var_dphi_sb__blk1324) - (assign54840_e84672 * locals.var_dphi_sb__blk1324_dn7)) / (locals.var_dphi_sb__blk1324 * locals.var_dphi_sb__blk1324)), ((((locals.var_t1_dn8 / locals.var_t1) * locals.var_dphi_sb__blk1324) - (assign54840_e84672 * locals.var_dphi_sb__blk1324_dn8)) / (locals.var_dphi_sb__blk1324 * locals.var_dphi_sb__blk1324)), ((((locals.var_t1_dn9 / locals.var_t1) * locals.var_dphi_sb__blk1324) - (assign54840_e84672 * locals.var_dphi_sb__blk1324_dn9)) / (locals.var_dphi_sb__blk1324 * locals.var_dphi_sb__blk1324)), ((((locals.var_t1_dn10 / locals.var_t1) * locals.var_dphi_sb__blk1324) - (assign54840_e84672 * locals.var_dphi_sb__blk1324_dn10)) / (locals.var_dphi_sb__blk1324 * locals.var_dphi_sb__blk1324)), ((((locals.var_t1_dn11 / locals.var_t1) * locals.var_dphi_sb__blk1324) - (assign54840_e84672 * locals.var_dphi_sb__blk1324_dn11)) / (locals.var_dphi_sb__blk1324 * locals.var_dphi_sb__blk1324)), ((((locals.var_t1_dn14 / locals.var_t1) * locals.var_dphi_sb__blk1324) - (assign54840_e84672 * locals.var_dphi_sb__blk1324_dn14)) / (locals.var_dphi_sb__blk1324 * locals.var_dphi_sb__blk1324)),)
    } else {
        (locals.var_c_sb__blk1325, locals.var_c_sb__blk1325_dn0, locals.var_c_sb__blk1325_dn2, locals.var_c_sb__blk1325_dn4, locals.var_c_sb__blk1325_dn5, locals.var_c_sb__blk1325_dn6, locals.var_c_sb__blk1325_dn7, locals.var_c_sb__blk1325_dn8, locals.var_c_sb__blk1325_dn9, locals.var_c_sb__blk1325_dn10, locals.var_c_sb__blk1325_dn11, locals.var_c_sb__blk1325_dn14,)
    }
};
        locals.var_c_sb__blk1325 = assign54840_e84676;
        locals.var_c_sb__blk1325_dn0 = assign54840_e84676_d_n0;
        locals.var_c_sb__blk1325_dn2 = assign54840_e84676_d_n2;
        locals.var_c_sb__blk1325_dn4 = assign54840_e84676_d_n4;
        locals.var_c_sb__blk1325_dn5 = assign54840_e84676_d_n5;
        locals.var_c_sb__blk1325_dn6 = assign54840_e84676_d_n6;
        locals.var_c_sb__blk1325_dn7 = assign54840_e84676_d_n7;
        locals.var_c_sb__blk1325_dn8 = assign54840_e84676_d_n8;
        locals.var_c_sb__blk1325_dn9 = assign54840_e84676_d_n9;
        locals.var_c_sb__blk1325_dn10 = assign54840_e84676_d_n10;
        locals.var_c_sb__blk1325_dn11 = assign54840_e84676_d_n11;
        locals.var_c_sb__blk1325_dn14 = assign54840_e84676_d_n14;

        let (assign54850_e84705, assign54850_e84705_d_n0, assign54850_e84705_d_n2, assign54850_e84705_d_n4, assign54850_e84705_d_n5, assign54850_e84705_d_n6, assign54850_e84705_d_n7, assign54850_e84705_d_n8, assign54850_e84705_d_n9, assign54850_e84705_d_n10, assign54850_e84705_d_n11, assign54850_e84705_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_guard1385 == 0.0)) {
        let assign54850_e84693: f64 = (locals.var_wdepsubsl * locals.var_wdepsubsl);
        let assign54850_e84695: f64 = (assign54850_e84693 * locals.var_beta);
        let assign54850_e84699: f64 = (0.1666666666666667 * locals.var_t0);
        let assign54850_e84701: f64 = (assign54850_e84699 * locals.var_t0);
        let assign54850_e84702: f64 = (1.0 - assign54850_e84701);
        let assign54850_e84703: f64 = (assign54850_e84695 * assign54850_e84702);
        (assign54850_e84703, ((((((locals.var_wdepsubsl_dn0 * locals.var_wdepsubsl) + (locals.var_wdepsubsl * locals.var_wdepsubsl_dn0)) * locals.var_beta) + (assign54850_e84693 * locals.var_beta_dn0)) * assign54850_e84702) + (assign54850_e84695 * (-(((0.1666666666666667 * locals.var_t0_dn0) * locals.var_t0) + (assign54850_e84699 * locals.var_t0_dn0))))), ((((((locals.var_wdepsubsl_dn2 * locals.var_wdepsubsl) + (locals.var_wdepsubsl * locals.var_wdepsubsl_dn2)) * locals.var_beta) + (assign54850_e84693 * locals.var_beta_dn2)) * assign54850_e84702) + (assign54850_e84695 * (-(((0.1666666666666667 * locals.var_t0_dn2) * locals.var_t0) + (assign54850_e84699 * locals.var_t0_dn2))))), ((((((locals.var_wdepsubsl_dn4 * locals.var_wdepsubsl) + (locals.var_wdepsubsl * locals.var_wdepsubsl_dn4)) * locals.var_beta) + (assign54850_e84693 * locals.var_beta_dn4)) * assign54850_e84702) + (assign54850_e84695 * (-(((0.1666666666666667 * locals.var_t0_dn4) * locals.var_t0) + (assign54850_e84699 * locals.var_t0_dn4))))), ((((((locals.var_wdepsubsl_dn5 * locals.var_wdepsubsl) + (locals.var_wdepsubsl * locals.var_wdepsubsl_dn5)) * locals.var_beta) + (assign54850_e84693 * locals.var_beta_dn5)) * assign54850_e84702) + (assign54850_e84695 * (-(((0.1666666666666667 * locals.var_t0_dn5) * locals.var_t0) + (assign54850_e84699 * locals.var_t0_dn5))))), ((((((locals.var_wdepsubsl_dn6 * locals.var_wdepsubsl) + (locals.var_wdepsubsl * locals.var_wdepsubsl_dn6)) * locals.var_beta) + (assign54850_e84693 * locals.var_beta_dn6)) * assign54850_e84702) + (assign54850_e84695 * (-(((0.1666666666666667 * locals.var_t0_dn6) * locals.var_t0) + (assign54850_e84699 * locals.var_t0_dn6))))), ((((((locals.var_wdepsubsl_dn7 * locals.var_wdepsubsl) + (locals.var_wdepsubsl * locals.var_wdepsubsl_dn7)) * locals.var_beta) + (assign54850_e84693 * locals.var_beta_dn7)) * assign54850_e84702) + (assign54850_e84695 * (-(((0.1666666666666667 * locals.var_t0_dn7) * locals.var_t0) + (assign54850_e84699 * locals.var_t0_dn7))))), ((((((locals.var_wdepsubsl_dn8 * locals.var_wdepsubsl) + (locals.var_wdepsubsl * locals.var_wdepsubsl_dn8)) * locals.var_beta) + (assign54850_e84693 * locals.var_beta_dn8)) * assign54850_e84702) + (assign54850_e84695 * (-(((0.1666666666666667 * locals.var_t0_dn8) * locals.var_t0) + (assign54850_e84699 * locals.var_t0_dn8))))), ((((((locals.var_wdepsubsl_dn9 * locals.var_wdepsubsl) + (locals.var_wdepsubsl * locals.var_wdepsubsl_dn9)) * locals.var_beta) + (assign54850_e84693 * locals.var_beta_dn9)) * assign54850_e84702) + (assign54850_e84695 * (-(((0.1666666666666667 * locals.var_t0_dn9) * locals.var_t0) + (assign54850_e84699 * locals.var_t0_dn9))))), ((((((locals.var_wdepsubsl_dn10 * locals.var_wdepsubsl) + (locals.var_wdepsubsl * locals.var_wdepsubsl_dn10)) * locals.var_beta) + (assign54850_e84693 * locals.var_beta_dn10)) * assign54850_e84702) + (assign54850_e84695 * (-(((0.1666666666666667 * locals.var_t0_dn10) * locals.var_t0) + (assign54850_e84699 * locals.var_t0_dn10))))), ((((((locals.var_wdepsubsl_dn11 * locals.var_wdepsubsl) + (locals.var_wdepsubsl * locals.var_wdepsubsl_dn11)) * locals.var_beta) + (assign54850_e84693 * locals.var_beta_dn11)) * assign54850_e84702) + (assign54850_e84695 * (-(((0.1666666666666667 * locals.var_t0_dn11) * locals.var_t0) + (assign54850_e84699 * locals.var_t0_dn11))))), ((((((locals.var_wdepsubsl_dn14 * locals.var_wdepsubsl) + (locals.var_wdepsubsl * locals.var_wdepsubsl_dn14)) * locals.var_beta) + (assign54850_e84693 * locals.var_beta_dn14)) * assign54850_e84702) + (assign54850_e84695 * (-(((0.1666666666666667 * locals.var_t0_dn14) * locals.var_t0) + (assign54850_e84699 * locals.var_t0_dn14))))),)
    } else {
        (locals.var_c_sb__blk1325, locals.var_c_sb__blk1325_dn0, locals.var_c_sb__blk1325_dn2, locals.var_c_sb__blk1325_dn4, locals.var_c_sb__blk1325_dn5, locals.var_c_sb__blk1325_dn6, locals.var_c_sb__blk1325_dn7, locals.var_c_sb__blk1325_dn8, locals.var_c_sb__blk1325_dn9, locals.var_c_sb__blk1325_dn10, locals.var_c_sb__blk1325_dn11, locals.var_c_sb__blk1325_dn14,)
    }
};
        locals.var_c_sb__blk1325 = assign54850_e84705;
        locals.var_c_sb__blk1325_dn0 = assign54850_e84705_d_n0;
        locals.var_c_sb__blk1325_dn2 = assign54850_e84705_d_n2;
        locals.var_c_sb__blk1325_dn4 = assign54850_e84705_d_n4;
        locals.var_c_sb__blk1325_dn5 = assign54850_e84705_d_n5;
        locals.var_c_sb__blk1325_dn6 = assign54850_e84705_d_n6;
        locals.var_c_sb__blk1325_dn7 = assign54850_e84705_d_n7;
        locals.var_c_sb__blk1325_dn8 = assign54850_e84705_d_n8;
        locals.var_c_sb__blk1325_dn9 = assign54850_e84705_d_n9;
        locals.var_c_sb__blk1325_dn10 = assign54850_e84705_d_n10;
        locals.var_c_sb__blk1325_dn11 = assign54850_e84705_d_n11;
        locals.var_c_sb__blk1325_dn14 = assign54850_e84705_d_n14;

        let (assign54860_e84721, assign54860_e84721_d_n0, assign54860_e84721_d_n2, assign54860_e84721_d_n4, assign54860_e84721_d_n5, assign54860_e84721_d_n6, assign54860_e84721_d_n7, assign54860_e84721_d_n8, assign54860_e84721_d_n9, assign54860_e84721_d_n10, assign54860_e84721_d_n11, assign54860_e84721_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) {
        let assign54860_e84719: f64 = (locals.var_c_sb__blk1325 * locals.var_ps0dep);
        (assign54860_e84719, ((locals.var_c_sb__blk1325_dn0 * locals.var_ps0dep) + (locals.var_c_sb__blk1325 * locals.var_ps0dep_dn0)), ((locals.var_c_sb__blk1325_dn2 * locals.var_ps0dep) + (locals.var_c_sb__blk1325 * locals.var_ps0dep_dn2)), ((locals.var_c_sb__blk1325_dn4 * locals.var_ps0dep) + (locals.var_c_sb__blk1325 * locals.var_ps0dep_dn4)), ((locals.var_c_sb__blk1325_dn5 * locals.var_ps0dep) + (locals.var_c_sb__blk1325 * locals.var_ps0dep_dn5)), ((locals.var_c_sb__blk1325_dn6 * locals.var_ps0dep) + (locals.var_c_sb__blk1325 * locals.var_ps0dep_dn6)), ((locals.var_c_sb__blk1325_dn7 * locals.var_ps0dep) + (locals.var_c_sb__blk1325 * locals.var_ps0dep_dn7)), ((locals.var_c_sb__blk1325_dn8 * locals.var_ps0dep) + (locals.var_c_sb__blk1325 * locals.var_ps0dep_dn8)), ((locals.var_c_sb__blk1325_dn9 * locals.var_ps0dep) + (locals.var_c_sb__blk1325 * locals.var_ps0dep_dn9)), ((locals.var_c_sb__blk1325_dn10 * locals.var_ps0dep) + (locals.var_c_sb__blk1325 * locals.var_ps0dep_dn10)), ((locals.var_c_sb__blk1325_dn11 * locals.var_ps0dep) + (locals.var_c_sb__blk1325 * locals.var_ps0dep_dn11)), ((locals.var_c_sb__blk1325_dn14 * locals.var_ps0dep) + (locals.var_c_sb__blk1325 * locals.var_ps0dep_dn14)),)
    } else {
        (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn4, locals.var_tx_dn5, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn8, locals.var_tx_dn9, locals.var_tx_dn10, locals.var_tx_dn11, locals.var_tx_dn14,)
    }
};
        locals.var_tx = assign54860_e84721;
        locals.var_tx_dn0 = assign54860_e84721_d_n0;
        locals.var_tx_dn2 = assign54860_e84721_d_n2;
        locals.var_tx_dn4 = assign54860_e84721_d_n4;
        locals.var_tx_dn5 = assign54860_e84721_d_n5;
        locals.var_tx_dn6 = assign54860_e84721_d_n6;
        locals.var_tx_dn7 = assign54860_e84721_d_n7;
        locals.var_tx_dn8 = assign54860_e84721_d_n8;
        locals.var_tx_dn9 = assign54860_e84721_d_n9;
        locals.var_tx_dn10 = assign54860_e84721_d_n10;
        locals.var_tx_dn11 = assign54860_e84721_d_n11;
        locals.var_tx_dn14 = assign54860_e84721_d_n14;

        let assign54870_e84724: f64 = if locals.var_tx > 500.0 { 1.0 } else { 0.0 };
        locals.var_guard1386 = assign54870_e84724;

        let (assign54880_e84742, assign54880_e84742_d_n0, assign54880_e84742_d_n2, assign54880_e84742_d_n4, assign54880_e84742_d_n5, assign54880_e84742_d_n6, assign54880_e84742_d_n7, assign54880_e84742_d_n8, assign54880_e84742_d_n9, assign54880_e84742_d_n10, assign54880_e84742_d_n11, assign54880_e84742_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_guard1386 != 0.0)) {
        let assign54880_e84740: f64 = (locals.var_ps0dep - locals.var_dphi_sb__blk1324);
        (assign54880_e84740, (locals.var_ps0dep_dn0 - locals.var_dphi_sb__blk1324_dn0), (locals.var_ps0dep_dn2 - locals.var_dphi_sb__blk1324_dn2), (locals.var_ps0dep_dn4 - locals.var_dphi_sb__blk1324_dn4), (locals.var_ps0dep_dn5 - locals.var_dphi_sb__blk1324_dn5), (locals.var_ps0dep_dn6 - locals.var_dphi_sb__blk1324_dn6), (locals.var_ps0dep_dn7 - locals.var_dphi_sb__blk1324_dn7), (locals.var_ps0dep_dn8 - locals.var_dphi_sb__blk1324_dn8), (locals.var_ps0dep_dn9 - locals.var_dphi_sb__blk1324_dn9), (locals.var_ps0dep_dn10 - locals.var_dphi_sb__blk1324_dn10), (locals.var_ps0dep_dn11 - locals.var_dphi_sb__blk1324_dn11), (locals.var_ps0dep_dn14 - locals.var_dphi_sb__blk1324_dn14),)
    } else {
        (locals.var_pb0dep__blk1169, locals.var_pb0dep__blk1169_dn0, locals.var_pb0dep__blk1169_dn2, locals.var_pb0dep__blk1169_dn4, locals.var_pb0dep__blk1169_dn5, locals.var_pb0dep__blk1169_dn6, locals.var_pb0dep__blk1169_dn7, locals.var_pb0dep__blk1169_dn8, locals.var_pb0dep__blk1169_dn9, locals.var_pb0dep__blk1169_dn10, locals.var_pb0dep__blk1169_dn11, locals.var_pb0dep__blk1169_dn14,)
    }
};
        locals.var_pb0dep__blk1169 = assign54880_e84742;
        locals.var_pb0dep__blk1169_dn0 = assign54880_e84742_d_n0;
        locals.var_pb0dep__blk1169_dn2 = assign54880_e84742_d_n2;
        locals.var_pb0dep__blk1169_dn4 = assign54880_e84742_d_n4;
        locals.var_pb0dep__blk1169_dn5 = assign54880_e84742_d_n5;
        locals.var_pb0dep__blk1169_dn6 = assign54880_e84742_d_n6;
        locals.var_pb0dep__blk1169_dn7 = assign54880_e84742_d_n7;
        locals.var_pb0dep__blk1169_dn8 = assign54880_e84742_d_n8;
        locals.var_pb0dep__blk1169_dn9 = assign54880_e84742_d_n9;
        locals.var_pb0dep__blk1169_dn10 = assign54880_e84742_d_n10;
        locals.var_pb0dep__blk1169_dn11 = assign54880_e84742_d_n11;
        locals.var_pb0dep__blk1169_dn14 = assign54880_e84742_d_n14;

        let (assign54890_e84763, assign54890_e84763_d_n0, assign54890_e84763_d_n2, assign54890_e84763_d_n4, assign54890_e84763_d_n5, assign54890_e84763_d_n6, assign54890_e84763_d_n7, assign54890_e84763_d_n8, assign54890_e84763_d_n9, assign54890_e84763_d_n10, assign54890_e84763_d_n11, assign54890_e84763_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_guard1386 == 0.0)) {
        let assign54890_e84758: f64 = (-locals.var_c_sb__blk1325);
        let assign54890_e84760: f64 = (assign54890_e84758 * locals.var_dphi_sb__blk1324);
        let assign54890_e84761: f64 = (assign54890_e84760).exp();
        (assign54890_e84761, (assign54890_e84761 * (((-locals.var_c_sb__blk1325_dn0) * locals.var_dphi_sb__blk1324) + (assign54890_e84758 * locals.var_dphi_sb__blk1324_dn0))), (assign54890_e84761 * (((-locals.var_c_sb__blk1325_dn2) * locals.var_dphi_sb__blk1324) + (assign54890_e84758 * locals.var_dphi_sb__blk1324_dn2))), (assign54890_e84761 * (((-locals.var_c_sb__blk1325_dn4) * locals.var_dphi_sb__blk1324) + (assign54890_e84758 * locals.var_dphi_sb__blk1324_dn4))), (assign54890_e84761 * (((-locals.var_c_sb__blk1325_dn5) * locals.var_dphi_sb__blk1324) + (assign54890_e84758 * locals.var_dphi_sb__blk1324_dn5))), (assign54890_e84761 * (((-locals.var_c_sb__blk1325_dn6) * locals.var_dphi_sb__blk1324) + (assign54890_e84758 * locals.var_dphi_sb__blk1324_dn6))), (assign54890_e84761 * (((-locals.var_c_sb__blk1325_dn7) * locals.var_dphi_sb__blk1324) + (assign54890_e84758 * locals.var_dphi_sb__blk1324_dn7))), (assign54890_e84761 * (((-locals.var_c_sb__blk1325_dn8) * locals.var_dphi_sb__blk1324) + (assign54890_e84758 * locals.var_dphi_sb__blk1324_dn8))), (assign54890_e84761 * (((-locals.var_c_sb__blk1325_dn9) * locals.var_dphi_sb__blk1324) + (assign54890_e84758 * locals.var_dphi_sb__blk1324_dn9))), (assign54890_e84761 * (((-locals.var_c_sb__blk1325_dn10) * locals.var_dphi_sb__blk1324) + (assign54890_e84758 * locals.var_dphi_sb__blk1324_dn10))), (assign54890_e84761 * (((-locals.var_c_sb__blk1325_dn11) * locals.var_dphi_sb__blk1324) + (assign54890_e84758 * locals.var_dphi_sb__blk1324_dn11))), (assign54890_e84761 * (((-locals.var_c_sb__blk1325_dn14) * locals.var_dphi_sb__blk1324) + (assign54890_e84758 * locals.var_dphi_sb__blk1324_dn14))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign54890_e84763;
        locals.var_t0_dn0 = assign54890_e84763_d_n0;
        locals.var_t0_dn2 = assign54890_e84763_d_n2;
        locals.var_t0_dn4 = assign54890_e84763_d_n4;
        locals.var_t0_dn5 = assign54890_e84763_d_n5;
        locals.var_t0_dn6 = assign54890_e84763_d_n6;
        locals.var_t0_dn7 = assign54890_e84763_d_n7;
        locals.var_t0_dn8 = assign54890_e84763_d_n8;
        locals.var_t0_dn9 = assign54890_e84763_d_n9;
        locals.var_t0_dn10 = assign54890_e84763_d_n10;
        locals.var_t0_dn11 = assign54890_e84763_d_n11;
        locals.var_t0_dn14 = assign54890_e84763_d_n14;

        let assign54900_e84765: f64 = (locals.var_tx).abs();
        let assign54900_e84767: f64 = if assign54900_e84765 > 1e-8 { 1.0 } else { 0.0 };
        locals.var_guard1387 = assign54900_e84767;

        let assign54910_e84770: f64 = if locals.var_tx >= 500.0 { 1.0 } else { 0.0 };
        locals.var_guard1388 = assign54910_e84770;

        let (assign54920_e84797, assign54920_e84797_d_n0, assign54920_e84797_d_n2, assign54920_e84797_d_n4, assign54920_e84797_d_n5, assign54920_e84797_d_n6, assign54920_e84797_d_n7, assign54920_e84797_d_n8, assign54920_e84797_d_n9, assign54920_e84797_d_n10, assign54920_e84797_d_n11, assign54920_e84797_d_n14,) = {
    if ((((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_guard1386 == 0.0)) && (locals.var_guard1387 != 0.0)) && (locals.var_guard1388 != 0.0)) {
        let assign54920_e84792: f64 = (1.0 + locals.var_tx);
        let assign54920_e84794: f64 = (assign54920_e84792 - 500.0);
        let assign54920_e84795: f64 = (1.403592217853e217 * assign54920_e84794);
        (assign54920_e84795, (1.403592217853e217 * locals.var_tx_dn0), (1.403592217853e217 * locals.var_tx_dn2), (1.403592217853e217 * locals.var_tx_dn4), (1.403592217853e217 * locals.var_tx_dn5), (1.403592217853e217 * locals.var_tx_dn6), (1.403592217853e217 * locals.var_tx_dn7), (1.403592217853e217 * locals.var_tx_dn8), (1.403592217853e217 * locals.var_tx_dn9), (1.403592217853e217 * locals.var_tx_dn10), (1.403592217853e217 * locals.var_tx_dn11), (1.403592217853e217 * locals.var_tx_dn14),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign54920_e84797;
        locals.var_t1_dn0 = assign54920_e84797_d_n0;
        locals.var_t1_dn2 = assign54920_e84797_d_n2;
        locals.var_t1_dn4 = assign54920_e84797_d_n4;
        locals.var_t1_dn5 = assign54920_e84797_d_n5;
        locals.var_t1_dn6 = assign54920_e84797_d_n6;
        locals.var_t1_dn7 = assign54920_e84797_d_n7;
        locals.var_t1_dn8 = assign54920_e84797_d_n8;
        locals.var_t1_dn9 = assign54920_e84797_d_n9;
        locals.var_t1_dn10 = assign54920_e84797_d_n10;
        locals.var_t1_dn11 = assign54920_e84797_d_n11;
        locals.var_t1_dn14 = assign54920_e84797_d_n14;

        let (assign54930_e84818, assign54930_e84818_d_n0, assign54930_e84818_d_n2, assign54930_e84818_d_n4, assign54930_e84818_d_n5, assign54930_e84818_d_n6, assign54930_e84818_d_n7, assign54930_e84818_d_n8, assign54930_e84818_d_n9, assign54930_e84818_d_n10, assign54930_e84818_d_n11, assign54930_e84818_d_n14,) = {
    if ((((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_guard1386 == 0.0)) && (locals.var_guard1387 != 0.0)) && (locals.var_guard1388 != 0.0)) {
        (1.403592217853e217, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign54930_e84818;
        locals.var_t3_dn0 = assign54930_e84818_d_n0;
        locals.var_t3_dn2 = assign54930_e84818_d_n2;
        locals.var_t3_dn4 = assign54930_e84818_d_n4;
        locals.var_t3_dn5 = assign54930_e84818_d_n5;
        locals.var_t3_dn6 = assign54930_e84818_d_n6;
        locals.var_t3_dn7 = assign54930_e84818_d_n7;
        locals.var_t3_dn8 = assign54930_e84818_d_n8;
        locals.var_t3_dn9 = assign54930_e84818_d_n9;
        locals.var_t3_dn10 = assign54930_e84818_d_n10;
        locals.var_t3_dn11 = assign54930_e84818_d_n11;
        locals.var_t3_dn14 = assign54930_e84818_d_n14;

        let (assign54940_e84840, assign54940_e84840_d_n0, assign54940_e84840_d_n2, assign54940_e84840_d_n4, assign54940_e84840_d_n5, assign54940_e84840_d_n6, assign54940_e84840_d_n7, assign54940_e84840_d_n8, assign54940_e84840_d_n9, assign54940_e84840_d_n10, assign54940_e84840_d_n11, assign54940_e84840_d_n14,) = {
    if ((((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_guard1386 == 0.0)) && (locals.var_guard1387 != 0.0)) && (locals.var_guard1388 == 0.0)) {
        (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn4, locals.var_tx_dn5, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn8, locals.var_tx_dn9, locals.var_tx_dn10, locals.var_tx_dn11, locals.var_tx_dn14,)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign54940_e84840;
        locals.var_tmf1_dn0 = assign54940_e84840_d_n0;
        locals.var_tmf1_dn2 = assign54940_e84840_d_n2;
        locals.var_tmf1_dn4 = assign54940_e84840_d_n4;
        locals.var_tmf1_dn5 = assign54940_e84840_d_n5;
        locals.var_tmf1_dn6 = assign54940_e84840_d_n6;
        locals.var_tmf1_dn7 = assign54940_e84840_d_n7;
        locals.var_tmf1_dn8 = assign54940_e84840_d_n8;
        locals.var_tmf1_dn9 = assign54940_e84840_d_n9;
        locals.var_tmf1_dn10 = assign54940_e84840_d_n10;
        locals.var_tmf1_dn11 = assign54940_e84840_d_n11;
        locals.var_tmf1_dn14 = assign54940_e84840_d_n14;

        let (assign54950_e84862, assign54950_e84862_d_n0, assign54950_e84862_d_n2, assign54950_e84862_d_n4, assign54950_e84862_d_n5, assign54950_e84862_d_n6, assign54950_e84862_d_n7, assign54950_e84862_d_n8, assign54950_e84862_d_n9, assign54950_e84862_d_n10, assign54950_e84862_d_n11, assign54950_e84862_d_n14,) = {
    if ((((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_guard1386 == 0.0)) && (locals.var_guard1387 != 0.0)) && (locals.var_guard1388 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign54950_e84862;
        locals.var_t1_dn0 = assign54950_e84862_d_n0;
        locals.var_t1_dn2 = assign54950_e84862_d_n2;
        locals.var_t1_dn4 = assign54950_e84862_d_n4;
        locals.var_t1_dn5 = assign54950_e84862_d_n5;
        locals.var_t1_dn6 = assign54950_e84862_d_n6;
        locals.var_t1_dn7 = assign54950_e84862_d_n7;
        locals.var_t1_dn8 = assign54950_e84862_d_n8;
        locals.var_t1_dn9 = assign54950_e84862_d_n9;
        locals.var_t1_dn10 = assign54950_e84862_d_n10;
        locals.var_t1_dn11 = assign54950_e84862_d_n11;
        locals.var_t1_dn14 = assign54950_e84862_d_n14;

        let mut assign54960_loop_guard: usize = 0;
        while {
            let assign54960_cond_e84885: f64 = if (((((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_guard1386 == 0.0)) && (locals.var_guard1387 != 0.0)) && (locals.var_guard1388 == 0.0)) && (locals.var_tmf1 >= 60.0)) { 1.0 } else { 0.0 };
            assign54960_cond_e84885 != 0.0
        } {
            assign54960_loop_guard += 1;
            assert!(assign54960_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign54960_body0_e84909, assign54960_body0_e84909_d_n0, assign54960_body0_e84909_d_n2, assign54960_body0_e84909_d_n4, assign54960_body0_e84909_d_n5, assign54960_body0_e84909_d_n6, assign54960_body0_e84909_d_n7, assign54960_body0_e84909_d_n8, assign54960_body0_e84909_d_n9, assign54960_body0_e84909_d_n10, assign54960_body0_e84909_d_n11, assign54960_body0_e84909_d_n14,) = {
    if ((((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_guard1386 == 0.0)) && (locals.var_guard1387 != 0.0)) && (locals.var_guard1388 == 0.0)) {
        let assign54960_body0_e84907: f64 = (locals.var_t1 * 1.14200738981568e26);
        (assign54960_body0_e84907, (locals.var_t1_dn0 * 1.14200738981568e26), (locals.var_t1_dn2 * 1.14200738981568e26), (locals.var_t1_dn4 * 1.14200738981568e26), (locals.var_t1_dn5 * 1.14200738981568e26), (locals.var_t1_dn6 * 1.14200738981568e26), (locals.var_t1_dn7 * 1.14200738981568e26), (locals.var_t1_dn8 * 1.14200738981568e26), (locals.var_t1_dn9 * 1.14200738981568e26), (locals.var_t1_dn10 * 1.14200738981568e26), (locals.var_t1_dn11 * 1.14200738981568e26), (locals.var_t1_dn14 * 1.14200738981568e26),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
            locals.var_t1 = assign54960_body0_e84909;
            locals.var_t1_dn0 = assign54960_body0_e84909_d_n0;
            locals.var_t1_dn2 = assign54960_body0_e84909_d_n2;
            locals.var_t1_dn4 = assign54960_body0_e84909_d_n4;
            locals.var_t1_dn5 = assign54960_body0_e84909_d_n5;
            locals.var_t1_dn6 = assign54960_body0_e84909_d_n6;
            locals.var_t1_dn7 = assign54960_body0_e84909_d_n7;
            locals.var_t1_dn8 = assign54960_body0_e84909_d_n8;
            locals.var_t1_dn9 = assign54960_body0_e84909_d_n9;
            locals.var_t1_dn10 = assign54960_body0_e84909_d_n10;
            locals.var_t1_dn11 = assign54960_body0_e84909_d_n11;
            locals.var_t1_dn14 = assign54960_body0_e84909_d_n14;
            let (assign54960_body1_e84933, assign54960_body1_e84933_d_n0, assign54960_body1_e84933_d_n2, assign54960_body1_e84933_d_n4, assign54960_body1_e84933_d_n5, assign54960_body1_e84933_d_n6, assign54960_body1_e84933_d_n7, assign54960_body1_e84933_d_n8, assign54960_body1_e84933_d_n9, assign54960_body1_e84933_d_n10, assign54960_body1_e84933_d_n11, assign54960_body1_e84933_d_n14,) = {
    if ((((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_guard1386 == 0.0)) && (locals.var_guard1387 != 0.0)) && (locals.var_guard1388 == 0.0)) {
        let assign54960_body1_e84931: f64 = (locals.var_tmf1 - 60.0);
        (assign54960_body1_e84931, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
            locals.var_tmf1 = assign54960_body1_e84933;
            locals.var_tmf1_dn0 = assign54960_body1_e84933_d_n0;
            locals.var_tmf1_dn2 = assign54960_body1_e84933_d_n2;
            locals.var_tmf1_dn4 = assign54960_body1_e84933_d_n4;
            locals.var_tmf1_dn5 = assign54960_body1_e84933_d_n5;
            locals.var_tmf1_dn6 = assign54960_body1_e84933_d_n6;
            locals.var_tmf1_dn7 = assign54960_body1_e84933_d_n7;
            locals.var_tmf1_dn8 = assign54960_body1_e84933_d_n8;
            locals.var_tmf1_dn9 = assign54960_body1_e84933_d_n9;
            locals.var_tmf1_dn10 = assign54960_body1_e84933_d_n10;
            locals.var_tmf1_dn11 = assign54960_body1_e84933_d_n11;
            locals.var_tmf1_dn14 = assign54960_body1_e84933_d_n14;
        }

        let (assign54970_e84958, assign54970_e84958_d_n0, assign54970_e84958_d_n2, assign54970_e84958_d_n4, assign54970_e84958_d_n5, assign54970_e84958_d_n6, assign54970_e84958_d_n7, assign54970_e84958_d_n8, assign54970_e84958_d_n9, assign54970_e84958_d_n10, assign54970_e84958_d_n11, assign54970_e84958_d_n14,) = {
    if ((((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_guard1386 == 0.0)) && (locals.var_guard1387 != 0.0)) && (locals.var_guard1388 == 0.0)) {
        let assign54970_e84955: f64 = (locals.var_tmf1).exp();
        let assign54970_e84956: f64 = (locals.var_t1 * assign54970_e84955);
        (assign54970_e84956, ((locals.var_t1_dn0 * assign54970_e84955) + (locals.var_t1 * (assign54970_e84955 * locals.var_tmf1_dn0))), ((locals.var_t1_dn2 * assign54970_e84955) + (locals.var_t1 * (assign54970_e84955 * locals.var_tmf1_dn2))), ((locals.var_t1_dn4 * assign54970_e84955) + (locals.var_t1 * (assign54970_e84955 * locals.var_tmf1_dn4))), ((locals.var_t1_dn5 * assign54970_e84955) + (locals.var_t1 * (assign54970_e84955 * locals.var_tmf1_dn5))), ((locals.var_t1_dn6 * assign54970_e84955) + (locals.var_t1 * (assign54970_e84955 * locals.var_tmf1_dn6))), ((locals.var_t1_dn7 * assign54970_e84955) + (locals.var_t1 * (assign54970_e84955 * locals.var_tmf1_dn7))), ((locals.var_t1_dn8 * assign54970_e84955) + (locals.var_t1 * (assign54970_e84955 * locals.var_tmf1_dn8))), ((locals.var_t1_dn9 * assign54970_e84955) + (locals.var_t1 * (assign54970_e84955 * locals.var_tmf1_dn9))), ((locals.var_t1_dn10 * assign54970_e84955) + (locals.var_t1 * (assign54970_e84955 * locals.var_tmf1_dn10))), ((locals.var_t1_dn11 * assign54970_e84955) + (locals.var_t1 * (assign54970_e84955 * locals.var_tmf1_dn11))), ((locals.var_t1_dn14 * assign54970_e84955) + (locals.var_t1 * (assign54970_e84955 * locals.var_tmf1_dn14))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign54970_e84958;
        locals.var_t1_dn0 = assign54970_e84958_d_n0;
        locals.var_t1_dn2 = assign54970_e84958_d_n2;
        locals.var_t1_dn4 = assign54970_e84958_d_n4;
        locals.var_t1_dn5 = assign54970_e84958_d_n5;
        locals.var_t1_dn6 = assign54970_e84958_d_n6;
        locals.var_t1_dn7 = assign54970_e84958_d_n7;
        locals.var_t1_dn8 = assign54970_e84958_d_n8;
        locals.var_t1_dn9 = assign54970_e84958_d_n9;
        locals.var_t1_dn10 = assign54970_e84958_d_n10;
        locals.var_t1_dn11 = assign54970_e84958_d_n11;
        locals.var_t1_dn14 = assign54970_e84958_d_n14;

        let (assign54980_e84980, assign54980_e84980_d_n0, assign54980_e84980_d_n2, assign54980_e84980_d_n4, assign54980_e84980_d_n5, assign54980_e84980_d_n6, assign54980_e84980_d_n7, assign54980_e84980_d_n8, assign54980_e84980_d_n9, assign54980_e84980_d_n10, assign54980_e84980_d_n11, assign54980_e84980_d_n14,) = {
    if ((((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_guard1386 == 0.0)) && (locals.var_guard1387 != 0.0)) && (locals.var_guard1388 == 0.0)) {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign54980_e84980;
        locals.var_t3_dn0 = assign54980_e84980_d_n0;
        locals.var_t3_dn2 = assign54980_e84980_d_n2;
        locals.var_t3_dn4 = assign54980_e84980_d_n4;
        locals.var_t3_dn5 = assign54980_e84980_d_n5;
        locals.var_t3_dn6 = assign54980_e84980_d_n6;
        locals.var_t3_dn7 = assign54980_e84980_d_n7;
        locals.var_t3_dn8 = assign54980_e84980_d_n8;
        locals.var_t3_dn9 = assign54980_e84980_d_n9;
        locals.var_t3_dn10 = assign54980_e84980_d_n10;
        locals.var_t3_dn11 = assign54980_e84980_d_n11;
        locals.var_t3_dn14 = assign54980_e84980_d_n14;

        let (assign54990_e85001, assign54990_e85001_d_n0, assign54990_e85001_d_n2, assign54990_e85001_d_n4, assign54990_e85001_d_n5, assign54990_e85001_d_n6, assign54990_e85001_d_n7, assign54990_e85001_d_n8, assign54990_e85001_d_n9, assign54990_e85001_d_n10, assign54990_e85001_d_n11, assign54990_e85001_d_n14,) = {
    if (((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_guard1386 == 0.0)) && (locals.var_guard1387 != 0.0)) {
        let assign54990_e84999: f64 = (locals.var_t1 * locals.var_t0);
        (assign54990_e84999, ((locals.var_t1_dn0 * locals.var_t0) + (locals.var_t1 * locals.var_t0_dn0)), ((locals.var_t1_dn2 * locals.var_t0) + (locals.var_t1 * locals.var_t0_dn2)), ((locals.var_t1_dn4 * locals.var_t0) + (locals.var_t1 * locals.var_t0_dn4)), ((locals.var_t1_dn5 * locals.var_t0) + (locals.var_t1 * locals.var_t0_dn5)), ((locals.var_t1_dn6 * locals.var_t0) + (locals.var_t1 * locals.var_t0_dn6)), ((locals.var_t1_dn7 * locals.var_t0) + (locals.var_t1 * locals.var_t0_dn7)), ((locals.var_t1_dn8 * locals.var_t0) + (locals.var_t1 * locals.var_t0_dn8)), ((locals.var_t1_dn9 * locals.var_t0) + (locals.var_t1 * locals.var_t0_dn9)), ((locals.var_t1_dn10 * locals.var_t0) + (locals.var_t1 * locals.var_t0_dn10)), ((locals.var_t1_dn11 * locals.var_t0) + (locals.var_t1 * locals.var_t0_dn11)), ((locals.var_t1_dn14 * locals.var_t0) + (locals.var_t1 * locals.var_t0_dn14)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign54990_e85001;
        locals.var_t1_dn0 = assign54990_e85001_d_n0;
        locals.var_t1_dn2 = assign54990_e85001_d_n2;
        locals.var_t1_dn4 = assign54990_e85001_d_n4;
        locals.var_t1_dn5 = assign54990_e85001_d_n5;
        locals.var_t1_dn6 = assign54990_e85001_d_n6;
        locals.var_t1_dn7 = assign54990_e85001_d_n7;
        locals.var_t1_dn8 = assign54990_e85001_d_n8;
        locals.var_t1_dn9 = assign54990_e85001_d_n9;
        locals.var_t1_dn10 = assign54990_e85001_d_n10;
        locals.var_t1_dn11 = assign54990_e85001_d_n11;
        locals.var_t1_dn14 = assign54990_e85001_d_n14;

        let (assign55000_e85022, assign55000_e85022_d_n0, assign55000_e85022_d_n2, assign55000_e85022_d_n4, assign55000_e85022_d_n5, assign55000_e85022_d_n6, assign55000_e85022_d_n7, assign55000_e85022_d_n8, assign55000_e85022_d_n9, assign55000_e85022_d_n10, assign55000_e85022_d_n11, assign55000_e85022_d_n14,) = {
    if (((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_guard1386 == 0.0)) && (locals.var_guard1387 != 0.0)) {
        let assign55000_e85020: f64 = (locals.var_t1 - locals.var_t0);
        (assign55000_e85020, (locals.var_t1_dn0 - locals.var_t0_dn0), (locals.var_t1_dn2 - locals.var_t0_dn2), (locals.var_t1_dn4 - locals.var_t0_dn4), (locals.var_t1_dn5 - locals.var_t0_dn5), (locals.var_t1_dn6 - locals.var_t0_dn6), (locals.var_t1_dn7 - locals.var_t0_dn7), (locals.var_t1_dn8 - locals.var_t0_dn8), (locals.var_t1_dn9 - locals.var_t0_dn9), (locals.var_t1_dn10 - locals.var_t0_dn10), (locals.var_t1_dn11 - locals.var_t0_dn11), (locals.var_t1_dn14 - locals.var_t0_dn14),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign55000_e85022;
        locals.var_t2_dn0 = assign55000_e85022_d_n0;
        locals.var_t2_dn2 = assign55000_e85022_d_n2;
        locals.var_t2_dn4 = assign55000_e85022_d_n4;
        locals.var_t2_dn5 = assign55000_e85022_d_n5;
        locals.var_t2_dn6 = assign55000_e85022_d_n6;
        locals.var_t2_dn7 = assign55000_e85022_d_n7;
        locals.var_t2_dn8 = assign55000_e85022_d_n8;
        locals.var_t2_dn9 = assign55000_e85022_d_n9;
        locals.var_t2_dn10 = assign55000_e85022_d_n10;
        locals.var_t2_dn11 = assign55000_e85022_d_n11;
        locals.var_t2_dn14 = assign55000_e85022_d_n14;

    }

    pub(super) fn stamp_transient_block_189(
        locals: &mut StampLocals,
    ) {
        let (assign55010_e85046, assign55010_e85046_d_n0, assign55010_e85046_d_n2, assign55010_e85046_d_n4, assign55010_e85046_d_n5, assign55010_e85046_d_n6, assign55010_e85046_d_n7, assign55010_e85046_d_n8, assign55010_e85046_d_n9, assign55010_e85046_d_n10, assign55010_e85046_d_n11, assign55010_e85046_d_n14,) = {
    if (((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_guard1386 == 0.0)) && (locals.var_guard1387 == 0.0)) {
        let assign55010_e85042: f64 = (1.0 + locals.var_tx);
        let assign55010_e85044: f64 = (assign55010_e85042 * locals.var_t0);
        (assign55010_e85044, ((locals.var_tx_dn0 * locals.var_t0) + (assign55010_e85042 * locals.var_t0_dn0)), ((locals.var_tx_dn2 * locals.var_t0) + (assign55010_e85042 * locals.var_t0_dn2)), ((locals.var_tx_dn4 * locals.var_t0) + (assign55010_e85042 * locals.var_t0_dn4)), ((locals.var_tx_dn5 * locals.var_t0) + (assign55010_e85042 * locals.var_t0_dn5)), ((locals.var_tx_dn6 * locals.var_t0) + (assign55010_e85042 * locals.var_t0_dn6)), ((locals.var_tx_dn7 * locals.var_t0) + (assign55010_e85042 * locals.var_t0_dn7)), ((locals.var_tx_dn8 * locals.var_t0) + (assign55010_e85042 * locals.var_t0_dn8)), ((locals.var_tx_dn9 * locals.var_t0) + (assign55010_e85042 * locals.var_t0_dn9)), ((locals.var_tx_dn10 * locals.var_t0) + (assign55010_e85042 * locals.var_t0_dn10)), ((locals.var_tx_dn11 * locals.var_t0) + (assign55010_e85042 * locals.var_t0_dn11)), ((locals.var_tx_dn14 * locals.var_t0) + (assign55010_e85042 * locals.var_t0_dn14)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign55010_e85046;
        locals.var_t1_dn0 = assign55010_e85046_d_n0;
        locals.var_t1_dn2 = assign55010_e85046_d_n2;
        locals.var_t1_dn4 = assign55010_e85046_d_n4;
        locals.var_t1_dn5 = assign55010_e85046_d_n5;
        locals.var_t1_dn6 = assign55010_e85046_d_n6;
        locals.var_t1_dn7 = assign55010_e85046_d_n7;
        locals.var_t1_dn8 = assign55010_e85046_d_n8;
        locals.var_t1_dn9 = assign55010_e85046_d_n9;
        locals.var_t1_dn10 = assign55010_e85046_d_n10;
        locals.var_t1_dn11 = assign55010_e85046_d_n11;
        locals.var_t1_dn14 = assign55010_e85046_d_n14;

        let (assign55020_e85074, assign55020_e85074_d_n0, assign55020_e85074_d_n2, assign55020_e85074_d_n4, assign55020_e85074_d_n5, assign55020_e85074_d_n6, assign55020_e85074_d_n7, assign55020_e85074_d_n8, assign55020_e85074_d_n9, assign55020_e85074_d_n10, assign55020_e85074_d_n11, assign55020_e85074_d_n14,) = {
    if (((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_guard1386 == 0.0)) && (locals.var_guard1387 == 0.0)) {
        let assign55020_e85068: f64 = (locals.var_tx / 2.0);
        let assign55020_e85069: f64 = (1.0 + assign55020_e85068);
        let assign55020_e85070: f64 = (locals.var_tx * assign55020_e85069);
        let assign55020_e85072: f64 = (assign55020_e85070 * locals.var_t0);
        (assign55020_e85072, ((((locals.var_tx_dn0 * assign55020_e85069) + (locals.var_tx * (locals.var_tx_dn0 / 2.0))) * locals.var_t0) + (assign55020_e85070 * locals.var_t0_dn0)), ((((locals.var_tx_dn2 * assign55020_e85069) + (locals.var_tx * (locals.var_tx_dn2 / 2.0))) * locals.var_t0) + (assign55020_e85070 * locals.var_t0_dn2)), ((((locals.var_tx_dn4 * assign55020_e85069) + (locals.var_tx * (locals.var_tx_dn4 / 2.0))) * locals.var_t0) + (assign55020_e85070 * locals.var_t0_dn4)), ((((locals.var_tx_dn5 * assign55020_e85069) + (locals.var_tx * (locals.var_tx_dn5 / 2.0))) * locals.var_t0) + (assign55020_e85070 * locals.var_t0_dn5)), ((((locals.var_tx_dn6 * assign55020_e85069) + (locals.var_tx * (locals.var_tx_dn6 / 2.0))) * locals.var_t0) + (assign55020_e85070 * locals.var_t0_dn6)), ((((locals.var_tx_dn7 * assign55020_e85069) + (locals.var_tx * (locals.var_tx_dn7 / 2.0))) * locals.var_t0) + (assign55020_e85070 * locals.var_t0_dn7)), ((((locals.var_tx_dn8 * assign55020_e85069) + (locals.var_tx * (locals.var_tx_dn8 / 2.0))) * locals.var_t0) + (assign55020_e85070 * locals.var_t0_dn8)), ((((locals.var_tx_dn9 * assign55020_e85069) + (locals.var_tx * (locals.var_tx_dn9 / 2.0))) * locals.var_t0) + (assign55020_e85070 * locals.var_t0_dn9)), ((((locals.var_tx_dn10 * assign55020_e85069) + (locals.var_tx * (locals.var_tx_dn10 / 2.0))) * locals.var_t0) + (assign55020_e85070 * locals.var_t0_dn10)), ((((locals.var_tx_dn11 * assign55020_e85069) + (locals.var_tx * (locals.var_tx_dn11 / 2.0))) * locals.var_t0) + (assign55020_e85070 * locals.var_t0_dn11)), ((((locals.var_tx_dn14 * assign55020_e85069) + (locals.var_tx * (locals.var_tx_dn14 / 2.0))) * locals.var_t0) + (assign55020_e85070 * locals.var_t0_dn14)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign55020_e85074;
        locals.var_t2_dn0 = assign55020_e85074_d_n0;
        locals.var_t2_dn2 = assign55020_e85074_d_n2;
        locals.var_t2_dn4 = assign55020_e85074_d_n4;
        locals.var_t2_dn5 = assign55020_e85074_d_n5;
        locals.var_t2_dn6 = assign55020_e85074_d_n6;
        locals.var_t2_dn7 = assign55020_e85074_d_n7;
        locals.var_t2_dn8 = assign55020_e85074_d_n8;
        locals.var_t2_dn9 = assign55020_e85074_d_n9;
        locals.var_t2_dn10 = assign55020_e85074_d_n10;
        locals.var_t2_dn11 = assign55020_e85074_d_n11;
        locals.var_t2_dn14 = assign55020_e85074_d_n14;

        let assign55030_e85076: f64 = (locals.var_t2).abs();
        let assign55030_e85078: f64 = if assign55030_e85076 > 1e-8 { 1.0 } else { 0.0 };
        locals.var_guard1389 = assign55030_e85078;

        let (assign55040_e85102, assign55040_e85102_d_n0, assign55040_e85102_d_n2, assign55040_e85102_d_n4, assign55040_e85102_d_n5, assign55040_e85102_d_n6, assign55040_e85102_d_n7, assign55040_e85102_d_n8, assign55040_e85102_d_n9, assign55040_e85102_d_n10, assign55040_e85102_d_n11, assign55040_e85102_d_n14,) = {
    if (((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_guard1386 == 0.0)) && (locals.var_guard1389 != 0.0)) {
        let assign55040_e85097: f64 = (1.0 + locals.var_t2);
        let assign55040_e85098: f64 = (assign55040_e85097).ln();
        let assign55040_e85100: f64 = (assign55040_e85098 / locals.var_c_sb__blk1325);
        (assign55040_e85100, ((((locals.var_t2_dn0 / assign55040_e85097) * locals.var_c_sb__blk1325) - (assign55040_e85098 * locals.var_c_sb__blk1325_dn0)) / (locals.var_c_sb__blk1325 * locals.var_c_sb__blk1325)), ((((locals.var_t2_dn2 / assign55040_e85097) * locals.var_c_sb__blk1325) - (assign55040_e85098 * locals.var_c_sb__blk1325_dn2)) / (locals.var_c_sb__blk1325 * locals.var_c_sb__blk1325)), ((((locals.var_t2_dn4 / assign55040_e85097) * locals.var_c_sb__blk1325) - (assign55040_e85098 * locals.var_c_sb__blk1325_dn4)) / (locals.var_c_sb__blk1325 * locals.var_c_sb__blk1325)), ((((locals.var_t2_dn5 / assign55040_e85097) * locals.var_c_sb__blk1325) - (assign55040_e85098 * locals.var_c_sb__blk1325_dn5)) / (locals.var_c_sb__blk1325 * locals.var_c_sb__blk1325)), ((((locals.var_t2_dn6 / assign55040_e85097) * locals.var_c_sb__blk1325) - (assign55040_e85098 * locals.var_c_sb__blk1325_dn6)) / (locals.var_c_sb__blk1325 * locals.var_c_sb__blk1325)), ((((locals.var_t2_dn7 / assign55040_e85097) * locals.var_c_sb__blk1325) - (assign55040_e85098 * locals.var_c_sb__blk1325_dn7)) / (locals.var_c_sb__blk1325 * locals.var_c_sb__blk1325)), ((((locals.var_t2_dn8 / assign55040_e85097) * locals.var_c_sb__blk1325) - (assign55040_e85098 * locals.var_c_sb__blk1325_dn8)) / (locals.var_c_sb__blk1325 * locals.var_c_sb__blk1325)), ((((locals.var_t2_dn9 / assign55040_e85097) * locals.var_c_sb__blk1325) - (assign55040_e85098 * locals.var_c_sb__blk1325_dn9)) / (locals.var_c_sb__blk1325 * locals.var_c_sb__blk1325)), ((((locals.var_t2_dn10 / assign55040_e85097) * locals.var_c_sb__blk1325) - (assign55040_e85098 * locals.var_c_sb__blk1325_dn10)) / (locals.var_c_sb__blk1325 * locals.var_c_sb__blk1325)), ((((locals.var_t2_dn11 / assign55040_e85097) * locals.var_c_sb__blk1325) - (assign55040_e85098 * locals.var_c_sb__blk1325_dn11)) / (locals.var_c_sb__blk1325 * locals.var_c_sb__blk1325)), ((((locals.var_t2_dn14 / assign55040_e85097) * locals.var_c_sb__blk1325) - (assign55040_e85098 * locals.var_c_sb__blk1325_dn14)) / (locals.var_c_sb__blk1325 * locals.var_c_sb__blk1325)),)
    } else {
        (locals.var_pb0dep__blk1169, locals.var_pb0dep__blk1169_dn0, locals.var_pb0dep__blk1169_dn2, locals.var_pb0dep__blk1169_dn4, locals.var_pb0dep__blk1169_dn5, locals.var_pb0dep__blk1169_dn6, locals.var_pb0dep__blk1169_dn7, locals.var_pb0dep__blk1169_dn8, locals.var_pb0dep__blk1169_dn9, locals.var_pb0dep__blk1169_dn10, locals.var_pb0dep__blk1169_dn11, locals.var_pb0dep__blk1169_dn14,)
    }
};
        locals.var_pb0dep__blk1169 = assign55040_e85102;
        locals.var_pb0dep__blk1169_dn0 = assign55040_e85102_d_n0;
        locals.var_pb0dep__blk1169_dn2 = assign55040_e85102_d_n2;
        locals.var_pb0dep__blk1169_dn4 = assign55040_e85102_d_n4;
        locals.var_pb0dep__blk1169_dn5 = assign55040_e85102_d_n5;
        locals.var_pb0dep__blk1169_dn6 = assign55040_e85102_d_n6;
        locals.var_pb0dep__blk1169_dn7 = assign55040_e85102_d_n7;
        locals.var_pb0dep__blk1169_dn8 = assign55040_e85102_d_n8;
        locals.var_pb0dep__blk1169_dn9 = assign55040_e85102_d_n9;
        locals.var_pb0dep__blk1169_dn10 = assign55040_e85102_d_n10;
        locals.var_pb0dep__blk1169_dn11 = assign55040_e85102_d_n11;
        locals.var_pb0dep__blk1169_dn14 = assign55040_e85102_d_n14;

        let (assign55050_e85124, assign55050_e85124_d_n0, assign55050_e85124_d_n2, assign55050_e85124_d_n4, assign55050_e85124_d_n5, assign55050_e85124_d_n6, assign55050_e85124_d_n7, assign55050_e85124_d_n8, assign55050_e85124_d_n9, assign55050_e85124_d_n10, assign55050_e85124_d_n11, assign55050_e85124_d_n14,) = {
    if (((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_guard1386 == 0.0)) && (locals.var_guard1389 == 0.0)) {
        let assign55050_e85122: f64 = (locals.var_t2 / locals.var_c_sb__blk1325);
        (assign55050_e85122, (((locals.var_t2_dn0 * locals.var_c_sb__blk1325) - (locals.var_t2 * locals.var_c_sb__blk1325_dn0)) / (locals.var_c_sb__blk1325 * locals.var_c_sb__blk1325)), (((locals.var_t2_dn2 * locals.var_c_sb__blk1325) - (locals.var_t2 * locals.var_c_sb__blk1325_dn2)) / (locals.var_c_sb__blk1325 * locals.var_c_sb__blk1325)), (((locals.var_t2_dn4 * locals.var_c_sb__blk1325) - (locals.var_t2 * locals.var_c_sb__blk1325_dn4)) / (locals.var_c_sb__blk1325 * locals.var_c_sb__blk1325)), (((locals.var_t2_dn5 * locals.var_c_sb__blk1325) - (locals.var_t2 * locals.var_c_sb__blk1325_dn5)) / (locals.var_c_sb__blk1325 * locals.var_c_sb__blk1325)), (((locals.var_t2_dn6 * locals.var_c_sb__blk1325) - (locals.var_t2 * locals.var_c_sb__blk1325_dn6)) / (locals.var_c_sb__blk1325 * locals.var_c_sb__blk1325)), (((locals.var_t2_dn7 * locals.var_c_sb__blk1325) - (locals.var_t2 * locals.var_c_sb__blk1325_dn7)) / (locals.var_c_sb__blk1325 * locals.var_c_sb__blk1325)), (((locals.var_t2_dn8 * locals.var_c_sb__blk1325) - (locals.var_t2 * locals.var_c_sb__blk1325_dn8)) / (locals.var_c_sb__blk1325 * locals.var_c_sb__blk1325)), (((locals.var_t2_dn9 * locals.var_c_sb__blk1325) - (locals.var_t2 * locals.var_c_sb__blk1325_dn9)) / (locals.var_c_sb__blk1325 * locals.var_c_sb__blk1325)), (((locals.var_t2_dn10 * locals.var_c_sb__blk1325) - (locals.var_t2 * locals.var_c_sb__blk1325_dn10)) / (locals.var_c_sb__blk1325 * locals.var_c_sb__blk1325)), (((locals.var_t2_dn11 * locals.var_c_sb__blk1325) - (locals.var_t2 * locals.var_c_sb__blk1325_dn11)) / (locals.var_c_sb__blk1325 * locals.var_c_sb__blk1325)), (((locals.var_t2_dn14 * locals.var_c_sb__blk1325) - (locals.var_t2 * locals.var_c_sb__blk1325_dn14)) / (locals.var_c_sb__blk1325 * locals.var_c_sb__blk1325)),)
    } else {
        (locals.var_pb0dep__blk1169, locals.var_pb0dep__blk1169_dn0, locals.var_pb0dep__blk1169_dn2, locals.var_pb0dep__blk1169_dn4, locals.var_pb0dep__blk1169_dn5, locals.var_pb0dep__blk1169_dn6, locals.var_pb0dep__blk1169_dn7, locals.var_pb0dep__blk1169_dn8, locals.var_pb0dep__blk1169_dn9, locals.var_pb0dep__blk1169_dn10, locals.var_pb0dep__blk1169_dn11, locals.var_pb0dep__blk1169_dn14,)
    }
};
        locals.var_pb0dep__blk1169 = assign55050_e85124;
        locals.var_pb0dep__blk1169_dn0 = assign55050_e85124_d_n0;
        locals.var_pb0dep__blk1169_dn2 = assign55050_e85124_d_n2;
        locals.var_pb0dep__blk1169_dn4 = assign55050_e85124_d_n4;
        locals.var_pb0dep__blk1169_dn5 = assign55050_e85124_d_n5;
        locals.var_pb0dep__blk1169_dn6 = assign55050_e85124_d_n6;
        locals.var_pb0dep__blk1169_dn7 = assign55050_e85124_d_n7;
        locals.var_pb0dep__blk1169_dn8 = assign55050_e85124_d_n8;
        locals.var_pb0dep__blk1169_dn9 = assign55050_e85124_d_n9;
        locals.var_pb0dep__blk1169_dn10 = assign55050_e85124_d_n10;
        locals.var_pb0dep__blk1169_dn11 = assign55050_e85124_d_n11;
        locals.var_pb0dep__blk1169_dn14 = assign55050_e85124_d_n14;

        let (assign55060_e85140, assign55060_e85140_d_n0, assign55060_e85140_d_n2, assign55060_e85140_d_n4, assign55060_e85140_d_n5, assign55060_e85140_d_n6, assign55060_e85140_d_n7, assign55060_e85140_d_n8, assign55060_e85140_d_n9, assign55060_e85140_d_n10, assign55060_e85140_d_n11, assign55060_e85140_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) {
        let assign55060_e85138: f64 = (locals.var_ps0dep - locals.var_pb0dep__blk1169);
        (assign55060_e85138, (locals.var_ps0dep_dn0 - locals.var_pb0dep__blk1169_dn0), (locals.var_ps0dep_dn2 - locals.var_pb0dep__blk1169_dn2), (locals.var_ps0dep_dn4 - locals.var_pb0dep__blk1169_dn4), (locals.var_ps0dep_dn5 - locals.var_pb0dep__blk1169_dn5), (locals.var_ps0dep_dn6 - locals.var_pb0dep__blk1169_dn6), (locals.var_ps0dep_dn7 - locals.var_pb0dep__blk1169_dn7), (locals.var_ps0dep_dn8 - locals.var_pb0dep__blk1169_dn8), (locals.var_ps0dep_dn9 - locals.var_pb0dep__blk1169_dn9), (locals.var_ps0dep_dn10 - locals.var_pb0dep__blk1169_dn10), (locals.var_ps0dep_dn11 - locals.var_pb0dep__blk1169_dn11), (locals.var_ps0dep_dn14 - locals.var_pb0dep__blk1169_dn14),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign55060_e85140;
        locals.var_t2_dn0 = assign55060_e85140_d_n0;
        locals.var_t2_dn2 = assign55060_e85140_d_n2;
        locals.var_t2_dn4 = assign55060_e85140_d_n4;
        locals.var_t2_dn5 = assign55060_e85140_d_n5;
        locals.var_t2_dn6 = assign55060_e85140_d_n6;
        locals.var_t2_dn7 = assign55060_e85140_d_n7;
        locals.var_t2_dn8 = assign55060_e85140_d_n8;
        locals.var_t2_dn9 = assign55060_e85140_d_n9;
        locals.var_t2_dn10 = assign55060_e85140_d_n10;
        locals.var_t2_dn11 = assign55060_e85140_d_n11;
        locals.var_t2_dn14 = assign55060_e85140_d_n14;

        let assign55070_e85143: f64 = if 0.0 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1390 = assign55070_e85143;

        let (assign55080_e85172, assign55080_e85172_d_n0, assign55080_e85172_d_n2, assign55080_e85172_d_n4, assign55080_e85172_d_n5, assign55080_e85172_d_n6, assign55080_e85172_d_n7, assign55080_e85172_d_n8, assign55080_e85172_d_n9, assign55080_e85172_d_n10, assign55080_e85172_d_n11, assign55080_e85172_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_guard1390 != 0.0)) {
        let (assign55080_e85170, assign55080_e85170_d_n0, assign55080_e85170_d_n2, assign55080_e85170_d_n4, assign55080_e85170_d_n5, assign55080_e85170_d_n6, assign55080_e85170_d_n7, assign55080_e85170_d_n8, assign55080_e85170_d_n9, assign55080_e85170_d_n10, assign55080_e85170_d_n11, assign55080_e85170_d_n14,) = {
            if (locals.var_t2 < 0.0) {
                let assign55080_e85161: f64 = (-locals.var_c_2esipq_ndepm__blk1140);
                let assign55080_e85163: f64 = (assign55080_e85161 * locals.var_t2);
                let assign55080_e85164: f64 = (assign55080_e85163).sqrt();
                let assign55080_e85165: f64 = (-assign55080_e85164);
                (assign55080_e85165, (-((((-locals.var_c_2esipq_ndepm__blk1140_dn0) * locals.var_t2) + (assign55080_e85161 * locals.var_t2_dn0)) / (2.0 * assign55080_e85164))), (-((((-locals.var_c_2esipq_ndepm__blk1140_dn2) * locals.var_t2) + (assign55080_e85161 * locals.var_t2_dn2)) / (2.0 * assign55080_e85164))), (-((((-locals.var_c_2esipq_ndepm__blk1140_dn4) * locals.var_t2) + (assign55080_e85161 * locals.var_t2_dn4)) / (2.0 * assign55080_e85164))), (-((((-locals.var_c_2esipq_ndepm__blk1140_dn5) * locals.var_t2) + (assign55080_e85161 * locals.var_t2_dn5)) / (2.0 * assign55080_e85164))), (-((((-locals.var_c_2esipq_ndepm__blk1140_dn6) * locals.var_t2) + (assign55080_e85161 * locals.var_t2_dn6)) / (2.0 * assign55080_e85164))), (-((((-locals.var_c_2esipq_ndepm__blk1140_dn7) * locals.var_t2) + (assign55080_e85161 * locals.var_t2_dn7)) / (2.0 * assign55080_e85164))), (-((((-locals.var_c_2esipq_ndepm__blk1140_dn8) * locals.var_t2) + (assign55080_e85161 * locals.var_t2_dn8)) / (2.0 * assign55080_e85164))), (-((((-locals.var_c_2esipq_ndepm__blk1140_dn9) * locals.var_t2) + (assign55080_e85161 * locals.var_t2_dn9)) / (2.0 * assign55080_e85164))), (-((((-locals.var_c_2esipq_ndepm__blk1140_dn10) * locals.var_t2) + (assign55080_e85161 * locals.var_t2_dn10)) / (2.0 * assign55080_e85164))), (-((((-locals.var_c_2esipq_ndepm__blk1140_dn11) * locals.var_t2) + (assign55080_e85161 * locals.var_t2_dn11)) / (2.0 * assign55080_e85164))), (-((((-locals.var_c_2esipq_ndepm__blk1140_dn14) * locals.var_t2) + (assign55080_e85161 * locals.var_t2_dn14)) / (2.0 * assign55080_e85164))),)
            } else {
                let assign55080_e85168: f64 = (locals.var_c_2esipq_ndepm__blk1140 * locals.var_t2);
                let assign55080_e85169: f64 = (assign55080_e85168).sqrt();
                (assign55080_e85169, (((locals.var_c_2esipq_ndepm__blk1140_dn0 * locals.var_t2) + (locals.var_c_2esipq_ndepm__blk1140 * locals.var_t2_dn0)) / (2.0 * assign55080_e85169)), (((locals.var_c_2esipq_ndepm__blk1140_dn2 * locals.var_t2) + (locals.var_c_2esipq_ndepm__blk1140 * locals.var_t2_dn2)) / (2.0 * assign55080_e85169)), (((locals.var_c_2esipq_ndepm__blk1140_dn4 * locals.var_t2) + (locals.var_c_2esipq_ndepm__blk1140 * locals.var_t2_dn4)) / (2.0 * assign55080_e85169)), (((locals.var_c_2esipq_ndepm__blk1140_dn5 * locals.var_t2) + (locals.var_c_2esipq_ndepm__blk1140 * locals.var_t2_dn5)) / (2.0 * assign55080_e85169)), (((locals.var_c_2esipq_ndepm__blk1140_dn6 * locals.var_t2) + (locals.var_c_2esipq_ndepm__blk1140 * locals.var_t2_dn6)) / (2.0 * assign55080_e85169)), (((locals.var_c_2esipq_ndepm__blk1140_dn7 * locals.var_t2) + (locals.var_c_2esipq_ndepm__blk1140 * locals.var_t2_dn7)) / (2.0 * assign55080_e85169)), (((locals.var_c_2esipq_ndepm__blk1140_dn8 * locals.var_t2) + (locals.var_c_2esipq_ndepm__blk1140 * locals.var_t2_dn8)) / (2.0 * assign55080_e85169)), (((locals.var_c_2esipq_ndepm__blk1140_dn9 * locals.var_t2) + (locals.var_c_2esipq_ndepm__blk1140 * locals.var_t2_dn9)) / (2.0 * assign55080_e85169)), (((locals.var_c_2esipq_ndepm__blk1140_dn10 * locals.var_t2) + (locals.var_c_2esipq_ndepm__blk1140 * locals.var_t2_dn10)) / (2.0 * assign55080_e85169)), (((locals.var_c_2esipq_ndepm__blk1140_dn11 * locals.var_t2) + (locals.var_c_2esipq_ndepm__blk1140 * locals.var_t2_dn11)) / (2.0 * assign55080_e85169)), (((locals.var_c_2esipq_ndepm__blk1140_dn14 * locals.var_t2) + (locals.var_c_2esipq_ndepm__blk1140 * locals.var_t2_dn14)) / (2.0 * assign55080_e85169)),)
            }
        };
        (assign55080_e85170, assign55080_e85170_d_n0, assign55080_e85170_d_n2, assign55080_e85170_d_n4, assign55080_e85170_d_n5, assign55080_e85170_d_n6, assign55080_e85170_d_n7, assign55080_e85170_d_n8, assign55080_e85170_d_n9, assign55080_e85170_d_n10, assign55080_e85170_d_n11, assign55080_e85170_d_n14,)
    } else {
        (locals.var_ws__blk1151, locals.var_ws__blk1151_dn0, locals.var_ws__blk1151_dn2, locals.var_ws__blk1151_dn4, locals.var_ws__blk1151_dn5, locals.var_ws__blk1151_dn6, locals.var_ws__blk1151_dn7, locals.var_ws__blk1151_dn8, locals.var_ws__blk1151_dn9, locals.var_ws__blk1151_dn10, locals.var_ws__blk1151_dn11, locals.var_ws__blk1151_dn14,)
    }
};
        locals.var_ws__blk1151 = assign55080_e85172;
        locals.var_ws__blk1151_dn0 = assign55080_e85172_d_n0;
        locals.var_ws__blk1151_dn2 = assign55080_e85172_d_n2;
        locals.var_ws__blk1151_dn4 = assign55080_e85172_d_n4;
        locals.var_ws__blk1151_dn5 = assign55080_e85172_d_n5;
        locals.var_ws__blk1151_dn6 = assign55080_e85172_d_n6;
        locals.var_ws__blk1151_dn7 = assign55080_e85172_d_n7;
        locals.var_ws__blk1151_dn8 = assign55080_e85172_d_n8;
        locals.var_ws__blk1151_dn9 = assign55080_e85172_d_n9;
        locals.var_ws__blk1151_dn10 = assign55080_e85172_d_n10;
        locals.var_ws__blk1151_dn11 = assign55080_e85172_d_n11;
        locals.var_ws__blk1151_dn14 = assign55080_e85172_d_n14;

        let assign55090_e85175: f64 = if locals.var_t2 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1391 = assign55090_e85175;

        let (assign55100_e85196, assign55100_e85196_d_n0, assign55100_e85196_d_n2, assign55100_e85196_d_n4, assign55100_e85196_d_n5, assign55100_e85196_d_n6, assign55100_e85196_d_n7, assign55100_e85196_d_n8, assign55100_e85196_d_n9, assign55100_e85196_d_n10, assign55100_e85196_d_n11, assign55100_e85196_d_n14,) = {
    if (((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_guard1390 == 0.0)) && (locals.var_guard1391 != 0.0)) {
        let assign55100_e85194: f64 = (locals.var_beta * locals.var_t2);
        (assign55100_e85194, ((locals.var_beta_dn0 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn0)), ((locals.var_beta_dn2 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn2)), ((locals.var_beta_dn4 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn4)), ((locals.var_beta_dn5 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn5)), ((locals.var_beta_dn6 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn6)), ((locals.var_beta_dn7 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn7)), ((locals.var_beta_dn8 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn8)), ((locals.var_beta_dn9 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn9)), ((locals.var_beta_dn10 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn10)), ((locals.var_beta_dn11 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn11)), ((locals.var_beta_dn14 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn14)),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign55100_e85196;
        locals.var_t3_dn0 = assign55100_e85196_d_n0;
        locals.var_t3_dn2 = assign55100_e85196_d_n2;
        locals.var_t3_dn4 = assign55100_e85196_d_n4;
        locals.var_t3_dn5 = assign55100_e85196_d_n5;
        locals.var_t3_dn6 = assign55100_e85196_d_n6;
        locals.var_t3_dn7 = assign55100_e85196_d_n7;
        locals.var_t3_dn8 = assign55100_e85196_d_n8;
        locals.var_t3_dn9 = assign55100_e85196_d_n9;
        locals.var_t3_dn10 = assign55100_e85196_d_n10;
        locals.var_t3_dn11 = assign55100_e85196_d_n11;
        locals.var_t3_dn14 = assign55100_e85196_d_n14;

        let (assign55110_e85226, assign55110_e85226_d_n0, assign55110_e85226_d_n2, assign55110_e85226_d_n4, assign55110_e85226_d_n5, assign55110_e85226_d_n6, assign55110_e85226_d_n7, assign55110_e85226_d_n8, assign55110_e85226_d_n9, assign55110_e85226_d_n10, assign55110_e85226_d_n11, assign55110_e85226_d_n14,) = {
    if (((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_guard1390 == 0.0)) && (locals.var_guard1391 != 0.0)) {
        let assign55110_e85215: f64 = (locals.var_c_2esipq_ndepm__blk1140 * locals.var_beta_inv);
        let assign55110_e85217: f64 = (locals.var_t3).exp();
        let assign55110_e85219: f64 = (assign55110_e85217 - locals.var_t3);
        let assign55110_e85221: f64 = (assign55110_e85219 - 1.0);
        let assign55110_e85222: f64 = (assign55110_e85215 * assign55110_e85221);
        let assign55110_e85223: f64 = (assign55110_e85222).sqrt();
        let assign55110_e85224: f64 = (-assign55110_e85223);
        (assign55110_e85224, (-(((((locals.var_c_2esipq_ndepm__blk1140_dn0 * locals.var_beta_inv) + (locals.var_c_2esipq_ndepm__blk1140 * locals.var_beta_inv_dn0)) * assign55110_e85221) + (assign55110_e85215 * ((assign55110_e85217 * locals.var_t3_dn0) - locals.var_t3_dn0))) / (2.0 * assign55110_e85223))), (-(((((locals.var_c_2esipq_ndepm__blk1140_dn2 * locals.var_beta_inv) + (locals.var_c_2esipq_ndepm__blk1140 * locals.var_beta_inv_dn2)) * assign55110_e85221) + (assign55110_e85215 * ((assign55110_e85217 * locals.var_t3_dn2) - locals.var_t3_dn2))) / (2.0 * assign55110_e85223))), (-(((((locals.var_c_2esipq_ndepm__blk1140_dn4 * locals.var_beta_inv) + (locals.var_c_2esipq_ndepm__blk1140 * locals.var_beta_inv_dn4)) * assign55110_e85221) + (assign55110_e85215 * ((assign55110_e85217 * locals.var_t3_dn4) - locals.var_t3_dn4))) / (2.0 * assign55110_e85223))), (-(((((locals.var_c_2esipq_ndepm__blk1140_dn5 * locals.var_beta_inv) + (locals.var_c_2esipq_ndepm__blk1140 * locals.var_beta_inv_dn5)) * assign55110_e85221) + (assign55110_e85215 * ((assign55110_e85217 * locals.var_t3_dn5) - locals.var_t3_dn5))) / (2.0 * assign55110_e85223))), (-(((((locals.var_c_2esipq_ndepm__blk1140_dn6 * locals.var_beta_inv) + (locals.var_c_2esipq_ndepm__blk1140 * locals.var_beta_inv_dn6)) * assign55110_e85221) + (assign55110_e85215 * ((assign55110_e85217 * locals.var_t3_dn6) - locals.var_t3_dn6))) / (2.0 * assign55110_e85223))), (-(((((locals.var_c_2esipq_ndepm__blk1140_dn7 * locals.var_beta_inv) + (locals.var_c_2esipq_ndepm__blk1140 * locals.var_beta_inv_dn7)) * assign55110_e85221) + (assign55110_e85215 * ((assign55110_e85217 * locals.var_t3_dn7) - locals.var_t3_dn7))) / (2.0 * assign55110_e85223))), (-(((((locals.var_c_2esipq_ndepm__blk1140_dn8 * locals.var_beta_inv) + (locals.var_c_2esipq_ndepm__blk1140 * locals.var_beta_inv_dn8)) * assign55110_e85221) + (assign55110_e85215 * ((assign55110_e85217 * locals.var_t3_dn8) - locals.var_t3_dn8))) / (2.0 * assign55110_e85223))), (-(((((locals.var_c_2esipq_ndepm__blk1140_dn9 * locals.var_beta_inv) + (locals.var_c_2esipq_ndepm__blk1140 * locals.var_beta_inv_dn9)) * assign55110_e85221) + (assign55110_e85215 * ((assign55110_e85217 * locals.var_t3_dn9) - locals.var_t3_dn9))) / (2.0 * assign55110_e85223))), (-(((((locals.var_c_2esipq_ndepm__blk1140_dn10 * locals.var_beta_inv) + (locals.var_c_2esipq_ndepm__blk1140 * locals.var_beta_inv_dn10)) * assign55110_e85221) + (assign55110_e85215 * ((assign55110_e85217 * locals.var_t3_dn10) - locals.var_t3_dn10))) / (2.0 * assign55110_e85223))), (-(((((locals.var_c_2esipq_ndepm__blk1140_dn11 * locals.var_beta_inv) + (locals.var_c_2esipq_ndepm__blk1140 * locals.var_beta_inv_dn11)) * assign55110_e85221) + (assign55110_e85215 * ((assign55110_e85217 * locals.var_t3_dn11) - locals.var_t3_dn11))) / (2.0 * assign55110_e85223))), (-(((((locals.var_c_2esipq_ndepm__blk1140_dn14 * locals.var_beta_inv) + (locals.var_c_2esipq_ndepm__blk1140 * locals.var_beta_inv_dn14)) * assign55110_e85221) + (assign55110_e85215 * ((assign55110_e85217 * locals.var_t3_dn14) - locals.var_t3_dn14))) / (2.0 * assign55110_e85223))),)
    } else {
        (locals.var_ws__blk1151, locals.var_ws__blk1151_dn0, locals.var_ws__blk1151_dn2, locals.var_ws__blk1151_dn4, locals.var_ws__blk1151_dn5, locals.var_ws__blk1151_dn6, locals.var_ws__blk1151_dn7, locals.var_ws__blk1151_dn8, locals.var_ws__blk1151_dn9, locals.var_ws__blk1151_dn10, locals.var_ws__blk1151_dn11, locals.var_ws__blk1151_dn14,)
    }
};
        locals.var_ws__blk1151 = assign55110_e85226;
        locals.var_ws__blk1151_dn0 = assign55110_e85226_d_n0;
        locals.var_ws__blk1151_dn2 = assign55110_e85226_d_n2;
        locals.var_ws__blk1151_dn4 = assign55110_e85226_d_n4;
        locals.var_ws__blk1151_dn5 = assign55110_e85226_d_n5;
        locals.var_ws__blk1151_dn6 = assign55110_e85226_d_n6;
        locals.var_ws__blk1151_dn7 = assign55110_e85226_d_n7;
        locals.var_ws__blk1151_dn8 = assign55110_e85226_d_n8;
        locals.var_ws__blk1151_dn9 = assign55110_e85226_d_n9;
        locals.var_ws__blk1151_dn10 = assign55110_e85226_d_n10;
        locals.var_ws__blk1151_dn11 = assign55110_e85226_d_n11;
        locals.var_ws__blk1151_dn14 = assign55110_e85226_d_n14;

        let (assign55120_e85249, assign55120_e85249_d_n0, assign55120_e85249_d_n2, assign55120_e85249_d_n4, assign55120_e85249_d_n5, assign55120_e85249_d_n6, assign55120_e85249_d_n7, assign55120_e85249_d_n8, assign55120_e85249_d_n9, assign55120_e85249_d_n10, assign55120_e85249_d_n11, assign55120_e85249_d_n14,) = {
    if (((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_guard1390 == 0.0)) && (locals.var_guard1391 == 0.0)) {
        let assign55120_e85245: f64 = (-locals.var_beta);
        let assign55120_e85247: f64 = (assign55120_e85245 * locals.var_t2);
        (assign55120_e85247, (((-locals.var_beta_dn0) * locals.var_t2) + (assign55120_e85245 * locals.var_t2_dn0)), (((-locals.var_beta_dn2) * locals.var_t2) + (assign55120_e85245 * locals.var_t2_dn2)), (((-locals.var_beta_dn4) * locals.var_t2) + (assign55120_e85245 * locals.var_t2_dn4)), (((-locals.var_beta_dn5) * locals.var_t2) + (assign55120_e85245 * locals.var_t2_dn5)), (((-locals.var_beta_dn6) * locals.var_t2) + (assign55120_e85245 * locals.var_t2_dn6)), (((-locals.var_beta_dn7) * locals.var_t2) + (assign55120_e85245 * locals.var_t2_dn7)), (((-locals.var_beta_dn8) * locals.var_t2) + (assign55120_e85245 * locals.var_t2_dn8)), (((-locals.var_beta_dn9) * locals.var_t2) + (assign55120_e85245 * locals.var_t2_dn9)), (((-locals.var_beta_dn10) * locals.var_t2) + (assign55120_e85245 * locals.var_t2_dn10)), (((-locals.var_beta_dn11) * locals.var_t2) + (assign55120_e85245 * locals.var_t2_dn11)), (((-locals.var_beta_dn14) * locals.var_t2) + (assign55120_e85245 * locals.var_t2_dn14)),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign55120_e85249;
        locals.var_t3_dn0 = assign55120_e85249_d_n0;
        locals.var_t3_dn2 = assign55120_e85249_d_n2;
        locals.var_t3_dn4 = assign55120_e85249_d_n4;
        locals.var_t3_dn5 = assign55120_e85249_d_n5;
        locals.var_t3_dn6 = assign55120_e85249_d_n6;
        locals.var_t3_dn7 = assign55120_e85249_d_n7;
        locals.var_t3_dn8 = assign55120_e85249_d_n8;
        locals.var_t3_dn9 = assign55120_e85249_d_n9;
        locals.var_t3_dn10 = assign55120_e85249_d_n10;
        locals.var_t3_dn11 = assign55120_e85249_d_n11;
        locals.var_t3_dn14 = assign55120_e85249_d_n14;

        let (assign55130_e85279, assign55130_e85279_d_n0, assign55130_e85279_d_n2, assign55130_e85279_d_n4, assign55130_e85279_d_n5, assign55130_e85279_d_n6, assign55130_e85279_d_n7, assign55130_e85279_d_n8, assign55130_e85279_d_n9, assign55130_e85279_d_n10, assign55130_e85279_d_n11, assign55130_e85279_d_n14,) = {
    if (((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_guard1390 == 0.0)) && (locals.var_guard1391 == 0.0)) {
        let assign55130_e85269: f64 = (locals.var_c_2esipq_ndepm__blk1140 * locals.var_beta_inv);
        let assign55130_e85271: f64 = (locals.var_t3).exp();
        let assign55130_e85273: f64 = (assign55130_e85271 - locals.var_t3);
        let assign55130_e85275: f64 = (assign55130_e85273 - 1.0);
        let assign55130_e85276: f64 = (assign55130_e85269 * assign55130_e85275);
        let assign55130_e85277: f64 = (assign55130_e85276).sqrt();
        (assign55130_e85277, (((((locals.var_c_2esipq_ndepm__blk1140_dn0 * locals.var_beta_inv) + (locals.var_c_2esipq_ndepm__blk1140 * locals.var_beta_inv_dn0)) * assign55130_e85275) + (assign55130_e85269 * ((assign55130_e85271 * locals.var_t3_dn0) - locals.var_t3_dn0))) / (2.0 * assign55130_e85277)), (((((locals.var_c_2esipq_ndepm__blk1140_dn2 * locals.var_beta_inv) + (locals.var_c_2esipq_ndepm__blk1140 * locals.var_beta_inv_dn2)) * assign55130_e85275) + (assign55130_e85269 * ((assign55130_e85271 * locals.var_t3_dn2) - locals.var_t3_dn2))) / (2.0 * assign55130_e85277)), (((((locals.var_c_2esipq_ndepm__blk1140_dn4 * locals.var_beta_inv) + (locals.var_c_2esipq_ndepm__blk1140 * locals.var_beta_inv_dn4)) * assign55130_e85275) + (assign55130_e85269 * ((assign55130_e85271 * locals.var_t3_dn4) - locals.var_t3_dn4))) / (2.0 * assign55130_e85277)), (((((locals.var_c_2esipq_ndepm__blk1140_dn5 * locals.var_beta_inv) + (locals.var_c_2esipq_ndepm__blk1140 * locals.var_beta_inv_dn5)) * assign55130_e85275) + (assign55130_e85269 * ((assign55130_e85271 * locals.var_t3_dn5) - locals.var_t3_dn5))) / (2.0 * assign55130_e85277)), (((((locals.var_c_2esipq_ndepm__blk1140_dn6 * locals.var_beta_inv) + (locals.var_c_2esipq_ndepm__blk1140 * locals.var_beta_inv_dn6)) * assign55130_e85275) + (assign55130_e85269 * ((assign55130_e85271 * locals.var_t3_dn6) - locals.var_t3_dn6))) / (2.0 * assign55130_e85277)), (((((locals.var_c_2esipq_ndepm__blk1140_dn7 * locals.var_beta_inv) + (locals.var_c_2esipq_ndepm__blk1140 * locals.var_beta_inv_dn7)) * assign55130_e85275) + (assign55130_e85269 * ((assign55130_e85271 * locals.var_t3_dn7) - locals.var_t3_dn7))) / (2.0 * assign55130_e85277)), (((((locals.var_c_2esipq_ndepm__blk1140_dn8 * locals.var_beta_inv) + (locals.var_c_2esipq_ndepm__blk1140 * locals.var_beta_inv_dn8)) * assign55130_e85275) + (assign55130_e85269 * ((assign55130_e85271 * locals.var_t3_dn8) - locals.var_t3_dn8))) / (2.0 * assign55130_e85277)), (((((locals.var_c_2esipq_ndepm__blk1140_dn9 * locals.var_beta_inv) + (locals.var_c_2esipq_ndepm__blk1140 * locals.var_beta_inv_dn9)) * assign55130_e85275) + (assign55130_e85269 * ((assign55130_e85271 * locals.var_t3_dn9) - locals.var_t3_dn9))) / (2.0 * assign55130_e85277)), (((((locals.var_c_2esipq_ndepm__blk1140_dn10 * locals.var_beta_inv) + (locals.var_c_2esipq_ndepm__blk1140 * locals.var_beta_inv_dn10)) * assign55130_e85275) + (assign55130_e85269 * ((assign55130_e85271 * locals.var_t3_dn10) - locals.var_t3_dn10))) / (2.0 * assign55130_e85277)), (((((locals.var_c_2esipq_ndepm__blk1140_dn11 * locals.var_beta_inv) + (locals.var_c_2esipq_ndepm__blk1140 * locals.var_beta_inv_dn11)) * assign55130_e85275) + (assign55130_e85269 * ((assign55130_e85271 * locals.var_t3_dn11) - locals.var_t3_dn11))) / (2.0 * assign55130_e85277)), (((((locals.var_c_2esipq_ndepm__blk1140_dn14 * locals.var_beta_inv) + (locals.var_c_2esipq_ndepm__blk1140 * locals.var_beta_inv_dn14)) * assign55130_e85275) + (assign55130_e85269 * ((assign55130_e85271 * locals.var_t3_dn14) - locals.var_t3_dn14))) / (2.0 * assign55130_e85277)),)
    } else {
        (locals.var_ws__blk1151, locals.var_ws__blk1151_dn0, locals.var_ws__blk1151_dn2, locals.var_ws__blk1151_dn4, locals.var_ws__blk1151_dn5, locals.var_ws__blk1151_dn6, locals.var_ws__blk1151_dn7, locals.var_ws__blk1151_dn8, locals.var_ws__blk1151_dn9, locals.var_ws__blk1151_dn10, locals.var_ws__blk1151_dn11, locals.var_ws__blk1151_dn14,)
    }
};
        locals.var_ws__blk1151 = assign55130_e85279;
        locals.var_ws__blk1151_dn0 = assign55130_e85279_d_n0;
        locals.var_ws__blk1151_dn2 = assign55130_e85279_d_n2;
        locals.var_ws__blk1151_dn4 = assign55130_e85279_d_n4;
        locals.var_ws__blk1151_dn5 = assign55130_e85279_d_n5;
        locals.var_ws__blk1151_dn6 = assign55130_e85279_d_n6;
        locals.var_ws__blk1151_dn7 = assign55130_e85279_d_n7;
        locals.var_ws__blk1151_dn8 = assign55130_e85279_d_n8;
        locals.var_ws__blk1151_dn9 = assign55130_e85279_d_n9;
        locals.var_ws__blk1151_dn10 = assign55130_e85279_d_n10;
        locals.var_ws__blk1151_dn11 = assign55130_e85279_d_n11;
        locals.var_ws__blk1151_dn14 = assign55130_e85279_d_n14;

        let (assign55140_e85295, assign55140_e85295_d_n0, assign55140_e85295_d_n2, assign55140_e85295_d_n4, assign55140_e85295_d_n5, assign55140_e85295_d_n6, assign55140_e85295_d_n7, assign55140_e85295_d_n8, assign55140_e85295_d_n9, assign55140_e85295_d_n10, assign55140_e85295_d_n11, assign55140_e85295_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) {
        let assign55140_e85293: f64 = (locals.var_tnp__blk1152 - locals.var_ws__blk1151);
        (assign55140_e85293, (locals.var_tnp__blk1152_dn0 - locals.var_ws__blk1151_dn0), (locals.var_tnp__blk1152_dn2 - locals.var_ws__blk1151_dn2), (locals.var_tnp__blk1152_dn4 - locals.var_ws__blk1151_dn4), (locals.var_tnp__blk1152_dn5 - locals.var_ws__blk1151_dn5), (locals.var_tnp__blk1152_dn6 - locals.var_ws__blk1151_dn6), (locals.var_tnp__blk1152_dn7 - locals.var_ws__blk1151_dn7), (locals.var_tnp__blk1152_dn8 - locals.var_ws__blk1151_dn8), (locals.var_tnp__blk1152_dn9 - locals.var_ws__blk1151_dn9), (locals.var_tnp__blk1152_dn10 - locals.var_ws__blk1151_dn10), (locals.var_tnp__blk1152_dn11 - locals.var_ws__blk1151_dn11), (locals.var_tnp__blk1152_dn14 - locals.var_ws__blk1151_dn14),)
    } else {
        (locals.var_w_res, locals.var_w_res_dn0, locals.var_w_res_dn2, locals.var_w_res_dn4, locals.var_w_res_dn5, locals.var_w_res_dn6, locals.var_w_res_dn7, locals.var_w_res_dn8, locals.var_w_res_dn9, locals.var_w_res_dn10, locals.var_w_res_dn11, locals.var_w_res_dn14,)
    }
};
        locals.var_w_res = assign55140_e85295;
        locals.var_w_res_dn0 = assign55140_e85295_d_n0;
        locals.var_w_res_dn2 = assign55140_e85295_d_n2;
        locals.var_w_res_dn4 = assign55140_e85295_d_n4;
        locals.var_w_res_dn5 = assign55140_e85295_d_n5;
        locals.var_w_res_dn6 = assign55140_e85295_d_n6;
        locals.var_w_res_dn7 = assign55140_e85295_d_n7;
        locals.var_w_res_dn8 = assign55140_e85295_d_n8;
        locals.var_w_res_dn9 = assign55140_e85295_d_n9;
        locals.var_w_res_dn10 = assign55140_e85295_d_n10;
        locals.var_w_res_dn11 = assign55140_e85295_d_n11;
        locals.var_w_res_dn14 = assign55140_e85295_d_n14;

        let assign55150_e85299: f64 = 1e-16;
        let assign55150_e85304: f64 = if ((locals.var_w_res < assign55150_e85299) && (1e-16 >= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1392 = assign55150_e85304;

        let (assign55160_e85324, assign55160_e85324_d_n0, assign55160_e85324_d_n2, assign55160_e85324_d_n4, assign55160_e85324_d_n5, assign55160_e85324_d_n6, assign55160_e85324_d_n7, assign55160_e85324_d_n8, assign55160_e85324_d_n9, assign55160_e85324_d_n10, assign55160_e85324_d_n11, assign55160_e85324_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_guard1392 != 0.0)) {
        let assign55160_e85320: f64 = 1e-16;
        let assign55160_e85322: f64 = (assign55160_e85320 - locals.var_w_res);
        (assign55160_e85322, (-locals.var_w_res_dn0), (-locals.var_w_res_dn2), (-locals.var_w_res_dn4), (-locals.var_w_res_dn5), (-locals.var_w_res_dn6), (-locals.var_w_res_dn7), (-locals.var_w_res_dn8), (-locals.var_w_res_dn9), (-locals.var_w_res_dn10), (-locals.var_w_res_dn11), (-locals.var_w_res_dn14),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign55160_e85324;
        locals.var_tmf1_dn0 = assign55160_e85324_d_n0;
        locals.var_tmf1_dn2 = assign55160_e85324_d_n2;
        locals.var_tmf1_dn4 = assign55160_e85324_d_n4;
        locals.var_tmf1_dn5 = assign55160_e85324_d_n5;
        locals.var_tmf1_dn6 = assign55160_e85324_d_n6;
        locals.var_tmf1_dn7 = assign55160_e85324_d_n7;
        locals.var_tmf1_dn8 = assign55160_e85324_d_n8;
        locals.var_tmf1_dn9 = assign55160_e85324_d_n9;
        locals.var_tmf1_dn10 = assign55160_e85324_d_n10;
        locals.var_tmf1_dn11 = assign55160_e85324_d_n11;
        locals.var_tmf1_dn14 = assign55160_e85324_d_n14;

        let (assign55170_e85342, assign55170_e85342_d_n0, assign55170_e85342_d_n2, assign55170_e85342_d_n4, assign55170_e85342_d_n5, assign55170_e85342_d_n6, assign55170_e85342_d_n7, assign55170_e85342_d_n8, assign55170_e85342_d_n9, assign55170_e85342_d_n10, assign55170_e85342_d_n11, assign55170_e85342_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_guard1392 != 0.0)) {
        let assign55170_e85340: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign55170_e85340, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)), ((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)), ((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)), ((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn4, locals.var_x2_dn5, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn8, locals.var_x2_dn9, locals.var_x2_dn10, locals.var_x2_dn11, locals.var_x2_dn14,)
    }
};
        locals.var_x2 = assign55170_e85342;
        locals.var_x2_dn0 = assign55170_e85342_d_n0;
        locals.var_x2_dn2 = assign55170_e85342_d_n2;
        locals.var_x2_dn4 = assign55170_e85342_d_n4;
        locals.var_x2_dn5 = assign55170_e85342_d_n5;
        locals.var_x2_dn6 = assign55170_e85342_d_n6;
        locals.var_x2_dn7 = assign55170_e85342_d_n7;
        locals.var_x2_dn8 = assign55170_e85342_d_n8;
        locals.var_x2_dn9 = assign55170_e85342_d_n9;
        locals.var_x2_dn10 = assign55170_e85342_d_n10;
        locals.var_x2_dn11 = assign55170_e85342_d_n11;
        locals.var_x2_dn14 = assign55170_e85342_d_n14;

        let (assign55180_e85360, assign55180_e85360_d_n0, assign55180_e85360_d_n2, assign55180_e85360_d_n4, assign55180_e85360_d_n5, assign55180_e85360_d_n6, assign55180_e85360_d_n7, assign55180_e85360_d_n8, assign55180_e85360_d_n9, assign55180_e85360_d_n10, assign55180_e85360_d_n11, assign55180_e85360_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_guard1392 != 0.0)) {
        let assign55180_e85358: f64 = (1e-16 * 1e-16);
        (assign55180_e85358, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn4, locals.var_xmax2_dn5, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn8, locals.var_xmax2_dn9, locals.var_xmax2_dn10, locals.var_xmax2_dn11, locals.var_xmax2_dn14,)
    }
};
        locals.var_xmax2 = assign55180_e85360;
        locals.var_xmax2_dn0 = assign55180_e85360_d_n0;
        locals.var_xmax2_dn2 = assign55180_e85360_d_n2;
        locals.var_xmax2_dn4 = assign55180_e85360_d_n4;
        locals.var_xmax2_dn5 = assign55180_e85360_d_n5;
        locals.var_xmax2_dn6 = assign55180_e85360_d_n6;
        locals.var_xmax2_dn7 = assign55180_e85360_d_n7;
        locals.var_xmax2_dn8 = assign55180_e85360_d_n8;
        locals.var_xmax2_dn9 = assign55180_e85360_d_n9;
        locals.var_xmax2_dn10 = assign55180_e85360_d_n10;
        locals.var_xmax2_dn11 = assign55180_e85360_d_n11;
        locals.var_xmax2_dn14 = assign55180_e85360_d_n14;

        let (assign55190_e85376, assign55190_e85376_d_n0, assign55190_e85376_d_n2, assign55190_e85376_d_n4, assign55190_e85376_d_n5, assign55190_e85376_d_n6, assign55190_e85376_d_n7, assign55190_e85376_d_n8, assign55190_e85376_d_n9, assign55190_e85376_d_n10, assign55190_e85376_d_n11, assign55190_e85376_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_guard1392 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign55190_e85376;
        locals.var_xp_dn0 = assign55190_e85376_d_n0;
        locals.var_xp_dn2 = assign55190_e85376_d_n2;
        locals.var_xp_dn4 = assign55190_e85376_d_n4;
        locals.var_xp_dn5 = assign55190_e85376_d_n5;
        locals.var_xp_dn6 = assign55190_e85376_d_n6;
        locals.var_xp_dn7 = assign55190_e85376_d_n7;
        locals.var_xp_dn8 = assign55190_e85376_d_n8;
        locals.var_xp_dn9 = assign55190_e85376_d_n9;
        locals.var_xp_dn10 = assign55190_e85376_d_n10;
        locals.var_xp_dn11 = assign55190_e85376_d_n11;
        locals.var_xp_dn14 = assign55190_e85376_d_n14;

        let (assign55200_e85392, assign55200_e85392_d_n0, assign55200_e85392_d_n2, assign55200_e85392_d_n4, assign55200_e85392_d_n5, assign55200_e85392_d_n6, assign55200_e85392_d_n7, assign55200_e85392_d_n8, assign55200_e85392_d_n9, assign55200_e85392_d_n10, assign55200_e85392_d_n11, assign55200_e85392_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_guard1392 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign55200_e85392;
        locals.var_xmp_dn0 = assign55200_e85392_d_n0;
        locals.var_xmp_dn2 = assign55200_e85392_d_n2;
        locals.var_xmp_dn4 = assign55200_e85392_d_n4;
        locals.var_xmp_dn5 = assign55200_e85392_d_n5;
        locals.var_xmp_dn6 = assign55200_e85392_d_n6;
        locals.var_xmp_dn7 = assign55200_e85392_d_n7;
        locals.var_xmp_dn8 = assign55200_e85392_d_n8;
        locals.var_xmp_dn9 = assign55200_e85392_d_n9;
        locals.var_xmp_dn10 = assign55200_e85392_d_n10;
        locals.var_xmp_dn11 = assign55200_e85392_d_n11;
        locals.var_xmp_dn14 = assign55200_e85392_d_n14;

        let (assign55210_e85408,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_guard1392 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign55210_e85408;

        let (assign55220_e85424,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_guard1392 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign55220_e85424;

        let (assign55230_e85440, assign55230_e85440_d_n0, assign55230_e85440_d_n2, assign55230_e85440_d_n4, assign55230_e85440_d_n5, assign55230_e85440_d_n6, assign55230_e85440_d_n7, assign55230_e85440_d_n8, assign55230_e85440_d_n9, assign55230_e85440_d_n10, assign55230_e85440_d_n11, assign55230_e85440_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_guard1392 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign55230_e85440;
        locals.var_arg_dn0 = assign55230_e85440_d_n0;
        locals.var_arg_dn2 = assign55230_e85440_d_n2;
        locals.var_arg_dn4 = assign55230_e85440_d_n4;
        locals.var_arg_dn5 = assign55230_e85440_d_n5;
        locals.var_arg_dn6 = assign55230_e85440_d_n6;
        locals.var_arg_dn7 = assign55230_e85440_d_n7;
        locals.var_arg_dn8 = assign55230_e85440_d_n8;
        locals.var_arg_dn9 = assign55230_e85440_d_n9;
        locals.var_arg_dn10 = assign55230_e85440_d_n10;
        locals.var_arg_dn11 = assign55230_e85440_d_n11;
        locals.var_arg_dn14 = assign55230_e85440_d_n14;

        let (assign55240_e85456, assign55240_e85456_d_n0, assign55240_e85456_d_n2, assign55240_e85456_d_n4, assign55240_e85456_d_n5, assign55240_e85456_d_n6, assign55240_e85456_d_n7, assign55240_e85456_d_n8, assign55240_e85456_d_n9, assign55240_e85456_d_n10, assign55240_e85456_d_n11, assign55240_e85456_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_guard1392 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign55240_e85456;
        locals.var_dnm_dn0 = assign55240_e85456_d_n0;
        locals.var_dnm_dn2 = assign55240_e85456_d_n2;
        locals.var_dnm_dn4 = assign55240_e85456_d_n4;
        locals.var_dnm_dn5 = assign55240_e85456_d_n5;
        locals.var_dnm_dn6 = assign55240_e85456_d_n6;
        locals.var_dnm_dn7 = assign55240_e85456_d_n7;
        locals.var_dnm_dn8 = assign55240_e85456_d_n8;
        locals.var_dnm_dn9 = assign55240_e85456_d_n9;
        locals.var_dnm_dn10 = assign55240_e85456_d_n10;
        locals.var_dnm_dn11 = assign55240_e85456_d_n11;
        locals.var_dnm_dn14 = assign55240_e85456_d_n14;

        let (assign55250_e85474, assign55250_e85474_d_n0, assign55250_e85474_d_n2, assign55250_e85474_d_n4, assign55250_e85474_d_n5, assign55250_e85474_d_n6, assign55250_e85474_d_n7, assign55250_e85474_d_n8, assign55250_e85474_d_n9, assign55250_e85474_d_n10, assign55250_e85474_d_n11, assign55250_e85474_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_guard1392 != 0.0)) {
        let assign55250_e85472: f64 = (locals.var_xp * locals.var_x2);
        (assign55250_e85472, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign55250_e85474;
        locals.var_xp_dn0 = assign55250_e85474_d_n0;
        locals.var_xp_dn2 = assign55250_e85474_d_n2;
        locals.var_xp_dn4 = assign55250_e85474_d_n4;
        locals.var_xp_dn5 = assign55250_e85474_d_n5;
        locals.var_xp_dn6 = assign55250_e85474_d_n6;
        locals.var_xp_dn7 = assign55250_e85474_d_n7;
        locals.var_xp_dn8 = assign55250_e85474_d_n8;
        locals.var_xp_dn9 = assign55250_e85474_d_n9;
        locals.var_xp_dn10 = assign55250_e85474_d_n10;
        locals.var_xp_dn11 = assign55250_e85474_d_n11;
        locals.var_xp_dn14 = assign55250_e85474_d_n14;

        let (assign55260_e85492, assign55260_e85492_d_n0, assign55260_e85492_d_n2, assign55260_e85492_d_n4, assign55260_e85492_d_n5, assign55260_e85492_d_n6, assign55260_e85492_d_n7, assign55260_e85492_d_n8, assign55260_e85492_d_n9, assign55260_e85492_d_n10, assign55260_e85492_d_n11, assign55260_e85492_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_guard1392 != 0.0)) {
        let assign55260_e85490: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign55260_e85490, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign55260_e85492;
        locals.var_xmp_dn0 = assign55260_e85492_d_n0;
        locals.var_xmp_dn2 = assign55260_e85492_d_n2;
        locals.var_xmp_dn4 = assign55260_e85492_d_n4;
        locals.var_xmp_dn5 = assign55260_e85492_d_n5;
        locals.var_xmp_dn6 = assign55260_e85492_d_n6;
        locals.var_xmp_dn7 = assign55260_e85492_d_n7;
        locals.var_xmp_dn8 = assign55260_e85492_d_n8;
        locals.var_xmp_dn9 = assign55260_e85492_d_n9;
        locals.var_xmp_dn10 = assign55260_e85492_d_n10;
        locals.var_xmp_dn11 = assign55260_e85492_d_n11;
        locals.var_xmp_dn14 = assign55260_e85492_d_n14;

        let (assign55270_e85510, assign55270_e85510_d_n0, assign55270_e85510_d_n2, assign55270_e85510_d_n4, assign55270_e85510_d_n5, assign55270_e85510_d_n6, assign55270_e85510_d_n7, assign55270_e85510_d_n8, assign55270_e85510_d_n9, assign55270_e85510_d_n10, assign55270_e85510_d_n11, assign55270_e85510_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_guard1392 != 0.0)) {
        let assign55270_e85508: f64 = (locals.var_xp * locals.var_x2);
        (assign55270_e85508, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign55270_e85510;
        locals.var_xp_dn0 = assign55270_e85510_d_n0;
        locals.var_xp_dn2 = assign55270_e85510_d_n2;
        locals.var_xp_dn4 = assign55270_e85510_d_n4;
        locals.var_xp_dn5 = assign55270_e85510_d_n5;
        locals.var_xp_dn6 = assign55270_e85510_d_n6;
        locals.var_xp_dn7 = assign55270_e85510_d_n7;
        locals.var_xp_dn8 = assign55270_e85510_d_n8;
        locals.var_xp_dn9 = assign55270_e85510_d_n9;
        locals.var_xp_dn10 = assign55270_e85510_d_n10;
        locals.var_xp_dn11 = assign55270_e85510_d_n11;
        locals.var_xp_dn14 = assign55270_e85510_d_n14;

    }

    pub(super) fn stamp_transient_block_190(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign55280_e85528, assign55280_e85528_d_n0, assign55280_e85528_d_n2, assign55280_e85528_d_n4, assign55280_e85528_d_n5, assign55280_e85528_d_n6, assign55280_e85528_d_n7, assign55280_e85528_d_n8, assign55280_e85528_d_n9, assign55280_e85528_d_n10, assign55280_e85528_d_n11, assign55280_e85528_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_guard1392 != 0.0)) {
        let assign55280_e85526: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign55280_e85526, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign55280_e85528;
        locals.var_xmp_dn0 = assign55280_e85528_d_n0;
        locals.var_xmp_dn2 = assign55280_e85528_d_n2;
        locals.var_xmp_dn4 = assign55280_e85528_d_n4;
        locals.var_xmp_dn5 = assign55280_e85528_d_n5;
        locals.var_xmp_dn6 = assign55280_e85528_d_n6;
        locals.var_xmp_dn7 = assign55280_e85528_d_n7;
        locals.var_xmp_dn8 = assign55280_e85528_d_n8;
        locals.var_xmp_dn9 = assign55280_e85528_d_n9;
        locals.var_xmp_dn10 = assign55280_e85528_d_n10;
        locals.var_xmp_dn11 = assign55280_e85528_d_n11;
        locals.var_xmp_dn14 = assign55280_e85528_d_n14;

        let (assign55290_e85546, assign55290_e85546_d_n0, assign55290_e85546_d_n2, assign55290_e85546_d_n4, assign55290_e85546_d_n5, assign55290_e85546_d_n6, assign55290_e85546_d_n7, assign55290_e85546_d_n8, assign55290_e85546_d_n9, assign55290_e85546_d_n10, assign55290_e85546_d_n11, assign55290_e85546_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_guard1392 != 0.0)) {
        let assign55290_e85544: f64 = (locals.var_xp + locals.var_xmp);
        (assign55290_e85544, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn4 + locals.var_xmp_dn4), (locals.var_xp_dn5 + locals.var_xmp_dn5), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn8 + locals.var_xmp_dn8), (locals.var_xp_dn9 + locals.var_xmp_dn9), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn11 + locals.var_xmp_dn11), (locals.var_xp_dn14 + locals.var_xmp_dn14),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign55290_e85546;
        locals.var_arg_dn0 = assign55290_e85546_d_n0;
        locals.var_arg_dn2 = assign55290_e85546_d_n2;
        locals.var_arg_dn4 = assign55290_e85546_d_n4;
        locals.var_arg_dn5 = assign55290_e85546_d_n5;
        locals.var_arg_dn6 = assign55290_e85546_d_n6;
        locals.var_arg_dn7 = assign55290_e85546_d_n7;
        locals.var_arg_dn8 = assign55290_e85546_d_n8;
        locals.var_arg_dn9 = assign55290_e85546_d_n9;
        locals.var_arg_dn10 = assign55290_e85546_d_n10;
        locals.var_arg_dn11 = assign55290_e85546_d_n11;
        locals.var_arg_dn14 = assign55290_e85546_d_n14;

        let (assign55300_e85562, assign55300_e85562_d_n0, assign55300_e85562_d_n2, assign55300_e85562_d_n4, assign55300_e85562_d_n5, assign55300_e85562_d_n6, assign55300_e85562_d_n7, assign55300_e85562_d_n8, assign55300_e85562_d_n9, assign55300_e85562_d_n10, assign55300_e85562_d_n11, assign55300_e85562_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_guard1392 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign55300_e85562;
        locals.var_dnm_dn0 = assign55300_e85562_d_n0;
        locals.var_dnm_dn2 = assign55300_e85562_d_n2;
        locals.var_dnm_dn4 = assign55300_e85562_d_n4;
        locals.var_dnm_dn5 = assign55300_e85562_d_n5;
        locals.var_dnm_dn6 = assign55300_e85562_d_n6;
        locals.var_dnm_dn7 = assign55300_e85562_d_n7;
        locals.var_dnm_dn8 = assign55300_e85562_d_n8;
        locals.var_dnm_dn9 = assign55300_e85562_d_n9;
        locals.var_dnm_dn10 = assign55300_e85562_d_n10;
        locals.var_dnm_dn11 = assign55300_e85562_d_n11;
        locals.var_dnm_dn14 = assign55300_e85562_d_n14;

        let assign55310_e85577: f64 = if ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard1393 = assign55310_e85577;

        let assign55320_e85580: f64 = if 2.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1394 = assign55320_e85580;

        let (assign55330_e85600,) = {
    if ((((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_guard1392 != 0.0)) && (locals.var_guard1393 != 0.0)) && (locals.var_guard1394 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign55330_e85600;

        let assign55340_e85603: f64 = if 2.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard1395 = assign55340_e85603;

        let (assign55350_e85626,) = {
    if (((((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_guard1392 != 0.0)) && (locals.var_guard1393 != 0.0)) && (locals.var_guard1394 == 0.0)) && (locals.var_guard1395 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign55350_e85626;

        let assign55360_e85629: f64 = if 2.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard1396 = assign55360_e85629;

        let (assign55370_e85655,) = {
    if ((((((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_guard1392 != 0.0)) && (locals.var_guard1393 != 0.0)) && (locals.var_guard1394 == 0.0)) && (locals.var_guard1395 == 0.0)) && (locals.var_guard1396 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign55370_e85655;

        let assign55380_e85658: f64 = if 2.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard1397 = assign55380_e85658;

        let (assign55390_e85687,) = {
    if (((((((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_guard1392 != 0.0)) && (locals.var_guard1393 != 0.0)) && (locals.var_guard1394 == 0.0)) && (locals.var_guard1395 == 0.0)) && (locals.var_guard1396 == 0.0)) && (locals.var_guard1397 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign55390_e85687;

        let (assign55400_e85705,) = {
    if (((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_guard1392 != 0.0)) && (locals.var_guard1393 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign55400_e85705;

        let mut assign55410_loop_guard: usize = 0;
        while {
            let assign55410_cond_e85724: f64 = if ((((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_guard1392 != 0.0)) && (locals.var_guard1393 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign55410_cond_e85724 != 0.0
        } {
            assign55410_loop_guard += 1;
            assert!(assign55410_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign55410_body0_e85743, assign55410_body0_e85743_d_n0, assign55410_body0_e85743_d_n2, assign55410_body0_e85743_d_n4, assign55410_body0_e85743_d_n5, assign55410_body0_e85743_d_n6, assign55410_body0_e85743_d_n7, assign55410_body0_e85743_d_n8, assign55410_body0_e85743_d_n9, assign55410_body0_e85743_d_n10, assign55410_body0_e85743_d_n11, assign55410_body0_e85743_d_n14,) = {
    if (((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_guard1392 != 0.0)) && (locals.var_guard1393 != 0.0)) {
        let assign55410_body0_e85741: f64 = (locals.var_dnm).sqrt();
        (assign55410_body0_e85741, (locals.var_dnm_dn0 / (2.0 * assign55410_body0_e85741)), (locals.var_dnm_dn2 / (2.0 * assign55410_body0_e85741)), (locals.var_dnm_dn4 / (2.0 * assign55410_body0_e85741)), (locals.var_dnm_dn5 / (2.0 * assign55410_body0_e85741)), (locals.var_dnm_dn6 / (2.0 * assign55410_body0_e85741)), (locals.var_dnm_dn7 / (2.0 * assign55410_body0_e85741)), (locals.var_dnm_dn8 / (2.0 * assign55410_body0_e85741)), (locals.var_dnm_dn9 / (2.0 * assign55410_body0_e85741)), (locals.var_dnm_dn10 / (2.0 * assign55410_body0_e85741)), (locals.var_dnm_dn11 / (2.0 * assign55410_body0_e85741)), (locals.var_dnm_dn14 / (2.0 * assign55410_body0_e85741)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
            locals.var_dnm = assign55410_body0_e85743;
            locals.var_dnm_dn0 = assign55410_body0_e85743_d_n0;
            locals.var_dnm_dn2 = assign55410_body0_e85743_d_n2;
            locals.var_dnm_dn4 = assign55410_body0_e85743_d_n4;
            locals.var_dnm_dn5 = assign55410_body0_e85743_d_n5;
            locals.var_dnm_dn6 = assign55410_body0_e85743_d_n6;
            locals.var_dnm_dn7 = assign55410_body0_e85743_d_n7;
            locals.var_dnm_dn8 = assign55410_body0_e85743_d_n8;
            locals.var_dnm_dn9 = assign55410_body0_e85743_d_n9;
            locals.var_dnm_dn10 = assign55410_body0_e85743_d_n10;
            locals.var_dnm_dn11 = assign55410_body0_e85743_d_n11;
            locals.var_dnm_dn14 = assign55410_body0_e85743_d_n14;
            let (assign55410_body1_e85763,) = {
    if (((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_guard1392 != 0.0)) && (locals.var_guard1393 != 0.0)) {
        let assign55410_body1_e85761: f64 = (locals.var_m0 + 1.0);
        (assign55410_body1_e85761,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign55410_body1_e85763;
        }

        let (assign55420_e85793, assign55420_e85793_d_n0, assign55420_e85793_d_n2, assign55420_e85793_d_n4, assign55420_e85793_d_n5, assign55420_e85793_d_n6, assign55420_e85793_d_n7, assign55420_e85793_d_n8, assign55420_e85793_d_n9, assign55420_e85793_d_n10, assign55420_e85793_d_n11, assign55420_e85793_d_n14,) = {
    if (((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_guard1392 != 0.0)) && (locals.var_guard1393 == 0.0)) {
        let (assign55420_e85791, assign55420_e85791_d_n0, assign55420_e85791_d_n2, assign55420_e85791_d_n4, assign55420_e85791_d_n5, assign55420_e85791_d_n6, assign55420_e85791_d_n7, assign55420_e85791_d_n8, assign55420_e85791_d_n9, assign55420_e85791_d_n10, assign55420_e85791_d_n11, assign55420_e85791_d_n14,) = {
            if (locals.var_dnm == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign55420_e85788: f64 = (2.0 * 2.0);
                let assign55420_e85789: f64 = (1.0 / assign55420_e85788);
                let assign55420_e85790: f64 = (locals.var_dnm).powf(assign55420_e85789);
                (assign55420_e85790, if 0.0 == 0.0 && ((assign55420_e85789) as f64).is_finite() && ((assign55420_e85789) as f64).fract() == 0.0 { if assign55420_e85789 == 0.0 { 0.0 } else { (assign55420_e85789 * ((locals.var_dnm).powf(assign55420_e85789 - 1.0) * locals.var_dnm_dn0)) } } else { (assign55420_e85790 * (assign55420_e85789 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign55420_e85789) as f64).is_finite() && ((assign55420_e85789) as f64).fract() == 0.0 { if assign55420_e85789 == 0.0 { 0.0 } else { (assign55420_e85789 * ((locals.var_dnm).powf(assign55420_e85789 - 1.0) * locals.var_dnm_dn2)) } } else { (assign55420_e85790 * (assign55420_e85789 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign55420_e85789) as f64).is_finite() && ((assign55420_e85789) as f64).fract() == 0.0 { if assign55420_e85789 == 0.0 { 0.0 } else { (assign55420_e85789 * ((locals.var_dnm).powf(assign55420_e85789 - 1.0) * locals.var_dnm_dn4)) } } else { (assign55420_e85790 * (assign55420_e85789 * (locals.var_dnm_dn4 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign55420_e85789) as f64).is_finite() && ((assign55420_e85789) as f64).fract() == 0.0 { if assign55420_e85789 == 0.0 { 0.0 } else { (assign55420_e85789 * ((locals.var_dnm).powf(assign55420_e85789 - 1.0) * locals.var_dnm_dn5)) } } else { (assign55420_e85790 * (assign55420_e85789 * (locals.var_dnm_dn5 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign55420_e85789) as f64).is_finite() && ((assign55420_e85789) as f64).fract() == 0.0 { if assign55420_e85789 == 0.0 { 0.0 } else { (assign55420_e85789 * ((locals.var_dnm).powf(assign55420_e85789 - 1.0) * locals.var_dnm_dn6)) } } else { (assign55420_e85790 * (assign55420_e85789 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign55420_e85789) as f64).is_finite() && ((assign55420_e85789) as f64).fract() == 0.0 { if assign55420_e85789 == 0.0 { 0.0 } else { (assign55420_e85789 * ((locals.var_dnm).powf(assign55420_e85789 - 1.0) * locals.var_dnm_dn7)) } } else { (assign55420_e85790 * (assign55420_e85789 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign55420_e85789) as f64).is_finite() && ((assign55420_e85789) as f64).fract() == 0.0 { if assign55420_e85789 == 0.0 { 0.0 } else { (assign55420_e85789 * ((locals.var_dnm).powf(assign55420_e85789 - 1.0) * locals.var_dnm_dn8)) } } else { (assign55420_e85790 * (assign55420_e85789 * (locals.var_dnm_dn8 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign55420_e85789) as f64).is_finite() && ((assign55420_e85789) as f64).fract() == 0.0 { if assign55420_e85789 == 0.0 { 0.0 } else { (assign55420_e85789 * ((locals.var_dnm).powf(assign55420_e85789 - 1.0) * locals.var_dnm_dn9)) } } else { (assign55420_e85790 * (assign55420_e85789 * (locals.var_dnm_dn9 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign55420_e85789) as f64).is_finite() && ((assign55420_e85789) as f64).fract() == 0.0 { if assign55420_e85789 == 0.0 { 0.0 } else { (assign55420_e85789 * ((locals.var_dnm).powf(assign55420_e85789 - 1.0) * locals.var_dnm_dn10)) } } else { (assign55420_e85790 * (assign55420_e85789 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign55420_e85789) as f64).is_finite() && ((assign55420_e85789) as f64).fract() == 0.0 { if assign55420_e85789 == 0.0 { 0.0 } else { (assign55420_e85789 * ((locals.var_dnm).powf(assign55420_e85789 - 1.0) * locals.var_dnm_dn11)) } } else { (assign55420_e85790 * (assign55420_e85789 * (locals.var_dnm_dn11 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign55420_e85789) as f64).is_finite() && ((assign55420_e85789) as f64).fract() == 0.0 { if assign55420_e85789 == 0.0 { 0.0 } else { (assign55420_e85789 * ((locals.var_dnm).powf(assign55420_e85789 - 1.0) * locals.var_dnm_dn14)) } } else { (assign55420_e85790 * (assign55420_e85789 * (locals.var_dnm_dn14 / locals.var_dnm))) },)
            }
        };
        (assign55420_e85791, assign55420_e85791_d_n0, assign55420_e85791_d_n2, assign55420_e85791_d_n4, assign55420_e85791_d_n5, assign55420_e85791_d_n6, assign55420_e85791_d_n7, assign55420_e85791_d_n8, assign55420_e85791_d_n9, assign55420_e85791_d_n10, assign55420_e85791_d_n11, assign55420_e85791_d_n14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign55420_e85793;
        locals.var_dnm_dn0 = assign55420_e85793_d_n0;
        locals.var_dnm_dn2 = assign55420_e85793_d_n2;
        locals.var_dnm_dn4 = assign55420_e85793_d_n4;
        locals.var_dnm_dn5 = assign55420_e85793_d_n5;
        locals.var_dnm_dn6 = assign55420_e85793_d_n6;
        locals.var_dnm_dn7 = assign55420_e85793_d_n7;
        locals.var_dnm_dn8 = assign55420_e85793_d_n8;
        locals.var_dnm_dn9 = assign55420_e85793_d_n9;
        locals.var_dnm_dn10 = assign55420_e85793_d_n10;
        locals.var_dnm_dn11 = assign55420_e85793_d_n11;
        locals.var_dnm_dn14 = assign55420_e85793_d_n14;

        let (assign55430_e85811, assign55430_e85811_d_n0, assign55430_e85811_d_n2, assign55430_e85811_d_n4, assign55430_e85811_d_n5, assign55430_e85811_d_n6, assign55430_e85811_d_n7, assign55430_e85811_d_n8, assign55430_e85811_d_n9, assign55430_e85811_d_n10, assign55430_e85811_d_n11, assign55430_e85811_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_guard1392 != 0.0)) {
        let assign55430_e85809: f64 = (1.0 / locals.var_dnm);
        (assign55430_e85809, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn4 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn5 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn8 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn9 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn11 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn14 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign55430_e85811;
        locals.var_dnm_dn0 = assign55430_e85811_d_n0;
        locals.var_dnm_dn2 = assign55430_e85811_d_n2;
        locals.var_dnm_dn4 = assign55430_e85811_d_n4;
        locals.var_dnm_dn5 = assign55430_e85811_d_n5;
        locals.var_dnm_dn6 = assign55430_e85811_d_n6;
        locals.var_dnm_dn7 = assign55430_e85811_d_n7;
        locals.var_dnm_dn8 = assign55430_e85811_d_n8;
        locals.var_dnm_dn9 = assign55430_e85811_d_n9;
        locals.var_dnm_dn10 = assign55430_e85811_d_n10;
        locals.var_dnm_dn11 = assign55430_e85811_d_n11;
        locals.var_dnm_dn14 = assign55430_e85811_d_n14;

        let (assign55440_e85831, assign55440_e85831_d_n0, assign55440_e85831_d_n2, assign55440_e85831_d_n4, assign55440_e85831_d_n5, assign55440_e85831_d_n6, assign55440_e85831_d_n7, assign55440_e85831_d_n8, assign55440_e85831_d_n9, assign55440_e85831_d_n10, assign55440_e85831_d_n11, assign55440_e85831_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_guard1392 != 0.0)) {
        let assign55440_e85827: f64 = (locals.var_tmf1 * 1e-16);
        let assign55440_e85829: f64 = (assign55440_e85827 * locals.var_dnm);
        (assign55440_e85829, (((locals.var_tmf1_dn0 * 1e-16) * locals.var_dnm) + (assign55440_e85827 * locals.var_dnm_dn0)), (((locals.var_tmf1_dn2 * 1e-16) * locals.var_dnm) + (assign55440_e85827 * locals.var_dnm_dn2)), (((locals.var_tmf1_dn4 * 1e-16) * locals.var_dnm) + (assign55440_e85827 * locals.var_dnm_dn4)), (((locals.var_tmf1_dn5 * 1e-16) * locals.var_dnm) + (assign55440_e85827 * locals.var_dnm_dn5)), (((locals.var_tmf1_dn6 * 1e-16) * locals.var_dnm) + (assign55440_e85827 * locals.var_dnm_dn6)), (((locals.var_tmf1_dn7 * 1e-16) * locals.var_dnm) + (assign55440_e85827 * locals.var_dnm_dn7)), (((locals.var_tmf1_dn8 * 1e-16) * locals.var_dnm) + (assign55440_e85827 * locals.var_dnm_dn8)), (((locals.var_tmf1_dn9 * 1e-16) * locals.var_dnm) + (assign55440_e85827 * locals.var_dnm_dn9)), (((locals.var_tmf1_dn10 * 1e-16) * locals.var_dnm) + (assign55440_e85827 * locals.var_dnm_dn10)), (((locals.var_tmf1_dn11 * 1e-16) * locals.var_dnm) + (assign55440_e85827 * locals.var_dnm_dn11)), (((locals.var_tmf1_dn14 * 1e-16) * locals.var_dnm) + (assign55440_e85827 * locals.var_dnm_dn14)),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn11, locals.var_tmf0_dn14,)
    }
};
        locals.var_tmf0 = assign55440_e85831;
        locals.var_tmf0_dn0 = assign55440_e85831_d_n0;
        locals.var_tmf0_dn2 = assign55440_e85831_d_n2;
        locals.var_tmf0_dn4 = assign55440_e85831_d_n4;
        locals.var_tmf0_dn5 = assign55440_e85831_d_n5;
        locals.var_tmf0_dn6 = assign55440_e85831_d_n6;
        locals.var_tmf0_dn7 = assign55440_e85831_d_n7;
        locals.var_tmf0_dn8 = assign55440_e85831_d_n8;
        locals.var_tmf0_dn9 = assign55440_e85831_d_n9;
        locals.var_tmf0_dn10 = assign55440_e85831_d_n10;
        locals.var_tmf0_dn11 = assign55440_e85831_d_n11;
        locals.var_tmf0_dn14 = assign55440_e85831_d_n14;

        let (assign55450_e85853, assign55450_e85853_d_n0, assign55450_e85853_d_n2, assign55450_e85853_d_n4, assign55450_e85853_d_n5, assign55450_e85853_d_n6, assign55450_e85853_d_n7, assign55450_e85853_d_n8, assign55450_e85853_d_n9, assign55450_e85853_d_n10, assign55450_e85853_d_n11, assign55450_e85853_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_guard1392 != 0.0)) {
        let assign55450_e85847: f64 = (1e-16 * locals.var_xmp);
        let assign55450_e85849: f64 = (assign55450_e85847 * locals.var_dnm);
        let assign55450_e85851: f64 = (assign55450_e85849 / locals.var_arg);
        (assign55450_e85851, ((((((1e-16 * locals.var_xmp_dn0) * locals.var_dnm) + (assign55450_e85847 * locals.var_dnm_dn0)) * locals.var_arg) - (assign55450_e85849 * locals.var_arg_dn0)) / (locals.var_arg * locals.var_arg)), ((((((1e-16 * locals.var_xmp_dn2) * locals.var_dnm) + (assign55450_e85847 * locals.var_dnm_dn2)) * locals.var_arg) - (assign55450_e85849 * locals.var_arg_dn2)) / (locals.var_arg * locals.var_arg)), ((((((1e-16 * locals.var_xmp_dn4) * locals.var_dnm) + (assign55450_e85847 * locals.var_dnm_dn4)) * locals.var_arg) - (assign55450_e85849 * locals.var_arg_dn4)) / (locals.var_arg * locals.var_arg)), ((((((1e-16 * locals.var_xmp_dn5) * locals.var_dnm) + (assign55450_e85847 * locals.var_dnm_dn5)) * locals.var_arg) - (assign55450_e85849 * locals.var_arg_dn5)) / (locals.var_arg * locals.var_arg)), ((((((1e-16 * locals.var_xmp_dn6) * locals.var_dnm) + (assign55450_e85847 * locals.var_dnm_dn6)) * locals.var_arg) - (assign55450_e85849 * locals.var_arg_dn6)) / (locals.var_arg * locals.var_arg)), ((((((1e-16 * locals.var_xmp_dn7) * locals.var_dnm) + (assign55450_e85847 * locals.var_dnm_dn7)) * locals.var_arg) - (assign55450_e85849 * locals.var_arg_dn7)) / (locals.var_arg * locals.var_arg)), ((((((1e-16 * locals.var_xmp_dn8) * locals.var_dnm) + (assign55450_e85847 * locals.var_dnm_dn8)) * locals.var_arg) - (assign55450_e85849 * locals.var_arg_dn8)) / (locals.var_arg * locals.var_arg)), ((((((1e-16 * locals.var_xmp_dn9) * locals.var_dnm) + (assign55450_e85847 * locals.var_dnm_dn9)) * locals.var_arg) - (assign55450_e85849 * locals.var_arg_dn9)) / (locals.var_arg * locals.var_arg)), ((((((1e-16 * locals.var_xmp_dn10) * locals.var_dnm) + (assign55450_e85847 * locals.var_dnm_dn10)) * locals.var_arg) - (assign55450_e85849 * locals.var_arg_dn10)) / (locals.var_arg * locals.var_arg)), ((((((1e-16 * locals.var_xmp_dn11) * locals.var_dnm) + (assign55450_e85847 * locals.var_dnm_dn11)) * locals.var_arg) - (assign55450_e85849 * locals.var_arg_dn11)) / (locals.var_arg * locals.var_arg)), ((((((1e-16 * locals.var_xmp_dn14) * locals.var_dnm) + (assign55450_e85847 * locals.var_dnm_dn14)) * locals.var_arg) - (assign55450_e85849 * locals.var_arg_dn14)) / (locals.var_arg * locals.var_arg)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign55450_e85853;
        locals.var_t0_dn0 = assign55450_e85853_d_n0;
        locals.var_t0_dn2 = assign55450_e85853_d_n2;
        locals.var_t0_dn4 = assign55450_e85853_d_n4;
        locals.var_t0_dn5 = assign55450_e85853_d_n5;
        locals.var_t0_dn6 = assign55450_e85853_d_n6;
        locals.var_t0_dn7 = assign55450_e85853_d_n7;
        locals.var_t0_dn8 = assign55450_e85853_d_n8;
        locals.var_t0_dn9 = assign55450_e85853_d_n9;
        locals.var_t0_dn10 = assign55450_e85853_d_n10;
        locals.var_t0_dn11 = assign55450_e85853_d_n11;
        locals.var_t0_dn14 = assign55450_e85853_d_n14;

        let (assign55460_e85873, assign55460_e85873_d_n0, assign55460_e85873_d_n2, assign55460_e85873_d_n4, assign55460_e85873_d_n5, assign55460_e85873_d_n6, assign55460_e85873_d_n7, assign55460_e85873_d_n8, assign55460_e85873_d_n9, assign55460_e85873_d_n10, assign55460_e85873_d_n11, assign55460_e85873_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_guard1392 != 0.0)) {
        let assign55460_e85869: f64 = 1e-16;
        let assign55460_e85871: f64 = (assign55460_e85869 - locals.var_tmf0);
        (assign55460_e85871, (-locals.var_tmf0_dn0), (-locals.var_tmf0_dn2), (-locals.var_tmf0_dn4), (-locals.var_tmf0_dn5), (-locals.var_tmf0_dn6), (-locals.var_tmf0_dn7), (-locals.var_tmf0_dn8), (-locals.var_tmf0_dn9), (-locals.var_tmf0_dn10), (-locals.var_tmf0_dn11), (-locals.var_tmf0_dn14),)
    } else {
        (locals.var_w_res, locals.var_w_res_dn0, locals.var_w_res_dn2, locals.var_w_res_dn4, locals.var_w_res_dn5, locals.var_w_res_dn6, locals.var_w_res_dn7, locals.var_w_res_dn8, locals.var_w_res_dn9, locals.var_w_res_dn10, locals.var_w_res_dn11, locals.var_w_res_dn14,)
    }
};
        locals.var_w_res = assign55460_e85873;
        locals.var_w_res_dn0 = assign55460_e85873_d_n0;
        locals.var_w_res_dn2 = assign55460_e85873_d_n2;
        locals.var_w_res_dn4 = assign55460_e85873_d_n4;
        locals.var_w_res_dn5 = assign55460_e85873_d_n5;
        locals.var_w_res_dn6 = assign55460_e85873_d_n6;
        locals.var_w_res_dn7 = assign55460_e85873_d_n7;
        locals.var_w_res_dn8 = assign55460_e85873_d_n8;
        locals.var_w_res_dn9 = assign55460_e85873_d_n9;
        locals.var_w_res_dn10 = assign55460_e85873_d_n10;
        locals.var_w_res_dn11 = assign55460_e85873_d_n11;
        locals.var_w_res_dn14 = assign55460_e85873_d_n14;

        let (assign55470_e85889, assign55470_e85889_d_n0, assign55470_e85889_d_n2, assign55470_e85889_d_n4, assign55470_e85889_d_n5, assign55470_e85889_d_n6, assign55470_e85889_d_n7, assign55470_e85889_d_n8, assign55470_e85889_d_n9, assign55470_e85889_d_n10, assign55470_e85889_d_n11, assign55470_e85889_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_guard1392 != 0.0)) {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign55470_e85889;
        locals.var_t0_dn0 = assign55470_e85889_d_n0;
        locals.var_t0_dn2 = assign55470_e85889_d_n2;
        locals.var_t0_dn4 = assign55470_e85889_d_n4;
        locals.var_t0_dn5 = assign55470_e85889_d_n5;
        locals.var_t0_dn6 = assign55470_e85889_d_n6;
        locals.var_t0_dn7 = assign55470_e85889_d_n7;
        locals.var_t0_dn8 = assign55470_e85889_d_n8;
        locals.var_t0_dn9 = assign55470_e85889_d_n9;
        locals.var_t0_dn10 = assign55470_e85889_d_n10;
        locals.var_t0_dn11 = assign55470_e85889_d_n11;
        locals.var_t0_dn14 = assign55470_e85889_d_n14;

        let (assign55480_e85906, assign55480_e85906_d_n0, assign55480_e85906_d_n2, assign55480_e85906_d_n4, assign55480_e85906_d_n5, assign55480_e85906_d_n6, assign55480_e85906_d_n7, assign55480_e85906_d_n8, assign55480_e85906_d_n9, assign55480_e85906_d_n10, assign55480_e85906_d_n11, assign55480_e85906_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_guard1392 == 0.0)) {
        (locals.var_w_res, locals.var_w_res_dn0, locals.var_w_res_dn2, locals.var_w_res_dn4, locals.var_w_res_dn5, locals.var_w_res_dn6, locals.var_w_res_dn7, locals.var_w_res_dn8, locals.var_w_res_dn9, locals.var_w_res_dn10, locals.var_w_res_dn11, locals.var_w_res_dn14,)
    } else {
        (locals.var_w_res, locals.var_w_res_dn0, locals.var_w_res_dn2, locals.var_w_res_dn4, locals.var_w_res_dn5, locals.var_w_res_dn6, locals.var_w_res_dn7, locals.var_w_res_dn8, locals.var_w_res_dn9, locals.var_w_res_dn10, locals.var_w_res_dn11, locals.var_w_res_dn14,)
    }
};
        locals.var_w_res = assign55480_e85906;
        locals.var_w_res_dn0 = assign55480_e85906_d_n0;
        locals.var_w_res_dn2 = assign55480_e85906_d_n2;
        locals.var_w_res_dn4 = assign55480_e85906_d_n4;
        locals.var_w_res_dn5 = assign55480_e85906_d_n5;
        locals.var_w_res_dn6 = assign55480_e85906_d_n6;
        locals.var_w_res_dn7 = assign55480_e85906_d_n7;
        locals.var_w_res_dn8 = assign55480_e85906_d_n8;
        locals.var_w_res_dn9 = assign55480_e85906_d_n9;
        locals.var_w_res_dn10 = assign55480_e85906_d_n10;
        locals.var_w_res_dn11 = assign55480_e85906_d_n11;
        locals.var_w_res_dn14 = assign55480_e85906_d_n14;

        let (assign55490_e85923, assign55490_e85923_d_n0, assign55490_e85923_d_n2, assign55490_e85923_d_n4, assign55490_e85923_d_n5, assign55490_e85923_d_n6, assign55490_e85923_d_n7, assign55490_e85923_d_n8, assign55490_e85923_d_n9, assign55490_e85923_d_n10, assign55490_e85923_d_n11, assign55490_e85923_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_guard1392 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign55490_e85923;
        locals.var_t0_dn0 = assign55490_e85923_d_n0;
        locals.var_t0_dn2 = assign55490_e85923_d_n2;
        locals.var_t0_dn4 = assign55490_e85923_d_n4;
        locals.var_t0_dn5 = assign55490_e85923_d_n5;
        locals.var_t0_dn6 = assign55490_e85923_d_n6;
        locals.var_t0_dn7 = assign55490_e85923_d_n7;
        locals.var_t0_dn8 = assign55490_e85923_d_n8;
        locals.var_t0_dn9 = assign55490_e85923_d_n9;
        locals.var_t0_dn10 = assign55490_e85923_d_n10;
        locals.var_t0_dn11 = assign55490_e85923_d_n11;
        locals.var_t0_dn14 = assign55490_e85923_d_n14;

        let assign55500_e85926: f64 = if 2.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1398 = assign55500_e85926;

        let (assign55510_e85942, assign55510_e85942_d_n0, assign55510_e85942_d_n2, assign55510_e85942_d_n4, assign55510_e85942_d_n5, assign55510_e85942_d_n6, assign55510_e85942_d_n7, assign55510_e85942_d_n8, assign55510_e85942_d_n9, assign55510_e85942_d_n10, assign55510_e85942_d_n11, assign55510_e85942_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1332 == 0.0)) && (locals.var_guard1398 != 0.0)) {
        (locals.var_w_res, locals.var_w_res_dn0, locals.var_w_res_dn2, locals.var_w_res_dn4, locals.var_w_res_dn5, locals.var_w_res_dn6, locals.var_w_res_dn7, locals.var_w_res_dn8, locals.var_w_res_dn9, locals.var_w_res_dn10, locals.var_w_res_dn11, locals.var_w_res_dn14,)
    } else {
        (locals.var_w_res_leak, locals.var_w_res_leak_dn0, locals.var_w_res_leak_dn2, locals.var_w_res_leak_dn4, locals.var_w_res_leak_dn5, locals.var_w_res_leak_dn6, locals.var_w_res_leak_dn7, locals.var_w_res_leak_dn8, locals.var_w_res_leak_dn9, locals.var_w_res_leak_dn10, locals.var_w_res_leak_dn11, locals.var_w_res_leak_dn14,)
    }
};
        locals.var_w_res_leak = assign55510_e85942;
        locals.var_w_res_leak_dn0 = assign55510_e85942_d_n0;
        locals.var_w_res_leak_dn2 = assign55510_e85942_d_n2;
        locals.var_w_res_leak_dn4 = assign55510_e85942_d_n4;
        locals.var_w_res_leak_dn5 = assign55510_e85942_d_n5;
        locals.var_w_res_leak_dn6 = assign55510_e85942_d_n6;
        locals.var_w_res_leak_dn7 = assign55510_e85942_d_n7;
        locals.var_w_res_leak_dn8 = assign55510_e85942_d_n8;
        locals.var_w_res_leak_dn9 = assign55510_e85942_d_n9;
        locals.var_w_res_leak_dn10 = assign55510_e85942_d_n10;
        locals.var_w_res_leak_dn11 = assign55510_e85942_d_n11;
        locals.var_w_res_leak_dn14 = assign55510_e85942_d_n14;

        let assign55520_e85945: f64 = if 0.0 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1399 = assign55520_e85945;

        let (assign55530_e85958, assign55530_e85958_d_n0, assign55530_e85958_d_n2, assign55530_e85958_d_n4, assign55530_e85958_d_n5, assign55530_e85958_d_n6, assign55530_e85958_d_n7, assign55530_e85958_d_n8, assign55530_e85958_d_n9, assign55530_e85958_d_n10, assign55530_e85958_d_n11, assign55530_e85958_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1399 != 0.0)) {
        (locals.var_vdsorg, locals.var_vdsorg_dn0, locals.var_vdsorg_dn2, locals.var_vdsorg_dn4, locals.var_vdsorg_dn5, locals.var_vdsorg_dn6, locals.var_vdsorg_dn7, locals.var_vdsorg_dn8, locals.var_vdsorg_dn9, locals.var_vdsorg_dn10, locals.var_vdsorg_dn11, locals.var_vdsorg_dn14,)
    } else {
        (locals.var_vds_res, locals.var_vds_res_dn0, locals.var_vds_res_dn2, locals.var_vds_res_dn4, locals.var_vds_res_dn5, locals.var_vds_res_dn6, locals.var_vds_res_dn7, locals.var_vds_res_dn8, locals.var_vds_res_dn9, locals.var_vds_res_dn10, locals.var_vds_res_dn11, locals.var_vds_res_dn14,)
    }
};
        locals.var_vds_res = assign55530_e85958;
        locals.var_vds_res_dn0 = assign55530_e85958_d_n0;
        locals.var_vds_res_dn2 = assign55530_e85958_d_n2;
        locals.var_vds_res_dn4 = assign55530_e85958_d_n4;
        locals.var_vds_res_dn5 = assign55530_e85958_d_n5;
        locals.var_vds_res_dn6 = assign55530_e85958_d_n6;
        locals.var_vds_res_dn7 = assign55530_e85958_d_n7;
        locals.var_vds_res_dn8 = assign55530_e85958_d_n8;
        locals.var_vds_res_dn9 = assign55530_e85958_d_n9;
        locals.var_vds_res_dn10 = assign55530_e85958_d_n10;
        locals.var_vds_res_dn11 = assign55530_e85958_d_n11;
        locals.var_vds_res_dn14 = assign55530_e85958_d_n14;

        let (assign55540_e85975, assign55540_e85975_d_n0, assign55540_e85975_d_n2, assign55540_e85975_d_n4, assign55540_e85975_d_n5, assign55540_e85975_d_n6, assign55540_e85975_d_n7, assign55540_e85975_d_n8, assign55540_e85975_d_n9, assign55540_e85975_d_n10, assign55540_e85975_d_n11, assign55540_e85975_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1399 != 0.0)) {
        let assign55540_e85971: f64 = (locals.var_vbsc__blk1121 + locals.var_beta_inv);
        let assign55540_e85973: f64 = (assign55540_e85971 * p.p396);
        (assign55540_e85973, ((locals.var_vbsc__blk1121_dn0 + locals.var_beta_inv_dn0) * p.p396), ((locals.var_vbsc__blk1121_dn2 + locals.var_beta_inv_dn2) * p.p396), ((locals.var_vbsc__blk1121_dn4 + locals.var_beta_inv_dn4) * p.p396), ((locals.var_vbsc__blk1121_dn5 + locals.var_beta_inv_dn5) * p.p396), ((locals.var_vbsc__blk1121_dn6 + locals.var_beta_inv_dn6) * p.p396), ((locals.var_vbsc__blk1121_dn7 + locals.var_beta_inv_dn7) * p.p396), ((locals.var_vbsc__blk1121_dn8 + locals.var_beta_inv_dn8) * p.p396), ((locals.var_vbsc__blk1121_dn9 + locals.var_beta_inv_dn9) * p.p396), ((locals.var_vbsc__blk1121_dn10 + locals.var_beta_inv_dn10) * p.p396), ((locals.var_vbsc__blk1121_dn11 + locals.var_beta_inv_dn11) * p.p396), ((locals.var_vbsc__blk1121_dn14 + locals.var_beta_inv_dn14) * p.p396),)
    } else {
        (locals.var_t10, locals.var_t10_dn0, locals.var_t10_dn2, locals.var_t10_dn4, locals.var_t10_dn5, locals.var_t10_dn6, locals.var_t10_dn7, locals.var_t10_dn8, locals.var_t10_dn9, locals.var_t10_dn10, locals.var_t10_dn11, locals.var_t10_dn14,)
    }
};
        locals.var_t10 = assign55540_e85975;
        locals.var_t10_dn0 = assign55540_e85975_d_n0;
        locals.var_t10_dn2 = assign55540_e85975_d_n2;
        locals.var_t10_dn4 = assign55540_e85975_d_n4;
        locals.var_t10_dn5 = assign55540_e85975_d_n5;
        locals.var_t10_dn6 = assign55540_e85975_d_n6;
        locals.var_t10_dn7 = assign55540_e85975_d_n7;
        locals.var_t10_dn8 = assign55540_e85975_d_n8;
        locals.var_t10_dn9 = assign55540_e85975_d_n9;
        locals.var_t10_dn10 = assign55540_e85975_d_n10;
        locals.var_t10_dn11 = assign55540_e85975_d_n11;
        locals.var_t10_dn14 = assign55540_e85975_d_n14;

        let (assign55550_e85994, assign55550_e85994_d_n0, assign55550_e85994_d_n2, assign55550_e85994_d_n4, assign55550_e85994_d_n5, assign55550_e85994_d_n6, assign55550_e85994_d_n7, assign55550_e85994_d_n8, assign55550_e85994_d_n9, assign55550_e85994_d_n10, assign55550_e85994_d_n11, assign55550_e85994_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1399 != 0.0)) {
        let assign55550_e85990: f64 = (locals.var_vgp - locals.var_t10);
        let assign55550_e85991: f64 = (locals.var_c2_q_ndepm_esi_cox_inv2__blk1139 * assign55550_e85990);
        let assign55550_e85992: f64 = (1.0 + assign55550_e85991);
        (assign55550_e85992, ((locals.var_c2_q_ndepm_esi_cox_inv2__blk1139_dn0 * assign55550_e85990) + (locals.var_c2_q_ndepm_esi_cox_inv2__blk1139 * (locals.var_vgp_dn0 - locals.var_t10_dn0))), ((locals.var_c2_q_ndepm_esi_cox_inv2__blk1139_dn2 * assign55550_e85990) + (locals.var_c2_q_ndepm_esi_cox_inv2__blk1139 * (locals.var_vgp_dn2 - locals.var_t10_dn2))), ((locals.var_c2_q_ndepm_esi_cox_inv2__blk1139_dn4 * assign55550_e85990) + (locals.var_c2_q_ndepm_esi_cox_inv2__blk1139 * (locals.var_vgp_dn4 - locals.var_t10_dn4))), ((locals.var_c2_q_ndepm_esi_cox_inv2__blk1139_dn5 * assign55550_e85990) + (locals.var_c2_q_ndepm_esi_cox_inv2__blk1139 * (locals.var_vgp_dn5 - locals.var_t10_dn5))), ((locals.var_c2_q_ndepm_esi_cox_inv2__blk1139_dn6 * assign55550_e85990) + (locals.var_c2_q_ndepm_esi_cox_inv2__blk1139 * (locals.var_vgp_dn6 - locals.var_t10_dn6))), ((locals.var_c2_q_ndepm_esi_cox_inv2__blk1139_dn7 * assign55550_e85990) + (locals.var_c2_q_ndepm_esi_cox_inv2__blk1139 * (locals.var_vgp_dn7 - locals.var_t10_dn7))), ((locals.var_c2_q_ndepm_esi_cox_inv2__blk1139_dn8 * assign55550_e85990) + (locals.var_c2_q_ndepm_esi_cox_inv2__blk1139 * (locals.var_vgp_dn8 - locals.var_t10_dn8))), ((locals.var_c2_q_ndepm_esi_cox_inv2__blk1139_dn9 * assign55550_e85990) + (locals.var_c2_q_ndepm_esi_cox_inv2__blk1139 * (locals.var_vgp_dn9 - locals.var_t10_dn9))), ((locals.var_c2_q_ndepm_esi_cox_inv2__blk1139_dn10 * assign55550_e85990) + (locals.var_c2_q_ndepm_esi_cox_inv2__blk1139 * (locals.var_vgp_dn10 - locals.var_t10_dn10))), ((locals.var_c2_q_ndepm_esi_cox_inv2__blk1139_dn11 * assign55550_e85990) + (locals.var_c2_q_ndepm_esi_cox_inv2__blk1139 * (locals.var_vgp_dn11 - locals.var_t10_dn11))), ((locals.var_c2_q_ndepm_esi_cox_inv2__blk1139_dn14 * assign55550_e85990) + (locals.var_c2_q_ndepm_esi_cox_inv2__blk1139 * (locals.var_vgp_dn14 - locals.var_t10_dn14))),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign55550_e85994;
        locals.var_t4_dn0 = assign55550_e85994_d_n0;
        locals.var_t4_dn2 = assign55550_e85994_d_n2;
        locals.var_t4_dn4 = assign55550_e85994_d_n4;
        locals.var_t4_dn5 = assign55550_e85994_d_n5;
        locals.var_t4_dn6 = assign55550_e85994_d_n6;
        locals.var_t4_dn7 = assign55550_e85994_d_n7;
        locals.var_t4_dn8 = assign55550_e85994_d_n8;
        locals.var_t4_dn9 = assign55550_e85994_d_n9;
        locals.var_t4_dn10 = assign55550_e85994_d_n10;
        locals.var_t4_dn11 = assign55550_e85994_d_n11;
        locals.var_t4_dn14 = assign55550_e85994_d_n14;

        let (assign55560_e86009, assign55560_e86009_d_n0, assign55560_e86009_d_n2, assign55560_e86009_d_n4, assign55560_e86009_d_n5, assign55560_e86009_d_n6, assign55560_e86009_d_n7, assign55560_e86009_d_n8, assign55560_e86009_d_n9, assign55560_e86009_d_n10, assign55560_e86009_d_n11, assign55560_e86009_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1399 != 0.0)) {
        let assign55560_e86007: f64 = (1.0 + locals.var_c2_q_ndepm_esi_cox_inv2__blk1139);
        (assign55560_e86007, locals.var_c2_q_ndepm_esi_cox_inv2__blk1139_dn0, locals.var_c2_q_ndepm_esi_cox_inv2__blk1139_dn2, locals.var_c2_q_ndepm_esi_cox_inv2__blk1139_dn4, locals.var_c2_q_ndepm_esi_cox_inv2__blk1139_dn5, locals.var_c2_q_ndepm_esi_cox_inv2__blk1139_dn6, locals.var_c2_q_ndepm_esi_cox_inv2__blk1139_dn7, locals.var_c2_q_ndepm_esi_cox_inv2__blk1139_dn8, locals.var_c2_q_ndepm_esi_cox_inv2__blk1139_dn9, locals.var_c2_q_ndepm_esi_cox_inv2__blk1139_dn10, locals.var_c2_q_ndepm_esi_cox_inv2__blk1139_dn11, locals.var_c2_q_ndepm_esi_cox_inv2__blk1139_dn14,)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign55560_e86009;
        locals.var_t5_dn0 = assign55560_e86009_d_n0;
        locals.var_t5_dn2 = assign55560_e86009_d_n2;
        locals.var_t5_dn4 = assign55560_e86009_d_n4;
        locals.var_t5_dn5 = assign55560_e86009_d_n5;
        locals.var_t5_dn6 = assign55560_e86009_d_n6;
        locals.var_t5_dn7 = assign55560_e86009_d_n7;
        locals.var_t5_dn8 = assign55560_e86009_d_n8;
        locals.var_t5_dn9 = assign55560_e86009_d_n9;
        locals.var_t5_dn10 = assign55560_e86009_d_n10;
        locals.var_t5_dn11 = assign55560_e86009_d_n11;
        locals.var_t5_dn14 = assign55560_e86009_d_n14;

        let assign55570_e86013: f64 = locals.var_t5;
        let assign55570_e86018: f64 = if ((locals.var_t4 < assign55570_e86013) && (locals.var_t5 >= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1400 = assign55570_e86018;

        let (assign55580_e86037, assign55580_e86037_d_n0, assign55580_e86037_d_n2, assign55580_e86037_d_n4, assign55580_e86037_d_n5, assign55580_e86037_d_n6, assign55580_e86037_d_n7, assign55580_e86037_d_n8, assign55580_e86037_d_n9, assign55580_e86037_d_n10, assign55580_e86037_d_n11, assign55580_e86037_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1399 != 0.0)) && (locals.var_guard1400 != 0.0)) {
        let assign55580_e86033: f64 = locals.var_t5;
        let assign55580_e86035: f64 = (assign55580_e86033 - locals.var_t4);
        (assign55580_e86035, (locals.var_t5_dn0 - locals.var_t4_dn0), (locals.var_t5_dn2 - locals.var_t4_dn2), (locals.var_t5_dn4 - locals.var_t4_dn4), (locals.var_t5_dn5 - locals.var_t4_dn5), (locals.var_t5_dn6 - locals.var_t4_dn6), (locals.var_t5_dn7 - locals.var_t4_dn7), (locals.var_t5_dn8 - locals.var_t4_dn8), (locals.var_t5_dn9 - locals.var_t4_dn9), (locals.var_t5_dn10 - locals.var_t4_dn10), (locals.var_t5_dn11 - locals.var_t4_dn11), (locals.var_t5_dn14 - locals.var_t4_dn14),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign55580_e86037;
        locals.var_tmf1_dn0 = assign55580_e86037_d_n0;
        locals.var_tmf1_dn2 = assign55580_e86037_d_n2;
        locals.var_tmf1_dn4 = assign55580_e86037_d_n4;
        locals.var_tmf1_dn5 = assign55580_e86037_d_n5;
        locals.var_tmf1_dn6 = assign55580_e86037_d_n6;
        locals.var_tmf1_dn7 = assign55580_e86037_d_n7;
        locals.var_tmf1_dn8 = assign55580_e86037_d_n8;
        locals.var_tmf1_dn9 = assign55580_e86037_d_n9;
        locals.var_tmf1_dn10 = assign55580_e86037_d_n10;
        locals.var_tmf1_dn11 = assign55580_e86037_d_n11;
        locals.var_tmf1_dn14 = assign55580_e86037_d_n14;

        let (assign55590_e86054, assign55590_e86054_d_n0, assign55590_e86054_d_n2, assign55590_e86054_d_n4, assign55590_e86054_d_n5, assign55590_e86054_d_n6, assign55590_e86054_d_n7, assign55590_e86054_d_n8, assign55590_e86054_d_n9, assign55590_e86054_d_n10, assign55590_e86054_d_n11, assign55590_e86054_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1399 != 0.0)) && (locals.var_guard1400 != 0.0)) {
        let assign55590_e86052: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign55590_e86052, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)), ((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)), ((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)), ((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn4, locals.var_x2_dn5, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn8, locals.var_x2_dn9, locals.var_x2_dn10, locals.var_x2_dn11, locals.var_x2_dn14,)
    }
};
        locals.var_x2 = assign55590_e86054;
        locals.var_x2_dn0 = assign55590_e86054_d_n0;
        locals.var_x2_dn2 = assign55590_e86054_d_n2;
        locals.var_x2_dn4 = assign55590_e86054_d_n4;
        locals.var_x2_dn5 = assign55590_e86054_d_n5;
        locals.var_x2_dn6 = assign55590_e86054_d_n6;
        locals.var_x2_dn7 = assign55590_e86054_d_n7;
        locals.var_x2_dn8 = assign55590_e86054_d_n8;
        locals.var_x2_dn9 = assign55590_e86054_d_n9;
        locals.var_x2_dn10 = assign55590_e86054_d_n10;
        locals.var_x2_dn11 = assign55590_e86054_d_n11;
        locals.var_x2_dn14 = assign55590_e86054_d_n14;

    }

    pub(super) fn stamp_transient_block_191(
        locals: &mut StampLocals,
    ) {
        let (assign55600_e86071, assign55600_e86071_d_n0, assign55600_e86071_d_n2, assign55600_e86071_d_n4, assign55600_e86071_d_n5, assign55600_e86071_d_n6, assign55600_e86071_d_n7, assign55600_e86071_d_n8, assign55600_e86071_d_n9, assign55600_e86071_d_n10, assign55600_e86071_d_n11, assign55600_e86071_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1399 != 0.0)) && (locals.var_guard1400 != 0.0)) {
        let assign55600_e86069: f64 = (locals.var_t5 * locals.var_t5);
        (assign55600_e86069, ((locals.var_t5_dn0 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn0)), ((locals.var_t5_dn2 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn2)), ((locals.var_t5_dn4 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn4)), ((locals.var_t5_dn5 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn5)), ((locals.var_t5_dn6 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn6)), ((locals.var_t5_dn7 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn7)), ((locals.var_t5_dn8 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn8)), ((locals.var_t5_dn9 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn9)), ((locals.var_t5_dn10 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn10)), ((locals.var_t5_dn11 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn11)), ((locals.var_t5_dn14 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn14)),)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn4, locals.var_xmax2_dn5, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn8, locals.var_xmax2_dn9, locals.var_xmax2_dn10, locals.var_xmax2_dn11, locals.var_xmax2_dn14,)
    }
};
        locals.var_xmax2 = assign55600_e86071;
        locals.var_xmax2_dn0 = assign55600_e86071_d_n0;
        locals.var_xmax2_dn2 = assign55600_e86071_d_n2;
        locals.var_xmax2_dn4 = assign55600_e86071_d_n4;
        locals.var_xmax2_dn5 = assign55600_e86071_d_n5;
        locals.var_xmax2_dn6 = assign55600_e86071_d_n6;
        locals.var_xmax2_dn7 = assign55600_e86071_d_n7;
        locals.var_xmax2_dn8 = assign55600_e86071_d_n8;
        locals.var_xmax2_dn9 = assign55600_e86071_d_n9;
        locals.var_xmax2_dn10 = assign55600_e86071_d_n10;
        locals.var_xmax2_dn11 = assign55600_e86071_d_n11;
        locals.var_xmax2_dn14 = assign55600_e86071_d_n14;

        let (assign55610_e86086, assign55610_e86086_d_n0, assign55610_e86086_d_n2, assign55610_e86086_d_n4, assign55610_e86086_d_n5, assign55610_e86086_d_n6, assign55610_e86086_d_n7, assign55610_e86086_d_n8, assign55610_e86086_d_n9, assign55610_e86086_d_n10, assign55610_e86086_d_n11, assign55610_e86086_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1399 != 0.0)) && (locals.var_guard1400 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign55610_e86086;
        locals.var_xp_dn0 = assign55610_e86086_d_n0;
        locals.var_xp_dn2 = assign55610_e86086_d_n2;
        locals.var_xp_dn4 = assign55610_e86086_d_n4;
        locals.var_xp_dn5 = assign55610_e86086_d_n5;
        locals.var_xp_dn6 = assign55610_e86086_d_n6;
        locals.var_xp_dn7 = assign55610_e86086_d_n7;
        locals.var_xp_dn8 = assign55610_e86086_d_n8;
        locals.var_xp_dn9 = assign55610_e86086_d_n9;
        locals.var_xp_dn10 = assign55610_e86086_d_n10;
        locals.var_xp_dn11 = assign55610_e86086_d_n11;
        locals.var_xp_dn14 = assign55610_e86086_d_n14;

        let (assign55620_e86101, assign55620_e86101_d_n0, assign55620_e86101_d_n2, assign55620_e86101_d_n4, assign55620_e86101_d_n5, assign55620_e86101_d_n6, assign55620_e86101_d_n7, assign55620_e86101_d_n8, assign55620_e86101_d_n9, assign55620_e86101_d_n10, assign55620_e86101_d_n11, assign55620_e86101_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1399 != 0.0)) && (locals.var_guard1400 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign55620_e86101;
        locals.var_xmp_dn0 = assign55620_e86101_d_n0;
        locals.var_xmp_dn2 = assign55620_e86101_d_n2;
        locals.var_xmp_dn4 = assign55620_e86101_d_n4;
        locals.var_xmp_dn5 = assign55620_e86101_d_n5;
        locals.var_xmp_dn6 = assign55620_e86101_d_n6;
        locals.var_xmp_dn7 = assign55620_e86101_d_n7;
        locals.var_xmp_dn8 = assign55620_e86101_d_n8;
        locals.var_xmp_dn9 = assign55620_e86101_d_n9;
        locals.var_xmp_dn10 = assign55620_e86101_d_n10;
        locals.var_xmp_dn11 = assign55620_e86101_d_n11;
        locals.var_xmp_dn14 = assign55620_e86101_d_n14;

        let (assign55630_e86116,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1399 != 0.0)) && (locals.var_guard1400 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign55630_e86116;

        let (assign55640_e86131,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1399 != 0.0)) && (locals.var_guard1400 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign55640_e86131;

        let (assign55650_e86146, assign55650_e86146_d_n0, assign55650_e86146_d_n2, assign55650_e86146_d_n4, assign55650_e86146_d_n5, assign55650_e86146_d_n6, assign55650_e86146_d_n7, assign55650_e86146_d_n8, assign55650_e86146_d_n9, assign55650_e86146_d_n10, assign55650_e86146_d_n11, assign55650_e86146_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1399 != 0.0)) && (locals.var_guard1400 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign55650_e86146;
        locals.var_arg_dn0 = assign55650_e86146_d_n0;
        locals.var_arg_dn2 = assign55650_e86146_d_n2;
        locals.var_arg_dn4 = assign55650_e86146_d_n4;
        locals.var_arg_dn5 = assign55650_e86146_d_n5;
        locals.var_arg_dn6 = assign55650_e86146_d_n6;
        locals.var_arg_dn7 = assign55650_e86146_d_n7;
        locals.var_arg_dn8 = assign55650_e86146_d_n8;
        locals.var_arg_dn9 = assign55650_e86146_d_n9;
        locals.var_arg_dn10 = assign55650_e86146_d_n10;
        locals.var_arg_dn11 = assign55650_e86146_d_n11;
        locals.var_arg_dn14 = assign55650_e86146_d_n14;

        let (assign55660_e86161, assign55660_e86161_d_n0, assign55660_e86161_d_n2, assign55660_e86161_d_n4, assign55660_e86161_d_n5, assign55660_e86161_d_n6, assign55660_e86161_d_n7, assign55660_e86161_d_n8, assign55660_e86161_d_n9, assign55660_e86161_d_n10, assign55660_e86161_d_n11, assign55660_e86161_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1399 != 0.0)) && (locals.var_guard1400 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign55660_e86161;
        locals.var_dnm_dn0 = assign55660_e86161_d_n0;
        locals.var_dnm_dn2 = assign55660_e86161_d_n2;
        locals.var_dnm_dn4 = assign55660_e86161_d_n4;
        locals.var_dnm_dn5 = assign55660_e86161_d_n5;
        locals.var_dnm_dn6 = assign55660_e86161_d_n6;
        locals.var_dnm_dn7 = assign55660_e86161_d_n7;
        locals.var_dnm_dn8 = assign55660_e86161_d_n8;
        locals.var_dnm_dn9 = assign55660_e86161_d_n9;
        locals.var_dnm_dn10 = assign55660_e86161_d_n10;
        locals.var_dnm_dn11 = assign55660_e86161_d_n11;
        locals.var_dnm_dn14 = assign55660_e86161_d_n14;

        let (assign55670_e86178, assign55670_e86178_d_n0, assign55670_e86178_d_n2, assign55670_e86178_d_n4, assign55670_e86178_d_n5, assign55670_e86178_d_n6, assign55670_e86178_d_n7, assign55670_e86178_d_n8, assign55670_e86178_d_n9, assign55670_e86178_d_n10, assign55670_e86178_d_n11, assign55670_e86178_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1399 != 0.0)) && (locals.var_guard1400 != 0.0)) {
        let assign55670_e86176: f64 = (locals.var_xp * locals.var_x2);
        (assign55670_e86176, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign55670_e86178;
        locals.var_xp_dn0 = assign55670_e86178_d_n0;
        locals.var_xp_dn2 = assign55670_e86178_d_n2;
        locals.var_xp_dn4 = assign55670_e86178_d_n4;
        locals.var_xp_dn5 = assign55670_e86178_d_n5;
        locals.var_xp_dn6 = assign55670_e86178_d_n6;
        locals.var_xp_dn7 = assign55670_e86178_d_n7;
        locals.var_xp_dn8 = assign55670_e86178_d_n8;
        locals.var_xp_dn9 = assign55670_e86178_d_n9;
        locals.var_xp_dn10 = assign55670_e86178_d_n10;
        locals.var_xp_dn11 = assign55670_e86178_d_n11;
        locals.var_xp_dn14 = assign55670_e86178_d_n14;

        let (assign55680_e86195, assign55680_e86195_d_n0, assign55680_e86195_d_n2, assign55680_e86195_d_n4, assign55680_e86195_d_n5, assign55680_e86195_d_n6, assign55680_e86195_d_n7, assign55680_e86195_d_n8, assign55680_e86195_d_n9, assign55680_e86195_d_n10, assign55680_e86195_d_n11, assign55680_e86195_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1399 != 0.0)) && (locals.var_guard1400 != 0.0)) {
        let assign55680_e86193: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign55680_e86193, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign55680_e86195;
        locals.var_xmp_dn0 = assign55680_e86195_d_n0;
        locals.var_xmp_dn2 = assign55680_e86195_d_n2;
        locals.var_xmp_dn4 = assign55680_e86195_d_n4;
        locals.var_xmp_dn5 = assign55680_e86195_d_n5;
        locals.var_xmp_dn6 = assign55680_e86195_d_n6;
        locals.var_xmp_dn7 = assign55680_e86195_d_n7;
        locals.var_xmp_dn8 = assign55680_e86195_d_n8;
        locals.var_xmp_dn9 = assign55680_e86195_d_n9;
        locals.var_xmp_dn10 = assign55680_e86195_d_n10;
        locals.var_xmp_dn11 = assign55680_e86195_d_n11;
        locals.var_xmp_dn14 = assign55680_e86195_d_n14;

        let (assign55690_e86212, assign55690_e86212_d_n0, assign55690_e86212_d_n2, assign55690_e86212_d_n4, assign55690_e86212_d_n5, assign55690_e86212_d_n6, assign55690_e86212_d_n7, assign55690_e86212_d_n8, assign55690_e86212_d_n9, assign55690_e86212_d_n10, assign55690_e86212_d_n11, assign55690_e86212_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1399 != 0.0)) && (locals.var_guard1400 != 0.0)) {
        let assign55690_e86210: f64 = (locals.var_xp * locals.var_x2);
        (assign55690_e86210, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign55690_e86212;
        locals.var_xp_dn0 = assign55690_e86212_d_n0;
        locals.var_xp_dn2 = assign55690_e86212_d_n2;
        locals.var_xp_dn4 = assign55690_e86212_d_n4;
        locals.var_xp_dn5 = assign55690_e86212_d_n5;
        locals.var_xp_dn6 = assign55690_e86212_d_n6;
        locals.var_xp_dn7 = assign55690_e86212_d_n7;
        locals.var_xp_dn8 = assign55690_e86212_d_n8;
        locals.var_xp_dn9 = assign55690_e86212_d_n9;
        locals.var_xp_dn10 = assign55690_e86212_d_n10;
        locals.var_xp_dn11 = assign55690_e86212_d_n11;
        locals.var_xp_dn14 = assign55690_e86212_d_n14;

        let (assign55700_e86229, assign55700_e86229_d_n0, assign55700_e86229_d_n2, assign55700_e86229_d_n4, assign55700_e86229_d_n5, assign55700_e86229_d_n6, assign55700_e86229_d_n7, assign55700_e86229_d_n8, assign55700_e86229_d_n9, assign55700_e86229_d_n10, assign55700_e86229_d_n11, assign55700_e86229_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1399 != 0.0)) && (locals.var_guard1400 != 0.0)) {
        let assign55700_e86227: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign55700_e86227, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign55700_e86229;
        locals.var_xmp_dn0 = assign55700_e86229_d_n0;
        locals.var_xmp_dn2 = assign55700_e86229_d_n2;
        locals.var_xmp_dn4 = assign55700_e86229_d_n4;
        locals.var_xmp_dn5 = assign55700_e86229_d_n5;
        locals.var_xmp_dn6 = assign55700_e86229_d_n6;
        locals.var_xmp_dn7 = assign55700_e86229_d_n7;
        locals.var_xmp_dn8 = assign55700_e86229_d_n8;
        locals.var_xmp_dn9 = assign55700_e86229_d_n9;
        locals.var_xmp_dn10 = assign55700_e86229_d_n10;
        locals.var_xmp_dn11 = assign55700_e86229_d_n11;
        locals.var_xmp_dn14 = assign55700_e86229_d_n14;

        let (assign55710_e86246, assign55710_e86246_d_n0, assign55710_e86246_d_n2, assign55710_e86246_d_n4, assign55710_e86246_d_n5, assign55710_e86246_d_n6, assign55710_e86246_d_n7, assign55710_e86246_d_n8, assign55710_e86246_d_n9, assign55710_e86246_d_n10, assign55710_e86246_d_n11, assign55710_e86246_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1399 != 0.0)) && (locals.var_guard1400 != 0.0)) {
        let assign55710_e86244: f64 = (locals.var_xp + locals.var_xmp);
        (assign55710_e86244, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn4 + locals.var_xmp_dn4), (locals.var_xp_dn5 + locals.var_xmp_dn5), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn8 + locals.var_xmp_dn8), (locals.var_xp_dn9 + locals.var_xmp_dn9), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn11 + locals.var_xmp_dn11), (locals.var_xp_dn14 + locals.var_xmp_dn14),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign55710_e86246;
        locals.var_arg_dn0 = assign55710_e86246_d_n0;
        locals.var_arg_dn2 = assign55710_e86246_d_n2;
        locals.var_arg_dn4 = assign55710_e86246_d_n4;
        locals.var_arg_dn5 = assign55710_e86246_d_n5;
        locals.var_arg_dn6 = assign55710_e86246_d_n6;
        locals.var_arg_dn7 = assign55710_e86246_d_n7;
        locals.var_arg_dn8 = assign55710_e86246_d_n8;
        locals.var_arg_dn9 = assign55710_e86246_d_n9;
        locals.var_arg_dn10 = assign55710_e86246_d_n10;
        locals.var_arg_dn11 = assign55710_e86246_d_n11;
        locals.var_arg_dn14 = assign55710_e86246_d_n14;

        let (assign55720_e86261, assign55720_e86261_d_n0, assign55720_e86261_d_n2, assign55720_e86261_d_n4, assign55720_e86261_d_n5, assign55720_e86261_d_n6, assign55720_e86261_d_n7, assign55720_e86261_d_n8, assign55720_e86261_d_n9, assign55720_e86261_d_n10, assign55720_e86261_d_n11, assign55720_e86261_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1399 != 0.0)) && (locals.var_guard1400 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign55720_e86261;
        locals.var_dnm_dn0 = assign55720_e86261_d_n0;
        locals.var_dnm_dn2 = assign55720_e86261_d_n2;
        locals.var_dnm_dn4 = assign55720_e86261_d_n4;
        locals.var_dnm_dn5 = assign55720_e86261_d_n5;
        locals.var_dnm_dn6 = assign55720_e86261_d_n6;
        locals.var_dnm_dn7 = assign55720_e86261_d_n7;
        locals.var_dnm_dn8 = assign55720_e86261_d_n8;
        locals.var_dnm_dn9 = assign55720_e86261_d_n9;
        locals.var_dnm_dn10 = assign55720_e86261_d_n10;
        locals.var_dnm_dn11 = assign55720_e86261_d_n11;
        locals.var_dnm_dn14 = assign55720_e86261_d_n14;

        let assign55730_e86276: f64 = if ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard1401 = assign55730_e86276;

        let assign55740_e86279: f64 = if 2.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1402 = assign55740_e86279;

        let (assign55750_e86298,) = {
    if ((((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1399 != 0.0)) && (locals.var_guard1400 != 0.0)) && (locals.var_guard1401 != 0.0)) && (locals.var_guard1402 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign55750_e86298;

        let assign55760_e86301: f64 = if 2.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard1403 = assign55760_e86301;

        let (assign55770_e86323,) = {
    if (((((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1399 != 0.0)) && (locals.var_guard1400 != 0.0)) && (locals.var_guard1401 != 0.0)) && (locals.var_guard1402 == 0.0)) && (locals.var_guard1403 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign55770_e86323;

        let assign55780_e86326: f64 = if 2.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard1404 = assign55780_e86326;

        let (assign55790_e86351,) = {
    if ((((((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1399 != 0.0)) && (locals.var_guard1400 != 0.0)) && (locals.var_guard1401 != 0.0)) && (locals.var_guard1402 == 0.0)) && (locals.var_guard1403 == 0.0)) && (locals.var_guard1404 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign55790_e86351;

        let assign55800_e86354: f64 = if 2.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard1405 = assign55800_e86354;

        let (assign55810_e86382,) = {
    if (((((((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1399 != 0.0)) && (locals.var_guard1400 != 0.0)) && (locals.var_guard1401 != 0.0)) && (locals.var_guard1402 == 0.0)) && (locals.var_guard1403 == 0.0)) && (locals.var_guard1404 == 0.0)) && (locals.var_guard1405 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign55810_e86382;

        let (assign55820_e86399,) = {
    if (((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1399 != 0.0)) && (locals.var_guard1400 != 0.0)) && (locals.var_guard1401 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign55820_e86399;

        let mut assign55830_loop_guard: usize = 0;
        while {
            let assign55830_cond_e86417: f64 = if ((((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1399 != 0.0)) && (locals.var_guard1400 != 0.0)) && (locals.var_guard1401 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign55830_cond_e86417 != 0.0
        } {
            assign55830_loop_guard += 1;
            assert!(assign55830_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign55830_body0_e86435, assign55830_body0_e86435_d_n0, assign55830_body0_e86435_d_n2, assign55830_body0_e86435_d_n4, assign55830_body0_e86435_d_n5, assign55830_body0_e86435_d_n6, assign55830_body0_e86435_d_n7, assign55830_body0_e86435_d_n8, assign55830_body0_e86435_d_n9, assign55830_body0_e86435_d_n10, assign55830_body0_e86435_d_n11, assign55830_body0_e86435_d_n14,) = {
    if (((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1399 != 0.0)) && (locals.var_guard1400 != 0.0)) && (locals.var_guard1401 != 0.0)) {
        let assign55830_body0_e86433: f64 = (locals.var_dnm).sqrt();
        (assign55830_body0_e86433, (locals.var_dnm_dn0 / (2.0 * assign55830_body0_e86433)), (locals.var_dnm_dn2 / (2.0 * assign55830_body0_e86433)), (locals.var_dnm_dn4 / (2.0 * assign55830_body0_e86433)), (locals.var_dnm_dn5 / (2.0 * assign55830_body0_e86433)), (locals.var_dnm_dn6 / (2.0 * assign55830_body0_e86433)), (locals.var_dnm_dn7 / (2.0 * assign55830_body0_e86433)), (locals.var_dnm_dn8 / (2.0 * assign55830_body0_e86433)), (locals.var_dnm_dn9 / (2.0 * assign55830_body0_e86433)), (locals.var_dnm_dn10 / (2.0 * assign55830_body0_e86433)), (locals.var_dnm_dn11 / (2.0 * assign55830_body0_e86433)), (locals.var_dnm_dn14 / (2.0 * assign55830_body0_e86433)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
            locals.var_dnm = assign55830_body0_e86435;
            locals.var_dnm_dn0 = assign55830_body0_e86435_d_n0;
            locals.var_dnm_dn2 = assign55830_body0_e86435_d_n2;
            locals.var_dnm_dn4 = assign55830_body0_e86435_d_n4;
            locals.var_dnm_dn5 = assign55830_body0_e86435_d_n5;
            locals.var_dnm_dn6 = assign55830_body0_e86435_d_n6;
            locals.var_dnm_dn7 = assign55830_body0_e86435_d_n7;
            locals.var_dnm_dn8 = assign55830_body0_e86435_d_n8;
            locals.var_dnm_dn9 = assign55830_body0_e86435_d_n9;
            locals.var_dnm_dn10 = assign55830_body0_e86435_d_n10;
            locals.var_dnm_dn11 = assign55830_body0_e86435_d_n11;
            locals.var_dnm_dn14 = assign55830_body0_e86435_d_n14;
            let (assign55830_body1_e86454,) = {
    if (((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1399 != 0.0)) && (locals.var_guard1400 != 0.0)) && (locals.var_guard1401 != 0.0)) {
        let assign55830_body1_e86452: f64 = (locals.var_m0 + 1.0);
        (assign55830_body1_e86452,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign55830_body1_e86454;
        }

        let (assign55840_e86483, assign55840_e86483_d_n0, assign55840_e86483_d_n2, assign55840_e86483_d_n4, assign55840_e86483_d_n5, assign55840_e86483_d_n6, assign55840_e86483_d_n7, assign55840_e86483_d_n8, assign55840_e86483_d_n9, assign55840_e86483_d_n10, assign55840_e86483_d_n11, assign55840_e86483_d_n14,) = {
    if (((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1399 != 0.0)) && (locals.var_guard1400 != 0.0)) && (locals.var_guard1401 == 0.0)) {
        let (assign55840_e86481, assign55840_e86481_d_n0, assign55840_e86481_d_n2, assign55840_e86481_d_n4, assign55840_e86481_d_n5, assign55840_e86481_d_n6, assign55840_e86481_d_n7, assign55840_e86481_d_n8, assign55840_e86481_d_n9, assign55840_e86481_d_n10, assign55840_e86481_d_n11, assign55840_e86481_d_n14,) = {
            if (locals.var_dnm == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign55840_e86478: f64 = (2.0 * 2.0);
                let assign55840_e86479: f64 = (1.0 / assign55840_e86478);
                let assign55840_e86480: f64 = (locals.var_dnm).powf(assign55840_e86479);
                (assign55840_e86480, if 0.0 == 0.0 && ((assign55840_e86479) as f64).is_finite() && ((assign55840_e86479) as f64).fract() == 0.0 { if assign55840_e86479 == 0.0 { 0.0 } else { (assign55840_e86479 * ((locals.var_dnm).powf(assign55840_e86479 - 1.0) * locals.var_dnm_dn0)) } } else { (assign55840_e86480 * (assign55840_e86479 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign55840_e86479) as f64).is_finite() && ((assign55840_e86479) as f64).fract() == 0.0 { if assign55840_e86479 == 0.0 { 0.0 } else { (assign55840_e86479 * ((locals.var_dnm).powf(assign55840_e86479 - 1.0) * locals.var_dnm_dn2)) } } else { (assign55840_e86480 * (assign55840_e86479 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign55840_e86479) as f64).is_finite() && ((assign55840_e86479) as f64).fract() == 0.0 { if assign55840_e86479 == 0.0 { 0.0 } else { (assign55840_e86479 * ((locals.var_dnm).powf(assign55840_e86479 - 1.0) * locals.var_dnm_dn4)) } } else { (assign55840_e86480 * (assign55840_e86479 * (locals.var_dnm_dn4 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign55840_e86479) as f64).is_finite() && ((assign55840_e86479) as f64).fract() == 0.0 { if assign55840_e86479 == 0.0 { 0.0 } else { (assign55840_e86479 * ((locals.var_dnm).powf(assign55840_e86479 - 1.0) * locals.var_dnm_dn5)) } } else { (assign55840_e86480 * (assign55840_e86479 * (locals.var_dnm_dn5 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign55840_e86479) as f64).is_finite() && ((assign55840_e86479) as f64).fract() == 0.0 { if assign55840_e86479 == 0.0 { 0.0 } else { (assign55840_e86479 * ((locals.var_dnm).powf(assign55840_e86479 - 1.0) * locals.var_dnm_dn6)) } } else { (assign55840_e86480 * (assign55840_e86479 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign55840_e86479) as f64).is_finite() && ((assign55840_e86479) as f64).fract() == 0.0 { if assign55840_e86479 == 0.0 { 0.0 } else { (assign55840_e86479 * ((locals.var_dnm).powf(assign55840_e86479 - 1.0) * locals.var_dnm_dn7)) } } else { (assign55840_e86480 * (assign55840_e86479 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign55840_e86479) as f64).is_finite() && ((assign55840_e86479) as f64).fract() == 0.0 { if assign55840_e86479 == 0.0 { 0.0 } else { (assign55840_e86479 * ((locals.var_dnm).powf(assign55840_e86479 - 1.0) * locals.var_dnm_dn8)) } } else { (assign55840_e86480 * (assign55840_e86479 * (locals.var_dnm_dn8 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign55840_e86479) as f64).is_finite() && ((assign55840_e86479) as f64).fract() == 0.0 { if assign55840_e86479 == 0.0 { 0.0 } else { (assign55840_e86479 * ((locals.var_dnm).powf(assign55840_e86479 - 1.0) * locals.var_dnm_dn9)) } } else { (assign55840_e86480 * (assign55840_e86479 * (locals.var_dnm_dn9 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign55840_e86479) as f64).is_finite() && ((assign55840_e86479) as f64).fract() == 0.0 { if assign55840_e86479 == 0.0 { 0.0 } else { (assign55840_e86479 * ((locals.var_dnm).powf(assign55840_e86479 - 1.0) * locals.var_dnm_dn10)) } } else { (assign55840_e86480 * (assign55840_e86479 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign55840_e86479) as f64).is_finite() && ((assign55840_e86479) as f64).fract() == 0.0 { if assign55840_e86479 == 0.0 { 0.0 } else { (assign55840_e86479 * ((locals.var_dnm).powf(assign55840_e86479 - 1.0) * locals.var_dnm_dn11)) } } else { (assign55840_e86480 * (assign55840_e86479 * (locals.var_dnm_dn11 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign55840_e86479) as f64).is_finite() && ((assign55840_e86479) as f64).fract() == 0.0 { if assign55840_e86479 == 0.0 { 0.0 } else { (assign55840_e86479 * ((locals.var_dnm).powf(assign55840_e86479 - 1.0) * locals.var_dnm_dn14)) } } else { (assign55840_e86480 * (assign55840_e86479 * (locals.var_dnm_dn14 / locals.var_dnm))) },)
            }
        };
        (assign55840_e86481, assign55840_e86481_d_n0, assign55840_e86481_d_n2, assign55840_e86481_d_n4, assign55840_e86481_d_n5, assign55840_e86481_d_n6, assign55840_e86481_d_n7, assign55840_e86481_d_n8, assign55840_e86481_d_n9, assign55840_e86481_d_n10, assign55840_e86481_d_n11, assign55840_e86481_d_n14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign55840_e86483;
        locals.var_dnm_dn0 = assign55840_e86483_d_n0;
        locals.var_dnm_dn2 = assign55840_e86483_d_n2;
        locals.var_dnm_dn4 = assign55840_e86483_d_n4;
        locals.var_dnm_dn5 = assign55840_e86483_d_n5;
        locals.var_dnm_dn6 = assign55840_e86483_d_n6;
        locals.var_dnm_dn7 = assign55840_e86483_d_n7;
        locals.var_dnm_dn8 = assign55840_e86483_d_n8;
        locals.var_dnm_dn9 = assign55840_e86483_d_n9;
        locals.var_dnm_dn10 = assign55840_e86483_d_n10;
        locals.var_dnm_dn11 = assign55840_e86483_d_n11;
        locals.var_dnm_dn14 = assign55840_e86483_d_n14;

        let (assign55850_e86500, assign55850_e86500_d_n0, assign55850_e86500_d_n2, assign55850_e86500_d_n4, assign55850_e86500_d_n5, assign55850_e86500_d_n6, assign55850_e86500_d_n7, assign55850_e86500_d_n8, assign55850_e86500_d_n9, assign55850_e86500_d_n10, assign55850_e86500_d_n11, assign55850_e86500_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1399 != 0.0)) && (locals.var_guard1400 != 0.0)) {
        let assign55850_e86498: f64 = (1.0 / locals.var_dnm);
        (assign55850_e86498, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn4 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn5 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn8 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn9 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn11 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn14 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign55850_e86500;
        locals.var_dnm_dn0 = assign55850_e86500_d_n0;
        locals.var_dnm_dn2 = assign55850_e86500_d_n2;
        locals.var_dnm_dn4 = assign55850_e86500_d_n4;
        locals.var_dnm_dn5 = assign55850_e86500_d_n5;
        locals.var_dnm_dn6 = assign55850_e86500_d_n6;
        locals.var_dnm_dn7 = assign55850_e86500_d_n7;
        locals.var_dnm_dn8 = assign55850_e86500_d_n8;
        locals.var_dnm_dn9 = assign55850_e86500_d_n9;
        locals.var_dnm_dn10 = assign55850_e86500_d_n10;
        locals.var_dnm_dn11 = assign55850_e86500_d_n11;
        locals.var_dnm_dn14 = assign55850_e86500_d_n14;

        let (assign55860_e86519, assign55860_e86519_d_n0, assign55860_e86519_d_n2, assign55860_e86519_d_n4, assign55860_e86519_d_n5, assign55860_e86519_d_n6, assign55860_e86519_d_n7, assign55860_e86519_d_n8, assign55860_e86519_d_n9, assign55860_e86519_d_n10, assign55860_e86519_d_n11, assign55860_e86519_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1399 != 0.0)) && (locals.var_guard1400 != 0.0)) {
        let assign55860_e86515: f64 = (locals.var_tmf1 * locals.var_t5);
        let assign55860_e86517: f64 = (assign55860_e86515 * locals.var_dnm);
        (assign55860_e86517, ((((locals.var_tmf1_dn0 * locals.var_t5) + (locals.var_tmf1 * locals.var_t5_dn0)) * locals.var_dnm) + (assign55860_e86515 * locals.var_dnm_dn0)), ((((locals.var_tmf1_dn2 * locals.var_t5) + (locals.var_tmf1 * locals.var_t5_dn2)) * locals.var_dnm) + (assign55860_e86515 * locals.var_dnm_dn2)), ((((locals.var_tmf1_dn4 * locals.var_t5) + (locals.var_tmf1 * locals.var_t5_dn4)) * locals.var_dnm) + (assign55860_e86515 * locals.var_dnm_dn4)), ((((locals.var_tmf1_dn5 * locals.var_t5) + (locals.var_tmf1 * locals.var_t5_dn5)) * locals.var_dnm) + (assign55860_e86515 * locals.var_dnm_dn5)), ((((locals.var_tmf1_dn6 * locals.var_t5) + (locals.var_tmf1 * locals.var_t5_dn6)) * locals.var_dnm) + (assign55860_e86515 * locals.var_dnm_dn6)), ((((locals.var_tmf1_dn7 * locals.var_t5) + (locals.var_tmf1 * locals.var_t5_dn7)) * locals.var_dnm) + (assign55860_e86515 * locals.var_dnm_dn7)), ((((locals.var_tmf1_dn8 * locals.var_t5) + (locals.var_tmf1 * locals.var_t5_dn8)) * locals.var_dnm) + (assign55860_e86515 * locals.var_dnm_dn8)), ((((locals.var_tmf1_dn9 * locals.var_t5) + (locals.var_tmf1 * locals.var_t5_dn9)) * locals.var_dnm) + (assign55860_e86515 * locals.var_dnm_dn9)), ((((locals.var_tmf1_dn10 * locals.var_t5) + (locals.var_tmf1 * locals.var_t5_dn10)) * locals.var_dnm) + (assign55860_e86515 * locals.var_dnm_dn10)), ((((locals.var_tmf1_dn11 * locals.var_t5) + (locals.var_tmf1 * locals.var_t5_dn11)) * locals.var_dnm) + (assign55860_e86515 * locals.var_dnm_dn11)), ((((locals.var_tmf1_dn14 * locals.var_t5) + (locals.var_tmf1 * locals.var_t5_dn14)) * locals.var_dnm) + (assign55860_e86515 * locals.var_dnm_dn14)),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn11, locals.var_tmf0_dn14,)
    }
};
        locals.var_tmf0 = assign55860_e86519;
        locals.var_tmf0_dn0 = assign55860_e86519_d_n0;
        locals.var_tmf0_dn2 = assign55860_e86519_d_n2;
        locals.var_tmf0_dn4 = assign55860_e86519_d_n4;
        locals.var_tmf0_dn5 = assign55860_e86519_d_n5;
        locals.var_tmf0_dn6 = assign55860_e86519_d_n6;
        locals.var_tmf0_dn7 = assign55860_e86519_d_n7;
        locals.var_tmf0_dn8 = assign55860_e86519_d_n8;
        locals.var_tmf0_dn9 = assign55860_e86519_d_n9;
        locals.var_tmf0_dn10 = assign55860_e86519_d_n10;
        locals.var_tmf0_dn11 = assign55860_e86519_d_n11;
        locals.var_tmf0_dn14 = assign55860_e86519_d_n14;

        let (assign55870_e86540, assign55870_e86540_d_n0, assign55870_e86540_d_n2, assign55870_e86540_d_n4, assign55870_e86540_d_n5, assign55870_e86540_d_n6, assign55870_e86540_d_n7, assign55870_e86540_d_n8, assign55870_e86540_d_n9, assign55870_e86540_d_n10, assign55870_e86540_d_n11, assign55870_e86540_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1399 != 0.0)) && (locals.var_guard1400 != 0.0)) {
        let assign55870_e86534: f64 = (locals.var_t5 * locals.var_xmp);
        let assign55870_e86536: f64 = (assign55870_e86534 * locals.var_dnm);
        let assign55870_e86538: f64 = (assign55870_e86536 / locals.var_arg);
        (assign55870_e86538, (((((((locals.var_t5_dn0 * locals.var_xmp) + (locals.var_t5 * locals.var_xmp_dn0)) * locals.var_dnm) + (assign55870_e86534 * locals.var_dnm_dn0)) * locals.var_arg) - (assign55870_e86536 * locals.var_arg_dn0)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t5_dn2 * locals.var_xmp) + (locals.var_t5 * locals.var_xmp_dn2)) * locals.var_dnm) + (assign55870_e86534 * locals.var_dnm_dn2)) * locals.var_arg) - (assign55870_e86536 * locals.var_arg_dn2)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t5_dn4 * locals.var_xmp) + (locals.var_t5 * locals.var_xmp_dn4)) * locals.var_dnm) + (assign55870_e86534 * locals.var_dnm_dn4)) * locals.var_arg) - (assign55870_e86536 * locals.var_arg_dn4)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t5_dn5 * locals.var_xmp) + (locals.var_t5 * locals.var_xmp_dn5)) * locals.var_dnm) + (assign55870_e86534 * locals.var_dnm_dn5)) * locals.var_arg) - (assign55870_e86536 * locals.var_arg_dn5)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t5_dn6 * locals.var_xmp) + (locals.var_t5 * locals.var_xmp_dn6)) * locals.var_dnm) + (assign55870_e86534 * locals.var_dnm_dn6)) * locals.var_arg) - (assign55870_e86536 * locals.var_arg_dn6)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t5_dn7 * locals.var_xmp) + (locals.var_t5 * locals.var_xmp_dn7)) * locals.var_dnm) + (assign55870_e86534 * locals.var_dnm_dn7)) * locals.var_arg) - (assign55870_e86536 * locals.var_arg_dn7)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t5_dn8 * locals.var_xmp) + (locals.var_t5 * locals.var_xmp_dn8)) * locals.var_dnm) + (assign55870_e86534 * locals.var_dnm_dn8)) * locals.var_arg) - (assign55870_e86536 * locals.var_arg_dn8)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t5_dn9 * locals.var_xmp) + (locals.var_t5 * locals.var_xmp_dn9)) * locals.var_dnm) + (assign55870_e86534 * locals.var_dnm_dn9)) * locals.var_arg) - (assign55870_e86536 * locals.var_arg_dn9)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t5_dn10 * locals.var_xmp) + (locals.var_t5 * locals.var_xmp_dn10)) * locals.var_dnm) + (assign55870_e86534 * locals.var_dnm_dn10)) * locals.var_arg) - (assign55870_e86536 * locals.var_arg_dn10)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t5_dn11 * locals.var_xmp) + (locals.var_t5 * locals.var_xmp_dn11)) * locals.var_dnm) + (assign55870_e86534 * locals.var_dnm_dn11)) * locals.var_arg) - (assign55870_e86536 * locals.var_arg_dn11)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t5_dn14 * locals.var_xmp) + (locals.var_t5 * locals.var_xmp_dn14)) * locals.var_dnm) + (assign55870_e86534 * locals.var_dnm_dn14)) * locals.var_arg) - (assign55870_e86536 * locals.var_arg_dn14)) / (locals.var_arg * locals.var_arg)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign55870_e86540;
        locals.var_t0_dn0 = assign55870_e86540_d_n0;
        locals.var_t0_dn2 = assign55870_e86540_d_n2;
        locals.var_t0_dn4 = assign55870_e86540_d_n4;
        locals.var_t0_dn5 = assign55870_e86540_d_n5;
        locals.var_t0_dn6 = assign55870_e86540_d_n6;
        locals.var_t0_dn7 = assign55870_e86540_d_n7;
        locals.var_t0_dn8 = assign55870_e86540_d_n8;
        locals.var_t0_dn9 = assign55870_e86540_d_n9;
        locals.var_t0_dn10 = assign55870_e86540_d_n10;
        locals.var_t0_dn11 = assign55870_e86540_d_n11;
        locals.var_t0_dn14 = assign55870_e86540_d_n14;

        let (assign55880_e86559, assign55880_e86559_d_n0, assign55880_e86559_d_n2, assign55880_e86559_d_n4, assign55880_e86559_d_n5, assign55880_e86559_d_n6, assign55880_e86559_d_n7, assign55880_e86559_d_n8, assign55880_e86559_d_n9, assign55880_e86559_d_n10, assign55880_e86559_d_n11, assign55880_e86559_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1399 != 0.0)) && (locals.var_guard1400 != 0.0)) {
        let assign55880_e86555: f64 = locals.var_t5;
        let assign55880_e86557: f64 = (assign55880_e86555 - locals.var_tmf0);
        (assign55880_e86557, (locals.var_t5_dn0 - locals.var_tmf0_dn0), (locals.var_t5_dn2 - locals.var_tmf0_dn2), (locals.var_t5_dn4 - locals.var_tmf0_dn4), (locals.var_t5_dn5 - locals.var_tmf0_dn5), (locals.var_t5_dn6 - locals.var_tmf0_dn6), (locals.var_t5_dn7 - locals.var_tmf0_dn7), (locals.var_t5_dn8 - locals.var_tmf0_dn8), (locals.var_t5_dn9 - locals.var_tmf0_dn9), (locals.var_t5_dn10 - locals.var_tmf0_dn10), (locals.var_t5_dn11 - locals.var_tmf0_dn11), (locals.var_t5_dn14 - locals.var_tmf0_dn14),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign55880_e86559;
        locals.var_t4_dn0 = assign55880_e86559_d_n0;
        locals.var_t4_dn2 = assign55880_e86559_d_n2;
        locals.var_t4_dn4 = assign55880_e86559_d_n4;
        locals.var_t4_dn5 = assign55880_e86559_d_n5;
        locals.var_t4_dn6 = assign55880_e86559_d_n6;
        locals.var_t4_dn7 = assign55880_e86559_d_n7;
        locals.var_t4_dn8 = assign55880_e86559_d_n8;
        locals.var_t4_dn9 = assign55880_e86559_d_n9;
        locals.var_t4_dn10 = assign55880_e86559_d_n10;
        locals.var_t4_dn11 = assign55880_e86559_d_n11;
        locals.var_t4_dn14 = assign55880_e86559_d_n14;

        let (assign55890_e86574, assign55890_e86574_d_n0, assign55890_e86574_d_n2, assign55890_e86574_d_n4, assign55890_e86574_d_n5, assign55890_e86574_d_n6, assign55890_e86574_d_n7, assign55890_e86574_d_n8, assign55890_e86574_d_n9, assign55890_e86574_d_n10, assign55890_e86574_d_n11, assign55890_e86574_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1399 != 0.0)) && (locals.var_guard1400 != 0.0)) {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign55890_e86574;
        locals.var_t0_dn0 = assign55890_e86574_d_n0;
        locals.var_t0_dn2 = assign55890_e86574_d_n2;
        locals.var_t0_dn4 = assign55890_e86574_d_n4;
        locals.var_t0_dn5 = assign55890_e86574_d_n5;
        locals.var_t0_dn6 = assign55890_e86574_d_n6;
        locals.var_t0_dn7 = assign55890_e86574_d_n7;
        locals.var_t0_dn8 = assign55890_e86574_d_n8;
        locals.var_t0_dn9 = assign55890_e86574_d_n9;
        locals.var_t0_dn10 = assign55890_e86574_d_n10;
        locals.var_t0_dn11 = assign55890_e86574_d_n11;
        locals.var_t0_dn14 = assign55890_e86574_d_n14;

        let (assign55900_e86590, assign55900_e86590_d_n0, assign55900_e86590_d_n2, assign55900_e86590_d_n4, assign55900_e86590_d_n5, assign55900_e86590_d_n6, assign55900_e86590_d_n7, assign55900_e86590_d_n8, assign55900_e86590_d_n9, assign55900_e86590_d_n10, assign55900_e86590_d_n11, assign55900_e86590_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard450 != 0.0) && (!((locals.var_guard448 != 0.0) || (locals.var_guard449 != 0.0))))) && (locals.var_guard1399 != 0.0)) && (locals.var_guard1400 == 0.0)) {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign55900_e86590;
        locals.var_t4_dn0 = assign55900_e86590_d_n0;
        locals.var_t4_dn2 = assign55900_e86590_d_n2;
        locals.var_t4_dn4 = assign55900_e86590_d_n4;
        locals.var_t4_dn5 = assign55900_e86590_d_n5;
        locals.var_t4_dn6 = assign55900_e86590_d_n6;
        locals.var_t4_dn7 = assign55900_e86590_d_n7;
        locals.var_t4_dn8 = assign55900_e86590_d_n8;
        locals.var_t4_dn9 = assign55900_e86590_d_n9;
        locals.var_t4_dn10 = assign55900_e86590_d_n10;
        locals.var_t4_dn11 = assign55900_e86590_d_n11;
        locals.var_t4_dn14 = assign55900_e86590_d_n14;

    }
}
