#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_reactive_block_359(
        locals: &mut StampLocals,
    ) {
        let (assign95610_e148202, assign95610_e148202_d_n0, assign95610_e148202_d_n2, assign95610_e148202_d_n4, assign95610_e148202_d_n5, assign95610_e148202_d_n6, assign95610_e148202_d_n7, assign95610_e148202_d_n8, assign95610_e148202_d_n9, assign95610_e148202_d_n10, assign95610_e148202_d_n13,) = {
    if ((locals.var_guard2222 != 0.0) && (locals.var_guard2223 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign95610_e148202;
        locals.var_dnm_dn0 = assign95610_e148202_d_n0;
        locals.var_dnm_dn2 = assign95610_e148202_d_n2;
        locals.var_dnm_dn4 = assign95610_e148202_d_n4;
        locals.var_dnm_dn5 = assign95610_e148202_d_n5;
        locals.var_dnm_dn6 = assign95610_e148202_d_n6;
        locals.var_dnm_dn7 = assign95610_e148202_d_n7;
        locals.var_dnm_dn8 = assign95610_e148202_d_n8;
        locals.var_dnm_dn9 = assign95610_e148202_d_n9;
        locals.var_dnm_dn10 = assign95610_e148202_d_n10;
        locals.var_dnm_dn13 = assign95610_e148202_d_n13;
        locals.var_dnm_rv = 0.0;

        let (assign95620_e148210, assign95620_e148210_d_n0, assign95620_e148210_d_n2, assign95620_e148210_d_n4, assign95620_e148210_d_n5, assign95620_e148210_d_n6, assign95620_e148210_d_n7, assign95620_e148210_d_n8, assign95620_e148210_d_n9, assign95620_e148210_d_n10, assign95620_e148210_d_n13,) = {
    if ((locals.var_guard2222 != 0.0) && (locals.var_guard2223 != 0.0)) {
        let assign95620_e148208: f64 = (locals.var_xp * locals.var_x2);
        (assign95620_e148208, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn13 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn13)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
        locals.var_xp = assign95620_e148210;
        locals.var_xp_dn0 = assign95620_e148210_d_n0;
        locals.var_xp_dn2 = assign95620_e148210_d_n2;
        locals.var_xp_dn4 = assign95620_e148210_d_n4;
        locals.var_xp_dn5 = assign95620_e148210_d_n5;
        locals.var_xp_dn6 = assign95620_e148210_d_n6;
        locals.var_xp_dn7 = assign95620_e148210_d_n7;
        locals.var_xp_dn8 = assign95620_e148210_d_n8;
        locals.var_xp_dn9 = assign95620_e148210_d_n9;
        locals.var_xp_dn10 = assign95620_e148210_d_n10;
        locals.var_xp_dn13 = assign95620_e148210_d_n13;
        locals.var_xp_rv = 0.0;

        let (assign95630_e148218, assign95630_e148218_d_n0, assign95630_e148218_d_n2, assign95630_e148218_d_n4, assign95630_e148218_d_n5, assign95630_e148218_d_n6, assign95630_e148218_d_n7, assign95630_e148218_d_n8, assign95630_e148218_d_n9, assign95630_e148218_d_n10, assign95630_e148218_d_n13,) = {
    if ((locals.var_guard2222 != 0.0) && (locals.var_guard2223 != 0.0)) {
        let assign95630_e148216: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign95630_e148216, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn13 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn13)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
        locals.var_xmp = assign95630_e148218;
        locals.var_xmp_dn0 = assign95630_e148218_d_n0;
        locals.var_xmp_dn2 = assign95630_e148218_d_n2;
        locals.var_xmp_dn4 = assign95630_e148218_d_n4;
        locals.var_xmp_dn5 = assign95630_e148218_d_n5;
        locals.var_xmp_dn6 = assign95630_e148218_d_n6;
        locals.var_xmp_dn7 = assign95630_e148218_d_n7;
        locals.var_xmp_dn8 = assign95630_e148218_d_n8;
        locals.var_xmp_dn9 = assign95630_e148218_d_n9;
        locals.var_xmp_dn10 = assign95630_e148218_d_n10;
        locals.var_xmp_dn13 = assign95630_e148218_d_n13;
        locals.var_xmp_rv = 0.0;

        let (assign95640_e148226, assign95640_e148226_d_n0, assign95640_e148226_d_n2, assign95640_e148226_d_n4, assign95640_e148226_d_n5, assign95640_e148226_d_n6, assign95640_e148226_d_n7, assign95640_e148226_d_n8, assign95640_e148226_d_n9, assign95640_e148226_d_n10, assign95640_e148226_d_n13,) = {
    if ((locals.var_guard2222 != 0.0) && (locals.var_guard2223 != 0.0)) {
        let assign95640_e148224: f64 = (locals.var_xp * locals.var_x2);
        (assign95640_e148224, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn13 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn13)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
        locals.var_xp = assign95640_e148226;
        locals.var_xp_dn0 = assign95640_e148226_d_n0;
        locals.var_xp_dn2 = assign95640_e148226_d_n2;
        locals.var_xp_dn4 = assign95640_e148226_d_n4;
        locals.var_xp_dn5 = assign95640_e148226_d_n5;
        locals.var_xp_dn6 = assign95640_e148226_d_n6;
        locals.var_xp_dn7 = assign95640_e148226_d_n7;
        locals.var_xp_dn8 = assign95640_e148226_d_n8;
        locals.var_xp_dn9 = assign95640_e148226_d_n9;
        locals.var_xp_dn10 = assign95640_e148226_d_n10;
        locals.var_xp_dn13 = assign95640_e148226_d_n13;
        locals.var_xp_rv = 0.0;

        let (assign95650_e148234, assign95650_e148234_d_n0, assign95650_e148234_d_n2, assign95650_e148234_d_n4, assign95650_e148234_d_n5, assign95650_e148234_d_n6, assign95650_e148234_d_n7, assign95650_e148234_d_n8, assign95650_e148234_d_n9, assign95650_e148234_d_n10, assign95650_e148234_d_n13,) = {
    if ((locals.var_guard2222 != 0.0) && (locals.var_guard2223 != 0.0)) {
        let assign95650_e148232: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign95650_e148232, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn13 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn13)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
        locals.var_xmp = assign95650_e148234;
        locals.var_xmp_dn0 = assign95650_e148234_d_n0;
        locals.var_xmp_dn2 = assign95650_e148234_d_n2;
        locals.var_xmp_dn4 = assign95650_e148234_d_n4;
        locals.var_xmp_dn5 = assign95650_e148234_d_n5;
        locals.var_xmp_dn6 = assign95650_e148234_d_n6;
        locals.var_xmp_dn7 = assign95650_e148234_d_n7;
        locals.var_xmp_dn8 = assign95650_e148234_d_n8;
        locals.var_xmp_dn9 = assign95650_e148234_d_n9;
        locals.var_xmp_dn10 = assign95650_e148234_d_n10;
        locals.var_xmp_dn13 = assign95650_e148234_d_n13;
        locals.var_xmp_rv = 0.0;

        let (assign95660_e148242, assign95660_e148242_d_n0, assign95660_e148242_d_n2, assign95660_e148242_d_n4, assign95660_e148242_d_n5, assign95660_e148242_d_n6, assign95660_e148242_d_n7, assign95660_e148242_d_n8, assign95660_e148242_d_n9, assign95660_e148242_d_n10, assign95660_e148242_d_n13,) = {
    if ((locals.var_guard2222 != 0.0) && (locals.var_guard2223 != 0.0)) {
        let assign95660_e148240: f64 = (locals.var_xp + locals.var_xmp);
        (assign95660_e148240, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn4 + locals.var_xmp_dn4), (locals.var_xp_dn5 + locals.var_xmp_dn5), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn8 + locals.var_xmp_dn8), (locals.var_xp_dn9 + locals.var_xmp_dn9), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn13 + locals.var_xmp_dn13),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    }
};
        locals.var_arg = assign95660_e148242;
        locals.var_arg_dn0 = assign95660_e148242_d_n0;
        locals.var_arg_dn2 = assign95660_e148242_d_n2;
        locals.var_arg_dn4 = assign95660_e148242_d_n4;
        locals.var_arg_dn5 = assign95660_e148242_d_n5;
        locals.var_arg_dn6 = assign95660_e148242_d_n6;
        locals.var_arg_dn7 = assign95660_e148242_d_n7;
        locals.var_arg_dn8 = assign95660_e148242_d_n8;
        locals.var_arg_dn9 = assign95660_e148242_d_n9;
        locals.var_arg_dn10 = assign95660_e148242_d_n10;
        locals.var_arg_dn13 = assign95660_e148242_d_n13;
        locals.var_arg_rv = 0.0;

        let (assign95670_e148248, assign95670_e148248_d_n0, assign95670_e148248_d_n2, assign95670_e148248_d_n4, assign95670_e148248_d_n5, assign95670_e148248_d_n6, assign95670_e148248_d_n7, assign95670_e148248_d_n8, assign95670_e148248_d_n9, assign95670_e148248_d_n10, assign95670_e148248_d_n13,) = {
    if ((locals.var_guard2222 != 0.0) && (locals.var_guard2223 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign95670_e148248;
        locals.var_dnm_dn0 = assign95670_e148248_d_n0;
        locals.var_dnm_dn2 = assign95670_e148248_d_n2;
        locals.var_dnm_dn4 = assign95670_e148248_d_n4;
        locals.var_dnm_dn5 = assign95670_e148248_d_n5;
        locals.var_dnm_dn6 = assign95670_e148248_d_n6;
        locals.var_dnm_dn7 = assign95670_e148248_d_n7;
        locals.var_dnm_dn8 = assign95670_e148248_d_n8;
        locals.var_dnm_dn9 = assign95670_e148248_d_n9;
        locals.var_dnm_dn10 = assign95670_e148248_d_n10;
        locals.var_dnm_dn13 = assign95670_e148248_d_n13;
        locals.var_dnm_rv = 0.0;

        let assign95680_e148263: f64 = if ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard2224 = assign95680_e148263;
        locals.var_guard2224_rv = 0.0;

        let assign95690_e148266: f64 = if 2.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard2225 = assign95690_e148266;
        locals.var_guard2225_rv = 0.0;

        let (assign95700_e148276,) = {
    if ((((locals.var_guard2222 != 0.0) && (locals.var_guard2223 != 0.0)) && (locals.var_guard2224 != 0.0)) && (locals.var_guard2225 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign95700_e148276;
        locals.var_mm_rv = 0.0;

        let assign95710_e148279: f64 = if 2.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard2226 = assign95710_e148279;
        locals.var_guard2226_rv = 0.0;

        let (assign95720_e148292,) = {
    if (((((locals.var_guard2222 != 0.0) && (locals.var_guard2223 != 0.0)) && (locals.var_guard2224 != 0.0)) && (locals.var_guard2225 == 0.0)) && (locals.var_guard2226 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign95720_e148292;
        locals.var_mm_rv = 0.0;

        let assign95730_e148295: f64 = if 2.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard2227 = assign95730_e148295;
        locals.var_guard2227_rv = 0.0;

        let (assign95740_e148311,) = {
    if ((((((locals.var_guard2222 != 0.0) && (locals.var_guard2223 != 0.0)) && (locals.var_guard2224 != 0.0)) && (locals.var_guard2225 == 0.0)) && (locals.var_guard2226 == 0.0)) && (locals.var_guard2227 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign95740_e148311;
        locals.var_mm_rv = 0.0;

        let assign95750_e148314: f64 = if 2.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard2228 = assign95750_e148314;
        locals.var_guard2228_rv = 0.0;

        let (assign95760_e148333,) = {
    if (((((((locals.var_guard2222 != 0.0) && (locals.var_guard2223 != 0.0)) && (locals.var_guard2224 != 0.0)) && (locals.var_guard2225 == 0.0)) && (locals.var_guard2226 == 0.0)) && (locals.var_guard2227 == 0.0)) && (locals.var_guard2228 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign95760_e148333;
        locals.var_mm_rv = 0.0;

        let (assign95770_e148341,) = {
    if (((locals.var_guard2222 != 0.0) && (locals.var_guard2223 != 0.0)) && (locals.var_guard2224 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign95770_e148341;
        locals.var_m0_rv = 0.0;

        let mut assign95780_loop_guard: usize = 0;
        while {
            let assign95780_cond_e148350: f64 = if ((((locals.var_guard2222 != 0.0) && (locals.var_guard2223 != 0.0)) && (locals.var_guard2224 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign95780_cond_e148350 != 0.0
        } {
            assign95780_loop_guard += 1;
            assert!(assign95780_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign95780_body0_e148359, assign95780_body0_e148359_d_n0, assign95780_body0_e148359_d_n2, assign95780_body0_e148359_d_n4, assign95780_body0_e148359_d_n5, assign95780_body0_e148359_d_n6, assign95780_body0_e148359_d_n7, assign95780_body0_e148359_d_n8, assign95780_body0_e148359_d_n9, assign95780_body0_e148359_d_n10, assign95780_body0_e148359_d_n13,) = {
    if (((locals.var_guard2222 != 0.0) && (locals.var_guard2223 != 0.0)) && (locals.var_guard2224 != 0.0)) {
        let assign95780_body0_e148357: f64 = (locals.var_dnm).sqrt();
        (assign95780_body0_e148357, (locals.var_dnm_dn0 / (2.0 * assign95780_body0_e148357)), (locals.var_dnm_dn2 / (2.0 * assign95780_body0_e148357)), (locals.var_dnm_dn4 / (2.0 * assign95780_body0_e148357)), (locals.var_dnm_dn5 / (2.0 * assign95780_body0_e148357)), (locals.var_dnm_dn6 / (2.0 * assign95780_body0_e148357)), (locals.var_dnm_dn7 / (2.0 * assign95780_body0_e148357)), (locals.var_dnm_dn8 / (2.0 * assign95780_body0_e148357)), (locals.var_dnm_dn9 / (2.0 * assign95780_body0_e148357)), (locals.var_dnm_dn10 / (2.0 * assign95780_body0_e148357)), (locals.var_dnm_dn13 / (2.0 * assign95780_body0_e148357)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
            locals.var_dnm = assign95780_body0_e148359;
            locals.var_dnm_dn0 = assign95780_body0_e148359_d_n0;
            locals.var_dnm_dn2 = assign95780_body0_e148359_d_n2;
            locals.var_dnm_dn4 = assign95780_body0_e148359_d_n4;
            locals.var_dnm_dn5 = assign95780_body0_e148359_d_n5;
            locals.var_dnm_dn6 = assign95780_body0_e148359_d_n6;
            locals.var_dnm_dn7 = assign95780_body0_e148359_d_n7;
            locals.var_dnm_dn8 = assign95780_body0_e148359_d_n8;
            locals.var_dnm_dn9 = assign95780_body0_e148359_d_n9;
            locals.var_dnm_dn10 = assign95780_body0_e148359_d_n10;
            locals.var_dnm_dn13 = assign95780_body0_e148359_d_n13;
            locals.var_dnm_rv = 0.0;
            let (assign95780_body1_e148369,) = {
    if (((locals.var_guard2222 != 0.0) && (locals.var_guard2223 != 0.0)) && (locals.var_guard2224 != 0.0)) {
        let assign95780_body1_e148367: f64 = (locals.var_m0 + 1.0);
        (assign95780_body1_e148367,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign95780_body1_e148369;
            locals.var_m0_rv = 0.0;
        }

        let (assign95790_e148389, assign95790_e148389_d_n0, assign95790_e148389_d_n2, assign95790_e148389_d_n4, assign95790_e148389_d_n5, assign95790_e148389_d_n6, assign95790_e148389_d_n7, assign95790_e148389_d_n8, assign95790_e148389_d_n9, assign95790_e148389_d_n10, assign95790_e148389_d_n13,) = {
    if (((locals.var_guard2222 != 0.0) && (locals.var_guard2223 != 0.0)) && (locals.var_guard2224 == 0.0)) {
        let (assign95790_e148387, assign95790_e148387_d_n0, assign95790_e148387_d_n2, assign95790_e148387_d_n4, assign95790_e148387_d_n5, assign95790_e148387_d_n6, assign95790_e148387_d_n7, assign95790_e148387_d_n8, assign95790_e148387_d_n9, assign95790_e148387_d_n10, assign95790_e148387_d_n13,) = {
            if (locals.var_dnm == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign95790_e148384: f64 = (2.0 * 2.0);
                let assign95790_e148385: f64 = (1.0 / assign95790_e148384);
                let assign95790_e148386: f64 = (locals.var_dnm).powf(assign95790_e148385);
                (assign95790_e148386, if 0.0 == 0.0 && ((assign95790_e148385) as f64).is_finite() && ((assign95790_e148385) as f64).fract() == 0.0 { if assign95790_e148385 == 0.0 { 0.0 } else { (assign95790_e148385 * ((locals.var_dnm).powf(assign95790_e148385 - 1.0) * locals.var_dnm_dn0)) } } else { (assign95790_e148386 * (assign95790_e148385 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign95790_e148385) as f64).is_finite() && ((assign95790_e148385) as f64).fract() == 0.0 { if assign95790_e148385 == 0.0 { 0.0 } else { (assign95790_e148385 * ((locals.var_dnm).powf(assign95790_e148385 - 1.0) * locals.var_dnm_dn2)) } } else { (assign95790_e148386 * (assign95790_e148385 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign95790_e148385) as f64).is_finite() && ((assign95790_e148385) as f64).fract() == 0.0 { if assign95790_e148385 == 0.0 { 0.0 } else { (assign95790_e148385 * ((locals.var_dnm).powf(assign95790_e148385 - 1.0) * locals.var_dnm_dn4)) } } else { (assign95790_e148386 * (assign95790_e148385 * (locals.var_dnm_dn4 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign95790_e148385) as f64).is_finite() && ((assign95790_e148385) as f64).fract() == 0.0 { if assign95790_e148385 == 0.0 { 0.0 } else { (assign95790_e148385 * ((locals.var_dnm).powf(assign95790_e148385 - 1.0) * locals.var_dnm_dn5)) } } else { (assign95790_e148386 * (assign95790_e148385 * (locals.var_dnm_dn5 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign95790_e148385) as f64).is_finite() && ((assign95790_e148385) as f64).fract() == 0.0 { if assign95790_e148385 == 0.0 { 0.0 } else { (assign95790_e148385 * ((locals.var_dnm).powf(assign95790_e148385 - 1.0) * locals.var_dnm_dn6)) } } else { (assign95790_e148386 * (assign95790_e148385 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign95790_e148385) as f64).is_finite() && ((assign95790_e148385) as f64).fract() == 0.0 { if assign95790_e148385 == 0.0 { 0.0 } else { (assign95790_e148385 * ((locals.var_dnm).powf(assign95790_e148385 - 1.0) * locals.var_dnm_dn7)) } } else { (assign95790_e148386 * (assign95790_e148385 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign95790_e148385) as f64).is_finite() && ((assign95790_e148385) as f64).fract() == 0.0 { if assign95790_e148385 == 0.0 { 0.0 } else { (assign95790_e148385 * ((locals.var_dnm).powf(assign95790_e148385 - 1.0) * locals.var_dnm_dn8)) } } else { (assign95790_e148386 * (assign95790_e148385 * (locals.var_dnm_dn8 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign95790_e148385) as f64).is_finite() && ((assign95790_e148385) as f64).fract() == 0.0 { if assign95790_e148385 == 0.0 { 0.0 } else { (assign95790_e148385 * ((locals.var_dnm).powf(assign95790_e148385 - 1.0) * locals.var_dnm_dn9)) } } else { (assign95790_e148386 * (assign95790_e148385 * (locals.var_dnm_dn9 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign95790_e148385) as f64).is_finite() && ((assign95790_e148385) as f64).fract() == 0.0 { if assign95790_e148385 == 0.0 { 0.0 } else { (assign95790_e148385 * ((locals.var_dnm).powf(assign95790_e148385 - 1.0) * locals.var_dnm_dn10)) } } else { (assign95790_e148386 * (assign95790_e148385 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign95790_e148385) as f64).is_finite() && ((assign95790_e148385) as f64).fract() == 0.0 { if assign95790_e148385 == 0.0 { 0.0 } else { (assign95790_e148385 * ((locals.var_dnm).powf(assign95790_e148385 - 1.0) * locals.var_dnm_dn13)) } } else { (assign95790_e148386 * (assign95790_e148385 * (locals.var_dnm_dn13 / locals.var_dnm))) },)
            }
        };
        (assign95790_e148387, assign95790_e148387_d_n0, assign95790_e148387_d_n2, assign95790_e148387_d_n4, assign95790_e148387_d_n5, assign95790_e148387_d_n6, assign95790_e148387_d_n7, assign95790_e148387_d_n8, assign95790_e148387_d_n9, assign95790_e148387_d_n10, assign95790_e148387_d_n13,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign95790_e148389;
        locals.var_dnm_dn0 = assign95790_e148389_d_n0;
        locals.var_dnm_dn2 = assign95790_e148389_d_n2;
        locals.var_dnm_dn4 = assign95790_e148389_d_n4;
        locals.var_dnm_dn5 = assign95790_e148389_d_n5;
        locals.var_dnm_dn6 = assign95790_e148389_d_n6;
        locals.var_dnm_dn7 = assign95790_e148389_d_n7;
        locals.var_dnm_dn8 = assign95790_e148389_d_n8;
        locals.var_dnm_dn9 = assign95790_e148389_d_n9;
        locals.var_dnm_dn10 = assign95790_e148389_d_n10;
        locals.var_dnm_dn13 = assign95790_e148389_d_n13;
        locals.var_dnm_rv = 0.0;

        let (assign95800_e148397, assign95800_e148397_d_n0, assign95800_e148397_d_n2, assign95800_e148397_d_n4, assign95800_e148397_d_n5, assign95800_e148397_d_n6, assign95800_e148397_d_n7, assign95800_e148397_d_n8, assign95800_e148397_d_n9, assign95800_e148397_d_n10, assign95800_e148397_d_n13,) = {
    if ((locals.var_guard2222 != 0.0) && (locals.var_guard2223 != 0.0)) {
        let assign95800_e148395: f64 = (1.0 / locals.var_dnm);
        (assign95800_e148395, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn4 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn5 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn8 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn9 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn13 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign95800_e148397;
        locals.var_dnm_dn0 = assign95800_e148397_d_n0;
        locals.var_dnm_dn2 = assign95800_e148397_d_n2;
        locals.var_dnm_dn4 = assign95800_e148397_d_n4;
        locals.var_dnm_dn5 = assign95800_e148397_d_n5;
        locals.var_dnm_dn6 = assign95800_e148397_d_n6;
        locals.var_dnm_dn7 = assign95800_e148397_d_n7;
        locals.var_dnm_dn8 = assign95800_e148397_d_n8;
        locals.var_dnm_dn9 = assign95800_e148397_d_n9;
        locals.var_dnm_dn10 = assign95800_e148397_d_n10;
        locals.var_dnm_dn13 = assign95800_e148397_d_n13;
        locals.var_dnm_rv = 0.0;

        let (assign95810_e148409, assign95810_e148409_d_n0, assign95810_e148409_d_n2, assign95810_e148409_d_n4, assign95810_e148409_d_n5, assign95810_e148409_d_n6, assign95810_e148409_d_n7, assign95810_e148409_d_n8, assign95810_e148409_d_n9, assign95810_e148409_d_n10, assign95810_e148409_d_n13,) = {
    if ((locals.var_guard2222 != 0.0) && (locals.var_guard2223 != 0.0)) {
        let assign95810_e148404: f64 = (10.0 * 2.220446049250313e-16);
        let assign95810_e148405: f64 = (locals.var_tmf1 * assign95810_e148404);
        let assign95810_e148407: f64 = (assign95810_e148405 * locals.var_dnm);
        (assign95810_e148407, (((locals.var_tmf1_dn0 * assign95810_e148404) * locals.var_dnm) + (assign95810_e148405 * locals.var_dnm_dn0)), (((locals.var_tmf1_dn2 * assign95810_e148404) * locals.var_dnm) + (assign95810_e148405 * locals.var_dnm_dn2)), (((locals.var_tmf1_dn4 * assign95810_e148404) * locals.var_dnm) + (assign95810_e148405 * locals.var_dnm_dn4)), (((locals.var_tmf1_dn5 * assign95810_e148404) * locals.var_dnm) + (assign95810_e148405 * locals.var_dnm_dn5)), (((locals.var_tmf1_dn6 * assign95810_e148404) * locals.var_dnm) + (assign95810_e148405 * locals.var_dnm_dn6)), (((locals.var_tmf1_dn7 * assign95810_e148404) * locals.var_dnm) + (assign95810_e148405 * locals.var_dnm_dn7)), (((locals.var_tmf1_dn8 * assign95810_e148404) * locals.var_dnm) + (assign95810_e148405 * locals.var_dnm_dn8)), (((locals.var_tmf1_dn9 * assign95810_e148404) * locals.var_dnm) + (assign95810_e148405 * locals.var_dnm_dn9)), (((locals.var_tmf1_dn10 * assign95810_e148404) * locals.var_dnm) + (assign95810_e148405 * locals.var_dnm_dn10)), (((locals.var_tmf1_dn13 * assign95810_e148404) * locals.var_dnm) + (assign95810_e148405 * locals.var_dnm_dn13)),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn13,)
    }
};
        locals.var_tmf0 = assign95810_e148409;
        locals.var_tmf0_dn0 = assign95810_e148409_d_n0;
        locals.var_tmf0_dn2 = assign95810_e148409_d_n2;
        locals.var_tmf0_dn4 = assign95810_e148409_d_n4;
        locals.var_tmf0_dn5 = assign95810_e148409_d_n5;
        locals.var_tmf0_dn6 = assign95810_e148409_d_n6;
        locals.var_tmf0_dn7 = assign95810_e148409_d_n7;
        locals.var_tmf0_dn8 = assign95810_e148409_d_n8;
        locals.var_tmf0_dn9 = assign95810_e148409_d_n9;
        locals.var_tmf0_dn10 = assign95810_e148409_d_n10;
        locals.var_tmf0_dn13 = assign95810_e148409_d_n13;
        locals.var_tmf0_rv = 0.0;

        let (assign95820_e148423, assign95820_e148423_d_n0, assign95820_e148423_d_n2, assign95820_e148423_d_n4, assign95820_e148423_d_n5, assign95820_e148423_d_n6, assign95820_e148423_d_n7, assign95820_e148423_d_n8, assign95820_e148423_d_n9, assign95820_e148423_d_n10, assign95820_e148423_d_n13,) = {
    if ((locals.var_guard2222 != 0.0) && (locals.var_guard2223 != 0.0)) {
        let assign95820_e148415: f64 = (10.0 * 2.220446049250313e-16);
        let assign95820_e148417: f64 = (assign95820_e148415 * locals.var_xmp);
        let assign95820_e148419: f64 = (assign95820_e148417 * locals.var_dnm);
        let assign95820_e148421: f64 = (assign95820_e148419 / locals.var_arg);
        (assign95820_e148421, ((((((assign95820_e148415 * locals.var_xmp_dn0) * locals.var_dnm) + (assign95820_e148417 * locals.var_dnm_dn0)) * locals.var_arg) - (assign95820_e148419 * locals.var_arg_dn0)) / (locals.var_arg * locals.var_arg)), ((((((assign95820_e148415 * locals.var_xmp_dn2) * locals.var_dnm) + (assign95820_e148417 * locals.var_dnm_dn2)) * locals.var_arg) - (assign95820_e148419 * locals.var_arg_dn2)) / (locals.var_arg * locals.var_arg)), ((((((assign95820_e148415 * locals.var_xmp_dn4) * locals.var_dnm) + (assign95820_e148417 * locals.var_dnm_dn4)) * locals.var_arg) - (assign95820_e148419 * locals.var_arg_dn4)) / (locals.var_arg * locals.var_arg)), ((((((assign95820_e148415 * locals.var_xmp_dn5) * locals.var_dnm) + (assign95820_e148417 * locals.var_dnm_dn5)) * locals.var_arg) - (assign95820_e148419 * locals.var_arg_dn5)) / (locals.var_arg * locals.var_arg)), ((((((assign95820_e148415 * locals.var_xmp_dn6) * locals.var_dnm) + (assign95820_e148417 * locals.var_dnm_dn6)) * locals.var_arg) - (assign95820_e148419 * locals.var_arg_dn6)) / (locals.var_arg * locals.var_arg)), ((((((assign95820_e148415 * locals.var_xmp_dn7) * locals.var_dnm) + (assign95820_e148417 * locals.var_dnm_dn7)) * locals.var_arg) - (assign95820_e148419 * locals.var_arg_dn7)) / (locals.var_arg * locals.var_arg)), ((((((assign95820_e148415 * locals.var_xmp_dn8) * locals.var_dnm) + (assign95820_e148417 * locals.var_dnm_dn8)) * locals.var_arg) - (assign95820_e148419 * locals.var_arg_dn8)) / (locals.var_arg * locals.var_arg)), ((((((assign95820_e148415 * locals.var_xmp_dn9) * locals.var_dnm) + (assign95820_e148417 * locals.var_dnm_dn9)) * locals.var_arg) - (assign95820_e148419 * locals.var_arg_dn9)) / (locals.var_arg * locals.var_arg)), ((((((assign95820_e148415 * locals.var_xmp_dn10) * locals.var_dnm) + (assign95820_e148417 * locals.var_dnm_dn10)) * locals.var_arg) - (assign95820_e148419 * locals.var_arg_dn10)) / (locals.var_arg * locals.var_arg)), ((((((assign95820_e148415 * locals.var_xmp_dn13) * locals.var_dnm) + (assign95820_e148417 * locals.var_dnm_dn13)) * locals.var_arg) - (assign95820_e148419 * locals.var_arg_dn13)) / (locals.var_arg * locals.var_arg)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign95820_e148423;
        locals.var_t0_dn0 = assign95820_e148423_d_n0;
        locals.var_t0_dn2 = assign95820_e148423_d_n2;
        locals.var_t0_dn4 = assign95820_e148423_d_n4;
        locals.var_t0_dn5 = assign95820_e148423_d_n5;
        locals.var_t0_dn6 = assign95820_e148423_d_n6;
        locals.var_t0_dn7 = assign95820_e148423_d_n7;
        locals.var_t0_dn8 = assign95820_e148423_d_n8;
        locals.var_t0_dn9 = assign95820_e148423_d_n9;
        locals.var_t0_dn10 = assign95820_e148423_d_n10;
        locals.var_t0_dn13 = assign95820_e148423_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign95830_e148441, assign95830_e148441_d_n0, assign95830_e148441_d_n2, assign95830_e148441_d_n4, assign95830_e148441_d_n5, assign95830_e148441_d_n6, assign95830_e148441_d_n7, assign95830_e148441_d_n8, assign95830_e148441_d_n9, assign95830_e148441_d_n10, assign95830_e148441_d_n13,) = {
    if ((locals.var_guard2222 != 0.0) && (locals.var_guard2223 != 0.0)) {
        let assign95830_e148429: f64 = (locals.var_ps0 + locals.var_vds);
        let assign95830_e148432: f64 = (10.0 * 2.220446049250313e-16);
        let assign95830_e148433: f64 = (assign95830_e148429 - assign95830_e148432);
        let assign95830_e148436: f64 = (10.0 * 2.220446049250313e-16);
        let assign95830_e148437: f64 = (assign95830_e148433 - assign95830_e148436);
        let assign95830_e148439: f64 = (assign95830_e148437 + locals.var_tmf0);
        (assign95830_e148439, ((locals.var_ps0_dn0 + locals.var_vds_dn0) + locals.var_tmf0_dn0), ((locals.var_ps0_dn2 + locals.var_vds_dn2) + locals.var_tmf0_dn2), ((locals.var_ps0_dn4 + locals.var_vds_dn4) + locals.var_tmf0_dn4), ((locals.var_ps0_dn5 + locals.var_vds_dn5) + locals.var_tmf0_dn5), ((locals.var_ps0_dn6 + locals.var_vds_dn6) + locals.var_tmf0_dn6), ((locals.var_ps0_dn7 + locals.var_vds_dn7) + locals.var_tmf0_dn7), ((locals.var_ps0_dn8 + locals.var_vds_dn8) + locals.var_tmf0_dn8), ((locals.var_ps0_dn9 + locals.var_vds_dn9) + locals.var_tmf0_dn9), ((locals.var_ps0_dn10 + locals.var_vds_dn10) + locals.var_tmf0_dn10), ((locals.var_ps0_dn13 + locals.var_vds_dn13) + locals.var_tmf0_dn13),)
    } else {
        (locals.var_psdl, locals.var_psdl_dn0, locals.var_psdl_dn2, locals.var_psdl_dn4, locals.var_psdl_dn5, locals.var_psdl_dn6, locals.var_psdl_dn7, locals.var_psdl_dn8, locals.var_psdl_dn9, locals.var_psdl_dn10, locals.var_psdl_dn13,)
    }
};
        locals.var_psdl = assign95830_e148441;
        locals.var_psdl_dn0 = assign95830_e148441_d_n0;
        locals.var_psdl_dn2 = assign95830_e148441_d_n2;
        locals.var_psdl_dn4 = assign95830_e148441_d_n4;
        locals.var_psdl_dn5 = assign95830_e148441_d_n5;
        locals.var_psdl_dn6 = assign95830_e148441_d_n6;
        locals.var_psdl_dn7 = assign95830_e148441_d_n7;
        locals.var_psdl_dn8 = assign95830_e148441_d_n8;
        locals.var_psdl_dn9 = assign95830_e148441_d_n9;
        locals.var_psdl_dn10 = assign95830_e148441_d_n10;
        locals.var_psdl_dn13 = assign95830_e148441_d_n13;
        locals.var_psdl_rv = 0.0;

        let (assign95840_e148447, assign95840_e148447_d_n0, assign95840_e148447_d_n2, assign95840_e148447_d_n4, assign95840_e148447_d_n5, assign95840_e148447_d_n6, assign95840_e148447_d_n7, assign95840_e148447_d_n8, assign95840_e148447_d_n9, assign95840_e148447_d_n10, assign95840_e148447_d_n13,) = {
    if ((locals.var_guard2222 != 0.0) && (locals.var_guard2223 != 0.0)) {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign95840_e148447;
        locals.var_t0_dn0 = assign95840_e148447_d_n0;
        locals.var_t0_dn2 = assign95840_e148447_d_n2;
        locals.var_t0_dn4 = assign95840_e148447_d_n4;
        locals.var_t0_dn5 = assign95840_e148447_d_n5;
        locals.var_t0_dn6 = assign95840_e148447_d_n6;
        locals.var_t0_dn7 = assign95840_e148447_d_n7;
        locals.var_t0_dn8 = assign95840_e148447_d_n8;
        locals.var_t0_dn9 = assign95840_e148447_d_n9;
        locals.var_t0_dn10 = assign95840_e148447_d_n10;
        locals.var_t0_dn13 = assign95840_e148447_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign95850_e148454, assign95850_e148454_d_n0, assign95850_e148454_d_n2, assign95850_e148454_d_n4, assign95850_e148454_d_n5, assign95850_e148454_d_n6, assign95850_e148454_d_n7, assign95850_e148454_d_n8, assign95850_e148454_d_n9, assign95850_e148454_d_n10, assign95850_e148454_d_n13,) = {
    if ((locals.var_guard2222 != 0.0) && (locals.var_guard2223 == 0.0)) {
        (locals.var_psdl, locals.var_psdl_dn0, locals.var_psdl_dn2, locals.var_psdl_dn4, locals.var_psdl_dn5, locals.var_psdl_dn6, locals.var_psdl_dn7, locals.var_psdl_dn8, locals.var_psdl_dn9, locals.var_psdl_dn10, locals.var_psdl_dn13,)
    } else {
        (locals.var_psdl, locals.var_psdl_dn0, locals.var_psdl_dn2, locals.var_psdl_dn4, locals.var_psdl_dn5, locals.var_psdl_dn6, locals.var_psdl_dn7, locals.var_psdl_dn8, locals.var_psdl_dn9, locals.var_psdl_dn10, locals.var_psdl_dn13,)
    }
};
        locals.var_psdl = assign95850_e148454;
        locals.var_psdl_dn0 = assign95850_e148454_d_n0;
        locals.var_psdl_dn2 = assign95850_e148454_d_n2;
        locals.var_psdl_dn4 = assign95850_e148454_d_n4;
        locals.var_psdl_dn5 = assign95850_e148454_d_n5;
        locals.var_psdl_dn6 = assign95850_e148454_d_n6;
        locals.var_psdl_dn7 = assign95850_e148454_d_n7;
        locals.var_psdl_dn8 = assign95850_e148454_d_n8;
        locals.var_psdl_dn9 = assign95850_e148454_d_n9;
        locals.var_psdl_dn10 = assign95850_e148454_d_n10;
        locals.var_psdl_dn13 = assign95850_e148454_d_n13;
        locals.var_psdl_rv = 0.0;

        let (assign95860_e148461, assign95860_e148461_d_n0, assign95860_e148461_d_n2, assign95860_e148461_d_n4, assign95860_e148461_d_n5, assign95860_e148461_d_n6, assign95860_e148461_d_n7, assign95860_e148461_d_n8, assign95860_e148461_d_n9, assign95860_e148461_d_n10, assign95860_e148461_d_n13,) = {
    if ((locals.var_guard2222 != 0.0) && (locals.var_guard2223 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign95860_e148461;
        locals.var_t0_dn0 = assign95860_e148461_d_n0;
        locals.var_t0_dn2 = assign95860_e148461_d_n2;
        locals.var_t0_dn4 = assign95860_e148461_d_n4;
        locals.var_t0_dn5 = assign95860_e148461_d_n5;
        locals.var_t0_dn6 = assign95860_e148461_d_n6;
        locals.var_t0_dn7 = assign95860_e148461_d_n7;
        locals.var_t0_dn8 = assign95860_e148461_d_n8;
        locals.var_t0_dn9 = assign95860_e148461_d_n9;
        locals.var_t0_dn10 = assign95860_e148461_d_n10;
        locals.var_t0_dn13 = assign95860_e148461_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign95870_e148467, assign95870_e148467_d_n0, assign95870_e148467_d_n2, assign95870_e148467_d_n4, assign95870_e148467_d_n5, assign95870_e148467_d_n6, assign95870_e148467_d_n7, assign95870_e148467_d_n8, assign95870_e148467_d_n9, assign95870_e148467_d_n10, assign95870_e148467_d_n13,) = {
    if ((locals.var_guard2222 != 0.0) && (locals.var_flg_qy != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ec, locals.var_ec_dn0, locals.var_ec_dn2, locals.var_ec_dn4, locals.var_ec_dn5, locals.var_ec_dn6, locals.var_ec_dn7, locals.var_ec_dn8, locals.var_ec_dn9, locals.var_ec_dn10, locals.var_ec_dn13,)
    }
};
        locals.var_ec = assign95870_e148467;
        locals.var_ec_dn0 = assign95870_e148467_d_n0;
        locals.var_ec_dn2 = assign95870_e148467_d_n2;
        locals.var_ec_dn4 = assign95870_e148467_d_n4;
        locals.var_ec_dn5 = assign95870_e148467_d_n5;
        locals.var_ec_dn6 = assign95870_e148467_d_n6;
        locals.var_ec_dn7 = assign95870_e148467_d_n7;
        locals.var_ec_dn8 = assign95870_e148467_d_n8;
        locals.var_ec_dn9 = assign95870_e148467_d_n9;
        locals.var_ec_dn10 = assign95870_e148467_d_n10;
        locals.var_ec_dn13 = assign95870_e148467_d_n13;
        locals.var_ec_rv = 0.0;

        let assign95880_e148474: f64 = if ((locals.var_idd < 1e-15) || (locals.var_vdseff < 1e-6)) { 1.0 } else { 0.0 };
        locals.var_guard2229 = assign95880_e148474;
        locals.var_guard2229_rv = 0.0;

        let (assign95890_e148483, assign95890_e148483_d_n0, assign95890_e148483_d_n2, assign95890_e148483_d_n4, assign95890_e148483_d_n5, assign95890_e148483_d_n6, assign95890_e148483_d_n7, assign95890_e148483_d_n8, assign95890_e148483_d_n9, assign95890_e148483_d_n10, assign95890_e148483_d_n13,) = {
    if (((locals.var_guard2222 == 0.0) && (locals.var_flg_qy != 0.0)) && (locals.var_guard2229 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ec, locals.var_ec_dn0, locals.var_ec_dn2, locals.var_ec_dn4, locals.var_ec_dn5, locals.var_ec_dn6, locals.var_ec_dn7, locals.var_ec_dn8, locals.var_ec_dn9, locals.var_ec_dn10, locals.var_ec_dn13,)
    }
};
        locals.var_ec = assign95890_e148483;
        locals.var_ec_dn0 = assign95890_e148483_d_n0;
        locals.var_ec_dn2 = assign95890_e148483_d_n2;
        locals.var_ec_dn4 = assign95890_e148483_d_n4;
        locals.var_ec_dn5 = assign95890_e148483_d_n5;
        locals.var_ec_dn6 = assign95890_e148483_d_n6;
        locals.var_ec_dn7 = assign95890_e148483_d_n7;
        locals.var_ec_dn8 = assign95890_e148483_d_n8;
        locals.var_ec_dn9 = assign95890_e148483_d_n9;
        locals.var_ec_dn10 = assign95890_e148483_d_n10;
        locals.var_ec_dn13 = assign95890_e148483_d_n13;
        locals.var_ec_rv = 0.0;

        let (assign95900_e148499, assign95900_e148499_d_n0, assign95900_e148499_d_n2, assign95900_e148499_d_n4, assign95900_e148499_d_n5, assign95900_e148499_d_n6, assign95900_e148499_d_n7, assign95900_e148499_d_n8, assign95900_e148499_d_n9, assign95900_e148499_d_n10, assign95900_e148499_d_n13,) = {
    if (((locals.var_guard2222 == 0.0) && (locals.var_flg_qy != 0.0)) && (locals.var_guard2229 == 0.0)) {
        let assign95900_e148493: f64 = (locals.var_idd / locals.var_qn0);
        let assign95900_e148495: f64 = (assign95900_e148493 * locals.var_beta_inv);
        let assign95900_e148497: f64 = (assign95900_e148495 / locals.var_leff);
        (assign95900_e148497, ((((((locals.var_idd_dn0 * locals.var_qn0) - (locals.var_idd * locals.var_qn0_dn0)) / (locals.var_qn0 * locals.var_qn0)) * locals.var_beta_inv) + (assign95900_e148493 * locals.var_beta_inv_dn0)) / locals.var_leff), ((((((locals.var_idd_dn2 * locals.var_qn0) - (locals.var_idd * locals.var_qn0_dn2)) / (locals.var_qn0 * locals.var_qn0)) * locals.var_beta_inv) + (assign95900_e148493 * locals.var_beta_inv_dn2)) / locals.var_leff), ((((((locals.var_idd_dn4 * locals.var_qn0) - (locals.var_idd * locals.var_qn0_dn4)) / (locals.var_qn0 * locals.var_qn0)) * locals.var_beta_inv) + (assign95900_e148493 * locals.var_beta_inv_dn4)) / locals.var_leff), ((((((locals.var_idd_dn5 * locals.var_qn0) - (locals.var_idd * locals.var_qn0_dn5)) / (locals.var_qn0 * locals.var_qn0)) * locals.var_beta_inv) + (assign95900_e148493 * locals.var_beta_inv_dn5)) / locals.var_leff), ((((((locals.var_idd_dn6 * locals.var_qn0) - (locals.var_idd * locals.var_qn0_dn6)) / (locals.var_qn0 * locals.var_qn0)) * locals.var_beta_inv) + (assign95900_e148493 * locals.var_beta_inv_dn6)) / locals.var_leff), ((((((locals.var_idd_dn7 * locals.var_qn0) - (locals.var_idd * locals.var_qn0_dn7)) / (locals.var_qn0 * locals.var_qn0)) * locals.var_beta_inv) + (assign95900_e148493 * locals.var_beta_inv_dn7)) / locals.var_leff), ((((((locals.var_idd_dn8 * locals.var_qn0) - (locals.var_idd * locals.var_qn0_dn8)) / (locals.var_qn0 * locals.var_qn0)) * locals.var_beta_inv) + (assign95900_e148493 * locals.var_beta_inv_dn8)) / locals.var_leff), ((((((locals.var_idd_dn9 * locals.var_qn0) - (locals.var_idd * locals.var_qn0_dn9)) / (locals.var_qn0 * locals.var_qn0)) * locals.var_beta_inv) + (assign95900_e148493 * locals.var_beta_inv_dn9)) / locals.var_leff), ((((((locals.var_idd_dn10 * locals.var_qn0) - (locals.var_idd * locals.var_qn0_dn10)) / (locals.var_qn0 * locals.var_qn0)) * locals.var_beta_inv) + (assign95900_e148493 * locals.var_beta_inv_dn10)) / locals.var_leff), ((((((locals.var_idd_dn13 * locals.var_qn0) - (locals.var_idd * locals.var_qn0_dn13)) / (locals.var_qn0 * locals.var_qn0)) * locals.var_beta_inv) + (assign95900_e148493 * locals.var_beta_inv_dn13)) / locals.var_leff),)
    } else {
        (locals.var_ec, locals.var_ec_dn0, locals.var_ec_dn2, locals.var_ec_dn4, locals.var_ec_dn5, locals.var_ec_dn6, locals.var_ec_dn7, locals.var_ec_dn8, locals.var_ec_dn9, locals.var_ec_dn10, locals.var_ec_dn13,)
    }
};
        locals.var_ec = assign95900_e148499;
        locals.var_ec_dn0 = assign95900_e148499_d_n0;
        locals.var_ec_dn2 = assign95900_e148499_d_n2;
        locals.var_ec_dn4 = assign95900_e148499_d_n4;
        locals.var_ec_dn5 = assign95900_e148499_d_n5;
        locals.var_ec_dn6 = assign95900_e148499_d_n6;
        locals.var_ec_dn7 = assign95900_e148499_d_n7;
        locals.var_ec_dn8 = assign95900_e148499_d_n8;
        locals.var_ec_dn9 = assign95900_e148499_d_n9;
        locals.var_ec_dn10 = assign95900_e148499_d_n10;
        locals.var_ec_dn13 = assign95900_e148499_d_n13;
        locals.var_ec_rv = 0.0;

        let assign95910_e148502: f64 = if locals.var_flg_qy == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2230 = assign95910_e148502;
        locals.var_guard2230_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_360(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign95920_e148506, assign95920_e148506_d_n0, assign95920_e148506_d_n2, assign95920_e148506_d_n4, assign95920_e148506_d_n5, assign95920_e148506_d_n6, assign95920_e148506_d_n7, assign95920_e148506_d_n8, assign95920_e148506_d_n9, assign95920_e148506_d_n10, assign95920_e148506_d_n13,) = {
    if (locals.var_guard2230 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qy, locals.var_qy_dn0, locals.var_qy_dn2, locals.var_qy_dn4, locals.var_qy_dn5, locals.var_qy_dn6, locals.var_qy_dn7, locals.var_qy_dn8, locals.var_qy_dn9, locals.var_qy_dn10, locals.var_qy_dn13,)
    }
};
        locals.var_qy = assign95920_e148506;
        locals.var_qy_dn0 = assign95920_e148506_d_n0;
        locals.var_qy_dn2 = assign95920_e148506_d_n2;
        locals.var_qy_dn4 = assign95920_e148506_d_n4;
        locals.var_qy_dn5 = assign95920_e148506_d_n5;
        locals.var_qy_dn6 = assign95920_e148506_d_n6;
        locals.var_qy_dn7 = assign95920_e148506_d_n7;
        locals.var_qy_dn8 = assign95920_e148506_d_n8;
        locals.var_qy_dn9 = assign95920_e148506_d_n9;
        locals.var_qy_dn10 = assign95920_e148506_d_n10;
        locals.var_qy_dn13 = assign95920_e148506_d_n13;
        locals.var_qy_rv = 0.0;

        let (assign95930_e148517, assign95930_e148517_d_n0, assign95930_e148517_d_n2, assign95930_e148517_d_n4, assign95930_e148517_d_n5, assign95930_e148517_d_n6, assign95930_e148517_d_n7, assign95930_e148517_d_n8, assign95930_e148517_d_n9, assign95930_e148517_d_n10, assign95930_e148517_d_n13,) = {
    if (locals.var_guard2230 == 0.0) {
        let assign95930_e148511: f64 = (1.034943e-10 * locals.var_weffcv_nf);
        let assign95930_e148513: f64 = (assign95930_e148511 * locals.var_wdpl);
        let assign95930_e148515: f64 = (assign95930_e148513 * 1.3);
        (assign95930_e148515, ((assign95930_e148511 * locals.var_wdpl_dn0) * 1.3), ((assign95930_e148511 * locals.var_wdpl_dn2) * 1.3), ((assign95930_e148511 * locals.var_wdpl_dn4) * 1.3), ((assign95930_e148511 * locals.var_wdpl_dn5) * 1.3), ((assign95930_e148511 * locals.var_wdpl_dn6) * 1.3), ((assign95930_e148511 * locals.var_wdpl_dn7) * 1.3), ((assign95930_e148511 * locals.var_wdpl_dn8) * 1.3), ((assign95930_e148511 * locals.var_wdpl_dn9) * 1.3), ((assign95930_e148511 * locals.var_wdpl_dn10) * 1.3), ((assign95930_e148511 * locals.var_wdpl_dn13) * 1.3),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign95930_e148517;
        locals.var_t2_dn0 = assign95930_e148517_d_n0;
        locals.var_t2_dn2 = assign95930_e148517_d_n2;
        locals.var_t2_dn4 = assign95930_e148517_d_n4;
        locals.var_t2_dn5 = assign95930_e148517_d_n5;
        locals.var_t2_dn6 = assign95930_e148517_d_n6;
        locals.var_t2_dn7 = assign95930_e148517_d_n7;
        locals.var_t2_dn8 = assign95930_e148517_d_n8;
        locals.var_t2_dn9 = assign95930_e148517_d_n9;
        locals.var_t2_dn10 = assign95930_e148517_d_n10;
        locals.var_t2_dn13 = assign95930_e148517_d_n13;
        locals.var_t2_rv = 0.0;

        let assign95940_e148520: f64 = if p.p133 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2231 = assign95940_e148520;
        locals.var_guard2231_rv = 0.0;

        let (assign95950_e148531, assign95950_e148531_d_n0, assign95950_e148531_d_n2, assign95950_e148531_d_n4, assign95950_e148531_d_n5, assign95950_e148531_d_n6, assign95950_e148531_d_n7, assign95950_e148531_d_n8, assign95950_e148531_d_n9, assign95950_e148531_d_n10, assign95950_e148531_d_n13,) = {
    if ((locals.var_guard2230 == 0.0) && (locals.var_guard2231 != 0.0)) {
        let assign95950_e148527: f64 = (locals.var_ec * locals.var_leff);
        let assign95950_e148529: f64 = (assign95950_e148527 + locals.var_ps0);
        (assign95950_e148529, ((locals.var_ec_dn0 * locals.var_leff) + locals.var_ps0_dn0), ((locals.var_ec_dn2 * locals.var_leff) + locals.var_ps0_dn2), ((locals.var_ec_dn4 * locals.var_leff) + locals.var_ps0_dn4), ((locals.var_ec_dn5 * locals.var_leff) + locals.var_ps0_dn5), ((locals.var_ec_dn6 * locals.var_leff) + locals.var_ps0_dn6), ((locals.var_ec_dn7 * locals.var_leff) + locals.var_ps0_dn7), ((locals.var_ec_dn8 * locals.var_leff) + locals.var_ps0_dn8), ((locals.var_ec_dn9 * locals.var_leff) + locals.var_ps0_dn9), ((locals.var_ec_dn10 * locals.var_leff) + locals.var_ps0_dn10), ((locals.var_ec_dn13 * locals.var_leff) + locals.var_ps0_dn13),)
    } else {
        (locals.var_pslk, locals.var_pslk_dn0, locals.var_pslk_dn2, locals.var_pslk_dn4, locals.var_pslk_dn5, locals.var_pslk_dn6, locals.var_pslk_dn7, locals.var_pslk_dn8, locals.var_pslk_dn9, locals.var_pslk_dn10, locals.var_pslk_dn13,)
    }
};
        locals.var_pslk = assign95950_e148531;
        locals.var_pslk_dn0 = assign95950_e148531_d_n0;
        locals.var_pslk_dn2 = assign95950_e148531_d_n2;
        locals.var_pslk_dn4 = assign95950_e148531_d_n4;
        locals.var_pslk_dn5 = assign95950_e148531_d_n5;
        locals.var_pslk_dn6 = assign95950_e148531_d_n6;
        locals.var_pslk_dn7 = assign95950_e148531_d_n7;
        locals.var_pslk_dn8 = assign95950_e148531_d_n8;
        locals.var_pslk_dn9 = assign95950_e148531_d_n9;
        locals.var_pslk_dn10 = assign95950_e148531_d_n10;
        locals.var_pslk_dn13 = assign95950_e148531_d_n13;
        locals.var_pslk_rv = 0.0;

        let (assign95960_e148548, assign95960_e148548_d_n0, assign95960_e148548_d_n2, assign95960_e148548_d_n4, assign95960_e148548_d_n5, assign95960_e148548_d_n6, assign95960_e148548_d_n7, assign95960_e148548_d_n8, assign95960_e148548_d_n9, assign95960_e148548_d_n10, assign95960_e148548_d_n13,) = {
    if ((locals.var_guard2230 == 0.0) && (locals.var_guard2231 != 0.0)) {
        let assign95960_e148539: f64 = (locals.var_vdsz__blk439 + locals.var_ps0);
        let assign95960_e148540: f64 = (locals.var_aclm * assign95960_e148539);
        let assign95960_e148543: f64 = (1.0 - locals.var_aclm);
        let assign95960_e148545: f64 = (assign95960_e148543 * locals.var_pslk);
        let assign95960_e148546: f64 = (assign95960_e148540 + assign95960_e148545);
        (assign95960_e148546, ((locals.var_aclm * (locals.var_vdsz__blk439_dn0 + locals.var_ps0_dn0)) + (assign95960_e148543 * locals.var_pslk_dn0)), ((locals.var_aclm * (locals.var_vdsz__blk439_dn2 + locals.var_ps0_dn2)) + (assign95960_e148543 * locals.var_pslk_dn2)), ((locals.var_aclm * (locals.var_vdsz__blk439_dn4 + locals.var_ps0_dn4)) + (assign95960_e148543 * locals.var_pslk_dn4)), ((locals.var_aclm * (locals.var_vdsz__blk439_dn5 + locals.var_ps0_dn5)) + (assign95960_e148543 * locals.var_pslk_dn5)), ((locals.var_aclm * (locals.var_vdsz__blk439_dn6 + locals.var_ps0_dn6)) + (assign95960_e148543 * locals.var_pslk_dn6)), ((locals.var_aclm * (locals.var_vdsz__blk439_dn7 + locals.var_ps0_dn7)) + (assign95960_e148543 * locals.var_pslk_dn7)), ((locals.var_aclm * (locals.var_vdsz__blk439_dn8 + locals.var_ps0_dn8)) + (assign95960_e148543 * locals.var_pslk_dn8)), ((locals.var_aclm * (locals.var_vdsz__blk439_dn9 + locals.var_ps0_dn9)) + (assign95960_e148543 * locals.var_pslk_dn9)), ((locals.var_aclm * (locals.var_vdsz__blk439_dn10 + locals.var_ps0_dn10)) + (assign95960_e148543 * locals.var_pslk_dn10)), ((locals.var_aclm * (locals.var_vdsz__blk439_dn13 + locals.var_ps0_dn13)) + (assign95960_e148543 * locals.var_pslk_dn13)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign95960_e148548;
        locals.var_t1_dn0 = assign95960_e148548_d_n0;
        locals.var_t1_dn2 = assign95960_e148548_d_n2;
        locals.var_t1_dn4 = assign95960_e148548_d_n4;
        locals.var_t1_dn5 = assign95960_e148548_d_n5;
        locals.var_t1_dn6 = assign95960_e148548_d_n6;
        locals.var_t1_dn7 = assign95960_e148548_d_n7;
        locals.var_t1_dn8 = assign95960_e148548_d_n8;
        locals.var_t1_dn9 = assign95960_e148548_d_n9;
        locals.var_t1_dn10 = assign95960_e148548_d_n10;
        locals.var_t1_dn13 = assign95960_e148548_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign95970_e148564, assign95970_e148564_d_n0, assign95970_e148564_d_n2, assign95970_e148564_d_n4, assign95970_e148564_d_n5, assign95970_e148564_d_n6, assign95970_e148564_d_n7, assign95970_e148564_d_n8, assign95970_e148564_d_n9, assign95970_e148564_d_n10, assign95970_e148564_d_n13,) = {
    if ((locals.var_guard2230 == 0.0) && (locals.var_guard2231 != 0.0)) {
        let assign95970_e148555: f64 = (locals.var_ps0 + locals.var_vdsz__blk439);
        let assign95970_e148557: f64 = (assign95970_e148555 - locals.var_t1);
        let assign95970_e148559: f64 = (assign95970_e148557 / p.p133);
        let assign95970_e148560: f64 = (-assign95970_e148559);
        let assign95970_e148562: f64 = (assign95970_e148560 * locals.var_t2);
        (assign95970_e148562, (((-(((locals.var_ps0_dn0 + locals.var_vdsz__blk439_dn0) - locals.var_t1_dn0) / p.p133)) * locals.var_t2) + (assign95970_e148560 * locals.var_t2_dn0)), (((-(((locals.var_ps0_dn2 + locals.var_vdsz__blk439_dn2) - locals.var_t1_dn2) / p.p133)) * locals.var_t2) + (assign95970_e148560 * locals.var_t2_dn2)), (((-(((locals.var_ps0_dn4 + locals.var_vdsz__blk439_dn4) - locals.var_t1_dn4) / p.p133)) * locals.var_t2) + (assign95970_e148560 * locals.var_t2_dn4)), (((-(((locals.var_ps0_dn5 + locals.var_vdsz__blk439_dn5) - locals.var_t1_dn5) / p.p133)) * locals.var_t2) + (assign95970_e148560 * locals.var_t2_dn5)), (((-(((locals.var_ps0_dn6 + locals.var_vdsz__blk439_dn6) - locals.var_t1_dn6) / p.p133)) * locals.var_t2) + (assign95970_e148560 * locals.var_t2_dn6)), (((-(((locals.var_ps0_dn7 + locals.var_vdsz__blk439_dn7) - locals.var_t1_dn7) / p.p133)) * locals.var_t2) + (assign95970_e148560 * locals.var_t2_dn7)), (((-(((locals.var_ps0_dn8 + locals.var_vdsz__blk439_dn8) - locals.var_t1_dn8) / p.p133)) * locals.var_t2) + (assign95970_e148560 * locals.var_t2_dn8)), (((-(((locals.var_ps0_dn9 + locals.var_vdsz__blk439_dn9) - locals.var_t1_dn9) / p.p133)) * locals.var_t2) + (assign95970_e148560 * locals.var_t2_dn9)), (((-(((locals.var_ps0_dn10 + locals.var_vdsz__blk439_dn10) - locals.var_t1_dn10) / p.p133)) * locals.var_t2) + (assign95970_e148560 * locals.var_t2_dn10)), (((-(((locals.var_ps0_dn13 + locals.var_vdsz__blk439_dn13) - locals.var_t1_dn13) / p.p133)) * locals.var_t2) + (assign95970_e148560 * locals.var_t2_dn13)),)
    } else {
        (locals.var_qy, locals.var_qy_dn0, locals.var_qy_dn2, locals.var_qy_dn4, locals.var_qy_dn5, locals.var_qy_dn6, locals.var_qy_dn7, locals.var_qy_dn8, locals.var_qy_dn9, locals.var_qy_dn10, locals.var_qy_dn13,)
    }
};
        locals.var_qy = assign95970_e148564;
        locals.var_qy_dn0 = assign95970_e148564_d_n0;
        locals.var_qy_dn2 = assign95970_e148564_d_n2;
        locals.var_qy_dn4 = assign95970_e148564_d_n4;
        locals.var_qy_dn5 = assign95970_e148564_d_n5;
        locals.var_qy_dn6 = assign95970_e148564_d_n6;
        locals.var_qy_dn7 = assign95970_e148564_d_n7;
        locals.var_qy_dn8 = assign95970_e148564_d_n8;
        locals.var_qy_dn9 = assign95970_e148564_d_n9;
        locals.var_qy_dn10 = assign95970_e148564_d_n10;
        locals.var_qy_dn13 = assign95970_e148564_d_n13;
        locals.var_qy_rv = 0.0;

        let assign95980_e148567: f64 = if p.p134 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2232 = assign95980_e148567;
        locals.var_guard2232_rv = 0.0;

        let (assign95990_e148578, assign95990_e148578_d_n0, assign95990_e148578_d_n2, assign95990_e148578_d_n4, assign95990_e148578_d_n5, assign95990_e148578_d_n6, assign95990_e148578_d_n7, assign95990_e148578_d_n8, assign95990_e148578_d_n9, assign95990_e148578_d_n10, assign95990_e148578_d_n13,) = {
    if ((locals.var_guard2230 == 0.0) && (locals.var_guard2232 != 0.0)) {
        let assign95990_e148575: f64 = (locals.var_cqyb0 * locals.var_vbs);
        let assign95990_e148576: f64 = (locals.var_qy + assign95990_e148575);
        (assign95990_e148576, locals.var_qy_dn0, locals.var_qy_dn2, locals.var_qy_dn4, (locals.var_qy_dn5 + (locals.var_cqyb0 * locals.var_vbs_dn5)), locals.var_qy_dn6, (locals.var_qy_dn7 + (locals.var_cqyb0 * locals.var_vbs_dn7)), (locals.var_qy_dn8 + (locals.var_cqyb0 * locals.var_vbs_dn8)), locals.var_qy_dn9, locals.var_qy_dn10, locals.var_qy_dn13,)
    } else {
        (locals.var_qy, locals.var_qy_dn0, locals.var_qy_dn2, locals.var_qy_dn4, locals.var_qy_dn5, locals.var_qy_dn6, locals.var_qy_dn7, locals.var_qy_dn8, locals.var_qy_dn9, locals.var_qy_dn10, locals.var_qy_dn13,)
    }
};
        locals.var_qy = assign95990_e148578;
        locals.var_qy_dn0 = assign95990_e148578_d_n0;
        locals.var_qy_dn2 = assign95990_e148578_d_n2;
        locals.var_qy_dn4 = assign95990_e148578_d_n4;
        locals.var_qy_dn5 = assign95990_e148578_d_n5;
        locals.var_qy_dn6 = assign95990_e148578_d_n6;
        locals.var_qy_dn7 = assign95990_e148578_d_n7;
        locals.var_qy_dn8 = assign95990_e148578_d_n8;
        locals.var_qy_dn9 = assign95990_e148578_d_n9;
        locals.var_qy_dn10 = assign95990_e148578_d_n10;
        locals.var_qy_dn13 = assign95990_e148578_d_n13;
        locals.var_qy_rv = 0.0;

        locals.var_cfd = locals.var_cfrng;
        locals.var_cfd_rv = 0.0;

        locals.var_cfs = locals.var_cfrng;
        locals.var_cfs_rv = 0.0;

        let assign96020_e148584: f64 = (locals.var_vgsei - locals.var_vdsei);
        let assign96020_e148585: f64 = (locals.var_cfd * assign96020_e148584);
        locals.var_qfd = assign96020_e148585;
        locals.var_qfd_dn0 = (locals.var_cfd * (-locals.var_vdsei_dn0));
        locals.var_qfd_dn2 = (locals.var_cfd * (locals.var_vgsei_dn2 - locals.var_vdsei_dn2));
        locals.var_qfd_dn6 = (locals.var_cfd * locals.var_vgsei_dn6);
        locals.var_qfd_rv = 0.0;

        let assign96030_e148588: f64 = (locals.var_cfs * locals.var_vgsei);
        locals.var_qfs = assign96030_e148588;
        locals.var_qfs_dn2 = (locals.var_cfs * locals.var_vgsei_dn2);
        locals.var_qfs_dn6 = (locals.var_cfs * locals.var_vgsei_dn6);
        locals.var_qfs_rv = 0.0;

        let assign96040_e148595: f64 = if ((p.p53 > 0.0) && (locals.var_uc_rth0 != 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard2233 = assign96040_e148595;
        locals.var_guard2233_rv = 0.0;

        let (assign96050_e148601, assign96050_e148601_d_n0, assign96050_e148601_d_n2, assign96050_e148601_d_n4, assign96050_e148601_d_n5, assign96050_e148601_d_n6, assign96050_e148601_d_n7, assign96050_e148601_d_n8, assign96050_e148601_d_n9, assign96050_e148601_d_n10, assign96050_e148601_d_n13,) = {
    if (locals.var_guard2233 != 0.0) {
        let assign96050_e148599: f64 = (locals.var_tratio * locals.var_tratio);
        (assign96050_e148599, ((locals.var_tratio_dn0 * locals.var_tratio) + (locals.var_tratio * locals.var_tratio_dn0)), ((locals.var_tratio_dn2 * locals.var_tratio) + (locals.var_tratio * locals.var_tratio_dn2)), ((locals.var_tratio_dn4 * locals.var_tratio) + (locals.var_tratio * locals.var_tratio_dn4)), ((locals.var_tratio_dn5 * locals.var_tratio) + (locals.var_tratio * locals.var_tratio_dn5)), ((locals.var_tratio_dn6 * locals.var_tratio) + (locals.var_tratio * locals.var_tratio_dn6)), ((locals.var_tratio_dn7 * locals.var_tratio) + (locals.var_tratio * locals.var_tratio_dn7)), ((locals.var_tratio_dn8 * locals.var_tratio) + (locals.var_tratio * locals.var_tratio_dn8)), ((locals.var_tratio_dn9 * locals.var_tratio) + (locals.var_tratio * locals.var_tratio_dn9)), ((locals.var_tratio_dn10 * locals.var_tratio) + (locals.var_tratio * locals.var_tratio_dn10)), ((locals.var_tratio_dn13 * locals.var_tratio) + (locals.var_tratio * locals.var_tratio_dn13)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign96050_e148601;
        locals.var_t0_dn0 = assign96050_e148601_d_n0;
        locals.var_t0_dn2 = assign96050_e148601_d_n2;
        locals.var_t0_dn4 = assign96050_e148601_d_n4;
        locals.var_t0_dn5 = assign96050_e148601_d_n5;
        locals.var_t0_dn6 = assign96050_e148601_d_n6;
        locals.var_t0_dn7 = assign96050_e148601_d_n7;
        locals.var_t0_dn8 = assign96050_e148601_d_n8;
        locals.var_t0_dn9 = assign96050_e148601_d_n9;
        locals.var_t0_dn10 = assign96050_e148601_d_n10;
        locals.var_t0_dn13 = assign96050_e148601_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign96060_e148620, assign96060_e148620_d_n0, assign96060_e148620_d_n2, assign96060_e148620_d_n4, assign96060_e148620_d_n5, assign96060_e148620_d_n6, assign96060_e148620_d_n7, assign96060_e148620_d_n8, assign96060_e148620_d_n9, assign96060_e148620_d_n10, assign96060_e148620_d_n13,) = {
    if (locals.var_guard2233 != 0.0) {
        let assign96060_e148606: f64 = (locals.var_egtnom * locals.var_betatnom);
        let assign96060_e148609: f64 = (locals.var_eg * locals.var_beta);
        let assign96060_e148610: f64 = (assign96060_e148606 - assign96060_e148609);
        let assign96060_e148613: f64 = (p.p499 * locals.var_log_tratio);
        let assign96060_e148614: f64 = (assign96060_e148610 + assign96060_e148613);
        let assign96060_e148616: f64 = (assign96060_e148614 / locals.var_uc_njd);
        let assign96060_e148617: f64 = (assign96060_e148616).exp();
        let assign96060_e148618: f64 = (locals.var_uc_js0d * assign96060_e148617);
        (assign96060_e148618, (locals.var_uc_js0d * (assign96060_e148617 * (((-((locals.var_eg_dn0 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn0))) + (p.p499 * locals.var_log_tratio_dn0)) / locals.var_uc_njd))), (locals.var_uc_js0d * (assign96060_e148617 * (((-((locals.var_eg_dn2 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn2))) + (p.p499 * locals.var_log_tratio_dn2)) / locals.var_uc_njd))), (locals.var_uc_js0d * (assign96060_e148617 * (((-((locals.var_eg_dn4 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn4))) + (p.p499 * locals.var_log_tratio_dn4)) / locals.var_uc_njd))), (locals.var_uc_js0d * (assign96060_e148617 * (((-((locals.var_eg_dn5 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn5))) + (p.p499 * locals.var_log_tratio_dn5)) / locals.var_uc_njd))), (locals.var_uc_js0d * (assign96060_e148617 * (((-((locals.var_eg_dn6 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn6))) + (p.p499 * locals.var_log_tratio_dn6)) / locals.var_uc_njd))), (locals.var_uc_js0d * (assign96060_e148617 * (((-((locals.var_eg_dn7 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn7))) + (p.p499 * locals.var_log_tratio_dn7)) / locals.var_uc_njd))), (locals.var_uc_js0d * (assign96060_e148617 * (((-((locals.var_eg_dn8 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn8))) + (p.p499 * locals.var_log_tratio_dn8)) / locals.var_uc_njd))), (locals.var_uc_js0d * (assign96060_e148617 * (((-((locals.var_eg_dn9 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn9))) + (p.p499 * locals.var_log_tratio_dn9)) / locals.var_uc_njd))), (locals.var_uc_js0d * (assign96060_e148617 * (((-((locals.var_eg_dn10 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn10))) + (p.p499 * locals.var_log_tratio_dn10)) / locals.var_uc_njd))), (locals.var_uc_js0d * (assign96060_e148617 * (((-((locals.var_eg_dn13 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn13))) + (p.p499 * locals.var_log_tratio_dn13)) / locals.var_uc_njd))),)
    } else {
        (locals.var_js, locals.var_js_dn0, locals.var_js_dn2, locals.var_js_dn4, locals.var_js_dn5, locals.var_js_dn6, locals.var_js_dn7, locals.var_js_dn8, locals.var_js_dn9, locals.var_js_dn10, locals.var_js_dn13,)
    }
};
        locals.var_js = assign96060_e148620;
        locals.var_js_dn0 = assign96060_e148620_d_n0;
        locals.var_js_dn2 = assign96060_e148620_d_n2;
        locals.var_js_dn4 = assign96060_e148620_d_n4;
        locals.var_js_dn5 = assign96060_e148620_d_n5;
        locals.var_js_dn6 = assign96060_e148620_d_n6;
        locals.var_js_dn7 = assign96060_e148620_d_n7;
        locals.var_js_dn8 = assign96060_e148620_d_n8;
        locals.var_js_dn9 = assign96060_e148620_d_n9;
        locals.var_js_dn10 = assign96060_e148620_d_n10;
        locals.var_js_dn13 = assign96060_e148620_d_n13;
        locals.var_js_rv = 0.0;

        let (assign96070_e148639, assign96070_e148639_d_n0, assign96070_e148639_d_n2, assign96070_e148639_d_n4, assign96070_e148639_d_n5, assign96070_e148639_d_n6, assign96070_e148639_d_n7, assign96070_e148639_d_n8, assign96070_e148639_d_n9, assign96070_e148639_d_n10, assign96070_e148639_d_n13,) = {
    if (locals.var_guard2233 != 0.0) {
        let assign96070_e148625: f64 = (locals.var_egtnom * locals.var_betatnom);
        let assign96070_e148628: f64 = (locals.var_eg * locals.var_beta);
        let assign96070_e148629: f64 = (assign96070_e148625 - assign96070_e148628);
        let assign96070_e148632: f64 = (p.p499 * locals.var_log_tratio);
        let assign96070_e148633: f64 = (assign96070_e148629 + assign96070_e148632);
        let assign96070_e148635: f64 = (assign96070_e148633 / p.p497);
        let assign96070_e148636: f64 = (assign96070_e148635).exp();
        let assign96070_e148637: f64 = (locals.var_uc_js0swd * assign96070_e148636);
        (assign96070_e148637, (locals.var_uc_js0swd * (assign96070_e148636 * (((-((locals.var_eg_dn0 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn0))) + (p.p499 * locals.var_log_tratio_dn0)) / p.p497))), (locals.var_uc_js0swd * (assign96070_e148636 * (((-((locals.var_eg_dn2 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn2))) + (p.p499 * locals.var_log_tratio_dn2)) / p.p497))), (locals.var_uc_js0swd * (assign96070_e148636 * (((-((locals.var_eg_dn4 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn4))) + (p.p499 * locals.var_log_tratio_dn4)) / p.p497))), (locals.var_uc_js0swd * (assign96070_e148636 * (((-((locals.var_eg_dn5 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn5))) + (p.p499 * locals.var_log_tratio_dn5)) / p.p497))), (locals.var_uc_js0swd * (assign96070_e148636 * (((-((locals.var_eg_dn6 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn6))) + (p.p499 * locals.var_log_tratio_dn6)) / p.p497))), (locals.var_uc_js0swd * (assign96070_e148636 * (((-((locals.var_eg_dn7 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn7))) + (p.p499 * locals.var_log_tratio_dn7)) / p.p497))), (locals.var_uc_js0swd * (assign96070_e148636 * (((-((locals.var_eg_dn8 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn8))) + (p.p499 * locals.var_log_tratio_dn8)) / p.p497))), (locals.var_uc_js0swd * (assign96070_e148636 * (((-((locals.var_eg_dn9 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn9))) + (p.p499 * locals.var_log_tratio_dn9)) / p.p497))), (locals.var_uc_js0swd * (assign96070_e148636 * (((-((locals.var_eg_dn10 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn10))) + (p.p499 * locals.var_log_tratio_dn10)) / p.p497))), (locals.var_uc_js0swd * (assign96070_e148636 * (((-((locals.var_eg_dn13 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn13))) + (p.p499 * locals.var_log_tratio_dn13)) / p.p497))),)
    } else {
        (locals.var_jssw, locals.var_jssw_dn0, locals.var_jssw_dn2, locals.var_jssw_dn4, locals.var_jssw_dn5, locals.var_jssw_dn6, locals.var_jssw_dn7, locals.var_jssw_dn8, locals.var_jssw_dn9, locals.var_jssw_dn10, locals.var_jssw_dn13,)
    }
};
        locals.var_jssw = assign96070_e148639;
        locals.var_jssw_dn0 = assign96070_e148639_d_n0;
        locals.var_jssw_dn2 = assign96070_e148639_d_n2;
        locals.var_jssw_dn4 = assign96070_e148639_d_n4;
        locals.var_jssw_dn5 = assign96070_e148639_d_n5;
        locals.var_jssw_dn6 = assign96070_e148639_d_n6;
        locals.var_jssw_dn7 = assign96070_e148639_d_n7;
        locals.var_jssw_dn8 = assign96070_e148639_d_n8;
        locals.var_jssw_dn9 = assign96070_e148639_d_n9;
        locals.var_jssw_dn10 = assign96070_e148639_d_n10;
        locals.var_jssw_dn13 = assign96070_e148639_d_n13;
        locals.var_jssw_rv = 0.0;

        let (assign96080_e148658, assign96080_e148658_d_n0, assign96080_e148658_d_n2, assign96080_e148658_d_n4, assign96080_e148658_d_n5, assign96080_e148658_d_n6, assign96080_e148658_d_n7, assign96080_e148658_d_n8, assign96080_e148658_d_n9, assign96080_e148658_d_n10, assign96080_e148658_d_n13,) = {
    if (locals.var_guard2233 != 0.0) {
        let assign96080_e148644: f64 = (locals.var_egtnom * locals.var_betatnom);
        let assign96080_e148647: f64 = (locals.var_eg * locals.var_beta);
        let assign96080_e148648: f64 = (assign96080_e148644 - assign96080_e148647);
        let assign96080_e148651: f64 = (p.p499 * locals.var_log_tratio);
        let assign96080_e148652: f64 = (assign96080_e148648 + assign96080_e148651);
        let assign96080_e148654: f64 = (assign96080_e148652 / p.p498);
        let assign96080_e148655: f64 = (assign96080_e148654).exp();
        let assign96080_e148656: f64 = (p.p495 * assign96080_e148655);
        (assign96080_e148656, (p.p495 * (assign96080_e148655 * (((-((locals.var_eg_dn0 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn0))) + (p.p499 * locals.var_log_tratio_dn0)) / p.p498))), (p.p495 * (assign96080_e148655 * (((-((locals.var_eg_dn2 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn2))) + (p.p499 * locals.var_log_tratio_dn2)) / p.p498))), (p.p495 * (assign96080_e148655 * (((-((locals.var_eg_dn4 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn4))) + (p.p499 * locals.var_log_tratio_dn4)) / p.p498))), (p.p495 * (assign96080_e148655 * (((-((locals.var_eg_dn5 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn5))) + (p.p499 * locals.var_log_tratio_dn5)) / p.p498))), (p.p495 * (assign96080_e148655 * (((-((locals.var_eg_dn6 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn6))) + (p.p499 * locals.var_log_tratio_dn6)) / p.p498))), (p.p495 * (assign96080_e148655 * (((-((locals.var_eg_dn7 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn7))) + (p.p499 * locals.var_log_tratio_dn7)) / p.p498))), (p.p495 * (assign96080_e148655 * (((-((locals.var_eg_dn8 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn8))) + (p.p499 * locals.var_log_tratio_dn8)) / p.p498))), (p.p495 * (assign96080_e148655 * (((-((locals.var_eg_dn9 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn9))) + (p.p499 * locals.var_log_tratio_dn9)) / p.p498))), (p.p495 * (assign96080_e148655 * (((-((locals.var_eg_dn10 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn10))) + (p.p499 * locals.var_log_tratio_dn10)) / p.p498))), (p.p495 * (assign96080_e148655 * (((-((locals.var_eg_dn13 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn13))) + (p.p499 * locals.var_log_tratio_dn13)) / p.p498))),)
    } else {
        (locals.var_jsswg, locals.var_jsswg_dn0, locals.var_jsswg_dn2, locals.var_jsswg_dn4, locals.var_jsswg_dn5, locals.var_jsswg_dn6, locals.var_jsswg_dn7, locals.var_jsswg_dn8, locals.var_jsswg_dn9, locals.var_jsswg_dn10, locals.var_jsswg_dn13,)
    }
};
        locals.var_jsswg = assign96080_e148658;
        locals.var_jsswg_dn0 = assign96080_e148658_d_n0;
        locals.var_jsswg_dn2 = assign96080_e148658_d_n2;
        locals.var_jsswg_dn4 = assign96080_e148658_d_n4;
        locals.var_jsswg_dn5 = assign96080_e148658_d_n5;
        locals.var_jsswg_dn6 = assign96080_e148658_d_n6;
        locals.var_jsswg_dn7 = assign96080_e148658_d_n7;
        locals.var_jsswg_dn8 = assign96080_e148658_d_n8;
        locals.var_jsswg_dn9 = assign96080_e148658_d_n9;
        locals.var_jsswg_dn10 = assign96080_e148658_d_n10;
        locals.var_jsswg_dn13 = assign96080_e148658_d_n13;
        locals.var_jsswg_rv = 0.0;

        let (assign96090_e148677, assign96090_e148677_d_n0, assign96090_e148677_d_n2, assign96090_e148677_d_n4, assign96090_e148677_d_n5, assign96090_e148677_d_n6, assign96090_e148677_d_n7, assign96090_e148677_d_n8, assign96090_e148677_d_n9, assign96090_e148677_d_n10, assign96090_e148677_d_n13,) = {
    if (locals.var_guard2233 != 0.0) {
        let assign96090_e148663: f64 = (locals.var_egtnom * locals.var_betatnom);
        let assign96090_e148666: f64 = (locals.var_eg * locals.var_beta);
        let assign96090_e148667: f64 = (assign96090_e148663 - assign96090_e148666);
        let assign96090_e148670: f64 = (p.p509 * locals.var_log_tratio);
        let assign96090_e148671: f64 = (assign96090_e148667 + assign96090_e148670);
        let assign96090_e148673: f64 = (assign96090_e148671 / locals.var_uc_njd);
        let assign96090_e148674: f64 = (assign96090_e148673).exp();
        let assign96090_e148675: f64 = (locals.var_uc_js0d * assign96090_e148674);
        (assign96090_e148675, (locals.var_uc_js0d * (assign96090_e148674 * (((-((locals.var_eg_dn0 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn0))) + (p.p509 * locals.var_log_tratio_dn0)) / locals.var_uc_njd))), (locals.var_uc_js0d * (assign96090_e148674 * (((-((locals.var_eg_dn2 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn2))) + (p.p509 * locals.var_log_tratio_dn2)) / locals.var_uc_njd))), (locals.var_uc_js0d * (assign96090_e148674 * (((-((locals.var_eg_dn4 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn4))) + (p.p509 * locals.var_log_tratio_dn4)) / locals.var_uc_njd))), (locals.var_uc_js0d * (assign96090_e148674 * (((-((locals.var_eg_dn5 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn5))) + (p.p509 * locals.var_log_tratio_dn5)) / locals.var_uc_njd))), (locals.var_uc_js0d * (assign96090_e148674 * (((-((locals.var_eg_dn6 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn6))) + (p.p509 * locals.var_log_tratio_dn6)) / locals.var_uc_njd))), (locals.var_uc_js0d * (assign96090_e148674 * (((-((locals.var_eg_dn7 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn7))) + (p.p509 * locals.var_log_tratio_dn7)) / locals.var_uc_njd))), (locals.var_uc_js0d * (assign96090_e148674 * (((-((locals.var_eg_dn8 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn8))) + (p.p509 * locals.var_log_tratio_dn8)) / locals.var_uc_njd))), (locals.var_uc_js0d * (assign96090_e148674 * (((-((locals.var_eg_dn9 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn9))) + (p.p509 * locals.var_log_tratio_dn9)) / locals.var_uc_njd))), (locals.var_uc_js0d * (assign96090_e148674 * (((-((locals.var_eg_dn10 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn10))) + (p.p509 * locals.var_log_tratio_dn10)) / locals.var_uc_njd))), (locals.var_uc_js0d * (assign96090_e148674 * (((-((locals.var_eg_dn13 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn13))) + (p.p509 * locals.var_log_tratio_dn13)) / locals.var_uc_njd))),)
    } else {
        (locals.var_js2, locals.var_js2_dn0, locals.var_js2_dn2, locals.var_js2_dn4, locals.var_js2_dn5, locals.var_js2_dn6, locals.var_js2_dn7, locals.var_js2_dn8, locals.var_js2_dn9, locals.var_js2_dn10, locals.var_js2_dn13,)
    }
};
        locals.var_js2 = assign96090_e148677;
        locals.var_js2_dn0 = assign96090_e148677_d_n0;
        locals.var_js2_dn2 = assign96090_e148677_d_n2;
        locals.var_js2_dn4 = assign96090_e148677_d_n4;
        locals.var_js2_dn5 = assign96090_e148677_d_n5;
        locals.var_js2_dn6 = assign96090_e148677_d_n6;
        locals.var_js2_dn7 = assign96090_e148677_d_n7;
        locals.var_js2_dn8 = assign96090_e148677_d_n8;
        locals.var_js2_dn9 = assign96090_e148677_d_n9;
        locals.var_js2_dn10 = assign96090_e148677_d_n10;
        locals.var_js2_dn13 = assign96090_e148677_d_n13;
        locals.var_js2_rv = 0.0;

        let (assign96100_e148696, assign96100_e148696_d_n0, assign96100_e148696_d_n2, assign96100_e148696_d_n4, assign96100_e148696_d_n5, assign96100_e148696_d_n6, assign96100_e148696_d_n7, assign96100_e148696_d_n8, assign96100_e148696_d_n9, assign96100_e148696_d_n10, assign96100_e148696_d_n13,) = {
    if (locals.var_guard2233 != 0.0) {
        let assign96100_e148682: f64 = (locals.var_egtnom * locals.var_betatnom);
        let assign96100_e148685: f64 = (locals.var_eg * locals.var_beta);
        let assign96100_e148686: f64 = (assign96100_e148682 - assign96100_e148685);
        let assign96100_e148689: f64 = (p.p509 * locals.var_log_tratio);
        let assign96100_e148690: f64 = (assign96100_e148686 + assign96100_e148689);
        let assign96100_e148692: f64 = (assign96100_e148690 / p.p497);
        let assign96100_e148693: f64 = (assign96100_e148692).exp();
        let assign96100_e148694: f64 = (locals.var_uc_js0swd * assign96100_e148693);
        (assign96100_e148694, (locals.var_uc_js0swd * (assign96100_e148693 * (((-((locals.var_eg_dn0 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn0))) + (p.p509 * locals.var_log_tratio_dn0)) / p.p497))), (locals.var_uc_js0swd * (assign96100_e148693 * (((-((locals.var_eg_dn2 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn2))) + (p.p509 * locals.var_log_tratio_dn2)) / p.p497))), (locals.var_uc_js0swd * (assign96100_e148693 * (((-((locals.var_eg_dn4 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn4))) + (p.p509 * locals.var_log_tratio_dn4)) / p.p497))), (locals.var_uc_js0swd * (assign96100_e148693 * (((-((locals.var_eg_dn5 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn5))) + (p.p509 * locals.var_log_tratio_dn5)) / p.p497))), (locals.var_uc_js0swd * (assign96100_e148693 * (((-((locals.var_eg_dn6 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn6))) + (p.p509 * locals.var_log_tratio_dn6)) / p.p497))), (locals.var_uc_js0swd * (assign96100_e148693 * (((-((locals.var_eg_dn7 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn7))) + (p.p509 * locals.var_log_tratio_dn7)) / p.p497))), (locals.var_uc_js0swd * (assign96100_e148693 * (((-((locals.var_eg_dn8 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn8))) + (p.p509 * locals.var_log_tratio_dn8)) / p.p497))), (locals.var_uc_js0swd * (assign96100_e148693 * (((-((locals.var_eg_dn9 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn9))) + (p.p509 * locals.var_log_tratio_dn9)) / p.p497))), (locals.var_uc_js0swd * (assign96100_e148693 * (((-((locals.var_eg_dn10 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn10))) + (p.p509 * locals.var_log_tratio_dn10)) / p.p497))), (locals.var_uc_js0swd * (assign96100_e148693 * (((-((locals.var_eg_dn13 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn13))) + (p.p509 * locals.var_log_tratio_dn13)) / p.p497))),)
    } else {
        (locals.var_jssw2, locals.var_jssw2_dn0, locals.var_jssw2_dn2, locals.var_jssw2_dn4, locals.var_jssw2_dn5, locals.var_jssw2_dn6, locals.var_jssw2_dn7, locals.var_jssw2_dn8, locals.var_jssw2_dn9, locals.var_jssw2_dn10, locals.var_jssw2_dn13,)
    }
};
        locals.var_jssw2 = assign96100_e148696;
        locals.var_jssw2_dn0 = assign96100_e148696_d_n0;
        locals.var_jssw2_dn2 = assign96100_e148696_d_n2;
        locals.var_jssw2_dn4 = assign96100_e148696_d_n4;
        locals.var_jssw2_dn5 = assign96100_e148696_d_n5;
        locals.var_jssw2_dn6 = assign96100_e148696_d_n6;
        locals.var_jssw2_dn7 = assign96100_e148696_d_n7;
        locals.var_jssw2_dn8 = assign96100_e148696_d_n8;
        locals.var_jssw2_dn9 = assign96100_e148696_d_n9;
        locals.var_jssw2_dn10 = assign96100_e148696_d_n10;
        locals.var_jssw2_dn13 = assign96100_e148696_d_n13;
        locals.var_jssw2_rv = 0.0;

        let (assign96110_e148715, assign96110_e148715_d_n0, assign96110_e148715_d_n2, assign96110_e148715_d_n4, assign96110_e148715_d_n5, assign96110_e148715_d_n6, assign96110_e148715_d_n7, assign96110_e148715_d_n8, assign96110_e148715_d_n9, assign96110_e148715_d_n10, assign96110_e148715_d_n13,) = {
    if (locals.var_guard2233 != 0.0) {
        let assign96110_e148701: f64 = (locals.var_egtnom * locals.var_betatnom);
        let assign96110_e148704: f64 = (locals.var_eg * locals.var_beta);
        let assign96110_e148705: f64 = (assign96110_e148701 - assign96110_e148704);
        let assign96110_e148708: f64 = (p.p509 * locals.var_log_tratio);
        let assign96110_e148709: f64 = (assign96110_e148705 + assign96110_e148708);
        let assign96110_e148711: f64 = (assign96110_e148709 / p.p498);
        let assign96110_e148712: f64 = (assign96110_e148711).exp();
        let assign96110_e148713: f64 = (p.p495 * assign96110_e148712);
        (assign96110_e148713, (p.p495 * (assign96110_e148712 * (((-((locals.var_eg_dn0 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn0))) + (p.p509 * locals.var_log_tratio_dn0)) / p.p498))), (p.p495 * (assign96110_e148712 * (((-((locals.var_eg_dn2 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn2))) + (p.p509 * locals.var_log_tratio_dn2)) / p.p498))), (p.p495 * (assign96110_e148712 * (((-((locals.var_eg_dn4 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn4))) + (p.p509 * locals.var_log_tratio_dn4)) / p.p498))), (p.p495 * (assign96110_e148712 * (((-((locals.var_eg_dn5 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn5))) + (p.p509 * locals.var_log_tratio_dn5)) / p.p498))), (p.p495 * (assign96110_e148712 * (((-((locals.var_eg_dn6 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn6))) + (p.p509 * locals.var_log_tratio_dn6)) / p.p498))), (p.p495 * (assign96110_e148712 * (((-((locals.var_eg_dn7 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn7))) + (p.p509 * locals.var_log_tratio_dn7)) / p.p498))), (p.p495 * (assign96110_e148712 * (((-((locals.var_eg_dn8 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn8))) + (p.p509 * locals.var_log_tratio_dn8)) / p.p498))), (p.p495 * (assign96110_e148712 * (((-((locals.var_eg_dn9 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn9))) + (p.p509 * locals.var_log_tratio_dn9)) / p.p498))), (p.p495 * (assign96110_e148712 * (((-((locals.var_eg_dn10 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn10))) + (p.p509 * locals.var_log_tratio_dn10)) / p.p498))), (p.p495 * (assign96110_e148712 * (((-((locals.var_eg_dn13 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn13))) + (p.p509 * locals.var_log_tratio_dn13)) / p.p498))),)
    } else {
        (locals.var_jsswg2, locals.var_jsswg2_dn0, locals.var_jsswg2_dn2, locals.var_jsswg2_dn4, locals.var_jsswg2_dn5, locals.var_jsswg2_dn6, locals.var_jsswg2_dn7, locals.var_jsswg2_dn8, locals.var_jsswg2_dn9, locals.var_jsswg2_dn10, locals.var_jsswg2_dn13,)
    }
};
        locals.var_jsswg2 = assign96110_e148715;
        locals.var_jsswg2_dn0 = assign96110_e148715_d_n0;
        locals.var_jsswg2_dn2 = assign96110_e148715_d_n2;
        locals.var_jsswg2_dn4 = assign96110_e148715_d_n4;
        locals.var_jsswg2_dn5 = assign96110_e148715_d_n5;
        locals.var_jsswg2_dn6 = assign96110_e148715_d_n6;
        locals.var_jsswg2_dn7 = assign96110_e148715_d_n7;
        locals.var_jsswg2_dn8 = assign96110_e148715_d_n8;
        locals.var_jsswg2_dn9 = assign96110_e148715_d_n9;
        locals.var_jsswg2_dn10 = assign96110_e148715_d_n10;
        locals.var_jsswg2_dn13 = assign96110_e148715_d_n13;
        locals.var_jsswg2_rv = 0.0;

        let assign96120_e148718: f64 = if p.p48 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2234 = assign96120_e148718;
        locals.var_guard2234_rv = 0.0;

        let assign96130_e148721: f64 = if p.p15 > locals.var_weff_nf { 1.0 } else { 0.0 };
        locals.var_guard2235 = assign96130_e148721;
        locals.var_guard2235_rv = 0.0;

        let (assign96140_e148731, assign96140_e148731_d_n0, assign96140_e148731_d_n2, assign96140_e148731_d_n4, assign96140_e148731_d_n5, assign96140_e148731_d_n6, assign96140_e148731_d_n7, assign96140_e148731_d_n8, assign96140_e148731_d_n9, assign96140_e148731_d_n10, assign96140_e148731_d_n13,) = {
    if (((locals.var_guard2233 != 0.0) && (locals.var_guard2234 != 0.0)) && (locals.var_guard2235 != 0.0)) {
        let assign96140_e148729: f64 = (p.p13 * locals.var_js);
        (assign96140_e148729, (p.p13 * locals.var_js_dn0), (p.p13 * locals.var_js_dn2), (p.p13 * locals.var_js_dn4), (p.p13 * locals.var_js_dn5), (p.p13 * locals.var_js_dn6), (p.p13 * locals.var_js_dn7), (p.p13 * locals.var_js_dn8), (p.p13 * locals.var_js_dn9), (p.p13 * locals.var_js_dn10), (p.p13 * locals.var_js_dn13),)
    } else {
        (locals.var_isbd_btm, locals.var_isbd_btm_dn0, locals.var_isbd_btm_dn2, locals.var_isbd_btm_dn4, locals.var_isbd_btm_dn5, locals.var_isbd_btm_dn6, locals.var_isbd_btm_dn7, locals.var_isbd_btm_dn8, locals.var_isbd_btm_dn9, locals.var_isbd_btm_dn10, locals.var_isbd_btm_dn13,)
    }
};
        locals.var_isbd_btm = assign96140_e148731;
        locals.var_isbd_btm_dn0 = assign96140_e148731_d_n0;
        locals.var_isbd_btm_dn2 = assign96140_e148731_d_n2;
        locals.var_isbd_btm_dn4 = assign96140_e148731_d_n4;
        locals.var_isbd_btm_dn5 = assign96140_e148731_d_n5;
        locals.var_isbd_btm_dn6 = assign96140_e148731_d_n6;
        locals.var_isbd_btm_dn7 = assign96140_e148731_d_n7;
        locals.var_isbd_btm_dn8 = assign96140_e148731_d_n8;
        locals.var_isbd_btm_dn9 = assign96140_e148731_d_n9;
        locals.var_isbd_btm_dn10 = assign96140_e148731_d_n10;
        locals.var_isbd_btm_dn13 = assign96140_e148731_d_n13;
        locals.var_isbd_btm_rv = 0.0;

        let (assign96150_e148741, assign96150_e148741_d_n0, assign96150_e148741_d_n2, assign96150_e148741_d_n4, assign96150_e148741_d_n5, assign96150_e148741_d_n6, assign96150_e148741_d_n7, assign96150_e148741_d_n8, assign96150_e148741_d_n9, assign96150_e148741_d_n10, assign96150_e148741_d_n13,) = {
    if (((locals.var_guard2233 != 0.0) && (locals.var_guard2234 != 0.0)) && (locals.var_guard2235 != 0.0)) {
        let assign96150_e148739: f64 = (p.p13 * locals.var_js2);
        (assign96150_e148739, (p.p13 * locals.var_js2_dn0), (p.p13 * locals.var_js2_dn2), (p.p13 * locals.var_js2_dn4), (p.p13 * locals.var_js2_dn5), (p.p13 * locals.var_js2_dn6), (p.p13 * locals.var_js2_dn7), (p.p13 * locals.var_js2_dn8), (p.p13 * locals.var_js2_dn9), (p.p13 * locals.var_js2_dn10), (p.p13 * locals.var_js2_dn13),)
    } else {
        (locals.var_isbd2_btm, locals.var_isbd2_btm_dn0, locals.var_isbd2_btm_dn2, locals.var_isbd2_btm_dn4, locals.var_isbd2_btm_dn5, locals.var_isbd2_btm_dn6, locals.var_isbd2_btm_dn7, locals.var_isbd2_btm_dn8, locals.var_isbd2_btm_dn9, locals.var_isbd2_btm_dn10, locals.var_isbd2_btm_dn13,)
    }
};
        locals.var_isbd2_btm = assign96150_e148741;
        locals.var_isbd2_btm_dn0 = assign96150_e148741_d_n0;
        locals.var_isbd2_btm_dn2 = assign96150_e148741_d_n2;
        locals.var_isbd2_btm_dn4 = assign96150_e148741_d_n4;
        locals.var_isbd2_btm_dn5 = assign96150_e148741_d_n5;
        locals.var_isbd2_btm_dn6 = assign96150_e148741_d_n6;
        locals.var_isbd2_btm_dn7 = assign96150_e148741_d_n7;
        locals.var_isbd2_btm_dn8 = assign96150_e148741_d_n8;
        locals.var_isbd2_btm_dn9 = assign96150_e148741_d_n9;
        locals.var_isbd2_btm_dn10 = assign96150_e148741_d_n10;
        locals.var_isbd2_btm_dn13 = assign96150_e148741_d_n13;
        locals.var_isbd2_btm_rv = 0.0;

        let (assign96160_e148753, assign96160_e148753_d_n0, assign96160_e148753_d_n2, assign96160_e148753_d_n4, assign96160_e148753_d_n5, assign96160_e148753_d_n6, assign96160_e148753_d_n7, assign96160_e148753_d_n8, assign96160_e148753_d_n9, assign96160_e148753_d_n10, assign96160_e148753_d_n13,) = {
    if (((locals.var_guard2233 != 0.0) && (locals.var_guard2234 != 0.0)) && (locals.var_guard2235 != 0.0)) {
        let assign96160_e148749: f64 = (p.p15 - locals.var_weff_nf);
        let assign96160_e148751: f64 = (assign96160_e148749 * locals.var_jssw);
        (assign96160_e148751, (assign96160_e148749 * locals.var_jssw_dn0), (assign96160_e148749 * locals.var_jssw_dn2), (assign96160_e148749 * locals.var_jssw_dn4), (assign96160_e148749 * locals.var_jssw_dn5), (assign96160_e148749 * locals.var_jssw_dn6), (assign96160_e148749 * locals.var_jssw_dn7), (assign96160_e148749 * locals.var_jssw_dn8), (assign96160_e148749 * locals.var_jssw_dn9), (assign96160_e148749 * locals.var_jssw_dn10), (assign96160_e148749 * locals.var_jssw_dn13),)
    } else {
        (locals.var_isbd_sws, locals.var_isbd_sws_dn0, locals.var_isbd_sws_dn2, locals.var_isbd_sws_dn4, locals.var_isbd_sws_dn5, locals.var_isbd_sws_dn6, locals.var_isbd_sws_dn7, locals.var_isbd_sws_dn8, locals.var_isbd_sws_dn9, locals.var_isbd_sws_dn10, locals.var_isbd_sws_dn13,)
    }
};
        locals.var_isbd_sws = assign96160_e148753;
        locals.var_isbd_sws_dn0 = assign96160_e148753_d_n0;
        locals.var_isbd_sws_dn2 = assign96160_e148753_d_n2;
        locals.var_isbd_sws_dn4 = assign96160_e148753_d_n4;
        locals.var_isbd_sws_dn5 = assign96160_e148753_d_n5;
        locals.var_isbd_sws_dn6 = assign96160_e148753_d_n6;
        locals.var_isbd_sws_dn7 = assign96160_e148753_d_n7;
        locals.var_isbd_sws_dn8 = assign96160_e148753_d_n8;
        locals.var_isbd_sws_dn9 = assign96160_e148753_d_n9;
        locals.var_isbd_sws_dn10 = assign96160_e148753_d_n10;
        locals.var_isbd_sws_dn13 = assign96160_e148753_d_n13;
        locals.var_isbd_sws_rv = 0.0;

        let (assign96170_e148765, assign96170_e148765_d_n0, assign96170_e148765_d_n2, assign96170_e148765_d_n4, assign96170_e148765_d_n5, assign96170_e148765_d_n6, assign96170_e148765_d_n7, assign96170_e148765_d_n8, assign96170_e148765_d_n9, assign96170_e148765_d_n10, assign96170_e148765_d_n13,) = {
    if (((locals.var_guard2233 != 0.0) && (locals.var_guard2234 != 0.0)) && (locals.var_guard2235 != 0.0)) {
        let assign96170_e148761: f64 = (p.p15 - locals.var_weff_nf);
        let assign96170_e148763: f64 = (assign96170_e148761 * locals.var_jssw2);
        (assign96170_e148763, (assign96170_e148761 * locals.var_jssw2_dn0), (assign96170_e148761 * locals.var_jssw2_dn2), (assign96170_e148761 * locals.var_jssw2_dn4), (assign96170_e148761 * locals.var_jssw2_dn5), (assign96170_e148761 * locals.var_jssw2_dn6), (assign96170_e148761 * locals.var_jssw2_dn7), (assign96170_e148761 * locals.var_jssw2_dn8), (assign96170_e148761 * locals.var_jssw2_dn9), (assign96170_e148761 * locals.var_jssw2_dn10), (assign96170_e148761 * locals.var_jssw2_dn13),)
    } else {
        (locals.var_isbd2_sws, locals.var_isbd2_sws_dn0, locals.var_isbd2_sws_dn2, locals.var_isbd2_sws_dn4, locals.var_isbd2_sws_dn5, locals.var_isbd2_sws_dn6, locals.var_isbd2_sws_dn7, locals.var_isbd2_sws_dn8, locals.var_isbd2_sws_dn9, locals.var_isbd2_sws_dn10, locals.var_isbd2_sws_dn13,)
    }
};
        locals.var_isbd2_sws = assign96170_e148765;
        locals.var_isbd2_sws_dn0 = assign96170_e148765_d_n0;
        locals.var_isbd2_sws_dn2 = assign96170_e148765_d_n2;
        locals.var_isbd2_sws_dn4 = assign96170_e148765_d_n4;
        locals.var_isbd2_sws_dn5 = assign96170_e148765_d_n5;
        locals.var_isbd2_sws_dn6 = assign96170_e148765_d_n6;
        locals.var_isbd2_sws_dn7 = assign96170_e148765_d_n7;
        locals.var_isbd2_sws_dn8 = assign96170_e148765_d_n8;
        locals.var_isbd2_sws_dn9 = assign96170_e148765_d_n9;
        locals.var_isbd2_sws_dn10 = assign96170_e148765_d_n10;
        locals.var_isbd2_sws_dn13 = assign96170_e148765_d_n13;
        locals.var_isbd2_sws_rv = 0.0;

        let (assign96180_e148775, assign96180_e148775_d_n0, assign96180_e148775_d_n2, assign96180_e148775_d_n4, assign96180_e148775_d_n5, assign96180_e148775_d_n6, assign96180_e148775_d_n7, assign96180_e148775_d_n8, assign96180_e148775_d_n9, assign96180_e148775_d_n10, assign96180_e148775_d_n13,) = {
    if (((locals.var_guard2233 != 0.0) && (locals.var_guard2234 != 0.0)) && (locals.var_guard2235 != 0.0)) {
        let assign96180_e148773: f64 = (locals.var_weff_nf * locals.var_jsswg);
        (assign96180_e148773, (locals.var_weff_nf * locals.var_jsswg_dn0), (locals.var_weff_nf * locals.var_jsswg_dn2), (locals.var_weff_nf * locals.var_jsswg_dn4), (locals.var_weff_nf * locals.var_jsswg_dn5), (locals.var_weff_nf * locals.var_jsswg_dn6), (locals.var_weff_nf * locals.var_jsswg_dn7), (locals.var_weff_nf * locals.var_jsswg_dn8), (locals.var_weff_nf * locals.var_jsswg_dn9), (locals.var_weff_nf * locals.var_jsswg_dn10), (locals.var_weff_nf * locals.var_jsswg_dn13),)
    } else {
        (locals.var_isbd_swg, locals.var_isbd_swg_dn0, locals.var_isbd_swg_dn2, locals.var_isbd_swg_dn4, locals.var_isbd_swg_dn5, locals.var_isbd_swg_dn6, locals.var_isbd_swg_dn7, locals.var_isbd_swg_dn8, locals.var_isbd_swg_dn9, locals.var_isbd_swg_dn10, locals.var_isbd_swg_dn13,)
    }
};
        locals.var_isbd_swg = assign96180_e148775;
        locals.var_isbd_swg_dn0 = assign96180_e148775_d_n0;
        locals.var_isbd_swg_dn2 = assign96180_e148775_d_n2;
        locals.var_isbd_swg_dn4 = assign96180_e148775_d_n4;
        locals.var_isbd_swg_dn5 = assign96180_e148775_d_n5;
        locals.var_isbd_swg_dn6 = assign96180_e148775_d_n6;
        locals.var_isbd_swg_dn7 = assign96180_e148775_d_n7;
        locals.var_isbd_swg_dn8 = assign96180_e148775_d_n8;
        locals.var_isbd_swg_dn9 = assign96180_e148775_d_n9;
        locals.var_isbd_swg_dn10 = assign96180_e148775_d_n10;
        locals.var_isbd_swg_dn13 = assign96180_e148775_d_n13;
        locals.var_isbd_swg_rv = 0.0;

        let (assign96190_e148785, assign96190_e148785_d_n0, assign96190_e148785_d_n2, assign96190_e148785_d_n4, assign96190_e148785_d_n5, assign96190_e148785_d_n6, assign96190_e148785_d_n7, assign96190_e148785_d_n8, assign96190_e148785_d_n9, assign96190_e148785_d_n10, assign96190_e148785_d_n13,) = {
    if (((locals.var_guard2233 != 0.0) && (locals.var_guard2234 != 0.0)) && (locals.var_guard2235 != 0.0)) {
        let assign96190_e148783: f64 = (locals.var_weff_nf * locals.var_jsswg2);
        (assign96190_e148783, (locals.var_weff_nf * locals.var_jsswg2_dn0), (locals.var_weff_nf * locals.var_jsswg2_dn2), (locals.var_weff_nf * locals.var_jsswg2_dn4), (locals.var_weff_nf * locals.var_jsswg2_dn5), (locals.var_weff_nf * locals.var_jsswg2_dn6), (locals.var_weff_nf * locals.var_jsswg2_dn7), (locals.var_weff_nf * locals.var_jsswg2_dn8), (locals.var_weff_nf * locals.var_jsswg2_dn9), (locals.var_weff_nf * locals.var_jsswg2_dn10), (locals.var_weff_nf * locals.var_jsswg2_dn13),)
    } else {
        (locals.var_isbd2_swg, locals.var_isbd2_swg_dn0, locals.var_isbd2_swg_dn2, locals.var_isbd2_swg_dn4, locals.var_isbd2_swg_dn5, locals.var_isbd2_swg_dn6, locals.var_isbd2_swg_dn7, locals.var_isbd2_swg_dn8, locals.var_isbd2_swg_dn9, locals.var_isbd2_swg_dn10, locals.var_isbd2_swg_dn13,)
    }
};
        locals.var_isbd2_swg = assign96190_e148785;
        locals.var_isbd2_swg_dn0 = assign96190_e148785_d_n0;
        locals.var_isbd2_swg_dn2 = assign96190_e148785_d_n2;
        locals.var_isbd2_swg_dn4 = assign96190_e148785_d_n4;
        locals.var_isbd2_swg_dn5 = assign96190_e148785_d_n5;
        locals.var_isbd2_swg_dn6 = assign96190_e148785_d_n6;
        locals.var_isbd2_swg_dn7 = assign96190_e148785_d_n7;
        locals.var_isbd2_swg_dn8 = assign96190_e148785_d_n8;
        locals.var_isbd2_swg_dn9 = assign96190_e148785_d_n9;
        locals.var_isbd2_swg_dn10 = assign96190_e148785_d_n10;
        locals.var_isbd2_swg_dn13 = assign96190_e148785_d_n13;
        locals.var_isbd2_swg_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_361(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign96200_e148796, assign96200_e148796_d_n0, assign96200_e148796_d_n2, assign96200_e148796_d_n4, assign96200_e148796_d_n5, assign96200_e148796_d_n6, assign96200_e148796_d_n7, assign96200_e148796_d_n8, assign96200_e148796_d_n9, assign96200_e148796_d_n10, assign96200_e148796_d_n13,) = {
    if (((locals.var_guard2233 != 0.0) && (locals.var_guard2234 != 0.0)) && (locals.var_guard2235 == 0.0)) {
        let assign96200_e148794: f64 = (p.p13 * locals.var_js);
        (assign96200_e148794, (p.p13 * locals.var_js_dn0), (p.p13 * locals.var_js_dn2), (p.p13 * locals.var_js_dn4), (p.p13 * locals.var_js_dn5), (p.p13 * locals.var_js_dn6), (p.p13 * locals.var_js_dn7), (p.p13 * locals.var_js_dn8), (p.p13 * locals.var_js_dn9), (p.p13 * locals.var_js_dn10), (p.p13 * locals.var_js_dn13),)
    } else {
        (locals.var_isbd_btm, locals.var_isbd_btm_dn0, locals.var_isbd_btm_dn2, locals.var_isbd_btm_dn4, locals.var_isbd_btm_dn5, locals.var_isbd_btm_dn6, locals.var_isbd_btm_dn7, locals.var_isbd_btm_dn8, locals.var_isbd_btm_dn9, locals.var_isbd_btm_dn10, locals.var_isbd_btm_dn13,)
    }
};
        locals.var_isbd_btm = assign96200_e148796;
        locals.var_isbd_btm_dn0 = assign96200_e148796_d_n0;
        locals.var_isbd_btm_dn2 = assign96200_e148796_d_n2;
        locals.var_isbd_btm_dn4 = assign96200_e148796_d_n4;
        locals.var_isbd_btm_dn5 = assign96200_e148796_d_n5;
        locals.var_isbd_btm_dn6 = assign96200_e148796_d_n6;
        locals.var_isbd_btm_dn7 = assign96200_e148796_d_n7;
        locals.var_isbd_btm_dn8 = assign96200_e148796_d_n8;
        locals.var_isbd_btm_dn9 = assign96200_e148796_d_n9;
        locals.var_isbd_btm_dn10 = assign96200_e148796_d_n10;
        locals.var_isbd_btm_dn13 = assign96200_e148796_d_n13;
        locals.var_isbd_btm_rv = 0.0;

        let (assign96210_e148807, assign96210_e148807_d_n0, assign96210_e148807_d_n2, assign96210_e148807_d_n4, assign96210_e148807_d_n5, assign96210_e148807_d_n6, assign96210_e148807_d_n7, assign96210_e148807_d_n8, assign96210_e148807_d_n9, assign96210_e148807_d_n10, assign96210_e148807_d_n13,) = {
    if (((locals.var_guard2233 != 0.0) && (locals.var_guard2234 != 0.0)) && (locals.var_guard2235 == 0.0)) {
        let assign96210_e148805: f64 = (p.p13 * locals.var_js2);
        (assign96210_e148805, (p.p13 * locals.var_js2_dn0), (p.p13 * locals.var_js2_dn2), (p.p13 * locals.var_js2_dn4), (p.p13 * locals.var_js2_dn5), (p.p13 * locals.var_js2_dn6), (p.p13 * locals.var_js2_dn7), (p.p13 * locals.var_js2_dn8), (p.p13 * locals.var_js2_dn9), (p.p13 * locals.var_js2_dn10), (p.p13 * locals.var_js2_dn13),)
    } else {
        (locals.var_isbd2_btm, locals.var_isbd2_btm_dn0, locals.var_isbd2_btm_dn2, locals.var_isbd2_btm_dn4, locals.var_isbd2_btm_dn5, locals.var_isbd2_btm_dn6, locals.var_isbd2_btm_dn7, locals.var_isbd2_btm_dn8, locals.var_isbd2_btm_dn9, locals.var_isbd2_btm_dn10, locals.var_isbd2_btm_dn13,)
    }
};
        locals.var_isbd2_btm = assign96210_e148807;
        locals.var_isbd2_btm_dn0 = assign96210_e148807_d_n0;
        locals.var_isbd2_btm_dn2 = assign96210_e148807_d_n2;
        locals.var_isbd2_btm_dn4 = assign96210_e148807_d_n4;
        locals.var_isbd2_btm_dn5 = assign96210_e148807_d_n5;
        locals.var_isbd2_btm_dn6 = assign96210_e148807_d_n6;
        locals.var_isbd2_btm_dn7 = assign96210_e148807_d_n7;
        locals.var_isbd2_btm_dn8 = assign96210_e148807_d_n8;
        locals.var_isbd2_btm_dn9 = assign96210_e148807_d_n9;
        locals.var_isbd2_btm_dn10 = assign96210_e148807_d_n10;
        locals.var_isbd2_btm_dn13 = assign96210_e148807_d_n13;
        locals.var_isbd2_btm_rv = 0.0;

        let (assign96220_e148816, assign96220_e148816_d_n0, assign96220_e148816_d_n2, assign96220_e148816_d_n4, assign96220_e148816_d_n5, assign96220_e148816_d_n6, assign96220_e148816_d_n7, assign96220_e148816_d_n8, assign96220_e148816_d_n9, assign96220_e148816_d_n10, assign96220_e148816_d_n13,) = {
    if (((locals.var_guard2233 != 0.0) && (locals.var_guard2234 != 0.0)) && (locals.var_guard2235 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_isbd_sws, locals.var_isbd_sws_dn0, locals.var_isbd_sws_dn2, locals.var_isbd_sws_dn4, locals.var_isbd_sws_dn5, locals.var_isbd_sws_dn6, locals.var_isbd_sws_dn7, locals.var_isbd_sws_dn8, locals.var_isbd_sws_dn9, locals.var_isbd_sws_dn10, locals.var_isbd_sws_dn13,)
    }
};
        locals.var_isbd_sws = assign96220_e148816;
        locals.var_isbd_sws_dn0 = assign96220_e148816_d_n0;
        locals.var_isbd_sws_dn2 = assign96220_e148816_d_n2;
        locals.var_isbd_sws_dn4 = assign96220_e148816_d_n4;
        locals.var_isbd_sws_dn5 = assign96220_e148816_d_n5;
        locals.var_isbd_sws_dn6 = assign96220_e148816_d_n6;
        locals.var_isbd_sws_dn7 = assign96220_e148816_d_n7;
        locals.var_isbd_sws_dn8 = assign96220_e148816_d_n8;
        locals.var_isbd_sws_dn9 = assign96220_e148816_d_n9;
        locals.var_isbd_sws_dn10 = assign96220_e148816_d_n10;
        locals.var_isbd_sws_dn13 = assign96220_e148816_d_n13;
        locals.var_isbd_sws_rv = 0.0;

        let (assign96230_e148825, assign96230_e148825_d_n0, assign96230_e148825_d_n2, assign96230_e148825_d_n4, assign96230_e148825_d_n5, assign96230_e148825_d_n6, assign96230_e148825_d_n7, assign96230_e148825_d_n8, assign96230_e148825_d_n9, assign96230_e148825_d_n10, assign96230_e148825_d_n13,) = {
    if (((locals.var_guard2233 != 0.0) && (locals.var_guard2234 != 0.0)) && (locals.var_guard2235 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_isbd2_sws, locals.var_isbd2_sws_dn0, locals.var_isbd2_sws_dn2, locals.var_isbd2_sws_dn4, locals.var_isbd2_sws_dn5, locals.var_isbd2_sws_dn6, locals.var_isbd2_sws_dn7, locals.var_isbd2_sws_dn8, locals.var_isbd2_sws_dn9, locals.var_isbd2_sws_dn10, locals.var_isbd2_sws_dn13,)
    }
};
        locals.var_isbd2_sws = assign96230_e148825;
        locals.var_isbd2_sws_dn0 = assign96230_e148825_d_n0;
        locals.var_isbd2_sws_dn2 = assign96230_e148825_d_n2;
        locals.var_isbd2_sws_dn4 = assign96230_e148825_d_n4;
        locals.var_isbd2_sws_dn5 = assign96230_e148825_d_n5;
        locals.var_isbd2_sws_dn6 = assign96230_e148825_d_n6;
        locals.var_isbd2_sws_dn7 = assign96230_e148825_d_n7;
        locals.var_isbd2_sws_dn8 = assign96230_e148825_d_n8;
        locals.var_isbd2_sws_dn9 = assign96230_e148825_d_n9;
        locals.var_isbd2_sws_dn10 = assign96230_e148825_d_n10;
        locals.var_isbd2_sws_dn13 = assign96230_e148825_d_n13;
        locals.var_isbd2_sws_rv = 0.0;

        let (assign96240_e148836, assign96240_e148836_d_n0, assign96240_e148836_d_n2, assign96240_e148836_d_n4, assign96240_e148836_d_n5, assign96240_e148836_d_n6, assign96240_e148836_d_n7, assign96240_e148836_d_n8, assign96240_e148836_d_n9, assign96240_e148836_d_n10, assign96240_e148836_d_n13,) = {
    if (((locals.var_guard2233 != 0.0) && (locals.var_guard2234 != 0.0)) && (locals.var_guard2235 == 0.0)) {
        let assign96240_e148834: f64 = (p.p15 * locals.var_jsswg);
        (assign96240_e148834, (p.p15 * locals.var_jsswg_dn0), (p.p15 * locals.var_jsswg_dn2), (p.p15 * locals.var_jsswg_dn4), (p.p15 * locals.var_jsswg_dn5), (p.p15 * locals.var_jsswg_dn6), (p.p15 * locals.var_jsswg_dn7), (p.p15 * locals.var_jsswg_dn8), (p.p15 * locals.var_jsswg_dn9), (p.p15 * locals.var_jsswg_dn10), (p.p15 * locals.var_jsswg_dn13),)
    } else {
        (locals.var_isbd_swg, locals.var_isbd_swg_dn0, locals.var_isbd_swg_dn2, locals.var_isbd_swg_dn4, locals.var_isbd_swg_dn5, locals.var_isbd_swg_dn6, locals.var_isbd_swg_dn7, locals.var_isbd_swg_dn8, locals.var_isbd_swg_dn9, locals.var_isbd_swg_dn10, locals.var_isbd_swg_dn13,)
    }
};
        locals.var_isbd_swg = assign96240_e148836;
        locals.var_isbd_swg_dn0 = assign96240_e148836_d_n0;
        locals.var_isbd_swg_dn2 = assign96240_e148836_d_n2;
        locals.var_isbd_swg_dn4 = assign96240_e148836_d_n4;
        locals.var_isbd_swg_dn5 = assign96240_e148836_d_n5;
        locals.var_isbd_swg_dn6 = assign96240_e148836_d_n6;
        locals.var_isbd_swg_dn7 = assign96240_e148836_d_n7;
        locals.var_isbd_swg_dn8 = assign96240_e148836_d_n8;
        locals.var_isbd_swg_dn9 = assign96240_e148836_d_n9;
        locals.var_isbd_swg_dn10 = assign96240_e148836_d_n10;
        locals.var_isbd_swg_dn13 = assign96240_e148836_d_n13;
        locals.var_isbd_swg_rv = 0.0;

        let (assign96250_e148847, assign96250_e148847_d_n0, assign96250_e148847_d_n2, assign96250_e148847_d_n4, assign96250_e148847_d_n5, assign96250_e148847_d_n6, assign96250_e148847_d_n7, assign96250_e148847_d_n8, assign96250_e148847_d_n9, assign96250_e148847_d_n10, assign96250_e148847_d_n13,) = {
    if (((locals.var_guard2233 != 0.0) && (locals.var_guard2234 != 0.0)) && (locals.var_guard2235 == 0.0)) {
        let assign96250_e148845: f64 = (p.p15 * locals.var_jsswg2);
        (assign96250_e148845, (p.p15 * locals.var_jsswg2_dn0), (p.p15 * locals.var_jsswg2_dn2), (p.p15 * locals.var_jsswg2_dn4), (p.p15 * locals.var_jsswg2_dn5), (p.p15 * locals.var_jsswg2_dn6), (p.p15 * locals.var_jsswg2_dn7), (p.p15 * locals.var_jsswg2_dn8), (p.p15 * locals.var_jsswg2_dn9), (p.p15 * locals.var_jsswg2_dn10), (p.p15 * locals.var_jsswg2_dn13),)
    } else {
        (locals.var_isbd2_swg, locals.var_isbd2_swg_dn0, locals.var_isbd2_swg_dn2, locals.var_isbd2_swg_dn4, locals.var_isbd2_swg_dn5, locals.var_isbd2_swg_dn6, locals.var_isbd2_swg_dn7, locals.var_isbd2_swg_dn8, locals.var_isbd2_swg_dn9, locals.var_isbd2_swg_dn10, locals.var_isbd2_swg_dn13,)
    }
};
        locals.var_isbd2_swg = assign96250_e148847;
        locals.var_isbd2_swg_dn0 = assign96250_e148847_d_n0;
        locals.var_isbd2_swg_dn2 = assign96250_e148847_d_n2;
        locals.var_isbd2_swg_dn4 = assign96250_e148847_d_n4;
        locals.var_isbd2_swg_dn5 = assign96250_e148847_d_n5;
        locals.var_isbd2_swg_dn6 = assign96250_e148847_d_n6;
        locals.var_isbd2_swg_dn7 = assign96250_e148847_d_n7;
        locals.var_isbd2_swg_dn8 = assign96250_e148847_d_n8;
        locals.var_isbd2_swg_dn9 = assign96250_e148847_d_n9;
        locals.var_isbd2_swg_dn10 = assign96250_e148847_d_n10;
        locals.var_isbd2_swg_dn13 = assign96250_e148847_d_n13;
        locals.var_isbd2_swg_rv = 0.0;

        let (assign96260_e148856, assign96260_e148856_d_n0, assign96260_e148856_d_n2, assign96260_e148856_d_n4, assign96260_e148856_d_n5, assign96260_e148856_d_n6, assign96260_e148856_d_n7, assign96260_e148856_d_n8, assign96260_e148856_d_n9, assign96260_e148856_d_n10, assign96260_e148856_d_n13,) = {
    if ((locals.var_guard2233 != 0.0) && (locals.var_guard2234 == 0.0)) {
        let assign96260_e148854: f64 = (p.p13 * locals.var_js);
        (assign96260_e148854, (p.p13 * locals.var_js_dn0), (p.p13 * locals.var_js_dn2), (p.p13 * locals.var_js_dn4), (p.p13 * locals.var_js_dn5), (p.p13 * locals.var_js_dn6), (p.p13 * locals.var_js_dn7), (p.p13 * locals.var_js_dn8), (p.p13 * locals.var_js_dn9), (p.p13 * locals.var_js_dn10), (p.p13 * locals.var_js_dn13),)
    } else {
        (locals.var_isbd_btm, locals.var_isbd_btm_dn0, locals.var_isbd_btm_dn2, locals.var_isbd_btm_dn4, locals.var_isbd_btm_dn5, locals.var_isbd_btm_dn6, locals.var_isbd_btm_dn7, locals.var_isbd_btm_dn8, locals.var_isbd_btm_dn9, locals.var_isbd_btm_dn10, locals.var_isbd_btm_dn13,)
    }
};
        locals.var_isbd_btm = assign96260_e148856;
        locals.var_isbd_btm_dn0 = assign96260_e148856_d_n0;
        locals.var_isbd_btm_dn2 = assign96260_e148856_d_n2;
        locals.var_isbd_btm_dn4 = assign96260_e148856_d_n4;
        locals.var_isbd_btm_dn5 = assign96260_e148856_d_n5;
        locals.var_isbd_btm_dn6 = assign96260_e148856_d_n6;
        locals.var_isbd_btm_dn7 = assign96260_e148856_d_n7;
        locals.var_isbd_btm_dn8 = assign96260_e148856_d_n8;
        locals.var_isbd_btm_dn9 = assign96260_e148856_d_n9;
        locals.var_isbd_btm_dn10 = assign96260_e148856_d_n10;
        locals.var_isbd_btm_dn13 = assign96260_e148856_d_n13;
        locals.var_isbd_btm_rv = 0.0;

        let (assign96270_e148865, assign96270_e148865_d_n0, assign96270_e148865_d_n2, assign96270_e148865_d_n4, assign96270_e148865_d_n5, assign96270_e148865_d_n6, assign96270_e148865_d_n7, assign96270_e148865_d_n8, assign96270_e148865_d_n9, assign96270_e148865_d_n10, assign96270_e148865_d_n13,) = {
    if ((locals.var_guard2233 != 0.0) && (locals.var_guard2234 == 0.0)) {
        let assign96270_e148863: f64 = (p.p13 * locals.var_js2);
        (assign96270_e148863, (p.p13 * locals.var_js2_dn0), (p.p13 * locals.var_js2_dn2), (p.p13 * locals.var_js2_dn4), (p.p13 * locals.var_js2_dn5), (p.p13 * locals.var_js2_dn6), (p.p13 * locals.var_js2_dn7), (p.p13 * locals.var_js2_dn8), (p.p13 * locals.var_js2_dn9), (p.p13 * locals.var_js2_dn10), (p.p13 * locals.var_js2_dn13),)
    } else {
        (locals.var_isbd2_btm, locals.var_isbd2_btm_dn0, locals.var_isbd2_btm_dn2, locals.var_isbd2_btm_dn4, locals.var_isbd2_btm_dn5, locals.var_isbd2_btm_dn6, locals.var_isbd2_btm_dn7, locals.var_isbd2_btm_dn8, locals.var_isbd2_btm_dn9, locals.var_isbd2_btm_dn10, locals.var_isbd2_btm_dn13,)
    }
};
        locals.var_isbd2_btm = assign96270_e148865;
        locals.var_isbd2_btm_dn0 = assign96270_e148865_d_n0;
        locals.var_isbd2_btm_dn2 = assign96270_e148865_d_n2;
        locals.var_isbd2_btm_dn4 = assign96270_e148865_d_n4;
        locals.var_isbd2_btm_dn5 = assign96270_e148865_d_n5;
        locals.var_isbd2_btm_dn6 = assign96270_e148865_d_n6;
        locals.var_isbd2_btm_dn7 = assign96270_e148865_d_n7;
        locals.var_isbd2_btm_dn8 = assign96270_e148865_d_n8;
        locals.var_isbd2_btm_dn9 = assign96270_e148865_d_n9;
        locals.var_isbd2_btm_dn10 = assign96270_e148865_d_n10;
        locals.var_isbd2_btm_dn13 = assign96270_e148865_d_n13;
        locals.var_isbd2_btm_rv = 0.0;

        let (assign96280_e148874, assign96280_e148874_d_n0, assign96280_e148874_d_n2, assign96280_e148874_d_n4, assign96280_e148874_d_n5, assign96280_e148874_d_n6, assign96280_e148874_d_n7, assign96280_e148874_d_n8, assign96280_e148874_d_n9, assign96280_e148874_d_n10, assign96280_e148874_d_n13,) = {
    if ((locals.var_guard2233 != 0.0) && (locals.var_guard2234 == 0.0)) {
        let assign96280_e148872: f64 = (p.p15 * locals.var_jssw);
        (assign96280_e148872, (p.p15 * locals.var_jssw_dn0), (p.p15 * locals.var_jssw_dn2), (p.p15 * locals.var_jssw_dn4), (p.p15 * locals.var_jssw_dn5), (p.p15 * locals.var_jssw_dn6), (p.p15 * locals.var_jssw_dn7), (p.p15 * locals.var_jssw_dn8), (p.p15 * locals.var_jssw_dn9), (p.p15 * locals.var_jssw_dn10), (p.p15 * locals.var_jssw_dn13),)
    } else {
        (locals.var_isbd_sws, locals.var_isbd_sws_dn0, locals.var_isbd_sws_dn2, locals.var_isbd_sws_dn4, locals.var_isbd_sws_dn5, locals.var_isbd_sws_dn6, locals.var_isbd_sws_dn7, locals.var_isbd_sws_dn8, locals.var_isbd_sws_dn9, locals.var_isbd_sws_dn10, locals.var_isbd_sws_dn13,)
    }
};
        locals.var_isbd_sws = assign96280_e148874;
        locals.var_isbd_sws_dn0 = assign96280_e148874_d_n0;
        locals.var_isbd_sws_dn2 = assign96280_e148874_d_n2;
        locals.var_isbd_sws_dn4 = assign96280_e148874_d_n4;
        locals.var_isbd_sws_dn5 = assign96280_e148874_d_n5;
        locals.var_isbd_sws_dn6 = assign96280_e148874_d_n6;
        locals.var_isbd_sws_dn7 = assign96280_e148874_d_n7;
        locals.var_isbd_sws_dn8 = assign96280_e148874_d_n8;
        locals.var_isbd_sws_dn9 = assign96280_e148874_d_n9;
        locals.var_isbd_sws_dn10 = assign96280_e148874_d_n10;
        locals.var_isbd_sws_dn13 = assign96280_e148874_d_n13;
        locals.var_isbd_sws_rv = 0.0;

        let (assign96290_e148883, assign96290_e148883_d_n0, assign96290_e148883_d_n2, assign96290_e148883_d_n4, assign96290_e148883_d_n5, assign96290_e148883_d_n6, assign96290_e148883_d_n7, assign96290_e148883_d_n8, assign96290_e148883_d_n9, assign96290_e148883_d_n10, assign96290_e148883_d_n13,) = {
    if ((locals.var_guard2233 != 0.0) && (locals.var_guard2234 == 0.0)) {
        let assign96290_e148881: f64 = (p.p15 * locals.var_jssw2);
        (assign96290_e148881, (p.p15 * locals.var_jssw2_dn0), (p.p15 * locals.var_jssw2_dn2), (p.p15 * locals.var_jssw2_dn4), (p.p15 * locals.var_jssw2_dn5), (p.p15 * locals.var_jssw2_dn6), (p.p15 * locals.var_jssw2_dn7), (p.p15 * locals.var_jssw2_dn8), (p.p15 * locals.var_jssw2_dn9), (p.p15 * locals.var_jssw2_dn10), (p.p15 * locals.var_jssw2_dn13),)
    } else {
        (locals.var_isbd2_sws, locals.var_isbd2_sws_dn0, locals.var_isbd2_sws_dn2, locals.var_isbd2_sws_dn4, locals.var_isbd2_sws_dn5, locals.var_isbd2_sws_dn6, locals.var_isbd2_sws_dn7, locals.var_isbd2_sws_dn8, locals.var_isbd2_sws_dn9, locals.var_isbd2_sws_dn10, locals.var_isbd2_sws_dn13,)
    }
};
        locals.var_isbd2_sws = assign96290_e148883;
        locals.var_isbd2_sws_dn0 = assign96290_e148883_d_n0;
        locals.var_isbd2_sws_dn2 = assign96290_e148883_d_n2;
        locals.var_isbd2_sws_dn4 = assign96290_e148883_d_n4;
        locals.var_isbd2_sws_dn5 = assign96290_e148883_d_n5;
        locals.var_isbd2_sws_dn6 = assign96290_e148883_d_n6;
        locals.var_isbd2_sws_dn7 = assign96290_e148883_d_n7;
        locals.var_isbd2_sws_dn8 = assign96290_e148883_d_n8;
        locals.var_isbd2_sws_dn9 = assign96290_e148883_d_n9;
        locals.var_isbd2_sws_dn10 = assign96290_e148883_d_n10;
        locals.var_isbd2_sws_dn13 = assign96290_e148883_d_n13;
        locals.var_isbd2_sws_rv = 0.0;

        let (assign96300_e148890, assign96300_e148890_d_n0, assign96300_e148890_d_n2, assign96300_e148890_d_n4, assign96300_e148890_d_n5, assign96300_e148890_d_n6, assign96300_e148890_d_n7, assign96300_e148890_d_n8, assign96300_e148890_d_n9, assign96300_e148890_d_n10, assign96300_e148890_d_n13,) = {
    if ((locals.var_guard2233 != 0.0) && (locals.var_guard2234 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_isbd_swg, locals.var_isbd_swg_dn0, locals.var_isbd_swg_dn2, locals.var_isbd_swg_dn4, locals.var_isbd_swg_dn5, locals.var_isbd_swg_dn6, locals.var_isbd_swg_dn7, locals.var_isbd_swg_dn8, locals.var_isbd_swg_dn9, locals.var_isbd_swg_dn10, locals.var_isbd_swg_dn13,)
    }
};
        locals.var_isbd_swg = assign96300_e148890;
        locals.var_isbd_swg_dn0 = assign96300_e148890_d_n0;
        locals.var_isbd_swg_dn2 = assign96300_e148890_d_n2;
        locals.var_isbd_swg_dn4 = assign96300_e148890_d_n4;
        locals.var_isbd_swg_dn5 = assign96300_e148890_d_n5;
        locals.var_isbd_swg_dn6 = assign96300_e148890_d_n6;
        locals.var_isbd_swg_dn7 = assign96300_e148890_d_n7;
        locals.var_isbd_swg_dn8 = assign96300_e148890_d_n8;
        locals.var_isbd_swg_dn9 = assign96300_e148890_d_n9;
        locals.var_isbd_swg_dn10 = assign96300_e148890_d_n10;
        locals.var_isbd_swg_dn13 = assign96300_e148890_d_n13;
        locals.var_isbd_swg_rv = 0.0;

        let (assign96310_e148897, assign96310_e148897_d_n0, assign96310_e148897_d_n2, assign96310_e148897_d_n4, assign96310_e148897_d_n5, assign96310_e148897_d_n6, assign96310_e148897_d_n7, assign96310_e148897_d_n8, assign96310_e148897_d_n9, assign96310_e148897_d_n10, assign96310_e148897_d_n13,) = {
    if ((locals.var_guard2233 != 0.0) && (locals.var_guard2234 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_isbd2_swg, locals.var_isbd2_swg_dn0, locals.var_isbd2_swg_dn2, locals.var_isbd2_swg_dn4, locals.var_isbd2_swg_dn5, locals.var_isbd2_swg_dn6, locals.var_isbd2_swg_dn7, locals.var_isbd2_swg_dn8, locals.var_isbd2_swg_dn9, locals.var_isbd2_swg_dn10, locals.var_isbd2_swg_dn13,)
    }
};
        locals.var_isbd2_swg = assign96310_e148897;
        locals.var_isbd2_swg_dn0 = assign96310_e148897_d_n0;
        locals.var_isbd2_swg_dn2 = assign96310_e148897_d_n2;
        locals.var_isbd2_swg_dn4 = assign96310_e148897_d_n4;
        locals.var_isbd2_swg_dn5 = assign96310_e148897_d_n5;
        locals.var_isbd2_swg_dn6 = assign96310_e148897_d_n6;
        locals.var_isbd2_swg_dn7 = assign96310_e148897_d_n7;
        locals.var_isbd2_swg_dn8 = assign96310_e148897_d_n8;
        locals.var_isbd2_swg_dn9 = assign96310_e148897_d_n9;
        locals.var_isbd2_swg_dn10 = assign96310_e148897_d_n10;
        locals.var_isbd2_swg_dn13 = assign96310_e148897_d_n13;
        locals.var_isbd2_swg_rv = 0.0;

        let (assign96320_e148905, assign96320_e148905_d_n0, assign96320_e148905_d_n2, assign96320_e148905_d_n4, assign96320_e148905_d_n5, assign96320_e148905_d_n6, assign96320_e148905_d_n7, assign96320_e148905_d_n8, assign96320_e148905_d_n9, assign96320_e148905_d_n10, assign96320_e148905_d_n13,) = {
    if (locals.var_guard2233 != 0.0) {
        let assign96320_e148901: f64 = (locals.var_isbd_btm + locals.var_isbd_sws);
        let assign96320_e148903: f64 = (assign96320_e148901 + locals.var_isbd_swg);
        (assign96320_e148903, ((locals.var_isbd_btm_dn0 + locals.var_isbd_sws_dn0) + locals.var_isbd_swg_dn0), ((locals.var_isbd_btm_dn2 + locals.var_isbd_sws_dn2) + locals.var_isbd_swg_dn2), ((locals.var_isbd_btm_dn4 + locals.var_isbd_sws_dn4) + locals.var_isbd_swg_dn4), ((locals.var_isbd_btm_dn5 + locals.var_isbd_sws_dn5) + locals.var_isbd_swg_dn5), ((locals.var_isbd_btm_dn6 + locals.var_isbd_sws_dn6) + locals.var_isbd_swg_dn6), ((locals.var_isbd_btm_dn7 + locals.var_isbd_sws_dn7) + locals.var_isbd_swg_dn7), ((locals.var_isbd_btm_dn8 + locals.var_isbd_sws_dn8) + locals.var_isbd_swg_dn8), ((locals.var_isbd_btm_dn9 + locals.var_isbd_sws_dn9) + locals.var_isbd_swg_dn9), ((locals.var_isbd_btm_dn10 + locals.var_isbd_sws_dn10) + locals.var_isbd_swg_dn10), ((locals.var_isbd_btm_dn13 + locals.var_isbd_sws_dn13) + locals.var_isbd_swg_dn13),)
    } else {
        (locals.var_isbd, locals.var_isbd_dn0, locals.var_isbd_dn2, locals.var_isbd_dn4, locals.var_isbd_dn5, locals.var_isbd_dn6, locals.var_isbd_dn7, locals.var_isbd_dn8, locals.var_isbd_dn9, locals.var_isbd_dn10, locals.var_isbd_dn13,)
    }
};
        locals.var_isbd = assign96320_e148905;
        locals.var_isbd_dn0 = assign96320_e148905_d_n0;
        locals.var_isbd_dn2 = assign96320_e148905_d_n2;
        locals.var_isbd_dn4 = assign96320_e148905_d_n4;
        locals.var_isbd_dn5 = assign96320_e148905_d_n5;
        locals.var_isbd_dn6 = assign96320_e148905_d_n6;
        locals.var_isbd_dn7 = assign96320_e148905_d_n7;
        locals.var_isbd_dn8 = assign96320_e148905_d_n8;
        locals.var_isbd_dn9 = assign96320_e148905_d_n9;
        locals.var_isbd_dn10 = assign96320_e148905_d_n10;
        locals.var_isbd_dn13 = assign96320_e148905_d_n13;
        locals.var_isbd_rv = 0.0;

        let assign96330_e148908: f64 = if locals.var_isbd > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2236 = assign96330_e148908;
        locals.var_guard2236_rv = 0.0;

        let (assign96340_e148916, assign96340_e148916_d_n0, assign96340_e148916_d_n2, assign96340_e148916_d_n4, assign96340_e148916_d_n5, assign96340_e148916_d_n6, assign96340_e148916_d_n7, assign96340_e148916_d_n8, assign96340_e148916_d_n9, assign96340_e148916_d_n10, assign96340_e148916_d_n13,) = {
    if ((locals.var_guard2233 != 0.0) && (locals.var_guard2236 != 0.0)) {
        let assign96340_e148914: f64 = (locals.var_isbd + 1e-25);
        (assign96340_e148914, locals.var_isbd_dn0, locals.var_isbd_dn2, locals.var_isbd_dn4, locals.var_isbd_dn5, locals.var_isbd_dn6, locals.var_isbd_dn7, locals.var_isbd_dn8, locals.var_isbd_dn9, locals.var_isbd_dn10, locals.var_isbd_dn13,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign96340_e148916;
        locals.var_t2_dn0 = assign96340_e148916_d_n0;
        locals.var_t2_dn2 = assign96340_e148916_d_n2;
        locals.var_t2_dn4 = assign96340_e148916_d_n4;
        locals.var_t2_dn5 = assign96340_e148916_d_n5;
        locals.var_t2_dn6 = assign96340_e148916_d_n6;
        locals.var_t2_dn7 = assign96340_e148916_d_n7;
        locals.var_t2_dn8 = assign96340_e148916_d_n8;
        locals.var_t2_dn9 = assign96340_e148916_d_n9;
        locals.var_t2_dn10 = assign96340_e148916_d_n10;
        locals.var_t2_dn13 = assign96340_e148916_d_n13;
        locals.var_t2_rv = 0.0;

        let (assign96350_e148933, assign96350_e148933_d_n0, assign96350_e148933_d_n2, assign96350_e148933_d_n4, assign96350_e148933_d_n5, assign96350_e148933_d_n6, assign96350_e148933_d_n7, assign96350_e148933_d_n8, assign96350_e148933_d_n9, assign96350_e148933_d_n10, assign96350_e148933_d_n13,) = {
    if ((locals.var_guard2233 != 0.0) && (locals.var_guard2236 != 0.0)) {
        let assign96350_e148922: f64 = (locals.var_uc_njd / locals.var_beta);
        let assign96350_e148925: f64 = (locals.var_uc_vdiffjd * locals.var_t0);
        let assign96350_e148927: f64 = (assign96350_e148925 / locals.var_t2);
        let assign96350_e148929: f64 = (assign96350_e148927 + 1.0);
        let assign96350_e148930: f64 = (assign96350_e148929).ln();
        let assign96350_e148931: f64 = (assign96350_e148922 * assign96350_e148930);
        (assign96350_e148931, (((-((locals.var_uc_njd * locals.var_beta_dn0) / (locals.var_beta * locals.var_beta))) * assign96350_e148930) + (assign96350_e148922 * (((((locals.var_uc_vdiffjd * locals.var_t0_dn0) * locals.var_t2) - (assign96350_e148925 * locals.var_t2_dn0)) / (locals.var_t2 * locals.var_t2)) / assign96350_e148929))), (((-((locals.var_uc_njd * locals.var_beta_dn2) / (locals.var_beta * locals.var_beta))) * assign96350_e148930) + (assign96350_e148922 * (((((locals.var_uc_vdiffjd * locals.var_t0_dn2) * locals.var_t2) - (assign96350_e148925 * locals.var_t2_dn2)) / (locals.var_t2 * locals.var_t2)) / assign96350_e148929))), (((-((locals.var_uc_njd * locals.var_beta_dn4) / (locals.var_beta * locals.var_beta))) * assign96350_e148930) + (assign96350_e148922 * (((((locals.var_uc_vdiffjd * locals.var_t0_dn4) * locals.var_t2) - (assign96350_e148925 * locals.var_t2_dn4)) / (locals.var_t2 * locals.var_t2)) / assign96350_e148929))), (((-((locals.var_uc_njd * locals.var_beta_dn5) / (locals.var_beta * locals.var_beta))) * assign96350_e148930) + (assign96350_e148922 * (((((locals.var_uc_vdiffjd * locals.var_t0_dn5) * locals.var_t2) - (assign96350_e148925 * locals.var_t2_dn5)) / (locals.var_t2 * locals.var_t2)) / assign96350_e148929))), (((-((locals.var_uc_njd * locals.var_beta_dn6) / (locals.var_beta * locals.var_beta))) * assign96350_e148930) + (assign96350_e148922 * (((((locals.var_uc_vdiffjd * locals.var_t0_dn6) * locals.var_t2) - (assign96350_e148925 * locals.var_t2_dn6)) / (locals.var_t2 * locals.var_t2)) / assign96350_e148929))), (((-((locals.var_uc_njd * locals.var_beta_dn7) / (locals.var_beta * locals.var_beta))) * assign96350_e148930) + (assign96350_e148922 * (((((locals.var_uc_vdiffjd * locals.var_t0_dn7) * locals.var_t2) - (assign96350_e148925 * locals.var_t2_dn7)) / (locals.var_t2 * locals.var_t2)) / assign96350_e148929))), (((-((locals.var_uc_njd * locals.var_beta_dn8) / (locals.var_beta * locals.var_beta))) * assign96350_e148930) + (assign96350_e148922 * (((((locals.var_uc_vdiffjd * locals.var_t0_dn8) * locals.var_t2) - (assign96350_e148925 * locals.var_t2_dn8)) / (locals.var_t2 * locals.var_t2)) / assign96350_e148929))), (((-((locals.var_uc_njd * locals.var_beta_dn9) / (locals.var_beta * locals.var_beta))) * assign96350_e148930) + (assign96350_e148922 * (((((locals.var_uc_vdiffjd * locals.var_t0_dn9) * locals.var_t2) - (assign96350_e148925 * locals.var_t2_dn9)) / (locals.var_t2 * locals.var_t2)) / assign96350_e148929))), (((-((locals.var_uc_njd * locals.var_beta_dn10) / (locals.var_beta * locals.var_beta))) * assign96350_e148930) + (assign96350_e148922 * (((((locals.var_uc_vdiffjd * locals.var_t0_dn10) * locals.var_t2) - (assign96350_e148925 * locals.var_t2_dn10)) / (locals.var_t2 * locals.var_t2)) / assign96350_e148929))), (((-((locals.var_uc_njd * locals.var_beta_dn13) / (locals.var_beta * locals.var_beta))) * assign96350_e148930) + (assign96350_e148922 * (((((locals.var_uc_vdiffjd * locals.var_t0_dn13) * locals.var_t2) - (assign96350_e148925 * locals.var_t2_dn13)) / (locals.var_t2 * locals.var_t2)) / assign96350_e148929))),)
    } else {
        (locals.var_vbdt, locals.var_vbdt_dn0, locals.var_vbdt_dn2, locals.var_vbdt_dn4, locals.var_vbdt_dn5, locals.var_vbdt_dn6, locals.var_vbdt_dn7, locals.var_vbdt_dn8, locals.var_vbdt_dn9, locals.var_vbdt_dn10, locals.var_vbdt_dn13,)
    }
};
        locals.var_vbdt = assign96350_e148933;
        locals.var_vbdt_dn0 = assign96350_e148933_d_n0;
        locals.var_vbdt_dn2 = assign96350_e148933_d_n2;
        locals.var_vbdt_dn4 = assign96350_e148933_d_n4;
        locals.var_vbdt_dn5 = assign96350_e148933_d_n5;
        locals.var_vbdt_dn6 = assign96350_e148933_d_n6;
        locals.var_vbdt_dn7 = assign96350_e148933_d_n7;
        locals.var_vbdt_dn8 = assign96350_e148933_d_n8;
        locals.var_vbdt_dn9 = assign96350_e148933_d_n9;
        locals.var_vbdt_dn10 = assign96350_e148933_d_n10;
        locals.var_vbdt_dn13 = assign96350_e148933_d_n13;
        locals.var_vbdt_rv = 0.0;

        let (assign96360_e148944, assign96360_e148944_d_n0, assign96360_e148944_d_n2, assign96360_e148944_d_n4, assign96360_e148944_d_n5, assign96360_e148944_d_n6, assign96360_e148944_d_n7, assign96360_e148944_d_n8, assign96360_e148944_d_n9, assign96360_e148944_d_n10, assign96360_e148944_d_n13,) = {
    if ((locals.var_guard2233 != 0.0) && (locals.var_guard2236 != 0.0)) {
        let assign96360_e148939: f64 = (locals.var_tratio - 1.0);
        let assign96360_e148941: f64 = (assign96360_e148939 * p.p512);
        let assign96360_e148942: f64 = (assign96360_e148941).exp();
        (assign96360_e148942, (assign96360_e148942 * (locals.var_tratio_dn0 * p.p512)), (assign96360_e148942 * (locals.var_tratio_dn2 * p.p512)), (assign96360_e148942 * (locals.var_tratio_dn4 * p.p512)), (assign96360_e148942 * (locals.var_tratio_dn5 * p.p512)), (assign96360_e148942 * (locals.var_tratio_dn6 * p.p512)), (assign96360_e148942 * (locals.var_tratio_dn7 * p.p512)), (assign96360_e148942 * (locals.var_tratio_dn8 * p.p512)), (assign96360_e148942 * (locals.var_tratio_dn9 * p.p512)), (assign96360_e148942 * (locals.var_tratio_dn10 * p.p512)), (assign96360_e148942 * (locals.var_tratio_dn13 * p.p512)),)
    } else {
        (locals.var_exptempd, locals.var_exptempd_dn0, locals.var_exptempd_dn2, locals.var_exptempd_dn4, locals.var_exptempd_dn5, locals.var_exptempd_dn6, locals.var_exptempd_dn7, locals.var_exptempd_dn8, locals.var_exptempd_dn9, locals.var_exptempd_dn10, locals.var_exptempd_dn13,)
    }
};
        locals.var_exptempd = assign96360_e148944;
        locals.var_exptempd_dn0 = assign96360_e148944_d_n0;
        locals.var_exptempd_dn2 = assign96360_e148944_d_n2;
        locals.var_exptempd_dn4 = assign96360_e148944_d_n4;
        locals.var_exptempd_dn5 = assign96360_e148944_d_n5;
        locals.var_exptempd_dn6 = assign96360_e148944_d_n6;
        locals.var_exptempd_dn7 = assign96360_e148944_d_n7;
        locals.var_exptempd_dn8 = assign96360_e148944_d_n8;
        locals.var_exptempd_dn9 = assign96360_e148944_d_n9;
        locals.var_exptempd_dn10 = assign96360_e148944_d_n10;
        locals.var_exptempd_dn13 = assign96360_e148944_d_n13;
        locals.var_exptempd_rv = 0.0;

        let (assign96370_e148954, assign96370_e148954_d_n0, assign96370_e148954_d_n2, assign96370_e148954_d_n4, assign96370_e148954_d_n5, assign96370_e148954_d_n6, assign96370_e148954_d_n7, assign96370_e148954_d_n8, assign96370_e148954_d_n9, assign96370_e148954_d_n10, assign96370_e148954_d_n13,) = {
    if ((locals.var_guard2233 != 0.0) && (locals.var_guard2236 != 0.0)) {
        let assign96370_e148951: f64 = (locals.var_uc_njd / locals.var_beta);
        let assign96370_e148952: f64 = (1.0 / assign96370_e148951);
        (assign96370_e148952, (-((-((locals.var_uc_njd * locals.var_beta_dn0) / (locals.var_beta * locals.var_beta))) / (assign96370_e148951 * assign96370_e148951))), (-((-((locals.var_uc_njd * locals.var_beta_dn2) / (locals.var_beta * locals.var_beta))) / (assign96370_e148951 * assign96370_e148951))), (-((-((locals.var_uc_njd * locals.var_beta_dn4) / (locals.var_beta * locals.var_beta))) / (assign96370_e148951 * assign96370_e148951))), (-((-((locals.var_uc_njd * locals.var_beta_dn5) / (locals.var_beta * locals.var_beta))) / (assign96370_e148951 * assign96370_e148951))), (-((-((locals.var_uc_njd * locals.var_beta_dn6) / (locals.var_beta * locals.var_beta))) / (assign96370_e148951 * assign96370_e148951))), (-((-((locals.var_uc_njd * locals.var_beta_dn7) / (locals.var_beta * locals.var_beta))) / (assign96370_e148951 * assign96370_e148951))), (-((-((locals.var_uc_njd * locals.var_beta_dn8) / (locals.var_beta * locals.var_beta))) / (assign96370_e148951 * assign96370_e148951))), (-((-((locals.var_uc_njd * locals.var_beta_dn9) / (locals.var_beta * locals.var_beta))) / (assign96370_e148951 * assign96370_e148951))), (-((-((locals.var_uc_njd * locals.var_beta_dn10) / (locals.var_beta * locals.var_beta))) / (assign96370_e148951 * assign96370_e148951))), (-((-((locals.var_uc_njd * locals.var_beta_dn13) / (locals.var_beta * locals.var_beta))) / (assign96370_e148951 * assign96370_e148951))),)
    } else {
        (locals.var_jd_nvtm_invd, locals.var_jd_nvtm_invd_dn0, locals.var_jd_nvtm_invd_dn2, locals.var_jd_nvtm_invd_dn4, locals.var_jd_nvtm_invd_dn5, locals.var_jd_nvtm_invd_dn6, locals.var_jd_nvtm_invd_dn7, locals.var_jd_nvtm_invd_dn8, locals.var_jd_nvtm_invd_dn9, locals.var_jd_nvtm_invd_dn10, locals.var_jd_nvtm_invd_dn13,)
    }
};
        locals.var_jd_nvtm_invd = assign96370_e148954;
        locals.var_jd_nvtm_invd_dn0 = assign96370_e148954_d_n0;
        locals.var_jd_nvtm_invd_dn2 = assign96370_e148954_d_n2;
        locals.var_jd_nvtm_invd_dn4 = assign96370_e148954_d_n4;
        locals.var_jd_nvtm_invd_dn5 = assign96370_e148954_d_n5;
        locals.var_jd_nvtm_invd_dn6 = assign96370_e148954_d_n6;
        locals.var_jd_nvtm_invd_dn7 = assign96370_e148954_d_n7;
        locals.var_jd_nvtm_invd_dn8 = assign96370_e148954_d_n8;
        locals.var_jd_nvtm_invd_dn9 = assign96370_e148954_d_n9;
        locals.var_jd_nvtm_invd_dn10 = assign96370_e148954_d_n10;
        locals.var_jd_nvtm_invd_dn13 = assign96370_e148954_d_n13;
        locals.var_jd_nvtm_invd_rv = 0.0;

        let (assign96380_e148963, assign96380_e148963_d_n0, assign96380_e148963_d_n2, assign96380_e148963_d_n4, assign96380_e148963_d_n5, assign96380_e148963_d_n6, assign96380_e148963_d_n7, assign96380_e148963_d_n8, assign96380_e148963_d_n9, assign96380_e148963_d_n10, assign96380_e148963_d_n13,) = {
    if ((locals.var_guard2233 != 0.0) && (locals.var_guard2236 != 0.0)) {
        let assign96380_e148960: f64 = (locals.var_vbdt * locals.var_jd_nvtm_invd);
        let assign96380_e148961: f64 = (assign96380_e148960).exp();
        (assign96380_e148961, (assign96380_e148961 * ((locals.var_vbdt_dn0 * locals.var_jd_nvtm_invd) + (locals.var_vbdt * locals.var_jd_nvtm_invd_dn0))), (assign96380_e148961 * ((locals.var_vbdt_dn2 * locals.var_jd_nvtm_invd) + (locals.var_vbdt * locals.var_jd_nvtm_invd_dn2))), (assign96380_e148961 * ((locals.var_vbdt_dn4 * locals.var_jd_nvtm_invd) + (locals.var_vbdt * locals.var_jd_nvtm_invd_dn4))), (assign96380_e148961 * ((locals.var_vbdt_dn5 * locals.var_jd_nvtm_invd) + (locals.var_vbdt * locals.var_jd_nvtm_invd_dn5))), (assign96380_e148961 * ((locals.var_vbdt_dn6 * locals.var_jd_nvtm_invd) + (locals.var_vbdt * locals.var_jd_nvtm_invd_dn6))), (assign96380_e148961 * ((locals.var_vbdt_dn7 * locals.var_jd_nvtm_invd) + (locals.var_vbdt * locals.var_jd_nvtm_invd_dn7))), (assign96380_e148961 * ((locals.var_vbdt_dn8 * locals.var_jd_nvtm_invd) + (locals.var_vbdt * locals.var_jd_nvtm_invd_dn8))), (assign96380_e148961 * ((locals.var_vbdt_dn9 * locals.var_jd_nvtm_invd) + (locals.var_vbdt * locals.var_jd_nvtm_invd_dn9))), (assign96380_e148961 * ((locals.var_vbdt_dn10 * locals.var_jd_nvtm_invd) + (locals.var_vbdt * locals.var_jd_nvtm_invd_dn10))), (assign96380_e148961 * ((locals.var_vbdt_dn13 * locals.var_jd_nvtm_invd) + (locals.var_vbdt * locals.var_jd_nvtm_invd_dn13))),)
    } else {
        (locals.var_jd_expcd, locals.var_jd_expcd_dn0, locals.var_jd_expcd_dn2, locals.var_jd_expcd_dn4, locals.var_jd_expcd_dn5, locals.var_jd_expcd_dn6, locals.var_jd_expcd_dn7, locals.var_jd_expcd_dn8, locals.var_jd_expcd_dn9, locals.var_jd_expcd_dn10, locals.var_jd_expcd_dn13,)
    }
};
        locals.var_jd_expcd = assign96380_e148963;
        locals.var_jd_expcd_dn0 = assign96380_e148963_d_n0;
        locals.var_jd_expcd_dn2 = assign96380_e148963_d_n2;
        locals.var_jd_expcd_dn4 = assign96380_e148963_d_n4;
        locals.var_jd_expcd_dn5 = assign96380_e148963_d_n5;
        locals.var_jd_expcd_dn6 = assign96380_e148963_d_n6;
        locals.var_jd_expcd_dn7 = assign96380_e148963_d_n7;
        locals.var_jd_expcd_dn8 = assign96380_e148963_d_n8;
        locals.var_jd_expcd_dn9 = assign96380_e148963_d_n9;
        locals.var_jd_expcd_dn10 = assign96380_e148963_d_n10;
        locals.var_jd_expcd_dn13 = assign96380_e148963_d_n13;
        locals.var_jd_expcd_rv = 0.0;

        let (assign96390_e148982, assign96390_e148982_d_n0, assign96390_e148982_d_n2, assign96390_e148982_d_n4, assign96390_e148982_d_n5, assign96390_e148982_d_n6, assign96390_e148982_d_n7, assign96390_e148982_d_n8, assign96390_e148982_d_n9, assign96390_e148982_d_n10, assign96390_e148982_d_n13,) = {
    if (locals.var_guard2233 != 0.0) {
        let assign96390_e148968: f64 = (locals.var_egtnom * locals.var_betatnom);
        let assign96390_e148971: f64 = (locals.var_eg * locals.var_beta);
        let assign96390_e148972: f64 = (assign96390_e148968 - assign96390_e148971);
        let assign96390_e148975: f64 = (p.p522 * locals.var_log_tratio);
        let assign96390_e148976: f64 = (assign96390_e148972 + assign96390_e148975);
        let assign96390_e148978: f64 = (assign96390_e148976 / locals.var_uc_njs);
        let assign96390_e148979: f64 = (assign96390_e148978).exp();
        let assign96390_e148980: f64 = (locals.var_uc_js0s * assign96390_e148979);
        (assign96390_e148980, (locals.var_uc_js0s * (assign96390_e148979 * (((-((locals.var_eg_dn0 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn0))) + (p.p522 * locals.var_log_tratio_dn0)) / locals.var_uc_njs))), (locals.var_uc_js0s * (assign96390_e148979 * (((-((locals.var_eg_dn2 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn2))) + (p.p522 * locals.var_log_tratio_dn2)) / locals.var_uc_njs))), (locals.var_uc_js0s * (assign96390_e148979 * (((-((locals.var_eg_dn4 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn4))) + (p.p522 * locals.var_log_tratio_dn4)) / locals.var_uc_njs))), (locals.var_uc_js0s * (assign96390_e148979 * (((-((locals.var_eg_dn5 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn5))) + (p.p522 * locals.var_log_tratio_dn5)) / locals.var_uc_njs))), (locals.var_uc_js0s * (assign96390_e148979 * (((-((locals.var_eg_dn6 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn6))) + (p.p522 * locals.var_log_tratio_dn6)) / locals.var_uc_njs))), (locals.var_uc_js0s * (assign96390_e148979 * (((-((locals.var_eg_dn7 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn7))) + (p.p522 * locals.var_log_tratio_dn7)) / locals.var_uc_njs))), (locals.var_uc_js0s * (assign96390_e148979 * (((-((locals.var_eg_dn8 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn8))) + (p.p522 * locals.var_log_tratio_dn8)) / locals.var_uc_njs))), (locals.var_uc_js0s * (assign96390_e148979 * (((-((locals.var_eg_dn9 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn9))) + (p.p522 * locals.var_log_tratio_dn9)) / locals.var_uc_njs))), (locals.var_uc_js0s * (assign96390_e148979 * (((-((locals.var_eg_dn10 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn10))) + (p.p522 * locals.var_log_tratio_dn10)) / locals.var_uc_njs))), (locals.var_uc_js0s * (assign96390_e148979 * (((-((locals.var_eg_dn13 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn13))) + (p.p522 * locals.var_log_tratio_dn13)) / locals.var_uc_njs))),)
    } else {
        (locals.var_js, locals.var_js_dn0, locals.var_js_dn2, locals.var_js_dn4, locals.var_js_dn5, locals.var_js_dn6, locals.var_js_dn7, locals.var_js_dn8, locals.var_js_dn9, locals.var_js_dn10, locals.var_js_dn13,)
    }
};
        locals.var_js = assign96390_e148982;
        locals.var_js_dn0 = assign96390_e148982_d_n0;
        locals.var_js_dn2 = assign96390_e148982_d_n2;
        locals.var_js_dn4 = assign96390_e148982_d_n4;
        locals.var_js_dn5 = assign96390_e148982_d_n5;
        locals.var_js_dn6 = assign96390_e148982_d_n6;
        locals.var_js_dn7 = assign96390_e148982_d_n7;
        locals.var_js_dn8 = assign96390_e148982_d_n8;
        locals.var_js_dn9 = assign96390_e148982_d_n9;
        locals.var_js_dn10 = assign96390_e148982_d_n10;
        locals.var_js_dn13 = assign96390_e148982_d_n13;
        locals.var_js_rv = 0.0;

        let (assign96400_e149001, assign96400_e149001_d_n0, assign96400_e149001_d_n2, assign96400_e149001_d_n4, assign96400_e149001_d_n5, assign96400_e149001_d_n6, assign96400_e149001_d_n7, assign96400_e149001_d_n8, assign96400_e149001_d_n9, assign96400_e149001_d_n10, assign96400_e149001_d_n13,) = {
    if (locals.var_guard2233 != 0.0) {
        let assign96400_e148987: f64 = (locals.var_egtnom * locals.var_betatnom);
        let assign96400_e148990: f64 = (locals.var_eg * locals.var_beta);
        let assign96400_e148991: f64 = (assign96400_e148987 - assign96400_e148990);
        let assign96400_e148994: f64 = (p.p522 * locals.var_log_tratio);
        let assign96400_e148995: f64 = (assign96400_e148991 + assign96400_e148994);
        let assign96400_e148997: f64 = (assign96400_e148995 / p.p520);
        let assign96400_e148998: f64 = (assign96400_e148997).exp();
        let assign96400_e148999: f64 = (locals.var_uc_js0sws * assign96400_e148998);
        (assign96400_e148999, (locals.var_uc_js0sws * (assign96400_e148998 * (((-((locals.var_eg_dn0 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn0))) + (p.p522 * locals.var_log_tratio_dn0)) / p.p520))), (locals.var_uc_js0sws * (assign96400_e148998 * (((-((locals.var_eg_dn2 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn2))) + (p.p522 * locals.var_log_tratio_dn2)) / p.p520))), (locals.var_uc_js0sws * (assign96400_e148998 * (((-((locals.var_eg_dn4 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn4))) + (p.p522 * locals.var_log_tratio_dn4)) / p.p520))), (locals.var_uc_js0sws * (assign96400_e148998 * (((-((locals.var_eg_dn5 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn5))) + (p.p522 * locals.var_log_tratio_dn5)) / p.p520))), (locals.var_uc_js0sws * (assign96400_e148998 * (((-((locals.var_eg_dn6 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn6))) + (p.p522 * locals.var_log_tratio_dn6)) / p.p520))), (locals.var_uc_js0sws * (assign96400_e148998 * (((-((locals.var_eg_dn7 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn7))) + (p.p522 * locals.var_log_tratio_dn7)) / p.p520))), (locals.var_uc_js0sws * (assign96400_e148998 * (((-((locals.var_eg_dn8 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn8))) + (p.p522 * locals.var_log_tratio_dn8)) / p.p520))), (locals.var_uc_js0sws * (assign96400_e148998 * (((-((locals.var_eg_dn9 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn9))) + (p.p522 * locals.var_log_tratio_dn9)) / p.p520))), (locals.var_uc_js0sws * (assign96400_e148998 * (((-((locals.var_eg_dn10 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn10))) + (p.p522 * locals.var_log_tratio_dn10)) / p.p520))), (locals.var_uc_js0sws * (assign96400_e148998 * (((-((locals.var_eg_dn13 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn13))) + (p.p522 * locals.var_log_tratio_dn13)) / p.p520))),)
    } else {
        (locals.var_jssw, locals.var_jssw_dn0, locals.var_jssw_dn2, locals.var_jssw_dn4, locals.var_jssw_dn5, locals.var_jssw_dn6, locals.var_jssw_dn7, locals.var_jssw_dn8, locals.var_jssw_dn9, locals.var_jssw_dn10, locals.var_jssw_dn13,)
    }
};
        locals.var_jssw = assign96400_e149001;
        locals.var_jssw_dn0 = assign96400_e149001_d_n0;
        locals.var_jssw_dn2 = assign96400_e149001_d_n2;
        locals.var_jssw_dn4 = assign96400_e149001_d_n4;
        locals.var_jssw_dn5 = assign96400_e149001_d_n5;
        locals.var_jssw_dn6 = assign96400_e149001_d_n6;
        locals.var_jssw_dn7 = assign96400_e149001_d_n7;
        locals.var_jssw_dn8 = assign96400_e149001_d_n8;
        locals.var_jssw_dn9 = assign96400_e149001_d_n9;
        locals.var_jssw_dn10 = assign96400_e149001_d_n10;
        locals.var_jssw_dn13 = assign96400_e149001_d_n13;
        locals.var_jssw_rv = 0.0;

        let (assign96410_e149020, assign96410_e149020_d_n0, assign96410_e149020_d_n2, assign96410_e149020_d_n4, assign96410_e149020_d_n5, assign96410_e149020_d_n6, assign96410_e149020_d_n7, assign96410_e149020_d_n8, assign96410_e149020_d_n9, assign96410_e149020_d_n10, assign96410_e149020_d_n13,) = {
    if (locals.var_guard2233 != 0.0) {
        let assign96410_e149006: f64 = (locals.var_egtnom * locals.var_betatnom);
        let assign96410_e149009: f64 = (locals.var_eg * locals.var_beta);
        let assign96410_e149010: f64 = (assign96410_e149006 - assign96410_e149009);
        let assign96410_e149013: f64 = (p.p522 * locals.var_log_tratio);
        let assign96410_e149014: f64 = (assign96410_e149010 + assign96410_e149013);
        let assign96410_e149016: f64 = (assign96410_e149014 / p.p521);
        let assign96410_e149017: f64 = (assign96410_e149016).exp();
        let assign96410_e149018: f64 = (p.p518 * assign96410_e149017);
        (assign96410_e149018, (p.p518 * (assign96410_e149017 * (((-((locals.var_eg_dn0 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn0))) + (p.p522 * locals.var_log_tratio_dn0)) / p.p521))), (p.p518 * (assign96410_e149017 * (((-((locals.var_eg_dn2 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn2))) + (p.p522 * locals.var_log_tratio_dn2)) / p.p521))), (p.p518 * (assign96410_e149017 * (((-((locals.var_eg_dn4 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn4))) + (p.p522 * locals.var_log_tratio_dn4)) / p.p521))), (p.p518 * (assign96410_e149017 * (((-((locals.var_eg_dn5 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn5))) + (p.p522 * locals.var_log_tratio_dn5)) / p.p521))), (p.p518 * (assign96410_e149017 * (((-((locals.var_eg_dn6 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn6))) + (p.p522 * locals.var_log_tratio_dn6)) / p.p521))), (p.p518 * (assign96410_e149017 * (((-((locals.var_eg_dn7 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn7))) + (p.p522 * locals.var_log_tratio_dn7)) / p.p521))), (p.p518 * (assign96410_e149017 * (((-((locals.var_eg_dn8 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn8))) + (p.p522 * locals.var_log_tratio_dn8)) / p.p521))), (p.p518 * (assign96410_e149017 * (((-((locals.var_eg_dn9 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn9))) + (p.p522 * locals.var_log_tratio_dn9)) / p.p521))), (p.p518 * (assign96410_e149017 * (((-((locals.var_eg_dn10 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn10))) + (p.p522 * locals.var_log_tratio_dn10)) / p.p521))), (p.p518 * (assign96410_e149017 * (((-((locals.var_eg_dn13 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn13))) + (p.p522 * locals.var_log_tratio_dn13)) / p.p521))),)
    } else {
        (locals.var_jsswg, locals.var_jsswg_dn0, locals.var_jsswg_dn2, locals.var_jsswg_dn4, locals.var_jsswg_dn5, locals.var_jsswg_dn6, locals.var_jsswg_dn7, locals.var_jsswg_dn8, locals.var_jsswg_dn9, locals.var_jsswg_dn10, locals.var_jsswg_dn13,)
    }
};
        locals.var_jsswg = assign96410_e149020;
        locals.var_jsswg_dn0 = assign96410_e149020_d_n0;
        locals.var_jsswg_dn2 = assign96410_e149020_d_n2;
        locals.var_jsswg_dn4 = assign96410_e149020_d_n4;
        locals.var_jsswg_dn5 = assign96410_e149020_d_n5;
        locals.var_jsswg_dn6 = assign96410_e149020_d_n6;
        locals.var_jsswg_dn7 = assign96410_e149020_d_n7;
        locals.var_jsswg_dn8 = assign96410_e149020_d_n8;
        locals.var_jsswg_dn9 = assign96410_e149020_d_n9;
        locals.var_jsswg_dn10 = assign96410_e149020_d_n10;
        locals.var_jsswg_dn13 = assign96410_e149020_d_n13;
        locals.var_jsswg_rv = 0.0;

        let (assign96420_e149039, assign96420_e149039_d_n0, assign96420_e149039_d_n2, assign96420_e149039_d_n4, assign96420_e149039_d_n5, assign96420_e149039_d_n6, assign96420_e149039_d_n7, assign96420_e149039_d_n8, assign96420_e149039_d_n9, assign96420_e149039_d_n10, assign96420_e149039_d_n13,) = {
    if (locals.var_guard2233 != 0.0) {
        let assign96420_e149025: f64 = (locals.var_egtnom * locals.var_betatnom);
        let assign96420_e149028: f64 = (locals.var_eg * locals.var_beta);
        let assign96420_e149029: f64 = (assign96420_e149025 - assign96420_e149028);
        let assign96420_e149032: f64 = (p.p532 * locals.var_log_tratio);
        let assign96420_e149033: f64 = (assign96420_e149029 + assign96420_e149032);
        let assign96420_e149035: f64 = (assign96420_e149033 / locals.var_uc_njs);
        let assign96420_e149036: f64 = (assign96420_e149035).exp();
        let assign96420_e149037: f64 = (locals.var_uc_js0s * assign96420_e149036);
        (assign96420_e149037, (locals.var_uc_js0s * (assign96420_e149036 * (((-((locals.var_eg_dn0 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn0))) + (p.p532 * locals.var_log_tratio_dn0)) / locals.var_uc_njs))), (locals.var_uc_js0s * (assign96420_e149036 * (((-((locals.var_eg_dn2 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn2))) + (p.p532 * locals.var_log_tratio_dn2)) / locals.var_uc_njs))), (locals.var_uc_js0s * (assign96420_e149036 * (((-((locals.var_eg_dn4 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn4))) + (p.p532 * locals.var_log_tratio_dn4)) / locals.var_uc_njs))), (locals.var_uc_js0s * (assign96420_e149036 * (((-((locals.var_eg_dn5 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn5))) + (p.p532 * locals.var_log_tratio_dn5)) / locals.var_uc_njs))), (locals.var_uc_js0s * (assign96420_e149036 * (((-((locals.var_eg_dn6 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn6))) + (p.p532 * locals.var_log_tratio_dn6)) / locals.var_uc_njs))), (locals.var_uc_js0s * (assign96420_e149036 * (((-((locals.var_eg_dn7 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn7))) + (p.p532 * locals.var_log_tratio_dn7)) / locals.var_uc_njs))), (locals.var_uc_js0s * (assign96420_e149036 * (((-((locals.var_eg_dn8 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn8))) + (p.p532 * locals.var_log_tratio_dn8)) / locals.var_uc_njs))), (locals.var_uc_js0s * (assign96420_e149036 * (((-((locals.var_eg_dn9 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn9))) + (p.p532 * locals.var_log_tratio_dn9)) / locals.var_uc_njs))), (locals.var_uc_js0s * (assign96420_e149036 * (((-((locals.var_eg_dn10 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn10))) + (p.p532 * locals.var_log_tratio_dn10)) / locals.var_uc_njs))), (locals.var_uc_js0s * (assign96420_e149036 * (((-((locals.var_eg_dn13 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn13))) + (p.p532 * locals.var_log_tratio_dn13)) / locals.var_uc_njs))),)
    } else {
        (locals.var_js2, locals.var_js2_dn0, locals.var_js2_dn2, locals.var_js2_dn4, locals.var_js2_dn5, locals.var_js2_dn6, locals.var_js2_dn7, locals.var_js2_dn8, locals.var_js2_dn9, locals.var_js2_dn10, locals.var_js2_dn13,)
    }
};
        locals.var_js2 = assign96420_e149039;
        locals.var_js2_dn0 = assign96420_e149039_d_n0;
        locals.var_js2_dn2 = assign96420_e149039_d_n2;
        locals.var_js2_dn4 = assign96420_e149039_d_n4;
        locals.var_js2_dn5 = assign96420_e149039_d_n5;
        locals.var_js2_dn6 = assign96420_e149039_d_n6;
        locals.var_js2_dn7 = assign96420_e149039_d_n7;
        locals.var_js2_dn8 = assign96420_e149039_d_n8;
        locals.var_js2_dn9 = assign96420_e149039_d_n9;
        locals.var_js2_dn10 = assign96420_e149039_d_n10;
        locals.var_js2_dn13 = assign96420_e149039_d_n13;
        locals.var_js2_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_362(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign96430_e149058, assign96430_e149058_d_n0, assign96430_e149058_d_n2, assign96430_e149058_d_n4, assign96430_e149058_d_n5, assign96430_e149058_d_n6, assign96430_e149058_d_n7, assign96430_e149058_d_n8, assign96430_e149058_d_n9, assign96430_e149058_d_n10, assign96430_e149058_d_n13,) = {
    if (locals.var_guard2233 != 0.0) {
        let assign96430_e149044: f64 = (locals.var_egtnom * locals.var_betatnom);
        let assign96430_e149047: f64 = (locals.var_eg * locals.var_beta);
        let assign96430_e149048: f64 = (assign96430_e149044 - assign96430_e149047);
        let assign96430_e149051: f64 = (p.p532 * locals.var_log_tratio);
        let assign96430_e149052: f64 = (assign96430_e149048 + assign96430_e149051);
        let assign96430_e149054: f64 = (assign96430_e149052 / p.p520);
        let assign96430_e149055: f64 = (assign96430_e149054).exp();
        let assign96430_e149056: f64 = (locals.var_uc_js0sws * assign96430_e149055);
        (assign96430_e149056, (locals.var_uc_js0sws * (assign96430_e149055 * (((-((locals.var_eg_dn0 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn0))) + (p.p532 * locals.var_log_tratio_dn0)) / p.p520))), (locals.var_uc_js0sws * (assign96430_e149055 * (((-((locals.var_eg_dn2 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn2))) + (p.p532 * locals.var_log_tratio_dn2)) / p.p520))), (locals.var_uc_js0sws * (assign96430_e149055 * (((-((locals.var_eg_dn4 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn4))) + (p.p532 * locals.var_log_tratio_dn4)) / p.p520))), (locals.var_uc_js0sws * (assign96430_e149055 * (((-((locals.var_eg_dn5 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn5))) + (p.p532 * locals.var_log_tratio_dn5)) / p.p520))), (locals.var_uc_js0sws * (assign96430_e149055 * (((-((locals.var_eg_dn6 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn6))) + (p.p532 * locals.var_log_tratio_dn6)) / p.p520))), (locals.var_uc_js0sws * (assign96430_e149055 * (((-((locals.var_eg_dn7 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn7))) + (p.p532 * locals.var_log_tratio_dn7)) / p.p520))), (locals.var_uc_js0sws * (assign96430_e149055 * (((-((locals.var_eg_dn8 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn8))) + (p.p532 * locals.var_log_tratio_dn8)) / p.p520))), (locals.var_uc_js0sws * (assign96430_e149055 * (((-((locals.var_eg_dn9 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn9))) + (p.p532 * locals.var_log_tratio_dn9)) / p.p520))), (locals.var_uc_js0sws * (assign96430_e149055 * (((-((locals.var_eg_dn10 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn10))) + (p.p532 * locals.var_log_tratio_dn10)) / p.p520))), (locals.var_uc_js0sws * (assign96430_e149055 * (((-((locals.var_eg_dn13 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn13))) + (p.p532 * locals.var_log_tratio_dn13)) / p.p520))),)
    } else {
        (locals.var_jssw2, locals.var_jssw2_dn0, locals.var_jssw2_dn2, locals.var_jssw2_dn4, locals.var_jssw2_dn5, locals.var_jssw2_dn6, locals.var_jssw2_dn7, locals.var_jssw2_dn8, locals.var_jssw2_dn9, locals.var_jssw2_dn10, locals.var_jssw2_dn13,)
    }
};
        locals.var_jssw2 = assign96430_e149058;
        locals.var_jssw2_dn0 = assign96430_e149058_d_n0;
        locals.var_jssw2_dn2 = assign96430_e149058_d_n2;
        locals.var_jssw2_dn4 = assign96430_e149058_d_n4;
        locals.var_jssw2_dn5 = assign96430_e149058_d_n5;
        locals.var_jssw2_dn6 = assign96430_e149058_d_n6;
        locals.var_jssw2_dn7 = assign96430_e149058_d_n7;
        locals.var_jssw2_dn8 = assign96430_e149058_d_n8;
        locals.var_jssw2_dn9 = assign96430_e149058_d_n9;
        locals.var_jssw2_dn10 = assign96430_e149058_d_n10;
        locals.var_jssw2_dn13 = assign96430_e149058_d_n13;
        locals.var_jssw2_rv = 0.0;

        let (assign96440_e149077, assign96440_e149077_d_n0, assign96440_e149077_d_n2, assign96440_e149077_d_n4, assign96440_e149077_d_n5, assign96440_e149077_d_n6, assign96440_e149077_d_n7, assign96440_e149077_d_n8, assign96440_e149077_d_n9, assign96440_e149077_d_n10, assign96440_e149077_d_n13,) = {
    if (locals.var_guard2233 != 0.0) {
        let assign96440_e149063: f64 = (locals.var_egtnom * locals.var_betatnom);
        let assign96440_e149066: f64 = (locals.var_eg * locals.var_beta);
        let assign96440_e149067: f64 = (assign96440_e149063 - assign96440_e149066);
        let assign96440_e149070: f64 = (p.p532 * locals.var_log_tratio);
        let assign96440_e149071: f64 = (assign96440_e149067 + assign96440_e149070);
        let assign96440_e149073: f64 = (assign96440_e149071 / p.p521);
        let assign96440_e149074: f64 = (assign96440_e149073).exp();
        let assign96440_e149075: f64 = (p.p518 * assign96440_e149074);
        (assign96440_e149075, (p.p518 * (assign96440_e149074 * (((-((locals.var_eg_dn0 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn0))) + (p.p532 * locals.var_log_tratio_dn0)) / p.p521))), (p.p518 * (assign96440_e149074 * (((-((locals.var_eg_dn2 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn2))) + (p.p532 * locals.var_log_tratio_dn2)) / p.p521))), (p.p518 * (assign96440_e149074 * (((-((locals.var_eg_dn4 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn4))) + (p.p532 * locals.var_log_tratio_dn4)) / p.p521))), (p.p518 * (assign96440_e149074 * (((-((locals.var_eg_dn5 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn5))) + (p.p532 * locals.var_log_tratio_dn5)) / p.p521))), (p.p518 * (assign96440_e149074 * (((-((locals.var_eg_dn6 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn6))) + (p.p532 * locals.var_log_tratio_dn6)) / p.p521))), (p.p518 * (assign96440_e149074 * (((-((locals.var_eg_dn7 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn7))) + (p.p532 * locals.var_log_tratio_dn7)) / p.p521))), (p.p518 * (assign96440_e149074 * (((-((locals.var_eg_dn8 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn8))) + (p.p532 * locals.var_log_tratio_dn8)) / p.p521))), (p.p518 * (assign96440_e149074 * (((-((locals.var_eg_dn9 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn9))) + (p.p532 * locals.var_log_tratio_dn9)) / p.p521))), (p.p518 * (assign96440_e149074 * (((-((locals.var_eg_dn10 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn10))) + (p.p532 * locals.var_log_tratio_dn10)) / p.p521))), (p.p518 * (assign96440_e149074 * (((-((locals.var_eg_dn13 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn13))) + (p.p532 * locals.var_log_tratio_dn13)) / p.p521))),)
    } else {
        (locals.var_jsswg2, locals.var_jsswg2_dn0, locals.var_jsswg2_dn2, locals.var_jsswg2_dn4, locals.var_jsswg2_dn5, locals.var_jsswg2_dn6, locals.var_jsswg2_dn7, locals.var_jsswg2_dn8, locals.var_jsswg2_dn9, locals.var_jsswg2_dn10, locals.var_jsswg2_dn13,)
    }
};
        locals.var_jsswg2 = assign96440_e149077;
        locals.var_jsswg2_dn0 = assign96440_e149077_d_n0;
        locals.var_jsswg2_dn2 = assign96440_e149077_d_n2;
        locals.var_jsswg2_dn4 = assign96440_e149077_d_n4;
        locals.var_jsswg2_dn5 = assign96440_e149077_d_n5;
        locals.var_jsswg2_dn6 = assign96440_e149077_d_n6;
        locals.var_jsswg2_dn7 = assign96440_e149077_d_n7;
        locals.var_jsswg2_dn8 = assign96440_e149077_d_n8;
        locals.var_jsswg2_dn9 = assign96440_e149077_d_n9;
        locals.var_jsswg2_dn10 = assign96440_e149077_d_n10;
        locals.var_jsswg2_dn13 = assign96440_e149077_d_n13;
        locals.var_jsswg2_rv = 0.0;

        let assign96450_e149080: f64 = if p.p48 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2237 = assign96450_e149080;
        locals.var_guard2237_rv = 0.0;

        let assign96460_e149083: f64 = if p.p16 > locals.var_weff_nf { 1.0 } else { 0.0 };
        locals.var_guard2238 = assign96460_e149083;
        locals.var_guard2238_rv = 0.0;

        let (assign96470_e149093, assign96470_e149093_d_n0, assign96470_e149093_d_n2, assign96470_e149093_d_n4, assign96470_e149093_d_n5, assign96470_e149093_d_n6, assign96470_e149093_d_n7, assign96470_e149093_d_n8, assign96470_e149093_d_n9, assign96470_e149093_d_n10, assign96470_e149093_d_n13,) = {
    if (((locals.var_guard2233 != 0.0) && (locals.var_guard2237 != 0.0)) && (locals.var_guard2238 != 0.0)) {
        let assign96470_e149091: f64 = (p.p14 * locals.var_js);
        (assign96470_e149091, (p.p14 * locals.var_js_dn0), (p.p14 * locals.var_js_dn2), (p.p14 * locals.var_js_dn4), (p.p14 * locals.var_js_dn5), (p.p14 * locals.var_js_dn6), (p.p14 * locals.var_js_dn7), (p.p14 * locals.var_js_dn8), (p.p14 * locals.var_js_dn9), (p.p14 * locals.var_js_dn10), (p.p14 * locals.var_js_dn13),)
    } else {
        (locals.var_isbs_btm, locals.var_isbs_btm_dn0, locals.var_isbs_btm_dn2, locals.var_isbs_btm_dn4, locals.var_isbs_btm_dn5, locals.var_isbs_btm_dn6, locals.var_isbs_btm_dn7, locals.var_isbs_btm_dn8, locals.var_isbs_btm_dn9, locals.var_isbs_btm_dn10, locals.var_isbs_btm_dn13,)
    }
};
        locals.var_isbs_btm = assign96470_e149093;
        locals.var_isbs_btm_dn0 = assign96470_e149093_d_n0;
        locals.var_isbs_btm_dn2 = assign96470_e149093_d_n2;
        locals.var_isbs_btm_dn4 = assign96470_e149093_d_n4;
        locals.var_isbs_btm_dn5 = assign96470_e149093_d_n5;
        locals.var_isbs_btm_dn6 = assign96470_e149093_d_n6;
        locals.var_isbs_btm_dn7 = assign96470_e149093_d_n7;
        locals.var_isbs_btm_dn8 = assign96470_e149093_d_n8;
        locals.var_isbs_btm_dn9 = assign96470_e149093_d_n9;
        locals.var_isbs_btm_dn10 = assign96470_e149093_d_n10;
        locals.var_isbs_btm_dn13 = assign96470_e149093_d_n13;
        locals.var_isbs_btm_rv = 0.0;

        let (assign96480_e149103, assign96480_e149103_d_n0, assign96480_e149103_d_n2, assign96480_e149103_d_n4, assign96480_e149103_d_n5, assign96480_e149103_d_n6, assign96480_e149103_d_n7, assign96480_e149103_d_n8, assign96480_e149103_d_n9, assign96480_e149103_d_n10, assign96480_e149103_d_n13,) = {
    if (((locals.var_guard2233 != 0.0) && (locals.var_guard2237 != 0.0)) && (locals.var_guard2238 != 0.0)) {
        let assign96480_e149101: f64 = (p.p14 * locals.var_js2);
        (assign96480_e149101, (p.p14 * locals.var_js2_dn0), (p.p14 * locals.var_js2_dn2), (p.p14 * locals.var_js2_dn4), (p.p14 * locals.var_js2_dn5), (p.p14 * locals.var_js2_dn6), (p.p14 * locals.var_js2_dn7), (p.p14 * locals.var_js2_dn8), (p.p14 * locals.var_js2_dn9), (p.p14 * locals.var_js2_dn10), (p.p14 * locals.var_js2_dn13),)
    } else {
        (locals.var_isbs2_btm, locals.var_isbs2_btm_dn0, locals.var_isbs2_btm_dn2, locals.var_isbs2_btm_dn4, locals.var_isbs2_btm_dn5, locals.var_isbs2_btm_dn6, locals.var_isbs2_btm_dn7, locals.var_isbs2_btm_dn8, locals.var_isbs2_btm_dn9, locals.var_isbs2_btm_dn10, locals.var_isbs2_btm_dn13,)
    }
};
        locals.var_isbs2_btm = assign96480_e149103;
        locals.var_isbs2_btm_dn0 = assign96480_e149103_d_n0;
        locals.var_isbs2_btm_dn2 = assign96480_e149103_d_n2;
        locals.var_isbs2_btm_dn4 = assign96480_e149103_d_n4;
        locals.var_isbs2_btm_dn5 = assign96480_e149103_d_n5;
        locals.var_isbs2_btm_dn6 = assign96480_e149103_d_n6;
        locals.var_isbs2_btm_dn7 = assign96480_e149103_d_n7;
        locals.var_isbs2_btm_dn8 = assign96480_e149103_d_n8;
        locals.var_isbs2_btm_dn9 = assign96480_e149103_d_n9;
        locals.var_isbs2_btm_dn10 = assign96480_e149103_d_n10;
        locals.var_isbs2_btm_dn13 = assign96480_e149103_d_n13;
        locals.var_isbs2_btm_rv = 0.0;

        let (assign96490_e149115, assign96490_e149115_d_n0, assign96490_e149115_d_n2, assign96490_e149115_d_n4, assign96490_e149115_d_n5, assign96490_e149115_d_n6, assign96490_e149115_d_n7, assign96490_e149115_d_n8, assign96490_e149115_d_n9, assign96490_e149115_d_n10, assign96490_e149115_d_n13,) = {
    if (((locals.var_guard2233 != 0.0) && (locals.var_guard2237 != 0.0)) && (locals.var_guard2238 != 0.0)) {
        let assign96490_e149111: f64 = (p.p16 - locals.var_weff_nf);
        let assign96490_e149113: f64 = (assign96490_e149111 * locals.var_jssw);
        (assign96490_e149113, (assign96490_e149111 * locals.var_jssw_dn0), (assign96490_e149111 * locals.var_jssw_dn2), (assign96490_e149111 * locals.var_jssw_dn4), (assign96490_e149111 * locals.var_jssw_dn5), (assign96490_e149111 * locals.var_jssw_dn6), (assign96490_e149111 * locals.var_jssw_dn7), (assign96490_e149111 * locals.var_jssw_dn8), (assign96490_e149111 * locals.var_jssw_dn9), (assign96490_e149111 * locals.var_jssw_dn10), (assign96490_e149111 * locals.var_jssw_dn13),)
    } else {
        (locals.var_isbs_sws, locals.var_isbs_sws_dn0, locals.var_isbs_sws_dn2, locals.var_isbs_sws_dn4, locals.var_isbs_sws_dn5, locals.var_isbs_sws_dn6, locals.var_isbs_sws_dn7, locals.var_isbs_sws_dn8, locals.var_isbs_sws_dn9, locals.var_isbs_sws_dn10, locals.var_isbs_sws_dn13,)
    }
};
        locals.var_isbs_sws = assign96490_e149115;
        locals.var_isbs_sws_dn0 = assign96490_e149115_d_n0;
        locals.var_isbs_sws_dn2 = assign96490_e149115_d_n2;
        locals.var_isbs_sws_dn4 = assign96490_e149115_d_n4;
        locals.var_isbs_sws_dn5 = assign96490_e149115_d_n5;
        locals.var_isbs_sws_dn6 = assign96490_e149115_d_n6;
        locals.var_isbs_sws_dn7 = assign96490_e149115_d_n7;
        locals.var_isbs_sws_dn8 = assign96490_e149115_d_n8;
        locals.var_isbs_sws_dn9 = assign96490_e149115_d_n9;
        locals.var_isbs_sws_dn10 = assign96490_e149115_d_n10;
        locals.var_isbs_sws_dn13 = assign96490_e149115_d_n13;
        locals.var_isbs_sws_rv = 0.0;

        let (assign96500_e149127, assign96500_e149127_d_n0, assign96500_e149127_d_n2, assign96500_e149127_d_n4, assign96500_e149127_d_n5, assign96500_e149127_d_n6, assign96500_e149127_d_n7, assign96500_e149127_d_n8, assign96500_e149127_d_n9, assign96500_e149127_d_n10, assign96500_e149127_d_n13,) = {
    if (((locals.var_guard2233 != 0.0) && (locals.var_guard2237 != 0.0)) && (locals.var_guard2238 != 0.0)) {
        let assign96500_e149123: f64 = (p.p16 - locals.var_weff_nf);
        let assign96500_e149125: f64 = (assign96500_e149123 * locals.var_jssw2);
        (assign96500_e149125, (assign96500_e149123 * locals.var_jssw2_dn0), (assign96500_e149123 * locals.var_jssw2_dn2), (assign96500_e149123 * locals.var_jssw2_dn4), (assign96500_e149123 * locals.var_jssw2_dn5), (assign96500_e149123 * locals.var_jssw2_dn6), (assign96500_e149123 * locals.var_jssw2_dn7), (assign96500_e149123 * locals.var_jssw2_dn8), (assign96500_e149123 * locals.var_jssw2_dn9), (assign96500_e149123 * locals.var_jssw2_dn10), (assign96500_e149123 * locals.var_jssw2_dn13),)
    } else {
        (locals.var_isbs2_sws, locals.var_isbs2_sws_dn0, locals.var_isbs2_sws_dn2, locals.var_isbs2_sws_dn4, locals.var_isbs2_sws_dn5, locals.var_isbs2_sws_dn6, locals.var_isbs2_sws_dn7, locals.var_isbs2_sws_dn8, locals.var_isbs2_sws_dn9, locals.var_isbs2_sws_dn10, locals.var_isbs2_sws_dn13,)
    }
};
        locals.var_isbs2_sws = assign96500_e149127;
        locals.var_isbs2_sws_dn0 = assign96500_e149127_d_n0;
        locals.var_isbs2_sws_dn2 = assign96500_e149127_d_n2;
        locals.var_isbs2_sws_dn4 = assign96500_e149127_d_n4;
        locals.var_isbs2_sws_dn5 = assign96500_e149127_d_n5;
        locals.var_isbs2_sws_dn6 = assign96500_e149127_d_n6;
        locals.var_isbs2_sws_dn7 = assign96500_e149127_d_n7;
        locals.var_isbs2_sws_dn8 = assign96500_e149127_d_n8;
        locals.var_isbs2_sws_dn9 = assign96500_e149127_d_n9;
        locals.var_isbs2_sws_dn10 = assign96500_e149127_d_n10;
        locals.var_isbs2_sws_dn13 = assign96500_e149127_d_n13;
        locals.var_isbs2_sws_rv = 0.0;

        let (assign96510_e149137, assign96510_e149137_d_n0, assign96510_e149137_d_n2, assign96510_e149137_d_n4, assign96510_e149137_d_n5, assign96510_e149137_d_n6, assign96510_e149137_d_n7, assign96510_e149137_d_n8, assign96510_e149137_d_n9, assign96510_e149137_d_n10, assign96510_e149137_d_n13,) = {
    if (((locals.var_guard2233 != 0.0) && (locals.var_guard2237 != 0.0)) && (locals.var_guard2238 != 0.0)) {
        let assign96510_e149135: f64 = (locals.var_weff_nf * locals.var_jsswg);
        (assign96510_e149135, (locals.var_weff_nf * locals.var_jsswg_dn0), (locals.var_weff_nf * locals.var_jsswg_dn2), (locals.var_weff_nf * locals.var_jsswg_dn4), (locals.var_weff_nf * locals.var_jsswg_dn5), (locals.var_weff_nf * locals.var_jsswg_dn6), (locals.var_weff_nf * locals.var_jsswg_dn7), (locals.var_weff_nf * locals.var_jsswg_dn8), (locals.var_weff_nf * locals.var_jsswg_dn9), (locals.var_weff_nf * locals.var_jsswg_dn10), (locals.var_weff_nf * locals.var_jsswg_dn13),)
    } else {
        (locals.var_isbs_swg, locals.var_isbs_swg_dn0, locals.var_isbs_swg_dn2, locals.var_isbs_swg_dn4, locals.var_isbs_swg_dn5, locals.var_isbs_swg_dn6, locals.var_isbs_swg_dn7, locals.var_isbs_swg_dn8, locals.var_isbs_swg_dn9, locals.var_isbs_swg_dn10, locals.var_isbs_swg_dn13,)
    }
};
        locals.var_isbs_swg = assign96510_e149137;
        locals.var_isbs_swg_dn0 = assign96510_e149137_d_n0;
        locals.var_isbs_swg_dn2 = assign96510_e149137_d_n2;
        locals.var_isbs_swg_dn4 = assign96510_e149137_d_n4;
        locals.var_isbs_swg_dn5 = assign96510_e149137_d_n5;
        locals.var_isbs_swg_dn6 = assign96510_e149137_d_n6;
        locals.var_isbs_swg_dn7 = assign96510_e149137_d_n7;
        locals.var_isbs_swg_dn8 = assign96510_e149137_d_n8;
        locals.var_isbs_swg_dn9 = assign96510_e149137_d_n9;
        locals.var_isbs_swg_dn10 = assign96510_e149137_d_n10;
        locals.var_isbs_swg_dn13 = assign96510_e149137_d_n13;
        locals.var_isbs_swg_rv = 0.0;

        let (assign96520_e149147, assign96520_e149147_d_n0, assign96520_e149147_d_n2, assign96520_e149147_d_n4, assign96520_e149147_d_n5, assign96520_e149147_d_n6, assign96520_e149147_d_n7, assign96520_e149147_d_n8, assign96520_e149147_d_n9, assign96520_e149147_d_n10, assign96520_e149147_d_n13,) = {
    if (((locals.var_guard2233 != 0.0) && (locals.var_guard2237 != 0.0)) && (locals.var_guard2238 != 0.0)) {
        let assign96520_e149145: f64 = (locals.var_weff_nf * locals.var_jsswg2);
        (assign96520_e149145, (locals.var_weff_nf * locals.var_jsswg2_dn0), (locals.var_weff_nf * locals.var_jsswg2_dn2), (locals.var_weff_nf * locals.var_jsswg2_dn4), (locals.var_weff_nf * locals.var_jsswg2_dn5), (locals.var_weff_nf * locals.var_jsswg2_dn6), (locals.var_weff_nf * locals.var_jsswg2_dn7), (locals.var_weff_nf * locals.var_jsswg2_dn8), (locals.var_weff_nf * locals.var_jsswg2_dn9), (locals.var_weff_nf * locals.var_jsswg2_dn10), (locals.var_weff_nf * locals.var_jsswg2_dn13),)
    } else {
        (locals.var_isbs2_swg, locals.var_isbs2_swg_dn0, locals.var_isbs2_swg_dn2, locals.var_isbs2_swg_dn4, locals.var_isbs2_swg_dn5, locals.var_isbs2_swg_dn6, locals.var_isbs2_swg_dn7, locals.var_isbs2_swg_dn8, locals.var_isbs2_swg_dn9, locals.var_isbs2_swg_dn10, locals.var_isbs2_swg_dn13,)
    }
};
        locals.var_isbs2_swg = assign96520_e149147;
        locals.var_isbs2_swg_dn0 = assign96520_e149147_d_n0;
        locals.var_isbs2_swg_dn2 = assign96520_e149147_d_n2;
        locals.var_isbs2_swg_dn4 = assign96520_e149147_d_n4;
        locals.var_isbs2_swg_dn5 = assign96520_e149147_d_n5;
        locals.var_isbs2_swg_dn6 = assign96520_e149147_d_n6;
        locals.var_isbs2_swg_dn7 = assign96520_e149147_d_n7;
        locals.var_isbs2_swg_dn8 = assign96520_e149147_d_n8;
        locals.var_isbs2_swg_dn9 = assign96520_e149147_d_n9;
        locals.var_isbs2_swg_dn10 = assign96520_e149147_d_n10;
        locals.var_isbs2_swg_dn13 = assign96520_e149147_d_n13;
        locals.var_isbs2_swg_rv = 0.0;

        let (assign96530_e149158, assign96530_e149158_d_n0, assign96530_e149158_d_n2, assign96530_e149158_d_n4, assign96530_e149158_d_n5, assign96530_e149158_d_n6, assign96530_e149158_d_n7, assign96530_e149158_d_n8, assign96530_e149158_d_n9, assign96530_e149158_d_n10, assign96530_e149158_d_n13,) = {
    if (((locals.var_guard2233 != 0.0) && (locals.var_guard2237 != 0.0)) && (locals.var_guard2238 == 0.0)) {
        let assign96530_e149156: f64 = (p.p14 * locals.var_js);
        (assign96530_e149156, (p.p14 * locals.var_js_dn0), (p.p14 * locals.var_js_dn2), (p.p14 * locals.var_js_dn4), (p.p14 * locals.var_js_dn5), (p.p14 * locals.var_js_dn6), (p.p14 * locals.var_js_dn7), (p.p14 * locals.var_js_dn8), (p.p14 * locals.var_js_dn9), (p.p14 * locals.var_js_dn10), (p.p14 * locals.var_js_dn13),)
    } else {
        (locals.var_isbs_btm, locals.var_isbs_btm_dn0, locals.var_isbs_btm_dn2, locals.var_isbs_btm_dn4, locals.var_isbs_btm_dn5, locals.var_isbs_btm_dn6, locals.var_isbs_btm_dn7, locals.var_isbs_btm_dn8, locals.var_isbs_btm_dn9, locals.var_isbs_btm_dn10, locals.var_isbs_btm_dn13,)
    }
};
        locals.var_isbs_btm = assign96530_e149158;
        locals.var_isbs_btm_dn0 = assign96530_e149158_d_n0;
        locals.var_isbs_btm_dn2 = assign96530_e149158_d_n2;
        locals.var_isbs_btm_dn4 = assign96530_e149158_d_n4;
        locals.var_isbs_btm_dn5 = assign96530_e149158_d_n5;
        locals.var_isbs_btm_dn6 = assign96530_e149158_d_n6;
        locals.var_isbs_btm_dn7 = assign96530_e149158_d_n7;
        locals.var_isbs_btm_dn8 = assign96530_e149158_d_n8;
        locals.var_isbs_btm_dn9 = assign96530_e149158_d_n9;
        locals.var_isbs_btm_dn10 = assign96530_e149158_d_n10;
        locals.var_isbs_btm_dn13 = assign96530_e149158_d_n13;
        locals.var_isbs_btm_rv = 0.0;

        let (assign96540_e149169, assign96540_e149169_d_n0, assign96540_e149169_d_n2, assign96540_e149169_d_n4, assign96540_e149169_d_n5, assign96540_e149169_d_n6, assign96540_e149169_d_n7, assign96540_e149169_d_n8, assign96540_e149169_d_n9, assign96540_e149169_d_n10, assign96540_e149169_d_n13,) = {
    if (((locals.var_guard2233 != 0.0) && (locals.var_guard2237 != 0.0)) && (locals.var_guard2238 == 0.0)) {
        let assign96540_e149167: f64 = (p.p14 * locals.var_js2);
        (assign96540_e149167, (p.p14 * locals.var_js2_dn0), (p.p14 * locals.var_js2_dn2), (p.p14 * locals.var_js2_dn4), (p.p14 * locals.var_js2_dn5), (p.p14 * locals.var_js2_dn6), (p.p14 * locals.var_js2_dn7), (p.p14 * locals.var_js2_dn8), (p.p14 * locals.var_js2_dn9), (p.p14 * locals.var_js2_dn10), (p.p14 * locals.var_js2_dn13),)
    } else {
        (locals.var_isbs2_btm, locals.var_isbs2_btm_dn0, locals.var_isbs2_btm_dn2, locals.var_isbs2_btm_dn4, locals.var_isbs2_btm_dn5, locals.var_isbs2_btm_dn6, locals.var_isbs2_btm_dn7, locals.var_isbs2_btm_dn8, locals.var_isbs2_btm_dn9, locals.var_isbs2_btm_dn10, locals.var_isbs2_btm_dn13,)
    }
};
        locals.var_isbs2_btm = assign96540_e149169;
        locals.var_isbs2_btm_dn0 = assign96540_e149169_d_n0;
        locals.var_isbs2_btm_dn2 = assign96540_e149169_d_n2;
        locals.var_isbs2_btm_dn4 = assign96540_e149169_d_n4;
        locals.var_isbs2_btm_dn5 = assign96540_e149169_d_n5;
        locals.var_isbs2_btm_dn6 = assign96540_e149169_d_n6;
        locals.var_isbs2_btm_dn7 = assign96540_e149169_d_n7;
        locals.var_isbs2_btm_dn8 = assign96540_e149169_d_n8;
        locals.var_isbs2_btm_dn9 = assign96540_e149169_d_n9;
        locals.var_isbs2_btm_dn10 = assign96540_e149169_d_n10;
        locals.var_isbs2_btm_dn13 = assign96540_e149169_d_n13;
        locals.var_isbs2_btm_rv = 0.0;

        let (assign96550_e149178, assign96550_e149178_d_n0, assign96550_e149178_d_n2, assign96550_e149178_d_n4, assign96550_e149178_d_n5, assign96550_e149178_d_n6, assign96550_e149178_d_n7, assign96550_e149178_d_n8, assign96550_e149178_d_n9, assign96550_e149178_d_n10, assign96550_e149178_d_n13,) = {
    if (((locals.var_guard2233 != 0.0) && (locals.var_guard2237 != 0.0)) && (locals.var_guard2238 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_isbs_sws, locals.var_isbs_sws_dn0, locals.var_isbs_sws_dn2, locals.var_isbs_sws_dn4, locals.var_isbs_sws_dn5, locals.var_isbs_sws_dn6, locals.var_isbs_sws_dn7, locals.var_isbs_sws_dn8, locals.var_isbs_sws_dn9, locals.var_isbs_sws_dn10, locals.var_isbs_sws_dn13,)
    }
};
        locals.var_isbs_sws = assign96550_e149178;
        locals.var_isbs_sws_dn0 = assign96550_e149178_d_n0;
        locals.var_isbs_sws_dn2 = assign96550_e149178_d_n2;
        locals.var_isbs_sws_dn4 = assign96550_e149178_d_n4;
        locals.var_isbs_sws_dn5 = assign96550_e149178_d_n5;
        locals.var_isbs_sws_dn6 = assign96550_e149178_d_n6;
        locals.var_isbs_sws_dn7 = assign96550_e149178_d_n7;
        locals.var_isbs_sws_dn8 = assign96550_e149178_d_n8;
        locals.var_isbs_sws_dn9 = assign96550_e149178_d_n9;
        locals.var_isbs_sws_dn10 = assign96550_e149178_d_n10;
        locals.var_isbs_sws_dn13 = assign96550_e149178_d_n13;
        locals.var_isbs_sws_rv = 0.0;

        let (assign96560_e149187, assign96560_e149187_d_n0, assign96560_e149187_d_n2, assign96560_e149187_d_n4, assign96560_e149187_d_n5, assign96560_e149187_d_n6, assign96560_e149187_d_n7, assign96560_e149187_d_n8, assign96560_e149187_d_n9, assign96560_e149187_d_n10, assign96560_e149187_d_n13,) = {
    if (((locals.var_guard2233 != 0.0) && (locals.var_guard2237 != 0.0)) && (locals.var_guard2238 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_isbs2_sws, locals.var_isbs2_sws_dn0, locals.var_isbs2_sws_dn2, locals.var_isbs2_sws_dn4, locals.var_isbs2_sws_dn5, locals.var_isbs2_sws_dn6, locals.var_isbs2_sws_dn7, locals.var_isbs2_sws_dn8, locals.var_isbs2_sws_dn9, locals.var_isbs2_sws_dn10, locals.var_isbs2_sws_dn13,)
    }
};
        locals.var_isbs2_sws = assign96560_e149187;
        locals.var_isbs2_sws_dn0 = assign96560_e149187_d_n0;
        locals.var_isbs2_sws_dn2 = assign96560_e149187_d_n2;
        locals.var_isbs2_sws_dn4 = assign96560_e149187_d_n4;
        locals.var_isbs2_sws_dn5 = assign96560_e149187_d_n5;
        locals.var_isbs2_sws_dn6 = assign96560_e149187_d_n6;
        locals.var_isbs2_sws_dn7 = assign96560_e149187_d_n7;
        locals.var_isbs2_sws_dn8 = assign96560_e149187_d_n8;
        locals.var_isbs2_sws_dn9 = assign96560_e149187_d_n9;
        locals.var_isbs2_sws_dn10 = assign96560_e149187_d_n10;
        locals.var_isbs2_sws_dn13 = assign96560_e149187_d_n13;
        locals.var_isbs2_sws_rv = 0.0;

        let (assign96570_e149198, assign96570_e149198_d_n0, assign96570_e149198_d_n2, assign96570_e149198_d_n4, assign96570_e149198_d_n5, assign96570_e149198_d_n6, assign96570_e149198_d_n7, assign96570_e149198_d_n8, assign96570_e149198_d_n9, assign96570_e149198_d_n10, assign96570_e149198_d_n13,) = {
    if (((locals.var_guard2233 != 0.0) && (locals.var_guard2237 != 0.0)) && (locals.var_guard2238 == 0.0)) {
        let assign96570_e149196: f64 = (p.p16 * locals.var_jsswg);
        (assign96570_e149196, (p.p16 * locals.var_jsswg_dn0), (p.p16 * locals.var_jsswg_dn2), (p.p16 * locals.var_jsswg_dn4), (p.p16 * locals.var_jsswg_dn5), (p.p16 * locals.var_jsswg_dn6), (p.p16 * locals.var_jsswg_dn7), (p.p16 * locals.var_jsswg_dn8), (p.p16 * locals.var_jsswg_dn9), (p.p16 * locals.var_jsswg_dn10), (p.p16 * locals.var_jsswg_dn13),)
    } else {
        (locals.var_isbs_swg, locals.var_isbs_swg_dn0, locals.var_isbs_swg_dn2, locals.var_isbs_swg_dn4, locals.var_isbs_swg_dn5, locals.var_isbs_swg_dn6, locals.var_isbs_swg_dn7, locals.var_isbs_swg_dn8, locals.var_isbs_swg_dn9, locals.var_isbs_swg_dn10, locals.var_isbs_swg_dn13,)
    }
};
        locals.var_isbs_swg = assign96570_e149198;
        locals.var_isbs_swg_dn0 = assign96570_e149198_d_n0;
        locals.var_isbs_swg_dn2 = assign96570_e149198_d_n2;
        locals.var_isbs_swg_dn4 = assign96570_e149198_d_n4;
        locals.var_isbs_swg_dn5 = assign96570_e149198_d_n5;
        locals.var_isbs_swg_dn6 = assign96570_e149198_d_n6;
        locals.var_isbs_swg_dn7 = assign96570_e149198_d_n7;
        locals.var_isbs_swg_dn8 = assign96570_e149198_d_n8;
        locals.var_isbs_swg_dn9 = assign96570_e149198_d_n9;
        locals.var_isbs_swg_dn10 = assign96570_e149198_d_n10;
        locals.var_isbs_swg_dn13 = assign96570_e149198_d_n13;
        locals.var_isbs_swg_rv = 0.0;

        let (assign96580_e149209, assign96580_e149209_d_n0, assign96580_e149209_d_n2, assign96580_e149209_d_n4, assign96580_e149209_d_n5, assign96580_e149209_d_n6, assign96580_e149209_d_n7, assign96580_e149209_d_n8, assign96580_e149209_d_n9, assign96580_e149209_d_n10, assign96580_e149209_d_n13,) = {
    if (((locals.var_guard2233 != 0.0) && (locals.var_guard2237 != 0.0)) && (locals.var_guard2238 == 0.0)) {
        let assign96580_e149207: f64 = (p.p16 * locals.var_jsswg2);
        (assign96580_e149207, (p.p16 * locals.var_jsswg2_dn0), (p.p16 * locals.var_jsswg2_dn2), (p.p16 * locals.var_jsswg2_dn4), (p.p16 * locals.var_jsswg2_dn5), (p.p16 * locals.var_jsswg2_dn6), (p.p16 * locals.var_jsswg2_dn7), (p.p16 * locals.var_jsswg2_dn8), (p.p16 * locals.var_jsswg2_dn9), (p.p16 * locals.var_jsswg2_dn10), (p.p16 * locals.var_jsswg2_dn13),)
    } else {
        (locals.var_isbs2_swg, locals.var_isbs2_swg_dn0, locals.var_isbs2_swg_dn2, locals.var_isbs2_swg_dn4, locals.var_isbs2_swg_dn5, locals.var_isbs2_swg_dn6, locals.var_isbs2_swg_dn7, locals.var_isbs2_swg_dn8, locals.var_isbs2_swg_dn9, locals.var_isbs2_swg_dn10, locals.var_isbs2_swg_dn13,)
    }
};
        locals.var_isbs2_swg = assign96580_e149209;
        locals.var_isbs2_swg_dn0 = assign96580_e149209_d_n0;
        locals.var_isbs2_swg_dn2 = assign96580_e149209_d_n2;
        locals.var_isbs2_swg_dn4 = assign96580_e149209_d_n4;
        locals.var_isbs2_swg_dn5 = assign96580_e149209_d_n5;
        locals.var_isbs2_swg_dn6 = assign96580_e149209_d_n6;
        locals.var_isbs2_swg_dn7 = assign96580_e149209_d_n7;
        locals.var_isbs2_swg_dn8 = assign96580_e149209_d_n8;
        locals.var_isbs2_swg_dn9 = assign96580_e149209_d_n9;
        locals.var_isbs2_swg_dn10 = assign96580_e149209_d_n10;
        locals.var_isbs2_swg_dn13 = assign96580_e149209_d_n13;
        locals.var_isbs2_swg_rv = 0.0;

        let (assign96590_e149218, assign96590_e149218_d_n0, assign96590_e149218_d_n2, assign96590_e149218_d_n4, assign96590_e149218_d_n5, assign96590_e149218_d_n6, assign96590_e149218_d_n7, assign96590_e149218_d_n8, assign96590_e149218_d_n9, assign96590_e149218_d_n10, assign96590_e149218_d_n13,) = {
    if ((locals.var_guard2233 != 0.0) && (locals.var_guard2237 == 0.0)) {
        let assign96590_e149216: f64 = (p.p14 * locals.var_js);
        (assign96590_e149216, (p.p14 * locals.var_js_dn0), (p.p14 * locals.var_js_dn2), (p.p14 * locals.var_js_dn4), (p.p14 * locals.var_js_dn5), (p.p14 * locals.var_js_dn6), (p.p14 * locals.var_js_dn7), (p.p14 * locals.var_js_dn8), (p.p14 * locals.var_js_dn9), (p.p14 * locals.var_js_dn10), (p.p14 * locals.var_js_dn13),)
    } else {
        (locals.var_isbs_btm, locals.var_isbs_btm_dn0, locals.var_isbs_btm_dn2, locals.var_isbs_btm_dn4, locals.var_isbs_btm_dn5, locals.var_isbs_btm_dn6, locals.var_isbs_btm_dn7, locals.var_isbs_btm_dn8, locals.var_isbs_btm_dn9, locals.var_isbs_btm_dn10, locals.var_isbs_btm_dn13,)
    }
};
        locals.var_isbs_btm = assign96590_e149218;
        locals.var_isbs_btm_dn0 = assign96590_e149218_d_n0;
        locals.var_isbs_btm_dn2 = assign96590_e149218_d_n2;
        locals.var_isbs_btm_dn4 = assign96590_e149218_d_n4;
        locals.var_isbs_btm_dn5 = assign96590_e149218_d_n5;
        locals.var_isbs_btm_dn6 = assign96590_e149218_d_n6;
        locals.var_isbs_btm_dn7 = assign96590_e149218_d_n7;
        locals.var_isbs_btm_dn8 = assign96590_e149218_d_n8;
        locals.var_isbs_btm_dn9 = assign96590_e149218_d_n9;
        locals.var_isbs_btm_dn10 = assign96590_e149218_d_n10;
        locals.var_isbs_btm_dn13 = assign96590_e149218_d_n13;
        locals.var_isbs_btm_rv = 0.0;

        let (assign96600_e149227, assign96600_e149227_d_n0, assign96600_e149227_d_n2, assign96600_e149227_d_n4, assign96600_e149227_d_n5, assign96600_e149227_d_n6, assign96600_e149227_d_n7, assign96600_e149227_d_n8, assign96600_e149227_d_n9, assign96600_e149227_d_n10, assign96600_e149227_d_n13,) = {
    if ((locals.var_guard2233 != 0.0) && (locals.var_guard2237 == 0.0)) {
        let assign96600_e149225: f64 = (p.p14 * locals.var_js2);
        (assign96600_e149225, (p.p14 * locals.var_js2_dn0), (p.p14 * locals.var_js2_dn2), (p.p14 * locals.var_js2_dn4), (p.p14 * locals.var_js2_dn5), (p.p14 * locals.var_js2_dn6), (p.p14 * locals.var_js2_dn7), (p.p14 * locals.var_js2_dn8), (p.p14 * locals.var_js2_dn9), (p.p14 * locals.var_js2_dn10), (p.p14 * locals.var_js2_dn13),)
    } else {
        (locals.var_isbs2_btm, locals.var_isbs2_btm_dn0, locals.var_isbs2_btm_dn2, locals.var_isbs2_btm_dn4, locals.var_isbs2_btm_dn5, locals.var_isbs2_btm_dn6, locals.var_isbs2_btm_dn7, locals.var_isbs2_btm_dn8, locals.var_isbs2_btm_dn9, locals.var_isbs2_btm_dn10, locals.var_isbs2_btm_dn13,)
    }
};
        locals.var_isbs2_btm = assign96600_e149227;
        locals.var_isbs2_btm_dn0 = assign96600_e149227_d_n0;
        locals.var_isbs2_btm_dn2 = assign96600_e149227_d_n2;
        locals.var_isbs2_btm_dn4 = assign96600_e149227_d_n4;
        locals.var_isbs2_btm_dn5 = assign96600_e149227_d_n5;
        locals.var_isbs2_btm_dn6 = assign96600_e149227_d_n6;
        locals.var_isbs2_btm_dn7 = assign96600_e149227_d_n7;
        locals.var_isbs2_btm_dn8 = assign96600_e149227_d_n8;
        locals.var_isbs2_btm_dn9 = assign96600_e149227_d_n9;
        locals.var_isbs2_btm_dn10 = assign96600_e149227_d_n10;
        locals.var_isbs2_btm_dn13 = assign96600_e149227_d_n13;
        locals.var_isbs2_btm_rv = 0.0;

        let (assign96610_e149236, assign96610_e149236_d_n0, assign96610_e149236_d_n2, assign96610_e149236_d_n4, assign96610_e149236_d_n5, assign96610_e149236_d_n6, assign96610_e149236_d_n7, assign96610_e149236_d_n8, assign96610_e149236_d_n9, assign96610_e149236_d_n10, assign96610_e149236_d_n13,) = {
    if ((locals.var_guard2233 != 0.0) && (locals.var_guard2237 == 0.0)) {
        let assign96610_e149234: f64 = (p.p16 * locals.var_jssw);
        (assign96610_e149234, (p.p16 * locals.var_jssw_dn0), (p.p16 * locals.var_jssw_dn2), (p.p16 * locals.var_jssw_dn4), (p.p16 * locals.var_jssw_dn5), (p.p16 * locals.var_jssw_dn6), (p.p16 * locals.var_jssw_dn7), (p.p16 * locals.var_jssw_dn8), (p.p16 * locals.var_jssw_dn9), (p.p16 * locals.var_jssw_dn10), (p.p16 * locals.var_jssw_dn13),)
    } else {
        (locals.var_isbs_sws, locals.var_isbs_sws_dn0, locals.var_isbs_sws_dn2, locals.var_isbs_sws_dn4, locals.var_isbs_sws_dn5, locals.var_isbs_sws_dn6, locals.var_isbs_sws_dn7, locals.var_isbs_sws_dn8, locals.var_isbs_sws_dn9, locals.var_isbs_sws_dn10, locals.var_isbs_sws_dn13,)
    }
};
        locals.var_isbs_sws = assign96610_e149236;
        locals.var_isbs_sws_dn0 = assign96610_e149236_d_n0;
        locals.var_isbs_sws_dn2 = assign96610_e149236_d_n2;
        locals.var_isbs_sws_dn4 = assign96610_e149236_d_n4;
        locals.var_isbs_sws_dn5 = assign96610_e149236_d_n5;
        locals.var_isbs_sws_dn6 = assign96610_e149236_d_n6;
        locals.var_isbs_sws_dn7 = assign96610_e149236_d_n7;
        locals.var_isbs_sws_dn8 = assign96610_e149236_d_n8;
        locals.var_isbs_sws_dn9 = assign96610_e149236_d_n9;
        locals.var_isbs_sws_dn10 = assign96610_e149236_d_n10;
        locals.var_isbs_sws_dn13 = assign96610_e149236_d_n13;
        locals.var_isbs_sws_rv = 0.0;

        let (assign96620_e149245, assign96620_e149245_d_n0, assign96620_e149245_d_n2, assign96620_e149245_d_n4, assign96620_e149245_d_n5, assign96620_e149245_d_n6, assign96620_e149245_d_n7, assign96620_e149245_d_n8, assign96620_e149245_d_n9, assign96620_e149245_d_n10, assign96620_e149245_d_n13,) = {
    if ((locals.var_guard2233 != 0.0) && (locals.var_guard2237 == 0.0)) {
        let assign96620_e149243: f64 = (p.p16 * locals.var_jssw2);
        (assign96620_e149243, (p.p16 * locals.var_jssw2_dn0), (p.p16 * locals.var_jssw2_dn2), (p.p16 * locals.var_jssw2_dn4), (p.p16 * locals.var_jssw2_dn5), (p.p16 * locals.var_jssw2_dn6), (p.p16 * locals.var_jssw2_dn7), (p.p16 * locals.var_jssw2_dn8), (p.p16 * locals.var_jssw2_dn9), (p.p16 * locals.var_jssw2_dn10), (p.p16 * locals.var_jssw2_dn13),)
    } else {
        (locals.var_isbs2_sws, locals.var_isbs2_sws_dn0, locals.var_isbs2_sws_dn2, locals.var_isbs2_sws_dn4, locals.var_isbs2_sws_dn5, locals.var_isbs2_sws_dn6, locals.var_isbs2_sws_dn7, locals.var_isbs2_sws_dn8, locals.var_isbs2_sws_dn9, locals.var_isbs2_sws_dn10, locals.var_isbs2_sws_dn13,)
    }
};
        locals.var_isbs2_sws = assign96620_e149245;
        locals.var_isbs2_sws_dn0 = assign96620_e149245_d_n0;
        locals.var_isbs2_sws_dn2 = assign96620_e149245_d_n2;
        locals.var_isbs2_sws_dn4 = assign96620_e149245_d_n4;
        locals.var_isbs2_sws_dn5 = assign96620_e149245_d_n5;
        locals.var_isbs2_sws_dn6 = assign96620_e149245_d_n6;
        locals.var_isbs2_sws_dn7 = assign96620_e149245_d_n7;
        locals.var_isbs2_sws_dn8 = assign96620_e149245_d_n8;
        locals.var_isbs2_sws_dn9 = assign96620_e149245_d_n9;
        locals.var_isbs2_sws_dn10 = assign96620_e149245_d_n10;
        locals.var_isbs2_sws_dn13 = assign96620_e149245_d_n13;
        locals.var_isbs2_sws_rv = 0.0;

        let (assign96630_e149252, assign96630_e149252_d_n0, assign96630_e149252_d_n2, assign96630_e149252_d_n4, assign96630_e149252_d_n5, assign96630_e149252_d_n6, assign96630_e149252_d_n7, assign96630_e149252_d_n8, assign96630_e149252_d_n9, assign96630_e149252_d_n10, assign96630_e149252_d_n13,) = {
    if ((locals.var_guard2233 != 0.0) && (locals.var_guard2237 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_isbs_swg, locals.var_isbs_swg_dn0, locals.var_isbs_swg_dn2, locals.var_isbs_swg_dn4, locals.var_isbs_swg_dn5, locals.var_isbs_swg_dn6, locals.var_isbs_swg_dn7, locals.var_isbs_swg_dn8, locals.var_isbs_swg_dn9, locals.var_isbs_swg_dn10, locals.var_isbs_swg_dn13,)
    }
};
        locals.var_isbs_swg = assign96630_e149252;
        locals.var_isbs_swg_dn0 = assign96630_e149252_d_n0;
        locals.var_isbs_swg_dn2 = assign96630_e149252_d_n2;
        locals.var_isbs_swg_dn4 = assign96630_e149252_d_n4;
        locals.var_isbs_swg_dn5 = assign96630_e149252_d_n5;
        locals.var_isbs_swg_dn6 = assign96630_e149252_d_n6;
        locals.var_isbs_swg_dn7 = assign96630_e149252_d_n7;
        locals.var_isbs_swg_dn8 = assign96630_e149252_d_n8;
        locals.var_isbs_swg_dn9 = assign96630_e149252_d_n9;
        locals.var_isbs_swg_dn10 = assign96630_e149252_d_n10;
        locals.var_isbs_swg_dn13 = assign96630_e149252_d_n13;
        locals.var_isbs_swg_rv = 0.0;

        let (assign96640_e149259, assign96640_e149259_d_n0, assign96640_e149259_d_n2, assign96640_e149259_d_n4, assign96640_e149259_d_n5, assign96640_e149259_d_n6, assign96640_e149259_d_n7, assign96640_e149259_d_n8, assign96640_e149259_d_n9, assign96640_e149259_d_n10, assign96640_e149259_d_n13,) = {
    if ((locals.var_guard2233 != 0.0) && (locals.var_guard2237 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_isbs2_swg, locals.var_isbs2_swg_dn0, locals.var_isbs2_swg_dn2, locals.var_isbs2_swg_dn4, locals.var_isbs2_swg_dn5, locals.var_isbs2_swg_dn6, locals.var_isbs2_swg_dn7, locals.var_isbs2_swg_dn8, locals.var_isbs2_swg_dn9, locals.var_isbs2_swg_dn10, locals.var_isbs2_swg_dn13,)
    }
};
        locals.var_isbs2_swg = assign96640_e149259;
        locals.var_isbs2_swg_dn0 = assign96640_e149259_d_n0;
        locals.var_isbs2_swg_dn2 = assign96640_e149259_d_n2;
        locals.var_isbs2_swg_dn4 = assign96640_e149259_d_n4;
        locals.var_isbs2_swg_dn5 = assign96640_e149259_d_n5;
        locals.var_isbs2_swg_dn6 = assign96640_e149259_d_n6;
        locals.var_isbs2_swg_dn7 = assign96640_e149259_d_n7;
        locals.var_isbs2_swg_dn8 = assign96640_e149259_d_n8;
        locals.var_isbs2_swg_dn9 = assign96640_e149259_d_n9;
        locals.var_isbs2_swg_dn10 = assign96640_e149259_d_n10;
        locals.var_isbs2_swg_dn13 = assign96640_e149259_d_n13;
        locals.var_isbs2_swg_rv = 0.0;

        let (assign96650_e149267, assign96650_e149267_d_n0, assign96650_e149267_d_n2, assign96650_e149267_d_n4, assign96650_e149267_d_n5, assign96650_e149267_d_n6, assign96650_e149267_d_n7, assign96650_e149267_d_n8, assign96650_e149267_d_n9, assign96650_e149267_d_n10, assign96650_e149267_d_n13,) = {
    if (locals.var_guard2233 != 0.0) {
        let assign96650_e149263: f64 = (locals.var_isbs_btm + locals.var_isbs_sws);
        let assign96650_e149265: f64 = (assign96650_e149263 + locals.var_isbs_swg);
        (assign96650_e149265, ((locals.var_isbs_btm_dn0 + locals.var_isbs_sws_dn0) + locals.var_isbs_swg_dn0), ((locals.var_isbs_btm_dn2 + locals.var_isbs_sws_dn2) + locals.var_isbs_swg_dn2), ((locals.var_isbs_btm_dn4 + locals.var_isbs_sws_dn4) + locals.var_isbs_swg_dn4), ((locals.var_isbs_btm_dn5 + locals.var_isbs_sws_dn5) + locals.var_isbs_swg_dn5), ((locals.var_isbs_btm_dn6 + locals.var_isbs_sws_dn6) + locals.var_isbs_swg_dn6), ((locals.var_isbs_btm_dn7 + locals.var_isbs_sws_dn7) + locals.var_isbs_swg_dn7), ((locals.var_isbs_btm_dn8 + locals.var_isbs_sws_dn8) + locals.var_isbs_swg_dn8), ((locals.var_isbs_btm_dn9 + locals.var_isbs_sws_dn9) + locals.var_isbs_swg_dn9), ((locals.var_isbs_btm_dn10 + locals.var_isbs_sws_dn10) + locals.var_isbs_swg_dn10), ((locals.var_isbs_btm_dn13 + locals.var_isbs_sws_dn13) + locals.var_isbs_swg_dn13),)
    } else {
        (locals.var_isbs, locals.var_isbs_dn0, locals.var_isbs_dn2, locals.var_isbs_dn4, locals.var_isbs_dn5, locals.var_isbs_dn6, locals.var_isbs_dn7, locals.var_isbs_dn8, locals.var_isbs_dn9, locals.var_isbs_dn10, locals.var_isbs_dn13,)
    }
};
        locals.var_isbs = assign96650_e149267;
        locals.var_isbs_dn0 = assign96650_e149267_d_n0;
        locals.var_isbs_dn2 = assign96650_e149267_d_n2;
        locals.var_isbs_dn4 = assign96650_e149267_d_n4;
        locals.var_isbs_dn5 = assign96650_e149267_d_n5;
        locals.var_isbs_dn6 = assign96650_e149267_d_n6;
        locals.var_isbs_dn7 = assign96650_e149267_d_n7;
        locals.var_isbs_dn8 = assign96650_e149267_d_n8;
        locals.var_isbs_dn9 = assign96650_e149267_d_n9;
        locals.var_isbs_dn10 = assign96650_e149267_d_n10;
        locals.var_isbs_dn13 = assign96650_e149267_d_n13;
        locals.var_isbs_rv = 0.0;

        let assign96660_e149270: f64 = if locals.var_isbs > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2239 = assign96660_e149270;
        locals.var_guard2239_rv = 0.0;

        let (assign96670_e149278, assign96670_e149278_d_n0, assign96670_e149278_d_n2, assign96670_e149278_d_n4, assign96670_e149278_d_n5, assign96670_e149278_d_n6, assign96670_e149278_d_n7, assign96670_e149278_d_n8, assign96670_e149278_d_n9, assign96670_e149278_d_n10, assign96670_e149278_d_n13,) = {
    if ((locals.var_guard2233 != 0.0) && (locals.var_guard2239 != 0.0)) {
        let assign96670_e149276: f64 = (locals.var_isbs + 1e-25);
        (assign96670_e149276, locals.var_isbs_dn0, locals.var_isbs_dn2, locals.var_isbs_dn4, locals.var_isbs_dn5, locals.var_isbs_dn6, locals.var_isbs_dn7, locals.var_isbs_dn8, locals.var_isbs_dn9, locals.var_isbs_dn10, locals.var_isbs_dn13,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn13,)
    }
};
        locals.var_t3 = assign96670_e149278;
        locals.var_t3_dn0 = assign96670_e149278_d_n0;
        locals.var_t3_dn2 = assign96670_e149278_d_n2;
        locals.var_t3_dn4 = assign96670_e149278_d_n4;
        locals.var_t3_dn5 = assign96670_e149278_d_n5;
        locals.var_t3_dn6 = assign96670_e149278_d_n6;
        locals.var_t3_dn7 = assign96670_e149278_d_n7;
        locals.var_t3_dn8 = assign96670_e149278_d_n8;
        locals.var_t3_dn9 = assign96670_e149278_d_n9;
        locals.var_t3_dn10 = assign96670_e149278_d_n10;
        locals.var_t3_dn13 = assign96670_e149278_d_n13;
        locals.var_t3_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_363(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign96680_e149295, assign96680_e149295_d_n0, assign96680_e149295_d_n2, assign96680_e149295_d_n4, assign96680_e149295_d_n5, assign96680_e149295_d_n6, assign96680_e149295_d_n7, assign96680_e149295_d_n8, assign96680_e149295_d_n9, assign96680_e149295_d_n10, assign96680_e149295_d_n13,) = {
    if ((locals.var_guard2233 != 0.0) && (locals.var_guard2239 != 0.0)) {
        let assign96680_e149284: f64 = (locals.var_uc_njs / locals.var_beta);
        let assign96680_e149287: f64 = (locals.var_uc_vdiffjs * locals.var_t0);
        let assign96680_e149289: f64 = (assign96680_e149287 / locals.var_t3);
        let assign96680_e149291: f64 = (assign96680_e149289 + 1.0);
        let assign96680_e149292: f64 = (assign96680_e149291).ln();
        let assign96680_e149293: f64 = (assign96680_e149284 * assign96680_e149292);
        (assign96680_e149293, (((-((locals.var_uc_njs * locals.var_beta_dn0) / (locals.var_beta * locals.var_beta))) * assign96680_e149292) + (assign96680_e149284 * (((((locals.var_uc_vdiffjs * locals.var_t0_dn0) * locals.var_t3) - (assign96680_e149287 * locals.var_t3_dn0)) / (locals.var_t3 * locals.var_t3)) / assign96680_e149291))), (((-((locals.var_uc_njs * locals.var_beta_dn2) / (locals.var_beta * locals.var_beta))) * assign96680_e149292) + (assign96680_e149284 * (((((locals.var_uc_vdiffjs * locals.var_t0_dn2) * locals.var_t3) - (assign96680_e149287 * locals.var_t3_dn2)) / (locals.var_t3 * locals.var_t3)) / assign96680_e149291))), (((-((locals.var_uc_njs * locals.var_beta_dn4) / (locals.var_beta * locals.var_beta))) * assign96680_e149292) + (assign96680_e149284 * (((((locals.var_uc_vdiffjs * locals.var_t0_dn4) * locals.var_t3) - (assign96680_e149287 * locals.var_t3_dn4)) / (locals.var_t3 * locals.var_t3)) / assign96680_e149291))), (((-((locals.var_uc_njs * locals.var_beta_dn5) / (locals.var_beta * locals.var_beta))) * assign96680_e149292) + (assign96680_e149284 * (((((locals.var_uc_vdiffjs * locals.var_t0_dn5) * locals.var_t3) - (assign96680_e149287 * locals.var_t3_dn5)) / (locals.var_t3 * locals.var_t3)) / assign96680_e149291))), (((-((locals.var_uc_njs * locals.var_beta_dn6) / (locals.var_beta * locals.var_beta))) * assign96680_e149292) + (assign96680_e149284 * (((((locals.var_uc_vdiffjs * locals.var_t0_dn6) * locals.var_t3) - (assign96680_e149287 * locals.var_t3_dn6)) / (locals.var_t3 * locals.var_t3)) / assign96680_e149291))), (((-((locals.var_uc_njs * locals.var_beta_dn7) / (locals.var_beta * locals.var_beta))) * assign96680_e149292) + (assign96680_e149284 * (((((locals.var_uc_vdiffjs * locals.var_t0_dn7) * locals.var_t3) - (assign96680_e149287 * locals.var_t3_dn7)) / (locals.var_t3 * locals.var_t3)) / assign96680_e149291))), (((-((locals.var_uc_njs * locals.var_beta_dn8) / (locals.var_beta * locals.var_beta))) * assign96680_e149292) + (assign96680_e149284 * (((((locals.var_uc_vdiffjs * locals.var_t0_dn8) * locals.var_t3) - (assign96680_e149287 * locals.var_t3_dn8)) / (locals.var_t3 * locals.var_t3)) / assign96680_e149291))), (((-((locals.var_uc_njs * locals.var_beta_dn9) / (locals.var_beta * locals.var_beta))) * assign96680_e149292) + (assign96680_e149284 * (((((locals.var_uc_vdiffjs * locals.var_t0_dn9) * locals.var_t3) - (assign96680_e149287 * locals.var_t3_dn9)) / (locals.var_t3 * locals.var_t3)) / assign96680_e149291))), (((-((locals.var_uc_njs * locals.var_beta_dn10) / (locals.var_beta * locals.var_beta))) * assign96680_e149292) + (assign96680_e149284 * (((((locals.var_uc_vdiffjs * locals.var_t0_dn10) * locals.var_t3) - (assign96680_e149287 * locals.var_t3_dn10)) / (locals.var_t3 * locals.var_t3)) / assign96680_e149291))), (((-((locals.var_uc_njs * locals.var_beta_dn13) / (locals.var_beta * locals.var_beta))) * assign96680_e149292) + (assign96680_e149284 * (((((locals.var_uc_vdiffjs * locals.var_t0_dn13) * locals.var_t3) - (assign96680_e149287 * locals.var_t3_dn13)) / (locals.var_t3 * locals.var_t3)) / assign96680_e149291))),)
    } else {
        (locals.var_vbst, locals.var_vbst_dn0, locals.var_vbst_dn2, locals.var_vbst_dn4, locals.var_vbst_dn5, locals.var_vbst_dn6, locals.var_vbst_dn7, locals.var_vbst_dn8, locals.var_vbst_dn9, locals.var_vbst_dn10, locals.var_vbst_dn13,)
    }
};
        locals.var_vbst = assign96680_e149295;
        locals.var_vbst_dn0 = assign96680_e149295_d_n0;
        locals.var_vbst_dn2 = assign96680_e149295_d_n2;
        locals.var_vbst_dn4 = assign96680_e149295_d_n4;
        locals.var_vbst_dn5 = assign96680_e149295_d_n5;
        locals.var_vbst_dn6 = assign96680_e149295_d_n6;
        locals.var_vbst_dn7 = assign96680_e149295_d_n7;
        locals.var_vbst_dn8 = assign96680_e149295_d_n8;
        locals.var_vbst_dn9 = assign96680_e149295_d_n9;
        locals.var_vbst_dn10 = assign96680_e149295_d_n10;
        locals.var_vbst_dn13 = assign96680_e149295_d_n13;
        locals.var_vbst_rv = 0.0;

        let (assign96690_e149306, assign96690_e149306_d_n0, assign96690_e149306_d_n2, assign96690_e149306_d_n4, assign96690_e149306_d_n5, assign96690_e149306_d_n6, assign96690_e149306_d_n7, assign96690_e149306_d_n8, assign96690_e149306_d_n9, assign96690_e149306_d_n10, assign96690_e149306_d_n13,) = {
    if ((locals.var_guard2233 != 0.0) && (locals.var_guard2239 != 0.0)) {
        let assign96690_e149301: f64 = (locals.var_tratio - 1.0);
        let assign96690_e149303: f64 = (assign96690_e149301 * p.p535);
        let assign96690_e149304: f64 = (assign96690_e149303).exp();
        (assign96690_e149304, (assign96690_e149304 * (locals.var_tratio_dn0 * p.p535)), (assign96690_e149304 * (locals.var_tratio_dn2 * p.p535)), (assign96690_e149304 * (locals.var_tratio_dn4 * p.p535)), (assign96690_e149304 * (locals.var_tratio_dn5 * p.p535)), (assign96690_e149304 * (locals.var_tratio_dn6 * p.p535)), (assign96690_e149304 * (locals.var_tratio_dn7 * p.p535)), (assign96690_e149304 * (locals.var_tratio_dn8 * p.p535)), (assign96690_e149304 * (locals.var_tratio_dn9 * p.p535)), (assign96690_e149304 * (locals.var_tratio_dn10 * p.p535)), (assign96690_e149304 * (locals.var_tratio_dn13 * p.p535)),)
    } else {
        (locals.var_exptemps, locals.var_exptemps_dn0, locals.var_exptemps_dn2, locals.var_exptemps_dn4, locals.var_exptemps_dn5, locals.var_exptemps_dn6, locals.var_exptemps_dn7, locals.var_exptemps_dn8, locals.var_exptemps_dn9, locals.var_exptemps_dn10, locals.var_exptemps_dn13,)
    }
};
        locals.var_exptemps = assign96690_e149306;
        locals.var_exptemps_dn0 = assign96690_e149306_d_n0;
        locals.var_exptemps_dn2 = assign96690_e149306_d_n2;
        locals.var_exptemps_dn4 = assign96690_e149306_d_n4;
        locals.var_exptemps_dn5 = assign96690_e149306_d_n5;
        locals.var_exptemps_dn6 = assign96690_e149306_d_n6;
        locals.var_exptemps_dn7 = assign96690_e149306_d_n7;
        locals.var_exptemps_dn8 = assign96690_e149306_d_n8;
        locals.var_exptemps_dn9 = assign96690_e149306_d_n9;
        locals.var_exptemps_dn10 = assign96690_e149306_d_n10;
        locals.var_exptemps_dn13 = assign96690_e149306_d_n13;
        locals.var_exptemps_rv = 0.0;

        let (assign96700_e149316, assign96700_e149316_d_n0, assign96700_e149316_d_n2, assign96700_e149316_d_n4, assign96700_e149316_d_n5, assign96700_e149316_d_n6, assign96700_e149316_d_n7, assign96700_e149316_d_n8, assign96700_e149316_d_n9, assign96700_e149316_d_n10, assign96700_e149316_d_n13,) = {
    if ((locals.var_guard2233 != 0.0) && (locals.var_guard2239 != 0.0)) {
        let assign96700_e149313: f64 = (locals.var_uc_njs / locals.var_beta);
        let assign96700_e149314: f64 = (1.0 / assign96700_e149313);
        (assign96700_e149314, (-((-((locals.var_uc_njs * locals.var_beta_dn0) / (locals.var_beta * locals.var_beta))) / (assign96700_e149313 * assign96700_e149313))), (-((-((locals.var_uc_njs * locals.var_beta_dn2) / (locals.var_beta * locals.var_beta))) / (assign96700_e149313 * assign96700_e149313))), (-((-((locals.var_uc_njs * locals.var_beta_dn4) / (locals.var_beta * locals.var_beta))) / (assign96700_e149313 * assign96700_e149313))), (-((-((locals.var_uc_njs * locals.var_beta_dn5) / (locals.var_beta * locals.var_beta))) / (assign96700_e149313 * assign96700_e149313))), (-((-((locals.var_uc_njs * locals.var_beta_dn6) / (locals.var_beta * locals.var_beta))) / (assign96700_e149313 * assign96700_e149313))), (-((-((locals.var_uc_njs * locals.var_beta_dn7) / (locals.var_beta * locals.var_beta))) / (assign96700_e149313 * assign96700_e149313))), (-((-((locals.var_uc_njs * locals.var_beta_dn8) / (locals.var_beta * locals.var_beta))) / (assign96700_e149313 * assign96700_e149313))), (-((-((locals.var_uc_njs * locals.var_beta_dn9) / (locals.var_beta * locals.var_beta))) / (assign96700_e149313 * assign96700_e149313))), (-((-((locals.var_uc_njs * locals.var_beta_dn10) / (locals.var_beta * locals.var_beta))) / (assign96700_e149313 * assign96700_e149313))), (-((-((locals.var_uc_njs * locals.var_beta_dn13) / (locals.var_beta * locals.var_beta))) / (assign96700_e149313 * assign96700_e149313))),)
    } else {
        (locals.var_jd_nvtm_invs, locals.var_jd_nvtm_invs_dn0, locals.var_jd_nvtm_invs_dn2, locals.var_jd_nvtm_invs_dn4, locals.var_jd_nvtm_invs_dn5, locals.var_jd_nvtm_invs_dn6, locals.var_jd_nvtm_invs_dn7, locals.var_jd_nvtm_invs_dn8, locals.var_jd_nvtm_invs_dn9, locals.var_jd_nvtm_invs_dn10, locals.var_jd_nvtm_invs_dn13,)
    }
};
        locals.var_jd_nvtm_invs = assign96700_e149316;
        locals.var_jd_nvtm_invs_dn0 = assign96700_e149316_d_n0;
        locals.var_jd_nvtm_invs_dn2 = assign96700_e149316_d_n2;
        locals.var_jd_nvtm_invs_dn4 = assign96700_e149316_d_n4;
        locals.var_jd_nvtm_invs_dn5 = assign96700_e149316_d_n5;
        locals.var_jd_nvtm_invs_dn6 = assign96700_e149316_d_n6;
        locals.var_jd_nvtm_invs_dn7 = assign96700_e149316_d_n7;
        locals.var_jd_nvtm_invs_dn8 = assign96700_e149316_d_n8;
        locals.var_jd_nvtm_invs_dn9 = assign96700_e149316_d_n9;
        locals.var_jd_nvtm_invs_dn10 = assign96700_e149316_d_n10;
        locals.var_jd_nvtm_invs_dn13 = assign96700_e149316_d_n13;
        locals.var_jd_nvtm_invs_rv = 0.0;

        let (assign96710_e149325, assign96710_e149325_d_n0, assign96710_e149325_d_n2, assign96710_e149325_d_n4, assign96710_e149325_d_n5, assign96710_e149325_d_n6, assign96710_e149325_d_n7, assign96710_e149325_d_n8, assign96710_e149325_d_n9, assign96710_e149325_d_n10, assign96710_e149325_d_n13,) = {
    if ((locals.var_guard2233 != 0.0) && (locals.var_guard2239 != 0.0)) {
        let assign96710_e149322: f64 = (locals.var_vbst * locals.var_jd_nvtm_invs);
        let assign96710_e149323: f64 = (assign96710_e149322).exp();
        (assign96710_e149323, (assign96710_e149323 * ((locals.var_vbst_dn0 * locals.var_jd_nvtm_invs) + (locals.var_vbst * locals.var_jd_nvtm_invs_dn0))), (assign96710_e149323 * ((locals.var_vbst_dn2 * locals.var_jd_nvtm_invs) + (locals.var_vbst * locals.var_jd_nvtm_invs_dn2))), (assign96710_e149323 * ((locals.var_vbst_dn4 * locals.var_jd_nvtm_invs) + (locals.var_vbst * locals.var_jd_nvtm_invs_dn4))), (assign96710_e149323 * ((locals.var_vbst_dn5 * locals.var_jd_nvtm_invs) + (locals.var_vbst * locals.var_jd_nvtm_invs_dn5))), (assign96710_e149323 * ((locals.var_vbst_dn6 * locals.var_jd_nvtm_invs) + (locals.var_vbst * locals.var_jd_nvtm_invs_dn6))), (assign96710_e149323 * ((locals.var_vbst_dn7 * locals.var_jd_nvtm_invs) + (locals.var_vbst * locals.var_jd_nvtm_invs_dn7))), (assign96710_e149323 * ((locals.var_vbst_dn8 * locals.var_jd_nvtm_invs) + (locals.var_vbst * locals.var_jd_nvtm_invs_dn8))), (assign96710_e149323 * ((locals.var_vbst_dn9 * locals.var_jd_nvtm_invs) + (locals.var_vbst * locals.var_jd_nvtm_invs_dn9))), (assign96710_e149323 * ((locals.var_vbst_dn10 * locals.var_jd_nvtm_invs) + (locals.var_vbst * locals.var_jd_nvtm_invs_dn10))), (assign96710_e149323 * ((locals.var_vbst_dn13 * locals.var_jd_nvtm_invs) + (locals.var_vbst * locals.var_jd_nvtm_invs_dn13))),)
    } else {
        (locals.var_jd_expcs, locals.var_jd_expcs_dn0, locals.var_jd_expcs_dn2, locals.var_jd_expcs_dn4, locals.var_jd_expcs_dn5, locals.var_jd_expcs_dn6, locals.var_jd_expcs_dn7, locals.var_jd_expcs_dn8, locals.var_jd_expcs_dn9, locals.var_jd_expcs_dn10, locals.var_jd_expcs_dn13,)
    }
};
        locals.var_jd_expcs = assign96710_e149325;
        locals.var_jd_expcs_dn0 = assign96710_e149325_d_n0;
        locals.var_jd_expcs_dn2 = assign96710_e149325_d_n2;
        locals.var_jd_expcs_dn4 = assign96710_e149325_d_n4;
        locals.var_jd_expcs_dn5 = assign96710_e149325_d_n5;
        locals.var_jd_expcs_dn6 = assign96710_e149325_d_n6;
        locals.var_jd_expcs_dn7 = assign96710_e149325_d_n7;
        locals.var_jd_expcs_dn8 = assign96710_e149325_d_n8;
        locals.var_jd_expcs_dn9 = assign96710_e149325_d_n9;
        locals.var_jd_expcs_dn10 = assign96710_e149325_d_n10;
        locals.var_jd_expcs_dn13 = assign96710_e149325_d_n13;
        locals.var_jd_expcs_rv = 0.0;

        let (assign96720_e149337, assign96720_e149337_d_n0, assign96720_e149337_d_n2, assign96720_e149337_d_n4, assign96720_e149337_d_n5, assign96720_e149337_d_n6, assign96720_e149337_d_n7, assign96720_e149337_d_n8, assign96720_e149337_d_n9, assign96720_e149337_d_n10, assign96720_e149337_d_n13,) = {
    if (locals.var_guard2233 != 0.0) {
        let assign96720_e149329: f64 = (p.p500 * p.p13);
        let assign96720_e149333: f64 = (p.p481 * locals.var_tdiff);
        let assign96720_e149334: f64 = (1.0 + assign96720_e149333);
        let assign96720_e149335: f64 = (assign96720_e149329 * assign96720_e149334);
        (assign96720_e149335, (assign96720_e149329 * (p.p481 * locals.var_tdiff_dn0)), (assign96720_e149329 * (p.p481 * locals.var_tdiff_dn2)), (assign96720_e149329 * (p.p481 * locals.var_tdiff_dn4)), (assign96720_e149329 * (p.p481 * locals.var_tdiff_dn5)), (assign96720_e149329 * (p.p481 * locals.var_tdiff_dn6)), (assign96720_e149329 * (p.p481 * locals.var_tdiff_dn7)), (assign96720_e149329 * (p.p481 * locals.var_tdiff_dn8)), (assign96720_e149329 * (p.p481 * locals.var_tdiff_dn9)), (assign96720_e149329 * (p.p481 * locals.var_tdiff_dn10)), (assign96720_e149329 * (p.p481 * locals.var_tdiff_dn13)),)
    } else {
        (locals.var_czbd, locals.var_czbd_dn0, locals.var_czbd_dn2, locals.var_czbd_dn4, locals.var_czbd_dn5, locals.var_czbd_dn6, locals.var_czbd_dn7, locals.var_czbd_dn8, locals.var_czbd_dn9, locals.var_czbd_dn10, locals.var_czbd_dn13,)
    }
};
        locals.var_czbd = assign96720_e149337;
        locals.var_czbd_dn0 = assign96720_e149337_d_n0;
        locals.var_czbd_dn2 = assign96720_e149337_d_n2;
        locals.var_czbd_dn4 = assign96720_e149337_d_n4;
        locals.var_czbd_dn5 = assign96720_e149337_d_n5;
        locals.var_czbd_dn6 = assign96720_e149337_d_n6;
        locals.var_czbd_dn7 = assign96720_e149337_d_n7;
        locals.var_czbd_dn8 = assign96720_e149337_d_n8;
        locals.var_czbd_dn9 = assign96720_e149337_d_n9;
        locals.var_czbd_dn10 = assign96720_e149337_d_n10;
        locals.var_czbd_dn13 = assign96720_e149337_d_n13;
        locals.var_czbd_rv = 0.0;

        let assign96730_e149340: f64 = if p.p15 > locals.var_weff_nf { 1.0 } else { 0.0 };
        locals.var_guard2240 = assign96730_e149340;
        locals.var_guard2240_rv = 0.0;

        let (assign96740_e149356, assign96740_e149356_d_n0, assign96740_e149356_d_n2, assign96740_e149356_d_n4, assign96740_e149356_d_n5, assign96740_e149356_d_n6, assign96740_e149356_d_n7, assign96740_e149356_d_n8, assign96740_e149356_d_n9, assign96740_e149356_d_n10, assign96740_e149356_d_n13,) = {
    if ((locals.var_guard2233 != 0.0) && (locals.var_guard2240 != 0.0)) {
        let assign96740_e149347: f64 = (p.p15 - locals.var_weff_nf);
        let assign96740_e149348: f64 = (p.p501 * assign96740_e149347);
        let assign96740_e149352: f64 = (p.p483 * locals.var_tdiff);
        let assign96740_e149353: f64 = (1.0 + assign96740_e149352);
        let assign96740_e149354: f64 = (assign96740_e149348 * assign96740_e149353);
        (assign96740_e149354, (assign96740_e149348 * (p.p483 * locals.var_tdiff_dn0)), (assign96740_e149348 * (p.p483 * locals.var_tdiff_dn2)), (assign96740_e149348 * (p.p483 * locals.var_tdiff_dn4)), (assign96740_e149348 * (p.p483 * locals.var_tdiff_dn5)), (assign96740_e149348 * (p.p483 * locals.var_tdiff_dn6)), (assign96740_e149348 * (p.p483 * locals.var_tdiff_dn7)), (assign96740_e149348 * (p.p483 * locals.var_tdiff_dn8)), (assign96740_e149348 * (p.p483 * locals.var_tdiff_dn9)), (assign96740_e149348 * (p.p483 * locals.var_tdiff_dn10)), (assign96740_e149348 * (p.p483 * locals.var_tdiff_dn13)),)
    } else {
        (locals.var_czbdsw, locals.var_czbdsw_dn0, locals.var_czbdsw_dn2, locals.var_czbdsw_dn4, locals.var_czbdsw_dn5, locals.var_czbdsw_dn6, locals.var_czbdsw_dn7, locals.var_czbdsw_dn8, locals.var_czbdsw_dn9, locals.var_czbdsw_dn10, locals.var_czbdsw_dn13,)
    }
};
        locals.var_czbdsw = assign96740_e149356;
        locals.var_czbdsw_dn0 = assign96740_e149356_d_n0;
        locals.var_czbdsw_dn2 = assign96740_e149356_d_n2;
        locals.var_czbdsw_dn4 = assign96740_e149356_d_n4;
        locals.var_czbdsw_dn5 = assign96740_e149356_d_n5;
        locals.var_czbdsw_dn6 = assign96740_e149356_d_n6;
        locals.var_czbdsw_dn7 = assign96740_e149356_d_n7;
        locals.var_czbdsw_dn8 = assign96740_e149356_d_n8;
        locals.var_czbdsw_dn9 = assign96740_e149356_d_n9;
        locals.var_czbdsw_dn10 = assign96740_e149356_d_n10;
        locals.var_czbdsw_dn13 = assign96740_e149356_d_n13;
        locals.var_czbdsw_rv = 0.0;

        let (assign96750_e149370, assign96750_e149370_d_n0, assign96750_e149370_d_n2, assign96750_e149370_d_n4, assign96750_e149370_d_n5, assign96750_e149370_d_n6, assign96750_e149370_d_n7, assign96750_e149370_d_n8, assign96750_e149370_d_n9, assign96750_e149370_d_n10, assign96750_e149370_d_n13,) = {
    if ((locals.var_guard2233 != 0.0) && (locals.var_guard2240 != 0.0)) {
        let assign96750_e149362: f64 = (p.p502 * locals.var_weff_nf);
        let assign96750_e149366: f64 = (p.p485 * locals.var_tdiff);
        let assign96750_e149367: f64 = (1.0 + assign96750_e149366);
        let assign96750_e149368: f64 = (assign96750_e149362 * assign96750_e149367);
        (assign96750_e149368, (assign96750_e149362 * (p.p485 * locals.var_tdiff_dn0)), (assign96750_e149362 * (p.p485 * locals.var_tdiff_dn2)), (assign96750_e149362 * (p.p485 * locals.var_tdiff_dn4)), (assign96750_e149362 * (p.p485 * locals.var_tdiff_dn5)), (assign96750_e149362 * (p.p485 * locals.var_tdiff_dn6)), (assign96750_e149362 * (p.p485 * locals.var_tdiff_dn7)), (assign96750_e149362 * (p.p485 * locals.var_tdiff_dn8)), (assign96750_e149362 * (p.p485 * locals.var_tdiff_dn9)), (assign96750_e149362 * (p.p485 * locals.var_tdiff_dn10)), (assign96750_e149362 * (p.p485 * locals.var_tdiff_dn13)),)
    } else {
        (locals.var_czbdswg, locals.var_czbdswg_dn0, locals.var_czbdswg_dn2, locals.var_czbdswg_dn4, locals.var_czbdswg_dn5, locals.var_czbdswg_dn6, locals.var_czbdswg_dn7, locals.var_czbdswg_dn8, locals.var_czbdswg_dn9, locals.var_czbdswg_dn10, locals.var_czbdswg_dn13,)
    }
};
        locals.var_czbdswg = assign96750_e149370;
        locals.var_czbdswg_dn0 = assign96750_e149370_d_n0;
        locals.var_czbdswg_dn2 = assign96750_e149370_d_n2;
        locals.var_czbdswg_dn4 = assign96750_e149370_d_n4;
        locals.var_czbdswg_dn5 = assign96750_e149370_d_n5;
        locals.var_czbdswg_dn6 = assign96750_e149370_d_n6;
        locals.var_czbdswg_dn7 = assign96750_e149370_d_n7;
        locals.var_czbdswg_dn8 = assign96750_e149370_d_n8;
        locals.var_czbdswg_dn9 = assign96750_e149370_d_n9;
        locals.var_czbdswg_dn10 = assign96750_e149370_d_n10;
        locals.var_czbdswg_dn13 = assign96750_e149370_d_n13;
        locals.var_czbdswg_rv = 0.0;

        let (assign96760_e149377, assign96760_e149377_d_n0, assign96760_e149377_d_n2, assign96760_e149377_d_n4, assign96760_e149377_d_n5, assign96760_e149377_d_n6, assign96760_e149377_d_n7, assign96760_e149377_d_n8, assign96760_e149377_d_n9, assign96760_e149377_d_n10, assign96760_e149377_d_n13,) = {
    if ((locals.var_guard2233 != 0.0) && (locals.var_guard2240 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_czbdsw, locals.var_czbdsw_dn0, locals.var_czbdsw_dn2, locals.var_czbdsw_dn4, locals.var_czbdsw_dn5, locals.var_czbdsw_dn6, locals.var_czbdsw_dn7, locals.var_czbdsw_dn8, locals.var_czbdsw_dn9, locals.var_czbdsw_dn10, locals.var_czbdsw_dn13,)
    }
};
        locals.var_czbdsw = assign96760_e149377;
        locals.var_czbdsw_dn0 = assign96760_e149377_d_n0;
        locals.var_czbdsw_dn2 = assign96760_e149377_d_n2;
        locals.var_czbdsw_dn4 = assign96760_e149377_d_n4;
        locals.var_czbdsw_dn5 = assign96760_e149377_d_n5;
        locals.var_czbdsw_dn6 = assign96760_e149377_d_n6;
        locals.var_czbdsw_dn7 = assign96760_e149377_d_n7;
        locals.var_czbdsw_dn8 = assign96760_e149377_d_n8;
        locals.var_czbdsw_dn9 = assign96760_e149377_d_n9;
        locals.var_czbdsw_dn10 = assign96760_e149377_d_n10;
        locals.var_czbdsw_dn13 = assign96760_e149377_d_n13;
        locals.var_czbdsw_rv = 0.0;

        let (assign96770_e149392, assign96770_e149392_d_n0, assign96770_e149392_d_n2, assign96770_e149392_d_n4, assign96770_e149392_d_n5, assign96770_e149392_d_n6, assign96770_e149392_d_n7, assign96770_e149392_d_n8, assign96770_e149392_d_n9, assign96770_e149392_d_n10, assign96770_e149392_d_n13,) = {
    if ((locals.var_guard2233 != 0.0) && (locals.var_guard2240 == 0.0)) {
        let assign96770_e149384: f64 = (p.p502 * p.p15);
        let assign96770_e149388: f64 = (p.p485 * locals.var_tdiff);
        let assign96770_e149389: f64 = (1.0 + assign96770_e149388);
        let assign96770_e149390: f64 = (assign96770_e149384 * assign96770_e149389);
        (assign96770_e149390, (assign96770_e149384 * (p.p485 * locals.var_tdiff_dn0)), (assign96770_e149384 * (p.p485 * locals.var_tdiff_dn2)), (assign96770_e149384 * (p.p485 * locals.var_tdiff_dn4)), (assign96770_e149384 * (p.p485 * locals.var_tdiff_dn5)), (assign96770_e149384 * (p.p485 * locals.var_tdiff_dn6)), (assign96770_e149384 * (p.p485 * locals.var_tdiff_dn7)), (assign96770_e149384 * (p.p485 * locals.var_tdiff_dn8)), (assign96770_e149384 * (p.p485 * locals.var_tdiff_dn9)), (assign96770_e149384 * (p.p485 * locals.var_tdiff_dn10)), (assign96770_e149384 * (p.p485 * locals.var_tdiff_dn13)),)
    } else {
        (locals.var_czbdswg, locals.var_czbdswg_dn0, locals.var_czbdswg_dn2, locals.var_czbdswg_dn4, locals.var_czbdswg_dn5, locals.var_czbdswg_dn6, locals.var_czbdswg_dn7, locals.var_czbdswg_dn8, locals.var_czbdswg_dn9, locals.var_czbdswg_dn10, locals.var_czbdswg_dn13,)
    }
};
        locals.var_czbdswg = assign96770_e149392;
        locals.var_czbdswg_dn0 = assign96770_e149392_d_n0;
        locals.var_czbdswg_dn2 = assign96770_e149392_d_n2;
        locals.var_czbdswg_dn4 = assign96770_e149392_d_n4;
        locals.var_czbdswg_dn5 = assign96770_e149392_d_n5;
        locals.var_czbdswg_dn6 = assign96770_e149392_d_n6;
        locals.var_czbdswg_dn7 = assign96770_e149392_d_n7;
        locals.var_czbdswg_dn8 = assign96770_e149392_d_n8;
        locals.var_czbdswg_dn9 = assign96770_e149392_d_n9;
        locals.var_czbdswg_dn10 = assign96770_e149392_d_n10;
        locals.var_czbdswg_dn13 = assign96770_e149392_d_n13;
        locals.var_czbdswg_rv = 0.0;

        let assign96780_e149395: f64 = if locals.var_czbd < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2241 = assign96780_e149395;
        locals.var_guard2241_rv = 0.0;

        let (assign96790_e149401, assign96790_e149401_d_n0, assign96790_e149401_d_n2, assign96790_e149401_d_n4, assign96790_e149401_d_n5, assign96790_e149401_d_n6, assign96790_e149401_d_n7, assign96790_e149401_d_n8, assign96790_e149401_d_n9, assign96790_e149401_d_n10, assign96790_e149401_d_n13,) = {
    if ((locals.var_guard2233 != 0.0) && (locals.var_guard2241 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_czbd, locals.var_czbd_dn0, locals.var_czbd_dn2, locals.var_czbd_dn4, locals.var_czbd_dn5, locals.var_czbd_dn6, locals.var_czbd_dn7, locals.var_czbd_dn8, locals.var_czbd_dn9, locals.var_czbd_dn10, locals.var_czbd_dn13,)
    }
};
        locals.var_czbd = assign96790_e149401;
        locals.var_czbd_dn0 = assign96790_e149401_d_n0;
        locals.var_czbd_dn2 = assign96790_e149401_d_n2;
        locals.var_czbd_dn4 = assign96790_e149401_d_n4;
        locals.var_czbd_dn5 = assign96790_e149401_d_n5;
        locals.var_czbd_dn6 = assign96790_e149401_d_n6;
        locals.var_czbd_dn7 = assign96790_e149401_d_n7;
        locals.var_czbd_dn8 = assign96790_e149401_d_n8;
        locals.var_czbd_dn9 = assign96790_e149401_d_n9;
        locals.var_czbd_dn10 = assign96790_e149401_d_n10;
        locals.var_czbd_dn13 = assign96790_e149401_d_n13;
        locals.var_czbd_rv = 0.0;

        let assign96800_e149404: f64 = if locals.var_czbdsw < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2242 = assign96800_e149404;
        locals.var_guard2242_rv = 0.0;

        let (assign96810_e149410, assign96810_e149410_d_n0, assign96810_e149410_d_n2, assign96810_e149410_d_n4, assign96810_e149410_d_n5, assign96810_e149410_d_n6, assign96810_e149410_d_n7, assign96810_e149410_d_n8, assign96810_e149410_d_n9, assign96810_e149410_d_n10, assign96810_e149410_d_n13,) = {
    if ((locals.var_guard2233 != 0.0) && (locals.var_guard2242 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_czbdsw, locals.var_czbdsw_dn0, locals.var_czbdsw_dn2, locals.var_czbdsw_dn4, locals.var_czbdsw_dn5, locals.var_czbdsw_dn6, locals.var_czbdsw_dn7, locals.var_czbdsw_dn8, locals.var_czbdsw_dn9, locals.var_czbdsw_dn10, locals.var_czbdsw_dn13,)
    }
};
        locals.var_czbdsw = assign96810_e149410;
        locals.var_czbdsw_dn0 = assign96810_e149410_d_n0;
        locals.var_czbdsw_dn2 = assign96810_e149410_d_n2;
        locals.var_czbdsw_dn4 = assign96810_e149410_d_n4;
        locals.var_czbdsw_dn5 = assign96810_e149410_d_n5;
        locals.var_czbdsw_dn6 = assign96810_e149410_d_n6;
        locals.var_czbdsw_dn7 = assign96810_e149410_d_n7;
        locals.var_czbdsw_dn8 = assign96810_e149410_d_n8;
        locals.var_czbdsw_dn9 = assign96810_e149410_d_n9;
        locals.var_czbdsw_dn10 = assign96810_e149410_d_n10;
        locals.var_czbdsw_dn13 = assign96810_e149410_d_n13;
        locals.var_czbdsw_rv = 0.0;

        let assign96820_e149413: f64 = if locals.var_czbdswg < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2243 = assign96820_e149413;
        locals.var_guard2243_rv = 0.0;

        let (assign96830_e149419, assign96830_e149419_d_n0, assign96830_e149419_d_n2, assign96830_e149419_d_n4, assign96830_e149419_d_n5, assign96830_e149419_d_n6, assign96830_e149419_d_n7, assign96830_e149419_d_n8, assign96830_e149419_d_n9, assign96830_e149419_d_n10, assign96830_e149419_d_n13,) = {
    if ((locals.var_guard2233 != 0.0) && (locals.var_guard2243 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_czbdswg, locals.var_czbdswg_dn0, locals.var_czbdswg_dn2, locals.var_czbdswg_dn4, locals.var_czbdswg_dn5, locals.var_czbdswg_dn6, locals.var_czbdswg_dn7, locals.var_czbdswg_dn8, locals.var_czbdswg_dn9, locals.var_czbdswg_dn10, locals.var_czbdswg_dn13,)
    }
};
        locals.var_czbdswg = assign96830_e149419;
        locals.var_czbdswg_dn0 = assign96830_e149419_d_n0;
        locals.var_czbdswg_dn2 = assign96830_e149419_d_n2;
        locals.var_czbdswg_dn4 = assign96830_e149419_d_n4;
        locals.var_czbdswg_dn5 = assign96830_e149419_d_n5;
        locals.var_czbdswg_dn6 = assign96830_e149419_d_n6;
        locals.var_czbdswg_dn7 = assign96830_e149419_d_n7;
        locals.var_czbdswg_dn8 = assign96830_e149419_d_n8;
        locals.var_czbdswg_dn9 = assign96830_e149419_d_n9;
        locals.var_czbdswg_dn10 = assign96830_e149419_d_n10;
        locals.var_czbdswg_dn13 = assign96830_e149419_d_n13;
        locals.var_czbdswg_rv = 0.0;

        let (assign96840_e149427, assign96840_e149427_d_n0, assign96840_e149427_d_n2, assign96840_e149427_d_n4, assign96840_e149427_d_n5, assign96840_e149427_d_n6, assign96840_e149427_d_n7, assign96840_e149427_d_n8, assign96840_e149427_d_n9, assign96840_e149427_d_n10, assign96840_e149427_d_n13,) = {
    if (locals.var_guard2233 != 0.0) {
        let assign96840_e149424: f64 = (p.p487 * locals.var_tdiff);
        let assign96840_e149425: f64 = (p.p506 - assign96840_e149424);
        (assign96840_e149425, (-(p.p487 * locals.var_tdiff_dn0)), (-(p.p487 * locals.var_tdiff_dn2)), (-(p.p487 * locals.var_tdiff_dn4)), (-(p.p487 * locals.var_tdiff_dn5)), (-(p.p487 * locals.var_tdiff_dn6)), (-(p.p487 * locals.var_tdiff_dn7)), (-(p.p487 * locals.var_tdiff_dn8)), (-(p.p487 * locals.var_tdiff_dn9)), (-(p.p487 * locals.var_tdiff_dn10)), (-(p.p487 * locals.var_tdiff_dn13)),)
    } else {
        (locals.var_pzbd, locals.var_pzbd_dn0, locals.var_pzbd_dn2, locals.var_pzbd_dn4, locals.var_pzbd_dn5, locals.var_pzbd_dn6, locals.var_pzbd_dn7, locals.var_pzbd_dn8, locals.var_pzbd_dn9, locals.var_pzbd_dn10, locals.var_pzbd_dn13,)
    }
};
        locals.var_pzbd = assign96840_e149427;
        locals.var_pzbd_dn0 = assign96840_e149427_d_n0;
        locals.var_pzbd_dn2 = assign96840_e149427_d_n2;
        locals.var_pzbd_dn4 = assign96840_e149427_d_n4;
        locals.var_pzbd_dn5 = assign96840_e149427_d_n5;
        locals.var_pzbd_dn6 = assign96840_e149427_d_n6;
        locals.var_pzbd_dn7 = assign96840_e149427_d_n7;
        locals.var_pzbd_dn8 = assign96840_e149427_d_n8;
        locals.var_pzbd_dn9 = assign96840_e149427_d_n9;
        locals.var_pzbd_dn10 = assign96840_e149427_d_n10;
        locals.var_pzbd_dn13 = assign96840_e149427_d_n13;
        locals.var_pzbd_rv = 0.0;

        let (assign96850_e149435, assign96850_e149435_d_n0, assign96850_e149435_d_n2, assign96850_e149435_d_n4, assign96850_e149435_d_n5, assign96850_e149435_d_n6, assign96850_e149435_d_n7, assign96850_e149435_d_n8, assign96850_e149435_d_n9, assign96850_e149435_d_n10, assign96850_e149435_d_n13,) = {
    if (locals.var_guard2233 != 0.0) {
        let assign96850_e149432: f64 = (p.p489 * locals.var_tdiff);
        let assign96850_e149433: f64 = (p.p507 - assign96850_e149432);
        (assign96850_e149433, (-(p.p489 * locals.var_tdiff_dn0)), (-(p.p489 * locals.var_tdiff_dn2)), (-(p.p489 * locals.var_tdiff_dn4)), (-(p.p489 * locals.var_tdiff_dn5)), (-(p.p489 * locals.var_tdiff_dn6)), (-(p.p489 * locals.var_tdiff_dn7)), (-(p.p489 * locals.var_tdiff_dn8)), (-(p.p489 * locals.var_tdiff_dn9)), (-(p.p489 * locals.var_tdiff_dn10)), (-(p.p489 * locals.var_tdiff_dn13)),)
    } else {
        (locals.var_pzbdsw, locals.var_pzbdsw_dn0, locals.var_pzbdsw_dn2, locals.var_pzbdsw_dn4, locals.var_pzbdsw_dn5, locals.var_pzbdsw_dn6, locals.var_pzbdsw_dn7, locals.var_pzbdsw_dn8, locals.var_pzbdsw_dn9, locals.var_pzbdsw_dn10, locals.var_pzbdsw_dn13,)
    }
};
        locals.var_pzbdsw = assign96850_e149435;
        locals.var_pzbdsw_dn0 = assign96850_e149435_d_n0;
        locals.var_pzbdsw_dn2 = assign96850_e149435_d_n2;
        locals.var_pzbdsw_dn4 = assign96850_e149435_d_n4;
        locals.var_pzbdsw_dn5 = assign96850_e149435_d_n5;
        locals.var_pzbdsw_dn6 = assign96850_e149435_d_n6;
        locals.var_pzbdsw_dn7 = assign96850_e149435_d_n7;
        locals.var_pzbdsw_dn8 = assign96850_e149435_d_n8;
        locals.var_pzbdsw_dn9 = assign96850_e149435_d_n9;
        locals.var_pzbdsw_dn10 = assign96850_e149435_d_n10;
        locals.var_pzbdsw_dn13 = assign96850_e149435_d_n13;
        locals.var_pzbdsw_rv = 0.0;

        let (assign96860_e149443, assign96860_e149443_d_n0, assign96860_e149443_d_n2, assign96860_e149443_d_n4, assign96860_e149443_d_n5, assign96860_e149443_d_n6, assign96860_e149443_d_n7, assign96860_e149443_d_n8, assign96860_e149443_d_n9, assign96860_e149443_d_n10, assign96860_e149443_d_n13,) = {
    if (locals.var_guard2233 != 0.0) {
        let assign96860_e149440: f64 = (p.p491 * locals.var_tdiff);
        let assign96860_e149441: f64 = (p.p508 - assign96860_e149440);
        (assign96860_e149441, (-(p.p491 * locals.var_tdiff_dn0)), (-(p.p491 * locals.var_tdiff_dn2)), (-(p.p491 * locals.var_tdiff_dn4)), (-(p.p491 * locals.var_tdiff_dn5)), (-(p.p491 * locals.var_tdiff_dn6)), (-(p.p491 * locals.var_tdiff_dn7)), (-(p.p491 * locals.var_tdiff_dn8)), (-(p.p491 * locals.var_tdiff_dn9)), (-(p.p491 * locals.var_tdiff_dn10)), (-(p.p491 * locals.var_tdiff_dn13)),)
    } else {
        (locals.var_pzbdswg, locals.var_pzbdswg_dn0, locals.var_pzbdswg_dn2, locals.var_pzbdswg_dn4, locals.var_pzbdswg_dn5, locals.var_pzbdswg_dn6, locals.var_pzbdswg_dn7, locals.var_pzbdswg_dn8, locals.var_pzbdswg_dn9, locals.var_pzbdswg_dn10, locals.var_pzbdswg_dn13,)
    }
};
        locals.var_pzbdswg = assign96860_e149443;
        locals.var_pzbdswg_dn0 = assign96860_e149443_d_n0;
        locals.var_pzbdswg_dn2 = assign96860_e149443_d_n2;
        locals.var_pzbdswg_dn4 = assign96860_e149443_d_n4;
        locals.var_pzbdswg_dn5 = assign96860_e149443_d_n5;
        locals.var_pzbdswg_dn6 = assign96860_e149443_d_n6;
        locals.var_pzbdswg_dn7 = assign96860_e149443_d_n7;
        locals.var_pzbdswg_dn8 = assign96860_e149443_d_n8;
        locals.var_pzbdswg_dn9 = assign96860_e149443_d_n9;
        locals.var_pzbdswg_dn10 = assign96860_e149443_d_n10;
        locals.var_pzbdswg_dn13 = assign96860_e149443_d_n13;
        locals.var_pzbdswg_rv = 0.0;

        let assign96870_e149450: f64 = if ((locals.var_pzbd < 0.01) && (p.p13 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard2244 = assign96870_e149450;
        locals.var_guard2244_rv = 0.0;

        let (assign96880_e149456, assign96880_e149456_d_n0, assign96880_e149456_d_n2, assign96880_e149456_d_n4, assign96880_e149456_d_n5, assign96880_e149456_d_n6, assign96880_e149456_d_n7, assign96880_e149456_d_n8, assign96880_e149456_d_n9, assign96880_e149456_d_n10, assign96880_e149456_d_n13,) = {
    if ((locals.var_guard2233 != 0.0) && (locals.var_guard2244 != 0.0)) {
        (0.01, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_pzbd, locals.var_pzbd_dn0, locals.var_pzbd_dn2, locals.var_pzbd_dn4, locals.var_pzbd_dn5, locals.var_pzbd_dn6, locals.var_pzbd_dn7, locals.var_pzbd_dn8, locals.var_pzbd_dn9, locals.var_pzbd_dn10, locals.var_pzbd_dn13,)
    }
};
        locals.var_pzbd = assign96880_e149456;
        locals.var_pzbd_dn0 = assign96880_e149456_d_n0;
        locals.var_pzbd_dn2 = assign96880_e149456_d_n2;
        locals.var_pzbd_dn4 = assign96880_e149456_d_n4;
        locals.var_pzbd_dn5 = assign96880_e149456_d_n5;
        locals.var_pzbd_dn6 = assign96880_e149456_d_n6;
        locals.var_pzbd_dn7 = assign96880_e149456_d_n7;
        locals.var_pzbd_dn8 = assign96880_e149456_d_n8;
        locals.var_pzbd_dn9 = assign96880_e149456_d_n9;
        locals.var_pzbd_dn10 = assign96880_e149456_d_n10;
        locals.var_pzbd_dn13 = assign96880_e149456_d_n13;
        locals.var_pzbd_rv = 0.0;

        let assign96890_e149463: f64 = if ((locals.var_pzbdsw < 0.01) && (p.p15 > locals.var_weff_nf)) { 1.0 } else { 0.0 };
        locals.var_guard2245 = assign96890_e149463;
        locals.var_guard2245_rv = 0.0;

        let (assign96900_e149469, assign96900_e149469_d_n0, assign96900_e149469_d_n2, assign96900_e149469_d_n4, assign96900_e149469_d_n5, assign96900_e149469_d_n6, assign96900_e149469_d_n7, assign96900_e149469_d_n8, assign96900_e149469_d_n9, assign96900_e149469_d_n10, assign96900_e149469_d_n13,) = {
    if ((locals.var_guard2233 != 0.0) && (locals.var_guard2245 != 0.0)) {
        (0.01, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_pzbdsw, locals.var_pzbdsw_dn0, locals.var_pzbdsw_dn2, locals.var_pzbdsw_dn4, locals.var_pzbdsw_dn5, locals.var_pzbdsw_dn6, locals.var_pzbdsw_dn7, locals.var_pzbdsw_dn8, locals.var_pzbdsw_dn9, locals.var_pzbdsw_dn10, locals.var_pzbdsw_dn13,)
    }
};
        locals.var_pzbdsw = assign96900_e149469;
        locals.var_pzbdsw_dn0 = assign96900_e149469_d_n0;
        locals.var_pzbdsw_dn2 = assign96900_e149469_d_n2;
        locals.var_pzbdsw_dn4 = assign96900_e149469_d_n4;
        locals.var_pzbdsw_dn5 = assign96900_e149469_d_n5;
        locals.var_pzbdsw_dn6 = assign96900_e149469_d_n6;
        locals.var_pzbdsw_dn7 = assign96900_e149469_d_n7;
        locals.var_pzbdsw_dn8 = assign96900_e149469_d_n8;
        locals.var_pzbdsw_dn9 = assign96900_e149469_d_n9;
        locals.var_pzbdsw_dn10 = assign96900_e149469_d_n10;
        locals.var_pzbdsw_dn13 = assign96900_e149469_d_n13;
        locals.var_pzbdsw_rv = 0.0;

        let assign96910_e149476: f64 = if ((locals.var_pzbdswg < 0.01) && (p.p15 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard2246 = assign96910_e149476;
        locals.var_guard2246_rv = 0.0;

        let (assign96920_e149482, assign96920_e149482_d_n0, assign96920_e149482_d_n2, assign96920_e149482_d_n4, assign96920_e149482_d_n5, assign96920_e149482_d_n6, assign96920_e149482_d_n7, assign96920_e149482_d_n8, assign96920_e149482_d_n9, assign96920_e149482_d_n10, assign96920_e149482_d_n13,) = {
    if ((locals.var_guard2233 != 0.0) && (locals.var_guard2246 != 0.0)) {
        (0.01, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_pzbdswg, locals.var_pzbdswg_dn0, locals.var_pzbdswg_dn2, locals.var_pzbdswg_dn4, locals.var_pzbdswg_dn5, locals.var_pzbdswg_dn6, locals.var_pzbdswg_dn7, locals.var_pzbdswg_dn8, locals.var_pzbdswg_dn9, locals.var_pzbdswg_dn10, locals.var_pzbdswg_dn13,)
    }
};
        locals.var_pzbdswg = assign96920_e149482;
        locals.var_pzbdswg_dn0 = assign96920_e149482_d_n0;
        locals.var_pzbdswg_dn2 = assign96920_e149482_d_n2;
        locals.var_pzbdswg_dn4 = assign96920_e149482_d_n4;
        locals.var_pzbdswg_dn5 = assign96920_e149482_d_n5;
        locals.var_pzbdswg_dn6 = assign96920_e149482_d_n6;
        locals.var_pzbdswg_dn7 = assign96920_e149482_d_n7;
        locals.var_pzbdswg_dn8 = assign96920_e149482_d_n8;
        locals.var_pzbdswg_dn9 = assign96920_e149482_d_n9;
        locals.var_pzbdswg_dn10 = assign96920_e149482_d_n10;
        locals.var_pzbdswg_dn13 = assign96920_e149482_d_n13;
        locals.var_pzbdswg_rv = 0.0;

        let (assign96930_e149494, assign96930_e149494_d_n0, assign96930_e149494_d_n2, assign96930_e149494_d_n4, assign96930_e149494_d_n5, assign96930_e149494_d_n6, assign96930_e149494_d_n7, assign96930_e149494_d_n8, assign96930_e149494_d_n9, assign96930_e149494_d_n10, assign96930_e149494_d_n13,) = {
    if (locals.var_guard2233 != 0.0) {
        let assign96930_e149486: f64 = (p.p523 * p.p14);
        let assign96930_e149490: f64 = (p.p482 * locals.var_tdiff);
        let assign96930_e149491: f64 = (1.0 + assign96930_e149490);
        let assign96930_e149492: f64 = (assign96930_e149486 * assign96930_e149491);
        (assign96930_e149492, (assign96930_e149486 * (p.p482 * locals.var_tdiff_dn0)), (assign96930_e149486 * (p.p482 * locals.var_tdiff_dn2)), (assign96930_e149486 * (p.p482 * locals.var_tdiff_dn4)), (assign96930_e149486 * (p.p482 * locals.var_tdiff_dn5)), (assign96930_e149486 * (p.p482 * locals.var_tdiff_dn6)), (assign96930_e149486 * (p.p482 * locals.var_tdiff_dn7)), (assign96930_e149486 * (p.p482 * locals.var_tdiff_dn8)), (assign96930_e149486 * (p.p482 * locals.var_tdiff_dn9)), (assign96930_e149486 * (p.p482 * locals.var_tdiff_dn10)), (assign96930_e149486 * (p.p482 * locals.var_tdiff_dn13)),)
    } else {
        (locals.var_czbs, locals.var_czbs_dn0, locals.var_czbs_dn2, locals.var_czbs_dn4, locals.var_czbs_dn5, locals.var_czbs_dn6, locals.var_czbs_dn7, locals.var_czbs_dn8, locals.var_czbs_dn9, locals.var_czbs_dn10, locals.var_czbs_dn13,)
    }
};
        locals.var_czbs = assign96930_e149494;
        locals.var_czbs_dn0 = assign96930_e149494_d_n0;
        locals.var_czbs_dn2 = assign96930_e149494_d_n2;
        locals.var_czbs_dn4 = assign96930_e149494_d_n4;
        locals.var_czbs_dn5 = assign96930_e149494_d_n5;
        locals.var_czbs_dn6 = assign96930_e149494_d_n6;
        locals.var_czbs_dn7 = assign96930_e149494_d_n7;
        locals.var_czbs_dn8 = assign96930_e149494_d_n8;
        locals.var_czbs_dn9 = assign96930_e149494_d_n9;
        locals.var_czbs_dn10 = assign96930_e149494_d_n10;
        locals.var_czbs_dn13 = assign96930_e149494_d_n13;
        locals.var_czbs_rv = 0.0;

        let assign96940_e149497: f64 = if p.p16 > locals.var_weff_nf { 1.0 } else { 0.0 };
        locals.var_guard2247 = assign96940_e149497;
        locals.var_guard2247_rv = 0.0;

        let (assign96950_e149513, assign96950_e149513_d_n0, assign96950_e149513_d_n2, assign96950_e149513_d_n4, assign96950_e149513_d_n5, assign96950_e149513_d_n6, assign96950_e149513_d_n7, assign96950_e149513_d_n8, assign96950_e149513_d_n9, assign96950_e149513_d_n10, assign96950_e149513_d_n13,) = {
    if ((locals.var_guard2233 != 0.0) && (locals.var_guard2247 != 0.0)) {
        let assign96950_e149504: f64 = (p.p16 - locals.var_weff_nf);
        let assign96950_e149505: f64 = (p.p524 * assign96950_e149504);
        let assign96950_e149509: f64 = (p.p484 * locals.var_tdiff);
        let assign96950_e149510: f64 = (1.0 + assign96950_e149509);
        let assign96950_e149511: f64 = (assign96950_e149505 * assign96950_e149510);
        (assign96950_e149511, (assign96950_e149505 * (p.p484 * locals.var_tdiff_dn0)), (assign96950_e149505 * (p.p484 * locals.var_tdiff_dn2)), (assign96950_e149505 * (p.p484 * locals.var_tdiff_dn4)), (assign96950_e149505 * (p.p484 * locals.var_tdiff_dn5)), (assign96950_e149505 * (p.p484 * locals.var_tdiff_dn6)), (assign96950_e149505 * (p.p484 * locals.var_tdiff_dn7)), (assign96950_e149505 * (p.p484 * locals.var_tdiff_dn8)), (assign96950_e149505 * (p.p484 * locals.var_tdiff_dn9)), (assign96950_e149505 * (p.p484 * locals.var_tdiff_dn10)), (assign96950_e149505 * (p.p484 * locals.var_tdiff_dn13)),)
    } else {
        (locals.var_czbssw, locals.var_czbssw_dn0, locals.var_czbssw_dn2, locals.var_czbssw_dn4, locals.var_czbssw_dn5, locals.var_czbssw_dn6, locals.var_czbssw_dn7, locals.var_czbssw_dn8, locals.var_czbssw_dn9, locals.var_czbssw_dn10, locals.var_czbssw_dn13,)
    }
};
        locals.var_czbssw = assign96950_e149513;
        locals.var_czbssw_dn0 = assign96950_e149513_d_n0;
        locals.var_czbssw_dn2 = assign96950_e149513_d_n2;
        locals.var_czbssw_dn4 = assign96950_e149513_d_n4;
        locals.var_czbssw_dn5 = assign96950_e149513_d_n5;
        locals.var_czbssw_dn6 = assign96950_e149513_d_n6;
        locals.var_czbssw_dn7 = assign96950_e149513_d_n7;
        locals.var_czbssw_dn8 = assign96950_e149513_d_n8;
        locals.var_czbssw_dn9 = assign96950_e149513_d_n9;
        locals.var_czbssw_dn10 = assign96950_e149513_d_n10;
        locals.var_czbssw_dn13 = assign96950_e149513_d_n13;
        locals.var_czbssw_rv = 0.0;

        let (assign96960_e149527, assign96960_e149527_d_n0, assign96960_e149527_d_n2, assign96960_e149527_d_n4, assign96960_e149527_d_n5, assign96960_e149527_d_n6, assign96960_e149527_d_n7, assign96960_e149527_d_n8, assign96960_e149527_d_n9, assign96960_e149527_d_n10, assign96960_e149527_d_n13,) = {
    if ((locals.var_guard2233 != 0.0) && (locals.var_guard2247 != 0.0)) {
        let assign96960_e149519: f64 = (p.p525 * locals.var_weff_nf);
        let assign96960_e149523: f64 = (p.p486 * locals.var_tdiff);
        let assign96960_e149524: f64 = (1.0 + assign96960_e149523);
        let assign96960_e149525: f64 = (assign96960_e149519 * assign96960_e149524);
        (assign96960_e149525, (assign96960_e149519 * (p.p486 * locals.var_tdiff_dn0)), (assign96960_e149519 * (p.p486 * locals.var_tdiff_dn2)), (assign96960_e149519 * (p.p486 * locals.var_tdiff_dn4)), (assign96960_e149519 * (p.p486 * locals.var_tdiff_dn5)), (assign96960_e149519 * (p.p486 * locals.var_tdiff_dn6)), (assign96960_e149519 * (p.p486 * locals.var_tdiff_dn7)), (assign96960_e149519 * (p.p486 * locals.var_tdiff_dn8)), (assign96960_e149519 * (p.p486 * locals.var_tdiff_dn9)), (assign96960_e149519 * (p.p486 * locals.var_tdiff_dn10)), (assign96960_e149519 * (p.p486 * locals.var_tdiff_dn13)),)
    } else {
        (locals.var_czbsswg, locals.var_czbsswg_dn0, locals.var_czbsswg_dn2, locals.var_czbsswg_dn4, locals.var_czbsswg_dn5, locals.var_czbsswg_dn6, locals.var_czbsswg_dn7, locals.var_czbsswg_dn8, locals.var_czbsswg_dn9, locals.var_czbsswg_dn10, locals.var_czbsswg_dn13,)
    }
};
        locals.var_czbsswg = assign96960_e149527;
        locals.var_czbsswg_dn0 = assign96960_e149527_d_n0;
        locals.var_czbsswg_dn2 = assign96960_e149527_d_n2;
        locals.var_czbsswg_dn4 = assign96960_e149527_d_n4;
        locals.var_czbsswg_dn5 = assign96960_e149527_d_n5;
        locals.var_czbsswg_dn6 = assign96960_e149527_d_n6;
        locals.var_czbsswg_dn7 = assign96960_e149527_d_n7;
        locals.var_czbsswg_dn8 = assign96960_e149527_d_n8;
        locals.var_czbsswg_dn9 = assign96960_e149527_d_n9;
        locals.var_czbsswg_dn10 = assign96960_e149527_d_n10;
        locals.var_czbsswg_dn13 = assign96960_e149527_d_n13;
        locals.var_czbsswg_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_364(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let ctx_temp = ctx.temperature();
        let (assign96970_e149534, assign96970_e149534_d_n0, assign96970_e149534_d_n2, assign96970_e149534_d_n4, assign96970_e149534_d_n5, assign96970_e149534_d_n6, assign96970_e149534_d_n7, assign96970_e149534_d_n8, assign96970_e149534_d_n9, assign96970_e149534_d_n10, assign96970_e149534_d_n13,) = {
    if ((locals.var_guard2233 != 0.0) && (locals.var_guard2247 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_czbssw, locals.var_czbssw_dn0, locals.var_czbssw_dn2, locals.var_czbssw_dn4, locals.var_czbssw_dn5, locals.var_czbssw_dn6, locals.var_czbssw_dn7, locals.var_czbssw_dn8, locals.var_czbssw_dn9, locals.var_czbssw_dn10, locals.var_czbssw_dn13,)
    }
};
        locals.var_czbssw = assign96970_e149534;
        locals.var_czbssw_dn0 = assign96970_e149534_d_n0;
        locals.var_czbssw_dn2 = assign96970_e149534_d_n2;
        locals.var_czbssw_dn4 = assign96970_e149534_d_n4;
        locals.var_czbssw_dn5 = assign96970_e149534_d_n5;
        locals.var_czbssw_dn6 = assign96970_e149534_d_n6;
        locals.var_czbssw_dn7 = assign96970_e149534_d_n7;
        locals.var_czbssw_dn8 = assign96970_e149534_d_n8;
        locals.var_czbssw_dn9 = assign96970_e149534_d_n9;
        locals.var_czbssw_dn10 = assign96970_e149534_d_n10;
        locals.var_czbssw_dn13 = assign96970_e149534_d_n13;
        locals.var_czbssw_rv = 0.0;

        let (assign96980_e149549, assign96980_e149549_d_n0, assign96980_e149549_d_n2, assign96980_e149549_d_n4, assign96980_e149549_d_n5, assign96980_e149549_d_n6, assign96980_e149549_d_n7, assign96980_e149549_d_n8, assign96980_e149549_d_n9, assign96980_e149549_d_n10, assign96980_e149549_d_n13,) = {
    if ((locals.var_guard2233 != 0.0) && (locals.var_guard2247 == 0.0)) {
        let assign96980_e149541: f64 = (p.p525 * p.p16);
        let assign96980_e149545: f64 = (p.p486 * locals.var_tdiff);
        let assign96980_e149546: f64 = (1.0 + assign96980_e149545);
        let assign96980_e149547: f64 = (assign96980_e149541 * assign96980_e149546);
        (assign96980_e149547, (assign96980_e149541 * (p.p486 * locals.var_tdiff_dn0)), (assign96980_e149541 * (p.p486 * locals.var_tdiff_dn2)), (assign96980_e149541 * (p.p486 * locals.var_tdiff_dn4)), (assign96980_e149541 * (p.p486 * locals.var_tdiff_dn5)), (assign96980_e149541 * (p.p486 * locals.var_tdiff_dn6)), (assign96980_e149541 * (p.p486 * locals.var_tdiff_dn7)), (assign96980_e149541 * (p.p486 * locals.var_tdiff_dn8)), (assign96980_e149541 * (p.p486 * locals.var_tdiff_dn9)), (assign96980_e149541 * (p.p486 * locals.var_tdiff_dn10)), (assign96980_e149541 * (p.p486 * locals.var_tdiff_dn13)),)
    } else {
        (locals.var_czbsswg, locals.var_czbsswg_dn0, locals.var_czbsswg_dn2, locals.var_czbsswg_dn4, locals.var_czbsswg_dn5, locals.var_czbsswg_dn6, locals.var_czbsswg_dn7, locals.var_czbsswg_dn8, locals.var_czbsswg_dn9, locals.var_czbsswg_dn10, locals.var_czbsswg_dn13,)
    }
};
        locals.var_czbsswg = assign96980_e149549;
        locals.var_czbsswg_dn0 = assign96980_e149549_d_n0;
        locals.var_czbsswg_dn2 = assign96980_e149549_d_n2;
        locals.var_czbsswg_dn4 = assign96980_e149549_d_n4;
        locals.var_czbsswg_dn5 = assign96980_e149549_d_n5;
        locals.var_czbsswg_dn6 = assign96980_e149549_d_n6;
        locals.var_czbsswg_dn7 = assign96980_e149549_d_n7;
        locals.var_czbsswg_dn8 = assign96980_e149549_d_n8;
        locals.var_czbsswg_dn9 = assign96980_e149549_d_n9;
        locals.var_czbsswg_dn10 = assign96980_e149549_d_n10;
        locals.var_czbsswg_dn13 = assign96980_e149549_d_n13;
        locals.var_czbsswg_rv = 0.0;

        let assign96990_e149552: f64 = if locals.var_czbs < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2248 = assign96990_e149552;
        locals.var_guard2248_rv = 0.0;

        let (assign97000_e149558, assign97000_e149558_d_n0, assign97000_e149558_d_n2, assign97000_e149558_d_n4, assign97000_e149558_d_n5, assign97000_e149558_d_n6, assign97000_e149558_d_n7, assign97000_e149558_d_n8, assign97000_e149558_d_n9, assign97000_e149558_d_n10, assign97000_e149558_d_n13,) = {
    if ((locals.var_guard2233 != 0.0) && (locals.var_guard2248 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_czbs, locals.var_czbs_dn0, locals.var_czbs_dn2, locals.var_czbs_dn4, locals.var_czbs_dn5, locals.var_czbs_dn6, locals.var_czbs_dn7, locals.var_czbs_dn8, locals.var_czbs_dn9, locals.var_czbs_dn10, locals.var_czbs_dn13,)
    }
};
        locals.var_czbs = assign97000_e149558;
        locals.var_czbs_dn0 = assign97000_e149558_d_n0;
        locals.var_czbs_dn2 = assign97000_e149558_d_n2;
        locals.var_czbs_dn4 = assign97000_e149558_d_n4;
        locals.var_czbs_dn5 = assign97000_e149558_d_n5;
        locals.var_czbs_dn6 = assign97000_e149558_d_n6;
        locals.var_czbs_dn7 = assign97000_e149558_d_n7;
        locals.var_czbs_dn8 = assign97000_e149558_d_n8;
        locals.var_czbs_dn9 = assign97000_e149558_d_n9;
        locals.var_czbs_dn10 = assign97000_e149558_d_n10;
        locals.var_czbs_dn13 = assign97000_e149558_d_n13;
        locals.var_czbs_rv = 0.0;

        let assign97010_e149561: f64 = if locals.var_czbssw < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2249 = assign97010_e149561;
        locals.var_guard2249_rv = 0.0;

        let (assign97020_e149567, assign97020_e149567_d_n0, assign97020_e149567_d_n2, assign97020_e149567_d_n4, assign97020_e149567_d_n5, assign97020_e149567_d_n6, assign97020_e149567_d_n7, assign97020_e149567_d_n8, assign97020_e149567_d_n9, assign97020_e149567_d_n10, assign97020_e149567_d_n13,) = {
    if ((locals.var_guard2233 != 0.0) && (locals.var_guard2249 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_czbssw, locals.var_czbssw_dn0, locals.var_czbssw_dn2, locals.var_czbssw_dn4, locals.var_czbssw_dn5, locals.var_czbssw_dn6, locals.var_czbssw_dn7, locals.var_czbssw_dn8, locals.var_czbssw_dn9, locals.var_czbssw_dn10, locals.var_czbssw_dn13,)
    }
};
        locals.var_czbssw = assign97020_e149567;
        locals.var_czbssw_dn0 = assign97020_e149567_d_n0;
        locals.var_czbssw_dn2 = assign97020_e149567_d_n2;
        locals.var_czbssw_dn4 = assign97020_e149567_d_n4;
        locals.var_czbssw_dn5 = assign97020_e149567_d_n5;
        locals.var_czbssw_dn6 = assign97020_e149567_d_n6;
        locals.var_czbssw_dn7 = assign97020_e149567_d_n7;
        locals.var_czbssw_dn8 = assign97020_e149567_d_n8;
        locals.var_czbssw_dn9 = assign97020_e149567_d_n9;
        locals.var_czbssw_dn10 = assign97020_e149567_d_n10;
        locals.var_czbssw_dn13 = assign97020_e149567_d_n13;
        locals.var_czbssw_rv = 0.0;

        let assign97030_e149570: f64 = if locals.var_czbsswg < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2250 = assign97030_e149570;
        locals.var_guard2250_rv = 0.0;

        let (assign97040_e149576, assign97040_e149576_d_n0, assign97040_e149576_d_n2, assign97040_e149576_d_n4, assign97040_e149576_d_n5, assign97040_e149576_d_n6, assign97040_e149576_d_n7, assign97040_e149576_d_n8, assign97040_e149576_d_n9, assign97040_e149576_d_n10, assign97040_e149576_d_n13,) = {
    if ((locals.var_guard2233 != 0.0) && (locals.var_guard2250 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_czbsswg, locals.var_czbsswg_dn0, locals.var_czbsswg_dn2, locals.var_czbsswg_dn4, locals.var_czbsswg_dn5, locals.var_czbsswg_dn6, locals.var_czbsswg_dn7, locals.var_czbsswg_dn8, locals.var_czbsswg_dn9, locals.var_czbsswg_dn10, locals.var_czbsswg_dn13,)
    }
};
        locals.var_czbsswg = assign97040_e149576;
        locals.var_czbsswg_dn0 = assign97040_e149576_d_n0;
        locals.var_czbsswg_dn2 = assign97040_e149576_d_n2;
        locals.var_czbsswg_dn4 = assign97040_e149576_d_n4;
        locals.var_czbsswg_dn5 = assign97040_e149576_d_n5;
        locals.var_czbsswg_dn6 = assign97040_e149576_d_n6;
        locals.var_czbsswg_dn7 = assign97040_e149576_d_n7;
        locals.var_czbsswg_dn8 = assign97040_e149576_d_n8;
        locals.var_czbsswg_dn9 = assign97040_e149576_d_n9;
        locals.var_czbsswg_dn10 = assign97040_e149576_d_n10;
        locals.var_czbsswg_dn13 = assign97040_e149576_d_n13;
        locals.var_czbsswg_rv = 0.0;

        let (assign97050_e149584, assign97050_e149584_d_n0, assign97050_e149584_d_n2, assign97050_e149584_d_n4, assign97050_e149584_d_n5, assign97050_e149584_d_n6, assign97050_e149584_d_n7, assign97050_e149584_d_n8, assign97050_e149584_d_n9, assign97050_e149584_d_n10, assign97050_e149584_d_n13,) = {
    if (locals.var_guard2233 != 0.0) {
        let assign97050_e149581: f64 = (p.p488 * locals.var_tdiff);
        let assign97050_e149582: f64 = (p.p529 - assign97050_e149581);
        (assign97050_e149582, (-(p.p488 * locals.var_tdiff_dn0)), (-(p.p488 * locals.var_tdiff_dn2)), (-(p.p488 * locals.var_tdiff_dn4)), (-(p.p488 * locals.var_tdiff_dn5)), (-(p.p488 * locals.var_tdiff_dn6)), (-(p.p488 * locals.var_tdiff_dn7)), (-(p.p488 * locals.var_tdiff_dn8)), (-(p.p488 * locals.var_tdiff_dn9)), (-(p.p488 * locals.var_tdiff_dn10)), (-(p.p488 * locals.var_tdiff_dn13)),)
    } else {
        (locals.var_pzbs, locals.var_pzbs_dn0, locals.var_pzbs_dn2, locals.var_pzbs_dn4, locals.var_pzbs_dn5, locals.var_pzbs_dn6, locals.var_pzbs_dn7, locals.var_pzbs_dn8, locals.var_pzbs_dn9, locals.var_pzbs_dn10, locals.var_pzbs_dn13,)
    }
};
        locals.var_pzbs = assign97050_e149584;
        locals.var_pzbs_dn0 = assign97050_e149584_d_n0;
        locals.var_pzbs_dn2 = assign97050_e149584_d_n2;
        locals.var_pzbs_dn4 = assign97050_e149584_d_n4;
        locals.var_pzbs_dn5 = assign97050_e149584_d_n5;
        locals.var_pzbs_dn6 = assign97050_e149584_d_n6;
        locals.var_pzbs_dn7 = assign97050_e149584_d_n7;
        locals.var_pzbs_dn8 = assign97050_e149584_d_n8;
        locals.var_pzbs_dn9 = assign97050_e149584_d_n9;
        locals.var_pzbs_dn10 = assign97050_e149584_d_n10;
        locals.var_pzbs_dn13 = assign97050_e149584_d_n13;
        locals.var_pzbs_rv = 0.0;

        let (assign97060_e149592, assign97060_e149592_d_n0, assign97060_e149592_d_n2, assign97060_e149592_d_n4, assign97060_e149592_d_n5, assign97060_e149592_d_n6, assign97060_e149592_d_n7, assign97060_e149592_d_n8, assign97060_e149592_d_n9, assign97060_e149592_d_n10, assign97060_e149592_d_n13,) = {
    if (locals.var_guard2233 != 0.0) {
        let assign97060_e149589: f64 = (p.p490 * locals.var_tdiff);
        let assign97060_e149590: f64 = (p.p530 - assign97060_e149589);
        (assign97060_e149590, (-(p.p490 * locals.var_tdiff_dn0)), (-(p.p490 * locals.var_tdiff_dn2)), (-(p.p490 * locals.var_tdiff_dn4)), (-(p.p490 * locals.var_tdiff_dn5)), (-(p.p490 * locals.var_tdiff_dn6)), (-(p.p490 * locals.var_tdiff_dn7)), (-(p.p490 * locals.var_tdiff_dn8)), (-(p.p490 * locals.var_tdiff_dn9)), (-(p.p490 * locals.var_tdiff_dn10)), (-(p.p490 * locals.var_tdiff_dn13)),)
    } else {
        (locals.var_pzbssw, locals.var_pzbssw_dn0, locals.var_pzbssw_dn2, locals.var_pzbssw_dn4, locals.var_pzbssw_dn5, locals.var_pzbssw_dn6, locals.var_pzbssw_dn7, locals.var_pzbssw_dn8, locals.var_pzbssw_dn9, locals.var_pzbssw_dn10, locals.var_pzbssw_dn13,)
    }
};
        locals.var_pzbssw = assign97060_e149592;
        locals.var_pzbssw_dn0 = assign97060_e149592_d_n0;
        locals.var_pzbssw_dn2 = assign97060_e149592_d_n2;
        locals.var_pzbssw_dn4 = assign97060_e149592_d_n4;
        locals.var_pzbssw_dn5 = assign97060_e149592_d_n5;
        locals.var_pzbssw_dn6 = assign97060_e149592_d_n6;
        locals.var_pzbssw_dn7 = assign97060_e149592_d_n7;
        locals.var_pzbssw_dn8 = assign97060_e149592_d_n8;
        locals.var_pzbssw_dn9 = assign97060_e149592_d_n9;
        locals.var_pzbssw_dn10 = assign97060_e149592_d_n10;
        locals.var_pzbssw_dn13 = assign97060_e149592_d_n13;
        locals.var_pzbssw_rv = 0.0;

        let (assign97070_e149600, assign97070_e149600_d_n0, assign97070_e149600_d_n2, assign97070_e149600_d_n4, assign97070_e149600_d_n5, assign97070_e149600_d_n6, assign97070_e149600_d_n7, assign97070_e149600_d_n8, assign97070_e149600_d_n9, assign97070_e149600_d_n10, assign97070_e149600_d_n13,) = {
    if (locals.var_guard2233 != 0.0) {
        let assign97070_e149597: f64 = (p.p492 * locals.var_tdiff);
        let assign97070_e149598: f64 = (p.p531 - assign97070_e149597);
        (assign97070_e149598, (-(p.p492 * locals.var_tdiff_dn0)), (-(p.p492 * locals.var_tdiff_dn2)), (-(p.p492 * locals.var_tdiff_dn4)), (-(p.p492 * locals.var_tdiff_dn5)), (-(p.p492 * locals.var_tdiff_dn6)), (-(p.p492 * locals.var_tdiff_dn7)), (-(p.p492 * locals.var_tdiff_dn8)), (-(p.p492 * locals.var_tdiff_dn9)), (-(p.p492 * locals.var_tdiff_dn10)), (-(p.p492 * locals.var_tdiff_dn13)),)
    } else {
        (locals.var_pzbsswg, locals.var_pzbsswg_dn0, locals.var_pzbsswg_dn2, locals.var_pzbsswg_dn4, locals.var_pzbsswg_dn5, locals.var_pzbsswg_dn6, locals.var_pzbsswg_dn7, locals.var_pzbsswg_dn8, locals.var_pzbsswg_dn9, locals.var_pzbsswg_dn10, locals.var_pzbsswg_dn13,)
    }
};
        locals.var_pzbsswg = assign97070_e149600;
        locals.var_pzbsswg_dn0 = assign97070_e149600_d_n0;
        locals.var_pzbsswg_dn2 = assign97070_e149600_d_n2;
        locals.var_pzbsswg_dn4 = assign97070_e149600_d_n4;
        locals.var_pzbsswg_dn5 = assign97070_e149600_d_n5;
        locals.var_pzbsswg_dn6 = assign97070_e149600_d_n6;
        locals.var_pzbsswg_dn7 = assign97070_e149600_d_n7;
        locals.var_pzbsswg_dn8 = assign97070_e149600_d_n8;
        locals.var_pzbsswg_dn9 = assign97070_e149600_d_n9;
        locals.var_pzbsswg_dn10 = assign97070_e149600_d_n10;
        locals.var_pzbsswg_dn13 = assign97070_e149600_d_n13;
        locals.var_pzbsswg_rv = 0.0;

        let assign97080_e149607: f64 = if ((locals.var_pzbs < 0.01) && (p.p14 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard2251 = assign97080_e149607;
        locals.var_guard2251_rv = 0.0;

        let (assign97090_e149613, assign97090_e149613_d_n0, assign97090_e149613_d_n2, assign97090_e149613_d_n4, assign97090_e149613_d_n5, assign97090_e149613_d_n6, assign97090_e149613_d_n7, assign97090_e149613_d_n8, assign97090_e149613_d_n9, assign97090_e149613_d_n10, assign97090_e149613_d_n13,) = {
    if ((locals.var_guard2233 != 0.0) && (locals.var_guard2251 != 0.0)) {
        (0.01, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_pzbs, locals.var_pzbs_dn0, locals.var_pzbs_dn2, locals.var_pzbs_dn4, locals.var_pzbs_dn5, locals.var_pzbs_dn6, locals.var_pzbs_dn7, locals.var_pzbs_dn8, locals.var_pzbs_dn9, locals.var_pzbs_dn10, locals.var_pzbs_dn13,)
    }
};
        locals.var_pzbs = assign97090_e149613;
        locals.var_pzbs_dn0 = assign97090_e149613_d_n0;
        locals.var_pzbs_dn2 = assign97090_e149613_d_n2;
        locals.var_pzbs_dn4 = assign97090_e149613_d_n4;
        locals.var_pzbs_dn5 = assign97090_e149613_d_n5;
        locals.var_pzbs_dn6 = assign97090_e149613_d_n6;
        locals.var_pzbs_dn7 = assign97090_e149613_d_n7;
        locals.var_pzbs_dn8 = assign97090_e149613_d_n8;
        locals.var_pzbs_dn9 = assign97090_e149613_d_n9;
        locals.var_pzbs_dn10 = assign97090_e149613_d_n10;
        locals.var_pzbs_dn13 = assign97090_e149613_d_n13;
        locals.var_pzbs_rv = 0.0;

        let assign97100_e149620: f64 = if ((locals.var_pzbssw < 0.01) && (p.p16 > locals.var_weff_nf)) { 1.0 } else { 0.0 };
        locals.var_guard2252 = assign97100_e149620;
        locals.var_guard2252_rv = 0.0;

        let (assign97110_e149626, assign97110_e149626_d_n0, assign97110_e149626_d_n2, assign97110_e149626_d_n4, assign97110_e149626_d_n5, assign97110_e149626_d_n6, assign97110_e149626_d_n7, assign97110_e149626_d_n8, assign97110_e149626_d_n9, assign97110_e149626_d_n10, assign97110_e149626_d_n13,) = {
    if ((locals.var_guard2233 != 0.0) && (locals.var_guard2252 != 0.0)) {
        (0.01, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_pzbssw, locals.var_pzbssw_dn0, locals.var_pzbssw_dn2, locals.var_pzbssw_dn4, locals.var_pzbssw_dn5, locals.var_pzbssw_dn6, locals.var_pzbssw_dn7, locals.var_pzbssw_dn8, locals.var_pzbssw_dn9, locals.var_pzbssw_dn10, locals.var_pzbssw_dn13,)
    }
};
        locals.var_pzbssw = assign97110_e149626;
        locals.var_pzbssw_dn0 = assign97110_e149626_d_n0;
        locals.var_pzbssw_dn2 = assign97110_e149626_d_n2;
        locals.var_pzbssw_dn4 = assign97110_e149626_d_n4;
        locals.var_pzbssw_dn5 = assign97110_e149626_d_n5;
        locals.var_pzbssw_dn6 = assign97110_e149626_d_n6;
        locals.var_pzbssw_dn7 = assign97110_e149626_d_n7;
        locals.var_pzbssw_dn8 = assign97110_e149626_d_n8;
        locals.var_pzbssw_dn9 = assign97110_e149626_d_n9;
        locals.var_pzbssw_dn10 = assign97110_e149626_d_n10;
        locals.var_pzbssw_dn13 = assign97110_e149626_d_n13;
        locals.var_pzbssw_rv = 0.0;

        let assign97120_e149633: f64 = if ((locals.var_pzbsswg < 0.01) && (p.p16 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard2253 = assign97120_e149633;
        locals.var_guard2253_rv = 0.0;

        let (assign97130_e149639, assign97130_e149639_d_n0, assign97130_e149639_d_n2, assign97130_e149639_d_n4, assign97130_e149639_d_n5, assign97130_e149639_d_n6, assign97130_e149639_d_n7, assign97130_e149639_d_n8, assign97130_e149639_d_n9, assign97130_e149639_d_n10, assign97130_e149639_d_n13,) = {
    if ((locals.var_guard2233 != 0.0) && (locals.var_guard2253 != 0.0)) {
        (0.01, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_pzbsswg, locals.var_pzbsswg_dn0, locals.var_pzbsswg_dn2, locals.var_pzbsswg_dn4, locals.var_pzbsswg_dn5, locals.var_pzbsswg_dn6, locals.var_pzbsswg_dn7, locals.var_pzbsswg_dn8, locals.var_pzbsswg_dn9, locals.var_pzbsswg_dn10, locals.var_pzbsswg_dn13,)
    }
};
        locals.var_pzbsswg = assign97130_e149639;
        locals.var_pzbsswg_dn0 = assign97130_e149639_d_n0;
        locals.var_pzbsswg_dn2 = assign97130_e149639_d_n2;
        locals.var_pzbsswg_dn4 = assign97130_e149639_d_n4;
        locals.var_pzbsswg_dn5 = assign97130_e149639_d_n5;
        locals.var_pzbsswg_dn6 = assign97130_e149639_d_n6;
        locals.var_pzbsswg_dn7 = assign97130_e149639_d_n7;
        locals.var_pzbsswg_dn8 = assign97130_e149639_d_n8;
        locals.var_pzbsswg_dn9 = assign97130_e149639_d_n9;
        locals.var_pzbsswg_dn10 = assign97130_e149639_d_n10;
        locals.var_pzbsswg_dn13 = assign97130_e149639_d_n13;
        locals.var_pzbsswg_rv = 0.0;

        let (assign97140_e149646, assign97140_e149646_d_n0, assign97140_e149646_d_n2, assign97140_e149646_d_n4, assign97140_e149646_d_n5, assign97140_e149646_d_n6, assign97140_e149646_d_n7, assign97140_e149646_d_n8, assign97140_e149646_d_n9, assign97140_e149646_d_n10, assign97140_e149646_d_n13,) = {
    if (locals.var_guard2233 == 0.0) {
        let assign97140_e149642: f64 = ctx_temp;
        let assign97140_e149644: f64 = (assign97140_e149642 + p.p11);
        (assign97140_e149644, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ttemp, locals.var_ttemp_dn0, locals.var_ttemp_dn2, locals.var_ttemp_dn4, locals.var_ttemp_dn5, locals.var_ttemp_dn6, locals.var_ttemp_dn7, locals.var_ttemp_dn8, locals.var_ttemp_dn9, locals.var_ttemp_dn10, locals.var_ttemp_dn13,)
    }
};
        locals.var_ttemp = assign97140_e149646;
        locals.var_ttemp_dn0 = assign97140_e149646_d_n0;
        locals.var_ttemp_dn2 = assign97140_e149646_d_n2;
        locals.var_ttemp_dn4 = assign97140_e149646_d_n4;
        locals.var_ttemp_dn5 = assign97140_e149646_d_n5;
        locals.var_ttemp_dn6 = assign97140_e149646_d_n6;
        locals.var_ttemp_dn7 = assign97140_e149646_d_n7;
        locals.var_ttemp_dn8 = assign97140_e149646_d_n8;
        locals.var_ttemp_dn9 = assign97140_e149646_d_n9;
        locals.var_ttemp_dn10 = assign97140_e149646_d_n10;
        locals.var_ttemp_dn13 = assign97140_e149646_d_n13;
        locals.var_ttemp_rv = 0.0;

        let assign97150_e149649: f64 = (p.p511 * locals.var_jd_nvtm_invd);
        locals.var_t10 = assign97150_e149649;
        locals.var_t10_dn0 = (p.p511 * locals.var_jd_nvtm_invd_dn0);
        locals.var_t10_dn2 = (p.p511 * locals.var_jd_nvtm_invd_dn2);
        locals.var_t10_dn4 = (p.p511 * locals.var_jd_nvtm_invd_dn4);
        locals.var_t10_dn5 = (p.p511 * locals.var_jd_nvtm_invd_dn5);
        locals.var_t10_dn6 = (p.p511 * locals.var_jd_nvtm_invd_dn6);
        locals.var_t10_dn7 = (p.p511 * locals.var_jd_nvtm_invd_dn7);
        locals.var_t10_dn8 = (p.p511 * locals.var_jd_nvtm_invd_dn8);
        locals.var_t10_dn9 = (p.p511 * locals.var_jd_nvtm_invd_dn9);
        locals.var_t10_dn10 = (p.p511 * locals.var_jd_nvtm_invd_dn10);
        locals.var_t10_dn13 = (p.p511 * locals.var_jd_nvtm_invd_dn13);
        locals.var_t10_rv = 0.0;

        let assign97160_e149652: f64 = (p.p510 * locals.var_exptempd);
        locals.var_t9 = assign97160_e149652;
        locals.var_t9_dn0 = (p.p510 * locals.var_exptempd_dn0);
        locals.var_t9_dn2 = (p.p510 * locals.var_exptempd_dn2);
        locals.var_t9_dn4 = (p.p510 * locals.var_exptempd_dn4);
        locals.var_t9_dn5 = (p.p510 * locals.var_exptempd_dn5);
        locals.var_t9_dn6 = (p.p510 * locals.var_exptempd_dn6);
        locals.var_t9_dn7 = (p.p510 * locals.var_exptempd_dn7);
        locals.var_t9_dn8 = (p.p510 * locals.var_exptempd_dn8);
        locals.var_t9_dn9 = (p.p510 * locals.var_exptempd_dn9);
        locals.var_t9_dn10 = (p.p510 * locals.var_exptempd_dn10);
        locals.var_t9_dn13 = (p.p510 * locals.var_exptempd_dn13);
        locals.var_t9_rv = 0.0;

        let assign97170_e149655: f64 = if locals.var_isbd_btm > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2254 = assign97170_e149655;
        locals.var_guard2254_rv = 0.0;

        let (assign97180_e149661, assign97180_e149661_d_n0, assign97180_e149661_d_n2, assign97180_e149661_d_n4, assign97180_e149661_d_n5, assign97180_e149661_d_n6, assign97180_e149661_d_n7, assign97180_e149661_d_n8, assign97180_e149661_d_n9, assign97180_e149661_d_n10, assign97180_e149661_d_n13,) = {
    if (locals.var_guard2254 != 0.0) {
        let assign97180_e149659: f64 = (locals.var_isbd2_btm * locals.var_t9);
        (assign97180_e149659, ((locals.var_isbd2_btm_dn0 * locals.var_t9) + (locals.var_isbd2_btm * locals.var_t9_dn0)), ((locals.var_isbd2_btm_dn2 * locals.var_t9) + (locals.var_isbd2_btm * locals.var_t9_dn2)), ((locals.var_isbd2_btm_dn4 * locals.var_t9) + (locals.var_isbd2_btm * locals.var_t9_dn4)), ((locals.var_isbd2_btm_dn5 * locals.var_t9) + (locals.var_isbd2_btm * locals.var_t9_dn5)), ((locals.var_isbd2_btm_dn6 * locals.var_t9) + (locals.var_isbd2_btm * locals.var_t9_dn6)), ((locals.var_isbd2_btm_dn7 * locals.var_t9) + (locals.var_isbd2_btm * locals.var_t9_dn7)), ((locals.var_isbd2_btm_dn8 * locals.var_t9) + (locals.var_isbd2_btm * locals.var_t9_dn8)), ((locals.var_isbd2_btm_dn9 * locals.var_t9) + (locals.var_isbd2_btm * locals.var_t9_dn9)), ((locals.var_isbd2_btm_dn10 * locals.var_t9) + (locals.var_isbd2_btm * locals.var_t9_dn10)), ((locals.var_isbd2_btm_dn13 * locals.var_t9) + (locals.var_isbd2_btm * locals.var_t9_dn13)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign97180_e149661;
        locals.var_t0_dn0 = assign97180_e149661_d_n0;
        locals.var_t0_dn2 = assign97180_e149661_d_n2;
        locals.var_t0_dn4 = assign97180_e149661_d_n4;
        locals.var_t0_dn5 = assign97180_e149661_d_n5;
        locals.var_t0_dn6 = assign97180_e149661_d_n6;
        locals.var_t0_dn7 = assign97180_e149661_d_n7;
        locals.var_t0_dn8 = assign97180_e149661_d_n8;
        locals.var_t0_dn9 = assign97180_e149661_d_n9;
        locals.var_t0_dn10 = assign97180_e149661_d_n10;
        locals.var_t0_dn13 = assign97180_e149661_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign97190_e149668, assign97190_e149668_d_n0, assign97190_e149668_d_n2, assign97190_e149668_d_n4, assign97190_e149668_d_n5, assign97190_e149668_d_n6, assign97190_e149668_d_n7, assign97190_e149668_d_n8, assign97190_e149668_d_n9, assign97190_e149668_d_n10, assign97190_e149668_d_n13,) = {
    if (locals.var_guard2254 != 0.0) {
        let assign97190_e149664: f64 = (-locals.var_vbd_jct);
        let assign97190_e149666: f64 = (assign97190_e149664 * locals.var_t10);
        (assign97190_e149666, (((-locals.var_vbd_jct_dn0) * locals.var_t10) + (assign97190_e149664 * locals.var_t10_dn0)), (assign97190_e149664 * locals.var_t10_dn2), (assign97190_e149664 * locals.var_t10_dn4), (assign97190_e149664 * locals.var_t10_dn5), (assign97190_e149664 * locals.var_t10_dn6), (assign97190_e149664 * locals.var_t10_dn7), (assign97190_e149664 * locals.var_t10_dn8), (((-locals.var_vbd_jct_dn9) * locals.var_t10) + (assign97190_e149664 * locals.var_t10_dn9)), (assign97190_e149664 * locals.var_t10_dn10), (assign97190_e149664 * locals.var_t10_dn13),)
    } else {
        (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn4, locals.var_tx_dn5, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn8, locals.var_tx_dn9, locals.var_tx_dn10, locals.var_tx_dn13,)
    }
};
        locals.var_tx = assign97190_e149668;
        locals.var_tx_dn0 = assign97190_e149668_d_n0;
        locals.var_tx_dn2 = assign97190_e149668_d_n2;
        locals.var_tx_dn4 = assign97190_e149668_d_n4;
        locals.var_tx_dn5 = assign97190_e149668_d_n5;
        locals.var_tx_dn6 = assign97190_e149668_d_n6;
        locals.var_tx_dn7 = assign97190_e149668_d_n7;
        locals.var_tx_dn8 = assign97190_e149668_d_n8;
        locals.var_tx_dn9 = assign97190_e149668_d_n9;
        locals.var_tx_dn10 = assign97190_e149668_d_n10;
        locals.var_tx_dn13 = assign97190_e149668_d_n13;
        locals.var_tx_rv = 0.0;

        let (assign97200_e149673, assign97200_e149673_d_n0, assign97200_e149673_d_n2, assign97200_e149673_d_n4, assign97200_e149673_d_n5, assign97200_e149673_d_n6, assign97200_e149673_d_n7, assign97200_e149673_d_n8, assign97200_e149673_d_n9, assign97200_e149673_d_n10, assign97200_e149673_d_n13,) = {
    if (locals.var_guard2254 != 0.0) {
        let assign97200_e149671: f64 = (locals.var_tx).exp();
        (assign97200_e149671, (assign97200_e149671 * locals.var_tx_dn0), (assign97200_e149671 * locals.var_tx_dn2), (assign97200_e149671 * locals.var_tx_dn4), (assign97200_e149671 * locals.var_tx_dn5), (assign97200_e149671 * locals.var_tx_dn6), (assign97200_e149671 * locals.var_tx_dn7), (assign97200_e149671 * locals.var_tx_dn8), (assign97200_e149671 * locals.var_tx_dn9), (assign97200_e149671 * locals.var_tx_dn10), (assign97200_e149671 * locals.var_tx_dn13),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign97200_e149673;
        locals.var_t2_dn0 = assign97200_e149673_d_n0;
        locals.var_t2_dn2 = assign97200_e149673_d_n2;
        locals.var_t2_dn4 = assign97200_e149673_d_n4;
        locals.var_t2_dn5 = assign97200_e149673_d_n5;
        locals.var_t2_dn6 = assign97200_e149673_d_n6;
        locals.var_t2_dn7 = assign97200_e149673_d_n7;
        locals.var_t2_dn8 = assign97200_e149673_d_n8;
        locals.var_t2_dn9 = assign97200_e149673_d_n9;
        locals.var_t2_dn10 = assign97200_e149673_d_n10;
        locals.var_t2_dn13 = assign97200_e149673_d_n13;
        locals.var_t2_rv = 0.0;

        let (assign97210_e149677, assign97210_e149677_d_n0, assign97210_e149677_d_n2, assign97210_e149677_d_n4, assign97210_e149677_d_n5, assign97210_e149677_d_n6, assign97210_e149677_d_n7, assign97210_e149677_d_n8, assign97210_e149677_d_n9, assign97210_e149677_d_n10, assign97210_e149677_d_n13,) = {
    if (locals.var_guard2254 != 0.0) {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn13,)
    }
};
        locals.var_t3 = assign97210_e149677;
        locals.var_t3_dn0 = assign97210_e149677_d_n0;
        locals.var_t3_dn2 = assign97210_e149677_d_n2;
        locals.var_t3_dn4 = assign97210_e149677_d_n4;
        locals.var_t3_dn5 = assign97210_e149677_d_n5;
        locals.var_t3_dn6 = assign97210_e149677_d_n6;
        locals.var_t3_dn7 = assign97210_e149677_d_n7;
        locals.var_t3_dn8 = assign97210_e149677_d_n8;
        locals.var_t3_dn9 = assign97210_e149677_d_n9;
        locals.var_t3_dn10 = assign97210_e149677_d_n10;
        locals.var_t3_dn13 = assign97210_e149677_d_n13;
        locals.var_t3_rv = 0.0;

        let assign97220_e149680: f64 = if locals.var_vbd_jct < locals.var_vbdt { 1.0 } else { 0.0 };
        locals.var_guard2255 = assign97220_e149680;
        locals.var_guard2255_rv = 0.0;

        let (assign97230_e149688, assign97230_e149688_d_n0, assign97230_e149688_d_n2, assign97230_e149688_d_n4, assign97230_e149688_d_n5, assign97230_e149688_d_n6, assign97230_e149688_d_n7, assign97230_e149688_d_n8, assign97230_e149688_d_n9, assign97230_e149688_d_n10, assign97230_e149688_d_n13,) = {
    if ((locals.var_guard2254 != 0.0) && (locals.var_guard2255 != 0.0)) {
        let assign97230_e149686: f64 = (locals.var_vbd_jct * locals.var_jd_nvtm_invd);
        (assign97230_e149686, ((locals.var_vbd_jct_dn0 * locals.var_jd_nvtm_invd) + (locals.var_vbd_jct * locals.var_jd_nvtm_invd_dn0)), (locals.var_vbd_jct * locals.var_jd_nvtm_invd_dn2), (locals.var_vbd_jct * locals.var_jd_nvtm_invd_dn4), (locals.var_vbd_jct * locals.var_jd_nvtm_invd_dn5), (locals.var_vbd_jct * locals.var_jd_nvtm_invd_dn6), (locals.var_vbd_jct * locals.var_jd_nvtm_invd_dn7), (locals.var_vbd_jct * locals.var_jd_nvtm_invd_dn8), ((locals.var_vbd_jct_dn9 * locals.var_jd_nvtm_invd) + (locals.var_vbd_jct * locals.var_jd_nvtm_invd_dn9)), (locals.var_vbd_jct * locals.var_jd_nvtm_invd_dn10), (locals.var_vbd_jct * locals.var_jd_nvtm_invd_dn13),)
    } else {
        (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn4, locals.var_tx_dn5, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn8, locals.var_tx_dn9, locals.var_tx_dn10, locals.var_tx_dn13,)
    }
};
        locals.var_tx = assign97230_e149688;
        locals.var_tx_dn0 = assign97230_e149688_d_n0;
        locals.var_tx_dn2 = assign97230_e149688_d_n2;
        locals.var_tx_dn4 = assign97230_e149688_d_n4;
        locals.var_tx_dn5 = assign97230_e149688_d_n5;
        locals.var_tx_dn6 = assign97230_e149688_d_n6;
        locals.var_tx_dn7 = assign97230_e149688_d_n7;
        locals.var_tx_dn8 = assign97230_e149688_d_n8;
        locals.var_tx_dn9 = assign97230_e149688_d_n9;
        locals.var_tx_dn10 = assign97230_e149688_d_n10;
        locals.var_tx_dn13 = assign97230_e149688_d_n13;
        locals.var_tx_rv = 0.0;

        let assign97240_e149691: f64 = (-3.0);
        let assign97240_e149693: f64 = (assign97240_e149691 * 34.0);
        let assign97240_e149694: f64 = if locals.var_tx < assign97240_e149693 { 1.0 } else { 0.0 };
        locals.var_guard2256 = assign97240_e149694;
        locals.var_guard2256_rv = 0.0;

        let (assign97250_e149702, assign97250_e149702_d_n0, assign97250_e149702_d_n2, assign97250_e149702_d_n4, assign97250_e149702_d_n5, assign97250_e149702_d_n6, assign97250_e149702_d_n7, assign97250_e149702_d_n8, assign97250_e149702_d_n9, assign97250_e149702_d_n10, assign97250_e149702_d_n13,) = {
    if (((locals.var_guard2254 != 0.0) && (locals.var_guard2255 != 0.0)) && (locals.var_guard2256 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign97250_e149702;
        locals.var_t1_dn0 = assign97250_e149702_d_n0;
        locals.var_t1_dn2 = assign97250_e149702_d_n2;
        locals.var_t1_dn4 = assign97250_e149702_d_n4;
        locals.var_t1_dn5 = assign97250_e149702_d_n5;
        locals.var_t1_dn6 = assign97250_e149702_d_n6;
        locals.var_t1_dn7 = assign97250_e149702_d_n7;
        locals.var_t1_dn8 = assign97250_e149702_d_n8;
        locals.var_t1_dn9 = assign97250_e149702_d_n9;
        locals.var_t1_dn10 = assign97250_e149702_d_n10;
        locals.var_t1_dn13 = assign97250_e149702_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign97260_e149712, assign97260_e149712_d_n0, assign97260_e149712_d_n2, assign97260_e149712_d_n4, assign97260_e149712_d_n5, assign97260_e149712_d_n6, assign97260_e149712_d_n7, assign97260_e149712_d_n8, assign97260_e149712_d_n9, assign97260_e149712_d_n10, assign97260_e149712_d_n13,) = {
    if (((locals.var_guard2254 != 0.0) && (locals.var_guard2255 != 0.0)) && (locals.var_guard2256 == 0.0)) {
        let assign97260_e149710: f64 = (locals.var_tx).exp();
        (assign97260_e149710, (assign97260_e149710 * locals.var_tx_dn0), (assign97260_e149710 * locals.var_tx_dn2), (assign97260_e149710 * locals.var_tx_dn4), (assign97260_e149710 * locals.var_tx_dn5), (assign97260_e149710 * locals.var_tx_dn6), (assign97260_e149710 * locals.var_tx_dn7), (assign97260_e149710 * locals.var_tx_dn8), (assign97260_e149710 * locals.var_tx_dn9), (assign97260_e149710 * locals.var_tx_dn10), (assign97260_e149710 * locals.var_tx_dn13),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign97260_e149712;
        locals.var_t1_dn0 = assign97260_e149712_d_n0;
        locals.var_t1_dn2 = assign97260_e149712_d_n2;
        locals.var_t1_dn4 = assign97260_e149712_d_n4;
        locals.var_t1_dn5 = assign97260_e149712_d_n5;
        locals.var_t1_dn6 = assign97260_e149712_d_n6;
        locals.var_t1_dn7 = assign97260_e149712_d_n7;
        locals.var_t1_dn8 = assign97260_e149712_d_n8;
        locals.var_t1_dn9 = assign97260_e149712_d_n9;
        locals.var_t1_dn10 = assign97260_e149712_d_n10;
        locals.var_t1_dn13 = assign97260_e149712_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign97280_e149741, assign97280_e149741_d_n0, assign97280_e149741_d_n2, assign97280_e149741_d_n4, assign97280_e149741_d_n5, assign97280_e149741_d_n6, assign97280_e149741_d_n7, assign97280_e149741_d_n8, assign97280_e149741_d_n9, assign97280_e149741_d_n10, assign97280_e149741_d_n13,) = {
    if ((locals.var_guard2254 != 0.0) && (locals.var_guard2255 == 0.0)) {
        (locals.var_jd_expcd, locals.var_jd_expcd_dn0, locals.var_jd_expcd_dn2, locals.var_jd_expcd_dn4, locals.var_jd_expcd_dn5, locals.var_jd_expcd_dn6, locals.var_jd_expcd_dn7, locals.var_jd_expcd_dn8, locals.var_jd_expcd_dn9, locals.var_jd_expcd_dn10, locals.var_jd_expcd_dn13,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign97280_e149741;
        locals.var_t1_dn0 = assign97280_e149741_d_n0;
        locals.var_t1_dn2 = assign97280_e149741_d_n2;
        locals.var_t1_dn4 = assign97280_e149741_d_n4;
        locals.var_t1_dn5 = assign97280_e149741_d_n5;
        locals.var_t1_dn6 = assign97280_e149741_d_n6;
        locals.var_t1_dn7 = assign97280_e149741_d_n7;
        locals.var_t1_dn8 = assign97280_e149741_d_n8;
        locals.var_t1_dn9 = assign97280_e149741_d_n9;
        locals.var_t1_dn10 = assign97280_e149741_d_n10;
        locals.var_t1_dn13 = assign97280_e149741_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign97290_e149752, assign97290_e149752_d_n0, assign97290_e149752_d_n2, assign97290_e149752_d_n4, assign97290_e149752_d_n5, assign97290_e149752_d_n6, assign97290_e149752_d_n7, assign97290_e149752_d_n8, assign97290_e149752_d_n9, assign97290_e149752_d_n10, assign97290_e149752_d_n13,) = {
    if ((locals.var_guard2254 != 0.0) && (locals.var_guard2255 == 0.0)) {
        let assign97290_e149748: f64 = (locals.var_isbd_btm * locals.var_jd_nvtm_invd);
        let assign97290_e149750: f64 = (assign97290_e149748 * locals.var_t1);
        (assign97290_e149750, ((((locals.var_isbd_btm_dn0 * locals.var_jd_nvtm_invd) + (locals.var_isbd_btm * locals.var_jd_nvtm_invd_dn0)) * locals.var_t1) + (assign97290_e149748 * locals.var_t1_dn0)), ((((locals.var_isbd_btm_dn2 * locals.var_jd_nvtm_invd) + (locals.var_isbd_btm * locals.var_jd_nvtm_invd_dn2)) * locals.var_t1) + (assign97290_e149748 * locals.var_t1_dn2)), ((((locals.var_isbd_btm_dn4 * locals.var_jd_nvtm_invd) + (locals.var_isbd_btm * locals.var_jd_nvtm_invd_dn4)) * locals.var_t1) + (assign97290_e149748 * locals.var_t1_dn4)), ((((locals.var_isbd_btm_dn5 * locals.var_jd_nvtm_invd) + (locals.var_isbd_btm * locals.var_jd_nvtm_invd_dn5)) * locals.var_t1) + (assign97290_e149748 * locals.var_t1_dn5)), ((((locals.var_isbd_btm_dn6 * locals.var_jd_nvtm_invd) + (locals.var_isbd_btm * locals.var_jd_nvtm_invd_dn6)) * locals.var_t1) + (assign97290_e149748 * locals.var_t1_dn6)), ((((locals.var_isbd_btm_dn7 * locals.var_jd_nvtm_invd) + (locals.var_isbd_btm * locals.var_jd_nvtm_invd_dn7)) * locals.var_t1) + (assign97290_e149748 * locals.var_t1_dn7)), ((((locals.var_isbd_btm_dn8 * locals.var_jd_nvtm_invd) + (locals.var_isbd_btm * locals.var_jd_nvtm_invd_dn8)) * locals.var_t1) + (assign97290_e149748 * locals.var_t1_dn8)), ((((locals.var_isbd_btm_dn9 * locals.var_jd_nvtm_invd) + (locals.var_isbd_btm * locals.var_jd_nvtm_invd_dn9)) * locals.var_t1) + (assign97290_e149748 * locals.var_t1_dn9)), ((((locals.var_isbd_btm_dn10 * locals.var_jd_nvtm_invd) + (locals.var_isbd_btm * locals.var_jd_nvtm_invd_dn10)) * locals.var_t1) + (assign97290_e149748 * locals.var_t1_dn10)), ((((locals.var_isbd_btm_dn13 * locals.var_jd_nvtm_invd) + (locals.var_isbd_btm * locals.var_jd_nvtm_invd_dn13)) * locals.var_t1) + (assign97290_e149748 * locals.var_t1_dn13)),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn13,)
    }
};
        locals.var_t4 = assign97290_e149752;
        locals.var_t4_dn0 = assign97290_e149752_d_n0;
        locals.var_t4_dn2 = assign97290_e149752_d_n2;
        locals.var_t4_dn4 = assign97290_e149752_d_n4;
        locals.var_t4_dn5 = assign97290_e149752_d_n5;
        locals.var_t4_dn6 = assign97290_e149752_d_n6;
        locals.var_t4_dn7 = assign97290_e149752_d_n7;
        locals.var_t4_dn8 = assign97290_e149752_d_n8;
        locals.var_t4_dn9 = assign97290_e149752_d_n9;
        locals.var_t4_dn10 = assign97290_e149752_d_n10;
        locals.var_t4_dn13 = assign97290_e149752_d_n13;
        locals.var_t4_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_365(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let assign97320_e149789: f64 = (p.p514 * locals.var_isbd2_btm);
        locals.var_t12 = assign97320_e149789;
        locals.var_t12_dn0 = (p.p514 * locals.var_isbd2_btm_dn0);
        locals.var_t12_dn2 = (p.p514 * locals.var_isbd2_btm_dn2);
        locals.var_t12_dn4 = (p.p514 * locals.var_isbd2_btm_dn4);
        locals.var_t12_dn5 = (p.p514 * locals.var_isbd2_btm_dn5);
        locals.var_t12_dn6 = (p.p514 * locals.var_isbd2_btm_dn6);
        locals.var_t12_dn7 = (p.p514 * locals.var_isbd2_btm_dn7);
        locals.var_t12_dn8 = (p.p514 * locals.var_isbd2_btm_dn8);
        locals.var_t12_dn9 = (p.p514 * locals.var_isbd2_btm_dn9);
        locals.var_t12_dn10 = (p.p514 * locals.var_isbd2_btm_dn10);
        locals.var_t12_dn13 = (p.p514 * locals.var_isbd2_btm_dn13);
        locals.var_t12_rv = 0.0;

        let assign97340_e149797: f64 = if locals.var_isbd_sws > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2257 = assign97340_e149797;
        locals.var_guard2257_rv = 0.0;

        let (assign97350_e149803, assign97350_e149803_d_n0, assign97350_e149803_d_n2, assign97350_e149803_d_n4, assign97350_e149803_d_n5, assign97350_e149803_d_n6, assign97350_e149803_d_n7, assign97350_e149803_d_n8, assign97350_e149803_d_n9, assign97350_e149803_d_n10, assign97350_e149803_d_n13,) = {
    if (locals.var_guard2257 != 0.0) {
        let assign97350_e149801: f64 = (locals.var_isbd2_sws * locals.var_t9);
        (assign97350_e149801, ((locals.var_isbd2_sws_dn0 * locals.var_t9) + (locals.var_isbd2_sws * locals.var_t9_dn0)), ((locals.var_isbd2_sws_dn2 * locals.var_t9) + (locals.var_isbd2_sws * locals.var_t9_dn2)), ((locals.var_isbd2_sws_dn4 * locals.var_t9) + (locals.var_isbd2_sws * locals.var_t9_dn4)), ((locals.var_isbd2_sws_dn5 * locals.var_t9) + (locals.var_isbd2_sws * locals.var_t9_dn5)), ((locals.var_isbd2_sws_dn6 * locals.var_t9) + (locals.var_isbd2_sws * locals.var_t9_dn6)), ((locals.var_isbd2_sws_dn7 * locals.var_t9) + (locals.var_isbd2_sws * locals.var_t9_dn7)), ((locals.var_isbd2_sws_dn8 * locals.var_t9) + (locals.var_isbd2_sws * locals.var_t9_dn8)), ((locals.var_isbd2_sws_dn9 * locals.var_t9) + (locals.var_isbd2_sws * locals.var_t9_dn9)), ((locals.var_isbd2_sws_dn10 * locals.var_t9) + (locals.var_isbd2_sws * locals.var_t9_dn10)), ((locals.var_isbd2_sws_dn13 * locals.var_t9) + (locals.var_isbd2_sws * locals.var_t9_dn13)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign97350_e149803;
        locals.var_t0_dn0 = assign97350_e149803_d_n0;
        locals.var_t0_dn2 = assign97350_e149803_d_n2;
        locals.var_t0_dn4 = assign97350_e149803_d_n4;
        locals.var_t0_dn5 = assign97350_e149803_d_n5;
        locals.var_t0_dn6 = assign97350_e149803_d_n6;
        locals.var_t0_dn7 = assign97350_e149803_d_n7;
        locals.var_t0_dn8 = assign97350_e149803_d_n8;
        locals.var_t0_dn9 = assign97350_e149803_d_n9;
        locals.var_t0_dn10 = assign97350_e149803_d_n10;
        locals.var_t0_dn13 = assign97350_e149803_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign97360_e149810, assign97360_e149810_d_n0, assign97360_e149810_d_n2, assign97360_e149810_d_n4, assign97360_e149810_d_n5, assign97360_e149810_d_n6, assign97360_e149810_d_n7, assign97360_e149810_d_n8, assign97360_e149810_d_n9, assign97360_e149810_d_n10, assign97360_e149810_d_n13,) = {
    if (locals.var_guard2257 != 0.0) {
        let assign97360_e149806: f64 = (-locals.var_vbd_jct);
        let assign97360_e149808: f64 = (assign97360_e149806 * locals.var_t10);
        (assign97360_e149808, (((-locals.var_vbd_jct_dn0) * locals.var_t10) + (assign97360_e149806 * locals.var_t10_dn0)), (assign97360_e149806 * locals.var_t10_dn2), (assign97360_e149806 * locals.var_t10_dn4), (assign97360_e149806 * locals.var_t10_dn5), (assign97360_e149806 * locals.var_t10_dn6), (assign97360_e149806 * locals.var_t10_dn7), (assign97360_e149806 * locals.var_t10_dn8), (((-locals.var_vbd_jct_dn9) * locals.var_t10) + (assign97360_e149806 * locals.var_t10_dn9)), (assign97360_e149806 * locals.var_t10_dn10), (assign97360_e149806 * locals.var_t10_dn13),)
    } else {
        (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn4, locals.var_tx_dn5, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn8, locals.var_tx_dn9, locals.var_tx_dn10, locals.var_tx_dn13,)
    }
};
        locals.var_tx = assign97360_e149810;
        locals.var_tx_dn0 = assign97360_e149810_d_n0;
        locals.var_tx_dn2 = assign97360_e149810_d_n2;
        locals.var_tx_dn4 = assign97360_e149810_d_n4;
        locals.var_tx_dn5 = assign97360_e149810_d_n5;
        locals.var_tx_dn6 = assign97360_e149810_d_n6;
        locals.var_tx_dn7 = assign97360_e149810_d_n7;
        locals.var_tx_dn8 = assign97360_e149810_d_n8;
        locals.var_tx_dn9 = assign97360_e149810_d_n9;
        locals.var_tx_dn10 = assign97360_e149810_d_n10;
        locals.var_tx_dn13 = assign97360_e149810_d_n13;
        locals.var_tx_rv = 0.0;

        let (assign97370_e149815, assign97370_e149815_d_n0, assign97370_e149815_d_n2, assign97370_e149815_d_n4, assign97370_e149815_d_n5, assign97370_e149815_d_n6, assign97370_e149815_d_n7, assign97370_e149815_d_n8, assign97370_e149815_d_n9, assign97370_e149815_d_n10, assign97370_e149815_d_n13,) = {
    if (locals.var_guard2257 != 0.0) {
        let assign97370_e149813: f64 = (locals.var_tx).exp();
        (assign97370_e149813, (assign97370_e149813 * locals.var_tx_dn0), (assign97370_e149813 * locals.var_tx_dn2), (assign97370_e149813 * locals.var_tx_dn4), (assign97370_e149813 * locals.var_tx_dn5), (assign97370_e149813 * locals.var_tx_dn6), (assign97370_e149813 * locals.var_tx_dn7), (assign97370_e149813 * locals.var_tx_dn8), (assign97370_e149813 * locals.var_tx_dn9), (assign97370_e149813 * locals.var_tx_dn10), (assign97370_e149813 * locals.var_tx_dn13),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign97370_e149815;
        locals.var_t2_dn0 = assign97370_e149815_d_n0;
        locals.var_t2_dn2 = assign97370_e149815_d_n2;
        locals.var_t2_dn4 = assign97370_e149815_d_n4;
        locals.var_t2_dn5 = assign97370_e149815_d_n5;
        locals.var_t2_dn6 = assign97370_e149815_d_n6;
        locals.var_t2_dn7 = assign97370_e149815_d_n7;
        locals.var_t2_dn8 = assign97370_e149815_d_n8;
        locals.var_t2_dn9 = assign97370_e149815_d_n9;
        locals.var_t2_dn10 = assign97370_e149815_d_n10;
        locals.var_t2_dn13 = assign97370_e149815_d_n13;
        locals.var_t2_rv = 0.0;

        let (assign97380_e149819, assign97380_e149819_d_n0, assign97380_e149819_d_n2, assign97380_e149819_d_n4, assign97380_e149819_d_n5, assign97380_e149819_d_n6, assign97380_e149819_d_n7, assign97380_e149819_d_n8, assign97380_e149819_d_n9, assign97380_e149819_d_n10, assign97380_e149819_d_n13,) = {
    if (locals.var_guard2257 != 0.0) {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn13,)
    }
};
        locals.var_t3 = assign97380_e149819;
        locals.var_t3_dn0 = assign97380_e149819_d_n0;
        locals.var_t3_dn2 = assign97380_e149819_d_n2;
        locals.var_t3_dn4 = assign97380_e149819_d_n4;
        locals.var_t3_dn5 = assign97380_e149819_d_n5;
        locals.var_t3_dn6 = assign97380_e149819_d_n6;
        locals.var_t3_dn7 = assign97380_e149819_d_n7;
        locals.var_t3_dn8 = assign97380_e149819_d_n8;
        locals.var_t3_dn9 = assign97380_e149819_d_n9;
        locals.var_t3_dn10 = assign97380_e149819_d_n10;
        locals.var_t3_dn13 = assign97380_e149819_d_n13;
        locals.var_t3_rv = 0.0;

        let assign97390_e149822: f64 = if locals.var_vbd_jct < locals.var_vbdt { 1.0 } else { 0.0 };
        locals.var_guard2258 = assign97390_e149822;
        locals.var_guard2258_rv = 0.0;

        let (assign97400_e149830, assign97400_e149830_d_n0, assign97400_e149830_d_n2, assign97400_e149830_d_n4, assign97400_e149830_d_n5, assign97400_e149830_d_n6, assign97400_e149830_d_n7, assign97400_e149830_d_n8, assign97400_e149830_d_n9, assign97400_e149830_d_n10, assign97400_e149830_d_n13,) = {
    if ((locals.var_guard2257 != 0.0) && (locals.var_guard2258 != 0.0)) {
        let assign97400_e149828: f64 = (locals.var_vbd_jct * locals.var_jd_nvtm_invd);
        (assign97400_e149828, ((locals.var_vbd_jct_dn0 * locals.var_jd_nvtm_invd) + (locals.var_vbd_jct * locals.var_jd_nvtm_invd_dn0)), (locals.var_vbd_jct * locals.var_jd_nvtm_invd_dn2), (locals.var_vbd_jct * locals.var_jd_nvtm_invd_dn4), (locals.var_vbd_jct * locals.var_jd_nvtm_invd_dn5), (locals.var_vbd_jct * locals.var_jd_nvtm_invd_dn6), (locals.var_vbd_jct * locals.var_jd_nvtm_invd_dn7), (locals.var_vbd_jct * locals.var_jd_nvtm_invd_dn8), ((locals.var_vbd_jct_dn9 * locals.var_jd_nvtm_invd) + (locals.var_vbd_jct * locals.var_jd_nvtm_invd_dn9)), (locals.var_vbd_jct * locals.var_jd_nvtm_invd_dn10), (locals.var_vbd_jct * locals.var_jd_nvtm_invd_dn13),)
    } else {
        (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn4, locals.var_tx_dn5, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn8, locals.var_tx_dn9, locals.var_tx_dn10, locals.var_tx_dn13,)
    }
};
        locals.var_tx = assign97400_e149830;
        locals.var_tx_dn0 = assign97400_e149830_d_n0;
        locals.var_tx_dn2 = assign97400_e149830_d_n2;
        locals.var_tx_dn4 = assign97400_e149830_d_n4;
        locals.var_tx_dn5 = assign97400_e149830_d_n5;
        locals.var_tx_dn6 = assign97400_e149830_d_n6;
        locals.var_tx_dn7 = assign97400_e149830_d_n7;
        locals.var_tx_dn8 = assign97400_e149830_d_n8;
        locals.var_tx_dn9 = assign97400_e149830_d_n9;
        locals.var_tx_dn10 = assign97400_e149830_d_n10;
        locals.var_tx_dn13 = assign97400_e149830_d_n13;
        locals.var_tx_rv = 0.0;

        let assign97410_e149833: f64 = (-3.0);
        let assign97410_e149835: f64 = (assign97410_e149833 * 34.0);
        let assign97410_e149836: f64 = if locals.var_tx < assign97410_e149835 { 1.0 } else { 0.0 };
        locals.var_guard2259 = assign97410_e149836;
        locals.var_guard2259_rv = 0.0;

        let (assign97420_e149844, assign97420_e149844_d_n0, assign97420_e149844_d_n2, assign97420_e149844_d_n4, assign97420_e149844_d_n5, assign97420_e149844_d_n6, assign97420_e149844_d_n7, assign97420_e149844_d_n8, assign97420_e149844_d_n9, assign97420_e149844_d_n10, assign97420_e149844_d_n13,) = {
    if (((locals.var_guard2257 != 0.0) && (locals.var_guard2258 != 0.0)) && (locals.var_guard2259 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign97420_e149844;
        locals.var_t1_dn0 = assign97420_e149844_d_n0;
        locals.var_t1_dn2 = assign97420_e149844_d_n2;
        locals.var_t1_dn4 = assign97420_e149844_d_n4;
        locals.var_t1_dn5 = assign97420_e149844_d_n5;
        locals.var_t1_dn6 = assign97420_e149844_d_n6;
        locals.var_t1_dn7 = assign97420_e149844_d_n7;
        locals.var_t1_dn8 = assign97420_e149844_d_n8;
        locals.var_t1_dn9 = assign97420_e149844_d_n9;
        locals.var_t1_dn10 = assign97420_e149844_d_n10;
        locals.var_t1_dn13 = assign97420_e149844_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign97430_e149854, assign97430_e149854_d_n0, assign97430_e149854_d_n2, assign97430_e149854_d_n4, assign97430_e149854_d_n5, assign97430_e149854_d_n6, assign97430_e149854_d_n7, assign97430_e149854_d_n8, assign97430_e149854_d_n9, assign97430_e149854_d_n10, assign97430_e149854_d_n13,) = {
    if (((locals.var_guard2257 != 0.0) && (locals.var_guard2258 != 0.0)) && (locals.var_guard2259 == 0.0)) {
        let assign97430_e149852: f64 = (locals.var_tx).exp();
        (assign97430_e149852, (assign97430_e149852 * locals.var_tx_dn0), (assign97430_e149852 * locals.var_tx_dn2), (assign97430_e149852 * locals.var_tx_dn4), (assign97430_e149852 * locals.var_tx_dn5), (assign97430_e149852 * locals.var_tx_dn6), (assign97430_e149852 * locals.var_tx_dn7), (assign97430_e149852 * locals.var_tx_dn8), (assign97430_e149852 * locals.var_tx_dn9), (assign97430_e149852 * locals.var_tx_dn10), (assign97430_e149852 * locals.var_tx_dn13),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign97430_e149854;
        locals.var_t1_dn0 = assign97430_e149854_d_n0;
        locals.var_t1_dn2 = assign97430_e149854_d_n2;
        locals.var_t1_dn4 = assign97430_e149854_d_n4;
        locals.var_t1_dn5 = assign97430_e149854_d_n5;
        locals.var_t1_dn6 = assign97430_e149854_d_n6;
        locals.var_t1_dn7 = assign97430_e149854_d_n7;
        locals.var_t1_dn8 = assign97430_e149854_d_n8;
        locals.var_t1_dn9 = assign97430_e149854_d_n9;
        locals.var_t1_dn10 = assign97430_e149854_d_n10;
        locals.var_t1_dn13 = assign97430_e149854_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign97450_e149883, assign97450_e149883_d_n0, assign97450_e149883_d_n2, assign97450_e149883_d_n4, assign97450_e149883_d_n5, assign97450_e149883_d_n6, assign97450_e149883_d_n7, assign97450_e149883_d_n8, assign97450_e149883_d_n9, assign97450_e149883_d_n10, assign97450_e149883_d_n13,) = {
    if ((locals.var_guard2257 != 0.0) && (locals.var_guard2258 == 0.0)) {
        (locals.var_jd_expcd, locals.var_jd_expcd_dn0, locals.var_jd_expcd_dn2, locals.var_jd_expcd_dn4, locals.var_jd_expcd_dn5, locals.var_jd_expcd_dn6, locals.var_jd_expcd_dn7, locals.var_jd_expcd_dn8, locals.var_jd_expcd_dn9, locals.var_jd_expcd_dn10, locals.var_jd_expcd_dn13,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign97450_e149883;
        locals.var_t1_dn0 = assign97450_e149883_d_n0;
        locals.var_t1_dn2 = assign97450_e149883_d_n2;
        locals.var_t1_dn4 = assign97450_e149883_d_n4;
        locals.var_t1_dn5 = assign97450_e149883_d_n5;
        locals.var_t1_dn6 = assign97450_e149883_d_n6;
        locals.var_t1_dn7 = assign97450_e149883_d_n7;
        locals.var_t1_dn8 = assign97450_e149883_d_n8;
        locals.var_t1_dn9 = assign97450_e149883_d_n9;
        locals.var_t1_dn10 = assign97450_e149883_d_n10;
        locals.var_t1_dn13 = assign97450_e149883_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign97460_e149894, assign97460_e149894_d_n0, assign97460_e149894_d_n2, assign97460_e149894_d_n4, assign97460_e149894_d_n5, assign97460_e149894_d_n6, assign97460_e149894_d_n7, assign97460_e149894_d_n8, assign97460_e149894_d_n9, assign97460_e149894_d_n10, assign97460_e149894_d_n13,) = {
    if ((locals.var_guard2257 != 0.0) && (locals.var_guard2258 == 0.0)) {
        let assign97460_e149890: f64 = (locals.var_isbd_sws * locals.var_jd_nvtm_invd);
        let assign97460_e149892: f64 = (assign97460_e149890 * locals.var_t1);
        (assign97460_e149892, ((((locals.var_isbd_sws_dn0 * locals.var_jd_nvtm_invd) + (locals.var_isbd_sws * locals.var_jd_nvtm_invd_dn0)) * locals.var_t1) + (assign97460_e149890 * locals.var_t1_dn0)), ((((locals.var_isbd_sws_dn2 * locals.var_jd_nvtm_invd) + (locals.var_isbd_sws * locals.var_jd_nvtm_invd_dn2)) * locals.var_t1) + (assign97460_e149890 * locals.var_t1_dn2)), ((((locals.var_isbd_sws_dn4 * locals.var_jd_nvtm_invd) + (locals.var_isbd_sws * locals.var_jd_nvtm_invd_dn4)) * locals.var_t1) + (assign97460_e149890 * locals.var_t1_dn4)), ((((locals.var_isbd_sws_dn5 * locals.var_jd_nvtm_invd) + (locals.var_isbd_sws * locals.var_jd_nvtm_invd_dn5)) * locals.var_t1) + (assign97460_e149890 * locals.var_t1_dn5)), ((((locals.var_isbd_sws_dn6 * locals.var_jd_nvtm_invd) + (locals.var_isbd_sws * locals.var_jd_nvtm_invd_dn6)) * locals.var_t1) + (assign97460_e149890 * locals.var_t1_dn6)), ((((locals.var_isbd_sws_dn7 * locals.var_jd_nvtm_invd) + (locals.var_isbd_sws * locals.var_jd_nvtm_invd_dn7)) * locals.var_t1) + (assign97460_e149890 * locals.var_t1_dn7)), ((((locals.var_isbd_sws_dn8 * locals.var_jd_nvtm_invd) + (locals.var_isbd_sws * locals.var_jd_nvtm_invd_dn8)) * locals.var_t1) + (assign97460_e149890 * locals.var_t1_dn8)), ((((locals.var_isbd_sws_dn9 * locals.var_jd_nvtm_invd) + (locals.var_isbd_sws * locals.var_jd_nvtm_invd_dn9)) * locals.var_t1) + (assign97460_e149890 * locals.var_t1_dn9)), ((((locals.var_isbd_sws_dn10 * locals.var_jd_nvtm_invd) + (locals.var_isbd_sws * locals.var_jd_nvtm_invd_dn10)) * locals.var_t1) + (assign97460_e149890 * locals.var_t1_dn10)), ((((locals.var_isbd_sws_dn13 * locals.var_jd_nvtm_invd) + (locals.var_isbd_sws * locals.var_jd_nvtm_invd_dn13)) * locals.var_t1) + (assign97460_e149890 * locals.var_t1_dn13)),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn13,)
    }
};
        locals.var_t4 = assign97460_e149894;
        locals.var_t4_dn0 = assign97460_e149894_d_n0;
        locals.var_t4_dn2 = assign97460_e149894_d_n2;
        locals.var_t4_dn4 = assign97460_e149894_d_n4;
        locals.var_t4_dn5 = assign97460_e149894_d_n5;
        locals.var_t4_dn6 = assign97460_e149894_d_n6;
        locals.var_t4_dn7 = assign97460_e149894_d_n7;
        locals.var_t4_dn8 = assign97460_e149894_d_n8;
        locals.var_t4_dn9 = assign97460_e149894_d_n9;
        locals.var_t4_dn10 = assign97460_e149894_d_n10;
        locals.var_t4_dn13 = assign97460_e149894_d_n13;
        locals.var_t4_rv = 0.0;

        let assign97490_e149931: f64 = (p.p514 * locals.var_isbd2_sws);
        locals.var_t12 = assign97490_e149931;
        locals.var_t12_dn0 = (p.p514 * locals.var_isbd2_sws_dn0);
        locals.var_t12_dn2 = (p.p514 * locals.var_isbd2_sws_dn2);
        locals.var_t12_dn4 = (p.p514 * locals.var_isbd2_sws_dn4);
        locals.var_t12_dn5 = (p.p514 * locals.var_isbd2_sws_dn5);
        locals.var_t12_dn6 = (p.p514 * locals.var_isbd2_sws_dn6);
        locals.var_t12_dn7 = (p.p514 * locals.var_isbd2_sws_dn7);
        locals.var_t12_dn8 = (p.p514 * locals.var_isbd2_sws_dn8);
        locals.var_t12_dn9 = (p.p514 * locals.var_isbd2_sws_dn9);
        locals.var_t12_dn10 = (p.p514 * locals.var_isbd2_sws_dn10);
        locals.var_t12_dn13 = (p.p514 * locals.var_isbd2_sws_dn13);
        locals.var_t12_rv = 0.0;

        let assign97510_e149939: f64 = if p.p48 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2260 = assign97510_e149939;
        locals.var_guard2260_rv = 0.0;

        let assign97520_e149942: f64 = if locals.var_isbd_swg > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2261 = assign97520_e149942;
        locals.var_guard2261_rv = 0.0;

        let (assign97530_e149950, assign97530_e149950_d_n0, assign97530_e149950_d_n2, assign97530_e149950_d_n4, assign97530_e149950_d_n5, assign97530_e149950_d_n6, assign97530_e149950_d_n7, assign97530_e149950_d_n8, assign97530_e149950_d_n9, assign97530_e149950_d_n10, assign97530_e149950_d_n13,) = {
    if ((locals.var_guard2260 != 0.0) && (locals.var_guard2261 != 0.0)) {
        let assign97530_e149948: f64 = (locals.var_isbd2_swg * locals.var_t9);
        (assign97530_e149948, ((locals.var_isbd2_swg_dn0 * locals.var_t9) + (locals.var_isbd2_swg * locals.var_t9_dn0)), ((locals.var_isbd2_swg_dn2 * locals.var_t9) + (locals.var_isbd2_swg * locals.var_t9_dn2)), ((locals.var_isbd2_swg_dn4 * locals.var_t9) + (locals.var_isbd2_swg * locals.var_t9_dn4)), ((locals.var_isbd2_swg_dn5 * locals.var_t9) + (locals.var_isbd2_swg * locals.var_t9_dn5)), ((locals.var_isbd2_swg_dn6 * locals.var_t9) + (locals.var_isbd2_swg * locals.var_t9_dn6)), ((locals.var_isbd2_swg_dn7 * locals.var_t9) + (locals.var_isbd2_swg * locals.var_t9_dn7)), ((locals.var_isbd2_swg_dn8 * locals.var_t9) + (locals.var_isbd2_swg * locals.var_t9_dn8)), ((locals.var_isbd2_swg_dn9 * locals.var_t9) + (locals.var_isbd2_swg * locals.var_t9_dn9)), ((locals.var_isbd2_swg_dn10 * locals.var_t9) + (locals.var_isbd2_swg * locals.var_t9_dn10)), ((locals.var_isbd2_swg_dn13 * locals.var_t9) + (locals.var_isbd2_swg * locals.var_t9_dn13)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign97530_e149950;
        locals.var_t0_dn0 = assign97530_e149950_d_n0;
        locals.var_t0_dn2 = assign97530_e149950_d_n2;
        locals.var_t0_dn4 = assign97530_e149950_d_n4;
        locals.var_t0_dn5 = assign97530_e149950_d_n5;
        locals.var_t0_dn6 = assign97530_e149950_d_n6;
        locals.var_t0_dn7 = assign97530_e149950_d_n7;
        locals.var_t0_dn8 = assign97530_e149950_d_n8;
        locals.var_t0_dn9 = assign97530_e149950_d_n9;
        locals.var_t0_dn10 = assign97530_e149950_d_n10;
        locals.var_t0_dn13 = assign97530_e149950_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign97540_e149959, assign97540_e149959_d_n0, assign97540_e149959_d_n2, assign97540_e149959_d_n4, assign97540_e149959_d_n5, assign97540_e149959_d_n6, assign97540_e149959_d_n7, assign97540_e149959_d_n8, assign97540_e149959_d_n9, assign97540_e149959_d_n10, assign97540_e149959_d_n13,) = {
    if ((locals.var_guard2260 != 0.0) && (locals.var_guard2261 != 0.0)) {
        let assign97540_e149955: f64 = (-locals.var_vbdi_jct);
        let assign97540_e149957: f64 = (assign97540_e149955 * locals.var_t10);
        (assign97540_e149957, (assign97540_e149955 * locals.var_t10_dn0), (assign97540_e149955 * locals.var_t10_dn2), (assign97540_e149955 * locals.var_t10_dn4), (((-locals.var_vbdi_jct_dn5) * locals.var_t10) + (assign97540_e149955 * locals.var_t10_dn5)), (assign97540_e149955 * locals.var_t10_dn6), (assign97540_e149955 * locals.var_t10_dn7), (((-locals.var_vbdi_jct_dn8) * locals.var_t10) + (assign97540_e149955 * locals.var_t10_dn8)), (assign97540_e149955 * locals.var_t10_dn9), (assign97540_e149955 * locals.var_t10_dn10), (assign97540_e149955 * locals.var_t10_dn13),)
    } else {
        (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn4, locals.var_tx_dn5, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn8, locals.var_tx_dn9, locals.var_tx_dn10, locals.var_tx_dn13,)
    }
};
        locals.var_tx = assign97540_e149959;
        locals.var_tx_dn0 = assign97540_e149959_d_n0;
        locals.var_tx_dn2 = assign97540_e149959_d_n2;
        locals.var_tx_dn4 = assign97540_e149959_d_n4;
        locals.var_tx_dn5 = assign97540_e149959_d_n5;
        locals.var_tx_dn6 = assign97540_e149959_d_n6;
        locals.var_tx_dn7 = assign97540_e149959_d_n7;
        locals.var_tx_dn8 = assign97540_e149959_d_n8;
        locals.var_tx_dn9 = assign97540_e149959_d_n9;
        locals.var_tx_dn10 = assign97540_e149959_d_n10;
        locals.var_tx_dn13 = assign97540_e149959_d_n13;
        locals.var_tx_rv = 0.0;

        let (assign97550_e149966, assign97550_e149966_d_n0, assign97550_e149966_d_n2, assign97550_e149966_d_n4, assign97550_e149966_d_n5, assign97550_e149966_d_n6, assign97550_e149966_d_n7, assign97550_e149966_d_n8, assign97550_e149966_d_n9, assign97550_e149966_d_n10, assign97550_e149966_d_n13,) = {
    if ((locals.var_guard2260 != 0.0) && (locals.var_guard2261 != 0.0)) {
        let assign97550_e149964: f64 = (locals.var_tx).exp();
        (assign97550_e149964, (assign97550_e149964 * locals.var_tx_dn0), (assign97550_e149964 * locals.var_tx_dn2), (assign97550_e149964 * locals.var_tx_dn4), (assign97550_e149964 * locals.var_tx_dn5), (assign97550_e149964 * locals.var_tx_dn6), (assign97550_e149964 * locals.var_tx_dn7), (assign97550_e149964 * locals.var_tx_dn8), (assign97550_e149964 * locals.var_tx_dn9), (assign97550_e149964 * locals.var_tx_dn10), (assign97550_e149964 * locals.var_tx_dn13),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign97550_e149966;
        locals.var_t2_dn0 = assign97550_e149966_d_n0;
        locals.var_t2_dn2 = assign97550_e149966_d_n2;
        locals.var_t2_dn4 = assign97550_e149966_d_n4;
        locals.var_t2_dn5 = assign97550_e149966_d_n5;
        locals.var_t2_dn6 = assign97550_e149966_d_n6;
        locals.var_t2_dn7 = assign97550_e149966_d_n7;
        locals.var_t2_dn8 = assign97550_e149966_d_n8;
        locals.var_t2_dn9 = assign97550_e149966_d_n9;
        locals.var_t2_dn10 = assign97550_e149966_d_n10;
        locals.var_t2_dn13 = assign97550_e149966_d_n13;
        locals.var_t2_rv = 0.0;

        let (assign97560_e149972, assign97560_e149972_d_n0, assign97560_e149972_d_n2, assign97560_e149972_d_n4, assign97560_e149972_d_n5, assign97560_e149972_d_n6, assign97560_e149972_d_n7, assign97560_e149972_d_n8, assign97560_e149972_d_n9, assign97560_e149972_d_n10, assign97560_e149972_d_n13,) = {
    if ((locals.var_guard2260 != 0.0) && (locals.var_guard2261 != 0.0)) {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn13,)
    }
};
        locals.var_t3 = assign97560_e149972;
        locals.var_t3_dn0 = assign97560_e149972_d_n0;
        locals.var_t3_dn2 = assign97560_e149972_d_n2;
        locals.var_t3_dn4 = assign97560_e149972_d_n4;
        locals.var_t3_dn5 = assign97560_e149972_d_n5;
        locals.var_t3_dn6 = assign97560_e149972_d_n6;
        locals.var_t3_dn7 = assign97560_e149972_d_n7;
        locals.var_t3_dn8 = assign97560_e149972_d_n8;
        locals.var_t3_dn9 = assign97560_e149972_d_n9;
        locals.var_t3_dn10 = assign97560_e149972_d_n10;
        locals.var_t3_dn13 = assign97560_e149972_d_n13;
        locals.var_t3_rv = 0.0;

        let assign97570_e149975: f64 = if locals.var_vbdi_jct < locals.var_vbdt { 1.0 } else { 0.0 };
        locals.var_guard2262 = assign97570_e149975;
        locals.var_guard2262_rv = 0.0;

        let (assign97580_e149985, assign97580_e149985_d_n0, assign97580_e149985_d_n2, assign97580_e149985_d_n4, assign97580_e149985_d_n5, assign97580_e149985_d_n6, assign97580_e149985_d_n7, assign97580_e149985_d_n8, assign97580_e149985_d_n9, assign97580_e149985_d_n10, assign97580_e149985_d_n13,) = {
    if (((locals.var_guard2260 != 0.0) && (locals.var_guard2261 != 0.0)) && (locals.var_guard2262 != 0.0)) {
        let assign97580_e149983: f64 = (locals.var_vbdi_jct * locals.var_jd_nvtm_invd);
        (assign97580_e149983, (locals.var_vbdi_jct * locals.var_jd_nvtm_invd_dn0), (locals.var_vbdi_jct * locals.var_jd_nvtm_invd_dn2), (locals.var_vbdi_jct * locals.var_jd_nvtm_invd_dn4), ((locals.var_vbdi_jct_dn5 * locals.var_jd_nvtm_invd) + (locals.var_vbdi_jct * locals.var_jd_nvtm_invd_dn5)), (locals.var_vbdi_jct * locals.var_jd_nvtm_invd_dn6), (locals.var_vbdi_jct * locals.var_jd_nvtm_invd_dn7), ((locals.var_vbdi_jct_dn8 * locals.var_jd_nvtm_invd) + (locals.var_vbdi_jct * locals.var_jd_nvtm_invd_dn8)), (locals.var_vbdi_jct * locals.var_jd_nvtm_invd_dn9), (locals.var_vbdi_jct * locals.var_jd_nvtm_invd_dn10), (locals.var_vbdi_jct * locals.var_jd_nvtm_invd_dn13),)
    } else {
        (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn4, locals.var_tx_dn5, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn8, locals.var_tx_dn9, locals.var_tx_dn10, locals.var_tx_dn13,)
    }
};
        locals.var_tx = assign97580_e149985;
        locals.var_tx_dn0 = assign97580_e149985_d_n0;
        locals.var_tx_dn2 = assign97580_e149985_d_n2;
        locals.var_tx_dn4 = assign97580_e149985_d_n4;
        locals.var_tx_dn5 = assign97580_e149985_d_n5;
        locals.var_tx_dn6 = assign97580_e149985_d_n6;
        locals.var_tx_dn7 = assign97580_e149985_d_n7;
        locals.var_tx_dn8 = assign97580_e149985_d_n8;
        locals.var_tx_dn9 = assign97580_e149985_d_n9;
        locals.var_tx_dn10 = assign97580_e149985_d_n10;
        locals.var_tx_dn13 = assign97580_e149985_d_n13;
        locals.var_tx_rv = 0.0;

        let assign97590_e149988: f64 = (-3.0);
        let assign97590_e149990: f64 = (assign97590_e149988 * 34.0);
        let assign97590_e149991: f64 = if locals.var_tx < assign97590_e149990 { 1.0 } else { 0.0 };
        locals.var_guard2263 = assign97590_e149991;
        locals.var_guard2263_rv = 0.0;

        let (assign97600_e150001, assign97600_e150001_d_n0, assign97600_e150001_d_n2, assign97600_e150001_d_n4, assign97600_e150001_d_n5, assign97600_e150001_d_n6, assign97600_e150001_d_n7, assign97600_e150001_d_n8, assign97600_e150001_d_n9, assign97600_e150001_d_n10, assign97600_e150001_d_n13,) = {
    if ((((locals.var_guard2260 != 0.0) && (locals.var_guard2261 != 0.0)) && (locals.var_guard2262 != 0.0)) && (locals.var_guard2263 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign97600_e150001;
        locals.var_t1_dn0 = assign97600_e150001_d_n0;
        locals.var_t1_dn2 = assign97600_e150001_d_n2;
        locals.var_t1_dn4 = assign97600_e150001_d_n4;
        locals.var_t1_dn5 = assign97600_e150001_d_n5;
        locals.var_t1_dn6 = assign97600_e150001_d_n6;
        locals.var_t1_dn7 = assign97600_e150001_d_n7;
        locals.var_t1_dn8 = assign97600_e150001_d_n8;
        locals.var_t1_dn9 = assign97600_e150001_d_n9;
        locals.var_t1_dn10 = assign97600_e150001_d_n10;
        locals.var_t1_dn13 = assign97600_e150001_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign97610_e150013, assign97610_e150013_d_n0, assign97610_e150013_d_n2, assign97610_e150013_d_n4, assign97610_e150013_d_n5, assign97610_e150013_d_n6, assign97610_e150013_d_n7, assign97610_e150013_d_n8, assign97610_e150013_d_n9, assign97610_e150013_d_n10, assign97610_e150013_d_n13,) = {
    if ((((locals.var_guard2260 != 0.0) && (locals.var_guard2261 != 0.0)) && (locals.var_guard2262 != 0.0)) && (locals.var_guard2263 == 0.0)) {
        let assign97610_e150011: f64 = (locals.var_tx).exp();
        (assign97610_e150011, (assign97610_e150011 * locals.var_tx_dn0), (assign97610_e150011 * locals.var_tx_dn2), (assign97610_e150011 * locals.var_tx_dn4), (assign97610_e150011 * locals.var_tx_dn5), (assign97610_e150011 * locals.var_tx_dn6), (assign97610_e150011 * locals.var_tx_dn7), (assign97610_e150011 * locals.var_tx_dn8), (assign97610_e150011 * locals.var_tx_dn9), (assign97610_e150011 * locals.var_tx_dn10), (assign97610_e150011 * locals.var_tx_dn13),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign97610_e150013;
        locals.var_t1_dn0 = assign97610_e150013_d_n0;
        locals.var_t1_dn2 = assign97610_e150013_d_n2;
        locals.var_t1_dn4 = assign97610_e150013_d_n4;
        locals.var_t1_dn5 = assign97610_e150013_d_n5;
        locals.var_t1_dn6 = assign97610_e150013_d_n6;
        locals.var_t1_dn7 = assign97610_e150013_d_n7;
        locals.var_t1_dn8 = assign97610_e150013_d_n8;
        locals.var_t1_dn9 = assign97610_e150013_d_n9;
        locals.var_t1_dn10 = assign97610_e150013_d_n10;
        locals.var_t1_dn13 = assign97610_e150013_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign97630_e150046, assign97630_e150046_d_n0, assign97630_e150046_d_n2, assign97630_e150046_d_n4, assign97630_e150046_d_n5, assign97630_e150046_d_n6, assign97630_e150046_d_n7, assign97630_e150046_d_n8, assign97630_e150046_d_n9, assign97630_e150046_d_n10, assign97630_e150046_d_n13,) = {
    if (((locals.var_guard2260 != 0.0) && (locals.var_guard2261 != 0.0)) && (locals.var_guard2262 == 0.0)) {
        (locals.var_jd_expcd, locals.var_jd_expcd_dn0, locals.var_jd_expcd_dn2, locals.var_jd_expcd_dn4, locals.var_jd_expcd_dn5, locals.var_jd_expcd_dn6, locals.var_jd_expcd_dn7, locals.var_jd_expcd_dn8, locals.var_jd_expcd_dn9, locals.var_jd_expcd_dn10, locals.var_jd_expcd_dn13,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign97630_e150046;
        locals.var_t1_dn0 = assign97630_e150046_d_n0;
        locals.var_t1_dn2 = assign97630_e150046_d_n2;
        locals.var_t1_dn4 = assign97630_e150046_d_n4;
        locals.var_t1_dn5 = assign97630_e150046_d_n5;
        locals.var_t1_dn6 = assign97630_e150046_d_n6;
        locals.var_t1_dn7 = assign97630_e150046_d_n7;
        locals.var_t1_dn8 = assign97630_e150046_d_n8;
        locals.var_t1_dn9 = assign97630_e150046_d_n9;
        locals.var_t1_dn10 = assign97630_e150046_d_n10;
        locals.var_t1_dn13 = assign97630_e150046_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign97640_e150059, assign97640_e150059_d_n0, assign97640_e150059_d_n2, assign97640_e150059_d_n4, assign97640_e150059_d_n5, assign97640_e150059_d_n6, assign97640_e150059_d_n7, assign97640_e150059_d_n8, assign97640_e150059_d_n9, assign97640_e150059_d_n10, assign97640_e150059_d_n13,) = {
    if (((locals.var_guard2260 != 0.0) && (locals.var_guard2261 != 0.0)) && (locals.var_guard2262 == 0.0)) {
        let assign97640_e150055: f64 = (locals.var_isbd_swg * locals.var_jd_nvtm_invd);
        let assign97640_e150057: f64 = (assign97640_e150055 * locals.var_t1);
        (assign97640_e150057, ((((locals.var_isbd_swg_dn0 * locals.var_jd_nvtm_invd) + (locals.var_isbd_swg * locals.var_jd_nvtm_invd_dn0)) * locals.var_t1) + (assign97640_e150055 * locals.var_t1_dn0)), ((((locals.var_isbd_swg_dn2 * locals.var_jd_nvtm_invd) + (locals.var_isbd_swg * locals.var_jd_nvtm_invd_dn2)) * locals.var_t1) + (assign97640_e150055 * locals.var_t1_dn2)), ((((locals.var_isbd_swg_dn4 * locals.var_jd_nvtm_invd) + (locals.var_isbd_swg * locals.var_jd_nvtm_invd_dn4)) * locals.var_t1) + (assign97640_e150055 * locals.var_t1_dn4)), ((((locals.var_isbd_swg_dn5 * locals.var_jd_nvtm_invd) + (locals.var_isbd_swg * locals.var_jd_nvtm_invd_dn5)) * locals.var_t1) + (assign97640_e150055 * locals.var_t1_dn5)), ((((locals.var_isbd_swg_dn6 * locals.var_jd_nvtm_invd) + (locals.var_isbd_swg * locals.var_jd_nvtm_invd_dn6)) * locals.var_t1) + (assign97640_e150055 * locals.var_t1_dn6)), ((((locals.var_isbd_swg_dn7 * locals.var_jd_nvtm_invd) + (locals.var_isbd_swg * locals.var_jd_nvtm_invd_dn7)) * locals.var_t1) + (assign97640_e150055 * locals.var_t1_dn7)), ((((locals.var_isbd_swg_dn8 * locals.var_jd_nvtm_invd) + (locals.var_isbd_swg * locals.var_jd_nvtm_invd_dn8)) * locals.var_t1) + (assign97640_e150055 * locals.var_t1_dn8)), ((((locals.var_isbd_swg_dn9 * locals.var_jd_nvtm_invd) + (locals.var_isbd_swg * locals.var_jd_nvtm_invd_dn9)) * locals.var_t1) + (assign97640_e150055 * locals.var_t1_dn9)), ((((locals.var_isbd_swg_dn10 * locals.var_jd_nvtm_invd) + (locals.var_isbd_swg * locals.var_jd_nvtm_invd_dn10)) * locals.var_t1) + (assign97640_e150055 * locals.var_t1_dn10)), ((((locals.var_isbd_swg_dn13 * locals.var_jd_nvtm_invd) + (locals.var_isbd_swg * locals.var_jd_nvtm_invd_dn13)) * locals.var_t1) + (assign97640_e150055 * locals.var_t1_dn13)),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn13,)
    }
};
        locals.var_t4 = assign97640_e150059;
        locals.var_t4_dn0 = assign97640_e150059_d_n0;
        locals.var_t4_dn2 = assign97640_e150059_d_n2;
        locals.var_t4_dn4 = assign97640_e150059_d_n4;
        locals.var_t4_dn5 = assign97640_e150059_d_n5;
        locals.var_t4_dn6 = assign97640_e150059_d_n6;
        locals.var_t4_dn7 = assign97640_e150059_d_n7;
        locals.var_t4_dn8 = assign97640_e150059_d_n8;
        locals.var_t4_dn9 = assign97640_e150059_d_n9;
        locals.var_t4_dn10 = assign97640_e150059_d_n10;
        locals.var_t4_dn13 = assign97640_e150059_d_n13;
        locals.var_t4_rv = 0.0;

        let (assign97670_e150103, assign97670_e150103_d_n0, assign97670_e150103_d_n2, assign97670_e150103_d_n4, assign97670_e150103_d_n5, assign97670_e150103_d_n6, assign97670_e150103_d_n7, assign97670_e150103_d_n8, assign97670_e150103_d_n9, assign97670_e150103_d_n10, assign97670_e150103_d_n13,) = {
    if (locals.var_guard2260 != 0.0) {
        let assign97670_e150101: f64 = (p.p514 * locals.var_isbd2_swg);
        (assign97670_e150101, (p.p514 * locals.var_isbd2_swg_dn0), (p.p514 * locals.var_isbd2_swg_dn2), (p.p514 * locals.var_isbd2_swg_dn4), (p.p514 * locals.var_isbd2_swg_dn5), (p.p514 * locals.var_isbd2_swg_dn6), (p.p514 * locals.var_isbd2_swg_dn7), (p.p514 * locals.var_isbd2_swg_dn8), (p.p514 * locals.var_isbd2_swg_dn9), (p.p514 * locals.var_isbd2_swg_dn10), (p.p514 * locals.var_isbd2_swg_dn13),)
    } else {
        (locals.var_t12, locals.var_t12_dn0, locals.var_t12_dn2, locals.var_t12_dn4, locals.var_t12_dn5, locals.var_t12_dn6, locals.var_t12_dn7, locals.var_t12_dn8, locals.var_t12_dn9, locals.var_t12_dn10, locals.var_t12_dn13,)
    }
};
        locals.var_t12 = assign97670_e150103;
        locals.var_t12_dn0 = assign97670_e150103_d_n0;
        locals.var_t12_dn2 = assign97670_e150103_d_n2;
        locals.var_t12_dn4 = assign97670_e150103_d_n4;
        locals.var_t12_dn5 = assign97670_e150103_d_n5;
        locals.var_t12_dn6 = assign97670_e150103_d_n6;
        locals.var_t12_dn7 = assign97670_e150103_d_n7;
        locals.var_t12_dn8 = assign97670_e150103_d_n8;
        locals.var_t12_dn9 = assign97670_e150103_d_n9;
        locals.var_t12_dn10 = assign97670_e150103_d_n10;
        locals.var_t12_dn13 = assign97670_e150103_d_n13;
        locals.var_t12_rv = 0.0;

        let assign97700_e150119: f64 = (p.p534 * locals.var_jd_nvtm_invs);
        locals.var_t10 = assign97700_e150119;
        locals.var_t10_dn0 = (p.p534 * locals.var_jd_nvtm_invs_dn0);
        locals.var_t10_dn2 = (p.p534 * locals.var_jd_nvtm_invs_dn2);
        locals.var_t10_dn4 = (p.p534 * locals.var_jd_nvtm_invs_dn4);
        locals.var_t10_dn5 = (p.p534 * locals.var_jd_nvtm_invs_dn5);
        locals.var_t10_dn6 = (p.p534 * locals.var_jd_nvtm_invs_dn6);
        locals.var_t10_dn7 = (p.p534 * locals.var_jd_nvtm_invs_dn7);
        locals.var_t10_dn8 = (p.p534 * locals.var_jd_nvtm_invs_dn8);
        locals.var_t10_dn9 = (p.p534 * locals.var_jd_nvtm_invs_dn9);
        locals.var_t10_dn10 = (p.p534 * locals.var_jd_nvtm_invs_dn10);
        locals.var_t10_dn13 = (p.p534 * locals.var_jd_nvtm_invs_dn13);
        locals.var_t10_rv = 0.0;

        let assign97710_e150122: f64 = (p.p533 * locals.var_exptemps);
        locals.var_t9 = assign97710_e150122;
        locals.var_t9_dn0 = (p.p533 * locals.var_exptemps_dn0);
        locals.var_t9_dn2 = (p.p533 * locals.var_exptemps_dn2);
        locals.var_t9_dn4 = (p.p533 * locals.var_exptemps_dn4);
        locals.var_t9_dn5 = (p.p533 * locals.var_exptemps_dn5);
        locals.var_t9_dn6 = (p.p533 * locals.var_exptemps_dn6);
        locals.var_t9_dn7 = (p.p533 * locals.var_exptemps_dn7);
        locals.var_t9_dn8 = (p.p533 * locals.var_exptemps_dn8);
        locals.var_t9_dn9 = (p.p533 * locals.var_exptemps_dn9);
        locals.var_t9_dn10 = (p.p533 * locals.var_exptemps_dn10);
        locals.var_t9_dn13 = (p.p533 * locals.var_exptemps_dn13);
        locals.var_t9_rv = 0.0;

        let assign97720_e150125: f64 = if locals.var_isbs_btm > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2264 = assign97720_e150125;
        locals.var_guard2264_rv = 0.0;

        let (assign97730_e150131, assign97730_e150131_d_n0, assign97730_e150131_d_n2, assign97730_e150131_d_n4, assign97730_e150131_d_n5, assign97730_e150131_d_n6, assign97730_e150131_d_n7, assign97730_e150131_d_n8, assign97730_e150131_d_n9, assign97730_e150131_d_n10, assign97730_e150131_d_n13,) = {
    if (locals.var_guard2264 != 0.0) {
        let assign97730_e150129: f64 = (locals.var_isbs2_btm * locals.var_t9);
        (assign97730_e150129, ((locals.var_isbs2_btm_dn0 * locals.var_t9) + (locals.var_isbs2_btm * locals.var_t9_dn0)), ((locals.var_isbs2_btm_dn2 * locals.var_t9) + (locals.var_isbs2_btm * locals.var_t9_dn2)), ((locals.var_isbs2_btm_dn4 * locals.var_t9) + (locals.var_isbs2_btm * locals.var_t9_dn4)), ((locals.var_isbs2_btm_dn5 * locals.var_t9) + (locals.var_isbs2_btm * locals.var_t9_dn5)), ((locals.var_isbs2_btm_dn6 * locals.var_t9) + (locals.var_isbs2_btm * locals.var_t9_dn6)), ((locals.var_isbs2_btm_dn7 * locals.var_t9) + (locals.var_isbs2_btm * locals.var_t9_dn7)), ((locals.var_isbs2_btm_dn8 * locals.var_t9) + (locals.var_isbs2_btm * locals.var_t9_dn8)), ((locals.var_isbs2_btm_dn9 * locals.var_t9) + (locals.var_isbs2_btm * locals.var_t9_dn9)), ((locals.var_isbs2_btm_dn10 * locals.var_t9) + (locals.var_isbs2_btm * locals.var_t9_dn10)), ((locals.var_isbs2_btm_dn13 * locals.var_t9) + (locals.var_isbs2_btm * locals.var_t9_dn13)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign97730_e150131;
        locals.var_t0_dn0 = assign97730_e150131_d_n0;
        locals.var_t0_dn2 = assign97730_e150131_d_n2;
        locals.var_t0_dn4 = assign97730_e150131_d_n4;
        locals.var_t0_dn5 = assign97730_e150131_d_n5;
        locals.var_t0_dn6 = assign97730_e150131_d_n6;
        locals.var_t0_dn7 = assign97730_e150131_d_n7;
        locals.var_t0_dn8 = assign97730_e150131_d_n8;
        locals.var_t0_dn9 = assign97730_e150131_d_n9;
        locals.var_t0_dn10 = assign97730_e150131_d_n10;
        locals.var_t0_dn13 = assign97730_e150131_d_n13;
        locals.var_t0_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_366(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign97740_e150138, assign97740_e150138_d_n0, assign97740_e150138_d_n2, assign97740_e150138_d_n4, assign97740_e150138_d_n5, assign97740_e150138_d_n6, assign97740_e150138_d_n7, assign97740_e150138_d_n8, assign97740_e150138_d_n9, assign97740_e150138_d_n10, assign97740_e150138_d_n13,) = {
    if (locals.var_guard2264 != 0.0) {
        let assign97740_e150134: f64 = (-locals.var_vbs_jct);
        let assign97740_e150136: f64 = (assign97740_e150134 * locals.var_t10);
        (assign97740_e150136, (assign97740_e150134 * locals.var_t10_dn0), (((-locals.var_vbs_jct_dn2) * locals.var_t10) + (assign97740_e150134 * locals.var_t10_dn2)), (assign97740_e150134 * locals.var_t10_dn4), (assign97740_e150134 * locals.var_t10_dn5), (assign97740_e150134 * locals.var_t10_dn6), (assign97740_e150134 * locals.var_t10_dn7), (assign97740_e150134 * locals.var_t10_dn8), (assign97740_e150134 * locals.var_t10_dn9), (((-locals.var_vbs_jct_dn10) * locals.var_t10) + (assign97740_e150134 * locals.var_t10_dn10)), (assign97740_e150134 * locals.var_t10_dn13),)
    } else {
        (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn4, locals.var_tx_dn5, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn8, locals.var_tx_dn9, locals.var_tx_dn10, locals.var_tx_dn13,)
    }
};
        locals.var_tx = assign97740_e150138;
        locals.var_tx_dn0 = assign97740_e150138_d_n0;
        locals.var_tx_dn2 = assign97740_e150138_d_n2;
        locals.var_tx_dn4 = assign97740_e150138_d_n4;
        locals.var_tx_dn5 = assign97740_e150138_d_n5;
        locals.var_tx_dn6 = assign97740_e150138_d_n6;
        locals.var_tx_dn7 = assign97740_e150138_d_n7;
        locals.var_tx_dn8 = assign97740_e150138_d_n8;
        locals.var_tx_dn9 = assign97740_e150138_d_n9;
        locals.var_tx_dn10 = assign97740_e150138_d_n10;
        locals.var_tx_dn13 = assign97740_e150138_d_n13;
        locals.var_tx_rv = 0.0;

        let (assign97750_e150143, assign97750_e150143_d_n0, assign97750_e150143_d_n2, assign97750_e150143_d_n4, assign97750_e150143_d_n5, assign97750_e150143_d_n6, assign97750_e150143_d_n7, assign97750_e150143_d_n8, assign97750_e150143_d_n9, assign97750_e150143_d_n10, assign97750_e150143_d_n13,) = {
    if (locals.var_guard2264 != 0.0) {
        let assign97750_e150141: f64 = (locals.var_tx).exp();
        (assign97750_e150141, (assign97750_e150141 * locals.var_tx_dn0), (assign97750_e150141 * locals.var_tx_dn2), (assign97750_e150141 * locals.var_tx_dn4), (assign97750_e150141 * locals.var_tx_dn5), (assign97750_e150141 * locals.var_tx_dn6), (assign97750_e150141 * locals.var_tx_dn7), (assign97750_e150141 * locals.var_tx_dn8), (assign97750_e150141 * locals.var_tx_dn9), (assign97750_e150141 * locals.var_tx_dn10), (assign97750_e150141 * locals.var_tx_dn13),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign97750_e150143;
        locals.var_t2_dn0 = assign97750_e150143_d_n0;
        locals.var_t2_dn2 = assign97750_e150143_d_n2;
        locals.var_t2_dn4 = assign97750_e150143_d_n4;
        locals.var_t2_dn5 = assign97750_e150143_d_n5;
        locals.var_t2_dn6 = assign97750_e150143_d_n6;
        locals.var_t2_dn7 = assign97750_e150143_d_n7;
        locals.var_t2_dn8 = assign97750_e150143_d_n8;
        locals.var_t2_dn9 = assign97750_e150143_d_n9;
        locals.var_t2_dn10 = assign97750_e150143_d_n10;
        locals.var_t2_dn13 = assign97750_e150143_d_n13;
        locals.var_t2_rv = 0.0;

        let (assign97760_e150147, assign97760_e150147_d_n0, assign97760_e150147_d_n2, assign97760_e150147_d_n4, assign97760_e150147_d_n5, assign97760_e150147_d_n6, assign97760_e150147_d_n7, assign97760_e150147_d_n8, assign97760_e150147_d_n9, assign97760_e150147_d_n10, assign97760_e150147_d_n13,) = {
    if (locals.var_guard2264 != 0.0) {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn13,)
    }
};
        locals.var_t3 = assign97760_e150147;
        locals.var_t3_dn0 = assign97760_e150147_d_n0;
        locals.var_t3_dn2 = assign97760_e150147_d_n2;
        locals.var_t3_dn4 = assign97760_e150147_d_n4;
        locals.var_t3_dn5 = assign97760_e150147_d_n5;
        locals.var_t3_dn6 = assign97760_e150147_d_n6;
        locals.var_t3_dn7 = assign97760_e150147_d_n7;
        locals.var_t3_dn8 = assign97760_e150147_d_n8;
        locals.var_t3_dn9 = assign97760_e150147_d_n9;
        locals.var_t3_dn10 = assign97760_e150147_d_n10;
        locals.var_t3_dn13 = assign97760_e150147_d_n13;
        locals.var_t3_rv = 0.0;

        let assign97770_e150150: f64 = if locals.var_vbs_jct < locals.var_vbst { 1.0 } else { 0.0 };
        locals.var_guard2265 = assign97770_e150150;
        locals.var_guard2265_rv = 0.0;

        let (assign97780_e150158, assign97780_e150158_d_n0, assign97780_e150158_d_n2, assign97780_e150158_d_n4, assign97780_e150158_d_n5, assign97780_e150158_d_n6, assign97780_e150158_d_n7, assign97780_e150158_d_n8, assign97780_e150158_d_n9, assign97780_e150158_d_n10, assign97780_e150158_d_n13,) = {
    if ((locals.var_guard2264 != 0.0) && (locals.var_guard2265 != 0.0)) {
        let assign97780_e150156: f64 = (locals.var_vbs_jct * locals.var_jd_nvtm_invs);
        (assign97780_e150156, (locals.var_vbs_jct * locals.var_jd_nvtm_invs_dn0), ((locals.var_vbs_jct_dn2 * locals.var_jd_nvtm_invs) + (locals.var_vbs_jct * locals.var_jd_nvtm_invs_dn2)), (locals.var_vbs_jct * locals.var_jd_nvtm_invs_dn4), (locals.var_vbs_jct * locals.var_jd_nvtm_invs_dn5), (locals.var_vbs_jct * locals.var_jd_nvtm_invs_dn6), (locals.var_vbs_jct * locals.var_jd_nvtm_invs_dn7), (locals.var_vbs_jct * locals.var_jd_nvtm_invs_dn8), (locals.var_vbs_jct * locals.var_jd_nvtm_invs_dn9), ((locals.var_vbs_jct_dn10 * locals.var_jd_nvtm_invs) + (locals.var_vbs_jct * locals.var_jd_nvtm_invs_dn10)), (locals.var_vbs_jct * locals.var_jd_nvtm_invs_dn13),)
    } else {
        (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn4, locals.var_tx_dn5, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn8, locals.var_tx_dn9, locals.var_tx_dn10, locals.var_tx_dn13,)
    }
};
        locals.var_tx = assign97780_e150158;
        locals.var_tx_dn0 = assign97780_e150158_d_n0;
        locals.var_tx_dn2 = assign97780_e150158_d_n2;
        locals.var_tx_dn4 = assign97780_e150158_d_n4;
        locals.var_tx_dn5 = assign97780_e150158_d_n5;
        locals.var_tx_dn6 = assign97780_e150158_d_n6;
        locals.var_tx_dn7 = assign97780_e150158_d_n7;
        locals.var_tx_dn8 = assign97780_e150158_d_n8;
        locals.var_tx_dn9 = assign97780_e150158_d_n9;
        locals.var_tx_dn10 = assign97780_e150158_d_n10;
        locals.var_tx_dn13 = assign97780_e150158_d_n13;
        locals.var_tx_rv = 0.0;

        let assign97790_e150161: f64 = (-3.0);
        let assign97790_e150163: f64 = (assign97790_e150161 * 34.0);
        let assign97790_e150164: f64 = if locals.var_tx < assign97790_e150163 { 1.0 } else { 0.0 };
        locals.var_guard2266 = assign97790_e150164;
        locals.var_guard2266_rv = 0.0;

        let (assign97800_e150172, assign97800_e150172_d_n0, assign97800_e150172_d_n2, assign97800_e150172_d_n4, assign97800_e150172_d_n5, assign97800_e150172_d_n6, assign97800_e150172_d_n7, assign97800_e150172_d_n8, assign97800_e150172_d_n9, assign97800_e150172_d_n10, assign97800_e150172_d_n13,) = {
    if (((locals.var_guard2264 != 0.0) && (locals.var_guard2265 != 0.0)) && (locals.var_guard2266 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign97800_e150172;
        locals.var_t1_dn0 = assign97800_e150172_d_n0;
        locals.var_t1_dn2 = assign97800_e150172_d_n2;
        locals.var_t1_dn4 = assign97800_e150172_d_n4;
        locals.var_t1_dn5 = assign97800_e150172_d_n5;
        locals.var_t1_dn6 = assign97800_e150172_d_n6;
        locals.var_t1_dn7 = assign97800_e150172_d_n7;
        locals.var_t1_dn8 = assign97800_e150172_d_n8;
        locals.var_t1_dn9 = assign97800_e150172_d_n9;
        locals.var_t1_dn10 = assign97800_e150172_d_n10;
        locals.var_t1_dn13 = assign97800_e150172_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign97810_e150182, assign97810_e150182_d_n0, assign97810_e150182_d_n2, assign97810_e150182_d_n4, assign97810_e150182_d_n5, assign97810_e150182_d_n6, assign97810_e150182_d_n7, assign97810_e150182_d_n8, assign97810_e150182_d_n9, assign97810_e150182_d_n10, assign97810_e150182_d_n13,) = {
    if (((locals.var_guard2264 != 0.0) && (locals.var_guard2265 != 0.0)) && (locals.var_guard2266 == 0.0)) {
        let assign97810_e150180: f64 = (locals.var_tx).exp();
        (assign97810_e150180, (assign97810_e150180 * locals.var_tx_dn0), (assign97810_e150180 * locals.var_tx_dn2), (assign97810_e150180 * locals.var_tx_dn4), (assign97810_e150180 * locals.var_tx_dn5), (assign97810_e150180 * locals.var_tx_dn6), (assign97810_e150180 * locals.var_tx_dn7), (assign97810_e150180 * locals.var_tx_dn8), (assign97810_e150180 * locals.var_tx_dn9), (assign97810_e150180 * locals.var_tx_dn10), (assign97810_e150180 * locals.var_tx_dn13),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign97810_e150182;
        locals.var_t1_dn0 = assign97810_e150182_d_n0;
        locals.var_t1_dn2 = assign97810_e150182_d_n2;
        locals.var_t1_dn4 = assign97810_e150182_d_n4;
        locals.var_t1_dn5 = assign97810_e150182_d_n5;
        locals.var_t1_dn6 = assign97810_e150182_d_n6;
        locals.var_t1_dn7 = assign97810_e150182_d_n7;
        locals.var_t1_dn8 = assign97810_e150182_d_n8;
        locals.var_t1_dn9 = assign97810_e150182_d_n9;
        locals.var_t1_dn10 = assign97810_e150182_d_n10;
        locals.var_t1_dn13 = assign97810_e150182_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign97830_e150211, assign97830_e150211_d_n0, assign97830_e150211_d_n2, assign97830_e150211_d_n4, assign97830_e150211_d_n5, assign97830_e150211_d_n6, assign97830_e150211_d_n7, assign97830_e150211_d_n8, assign97830_e150211_d_n9, assign97830_e150211_d_n10, assign97830_e150211_d_n13,) = {
    if ((locals.var_guard2264 != 0.0) && (locals.var_guard2265 == 0.0)) {
        (locals.var_jd_expcs, locals.var_jd_expcs_dn0, locals.var_jd_expcs_dn2, locals.var_jd_expcs_dn4, locals.var_jd_expcs_dn5, locals.var_jd_expcs_dn6, locals.var_jd_expcs_dn7, locals.var_jd_expcs_dn8, locals.var_jd_expcs_dn9, locals.var_jd_expcs_dn10, locals.var_jd_expcs_dn13,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign97830_e150211;
        locals.var_t1_dn0 = assign97830_e150211_d_n0;
        locals.var_t1_dn2 = assign97830_e150211_d_n2;
        locals.var_t1_dn4 = assign97830_e150211_d_n4;
        locals.var_t1_dn5 = assign97830_e150211_d_n5;
        locals.var_t1_dn6 = assign97830_e150211_d_n6;
        locals.var_t1_dn7 = assign97830_e150211_d_n7;
        locals.var_t1_dn8 = assign97830_e150211_d_n8;
        locals.var_t1_dn9 = assign97830_e150211_d_n9;
        locals.var_t1_dn10 = assign97830_e150211_d_n10;
        locals.var_t1_dn13 = assign97830_e150211_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign97840_e150222, assign97840_e150222_d_n0, assign97840_e150222_d_n2, assign97840_e150222_d_n4, assign97840_e150222_d_n5, assign97840_e150222_d_n6, assign97840_e150222_d_n7, assign97840_e150222_d_n8, assign97840_e150222_d_n9, assign97840_e150222_d_n10, assign97840_e150222_d_n13,) = {
    if ((locals.var_guard2264 != 0.0) && (locals.var_guard2265 == 0.0)) {
        let assign97840_e150218: f64 = (locals.var_isbs_btm * locals.var_jd_nvtm_invs);
        let assign97840_e150220: f64 = (assign97840_e150218 * locals.var_t1);
        (assign97840_e150220, ((((locals.var_isbs_btm_dn0 * locals.var_jd_nvtm_invs) + (locals.var_isbs_btm * locals.var_jd_nvtm_invs_dn0)) * locals.var_t1) + (assign97840_e150218 * locals.var_t1_dn0)), ((((locals.var_isbs_btm_dn2 * locals.var_jd_nvtm_invs) + (locals.var_isbs_btm * locals.var_jd_nvtm_invs_dn2)) * locals.var_t1) + (assign97840_e150218 * locals.var_t1_dn2)), ((((locals.var_isbs_btm_dn4 * locals.var_jd_nvtm_invs) + (locals.var_isbs_btm * locals.var_jd_nvtm_invs_dn4)) * locals.var_t1) + (assign97840_e150218 * locals.var_t1_dn4)), ((((locals.var_isbs_btm_dn5 * locals.var_jd_nvtm_invs) + (locals.var_isbs_btm * locals.var_jd_nvtm_invs_dn5)) * locals.var_t1) + (assign97840_e150218 * locals.var_t1_dn5)), ((((locals.var_isbs_btm_dn6 * locals.var_jd_nvtm_invs) + (locals.var_isbs_btm * locals.var_jd_nvtm_invs_dn6)) * locals.var_t1) + (assign97840_e150218 * locals.var_t1_dn6)), ((((locals.var_isbs_btm_dn7 * locals.var_jd_nvtm_invs) + (locals.var_isbs_btm * locals.var_jd_nvtm_invs_dn7)) * locals.var_t1) + (assign97840_e150218 * locals.var_t1_dn7)), ((((locals.var_isbs_btm_dn8 * locals.var_jd_nvtm_invs) + (locals.var_isbs_btm * locals.var_jd_nvtm_invs_dn8)) * locals.var_t1) + (assign97840_e150218 * locals.var_t1_dn8)), ((((locals.var_isbs_btm_dn9 * locals.var_jd_nvtm_invs) + (locals.var_isbs_btm * locals.var_jd_nvtm_invs_dn9)) * locals.var_t1) + (assign97840_e150218 * locals.var_t1_dn9)), ((((locals.var_isbs_btm_dn10 * locals.var_jd_nvtm_invs) + (locals.var_isbs_btm * locals.var_jd_nvtm_invs_dn10)) * locals.var_t1) + (assign97840_e150218 * locals.var_t1_dn10)), ((((locals.var_isbs_btm_dn13 * locals.var_jd_nvtm_invs) + (locals.var_isbs_btm * locals.var_jd_nvtm_invs_dn13)) * locals.var_t1) + (assign97840_e150218 * locals.var_t1_dn13)),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn13,)
    }
};
        locals.var_t4 = assign97840_e150222;
        locals.var_t4_dn0 = assign97840_e150222_d_n0;
        locals.var_t4_dn2 = assign97840_e150222_d_n2;
        locals.var_t4_dn4 = assign97840_e150222_d_n4;
        locals.var_t4_dn5 = assign97840_e150222_d_n5;
        locals.var_t4_dn6 = assign97840_e150222_d_n6;
        locals.var_t4_dn7 = assign97840_e150222_d_n7;
        locals.var_t4_dn8 = assign97840_e150222_d_n8;
        locals.var_t4_dn9 = assign97840_e150222_d_n9;
        locals.var_t4_dn10 = assign97840_e150222_d_n10;
        locals.var_t4_dn13 = assign97840_e150222_d_n13;
        locals.var_t4_rv = 0.0;

        let assign97870_e150259: f64 = (p.p537 * locals.var_isbs2_btm);
        locals.var_t12 = assign97870_e150259;
        locals.var_t12_dn0 = (p.p537 * locals.var_isbs2_btm_dn0);
        locals.var_t12_dn2 = (p.p537 * locals.var_isbs2_btm_dn2);
        locals.var_t12_dn4 = (p.p537 * locals.var_isbs2_btm_dn4);
        locals.var_t12_dn5 = (p.p537 * locals.var_isbs2_btm_dn5);
        locals.var_t12_dn6 = (p.p537 * locals.var_isbs2_btm_dn6);
        locals.var_t12_dn7 = (p.p537 * locals.var_isbs2_btm_dn7);
        locals.var_t12_dn8 = (p.p537 * locals.var_isbs2_btm_dn8);
        locals.var_t12_dn9 = (p.p537 * locals.var_isbs2_btm_dn9);
        locals.var_t12_dn10 = (p.p537 * locals.var_isbs2_btm_dn10);
        locals.var_t12_dn13 = (p.p537 * locals.var_isbs2_btm_dn13);
        locals.var_t12_rv = 0.0;

        let assign97890_e150267: f64 = if locals.var_isbs_sws > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2267 = assign97890_e150267;
        locals.var_guard2267_rv = 0.0;

        let (assign97900_e150273, assign97900_e150273_d_n0, assign97900_e150273_d_n2, assign97900_e150273_d_n4, assign97900_e150273_d_n5, assign97900_e150273_d_n6, assign97900_e150273_d_n7, assign97900_e150273_d_n8, assign97900_e150273_d_n9, assign97900_e150273_d_n10, assign97900_e150273_d_n13,) = {
    if (locals.var_guard2267 != 0.0) {
        let assign97900_e150271: f64 = (locals.var_isbs2_sws * locals.var_t9);
        (assign97900_e150271, ((locals.var_isbs2_sws_dn0 * locals.var_t9) + (locals.var_isbs2_sws * locals.var_t9_dn0)), ((locals.var_isbs2_sws_dn2 * locals.var_t9) + (locals.var_isbs2_sws * locals.var_t9_dn2)), ((locals.var_isbs2_sws_dn4 * locals.var_t9) + (locals.var_isbs2_sws * locals.var_t9_dn4)), ((locals.var_isbs2_sws_dn5 * locals.var_t9) + (locals.var_isbs2_sws * locals.var_t9_dn5)), ((locals.var_isbs2_sws_dn6 * locals.var_t9) + (locals.var_isbs2_sws * locals.var_t9_dn6)), ((locals.var_isbs2_sws_dn7 * locals.var_t9) + (locals.var_isbs2_sws * locals.var_t9_dn7)), ((locals.var_isbs2_sws_dn8 * locals.var_t9) + (locals.var_isbs2_sws * locals.var_t9_dn8)), ((locals.var_isbs2_sws_dn9 * locals.var_t9) + (locals.var_isbs2_sws * locals.var_t9_dn9)), ((locals.var_isbs2_sws_dn10 * locals.var_t9) + (locals.var_isbs2_sws * locals.var_t9_dn10)), ((locals.var_isbs2_sws_dn13 * locals.var_t9) + (locals.var_isbs2_sws * locals.var_t9_dn13)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign97900_e150273;
        locals.var_t0_dn0 = assign97900_e150273_d_n0;
        locals.var_t0_dn2 = assign97900_e150273_d_n2;
        locals.var_t0_dn4 = assign97900_e150273_d_n4;
        locals.var_t0_dn5 = assign97900_e150273_d_n5;
        locals.var_t0_dn6 = assign97900_e150273_d_n6;
        locals.var_t0_dn7 = assign97900_e150273_d_n7;
        locals.var_t0_dn8 = assign97900_e150273_d_n8;
        locals.var_t0_dn9 = assign97900_e150273_d_n9;
        locals.var_t0_dn10 = assign97900_e150273_d_n10;
        locals.var_t0_dn13 = assign97900_e150273_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign97910_e150280, assign97910_e150280_d_n0, assign97910_e150280_d_n2, assign97910_e150280_d_n4, assign97910_e150280_d_n5, assign97910_e150280_d_n6, assign97910_e150280_d_n7, assign97910_e150280_d_n8, assign97910_e150280_d_n9, assign97910_e150280_d_n10, assign97910_e150280_d_n13,) = {
    if (locals.var_guard2267 != 0.0) {
        let assign97910_e150276: f64 = (-locals.var_vbs_jct);
        let assign97910_e150278: f64 = (assign97910_e150276 * locals.var_t10);
        (assign97910_e150278, (assign97910_e150276 * locals.var_t10_dn0), (((-locals.var_vbs_jct_dn2) * locals.var_t10) + (assign97910_e150276 * locals.var_t10_dn2)), (assign97910_e150276 * locals.var_t10_dn4), (assign97910_e150276 * locals.var_t10_dn5), (assign97910_e150276 * locals.var_t10_dn6), (assign97910_e150276 * locals.var_t10_dn7), (assign97910_e150276 * locals.var_t10_dn8), (assign97910_e150276 * locals.var_t10_dn9), (((-locals.var_vbs_jct_dn10) * locals.var_t10) + (assign97910_e150276 * locals.var_t10_dn10)), (assign97910_e150276 * locals.var_t10_dn13),)
    } else {
        (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn4, locals.var_tx_dn5, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn8, locals.var_tx_dn9, locals.var_tx_dn10, locals.var_tx_dn13,)
    }
};
        locals.var_tx = assign97910_e150280;
        locals.var_tx_dn0 = assign97910_e150280_d_n0;
        locals.var_tx_dn2 = assign97910_e150280_d_n2;
        locals.var_tx_dn4 = assign97910_e150280_d_n4;
        locals.var_tx_dn5 = assign97910_e150280_d_n5;
        locals.var_tx_dn6 = assign97910_e150280_d_n6;
        locals.var_tx_dn7 = assign97910_e150280_d_n7;
        locals.var_tx_dn8 = assign97910_e150280_d_n8;
        locals.var_tx_dn9 = assign97910_e150280_d_n9;
        locals.var_tx_dn10 = assign97910_e150280_d_n10;
        locals.var_tx_dn13 = assign97910_e150280_d_n13;
        locals.var_tx_rv = 0.0;

        let (assign97920_e150285, assign97920_e150285_d_n0, assign97920_e150285_d_n2, assign97920_e150285_d_n4, assign97920_e150285_d_n5, assign97920_e150285_d_n6, assign97920_e150285_d_n7, assign97920_e150285_d_n8, assign97920_e150285_d_n9, assign97920_e150285_d_n10, assign97920_e150285_d_n13,) = {
    if (locals.var_guard2267 != 0.0) {
        let assign97920_e150283: f64 = (locals.var_tx).exp();
        (assign97920_e150283, (assign97920_e150283 * locals.var_tx_dn0), (assign97920_e150283 * locals.var_tx_dn2), (assign97920_e150283 * locals.var_tx_dn4), (assign97920_e150283 * locals.var_tx_dn5), (assign97920_e150283 * locals.var_tx_dn6), (assign97920_e150283 * locals.var_tx_dn7), (assign97920_e150283 * locals.var_tx_dn8), (assign97920_e150283 * locals.var_tx_dn9), (assign97920_e150283 * locals.var_tx_dn10), (assign97920_e150283 * locals.var_tx_dn13),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign97920_e150285;
        locals.var_t2_dn0 = assign97920_e150285_d_n0;
        locals.var_t2_dn2 = assign97920_e150285_d_n2;
        locals.var_t2_dn4 = assign97920_e150285_d_n4;
        locals.var_t2_dn5 = assign97920_e150285_d_n5;
        locals.var_t2_dn6 = assign97920_e150285_d_n6;
        locals.var_t2_dn7 = assign97920_e150285_d_n7;
        locals.var_t2_dn8 = assign97920_e150285_d_n8;
        locals.var_t2_dn9 = assign97920_e150285_d_n9;
        locals.var_t2_dn10 = assign97920_e150285_d_n10;
        locals.var_t2_dn13 = assign97920_e150285_d_n13;
        locals.var_t2_rv = 0.0;

        let (assign97930_e150289, assign97930_e150289_d_n0, assign97930_e150289_d_n2, assign97930_e150289_d_n4, assign97930_e150289_d_n5, assign97930_e150289_d_n6, assign97930_e150289_d_n7, assign97930_e150289_d_n8, assign97930_e150289_d_n9, assign97930_e150289_d_n10, assign97930_e150289_d_n13,) = {
    if (locals.var_guard2267 != 0.0) {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn13,)
    }
};
        locals.var_t3 = assign97930_e150289;
        locals.var_t3_dn0 = assign97930_e150289_d_n0;
        locals.var_t3_dn2 = assign97930_e150289_d_n2;
        locals.var_t3_dn4 = assign97930_e150289_d_n4;
        locals.var_t3_dn5 = assign97930_e150289_d_n5;
        locals.var_t3_dn6 = assign97930_e150289_d_n6;
        locals.var_t3_dn7 = assign97930_e150289_d_n7;
        locals.var_t3_dn8 = assign97930_e150289_d_n8;
        locals.var_t3_dn9 = assign97930_e150289_d_n9;
        locals.var_t3_dn10 = assign97930_e150289_d_n10;
        locals.var_t3_dn13 = assign97930_e150289_d_n13;
        locals.var_t3_rv = 0.0;

        let assign97940_e150292: f64 = if locals.var_vbs_jct < locals.var_vbst { 1.0 } else { 0.0 };
        locals.var_guard2268 = assign97940_e150292;
        locals.var_guard2268_rv = 0.0;

        let (assign97950_e150300, assign97950_e150300_d_n0, assign97950_e150300_d_n2, assign97950_e150300_d_n4, assign97950_e150300_d_n5, assign97950_e150300_d_n6, assign97950_e150300_d_n7, assign97950_e150300_d_n8, assign97950_e150300_d_n9, assign97950_e150300_d_n10, assign97950_e150300_d_n13,) = {
    if ((locals.var_guard2267 != 0.0) && (locals.var_guard2268 != 0.0)) {
        let assign97950_e150298: f64 = (locals.var_vbs_jct * locals.var_jd_nvtm_invs);
        (assign97950_e150298, (locals.var_vbs_jct * locals.var_jd_nvtm_invs_dn0), ((locals.var_vbs_jct_dn2 * locals.var_jd_nvtm_invs) + (locals.var_vbs_jct * locals.var_jd_nvtm_invs_dn2)), (locals.var_vbs_jct * locals.var_jd_nvtm_invs_dn4), (locals.var_vbs_jct * locals.var_jd_nvtm_invs_dn5), (locals.var_vbs_jct * locals.var_jd_nvtm_invs_dn6), (locals.var_vbs_jct * locals.var_jd_nvtm_invs_dn7), (locals.var_vbs_jct * locals.var_jd_nvtm_invs_dn8), (locals.var_vbs_jct * locals.var_jd_nvtm_invs_dn9), ((locals.var_vbs_jct_dn10 * locals.var_jd_nvtm_invs) + (locals.var_vbs_jct * locals.var_jd_nvtm_invs_dn10)), (locals.var_vbs_jct * locals.var_jd_nvtm_invs_dn13),)
    } else {
        (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn4, locals.var_tx_dn5, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn8, locals.var_tx_dn9, locals.var_tx_dn10, locals.var_tx_dn13,)
    }
};
        locals.var_tx = assign97950_e150300;
        locals.var_tx_dn0 = assign97950_e150300_d_n0;
        locals.var_tx_dn2 = assign97950_e150300_d_n2;
        locals.var_tx_dn4 = assign97950_e150300_d_n4;
        locals.var_tx_dn5 = assign97950_e150300_d_n5;
        locals.var_tx_dn6 = assign97950_e150300_d_n6;
        locals.var_tx_dn7 = assign97950_e150300_d_n7;
        locals.var_tx_dn8 = assign97950_e150300_d_n8;
        locals.var_tx_dn9 = assign97950_e150300_d_n9;
        locals.var_tx_dn10 = assign97950_e150300_d_n10;
        locals.var_tx_dn13 = assign97950_e150300_d_n13;
        locals.var_tx_rv = 0.0;

        let assign97960_e150303: f64 = (-3.0);
        let assign97960_e150305: f64 = (assign97960_e150303 * 34.0);
        let assign97960_e150306: f64 = if locals.var_tx < assign97960_e150305 { 1.0 } else { 0.0 };
        locals.var_guard2269 = assign97960_e150306;
        locals.var_guard2269_rv = 0.0;

        let (assign97970_e150314, assign97970_e150314_d_n0, assign97970_e150314_d_n2, assign97970_e150314_d_n4, assign97970_e150314_d_n5, assign97970_e150314_d_n6, assign97970_e150314_d_n7, assign97970_e150314_d_n8, assign97970_e150314_d_n9, assign97970_e150314_d_n10, assign97970_e150314_d_n13,) = {
    if (((locals.var_guard2267 != 0.0) && (locals.var_guard2268 != 0.0)) && (locals.var_guard2269 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign97970_e150314;
        locals.var_t1_dn0 = assign97970_e150314_d_n0;
        locals.var_t1_dn2 = assign97970_e150314_d_n2;
        locals.var_t1_dn4 = assign97970_e150314_d_n4;
        locals.var_t1_dn5 = assign97970_e150314_d_n5;
        locals.var_t1_dn6 = assign97970_e150314_d_n6;
        locals.var_t1_dn7 = assign97970_e150314_d_n7;
        locals.var_t1_dn8 = assign97970_e150314_d_n8;
        locals.var_t1_dn9 = assign97970_e150314_d_n9;
        locals.var_t1_dn10 = assign97970_e150314_d_n10;
        locals.var_t1_dn13 = assign97970_e150314_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign97980_e150324, assign97980_e150324_d_n0, assign97980_e150324_d_n2, assign97980_e150324_d_n4, assign97980_e150324_d_n5, assign97980_e150324_d_n6, assign97980_e150324_d_n7, assign97980_e150324_d_n8, assign97980_e150324_d_n9, assign97980_e150324_d_n10, assign97980_e150324_d_n13,) = {
    if (((locals.var_guard2267 != 0.0) && (locals.var_guard2268 != 0.0)) && (locals.var_guard2269 == 0.0)) {
        let assign97980_e150322: f64 = (locals.var_tx).exp();
        (assign97980_e150322, (assign97980_e150322 * locals.var_tx_dn0), (assign97980_e150322 * locals.var_tx_dn2), (assign97980_e150322 * locals.var_tx_dn4), (assign97980_e150322 * locals.var_tx_dn5), (assign97980_e150322 * locals.var_tx_dn6), (assign97980_e150322 * locals.var_tx_dn7), (assign97980_e150322 * locals.var_tx_dn8), (assign97980_e150322 * locals.var_tx_dn9), (assign97980_e150322 * locals.var_tx_dn10), (assign97980_e150322 * locals.var_tx_dn13),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign97980_e150324;
        locals.var_t1_dn0 = assign97980_e150324_d_n0;
        locals.var_t1_dn2 = assign97980_e150324_d_n2;
        locals.var_t1_dn4 = assign97980_e150324_d_n4;
        locals.var_t1_dn5 = assign97980_e150324_d_n5;
        locals.var_t1_dn6 = assign97980_e150324_d_n6;
        locals.var_t1_dn7 = assign97980_e150324_d_n7;
        locals.var_t1_dn8 = assign97980_e150324_d_n8;
        locals.var_t1_dn9 = assign97980_e150324_d_n9;
        locals.var_t1_dn10 = assign97980_e150324_d_n10;
        locals.var_t1_dn13 = assign97980_e150324_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign98000_e150353, assign98000_e150353_d_n0, assign98000_e150353_d_n2, assign98000_e150353_d_n4, assign98000_e150353_d_n5, assign98000_e150353_d_n6, assign98000_e150353_d_n7, assign98000_e150353_d_n8, assign98000_e150353_d_n9, assign98000_e150353_d_n10, assign98000_e150353_d_n13,) = {
    if ((locals.var_guard2267 != 0.0) && (locals.var_guard2268 == 0.0)) {
        (locals.var_jd_expcs, locals.var_jd_expcs_dn0, locals.var_jd_expcs_dn2, locals.var_jd_expcs_dn4, locals.var_jd_expcs_dn5, locals.var_jd_expcs_dn6, locals.var_jd_expcs_dn7, locals.var_jd_expcs_dn8, locals.var_jd_expcs_dn9, locals.var_jd_expcs_dn10, locals.var_jd_expcs_dn13,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign98000_e150353;
        locals.var_t1_dn0 = assign98000_e150353_d_n0;
        locals.var_t1_dn2 = assign98000_e150353_d_n2;
        locals.var_t1_dn4 = assign98000_e150353_d_n4;
        locals.var_t1_dn5 = assign98000_e150353_d_n5;
        locals.var_t1_dn6 = assign98000_e150353_d_n6;
        locals.var_t1_dn7 = assign98000_e150353_d_n7;
        locals.var_t1_dn8 = assign98000_e150353_d_n8;
        locals.var_t1_dn9 = assign98000_e150353_d_n9;
        locals.var_t1_dn10 = assign98000_e150353_d_n10;
        locals.var_t1_dn13 = assign98000_e150353_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign98010_e150364, assign98010_e150364_d_n0, assign98010_e150364_d_n2, assign98010_e150364_d_n4, assign98010_e150364_d_n5, assign98010_e150364_d_n6, assign98010_e150364_d_n7, assign98010_e150364_d_n8, assign98010_e150364_d_n9, assign98010_e150364_d_n10, assign98010_e150364_d_n13,) = {
    if ((locals.var_guard2267 != 0.0) && (locals.var_guard2268 == 0.0)) {
        let assign98010_e150360: f64 = (locals.var_isbs_sws * locals.var_jd_nvtm_invs);
        let assign98010_e150362: f64 = (assign98010_e150360 * locals.var_t1);
        (assign98010_e150362, ((((locals.var_isbs_sws_dn0 * locals.var_jd_nvtm_invs) + (locals.var_isbs_sws * locals.var_jd_nvtm_invs_dn0)) * locals.var_t1) + (assign98010_e150360 * locals.var_t1_dn0)), ((((locals.var_isbs_sws_dn2 * locals.var_jd_nvtm_invs) + (locals.var_isbs_sws * locals.var_jd_nvtm_invs_dn2)) * locals.var_t1) + (assign98010_e150360 * locals.var_t1_dn2)), ((((locals.var_isbs_sws_dn4 * locals.var_jd_nvtm_invs) + (locals.var_isbs_sws * locals.var_jd_nvtm_invs_dn4)) * locals.var_t1) + (assign98010_e150360 * locals.var_t1_dn4)), ((((locals.var_isbs_sws_dn5 * locals.var_jd_nvtm_invs) + (locals.var_isbs_sws * locals.var_jd_nvtm_invs_dn5)) * locals.var_t1) + (assign98010_e150360 * locals.var_t1_dn5)), ((((locals.var_isbs_sws_dn6 * locals.var_jd_nvtm_invs) + (locals.var_isbs_sws * locals.var_jd_nvtm_invs_dn6)) * locals.var_t1) + (assign98010_e150360 * locals.var_t1_dn6)), ((((locals.var_isbs_sws_dn7 * locals.var_jd_nvtm_invs) + (locals.var_isbs_sws * locals.var_jd_nvtm_invs_dn7)) * locals.var_t1) + (assign98010_e150360 * locals.var_t1_dn7)), ((((locals.var_isbs_sws_dn8 * locals.var_jd_nvtm_invs) + (locals.var_isbs_sws * locals.var_jd_nvtm_invs_dn8)) * locals.var_t1) + (assign98010_e150360 * locals.var_t1_dn8)), ((((locals.var_isbs_sws_dn9 * locals.var_jd_nvtm_invs) + (locals.var_isbs_sws * locals.var_jd_nvtm_invs_dn9)) * locals.var_t1) + (assign98010_e150360 * locals.var_t1_dn9)), ((((locals.var_isbs_sws_dn10 * locals.var_jd_nvtm_invs) + (locals.var_isbs_sws * locals.var_jd_nvtm_invs_dn10)) * locals.var_t1) + (assign98010_e150360 * locals.var_t1_dn10)), ((((locals.var_isbs_sws_dn13 * locals.var_jd_nvtm_invs) + (locals.var_isbs_sws * locals.var_jd_nvtm_invs_dn13)) * locals.var_t1) + (assign98010_e150360 * locals.var_t1_dn13)),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn13,)
    }
};
        locals.var_t4 = assign98010_e150364;
        locals.var_t4_dn0 = assign98010_e150364_d_n0;
        locals.var_t4_dn2 = assign98010_e150364_d_n2;
        locals.var_t4_dn4 = assign98010_e150364_d_n4;
        locals.var_t4_dn5 = assign98010_e150364_d_n5;
        locals.var_t4_dn6 = assign98010_e150364_d_n6;
        locals.var_t4_dn7 = assign98010_e150364_d_n7;
        locals.var_t4_dn8 = assign98010_e150364_d_n8;
        locals.var_t4_dn9 = assign98010_e150364_d_n9;
        locals.var_t4_dn10 = assign98010_e150364_d_n10;
        locals.var_t4_dn13 = assign98010_e150364_d_n13;
        locals.var_t4_rv = 0.0;

        let assign98040_e150401: f64 = (p.p537 * locals.var_isbs2_sws);
        locals.var_t12 = assign98040_e150401;
        locals.var_t12_dn0 = (p.p537 * locals.var_isbs2_sws_dn0);
        locals.var_t12_dn2 = (p.p537 * locals.var_isbs2_sws_dn2);
        locals.var_t12_dn4 = (p.p537 * locals.var_isbs2_sws_dn4);
        locals.var_t12_dn5 = (p.p537 * locals.var_isbs2_sws_dn5);
        locals.var_t12_dn6 = (p.p537 * locals.var_isbs2_sws_dn6);
        locals.var_t12_dn7 = (p.p537 * locals.var_isbs2_sws_dn7);
        locals.var_t12_dn8 = (p.p537 * locals.var_isbs2_sws_dn8);
        locals.var_t12_dn9 = (p.p537 * locals.var_isbs2_sws_dn9);
        locals.var_t12_dn10 = (p.p537 * locals.var_isbs2_sws_dn10);
        locals.var_t12_dn13 = (p.p537 * locals.var_isbs2_sws_dn13);
        locals.var_t12_rv = 0.0;

        let assign98060_e150409: f64 = if p.p48 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2270 = assign98060_e150409;
        locals.var_guard2270_rv = 0.0;

        let assign98070_e150412: f64 = if locals.var_isbs_swg > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2271 = assign98070_e150412;
        locals.var_guard2271_rv = 0.0;

        let (assign98080_e150420, assign98080_e150420_d_n0, assign98080_e150420_d_n2, assign98080_e150420_d_n4, assign98080_e150420_d_n5, assign98080_e150420_d_n6, assign98080_e150420_d_n7, assign98080_e150420_d_n8, assign98080_e150420_d_n9, assign98080_e150420_d_n10, assign98080_e150420_d_n13,) = {
    if ((locals.var_guard2270 != 0.0) && (locals.var_guard2271 != 0.0)) {
        let assign98080_e150418: f64 = (locals.var_isbs2_swg * locals.var_t9);
        (assign98080_e150418, ((locals.var_isbs2_swg_dn0 * locals.var_t9) + (locals.var_isbs2_swg * locals.var_t9_dn0)), ((locals.var_isbs2_swg_dn2 * locals.var_t9) + (locals.var_isbs2_swg * locals.var_t9_dn2)), ((locals.var_isbs2_swg_dn4 * locals.var_t9) + (locals.var_isbs2_swg * locals.var_t9_dn4)), ((locals.var_isbs2_swg_dn5 * locals.var_t9) + (locals.var_isbs2_swg * locals.var_t9_dn5)), ((locals.var_isbs2_swg_dn6 * locals.var_t9) + (locals.var_isbs2_swg * locals.var_t9_dn6)), ((locals.var_isbs2_swg_dn7 * locals.var_t9) + (locals.var_isbs2_swg * locals.var_t9_dn7)), ((locals.var_isbs2_swg_dn8 * locals.var_t9) + (locals.var_isbs2_swg * locals.var_t9_dn8)), ((locals.var_isbs2_swg_dn9 * locals.var_t9) + (locals.var_isbs2_swg * locals.var_t9_dn9)), ((locals.var_isbs2_swg_dn10 * locals.var_t9) + (locals.var_isbs2_swg * locals.var_t9_dn10)), ((locals.var_isbs2_swg_dn13 * locals.var_t9) + (locals.var_isbs2_swg * locals.var_t9_dn13)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign98080_e150420;
        locals.var_t0_dn0 = assign98080_e150420_d_n0;
        locals.var_t0_dn2 = assign98080_e150420_d_n2;
        locals.var_t0_dn4 = assign98080_e150420_d_n4;
        locals.var_t0_dn5 = assign98080_e150420_d_n5;
        locals.var_t0_dn6 = assign98080_e150420_d_n6;
        locals.var_t0_dn7 = assign98080_e150420_d_n7;
        locals.var_t0_dn8 = assign98080_e150420_d_n8;
        locals.var_t0_dn9 = assign98080_e150420_d_n9;
        locals.var_t0_dn10 = assign98080_e150420_d_n10;
        locals.var_t0_dn13 = assign98080_e150420_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign98090_e150429, assign98090_e150429_d_n0, assign98090_e150429_d_n2, assign98090_e150429_d_n4, assign98090_e150429_d_n5, assign98090_e150429_d_n6, assign98090_e150429_d_n7, assign98090_e150429_d_n8, assign98090_e150429_d_n9, assign98090_e150429_d_n10, assign98090_e150429_d_n13,) = {
    if ((locals.var_guard2270 != 0.0) && (locals.var_guard2271 != 0.0)) {
        let assign98090_e150425: f64 = (-locals.var_vbsi_jct);
        let assign98090_e150427: f64 = (assign98090_e150425 * locals.var_t10);
        (assign98090_e150427, (assign98090_e150425 * locals.var_t10_dn0), (assign98090_e150425 * locals.var_t10_dn2), (assign98090_e150425 * locals.var_t10_dn4), (assign98090_e150425 * locals.var_t10_dn5), (assign98090_e150425 * locals.var_t10_dn6), (((-locals.var_vbsi_jct_dn7) * locals.var_t10) + (assign98090_e150425 * locals.var_t10_dn7)), (((-locals.var_vbsi_jct_dn8) * locals.var_t10) + (assign98090_e150425 * locals.var_t10_dn8)), (assign98090_e150425 * locals.var_t10_dn9), (assign98090_e150425 * locals.var_t10_dn10), (assign98090_e150425 * locals.var_t10_dn13),)
    } else {
        (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn4, locals.var_tx_dn5, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn8, locals.var_tx_dn9, locals.var_tx_dn10, locals.var_tx_dn13,)
    }
};
        locals.var_tx = assign98090_e150429;
        locals.var_tx_dn0 = assign98090_e150429_d_n0;
        locals.var_tx_dn2 = assign98090_e150429_d_n2;
        locals.var_tx_dn4 = assign98090_e150429_d_n4;
        locals.var_tx_dn5 = assign98090_e150429_d_n5;
        locals.var_tx_dn6 = assign98090_e150429_d_n6;
        locals.var_tx_dn7 = assign98090_e150429_d_n7;
        locals.var_tx_dn8 = assign98090_e150429_d_n8;
        locals.var_tx_dn9 = assign98090_e150429_d_n9;
        locals.var_tx_dn10 = assign98090_e150429_d_n10;
        locals.var_tx_dn13 = assign98090_e150429_d_n13;
        locals.var_tx_rv = 0.0;

        let (assign98100_e150436, assign98100_e150436_d_n0, assign98100_e150436_d_n2, assign98100_e150436_d_n4, assign98100_e150436_d_n5, assign98100_e150436_d_n6, assign98100_e150436_d_n7, assign98100_e150436_d_n8, assign98100_e150436_d_n9, assign98100_e150436_d_n10, assign98100_e150436_d_n13,) = {
    if ((locals.var_guard2270 != 0.0) && (locals.var_guard2271 != 0.0)) {
        let assign98100_e150434: f64 = (locals.var_tx).exp();
        (assign98100_e150434, (assign98100_e150434 * locals.var_tx_dn0), (assign98100_e150434 * locals.var_tx_dn2), (assign98100_e150434 * locals.var_tx_dn4), (assign98100_e150434 * locals.var_tx_dn5), (assign98100_e150434 * locals.var_tx_dn6), (assign98100_e150434 * locals.var_tx_dn7), (assign98100_e150434 * locals.var_tx_dn8), (assign98100_e150434 * locals.var_tx_dn9), (assign98100_e150434 * locals.var_tx_dn10), (assign98100_e150434 * locals.var_tx_dn13),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign98100_e150436;
        locals.var_t2_dn0 = assign98100_e150436_d_n0;
        locals.var_t2_dn2 = assign98100_e150436_d_n2;
        locals.var_t2_dn4 = assign98100_e150436_d_n4;
        locals.var_t2_dn5 = assign98100_e150436_d_n5;
        locals.var_t2_dn6 = assign98100_e150436_d_n6;
        locals.var_t2_dn7 = assign98100_e150436_d_n7;
        locals.var_t2_dn8 = assign98100_e150436_d_n8;
        locals.var_t2_dn9 = assign98100_e150436_d_n9;
        locals.var_t2_dn10 = assign98100_e150436_d_n10;
        locals.var_t2_dn13 = assign98100_e150436_d_n13;
        locals.var_t2_rv = 0.0;

        let (assign98110_e150442, assign98110_e150442_d_n0, assign98110_e150442_d_n2, assign98110_e150442_d_n4, assign98110_e150442_d_n5, assign98110_e150442_d_n6, assign98110_e150442_d_n7, assign98110_e150442_d_n8, assign98110_e150442_d_n9, assign98110_e150442_d_n10, assign98110_e150442_d_n13,) = {
    if ((locals.var_guard2270 != 0.0) && (locals.var_guard2271 != 0.0)) {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn13,)
    }
};
        locals.var_t3 = assign98110_e150442;
        locals.var_t3_dn0 = assign98110_e150442_d_n0;
        locals.var_t3_dn2 = assign98110_e150442_d_n2;
        locals.var_t3_dn4 = assign98110_e150442_d_n4;
        locals.var_t3_dn5 = assign98110_e150442_d_n5;
        locals.var_t3_dn6 = assign98110_e150442_d_n6;
        locals.var_t3_dn7 = assign98110_e150442_d_n7;
        locals.var_t3_dn8 = assign98110_e150442_d_n8;
        locals.var_t3_dn9 = assign98110_e150442_d_n9;
        locals.var_t3_dn10 = assign98110_e150442_d_n10;
        locals.var_t3_dn13 = assign98110_e150442_d_n13;
        locals.var_t3_rv = 0.0;

        let assign98120_e150445: f64 = if locals.var_vbsi_jct < locals.var_vbst { 1.0 } else { 0.0 };
        locals.var_guard2272 = assign98120_e150445;
        locals.var_guard2272_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_367(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign98130_e150455, assign98130_e150455_d_n0, assign98130_e150455_d_n2, assign98130_e150455_d_n4, assign98130_e150455_d_n5, assign98130_e150455_d_n6, assign98130_e150455_d_n7, assign98130_e150455_d_n8, assign98130_e150455_d_n9, assign98130_e150455_d_n10, assign98130_e150455_d_n13,) = {
    if (((locals.var_guard2270 != 0.0) && (locals.var_guard2271 != 0.0)) && (locals.var_guard2272 != 0.0)) {
        let assign98130_e150453: f64 = (locals.var_vbsi_jct * locals.var_jd_nvtm_invs);
        (assign98130_e150453, (locals.var_vbsi_jct * locals.var_jd_nvtm_invs_dn0), (locals.var_vbsi_jct * locals.var_jd_nvtm_invs_dn2), (locals.var_vbsi_jct * locals.var_jd_nvtm_invs_dn4), (locals.var_vbsi_jct * locals.var_jd_nvtm_invs_dn5), (locals.var_vbsi_jct * locals.var_jd_nvtm_invs_dn6), ((locals.var_vbsi_jct_dn7 * locals.var_jd_nvtm_invs) + (locals.var_vbsi_jct * locals.var_jd_nvtm_invs_dn7)), ((locals.var_vbsi_jct_dn8 * locals.var_jd_nvtm_invs) + (locals.var_vbsi_jct * locals.var_jd_nvtm_invs_dn8)), (locals.var_vbsi_jct * locals.var_jd_nvtm_invs_dn9), (locals.var_vbsi_jct * locals.var_jd_nvtm_invs_dn10), (locals.var_vbsi_jct * locals.var_jd_nvtm_invs_dn13),)
    } else {
        (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn4, locals.var_tx_dn5, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn8, locals.var_tx_dn9, locals.var_tx_dn10, locals.var_tx_dn13,)
    }
};
        locals.var_tx = assign98130_e150455;
        locals.var_tx_dn0 = assign98130_e150455_d_n0;
        locals.var_tx_dn2 = assign98130_e150455_d_n2;
        locals.var_tx_dn4 = assign98130_e150455_d_n4;
        locals.var_tx_dn5 = assign98130_e150455_d_n5;
        locals.var_tx_dn6 = assign98130_e150455_d_n6;
        locals.var_tx_dn7 = assign98130_e150455_d_n7;
        locals.var_tx_dn8 = assign98130_e150455_d_n8;
        locals.var_tx_dn9 = assign98130_e150455_d_n9;
        locals.var_tx_dn10 = assign98130_e150455_d_n10;
        locals.var_tx_dn13 = assign98130_e150455_d_n13;
        locals.var_tx_rv = 0.0;

        let assign98140_e150458: f64 = (-3.0);
        let assign98140_e150460: f64 = (assign98140_e150458 * 34.0);
        let assign98140_e150461: f64 = if locals.var_tx < assign98140_e150460 { 1.0 } else { 0.0 };
        locals.var_guard2273 = assign98140_e150461;
        locals.var_guard2273_rv = 0.0;

        let (assign98150_e150471, assign98150_e150471_d_n0, assign98150_e150471_d_n2, assign98150_e150471_d_n4, assign98150_e150471_d_n5, assign98150_e150471_d_n6, assign98150_e150471_d_n7, assign98150_e150471_d_n8, assign98150_e150471_d_n9, assign98150_e150471_d_n10, assign98150_e150471_d_n13,) = {
    if ((((locals.var_guard2270 != 0.0) && (locals.var_guard2271 != 0.0)) && (locals.var_guard2272 != 0.0)) && (locals.var_guard2273 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign98150_e150471;
        locals.var_t1_dn0 = assign98150_e150471_d_n0;
        locals.var_t1_dn2 = assign98150_e150471_d_n2;
        locals.var_t1_dn4 = assign98150_e150471_d_n4;
        locals.var_t1_dn5 = assign98150_e150471_d_n5;
        locals.var_t1_dn6 = assign98150_e150471_d_n6;
        locals.var_t1_dn7 = assign98150_e150471_d_n7;
        locals.var_t1_dn8 = assign98150_e150471_d_n8;
        locals.var_t1_dn9 = assign98150_e150471_d_n9;
        locals.var_t1_dn10 = assign98150_e150471_d_n10;
        locals.var_t1_dn13 = assign98150_e150471_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign98160_e150483, assign98160_e150483_d_n0, assign98160_e150483_d_n2, assign98160_e150483_d_n4, assign98160_e150483_d_n5, assign98160_e150483_d_n6, assign98160_e150483_d_n7, assign98160_e150483_d_n8, assign98160_e150483_d_n9, assign98160_e150483_d_n10, assign98160_e150483_d_n13,) = {
    if ((((locals.var_guard2270 != 0.0) && (locals.var_guard2271 != 0.0)) && (locals.var_guard2272 != 0.0)) && (locals.var_guard2273 == 0.0)) {
        let assign98160_e150481: f64 = (locals.var_tx).exp();
        (assign98160_e150481, (assign98160_e150481 * locals.var_tx_dn0), (assign98160_e150481 * locals.var_tx_dn2), (assign98160_e150481 * locals.var_tx_dn4), (assign98160_e150481 * locals.var_tx_dn5), (assign98160_e150481 * locals.var_tx_dn6), (assign98160_e150481 * locals.var_tx_dn7), (assign98160_e150481 * locals.var_tx_dn8), (assign98160_e150481 * locals.var_tx_dn9), (assign98160_e150481 * locals.var_tx_dn10), (assign98160_e150481 * locals.var_tx_dn13),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign98160_e150483;
        locals.var_t1_dn0 = assign98160_e150483_d_n0;
        locals.var_t1_dn2 = assign98160_e150483_d_n2;
        locals.var_t1_dn4 = assign98160_e150483_d_n4;
        locals.var_t1_dn5 = assign98160_e150483_d_n5;
        locals.var_t1_dn6 = assign98160_e150483_d_n6;
        locals.var_t1_dn7 = assign98160_e150483_d_n7;
        locals.var_t1_dn8 = assign98160_e150483_d_n8;
        locals.var_t1_dn9 = assign98160_e150483_d_n9;
        locals.var_t1_dn10 = assign98160_e150483_d_n10;
        locals.var_t1_dn13 = assign98160_e150483_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign98180_e150516, assign98180_e150516_d_n0, assign98180_e150516_d_n2, assign98180_e150516_d_n4, assign98180_e150516_d_n5, assign98180_e150516_d_n6, assign98180_e150516_d_n7, assign98180_e150516_d_n8, assign98180_e150516_d_n9, assign98180_e150516_d_n10, assign98180_e150516_d_n13,) = {
    if (((locals.var_guard2270 != 0.0) && (locals.var_guard2271 != 0.0)) && (locals.var_guard2272 == 0.0)) {
        (locals.var_jd_expcs, locals.var_jd_expcs_dn0, locals.var_jd_expcs_dn2, locals.var_jd_expcs_dn4, locals.var_jd_expcs_dn5, locals.var_jd_expcs_dn6, locals.var_jd_expcs_dn7, locals.var_jd_expcs_dn8, locals.var_jd_expcs_dn9, locals.var_jd_expcs_dn10, locals.var_jd_expcs_dn13,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign98180_e150516;
        locals.var_t1_dn0 = assign98180_e150516_d_n0;
        locals.var_t1_dn2 = assign98180_e150516_d_n2;
        locals.var_t1_dn4 = assign98180_e150516_d_n4;
        locals.var_t1_dn5 = assign98180_e150516_d_n5;
        locals.var_t1_dn6 = assign98180_e150516_d_n6;
        locals.var_t1_dn7 = assign98180_e150516_d_n7;
        locals.var_t1_dn8 = assign98180_e150516_d_n8;
        locals.var_t1_dn9 = assign98180_e150516_d_n9;
        locals.var_t1_dn10 = assign98180_e150516_d_n10;
        locals.var_t1_dn13 = assign98180_e150516_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign98190_e150529, assign98190_e150529_d_n0, assign98190_e150529_d_n2, assign98190_e150529_d_n4, assign98190_e150529_d_n5, assign98190_e150529_d_n6, assign98190_e150529_d_n7, assign98190_e150529_d_n8, assign98190_e150529_d_n9, assign98190_e150529_d_n10, assign98190_e150529_d_n13,) = {
    if (((locals.var_guard2270 != 0.0) && (locals.var_guard2271 != 0.0)) && (locals.var_guard2272 == 0.0)) {
        let assign98190_e150525: f64 = (locals.var_isbs_swg * locals.var_jd_nvtm_invs);
        let assign98190_e150527: f64 = (assign98190_e150525 * locals.var_t1);
        (assign98190_e150527, ((((locals.var_isbs_swg_dn0 * locals.var_jd_nvtm_invs) + (locals.var_isbs_swg * locals.var_jd_nvtm_invs_dn0)) * locals.var_t1) + (assign98190_e150525 * locals.var_t1_dn0)), ((((locals.var_isbs_swg_dn2 * locals.var_jd_nvtm_invs) + (locals.var_isbs_swg * locals.var_jd_nvtm_invs_dn2)) * locals.var_t1) + (assign98190_e150525 * locals.var_t1_dn2)), ((((locals.var_isbs_swg_dn4 * locals.var_jd_nvtm_invs) + (locals.var_isbs_swg * locals.var_jd_nvtm_invs_dn4)) * locals.var_t1) + (assign98190_e150525 * locals.var_t1_dn4)), ((((locals.var_isbs_swg_dn5 * locals.var_jd_nvtm_invs) + (locals.var_isbs_swg * locals.var_jd_nvtm_invs_dn5)) * locals.var_t1) + (assign98190_e150525 * locals.var_t1_dn5)), ((((locals.var_isbs_swg_dn6 * locals.var_jd_nvtm_invs) + (locals.var_isbs_swg * locals.var_jd_nvtm_invs_dn6)) * locals.var_t1) + (assign98190_e150525 * locals.var_t1_dn6)), ((((locals.var_isbs_swg_dn7 * locals.var_jd_nvtm_invs) + (locals.var_isbs_swg * locals.var_jd_nvtm_invs_dn7)) * locals.var_t1) + (assign98190_e150525 * locals.var_t1_dn7)), ((((locals.var_isbs_swg_dn8 * locals.var_jd_nvtm_invs) + (locals.var_isbs_swg * locals.var_jd_nvtm_invs_dn8)) * locals.var_t1) + (assign98190_e150525 * locals.var_t1_dn8)), ((((locals.var_isbs_swg_dn9 * locals.var_jd_nvtm_invs) + (locals.var_isbs_swg * locals.var_jd_nvtm_invs_dn9)) * locals.var_t1) + (assign98190_e150525 * locals.var_t1_dn9)), ((((locals.var_isbs_swg_dn10 * locals.var_jd_nvtm_invs) + (locals.var_isbs_swg * locals.var_jd_nvtm_invs_dn10)) * locals.var_t1) + (assign98190_e150525 * locals.var_t1_dn10)), ((((locals.var_isbs_swg_dn13 * locals.var_jd_nvtm_invs) + (locals.var_isbs_swg * locals.var_jd_nvtm_invs_dn13)) * locals.var_t1) + (assign98190_e150525 * locals.var_t1_dn13)),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn13,)
    }
};
        locals.var_t4 = assign98190_e150529;
        locals.var_t4_dn0 = assign98190_e150529_d_n0;
        locals.var_t4_dn2 = assign98190_e150529_d_n2;
        locals.var_t4_dn4 = assign98190_e150529_d_n4;
        locals.var_t4_dn5 = assign98190_e150529_d_n5;
        locals.var_t4_dn6 = assign98190_e150529_d_n6;
        locals.var_t4_dn7 = assign98190_e150529_d_n7;
        locals.var_t4_dn8 = assign98190_e150529_d_n8;
        locals.var_t4_dn9 = assign98190_e150529_d_n9;
        locals.var_t4_dn10 = assign98190_e150529_d_n10;
        locals.var_t4_dn13 = assign98190_e150529_d_n13;
        locals.var_t4_rv = 0.0;

        let (assign98220_e150573, assign98220_e150573_d_n0, assign98220_e150573_d_n2, assign98220_e150573_d_n4, assign98220_e150573_d_n5, assign98220_e150573_d_n6, assign98220_e150573_d_n7, assign98220_e150573_d_n8, assign98220_e150573_d_n9, assign98220_e150573_d_n10, assign98220_e150573_d_n13,) = {
    if (locals.var_guard2270 != 0.0) {
        let assign98220_e150571: f64 = (p.p537 * locals.var_isbs2_swg);
        (assign98220_e150571, (p.p537 * locals.var_isbs2_swg_dn0), (p.p537 * locals.var_isbs2_swg_dn2), (p.p537 * locals.var_isbs2_swg_dn4), (p.p537 * locals.var_isbs2_swg_dn5), (p.p537 * locals.var_isbs2_swg_dn6), (p.p537 * locals.var_isbs2_swg_dn7), (p.p537 * locals.var_isbs2_swg_dn8), (p.p537 * locals.var_isbs2_swg_dn9), (p.p537 * locals.var_isbs2_swg_dn10), (p.p537 * locals.var_isbs2_swg_dn13),)
    } else {
        (locals.var_t12, locals.var_t12_dn0, locals.var_t12_dn2, locals.var_t12_dn4, locals.var_t12_dn5, locals.var_t12_dn6, locals.var_t12_dn7, locals.var_t12_dn8, locals.var_t12_dn9, locals.var_t12_dn10, locals.var_t12_dn13,)
    }
};
        locals.var_t12 = assign98220_e150573;
        locals.var_t12_dn0 = assign98220_e150573_d_n0;
        locals.var_t12_dn2 = assign98220_e150573_d_n2;
        locals.var_t12_dn4 = assign98220_e150573_d_n4;
        locals.var_t12_dn5 = assign98220_e150573_d_n5;
        locals.var_t12_dn6 = assign98220_e150573_d_n6;
        locals.var_t12_dn7 = assign98220_e150573_d_n7;
        locals.var_t12_dn8 = assign98220_e150573_d_n8;
        locals.var_t12_dn9 = assign98220_e150573_d_n9;
        locals.var_t12_dn10 = assign98220_e150573_d_n10;
        locals.var_t12_dn13 = assign98220_e150573_d_n13;
        locals.var_t12_rv = 0.0;

        let assign98250_e150589: f64 = if locals.var_czbd > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2274 = assign98250_e150589;
        locals.var_guard2274_rv = 0.0;

        let assign98260_e150592: f64 = if locals.var_vbd_jct < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2275 = assign98260_e150592;
        locals.var_guard2275_rv = 0.0;

        let (assign98270_e150602, assign98270_e150602_d_n0, assign98270_e150602_d_n2, assign98270_e150602_d_n4, assign98270_e150602_d_n5, assign98270_e150602_d_n6, assign98270_e150602_d_n7, assign98270_e150602_d_n8, assign98270_e150602_d_n9, assign98270_e150602_d_n10, assign98270_e150602_d_n13,) = {
    if ((locals.var_guard2274 != 0.0) && (locals.var_guard2275 != 0.0)) {
        let assign98270_e150599: f64 = (locals.var_vbd_jct / locals.var_pzbd);
        let assign98270_e150600: f64 = (1.0 - assign98270_e150599);
        (assign98270_e150600, (-(((locals.var_vbd_jct_dn0 * locals.var_pzbd) - (locals.var_vbd_jct * locals.var_pzbd_dn0)) / (locals.var_pzbd * locals.var_pzbd))), (-(-((locals.var_vbd_jct * locals.var_pzbd_dn2) / (locals.var_pzbd * locals.var_pzbd)))), (-(-((locals.var_vbd_jct * locals.var_pzbd_dn4) / (locals.var_pzbd * locals.var_pzbd)))), (-(-((locals.var_vbd_jct * locals.var_pzbd_dn5) / (locals.var_pzbd * locals.var_pzbd)))), (-(-((locals.var_vbd_jct * locals.var_pzbd_dn6) / (locals.var_pzbd * locals.var_pzbd)))), (-(-((locals.var_vbd_jct * locals.var_pzbd_dn7) / (locals.var_pzbd * locals.var_pzbd)))), (-(-((locals.var_vbd_jct * locals.var_pzbd_dn8) / (locals.var_pzbd * locals.var_pzbd)))), (-(((locals.var_vbd_jct_dn9 * locals.var_pzbd) - (locals.var_vbd_jct * locals.var_pzbd_dn9)) / (locals.var_pzbd * locals.var_pzbd))), (-(-((locals.var_vbd_jct * locals.var_pzbd_dn10) / (locals.var_pzbd * locals.var_pzbd)))), (-(-((locals.var_vbd_jct * locals.var_pzbd_dn13) / (locals.var_pzbd * locals.var_pzbd)))),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    }
};
        locals.var_arg = assign98270_e150602;
        locals.var_arg_dn0 = assign98270_e150602_d_n0;
        locals.var_arg_dn2 = assign98270_e150602_d_n2;
        locals.var_arg_dn4 = assign98270_e150602_d_n4;
        locals.var_arg_dn5 = assign98270_e150602_d_n5;
        locals.var_arg_dn6 = assign98270_e150602_d_n6;
        locals.var_arg_dn7 = assign98270_e150602_d_n7;
        locals.var_arg_dn8 = assign98270_e150602_d_n8;
        locals.var_arg_dn9 = assign98270_e150602_d_n9;
        locals.var_arg_dn10 = assign98270_e150602_d_n10;
        locals.var_arg_dn13 = assign98270_e150602_d_n13;
        locals.var_arg_rv = 0.0;

        let assign98280_e150605: f64 = if p.p503 == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard2276 = assign98280_e150605;
        locals.var_guard2276_rv = 0.0;

        let (assign98290_e150616, assign98290_e150616_d_n0, assign98290_e150616_d_n2, assign98290_e150616_d_n4, assign98290_e150616_d_n5, assign98290_e150616_d_n6, assign98290_e150616_d_n7, assign98290_e150616_d_n8, assign98290_e150616_d_n9, assign98290_e150616_d_n10, assign98290_e150616_d_n13,) = {
    if (((locals.var_guard2274 != 0.0) && (locals.var_guard2275 != 0.0)) && (locals.var_guard2276 != 0.0)) {
        let assign98290_e150613: f64 = (locals.var_arg).sqrt();
        let assign98290_e150614: f64 = (1.0 / assign98290_e150613);
        (assign98290_e150614, (-((locals.var_arg_dn0 / (2.0 * assign98290_e150613)) / (assign98290_e150613 * assign98290_e150613))), (-((locals.var_arg_dn2 / (2.0 * assign98290_e150613)) / (assign98290_e150613 * assign98290_e150613))), (-((locals.var_arg_dn4 / (2.0 * assign98290_e150613)) / (assign98290_e150613 * assign98290_e150613))), (-((locals.var_arg_dn5 / (2.0 * assign98290_e150613)) / (assign98290_e150613 * assign98290_e150613))), (-((locals.var_arg_dn6 / (2.0 * assign98290_e150613)) / (assign98290_e150613 * assign98290_e150613))), (-((locals.var_arg_dn7 / (2.0 * assign98290_e150613)) / (assign98290_e150613 * assign98290_e150613))), (-((locals.var_arg_dn8 / (2.0 * assign98290_e150613)) / (assign98290_e150613 * assign98290_e150613))), (-((locals.var_arg_dn9 / (2.0 * assign98290_e150613)) / (assign98290_e150613 * assign98290_e150613))), (-((locals.var_arg_dn10 / (2.0 * assign98290_e150613)) / (assign98290_e150613 * assign98290_e150613))), (-((locals.var_arg_dn13 / (2.0 * assign98290_e150613)) / (assign98290_e150613 * assign98290_e150613))),)
    } else {
        (locals.var_sarg, locals.var_sarg_dn0, locals.var_sarg_dn2, locals.var_sarg_dn4, locals.var_sarg_dn5, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn8, locals.var_sarg_dn9, locals.var_sarg_dn10, locals.var_sarg_dn13,)
    }
};
        locals.var_sarg = assign98290_e150616;
        locals.var_sarg_dn0 = assign98290_e150616_d_n0;
        locals.var_sarg_dn2 = assign98290_e150616_d_n2;
        locals.var_sarg_dn4 = assign98290_e150616_d_n4;
        locals.var_sarg_dn5 = assign98290_e150616_d_n5;
        locals.var_sarg_dn6 = assign98290_e150616_d_n6;
        locals.var_sarg_dn7 = assign98290_e150616_d_n7;
        locals.var_sarg_dn8 = assign98290_e150616_d_n8;
        locals.var_sarg_dn9 = assign98290_e150616_d_n9;
        locals.var_sarg_dn10 = assign98290_e150616_d_n10;
        locals.var_sarg_dn13 = assign98290_e150616_d_n13;
        locals.var_sarg_rv = 0.0;

        let (assign98300_e150633, assign98300_e150633_d_n0, assign98300_e150633_d_n2, assign98300_e150633_d_n4, assign98300_e150633_d_n5, assign98300_e150633_d_n6, assign98300_e150633_d_n7, assign98300_e150633_d_n8, assign98300_e150633_d_n9, assign98300_e150633_d_n10, assign98300_e150633_d_n13,) = {
    if (((locals.var_guard2274 != 0.0) && (locals.var_guard2275 != 0.0)) && (locals.var_guard2276 == 0.0)) {
        let (assign98300_e150631, assign98300_e150631_d_n0, assign98300_e150631_d_n2, assign98300_e150631_d_n4, assign98300_e150631_d_n5, assign98300_e150631_d_n6, assign98300_e150631_d_n7, assign98300_e150631_d_n8, assign98300_e150631_d_n9, assign98300_e150631_d_n10, assign98300_e150631_d_n13,) = {
            if (locals.var_arg == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign98300_e150629: f64 = (-p.p503);
                let assign98300_e150630: f64 = (locals.var_arg).powf(assign98300_e150629);
                (assign98300_e150630, if 0.0 == 0.0 && ((assign98300_e150629) as f64).is_finite() && ((assign98300_e150629) as f64).fract() == 0.0 { if assign98300_e150629 == 0.0 { 0.0 } else { (assign98300_e150629 * ((locals.var_arg).powf(assign98300_e150629 - 1.0) * locals.var_arg_dn0)) } } else { (assign98300_e150630 * (assign98300_e150629 * (locals.var_arg_dn0 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign98300_e150629) as f64).is_finite() && ((assign98300_e150629) as f64).fract() == 0.0 { if assign98300_e150629 == 0.0 { 0.0 } else { (assign98300_e150629 * ((locals.var_arg).powf(assign98300_e150629 - 1.0) * locals.var_arg_dn2)) } } else { (assign98300_e150630 * (assign98300_e150629 * (locals.var_arg_dn2 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign98300_e150629) as f64).is_finite() && ((assign98300_e150629) as f64).fract() == 0.0 { if assign98300_e150629 == 0.0 { 0.0 } else { (assign98300_e150629 * ((locals.var_arg).powf(assign98300_e150629 - 1.0) * locals.var_arg_dn4)) } } else { (assign98300_e150630 * (assign98300_e150629 * (locals.var_arg_dn4 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign98300_e150629) as f64).is_finite() && ((assign98300_e150629) as f64).fract() == 0.0 { if assign98300_e150629 == 0.0 { 0.0 } else { (assign98300_e150629 * ((locals.var_arg).powf(assign98300_e150629 - 1.0) * locals.var_arg_dn5)) } } else { (assign98300_e150630 * (assign98300_e150629 * (locals.var_arg_dn5 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign98300_e150629) as f64).is_finite() && ((assign98300_e150629) as f64).fract() == 0.0 { if assign98300_e150629 == 0.0 { 0.0 } else { (assign98300_e150629 * ((locals.var_arg).powf(assign98300_e150629 - 1.0) * locals.var_arg_dn6)) } } else { (assign98300_e150630 * (assign98300_e150629 * (locals.var_arg_dn6 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign98300_e150629) as f64).is_finite() && ((assign98300_e150629) as f64).fract() == 0.0 { if assign98300_e150629 == 0.0 { 0.0 } else { (assign98300_e150629 * ((locals.var_arg).powf(assign98300_e150629 - 1.0) * locals.var_arg_dn7)) } } else { (assign98300_e150630 * (assign98300_e150629 * (locals.var_arg_dn7 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign98300_e150629) as f64).is_finite() && ((assign98300_e150629) as f64).fract() == 0.0 { if assign98300_e150629 == 0.0 { 0.0 } else { (assign98300_e150629 * ((locals.var_arg).powf(assign98300_e150629 - 1.0) * locals.var_arg_dn8)) } } else { (assign98300_e150630 * (assign98300_e150629 * (locals.var_arg_dn8 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign98300_e150629) as f64).is_finite() && ((assign98300_e150629) as f64).fract() == 0.0 { if assign98300_e150629 == 0.0 { 0.0 } else { (assign98300_e150629 * ((locals.var_arg).powf(assign98300_e150629 - 1.0) * locals.var_arg_dn9)) } } else { (assign98300_e150630 * (assign98300_e150629 * (locals.var_arg_dn9 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign98300_e150629) as f64).is_finite() && ((assign98300_e150629) as f64).fract() == 0.0 { if assign98300_e150629 == 0.0 { 0.0 } else { (assign98300_e150629 * ((locals.var_arg).powf(assign98300_e150629 - 1.0) * locals.var_arg_dn10)) } } else { (assign98300_e150630 * (assign98300_e150629 * (locals.var_arg_dn10 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign98300_e150629) as f64).is_finite() && ((assign98300_e150629) as f64).fract() == 0.0 { if assign98300_e150629 == 0.0 { 0.0 } else { (assign98300_e150629 * ((locals.var_arg).powf(assign98300_e150629 - 1.0) * locals.var_arg_dn13)) } } else { (assign98300_e150630 * (assign98300_e150629 * (locals.var_arg_dn13 / locals.var_arg))) },)
            }
        };
        (assign98300_e150631, assign98300_e150631_d_n0, assign98300_e150631_d_n2, assign98300_e150631_d_n4, assign98300_e150631_d_n5, assign98300_e150631_d_n6, assign98300_e150631_d_n7, assign98300_e150631_d_n8, assign98300_e150631_d_n9, assign98300_e150631_d_n10, assign98300_e150631_d_n13,)
    } else {
        (locals.var_sarg, locals.var_sarg_dn0, locals.var_sarg_dn2, locals.var_sarg_dn4, locals.var_sarg_dn5, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn8, locals.var_sarg_dn9, locals.var_sarg_dn10, locals.var_sarg_dn13,)
    }
};
        locals.var_sarg = assign98300_e150633;
        locals.var_sarg_dn0 = assign98300_e150633_d_n0;
        locals.var_sarg_dn2 = assign98300_e150633_d_n2;
        locals.var_sarg_dn4 = assign98300_e150633_d_n4;
        locals.var_sarg_dn5 = assign98300_e150633_d_n5;
        locals.var_sarg_dn6 = assign98300_e150633_d_n6;
        locals.var_sarg_dn7 = assign98300_e150633_d_n7;
        locals.var_sarg_dn8 = assign98300_e150633_d_n8;
        locals.var_sarg_dn9 = assign98300_e150633_d_n9;
        locals.var_sarg_dn10 = assign98300_e150633_d_n10;
        locals.var_sarg_dn13 = assign98300_e150633_d_n13;
        locals.var_sarg_rv = 0.0;

        let (assign98310_e150651, assign98310_e150651_d_n0, assign98310_e150651_d_n2, assign98310_e150651_d_n4, assign98310_e150651_d_n5, assign98310_e150651_d_n6, assign98310_e150651_d_n7, assign98310_e150651_d_n8, assign98310_e150651_d_n9, assign98310_e150651_d_n10, assign98310_e150651_d_n13,) = {
    if ((locals.var_guard2274 != 0.0) && (locals.var_guard2275 != 0.0)) {
        let assign98310_e150639: f64 = (locals.var_pzbd * locals.var_czbd);
        let assign98310_e150643: f64 = (locals.var_arg * locals.var_sarg);
        let assign98310_e150644: f64 = (1.0 - assign98310_e150643);
        let assign98310_e150645: f64 = (assign98310_e150639 * assign98310_e150644);
        let assign98310_e150648: f64 = (1.0 - p.p503);
        let assign98310_e150649: f64 = (assign98310_e150645 / assign98310_e150648);
        (assign98310_e150649, (((((locals.var_pzbd_dn0 * locals.var_czbd) + (locals.var_pzbd * locals.var_czbd_dn0)) * assign98310_e150644) + (assign98310_e150639 * (-((locals.var_arg_dn0 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn0))))) / assign98310_e150648), (((((locals.var_pzbd_dn2 * locals.var_czbd) + (locals.var_pzbd * locals.var_czbd_dn2)) * assign98310_e150644) + (assign98310_e150639 * (-((locals.var_arg_dn2 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn2))))) / assign98310_e150648), (((((locals.var_pzbd_dn4 * locals.var_czbd) + (locals.var_pzbd * locals.var_czbd_dn4)) * assign98310_e150644) + (assign98310_e150639 * (-((locals.var_arg_dn4 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn4))))) / assign98310_e150648), (((((locals.var_pzbd_dn5 * locals.var_czbd) + (locals.var_pzbd * locals.var_czbd_dn5)) * assign98310_e150644) + (assign98310_e150639 * (-((locals.var_arg_dn5 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn5))))) / assign98310_e150648), (((((locals.var_pzbd_dn6 * locals.var_czbd) + (locals.var_pzbd * locals.var_czbd_dn6)) * assign98310_e150644) + (assign98310_e150639 * (-((locals.var_arg_dn6 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn6))))) / assign98310_e150648), (((((locals.var_pzbd_dn7 * locals.var_czbd) + (locals.var_pzbd * locals.var_czbd_dn7)) * assign98310_e150644) + (assign98310_e150639 * (-((locals.var_arg_dn7 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn7))))) / assign98310_e150648), (((((locals.var_pzbd_dn8 * locals.var_czbd) + (locals.var_pzbd * locals.var_czbd_dn8)) * assign98310_e150644) + (assign98310_e150639 * (-((locals.var_arg_dn8 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn8))))) / assign98310_e150648), (((((locals.var_pzbd_dn9 * locals.var_czbd) + (locals.var_pzbd * locals.var_czbd_dn9)) * assign98310_e150644) + (assign98310_e150639 * (-((locals.var_arg_dn9 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn9))))) / assign98310_e150648), (((((locals.var_pzbd_dn10 * locals.var_czbd) + (locals.var_pzbd * locals.var_czbd_dn10)) * assign98310_e150644) + (assign98310_e150639 * (-((locals.var_arg_dn10 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn10))))) / assign98310_e150648), (((((locals.var_pzbd_dn13 * locals.var_czbd) + (locals.var_pzbd * locals.var_czbd_dn13)) * assign98310_e150644) + (assign98310_e150639 * (-((locals.var_arg_dn13 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn13))))) / assign98310_e150648),)
    } else {
        (locals.var_qbd_btm, locals.var_qbd_btm_dn0, locals.var_qbd_btm_dn2, locals.var_qbd_btm_dn4, locals.var_qbd_btm_dn5, locals.var_qbd_btm_dn6, locals.var_qbd_btm_dn7, locals.var_qbd_btm_dn8, locals.var_qbd_btm_dn9, locals.var_qbd_btm_dn10, locals.var_qbd_btm_dn13,)
    }
};
        locals.var_qbd_btm = assign98310_e150651;
        locals.var_qbd_btm_dn0 = assign98310_e150651_d_n0;
        locals.var_qbd_btm_dn2 = assign98310_e150651_d_n2;
        locals.var_qbd_btm_dn4 = assign98310_e150651_d_n4;
        locals.var_qbd_btm_dn5 = assign98310_e150651_d_n5;
        locals.var_qbd_btm_dn6 = assign98310_e150651_d_n6;
        locals.var_qbd_btm_dn7 = assign98310_e150651_d_n7;
        locals.var_qbd_btm_dn8 = assign98310_e150651_d_n8;
        locals.var_qbd_btm_dn9 = assign98310_e150651_d_n9;
        locals.var_qbd_btm_dn10 = assign98310_e150651_d_n10;
        locals.var_qbd_btm_dn13 = assign98310_e150651_d_n13;
        locals.var_qbd_btm_rv = 0.0;

        let (assign98330_e150666, assign98330_e150666_d_n0, assign98330_e150666_d_n2, assign98330_e150666_d_n4, assign98330_e150666_d_n5, assign98330_e150666_d_n6, assign98330_e150666_d_n7, assign98330_e150666_d_n8, assign98330_e150666_d_n9, assign98330_e150666_d_n10, assign98330_e150666_d_n13,) = {
    if ((locals.var_guard2274 != 0.0) && (locals.var_guard2275 == 0.0)) {
        (locals.var_czbd, locals.var_czbd_dn0, locals.var_czbd_dn2, locals.var_czbd_dn4, locals.var_czbd_dn5, locals.var_czbd_dn6, locals.var_czbd_dn7, locals.var_czbd_dn8, locals.var_czbd_dn9, locals.var_czbd_dn10, locals.var_czbd_dn13,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign98330_e150666;
        locals.var_t1_dn0 = assign98330_e150666_d_n0;
        locals.var_t1_dn2 = assign98330_e150666_d_n2;
        locals.var_t1_dn4 = assign98330_e150666_d_n4;
        locals.var_t1_dn5 = assign98330_e150666_d_n5;
        locals.var_t1_dn6 = assign98330_e150666_d_n6;
        locals.var_t1_dn7 = assign98330_e150666_d_n7;
        locals.var_t1_dn8 = assign98330_e150666_d_n8;
        locals.var_t1_dn9 = assign98330_e150666_d_n9;
        locals.var_t1_dn10 = assign98330_e150666_d_n10;
        locals.var_t1_dn13 = assign98330_e150666_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign98340_e150677, assign98340_e150677_d_n0, assign98340_e150677_d_n2, assign98340_e150677_d_n4, assign98340_e150677_d_n5, assign98340_e150677_d_n6, assign98340_e150677_d_n7, assign98340_e150677_d_n8, assign98340_e150677_d_n9, assign98340_e150677_d_n10, assign98340_e150677_d_n13,) = {
    if ((locals.var_guard2274 != 0.0) && (locals.var_guard2275 == 0.0)) {
        let assign98340_e150673: f64 = (locals.var_czbd * p.p503);
        let assign98340_e150675: f64 = (assign98340_e150673 / locals.var_pzbd);
        (assign98340_e150675, ((((locals.var_czbd_dn0 * p.p503) * locals.var_pzbd) - (assign98340_e150673 * locals.var_pzbd_dn0)) / (locals.var_pzbd * locals.var_pzbd)), ((((locals.var_czbd_dn2 * p.p503) * locals.var_pzbd) - (assign98340_e150673 * locals.var_pzbd_dn2)) / (locals.var_pzbd * locals.var_pzbd)), ((((locals.var_czbd_dn4 * p.p503) * locals.var_pzbd) - (assign98340_e150673 * locals.var_pzbd_dn4)) / (locals.var_pzbd * locals.var_pzbd)), ((((locals.var_czbd_dn5 * p.p503) * locals.var_pzbd) - (assign98340_e150673 * locals.var_pzbd_dn5)) / (locals.var_pzbd * locals.var_pzbd)), ((((locals.var_czbd_dn6 * p.p503) * locals.var_pzbd) - (assign98340_e150673 * locals.var_pzbd_dn6)) / (locals.var_pzbd * locals.var_pzbd)), ((((locals.var_czbd_dn7 * p.p503) * locals.var_pzbd) - (assign98340_e150673 * locals.var_pzbd_dn7)) / (locals.var_pzbd * locals.var_pzbd)), ((((locals.var_czbd_dn8 * p.p503) * locals.var_pzbd) - (assign98340_e150673 * locals.var_pzbd_dn8)) / (locals.var_pzbd * locals.var_pzbd)), ((((locals.var_czbd_dn9 * p.p503) * locals.var_pzbd) - (assign98340_e150673 * locals.var_pzbd_dn9)) / (locals.var_pzbd * locals.var_pzbd)), ((((locals.var_czbd_dn10 * p.p503) * locals.var_pzbd) - (assign98340_e150673 * locals.var_pzbd_dn10)) / (locals.var_pzbd * locals.var_pzbd)), ((((locals.var_czbd_dn13 * p.p503) * locals.var_pzbd) - (assign98340_e150673 * locals.var_pzbd_dn13)) / (locals.var_pzbd * locals.var_pzbd)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign98340_e150677;
        locals.var_t2_dn0 = assign98340_e150677_d_n0;
        locals.var_t2_dn2 = assign98340_e150677_d_n2;
        locals.var_t2_dn4 = assign98340_e150677_d_n4;
        locals.var_t2_dn5 = assign98340_e150677_d_n5;
        locals.var_t2_dn6 = assign98340_e150677_d_n6;
        locals.var_t2_dn7 = assign98340_e150677_d_n7;
        locals.var_t2_dn8 = assign98340_e150677_d_n8;
        locals.var_t2_dn9 = assign98340_e150677_d_n9;
        locals.var_t2_dn10 = assign98340_e150677_d_n10;
        locals.var_t2_dn13 = assign98340_e150677_d_n13;
        locals.var_t2_rv = 0.0;

        let (assign98350_e150692, assign98350_e150692_d_n0, assign98350_e150692_d_n2, assign98350_e150692_d_n4, assign98350_e150692_d_n5, assign98350_e150692_d_n6, assign98350_e150692_d_n7, assign98350_e150692_d_n8, assign98350_e150692_d_n9, assign98350_e150692_d_n10, assign98350_e150692_d_n13,) = {
    if ((locals.var_guard2274 != 0.0) && (locals.var_guard2275 == 0.0)) {
        let assign98350_e150686: f64 = (locals.var_vbd_jct * 0.5);
        let assign98350_e150688: f64 = (assign98350_e150686 * locals.var_t2);
        let assign98350_e150689: f64 = (locals.var_t1 + assign98350_e150688);
        let assign98350_e150690: f64 = (locals.var_vbd_jct * assign98350_e150689);
        (assign98350_e150690, ((locals.var_vbd_jct_dn0 * assign98350_e150689) + (locals.var_vbd_jct * (locals.var_t1_dn0 + (((locals.var_vbd_jct_dn0 * 0.5) * locals.var_t2) + (assign98350_e150686 * locals.var_t2_dn0))))), (locals.var_vbd_jct * (locals.var_t1_dn2 + (assign98350_e150686 * locals.var_t2_dn2))), (locals.var_vbd_jct * (locals.var_t1_dn4 + (assign98350_e150686 * locals.var_t2_dn4))), (locals.var_vbd_jct * (locals.var_t1_dn5 + (assign98350_e150686 * locals.var_t2_dn5))), (locals.var_vbd_jct * (locals.var_t1_dn6 + (assign98350_e150686 * locals.var_t2_dn6))), (locals.var_vbd_jct * (locals.var_t1_dn7 + (assign98350_e150686 * locals.var_t2_dn7))), (locals.var_vbd_jct * (locals.var_t1_dn8 + (assign98350_e150686 * locals.var_t2_dn8))), ((locals.var_vbd_jct_dn9 * assign98350_e150689) + (locals.var_vbd_jct * (locals.var_t1_dn9 + (((locals.var_vbd_jct_dn9 * 0.5) * locals.var_t2) + (assign98350_e150686 * locals.var_t2_dn9))))), (locals.var_vbd_jct * (locals.var_t1_dn10 + (assign98350_e150686 * locals.var_t2_dn10))), (locals.var_vbd_jct * (locals.var_t1_dn13 + (assign98350_e150686 * locals.var_t2_dn13))),)
    } else {
        (locals.var_qbd_btm, locals.var_qbd_btm_dn0, locals.var_qbd_btm_dn2, locals.var_qbd_btm_dn4, locals.var_qbd_btm_dn5, locals.var_qbd_btm_dn6, locals.var_qbd_btm_dn7, locals.var_qbd_btm_dn8, locals.var_qbd_btm_dn9, locals.var_qbd_btm_dn10, locals.var_qbd_btm_dn13,)
    }
};
        locals.var_qbd_btm = assign98350_e150692;
        locals.var_qbd_btm_dn0 = assign98350_e150692_d_n0;
        locals.var_qbd_btm_dn2 = assign98350_e150692_d_n2;
        locals.var_qbd_btm_dn4 = assign98350_e150692_d_n4;
        locals.var_qbd_btm_dn5 = assign98350_e150692_d_n5;
        locals.var_qbd_btm_dn6 = assign98350_e150692_d_n6;
        locals.var_qbd_btm_dn7 = assign98350_e150692_d_n7;
        locals.var_qbd_btm_dn8 = assign98350_e150692_d_n8;
        locals.var_qbd_btm_dn9 = assign98350_e150692_d_n9;
        locals.var_qbd_btm_dn10 = assign98350_e150692_d_n10;
        locals.var_qbd_btm_dn13 = assign98350_e150692_d_n13;
        locals.var_qbd_btm_rv = 0.0;

        let (assign98370_e150708, assign98370_e150708_d_n0, assign98370_e150708_d_n2, assign98370_e150708_d_n4, assign98370_e150708_d_n5, assign98370_e150708_d_n6, assign98370_e150708_d_n7, assign98370_e150708_d_n8, assign98370_e150708_d_n9, assign98370_e150708_d_n10, assign98370_e150708_d_n13,) = {
    if (locals.var_guard2274 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qbd_btm, locals.var_qbd_btm_dn0, locals.var_qbd_btm_dn2, locals.var_qbd_btm_dn4, locals.var_qbd_btm_dn5, locals.var_qbd_btm_dn6, locals.var_qbd_btm_dn7, locals.var_qbd_btm_dn8, locals.var_qbd_btm_dn9, locals.var_qbd_btm_dn10, locals.var_qbd_btm_dn13,)
    }
};
        locals.var_qbd_btm = assign98370_e150708;
        locals.var_qbd_btm_dn0 = assign98370_e150708_d_n0;
        locals.var_qbd_btm_dn2 = assign98370_e150708_d_n2;
        locals.var_qbd_btm_dn4 = assign98370_e150708_d_n4;
        locals.var_qbd_btm_dn5 = assign98370_e150708_d_n5;
        locals.var_qbd_btm_dn6 = assign98370_e150708_d_n6;
        locals.var_qbd_btm_dn7 = assign98370_e150708_d_n7;
        locals.var_qbd_btm_dn8 = assign98370_e150708_d_n8;
        locals.var_qbd_btm_dn9 = assign98370_e150708_d_n9;
        locals.var_qbd_btm_dn10 = assign98370_e150708_d_n10;
        locals.var_qbd_btm_dn13 = assign98370_e150708_d_n13;
        locals.var_qbd_btm_rv = 0.0;

        let assign98390_e150716: f64 = if locals.var_czbdsw > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2277 = assign98390_e150716;
        locals.var_guard2277_rv = 0.0;

        let assign98400_e150719: f64 = if locals.var_vbd_jct < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2278 = assign98400_e150719;
        locals.var_guard2278_rv = 0.0;

        let (assign98410_e150729, assign98410_e150729_d_n0, assign98410_e150729_d_n2, assign98410_e150729_d_n4, assign98410_e150729_d_n5, assign98410_e150729_d_n6, assign98410_e150729_d_n7, assign98410_e150729_d_n8, assign98410_e150729_d_n9, assign98410_e150729_d_n10, assign98410_e150729_d_n13,) = {
    if ((locals.var_guard2277 != 0.0) && (locals.var_guard2278 != 0.0)) {
        let assign98410_e150726: f64 = (locals.var_vbd_jct / locals.var_pzbdsw);
        let assign98410_e150727: f64 = (1.0 - assign98410_e150726);
        (assign98410_e150727, (-(((locals.var_vbd_jct_dn0 * locals.var_pzbdsw) - (locals.var_vbd_jct * locals.var_pzbdsw_dn0)) / (locals.var_pzbdsw * locals.var_pzbdsw))), (-(-((locals.var_vbd_jct * locals.var_pzbdsw_dn2) / (locals.var_pzbdsw * locals.var_pzbdsw)))), (-(-((locals.var_vbd_jct * locals.var_pzbdsw_dn4) / (locals.var_pzbdsw * locals.var_pzbdsw)))), (-(-((locals.var_vbd_jct * locals.var_pzbdsw_dn5) / (locals.var_pzbdsw * locals.var_pzbdsw)))), (-(-((locals.var_vbd_jct * locals.var_pzbdsw_dn6) / (locals.var_pzbdsw * locals.var_pzbdsw)))), (-(-((locals.var_vbd_jct * locals.var_pzbdsw_dn7) / (locals.var_pzbdsw * locals.var_pzbdsw)))), (-(-((locals.var_vbd_jct * locals.var_pzbdsw_dn8) / (locals.var_pzbdsw * locals.var_pzbdsw)))), (-(((locals.var_vbd_jct_dn9 * locals.var_pzbdsw) - (locals.var_vbd_jct * locals.var_pzbdsw_dn9)) / (locals.var_pzbdsw * locals.var_pzbdsw))), (-(-((locals.var_vbd_jct * locals.var_pzbdsw_dn10) / (locals.var_pzbdsw * locals.var_pzbdsw)))), (-(-((locals.var_vbd_jct * locals.var_pzbdsw_dn13) / (locals.var_pzbdsw * locals.var_pzbdsw)))),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    }
};
        locals.var_arg = assign98410_e150729;
        locals.var_arg_dn0 = assign98410_e150729_d_n0;
        locals.var_arg_dn2 = assign98410_e150729_d_n2;
        locals.var_arg_dn4 = assign98410_e150729_d_n4;
        locals.var_arg_dn5 = assign98410_e150729_d_n5;
        locals.var_arg_dn6 = assign98410_e150729_d_n6;
        locals.var_arg_dn7 = assign98410_e150729_d_n7;
        locals.var_arg_dn8 = assign98410_e150729_d_n8;
        locals.var_arg_dn9 = assign98410_e150729_d_n9;
        locals.var_arg_dn10 = assign98410_e150729_d_n10;
        locals.var_arg_dn13 = assign98410_e150729_d_n13;
        locals.var_arg_rv = 0.0;

        let assign98420_e150732: f64 = if p.p504 == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard2279 = assign98420_e150732;
        locals.var_guard2279_rv = 0.0;

        let (assign98430_e150743, assign98430_e150743_d_n0, assign98430_e150743_d_n2, assign98430_e150743_d_n4, assign98430_e150743_d_n5, assign98430_e150743_d_n6, assign98430_e150743_d_n7, assign98430_e150743_d_n8, assign98430_e150743_d_n9, assign98430_e150743_d_n10, assign98430_e150743_d_n13,) = {
    if (((locals.var_guard2277 != 0.0) && (locals.var_guard2278 != 0.0)) && (locals.var_guard2279 != 0.0)) {
        let assign98430_e150740: f64 = (locals.var_arg).sqrt();
        let assign98430_e150741: f64 = (1.0 / assign98430_e150740);
        (assign98430_e150741, (-((locals.var_arg_dn0 / (2.0 * assign98430_e150740)) / (assign98430_e150740 * assign98430_e150740))), (-((locals.var_arg_dn2 / (2.0 * assign98430_e150740)) / (assign98430_e150740 * assign98430_e150740))), (-((locals.var_arg_dn4 / (2.0 * assign98430_e150740)) / (assign98430_e150740 * assign98430_e150740))), (-((locals.var_arg_dn5 / (2.0 * assign98430_e150740)) / (assign98430_e150740 * assign98430_e150740))), (-((locals.var_arg_dn6 / (2.0 * assign98430_e150740)) / (assign98430_e150740 * assign98430_e150740))), (-((locals.var_arg_dn7 / (2.0 * assign98430_e150740)) / (assign98430_e150740 * assign98430_e150740))), (-((locals.var_arg_dn8 / (2.0 * assign98430_e150740)) / (assign98430_e150740 * assign98430_e150740))), (-((locals.var_arg_dn9 / (2.0 * assign98430_e150740)) / (assign98430_e150740 * assign98430_e150740))), (-((locals.var_arg_dn10 / (2.0 * assign98430_e150740)) / (assign98430_e150740 * assign98430_e150740))), (-((locals.var_arg_dn13 / (2.0 * assign98430_e150740)) / (assign98430_e150740 * assign98430_e150740))),)
    } else {
        (locals.var_sarg, locals.var_sarg_dn0, locals.var_sarg_dn2, locals.var_sarg_dn4, locals.var_sarg_dn5, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn8, locals.var_sarg_dn9, locals.var_sarg_dn10, locals.var_sarg_dn13,)
    }
};
        locals.var_sarg = assign98430_e150743;
        locals.var_sarg_dn0 = assign98430_e150743_d_n0;
        locals.var_sarg_dn2 = assign98430_e150743_d_n2;
        locals.var_sarg_dn4 = assign98430_e150743_d_n4;
        locals.var_sarg_dn5 = assign98430_e150743_d_n5;
        locals.var_sarg_dn6 = assign98430_e150743_d_n6;
        locals.var_sarg_dn7 = assign98430_e150743_d_n7;
        locals.var_sarg_dn8 = assign98430_e150743_d_n8;
        locals.var_sarg_dn9 = assign98430_e150743_d_n9;
        locals.var_sarg_dn10 = assign98430_e150743_d_n10;
        locals.var_sarg_dn13 = assign98430_e150743_d_n13;
        locals.var_sarg_rv = 0.0;

        let (assign98440_e150760, assign98440_e150760_d_n0, assign98440_e150760_d_n2, assign98440_e150760_d_n4, assign98440_e150760_d_n5, assign98440_e150760_d_n6, assign98440_e150760_d_n7, assign98440_e150760_d_n8, assign98440_e150760_d_n9, assign98440_e150760_d_n10, assign98440_e150760_d_n13,) = {
    if (((locals.var_guard2277 != 0.0) && (locals.var_guard2278 != 0.0)) && (locals.var_guard2279 == 0.0)) {
        let (assign98440_e150758, assign98440_e150758_d_n0, assign98440_e150758_d_n2, assign98440_e150758_d_n4, assign98440_e150758_d_n5, assign98440_e150758_d_n6, assign98440_e150758_d_n7, assign98440_e150758_d_n8, assign98440_e150758_d_n9, assign98440_e150758_d_n10, assign98440_e150758_d_n13,) = {
            if (locals.var_arg == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign98440_e150756: f64 = (-p.p504);
                let assign98440_e150757: f64 = (locals.var_arg).powf(assign98440_e150756);
                (assign98440_e150757, if 0.0 == 0.0 && ((assign98440_e150756) as f64).is_finite() && ((assign98440_e150756) as f64).fract() == 0.0 { if assign98440_e150756 == 0.0 { 0.0 } else { (assign98440_e150756 * ((locals.var_arg).powf(assign98440_e150756 - 1.0) * locals.var_arg_dn0)) } } else { (assign98440_e150757 * (assign98440_e150756 * (locals.var_arg_dn0 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign98440_e150756) as f64).is_finite() && ((assign98440_e150756) as f64).fract() == 0.0 { if assign98440_e150756 == 0.0 { 0.0 } else { (assign98440_e150756 * ((locals.var_arg).powf(assign98440_e150756 - 1.0) * locals.var_arg_dn2)) } } else { (assign98440_e150757 * (assign98440_e150756 * (locals.var_arg_dn2 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign98440_e150756) as f64).is_finite() && ((assign98440_e150756) as f64).fract() == 0.0 { if assign98440_e150756 == 0.0 { 0.0 } else { (assign98440_e150756 * ((locals.var_arg).powf(assign98440_e150756 - 1.0) * locals.var_arg_dn4)) } } else { (assign98440_e150757 * (assign98440_e150756 * (locals.var_arg_dn4 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign98440_e150756) as f64).is_finite() && ((assign98440_e150756) as f64).fract() == 0.0 { if assign98440_e150756 == 0.0 { 0.0 } else { (assign98440_e150756 * ((locals.var_arg).powf(assign98440_e150756 - 1.0) * locals.var_arg_dn5)) } } else { (assign98440_e150757 * (assign98440_e150756 * (locals.var_arg_dn5 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign98440_e150756) as f64).is_finite() && ((assign98440_e150756) as f64).fract() == 0.0 { if assign98440_e150756 == 0.0 { 0.0 } else { (assign98440_e150756 * ((locals.var_arg).powf(assign98440_e150756 - 1.0) * locals.var_arg_dn6)) } } else { (assign98440_e150757 * (assign98440_e150756 * (locals.var_arg_dn6 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign98440_e150756) as f64).is_finite() && ((assign98440_e150756) as f64).fract() == 0.0 { if assign98440_e150756 == 0.0 { 0.0 } else { (assign98440_e150756 * ((locals.var_arg).powf(assign98440_e150756 - 1.0) * locals.var_arg_dn7)) } } else { (assign98440_e150757 * (assign98440_e150756 * (locals.var_arg_dn7 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign98440_e150756) as f64).is_finite() && ((assign98440_e150756) as f64).fract() == 0.0 { if assign98440_e150756 == 0.0 { 0.0 } else { (assign98440_e150756 * ((locals.var_arg).powf(assign98440_e150756 - 1.0) * locals.var_arg_dn8)) } } else { (assign98440_e150757 * (assign98440_e150756 * (locals.var_arg_dn8 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign98440_e150756) as f64).is_finite() && ((assign98440_e150756) as f64).fract() == 0.0 { if assign98440_e150756 == 0.0 { 0.0 } else { (assign98440_e150756 * ((locals.var_arg).powf(assign98440_e150756 - 1.0) * locals.var_arg_dn9)) } } else { (assign98440_e150757 * (assign98440_e150756 * (locals.var_arg_dn9 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign98440_e150756) as f64).is_finite() && ((assign98440_e150756) as f64).fract() == 0.0 { if assign98440_e150756 == 0.0 { 0.0 } else { (assign98440_e150756 * ((locals.var_arg).powf(assign98440_e150756 - 1.0) * locals.var_arg_dn10)) } } else { (assign98440_e150757 * (assign98440_e150756 * (locals.var_arg_dn10 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign98440_e150756) as f64).is_finite() && ((assign98440_e150756) as f64).fract() == 0.0 { if assign98440_e150756 == 0.0 { 0.0 } else { (assign98440_e150756 * ((locals.var_arg).powf(assign98440_e150756 - 1.0) * locals.var_arg_dn13)) } } else { (assign98440_e150757 * (assign98440_e150756 * (locals.var_arg_dn13 / locals.var_arg))) },)
            }
        };
        (assign98440_e150758, assign98440_e150758_d_n0, assign98440_e150758_d_n2, assign98440_e150758_d_n4, assign98440_e150758_d_n5, assign98440_e150758_d_n6, assign98440_e150758_d_n7, assign98440_e150758_d_n8, assign98440_e150758_d_n9, assign98440_e150758_d_n10, assign98440_e150758_d_n13,)
    } else {
        (locals.var_sarg, locals.var_sarg_dn0, locals.var_sarg_dn2, locals.var_sarg_dn4, locals.var_sarg_dn5, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn8, locals.var_sarg_dn9, locals.var_sarg_dn10, locals.var_sarg_dn13,)
    }
};
        locals.var_sarg = assign98440_e150760;
        locals.var_sarg_dn0 = assign98440_e150760_d_n0;
        locals.var_sarg_dn2 = assign98440_e150760_d_n2;
        locals.var_sarg_dn4 = assign98440_e150760_d_n4;
        locals.var_sarg_dn5 = assign98440_e150760_d_n5;
        locals.var_sarg_dn6 = assign98440_e150760_d_n6;
        locals.var_sarg_dn7 = assign98440_e150760_d_n7;
        locals.var_sarg_dn8 = assign98440_e150760_d_n8;
        locals.var_sarg_dn9 = assign98440_e150760_d_n9;
        locals.var_sarg_dn10 = assign98440_e150760_d_n10;
        locals.var_sarg_dn13 = assign98440_e150760_d_n13;
        locals.var_sarg_rv = 0.0;

        let (assign98450_e150778, assign98450_e150778_d_n0, assign98450_e150778_d_n2, assign98450_e150778_d_n4, assign98450_e150778_d_n5, assign98450_e150778_d_n6, assign98450_e150778_d_n7, assign98450_e150778_d_n8, assign98450_e150778_d_n9, assign98450_e150778_d_n10, assign98450_e150778_d_n13,) = {
    if ((locals.var_guard2277 != 0.0) && (locals.var_guard2278 != 0.0)) {
        let assign98450_e150766: f64 = (locals.var_pzbdsw * locals.var_czbdsw);
        let assign98450_e150770: f64 = (locals.var_arg * locals.var_sarg);
        let assign98450_e150771: f64 = (1.0 - assign98450_e150770);
        let assign98450_e150772: f64 = (assign98450_e150766 * assign98450_e150771);
        let assign98450_e150775: f64 = (1.0 - p.p504);
        let assign98450_e150776: f64 = (assign98450_e150772 / assign98450_e150775);
        (assign98450_e150776, (((((locals.var_pzbdsw_dn0 * locals.var_czbdsw) + (locals.var_pzbdsw * locals.var_czbdsw_dn0)) * assign98450_e150771) + (assign98450_e150766 * (-((locals.var_arg_dn0 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn0))))) / assign98450_e150775), (((((locals.var_pzbdsw_dn2 * locals.var_czbdsw) + (locals.var_pzbdsw * locals.var_czbdsw_dn2)) * assign98450_e150771) + (assign98450_e150766 * (-((locals.var_arg_dn2 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn2))))) / assign98450_e150775), (((((locals.var_pzbdsw_dn4 * locals.var_czbdsw) + (locals.var_pzbdsw * locals.var_czbdsw_dn4)) * assign98450_e150771) + (assign98450_e150766 * (-((locals.var_arg_dn4 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn4))))) / assign98450_e150775), (((((locals.var_pzbdsw_dn5 * locals.var_czbdsw) + (locals.var_pzbdsw * locals.var_czbdsw_dn5)) * assign98450_e150771) + (assign98450_e150766 * (-((locals.var_arg_dn5 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn5))))) / assign98450_e150775), (((((locals.var_pzbdsw_dn6 * locals.var_czbdsw) + (locals.var_pzbdsw * locals.var_czbdsw_dn6)) * assign98450_e150771) + (assign98450_e150766 * (-((locals.var_arg_dn6 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn6))))) / assign98450_e150775), (((((locals.var_pzbdsw_dn7 * locals.var_czbdsw) + (locals.var_pzbdsw * locals.var_czbdsw_dn7)) * assign98450_e150771) + (assign98450_e150766 * (-((locals.var_arg_dn7 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn7))))) / assign98450_e150775), (((((locals.var_pzbdsw_dn8 * locals.var_czbdsw) + (locals.var_pzbdsw * locals.var_czbdsw_dn8)) * assign98450_e150771) + (assign98450_e150766 * (-((locals.var_arg_dn8 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn8))))) / assign98450_e150775), (((((locals.var_pzbdsw_dn9 * locals.var_czbdsw) + (locals.var_pzbdsw * locals.var_czbdsw_dn9)) * assign98450_e150771) + (assign98450_e150766 * (-((locals.var_arg_dn9 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn9))))) / assign98450_e150775), (((((locals.var_pzbdsw_dn10 * locals.var_czbdsw) + (locals.var_pzbdsw * locals.var_czbdsw_dn10)) * assign98450_e150771) + (assign98450_e150766 * (-((locals.var_arg_dn10 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn10))))) / assign98450_e150775), (((((locals.var_pzbdsw_dn13 * locals.var_czbdsw) + (locals.var_pzbdsw * locals.var_czbdsw_dn13)) * assign98450_e150771) + (assign98450_e150766 * (-((locals.var_arg_dn13 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn13))))) / assign98450_e150775),)
    } else {
        (locals.var_qbd_sws, locals.var_qbd_sws_dn0, locals.var_qbd_sws_dn2, locals.var_qbd_sws_dn4, locals.var_qbd_sws_dn5, locals.var_qbd_sws_dn6, locals.var_qbd_sws_dn7, locals.var_qbd_sws_dn8, locals.var_qbd_sws_dn9, locals.var_qbd_sws_dn10, locals.var_qbd_sws_dn13,)
    }
};
        locals.var_qbd_sws = assign98450_e150778;
        locals.var_qbd_sws_dn0 = assign98450_e150778_d_n0;
        locals.var_qbd_sws_dn2 = assign98450_e150778_d_n2;
        locals.var_qbd_sws_dn4 = assign98450_e150778_d_n4;
        locals.var_qbd_sws_dn5 = assign98450_e150778_d_n5;
        locals.var_qbd_sws_dn6 = assign98450_e150778_d_n6;
        locals.var_qbd_sws_dn7 = assign98450_e150778_d_n7;
        locals.var_qbd_sws_dn8 = assign98450_e150778_d_n8;
        locals.var_qbd_sws_dn9 = assign98450_e150778_d_n9;
        locals.var_qbd_sws_dn10 = assign98450_e150778_d_n10;
        locals.var_qbd_sws_dn13 = assign98450_e150778_d_n13;
        locals.var_qbd_sws_rv = 0.0;

        let (assign98470_e150793, assign98470_e150793_d_n0, assign98470_e150793_d_n2, assign98470_e150793_d_n4, assign98470_e150793_d_n5, assign98470_e150793_d_n6, assign98470_e150793_d_n7, assign98470_e150793_d_n8, assign98470_e150793_d_n9, assign98470_e150793_d_n10, assign98470_e150793_d_n13,) = {
    if ((locals.var_guard2277 != 0.0) && (locals.var_guard2278 == 0.0)) {
        (locals.var_czbdsw, locals.var_czbdsw_dn0, locals.var_czbdsw_dn2, locals.var_czbdsw_dn4, locals.var_czbdsw_dn5, locals.var_czbdsw_dn6, locals.var_czbdsw_dn7, locals.var_czbdsw_dn8, locals.var_czbdsw_dn9, locals.var_czbdsw_dn10, locals.var_czbdsw_dn13,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign98470_e150793;
        locals.var_t1_dn0 = assign98470_e150793_d_n0;
        locals.var_t1_dn2 = assign98470_e150793_d_n2;
        locals.var_t1_dn4 = assign98470_e150793_d_n4;
        locals.var_t1_dn5 = assign98470_e150793_d_n5;
        locals.var_t1_dn6 = assign98470_e150793_d_n6;
        locals.var_t1_dn7 = assign98470_e150793_d_n7;
        locals.var_t1_dn8 = assign98470_e150793_d_n8;
        locals.var_t1_dn9 = assign98470_e150793_d_n9;
        locals.var_t1_dn10 = assign98470_e150793_d_n10;
        locals.var_t1_dn13 = assign98470_e150793_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign98480_e150804, assign98480_e150804_d_n0, assign98480_e150804_d_n2, assign98480_e150804_d_n4, assign98480_e150804_d_n5, assign98480_e150804_d_n6, assign98480_e150804_d_n7, assign98480_e150804_d_n8, assign98480_e150804_d_n9, assign98480_e150804_d_n10, assign98480_e150804_d_n13,) = {
    if ((locals.var_guard2277 != 0.0) && (locals.var_guard2278 == 0.0)) {
        let assign98480_e150800: f64 = (locals.var_czbdsw * p.p504);
        let assign98480_e150802: f64 = (assign98480_e150800 / locals.var_pzbdsw);
        (assign98480_e150802, ((((locals.var_czbdsw_dn0 * p.p504) * locals.var_pzbdsw) - (assign98480_e150800 * locals.var_pzbdsw_dn0)) / (locals.var_pzbdsw * locals.var_pzbdsw)), ((((locals.var_czbdsw_dn2 * p.p504) * locals.var_pzbdsw) - (assign98480_e150800 * locals.var_pzbdsw_dn2)) / (locals.var_pzbdsw * locals.var_pzbdsw)), ((((locals.var_czbdsw_dn4 * p.p504) * locals.var_pzbdsw) - (assign98480_e150800 * locals.var_pzbdsw_dn4)) / (locals.var_pzbdsw * locals.var_pzbdsw)), ((((locals.var_czbdsw_dn5 * p.p504) * locals.var_pzbdsw) - (assign98480_e150800 * locals.var_pzbdsw_dn5)) / (locals.var_pzbdsw * locals.var_pzbdsw)), ((((locals.var_czbdsw_dn6 * p.p504) * locals.var_pzbdsw) - (assign98480_e150800 * locals.var_pzbdsw_dn6)) / (locals.var_pzbdsw * locals.var_pzbdsw)), ((((locals.var_czbdsw_dn7 * p.p504) * locals.var_pzbdsw) - (assign98480_e150800 * locals.var_pzbdsw_dn7)) / (locals.var_pzbdsw * locals.var_pzbdsw)), ((((locals.var_czbdsw_dn8 * p.p504) * locals.var_pzbdsw) - (assign98480_e150800 * locals.var_pzbdsw_dn8)) / (locals.var_pzbdsw * locals.var_pzbdsw)), ((((locals.var_czbdsw_dn9 * p.p504) * locals.var_pzbdsw) - (assign98480_e150800 * locals.var_pzbdsw_dn9)) / (locals.var_pzbdsw * locals.var_pzbdsw)), ((((locals.var_czbdsw_dn10 * p.p504) * locals.var_pzbdsw) - (assign98480_e150800 * locals.var_pzbdsw_dn10)) / (locals.var_pzbdsw * locals.var_pzbdsw)), ((((locals.var_czbdsw_dn13 * p.p504) * locals.var_pzbdsw) - (assign98480_e150800 * locals.var_pzbdsw_dn13)) / (locals.var_pzbdsw * locals.var_pzbdsw)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign98480_e150804;
        locals.var_t2_dn0 = assign98480_e150804_d_n0;
        locals.var_t2_dn2 = assign98480_e150804_d_n2;
        locals.var_t2_dn4 = assign98480_e150804_d_n4;
        locals.var_t2_dn5 = assign98480_e150804_d_n5;
        locals.var_t2_dn6 = assign98480_e150804_d_n6;
        locals.var_t2_dn7 = assign98480_e150804_d_n7;
        locals.var_t2_dn8 = assign98480_e150804_d_n8;
        locals.var_t2_dn9 = assign98480_e150804_d_n9;
        locals.var_t2_dn10 = assign98480_e150804_d_n10;
        locals.var_t2_dn13 = assign98480_e150804_d_n13;
        locals.var_t2_rv = 0.0;

        let (assign98490_e150819, assign98490_e150819_d_n0, assign98490_e150819_d_n2, assign98490_e150819_d_n4, assign98490_e150819_d_n5, assign98490_e150819_d_n6, assign98490_e150819_d_n7, assign98490_e150819_d_n8, assign98490_e150819_d_n9, assign98490_e150819_d_n10, assign98490_e150819_d_n13,) = {
    if ((locals.var_guard2277 != 0.0) && (locals.var_guard2278 == 0.0)) {
        let assign98490_e150813: f64 = (locals.var_vbd_jct * 0.5);
        let assign98490_e150815: f64 = (assign98490_e150813 * locals.var_t2);
        let assign98490_e150816: f64 = (locals.var_t1 + assign98490_e150815);
        let assign98490_e150817: f64 = (locals.var_vbd_jct * assign98490_e150816);
        (assign98490_e150817, ((locals.var_vbd_jct_dn0 * assign98490_e150816) + (locals.var_vbd_jct * (locals.var_t1_dn0 + (((locals.var_vbd_jct_dn0 * 0.5) * locals.var_t2) + (assign98490_e150813 * locals.var_t2_dn0))))), (locals.var_vbd_jct * (locals.var_t1_dn2 + (assign98490_e150813 * locals.var_t2_dn2))), (locals.var_vbd_jct * (locals.var_t1_dn4 + (assign98490_e150813 * locals.var_t2_dn4))), (locals.var_vbd_jct * (locals.var_t1_dn5 + (assign98490_e150813 * locals.var_t2_dn5))), (locals.var_vbd_jct * (locals.var_t1_dn6 + (assign98490_e150813 * locals.var_t2_dn6))), (locals.var_vbd_jct * (locals.var_t1_dn7 + (assign98490_e150813 * locals.var_t2_dn7))), (locals.var_vbd_jct * (locals.var_t1_dn8 + (assign98490_e150813 * locals.var_t2_dn8))), ((locals.var_vbd_jct_dn9 * assign98490_e150816) + (locals.var_vbd_jct * (locals.var_t1_dn9 + (((locals.var_vbd_jct_dn9 * 0.5) * locals.var_t2) + (assign98490_e150813 * locals.var_t2_dn9))))), (locals.var_vbd_jct * (locals.var_t1_dn10 + (assign98490_e150813 * locals.var_t2_dn10))), (locals.var_vbd_jct * (locals.var_t1_dn13 + (assign98490_e150813 * locals.var_t2_dn13))),)
    } else {
        (locals.var_qbd_sws, locals.var_qbd_sws_dn0, locals.var_qbd_sws_dn2, locals.var_qbd_sws_dn4, locals.var_qbd_sws_dn5, locals.var_qbd_sws_dn6, locals.var_qbd_sws_dn7, locals.var_qbd_sws_dn8, locals.var_qbd_sws_dn9, locals.var_qbd_sws_dn10, locals.var_qbd_sws_dn13,)
    }
};
        locals.var_qbd_sws = assign98490_e150819;
        locals.var_qbd_sws_dn0 = assign98490_e150819_d_n0;
        locals.var_qbd_sws_dn2 = assign98490_e150819_d_n2;
        locals.var_qbd_sws_dn4 = assign98490_e150819_d_n4;
        locals.var_qbd_sws_dn5 = assign98490_e150819_d_n5;
        locals.var_qbd_sws_dn6 = assign98490_e150819_d_n6;
        locals.var_qbd_sws_dn7 = assign98490_e150819_d_n7;
        locals.var_qbd_sws_dn8 = assign98490_e150819_d_n8;
        locals.var_qbd_sws_dn9 = assign98490_e150819_d_n9;
        locals.var_qbd_sws_dn10 = assign98490_e150819_d_n10;
        locals.var_qbd_sws_dn13 = assign98490_e150819_d_n13;
        locals.var_qbd_sws_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_368(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign98510_e150835, assign98510_e150835_d_n0, assign98510_e150835_d_n2, assign98510_e150835_d_n4, assign98510_e150835_d_n5, assign98510_e150835_d_n6, assign98510_e150835_d_n7, assign98510_e150835_d_n8, assign98510_e150835_d_n9, assign98510_e150835_d_n10, assign98510_e150835_d_n13,) = {
    if (locals.var_guard2277 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qbd_sws, locals.var_qbd_sws_dn0, locals.var_qbd_sws_dn2, locals.var_qbd_sws_dn4, locals.var_qbd_sws_dn5, locals.var_qbd_sws_dn6, locals.var_qbd_sws_dn7, locals.var_qbd_sws_dn8, locals.var_qbd_sws_dn9, locals.var_qbd_sws_dn10, locals.var_qbd_sws_dn13,)
    }
};
        locals.var_qbd_sws = assign98510_e150835;
        locals.var_qbd_sws_dn0 = assign98510_e150835_d_n0;
        locals.var_qbd_sws_dn2 = assign98510_e150835_d_n2;
        locals.var_qbd_sws_dn4 = assign98510_e150835_d_n4;
        locals.var_qbd_sws_dn5 = assign98510_e150835_d_n5;
        locals.var_qbd_sws_dn6 = assign98510_e150835_d_n6;
        locals.var_qbd_sws_dn7 = assign98510_e150835_d_n7;
        locals.var_qbd_sws_dn8 = assign98510_e150835_d_n8;
        locals.var_qbd_sws_dn9 = assign98510_e150835_d_n9;
        locals.var_qbd_sws_dn10 = assign98510_e150835_d_n10;
        locals.var_qbd_sws_dn13 = assign98510_e150835_d_n13;
        locals.var_qbd_sws_rv = 0.0;

        let assign98530_e150843: f64 = if p.p48 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2280 = assign98530_e150843;
        locals.var_guard2280_rv = 0.0;

        let assign98540_e150846: f64 = if locals.var_czbdswg > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2281 = assign98540_e150846;
        locals.var_guard2281_rv = 0.0;

        let assign98550_e150849: f64 = if locals.var_vbdi_jct < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2282 = assign98550_e150849;
        locals.var_guard2282_rv = 0.0;

        let (assign98560_e150861, assign98560_e150861_d_n0, assign98560_e150861_d_n2, assign98560_e150861_d_n4, assign98560_e150861_d_n5, assign98560_e150861_d_n6, assign98560_e150861_d_n7, assign98560_e150861_d_n8, assign98560_e150861_d_n9, assign98560_e150861_d_n10, assign98560_e150861_d_n13,) = {
    if (((locals.var_guard2280 != 0.0) && (locals.var_guard2281 != 0.0)) && (locals.var_guard2282 != 0.0)) {
        let assign98560_e150858: f64 = (locals.var_vbdi_jct / locals.var_pzbdswg);
        let assign98560_e150859: f64 = (1.0 - assign98560_e150858);
        (assign98560_e150859, (-(-((locals.var_vbdi_jct * locals.var_pzbdswg_dn0) / (locals.var_pzbdswg * locals.var_pzbdswg)))), (-(-((locals.var_vbdi_jct * locals.var_pzbdswg_dn2) / (locals.var_pzbdswg * locals.var_pzbdswg)))), (-(-((locals.var_vbdi_jct * locals.var_pzbdswg_dn4) / (locals.var_pzbdswg * locals.var_pzbdswg)))), (-(((locals.var_vbdi_jct_dn5 * locals.var_pzbdswg) - (locals.var_vbdi_jct * locals.var_pzbdswg_dn5)) / (locals.var_pzbdswg * locals.var_pzbdswg))), (-(-((locals.var_vbdi_jct * locals.var_pzbdswg_dn6) / (locals.var_pzbdswg * locals.var_pzbdswg)))), (-(-((locals.var_vbdi_jct * locals.var_pzbdswg_dn7) / (locals.var_pzbdswg * locals.var_pzbdswg)))), (-(((locals.var_vbdi_jct_dn8 * locals.var_pzbdswg) - (locals.var_vbdi_jct * locals.var_pzbdswg_dn8)) / (locals.var_pzbdswg * locals.var_pzbdswg))), (-(-((locals.var_vbdi_jct * locals.var_pzbdswg_dn9) / (locals.var_pzbdswg * locals.var_pzbdswg)))), (-(-((locals.var_vbdi_jct * locals.var_pzbdswg_dn10) / (locals.var_pzbdswg * locals.var_pzbdswg)))), (-(-((locals.var_vbdi_jct * locals.var_pzbdswg_dn13) / (locals.var_pzbdswg * locals.var_pzbdswg)))),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    }
};
        locals.var_arg = assign98560_e150861;
        locals.var_arg_dn0 = assign98560_e150861_d_n0;
        locals.var_arg_dn2 = assign98560_e150861_d_n2;
        locals.var_arg_dn4 = assign98560_e150861_d_n4;
        locals.var_arg_dn5 = assign98560_e150861_d_n5;
        locals.var_arg_dn6 = assign98560_e150861_d_n6;
        locals.var_arg_dn7 = assign98560_e150861_d_n7;
        locals.var_arg_dn8 = assign98560_e150861_d_n8;
        locals.var_arg_dn9 = assign98560_e150861_d_n9;
        locals.var_arg_dn10 = assign98560_e150861_d_n10;
        locals.var_arg_dn13 = assign98560_e150861_d_n13;
        locals.var_arg_rv = 0.0;

        let assign98570_e150864: f64 = if p.p505 == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard2283 = assign98570_e150864;
        locals.var_guard2283_rv = 0.0;

        let (assign98580_e150877, assign98580_e150877_d_n0, assign98580_e150877_d_n2, assign98580_e150877_d_n4, assign98580_e150877_d_n5, assign98580_e150877_d_n6, assign98580_e150877_d_n7, assign98580_e150877_d_n8, assign98580_e150877_d_n9, assign98580_e150877_d_n10, assign98580_e150877_d_n13,) = {
    if ((((locals.var_guard2280 != 0.0) && (locals.var_guard2281 != 0.0)) && (locals.var_guard2282 != 0.0)) && (locals.var_guard2283 != 0.0)) {
        let assign98580_e150874: f64 = (locals.var_arg).sqrt();
        let assign98580_e150875: f64 = (1.0 / assign98580_e150874);
        (assign98580_e150875, (-((locals.var_arg_dn0 / (2.0 * assign98580_e150874)) / (assign98580_e150874 * assign98580_e150874))), (-((locals.var_arg_dn2 / (2.0 * assign98580_e150874)) / (assign98580_e150874 * assign98580_e150874))), (-((locals.var_arg_dn4 / (2.0 * assign98580_e150874)) / (assign98580_e150874 * assign98580_e150874))), (-((locals.var_arg_dn5 / (2.0 * assign98580_e150874)) / (assign98580_e150874 * assign98580_e150874))), (-((locals.var_arg_dn6 / (2.0 * assign98580_e150874)) / (assign98580_e150874 * assign98580_e150874))), (-((locals.var_arg_dn7 / (2.0 * assign98580_e150874)) / (assign98580_e150874 * assign98580_e150874))), (-((locals.var_arg_dn8 / (2.0 * assign98580_e150874)) / (assign98580_e150874 * assign98580_e150874))), (-((locals.var_arg_dn9 / (2.0 * assign98580_e150874)) / (assign98580_e150874 * assign98580_e150874))), (-((locals.var_arg_dn10 / (2.0 * assign98580_e150874)) / (assign98580_e150874 * assign98580_e150874))), (-((locals.var_arg_dn13 / (2.0 * assign98580_e150874)) / (assign98580_e150874 * assign98580_e150874))),)
    } else {
        (locals.var_sarg, locals.var_sarg_dn0, locals.var_sarg_dn2, locals.var_sarg_dn4, locals.var_sarg_dn5, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn8, locals.var_sarg_dn9, locals.var_sarg_dn10, locals.var_sarg_dn13,)
    }
};
        locals.var_sarg = assign98580_e150877;
        locals.var_sarg_dn0 = assign98580_e150877_d_n0;
        locals.var_sarg_dn2 = assign98580_e150877_d_n2;
        locals.var_sarg_dn4 = assign98580_e150877_d_n4;
        locals.var_sarg_dn5 = assign98580_e150877_d_n5;
        locals.var_sarg_dn6 = assign98580_e150877_d_n6;
        locals.var_sarg_dn7 = assign98580_e150877_d_n7;
        locals.var_sarg_dn8 = assign98580_e150877_d_n8;
        locals.var_sarg_dn9 = assign98580_e150877_d_n9;
        locals.var_sarg_dn10 = assign98580_e150877_d_n10;
        locals.var_sarg_dn13 = assign98580_e150877_d_n13;
        locals.var_sarg_rv = 0.0;

        let (assign98590_e150896, assign98590_e150896_d_n0, assign98590_e150896_d_n2, assign98590_e150896_d_n4, assign98590_e150896_d_n5, assign98590_e150896_d_n6, assign98590_e150896_d_n7, assign98590_e150896_d_n8, assign98590_e150896_d_n9, assign98590_e150896_d_n10, assign98590_e150896_d_n13,) = {
    if ((((locals.var_guard2280 != 0.0) && (locals.var_guard2281 != 0.0)) && (locals.var_guard2282 != 0.0)) && (locals.var_guard2283 == 0.0)) {
        let (assign98590_e150894, assign98590_e150894_d_n0, assign98590_e150894_d_n2, assign98590_e150894_d_n4, assign98590_e150894_d_n5, assign98590_e150894_d_n6, assign98590_e150894_d_n7, assign98590_e150894_d_n8, assign98590_e150894_d_n9, assign98590_e150894_d_n10, assign98590_e150894_d_n13,) = {
            if (locals.var_arg == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign98590_e150892: f64 = (-p.p505);
                let assign98590_e150893: f64 = (locals.var_arg).powf(assign98590_e150892);
                (assign98590_e150893, if 0.0 == 0.0 && ((assign98590_e150892) as f64).is_finite() && ((assign98590_e150892) as f64).fract() == 0.0 { if assign98590_e150892 == 0.0 { 0.0 } else { (assign98590_e150892 * ((locals.var_arg).powf(assign98590_e150892 - 1.0) * locals.var_arg_dn0)) } } else { (assign98590_e150893 * (assign98590_e150892 * (locals.var_arg_dn0 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign98590_e150892) as f64).is_finite() && ((assign98590_e150892) as f64).fract() == 0.0 { if assign98590_e150892 == 0.0 { 0.0 } else { (assign98590_e150892 * ((locals.var_arg).powf(assign98590_e150892 - 1.0) * locals.var_arg_dn2)) } } else { (assign98590_e150893 * (assign98590_e150892 * (locals.var_arg_dn2 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign98590_e150892) as f64).is_finite() && ((assign98590_e150892) as f64).fract() == 0.0 { if assign98590_e150892 == 0.0 { 0.0 } else { (assign98590_e150892 * ((locals.var_arg).powf(assign98590_e150892 - 1.0) * locals.var_arg_dn4)) } } else { (assign98590_e150893 * (assign98590_e150892 * (locals.var_arg_dn4 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign98590_e150892) as f64).is_finite() && ((assign98590_e150892) as f64).fract() == 0.0 { if assign98590_e150892 == 0.0 { 0.0 } else { (assign98590_e150892 * ((locals.var_arg).powf(assign98590_e150892 - 1.0) * locals.var_arg_dn5)) } } else { (assign98590_e150893 * (assign98590_e150892 * (locals.var_arg_dn5 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign98590_e150892) as f64).is_finite() && ((assign98590_e150892) as f64).fract() == 0.0 { if assign98590_e150892 == 0.0 { 0.0 } else { (assign98590_e150892 * ((locals.var_arg).powf(assign98590_e150892 - 1.0) * locals.var_arg_dn6)) } } else { (assign98590_e150893 * (assign98590_e150892 * (locals.var_arg_dn6 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign98590_e150892) as f64).is_finite() && ((assign98590_e150892) as f64).fract() == 0.0 { if assign98590_e150892 == 0.0 { 0.0 } else { (assign98590_e150892 * ((locals.var_arg).powf(assign98590_e150892 - 1.0) * locals.var_arg_dn7)) } } else { (assign98590_e150893 * (assign98590_e150892 * (locals.var_arg_dn7 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign98590_e150892) as f64).is_finite() && ((assign98590_e150892) as f64).fract() == 0.0 { if assign98590_e150892 == 0.0 { 0.0 } else { (assign98590_e150892 * ((locals.var_arg).powf(assign98590_e150892 - 1.0) * locals.var_arg_dn8)) } } else { (assign98590_e150893 * (assign98590_e150892 * (locals.var_arg_dn8 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign98590_e150892) as f64).is_finite() && ((assign98590_e150892) as f64).fract() == 0.0 { if assign98590_e150892 == 0.0 { 0.0 } else { (assign98590_e150892 * ((locals.var_arg).powf(assign98590_e150892 - 1.0) * locals.var_arg_dn9)) } } else { (assign98590_e150893 * (assign98590_e150892 * (locals.var_arg_dn9 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign98590_e150892) as f64).is_finite() && ((assign98590_e150892) as f64).fract() == 0.0 { if assign98590_e150892 == 0.0 { 0.0 } else { (assign98590_e150892 * ((locals.var_arg).powf(assign98590_e150892 - 1.0) * locals.var_arg_dn10)) } } else { (assign98590_e150893 * (assign98590_e150892 * (locals.var_arg_dn10 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign98590_e150892) as f64).is_finite() && ((assign98590_e150892) as f64).fract() == 0.0 { if assign98590_e150892 == 0.0 { 0.0 } else { (assign98590_e150892 * ((locals.var_arg).powf(assign98590_e150892 - 1.0) * locals.var_arg_dn13)) } } else { (assign98590_e150893 * (assign98590_e150892 * (locals.var_arg_dn13 / locals.var_arg))) },)
            }
        };
        (assign98590_e150894, assign98590_e150894_d_n0, assign98590_e150894_d_n2, assign98590_e150894_d_n4, assign98590_e150894_d_n5, assign98590_e150894_d_n6, assign98590_e150894_d_n7, assign98590_e150894_d_n8, assign98590_e150894_d_n9, assign98590_e150894_d_n10, assign98590_e150894_d_n13,)
    } else {
        (locals.var_sarg, locals.var_sarg_dn0, locals.var_sarg_dn2, locals.var_sarg_dn4, locals.var_sarg_dn5, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn8, locals.var_sarg_dn9, locals.var_sarg_dn10, locals.var_sarg_dn13,)
    }
};
        locals.var_sarg = assign98590_e150896;
        locals.var_sarg_dn0 = assign98590_e150896_d_n0;
        locals.var_sarg_dn2 = assign98590_e150896_d_n2;
        locals.var_sarg_dn4 = assign98590_e150896_d_n4;
        locals.var_sarg_dn5 = assign98590_e150896_d_n5;
        locals.var_sarg_dn6 = assign98590_e150896_d_n6;
        locals.var_sarg_dn7 = assign98590_e150896_d_n7;
        locals.var_sarg_dn8 = assign98590_e150896_d_n8;
        locals.var_sarg_dn9 = assign98590_e150896_d_n9;
        locals.var_sarg_dn10 = assign98590_e150896_d_n10;
        locals.var_sarg_dn13 = assign98590_e150896_d_n13;
        locals.var_sarg_rv = 0.0;

        let (assign98600_e150916, assign98600_e150916_d_n0, assign98600_e150916_d_n2, assign98600_e150916_d_n4, assign98600_e150916_d_n5, assign98600_e150916_d_n6, assign98600_e150916_d_n7, assign98600_e150916_d_n8, assign98600_e150916_d_n9, assign98600_e150916_d_n10, assign98600_e150916_d_n13,) = {
    if (((locals.var_guard2280 != 0.0) && (locals.var_guard2281 != 0.0)) && (locals.var_guard2282 != 0.0)) {
        let assign98600_e150904: f64 = (locals.var_pzbdswg * locals.var_czbdswg);
        let assign98600_e150908: f64 = (locals.var_arg * locals.var_sarg);
        let assign98600_e150909: f64 = (1.0 - assign98600_e150908);
        let assign98600_e150910: f64 = (assign98600_e150904 * assign98600_e150909);
        let assign98600_e150913: f64 = (1.0 - p.p505);
        let assign98600_e150914: f64 = (assign98600_e150910 / assign98600_e150913);
        (assign98600_e150914, (((((locals.var_pzbdswg_dn0 * locals.var_czbdswg) + (locals.var_pzbdswg * locals.var_czbdswg_dn0)) * assign98600_e150909) + (assign98600_e150904 * (-((locals.var_arg_dn0 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn0))))) / assign98600_e150913), (((((locals.var_pzbdswg_dn2 * locals.var_czbdswg) + (locals.var_pzbdswg * locals.var_czbdswg_dn2)) * assign98600_e150909) + (assign98600_e150904 * (-((locals.var_arg_dn2 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn2))))) / assign98600_e150913), (((((locals.var_pzbdswg_dn4 * locals.var_czbdswg) + (locals.var_pzbdswg * locals.var_czbdswg_dn4)) * assign98600_e150909) + (assign98600_e150904 * (-((locals.var_arg_dn4 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn4))))) / assign98600_e150913), (((((locals.var_pzbdswg_dn5 * locals.var_czbdswg) + (locals.var_pzbdswg * locals.var_czbdswg_dn5)) * assign98600_e150909) + (assign98600_e150904 * (-((locals.var_arg_dn5 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn5))))) / assign98600_e150913), (((((locals.var_pzbdswg_dn6 * locals.var_czbdswg) + (locals.var_pzbdswg * locals.var_czbdswg_dn6)) * assign98600_e150909) + (assign98600_e150904 * (-((locals.var_arg_dn6 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn6))))) / assign98600_e150913), (((((locals.var_pzbdswg_dn7 * locals.var_czbdswg) + (locals.var_pzbdswg * locals.var_czbdswg_dn7)) * assign98600_e150909) + (assign98600_e150904 * (-((locals.var_arg_dn7 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn7))))) / assign98600_e150913), (((((locals.var_pzbdswg_dn8 * locals.var_czbdswg) + (locals.var_pzbdswg * locals.var_czbdswg_dn8)) * assign98600_e150909) + (assign98600_e150904 * (-((locals.var_arg_dn8 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn8))))) / assign98600_e150913), (((((locals.var_pzbdswg_dn9 * locals.var_czbdswg) + (locals.var_pzbdswg * locals.var_czbdswg_dn9)) * assign98600_e150909) + (assign98600_e150904 * (-((locals.var_arg_dn9 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn9))))) / assign98600_e150913), (((((locals.var_pzbdswg_dn10 * locals.var_czbdswg) + (locals.var_pzbdswg * locals.var_czbdswg_dn10)) * assign98600_e150909) + (assign98600_e150904 * (-((locals.var_arg_dn10 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn10))))) / assign98600_e150913), (((((locals.var_pzbdswg_dn13 * locals.var_czbdswg) + (locals.var_pzbdswg * locals.var_czbdswg_dn13)) * assign98600_e150909) + (assign98600_e150904 * (-((locals.var_arg_dn13 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn13))))) / assign98600_e150913),)
    } else {
        (locals.var_qbd_swg, locals.var_qbd_swg_dn0, locals.var_qbd_swg_dn2, locals.var_qbd_swg_dn4, locals.var_qbd_swg_dn5, locals.var_qbd_swg_dn6, locals.var_qbd_swg_dn7, locals.var_qbd_swg_dn8, locals.var_qbd_swg_dn9, locals.var_qbd_swg_dn10, locals.var_qbd_swg_dn13,)
    }
};
        locals.var_qbd_swg = assign98600_e150916;
        locals.var_qbd_swg_dn0 = assign98600_e150916_d_n0;
        locals.var_qbd_swg_dn2 = assign98600_e150916_d_n2;
        locals.var_qbd_swg_dn4 = assign98600_e150916_d_n4;
        locals.var_qbd_swg_dn5 = assign98600_e150916_d_n5;
        locals.var_qbd_swg_dn6 = assign98600_e150916_d_n6;
        locals.var_qbd_swg_dn7 = assign98600_e150916_d_n7;
        locals.var_qbd_swg_dn8 = assign98600_e150916_d_n8;
        locals.var_qbd_swg_dn9 = assign98600_e150916_d_n9;
        locals.var_qbd_swg_dn10 = assign98600_e150916_d_n10;
        locals.var_qbd_swg_dn13 = assign98600_e150916_d_n13;
        locals.var_qbd_swg_rv = 0.0;

        let (assign98620_e150935, assign98620_e150935_d_n0, assign98620_e150935_d_n2, assign98620_e150935_d_n4, assign98620_e150935_d_n5, assign98620_e150935_d_n6, assign98620_e150935_d_n7, assign98620_e150935_d_n8, assign98620_e150935_d_n9, assign98620_e150935_d_n10, assign98620_e150935_d_n13,) = {
    if (((locals.var_guard2280 != 0.0) && (locals.var_guard2281 != 0.0)) && (locals.var_guard2282 == 0.0)) {
        (locals.var_czbdswg, locals.var_czbdswg_dn0, locals.var_czbdswg_dn2, locals.var_czbdswg_dn4, locals.var_czbdswg_dn5, locals.var_czbdswg_dn6, locals.var_czbdswg_dn7, locals.var_czbdswg_dn8, locals.var_czbdswg_dn9, locals.var_czbdswg_dn10, locals.var_czbdswg_dn13,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign98620_e150935;
        locals.var_t1_dn0 = assign98620_e150935_d_n0;
        locals.var_t1_dn2 = assign98620_e150935_d_n2;
        locals.var_t1_dn4 = assign98620_e150935_d_n4;
        locals.var_t1_dn5 = assign98620_e150935_d_n5;
        locals.var_t1_dn6 = assign98620_e150935_d_n6;
        locals.var_t1_dn7 = assign98620_e150935_d_n7;
        locals.var_t1_dn8 = assign98620_e150935_d_n8;
        locals.var_t1_dn9 = assign98620_e150935_d_n9;
        locals.var_t1_dn10 = assign98620_e150935_d_n10;
        locals.var_t1_dn13 = assign98620_e150935_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign98630_e150948, assign98630_e150948_d_n0, assign98630_e150948_d_n2, assign98630_e150948_d_n4, assign98630_e150948_d_n5, assign98630_e150948_d_n6, assign98630_e150948_d_n7, assign98630_e150948_d_n8, assign98630_e150948_d_n9, assign98630_e150948_d_n10, assign98630_e150948_d_n13,) = {
    if (((locals.var_guard2280 != 0.0) && (locals.var_guard2281 != 0.0)) && (locals.var_guard2282 == 0.0)) {
        let assign98630_e150944: f64 = (locals.var_czbdswg * p.p505);
        let assign98630_e150946: f64 = (assign98630_e150944 / locals.var_pzbdswg);
        (assign98630_e150946, ((((locals.var_czbdswg_dn0 * p.p505) * locals.var_pzbdswg) - (assign98630_e150944 * locals.var_pzbdswg_dn0)) / (locals.var_pzbdswg * locals.var_pzbdswg)), ((((locals.var_czbdswg_dn2 * p.p505) * locals.var_pzbdswg) - (assign98630_e150944 * locals.var_pzbdswg_dn2)) / (locals.var_pzbdswg * locals.var_pzbdswg)), ((((locals.var_czbdswg_dn4 * p.p505) * locals.var_pzbdswg) - (assign98630_e150944 * locals.var_pzbdswg_dn4)) / (locals.var_pzbdswg * locals.var_pzbdswg)), ((((locals.var_czbdswg_dn5 * p.p505) * locals.var_pzbdswg) - (assign98630_e150944 * locals.var_pzbdswg_dn5)) / (locals.var_pzbdswg * locals.var_pzbdswg)), ((((locals.var_czbdswg_dn6 * p.p505) * locals.var_pzbdswg) - (assign98630_e150944 * locals.var_pzbdswg_dn6)) / (locals.var_pzbdswg * locals.var_pzbdswg)), ((((locals.var_czbdswg_dn7 * p.p505) * locals.var_pzbdswg) - (assign98630_e150944 * locals.var_pzbdswg_dn7)) / (locals.var_pzbdswg * locals.var_pzbdswg)), ((((locals.var_czbdswg_dn8 * p.p505) * locals.var_pzbdswg) - (assign98630_e150944 * locals.var_pzbdswg_dn8)) / (locals.var_pzbdswg * locals.var_pzbdswg)), ((((locals.var_czbdswg_dn9 * p.p505) * locals.var_pzbdswg) - (assign98630_e150944 * locals.var_pzbdswg_dn9)) / (locals.var_pzbdswg * locals.var_pzbdswg)), ((((locals.var_czbdswg_dn10 * p.p505) * locals.var_pzbdswg) - (assign98630_e150944 * locals.var_pzbdswg_dn10)) / (locals.var_pzbdswg * locals.var_pzbdswg)), ((((locals.var_czbdswg_dn13 * p.p505) * locals.var_pzbdswg) - (assign98630_e150944 * locals.var_pzbdswg_dn13)) / (locals.var_pzbdswg * locals.var_pzbdswg)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign98630_e150948;
        locals.var_t2_dn0 = assign98630_e150948_d_n0;
        locals.var_t2_dn2 = assign98630_e150948_d_n2;
        locals.var_t2_dn4 = assign98630_e150948_d_n4;
        locals.var_t2_dn5 = assign98630_e150948_d_n5;
        locals.var_t2_dn6 = assign98630_e150948_d_n6;
        locals.var_t2_dn7 = assign98630_e150948_d_n7;
        locals.var_t2_dn8 = assign98630_e150948_d_n8;
        locals.var_t2_dn9 = assign98630_e150948_d_n9;
        locals.var_t2_dn10 = assign98630_e150948_d_n10;
        locals.var_t2_dn13 = assign98630_e150948_d_n13;
        locals.var_t2_rv = 0.0;

        let (assign98640_e150965, assign98640_e150965_d_n0, assign98640_e150965_d_n2, assign98640_e150965_d_n4, assign98640_e150965_d_n5, assign98640_e150965_d_n6, assign98640_e150965_d_n7, assign98640_e150965_d_n8, assign98640_e150965_d_n9, assign98640_e150965_d_n10, assign98640_e150965_d_n13,) = {
    if (((locals.var_guard2280 != 0.0) && (locals.var_guard2281 != 0.0)) && (locals.var_guard2282 == 0.0)) {
        let assign98640_e150959: f64 = (locals.var_vbdi_jct * 0.5);
        let assign98640_e150961: f64 = (assign98640_e150959 * locals.var_t2);
        let assign98640_e150962: f64 = (locals.var_t1 + assign98640_e150961);
        let assign98640_e150963: f64 = (locals.var_vbdi_jct * assign98640_e150962);
        (assign98640_e150963, (locals.var_vbdi_jct * (locals.var_t1_dn0 + (assign98640_e150959 * locals.var_t2_dn0))), (locals.var_vbdi_jct * (locals.var_t1_dn2 + (assign98640_e150959 * locals.var_t2_dn2))), (locals.var_vbdi_jct * (locals.var_t1_dn4 + (assign98640_e150959 * locals.var_t2_dn4))), ((locals.var_vbdi_jct_dn5 * assign98640_e150962) + (locals.var_vbdi_jct * (locals.var_t1_dn5 + (((locals.var_vbdi_jct_dn5 * 0.5) * locals.var_t2) + (assign98640_e150959 * locals.var_t2_dn5))))), (locals.var_vbdi_jct * (locals.var_t1_dn6 + (assign98640_e150959 * locals.var_t2_dn6))), (locals.var_vbdi_jct * (locals.var_t1_dn7 + (assign98640_e150959 * locals.var_t2_dn7))), ((locals.var_vbdi_jct_dn8 * assign98640_e150962) + (locals.var_vbdi_jct * (locals.var_t1_dn8 + (((locals.var_vbdi_jct_dn8 * 0.5) * locals.var_t2) + (assign98640_e150959 * locals.var_t2_dn8))))), (locals.var_vbdi_jct * (locals.var_t1_dn9 + (assign98640_e150959 * locals.var_t2_dn9))), (locals.var_vbdi_jct * (locals.var_t1_dn10 + (assign98640_e150959 * locals.var_t2_dn10))), (locals.var_vbdi_jct * (locals.var_t1_dn13 + (assign98640_e150959 * locals.var_t2_dn13))),)
    } else {
        (locals.var_qbd_swg, locals.var_qbd_swg_dn0, locals.var_qbd_swg_dn2, locals.var_qbd_swg_dn4, locals.var_qbd_swg_dn5, locals.var_qbd_swg_dn6, locals.var_qbd_swg_dn7, locals.var_qbd_swg_dn8, locals.var_qbd_swg_dn9, locals.var_qbd_swg_dn10, locals.var_qbd_swg_dn13,)
    }
};
        locals.var_qbd_swg = assign98640_e150965;
        locals.var_qbd_swg_dn0 = assign98640_e150965_d_n0;
        locals.var_qbd_swg_dn2 = assign98640_e150965_d_n2;
        locals.var_qbd_swg_dn4 = assign98640_e150965_d_n4;
        locals.var_qbd_swg_dn5 = assign98640_e150965_d_n5;
        locals.var_qbd_swg_dn6 = assign98640_e150965_d_n6;
        locals.var_qbd_swg_dn7 = assign98640_e150965_d_n7;
        locals.var_qbd_swg_dn8 = assign98640_e150965_d_n8;
        locals.var_qbd_swg_dn9 = assign98640_e150965_d_n9;
        locals.var_qbd_swg_dn10 = assign98640_e150965_d_n10;
        locals.var_qbd_swg_dn13 = assign98640_e150965_d_n13;
        locals.var_qbd_swg_rv = 0.0;

        let (assign98660_e150985, assign98660_e150985_d_n0, assign98660_e150985_d_n2, assign98660_e150985_d_n4, assign98660_e150985_d_n5, assign98660_e150985_d_n6, assign98660_e150985_d_n7, assign98660_e150985_d_n8, assign98660_e150985_d_n9, assign98660_e150985_d_n10, assign98660_e150985_d_n13,) = {
    if ((locals.var_guard2280 != 0.0) && (locals.var_guard2281 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qbd_swg, locals.var_qbd_swg_dn0, locals.var_qbd_swg_dn2, locals.var_qbd_swg_dn4, locals.var_qbd_swg_dn5, locals.var_qbd_swg_dn6, locals.var_qbd_swg_dn7, locals.var_qbd_swg_dn8, locals.var_qbd_swg_dn9, locals.var_qbd_swg_dn10, locals.var_qbd_swg_dn13,)
    }
};
        locals.var_qbd_swg = assign98660_e150985;
        locals.var_qbd_swg_dn0 = assign98660_e150985_d_n0;
        locals.var_qbd_swg_dn2 = assign98660_e150985_d_n2;
        locals.var_qbd_swg_dn4 = assign98660_e150985_d_n4;
        locals.var_qbd_swg_dn5 = assign98660_e150985_d_n5;
        locals.var_qbd_swg_dn6 = assign98660_e150985_d_n6;
        locals.var_qbd_swg_dn7 = assign98660_e150985_d_n7;
        locals.var_qbd_swg_dn8 = assign98660_e150985_d_n8;
        locals.var_qbd_swg_dn9 = assign98660_e150985_d_n9;
        locals.var_qbd_swg_dn10 = assign98660_e150985_d_n10;
        locals.var_qbd_swg_dn13 = assign98660_e150985_d_n13;
        locals.var_qbd_swg_rv = 0.0;

        let assign98680_e150995: f64 = if locals.var_czbdswg > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2284 = assign98680_e150995;
        locals.var_guard2284_rv = 0.0;

        let assign98690_e150998: f64 = if locals.var_vbd_jct < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2285 = assign98690_e150998;
        locals.var_guard2285_rv = 0.0;

        let (assign98700_e151011, assign98700_e151011_d_n0, assign98700_e151011_d_n2, assign98700_e151011_d_n4, assign98700_e151011_d_n5, assign98700_e151011_d_n6, assign98700_e151011_d_n7, assign98700_e151011_d_n8, assign98700_e151011_d_n9, assign98700_e151011_d_n10, assign98700_e151011_d_n13,) = {
    if (((locals.var_guard2280 == 0.0) && (locals.var_guard2284 != 0.0)) && (locals.var_guard2285 != 0.0)) {
        let assign98700_e151008: f64 = (locals.var_vbd_jct / locals.var_pzbdswg);
        let assign98700_e151009: f64 = (1.0 - assign98700_e151008);
        (assign98700_e151009, (-(((locals.var_vbd_jct_dn0 * locals.var_pzbdswg) - (locals.var_vbd_jct * locals.var_pzbdswg_dn0)) / (locals.var_pzbdswg * locals.var_pzbdswg))), (-(-((locals.var_vbd_jct * locals.var_pzbdswg_dn2) / (locals.var_pzbdswg * locals.var_pzbdswg)))), (-(-((locals.var_vbd_jct * locals.var_pzbdswg_dn4) / (locals.var_pzbdswg * locals.var_pzbdswg)))), (-(-((locals.var_vbd_jct * locals.var_pzbdswg_dn5) / (locals.var_pzbdswg * locals.var_pzbdswg)))), (-(-((locals.var_vbd_jct * locals.var_pzbdswg_dn6) / (locals.var_pzbdswg * locals.var_pzbdswg)))), (-(-((locals.var_vbd_jct * locals.var_pzbdswg_dn7) / (locals.var_pzbdswg * locals.var_pzbdswg)))), (-(-((locals.var_vbd_jct * locals.var_pzbdswg_dn8) / (locals.var_pzbdswg * locals.var_pzbdswg)))), (-(((locals.var_vbd_jct_dn9 * locals.var_pzbdswg) - (locals.var_vbd_jct * locals.var_pzbdswg_dn9)) / (locals.var_pzbdswg * locals.var_pzbdswg))), (-(-((locals.var_vbd_jct * locals.var_pzbdswg_dn10) / (locals.var_pzbdswg * locals.var_pzbdswg)))), (-(-((locals.var_vbd_jct * locals.var_pzbdswg_dn13) / (locals.var_pzbdswg * locals.var_pzbdswg)))),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    }
};
        locals.var_arg = assign98700_e151011;
        locals.var_arg_dn0 = assign98700_e151011_d_n0;
        locals.var_arg_dn2 = assign98700_e151011_d_n2;
        locals.var_arg_dn4 = assign98700_e151011_d_n4;
        locals.var_arg_dn5 = assign98700_e151011_d_n5;
        locals.var_arg_dn6 = assign98700_e151011_d_n6;
        locals.var_arg_dn7 = assign98700_e151011_d_n7;
        locals.var_arg_dn8 = assign98700_e151011_d_n8;
        locals.var_arg_dn9 = assign98700_e151011_d_n9;
        locals.var_arg_dn10 = assign98700_e151011_d_n10;
        locals.var_arg_dn13 = assign98700_e151011_d_n13;
        locals.var_arg_rv = 0.0;

        let assign98710_e151014: f64 = if p.p505 == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard2286 = assign98710_e151014;
        locals.var_guard2286_rv = 0.0;

        let (assign98720_e151028, assign98720_e151028_d_n0, assign98720_e151028_d_n2, assign98720_e151028_d_n4, assign98720_e151028_d_n5, assign98720_e151028_d_n6, assign98720_e151028_d_n7, assign98720_e151028_d_n8, assign98720_e151028_d_n9, assign98720_e151028_d_n10, assign98720_e151028_d_n13,) = {
    if ((((locals.var_guard2280 == 0.0) && (locals.var_guard2284 != 0.0)) && (locals.var_guard2285 != 0.0)) && (locals.var_guard2286 != 0.0)) {
        let assign98720_e151025: f64 = (locals.var_arg).sqrt();
        let assign98720_e151026: f64 = (1.0 / assign98720_e151025);
        (assign98720_e151026, (-((locals.var_arg_dn0 / (2.0 * assign98720_e151025)) / (assign98720_e151025 * assign98720_e151025))), (-((locals.var_arg_dn2 / (2.0 * assign98720_e151025)) / (assign98720_e151025 * assign98720_e151025))), (-((locals.var_arg_dn4 / (2.0 * assign98720_e151025)) / (assign98720_e151025 * assign98720_e151025))), (-((locals.var_arg_dn5 / (2.0 * assign98720_e151025)) / (assign98720_e151025 * assign98720_e151025))), (-((locals.var_arg_dn6 / (2.0 * assign98720_e151025)) / (assign98720_e151025 * assign98720_e151025))), (-((locals.var_arg_dn7 / (2.0 * assign98720_e151025)) / (assign98720_e151025 * assign98720_e151025))), (-((locals.var_arg_dn8 / (2.0 * assign98720_e151025)) / (assign98720_e151025 * assign98720_e151025))), (-((locals.var_arg_dn9 / (2.0 * assign98720_e151025)) / (assign98720_e151025 * assign98720_e151025))), (-((locals.var_arg_dn10 / (2.0 * assign98720_e151025)) / (assign98720_e151025 * assign98720_e151025))), (-((locals.var_arg_dn13 / (2.0 * assign98720_e151025)) / (assign98720_e151025 * assign98720_e151025))),)
    } else {
        (locals.var_sarg, locals.var_sarg_dn0, locals.var_sarg_dn2, locals.var_sarg_dn4, locals.var_sarg_dn5, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn8, locals.var_sarg_dn9, locals.var_sarg_dn10, locals.var_sarg_dn13,)
    }
};
        locals.var_sarg = assign98720_e151028;
        locals.var_sarg_dn0 = assign98720_e151028_d_n0;
        locals.var_sarg_dn2 = assign98720_e151028_d_n2;
        locals.var_sarg_dn4 = assign98720_e151028_d_n4;
        locals.var_sarg_dn5 = assign98720_e151028_d_n5;
        locals.var_sarg_dn6 = assign98720_e151028_d_n6;
        locals.var_sarg_dn7 = assign98720_e151028_d_n7;
        locals.var_sarg_dn8 = assign98720_e151028_d_n8;
        locals.var_sarg_dn9 = assign98720_e151028_d_n9;
        locals.var_sarg_dn10 = assign98720_e151028_d_n10;
        locals.var_sarg_dn13 = assign98720_e151028_d_n13;
        locals.var_sarg_rv = 0.0;

        let (assign98730_e151048, assign98730_e151048_d_n0, assign98730_e151048_d_n2, assign98730_e151048_d_n4, assign98730_e151048_d_n5, assign98730_e151048_d_n6, assign98730_e151048_d_n7, assign98730_e151048_d_n8, assign98730_e151048_d_n9, assign98730_e151048_d_n10, assign98730_e151048_d_n13,) = {
    if ((((locals.var_guard2280 == 0.0) && (locals.var_guard2284 != 0.0)) && (locals.var_guard2285 != 0.0)) && (locals.var_guard2286 == 0.0)) {
        let (assign98730_e151046, assign98730_e151046_d_n0, assign98730_e151046_d_n2, assign98730_e151046_d_n4, assign98730_e151046_d_n5, assign98730_e151046_d_n6, assign98730_e151046_d_n7, assign98730_e151046_d_n8, assign98730_e151046_d_n9, assign98730_e151046_d_n10, assign98730_e151046_d_n13,) = {
            if (locals.var_arg == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign98730_e151044: f64 = (-p.p505);
                let assign98730_e151045: f64 = (locals.var_arg).powf(assign98730_e151044);
                (assign98730_e151045, if 0.0 == 0.0 && ((assign98730_e151044) as f64).is_finite() && ((assign98730_e151044) as f64).fract() == 0.0 { if assign98730_e151044 == 0.0 { 0.0 } else { (assign98730_e151044 * ((locals.var_arg).powf(assign98730_e151044 - 1.0) * locals.var_arg_dn0)) } } else { (assign98730_e151045 * (assign98730_e151044 * (locals.var_arg_dn0 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign98730_e151044) as f64).is_finite() && ((assign98730_e151044) as f64).fract() == 0.0 { if assign98730_e151044 == 0.0 { 0.0 } else { (assign98730_e151044 * ((locals.var_arg).powf(assign98730_e151044 - 1.0) * locals.var_arg_dn2)) } } else { (assign98730_e151045 * (assign98730_e151044 * (locals.var_arg_dn2 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign98730_e151044) as f64).is_finite() && ((assign98730_e151044) as f64).fract() == 0.0 { if assign98730_e151044 == 0.0 { 0.0 } else { (assign98730_e151044 * ((locals.var_arg).powf(assign98730_e151044 - 1.0) * locals.var_arg_dn4)) } } else { (assign98730_e151045 * (assign98730_e151044 * (locals.var_arg_dn4 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign98730_e151044) as f64).is_finite() && ((assign98730_e151044) as f64).fract() == 0.0 { if assign98730_e151044 == 0.0 { 0.0 } else { (assign98730_e151044 * ((locals.var_arg).powf(assign98730_e151044 - 1.0) * locals.var_arg_dn5)) } } else { (assign98730_e151045 * (assign98730_e151044 * (locals.var_arg_dn5 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign98730_e151044) as f64).is_finite() && ((assign98730_e151044) as f64).fract() == 0.0 { if assign98730_e151044 == 0.0 { 0.0 } else { (assign98730_e151044 * ((locals.var_arg).powf(assign98730_e151044 - 1.0) * locals.var_arg_dn6)) } } else { (assign98730_e151045 * (assign98730_e151044 * (locals.var_arg_dn6 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign98730_e151044) as f64).is_finite() && ((assign98730_e151044) as f64).fract() == 0.0 { if assign98730_e151044 == 0.0 { 0.0 } else { (assign98730_e151044 * ((locals.var_arg).powf(assign98730_e151044 - 1.0) * locals.var_arg_dn7)) } } else { (assign98730_e151045 * (assign98730_e151044 * (locals.var_arg_dn7 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign98730_e151044) as f64).is_finite() && ((assign98730_e151044) as f64).fract() == 0.0 { if assign98730_e151044 == 0.0 { 0.0 } else { (assign98730_e151044 * ((locals.var_arg).powf(assign98730_e151044 - 1.0) * locals.var_arg_dn8)) } } else { (assign98730_e151045 * (assign98730_e151044 * (locals.var_arg_dn8 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign98730_e151044) as f64).is_finite() && ((assign98730_e151044) as f64).fract() == 0.0 { if assign98730_e151044 == 0.0 { 0.0 } else { (assign98730_e151044 * ((locals.var_arg).powf(assign98730_e151044 - 1.0) * locals.var_arg_dn9)) } } else { (assign98730_e151045 * (assign98730_e151044 * (locals.var_arg_dn9 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign98730_e151044) as f64).is_finite() && ((assign98730_e151044) as f64).fract() == 0.0 { if assign98730_e151044 == 0.0 { 0.0 } else { (assign98730_e151044 * ((locals.var_arg).powf(assign98730_e151044 - 1.0) * locals.var_arg_dn10)) } } else { (assign98730_e151045 * (assign98730_e151044 * (locals.var_arg_dn10 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign98730_e151044) as f64).is_finite() && ((assign98730_e151044) as f64).fract() == 0.0 { if assign98730_e151044 == 0.0 { 0.0 } else { (assign98730_e151044 * ((locals.var_arg).powf(assign98730_e151044 - 1.0) * locals.var_arg_dn13)) } } else { (assign98730_e151045 * (assign98730_e151044 * (locals.var_arg_dn13 / locals.var_arg))) },)
            }
        };
        (assign98730_e151046, assign98730_e151046_d_n0, assign98730_e151046_d_n2, assign98730_e151046_d_n4, assign98730_e151046_d_n5, assign98730_e151046_d_n6, assign98730_e151046_d_n7, assign98730_e151046_d_n8, assign98730_e151046_d_n9, assign98730_e151046_d_n10, assign98730_e151046_d_n13,)
    } else {
        (locals.var_sarg, locals.var_sarg_dn0, locals.var_sarg_dn2, locals.var_sarg_dn4, locals.var_sarg_dn5, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn8, locals.var_sarg_dn9, locals.var_sarg_dn10, locals.var_sarg_dn13,)
    }
};
        locals.var_sarg = assign98730_e151048;
        locals.var_sarg_dn0 = assign98730_e151048_d_n0;
        locals.var_sarg_dn2 = assign98730_e151048_d_n2;
        locals.var_sarg_dn4 = assign98730_e151048_d_n4;
        locals.var_sarg_dn5 = assign98730_e151048_d_n5;
        locals.var_sarg_dn6 = assign98730_e151048_d_n6;
        locals.var_sarg_dn7 = assign98730_e151048_d_n7;
        locals.var_sarg_dn8 = assign98730_e151048_d_n8;
        locals.var_sarg_dn9 = assign98730_e151048_d_n9;
        locals.var_sarg_dn10 = assign98730_e151048_d_n10;
        locals.var_sarg_dn13 = assign98730_e151048_d_n13;
        locals.var_sarg_rv = 0.0;

        let (assign98740_e151069, assign98740_e151069_d_n0, assign98740_e151069_d_n2, assign98740_e151069_d_n4, assign98740_e151069_d_n5, assign98740_e151069_d_n6, assign98740_e151069_d_n7, assign98740_e151069_d_n8, assign98740_e151069_d_n9, assign98740_e151069_d_n10, assign98740_e151069_d_n13,) = {
    if (((locals.var_guard2280 == 0.0) && (locals.var_guard2284 != 0.0)) && (locals.var_guard2285 != 0.0)) {
        let assign98740_e151057: f64 = (locals.var_pzbdswg * locals.var_czbdswg);
        let assign98740_e151061: f64 = (locals.var_arg * locals.var_sarg);
        let assign98740_e151062: f64 = (1.0 - assign98740_e151061);
        let assign98740_e151063: f64 = (assign98740_e151057 * assign98740_e151062);
        let assign98740_e151066: f64 = (1.0 - p.p505);
        let assign98740_e151067: f64 = (assign98740_e151063 / assign98740_e151066);
        (assign98740_e151067, (((((locals.var_pzbdswg_dn0 * locals.var_czbdswg) + (locals.var_pzbdswg * locals.var_czbdswg_dn0)) * assign98740_e151062) + (assign98740_e151057 * (-((locals.var_arg_dn0 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn0))))) / assign98740_e151066), (((((locals.var_pzbdswg_dn2 * locals.var_czbdswg) + (locals.var_pzbdswg * locals.var_czbdswg_dn2)) * assign98740_e151062) + (assign98740_e151057 * (-((locals.var_arg_dn2 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn2))))) / assign98740_e151066), (((((locals.var_pzbdswg_dn4 * locals.var_czbdswg) + (locals.var_pzbdswg * locals.var_czbdswg_dn4)) * assign98740_e151062) + (assign98740_e151057 * (-((locals.var_arg_dn4 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn4))))) / assign98740_e151066), (((((locals.var_pzbdswg_dn5 * locals.var_czbdswg) + (locals.var_pzbdswg * locals.var_czbdswg_dn5)) * assign98740_e151062) + (assign98740_e151057 * (-((locals.var_arg_dn5 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn5))))) / assign98740_e151066), (((((locals.var_pzbdswg_dn6 * locals.var_czbdswg) + (locals.var_pzbdswg * locals.var_czbdswg_dn6)) * assign98740_e151062) + (assign98740_e151057 * (-((locals.var_arg_dn6 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn6))))) / assign98740_e151066), (((((locals.var_pzbdswg_dn7 * locals.var_czbdswg) + (locals.var_pzbdswg * locals.var_czbdswg_dn7)) * assign98740_e151062) + (assign98740_e151057 * (-((locals.var_arg_dn7 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn7))))) / assign98740_e151066), (((((locals.var_pzbdswg_dn8 * locals.var_czbdswg) + (locals.var_pzbdswg * locals.var_czbdswg_dn8)) * assign98740_e151062) + (assign98740_e151057 * (-((locals.var_arg_dn8 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn8))))) / assign98740_e151066), (((((locals.var_pzbdswg_dn9 * locals.var_czbdswg) + (locals.var_pzbdswg * locals.var_czbdswg_dn9)) * assign98740_e151062) + (assign98740_e151057 * (-((locals.var_arg_dn9 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn9))))) / assign98740_e151066), (((((locals.var_pzbdswg_dn10 * locals.var_czbdswg) + (locals.var_pzbdswg * locals.var_czbdswg_dn10)) * assign98740_e151062) + (assign98740_e151057 * (-((locals.var_arg_dn10 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn10))))) / assign98740_e151066), (((((locals.var_pzbdswg_dn13 * locals.var_czbdswg) + (locals.var_pzbdswg * locals.var_czbdswg_dn13)) * assign98740_e151062) + (assign98740_e151057 * (-((locals.var_arg_dn13 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn13))))) / assign98740_e151066),)
    } else {
        (locals.var_qbd_swg, locals.var_qbd_swg_dn0, locals.var_qbd_swg_dn2, locals.var_qbd_swg_dn4, locals.var_qbd_swg_dn5, locals.var_qbd_swg_dn6, locals.var_qbd_swg_dn7, locals.var_qbd_swg_dn8, locals.var_qbd_swg_dn9, locals.var_qbd_swg_dn10, locals.var_qbd_swg_dn13,)
    }
};
        locals.var_qbd_swg = assign98740_e151069;
        locals.var_qbd_swg_dn0 = assign98740_e151069_d_n0;
        locals.var_qbd_swg_dn2 = assign98740_e151069_d_n2;
        locals.var_qbd_swg_dn4 = assign98740_e151069_d_n4;
        locals.var_qbd_swg_dn5 = assign98740_e151069_d_n5;
        locals.var_qbd_swg_dn6 = assign98740_e151069_d_n6;
        locals.var_qbd_swg_dn7 = assign98740_e151069_d_n7;
        locals.var_qbd_swg_dn8 = assign98740_e151069_d_n8;
        locals.var_qbd_swg_dn9 = assign98740_e151069_d_n9;
        locals.var_qbd_swg_dn10 = assign98740_e151069_d_n10;
        locals.var_qbd_swg_dn13 = assign98740_e151069_d_n13;
        locals.var_qbd_swg_rv = 0.0;

        let (assign98760_e151090, assign98760_e151090_d_n0, assign98760_e151090_d_n2, assign98760_e151090_d_n4, assign98760_e151090_d_n5, assign98760_e151090_d_n6, assign98760_e151090_d_n7, assign98760_e151090_d_n8, assign98760_e151090_d_n9, assign98760_e151090_d_n10, assign98760_e151090_d_n13,) = {
    if (((locals.var_guard2280 == 0.0) && (locals.var_guard2284 != 0.0)) && (locals.var_guard2285 == 0.0)) {
        (locals.var_czbdswg, locals.var_czbdswg_dn0, locals.var_czbdswg_dn2, locals.var_czbdswg_dn4, locals.var_czbdswg_dn5, locals.var_czbdswg_dn6, locals.var_czbdswg_dn7, locals.var_czbdswg_dn8, locals.var_czbdswg_dn9, locals.var_czbdswg_dn10, locals.var_czbdswg_dn13,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign98760_e151090;
        locals.var_t1_dn0 = assign98760_e151090_d_n0;
        locals.var_t1_dn2 = assign98760_e151090_d_n2;
        locals.var_t1_dn4 = assign98760_e151090_d_n4;
        locals.var_t1_dn5 = assign98760_e151090_d_n5;
        locals.var_t1_dn6 = assign98760_e151090_d_n6;
        locals.var_t1_dn7 = assign98760_e151090_d_n7;
        locals.var_t1_dn8 = assign98760_e151090_d_n8;
        locals.var_t1_dn9 = assign98760_e151090_d_n9;
        locals.var_t1_dn10 = assign98760_e151090_d_n10;
        locals.var_t1_dn13 = assign98760_e151090_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign98770_e151104, assign98770_e151104_d_n0, assign98770_e151104_d_n2, assign98770_e151104_d_n4, assign98770_e151104_d_n5, assign98770_e151104_d_n6, assign98770_e151104_d_n7, assign98770_e151104_d_n8, assign98770_e151104_d_n9, assign98770_e151104_d_n10, assign98770_e151104_d_n13,) = {
    if (((locals.var_guard2280 == 0.0) && (locals.var_guard2284 != 0.0)) && (locals.var_guard2285 == 0.0)) {
        let assign98770_e151100: f64 = (locals.var_czbdswg * p.p505);
        let assign98770_e151102: f64 = (assign98770_e151100 / locals.var_pzbdswg);
        (assign98770_e151102, ((((locals.var_czbdswg_dn0 * p.p505) * locals.var_pzbdswg) - (assign98770_e151100 * locals.var_pzbdswg_dn0)) / (locals.var_pzbdswg * locals.var_pzbdswg)), ((((locals.var_czbdswg_dn2 * p.p505) * locals.var_pzbdswg) - (assign98770_e151100 * locals.var_pzbdswg_dn2)) / (locals.var_pzbdswg * locals.var_pzbdswg)), ((((locals.var_czbdswg_dn4 * p.p505) * locals.var_pzbdswg) - (assign98770_e151100 * locals.var_pzbdswg_dn4)) / (locals.var_pzbdswg * locals.var_pzbdswg)), ((((locals.var_czbdswg_dn5 * p.p505) * locals.var_pzbdswg) - (assign98770_e151100 * locals.var_pzbdswg_dn5)) / (locals.var_pzbdswg * locals.var_pzbdswg)), ((((locals.var_czbdswg_dn6 * p.p505) * locals.var_pzbdswg) - (assign98770_e151100 * locals.var_pzbdswg_dn6)) / (locals.var_pzbdswg * locals.var_pzbdswg)), ((((locals.var_czbdswg_dn7 * p.p505) * locals.var_pzbdswg) - (assign98770_e151100 * locals.var_pzbdswg_dn7)) / (locals.var_pzbdswg * locals.var_pzbdswg)), ((((locals.var_czbdswg_dn8 * p.p505) * locals.var_pzbdswg) - (assign98770_e151100 * locals.var_pzbdswg_dn8)) / (locals.var_pzbdswg * locals.var_pzbdswg)), ((((locals.var_czbdswg_dn9 * p.p505) * locals.var_pzbdswg) - (assign98770_e151100 * locals.var_pzbdswg_dn9)) / (locals.var_pzbdswg * locals.var_pzbdswg)), ((((locals.var_czbdswg_dn10 * p.p505) * locals.var_pzbdswg) - (assign98770_e151100 * locals.var_pzbdswg_dn10)) / (locals.var_pzbdswg * locals.var_pzbdswg)), ((((locals.var_czbdswg_dn13 * p.p505) * locals.var_pzbdswg) - (assign98770_e151100 * locals.var_pzbdswg_dn13)) / (locals.var_pzbdswg * locals.var_pzbdswg)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign98770_e151104;
        locals.var_t2_dn0 = assign98770_e151104_d_n0;
        locals.var_t2_dn2 = assign98770_e151104_d_n2;
        locals.var_t2_dn4 = assign98770_e151104_d_n4;
        locals.var_t2_dn5 = assign98770_e151104_d_n5;
        locals.var_t2_dn6 = assign98770_e151104_d_n6;
        locals.var_t2_dn7 = assign98770_e151104_d_n7;
        locals.var_t2_dn8 = assign98770_e151104_d_n8;
        locals.var_t2_dn9 = assign98770_e151104_d_n9;
        locals.var_t2_dn10 = assign98770_e151104_d_n10;
        locals.var_t2_dn13 = assign98770_e151104_d_n13;
        locals.var_t2_rv = 0.0;

        let (assign98780_e151122, assign98780_e151122_d_n0, assign98780_e151122_d_n2, assign98780_e151122_d_n4, assign98780_e151122_d_n5, assign98780_e151122_d_n6, assign98780_e151122_d_n7, assign98780_e151122_d_n8, assign98780_e151122_d_n9, assign98780_e151122_d_n10, assign98780_e151122_d_n13,) = {
    if (((locals.var_guard2280 == 0.0) && (locals.var_guard2284 != 0.0)) && (locals.var_guard2285 == 0.0)) {
        let assign98780_e151116: f64 = (locals.var_vbd_jct * 0.5);
        let assign98780_e151118: f64 = (assign98780_e151116 * locals.var_t2);
        let assign98780_e151119: f64 = (locals.var_t1 + assign98780_e151118);
        let assign98780_e151120: f64 = (locals.var_vbd_jct * assign98780_e151119);
        (assign98780_e151120, ((locals.var_vbd_jct_dn0 * assign98780_e151119) + (locals.var_vbd_jct * (locals.var_t1_dn0 + (((locals.var_vbd_jct_dn0 * 0.5) * locals.var_t2) + (assign98780_e151116 * locals.var_t2_dn0))))), (locals.var_vbd_jct * (locals.var_t1_dn2 + (assign98780_e151116 * locals.var_t2_dn2))), (locals.var_vbd_jct * (locals.var_t1_dn4 + (assign98780_e151116 * locals.var_t2_dn4))), (locals.var_vbd_jct * (locals.var_t1_dn5 + (assign98780_e151116 * locals.var_t2_dn5))), (locals.var_vbd_jct * (locals.var_t1_dn6 + (assign98780_e151116 * locals.var_t2_dn6))), (locals.var_vbd_jct * (locals.var_t1_dn7 + (assign98780_e151116 * locals.var_t2_dn7))), (locals.var_vbd_jct * (locals.var_t1_dn8 + (assign98780_e151116 * locals.var_t2_dn8))), ((locals.var_vbd_jct_dn9 * assign98780_e151119) + (locals.var_vbd_jct * (locals.var_t1_dn9 + (((locals.var_vbd_jct_dn9 * 0.5) * locals.var_t2) + (assign98780_e151116 * locals.var_t2_dn9))))), (locals.var_vbd_jct * (locals.var_t1_dn10 + (assign98780_e151116 * locals.var_t2_dn10))), (locals.var_vbd_jct * (locals.var_t1_dn13 + (assign98780_e151116 * locals.var_t2_dn13))),)
    } else {
        (locals.var_qbd_swg, locals.var_qbd_swg_dn0, locals.var_qbd_swg_dn2, locals.var_qbd_swg_dn4, locals.var_qbd_swg_dn5, locals.var_qbd_swg_dn6, locals.var_qbd_swg_dn7, locals.var_qbd_swg_dn8, locals.var_qbd_swg_dn9, locals.var_qbd_swg_dn10, locals.var_qbd_swg_dn13,)
    }
};
        locals.var_qbd_swg = assign98780_e151122;
        locals.var_qbd_swg_dn0 = assign98780_e151122_d_n0;
        locals.var_qbd_swg_dn2 = assign98780_e151122_d_n2;
        locals.var_qbd_swg_dn4 = assign98780_e151122_d_n4;
        locals.var_qbd_swg_dn5 = assign98780_e151122_d_n5;
        locals.var_qbd_swg_dn6 = assign98780_e151122_d_n6;
        locals.var_qbd_swg_dn7 = assign98780_e151122_d_n7;
        locals.var_qbd_swg_dn8 = assign98780_e151122_d_n8;
        locals.var_qbd_swg_dn9 = assign98780_e151122_d_n9;
        locals.var_qbd_swg_dn10 = assign98780_e151122_d_n10;
        locals.var_qbd_swg_dn13 = assign98780_e151122_d_n13;
        locals.var_qbd_swg_rv = 0.0;

        let (assign98800_e151144, assign98800_e151144_d_n0, assign98800_e151144_d_n2, assign98800_e151144_d_n4, assign98800_e151144_d_n5, assign98800_e151144_d_n6, assign98800_e151144_d_n7, assign98800_e151144_d_n8, assign98800_e151144_d_n9, assign98800_e151144_d_n10, assign98800_e151144_d_n13,) = {
    if ((locals.var_guard2280 == 0.0) && (locals.var_guard2284 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qbd_swg, locals.var_qbd_swg_dn0, locals.var_qbd_swg_dn2, locals.var_qbd_swg_dn4, locals.var_qbd_swg_dn5, locals.var_qbd_swg_dn6, locals.var_qbd_swg_dn7, locals.var_qbd_swg_dn8, locals.var_qbd_swg_dn9, locals.var_qbd_swg_dn10, locals.var_qbd_swg_dn13,)
    }
};
        locals.var_qbd_swg = assign98800_e151144;
        locals.var_qbd_swg_dn0 = assign98800_e151144_d_n0;
        locals.var_qbd_swg_dn2 = assign98800_e151144_d_n2;
        locals.var_qbd_swg_dn4 = assign98800_e151144_d_n4;
        locals.var_qbd_swg_dn5 = assign98800_e151144_d_n5;
        locals.var_qbd_swg_dn6 = assign98800_e151144_d_n6;
        locals.var_qbd_swg_dn7 = assign98800_e151144_d_n7;
        locals.var_qbd_swg_dn8 = assign98800_e151144_d_n8;
        locals.var_qbd_swg_dn9 = assign98800_e151144_d_n9;
        locals.var_qbd_swg_dn10 = assign98800_e151144_d_n10;
        locals.var_qbd_swg_dn13 = assign98800_e151144_d_n13;
        locals.var_qbd_swg_rv = 0.0;

        let assign98820_e151155: f64 = if locals.var_czbs > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2287 = assign98820_e151155;
        locals.var_guard2287_rv = 0.0;

        let assign98830_e151158: f64 = if locals.var_vbs_jct < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2288 = assign98830_e151158;
        locals.var_guard2288_rv = 0.0;

        let (assign98840_e151168, assign98840_e151168_d_n0, assign98840_e151168_d_n2, assign98840_e151168_d_n4, assign98840_e151168_d_n5, assign98840_e151168_d_n6, assign98840_e151168_d_n7, assign98840_e151168_d_n8, assign98840_e151168_d_n9, assign98840_e151168_d_n10, assign98840_e151168_d_n13,) = {
    if ((locals.var_guard2287 != 0.0) && (locals.var_guard2288 != 0.0)) {
        let assign98840_e151165: f64 = (locals.var_vbs_jct / locals.var_pzbs);
        let assign98840_e151166: f64 = (1.0 - assign98840_e151165);
        (assign98840_e151166, (-(-((locals.var_vbs_jct * locals.var_pzbs_dn0) / (locals.var_pzbs * locals.var_pzbs)))), (-(((locals.var_vbs_jct_dn2 * locals.var_pzbs) - (locals.var_vbs_jct * locals.var_pzbs_dn2)) / (locals.var_pzbs * locals.var_pzbs))), (-(-((locals.var_vbs_jct * locals.var_pzbs_dn4) / (locals.var_pzbs * locals.var_pzbs)))), (-(-((locals.var_vbs_jct * locals.var_pzbs_dn5) / (locals.var_pzbs * locals.var_pzbs)))), (-(-((locals.var_vbs_jct * locals.var_pzbs_dn6) / (locals.var_pzbs * locals.var_pzbs)))), (-(-((locals.var_vbs_jct * locals.var_pzbs_dn7) / (locals.var_pzbs * locals.var_pzbs)))), (-(-((locals.var_vbs_jct * locals.var_pzbs_dn8) / (locals.var_pzbs * locals.var_pzbs)))), (-(-((locals.var_vbs_jct * locals.var_pzbs_dn9) / (locals.var_pzbs * locals.var_pzbs)))), (-(((locals.var_vbs_jct_dn10 * locals.var_pzbs) - (locals.var_vbs_jct * locals.var_pzbs_dn10)) / (locals.var_pzbs * locals.var_pzbs))), (-(-((locals.var_vbs_jct * locals.var_pzbs_dn13) / (locals.var_pzbs * locals.var_pzbs)))),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    }
};
        locals.var_arg = assign98840_e151168;
        locals.var_arg_dn0 = assign98840_e151168_d_n0;
        locals.var_arg_dn2 = assign98840_e151168_d_n2;
        locals.var_arg_dn4 = assign98840_e151168_d_n4;
        locals.var_arg_dn5 = assign98840_e151168_d_n5;
        locals.var_arg_dn6 = assign98840_e151168_d_n6;
        locals.var_arg_dn7 = assign98840_e151168_d_n7;
        locals.var_arg_dn8 = assign98840_e151168_d_n8;
        locals.var_arg_dn9 = assign98840_e151168_d_n9;
        locals.var_arg_dn10 = assign98840_e151168_d_n10;
        locals.var_arg_dn13 = assign98840_e151168_d_n13;
        locals.var_arg_rv = 0.0;

        let assign98850_e151171: f64 = if p.p526 == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard2289 = assign98850_e151171;
        locals.var_guard2289_rv = 0.0;

        let (assign98860_e151182, assign98860_e151182_d_n0, assign98860_e151182_d_n2, assign98860_e151182_d_n4, assign98860_e151182_d_n5, assign98860_e151182_d_n6, assign98860_e151182_d_n7, assign98860_e151182_d_n8, assign98860_e151182_d_n9, assign98860_e151182_d_n10, assign98860_e151182_d_n13,) = {
    if (((locals.var_guard2287 != 0.0) && (locals.var_guard2288 != 0.0)) && (locals.var_guard2289 != 0.0)) {
        let assign98860_e151179: f64 = (locals.var_arg).sqrt();
        let assign98860_e151180: f64 = (1.0 / assign98860_e151179);
        (assign98860_e151180, (-((locals.var_arg_dn0 / (2.0 * assign98860_e151179)) / (assign98860_e151179 * assign98860_e151179))), (-((locals.var_arg_dn2 / (2.0 * assign98860_e151179)) / (assign98860_e151179 * assign98860_e151179))), (-((locals.var_arg_dn4 / (2.0 * assign98860_e151179)) / (assign98860_e151179 * assign98860_e151179))), (-((locals.var_arg_dn5 / (2.0 * assign98860_e151179)) / (assign98860_e151179 * assign98860_e151179))), (-((locals.var_arg_dn6 / (2.0 * assign98860_e151179)) / (assign98860_e151179 * assign98860_e151179))), (-((locals.var_arg_dn7 / (2.0 * assign98860_e151179)) / (assign98860_e151179 * assign98860_e151179))), (-((locals.var_arg_dn8 / (2.0 * assign98860_e151179)) / (assign98860_e151179 * assign98860_e151179))), (-((locals.var_arg_dn9 / (2.0 * assign98860_e151179)) / (assign98860_e151179 * assign98860_e151179))), (-((locals.var_arg_dn10 / (2.0 * assign98860_e151179)) / (assign98860_e151179 * assign98860_e151179))), (-((locals.var_arg_dn13 / (2.0 * assign98860_e151179)) / (assign98860_e151179 * assign98860_e151179))),)
    } else {
        (locals.var_sarg, locals.var_sarg_dn0, locals.var_sarg_dn2, locals.var_sarg_dn4, locals.var_sarg_dn5, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn8, locals.var_sarg_dn9, locals.var_sarg_dn10, locals.var_sarg_dn13,)
    }
};
        locals.var_sarg = assign98860_e151182;
        locals.var_sarg_dn0 = assign98860_e151182_d_n0;
        locals.var_sarg_dn2 = assign98860_e151182_d_n2;
        locals.var_sarg_dn4 = assign98860_e151182_d_n4;
        locals.var_sarg_dn5 = assign98860_e151182_d_n5;
        locals.var_sarg_dn6 = assign98860_e151182_d_n6;
        locals.var_sarg_dn7 = assign98860_e151182_d_n7;
        locals.var_sarg_dn8 = assign98860_e151182_d_n8;
        locals.var_sarg_dn9 = assign98860_e151182_d_n9;
        locals.var_sarg_dn10 = assign98860_e151182_d_n10;
        locals.var_sarg_dn13 = assign98860_e151182_d_n13;
        locals.var_sarg_rv = 0.0;

        let (assign98870_e151199, assign98870_e151199_d_n0, assign98870_e151199_d_n2, assign98870_e151199_d_n4, assign98870_e151199_d_n5, assign98870_e151199_d_n6, assign98870_e151199_d_n7, assign98870_e151199_d_n8, assign98870_e151199_d_n9, assign98870_e151199_d_n10, assign98870_e151199_d_n13,) = {
    if (((locals.var_guard2287 != 0.0) && (locals.var_guard2288 != 0.0)) && (locals.var_guard2289 == 0.0)) {
        let (assign98870_e151197, assign98870_e151197_d_n0, assign98870_e151197_d_n2, assign98870_e151197_d_n4, assign98870_e151197_d_n5, assign98870_e151197_d_n6, assign98870_e151197_d_n7, assign98870_e151197_d_n8, assign98870_e151197_d_n9, assign98870_e151197_d_n10, assign98870_e151197_d_n13,) = {
            if (locals.var_arg == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign98870_e151195: f64 = (-p.p526);
                let assign98870_e151196: f64 = (locals.var_arg).powf(assign98870_e151195);
                (assign98870_e151196, if 0.0 == 0.0 && ((assign98870_e151195) as f64).is_finite() && ((assign98870_e151195) as f64).fract() == 0.0 { if assign98870_e151195 == 0.0 { 0.0 } else { (assign98870_e151195 * ((locals.var_arg).powf(assign98870_e151195 - 1.0) * locals.var_arg_dn0)) } } else { (assign98870_e151196 * (assign98870_e151195 * (locals.var_arg_dn0 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign98870_e151195) as f64).is_finite() && ((assign98870_e151195) as f64).fract() == 0.0 { if assign98870_e151195 == 0.0 { 0.0 } else { (assign98870_e151195 * ((locals.var_arg).powf(assign98870_e151195 - 1.0) * locals.var_arg_dn2)) } } else { (assign98870_e151196 * (assign98870_e151195 * (locals.var_arg_dn2 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign98870_e151195) as f64).is_finite() && ((assign98870_e151195) as f64).fract() == 0.0 { if assign98870_e151195 == 0.0 { 0.0 } else { (assign98870_e151195 * ((locals.var_arg).powf(assign98870_e151195 - 1.0) * locals.var_arg_dn4)) } } else { (assign98870_e151196 * (assign98870_e151195 * (locals.var_arg_dn4 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign98870_e151195) as f64).is_finite() && ((assign98870_e151195) as f64).fract() == 0.0 { if assign98870_e151195 == 0.0 { 0.0 } else { (assign98870_e151195 * ((locals.var_arg).powf(assign98870_e151195 - 1.0) * locals.var_arg_dn5)) } } else { (assign98870_e151196 * (assign98870_e151195 * (locals.var_arg_dn5 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign98870_e151195) as f64).is_finite() && ((assign98870_e151195) as f64).fract() == 0.0 { if assign98870_e151195 == 0.0 { 0.0 } else { (assign98870_e151195 * ((locals.var_arg).powf(assign98870_e151195 - 1.0) * locals.var_arg_dn6)) } } else { (assign98870_e151196 * (assign98870_e151195 * (locals.var_arg_dn6 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign98870_e151195) as f64).is_finite() && ((assign98870_e151195) as f64).fract() == 0.0 { if assign98870_e151195 == 0.0 { 0.0 } else { (assign98870_e151195 * ((locals.var_arg).powf(assign98870_e151195 - 1.0) * locals.var_arg_dn7)) } } else { (assign98870_e151196 * (assign98870_e151195 * (locals.var_arg_dn7 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign98870_e151195) as f64).is_finite() && ((assign98870_e151195) as f64).fract() == 0.0 { if assign98870_e151195 == 0.0 { 0.0 } else { (assign98870_e151195 * ((locals.var_arg).powf(assign98870_e151195 - 1.0) * locals.var_arg_dn8)) } } else { (assign98870_e151196 * (assign98870_e151195 * (locals.var_arg_dn8 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign98870_e151195) as f64).is_finite() && ((assign98870_e151195) as f64).fract() == 0.0 { if assign98870_e151195 == 0.0 { 0.0 } else { (assign98870_e151195 * ((locals.var_arg).powf(assign98870_e151195 - 1.0) * locals.var_arg_dn9)) } } else { (assign98870_e151196 * (assign98870_e151195 * (locals.var_arg_dn9 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign98870_e151195) as f64).is_finite() && ((assign98870_e151195) as f64).fract() == 0.0 { if assign98870_e151195 == 0.0 { 0.0 } else { (assign98870_e151195 * ((locals.var_arg).powf(assign98870_e151195 - 1.0) * locals.var_arg_dn10)) } } else { (assign98870_e151196 * (assign98870_e151195 * (locals.var_arg_dn10 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign98870_e151195) as f64).is_finite() && ((assign98870_e151195) as f64).fract() == 0.0 { if assign98870_e151195 == 0.0 { 0.0 } else { (assign98870_e151195 * ((locals.var_arg).powf(assign98870_e151195 - 1.0) * locals.var_arg_dn13)) } } else { (assign98870_e151196 * (assign98870_e151195 * (locals.var_arg_dn13 / locals.var_arg))) },)
            }
        };
        (assign98870_e151197, assign98870_e151197_d_n0, assign98870_e151197_d_n2, assign98870_e151197_d_n4, assign98870_e151197_d_n5, assign98870_e151197_d_n6, assign98870_e151197_d_n7, assign98870_e151197_d_n8, assign98870_e151197_d_n9, assign98870_e151197_d_n10, assign98870_e151197_d_n13,)
    } else {
        (locals.var_sarg, locals.var_sarg_dn0, locals.var_sarg_dn2, locals.var_sarg_dn4, locals.var_sarg_dn5, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn8, locals.var_sarg_dn9, locals.var_sarg_dn10, locals.var_sarg_dn13,)
    }
};
        locals.var_sarg = assign98870_e151199;
        locals.var_sarg_dn0 = assign98870_e151199_d_n0;
        locals.var_sarg_dn2 = assign98870_e151199_d_n2;
        locals.var_sarg_dn4 = assign98870_e151199_d_n4;
        locals.var_sarg_dn5 = assign98870_e151199_d_n5;
        locals.var_sarg_dn6 = assign98870_e151199_d_n6;
        locals.var_sarg_dn7 = assign98870_e151199_d_n7;
        locals.var_sarg_dn8 = assign98870_e151199_d_n8;
        locals.var_sarg_dn9 = assign98870_e151199_d_n9;
        locals.var_sarg_dn10 = assign98870_e151199_d_n10;
        locals.var_sarg_dn13 = assign98870_e151199_d_n13;
        locals.var_sarg_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_369(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign98880_e151217, assign98880_e151217_d_n0, assign98880_e151217_d_n2, assign98880_e151217_d_n4, assign98880_e151217_d_n5, assign98880_e151217_d_n6, assign98880_e151217_d_n7, assign98880_e151217_d_n8, assign98880_e151217_d_n9, assign98880_e151217_d_n10, assign98880_e151217_d_n13,) = {
    if ((locals.var_guard2287 != 0.0) && (locals.var_guard2288 != 0.0)) {
        let assign98880_e151205: f64 = (locals.var_pzbs * locals.var_czbs);
        let assign98880_e151209: f64 = (locals.var_arg * locals.var_sarg);
        let assign98880_e151210: f64 = (1.0 - assign98880_e151209);
        let assign98880_e151211: f64 = (assign98880_e151205 * assign98880_e151210);
        let assign98880_e151214: f64 = (1.0 - p.p526);
        let assign98880_e151215: f64 = (assign98880_e151211 / assign98880_e151214);
        (assign98880_e151215, (((((locals.var_pzbs_dn0 * locals.var_czbs) + (locals.var_pzbs * locals.var_czbs_dn0)) * assign98880_e151210) + (assign98880_e151205 * (-((locals.var_arg_dn0 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn0))))) / assign98880_e151214), (((((locals.var_pzbs_dn2 * locals.var_czbs) + (locals.var_pzbs * locals.var_czbs_dn2)) * assign98880_e151210) + (assign98880_e151205 * (-((locals.var_arg_dn2 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn2))))) / assign98880_e151214), (((((locals.var_pzbs_dn4 * locals.var_czbs) + (locals.var_pzbs * locals.var_czbs_dn4)) * assign98880_e151210) + (assign98880_e151205 * (-((locals.var_arg_dn4 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn4))))) / assign98880_e151214), (((((locals.var_pzbs_dn5 * locals.var_czbs) + (locals.var_pzbs * locals.var_czbs_dn5)) * assign98880_e151210) + (assign98880_e151205 * (-((locals.var_arg_dn5 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn5))))) / assign98880_e151214), (((((locals.var_pzbs_dn6 * locals.var_czbs) + (locals.var_pzbs * locals.var_czbs_dn6)) * assign98880_e151210) + (assign98880_e151205 * (-((locals.var_arg_dn6 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn6))))) / assign98880_e151214), (((((locals.var_pzbs_dn7 * locals.var_czbs) + (locals.var_pzbs * locals.var_czbs_dn7)) * assign98880_e151210) + (assign98880_e151205 * (-((locals.var_arg_dn7 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn7))))) / assign98880_e151214), (((((locals.var_pzbs_dn8 * locals.var_czbs) + (locals.var_pzbs * locals.var_czbs_dn8)) * assign98880_e151210) + (assign98880_e151205 * (-((locals.var_arg_dn8 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn8))))) / assign98880_e151214), (((((locals.var_pzbs_dn9 * locals.var_czbs) + (locals.var_pzbs * locals.var_czbs_dn9)) * assign98880_e151210) + (assign98880_e151205 * (-((locals.var_arg_dn9 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn9))))) / assign98880_e151214), (((((locals.var_pzbs_dn10 * locals.var_czbs) + (locals.var_pzbs * locals.var_czbs_dn10)) * assign98880_e151210) + (assign98880_e151205 * (-((locals.var_arg_dn10 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn10))))) / assign98880_e151214), (((((locals.var_pzbs_dn13 * locals.var_czbs) + (locals.var_pzbs * locals.var_czbs_dn13)) * assign98880_e151210) + (assign98880_e151205 * (-((locals.var_arg_dn13 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn13))))) / assign98880_e151214),)
    } else {
        (locals.var_qbs_btm, locals.var_qbs_btm_dn0, locals.var_qbs_btm_dn2, locals.var_qbs_btm_dn4, locals.var_qbs_btm_dn5, locals.var_qbs_btm_dn6, locals.var_qbs_btm_dn7, locals.var_qbs_btm_dn8, locals.var_qbs_btm_dn9, locals.var_qbs_btm_dn10, locals.var_qbs_btm_dn13,)
    }
};
        locals.var_qbs_btm = assign98880_e151217;
        locals.var_qbs_btm_dn0 = assign98880_e151217_d_n0;
        locals.var_qbs_btm_dn2 = assign98880_e151217_d_n2;
        locals.var_qbs_btm_dn4 = assign98880_e151217_d_n4;
        locals.var_qbs_btm_dn5 = assign98880_e151217_d_n5;
        locals.var_qbs_btm_dn6 = assign98880_e151217_d_n6;
        locals.var_qbs_btm_dn7 = assign98880_e151217_d_n7;
        locals.var_qbs_btm_dn8 = assign98880_e151217_d_n8;
        locals.var_qbs_btm_dn9 = assign98880_e151217_d_n9;
        locals.var_qbs_btm_dn10 = assign98880_e151217_d_n10;
        locals.var_qbs_btm_dn13 = assign98880_e151217_d_n13;
        locals.var_qbs_btm_rv = 0.0;

        let (assign98900_e151232, assign98900_e151232_d_n0, assign98900_e151232_d_n2, assign98900_e151232_d_n4, assign98900_e151232_d_n5, assign98900_e151232_d_n6, assign98900_e151232_d_n7, assign98900_e151232_d_n8, assign98900_e151232_d_n9, assign98900_e151232_d_n10, assign98900_e151232_d_n13,) = {
    if ((locals.var_guard2287 != 0.0) && (locals.var_guard2288 == 0.0)) {
        (locals.var_czbs, locals.var_czbs_dn0, locals.var_czbs_dn2, locals.var_czbs_dn4, locals.var_czbs_dn5, locals.var_czbs_dn6, locals.var_czbs_dn7, locals.var_czbs_dn8, locals.var_czbs_dn9, locals.var_czbs_dn10, locals.var_czbs_dn13,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign98900_e151232;
        locals.var_t1_dn0 = assign98900_e151232_d_n0;
        locals.var_t1_dn2 = assign98900_e151232_d_n2;
        locals.var_t1_dn4 = assign98900_e151232_d_n4;
        locals.var_t1_dn5 = assign98900_e151232_d_n5;
        locals.var_t1_dn6 = assign98900_e151232_d_n6;
        locals.var_t1_dn7 = assign98900_e151232_d_n7;
        locals.var_t1_dn8 = assign98900_e151232_d_n8;
        locals.var_t1_dn9 = assign98900_e151232_d_n9;
        locals.var_t1_dn10 = assign98900_e151232_d_n10;
        locals.var_t1_dn13 = assign98900_e151232_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign98910_e151243, assign98910_e151243_d_n0, assign98910_e151243_d_n2, assign98910_e151243_d_n4, assign98910_e151243_d_n5, assign98910_e151243_d_n6, assign98910_e151243_d_n7, assign98910_e151243_d_n8, assign98910_e151243_d_n9, assign98910_e151243_d_n10, assign98910_e151243_d_n13,) = {
    if ((locals.var_guard2287 != 0.0) && (locals.var_guard2288 == 0.0)) {
        let assign98910_e151239: f64 = (locals.var_czbs * p.p526);
        let assign98910_e151241: f64 = (assign98910_e151239 / locals.var_pzbs);
        (assign98910_e151241, ((((locals.var_czbs_dn0 * p.p526) * locals.var_pzbs) - (assign98910_e151239 * locals.var_pzbs_dn0)) / (locals.var_pzbs * locals.var_pzbs)), ((((locals.var_czbs_dn2 * p.p526) * locals.var_pzbs) - (assign98910_e151239 * locals.var_pzbs_dn2)) / (locals.var_pzbs * locals.var_pzbs)), ((((locals.var_czbs_dn4 * p.p526) * locals.var_pzbs) - (assign98910_e151239 * locals.var_pzbs_dn4)) / (locals.var_pzbs * locals.var_pzbs)), ((((locals.var_czbs_dn5 * p.p526) * locals.var_pzbs) - (assign98910_e151239 * locals.var_pzbs_dn5)) / (locals.var_pzbs * locals.var_pzbs)), ((((locals.var_czbs_dn6 * p.p526) * locals.var_pzbs) - (assign98910_e151239 * locals.var_pzbs_dn6)) / (locals.var_pzbs * locals.var_pzbs)), ((((locals.var_czbs_dn7 * p.p526) * locals.var_pzbs) - (assign98910_e151239 * locals.var_pzbs_dn7)) / (locals.var_pzbs * locals.var_pzbs)), ((((locals.var_czbs_dn8 * p.p526) * locals.var_pzbs) - (assign98910_e151239 * locals.var_pzbs_dn8)) / (locals.var_pzbs * locals.var_pzbs)), ((((locals.var_czbs_dn9 * p.p526) * locals.var_pzbs) - (assign98910_e151239 * locals.var_pzbs_dn9)) / (locals.var_pzbs * locals.var_pzbs)), ((((locals.var_czbs_dn10 * p.p526) * locals.var_pzbs) - (assign98910_e151239 * locals.var_pzbs_dn10)) / (locals.var_pzbs * locals.var_pzbs)), ((((locals.var_czbs_dn13 * p.p526) * locals.var_pzbs) - (assign98910_e151239 * locals.var_pzbs_dn13)) / (locals.var_pzbs * locals.var_pzbs)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign98910_e151243;
        locals.var_t2_dn0 = assign98910_e151243_d_n0;
        locals.var_t2_dn2 = assign98910_e151243_d_n2;
        locals.var_t2_dn4 = assign98910_e151243_d_n4;
        locals.var_t2_dn5 = assign98910_e151243_d_n5;
        locals.var_t2_dn6 = assign98910_e151243_d_n6;
        locals.var_t2_dn7 = assign98910_e151243_d_n7;
        locals.var_t2_dn8 = assign98910_e151243_d_n8;
        locals.var_t2_dn9 = assign98910_e151243_d_n9;
        locals.var_t2_dn10 = assign98910_e151243_d_n10;
        locals.var_t2_dn13 = assign98910_e151243_d_n13;
        locals.var_t2_rv = 0.0;

        let (assign98920_e151258, assign98920_e151258_d_n0, assign98920_e151258_d_n2, assign98920_e151258_d_n4, assign98920_e151258_d_n5, assign98920_e151258_d_n6, assign98920_e151258_d_n7, assign98920_e151258_d_n8, assign98920_e151258_d_n9, assign98920_e151258_d_n10, assign98920_e151258_d_n13,) = {
    if ((locals.var_guard2287 != 0.0) && (locals.var_guard2288 == 0.0)) {
        let assign98920_e151252: f64 = (locals.var_vbs_jct * 0.5);
        let assign98920_e151254: f64 = (assign98920_e151252 * locals.var_t2);
        let assign98920_e151255: f64 = (locals.var_t1 + assign98920_e151254);
        let assign98920_e151256: f64 = (locals.var_vbs_jct * assign98920_e151255);
        (assign98920_e151256, (locals.var_vbs_jct * (locals.var_t1_dn0 + (assign98920_e151252 * locals.var_t2_dn0))), ((locals.var_vbs_jct_dn2 * assign98920_e151255) + (locals.var_vbs_jct * (locals.var_t1_dn2 + (((locals.var_vbs_jct_dn2 * 0.5) * locals.var_t2) + (assign98920_e151252 * locals.var_t2_dn2))))), (locals.var_vbs_jct * (locals.var_t1_dn4 + (assign98920_e151252 * locals.var_t2_dn4))), (locals.var_vbs_jct * (locals.var_t1_dn5 + (assign98920_e151252 * locals.var_t2_dn5))), (locals.var_vbs_jct * (locals.var_t1_dn6 + (assign98920_e151252 * locals.var_t2_dn6))), (locals.var_vbs_jct * (locals.var_t1_dn7 + (assign98920_e151252 * locals.var_t2_dn7))), (locals.var_vbs_jct * (locals.var_t1_dn8 + (assign98920_e151252 * locals.var_t2_dn8))), (locals.var_vbs_jct * (locals.var_t1_dn9 + (assign98920_e151252 * locals.var_t2_dn9))), ((locals.var_vbs_jct_dn10 * assign98920_e151255) + (locals.var_vbs_jct * (locals.var_t1_dn10 + (((locals.var_vbs_jct_dn10 * 0.5) * locals.var_t2) + (assign98920_e151252 * locals.var_t2_dn10))))), (locals.var_vbs_jct * (locals.var_t1_dn13 + (assign98920_e151252 * locals.var_t2_dn13))),)
    } else {
        (locals.var_qbs_btm, locals.var_qbs_btm_dn0, locals.var_qbs_btm_dn2, locals.var_qbs_btm_dn4, locals.var_qbs_btm_dn5, locals.var_qbs_btm_dn6, locals.var_qbs_btm_dn7, locals.var_qbs_btm_dn8, locals.var_qbs_btm_dn9, locals.var_qbs_btm_dn10, locals.var_qbs_btm_dn13,)
    }
};
        locals.var_qbs_btm = assign98920_e151258;
        locals.var_qbs_btm_dn0 = assign98920_e151258_d_n0;
        locals.var_qbs_btm_dn2 = assign98920_e151258_d_n2;
        locals.var_qbs_btm_dn4 = assign98920_e151258_d_n4;
        locals.var_qbs_btm_dn5 = assign98920_e151258_d_n5;
        locals.var_qbs_btm_dn6 = assign98920_e151258_d_n6;
        locals.var_qbs_btm_dn7 = assign98920_e151258_d_n7;
        locals.var_qbs_btm_dn8 = assign98920_e151258_d_n8;
        locals.var_qbs_btm_dn9 = assign98920_e151258_d_n9;
        locals.var_qbs_btm_dn10 = assign98920_e151258_d_n10;
        locals.var_qbs_btm_dn13 = assign98920_e151258_d_n13;
        locals.var_qbs_btm_rv = 0.0;

        let (assign98940_e151274, assign98940_e151274_d_n0, assign98940_e151274_d_n2, assign98940_e151274_d_n4, assign98940_e151274_d_n5, assign98940_e151274_d_n6, assign98940_e151274_d_n7, assign98940_e151274_d_n8, assign98940_e151274_d_n9, assign98940_e151274_d_n10, assign98940_e151274_d_n13,) = {
    if (locals.var_guard2287 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qbs_btm, locals.var_qbs_btm_dn0, locals.var_qbs_btm_dn2, locals.var_qbs_btm_dn4, locals.var_qbs_btm_dn5, locals.var_qbs_btm_dn6, locals.var_qbs_btm_dn7, locals.var_qbs_btm_dn8, locals.var_qbs_btm_dn9, locals.var_qbs_btm_dn10, locals.var_qbs_btm_dn13,)
    }
};
        locals.var_qbs_btm = assign98940_e151274;
        locals.var_qbs_btm_dn0 = assign98940_e151274_d_n0;
        locals.var_qbs_btm_dn2 = assign98940_e151274_d_n2;
        locals.var_qbs_btm_dn4 = assign98940_e151274_d_n4;
        locals.var_qbs_btm_dn5 = assign98940_e151274_d_n5;
        locals.var_qbs_btm_dn6 = assign98940_e151274_d_n6;
        locals.var_qbs_btm_dn7 = assign98940_e151274_d_n7;
        locals.var_qbs_btm_dn8 = assign98940_e151274_d_n8;
        locals.var_qbs_btm_dn9 = assign98940_e151274_d_n9;
        locals.var_qbs_btm_dn10 = assign98940_e151274_d_n10;
        locals.var_qbs_btm_dn13 = assign98940_e151274_d_n13;
        locals.var_qbs_btm_rv = 0.0;

        let assign98960_e151282: f64 = if locals.var_czbssw > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2290 = assign98960_e151282;
        locals.var_guard2290_rv = 0.0;

        let assign98970_e151285: f64 = if locals.var_vbs_jct < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2291 = assign98970_e151285;
        locals.var_guard2291_rv = 0.0;

        let (assign98980_e151295, assign98980_e151295_d_n0, assign98980_e151295_d_n2, assign98980_e151295_d_n4, assign98980_e151295_d_n5, assign98980_e151295_d_n6, assign98980_e151295_d_n7, assign98980_e151295_d_n8, assign98980_e151295_d_n9, assign98980_e151295_d_n10, assign98980_e151295_d_n13,) = {
    if ((locals.var_guard2290 != 0.0) && (locals.var_guard2291 != 0.0)) {
        let assign98980_e151292: f64 = (locals.var_vbs_jct / locals.var_pzbssw);
        let assign98980_e151293: f64 = (1.0 - assign98980_e151292);
        (assign98980_e151293, (-(-((locals.var_vbs_jct * locals.var_pzbssw_dn0) / (locals.var_pzbssw * locals.var_pzbssw)))), (-(((locals.var_vbs_jct_dn2 * locals.var_pzbssw) - (locals.var_vbs_jct * locals.var_pzbssw_dn2)) / (locals.var_pzbssw * locals.var_pzbssw))), (-(-((locals.var_vbs_jct * locals.var_pzbssw_dn4) / (locals.var_pzbssw * locals.var_pzbssw)))), (-(-((locals.var_vbs_jct * locals.var_pzbssw_dn5) / (locals.var_pzbssw * locals.var_pzbssw)))), (-(-((locals.var_vbs_jct * locals.var_pzbssw_dn6) / (locals.var_pzbssw * locals.var_pzbssw)))), (-(-((locals.var_vbs_jct * locals.var_pzbssw_dn7) / (locals.var_pzbssw * locals.var_pzbssw)))), (-(-((locals.var_vbs_jct * locals.var_pzbssw_dn8) / (locals.var_pzbssw * locals.var_pzbssw)))), (-(-((locals.var_vbs_jct * locals.var_pzbssw_dn9) / (locals.var_pzbssw * locals.var_pzbssw)))), (-(((locals.var_vbs_jct_dn10 * locals.var_pzbssw) - (locals.var_vbs_jct * locals.var_pzbssw_dn10)) / (locals.var_pzbssw * locals.var_pzbssw))), (-(-((locals.var_vbs_jct * locals.var_pzbssw_dn13) / (locals.var_pzbssw * locals.var_pzbssw)))),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    }
};
        locals.var_arg = assign98980_e151295;
        locals.var_arg_dn0 = assign98980_e151295_d_n0;
        locals.var_arg_dn2 = assign98980_e151295_d_n2;
        locals.var_arg_dn4 = assign98980_e151295_d_n4;
        locals.var_arg_dn5 = assign98980_e151295_d_n5;
        locals.var_arg_dn6 = assign98980_e151295_d_n6;
        locals.var_arg_dn7 = assign98980_e151295_d_n7;
        locals.var_arg_dn8 = assign98980_e151295_d_n8;
        locals.var_arg_dn9 = assign98980_e151295_d_n9;
        locals.var_arg_dn10 = assign98980_e151295_d_n10;
        locals.var_arg_dn13 = assign98980_e151295_d_n13;
        locals.var_arg_rv = 0.0;

        let assign98990_e151298: f64 = if p.p527 == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard2292 = assign98990_e151298;
        locals.var_guard2292_rv = 0.0;

        let (assign99000_e151309, assign99000_e151309_d_n0, assign99000_e151309_d_n2, assign99000_e151309_d_n4, assign99000_e151309_d_n5, assign99000_e151309_d_n6, assign99000_e151309_d_n7, assign99000_e151309_d_n8, assign99000_e151309_d_n9, assign99000_e151309_d_n10, assign99000_e151309_d_n13,) = {
    if (((locals.var_guard2290 != 0.0) && (locals.var_guard2291 != 0.0)) && (locals.var_guard2292 != 0.0)) {
        let assign99000_e151306: f64 = (locals.var_arg).sqrt();
        let assign99000_e151307: f64 = (1.0 / assign99000_e151306);
        (assign99000_e151307, (-((locals.var_arg_dn0 / (2.0 * assign99000_e151306)) / (assign99000_e151306 * assign99000_e151306))), (-((locals.var_arg_dn2 / (2.0 * assign99000_e151306)) / (assign99000_e151306 * assign99000_e151306))), (-((locals.var_arg_dn4 / (2.0 * assign99000_e151306)) / (assign99000_e151306 * assign99000_e151306))), (-((locals.var_arg_dn5 / (2.0 * assign99000_e151306)) / (assign99000_e151306 * assign99000_e151306))), (-((locals.var_arg_dn6 / (2.0 * assign99000_e151306)) / (assign99000_e151306 * assign99000_e151306))), (-((locals.var_arg_dn7 / (2.0 * assign99000_e151306)) / (assign99000_e151306 * assign99000_e151306))), (-((locals.var_arg_dn8 / (2.0 * assign99000_e151306)) / (assign99000_e151306 * assign99000_e151306))), (-((locals.var_arg_dn9 / (2.0 * assign99000_e151306)) / (assign99000_e151306 * assign99000_e151306))), (-((locals.var_arg_dn10 / (2.0 * assign99000_e151306)) / (assign99000_e151306 * assign99000_e151306))), (-((locals.var_arg_dn13 / (2.0 * assign99000_e151306)) / (assign99000_e151306 * assign99000_e151306))),)
    } else {
        (locals.var_sarg, locals.var_sarg_dn0, locals.var_sarg_dn2, locals.var_sarg_dn4, locals.var_sarg_dn5, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn8, locals.var_sarg_dn9, locals.var_sarg_dn10, locals.var_sarg_dn13,)
    }
};
        locals.var_sarg = assign99000_e151309;
        locals.var_sarg_dn0 = assign99000_e151309_d_n0;
        locals.var_sarg_dn2 = assign99000_e151309_d_n2;
        locals.var_sarg_dn4 = assign99000_e151309_d_n4;
        locals.var_sarg_dn5 = assign99000_e151309_d_n5;
        locals.var_sarg_dn6 = assign99000_e151309_d_n6;
        locals.var_sarg_dn7 = assign99000_e151309_d_n7;
        locals.var_sarg_dn8 = assign99000_e151309_d_n8;
        locals.var_sarg_dn9 = assign99000_e151309_d_n9;
        locals.var_sarg_dn10 = assign99000_e151309_d_n10;
        locals.var_sarg_dn13 = assign99000_e151309_d_n13;
        locals.var_sarg_rv = 0.0;

        let (assign99010_e151326, assign99010_e151326_d_n0, assign99010_e151326_d_n2, assign99010_e151326_d_n4, assign99010_e151326_d_n5, assign99010_e151326_d_n6, assign99010_e151326_d_n7, assign99010_e151326_d_n8, assign99010_e151326_d_n9, assign99010_e151326_d_n10, assign99010_e151326_d_n13,) = {
    if (((locals.var_guard2290 != 0.0) && (locals.var_guard2291 != 0.0)) && (locals.var_guard2292 == 0.0)) {
        let (assign99010_e151324, assign99010_e151324_d_n0, assign99010_e151324_d_n2, assign99010_e151324_d_n4, assign99010_e151324_d_n5, assign99010_e151324_d_n6, assign99010_e151324_d_n7, assign99010_e151324_d_n8, assign99010_e151324_d_n9, assign99010_e151324_d_n10, assign99010_e151324_d_n13,) = {
            if (locals.var_arg == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign99010_e151322: f64 = (-p.p527);
                let assign99010_e151323: f64 = (locals.var_arg).powf(assign99010_e151322);
                (assign99010_e151323, if 0.0 == 0.0 && ((assign99010_e151322) as f64).is_finite() && ((assign99010_e151322) as f64).fract() == 0.0 { if assign99010_e151322 == 0.0 { 0.0 } else { (assign99010_e151322 * ((locals.var_arg).powf(assign99010_e151322 - 1.0) * locals.var_arg_dn0)) } } else { (assign99010_e151323 * (assign99010_e151322 * (locals.var_arg_dn0 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign99010_e151322) as f64).is_finite() && ((assign99010_e151322) as f64).fract() == 0.0 { if assign99010_e151322 == 0.0 { 0.0 } else { (assign99010_e151322 * ((locals.var_arg).powf(assign99010_e151322 - 1.0) * locals.var_arg_dn2)) } } else { (assign99010_e151323 * (assign99010_e151322 * (locals.var_arg_dn2 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign99010_e151322) as f64).is_finite() && ((assign99010_e151322) as f64).fract() == 0.0 { if assign99010_e151322 == 0.0 { 0.0 } else { (assign99010_e151322 * ((locals.var_arg).powf(assign99010_e151322 - 1.0) * locals.var_arg_dn4)) } } else { (assign99010_e151323 * (assign99010_e151322 * (locals.var_arg_dn4 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign99010_e151322) as f64).is_finite() && ((assign99010_e151322) as f64).fract() == 0.0 { if assign99010_e151322 == 0.0 { 0.0 } else { (assign99010_e151322 * ((locals.var_arg).powf(assign99010_e151322 - 1.0) * locals.var_arg_dn5)) } } else { (assign99010_e151323 * (assign99010_e151322 * (locals.var_arg_dn5 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign99010_e151322) as f64).is_finite() && ((assign99010_e151322) as f64).fract() == 0.0 { if assign99010_e151322 == 0.0 { 0.0 } else { (assign99010_e151322 * ((locals.var_arg).powf(assign99010_e151322 - 1.0) * locals.var_arg_dn6)) } } else { (assign99010_e151323 * (assign99010_e151322 * (locals.var_arg_dn6 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign99010_e151322) as f64).is_finite() && ((assign99010_e151322) as f64).fract() == 0.0 { if assign99010_e151322 == 0.0 { 0.0 } else { (assign99010_e151322 * ((locals.var_arg).powf(assign99010_e151322 - 1.0) * locals.var_arg_dn7)) } } else { (assign99010_e151323 * (assign99010_e151322 * (locals.var_arg_dn7 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign99010_e151322) as f64).is_finite() && ((assign99010_e151322) as f64).fract() == 0.0 { if assign99010_e151322 == 0.0 { 0.0 } else { (assign99010_e151322 * ((locals.var_arg).powf(assign99010_e151322 - 1.0) * locals.var_arg_dn8)) } } else { (assign99010_e151323 * (assign99010_e151322 * (locals.var_arg_dn8 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign99010_e151322) as f64).is_finite() && ((assign99010_e151322) as f64).fract() == 0.0 { if assign99010_e151322 == 0.0 { 0.0 } else { (assign99010_e151322 * ((locals.var_arg).powf(assign99010_e151322 - 1.0) * locals.var_arg_dn9)) } } else { (assign99010_e151323 * (assign99010_e151322 * (locals.var_arg_dn9 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign99010_e151322) as f64).is_finite() && ((assign99010_e151322) as f64).fract() == 0.0 { if assign99010_e151322 == 0.0 { 0.0 } else { (assign99010_e151322 * ((locals.var_arg).powf(assign99010_e151322 - 1.0) * locals.var_arg_dn10)) } } else { (assign99010_e151323 * (assign99010_e151322 * (locals.var_arg_dn10 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign99010_e151322) as f64).is_finite() && ((assign99010_e151322) as f64).fract() == 0.0 { if assign99010_e151322 == 0.0 { 0.0 } else { (assign99010_e151322 * ((locals.var_arg).powf(assign99010_e151322 - 1.0) * locals.var_arg_dn13)) } } else { (assign99010_e151323 * (assign99010_e151322 * (locals.var_arg_dn13 / locals.var_arg))) },)
            }
        };
        (assign99010_e151324, assign99010_e151324_d_n0, assign99010_e151324_d_n2, assign99010_e151324_d_n4, assign99010_e151324_d_n5, assign99010_e151324_d_n6, assign99010_e151324_d_n7, assign99010_e151324_d_n8, assign99010_e151324_d_n9, assign99010_e151324_d_n10, assign99010_e151324_d_n13,)
    } else {
        (locals.var_sarg, locals.var_sarg_dn0, locals.var_sarg_dn2, locals.var_sarg_dn4, locals.var_sarg_dn5, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn8, locals.var_sarg_dn9, locals.var_sarg_dn10, locals.var_sarg_dn13,)
    }
};
        locals.var_sarg = assign99010_e151326;
        locals.var_sarg_dn0 = assign99010_e151326_d_n0;
        locals.var_sarg_dn2 = assign99010_e151326_d_n2;
        locals.var_sarg_dn4 = assign99010_e151326_d_n4;
        locals.var_sarg_dn5 = assign99010_e151326_d_n5;
        locals.var_sarg_dn6 = assign99010_e151326_d_n6;
        locals.var_sarg_dn7 = assign99010_e151326_d_n7;
        locals.var_sarg_dn8 = assign99010_e151326_d_n8;
        locals.var_sarg_dn9 = assign99010_e151326_d_n9;
        locals.var_sarg_dn10 = assign99010_e151326_d_n10;
        locals.var_sarg_dn13 = assign99010_e151326_d_n13;
        locals.var_sarg_rv = 0.0;

        let (assign99020_e151344, assign99020_e151344_d_n0, assign99020_e151344_d_n2, assign99020_e151344_d_n4, assign99020_e151344_d_n5, assign99020_e151344_d_n6, assign99020_e151344_d_n7, assign99020_e151344_d_n8, assign99020_e151344_d_n9, assign99020_e151344_d_n10, assign99020_e151344_d_n13,) = {
    if ((locals.var_guard2290 != 0.0) && (locals.var_guard2291 != 0.0)) {
        let assign99020_e151332: f64 = (locals.var_pzbssw * locals.var_czbssw);
        let assign99020_e151336: f64 = (locals.var_arg * locals.var_sarg);
        let assign99020_e151337: f64 = (1.0 - assign99020_e151336);
        let assign99020_e151338: f64 = (assign99020_e151332 * assign99020_e151337);
        let assign99020_e151341: f64 = (1.0 - p.p527);
        let assign99020_e151342: f64 = (assign99020_e151338 / assign99020_e151341);
        (assign99020_e151342, (((((locals.var_pzbssw_dn0 * locals.var_czbssw) + (locals.var_pzbssw * locals.var_czbssw_dn0)) * assign99020_e151337) + (assign99020_e151332 * (-((locals.var_arg_dn0 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn0))))) / assign99020_e151341), (((((locals.var_pzbssw_dn2 * locals.var_czbssw) + (locals.var_pzbssw * locals.var_czbssw_dn2)) * assign99020_e151337) + (assign99020_e151332 * (-((locals.var_arg_dn2 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn2))))) / assign99020_e151341), (((((locals.var_pzbssw_dn4 * locals.var_czbssw) + (locals.var_pzbssw * locals.var_czbssw_dn4)) * assign99020_e151337) + (assign99020_e151332 * (-((locals.var_arg_dn4 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn4))))) / assign99020_e151341), (((((locals.var_pzbssw_dn5 * locals.var_czbssw) + (locals.var_pzbssw * locals.var_czbssw_dn5)) * assign99020_e151337) + (assign99020_e151332 * (-((locals.var_arg_dn5 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn5))))) / assign99020_e151341), (((((locals.var_pzbssw_dn6 * locals.var_czbssw) + (locals.var_pzbssw * locals.var_czbssw_dn6)) * assign99020_e151337) + (assign99020_e151332 * (-((locals.var_arg_dn6 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn6))))) / assign99020_e151341), (((((locals.var_pzbssw_dn7 * locals.var_czbssw) + (locals.var_pzbssw * locals.var_czbssw_dn7)) * assign99020_e151337) + (assign99020_e151332 * (-((locals.var_arg_dn7 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn7))))) / assign99020_e151341), (((((locals.var_pzbssw_dn8 * locals.var_czbssw) + (locals.var_pzbssw * locals.var_czbssw_dn8)) * assign99020_e151337) + (assign99020_e151332 * (-((locals.var_arg_dn8 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn8))))) / assign99020_e151341), (((((locals.var_pzbssw_dn9 * locals.var_czbssw) + (locals.var_pzbssw * locals.var_czbssw_dn9)) * assign99020_e151337) + (assign99020_e151332 * (-((locals.var_arg_dn9 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn9))))) / assign99020_e151341), (((((locals.var_pzbssw_dn10 * locals.var_czbssw) + (locals.var_pzbssw * locals.var_czbssw_dn10)) * assign99020_e151337) + (assign99020_e151332 * (-((locals.var_arg_dn10 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn10))))) / assign99020_e151341), (((((locals.var_pzbssw_dn13 * locals.var_czbssw) + (locals.var_pzbssw * locals.var_czbssw_dn13)) * assign99020_e151337) + (assign99020_e151332 * (-((locals.var_arg_dn13 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn13))))) / assign99020_e151341),)
    } else {
        (locals.var_qbs_sws, locals.var_qbs_sws_dn0, locals.var_qbs_sws_dn2, locals.var_qbs_sws_dn4, locals.var_qbs_sws_dn5, locals.var_qbs_sws_dn6, locals.var_qbs_sws_dn7, locals.var_qbs_sws_dn8, locals.var_qbs_sws_dn9, locals.var_qbs_sws_dn10, locals.var_qbs_sws_dn13,)
    }
};
        locals.var_qbs_sws = assign99020_e151344;
        locals.var_qbs_sws_dn0 = assign99020_e151344_d_n0;
        locals.var_qbs_sws_dn2 = assign99020_e151344_d_n2;
        locals.var_qbs_sws_dn4 = assign99020_e151344_d_n4;
        locals.var_qbs_sws_dn5 = assign99020_e151344_d_n5;
        locals.var_qbs_sws_dn6 = assign99020_e151344_d_n6;
        locals.var_qbs_sws_dn7 = assign99020_e151344_d_n7;
        locals.var_qbs_sws_dn8 = assign99020_e151344_d_n8;
        locals.var_qbs_sws_dn9 = assign99020_e151344_d_n9;
        locals.var_qbs_sws_dn10 = assign99020_e151344_d_n10;
        locals.var_qbs_sws_dn13 = assign99020_e151344_d_n13;
        locals.var_qbs_sws_rv = 0.0;

        let (assign99040_e151359, assign99040_e151359_d_n0, assign99040_e151359_d_n2, assign99040_e151359_d_n4, assign99040_e151359_d_n5, assign99040_e151359_d_n6, assign99040_e151359_d_n7, assign99040_e151359_d_n8, assign99040_e151359_d_n9, assign99040_e151359_d_n10, assign99040_e151359_d_n13,) = {
    if ((locals.var_guard2290 != 0.0) && (locals.var_guard2291 == 0.0)) {
        (locals.var_czbssw, locals.var_czbssw_dn0, locals.var_czbssw_dn2, locals.var_czbssw_dn4, locals.var_czbssw_dn5, locals.var_czbssw_dn6, locals.var_czbssw_dn7, locals.var_czbssw_dn8, locals.var_czbssw_dn9, locals.var_czbssw_dn10, locals.var_czbssw_dn13,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign99040_e151359;
        locals.var_t1_dn0 = assign99040_e151359_d_n0;
        locals.var_t1_dn2 = assign99040_e151359_d_n2;
        locals.var_t1_dn4 = assign99040_e151359_d_n4;
        locals.var_t1_dn5 = assign99040_e151359_d_n5;
        locals.var_t1_dn6 = assign99040_e151359_d_n6;
        locals.var_t1_dn7 = assign99040_e151359_d_n7;
        locals.var_t1_dn8 = assign99040_e151359_d_n8;
        locals.var_t1_dn9 = assign99040_e151359_d_n9;
        locals.var_t1_dn10 = assign99040_e151359_d_n10;
        locals.var_t1_dn13 = assign99040_e151359_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign99050_e151370, assign99050_e151370_d_n0, assign99050_e151370_d_n2, assign99050_e151370_d_n4, assign99050_e151370_d_n5, assign99050_e151370_d_n6, assign99050_e151370_d_n7, assign99050_e151370_d_n8, assign99050_e151370_d_n9, assign99050_e151370_d_n10, assign99050_e151370_d_n13,) = {
    if ((locals.var_guard2290 != 0.0) && (locals.var_guard2291 == 0.0)) {
        let assign99050_e151366: f64 = (locals.var_czbssw * p.p527);
        let assign99050_e151368: f64 = (assign99050_e151366 / locals.var_pzbssw);
        (assign99050_e151368, ((((locals.var_czbssw_dn0 * p.p527) * locals.var_pzbssw) - (assign99050_e151366 * locals.var_pzbssw_dn0)) / (locals.var_pzbssw * locals.var_pzbssw)), ((((locals.var_czbssw_dn2 * p.p527) * locals.var_pzbssw) - (assign99050_e151366 * locals.var_pzbssw_dn2)) / (locals.var_pzbssw * locals.var_pzbssw)), ((((locals.var_czbssw_dn4 * p.p527) * locals.var_pzbssw) - (assign99050_e151366 * locals.var_pzbssw_dn4)) / (locals.var_pzbssw * locals.var_pzbssw)), ((((locals.var_czbssw_dn5 * p.p527) * locals.var_pzbssw) - (assign99050_e151366 * locals.var_pzbssw_dn5)) / (locals.var_pzbssw * locals.var_pzbssw)), ((((locals.var_czbssw_dn6 * p.p527) * locals.var_pzbssw) - (assign99050_e151366 * locals.var_pzbssw_dn6)) / (locals.var_pzbssw * locals.var_pzbssw)), ((((locals.var_czbssw_dn7 * p.p527) * locals.var_pzbssw) - (assign99050_e151366 * locals.var_pzbssw_dn7)) / (locals.var_pzbssw * locals.var_pzbssw)), ((((locals.var_czbssw_dn8 * p.p527) * locals.var_pzbssw) - (assign99050_e151366 * locals.var_pzbssw_dn8)) / (locals.var_pzbssw * locals.var_pzbssw)), ((((locals.var_czbssw_dn9 * p.p527) * locals.var_pzbssw) - (assign99050_e151366 * locals.var_pzbssw_dn9)) / (locals.var_pzbssw * locals.var_pzbssw)), ((((locals.var_czbssw_dn10 * p.p527) * locals.var_pzbssw) - (assign99050_e151366 * locals.var_pzbssw_dn10)) / (locals.var_pzbssw * locals.var_pzbssw)), ((((locals.var_czbssw_dn13 * p.p527) * locals.var_pzbssw) - (assign99050_e151366 * locals.var_pzbssw_dn13)) / (locals.var_pzbssw * locals.var_pzbssw)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign99050_e151370;
        locals.var_t2_dn0 = assign99050_e151370_d_n0;
        locals.var_t2_dn2 = assign99050_e151370_d_n2;
        locals.var_t2_dn4 = assign99050_e151370_d_n4;
        locals.var_t2_dn5 = assign99050_e151370_d_n5;
        locals.var_t2_dn6 = assign99050_e151370_d_n6;
        locals.var_t2_dn7 = assign99050_e151370_d_n7;
        locals.var_t2_dn8 = assign99050_e151370_d_n8;
        locals.var_t2_dn9 = assign99050_e151370_d_n9;
        locals.var_t2_dn10 = assign99050_e151370_d_n10;
        locals.var_t2_dn13 = assign99050_e151370_d_n13;
        locals.var_t2_rv = 0.0;

        let (assign99060_e151385, assign99060_e151385_d_n0, assign99060_e151385_d_n2, assign99060_e151385_d_n4, assign99060_e151385_d_n5, assign99060_e151385_d_n6, assign99060_e151385_d_n7, assign99060_e151385_d_n8, assign99060_e151385_d_n9, assign99060_e151385_d_n10, assign99060_e151385_d_n13,) = {
    if ((locals.var_guard2290 != 0.0) && (locals.var_guard2291 == 0.0)) {
        let assign99060_e151379: f64 = (locals.var_vbs_jct * 0.5);
        let assign99060_e151381: f64 = (assign99060_e151379 * locals.var_t2);
        let assign99060_e151382: f64 = (locals.var_t1 + assign99060_e151381);
        let assign99060_e151383: f64 = (locals.var_vbs_jct * assign99060_e151382);
        (assign99060_e151383, (locals.var_vbs_jct * (locals.var_t1_dn0 + (assign99060_e151379 * locals.var_t2_dn0))), ((locals.var_vbs_jct_dn2 * assign99060_e151382) + (locals.var_vbs_jct * (locals.var_t1_dn2 + (((locals.var_vbs_jct_dn2 * 0.5) * locals.var_t2) + (assign99060_e151379 * locals.var_t2_dn2))))), (locals.var_vbs_jct * (locals.var_t1_dn4 + (assign99060_e151379 * locals.var_t2_dn4))), (locals.var_vbs_jct * (locals.var_t1_dn5 + (assign99060_e151379 * locals.var_t2_dn5))), (locals.var_vbs_jct * (locals.var_t1_dn6 + (assign99060_e151379 * locals.var_t2_dn6))), (locals.var_vbs_jct * (locals.var_t1_dn7 + (assign99060_e151379 * locals.var_t2_dn7))), (locals.var_vbs_jct * (locals.var_t1_dn8 + (assign99060_e151379 * locals.var_t2_dn8))), (locals.var_vbs_jct * (locals.var_t1_dn9 + (assign99060_e151379 * locals.var_t2_dn9))), ((locals.var_vbs_jct_dn10 * assign99060_e151382) + (locals.var_vbs_jct * (locals.var_t1_dn10 + (((locals.var_vbs_jct_dn10 * 0.5) * locals.var_t2) + (assign99060_e151379 * locals.var_t2_dn10))))), (locals.var_vbs_jct * (locals.var_t1_dn13 + (assign99060_e151379 * locals.var_t2_dn13))),)
    } else {
        (locals.var_qbs_sws, locals.var_qbs_sws_dn0, locals.var_qbs_sws_dn2, locals.var_qbs_sws_dn4, locals.var_qbs_sws_dn5, locals.var_qbs_sws_dn6, locals.var_qbs_sws_dn7, locals.var_qbs_sws_dn8, locals.var_qbs_sws_dn9, locals.var_qbs_sws_dn10, locals.var_qbs_sws_dn13,)
    }
};
        locals.var_qbs_sws = assign99060_e151385;
        locals.var_qbs_sws_dn0 = assign99060_e151385_d_n0;
        locals.var_qbs_sws_dn2 = assign99060_e151385_d_n2;
        locals.var_qbs_sws_dn4 = assign99060_e151385_d_n4;
        locals.var_qbs_sws_dn5 = assign99060_e151385_d_n5;
        locals.var_qbs_sws_dn6 = assign99060_e151385_d_n6;
        locals.var_qbs_sws_dn7 = assign99060_e151385_d_n7;
        locals.var_qbs_sws_dn8 = assign99060_e151385_d_n8;
        locals.var_qbs_sws_dn9 = assign99060_e151385_d_n9;
        locals.var_qbs_sws_dn10 = assign99060_e151385_d_n10;
        locals.var_qbs_sws_dn13 = assign99060_e151385_d_n13;
        locals.var_qbs_sws_rv = 0.0;

        let (assign99080_e151401, assign99080_e151401_d_n0, assign99080_e151401_d_n2, assign99080_e151401_d_n4, assign99080_e151401_d_n5, assign99080_e151401_d_n6, assign99080_e151401_d_n7, assign99080_e151401_d_n8, assign99080_e151401_d_n9, assign99080_e151401_d_n10, assign99080_e151401_d_n13,) = {
    if (locals.var_guard2290 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qbs_sws, locals.var_qbs_sws_dn0, locals.var_qbs_sws_dn2, locals.var_qbs_sws_dn4, locals.var_qbs_sws_dn5, locals.var_qbs_sws_dn6, locals.var_qbs_sws_dn7, locals.var_qbs_sws_dn8, locals.var_qbs_sws_dn9, locals.var_qbs_sws_dn10, locals.var_qbs_sws_dn13,)
    }
};
        locals.var_qbs_sws = assign99080_e151401;
        locals.var_qbs_sws_dn0 = assign99080_e151401_d_n0;
        locals.var_qbs_sws_dn2 = assign99080_e151401_d_n2;
        locals.var_qbs_sws_dn4 = assign99080_e151401_d_n4;
        locals.var_qbs_sws_dn5 = assign99080_e151401_d_n5;
        locals.var_qbs_sws_dn6 = assign99080_e151401_d_n6;
        locals.var_qbs_sws_dn7 = assign99080_e151401_d_n7;
        locals.var_qbs_sws_dn8 = assign99080_e151401_d_n8;
        locals.var_qbs_sws_dn9 = assign99080_e151401_d_n9;
        locals.var_qbs_sws_dn10 = assign99080_e151401_d_n10;
        locals.var_qbs_sws_dn13 = assign99080_e151401_d_n13;
        locals.var_qbs_sws_rv = 0.0;

        let assign99100_e151409: f64 = if p.p48 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2293 = assign99100_e151409;
        locals.var_guard2293_rv = 0.0;

        let assign99110_e151412: f64 = if locals.var_czbsswg > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2294 = assign99110_e151412;
        locals.var_guard2294_rv = 0.0;

        let assign99120_e151415: f64 = if locals.var_vbsi_jct < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2295 = assign99120_e151415;
        locals.var_guard2295_rv = 0.0;

        let (assign99130_e151427, assign99130_e151427_d_n0, assign99130_e151427_d_n2, assign99130_e151427_d_n4, assign99130_e151427_d_n5, assign99130_e151427_d_n6, assign99130_e151427_d_n7, assign99130_e151427_d_n8, assign99130_e151427_d_n9, assign99130_e151427_d_n10, assign99130_e151427_d_n13,) = {
    if (((locals.var_guard2293 != 0.0) && (locals.var_guard2294 != 0.0)) && (locals.var_guard2295 != 0.0)) {
        let assign99130_e151424: f64 = (locals.var_vbsi_jct / locals.var_pzbsswg);
        let assign99130_e151425: f64 = (1.0 - assign99130_e151424);
        (assign99130_e151425, (-(-((locals.var_vbsi_jct * locals.var_pzbsswg_dn0) / (locals.var_pzbsswg * locals.var_pzbsswg)))), (-(-((locals.var_vbsi_jct * locals.var_pzbsswg_dn2) / (locals.var_pzbsswg * locals.var_pzbsswg)))), (-(-((locals.var_vbsi_jct * locals.var_pzbsswg_dn4) / (locals.var_pzbsswg * locals.var_pzbsswg)))), (-(-((locals.var_vbsi_jct * locals.var_pzbsswg_dn5) / (locals.var_pzbsswg * locals.var_pzbsswg)))), (-(-((locals.var_vbsi_jct * locals.var_pzbsswg_dn6) / (locals.var_pzbsswg * locals.var_pzbsswg)))), (-(((locals.var_vbsi_jct_dn7 * locals.var_pzbsswg) - (locals.var_vbsi_jct * locals.var_pzbsswg_dn7)) / (locals.var_pzbsswg * locals.var_pzbsswg))), (-(((locals.var_vbsi_jct_dn8 * locals.var_pzbsswg) - (locals.var_vbsi_jct * locals.var_pzbsswg_dn8)) / (locals.var_pzbsswg * locals.var_pzbsswg))), (-(-((locals.var_vbsi_jct * locals.var_pzbsswg_dn9) / (locals.var_pzbsswg * locals.var_pzbsswg)))), (-(-((locals.var_vbsi_jct * locals.var_pzbsswg_dn10) / (locals.var_pzbsswg * locals.var_pzbsswg)))), (-(-((locals.var_vbsi_jct * locals.var_pzbsswg_dn13) / (locals.var_pzbsswg * locals.var_pzbsswg)))),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    }
};
        locals.var_arg = assign99130_e151427;
        locals.var_arg_dn0 = assign99130_e151427_d_n0;
        locals.var_arg_dn2 = assign99130_e151427_d_n2;
        locals.var_arg_dn4 = assign99130_e151427_d_n4;
        locals.var_arg_dn5 = assign99130_e151427_d_n5;
        locals.var_arg_dn6 = assign99130_e151427_d_n6;
        locals.var_arg_dn7 = assign99130_e151427_d_n7;
        locals.var_arg_dn8 = assign99130_e151427_d_n8;
        locals.var_arg_dn9 = assign99130_e151427_d_n9;
        locals.var_arg_dn10 = assign99130_e151427_d_n10;
        locals.var_arg_dn13 = assign99130_e151427_d_n13;
        locals.var_arg_rv = 0.0;

        let assign99140_e151430: f64 = if p.p528 == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard2296 = assign99140_e151430;
        locals.var_guard2296_rv = 0.0;

        let (assign99150_e151443, assign99150_e151443_d_n0, assign99150_e151443_d_n2, assign99150_e151443_d_n4, assign99150_e151443_d_n5, assign99150_e151443_d_n6, assign99150_e151443_d_n7, assign99150_e151443_d_n8, assign99150_e151443_d_n9, assign99150_e151443_d_n10, assign99150_e151443_d_n13,) = {
    if ((((locals.var_guard2293 != 0.0) && (locals.var_guard2294 != 0.0)) && (locals.var_guard2295 != 0.0)) && (locals.var_guard2296 != 0.0)) {
        let assign99150_e151440: f64 = (locals.var_arg).sqrt();
        let assign99150_e151441: f64 = (1.0 / assign99150_e151440);
        (assign99150_e151441, (-((locals.var_arg_dn0 / (2.0 * assign99150_e151440)) / (assign99150_e151440 * assign99150_e151440))), (-((locals.var_arg_dn2 / (2.0 * assign99150_e151440)) / (assign99150_e151440 * assign99150_e151440))), (-((locals.var_arg_dn4 / (2.0 * assign99150_e151440)) / (assign99150_e151440 * assign99150_e151440))), (-((locals.var_arg_dn5 / (2.0 * assign99150_e151440)) / (assign99150_e151440 * assign99150_e151440))), (-((locals.var_arg_dn6 / (2.0 * assign99150_e151440)) / (assign99150_e151440 * assign99150_e151440))), (-((locals.var_arg_dn7 / (2.0 * assign99150_e151440)) / (assign99150_e151440 * assign99150_e151440))), (-((locals.var_arg_dn8 / (2.0 * assign99150_e151440)) / (assign99150_e151440 * assign99150_e151440))), (-((locals.var_arg_dn9 / (2.0 * assign99150_e151440)) / (assign99150_e151440 * assign99150_e151440))), (-((locals.var_arg_dn10 / (2.0 * assign99150_e151440)) / (assign99150_e151440 * assign99150_e151440))), (-((locals.var_arg_dn13 / (2.0 * assign99150_e151440)) / (assign99150_e151440 * assign99150_e151440))),)
    } else {
        (locals.var_sarg, locals.var_sarg_dn0, locals.var_sarg_dn2, locals.var_sarg_dn4, locals.var_sarg_dn5, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn8, locals.var_sarg_dn9, locals.var_sarg_dn10, locals.var_sarg_dn13,)
    }
};
        locals.var_sarg = assign99150_e151443;
        locals.var_sarg_dn0 = assign99150_e151443_d_n0;
        locals.var_sarg_dn2 = assign99150_e151443_d_n2;
        locals.var_sarg_dn4 = assign99150_e151443_d_n4;
        locals.var_sarg_dn5 = assign99150_e151443_d_n5;
        locals.var_sarg_dn6 = assign99150_e151443_d_n6;
        locals.var_sarg_dn7 = assign99150_e151443_d_n7;
        locals.var_sarg_dn8 = assign99150_e151443_d_n8;
        locals.var_sarg_dn9 = assign99150_e151443_d_n9;
        locals.var_sarg_dn10 = assign99150_e151443_d_n10;
        locals.var_sarg_dn13 = assign99150_e151443_d_n13;
        locals.var_sarg_rv = 0.0;

        let (assign99160_e151462, assign99160_e151462_d_n0, assign99160_e151462_d_n2, assign99160_e151462_d_n4, assign99160_e151462_d_n5, assign99160_e151462_d_n6, assign99160_e151462_d_n7, assign99160_e151462_d_n8, assign99160_e151462_d_n9, assign99160_e151462_d_n10, assign99160_e151462_d_n13,) = {
    if ((((locals.var_guard2293 != 0.0) && (locals.var_guard2294 != 0.0)) && (locals.var_guard2295 != 0.0)) && (locals.var_guard2296 == 0.0)) {
        let (assign99160_e151460, assign99160_e151460_d_n0, assign99160_e151460_d_n2, assign99160_e151460_d_n4, assign99160_e151460_d_n5, assign99160_e151460_d_n6, assign99160_e151460_d_n7, assign99160_e151460_d_n8, assign99160_e151460_d_n9, assign99160_e151460_d_n10, assign99160_e151460_d_n13,) = {
            if (locals.var_arg == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign99160_e151458: f64 = (-p.p528);
                let assign99160_e151459: f64 = (locals.var_arg).powf(assign99160_e151458);
                (assign99160_e151459, if 0.0 == 0.0 && ((assign99160_e151458) as f64).is_finite() && ((assign99160_e151458) as f64).fract() == 0.0 { if assign99160_e151458 == 0.0 { 0.0 } else { (assign99160_e151458 * ((locals.var_arg).powf(assign99160_e151458 - 1.0) * locals.var_arg_dn0)) } } else { (assign99160_e151459 * (assign99160_e151458 * (locals.var_arg_dn0 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign99160_e151458) as f64).is_finite() && ((assign99160_e151458) as f64).fract() == 0.0 { if assign99160_e151458 == 0.0 { 0.0 } else { (assign99160_e151458 * ((locals.var_arg).powf(assign99160_e151458 - 1.0) * locals.var_arg_dn2)) } } else { (assign99160_e151459 * (assign99160_e151458 * (locals.var_arg_dn2 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign99160_e151458) as f64).is_finite() && ((assign99160_e151458) as f64).fract() == 0.0 { if assign99160_e151458 == 0.0 { 0.0 } else { (assign99160_e151458 * ((locals.var_arg).powf(assign99160_e151458 - 1.0) * locals.var_arg_dn4)) } } else { (assign99160_e151459 * (assign99160_e151458 * (locals.var_arg_dn4 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign99160_e151458) as f64).is_finite() && ((assign99160_e151458) as f64).fract() == 0.0 { if assign99160_e151458 == 0.0 { 0.0 } else { (assign99160_e151458 * ((locals.var_arg).powf(assign99160_e151458 - 1.0) * locals.var_arg_dn5)) } } else { (assign99160_e151459 * (assign99160_e151458 * (locals.var_arg_dn5 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign99160_e151458) as f64).is_finite() && ((assign99160_e151458) as f64).fract() == 0.0 { if assign99160_e151458 == 0.0 { 0.0 } else { (assign99160_e151458 * ((locals.var_arg).powf(assign99160_e151458 - 1.0) * locals.var_arg_dn6)) } } else { (assign99160_e151459 * (assign99160_e151458 * (locals.var_arg_dn6 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign99160_e151458) as f64).is_finite() && ((assign99160_e151458) as f64).fract() == 0.0 { if assign99160_e151458 == 0.0 { 0.0 } else { (assign99160_e151458 * ((locals.var_arg).powf(assign99160_e151458 - 1.0) * locals.var_arg_dn7)) } } else { (assign99160_e151459 * (assign99160_e151458 * (locals.var_arg_dn7 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign99160_e151458) as f64).is_finite() && ((assign99160_e151458) as f64).fract() == 0.0 { if assign99160_e151458 == 0.0 { 0.0 } else { (assign99160_e151458 * ((locals.var_arg).powf(assign99160_e151458 - 1.0) * locals.var_arg_dn8)) } } else { (assign99160_e151459 * (assign99160_e151458 * (locals.var_arg_dn8 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign99160_e151458) as f64).is_finite() && ((assign99160_e151458) as f64).fract() == 0.0 { if assign99160_e151458 == 0.0 { 0.0 } else { (assign99160_e151458 * ((locals.var_arg).powf(assign99160_e151458 - 1.0) * locals.var_arg_dn9)) } } else { (assign99160_e151459 * (assign99160_e151458 * (locals.var_arg_dn9 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign99160_e151458) as f64).is_finite() && ((assign99160_e151458) as f64).fract() == 0.0 { if assign99160_e151458 == 0.0 { 0.0 } else { (assign99160_e151458 * ((locals.var_arg).powf(assign99160_e151458 - 1.0) * locals.var_arg_dn10)) } } else { (assign99160_e151459 * (assign99160_e151458 * (locals.var_arg_dn10 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign99160_e151458) as f64).is_finite() && ((assign99160_e151458) as f64).fract() == 0.0 { if assign99160_e151458 == 0.0 { 0.0 } else { (assign99160_e151458 * ((locals.var_arg).powf(assign99160_e151458 - 1.0) * locals.var_arg_dn13)) } } else { (assign99160_e151459 * (assign99160_e151458 * (locals.var_arg_dn13 / locals.var_arg))) },)
            }
        };
        (assign99160_e151460, assign99160_e151460_d_n0, assign99160_e151460_d_n2, assign99160_e151460_d_n4, assign99160_e151460_d_n5, assign99160_e151460_d_n6, assign99160_e151460_d_n7, assign99160_e151460_d_n8, assign99160_e151460_d_n9, assign99160_e151460_d_n10, assign99160_e151460_d_n13,)
    } else {
        (locals.var_sarg, locals.var_sarg_dn0, locals.var_sarg_dn2, locals.var_sarg_dn4, locals.var_sarg_dn5, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn8, locals.var_sarg_dn9, locals.var_sarg_dn10, locals.var_sarg_dn13,)
    }
};
        locals.var_sarg = assign99160_e151462;
        locals.var_sarg_dn0 = assign99160_e151462_d_n0;
        locals.var_sarg_dn2 = assign99160_e151462_d_n2;
        locals.var_sarg_dn4 = assign99160_e151462_d_n4;
        locals.var_sarg_dn5 = assign99160_e151462_d_n5;
        locals.var_sarg_dn6 = assign99160_e151462_d_n6;
        locals.var_sarg_dn7 = assign99160_e151462_d_n7;
        locals.var_sarg_dn8 = assign99160_e151462_d_n8;
        locals.var_sarg_dn9 = assign99160_e151462_d_n9;
        locals.var_sarg_dn10 = assign99160_e151462_d_n10;
        locals.var_sarg_dn13 = assign99160_e151462_d_n13;
        locals.var_sarg_rv = 0.0;

        let (assign99170_e151482, assign99170_e151482_d_n0, assign99170_e151482_d_n2, assign99170_e151482_d_n4, assign99170_e151482_d_n5, assign99170_e151482_d_n6, assign99170_e151482_d_n7, assign99170_e151482_d_n8, assign99170_e151482_d_n9, assign99170_e151482_d_n10, assign99170_e151482_d_n13,) = {
    if (((locals.var_guard2293 != 0.0) && (locals.var_guard2294 != 0.0)) && (locals.var_guard2295 != 0.0)) {
        let assign99170_e151470: f64 = (locals.var_pzbsswg * locals.var_czbsswg);
        let assign99170_e151474: f64 = (locals.var_arg * locals.var_sarg);
        let assign99170_e151475: f64 = (1.0 - assign99170_e151474);
        let assign99170_e151476: f64 = (assign99170_e151470 * assign99170_e151475);
        let assign99170_e151479: f64 = (1.0 - p.p528);
        let assign99170_e151480: f64 = (assign99170_e151476 / assign99170_e151479);
        (assign99170_e151480, (((((locals.var_pzbsswg_dn0 * locals.var_czbsswg) + (locals.var_pzbsswg * locals.var_czbsswg_dn0)) * assign99170_e151475) + (assign99170_e151470 * (-((locals.var_arg_dn0 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn0))))) / assign99170_e151479), (((((locals.var_pzbsswg_dn2 * locals.var_czbsswg) + (locals.var_pzbsswg * locals.var_czbsswg_dn2)) * assign99170_e151475) + (assign99170_e151470 * (-((locals.var_arg_dn2 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn2))))) / assign99170_e151479), (((((locals.var_pzbsswg_dn4 * locals.var_czbsswg) + (locals.var_pzbsswg * locals.var_czbsswg_dn4)) * assign99170_e151475) + (assign99170_e151470 * (-((locals.var_arg_dn4 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn4))))) / assign99170_e151479), (((((locals.var_pzbsswg_dn5 * locals.var_czbsswg) + (locals.var_pzbsswg * locals.var_czbsswg_dn5)) * assign99170_e151475) + (assign99170_e151470 * (-((locals.var_arg_dn5 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn5))))) / assign99170_e151479), (((((locals.var_pzbsswg_dn6 * locals.var_czbsswg) + (locals.var_pzbsswg * locals.var_czbsswg_dn6)) * assign99170_e151475) + (assign99170_e151470 * (-((locals.var_arg_dn6 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn6))))) / assign99170_e151479), (((((locals.var_pzbsswg_dn7 * locals.var_czbsswg) + (locals.var_pzbsswg * locals.var_czbsswg_dn7)) * assign99170_e151475) + (assign99170_e151470 * (-((locals.var_arg_dn7 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn7))))) / assign99170_e151479), (((((locals.var_pzbsswg_dn8 * locals.var_czbsswg) + (locals.var_pzbsswg * locals.var_czbsswg_dn8)) * assign99170_e151475) + (assign99170_e151470 * (-((locals.var_arg_dn8 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn8))))) / assign99170_e151479), (((((locals.var_pzbsswg_dn9 * locals.var_czbsswg) + (locals.var_pzbsswg * locals.var_czbsswg_dn9)) * assign99170_e151475) + (assign99170_e151470 * (-((locals.var_arg_dn9 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn9))))) / assign99170_e151479), (((((locals.var_pzbsswg_dn10 * locals.var_czbsswg) + (locals.var_pzbsswg * locals.var_czbsswg_dn10)) * assign99170_e151475) + (assign99170_e151470 * (-((locals.var_arg_dn10 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn10))))) / assign99170_e151479), (((((locals.var_pzbsswg_dn13 * locals.var_czbsswg) + (locals.var_pzbsswg * locals.var_czbsswg_dn13)) * assign99170_e151475) + (assign99170_e151470 * (-((locals.var_arg_dn13 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn13))))) / assign99170_e151479),)
    } else {
        (locals.var_qbs_swg, locals.var_qbs_swg_dn0, locals.var_qbs_swg_dn2, locals.var_qbs_swg_dn4, locals.var_qbs_swg_dn5, locals.var_qbs_swg_dn6, locals.var_qbs_swg_dn7, locals.var_qbs_swg_dn8, locals.var_qbs_swg_dn9, locals.var_qbs_swg_dn10, locals.var_qbs_swg_dn13,)
    }
};
        locals.var_qbs_swg = assign99170_e151482;
        locals.var_qbs_swg_dn0 = assign99170_e151482_d_n0;
        locals.var_qbs_swg_dn2 = assign99170_e151482_d_n2;
        locals.var_qbs_swg_dn4 = assign99170_e151482_d_n4;
        locals.var_qbs_swg_dn5 = assign99170_e151482_d_n5;
        locals.var_qbs_swg_dn6 = assign99170_e151482_d_n6;
        locals.var_qbs_swg_dn7 = assign99170_e151482_d_n7;
        locals.var_qbs_swg_dn8 = assign99170_e151482_d_n8;
        locals.var_qbs_swg_dn9 = assign99170_e151482_d_n9;
        locals.var_qbs_swg_dn10 = assign99170_e151482_d_n10;
        locals.var_qbs_swg_dn13 = assign99170_e151482_d_n13;
        locals.var_qbs_swg_rv = 0.0;

        let (assign99190_e151501, assign99190_e151501_d_n0, assign99190_e151501_d_n2, assign99190_e151501_d_n4, assign99190_e151501_d_n5, assign99190_e151501_d_n6, assign99190_e151501_d_n7, assign99190_e151501_d_n8, assign99190_e151501_d_n9, assign99190_e151501_d_n10, assign99190_e151501_d_n13,) = {
    if (((locals.var_guard2293 != 0.0) && (locals.var_guard2294 != 0.0)) && (locals.var_guard2295 == 0.0)) {
        (locals.var_czbsswg, locals.var_czbsswg_dn0, locals.var_czbsswg_dn2, locals.var_czbsswg_dn4, locals.var_czbsswg_dn5, locals.var_czbsswg_dn6, locals.var_czbsswg_dn7, locals.var_czbsswg_dn8, locals.var_czbsswg_dn9, locals.var_czbsswg_dn10, locals.var_czbsswg_dn13,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign99190_e151501;
        locals.var_t1_dn0 = assign99190_e151501_d_n0;
        locals.var_t1_dn2 = assign99190_e151501_d_n2;
        locals.var_t1_dn4 = assign99190_e151501_d_n4;
        locals.var_t1_dn5 = assign99190_e151501_d_n5;
        locals.var_t1_dn6 = assign99190_e151501_d_n6;
        locals.var_t1_dn7 = assign99190_e151501_d_n7;
        locals.var_t1_dn8 = assign99190_e151501_d_n8;
        locals.var_t1_dn9 = assign99190_e151501_d_n9;
        locals.var_t1_dn10 = assign99190_e151501_d_n10;
        locals.var_t1_dn13 = assign99190_e151501_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign99200_e151514, assign99200_e151514_d_n0, assign99200_e151514_d_n2, assign99200_e151514_d_n4, assign99200_e151514_d_n5, assign99200_e151514_d_n6, assign99200_e151514_d_n7, assign99200_e151514_d_n8, assign99200_e151514_d_n9, assign99200_e151514_d_n10, assign99200_e151514_d_n13,) = {
    if (((locals.var_guard2293 != 0.0) && (locals.var_guard2294 != 0.0)) && (locals.var_guard2295 == 0.0)) {
        let assign99200_e151510: f64 = (locals.var_czbsswg * p.p528);
        let assign99200_e151512: f64 = (assign99200_e151510 / locals.var_pzbsswg);
        (assign99200_e151512, ((((locals.var_czbsswg_dn0 * p.p528) * locals.var_pzbsswg) - (assign99200_e151510 * locals.var_pzbsswg_dn0)) / (locals.var_pzbsswg * locals.var_pzbsswg)), ((((locals.var_czbsswg_dn2 * p.p528) * locals.var_pzbsswg) - (assign99200_e151510 * locals.var_pzbsswg_dn2)) / (locals.var_pzbsswg * locals.var_pzbsswg)), ((((locals.var_czbsswg_dn4 * p.p528) * locals.var_pzbsswg) - (assign99200_e151510 * locals.var_pzbsswg_dn4)) / (locals.var_pzbsswg * locals.var_pzbsswg)), ((((locals.var_czbsswg_dn5 * p.p528) * locals.var_pzbsswg) - (assign99200_e151510 * locals.var_pzbsswg_dn5)) / (locals.var_pzbsswg * locals.var_pzbsswg)), ((((locals.var_czbsswg_dn6 * p.p528) * locals.var_pzbsswg) - (assign99200_e151510 * locals.var_pzbsswg_dn6)) / (locals.var_pzbsswg * locals.var_pzbsswg)), ((((locals.var_czbsswg_dn7 * p.p528) * locals.var_pzbsswg) - (assign99200_e151510 * locals.var_pzbsswg_dn7)) / (locals.var_pzbsswg * locals.var_pzbsswg)), ((((locals.var_czbsswg_dn8 * p.p528) * locals.var_pzbsswg) - (assign99200_e151510 * locals.var_pzbsswg_dn8)) / (locals.var_pzbsswg * locals.var_pzbsswg)), ((((locals.var_czbsswg_dn9 * p.p528) * locals.var_pzbsswg) - (assign99200_e151510 * locals.var_pzbsswg_dn9)) / (locals.var_pzbsswg * locals.var_pzbsswg)), ((((locals.var_czbsswg_dn10 * p.p528) * locals.var_pzbsswg) - (assign99200_e151510 * locals.var_pzbsswg_dn10)) / (locals.var_pzbsswg * locals.var_pzbsswg)), ((((locals.var_czbsswg_dn13 * p.p528) * locals.var_pzbsswg) - (assign99200_e151510 * locals.var_pzbsswg_dn13)) / (locals.var_pzbsswg * locals.var_pzbsswg)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign99200_e151514;
        locals.var_t2_dn0 = assign99200_e151514_d_n0;
        locals.var_t2_dn2 = assign99200_e151514_d_n2;
        locals.var_t2_dn4 = assign99200_e151514_d_n4;
        locals.var_t2_dn5 = assign99200_e151514_d_n5;
        locals.var_t2_dn6 = assign99200_e151514_d_n6;
        locals.var_t2_dn7 = assign99200_e151514_d_n7;
        locals.var_t2_dn8 = assign99200_e151514_d_n8;
        locals.var_t2_dn9 = assign99200_e151514_d_n9;
        locals.var_t2_dn10 = assign99200_e151514_d_n10;
        locals.var_t2_dn13 = assign99200_e151514_d_n13;
        locals.var_t2_rv = 0.0;

        let (assign99210_e151531, assign99210_e151531_d_n0, assign99210_e151531_d_n2, assign99210_e151531_d_n4, assign99210_e151531_d_n5, assign99210_e151531_d_n6, assign99210_e151531_d_n7, assign99210_e151531_d_n8, assign99210_e151531_d_n9, assign99210_e151531_d_n10, assign99210_e151531_d_n13,) = {
    if (((locals.var_guard2293 != 0.0) && (locals.var_guard2294 != 0.0)) && (locals.var_guard2295 == 0.0)) {
        let assign99210_e151525: f64 = (locals.var_vbsi_jct * 0.5);
        let assign99210_e151527: f64 = (assign99210_e151525 * locals.var_t2);
        let assign99210_e151528: f64 = (locals.var_t1 + assign99210_e151527);
        let assign99210_e151529: f64 = (locals.var_vbsi_jct * assign99210_e151528);
        (assign99210_e151529, (locals.var_vbsi_jct * (locals.var_t1_dn0 + (assign99210_e151525 * locals.var_t2_dn0))), (locals.var_vbsi_jct * (locals.var_t1_dn2 + (assign99210_e151525 * locals.var_t2_dn2))), (locals.var_vbsi_jct * (locals.var_t1_dn4 + (assign99210_e151525 * locals.var_t2_dn4))), (locals.var_vbsi_jct * (locals.var_t1_dn5 + (assign99210_e151525 * locals.var_t2_dn5))), (locals.var_vbsi_jct * (locals.var_t1_dn6 + (assign99210_e151525 * locals.var_t2_dn6))), ((locals.var_vbsi_jct_dn7 * assign99210_e151528) + (locals.var_vbsi_jct * (locals.var_t1_dn7 + (((locals.var_vbsi_jct_dn7 * 0.5) * locals.var_t2) + (assign99210_e151525 * locals.var_t2_dn7))))), ((locals.var_vbsi_jct_dn8 * assign99210_e151528) + (locals.var_vbsi_jct * (locals.var_t1_dn8 + (((locals.var_vbsi_jct_dn8 * 0.5) * locals.var_t2) + (assign99210_e151525 * locals.var_t2_dn8))))), (locals.var_vbsi_jct * (locals.var_t1_dn9 + (assign99210_e151525 * locals.var_t2_dn9))), (locals.var_vbsi_jct * (locals.var_t1_dn10 + (assign99210_e151525 * locals.var_t2_dn10))), (locals.var_vbsi_jct * (locals.var_t1_dn13 + (assign99210_e151525 * locals.var_t2_dn13))),)
    } else {
        (locals.var_qbs_swg, locals.var_qbs_swg_dn0, locals.var_qbs_swg_dn2, locals.var_qbs_swg_dn4, locals.var_qbs_swg_dn5, locals.var_qbs_swg_dn6, locals.var_qbs_swg_dn7, locals.var_qbs_swg_dn8, locals.var_qbs_swg_dn9, locals.var_qbs_swg_dn10, locals.var_qbs_swg_dn13,)
    }
};
        locals.var_qbs_swg = assign99210_e151531;
        locals.var_qbs_swg_dn0 = assign99210_e151531_d_n0;
        locals.var_qbs_swg_dn2 = assign99210_e151531_d_n2;
        locals.var_qbs_swg_dn4 = assign99210_e151531_d_n4;
        locals.var_qbs_swg_dn5 = assign99210_e151531_d_n5;
        locals.var_qbs_swg_dn6 = assign99210_e151531_d_n6;
        locals.var_qbs_swg_dn7 = assign99210_e151531_d_n7;
        locals.var_qbs_swg_dn8 = assign99210_e151531_d_n8;
        locals.var_qbs_swg_dn9 = assign99210_e151531_d_n9;
        locals.var_qbs_swg_dn10 = assign99210_e151531_d_n10;
        locals.var_qbs_swg_dn13 = assign99210_e151531_d_n13;
        locals.var_qbs_swg_rv = 0.0;

        let (assign99230_e151551, assign99230_e151551_d_n0, assign99230_e151551_d_n2, assign99230_e151551_d_n4, assign99230_e151551_d_n5, assign99230_e151551_d_n6, assign99230_e151551_d_n7, assign99230_e151551_d_n8, assign99230_e151551_d_n9, assign99230_e151551_d_n10, assign99230_e151551_d_n13,) = {
    if ((locals.var_guard2293 != 0.0) && (locals.var_guard2294 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qbs_swg, locals.var_qbs_swg_dn0, locals.var_qbs_swg_dn2, locals.var_qbs_swg_dn4, locals.var_qbs_swg_dn5, locals.var_qbs_swg_dn6, locals.var_qbs_swg_dn7, locals.var_qbs_swg_dn8, locals.var_qbs_swg_dn9, locals.var_qbs_swg_dn10, locals.var_qbs_swg_dn13,)
    }
};
        locals.var_qbs_swg = assign99230_e151551;
        locals.var_qbs_swg_dn0 = assign99230_e151551_d_n0;
        locals.var_qbs_swg_dn2 = assign99230_e151551_d_n2;
        locals.var_qbs_swg_dn4 = assign99230_e151551_d_n4;
        locals.var_qbs_swg_dn5 = assign99230_e151551_d_n5;
        locals.var_qbs_swg_dn6 = assign99230_e151551_d_n6;
        locals.var_qbs_swg_dn7 = assign99230_e151551_d_n7;
        locals.var_qbs_swg_dn8 = assign99230_e151551_d_n8;
        locals.var_qbs_swg_dn9 = assign99230_e151551_d_n9;
        locals.var_qbs_swg_dn10 = assign99230_e151551_d_n10;
        locals.var_qbs_swg_dn13 = assign99230_e151551_d_n13;
        locals.var_qbs_swg_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_370(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let assign99250_e151561: f64 = if locals.var_czbsswg > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2297 = assign99250_e151561;
        locals.var_guard2297_rv = 0.0;

        let assign99260_e151564: f64 = if locals.var_vbs_jct < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2298 = assign99260_e151564;
        locals.var_guard2298_rv = 0.0;

        let (assign99270_e151577, assign99270_e151577_d_n0, assign99270_e151577_d_n2, assign99270_e151577_d_n4, assign99270_e151577_d_n5, assign99270_e151577_d_n6, assign99270_e151577_d_n7, assign99270_e151577_d_n8, assign99270_e151577_d_n9, assign99270_e151577_d_n10, assign99270_e151577_d_n13,) = {
    if (((locals.var_guard2293 == 0.0) && (locals.var_guard2297 != 0.0)) && (locals.var_guard2298 != 0.0)) {
        let assign99270_e151574: f64 = (locals.var_vbs_jct / locals.var_pzbsswg);
        let assign99270_e151575: f64 = (1.0 - assign99270_e151574);
        (assign99270_e151575, (-(-((locals.var_vbs_jct * locals.var_pzbsswg_dn0) / (locals.var_pzbsswg * locals.var_pzbsswg)))), (-(((locals.var_vbs_jct_dn2 * locals.var_pzbsswg) - (locals.var_vbs_jct * locals.var_pzbsswg_dn2)) / (locals.var_pzbsswg * locals.var_pzbsswg))), (-(-((locals.var_vbs_jct * locals.var_pzbsswg_dn4) / (locals.var_pzbsswg * locals.var_pzbsswg)))), (-(-((locals.var_vbs_jct * locals.var_pzbsswg_dn5) / (locals.var_pzbsswg * locals.var_pzbsswg)))), (-(-((locals.var_vbs_jct * locals.var_pzbsswg_dn6) / (locals.var_pzbsswg * locals.var_pzbsswg)))), (-(-((locals.var_vbs_jct * locals.var_pzbsswg_dn7) / (locals.var_pzbsswg * locals.var_pzbsswg)))), (-(-((locals.var_vbs_jct * locals.var_pzbsswg_dn8) / (locals.var_pzbsswg * locals.var_pzbsswg)))), (-(-((locals.var_vbs_jct * locals.var_pzbsswg_dn9) / (locals.var_pzbsswg * locals.var_pzbsswg)))), (-(((locals.var_vbs_jct_dn10 * locals.var_pzbsswg) - (locals.var_vbs_jct * locals.var_pzbsswg_dn10)) / (locals.var_pzbsswg * locals.var_pzbsswg))), (-(-((locals.var_vbs_jct * locals.var_pzbsswg_dn13) / (locals.var_pzbsswg * locals.var_pzbsswg)))),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    }
};
        locals.var_arg = assign99270_e151577;
        locals.var_arg_dn0 = assign99270_e151577_d_n0;
        locals.var_arg_dn2 = assign99270_e151577_d_n2;
        locals.var_arg_dn4 = assign99270_e151577_d_n4;
        locals.var_arg_dn5 = assign99270_e151577_d_n5;
        locals.var_arg_dn6 = assign99270_e151577_d_n6;
        locals.var_arg_dn7 = assign99270_e151577_d_n7;
        locals.var_arg_dn8 = assign99270_e151577_d_n8;
        locals.var_arg_dn9 = assign99270_e151577_d_n9;
        locals.var_arg_dn10 = assign99270_e151577_d_n10;
        locals.var_arg_dn13 = assign99270_e151577_d_n13;
        locals.var_arg_rv = 0.0;

        let assign99280_e151580: f64 = if p.p528 == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard2299 = assign99280_e151580;
        locals.var_guard2299_rv = 0.0;

        let (assign99290_e151594, assign99290_e151594_d_n0, assign99290_e151594_d_n2, assign99290_e151594_d_n4, assign99290_e151594_d_n5, assign99290_e151594_d_n6, assign99290_e151594_d_n7, assign99290_e151594_d_n8, assign99290_e151594_d_n9, assign99290_e151594_d_n10, assign99290_e151594_d_n13,) = {
    if ((((locals.var_guard2293 == 0.0) && (locals.var_guard2297 != 0.0)) && (locals.var_guard2298 != 0.0)) && (locals.var_guard2299 != 0.0)) {
        let assign99290_e151591: f64 = (locals.var_arg).sqrt();
        let assign99290_e151592: f64 = (1.0 / assign99290_e151591);
        (assign99290_e151592, (-((locals.var_arg_dn0 / (2.0 * assign99290_e151591)) / (assign99290_e151591 * assign99290_e151591))), (-((locals.var_arg_dn2 / (2.0 * assign99290_e151591)) / (assign99290_e151591 * assign99290_e151591))), (-((locals.var_arg_dn4 / (2.0 * assign99290_e151591)) / (assign99290_e151591 * assign99290_e151591))), (-((locals.var_arg_dn5 / (2.0 * assign99290_e151591)) / (assign99290_e151591 * assign99290_e151591))), (-((locals.var_arg_dn6 / (2.0 * assign99290_e151591)) / (assign99290_e151591 * assign99290_e151591))), (-((locals.var_arg_dn7 / (2.0 * assign99290_e151591)) / (assign99290_e151591 * assign99290_e151591))), (-((locals.var_arg_dn8 / (2.0 * assign99290_e151591)) / (assign99290_e151591 * assign99290_e151591))), (-((locals.var_arg_dn9 / (2.0 * assign99290_e151591)) / (assign99290_e151591 * assign99290_e151591))), (-((locals.var_arg_dn10 / (2.0 * assign99290_e151591)) / (assign99290_e151591 * assign99290_e151591))), (-((locals.var_arg_dn13 / (2.0 * assign99290_e151591)) / (assign99290_e151591 * assign99290_e151591))),)
    } else {
        (locals.var_sarg, locals.var_sarg_dn0, locals.var_sarg_dn2, locals.var_sarg_dn4, locals.var_sarg_dn5, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn8, locals.var_sarg_dn9, locals.var_sarg_dn10, locals.var_sarg_dn13,)
    }
};
        locals.var_sarg = assign99290_e151594;
        locals.var_sarg_dn0 = assign99290_e151594_d_n0;
        locals.var_sarg_dn2 = assign99290_e151594_d_n2;
        locals.var_sarg_dn4 = assign99290_e151594_d_n4;
        locals.var_sarg_dn5 = assign99290_e151594_d_n5;
        locals.var_sarg_dn6 = assign99290_e151594_d_n6;
        locals.var_sarg_dn7 = assign99290_e151594_d_n7;
        locals.var_sarg_dn8 = assign99290_e151594_d_n8;
        locals.var_sarg_dn9 = assign99290_e151594_d_n9;
        locals.var_sarg_dn10 = assign99290_e151594_d_n10;
        locals.var_sarg_dn13 = assign99290_e151594_d_n13;
        locals.var_sarg_rv = 0.0;

        let (assign99300_e151614, assign99300_e151614_d_n0, assign99300_e151614_d_n2, assign99300_e151614_d_n4, assign99300_e151614_d_n5, assign99300_e151614_d_n6, assign99300_e151614_d_n7, assign99300_e151614_d_n8, assign99300_e151614_d_n9, assign99300_e151614_d_n10, assign99300_e151614_d_n13,) = {
    if ((((locals.var_guard2293 == 0.0) && (locals.var_guard2297 != 0.0)) && (locals.var_guard2298 != 0.0)) && (locals.var_guard2299 == 0.0)) {
        let (assign99300_e151612, assign99300_e151612_d_n0, assign99300_e151612_d_n2, assign99300_e151612_d_n4, assign99300_e151612_d_n5, assign99300_e151612_d_n6, assign99300_e151612_d_n7, assign99300_e151612_d_n8, assign99300_e151612_d_n9, assign99300_e151612_d_n10, assign99300_e151612_d_n13,) = {
            if (locals.var_arg == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign99300_e151610: f64 = (-p.p528);
                let assign99300_e151611: f64 = (locals.var_arg).powf(assign99300_e151610);
                (assign99300_e151611, if 0.0 == 0.0 && ((assign99300_e151610) as f64).is_finite() && ((assign99300_e151610) as f64).fract() == 0.0 { if assign99300_e151610 == 0.0 { 0.0 } else { (assign99300_e151610 * ((locals.var_arg).powf(assign99300_e151610 - 1.0) * locals.var_arg_dn0)) } } else { (assign99300_e151611 * (assign99300_e151610 * (locals.var_arg_dn0 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign99300_e151610) as f64).is_finite() && ((assign99300_e151610) as f64).fract() == 0.0 { if assign99300_e151610 == 0.0 { 0.0 } else { (assign99300_e151610 * ((locals.var_arg).powf(assign99300_e151610 - 1.0) * locals.var_arg_dn2)) } } else { (assign99300_e151611 * (assign99300_e151610 * (locals.var_arg_dn2 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign99300_e151610) as f64).is_finite() && ((assign99300_e151610) as f64).fract() == 0.0 { if assign99300_e151610 == 0.0 { 0.0 } else { (assign99300_e151610 * ((locals.var_arg).powf(assign99300_e151610 - 1.0) * locals.var_arg_dn4)) } } else { (assign99300_e151611 * (assign99300_e151610 * (locals.var_arg_dn4 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign99300_e151610) as f64).is_finite() && ((assign99300_e151610) as f64).fract() == 0.0 { if assign99300_e151610 == 0.0 { 0.0 } else { (assign99300_e151610 * ((locals.var_arg).powf(assign99300_e151610 - 1.0) * locals.var_arg_dn5)) } } else { (assign99300_e151611 * (assign99300_e151610 * (locals.var_arg_dn5 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign99300_e151610) as f64).is_finite() && ((assign99300_e151610) as f64).fract() == 0.0 { if assign99300_e151610 == 0.0 { 0.0 } else { (assign99300_e151610 * ((locals.var_arg).powf(assign99300_e151610 - 1.0) * locals.var_arg_dn6)) } } else { (assign99300_e151611 * (assign99300_e151610 * (locals.var_arg_dn6 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign99300_e151610) as f64).is_finite() && ((assign99300_e151610) as f64).fract() == 0.0 { if assign99300_e151610 == 0.0 { 0.0 } else { (assign99300_e151610 * ((locals.var_arg).powf(assign99300_e151610 - 1.0) * locals.var_arg_dn7)) } } else { (assign99300_e151611 * (assign99300_e151610 * (locals.var_arg_dn7 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign99300_e151610) as f64).is_finite() && ((assign99300_e151610) as f64).fract() == 0.0 { if assign99300_e151610 == 0.0 { 0.0 } else { (assign99300_e151610 * ((locals.var_arg).powf(assign99300_e151610 - 1.0) * locals.var_arg_dn8)) } } else { (assign99300_e151611 * (assign99300_e151610 * (locals.var_arg_dn8 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign99300_e151610) as f64).is_finite() && ((assign99300_e151610) as f64).fract() == 0.0 { if assign99300_e151610 == 0.0 { 0.0 } else { (assign99300_e151610 * ((locals.var_arg).powf(assign99300_e151610 - 1.0) * locals.var_arg_dn9)) } } else { (assign99300_e151611 * (assign99300_e151610 * (locals.var_arg_dn9 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign99300_e151610) as f64).is_finite() && ((assign99300_e151610) as f64).fract() == 0.0 { if assign99300_e151610 == 0.0 { 0.0 } else { (assign99300_e151610 * ((locals.var_arg).powf(assign99300_e151610 - 1.0) * locals.var_arg_dn10)) } } else { (assign99300_e151611 * (assign99300_e151610 * (locals.var_arg_dn10 / locals.var_arg))) }, if 0.0 == 0.0 && ((assign99300_e151610) as f64).is_finite() && ((assign99300_e151610) as f64).fract() == 0.0 { if assign99300_e151610 == 0.0 { 0.0 } else { (assign99300_e151610 * ((locals.var_arg).powf(assign99300_e151610 - 1.0) * locals.var_arg_dn13)) } } else { (assign99300_e151611 * (assign99300_e151610 * (locals.var_arg_dn13 / locals.var_arg))) },)
            }
        };
        (assign99300_e151612, assign99300_e151612_d_n0, assign99300_e151612_d_n2, assign99300_e151612_d_n4, assign99300_e151612_d_n5, assign99300_e151612_d_n6, assign99300_e151612_d_n7, assign99300_e151612_d_n8, assign99300_e151612_d_n9, assign99300_e151612_d_n10, assign99300_e151612_d_n13,)
    } else {
        (locals.var_sarg, locals.var_sarg_dn0, locals.var_sarg_dn2, locals.var_sarg_dn4, locals.var_sarg_dn5, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn8, locals.var_sarg_dn9, locals.var_sarg_dn10, locals.var_sarg_dn13,)
    }
};
        locals.var_sarg = assign99300_e151614;
        locals.var_sarg_dn0 = assign99300_e151614_d_n0;
        locals.var_sarg_dn2 = assign99300_e151614_d_n2;
        locals.var_sarg_dn4 = assign99300_e151614_d_n4;
        locals.var_sarg_dn5 = assign99300_e151614_d_n5;
        locals.var_sarg_dn6 = assign99300_e151614_d_n6;
        locals.var_sarg_dn7 = assign99300_e151614_d_n7;
        locals.var_sarg_dn8 = assign99300_e151614_d_n8;
        locals.var_sarg_dn9 = assign99300_e151614_d_n9;
        locals.var_sarg_dn10 = assign99300_e151614_d_n10;
        locals.var_sarg_dn13 = assign99300_e151614_d_n13;
        locals.var_sarg_rv = 0.0;

        let (assign99310_e151635, assign99310_e151635_d_n0, assign99310_e151635_d_n2, assign99310_e151635_d_n4, assign99310_e151635_d_n5, assign99310_e151635_d_n6, assign99310_e151635_d_n7, assign99310_e151635_d_n8, assign99310_e151635_d_n9, assign99310_e151635_d_n10, assign99310_e151635_d_n13,) = {
    if (((locals.var_guard2293 == 0.0) && (locals.var_guard2297 != 0.0)) && (locals.var_guard2298 != 0.0)) {
        let assign99310_e151623: f64 = (locals.var_pzbsswg * locals.var_czbsswg);
        let assign99310_e151627: f64 = (locals.var_arg * locals.var_sarg);
        let assign99310_e151628: f64 = (1.0 - assign99310_e151627);
        let assign99310_e151629: f64 = (assign99310_e151623 * assign99310_e151628);
        let assign99310_e151632: f64 = (1.0 - p.p528);
        let assign99310_e151633: f64 = (assign99310_e151629 / assign99310_e151632);
        (assign99310_e151633, (((((locals.var_pzbsswg_dn0 * locals.var_czbsswg) + (locals.var_pzbsswg * locals.var_czbsswg_dn0)) * assign99310_e151628) + (assign99310_e151623 * (-((locals.var_arg_dn0 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn0))))) / assign99310_e151632), (((((locals.var_pzbsswg_dn2 * locals.var_czbsswg) + (locals.var_pzbsswg * locals.var_czbsswg_dn2)) * assign99310_e151628) + (assign99310_e151623 * (-((locals.var_arg_dn2 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn2))))) / assign99310_e151632), (((((locals.var_pzbsswg_dn4 * locals.var_czbsswg) + (locals.var_pzbsswg * locals.var_czbsswg_dn4)) * assign99310_e151628) + (assign99310_e151623 * (-((locals.var_arg_dn4 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn4))))) / assign99310_e151632), (((((locals.var_pzbsswg_dn5 * locals.var_czbsswg) + (locals.var_pzbsswg * locals.var_czbsswg_dn5)) * assign99310_e151628) + (assign99310_e151623 * (-((locals.var_arg_dn5 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn5))))) / assign99310_e151632), (((((locals.var_pzbsswg_dn6 * locals.var_czbsswg) + (locals.var_pzbsswg * locals.var_czbsswg_dn6)) * assign99310_e151628) + (assign99310_e151623 * (-((locals.var_arg_dn6 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn6))))) / assign99310_e151632), (((((locals.var_pzbsswg_dn7 * locals.var_czbsswg) + (locals.var_pzbsswg * locals.var_czbsswg_dn7)) * assign99310_e151628) + (assign99310_e151623 * (-((locals.var_arg_dn7 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn7))))) / assign99310_e151632), (((((locals.var_pzbsswg_dn8 * locals.var_czbsswg) + (locals.var_pzbsswg * locals.var_czbsswg_dn8)) * assign99310_e151628) + (assign99310_e151623 * (-((locals.var_arg_dn8 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn8))))) / assign99310_e151632), (((((locals.var_pzbsswg_dn9 * locals.var_czbsswg) + (locals.var_pzbsswg * locals.var_czbsswg_dn9)) * assign99310_e151628) + (assign99310_e151623 * (-((locals.var_arg_dn9 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn9))))) / assign99310_e151632), (((((locals.var_pzbsswg_dn10 * locals.var_czbsswg) + (locals.var_pzbsswg * locals.var_czbsswg_dn10)) * assign99310_e151628) + (assign99310_e151623 * (-((locals.var_arg_dn10 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn10))))) / assign99310_e151632), (((((locals.var_pzbsswg_dn13 * locals.var_czbsswg) + (locals.var_pzbsswg * locals.var_czbsswg_dn13)) * assign99310_e151628) + (assign99310_e151623 * (-((locals.var_arg_dn13 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn13))))) / assign99310_e151632),)
    } else {
        (locals.var_qbs_swg, locals.var_qbs_swg_dn0, locals.var_qbs_swg_dn2, locals.var_qbs_swg_dn4, locals.var_qbs_swg_dn5, locals.var_qbs_swg_dn6, locals.var_qbs_swg_dn7, locals.var_qbs_swg_dn8, locals.var_qbs_swg_dn9, locals.var_qbs_swg_dn10, locals.var_qbs_swg_dn13,)
    }
};
        locals.var_qbs_swg = assign99310_e151635;
        locals.var_qbs_swg_dn0 = assign99310_e151635_d_n0;
        locals.var_qbs_swg_dn2 = assign99310_e151635_d_n2;
        locals.var_qbs_swg_dn4 = assign99310_e151635_d_n4;
        locals.var_qbs_swg_dn5 = assign99310_e151635_d_n5;
        locals.var_qbs_swg_dn6 = assign99310_e151635_d_n6;
        locals.var_qbs_swg_dn7 = assign99310_e151635_d_n7;
        locals.var_qbs_swg_dn8 = assign99310_e151635_d_n8;
        locals.var_qbs_swg_dn9 = assign99310_e151635_d_n9;
        locals.var_qbs_swg_dn10 = assign99310_e151635_d_n10;
        locals.var_qbs_swg_dn13 = assign99310_e151635_d_n13;
        locals.var_qbs_swg_rv = 0.0;

        let (assign99330_e151656, assign99330_e151656_d_n0, assign99330_e151656_d_n2, assign99330_e151656_d_n4, assign99330_e151656_d_n5, assign99330_e151656_d_n6, assign99330_e151656_d_n7, assign99330_e151656_d_n8, assign99330_e151656_d_n9, assign99330_e151656_d_n10, assign99330_e151656_d_n13,) = {
    if (((locals.var_guard2293 == 0.0) && (locals.var_guard2297 != 0.0)) && (locals.var_guard2298 == 0.0)) {
        (locals.var_czbsswg, locals.var_czbsswg_dn0, locals.var_czbsswg_dn2, locals.var_czbsswg_dn4, locals.var_czbsswg_dn5, locals.var_czbsswg_dn6, locals.var_czbsswg_dn7, locals.var_czbsswg_dn8, locals.var_czbsswg_dn9, locals.var_czbsswg_dn10, locals.var_czbsswg_dn13,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign99330_e151656;
        locals.var_t1_dn0 = assign99330_e151656_d_n0;
        locals.var_t1_dn2 = assign99330_e151656_d_n2;
        locals.var_t1_dn4 = assign99330_e151656_d_n4;
        locals.var_t1_dn5 = assign99330_e151656_d_n5;
        locals.var_t1_dn6 = assign99330_e151656_d_n6;
        locals.var_t1_dn7 = assign99330_e151656_d_n7;
        locals.var_t1_dn8 = assign99330_e151656_d_n8;
        locals.var_t1_dn9 = assign99330_e151656_d_n9;
        locals.var_t1_dn10 = assign99330_e151656_d_n10;
        locals.var_t1_dn13 = assign99330_e151656_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign99340_e151670, assign99340_e151670_d_n0, assign99340_e151670_d_n2, assign99340_e151670_d_n4, assign99340_e151670_d_n5, assign99340_e151670_d_n6, assign99340_e151670_d_n7, assign99340_e151670_d_n8, assign99340_e151670_d_n9, assign99340_e151670_d_n10, assign99340_e151670_d_n13,) = {
    if (((locals.var_guard2293 == 0.0) && (locals.var_guard2297 != 0.0)) && (locals.var_guard2298 == 0.0)) {
        let assign99340_e151666: f64 = (locals.var_czbsswg * p.p528);
        let assign99340_e151668: f64 = (assign99340_e151666 / locals.var_pzbsswg);
        (assign99340_e151668, ((((locals.var_czbsswg_dn0 * p.p528) * locals.var_pzbsswg) - (assign99340_e151666 * locals.var_pzbsswg_dn0)) / (locals.var_pzbsswg * locals.var_pzbsswg)), ((((locals.var_czbsswg_dn2 * p.p528) * locals.var_pzbsswg) - (assign99340_e151666 * locals.var_pzbsswg_dn2)) / (locals.var_pzbsswg * locals.var_pzbsswg)), ((((locals.var_czbsswg_dn4 * p.p528) * locals.var_pzbsswg) - (assign99340_e151666 * locals.var_pzbsswg_dn4)) / (locals.var_pzbsswg * locals.var_pzbsswg)), ((((locals.var_czbsswg_dn5 * p.p528) * locals.var_pzbsswg) - (assign99340_e151666 * locals.var_pzbsswg_dn5)) / (locals.var_pzbsswg * locals.var_pzbsswg)), ((((locals.var_czbsswg_dn6 * p.p528) * locals.var_pzbsswg) - (assign99340_e151666 * locals.var_pzbsswg_dn6)) / (locals.var_pzbsswg * locals.var_pzbsswg)), ((((locals.var_czbsswg_dn7 * p.p528) * locals.var_pzbsswg) - (assign99340_e151666 * locals.var_pzbsswg_dn7)) / (locals.var_pzbsswg * locals.var_pzbsswg)), ((((locals.var_czbsswg_dn8 * p.p528) * locals.var_pzbsswg) - (assign99340_e151666 * locals.var_pzbsswg_dn8)) / (locals.var_pzbsswg * locals.var_pzbsswg)), ((((locals.var_czbsswg_dn9 * p.p528) * locals.var_pzbsswg) - (assign99340_e151666 * locals.var_pzbsswg_dn9)) / (locals.var_pzbsswg * locals.var_pzbsswg)), ((((locals.var_czbsswg_dn10 * p.p528) * locals.var_pzbsswg) - (assign99340_e151666 * locals.var_pzbsswg_dn10)) / (locals.var_pzbsswg * locals.var_pzbsswg)), ((((locals.var_czbsswg_dn13 * p.p528) * locals.var_pzbsswg) - (assign99340_e151666 * locals.var_pzbsswg_dn13)) / (locals.var_pzbsswg * locals.var_pzbsswg)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign99340_e151670;
        locals.var_t2_dn0 = assign99340_e151670_d_n0;
        locals.var_t2_dn2 = assign99340_e151670_d_n2;
        locals.var_t2_dn4 = assign99340_e151670_d_n4;
        locals.var_t2_dn5 = assign99340_e151670_d_n5;
        locals.var_t2_dn6 = assign99340_e151670_d_n6;
        locals.var_t2_dn7 = assign99340_e151670_d_n7;
        locals.var_t2_dn8 = assign99340_e151670_d_n8;
        locals.var_t2_dn9 = assign99340_e151670_d_n9;
        locals.var_t2_dn10 = assign99340_e151670_d_n10;
        locals.var_t2_dn13 = assign99340_e151670_d_n13;
        locals.var_t2_rv = 0.0;

        let (assign99350_e151688, assign99350_e151688_d_n0, assign99350_e151688_d_n2, assign99350_e151688_d_n4, assign99350_e151688_d_n5, assign99350_e151688_d_n6, assign99350_e151688_d_n7, assign99350_e151688_d_n8, assign99350_e151688_d_n9, assign99350_e151688_d_n10, assign99350_e151688_d_n13,) = {
    if (((locals.var_guard2293 == 0.0) && (locals.var_guard2297 != 0.0)) && (locals.var_guard2298 == 0.0)) {
        let assign99350_e151682: f64 = (locals.var_vbs_jct * 0.5);
        let assign99350_e151684: f64 = (assign99350_e151682 * locals.var_t2);
        let assign99350_e151685: f64 = (locals.var_t1 + assign99350_e151684);
        let assign99350_e151686: f64 = (locals.var_vbs_jct * assign99350_e151685);
        (assign99350_e151686, (locals.var_vbs_jct * (locals.var_t1_dn0 + (assign99350_e151682 * locals.var_t2_dn0))), ((locals.var_vbs_jct_dn2 * assign99350_e151685) + (locals.var_vbs_jct * (locals.var_t1_dn2 + (((locals.var_vbs_jct_dn2 * 0.5) * locals.var_t2) + (assign99350_e151682 * locals.var_t2_dn2))))), (locals.var_vbs_jct * (locals.var_t1_dn4 + (assign99350_e151682 * locals.var_t2_dn4))), (locals.var_vbs_jct * (locals.var_t1_dn5 + (assign99350_e151682 * locals.var_t2_dn5))), (locals.var_vbs_jct * (locals.var_t1_dn6 + (assign99350_e151682 * locals.var_t2_dn6))), (locals.var_vbs_jct * (locals.var_t1_dn7 + (assign99350_e151682 * locals.var_t2_dn7))), (locals.var_vbs_jct * (locals.var_t1_dn8 + (assign99350_e151682 * locals.var_t2_dn8))), (locals.var_vbs_jct * (locals.var_t1_dn9 + (assign99350_e151682 * locals.var_t2_dn9))), ((locals.var_vbs_jct_dn10 * assign99350_e151685) + (locals.var_vbs_jct * (locals.var_t1_dn10 + (((locals.var_vbs_jct_dn10 * 0.5) * locals.var_t2) + (assign99350_e151682 * locals.var_t2_dn10))))), (locals.var_vbs_jct * (locals.var_t1_dn13 + (assign99350_e151682 * locals.var_t2_dn13))),)
    } else {
        (locals.var_qbs_swg, locals.var_qbs_swg_dn0, locals.var_qbs_swg_dn2, locals.var_qbs_swg_dn4, locals.var_qbs_swg_dn5, locals.var_qbs_swg_dn6, locals.var_qbs_swg_dn7, locals.var_qbs_swg_dn8, locals.var_qbs_swg_dn9, locals.var_qbs_swg_dn10, locals.var_qbs_swg_dn13,)
    }
};
        locals.var_qbs_swg = assign99350_e151688;
        locals.var_qbs_swg_dn0 = assign99350_e151688_d_n0;
        locals.var_qbs_swg_dn2 = assign99350_e151688_d_n2;
        locals.var_qbs_swg_dn4 = assign99350_e151688_d_n4;
        locals.var_qbs_swg_dn5 = assign99350_e151688_d_n5;
        locals.var_qbs_swg_dn6 = assign99350_e151688_d_n6;
        locals.var_qbs_swg_dn7 = assign99350_e151688_d_n7;
        locals.var_qbs_swg_dn8 = assign99350_e151688_d_n8;
        locals.var_qbs_swg_dn9 = assign99350_e151688_d_n9;
        locals.var_qbs_swg_dn10 = assign99350_e151688_d_n10;
        locals.var_qbs_swg_dn13 = assign99350_e151688_d_n13;
        locals.var_qbs_swg_rv = 0.0;

        let (assign99370_e151710, assign99370_e151710_d_n0, assign99370_e151710_d_n2, assign99370_e151710_d_n4, assign99370_e151710_d_n5, assign99370_e151710_d_n6, assign99370_e151710_d_n7, assign99370_e151710_d_n8, assign99370_e151710_d_n9, assign99370_e151710_d_n10, assign99370_e151710_d_n13,) = {
    if ((locals.var_guard2293 == 0.0) && (locals.var_guard2297 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qbs_swg, locals.var_qbs_swg_dn0, locals.var_qbs_swg_dn2, locals.var_qbs_swg_dn4, locals.var_qbs_swg_dn5, locals.var_qbs_swg_dn6, locals.var_qbs_swg_dn7, locals.var_qbs_swg_dn8, locals.var_qbs_swg_dn9, locals.var_qbs_swg_dn10, locals.var_qbs_swg_dn13,)
    }
};
        locals.var_qbs_swg = assign99370_e151710;
        locals.var_qbs_swg_dn0 = assign99370_e151710_d_n0;
        locals.var_qbs_swg_dn2 = assign99370_e151710_d_n2;
        locals.var_qbs_swg_dn4 = assign99370_e151710_d_n4;
        locals.var_qbs_swg_dn5 = assign99370_e151710_d_n5;
        locals.var_qbs_swg_dn6 = assign99370_e151710_d_n6;
        locals.var_qbs_swg_dn7 = assign99370_e151710_d_n7;
        locals.var_qbs_swg_dn8 = assign99370_e151710_d_n8;
        locals.var_qbs_swg_dn9 = assign99370_e151710_d_n9;
        locals.var_qbs_swg_dn10 = assign99370_e151710_d_n10;
        locals.var_qbs_swg_dn13 = assign99370_e151710_d_n13;
        locals.var_qbs_swg_rv = 0.0;

        let assign99410_e151731: f64 = if p.p48 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2300 = assign99410_e151731;
        locals.var_guard2300_rv = 0.0;

        let (assign99440_e151751, assign99440_e151751_d_n0, assign99440_e151751_d_n2, assign99440_e151751_d_n4, assign99440_e151751_d_n5, assign99440_e151751_d_n6, assign99440_e151751_d_n7, assign99440_e151751_d_n8, assign99440_e151751_d_n9, assign99440_e151751_d_n10, assign99440_e151751_d_n13,) = {
    if (locals.var_guard2300 != 0.0) {
        let assign99440_e151748: f64 = (locals.var_qbs_btm + locals.var_qbs_sws);
        let assign99440_e151749: f64 = (locals.var_mfactor * assign99440_e151748);
        (assign99440_e151749, (locals.var_mfactor * (locals.var_qbs_btm_dn0 + locals.var_qbs_sws_dn0)), (locals.var_mfactor * (locals.var_qbs_btm_dn2 + locals.var_qbs_sws_dn2)), (locals.var_mfactor * (locals.var_qbs_btm_dn4 + locals.var_qbs_sws_dn4)), (locals.var_mfactor * (locals.var_qbs_btm_dn5 + locals.var_qbs_sws_dn5)), (locals.var_mfactor * (locals.var_qbs_btm_dn6 + locals.var_qbs_sws_dn6)), (locals.var_mfactor * (locals.var_qbs_btm_dn7 + locals.var_qbs_sws_dn7)), (locals.var_mfactor * (locals.var_qbs_btm_dn8 + locals.var_qbs_sws_dn8)), (locals.var_mfactor * (locals.var_qbs_btm_dn9 + locals.var_qbs_sws_dn9)), (locals.var_mfactor * (locals.var_qbs_btm_dn10 + locals.var_qbs_sws_dn10)), (locals.var_mfactor * (locals.var_qbs_btm_dn13 + locals.var_qbs_sws_dn13)),)
    } else {
        (locals.var_qbs, locals.var_qbs_dn0, locals.var_qbs_dn2, locals.var_qbs_dn4, locals.var_qbs_dn5, locals.var_qbs_dn6, locals.var_qbs_dn7, locals.var_qbs_dn8, locals.var_qbs_dn9, locals.var_qbs_dn10, locals.var_qbs_dn13,)
    }
};
        locals.var_qbs = assign99440_e151751;
        locals.var_qbs_dn0 = assign99440_e151751_d_n0;
        locals.var_qbs_dn2 = assign99440_e151751_d_n2;
        locals.var_qbs_dn4 = assign99440_e151751_d_n4;
        locals.var_qbs_dn5 = assign99440_e151751_d_n5;
        locals.var_qbs_dn6 = assign99440_e151751_d_n6;
        locals.var_qbs_dn7 = assign99440_e151751_d_n7;
        locals.var_qbs_dn8 = assign99440_e151751_d_n8;
        locals.var_qbs_dn9 = assign99440_e151751_d_n9;
        locals.var_qbs_dn10 = assign99440_e151751_d_n10;
        locals.var_qbs_dn13 = assign99440_e151751_d_n13;
        locals.var_qbs_rv = 0.0;

        let (assign99450_e151759, assign99450_e151759_d_n0, assign99450_e151759_d_n2, assign99450_e151759_d_n4, assign99450_e151759_d_n5, assign99450_e151759_d_n6, assign99450_e151759_d_n7, assign99450_e151759_d_n8, assign99450_e151759_d_n9, assign99450_e151759_d_n10, assign99450_e151759_d_n13, assign99450_e151759_d_n15, assign99450_e151759_d_n16, assign99450_e151759_d_n17,) = {
    if (locals.var_guard2300 != 0.0) {
        let assign99450_e151756: f64 = (locals.var_qbd_btm + locals.var_qbd_sws);
        let assign99450_e151757: f64 = (locals.var_mfactor * assign99450_e151756);
        (assign99450_e151757, (locals.var_mfactor * (locals.var_qbd_btm_dn0 + locals.var_qbd_sws_dn0)), (locals.var_mfactor * (locals.var_qbd_btm_dn2 + locals.var_qbd_sws_dn2)), (locals.var_mfactor * (locals.var_qbd_btm_dn4 + locals.var_qbd_sws_dn4)), (locals.var_mfactor * (locals.var_qbd_btm_dn5 + locals.var_qbd_sws_dn5)), (locals.var_mfactor * (locals.var_qbd_btm_dn6 + locals.var_qbd_sws_dn6)), (locals.var_mfactor * (locals.var_qbd_btm_dn7 + locals.var_qbd_sws_dn7)), (locals.var_mfactor * (locals.var_qbd_btm_dn8 + locals.var_qbd_sws_dn8)), (locals.var_mfactor * (locals.var_qbd_btm_dn9 + locals.var_qbd_sws_dn9)), (locals.var_mfactor * (locals.var_qbd_btm_dn10 + locals.var_qbd_sws_dn10)), (locals.var_mfactor * (locals.var_qbd_btm_dn13 + locals.var_qbd_sws_dn13)), 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qbd, locals.var_qbd_dn0, locals.var_qbd_dn2, locals.var_qbd_dn4, locals.var_qbd_dn5, locals.var_qbd_dn6, locals.var_qbd_dn7, locals.var_qbd_dn8, locals.var_qbd_dn9, locals.var_qbd_dn10, locals.var_qbd_dn13, locals.var_qbd_dn15, locals.var_qbd_dn16, locals.var_qbd_dn17,)
    }
};
        locals.var_qbd = assign99450_e151759;
        locals.var_qbd_dn0 = assign99450_e151759_d_n0;
        locals.var_qbd_dn2 = assign99450_e151759_d_n2;
        locals.var_qbd_dn4 = assign99450_e151759_d_n4;
        locals.var_qbd_dn5 = assign99450_e151759_d_n5;
        locals.var_qbd_dn6 = assign99450_e151759_d_n6;
        locals.var_qbd_dn7 = assign99450_e151759_d_n7;
        locals.var_qbd_dn8 = assign99450_e151759_d_n8;
        locals.var_qbd_dn9 = assign99450_e151759_d_n9;
        locals.var_qbd_dn10 = assign99450_e151759_d_n10;
        locals.var_qbd_dn13 = assign99450_e151759_d_n13;
        locals.var_qbd_dn15 = assign99450_e151759_d_n15;
        locals.var_qbd_dn16 = assign99450_e151759_d_n16;
        locals.var_qbd_dn17 = assign99450_e151759_d_n17;
        locals.var_qbd_rv = 0.0;

        let (assign99460_e151765, assign99460_e151765_d_n0, assign99460_e151765_d_n2, assign99460_e151765_d_n4, assign99460_e151765_d_n5, assign99460_e151765_d_n6, assign99460_e151765_d_n7, assign99460_e151765_d_n8, assign99460_e151765_d_n9, assign99460_e151765_d_n10, assign99460_e151765_d_n13,) = {
    if (locals.var_guard2300 != 0.0) {
        let assign99460_e151763: f64 = (locals.var_mfactor * locals.var_qbs_swg);
        (assign99460_e151763, (locals.var_mfactor * locals.var_qbs_swg_dn0), (locals.var_mfactor * locals.var_qbs_swg_dn2), (locals.var_mfactor * locals.var_qbs_swg_dn4), (locals.var_mfactor * locals.var_qbs_swg_dn5), (locals.var_mfactor * locals.var_qbs_swg_dn6), (locals.var_mfactor * locals.var_qbs_swg_dn7), (locals.var_mfactor * locals.var_qbs_swg_dn8), (locals.var_mfactor * locals.var_qbs_swg_dn9), (locals.var_mfactor * locals.var_qbs_swg_dn10), (locals.var_mfactor * locals.var_qbs_swg_dn13),)
    } else {
        (locals.var_qbsi, locals.var_qbsi_dn0, locals.var_qbsi_dn2, locals.var_qbsi_dn4, locals.var_qbsi_dn5, locals.var_qbsi_dn6, locals.var_qbsi_dn7, locals.var_qbsi_dn8, locals.var_qbsi_dn9, locals.var_qbsi_dn10, locals.var_qbsi_dn13,)
    }
};
        locals.var_qbsi = assign99460_e151765;
        locals.var_qbsi_dn0 = assign99460_e151765_d_n0;
        locals.var_qbsi_dn2 = assign99460_e151765_d_n2;
        locals.var_qbsi_dn4 = assign99460_e151765_d_n4;
        locals.var_qbsi_dn5 = assign99460_e151765_d_n5;
        locals.var_qbsi_dn6 = assign99460_e151765_d_n6;
        locals.var_qbsi_dn7 = assign99460_e151765_d_n7;
        locals.var_qbsi_dn8 = assign99460_e151765_d_n8;
        locals.var_qbsi_dn9 = assign99460_e151765_d_n9;
        locals.var_qbsi_dn10 = assign99460_e151765_d_n10;
        locals.var_qbsi_dn13 = assign99460_e151765_d_n13;
        locals.var_qbsi_rv = 0.0;

        let (assign99470_e151771, assign99470_e151771_d_n0, assign99470_e151771_d_n2, assign99470_e151771_d_n4, assign99470_e151771_d_n5, assign99470_e151771_d_n6, assign99470_e151771_d_n7, assign99470_e151771_d_n8, assign99470_e151771_d_n9, assign99470_e151771_d_n10, assign99470_e151771_d_n13,) = {
    if (locals.var_guard2300 != 0.0) {
        let assign99470_e151769: f64 = (locals.var_mfactor * locals.var_qbd_swg);
        (assign99470_e151769, (locals.var_mfactor * locals.var_qbd_swg_dn0), (locals.var_mfactor * locals.var_qbd_swg_dn2), (locals.var_mfactor * locals.var_qbd_swg_dn4), (locals.var_mfactor * locals.var_qbd_swg_dn5), (locals.var_mfactor * locals.var_qbd_swg_dn6), (locals.var_mfactor * locals.var_qbd_swg_dn7), (locals.var_mfactor * locals.var_qbd_swg_dn8), (locals.var_mfactor * locals.var_qbd_swg_dn9), (locals.var_mfactor * locals.var_qbd_swg_dn10), (locals.var_mfactor * locals.var_qbd_swg_dn13),)
    } else {
        (locals.var_qbdi, locals.var_qbdi_dn0, locals.var_qbdi_dn2, locals.var_qbdi_dn4, locals.var_qbdi_dn5, locals.var_qbdi_dn6, locals.var_qbdi_dn7, locals.var_qbdi_dn8, locals.var_qbdi_dn9, locals.var_qbdi_dn10, locals.var_qbdi_dn13,)
    }
};
        locals.var_qbdi = assign99470_e151771;
        locals.var_qbdi_dn0 = assign99470_e151771_d_n0;
        locals.var_qbdi_dn2 = assign99470_e151771_d_n2;
        locals.var_qbdi_dn4 = assign99470_e151771_d_n4;
        locals.var_qbdi_dn5 = assign99470_e151771_d_n5;
        locals.var_qbdi_dn6 = assign99470_e151771_d_n6;
        locals.var_qbdi_dn7 = assign99470_e151771_d_n7;
        locals.var_qbdi_dn8 = assign99470_e151771_d_n8;
        locals.var_qbdi_dn9 = assign99470_e151771_d_n9;
        locals.var_qbdi_dn10 = assign99470_e151771_d_n10;
        locals.var_qbdi_dn13 = assign99470_e151771_d_n13;
        locals.var_qbdi_rv = 0.0;

        let (assign99540_e151820, assign99540_e151820_d_n0, assign99540_e151820_d_n2, assign99540_e151820_d_n4, assign99540_e151820_d_n5, assign99540_e151820_d_n6, assign99540_e151820_d_n7, assign99540_e151820_d_n8, assign99540_e151820_d_n9, assign99540_e151820_d_n10, assign99540_e151820_d_n13,) = {
    if (locals.var_guard2300 == 0.0) {
        let assign99540_e151815: f64 = (locals.var_qbs_btm + locals.var_qbs_sws);
        let assign99540_e151817: f64 = (assign99540_e151815 + locals.var_qbs_swg);
        let assign99540_e151818: f64 = (locals.var_mfactor * assign99540_e151817);
        (assign99540_e151818, (locals.var_mfactor * ((locals.var_qbs_btm_dn0 + locals.var_qbs_sws_dn0) + locals.var_qbs_swg_dn0)), (locals.var_mfactor * ((locals.var_qbs_btm_dn2 + locals.var_qbs_sws_dn2) + locals.var_qbs_swg_dn2)), (locals.var_mfactor * ((locals.var_qbs_btm_dn4 + locals.var_qbs_sws_dn4) + locals.var_qbs_swg_dn4)), (locals.var_mfactor * ((locals.var_qbs_btm_dn5 + locals.var_qbs_sws_dn5) + locals.var_qbs_swg_dn5)), (locals.var_mfactor * ((locals.var_qbs_btm_dn6 + locals.var_qbs_sws_dn6) + locals.var_qbs_swg_dn6)), (locals.var_mfactor * ((locals.var_qbs_btm_dn7 + locals.var_qbs_sws_dn7) + locals.var_qbs_swg_dn7)), (locals.var_mfactor * ((locals.var_qbs_btm_dn8 + locals.var_qbs_sws_dn8) + locals.var_qbs_swg_dn8)), (locals.var_mfactor * ((locals.var_qbs_btm_dn9 + locals.var_qbs_sws_dn9) + locals.var_qbs_swg_dn9)), (locals.var_mfactor * ((locals.var_qbs_btm_dn10 + locals.var_qbs_sws_dn10) + locals.var_qbs_swg_dn10)), (locals.var_mfactor * ((locals.var_qbs_btm_dn13 + locals.var_qbs_sws_dn13) + locals.var_qbs_swg_dn13)),)
    } else {
        (locals.var_qbs, locals.var_qbs_dn0, locals.var_qbs_dn2, locals.var_qbs_dn4, locals.var_qbs_dn5, locals.var_qbs_dn6, locals.var_qbs_dn7, locals.var_qbs_dn8, locals.var_qbs_dn9, locals.var_qbs_dn10, locals.var_qbs_dn13,)
    }
};
        locals.var_qbs = assign99540_e151820;
        locals.var_qbs_dn0 = assign99540_e151820_d_n0;
        locals.var_qbs_dn2 = assign99540_e151820_d_n2;
        locals.var_qbs_dn4 = assign99540_e151820_d_n4;
        locals.var_qbs_dn5 = assign99540_e151820_d_n5;
        locals.var_qbs_dn6 = assign99540_e151820_d_n6;
        locals.var_qbs_dn7 = assign99540_e151820_d_n7;
        locals.var_qbs_dn8 = assign99540_e151820_d_n8;
        locals.var_qbs_dn9 = assign99540_e151820_d_n9;
        locals.var_qbs_dn10 = assign99540_e151820_d_n10;
        locals.var_qbs_dn13 = assign99540_e151820_d_n13;
        locals.var_qbs_rv = 0.0;

        let (assign99550_e151831, assign99550_e151831_d_n0, assign99550_e151831_d_n2, assign99550_e151831_d_n4, assign99550_e151831_d_n5, assign99550_e151831_d_n6, assign99550_e151831_d_n7, assign99550_e151831_d_n8, assign99550_e151831_d_n9, assign99550_e151831_d_n10, assign99550_e151831_d_n13, assign99550_e151831_d_n15, assign99550_e151831_d_n16, assign99550_e151831_d_n17,) = {
    if (locals.var_guard2300 == 0.0) {
        let assign99550_e151826: f64 = (locals.var_qbd_btm + locals.var_qbd_sws);
        let assign99550_e151828: f64 = (assign99550_e151826 + locals.var_qbd_swg);
        let assign99550_e151829: f64 = (locals.var_mfactor * assign99550_e151828);
        (assign99550_e151829, (locals.var_mfactor * ((locals.var_qbd_btm_dn0 + locals.var_qbd_sws_dn0) + locals.var_qbd_swg_dn0)), (locals.var_mfactor * ((locals.var_qbd_btm_dn2 + locals.var_qbd_sws_dn2) + locals.var_qbd_swg_dn2)), (locals.var_mfactor * ((locals.var_qbd_btm_dn4 + locals.var_qbd_sws_dn4) + locals.var_qbd_swg_dn4)), (locals.var_mfactor * ((locals.var_qbd_btm_dn5 + locals.var_qbd_sws_dn5) + locals.var_qbd_swg_dn5)), (locals.var_mfactor * ((locals.var_qbd_btm_dn6 + locals.var_qbd_sws_dn6) + locals.var_qbd_swg_dn6)), (locals.var_mfactor * ((locals.var_qbd_btm_dn7 + locals.var_qbd_sws_dn7) + locals.var_qbd_swg_dn7)), (locals.var_mfactor * ((locals.var_qbd_btm_dn8 + locals.var_qbd_sws_dn8) + locals.var_qbd_swg_dn8)), (locals.var_mfactor * ((locals.var_qbd_btm_dn9 + locals.var_qbd_sws_dn9) + locals.var_qbd_swg_dn9)), (locals.var_mfactor * ((locals.var_qbd_btm_dn10 + locals.var_qbd_sws_dn10) + locals.var_qbd_swg_dn10)), (locals.var_mfactor * ((locals.var_qbd_btm_dn13 + locals.var_qbd_sws_dn13) + locals.var_qbd_swg_dn13)), 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qbd, locals.var_qbd_dn0, locals.var_qbd_dn2, locals.var_qbd_dn4, locals.var_qbd_dn5, locals.var_qbd_dn6, locals.var_qbd_dn7, locals.var_qbd_dn8, locals.var_qbd_dn9, locals.var_qbd_dn10, locals.var_qbd_dn13, locals.var_qbd_dn15, locals.var_qbd_dn16, locals.var_qbd_dn17,)
    }
};
        locals.var_qbd = assign99550_e151831;
        locals.var_qbd_dn0 = assign99550_e151831_d_n0;
        locals.var_qbd_dn2 = assign99550_e151831_d_n2;
        locals.var_qbd_dn4 = assign99550_e151831_d_n4;
        locals.var_qbd_dn5 = assign99550_e151831_d_n5;
        locals.var_qbd_dn6 = assign99550_e151831_d_n6;
        locals.var_qbd_dn7 = assign99550_e151831_d_n7;
        locals.var_qbd_dn8 = assign99550_e151831_d_n8;
        locals.var_qbd_dn9 = assign99550_e151831_d_n9;
        locals.var_qbd_dn10 = assign99550_e151831_d_n10;
        locals.var_qbd_dn13 = assign99550_e151831_d_n13;
        locals.var_qbd_dn15 = assign99550_e151831_d_n15;
        locals.var_qbd_dn16 = assign99550_e151831_d_n16;
        locals.var_qbd_dn17 = assign99550_e151831_d_n17;
        locals.var_qbd_rv = 0.0;

        let (assign99580_e151858, assign99580_e151858_d_n0, assign99580_e151858_d_n2, assign99580_e151858_d_n4, assign99580_e151858_d_n5, assign99580_e151858_d_n6, assign99580_e151858_d_n7, assign99580_e151858_d_n8, assign99580_e151858_d_n9, assign99580_e151858_d_n10, assign99580_e151858_d_n13,) = {
    if (locals.var_guard2300 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qbsi, locals.var_qbsi_dn0, locals.var_qbsi_dn2, locals.var_qbsi_dn4, locals.var_qbsi_dn5, locals.var_qbsi_dn6, locals.var_qbsi_dn7, locals.var_qbsi_dn8, locals.var_qbsi_dn9, locals.var_qbsi_dn10, locals.var_qbsi_dn13,)
    }
};
        locals.var_qbsi = assign99580_e151858;
        locals.var_qbsi_dn0 = assign99580_e151858_d_n0;
        locals.var_qbsi_dn2 = assign99580_e151858_d_n2;
        locals.var_qbsi_dn4 = assign99580_e151858_d_n4;
        locals.var_qbsi_dn5 = assign99580_e151858_d_n5;
        locals.var_qbsi_dn6 = assign99580_e151858_d_n6;
        locals.var_qbsi_dn7 = assign99580_e151858_d_n7;
        locals.var_qbsi_dn8 = assign99580_e151858_d_n8;
        locals.var_qbsi_dn9 = assign99580_e151858_d_n9;
        locals.var_qbsi_dn10 = assign99580_e151858_d_n10;
        locals.var_qbsi_dn13 = assign99580_e151858_d_n13;
        locals.var_qbsi_rv = 0.0;

        let (assign99590_e151863, assign99590_e151863_d_n0, assign99590_e151863_d_n2, assign99590_e151863_d_n4, assign99590_e151863_d_n5, assign99590_e151863_d_n6, assign99590_e151863_d_n7, assign99590_e151863_d_n8, assign99590_e151863_d_n9, assign99590_e151863_d_n10, assign99590_e151863_d_n13,) = {
    if (locals.var_guard2300 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qbdi, locals.var_qbdi_dn0, locals.var_qbdi_dn2, locals.var_qbdi_dn4, locals.var_qbdi_dn5, locals.var_qbdi_dn6, locals.var_qbdi_dn7, locals.var_qbdi_dn8, locals.var_qbdi_dn9, locals.var_qbdi_dn10, locals.var_qbdi_dn13,)
    }
};
        locals.var_qbdi = assign99590_e151863;
        locals.var_qbdi_dn0 = assign99590_e151863_d_n0;
        locals.var_qbdi_dn2 = assign99590_e151863_d_n2;
        locals.var_qbdi_dn4 = assign99590_e151863_d_n4;
        locals.var_qbdi_dn5 = assign99590_e151863_d_n5;
        locals.var_qbdi_dn6 = assign99590_e151863_d_n6;
        locals.var_qbdi_dn7 = assign99590_e151863_d_n7;
        locals.var_qbdi_dn8 = assign99590_e151863_d_n8;
        locals.var_qbdi_dn9 = assign99590_e151863_d_n9;
        locals.var_qbdi_dn10 = assign99590_e151863_d_n10;
        locals.var_qbdi_dn13 = assign99590_e151863_d_n13;
        locals.var_qbdi_rv = 0.0;

        let assign99620_e151876: f64 = (p.p540 / 1e-6);
        locals.var_ndi_i = assign99620_e151876;
        locals.var_ndi_i_rv = 0.0;

        locals.var_njl = locals.var_uc_njd;
        locals.var_njl_rv = 0.0;

        let assign99640_e151880: f64 = (1450.0 / 10000.0);
        locals.var_muen_i = assign99640_e151880;
        locals.var_muen_i_rv = 0.0;

        let assign99650_e151883: f64 = (500.0 / 10000.0);
        locals.var_muep_i = assign99650_e151883;
        locals.var_muep_i_rv = 0.0;

        locals.var_juncdlt = 0.001;
        locals.var_juncdlt_rv = 0.0;

        let assign99670_e151888: f64 = (locals.var_egtnom * locals.var_betatnom);
        let assign99670_e151891: f64 = (locals.var_eg * locals.var_beta);
        let assign99670_e151892: f64 = (assign99670_e151888 - assign99670_e151891);
        let assign99670_e151895: f64 = (p.p499 * locals.var_log_tratio);
        let assign99670_e151896: f64 = (assign99670_e151892 + assign99670_e151895);
        let assign99670_e151898: f64 = (assign99670_e151896 / locals.var_uc_njd);
        let assign99670_e151899: f64 = (assign99670_e151898).exp();
        let assign99670_e151900: f64 = (1.45e16 * assign99670_e151899);
        locals.var_nin_dio = assign99670_e151900;
        locals.var_nin_dio_dn0 = (1.45e16 * (assign99670_e151899 * (((-((locals.var_eg_dn0 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn0))) + (p.p499 * locals.var_log_tratio_dn0)) / locals.var_uc_njd)));
        locals.var_nin_dio_dn2 = (1.45e16 * (assign99670_e151899 * (((-((locals.var_eg_dn2 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn2))) + (p.p499 * locals.var_log_tratio_dn2)) / locals.var_uc_njd)));
        locals.var_nin_dio_dn4 = (1.45e16 * (assign99670_e151899 * (((-((locals.var_eg_dn4 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn4))) + (p.p499 * locals.var_log_tratio_dn4)) / locals.var_uc_njd)));
        locals.var_nin_dio_dn5 = (1.45e16 * (assign99670_e151899 * (((-((locals.var_eg_dn5 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn5))) + (p.p499 * locals.var_log_tratio_dn5)) / locals.var_uc_njd)));
        locals.var_nin_dio_dn6 = (1.45e16 * (assign99670_e151899 * (((-((locals.var_eg_dn6 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn6))) + (p.p499 * locals.var_log_tratio_dn6)) / locals.var_uc_njd)));
        locals.var_nin_dio_dn7 = (1.45e16 * (assign99670_e151899 * (((-((locals.var_eg_dn7 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn7))) + (p.p499 * locals.var_log_tratio_dn7)) / locals.var_uc_njd)));
        locals.var_nin_dio_dn8 = (1.45e16 * (assign99670_e151899 * (((-((locals.var_eg_dn8 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn8))) + (p.p499 * locals.var_log_tratio_dn8)) / locals.var_uc_njd)));
        locals.var_nin_dio_dn9 = (1.45e16 * (assign99670_e151899 * (((-((locals.var_eg_dn9 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn9))) + (p.p499 * locals.var_log_tratio_dn9)) / locals.var_uc_njd)));
        locals.var_nin_dio_dn10 = (1.45e16 * (assign99670_e151899 * (((-((locals.var_eg_dn10 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn10))) + (p.p499 * locals.var_log_tratio_dn10)) / locals.var_uc_njd)));
        locals.var_nin_dio_dn13 = (1.45e16 * (assign99670_e151899 * (((-((locals.var_eg_dn13 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn13))) + (p.p499 * locals.var_log_tratio_dn13)) / locals.var_uc_njd)));
        locals.var_nin_dio_rv = 0.0;

        let assign99680_e151903: f64 = (locals.var_nin_dio * locals.var_nin_dio);
        let assign99680_e151905: f64 = (assign99680_e151903 / locals.var_ndi_i);
        locals.var_pn0 = assign99680_e151905;
        locals.var_pn0_dn0 = (((locals.var_nin_dio_dn0 * locals.var_nin_dio) + (locals.var_nin_dio * locals.var_nin_dio_dn0)) / locals.var_ndi_i);
        locals.var_pn0_dn2 = (((locals.var_nin_dio_dn2 * locals.var_nin_dio) + (locals.var_nin_dio * locals.var_nin_dio_dn2)) / locals.var_ndi_i);
        locals.var_pn0_dn4 = (((locals.var_nin_dio_dn4 * locals.var_nin_dio) + (locals.var_nin_dio * locals.var_nin_dio_dn4)) / locals.var_ndi_i);
        locals.var_pn0_dn5 = (((locals.var_nin_dio_dn5 * locals.var_nin_dio) + (locals.var_nin_dio * locals.var_nin_dio_dn5)) / locals.var_ndi_i);
        locals.var_pn0_dn6 = (((locals.var_nin_dio_dn6 * locals.var_nin_dio) + (locals.var_nin_dio * locals.var_nin_dio_dn6)) / locals.var_ndi_i);
        locals.var_pn0_dn7 = (((locals.var_nin_dio_dn7 * locals.var_nin_dio) + (locals.var_nin_dio * locals.var_nin_dio_dn7)) / locals.var_ndi_i);
        locals.var_pn0_dn8 = (((locals.var_nin_dio_dn8 * locals.var_nin_dio) + (locals.var_nin_dio * locals.var_nin_dio_dn8)) / locals.var_ndi_i);
        locals.var_pn0_dn9 = (((locals.var_nin_dio_dn9 * locals.var_nin_dio) + (locals.var_nin_dio * locals.var_nin_dio_dn9)) / locals.var_ndi_i);
        locals.var_pn0_dn10 = (((locals.var_nin_dio_dn10 * locals.var_nin_dio) + (locals.var_nin_dio * locals.var_nin_dio_dn10)) / locals.var_ndi_i);
        locals.var_pn0_dn13 = (((locals.var_nin_dio_dn13 * locals.var_nin_dio) + (locals.var_nin_dio * locals.var_nin_dio_dn13)) / locals.var_ndi_i);
        locals.var_pn0_rv = 0.0;

        let assign99690_e151908: f64 = (-1.5);
        let assign99690_e151909: f64 = (locals.var_tratio).powf(assign99690_e151908);
        locals.var_t1 = assign99690_e151909;
        locals.var_t1_dn0 = if 0.0 == 0.0 && ((assign99690_e151908) as f64).is_finite() && ((assign99690_e151908) as f64).fract() == 0.0 { if assign99690_e151908 == 0.0 { 0.0 } else { (assign99690_e151908 * ((locals.var_tratio).powf(assign99690_e151908 - 1.0) * locals.var_tratio_dn0)) } } else { (assign99690_e151909 * (assign99690_e151908 * (locals.var_tratio_dn0 / locals.var_tratio))) };
        locals.var_t1_dn2 = if 0.0 == 0.0 && ((assign99690_e151908) as f64).is_finite() && ((assign99690_e151908) as f64).fract() == 0.0 { if assign99690_e151908 == 0.0 { 0.0 } else { (assign99690_e151908 * ((locals.var_tratio).powf(assign99690_e151908 - 1.0) * locals.var_tratio_dn2)) } } else { (assign99690_e151909 * (assign99690_e151908 * (locals.var_tratio_dn2 / locals.var_tratio))) };
        locals.var_t1_dn4 = if 0.0 == 0.0 && ((assign99690_e151908) as f64).is_finite() && ((assign99690_e151908) as f64).fract() == 0.0 { if assign99690_e151908 == 0.0 { 0.0 } else { (assign99690_e151908 * ((locals.var_tratio).powf(assign99690_e151908 - 1.0) * locals.var_tratio_dn4)) } } else { (assign99690_e151909 * (assign99690_e151908 * (locals.var_tratio_dn4 / locals.var_tratio))) };
        locals.var_t1_dn5 = if 0.0 == 0.0 && ((assign99690_e151908) as f64).is_finite() && ((assign99690_e151908) as f64).fract() == 0.0 { if assign99690_e151908 == 0.0 { 0.0 } else { (assign99690_e151908 * ((locals.var_tratio).powf(assign99690_e151908 - 1.0) * locals.var_tratio_dn5)) } } else { (assign99690_e151909 * (assign99690_e151908 * (locals.var_tratio_dn5 / locals.var_tratio))) };
        locals.var_t1_dn6 = if 0.0 == 0.0 && ((assign99690_e151908) as f64).is_finite() && ((assign99690_e151908) as f64).fract() == 0.0 { if assign99690_e151908 == 0.0 { 0.0 } else { (assign99690_e151908 * ((locals.var_tratio).powf(assign99690_e151908 - 1.0) * locals.var_tratio_dn6)) } } else { (assign99690_e151909 * (assign99690_e151908 * (locals.var_tratio_dn6 / locals.var_tratio))) };
        locals.var_t1_dn7 = if 0.0 == 0.0 && ((assign99690_e151908) as f64).is_finite() && ((assign99690_e151908) as f64).fract() == 0.0 { if assign99690_e151908 == 0.0 { 0.0 } else { (assign99690_e151908 * ((locals.var_tratio).powf(assign99690_e151908 - 1.0) * locals.var_tratio_dn7)) } } else { (assign99690_e151909 * (assign99690_e151908 * (locals.var_tratio_dn7 / locals.var_tratio))) };
        locals.var_t1_dn8 = if 0.0 == 0.0 && ((assign99690_e151908) as f64).is_finite() && ((assign99690_e151908) as f64).fract() == 0.0 { if assign99690_e151908 == 0.0 { 0.0 } else { (assign99690_e151908 * ((locals.var_tratio).powf(assign99690_e151908 - 1.0) * locals.var_tratio_dn8)) } } else { (assign99690_e151909 * (assign99690_e151908 * (locals.var_tratio_dn8 / locals.var_tratio))) };
        locals.var_t1_dn9 = if 0.0 == 0.0 && ((assign99690_e151908) as f64).is_finite() && ((assign99690_e151908) as f64).fract() == 0.0 { if assign99690_e151908 == 0.0 { 0.0 } else { (assign99690_e151908 * ((locals.var_tratio).powf(assign99690_e151908 - 1.0) * locals.var_tratio_dn9)) } } else { (assign99690_e151909 * (assign99690_e151908 * (locals.var_tratio_dn9 / locals.var_tratio))) };
        locals.var_t1_dn10 = if 0.0 == 0.0 && ((assign99690_e151908) as f64).is_finite() && ((assign99690_e151908) as f64).fract() == 0.0 { if assign99690_e151908 == 0.0 { 0.0 } else { (assign99690_e151908 * ((locals.var_tratio).powf(assign99690_e151908 - 1.0) * locals.var_tratio_dn10)) } } else { (assign99690_e151909 * (assign99690_e151908 * (locals.var_tratio_dn10 / locals.var_tratio))) };
        locals.var_t1_dn13 = if 0.0 == 0.0 && ((assign99690_e151908) as f64).is_finite() && ((assign99690_e151908) as f64).fract() == 0.0 { if assign99690_e151908 == 0.0 { 0.0 } else { (assign99690_e151908 * ((locals.var_tratio).powf(assign99690_e151908 - 1.0) * locals.var_tratio_dn13)) } } else { (assign99690_e151909 * (assign99690_e151908 * (locals.var_tratio_dn13 / locals.var_tratio))) };
        locals.var_t1_rv = 0.0;

        let assign99700_e151912: f64 = (locals.var_muen_i * locals.var_t1);
        let assign99700_e151914: f64 = (assign99700_e151912 * locals.var_beta_inv);
        locals.var_dn = assign99700_e151914;
        locals.var_dn_dn0 = (((locals.var_muen_i * locals.var_t1_dn0) * locals.var_beta_inv) + (assign99700_e151912 * locals.var_beta_inv_dn0));
        locals.var_dn_dn2 = (((locals.var_muen_i * locals.var_t1_dn2) * locals.var_beta_inv) + (assign99700_e151912 * locals.var_beta_inv_dn2));
        locals.var_dn_dn4 = (((locals.var_muen_i * locals.var_t1_dn4) * locals.var_beta_inv) + (assign99700_e151912 * locals.var_beta_inv_dn4));
        locals.var_dn_dn5 = (((locals.var_muen_i * locals.var_t1_dn5) * locals.var_beta_inv) + (assign99700_e151912 * locals.var_beta_inv_dn5));
        locals.var_dn_dn6 = (((locals.var_muen_i * locals.var_t1_dn6) * locals.var_beta_inv) + (assign99700_e151912 * locals.var_beta_inv_dn6));
        locals.var_dn_dn7 = (((locals.var_muen_i * locals.var_t1_dn7) * locals.var_beta_inv) + (assign99700_e151912 * locals.var_beta_inv_dn7));
        locals.var_dn_dn8 = (((locals.var_muen_i * locals.var_t1_dn8) * locals.var_beta_inv) + (assign99700_e151912 * locals.var_beta_inv_dn8));
        locals.var_dn_dn9 = (((locals.var_muen_i * locals.var_t1_dn9) * locals.var_beta_inv) + (assign99700_e151912 * locals.var_beta_inv_dn9));
        locals.var_dn_dn10 = (((locals.var_muen_i * locals.var_t1_dn10) * locals.var_beta_inv) + (assign99700_e151912 * locals.var_beta_inv_dn10));
        locals.var_dn_dn13 = (((locals.var_muen_i * locals.var_t1_dn13) * locals.var_beta_inv) + (assign99700_e151912 * locals.var_beta_inv_dn13));
        locals.var_dn_rv = 0.0;

        let assign99710_e151917: f64 = (locals.var_muep_i * locals.var_t1);
        let assign99710_e151919: f64 = (assign99710_e151917 * locals.var_beta_inv);
        locals.var_dp = assign99710_e151919;
        locals.var_dp_dn0 = (((locals.var_muep_i * locals.var_t1_dn0) * locals.var_beta_inv) + (assign99710_e151917 * locals.var_beta_inv_dn0));
        locals.var_dp_dn2 = (((locals.var_muep_i * locals.var_t1_dn2) * locals.var_beta_inv) + (assign99710_e151917 * locals.var_beta_inv_dn2));
        locals.var_dp_dn4 = (((locals.var_muep_i * locals.var_t1_dn4) * locals.var_beta_inv) + (assign99710_e151917 * locals.var_beta_inv_dn4));
        locals.var_dp_dn5 = (((locals.var_muep_i * locals.var_t1_dn5) * locals.var_beta_inv) + (assign99710_e151917 * locals.var_beta_inv_dn5));
        locals.var_dp_dn6 = (((locals.var_muep_i * locals.var_t1_dn6) * locals.var_beta_inv) + (assign99710_e151917 * locals.var_beta_inv_dn6));
        locals.var_dp_dn7 = (((locals.var_muep_i * locals.var_t1_dn7) * locals.var_beta_inv) + (assign99710_e151917 * locals.var_beta_inv_dn7));
        locals.var_dp_dn8 = (((locals.var_muep_i * locals.var_t1_dn8) * locals.var_beta_inv) + (assign99710_e151917 * locals.var_beta_inv_dn8));
        locals.var_dp_dn9 = (((locals.var_muep_i * locals.var_t1_dn9) * locals.var_beta_inv) + (assign99710_e151917 * locals.var_beta_inv_dn9));
        locals.var_dp_dn10 = (((locals.var_muep_i * locals.var_t1_dn10) * locals.var_beta_inv) + (assign99710_e151917 * locals.var_beta_inv_dn10));
        locals.var_dp_dn13 = (((locals.var_muep_i * locals.var_t1_dn13) * locals.var_beta_inv) + (assign99710_e151917 * locals.var_beta_inv_dn13));
        locals.var_dp_rv = 0.0;

        let assign99720_e151922: f64 = (2.0 * locals.var_dn);
        let assign99720_e151924: f64 = (assign99720_e151922 * locals.var_dp);
        let assign99720_e151927: f64 = (locals.var_dn + locals.var_dp);
        let assign99720_e151928: f64 = (assign99720_e151924 / assign99720_e151927);
        locals.var_da = assign99720_e151928;
        locals.var_da_dn0 = ((((((2.0 * locals.var_dn_dn0) * locals.var_dp) + (assign99720_e151922 * locals.var_dp_dn0)) * assign99720_e151927) - (assign99720_e151924 * (locals.var_dn_dn0 + locals.var_dp_dn0))) / (assign99720_e151927 * assign99720_e151927));
        locals.var_da_dn2 = ((((((2.0 * locals.var_dn_dn2) * locals.var_dp) + (assign99720_e151922 * locals.var_dp_dn2)) * assign99720_e151927) - (assign99720_e151924 * (locals.var_dn_dn2 + locals.var_dp_dn2))) / (assign99720_e151927 * assign99720_e151927));
        locals.var_da_dn4 = ((((((2.0 * locals.var_dn_dn4) * locals.var_dp) + (assign99720_e151922 * locals.var_dp_dn4)) * assign99720_e151927) - (assign99720_e151924 * (locals.var_dn_dn4 + locals.var_dp_dn4))) / (assign99720_e151927 * assign99720_e151927));
        locals.var_da_dn5 = ((((((2.0 * locals.var_dn_dn5) * locals.var_dp) + (assign99720_e151922 * locals.var_dp_dn5)) * assign99720_e151927) - (assign99720_e151924 * (locals.var_dn_dn5 + locals.var_dp_dn5))) / (assign99720_e151927 * assign99720_e151927));
        locals.var_da_dn6 = ((((((2.0 * locals.var_dn_dn6) * locals.var_dp) + (assign99720_e151922 * locals.var_dp_dn6)) * assign99720_e151927) - (assign99720_e151924 * (locals.var_dn_dn6 + locals.var_dp_dn6))) / (assign99720_e151927 * assign99720_e151927));
        locals.var_da_dn7 = ((((((2.0 * locals.var_dn_dn7) * locals.var_dp) + (assign99720_e151922 * locals.var_dp_dn7)) * assign99720_e151927) - (assign99720_e151924 * (locals.var_dn_dn7 + locals.var_dp_dn7))) / (assign99720_e151927 * assign99720_e151927));
        locals.var_da_dn8 = ((((((2.0 * locals.var_dn_dn8) * locals.var_dp) + (assign99720_e151922 * locals.var_dp_dn8)) * assign99720_e151927) - (assign99720_e151924 * (locals.var_dn_dn8 + locals.var_dp_dn8))) / (assign99720_e151927 * assign99720_e151927));
        locals.var_da_dn9 = ((((((2.0 * locals.var_dn_dn9) * locals.var_dp) + (assign99720_e151922 * locals.var_dp_dn9)) * assign99720_e151927) - (assign99720_e151924 * (locals.var_dn_dn9 + locals.var_dp_dn9))) / (assign99720_e151927 * assign99720_e151927));
        locals.var_da_dn10 = ((((((2.0 * locals.var_dn_dn10) * locals.var_dp) + (assign99720_e151922 * locals.var_dp_dn10)) * assign99720_e151927) - (assign99720_e151924 * (locals.var_dn_dn10 + locals.var_dp_dn10))) / (assign99720_e151927 * assign99720_e151927));
        locals.var_da_dn13 = ((((((2.0 * locals.var_dn_dn13) * locals.var_dp) + (assign99720_e151922 * locals.var_dp_dn13)) * assign99720_e151927) - (assign99720_e151924 * (locals.var_dn_dn13 + locals.var_dp_dn13))) / (assign99720_e151927 * assign99720_e151927));
        locals.var_da_rv = 0.0;

        let assign99730_e151931: f64 = (locals.var_tratio).powf(p.p547);
        locals.var_t2 = assign99730_e151931;
        locals.var_t2_dn0 = if 0.0 == 0.0 && ((p.p547) as f64).is_finite() && ((p.p547) as f64).fract() == 0.0 { if p.p547 == 0.0 { 0.0 } else { (p.p547 * ((locals.var_tratio).powf(p.p547 - 1.0) * locals.var_tratio_dn0)) } } else { (assign99730_e151931 * (p.p547 * (locals.var_tratio_dn0 / locals.var_tratio))) };
        locals.var_t2_dn2 = if 0.0 == 0.0 && ((p.p547) as f64).is_finite() && ((p.p547) as f64).fract() == 0.0 { if p.p547 == 0.0 { 0.0 } else { (p.p547 * ((locals.var_tratio).powf(p.p547 - 1.0) * locals.var_tratio_dn2)) } } else { (assign99730_e151931 * (p.p547 * (locals.var_tratio_dn2 / locals.var_tratio))) };
        locals.var_t2_dn4 = if 0.0 == 0.0 && ((p.p547) as f64).is_finite() && ((p.p547) as f64).fract() == 0.0 { if p.p547 == 0.0 { 0.0 } else { (p.p547 * ((locals.var_tratio).powf(p.p547 - 1.0) * locals.var_tratio_dn4)) } } else { (assign99730_e151931 * (p.p547 * (locals.var_tratio_dn4 / locals.var_tratio))) };
        locals.var_t2_dn5 = if 0.0 == 0.0 && ((p.p547) as f64).is_finite() && ((p.p547) as f64).fract() == 0.0 { if p.p547 == 0.0 { 0.0 } else { (p.p547 * ((locals.var_tratio).powf(p.p547 - 1.0) * locals.var_tratio_dn5)) } } else { (assign99730_e151931 * (p.p547 * (locals.var_tratio_dn5 / locals.var_tratio))) };
        locals.var_t2_dn6 = if 0.0 == 0.0 && ((p.p547) as f64).is_finite() && ((p.p547) as f64).fract() == 0.0 { if p.p547 == 0.0 { 0.0 } else { (p.p547 * ((locals.var_tratio).powf(p.p547 - 1.0) * locals.var_tratio_dn6)) } } else { (assign99730_e151931 * (p.p547 * (locals.var_tratio_dn6 / locals.var_tratio))) };
        locals.var_t2_dn7 = if 0.0 == 0.0 && ((p.p547) as f64).is_finite() && ((p.p547) as f64).fract() == 0.0 { if p.p547 == 0.0 { 0.0 } else { (p.p547 * ((locals.var_tratio).powf(p.p547 - 1.0) * locals.var_tratio_dn7)) } } else { (assign99730_e151931 * (p.p547 * (locals.var_tratio_dn7 / locals.var_tratio))) };
        locals.var_t2_dn8 = if 0.0 == 0.0 && ((p.p547) as f64).is_finite() && ((p.p547) as f64).fract() == 0.0 { if p.p547 == 0.0 { 0.0 } else { (p.p547 * ((locals.var_tratio).powf(p.p547 - 1.0) * locals.var_tratio_dn8)) } } else { (assign99730_e151931 * (p.p547 * (locals.var_tratio_dn8 / locals.var_tratio))) };
        locals.var_t2_dn9 = if 0.0 == 0.0 && ((p.p547) as f64).is_finite() && ((p.p547) as f64).fract() == 0.0 { if p.p547 == 0.0 { 0.0 } else { (p.p547 * ((locals.var_tratio).powf(p.p547 - 1.0) * locals.var_tratio_dn9)) } } else { (assign99730_e151931 * (p.p547 * (locals.var_tratio_dn9 / locals.var_tratio))) };
        locals.var_t2_dn10 = if 0.0 == 0.0 && ((p.p547) as f64).is_finite() && ((p.p547) as f64).fract() == 0.0 { if p.p547 == 0.0 { 0.0 } else { (p.p547 * ((locals.var_tratio).powf(p.p547 - 1.0) * locals.var_tratio_dn10)) } } else { (assign99730_e151931 * (p.p547 * (locals.var_tratio_dn10 / locals.var_tratio))) };
        locals.var_t2_dn13 = if 0.0 == 0.0 && ((p.p547) as f64).is_finite() && ((p.p547) as f64).fract() == 0.0 { if p.p547 == 0.0 { 0.0 } else { (p.p547 * ((locals.var_tratio).powf(p.p547 - 1.0) * locals.var_tratio_dn13)) } } else { (assign99730_e151931 * (p.p547 * (locals.var_tratio_dn13 / locals.var_tratio))) };
        locals.var_t2_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_371(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        locals: &mut StampLocals,
    ) {
        let nv15 = ctx.node_voltage(nodes[15]);
        let assign99740_e151934: f64 = (p.p544 * locals.var_t2);
        locals.var_tau_hl = assign99740_e151934;
        locals.var_tau_hl_dn0 = (p.p544 * locals.var_t2_dn0);
        locals.var_tau_hl_dn2 = (p.p544 * locals.var_t2_dn2);
        locals.var_tau_hl_dn4 = (p.p544 * locals.var_t2_dn4);
        locals.var_tau_hl_dn5 = (p.p544 * locals.var_t2_dn5);
        locals.var_tau_hl_dn6 = (p.p544 * locals.var_t2_dn6);
        locals.var_tau_hl_dn7 = (p.p544 * locals.var_t2_dn7);
        locals.var_tau_hl_dn8 = (p.p544 * locals.var_t2_dn8);
        locals.var_tau_hl_dn9 = (p.p544 * locals.var_t2_dn9);
        locals.var_tau_hl_dn10 = (p.p544 * locals.var_t2_dn10);
        locals.var_tau_hl_dn13 = (p.p544 * locals.var_t2_dn13);
        locals.var_tau_hl_rv = 0.0;

        let assign99750_e151937: f64 = (locals.var_tau_hl * locals.var_da);
        let assign99750_e151938: f64 = (assign99750_e151937).sqrt();
        locals.var_la = assign99750_e151938;
        locals.var_la_dn0 = (((locals.var_tau_hl_dn0 * locals.var_da) + (locals.var_tau_hl * locals.var_da_dn0)) / (2.0 * assign99750_e151938));
        locals.var_la_dn2 = (((locals.var_tau_hl_dn2 * locals.var_da) + (locals.var_tau_hl * locals.var_da_dn2)) / (2.0 * assign99750_e151938));
        locals.var_la_dn4 = (((locals.var_tau_hl_dn4 * locals.var_da) + (locals.var_tau_hl * locals.var_da_dn4)) / (2.0 * assign99750_e151938));
        locals.var_la_dn5 = (((locals.var_tau_hl_dn5 * locals.var_da) + (locals.var_tau_hl * locals.var_da_dn5)) / (2.0 * assign99750_e151938));
        locals.var_la_dn6 = (((locals.var_tau_hl_dn6 * locals.var_da) + (locals.var_tau_hl * locals.var_da_dn6)) / (2.0 * assign99750_e151938));
        locals.var_la_dn7 = (((locals.var_tau_hl_dn7 * locals.var_da) + (locals.var_tau_hl * locals.var_da_dn7)) / (2.0 * assign99750_e151938));
        locals.var_la_dn8 = (((locals.var_tau_hl_dn8 * locals.var_da) + (locals.var_tau_hl * locals.var_da_dn8)) / (2.0 * assign99750_e151938));
        locals.var_la_dn9 = (((locals.var_tau_hl_dn9 * locals.var_da) + (locals.var_tau_hl * locals.var_da_dn9)) / (2.0 * assign99750_e151938));
        locals.var_la_dn10 = (((locals.var_tau_hl_dn10 * locals.var_da) + (locals.var_tau_hl * locals.var_da_dn10)) / (2.0 * assign99750_e151938));
        locals.var_la_dn13 = (((locals.var_tau_hl_dn13 * locals.var_da) + (locals.var_tau_hl * locals.var_da_dn13)) / (2.0 * assign99750_e151938));
        locals.var_la_rv = 0.0;

        let assign99760_e151941: f64 = (locals.var_njl * locals.var_beta_inv);
        let assign99760_e151944: f64 = (locals.var_ndi_i / locals.var_pn0);
        let assign99760_e151945: f64 = (assign99760_e151944).ln();
        let assign99760_e151946: f64 = (assign99760_e151941 * assign99760_e151945);
        locals.var_v_ha = assign99760_e151946;
        locals.var_v_ha_dn0 = (((locals.var_njl * locals.var_beta_inv_dn0) * assign99760_e151945) + (assign99760_e151941 * ((-((locals.var_ndi_i * locals.var_pn0_dn0) / (locals.var_pn0 * locals.var_pn0))) / assign99760_e151944)));
        locals.var_v_ha_dn2 = (((locals.var_njl * locals.var_beta_inv_dn2) * assign99760_e151945) + (assign99760_e151941 * ((-((locals.var_ndi_i * locals.var_pn0_dn2) / (locals.var_pn0 * locals.var_pn0))) / assign99760_e151944)));
        locals.var_v_ha_dn4 = (((locals.var_njl * locals.var_beta_inv_dn4) * assign99760_e151945) + (assign99760_e151941 * ((-((locals.var_ndi_i * locals.var_pn0_dn4) / (locals.var_pn0 * locals.var_pn0))) / assign99760_e151944)));
        locals.var_v_ha_dn5 = (((locals.var_njl * locals.var_beta_inv_dn5) * assign99760_e151945) + (assign99760_e151941 * ((-((locals.var_ndi_i * locals.var_pn0_dn5) / (locals.var_pn0 * locals.var_pn0))) / assign99760_e151944)));
        locals.var_v_ha_dn6 = (((locals.var_njl * locals.var_beta_inv_dn6) * assign99760_e151945) + (assign99760_e151941 * ((-((locals.var_ndi_i * locals.var_pn0_dn6) / (locals.var_pn0 * locals.var_pn0))) / assign99760_e151944)));
        locals.var_v_ha_dn7 = (((locals.var_njl * locals.var_beta_inv_dn7) * assign99760_e151945) + (assign99760_e151941 * ((-((locals.var_ndi_i * locals.var_pn0_dn7) / (locals.var_pn0 * locals.var_pn0))) / assign99760_e151944)));
        locals.var_v_ha_dn8 = (((locals.var_njl * locals.var_beta_inv_dn8) * assign99760_e151945) + (assign99760_e151941 * ((-((locals.var_ndi_i * locals.var_pn0_dn8) / (locals.var_pn0 * locals.var_pn0))) / assign99760_e151944)));
        locals.var_v_ha_dn9 = (((locals.var_njl * locals.var_beta_inv_dn9) * assign99760_e151945) + (assign99760_e151941 * ((-((locals.var_ndi_i * locals.var_pn0_dn9) / (locals.var_pn0 * locals.var_pn0))) / assign99760_e151944)));
        locals.var_v_ha_dn10 = (((locals.var_njl * locals.var_beta_inv_dn10) * assign99760_e151945) + (assign99760_e151941 * ((-((locals.var_ndi_i * locals.var_pn0_dn10) / (locals.var_pn0 * locals.var_pn0))) / assign99760_e151944)));
        locals.var_v_ha_dn13 = (((locals.var_njl * locals.var_beta_inv_dn13) * assign99760_e151945) + (assign99760_e151941 * ((-((locals.var_ndi_i * locals.var_pn0_dn13) / (locals.var_pn0 * locals.var_pn0))) / assign99760_e151944)));
        locals.var_v_ha_rv = 0.0;

        let assign99770_e151949: f64 = (locals.var_njl * locals.var_beta_inv);
        let assign99770_e151952: f64 = (locals.var_ndi_i / locals.var_pn0);
        let assign99770_e151953: f64 = (assign99770_e151952).ln();
        let assign99770_e151956: f64 = (p.p545 / locals.var_la);
        let assign99770_e151957: f64 = (assign99770_e151953 + assign99770_e151956);
        let assign99770_e151958: f64 = (assign99770_e151949 * assign99770_e151957);
        locals.var_v_hk = assign99770_e151958;
        locals.var_v_hk_dn0 = (((locals.var_njl * locals.var_beta_inv_dn0) * assign99770_e151957) + (assign99770_e151949 * (((-((locals.var_ndi_i * locals.var_pn0_dn0) / (locals.var_pn0 * locals.var_pn0))) / assign99770_e151952) + (-((p.p545 * locals.var_la_dn0) / (locals.var_la * locals.var_la))))));
        locals.var_v_hk_dn2 = (((locals.var_njl * locals.var_beta_inv_dn2) * assign99770_e151957) + (assign99770_e151949 * (((-((locals.var_ndi_i * locals.var_pn0_dn2) / (locals.var_pn0 * locals.var_pn0))) / assign99770_e151952) + (-((p.p545 * locals.var_la_dn2) / (locals.var_la * locals.var_la))))));
        locals.var_v_hk_dn4 = (((locals.var_njl * locals.var_beta_inv_dn4) * assign99770_e151957) + (assign99770_e151949 * (((-((locals.var_ndi_i * locals.var_pn0_dn4) / (locals.var_pn0 * locals.var_pn0))) / assign99770_e151952) + (-((p.p545 * locals.var_la_dn4) / (locals.var_la * locals.var_la))))));
        locals.var_v_hk_dn5 = (((locals.var_njl * locals.var_beta_inv_dn5) * assign99770_e151957) + (assign99770_e151949 * (((-((locals.var_ndi_i * locals.var_pn0_dn5) / (locals.var_pn0 * locals.var_pn0))) / assign99770_e151952) + (-((p.p545 * locals.var_la_dn5) / (locals.var_la * locals.var_la))))));
        locals.var_v_hk_dn6 = (((locals.var_njl * locals.var_beta_inv_dn6) * assign99770_e151957) + (assign99770_e151949 * (((-((locals.var_ndi_i * locals.var_pn0_dn6) / (locals.var_pn0 * locals.var_pn0))) / assign99770_e151952) + (-((p.p545 * locals.var_la_dn6) / (locals.var_la * locals.var_la))))));
        locals.var_v_hk_dn7 = (((locals.var_njl * locals.var_beta_inv_dn7) * assign99770_e151957) + (assign99770_e151949 * (((-((locals.var_ndi_i * locals.var_pn0_dn7) / (locals.var_pn0 * locals.var_pn0))) / assign99770_e151952) + (-((p.p545 * locals.var_la_dn7) / (locals.var_la * locals.var_la))))));
        locals.var_v_hk_dn8 = (((locals.var_njl * locals.var_beta_inv_dn8) * assign99770_e151957) + (assign99770_e151949 * (((-((locals.var_ndi_i * locals.var_pn0_dn8) / (locals.var_pn0 * locals.var_pn0))) / assign99770_e151952) + (-((p.p545 * locals.var_la_dn8) / (locals.var_la * locals.var_la))))));
        locals.var_v_hk_dn9 = (((locals.var_njl * locals.var_beta_inv_dn9) * assign99770_e151957) + (assign99770_e151949 * (((-((locals.var_ndi_i * locals.var_pn0_dn9) / (locals.var_pn0 * locals.var_pn0))) / assign99770_e151952) + (-((p.p545 * locals.var_la_dn9) / (locals.var_la * locals.var_la))))));
        locals.var_v_hk_dn10 = (((locals.var_njl * locals.var_beta_inv_dn10) * assign99770_e151957) + (assign99770_e151949 * (((-((locals.var_ndi_i * locals.var_pn0_dn10) / (locals.var_pn0 * locals.var_pn0))) / assign99770_e151952) + (-((p.p545 * locals.var_la_dn10) / (locals.var_la * locals.var_la))))));
        locals.var_v_hk_dn13 = (((locals.var_njl * locals.var_beta_inv_dn13) * assign99770_e151957) + (assign99770_e151949 * (((-((locals.var_ndi_i * locals.var_pn0_dn13) / (locals.var_pn0 * locals.var_pn0))) / assign99770_e151952) + (-((p.p545 * locals.var_la_dn13) / (locals.var_la * locals.var_la))))));
        locals.var_v_hk_rv = 0.0;

        let assign99780_e151961: f64 = if p.p539 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2301 = assign99780_e151961;
        locals.var_guard2301_rv = 0.0;

        let (assign99790_e151965,) = {
    if (locals.var_guard2301 != 0.0) {
        (locals.var_uc_njd,)
    } else {
        (locals.var_nj_k,)
    }
};
        locals.var_nj_k = assign99790_e151965;
        locals.var_nj_k_rv = 0.0;

        let (assign99800_e151972, assign99800_e151972_d_n0, assign99800_e151972_d_n2, assign99800_e151972_d_n4, assign99800_e151972_d_n5, assign99800_e151972_d_n6, assign99800_e151972_d_n7, assign99800_e151972_d_n8, assign99800_e151972_d_n9, assign99800_e151972_d_n10, assign99800_e151972_d_n13,) = {
    if (locals.var_guard2301 != 0.0) {
        let assign99800_e151969: f64 = (locals.var_vbd_jct * locals.var_jd_nvtm_invd);
        let assign99800_e151970: f64 = (assign99800_e151969).exp();
        (assign99800_e151970, (assign99800_e151970 * ((locals.var_vbd_jct_dn0 * locals.var_jd_nvtm_invd) + (locals.var_vbd_jct * locals.var_jd_nvtm_invd_dn0))), (assign99800_e151970 * (locals.var_vbd_jct * locals.var_jd_nvtm_invd_dn2)), (assign99800_e151970 * (locals.var_vbd_jct * locals.var_jd_nvtm_invd_dn4)), (assign99800_e151970 * (locals.var_vbd_jct * locals.var_jd_nvtm_invd_dn5)), (assign99800_e151970 * (locals.var_vbd_jct * locals.var_jd_nvtm_invd_dn6)), (assign99800_e151970 * (locals.var_vbd_jct * locals.var_jd_nvtm_invd_dn7)), (assign99800_e151970 * (locals.var_vbd_jct * locals.var_jd_nvtm_invd_dn8)), (assign99800_e151970 * ((locals.var_vbd_jct_dn9 * locals.var_jd_nvtm_invd) + (locals.var_vbd_jct * locals.var_jd_nvtm_invd_dn9))), (assign99800_e151970 * (locals.var_vbd_jct * locals.var_jd_nvtm_invd_dn10)), (assign99800_e151970 * (locals.var_vbd_jct * locals.var_jd_nvtm_invd_dn13)),)
    } else {
        (locals.var_exp_a, locals.var_exp_a_dn0, locals.var_exp_a_dn2, locals.var_exp_a_dn4, locals.var_exp_a_dn5, locals.var_exp_a_dn6, locals.var_exp_a_dn7, locals.var_exp_a_dn8, locals.var_exp_a_dn9, locals.var_exp_a_dn10, locals.var_exp_a_dn13,)
    }
};
        locals.var_exp_a = assign99800_e151972;
        locals.var_exp_a_dn0 = assign99800_e151972_d_n0;
        locals.var_exp_a_dn2 = assign99800_e151972_d_n2;
        locals.var_exp_a_dn4 = assign99800_e151972_d_n4;
        locals.var_exp_a_dn5 = assign99800_e151972_d_n5;
        locals.var_exp_a_dn6 = assign99800_e151972_d_n6;
        locals.var_exp_a_dn7 = assign99800_e151972_d_n7;
        locals.var_exp_a_dn8 = assign99800_e151972_d_n8;
        locals.var_exp_a_dn9 = assign99800_e151972_d_n9;
        locals.var_exp_a_dn10 = assign99800_e151972_d_n10;
        locals.var_exp_a_dn13 = assign99800_e151972_d_n13;
        locals.var_exp_a_rv = 0.0;

        let assign99810_e151976: f64 = (locals.var_v_hk - locals.var_v_ha);
        let assign99810_e151977: f64 = (locals.var_vbd_jct - assign99810_e151976);
        let assign99810_e151979: f64 = if assign99810_e151977 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2302 = assign99810_e151979;
        locals.var_guard2302_rv = 0.0;

        let (assign99820_e151996, assign99820_e151996_d_n0, assign99820_e151996_d_n2, assign99820_e151996_d_n4, assign99820_e151996_d_n5, assign99820_e151996_d_n6, assign99820_e151996_d_n7, assign99820_e151996_d_n8, assign99820_e151996_d_n9, assign99820_e151996_d_n10, assign99820_e151996_d_n13,) = {
    if ((locals.var_guard2301 != 0.0) && (locals.var_guard2302 != 0.0)) {
        let assign99820_e151986: f64 = (locals.var_vbd_jct / locals.var_nj_k);
        let assign99820_e151989: f64 = (locals.var_v_hk - locals.var_v_ha);
        let assign99820_e151991: f64 = (assign99820_e151989 / locals.var_nj_k);
        let assign99820_e151992: f64 = (assign99820_e151986 - assign99820_e151991);
        let assign99820_e151993: f64 = (locals.var_beta * assign99820_e151992);
        let assign99820_e151994: f64 = (assign99820_e151993).exp();
        (assign99820_e151994, (assign99820_e151994 * ((locals.var_beta_dn0 * assign99820_e151992) + (locals.var_beta * ((locals.var_vbd_jct_dn0 / locals.var_nj_k) - ((locals.var_v_hk_dn0 - locals.var_v_ha_dn0) / locals.var_nj_k))))), (assign99820_e151994 * ((locals.var_beta_dn2 * assign99820_e151992) + (locals.var_beta * (-((locals.var_v_hk_dn2 - locals.var_v_ha_dn2) / locals.var_nj_k))))), (assign99820_e151994 * ((locals.var_beta_dn4 * assign99820_e151992) + (locals.var_beta * (-((locals.var_v_hk_dn4 - locals.var_v_ha_dn4) / locals.var_nj_k))))), (assign99820_e151994 * ((locals.var_beta_dn5 * assign99820_e151992) + (locals.var_beta * (-((locals.var_v_hk_dn5 - locals.var_v_ha_dn5) / locals.var_nj_k))))), (assign99820_e151994 * ((locals.var_beta_dn6 * assign99820_e151992) + (locals.var_beta * (-((locals.var_v_hk_dn6 - locals.var_v_ha_dn6) / locals.var_nj_k))))), (assign99820_e151994 * ((locals.var_beta_dn7 * assign99820_e151992) + (locals.var_beta * (-((locals.var_v_hk_dn7 - locals.var_v_ha_dn7) / locals.var_nj_k))))), (assign99820_e151994 * ((locals.var_beta_dn8 * assign99820_e151992) + (locals.var_beta * (-((locals.var_v_hk_dn8 - locals.var_v_ha_dn8) / locals.var_nj_k))))), (assign99820_e151994 * ((locals.var_beta_dn9 * assign99820_e151992) + (locals.var_beta * ((locals.var_vbd_jct_dn9 / locals.var_nj_k) - ((locals.var_v_hk_dn9 - locals.var_v_ha_dn9) / locals.var_nj_k))))), (assign99820_e151994 * ((locals.var_beta_dn10 * assign99820_e151992) + (locals.var_beta * (-((locals.var_v_hk_dn10 - locals.var_v_ha_dn10) / locals.var_nj_k))))), (assign99820_e151994 * ((locals.var_beta_dn13 * assign99820_e151992) + (locals.var_beta * (-((locals.var_v_hk_dn13 - locals.var_v_ha_dn13) / locals.var_nj_k))))),)
    } else {
        (locals.var_exp_k, locals.var_exp_k_dn0, locals.var_exp_k_dn2, locals.var_exp_k_dn4, locals.var_exp_k_dn5, locals.var_exp_k_dn6, locals.var_exp_k_dn7, locals.var_exp_k_dn8, locals.var_exp_k_dn9, locals.var_exp_k_dn10, locals.var_exp_k_dn13,)
    }
};
        locals.var_exp_k = assign99820_e151996;
        locals.var_exp_k_dn0 = assign99820_e151996_d_n0;
        locals.var_exp_k_dn2 = assign99820_e151996_d_n2;
        locals.var_exp_k_dn4 = assign99820_e151996_d_n4;
        locals.var_exp_k_dn5 = assign99820_e151996_d_n5;
        locals.var_exp_k_dn6 = assign99820_e151996_d_n6;
        locals.var_exp_k_dn7 = assign99820_e151996_d_n7;
        locals.var_exp_k_dn8 = assign99820_e151996_d_n8;
        locals.var_exp_k_dn9 = assign99820_e151996_d_n9;
        locals.var_exp_k_dn10 = assign99820_e151996_d_n10;
        locals.var_exp_k_dn13 = assign99820_e151996_d_n13;
        locals.var_exp_k_rv = 0.0;

        let (assign99830_e152003, assign99830_e152003_d_n0, assign99830_e152003_d_n2, assign99830_e152003_d_n4, assign99830_e152003_d_n5, assign99830_e152003_d_n6, assign99830_e152003_d_n7, assign99830_e152003_d_n8, assign99830_e152003_d_n9, assign99830_e152003_d_n10, assign99830_e152003_d_n13,) = {
    if ((locals.var_guard2301 != 0.0) && (locals.var_guard2302 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_exp_k, locals.var_exp_k_dn0, locals.var_exp_k_dn2, locals.var_exp_k_dn4, locals.var_exp_k_dn5, locals.var_exp_k_dn6, locals.var_exp_k_dn7, locals.var_exp_k_dn8, locals.var_exp_k_dn9, locals.var_exp_k_dn10, locals.var_exp_k_dn13,)
    }
};
        locals.var_exp_k = assign99830_e152003;
        locals.var_exp_k_dn0 = assign99830_e152003_d_n0;
        locals.var_exp_k_dn2 = assign99830_e152003_d_n2;
        locals.var_exp_k_dn4 = assign99830_e152003_d_n4;
        locals.var_exp_k_dn5 = assign99830_e152003_d_n5;
        locals.var_exp_k_dn6 = assign99830_e152003_d_n6;
        locals.var_exp_k_dn7 = assign99830_e152003_d_n7;
        locals.var_exp_k_dn8 = assign99830_e152003_d_n8;
        locals.var_exp_k_dn9 = assign99830_e152003_d_n9;
        locals.var_exp_k_dn10 = assign99830_e152003_d_n10;
        locals.var_exp_k_dn13 = assign99830_e152003_d_n13;
        locals.var_exp_k_rv = 0.0;

        let assign99840_e152010: f64 = if ((p.p542 == 0.0) || (locals.var_vbd_jct < locals.var_v_ha)) { 1.0 } else { 0.0 };
        locals.var_guard2303 = assign99840_e152010;
        locals.var_guard2303_rv = 0.0;

        let (assign99850_e152018, assign99850_e152018_d_n0, assign99850_e152018_d_n2, assign99850_e152018_d_n4, assign99850_e152018_d_n5, assign99850_e152018_d_n6, assign99850_e152018_d_n7, assign99850_e152018_d_n8, assign99850_e152018_d_n9, assign99850_e152018_d_n10, assign99850_e152018_d_n13,) = {
    if ((locals.var_guard2301 != 0.0) && (locals.var_guard2303 != 0.0)) {
        let assign99850_e152016: f64 = (locals.var_exp_a * p.p541);
        (assign99850_e152016, (locals.var_exp_a_dn0 * p.p541), (locals.var_exp_a_dn2 * p.p541), (locals.var_exp_a_dn4 * p.p541), (locals.var_exp_a_dn5 * p.p541), (locals.var_exp_a_dn6 * p.p541), (locals.var_exp_a_dn7 * p.p541), (locals.var_exp_a_dn8 * p.p541), (locals.var_exp_a_dn9 * p.p541), (locals.var_exp_a_dn10 * p.p541), (locals.var_exp_a_dn13 * p.p541),)
    } else {
        (locals.var_exp_a2, locals.var_exp_a2_dn0, locals.var_exp_a2_dn2, locals.var_exp_a2_dn4, locals.var_exp_a2_dn5, locals.var_exp_a2_dn6, locals.var_exp_a2_dn7, locals.var_exp_a2_dn8, locals.var_exp_a2_dn9, locals.var_exp_a2_dn10, locals.var_exp_a2_dn13,)
    }
};
        locals.var_exp_a2 = assign99850_e152018;
        locals.var_exp_a2_dn0 = assign99850_e152018_d_n0;
        locals.var_exp_a2_dn2 = assign99850_e152018_d_n2;
        locals.var_exp_a2_dn4 = assign99850_e152018_d_n4;
        locals.var_exp_a2_dn5 = assign99850_e152018_d_n5;
        locals.var_exp_a2_dn6 = assign99850_e152018_d_n6;
        locals.var_exp_a2_dn7 = assign99850_e152018_d_n7;
        locals.var_exp_a2_dn8 = assign99850_e152018_d_n8;
        locals.var_exp_a2_dn9 = assign99850_e152018_d_n9;
        locals.var_exp_a2_dn10 = assign99850_e152018_d_n10;
        locals.var_exp_a2_dn13 = assign99850_e152018_d_n13;
        locals.var_exp_a2_rv = 0.0;

        let (assign99860_e152047, assign99860_e152047_d_n0, assign99860_e152047_d_n2, assign99860_e152047_d_n4, assign99860_e152047_d_n5, assign99860_e152047_d_n6, assign99860_e152047_d_n7, assign99860_e152047_d_n8, assign99860_e152047_d_n9, assign99860_e152047_d_n10, assign99860_e152047_d_n13,) = {
    if ((locals.var_guard2301 != 0.0) && (locals.var_guard2303 == 0.0)) {
        let assign99860_e152025: f64 = (locals.var_exp_a * p.p541);
        let assign99860_e152027: f64 = (-p.p542);
        let assign99860_e152030: f64 = (locals.var_vbd_jct - locals.var_v_ha);
        let assign99860_e152031: f64 = (assign99860_e152027 * assign99860_e152030);
        let assign99860_e152034: f64 = (locals.var_vbd_jct - locals.var_v_ha);
        let assign99860_e152035: f64 = (assign99860_e152031 * assign99860_e152034);
        let assign99860_e152039: f64 = (1.0 / locals.var_tratio);
        let assign99860_e152040: f64 = (assign99860_e152039).ln();
        let assign99860_e152041: f64 = (p.p548 * assign99860_e152040);
        let assign99860_e152042: f64 = (assign99860_e152041).exp();
        let assign99860_e152043: f64 = (assign99860_e152035 * assign99860_e152042);
        let assign99860_e152044: f64 = (assign99860_e152043).exp();
        let assign99860_e152045: f64 = (assign99860_e152025 * assign99860_e152044);
        (assign99860_e152045, (((locals.var_exp_a_dn0 * p.p541) * assign99860_e152044) + (assign99860_e152025 * (assign99860_e152044 * (((((assign99860_e152027 * (locals.var_vbd_jct_dn0 - locals.var_v_ha_dn0)) * assign99860_e152034) + (assign99860_e152031 * (locals.var_vbd_jct_dn0 - locals.var_v_ha_dn0))) * assign99860_e152042) + (assign99860_e152035 * (assign99860_e152042 * (p.p548 * ((-(locals.var_tratio_dn0 / (locals.var_tratio * locals.var_tratio))) / assign99860_e152039)))))))), (((locals.var_exp_a_dn2 * p.p541) * assign99860_e152044) + (assign99860_e152025 * (assign99860_e152044 * (((((assign99860_e152027 * (-locals.var_v_ha_dn2)) * assign99860_e152034) + (assign99860_e152031 * (-locals.var_v_ha_dn2))) * assign99860_e152042) + (assign99860_e152035 * (assign99860_e152042 * (p.p548 * ((-(locals.var_tratio_dn2 / (locals.var_tratio * locals.var_tratio))) / assign99860_e152039)))))))), (((locals.var_exp_a_dn4 * p.p541) * assign99860_e152044) + (assign99860_e152025 * (assign99860_e152044 * (((((assign99860_e152027 * (-locals.var_v_ha_dn4)) * assign99860_e152034) + (assign99860_e152031 * (-locals.var_v_ha_dn4))) * assign99860_e152042) + (assign99860_e152035 * (assign99860_e152042 * (p.p548 * ((-(locals.var_tratio_dn4 / (locals.var_tratio * locals.var_tratio))) / assign99860_e152039)))))))), (((locals.var_exp_a_dn5 * p.p541) * assign99860_e152044) + (assign99860_e152025 * (assign99860_e152044 * (((((assign99860_e152027 * (-locals.var_v_ha_dn5)) * assign99860_e152034) + (assign99860_e152031 * (-locals.var_v_ha_dn5))) * assign99860_e152042) + (assign99860_e152035 * (assign99860_e152042 * (p.p548 * ((-(locals.var_tratio_dn5 / (locals.var_tratio * locals.var_tratio))) / assign99860_e152039)))))))), (((locals.var_exp_a_dn6 * p.p541) * assign99860_e152044) + (assign99860_e152025 * (assign99860_e152044 * (((((assign99860_e152027 * (-locals.var_v_ha_dn6)) * assign99860_e152034) + (assign99860_e152031 * (-locals.var_v_ha_dn6))) * assign99860_e152042) + (assign99860_e152035 * (assign99860_e152042 * (p.p548 * ((-(locals.var_tratio_dn6 / (locals.var_tratio * locals.var_tratio))) / assign99860_e152039)))))))), (((locals.var_exp_a_dn7 * p.p541) * assign99860_e152044) + (assign99860_e152025 * (assign99860_e152044 * (((((assign99860_e152027 * (-locals.var_v_ha_dn7)) * assign99860_e152034) + (assign99860_e152031 * (-locals.var_v_ha_dn7))) * assign99860_e152042) + (assign99860_e152035 * (assign99860_e152042 * (p.p548 * ((-(locals.var_tratio_dn7 / (locals.var_tratio * locals.var_tratio))) / assign99860_e152039)))))))), (((locals.var_exp_a_dn8 * p.p541) * assign99860_e152044) + (assign99860_e152025 * (assign99860_e152044 * (((((assign99860_e152027 * (-locals.var_v_ha_dn8)) * assign99860_e152034) + (assign99860_e152031 * (-locals.var_v_ha_dn8))) * assign99860_e152042) + (assign99860_e152035 * (assign99860_e152042 * (p.p548 * ((-(locals.var_tratio_dn8 / (locals.var_tratio * locals.var_tratio))) / assign99860_e152039)))))))), (((locals.var_exp_a_dn9 * p.p541) * assign99860_e152044) + (assign99860_e152025 * (assign99860_e152044 * (((((assign99860_e152027 * (locals.var_vbd_jct_dn9 - locals.var_v_ha_dn9)) * assign99860_e152034) + (assign99860_e152031 * (locals.var_vbd_jct_dn9 - locals.var_v_ha_dn9))) * assign99860_e152042) + (assign99860_e152035 * (assign99860_e152042 * (p.p548 * ((-(locals.var_tratio_dn9 / (locals.var_tratio * locals.var_tratio))) / assign99860_e152039)))))))), (((locals.var_exp_a_dn10 * p.p541) * assign99860_e152044) + (assign99860_e152025 * (assign99860_e152044 * (((((assign99860_e152027 * (-locals.var_v_ha_dn10)) * assign99860_e152034) + (assign99860_e152031 * (-locals.var_v_ha_dn10))) * assign99860_e152042) + (assign99860_e152035 * (assign99860_e152042 * (p.p548 * ((-(locals.var_tratio_dn10 / (locals.var_tratio * locals.var_tratio))) / assign99860_e152039)))))))), (((locals.var_exp_a_dn13 * p.p541) * assign99860_e152044) + (assign99860_e152025 * (assign99860_e152044 * (((((assign99860_e152027 * (-locals.var_v_ha_dn13)) * assign99860_e152034) + (assign99860_e152031 * (-locals.var_v_ha_dn13))) * assign99860_e152042) + (assign99860_e152035 * (assign99860_e152042 * (p.p548 * ((-(locals.var_tratio_dn13 / (locals.var_tratio * locals.var_tratio))) / assign99860_e152039)))))))),)
    } else {
        (locals.var_exp_a2, locals.var_exp_a2_dn0, locals.var_exp_a2_dn2, locals.var_exp_a2_dn4, locals.var_exp_a2_dn5, locals.var_exp_a2_dn6, locals.var_exp_a2_dn7, locals.var_exp_a2_dn8, locals.var_exp_a2_dn9, locals.var_exp_a2_dn10, locals.var_exp_a2_dn13,)
    }
};
        locals.var_exp_a2 = assign99860_e152047;
        locals.var_exp_a2_dn0 = assign99860_e152047_d_n0;
        locals.var_exp_a2_dn2 = assign99860_e152047_d_n2;
        locals.var_exp_a2_dn4 = assign99860_e152047_d_n4;
        locals.var_exp_a2_dn5 = assign99860_e152047_d_n5;
        locals.var_exp_a2_dn6 = assign99860_e152047_d_n6;
        locals.var_exp_a2_dn7 = assign99860_e152047_d_n7;
        locals.var_exp_a2_dn8 = assign99860_e152047_d_n8;
        locals.var_exp_a2_dn9 = assign99860_e152047_d_n9;
        locals.var_exp_a2_dn10 = assign99860_e152047_d_n10;
        locals.var_exp_a2_dn13 = assign99860_e152047_d_n13;
        locals.var_exp_a2_rv = 0.0;

        let (assign99870_e152056, assign99870_e152056_d_n0, assign99870_e152056_d_n2, assign99870_e152056_d_n4, assign99870_e152056_d_n5, assign99870_e152056_d_n6, assign99870_e152056_d_n7, assign99870_e152056_d_n8, assign99870_e152056_d_n9, assign99870_e152056_d_n10, assign99870_e152056_d_n13,) = {
    if (locals.var_guard2301 != 0.0) {
        let (assign99870_e152054, assign99870_e152054_d_n0, assign99870_e152054_d_n2, assign99870_e152054_d_n4, assign99870_e152054_d_n5, assign99870_e152054_d_n6, assign99870_e152054_d_n7, assign99870_e152054_d_n8, assign99870_e152054_d_n9, assign99870_e152054_d_n10, assign99870_e152054_d_n13,) = {
            if (locals.var_exp_a2 > 1e20) {
                (1e20, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                (locals.var_exp_a2, locals.var_exp_a2_dn0, locals.var_exp_a2_dn2, locals.var_exp_a2_dn4, locals.var_exp_a2_dn5, locals.var_exp_a2_dn6, locals.var_exp_a2_dn7, locals.var_exp_a2_dn8, locals.var_exp_a2_dn9, locals.var_exp_a2_dn10, locals.var_exp_a2_dn13,)
            }
        };
        (assign99870_e152054, assign99870_e152054_d_n0, assign99870_e152054_d_n2, assign99870_e152054_d_n4, assign99870_e152054_d_n5, assign99870_e152054_d_n6, assign99870_e152054_d_n7, assign99870_e152054_d_n8, assign99870_e152054_d_n9, assign99870_e152054_d_n10, assign99870_e152054_d_n13,)
    } else {
        (locals.var_exp_a2, locals.var_exp_a2_dn0, locals.var_exp_a2_dn2, locals.var_exp_a2_dn4, locals.var_exp_a2_dn5, locals.var_exp_a2_dn6, locals.var_exp_a2_dn7, locals.var_exp_a2_dn8, locals.var_exp_a2_dn9, locals.var_exp_a2_dn10, locals.var_exp_a2_dn13,)
    }
};
        locals.var_exp_a2 = assign99870_e152056;
        locals.var_exp_a2_dn0 = assign99870_e152056_d_n0;
        locals.var_exp_a2_dn2 = assign99870_e152056_d_n2;
        locals.var_exp_a2_dn4 = assign99870_e152056_d_n4;
        locals.var_exp_a2_dn5 = assign99870_e152056_d_n5;
        locals.var_exp_a2_dn6 = assign99870_e152056_d_n6;
        locals.var_exp_a2_dn7 = assign99870_e152056_d_n7;
        locals.var_exp_a2_dn8 = assign99870_e152056_d_n8;
        locals.var_exp_a2_dn9 = assign99870_e152056_d_n9;
        locals.var_exp_a2_dn10 = assign99870_e152056_d_n10;
        locals.var_exp_a2_dn13 = assign99870_e152056_d_n13;
        locals.var_exp_a2_rv = 0.0;

        let (assign99880_e152062, assign99880_e152062_d_n0, assign99880_e152062_d_n2, assign99880_e152062_d_n4, assign99880_e152062_d_n5, assign99880_e152062_d_n6, assign99880_e152062_d_n7, assign99880_e152062_d_n8, assign99880_e152062_d_n9, assign99880_e152062_d_n10, assign99880_e152062_d_n13,) = {
    if (locals.var_guard2301 != 0.0) {
        let assign99880_e152060: f64 = (locals.var_pn0 * locals.var_exp_a2);
        (assign99880_e152060, ((locals.var_pn0_dn0 * locals.var_exp_a2) + (locals.var_pn0 * locals.var_exp_a2_dn0)), ((locals.var_pn0_dn2 * locals.var_exp_a2) + (locals.var_pn0 * locals.var_exp_a2_dn2)), ((locals.var_pn0_dn4 * locals.var_exp_a2) + (locals.var_pn0 * locals.var_exp_a2_dn4)), ((locals.var_pn0_dn5 * locals.var_exp_a2) + (locals.var_pn0 * locals.var_exp_a2_dn5)), ((locals.var_pn0_dn6 * locals.var_exp_a2) + (locals.var_pn0 * locals.var_exp_a2_dn6)), ((locals.var_pn0_dn7 * locals.var_exp_a2) + (locals.var_pn0 * locals.var_exp_a2_dn7)), ((locals.var_pn0_dn8 * locals.var_exp_a2) + (locals.var_pn0 * locals.var_exp_a2_dn8)), ((locals.var_pn0_dn9 * locals.var_exp_a2) + (locals.var_pn0 * locals.var_exp_a2_dn9)), ((locals.var_pn0_dn10 * locals.var_exp_a2) + (locals.var_pn0 * locals.var_exp_a2_dn10)), ((locals.var_pn0_dn13 * locals.var_exp_a2) + (locals.var_pn0 * locals.var_exp_a2_dn13)),)
    } else {
        (locals.var_p_na, locals.var_p_na_dn0, locals.var_p_na_dn2, locals.var_p_na_dn4, locals.var_p_na_dn5, locals.var_p_na_dn6, locals.var_p_na_dn7, locals.var_p_na_dn8, locals.var_p_na_dn9, locals.var_p_na_dn10, locals.var_p_na_dn13,)
    }
};
        locals.var_p_na = assign99880_e152062;
        locals.var_p_na_dn0 = assign99880_e152062_d_n0;
        locals.var_p_na_dn2 = assign99880_e152062_d_n2;
        locals.var_p_na_dn4 = assign99880_e152062_d_n4;
        locals.var_p_na_dn5 = assign99880_e152062_d_n5;
        locals.var_p_na_dn6 = assign99880_e152062_d_n6;
        locals.var_p_na_dn7 = assign99880_e152062_d_n7;
        locals.var_p_na_dn8 = assign99880_e152062_d_n8;
        locals.var_p_na_dn9 = assign99880_e152062_d_n9;
        locals.var_p_na_dn10 = assign99880_e152062_d_n10;
        locals.var_p_na_dn13 = assign99880_e152062_d_n13;
        locals.var_p_na_rv = 0.0;

        let (assign99890_e152072, assign99890_e152072_d_n0, assign99890_e152072_d_n2, assign99890_e152072_d_n4, assign99890_e152072_d_n5, assign99890_e152072_d_n6, assign99890_e152072_d_n7, assign99890_e152072_d_n8, assign99890_e152072_d_n9, assign99890_e152072_d_n10, assign99890_e152072_d_n13,) = {
    if (locals.var_guard2301 != 0.0) {
        let assign99890_e152066: f64 = (1.6021918e-19 * p.p13);
        let assign99890_e152069: f64 = (locals.var_p_na - locals.var_pn0);
        let assign99890_e152070: f64 = (assign99890_e152066 * assign99890_e152069);
        (assign99890_e152070, (assign99890_e152066 * (locals.var_p_na_dn0 - locals.var_pn0_dn0)), (assign99890_e152066 * (locals.var_p_na_dn2 - locals.var_pn0_dn2)), (assign99890_e152066 * (locals.var_p_na_dn4 - locals.var_pn0_dn4)), (assign99890_e152066 * (locals.var_p_na_dn5 - locals.var_pn0_dn5)), (assign99890_e152066 * (locals.var_p_na_dn6 - locals.var_pn0_dn6)), (assign99890_e152066 * (locals.var_p_na_dn7 - locals.var_pn0_dn7)), (assign99890_e152066 * (locals.var_p_na_dn8 - locals.var_pn0_dn8)), (assign99890_e152066 * (locals.var_p_na_dn9 - locals.var_pn0_dn9)), (assign99890_e152066 * (locals.var_p_na_dn10 - locals.var_pn0_dn10)), (assign99890_e152066 * (locals.var_p_na_dn13 - locals.var_pn0_dn13)),)
    } else {
        (locals.var_q_pexa, locals.var_q_pexa_dn0, locals.var_q_pexa_dn2, locals.var_q_pexa_dn4, locals.var_q_pexa_dn5, locals.var_q_pexa_dn6, locals.var_q_pexa_dn7, locals.var_q_pexa_dn8, locals.var_q_pexa_dn9, locals.var_q_pexa_dn10, locals.var_q_pexa_dn13,)
    }
};
        locals.var_q_pexa = assign99890_e152072;
        locals.var_q_pexa_dn0 = assign99890_e152072_d_n0;
        locals.var_q_pexa_dn2 = assign99890_e152072_d_n2;
        locals.var_q_pexa_dn4 = assign99890_e152072_d_n4;
        locals.var_q_pexa_dn5 = assign99890_e152072_d_n5;
        locals.var_q_pexa_dn6 = assign99890_e152072_d_n6;
        locals.var_q_pexa_dn7 = assign99890_e152072_d_n7;
        locals.var_q_pexa_dn8 = assign99890_e152072_d_n8;
        locals.var_q_pexa_dn9 = assign99890_e152072_d_n9;
        locals.var_q_pexa_dn10 = assign99890_e152072_d_n10;
        locals.var_q_pexa_dn13 = assign99890_e152072_d_n13;
        locals.var_q_pexa_rv = 0.0;

        let assign99900_e152075: f64 = if p.p543 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2304 = assign99900_e152075;
        locals.var_guard2304_rv = 0.0;

        let (assign99910_e152083, assign99910_e152083_d_n0, assign99910_e152083_d_n2, assign99910_e152083_d_n4, assign99910_e152083_d_n5, assign99910_e152083_d_n6, assign99910_e152083_d_n7, assign99910_e152083_d_n8, assign99910_e152083_d_n9, assign99910_e152083_d_n10, assign99910_e152083_d_n13,) = {
    if ((locals.var_guard2301 != 0.0) && (locals.var_guard2304 != 0.0)) {
        let assign99910_e152081: f64 = (locals.var_q_pexa * p.p543);
        (assign99910_e152081, (locals.var_q_pexa_dn0 * p.p543), (locals.var_q_pexa_dn2 * p.p543), (locals.var_q_pexa_dn4 * p.p543), (locals.var_q_pexa_dn5 * p.p543), (locals.var_q_pexa_dn6 * p.p543), (locals.var_q_pexa_dn7 * p.p543), (locals.var_q_pexa_dn8 * p.p543), (locals.var_q_pexa_dn9 * p.p543), (locals.var_q_pexa_dn10 * p.p543), (locals.var_q_pexa_dn13 * p.p543),)
    } else {
        (locals.var_q_qs_a, locals.var_q_qs_a_dn0, locals.var_q_qs_a_dn2, locals.var_q_qs_a_dn4, locals.var_q_qs_a_dn5, locals.var_q_qs_a_dn6, locals.var_q_qs_a_dn7, locals.var_q_qs_a_dn8, locals.var_q_qs_a_dn9, locals.var_q_qs_a_dn10, locals.var_q_qs_a_dn13,)
    }
};
        locals.var_q_qs_a = assign99910_e152083;
        locals.var_q_qs_a_dn0 = assign99910_e152083_d_n0;
        locals.var_q_qs_a_dn2 = assign99910_e152083_d_n2;
        locals.var_q_qs_a_dn4 = assign99910_e152083_d_n4;
        locals.var_q_qs_a_dn5 = assign99910_e152083_d_n5;
        locals.var_q_qs_a_dn6 = assign99910_e152083_d_n6;
        locals.var_q_qs_a_dn7 = assign99910_e152083_d_n7;
        locals.var_q_qs_a_dn8 = assign99910_e152083_d_n8;
        locals.var_q_qs_a_dn9 = assign99910_e152083_d_n9;
        locals.var_q_qs_a_dn10 = assign99910_e152083_d_n10;
        locals.var_q_qs_a_dn13 = assign99910_e152083_d_n13;
        locals.var_q_qs_a_rv = 0.0;

        let (assign99920_e152091, assign99920_e152091_d_n15,) = {
    if ((locals.var_guard2301 != 0.0) && (locals.var_guard2304 != 0.0)) {
        let assign99920_e152089: f64 = (p.p543 * (nv15 - 0.0));
        (assign99920_e152089, p.p543,)
    } else {
        (locals.var_q_nqs_a, locals.var_q_nqs_a_dn15,)
    }
};
        locals.var_q_nqs_a = assign99920_e152091;
        locals.var_q_nqs_a_dn15 = assign99920_e152091_d_n15;
        locals.var_q_nqs_a_rv = 0.0;

        let (assign99930_e152101, assign99930_e152101_d_n0, assign99930_e152101_d_n2, assign99930_e152101_d_n4, assign99930_e152101_d_n5, assign99930_e152101_d_n6, assign99930_e152101_d_n7, assign99930_e152101_d_n8, assign99930_e152101_d_n9, assign99930_e152101_d_n10, assign99930_e152101_d_n13, assign99930_e152101_d_n15,) = {
    if ((locals.var_guard2301 != 0.0) && (locals.var_guard2304 != 0.0)) {
        let assign99930_e152097: f64 = (locals.var_q_nqs_a - locals.var_q_qs_a);
        let assign99930_e152099: f64 = (assign99930_e152097 / p.p543);
        (assign99930_e152099, ((-locals.var_q_qs_a_dn0) / p.p543), ((-locals.var_q_qs_a_dn2) / p.p543), ((-locals.var_q_qs_a_dn4) / p.p543), ((-locals.var_q_qs_a_dn5) / p.p543), ((-locals.var_q_qs_a_dn6) / p.p543), ((-locals.var_q_qs_a_dn7) / p.p543), ((-locals.var_q_qs_a_dn8) / p.p543), ((-locals.var_q_qs_a_dn9) / p.p543), ((-locals.var_q_qs_a_dn10) / p.p543), ((-locals.var_q_qs_a_dn13) / p.p543), (locals.var_q_nqs_a_dn15 / p.p543),)
    } else {
        (locals.var_inqs0_a, locals.var_inqs0_a_dn0, locals.var_inqs0_a_dn2, locals.var_inqs0_a_dn4, locals.var_inqs0_a_dn5, locals.var_inqs0_a_dn6, locals.var_inqs0_a_dn7, locals.var_inqs0_a_dn8, locals.var_inqs0_a_dn9, locals.var_inqs0_a_dn10, locals.var_inqs0_a_dn13, locals.var_inqs0_a_dn15,)
    }
};
        locals.var_inqs0_a = assign99930_e152101;
        locals.var_inqs0_a_dn0 = assign99930_e152101_d_n0;
        locals.var_inqs0_a_dn2 = assign99930_e152101_d_n2;
        locals.var_inqs0_a_dn4 = assign99930_e152101_d_n4;
        locals.var_inqs0_a_dn5 = assign99930_e152101_d_n5;
        locals.var_inqs0_a_dn6 = assign99930_e152101_d_n6;
        locals.var_inqs0_a_dn7 = assign99930_e152101_d_n7;
        locals.var_inqs0_a_dn8 = assign99930_e152101_d_n8;
        locals.var_inqs0_a_dn9 = assign99930_e152101_d_n9;
        locals.var_inqs0_a_dn10 = assign99930_e152101_d_n10;
        locals.var_inqs0_a_dn13 = assign99930_e152101_d_n13;
        locals.var_inqs0_a_dn15 = assign99930_e152101_d_n15;
        locals.var_inqs0_a_rv = 0.0;

        let (assign99940_e152109, assign99940_e152109_d_n0, assign99940_e152109_d_n2, assign99940_e152109_d_n4, assign99940_e152109_d_n5, assign99940_e152109_d_n6, assign99940_e152109_d_n7, assign99940_e152109_d_n8, assign99940_e152109_d_n9, assign99940_e152109_d_n10, assign99940_e152109_d_n13, assign99940_e152109_d_n15,) = {
    if ((locals.var_guard2301 != 0.0) && (locals.var_guard2304 != 0.0)) {
        let assign99940_e152107: f64 = (locals.var_q_nqs_a / p.p543);
        (assign99940_e152107, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, (locals.var_q_nqs_a_dn15 / p.p543),)
    } else {
        (locals.var_q_pexa_nqs, locals.var_q_pexa_nqs_dn0, locals.var_q_pexa_nqs_dn2, locals.var_q_pexa_nqs_dn4, locals.var_q_pexa_nqs_dn5, locals.var_q_pexa_nqs_dn6, locals.var_q_pexa_nqs_dn7, locals.var_q_pexa_nqs_dn8, locals.var_q_pexa_nqs_dn9, locals.var_q_pexa_nqs_dn10, locals.var_q_pexa_nqs_dn13, locals.var_q_pexa_nqs_dn15,)
    }
};
        locals.var_q_pexa_nqs = assign99940_e152109;
        locals.var_q_pexa_nqs_dn0 = assign99940_e152109_d_n0;
        locals.var_q_pexa_nqs_dn2 = assign99940_e152109_d_n2;
        locals.var_q_pexa_nqs_dn4 = assign99940_e152109_d_n4;
        locals.var_q_pexa_nqs_dn5 = assign99940_e152109_d_n5;
        locals.var_q_pexa_nqs_dn6 = assign99940_e152109_d_n6;
        locals.var_q_pexa_nqs_dn7 = assign99940_e152109_d_n7;
        locals.var_q_pexa_nqs_dn8 = assign99940_e152109_d_n8;
        locals.var_q_pexa_nqs_dn9 = assign99940_e152109_d_n9;
        locals.var_q_pexa_nqs_dn10 = assign99940_e152109_d_n10;
        locals.var_q_pexa_nqs_dn13 = assign99940_e152109_d_n13;
        locals.var_q_pexa_nqs_dn15 = assign99940_e152109_d_n15;
        locals.var_q_pexa_nqs_rv = 0.0;

        let (assign99950_e152116, assign99950_e152116_d_n0, assign99950_e152116_d_n2, assign99950_e152116_d_n4, assign99950_e152116_d_n5, assign99950_e152116_d_n6, assign99950_e152116_d_n7, assign99950_e152116_d_n8, assign99950_e152116_d_n9, assign99950_e152116_d_n10, assign99950_e152116_d_n13,) = {
    if ((locals.var_guard2301 != 0.0) && (locals.var_guard2304 == 0.0)) {
        (locals.var_q_pexa, locals.var_q_pexa_dn0, locals.var_q_pexa_dn2, locals.var_q_pexa_dn4, locals.var_q_pexa_dn5, locals.var_q_pexa_dn6, locals.var_q_pexa_dn7, locals.var_q_pexa_dn8, locals.var_q_pexa_dn9, locals.var_q_pexa_dn10, locals.var_q_pexa_dn13,)
    } else {
        (locals.var_q_qs_a, locals.var_q_qs_a_dn0, locals.var_q_qs_a_dn2, locals.var_q_qs_a_dn4, locals.var_q_qs_a_dn5, locals.var_q_qs_a_dn6, locals.var_q_qs_a_dn7, locals.var_q_qs_a_dn8, locals.var_q_qs_a_dn9, locals.var_q_qs_a_dn10, locals.var_q_qs_a_dn13,)
    }
};
        locals.var_q_qs_a = assign99950_e152116;
        locals.var_q_qs_a_dn0 = assign99950_e152116_d_n0;
        locals.var_q_qs_a_dn2 = assign99950_e152116_d_n2;
        locals.var_q_qs_a_dn4 = assign99950_e152116_d_n4;
        locals.var_q_qs_a_dn5 = assign99950_e152116_d_n5;
        locals.var_q_qs_a_dn6 = assign99950_e152116_d_n6;
        locals.var_q_qs_a_dn7 = assign99950_e152116_d_n7;
        locals.var_q_qs_a_dn8 = assign99950_e152116_d_n8;
        locals.var_q_qs_a_dn9 = assign99950_e152116_d_n9;
        locals.var_q_qs_a_dn10 = assign99950_e152116_d_n10;
        locals.var_q_qs_a_dn13 = assign99950_e152116_d_n13;
        locals.var_q_qs_a_rv = 0.0;

        let (assign99960_e152123, assign99960_e152123_d_n0, assign99960_e152123_d_n2, assign99960_e152123_d_n4, assign99960_e152123_d_n5, assign99960_e152123_d_n6, assign99960_e152123_d_n7, assign99960_e152123_d_n8, assign99960_e152123_d_n9, assign99960_e152123_d_n10, assign99960_e152123_d_n13, assign99960_e152123_d_n15,) = {
    if ((locals.var_guard2301 != 0.0) && (locals.var_guard2304 == 0.0)) {
        (locals.var_q_qs_a, locals.var_q_qs_a_dn0, locals.var_q_qs_a_dn2, locals.var_q_qs_a_dn4, locals.var_q_qs_a_dn5, locals.var_q_qs_a_dn6, locals.var_q_qs_a_dn7, locals.var_q_qs_a_dn8, locals.var_q_qs_a_dn9, locals.var_q_qs_a_dn10, locals.var_q_qs_a_dn13, 0.0,)
    } else {
        (locals.var_q_pexa_nqs, locals.var_q_pexa_nqs_dn0, locals.var_q_pexa_nqs_dn2, locals.var_q_pexa_nqs_dn4, locals.var_q_pexa_nqs_dn5, locals.var_q_pexa_nqs_dn6, locals.var_q_pexa_nqs_dn7, locals.var_q_pexa_nqs_dn8, locals.var_q_pexa_nqs_dn9, locals.var_q_pexa_nqs_dn10, locals.var_q_pexa_nqs_dn13, locals.var_q_pexa_nqs_dn15,)
    }
};
        locals.var_q_pexa_nqs = assign99960_e152123;
        locals.var_q_pexa_nqs_dn0 = assign99960_e152123_d_n0;
        locals.var_q_pexa_nqs_dn2 = assign99960_e152123_d_n2;
        locals.var_q_pexa_nqs_dn4 = assign99960_e152123_d_n4;
        locals.var_q_pexa_nqs_dn5 = assign99960_e152123_d_n5;
        locals.var_q_pexa_nqs_dn6 = assign99960_e152123_d_n6;
        locals.var_q_pexa_nqs_dn7 = assign99960_e152123_d_n7;
        locals.var_q_pexa_nqs_dn8 = assign99960_e152123_d_n8;
        locals.var_q_pexa_nqs_dn9 = assign99960_e152123_d_n9;
        locals.var_q_pexa_nqs_dn10 = assign99960_e152123_d_n10;
        locals.var_q_pexa_nqs_dn13 = assign99960_e152123_d_n13;
        locals.var_q_pexa_nqs_dn15 = assign99960_e152123_d_n15;
        locals.var_q_pexa_nqs_rv = 0.0;

        let assign99970_e152130: f64 = if ((p.p542 == 0.0) || (locals.var_vbd_jct < locals.var_v_hk)) { 1.0 } else { 0.0 };
        locals.var_guard2305 = assign99970_e152130;
        locals.var_guard2305_rv = 0.0;

        let (assign99980_e152138, assign99980_e152138_d_n0, assign99980_e152138_d_n2, assign99980_e152138_d_n4, assign99980_e152138_d_n5, assign99980_e152138_d_n6, assign99980_e152138_d_n7, assign99980_e152138_d_n8, assign99980_e152138_d_n9, assign99980_e152138_d_n10, assign99980_e152138_d_n13,) = {
    if ((locals.var_guard2301 != 0.0) && (locals.var_guard2305 != 0.0)) {
        let assign99980_e152136: f64 = (locals.var_exp_k * p.p541);
        (assign99980_e152136, (locals.var_exp_k_dn0 * p.p541), (locals.var_exp_k_dn2 * p.p541), (locals.var_exp_k_dn4 * p.p541), (locals.var_exp_k_dn5 * p.p541), (locals.var_exp_k_dn6 * p.p541), (locals.var_exp_k_dn7 * p.p541), (locals.var_exp_k_dn8 * p.p541), (locals.var_exp_k_dn9 * p.p541), (locals.var_exp_k_dn10 * p.p541), (locals.var_exp_k_dn13 * p.p541),)
    } else {
        (locals.var_exp_k2, locals.var_exp_k2_dn0, locals.var_exp_k2_dn2, locals.var_exp_k2_dn4, locals.var_exp_k2_dn5, locals.var_exp_k2_dn6, locals.var_exp_k2_dn7, locals.var_exp_k2_dn8, locals.var_exp_k2_dn9, locals.var_exp_k2_dn10, locals.var_exp_k2_dn13,)
    }
};
        locals.var_exp_k2 = assign99980_e152138;
        locals.var_exp_k2_dn0 = assign99980_e152138_d_n0;
        locals.var_exp_k2_dn2 = assign99980_e152138_d_n2;
        locals.var_exp_k2_dn4 = assign99980_e152138_d_n4;
        locals.var_exp_k2_dn5 = assign99980_e152138_d_n5;
        locals.var_exp_k2_dn6 = assign99980_e152138_d_n6;
        locals.var_exp_k2_dn7 = assign99980_e152138_d_n7;
        locals.var_exp_k2_dn8 = assign99980_e152138_d_n8;
        locals.var_exp_k2_dn9 = assign99980_e152138_d_n9;
        locals.var_exp_k2_dn10 = assign99980_e152138_d_n10;
        locals.var_exp_k2_dn13 = assign99980_e152138_d_n13;
        locals.var_exp_k2_rv = 0.0;

        let (assign99990_e152167, assign99990_e152167_d_n0, assign99990_e152167_d_n2, assign99990_e152167_d_n4, assign99990_e152167_d_n5, assign99990_e152167_d_n6, assign99990_e152167_d_n7, assign99990_e152167_d_n8, assign99990_e152167_d_n9, assign99990_e152167_d_n10, assign99990_e152167_d_n13,) = {
    if ((locals.var_guard2301 != 0.0) && (locals.var_guard2305 == 0.0)) {
        let assign99990_e152145: f64 = (locals.var_exp_k * p.p541);
        let assign99990_e152147: f64 = (-p.p542);
        let assign99990_e152150: f64 = (locals.var_vbd_jct - locals.var_v_hk);
        let assign99990_e152151: f64 = (assign99990_e152147 * assign99990_e152150);
        let assign99990_e152154: f64 = (locals.var_vbd_jct - locals.var_v_hk);
        let assign99990_e152155: f64 = (assign99990_e152151 * assign99990_e152154);
        let assign99990_e152159: f64 = (1.0 / locals.var_tratio);
        let assign99990_e152160: f64 = (assign99990_e152159).ln();
        let assign99990_e152161: f64 = (p.p548 * assign99990_e152160);
        let assign99990_e152162: f64 = (assign99990_e152161).exp();
        let assign99990_e152163: f64 = (assign99990_e152155 * assign99990_e152162);
        let assign99990_e152164: f64 = (assign99990_e152163).exp();
        let assign99990_e152165: f64 = (assign99990_e152145 * assign99990_e152164);
        (assign99990_e152165, (((locals.var_exp_k_dn0 * p.p541) * assign99990_e152164) + (assign99990_e152145 * (assign99990_e152164 * (((((assign99990_e152147 * (locals.var_vbd_jct_dn0 - locals.var_v_hk_dn0)) * assign99990_e152154) + (assign99990_e152151 * (locals.var_vbd_jct_dn0 - locals.var_v_hk_dn0))) * assign99990_e152162) + (assign99990_e152155 * (assign99990_e152162 * (p.p548 * ((-(locals.var_tratio_dn0 / (locals.var_tratio * locals.var_tratio))) / assign99990_e152159)))))))), (((locals.var_exp_k_dn2 * p.p541) * assign99990_e152164) + (assign99990_e152145 * (assign99990_e152164 * (((((assign99990_e152147 * (-locals.var_v_hk_dn2)) * assign99990_e152154) + (assign99990_e152151 * (-locals.var_v_hk_dn2))) * assign99990_e152162) + (assign99990_e152155 * (assign99990_e152162 * (p.p548 * ((-(locals.var_tratio_dn2 / (locals.var_tratio * locals.var_tratio))) / assign99990_e152159)))))))), (((locals.var_exp_k_dn4 * p.p541) * assign99990_e152164) + (assign99990_e152145 * (assign99990_e152164 * (((((assign99990_e152147 * (-locals.var_v_hk_dn4)) * assign99990_e152154) + (assign99990_e152151 * (-locals.var_v_hk_dn4))) * assign99990_e152162) + (assign99990_e152155 * (assign99990_e152162 * (p.p548 * ((-(locals.var_tratio_dn4 / (locals.var_tratio * locals.var_tratio))) / assign99990_e152159)))))))), (((locals.var_exp_k_dn5 * p.p541) * assign99990_e152164) + (assign99990_e152145 * (assign99990_e152164 * (((((assign99990_e152147 * (-locals.var_v_hk_dn5)) * assign99990_e152154) + (assign99990_e152151 * (-locals.var_v_hk_dn5))) * assign99990_e152162) + (assign99990_e152155 * (assign99990_e152162 * (p.p548 * ((-(locals.var_tratio_dn5 / (locals.var_tratio * locals.var_tratio))) / assign99990_e152159)))))))), (((locals.var_exp_k_dn6 * p.p541) * assign99990_e152164) + (assign99990_e152145 * (assign99990_e152164 * (((((assign99990_e152147 * (-locals.var_v_hk_dn6)) * assign99990_e152154) + (assign99990_e152151 * (-locals.var_v_hk_dn6))) * assign99990_e152162) + (assign99990_e152155 * (assign99990_e152162 * (p.p548 * ((-(locals.var_tratio_dn6 / (locals.var_tratio * locals.var_tratio))) / assign99990_e152159)))))))), (((locals.var_exp_k_dn7 * p.p541) * assign99990_e152164) + (assign99990_e152145 * (assign99990_e152164 * (((((assign99990_e152147 * (-locals.var_v_hk_dn7)) * assign99990_e152154) + (assign99990_e152151 * (-locals.var_v_hk_dn7))) * assign99990_e152162) + (assign99990_e152155 * (assign99990_e152162 * (p.p548 * ((-(locals.var_tratio_dn7 / (locals.var_tratio * locals.var_tratio))) / assign99990_e152159)))))))), (((locals.var_exp_k_dn8 * p.p541) * assign99990_e152164) + (assign99990_e152145 * (assign99990_e152164 * (((((assign99990_e152147 * (-locals.var_v_hk_dn8)) * assign99990_e152154) + (assign99990_e152151 * (-locals.var_v_hk_dn8))) * assign99990_e152162) + (assign99990_e152155 * (assign99990_e152162 * (p.p548 * ((-(locals.var_tratio_dn8 / (locals.var_tratio * locals.var_tratio))) / assign99990_e152159)))))))), (((locals.var_exp_k_dn9 * p.p541) * assign99990_e152164) + (assign99990_e152145 * (assign99990_e152164 * (((((assign99990_e152147 * (locals.var_vbd_jct_dn9 - locals.var_v_hk_dn9)) * assign99990_e152154) + (assign99990_e152151 * (locals.var_vbd_jct_dn9 - locals.var_v_hk_dn9))) * assign99990_e152162) + (assign99990_e152155 * (assign99990_e152162 * (p.p548 * ((-(locals.var_tratio_dn9 / (locals.var_tratio * locals.var_tratio))) / assign99990_e152159)))))))), (((locals.var_exp_k_dn10 * p.p541) * assign99990_e152164) + (assign99990_e152145 * (assign99990_e152164 * (((((assign99990_e152147 * (-locals.var_v_hk_dn10)) * assign99990_e152154) + (assign99990_e152151 * (-locals.var_v_hk_dn10))) * assign99990_e152162) + (assign99990_e152155 * (assign99990_e152162 * (p.p548 * ((-(locals.var_tratio_dn10 / (locals.var_tratio * locals.var_tratio))) / assign99990_e152159)))))))), (((locals.var_exp_k_dn13 * p.p541) * assign99990_e152164) + (assign99990_e152145 * (assign99990_e152164 * (((((assign99990_e152147 * (-locals.var_v_hk_dn13)) * assign99990_e152154) + (assign99990_e152151 * (-locals.var_v_hk_dn13))) * assign99990_e152162) + (assign99990_e152155 * (assign99990_e152162 * (p.p548 * ((-(locals.var_tratio_dn13 / (locals.var_tratio * locals.var_tratio))) / assign99990_e152159)))))))),)
    } else {
        (locals.var_exp_k2, locals.var_exp_k2_dn0, locals.var_exp_k2_dn2, locals.var_exp_k2_dn4, locals.var_exp_k2_dn5, locals.var_exp_k2_dn6, locals.var_exp_k2_dn7, locals.var_exp_k2_dn8, locals.var_exp_k2_dn9, locals.var_exp_k2_dn10, locals.var_exp_k2_dn13,)
    }
};
        locals.var_exp_k2 = assign99990_e152167;
        locals.var_exp_k2_dn0 = assign99990_e152167_d_n0;
        locals.var_exp_k2_dn2 = assign99990_e152167_d_n2;
        locals.var_exp_k2_dn4 = assign99990_e152167_d_n4;
        locals.var_exp_k2_dn5 = assign99990_e152167_d_n5;
        locals.var_exp_k2_dn6 = assign99990_e152167_d_n6;
        locals.var_exp_k2_dn7 = assign99990_e152167_d_n7;
        locals.var_exp_k2_dn8 = assign99990_e152167_d_n8;
        locals.var_exp_k2_dn9 = assign99990_e152167_d_n9;
        locals.var_exp_k2_dn10 = assign99990_e152167_d_n10;
        locals.var_exp_k2_dn13 = assign99990_e152167_d_n13;
        locals.var_exp_k2_rv = 0.0;

        let (assign100000_e152176, assign100000_e152176_d_n0, assign100000_e152176_d_n2, assign100000_e152176_d_n4, assign100000_e152176_d_n5, assign100000_e152176_d_n6, assign100000_e152176_d_n7, assign100000_e152176_d_n8, assign100000_e152176_d_n9, assign100000_e152176_d_n10, assign100000_e152176_d_n13,) = {
    if (locals.var_guard2301 != 0.0) {
        let (assign100000_e152174, assign100000_e152174_d_n0, assign100000_e152174_d_n2, assign100000_e152174_d_n4, assign100000_e152174_d_n5, assign100000_e152174_d_n6, assign100000_e152174_d_n7, assign100000_e152174_d_n8, assign100000_e152174_d_n9, assign100000_e152174_d_n10, assign100000_e152174_d_n13,) = {
            if (locals.var_exp_k2 > 1e20) {
                (1e20, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                (locals.var_exp_k2, locals.var_exp_k2_dn0, locals.var_exp_k2_dn2, locals.var_exp_k2_dn4, locals.var_exp_k2_dn5, locals.var_exp_k2_dn6, locals.var_exp_k2_dn7, locals.var_exp_k2_dn8, locals.var_exp_k2_dn9, locals.var_exp_k2_dn10, locals.var_exp_k2_dn13,)
            }
        };
        (assign100000_e152174, assign100000_e152174_d_n0, assign100000_e152174_d_n2, assign100000_e152174_d_n4, assign100000_e152174_d_n5, assign100000_e152174_d_n6, assign100000_e152174_d_n7, assign100000_e152174_d_n8, assign100000_e152174_d_n9, assign100000_e152174_d_n10, assign100000_e152174_d_n13,)
    } else {
        (locals.var_exp_k2, locals.var_exp_k2_dn0, locals.var_exp_k2_dn2, locals.var_exp_k2_dn4, locals.var_exp_k2_dn5, locals.var_exp_k2_dn6, locals.var_exp_k2_dn7, locals.var_exp_k2_dn8, locals.var_exp_k2_dn9, locals.var_exp_k2_dn10, locals.var_exp_k2_dn13,)
    }
};
        locals.var_exp_k2 = assign100000_e152176;
        locals.var_exp_k2_dn0 = assign100000_e152176_d_n0;
        locals.var_exp_k2_dn2 = assign100000_e152176_d_n2;
        locals.var_exp_k2_dn4 = assign100000_e152176_d_n4;
        locals.var_exp_k2_dn5 = assign100000_e152176_d_n5;
        locals.var_exp_k2_dn6 = assign100000_e152176_d_n6;
        locals.var_exp_k2_dn7 = assign100000_e152176_d_n7;
        locals.var_exp_k2_dn8 = assign100000_e152176_d_n8;
        locals.var_exp_k2_dn9 = assign100000_e152176_d_n9;
        locals.var_exp_k2_dn10 = assign100000_e152176_d_n10;
        locals.var_exp_k2_dn13 = assign100000_e152176_d_n13;
        locals.var_exp_k2_rv = 0.0;

        let (assign100010_e152182, assign100010_e152182_d_n0, assign100010_e152182_d_n2, assign100010_e152182_d_n4, assign100010_e152182_d_n5, assign100010_e152182_d_n6, assign100010_e152182_d_n7, assign100010_e152182_d_n8, assign100010_e152182_d_n9, assign100010_e152182_d_n10, assign100010_e152182_d_n13,) = {
    if (locals.var_guard2301 != 0.0) {
        let assign100010_e152180: f64 = (locals.var_pn0 * locals.var_exp_k2);
        (assign100010_e152180, ((locals.var_pn0_dn0 * locals.var_exp_k2) + (locals.var_pn0 * locals.var_exp_k2_dn0)), ((locals.var_pn0_dn2 * locals.var_exp_k2) + (locals.var_pn0 * locals.var_exp_k2_dn2)), ((locals.var_pn0_dn4 * locals.var_exp_k2) + (locals.var_pn0 * locals.var_exp_k2_dn4)), ((locals.var_pn0_dn5 * locals.var_exp_k2) + (locals.var_pn0 * locals.var_exp_k2_dn5)), ((locals.var_pn0_dn6 * locals.var_exp_k2) + (locals.var_pn0 * locals.var_exp_k2_dn6)), ((locals.var_pn0_dn7 * locals.var_exp_k2) + (locals.var_pn0 * locals.var_exp_k2_dn7)), ((locals.var_pn0_dn8 * locals.var_exp_k2) + (locals.var_pn0 * locals.var_exp_k2_dn8)), ((locals.var_pn0_dn9 * locals.var_exp_k2) + (locals.var_pn0 * locals.var_exp_k2_dn9)), ((locals.var_pn0_dn10 * locals.var_exp_k2) + (locals.var_pn0 * locals.var_exp_k2_dn10)), ((locals.var_pn0_dn13 * locals.var_exp_k2) + (locals.var_pn0 * locals.var_exp_k2_dn13)),)
    } else {
        (locals.var_p_nk, locals.var_p_nk_dn0, locals.var_p_nk_dn2, locals.var_p_nk_dn4, locals.var_p_nk_dn5, locals.var_p_nk_dn6, locals.var_p_nk_dn7, locals.var_p_nk_dn8, locals.var_p_nk_dn9, locals.var_p_nk_dn10, locals.var_p_nk_dn13,)
    }
};
        locals.var_p_nk = assign100010_e152182;
        locals.var_p_nk_dn0 = assign100010_e152182_d_n0;
        locals.var_p_nk_dn2 = assign100010_e152182_d_n2;
        locals.var_p_nk_dn4 = assign100010_e152182_d_n4;
        locals.var_p_nk_dn5 = assign100010_e152182_d_n5;
        locals.var_p_nk_dn6 = assign100010_e152182_d_n6;
        locals.var_p_nk_dn7 = assign100010_e152182_d_n7;
        locals.var_p_nk_dn8 = assign100010_e152182_d_n8;
        locals.var_p_nk_dn9 = assign100010_e152182_d_n9;
        locals.var_p_nk_dn10 = assign100010_e152182_d_n10;
        locals.var_p_nk_dn13 = assign100010_e152182_d_n13;
        locals.var_p_nk_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_372(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        locals: &mut StampLocals,
    ) {
        let nv16 = ctx.node_voltage(nodes[16]);
        let nv17 = ctx.node_voltage(nodes[17]);
        let (assign100020_e152192, assign100020_e152192_d_n0, assign100020_e152192_d_n2, assign100020_e152192_d_n4, assign100020_e152192_d_n5, assign100020_e152192_d_n6, assign100020_e152192_d_n7, assign100020_e152192_d_n8, assign100020_e152192_d_n9, assign100020_e152192_d_n10, assign100020_e152192_d_n13,) = {
    if (locals.var_guard2301 != 0.0) {
        let assign100020_e152186: f64 = (1.6021918e-19 * p.p13);
        let assign100020_e152189: f64 = (locals.var_p_nk - locals.var_pn0);
        let assign100020_e152190: f64 = (assign100020_e152186 * assign100020_e152189);
        (assign100020_e152190, (assign100020_e152186 * (locals.var_p_nk_dn0 - locals.var_pn0_dn0)), (assign100020_e152186 * (locals.var_p_nk_dn2 - locals.var_pn0_dn2)), (assign100020_e152186 * (locals.var_p_nk_dn4 - locals.var_pn0_dn4)), (assign100020_e152186 * (locals.var_p_nk_dn5 - locals.var_pn0_dn5)), (assign100020_e152186 * (locals.var_p_nk_dn6 - locals.var_pn0_dn6)), (assign100020_e152186 * (locals.var_p_nk_dn7 - locals.var_pn0_dn7)), (assign100020_e152186 * (locals.var_p_nk_dn8 - locals.var_pn0_dn8)), (assign100020_e152186 * (locals.var_p_nk_dn9 - locals.var_pn0_dn9)), (assign100020_e152186 * (locals.var_p_nk_dn10 - locals.var_pn0_dn10)), (assign100020_e152186 * (locals.var_p_nk_dn13 - locals.var_pn0_dn13)),)
    } else {
        (locals.var_q_pexk, locals.var_q_pexk_dn0, locals.var_q_pexk_dn2, locals.var_q_pexk_dn4, locals.var_q_pexk_dn5, locals.var_q_pexk_dn6, locals.var_q_pexk_dn7, locals.var_q_pexk_dn8, locals.var_q_pexk_dn9, locals.var_q_pexk_dn10, locals.var_q_pexk_dn13,)
    }
};
        locals.var_q_pexk = assign100020_e152192;
        locals.var_q_pexk_dn0 = assign100020_e152192_d_n0;
        locals.var_q_pexk_dn2 = assign100020_e152192_d_n2;
        locals.var_q_pexk_dn4 = assign100020_e152192_d_n4;
        locals.var_q_pexk_dn5 = assign100020_e152192_d_n5;
        locals.var_q_pexk_dn6 = assign100020_e152192_d_n6;
        locals.var_q_pexk_dn7 = assign100020_e152192_d_n7;
        locals.var_q_pexk_dn8 = assign100020_e152192_d_n8;
        locals.var_q_pexk_dn9 = assign100020_e152192_d_n9;
        locals.var_q_pexk_dn10 = assign100020_e152192_d_n10;
        locals.var_q_pexk_dn13 = assign100020_e152192_d_n13;
        locals.var_q_pexk_rv = 0.0;

        let assign100030_e152195: f64 = if p.p543 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2306 = assign100030_e152195;
        locals.var_guard2306_rv = 0.0;

        let (assign100040_e152203, assign100040_e152203_d_n0, assign100040_e152203_d_n2, assign100040_e152203_d_n4, assign100040_e152203_d_n5, assign100040_e152203_d_n6, assign100040_e152203_d_n7, assign100040_e152203_d_n8, assign100040_e152203_d_n9, assign100040_e152203_d_n10, assign100040_e152203_d_n13,) = {
    if ((locals.var_guard2301 != 0.0) && (locals.var_guard2306 != 0.0)) {
        let assign100040_e152201: f64 = (locals.var_q_pexk * p.p543);
        (assign100040_e152201, (locals.var_q_pexk_dn0 * p.p543), (locals.var_q_pexk_dn2 * p.p543), (locals.var_q_pexk_dn4 * p.p543), (locals.var_q_pexk_dn5 * p.p543), (locals.var_q_pexk_dn6 * p.p543), (locals.var_q_pexk_dn7 * p.p543), (locals.var_q_pexk_dn8 * p.p543), (locals.var_q_pexk_dn9 * p.p543), (locals.var_q_pexk_dn10 * p.p543), (locals.var_q_pexk_dn13 * p.p543),)
    } else {
        (locals.var_q_qs_k, locals.var_q_qs_k_dn0, locals.var_q_qs_k_dn2, locals.var_q_qs_k_dn4, locals.var_q_qs_k_dn5, locals.var_q_qs_k_dn6, locals.var_q_qs_k_dn7, locals.var_q_qs_k_dn8, locals.var_q_qs_k_dn9, locals.var_q_qs_k_dn10, locals.var_q_qs_k_dn13,)
    }
};
        locals.var_q_qs_k = assign100040_e152203;
        locals.var_q_qs_k_dn0 = assign100040_e152203_d_n0;
        locals.var_q_qs_k_dn2 = assign100040_e152203_d_n2;
        locals.var_q_qs_k_dn4 = assign100040_e152203_d_n4;
        locals.var_q_qs_k_dn5 = assign100040_e152203_d_n5;
        locals.var_q_qs_k_dn6 = assign100040_e152203_d_n6;
        locals.var_q_qs_k_dn7 = assign100040_e152203_d_n7;
        locals.var_q_qs_k_dn8 = assign100040_e152203_d_n8;
        locals.var_q_qs_k_dn9 = assign100040_e152203_d_n9;
        locals.var_q_qs_k_dn10 = assign100040_e152203_d_n10;
        locals.var_q_qs_k_dn13 = assign100040_e152203_d_n13;
        locals.var_q_qs_k_rv = 0.0;

        let (assign100050_e152211, assign100050_e152211_d_n16,) = {
    if ((locals.var_guard2301 != 0.0) && (locals.var_guard2306 != 0.0)) {
        let assign100050_e152209: f64 = (p.p543 * (nv16 - 0.0));
        (assign100050_e152209, p.p543,)
    } else {
        (locals.var_q_nqs_k, locals.var_q_nqs_k_dn16,)
    }
};
        locals.var_q_nqs_k = assign100050_e152211;
        locals.var_q_nqs_k_dn16 = assign100050_e152211_d_n16;
        locals.var_q_nqs_k_rv = 0.0;

        let (assign100060_e152221, assign100060_e152221_d_n0, assign100060_e152221_d_n2, assign100060_e152221_d_n4, assign100060_e152221_d_n5, assign100060_e152221_d_n6, assign100060_e152221_d_n7, assign100060_e152221_d_n8, assign100060_e152221_d_n9, assign100060_e152221_d_n10, assign100060_e152221_d_n13, assign100060_e152221_d_n16,) = {
    if ((locals.var_guard2301 != 0.0) && (locals.var_guard2306 != 0.0)) {
        let assign100060_e152217: f64 = (locals.var_q_nqs_k - locals.var_q_qs_k);
        let assign100060_e152219: f64 = (assign100060_e152217 / p.p543);
        (assign100060_e152219, ((-locals.var_q_qs_k_dn0) / p.p543), ((-locals.var_q_qs_k_dn2) / p.p543), ((-locals.var_q_qs_k_dn4) / p.p543), ((-locals.var_q_qs_k_dn5) / p.p543), ((-locals.var_q_qs_k_dn6) / p.p543), ((-locals.var_q_qs_k_dn7) / p.p543), ((-locals.var_q_qs_k_dn8) / p.p543), ((-locals.var_q_qs_k_dn9) / p.p543), ((-locals.var_q_qs_k_dn10) / p.p543), ((-locals.var_q_qs_k_dn13) / p.p543), (locals.var_q_nqs_k_dn16 / p.p543),)
    } else {
        (locals.var_inqs0_k, locals.var_inqs0_k_dn0, locals.var_inqs0_k_dn2, locals.var_inqs0_k_dn4, locals.var_inqs0_k_dn5, locals.var_inqs0_k_dn6, locals.var_inqs0_k_dn7, locals.var_inqs0_k_dn8, locals.var_inqs0_k_dn9, locals.var_inqs0_k_dn10, locals.var_inqs0_k_dn13, locals.var_inqs0_k_dn16,)
    }
};
        locals.var_inqs0_k = assign100060_e152221;
        locals.var_inqs0_k_dn0 = assign100060_e152221_d_n0;
        locals.var_inqs0_k_dn2 = assign100060_e152221_d_n2;
        locals.var_inqs0_k_dn4 = assign100060_e152221_d_n4;
        locals.var_inqs0_k_dn5 = assign100060_e152221_d_n5;
        locals.var_inqs0_k_dn6 = assign100060_e152221_d_n6;
        locals.var_inqs0_k_dn7 = assign100060_e152221_d_n7;
        locals.var_inqs0_k_dn8 = assign100060_e152221_d_n8;
        locals.var_inqs0_k_dn9 = assign100060_e152221_d_n9;
        locals.var_inqs0_k_dn10 = assign100060_e152221_d_n10;
        locals.var_inqs0_k_dn13 = assign100060_e152221_d_n13;
        locals.var_inqs0_k_dn16 = assign100060_e152221_d_n16;
        locals.var_inqs0_k_rv = 0.0;

        let (assign100070_e152229, assign100070_e152229_d_n0, assign100070_e152229_d_n2, assign100070_e152229_d_n4, assign100070_e152229_d_n5, assign100070_e152229_d_n6, assign100070_e152229_d_n7, assign100070_e152229_d_n8, assign100070_e152229_d_n9, assign100070_e152229_d_n10, assign100070_e152229_d_n13, assign100070_e152229_d_n16,) = {
    if ((locals.var_guard2301 != 0.0) && (locals.var_guard2306 != 0.0)) {
        let assign100070_e152227: f64 = (locals.var_q_nqs_k / p.p543);
        (assign100070_e152227, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, (locals.var_q_nqs_k_dn16 / p.p543),)
    } else {
        (locals.var_q_pexk_nqs, locals.var_q_pexk_nqs_dn0, locals.var_q_pexk_nqs_dn2, locals.var_q_pexk_nqs_dn4, locals.var_q_pexk_nqs_dn5, locals.var_q_pexk_nqs_dn6, locals.var_q_pexk_nqs_dn7, locals.var_q_pexk_nqs_dn8, locals.var_q_pexk_nqs_dn9, locals.var_q_pexk_nqs_dn10, locals.var_q_pexk_nqs_dn13, locals.var_q_pexk_nqs_dn16,)
    }
};
        locals.var_q_pexk_nqs = assign100070_e152229;
        locals.var_q_pexk_nqs_dn0 = assign100070_e152229_d_n0;
        locals.var_q_pexk_nqs_dn2 = assign100070_e152229_d_n2;
        locals.var_q_pexk_nqs_dn4 = assign100070_e152229_d_n4;
        locals.var_q_pexk_nqs_dn5 = assign100070_e152229_d_n5;
        locals.var_q_pexk_nqs_dn6 = assign100070_e152229_d_n6;
        locals.var_q_pexk_nqs_dn7 = assign100070_e152229_d_n7;
        locals.var_q_pexk_nqs_dn8 = assign100070_e152229_d_n8;
        locals.var_q_pexk_nqs_dn9 = assign100070_e152229_d_n9;
        locals.var_q_pexk_nqs_dn10 = assign100070_e152229_d_n10;
        locals.var_q_pexk_nqs_dn13 = assign100070_e152229_d_n13;
        locals.var_q_pexk_nqs_dn16 = assign100070_e152229_d_n16;
        locals.var_q_pexk_nqs_rv = 0.0;

        let (assign100080_e152236, assign100080_e152236_d_n0, assign100080_e152236_d_n2, assign100080_e152236_d_n4, assign100080_e152236_d_n5, assign100080_e152236_d_n6, assign100080_e152236_d_n7, assign100080_e152236_d_n8, assign100080_e152236_d_n9, assign100080_e152236_d_n10, assign100080_e152236_d_n13,) = {
    if ((locals.var_guard2301 != 0.0) && (locals.var_guard2306 == 0.0)) {
        (locals.var_q_pexk, locals.var_q_pexk_dn0, locals.var_q_pexk_dn2, locals.var_q_pexk_dn4, locals.var_q_pexk_dn5, locals.var_q_pexk_dn6, locals.var_q_pexk_dn7, locals.var_q_pexk_dn8, locals.var_q_pexk_dn9, locals.var_q_pexk_dn10, locals.var_q_pexk_dn13,)
    } else {
        (locals.var_q_qs_k, locals.var_q_qs_k_dn0, locals.var_q_qs_k_dn2, locals.var_q_qs_k_dn4, locals.var_q_qs_k_dn5, locals.var_q_qs_k_dn6, locals.var_q_qs_k_dn7, locals.var_q_qs_k_dn8, locals.var_q_qs_k_dn9, locals.var_q_qs_k_dn10, locals.var_q_qs_k_dn13,)
    }
};
        locals.var_q_qs_k = assign100080_e152236;
        locals.var_q_qs_k_dn0 = assign100080_e152236_d_n0;
        locals.var_q_qs_k_dn2 = assign100080_e152236_d_n2;
        locals.var_q_qs_k_dn4 = assign100080_e152236_d_n4;
        locals.var_q_qs_k_dn5 = assign100080_e152236_d_n5;
        locals.var_q_qs_k_dn6 = assign100080_e152236_d_n6;
        locals.var_q_qs_k_dn7 = assign100080_e152236_d_n7;
        locals.var_q_qs_k_dn8 = assign100080_e152236_d_n8;
        locals.var_q_qs_k_dn9 = assign100080_e152236_d_n9;
        locals.var_q_qs_k_dn10 = assign100080_e152236_d_n10;
        locals.var_q_qs_k_dn13 = assign100080_e152236_d_n13;
        locals.var_q_qs_k_rv = 0.0;

        let (assign100090_e152243, assign100090_e152243_d_n0, assign100090_e152243_d_n2, assign100090_e152243_d_n4, assign100090_e152243_d_n5, assign100090_e152243_d_n6, assign100090_e152243_d_n7, assign100090_e152243_d_n8, assign100090_e152243_d_n9, assign100090_e152243_d_n10, assign100090_e152243_d_n13, assign100090_e152243_d_n16,) = {
    if ((locals.var_guard2301 != 0.0) && (locals.var_guard2306 == 0.0)) {
        (locals.var_q_qs_k, locals.var_q_qs_k_dn0, locals.var_q_qs_k_dn2, locals.var_q_qs_k_dn4, locals.var_q_qs_k_dn5, locals.var_q_qs_k_dn6, locals.var_q_qs_k_dn7, locals.var_q_qs_k_dn8, locals.var_q_qs_k_dn9, locals.var_q_qs_k_dn10, locals.var_q_qs_k_dn13, 0.0,)
    } else {
        (locals.var_q_pexk_nqs, locals.var_q_pexk_nqs_dn0, locals.var_q_pexk_nqs_dn2, locals.var_q_pexk_nqs_dn4, locals.var_q_pexk_nqs_dn5, locals.var_q_pexk_nqs_dn6, locals.var_q_pexk_nqs_dn7, locals.var_q_pexk_nqs_dn8, locals.var_q_pexk_nqs_dn9, locals.var_q_pexk_nqs_dn10, locals.var_q_pexk_nqs_dn13, locals.var_q_pexk_nqs_dn16,)
    }
};
        locals.var_q_pexk_nqs = assign100090_e152243;
        locals.var_q_pexk_nqs_dn0 = assign100090_e152243_d_n0;
        locals.var_q_pexk_nqs_dn2 = assign100090_e152243_d_n2;
        locals.var_q_pexk_nqs_dn4 = assign100090_e152243_d_n4;
        locals.var_q_pexk_nqs_dn5 = assign100090_e152243_d_n5;
        locals.var_q_pexk_nqs_dn6 = assign100090_e152243_d_n6;
        locals.var_q_pexk_nqs_dn7 = assign100090_e152243_d_n7;
        locals.var_q_pexk_nqs_dn8 = assign100090_e152243_d_n8;
        locals.var_q_pexk_nqs_dn9 = assign100090_e152243_d_n9;
        locals.var_q_pexk_nqs_dn10 = assign100090_e152243_d_n10;
        locals.var_q_pexk_nqs_dn13 = assign100090_e152243_d_n13;
        locals.var_q_pexk_nqs_dn16 = assign100090_e152243_d_n16;
        locals.var_q_pexk_nqs_rv = 0.0;

        let (assign100100_e152249, assign100100_e152249_d_n0, assign100100_e152249_d_n2, assign100100_e152249_d_n4, assign100100_e152249_d_n5, assign100100_e152249_d_n6, assign100100_e152249_d_n7, assign100100_e152249_d_n8, assign100100_e152249_d_n9, assign100100_e152249_d_n10, assign100100_e152249_d_n13,) = {
    if (locals.var_guard2301 != 0.0) {
        let assign100100_e152247: f64 = (p.p506 - locals.var_vbd_jct);
        (assign100100_e152247, (-locals.var_vbd_jct_dn0), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, (-locals.var_vbd_jct_dn9), 0.0, 0.0,)
    } else {
        (locals.var_vjunc_a, locals.var_vjunc_a_dn0, locals.var_vjunc_a_dn2, locals.var_vjunc_a_dn4, locals.var_vjunc_a_dn5, locals.var_vjunc_a_dn6, locals.var_vjunc_a_dn7, locals.var_vjunc_a_dn8, locals.var_vjunc_a_dn9, locals.var_vjunc_a_dn10, locals.var_vjunc_a_dn13,)
    }
};
        locals.var_vjunc_a = assign100100_e152249;
        locals.var_vjunc_a_dn0 = assign100100_e152249_d_n0;
        locals.var_vjunc_a_dn2 = assign100100_e152249_d_n2;
        locals.var_vjunc_a_dn4 = assign100100_e152249_d_n4;
        locals.var_vjunc_a_dn5 = assign100100_e152249_d_n5;
        locals.var_vjunc_a_dn6 = assign100100_e152249_d_n6;
        locals.var_vjunc_a_dn7 = assign100100_e152249_d_n7;
        locals.var_vjunc_a_dn8 = assign100100_e152249_d_n8;
        locals.var_vjunc_a_dn9 = assign100100_e152249_d_n9;
        locals.var_vjunc_a_dn10 = assign100100_e152249_d_n10;
        locals.var_vjunc_a_dn13 = assign100100_e152249_d_n13;
        locals.var_vjunc_a_rv = 0.0;

        let (assign100110_e152262, assign100110_e152262_d_n0, assign100110_e152262_d_n2, assign100110_e152262_d_n4, assign100110_e152262_d_n5, assign100110_e152262_d_n6, assign100110_e152262_d_n7, assign100110_e152262_d_n8, assign100110_e152262_d_n9, assign100110_e152262_d_n10, assign100110_e152262_d_n13,) = {
    if (locals.var_guard2301 != 0.0) {
        let assign100110_e152253: f64 = (locals.var_vjunc_a * locals.var_vjunc_a);
        let assign100110_e152256: f64 = (4.0 * locals.var_juncdlt);
        let assign100110_e152258: f64 = (assign100110_e152256 * locals.var_juncdlt);
        let assign100110_e152259: f64 = (assign100110_e152253 + assign100110_e152258);
        let assign100110_e152260: f64 = (assign100110_e152259).sqrt();
        (assign100110_e152260, (((locals.var_vjunc_a_dn0 * locals.var_vjunc_a) + (locals.var_vjunc_a * locals.var_vjunc_a_dn0)) / (2.0 * assign100110_e152260)), (((locals.var_vjunc_a_dn2 * locals.var_vjunc_a) + (locals.var_vjunc_a * locals.var_vjunc_a_dn2)) / (2.0 * assign100110_e152260)), (((locals.var_vjunc_a_dn4 * locals.var_vjunc_a) + (locals.var_vjunc_a * locals.var_vjunc_a_dn4)) / (2.0 * assign100110_e152260)), (((locals.var_vjunc_a_dn5 * locals.var_vjunc_a) + (locals.var_vjunc_a * locals.var_vjunc_a_dn5)) / (2.0 * assign100110_e152260)), (((locals.var_vjunc_a_dn6 * locals.var_vjunc_a) + (locals.var_vjunc_a * locals.var_vjunc_a_dn6)) / (2.0 * assign100110_e152260)), (((locals.var_vjunc_a_dn7 * locals.var_vjunc_a) + (locals.var_vjunc_a * locals.var_vjunc_a_dn7)) / (2.0 * assign100110_e152260)), (((locals.var_vjunc_a_dn8 * locals.var_vjunc_a) + (locals.var_vjunc_a * locals.var_vjunc_a_dn8)) / (2.0 * assign100110_e152260)), (((locals.var_vjunc_a_dn9 * locals.var_vjunc_a) + (locals.var_vjunc_a * locals.var_vjunc_a_dn9)) / (2.0 * assign100110_e152260)), (((locals.var_vjunc_a_dn10 * locals.var_vjunc_a) + (locals.var_vjunc_a * locals.var_vjunc_a_dn10)) / (2.0 * assign100110_e152260)), (((locals.var_vjunc_a_dn13 * locals.var_vjunc_a) + (locals.var_vjunc_a * locals.var_vjunc_a_dn13)) / (2.0 * assign100110_e152260)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign100110_e152262;
        locals.var_tmf2_dn0 = assign100110_e152262_d_n0;
        locals.var_tmf2_dn2 = assign100110_e152262_d_n2;
        locals.var_tmf2_dn4 = assign100110_e152262_d_n4;
        locals.var_tmf2_dn5 = assign100110_e152262_d_n5;
        locals.var_tmf2_dn6 = assign100110_e152262_d_n6;
        locals.var_tmf2_dn7 = assign100110_e152262_d_n7;
        locals.var_tmf2_dn8 = assign100110_e152262_d_n8;
        locals.var_tmf2_dn9 = assign100110_e152262_d_n9;
        locals.var_tmf2_dn10 = assign100110_e152262_d_n10;
        locals.var_tmf2_dn13 = assign100110_e152262_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign100120_e152272, assign100120_e152272_d_n0, assign100120_e152272_d_n2, assign100120_e152272_d_n4, assign100120_e152272_d_n5, assign100120_e152272_d_n6, assign100120_e152272_d_n7, assign100120_e152272_d_n8, assign100120_e152272_d_n9, assign100120_e152272_d_n10, assign100120_e152272_d_n13,) = {
    if (locals.var_guard2301 != 0.0) {
        let assign100120_e152268: f64 = (locals.var_vjunc_a / locals.var_tmf2);
        let assign100120_e152269: f64 = (1.0 + assign100120_e152268);
        let assign100120_e152270: f64 = (0.5 * assign100120_e152269);
        (assign100120_e152270, (0.5 * (((locals.var_vjunc_a_dn0 * locals.var_tmf2) - (locals.var_vjunc_a * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vjunc_a_dn2 * locals.var_tmf2) - (locals.var_vjunc_a * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vjunc_a_dn4 * locals.var_tmf2) - (locals.var_vjunc_a * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vjunc_a_dn5 * locals.var_tmf2) - (locals.var_vjunc_a * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vjunc_a_dn6 * locals.var_tmf2) - (locals.var_vjunc_a * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vjunc_a_dn7 * locals.var_tmf2) - (locals.var_vjunc_a * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vjunc_a_dn8 * locals.var_tmf2) - (locals.var_vjunc_a * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vjunc_a_dn9 * locals.var_tmf2) - (locals.var_vjunc_a * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vjunc_a_dn10 * locals.var_tmf2) - (locals.var_vjunc_a * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_vjunc_a_dn13 * locals.var_tmf2) - (locals.var_vjunc_a * locals.var_tmf2_dn13)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign100120_e152272;
        locals.var_t0_dn0 = assign100120_e152272_d_n0;
        locals.var_t0_dn2 = assign100120_e152272_d_n2;
        locals.var_t0_dn4 = assign100120_e152272_d_n4;
        locals.var_t0_dn5 = assign100120_e152272_d_n5;
        locals.var_t0_dn6 = assign100120_e152272_d_n6;
        locals.var_t0_dn7 = assign100120_e152272_d_n7;
        locals.var_t0_dn8 = assign100120_e152272_d_n8;
        locals.var_t0_dn9 = assign100120_e152272_d_n9;
        locals.var_t0_dn10 = assign100120_e152272_d_n10;
        locals.var_t0_dn13 = assign100120_e152272_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign100130_e152280, assign100130_e152280_d_n0, assign100130_e152280_d_n2, assign100130_e152280_d_n4, assign100130_e152280_d_n5, assign100130_e152280_d_n6, assign100130_e152280_d_n7, assign100130_e152280_d_n8, assign100130_e152280_d_n9, assign100130_e152280_d_n10, assign100130_e152280_d_n13,) = {
    if (locals.var_guard2301 != 0.0) {
        let assign100130_e152277: f64 = (locals.var_vjunc_a + locals.var_tmf2);
        let assign100130_e152278: f64 = (0.5 * assign100130_e152277);
        (assign100130_e152278, (0.5 * (locals.var_vjunc_a_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_vjunc_a_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_vjunc_a_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_vjunc_a_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_vjunc_a_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_vjunc_a_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_vjunc_a_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_vjunc_a_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_vjunc_a_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_vjunc_a_dn13 + locals.var_tmf2_dn13)),)
    } else {
        (locals.var_vjunc_a, locals.var_vjunc_a_dn0, locals.var_vjunc_a_dn2, locals.var_vjunc_a_dn4, locals.var_vjunc_a_dn5, locals.var_vjunc_a_dn6, locals.var_vjunc_a_dn7, locals.var_vjunc_a_dn8, locals.var_vjunc_a_dn9, locals.var_vjunc_a_dn10, locals.var_vjunc_a_dn13,)
    }
};
        locals.var_vjunc_a = assign100130_e152280;
        locals.var_vjunc_a_dn0 = assign100130_e152280_d_n0;
        locals.var_vjunc_a_dn2 = assign100130_e152280_d_n2;
        locals.var_vjunc_a_dn4 = assign100130_e152280_d_n4;
        locals.var_vjunc_a_dn5 = assign100130_e152280_d_n5;
        locals.var_vjunc_a_dn6 = assign100130_e152280_d_n6;
        locals.var_vjunc_a_dn7 = assign100130_e152280_d_n7;
        locals.var_vjunc_a_dn8 = assign100130_e152280_d_n8;
        locals.var_vjunc_a_dn9 = assign100130_e152280_d_n9;
        locals.var_vjunc_a_dn10 = assign100130_e152280_d_n10;
        locals.var_vjunc_a_dn13 = assign100130_e152280_d_n13;
        locals.var_vjunc_a_rv = 0.0;

        let assign100140_e152283: f64 = if locals.var_vjunc_a < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2307 = assign100140_e152283;
        locals.var_guard2307_rv = 0.0;

        let (assign100150_e152289, assign100150_e152289_d_n0, assign100150_e152289_d_n2, assign100150_e152289_d_n4, assign100150_e152289_d_n5, assign100150_e152289_d_n6, assign100150_e152289_d_n7, assign100150_e152289_d_n8, assign100150_e152289_d_n9, assign100150_e152289_d_n10, assign100150_e152289_d_n13,) = {
    if ((locals.var_guard2301 != 0.0) && (locals.var_guard2307 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vjunc_a, locals.var_vjunc_a_dn0, locals.var_vjunc_a_dn2, locals.var_vjunc_a_dn4, locals.var_vjunc_a_dn5, locals.var_vjunc_a_dn6, locals.var_vjunc_a_dn7, locals.var_vjunc_a_dn8, locals.var_vjunc_a_dn9, locals.var_vjunc_a_dn10, locals.var_vjunc_a_dn13,)
    }
};
        locals.var_vjunc_a = assign100150_e152289;
        locals.var_vjunc_a_dn0 = assign100150_e152289_d_n0;
        locals.var_vjunc_a_dn2 = assign100150_e152289_d_n2;
        locals.var_vjunc_a_dn4 = assign100150_e152289_d_n4;
        locals.var_vjunc_a_dn5 = assign100150_e152289_d_n5;
        locals.var_vjunc_a_dn6 = assign100150_e152289_d_n6;
        locals.var_vjunc_a_dn7 = assign100150_e152289_d_n7;
        locals.var_vjunc_a_dn8 = assign100150_e152289_d_n8;
        locals.var_vjunc_a_dn9 = assign100150_e152289_d_n9;
        locals.var_vjunc_a_dn10 = assign100150_e152289_d_n10;
        locals.var_vjunc_a_dn13 = assign100150_e152289_d_n13;
        locals.var_vjunc_a_rv = 0.0;

        let (assign100160_e152295, assign100160_e152295_d_n0, assign100160_e152295_d_n2, assign100160_e152295_d_n4, assign100160_e152295_d_n5, assign100160_e152295_d_n6, assign100160_e152295_d_n7, assign100160_e152295_d_n8, assign100160_e152295_d_n9, assign100160_e152295_d_n10, assign100160_e152295_d_n13,) = {
    if ((locals.var_guard2301 != 0.0) && (locals.var_guard2307 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign100160_e152295;
        locals.var_t0_dn0 = assign100160_e152295_d_n0;
        locals.var_t0_dn2 = assign100160_e152295_d_n2;
        locals.var_t0_dn4 = assign100160_e152295_d_n4;
        locals.var_t0_dn5 = assign100160_e152295_d_n5;
        locals.var_t0_dn6 = assign100160_e152295_d_n6;
        locals.var_t0_dn7 = assign100160_e152295_d_n7;
        locals.var_t0_dn8 = assign100160_e152295_d_n8;
        locals.var_t0_dn9 = assign100160_e152295_d_n9;
        locals.var_t0_dn10 = assign100160_e152295_d_n10;
        locals.var_t0_dn13 = assign100160_e152295_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign100170_e152308, assign100170_e152308_d_n0, assign100170_e152308_d_n2, assign100170_e152308_d_n4, assign100170_e152308_d_n5, assign100170_e152308_d_n6, assign100170_e152308_d_n7, assign100170_e152308_d_n8, assign100170_e152308_d_n9, assign100170_e152308_d_n10, assign100170_e152308_d_n13,) = {
    if (locals.var_guard2301 != 0.0) {
        let assign100170_e152299: f64 = (2.0 * 1.034943e-10);
        let assign100170_e152301: f64 = (assign100170_e152299 * locals.var_vjunc_a);
        let assign100170_e152304: f64 = (1.6021918e-19 * locals.var_ndi_i);
        let assign100170_e152305: f64 = (assign100170_e152301 / assign100170_e152304);
        let assign100170_e152306: f64 = (assign100170_e152305).sqrt();
        (assign100170_e152306, (((assign100170_e152299 * locals.var_vjunc_a_dn0) / assign100170_e152304) / (2.0 * assign100170_e152306)), (((assign100170_e152299 * locals.var_vjunc_a_dn2) / assign100170_e152304) / (2.0 * assign100170_e152306)), (((assign100170_e152299 * locals.var_vjunc_a_dn4) / assign100170_e152304) / (2.0 * assign100170_e152306)), (((assign100170_e152299 * locals.var_vjunc_a_dn5) / assign100170_e152304) / (2.0 * assign100170_e152306)), (((assign100170_e152299 * locals.var_vjunc_a_dn6) / assign100170_e152304) / (2.0 * assign100170_e152306)), (((assign100170_e152299 * locals.var_vjunc_a_dn7) / assign100170_e152304) / (2.0 * assign100170_e152306)), (((assign100170_e152299 * locals.var_vjunc_a_dn8) / assign100170_e152304) / (2.0 * assign100170_e152306)), (((assign100170_e152299 * locals.var_vjunc_a_dn9) / assign100170_e152304) / (2.0 * assign100170_e152306)), (((assign100170_e152299 * locals.var_vjunc_a_dn10) / assign100170_e152304) / (2.0 * assign100170_e152306)), (((assign100170_e152299 * locals.var_vjunc_a_dn13) / assign100170_e152304) / (2.0 * assign100170_e152306)),)
    } else {
        (locals.var_w_depa, locals.var_w_depa_dn0, locals.var_w_depa_dn2, locals.var_w_depa_dn4, locals.var_w_depa_dn5, locals.var_w_depa_dn6, locals.var_w_depa_dn7, locals.var_w_depa_dn8, locals.var_w_depa_dn9, locals.var_w_depa_dn10, locals.var_w_depa_dn13,)
    }
};
        locals.var_w_depa = assign100170_e152308;
        locals.var_w_depa_dn0 = assign100170_e152308_d_n0;
        locals.var_w_depa_dn2 = assign100170_e152308_d_n2;
        locals.var_w_depa_dn4 = assign100170_e152308_d_n4;
        locals.var_w_depa_dn5 = assign100170_e152308_d_n5;
        locals.var_w_depa_dn6 = assign100170_e152308_d_n6;
        locals.var_w_depa_dn7 = assign100170_e152308_d_n7;
        locals.var_w_depa_dn8 = assign100170_e152308_d_n8;
        locals.var_w_depa_dn9 = assign100170_e152308_d_n9;
        locals.var_w_depa_dn10 = assign100170_e152308_d_n10;
        locals.var_w_depa_dn13 = assign100170_e152308_d_n13;
        locals.var_w_depa_rv = 0.0;

        let (assign100180_e152316, assign100180_e152316_d_n0, assign100180_e152316_d_n2, assign100180_e152316_d_n4, assign100180_e152316_d_n5, assign100180_e152316_d_n6, assign100180_e152316_d_n7, assign100180_e152316_d_n8, assign100180_e152316_d_n9, assign100180_e152316_d_n10, assign100180_e152316_d_n13,) = {
    if (locals.var_guard2301 != 0.0) {
        let assign100180_e152312: f64 = (p.p545 - locals.var_w_depa);
        let assign100180_e152314: f64 = (assign100180_e152312 - 1e-7);
        (assign100180_e152314, (-locals.var_w_depa_dn0), (-locals.var_w_depa_dn2), (-locals.var_w_depa_dn4), (-locals.var_w_depa_dn5), (-locals.var_w_depa_dn6), (-locals.var_w_depa_dn7), (-locals.var_w_depa_dn8), (-locals.var_w_depa_dn9), (-locals.var_w_depa_dn10), (-locals.var_w_depa_dn13),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13,)
    }
};
        locals.var_tmf1 = assign100180_e152316;
        locals.var_tmf1_dn0 = assign100180_e152316_d_n0;
        locals.var_tmf1_dn2 = assign100180_e152316_d_n2;
        locals.var_tmf1_dn4 = assign100180_e152316_d_n4;
        locals.var_tmf1_dn5 = assign100180_e152316_d_n5;
        locals.var_tmf1_dn6 = assign100180_e152316_d_n6;
        locals.var_tmf1_dn7 = assign100180_e152316_d_n7;
        locals.var_tmf1_dn8 = assign100180_e152316_d_n8;
        locals.var_tmf1_dn9 = assign100180_e152316_d_n9;
        locals.var_tmf1_dn10 = assign100180_e152316_d_n10;
        locals.var_tmf1_dn13 = assign100180_e152316_d_n13;
        locals.var_tmf1_rv = 0.0;

        let (assign100190_e152324, assign100190_e152324_d_n0, assign100190_e152324_d_n2, assign100190_e152324_d_n4, assign100190_e152324_d_n5, assign100190_e152324_d_n6, assign100190_e152324_d_n7, assign100190_e152324_d_n8, assign100190_e152324_d_n9, assign100190_e152324_d_n10, assign100190_e152324_d_n13,) = {
    if (locals.var_guard2301 != 0.0) {
        let assign100190_e152320: f64 = (4.0 * p.p545);
        let assign100190_e152322: f64 = (assign100190_e152320 * 1e-7);
        (assign100190_e152322, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign100190_e152324;
        locals.var_tmf2_dn0 = assign100190_e152324_d_n0;
        locals.var_tmf2_dn2 = assign100190_e152324_d_n2;
        locals.var_tmf2_dn4 = assign100190_e152324_d_n4;
        locals.var_tmf2_dn5 = assign100190_e152324_d_n5;
        locals.var_tmf2_dn6 = assign100190_e152324_d_n6;
        locals.var_tmf2_dn7 = assign100190_e152324_d_n7;
        locals.var_tmf2_dn8 = assign100190_e152324_d_n8;
        locals.var_tmf2_dn9 = assign100190_e152324_d_n9;
        locals.var_tmf2_dn10 = assign100190_e152324_d_n10;
        locals.var_tmf2_dn13 = assign100190_e152324_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign100200_e152334, assign100200_e152334_d_n0, assign100200_e152334_d_n2, assign100200_e152334_d_n4, assign100200_e152334_d_n5, assign100200_e152334_d_n6, assign100200_e152334_d_n7, assign100200_e152334_d_n8, assign100200_e152334_d_n9, assign100200_e152334_d_n10, assign100200_e152334_d_n13,) = {
    if (locals.var_guard2301 != 0.0) {
        let (assign100200_e152332, assign100200_e152332_d_n0, assign100200_e152332_d_n2, assign100200_e152332_d_n4, assign100200_e152332_d_n5, assign100200_e152332_d_n6, assign100200_e152332_d_n7, assign100200_e152332_d_n8, assign100200_e152332_d_n9, assign100200_e152332_d_n10, assign100200_e152332_d_n13,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
            } else {
                let assign100200_e152331: f64 = (-locals.var_tmf2);
                (assign100200_e152331, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn13),)
            }
        };
        (assign100200_e152332, assign100200_e152332_d_n0, assign100200_e152332_d_n2, assign100200_e152332_d_n4, assign100200_e152332_d_n5, assign100200_e152332_d_n6, assign100200_e152332_d_n7, assign100200_e152332_d_n8, assign100200_e152332_d_n9, assign100200_e152332_d_n10, assign100200_e152332_d_n13,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign100200_e152334;
        locals.var_tmf2_dn0 = assign100200_e152334_d_n0;
        locals.var_tmf2_dn2 = assign100200_e152334_d_n2;
        locals.var_tmf2_dn4 = assign100200_e152334_d_n4;
        locals.var_tmf2_dn5 = assign100200_e152334_d_n5;
        locals.var_tmf2_dn6 = assign100200_e152334_d_n6;
        locals.var_tmf2_dn7 = assign100200_e152334_d_n7;
        locals.var_tmf2_dn8 = assign100200_e152334_d_n8;
        locals.var_tmf2_dn9 = assign100200_e152334_d_n9;
        locals.var_tmf2_dn10 = assign100200_e152334_d_n10;
        locals.var_tmf2_dn13 = assign100200_e152334_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign100210_e152343, assign100210_e152343_d_n0, assign100210_e152343_d_n2, assign100210_e152343_d_n4, assign100210_e152343_d_n5, assign100210_e152343_d_n6, assign100210_e152343_d_n7, assign100210_e152343_d_n8, assign100210_e152343_d_n9, assign100210_e152343_d_n10, assign100210_e152343_d_n13,) = {
    if (locals.var_guard2301 != 0.0) {
        let assign100210_e152338: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign100210_e152340: f64 = (assign100210_e152338 + locals.var_tmf2);
        let assign100210_e152341: f64 = (assign100210_e152340).sqrt();
        (assign100210_e152341, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign100210_e152341)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign100210_e152341)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign100210_e152341)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign100210_e152341)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign100210_e152341)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign100210_e152341)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign100210_e152341)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign100210_e152341)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign100210_e152341)), ((((locals.var_tmf1_dn13 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn13)) + locals.var_tmf2_dn13) / (2.0 * assign100210_e152341)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign100210_e152343;
        locals.var_tmf2_dn0 = assign100210_e152343_d_n0;
        locals.var_tmf2_dn2 = assign100210_e152343_d_n2;
        locals.var_tmf2_dn4 = assign100210_e152343_d_n4;
        locals.var_tmf2_dn5 = assign100210_e152343_d_n5;
        locals.var_tmf2_dn6 = assign100210_e152343_d_n6;
        locals.var_tmf2_dn7 = assign100210_e152343_d_n7;
        locals.var_tmf2_dn8 = assign100210_e152343_d_n8;
        locals.var_tmf2_dn9 = assign100210_e152343_d_n9;
        locals.var_tmf2_dn10 = assign100210_e152343_d_n10;
        locals.var_tmf2_dn13 = assign100210_e152343_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign100220_e152353, assign100220_e152353_d_n0, assign100220_e152353_d_n2, assign100220_e152353_d_n4, assign100220_e152353_d_n5, assign100220_e152353_d_n6, assign100220_e152353_d_n7, assign100220_e152353_d_n8, assign100220_e152353_d_n9, assign100220_e152353_d_n10, assign100220_e152353_d_n13,) = {
    if (locals.var_guard2301 != 0.0) {
        let assign100220_e152349: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign100220_e152350: f64 = (1.0 + assign100220_e152349);
        let assign100220_e152351: f64 = (0.5 * assign100220_e152350);
        (assign100220_e152351, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn13 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn13)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign100220_e152353;
        locals.var_t0_dn0 = assign100220_e152353_d_n0;
        locals.var_t0_dn2 = assign100220_e152353_d_n2;
        locals.var_t0_dn4 = assign100220_e152353_d_n4;
        locals.var_t0_dn5 = assign100220_e152353_d_n5;
        locals.var_t0_dn6 = assign100220_e152353_d_n6;
        locals.var_t0_dn7 = assign100220_e152353_d_n7;
        locals.var_t0_dn8 = assign100220_e152353_d_n8;
        locals.var_t0_dn9 = assign100220_e152353_d_n9;
        locals.var_t0_dn10 = assign100220_e152353_d_n10;
        locals.var_t0_dn13 = assign100220_e152353_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign100230_e152363, assign100230_e152363_d_n0, assign100230_e152363_d_n2, assign100230_e152363_d_n4, assign100230_e152363_d_n5, assign100230_e152363_d_n6, assign100230_e152363_d_n7, assign100230_e152363_d_n8, assign100230_e152363_d_n9, assign100230_e152363_d_n10, assign100230_e152363_d_n13,) = {
    if (locals.var_guard2301 != 0.0) {
        let assign100230_e152359: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign100230_e152360: f64 = (0.5 * assign100230_e152359);
        let assign100230_e152361: f64 = (p.p545 - assign100230_e152360);
        (assign100230_e152361, (-(0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (-(0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), (-(0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4))), (-(0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5))), (-(0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), (-(0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))), (-(0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8))), (-(0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9))), (-(0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), (-(0.5 * (locals.var_tmf1_dn13 + locals.var_tmf2_dn13))),)
    } else {
        (locals.var_w_depa, locals.var_w_depa_dn0, locals.var_w_depa_dn2, locals.var_w_depa_dn4, locals.var_w_depa_dn5, locals.var_w_depa_dn6, locals.var_w_depa_dn7, locals.var_w_depa_dn8, locals.var_w_depa_dn9, locals.var_w_depa_dn10, locals.var_w_depa_dn13,)
    }
};
        locals.var_w_depa = assign100230_e152363;
        locals.var_w_depa_dn0 = assign100230_e152363_d_n0;
        locals.var_w_depa_dn2 = assign100230_e152363_d_n2;
        locals.var_w_depa_dn4 = assign100230_e152363_d_n4;
        locals.var_w_depa_dn5 = assign100230_e152363_d_n5;
        locals.var_w_depa_dn6 = assign100230_e152363_d_n6;
        locals.var_w_depa_dn7 = assign100230_e152363_d_n7;
        locals.var_w_depa_dn8 = assign100230_e152363_d_n8;
        locals.var_w_depa_dn9 = assign100230_e152363_d_n9;
        locals.var_w_depa_dn10 = assign100230_e152363_d_n10;
        locals.var_w_depa_dn13 = assign100230_e152363_d_n13;
        locals.var_w_depa_rv = 0.0;

        let assign100240_e152366: f64 = if p.p546 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2308 = assign100240_e152366;
        locals.var_guard2308_rv = 0.0;

        let (assign100250_e152374, assign100250_e152374_d_n0, assign100250_e152374_d_n2, assign100250_e152374_d_n4, assign100250_e152374_d_n5, assign100250_e152374_d_n6, assign100250_e152374_d_n7, assign100250_e152374_d_n8, assign100250_e152374_d_n9, assign100250_e152374_d_n10, assign100250_e152374_d_n13,) = {
    if ((locals.var_guard2301 != 0.0) && (locals.var_guard2308 != 0.0)) {
        let assign100250_e152372: f64 = (locals.var_w_depa * p.p546);
        (assign100250_e152372, (locals.var_w_depa_dn0 * p.p546), (locals.var_w_depa_dn2 * p.p546), (locals.var_w_depa_dn4 * p.p546), (locals.var_w_depa_dn5 * p.p546), (locals.var_w_depa_dn6 * p.p546), (locals.var_w_depa_dn7 * p.p546), (locals.var_w_depa_dn8 * p.p546), (locals.var_w_depa_dn9 * p.p546), (locals.var_w_depa_dn10 * p.p546), (locals.var_w_depa_dn13 * p.p546),)
    } else {
        (locals.var_w_qs_a, locals.var_w_qs_a_dn0, locals.var_w_qs_a_dn2, locals.var_w_qs_a_dn4, locals.var_w_qs_a_dn5, locals.var_w_qs_a_dn6, locals.var_w_qs_a_dn7, locals.var_w_qs_a_dn8, locals.var_w_qs_a_dn9, locals.var_w_qs_a_dn10, locals.var_w_qs_a_dn13,)
    }
};
        locals.var_w_qs_a = assign100250_e152374;
        locals.var_w_qs_a_dn0 = assign100250_e152374_d_n0;
        locals.var_w_qs_a_dn2 = assign100250_e152374_d_n2;
        locals.var_w_qs_a_dn4 = assign100250_e152374_d_n4;
        locals.var_w_qs_a_dn5 = assign100250_e152374_d_n5;
        locals.var_w_qs_a_dn6 = assign100250_e152374_d_n6;
        locals.var_w_qs_a_dn7 = assign100250_e152374_d_n7;
        locals.var_w_qs_a_dn8 = assign100250_e152374_d_n8;
        locals.var_w_qs_a_dn9 = assign100250_e152374_d_n9;
        locals.var_w_qs_a_dn10 = assign100250_e152374_d_n10;
        locals.var_w_qs_a_dn13 = assign100250_e152374_d_n13;
        locals.var_w_qs_a_rv = 0.0;

        let (assign100260_e152382, assign100260_e152382_d_n17,) = {
    if ((locals.var_guard2301 != 0.0) && (locals.var_guard2308 != 0.0)) {
        let assign100260_e152380: f64 = (p.p546 * (nv17 - 0.0));
        (assign100260_e152380, p.p546,)
    } else {
        (locals.var_w_nqs_a, locals.var_w_nqs_a_dn17,)
    }
};
        locals.var_w_nqs_a = assign100260_e152382;
        locals.var_w_nqs_a_dn17 = assign100260_e152382_d_n17;
        locals.var_w_nqs_a_rv = 0.0;

        let (assign100270_e152392, assign100270_e152392_d_n0, assign100270_e152392_d_n2, assign100270_e152392_d_n4, assign100270_e152392_d_n5, assign100270_e152392_d_n6, assign100270_e152392_d_n7, assign100270_e152392_d_n8, assign100270_e152392_d_n9, assign100270_e152392_d_n10, assign100270_e152392_d_n13, assign100270_e152392_d_n17,) = {
    if ((locals.var_guard2301 != 0.0) && (locals.var_guard2308 != 0.0)) {
        let assign100270_e152388: f64 = (locals.var_w_nqs_a - locals.var_w_qs_a);
        let assign100270_e152390: f64 = (assign100270_e152388 / p.p546);
        (assign100270_e152390, ((-locals.var_w_qs_a_dn0) / p.p546), ((-locals.var_w_qs_a_dn2) / p.p546), ((-locals.var_w_qs_a_dn4) / p.p546), ((-locals.var_w_qs_a_dn5) / p.p546), ((-locals.var_w_qs_a_dn6) / p.p546), ((-locals.var_w_qs_a_dn7) / p.p546), ((-locals.var_w_qs_a_dn8) / p.p546), ((-locals.var_w_qs_a_dn9) / p.p546), ((-locals.var_w_qs_a_dn10) / p.p546), ((-locals.var_w_qs_a_dn13) / p.p546), (locals.var_w_nqs_a_dn17 / p.p546),)
    } else {
        (locals.var_iwnqs0_a, locals.var_iwnqs0_a_dn0, locals.var_iwnqs0_a_dn2, locals.var_iwnqs0_a_dn4, locals.var_iwnqs0_a_dn5, locals.var_iwnqs0_a_dn6, locals.var_iwnqs0_a_dn7, locals.var_iwnqs0_a_dn8, locals.var_iwnqs0_a_dn9, locals.var_iwnqs0_a_dn10, locals.var_iwnqs0_a_dn13, locals.var_iwnqs0_a_dn17,)
    }
};
        locals.var_iwnqs0_a = assign100270_e152392;
        locals.var_iwnqs0_a_dn0 = assign100270_e152392_d_n0;
        locals.var_iwnqs0_a_dn2 = assign100270_e152392_d_n2;
        locals.var_iwnqs0_a_dn4 = assign100270_e152392_d_n4;
        locals.var_iwnqs0_a_dn5 = assign100270_e152392_d_n5;
        locals.var_iwnqs0_a_dn6 = assign100270_e152392_d_n6;
        locals.var_iwnqs0_a_dn7 = assign100270_e152392_d_n7;
        locals.var_iwnqs0_a_dn8 = assign100270_e152392_d_n8;
        locals.var_iwnqs0_a_dn9 = assign100270_e152392_d_n9;
        locals.var_iwnqs0_a_dn10 = assign100270_e152392_d_n10;
        locals.var_iwnqs0_a_dn13 = assign100270_e152392_d_n13;
        locals.var_iwnqs0_a_dn17 = assign100270_e152392_d_n17;
        locals.var_iwnqs0_a_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_373(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign100280_e152400, assign100280_e152400_d_n0, assign100280_e152400_d_n2, assign100280_e152400_d_n4, assign100280_e152400_d_n5, assign100280_e152400_d_n6, assign100280_e152400_d_n7, assign100280_e152400_d_n8, assign100280_e152400_d_n9, assign100280_e152400_d_n10, assign100280_e152400_d_n13, assign100280_e152400_d_n17,) = {
    if ((locals.var_guard2301 != 0.0) && (locals.var_guard2308 != 0.0)) {
        let assign100280_e152398: f64 = (locals.var_w_nqs_a / p.p546);
        (assign100280_e152398, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, (locals.var_w_nqs_a_dn17 / p.p546),)
    } else {
        (locals.var_w_depa_nqs, locals.var_w_depa_nqs_dn0, locals.var_w_depa_nqs_dn2, locals.var_w_depa_nqs_dn4, locals.var_w_depa_nqs_dn5, locals.var_w_depa_nqs_dn6, locals.var_w_depa_nqs_dn7, locals.var_w_depa_nqs_dn8, locals.var_w_depa_nqs_dn9, locals.var_w_depa_nqs_dn10, locals.var_w_depa_nqs_dn13, locals.var_w_depa_nqs_dn17,)
    }
};
        locals.var_w_depa_nqs = assign100280_e152400;
        locals.var_w_depa_nqs_dn0 = assign100280_e152400_d_n0;
        locals.var_w_depa_nqs_dn2 = assign100280_e152400_d_n2;
        locals.var_w_depa_nqs_dn4 = assign100280_e152400_d_n4;
        locals.var_w_depa_nqs_dn5 = assign100280_e152400_d_n5;
        locals.var_w_depa_nqs_dn6 = assign100280_e152400_d_n6;
        locals.var_w_depa_nqs_dn7 = assign100280_e152400_d_n7;
        locals.var_w_depa_nqs_dn8 = assign100280_e152400_d_n8;
        locals.var_w_depa_nqs_dn9 = assign100280_e152400_d_n9;
        locals.var_w_depa_nqs_dn10 = assign100280_e152400_d_n10;
        locals.var_w_depa_nqs_dn13 = assign100280_e152400_d_n13;
        locals.var_w_depa_nqs_dn17 = assign100280_e152400_d_n17;
        locals.var_w_depa_nqs_rv = 0.0;

        let (assign100290_e152407, assign100290_e152407_d_n0, assign100290_e152407_d_n2, assign100290_e152407_d_n4, assign100290_e152407_d_n5, assign100290_e152407_d_n6, assign100290_e152407_d_n7, assign100290_e152407_d_n8, assign100290_e152407_d_n9, assign100290_e152407_d_n10, assign100290_e152407_d_n13,) = {
    if ((locals.var_guard2301 != 0.0) && (locals.var_guard2308 == 0.0)) {
        (locals.var_w_depa, locals.var_w_depa_dn0, locals.var_w_depa_dn2, locals.var_w_depa_dn4, locals.var_w_depa_dn5, locals.var_w_depa_dn6, locals.var_w_depa_dn7, locals.var_w_depa_dn8, locals.var_w_depa_dn9, locals.var_w_depa_dn10, locals.var_w_depa_dn13,)
    } else {
        (locals.var_w_qs_a, locals.var_w_qs_a_dn0, locals.var_w_qs_a_dn2, locals.var_w_qs_a_dn4, locals.var_w_qs_a_dn5, locals.var_w_qs_a_dn6, locals.var_w_qs_a_dn7, locals.var_w_qs_a_dn8, locals.var_w_qs_a_dn9, locals.var_w_qs_a_dn10, locals.var_w_qs_a_dn13,)
    }
};
        locals.var_w_qs_a = assign100290_e152407;
        locals.var_w_qs_a_dn0 = assign100290_e152407_d_n0;
        locals.var_w_qs_a_dn2 = assign100290_e152407_d_n2;
        locals.var_w_qs_a_dn4 = assign100290_e152407_d_n4;
        locals.var_w_qs_a_dn5 = assign100290_e152407_d_n5;
        locals.var_w_qs_a_dn6 = assign100290_e152407_d_n6;
        locals.var_w_qs_a_dn7 = assign100290_e152407_d_n7;
        locals.var_w_qs_a_dn8 = assign100290_e152407_d_n8;
        locals.var_w_qs_a_dn9 = assign100290_e152407_d_n9;
        locals.var_w_qs_a_dn10 = assign100290_e152407_d_n10;
        locals.var_w_qs_a_dn13 = assign100290_e152407_d_n13;
        locals.var_w_qs_a_rv = 0.0;

        let (assign100300_e152414, assign100300_e152414_d_n0, assign100300_e152414_d_n2, assign100300_e152414_d_n4, assign100300_e152414_d_n5, assign100300_e152414_d_n6, assign100300_e152414_d_n7, assign100300_e152414_d_n8, assign100300_e152414_d_n9, assign100300_e152414_d_n10, assign100300_e152414_d_n13, assign100300_e152414_d_n17,) = {
    if ((locals.var_guard2301 != 0.0) && (locals.var_guard2308 == 0.0)) {
        (locals.var_w_qs_a, locals.var_w_qs_a_dn0, locals.var_w_qs_a_dn2, locals.var_w_qs_a_dn4, locals.var_w_qs_a_dn5, locals.var_w_qs_a_dn6, locals.var_w_qs_a_dn7, locals.var_w_qs_a_dn8, locals.var_w_qs_a_dn9, locals.var_w_qs_a_dn10, locals.var_w_qs_a_dn13, 0.0,)
    } else {
        (locals.var_w_depa_nqs, locals.var_w_depa_nqs_dn0, locals.var_w_depa_nqs_dn2, locals.var_w_depa_nqs_dn4, locals.var_w_depa_nqs_dn5, locals.var_w_depa_nqs_dn6, locals.var_w_depa_nqs_dn7, locals.var_w_depa_nqs_dn8, locals.var_w_depa_nqs_dn9, locals.var_w_depa_nqs_dn10, locals.var_w_depa_nqs_dn13, locals.var_w_depa_nqs_dn17,)
    }
};
        locals.var_w_depa_nqs = assign100300_e152414;
        locals.var_w_depa_nqs_dn0 = assign100300_e152414_d_n0;
        locals.var_w_depa_nqs_dn2 = assign100300_e152414_d_n2;
        locals.var_w_depa_nqs_dn4 = assign100300_e152414_d_n4;
        locals.var_w_depa_nqs_dn5 = assign100300_e152414_d_n5;
        locals.var_w_depa_nqs_dn6 = assign100300_e152414_d_n6;
        locals.var_w_depa_nqs_dn7 = assign100300_e152414_d_n7;
        locals.var_w_depa_nqs_dn8 = assign100300_e152414_d_n8;
        locals.var_w_depa_nqs_dn9 = assign100300_e152414_d_n9;
        locals.var_w_depa_nqs_dn10 = assign100300_e152414_d_n10;
        locals.var_w_depa_nqs_dn13 = assign100300_e152414_d_n13;
        locals.var_w_depa_nqs_dn17 = assign100300_e152414_d_n17;
        locals.var_w_depa_nqs_rv = 0.0;

        let (assign100310_e152425,) = {
    if (locals.var_guard2301 != 0.0) {
        let assign100310_e152418: f64 = (locals.var_ndi_i * p.p13);
        let assign100310_e152420: f64 = (assign100310_e152418 * 1.6021918e-19);
        let assign100310_e152421: f64 = (-assign100310_e152420);
        let assign100310_e152423: f64 = (assign100310_e152421 * p.p545);
        (assign100310_e152423,)
    } else {
        (locals.var_q_n0,)
    }
};
        locals.var_q_n0 = assign100310_e152425;
        locals.var_q_n0_rv = 0.0;

        let (assign100320_e152443, assign100320_e152443_d_n0, assign100320_e152443_d_n2, assign100320_e152443_d_n4, assign100320_e152443_d_n5, assign100320_e152443_d_n6, assign100320_e152443_d_n7, assign100320_e152443_d_n8, assign100320_e152443_d_n9, assign100320_e152443_d_n10, assign100320_e152443_d_n13, assign100320_e152443_d_n15, assign100320_e152443_d_n17,) = {
    if (locals.var_guard2301 != 0.0) {
        let assign100320_e152429: f64 = (locals.var_la * locals.var_q_pexa_nqs);
        let assign100320_e152431: f64 = (-p.p545);
        let assign100320_e152433: f64 = (assign100320_e152431 / locals.var_la);
        let assign100320_e152434: f64 = (assign100320_e152433).exp();
        let assign100320_e152436: f64 = (-locals.var_w_depa_nqs);
        let assign100320_e152438: f64 = (assign100320_e152436 / locals.var_la);
        let assign100320_e152439: f64 = (assign100320_e152438).exp();
        let assign100320_e152440: f64 = (assign100320_e152434 - assign100320_e152439);
        let assign100320_e152441: f64 = (assign100320_e152429 * assign100320_e152440);
        (assign100320_e152441, ((((locals.var_la_dn0 * locals.var_q_pexa_nqs) + (locals.var_la * locals.var_q_pexa_nqs_dn0)) * assign100320_e152440) + (assign100320_e152429 * ((assign100320_e152434 * (-((assign100320_e152431 * locals.var_la_dn0) / (locals.var_la * locals.var_la)))) - (assign100320_e152439 * ((((-locals.var_w_depa_nqs_dn0) * locals.var_la) - (assign100320_e152436 * locals.var_la_dn0)) / (locals.var_la * locals.var_la)))))), ((((locals.var_la_dn2 * locals.var_q_pexa_nqs) + (locals.var_la * locals.var_q_pexa_nqs_dn2)) * assign100320_e152440) + (assign100320_e152429 * ((assign100320_e152434 * (-((assign100320_e152431 * locals.var_la_dn2) / (locals.var_la * locals.var_la)))) - (assign100320_e152439 * ((((-locals.var_w_depa_nqs_dn2) * locals.var_la) - (assign100320_e152436 * locals.var_la_dn2)) / (locals.var_la * locals.var_la)))))), ((((locals.var_la_dn4 * locals.var_q_pexa_nqs) + (locals.var_la * locals.var_q_pexa_nqs_dn4)) * assign100320_e152440) + (assign100320_e152429 * ((assign100320_e152434 * (-((assign100320_e152431 * locals.var_la_dn4) / (locals.var_la * locals.var_la)))) - (assign100320_e152439 * ((((-locals.var_w_depa_nqs_dn4) * locals.var_la) - (assign100320_e152436 * locals.var_la_dn4)) / (locals.var_la * locals.var_la)))))), ((((locals.var_la_dn5 * locals.var_q_pexa_nqs) + (locals.var_la * locals.var_q_pexa_nqs_dn5)) * assign100320_e152440) + (assign100320_e152429 * ((assign100320_e152434 * (-((assign100320_e152431 * locals.var_la_dn5) / (locals.var_la * locals.var_la)))) - (assign100320_e152439 * ((((-locals.var_w_depa_nqs_dn5) * locals.var_la) - (assign100320_e152436 * locals.var_la_dn5)) / (locals.var_la * locals.var_la)))))), ((((locals.var_la_dn6 * locals.var_q_pexa_nqs) + (locals.var_la * locals.var_q_pexa_nqs_dn6)) * assign100320_e152440) + (assign100320_e152429 * ((assign100320_e152434 * (-((assign100320_e152431 * locals.var_la_dn6) / (locals.var_la * locals.var_la)))) - (assign100320_e152439 * ((((-locals.var_w_depa_nqs_dn6) * locals.var_la) - (assign100320_e152436 * locals.var_la_dn6)) / (locals.var_la * locals.var_la)))))), ((((locals.var_la_dn7 * locals.var_q_pexa_nqs) + (locals.var_la * locals.var_q_pexa_nqs_dn7)) * assign100320_e152440) + (assign100320_e152429 * ((assign100320_e152434 * (-((assign100320_e152431 * locals.var_la_dn7) / (locals.var_la * locals.var_la)))) - (assign100320_e152439 * ((((-locals.var_w_depa_nqs_dn7) * locals.var_la) - (assign100320_e152436 * locals.var_la_dn7)) / (locals.var_la * locals.var_la)))))), ((((locals.var_la_dn8 * locals.var_q_pexa_nqs) + (locals.var_la * locals.var_q_pexa_nqs_dn8)) * assign100320_e152440) + (assign100320_e152429 * ((assign100320_e152434 * (-((assign100320_e152431 * locals.var_la_dn8) / (locals.var_la * locals.var_la)))) - (assign100320_e152439 * ((((-locals.var_w_depa_nqs_dn8) * locals.var_la) - (assign100320_e152436 * locals.var_la_dn8)) / (locals.var_la * locals.var_la)))))), ((((locals.var_la_dn9 * locals.var_q_pexa_nqs) + (locals.var_la * locals.var_q_pexa_nqs_dn9)) * assign100320_e152440) + (assign100320_e152429 * ((assign100320_e152434 * (-((assign100320_e152431 * locals.var_la_dn9) / (locals.var_la * locals.var_la)))) - (assign100320_e152439 * ((((-locals.var_w_depa_nqs_dn9) * locals.var_la) - (assign100320_e152436 * locals.var_la_dn9)) / (locals.var_la * locals.var_la)))))), ((((locals.var_la_dn10 * locals.var_q_pexa_nqs) + (locals.var_la * locals.var_q_pexa_nqs_dn10)) * assign100320_e152440) + (assign100320_e152429 * ((assign100320_e152434 * (-((assign100320_e152431 * locals.var_la_dn10) / (locals.var_la * locals.var_la)))) - (assign100320_e152439 * ((((-locals.var_w_depa_nqs_dn10) * locals.var_la) - (assign100320_e152436 * locals.var_la_dn10)) / (locals.var_la * locals.var_la)))))), ((((locals.var_la_dn13 * locals.var_q_pexa_nqs) + (locals.var_la * locals.var_q_pexa_nqs_dn13)) * assign100320_e152440) + (assign100320_e152429 * ((assign100320_e152434 * (-((assign100320_e152431 * locals.var_la_dn13) / (locals.var_la * locals.var_la)))) - (assign100320_e152439 * ((((-locals.var_w_depa_nqs_dn13) * locals.var_la) - (assign100320_e152436 * locals.var_la_dn13)) / (locals.var_la * locals.var_la)))))), ((locals.var_la * locals.var_q_pexa_nqs_dn15) * assign100320_e152440), (assign100320_e152429 * (-(assign100320_e152439 * ((-locals.var_w_depa_nqs_dn17) / locals.var_la)))),)
    } else {
        (locals.var_q_nexa_nqs, locals.var_q_nexa_nqs_dn0, locals.var_q_nexa_nqs_dn2, locals.var_q_nexa_nqs_dn4, locals.var_q_nexa_nqs_dn5, locals.var_q_nexa_nqs_dn6, locals.var_q_nexa_nqs_dn7, locals.var_q_nexa_nqs_dn8, locals.var_q_nexa_nqs_dn9, locals.var_q_nexa_nqs_dn10, locals.var_q_nexa_nqs_dn13, locals.var_q_nexa_nqs_dn15, locals.var_q_nexa_nqs_dn17,)
    }
};
        locals.var_q_nexa_nqs = assign100320_e152443;
        locals.var_q_nexa_nqs_dn0 = assign100320_e152443_d_n0;
        locals.var_q_nexa_nqs_dn2 = assign100320_e152443_d_n2;
        locals.var_q_nexa_nqs_dn4 = assign100320_e152443_d_n4;
        locals.var_q_nexa_nqs_dn5 = assign100320_e152443_d_n5;
        locals.var_q_nexa_nqs_dn6 = assign100320_e152443_d_n6;
        locals.var_q_nexa_nqs_dn7 = assign100320_e152443_d_n7;
        locals.var_q_nexa_nqs_dn8 = assign100320_e152443_d_n8;
        locals.var_q_nexa_nqs_dn9 = assign100320_e152443_d_n9;
        locals.var_q_nexa_nqs_dn10 = assign100320_e152443_d_n10;
        locals.var_q_nexa_nqs_dn13 = assign100320_e152443_d_n13;
        locals.var_q_nexa_nqs_dn15 = assign100320_e152443_d_n15;
        locals.var_q_nexa_nqs_dn17 = assign100320_e152443_d_n17;
        locals.var_q_nexa_nqs_rv = 0.0;

        let (assign100330_e152459, assign100330_e152459_d_n0, assign100330_e152459_d_n2, assign100330_e152459_d_n4, assign100330_e152459_d_n5, assign100330_e152459_d_n6, assign100330_e152459_d_n7, assign100330_e152459_d_n8, assign100330_e152459_d_n9, assign100330_e152459_d_n10, assign100330_e152459_d_n13, assign100330_e152459_d_n16, assign100330_e152459_d_n17,) = {
    if (locals.var_guard2301 != 0.0) {
        let assign100330_e152447: f64 = (locals.var_la * locals.var_q_pexk_nqs);
        let assign100330_e152450: f64 = (p.p545 - locals.var_w_depa_nqs);
        let assign100330_e152451: f64 = (-assign100330_e152450);
        let assign100330_e152453: f64 = (assign100330_e152451 / locals.var_la);
        let assign100330_e152454: f64 = (assign100330_e152453).exp();
        let assign100330_e152456: f64 = (assign100330_e152454 - 1.0);
        let assign100330_e152457: f64 = (assign100330_e152447 * assign100330_e152456);
        (assign100330_e152457, ((((locals.var_la_dn0 * locals.var_q_pexk_nqs) + (locals.var_la * locals.var_q_pexk_nqs_dn0)) * assign100330_e152456) + (assign100330_e152447 * (assign100330_e152454 * ((((-(-locals.var_w_depa_nqs_dn0)) * locals.var_la) - (assign100330_e152451 * locals.var_la_dn0)) / (locals.var_la * locals.var_la))))), ((((locals.var_la_dn2 * locals.var_q_pexk_nqs) + (locals.var_la * locals.var_q_pexk_nqs_dn2)) * assign100330_e152456) + (assign100330_e152447 * (assign100330_e152454 * ((((-(-locals.var_w_depa_nqs_dn2)) * locals.var_la) - (assign100330_e152451 * locals.var_la_dn2)) / (locals.var_la * locals.var_la))))), ((((locals.var_la_dn4 * locals.var_q_pexk_nqs) + (locals.var_la * locals.var_q_pexk_nqs_dn4)) * assign100330_e152456) + (assign100330_e152447 * (assign100330_e152454 * ((((-(-locals.var_w_depa_nqs_dn4)) * locals.var_la) - (assign100330_e152451 * locals.var_la_dn4)) / (locals.var_la * locals.var_la))))), ((((locals.var_la_dn5 * locals.var_q_pexk_nqs) + (locals.var_la * locals.var_q_pexk_nqs_dn5)) * assign100330_e152456) + (assign100330_e152447 * (assign100330_e152454 * ((((-(-locals.var_w_depa_nqs_dn5)) * locals.var_la) - (assign100330_e152451 * locals.var_la_dn5)) / (locals.var_la * locals.var_la))))), ((((locals.var_la_dn6 * locals.var_q_pexk_nqs) + (locals.var_la * locals.var_q_pexk_nqs_dn6)) * assign100330_e152456) + (assign100330_e152447 * (assign100330_e152454 * ((((-(-locals.var_w_depa_nqs_dn6)) * locals.var_la) - (assign100330_e152451 * locals.var_la_dn6)) / (locals.var_la * locals.var_la))))), ((((locals.var_la_dn7 * locals.var_q_pexk_nqs) + (locals.var_la * locals.var_q_pexk_nqs_dn7)) * assign100330_e152456) + (assign100330_e152447 * (assign100330_e152454 * ((((-(-locals.var_w_depa_nqs_dn7)) * locals.var_la) - (assign100330_e152451 * locals.var_la_dn7)) / (locals.var_la * locals.var_la))))), ((((locals.var_la_dn8 * locals.var_q_pexk_nqs) + (locals.var_la * locals.var_q_pexk_nqs_dn8)) * assign100330_e152456) + (assign100330_e152447 * (assign100330_e152454 * ((((-(-locals.var_w_depa_nqs_dn8)) * locals.var_la) - (assign100330_e152451 * locals.var_la_dn8)) / (locals.var_la * locals.var_la))))), ((((locals.var_la_dn9 * locals.var_q_pexk_nqs) + (locals.var_la * locals.var_q_pexk_nqs_dn9)) * assign100330_e152456) + (assign100330_e152447 * (assign100330_e152454 * ((((-(-locals.var_w_depa_nqs_dn9)) * locals.var_la) - (assign100330_e152451 * locals.var_la_dn9)) / (locals.var_la * locals.var_la))))), ((((locals.var_la_dn10 * locals.var_q_pexk_nqs) + (locals.var_la * locals.var_q_pexk_nqs_dn10)) * assign100330_e152456) + (assign100330_e152447 * (assign100330_e152454 * ((((-(-locals.var_w_depa_nqs_dn10)) * locals.var_la) - (assign100330_e152451 * locals.var_la_dn10)) / (locals.var_la * locals.var_la))))), ((((locals.var_la_dn13 * locals.var_q_pexk_nqs) + (locals.var_la * locals.var_q_pexk_nqs_dn13)) * assign100330_e152456) + (assign100330_e152447 * (assign100330_e152454 * ((((-(-locals.var_w_depa_nqs_dn13)) * locals.var_la) - (assign100330_e152451 * locals.var_la_dn13)) / (locals.var_la * locals.var_la))))), ((locals.var_la * locals.var_q_pexk_nqs_dn16) * assign100330_e152456), (assign100330_e152447 * (assign100330_e152454 * ((-(-locals.var_w_depa_nqs_dn17)) / locals.var_la))),)
    } else {
        (locals.var_q_nexk_nqs, locals.var_q_nexk_nqs_dn0, locals.var_q_nexk_nqs_dn2, locals.var_q_nexk_nqs_dn4, locals.var_q_nexk_nqs_dn5, locals.var_q_nexk_nqs_dn6, locals.var_q_nexk_nqs_dn7, locals.var_q_nexk_nqs_dn8, locals.var_q_nexk_nqs_dn9, locals.var_q_nexk_nqs_dn10, locals.var_q_nexk_nqs_dn13, locals.var_q_nexk_nqs_dn16, locals.var_q_nexk_nqs_dn17,)
    }
};
        locals.var_q_nexk_nqs = assign100330_e152459;
        locals.var_q_nexk_nqs_dn0 = assign100330_e152459_d_n0;
        locals.var_q_nexk_nqs_dn2 = assign100330_e152459_d_n2;
        locals.var_q_nexk_nqs_dn4 = assign100330_e152459_d_n4;
        locals.var_q_nexk_nqs_dn5 = assign100330_e152459_d_n5;
        locals.var_q_nexk_nqs_dn6 = assign100330_e152459_d_n6;
        locals.var_q_nexk_nqs_dn7 = assign100330_e152459_d_n7;
        locals.var_q_nexk_nqs_dn8 = assign100330_e152459_d_n8;
        locals.var_q_nexk_nqs_dn9 = assign100330_e152459_d_n9;
        locals.var_q_nexk_nqs_dn10 = assign100330_e152459_d_n10;
        locals.var_q_nexk_nqs_dn13 = assign100330_e152459_d_n13;
        locals.var_q_nexk_nqs_dn16 = assign100330_e152459_d_n16;
        locals.var_q_nexk_nqs_dn17 = assign100330_e152459_d_n17;
        locals.var_q_nexk_nqs_rv = 0.0;

        let (assign100340_e152468, assign100340_e152468_d_n0, assign100340_e152468_d_n2, assign100340_e152468_d_n4, assign100340_e152468_d_n5, assign100340_e152468_d_n6, assign100340_e152468_d_n7, assign100340_e152468_d_n8, assign100340_e152468_d_n9, assign100340_e152468_d_n10, assign100340_e152468_d_n13, assign100340_e152468_d_n15, assign100340_e152468_d_n16, assign100340_e152468_d_n17,) = {
    if (locals.var_guard2301 != 0.0) {
        let assign100340_e152463: f64 = (locals.var_q_n0 + locals.var_q_nexa_nqs);
        let assign100340_e152465: f64 = (assign100340_e152463 + locals.var_q_nexk_nqs);
        let assign100340_e152466: f64 = (-assign100340_e152465);
        (assign100340_e152466, (-(locals.var_q_nexa_nqs_dn0 + locals.var_q_nexk_nqs_dn0)), (-(locals.var_q_nexa_nqs_dn2 + locals.var_q_nexk_nqs_dn2)), (-(locals.var_q_nexa_nqs_dn4 + locals.var_q_nexk_nqs_dn4)), (-(locals.var_q_nexa_nqs_dn5 + locals.var_q_nexk_nqs_dn5)), (-(locals.var_q_nexa_nqs_dn6 + locals.var_q_nexk_nqs_dn6)), (-(locals.var_q_nexa_nqs_dn7 + locals.var_q_nexk_nqs_dn7)), (-(locals.var_q_nexa_nqs_dn8 + locals.var_q_nexk_nqs_dn8)), (-(locals.var_q_nexa_nqs_dn9 + locals.var_q_nexk_nqs_dn9)), (-(locals.var_q_nexa_nqs_dn10 + locals.var_q_nexk_nqs_dn10)), (-(locals.var_q_nexa_nqs_dn13 + locals.var_q_nexk_nqs_dn13)), (-locals.var_q_nexa_nqs_dn15), (-locals.var_q_nexk_nqs_dn16), (-(locals.var_q_nexa_nqs_dn17 + locals.var_q_nexk_nqs_dn17)),)
    } else {
        (locals.var_qrr, locals.var_qrr_dn0, locals.var_qrr_dn2, locals.var_qrr_dn4, locals.var_qrr_dn5, locals.var_qrr_dn6, locals.var_qrr_dn7, locals.var_qrr_dn8, locals.var_qrr_dn9, locals.var_qrr_dn10, locals.var_qrr_dn13, locals.var_qrr_dn15, locals.var_qrr_dn16, locals.var_qrr_dn17,)
    }
};
        locals.var_qrr = assign100340_e152468;
        locals.var_qrr_dn0 = assign100340_e152468_d_n0;
        locals.var_qrr_dn2 = assign100340_e152468_d_n2;
        locals.var_qrr_dn4 = assign100340_e152468_d_n4;
        locals.var_qrr_dn5 = assign100340_e152468_d_n5;
        locals.var_qrr_dn6 = assign100340_e152468_d_n6;
        locals.var_qrr_dn7 = assign100340_e152468_d_n7;
        locals.var_qrr_dn8 = assign100340_e152468_d_n8;
        locals.var_qrr_dn9 = assign100340_e152468_d_n9;
        locals.var_qrr_dn10 = assign100340_e152468_d_n10;
        locals.var_qrr_dn13 = assign100340_e152468_d_n13;
        locals.var_qrr_dn15 = assign100340_e152468_d_n15;
        locals.var_qrr_dn16 = assign100340_e152468_d_n16;
        locals.var_qrr_dn17 = assign100340_e152468_d_n17;
        locals.var_qrr_rv = 0.0;

        let (assign100350_e152476, assign100350_e152476_d_n0, assign100350_e152476_d_n2, assign100350_e152476_d_n4, assign100350_e152476_d_n5, assign100350_e152476_d_n6, assign100350_e152476_d_n7, assign100350_e152476_d_n8, assign100350_e152476_d_n9, assign100350_e152476_d_n10, assign100350_e152476_d_n13, assign100350_e152476_d_n15, assign100350_e152476_d_n16, assign100350_e152476_d_n17,) = {
    if (locals.var_guard2301 != 0.0) {
        let assign100350_e152473: f64 = (locals.var_mfactor * locals.var_qrr);
        let assign100350_e152474: f64 = (locals.var_qbd + assign100350_e152473);
        (assign100350_e152474, (locals.var_qbd_dn0 + (locals.var_mfactor * locals.var_qrr_dn0)), (locals.var_qbd_dn2 + (locals.var_mfactor * locals.var_qrr_dn2)), (locals.var_qbd_dn4 + (locals.var_mfactor * locals.var_qrr_dn4)), (locals.var_qbd_dn5 + (locals.var_mfactor * locals.var_qrr_dn5)), (locals.var_qbd_dn6 + (locals.var_mfactor * locals.var_qrr_dn6)), (locals.var_qbd_dn7 + (locals.var_mfactor * locals.var_qrr_dn7)), (locals.var_qbd_dn8 + (locals.var_mfactor * locals.var_qrr_dn8)), (locals.var_qbd_dn9 + (locals.var_mfactor * locals.var_qrr_dn9)), (locals.var_qbd_dn10 + (locals.var_mfactor * locals.var_qrr_dn10)), (locals.var_qbd_dn13 + (locals.var_mfactor * locals.var_qrr_dn13)), (locals.var_qbd_dn15 + (locals.var_mfactor * locals.var_qrr_dn15)), (locals.var_qbd_dn16 + (locals.var_mfactor * locals.var_qrr_dn16)), (locals.var_qbd_dn17 + (locals.var_mfactor * locals.var_qrr_dn17)),)
    } else {
        (locals.var_qbd, locals.var_qbd_dn0, locals.var_qbd_dn2, locals.var_qbd_dn4, locals.var_qbd_dn5, locals.var_qbd_dn6, locals.var_qbd_dn7, locals.var_qbd_dn8, locals.var_qbd_dn9, locals.var_qbd_dn10, locals.var_qbd_dn13, locals.var_qbd_dn15, locals.var_qbd_dn16, locals.var_qbd_dn17,)
    }
};
        locals.var_qbd = assign100350_e152476;
        locals.var_qbd_dn0 = assign100350_e152476_d_n0;
        locals.var_qbd_dn2 = assign100350_e152476_d_n2;
        locals.var_qbd_dn4 = assign100350_e152476_d_n4;
        locals.var_qbd_dn5 = assign100350_e152476_d_n5;
        locals.var_qbd_dn6 = assign100350_e152476_d_n6;
        locals.var_qbd_dn7 = assign100350_e152476_d_n7;
        locals.var_qbd_dn8 = assign100350_e152476_d_n8;
        locals.var_qbd_dn9 = assign100350_e152476_d_n9;
        locals.var_qbd_dn10 = assign100350_e152476_d_n10;
        locals.var_qbd_dn13 = assign100350_e152476_d_n13;
        locals.var_qbd_dn15 = assign100350_e152476_d_n15;
        locals.var_qbd_dn16 = assign100350_e152476_d_n16;
        locals.var_qbd_dn17 = assign100350_e152476_d_n17;
        locals.var_qbd_rv = 0.0;

        let assign100360_e152483: f64 = if ((p.p539 > 0.0) && (p.p543 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard2309 = assign100360_e152483;
        locals.var_guard2309_rv = 0.0;

        let assign100370_e152490: f64 = if ((p.p539 > 0.0) && (p.p546 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard2310 = assign100370_e152490;
        locals.var_guard2310_rv = 0.0;

        let assign100380_e152493: f64 = if p.p46 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard2311 = assign100380_e152493;
        locals.var_guard2311_rv = 0.0;

        let assign100390_e152500: f64 = if ((locals.var_uc_sub1snp > 0.0) && (locals.var_uc_vmax > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard2312 = assign100390_e152500;
        locals.var_guard2312_rv = 0.0;

        let (assign100400_e152508, assign100400_e152508_d_n0, assign100400_e152508_d_n2, assign100400_e152508_d_n4, assign100400_e152508_d_n5, assign100400_e152508_d_n6, assign100400_e152508_d_n7, assign100400_e152508_d_n8, assign100400_e152508_d_n9, assign100400_e152508_d_n10, assign100400_e152508_d_n13,) = {
    if ((locals.var_guard2311 != 0.0) && (locals.var_guard2312 != 0.0)) {
        let assign100400_e152506: f64 = (locals.var_vg2const_1 * locals.var_vgp);
        (assign100400_e152506, ((locals.var_vg2const_1_dn0 * locals.var_vgp) + (locals.var_vg2const_1 * locals.var_vgp_dn0)), ((locals.var_vg2const_1_dn2 * locals.var_vgp) + (locals.var_vg2const_1 * locals.var_vgp_dn2)), ((locals.var_vg2const_1_dn4 * locals.var_vgp) + (locals.var_vg2const_1 * locals.var_vgp_dn4)), ((locals.var_vg2const_1_dn5 * locals.var_vgp) + (locals.var_vg2const_1 * locals.var_vgp_dn5)), ((locals.var_vg2const_1_dn6 * locals.var_vgp) + (locals.var_vg2const_1 * locals.var_vgp_dn6)), ((locals.var_vg2const_1_dn7 * locals.var_vgp) + (locals.var_vg2const_1 * locals.var_vgp_dn7)), ((locals.var_vg2const_1_dn8 * locals.var_vgp) + (locals.var_vg2const_1 * locals.var_vgp_dn8)), ((locals.var_vg2const_1_dn9 * locals.var_vgp) + (locals.var_vg2const_1 * locals.var_vgp_dn9)), ((locals.var_vg2const_1_dn10 * locals.var_vgp) + (locals.var_vg2const_1 * locals.var_vgp_dn10)), ((locals.var_vg2const_1_dn13 * locals.var_vgp) + (locals.var_vg2const_1 * locals.var_vgp_dn13)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign100400_e152508;
        locals.var_t1_dn0 = assign100400_e152508_d_n0;
        locals.var_t1_dn2 = assign100400_e152508_d_n2;
        locals.var_t1_dn4 = assign100400_e152508_d_n4;
        locals.var_t1_dn5 = assign100400_e152508_d_n5;
        locals.var_t1_dn6 = assign100400_e152508_d_n6;
        locals.var_t1_dn7 = assign100400_e152508_d_n7;
        locals.var_t1_dn8 = assign100400_e152508_d_n8;
        locals.var_t1_dn9 = assign100400_e152508_d_n9;
        locals.var_t1_dn10 = assign100400_e152508_d_n10;
        locals.var_t1_dn13 = assign100400_e152508_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign100410_e152518, assign100410_e152518_d_n0, assign100410_e152518_d_n2, assign100410_e152518_d_n4, assign100410_e152518_d_n5, assign100410_e152518_d_n6, assign100410_e152518_d_n7, assign100410_e152518_d_n8, assign100410_e152518_d_n9, assign100410_e152518_d_n10, assign100410_e152518_d_n13,) = {
    if ((locals.var_guard2311 != 0.0) && (locals.var_guard2312 != 0.0)) {
        let assign100410_e152515: f64 = (locals.var_cox0 * locals.var_cox0);
        let assign100410_e152516: f64 = (locals.var_qnsub_esi / assign100410_e152515);
        (assign100410_e152516, (locals.var_qnsub_esi_dn0 / assign100410_e152515), (locals.var_qnsub_esi_dn2 / assign100410_e152515), (locals.var_qnsub_esi_dn4 / assign100410_e152515), (locals.var_qnsub_esi_dn5 / assign100410_e152515), (locals.var_qnsub_esi_dn6 / assign100410_e152515), (locals.var_qnsub_esi_dn7 / assign100410_e152515), (locals.var_qnsub_esi_dn8 / assign100410_e152515), (locals.var_qnsub_esi_dn9 / assign100410_e152515), (locals.var_qnsub_esi_dn10 / assign100410_e152515), (locals.var_qnsub_esi_dn13 / assign100410_e152515),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn13,)
    }
};
        locals.var_t3 = assign100410_e152518;
        locals.var_t3_dn0 = assign100410_e152518_d_n0;
        locals.var_t3_dn2 = assign100410_e152518_d_n2;
        locals.var_t3_dn4 = assign100410_e152518_d_n4;
        locals.var_t3_dn5 = assign100410_e152518_d_n5;
        locals.var_t3_dn6 = assign100410_e152518_d_n6;
        locals.var_t3_dn7 = assign100410_e152518_d_n7;
        locals.var_t3_dn8 = assign100410_e152518_d_n8;
        locals.var_t3_dn9 = assign100410_e152518_d_n9;
        locals.var_t3_dn10 = assign100410_e152518_d_n10;
        locals.var_t3_dn13 = assign100410_e152518_d_n13;
        locals.var_t3_rv = 0.0;

        let (assign100420_e152530, assign100420_e152530_d_n0, assign100420_e152530_d_n2, assign100420_e152530_d_n4, assign100420_e152530_d_n5, assign100420_e152530_d_n6, assign100420_e152530_d_n7, assign100420_e152530_d_n8, assign100420_e152530_d_n9, assign100420_e152530_d_n10, assign100420_e152530_d_n13,) = {
    if ((locals.var_guard2311 != 0.0) && (locals.var_guard2312 != 0.0)) {
        let assign100420_e152524: f64 = (2.0 / locals.var_qnsub_esi);
        let assign100420_e152527: f64 = (locals.var_cox0 * locals.var_cox0);
        let assign100420_e152528: f64 = (assign100420_e152524 * assign100420_e152527);
        (assign100420_e152528, ((-((2.0 * locals.var_qnsub_esi_dn0) / (locals.var_qnsub_esi * locals.var_qnsub_esi))) * assign100420_e152527), ((-((2.0 * locals.var_qnsub_esi_dn2) / (locals.var_qnsub_esi * locals.var_qnsub_esi))) * assign100420_e152527), ((-((2.0 * locals.var_qnsub_esi_dn4) / (locals.var_qnsub_esi * locals.var_qnsub_esi))) * assign100420_e152527), ((-((2.0 * locals.var_qnsub_esi_dn5) / (locals.var_qnsub_esi * locals.var_qnsub_esi))) * assign100420_e152527), ((-((2.0 * locals.var_qnsub_esi_dn6) / (locals.var_qnsub_esi * locals.var_qnsub_esi))) * assign100420_e152527), ((-((2.0 * locals.var_qnsub_esi_dn7) / (locals.var_qnsub_esi * locals.var_qnsub_esi))) * assign100420_e152527), ((-((2.0 * locals.var_qnsub_esi_dn8) / (locals.var_qnsub_esi * locals.var_qnsub_esi))) * assign100420_e152527), ((-((2.0 * locals.var_qnsub_esi_dn9) / (locals.var_qnsub_esi * locals.var_qnsub_esi))) * assign100420_e152527), ((-((2.0 * locals.var_qnsub_esi_dn10) / (locals.var_qnsub_esi * locals.var_qnsub_esi))) * assign100420_e152527), ((-((2.0 * locals.var_qnsub_esi_dn13) / (locals.var_qnsub_esi * locals.var_qnsub_esi))) * assign100420_e152527),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn13,)
    }
};
        locals.var_t4 = assign100420_e152530;
        locals.var_t4_dn0 = assign100420_e152530_d_n0;
        locals.var_t4_dn2 = assign100420_e152530_d_n2;
        locals.var_t4_dn4 = assign100420_e152530_d_n4;
        locals.var_t4_dn5 = assign100420_e152530_d_n5;
        locals.var_t4_dn6 = assign100420_e152530_d_n6;
        locals.var_t4_dn7 = assign100420_e152530_d_n7;
        locals.var_t4_dn8 = assign100420_e152530_d_n8;
        locals.var_t4_dn9 = assign100420_e152530_d_n9;
        locals.var_t4_dn10 = assign100420_e152530_d_n10;
        locals.var_t4_dn13 = assign100420_e152530_d_n13;
        locals.var_t4_rv = 0.0;

        let (assign100430_e152542, assign100430_e152542_d_n0, assign100430_e152542_d_n2, assign100430_e152542_d_n4, assign100430_e152542_d_n5, assign100430_e152542_d_n6, assign100430_e152542_d_n7, assign100430_e152542_d_n8, assign100430_e152542_d_n9, assign100430_e152542_d_n10, assign100430_e152542_d_n13,) = {
    if ((locals.var_guard2311 != 0.0) && (locals.var_guard2312 != 0.0)) {
        let assign100430_e152536: f64 = (locals.var_t1 - locals.var_beta_inv);
        let assign100430_e152539: f64 = (locals.var_xvbs_1 * locals.var_vbsz__blk438);
        let assign100430_e152540: f64 = (assign100430_e152536 - assign100430_e152539);
        (assign100430_e152540, ((locals.var_t1_dn0 - locals.var_beta_inv_dn0) - (locals.var_xvbs_1 * locals.var_vbsz__blk438_dn0)), ((locals.var_t1_dn2 - locals.var_beta_inv_dn2) - (locals.var_xvbs_1 * locals.var_vbsz__blk438_dn2)), ((locals.var_t1_dn4 - locals.var_beta_inv_dn4) - (locals.var_xvbs_1 * locals.var_vbsz__blk438_dn4)), ((locals.var_t1_dn5 - locals.var_beta_inv_dn5) - (locals.var_xvbs_1 * locals.var_vbsz__blk438_dn5)), ((locals.var_t1_dn6 - locals.var_beta_inv_dn6) - (locals.var_xvbs_1 * locals.var_vbsz__blk438_dn6)), ((locals.var_t1_dn7 - locals.var_beta_inv_dn7) - (locals.var_xvbs_1 * locals.var_vbsz__blk438_dn7)), ((locals.var_t1_dn8 - locals.var_beta_inv_dn8) - (locals.var_xvbs_1 * locals.var_vbsz__blk438_dn8)), ((locals.var_t1_dn9 - locals.var_beta_inv_dn9) - (locals.var_xvbs_1 * locals.var_vbsz__blk438_dn9)), ((locals.var_t1_dn10 - locals.var_beta_inv_dn10) - (locals.var_xvbs_1 * locals.var_vbsz__blk438_dn10)), ((locals.var_t1_dn13 - locals.var_beta_inv_dn13) - (locals.var_xvbs_1 * locals.var_vbsz__blk438_dn13)),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn13,)
    }
};
        locals.var_t5 = assign100430_e152542;
        locals.var_t5_dn0 = assign100430_e152542_d_n0;
        locals.var_t5_dn2 = assign100430_e152542_d_n2;
        locals.var_t5_dn4 = assign100430_e152542_d_n4;
        locals.var_t5_dn5 = assign100430_e152542_d_n5;
        locals.var_t5_dn6 = assign100430_e152542_d_n6;
        locals.var_t5_dn7 = assign100430_e152542_d_n7;
        locals.var_t5_dn8 = assign100430_e152542_d_n8;
        locals.var_t5_dn9 = assign100430_e152542_d_n9;
        locals.var_t5_dn10 = assign100430_e152542_d_n10;
        locals.var_t5_dn13 = assign100430_e152542_d_n13;
        locals.var_t5_rv = 0.0;

        let (assign100440_e152552, assign100440_e152552_d_n0, assign100440_e152552_d_n2, assign100440_e152552_d_n4, assign100440_e152552_d_n5, assign100440_e152552_d_n6, assign100440_e152552_d_n7, assign100440_e152552_d_n8, assign100440_e152552_d_n9, assign100440_e152552_d_n10, assign100440_e152552_d_n13,) = {
    if ((locals.var_guard2311 != 0.0) && (locals.var_guard2312 != 0.0)) {
        let assign100440_e152549: f64 = (locals.var_t4 * locals.var_t5);
        let assign100440_e152550: f64 = (1.0 + assign100440_e152549);
        (assign100440_e152550, ((locals.var_t4_dn0 * locals.var_t5) + (locals.var_t4 * locals.var_t5_dn0)), ((locals.var_t4_dn2 * locals.var_t5) + (locals.var_t4 * locals.var_t5_dn2)), ((locals.var_t4_dn4 * locals.var_t5) + (locals.var_t4 * locals.var_t5_dn4)), ((locals.var_t4_dn5 * locals.var_t5) + (locals.var_t4 * locals.var_t5_dn5)), ((locals.var_t4_dn6 * locals.var_t5) + (locals.var_t4 * locals.var_t5_dn6)), ((locals.var_t4_dn7 * locals.var_t5) + (locals.var_t4 * locals.var_t5_dn7)), ((locals.var_t4_dn8 * locals.var_t5) + (locals.var_t4 * locals.var_t5_dn8)), ((locals.var_t4_dn9 * locals.var_t5) + (locals.var_t4 * locals.var_t5_dn9)), ((locals.var_t4_dn10 * locals.var_t5) + (locals.var_t4 * locals.var_t5_dn10)), ((locals.var_t4_dn13 * locals.var_t5) + (locals.var_t4 * locals.var_t5_dn13)),)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn13,)
    }
};
        locals.var_t6 = assign100440_e152552;
        locals.var_t6_dn0 = assign100440_e152552_d_n0;
        locals.var_t6_dn2 = assign100440_e152552_d_n2;
        locals.var_t6_dn4 = assign100440_e152552_d_n4;
        locals.var_t6_dn5 = assign100440_e152552_d_n5;
        locals.var_t6_dn6 = assign100440_e152552_d_n6;
        locals.var_t6_dn7 = assign100440_e152552_d_n7;
        locals.var_t6_dn8 = assign100440_e152552_d_n8;
        locals.var_t6_dn9 = assign100440_e152552_d_n9;
        locals.var_t6_dn10 = assign100440_e152552_d_n10;
        locals.var_t6_dn13 = assign100440_e152552_d_n13;
        locals.var_t6_rv = 0.0;

        let (assign100450_e152562, assign100450_e152562_d_n0, assign100450_e152562_d_n2, assign100450_e152562_d_n4, assign100450_e152562_d_n5, assign100450_e152562_d_n6, assign100450_e152562_d_n7, assign100450_e152562_d_n8, assign100450_e152562_d_n9, assign100450_e152562_d_n10, assign100450_e152562_d_n13,) = {
    if ((locals.var_guard2311 != 0.0) && (locals.var_guard2312 != 0.0)) {
        let assign100450_e152559: f64 = (1.0 + locals.var_t4);
        let assign100450_e152560: f64 = (2.0 * assign100450_e152559);
        (assign100450_e152560, (2.0 * locals.var_t4_dn0), (2.0 * locals.var_t4_dn2), (2.0 * locals.var_t4_dn4), (2.0 * locals.var_t4_dn5), (2.0 * locals.var_t4_dn6), (2.0 * locals.var_t4_dn7), (2.0 * locals.var_t4_dn8), (2.0 * locals.var_t4_dn9), (2.0 * locals.var_t4_dn10), (2.0 * locals.var_t4_dn13),)
    } else {
        (locals.var_t7, locals.var_t7_dn0, locals.var_t7_dn2, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn13,)
    }
};
        locals.var_t7 = assign100450_e152562;
        locals.var_t7_dn0 = assign100450_e152562_d_n0;
        locals.var_t7_dn2 = assign100450_e152562_d_n2;
        locals.var_t7_dn4 = assign100450_e152562_d_n4;
        locals.var_t7_dn5 = assign100450_e152562_d_n5;
        locals.var_t7_dn6 = assign100450_e152562_d_n6;
        locals.var_t7_dn7 = assign100450_e152562_d_n7;
        locals.var_t7_dn8 = assign100450_e152562_d_n8;
        locals.var_t7_dn9 = assign100450_e152562_d_n9;
        locals.var_t7_dn10 = assign100450_e152562_d_n10;
        locals.var_t7_dn13 = assign100450_e152562_d_n13;
        locals.var_t7_rv = 0.0;

        let assign100460_e152566: f64 = locals.var_t7;
        let assign100460_e152571: f64 = if ((locals.var_t6 < assign100460_e152566) && (locals.var_t7 >= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard2313 = assign100460_e152571;
        locals.var_guard2313_rv = 0.0;

        let (assign100470_e152583, assign100470_e152583_d_n0, assign100470_e152583_d_n2, assign100470_e152583_d_n4, assign100470_e152583_d_n5, assign100470_e152583_d_n6, assign100470_e152583_d_n7, assign100470_e152583_d_n8, assign100470_e152583_d_n9, assign100470_e152583_d_n10, assign100470_e152583_d_n13,) = {
    if (((locals.var_guard2311 != 0.0) && (locals.var_guard2312 != 0.0)) && (locals.var_guard2313 != 0.0)) {
        let assign100470_e152579: f64 = locals.var_t7;
        let assign100470_e152581: f64 = (assign100470_e152579 - locals.var_t6);
        (assign100470_e152581, (locals.var_t7_dn0 - locals.var_t6_dn0), (locals.var_t7_dn2 - locals.var_t6_dn2), (locals.var_t7_dn4 - locals.var_t6_dn4), (locals.var_t7_dn5 - locals.var_t6_dn5), (locals.var_t7_dn6 - locals.var_t6_dn6), (locals.var_t7_dn7 - locals.var_t6_dn7), (locals.var_t7_dn8 - locals.var_t6_dn8), (locals.var_t7_dn9 - locals.var_t6_dn9), (locals.var_t7_dn10 - locals.var_t6_dn10), (locals.var_t7_dn13 - locals.var_t6_dn13),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13,)
    }
};
        locals.var_tmf1 = assign100470_e152583;
        locals.var_tmf1_dn0 = assign100470_e152583_d_n0;
        locals.var_tmf1_dn2 = assign100470_e152583_d_n2;
        locals.var_tmf1_dn4 = assign100470_e152583_d_n4;
        locals.var_tmf1_dn5 = assign100470_e152583_d_n5;
        locals.var_tmf1_dn6 = assign100470_e152583_d_n6;
        locals.var_tmf1_dn7 = assign100470_e152583_d_n7;
        locals.var_tmf1_dn8 = assign100470_e152583_d_n8;
        locals.var_tmf1_dn9 = assign100470_e152583_d_n9;
        locals.var_tmf1_dn10 = assign100470_e152583_d_n10;
        locals.var_tmf1_dn13 = assign100470_e152583_d_n13;
        locals.var_tmf1_rv = 0.0;

        let (assign100480_e152593, assign100480_e152593_d_n0, assign100480_e152593_d_n2, assign100480_e152593_d_n4, assign100480_e152593_d_n5, assign100480_e152593_d_n6, assign100480_e152593_d_n7, assign100480_e152593_d_n8, assign100480_e152593_d_n9, assign100480_e152593_d_n10, assign100480_e152593_d_n13,) = {
    if (((locals.var_guard2311 != 0.0) && (locals.var_guard2312 != 0.0)) && (locals.var_guard2313 != 0.0)) {
        let assign100480_e152591: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign100480_e152591, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)), ((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)), ((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn13 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn13)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn4, locals.var_x2_dn5, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn8, locals.var_x2_dn9, locals.var_x2_dn10, locals.var_x2_dn13,)
    }
};
        locals.var_x2 = assign100480_e152593;
        locals.var_x2_dn0 = assign100480_e152593_d_n0;
        locals.var_x2_dn2 = assign100480_e152593_d_n2;
        locals.var_x2_dn4 = assign100480_e152593_d_n4;
        locals.var_x2_dn5 = assign100480_e152593_d_n5;
        locals.var_x2_dn6 = assign100480_e152593_d_n6;
        locals.var_x2_dn7 = assign100480_e152593_d_n7;
        locals.var_x2_dn8 = assign100480_e152593_d_n8;
        locals.var_x2_dn9 = assign100480_e152593_d_n9;
        locals.var_x2_dn10 = assign100480_e152593_d_n10;
        locals.var_x2_dn13 = assign100480_e152593_d_n13;
        locals.var_x2_rv = 0.0;

        let (assign100490_e152603, assign100490_e152603_d_n0, assign100490_e152603_d_n2, assign100490_e152603_d_n4, assign100490_e152603_d_n5, assign100490_e152603_d_n6, assign100490_e152603_d_n7, assign100490_e152603_d_n8, assign100490_e152603_d_n9, assign100490_e152603_d_n10, assign100490_e152603_d_n13,) = {
    if (((locals.var_guard2311 != 0.0) && (locals.var_guard2312 != 0.0)) && (locals.var_guard2313 != 0.0)) {
        let assign100490_e152601: f64 = (locals.var_t7 * locals.var_t7);
        (assign100490_e152601, ((locals.var_t7_dn0 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn0)), ((locals.var_t7_dn2 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn2)), ((locals.var_t7_dn4 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn4)), ((locals.var_t7_dn5 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn5)), ((locals.var_t7_dn6 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn6)), ((locals.var_t7_dn7 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn7)), ((locals.var_t7_dn8 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn8)), ((locals.var_t7_dn9 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn9)), ((locals.var_t7_dn10 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn10)), ((locals.var_t7_dn13 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn13)),)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn4, locals.var_xmax2_dn5, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn8, locals.var_xmax2_dn9, locals.var_xmax2_dn10, locals.var_xmax2_dn13,)
    }
};
        locals.var_xmax2 = assign100490_e152603;
        locals.var_xmax2_dn0 = assign100490_e152603_d_n0;
        locals.var_xmax2_dn2 = assign100490_e152603_d_n2;
        locals.var_xmax2_dn4 = assign100490_e152603_d_n4;
        locals.var_xmax2_dn5 = assign100490_e152603_d_n5;
        locals.var_xmax2_dn6 = assign100490_e152603_d_n6;
        locals.var_xmax2_dn7 = assign100490_e152603_d_n7;
        locals.var_xmax2_dn8 = assign100490_e152603_d_n8;
        locals.var_xmax2_dn9 = assign100490_e152603_d_n9;
        locals.var_xmax2_dn10 = assign100490_e152603_d_n10;
        locals.var_xmax2_dn13 = assign100490_e152603_d_n13;
        locals.var_xmax2_rv = 0.0;

        let (assign100500_e152611, assign100500_e152611_d_n0, assign100500_e152611_d_n2, assign100500_e152611_d_n4, assign100500_e152611_d_n5, assign100500_e152611_d_n6, assign100500_e152611_d_n7, assign100500_e152611_d_n8, assign100500_e152611_d_n9, assign100500_e152611_d_n10, assign100500_e152611_d_n13,) = {
    if (((locals.var_guard2311 != 0.0) && (locals.var_guard2312 != 0.0)) && (locals.var_guard2313 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
        locals.var_xp = assign100500_e152611;
        locals.var_xp_dn0 = assign100500_e152611_d_n0;
        locals.var_xp_dn2 = assign100500_e152611_d_n2;
        locals.var_xp_dn4 = assign100500_e152611_d_n4;
        locals.var_xp_dn5 = assign100500_e152611_d_n5;
        locals.var_xp_dn6 = assign100500_e152611_d_n6;
        locals.var_xp_dn7 = assign100500_e152611_d_n7;
        locals.var_xp_dn8 = assign100500_e152611_d_n8;
        locals.var_xp_dn9 = assign100500_e152611_d_n9;
        locals.var_xp_dn10 = assign100500_e152611_d_n10;
        locals.var_xp_dn13 = assign100500_e152611_d_n13;
        locals.var_xp_rv = 0.0;

        let (assign100510_e152619, assign100510_e152619_d_n0, assign100510_e152619_d_n2, assign100510_e152619_d_n4, assign100510_e152619_d_n5, assign100510_e152619_d_n6, assign100510_e152619_d_n7, assign100510_e152619_d_n8, assign100510_e152619_d_n9, assign100510_e152619_d_n10, assign100510_e152619_d_n13,) = {
    if (((locals.var_guard2311 != 0.0) && (locals.var_guard2312 != 0.0)) && (locals.var_guard2313 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
        locals.var_xmp = assign100510_e152619;
        locals.var_xmp_dn0 = assign100510_e152619_d_n0;
        locals.var_xmp_dn2 = assign100510_e152619_d_n2;
        locals.var_xmp_dn4 = assign100510_e152619_d_n4;
        locals.var_xmp_dn5 = assign100510_e152619_d_n5;
        locals.var_xmp_dn6 = assign100510_e152619_d_n6;
        locals.var_xmp_dn7 = assign100510_e152619_d_n7;
        locals.var_xmp_dn8 = assign100510_e152619_d_n8;
        locals.var_xmp_dn9 = assign100510_e152619_d_n9;
        locals.var_xmp_dn10 = assign100510_e152619_d_n10;
        locals.var_xmp_dn13 = assign100510_e152619_d_n13;
        locals.var_xmp_rv = 0.0;

        let (assign100520_e152627,) = {
    if (((locals.var_guard2311 != 0.0) && (locals.var_guard2312 != 0.0)) && (locals.var_guard2313 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign100520_e152627;
        locals.var_m0_rv = 0.0;

        let (assign100530_e152635,) = {
    if (((locals.var_guard2311 != 0.0) && (locals.var_guard2312 != 0.0)) && (locals.var_guard2313 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign100530_e152635;
        locals.var_mm_rv = 0.0;

        let (assign100540_e152643, assign100540_e152643_d_n0, assign100540_e152643_d_n2, assign100540_e152643_d_n4, assign100540_e152643_d_n5, assign100540_e152643_d_n6, assign100540_e152643_d_n7, assign100540_e152643_d_n8, assign100540_e152643_d_n9, assign100540_e152643_d_n10, assign100540_e152643_d_n13,) = {
    if (((locals.var_guard2311 != 0.0) && (locals.var_guard2312 != 0.0)) && (locals.var_guard2313 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    }
};
        locals.var_arg = assign100540_e152643;
        locals.var_arg_dn0 = assign100540_e152643_d_n0;
        locals.var_arg_dn2 = assign100540_e152643_d_n2;
        locals.var_arg_dn4 = assign100540_e152643_d_n4;
        locals.var_arg_dn5 = assign100540_e152643_d_n5;
        locals.var_arg_dn6 = assign100540_e152643_d_n6;
        locals.var_arg_dn7 = assign100540_e152643_d_n7;
        locals.var_arg_dn8 = assign100540_e152643_d_n8;
        locals.var_arg_dn9 = assign100540_e152643_d_n9;
        locals.var_arg_dn10 = assign100540_e152643_d_n10;
        locals.var_arg_dn13 = assign100540_e152643_d_n13;
        locals.var_arg_rv = 0.0;

        let (assign100550_e152651, assign100550_e152651_d_n0, assign100550_e152651_d_n2, assign100550_e152651_d_n4, assign100550_e152651_d_n5, assign100550_e152651_d_n6, assign100550_e152651_d_n7, assign100550_e152651_d_n8, assign100550_e152651_d_n9, assign100550_e152651_d_n10, assign100550_e152651_d_n13,) = {
    if (((locals.var_guard2311 != 0.0) && (locals.var_guard2312 != 0.0)) && (locals.var_guard2313 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign100550_e152651;
        locals.var_dnm_dn0 = assign100550_e152651_d_n0;
        locals.var_dnm_dn2 = assign100550_e152651_d_n2;
        locals.var_dnm_dn4 = assign100550_e152651_d_n4;
        locals.var_dnm_dn5 = assign100550_e152651_d_n5;
        locals.var_dnm_dn6 = assign100550_e152651_d_n6;
        locals.var_dnm_dn7 = assign100550_e152651_d_n7;
        locals.var_dnm_dn8 = assign100550_e152651_d_n8;
        locals.var_dnm_dn9 = assign100550_e152651_d_n9;
        locals.var_dnm_dn10 = assign100550_e152651_d_n10;
        locals.var_dnm_dn13 = assign100550_e152651_d_n13;
        locals.var_dnm_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_374(
        locals: &mut StampLocals,
    ) {
        let (assign100560_e152661, assign100560_e152661_d_n0, assign100560_e152661_d_n2, assign100560_e152661_d_n4, assign100560_e152661_d_n5, assign100560_e152661_d_n6, assign100560_e152661_d_n7, assign100560_e152661_d_n8, assign100560_e152661_d_n9, assign100560_e152661_d_n10, assign100560_e152661_d_n13,) = {
    if (((locals.var_guard2311 != 0.0) && (locals.var_guard2312 != 0.0)) && (locals.var_guard2313 != 0.0)) {
        let assign100560_e152659: f64 = (locals.var_xp * locals.var_x2);
        (assign100560_e152659, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn13 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn13)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
        locals.var_xp = assign100560_e152661;
        locals.var_xp_dn0 = assign100560_e152661_d_n0;
        locals.var_xp_dn2 = assign100560_e152661_d_n2;
        locals.var_xp_dn4 = assign100560_e152661_d_n4;
        locals.var_xp_dn5 = assign100560_e152661_d_n5;
        locals.var_xp_dn6 = assign100560_e152661_d_n6;
        locals.var_xp_dn7 = assign100560_e152661_d_n7;
        locals.var_xp_dn8 = assign100560_e152661_d_n8;
        locals.var_xp_dn9 = assign100560_e152661_d_n9;
        locals.var_xp_dn10 = assign100560_e152661_d_n10;
        locals.var_xp_dn13 = assign100560_e152661_d_n13;
        locals.var_xp_rv = 0.0;

        let (assign100570_e152671, assign100570_e152671_d_n0, assign100570_e152671_d_n2, assign100570_e152671_d_n4, assign100570_e152671_d_n5, assign100570_e152671_d_n6, assign100570_e152671_d_n7, assign100570_e152671_d_n8, assign100570_e152671_d_n9, assign100570_e152671_d_n10, assign100570_e152671_d_n13,) = {
    if (((locals.var_guard2311 != 0.0) && (locals.var_guard2312 != 0.0)) && (locals.var_guard2313 != 0.0)) {
        let assign100570_e152669: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign100570_e152669, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn13 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn13)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
        locals.var_xmp = assign100570_e152671;
        locals.var_xmp_dn0 = assign100570_e152671_d_n0;
        locals.var_xmp_dn2 = assign100570_e152671_d_n2;
        locals.var_xmp_dn4 = assign100570_e152671_d_n4;
        locals.var_xmp_dn5 = assign100570_e152671_d_n5;
        locals.var_xmp_dn6 = assign100570_e152671_d_n6;
        locals.var_xmp_dn7 = assign100570_e152671_d_n7;
        locals.var_xmp_dn8 = assign100570_e152671_d_n8;
        locals.var_xmp_dn9 = assign100570_e152671_d_n9;
        locals.var_xmp_dn10 = assign100570_e152671_d_n10;
        locals.var_xmp_dn13 = assign100570_e152671_d_n13;
        locals.var_xmp_rv = 0.0;

        let (assign100580_e152681, assign100580_e152681_d_n0, assign100580_e152681_d_n2, assign100580_e152681_d_n4, assign100580_e152681_d_n5, assign100580_e152681_d_n6, assign100580_e152681_d_n7, assign100580_e152681_d_n8, assign100580_e152681_d_n9, assign100580_e152681_d_n10, assign100580_e152681_d_n13,) = {
    if (((locals.var_guard2311 != 0.0) && (locals.var_guard2312 != 0.0)) && (locals.var_guard2313 != 0.0)) {
        let assign100580_e152679: f64 = (locals.var_xp * locals.var_x2);
        (assign100580_e152679, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn13 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn13)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
        locals.var_xp = assign100580_e152681;
        locals.var_xp_dn0 = assign100580_e152681_d_n0;
        locals.var_xp_dn2 = assign100580_e152681_d_n2;
        locals.var_xp_dn4 = assign100580_e152681_d_n4;
        locals.var_xp_dn5 = assign100580_e152681_d_n5;
        locals.var_xp_dn6 = assign100580_e152681_d_n6;
        locals.var_xp_dn7 = assign100580_e152681_d_n7;
        locals.var_xp_dn8 = assign100580_e152681_d_n8;
        locals.var_xp_dn9 = assign100580_e152681_d_n9;
        locals.var_xp_dn10 = assign100580_e152681_d_n10;
        locals.var_xp_dn13 = assign100580_e152681_d_n13;
        locals.var_xp_rv = 0.0;

        let (assign100590_e152691, assign100590_e152691_d_n0, assign100590_e152691_d_n2, assign100590_e152691_d_n4, assign100590_e152691_d_n5, assign100590_e152691_d_n6, assign100590_e152691_d_n7, assign100590_e152691_d_n8, assign100590_e152691_d_n9, assign100590_e152691_d_n10, assign100590_e152691_d_n13,) = {
    if (((locals.var_guard2311 != 0.0) && (locals.var_guard2312 != 0.0)) && (locals.var_guard2313 != 0.0)) {
        let assign100590_e152689: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign100590_e152689, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn13 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn13)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
        locals.var_xmp = assign100590_e152691;
        locals.var_xmp_dn0 = assign100590_e152691_d_n0;
        locals.var_xmp_dn2 = assign100590_e152691_d_n2;
        locals.var_xmp_dn4 = assign100590_e152691_d_n4;
        locals.var_xmp_dn5 = assign100590_e152691_d_n5;
        locals.var_xmp_dn6 = assign100590_e152691_d_n6;
        locals.var_xmp_dn7 = assign100590_e152691_d_n7;
        locals.var_xmp_dn8 = assign100590_e152691_d_n8;
        locals.var_xmp_dn9 = assign100590_e152691_d_n9;
        locals.var_xmp_dn10 = assign100590_e152691_d_n10;
        locals.var_xmp_dn13 = assign100590_e152691_d_n13;
        locals.var_xmp_rv = 0.0;

        let (assign100600_e152701, assign100600_e152701_d_n0, assign100600_e152701_d_n2, assign100600_e152701_d_n4, assign100600_e152701_d_n5, assign100600_e152701_d_n6, assign100600_e152701_d_n7, assign100600_e152701_d_n8, assign100600_e152701_d_n9, assign100600_e152701_d_n10, assign100600_e152701_d_n13,) = {
    if (((locals.var_guard2311 != 0.0) && (locals.var_guard2312 != 0.0)) && (locals.var_guard2313 != 0.0)) {
        let assign100600_e152699: f64 = (locals.var_xp * locals.var_x2);
        (assign100600_e152699, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn13 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn13)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
        locals.var_xp = assign100600_e152701;
        locals.var_xp_dn0 = assign100600_e152701_d_n0;
        locals.var_xp_dn2 = assign100600_e152701_d_n2;
        locals.var_xp_dn4 = assign100600_e152701_d_n4;
        locals.var_xp_dn5 = assign100600_e152701_d_n5;
        locals.var_xp_dn6 = assign100600_e152701_d_n6;
        locals.var_xp_dn7 = assign100600_e152701_d_n7;
        locals.var_xp_dn8 = assign100600_e152701_d_n8;
        locals.var_xp_dn9 = assign100600_e152701_d_n9;
        locals.var_xp_dn10 = assign100600_e152701_d_n10;
        locals.var_xp_dn13 = assign100600_e152701_d_n13;
        locals.var_xp_rv = 0.0;

        let (assign100610_e152711, assign100610_e152711_d_n0, assign100610_e152711_d_n2, assign100610_e152711_d_n4, assign100610_e152711_d_n5, assign100610_e152711_d_n6, assign100610_e152711_d_n7, assign100610_e152711_d_n8, assign100610_e152711_d_n9, assign100610_e152711_d_n10, assign100610_e152711_d_n13,) = {
    if (((locals.var_guard2311 != 0.0) && (locals.var_guard2312 != 0.0)) && (locals.var_guard2313 != 0.0)) {
        let assign100610_e152709: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign100610_e152709, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn13 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn13)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
        locals.var_xmp = assign100610_e152711;
        locals.var_xmp_dn0 = assign100610_e152711_d_n0;
        locals.var_xmp_dn2 = assign100610_e152711_d_n2;
        locals.var_xmp_dn4 = assign100610_e152711_d_n4;
        locals.var_xmp_dn5 = assign100610_e152711_d_n5;
        locals.var_xmp_dn6 = assign100610_e152711_d_n6;
        locals.var_xmp_dn7 = assign100610_e152711_d_n7;
        locals.var_xmp_dn8 = assign100610_e152711_d_n8;
        locals.var_xmp_dn9 = assign100610_e152711_d_n9;
        locals.var_xmp_dn10 = assign100610_e152711_d_n10;
        locals.var_xmp_dn13 = assign100610_e152711_d_n13;
        locals.var_xmp_rv = 0.0;

        let (assign100620_e152721, assign100620_e152721_d_n0, assign100620_e152721_d_n2, assign100620_e152721_d_n4, assign100620_e152721_d_n5, assign100620_e152721_d_n6, assign100620_e152721_d_n7, assign100620_e152721_d_n8, assign100620_e152721_d_n9, assign100620_e152721_d_n10, assign100620_e152721_d_n13,) = {
    if (((locals.var_guard2311 != 0.0) && (locals.var_guard2312 != 0.0)) && (locals.var_guard2313 != 0.0)) {
        let assign100620_e152719: f64 = (locals.var_xp * locals.var_x2);
        (assign100620_e152719, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn13 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn13)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
        locals.var_xp = assign100620_e152721;
        locals.var_xp_dn0 = assign100620_e152721_d_n0;
        locals.var_xp_dn2 = assign100620_e152721_d_n2;
        locals.var_xp_dn4 = assign100620_e152721_d_n4;
        locals.var_xp_dn5 = assign100620_e152721_d_n5;
        locals.var_xp_dn6 = assign100620_e152721_d_n6;
        locals.var_xp_dn7 = assign100620_e152721_d_n7;
        locals.var_xp_dn8 = assign100620_e152721_d_n8;
        locals.var_xp_dn9 = assign100620_e152721_d_n9;
        locals.var_xp_dn10 = assign100620_e152721_d_n10;
        locals.var_xp_dn13 = assign100620_e152721_d_n13;
        locals.var_xp_rv = 0.0;

        let (assign100630_e152731, assign100630_e152731_d_n0, assign100630_e152731_d_n2, assign100630_e152731_d_n4, assign100630_e152731_d_n5, assign100630_e152731_d_n6, assign100630_e152731_d_n7, assign100630_e152731_d_n8, assign100630_e152731_d_n9, assign100630_e152731_d_n10, assign100630_e152731_d_n13,) = {
    if (((locals.var_guard2311 != 0.0) && (locals.var_guard2312 != 0.0)) && (locals.var_guard2313 != 0.0)) {
        let assign100630_e152729: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign100630_e152729, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn13 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn13)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
        locals.var_xmp = assign100630_e152731;
        locals.var_xmp_dn0 = assign100630_e152731_d_n0;
        locals.var_xmp_dn2 = assign100630_e152731_d_n2;
        locals.var_xmp_dn4 = assign100630_e152731_d_n4;
        locals.var_xmp_dn5 = assign100630_e152731_d_n5;
        locals.var_xmp_dn6 = assign100630_e152731_d_n6;
        locals.var_xmp_dn7 = assign100630_e152731_d_n7;
        locals.var_xmp_dn8 = assign100630_e152731_d_n8;
        locals.var_xmp_dn9 = assign100630_e152731_d_n9;
        locals.var_xmp_dn10 = assign100630_e152731_d_n10;
        locals.var_xmp_dn13 = assign100630_e152731_d_n13;
        locals.var_xmp_rv = 0.0;

        let (assign100640_e152741, assign100640_e152741_d_n0, assign100640_e152741_d_n2, assign100640_e152741_d_n4, assign100640_e152741_d_n5, assign100640_e152741_d_n6, assign100640_e152741_d_n7, assign100640_e152741_d_n8, assign100640_e152741_d_n9, assign100640_e152741_d_n10, assign100640_e152741_d_n13,) = {
    if (((locals.var_guard2311 != 0.0) && (locals.var_guard2312 != 0.0)) && (locals.var_guard2313 != 0.0)) {
        let assign100640_e152739: f64 = (locals.var_xp + locals.var_xmp);
        (assign100640_e152739, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn4 + locals.var_xmp_dn4), (locals.var_xp_dn5 + locals.var_xmp_dn5), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn8 + locals.var_xmp_dn8), (locals.var_xp_dn9 + locals.var_xmp_dn9), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn13 + locals.var_xmp_dn13),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    }
};
        locals.var_arg = assign100640_e152741;
        locals.var_arg_dn0 = assign100640_e152741_d_n0;
        locals.var_arg_dn2 = assign100640_e152741_d_n2;
        locals.var_arg_dn4 = assign100640_e152741_d_n4;
        locals.var_arg_dn5 = assign100640_e152741_d_n5;
        locals.var_arg_dn6 = assign100640_e152741_d_n6;
        locals.var_arg_dn7 = assign100640_e152741_d_n7;
        locals.var_arg_dn8 = assign100640_e152741_d_n8;
        locals.var_arg_dn9 = assign100640_e152741_d_n9;
        locals.var_arg_dn10 = assign100640_e152741_d_n10;
        locals.var_arg_dn13 = assign100640_e152741_d_n13;
        locals.var_arg_rv = 0.0;

        let (assign100650_e152749, assign100650_e152749_d_n0, assign100650_e152749_d_n2, assign100650_e152749_d_n4, assign100650_e152749_d_n5, assign100650_e152749_d_n6, assign100650_e152749_d_n7, assign100650_e152749_d_n8, assign100650_e152749_d_n9, assign100650_e152749_d_n10, assign100650_e152749_d_n13,) = {
    if (((locals.var_guard2311 != 0.0) && (locals.var_guard2312 != 0.0)) && (locals.var_guard2313 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign100650_e152749;
        locals.var_dnm_dn0 = assign100650_e152749_d_n0;
        locals.var_dnm_dn2 = assign100650_e152749_d_n2;
        locals.var_dnm_dn4 = assign100650_e152749_d_n4;
        locals.var_dnm_dn5 = assign100650_e152749_d_n5;
        locals.var_dnm_dn6 = assign100650_e152749_d_n6;
        locals.var_dnm_dn7 = assign100650_e152749_d_n7;
        locals.var_dnm_dn8 = assign100650_e152749_d_n8;
        locals.var_dnm_dn9 = assign100650_e152749_d_n9;
        locals.var_dnm_dn10 = assign100650_e152749_d_n10;
        locals.var_dnm_dn13 = assign100650_e152749_d_n13;
        locals.var_dnm_rv = 0.0;

        let assign100660_e152764: f64 = if ((((4.0 == 1.0) || (4.0 == 2.0)) || (4.0 == 4.0)) || (4.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard2314 = assign100660_e152764;
        locals.var_guard2314_rv = 0.0;

        let assign100670_e152767: f64 = if 4.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard2315 = assign100670_e152767;
        locals.var_guard2315_rv = 0.0;

        let (assign100680_e152779,) = {
    if (((((locals.var_guard2311 != 0.0) && (locals.var_guard2312 != 0.0)) && (locals.var_guard2313 != 0.0)) && (locals.var_guard2314 != 0.0)) && (locals.var_guard2315 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign100680_e152779;
        locals.var_mm_rv = 0.0;

        let assign100690_e152782: f64 = if 4.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard2316 = assign100690_e152782;
        locals.var_guard2316_rv = 0.0;

        let (assign100700_e152797,) = {
    if ((((((locals.var_guard2311 != 0.0) && (locals.var_guard2312 != 0.0)) && (locals.var_guard2313 != 0.0)) && (locals.var_guard2314 != 0.0)) && (locals.var_guard2315 == 0.0)) && (locals.var_guard2316 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign100700_e152797;
        locals.var_mm_rv = 0.0;

        let assign100710_e152800: f64 = if 4.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard2317 = assign100710_e152800;
        locals.var_guard2317_rv = 0.0;

        let (assign100720_e152818,) = {
    if (((((((locals.var_guard2311 != 0.0) && (locals.var_guard2312 != 0.0)) && (locals.var_guard2313 != 0.0)) && (locals.var_guard2314 != 0.0)) && (locals.var_guard2315 == 0.0)) && (locals.var_guard2316 == 0.0)) && (locals.var_guard2317 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign100720_e152818;
        locals.var_mm_rv = 0.0;

        let assign100730_e152821: f64 = if 4.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard2318 = assign100730_e152821;
        locals.var_guard2318_rv = 0.0;

        let (assign100740_e152842,) = {
    if ((((((((locals.var_guard2311 != 0.0) && (locals.var_guard2312 != 0.0)) && (locals.var_guard2313 != 0.0)) && (locals.var_guard2314 != 0.0)) && (locals.var_guard2315 == 0.0)) && (locals.var_guard2316 == 0.0)) && (locals.var_guard2317 == 0.0)) && (locals.var_guard2318 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign100740_e152842;
        locals.var_mm_rv = 0.0;

        let (assign100750_e152852,) = {
    if ((((locals.var_guard2311 != 0.0) && (locals.var_guard2312 != 0.0)) && (locals.var_guard2313 != 0.0)) && (locals.var_guard2314 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign100750_e152852;
        locals.var_m0_rv = 0.0;

        let mut assign100760_loop_guard: usize = 0;
        while {
            let assign100760_cond_e152863: f64 = if (((((locals.var_guard2311 != 0.0) && (locals.var_guard2312 != 0.0)) && (locals.var_guard2313 != 0.0)) && (locals.var_guard2314 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign100760_cond_e152863 != 0.0
        } {
            assign100760_loop_guard += 1;
            assert!(assign100760_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign100760_body0_e152874, assign100760_body0_e152874_d_n0, assign100760_body0_e152874_d_n2, assign100760_body0_e152874_d_n4, assign100760_body0_e152874_d_n5, assign100760_body0_e152874_d_n6, assign100760_body0_e152874_d_n7, assign100760_body0_e152874_d_n8, assign100760_body0_e152874_d_n9, assign100760_body0_e152874_d_n10, assign100760_body0_e152874_d_n13,) = {
    if ((((locals.var_guard2311 != 0.0) && (locals.var_guard2312 != 0.0)) && (locals.var_guard2313 != 0.0)) && (locals.var_guard2314 != 0.0)) {
        let assign100760_body0_e152872: f64 = (locals.var_dnm).sqrt();
        (assign100760_body0_e152872, (locals.var_dnm_dn0 / (2.0 * assign100760_body0_e152872)), (locals.var_dnm_dn2 / (2.0 * assign100760_body0_e152872)), (locals.var_dnm_dn4 / (2.0 * assign100760_body0_e152872)), (locals.var_dnm_dn5 / (2.0 * assign100760_body0_e152872)), (locals.var_dnm_dn6 / (2.0 * assign100760_body0_e152872)), (locals.var_dnm_dn7 / (2.0 * assign100760_body0_e152872)), (locals.var_dnm_dn8 / (2.0 * assign100760_body0_e152872)), (locals.var_dnm_dn9 / (2.0 * assign100760_body0_e152872)), (locals.var_dnm_dn10 / (2.0 * assign100760_body0_e152872)), (locals.var_dnm_dn13 / (2.0 * assign100760_body0_e152872)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
            locals.var_dnm = assign100760_body0_e152874;
            locals.var_dnm_dn0 = assign100760_body0_e152874_d_n0;
            locals.var_dnm_dn2 = assign100760_body0_e152874_d_n2;
            locals.var_dnm_dn4 = assign100760_body0_e152874_d_n4;
            locals.var_dnm_dn5 = assign100760_body0_e152874_d_n5;
            locals.var_dnm_dn6 = assign100760_body0_e152874_d_n6;
            locals.var_dnm_dn7 = assign100760_body0_e152874_d_n7;
            locals.var_dnm_dn8 = assign100760_body0_e152874_d_n8;
            locals.var_dnm_dn9 = assign100760_body0_e152874_d_n9;
            locals.var_dnm_dn10 = assign100760_body0_e152874_d_n10;
            locals.var_dnm_dn13 = assign100760_body0_e152874_d_n13;
            locals.var_dnm_rv = 0.0;
            let (assign100760_body1_e152886,) = {
    if ((((locals.var_guard2311 != 0.0) && (locals.var_guard2312 != 0.0)) && (locals.var_guard2313 != 0.0)) && (locals.var_guard2314 != 0.0)) {
        let assign100760_body1_e152884: f64 = (locals.var_m0 + 1.0);
        (assign100760_body1_e152884,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign100760_body1_e152886;
            locals.var_m0_rv = 0.0;
        }

        let (assign100770_e152908, assign100770_e152908_d_n0, assign100770_e152908_d_n2, assign100770_e152908_d_n4, assign100770_e152908_d_n5, assign100770_e152908_d_n6, assign100770_e152908_d_n7, assign100770_e152908_d_n8, assign100770_e152908_d_n9, assign100770_e152908_d_n10, assign100770_e152908_d_n13,) = {
    if ((((locals.var_guard2311 != 0.0) && (locals.var_guard2312 != 0.0)) && (locals.var_guard2313 != 0.0)) && (locals.var_guard2314 == 0.0)) {
        let (assign100770_e152906, assign100770_e152906_d_n0, assign100770_e152906_d_n2, assign100770_e152906_d_n4, assign100770_e152906_d_n5, assign100770_e152906_d_n6, assign100770_e152906_d_n7, assign100770_e152906_d_n8, assign100770_e152906_d_n9, assign100770_e152906_d_n10, assign100770_e152906_d_n13,) = {
            if (locals.var_dnm == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign100770_e152903: f64 = (2.0 * 4.0);
                let assign100770_e152904: f64 = (1.0 / assign100770_e152903);
                let assign100770_e152905: f64 = (locals.var_dnm).powf(assign100770_e152904);
                (assign100770_e152905, if 0.0 == 0.0 && ((assign100770_e152904) as f64).is_finite() && ((assign100770_e152904) as f64).fract() == 0.0 { if assign100770_e152904 == 0.0 { 0.0 } else { (assign100770_e152904 * ((locals.var_dnm).powf(assign100770_e152904 - 1.0) * locals.var_dnm_dn0)) } } else { (assign100770_e152905 * (assign100770_e152904 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign100770_e152904) as f64).is_finite() && ((assign100770_e152904) as f64).fract() == 0.0 { if assign100770_e152904 == 0.0 { 0.0 } else { (assign100770_e152904 * ((locals.var_dnm).powf(assign100770_e152904 - 1.0) * locals.var_dnm_dn2)) } } else { (assign100770_e152905 * (assign100770_e152904 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign100770_e152904) as f64).is_finite() && ((assign100770_e152904) as f64).fract() == 0.0 { if assign100770_e152904 == 0.0 { 0.0 } else { (assign100770_e152904 * ((locals.var_dnm).powf(assign100770_e152904 - 1.0) * locals.var_dnm_dn4)) } } else { (assign100770_e152905 * (assign100770_e152904 * (locals.var_dnm_dn4 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign100770_e152904) as f64).is_finite() && ((assign100770_e152904) as f64).fract() == 0.0 { if assign100770_e152904 == 0.0 { 0.0 } else { (assign100770_e152904 * ((locals.var_dnm).powf(assign100770_e152904 - 1.0) * locals.var_dnm_dn5)) } } else { (assign100770_e152905 * (assign100770_e152904 * (locals.var_dnm_dn5 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign100770_e152904) as f64).is_finite() && ((assign100770_e152904) as f64).fract() == 0.0 { if assign100770_e152904 == 0.0 { 0.0 } else { (assign100770_e152904 * ((locals.var_dnm).powf(assign100770_e152904 - 1.0) * locals.var_dnm_dn6)) } } else { (assign100770_e152905 * (assign100770_e152904 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign100770_e152904) as f64).is_finite() && ((assign100770_e152904) as f64).fract() == 0.0 { if assign100770_e152904 == 0.0 { 0.0 } else { (assign100770_e152904 * ((locals.var_dnm).powf(assign100770_e152904 - 1.0) * locals.var_dnm_dn7)) } } else { (assign100770_e152905 * (assign100770_e152904 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign100770_e152904) as f64).is_finite() && ((assign100770_e152904) as f64).fract() == 0.0 { if assign100770_e152904 == 0.0 { 0.0 } else { (assign100770_e152904 * ((locals.var_dnm).powf(assign100770_e152904 - 1.0) * locals.var_dnm_dn8)) } } else { (assign100770_e152905 * (assign100770_e152904 * (locals.var_dnm_dn8 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign100770_e152904) as f64).is_finite() && ((assign100770_e152904) as f64).fract() == 0.0 { if assign100770_e152904 == 0.0 { 0.0 } else { (assign100770_e152904 * ((locals.var_dnm).powf(assign100770_e152904 - 1.0) * locals.var_dnm_dn9)) } } else { (assign100770_e152905 * (assign100770_e152904 * (locals.var_dnm_dn9 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign100770_e152904) as f64).is_finite() && ((assign100770_e152904) as f64).fract() == 0.0 { if assign100770_e152904 == 0.0 { 0.0 } else { (assign100770_e152904 * ((locals.var_dnm).powf(assign100770_e152904 - 1.0) * locals.var_dnm_dn10)) } } else { (assign100770_e152905 * (assign100770_e152904 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign100770_e152904) as f64).is_finite() && ((assign100770_e152904) as f64).fract() == 0.0 { if assign100770_e152904 == 0.0 { 0.0 } else { (assign100770_e152904 * ((locals.var_dnm).powf(assign100770_e152904 - 1.0) * locals.var_dnm_dn13)) } } else { (assign100770_e152905 * (assign100770_e152904 * (locals.var_dnm_dn13 / locals.var_dnm))) },)
            }
        };
        (assign100770_e152906, assign100770_e152906_d_n0, assign100770_e152906_d_n2, assign100770_e152906_d_n4, assign100770_e152906_d_n5, assign100770_e152906_d_n6, assign100770_e152906_d_n7, assign100770_e152906_d_n8, assign100770_e152906_d_n9, assign100770_e152906_d_n10, assign100770_e152906_d_n13,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign100770_e152908;
        locals.var_dnm_dn0 = assign100770_e152908_d_n0;
        locals.var_dnm_dn2 = assign100770_e152908_d_n2;
        locals.var_dnm_dn4 = assign100770_e152908_d_n4;
        locals.var_dnm_dn5 = assign100770_e152908_d_n5;
        locals.var_dnm_dn6 = assign100770_e152908_d_n6;
        locals.var_dnm_dn7 = assign100770_e152908_d_n7;
        locals.var_dnm_dn8 = assign100770_e152908_d_n8;
        locals.var_dnm_dn9 = assign100770_e152908_d_n9;
        locals.var_dnm_dn10 = assign100770_e152908_d_n10;
        locals.var_dnm_dn13 = assign100770_e152908_d_n13;
        locals.var_dnm_rv = 0.0;

        let (assign100780_e152918, assign100780_e152918_d_n0, assign100780_e152918_d_n2, assign100780_e152918_d_n4, assign100780_e152918_d_n5, assign100780_e152918_d_n6, assign100780_e152918_d_n7, assign100780_e152918_d_n8, assign100780_e152918_d_n9, assign100780_e152918_d_n10, assign100780_e152918_d_n13,) = {
    if (((locals.var_guard2311 != 0.0) && (locals.var_guard2312 != 0.0)) && (locals.var_guard2313 != 0.0)) {
        let assign100780_e152916: f64 = (1.0 / locals.var_dnm);
        (assign100780_e152916, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn4 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn5 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn8 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn9 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn13 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign100780_e152918;
        locals.var_dnm_dn0 = assign100780_e152918_d_n0;
        locals.var_dnm_dn2 = assign100780_e152918_d_n2;
        locals.var_dnm_dn4 = assign100780_e152918_d_n4;
        locals.var_dnm_dn5 = assign100780_e152918_d_n5;
        locals.var_dnm_dn6 = assign100780_e152918_d_n6;
        locals.var_dnm_dn7 = assign100780_e152918_d_n7;
        locals.var_dnm_dn8 = assign100780_e152918_d_n8;
        locals.var_dnm_dn9 = assign100780_e152918_d_n9;
        locals.var_dnm_dn10 = assign100780_e152918_d_n10;
        locals.var_dnm_dn13 = assign100780_e152918_d_n13;
        locals.var_dnm_rv = 0.0;

        let (assign100790_e152930, assign100790_e152930_d_n0, assign100790_e152930_d_n2, assign100790_e152930_d_n4, assign100790_e152930_d_n5, assign100790_e152930_d_n6, assign100790_e152930_d_n7, assign100790_e152930_d_n8, assign100790_e152930_d_n9, assign100790_e152930_d_n10, assign100790_e152930_d_n13,) = {
    if (((locals.var_guard2311 != 0.0) && (locals.var_guard2312 != 0.0)) && (locals.var_guard2313 != 0.0)) {
        let assign100790_e152926: f64 = (locals.var_tmf1 * locals.var_t7);
        let assign100790_e152928: f64 = (assign100790_e152926 * locals.var_dnm);
        (assign100790_e152928, ((((locals.var_tmf1_dn0 * locals.var_t7) + (locals.var_tmf1 * locals.var_t7_dn0)) * locals.var_dnm) + (assign100790_e152926 * locals.var_dnm_dn0)), ((((locals.var_tmf1_dn2 * locals.var_t7) + (locals.var_tmf1 * locals.var_t7_dn2)) * locals.var_dnm) + (assign100790_e152926 * locals.var_dnm_dn2)), ((((locals.var_tmf1_dn4 * locals.var_t7) + (locals.var_tmf1 * locals.var_t7_dn4)) * locals.var_dnm) + (assign100790_e152926 * locals.var_dnm_dn4)), ((((locals.var_tmf1_dn5 * locals.var_t7) + (locals.var_tmf1 * locals.var_t7_dn5)) * locals.var_dnm) + (assign100790_e152926 * locals.var_dnm_dn5)), ((((locals.var_tmf1_dn6 * locals.var_t7) + (locals.var_tmf1 * locals.var_t7_dn6)) * locals.var_dnm) + (assign100790_e152926 * locals.var_dnm_dn6)), ((((locals.var_tmf1_dn7 * locals.var_t7) + (locals.var_tmf1 * locals.var_t7_dn7)) * locals.var_dnm) + (assign100790_e152926 * locals.var_dnm_dn7)), ((((locals.var_tmf1_dn8 * locals.var_t7) + (locals.var_tmf1 * locals.var_t7_dn8)) * locals.var_dnm) + (assign100790_e152926 * locals.var_dnm_dn8)), ((((locals.var_tmf1_dn9 * locals.var_t7) + (locals.var_tmf1 * locals.var_t7_dn9)) * locals.var_dnm) + (assign100790_e152926 * locals.var_dnm_dn9)), ((((locals.var_tmf1_dn10 * locals.var_t7) + (locals.var_tmf1 * locals.var_t7_dn10)) * locals.var_dnm) + (assign100790_e152926 * locals.var_dnm_dn10)), ((((locals.var_tmf1_dn13 * locals.var_t7) + (locals.var_tmf1 * locals.var_t7_dn13)) * locals.var_dnm) + (assign100790_e152926 * locals.var_dnm_dn13)),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn13,)
    }
};
        locals.var_tmf0 = assign100790_e152930;
        locals.var_tmf0_dn0 = assign100790_e152930_d_n0;
        locals.var_tmf0_dn2 = assign100790_e152930_d_n2;
        locals.var_tmf0_dn4 = assign100790_e152930_d_n4;
        locals.var_tmf0_dn5 = assign100790_e152930_d_n5;
        locals.var_tmf0_dn6 = assign100790_e152930_d_n6;
        locals.var_tmf0_dn7 = assign100790_e152930_d_n7;
        locals.var_tmf0_dn8 = assign100790_e152930_d_n8;
        locals.var_tmf0_dn9 = assign100790_e152930_d_n9;
        locals.var_tmf0_dn10 = assign100790_e152930_d_n10;
        locals.var_tmf0_dn13 = assign100790_e152930_d_n13;
        locals.var_tmf0_rv = 0.0;

        let (assign100800_e152944, assign100800_e152944_d_n0, assign100800_e152944_d_n2, assign100800_e152944_d_n4, assign100800_e152944_d_n5, assign100800_e152944_d_n6, assign100800_e152944_d_n7, assign100800_e152944_d_n8, assign100800_e152944_d_n9, assign100800_e152944_d_n10, assign100800_e152944_d_n13,) = {
    if (((locals.var_guard2311 != 0.0) && (locals.var_guard2312 != 0.0)) && (locals.var_guard2313 != 0.0)) {
        let assign100800_e152938: f64 = (locals.var_t7 * locals.var_xmp);
        let assign100800_e152940: f64 = (assign100800_e152938 * locals.var_dnm);
        let assign100800_e152942: f64 = (assign100800_e152940 / locals.var_arg);
        (assign100800_e152942, (((((((locals.var_t7_dn0 * locals.var_xmp) + (locals.var_t7 * locals.var_xmp_dn0)) * locals.var_dnm) + (assign100800_e152938 * locals.var_dnm_dn0)) * locals.var_arg) - (assign100800_e152940 * locals.var_arg_dn0)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t7_dn2 * locals.var_xmp) + (locals.var_t7 * locals.var_xmp_dn2)) * locals.var_dnm) + (assign100800_e152938 * locals.var_dnm_dn2)) * locals.var_arg) - (assign100800_e152940 * locals.var_arg_dn2)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t7_dn4 * locals.var_xmp) + (locals.var_t7 * locals.var_xmp_dn4)) * locals.var_dnm) + (assign100800_e152938 * locals.var_dnm_dn4)) * locals.var_arg) - (assign100800_e152940 * locals.var_arg_dn4)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t7_dn5 * locals.var_xmp) + (locals.var_t7 * locals.var_xmp_dn5)) * locals.var_dnm) + (assign100800_e152938 * locals.var_dnm_dn5)) * locals.var_arg) - (assign100800_e152940 * locals.var_arg_dn5)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t7_dn6 * locals.var_xmp) + (locals.var_t7 * locals.var_xmp_dn6)) * locals.var_dnm) + (assign100800_e152938 * locals.var_dnm_dn6)) * locals.var_arg) - (assign100800_e152940 * locals.var_arg_dn6)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t7_dn7 * locals.var_xmp) + (locals.var_t7 * locals.var_xmp_dn7)) * locals.var_dnm) + (assign100800_e152938 * locals.var_dnm_dn7)) * locals.var_arg) - (assign100800_e152940 * locals.var_arg_dn7)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t7_dn8 * locals.var_xmp) + (locals.var_t7 * locals.var_xmp_dn8)) * locals.var_dnm) + (assign100800_e152938 * locals.var_dnm_dn8)) * locals.var_arg) - (assign100800_e152940 * locals.var_arg_dn8)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t7_dn9 * locals.var_xmp) + (locals.var_t7 * locals.var_xmp_dn9)) * locals.var_dnm) + (assign100800_e152938 * locals.var_dnm_dn9)) * locals.var_arg) - (assign100800_e152940 * locals.var_arg_dn9)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t7_dn10 * locals.var_xmp) + (locals.var_t7 * locals.var_xmp_dn10)) * locals.var_dnm) + (assign100800_e152938 * locals.var_dnm_dn10)) * locals.var_arg) - (assign100800_e152940 * locals.var_arg_dn10)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t7_dn13 * locals.var_xmp) + (locals.var_t7 * locals.var_xmp_dn13)) * locals.var_dnm) + (assign100800_e152938 * locals.var_dnm_dn13)) * locals.var_arg) - (assign100800_e152940 * locals.var_arg_dn13)) / (locals.var_arg * locals.var_arg)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign100800_e152944;
        locals.var_t0_dn0 = assign100800_e152944_d_n0;
        locals.var_t0_dn2 = assign100800_e152944_d_n2;
        locals.var_t0_dn4 = assign100800_e152944_d_n4;
        locals.var_t0_dn5 = assign100800_e152944_d_n5;
        locals.var_t0_dn6 = assign100800_e152944_d_n6;
        locals.var_t0_dn7 = assign100800_e152944_d_n7;
        locals.var_t0_dn8 = assign100800_e152944_d_n8;
        locals.var_t0_dn9 = assign100800_e152944_d_n9;
        locals.var_t0_dn10 = assign100800_e152944_d_n10;
        locals.var_t0_dn13 = assign100800_e152944_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign100810_e152956, assign100810_e152956_d_n0, assign100810_e152956_d_n2, assign100810_e152956_d_n4, assign100810_e152956_d_n5, assign100810_e152956_d_n6, assign100810_e152956_d_n7, assign100810_e152956_d_n8, assign100810_e152956_d_n9, assign100810_e152956_d_n10, assign100810_e152956_d_n13,) = {
    if (((locals.var_guard2311 != 0.0) && (locals.var_guard2312 != 0.0)) && (locals.var_guard2313 != 0.0)) {
        let assign100810_e152952: f64 = locals.var_t7;
        let assign100810_e152954: f64 = (assign100810_e152952 - locals.var_tmf0);
        (assign100810_e152954, (locals.var_t7_dn0 - locals.var_tmf0_dn0), (locals.var_t7_dn2 - locals.var_tmf0_dn2), (locals.var_t7_dn4 - locals.var_tmf0_dn4), (locals.var_t7_dn5 - locals.var_tmf0_dn5), (locals.var_t7_dn6 - locals.var_tmf0_dn6), (locals.var_t7_dn7 - locals.var_tmf0_dn7), (locals.var_t7_dn8 - locals.var_tmf0_dn8), (locals.var_t7_dn9 - locals.var_tmf0_dn9), (locals.var_t7_dn10 - locals.var_tmf0_dn10), (locals.var_t7_dn13 - locals.var_tmf0_dn13),)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn13,)
    }
};
        locals.var_t6 = assign100810_e152956;
        locals.var_t6_dn0 = assign100810_e152956_d_n0;
        locals.var_t6_dn2 = assign100810_e152956_d_n2;
        locals.var_t6_dn4 = assign100810_e152956_d_n4;
        locals.var_t6_dn5 = assign100810_e152956_d_n5;
        locals.var_t6_dn6 = assign100810_e152956_d_n6;
        locals.var_t6_dn7 = assign100810_e152956_d_n7;
        locals.var_t6_dn8 = assign100810_e152956_d_n8;
        locals.var_t6_dn9 = assign100810_e152956_d_n9;
        locals.var_t6_dn10 = assign100810_e152956_d_n10;
        locals.var_t6_dn13 = assign100810_e152956_d_n13;
        locals.var_t6_rv = 0.0;

        let (assign100820_e152964, assign100820_e152964_d_n0, assign100820_e152964_d_n2, assign100820_e152964_d_n4, assign100820_e152964_d_n5, assign100820_e152964_d_n6, assign100820_e152964_d_n7, assign100820_e152964_d_n8, assign100820_e152964_d_n9, assign100820_e152964_d_n10, assign100820_e152964_d_n13,) = {
    if (((locals.var_guard2311 != 0.0) && (locals.var_guard2312 != 0.0)) && (locals.var_guard2313 != 0.0)) {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign100820_e152964;
        locals.var_t0_dn0 = assign100820_e152964_d_n0;
        locals.var_t0_dn2 = assign100820_e152964_d_n2;
        locals.var_t0_dn4 = assign100820_e152964_d_n4;
        locals.var_t0_dn5 = assign100820_e152964_d_n5;
        locals.var_t0_dn6 = assign100820_e152964_d_n6;
        locals.var_t0_dn7 = assign100820_e152964_d_n7;
        locals.var_t0_dn8 = assign100820_e152964_d_n8;
        locals.var_t0_dn9 = assign100820_e152964_d_n9;
        locals.var_t0_dn10 = assign100820_e152964_d_n10;
        locals.var_t0_dn13 = assign100820_e152964_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign100830_e152973, assign100830_e152973_d_n0, assign100830_e152973_d_n2, assign100830_e152973_d_n4, assign100830_e152973_d_n5, assign100830_e152973_d_n6, assign100830_e152973_d_n7, assign100830_e152973_d_n8, assign100830_e152973_d_n9, assign100830_e152973_d_n10, assign100830_e152973_d_n13,) = {
    if (((locals.var_guard2311 != 0.0) && (locals.var_guard2312 != 0.0)) && (locals.var_guard2313 == 0.0)) {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn13,)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn13,)
    }
};
        locals.var_t6 = assign100830_e152973;
        locals.var_t6_dn0 = assign100830_e152973_d_n0;
        locals.var_t6_dn2 = assign100830_e152973_d_n2;
        locals.var_t6_dn4 = assign100830_e152973_d_n4;
        locals.var_t6_dn5 = assign100830_e152973_d_n5;
        locals.var_t6_dn6 = assign100830_e152973_d_n6;
        locals.var_t6_dn7 = assign100830_e152973_d_n7;
        locals.var_t6_dn8 = assign100830_e152973_d_n8;
        locals.var_t6_dn9 = assign100830_e152973_d_n9;
        locals.var_t6_dn10 = assign100830_e152973_d_n10;
        locals.var_t6_dn13 = assign100830_e152973_d_n13;
        locals.var_t6_rv = 0.0;

        let (assign100840_e152982, assign100840_e152982_d_n0, assign100840_e152982_d_n2, assign100840_e152982_d_n4, assign100840_e152982_d_n5, assign100840_e152982_d_n6, assign100840_e152982_d_n7, assign100840_e152982_d_n8, assign100840_e152982_d_n9, assign100840_e152982_d_n10, assign100840_e152982_d_n13,) = {
    if (((locals.var_guard2311 != 0.0) && (locals.var_guard2312 != 0.0)) && (locals.var_guard2313 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign100840_e152982;
        locals.var_t0_dn0 = assign100840_e152982_d_n0;
        locals.var_t0_dn2 = assign100840_e152982_d_n2;
        locals.var_t0_dn4 = assign100840_e152982_d_n4;
        locals.var_t0_dn5 = assign100840_e152982_d_n5;
        locals.var_t0_dn6 = assign100840_e152982_d_n6;
        locals.var_t0_dn7 = assign100840_e152982_d_n7;
        locals.var_t0_dn8 = assign100840_e152982_d_n8;
        locals.var_t0_dn9 = assign100840_e152982_d_n9;
        locals.var_t0_dn10 = assign100840_e152982_d_n10;
        locals.var_t0_dn13 = assign100840_e152982_d_n13;
        locals.var_t0_rv = 0.0;

    }
}
