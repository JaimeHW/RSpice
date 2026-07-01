#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_transient_block_368(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let mut assign104390_loop_guard: usize = 0;
        while {
            let assign104390_cond_e156585: f64 = if (((((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2371 != 0.0)) && (locals.var_guard2372 != 0.0)) && (locals.var_m0 < p.p442)) { 1.0 } else { 0.0 };
            assign104390_cond_e156585 != 0.0
        } {
            assign104390_loop_guard += 1;
            assert!(assign104390_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign104390_body0_e156598, assign104390_body0_e156598_d_n0, assign104390_body0_e156598_d_n2, assign104390_body0_e156598_d_n4, assign104390_body0_e156598_d_n5, assign104390_body0_e156598_d_n6, assign104390_body0_e156598_d_n7, assign104390_body0_e156598_d_n8, assign104390_body0_e156598_d_n9, assign104390_body0_e156598_d_n10, assign104390_body0_e156598_d_n13,) = {
    if ((((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2371 != 0.0)) && (locals.var_guard2372 != 0.0)) {
        let assign104390_body0_e156596: f64 = (locals.var_xp * locals.var_x2);
        (assign104390_body0_e156596, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn13 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn13)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
            locals.var_xp = assign104390_body0_e156598;
            locals.var_xp_dn0 = assign104390_body0_e156598_d_n0;
            locals.var_xp_dn2 = assign104390_body0_e156598_d_n2;
            locals.var_xp_dn4 = assign104390_body0_e156598_d_n4;
            locals.var_xp_dn5 = assign104390_body0_e156598_d_n5;
            locals.var_xp_dn6 = assign104390_body0_e156598_d_n6;
            locals.var_xp_dn7 = assign104390_body0_e156598_d_n7;
            locals.var_xp_dn8 = assign104390_body0_e156598_d_n8;
            locals.var_xp_dn9 = assign104390_body0_e156598_d_n9;
            locals.var_xp_dn10 = assign104390_body0_e156598_d_n10;
            locals.var_xp_dn13 = assign104390_body0_e156598_d_n13;
            let (assign104390_body1_e156611, assign104390_body1_e156611_d_n0, assign104390_body1_e156611_d_n2, assign104390_body1_e156611_d_n4, assign104390_body1_e156611_d_n5, assign104390_body1_e156611_d_n6, assign104390_body1_e156611_d_n7, assign104390_body1_e156611_d_n8, assign104390_body1_e156611_d_n9, assign104390_body1_e156611_d_n10, assign104390_body1_e156611_d_n13,) = {
    if ((((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2371 != 0.0)) && (locals.var_guard2372 != 0.0)) {
        let assign104390_body1_e156609: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign104390_body1_e156609, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn13 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn13)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
            locals.var_xmp = assign104390_body1_e156611;
            locals.var_xmp_dn0 = assign104390_body1_e156611_d_n0;
            locals.var_xmp_dn2 = assign104390_body1_e156611_d_n2;
            locals.var_xmp_dn4 = assign104390_body1_e156611_d_n4;
            locals.var_xmp_dn5 = assign104390_body1_e156611_d_n5;
            locals.var_xmp_dn6 = assign104390_body1_e156611_d_n6;
            locals.var_xmp_dn7 = assign104390_body1_e156611_d_n7;
            locals.var_xmp_dn8 = assign104390_body1_e156611_d_n8;
            locals.var_xmp_dn9 = assign104390_body1_e156611_d_n9;
            locals.var_xmp_dn10 = assign104390_body1_e156611_d_n10;
            locals.var_xmp_dn13 = assign104390_body1_e156611_d_n13;
            let (assign104390_body2_e156624,) = {
    if ((((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2371 != 0.0)) && (locals.var_guard2372 != 0.0)) {
        let assign104390_body2_e156622: f64 = (locals.var_m0 + 1.0);
        (assign104390_body2_e156622,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign104390_body2_e156624;
        }

        let (assign104400_e156637, assign104400_e156637_d_n0, assign104400_e156637_d_n2, assign104400_e156637_d_n4, assign104400_e156637_d_n5, assign104400_e156637_d_n6, assign104400_e156637_d_n7, assign104400_e156637_d_n8, assign104400_e156637_d_n9, assign104400_e156637_d_n10, assign104400_e156637_d_n13,) = {
    if ((((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2371 != 0.0)) && (locals.var_guard2372 != 0.0)) {
        let assign104400_e156635: f64 = (locals.var_xp + locals.var_xmp);
        (assign104400_e156635, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn4 + locals.var_xmp_dn4), (locals.var_xp_dn5 + locals.var_xmp_dn5), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn8 + locals.var_xmp_dn8), (locals.var_xp_dn9 + locals.var_xmp_dn9), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn13 + locals.var_xmp_dn13),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    }
};
        locals.var_arg = assign104400_e156637;
        locals.var_arg_dn0 = assign104400_e156637_d_n0;
        locals.var_arg_dn2 = assign104400_e156637_d_n2;
        locals.var_arg_dn4 = assign104400_e156637_d_n4;
        locals.var_arg_dn5 = assign104400_e156637_d_n5;
        locals.var_arg_dn6 = assign104400_e156637_d_n6;
        locals.var_arg_dn7 = assign104400_e156637_d_n7;
        locals.var_arg_dn8 = assign104400_e156637_d_n8;
        locals.var_arg_dn9 = assign104400_e156637_d_n9;
        locals.var_arg_dn10 = assign104400_e156637_d_n10;
        locals.var_arg_dn13 = assign104400_e156637_d_n13;

        let (assign104410_e156648, assign104410_e156648_d_n0, assign104410_e156648_d_n2, assign104410_e156648_d_n4, assign104410_e156648_d_n5, assign104410_e156648_d_n6, assign104410_e156648_d_n7, assign104410_e156648_d_n8, assign104410_e156648_d_n9, assign104410_e156648_d_n10, assign104410_e156648_d_n13,) = {
    if ((((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2371 != 0.0)) && (locals.var_guard2372 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign104410_e156648;
        locals.var_dnm_dn0 = assign104410_e156648_d_n0;
        locals.var_dnm_dn2 = assign104410_e156648_d_n2;
        locals.var_dnm_dn4 = assign104410_e156648_d_n4;
        locals.var_dnm_dn5 = assign104410_e156648_d_n5;
        locals.var_dnm_dn6 = assign104410_e156648_d_n6;
        locals.var_dnm_dn7 = assign104410_e156648_d_n7;
        locals.var_dnm_dn8 = assign104410_e156648_d_n8;
        locals.var_dnm_dn9 = assign104410_e156648_d_n9;
        locals.var_dnm_dn10 = assign104410_e156648_d_n10;
        locals.var_dnm_dn13 = assign104410_e156648_d_n13;

        let assign104420_e156663: f64 = if ((((p.p442 == 1.0) || (p.p442 == 2.0)) || (p.p442 == 4.0)) || (p.p442 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard2373 = assign104420_e156663;

        let assign104430_e156666: f64 = if p.p442 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard2374 = assign104430_e156666;

        let (assign104440_e156681,) = {
    if ((((((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2371 != 0.0)) && (locals.var_guard2372 != 0.0)) && (locals.var_guard2373 != 0.0)) && (locals.var_guard2374 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign104440_e156681;

        let assign104450_e156684: f64 = if p.p442 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard2375 = assign104450_e156684;

        let (assign104460_e156702,) = {
    if (((((((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2371 != 0.0)) && (locals.var_guard2372 != 0.0)) && (locals.var_guard2373 != 0.0)) && (locals.var_guard2374 == 0.0)) && (locals.var_guard2375 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign104460_e156702;

        let assign104470_e156705: f64 = if p.p442 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard2376 = assign104470_e156705;

        let (assign104480_e156726,) = {
    if ((((((((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2371 != 0.0)) && (locals.var_guard2372 != 0.0)) && (locals.var_guard2373 != 0.0)) && (locals.var_guard2374 == 0.0)) && (locals.var_guard2375 == 0.0)) && (locals.var_guard2376 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign104480_e156726;

        let assign104490_e156729: f64 = if p.p442 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard2377 = assign104490_e156729;

        let (assign104500_e156753,) = {
    if (((((((((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2371 != 0.0)) && (locals.var_guard2372 != 0.0)) && (locals.var_guard2373 != 0.0)) && (locals.var_guard2374 == 0.0)) && (locals.var_guard2375 == 0.0)) && (locals.var_guard2376 == 0.0)) && (locals.var_guard2377 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign104500_e156753;

        let (assign104510_e156766,) = {
    if (((((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2371 != 0.0)) && (locals.var_guard2372 != 0.0)) && (locals.var_guard2373 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign104510_e156766;

        let mut assign104520_loop_guard: usize = 0;
        while {
            let assign104520_cond_e156780: f64 = if ((((((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2371 != 0.0)) && (locals.var_guard2372 != 0.0)) && (locals.var_guard2373 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign104520_cond_e156780 != 0.0
        } {
            assign104520_loop_guard += 1;
            assert!(assign104520_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign104520_body0_e156794, assign104520_body0_e156794_d_n0, assign104520_body0_e156794_d_n2, assign104520_body0_e156794_d_n4, assign104520_body0_e156794_d_n5, assign104520_body0_e156794_d_n6, assign104520_body0_e156794_d_n7, assign104520_body0_e156794_d_n8, assign104520_body0_e156794_d_n9, assign104520_body0_e156794_d_n10, assign104520_body0_e156794_d_n13,) = {
    if (((((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2371 != 0.0)) && (locals.var_guard2372 != 0.0)) && (locals.var_guard2373 != 0.0)) {
        let assign104520_body0_e156792: f64 = (locals.var_dnm).sqrt();
        (assign104520_body0_e156792, (locals.var_dnm_dn0 / (2.0 * assign104520_body0_e156792)), (locals.var_dnm_dn2 / (2.0 * assign104520_body0_e156792)), (locals.var_dnm_dn4 / (2.0 * assign104520_body0_e156792)), (locals.var_dnm_dn5 / (2.0 * assign104520_body0_e156792)), (locals.var_dnm_dn6 / (2.0 * assign104520_body0_e156792)), (locals.var_dnm_dn7 / (2.0 * assign104520_body0_e156792)), (locals.var_dnm_dn8 / (2.0 * assign104520_body0_e156792)), (locals.var_dnm_dn9 / (2.0 * assign104520_body0_e156792)), (locals.var_dnm_dn10 / (2.0 * assign104520_body0_e156792)), (locals.var_dnm_dn13 / (2.0 * assign104520_body0_e156792)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
            locals.var_dnm = assign104520_body0_e156794;
            locals.var_dnm_dn0 = assign104520_body0_e156794_d_n0;
            locals.var_dnm_dn2 = assign104520_body0_e156794_d_n2;
            locals.var_dnm_dn4 = assign104520_body0_e156794_d_n4;
            locals.var_dnm_dn5 = assign104520_body0_e156794_d_n5;
            locals.var_dnm_dn6 = assign104520_body0_e156794_d_n6;
            locals.var_dnm_dn7 = assign104520_body0_e156794_d_n7;
            locals.var_dnm_dn8 = assign104520_body0_e156794_d_n8;
            locals.var_dnm_dn9 = assign104520_body0_e156794_d_n9;
            locals.var_dnm_dn10 = assign104520_body0_e156794_d_n10;
            locals.var_dnm_dn13 = assign104520_body0_e156794_d_n13;
            let (assign104520_body1_e156809,) = {
    if (((((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2371 != 0.0)) && (locals.var_guard2372 != 0.0)) && (locals.var_guard2373 != 0.0)) {
        let assign104520_body1_e156807: f64 = (locals.var_m0 + 1.0);
        (assign104520_body1_e156807,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign104520_body1_e156809;
        }

        let (assign104530_e156834, assign104530_e156834_d_n0, assign104530_e156834_d_n2, assign104530_e156834_d_n4, assign104530_e156834_d_n5, assign104530_e156834_d_n6, assign104530_e156834_d_n7, assign104530_e156834_d_n8, assign104530_e156834_d_n9, assign104530_e156834_d_n10, assign104530_e156834_d_n13,) = {
    if (((((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2371 != 0.0)) && (locals.var_guard2372 != 0.0)) && (locals.var_guard2373 == 0.0)) {
        let (assign104530_e156832, assign104530_e156832_d_n0, assign104530_e156832_d_n2, assign104530_e156832_d_n4, assign104530_e156832_d_n5, assign104530_e156832_d_n6, assign104530_e156832_d_n7, assign104530_e156832_d_n8, assign104530_e156832_d_n9, assign104530_e156832_d_n10, assign104530_e156832_d_n13,) = {
            if (locals.var_dnm == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign104530_e156829: f64 = (2.0 * p.p442);
                let assign104530_e156830: f64 = (1.0 / assign104530_e156829);
                let assign104530_e156831: f64 = (locals.var_dnm).powf(assign104530_e156830);
                (assign104530_e156831, if 0.0 == 0.0 && ((assign104530_e156830) as f64).is_finite() && ((assign104530_e156830) as f64).fract() == 0.0 { if assign104530_e156830 == 0.0 { 0.0 } else { (assign104530_e156830 * ((locals.var_dnm).powf(assign104530_e156830 - 1.0) * locals.var_dnm_dn0)) } } else { (assign104530_e156831 * (assign104530_e156830 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign104530_e156830) as f64).is_finite() && ((assign104530_e156830) as f64).fract() == 0.0 { if assign104530_e156830 == 0.0 { 0.0 } else { (assign104530_e156830 * ((locals.var_dnm).powf(assign104530_e156830 - 1.0) * locals.var_dnm_dn2)) } } else { (assign104530_e156831 * (assign104530_e156830 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign104530_e156830) as f64).is_finite() && ((assign104530_e156830) as f64).fract() == 0.0 { if assign104530_e156830 == 0.0 { 0.0 } else { (assign104530_e156830 * ((locals.var_dnm).powf(assign104530_e156830 - 1.0) * locals.var_dnm_dn4)) } } else { (assign104530_e156831 * (assign104530_e156830 * (locals.var_dnm_dn4 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign104530_e156830) as f64).is_finite() && ((assign104530_e156830) as f64).fract() == 0.0 { if assign104530_e156830 == 0.0 { 0.0 } else { (assign104530_e156830 * ((locals.var_dnm).powf(assign104530_e156830 - 1.0) * locals.var_dnm_dn5)) } } else { (assign104530_e156831 * (assign104530_e156830 * (locals.var_dnm_dn5 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign104530_e156830) as f64).is_finite() && ((assign104530_e156830) as f64).fract() == 0.0 { if assign104530_e156830 == 0.0 { 0.0 } else { (assign104530_e156830 * ((locals.var_dnm).powf(assign104530_e156830 - 1.0) * locals.var_dnm_dn6)) } } else { (assign104530_e156831 * (assign104530_e156830 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign104530_e156830) as f64).is_finite() && ((assign104530_e156830) as f64).fract() == 0.0 { if assign104530_e156830 == 0.0 { 0.0 } else { (assign104530_e156830 * ((locals.var_dnm).powf(assign104530_e156830 - 1.0) * locals.var_dnm_dn7)) } } else { (assign104530_e156831 * (assign104530_e156830 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign104530_e156830) as f64).is_finite() && ((assign104530_e156830) as f64).fract() == 0.0 { if assign104530_e156830 == 0.0 { 0.0 } else { (assign104530_e156830 * ((locals.var_dnm).powf(assign104530_e156830 - 1.0) * locals.var_dnm_dn8)) } } else { (assign104530_e156831 * (assign104530_e156830 * (locals.var_dnm_dn8 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign104530_e156830) as f64).is_finite() && ((assign104530_e156830) as f64).fract() == 0.0 { if assign104530_e156830 == 0.0 { 0.0 } else { (assign104530_e156830 * ((locals.var_dnm).powf(assign104530_e156830 - 1.0) * locals.var_dnm_dn9)) } } else { (assign104530_e156831 * (assign104530_e156830 * (locals.var_dnm_dn9 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign104530_e156830) as f64).is_finite() && ((assign104530_e156830) as f64).fract() == 0.0 { if assign104530_e156830 == 0.0 { 0.0 } else { (assign104530_e156830 * ((locals.var_dnm).powf(assign104530_e156830 - 1.0) * locals.var_dnm_dn10)) } } else { (assign104530_e156831 * (assign104530_e156830 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign104530_e156830) as f64).is_finite() && ((assign104530_e156830) as f64).fract() == 0.0 { if assign104530_e156830 == 0.0 { 0.0 } else { (assign104530_e156830 * ((locals.var_dnm).powf(assign104530_e156830 - 1.0) * locals.var_dnm_dn13)) } } else { (assign104530_e156831 * (assign104530_e156830 * (locals.var_dnm_dn13 / locals.var_dnm))) },)
            }
        };
        (assign104530_e156832, assign104530_e156832_d_n0, assign104530_e156832_d_n2, assign104530_e156832_d_n4, assign104530_e156832_d_n5, assign104530_e156832_d_n6, assign104530_e156832_d_n7, assign104530_e156832_d_n8, assign104530_e156832_d_n9, assign104530_e156832_d_n10, assign104530_e156832_d_n13,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign104530_e156834;
        locals.var_dnm_dn0 = assign104530_e156834_d_n0;
        locals.var_dnm_dn2 = assign104530_e156834_d_n2;
        locals.var_dnm_dn4 = assign104530_e156834_d_n4;
        locals.var_dnm_dn5 = assign104530_e156834_d_n5;
        locals.var_dnm_dn6 = assign104530_e156834_d_n6;
        locals.var_dnm_dn7 = assign104530_e156834_d_n7;
        locals.var_dnm_dn8 = assign104530_e156834_d_n8;
        locals.var_dnm_dn9 = assign104530_e156834_d_n9;
        locals.var_dnm_dn10 = assign104530_e156834_d_n10;
        locals.var_dnm_dn13 = assign104530_e156834_d_n13;

        let (assign104540_e156847, assign104540_e156847_d_n0, assign104540_e156847_d_n2, assign104540_e156847_d_n4, assign104540_e156847_d_n5, assign104540_e156847_d_n6, assign104540_e156847_d_n7, assign104540_e156847_d_n8, assign104540_e156847_d_n9, assign104540_e156847_d_n10, assign104540_e156847_d_n13,) = {
    if ((((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2371 != 0.0)) && (locals.var_guard2372 != 0.0)) {
        let assign104540_e156845: f64 = (1.0 / locals.var_dnm);
        (assign104540_e156845, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn4 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn5 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn8 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn9 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn13 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign104540_e156847;
        locals.var_dnm_dn0 = assign104540_e156847_d_n0;
        locals.var_dnm_dn2 = assign104540_e156847_d_n2;
        locals.var_dnm_dn4 = assign104540_e156847_d_n4;
        locals.var_dnm_dn5 = assign104540_e156847_d_n5;
        locals.var_dnm_dn6 = assign104540_e156847_d_n6;
        locals.var_dnm_dn7 = assign104540_e156847_d_n7;
        locals.var_dnm_dn8 = assign104540_e156847_d_n8;
        locals.var_dnm_dn9 = assign104540_e156847_d_n9;
        locals.var_dnm_dn10 = assign104540_e156847_d_n10;
        locals.var_dnm_dn13 = assign104540_e156847_d_n13;

        let (assign104550_e156864, assign104550_e156864_d_n0, assign104550_e156864_d_n2, assign104550_e156864_d_n4, assign104550_e156864_d_n5, assign104550_e156864_d_n6, assign104550_e156864_d_n7, assign104550_e156864_d_n8, assign104550_e156864_d_n9, assign104550_e156864_d_n10, assign104550_e156864_d_n13,) = {
    if ((((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2371 != 0.0)) && (locals.var_guard2372 != 0.0)) {
        let assign104550_e156859: f64 = (locals.var_noverd * p.p441);
        let assign104550_e156860: f64 = (locals.var_tmf1 * assign104550_e156859);
        let assign104550_e156862: f64 = (assign104550_e156860 * locals.var_dnm);
        (assign104550_e156862, (((locals.var_tmf1_dn0 * assign104550_e156859) * locals.var_dnm) + (assign104550_e156860 * locals.var_dnm_dn0)), (((locals.var_tmf1_dn2 * assign104550_e156859) * locals.var_dnm) + (assign104550_e156860 * locals.var_dnm_dn2)), (((locals.var_tmf1_dn4 * assign104550_e156859) * locals.var_dnm) + (assign104550_e156860 * locals.var_dnm_dn4)), (((locals.var_tmf1_dn5 * assign104550_e156859) * locals.var_dnm) + (assign104550_e156860 * locals.var_dnm_dn5)), (((locals.var_tmf1_dn6 * assign104550_e156859) * locals.var_dnm) + (assign104550_e156860 * locals.var_dnm_dn6)), (((locals.var_tmf1_dn7 * assign104550_e156859) * locals.var_dnm) + (assign104550_e156860 * locals.var_dnm_dn7)), (((locals.var_tmf1_dn8 * assign104550_e156859) * locals.var_dnm) + (assign104550_e156860 * locals.var_dnm_dn8)), (((locals.var_tmf1_dn9 * assign104550_e156859) * locals.var_dnm) + (assign104550_e156860 * locals.var_dnm_dn9)), (((locals.var_tmf1_dn10 * assign104550_e156859) * locals.var_dnm) + (assign104550_e156860 * locals.var_dnm_dn10)), (((locals.var_tmf1_dn13 * assign104550_e156859) * locals.var_dnm) + (assign104550_e156860 * locals.var_dnm_dn13)),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn13,)
    }
};
        locals.var_tmf0 = assign104550_e156864;
        locals.var_tmf0_dn0 = assign104550_e156864_d_n0;
        locals.var_tmf0_dn2 = assign104550_e156864_d_n2;
        locals.var_tmf0_dn4 = assign104550_e156864_d_n4;
        locals.var_tmf0_dn5 = assign104550_e156864_d_n5;
        locals.var_tmf0_dn6 = assign104550_e156864_d_n6;
        locals.var_tmf0_dn7 = assign104550_e156864_d_n7;
        locals.var_tmf0_dn8 = assign104550_e156864_d_n8;
        locals.var_tmf0_dn9 = assign104550_e156864_d_n9;
        locals.var_tmf0_dn10 = assign104550_e156864_d_n10;
        locals.var_tmf0_dn13 = assign104550_e156864_d_n13;

        let (assign104560_e156883, assign104560_e156883_d_n0, assign104560_e156883_d_n2, assign104560_e156883_d_n4, assign104560_e156883_d_n5, assign104560_e156883_d_n6, assign104560_e156883_d_n7, assign104560_e156883_d_n8, assign104560_e156883_d_n9, assign104560_e156883_d_n10, assign104560_e156883_d_n13,) = {
    if ((((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2371 != 0.0)) && (locals.var_guard2372 != 0.0)) {
        let assign104560_e156875: f64 = (locals.var_noverd * p.p441);
        let assign104560_e156877: f64 = (assign104560_e156875 * locals.var_xmp);
        let assign104560_e156879: f64 = (assign104560_e156877 * locals.var_dnm);
        let assign104560_e156881: f64 = (assign104560_e156879 / locals.var_arg);
        (assign104560_e156881, ((((((assign104560_e156875 * locals.var_xmp_dn0) * locals.var_dnm) + (assign104560_e156877 * locals.var_dnm_dn0)) * locals.var_arg) - (assign104560_e156879 * locals.var_arg_dn0)) / (locals.var_arg * locals.var_arg)), ((((((assign104560_e156875 * locals.var_xmp_dn2) * locals.var_dnm) + (assign104560_e156877 * locals.var_dnm_dn2)) * locals.var_arg) - (assign104560_e156879 * locals.var_arg_dn2)) / (locals.var_arg * locals.var_arg)), ((((((assign104560_e156875 * locals.var_xmp_dn4) * locals.var_dnm) + (assign104560_e156877 * locals.var_dnm_dn4)) * locals.var_arg) - (assign104560_e156879 * locals.var_arg_dn4)) / (locals.var_arg * locals.var_arg)), ((((((assign104560_e156875 * locals.var_xmp_dn5) * locals.var_dnm) + (assign104560_e156877 * locals.var_dnm_dn5)) * locals.var_arg) - (assign104560_e156879 * locals.var_arg_dn5)) / (locals.var_arg * locals.var_arg)), ((((((assign104560_e156875 * locals.var_xmp_dn6) * locals.var_dnm) + (assign104560_e156877 * locals.var_dnm_dn6)) * locals.var_arg) - (assign104560_e156879 * locals.var_arg_dn6)) / (locals.var_arg * locals.var_arg)), ((((((assign104560_e156875 * locals.var_xmp_dn7) * locals.var_dnm) + (assign104560_e156877 * locals.var_dnm_dn7)) * locals.var_arg) - (assign104560_e156879 * locals.var_arg_dn7)) / (locals.var_arg * locals.var_arg)), ((((((assign104560_e156875 * locals.var_xmp_dn8) * locals.var_dnm) + (assign104560_e156877 * locals.var_dnm_dn8)) * locals.var_arg) - (assign104560_e156879 * locals.var_arg_dn8)) / (locals.var_arg * locals.var_arg)), ((((((assign104560_e156875 * locals.var_xmp_dn9) * locals.var_dnm) + (assign104560_e156877 * locals.var_dnm_dn9)) * locals.var_arg) - (assign104560_e156879 * locals.var_arg_dn9)) / (locals.var_arg * locals.var_arg)), ((((((assign104560_e156875 * locals.var_xmp_dn10) * locals.var_dnm) + (assign104560_e156877 * locals.var_dnm_dn10)) * locals.var_arg) - (assign104560_e156879 * locals.var_arg_dn10)) / (locals.var_arg * locals.var_arg)), ((((((assign104560_e156875 * locals.var_xmp_dn13) * locals.var_dnm) + (assign104560_e156877 * locals.var_dnm_dn13)) * locals.var_arg) - (assign104560_e156879 * locals.var_arg_dn13)) / (locals.var_arg * locals.var_arg)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign104560_e156883;
        locals.var_t0_dn0 = assign104560_e156883_d_n0;
        locals.var_t0_dn2 = assign104560_e156883_d_n2;
        locals.var_t0_dn4 = assign104560_e156883_d_n4;
        locals.var_t0_dn5 = assign104560_e156883_d_n5;
        locals.var_t0_dn6 = assign104560_e156883_d_n6;
        locals.var_t0_dn7 = assign104560_e156883_d_n7;
        locals.var_t0_dn8 = assign104560_e156883_d_n8;
        locals.var_t0_dn9 = assign104560_e156883_d_n9;
        locals.var_t0_dn10 = assign104560_e156883_d_n10;
        locals.var_t0_dn13 = assign104560_e156883_d_n13;

        let (assign104570_e156902, assign104570_e156902_d_n0, assign104570_e156902_d_n2, assign104570_e156902_d_n4, assign104570_e156902_d_n5, assign104570_e156902_d_n6, assign104570_e156902_d_n7, assign104570_e156902_d_n8, assign104570_e156902_d_n9, assign104570_e156902_d_n10, assign104570_e156902_d_n13,) = {
    if ((((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2371 != 0.0)) && (locals.var_guard2372 != 0.0)) {
        let assign104570_e156894: f64 = (locals.var_noverd * p.p440);
        let assign104570_e156897: f64 = (locals.var_noverd * p.p441);
        let assign104570_e156898: f64 = (assign104570_e156894 - assign104570_e156897);
        let assign104570_e156900: f64 = (assign104570_e156898 + locals.var_tmf0);
        (assign104570_e156900, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn13,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign104570_e156902;
        locals.var_t2_dn0 = assign104570_e156902_d_n0;
        locals.var_t2_dn2 = assign104570_e156902_d_n2;
        locals.var_t2_dn4 = assign104570_e156902_d_n4;
        locals.var_t2_dn5 = assign104570_e156902_d_n5;
        locals.var_t2_dn6 = assign104570_e156902_d_n6;
        locals.var_t2_dn7 = assign104570_e156902_d_n7;
        locals.var_t2_dn8 = assign104570_e156902_d_n8;
        locals.var_t2_dn9 = assign104570_e156902_d_n9;
        locals.var_t2_dn10 = assign104570_e156902_d_n10;
        locals.var_t2_dn13 = assign104570_e156902_d_n13;

        let (assign104580_e156913, assign104580_e156913_d_n0, assign104580_e156913_d_n2, assign104580_e156913_d_n4, assign104580_e156913_d_n5, assign104580_e156913_d_n6, assign104580_e156913_d_n7, assign104580_e156913_d_n8, assign104580_e156913_d_n9, assign104580_e156913_d_n10, assign104580_e156913_d_n13,) = {
    if ((((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2371 != 0.0)) && (locals.var_guard2372 != 0.0)) {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign104580_e156913;
        locals.var_t0_dn0 = assign104580_e156913_d_n0;
        locals.var_t0_dn2 = assign104580_e156913_d_n2;
        locals.var_t0_dn4 = assign104580_e156913_d_n4;
        locals.var_t0_dn5 = assign104580_e156913_d_n5;
        locals.var_t0_dn6 = assign104580_e156913_d_n6;
        locals.var_t0_dn7 = assign104580_e156913_d_n7;
        locals.var_t0_dn8 = assign104580_e156913_d_n8;
        locals.var_t0_dn9 = assign104580_e156913_d_n9;
        locals.var_t0_dn10 = assign104580_e156913_d_n10;
        locals.var_t0_dn13 = assign104580_e156913_d_n13;

        let (assign104590_e156925, assign104590_e156925_d_n0, assign104590_e156925_d_n2, assign104590_e156925_d_n4, assign104590_e156925_d_n5, assign104590_e156925_d_n6, assign104590_e156925_d_n7, assign104590_e156925_d_n8, assign104590_e156925_d_n9, assign104590_e156925_d_n10, assign104590_e156925_d_n13,) = {
    if ((((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2371 != 0.0)) && (locals.var_guard2372 == 0.0)) {
        (locals.var_carr, locals.var_carr_dn0, locals.var_carr_dn2, locals.var_carr_dn4, locals.var_carr_dn5, locals.var_carr_dn6, locals.var_carr_dn7, locals.var_carr_dn8, locals.var_carr_dn9, locals.var_carr_dn10, locals.var_carr_dn13,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign104590_e156925;
        locals.var_t2_dn0 = assign104590_e156925_d_n0;
        locals.var_t2_dn2 = assign104590_e156925_d_n2;
        locals.var_t2_dn4 = assign104590_e156925_d_n4;
        locals.var_t2_dn5 = assign104590_e156925_d_n5;
        locals.var_t2_dn6 = assign104590_e156925_d_n6;
        locals.var_t2_dn7 = assign104590_e156925_d_n7;
        locals.var_t2_dn8 = assign104590_e156925_d_n8;
        locals.var_t2_dn9 = assign104590_e156925_d_n9;
        locals.var_t2_dn10 = assign104590_e156925_d_n10;
        locals.var_t2_dn13 = assign104590_e156925_d_n13;

        let (assign104600_e156937, assign104600_e156937_d_n0, assign104600_e156937_d_n2, assign104600_e156937_d_n4, assign104600_e156937_d_n5, assign104600_e156937_d_n6, assign104600_e156937_d_n7, assign104600_e156937_d_n8, assign104600_e156937_d_n9, assign104600_e156937_d_n10, assign104600_e156937_d_n13,) = {
    if ((((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2371 != 0.0)) && (locals.var_guard2372 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign104600_e156937;
        locals.var_t0_dn0 = assign104600_e156937_d_n0;
        locals.var_t0_dn2 = assign104600_e156937_d_n2;
        locals.var_t0_dn4 = assign104600_e156937_d_n4;
        locals.var_t0_dn5 = assign104600_e156937_d_n5;
        locals.var_t0_dn6 = assign104600_e156937_d_n6;
        locals.var_t0_dn7 = assign104600_e156937_d_n7;
        locals.var_t0_dn8 = assign104600_e156937_d_n8;
        locals.var_t0_dn9 = assign104600_e156937_d_n9;
        locals.var_t0_dn10 = assign104600_e156937_d_n10;
        locals.var_t0_dn13 = assign104600_e156937_d_n13;

        let (assign104610_e156946, assign104610_e156946_d_n0, assign104610_e156946_d_n2, assign104610_e156946_d_n4, assign104610_e156946_d_n5, assign104610_e156946_d_n6, assign104610_e156946_d_n7, assign104610_e156946_d_n8, assign104610_e156946_d_n9, assign104610_e156946_d_n10, assign104610_e156946_d_n13,) = {
    if (((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2371 != 0.0)) {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    } else {
        (locals.var_carr, locals.var_carr_dn0, locals.var_carr_dn2, locals.var_carr_dn4, locals.var_carr_dn5, locals.var_carr_dn6, locals.var_carr_dn7, locals.var_carr_dn8, locals.var_carr_dn9, locals.var_carr_dn10, locals.var_carr_dn13,)
    }
};
        locals.var_carr = assign104610_e156946;
        locals.var_carr_dn0 = assign104610_e156946_d_n0;
        locals.var_carr_dn2 = assign104610_e156946_d_n2;
        locals.var_carr_dn4 = assign104610_e156946_d_n4;
        locals.var_carr_dn5 = assign104610_e156946_d_n5;
        locals.var_carr_dn6 = assign104610_e156946_d_n6;
        locals.var_carr_dn7 = assign104610_e156946_d_n7;
        locals.var_carr_dn8 = assign104610_e156946_d_n8;
        locals.var_carr_dn9 = assign104610_e156946_d_n9;
        locals.var_carr_dn10 = assign104610_e156946_d_n10;
        locals.var_carr_dn13 = assign104610_e156946_d_n13;

        let (assign104620_e156954, assign104620_e156954_d_n0, assign104620_e156954_d_n2, assign104620_e156954_d_n4, assign104620_e156954_d_n5, assign104620_e156954_d_n6, assign104620_e156954_d_n7, assign104620_e156954_d_n8, assign104620_e156954_d_n9, assign104620_e156954_d_n10, assign104620_e156954_d_n13,) = {
    if ((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) {
        let assign104620_e156952: f64 = (-locals.var_rd_ps0ld);
        (assign104620_e156952, (-locals.var_rd_ps0ld_dn0), (-locals.var_rd_ps0ld_dn2), (-locals.var_rd_ps0ld_dn4), (-locals.var_rd_ps0ld_dn5), (-locals.var_rd_ps0ld_dn6), (-locals.var_rd_ps0ld_dn7), (-locals.var_rd_ps0ld_dn8), (-locals.var_rd_ps0ld_dn9), (-locals.var_rd_ps0ld_dn10), (-locals.var_rd_ps0ld_dn13),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign104620_e156954;
        locals.var_t0_dn0 = assign104620_e156954_d_n0;
        locals.var_t0_dn2 = assign104620_e156954_d_n2;
        locals.var_t0_dn4 = assign104620_e156954_d_n4;
        locals.var_t0_dn5 = assign104620_e156954_d_n5;
        locals.var_t0_dn6 = assign104620_e156954_d_n6;
        locals.var_t0_dn7 = assign104620_e156954_d_n7;
        locals.var_t0_dn8 = assign104620_e156954_d_n8;
        locals.var_t0_dn9 = assign104620_e156954_d_n9;
        locals.var_t0_dn10 = assign104620_e156954_d_n10;
        locals.var_t0_dn13 = assign104620_e156954_d_n13;

        let (assign104630_e156970, assign104630_e156970_d_n0, assign104630_e156970_d_n2, assign104630_e156970_d_n4, assign104630_e156970_d_n5, assign104630_e156970_d_n6, assign104630_e156970_d_n7, assign104630_e156970_d_n8, assign104630_e156970_d_n9, assign104630_e156970_d_n10, assign104630_e156970_d_n13,) = {
    if ((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) {
        let assign104630_e156961: f64 = (locals.var_t0 * locals.var_t0);
        let assign104630_e156964: f64 = (4.0 * 0.01);
        let assign104630_e156966: f64 = (assign104630_e156964 * 0.01);
        let assign104630_e156967: f64 = (assign104630_e156961 + assign104630_e156966);
        let assign104630_e156968: f64 = (assign104630_e156967).sqrt();
        (assign104630_e156968, (((locals.var_t0_dn0 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn0)) / (2.0 * assign104630_e156968)), (((locals.var_t0_dn2 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn2)) / (2.0 * assign104630_e156968)), (((locals.var_t0_dn4 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn4)) / (2.0 * assign104630_e156968)), (((locals.var_t0_dn5 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn5)) / (2.0 * assign104630_e156968)), (((locals.var_t0_dn6 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn6)) / (2.0 * assign104630_e156968)), (((locals.var_t0_dn7 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn7)) / (2.0 * assign104630_e156968)), (((locals.var_t0_dn8 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn8)) / (2.0 * assign104630_e156968)), (((locals.var_t0_dn9 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn9)) / (2.0 * assign104630_e156968)), (((locals.var_t0_dn10 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn10)) / (2.0 * assign104630_e156968)), (((locals.var_t0_dn13 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn13)) / (2.0 * assign104630_e156968)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign104630_e156970;
        locals.var_tmf2_dn0 = assign104630_e156970_d_n0;
        locals.var_tmf2_dn2 = assign104630_e156970_d_n2;
        locals.var_tmf2_dn4 = assign104630_e156970_d_n4;
        locals.var_tmf2_dn5 = assign104630_e156970_d_n5;
        locals.var_tmf2_dn6 = assign104630_e156970_d_n6;
        locals.var_tmf2_dn7 = assign104630_e156970_d_n7;
        locals.var_tmf2_dn8 = assign104630_e156970_d_n8;
        locals.var_tmf2_dn9 = assign104630_e156970_d_n9;
        locals.var_tmf2_dn10 = assign104630_e156970_d_n10;
        locals.var_tmf2_dn13 = assign104630_e156970_d_n13;

        let (assign104640_e156983, assign104640_e156983_d_n0, assign104640_e156983_d_n2, assign104640_e156983_d_n4, assign104640_e156983_d_n5, assign104640_e156983_d_n6, assign104640_e156983_d_n7, assign104640_e156983_d_n8, assign104640_e156983_d_n9, assign104640_e156983_d_n10, assign104640_e156983_d_n13,) = {
    if ((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) {
        let assign104640_e156979: f64 = (locals.var_t0 / locals.var_tmf2);
        let assign104640_e156980: f64 = (1.0 + assign104640_e156979);
        let assign104640_e156981: f64 = (0.5 * assign104640_e156980);
        (assign104640_e156981, (0.5 * (((locals.var_t0_dn0 * locals.var_tmf2) - (locals.var_t0 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t0_dn2 * locals.var_tmf2) - (locals.var_t0 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t0_dn4 * locals.var_tmf2) - (locals.var_t0 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t0_dn5 * locals.var_tmf2) - (locals.var_t0 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t0_dn6 * locals.var_tmf2) - (locals.var_t0 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t0_dn7 * locals.var_tmf2) - (locals.var_t0 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t0_dn8 * locals.var_tmf2) - (locals.var_t0 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t0_dn9 * locals.var_tmf2) - (locals.var_t0 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t0_dn10 * locals.var_tmf2) - (locals.var_t0 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t0_dn13 * locals.var_tmf2) - (locals.var_t0 * locals.var_tmf2_dn13)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn13,)
    }
};
        locals.var_t9 = assign104640_e156983;
        locals.var_t9_dn0 = assign104640_e156983_d_n0;
        locals.var_t9_dn2 = assign104640_e156983_d_n2;
        locals.var_t9_dn4 = assign104640_e156983_d_n4;
        locals.var_t9_dn5 = assign104640_e156983_d_n5;
        locals.var_t9_dn6 = assign104640_e156983_d_n6;
        locals.var_t9_dn7 = assign104640_e156983_d_n7;
        locals.var_t9_dn8 = assign104640_e156983_d_n8;
        locals.var_t9_dn9 = assign104640_e156983_d_n9;
        locals.var_t9_dn10 = assign104640_e156983_d_n10;
        locals.var_t9_dn13 = assign104640_e156983_d_n13;

        let (assign104650_e156994, assign104650_e156994_d_n0, assign104650_e156994_d_n2, assign104650_e156994_d_n4, assign104650_e156994_d_n5, assign104650_e156994_d_n6, assign104650_e156994_d_n7, assign104650_e156994_d_n8, assign104650_e156994_d_n9, assign104650_e156994_d_n10, assign104650_e156994_d_n13,) = {
    if ((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) {
        let assign104650_e156991: f64 = (locals.var_t0 + locals.var_tmf2);
        let assign104650_e156992: f64 = (0.5 * assign104650_e156991);
        (assign104650_e156992, (0.5 * (locals.var_t0_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_t0_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_t0_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_t0_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_t0_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_t0_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_t0_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_t0_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_t0_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_t0_dn13 + locals.var_tmf2_dn13)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign104650_e156994;
        locals.var_t0_dn0 = assign104650_e156994_d_n0;
        locals.var_t0_dn2 = assign104650_e156994_d_n2;
        locals.var_t0_dn4 = assign104650_e156994_d_n4;
        locals.var_t0_dn5 = assign104650_e156994_d_n5;
        locals.var_t0_dn6 = assign104650_e156994_d_n6;
        locals.var_t0_dn7 = assign104650_e156994_d_n7;
        locals.var_t0_dn8 = assign104650_e156994_d_n8;
        locals.var_t0_dn9 = assign104650_e156994_d_n9;
        locals.var_t0_dn10 = assign104650_e156994_d_n10;
        locals.var_t0_dn13 = assign104650_e156994_d_n13;

        let assign104660_e156997: f64 = if locals.var_t0 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2378 = assign104660_e156997;

        let (assign104670_e157006, assign104670_e157006_d_n0, assign104670_e157006_d_n2, assign104670_e157006_d_n4, assign104670_e157006_d_n5, assign104670_e157006_d_n6, assign104670_e157006_d_n7, assign104670_e157006_d_n8, assign104670_e157006_d_n9, assign104670_e157006_d_n10, assign104670_e157006_d_n13,) = {
    if (((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2378 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign104670_e157006;
        locals.var_t0_dn0 = assign104670_e157006_d_n0;
        locals.var_t0_dn2 = assign104670_e157006_d_n2;
        locals.var_t0_dn4 = assign104670_e157006_d_n4;
        locals.var_t0_dn5 = assign104670_e157006_d_n5;
        locals.var_t0_dn6 = assign104670_e157006_d_n6;
        locals.var_t0_dn7 = assign104670_e157006_d_n7;
        locals.var_t0_dn8 = assign104670_e157006_d_n8;
        locals.var_t0_dn9 = assign104670_e157006_d_n9;
        locals.var_t0_dn10 = assign104670_e157006_d_n10;
        locals.var_t0_dn13 = assign104670_e157006_d_n13;

    }

    pub(super) fn stamp_transient_block_369(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign104680_e157015, assign104680_e157015_d_n0, assign104680_e157015_d_n2, assign104680_e157015_d_n4, assign104680_e157015_d_n5, assign104680_e157015_d_n6, assign104680_e157015_d_n7, assign104680_e157015_d_n8, assign104680_e157015_d_n9, assign104680_e157015_d_n10, assign104680_e157015_d_n13,) = {
    if (((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2378 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn13,)
    }
};
        locals.var_t9 = assign104680_e157015;
        locals.var_t9_dn0 = assign104680_e157015_d_n0;
        locals.var_t9_dn2 = assign104680_e157015_d_n2;
        locals.var_t9_dn4 = assign104680_e157015_d_n4;
        locals.var_t9_dn5 = assign104680_e157015_d_n5;
        locals.var_t9_dn6 = assign104680_e157015_d_n6;
        locals.var_t9_dn7 = assign104680_e157015_d_n7;
        locals.var_t9_dn8 = assign104680_e157015_d_n8;
        locals.var_t9_dn9 = assign104680_e157015_d_n9;
        locals.var_t9_dn10 = assign104680_e157015_d_n10;
        locals.var_t9_dn13 = assign104680_e157015_d_n13;

        let (assign104690_e157026, assign104690_e157026_d_n0, assign104690_e157026_d_n2, assign104690_e157026_d_n4, assign104690_e157026_d_n5, assign104690_e157026_d_n6, assign104690_e157026_d_n7, assign104690_e157026_d_n8, assign104690_e157026_d_n9, assign104690_e157026_d_n10, assign104690_e157026_d_n13,) = {
    if ((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) {
        let assign104690_e157023: f64 = (10.0 * 2.220446049250313e-16);
        let assign104690_e157024: f64 = (locals.var_t0 + assign104690_e157023);
        (assign104690_e157024, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign104690_e157026;
        locals.var_t0_dn0 = assign104690_e157026_d_n0;
        locals.var_t0_dn2 = assign104690_e157026_d_n2;
        locals.var_t0_dn4 = assign104690_e157026_d_n4;
        locals.var_t0_dn5 = assign104690_e157026_d_n5;
        locals.var_t0_dn6 = assign104690_e157026_d_n6;
        locals.var_t0_dn7 = assign104690_e157026_d_n7;
        locals.var_t0_dn8 = assign104690_e157026_d_n8;
        locals.var_t0_dn9 = assign104690_e157026_d_n9;
        locals.var_t0_dn10 = assign104690_e157026_d_n10;
        locals.var_t0_dn13 = assign104690_e157026_d_n13;

        let (assign104700_e157036, assign104700_e157036_d_n0, assign104700_e157036_d_n2, assign104700_e157036_d_n4, assign104700_e157036_d_n5, assign104700_e157036_d_n6, assign104700_e157036_d_n7, assign104700_e157036_d_n8, assign104700_e157036_d_n9, assign104700_e157036_d_n10, assign104700_e157036_d_n13,) = {
    if ((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) {
        let assign104700_e157033: f64 = (locals.var_kdep * locals.var_t0);
        let assign104700_e157034: f64 = (assign104700_e157033).sqrt();
        (assign104700_e157034, ((locals.var_kdep * locals.var_t0_dn0) / (2.0 * assign104700_e157034)), ((locals.var_kdep * locals.var_t0_dn2) / (2.0 * assign104700_e157034)), ((locals.var_kdep * locals.var_t0_dn4) / (2.0 * assign104700_e157034)), ((locals.var_kdep * locals.var_t0_dn5) / (2.0 * assign104700_e157034)), ((locals.var_kdep * locals.var_t0_dn6) / (2.0 * assign104700_e157034)), ((locals.var_kdep * locals.var_t0_dn7) / (2.0 * assign104700_e157034)), ((locals.var_kdep * locals.var_t0_dn8) / (2.0 * assign104700_e157034)), ((locals.var_kdep * locals.var_t0_dn9) / (2.0 * assign104700_e157034)), ((locals.var_kdep * locals.var_t0_dn10) / (2.0 * assign104700_e157034)), ((locals.var_kdep * locals.var_t0_dn13) / (2.0 * assign104700_e157034)),)
    } else {
        (locals.var_wdepl, locals.var_wdepl_dn0, locals.var_wdepl_dn2, locals.var_wdepl_dn4, locals.var_wdepl_dn5, locals.var_wdepl_dn6, locals.var_wdepl_dn7, locals.var_wdepl_dn8, locals.var_wdepl_dn9, locals.var_wdepl_dn10, locals.var_wdepl_dn13,)
    }
};
        locals.var_wdepl = assign104700_e157036;
        locals.var_wdepl_dn0 = assign104700_e157036_d_n0;
        locals.var_wdepl_dn2 = assign104700_e157036_d_n2;
        locals.var_wdepl_dn4 = assign104700_e157036_d_n4;
        locals.var_wdepl_dn5 = assign104700_e157036_d_n5;
        locals.var_wdepl_dn6 = assign104700_e157036_d_n6;
        locals.var_wdepl_dn7 = assign104700_e157036_d_n7;
        locals.var_wdepl_dn8 = assign104700_e157036_d_n8;
        locals.var_wdepl_dn9 = assign104700_e157036_d_n9;
        locals.var_wdepl_dn10 = assign104700_e157036_d_n10;
        locals.var_wdepl_dn13 = assign104700_e157036_d_n13;

        let (assign104710_e157047, assign104710_e157047_d_n0, assign104710_e157047_d_n2, assign104710_e157047_d_n4, assign104710_e157047_d_n5, assign104710_e157047_d_n6, assign104710_e157047_d_n7, assign104710_e157047_d_n8, assign104710_e157047_d_n9, assign104710_e157047_d_n10, assign104710_e157047_d_n13,) = {
    if ((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) {
        let assign104710_e157043: f64 = (locals.var_vds__blk2352 - locals.var_vbs__blk2353);
        let assign104710_e157045: f64 = (assign104710_e157043 + p.p137);
        (assign104710_e157045, 0.0, 0.0, 0.0, locals.var_vds__blk2352_dn5, 0.0, (locals.var_vds__blk2352_dn7 - locals.var_vbs__blk2353_dn7), (-locals.var_vbs__blk2353_dn8), 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign104710_e157047;
        locals.var_t2_dn0 = assign104710_e157047_d_n0;
        locals.var_t2_dn2 = assign104710_e157047_d_n2;
        locals.var_t2_dn4 = assign104710_e157047_d_n4;
        locals.var_t2_dn5 = assign104710_e157047_d_n5;
        locals.var_t2_dn6 = assign104710_e157047_d_n6;
        locals.var_t2_dn7 = assign104710_e157047_d_n7;
        locals.var_t2_dn8 = assign104710_e157047_d_n8;
        locals.var_t2_dn9 = assign104710_e157047_d_n9;
        locals.var_t2_dn10 = assign104710_e157047_d_n10;
        locals.var_t2_dn13 = assign104710_e157047_d_n13;

        let (assign104720_e157063, assign104720_e157063_d_n0, assign104720_e157063_d_n2, assign104720_e157063_d_n4, assign104720_e157063_d_n5, assign104720_e157063_d_n6, assign104720_e157063_d_n7, assign104720_e157063_d_n8, assign104720_e157063_d_n9, assign104720_e157063_d_n10, assign104720_e157063_d_n13,) = {
    if ((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) {
        let assign104720_e157054: f64 = (locals.var_t2 * locals.var_t2);
        let assign104720_e157057: f64 = (4.0 * 0.01);
        let assign104720_e157059: f64 = (assign104720_e157057 * 0.01);
        let assign104720_e157060: f64 = (assign104720_e157054 + assign104720_e157059);
        let assign104720_e157061: f64 = (assign104720_e157060).sqrt();
        (assign104720_e157061, (((locals.var_t2_dn0 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn0)) / (2.0 * assign104720_e157061)), (((locals.var_t2_dn2 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn2)) / (2.0 * assign104720_e157061)), (((locals.var_t2_dn4 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn4)) / (2.0 * assign104720_e157061)), (((locals.var_t2_dn5 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn5)) / (2.0 * assign104720_e157061)), (((locals.var_t2_dn6 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn6)) / (2.0 * assign104720_e157061)), (((locals.var_t2_dn7 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn7)) / (2.0 * assign104720_e157061)), (((locals.var_t2_dn8 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn8)) / (2.0 * assign104720_e157061)), (((locals.var_t2_dn9 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn9)) / (2.0 * assign104720_e157061)), (((locals.var_t2_dn10 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn10)) / (2.0 * assign104720_e157061)), (((locals.var_t2_dn13 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn13)) / (2.0 * assign104720_e157061)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign104720_e157063;
        locals.var_tmf2_dn0 = assign104720_e157063_d_n0;
        locals.var_tmf2_dn2 = assign104720_e157063_d_n2;
        locals.var_tmf2_dn4 = assign104720_e157063_d_n4;
        locals.var_tmf2_dn5 = assign104720_e157063_d_n5;
        locals.var_tmf2_dn6 = assign104720_e157063_d_n6;
        locals.var_tmf2_dn7 = assign104720_e157063_d_n7;
        locals.var_tmf2_dn8 = assign104720_e157063_d_n8;
        locals.var_tmf2_dn9 = assign104720_e157063_d_n9;
        locals.var_tmf2_dn10 = assign104720_e157063_d_n10;
        locals.var_tmf2_dn13 = assign104720_e157063_d_n13;

        let (assign104730_e157076, assign104730_e157076_d_n0, assign104730_e157076_d_n2, assign104730_e157076_d_n4, assign104730_e157076_d_n5, assign104730_e157076_d_n6, assign104730_e157076_d_n7, assign104730_e157076_d_n8, assign104730_e157076_d_n9, assign104730_e157076_d_n10, assign104730_e157076_d_n13,) = {
    if ((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) {
        let assign104730_e157072: f64 = (locals.var_t2 / locals.var_tmf2);
        let assign104730_e157073: f64 = (1.0 + assign104730_e157072);
        let assign104730_e157074: f64 = (0.5 * assign104730_e157073);
        (assign104730_e157074, (0.5 * (((locals.var_t2_dn0 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn2 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn4 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn5 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn6 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn7 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn8 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn9 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn10 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn13 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn13)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn13,)
    }
};
        locals.var_t9 = assign104730_e157076;
        locals.var_t9_dn0 = assign104730_e157076_d_n0;
        locals.var_t9_dn2 = assign104730_e157076_d_n2;
        locals.var_t9_dn4 = assign104730_e157076_d_n4;
        locals.var_t9_dn5 = assign104730_e157076_d_n5;
        locals.var_t9_dn6 = assign104730_e157076_d_n6;
        locals.var_t9_dn7 = assign104730_e157076_d_n7;
        locals.var_t9_dn8 = assign104730_e157076_d_n8;
        locals.var_t9_dn9 = assign104730_e157076_d_n9;
        locals.var_t9_dn10 = assign104730_e157076_d_n10;
        locals.var_t9_dn13 = assign104730_e157076_d_n13;

        let (assign104740_e157087, assign104740_e157087_d_n0, assign104740_e157087_d_n2, assign104740_e157087_d_n4, assign104740_e157087_d_n5, assign104740_e157087_d_n6, assign104740_e157087_d_n7, assign104740_e157087_d_n8, assign104740_e157087_d_n9, assign104740_e157087_d_n10, assign104740_e157087_d_n13,) = {
    if ((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) {
        let assign104740_e157084: f64 = (locals.var_t2 + locals.var_tmf2);
        let assign104740_e157085: f64 = (0.5 * assign104740_e157084);
        (assign104740_e157085, (0.5 * (locals.var_t2_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_t2_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_t2_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_t2_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_t2_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_t2_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_t2_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_t2_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_t2_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_t2_dn13 + locals.var_tmf2_dn13)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign104740_e157087;
        locals.var_t2_dn0 = assign104740_e157087_d_n0;
        locals.var_t2_dn2 = assign104740_e157087_d_n2;
        locals.var_t2_dn4 = assign104740_e157087_d_n4;
        locals.var_t2_dn5 = assign104740_e157087_d_n5;
        locals.var_t2_dn6 = assign104740_e157087_d_n6;
        locals.var_t2_dn7 = assign104740_e157087_d_n7;
        locals.var_t2_dn8 = assign104740_e157087_d_n8;
        locals.var_t2_dn9 = assign104740_e157087_d_n9;
        locals.var_t2_dn10 = assign104740_e157087_d_n10;
        locals.var_t2_dn13 = assign104740_e157087_d_n13;

        let assign104750_e157090: f64 = if locals.var_t2 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2379 = assign104750_e157090;

        let (assign104760_e157099, assign104760_e157099_d_n0, assign104760_e157099_d_n2, assign104760_e157099_d_n4, assign104760_e157099_d_n5, assign104760_e157099_d_n6, assign104760_e157099_d_n7, assign104760_e157099_d_n8, assign104760_e157099_d_n9, assign104760_e157099_d_n10, assign104760_e157099_d_n13,) = {
    if (((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2379 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign104760_e157099;
        locals.var_t2_dn0 = assign104760_e157099_d_n0;
        locals.var_t2_dn2 = assign104760_e157099_d_n2;
        locals.var_t2_dn4 = assign104760_e157099_d_n4;
        locals.var_t2_dn5 = assign104760_e157099_d_n5;
        locals.var_t2_dn6 = assign104760_e157099_d_n6;
        locals.var_t2_dn7 = assign104760_e157099_d_n7;
        locals.var_t2_dn8 = assign104760_e157099_d_n8;
        locals.var_t2_dn9 = assign104760_e157099_d_n9;
        locals.var_t2_dn10 = assign104760_e157099_d_n10;
        locals.var_t2_dn13 = assign104760_e157099_d_n13;

        let (assign104770_e157108, assign104770_e157108_d_n0, assign104770_e157108_d_n2, assign104770_e157108_d_n4, assign104770_e157108_d_n5, assign104770_e157108_d_n6, assign104770_e157108_d_n7, assign104770_e157108_d_n8, assign104770_e157108_d_n9, assign104770_e157108_d_n10, assign104770_e157108_d_n13,) = {
    if (((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2379 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn13,)
    }
};
        locals.var_t9 = assign104770_e157108;
        locals.var_t9_dn0 = assign104770_e157108_d_n0;
        locals.var_t9_dn2 = assign104770_e157108_d_n2;
        locals.var_t9_dn4 = assign104770_e157108_d_n4;
        locals.var_t9_dn5 = assign104770_e157108_d_n5;
        locals.var_t9_dn6 = assign104770_e157108_d_n6;
        locals.var_t9_dn7 = assign104770_e157108_d_n7;
        locals.var_t9_dn8 = assign104770_e157108_d_n8;
        locals.var_t9_dn9 = assign104770_e157108_d_n9;
        locals.var_t9_dn10 = assign104770_e157108_d_n10;
        locals.var_t9_dn13 = assign104770_e157108_d_n13;

        let (assign104780_e157119, assign104780_e157119_d_n0, assign104780_e157119_d_n2, assign104780_e157119_d_n4, assign104780_e157119_d_n5, assign104780_e157119_d_n6, assign104780_e157119_d_n7, assign104780_e157119_d_n8, assign104780_e157119_d_n9, assign104780_e157119_d_n10, assign104780_e157119_d_n13,) = {
    if ((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) {
        let assign104780_e157116: f64 = (10.0 * 2.220446049250313e-16);
        let assign104780_e157117: f64 = (locals.var_t2 + assign104780_e157116);
        (assign104780_e157117, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign104780_e157119;
        locals.var_t2_dn0 = assign104780_e157119_d_n0;
        locals.var_t2_dn2 = assign104780_e157119_d_n2;
        locals.var_t2_dn4 = assign104780_e157119_d_n4;
        locals.var_t2_dn5 = assign104780_e157119_d_n5;
        locals.var_t2_dn6 = assign104780_e157119_d_n6;
        locals.var_t2_dn7 = assign104780_e157119_d_n7;
        locals.var_t2_dn8 = assign104780_e157119_d_n8;
        locals.var_t2_dn9 = assign104780_e157119_d_n9;
        locals.var_t2_dn10 = assign104780_e157119_d_n10;
        locals.var_t2_dn13 = assign104780_e157119_d_n13;

        let (assign104790_e157129, assign104790_e157129_d_n0, assign104790_e157129_d_n2, assign104790_e157129_d_n4, assign104790_e157129_d_n5, assign104790_e157129_d_n6, assign104790_e157129_d_n7, assign104790_e157129_d_n8, assign104790_e157129_d_n9, assign104790_e157129_d_n10, assign104790_e157129_d_n13,) = {
    if ((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) {
        let assign104790_e157126: f64 = (locals.var_kjunc * locals.var_t2);
        let assign104790_e157127: f64 = (assign104790_e157126).sqrt();
        (assign104790_e157127, (((locals.var_kjunc_dn0 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn0)) / (2.0 * assign104790_e157127)), (((locals.var_kjunc_dn2 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn2)) / (2.0 * assign104790_e157127)), (((locals.var_kjunc_dn4 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn4)) / (2.0 * assign104790_e157127)), (((locals.var_kjunc_dn5 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn5)) / (2.0 * assign104790_e157127)), (((locals.var_kjunc_dn6 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn6)) / (2.0 * assign104790_e157127)), (((locals.var_kjunc_dn7 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn7)) / (2.0 * assign104790_e157127)), (((locals.var_kjunc_dn8 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn8)) / (2.0 * assign104790_e157127)), (((locals.var_kjunc_dn9 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn9)) / (2.0 * assign104790_e157127)), (((locals.var_kjunc_dn10 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn10)) / (2.0 * assign104790_e157127)), (((locals.var_kjunc_dn13 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn13)) / (2.0 * assign104790_e157127)),)
    } else {
        (locals.var_wjunc0, locals.var_wjunc0_dn0, locals.var_wjunc0_dn2, locals.var_wjunc0_dn4, locals.var_wjunc0_dn5, locals.var_wjunc0_dn6, locals.var_wjunc0_dn7, locals.var_wjunc0_dn8, locals.var_wjunc0_dn9, locals.var_wjunc0_dn10, locals.var_wjunc0_dn13,)
    }
};
        locals.var_wjunc0 = assign104790_e157129;
        locals.var_wjunc0_dn0 = assign104790_e157129_d_n0;
        locals.var_wjunc0_dn2 = assign104790_e157129_d_n2;
        locals.var_wjunc0_dn4 = assign104790_e157129_d_n4;
        locals.var_wjunc0_dn5 = assign104790_e157129_d_n5;
        locals.var_wjunc0_dn6 = assign104790_e157129_d_n6;
        locals.var_wjunc0_dn7 = assign104790_e157129_d_n7;
        locals.var_wjunc0_dn8 = assign104790_e157129_d_n8;
        locals.var_wjunc0_dn9 = assign104790_e157129_d_n9;
        locals.var_wjunc0_dn10 = assign104790_e157129_d_n10;
        locals.var_wjunc0_dn13 = assign104790_e157129_d_n13;

        let (assign104800_e157142, assign104800_e157142_d_n0, assign104800_e157142_d_n2, assign104800_e157142_d_n4, assign104800_e157142_d_n5, assign104800_e157142_d_n6, assign104800_e157142_d_n7, assign104800_e157142_d_n8, assign104800_e157142_d_n9, assign104800_e157142_d_n10, assign104800_e157142_d_n13,) = {
    if ((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) {
        let assign104800_e157136: f64 = (locals.var_rd_xldld - locals.var_wjunc0);
        let assign104800_e157139: f64 = (0.01 * locals.var_rd_xldld);
        let assign104800_e157140: f64 = (assign104800_e157136 - assign104800_e157139);
        (assign104800_e157140, (-locals.var_wjunc0_dn0), (-locals.var_wjunc0_dn2), (-locals.var_wjunc0_dn4), (-locals.var_wjunc0_dn5), (-locals.var_wjunc0_dn6), (-locals.var_wjunc0_dn7), (-locals.var_wjunc0_dn8), (-locals.var_wjunc0_dn9), (-locals.var_wjunc0_dn10), (-locals.var_wjunc0_dn13),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13,)
    }
};
        locals.var_tmf1 = assign104800_e157142;
        locals.var_tmf1_dn0 = assign104800_e157142_d_n0;
        locals.var_tmf1_dn2 = assign104800_e157142_d_n2;
        locals.var_tmf1_dn4 = assign104800_e157142_d_n4;
        locals.var_tmf1_dn5 = assign104800_e157142_d_n5;
        locals.var_tmf1_dn6 = assign104800_e157142_d_n6;
        locals.var_tmf1_dn7 = assign104800_e157142_d_n7;
        locals.var_tmf1_dn8 = assign104800_e157142_d_n8;
        locals.var_tmf1_dn9 = assign104800_e157142_d_n9;
        locals.var_tmf1_dn10 = assign104800_e157142_d_n10;
        locals.var_tmf1_dn13 = assign104800_e157142_d_n13;

        let (assign104810_e157155, assign104810_e157155_d_n0, assign104810_e157155_d_n2, assign104810_e157155_d_n4, assign104810_e157155_d_n5, assign104810_e157155_d_n6, assign104810_e157155_d_n7, assign104810_e157155_d_n8, assign104810_e157155_d_n9, assign104810_e157155_d_n10, assign104810_e157155_d_n13,) = {
    if ((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) {
        let assign104810_e157149: f64 = (4.0 * locals.var_rd_xldld);
        let assign104810_e157152: f64 = (0.01 * locals.var_rd_xldld);
        let assign104810_e157153: f64 = (assign104810_e157149 * assign104810_e157152);
        (assign104810_e157153, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign104810_e157155;
        locals.var_tmf2_dn0 = assign104810_e157155_d_n0;
        locals.var_tmf2_dn2 = assign104810_e157155_d_n2;
        locals.var_tmf2_dn4 = assign104810_e157155_d_n4;
        locals.var_tmf2_dn5 = assign104810_e157155_d_n5;
        locals.var_tmf2_dn6 = assign104810_e157155_d_n6;
        locals.var_tmf2_dn7 = assign104810_e157155_d_n7;
        locals.var_tmf2_dn8 = assign104810_e157155_d_n8;
        locals.var_tmf2_dn9 = assign104810_e157155_d_n9;
        locals.var_tmf2_dn10 = assign104810_e157155_d_n10;
        locals.var_tmf2_dn13 = assign104810_e157155_d_n13;

        let (assign104820_e157168, assign104820_e157168_d_n0, assign104820_e157168_d_n2, assign104820_e157168_d_n4, assign104820_e157168_d_n5, assign104820_e157168_d_n6, assign104820_e157168_d_n7, assign104820_e157168_d_n8, assign104820_e157168_d_n9, assign104820_e157168_d_n10, assign104820_e157168_d_n13,) = {
    if ((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) {
        let (assign104820_e157166, assign104820_e157166_d_n0, assign104820_e157166_d_n2, assign104820_e157166_d_n4, assign104820_e157166_d_n5, assign104820_e157166_d_n6, assign104820_e157166_d_n7, assign104820_e157166_d_n8, assign104820_e157166_d_n9, assign104820_e157166_d_n10, assign104820_e157166_d_n13,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
            } else {
                let assign104820_e157165: f64 = (-locals.var_tmf2);
                (assign104820_e157165, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn13),)
            }
        };
        (assign104820_e157166, assign104820_e157166_d_n0, assign104820_e157166_d_n2, assign104820_e157166_d_n4, assign104820_e157166_d_n5, assign104820_e157166_d_n6, assign104820_e157166_d_n7, assign104820_e157166_d_n8, assign104820_e157166_d_n9, assign104820_e157166_d_n10, assign104820_e157166_d_n13,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign104820_e157168;
        locals.var_tmf2_dn0 = assign104820_e157168_d_n0;
        locals.var_tmf2_dn2 = assign104820_e157168_d_n2;
        locals.var_tmf2_dn4 = assign104820_e157168_d_n4;
        locals.var_tmf2_dn5 = assign104820_e157168_d_n5;
        locals.var_tmf2_dn6 = assign104820_e157168_d_n6;
        locals.var_tmf2_dn7 = assign104820_e157168_d_n7;
        locals.var_tmf2_dn8 = assign104820_e157168_d_n8;
        locals.var_tmf2_dn9 = assign104820_e157168_d_n9;
        locals.var_tmf2_dn10 = assign104820_e157168_d_n10;
        locals.var_tmf2_dn13 = assign104820_e157168_d_n13;

        let (assign104830_e157180, assign104830_e157180_d_n0, assign104830_e157180_d_n2, assign104830_e157180_d_n4, assign104830_e157180_d_n5, assign104830_e157180_d_n6, assign104830_e157180_d_n7, assign104830_e157180_d_n8, assign104830_e157180_d_n9, assign104830_e157180_d_n10, assign104830_e157180_d_n13,) = {
    if ((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) {
        let assign104830_e157175: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign104830_e157177: f64 = (assign104830_e157175 + locals.var_tmf2);
        let assign104830_e157178: f64 = (assign104830_e157177).sqrt();
        (assign104830_e157178, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign104830_e157178)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign104830_e157178)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign104830_e157178)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign104830_e157178)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign104830_e157178)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign104830_e157178)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign104830_e157178)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign104830_e157178)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign104830_e157178)), ((((locals.var_tmf1_dn13 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn13)) + locals.var_tmf2_dn13) / (2.0 * assign104830_e157178)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign104830_e157180;
        locals.var_tmf2_dn0 = assign104830_e157180_d_n0;
        locals.var_tmf2_dn2 = assign104830_e157180_d_n2;
        locals.var_tmf2_dn4 = assign104830_e157180_d_n4;
        locals.var_tmf2_dn5 = assign104830_e157180_d_n5;
        locals.var_tmf2_dn6 = assign104830_e157180_d_n6;
        locals.var_tmf2_dn7 = assign104830_e157180_d_n7;
        locals.var_tmf2_dn8 = assign104830_e157180_d_n8;
        locals.var_tmf2_dn9 = assign104830_e157180_d_n9;
        locals.var_tmf2_dn10 = assign104830_e157180_d_n10;
        locals.var_tmf2_dn13 = assign104830_e157180_d_n13;

        let (assign104840_e157193, assign104840_e157193_d_n0, assign104840_e157193_d_n2, assign104840_e157193_d_n4, assign104840_e157193_d_n5, assign104840_e157193_d_n6, assign104840_e157193_d_n7, assign104840_e157193_d_n8, assign104840_e157193_d_n9, assign104840_e157193_d_n10, assign104840_e157193_d_n13,) = {
    if ((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) {
        let assign104840_e157189: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign104840_e157190: f64 = (1.0 + assign104840_e157189);
        let assign104840_e157191: f64 = (0.5 * assign104840_e157190);
        (assign104840_e157191, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn13 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn13)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign104840_e157193;
        locals.var_t0_dn0 = assign104840_e157193_d_n0;
        locals.var_t0_dn2 = assign104840_e157193_d_n2;
        locals.var_t0_dn4 = assign104840_e157193_d_n4;
        locals.var_t0_dn5 = assign104840_e157193_d_n5;
        locals.var_t0_dn6 = assign104840_e157193_d_n6;
        locals.var_t0_dn7 = assign104840_e157193_d_n7;
        locals.var_t0_dn8 = assign104840_e157193_d_n8;
        locals.var_t0_dn9 = assign104840_e157193_d_n9;
        locals.var_t0_dn10 = assign104840_e157193_d_n10;
        locals.var_t0_dn13 = assign104840_e157193_d_n13;

        let (assign104850_e157206, assign104850_e157206_d_n0, assign104850_e157206_d_n2, assign104850_e157206_d_n4, assign104850_e157206_d_n5, assign104850_e157206_d_n6, assign104850_e157206_d_n7, assign104850_e157206_d_n8, assign104850_e157206_d_n9, assign104850_e157206_d_n10, assign104850_e157206_d_n13,) = {
    if ((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) {
        let assign104850_e157202: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign104850_e157203: f64 = (0.5 * assign104850_e157202);
        let assign104850_e157204: f64 = (locals.var_rd_xldld - assign104850_e157203);
        (assign104850_e157204, (-(0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (-(0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), (-(0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4))), (-(0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5))), (-(0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), (-(0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))), (-(0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8))), (-(0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9))), (-(0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), (-(0.5 * (locals.var_tmf1_dn13 + locals.var_tmf2_dn13))),)
    } else {
        (locals.var_wjunc, locals.var_wjunc_dn0, locals.var_wjunc_dn2, locals.var_wjunc_dn4, locals.var_wjunc_dn5, locals.var_wjunc_dn6, locals.var_wjunc_dn7, locals.var_wjunc_dn8, locals.var_wjunc_dn9, locals.var_wjunc_dn10, locals.var_wjunc_dn13,)
    }
};
        locals.var_wjunc = assign104850_e157206;
        locals.var_wjunc_dn0 = assign104850_e157206_d_n0;
        locals.var_wjunc_dn2 = assign104850_e157206_d_n2;
        locals.var_wjunc_dn4 = assign104850_e157206_d_n4;
        locals.var_wjunc_dn5 = assign104850_e157206_d_n5;
        locals.var_wjunc_dn6 = assign104850_e157206_d_n6;
        locals.var_wjunc_dn7 = assign104850_e157206_d_n7;
        locals.var_wjunc_dn8 = assign104850_e157206_d_n8;
        locals.var_wjunc_dn9 = assign104850_e157206_d_n9;
        locals.var_wjunc_dn10 = assign104850_e157206_d_n10;
        locals.var_wjunc_dn13 = assign104850_e157206_d_n13;

        let (assign104860_e157215,) = {
    if ((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) {
        let assign104860_e157213: f64 = (p.p419 + 1e-25);
        (assign104860_e157213,)
    } else {
        (locals.var_wrdrdjunc,)
    }
};
        locals.var_wrdrdjunc = assign104860_e157215;

        let (assign104870_e157234, assign104870_e157234_d_n0, assign104870_e157234_d_n2, assign104870_e157234_d_n4, assign104870_e157234_d_n5, assign104870_e157234_d_n6, assign104870_e157234_d_n7, assign104870_e157234_d_n8, assign104870_e157234_d_n9, assign104870_e157234_d_n10, assign104870_e157234_d_n13,) = {
    if ((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) {
        let assign104870_e157225: f64 = (locals.var_wdepl / locals.var_wrdrdjunc);
        let assign104870_e157228: f64 = (locals.var_wjunc / locals.var_rd_xldld);
        let assign104870_e157229: f64 = (assign104870_e157225 + assign104870_e157228);
        let assign104870_e157230: f64 = (locals.var_cx * assign104870_e157229);
        let assign104870_e157231: f64 = (1.0 - assign104870_e157230);
        let assign104870_e157232: f64 = (locals.var_xmax * assign104870_e157231);
        (assign104870_e157232, (locals.var_xmax * (-(locals.var_cx * ((locals.var_wdepl_dn0 / locals.var_wrdrdjunc) + (locals.var_wjunc_dn0 / locals.var_rd_xldld))))), (locals.var_xmax * (-(locals.var_cx * ((locals.var_wdepl_dn2 / locals.var_wrdrdjunc) + (locals.var_wjunc_dn2 / locals.var_rd_xldld))))), (locals.var_xmax * (-(locals.var_cx * ((locals.var_wdepl_dn4 / locals.var_wrdrdjunc) + (locals.var_wjunc_dn4 / locals.var_rd_xldld))))), (locals.var_xmax * (-(locals.var_cx * ((locals.var_wdepl_dn5 / locals.var_wrdrdjunc) + (locals.var_wjunc_dn5 / locals.var_rd_xldld))))), (locals.var_xmax * (-(locals.var_cx * ((locals.var_wdepl_dn6 / locals.var_wrdrdjunc) + (locals.var_wjunc_dn6 / locals.var_rd_xldld))))), (locals.var_xmax * (-(locals.var_cx * ((locals.var_wdepl_dn7 / locals.var_wrdrdjunc) + (locals.var_wjunc_dn7 / locals.var_rd_xldld))))), (locals.var_xmax * (-(locals.var_cx * ((locals.var_wdepl_dn8 / locals.var_wrdrdjunc) + (locals.var_wjunc_dn8 / locals.var_rd_xldld))))), (locals.var_xmax * (-(locals.var_cx * ((locals.var_wdepl_dn9 / locals.var_wrdrdjunc) + (locals.var_wjunc_dn9 / locals.var_rd_xldld))))), (locals.var_xmax * (-(locals.var_cx * ((locals.var_wdepl_dn10 / locals.var_wrdrdjunc) + (locals.var_wjunc_dn10 / locals.var_rd_xldld))))), (locals.var_xmax * (-(locals.var_cx * ((locals.var_wdepl_dn13 / locals.var_wrdrdjunc) + (locals.var_wjunc_dn13 / locals.var_rd_xldld))))),)
    } else {
        (locals.var_xov, locals.var_xov_dn0, locals.var_xov_dn2, locals.var_xov_dn4, locals.var_xov_dn5, locals.var_xov_dn6, locals.var_xov_dn7, locals.var_xov_dn8, locals.var_xov_dn9, locals.var_xov_dn10, locals.var_xov_dn13,)
    }
};
        locals.var_xov = assign104870_e157234;
        locals.var_xov_dn0 = assign104870_e157234_d_n0;
        locals.var_xov_dn2 = assign104870_e157234_d_n2;
        locals.var_xov_dn4 = assign104870_e157234_d_n4;
        locals.var_xov_dn5 = assign104870_e157234_d_n5;
        locals.var_xov_dn6 = assign104870_e157234_d_n6;
        locals.var_xov_dn7 = assign104870_e157234_d_n7;
        locals.var_xov_dn8 = assign104870_e157234_d_n8;
        locals.var_xov_dn9 = assign104870_e157234_d_n9;
        locals.var_xov_dn10 = assign104870_e157234_d_n10;
        locals.var_xov_dn13 = assign104870_e157234_d_n13;

        let (assign104880_e157262, assign104880_e157262_d_n0, assign104880_e157262_d_n2, assign104880_e157262_d_n4, assign104880_e157262_d_n5, assign104880_e157262_d_n6, assign104880_e157262_d_n7, assign104880_e157262_d_n8, assign104880_e157262_d_n9, assign104880_e157262_d_n10, assign104880_e157262_d_n13,) = {
    if ((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) {
        let assign104880_e157241: f64 = (locals.var_xov * locals.var_xov);
        let assign104880_e157245: f64 = (1.0 - locals.var_uc_rdrcx);
        let assign104880_e157247: f64 = (assign104880_e157245 * locals.var_xmax);
        let assign104880_e157249: f64 = (assign104880_e157247 / 100.0);
        let assign104880_e157250: f64 = (4.0 * assign104880_e157249);
        let assign104880_e157253: f64 = (1.0 - locals.var_uc_rdrcx);
        let assign104880_e157255: f64 = (assign104880_e157253 * locals.var_xmax);
        let assign104880_e157257: f64 = (assign104880_e157255 / 100.0);
        let assign104880_e157258: f64 = (assign104880_e157250 * assign104880_e157257);
        let assign104880_e157259: f64 = (assign104880_e157241 + assign104880_e157258);
        let assign104880_e157260: f64 = (assign104880_e157259).sqrt();
        (assign104880_e157260, (((locals.var_xov_dn0 * locals.var_xov) + (locals.var_xov * locals.var_xov_dn0)) / (2.0 * assign104880_e157260)), (((locals.var_xov_dn2 * locals.var_xov) + (locals.var_xov * locals.var_xov_dn2)) / (2.0 * assign104880_e157260)), (((locals.var_xov_dn4 * locals.var_xov) + (locals.var_xov * locals.var_xov_dn4)) / (2.0 * assign104880_e157260)), (((locals.var_xov_dn5 * locals.var_xov) + (locals.var_xov * locals.var_xov_dn5)) / (2.0 * assign104880_e157260)), (((locals.var_xov_dn6 * locals.var_xov) + (locals.var_xov * locals.var_xov_dn6)) / (2.0 * assign104880_e157260)), (((locals.var_xov_dn7 * locals.var_xov) + (locals.var_xov * locals.var_xov_dn7)) / (2.0 * assign104880_e157260)), (((locals.var_xov_dn8 * locals.var_xov) + (locals.var_xov * locals.var_xov_dn8)) / (2.0 * assign104880_e157260)), (((locals.var_xov_dn9 * locals.var_xov) + (locals.var_xov * locals.var_xov_dn9)) / (2.0 * assign104880_e157260)), (((locals.var_xov_dn10 * locals.var_xov) + (locals.var_xov * locals.var_xov_dn10)) / (2.0 * assign104880_e157260)), (((locals.var_xov_dn13 * locals.var_xov) + (locals.var_xov * locals.var_xov_dn13)) / (2.0 * assign104880_e157260)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign104880_e157262;
        locals.var_tmf2_dn0 = assign104880_e157262_d_n0;
        locals.var_tmf2_dn2 = assign104880_e157262_d_n2;
        locals.var_tmf2_dn4 = assign104880_e157262_d_n4;
        locals.var_tmf2_dn5 = assign104880_e157262_d_n5;
        locals.var_tmf2_dn6 = assign104880_e157262_d_n6;
        locals.var_tmf2_dn7 = assign104880_e157262_d_n7;
        locals.var_tmf2_dn8 = assign104880_e157262_d_n8;
        locals.var_tmf2_dn9 = assign104880_e157262_d_n9;
        locals.var_tmf2_dn10 = assign104880_e157262_d_n10;
        locals.var_tmf2_dn13 = assign104880_e157262_d_n13;

        let (assign104890_e157275, assign104890_e157275_d_n0, assign104890_e157275_d_n2, assign104890_e157275_d_n4, assign104890_e157275_d_n5, assign104890_e157275_d_n6, assign104890_e157275_d_n7, assign104890_e157275_d_n8, assign104890_e157275_d_n9, assign104890_e157275_d_n10, assign104890_e157275_d_n13,) = {
    if ((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) {
        let assign104890_e157271: f64 = (locals.var_xov / locals.var_tmf2);
        let assign104890_e157272: f64 = (1.0 + assign104890_e157271);
        let assign104890_e157273: f64 = (0.5 * assign104890_e157272);
        (assign104890_e157273, (0.5 * (((locals.var_xov_dn0 * locals.var_tmf2) - (locals.var_xov * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_xov_dn2 * locals.var_tmf2) - (locals.var_xov * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_xov_dn4 * locals.var_tmf2) - (locals.var_xov * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_xov_dn5 * locals.var_tmf2) - (locals.var_xov * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_xov_dn6 * locals.var_tmf2) - (locals.var_xov * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_xov_dn7 * locals.var_tmf2) - (locals.var_xov * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_xov_dn8 * locals.var_tmf2) - (locals.var_xov * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_xov_dn9 * locals.var_tmf2) - (locals.var_xov * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_xov_dn10 * locals.var_tmf2) - (locals.var_xov * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_xov_dn13 * locals.var_tmf2) - (locals.var_xov * locals.var_tmf2_dn13)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn13,)
    }
};
        locals.var_t9 = assign104890_e157275;
        locals.var_t9_dn0 = assign104890_e157275_d_n0;
        locals.var_t9_dn2 = assign104890_e157275_d_n2;
        locals.var_t9_dn4 = assign104890_e157275_d_n4;
        locals.var_t9_dn5 = assign104890_e157275_d_n5;
        locals.var_t9_dn6 = assign104890_e157275_d_n6;
        locals.var_t9_dn7 = assign104890_e157275_d_n7;
        locals.var_t9_dn8 = assign104890_e157275_d_n8;
        locals.var_t9_dn9 = assign104890_e157275_d_n9;
        locals.var_t9_dn10 = assign104890_e157275_d_n10;
        locals.var_t9_dn13 = assign104890_e157275_d_n13;

        let (assign104900_e157286, assign104900_e157286_d_n0, assign104900_e157286_d_n2, assign104900_e157286_d_n4, assign104900_e157286_d_n5, assign104900_e157286_d_n6, assign104900_e157286_d_n7, assign104900_e157286_d_n8, assign104900_e157286_d_n9, assign104900_e157286_d_n10, assign104900_e157286_d_n13,) = {
    if ((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) {
        let assign104900_e157283: f64 = (locals.var_xov + locals.var_tmf2);
        let assign104900_e157284: f64 = (0.5 * assign104900_e157283);
        (assign104900_e157284, (0.5 * (locals.var_xov_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_xov_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_xov_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_xov_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_xov_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_xov_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_xov_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_xov_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_xov_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_xov_dn13 + locals.var_tmf2_dn13)),)
    } else {
        (locals.var_xov, locals.var_xov_dn0, locals.var_xov_dn2, locals.var_xov_dn4, locals.var_xov_dn5, locals.var_xov_dn6, locals.var_xov_dn7, locals.var_xov_dn8, locals.var_xov_dn9, locals.var_xov_dn10, locals.var_xov_dn13,)
    }
};
        locals.var_xov = assign104900_e157286;
        locals.var_xov_dn0 = assign104900_e157286_d_n0;
        locals.var_xov_dn2 = assign104900_e157286_d_n2;
        locals.var_xov_dn4 = assign104900_e157286_d_n4;
        locals.var_xov_dn5 = assign104900_e157286_d_n5;
        locals.var_xov_dn6 = assign104900_e157286_d_n6;
        locals.var_xov_dn7 = assign104900_e157286_d_n7;
        locals.var_xov_dn8 = assign104900_e157286_d_n8;
        locals.var_xov_dn9 = assign104900_e157286_d_n9;
        locals.var_xov_dn10 = assign104900_e157286_d_n10;
        locals.var_xov_dn13 = assign104900_e157286_d_n13;

        let assign104910_e157289: f64 = if locals.var_xov < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2380 = assign104910_e157289;

        let (assign104920_e157298, assign104920_e157298_d_n0, assign104920_e157298_d_n2, assign104920_e157298_d_n4, assign104920_e157298_d_n5, assign104920_e157298_d_n6, assign104920_e157298_d_n7, assign104920_e157298_d_n8, assign104920_e157298_d_n9, assign104920_e157298_d_n10, assign104920_e157298_d_n13,) = {
    if (((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2380 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xov, locals.var_xov_dn0, locals.var_xov_dn2, locals.var_xov_dn4, locals.var_xov_dn5, locals.var_xov_dn6, locals.var_xov_dn7, locals.var_xov_dn8, locals.var_xov_dn9, locals.var_xov_dn10, locals.var_xov_dn13,)
    }
};
        locals.var_xov = assign104920_e157298;
        locals.var_xov_dn0 = assign104920_e157298_d_n0;
        locals.var_xov_dn2 = assign104920_e157298_d_n2;
        locals.var_xov_dn4 = assign104920_e157298_d_n4;
        locals.var_xov_dn5 = assign104920_e157298_d_n5;
        locals.var_xov_dn6 = assign104920_e157298_d_n6;
        locals.var_xov_dn7 = assign104920_e157298_d_n7;
        locals.var_xov_dn8 = assign104920_e157298_d_n8;
        locals.var_xov_dn9 = assign104920_e157298_d_n9;
        locals.var_xov_dn10 = assign104920_e157298_d_n10;
        locals.var_xov_dn13 = assign104920_e157298_d_n13;

    }

    pub(super) fn stamp_transient_block_370(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign104930_e157307, assign104930_e157307_d_n0, assign104930_e157307_d_n2, assign104930_e157307_d_n4, assign104930_e157307_d_n5, assign104930_e157307_d_n6, assign104930_e157307_d_n7, assign104930_e157307_d_n8, assign104930_e157307_d_n9, assign104930_e157307_d_n10, assign104930_e157307_d_n13,) = {
    if (((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2380 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn13,)
    }
};
        locals.var_t9 = assign104930_e157307;
        locals.var_t9_dn0 = assign104930_e157307_d_n0;
        locals.var_t9_dn2 = assign104930_e157307_d_n2;
        locals.var_t9_dn4 = assign104930_e157307_d_n4;
        locals.var_t9_dn5 = assign104930_e157307_d_n5;
        locals.var_t9_dn6 = assign104930_e157307_d_n6;
        locals.var_t9_dn7 = assign104930_e157307_d_n7;
        locals.var_t9_dn8 = assign104930_e157307_d_n8;
        locals.var_t9_dn9 = assign104930_e157307_d_n9;
        locals.var_t9_dn10 = assign104930_e157307_d_n10;
        locals.var_t9_dn13 = assign104930_e157307_d_n13;

        let (assign104940_e157318, assign104940_e157318_d_n0, assign104940_e157318_d_n2, assign104940_e157318_d_n4, assign104940_e157318_d_n5, assign104940_e157318_d_n6, assign104940_e157318_d_n7, assign104940_e157318_d_n8, assign104940_e157318_d_n9, assign104940_e157318_d_n10, assign104940_e157318_d_n13,) = {
    if ((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) {
        let assign104940_e157315: f64 = (locals.var_ldrifte + p.p422);
        let assign104940_e157316: f64 = (1.6021918e-19 / assign104940_e157315);
        (assign104940_e157316, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign104940_e157318;
        locals.var_t1_dn0 = assign104940_e157318_d_n0;
        locals.var_t1_dn2 = assign104940_e157318_d_n2;
        locals.var_t1_dn4 = assign104940_e157318_d_n4;
        locals.var_t1_dn5 = assign104940_e157318_d_n5;
        locals.var_t1_dn6 = assign104940_e157318_d_n6;
        locals.var_t1_dn7 = assign104940_e157318_d_n7;
        locals.var_t1_dn8 = assign104940_e157318_d_n8;
        locals.var_t1_dn9 = assign104940_e157318_d_n9;
        locals.var_t1_dn10 = assign104940_e157318_d_n10;
        locals.var_t1_dn13 = assign104940_e157318_d_n13;

        let (assign104950_e157331, assign104950_e157331_d_n0, assign104950_e157331_d_n2, assign104950_e157331_d_n4, assign104950_e157331_d_n5, assign104950_e157331_d_n6, assign104950_e157331_d_n7, assign104950_e157331_d_n8, assign104950_e157331_d_n9, assign104950_e157331_d_n10, assign104950_e157331_d_n13,) = {
    if ((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) {
        let assign104950_e157325: f64 = (locals.var_t1 * locals.var_xov);
        let assign104950_e157327: f64 = (assign104950_e157325 * locals.var_mu__blk2354);
        let assign104950_e157329: f64 = (assign104950_e157327 * locals.var_carr);
        (assign104950_e157329, ((((((locals.var_t1_dn0 * locals.var_xov) + (locals.var_t1 * locals.var_xov_dn0)) * locals.var_mu__blk2354) + (assign104950_e157325 * locals.var_mu__blk2354_dn0)) * locals.var_carr) + (assign104950_e157327 * locals.var_carr_dn0)), ((((((locals.var_t1_dn2 * locals.var_xov) + (locals.var_t1 * locals.var_xov_dn2)) * locals.var_mu__blk2354) + (assign104950_e157325 * locals.var_mu__blk2354_dn2)) * locals.var_carr) + (assign104950_e157327 * locals.var_carr_dn2)), ((((((locals.var_t1_dn4 * locals.var_xov) + (locals.var_t1 * locals.var_xov_dn4)) * locals.var_mu__blk2354) + (assign104950_e157325 * locals.var_mu__blk2354_dn4)) * locals.var_carr) + (assign104950_e157327 * locals.var_carr_dn4)), ((((((locals.var_t1_dn5 * locals.var_xov) + (locals.var_t1 * locals.var_xov_dn5)) * locals.var_mu__blk2354) + (assign104950_e157325 * locals.var_mu__blk2354_dn5)) * locals.var_carr) + (assign104950_e157327 * locals.var_carr_dn5)), ((((((locals.var_t1_dn6 * locals.var_xov) + (locals.var_t1 * locals.var_xov_dn6)) * locals.var_mu__blk2354) + (assign104950_e157325 * locals.var_mu__blk2354_dn6)) * locals.var_carr) + (assign104950_e157327 * locals.var_carr_dn6)), ((((((locals.var_t1_dn7 * locals.var_xov) + (locals.var_t1 * locals.var_xov_dn7)) * locals.var_mu__blk2354) + (assign104950_e157325 * locals.var_mu__blk2354_dn7)) * locals.var_carr) + (assign104950_e157327 * locals.var_carr_dn7)), ((((((locals.var_t1_dn8 * locals.var_xov) + (locals.var_t1 * locals.var_xov_dn8)) * locals.var_mu__blk2354) + (assign104950_e157325 * locals.var_mu__blk2354_dn8)) * locals.var_carr) + (assign104950_e157327 * locals.var_carr_dn8)), ((((((locals.var_t1_dn9 * locals.var_xov) + (locals.var_t1 * locals.var_xov_dn9)) * locals.var_mu__blk2354) + (assign104950_e157325 * locals.var_mu__blk2354_dn9)) * locals.var_carr) + (assign104950_e157327 * locals.var_carr_dn9)), ((((((locals.var_t1_dn10 * locals.var_xov) + (locals.var_t1 * locals.var_xov_dn10)) * locals.var_mu__blk2354) + (assign104950_e157325 * locals.var_mu__blk2354_dn10)) * locals.var_carr) + (assign104950_e157327 * locals.var_carr_dn10)), ((((((locals.var_t1_dn13 * locals.var_xov) + (locals.var_t1 * locals.var_xov_dn13)) * locals.var_mu__blk2354) + (assign104950_e157325 * locals.var_mu__blk2354_dn13)) * locals.var_carr) + (assign104950_e157327 * locals.var_carr_dn13)),)
    } else {
        (locals.var_gd, locals.var_gd_dn0, locals.var_gd_dn2, locals.var_gd_dn4, locals.var_gd_dn5, locals.var_gd_dn6, locals.var_gd_dn7, locals.var_gd_dn8, locals.var_gd_dn9, locals.var_gd_dn10, locals.var_gd_dn13,)
    }
};
        locals.var_gd = assign104950_e157331;
        locals.var_gd_dn0 = assign104950_e157331_d_n0;
        locals.var_gd_dn2 = assign104950_e157331_d_n2;
        locals.var_gd_dn4 = assign104950_e157331_d_n4;
        locals.var_gd_dn5 = assign104950_e157331_d_n5;
        locals.var_gd_dn6 = assign104950_e157331_d_n6;
        locals.var_gd_dn7 = assign104950_e157331_d_n7;
        locals.var_gd_dn8 = assign104950_e157331_d_n8;
        locals.var_gd_dn9 = assign104950_e157331_d_n9;
        locals.var_gd_dn10 = assign104950_e157331_d_n10;
        locals.var_gd_dn13 = assign104950_e157331_d_n13;

        let assign104960_e157335: f64 = 1e-25;
        let assign104960_e157340: f64 = if ((locals.var_gd < assign104960_e157335) && (1e-25 >= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard2381 = assign104960_e157340;

        let (assign104970_e157353, assign104970_e157353_d_n0, assign104970_e157353_d_n2, assign104970_e157353_d_n4, assign104970_e157353_d_n5, assign104970_e157353_d_n6, assign104970_e157353_d_n7, assign104970_e157353_d_n8, assign104970_e157353_d_n9, assign104970_e157353_d_n10, assign104970_e157353_d_n13,) = {
    if (((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2381 != 0.0)) {
        let assign104970_e157349: f64 = 1e-25;
        let assign104970_e157351: f64 = (assign104970_e157349 - locals.var_gd);
        (assign104970_e157351, (-locals.var_gd_dn0), (-locals.var_gd_dn2), (-locals.var_gd_dn4), (-locals.var_gd_dn5), (-locals.var_gd_dn6), (-locals.var_gd_dn7), (-locals.var_gd_dn8), (-locals.var_gd_dn9), (-locals.var_gd_dn10), (-locals.var_gd_dn13),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13,)
    }
};
        locals.var_tmf1 = assign104970_e157353;
        locals.var_tmf1_dn0 = assign104970_e157353_d_n0;
        locals.var_tmf1_dn2 = assign104970_e157353_d_n2;
        locals.var_tmf1_dn4 = assign104970_e157353_d_n4;
        locals.var_tmf1_dn5 = assign104970_e157353_d_n5;
        locals.var_tmf1_dn6 = assign104970_e157353_d_n6;
        locals.var_tmf1_dn7 = assign104970_e157353_d_n7;
        locals.var_tmf1_dn8 = assign104970_e157353_d_n8;
        locals.var_tmf1_dn9 = assign104970_e157353_d_n9;
        locals.var_tmf1_dn10 = assign104970_e157353_d_n10;
        locals.var_tmf1_dn13 = assign104970_e157353_d_n13;

        let (assign104980_e157364, assign104980_e157364_d_n0, assign104980_e157364_d_n2, assign104980_e157364_d_n4, assign104980_e157364_d_n5, assign104980_e157364_d_n6, assign104980_e157364_d_n7, assign104980_e157364_d_n8, assign104980_e157364_d_n9, assign104980_e157364_d_n10, assign104980_e157364_d_n13,) = {
    if (((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2381 != 0.0)) {
        let assign104980_e157362: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign104980_e157362, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)), ((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)), ((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn13 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn13)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn4, locals.var_x2_dn5, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn8, locals.var_x2_dn9, locals.var_x2_dn10, locals.var_x2_dn13,)
    }
};
        locals.var_x2 = assign104980_e157364;
        locals.var_x2_dn0 = assign104980_e157364_d_n0;
        locals.var_x2_dn2 = assign104980_e157364_d_n2;
        locals.var_x2_dn4 = assign104980_e157364_d_n4;
        locals.var_x2_dn5 = assign104980_e157364_d_n5;
        locals.var_x2_dn6 = assign104980_e157364_d_n6;
        locals.var_x2_dn7 = assign104980_e157364_d_n7;
        locals.var_x2_dn8 = assign104980_e157364_d_n8;
        locals.var_x2_dn9 = assign104980_e157364_d_n9;
        locals.var_x2_dn10 = assign104980_e157364_d_n10;
        locals.var_x2_dn13 = assign104980_e157364_d_n13;

        let (assign104990_e157375, assign104990_e157375_d_n0, assign104990_e157375_d_n2, assign104990_e157375_d_n4, assign104990_e157375_d_n5, assign104990_e157375_d_n6, assign104990_e157375_d_n7, assign104990_e157375_d_n8, assign104990_e157375_d_n9, assign104990_e157375_d_n10, assign104990_e157375_d_n13,) = {
    if (((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2381 != 0.0)) {
        let assign104990_e157373: f64 = (1e-25 * 1e-25);
        (assign104990_e157373, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn4, locals.var_xmax2_dn5, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn8, locals.var_xmax2_dn9, locals.var_xmax2_dn10, locals.var_xmax2_dn13,)
    }
};
        locals.var_xmax2 = assign104990_e157375;
        locals.var_xmax2_dn0 = assign104990_e157375_d_n0;
        locals.var_xmax2_dn2 = assign104990_e157375_d_n2;
        locals.var_xmax2_dn4 = assign104990_e157375_d_n4;
        locals.var_xmax2_dn5 = assign104990_e157375_d_n5;
        locals.var_xmax2_dn6 = assign104990_e157375_d_n6;
        locals.var_xmax2_dn7 = assign104990_e157375_d_n7;
        locals.var_xmax2_dn8 = assign104990_e157375_d_n8;
        locals.var_xmax2_dn9 = assign104990_e157375_d_n9;
        locals.var_xmax2_dn10 = assign104990_e157375_d_n10;
        locals.var_xmax2_dn13 = assign104990_e157375_d_n13;

        let (assign105000_e157384, assign105000_e157384_d_n0, assign105000_e157384_d_n2, assign105000_e157384_d_n4, assign105000_e157384_d_n5, assign105000_e157384_d_n6, assign105000_e157384_d_n7, assign105000_e157384_d_n8, assign105000_e157384_d_n9, assign105000_e157384_d_n10, assign105000_e157384_d_n13,) = {
    if (((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2381 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
        locals.var_xp = assign105000_e157384;
        locals.var_xp_dn0 = assign105000_e157384_d_n0;
        locals.var_xp_dn2 = assign105000_e157384_d_n2;
        locals.var_xp_dn4 = assign105000_e157384_d_n4;
        locals.var_xp_dn5 = assign105000_e157384_d_n5;
        locals.var_xp_dn6 = assign105000_e157384_d_n6;
        locals.var_xp_dn7 = assign105000_e157384_d_n7;
        locals.var_xp_dn8 = assign105000_e157384_d_n8;
        locals.var_xp_dn9 = assign105000_e157384_d_n9;
        locals.var_xp_dn10 = assign105000_e157384_d_n10;
        locals.var_xp_dn13 = assign105000_e157384_d_n13;

        let (assign105010_e157393, assign105010_e157393_d_n0, assign105010_e157393_d_n2, assign105010_e157393_d_n4, assign105010_e157393_d_n5, assign105010_e157393_d_n6, assign105010_e157393_d_n7, assign105010_e157393_d_n8, assign105010_e157393_d_n9, assign105010_e157393_d_n10, assign105010_e157393_d_n13,) = {
    if (((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2381 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
        locals.var_xmp = assign105010_e157393;
        locals.var_xmp_dn0 = assign105010_e157393_d_n0;
        locals.var_xmp_dn2 = assign105010_e157393_d_n2;
        locals.var_xmp_dn4 = assign105010_e157393_d_n4;
        locals.var_xmp_dn5 = assign105010_e157393_d_n5;
        locals.var_xmp_dn6 = assign105010_e157393_d_n6;
        locals.var_xmp_dn7 = assign105010_e157393_d_n7;
        locals.var_xmp_dn8 = assign105010_e157393_d_n8;
        locals.var_xmp_dn9 = assign105010_e157393_d_n9;
        locals.var_xmp_dn10 = assign105010_e157393_d_n10;
        locals.var_xmp_dn13 = assign105010_e157393_d_n13;

        let (assign105020_e157402,) = {
    if (((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2381 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign105020_e157402;

        let (assign105030_e157411,) = {
    if (((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2381 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign105030_e157411;

        let (assign105040_e157420, assign105040_e157420_d_n0, assign105040_e157420_d_n2, assign105040_e157420_d_n4, assign105040_e157420_d_n5, assign105040_e157420_d_n6, assign105040_e157420_d_n7, assign105040_e157420_d_n8, assign105040_e157420_d_n9, assign105040_e157420_d_n10, assign105040_e157420_d_n13,) = {
    if (((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2381 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    }
};
        locals.var_arg = assign105040_e157420;
        locals.var_arg_dn0 = assign105040_e157420_d_n0;
        locals.var_arg_dn2 = assign105040_e157420_d_n2;
        locals.var_arg_dn4 = assign105040_e157420_d_n4;
        locals.var_arg_dn5 = assign105040_e157420_d_n5;
        locals.var_arg_dn6 = assign105040_e157420_d_n6;
        locals.var_arg_dn7 = assign105040_e157420_d_n7;
        locals.var_arg_dn8 = assign105040_e157420_d_n8;
        locals.var_arg_dn9 = assign105040_e157420_d_n9;
        locals.var_arg_dn10 = assign105040_e157420_d_n10;
        locals.var_arg_dn13 = assign105040_e157420_d_n13;

        let (assign105050_e157429, assign105050_e157429_d_n0, assign105050_e157429_d_n2, assign105050_e157429_d_n4, assign105050_e157429_d_n5, assign105050_e157429_d_n6, assign105050_e157429_d_n7, assign105050_e157429_d_n8, assign105050_e157429_d_n9, assign105050_e157429_d_n10, assign105050_e157429_d_n13,) = {
    if (((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2381 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign105050_e157429;
        locals.var_dnm_dn0 = assign105050_e157429_d_n0;
        locals.var_dnm_dn2 = assign105050_e157429_d_n2;
        locals.var_dnm_dn4 = assign105050_e157429_d_n4;
        locals.var_dnm_dn5 = assign105050_e157429_d_n5;
        locals.var_dnm_dn6 = assign105050_e157429_d_n6;
        locals.var_dnm_dn7 = assign105050_e157429_d_n7;
        locals.var_dnm_dn8 = assign105050_e157429_d_n8;
        locals.var_dnm_dn9 = assign105050_e157429_d_n9;
        locals.var_dnm_dn10 = assign105050_e157429_d_n10;
        locals.var_dnm_dn13 = assign105050_e157429_d_n13;

        let (assign105060_e157440, assign105060_e157440_d_n0, assign105060_e157440_d_n2, assign105060_e157440_d_n4, assign105060_e157440_d_n5, assign105060_e157440_d_n6, assign105060_e157440_d_n7, assign105060_e157440_d_n8, assign105060_e157440_d_n9, assign105060_e157440_d_n10, assign105060_e157440_d_n13,) = {
    if (((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2381 != 0.0)) {
        let assign105060_e157438: f64 = (locals.var_xp * locals.var_x2);
        (assign105060_e157438, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn13 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn13)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
        locals.var_xp = assign105060_e157440;
        locals.var_xp_dn0 = assign105060_e157440_d_n0;
        locals.var_xp_dn2 = assign105060_e157440_d_n2;
        locals.var_xp_dn4 = assign105060_e157440_d_n4;
        locals.var_xp_dn5 = assign105060_e157440_d_n5;
        locals.var_xp_dn6 = assign105060_e157440_d_n6;
        locals.var_xp_dn7 = assign105060_e157440_d_n7;
        locals.var_xp_dn8 = assign105060_e157440_d_n8;
        locals.var_xp_dn9 = assign105060_e157440_d_n9;
        locals.var_xp_dn10 = assign105060_e157440_d_n10;
        locals.var_xp_dn13 = assign105060_e157440_d_n13;

        let (assign105070_e157451, assign105070_e157451_d_n0, assign105070_e157451_d_n2, assign105070_e157451_d_n4, assign105070_e157451_d_n5, assign105070_e157451_d_n6, assign105070_e157451_d_n7, assign105070_e157451_d_n8, assign105070_e157451_d_n9, assign105070_e157451_d_n10, assign105070_e157451_d_n13,) = {
    if (((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2381 != 0.0)) {
        let assign105070_e157449: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign105070_e157449, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn13 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn13)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
        locals.var_xmp = assign105070_e157451;
        locals.var_xmp_dn0 = assign105070_e157451_d_n0;
        locals.var_xmp_dn2 = assign105070_e157451_d_n2;
        locals.var_xmp_dn4 = assign105070_e157451_d_n4;
        locals.var_xmp_dn5 = assign105070_e157451_d_n5;
        locals.var_xmp_dn6 = assign105070_e157451_d_n6;
        locals.var_xmp_dn7 = assign105070_e157451_d_n7;
        locals.var_xmp_dn8 = assign105070_e157451_d_n8;
        locals.var_xmp_dn9 = assign105070_e157451_d_n9;
        locals.var_xmp_dn10 = assign105070_e157451_d_n10;
        locals.var_xmp_dn13 = assign105070_e157451_d_n13;

        let (assign105080_e157462, assign105080_e157462_d_n0, assign105080_e157462_d_n2, assign105080_e157462_d_n4, assign105080_e157462_d_n5, assign105080_e157462_d_n6, assign105080_e157462_d_n7, assign105080_e157462_d_n8, assign105080_e157462_d_n9, assign105080_e157462_d_n10, assign105080_e157462_d_n13,) = {
    if (((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2381 != 0.0)) {
        let assign105080_e157460: f64 = (locals.var_xp * locals.var_x2);
        (assign105080_e157460, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn13 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn13)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
        locals.var_xp = assign105080_e157462;
        locals.var_xp_dn0 = assign105080_e157462_d_n0;
        locals.var_xp_dn2 = assign105080_e157462_d_n2;
        locals.var_xp_dn4 = assign105080_e157462_d_n4;
        locals.var_xp_dn5 = assign105080_e157462_d_n5;
        locals.var_xp_dn6 = assign105080_e157462_d_n6;
        locals.var_xp_dn7 = assign105080_e157462_d_n7;
        locals.var_xp_dn8 = assign105080_e157462_d_n8;
        locals.var_xp_dn9 = assign105080_e157462_d_n9;
        locals.var_xp_dn10 = assign105080_e157462_d_n10;
        locals.var_xp_dn13 = assign105080_e157462_d_n13;

        let (assign105090_e157473, assign105090_e157473_d_n0, assign105090_e157473_d_n2, assign105090_e157473_d_n4, assign105090_e157473_d_n5, assign105090_e157473_d_n6, assign105090_e157473_d_n7, assign105090_e157473_d_n8, assign105090_e157473_d_n9, assign105090_e157473_d_n10, assign105090_e157473_d_n13,) = {
    if (((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2381 != 0.0)) {
        let assign105090_e157471: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign105090_e157471, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn13 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn13)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
        locals.var_xmp = assign105090_e157473;
        locals.var_xmp_dn0 = assign105090_e157473_d_n0;
        locals.var_xmp_dn2 = assign105090_e157473_d_n2;
        locals.var_xmp_dn4 = assign105090_e157473_d_n4;
        locals.var_xmp_dn5 = assign105090_e157473_d_n5;
        locals.var_xmp_dn6 = assign105090_e157473_d_n6;
        locals.var_xmp_dn7 = assign105090_e157473_d_n7;
        locals.var_xmp_dn8 = assign105090_e157473_d_n8;
        locals.var_xmp_dn9 = assign105090_e157473_d_n9;
        locals.var_xmp_dn10 = assign105090_e157473_d_n10;
        locals.var_xmp_dn13 = assign105090_e157473_d_n13;

        let (assign105100_e157484, assign105100_e157484_d_n0, assign105100_e157484_d_n2, assign105100_e157484_d_n4, assign105100_e157484_d_n5, assign105100_e157484_d_n6, assign105100_e157484_d_n7, assign105100_e157484_d_n8, assign105100_e157484_d_n9, assign105100_e157484_d_n10, assign105100_e157484_d_n13,) = {
    if (((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2381 != 0.0)) {
        let assign105100_e157482: f64 = (locals.var_xp + locals.var_xmp);
        (assign105100_e157482, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn4 + locals.var_xmp_dn4), (locals.var_xp_dn5 + locals.var_xmp_dn5), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn8 + locals.var_xmp_dn8), (locals.var_xp_dn9 + locals.var_xmp_dn9), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn13 + locals.var_xmp_dn13),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    }
};
        locals.var_arg = assign105100_e157484;
        locals.var_arg_dn0 = assign105100_e157484_d_n0;
        locals.var_arg_dn2 = assign105100_e157484_d_n2;
        locals.var_arg_dn4 = assign105100_e157484_d_n4;
        locals.var_arg_dn5 = assign105100_e157484_d_n5;
        locals.var_arg_dn6 = assign105100_e157484_d_n6;
        locals.var_arg_dn7 = assign105100_e157484_d_n7;
        locals.var_arg_dn8 = assign105100_e157484_d_n8;
        locals.var_arg_dn9 = assign105100_e157484_d_n9;
        locals.var_arg_dn10 = assign105100_e157484_d_n10;
        locals.var_arg_dn13 = assign105100_e157484_d_n13;

        let (assign105110_e157493, assign105110_e157493_d_n0, assign105110_e157493_d_n2, assign105110_e157493_d_n4, assign105110_e157493_d_n5, assign105110_e157493_d_n6, assign105110_e157493_d_n7, assign105110_e157493_d_n8, assign105110_e157493_d_n9, assign105110_e157493_d_n10, assign105110_e157493_d_n13,) = {
    if (((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2381 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign105110_e157493;
        locals.var_dnm_dn0 = assign105110_e157493_d_n0;
        locals.var_dnm_dn2 = assign105110_e157493_d_n2;
        locals.var_dnm_dn4 = assign105110_e157493_d_n4;
        locals.var_dnm_dn5 = assign105110_e157493_d_n5;
        locals.var_dnm_dn6 = assign105110_e157493_d_n6;
        locals.var_dnm_dn7 = assign105110_e157493_d_n7;
        locals.var_dnm_dn8 = assign105110_e157493_d_n8;
        locals.var_dnm_dn9 = assign105110_e157493_d_n9;
        locals.var_dnm_dn10 = assign105110_e157493_d_n10;
        locals.var_dnm_dn13 = assign105110_e157493_d_n13;

        let assign105120_e157508: f64 = if ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard2382 = assign105120_e157508;

        let assign105130_e157511: f64 = if 2.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard2383 = assign105130_e157511;

        let (assign105140_e157524,) = {
    if (((((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2381 != 0.0)) && (locals.var_guard2382 != 0.0)) && (locals.var_guard2383 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign105140_e157524;

        let assign105150_e157527: f64 = if 2.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard2384 = assign105150_e157527;

        let (assign105160_e157543,) = {
    if ((((((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2381 != 0.0)) && (locals.var_guard2382 != 0.0)) && (locals.var_guard2383 == 0.0)) && (locals.var_guard2384 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign105160_e157543;

        let assign105170_e157546: f64 = if 2.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard2385 = assign105170_e157546;

        let (assign105180_e157565,) = {
    if (((((((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2381 != 0.0)) && (locals.var_guard2382 != 0.0)) && (locals.var_guard2383 == 0.0)) && (locals.var_guard2384 == 0.0)) && (locals.var_guard2385 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign105180_e157565;

        let assign105190_e157568: f64 = if 2.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard2386 = assign105190_e157568;

        let (assign105200_e157590,) = {
    if ((((((((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2381 != 0.0)) && (locals.var_guard2382 != 0.0)) && (locals.var_guard2383 == 0.0)) && (locals.var_guard2384 == 0.0)) && (locals.var_guard2385 == 0.0)) && (locals.var_guard2386 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign105200_e157590;

        let (assign105210_e157601,) = {
    if ((((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2381 != 0.0)) && (locals.var_guard2382 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign105210_e157601;

        let mut assign105220_loop_guard: usize = 0;
        while {
            let assign105220_cond_e157613: f64 = if (((((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2381 != 0.0)) && (locals.var_guard2382 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign105220_cond_e157613 != 0.0
        } {
            assign105220_loop_guard += 1;
            assert!(assign105220_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign105220_body0_e157625, assign105220_body0_e157625_d_n0, assign105220_body0_e157625_d_n2, assign105220_body0_e157625_d_n4, assign105220_body0_e157625_d_n5, assign105220_body0_e157625_d_n6, assign105220_body0_e157625_d_n7, assign105220_body0_e157625_d_n8, assign105220_body0_e157625_d_n9, assign105220_body0_e157625_d_n10, assign105220_body0_e157625_d_n13,) = {
    if ((((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2381 != 0.0)) && (locals.var_guard2382 != 0.0)) {
        let assign105220_body0_e157623: f64 = (locals.var_dnm).sqrt();
        (assign105220_body0_e157623, (locals.var_dnm_dn0 / (2.0 * assign105220_body0_e157623)), (locals.var_dnm_dn2 / (2.0 * assign105220_body0_e157623)), (locals.var_dnm_dn4 / (2.0 * assign105220_body0_e157623)), (locals.var_dnm_dn5 / (2.0 * assign105220_body0_e157623)), (locals.var_dnm_dn6 / (2.0 * assign105220_body0_e157623)), (locals.var_dnm_dn7 / (2.0 * assign105220_body0_e157623)), (locals.var_dnm_dn8 / (2.0 * assign105220_body0_e157623)), (locals.var_dnm_dn9 / (2.0 * assign105220_body0_e157623)), (locals.var_dnm_dn10 / (2.0 * assign105220_body0_e157623)), (locals.var_dnm_dn13 / (2.0 * assign105220_body0_e157623)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
            locals.var_dnm = assign105220_body0_e157625;
            locals.var_dnm_dn0 = assign105220_body0_e157625_d_n0;
            locals.var_dnm_dn2 = assign105220_body0_e157625_d_n2;
            locals.var_dnm_dn4 = assign105220_body0_e157625_d_n4;
            locals.var_dnm_dn5 = assign105220_body0_e157625_d_n5;
            locals.var_dnm_dn6 = assign105220_body0_e157625_d_n6;
            locals.var_dnm_dn7 = assign105220_body0_e157625_d_n7;
            locals.var_dnm_dn8 = assign105220_body0_e157625_d_n8;
            locals.var_dnm_dn9 = assign105220_body0_e157625_d_n9;
            locals.var_dnm_dn10 = assign105220_body0_e157625_d_n10;
            locals.var_dnm_dn13 = assign105220_body0_e157625_d_n13;
            let (assign105220_body1_e157638,) = {
    if ((((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2381 != 0.0)) && (locals.var_guard2382 != 0.0)) {
        let assign105220_body1_e157636: f64 = (locals.var_m0 + 1.0);
        (assign105220_body1_e157636,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign105220_body1_e157638;
        }

        let (assign105230_e157661, assign105230_e157661_d_n0, assign105230_e157661_d_n2, assign105230_e157661_d_n4, assign105230_e157661_d_n5, assign105230_e157661_d_n6, assign105230_e157661_d_n7, assign105230_e157661_d_n8, assign105230_e157661_d_n9, assign105230_e157661_d_n10, assign105230_e157661_d_n13,) = {
    if ((((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2381 != 0.0)) && (locals.var_guard2382 == 0.0)) {
        let (assign105230_e157659, assign105230_e157659_d_n0, assign105230_e157659_d_n2, assign105230_e157659_d_n4, assign105230_e157659_d_n5, assign105230_e157659_d_n6, assign105230_e157659_d_n7, assign105230_e157659_d_n8, assign105230_e157659_d_n9, assign105230_e157659_d_n10, assign105230_e157659_d_n13,) = {
            if (locals.var_dnm == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign105230_e157656: f64 = (2.0 * 2.0);
                let assign105230_e157657: f64 = (1.0 / assign105230_e157656);
                let assign105230_e157658: f64 = (locals.var_dnm).powf(assign105230_e157657);
                (assign105230_e157658, if 0.0 == 0.0 && ((assign105230_e157657) as f64).is_finite() && ((assign105230_e157657) as f64).fract() == 0.0 { if assign105230_e157657 == 0.0 { 0.0 } else { (assign105230_e157657 * ((locals.var_dnm).powf(assign105230_e157657 - 1.0) * locals.var_dnm_dn0)) } } else { (assign105230_e157658 * (assign105230_e157657 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign105230_e157657) as f64).is_finite() && ((assign105230_e157657) as f64).fract() == 0.0 { if assign105230_e157657 == 0.0 { 0.0 } else { (assign105230_e157657 * ((locals.var_dnm).powf(assign105230_e157657 - 1.0) * locals.var_dnm_dn2)) } } else { (assign105230_e157658 * (assign105230_e157657 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign105230_e157657) as f64).is_finite() && ((assign105230_e157657) as f64).fract() == 0.0 { if assign105230_e157657 == 0.0 { 0.0 } else { (assign105230_e157657 * ((locals.var_dnm).powf(assign105230_e157657 - 1.0) * locals.var_dnm_dn4)) } } else { (assign105230_e157658 * (assign105230_e157657 * (locals.var_dnm_dn4 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign105230_e157657) as f64).is_finite() && ((assign105230_e157657) as f64).fract() == 0.0 { if assign105230_e157657 == 0.0 { 0.0 } else { (assign105230_e157657 * ((locals.var_dnm).powf(assign105230_e157657 - 1.0) * locals.var_dnm_dn5)) } } else { (assign105230_e157658 * (assign105230_e157657 * (locals.var_dnm_dn5 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign105230_e157657) as f64).is_finite() && ((assign105230_e157657) as f64).fract() == 0.0 { if assign105230_e157657 == 0.0 { 0.0 } else { (assign105230_e157657 * ((locals.var_dnm).powf(assign105230_e157657 - 1.0) * locals.var_dnm_dn6)) } } else { (assign105230_e157658 * (assign105230_e157657 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign105230_e157657) as f64).is_finite() && ((assign105230_e157657) as f64).fract() == 0.0 { if assign105230_e157657 == 0.0 { 0.0 } else { (assign105230_e157657 * ((locals.var_dnm).powf(assign105230_e157657 - 1.0) * locals.var_dnm_dn7)) } } else { (assign105230_e157658 * (assign105230_e157657 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign105230_e157657) as f64).is_finite() && ((assign105230_e157657) as f64).fract() == 0.0 { if assign105230_e157657 == 0.0 { 0.0 } else { (assign105230_e157657 * ((locals.var_dnm).powf(assign105230_e157657 - 1.0) * locals.var_dnm_dn8)) } } else { (assign105230_e157658 * (assign105230_e157657 * (locals.var_dnm_dn8 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign105230_e157657) as f64).is_finite() && ((assign105230_e157657) as f64).fract() == 0.0 { if assign105230_e157657 == 0.0 { 0.0 } else { (assign105230_e157657 * ((locals.var_dnm).powf(assign105230_e157657 - 1.0) * locals.var_dnm_dn9)) } } else { (assign105230_e157658 * (assign105230_e157657 * (locals.var_dnm_dn9 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign105230_e157657) as f64).is_finite() && ((assign105230_e157657) as f64).fract() == 0.0 { if assign105230_e157657 == 0.0 { 0.0 } else { (assign105230_e157657 * ((locals.var_dnm).powf(assign105230_e157657 - 1.0) * locals.var_dnm_dn10)) } } else { (assign105230_e157658 * (assign105230_e157657 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign105230_e157657) as f64).is_finite() && ((assign105230_e157657) as f64).fract() == 0.0 { if assign105230_e157657 == 0.0 { 0.0 } else { (assign105230_e157657 * ((locals.var_dnm).powf(assign105230_e157657 - 1.0) * locals.var_dnm_dn13)) } } else { (assign105230_e157658 * (assign105230_e157657 * (locals.var_dnm_dn13 / locals.var_dnm))) },)
            }
        };
        (assign105230_e157659, assign105230_e157659_d_n0, assign105230_e157659_d_n2, assign105230_e157659_d_n4, assign105230_e157659_d_n5, assign105230_e157659_d_n6, assign105230_e157659_d_n7, assign105230_e157659_d_n8, assign105230_e157659_d_n9, assign105230_e157659_d_n10, assign105230_e157659_d_n13,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign105230_e157661;
        locals.var_dnm_dn0 = assign105230_e157661_d_n0;
        locals.var_dnm_dn2 = assign105230_e157661_d_n2;
        locals.var_dnm_dn4 = assign105230_e157661_d_n4;
        locals.var_dnm_dn5 = assign105230_e157661_d_n5;
        locals.var_dnm_dn6 = assign105230_e157661_d_n6;
        locals.var_dnm_dn7 = assign105230_e157661_d_n7;
        locals.var_dnm_dn8 = assign105230_e157661_d_n8;
        locals.var_dnm_dn9 = assign105230_e157661_d_n9;
        locals.var_dnm_dn10 = assign105230_e157661_d_n10;
        locals.var_dnm_dn13 = assign105230_e157661_d_n13;

        let (assign105240_e157672, assign105240_e157672_d_n0, assign105240_e157672_d_n2, assign105240_e157672_d_n4, assign105240_e157672_d_n5, assign105240_e157672_d_n6, assign105240_e157672_d_n7, assign105240_e157672_d_n8, assign105240_e157672_d_n9, assign105240_e157672_d_n10, assign105240_e157672_d_n13,) = {
    if (((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2381 != 0.0)) {
        let assign105240_e157670: f64 = (1.0 / locals.var_dnm);
        (assign105240_e157670, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn4 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn5 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn8 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn9 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn13 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign105240_e157672;
        locals.var_dnm_dn0 = assign105240_e157672_d_n0;
        locals.var_dnm_dn2 = assign105240_e157672_d_n2;
        locals.var_dnm_dn4 = assign105240_e157672_d_n4;
        locals.var_dnm_dn5 = assign105240_e157672_d_n5;
        locals.var_dnm_dn6 = assign105240_e157672_d_n6;
        locals.var_dnm_dn7 = assign105240_e157672_d_n7;
        locals.var_dnm_dn8 = assign105240_e157672_d_n8;
        locals.var_dnm_dn9 = assign105240_e157672_d_n9;
        locals.var_dnm_dn10 = assign105240_e157672_d_n10;
        locals.var_dnm_dn13 = assign105240_e157672_d_n13;

        let (assign105250_e157685, assign105250_e157685_d_n0, assign105250_e157685_d_n2, assign105250_e157685_d_n4, assign105250_e157685_d_n5, assign105250_e157685_d_n6, assign105250_e157685_d_n7, assign105250_e157685_d_n8, assign105250_e157685_d_n9, assign105250_e157685_d_n10, assign105250_e157685_d_n13,) = {
    if (((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2381 != 0.0)) {
        let assign105250_e157681: f64 = (locals.var_tmf1 * 1e-25);
        let assign105250_e157683: f64 = (assign105250_e157681 * locals.var_dnm);
        (assign105250_e157683, (((locals.var_tmf1_dn0 * 1e-25) * locals.var_dnm) + (assign105250_e157681 * locals.var_dnm_dn0)), (((locals.var_tmf1_dn2 * 1e-25) * locals.var_dnm) + (assign105250_e157681 * locals.var_dnm_dn2)), (((locals.var_tmf1_dn4 * 1e-25) * locals.var_dnm) + (assign105250_e157681 * locals.var_dnm_dn4)), (((locals.var_tmf1_dn5 * 1e-25) * locals.var_dnm) + (assign105250_e157681 * locals.var_dnm_dn5)), (((locals.var_tmf1_dn6 * 1e-25) * locals.var_dnm) + (assign105250_e157681 * locals.var_dnm_dn6)), (((locals.var_tmf1_dn7 * 1e-25) * locals.var_dnm) + (assign105250_e157681 * locals.var_dnm_dn7)), (((locals.var_tmf1_dn8 * 1e-25) * locals.var_dnm) + (assign105250_e157681 * locals.var_dnm_dn8)), (((locals.var_tmf1_dn9 * 1e-25) * locals.var_dnm) + (assign105250_e157681 * locals.var_dnm_dn9)), (((locals.var_tmf1_dn10 * 1e-25) * locals.var_dnm) + (assign105250_e157681 * locals.var_dnm_dn10)), (((locals.var_tmf1_dn13 * 1e-25) * locals.var_dnm) + (assign105250_e157681 * locals.var_dnm_dn13)),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn13,)
    }
};
        locals.var_tmf0 = assign105250_e157685;
        locals.var_tmf0_dn0 = assign105250_e157685_d_n0;
        locals.var_tmf0_dn2 = assign105250_e157685_d_n2;
        locals.var_tmf0_dn4 = assign105250_e157685_d_n4;
        locals.var_tmf0_dn5 = assign105250_e157685_d_n5;
        locals.var_tmf0_dn6 = assign105250_e157685_d_n6;
        locals.var_tmf0_dn7 = assign105250_e157685_d_n7;
        locals.var_tmf0_dn8 = assign105250_e157685_d_n8;
        locals.var_tmf0_dn9 = assign105250_e157685_d_n9;
        locals.var_tmf0_dn10 = assign105250_e157685_d_n10;
        locals.var_tmf0_dn13 = assign105250_e157685_d_n13;

    }

    pub(super) fn stamp_transient_block_371(
        locals: &mut StampLocals,
    ) {
        let (assign105260_e157700, assign105260_e157700_d_n0, assign105260_e157700_d_n2, assign105260_e157700_d_n4, assign105260_e157700_d_n5, assign105260_e157700_d_n6, assign105260_e157700_d_n7, assign105260_e157700_d_n8, assign105260_e157700_d_n9, assign105260_e157700_d_n10, assign105260_e157700_d_n13,) = {
    if (((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2381 != 0.0)) {
        let assign105260_e157694: f64 = (1e-25 * locals.var_xmp);
        let assign105260_e157696: f64 = (assign105260_e157694 * locals.var_dnm);
        let assign105260_e157698: f64 = (assign105260_e157696 / locals.var_arg);
        (assign105260_e157698, ((((((1e-25 * locals.var_xmp_dn0) * locals.var_dnm) + (assign105260_e157694 * locals.var_dnm_dn0)) * locals.var_arg) - (assign105260_e157696 * locals.var_arg_dn0)) / (locals.var_arg * locals.var_arg)), ((((((1e-25 * locals.var_xmp_dn2) * locals.var_dnm) + (assign105260_e157694 * locals.var_dnm_dn2)) * locals.var_arg) - (assign105260_e157696 * locals.var_arg_dn2)) / (locals.var_arg * locals.var_arg)), ((((((1e-25 * locals.var_xmp_dn4) * locals.var_dnm) + (assign105260_e157694 * locals.var_dnm_dn4)) * locals.var_arg) - (assign105260_e157696 * locals.var_arg_dn4)) / (locals.var_arg * locals.var_arg)), ((((((1e-25 * locals.var_xmp_dn5) * locals.var_dnm) + (assign105260_e157694 * locals.var_dnm_dn5)) * locals.var_arg) - (assign105260_e157696 * locals.var_arg_dn5)) / (locals.var_arg * locals.var_arg)), ((((((1e-25 * locals.var_xmp_dn6) * locals.var_dnm) + (assign105260_e157694 * locals.var_dnm_dn6)) * locals.var_arg) - (assign105260_e157696 * locals.var_arg_dn6)) / (locals.var_arg * locals.var_arg)), ((((((1e-25 * locals.var_xmp_dn7) * locals.var_dnm) + (assign105260_e157694 * locals.var_dnm_dn7)) * locals.var_arg) - (assign105260_e157696 * locals.var_arg_dn7)) / (locals.var_arg * locals.var_arg)), ((((((1e-25 * locals.var_xmp_dn8) * locals.var_dnm) + (assign105260_e157694 * locals.var_dnm_dn8)) * locals.var_arg) - (assign105260_e157696 * locals.var_arg_dn8)) / (locals.var_arg * locals.var_arg)), ((((((1e-25 * locals.var_xmp_dn9) * locals.var_dnm) + (assign105260_e157694 * locals.var_dnm_dn9)) * locals.var_arg) - (assign105260_e157696 * locals.var_arg_dn9)) / (locals.var_arg * locals.var_arg)), ((((((1e-25 * locals.var_xmp_dn10) * locals.var_dnm) + (assign105260_e157694 * locals.var_dnm_dn10)) * locals.var_arg) - (assign105260_e157696 * locals.var_arg_dn10)) / (locals.var_arg * locals.var_arg)), ((((((1e-25 * locals.var_xmp_dn13) * locals.var_dnm) + (assign105260_e157694 * locals.var_dnm_dn13)) * locals.var_arg) - (assign105260_e157696 * locals.var_arg_dn13)) / (locals.var_arg * locals.var_arg)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign105260_e157700;
        locals.var_t0_dn0 = assign105260_e157700_d_n0;
        locals.var_t0_dn2 = assign105260_e157700_d_n2;
        locals.var_t0_dn4 = assign105260_e157700_d_n4;
        locals.var_t0_dn5 = assign105260_e157700_d_n5;
        locals.var_t0_dn6 = assign105260_e157700_d_n6;
        locals.var_t0_dn7 = assign105260_e157700_d_n7;
        locals.var_t0_dn8 = assign105260_e157700_d_n8;
        locals.var_t0_dn9 = assign105260_e157700_d_n9;
        locals.var_t0_dn10 = assign105260_e157700_d_n10;
        locals.var_t0_dn13 = assign105260_e157700_d_n13;

        let (assign105270_e157713, assign105270_e157713_d_n0, assign105270_e157713_d_n2, assign105270_e157713_d_n4, assign105270_e157713_d_n5, assign105270_e157713_d_n6, assign105270_e157713_d_n7, assign105270_e157713_d_n8, assign105270_e157713_d_n9, assign105270_e157713_d_n10, assign105270_e157713_d_n13,) = {
    if (((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2381 != 0.0)) {
        let assign105270_e157709: f64 = 1e-25;
        let assign105270_e157711: f64 = (assign105270_e157709 - locals.var_tmf0);
        (assign105270_e157711, (-locals.var_tmf0_dn0), (-locals.var_tmf0_dn2), (-locals.var_tmf0_dn4), (-locals.var_tmf0_dn5), (-locals.var_tmf0_dn6), (-locals.var_tmf0_dn7), (-locals.var_tmf0_dn8), (-locals.var_tmf0_dn9), (-locals.var_tmf0_dn10), (-locals.var_tmf0_dn13),)
    } else {
        (locals.var_gd, locals.var_gd_dn0, locals.var_gd_dn2, locals.var_gd_dn4, locals.var_gd_dn5, locals.var_gd_dn6, locals.var_gd_dn7, locals.var_gd_dn8, locals.var_gd_dn9, locals.var_gd_dn10, locals.var_gd_dn13,)
    }
};
        locals.var_gd = assign105270_e157713;
        locals.var_gd_dn0 = assign105270_e157713_d_n0;
        locals.var_gd_dn2 = assign105270_e157713_d_n2;
        locals.var_gd_dn4 = assign105270_e157713_d_n4;
        locals.var_gd_dn5 = assign105270_e157713_d_n5;
        locals.var_gd_dn6 = assign105270_e157713_d_n6;
        locals.var_gd_dn7 = assign105270_e157713_d_n7;
        locals.var_gd_dn8 = assign105270_e157713_d_n8;
        locals.var_gd_dn9 = assign105270_e157713_d_n9;
        locals.var_gd_dn10 = assign105270_e157713_d_n10;
        locals.var_gd_dn13 = assign105270_e157713_d_n13;

        let (assign105280_e157722, assign105280_e157722_d_n0, assign105280_e157722_d_n2, assign105280_e157722_d_n4, assign105280_e157722_d_n5, assign105280_e157722_d_n6, assign105280_e157722_d_n7, assign105280_e157722_d_n8, assign105280_e157722_d_n9, assign105280_e157722_d_n10, assign105280_e157722_d_n13,) = {
    if (((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2381 != 0.0)) {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign105280_e157722;
        locals.var_t0_dn0 = assign105280_e157722_d_n0;
        locals.var_t0_dn2 = assign105280_e157722_d_n2;
        locals.var_t0_dn4 = assign105280_e157722_d_n4;
        locals.var_t0_dn5 = assign105280_e157722_d_n5;
        locals.var_t0_dn6 = assign105280_e157722_d_n6;
        locals.var_t0_dn7 = assign105280_e157722_d_n7;
        locals.var_t0_dn8 = assign105280_e157722_d_n8;
        locals.var_t0_dn9 = assign105280_e157722_d_n9;
        locals.var_t0_dn10 = assign105280_e157722_d_n10;
        locals.var_t0_dn13 = assign105280_e157722_d_n13;

        let (assign105290_e157732, assign105290_e157732_d_n0, assign105290_e157732_d_n2, assign105290_e157732_d_n4, assign105290_e157732_d_n5, assign105290_e157732_d_n6, assign105290_e157732_d_n7, assign105290_e157732_d_n8, assign105290_e157732_d_n9, assign105290_e157732_d_n10, assign105290_e157732_d_n13,) = {
    if (((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2381 == 0.0)) {
        (locals.var_gd, locals.var_gd_dn0, locals.var_gd_dn2, locals.var_gd_dn4, locals.var_gd_dn5, locals.var_gd_dn6, locals.var_gd_dn7, locals.var_gd_dn8, locals.var_gd_dn9, locals.var_gd_dn10, locals.var_gd_dn13,)
    } else {
        (locals.var_gd, locals.var_gd_dn0, locals.var_gd_dn2, locals.var_gd_dn4, locals.var_gd_dn5, locals.var_gd_dn6, locals.var_gd_dn7, locals.var_gd_dn8, locals.var_gd_dn9, locals.var_gd_dn10, locals.var_gd_dn13,)
    }
};
        locals.var_gd = assign105290_e157732;
        locals.var_gd_dn0 = assign105290_e157732_d_n0;
        locals.var_gd_dn2 = assign105290_e157732_d_n2;
        locals.var_gd_dn4 = assign105290_e157732_d_n4;
        locals.var_gd_dn5 = assign105290_e157732_d_n5;
        locals.var_gd_dn6 = assign105290_e157732_d_n6;
        locals.var_gd_dn7 = assign105290_e157732_d_n7;
        locals.var_gd_dn8 = assign105290_e157732_d_n8;
        locals.var_gd_dn9 = assign105290_e157732_d_n9;
        locals.var_gd_dn10 = assign105290_e157732_d_n10;
        locals.var_gd_dn13 = assign105290_e157732_d_n13;

        let (assign105300_e157742, assign105300_e157742_d_n0, assign105300_e157742_d_n2, assign105300_e157742_d_n4, assign105300_e157742_d_n5, assign105300_e157742_d_n6, assign105300_e157742_d_n7, assign105300_e157742_d_n8, assign105300_e157742_d_n9, assign105300_e157742_d_n10, assign105300_e157742_d_n13,) = {
    if (((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2381 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign105300_e157742;
        locals.var_t0_dn0 = assign105300_e157742_d_n0;
        locals.var_t0_dn2 = assign105300_e157742_d_n2;
        locals.var_t0_dn4 = assign105300_e157742_d_n4;
        locals.var_t0_dn5 = assign105300_e157742_d_n5;
        locals.var_t0_dn6 = assign105300_e157742_d_n6;
        locals.var_t0_dn7 = assign105300_e157742_d_n7;
        locals.var_t0_dn8 = assign105300_e157742_d_n8;
        locals.var_t0_dn9 = assign105300_e157742_d_n9;
        locals.var_t0_dn10 = assign105300_e157742_d_n10;
        locals.var_t0_dn13 = assign105300_e157742_d_n13;

        let (assign105310_e157751, assign105310_e157751_d_n0, assign105310_e157751_d_n2, assign105310_e157751_d_n4, assign105310_e157751_d_n5, assign105310_e157751_d_n6, assign105310_e157751_d_n7, assign105310_e157751_d_n8, assign105310_e157751_d_n9, assign105310_e157751_d_n10, assign105310_e157751_d_n13,) = {
    if ((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) {
        let assign105310_e157749: f64 = (1.0 / locals.var_gd);
        (assign105310_e157749, (-(locals.var_gd_dn0 / (locals.var_gd * locals.var_gd))), (-(locals.var_gd_dn2 / (locals.var_gd * locals.var_gd))), (-(locals.var_gd_dn4 / (locals.var_gd * locals.var_gd))), (-(locals.var_gd_dn5 / (locals.var_gd * locals.var_gd))), (-(locals.var_gd_dn6 / (locals.var_gd * locals.var_gd))), (-(locals.var_gd_dn7 / (locals.var_gd * locals.var_gd))), (-(locals.var_gd_dn8 / (locals.var_gd * locals.var_gd))), (-(locals.var_gd_dn9 / (locals.var_gd * locals.var_gd))), (-(locals.var_gd_dn10 / (locals.var_gd * locals.var_gd))), (-(locals.var_gd_dn13 / (locals.var_gd * locals.var_gd))),)
    } else {
        (locals.var_rdd, locals.var_rdd_dn0, locals.var_rdd_dn2, locals.var_rdd_dn4, locals.var_rdd_dn5, locals.var_rdd_dn6, locals.var_rdd_dn7, locals.var_rdd_dn8, locals.var_rdd_dn9, locals.var_rdd_dn10, locals.var_rdd_dn13,)
    }
};
        locals.var_rdd = assign105310_e157751;
        locals.var_rdd_dn0 = assign105310_e157751_d_n0;
        locals.var_rdd_dn2 = assign105310_e157751_d_n2;
        locals.var_rdd_dn4 = assign105310_e157751_d_n4;
        locals.var_rdd_dn5 = assign105310_e157751_d_n5;
        locals.var_rdd_dn6 = assign105310_e157751_d_n6;
        locals.var_rdd_dn7 = assign105310_e157751_d_n7;
        locals.var_rdd_dn8 = assign105310_e157751_d_n8;
        locals.var_rdd_dn9 = assign105310_e157751_d_n9;
        locals.var_rdd_dn10 = assign105310_e157751_d_n10;
        locals.var_rdd_dn13 = assign105310_e157751_d_n13;

        let (assign105320_e157760, assign105320_e157760_d_n0, assign105320_e157760_d_n2, assign105320_e157760_d_n4, assign105320_e157760_d_n5, assign105320_e157760_d_n6, assign105320_e157760_d_n7, assign105320_e157760_d_n8, assign105320_e157760_d_n9, assign105320_e157760_d_n10, assign105320_e157760_d_n13,) = {
    if ((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) {
        let assign105320_e157758: f64 = (locals.var_rdd / locals.var_weffld_nf);
        (assign105320_e157758, (locals.var_rdd_dn0 / locals.var_weffld_nf), (locals.var_rdd_dn2 / locals.var_weffld_nf), (locals.var_rdd_dn4 / locals.var_weffld_nf), (locals.var_rdd_dn5 / locals.var_weffld_nf), (locals.var_rdd_dn6 / locals.var_weffld_nf), (locals.var_rdd_dn7 / locals.var_weffld_nf), (locals.var_rdd_dn8 / locals.var_weffld_nf), (locals.var_rdd_dn9 / locals.var_weffld_nf), (locals.var_rdd_dn10 / locals.var_weffld_nf), (locals.var_rdd_dn13 / locals.var_weffld_nf),)
    } else {
        (locals.var_rdd, locals.var_rdd_dn0, locals.var_rdd_dn2, locals.var_rdd_dn4, locals.var_rdd_dn5, locals.var_rdd_dn6, locals.var_rdd_dn7, locals.var_rdd_dn8, locals.var_rdd_dn9, locals.var_rdd_dn10, locals.var_rdd_dn13,)
    }
};
        locals.var_rdd = assign105320_e157760;
        locals.var_rdd_dn0 = assign105320_e157760_d_n0;
        locals.var_rdd_dn2 = assign105320_e157760_d_n2;
        locals.var_rdd_dn4 = assign105320_e157760_d_n4;
        locals.var_rdd_dn5 = assign105320_e157760_d_n5;
        locals.var_rdd_dn6 = assign105320_e157760_d_n6;
        locals.var_rdd_dn7 = assign105320_e157760_d_n7;
        locals.var_rdd_dn8 = assign105320_e157760_d_n8;
        locals.var_rdd_dn9 = assign105320_e157760_d_n9;
        locals.var_rdd_dn10 = assign105320_e157760_d_n10;
        locals.var_rdd_dn13 = assign105320_e157760_d_n13;

        let assign105330_e157764: f64 = (1000000.0 - 1000.0);
        let assign105330_e157769: f64 = if ((locals.var_rdd > assign105330_e157764) && (1000.0 >= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard2387 = assign105330_e157769;

        let (assign105340_e157782, assign105340_e157782_d_n0, assign105340_e157782_d_n2, assign105340_e157782_d_n4, assign105340_e157782_d_n5, assign105340_e157782_d_n6, assign105340_e157782_d_n7, assign105340_e157782_d_n8, assign105340_e157782_d_n9, assign105340_e157782_d_n10, assign105340_e157782_d_n13,) = {
    if (((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2387 != 0.0)) {
        let assign105340_e157778: f64 = (locals.var_rdd - 1000000.0);
        let assign105340_e157780: f64 = (assign105340_e157778 + 1000.0);
        (assign105340_e157780, locals.var_rdd_dn0, locals.var_rdd_dn2, locals.var_rdd_dn4, locals.var_rdd_dn5, locals.var_rdd_dn6, locals.var_rdd_dn7, locals.var_rdd_dn8, locals.var_rdd_dn9, locals.var_rdd_dn10, locals.var_rdd_dn13,)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13,)
    }
};
        locals.var_tmf1 = assign105340_e157782;
        locals.var_tmf1_dn0 = assign105340_e157782_d_n0;
        locals.var_tmf1_dn2 = assign105340_e157782_d_n2;
        locals.var_tmf1_dn4 = assign105340_e157782_d_n4;
        locals.var_tmf1_dn5 = assign105340_e157782_d_n5;
        locals.var_tmf1_dn6 = assign105340_e157782_d_n6;
        locals.var_tmf1_dn7 = assign105340_e157782_d_n7;
        locals.var_tmf1_dn8 = assign105340_e157782_d_n8;
        locals.var_tmf1_dn9 = assign105340_e157782_d_n9;
        locals.var_tmf1_dn10 = assign105340_e157782_d_n10;
        locals.var_tmf1_dn13 = assign105340_e157782_d_n13;

        let (assign105350_e157793, assign105350_e157793_d_n0, assign105350_e157793_d_n2, assign105350_e157793_d_n4, assign105350_e157793_d_n5, assign105350_e157793_d_n6, assign105350_e157793_d_n7, assign105350_e157793_d_n8, assign105350_e157793_d_n9, assign105350_e157793_d_n10, assign105350_e157793_d_n13,) = {
    if (((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2387 != 0.0)) {
        let assign105350_e157791: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign105350_e157791, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)), ((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)), ((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn13 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn13)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn4, locals.var_x2_dn5, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn8, locals.var_x2_dn9, locals.var_x2_dn10, locals.var_x2_dn13,)
    }
};
        locals.var_x2 = assign105350_e157793;
        locals.var_x2_dn0 = assign105350_e157793_d_n0;
        locals.var_x2_dn2 = assign105350_e157793_d_n2;
        locals.var_x2_dn4 = assign105350_e157793_d_n4;
        locals.var_x2_dn5 = assign105350_e157793_d_n5;
        locals.var_x2_dn6 = assign105350_e157793_d_n6;
        locals.var_x2_dn7 = assign105350_e157793_d_n7;
        locals.var_x2_dn8 = assign105350_e157793_d_n8;
        locals.var_x2_dn9 = assign105350_e157793_d_n9;
        locals.var_x2_dn10 = assign105350_e157793_d_n10;
        locals.var_x2_dn13 = assign105350_e157793_d_n13;

        let (assign105360_e157804, assign105360_e157804_d_n0, assign105360_e157804_d_n2, assign105360_e157804_d_n4, assign105360_e157804_d_n5, assign105360_e157804_d_n6, assign105360_e157804_d_n7, assign105360_e157804_d_n8, assign105360_e157804_d_n9, assign105360_e157804_d_n10, assign105360_e157804_d_n13,) = {
    if (((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2387 != 0.0)) {
        let assign105360_e157802: f64 = (1000.0 * 1000.0);
        (assign105360_e157802, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn4, locals.var_xmax2_dn5, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn8, locals.var_xmax2_dn9, locals.var_xmax2_dn10, locals.var_xmax2_dn13,)
    }
};
        locals.var_xmax2 = assign105360_e157804;
        locals.var_xmax2_dn0 = assign105360_e157804_d_n0;
        locals.var_xmax2_dn2 = assign105360_e157804_d_n2;
        locals.var_xmax2_dn4 = assign105360_e157804_d_n4;
        locals.var_xmax2_dn5 = assign105360_e157804_d_n5;
        locals.var_xmax2_dn6 = assign105360_e157804_d_n6;
        locals.var_xmax2_dn7 = assign105360_e157804_d_n7;
        locals.var_xmax2_dn8 = assign105360_e157804_d_n8;
        locals.var_xmax2_dn9 = assign105360_e157804_d_n9;
        locals.var_xmax2_dn10 = assign105360_e157804_d_n10;
        locals.var_xmax2_dn13 = assign105360_e157804_d_n13;

        let (assign105370_e157813, assign105370_e157813_d_n0, assign105370_e157813_d_n2, assign105370_e157813_d_n4, assign105370_e157813_d_n5, assign105370_e157813_d_n6, assign105370_e157813_d_n7, assign105370_e157813_d_n8, assign105370_e157813_d_n9, assign105370_e157813_d_n10, assign105370_e157813_d_n13,) = {
    if (((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2387 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
        locals.var_xp = assign105370_e157813;
        locals.var_xp_dn0 = assign105370_e157813_d_n0;
        locals.var_xp_dn2 = assign105370_e157813_d_n2;
        locals.var_xp_dn4 = assign105370_e157813_d_n4;
        locals.var_xp_dn5 = assign105370_e157813_d_n5;
        locals.var_xp_dn6 = assign105370_e157813_d_n6;
        locals.var_xp_dn7 = assign105370_e157813_d_n7;
        locals.var_xp_dn8 = assign105370_e157813_d_n8;
        locals.var_xp_dn9 = assign105370_e157813_d_n9;
        locals.var_xp_dn10 = assign105370_e157813_d_n10;
        locals.var_xp_dn13 = assign105370_e157813_d_n13;

        let (assign105380_e157822, assign105380_e157822_d_n0, assign105380_e157822_d_n2, assign105380_e157822_d_n4, assign105380_e157822_d_n5, assign105380_e157822_d_n6, assign105380_e157822_d_n7, assign105380_e157822_d_n8, assign105380_e157822_d_n9, assign105380_e157822_d_n10, assign105380_e157822_d_n13,) = {
    if (((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2387 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
        locals.var_xmp = assign105380_e157822;
        locals.var_xmp_dn0 = assign105380_e157822_d_n0;
        locals.var_xmp_dn2 = assign105380_e157822_d_n2;
        locals.var_xmp_dn4 = assign105380_e157822_d_n4;
        locals.var_xmp_dn5 = assign105380_e157822_d_n5;
        locals.var_xmp_dn6 = assign105380_e157822_d_n6;
        locals.var_xmp_dn7 = assign105380_e157822_d_n7;
        locals.var_xmp_dn8 = assign105380_e157822_d_n8;
        locals.var_xmp_dn9 = assign105380_e157822_d_n9;
        locals.var_xmp_dn10 = assign105380_e157822_d_n10;
        locals.var_xmp_dn13 = assign105380_e157822_d_n13;

        let (assign105390_e157831,) = {
    if (((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2387 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign105390_e157831;

        let (assign105400_e157840,) = {
    if (((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2387 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign105400_e157840;

        let (assign105410_e157849, assign105410_e157849_d_n0, assign105410_e157849_d_n2, assign105410_e157849_d_n4, assign105410_e157849_d_n5, assign105410_e157849_d_n6, assign105410_e157849_d_n7, assign105410_e157849_d_n8, assign105410_e157849_d_n9, assign105410_e157849_d_n10, assign105410_e157849_d_n13,) = {
    if (((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2387 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    }
};
        locals.var_arg = assign105410_e157849;
        locals.var_arg_dn0 = assign105410_e157849_d_n0;
        locals.var_arg_dn2 = assign105410_e157849_d_n2;
        locals.var_arg_dn4 = assign105410_e157849_d_n4;
        locals.var_arg_dn5 = assign105410_e157849_d_n5;
        locals.var_arg_dn6 = assign105410_e157849_d_n6;
        locals.var_arg_dn7 = assign105410_e157849_d_n7;
        locals.var_arg_dn8 = assign105410_e157849_d_n8;
        locals.var_arg_dn9 = assign105410_e157849_d_n9;
        locals.var_arg_dn10 = assign105410_e157849_d_n10;
        locals.var_arg_dn13 = assign105410_e157849_d_n13;

        let (assign105420_e157858, assign105420_e157858_d_n0, assign105420_e157858_d_n2, assign105420_e157858_d_n4, assign105420_e157858_d_n5, assign105420_e157858_d_n6, assign105420_e157858_d_n7, assign105420_e157858_d_n8, assign105420_e157858_d_n9, assign105420_e157858_d_n10, assign105420_e157858_d_n13,) = {
    if (((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2387 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign105420_e157858;
        locals.var_dnm_dn0 = assign105420_e157858_d_n0;
        locals.var_dnm_dn2 = assign105420_e157858_d_n2;
        locals.var_dnm_dn4 = assign105420_e157858_d_n4;
        locals.var_dnm_dn5 = assign105420_e157858_d_n5;
        locals.var_dnm_dn6 = assign105420_e157858_d_n6;
        locals.var_dnm_dn7 = assign105420_e157858_d_n7;
        locals.var_dnm_dn8 = assign105420_e157858_d_n8;
        locals.var_dnm_dn9 = assign105420_e157858_d_n9;
        locals.var_dnm_dn10 = assign105420_e157858_d_n10;
        locals.var_dnm_dn13 = assign105420_e157858_d_n13;

        let (assign105430_e157869, assign105430_e157869_d_n0, assign105430_e157869_d_n2, assign105430_e157869_d_n4, assign105430_e157869_d_n5, assign105430_e157869_d_n6, assign105430_e157869_d_n7, assign105430_e157869_d_n8, assign105430_e157869_d_n9, assign105430_e157869_d_n10, assign105430_e157869_d_n13,) = {
    if (((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2387 != 0.0)) {
        let assign105430_e157867: f64 = (locals.var_xp * locals.var_x2);
        (assign105430_e157867, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn13 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn13)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
        locals.var_xp = assign105430_e157869;
        locals.var_xp_dn0 = assign105430_e157869_d_n0;
        locals.var_xp_dn2 = assign105430_e157869_d_n2;
        locals.var_xp_dn4 = assign105430_e157869_d_n4;
        locals.var_xp_dn5 = assign105430_e157869_d_n5;
        locals.var_xp_dn6 = assign105430_e157869_d_n6;
        locals.var_xp_dn7 = assign105430_e157869_d_n7;
        locals.var_xp_dn8 = assign105430_e157869_d_n8;
        locals.var_xp_dn9 = assign105430_e157869_d_n9;
        locals.var_xp_dn10 = assign105430_e157869_d_n10;
        locals.var_xp_dn13 = assign105430_e157869_d_n13;

        let (assign105440_e157880, assign105440_e157880_d_n0, assign105440_e157880_d_n2, assign105440_e157880_d_n4, assign105440_e157880_d_n5, assign105440_e157880_d_n6, assign105440_e157880_d_n7, assign105440_e157880_d_n8, assign105440_e157880_d_n9, assign105440_e157880_d_n10, assign105440_e157880_d_n13,) = {
    if (((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2387 != 0.0)) {
        let assign105440_e157878: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign105440_e157878, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn13 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn13)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
        locals.var_xmp = assign105440_e157880;
        locals.var_xmp_dn0 = assign105440_e157880_d_n0;
        locals.var_xmp_dn2 = assign105440_e157880_d_n2;
        locals.var_xmp_dn4 = assign105440_e157880_d_n4;
        locals.var_xmp_dn5 = assign105440_e157880_d_n5;
        locals.var_xmp_dn6 = assign105440_e157880_d_n6;
        locals.var_xmp_dn7 = assign105440_e157880_d_n7;
        locals.var_xmp_dn8 = assign105440_e157880_d_n8;
        locals.var_xmp_dn9 = assign105440_e157880_d_n9;
        locals.var_xmp_dn10 = assign105440_e157880_d_n10;
        locals.var_xmp_dn13 = assign105440_e157880_d_n13;

        let (assign105450_e157891, assign105450_e157891_d_n0, assign105450_e157891_d_n2, assign105450_e157891_d_n4, assign105450_e157891_d_n5, assign105450_e157891_d_n6, assign105450_e157891_d_n7, assign105450_e157891_d_n8, assign105450_e157891_d_n9, assign105450_e157891_d_n10, assign105450_e157891_d_n13,) = {
    if (((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2387 != 0.0)) {
        let assign105450_e157889: f64 = (locals.var_xp * locals.var_x2);
        (assign105450_e157889, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn13 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn13)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
        locals.var_xp = assign105450_e157891;
        locals.var_xp_dn0 = assign105450_e157891_d_n0;
        locals.var_xp_dn2 = assign105450_e157891_d_n2;
        locals.var_xp_dn4 = assign105450_e157891_d_n4;
        locals.var_xp_dn5 = assign105450_e157891_d_n5;
        locals.var_xp_dn6 = assign105450_e157891_d_n6;
        locals.var_xp_dn7 = assign105450_e157891_d_n7;
        locals.var_xp_dn8 = assign105450_e157891_d_n8;
        locals.var_xp_dn9 = assign105450_e157891_d_n9;
        locals.var_xp_dn10 = assign105450_e157891_d_n10;
        locals.var_xp_dn13 = assign105450_e157891_d_n13;

        let (assign105460_e157902, assign105460_e157902_d_n0, assign105460_e157902_d_n2, assign105460_e157902_d_n4, assign105460_e157902_d_n5, assign105460_e157902_d_n6, assign105460_e157902_d_n7, assign105460_e157902_d_n8, assign105460_e157902_d_n9, assign105460_e157902_d_n10, assign105460_e157902_d_n13,) = {
    if (((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2387 != 0.0)) {
        let assign105460_e157900: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign105460_e157900, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn13 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn13)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
        locals.var_xmp = assign105460_e157902;
        locals.var_xmp_dn0 = assign105460_e157902_d_n0;
        locals.var_xmp_dn2 = assign105460_e157902_d_n2;
        locals.var_xmp_dn4 = assign105460_e157902_d_n4;
        locals.var_xmp_dn5 = assign105460_e157902_d_n5;
        locals.var_xmp_dn6 = assign105460_e157902_d_n6;
        locals.var_xmp_dn7 = assign105460_e157902_d_n7;
        locals.var_xmp_dn8 = assign105460_e157902_d_n8;
        locals.var_xmp_dn9 = assign105460_e157902_d_n9;
        locals.var_xmp_dn10 = assign105460_e157902_d_n10;
        locals.var_xmp_dn13 = assign105460_e157902_d_n13;

        let (assign105470_e157913, assign105470_e157913_d_n0, assign105470_e157913_d_n2, assign105470_e157913_d_n4, assign105470_e157913_d_n5, assign105470_e157913_d_n6, assign105470_e157913_d_n7, assign105470_e157913_d_n8, assign105470_e157913_d_n9, assign105470_e157913_d_n10, assign105470_e157913_d_n13,) = {
    if (((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2387 != 0.0)) {
        let assign105470_e157911: f64 = (locals.var_xp + locals.var_xmp);
        (assign105470_e157911, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn4 + locals.var_xmp_dn4), (locals.var_xp_dn5 + locals.var_xmp_dn5), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn8 + locals.var_xmp_dn8), (locals.var_xp_dn9 + locals.var_xmp_dn9), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn13 + locals.var_xmp_dn13),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    }
};
        locals.var_arg = assign105470_e157913;
        locals.var_arg_dn0 = assign105470_e157913_d_n0;
        locals.var_arg_dn2 = assign105470_e157913_d_n2;
        locals.var_arg_dn4 = assign105470_e157913_d_n4;
        locals.var_arg_dn5 = assign105470_e157913_d_n5;
        locals.var_arg_dn6 = assign105470_e157913_d_n6;
        locals.var_arg_dn7 = assign105470_e157913_d_n7;
        locals.var_arg_dn8 = assign105470_e157913_d_n8;
        locals.var_arg_dn9 = assign105470_e157913_d_n9;
        locals.var_arg_dn10 = assign105470_e157913_d_n10;
        locals.var_arg_dn13 = assign105470_e157913_d_n13;

        let (assign105480_e157922, assign105480_e157922_d_n0, assign105480_e157922_d_n2, assign105480_e157922_d_n4, assign105480_e157922_d_n5, assign105480_e157922_d_n6, assign105480_e157922_d_n7, assign105480_e157922_d_n8, assign105480_e157922_d_n9, assign105480_e157922_d_n10, assign105480_e157922_d_n13,) = {
    if (((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2387 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign105480_e157922;
        locals.var_dnm_dn0 = assign105480_e157922_d_n0;
        locals.var_dnm_dn2 = assign105480_e157922_d_n2;
        locals.var_dnm_dn4 = assign105480_e157922_d_n4;
        locals.var_dnm_dn5 = assign105480_e157922_d_n5;
        locals.var_dnm_dn6 = assign105480_e157922_d_n6;
        locals.var_dnm_dn7 = assign105480_e157922_d_n7;
        locals.var_dnm_dn8 = assign105480_e157922_d_n8;
        locals.var_dnm_dn9 = assign105480_e157922_d_n9;
        locals.var_dnm_dn10 = assign105480_e157922_d_n10;
        locals.var_dnm_dn13 = assign105480_e157922_d_n13;

        let assign105490_e157937: f64 = if ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard2388 = assign105490_e157937;

        let assign105500_e157940: f64 = if 2.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard2389 = assign105500_e157940;

        let (assign105510_e157953,) = {
    if (((((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2387 != 0.0)) && (locals.var_guard2388 != 0.0)) && (locals.var_guard2389 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign105510_e157953;

        let assign105520_e157956: f64 = if 2.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard2390 = assign105520_e157956;

        let (assign105530_e157972,) = {
    if ((((((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2387 != 0.0)) && (locals.var_guard2388 != 0.0)) && (locals.var_guard2389 == 0.0)) && (locals.var_guard2390 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign105530_e157972;

        let assign105540_e157975: f64 = if 2.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard2391 = assign105540_e157975;

        let (assign105550_e157994,) = {
    if (((((((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2387 != 0.0)) && (locals.var_guard2388 != 0.0)) && (locals.var_guard2389 == 0.0)) && (locals.var_guard2390 == 0.0)) && (locals.var_guard2391 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign105550_e157994;

        let assign105560_e157997: f64 = if 2.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard2392 = assign105560_e157997;

        let (assign105570_e158019,) = {
    if ((((((((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2387 != 0.0)) && (locals.var_guard2388 != 0.0)) && (locals.var_guard2389 == 0.0)) && (locals.var_guard2390 == 0.0)) && (locals.var_guard2391 == 0.0)) && (locals.var_guard2392 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign105570_e158019;

        let (assign105580_e158030,) = {
    if ((((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2387 != 0.0)) && (locals.var_guard2388 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign105580_e158030;

    }

    pub(super) fn stamp_transient_block_372(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let mut assign105590_loop_guard: usize = 0;
        while {
            let assign105590_cond_e158042: f64 = if (((((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2387 != 0.0)) && (locals.var_guard2388 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign105590_cond_e158042 != 0.0
        } {
            assign105590_loop_guard += 1;
            assert!(assign105590_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign105590_body0_e158054, assign105590_body0_e158054_d_n0, assign105590_body0_e158054_d_n2, assign105590_body0_e158054_d_n4, assign105590_body0_e158054_d_n5, assign105590_body0_e158054_d_n6, assign105590_body0_e158054_d_n7, assign105590_body0_e158054_d_n8, assign105590_body0_e158054_d_n9, assign105590_body0_e158054_d_n10, assign105590_body0_e158054_d_n13,) = {
    if ((((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2387 != 0.0)) && (locals.var_guard2388 != 0.0)) {
        let assign105590_body0_e158052: f64 = (locals.var_dnm).sqrt();
        (assign105590_body0_e158052, (locals.var_dnm_dn0 / (2.0 * assign105590_body0_e158052)), (locals.var_dnm_dn2 / (2.0 * assign105590_body0_e158052)), (locals.var_dnm_dn4 / (2.0 * assign105590_body0_e158052)), (locals.var_dnm_dn5 / (2.0 * assign105590_body0_e158052)), (locals.var_dnm_dn6 / (2.0 * assign105590_body0_e158052)), (locals.var_dnm_dn7 / (2.0 * assign105590_body0_e158052)), (locals.var_dnm_dn8 / (2.0 * assign105590_body0_e158052)), (locals.var_dnm_dn9 / (2.0 * assign105590_body0_e158052)), (locals.var_dnm_dn10 / (2.0 * assign105590_body0_e158052)), (locals.var_dnm_dn13 / (2.0 * assign105590_body0_e158052)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
            locals.var_dnm = assign105590_body0_e158054;
            locals.var_dnm_dn0 = assign105590_body0_e158054_d_n0;
            locals.var_dnm_dn2 = assign105590_body0_e158054_d_n2;
            locals.var_dnm_dn4 = assign105590_body0_e158054_d_n4;
            locals.var_dnm_dn5 = assign105590_body0_e158054_d_n5;
            locals.var_dnm_dn6 = assign105590_body0_e158054_d_n6;
            locals.var_dnm_dn7 = assign105590_body0_e158054_d_n7;
            locals.var_dnm_dn8 = assign105590_body0_e158054_d_n8;
            locals.var_dnm_dn9 = assign105590_body0_e158054_d_n9;
            locals.var_dnm_dn10 = assign105590_body0_e158054_d_n10;
            locals.var_dnm_dn13 = assign105590_body0_e158054_d_n13;
            let (assign105590_body1_e158067,) = {
    if ((((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2387 != 0.0)) && (locals.var_guard2388 != 0.0)) {
        let assign105590_body1_e158065: f64 = (locals.var_m0 + 1.0);
        (assign105590_body1_e158065,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign105590_body1_e158067;
        }

        let (assign105600_e158090, assign105600_e158090_d_n0, assign105600_e158090_d_n2, assign105600_e158090_d_n4, assign105600_e158090_d_n5, assign105600_e158090_d_n6, assign105600_e158090_d_n7, assign105600_e158090_d_n8, assign105600_e158090_d_n9, assign105600_e158090_d_n10, assign105600_e158090_d_n13,) = {
    if ((((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2387 != 0.0)) && (locals.var_guard2388 == 0.0)) {
        let (assign105600_e158088, assign105600_e158088_d_n0, assign105600_e158088_d_n2, assign105600_e158088_d_n4, assign105600_e158088_d_n5, assign105600_e158088_d_n6, assign105600_e158088_d_n7, assign105600_e158088_d_n8, assign105600_e158088_d_n9, assign105600_e158088_d_n10, assign105600_e158088_d_n13,) = {
            if (locals.var_dnm == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign105600_e158085: f64 = (2.0 * 2.0);
                let assign105600_e158086: f64 = (1.0 / assign105600_e158085);
                let assign105600_e158087: f64 = (locals.var_dnm).powf(assign105600_e158086);
                (assign105600_e158087, if 0.0 == 0.0 && ((assign105600_e158086) as f64).is_finite() && ((assign105600_e158086) as f64).fract() == 0.0 { if assign105600_e158086 == 0.0 { 0.0 } else { (assign105600_e158086 * ((locals.var_dnm).powf(assign105600_e158086 - 1.0) * locals.var_dnm_dn0)) } } else { (assign105600_e158087 * (assign105600_e158086 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign105600_e158086) as f64).is_finite() && ((assign105600_e158086) as f64).fract() == 0.0 { if assign105600_e158086 == 0.0 { 0.0 } else { (assign105600_e158086 * ((locals.var_dnm).powf(assign105600_e158086 - 1.0) * locals.var_dnm_dn2)) } } else { (assign105600_e158087 * (assign105600_e158086 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign105600_e158086) as f64).is_finite() && ((assign105600_e158086) as f64).fract() == 0.0 { if assign105600_e158086 == 0.0 { 0.0 } else { (assign105600_e158086 * ((locals.var_dnm).powf(assign105600_e158086 - 1.0) * locals.var_dnm_dn4)) } } else { (assign105600_e158087 * (assign105600_e158086 * (locals.var_dnm_dn4 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign105600_e158086) as f64).is_finite() && ((assign105600_e158086) as f64).fract() == 0.0 { if assign105600_e158086 == 0.0 { 0.0 } else { (assign105600_e158086 * ((locals.var_dnm).powf(assign105600_e158086 - 1.0) * locals.var_dnm_dn5)) } } else { (assign105600_e158087 * (assign105600_e158086 * (locals.var_dnm_dn5 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign105600_e158086) as f64).is_finite() && ((assign105600_e158086) as f64).fract() == 0.0 { if assign105600_e158086 == 0.0 { 0.0 } else { (assign105600_e158086 * ((locals.var_dnm).powf(assign105600_e158086 - 1.0) * locals.var_dnm_dn6)) } } else { (assign105600_e158087 * (assign105600_e158086 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign105600_e158086) as f64).is_finite() && ((assign105600_e158086) as f64).fract() == 0.0 { if assign105600_e158086 == 0.0 { 0.0 } else { (assign105600_e158086 * ((locals.var_dnm).powf(assign105600_e158086 - 1.0) * locals.var_dnm_dn7)) } } else { (assign105600_e158087 * (assign105600_e158086 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign105600_e158086) as f64).is_finite() && ((assign105600_e158086) as f64).fract() == 0.0 { if assign105600_e158086 == 0.0 { 0.0 } else { (assign105600_e158086 * ((locals.var_dnm).powf(assign105600_e158086 - 1.0) * locals.var_dnm_dn8)) } } else { (assign105600_e158087 * (assign105600_e158086 * (locals.var_dnm_dn8 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign105600_e158086) as f64).is_finite() && ((assign105600_e158086) as f64).fract() == 0.0 { if assign105600_e158086 == 0.0 { 0.0 } else { (assign105600_e158086 * ((locals.var_dnm).powf(assign105600_e158086 - 1.0) * locals.var_dnm_dn9)) } } else { (assign105600_e158087 * (assign105600_e158086 * (locals.var_dnm_dn9 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign105600_e158086) as f64).is_finite() && ((assign105600_e158086) as f64).fract() == 0.0 { if assign105600_e158086 == 0.0 { 0.0 } else { (assign105600_e158086 * ((locals.var_dnm).powf(assign105600_e158086 - 1.0) * locals.var_dnm_dn10)) } } else { (assign105600_e158087 * (assign105600_e158086 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign105600_e158086) as f64).is_finite() && ((assign105600_e158086) as f64).fract() == 0.0 { if assign105600_e158086 == 0.0 { 0.0 } else { (assign105600_e158086 * ((locals.var_dnm).powf(assign105600_e158086 - 1.0) * locals.var_dnm_dn13)) } } else { (assign105600_e158087 * (assign105600_e158086 * (locals.var_dnm_dn13 / locals.var_dnm))) },)
            }
        };
        (assign105600_e158088, assign105600_e158088_d_n0, assign105600_e158088_d_n2, assign105600_e158088_d_n4, assign105600_e158088_d_n5, assign105600_e158088_d_n6, assign105600_e158088_d_n7, assign105600_e158088_d_n8, assign105600_e158088_d_n9, assign105600_e158088_d_n10, assign105600_e158088_d_n13,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign105600_e158090;
        locals.var_dnm_dn0 = assign105600_e158090_d_n0;
        locals.var_dnm_dn2 = assign105600_e158090_d_n2;
        locals.var_dnm_dn4 = assign105600_e158090_d_n4;
        locals.var_dnm_dn5 = assign105600_e158090_d_n5;
        locals.var_dnm_dn6 = assign105600_e158090_d_n6;
        locals.var_dnm_dn7 = assign105600_e158090_d_n7;
        locals.var_dnm_dn8 = assign105600_e158090_d_n8;
        locals.var_dnm_dn9 = assign105600_e158090_d_n9;
        locals.var_dnm_dn10 = assign105600_e158090_d_n10;
        locals.var_dnm_dn13 = assign105600_e158090_d_n13;

        let (assign105610_e158101, assign105610_e158101_d_n0, assign105610_e158101_d_n2, assign105610_e158101_d_n4, assign105610_e158101_d_n5, assign105610_e158101_d_n6, assign105610_e158101_d_n7, assign105610_e158101_d_n8, assign105610_e158101_d_n9, assign105610_e158101_d_n10, assign105610_e158101_d_n13,) = {
    if (((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2387 != 0.0)) {
        let assign105610_e158099: f64 = (1.0 / locals.var_dnm);
        (assign105610_e158099, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn4 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn5 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn8 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn9 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn13 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign105610_e158101;
        locals.var_dnm_dn0 = assign105610_e158101_d_n0;
        locals.var_dnm_dn2 = assign105610_e158101_d_n2;
        locals.var_dnm_dn4 = assign105610_e158101_d_n4;
        locals.var_dnm_dn5 = assign105610_e158101_d_n5;
        locals.var_dnm_dn6 = assign105610_e158101_d_n6;
        locals.var_dnm_dn7 = assign105610_e158101_d_n7;
        locals.var_dnm_dn8 = assign105610_e158101_d_n8;
        locals.var_dnm_dn9 = assign105610_e158101_d_n9;
        locals.var_dnm_dn10 = assign105610_e158101_d_n10;
        locals.var_dnm_dn13 = assign105610_e158101_d_n13;

        let (assign105620_e158114, assign105620_e158114_d_n0, assign105620_e158114_d_n2, assign105620_e158114_d_n4, assign105620_e158114_d_n5, assign105620_e158114_d_n6, assign105620_e158114_d_n7, assign105620_e158114_d_n8, assign105620_e158114_d_n9, assign105620_e158114_d_n10, assign105620_e158114_d_n13,) = {
    if (((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2387 != 0.0)) {
        let assign105620_e158110: f64 = (locals.var_tmf1 * 1000.0);
        let assign105620_e158112: f64 = (assign105620_e158110 * locals.var_dnm);
        (assign105620_e158112, (((locals.var_tmf1_dn0 * 1000.0) * locals.var_dnm) + (assign105620_e158110 * locals.var_dnm_dn0)), (((locals.var_tmf1_dn2 * 1000.0) * locals.var_dnm) + (assign105620_e158110 * locals.var_dnm_dn2)), (((locals.var_tmf1_dn4 * 1000.0) * locals.var_dnm) + (assign105620_e158110 * locals.var_dnm_dn4)), (((locals.var_tmf1_dn5 * 1000.0) * locals.var_dnm) + (assign105620_e158110 * locals.var_dnm_dn5)), (((locals.var_tmf1_dn6 * 1000.0) * locals.var_dnm) + (assign105620_e158110 * locals.var_dnm_dn6)), (((locals.var_tmf1_dn7 * 1000.0) * locals.var_dnm) + (assign105620_e158110 * locals.var_dnm_dn7)), (((locals.var_tmf1_dn8 * 1000.0) * locals.var_dnm) + (assign105620_e158110 * locals.var_dnm_dn8)), (((locals.var_tmf1_dn9 * 1000.0) * locals.var_dnm) + (assign105620_e158110 * locals.var_dnm_dn9)), (((locals.var_tmf1_dn10 * 1000.0) * locals.var_dnm) + (assign105620_e158110 * locals.var_dnm_dn10)), (((locals.var_tmf1_dn13 * 1000.0) * locals.var_dnm) + (assign105620_e158110 * locals.var_dnm_dn13)),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn13,)
    }
};
        locals.var_tmf0 = assign105620_e158114;
        locals.var_tmf0_dn0 = assign105620_e158114_d_n0;
        locals.var_tmf0_dn2 = assign105620_e158114_d_n2;
        locals.var_tmf0_dn4 = assign105620_e158114_d_n4;
        locals.var_tmf0_dn5 = assign105620_e158114_d_n5;
        locals.var_tmf0_dn6 = assign105620_e158114_d_n6;
        locals.var_tmf0_dn7 = assign105620_e158114_d_n7;
        locals.var_tmf0_dn8 = assign105620_e158114_d_n8;
        locals.var_tmf0_dn9 = assign105620_e158114_d_n9;
        locals.var_tmf0_dn10 = assign105620_e158114_d_n10;
        locals.var_tmf0_dn13 = assign105620_e158114_d_n13;

        let (assign105630_e158129, assign105630_e158129_d_n0, assign105630_e158129_d_n2, assign105630_e158129_d_n4, assign105630_e158129_d_n5, assign105630_e158129_d_n6, assign105630_e158129_d_n7, assign105630_e158129_d_n8, assign105630_e158129_d_n9, assign105630_e158129_d_n10, assign105630_e158129_d_n13,) = {
    if (((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2387 != 0.0)) {
        let assign105630_e158123: f64 = (1000.0 * locals.var_xmp);
        let assign105630_e158125: f64 = (assign105630_e158123 * locals.var_dnm);
        let assign105630_e158127: f64 = (assign105630_e158125 / locals.var_arg);
        (assign105630_e158127, ((((((1000.0 * locals.var_xmp_dn0) * locals.var_dnm) + (assign105630_e158123 * locals.var_dnm_dn0)) * locals.var_arg) - (assign105630_e158125 * locals.var_arg_dn0)) / (locals.var_arg * locals.var_arg)), ((((((1000.0 * locals.var_xmp_dn2) * locals.var_dnm) + (assign105630_e158123 * locals.var_dnm_dn2)) * locals.var_arg) - (assign105630_e158125 * locals.var_arg_dn2)) / (locals.var_arg * locals.var_arg)), ((((((1000.0 * locals.var_xmp_dn4) * locals.var_dnm) + (assign105630_e158123 * locals.var_dnm_dn4)) * locals.var_arg) - (assign105630_e158125 * locals.var_arg_dn4)) / (locals.var_arg * locals.var_arg)), ((((((1000.0 * locals.var_xmp_dn5) * locals.var_dnm) + (assign105630_e158123 * locals.var_dnm_dn5)) * locals.var_arg) - (assign105630_e158125 * locals.var_arg_dn5)) / (locals.var_arg * locals.var_arg)), ((((((1000.0 * locals.var_xmp_dn6) * locals.var_dnm) + (assign105630_e158123 * locals.var_dnm_dn6)) * locals.var_arg) - (assign105630_e158125 * locals.var_arg_dn6)) / (locals.var_arg * locals.var_arg)), ((((((1000.0 * locals.var_xmp_dn7) * locals.var_dnm) + (assign105630_e158123 * locals.var_dnm_dn7)) * locals.var_arg) - (assign105630_e158125 * locals.var_arg_dn7)) / (locals.var_arg * locals.var_arg)), ((((((1000.0 * locals.var_xmp_dn8) * locals.var_dnm) + (assign105630_e158123 * locals.var_dnm_dn8)) * locals.var_arg) - (assign105630_e158125 * locals.var_arg_dn8)) / (locals.var_arg * locals.var_arg)), ((((((1000.0 * locals.var_xmp_dn9) * locals.var_dnm) + (assign105630_e158123 * locals.var_dnm_dn9)) * locals.var_arg) - (assign105630_e158125 * locals.var_arg_dn9)) / (locals.var_arg * locals.var_arg)), ((((((1000.0 * locals.var_xmp_dn10) * locals.var_dnm) + (assign105630_e158123 * locals.var_dnm_dn10)) * locals.var_arg) - (assign105630_e158125 * locals.var_arg_dn10)) / (locals.var_arg * locals.var_arg)), ((((((1000.0 * locals.var_xmp_dn13) * locals.var_dnm) + (assign105630_e158123 * locals.var_dnm_dn13)) * locals.var_arg) - (assign105630_e158125 * locals.var_arg_dn13)) / (locals.var_arg * locals.var_arg)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign105630_e158129;
        locals.var_t0_dn0 = assign105630_e158129_d_n0;
        locals.var_t0_dn2 = assign105630_e158129_d_n2;
        locals.var_t0_dn4 = assign105630_e158129_d_n4;
        locals.var_t0_dn5 = assign105630_e158129_d_n5;
        locals.var_t0_dn6 = assign105630_e158129_d_n6;
        locals.var_t0_dn7 = assign105630_e158129_d_n7;
        locals.var_t0_dn8 = assign105630_e158129_d_n8;
        locals.var_t0_dn9 = assign105630_e158129_d_n9;
        locals.var_t0_dn10 = assign105630_e158129_d_n10;
        locals.var_t0_dn13 = assign105630_e158129_d_n13;

        let (assign105640_e158142, assign105640_e158142_d_n0, assign105640_e158142_d_n2, assign105640_e158142_d_n4, assign105640_e158142_d_n5, assign105640_e158142_d_n6, assign105640_e158142_d_n7, assign105640_e158142_d_n8, assign105640_e158142_d_n9, assign105640_e158142_d_n10, assign105640_e158142_d_n13,) = {
    if (((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2387 != 0.0)) {
        let assign105640_e158138: f64 = (1000000.0 - 1000.0);
        let assign105640_e158140: f64 = (assign105640_e158138 + locals.var_tmf0);
        (assign105640_e158140, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn13,)
    } else {
        (locals.var_rdd, locals.var_rdd_dn0, locals.var_rdd_dn2, locals.var_rdd_dn4, locals.var_rdd_dn5, locals.var_rdd_dn6, locals.var_rdd_dn7, locals.var_rdd_dn8, locals.var_rdd_dn9, locals.var_rdd_dn10, locals.var_rdd_dn13,)
    }
};
        locals.var_rdd = assign105640_e158142;
        locals.var_rdd_dn0 = assign105640_e158142_d_n0;
        locals.var_rdd_dn2 = assign105640_e158142_d_n2;
        locals.var_rdd_dn4 = assign105640_e158142_d_n4;
        locals.var_rdd_dn5 = assign105640_e158142_d_n5;
        locals.var_rdd_dn6 = assign105640_e158142_d_n6;
        locals.var_rdd_dn7 = assign105640_e158142_d_n7;
        locals.var_rdd_dn8 = assign105640_e158142_d_n8;
        locals.var_rdd_dn9 = assign105640_e158142_d_n9;
        locals.var_rdd_dn10 = assign105640_e158142_d_n10;
        locals.var_rdd_dn13 = assign105640_e158142_d_n13;

        let (assign105650_e158151, assign105650_e158151_d_n0, assign105650_e158151_d_n2, assign105650_e158151_d_n4, assign105650_e158151_d_n5, assign105650_e158151_d_n6, assign105650_e158151_d_n7, assign105650_e158151_d_n8, assign105650_e158151_d_n9, assign105650_e158151_d_n10, assign105650_e158151_d_n13,) = {
    if (((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2387 != 0.0)) {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign105650_e158151;
        locals.var_t0_dn0 = assign105650_e158151_d_n0;
        locals.var_t0_dn2 = assign105650_e158151_d_n2;
        locals.var_t0_dn4 = assign105650_e158151_d_n4;
        locals.var_t0_dn5 = assign105650_e158151_d_n5;
        locals.var_t0_dn6 = assign105650_e158151_d_n6;
        locals.var_t0_dn7 = assign105650_e158151_d_n7;
        locals.var_t0_dn8 = assign105650_e158151_d_n8;
        locals.var_t0_dn9 = assign105650_e158151_d_n9;
        locals.var_t0_dn10 = assign105650_e158151_d_n10;
        locals.var_t0_dn13 = assign105650_e158151_d_n13;

        let (assign105660_e158161, assign105660_e158161_d_n0, assign105660_e158161_d_n2, assign105660_e158161_d_n4, assign105660_e158161_d_n5, assign105660_e158161_d_n6, assign105660_e158161_d_n7, assign105660_e158161_d_n8, assign105660_e158161_d_n9, assign105660_e158161_d_n10, assign105660_e158161_d_n13,) = {
    if (((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2387 == 0.0)) {
        (locals.var_rdd, locals.var_rdd_dn0, locals.var_rdd_dn2, locals.var_rdd_dn4, locals.var_rdd_dn5, locals.var_rdd_dn6, locals.var_rdd_dn7, locals.var_rdd_dn8, locals.var_rdd_dn9, locals.var_rdd_dn10, locals.var_rdd_dn13,)
    } else {
        (locals.var_rdd, locals.var_rdd_dn0, locals.var_rdd_dn2, locals.var_rdd_dn4, locals.var_rdd_dn5, locals.var_rdd_dn6, locals.var_rdd_dn7, locals.var_rdd_dn8, locals.var_rdd_dn9, locals.var_rdd_dn10, locals.var_rdd_dn13,)
    }
};
        locals.var_rdd = assign105660_e158161;
        locals.var_rdd_dn0 = assign105660_e158161_d_n0;
        locals.var_rdd_dn2 = assign105660_e158161_d_n2;
        locals.var_rdd_dn4 = assign105660_e158161_d_n4;
        locals.var_rdd_dn5 = assign105660_e158161_d_n5;
        locals.var_rdd_dn6 = assign105660_e158161_d_n6;
        locals.var_rdd_dn7 = assign105660_e158161_d_n7;
        locals.var_rdd_dn8 = assign105660_e158161_d_n8;
        locals.var_rdd_dn9 = assign105660_e158161_d_n9;
        locals.var_rdd_dn10 = assign105660_e158161_d_n10;
        locals.var_rdd_dn13 = assign105660_e158161_d_n13;

        let (assign105670_e158171, assign105670_e158171_d_n0, assign105670_e158171_d_n2, assign105670_e158171_d_n4, assign105670_e158171_d_n5, assign105670_e158171_d_n6, assign105670_e158171_d_n7, assign105670_e158171_d_n8, assign105670_e158171_d_n9, assign105670_e158171_d_n10, assign105670_e158171_d_n13,) = {
    if (((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2387 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign105670_e158171;
        locals.var_t0_dn0 = assign105670_e158171_d_n0;
        locals.var_t0_dn2 = assign105670_e158171_d_n2;
        locals.var_t0_dn4 = assign105670_e158171_d_n4;
        locals.var_t0_dn5 = assign105670_e158171_d_n5;
        locals.var_t0_dn6 = assign105670_e158171_d_n6;
        locals.var_t0_dn7 = assign105670_e158171_d_n7;
        locals.var_t0_dn8 = assign105670_e158171_d_n8;
        locals.var_t0_dn9 = assign105670_e158171_d_n9;
        locals.var_t0_dn10 = assign105670_e158171_d_n10;
        locals.var_t0_dn13 = assign105670_e158171_d_n13;

        let assign105680_e158178: f64 = (locals.var_mks_nsubsub + locals.var_uc_nover);
        let assign105680_e158179: f64 = (locals.var_uc_nover * assign105680_e158178);
        let assign105680_e158182: f64 = if ((p.p54 == 1.0) && (assign105680_e158179 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard2393 = assign105680_e158182;

        let (assign105690_e158193, assign105690_e158193_d_n0, assign105690_e158193_d_n2, assign105690_e158193_d_n4, assign105690_e158193_d_n5, assign105690_e158193_d_n6, assign105690_e158193_d_n7, assign105690_e158193_d_n8, assign105690_e158193_d_n9, assign105690_e158193_d_n10, assign105690_e158193_d_n13,) = {
    if (((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2393 != 0.0)) {
        let assign105690_e158191: f64 = (p.p334 - locals.var_wdep);
        (assign105690_e158191, (-locals.var_wdep_dn0), (-locals.var_wdep_dn2), (-locals.var_wdep_dn4), (-locals.var_wdep_dn5), (-locals.var_wdep_dn6), (-locals.var_wdep_dn7), (-locals.var_wdep_dn8), (-locals.var_wdep_dn9), (-locals.var_wdep_dn10), (-locals.var_wdep_dn13),)
    } else {
        (locals.var_ddriftld, locals.var_ddriftld_dn0, locals.var_ddriftld_dn2, locals.var_ddriftld_dn4, locals.var_ddriftld_dn5, locals.var_ddriftld_dn6, locals.var_ddriftld_dn7, locals.var_ddriftld_dn8, locals.var_ddriftld_dn9, locals.var_ddriftld_dn10, locals.var_ddriftld_dn13,)
    }
};
        locals.var_ddriftld = assign105690_e158193;
        locals.var_ddriftld_dn0 = assign105690_e158193_d_n0;
        locals.var_ddriftld_dn2 = assign105690_e158193_d_n2;
        locals.var_ddriftld_dn4 = assign105690_e158193_d_n4;
        locals.var_ddriftld_dn5 = assign105690_e158193_d_n5;
        locals.var_ddriftld_dn6 = assign105690_e158193_d_n6;
        locals.var_ddriftld_dn7 = assign105690_e158193_d_n7;
        locals.var_ddriftld_dn8 = assign105690_e158193_d_n8;
        locals.var_ddriftld_dn9 = assign105690_e158193_d_n9;
        locals.var_ddriftld_dn10 = assign105690_e158193_d_n10;
        locals.var_ddriftld_dn13 = assign105690_e158193_d_n13;

        let (assign105700_e158206, assign105700_e158206_d_n0, assign105700_e158206_d_n2, assign105700_e158206_d_n4, assign105700_e158206_d_n5, assign105700_e158206_d_n6, assign105700_e158206_d_n7, assign105700_e158206_d_n8, assign105700_e158206_d_n9, assign105700_e158206_d_n10, assign105700_e158206_d_n13,) = {
    if (((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2393 != 0.0)) {
        let assign105700_e158202: f64 = (locals.var_rdd * locals.var_ldrift0);
        let assign105700_e158204: f64 = (assign105700_e158202 / locals.var_ddriftld);
        (assign105700_e158204, ((((locals.var_rdd_dn0 * locals.var_ldrift0) * locals.var_ddriftld) - (assign105700_e158202 * locals.var_ddriftld_dn0)) / (locals.var_ddriftld * locals.var_ddriftld)), ((((locals.var_rdd_dn2 * locals.var_ldrift0) * locals.var_ddriftld) - (assign105700_e158202 * locals.var_ddriftld_dn2)) / (locals.var_ddriftld * locals.var_ddriftld)), ((((locals.var_rdd_dn4 * locals.var_ldrift0) * locals.var_ddriftld) - (assign105700_e158202 * locals.var_ddriftld_dn4)) / (locals.var_ddriftld * locals.var_ddriftld)), ((((locals.var_rdd_dn5 * locals.var_ldrift0) * locals.var_ddriftld) - (assign105700_e158202 * locals.var_ddriftld_dn5)) / (locals.var_ddriftld * locals.var_ddriftld)), ((((locals.var_rdd_dn6 * locals.var_ldrift0) * locals.var_ddriftld) - (assign105700_e158202 * locals.var_ddriftld_dn6)) / (locals.var_ddriftld * locals.var_ddriftld)), ((((locals.var_rdd_dn7 * locals.var_ldrift0) * locals.var_ddriftld) - (assign105700_e158202 * locals.var_ddriftld_dn7)) / (locals.var_ddriftld * locals.var_ddriftld)), ((((locals.var_rdd_dn8 * locals.var_ldrift0) * locals.var_ddriftld) - (assign105700_e158202 * locals.var_ddriftld_dn8)) / (locals.var_ddriftld * locals.var_ddriftld)), ((((locals.var_rdd_dn9 * locals.var_ldrift0) * locals.var_ddriftld) - (assign105700_e158202 * locals.var_ddriftld_dn9)) / (locals.var_ddriftld * locals.var_ddriftld)), ((((locals.var_rdd_dn10 * locals.var_ldrift0) * locals.var_ddriftld) - (assign105700_e158202 * locals.var_ddriftld_dn10)) / (locals.var_ddriftld * locals.var_ddriftld)), ((((locals.var_rdd_dn13 * locals.var_ldrift0) * locals.var_ddriftld) - (assign105700_e158202 * locals.var_ddriftld_dn13)) / (locals.var_ddriftld * locals.var_ddriftld)),)
    } else {
        (locals.var_rdd, locals.var_rdd_dn0, locals.var_rdd_dn2, locals.var_rdd_dn4, locals.var_rdd_dn5, locals.var_rdd_dn6, locals.var_rdd_dn7, locals.var_rdd_dn8, locals.var_rdd_dn9, locals.var_rdd_dn10, locals.var_rdd_dn13,)
    }
};
        locals.var_rdd = assign105700_e158206;
        locals.var_rdd_dn0 = assign105700_e158206_d_n0;
        locals.var_rdd_dn2 = assign105700_e158206_d_n2;
        locals.var_rdd_dn4 = assign105700_e158206_d_n4;
        locals.var_rdd_dn5 = assign105700_e158206_d_n5;
        locals.var_rdd_dn6 = assign105700_e158206_d_n6;
        locals.var_rdd_dn7 = assign105700_e158206_d_n7;
        locals.var_rdd_dn8 = assign105700_e158206_d_n8;
        locals.var_rdd_dn9 = assign105700_e158206_d_n9;
        locals.var_rdd_dn10 = assign105700_e158206_d_n10;
        locals.var_rdd_dn13 = assign105700_e158206_d_n13;

        let (assign105710_e158215, assign105710_e158215_d_n0, assign105710_e158215_d_n2, assign105710_e158215_d_n4, assign105710_e158215_d_n5, assign105710_e158215_d_n6, assign105710_e158215_d_n7, assign105710_e158215_d_n8, assign105710_e158215_d_n9, assign105710_e158215_d_n10, assign105710_e158215_d_n13,) = {
    if ((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) {
        let assign105710_e158213: f64 = (locals.var_rdd + locals.var_rd0);
        (assign105710_e158213, locals.var_rdd_dn0, locals.var_rdd_dn2, locals.var_rdd_dn4, locals.var_rdd_dn5, locals.var_rdd_dn6, locals.var_rdd_dn7, locals.var_rdd_dn8, locals.var_rdd_dn9, locals.var_rdd_dn10, locals.var_rdd_dn13,)
    } else {
        (locals.var_rdd, locals.var_rdd_dn0, locals.var_rdd_dn2, locals.var_rdd_dn4, locals.var_rdd_dn5, locals.var_rdd_dn6, locals.var_rdd_dn7, locals.var_rdd_dn8, locals.var_rdd_dn9, locals.var_rdd_dn10, locals.var_rdd_dn13,)
    }
};
        locals.var_rdd = assign105710_e158215;
        locals.var_rdd_dn0 = assign105710_e158215_d_n0;
        locals.var_rdd_dn2 = assign105710_e158215_d_n2;
        locals.var_rdd_dn4 = assign105710_e158215_d_n4;
        locals.var_rdd_dn5 = assign105710_e158215_d_n5;
        locals.var_rdd_dn6 = assign105710_e158215_d_n6;
        locals.var_rdd_dn7 = assign105710_e158215_d_n7;
        locals.var_rdd_dn8 = assign105710_e158215_d_n8;
        locals.var_rdd_dn9 = assign105710_e158215_d_n9;
        locals.var_rdd_dn10 = assign105710_e158215_d_n10;
        locals.var_rdd_dn13 = assign105710_e158215_d_n13;

        let assign105750_e158246: f64 = if locals.var_rdd < p.p444 { 1.0 } else { 0.0 };
        locals.var_guard2395 = assign105750_e158246;

        let (assign105760_e158255, assign105760_e158255_d_n0, assign105760_e158255_d_n2, assign105760_e158255_d_n4, assign105760_e158255_d_n5, assign105760_e158255_d_n6, assign105760_e158255_d_n7, assign105760_e158255_d_n8, assign105760_e158255_d_n9, assign105760_e158255_d_n10, assign105760_e158255_d_n13,) = {
    if (((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) && (locals.var_guard2395 != 0.0)) {
        (p.p444, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_rdd, locals.var_rdd_dn0, locals.var_rdd_dn2, locals.var_rdd_dn4, locals.var_rdd_dn5, locals.var_rdd_dn6, locals.var_rdd_dn7, locals.var_rdd_dn8, locals.var_rdd_dn9, locals.var_rdd_dn10, locals.var_rdd_dn13,)
    }
};
        locals.var_rdd = assign105760_e158255;
        locals.var_rdd_dn0 = assign105760_e158255_d_n0;
        locals.var_rdd_dn2 = assign105760_e158255_d_n2;
        locals.var_rdd_dn4 = assign105760_e158255_d_n4;
        locals.var_rdd_dn5 = assign105760_e158255_d_n5;
        locals.var_rdd_dn6 = assign105760_e158255_d_n6;
        locals.var_rdd_dn7 = assign105760_e158255_d_n7;
        locals.var_rdd_dn8 = assign105760_e158255_d_n8;
        locals.var_rdd_dn9 = assign105760_e158255_d_n9;
        locals.var_rdd_dn10 = assign105760_e158255_d_n10;
        locals.var_rdd_dn13 = assign105760_e158255_d_n13;

        let (assign105770_e158264, assign105770_e158264_d_n0, assign105770_e158264_d_n2, assign105770_e158264_d_n4, assign105770_e158264_d_n5, assign105770_e158264_d_n6, assign105770_e158264_d_n7, assign105770_e158264_d_n8, assign105770_e158264_d_n9, assign105770_e158264_d_n10, assign105770_e158264_d_n13,) = {
    if ((locals.var_guard2336 != 0.0) && (locals.var_guard2356 == 0.0)) {
        let assign105770_e158262: f64 = (locals.var_rdd / locals.var_mfactor);
        (assign105770_e158262, (locals.var_rdd_dn0 / locals.var_mfactor), (locals.var_rdd_dn2 / locals.var_mfactor), (locals.var_rdd_dn4 / locals.var_mfactor), (locals.var_rdd_dn5 / locals.var_mfactor), (locals.var_rdd_dn6 / locals.var_mfactor), (locals.var_rdd_dn7 / locals.var_mfactor), (locals.var_rdd_dn8 / locals.var_mfactor), (locals.var_rdd_dn9 / locals.var_mfactor), (locals.var_rdd_dn10 / locals.var_mfactor), (locals.var_rdd_dn13 / locals.var_mfactor),)
    } else {
        (locals.var_rdde, locals.var_rdde_dn0, locals.var_rdde_dn2, locals.var_rdde_dn4, locals.var_rdde_dn5, locals.var_rdde_dn6, locals.var_rdde_dn7, locals.var_rdde_dn8, locals.var_rdde_dn9, locals.var_rdde_dn10, locals.var_rdde_dn13,)
    }
};
        locals.var_rdde = assign105770_e158264;
        locals.var_rdde_dn0 = assign105770_e158264_d_n0;
        locals.var_rdde_dn2 = assign105770_e158264_d_n2;
        locals.var_rdde_dn4 = assign105770_e158264_d_n4;
        locals.var_rdde_dn5 = assign105770_e158264_d_n5;
        locals.var_rdde_dn6 = assign105770_e158264_d_n6;
        locals.var_rdde_dn7 = assign105770_e158264_d_n7;
        locals.var_rdde_dn8 = assign105770_e158264_d_n8;
        locals.var_rdde_dn9 = assign105770_e158264_d_n9;
        locals.var_rdde_dn10 = assign105770_e158264_d_n10;
        locals.var_rdde_dn13 = assign105770_e158264_d_n13;

        let assign105780_e158267: f64 = if locals.var_rdd < p.p444 { 1.0 } else { 0.0 };
        locals.var_guard2396 = assign105780_e158267;

        let (assign105790_e158274, assign105790_e158274_d_n0, assign105790_e158274_d_n2, assign105790_e158274_d_n4, assign105790_e158274_d_n5, assign105790_e158274_d_n6, assign105790_e158274_d_n7, assign105790_e158274_d_n8, assign105790_e158274_d_n9, assign105790_e158274_d_n10, assign105790_e158274_d_n13,) = {
    if ((locals.var_guard2336 == 0.0) && (locals.var_guard2396 != 0.0)) {
        (p.p444, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_rdd, locals.var_rdd_dn0, locals.var_rdd_dn2, locals.var_rdd_dn4, locals.var_rdd_dn5, locals.var_rdd_dn6, locals.var_rdd_dn7, locals.var_rdd_dn8, locals.var_rdd_dn9, locals.var_rdd_dn10, locals.var_rdd_dn13,)
    }
};
        locals.var_rdd = assign105790_e158274;
        locals.var_rdd_dn0 = assign105790_e158274_d_n0;
        locals.var_rdd_dn2 = assign105790_e158274_d_n2;
        locals.var_rdd_dn4 = assign105790_e158274_d_n4;
        locals.var_rdd_dn5 = assign105790_e158274_d_n5;
        locals.var_rdd_dn6 = assign105790_e158274_d_n6;
        locals.var_rdd_dn7 = assign105790_e158274_d_n7;
        locals.var_rdd_dn8 = assign105790_e158274_d_n8;
        locals.var_rdd_dn9 = assign105790_e158274_d_n9;
        locals.var_rdd_dn10 = assign105790_e158274_d_n10;
        locals.var_rdd_dn13 = assign105790_e158274_d_n13;

        let assign105800_e158277: f64 = if locals.var_rsd < p.p444 { 1.0 } else { 0.0 };
        locals.var_guard2397 = assign105800_e158277;

        let (assign105810_e158284, assign105810_e158284_d_n0, assign105810_e158284_d_n2, assign105810_e158284_d_n4, assign105810_e158284_d_n5, assign105810_e158284_d_n6, assign105810_e158284_d_n7, assign105810_e158284_d_n8, assign105810_e158284_d_n9, assign105810_e158284_d_n10, assign105810_e158284_d_n13,) = {
    if ((locals.var_guard2336 == 0.0) && (locals.var_guard2397 != 0.0)) {
        (p.p444, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_rsd, locals.var_rsd_dn0, locals.var_rsd_dn2, locals.var_rsd_dn4, locals.var_rsd_dn5, locals.var_rsd_dn6, locals.var_rsd_dn7, locals.var_rsd_dn8, locals.var_rsd_dn9, locals.var_rsd_dn10, locals.var_rsd_dn13,)
    }
};
        locals.var_rsd = assign105810_e158284;
        locals.var_rsd_dn0 = assign105810_e158284_d_n0;
        locals.var_rsd_dn2 = assign105810_e158284_d_n2;
        locals.var_rsd_dn4 = assign105810_e158284_d_n4;
        locals.var_rsd_dn5 = assign105810_e158284_d_n5;
        locals.var_rsd_dn6 = assign105810_e158284_d_n6;
        locals.var_rsd_dn7 = assign105810_e158284_d_n7;
        locals.var_rsd_dn8 = assign105810_e158284_d_n8;
        locals.var_rsd_dn9 = assign105810_e158284_d_n9;
        locals.var_rsd_dn10 = assign105810_e158284_d_n10;
        locals.var_rsd_dn13 = assign105810_e158284_d_n13;

        let assign105820_e158287: f64 = if locals.var_vdsemodenml > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2398 = assign105820_e158287;

        let (assign105830_e158296, assign105830_e158296_d_n0, assign105830_e158296_d_n2, assign105830_e158296_d_n4, assign105830_e158296_d_n5, assign105830_e158296_d_n6, assign105830_e158296_d_n7, assign105830_e158296_d_n8, assign105830_e158296_d_n9, assign105830_e158296_d_n10, assign105830_e158296_d_n13,) = {
    if ((locals.var_guard2336 == 0.0) && (locals.var_guard2398 != 0.0)) {
        let assign105830_e158294: f64 = (locals.var_rdd / locals.var_mfactor);
        (assign105830_e158294, (locals.var_rdd_dn0 / locals.var_mfactor), (locals.var_rdd_dn2 / locals.var_mfactor), (locals.var_rdd_dn4 / locals.var_mfactor), (locals.var_rdd_dn5 / locals.var_mfactor), (locals.var_rdd_dn6 / locals.var_mfactor), (locals.var_rdd_dn7 / locals.var_mfactor), (locals.var_rdd_dn8 / locals.var_mfactor), (locals.var_rdd_dn9 / locals.var_mfactor), (locals.var_rdd_dn10 / locals.var_mfactor), (locals.var_rdd_dn13 / locals.var_mfactor),)
    } else {
        (locals.var_rdde, locals.var_rdde_dn0, locals.var_rdde_dn2, locals.var_rdde_dn4, locals.var_rdde_dn5, locals.var_rdde_dn6, locals.var_rdde_dn7, locals.var_rdde_dn8, locals.var_rdde_dn9, locals.var_rdde_dn10, locals.var_rdde_dn13,)
    }
};
        locals.var_rdde = assign105830_e158296;
        locals.var_rdde_dn0 = assign105830_e158296_d_n0;
        locals.var_rdde_dn2 = assign105830_e158296_d_n2;
        locals.var_rdde_dn4 = assign105830_e158296_d_n4;
        locals.var_rdde_dn5 = assign105830_e158296_d_n5;
        locals.var_rdde_dn6 = assign105830_e158296_d_n6;
        locals.var_rdde_dn7 = assign105830_e158296_d_n7;
        locals.var_rdde_dn8 = assign105830_e158296_d_n8;
        locals.var_rdde_dn9 = assign105830_e158296_d_n9;
        locals.var_rdde_dn10 = assign105830_e158296_d_n10;
        locals.var_rdde_dn13 = assign105830_e158296_d_n13;

        let (assign105840_e158305, assign105840_e158305_d_n0, assign105840_e158305_d_n2, assign105840_e158305_d_n4, assign105840_e158305_d_n5, assign105840_e158305_d_n6, assign105840_e158305_d_n7, assign105840_e158305_d_n8, assign105840_e158305_d_n9, assign105840_e158305_d_n10, assign105840_e158305_d_n13,) = {
    if ((locals.var_guard2336 == 0.0) && (locals.var_guard2398 != 0.0)) {
        let assign105840_e158303: f64 = (locals.var_rsd / locals.var_mfactor);
        (assign105840_e158303, (locals.var_rsd_dn0 / locals.var_mfactor), (locals.var_rsd_dn2 / locals.var_mfactor), (locals.var_rsd_dn4 / locals.var_mfactor), (locals.var_rsd_dn5 / locals.var_mfactor), (locals.var_rsd_dn6 / locals.var_mfactor), (locals.var_rsd_dn7 / locals.var_mfactor), (locals.var_rsd_dn8 / locals.var_mfactor), (locals.var_rsd_dn9 / locals.var_mfactor), (locals.var_rsd_dn10 / locals.var_mfactor), (locals.var_rsd_dn13 / locals.var_mfactor),)
    } else {
        (locals.var_rsde, locals.var_rsde_dn0, locals.var_rsde_dn2, locals.var_rsde_dn4, locals.var_rsde_dn5, locals.var_rsde_dn6, locals.var_rsde_dn7, locals.var_rsde_dn8, locals.var_rsde_dn9, locals.var_rsde_dn10, locals.var_rsde_dn13,)
    }
};
        locals.var_rsde = assign105840_e158305;
        locals.var_rsde_dn0 = assign105840_e158305_d_n0;
        locals.var_rsde_dn2 = assign105840_e158305_d_n2;
        locals.var_rsde_dn4 = assign105840_e158305_d_n4;
        locals.var_rsde_dn5 = assign105840_e158305_d_n5;
        locals.var_rsde_dn6 = assign105840_e158305_d_n6;
        locals.var_rsde_dn7 = assign105840_e158305_d_n7;
        locals.var_rsde_dn8 = assign105840_e158305_d_n8;
        locals.var_rsde_dn9 = assign105840_e158305_d_n9;
        locals.var_rsde_dn10 = assign105840_e158305_d_n10;
        locals.var_rsde_dn13 = assign105840_e158305_d_n13;

        let (assign105850_e158315, assign105850_e158315_d_n0, assign105850_e158315_d_n2, assign105850_e158315_d_n4, assign105850_e158315_d_n5, assign105850_e158315_d_n6, assign105850_e158315_d_n7, assign105850_e158315_d_n8, assign105850_e158315_d_n9, assign105850_e158315_d_n10, assign105850_e158315_d_n13,) = {
    if ((locals.var_guard2336 == 0.0) && (locals.var_guard2398 == 0.0)) {
        let assign105850_e158313: f64 = (locals.var_rsd / locals.var_mfactor);
        (assign105850_e158313, (locals.var_rsd_dn0 / locals.var_mfactor), (locals.var_rsd_dn2 / locals.var_mfactor), (locals.var_rsd_dn4 / locals.var_mfactor), (locals.var_rsd_dn5 / locals.var_mfactor), (locals.var_rsd_dn6 / locals.var_mfactor), (locals.var_rsd_dn7 / locals.var_mfactor), (locals.var_rsd_dn8 / locals.var_mfactor), (locals.var_rsd_dn9 / locals.var_mfactor), (locals.var_rsd_dn10 / locals.var_mfactor), (locals.var_rsd_dn13 / locals.var_mfactor),)
    } else {
        (locals.var_rdde, locals.var_rdde_dn0, locals.var_rdde_dn2, locals.var_rdde_dn4, locals.var_rdde_dn5, locals.var_rdde_dn6, locals.var_rdde_dn7, locals.var_rdde_dn8, locals.var_rdde_dn9, locals.var_rdde_dn10, locals.var_rdde_dn13,)
    }
};
        locals.var_rdde = assign105850_e158315;
        locals.var_rdde_dn0 = assign105850_e158315_d_n0;
        locals.var_rdde_dn2 = assign105850_e158315_d_n2;
        locals.var_rdde_dn4 = assign105850_e158315_d_n4;
        locals.var_rdde_dn5 = assign105850_e158315_d_n5;
        locals.var_rdde_dn6 = assign105850_e158315_d_n6;
        locals.var_rdde_dn7 = assign105850_e158315_d_n7;
        locals.var_rdde_dn8 = assign105850_e158315_d_n8;
        locals.var_rdde_dn9 = assign105850_e158315_d_n9;
        locals.var_rdde_dn10 = assign105850_e158315_d_n10;
        locals.var_rdde_dn13 = assign105850_e158315_d_n13;

        let (assign105860_e158325, assign105860_e158325_d_n0, assign105860_e158325_d_n2, assign105860_e158325_d_n4, assign105860_e158325_d_n5, assign105860_e158325_d_n6, assign105860_e158325_d_n7, assign105860_e158325_d_n8, assign105860_e158325_d_n9, assign105860_e158325_d_n10, assign105860_e158325_d_n13,) = {
    if ((locals.var_guard2336 == 0.0) && (locals.var_guard2398 == 0.0)) {
        let assign105860_e158323: f64 = (locals.var_rdd / locals.var_mfactor);
        (assign105860_e158323, (locals.var_rdd_dn0 / locals.var_mfactor), (locals.var_rdd_dn2 / locals.var_mfactor), (locals.var_rdd_dn4 / locals.var_mfactor), (locals.var_rdd_dn5 / locals.var_mfactor), (locals.var_rdd_dn6 / locals.var_mfactor), (locals.var_rdd_dn7 / locals.var_mfactor), (locals.var_rdd_dn8 / locals.var_mfactor), (locals.var_rdd_dn9 / locals.var_mfactor), (locals.var_rdd_dn10 / locals.var_mfactor), (locals.var_rdd_dn13 / locals.var_mfactor),)
    } else {
        (locals.var_rsde, locals.var_rsde_dn0, locals.var_rsde_dn2, locals.var_rsde_dn4, locals.var_rsde_dn5, locals.var_rsde_dn6, locals.var_rsde_dn7, locals.var_rsde_dn8, locals.var_rsde_dn9, locals.var_rsde_dn10, locals.var_rsde_dn13,)
    }
};
        locals.var_rsde = assign105860_e158325;
        locals.var_rsde_dn0 = assign105860_e158325_d_n0;
        locals.var_rsde_dn2 = assign105860_e158325_d_n2;
        locals.var_rsde_dn4 = assign105860_e158325_d_n4;
        locals.var_rsde_dn5 = assign105860_e158325_d_n5;
        locals.var_rsde_dn6 = assign105860_e158325_d_n6;
        locals.var_rsde_dn7 = assign105860_e158325_d_n7;
        locals.var_rsde_dn8 = assign105860_e158325_d_n8;
        locals.var_rsde_dn9 = assign105860_e158325_d_n9;
        locals.var_rsde_dn10 = assign105860_e158325_d_n10;
        locals.var_rsde_dn13 = assign105860_e158325_d_n13;

        locals.var_rdd = locals.var_rdde;
        locals.var_rdd_dn0 = locals.var_rdde_dn0;
        locals.var_rdd_dn2 = locals.var_rdde_dn2;
        locals.var_rdd_dn4 = locals.var_rdde_dn4;
        locals.var_rdd_dn5 = locals.var_rdde_dn5;
        locals.var_rdd_dn6 = locals.var_rdde_dn6;
        locals.var_rdd_dn7 = locals.var_rdde_dn7;
        locals.var_rdd_dn8 = locals.var_rdde_dn8;
        locals.var_rdd_dn9 = locals.var_rdde_dn9;
        locals.var_rdd_dn10 = locals.var_rdde_dn10;
        locals.var_rdd_dn13 = locals.var_rdde_dn13;

        locals.var_rsd = locals.var_rsde;
        locals.var_rsd_dn0 = locals.var_rsde_dn0;
        locals.var_rsd_dn2 = locals.var_rsde_dn2;
        locals.var_rsd_dn4 = locals.var_rsde_dn4;
        locals.var_rsd_dn5 = locals.var_rsde_dn5;
        locals.var_rsd_dn6 = locals.var_rsde_dn6;
        locals.var_rsd_dn7 = locals.var_rsde_dn7;
        locals.var_rsd_dn8 = locals.var_rsde_dn8;
        locals.var_rsd_dn9 = locals.var_rsde_dn9;
        locals.var_rsd_dn10 = locals.var_rsde_dn10;
        locals.var_rsd_dn13 = locals.var_rsde_dn13;

        let assign105920_e158333: f64 = if locals.var_mode > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2399 = assign105920_e158333;

        let (assign105930_e158337, assign105930_e158337_d_n0, assign105930_e158337_d_n2, assign105930_e158337_d_n4, assign105930_e158337_d_n5, assign105930_e158337_d_n6, assign105930_e158337_d_n7, assign105930_e158337_d_n8, assign105930_e158337_d_n9, assign105930_e158337_d_n10, assign105930_e158337_d_n13,) = {
    if (locals.var_guard2399 != 0.0) {
        (locals.var_idse, locals.var_idse_dn0, locals.var_idse_dn2, locals.var_idse_dn4, locals.var_idse_dn5, locals.var_idse_dn6, locals.var_idse_dn7, locals.var_idse_dn8, locals.var_idse_dn9, locals.var_idse_dn10, locals.var_idse_dn13,)
    } else {
        (locals.var_ids, locals.var_ids_dn0, locals.var_ids_dn2, locals.var_ids_dn4, locals.var_ids_dn5, locals.var_ids_dn6, locals.var_ids_dn7, locals.var_ids_dn8, locals.var_ids_dn9, locals.var_ids_dn10, locals.var_ids_dn13,)
    }
};
        locals.var_ids = assign105930_e158337;
        locals.var_ids_dn0 = assign105930_e158337_d_n0;
        locals.var_ids_dn2 = assign105930_e158337_d_n2;
        locals.var_ids_dn4 = assign105930_e158337_d_n4;
        locals.var_ids_dn5 = assign105930_e158337_d_n5;
        locals.var_ids_dn6 = assign105930_e158337_d_n6;
        locals.var_ids_dn7 = assign105930_e158337_d_n7;
        locals.var_ids_dn8 = assign105930_e158337_d_n8;
        locals.var_ids_dn9 = assign105930_e158337_d_n9;
        locals.var_ids_dn10 = assign105930_e158337_d_n10;
        locals.var_ids_dn13 = assign105930_e158337_d_n13;

        let (assign105940_e158341, assign105940_e158341_d_n0, assign105940_e158341_d_n2, assign105940_e158341_d_n4, assign105940_e158341_d_n5, assign105940_e158341_d_n6, assign105940_e158341_d_n7, assign105940_e158341_d_n8, assign105940_e158341_d_n9, assign105940_e158341_d_n10, assign105940_e158341_d_n13,) = {
    if (locals.var_guard2399 != 0.0) {
        (locals.var_qde, locals.var_qde_dn0, locals.var_qde_dn2, locals.var_qde_dn4, locals.var_qde_dn5, locals.var_qde_dn6, locals.var_qde_dn7, locals.var_qde_dn8, locals.var_qde_dn9, locals.var_qde_dn10, locals.var_qde_dn13,)
    } else {
        (locals.var_qd, locals.var_qd_dn0, locals.var_qd_dn2, locals.var_qd_dn4, locals.var_qd_dn5, locals.var_qd_dn6, locals.var_qd_dn7, locals.var_qd_dn8, locals.var_qd_dn9, locals.var_qd_dn10, locals.var_qd_dn13,)
    }
};
        locals.var_qd = assign105940_e158341;
        locals.var_qd_dn0 = assign105940_e158341_d_n0;
        locals.var_qd_dn2 = assign105940_e158341_d_n2;
        locals.var_qd_dn4 = assign105940_e158341_d_n4;
        locals.var_qd_dn5 = assign105940_e158341_d_n5;
        locals.var_qd_dn6 = assign105940_e158341_d_n6;
        locals.var_qd_dn7 = assign105940_e158341_d_n7;
        locals.var_qd_dn8 = assign105940_e158341_d_n8;
        locals.var_qd_dn9 = assign105940_e158341_d_n9;
        locals.var_qd_dn10 = assign105940_e158341_d_n10;
        locals.var_qd_dn13 = assign105940_e158341_d_n13;

    }

    pub(super) fn stamp_transient_block_373(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign105950_e158345, assign105950_e158345_d_n0, assign105950_e158345_d_n2, assign105950_e158345_d_n4, assign105950_e158345_d_n5, assign105950_e158345_d_n6, assign105950_e158345_d_n7, assign105950_e158345_d_n8, assign105950_e158345_d_n9, assign105950_e158345_d_n10, assign105950_e158345_d_n13,) = {
    if (locals.var_guard2399 != 0.0) {
        (locals.var_qge, locals.var_qge_dn0, locals.var_qge_dn2, locals.var_qge_dn4, locals.var_qge_dn5, locals.var_qge_dn6, locals.var_qge_dn7, locals.var_qge_dn8, locals.var_qge_dn9, locals.var_qge_dn10, locals.var_qge_dn13,)
    } else {
        (locals.var_qg, locals.var_qg_dn0, locals.var_qg_dn2, locals.var_qg_dn4, locals.var_qg_dn5, locals.var_qg_dn6, locals.var_qg_dn7, locals.var_qg_dn8, locals.var_qg_dn9, locals.var_qg_dn10, locals.var_qg_dn13,)
    }
};
        locals.var_qg = assign105950_e158345;
        locals.var_qg_dn0 = assign105950_e158345_d_n0;
        locals.var_qg_dn2 = assign105950_e158345_d_n2;
        locals.var_qg_dn4 = assign105950_e158345_d_n4;
        locals.var_qg_dn5 = assign105950_e158345_d_n5;
        locals.var_qg_dn6 = assign105950_e158345_d_n6;
        locals.var_qg_dn7 = assign105950_e158345_d_n7;
        locals.var_qg_dn8 = assign105950_e158345_d_n8;
        locals.var_qg_dn9 = assign105950_e158345_d_n9;
        locals.var_qg_dn10 = assign105950_e158345_d_n10;
        locals.var_qg_dn13 = assign105950_e158345_d_n13;

        let (assign105960_e158349, assign105960_e158349_d_n0, assign105960_e158349_d_n2, assign105960_e158349_d_n4, assign105960_e158349_d_n5, assign105960_e158349_d_n6, assign105960_e158349_d_n7, assign105960_e158349_d_n8, assign105960_e158349_d_n9, assign105960_e158349_d_n10, assign105960_e158349_d_n13,) = {
    if (locals.var_guard2399 != 0.0) {
        (locals.var_qse, locals.var_qse_dn0, locals.var_qse_dn2, locals.var_qse_dn4, locals.var_qse_dn5, locals.var_qse_dn6, locals.var_qse_dn7, locals.var_qse_dn8, locals.var_qse_dn9, locals.var_qse_dn10, locals.var_qse_dn13,)
    } else {
        (locals.var_qs, locals.var_qs_dn0, locals.var_qs_dn2, locals.var_qs_dn4, locals.var_qs_dn5, locals.var_qs_dn6, locals.var_qs_dn7, locals.var_qs_dn8, locals.var_qs_dn9, locals.var_qs_dn10, locals.var_qs_dn13,)
    }
};
        locals.var_qs = assign105960_e158349;
        locals.var_qs_dn0 = assign105960_e158349_d_n0;
        locals.var_qs_dn2 = assign105960_e158349_d_n2;
        locals.var_qs_dn4 = assign105960_e158349_d_n4;
        locals.var_qs_dn5 = assign105960_e158349_d_n5;
        locals.var_qs_dn6 = assign105960_e158349_d_n6;
        locals.var_qs_dn7 = assign105960_e158349_d_n7;
        locals.var_qs_dn8 = assign105960_e158349_d_n8;
        locals.var_qs_dn9 = assign105960_e158349_d_n9;
        locals.var_qs_dn10 = assign105960_e158349_d_n10;
        locals.var_qs_dn13 = assign105960_e158349_d_n13;

        let (assign105970_e158358, assign105970_e158358_d_n0, assign105970_e158358_d_n2, assign105970_e158358_d_n4, assign105970_e158358_d_n5, assign105970_e158358_d_n6, assign105970_e158358_d_n7, assign105970_e158358_d_n8, assign105970_e158358_d_n9, assign105970_e158358_d_n10, assign105970_e158358_d_n13,) = {
    if (locals.var_guard2399 != 0.0) {
        let assign105970_e158353: f64 = (locals.var_qge + locals.var_qde);
        let assign105970_e158355: f64 = (assign105970_e158353 + locals.var_qse);
        let assign105970_e158356: f64 = (-assign105970_e158355);
        (assign105970_e158356, (-((locals.var_qge_dn0 + locals.var_qde_dn0) + locals.var_qse_dn0)), (-((locals.var_qge_dn2 + locals.var_qde_dn2) + locals.var_qse_dn2)), (-((locals.var_qge_dn4 + locals.var_qde_dn4) + locals.var_qse_dn4)), (-((locals.var_qge_dn5 + locals.var_qde_dn5) + locals.var_qse_dn5)), (-((locals.var_qge_dn6 + locals.var_qde_dn6) + locals.var_qse_dn6)), (-((locals.var_qge_dn7 + locals.var_qde_dn7) + locals.var_qse_dn7)), (-((locals.var_qge_dn8 + locals.var_qde_dn8) + locals.var_qse_dn8)), (-((locals.var_qge_dn9 + locals.var_qde_dn9) + locals.var_qse_dn9)), (-((locals.var_qge_dn10 + locals.var_qde_dn10) + locals.var_qse_dn10)), (-((locals.var_qge_dn13 + locals.var_qde_dn13) + locals.var_qse_dn13)),)
    } else {
        (locals.var_qb, locals.var_qb_dn0, locals.var_qb_dn2, locals.var_qb_dn4, locals.var_qb_dn5, locals.var_qb_dn6, locals.var_qb_dn7, locals.var_qb_dn8, locals.var_qb_dn9, locals.var_qb_dn10, locals.var_qb_dn13,)
    }
};
        locals.var_qb = assign105970_e158358;
        locals.var_qb_dn0 = assign105970_e158358_d_n0;
        locals.var_qb_dn2 = assign105970_e158358_d_n2;
        locals.var_qb_dn4 = assign105970_e158358_d_n4;
        locals.var_qb_dn5 = assign105970_e158358_d_n5;
        locals.var_qb_dn6 = assign105970_e158358_d_n6;
        locals.var_qb_dn7 = assign105970_e158358_d_n7;
        locals.var_qb_dn8 = assign105970_e158358_d_n8;
        locals.var_qb_dn9 = assign105970_e158358_d_n9;
        locals.var_qb_dn10 = assign105970_e158358_d_n10;
        locals.var_qb_dn13 = assign105970_e158358_d_n13;

        let (assign105980_e158362, assign105980_e158362_d_n0, assign105980_e158362_d_n2, assign105980_e158362_d_n4, assign105980_e158362_d_n5, assign105980_e158362_d_n6, assign105980_e158362_d_n7, assign105980_e158362_d_n8, assign105980_e158362_d_n9, assign105980_e158362_d_n10, assign105980_e158362_d_n13,) = {
    if (locals.var_guard2399 != 0.0) {
        (locals.var_isube, locals.var_isube_dn0, locals.var_isube_dn2, locals.var_isube_dn4, locals.var_isube_dn5, locals.var_isube_dn6, locals.var_isube_dn7, locals.var_isube_dn8, locals.var_isube_dn9, locals.var_isube_dn10, locals.var_isube_dn13,)
    } else {
        (locals.var_isub, locals.var_isub_dn0, locals.var_isub_dn2, locals.var_isub_dn4, locals.var_isub_dn5, locals.var_isub_dn6, locals.var_isub_dn7, locals.var_isub_dn8, locals.var_isub_dn9, locals.var_isub_dn10, locals.var_isub_dn13,)
    }
};
        locals.var_isub = assign105980_e158362;
        locals.var_isub_dn0 = assign105980_e158362_d_n0;
        locals.var_isub_dn2 = assign105980_e158362_d_n2;
        locals.var_isub_dn4 = assign105980_e158362_d_n4;
        locals.var_isub_dn5 = assign105980_e158362_d_n5;
        locals.var_isub_dn6 = assign105980_e158362_d_n6;
        locals.var_isub_dn7 = assign105980_e158362_d_n7;
        locals.var_isub_dn8 = assign105980_e158362_d_n8;
        locals.var_isub_dn9 = assign105980_e158362_d_n9;
        locals.var_isub_dn10 = assign105980_e158362_d_n10;
        locals.var_isub_dn13 = assign105980_e158362_d_n13;

        let (assign106000_e158370, assign106000_e158370_d_n0, assign106000_e158370_d_n2, assign106000_e158370_d_n4, assign106000_e158370_d_n5, assign106000_e158370_d_n6, assign106000_e158370_d_n7, assign106000_e158370_d_n8, assign106000_e158370_d_n9, assign106000_e158370_d_n10, assign106000_e158370_d_n13,) = {
    if (locals.var_guard2399 != 0.0) {
        (locals.var_isublde, locals.var_isublde_dn0, locals.var_isublde_dn2, locals.var_isublde_dn4, locals.var_isublde_dn5, locals.var_isublde_dn6, locals.var_isublde_dn7, locals.var_isublde_dn8, locals.var_isublde_dn9, locals.var_isublde_dn10, locals.var_isublde_dn13,)
    } else {
        (locals.var_isubld, locals.var_isubld_dn0, locals.var_isubld_dn2, locals.var_isubld_dn4, locals.var_isubld_dn5, locals.var_isubld_dn6, locals.var_isubld_dn7, locals.var_isubld_dn8, locals.var_isubld_dn9, locals.var_isubld_dn10, locals.var_isubld_dn13,)
    }
};
        locals.var_isubld = assign106000_e158370;
        locals.var_isubld_dn0 = assign106000_e158370_d_n0;
        locals.var_isubld_dn2 = assign106000_e158370_d_n2;
        locals.var_isubld_dn4 = assign106000_e158370_d_n4;
        locals.var_isubld_dn5 = assign106000_e158370_d_n5;
        locals.var_isubld_dn6 = assign106000_e158370_d_n6;
        locals.var_isubld_dn7 = assign106000_e158370_d_n7;
        locals.var_isubld_dn8 = assign106000_e158370_d_n8;
        locals.var_isubld_dn9 = assign106000_e158370_d_n9;
        locals.var_isubld_dn10 = assign106000_e158370_d_n10;
        locals.var_isubld_dn13 = assign106000_e158370_d_n13;

        let (assign106020_e158378, assign106020_e158378_d_n0, assign106020_e158378_d_n2, assign106020_e158378_d_n4, assign106020_e158378_d_n5, assign106020_e158378_d_n6, assign106020_e158378_d_n7, assign106020_e158378_d_n8, assign106020_e158378_d_n9, assign106020_e158378_d_n10, assign106020_e158378_d_n13,) = {
    if (locals.var_guard2399 != 0.0) {
        (locals.var_idsibpce, locals.var_idsibpce_dn0, locals.var_idsibpce_dn2, locals.var_idsibpce_dn4, locals.var_idsibpce_dn5, locals.var_idsibpce_dn6, locals.var_idsibpce_dn7, locals.var_idsibpce_dn8, locals.var_idsibpce_dn9, locals.var_idsibpce_dn10, locals.var_idsibpce_dn13,)
    } else {
        (locals.var_idsibpc, locals.var_idsibpc_dn0, locals.var_idsibpc_dn2, locals.var_idsibpc_dn4, locals.var_idsibpc_dn5, locals.var_idsibpc_dn6, locals.var_idsibpc_dn7, locals.var_idsibpc_dn8, locals.var_idsibpc_dn9, locals.var_idsibpc_dn10, locals.var_idsibpc_dn13,)
    }
};
        locals.var_idsibpc = assign106020_e158378;
        locals.var_idsibpc_dn0 = assign106020_e158378_d_n0;
        locals.var_idsibpc_dn2 = assign106020_e158378_d_n2;
        locals.var_idsibpc_dn4 = assign106020_e158378_d_n4;
        locals.var_idsibpc_dn5 = assign106020_e158378_d_n5;
        locals.var_idsibpc_dn6 = assign106020_e158378_d_n6;
        locals.var_idsibpc_dn7 = assign106020_e158378_d_n7;
        locals.var_idsibpc_dn8 = assign106020_e158378_d_n8;
        locals.var_idsibpc_dn9 = assign106020_e158378_d_n9;
        locals.var_idsibpc_dn10 = assign106020_e158378_d_n10;
        locals.var_idsibpc_dn13 = assign106020_e158378_d_n13;

        let (assign106100_e158412, assign106100_e158412_d_n0, assign106100_e158412_d_n2, assign106100_e158412_d_n4, assign106100_e158412_d_n5, assign106100_e158412_d_n6, assign106100_e158412_d_n7, assign106100_e158412_d_n8, assign106100_e158412_d_n9, assign106100_e158412_d_n10, assign106100_e158412_d_n13,) = {
    if ((locals.var_guard2399 != 0.0) && (locals.var_flg_nqs != 0.0)) {
        (locals.var_xd, locals.var_xd_dn0, locals.var_xd_dn2, locals.var_xd_dn4, locals.var_xd_dn5, locals.var_xd_dn6, locals.var_xd_dn7, locals.var_xd_dn8, locals.var_xd_dn9, locals.var_xd_dn10, locals.var_xd_dn13,)
    } else {
        (locals.var_qdrat, locals.var_qdrat_dn0, locals.var_qdrat_dn2, locals.var_qdrat_dn4, locals.var_qdrat_dn5, locals.var_qdrat_dn6, locals.var_qdrat_dn7, locals.var_qdrat_dn8, locals.var_qdrat_dn9, locals.var_qdrat_dn10, locals.var_qdrat_dn13,)
    }
};
        locals.var_qdrat = assign106100_e158412;
        locals.var_qdrat_dn0 = assign106100_e158412_d_n0;
        locals.var_qdrat_dn2 = assign106100_e158412_d_n2;
        locals.var_qdrat_dn4 = assign106100_e158412_d_n4;
        locals.var_qdrat_dn5 = assign106100_e158412_d_n5;
        locals.var_qdrat_dn6 = assign106100_e158412_d_n6;
        locals.var_qdrat_dn7 = assign106100_e158412_d_n7;
        locals.var_qdrat_dn8 = assign106100_e158412_d_n8;
        locals.var_qdrat_dn9 = assign106100_e158412_d_n9;
        locals.var_qdrat_dn10 = assign106100_e158412_d_n10;
        locals.var_qdrat_dn13 = assign106100_e158412_d_n13;

        let (assign106110_e158418, assign106110_e158418_d_n0, assign106110_e158418_d_n2, assign106110_e158418_d_n4, assign106110_e158418_d_n5, assign106110_e158418_d_n6, assign106110_e158418_d_n7, assign106110_e158418_d_n8, assign106110_e158418_d_n9, assign106110_e158418_d_n10, assign106110_e158418_d_n13,) = {
    if (locals.var_guard2399 == 0.0) {
        let assign106110_e158416: f64 = (-locals.var_idse);
        (assign106110_e158416, (-locals.var_idse_dn0), (-locals.var_idse_dn2), (-locals.var_idse_dn4), (-locals.var_idse_dn5), (-locals.var_idse_dn6), (-locals.var_idse_dn7), (-locals.var_idse_dn8), (-locals.var_idse_dn9), (-locals.var_idse_dn10), (-locals.var_idse_dn13),)
    } else {
        (locals.var_ids, locals.var_ids_dn0, locals.var_ids_dn2, locals.var_ids_dn4, locals.var_ids_dn5, locals.var_ids_dn6, locals.var_ids_dn7, locals.var_ids_dn8, locals.var_ids_dn9, locals.var_ids_dn10, locals.var_ids_dn13,)
    }
};
        locals.var_ids = assign106110_e158418;
        locals.var_ids_dn0 = assign106110_e158418_d_n0;
        locals.var_ids_dn2 = assign106110_e158418_d_n2;
        locals.var_ids_dn4 = assign106110_e158418_d_n4;
        locals.var_ids_dn5 = assign106110_e158418_d_n5;
        locals.var_ids_dn6 = assign106110_e158418_d_n6;
        locals.var_ids_dn7 = assign106110_e158418_d_n7;
        locals.var_ids_dn8 = assign106110_e158418_d_n8;
        locals.var_ids_dn9 = assign106110_e158418_d_n9;
        locals.var_ids_dn10 = assign106110_e158418_d_n10;
        locals.var_ids_dn13 = assign106110_e158418_d_n13;

        let (assign106120_e158423, assign106120_e158423_d_n0, assign106120_e158423_d_n2, assign106120_e158423_d_n4, assign106120_e158423_d_n5, assign106120_e158423_d_n6, assign106120_e158423_d_n7, assign106120_e158423_d_n8, assign106120_e158423_d_n9, assign106120_e158423_d_n10, assign106120_e158423_d_n13,) = {
    if (locals.var_guard2399 == 0.0) {
        (locals.var_qse, locals.var_qse_dn0, locals.var_qse_dn2, locals.var_qse_dn4, locals.var_qse_dn5, locals.var_qse_dn6, locals.var_qse_dn7, locals.var_qse_dn8, locals.var_qse_dn9, locals.var_qse_dn10, locals.var_qse_dn13,)
    } else {
        (locals.var_qd, locals.var_qd_dn0, locals.var_qd_dn2, locals.var_qd_dn4, locals.var_qd_dn5, locals.var_qd_dn6, locals.var_qd_dn7, locals.var_qd_dn8, locals.var_qd_dn9, locals.var_qd_dn10, locals.var_qd_dn13,)
    }
};
        locals.var_qd = assign106120_e158423;
        locals.var_qd_dn0 = assign106120_e158423_d_n0;
        locals.var_qd_dn2 = assign106120_e158423_d_n2;
        locals.var_qd_dn4 = assign106120_e158423_d_n4;
        locals.var_qd_dn5 = assign106120_e158423_d_n5;
        locals.var_qd_dn6 = assign106120_e158423_d_n6;
        locals.var_qd_dn7 = assign106120_e158423_d_n7;
        locals.var_qd_dn8 = assign106120_e158423_d_n8;
        locals.var_qd_dn9 = assign106120_e158423_d_n9;
        locals.var_qd_dn10 = assign106120_e158423_d_n10;
        locals.var_qd_dn13 = assign106120_e158423_d_n13;

        let (assign106130_e158428, assign106130_e158428_d_n0, assign106130_e158428_d_n2, assign106130_e158428_d_n4, assign106130_e158428_d_n5, assign106130_e158428_d_n6, assign106130_e158428_d_n7, assign106130_e158428_d_n8, assign106130_e158428_d_n9, assign106130_e158428_d_n10, assign106130_e158428_d_n13,) = {
    if (locals.var_guard2399 == 0.0) {
        (locals.var_qge, locals.var_qge_dn0, locals.var_qge_dn2, locals.var_qge_dn4, locals.var_qge_dn5, locals.var_qge_dn6, locals.var_qge_dn7, locals.var_qge_dn8, locals.var_qge_dn9, locals.var_qge_dn10, locals.var_qge_dn13,)
    } else {
        (locals.var_qg, locals.var_qg_dn0, locals.var_qg_dn2, locals.var_qg_dn4, locals.var_qg_dn5, locals.var_qg_dn6, locals.var_qg_dn7, locals.var_qg_dn8, locals.var_qg_dn9, locals.var_qg_dn10, locals.var_qg_dn13,)
    }
};
        locals.var_qg = assign106130_e158428;
        locals.var_qg_dn0 = assign106130_e158428_d_n0;
        locals.var_qg_dn2 = assign106130_e158428_d_n2;
        locals.var_qg_dn4 = assign106130_e158428_d_n4;
        locals.var_qg_dn5 = assign106130_e158428_d_n5;
        locals.var_qg_dn6 = assign106130_e158428_d_n6;
        locals.var_qg_dn7 = assign106130_e158428_d_n7;
        locals.var_qg_dn8 = assign106130_e158428_d_n8;
        locals.var_qg_dn9 = assign106130_e158428_d_n9;
        locals.var_qg_dn10 = assign106130_e158428_d_n10;
        locals.var_qg_dn13 = assign106130_e158428_d_n13;

        let (assign106140_e158433, assign106140_e158433_d_n0, assign106140_e158433_d_n2, assign106140_e158433_d_n4, assign106140_e158433_d_n5, assign106140_e158433_d_n6, assign106140_e158433_d_n7, assign106140_e158433_d_n8, assign106140_e158433_d_n9, assign106140_e158433_d_n10, assign106140_e158433_d_n13,) = {
    if (locals.var_guard2399 == 0.0) {
        (locals.var_qde, locals.var_qde_dn0, locals.var_qde_dn2, locals.var_qde_dn4, locals.var_qde_dn5, locals.var_qde_dn6, locals.var_qde_dn7, locals.var_qde_dn8, locals.var_qde_dn9, locals.var_qde_dn10, locals.var_qde_dn13,)
    } else {
        (locals.var_qs, locals.var_qs_dn0, locals.var_qs_dn2, locals.var_qs_dn4, locals.var_qs_dn5, locals.var_qs_dn6, locals.var_qs_dn7, locals.var_qs_dn8, locals.var_qs_dn9, locals.var_qs_dn10, locals.var_qs_dn13,)
    }
};
        locals.var_qs = assign106140_e158433;
        locals.var_qs_dn0 = assign106140_e158433_d_n0;
        locals.var_qs_dn2 = assign106140_e158433_d_n2;
        locals.var_qs_dn4 = assign106140_e158433_d_n4;
        locals.var_qs_dn5 = assign106140_e158433_d_n5;
        locals.var_qs_dn6 = assign106140_e158433_d_n6;
        locals.var_qs_dn7 = assign106140_e158433_d_n7;
        locals.var_qs_dn8 = assign106140_e158433_d_n8;
        locals.var_qs_dn9 = assign106140_e158433_d_n9;
        locals.var_qs_dn10 = assign106140_e158433_d_n10;
        locals.var_qs_dn13 = assign106140_e158433_d_n13;

        let (assign106150_e158443, assign106150_e158443_d_n0, assign106150_e158443_d_n2, assign106150_e158443_d_n4, assign106150_e158443_d_n5, assign106150_e158443_d_n6, assign106150_e158443_d_n7, assign106150_e158443_d_n8, assign106150_e158443_d_n9, assign106150_e158443_d_n10, assign106150_e158443_d_n13,) = {
    if (locals.var_guard2399 == 0.0) {
        let assign106150_e158438: f64 = (locals.var_qge + locals.var_qde);
        let assign106150_e158440: f64 = (assign106150_e158438 + locals.var_qse);
        let assign106150_e158441: f64 = (-assign106150_e158440);
        (assign106150_e158441, (-((locals.var_qge_dn0 + locals.var_qde_dn0) + locals.var_qse_dn0)), (-((locals.var_qge_dn2 + locals.var_qde_dn2) + locals.var_qse_dn2)), (-((locals.var_qge_dn4 + locals.var_qde_dn4) + locals.var_qse_dn4)), (-((locals.var_qge_dn5 + locals.var_qde_dn5) + locals.var_qse_dn5)), (-((locals.var_qge_dn6 + locals.var_qde_dn6) + locals.var_qse_dn6)), (-((locals.var_qge_dn7 + locals.var_qde_dn7) + locals.var_qse_dn7)), (-((locals.var_qge_dn8 + locals.var_qde_dn8) + locals.var_qse_dn8)), (-((locals.var_qge_dn9 + locals.var_qde_dn9) + locals.var_qse_dn9)), (-((locals.var_qge_dn10 + locals.var_qde_dn10) + locals.var_qse_dn10)), (-((locals.var_qge_dn13 + locals.var_qde_dn13) + locals.var_qse_dn13)),)
    } else {
        (locals.var_qb, locals.var_qb_dn0, locals.var_qb_dn2, locals.var_qb_dn4, locals.var_qb_dn5, locals.var_qb_dn6, locals.var_qb_dn7, locals.var_qb_dn8, locals.var_qb_dn9, locals.var_qb_dn10, locals.var_qb_dn13,)
    }
};
        locals.var_qb = assign106150_e158443;
        locals.var_qb_dn0 = assign106150_e158443_d_n0;
        locals.var_qb_dn2 = assign106150_e158443_d_n2;
        locals.var_qb_dn4 = assign106150_e158443_d_n4;
        locals.var_qb_dn5 = assign106150_e158443_d_n5;
        locals.var_qb_dn6 = assign106150_e158443_d_n6;
        locals.var_qb_dn7 = assign106150_e158443_d_n7;
        locals.var_qb_dn8 = assign106150_e158443_d_n8;
        locals.var_qb_dn9 = assign106150_e158443_d_n9;
        locals.var_qb_dn10 = assign106150_e158443_d_n10;
        locals.var_qb_dn13 = assign106150_e158443_d_n13;

        let (assign106160_e158448, assign106160_e158448_d_n0, assign106160_e158448_d_n2, assign106160_e158448_d_n4, assign106160_e158448_d_n5, assign106160_e158448_d_n6, assign106160_e158448_d_n7, assign106160_e158448_d_n8, assign106160_e158448_d_n9, assign106160_e158448_d_n10, assign106160_e158448_d_n13,) = {
    if (locals.var_guard2399 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_isub, locals.var_isub_dn0, locals.var_isub_dn2, locals.var_isub_dn4, locals.var_isub_dn5, locals.var_isub_dn6, locals.var_isub_dn7, locals.var_isub_dn8, locals.var_isub_dn9, locals.var_isub_dn10, locals.var_isub_dn13,)
    }
};
        locals.var_isub = assign106160_e158448;
        locals.var_isub_dn0 = assign106160_e158448_d_n0;
        locals.var_isub_dn2 = assign106160_e158448_d_n2;
        locals.var_isub_dn4 = assign106160_e158448_d_n4;
        locals.var_isub_dn5 = assign106160_e158448_d_n5;
        locals.var_isub_dn6 = assign106160_e158448_d_n6;
        locals.var_isub_dn7 = assign106160_e158448_d_n7;
        locals.var_isub_dn8 = assign106160_e158448_d_n8;
        locals.var_isub_dn9 = assign106160_e158448_d_n9;
        locals.var_isub_dn10 = assign106160_e158448_d_n10;
        locals.var_isub_dn13 = assign106160_e158448_d_n13;

        let (assign106180_e158458, assign106180_e158458_d_n0, assign106180_e158458_d_n2, assign106180_e158458_d_n4, assign106180_e158458_d_n5, assign106180_e158458_d_n6, assign106180_e158458_d_n7, assign106180_e158458_d_n8, assign106180_e158458_d_n9, assign106180_e158458_d_n10, assign106180_e158458_d_n13,) = {
    if (locals.var_guard2399 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_isubld, locals.var_isubld_dn0, locals.var_isubld_dn2, locals.var_isubld_dn4, locals.var_isubld_dn5, locals.var_isubld_dn6, locals.var_isubld_dn7, locals.var_isubld_dn8, locals.var_isubld_dn9, locals.var_isubld_dn10, locals.var_isubld_dn13,)
    }
};
        locals.var_isubld = assign106180_e158458;
        locals.var_isubld_dn0 = assign106180_e158458_d_n0;
        locals.var_isubld_dn2 = assign106180_e158458_d_n2;
        locals.var_isubld_dn4 = assign106180_e158458_d_n4;
        locals.var_isubld_dn5 = assign106180_e158458_d_n5;
        locals.var_isubld_dn6 = assign106180_e158458_d_n6;
        locals.var_isubld_dn7 = assign106180_e158458_d_n7;
        locals.var_isubld_dn8 = assign106180_e158458_d_n8;
        locals.var_isubld_dn9 = assign106180_e158458_d_n9;
        locals.var_isubld_dn10 = assign106180_e158458_d_n10;
        locals.var_isubld_dn13 = assign106180_e158458_d_n13;

        let (assign106200_e158468, assign106200_e158468_d_n0, assign106200_e158468_d_n2, assign106200_e158468_d_n4, assign106200_e158468_d_n5, assign106200_e158468_d_n6, assign106200_e158468_d_n7, assign106200_e158468_d_n8, assign106200_e158468_d_n9, assign106200_e158468_d_n10, assign106200_e158468_d_n13,) = {
    if (locals.var_guard2399 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_idsibpc, locals.var_idsibpc_dn0, locals.var_idsibpc_dn2, locals.var_idsibpc_dn4, locals.var_idsibpc_dn5, locals.var_idsibpc_dn6, locals.var_idsibpc_dn7, locals.var_idsibpc_dn8, locals.var_idsibpc_dn9, locals.var_idsibpc_dn10, locals.var_idsibpc_dn13,)
    }
};
        locals.var_idsibpc = assign106200_e158468;
        locals.var_idsibpc_dn0 = assign106200_e158468_d_n0;
        locals.var_idsibpc_dn2 = assign106200_e158468_d_n2;
        locals.var_idsibpc_dn4 = assign106200_e158468_d_n4;
        locals.var_idsibpc_dn5 = assign106200_e158468_d_n5;
        locals.var_idsibpc_dn6 = assign106200_e158468_d_n6;
        locals.var_idsibpc_dn7 = assign106200_e158468_d_n7;
        locals.var_idsibpc_dn8 = assign106200_e158468_d_n8;
        locals.var_idsibpc_dn9 = assign106200_e158468_d_n9;
        locals.var_idsibpc_dn10 = assign106200_e158468_d_n10;
        locals.var_idsibpc_dn13 = assign106200_e158468_d_n13;

        let (assign106280_e158512, assign106280_e158512_d_n0, assign106280_e158512_d_n2, assign106280_e158512_d_n4, assign106280_e158512_d_n5, assign106280_e158512_d_n6, assign106280_e158512_d_n7, assign106280_e158512_d_n8, assign106280_e158512_d_n9, assign106280_e158512_d_n10, assign106280_e158512_d_n13,) = {
    if ((locals.var_guard2399 == 0.0) && (locals.var_flg_nqs != 0.0)) {
        let assign106280_e158510: f64 = (1.0 - locals.var_xd);
        (assign106280_e158510, (-locals.var_xd_dn0), (-locals.var_xd_dn2), (-locals.var_xd_dn4), (-locals.var_xd_dn5), (-locals.var_xd_dn6), (-locals.var_xd_dn7), (-locals.var_xd_dn8), (-locals.var_xd_dn9), (-locals.var_xd_dn10), (-locals.var_xd_dn13),)
    } else {
        (locals.var_qdrat, locals.var_qdrat_dn0, locals.var_qdrat_dn2, locals.var_qdrat_dn4, locals.var_qdrat_dn5, locals.var_qdrat_dn6, locals.var_qdrat_dn7, locals.var_qdrat_dn8, locals.var_qdrat_dn9, locals.var_qdrat_dn10, locals.var_qdrat_dn13,)
    }
};
        locals.var_qdrat = assign106280_e158512;
        locals.var_qdrat_dn0 = assign106280_e158512_d_n0;
        locals.var_qdrat_dn2 = assign106280_e158512_d_n2;
        locals.var_qdrat_dn4 = assign106280_e158512_d_n4;
        locals.var_qdrat_dn5 = assign106280_e158512_d_n5;
        locals.var_qdrat_dn6 = assign106280_e158512_d_n6;
        locals.var_qdrat_dn7 = assign106280_e158512_d_n7;
        locals.var_qdrat_dn8 = assign106280_e158512_d_n8;
        locals.var_qdrat_dn9 = assign106280_e158512_d_n9;
        locals.var_qdrat_dn10 = assign106280_e158512_d_n10;
        locals.var_qdrat_dn13 = assign106280_e158512_d_n13;

        let assign106290_e158515: f64 = (locals.var_qg + locals.var_qgov);
        locals.var_qg = assign106290_e158515;
        locals.var_qg_dn0 = (locals.var_qg_dn0 + locals.var_qgov_dn0);
        locals.var_qg_dn2 = (locals.var_qg_dn2 + locals.var_qgov_dn2);
        locals.var_qg_dn4 = (locals.var_qg_dn4 + locals.var_qgov_dn4);
        locals.var_qg_dn5 = (locals.var_qg_dn5 + locals.var_qgov_dn5);
        locals.var_qg_dn6 = (locals.var_qg_dn6 + locals.var_qgov_dn6);
        locals.var_qg_dn7 = (locals.var_qg_dn7 + locals.var_qgov_dn7);
        locals.var_qg_dn8 = (locals.var_qg_dn8 + locals.var_qgov_dn8);
        locals.var_qg_dn9 = (locals.var_qg_dn9 + locals.var_qgov_dn9);
        locals.var_qg_dn10 = (locals.var_qg_dn10 + locals.var_qgov_dn10);
        locals.var_qg_dn13 = (locals.var_qg_dn13 + locals.var_qgov_dn13);

        let assign106300_e158518: f64 = (locals.var_qd + locals.var_qdov);
        locals.var_qd = assign106300_e158518;
        locals.var_qd_dn0 = (locals.var_qd_dn0 + locals.var_qdov_dn0);
        locals.var_qd_dn2 = (locals.var_qd_dn2 + locals.var_qdov_dn2);
        locals.var_qd_dn4 = (locals.var_qd_dn4 + locals.var_qdov_dn4);
        locals.var_qd_dn5 = (locals.var_qd_dn5 + locals.var_qdov_dn5);
        locals.var_qd_dn6 = (locals.var_qd_dn6 + locals.var_qdov_dn6);
        locals.var_qd_dn7 = (locals.var_qd_dn7 + locals.var_qdov_dn7);
        locals.var_qd_dn8 = (locals.var_qd_dn8 + locals.var_qdov_dn8);
        locals.var_qd_dn9 = (locals.var_qd_dn9 + locals.var_qdov_dn9);
        locals.var_qd_dn10 = (locals.var_qd_dn10 + locals.var_qdov_dn10);
        locals.var_qd_dn13 = (locals.var_qd_dn13 + locals.var_qdov_dn13);

        let assign106310_e158521: f64 = (locals.var_qs + locals.var_qsov);
        locals.var_qs = assign106310_e158521;
        locals.var_qs_dn0 = (locals.var_qs_dn0 + locals.var_qsov_dn0);
        locals.var_qs_dn2 = (locals.var_qs_dn2 + locals.var_qsov_dn2);
        locals.var_qs_dn4 = (locals.var_qs_dn4 + locals.var_qsov_dn4);
        locals.var_qs_dn5 = (locals.var_qs_dn5 + locals.var_qsov_dn5);
        locals.var_qs_dn6 = (locals.var_qs_dn6 + locals.var_qsov_dn6);
        locals.var_qs_dn7 = (locals.var_qs_dn7 + locals.var_qsov_dn7);
        locals.var_qs_dn8 = (locals.var_qs_dn8 + locals.var_qsov_dn8);
        locals.var_qs_dn9 = (locals.var_qs_dn9 + locals.var_qsov_dn9);
        locals.var_qs_dn10 = (locals.var_qs_dn10 + locals.var_qsov_dn10);
        locals.var_qs_dn13 = (locals.var_qs_dn13 + locals.var_qsov_dn13);

        let assign106320_e158524: f64 = (locals.var_qg + locals.var_qd);
        let assign106320_e158526: f64 = (assign106320_e158524 + locals.var_qs);
        let assign106320_e158527: f64 = (-assign106320_e158526);
        locals.var_qb = assign106320_e158527;
        locals.var_qb_dn0 = (-((locals.var_qg_dn0 + locals.var_qd_dn0) + locals.var_qs_dn0));
        locals.var_qb_dn2 = (-((locals.var_qg_dn2 + locals.var_qd_dn2) + locals.var_qs_dn2));
        locals.var_qb_dn4 = (-((locals.var_qg_dn4 + locals.var_qd_dn4) + locals.var_qs_dn4));
        locals.var_qb_dn5 = (-((locals.var_qg_dn5 + locals.var_qd_dn5) + locals.var_qs_dn5));
        locals.var_qb_dn6 = (-((locals.var_qg_dn6 + locals.var_qd_dn6) + locals.var_qs_dn6));
        locals.var_qb_dn7 = (-((locals.var_qg_dn7 + locals.var_qd_dn7) + locals.var_qs_dn7));
        locals.var_qb_dn8 = (-((locals.var_qg_dn8 + locals.var_qd_dn8) + locals.var_qs_dn8));
        locals.var_qb_dn9 = (-((locals.var_qg_dn9 + locals.var_qd_dn9) + locals.var_qs_dn9));
        locals.var_qb_dn10 = (-((locals.var_qg_dn10 + locals.var_qd_dn10) + locals.var_qs_dn10));
        locals.var_qb_dn13 = (-((locals.var_qg_dn13 + locals.var_qd_dn13) + locals.var_qs_dn13));

        locals.var_qfd = locals.var_qdp;
        locals.var_qfd_dn0 = locals.var_qdp_dn0;
        locals.var_qfd_dn2 = locals.var_qdp_dn2;
        locals.var_qfd_dn6 = locals.var_qdp_dn6;

        locals.var_qfs = locals.var_qsp;
        locals.var_qfs_dn2 = locals.var_qsp_dn2;
        locals.var_qfs_dn6 = locals.var_qsp_dn6;

        locals.var_qdext = locals.var_qdexte;
        locals.var_qdext_dn0 = locals.var_qdexte_dn0;
        locals.var_qdext_dn2 = locals.var_qdexte_dn2;
        locals.var_qdext_dn4 = locals.var_qdexte_dn4;
        locals.var_qdext_dn5 = locals.var_qdexte_dn5;
        locals.var_qdext_dn6 = locals.var_qdexte_dn6;
        locals.var_qdext_dn7 = locals.var_qdexte_dn7;
        locals.var_qdext_dn8 = locals.var_qdexte_dn8;
        locals.var_qdext_dn9 = locals.var_qdexte_dn9;
        locals.var_qdext_dn10 = locals.var_qdexte_dn10;
        locals.var_qdext_dn13 = locals.var_qdexte_dn13;

        locals.var_qgext = locals.var_qgexte;
        locals.var_qgext_dn0 = locals.var_qgexte_dn0;
        locals.var_qgext_dn2 = locals.var_qgexte_dn2;
        locals.var_qgext_dn4 = locals.var_qgexte_dn4;
        locals.var_qgext_dn5 = locals.var_qgexte_dn5;
        locals.var_qgext_dn6 = locals.var_qgexte_dn6;
        locals.var_qgext_dn7 = locals.var_qgexte_dn7;
        locals.var_qgext_dn8 = locals.var_qgexte_dn8;
        locals.var_qgext_dn9 = locals.var_qgexte_dn9;
        locals.var_qgext_dn10 = locals.var_qgexte_dn10;
        locals.var_qgext_dn13 = locals.var_qgexte_dn13;

        let assign106370_e158534: f64 = (locals.var_qgexte + locals.var_qdexte);
        let assign106370_e158536: f64 = (assign106370_e158534 + locals.var_qsexte);
        let assign106370_e158537: f64 = (-assign106370_e158536);
        locals.var_qbext = assign106370_e158537;
        locals.var_qbext_dn0 = (-((locals.var_qgexte_dn0 + locals.var_qdexte_dn0) + locals.var_qsexte_dn0));
        locals.var_qbext_dn2 = (-((locals.var_qgexte_dn2 + locals.var_qdexte_dn2) + locals.var_qsexte_dn2));
        locals.var_qbext_dn4 = (-((locals.var_qgexte_dn4 + locals.var_qdexte_dn4) + locals.var_qsexte_dn4));
        locals.var_qbext_dn5 = (-((locals.var_qgexte_dn5 + locals.var_qdexte_dn5) + locals.var_qsexte_dn5));
        locals.var_qbext_dn6 = (-((locals.var_qgexte_dn6 + locals.var_qdexte_dn6) + locals.var_qsexte_dn6));
        locals.var_qbext_dn7 = (-((locals.var_qgexte_dn7 + locals.var_qdexte_dn7) + locals.var_qsexte_dn7));
        locals.var_qbext_dn8 = (-((locals.var_qgexte_dn8 + locals.var_qdexte_dn8) + locals.var_qsexte_dn8));
        locals.var_qbext_dn9 = (-((locals.var_qgexte_dn9 + locals.var_qdexte_dn9) + locals.var_qsexte_dn9));
        locals.var_qbext_dn10 = (-((locals.var_qgexte_dn10 + locals.var_qdexte_dn10) + locals.var_qsexte_dn10));
        locals.var_qbext_dn13 = (-((locals.var_qgexte_dn13 + locals.var_qdexte_dn13) + locals.var_qsexte_dn13));

        let assign106380_e158540: f64 = if p.p53 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2400 = assign106380_e158540;

        let assign106390_e158543: f64 = if locals.var_rth > 0.0001 { 1.0 } else { 0.0 };
        locals.var_guard2401 = assign106390_e158543;

        let (assign106400_e158551, assign106400_e158551_d_n0, assign106400_e158551_d_n2, assign106400_e158551_d_n4, assign106400_e158551_d_n5, assign106400_e158551_d_n6, assign106400_e158551_d_n7, assign106400_e158551_d_n8, assign106400_e158551_d_n9, assign106400_e158551_d_n10, assign106400_e158551_d_n13,) = {
    if ((locals.var_guard2400 != 0.0) && (locals.var_guard2401 != 0.0)) {
        let assign106400_e158549: f64 = (1.0 / locals.var_rth);
        (assign106400_e158549, (-(locals.var_rth_dn0 / (locals.var_rth * locals.var_rth))), (-(locals.var_rth_dn2 / (locals.var_rth * locals.var_rth))), (-(locals.var_rth_dn4 / (locals.var_rth * locals.var_rth))), (-(locals.var_rth_dn5 / (locals.var_rth * locals.var_rth))), (-(locals.var_rth_dn6 / (locals.var_rth * locals.var_rth))), (-(locals.var_rth_dn7 / (locals.var_rth * locals.var_rth))), (-(locals.var_rth_dn8 / (locals.var_rth * locals.var_rth))), (-(locals.var_rth_dn9 / (locals.var_rth * locals.var_rth))), (-(locals.var_rth_dn10 / (locals.var_rth * locals.var_rth))), (-(locals.var_rth_dn13 / (locals.var_rth * locals.var_rth))),)
    } else {
        (locals.var_gth, locals.var_gth_dn0, locals.var_gth_dn2, locals.var_gth_dn4, locals.var_gth_dn5, locals.var_gth_dn6, locals.var_gth_dn7, locals.var_gth_dn8, locals.var_gth_dn9, locals.var_gth_dn10, locals.var_gth_dn13,)
    }
};
        locals.var_gth = assign106400_e158551;
        locals.var_gth_dn0 = assign106400_e158551_d_n0;
        locals.var_gth_dn2 = assign106400_e158551_d_n2;
        locals.var_gth_dn4 = assign106400_e158551_d_n4;
        locals.var_gth_dn5 = assign106400_e158551_d_n5;
        locals.var_gth_dn6 = assign106400_e158551_d_n6;
        locals.var_gth_dn7 = assign106400_e158551_d_n7;
        locals.var_gth_dn8 = assign106400_e158551_d_n8;
        locals.var_gth_dn9 = assign106400_e158551_d_n9;
        locals.var_gth_dn10 = assign106400_e158551_d_n10;
        locals.var_gth_dn13 = assign106400_e158551_d_n13;

        let (assign106410_e158560, assign106410_e158560_d_n0, assign106410_e158560_d_n2, assign106410_e158560_d_n4, assign106410_e158560_d_n5, assign106410_e158560_d_n6, assign106410_e158560_d_n7, assign106410_e158560_d_n8, assign106410_e158560_d_n9, assign106410_e158560_d_n10, assign106410_e158560_d_n13,) = {
    if ((locals.var_guard2400 != 0.0) && (locals.var_guard2401 == 0.0)) {
        let assign106410_e158558: f64 = (1.0 / 0.0001);
        (assign106410_e158558, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_gth, locals.var_gth_dn0, locals.var_gth_dn2, locals.var_gth_dn4, locals.var_gth_dn5, locals.var_gth_dn6, locals.var_gth_dn7, locals.var_gth_dn8, locals.var_gth_dn9, locals.var_gth_dn10, locals.var_gth_dn13,)
    }
};
        locals.var_gth = assign106410_e158560;
        locals.var_gth_dn0 = assign106410_e158560_d_n0;
        locals.var_gth_dn2 = assign106410_e158560_d_n2;
        locals.var_gth_dn4 = assign106410_e158560_d_n4;
        locals.var_gth_dn5 = assign106410_e158560_d_n5;
        locals.var_gth_dn6 = assign106410_e158560_d_n6;
        locals.var_gth_dn7 = assign106410_e158560_d_n7;
        locals.var_gth_dn8 = assign106410_e158560_d_n8;
        locals.var_gth_dn9 = assign106410_e158560_d_n9;
        locals.var_gth_dn10 = assign106410_e158560_d_n10;
        locals.var_gth_dn13 = assign106410_e158560_d_n13;

        let assign106420_e158564: f64 = (locals.var_vdsei - locals.var_vdsi);
        let assign106420_e158565: f64 = (locals.var_vdsi * assign106420_e158564);
        let assign106420_e158567: f64 = if assign106420_e158565 >= 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2402 = assign106420_e158567;

        let assign106430_e158570: f64 = if locals.var_uc_powrat == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard2403 = assign106430_e158570;

        let (assign106440_e158578, assign106440_e158578_d_n0, assign106440_e158578_d_n2, assign106440_e158578_d_n4, assign106440_e158578_d_n5, assign106440_e158578_d_n6, assign106440_e158578_d_n7, assign106440_e158578_d_n8, assign106440_e158578_d_n9, assign106440_e158578_d_n10, assign106440_e158578_d_n13,) = {
    if (((locals.var_guard2400 != 0.0) && (locals.var_guard2402 != 0.0)) && (locals.var_guard2403 != 0.0)) {
        (locals.var_vdsei, locals.var_vdsei_dn0, locals.var_vdsei_dn2, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_veffpower, locals.var_veffpower_dn0, locals.var_veffpower_dn2, locals.var_veffpower_dn4, locals.var_veffpower_dn5, locals.var_veffpower_dn6, locals.var_veffpower_dn7, locals.var_veffpower_dn8, locals.var_veffpower_dn9, locals.var_veffpower_dn10, locals.var_veffpower_dn13,)
    }
};
        locals.var_veffpower = assign106440_e158578;
        locals.var_veffpower_dn0 = assign106440_e158578_d_n0;
        locals.var_veffpower_dn2 = assign106440_e158578_d_n2;
        locals.var_veffpower_dn4 = assign106440_e158578_d_n4;
        locals.var_veffpower_dn5 = assign106440_e158578_d_n5;
        locals.var_veffpower_dn6 = assign106440_e158578_d_n6;
        locals.var_veffpower_dn7 = assign106440_e158578_d_n7;
        locals.var_veffpower_dn8 = assign106440_e158578_d_n8;
        locals.var_veffpower_dn9 = assign106440_e158578_d_n9;
        locals.var_veffpower_dn10 = assign106440_e158578_d_n10;
        locals.var_veffpower_dn13 = assign106440_e158578_d_n13;

        let (assign106450_e158593, assign106450_e158593_d_n0, assign106450_e158593_d_n2, assign106450_e158593_d_n4, assign106450_e158593_d_n5, assign106450_e158593_d_n6, assign106450_e158593_d_n7, assign106450_e158593_d_n8, assign106450_e158593_d_n9, assign106450_e158593_d_n10, assign106450_e158593_d_n13,) = {
    if (((locals.var_guard2400 != 0.0) && (locals.var_guard2402 != 0.0)) && (locals.var_guard2403 == 0.0)) {
        let assign106450_e158589: f64 = (locals.var_vdsei - locals.var_vdsi);
        let assign106450_e158590: f64 = (locals.var_powratio * assign106450_e158589);
        let assign106450_e158591: f64 = (locals.var_vdsi + assign106450_e158590);
        (assign106450_e158591, ((locals.var_powratio_dn0 * assign106450_e158589) + (locals.var_powratio * locals.var_vdsei_dn0)), ((locals.var_powratio_dn2 * assign106450_e158589) + (locals.var_powratio * locals.var_vdsei_dn2)), (locals.var_powratio_dn4 * assign106450_e158589), (locals.var_vdsi_dn5 + ((locals.var_powratio_dn5 * assign106450_e158589) + (locals.var_powratio * (-locals.var_vdsi_dn5)))), (locals.var_powratio_dn6 * assign106450_e158589), (locals.var_vdsi_dn7 + ((locals.var_powratio_dn7 * assign106450_e158589) + (locals.var_powratio * (-locals.var_vdsi_dn7)))), (locals.var_powratio_dn8 * assign106450_e158589), (locals.var_powratio_dn9 * assign106450_e158589), (locals.var_powratio_dn10 * assign106450_e158589), (locals.var_powratio_dn13 * assign106450_e158589),)
    } else {
        (locals.var_veffpower, locals.var_veffpower_dn0, locals.var_veffpower_dn2, locals.var_veffpower_dn4, locals.var_veffpower_dn5, locals.var_veffpower_dn6, locals.var_veffpower_dn7, locals.var_veffpower_dn8, locals.var_veffpower_dn9, locals.var_veffpower_dn10, locals.var_veffpower_dn13,)
    }
};
        locals.var_veffpower = assign106450_e158593;
        locals.var_veffpower_dn0 = assign106450_e158593_d_n0;
        locals.var_veffpower_dn2 = assign106450_e158593_d_n2;
        locals.var_veffpower_dn4 = assign106450_e158593_d_n4;
        locals.var_veffpower_dn5 = assign106450_e158593_d_n5;
        locals.var_veffpower_dn6 = assign106450_e158593_d_n6;
        locals.var_veffpower_dn7 = assign106450_e158593_d_n7;
        locals.var_veffpower_dn8 = assign106450_e158593_d_n8;
        locals.var_veffpower_dn9 = assign106450_e158593_d_n9;
        locals.var_veffpower_dn10 = assign106450_e158593_d_n10;
        locals.var_veffpower_dn13 = assign106450_e158593_d_n13;

    }

    pub(super) fn stamp_transient_block_374(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign106460_e158600, assign106460_e158600_d_n0, assign106460_e158600_d_n2, assign106460_e158600_d_n4, assign106460_e158600_d_n5, assign106460_e158600_d_n6, assign106460_e158600_d_n7, assign106460_e158600_d_n8, assign106460_e158600_d_n9, assign106460_e158600_d_n10, assign106460_e158600_d_n13,) = {
    if ((locals.var_guard2400 != 0.0) && (locals.var_guard2402 == 0.0)) {
        (locals.var_vdsi, 0.0, 0.0, 0.0, locals.var_vdsi_dn5, 0.0, locals.var_vdsi_dn7, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_veffpower, locals.var_veffpower_dn0, locals.var_veffpower_dn2, locals.var_veffpower_dn4, locals.var_veffpower_dn5, locals.var_veffpower_dn6, locals.var_veffpower_dn7, locals.var_veffpower_dn8, locals.var_veffpower_dn9, locals.var_veffpower_dn10, locals.var_veffpower_dn13,)
    }
};
        locals.var_veffpower = assign106460_e158600;
        locals.var_veffpower_dn0 = assign106460_e158600_d_n0;
        locals.var_veffpower_dn2 = assign106460_e158600_d_n2;
        locals.var_veffpower_dn4 = assign106460_e158600_d_n4;
        locals.var_veffpower_dn5 = assign106460_e158600_d_n5;
        locals.var_veffpower_dn6 = assign106460_e158600_d_n6;
        locals.var_veffpower_dn7 = assign106460_e158600_d_n7;
        locals.var_veffpower_dn8 = assign106460_e158600_d_n8;
        locals.var_veffpower_dn9 = assign106460_e158600_d_n9;
        locals.var_veffpower_dn10 = assign106460_e158600_d_n10;
        locals.var_veffpower_dn13 = assign106460_e158600_d_n13;

        let (assign106470_e158606, assign106470_e158606_d_n0, assign106470_e158606_d_n2, assign106470_e158606_d_n4, assign106470_e158606_d_n5, assign106470_e158606_d_n6, assign106470_e158606_d_n7, assign106470_e158606_d_n8, assign106470_e158606_d_n9, assign106470_e158606_d_n10, assign106470_e158606_d_n13,) = {
    if (locals.var_guard2400 != 0.0) {
        let assign106470_e158604: f64 = (locals.var_ids * locals.var_veffpower);
        (assign106470_e158604, ((locals.var_ids_dn0 * locals.var_veffpower) + (locals.var_ids * locals.var_veffpower_dn0)), ((locals.var_ids_dn2 * locals.var_veffpower) + (locals.var_ids * locals.var_veffpower_dn2)), ((locals.var_ids_dn4 * locals.var_veffpower) + (locals.var_ids * locals.var_veffpower_dn4)), ((locals.var_ids_dn5 * locals.var_veffpower) + (locals.var_ids * locals.var_veffpower_dn5)), ((locals.var_ids_dn6 * locals.var_veffpower) + (locals.var_ids * locals.var_veffpower_dn6)), ((locals.var_ids_dn7 * locals.var_veffpower) + (locals.var_ids * locals.var_veffpower_dn7)), ((locals.var_ids_dn8 * locals.var_veffpower) + (locals.var_ids * locals.var_veffpower_dn8)), ((locals.var_ids_dn9 * locals.var_veffpower) + (locals.var_ids * locals.var_veffpower_dn9)), ((locals.var_ids_dn10 * locals.var_veffpower) + (locals.var_ids * locals.var_veffpower_dn10)), ((locals.var_ids_dn13 * locals.var_veffpower) + (locals.var_ids * locals.var_veffpower_dn13)),)
    } else {
        (locals.var_p, locals.var_p_dn0, locals.var_p_dn2, locals.var_p_dn4, locals.var_p_dn5, locals.var_p_dn6, locals.var_p_dn7, locals.var_p_dn8, locals.var_p_dn9, locals.var_p_dn10, locals.var_p_dn13,)
    }
};
        locals.var_p = assign106470_e158606;
        locals.var_p_dn0 = assign106470_e158606_d_n0;
        locals.var_p_dn2 = assign106470_e158606_d_n2;
        locals.var_p_dn4 = assign106470_e158606_d_n4;
        locals.var_p_dn5 = assign106470_e158606_d_n5;
        locals.var_p_dn6 = assign106470_e158606_d_n6;
        locals.var_p_dn7 = assign106470_e158606_d_n7;
        locals.var_p_dn8 = assign106470_e158606_d_n8;
        locals.var_p_dn9 = assign106470_e158606_d_n9;
        locals.var_p_dn10 = assign106470_e158606_d_n10;
        locals.var_p_dn13 = assign106470_e158606_d_n13;

        let assign106480_e158609: f64 = if p.p53 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard2404 = assign106480_e158609;

        let (assign106490_e158617, assign106490_e158617_d_n0, assign106490_e158617_d_n2, assign106490_e158617_d_n4, assign106490_e158617_d_n5, assign106490_e158617_d_n6, assign106490_e158617_d_n7, assign106490_e158617_d_n8, assign106490_e158617_d_n9, assign106490_e158617_d_n10, assign106490_e158617_d_n13,) = {
    if ((locals.var_guard2400 != 0.0) && (locals.var_guard2404 != 0.0)) {
        let assign106490_e158615: f64 = (p.p433 * locals.var_gth);
        (assign106490_e158615, (p.p433 * locals.var_gth_dn0), (p.p433 * locals.var_gth_dn2), (p.p433 * locals.var_gth_dn4), (p.p433 * locals.var_gth_dn5), (p.p433 * locals.var_gth_dn6), (p.p433 * locals.var_gth_dn7), (p.p433 * locals.var_gth_dn8), (p.p433 * locals.var_gth_dn9), (p.p433 * locals.var_gth_dn10), (p.p433 * locals.var_gth_dn13),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign106490_e158617;
        locals.var_t1_dn0 = assign106490_e158617_d_n0;
        locals.var_t1_dn2 = assign106490_e158617_d_n2;
        locals.var_t1_dn4 = assign106490_e158617_d_n4;
        locals.var_t1_dn5 = assign106490_e158617_d_n5;
        locals.var_t1_dn6 = assign106490_e158617_d_n6;
        locals.var_t1_dn7 = assign106490_e158617_d_n7;
        locals.var_t1_dn8 = assign106490_e158617_d_n8;
        locals.var_t1_dn9 = assign106490_e158617_d_n9;
        locals.var_t1_dn10 = assign106490_e158617_d_n10;
        locals.var_t1_dn13 = assign106490_e158617_d_n13;

        let (assign106500_e158629, assign106500_e158629_d_n0, assign106500_e158629_d_n2, assign106500_e158629_d_n4, assign106500_e158629_d_n5, assign106500_e158629_d_n6, assign106500_e158629_d_n7, assign106500_e158629_d_n8, assign106500_e158629_d_n9, assign106500_e158629_d_n10, assign106500_e158629_d_n13,) = {
    if ((locals.var_guard2400 != 0.0) && (locals.var_guard2404 != 0.0)) {
        let assign106500_e158623: f64 = (locals.var_t1 - locals.var_p);
        let assign106500_e158626: f64 = (p.p337 * locals.var_gth);
        let assign106500_e158627: f64 = (assign106500_e158623 - assign106500_e158626);
        (assign106500_e158627, ((locals.var_t1_dn0 - locals.var_p_dn0) - (p.p337 * locals.var_gth_dn0)), ((locals.var_t1_dn2 - locals.var_p_dn2) - (p.p337 * locals.var_gth_dn2)), ((locals.var_t1_dn4 - locals.var_p_dn4) - (p.p337 * locals.var_gth_dn4)), ((locals.var_t1_dn5 - locals.var_p_dn5) - (p.p337 * locals.var_gth_dn5)), ((locals.var_t1_dn6 - locals.var_p_dn6) - (p.p337 * locals.var_gth_dn6)), ((locals.var_t1_dn7 - locals.var_p_dn7) - (p.p337 * locals.var_gth_dn7)), ((locals.var_t1_dn8 - locals.var_p_dn8) - (p.p337 * locals.var_gth_dn8)), ((locals.var_t1_dn9 - locals.var_p_dn9) - (p.p337 * locals.var_gth_dn9)), ((locals.var_t1_dn10 - locals.var_p_dn10) - (p.p337 * locals.var_gth_dn10)), ((locals.var_t1_dn13 - locals.var_p_dn13) - (p.p337 * locals.var_gth_dn13)),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13,)
    }
};
        locals.var_tmf1 = assign106500_e158629;
        locals.var_tmf1_dn0 = assign106500_e158629_d_n0;
        locals.var_tmf1_dn2 = assign106500_e158629_d_n2;
        locals.var_tmf1_dn4 = assign106500_e158629_d_n4;
        locals.var_tmf1_dn5 = assign106500_e158629_d_n5;
        locals.var_tmf1_dn6 = assign106500_e158629_d_n6;
        locals.var_tmf1_dn7 = assign106500_e158629_d_n7;
        locals.var_tmf1_dn8 = assign106500_e158629_d_n8;
        locals.var_tmf1_dn9 = assign106500_e158629_d_n9;
        locals.var_tmf1_dn10 = assign106500_e158629_d_n10;
        locals.var_tmf1_dn13 = assign106500_e158629_d_n13;

        let (assign106510_e158641, assign106510_e158641_d_n0, assign106510_e158641_d_n2, assign106510_e158641_d_n4, assign106510_e158641_d_n5, assign106510_e158641_d_n6, assign106510_e158641_d_n7, assign106510_e158641_d_n8, assign106510_e158641_d_n9, assign106510_e158641_d_n10, assign106510_e158641_d_n13,) = {
    if ((locals.var_guard2400 != 0.0) && (locals.var_guard2404 != 0.0)) {
        let assign106510_e158635: f64 = (4.0 * locals.var_t1);
        let assign106510_e158638: f64 = (p.p337 * locals.var_gth);
        let assign106510_e158639: f64 = (assign106510_e158635 * assign106510_e158638);
        (assign106510_e158639, (((4.0 * locals.var_t1_dn0) * assign106510_e158638) + (assign106510_e158635 * (p.p337 * locals.var_gth_dn0))), (((4.0 * locals.var_t1_dn2) * assign106510_e158638) + (assign106510_e158635 * (p.p337 * locals.var_gth_dn2))), (((4.0 * locals.var_t1_dn4) * assign106510_e158638) + (assign106510_e158635 * (p.p337 * locals.var_gth_dn4))), (((4.0 * locals.var_t1_dn5) * assign106510_e158638) + (assign106510_e158635 * (p.p337 * locals.var_gth_dn5))), (((4.0 * locals.var_t1_dn6) * assign106510_e158638) + (assign106510_e158635 * (p.p337 * locals.var_gth_dn6))), (((4.0 * locals.var_t1_dn7) * assign106510_e158638) + (assign106510_e158635 * (p.p337 * locals.var_gth_dn7))), (((4.0 * locals.var_t1_dn8) * assign106510_e158638) + (assign106510_e158635 * (p.p337 * locals.var_gth_dn8))), (((4.0 * locals.var_t1_dn9) * assign106510_e158638) + (assign106510_e158635 * (p.p337 * locals.var_gth_dn9))), (((4.0 * locals.var_t1_dn10) * assign106510_e158638) + (assign106510_e158635 * (p.p337 * locals.var_gth_dn10))), (((4.0 * locals.var_t1_dn13) * assign106510_e158638) + (assign106510_e158635 * (p.p337 * locals.var_gth_dn13))),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign106510_e158641;
        locals.var_tmf2_dn0 = assign106510_e158641_d_n0;
        locals.var_tmf2_dn2 = assign106510_e158641_d_n2;
        locals.var_tmf2_dn4 = assign106510_e158641_d_n4;
        locals.var_tmf2_dn5 = assign106510_e158641_d_n5;
        locals.var_tmf2_dn6 = assign106510_e158641_d_n6;
        locals.var_tmf2_dn7 = assign106510_e158641_d_n7;
        locals.var_tmf2_dn8 = assign106510_e158641_d_n8;
        locals.var_tmf2_dn9 = assign106510_e158641_d_n9;
        locals.var_tmf2_dn10 = assign106510_e158641_d_n10;
        locals.var_tmf2_dn13 = assign106510_e158641_d_n13;

        let (assign106520_e158653, assign106520_e158653_d_n0, assign106520_e158653_d_n2, assign106520_e158653_d_n4, assign106520_e158653_d_n5, assign106520_e158653_d_n6, assign106520_e158653_d_n7, assign106520_e158653_d_n8, assign106520_e158653_d_n9, assign106520_e158653_d_n10, assign106520_e158653_d_n13,) = {
    if ((locals.var_guard2400 != 0.0) && (locals.var_guard2404 != 0.0)) {
        let (assign106520_e158651, assign106520_e158651_d_n0, assign106520_e158651_d_n2, assign106520_e158651_d_n4, assign106520_e158651_d_n5, assign106520_e158651_d_n6, assign106520_e158651_d_n7, assign106520_e158651_d_n8, assign106520_e158651_d_n9, assign106520_e158651_d_n10, assign106520_e158651_d_n13,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
            } else {
                let assign106520_e158650: f64 = (-locals.var_tmf2);
                (assign106520_e158650, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn13),)
            }
        };
        (assign106520_e158651, assign106520_e158651_d_n0, assign106520_e158651_d_n2, assign106520_e158651_d_n4, assign106520_e158651_d_n5, assign106520_e158651_d_n6, assign106520_e158651_d_n7, assign106520_e158651_d_n8, assign106520_e158651_d_n9, assign106520_e158651_d_n10, assign106520_e158651_d_n13,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign106520_e158653;
        locals.var_tmf2_dn0 = assign106520_e158653_d_n0;
        locals.var_tmf2_dn2 = assign106520_e158653_d_n2;
        locals.var_tmf2_dn4 = assign106520_e158653_d_n4;
        locals.var_tmf2_dn5 = assign106520_e158653_d_n5;
        locals.var_tmf2_dn6 = assign106520_e158653_d_n6;
        locals.var_tmf2_dn7 = assign106520_e158653_d_n7;
        locals.var_tmf2_dn8 = assign106520_e158653_d_n8;
        locals.var_tmf2_dn9 = assign106520_e158653_d_n9;
        locals.var_tmf2_dn10 = assign106520_e158653_d_n10;
        locals.var_tmf2_dn13 = assign106520_e158653_d_n13;

        let (assign106530_e158664, assign106530_e158664_d_n0, assign106530_e158664_d_n2, assign106530_e158664_d_n4, assign106530_e158664_d_n5, assign106530_e158664_d_n6, assign106530_e158664_d_n7, assign106530_e158664_d_n8, assign106530_e158664_d_n9, assign106530_e158664_d_n10, assign106530_e158664_d_n13,) = {
    if ((locals.var_guard2400 != 0.0) && (locals.var_guard2404 != 0.0)) {
        let assign106530_e158659: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign106530_e158661: f64 = (assign106530_e158659 + locals.var_tmf2);
        let assign106530_e158662: f64 = (assign106530_e158661).sqrt();
        (assign106530_e158662, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign106530_e158662)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign106530_e158662)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign106530_e158662)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign106530_e158662)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign106530_e158662)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign106530_e158662)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign106530_e158662)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign106530_e158662)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign106530_e158662)), ((((locals.var_tmf1_dn13 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn13)) + locals.var_tmf2_dn13) / (2.0 * assign106530_e158662)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign106530_e158664;
        locals.var_tmf2_dn0 = assign106530_e158664_d_n0;
        locals.var_tmf2_dn2 = assign106530_e158664_d_n2;
        locals.var_tmf2_dn4 = assign106530_e158664_d_n4;
        locals.var_tmf2_dn5 = assign106530_e158664_d_n5;
        locals.var_tmf2_dn6 = assign106530_e158664_d_n6;
        locals.var_tmf2_dn7 = assign106530_e158664_d_n7;
        locals.var_tmf2_dn8 = assign106530_e158664_d_n8;
        locals.var_tmf2_dn9 = assign106530_e158664_d_n9;
        locals.var_tmf2_dn10 = assign106530_e158664_d_n10;
        locals.var_tmf2_dn13 = assign106530_e158664_d_n13;

        let (assign106540_e158676, assign106540_e158676_d_n0, assign106540_e158676_d_n2, assign106540_e158676_d_n4, assign106540_e158676_d_n5, assign106540_e158676_d_n6, assign106540_e158676_d_n7, assign106540_e158676_d_n8, assign106540_e158676_d_n9, assign106540_e158676_d_n10, assign106540_e158676_d_n13,) = {
    if ((locals.var_guard2400 != 0.0) && (locals.var_guard2404 != 0.0)) {
        let assign106540_e158672: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign106540_e158673: f64 = (1.0 + assign106540_e158672);
        let assign106540_e158674: f64 = (0.5 * assign106540_e158673);
        (assign106540_e158674, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn13 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn13)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign106540_e158676;
        locals.var_t0_dn0 = assign106540_e158676_d_n0;
        locals.var_t0_dn2 = assign106540_e158676_d_n2;
        locals.var_t0_dn4 = assign106540_e158676_d_n4;
        locals.var_t0_dn5 = assign106540_e158676_d_n5;
        locals.var_t0_dn6 = assign106540_e158676_d_n6;
        locals.var_t0_dn7 = assign106540_e158676_d_n7;
        locals.var_t0_dn8 = assign106540_e158676_d_n8;
        locals.var_t0_dn9 = assign106540_e158676_d_n9;
        locals.var_t0_dn10 = assign106540_e158676_d_n10;
        locals.var_t0_dn13 = assign106540_e158676_d_n13;

        let (assign106550_e158688, assign106550_e158688_d_n0, assign106550_e158688_d_n2, assign106550_e158688_d_n4, assign106550_e158688_d_n5, assign106550_e158688_d_n6, assign106550_e158688_d_n7, assign106550_e158688_d_n8, assign106550_e158688_d_n9, assign106550_e158688_d_n10, assign106550_e158688_d_n13,) = {
    if ((locals.var_guard2400 != 0.0) && (locals.var_guard2404 != 0.0)) {
        let assign106550_e158684: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign106550_e158685: f64 = (0.5 * assign106550_e158684);
        let assign106550_e158686: f64 = (locals.var_t1 - assign106550_e158685);
        (assign106550_e158686, (locals.var_t1_dn0 - (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (locals.var_t1_dn2 - (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), (locals.var_t1_dn4 - (0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4))), (locals.var_t1_dn5 - (0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5))), (locals.var_t1_dn6 - (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), (locals.var_t1_dn7 - (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))), (locals.var_t1_dn8 - (0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8))), (locals.var_t1_dn9 - (0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9))), (locals.var_t1_dn10 - (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), (locals.var_t1_dn13 - (0.5 * (locals.var_tmf1_dn13 + locals.var_tmf2_dn13))),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign106550_e158688;
        locals.var_t2_dn0 = assign106550_e158688_d_n0;
        locals.var_t2_dn2 = assign106550_e158688_d_n2;
        locals.var_t2_dn4 = assign106550_e158688_d_n4;
        locals.var_t2_dn5 = assign106550_e158688_d_n5;
        locals.var_t2_dn6 = assign106550_e158688_d_n6;
        locals.var_t2_dn7 = assign106550_e158688_d_n7;
        locals.var_t2_dn8 = assign106550_e158688_d_n8;
        locals.var_t2_dn9 = assign106550_e158688_d_n9;
        locals.var_t2_dn10 = assign106550_e158688_d_n10;
        locals.var_t2_dn13 = assign106550_e158688_d_n13;

        let (assign106560_e158694, assign106560_e158694_d_n0, assign106560_e158694_d_n2, assign106560_e158694_d_n4, assign106560_e158694_d_n5, assign106560_e158694_d_n6, assign106560_e158694_d_n7, assign106560_e158694_d_n8, assign106560_e158694_d_n9, assign106560_e158694_d_n10, assign106560_e158694_d_n13,) = {
    if ((locals.var_guard2400 != 0.0) && (locals.var_guard2404 != 0.0)) {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    } else {
        (locals.var_p, locals.var_p_dn0, locals.var_p_dn2, locals.var_p_dn4, locals.var_p_dn5, locals.var_p_dn6, locals.var_p_dn7, locals.var_p_dn8, locals.var_p_dn9, locals.var_p_dn10, locals.var_p_dn13,)
    }
};
        locals.var_p = assign106560_e158694;
        locals.var_p_dn0 = assign106560_e158694_d_n0;
        locals.var_p_dn2 = assign106560_e158694_d_n2;
        locals.var_p_dn4 = assign106560_e158694_d_n4;
        locals.var_p_dn5 = assign106560_e158694_d_n5;
        locals.var_p_dn6 = assign106560_e158694_d_n6;
        locals.var_p_dn7 = assign106560_e158694_d_n7;
        locals.var_p_dn8 = assign106560_e158694_d_n8;
        locals.var_p_dn9 = assign106560_e158694_d_n9;
        locals.var_p_dn10 = assign106560_e158694_d_n10;
        locals.var_p_dn13 = assign106560_e158694_d_n13;

        let (assign106570_e158699, assign106570_e158699_d_n0, assign106570_e158699_d_n2, assign106570_e158699_d_n4, assign106570_e158699_d_n5, assign106570_e158699_d_n6, assign106570_e158699_d_n7, assign106570_e158699_d_n8, assign106570_e158699_d_n9, assign106570_e158699_d_n10, assign106570_e158699_d_n13,) = {
    if (locals.var_guard2400 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_gth, locals.var_gth_dn0, locals.var_gth_dn2, locals.var_gth_dn4, locals.var_gth_dn5, locals.var_gth_dn6, locals.var_gth_dn7, locals.var_gth_dn8, locals.var_gth_dn9, locals.var_gth_dn10, locals.var_gth_dn13,)
    }
};
        locals.var_gth = assign106570_e158699;
        locals.var_gth_dn0 = assign106570_e158699_d_n0;
        locals.var_gth_dn2 = assign106570_e158699_d_n2;
        locals.var_gth_dn4 = assign106570_e158699_d_n4;
        locals.var_gth_dn5 = assign106570_e158699_d_n5;
        locals.var_gth_dn6 = assign106570_e158699_d_n6;
        locals.var_gth_dn7 = assign106570_e158699_d_n7;
        locals.var_gth_dn8 = assign106570_e158699_d_n8;
        locals.var_gth_dn9 = assign106570_e158699_d_n9;
        locals.var_gth_dn10 = assign106570_e158699_d_n10;
        locals.var_gth_dn13 = assign106570_e158699_d_n13;

        let (assign106580_e158704, assign106580_e158704_d_n0, assign106580_e158704_d_n2, assign106580_e158704_d_n4, assign106580_e158704_d_n5, assign106580_e158704_d_n6, assign106580_e158704_d_n7, assign106580_e158704_d_n8, assign106580_e158704_d_n9, assign106580_e158704_d_n10, assign106580_e158704_d_n13,) = {
    if (locals.var_guard2400 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_p, locals.var_p_dn0, locals.var_p_dn2, locals.var_p_dn4, locals.var_p_dn5, locals.var_p_dn6, locals.var_p_dn7, locals.var_p_dn8, locals.var_p_dn9, locals.var_p_dn10, locals.var_p_dn13,)
    }
};
        locals.var_p = assign106580_e158704;
        locals.var_p_dn0 = assign106580_e158704_d_n0;
        locals.var_p_dn2 = assign106580_e158704_d_n2;
        locals.var_p_dn4 = assign106580_e158704_d_n4;
        locals.var_p_dn5 = assign106580_e158704_d_n5;
        locals.var_p_dn6 = assign106580_e158704_d_n6;
        locals.var_p_dn7 = assign106580_e158704_d_n7;
        locals.var_p_dn8 = assign106580_e158704_d_n8;
        locals.var_p_dn9 = assign106580_e158704_d_n9;
        locals.var_p_dn10 = assign106580_e158704_d_n10;
        locals.var_p_dn13 = assign106580_e158704_d_n13;

        let (assign106650_e158744, assign106650_e158744_d_n0, assign106650_e158744_d_n2, assign106650_e158744_d_n4, assign106650_e158744_d_n5, assign106650_e158744_d_n6, assign106650_e158744_d_n7, assign106650_e158744_d_n8, assign106650_e158744_d_n9, assign106650_e158744_d_n10, assign106650_e158744_d_n11, assign106650_e158744_d_n13,) = {
    if (locals.var_flg_nqs != 0.0) {
        let assign106650_e158742: f64 = (locals.var_qi_nqs * locals.var_qdrat);
        (assign106650_e158742, (locals.var_qi_nqs * locals.var_qdrat_dn0), (locals.var_qi_nqs * locals.var_qdrat_dn2), (locals.var_qi_nqs * locals.var_qdrat_dn4), (locals.var_qi_nqs * locals.var_qdrat_dn5), (locals.var_qi_nqs * locals.var_qdrat_dn6), (locals.var_qi_nqs * locals.var_qdrat_dn7), (locals.var_qi_nqs * locals.var_qdrat_dn8), (locals.var_qi_nqs * locals.var_qdrat_dn9), (locals.var_qi_nqs * locals.var_qdrat_dn10), (locals.var_qi_nqs_dn11 * locals.var_qdrat), (locals.var_qi_nqs * locals.var_qdrat_dn13),)
    } else {
        (locals.var_qd_nqs, locals.var_qd_nqs_dn0, locals.var_qd_nqs_dn2, locals.var_qd_nqs_dn4, locals.var_qd_nqs_dn5, locals.var_qd_nqs_dn6, locals.var_qd_nqs_dn7, locals.var_qd_nqs_dn8, locals.var_qd_nqs_dn9, locals.var_qd_nqs_dn10, locals.var_qd_nqs_dn11, locals.var_qd_nqs_dn13,)
    }
};
        locals.var_qd_nqs = assign106650_e158744;
        locals.var_qd_nqs_dn0 = assign106650_e158744_d_n0;
        locals.var_qd_nqs_dn2 = assign106650_e158744_d_n2;
        locals.var_qd_nqs_dn4 = assign106650_e158744_d_n4;
        locals.var_qd_nqs_dn5 = assign106650_e158744_d_n5;
        locals.var_qd_nqs_dn6 = assign106650_e158744_d_n6;
        locals.var_qd_nqs_dn7 = assign106650_e158744_d_n7;
        locals.var_qd_nqs_dn8 = assign106650_e158744_d_n8;
        locals.var_qd_nqs_dn9 = assign106650_e158744_d_n9;
        locals.var_qd_nqs_dn10 = assign106650_e158744_d_n10;
        locals.var_qd_nqs_dn11 = assign106650_e158744_d_n11;
        locals.var_qd_nqs_dn13 = assign106650_e158744_d_n13;

        let (assign106660_e158751, assign106660_e158751_d_n11, assign106660_e158751_d_n12,) = {
    if (locals.var_flg_nqs != 0.0) {
        let assign106660_e158747: f64 = (-locals.var_qi_nqs);
        let assign106660_e158749: f64 = (assign106660_e158747 - locals.var_qb_nqs);
        (assign106660_e158749, (-locals.var_qi_nqs_dn11), (-locals.var_qb_nqs_dn12),)
    } else {
        (locals.var_qg_nqs, locals.var_qg_nqs_dn11, locals.var_qg_nqs_dn12,)
    }
};
        locals.var_qg_nqs = assign106660_e158751;
        locals.var_qg_nqs_dn11 = assign106660_e158751_d_n11;
        locals.var_qg_nqs_dn12 = assign106660_e158751_d_n12;

        let (assign106670_e158759, assign106670_e158759_d_n0, assign106670_e158759_d_n2, assign106670_e158759_d_n4, assign106670_e158759_d_n5, assign106670_e158759_d_n6, assign106670_e158759_d_n7, assign106670_e158759_d_n8, assign106670_e158759_d_n9, assign106670_e158759_d_n10, assign106670_e158759_d_n11, assign106670_e158759_d_n13,) = {
    if (locals.var_flg_nqs != 0.0) {
        let assign106670_e158756: f64 = (1.0 - locals.var_qdrat);
        let assign106670_e158757: f64 = (locals.var_qi_nqs * assign106670_e158756);
        (assign106670_e158757, (locals.var_qi_nqs * (-locals.var_qdrat_dn0)), (locals.var_qi_nqs * (-locals.var_qdrat_dn2)), (locals.var_qi_nqs * (-locals.var_qdrat_dn4)), (locals.var_qi_nqs * (-locals.var_qdrat_dn5)), (locals.var_qi_nqs * (-locals.var_qdrat_dn6)), (locals.var_qi_nqs * (-locals.var_qdrat_dn7)), (locals.var_qi_nqs * (-locals.var_qdrat_dn8)), (locals.var_qi_nqs * (-locals.var_qdrat_dn9)), (locals.var_qi_nqs * (-locals.var_qdrat_dn10)), (locals.var_qi_nqs_dn11 * assign106670_e158756), (locals.var_qi_nqs * (-locals.var_qdrat_dn13)),)
    } else {
        (locals.var_qs_nqs, locals.var_qs_nqs_dn0, locals.var_qs_nqs_dn2, locals.var_qs_nqs_dn4, locals.var_qs_nqs_dn5, locals.var_qs_nqs_dn6, locals.var_qs_nqs_dn7, locals.var_qs_nqs_dn8, locals.var_qs_nqs_dn9, locals.var_qs_nqs_dn10, locals.var_qs_nqs_dn11, locals.var_qs_nqs_dn13,)
    }
};
        locals.var_qs_nqs = assign106670_e158759;
        locals.var_qs_nqs_dn0 = assign106670_e158759_d_n0;
        locals.var_qs_nqs_dn2 = assign106670_e158759_d_n2;
        locals.var_qs_nqs_dn4 = assign106670_e158759_d_n4;
        locals.var_qs_nqs_dn5 = assign106670_e158759_d_n5;
        locals.var_qs_nqs_dn6 = assign106670_e158759_d_n6;
        locals.var_qs_nqs_dn7 = assign106670_e158759_d_n7;
        locals.var_qs_nqs_dn8 = assign106670_e158759_d_n8;
        locals.var_qs_nqs_dn9 = assign106670_e158759_d_n9;
        locals.var_qs_nqs_dn10 = assign106670_e158759_d_n10;
        locals.var_qs_nqs_dn11 = assign106670_e158759_d_n11;
        locals.var_qs_nqs_dn13 = assign106670_e158759_d_n13;

        let (assign106700_e158774, assign106700_e158774_d_n0, assign106700_e158774_d_n2, assign106700_e158774_d_n4, assign106700_e158774_d_n5, assign106700_e158774_d_n6, assign106700_e158774_d_n7, assign106700_e158774_d_n8, assign106700_e158774_d_n9, assign106700_e158774_d_n10, assign106700_e158774_d_n11, assign106700_e158774_d_n13,) = {
    if (locals.var_flg_nqs == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qd_nqs, locals.var_qd_nqs_dn0, locals.var_qd_nqs_dn2, locals.var_qd_nqs_dn4, locals.var_qd_nqs_dn5, locals.var_qd_nqs_dn6, locals.var_qd_nqs_dn7, locals.var_qd_nqs_dn8, locals.var_qd_nqs_dn9, locals.var_qd_nqs_dn10, locals.var_qd_nqs_dn11, locals.var_qd_nqs_dn13,)
    }
};
        locals.var_qd_nqs = assign106700_e158774;
        locals.var_qd_nqs_dn0 = assign106700_e158774_d_n0;
        locals.var_qd_nqs_dn2 = assign106700_e158774_d_n2;
        locals.var_qd_nqs_dn4 = assign106700_e158774_d_n4;
        locals.var_qd_nqs_dn5 = assign106700_e158774_d_n5;
        locals.var_qd_nqs_dn6 = assign106700_e158774_d_n6;
        locals.var_qd_nqs_dn7 = assign106700_e158774_d_n7;
        locals.var_qd_nqs_dn8 = assign106700_e158774_d_n8;
        locals.var_qd_nqs_dn9 = assign106700_e158774_d_n9;
        locals.var_qd_nqs_dn10 = assign106700_e158774_d_n10;
        locals.var_qd_nqs_dn11 = assign106700_e158774_d_n11;
        locals.var_qd_nqs_dn13 = assign106700_e158774_d_n13;

        let (assign106710_e158779, assign106710_e158779_d_n11, assign106710_e158779_d_n12,) = {
    if (locals.var_flg_nqs == 0.0) {
        (0.0, 0.0, 0.0,)
    } else {
        (locals.var_qg_nqs, locals.var_qg_nqs_dn11, locals.var_qg_nqs_dn12,)
    }
};
        locals.var_qg_nqs = assign106710_e158779;
        locals.var_qg_nqs_dn11 = assign106710_e158779_d_n11;
        locals.var_qg_nqs_dn12 = assign106710_e158779_d_n12;

        let (assign106720_e158784, assign106720_e158784_d_n0, assign106720_e158784_d_n2, assign106720_e158784_d_n4, assign106720_e158784_d_n5, assign106720_e158784_d_n6, assign106720_e158784_d_n7, assign106720_e158784_d_n8, assign106720_e158784_d_n9, assign106720_e158784_d_n10, assign106720_e158784_d_n11, assign106720_e158784_d_n13,) = {
    if (locals.var_flg_nqs == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qs_nqs, locals.var_qs_nqs_dn0, locals.var_qs_nqs_dn2, locals.var_qs_nqs_dn4, locals.var_qs_nqs_dn5, locals.var_qs_nqs_dn6, locals.var_qs_nqs_dn7, locals.var_qs_nqs_dn8, locals.var_qs_nqs_dn9, locals.var_qs_nqs_dn10, locals.var_qs_nqs_dn11, locals.var_qs_nqs_dn13,)
    }
};
        locals.var_qs_nqs = assign106720_e158784;
        locals.var_qs_nqs_dn0 = assign106720_e158784_d_n0;
        locals.var_qs_nqs_dn2 = assign106720_e158784_d_n2;
        locals.var_qs_nqs_dn4 = assign106720_e158784_d_n4;
        locals.var_qs_nqs_dn5 = assign106720_e158784_d_n5;
        locals.var_qs_nqs_dn6 = assign106720_e158784_d_n6;
        locals.var_qs_nqs_dn7 = assign106720_e158784_d_n7;
        locals.var_qs_nqs_dn8 = assign106720_e158784_d_n8;
        locals.var_qs_nqs_dn9 = assign106720_e158784_d_n9;
        locals.var_qs_nqs_dn10 = assign106720_e158784_d_n10;
        locals.var_qs_nqs_dn11 = assign106720_e158784_d_n11;
        locals.var_qs_nqs_dn13 = assign106720_e158784_d_n13;

        let assign106730_e158787: f64 = (p.p87 * locals.var_mode);
        let assign106730_e158789: f64 = (assign106730_e158787 * locals.var_ids);
        locals.var_idse = assign106730_e158789;
        locals.var_idse_dn0 = (assign106730_e158787 * locals.var_ids_dn0);
        locals.var_idse_dn2 = (assign106730_e158787 * locals.var_ids_dn2);
        locals.var_idse_dn4 = (assign106730_e158787 * locals.var_ids_dn4);
        locals.var_idse_dn5 = (assign106730_e158787 * locals.var_ids_dn5);
        locals.var_idse_dn6 = (assign106730_e158787 * locals.var_ids_dn6);
        locals.var_idse_dn7 = (assign106730_e158787 * locals.var_ids_dn7);
        locals.var_idse_dn8 = (assign106730_e158787 * locals.var_ids_dn8);
        locals.var_idse_dn9 = (assign106730_e158787 * locals.var_ids_dn9);
        locals.var_idse_dn10 = (assign106730_e158787 * locals.var_ids_dn10);
        locals.var_idse_dn13 = (assign106730_e158787 * locals.var_ids_dn13);

        let assign106890_e158837: f64 = locals.var_qg_dn5;
        locals.var_cgdbd = assign106890_e158837;
        locals.var_cgdbd_dn0 = 0.0;
        locals.var_cgdbd_dn2 = 0.0;
        locals.var_cgdbd_dn4 = 0.0;
        locals.var_cgdbd_dn5 = 0.0;
        locals.var_cgdbd_dn6 = 0.0;
        locals.var_cgdbd_dn7 = 0.0;
        locals.var_cgdbd_dn8 = 0.0;
        locals.var_cgdbd_dn9 = 0.0;
        locals.var_cgdbd_dn10 = 0.0;
        locals.var_cgdbd_dn13 = 0.0;

        let assign106900_e158840: f64 = (p.p87 * locals.var_cgdbd);
        locals.var_cgdbd = assign106900_e158840;
        locals.var_cgdbd_dn0 = (p.p87 * locals.var_cgdbd_dn0);
        locals.var_cgdbd_dn2 = (p.p87 * locals.var_cgdbd_dn2);
        locals.var_cgdbd_dn4 = (p.p87 * locals.var_cgdbd_dn4);
        locals.var_cgdbd_dn5 = (p.p87 * locals.var_cgdbd_dn5);
        locals.var_cgdbd_dn6 = (p.p87 * locals.var_cgdbd_dn6);
        locals.var_cgdbd_dn7 = (p.p87 * locals.var_cgdbd_dn7);
        locals.var_cgdbd_dn8 = (p.p87 * locals.var_cgdbd_dn8);
        locals.var_cgdbd_dn9 = (p.p87 * locals.var_cgdbd_dn9);
        locals.var_cgdbd_dn10 = (p.p87 * locals.var_cgdbd_dn10);
        locals.var_cgdbd_dn13 = (p.p87 * locals.var_cgdbd_dn13);

        let assign106910_e158843: f64 = locals.var_qg_dn7;
        locals.var_cgsbd = assign106910_e158843;
        locals.var_cgsbd_dn0 = 0.0;
        locals.var_cgsbd_dn2 = 0.0;
        locals.var_cgsbd_dn4 = 0.0;
        locals.var_cgsbd_dn5 = 0.0;
        locals.var_cgsbd_dn6 = 0.0;
        locals.var_cgsbd_dn7 = 0.0;
        locals.var_cgsbd_dn8 = 0.0;
        locals.var_cgsbd_dn9 = 0.0;
        locals.var_cgsbd_dn10 = 0.0;
        locals.var_cgsbd_dn13 = 0.0;

        let assign106920_e158846: f64 = (p.p87 * locals.var_cgsbd);
        locals.var_cgsbd = assign106920_e158846;
        locals.var_cgsbd_dn0 = (p.p87 * locals.var_cgsbd_dn0);
        locals.var_cgsbd_dn2 = (p.p87 * locals.var_cgsbd_dn2);
        locals.var_cgsbd_dn4 = (p.p87 * locals.var_cgsbd_dn4);
        locals.var_cgsbd_dn5 = (p.p87 * locals.var_cgsbd_dn5);
        locals.var_cgsbd_dn6 = (p.p87 * locals.var_cgsbd_dn6);
        locals.var_cgsbd_dn7 = (p.p87 * locals.var_cgsbd_dn7);
        locals.var_cgsbd_dn8 = (p.p87 * locals.var_cgsbd_dn8);
        locals.var_cgsbd_dn9 = (p.p87 * locals.var_cgsbd_dn9);
        locals.var_cgsbd_dn10 = (p.p87 * locals.var_cgsbd_dn10);
        locals.var_cgsbd_dn13 = (p.p87 * locals.var_cgsbd_dn13);

        let assign107290_e158961: f64 = if locals.var_mode == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard2407 = assign107290_e158961;

        let (assign107320_e158973, assign107320_e158973_d_n0, assign107320_e158973_d_n2, assign107320_e158973_d_n4, assign107320_e158973_d_n5, assign107320_e158973_d_n6, assign107320_e158973_d_n7, assign107320_e158973_d_n8, assign107320_e158973_d_n9, assign107320_e158973_d_n10, assign107320_e158973_d_n13,) = {
    if (locals.var_guard2407 != 0.0) {
        (locals.var_cgsbd, locals.var_cgsbd_dn0, locals.var_cgsbd_dn2, locals.var_cgsbd_dn4, locals.var_cgsbd_dn5, locals.var_cgsbd_dn6, locals.var_cgsbd_dn7, locals.var_cgsbd_dn8, locals.var_cgsbd_dn9, locals.var_cgsbd_dn10, locals.var_cgsbd_dn13,)
    } else {
        (locals.var_cgsb, locals.var_cgsb_dn0, locals.var_cgsb_dn2, locals.var_cgsb_dn4, locals.var_cgsb_dn5, locals.var_cgsb_dn6, locals.var_cgsb_dn7, locals.var_cgsb_dn8, locals.var_cgsb_dn9, locals.var_cgsb_dn10, locals.var_cgsb_dn13,)
    }
};
        locals.var_cgsb = assign107320_e158973;
        locals.var_cgsb_dn0 = assign107320_e158973_d_n0;
        locals.var_cgsb_dn2 = assign107320_e158973_d_n2;
        locals.var_cgsb_dn4 = assign107320_e158973_d_n4;
        locals.var_cgsb_dn5 = assign107320_e158973_d_n5;
        locals.var_cgsb_dn6 = assign107320_e158973_d_n6;
        locals.var_cgsb_dn7 = assign107320_e158973_d_n7;
        locals.var_cgsb_dn8 = assign107320_e158973_d_n8;
        locals.var_cgsb_dn9 = assign107320_e158973_d_n9;
        locals.var_cgsb_dn10 = assign107320_e158973_d_n10;
        locals.var_cgsb_dn13 = assign107320_e158973_d_n13;

        let (assign107420_e159017, assign107420_e159017_d_n0, assign107420_e159017_d_n2, assign107420_e159017_d_n4, assign107420_e159017_d_n5, assign107420_e159017_d_n6, assign107420_e159017_d_n7, assign107420_e159017_d_n8, assign107420_e159017_d_n9, assign107420_e159017_d_n10, assign107420_e159017_d_n13,) = {
    if (locals.var_guard2407 == 0.0) {
        (locals.var_cgdbd, locals.var_cgdbd_dn0, locals.var_cgdbd_dn2, locals.var_cgdbd_dn4, locals.var_cgdbd_dn5, locals.var_cgdbd_dn6, locals.var_cgdbd_dn7, locals.var_cgdbd_dn8, locals.var_cgdbd_dn9, locals.var_cgdbd_dn10, locals.var_cgdbd_dn13,)
    } else {
        (locals.var_cgsb, locals.var_cgsb_dn0, locals.var_cgsb_dn2, locals.var_cgsb_dn4, locals.var_cgsb_dn5, locals.var_cgsb_dn6, locals.var_cgsb_dn7, locals.var_cgsb_dn8, locals.var_cgsb_dn9, locals.var_cgsb_dn10, locals.var_cgsb_dn13,)
    }
};
        locals.var_cgsb = assign107420_e159017;
        locals.var_cgsb_dn0 = assign107420_e159017_d_n0;
        locals.var_cgsb_dn2 = assign107420_e159017_d_n2;
        locals.var_cgsb_dn4 = assign107420_e159017_d_n4;
        locals.var_cgsb_dn5 = assign107420_e159017_d_n5;
        locals.var_cgsb_dn6 = assign107420_e159017_d_n6;
        locals.var_cgsb_dn7 = assign107420_e159017_d_n7;
        locals.var_cgsb_dn8 = assign107420_e159017_d_n8;
        locals.var_cgsb_dn9 = assign107420_e159017_d_n9;
        locals.var_cgsb_dn10 = assign107420_e159017_d_n10;
        locals.var_cgsb_dn13 = assign107420_e159017_d_n13;

        let assign107650_e159080: f64 = if p.p48 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard2409 = assign107650_e159080;

        let (assign107740_e159125,) = {
    if (p.p28 != 0.0) {
        (1.0,)
    } else {
        (locals.var_cqi,)
    }
};
        locals.var_cqi = assign107740_e159125;

        let (assign107750_e159129,) = {
    if (p.p28 != 0.0) {
        (1.0,)
    } else {
        (locals.var_cqb,)
    }
};
        locals.var_cqb = assign107750_e159129;

    }

    pub(super) fn stamp_reactive_block_0(
        param_given: &[bool; Instance::PARAMETER_COUNT],
        locals: &mut StampLocals,
    ) {
        let assign10_e1394: f64 = if param_given[12] { 1.0 } else { 0.0 };
        locals.var_nsubcdfm_given = assign10_e1394;
        locals.var_nsubcdfm_given_rv = 0.0;

        let assign20_e1396: f64 = if param_given[268] { 1.0 } else { 0.0 };
        locals.var_cgdo_given = assign20_e1396;
        locals.var_cgdo_given_rv = 0.0;

        let assign30_e1398: f64 = if param_given[269] { 1.0 } else { 0.0 };
        locals.var_cgso_given = assign30_e1398;
        locals.var_cgso_given_rv = 0.0;

        locals.var_cgdoe = 0.0;
        locals.var_cgdoe_rv = 0.0;

        locals.var_cgsoe = 0.0;
        locals.var_cgsoe_rv = 0.0;

        locals.var_xd = 0.0;
        locals.var_xd_dn0 = 0.0;
        locals.var_xd_dn2 = 0.0;
        locals.var_xd_dn4 = 0.0;
        locals.var_xd_dn5 = 0.0;
        locals.var_xd_dn6 = 0.0;
        locals.var_xd_dn7 = 0.0;
        locals.var_xd_dn8 = 0.0;
        locals.var_xd_dn9 = 0.0;
        locals.var_xd_dn10 = 0.0;
        locals.var_xd_dn13 = 0.0;
        locals.var_xd_rv = 0.0;

        locals.var_rdd = 0.0;
        locals.var_rdd_dn0 = 0.0;
        locals.var_rdd_dn2 = 0.0;
        locals.var_rdd_dn4 = 0.0;
        locals.var_rdd_dn5 = 0.0;
        locals.var_rdd_dn6 = 0.0;
        locals.var_rdd_dn7 = 0.0;
        locals.var_rdd_dn8 = 0.0;
        locals.var_rdd_dn9 = 0.0;
        locals.var_rdd_dn10 = 0.0;
        locals.var_rdd_dn13 = 0.0;
        locals.var_rdd_rv = 0.0;

        locals.var_rsd = 0.0;
        locals.var_rsd_dn0 = 0.0;
        locals.var_rsd_dn2 = 0.0;
        locals.var_rsd_dn4 = 0.0;
        locals.var_rsd_dn5 = 0.0;
        locals.var_rsd_dn6 = 0.0;
        locals.var_rsd_dn7 = 0.0;
        locals.var_rsd_dn8 = 0.0;
        locals.var_rsd_dn9 = 0.0;
        locals.var_rsd_dn10 = 0.0;
        locals.var_rsd_dn13 = 0.0;
        locals.var_rsd_rv = 0.0;

        locals.var_flg_ign = 0.0;
        locals.var_flg_ign_rv = 0.0;

        locals.var_flg_noqi = 0.0;
        locals.var_flg_noqi_rv = 0.0;

        locals.var_flg_rsrd = 0.0;
        locals.var_flg_rsrd_rv = 0.0;

        locals.var_flg_zone = 0.0;
        locals.var_flg_zone_rv = 0.0;

        locals.var_rd_ps0ld = 0.0;
        locals.var_rd_ps0ld_dn0 = 0.0;
        locals.var_rd_ps0ld_dn2 = 0.0;
        locals.var_rd_ps0ld_dn4 = 0.0;
        locals.var_rd_ps0ld_dn5 = 0.0;
        locals.var_rd_ps0ld_dn6 = 0.0;
        locals.var_rd_ps0ld_dn7 = 0.0;
        locals.var_rd_ps0ld_dn8 = 0.0;
        locals.var_rd_ps0ld_dn9 = 0.0;
        locals.var_rd_ps0ld_dn10 = 0.0;
        locals.var_rd_ps0ld_dn13 = 0.0;
        locals.var_rd_ps0ld_rv = 0.0;

        locals.var_rd_qbuld = 0.0;
        locals.var_rd_qbuld_dn0 = 0.0;
        locals.var_rd_qbuld_dn2 = 0.0;
        locals.var_rd_qbuld_dn4 = 0.0;
        locals.var_rd_qbuld_dn5 = 0.0;
        locals.var_rd_qbuld_dn6 = 0.0;
        locals.var_rd_qbuld_dn7 = 0.0;
        locals.var_rd_qbuld_dn8 = 0.0;
        locals.var_rd_qbuld_dn9 = 0.0;
        locals.var_rd_qbuld_dn10 = 0.0;
        locals.var_rd_qbuld_dn13 = 0.0;
        locals.var_rd_qbuld_rv = 0.0;

        locals.var_vbs_max = 0.8;
        locals.var_vbs_max_dn0 = 0.0;
        locals.var_vbs_max_dn2 = 0.0;
        locals.var_vbs_max_dn4 = 0.0;
        locals.var_vbs_max_dn5 = 0.0;
        locals.var_vbs_max_dn6 = 0.0;
        locals.var_vbs_max_dn7 = 0.0;
        locals.var_vbs_max_dn8 = 0.0;
        locals.var_vbs_max_dn9 = 0.0;
        locals.var_vbs_max_dn10 = 0.0;
        locals.var_vbs_max_dn13 = 0.0;
        locals.var_vbs_max_rv = 0.0;

        locals.var_vbs_bnd = 0.4;
        locals.var_vbs_bnd_dn0 = 0.0;
        locals.var_vbs_bnd_dn2 = 0.0;
        locals.var_vbs_bnd_dn4 = 0.0;
        locals.var_vbs_bnd_dn5 = 0.0;
        locals.var_vbs_bnd_dn6 = 0.0;
        locals.var_vbs_bnd_dn7 = 0.0;
        locals.var_vbs_bnd_dn8 = 0.0;
        locals.var_vbs_bnd_dn9 = 0.0;
        locals.var_vbs_bnd_dn10 = 0.0;
        locals.var_vbs_bnd_dn13 = 0.0;
        locals.var_vbs_bnd_rv = 0.0;

        locals.var_flg_pprv = 0.0;
        locals.var_flg_pprv_rv = 0.0;

        locals.var_flg_conv = 0.0;
        locals.var_flg_conv_rv = 0.0;

        locals.var_flg_qme = 0.0;
        locals.var_flg_qme_rv = 0.0;

        locals.var_flg_nqs = 0.0;
        locals.var_flg_nqs_rv = 0.0;

        locals.var_vbscl = 0.0;
        locals.var_vbscl_dn0 = 0.0;
        locals.var_vbscl_dn2 = 0.0;
        locals.var_vbscl_dn4 = 0.0;
        locals.var_vbscl_dn5 = 0.0;
        locals.var_vbscl_dn6 = 0.0;
        locals.var_vbscl_dn7 = 0.0;
        locals.var_vbscl_dn8 = 0.0;
        locals.var_vbscl_dn9 = 0.0;
        locals.var_vbscl_dn10 = 0.0;
        locals.var_vbscl_dn13 = 0.0;
        locals.var_vbscl_rv = 0.0;

        locals.var_vbscldvbs = 0.0;
        locals.var_vbscldvbs_dn0 = 0.0;
        locals.var_vbscldvbs_dn2 = 0.0;
        locals.var_vbscldvbs_dn4 = 0.0;
        locals.var_vbscldvbs_dn5 = 0.0;
        locals.var_vbscldvbs_dn6 = 0.0;
        locals.var_vbscldvbs_dn7 = 0.0;
        locals.var_vbscldvbs_dn8 = 0.0;
        locals.var_vbscldvbs_dn9 = 0.0;
        locals.var_vbscldvbs_dn10 = 0.0;
        locals.var_vbscldvbs_dn13 = 0.0;
        locals.var_vbscldvbs_rv = 0.0;

        locals.var_vgp = 0.0;
        locals.var_vgp_dn0 = 0.0;
        locals.var_vgp_dn2 = 0.0;
        locals.var_vgp_dn4 = 0.0;
        locals.var_vgp_dn5 = 0.0;
        locals.var_vgp_dn6 = 0.0;
        locals.var_vgp_dn7 = 0.0;
        locals.var_vgp_dn8 = 0.0;
        locals.var_vgp_dn9 = 0.0;
        locals.var_vgp_dn10 = 0.0;
        locals.var_vgp_dn13 = 0.0;
        locals.var_vgp_rv = 0.0;

        locals.var_vgs_fb = 0.0;
        locals.var_vgs_fb_dn0 = 0.0;
        locals.var_vgs_fb_dn2 = 0.0;
        locals.var_vgs_fb_dn4 = 0.0;
        locals.var_vgs_fb_dn5 = 0.0;
        locals.var_vgs_fb_dn6 = 0.0;
        locals.var_vgs_fb_dn7 = 0.0;
        locals.var_vgs_fb_dn8 = 0.0;
        locals.var_vgs_fb_dn9 = 0.0;
        locals.var_vgs_fb_dn10 = 0.0;
        locals.var_vgs_fb_dn13 = 0.0;
        locals.var_vgs_fb_rv = 0.0;

        locals.var_ps0 = 0.0;
        locals.var_ps0_dn0 = 0.0;
        locals.var_ps0_dn2 = 0.0;
        locals.var_ps0_dn4 = 0.0;
        locals.var_ps0_dn5 = 0.0;
        locals.var_ps0_dn6 = 0.0;
        locals.var_ps0_dn7 = 0.0;
        locals.var_ps0_dn8 = 0.0;
        locals.var_ps0_dn9 = 0.0;
        locals.var_ps0_dn10 = 0.0;
        locals.var_ps0_dn13 = 0.0;
        locals.var_ps0_rv = 0.0;

        locals.var_ps0_ini = 0.0;
        locals.var_ps0_ini_dn0 = 0.0;
        locals.var_ps0_ini_dn2 = 0.0;
        locals.var_ps0_ini_dn4 = 0.0;
        locals.var_ps0_ini_dn5 = 0.0;
        locals.var_ps0_ini_dn6 = 0.0;
        locals.var_ps0_ini_dn7 = 0.0;
        locals.var_ps0_ini_dn8 = 0.0;
        locals.var_ps0_ini_dn9 = 0.0;
        locals.var_ps0_ini_dn10 = 0.0;
        locals.var_ps0_ini_dn13 = 0.0;
        locals.var_ps0_ini_rv = 0.0;

        locals.var_ps0_inia = 0.0;
        locals.var_ps0_inia_dn0 = 0.0;
        locals.var_ps0_inia_dn2 = 0.0;
        locals.var_ps0_inia_dn4 = 0.0;
        locals.var_ps0_inia_dn5 = 0.0;
        locals.var_ps0_inia_dn6 = 0.0;
        locals.var_ps0_inia_dn7 = 0.0;
        locals.var_ps0_inia_dn8 = 0.0;
        locals.var_ps0_inia_dn9 = 0.0;
        locals.var_ps0_inia_dn10 = 0.0;
        locals.var_ps0_inia_dn13 = 0.0;
        locals.var_ps0_inia_rv = 0.0;

        locals.var_ps0_inib = 0.0;
        locals.var_ps0_inib_dn0 = 0.0;
        locals.var_ps0_inib_dn2 = 0.0;
        locals.var_ps0_inib_dn4 = 0.0;
        locals.var_ps0_inib_dn5 = 0.0;
        locals.var_ps0_inib_dn6 = 0.0;
        locals.var_ps0_inib_dn7 = 0.0;
        locals.var_ps0_inib_dn8 = 0.0;
        locals.var_ps0_inib_dn9 = 0.0;
        locals.var_ps0_inib_dn10 = 0.0;
        locals.var_ps0_inib_dn13 = 0.0;
        locals.var_ps0_inib_rv = 0.0;

        locals.var_psl = 0.0;
        locals.var_psl_dn0 = 0.0;
        locals.var_psl_dn2 = 0.0;
        locals.var_psl_dn4 = 0.0;
        locals.var_psl_dn5 = 0.0;
        locals.var_psl_dn6 = 0.0;
        locals.var_psl_dn7 = 0.0;
        locals.var_psl_dn8 = 0.0;
        locals.var_psl_dn9 = 0.0;
        locals.var_psl_dn10 = 0.0;
        locals.var_psl_dn13 = 0.0;
        locals.var_psl_rv = 0.0;

        locals.var_psl_lim = 0.0;
        locals.var_psl_lim_dn0 = 0.0;
        locals.var_psl_lim_dn2 = 0.0;
        locals.var_psl_lim_dn4 = 0.0;
        locals.var_psl_lim_dn5 = 0.0;
        locals.var_psl_lim_dn6 = 0.0;
        locals.var_psl_lim_dn7 = 0.0;
        locals.var_psl_lim_dn8 = 0.0;
        locals.var_psl_lim_dn9 = 0.0;
        locals.var_psl_lim_dn10 = 0.0;
        locals.var_psl_lim_dn13 = 0.0;
        locals.var_psl_lim_rv = 0.0;

        locals.var_dplim = 0.0;
        locals.var_dplim_dn0 = 0.0;
        locals.var_dplim_dn2 = 0.0;
        locals.var_dplim_dn4 = 0.0;
        locals.var_dplim_dn5 = 0.0;
        locals.var_dplim_dn6 = 0.0;
        locals.var_dplim_dn7 = 0.0;
        locals.var_dplim_dn8 = 0.0;
        locals.var_dplim_dn9 = 0.0;
        locals.var_dplim_dn10 = 0.0;
        locals.var_dplim_dn13 = 0.0;
        locals.var_dplim_rv = 0.0;

        locals.var_pds = 0.0;
        locals.var_pds_dn0 = 0.0;
        locals.var_pds_dn2 = 0.0;
        locals.var_pds_dn4 = 0.0;
        locals.var_pds_dn5 = 0.0;
        locals.var_pds_dn6 = 0.0;
        locals.var_pds_dn7 = 0.0;
        locals.var_pds_dn8 = 0.0;
        locals.var_pds_dn9 = 0.0;
        locals.var_pds_dn10 = 0.0;
        locals.var_pds_dn13 = 0.0;
        locals.var_pds_rv = 0.0;

        locals.var_pds_ini = 0.0;
        locals.var_pds_ini_dn0 = 0.0;
        locals.var_pds_ini_dn2 = 0.0;
        locals.var_pds_ini_dn4 = 0.0;
        locals.var_pds_ini_dn5 = 0.0;
        locals.var_pds_ini_dn6 = 0.0;
        locals.var_pds_ini_dn7 = 0.0;
        locals.var_pds_ini_dn8 = 0.0;
        locals.var_pds_ini_dn9 = 0.0;
        locals.var_pds_ini_dn10 = 0.0;
        locals.var_pds_ini_dn13 = 0.0;
        locals.var_pds_ini_rv = 0.0;

        locals.var_pds_max = 0.0;
        locals.var_pds_max_dn0 = 0.0;
        locals.var_pds_max_dn2 = 0.0;
        locals.var_pds_max_dn4 = 0.0;
        locals.var_pds_max_dn5 = 0.0;
        locals.var_pds_max_dn6 = 0.0;
        locals.var_pds_max_dn7 = 0.0;
        locals.var_pds_max_dn8 = 0.0;
        locals.var_pds_max_dn9 = 0.0;
        locals.var_pds_max_dn10 = 0.0;
        locals.var_pds_max_dn13 = 0.0;
        locals.var_pds_max_rv = 0.0;

        locals.var_lp_s0 = 0.0;
        locals.var_lp_s0_rv = 0.0;

        locals.var_lp_sl = 0.0;
        locals.var_lp_sl_rv = 0.0;

        locals.var_xi0 = 0.0;
        locals.var_xi0_dn0 = 0.0;
        locals.var_xi0_dn2 = 0.0;
        locals.var_xi0_dn4 = 0.0;
        locals.var_xi0_dn5 = 0.0;
        locals.var_xi0_dn6 = 0.0;
        locals.var_xi0_dn7 = 0.0;
        locals.var_xi0_dn8 = 0.0;
        locals.var_xi0_dn9 = 0.0;
        locals.var_xi0_dn10 = 0.0;
        locals.var_xi0_dn13 = 0.0;
        locals.var_xi0_rv = 0.0;

        locals.var_xi0p12 = 0.0;
        locals.var_xi0p12_dn0 = 0.0;
        locals.var_xi0p12_dn2 = 0.0;
        locals.var_xi0p12_dn4 = 0.0;
        locals.var_xi0p12_dn5 = 0.0;
        locals.var_xi0p12_dn6 = 0.0;
        locals.var_xi0p12_dn7 = 0.0;
        locals.var_xi0p12_dn8 = 0.0;
        locals.var_xi0p12_dn9 = 0.0;
        locals.var_xi0p12_dn10 = 0.0;
        locals.var_xi0p12_dn13 = 0.0;
        locals.var_xi0p12_rv = 0.0;

        locals.var_xi0p32 = 0.0;
        locals.var_xi0p32_dn0 = 0.0;
        locals.var_xi0p32_dn2 = 0.0;
        locals.var_xi0p32_dn4 = 0.0;
        locals.var_xi0p32_dn5 = 0.0;
        locals.var_xi0p32_dn6 = 0.0;
        locals.var_xi0p32_dn7 = 0.0;
        locals.var_xi0p32_dn8 = 0.0;
        locals.var_xi0p32_dn9 = 0.0;
        locals.var_xi0p32_dn10 = 0.0;
        locals.var_xi0p32_dn13 = 0.0;
        locals.var_xi0p32_rv = 0.0;

        locals.var_xil = 0.0;
        locals.var_xil_dn0 = 0.0;
        locals.var_xil_dn2 = 0.0;
        locals.var_xil_dn4 = 0.0;
        locals.var_xil_dn5 = 0.0;
        locals.var_xil_dn6 = 0.0;
        locals.var_xil_dn7 = 0.0;
        locals.var_xil_dn8 = 0.0;
        locals.var_xil_dn9 = 0.0;
        locals.var_xil_dn10 = 0.0;
        locals.var_xil_dn13 = 0.0;
        locals.var_xil_rv = 0.0;

        locals.var_xilp12 = 0.0;
        locals.var_xilp12_dn0 = 0.0;
        locals.var_xilp12_dn2 = 0.0;
        locals.var_xilp12_dn4 = 0.0;
        locals.var_xilp12_dn5 = 0.0;
        locals.var_xilp12_dn6 = 0.0;
        locals.var_xilp12_dn7 = 0.0;
        locals.var_xilp12_dn8 = 0.0;
        locals.var_xilp12_dn9 = 0.0;
        locals.var_xilp12_dn10 = 0.0;
        locals.var_xilp12_dn13 = 0.0;
        locals.var_xilp12_rv = 0.0;

        locals.var_xilp32 = 0.0;
        locals.var_xilp32_dn0 = 0.0;
        locals.var_xilp32_dn2 = 0.0;
        locals.var_xilp32_dn4 = 0.0;
        locals.var_xilp32_dn5 = 0.0;
        locals.var_xilp32_dn6 = 0.0;
        locals.var_xilp32_dn7 = 0.0;
        locals.var_xilp32_dn8 = 0.0;
        locals.var_xilp32_dn9 = 0.0;
        locals.var_xilp32_dn10 = 0.0;
        locals.var_xilp32_dn13 = 0.0;
        locals.var_xilp32_rv = 0.0;

        locals.var_vbsz = 0.0;
        locals.var_vbsz_dn0 = 0.0;
        locals.var_vbsz_dn2 = 0.0;
        locals.var_vbsz_dn4 = 0.0;
        locals.var_vbsz_dn5 = 0.0;
        locals.var_vbsz_dn6 = 0.0;
        locals.var_vbsz_dn7 = 0.0;
        locals.var_vbsz_dn8 = 0.0;
        locals.var_vbsz_dn9 = 0.0;
        locals.var_vbsz_dn10 = 0.0;
        locals.var_vbsz_dn13 = 0.0;
        locals.var_vbsz_rv = 0.0;

        locals.var_vdsz = 0.0;
        locals.var_vdsz_dn0 = 0.0;
        locals.var_vdsz_dn2 = 0.0;
        locals.var_vdsz_dn4 = 0.0;
        locals.var_vdsz_dn5 = 0.0;
        locals.var_vdsz_dn6 = 0.0;
        locals.var_vdsz_dn7 = 0.0;
        locals.var_vdsz_dn8 = 0.0;
        locals.var_vdsz_dn9 = 0.0;
        locals.var_vdsz_dn10 = 0.0;
        locals.var_vdsz_dn13 = 0.0;
        locals.var_vdsz_rv = 0.0;

        locals.var_vgsz = 0.0;
        locals.var_vgsz_dn0 = 0.0;
        locals.var_vgsz_dn2 = 0.0;
        locals.var_vgsz_dn4 = 0.0;
        locals.var_vgsz_dn5 = 0.0;
        locals.var_vgsz_dn6 = 0.0;
        locals.var_vgsz_dn7 = 0.0;
        locals.var_vgsz_dn8 = 0.0;
        locals.var_vgsz_dn9 = 0.0;
        locals.var_vgsz_dn10 = 0.0;
        locals.var_vgsz_dn13 = 0.0;
        locals.var_vgsz_rv = 0.0;

        locals.var_vzadd = 0.0;
        locals.var_vzadd_dn0 = 0.0;
        locals.var_vzadd_dn2 = 0.0;
        locals.var_vzadd_dn4 = 0.0;
        locals.var_vzadd_dn5 = 0.0;
        locals.var_vzadd_dn6 = 0.0;
        locals.var_vzadd_dn7 = 0.0;
        locals.var_vzadd_dn8 = 0.0;
        locals.var_vzadd_dn9 = 0.0;
        locals.var_vzadd_dn10 = 0.0;
        locals.var_vzadd_dn13 = 0.0;
        locals.var_vzadd_rv = 0.0;

        locals.var_ps0z = 0.0;
        locals.var_ps0z_dn0 = 0.0;
        locals.var_ps0z_dn2 = 0.0;
        locals.var_ps0z_dn4 = 0.0;
        locals.var_ps0z_dn5 = 0.0;
        locals.var_ps0z_dn6 = 0.0;
        locals.var_ps0z_dn7 = 0.0;
        locals.var_ps0z_dn8 = 0.0;
        locals.var_ps0z_dn9 = 0.0;
        locals.var_ps0z_dn10 = 0.0;
        locals.var_ps0z_dn13 = 0.0;
        locals.var_ps0z_rv = 0.0;

        locals.var_pzadd = 0.0;
        locals.var_pzadd_dn0 = 0.0;
        locals.var_pzadd_dn2 = 0.0;
        locals.var_pzadd_dn4 = 0.0;
        locals.var_pzadd_dn5 = 0.0;
        locals.var_pzadd_dn6 = 0.0;
        locals.var_pzadd_dn7 = 0.0;
        locals.var_pzadd_dn8 = 0.0;
        locals.var_pzadd_dn9 = 0.0;
        locals.var_pzadd_dn10 = 0.0;
        locals.var_pzadd_dn13 = 0.0;
        locals.var_pzadd_rv = 0.0;

        locals.var_dvbsibpc = 0.0;
        locals.var_dvbsibpc_dn0 = 0.0;
        locals.var_dvbsibpc_dn2 = 0.0;
        locals.var_dvbsibpc_dn4 = 0.0;
        locals.var_dvbsibpc_dn5 = 0.0;
        locals.var_dvbsibpc_dn6 = 0.0;
        locals.var_dvbsibpc_dn7 = 0.0;
        locals.var_dvbsibpc_dn8 = 0.0;
        locals.var_dvbsibpc_dn9 = 0.0;
        locals.var_dvbsibpc_dn10 = 0.0;
        locals.var_dvbsibpc_dn13 = 0.0;
        locals.var_dvbsibpc_rv = 0.0;

        locals.var_dg3 = 0.0;
        locals.var_dg3_dn0 = 0.0;
        locals.var_dg3_dn2 = 0.0;
        locals.var_dg3_dn4 = 0.0;
        locals.var_dg3_dn5 = 0.0;
        locals.var_dg3_dn6 = 0.0;
        locals.var_dg3_dn7 = 0.0;
        locals.var_dg3_dn8 = 0.0;
        locals.var_dg3_dn9 = 0.0;
        locals.var_dg3_dn10 = 0.0;
        locals.var_dg3_dn13 = 0.0;
        locals.var_dg3_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_1(
        locals: &mut StampLocals,
    ) {
        locals.var_dg4 = 0.0;
        locals.var_dg4_dn0 = 0.0;
        locals.var_dg4_dn2 = 0.0;
        locals.var_dg4_dn4 = 0.0;
        locals.var_dg4_dn5 = 0.0;
        locals.var_dg4_dn6 = 0.0;
        locals.var_dg4_dn7 = 0.0;
        locals.var_dg4_dn8 = 0.0;
        locals.var_dg4_dn9 = 0.0;
        locals.var_dg4_dn10 = 0.0;
        locals.var_dg4_dn13 = 0.0;
        locals.var_dg4_rv = 0.0;

        locals.var_didd = 0.0;
        locals.var_didd_dn0 = 0.0;
        locals.var_didd_dn2 = 0.0;
        locals.var_didd_dn4 = 0.0;
        locals.var_didd_dn5 = 0.0;
        locals.var_didd_dn6 = 0.0;
        locals.var_didd_dn7 = 0.0;
        locals.var_didd_dn8 = 0.0;
        locals.var_didd_dn9 = 0.0;
        locals.var_didd_dn10 = 0.0;
        locals.var_didd_dn13 = 0.0;
        locals.var_didd_rv = 0.0;

        locals.var_betawl = 0.0;
        locals.var_betawl_dn0 = 0.0;
        locals.var_betawl_dn2 = 0.0;
        locals.var_betawl_dn4 = 0.0;
        locals.var_betawl_dn5 = 0.0;
        locals.var_betawl_dn6 = 0.0;
        locals.var_betawl_dn7 = 0.0;
        locals.var_betawl_dn8 = 0.0;
        locals.var_betawl_dn9 = 0.0;
        locals.var_betawl_dn10 = 0.0;
        locals.var_betawl_dn13 = 0.0;
        locals.var_betawl_rv = 0.0;

        locals.var_chi = 0.0;
        locals.var_chi_dn0 = 0.0;
        locals.var_chi_dn2 = 0.0;
        locals.var_chi_dn4 = 0.0;
        locals.var_chi_dn5 = 0.0;
        locals.var_chi_dn6 = 0.0;
        locals.var_chi_dn7 = 0.0;
        locals.var_chi_dn8 = 0.0;
        locals.var_chi_dn9 = 0.0;
        locals.var_chi_dn10 = 0.0;
        locals.var_chi_dn13 = 0.0;
        locals.var_chi_rv = 0.0;

        locals.var_chib = 0.0;
        locals.var_chib_dn0 = 0.0;
        locals.var_chib_dn2 = 0.0;
        locals.var_chib_dn4 = 0.0;
        locals.var_chib_dn5 = 0.0;
        locals.var_chib_dn6 = 0.0;
        locals.var_chib_dn7 = 0.0;
        locals.var_chib_dn8 = 0.0;
        locals.var_chib_dn9 = 0.0;
        locals.var_chib_dn10 = 0.0;
        locals.var_chib_dn13 = 0.0;
        locals.var_chib_rv = 0.0;

        locals.var_rho = 0.0;
        locals.var_rho_dn0 = 0.0;
        locals.var_rho_dn2 = 0.0;
        locals.var_rho_dn4 = 0.0;
        locals.var_rho_dn5 = 0.0;
        locals.var_rho_dn6 = 0.0;
        locals.var_rho_dn7 = 0.0;
        locals.var_rho_dn8 = 0.0;
        locals.var_rho_dn9 = 0.0;
        locals.var_rho_dn10 = 0.0;
        locals.var_rho_dn13 = 0.0;
        locals.var_rho_rv = 0.0;

        locals.var_vth = 0.0;
        locals.var_vth_dn0 = 0.0;
        locals.var_vth_dn2 = 0.0;
        locals.var_vth_dn4 = 0.0;
        locals.var_vth_dn5 = 0.0;
        locals.var_vth_dn6 = 0.0;
        locals.var_vth_dn7 = 0.0;
        locals.var_vth_dn8 = 0.0;
        locals.var_vth_dn9 = 0.0;
        locals.var_vth_dn10 = 0.0;
        locals.var_vth_dn13 = 0.0;
        locals.var_vth_rv = 0.0;

        locals.var_vth0 = 0.0;
        locals.var_vth0_dn0 = 0.0;
        locals.var_vth0_dn2 = 0.0;
        locals.var_vth0_dn4 = 0.0;
        locals.var_vth0_dn5 = 0.0;
        locals.var_vth0_dn6 = 0.0;
        locals.var_vth0_dn7 = 0.0;
        locals.var_vth0_dn8 = 0.0;
        locals.var_vth0_dn9 = 0.0;
        locals.var_vth0_dn10 = 0.0;
        locals.var_vth0_dn13 = 0.0;
        locals.var_vth0_rv = 0.0;

        locals.var_dvth = 0.0;
        locals.var_dvth_dn0 = 0.0;
        locals.var_dvth_dn2 = 0.0;
        locals.var_dvth_dn4 = 0.0;
        locals.var_dvth_dn5 = 0.0;
        locals.var_dvth_dn6 = 0.0;
        locals.var_dvth_dn7 = 0.0;
        locals.var_dvth_dn8 = 0.0;
        locals.var_dvth_dn9 = 0.0;
        locals.var_dvth_dn10 = 0.0;
        locals.var_dvth_dn13 = 0.0;
        locals.var_dvth_rv = 0.0;

        locals.var_dvth0 = 0.0;
        locals.var_dvth0_dn0 = 0.0;
        locals.var_dvth0_dn2 = 0.0;
        locals.var_dvth0_dn4 = 0.0;
        locals.var_dvth0_dn5 = 0.0;
        locals.var_dvth0_dn6 = 0.0;
        locals.var_dvth0_dn7 = 0.0;
        locals.var_dvth0_dn8 = 0.0;
        locals.var_dvth0_dn9 = 0.0;
        locals.var_dvth0_dn10 = 0.0;
        locals.var_dvth0_dn13 = 0.0;
        locals.var_dvth0_rv = 0.0;

        locals.var_dvthsc = 0.0;
        locals.var_dvthsc_dn0 = 0.0;
        locals.var_dvthsc_dn2 = 0.0;
        locals.var_dvthsc_dn4 = 0.0;
        locals.var_dvthsc_dn5 = 0.0;
        locals.var_dvthsc_dn6 = 0.0;
        locals.var_dvthsc_dn7 = 0.0;
        locals.var_dvthsc_dn8 = 0.0;
        locals.var_dvthsc_dn9 = 0.0;
        locals.var_dvthsc_dn10 = 0.0;
        locals.var_dvthsc_dn13 = 0.0;
        locals.var_dvthsc_rv = 0.0;

        locals.var_pb20b = 0.0;
        locals.var_pb20b_dn0 = 0.0;
        locals.var_pb20b_dn2 = 0.0;
        locals.var_pb20b_dn4 = 0.0;
        locals.var_pb20b_dn5 = 0.0;
        locals.var_pb20b_dn6 = 0.0;
        locals.var_pb20b_dn7 = 0.0;
        locals.var_pb20b_dn8 = 0.0;
        locals.var_pb20b_dn9 = 0.0;
        locals.var_pb20b_dn10 = 0.0;
        locals.var_pb20b_dn13 = 0.0;
        locals.var_pb20b_rv = 0.0;

        locals.var_dvthw = 0.0;
        locals.var_dvthw_dn0 = 0.0;
        locals.var_dvthw_dn2 = 0.0;
        locals.var_dvthw_dn4 = 0.0;
        locals.var_dvthw_dn5 = 0.0;
        locals.var_dvthw_dn6 = 0.0;
        locals.var_dvthw_dn7 = 0.0;
        locals.var_dvthw_dn8 = 0.0;
        locals.var_dvthw_dn9 = 0.0;
        locals.var_dvthw_dn10 = 0.0;
        locals.var_dvthw_dn13 = 0.0;
        locals.var_dvthw_rv = 0.0;

        locals.var_alpha = 0.0;
        locals.var_alpha_dn0 = 0.0;
        locals.var_alpha_dn2 = 0.0;
        locals.var_alpha_dn4 = 0.0;
        locals.var_alpha_dn5 = 0.0;
        locals.var_alpha_dn6 = 0.0;
        locals.var_alpha_dn7 = 0.0;
        locals.var_alpha_dn8 = 0.0;
        locals.var_alpha_dn9 = 0.0;
        locals.var_alpha_dn10 = 0.0;
        locals.var_alpha_dn13 = 0.0;
        locals.var_alpha_rv = 0.0;

        locals.var_achi = 0.0;
        locals.var_achi_dn0 = 0.0;
        locals.var_achi_dn2 = 0.0;
        locals.var_achi_dn4 = 0.0;
        locals.var_achi_dn5 = 0.0;
        locals.var_achi_dn6 = 0.0;
        locals.var_achi_dn7 = 0.0;
        locals.var_achi_dn8 = 0.0;
        locals.var_achi_dn9 = 0.0;
        locals.var_achi_dn10 = 0.0;
        locals.var_achi_dn13 = 0.0;
        locals.var_achi_rv = 0.0;

        locals.var_vgvt = 0.0;
        locals.var_vgvt_dn0 = 0.0;
        locals.var_vgvt_dn2 = 0.0;
        locals.var_vgvt_dn4 = 0.0;
        locals.var_vgvt_dn5 = 0.0;
        locals.var_vgvt_dn6 = 0.0;
        locals.var_vgvt_dn7 = 0.0;
        locals.var_vgvt_dn8 = 0.0;
        locals.var_vgvt_dn9 = 0.0;
        locals.var_vgvt_dn10 = 0.0;
        locals.var_vgvt_dn13 = 0.0;
        locals.var_vgvt_rv = 0.0;

        locals.var_pslsat = 0.0;
        locals.var_pslsat_dn0 = 0.0;
        locals.var_pslsat_dn2 = 0.0;
        locals.var_pslsat_dn4 = 0.0;
        locals.var_pslsat_dn5 = 0.0;
        locals.var_pslsat_dn6 = 0.0;
        locals.var_pslsat_dn7 = 0.0;
        locals.var_pslsat_dn8 = 0.0;
        locals.var_pslsat_dn9 = 0.0;
        locals.var_pslsat_dn10 = 0.0;
        locals.var_pslsat_dn13 = 0.0;
        locals.var_pslsat_rv = 0.0;

        locals.var_vdsats = 0.0;
        locals.var_vdsats_dn0 = 0.0;
        locals.var_vdsats_dn2 = 0.0;
        locals.var_vdsats_dn4 = 0.0;
        locals.var_vdsats_dn5 = 0.0;
        locals.var_vdsats_dn6 = 0.0;
        locals.var_vdsats_dn7 = 0.0;
        locals.var_vdsats_dn8 = 0.0;
        locals.var_vdsats_dn9 = 0.0;
        locals.var_vdsats_dn10 = 0.0;
        locals.var_vdsats_dn13 = 0.0;
        locals.var_vdsats_rv = 0.0;

        locals.var_delta = 0.0;
        locals.var_delta_dn0 = 0.0;
        locals.var_delta_dn2 = 0.0;
        locals.var_delta_dn4 = 0.0;
        locals.var_delta_dn5 = 0.0;
        locals.var_delta_dn6 = 0.0;
        locals.var_delta_dn7 = 0.0;
        locals.var_delta_dn8 = 0.0;
        locals.var_delta_dn9 = 0.0;
        locals.var_delta_dn10 = 0.0;
        locals.var_delta_dn13 = 0.0;
        locals.var_delta_rv = 0.0;

        locals.var_qb = 0.0;
        locals.var_qb_dn0 = 0.0;
        locals.var_qb_dn2 = 0.0;
        locals.var_qb_dn4 = 0.0;
        locals.var_qb_dn5 = 0.0;
        locals.var_qb_dn6 = 0.0;
        locals.var_qb_dn7 = 0.0;
        locals.var_qb_dn8 = 0.0;
        locals.var_qb_dn9 = 0.0;
        locals.var_qb_dn10 = 0.0;
        locals.var_qb_dn13 = 0.0;
        locals.var_qb_rv = 0.0;

        locals.var_qbu = 0.0;
        locals.var_qbu_dn0 = 0.0;
        locals.var_qbu_dn2 = 0.0;
        locals.var_qbu_dn4 = 0.0;
        locals.var_qbu_dn5 = 0.0;
        locals.var_qbu_dn6 = 0.0;
        locals.var_qbu_dn7 = 0.0;
        locals.var_qbu_dn8 = 0.0;
        locals.var_qbu_dn9 = 0.0;
        locals.var_qbu_dn10 = 0.0;
        locals.var_qbu_dn13 = 0.0;
        locals.var_qbu_rv = 0.0;

        locals.var_qi = 0.0;
        locals.var_qi_dn0 = 0.0;
        locals.var_qi_dn2 = 0.0;
        locals.var_qi_dn4 = 0.0;
        locals.var_qi_dn5 = 0.0;
        locals.var_qi_dn6 = 0.0;
        locals.var_qi_dn7 = 0.0;
        locals.var_qi_dn8 = 0.0;
        locals.var_qi_dn9 = 0.0;
        locals.var_qi_dn10 = 0.0;
        locals.var_qi_dn13 = 0.0;
        locals.var_qi_rv = 0.0;

        locals.var_qiu = 0.0;
        locals.var_qiu_dn0 = 0.0;
        locals.var_qiu_dn2 = 0.0;
        locals.var_qiu_dn4 = 0.0;
        locals.var_qiu_dn5 = 0.0;
        locals.var_qiu_dn6 = 0.0;
        locals.var_qiu_dn7 = 0.0;
        locals.var_qiu_dn8 = 0.0;
        locals.var_qiu_dn9 = 0.0;
        locals.var_qiu_dn10 = 0.0;
        locals.var_qiu_dn13 = 0.0;
        locals.var_qiu_rv = 0.0;

        locals.var_qd = 0.0;
        locals.var_qd_dn0 = 0.0;
        locals.var_qd_dn2 = 0.0;
        locals.var_qd_dn4 = 0.0;
        locals.var_qd_dn5 = 0.0;
        locals.var_qd_dn6 = 0.0;
        locals.var_qd_dn7 = 0.0;
        locals.var_qd_dn8 = 0.0;
        locals.var_qd_dn9 = 0.0;
        locals.var_qd_dn10 = 0.0;
        locals.var_qd_dn13 = 0.0;
        locals.var_qd_rv = 0.0;

        locals.var_ids = 0.0;
        locals.var_ids_dn0 = 0.0;
        locals.var_ids_dn2 = 0.0;
        locals.var_ids_dn4 = 0.0;
        locals.var_ids_dn5 = 0.0;
        locals.var_ids_dn6 = 0.0;
        locals.var_ids_dn7 = 0.0;
        locals.var_ids_dn8 = 0.0;
        locals.var_ids_dn9 = 0.0;
        locals.var_ids_dn10 = 0.0;
        locals.var_ids_dn13 = 0.0;
        locals.var_ids_rv = 0.0;

        locals.var_ids0 = 0.0;
        locals.var_ids0_dn0 = 0.0;
        locals.var_ids0_dn2 = 0.0;
        locals.var_ids0_dn4 = 0.0;
        locals.var_ids0_dn5 = 0.0;
        locals.var_ids0_dn6 = 0.0;
        locals.var_ids0_dn7 = 0.0;
        locals.var_ids0_dn8 = 0.0;
        locals.var_ids0_dn9 = 0.0;
        locals.var_ids0_dn10 = 0.0;
        locals.var_ids0_dn13 = 0.0;
        locals.var_ids0_rv = 0.0;

        locals.var_dvthscsti = 0.0;
        locals.var_dvthscsti_dn0 = 0.0;
        locals.var_dvthscsti_dn2 = 0.0;
        locals.var_dvthscsti_dn4 = 0.0;
        locals.var_dvthscsti_dn5 = 0.0;
        locals.var_dvthscsti_dn6 = 0.0;
        locals.var_dvthscsti_dn7 = 0.0;
        locals.var_dvthscsti_dn8 = 0.0;
        locals.var_dvthscsti_dn9 = 0.0;
        locals.var_dvthscsti_dn10 = 0.0;
        locals.var_dvthscsti_dn13 = 0.0;
        locals.var_dvthscsti_rv = 0.0;

        locals.var_vgssti = 0.0;
        locals.var_vgssti_dn0 = 0.0;
        locals.var_vgssti_dn2 = 0.0;
        locals.var_vgssti_dn4 = 0.0;
        locals.var_vgssti_dn5 = 0.0;
        locals.var_vgssti_dn6 = 0.0;
        locals.var_vgssti_dn7 = 0.0;
        locals.var_vgssti_dn8 = 0.0;
        locals.var_vgssti_dn9 = 0.0;
        locals.var_vgssti_dn10 = 0.0;
        locals.var_vgssti_dn13 = 0.0;
        locals.var_vgssti_rv = 0.0;

        locals.var_costi0 = 0.0;
        locals.var_costi0_dn0 = 0.0;
        locals.var_costi0_dn2 = 0.0;
        locals.var_costi0_dn4 = 0.0;
        locals.var_costi0_dn5 = 0.0;
        locals.var_costi0_dn6 = 0.0;
        locals.var_costi0_dn7 = 0.0;
        locals.var_costi0_dn8 = 0.0;
        locals.var_costi0_dn9 = 0.0;
        locals.var_costi0_dn10 = 0.0;
        locals.var_costi0_dn13 = 0.0;
        locals.var_costi0_rv = 0.0;

        locals.var_costi1 = 0.0;
        locals.var_costi1_dn0 = 0.0;
        locals.var_costi1_dn2 = 0.0;
        locals.var_costi1_dn4 = 0.0;
        locals.var_costi1_dn5 = 0.0;
        locals.var_costi1_dn6 = 0.0;
        locals.var_costi1_dn7 = 0.0;
        locals.var_costi1_dn8 = 0.0;
        locals.var_costi1_dn9 = 0.0;
        locals.var_costi1_dn10 = 0.0;
        locals.var_costi1_dn13 = 0.0;
        locals.var_costi1_rv = 0.0;

        locals.var_costi3 = 0.0;
        locals.var_costi3_dn0 = 0.0;
        locals.var_costi3_dn2 = 0.0;
        locals.var_costi3_dn4 = 0.0;
        locals.var_costi3_dn5 = 0.0;
        locals.var_costi3_dn6 = 0.0;
        locals.var_costi3_dn7 = 0.0;
        locals.var_costi3_dn8 = 0.0;
        locals.var_costi3_dn9 = 0.0;
        locals.var_costi3_dn10 = 0.0;
        locals.var_costi3_dn13 = 0.0;
        locals.var_costi3_rv = 0.0;

        locals.var_costi4 = 0.0;
        locals.var_costi4_dn0 = 0.0;
        locals.var_costi4_dn2 = 0.0;
        locals.var_costi4_dn4 = 0.0;
        locals.var_costi4_dn5 = 0.0;
        locals.var_costi4_dn6 = 0.0;
        locals.var_costi4_dn7 = 0.0;
        locals.var_costi4_dn8 = 0.0;
        locals.var_costi4_dn9 = 0.0;
        locals.var_costi4_dn10 = 0.0;
        locals.var_costi4_dn13 = 0.0;
        locals.var_costi4_rv = 0.0;

        locals.var_costi5 = 0.0;
        locals.var_costi5_dn0 = 0.0;
        locals.var_costi5_dn2 = 0.0;
        locals.var_costi5_dn4 = 0.0;
        locals.var_costi5_dn5 = 0.0;
        locals.var_costi5_dn6 = 0.0;
        locals.var_costi5_dn7 = 0.0;
        locals.var_costi5_dn8 = 0.0;
        locals.var_costi5_dn9 = 0.0;
        locals.var_costi5_dn10 = 0.0;
        locals.var_costi5_dn13 = 0.0;
        locals.var_costi5_rv = 0.0;

        locals.var_costi6 = 0.0;
        locals.var_costi6_dn0 = 0.0;
        locals.var_costi6_dn2 = 0.0;
        locals.var_costi6_dn4 = 0.0;
        locals.var_costi6_dn5 = 0.0;
        locals.var_costi6_dn6 = 0.0;
        locals.var_costi6_dn7 = 0.0;
        locals.var_costi6_dn8 = 0.0;
        locals.var_costi6_dn9 = 0.0;
        locals.var_costi6_dn10 = 0.0;
        locals.var_costi6_dn13 = 0.0;
        locals.var_costi6_rv = 0.0;

        locals.var_costi7 = 0.0;
        locals.var_costi7_dn0 = 0.0;
        locals.var_costi7_dn2 = 0.0;
        locals.var_costi7_dn4 = 0.0;
        locals.var_costi7_dn5 = 0.0;
        locals.var_costi7_dn6 = 0.0;
        locals.var_costi7_dn7 = 0.0;
        locals.var_costi7_dn8 = 0.0;
        locals.var_costi7_dn9 = 0.0;
        locals.var_costi7_dn10 = 0.0;
        locals.var_costi7_dn13 = 0.0;
        locals.var_costi7_rv = 0.0;

        locals.var_psasti = 0.0;
        locals.var_psasti_dn0 = 0.0;
        locals.var_psasti_dn2 = 0.0;
        locals.var_psasti_dn4 = 0.0;
        locals.var_psasti_dn5 = 0.0;
        locals.var_psasti_dn6 = 0.0;
        locals.var_psasti_dn7 = 0.0;
        locals.var_psasti_dn8 = 0.0;
        locals.var_psasti_dn9 = 0.0;
        locals.var_psasti_dn10 = 0.0;
        locals.var_psasti_dn13 = 0.0;
        locals.var_psasti_rv = 0.0;

        locals.var_psbsti = 0.0;
        locals.var_psbsti_dn0 = 0.0;
        locals.var_psbsti_dn2 = 0.0;
        locals.var_psbsti_dn4 = 0.0;
        locals.var_psbsti_dn5 = 0.0;
        locals.var_psbsti_dn6 = 0.0;
        locals.var_psbsti_dn7 = 0.0;
        locals.var_psbsti_dn8 = 0.0;
        locals.var_psbsti_dn9 = 0.0;
        locals.var_psbsti_dn10 = 0.0;
        locals.var_psbsti_dn13 = 0.0;
        locals.var_psbsti_rv = 0.0;

        locals.var_psab = 0.0;
        locals.var_psab_dn0 = 0.0;
        locals.var_psab_dn2 = 0.0;
        locals.var_psab_dn4 = 0.0;
        locals.var_psab_dn5 = 0.0;
        locals.var_psab_dn6 = 0.0;
        locals.var_psab_dn7 = 0.0;
        locals.var_psab_dn8 = 0.0;
        locals.var_psab_dn9 = 0.0;
        locals.var_psab_dn10 = 0.0;
        locals.var_psab_dn13 = 0.0;
        locals.var_psab_rv = 0.0;

        locals.var_psti = 0.0;
        locals.var_psti_dn0 = 0.0;
        locals.var_psti_dn2 = 0.0;
        locals.var_psti_dn4 = 0.0;
        locals.var_psti_dn5 = 0.0;
        locals.var_psti_dn6 = 0.0;
        locals.var_psti_dn7 = 0.0;
        locals.var_psti_dn8 = 0.0;
        locals.var_psti_dn9 = 0.0;
        locals.var_psti_dn10 = 0.0;
        locals.var_psti_dn13 = 0.0;
        locals.var_psti_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_2(
        locals: &mut StampLocals,
    ) {
        locals.var_sq1sti = 0.0;
        locals.var_sq1sti_dn0 = 0.0;
        locals.var_sq1sti_dn2 = 0.0;
        locals.var_sq1sti_dn4 = 0.0;
        locals.var_sq1sti_dn5 = 0.0;
        locals.var_sq1sti_dn6 = 0.0;
        locals.var_sq1sti_dn7 = 0.0;
        locals.var_sq1sti_dn8 = 0.0;
        locals.var_sq1sti_dn9 = 0.0;
        locals.var_sq1sti_dn10 = 0.0;
        locals.var_sq1sti_dn13 = 0.0;
        locals.var_sq1sti_rv = 0.0;

        locals.var_sq2sti = 0.0;
        locals.var_sq2sti_dn0 = 0.0;
        locals.var_sq2sti_dn2 = 0.0;
        locals.var_sq2sti_dn4 = 0.0;
        locals.var_sq2sti_dn5 = 0.0;
        locals.var_sq2sti_dn6 = 0.0;
        locals.var_sq2sti_dn7 = 0.0;
        locals.var_sq2sti_dn8 = 0.0;
        locals.var_sq2sti_dn9 = 0.0;
        locals.var_sq2sti_dn10 = 0.0;
        locals.var_sq2sti_dn13 = 0.0;
        locals.var_sq2sti_rv = 0.0;

        locals.var_qn0sti = 0.0;
        locals.var_qn0sti_dn0 = 0.0;
        locals.var_qn0sti_dn2 = 0.0;
        locals.var_qn0sti_dn4 = 0.0;
        locals.var_qn0sti_dn5 = 0.0;
        locals.var_qn0sti_dn6 = 0.0;
        locals.var_qn0sti_dn7 = 0.0;
        locals.var_qn0sti_dn8 = 0.0;
        locals.var_qn0sti_dn9 = 0.0;
        locals.var_qn0sti_dn10 = 0.0;
        locals.var_qn0sti_dn13 = 0.0;
        locals.var_qn0sti_rv = 0.0;

        locals.var_idssti = 0.0;
        locals.var_idssti_dn0 = 0.0;
        locals.var_idssti_dn2 = 0.0;
        locals.var_idssti_dn4 = 0.0;
        locals.var_idssti_dn5 = 0.0;
        locals.var_idssti_dn6 = 0.0;
        locals.var_idssti_dn7 = 0.0;
        locals.var_idssti_dn8 = 0.0;
        locals.var_idssti_dn9 = 0.0;
        locals.var_idssti_dn10 = 0.0;
        locals.var_idssti_dn13 = 0.0;
        locals.var_idssti_rv = 0.0;

        locals.var_beta = 0.0;
        locals.var_beta_dn0 = 0.0;
        locals.var_beta_dn2 = 0.0;
        locals.var_beta_dn4 = 0.0;
        locals.var_beta_dn5 = 0.0;
        locals.var_beta_dn6 = 0.0;
        locals.var_beta_dn7 = 0.0;
        locals.var_beta_dn8 = 0.0;
        locals.var_beta_dn9 = 0.0;
        locals.var_beta_dn10 = 0.0;
        locals.var_beta_dn13 = 0.0;
        locals.var_beta_rv = 0.0;

        locals.var_beta_inv = 0.0;
        locals.var_beta_inv_dn0 = 0.0;
        locals.var_beta_inv_dn2 = 0.0;
        locals.var_beta_inv_dn4 = 0.0;
        locals.var_beta_inv_dn5 = 0.0;
        locals.var_beta_inv_dn6 = 0.0;
        locals.var_beta_inv_dn7 = 0.0;
        locals.var_beta_inv_dn8 = 0.0;
        locals.var_beta_inv_dn9 = 0.0;
        locals.var_beta_inv_dn10 = 0.0;
        locals.var_beta_inv_dn13 = 0.0;
        locals.var_beta_inv_rv = 0.0;

        locals.var_beta2 = 0.0;
        locals.var_beta2_dn0 = 0.0;
        locals.var_beta2_dn2 = 0.0;
        locals.var_beta2_dn4 = 0.0;
        locals.var_beta2_dn5 = 0.0;
        locals.var_beta2_dn6 = 0.0;
        locals.var_beta2_dn7 = 0.0;
        locals.var_beta2_dn8 = 0.0;
        locals.var_beta2_dn9 = 0.0;
        locals.var_beta2_dn10 = 0.0;
        locals.var_beta2_dn13 = 0.0;
        locals.var_beta2_rv = 0.0;

        locals.var_pb2 = 0.0;
        locals.var_pb2_dn0 = 0.0;
        locals.var_pb2_dn2 = 0.0;
        locals.var_pb2_dn4 = 0.0;
        locals.var_pb2_dn5 = 0.0;
        locals.var_pb2_dn6 = 0.0;
        locals.var_pb2_dn7 = 0.0;
        locals.var_pb2_dn8 = 0.0;
        locals.var_pb2_dn9 = 0.0;
        locals.var_pb2_dn10 = 0.0;
        locals.var_pb2_dn13 = 0.0;
        locals.var_pb2_rv = 0.0;

        locals.var_pb20 = 0.0;
        locals.var_pb20_dn0 = 0.0;
        locals.var_pb20_dn2 = 0.0;
        locals.var_pb20_dn4 = 0.0;
        locals.var_pb20_dn5 = 0.0;
        locals.var_pb20_dn6 = 0.0;
        locals.var_pb20_dn7 = 0.0;
        locals.var_pb20_dn8 = 0.0;
        locals.var_pb20_dn9 = 0.0;
        locals.var_pb20_dn10 = 0.0;
        locals.var_pb20_dn13 = 0.0;
        locals.var_pb20_rv = 0.0;

        locals.var_pb2c = 0.0;
        locals.var_pb2c_dn0 = 0.0;
        locals.var_pb2c_dn2 = 0.0;
        locals.var_pb2c_dn4 = 0.0;
        locals.var_pb2c_dn5 = 0.0;
        locals.var_pb2c_dn6 = 0.0;
        locals.var_pb2c_dn7 = 0.0;
        locals.var_pb2c_dn8 = 0.0;
        locals.var_pb2c_dn9 = 0.0;
        locals.var_pb2c_dn10 = 0.0;
        locals.var_pb2c_dn13 = 0.0;
        locals.var_pb2c_rv = 0.0;

        locals.var_vfb = 0.0;
        locals.var_vfb_rv = 0.0;

        locals.var_c_eox = 0.0;
        locals.var_c_eox_rv = 0.0;

        locals.var_leff = 0.0;
        locals.var_leff_rv = 0.0;

        locals.var_weff = 0.0;
        locals.var_weff_rv = 0.0;

        locals.var_weffld_nf = 0.0;
        locals.var_weffld_nf_rv = 0.0;

        locals.var_ldrift0 = 0.0;
        locals.var_ldrift0_rv = 0.0;

        locals.var_q_nsub = 0.0;
        locals.var_q_nsub_dn0 = 0.0;
        locals.var_q_nsub_dn2 = 0.0;
        locals.var_q_nsub_dn4 = 0.0;
        locals.var_q_nsub_dn5 = 0.0;
        locals.var_q_nsub_dn6 = 0.0;
        locals.var_q_nsub_dn7 = 0.0;
        locals.var_q_nsub_dn8 = 0.0;
        locals.var_q_nsub_dn9 = 0.0;
        locals.var_q_nsub_dn10 = 0.0;
        locals.var_q_nsub_dn13 = 0.0;
        locals.var_q_nsub_rv = 0.0;

        locals.var_psa = 0.0;
        locals.var_psa_dn0 = 0.0;
        locals.var_psa_dn2 = 0.0;
        locals.var_psa_dn4 = 0.0;
        locals.var_psa_dn5 = 0.0;
        locals.var_psa_dn6 = 0.0;
        locals.var_psa_dn7 = 0.0;
        locals.var_psa_dn8 = 0.0;
        locals.var_psa_dn9 = 0.0;
        locals.var_psa_dn10 = 0.0;
        locals.var_psa_dn13 = 0.0;
        locals.var_psa_rv = 0.0;

        locals.var_psdl = 0.0;
        locals.var_psdl_dn0 = 0.0;
        locals.var_psdl_dn2 = 0.0;
        locals.var_psdl_dn4 = 0.0;
        locals.var_psdl_dn5 = 0.0;
        locals.var_psdl_dn6 = 0.0;
        locals.var_psdl_dn7 = 0.0;
        locals.var_psdl_dn8 = 0.0;
        locals.var_psdl_dn9 = 0.0;
        locals.var_psdl_dn10 = 0.0;
        locals.var_psdl_dn13 = 0.0;
        locals.var_psdl_rv = 0.0;

        locals.var_lred = 0.0;
        locals.var_lred_dn0 = 0.0;
        locals.var_lred_dn2 = 0.0;
        locals.var_lred_dn4 = 0.0;
        locals.var_lred_dn5 = 0.0;
        locals.var_lred_dn6 = 0.0;
        locals.var_lred_dn7 = 0.0;
        locals.var_lred_dn8 = 0.0;
        locals.var_lred_dn9 = 0.0;
        locals.var_lred_dn10 = 0.0;
        locals.var_lred_dn13 = 0.0;
        locals.var_lred_rv = 0.0;

        locals.var_lch = 0.0;
        locals.var_lch_dn0 = 0.0;
        locals.var_lch_dn2 = 0.0;
        locals.var_lch_dn4 = 0.0;
        locals.var_lch_dn5 = 0.0;
        locals.var_lch_dn6 = 0.0;
        locals.var_lch_dn7 = 0.0;
        locals.var_lch_dn8 = 0.0;
        locals.var_lch_dn9 = 0.0;
        locals.var_lch_dn10 = 0.0;
        locals.var_lch_dn13 = 0.0;
        locals.var_lch_rv = 0.0;

        locals.var_wd = 0.0;
        locals.var_wd_dn0 = 0.0;
        locals.var_wd_dn2 = 0.0;
        locals.var_wd_dn4 = 0.0;
        locals.var_wd_dn5 = 0.0;
        locals.var_wd_dn6 = 0.0;
        locals.var_wd_dn7 = 0.0;
        locals.var_wd_dn8 = 0.0;
        locals.var_wd_dn9 = 0.0;
        locals.var_wd_dn10 = 0.0;
        locals.var_wd_dn13 = 0.0;
        locals.var_wd_rv = 0.0;

        locals.var_aclm = 0.0;
        locals.var_aclm_rv = 0.0;

        locals.var_vthp = 0.0;
        locals.var_vthp_dn0 = 0.0;
        locals.var_vthp_dn2 = 0.0;
        locals.var_vthp_dn4 = 0.0;
        locals.var_vthp_dn5 = 0.0;
        locals.var_vthp_dn6 = 0.0;
        locals.var_vthp_dn7 = 0.0;
        locals.var_vthp_dn8 = 0.0;
        locals.var_vthp_dn9 = 0.0;
        locals.var_vthp_dn10 = 0.0;
        locals.var_vthp_dn13 = 0.0;
        locals.var_vthp_rv = 0.0;

        locals.var_dvthlp = 0.0;
        locals.var_dvthlp_dn0 = 0.0;
        locals.var_dvthlp_dn2 = 0.0;
        locals.var_dvthlp_dn4 = 0.0;
        locals.var_dvthlp_dn5 = 0.0;
        locals.var_dvthlp_dn6 = 0.0;
        locals.var_dvthlp_dn7 = 0.0;
        locals.var_dvthlp_dn8 = 0.0;
        locals.var_dvthlp_dn9 = 0.0;
        locals.var_dvthlp_dn10 = 0.0;
        locals.var_dvthlp_dn13 = 0.0;
        locals.var_dvthlp_rv = 0.0;

        locals.var_bs12 = 0.0;
        locals.var_bs12_dn0 = 0.0;
        locals.var_bs12_dn2 = 0.0;
        locals.var_bs12_dn4 = 0.0;
        locals.var_bs12_dn5 = 0.0;
        locals.var_bs12_dn6 = 0.0;
        locals.var_bs12_dn7 = 0.0;
        locals.var_bs12_dn8 = 0.0;
        locals.var_bs12_dn9 = 0.0;
        locals.var_bs12_dn10 = 0.0;
        locals.var_bs12_dn13 = 0.0;
        locals.var_bs12_rv = 0.0;

        locals.var_qbmm = 0.0;
        locals.var_qbmm_dn0 = 0.0;
        locals.var_qbmm_dn2 = 0.0;
        locals.var_qbmm_dn4 = 0.0;
        locals.var_qbmm_dn5 = 0.0;
        locals.var_qbmm_dn6 = 0.0;
        locals.var_qbmm_dn7 = 0.0;
        locals.var_qbmm_dn8 = 0.0;
        locals.var_qbmm_dn9 = 0.0;
        locals.var_qbmm_dn10 = 0.0;
        locals.var_qbmm_dn13 = 0.0;
        locals.var_qbmm_rv = 0.0;

        locals.var_dqb = 0.0;
        locals.var_dqb_dn0 = 0.0;
        locals.var_dqb_dn2 = 0.0;
        locals.var_dqb_dn4 = 0.0;
        locals.var_dqb_dn5 = 0.0;
        locals.var_dqb_dn6 = 0.0;
        locals.var_dqb_dn7 = 0.0;
        locals.var_dqb_dn8 = 0.0;
        locals.var_dqb_dn9 = 0.0;
        locals.var_dqb_dn10 = 0.0;
        locals.var_dqb_dn13 = 0.0;
        locals.var_dqb_rv = 0.0;

        locals.var_vdx = 0.0;
        locals.var_vdx_dn0 = 0.0;
        locals.var_vdx_dn2 = 0.0;
        locals.var_vdx_dn4 = 0.0;
        locals.var_vdx_dn5 = 0.0;
        locals.var_vdx_dn6 = 0.0;
        locals.var_vdx_dn7 = 0.0;
        locals.var_vdx_dn8 = 0.0;
        locals.var_vdx_dn9 = 0.0;
        locals.var_vdx_dn10 = 0.0;
        locals.var_vdx_dn13 = 0.0;
        locals.var_vdx_rv = 0.0;

        locals.var_vdx2 = 0.0;
        locals.var_vdx2_dn0 = 0.0;
        locals.var_vdx2_dn2 = 0.0;
        locals.var_vdx2_dn4 = 0.0;
        locals.var_vdx2_dn5 = 0.0;
        locals.var_vdx2_dn6 = 0.0;
        locals.var_vdx2_dn7 = 0.0;
        locals.var_vdx2_dn8 = 0.0;
        locals.var_vdx2_dn9 = 0.0;
        locals.var_vdx2_dn10 = 0.0;
        locals.var_vdx2_dn13 = 0.0;
        locals.var_vdx2_rv = 0.0;

        locals.var_pbsum = 0.0;
        locals.var_pbsum_dn0 = 0.0;
        locals.var_pbsum_dn2 = 0.0;
        locals.var_pbsum_dn4 = 0.0;
        locals.var_pbsum_dn5 = 0.0;
        locals.var_pbsum_dn6 = 0.0;
        locals.var_pbsum_dn7 = 0.0;
        locals.var_pbsum_dn8 = 0.0;
        locals.var_pbsum_dn9 = 0.0;
        locals.var_pbsum_dn10 = 0.0;
        locals.var_pbsum_dn13 = 0.0;
        locals.var_pbsum_rv = 0.0;

        locals.var_sqrt_pbsum = 0.0;
        locals.var_sqrt_pbsum_dn0 = 0.0;
        locals.var_sqrt_pbsum_dn2 = 0.0;
        locals.var_sqrt_pbsum_dn4 = 0.0;
        locals.var_sqrt_pbsum_dn5 = 0.0;
        locals.var_sqrt_pbsum_dn6 = 0.0;
        locals.var_sqrt_pbsum_dn7 = 0.0;
        locals.var_sqrt_pbsum_dn8 = 0.0;
        locals.var_sqrt_pbsum_dn9 = 0.0;
        locals.var_sqrt_pbsum_dn10 = 0.0;
        locals.var_sqrt_pbsum_dn13 = 0.0;
        locals.var_sqrt_pbsum_rv = 0.0;

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

        locals.var_dtox = 0.0;
        locals.var_dtox_dn0 = 0.0;
        locals.var_dtox_dn2 = 0.0;
        locals.var_dtox_dn4 = 0.0;
        locals.var_dtox_dn5 = 0.0;
        locals.var_dtox_dn6 = 0.0;
        locals.var_dtox_dn7 = 0.0;
        locals.var_dtox_dn8 = 0.0;
        locals.var_dtox_dn9 = 0.0;
        locals.var_dtox_dn10 = 0.0;
        locals.var_dtox_dn13 = 0.0;
        locals.var_dtox_rv = 0.0;

        locals.var_cox = 0.0;
        locals.var_cox_dn0 = 0.0;
        locals.var_cox_dn2 = 0.0;
        locals.var_cox_dn4 = 0.0;
        locals.var_cox_dn5 = 0.0;
        locals.var_cox_dn6 = 0.0;
        locals.var_cox_dn7 = 0.0;
        locals.var_cox_dn8 = 0.0;
        locals.var_cox_dn9 = 0.0;
        locals.var_cox_dn10 = 0.0;
        locals.var_cox_dn13 = 0.0;
        locals.var_cox_rv = 0.0;

        locals.var_cox_inv = 0.0;
        locals.var_cox_inv_dn0 = 0.0;
        locals.var_cox_inv_dn2 = 0.0;
        locals.var_cox_inv_dn4 = 0.0;
        locals.var_cox_inv_dn5 = 0.0;
        locals.var_cox_inv_dn6 = 0.0;
        locals.var_cox_inv_dn7 = 0.0;
        locals.var_cox_inv_dn8 = 0.0;
        locals.var_cox_inv_dn9 = 0.0;
        locals.var_cox_inv_dn10 = 0.0;
        locals.var_cox_inv_dn13 = 0.0;
        locals.var_cox_inv_rv = 0.0;

        locals.var_tox0 = 0.0;
        locals.var_tox0_rv = 0.0;

        locals.var_cox0 = 0.0;
        locals.var_cox0_rv = 0.0;

        locals.var_coxb0 = 0.0;
        locals.var_coxb0_rv = 0.0;

        locals.var_cox0_inv = 0.0;
        locals.var_cox0_inv_rv = 0.0;

        locals.var_vthq = 0.0;
        locals.var_vthq_dn0 = 0.0;
        locals.var_vthq_dn2 = 0.0;
        locals.var_vthq_dn4 = 0.0;
        locals.var_vthq_dn5 = 0.0;
        locals.var_vthq_dn6 = 0.0;
        locals.var_vthq_dn7 = 0.0;
        locals.var_vthq_dn8 = 0.0;
        locals.var_vthq_dn9 = 0.0;
        locals.var_vthq_dn10 = 0.0;
        locals.var_vthq_dn13 = 0.0;
        locals.var_vthq_rv = 0.0;

        locals.var_psdlz = 0.0;
        locals.var_psdlz_dn0 = 0.0;
        locals.var_psdlz_dn2 = 0.0;
        locals.var_psdlz_dn4 = 0.0;
        locals.var_psdlz_dn5 = 0.0;
        locals.var_psdlz_dn6 = 0.0;
        locals.var_psdlz_dn7 = 0.0;
        locals.var_psdlz_dn8 = 0.0;
        locals.var_psdlz_dn9 = 0.0;
        locals.var_psdlz_dn10 = 0.0;
        locals.var_psdlz_dn13 = 0.0;
        locals.var_psdlz_rv = 0.0;

        locals.var_egp12 = 0.0;
        locals.var_egp12_dn0 = 0.0;
        locals.var_egp12_dn2 = 0.0;
        locals.var_egp12_dn4 = 0.0;
        locals.var_egp12_dn5 = 0.0;
        locals.var_egp12_dn6 = 0.0;
        locals.var_egp12_dn7 = 0.0;
        locals.var_egp12_dn8 = 0.0;
        locals.var_egp12_dn9 = 0.0;
        locals.var_egp12_dn10 = 0.0;
        locals.var_egp12_dn13 = 0.0;
        locals.var_egp12_rv = 0.0;

        locals.var_egp32 = 0.0;
        locals.var_egp32_dn0 = 0.0;
        locals.var_egp32_dn2 = 0.0;
        locals.var_egp32_dn4 = 0.0;
        locals.var_egp32_dn5 = 0.0;
        locals.var_egp32_dn6 = 0.0;
        locals.var_egp32_dn7 = 0.0;
        locals.var_egp32_dn8 = 0.0;
        locals.var_egp32_dn9 = 0.0;
        locals.var_egp32_dn10 = 0.0;
        locals.var_egp32_dn13 = 0.0;
        locals.var_egp32_rv = 0.0;

        locals.var_e1 = 0.0;
        locals.var_e1_dn0 = 0.0;
        locals.var_e1_dn2 = 0.0;
        locals.var_e1_dn4 = 0.0;
        locals.var_e1_dn5 = 0.0;
        locals.var_e1_dn6 = 0.0;
        locals.var_e1_dn7 = 0.0;
        locals.var_e1_dn8 = 0.0;
        locals.var_e1_dn9 = 0.0;
        locals.var_e1_dn10 = 0.0;
        locals.var_e1_dn13 = 0.0;
        locals.var_e1_rv = 0.0;

        locals.var_etun = 0.0;
        locals.var_etun_dn0 = 0.0;
        locals.var_etun_dn2 = 0.0;
        locals.var_etun_dn4 = 0.0;
        locals.var_etun_dn5 = 0.0;
        locals.var_etun_dn6 = 0.0;
        locals.var_etun_dn7 = 0.0;
        locals.var_etun_dn8 = 0.0;
        locals.var_etun_dn9 = 0.0;
        locals.var_etun_dn10 = 0.0;
        locals.var_etun_dn13 = 0.0;
        locals.var_etun_rv = 0.0;

        locals.var_vdsp = 0.0;
        locals.var_vdsp_dn0 = 0.0;
        locals.var_vdsp_dn2 = 0.0;
        locals.var_vdsp_dn4 = 0.0;
        locals.var_vdsp_dn5 = 0.0;
        locals.var_vdsp_dn6 = 0.0;
        locals.var_vdsp_dn7 = 0.0;
        locals.var_vdsp_dn8 = 0.0;
        locals.var_vdsp_dn9 = 0.0;
        locals.var_vdsp_dn10 = 0.0;
        locals.var_vdsp_dn13 = 0.0;
        locals.var_vdsp_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_3(
        locals: &mut StampLocals,
    ) {
        locals.var_egidl = 0.0;
        locals.var_egidl_dn0 = 0.0;
        locals.var_egidl_dn2 = 0.0;
        locals.var_egidl_dn4 = 0.0;
        locals.var_egidl_dn5 = 0.0;
        locals.var_egidl_dn6 = 0.0;
        locals.var_egidl_dn7 = 0.0;
        locals.var_egidl_dn8 = 0.0;
        locals.var_egidl_dn9 = 0.0;
        locals.var_egidl_dn10 = 0.0;
        locals.var_egidl_dn13 = 0.0;
        locals.var_egidl_rv = 0.0;

        locals.var_egisl = 0.0;
        locals.var_egisl_dn0 = 0.0;
        locals.var_egisl_dn2 = 0.0;
        locals.var_egisl_dn4 = 0.0;
        locals.var_egisl_dn5 = 0.0;
        locals.var_egisl_dn6 = 0.0;
        locals.var_egisl_dn7 = 0.0;
        locals.var_egisl_dn8 = 0.0;
        locals.var_egisl_dn9 = 0.0;
        locals.var_egisl_dn10 = 0.0;
        locals.var_egisl_dn13 = 0.0;
        locals.var_egisl_rv = 0.0;

        locals.var_vdb = 0.0;
        locals.var_vdb_dn0 = 0.0;
        locals.var_vdb_dn2 = 0.0;
        locals.var_vdb_dn4 = 0.0;
        locals.var_vdb_dn5 = 0.0;
        locals.var_vdb_dn6 = 0.0;
        locals.var_vdb_dn7 = 0.0;
        locals.var_vdb_dn8 = 0.0;
        locals.var_vdb_dn9 = 0.0;
        locals.var_vdb_dn10 = 0.0;
        locals.var_vdb_dn13 = 0.0;
        locals.var_vdb_rv = 0.0;

        locals.var_vsb = 0.0;
        locals.var_vsb_dn5 = 0.0;
        locals.var_vsb_dn7 = 0.0;
        locals.var_vsb_dn8 = 0.0;
        locals.var_vsb_rv = 0.0;

        locals.var_fd2 = 0.0;
        locals.var_fd2_dn0 = 0.0;
        locals.var_fd2_dn2 = 0.0;
        locals.var_fd2_dn4 = 0.0;
        locals.var_fd2_dn5 = 0.0;
        locals.var_fd2_dn6 = 0.0;
        locals.var_fd2_dn7 = 0.0;
        locals.var_fd2_dn8 = 0.0;
        locals.var_fd2_dn9 = 0.0;
        locals.var_fd2_dn10 = 0.0;
        locals.var_fd2_dn13 = 0.0;
        locals.var_fd2_rv = 0.0;

        locals.var_fmdvds = 0.0;
        locals.var_fmdvds_dn0 = 0.0;
        locals.var_fmdvds_dn2 = 0.0;
        locals.var_fmdvds_dn4 = 0.0;
        locals.var_fmdvds_dn5 = 0.0;
        locals.var_fmdvds_dn6 = 0.0;
        locals.var_fmdvds_dn7 = 0.0;
        locals.var_fmdvds_dn8 = 0.0;
        locals.var_fmdvds_dn9 = 0.0;
        locals.var_fmdvds_dn10 = 0.0;
        locals.var_fmdvds_dn13 = 0.0;
        locals.var_fmdvds_rv = 0.0;

        locals.var_cnst0 = 0.0;
        locals.var_cnst0_dn0 = 0.0;
        locals.var_cnst0_dn2 = 0.0;
        locals.var_cnst0_dn4 = 0.0;
        locals.var_cnst0_dn5 = 0.0;
        locals.var_cnst0_dn6 = 0.0;
        locals.var_cnst0_dn7 = 0.0;
        locals.var_cnst0_dn8 = 0.0;
        locals.var_cnst0_dn9 = 0.0;
        locals.var_cnst0_dn10 = 0.0;
        locals.var_cnst0_dn13 = 0.0;
        locals.var_cnst0_rv = 0.0;

        locals.var_cnst1 = 0.0;
        locals.var_cnst1_dn0 = 0.0;
        locals.var_cnst1_dn2 = 0.0;
        locals.var_cnst1_dn4 = 0.0;
        locals.var_cnst1_dn5 = 0.0;
        locals.var_cnst1_dn6 = 0.0;
        locals.var_cnst1_dn7 = 0.0;
        locals.var_cnst1_dn8 = 0.0;
        locals.var_cnst1_dn9 = 0.0;
        locals.var_cnst1_dn10 = 0.0;
        locals.var_cnst1_dn13 = 0.0;
        locals.var_cnst1_rv = 0.0;

        locals.var_cnstcoxi = 0.0;
        locals.var_cnstcoxi_dn0 = 0.0;
        locals.var_cnstcoxi_dn2 = 0.0;
        locals.var_cnstcoxi_dn4 = 0.0;
        locals.var_cnstcoxi_dn5 = 0.0;
        locals.var_cnstcoxi_dn6 = 0.0;
        locals.var_cnstcoxi_dn7 = 0.0;
        locals.var_cnstcoxi_dn8 = 0.0;
        locals.var_cnstcoxi_dn9 = 0.0;
        locals.var_cnstcoxi_dn10 = 0.0;
        locals.var_cnstcoxi_dn13 = 0.0;
        locals.var_cnstcoxi_rv = 0.0;

        locals.var_fac1 = 0.0;
        locals.var_fac1_dn0 = 0.0;
        locals.var_fac1_dn2 = 0.0;
        locals.var_fac1_dn4 = 0.0;
        locals.var_fac1_dn5 = 0.0;
        locals.var_fac1_dn6 = 0.0;
        locals.var_fac1_dn7 = 0.0;
        locals.var_fac1_dn8 = 0.0;
        locals.var_fac1_dn9 = 0.0;
        locals.var_fac1_dn10 = 0.0;
        locals.var_fac1_dn13 = 0.0;
        locals.var_fac1_rv = 0.0;

        locals.var_fac1p2 = 0.0;
        locals.var_fac1p2_dn0 = 0.0;
        locals.var_fac1p2_dn2 = 0.0;
        locals.var_fac1p2_dn4 = 0.0;
        locals.var_fac1p2_dn5 = 0.0;
        locals.var_fac1p2_dn6 = 0.0;
        locals.var_fac1p2_dn7 = 0.0;
        locals.var_fac1p2_dn8 = 0.0;
        locals.var_fac1p2_dn9 = 0.0;
        locals.var_fac1p2_dn10 = 0.0;
        locals.var_fac1p2_dn13 = 0.0;
        locals.var_fac1p2_rv = 0.0;

        locals.var_fs01 = 0.0;
        locals.var_fs01_dn0 = 0.0;
        locals.var_fs01_dn2 = 0.0;
        locals.var_fs01_dn4 = 0.0;
        locals.var_fs01_dn5 = 0.0;
        locals.var_fs01_dn6 = 0.0;
        locals.var_fs01_dn7 = 0.0;
        locals.var_fs01_dn8 = 0.0;
        locals.var_fs01_dn9 = 0.0;
        locals.var_fs01_dn10 = 0.0;
        locals.var_fs01_dn13 = 0.0;
        locals.var_fs01_rv = 0.0;

        locals.var_fs01_dps0 = 0.0;
        locals.var_fs01_dps0_dn0 = 0.0;
        locals.var_fs01_dps0_dn2 = 0.0;
        locals.var_fs01_dps0_dn4 = 0.0;
        locals.var_fs01_dps0_dn5 = 0.0;
        locals.var_fs01_dps0_dn6 = 0.0;
        locals.var_fs01_dps0_dn7 = 0.0;
        locals.var_fs01_dps0_dn8 = 0.0;
        locals.var_fs01_dps0_dn9 = 0.0;
        locals.var_fs01_dps0_dn10 = 0.0;
        locals.var_fs01_dps0_dn13 = 0.0;
        locals.var_fs01_dps0_rv = 0.0;

        locals.var_fs02 = 0.0;
        locals.var_fs02_dn0 = 0.0;
        locals.var_fs02_dn2 = 0.0;
        locals.var_fs02_dn4 = 0.0;
        locals.var_fs02_dn5 = 0.0;
        locals.var_fs02_dn6 = 0.0;
        locals.var_fs02_dn7 = 0.0;
        locals.var_fs02_dn8 = 0.0;
        locals.var_fs02_dn9 = 0.0;
        locals.var_fs02_dn10 = 0.0;
        locals.var_fs02_dn13 = 0.0;
        locals.var_fs02_rv = 0.0;

        locals.var_fs02_dps0 = 0.0;
        locals.var_fs02_dps0_dn0 = 0.0;
        locals.var_fs02_dps0_dn2 = 0.0;
        locals.var_fs02_dps0_dn4 = 0.0;
        locals.var_fs02_dps0_dn5 = 0.0;
        locals.var_fs02_dps0_dn6 = 0.0;
        locals.var_fs02_dps0_dn7 = 0.0;
        locals.var_fs02_dps0_dn8 = 0.0;
        locals.var_fs02_dps0_dn9 = 0.0;
        locals.var_fs02_dps0_dn10 = 0.0;
        locals.var_fs02_dps0_dn13 = 0.0;
        locals.var_fs02_dps0_rv = 0.0;

        locals.var_fsl1 = 0.0;
        locals.var_fsl1_dn0 = 0.0;
        locals.var_fsl1_dn2 = 0.0;
        locals.var_fsl1_dn4 = 0.0;
        locals.var_fsl1_dn5 = 0.0;
        locals.var_fsl1_dn6 = 0.0;
        locals.var_fsl1_dn7 = 0.0;
        locals.var_fsl1_dn8 = 0.0;
        locals.var_fsl1_dn9 = 0.0;
        locals.var_fsl1_dn10 = 0.0;
        locals.var_fsl1_dn13 = 0.0;
        locals.var_fsl1_rv = 0.0;

        locals.var_fsl1_dpsl = 0.0;
        locals.var_fsl1_dpsl_dn0 = 0.0;
        locals.var_fsl1_dpsl_dn2 = 0.0;
        locals.var_fsl1_dpsl_dn4 = 0.0;
        locals.var_fsl1_dpsl_dn5 = 0.0;
        locals.var_fsl1_dpsl_dn6 = 0.0;
        locals.var_fsl1_dpsl_dn7 = 0.0;
        locals.var_fsl1_dpsl_dn8 = 0.0;
        locals.var_fsl1_dpsl_dn9 = 0.0;
        locals.var_fsl1_dpsl_dn10 = 0.0;
        locals.var_fsl1_dpsl_dn13 = 0.0;
        locals.var_fsl1_dpsl_rv = 0.0;

        locals.var_fsl2 = 0.0;
        locals.var_fsl2_dn0 = 0.0;
        locals.var_fsl2_dn2 = 0.0;
        locals.var_fsl2_dn4 = 0.0;
        locals.var_fsl2_dn5 = 0.0;
        locals.var_fsl2_dn6 = 0.0;
        locals.var_fsl2_dn7 = 0.0;
        locals.var_fsl2_dn8 = 0.0;
        locals.var_fsl2_dn9 = 0.0;
        locals.var_fsl2_dn10 = 0.0;
        locals.var_fsl2_dn13 = 0.0;
        locals.var_fsl2_rv = 0.0;

        locals.var_fsl2_dpsl = 0.0;
        locals.var_fsl2_dpsl_dn0 = 0.0;
        locals.var_fsl2_dpsl_dn2 = 0.0;
        locals.var_fsl2_dpsl_dn4 = 0.0;
        locals.var_fsl2_dpsl_dn5 = 0.0;
        locals.var_fsl2_dpsl_dn6 = 0.0;
        locals.var_fsl2_dpsl_dn7 = 0.0;
        locals.var_fsl2_dpsl_dn8 = 0.0;
        locals.var_fsl2_dpsl_dn9 = 0.0;
        locals.var_fsl2_dpsl_dn10 = 0.0;
        locals.var_fsl2_dpsl_dn13 = 0.0;
        locals.var_fsl2_dpsl_rv = 0.0;

        locals.var_cfs1 = 0.0;
        locals.var_cfs1_dn0 = 0.0;
        locals.var_cfs1_dn2 = 0.0;
        locals.var_cfs1_dn4 = 0.0;
        locals.var_cfs1_dn5 = 0.0;
        locals.var_cfs1_dn6 = 0.0;
        locals.var_cfs1_dn7 = 0.0;
        locals.var_cfs1_dn8 = 0.0;
        locals.var_cfs1_dn9 = 0.0;
        locals.var_cfs1_dn10 = 0.0;
        locals.var_cfs1_dn13 = 0.0;
        locals.var_cfs1_rv = 0.0;

        locals.var_fb = 0.0;
        locals.var_fb_dn0 = 0.0;
        locals.var_fb_dn2 = 0.0;
        locals.var_fb_dn4 = 0.0;
        locals.var_fb_dn5 = 0.0;
        locals.var_fb_dn6 = 0.0;
        locals.var_fb_dn7 = 0.0;
        locals.var_fb_dn8 = 0.0;
        locals.var_fb_dn9 = 0.0;
        locals.var_fb_dn10 = 0.0;
        locals.var_fb_dn13 = 0.0;
        locals.var_fb_rv = 0.0;

        locals.var_fb_dchi = 0.0;
        locals.var_fb_dchi_dn0 = 0.0;
        locals.var_fb_dchi_dn2 = 0.0;
        locals.var_fb_dchi_dn4 = 0.0;
        locals.var_fb_dchi_dn5 = 0.0;
        locals.var_fb_dchi_dn6 = 0.0;
        locals.var_fb_dchi_dn7 = 0.0;
        locals.var_fb_dchi_dn8 = 0.0;
        locals.var_fb_dchi_dn9 = 0.0;
        locals.var_fb_dchi_dn10 = 0.0;
        locals.var_fb_dchi_dn13 = 0.0;
        locals.var_fb_dchi_rv = 0.0;

        locals.var_fi = 0.0;
        locals.var_fi_dn0 = 0.0;
        locals.var_fi_dn2 = 0.0;
        locals.var_fi_dn4 = 0.0;
        locals.var_fi_dn5 = 0.0;
        locals.var_fi_dn6 = 0.0;
        locals.var_fi_dn7 = 0.0;
        locals.var_fi_dn8 = 0.0;
        locals.var_fi_dn9 = 0.0;
        locals.var_fi_dn10 = 0.0;
        locals.var_fi_dn13 = 0.0;
        locals.var_fi_rv = 0.0;

        locals.var_fi_dchi = 0.0;
        locals.var_fi_dchi_dn0 = 0.0;
        locals.var_fi_dchi_dn2 = 0.0;
        locals.var_fi_dchi_dn4 = 0.0;
        locals.var_fi_dchi_dn5 = 0.0;
        locals.var_fi_dchi_dn6 = 0.0;
        locals.var_fi_dchi_dn7 = 0.0;
        locals.var_fi_dchi_dn8 = 0.0;
        locals.var_fi_dchi_dn9 = 0.0;
        locals.var_fi_dchi_dn10 = 0.0;
        locals.var_fi_dchi_dn13 = 0.0;
        locals.var_fi_dchi_rv = 0.0;

        locals.var_exp_chi = 0.0;
        locals.var_exp_chi_dn0 = 0.0;
        locals.var_exp_chi_dn2 = 0.0;
        locals.var_exp_chi_dn4 = 0.0;
        locals.var_exp_chi_dn5 = 0.0;
        locals.var_exp_chi_dn6 = 0.0;
        locals.var_exp_chi_dn7 = 0.0;
        locals.var_exp_chi_dn8 = 0.0;
        locals.var_exp_chi_dn9 = 0.0;
        locals.var_exp_chi_dn10 = 0.0;
        locals.var_exp_chi_dn13 = 0.0;
        locals.var_exp_chi_rv = 0.0;

        locals.var_exp_rho = 0.0;
        locals.var_exp_rho_dn0 = 0.0;
        locals.var_exp_rho_dn2 = 0.0;
        locals.var_exp_rho_dn4 = 0.0;
        locals.var_exp_rho_dn5 = 0.0;
        locals.var_exp_rho_dn6 = 0.0;
        locals.var_exp_rho_dn7 = 0.0;
        locals.var_exp_rho_dn8 = 0.0;
        locals.var_exp_rho_dn9 = 0.0;
        locals.var_exp_rho_dn10 = 0.0;
        locals.var_exp_rho_dn13 = 0.0;
        locals.var_exp_rho_rv = 0.0;

        locals.var_exp_bvbs = 0.0;
        locals.var_exp_bvbs_dn0 = 0.0;
        locals.var_exp_bvbs_dn2 = 0.0;
        locals.var_exp_bvbs_dn4 = 0.0;
        locals.var_exp_bvbs_dn5 = 0.0;
        locals.var_exp_bvbs_dn6 = 0.0;
        locals.var_exp_bvbs_dn7 = 0.0;
        locals.var_exp_bvbs_dn8 = 0.0;
        locals.var_exp_bvbs_dn9 = 0.0;
        locals.var_exp_bvbs_dn10 = 0.0;
        locals.var_exp_bvbs_dn13 = 0.0;
        locals.var_exp_bvbs_rv = 0.0;

        locals.var_exp_bvbsvds = 0.0;
        locals.var_exp_bvbsvds_dn0 = 0.0;
        locals.var_exp_bvbsvds_dn2 = 0.0;
        locals.var_exp_bvbsvds_dn4 = 0.0;
        locals.var_exp_bvbsvds_dn5 = 0.0;
        locals.var_exp_bvbsvds_dn6 = 0.0;
        locals.var_exp_bvbsvds_dn7 = 0.0;
        locals.var_exp_bvbsvds_dn8 = 0.0;
        locals.var_exp_bvbsvds_dn9 = 0.0;
        locals.var_exp_bvbsvds_dn10 = 0.0;
        locals.var_exp_bvbsvds_dn13 = 0.0;
        locals.var_exp_bvbsvds_rv = 0.0;

        locals.var_exp_bps0 = 0.0;
        locals.var_exp_bps0_dn0 = 0.0;
        locals.var_exp_bps0_dn2 = 0.0;
        locals.var_exp_bps0_dn4 = 0.0;
        locals.var_exp_bps0_dn5 = 0.0;
        locals.var_exp_bps0_dn6 = 0.0;
        locals.var_exp_bps0_dn7 = 0.0;
        locals.var_exp_bps0_dn8 = 0.0;
        locals.var_exp_bps0_dn9 = 0.0;
        locals.var_exp_bps0_dn10 = 0.0;
        locals.var_exp_bps0_dn13 = 0.0;
        locals.var_exp_bps0_rv = 0.0;

        locals.var_fs0 = 0.0;
        locals.var_fs0_dn0 = 0.0;
        locals.var_fs0_dn2 = 0.0;
        locals.var_fs0_dn4 = 0.0;
        locals.var_fs0_dn5 = 0.0;
        locals.var_fs0_dn6 = 0.0;
        locals.var_fs0_dn7 = 0.0;
        locals.var_fs0_dn8 = 0.0;
        locals.var_fs0_dn9 = 0.0;
        locals.var_fs0_dn10 = 0.0;
        locals.var_fs0_dn13 = 0.0;
        locals.var_fs0_rv = 0.0;

        locals.var_fs0_dps0 = 0.0;
        locals.var_fs0_dps0_dn0 = 0.0;
        locals.var_fs0_dps0_dn2 = 0.0;
        locals.var_fs0_dps0_dn4 = 0.0;
        locals.var_fs0_dps0_dn5 = 0.0;
        locals.var_fs0_dps0_dn6 = 0.0;
        locals.var_fs0_dps0_dn7 = 0.0;
        locals.var_fs0_dps0_dn8 = 0.0;
        locals.var_fs0_dps0_dn9 = 0.0;
        locals.var_fs0_dps0_dn10 = 0.0;
        locals.var_fs0_dps0_dn13 = 0.0;
        locals.var_fs0_dps0_rv = 0.0;

        locals.var_fsl = 0.0;
        locals.var_fsl_dn0 = 0.0;
        locals.var_fsl_dn2 = 0.0;
        locals.var_fsl_dn4 = 0.0;
        locals.var_fsl_dn5 = 0.0;
        locals.var_fsl_dn6 = 0.0;
        locals.var_fsl_dn7 = 0.0;
        locals.var_fsl_dn8 = 0.0;
        locals.var_fsl_dn9 = 0.0;
        locals.var_fsl_dn10 = 0.0;
        locals.var_fsl_dn13 = 0.0;
        locals.var_fsl_rv = 0.0;

        locals.var_fsl_dpsl = 0.0;
        locals.var_fsl_dpsl_dn0 = 0.0;
        locals.var_fsl_dpsl_dn2 = 0.0;
        locals.var_fsl_dpsl_dn4 = 0.0;
        locals.var_fsl_dpsl_dn5 = 0.0;
        locals.var_fsl_dpsl_dn6 = 0.0;
        locals.var_fsl_dpsl_dn7 = 0.0;
        locals.var_fsl_dpsl_dn8 = 0.0;
        locals.var_fsl_dpsl_dn9 = 0.0;
        locals.var_fsl_dpsl_dn10 = 0.0;
        locals.var_fsl_dpsl_dn13 = 0.0;
        locals.var_fsl_dpsl_rv = 0.0;

        locals.var_dps0 = 0.0;
        locals.var_dps0_dn0 = 0.0;
        locals.var_dps0_dn2 = 0.0;
        locals.var_dps0_dn4 = 0.0;
        locals.var_dps0_dn5 = 0.0;
        locals.var_dps0_dn6 = 0.0;
        locals.var_dps0_dn7 = 0.0;
        locals.var_dps0_dn8 = 0.0;
        locals.var_dps0_dn9 = 0.0;
        locals.var_dps0_dn10 = 0.0;
        locals.var_dps0_dn13 = 0.0;
        locals.var_dps0_rv = 0.0;

        locals.var_dpsl = 0.0;
        locals.var_dpsl_dn0 = 0.0;
        locals.var_dpsl_dn2 = 0.0;
        locals.var_dpsl_dn4 = 0.0;
        locals.var_dpsl_dn5 = 0.0;
        locals.var_dpsl_dn6 = 0.0;
        locals.var_dpsl_dn7 = 0.0;
        locals.var_dpsl_dn8 = 0.0;
        locals.var_dpsl_dn9 = 0.0;
        locals.var_dpsl_dn10 = 0.0;
        locals.var_dpsl_dn13 = 0.0;
        locals.var_dpsl_rv = 0.0;

        locals.var_qn0 = 0.0;
        locals.var_qn0_dn0 = 0.0;
        locals.var_qn0_dn2 = 0.0;
        locals.var_qn0_dn4 = 0.0;
        locals.var_qn0_dn5 = 0.0;
        locals.var_qn0_dn6 = 0.0;
        locals.var_qn0_dn7 = 0.0;
        locals.var_qn0_dn8 = 0.0;
        locals.var_qn0_dn9 = 0.0;
        locals.var_qn0_dn10 = 0.0;
        locals.var_qn0_dn13 = 0.0;
        locals.var_qn0_rv = 0.0;

        locals.var_qb0 = 0.0;
        locals.var_qb0_dn0 = 0.0;
        locals.var_qb0_dn2 = 0.0;
        locals.var_qb0_dn4 = 0.0;
        locals.var_qb0_dn5 = 0.0;
        locals.var_qb0_dn6 = 0.0;
        locals.var_qb0_dn7 = 0.0;
        locals.var_qb0_dn8 = 0.0;
        locals.var_qb0_dn9 = 0.0;
        locals.var_qb0_dn10 = 0.0;
        locals.var_qb0_dn13 = 0.0;
        locals.var_qb0_rv = 0.0;

        locals.var_qbnm = 0.0;
        locals.var_qbnm_dn0 = 0.0;
        locals.var_qbnm_dn2 = 0.0;
        locals.var_qbnm_dn4 = 0.0;
        locals.var_qbnm_dn5 = 0.0;
        locals.var_qbnm_dn6 = 0.0;
        locals.var_qbnm_dn7 = 0.0;
        locals.var_qbnm_dn8 = 0.0;
        locals.var_qbnm_dn9 = 0.0;
        locals.var_qbnm_dn10 = 0.0;
        locals.var_qbnm_dn13 = 0.0;
        locals.var_qbnm_rv = 0.0;

        locals.var_dtpds = 0.0;
        locals.var_dtpds_dn0 = 0.0;
        locals.var_dtpds_dn2 = 0.0;
        locals.var_dtpds_dn4 = 0.0;
        locals.var_dtpds_dn5 = 0.0;
        locals.var_dtpds_dn6 = 0.0;
        locals.var_dtpds_dn7 = 0.0;
        locals.var_dtpds_dn8 = 0.0;
        locals.var_dtpds_dn9 = 0.0;
        locals.var_dtpds_dn10 = 0.0;
        locals.var_dtpds_dn13 = 0.0;
        locals.var_dtpds_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_4(
        locals: &mut StampLocals,
    ) {
        locals.var_qinm = 0.0;
        locals.var_qinm_dn0 = 0.0;
        locals.var_qinm_dn2 = 0.0;
        locals.var_qinm_dn4 = 0.0;
        locals.var_qinm_dn5 = 0.0;
        locals.var_qinm_dn6 = 0.0;
        locals.var_qinm_dn7 = 0.0;
        locals.var_qinm_dn8 = 0.0;
        locals.var_qinm_dn9 = 0.0;
        locals.var_qinm_dn10 = 0.0;
        locals.var_qinm_dn13 = 0.0;
        locals.var_qinm_rv = 0.0;

        locals.var_qidn = 0.0;
        locals.var_qidn_dn0 = 0.0;
        locals.var_qidn_dn2 = 0.0;
        locals.var_qidn_dn4 = 0.0;
        locals.var_qidn_dn5 = 0.0;
        locals.var_qidn_dn6 = 0.0;
        locals.var_qidn_dn7 = 0.0;
        locals.var_qidn_dn8 = 0.0;
        locals.var_qidn_dn9 = 0.0;
        locals.var_qidn_dn10 = 0.0;
        locals.var_qidn_dn13 = 0.0;
        locals.var_qidn_rv = 0.0;

        locals.var_qdnm = 0.0;
        locals.var_qdnm_dn0 = 0.0;
        locals.var_qdnm_dn2 = 0.0;
        locals.var_qdnm_dn4 = 0.0;
        locals.var_qdnm_dn5 = 0.0;
        locals.var_qdnm_dn6 = 0.0;
        locals.var_qdnm_dn7 = 0.0;
        locals.var_qdnm_dn8 = 0.0;
        locals.var_qdnm_dn9 = 0.0;
        locals.var_qdnm_dn10 = 0.0;
        locals.var_qdnm_dn13 = 0.0;
        locals.var_qdnm_rv = 0.0;

        locals.var_qddn = 0.0;
        locals.var_qddn_dn0 = 0.0;
        locals.var_qddn_dn2 = 0.0;
        locals.var_qddn_dn4 = 0.0;
        locals.var_qddn_dn5 = 0.0;
        locals.var_qddn_dn6 = 0.0;
        locals.var_qddn_dn7 = 0.0;
        locals.var_qddn_dn8 = 0.0;
        locals.var_qddn_dn9 = 0.0;
        locals.var_qddn_dn10 = 0.0;
        locals.var_qddn_dn13 = 0.0;
        locals.var_qddn_rv = 0.0;

        locals.var_quot = 0.0;
        locals.var_quot_dn0 = 0.0;
        locals.var_quot_dn2 = 0.0;
        locals.var_quot_dn4 = 0.0;
        locals.var_quot_dn5 = 0.0;
        locals.var_quot_dn6 = 0.0;
        locals.var_quot_dn7 = 0.0;
        locals.var_quot_dn8 = 0.0;
        locals.var_quot_dn9 = 0.0;
        locals.var_quot_dn10 = 0.0;
        locals.var_quot_dn13 = 0.0;
        locals.var_quot_rv = 0.0;

        locals.var_qdrat = 0.5;
        locals.var_qdrat_dn0 = 0.0;
        locals.var_qdrat_dn2 = 0.0;
        locals.var_qdrat_dn4 = 0.0;
        locals.var_qdrat_dn5 = 0.0;
        locals.var_qdrat_dn6 = 0.0;
        locals.var_qdrat_dn7 = 0.0;
        locals.var_qdrat_dn8 = 0.0;
        locals.var_qdrat_dn9 = 0.0;
        locals.var_qdrat_dn10 = 0.0;
        locals.var_qdrat_dn13 = 0.0;
        locals.var_qdrat_rv = 0.0;

        locals.var_idd = 0.0;
        locals.var_idd_dn0 = 0.0;
        locals.var_idd_dn2 = 0.0;
        locals.var_idd_dn4 = 0.0;
        locals.var_idd_dn5 = 0.0;
        locals.var_idd_dn6 = 0.0;
        locals.var_idd_dn7 = 0.0;
        locals.var_idd_dn8 = 0.0;
        locals.var_idd_dn9 = 0.0;
        locals.var_idd_dn10 = 0.0;
        locals.var_idd_dn13 = 0.0;
        locals.var_idd_rv = 0.0;

        locals.var_idd1 = 0.0;
        locals.var_idd1_dn0 = 0.0;
        locals.var_idd1_dn2 = 0.0;
        locals.var_idd1_dn4 = 0.0;
        locals.var_idd1_dn5 = 0.0;
        locals.var_idd1_dn6 = 0.0;
        locals.var_idd1_dn7 = 0.0;
        locals.var_idd1_dn8 = 0.0;
        locals.var_idd1_dn9 = 0.0;
        locals.var_idd1_dn10 = 0.0;
        locals.var_idd1_dn13 = 0.0;
        locals.var_idd1_rv = 0.0;

        locals.var_fdd = 0.0;
        locals.var_fdd_dn0 = 0.0;
        locals.var_fdd_dn2 = 0.0;
        locals.var_fdd_dn4 = 0.0;
        locals.var_fdd_dn5 = 0.0;
        locals.var_fdd_dn6 = 0.0;
        locals.var_fdd_dn7 = 0.0;
        locals.var_fdd_dn8 = 0.0;
        locals.var_fdd_dn9 = 0.0;
        locals.var_fdd_dn10 = 0.0;
        locals.var_fdd_dn13 = 0.0;
        locals.var_fdd_rv = 0.0;

        locals.var_eeff = 0.0;
        locals.var_eeff_dn0 = 0.0;
        locals.var_eeff_dn2 = 0.0;
        locals.var_eeff_dn4 = 0.0;
        locals.var_eeff_dn5 = 0.0;
        locals.var_eeff_dn6 = 0.0;
        locals.var_eeff_dn7 = 0.0;
        locals.var_eeff_dn8 = 0.0;
        locals.var_eeff_dn9 = 0.0;
        locals.var_eeff_dn10 = 0.0;
        locals.var_eeff_dn13 = 0.0;
        locals.var_eeff_rv = 0.0;

        locals.var_rns = 0.0;
        locals.var_rns_dn0 = 0.0;
        locals.var_rns_dn2 = 0.0;
        locals.var_rns_dn4 = 0.0;
        locals.var_rns_dn5 = 0.0;
        locals.var_rns_dn6 = 0.0;
        locals.var_rns_dn7 = 0.0;
        locals.var_rns_dn8 = 0.0;
        locals.var_rns_dn9 = 0.0;
        locals.var_rns_dn10 = 0.0;
        locals.var_rns_dn13 = 0.0;
        locals.var_rns_rv = 0.0;

        locals.var_mu = 0.0;
        locals.var_mu_dn0 = 0.0;
        locals.var_mu_dn2 = 0.0;
        locals.var_mu_dn4 = 0.0;
        locals.var_mu_dn5 = 0.0;
        locals.var_mu_dn6 = 0.0;
        locals.var_mu_dn7 = 0.0;
        locals.var_mu_dn8 = 0.0;
        locals.var_mu_dn9 = 0.0;
        locals.var_mu_dn10 = 0.0;
        locals.var_mu_dn13 = 0.0;
        locals.var_mu_rv = 0.0;

        locals.var_muun = 0.0;
        locals.var_muun_dn0 = 0.0;
        locals.var_muun_dn2 = 0.0;
        locals.var_muun_dn4 = 0.0;
        locals.var_muun_dn5 = 0.0;
        locals.var_muun_dn6 = 0.0;
        locals.var_muun_dn7 = 0.0;
        locals.var_muun_dn8 = 0.0;
        locals.var_muun_dn9 = 0.0;
        locals.var_muun_dn10 = 0.0;
        locals.var_muun_dn13 = 0.0;
        locals.var_muun_rv = 0.0;

        locals.var_ey = 0.0;
        locals.var_ey_dn0 = 0.0;
        locals.var_ey_dn2 = 0.0;
        locals.var_ey_dn4 = 0.0;
        locals.var_ey_dn5 = 0.0;
        locals.var_ey_dn6 = 0.0;
        locals.var_ey_dn7 = 0.0;
        locals.var_ey_dn8 = 0.0;
        locals.var_ey_dn9 = 0.0;
        locals.var_ey_dn10 = 0.0;
        locals.var_ey_dn13 = 0.0;
        locals.var_ey_rv = 0.0;

        locals.var_em = 0.0;
        locals.var_em_dn0 = 0.0;
        locals.var_em_dn2 = 0.0;
        locals.var_em_dn4 = 0.0;
        locals.var_em_dn5 = 0.0;
        locals.var_em_dn6 = 0.0;
        locals.var_em_dn7 = 0.0;
        locals.var_em_dn8 = 0.0;
        locals.var_em_dn9 = 0.0;
        locals.var_em_dn10 = 0.0;
        locals.var_em_dn13 = 0.0;
        locals.var_em_rv = 0.0;

        locals.var_eta = 0.0;
        locals.var_eta_dn0 = 0.0;
        locals.var_eta_dn2 = 0.0;
        locals.var_eta_dn4 = 0.0;
        locals.var_eta_dn5 = 0.0;
        locals.var_eta_dn6 = 0.0;
        locals.var_eta_dn7 = 0.0;
        locals.var_eta_dn8 = 0.0;
        locals.var_eta_dn9 = 0.0;
        locals.var_eta_dn10 = 0.0;
        locals.var_eta_dn13 = 0.0;
        locals.var_eta_rv = 0.0;

        locals.var_eta1 = 0.0;
        locals.var_eta1_dn0 = 0.0;
        locals.var_eta1_dn2 = 0.0;
        locals.var_eta1_dn4 = 0.0;
        locals.var_eta1_dn5 = 0.0;
        locals.var_eta1_dn6 = 0.0;
        locals.var_eta1_dn7 = 0.0;
        locals.var_eta1_dn8 = 0.0;
        locals.var_eta1_dn9 = 0.0;
        locals.var_eta1_dn10 = 0.0;
        locals.var_eta1_dn13 = 0.0;
        locals.var_eta1_rv = 0.0;

        locals.var_eta1p12 = 0.0;
        locals.var_eta1p12_dn0 = 0.0;
        locals.var_eta1p12_dn2 = 0.0;
        locals.var_eta1p12_dn4 = 0.0;
        locals.var_eta1p12_dn5 = 0.0;
        locals.var_eta1p12_dn6 = 0.0;
        locals.var_eta1p12_dn7 = 0.0;
        locals.var_eta1p12_dn8 = 0.0;
        locals.var_eta1p12_dn9 = 0.0;
        locals.var_eta1p12_dn10 = 0.0;
        locals.var_eta1p12_dn13 = 0.0;
        locals.var_eta1p12_rv = 0.0;

        locals.var_eta1p32 = 0.0;
        locals.var_eta1p32_dn0 = 0.0;
        locals.var_eta1p32_dn2 = 0.0;
        locals.var_eta1p32_dn4 = 0.0;
        locals.var_eta1p32_dn5 = 0.0;
        locals.var_eta1p32_dn6 = 0.0;
        locals.var_eta1p32_dn7 = 0.0;
        locals.var_eta1p32_dn8 = 0.0;
        locals.var_eta1p32_dn9 = 0.0;
        locals.var_eta1p32_dn10 = 0.0;
        locals.var_eta1p32_dn13 = 0.0;
        locals.var_eta1p32_rv = 0.0;

        locals.var_eta1p52 = 0.0;
        locals.var_eta1p52_dn0 = 0.0;
        locals.var_eta1p52_dn2 = 0.0;
        locals.var_eta1p52_dn4 = 0.0;
        locals.var_eta1p52_dn5 = 0.0;
        locals.var_eta1p52_dn6 = 0.0;
        locals.var_eta1p52_dn7 = 0.0;
        locals.var_eta1p52_dn8 = 0.0;
        locals.var_eta1p52_dn9 = 0.0;
        locals.var_eta1p52_dn10 = 0.0;
        locals.var_eta1p52_dn13 = 0.0;
        locals.var_eta1p52_rv = 0.0;

        locals.var_zeta12 = 0.0;
        locals.var_zeta12_dn0 = 0.0;
        locals.var_zeta12_dn2 = 0.0;
        locals.var_zeta12_dn4 = 0.0;
        locals.var_zeta12_dn5 = 0.0;
        locals.var_zeta12_dn6 = 0.0;
        locals.var_zeta12_dn7 = 0.0;
        locals.var_zeta12_dn8 = 0.0;
        locals.var_zeta12_dn9 = 0.0;
        locals.var_zeta12_dn10 = 0.0;
        locals.var_zeta12_dn13 = 0.0;
        locals.var_zeta12_rv = 0.0;

        locals.var_zeta32 = 0.0;
        locals.var_zeta32_dn0 = 0.0;
        locals.var_zeta32_dn2 = 0.0;
        locals.var_zeta32_dn4 = 0.0;
        locals.var_zeta32_dn5 = 0.0;
        locals.var_zeta32_dn6 = 0.0;
        locals.var_zeta32_dn7 = 0.0;
        locals.var_zeta32_dn8 = 0.0;
        locals.var_zeta32_dn9 = 0.0;
        locals.var_zeta32_dn10 = 0.0;
        locals.var_zeta32_dn13 = 0.0;
        locals.var_zeta32_rv = 0.0;

        locals.var_zeta52 = 0.0;
        locals.var_zeta52_dn0 = 0.0;
        locals.var_zeta52_dn2 = 0.0;
        locals.var_zeta52_dn4 = 0.0;
        locals.var_zeta52_dn5 = 0.0;
        locals.var_zeta52_dn6 = 0.0;
        locals.var_zeta52_dn7 = 0.0;
        locals.var_zeta52_dn8 = 0.0;
        locals.var_zeta52_dn9 = 0.0;
        locals.var_zeta52_dn10 = 0.0;
        locals.var_zeta52_dn13 = 0.0;
        locals.var_zeta52_rv = 0.0;

        locals.var_f00 = 0.0;
        locals.var_f00_dn0 = 0.0;
        locals.var_f00_dn2 = 0.0;
        locals.var_f00_dn4 = 0.0;
        locals.var_f00_dn5 = 0.0;
        locals.var_f00_dn6 = 0.0;
        locals.var_f00_dn7 = 0.0;
        locals.var_f00_dn8 = 0.0;
        locals.var_f00_dn9 = 0.0;
        locals.var_f00_dn10 = 0.0;
        locals.var_f00_dn13 = 0.0;
        locals.var_f00_rv = 0.0;

        locals.var_f10 = 0.0;
        locals.var_f10_dn0 = 0.0;
        locals.var_f10_dn2 = 0.0;
        locals.var_f10_dn4 = 0.0;
        locals.var_f10_dn5 = 0.0;
        locals.var_f10_dn6 = 0.0;
        locals.var_f10_dn7 = 0.0;
        locals.var_f10_dn8 = 0.0;
        locals.var_f10_dn9 = 0.0;
        locals.var_f10_dn10 = 0.0;
        locals.var_f10_dn13 = 0.0;
        locals.var_f10_rv = 0.0;

        locals.var_f30 = 0.0;
        locals.var_f30_dn0 = 0.0;
        locals.var_f30_dn2 = 0.0;
        locals.var_f30_dn4 = 0.0;
        locals.var_f30_dn5 = 0.0;
        locals.var_f30_dn6 = 0.0;
        locals.var_f30_dn7 = 0.0;
        locals.var_f30_dn8 = 0.0;
        locals.var_f30_dn9 = 0.0;
        locals.var_f30_dn10 = 0.0;
        locals.var_f30_dn13 = 0.0;
        locals.var_f30_rv = 0.0;

        locals.var_f11 = 0.0;
        locals.var_f11_dn0 = 0.0;
        locals.var_f11_dn2 = 0.0;
        locals.var_f11_dn4 = 0.0;
        locals.var_f11_dn5 = 0.0;
        locals.var_f11_dn6 = 0.0;
        locals.var_f11_dn7 = 0.0;
        locals.var_f11_dn8 = 0.0;
        locals.var_f11_dn9 = 0.0;
        locals.var_f11_dn10 = 0.0;
        locals.var_f11_dn13 = 0.0;
        locals.var_f11_rv = 0.0;

        locals.var_vgs_min = 0.0;
        locals.var_vgs_min_rv = 0.0;

        locals.var_ps0_min = 0.0;
        locals.var_ps0_min_dn0 = 0.0;
        locals.var_ps0_min_dn2 = 0.0;
        locals.var_ps0_min_dn4 = 0.0;
        locals.var_ps0_min_dn5 = 0.0;
        locals.var_ps0_min_dn6 = 0.0;
        locals.var_ps0_min_dn7 = 0.0;
        locals.var_ps0_min_dn8 = 0.0;
        locals.var_ps0_min_dn9 = 0.0;
        locals.var_ps0_min_dn10 = 0.0;
        locals.var_ps0_min_dn13 = 0.0;
        locals.var_ps0_min_rv = 0.0;

        locals.var_acn = 0.0;
        locals.var_acn_dn0 = 0.0;
        locals.var_acn_dn2 = 0.0;
        locals.var_acn_dn4 = 0.0;
        locals.var_acn_dn5 = 0.0;
        locals.var_acn_dn6 = 0.0;
        locals.var_acn_dn7 = 0.0;
        locals.var_acn_dn8 = 0.0;
        locals.var_acn_dn9 = 0.0;
        locals.var_acn_dn10 = 0.0;
        locals.var_acn_dn13 = 0.0;
        locals.var_acn_rv = 0.0;

        locals.var_acd = 0.0;
        locals.var_acd_dn0 = 0.0;
        locals.var_acd_dn2 = 0.0;
        locals.var_acd_dn4 = 0.0;
        locals.var_acd_dn5 = 0.0;
        locals.var_acd_dn6 = 0.0;
        locals.var_acd_dn7 = 0.0;
        locals.var_acd_dn8 = 0.0;
        locals.var_acd_dn9 = 0.0;
        locals.var_acd_dn10 = 0.0;
        locals.var_acd_dn13 = 0.0;
        locals.var_acd_rv = 0.0;

        locals.var_ac1 = 0.0;
        locals.var_ac1_dn0 = 0.0;
        locals.var_ac1_dn2 = 0.0;
        locals.var_ac1_dn4 = 0.0;
        locals.var_ac1_dn5 = 0.0;
        locals.var_ac1_dn6 = 0.0;
        locals.var_ac1_dn7 = 0.0;
        locals.var_ac1_dn8 = 0.0;
        locals.var_ac1_dn9 = 0.0;
        locals.var_ac1_dn10 = 0.0;
        locals.var_ac1_dn13 = 0.0;
        locals.var_ac1_rv = 0.0;

        locals.var_ac2 = 0.0;
        locals.var_ac2_dn0 = 0.0;
        locals.var_ac2_dn2 = 0.0;
        locals.var_ac2_dn4 = 0.0;
        locals.var_ac2_dn5 = 0.0;
        locals.var_ac2_dn6 = 0.0;
        locals.var_ac2_dn7 = 0.0;
        locals.var_ac2_dn8 = 0.0;
        locals.var_ac2_dn9 = 0.0;
        locals.var_ac2_dn10 = 0.0;
        locals.var_ac2_dn13 = 0.0;
        locals.var_ac2_rv = 0.0;

        locals.var_ac3 = 0.0;
        locals.var_ac3_dn0 = 0.0;
        locals.var_ac3_dn2 = 0.0;
        locals.var_ac3_dn4 = 0.0;
        locals.var_ac3_dn5 = 0.0;
        locals.var_ac3_dn6 = 0.0;
        locals.var_ac3_dn7 = 0.0;
        locals.var_ac3_dn8 = 0.0;
        locals.var_ac3_dn9 = 0.0;
        locals.var_ac3_dn10 = 0.0;
        locals.var_ac3_dn13 = 0.0;
        locals.var_ac3_rv = 0.0;

        locals.var_ac4 = 0.0;
        locals.var_ac4_dn0 = 0.0;
        locals.var_ac4_dn2 = 0.0;
        locals.var_ac4_dn4 = 0.0;
        locals.var_ac4_dn5 = 0.0;
        locals.var_ac4_dn6 = 0.0;
        locals.var_ac4_dn7 = 0.0;
        locals.var_ac4_dn8 = 0.0;
        locals.var_ac4_dn9 = 0.0;
        locals.var_ac4_dn10 = 0.0;
        locals.var_ac4_dn13 = 0.0;
        locals.var_ac4_rv = 0.0;

        locals.var_ac31 = 0.0;
        locals.var_ac31_dn0 = 0.0;
        locals.var_ac31_dn2 = 0.0;
        locals.var_ac31_dn4 = 0.0;
        locals.var_ac31_dn5 = 0.0;
        locals.var_ac31_dn6 = 0.0;
        locals.var_ac31_dn7 = 0.0;
        locals.var_ac31_dn8 = 0.0;
        locals.var_ac31_dn9 = 0.0;
        locals.var_ac31_dn10 = 0.0;
        locals.var_ac31_dn13 = 0.0;
        locals.var_ac31_rv = 0.0;

        locals.var_ac41 = 0.0;
        locals.var_ac41_dn0 = 0.0;
        locals.var_ac41_dn2 = 0.0;
        locals.var_ac41_dn4 = 0.0;
        locals.var_ac41_dn5 = 0.0;
        locals.var_ac41_dn6 = 0.0;
        locals.var_ac41_dn7 = 0.0;
        locals.var_ac41_dn8 = 0.0;
        locals.var_ac41_dn9 = 0.0;
        locals.var_ac41_dn10 = 0.0;
        locals.var_ac41_dn13 = 0.0;
        locals.var_ac41_rv = 0.0;

        locals.var_isub = 0.0;
        locals.var_isub_dn0 = 0.0;
        locals.var_isub_dn2 = 0.0;
        locals.var_isub_dn4 = 0.0;
        locals.var_isub_dn5 = 0.0;
        locals.var_isub_dn6 = 0.0;
        locals.var_isub_dn7 = 0.0;
        locals.var_isub_dn8 = 0.0;
        locals.var_isub_dn9 = 0.0;
        locals.var_isub_dn10 = 0.0;
        locals.var_isub_dn13 = 0.0;
        locals.var_isub_rv = 0.0;

        locals.var_isubld = 0.0;
        locals.var_isubld_dn0 = 0.0;
        locals.var_isubld_dn2 = 0.0;
        locals.var_isubld_dn4 = 0.0;
        locals.var_isubld_dn5 = 0.0;
        locals.var_isubld_dn6 = 0.0;
        locals.var_isubld_dn7 = 0.0;
        locals.var_isubld_dn8 = 0.0;
        locals.var_isubld_dn9 = 0.0;
        locals.var_isubld_dn10 = 0.0;
        locals.var_isubld_dn13 = 0.0;
        locals.var_isubld_rv = 0.0;

        locals.var_psislsat = 0.0;
        locals.var_psislsat_dn0 = 0.0;
        locals.var_psislsat_dn2 = 0.0;
        locals.var_psislsat_dn4 = 0.0;
        locals.var_psislsat_dn5 = 0.0;
        locals.var_psislsat_dn6 = 0.0;
        locals.var_psislsat_dn7 = 0.0;
        locals.var_psislsat_dn8 = 0.0;
        locals.var_psislsat_dn9 = 0.0;
        locals.var_psislsat_dn10 = 0.0;
        locals.var_psislsat_dn13 = 0.0;
        locals.var_psislsat_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_5(
        locals: &mut StampLocals,
    ) {
        locals.var_psisubsat = 0.0;
        locals.var_psisubsat_dn0 = 0.0;
        locals.var_psisubsat_dn2 = 0.0;
        locals.var_psisubsat_dn4 = 0.0;
        locals.var_psisubsat_dn5 = 0.0;
        locals.var_psisubsat_dn6 = 0.0;
        locals.var_psisubsat_dn7 = 0.0;
        locals.var_psisubsat_dn8 = 0.0;
        locals.var_psisubsat_dn9 = 0.0;
        locals.var_psisubsat_dn10 = 0.0;
        locals.var_psisubsat_dn13 = 0.0;
        locals.var_psisubsat_rv = 0.0;

        locals.var_eg12 = 0.0;
        locals.var_eg12_dn0 = 0.0;
        locals.var_eg12_dn2 = 0.0;
        locals.var_eg12_dn4 = 0.0;
        locals.var_eg12_dn5 = 0.0;
        locals.var_eg12_dn6 = 0.0;
        locals.var_eg12_dn7 = 0.0;
        locals.var_eg12_dn8 = 0.0;
        locals.var_eg12_dn9 = 0.0;
        locals.var_eg12_dn10 = 0.0;
        locals.var_eg12_dn13 = 0.0;
        locals.var_eg12_rv = 0.0;

        locals.var_eg32 = 0.0;
        locals.var_eg32_dn0 = 0.0;
        locals.var_eg32_dn2 = 0.0;
        locals.var_eg32_dn4 = 0.0;
        locals.var_eg32_dn5 = 0.0;
        locals.var_eg32_dn6 = 0.0;
        locals.var_eg32_dn7 = 0.0;
        locals.var_eg32_dn8 = 0.0;
        locals.var_eg32_dn9 = 0.0;
        locals.var_eg32_dn10 = 0.0;
        locals.var_eg32_dn13 = 0.0;
        locals.var_eg32_rv = 0.0;

        locals.var_cov_slp = 0.0;
        locals.var_cov_slp_rv = 0.0;

        locals.var_cov_mag = 0.0;
        locals.var_cov_mag_rv = 0.0;

        locals.var_qgos = 0.0;
        locals.var_qgos_dn0 = 0.0;
        locals.var_qgos_dn2 = 0.0;
        locals.var_qgos_dn4 = 0.0;
        locals.var_qgos_dn5 = 0.0;
        locals.var_qgos_dn6 = 0.0;
        locals.var_qgos_dn7 = 0.0;
        locals.var_qgos_dn8 = 0.0;
        locals.var_qgos_dn9 = 0.0;
        locals.var_qgos_dn10 = 0.0;
        locals.var_qgos_dn13 = 0.0;
        locals.var_qgos_rv = 0.0;

        locals.var_qgod = 0.0;
        locals.var_qgod_dn0 = 0.0;
        locals.var_qgod_dn2 = 0.0;
        locals.var_qgod_dn4 = 0.0;
        locals.var_qgod_dn5 = 0.0;
        locals.var_qgod_dn6 = 0.0;
        locals.var_qgod_dn7 = 0.0;
        locals.var_qgod_dn8 = 0.0;
        locals.var_qgod_dn9 = 0.0;
        locals.var_qgod_dn10 = 0.0;
        locals.var_qgod_dn13 = 0.0;
        locals.var_qgod_rv = 0.0;

        locals.var_qgbo = 0.0;
        locals.var_qgbo_dn6 = 0.0;
        locals.var_qgbo_dn7 = 0.0;
        locals.var_qgbo_dn8 = 0.0;
        locals.var_qgbo_rv = 0.0;

        locals.var_cgbo_loc = 0.0;
        locals.var_cgbo_loc_rv = 0.0;

        locals.var_qgso = 0.0;
        locals.var_qgso_dn2 = 0.0;
        locals.var_qgso_dn6 = 0.0;
        locals.var_qgso_rv = 0.0;

        locals.var_qgdo = 0.0;
        locals.var_qgdo_dn0 = 0.0;
        locals.var_qgdo_dn2 = 0.0;
        locals.var_qgdo_dn6 = 0.0;
        locals.var_qgdo_rv = 0.0;

        locals.var_qfd = 0.0;
        locals.var_qfd_dn0 = 0.0;
        locals.var_qfd_dn2 = 0.0;
        locals.var_qfd_dn6 = 0.0;
        locals.var_qfd_rv = 0.0;

        locals.var_cfd = 0.0;
        locals.var_cfd_rv = 0.0;

        locals.var_qfs = 0.0;
        locals.var_qfs_dn2 = 0.0;
        locals.var_qfs_dn6 = 0.0;
        locals.var_qfs_rv = 0.0;

        locals.var_cfs = 0.0;
        locals.var_cfs_rv = 0.0;

        locals.var_ec = 0.0;
        locals.var_ec_dn0 = 0.0;
        locals.var_ec_dn2 = 0.0;
        locals.var_ec_dn4 = 0.0;
        locals.var_ec_dn5 = 0.0;
        locals.var_ec_dn6 = 0.0;
        locals.var_ec_dn7 = 0.0;
        locals.var_ec_dn8 = 0.0;
        locals.var_ec_dn9 = 0.0;
        locals.var_ec_dn10 = 0.0;
        locals.var_ec_dn13 = 0.0;
        locals.var_ec_rv = 0.0;

        locals.var_pslk = 0.0;
        locals.var_pslk_dn0 = 0.0;
        locals.var_pslk_dn2 = 0.0;
        locals.var_pslk_dn4 = 0.0;
        locals.var_pslk_dn5 = 0.0;
        locals.var_pslk_dn6 = 0.0;
        locals.var_pslk_dn7 = 0.0;
        locals.var_pslk_dn8 = 0.0;
        locals.var_pslk_dn9 = 0.0;
        locals.var_pslk_dn10 = 0.0;
        locals.var_pslk_dn13 = 0.0;
        locals.var_pslk_rv = 0.0;

        locals.var_qy = 0.0;
        locals.var_qy_dn0 = 0.0;
        locals.var_qy_dn2 = 0.0;
        locals.var_qy_dn4 = 0.0;
        locals.var_qy_dn5 = 0.0;
        locals.var_qy_dn6 = 0.0;
        locals.var_qy_dn7 = 0.0;
        locals.var_qy_dn8 = 0.0;
        locals.var_qy_dn9 = 0.0;
        locals.var_qy_dn10 = 0.0;
        locals.var_qy_dn13 = 0.0;
        locals.var_qy_rv = 0.0;

        locals.var_eyd = 0.0;
        locals.var_eyd_dn0 = 0.0;
        locals.var_eyd_dn2 = 0.0;
        locals.var_eyd_dn4 = 0.0;
        locals.var_eyd_dn5 = 0.0;
        locals.var_eyd_dn6 = 0.0;
        locals.var_eyd_dn7 = 0.0;
        locals.var_eyd_dn8 = 0.0;
        locals.var_eyd_dn9 = 0.0;
        locals.var_eyd_dn10 = 0.0;
        locals.var_eyd_dn13 = 0.0;
        locals.var_eyd_rv = 0.0;

        locals.var_mu_ave = 0.0;
        locals.var_mu_ave_dn0 = 0.0;
        locals.var_mu_ave_dn2 = 0.0;
        locals.var_mu_ave_dn4 = 0.0;
        locals.var_mu_ave_dn5 = 0.0;
        locals.var_mu_ave_dn6 = 0.0;
        locals.var_mu_ave_dn7 = 0.0;
        locals.var_mu_ave_dn8 = 0.0;
        locals.var_mu_ave_dn9 = 0.0;
        locals.var_mu_ave_dn10 = 0.0;
        locals.var_mu_ave_dn13 = 0.0;
        locals.var_mu_ave_rv = 0.0;

        locals.var_nthrml = 0.0;
        locals.var_nthrml_dn0 = 0.0;
        locals.var_nthrml_dn2 = 0.0;
        locals.var_nthrml_dn4 = 0.0;
        locals.var_nthrml_dn5 = 0.0;
        locals.var_nthrml_dn6 = 0.0;
        locals.var_nthrml_dn7 = 0.0;
        locals.var_nthrml_dn8 = 0.0;
        locals.var_nthrml_dn9 = 0.0;
        locals.var_nthrml_dn10 = 0.0;
        locals.var_nthrml_dn13 = 0.0;
        locals.var_nthrml_rv = 0.0;

        locals.var_mud_hoso = 0.0;
        locals.var_mud_hoso_dn0 = 0.0;
        locals.var_mud_hoso_dn2 = 0.0;
        locals.var_mud_hoso_dn4 = 0.0;
        locals.var_mud_hoso_dn5 = 0.0;
        locals.var_mud_hoso_dn6 = 0.0;
        locals.var_mud_hoso_dn7 = 0.0;
        locals.var_mud_hoso_dn8 = 0.0;
        locals.var_mud_hoso_dn9 = 0.0;
        locals.var_mud_hoso_dn10 = 0.0;
        locals.var_mud_hoso_dn13 = 0.0;
        locals.var_mud_hoso_rv = 0.0;

        locals.var_kusai00 = 0.0;
        locals.var_kusai00_dn0 = 0.0;
        locals.var_kusai00_dn2 = 0.0;
        locals.var_kusai00_dn4 = 0.0;
        locals.var_kusai00_dn5 = 0.0;
        locals.var_kusai00_dn6 = 0.0;
        locals.var_kusai00_dn7 = 0.0;
        locals.var_kusai00_dn8 = 0.0;
        locals.var_kusai00_dn9 = 0.0;
        locals.var_kusai00_dn10 = 0.0;
        locals.var_kusai00_dn13 = 0.0;
        locals.var_kusai00_rv = 0.0;

        locals.var_kusaidd = 0.0;
        locals.var_kusaidd_dn0 = 0.0;
        locals.var_kusaidd_dn2 = 0.0;
        locals.var_kusaidd_dn4 = 0.0;
        locals.var_kusaidd_dn5 = 0.0;
        locals.var_kusaidd_dn6 = 0.0;
        locals.var_kusaidd_dn7 = 0.0;
        locals.var_kusaidd_dn8 = 0.0;
        locals.var_kusaidd_dn9 = 0.0;
        locals.var_kusaidd_dn10 = 0.0;
        locals.var_kusaidd_dn13 = 0.0;
        locals.var_kusaidd_rv = 0.0;

        locals.var_kusail = 0.0;
        locals.var_kusail_dn0 = 0.0;
        locals.var_kusail_dn2 = 0.0;
        locals.var_kusail_dn4 = 0.0;
        locals.var_kusail_dn5 = 0.0;
        locals.var_kusail_dn6 = 0.0;
        locals.var_kusail_dn7 = 0.0;
        locals.var_kusail_dn8 = 0.0;
        locals.var_kusail_dn9 = 0.0;
        locals.var_kusail_dn10 = 0.0;
        locals.var_kusail_dn13 = 0.0;
        locals.var_kusail_rv = 0.0;

        locals.var_kusai00l = 0.0;
        locals.var_kusai00l_dn0 = 0.0;
        locals.var_kusai00l_dn2 = 0.0;
        locals.var_kusai00l_dn4 = 0.0;
        locals.var_kusai00l_dn5 = 0.0;
        locals.var_kusai00l_dn6 = 0.0;
        locals.var_kusai00l_dn7 = 0.0;
        locals.var_kusai00l_dn8 = 0.0;
        locals.var_kusai00l_dn9 = 0.0;
        locals.var_kusai00l_dn10 = 0.0;
        locals.var_kusai00l_dn13 = 0.0;
        locals.var_kusai00l_rv = 0.0;

        locals.var_sqrtkusail = 0.0;
        locals.var_sqrtkusail_dn0 = 0.0;
        locals.var_sqrtkusail_dn2 = 0.0;
        locals.var_sqrtkusail_dn4 = 0.0;
        locals.var_sqrtkusail_dn5 = 0.0;
        locals.var_sqrtkusail_dn6 = 0.0;
        locals.var_sqrtkusail_dn7 = 0.0;
        locals.var_sqrtkusail_dn8 = 0.0;
        locals.var_sqrtkusail_dn9 = 0.0;
        locals.var_sqrtkusail_dn10 = 0.0;
        locals.var_sqrtkusail_dn13 = 0.0;
        locals.var_sqrtkusail_rv = 0.0;

        locals.var_kusai_ig = 0.0;
        locals.var_kusai_ig_dn0 = 0.0;
        locals.var_kusai_ig_dn2 = 0.0;
        locals.var_kusai_ig_dn4 = 0.0;
        locals.var_kusai_ig_dn5 = 0.0;
        locals.var_kusai_ig_dn6 = 0.0;
        locals.var_kusai_ig_dn7 = 0.0;
        locals.var_kusai_ig_dn8 = 0.0;
        locals.var_kusai_ig_dn9 = 0.0;
        locals.var_kusai_ig_dn10 = 0.0;
        locals.var_kusai_ig_dn13 = 0.0;
        locals.var_kusai_ig_rv = 0.0;

        locals.var_gds0_ign = 0.0;
        locals.var_gds0_ign_dn0 = 0.0;
        locals.var_gds0_ign_dn2 = 0.0;
        locals.var_gds0_ign_dn4 = 0.0;
        locals.var_gds0_ign_dn5 = 0.0;
        locals.var_gds0_ign_dn6 = 0.0;
        locals.var_gds0_ign_dn7 = 0.0;
        locals.var_gds0_ign_dn8 = 0.0;
        locals.var_gds0_ign_dn9 = 0.0;
        locals.var_gds0_ign_dn10 = 0.0;
        locals.var_gds0_ign_dn13 = 0.0;
        locals.var_gds0_ign_rv = 0.0;

        locals.var_nign0 = 0.0;
        locals.var_nign0_dn0 = 0.0;
        locals.var_nign0_dn2 = 0.0;
        locals.var_nign0_dn4 = 0.0;
        locals.var_nign0_dn5 = 0.0;
        locals.var_nign0_dn6 = 0.0;
        locals.var_nign0_dn7 = 0.0;
        locals.var_nign0_dn8 = 0.0;
        locals.var_nign0_dn9 = 0.0;
        locals.var_nign0_dn10 = 0.0;
        locals.var_nign0_dn13 = 0.0;
        locals.var_nign0_rv = 0.0;

        locals.var_mumoda = 0.0;
        locals.var_mumoda_dn0 = 0.0;
        locals.var_mumoda_dn2 = 0.0;
        locals.var_mumoda_dn4 = 0.0;
        locals.var_mumoda_dn5 = 0.0;
        locals.var_mumoda_dn6 = 0.0;
        locals.var_mumoda_dn7 = 0.0;
        locals.var_mumoda_dn8 = 0.0;
        locals.var_mumoda_dn9 = 0.0;
        locals.var_mumoda_dn10 = 0.0;
        locals.var_mumoda_dn13 = 0.0;
        locals.var_mumoda_rv = 0.0;

        locals.var_mumodb = 0.0;
        locals.var_mumodb_dn0 = 0.0;
        locals.var_mumodb_dn2 = 0.0;
        locals.var_mumodb_dn4 = 0.0;
        locals.var_mumodb_dn5 = 0.0;
        locals.var_mumodb_dn6 = 0.0;
        locals.var_mumodb_dn7 = 0.0;
        locals.var_mumodb_dn8 = 0.0;
        locals.var_mumodb_dn9 = 0.0;
        locals.var_mumodb_dn10 = 0.0;
        locals.var_mumodb_dn13 = 0.0;
        locals.var_mumodb_rv = 0.0;

        locals.var_correct_w1 = 0.0;
        locals.var_correct_w1_dn0 = 0.0;
        locals.var_correct_w1_dn2 = 0.0;
        locals.var_correct_w1_dn4 = 0.0;
        locals.var_correct_w1_dn5 = 0.0;
        locals.var_correct_w1_dn6 = 0.0;
        locals.var_correct_w1_dn7 = 0.0;
        locals.var_correct_w1_dn8 = 0.0;
        locals.var_correct_w1_dn9 = 0.0;
        locals.var_correct_w1_dn10 = 0.0;
        locals.var_correct_w1_dn13 = 0.0;
        locals.var_correct_w1_rv = 0.0;

        locals.var_tx = 0.0;
        locals.var_tx_dn0 = 0.0;
        locals.var_tx_dn2 = 0.0;
        locals.var_tx_dn4 = 0.0;
        locals.var_tx_dn5 = 0.0;
        locals.var_tx_dn6 = 0.0;
        locals.var_tx_dn7 = 0.0;
        locals.var_tx_dn8 = 0.0;
        locals.var_tx_dn9 = 0.0;
        locals.var_tx_dn10 = 0.0;
        locals.var_tx_dn13 = 0.0;
        locals.var_tx_rv = 0.0;

        locals.var_ty = 0.0;
        locals.var_ty_dn0 = 0.0;
        locals.var_ty_dn2 = 0.0;
        locals.var_ty_dn4 = 0.0;
        locals.var_ty_dn5 = 0.0;
        locals.var_ty_dn6 = 0.0;
        locals.var_ty_dn7 = 0.0;
        locals.var_ty_dn8 = 0.0;
        locals.var_ty_dn9 = 0.0;
        locals.var_ty_dn10 = 0.0;
        locals.var_ty_dn13 = 0.0;
        locals.var_ty_rv = 0.0;

        locals.var_t0 = 0.0;
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

        locals.var_t1 = 0.0;
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

        locals.var_t2 = 0.0;
        locals.var_t2_dn0 = 0.0;
        locals.var_t2_dn2 = 0.0;
        locals.var_t2_dn4 = 0.0;
        locals.var_t2_dn5 = 0.0;
        locals.var_t2_dn6 = 0.0;
        locals.var_t2_dn7 = 0.0;
        locals.var_t2_dn8 = 0.0;
        locals.var_t2_dn9 = 0.0;
        locals.var_t2_dn10 = 0.0;
        locals.var_t2_dn13 = 0.0;
        locals.var_t2_rv = 0.0;

        locals.var_t3 = 0.0;
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

        locals.var_t4 = 0.0;
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

        locals.var_t5 = 0.0;
        locals.var_t5_dn0 = 0.0;
        locals.var_t5_dn2 = 0.0;
        locals.var_t5_dn4 = 0.0;
        locals.var_t5_dn5 = 0.0;
        locals.var_t5_dn6 = 0.0;
        locals.var_t5_dn7 = 0.0;
        locals.var_t5_dn8 = 0.0;
        locals.var_t5_dn9 = 0.0;
        locals.var_t5_dn10 = 0.0;
        locals.var_t5_dn13 = 0.0;
        locals.var_t5_rv = 0.0;

        locals.var_t6 = 0.0;
        locals.var_t6_dn0 = 0.0;
        locals.var_t6_dn2 = 0.0;
        locals.var_t6_dn4 = 0.0;
        locals.var_t6_dn5 = 0.0;
        locals.var_t6_dn6 = 0.0;
        locals.var_t6_dn7 = 0.0;
        locals.var_t6_dn8 = 0.0;
        locals.var_t6_dn9 = 0.0;
        locals.var_t6_dn10 = 0.0;
        locals.var_t6_dn13 = 0.0;
        locals.var_t6_rv = 0.0;

        locals.var_t7 = 0.0;
        locals.var_t7_dn0 = 0.0;
        locals.var_t7_dn2 = 0.0;
        locals.var_t7_dn4 = 0.0;
        locals.var_t7_dn5 = 0.0;
        locals.var_t7_dn6 = 0.0;
        locals.var_t7_dn7 = 0.0;
        locals.var_t7_dn8 = 0.0;
        locals.var_t7_dn9 = 0.0;
        locals.var_t7_dn10 = 0.0;
        locals.var_t7_dn13 = 0.0;
        locals.var_t7_rv = 0.0;

        locals.var_t8 = 0.0;
        locals.var_t8_dn0 = 0.0;
        locals.var_t8_dn2 = 0.0;
        locals.var_t8_dn4 = 0.0;
        locals.var_t8_dn5 = 0.0;
        locals.var_t8_dn6 = 0.0;
        locals.var_t8_dn7 = 0.0;
        locals.var_t8_dn8 = 0.0;
        locals.var_t8_dn9 = 0.0;
        locals.var_t8_dn10 = 0.0;
        locals.var_t8_dn13 = 0.0;
        locals.var_t8_rv = 0.0;

        locals.var_t9 = 0.0;
        locals.var_t9_dn0 = 0.0;
        locals.var_t9_dn2 = 0.0;
        locals.var_t9_dn4 = 0.0;
        locals.var_t9_dn5 = 0.0;
        locals.var_t9_dn6 = 0.0;
        locals.var_t9_dn7 = 0.0;
        locals.var_t9_dn8 = 0.0;
        locals.var_t9_dn9 = 0.0;
        locals.var_t9_dn10 = 0.0;
        locals.var_t9_dn13 = 0.0;
        locals.var_t9_rv = 0.0;

        locals.var_t10 = 0.0;
        locals.var_t10_dn0 = 0.0;
        locals.var_t10_dn2 = 0.0;
        locals.var_t10_dn4 = 0.0;
        locals.var_t10_dn5 = 0.0;
        locals.var_t10_dn6 = 0.0;
        locals.var_t10_dn7 = 0.0;
        locals.var_t10_dn8 = 0.0;
        locals.var_t10_dn9 = 0.0;
        locals.var_t10_dn10 = 0.0;
        locals.var_t10_dn13 = 0.0;
        locals.var_t10_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_6(
        locals: &mut StampLocals,
    ) {
        locals.var_t11 = 0.0;
        locals.var_t11_dn0 = 0.0;
        locals.var_t11_dn2 = 0.0;
        locals.var_t11_dn4 = 0.0;
        locals.var_t11_dn5 = 0.0;
        locals.var_t11_dn6 = 0.0;
        locals.var_t11_dn7 = 0.0;
        locals.var_t11_dn8 = 0.0;
        locals.var_t11_dn9 = 0.0;
        locals.var_t11_dn10 = 0.0;
        locals.var_t11_dn13 = 0.0;
        locals.var_t11_rv = 0.0;

        locals.var_t12 = 0.0;
        locals.var_t12_dn0 = 0.0;
        locals.var_t12_dn2 = 0.0;
        locals.var_t12_dn4 = 0.0;
        locals.var_t12_dn5 = 0.0;
        locals.var_t12_dn6 = 0.0;
        locals.var_t12_dn7 = 0.0;
        locals.var_t12_dn8 = 0.0;
        locals.var_t12_dn9 = 0.0;
        locals.var_t12_dn10 = 0.0;
        locals.var_t12_dn13 = 0.0;
        locals.var_t12_rv = 0.0;

        locals.var_vdseff = 0.0;
        locals.var_vdseff_dn0 = 0.0;
        locals.var_vdseff_dn2 = 0.0;
        locals.var_vdseff_dn4 = 0.0;
        locals.var_vdseff_dn5 = 0.0;
        locals.var_vdseff_dn6 = 0.0;
        locals.var_vdseff_dn7 = 0.0;
        locals.var_vdseff_dn8 = 0.0;
        locals.var_vdseff_dn9 = 0.0;
        locals.var_vdseff_dn10 = 0.0;
        locals.var_vdseff_dn13 = 0.0;
        locals.var_vdseff_rv = 0.0;

        locals.var_vdsorg = 0.0;
        locals.var_vdsorg_dn0 = 0.0;
        locals.var_vdsorg_dn2 = 0.0;
        locals.var_vdsorg_dn4 = 0.0;
        locals.var_vdsorg_dn5 = 0.0;
        locals.var_vdsorg_dn6 = 0.0;
        locals.var_vdsorg_dn7 = 0.0;
        locals.var_vdsorg_dn8 = 0.0;
        locals.var_vdsorg_dn9 = 0.0;
        locals.var_vdsorg_dn10 = 0.0;
        locals.var_vdsorg_dn13 = 0.0;
        locals.var_vdsorg_rv = 0.0;

        locals.var_qovdext = 0.0;
        locals.var_qovdext_dn0 = 0.0;
        locals.var_qovdext_dn2 = 0.0;
        locals.var_qovdext_dn4 = 0.0;
        locals.var_qovdext_dn5 = 0.0;
        locals.var_qovdext_dn6 = 0.0;
        locals.var_qovdext_dn7 = 0.0;
        locals.var_qovdext_dn8 = 0.0;
        locals.var_qovdext_dn9 = 0.0;
        locals.var_qovdext_dn10 = 0.0;
        locals.var_qovdext_dn13 = 0.0;
        locals.var_qovdext_rv = 0.0;

        locals.var_qovsext = 0.0;
        locals.var_qovsext_dn0 = 0.0;
        locals.var_qovsext_dn2 = 0.0;
        locals.var_qovsext_dn4 = 0.0;
        locals.var_qovsext_dn5 = 0.0;
        locals.var_qovsext_dn6 = 0.0;
        locals.var_qovsext_dn7 = 0.0;
        locals.var_qovsext_dn8 = 0.0;
        locals.var_qovsext_dn9 = 0.0;
        locals.var_qovsext_dn10 = 0.0;
        locals.var_qovsext_dn13 = 0.0;
        locals.var_qovsext_rv = 0.0;

        locals.var_qovd = 0.0;
        locals.var_qovd_dn0 = 0.0;
        locals.var_qovd_dn2 = 0.0;
        locals.var_qovd_dn4 = 0.0;
        locals.var_qovd_dn5 = 0.0;
        locals.var_qovd_dn6 = 0.0;
        locals.var_qovd_dn7 = 0.0;
        locals.var_qovd_dn8 = 0.0;
        locals.var_qovd_dn9 = 0.0;
        locals.var_qovd_dn10 = 0.0;
        locals.var_qovd_dn13 = 0.0;
        locals.var_qovd_rv = 0.0;

        locals.var_qovs = 0.0;
        locals.var_qovs_dn0 = 0.0;
        locals.var_qovs_dn2 = 0.0;
        locals.var_qovs_dn4 = 0.0;
        locals.var_qovs_dn5 = 0.0;
        locals.var_qovs_dn6 = 0.0;
        locals.var_qovs_dn7 = 0.0;
        locals.var_qovs_dn8 = 0.0;
        locals.var_qovs_dn9 = 0.0;
        locals.var_qovs_dn10 = 0.0;
        locals.var_qovs_dn13 = 0.0;
        locals.var_qovs_rv = 0.0;

        locals.var_qbuld = 0.0;
        locals.var_qbuld_dn0 = 0.0;
        locals.var_qbuld_dn2 = 0.0;
        locals.var_qbuld_dn4 = 0.0;
        locals.var_qbuld_dn5 = 0.0;
        locals.var_qbuld_dn6 = 0.0;
        locals.var_qbuld_dn7 = 0.0;
        locals.var_qbuld_dn8 = 0.0;
        locals.var_qbuld_dn9 = 0.0;
        locals.var_qbuld_dn10 = 0.0;
        locals.var_qbuld_dn13 = 0.0;
        locals.var_qbuld_rv = 0.0;

        locals.var_qbdld = 0.0;
        locals.var_qbdld_dn0 = 0.0;
        locals.var_qbdld_dn2 = 0.0;
        locals.var_qbdld_dn4 = 0.0;
        locals.var_qbdld_dn5 = 0.0;
        locals.var_qbdld_dn6 = 0.0;
        locals.var_qbdld_dn7 = 0.0;
        locals.var_qbdld_dn8 = 0.0;
        locals.var_qbdld_dn9 = 0.0;
        locals.var_qbdld_dn10 = 0.0;
        locals.var_qbdld_dn13 = 0.0;
        locals.var_qbdld_rv = 0.0;

        locals.var_qbsld = 0.0;
        locals.var_qbsld_dn0 = 0.0;
        locals.var_qbsld_dn2 = 0.0;
        locals.var_qbsld_dn4 = 0.0;
        locals.var_qbsld_dn5 = 0.0;
        locals.var_qbsld_dn6 = 0.0;
        locals.var_qbsld_dn7 = 0.0;
        locals.var_qbsld_dn8 = 0.0;
        locals.var_qbsld_dn9 = 0.0;
        locals.var_qbsld_dn10 = 0.0;
        locals.var_qbsld_dn13 = 0.0;
        locals.var_qbsld_rv = 0.0;

        locals.var_qodad = 0.0;
        locals.var_qodad_dn0 = 0.0;
        locals.var_qodad_dn2 = 0.0;
        locals.var_qodad_dn4 = 0.0;
        locals.var_qodad_dn5 = 0.0;
        locals.var_qodad_dn6 = 0.0;
        locals.var_qodad_dn7 = 0.0;
        locals.var_qodad_dn8 = 0.0;
        locals.var_qodad_dn9 = 0.0;
        locals.var_qodad_dn10 = 0.0;
        locals.var_qodad_dn13 = 0.0;
        locals.var_qodad_rv = 0.0;

        locals.var_qbdldext = 0.0;
        locals.var_qbdldext_dn0 = 0.0;
        locals.var_qbdldext_dn2 = 0.0;
        locals.var_qbdldext_dn4 = 0.0;
        locals.var_qbdldext_dn5 = 0.0;
        locals.var_qbdldext_dn6 = 0.0;
        locals.var_qbdldext_dn7 = 0.0;
        locals.var_qbdldext_dn8 = 0.0;
        locals.var_qbdldext_dn9 = 0.0;
        locals.var_qbdldext_dn10 = 0.0;
        locals.var_qbdldext_dn13 = 0.0;
        locals.var_qbdldext_rv = 0.0;

        locals.var_qbsldext = 0.0;
        locals.var_qbsldext_dn0 = 0.0;
        locals.var_qbsldext_dn2 = 0.0;
        locals.var_qbsldext_dn4 = 0.0;
        locals.var_qbsldext_dn5 = 0.0;
        locals.var_qbsldext_dn6 = 0.0;
        locals.var_qbsldext_dn7 = 0.0;
        locals.var_qbsldext_dn8 = 0.0;
        locals.var_qbsldext_dn9 = 0.0;
        locals.var_qbsldext_dn10 = 0.0;
        locals.var_qbsldext_dn13 = 0.0;
        locals.var_qbsldext_rv = 0.0;

        locals.var_vbsz2 = 0.0;
        locals.var_vbsz2_dn0 = 0.0;
        locals.var_vbsz2_dn2 = 0.0;
        locals.var_vbsz2_dn4 = 0.0;
        locals.var_vbsz2_dn5 = 0.0;
        locals.var_vbsz2_dn6 = 0.0;
        locals.var_vbsz2_dn7 = 0.0;
        locals.var_vbsz2_dn8 = 0.0;
        locals.var_vbsz2_dn9 = 0.0;
        locals.var_vbsz2_dn10 = 0.0;
        locals.var_vbsz2_dn13 = 0.0;
        locals.var_vbsz2_rv = 0.0;

        locals.var_rdrift = 0.0;
        locals.var_rdrift_dn0 = 0.0;
        locals.var_rdrift_dn2 = 0.0;
        locals.var_rdrift_dn4 = 0.0;
        locals.var_rdrift_dn5 = 0.0;
        locals.var_rdrift_dn6 = 0.0;
        locals.var_rdrift_dn7 = 0.0;
        locals.var_rdrift_dn8 = 0.0;
        locals.var_rdrift_dn9 = 0.0;
        locals.var_rdrift_dn10 = 0.0;
        locals.var_rdrift_dn13 = 0.0;
        locals.var_rdrift_rv = 0.0;

        locals.var_rsdrift = 0.0;
        locals.var_rsdrift_dn0 = 0.0;
        locals.var_rsdrift_dn2 = 0.0;
        locals.var_rsdrift_dn4 = 0.0;
        locals.var_rsdrift_dn5 = 0.0;
        locals.var_rsdrift_dn6 = 0.0;
        locals.var_rsdrift_dn7 = 0.0;
        locals.var_rsdrift_dn8 = 0.0;
        locals.var_rsdrift_dn9 = 0.0;
        locals.var_rsdrift_dn10 = 0.0;
        locals.var_rsdrift_dn13 = 0.0;
        locals.var_rsdrift_rv = 0.0;

        locals.var_ra = 0.0;
        locals.var_ra_dn0 = 0.0;
        locals.var_ra_dn2 = 0.0;
        locals.var_ra_dn4 = 0.0;
        locals.var_ra_dn5 = 0.0;
        locals.var_ra_dn6 = 0.0;
        locals.var_ra_dn7 = 0.0;
        locals.var_ra_dn8 = 0.0;
        locals.var_ra_dn9 = 0.0;
        locals.var_ra_dn10 = 0.0;
        locals.var_ra_dn13 = 0.0;
        locals.var_ra_rv = 0.0;

        locals.var_vdse_eff = 0.0;
        locals.var_vdse_eff_dn0 = 0.0;
        locals.var_vdse_eff_dn2 = 0.0;
        locals.var_vdse_eff_rv = 0.0;

        locals.var_vdsemodenml = 0.0;
        locals.var_vdsemodenml_rv = 0.0;

        locals.var_vdsemodervs = 0.0;
        locals.var_vdsemodervs_rv = 0.0;

        locals.var_vbsegmt = 0.0;
        locals.var_vbsegmt_dn2 = 0.0;
        locals.var_vbsegmt_dn8 = 0.0;
        locals.var_vbsegmt_rv = 0.0;

        locals.var_vdsegmt = 0.0;
        locals.var_vdsegmt_dn0 = 0.0;
        locals.var_vdsegmt_dn2 = 0.0;
        locals.var_vdsegmt_rv = 0.0;

        locals.var_vgsegmt = 0.0;
        locals.var_vgsegmt_dn2 = 0.0;
        locals.var_vgsegmt_dn6 = 0.0;
        locals.var_vgsegmt_rv = 0.0;

        locals.var_vbserev = 0.0;
        locals.var_vbserev_dn0 = 0.0;
        locals.var_vbserev_dn2 = 0.0;
        locals.var_vbserev_dn8 = 0.0;
        locals.var_vbserev_rv = 0.0;

        locals.var_vdserev = 0.0;
        locals.var_vdserev_dn0 = 0.0;
        locals.var_vdserev_dn2 = 0.0;
        locals.var_vdserev_rv = 0.0;

        locals.var_vgserev = 0.0;
        locals.var_vgserev_dn0 = 0.0;
        locals.var_vgserev_dn2 = 0.0;
        locals.var_vgserev_dn6 = 0.0;
        locals.var_vgserev_rv = 0.0;

        locals.var_vdserevz = 0.0;
        locals.var_vdserevz_dn0 = 0.0;
        locals.var_vdserevz_dn2 = 0.0;
        locals.var_vdserevz_dn4 = 0.0;
        locals.var_vdserevz_dn5 = 0.0;
        locals.var_vdserevz_dn6 = 0.0;
        locals.var_vdserevz_dn7 = 0.0;
        locals.var_vdserevz_dn8 = 0.0;
        locals.var_vdserevz_dn9 = 0.0;
        locals.var_vdserevz_dn10 = 0.0;
        locals.var_vdserevz_dn13 = 0.0;
        locals.var_vdserevz_rv = 0.0;

        locals.var_vgserevz = 0.0;
        locals.var_vgserevz_dn0 = 0.0;
        locals.var_vgserevz_dn2 = 0.0;
        locals.var_vgserevz_dn4 = 0.0;
        locals.var_vgserevz_dn5 = 0.0;
        locals.var_vgserevz_dn6 = 0.0;
        locals.var_vgserevz_dn7 = 0.0;
        locals.var_vgserevz_dn8 = 0.0;
        locals.var_vgserevz_dn9 = 0.0;
        locals.var_vgserevz_dn10 = 0.0;
        locals.var_vgserevz_dn13 = 0.0;
        locals.var_vgserevz_rv = 0.0;

        locals.var_vbserevz = 0.0;
        locals.var_vbserevz_dn0 = 0.0;
        locals.var_vbserevz_dn2 = 0.0;
        locals.var_vbserevz_dn4 = 0.0;
        locals.var_vbserevz_dn5 = 0.0;
        locals.var_vbserevz_dn6 = 0.0;
        locals.var_vbserevz_dn7 = 0.0;
        locals.var_vbserevz_dn8 = 0.0;
        locals.var_vbserevz_dn9 = 0.0;
        locals.var_vbserevz_dn10 = 0.0;
        locals.var_vbserevz_dn13 = 0.0;
        locals.var_vbserevz_rv = 0.0;

        locals.var_vsubsrev = 0.0;
        locals.var_vsubsrev_dn0 = 0.0;
        locals.var_vsubsrev_dn2 = 0.0;
        locals.var_vsubsrev_rv = 0.0;

        locals.var_ttemp = 0.0;
        locals.var_ttemp_dn0 = 0.0;
        locals.var_ttemp_dn2 = 0.0;
        locals.var_ttemp_dn4 = 0.0;
        locals.var_ttemp_dn5 = 0.0;
        locals.var_ttemp_dn6 = 0.0;
        locals.var_ttemp_dn7 = 0.0;
        locals.var_ttemp_dn8 = 0.0;
        locals.var_ttemp_dn9 = 0.0;
        locals.var_ttemp_dn10 = 0.0;
        locals.var_ttemp_dn13 = 0.0;
        locals.var_ttemp_rv = 0.0;

        locals.var_ttemp0 = 0.0;
        locals.var_ttemp0_dn0 = 0.0;
        locals.var_ttemp0_dn2 = 0.0;
        locals.var_ttemp0_dn4 = 0.0;
        locals.var_ttemp0_dn5 = 0.0;
        locals.var_ttemp0_dn6 = 0.0;
        locals.var_ttemp0_dn7 = 0.0;
        locals.var_ttemp0_dn8 = 0.0;
        locals.var_ttemp0_dn9 = 0.0;
        locals.var_ttemp0_dn10 = 0.0;
        locals.var_ttemp0_dn13 = 0.0;
        locals.var_ttemp0_rv = 0.0;

        locals.var_tdiff0 = 0.0;
        locals.var_tdiff0_dn0 = 0.0;
        locals.var_tdiff0_dn2 = 0.0;
        locals.var_tdiff0_dn4 = 0.0;
        locals.var_tdiff0_dn5 = 0.0;
        locals.var_tdiff0_dn6 = 0.0;
        locals.var_tdiff0_dn7 = 0.0;
        locals.var_tdiff0_dn8 = 0.0;
        locals.var_tdiff0_dn9 = 0.0;
        locals.var_tdiff0_dn10 = 0.0;
        locals.var_tdiff0_dn13 = 0.0;
        locals.var_tdiff0_rv = 0.0;

        locals.var_tdiff0_2 = 0.0;
        locals.var_tdiff0_2_dn0 = 0.0;
        locals.var_tdiff0_2_dn2 = 0.0;
        locals.var_tdiff0_2_dn4 = 0.0;
        locals.var_tdiff0_2_dn5 = 0.0;
        locals.var_tdiff0_2_dn6 = 0.0;
        locals.var_tdiff0_2_dn7 = 0.0;
        locals.var_tdiff0_2_dn8 = 0.0;
        locals.var_tdiff0_2_dn9 = 0.0;
        locals.var_tdiff0_2_dn10 = 0.0;
        locals.var_tdiff0_2_dn13 = 0.0;
        locals.var_tdiff0_2_rv = 0.0;

        locals.var_tdiff = 0.0;
        locals.var_tdiff_dn0 = 0.0;
        locals.var_tdiff_dn2 = 0.0;
        locals.var_tdiff_dn4 = 0.0;
        locals.var_tdiff_dn5 = 0.0;
        locals.var_tdiff_dn6 = 0.0;
        locals.var_tdiff_dn7 = 0.0;
        locals.var_tdiff_dn8 = 0.0;
        locals.var_tdiff_dn9 = 0.0;
        locals.var_tdiff_dn10 = 0.0;
        locals.var_tdiff_dn13 = 0.0;
        locals.var_tdiff_rv = 0.0;

        locals.var_tdiff_2 = 0.0;
        locals.var_tdiff_2_dn0 = 0.0;
        locals.var_tdiff_2_dn2 = 0.0;
        locals.var_tdiff_2_dn4 = 0.0;
        locals.var_tdiff_2_dn5 = 0.0;
        locals.var_tdiff_2_dn6 = 0.0;
        locals.var_tdiff_2_dn7 = 0.0;
        locals.var_tdiff_2_dn8 = 0.0;
        locals.var_tdiff_2_dn9 = 0.0;
        locals.var_tdiff_2_dn10 = 0.0;
        locals.var_tdiff_2_dn13 = 0.0;
        locals.var_tdiff_2_rv = 0.0;

        locals.var_eg = 0.0;
        locals.var_eg_dn0 = 0.0;
        locals.var_eg_dn2 = 0.0;
        locals.var_eg_dn4 = 0.0;
        locals.var_eg_dn5 = 0.0;
        locals.var_eg_dn6 = 0.0;
        locals.var_eg_dn7 = 0.0;
        locals.var_eg_dn8 = 0.0;
        locals.var_eg_dn9 = 0.0;
        locals.var_eg_dn10 = 0.0;
        locals.var_eg_dn13 = 0.0;
        locals.var_eg_rv = 0.0;

        locals.var_nin = 0.0;
        locals.var_nin_dn0 = 0.0;
        locals.var_nin_dn2 = 0.0;
        locals.var_nin_dn4 = 0.0;
        locals.var_nin_dn5 = 0.0;
        locals.var_nin_dn6 = 0.0;
        locals.var_nin_dn7 = 0.0;
        locals.var_nin_dn8 = 0.0;
        locals.var_nin_dn9 = 0.0;
        locals.var_nin_dn10 = 0.0;
        locals.var_nin_dn13 = 0.0;
        locals.var_nin_rv = 0.0;

        locals.var_vgbgmt = 0.0;
        locals.var_vgbgmt_dn2 = 0.0;
        locals.var_vgbgmt_dn6 = 0.0;
        locals.var_vgbgmt_dn7 = 0.0;
        locals.var_vgbgmt_dn8 = 0.0;
        locals.var_vgbgmt_rv = 0.0;

        locals.var_vxbgmt = 0.0;
        locals.var_vxbgmt_dn0 = 0.0;
        locals.var_vxbgmt_dn2 = 0.0;
        locals.var_vxbgmt_dn4 = 0.0;
        locals.var_vxbgmt_dn5 = 0.0;
        locals.var_vxbgmt_dn6 = 0.0;
        locals.var_vxbgmt_dn7 = 0.0;
        locals.var_vxbgmt_dn8 = 0.0;
        locals.var_vxbgmt_dn9 = 0.0;
        locals.var_vxbgmt_dn10 = 0.0;
        locals.var_vxbgmt_dn13 = 0.0;
        locals.var_vxbgmt_rv = 0.0;

        locals.var_vxbgmtcl = 0.0;
        locals.var_vxbgmtcl_dn0 = 0.0;
        locals.var_vxbgmtcl_dn2 = 0.0;
        locals.var_vxbgmtcl_dn4 = 0.0;
        locals.var_vxbgmtcl_dn5 = 0.0;
        locals.var_vxbgmtcl_dn6 = 0.0;
        locals.var_vxbgmtcl_dn7 = 0.0;
        locals.var_vxbgmtcl_dn8 = 0.0;
        locals.var_vxbgmtcl_dn9 = 0.0;
        locals.var_vxbgmtcl_dn10 = 0.0;
        locals.var_vxbgmtcl_dn13 = 0.0;
        locals.var_vxbgmtcl_rv = 0.0;

        locals.var_qsuld = 0.0;
        locals.var_qsuld_dn0 = 0.0;
        locals.var_qsuld_dn2 = 0.0;
        locals.var_qsuld_dn4 = 0.0;
        locals.var_qsuld_dn5 = 0.0;
        locals.var_qsuld_dn6 = 0.0;
        locals.var_qsuld_dn7 = 0.0;
        locals.var_qsuld_dn8 = 0.0;
        locals.var_qsuld_dn9 = 0.0;
        locals.var_qsuld_dn10 = 0.0;
        locals.var_qsuld_dn13 = 0.0;
        locals.var_qsuld_rv = 0.0;

        locals.var_qiuld = 0.0;
        locals.var_qiuld_dn0 = 0.0;
        locals.var_qiuld_dn2 = 0.0;
        locals.var_qiuld_dn4 = 0.0;
        locals.var_qiuld_dn5 = 0.0;
        locals.var_qiuld_dn6 = 0.0;
        locals.var_qiuld_dn7 = 0.0;
        locals.var_qiuld_dn8 = 0.0;
        locals.var_qiuld_dn9 = 0.0;
        locals.var_qiuld_dn10 = 0.0;
        locals.var_qiuld_dn13 = 0.0;
        locals.var_qiuld_rv = 0.0;

        locals.var_idsibpc = 0.0;
        locals.var_idsibpc_dn0 = 0.0;
        locals.var_idsibpc_dn2 = 0.0;
        locals.var_idsibpc_dn4 = 0.0;
        locals.var_idsibpc_dn5 = 0.0;
        locals.var_idsibpc_dn6 = 0.0;
        locals.var_idsibpc_dn7 = 0.0;
        locals.var_idsibpc_dn8 = 0.0;
        locals.var_idsibpc_dn9 = 0.0;
        locals.var_idsibpc_dn10 = 0.0;
        locals.var_idsibpc_dn13 = 0.0;
        locals.var_idsibpc_rv = 0.0;

        locals.var_vgpld = 0.0;
        locals.var_vgpld_dn2 = 0.0;
        locals.var_vgpld_dn6 = 0.0;
        locals.var_vgpld_dn7 = 0.0;
        locals.var_vgpld_dn8 = 0.0;
        locals.var_vgpld_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_7(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        locals.var_vgb_fb_ld = 0.0;
        locals.var_vgb_fb_ld_dn0 = 0.0;
        locals.var_vgb_fb_ld_dn2 = 0.0;
        locals.var_vgb_fb_ld_dn4 = 0.0;
        locals.var_vgb_fb_ld_dn5 = 0.0;
        locals.var_vgb_fb_ld_dn6 = 0.0;
        locals.var_vgb_fb_ld_dn7 = 0.0;
        locals.var_vgb_fb_ld_dn8 = 0.0;
        locals.var_vgb_fb_ld_dn9 = 0.0;
        locals.var_vgb_fb_ld_dn10 = 0.0;
        locals.var_vgb_fb_ld_dn13 = 0.0;
        locals.var_vgb_fb_ld_rv = 0.0;

        locals.var_ps0ld = 0.0;
        locals.var_ps0ld_dn0 = 0.0;
        locals.var_ps0ld_dn2 = 0.0;
        locals.var_ps0ld_dn4 = 0.0;
        locals.var_ps0ld_dn5 = 0.0;
        locals.var_ps0ld_dn6 = 0.0;
        locals.var_ps0ld_dn7 = 0.0;
        locals.var_ps0ld_dn8 = 0.0;
        locals.var_ps0ld_dn9 = 0.0;
        locals.var_ps0ld_dn10 = 0.0;
        locals.var_ps0ld_dn13 = 0.0;
        locals.var_ps0ld_rv = 0.0;

        locals.var_cnst1over = 0.0;
        locals.var_cnst1over_dn0 = 0.0;
        locals.var_cnst1over_dn2 = 0.0;
        locals.var_cnst1over_dn4 = 0.0;
        locals.var_cnst1over_dn5 = 0.0;
        locals.var_cnst1over_dn6 = 0.0;
        locals.var_cnst1over_dn7 = 0.0;
        locals.var_cnst1over_dn8 = 0.0;
        locals.var_cnst1over_dn9 = 0.0;
        locals.var_cnst1over_dn10 = 0.0;
        locals.var_cnst1over_dn13 = 0.0;
        locals.var_cnst1over_rv = 0.0;

        locals.var_ddriftld = p.p334;
        locals.var_ddriftld_dn0 = 0.0;
        locals.var_ddriftld_dn2 = 0.0;
        locals.var_ddriftld_dn4 = 0.0;
        locals.var_ddriftld_dn5 = 0.0;
        locals.var_ddriftld_dn6 = 0.0;
        locals.var_ddriftld_dn7 = 0.0;
        locals.var_ddriftld_dn8 = 0.0;
        locals.var_ddriftld_dn9 = 0.0;
        locals.var_ddriftld_dn10 = 0.0;
        locals.var_ddriftld_dn13 = 0.0;
        locals.var_ddriftld_rv = 0.0;

        locals.var_ddriftldc = p.p334;
        locals.var_ddriftldc_dn0 = 0.0;
        locals.var_ddriftldc_dn2 = 0.0;
        locals.var_ddriftldc_dn4 = 0.0;
        locals.var_ddriftldc_dn5 = 0.0;
        locals.var_ddriftldc_dn6 = 0.0;
        locals.var_ddriftldc_dn7 = 0.0;
        locals.var_ddriftldc_dn8 = 0.0;
        locals.var_ddriftldc_dn9 = 0.0;
        locals.var_ddriftldc_dn10 = 0.0;
        locals.var_ddriftldc_dn13 = 0.0;
        locals.var_ddriftldc_rv = 0.0;

        locals.var_nover_func = 0.0;
        locals.var_nover_func_rv = 0.0;

        locals.var_cnst0over_func = 0.0;
        locals.var_cnst0over_func_dn0 = 0.0;
        locals.var_cnst0over_func_dn2 = 0.0;
        locals.var_cnst0over_func_dn4 = 0.0;
        locals.var_cnst0over_func_dn5 = 0.0;
        locals.var_cnst0over_func_dn6 = 0.0;
        locals.var_cnst0over_func_dn7 = 0.0;
        locals.var_cnst0over_func_dn8 = 0.0;
        locals.var_cnst0over_func_dn9 = 0.0;
        locals.var_cnst0over_func_dn10 = 0.0;
        locals.var_cnst0over_func_dn13 = 0.0;
        locals.var_cnst0over_func_rv = 0.0;

        locals.var_ta = 0.0093868;
        locals.var_ta_rv = 0.0;

        let assign3320_e1728: f64 = (-0.1047839);
        locals.var_tb = assign3320_e1728;
        locals.var_tb_rv = 0.0;

        locals.var_chi_1 = 0.0;
        locals.var_chi_1_dn0 = 0.0;
        locals.var_chi_1_dn2 = 0.0;
        locals.var_chi_1_dn4 = 0.0;
        locals.var_chi_1_dn5 = 0.0;
        locals.var_chi_1_dn6 = 0.0;
        locals.var_chi_1_dn7 = 0.0;
        locals.var_chi_1_dn8 = 0.0;
        locals.var_chi_1_dn9 = 0.0;
        locals.var_chi_1_dn10 = 0.0;
        locals.var_chi_1_dn13 = 0.0;
        locals.var_chi_1_rv = 0.0;

        locals.var_mueph = 0.0;
        locals.var_mueph_dn0 = 0.0;
        locals.var_mueph_dn2 = 0.0;
        locals.var_mueph_dn4 = 0.0;
        locals.var_mueph_dn5 = 0.0;
        locals.var_mueph_dn6 = 0.0;
        locals.var_mueph_dn7 = 0.0;
        locals.var_mueph_dn8 = 0.0;
        locals.var_mueph_dn9 = 0.0;
        locals.var_mueph_dn10 = 0.0;
        locals.var_mueph_dn13 = 0.0;
        locals.var_mueph_rv = 0.0;

        locals.var_dl = 0.0;
        locals.var_dl_rv = 0.0;

        locals.var_dlld = 0.0;
        locals.var_dlld_rv = 0.0;

        locals.var_lg = 0.0;
        locals.var_lg_rv = 0.0;

        locals.var_dw = 0.0;
        locals.var_dw_rv = 0.0;

        locals.var_dwld = 0.0;
        locals.var_dwld_rv = 0.0;

        locals.var_dwcv = 0.0;
        locals.var_dwcv_rv = 0.0;

        locals.var_wg = 0.0;
        locals.var_wg_rv = 0.0;

        locals.var_wlg = 0.0;
        locals.var_wlg_rv = 0.0;

        locals.var_lgate = 0.0;
        locals.var_lgate_rv = 0.0;

        locals.var_wgate = 0.0;
        locals.var_wgate_rv = 0.0;

        locals.var_nsubpp = 0.0;
        locals.var_nsubpp_dn0 = 0.0;
        locals.var_nsubpp_dn2 = 0.0;
        locals.var_nsubpp_dn4 = 0.0;
        locals.var_nsubpp_dn5 = 0.0;
        locals.var_nsubpp_dn6 = 0.0;
        locals.var_nsubpp_dn7 = 0.0;
        locals.var_nsubpp_dn8 = 0.0;
        locals.var_nsubpp_dn9 = 0.0;
        locals.var_nsubpp_dn10 = 0.0;
        locals.var_nsubpp_dn13 = 0.0;
        locals.var_nsubpp_rv = 0.0;

        locals.var_nsubps = 0.0;
        locals.var_nsubps_dn0 = 0.0;
        locals.var_nsubps_dn2 = 0.0;
        locals.var_nsubps_dn4 = 0.0;
        locals.var_nsubps_dn5 = 0.0;
        locals.var_nsubps_dn6 = 0.0;
        locals.var_nsubps_dn7 = 0.0;
        locals.var_nsubps_dn8 = 0.0;
        locals.var_nsubps_dn9 = 0.0;
        locals.var_nsubps_dn10 = 0.0;
        locals.var_nsubps_dn13 = 0.0;
        locals.var_nsubps_rv = 0.0;

        locals.var_nsub = 0.0;
        locals.var_nsub_dn0 = 0.0;
        locals.var_nsub_dn2 = 0.0;
        locals.var_nsub_dn4 = 0.0;
        locals.var_nsub_dn5 = 0.0;
        locals.var_nsub_dn6 = 0.0;
        locals.var_nsub_dn7 = 0.0;
        locals.var_nsub_dn8 = 0.0;
        locals.var_nsub_dn9 = 0.0;
        locals.var_nsub_dn10 = 0.0;
        locals.var_nsub_dn13 = 0.0;
        locals.var_nsub_rv = 0.0;

        locals.var_nsubb = 0.0;
        locals.var_nsubb_dn0 = 0.0;
        locals.var_nsubb_dn2 = 0.0;
        locals.var_nsubb_dn4 = 0.0;
        locals.var_nsubb_dn5 = 0.0;
        locals.var_nsubb_dn6 = 0.0;
        locals.var_nsubb_dn7 = 0.0;
        locals.var_nsubb_dn8 = 0.0;
        locals.var_nsubb_dn9 = 0.0;
        locals.var_nsubb_dn10 = 0.0;
        locals.var_nsubb_dn13 = 0.0;
        locals.var_nsubb_rv = 0.0;

        locals.var_lod_half = 0.0;
        locals.var_lod_half_dn0 = 0.0;
        locals.var_lod_half_dn2 = 0.0;
        locals.var_lod_half_dn4 = 0.0;
        locals.var_lod_half_dn5 = 0.0;
        locals.var_lod_half_dn6 = 0.0;
        locals.var_lod_half_dn7 = 0.0;
        locals.var_lod_half_dn8 = 0.0;
        locals.var_lod_half_dn9 = 0.0;
        locals.var_lod_half_dn10 = 0.0;
        locals.var_lod_half_dn13 = 0.0;
        locals.var_lod_half_rv = 0.0;

        locals.var_lod_half_ref = 0.0;
        locals.var_lod_half_ref_dn0 = 0.0;
        locals.var_lod_half_ref_dn2 = 0.0;
        locals.var_lod_half_ref_dn4 = 0.0;
        locals.var_lod_half_ref_dn5 = 0.0;
        locals.var_lod_half_ref_dn6 = 0.0;
        locals.var_lod_half_ref_dn7 = 0.0;
        locals.var_lod_half_ref_dn8 = 0.0;
        locals.var_lod_half_ref_dn9 = 0.0;
        locals.var_lod_half_ref_dn10 = 0.0;
        locals.var_lod_half_ref_dn13 = 0.0;
        locals.var_lod_half_ref_rv = 0.0;

        locals.var_log_tratio = 0.0;
        locals.var_log_tratio_dn0 = 0.0;
        locals.var_log_tratio_dn2 = 0.0;
        locals.var_log_tratio_dn4 = 0.0;
        locals.var_log_tratio_dn5 = 0.0;
        locals.var_log_tratio_dn6 = 0.0;
        locals.var_log_tratio_dn7 = 0.0;
        locals.var_log_tratio_dn8 = 0.0;
        locals.var_log_tratio_dn9 = 0.0;
        locals.var_log_tratio_dn10 = 0.0;
        locals.var_log_tratio_dn13 = 0.0;
        locals.var_log_tratio_rv = 0.0;

        locals.var_edri = 0.0;
        locals.var_edri_dn0 = 0.0;
        locals.var_edri_dn2 = 0.0;
        locals.var_edri_dn4 = 0.0;
        locals.var_edri_dn5 = 0.0;
        locals.var_edri_dn6 = 0.0;
        locals.var_edri_dn7 = 0.0;
        locals.var_edri_dn8 = 0.0;
        locals.var_edri_dn9 = 0.0;
        locals.var_edri_dn10 = 0.0;
        locals.var_edri_dn13 = 0.0;
        locals.var_edri_rv = 0.0;

        locals.var_vdri = 0.0;
        locals.var_vdri_dn0 = 0.0;
        locals.var_vdri_dn2 = 0.0;
        locals.var_vdri_dn4 = 0.0;
        locals.var_vdri_dn5 = 0.0;
        locals.var_vdri_dn6 = 0.0;
        locals.var_vdri_dn7 = 0.0;
        locals.var_vdri_dn8 = 0.0;
        locals.var_vdri_dn9 = 0.0;
        locals.var_vdri_dn10 = 0.0;
        locals.var_vdri_dn13 = 0.0;
        locals.var_vdri_rv = 0.0;

        locals.var_mu0 = 0.0;
        locals.var_mu0_dn0 = 0.0;
        locals.var_mu0_dn2 = 0.0;
        locals.var_mu0_dn4 = 0.0;
        locals.var_mu0_dn5 = 0.0;
        locals.var_mu0_dn6 = 0.0;
        locals.var_mu0_dn7 = 0.0;
        locals.var_mu0_dn8 = 0.0;
        locals.var_mu0_dn9 = 0.0;
        locals.var_mu0_dn10 = 0.0;
        locals.var_mu0_dn13 = 0.0;
        locals.var_mu0_rv = 0.0;

        locals.var_cx = 0.0;
        locals.var_cx_rv = 0.0;

        locals.var_car = 0.0;
        locals.var_car_rv = 0.0;

        locals.var_xov = 0.0;
        locals.var_xov_dn0 = 0.0;
        locals.var_xov_dn2 = 0.0;
        locals.var_xov_dn4 = 0.0;
        locals.var_xov_dn5 = 0.0;
        locals.var_xov_dn6 = 0.0;
        locals.var_xov_dn7 = 0.0;
        locals.var_xov_dn8 = 0.0;
        locals.var_xov_dn9 = 0.0;
        locals.var_xov_dn10 = 0.0;
        locals.var_xov_dn13 = 0.0;
        locals.var_xov_rv = 0.0;

        locals.var_carr = 0.0;
        locals.var_carr_dn0 = 0.0;
        locals.var_carr_dn2 = 0.0;
        locals.var_carr_dn4 = 0.0;
        locals.var_carr_dn5 = 0.0;
        locals.var_carr_dn6 = 0.0;
        locals.var_carr_dn7 = 0.0;
        locals.var_carr_dn8 = 0.0;
        locals.var_carr_dn9 = 0.0;
        locals.var_carr_dn10 = 0.0;
        locals.var_carr_dn13 = 0.0;
        locals.var_carr_rv = 0.0;

        locals.var_gd = 0.0;
        locals.var_gd_dn0 = 0.0;
        locals.var_gd_dn2 = 0.0;
        locals.var_gd_dn4 = 0.0;
        locals.var_gd_dn5 = 0.0;
        locals.var_gd_dn6 = 0.0;
        locals.var_gd_dn7 = 0.0;
        locals.var_gd_dn8 = 0.0;
        locals.var_gd_dn9 = 0.0;
        locals.var_gd_dn10 = 0.0;
        locals.var_gd_dn13 = 0.0;
        locals.var_gd_rv = 0.0;

        locals.var_vddpz = 0.0;
        locals.var_vddpz_dn0 = 0.0;
        locals.var_vddpz_dn2 = 0.0;
        locals.var_vddpz_dn4 = 0.0;
        locals.var_vddpz_dn5 = 0.0;
        locals.var_vddpz_dn6 = 0.0;
        locals.var_vddpz_dn7 = 0.0;
        locals.var_vddpz_dn8 = 0.0;
        locals.var_vddpz_dn9 = 0.0;
        locals.var_vddpz_dn10 = 0.0;
        locals.var_vddpz_dn13 = 0.0;
        locals.var_vddpz_rv = 0.0;

        locals.var_arg = 0.0;
        locals.var_arg_dn0 = 0.0;
        locals.var_arg_dn2 = 0.0;
        locals.var_arg_dn4 = 0.0;
        locals.var_arg_dn5 = 0.0;
        locals.var_arg_dn6 = 0.0;
        locals.var_arg_dn7 = 0.0;
        locals.var_arg_dn8 = 0.0;
        locals.var_arg_dn9 = 0.0;
        locals.var_arg_dn10 = 0.0;
        locals.var_arg_dn13 = 0.0;
        locals.var_arg_rv = 0.0;

        locals.var_vbd = 0.0;
        locals.var_vbd_dn5 = 0.0;
        locals.var_vbd_dn7 = 0.0;
        locals.var_vbd_dn8 = 0.0;
        locals.var_vbd_rv = 0.0;

        locals.var_vbsi = 0.0;
        locals.var_vbsi_dn7 = 0.0;
        locals.var_vbsi_dn8 = 0.0;
        locals.var_vbsi_rv = 0.0;

        locals.var_vdsi = 0.0;
        locals.var_vdsi_dn5 = 0.0;
        locals.var_vdsi_dn7 = 0.0;
        locals.var_vdsi_rv = 0.0;

        locals.var_vgd = 0.0;
        locals.var_vgd_dn5 = 0.0;
        locals.var_vgd_dn6 = 0.0;
        locals.var_vgd_dn7 = 0.0;
        locals.var_vgd_rv = 0.0;

        locals.var_vgsi = 0.0;
        locals.var_vgsi_dn6 = 0.0;
        locals.var_vgsi_dn7 = 0.0;
        locals.var_vgsi_rv = 0.0;

        locals.var_deltemp = 0.0;
        locals.var_deltemp_dn0 = 0.0;
        locals.var_deltemp_dn2 = 0.0;
        locals.var_deltemp_dn4 = 0.0;
        locals.var_deltemp_dn5 = 0.0;
        locals.var_deltemp_dn6 = 0.0;
        locals.var_deltemp_dn7 = 0.0;
        locals.var_deltemp_dn8 = 0.0;
        locals.var_deltemp_dn9 = 0.0;
        locals.var_deltemp_dn10 = 0.0;
        locals.var_deltemp_dn13 = 0.0;
        locals.var_deltemp_rv = 0.0;

        locals.var_vdsei = 0.0;
        locals.var_vdsei_dn0 = 0.0;
        locals.var_vdsei_dn2 = 0.0;
        locals.var_vdsei_rv = 0.0;

        locals.var_vgsei = 0.0;
        locals.var_vgsei_dn2 = 0.0;
        locals.var_vgsei_dn6 = 0.0;
        locals.var_vgsei_rv = 0.0;

        locals.var_vbsei = 0.0;
        locals.var_vbsei_dn2 = 0.0;
        locals.var_vbsei_dn8 = 0.0;
        locals.var_vbsei_rv = 0.0;

        locals.var_gth = 0.0;
        locals.var_gth_dn0 = 0.0;
        locals.var_gth_dn2 = 0.0;
        locals.var_gth_dn4 = 0.0;
        locals.var_gth_dn5 = 0.0;
        locals.var_gth_dn6 = 0.0;
        locals.var_gth_dn7 = 0.0;
        locals.var_gth_dn8 = 0.0;
        locals.var_gth_dn9 = 0.0;
        locals.var_gth_dn10 = 0.0;
        locals.var_gth_dn13 = 0.0;
        locals.var_gth_rv = 0.0;

        locals.var_qg = 0.0;
        locals.var_qg_dn0 = 0.0;
        locals.var_qg_dn2 = 0.0;
        locals.var_qg_dn4 = 0.0;
        locals.var_qg_dn5 = 0.0;
        locals.var_qg_dn6 = 0.0;
        locals.var_qg_dn7 = 0.0;
        locals.var_qg_dn8 = 0.0;
        locals.var_qg_dn9 = 0.0;
        locals.var_qg_dn10 = 0.0;
        locals.var_qg_dn13 = 0.0;
        locals.var_qg_rv = 0.0;

        locals.var_qs = 0.0;
        locals.var_qs_dn0 = 0.0;
        locals.var_qs_dn2 = 0.0;
        locals.var_qs_dn4 = 0.0;
        locals.var_qs_dn5 = 0.0;
        locals.var_qs_dn6 = 0.0;
        locals.var_qs_dn7 = 0.0;
        locals.var_qs_dn8 = 0.0;
        locals.var_qs_dn9 = 0.0;
        locals.var_qs_dn10 = 0.0;
        locals.var_qs_dn13 = 0.0;
        locals.var_qs_rv = 0.0;

        locals.var_veffpower = 0.0;
        locals.var_veffpower_dn0 = 0.0;
        locals.var_veffpower_dn2 = 0.0;
        locals.var_veffpower_dn4 = 0.0;
        locals.var_veffpower_dn5 = 0.0;
        locals.var_veffpower_dn6 = 0.0;
        locals.var_veffpower_dn7 = 0.0;
        locals.var_veffpower_dn8 = 0.0;
        locals.var_veffpower_dn9 = 0.0;
        locals.var_veffpower_dn10 = 0.0;
        locals.var_veffpower_dn13 = 0.0;
        locals.var_veffpower_rv = 0.0;

        locals.var_p = 0.0;
        locals.var_p_dn0 = 0.0;
        locals.var_p_dn2 = 0.0;
        locals.var_p_dn4 = 0.0;
        locals.var_p_dn5 = 0.0;
        locals.var_p_dn6 = 0.0;
        locals.var_p_dn7 = 0.0;
        locals.var_p_dn8 = 0.0;
        locals.var_p_dn9 = 0.0;
        locals.var_p_dn10 = 0.0;
        locals.var_p_dn13 = 0.0;
        locals.var_p_rv = 0.0;

        locals.var_qi_nqs = 0.0;
        locals.var_qi_nqs_dn11 = 0.0;
        locals.var_qi_nqs_rv = 0.0;

        locals.var_qb_nqs = 0.0;
        locals.var_qb_nqs_dn12 = 0.0;
        locals.var_qb_nqs_rv = 0.0;

        locals.var_qd_nqs = 0.0;
        locals.var_qd_nqs_dn0 = 0.0;
        locals.var_qd_nqs_dn2 = 0.0;
        locals.var_qd_nqs_dn4 = 0.0;
        locals.var_qd_nqs_dn5 = 0.0;
        locals.var_qd_nqs_dn6 = 0.0;
        locals.var_qd_nqs_dn7 = 0.0;
        locals.var_qd_nqs_dn8 = 0.0;
        locals.var_qd_nqs_dn9 = 0.0;
        locals.var_qd_nqs_dn10 = 0.0;
        locals.var_qd_nqs_dn11 = 0.0;
        locals.var_qd_nqs_dn13 = 0.0;
        locals.var_qd_nqs_rv = 0.0;

        locals.var_qs_nqs = 0.0;
        locals.var_qs_nqs_dn0 = 0.0;
        locals.var_qs_nqs_dn2 = 0.0;
        locals.var_qs_nqs_dn4 = 0.0;
        locals.var_qs_nqs_dn5 = 0.0;
        locals.var_qs_nqs_dn6 = 0.0;
        locals.var_qs_nqs_dn7 = 0.0;
        locals.var_qs_nqs_dn8 = 0.0;
        locals.var_qs_nqs_dn9 = 0.0;
        locals.var_qs_nqs_dn10 = 0.0;
        locals.var_qs_nqs_dn11 = 0.0;
        locals.var_qs_nqs_dn13 = 0.0;
        locals.var_qs_nqs_rv = 0.0;

        locals.var_qg_nqs = 0.0;
        locals.var_qg_nqs_dn11 = 0.0;
        locals.var_qg_nqs_dn12 = 0.0;
        locals.var_qg_nqs_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_8(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        locals.var_cgsb = 0.0;
        locals.var_cgsb_dn0 = 0.0;
        locals.var_cgsb_dn2 = 0.0;
        locals.var_cgsb_dn4 = 0.0;
        locals.var_cgsb_dn5 = 0.0;
        locals.var_cgsb_dn6 = 0.0;
        locals.var_cgsb_dn7 = 0.0;
        locals.var_cgsb_dn8 = 0.0;
        locals.var_cgsb_dn9 = 0.0;
        locals.var_cgsb_dn10 = 0.0;
        locals.var_cgsb_dn13 = 0.0;
        locals.var_cgsb_rv = 0.0;

        locals.var_ninvde = 0.0;
        locals.var_ninvde_dn0 = 0.0;
        locals.var_ninvde_dn2 = 0.0;
        locals.var_ninvde_dn4 = 0.0;
        locals.var_ninvde_dn5 = 0.0;
        locals.var_ninvde_dn6 = 0.0;
        locals.var_ninvde_dn7 = 0.0;
        locals.var_ninvde_dn8 = 0.0;
        locals.var_ninvde_dn9 = 0.0;
        locals.var_ninvde_dn10 = 0.0;
        locals.var_ninvde_dn13 = 0.0;
        locals.var_ninvde_rv = 0.0;

        locals.var_ninvdecres = 0.0;
        locals.var_ninvdecres_dn0 = 0.0;
        locals.var_ninvdecres_dn2 = 0.0;
        locals.var_ninvdecres_dn4 = 0.0;
        locals.var_ninvdecres_dn5 = 0.0;
        locals.var_ninvdecres_dn6 = 0.0;
        locals.var_ninvdecres_dn7 = 0.0;
        locals.var_ninvdecres_dn8 = 0.0;
        locals.var_ninvdecres_dn9 = 0.0;
        locals.var_ninvdecres_dn10 = 0.0;
        locals.var_ninvdecres_dn13 = 0.0;
        locals.var_ninvdecres_rv = 0.0;

        locals.var_ninvdehres = 0.0;
        locals.var_ninvdehres_dn0 = 0.0;
        locals.var_ninvdehres_dn2 = 0.0;
        locals.var_ninvdehres_dn4 = 0.0;
        locals.var_ninvdehres_dn5 = 0.0;
        locals.var_ninvdehres_dn6 = 0.0;
        locals.var_ninvdehres_dn7 = 0.0;
        locals.var_ninvdehres_dn8 = 0.0;
        locals.var_ninvdehres_dn9 = 0.0;
        locals.var_ninvdehres_dn10 = 0.0;
        locals.var_ninvdehres_dn13 = 0.0;
        locals.var_ninvdehres_rv = 0.0;

        locals.var_rrdrmue = 0.0;
        locals.var_rrdrmue_dn0 = 0.0;
        locals.var_rrdrmue_dn2 = 0.0;
        locals.var_rrdrmue_dn4 = 0.0;
        locals.var_rrdrmue_dn5 = 0.0;
        locals.var_rrdrmue_dn6 = 0.0;
        locals.var_rrdrmue_dn7 = 0.0;
        locals.var_rrdrmue_dn8 = 0.0;
        locals.var_rrdrmue_dn9 = 0.0;
        locals.var_rrdrmue_dn10 = 0.0;
        locals.var_rrdrmue_dn13 = 0.0;
        locals.var_rrdrmue_rv = 0.0;

        locals.var_rrdrmues = 0.0;
        locals.var_rrdrmues_dn0 = 0.0;
        locals.var_rrdrmues_dn2 = 0.0;
        locals.var_rrdrmues_dn4 = 0.0;
        locals.var_rrdrmues_dn5 = 0.0;
        locals.var_rrdrmues_dn6 = 0.0;
        locals.var_rrdrmues_dn7 = 0.0;
        locals.var_rrdrmues_dn8 = 0.0;
        locals.var_rrdrmues_dn9 = 0.0;
        locals.var_rrdrmues_dn10 = 0.0;
        locals.var_rrdrmues_dn13 = 0.0;
        locals.var_rrdrmues_rv = 0.0;

        locals.var_rrdrvmax = 0.0;
        locals.var_rrdrvmax_dn0 = 0.0;
        locals.var_rrdrvmax_dn2 = 0.0;
        locals.var_rrdrvmax_dn4 = 0.0;
        locals.var_rrdrvmax_dn5 = 0.0;
        locals.var_rrdrvmax_dn6 = 0.0;
        locals.var_rrdrvmax_dn7 = 0.0;
        locals.var_rrdrvmax_dn8 = 0.0;
        locals.var_rrdrvmax_dn9 = 0.0;
        locals.var_rrdrvmax_dn10 = 0.0;
        locals.var_rrdrvmax_dn13 = 0.0;
        locals.var_rrdrvmax_rv = 0.0;

        locals.var_rde = 0.0;
        locals.var_rde_dn0 = 0.0;
        locals.var_rde_dn2 = 0.0;
        locals.var_rde_dn4 = 0.0;
        locals.var_rde_dn5 = 0.0;
        locals.var_rde_dn6 = 0.0;
        locals.var_rde_dn7 = 0.0;
        locals.var_rde_dn8 = 0.0;
        locals.var_rde_dn9 = 0.0;
        locals.var_rde_dn10 = 0.0;
        locals.var_rde_dn13 = 0.0;
        locals.var_rde_rv = 0.0;

        locals.var_rdvde = 0.0;
        locals.var_rdvde_dn0 = 0.0;
        locals.var_rdvde_dn2 = 0.0;
        locals.var_rdvde_dn4 = 0.0;
        locals.var_rdvde_dn5 = 0.0;
        locals.var_rdvde_dn6 = 0.0;
        locals.var_rdvde_dn7 = 0.0;
        locals.var_rdvde_dn8 = 0.0;
        locals.var_rdvde_dn9 = 0.0;
        locals.var_rdvde_dn10 = 0.0;
        locals.var_rdvde_dn13 = 0.0;
        locals.var_rdvde_rv = 0.0;

        locals.var_rse = 0.0;
        locals.var_rse_dn0 = 0.0;
        locals.var_rse_dn2 = 0.0;
        locals.var_rse_dn4 = 0.0;
        locals.var_rse_dn5 = 0.0;
        locals.var_rse_dn6 = 0.0;
        locals.var_rse_dn7 = 0.0;
        locals.var_rse_dn8 = 0.0;
        locals.var_rse_dn9 = 0.0;
        locals.var_rse_dn10 = 0.0;
        locals.var_rse_dn13 = 0.0;
        locals.var_rse_rv = 0.0;

        locals.var_rsvde = 0.0;
        locals.var_rsvde_dn0 = 0.0;
        locals.var_rsvde_dn2 = 0.0;
        locals.var_rsvde_dn4 = 0.0;
        locals.var_rsvde_dn5 = 0.0;
        locals.var_rsvde_dn6 = 0.0;
        locals.var_rsvde_dn7 = 0.0;
        locals.var_rsvde_dn8 = 0.0;
        locals.var_rsvde_dn9 = 0.0;
        locals.var_rsvde_dn10 = 0.0;
        locals.var_rsvde_dn13 = 0.0;
        locals.var_rsvde_rv = 0.0;

        locals.var_rrdrvmaxs = 0.0;
        locals.var_rrdrvmaxs_dn0 = 0.0;
        locals.var_rrdrvmaxs_dn2 = 0.0;
        locals.var_rrdrvmaxs_dn4 = 0.0;
        locals.var_rrdrvmaxs_dn5 = 0.0;
        locals.var_rrdrvmaxs_dn6 = 0.0;
        locals.var_rrdrvmaxs_dn7 = 0.0;
        locals.var_rrdrvmaxs_dn8 = 0.0;
        locals.var_rrdrvmaxs_dn9 = 0.0;
        locals.var_rrdrvmaxs_dn10 = 0.0;
        locals.var_rrdrvmaxs_dn13 = 0.0;
        locals.var_rrdrvmaxs_rv = 0.0;

        locals.var_tratio = 0.0;
        locals.var_tratio_dn0 = 0.0;
        locals.var_tratio_dn2 = 0.0;
        locals.var_tratio_dn4 = 0.0;
        locals.var_tratio_dn5 = 0.0;
        locals.var_tratio_dn6 = 0.0;
        locals.var_tratio_dn7 = 0.0;
        locals.var_tratio_dn8 = 0.0;
        locals.var_tratio_dn9 = 0.0;
        locals.var_tratio_dn10 = 0.0;
        locals.var_tratio_dn13 = 0.0;
        locals.var_tratio_rv = 0.0;

        locals.var_vmaxeff = 0.0;
        locals.var_vmaxeff_dn0 = 0.0;
        locals.var_vmaxeff_dn2 = 0.0;
        locals.var_vmaxeff_dn4 = 0.0;
        locals.var_vmaxeff_dn5 = 0.0;
        locals.var_vmaxeff_dn6 = 0.0;
        locals.var_vmaxeff_dn7 = 0.0;
        locals.var_vmaxeff_dn8 = 0.0;
        locals.var_vmaxeff_dn9 = 0.0;
        locals.var_vmaxeff_dn10 = 0.0;
        locals.var_vmaxeff_dn13 = 0.0;
        locals.var_vmaxeff_rv = 0.0;

        locals.var_betatnom = 0.0;
        locals.var_betatnom_rv = 0.0;

        locals.var_cnst0over = 0.0;
        locals.var_cnst0over_dn0 = 0.0;
        locals.var_cnst0over_dn2 = 0.0;
        locals.var_cnst0over_dn4 = 0.0;
        locals.var_cnst0over_dn5 = 0.0;
        locals.var_cnst0over_dn6 = 0.0;
        locals.var_cnst0over_dn7 = 0.0;
        locals.var_cnst0over_dn8 = 0.0;
        locals.var_cnst0over_dn9 = 0.0;
        locals.var_cnst0over_dn10 = 0.0;
        locals.var_cnst0over_dn13 = 0.0;
        locals.var_cnst0over_rv = 0.0;

        locals.var_cnst0overs = 0.0;
        locals.var_cnst0overs_dn0 = 0.0;
        locals.var_cnst0overs_dn2 = 0.0;
        locals.var_cnst0overs_dn4 = 0.0;
        locals.var_cnst0overs_dn5 = 0.0;
        locals.var_cnst0overs_dn6 = 0.0;
        locals.var_cnst0overs_dn7 = 0.0;
        locals.var_cnst0overs_dn8 = 0.0;
        locals.var_cnst0overs_dn9 = 0.0;
        locals.var_cnst0overs_dn10 = 0.0;
        locals.var_cnst0overs_dn13 = 0.0;
        locals.var_cnst0overs_rv = 0.0;

        locals.var_costi0_p2 = 0.0;
        locals.var_costi0_p2_dn0 = 0.0;
        locals.var_costi0_p2_dn2 = 0.0;
        locals.var_costi0_p2_dn4 = 0.0;
        locals.var_costi0_p2_dn5 = 0.0;
        locals.var_costi0_p2_dn6 = 0.0;
        locals.var_costi0_p2_dn7 = 0.0;
        locals.var_costi0_p2_dn8 = 0.0;
        locals.var_costi0_p2_dn9 = 0.0;
        locals.var_costi0_p2_dn10 = 0.0;
        locals.var_costi0_p2_dn13 = 0.0;
        locals.var_costi0_p2_rv = 0.0;

        locals.var_mphn0 = 0.0;
        locals.var_mphn0_dn0 = 0.0;
        locals.var_mphn0_dn2 = 0.0;
        locals.var_mphn0_dn4 = 0.0;
        locals.var_mphn0_dn5 = 0.0;
        locals.var_mphn0_dn6 = 0.0;
        locals.var_mphn0_dn7 = 0.0;
        locals.var_mphn0_dn8 = 0.0;
        locals.var_mphn0_dn9 = 0.0;
        locals.var_mphn0_dn10 = 0.0;
        locals.var_mphn0_dn13 = 0.0;
        locals.var_mphn0_rv = 0.0;

        locals.var_powratio = 0.0;
        locals.var_powratio_dn0 = 0.0;
        locals.var_powratio_dn2 = 0.0;
        locals.var_powratio_dn4 = 0.0;
        locals.var_powratio_dn5 = 0.0;
        locals.var_powratio_dn6 = 0.0;
        locals.var_powratio_dn7 = 0.0;
        locals.var_powratio_dn8 = 0.0;
        locals.var_powratio_dn9 = 0.0;
        locals.var_powratio_dn10 = 0.0;
        locals.var_powratio_dn13 = 0.0;
        locals.var_powratio_rv = 0.0;

        locals.var_ptovr = 0.0;
        locals.var_ptovr_dn0 = 0.0;
        locals.var_ptovr_dn2 = 0.0;
        locals.var_ptovr_dn4 = 0.0;
        locals.var_ptovr_dn5 = 0.0;
        locals.var_ptovr_dn6 = 0.0;
        locals.var_ptovr_dn7 = 0.0;
        locals.var_ptovr_dn8 = 0.0;
        locals.var_ptovr_dn9 = 0.0;
        locals.var_ptovr_dn10 = 0.0;
        locals.var_ptovr_dn13 = 0.0;
        locals.var_ptovr_rv = 0.0;

        locals.var_sqrt_eg = 0.0;
        locals.var_sqrt_eg_dn0 = 0.0;
        locals.var_sqrt_eg_dn2 = 0.0;
        locals.var_sqrt_eg_dn4 = 0.0;
        locals.var_sqrt_eg_dn5 = 0.0;
        locals.var_sqrt_eg_dn6 = 0.0;
        locals.var_sqrt_eg_dn7 = 0.0;
        locals.var_sqrt_eg_dn8 = 0.0;
        locals.var_sqrt_eg_dn9 = 0.0;
        locals.var_sqrt_eg_dn10 = 0.0;
        locals.var_sqrt_eg_dn13 = 0.0;
        locals.var_sqrt_eg_rv = 0.0;

        locals.var_wdpl = 0.0;
        locals.var_wdpl_dn0 = 0.0;
        locals.var_wdpl_dn2 = 0.0;
        locals.var_wdpl_dn4 = 0.0;
        locals.var_wdpl_dn5 = 0.0;
        locals.var_wdpl_dn6 = 0.0;
        locals.var_wdpl_dn7 = 0.0;
        locals.var_wdpl_dn8 = 0.0;
        locals.var_wdpl_dn9 = 0.0;
        locals.var_wdpl_dn10 = 0.0;
        locals.var_wdpl_dn13 = 0.0;
        locals.var_wdpl_rv = 0.0;

        locals.var_wdplp = 0.0;
        locals.var_wdplp_dn0 = 0.0;
        locals.var_wdplp_dn2 = 0.0;
        locals.var_wdplp_dn4 = 0.0;
        locals.var_wdplp_dn5 = 0.0;
        locals.var_wdplp_dn6 = 0.0;
        locals.var_wdplp_dn7 = 0.0;
        locals.var_wdplp_dn8 = 0.0;
        locals.var_wdplp_dn9 = 0.0;
        locals.var_wdplp_dn10 = 0.0;
        locals.var_wdplp_dn13 = 0.0;
        locals.var_wdplp_rv = 0.0;

        locals.var_uc_rdrbb = p.p436;
        locals.var_uc_rdrbb_dn0 = 0.0;
        locals.var_uc_rdrbb_dn2 = 0.0;
        locals.var_uc_rdrbb_dn4 = 0.0;
        locals.var_uc_rdrbb_dn5 = 0.0;
        locals.var_uc_rdrbb_dn6 = 0.0;
        locals.var_uc_rdrbb_dn7 = 0.0;
        locals.var_uc_rdrbb_dn8 = 0.0;
        locals.var_uc_rdrbb_dn9 = 0.0;
        locals.var_uc_rdrbb_dn10 = 0.0;
        locals.var_uc_rdrbb_dn13 = 0.0;
        locals.var_uc_rdrbb_rv = 0.0;

        locals.var_uc_rdrbb_s = p.p437;
        locals.var_uc_rdrbb_s_dn0 = 0.0;
        locals.var_uc_rdrbb_s_dn2 = 0.0;
        locals.var_uc_rdrbb_s_dn4 = 0.0;
        locals.var_uc_rdrbb_s_dn5 = 0.0;
        locals.var_uc_rdrbb_s_dn6 = 0.0;
        locals.var_uc_rdrbb_s_dn7 = 0.0;
        locals.var_uc_rdrbb_s_dn8 = 0.0;
        locals.var_uc_rdrbb_s_dn9 = 0.0;
        locals.var_uc_rdrbb_s_dn10 = 0.0;
        locals.var_uc_rdrbb_s_dn13 = 0.0;
        locals.var_uc_rdrbb_s_rv = 0.0;

        locals.var_ids_acc = 0.0;
        locals.var_ids_acc_dn0 = 0.0;
        locals.var_ids_acc_dn2 = 0.0;
        locals.var_ids_acc_dn4 = 0.0;
        locals.var_ids_acc_dn5 = 0.0;
        locals.var_ids_acc_dn6 = 0.0;
        locals.var_ids_acc_dn7 = 0.0;
        locals.var_ids_acc_dn8 = 0.0;
        locals.var_ids_acc_dn9 = 0.0;
        locals.var_ids_acc_dn10 = 0.0;
        locals.var_ids_acc_dn13 = 0.0;
        locals.var_ids_acc_rv = 0.0;

        locals.var_ids_res = 0.0;
        locals.var_ids_res_dn0 = 0.0;
        locals.var_ids_res_dn2 = 0.0;
        locals.var_ids_res_dn4 = 0.0;
        locals.var_ids_res_dn5 = 0.0;
        locals.var_ids_res_dn6 = 0.0;
        locals.var_ids_res_dn7 = 0.0;
        locals.var_ids_res_dn8 = 0.0;
        locals.var_ids_res_dn9 = 0.0;
        locals.var_ids_res_dn10 = 0.0;
        locals.var_ids_res_dn13 = 0.0;
        locals.var_ids_res_rv = 0.0;

        locals.var_ires_leak = 0.0;
        locals.var_ires_leak_dn0 = 0.0;
        locals.var_ires_leak_dn2 = 0.0;
        locals.var_ires_leak_dn4 = 0.0;
        locals.var_ires_leak_dn5 = 0.0;
        locals.var_ires_leak_dn6 = 0.0;
        locals.var_ires_leak_dn7 = 0.0;
        locals.var_ires_leak_dn8 = 0.0;
        locals.var_ires_leak_dn9 = 0.0;
        locals.var_ires_leak_dn10 = 0.0;
        locals.var_ires_leak_dn13 = 0.0;
        locals.var_ires_leak_rv = 0.0;

        locals.var_pb2n = 0.0;
        locals.var_pb2n_dn0 = 0.0;
        locals.var_pb2n_dn2 = 0.0;
        locals.var_pb2n_dn4 = 0.0;
        locals.var_pb2n_dn5 = 0.0;
        locals.var_pb2n_dn6 = 0.0;
        locals.var_pb2n_dn7 = 0.0;
        locals.var_pb2n_dn8 = 0.0;
        locals.var_pb2n_dn9 = 0.0;
        locals.var_pb2n_dn10 = 0.0;
        locals.var_pb2n_dn13 = 0.0;
        locals.var_pb2n_rv = 0.0;

        locals.var_vbipn = 0.0;
        locals.var_vbipn_dn0 = 0.0;
        locals.var_vbipn_dn2 = 0.0;
        locals.var_vbipn_dn4 = 0.0;
        locals.var_vbipn_dn5 = 0.0;
        locals.var_vbipn_dn6 = 0.0;
        locals.var_vbipn_dn7 = 0.0;
        locals.var_vbipn_dn8 = 0.0;
        locals.var_vbipn_dn9 = 0.0;
        locals.var_vbipn_dn10 = 0.0;
        locals.var_vbipn_dn13 = 0.0;
        locals.var_vbipn_rv = 0.0;

        locals.var_hbdceff = p.p447;
        locals.var_hbdceff_dn0 = 0.0;
        locals.var_hbdceff_dn2 = 0.0;
        locals.var_hbdceff_dn4 = 0.0;
        locals.var_hbdceff_dn5 = 0.0;
        locals.var_hbdceff_dn6 = 0.0;
        locals.var_hbdceff_dn7 = 0.0;
        locals.var_hbdceff_dn8 = 0.0;
        locals.var_hbdceff_dn9 = 0.0;
        locals.var_hbdceff_dn10 = 0.0;
        locals.var_hbdceff_dn13 = 0.0;
        locals.var_hbdceff_rv = 0.0;

        locals.var_uc_subtmp = p.p193;
        locals.var_uc_subtmp_rv = 0.0;

        locals.var_depmphn0 = 0.0;
        locals.var_depmphn0_dn0 = 0.0;
        locals.var_depmphn0_dn2 = 0.0;
        locals.var_depmphn0_dn4 = 0.0;
        locals.var_depmphn0_dn5 = 0.0;
        locals.var_depmphn0_dn6 = 0.0;
        locals.var_depmphn0_dn7 = 0.0;
        locals.var_depmphn0_dn8 = 0.0;
        locals.var_depmphn0_dn9 = 0.0;
        locals.var_depmphn0_dn10 = 0.0;
        locals.var_depmphn0_dn13 = 0.0;
        locals.var_depmphn0_rv = 0.0;

        locals.var_qiu_noi = 0.0;
        locals.var_qiu_noi_dn0 = 0.0;
        locals.var_qiu_noi_dn2 = 0.0;
        locals.var_qiu_noi_dn4 = 0.0;
        locals.var_qiu_noi_dn5 = 0.0;
        locals.var_qiu_noi_dn6 = 0.0;
        locals.var_qiu_noi_dn7 = 0.0;
        locals.var_qiu_noi_dn8 = 0.0;
        locals.var_qiu_noi_dn9 = 0.0;
        locals.var_qiu_noi_dn10 = 0.0;
        locals.var_qiu_noi_dn13 = 0.0;
        locals.var_qiu_noi_rv = 0.0;

        locals.var_lp_s0_max = 40.0;
        locals.var_lp_s0_max_rv = 0.0;

        locals.var_js = 0.0;
        locals.var_js_dn0 = 0.0;
        locals.var_js_dn2 = 0.0;
        locals.var_js_dn4 = 0.0;
        locals.var_js_dn5 = 0.0;
        locals.var_js_dn6 = 0.0;
        locals.var_js_dn7 = 0.0;
        locals.var_js_dn8 = 0.0;
        locals.var_js_dn9 = 0.0;
        locals.var_js_dn10 = 0.0;
        locals.var_js_dn13 = 0.0;
        locals.var_js_rv = 0.0;

        locals.var_jssw = 0.0;
        locals.var_jssw_dn0 = 0.0;
        locals.var_jssw_dn2 = 0.0;
        locals.var_jssw_dn4 = 0.0;
        locals.var_jssw_dn5 = 0.0;
        locals.var_jssw_dn6 = 0.0;
        locals.var_jssw_dn7 = 0.0;
        locals.var_jssw_dn8 = 0.0;
        locals.var_jssw_dn9 = 0.0;
        locals.var_jssw_dn10 = 0.0;
        locals.var_jssw_dn13 = 0.0;
        locals.var_jssw_rv = 0.0;

        locals.var_js2 = 0.0;
        locals.var_js2_dn0 = 0.0;
        locals.var_js2_dn2 = 0.0;
        locals.var_js2_dn4 = 0.0;
        locals.var_js2_dn5 = 0.0;
        locals.var_js2_dn6 = 0.0;
        locals.var_js2_dn7 = 0.0;
        locals.var_js2_dn8 = 0.0;
        locals.var_js2_dn9 = 0.0;
        locals.var_js2_dn10 = 0.0;
        locals.var_js2_dn13 = 0.0;
        locals.var_js2_rv = 0.0;

        locals.var_jssw2 = 0.0;
        locals.var_jssw2_dn0 = 0.0;
        locals.var_jssw2_dn2 = 0.0;
        locals.var_jssw2_dn4 = 0.0;
        locals.var_jssw2_dn5 = 0.0;
        locals.var_jssw2_dn6 = 0.0;
        locals.var_jssw2_dn7 = 0.0;
        locals.var_jssw2_dn8 = 0.0;
        locals.var_jssw2_dn9 = 0.0;
        locals.var_jssw2_dn10 = 0.0;
        locals.var_jssw2_dn13 = 0.0;
        locals.var_jssw2_rv = 0.0;

        locals.var_qbs = 0.0;
        locals.var_qbs_dn0 = 0.0;
        locals.var_qbs_dn2 = 0.0;
        locals.var_qbs_dn4 = 0.0;
        locals.var_qbs_dn5 = 0.0;
        locals.var_qbs_dn6 = 0.0;
        locals.var_qbs_dn7 = 0.0;
        locals.var_qbs_dn8 = 0.0;
        locals.var_qbs_dn9 = 0.0;
        locals.var_qbs_dn10 = 0.0;
        locals.var_qbs_dn13 = 0.0;
        locals.var_qbs_rv = 0.0;

    }
}
