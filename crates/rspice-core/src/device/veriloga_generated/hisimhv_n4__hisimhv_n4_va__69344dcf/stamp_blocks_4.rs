#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, Scratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_transient_block_64(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((!s.b[1439]) && s.b[2426]) && s.b[2499]) {
            s.store_sqrt_add_scaled_square_input(334, 148, 1.0, 147, (4.0 * 0.002));
            s.store_add_scaled_inputs3_indices(149, 147, 1.0, 148, (-0.5), 334, (-0.5));
            s.store_mul_exp_ad_rhs(334, 140, A::mul(s.ad_value(154), s.ad_value(149)));
            s.store_add_offset_ad_lhs(335, A::mul(s.ad_value(154), A::sub(s.ad_value(149), s.ad_value(1434))), (-1.0), 334);
            s.store_sqrt_square_offset(782, 335, ((4.0 * 0.01) * 0.01));
            s.store_offset_scaled_div(334, 335, 782, 0.5, 0.5);
            s.store_scaled_add(335, 335, 782, 0.5);
        }

        s.b[2501] = (s.v[335] < 0.0);
        s.v[2501] = if s.b[2501] { 1.0 } else { 0.0 };

        if ((((!s.b[1439]) && s.b[2426]) && s.b[2499]) && s.b[2501]) {
            s.store_scalar(335, 0.0);
            s.store_scalar(334, 0.0);
        }

        if (((!s.b[1439]) && s.b[2426]) && s.b[2499]) {
            s.store_offset(335, 335, 1e-25);
            s.store_sqrt(150, 335);
            s.store_offset_mul_ad(335, s.ad_value(154), A::sub(s.ad_value(149), s.ad_value(1434)), (-1.0));
            s.store_sqrt_square_offset(782, 335, ((4.0 * 0.01) * 0.01));
            s.store_offset_scaled_div(334, 335, 782, 0.5, 0.5);
            s.store_scaled_add(335, 335, 782, 0.5);
        }

        s.b[2502] = (s.v[335] < 0.0);
        s.v[2502] = if s.b[2502] { 1.0 } else { 0.0 };

        if ((((!s.b[1439]) && s.b[2426]) && s.b[2499]) && s.b[2502]) {
            s.store_scalar(335, 0.0);
            s.store_scalar(334, 0.0);
        }

        if (((!s.b[1439]) && s.b[2426]) && s.b[2499]) {
            s.store_offset(335, 335, 1e-25);
            s.store_sqrt(151, 335);
            s.store_div_from_scalar(336, 0.5, 151);
            s.store_mul_sub_rhs(152, 139, 150, 151);
            s.store_sub(335, 146, 149);
            s.store_sqrt_square_offset(782, 335, ((4.0 * 0.1) * 0.1));
            s.store_offset_scaled_div(336, 335, 782, 0.5, 0.5);
            s.store_scaled_add(335, 335, 782, 0.5);
        }

        s.b[2503] = (s.v[335] < 0.0);
        s.v[2503] = if s.b[2503] { 1.0 } else { 0.0 };

        if ((((!s.b[1439]) && s.b[2426]) && s.b[2499]) && s.b[2503]) {
            s.store_scalar(335, 0.0);
            s.store_scalar(336, 0.0);
        }

        if (((!s.b[1439]) && s.b[2426]) && s.b[2499]) {
            s.store_offset(335, 335, 1e-25);
            s.store_div(332, 790, 335);
            s.store_div_from_scalar_square_ad(336, 1.0, s.ad_value(335));
            s.store_square(722, 332);
            s.store_scalar(723, 1.0);
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
        }

        let (assign63840_e98748,) = {
    if (((!s.b[1439]) && s.b[2426]) && s.b[2499]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign63840_e98748;

        let (assign63850_e98757,) = {
    if (((!s.b[1439]) && s.b[2426]) && s.b[2499]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign63850_e98757;

        if (((!s.b[1439]) && s.b[2426]) && s.b[2499]) {
            s.store_scalar(770, 0.0);
            s.store_scalar(726, 0.0);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_add(770, 724, 725);
            s.copy_ad(726, 770);
        }

        s.b[2504] = ((((4.0 == 1.0) || (4.0 == 2.0)) || (4.0 == 4.0)) || (4.0 == 8.0));
        s.v[2504] = if s.b[2504] { 1.0 } else { 0.0 };

        s.b[2505] = (4.0 == 1.0);
        s.v[2505] = if s.b[2505] { 1.0 } else { 0.0 };

        let (assign64000_e98914,) = {
    if (((((!s.b[1439]) && s.b[2426]) && s.b[2499]) && s.b[2504]) && s.b[2505]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign64000_e98914;

        s.b[2506] = (4.0 == 2.0);
        s.v[2506] = if s.b[2506] { 1.0 } else { 0.0 };

        let (assign64020_e98933,) = {
    if ((((((!s.b[1439]) && s.b[2426]) && s.b[2499]) && s.b[2504]) && (!s.b[2505])) && s.b[2506]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign64020_e98933;

        s.b[2507] = (4.0 == 4.0);
        s.v[2507] = if s.b[2507] { 1.0 } else { 0.0 };

        let (assign64040_e98955,) = {
    if (((((((!s.b[1439]) && s.b[2426]) && s.b[2499]) && s.b[2504]) && (!s.b[2505])) && (!s.b[2506])) && s.b[2507]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign64040_e98955;

        s.b[2508] = (4.0 == 8.0);
        s.v[2508] = if s.b[2508] { 1.0 } else { 0.0 };

        let (assign64060_e98980,) = {
    if ((((((((!s.b[1439]) && s.b[2426]) && s.b[2499]) && s.b[2504]) && (!s.b[2505])) && (!s.b[2506])) && (!s.b[2507])) && s.b[2508]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign64060_e98980;

        let (assign64070_e98991,) = {
    if ((((!s.b[1439]) && s.b[2426]) && s.b[2499]) && s.b[2504]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign64070_e98991;

        let mut assign64080_loop_guard: usize = 0;
        while {
            let assign64080_cond_e99003: f64 = if (((((!s.b[1439]) && s.b[2426]) && s.b[2499]) && s.b[2504]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign64080_cond_e99003 != 0.0
        } {
            assign64080_loop_guard += 1;
            assert!(assign64080_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((!s.b[1439]) && s.b[2426]) && s.b[2499]) && s.b[2504]) {
                s.store_sqrt(726, 726);
            }
            let (assign64080_body1_e99028,) = {
    if ((((!s.b[1439]) && s.b[2426]) && s.b[2499]) && s.b[2504]) {
        let assign64080_body1_e99026: f64 = (s.v[719] + 1.0);
        (assign64080_body1_e99026,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign64080_body1_e99028;
        }

        if ((((!s.b[1439]) && s.b[2426]) && s.b[2499]) && (!s.b[2504])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 4.0)));
            }
        }

        if (((!s.b[1439]) && s.b[2426]) && s.b[2499]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(333, 332, 726, 1.0);
            s.store_div_scaled_product_indices(336, 725, 726, 1.0, 770, 1.0);
            s.store_scale(145, 155, ((2.0 * s.v[495]) * p.p7));
            s.copy_ad(335, 170);
            s.store_div_scaled_product_left_ad(153, A::mul3(s.ad_value(145), s.ad_value(253), s.ad_value(152)), 333, 1.0, 335, 1.0);
            s.store_add(134, 134, 153);
        }

        s.b[2509] = (((p.p31 != 0.0) && (p.p30 != 0.0)) && (s.v[963] == 0.0));
        s.v[2509] = if s.b[2509] { 1.0 } else { 0.0 };

        if (((!s.b[1439]) && s.b[2426]) && s.b[2509]) {
            s.store_square(317, 127);
            s.store_mul3_affine_lhs(318, 155, 186, 2.0, 0.0, 248);
            s.store_sub(319, 317, 318);
            s.store_sqrt_square_offset(782, 317, ((4.0 * 0.001) * 0.001));
            s.store_offset_scaled_div(334, 317, 782, 0.5, 0.5);
            s.store_scaled_add(317, 317, 782, 0.5);
        }

        s.b[2510] = (s.v[317] < 0.0);
        s.v[2510] = if s.b[2510] { 1.0 } else { 0.0 };

        if ((((!s.b[1439]) && s.b[2426]) && s.b[2509]) && s.b[2510]) {
            s.store_scalar(317, 0.0);
            s.store_scalar(334, 0.0);
        }

        if (((!s.b[1439]) && s.b[2426]) && s.b[2509]) {
            s.store_sqrt_square_offset(782, 319, ((4.0 * 0.001) * 0.001));
            s.store_offset_scaled_div(334, 319, 782, 0.5, 0.5);
            s.store_scaled_add(319, 319, 782, 0.5);
        }

        s.b[2511] = (s.v[319] < 0.0);
        s.v[2511] = if s.b[2511] { 1.0 } else { 0.0 };

        if ((((!s.b[1439]) && s.b[2426]) && s.b[2509]) && s.b[2511]) {
            s.store_scalar(319, 0.0);
            s.store_scalar(334, 0.0);
        }

        if (((!s.b[1439]) && s.b[2426]) && s.b[2509]) {
            s.store_sub(320, 317, 319);
        }

        s.b[2512] = ((s.v[238] < (10.0 * 2.220446049250313e-16)) || (s.v[320] < (10.0 * 2.220446049250313e-16)));
        s.v[2512] = if s.b[2512] { 1.0 } else { 0.0 };

        let (assign64350_e99365,) = {
    if ((((!s.b[1439]) && s.b[2426]) && s.b[2509]) && s.b[2512]) {
        (0.0,)
    } else {
        (s.v[321],)
    }
};
        s.v[321] = assign64350_e99365;

        let (assign64360_e99377,) = {
    if ((((!s.b[1439]) && s.b[2426]) && s.b[2509]) && (!s.b[2512])) {
        (1.0,)
    } else {
        (s.v[321],)
    }
};
        s.v[321] = assign64360_e99377;

        let (assign64370_e99384,) = {
    if ((!s.b[1439]) && (s.v[946] != 0.0)) {
        (0.0,)
    } else {
        (s.v[946],)
    }
};
        s.v[946] = assign64370_e99384;

        s.b[2513] = ((s.v[78] == 0.0) && (s.v[127] > 1e-12));
        s.v[2513] = if s.b[2513] { 1.0 } else { 0.0 };

        if ((!s.b[1439]) && s.b[2513]) {
            s.store_div_scaled_product_indices(130, 212, 154, 1.0, 100, 2.0);
            s.store_add_ad_lhs(128, A::div_scaled_value_offset_denominator(s.ad_value(127), 1.0, s.ad_value(130), 1.0, 1.0), 87);
        }

        if ((!s.b[1439]) && (!s.b[2513])) {
            s.store_scalar(128, 0.0);
        }

        if (!s.b[1439]) {
            s.copy_ad(136, 134);
            s.store_scalar(46, 0.0);
        }

        s.b[2515] = ((p.p450 > 0.0) && (p.p454 > 0.0));
        s.v[2515] = if s.b[2515] { 1.0 } else { 0.0 };

        if ((!s.b[1439]) && s.b[2515]) {
            s.store_scalar(2520, 1e-5);
            s.store_offset_add_scaled_inputs3_offset_indices(2521, 791, 1.0, 120, 1.0, 182, -1.0, (-s.v[160]), (-p.p455));
        }

        let (assign64500_e99490,) = {
    if ((!s.b[1439]) && s.b[2515]) {
        let assign64500_e99488: f64 = (s.v[118] + p.p455);
        (assign64500_e99488,)
    } else {
        (s.v[2522],)
    }
};
        s.v[2522] = assign64500_e99490;

        if ((!s.b[1439]) && s.b[2515]) {
            s.store_sqrt_offset_ad(781, A::mul(A::sub(s.ad_value(960), s.ad_value(1431)), A::sub(s.ad_value(960), s.ad_value(1431))), ((4.0 * 0.01) * 0.01));
            s.store_add_scaled_inputs3_indices(2532, 960, 0.5, 1431, ((-1.0) * 0.5), 781, 0.5);
            s.store_sqrt_ad(2516, A::div_scaled_product_offset_denominator(s.ad_value(2532), s.ad_value(586), (((2.0 * 1.6021918e-19) * 1.0 / (1.034943e-10)) * s.v[489]), s.ad_value(586), s.v[489], 1.0));
            s.store_mul(2518, 2516, 162);
            s.store_div_scaled_product_add_scaled_denominator_indices(993, 2518, 2518, (-0.25), 790, 1.0, 2518, 1.0, 1.0);
        }

        s.b[2534] = (p.p457 > 0.0);
        s.v[2534] = if s.b[2534] { 1.0 } else { 0.0 };

        if (((!s.b[1439]) && s.b[2515]) && s.b[2534]) {
            s.store_scalar(2519, p.p457);
        }

        if (((!s.b[1439]) && s.b[2515]) && (!s.b[2534])) {
            s.copy_ad(2535, 993);
        }

        let (assign64590_e99602,) = {
    if (((!s.b[1439]) && s.b[2515]) && (!s.b[2534])) {
        (s.v[2522],)
    } else {
        (s.v[2536],)
    }
};
        s.v[2536] = assign64590_e99602;

    }

    pub(super) fn stamp_transient_block_65(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((!s.b[1439]) && s.b[2515]) && (!s.b[2534])) {
            s.store_offset_div_scaled_offset_numerator(332, A::mul(s.ad_value(154), A::sub(s.ad_value(2521), s.ad_value(2535))), 4.0, ((-1.0) * 4.0), A::mul(s.ad_value(213), s.ad_value(156)), 1.0, 1.0);
        }

        if (((!s.b[1439]) && s.b[2515]) && (!s.b[2534])) {
            if (s.v[332] >= (10.0 * 2.220446049250313e-16)) {
            } else {
                s.store_scalar(332, (10.0 * 2.220446049250313e-16));
            }
        }

        if (((!s.b[1439]) && s.b[2515]) && (!s.b[2534])) {
            s.store_add_ad_rhs(89, 2521, A::mul3_scaled_output(s.ad_value(213), s.ad_value(154), A::sub_from_scalar(1.0, A::sqrt(s.ad_value(332))), 0.5));
            s.store_mul_sub_rhs(116, 154, 89, 2535);
        }

        s.b[2537] = (s.v[116] < 3.0);
        s.v[2537] = if s.b[2537] { 1.0 } else { 0.0 };

        if ((((!s.b[1439]) && s.b[2515]) && (!s.b[2534])) && s.b[2537]) {
            s.store_mul_sub_rhs(333, 154, 2521, 2535);
            s.store_div_from_scalar_ad(335, 1.0, A::mul_scaled_lhs(s.ad_value(154), (1.414213562373095 / 108.0), s.ad_value(212)));
            s.store_offset_scaled(336, 335, 3.0, 81.0);
            s.store_add_scaled_sub_value_product_mixed_aii(337, (-2916.0), A::scale(s.ad_value(335), 81.0), 1.0, 335, 333, 27.0);
            s.store_add_scaled_sub_value_product_mixed_aii(338, 1458.0, A::scaled_offset(s.ad_value(335), 54.0, 81.0), 1.0, 335, 333, 27.0);
            s.store_square(338, 338);
        }

        if ((((!s.b[1439]) && s.b[2515]) && (!s.b[2534])) && s.b[2537]) {
            if ((s.v[337] + ((((((4.0 * s.v[336]) * s.v[336]) * s.v[336]) + s.v[338])) as f64).sqrt()) == 0.0) {
                s.store_scalar(339, 0.0);
            } else {
                s.store_powf_ad(339, A::add(s.ad_value(337), A::sqrt(A::add(A::mul3_scaled_output(s.ad_value(336), s.ad_value(336), s.ad_value(336), 4.0), s.ad_value(338)))), 0.3333333333333333);
            }
        }

        if ((((!s.b[1439]) && s.b[2515]) && (!s.b[2534])) && s.b[2537]) {
            s.store_add_scaled_ad_lhs(332, A::sub_from_scalar(3.0, A::div_scaled_inputs(s.ad_value(336), 1.259921049894873, s.ad_value(339), 3.0)), 339, (1.0 / (3.0 * 1.259921049894873)));
            s.store_add_scaled_product_indices(89, 2535, 1.0, 332, 155, 1.0);
            s.copy_ad(88, 89);
        }

        s.b[2538] = (s.v[791] <= s.v[2536]);
        s.v[2538] = if s.b[2538] { 1.0 } else { 0.0 };

        if (((((!s.b[1439]) && s.b[2515]) && (!s.b[2534])) && (!s.b[2537])) && s.b[2538]) {
            s.copy_ad(88, 89);
        }

        if (((((!s.b[1439]) && s.b[2515]) && (!s.b[2534])) && (!s.b[2537])) && (!s.b[2538])) {
            s.store_div_scalar_by_product(335, 1.0, s.ad_value(210), s.ad_value(211), 1.0);
            s.store_mul3_lhs(336, 335, 2521, 2521);
            s.store_add_ad_rhs(337, 154, A::div_from_scalar(2.0, s.ad_value(2521)));
            s.store_offset_div_ad(90, A::ln(s.ad_value(336)), s.ad_value(337), p.p456);
            s.store_offset_sub(781, 90, 89, (-0.0008));
            s.store_scale(782, 90, (4.0 * 0.0008));
        }

        if (((((!s.b[1439]) && s.b[2515]) && (!s.b[2534])) && (!s.b[2537])) && (!s.b[2538])) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if (((((!s.b[1439]) && s.b[2515]) && (!s.b[2534])) && (!s.b[2537])) && (!s.b[2538])) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_add_scaled_inputs3_indices(88, 90, 1.0, 781, (-0.5), 782, (-0.5));
        }

        if (((!s.b[1439]) && s.b[2515]) && (!s.b[2534])) {
            s.store_offset(332, 2535, (1e-12 / 2.0));
        }

        s.b[2539] = (s.v[88] < s.v[332]);
        s.v[2539] = if s.b[2539] { 1.0 } else { 0.0 };

        if ((((!s.b[1439]) && s.b[2515]) && (!s.b[2534])) && s.b[2539]) {
            s.copy_ad(88, 332);
        }

        if (((!s.b[1439]) && s.b[2515]) && (!s.b[2534])) {
            s.copy_ad(2519, 88);
        }

        s.b[2540] = (p.p451 == 1.0);
        s.v[2540] = if s.b[2540] { 1.0 } else { 0.0 };

        if ((((!s.b[1439]) && s.b[2515]) && (!s.b[2534])) && s.b[2540]) {
            s.copy_ad(88, 2519);
            s.copy_ad(2541, 993);
        }

        let (assign64930_e100183,) = {
    if ((((!s.b[1439]) && s.b[2515]) && (!s.b[2534])) && s.b[2540]) {
        let assign64930_e100175: f64 = (s.v[160] - s.v[120]);
        let assign64930_e100177: f64 = (assign64930_e100175 + s.v[182]);
        let assign64930_e100179: f64 = (assign64930_e100177 + s.v[2541]);
        let assign64930_e100181: f64 = (assign64930_e100179 + p.p455);
        (assign64930_e100181,)
    } else {
        (s.v[86],)
    }
};
        s.v[86] = assign64930_e100183;

        s.b[2550] = (s.v[791] < s.v[86]);
        s.v[2550] = if s.b[2550] { 1.0 } else { 0.0 };

        let (assign64950_e100201,) = {
    if (((((!s.b[1439]) && s.b[2515]) && (!s.b[2534])) && s.b[2540]) && s.b[2550]) {
        let assign64950_e100199: f64 = (-1.0);
        (assign64950_e100199,)
    } else {
        (s.v[347],)
    }
};
        s.v[347] = assign64950_e100201;

        if (((((!s.b[1439]) && s.b[2515]) && (!s.b[2534])) && s.b[2540]) && s.b[2550]) {
            s.store_mul_scaled_ln_ad_rhs(271, 155, 2.0, A::div_from_scalar((-s.v[270]), s.ad_value(212)));
            s.store_mul_sub_rhs(332, 154, 2521, 2541);
            s.store_div_from_scalar_mul_ad(335, 1.0, s.ad_value(154), s.ad_value(209));
            s.store_mul(333, 335, 185);
            s.store_offset_scaled(279, 333, (3.0 * 1.414213562373095), 2.0);
            s.store_mul3_affine_lhs(277, 279, 279, 8.0, 0.0, 279);
            s.store_offset(338, 332, (-2.0));
            s.store_scaled_mul(339, 333, 338, 9.0);
            s.store_sub_from_scalar(278, (7.0 * 1.414213562373095), 339);
            s.store_square(276, 278);
        }

        s.b[2551] = (s.v[277] < (s.v[276] * 1e-8));
        s.v[2551] = if s.b[2551] { 1.0 } else { 0.0 };

        if ((((((!s.b[1439]) && s.b[2515]) && (!s.b[2534])) && s.b[2540]) && s.b[2550]) && s.b[2551]) {
            s.store_add_scaled_inputs3_offset_mixed_iai(274, 278, 1.0, A::div_scaled_inputs(s.ad_value(277), 0.5, s.ad_value(278), 1.0), 1.0, 339, 1.0, ((-7.0) * 1.414213562373095));
        }

        if ((((((!s.b[1439]) && s.b[2515]) && (!s.b[2534])) && s.b[2540]) && s.b[2550]) && (!s.b[2551])) {
            s.store_sqrt_add(275, 277, 276);
            s.store_add_offset_lhs(274, 275, ((-7.0) * 1.414213562373095), 339);
        }

        if (((((!s.b[1439]) && s.b[2515]) && (!s.b[2534])) && s.b[2540]) && s.b[2550]) {
            if (s.v[274] == 0.0) {
                s.store_scalar(273, 0.0);
            } else {
                s.store_powf(273, 274, 0.3333333333333333);
            }
        }

        if (((((!s.b[1439]) && s.b[2515]) && (!s.b[2534])) && s.b[2540]) && s.b[2550]) {
            s.store_add_scaled_inputs_product_first_ad(272, A::sub_from_scalar(((-4.0) * 1.414213562373095), A::scale(s.ad_value(333), 12.0)), 1.0, 273, 2.0, 273, 273, 1.414213562373095);
            s.store_div_from_scalar(335, 1.0, 273);
            s.store_mul(116, 272, 335);
            s.store_add_scaled_product_indices(167, 2541, 1.0, 116, 155, 1.0);
            s.store_sub(335, 167, 2541);
            s.store_div(336, 335, 271);
            s.store_sqrt_square_offset(337, 336, 1.0);
            s.store_add_ad_lhs(2519, A::div(s.ad_value(335), s.ad_value(337)), 2541);
        }

        if (((((!s.b[1439]) && s.b[2515]) && (!s.b[2534])) && s.b[2540]) && (!s.b[2550])) {
            s.store_exp_ad(230, A::mul_offset_rhs(s.ad_value(154), s.ad_value(2541), (-p.p456)));
        }

        let (assign65200_e100667,) = {
    if (((((!s.b[1439]) && s.b[2515]) && (!s.b[2534])) && s.b[2540]) && (!s.b[2550])) {
        (0.0,)
    } else {
        (s.v[79],)
    }
};
        s.v[79] = assign65200_e100667;

        if (((((!s.b[1439]) && s.b[2515]) && (!s.b[2534])) && s.b[2540]) && (!s.b[2550])) {
            s.copy_ad(2542, 88);
            s.store_mul3_affine_lhs(2543, 166, 2520, (0.5 * 9662367879.197212), 0.0, 2520);
            s.store_sqrt_ad(334, A::mul_scaled_lhs(s.ad_value(154), 2.0, s.ad_value(2543)));
            s.store_scaled_add_ad(335, A::limited_exp(s.ad_value(334)), A::limited_exp_scaled_input(s.ad_value(334), -1.0), 0.5);
            s.store_div_ad_lhs(2544, A::ln(s.ad_value(335)), 2543);
        }

        let (assign65260_e100780,) = {
    if (((((!s.b[1439]) && s.b[2515]) && (!s.b[2534])) && s.b[2540]) && (!s.b[2550])) {
        (1.0,)
    } else {
        (s.v[97],)
    }
};
        s.v[97] = assign65260_e100780;

        let mut assign65270_loop_guard: usize = 0;
        while {
            let assign65270_cond_e100796: f64 = (s.v[421] + 1.0);
            let assign65270_cond_e100798: f64 = if ((((((!s.b[1439]) && s.b[2515]) && (!s.b[2534])) && s.b[2540]) && (!s.b[2550])) && (s.v[97] <= assign65270_cond_e100796)) { 1.0 } else { 0.0 };
            assign65270_cond_e100798 != 0.0
        } {
            assign65270_loop_guard += 1;
            assert!(assign65270_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((((!s.b[1439]) && s.b[2515]) && (!s.b[2534])) && s.b[2540]) && (!s.b[2550])) {
                s.store_sub(2545, 2542, 2541);
                s.store_mul(116, 154, 2545);
                s.store_mul_sub_rhs(333, 2544, 2545, 2543);
            }
            s.b[2552] = (s.v[333] < 60.0);
            s.v[2552] = if s.b[2552] { 1.0 } else { 0.0 };
            if ((((((!s.b[1439]) && s.b[2515]) && (!s.b[2534])) && s.b[2540]) && (!s.b[2550])) && s.b[2552]) {
                s.store_exp(335, 333);
                s.store_exp_mul_scaled_lhs_indices(334, 2544, -1.0, 2543);
                s.store_sub(336, 335, 334);
                s.store_div_ad_lhs(2547, A::ln(A::offset(s.ad_value(336), 1.0)), 2544);
                s.store_div_scaled_value_offset_denominator(2548, s.ad_value(335), 1.0, s.ad_value(336), 1.0, 1.0);
            }
            if ((((((!s.b[1439]) && s.b[2515]) && (!s.b[2534])) && s.b[2540]) && (!s.b[2550])) && (!s.b[2552])) {
                s.store_sub(2547, 2545, 2543);
                s.store_scalar(2548, 1.0);
            }
            if (((((!s.b[1439]) && s.b[2515]) && (!s.b[2534])) && s.b[2540]) && (!s.b[2550])) {
                s.store_mul(2546, 154, 2547);
            }
            s.b[2553] = (((s.v[116]) as f64).abs() < 1e-16);
            s.v[2553] = if s.b[2553] { 1.0 } else { 0.0 };
            if ((((((!s.b[1439]) && s.b[2515]) && (!s.b[2534])) && s.b[2540]) && (!s.b[2550])) && s.b[2553]) {
                s.store_sqrt_scaled_input_ad(334, A::sub_from_scalar(1.0, A::square(s.ad_value(2548))), 1.0 / (2.0));
                s.store_mul(223, 116, 334);
                s.store_mul(2549, 154, 334);
            }
            s.b[2554] = (s.v[116] < 0.0);
            s.v[2554] = if s.b[2554] { 1.0 } else { 0.0 };
            if (((((((!s.b[1439]) && s.b[2515]) && (!s.b[2534])) && s.b[2540]) && (!s.b[2550])) && s.b[2553]) && s.b[2554]) {
                s.store_neg(223, 223);
                s.store_neg(2549, 2549);
            }
            s.b[2555] = (((s.v[116]) as f64).abs() < 0.005);
            s.v[2555] = if s.b[2555] { 1.0 } else { 0.0 };
            if (((((((!s.b[1439]) && s.b[2515]) && (!s.b[2534])) && s.b[2540]) && (!s.b[2550])) && (!s.b[2553])) && s.b[2555]) {
                s.store_square_mul_sub_from_scalar_double_scaled_sub_rhs_scaled_output(334, 116, 1.0, 116, 1.0, 116, 1.0, 116, 0.2, 1.0 / (4.0), 1.0 / (3.0), 1.0 / (2.0));
                s.store_mul_sub_from_scalar_ad_rhs(335, 116, 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(116), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(116), 1.0, A::scale(s.ad_value(116), 0.25), 1.0 / (3.0)), 1.0 / (2.0)));
                s.store_square_mul_sub_from_scalar_double_scaled_sub_rhs_scaled_output(336, 2546, 1.0, 2546, 1.0, 2546, 1.0, 2546, 0.2, 1.0 / (4.0), 1.0 / (3.0), 1.0 / (2.0));
                s.store_mul_sub_from_scalar_ad_rhs(337, 2546, 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(2546), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(2546), 1.0, A::scale(s.ad_value(2546), 0.25), 1.0 / (3.0)), 1.0 / (2.0)));
                s.store_sqrt_sub(223, 334, 336);
                s.store_div_scaled_product_right_ad(2549, 154, A::add_scaled_product(s.ad_value(335), 1.0, s.ad_value(2548), s.ad_value(337), (-1.0)), 0.5, 223, 1.0);
            }
            if (((((((!s.b[1439]) && s.b[2515]) && (!s.b[2534])) && s.b[2540]) && (!s.b[2550])) && (!s.b[2553])) && (!s.b[2555])) {
                s.store_exp_neg_input(334, 116);
                s.store_exp_neg_input(335, 2546);
                s.store_sqrt_ad(223, A::add_scaled_inputs4(s.ad_value(116), 1.0, s.ad_value(2546), (-1.0), s.ad_value(334), 1.0, s.ad_value(335), (-1.0)));
                s.store_div_scaled_product_right_ad(2549, 154, A::sub(A::sub_from_scalar(1.0, s.ad_value(334)), A::mul_sub_from_scalar_rhs(s.ad_value(2548), 1.0, s.ad_value(335))), 0.5, 223, 1.0);
            }
            s.b[2556] = ((s.v[79] == 1.0) && (s.v[116] < 0.0));
            s.v[2556] = if s.b[2556] { 1.0 } else { 0.0 };
            let (assign65270_body31_e101470,) = {
    if ((((((!s.b[1439]) && s.b[2515]) && (!s.b[2534])) && s.b[2540]) && (!s.b[2550])) && s.b[2556]) {
        let assign65270_body31_e101468: f64 = (-1.0);
        (assign65270_body31_e101468,)
    } else {
        (s.v[347],)
    }
};
            s.v[347] = assign65270_body31_e101470;
            s.b[2557] = (s.v[116] < 0.0);
            s.v[2557] = if s.b[2557] { 1.0 } else { 0.0 };
            if ((((((!s.b[1439]) && s.b[2515]) && (!s.b[2534])) && s.b[2540]) && (!s.b[2550])) && s.b[2557]) {
                s.store_neg(216, 223);
                s.store_neg(217, 2549);
            }
            s.b[2558] = (s.v[116] < 1e-7);
            s.v[2558] = if s.b[2558] { 1.0 } else { 0.0 };
            if (((((((!s.b[1439]) && s.b[2515]) && (!s.b[2534])) && s.b[2540]) && (!s.b[2550])) && (!s.b[2557])) && s.b[2558]) {
                s.copy_ad(216, 223);
                s.copy_ad(217, 2549);
            }
            if (((((((!s.b[1439]) && s.b[2515]) && (!s.b[2534])) && s.b[2540]) && (!s.b[2550])) && (!s.b[2557])) && (!s.b[2558])) {
                s.store_mul_offset_rhs(117, 154, 2542, (-p.p456));
                s.store_exp(228, 117);
                s.store_mul_ad_rhs(214, 210, A::add_scaled_offset_product_rhs(s.ad_value(228), 1.0, s.ad_value(230), s.ad_value(116), 1.0, (-1.0)));
                s.store_mul_ad_product_rhs(215, 210, s.ad_value(154), A::sub(s.ad_value(228), s.ad_value(230)));
                s.store_sqrt_square_add(216, 223, 214);
                s.store_div_scaled_add_product(217, s.ad_value(215), 0.5, s.ad_value(2549), s.ad_value(223), (2.0 * 0.5), s.ad_value(216), 1.0);
            }
            if (((((!s.b[1439]) && s.b[2515]) && (!s.b[2534])) && s.b[2540]) && (!s.b[2550])) {
                s.store_add_scaled_inputs_product_indices(232, 2542, 1.0, 2521, (-1.0), 212, 216, 1.0);
                s.store_offset_mul(233, 212, 217, 1.0);
            }
            s.b[2559] = (s.v[79] == 1.0);
            s.v[2559] = if s.b[2559] { 1.0 } else { 0.0 };
            let (assign65270_body47_e101775,) = {
    if ((((((!s.b[1439]) && s.b[2515]) && (!s.b[2534])) && s.b[2540]) && (!s.b[2550])) && s.b[2559]) {
        let assign65270_body47_e101773: f64 = (s.v[421] + 1.0);
        (assign65270_body47_e101773,)
    } else {
        (s.v[97],)
    }
};
            s.v[97] = assign65270_body47_e101775;
            if ((((((!s.b[1439]) && s.b[2515]) && (!s.b[2534])) && s.b[2540]) && (!s.b[2550])) && (!s.b[2559])) {
                s.store_div_scaled_inputs(236, s.ad_value(232), -1.0, s.ad_value(233), 1.0);
            }
            if ((((((!s.b[1439]) && s.b[2515]) && (!s.b[2534])) && s.b[2540]) && (!s.b[2550])) && (!s.b[2559])) {
                s.store_scaled_offset_ad(93, {
                    if (1.0 >= ((s.v[2542]) as f64).abs()) {
                        A::constant(1.0)
                    } else {
                        A::abs(s.ad_value(2542))
                    }
                }, 1.0, (0.5 * 0.1));
            }
            s.b[2560] = (((s.v[236]) as f64).abs() > s.v[93]);
            s.v[2560] = if s.b[2560] { 1.0 } else { 0.0 };
            if (((((((!s.b[1439]) && s.b[2515]) && (!s.b[2534])) && s.b[2540]) && (!s.b[2550])) && (!s.b[2559])) && s.b[2560]) {
                s.store_scale(236, 93, (if (s.v[236] >= 0.0) { 1.0 } else { (-1.0) }));
            }
            if ((((((!s.b[1439]) && s.b[2515]) && (!s.b[2534])) && s.b[2540]) && (!s.b[2550])) && (!s.b[2559])) {
                s.store_add(2542, 2542, 236);
            }
            s.b[2561] = ((((s.v[236]) as f64).abs() <= 1e-12) && (((s.v[232]) as f64).abs() <= 1e-8));
            s.v[2561] = if s.b[2561] { 1.0 } else { 0.0 };
            let (assign65270_body54_e101908,) = {
    if (((((((!s.b[1439]) && s.b[2515]) && (!s.b[2534])) && s.b[2540]) && (!s.b[2550])) && (!s.b[2559])) && s.b[2561]) {
        (1.0,)
    } else {
        (s.v[79],)
    }
};
            s.v[79] = assign65270_body54_e101908;
            let (assign65270_body55_e101925,) = {
    if (((((!s.b[1439]) && s.b[2515]) && (!s.b[2534])) && s.b[2540]) && (!s.b[2550])) {
        let assign65270_body55_e101923: f64 = (s.v[97] + 1.0);
        (assign65270_body55_e101923,)
    } else {
        (s.v[97],)
    }
};
            s.v[97] = assign65270_body55_e101925;
        }

        if (((((!s.b[1439]) && s.b[2515]) && (!s.b[2534])) && s.b[2540]) && (!s.b[2550])) {
            s.copy_ad(2519, 2542);
        }

    }

    pub(super) fn stamp_transient_block_66(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((!s.b[1439]) && s.b[2515]) {
            s.store_mul_sub_scaled_inputs_rhs(339, 154, s.ad_value(2519), -1.0, s.ad_value(993), -1.0);
            s.store_abs(2531, 339);
            s.store_exp(340, 339);
            s.store_sub_offset_lhs(341, 340, (-1.0), 339);
        }

        s.b[2562] = (s.v[339] > 1e-7);
        s.v[2562] = if s.b[2562] { 1.0 } else { 0.0 };

        if (((!s.b[1439]) && s.b[2515]) && s.b[2562]) {
            s.store_mul_scaled_sqrt_rhs(2533, 209, -1.0, 341);
        }

        s.b[2563] = (s.v[2531] > 1e-7);
        s.v[2563] = if s.b[2563] { 1.0 } else { 0.0 };

        if ((((!s.b[1439]) && s.b[2515]) && (!s.b[2562])) && s.b[2563]) {
            s.store_mul_sqrt_rhs(2533, 209, 341);
        }

        if ((((!s.b[1439]) && s.b[2515]) && (!s.b[2562])) && (!s.b[2563])) {
            s.store_mul_scaled_sqrt_ad_rhs(2533, 339, (-0.7071067811865475), A::offset(A::mul_scaled_lhs(s.ad_value(2531), 0.3333333333333333, A::scale_offset(s.ad_value(2531), 0.25, 1.0)), 1.0));
        }

        if ((!s.b[1439]) && s.b[2515]) {
            s.store_sqrt_square_offset(781, 2533, ((4.0 * 1e-6) * 1e-6));
            s.store_scaled_add(2528, 2533, 781, 0.5);
            s.store_div_scaled_inputs(2529, s.ad_value(2528), 1.0, s.ad_value(586), 1.6021918e-19);
            s.store_offset(335, 2529, (-p.p452));
            s.store_scale(2530, 2529, 0.01);
            s.store_sqrt_ad(781, A::add_scaled_square_product(s.ad_value(335), 1.0, s.ad_value(2530), s.ad_value(2530), 4.0));
            s.store_scaled_add(336, 335, 781, 0.5);
            s.store_div_scaled_product_by_product(2527, s.ad_value(336), s.ad_value(336), 1.0, s.ad_value(2529), s.ad_value(2529), 1.0);
            s.store_add_scaled_product_left_ad(994, 993, 1.0, A::sub(s.ad_value(2519), s.ad_value(993)), 2527, 1.0);
            s.store_mul_sub_from_scalar_rhs_ad(333, A::exp(A::mul(s.ad_value(154), A::add_scaled_inputs3(s.ad_value(994), 1.0, s.ad_value(960), -1.0, s.ad_value(1431), 1.0))), 1.0, A::exp(A::mul_scaled_lhs(s.ad_value(154), -1.0, s.ad_value(790))));
            s.store_scalar(2523, (((((2.0 * 1.6021918e-19) * s.v[489]) * 1.034943e-10)) as f64).sqrt());
            s.store_mul_sqrt_rhs(2524, 2523, 155);
            s.store_mul_sub_rhs(2517, 154, 994, 993);
        }

        s.b[2564] = ((s.v[2517] < (0.2 * s.v[154])) && ((0.2 * s.v[154]) >= 0.0));
        s.v[2564] = if s.b[2564] { 1.0 } else { 0.0 };

        if (((!s.b[1439]) && s.b[2515]) && s.b[2564]) {
            s.store_sub_scaled_inputs(781, 154, 0.2, 2517, 1.0);
            s.store_square(722, 781);
            s.store_scaled_mul(723, 154, 154, (0.2 * 0.2));
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
        }

        let (assign65570_e102289,) = {
    if (((!s.b[1439]) && s.b[2515]) && s.b[2564]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign65570_e102289;

        let (assign65580_e102298,) = {
    if (((!s.b[1439]) && s.b[2515]) && s.b[2564]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign65580_e102298;

        if (((!s.b[1439]) && s.b[2515]) && s.b[2564]) {
            s.store_scalar(770, 0.0);
            s.store_scalar(726, 0.0);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_add(770, 724, 725);
            s.copy_ad(726, 770);
        }

        s.b[2565] = ((((1.0 == 1.0) || (1.0 == 2.0)) || (1.0 == 4.0)) || (1.0 == 8.0));
        s.v[2565] = if s.b[2565] { 1.0 } else { 0.0 };

        s.b[2566] = (1.0 == 1.0);
        s.v[2566] = if s.b[2566] { 1.0 } else { 0.0 };

        let (assign65670_e102389,) = {
    if (((((!s.b[1439]) && s.b[2515]) && s.b[2564]) && s.b[2565]) && s.b[2566]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign65670_e102389;

        s.b[2567] = (1.0 == 2.0);
        s.v[2567] = if s.b[2567] { 1.0 } else { 0.0 };

        let (assign65690_e102408,) = {
    if ((((((!s.b[1439]) && s.b[2515]) && s.b[2564]) && s.b[2565]) && (!s.b[2566])) && s.b[2567]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign65690_e102408;

        s.b[2568] = (1.0 == 4.0);
        s.v[2568] = if s.b[2568] { 1.0 } else { 0.0 };

        let (assign65710_e102430,) = {
    if (((((((!s.b[1439]) && s.b[2515]) && s.b[2564]) && s.b[2565]) && (!s.b[2566])) && (!s.b[2567])) && s.b[2568]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign65710_e102430;

        s.b[2569] = (1.0 == 8.0);
        s.v[2569] = if s.b[2569] { 1.0 } else { 0.0 };

        let (assign65730_e102455,) = {
    if ((((((((!s.b[1439]) && s.b[2515]) && s.b[2564]) && s.b[2565]) && (!s.b[2566])) && (!s.b[2567])) && (!s.b[2568])) && s.b[2569]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign65730_e102455;

        let (assign65740_e102466,) = {
    if ((((!s.b[1439]) && s.b[2515]) && s.b[2564]) && s.b[2565]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign65740_e102466;

        let mut assign65750_loop_guard: usize = 0;
        while {
            let assign65750_cond_e102478: f64 = if (((((!s.b[1439]) && s.b[2515]) && s.b[2564]) && s.b[2565]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign65750_cond_e102478 != 0.0
        } {
            assign65750_loop_guard += 1;
            assert!(assign65750_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((!s.b[1439]) && s.b[2515]) && s.b[2564]) && s.b[2565]) {
                s.store_sqrt(726, 726);
            }
            let (assign65750_body1_e102503,) = {
    if ((((!s.b[1439]) && s.b[2515]) && s.b[2564]) && s.b[2565]) {
        let assign65750_body1_e102501: f64 = (s.v[719] + 1.0);
        (assign65750_body1_e102501,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign65750_body1_e102503;
        }

        if ((((!s.b[1439]) && s.b[2515]) && s.b[2564]) && (!s.b[2565])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / 2.0));
            }
        }

        if (((!s.b[1439]) && s.b[2515]) && s.b[2564]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_mul3_affine_lhs(780, 781, 154, 0.2, 0.0, 726);
            s.store_div_scaled_product3_indices(334, 154, 725, 726, 0.2, 770, 1.0);
            s.store_sub_scaled_inputs(335, 154, 0.2, 780, 1.0);
        }

        if (((!s.b[1439]) && s.b[2515]) && s.b[2564]) {
        }

        if (((!s.b[1439]) && s.b[2515]) && (!s.b[2564])) {
            s.copy_ad(335, 2517);
            s.store_scalar(334, 1.0);
        }

        if ((!s.b[1439]) && s.b[2515]) {
            s.store_sqrt_offset_input(2525, 335, (10.0 * 2.220446049250313e-16));
            s.store_mul(2526, 2524, 2525);
            s.store_mul_scaled_ad_lhs(995, A::div_scaled_inputs(s.ad_value(155), 2.0, s.ad_value(162), 1.0), 2526, p.p454);
            s.store_scaled_mul(46, 995, 333, s.v[632]);
            s.store_add(134, 136, 46);
        }

        if (!s.b[1439]) {
            s.store_add(134, 136, 46);
            s.copy_ad(978, 133);
        }

        s.store_scale(335, 162, (-s.v[635]));

        s.store_mul(20, 335, 131);

        s.store_mul(132, 335, 133);

        s.store_mul(19, 132, 247);

        s.store_mul(979, 335, 978);

        s.store_scaled_sub(335, 790, 94, 0.5);

        s.store_scale(781, 335, (2.0 * 1.0 / (p.p263)));

        s.store_offset_mul_offset_rhs_ad_rhs(782, 781, A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul(s.ad_value(781), A::scale_offset(s.ad_value(781), (1.0 / 5040.0), (1.0 / 720.0))), (1.0 / 120.0)), (1.0 / 24.0)), (1.0 / 6.0)), (1.0 / 2.0), 1.0);

        s.store_offset_mul_offset_rhs_ad_rhs(783, 781, A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul(s.ad_value(781), A::scale_offset(s.ad_value(781), (1.0 / 840.0), (1.0 / 144.0))), (1.0 / 30.0)), (1.0 / 8.0)), (1.0 / 3.0), (1.0 / 2.0));

        s.store_div_from_scalar(110, p.p263, 782);

        s.store_div_scaled_inputs(336, s.ad_value(783), (-2.0), A::square(s.ad_value(782)), 1.0);

        s.b[2570] = ((s.v[110] < ((10.0 * 2.220446049250313e-16) + (10.0 * 2.220446049250313e-16))) && ((10.0 * 2.220446049250313e-16) >= 0.0));
        s.v[2570] = if s.b[2570] { 1.0 } else { 0.0 };

        if s.b[2570] {
            s.store_sub_from_scalar(781, ((10.0 * 2.220446049250313e-16) + (10.0 * 2.220446049250313e-16)), 110);
            s.store_square(722, 781);
            s.store_scalar(723, ((10.0 * 2.220446049250313e-16) * (10.0 * 2.220446049250313e-16)));
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
        }

        let (assign66090_e102850,) = {
    if s.b[2570] {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign66090_e102850;

        let (assign66100_e102854,) = {
    if s.b[2570] {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign66100_e102854;

        if s.b[2570] {
            s.store_scalar(770, 0.0);
            s.store_scalar(726, 0.0);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_add(770, 724, 725);
            s.copy_ad(726, 770);
        }

        s.b[2571] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.v[2571] = if s.b[2571] { 1.0 } else { 0.0 };

        s.b[2572] = (2.0 == 1.0);
        s.v[2572] = if s.b[2572] { 1.0 } else { 0.0 };

        let (assign66210_e102922,) = {
    if ((s.b[2570] && s.b[2571]) && s.b[2572]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign66210_e102922;

        s.b[2573] = (2.0 == 2.0);
        s.v[2573] = if s.b[2573] { 1.0 } else { 0.0 };

        let (assign66230_e102936,) = {
    if (((s.b[2570] && s.b[2571]) && (!s.b[2572])) && s.b[2573]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign66230_e102936;

        s.b[2574] = (2.0 == 4.0);
        s.v[2574] = if s.b[2574] { 1.0 } else { 0.0 };

        let (assign66250_e102953,) = {
    if ((((s.b[2570] && s.b[2571]) && (!s.b[2572])) && (!s.b[2573])) && s.b[2574]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign66250_e102953;

        s.b[2575] = (2.0 == 8.0);
        s.v[2575] = if s.b[2575] { 1.0 } else { 0.0 };

        let (assign66270_e102973,) = {
    if (((((s.b[2570] && s.b[2571]) && (!s.b[2572])) && (!s.b[2573])) && (!s.b[2574])) && s.b[2575]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign66270_e102973;

        let (assign66280_e102979,) = {
    if (s.b[2570] && s.b[2571]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign66280_e102979;

        let mut assign66290_loop_guard: usize = 0;
        while {
            let assign66290_cond_e102986: f64 = if ((s.b[2570] && s.b[2571]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign66290_cond_e102986 != 0.0
        } {
            assign66290_loop_guard += 1;
            assert!(assign66290_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (s.b[2570] && s.b[2571]) {
                s.store_sqrt(726, 726);
            }
            let (assign66290_body1_e103001,) = {
    if (s.b[2570] && s.b[2571]) {
        let assign66290_body1_e102999: f64 = (s.v[719] + 1.0);
        (assign66290_body1_e102999,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign66290_body1_e103001;
        }

        if (s.b[2570] && (!s.b[2571])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }

        if s.b[2570] {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, (10.0 * 2.220446049250313e-16));
            s.store_div_scaled_product_indices(334, 725, 726, (10.0 * 2.220446049250313e-16), 770, 1.0);
            s.store_sub_from_scalar(110, ((10.0 * 2.220446049250313e-16) + (10.0 * 2.220446049250313e-16)), 780);
        }

        if s.b[2570] {
        }

        if (!s.b[2570]) {
        }

        if (!s.b[2570]) {
            s.store_scalar(334, 1.0);
        }

        s.store_add(109, 87, 110);

    }

    pub(super) fn stamp_transient_block_67(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.store_add_scaled_product_left_ad(134, 134, 1.0, A::div_from_scalar(s.v[163], s.ad_value(162)), 790, p.p435);

        s.b[2576] = (p.p23 == 0.0);
        s.v[2576] = if s.b[2576] { 1.0 } else { 0.0 };

        if s.b[2576] {
            s.store_scalar(280, 0.0);
            s.store_scalar(288, 0.0);
        }

        s.b[2577] = ((s.v[481] > 0.0) && (s.v[454] > 0.0));
        s.v[2577] = if s.b[2577] { 1.0 } else { 0.0 };

        if ((!s.b[2576]) && s.b[2577]) {
            s.store_mul(335, 659, 85);
            s.store_scale(337, 636, 1.0 / ((s.v[188] * s.v[188])));
            s.store_scale_ad(338, A::div_from_scalar(2.0, s.ad_value(636)), (s.v[188] * s.v[188]));
            s.store_add_scaled_inputs_product_indices(339, 335, 1.0, 155, (-1.0), 660, 1434, (-1.0));
            s.store_offset_mul(340, 338, 339, 1.0);
            s.store_scaled_offset(341, 338, 1.0, 2.0);
        }

        s.b[2578] = ((s.v[340] < (1e-6 + s.v[341])) && (s.v[341] >= 0.0));
        s.v[2578] = if s.b[2578] { 1.0 } else { 0.0 };

        if (((!s.b[2576]) && s.b[2577]) && s.b[2578]) {
            s.store_sub_offset_lhs(781, 341, 1e-6, 340);
            s.store_square(722, 781);
            s.store_square(723, 341);
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
        }

        let (assign66560_e103242,) = {
    if (((!s.b[2576]) && s.b[2577]) && s.b[2578]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign66560_e103242;

        let (assign66570_e103251,) = {
    if (((!s.b[2576]) && s.b[2577]) && s.b[2578]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign66570_e103251;

        if (((!s.b[2576]) && s.b[2577]) && s.b[2578]) {
            s.store_scalar(770, 0.0);
            s.store_scalar(726, 0.0);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_add(770, 724, 725);
            s.copy_ad(726, 770);
        }

        s.b[2579] = ((((4.0 == 1.0) || (4.0 == 2.0)) || (4.0 == 4.0)) || (4.0 == 8.0));
        s.v[2579] = if s.b[2579] { 1.0 } else { 0.0 };

        s.b[2580] = (4.0 == 1.0);
        s.v[2580] = if s.b[2580] { 1.0 } else { 0.0 };

        let (assign66720_e103408,) = {
    if (((((!s.b[2576]) && s.b[2577]) && s.b[2578]) && s.b[2579]) && s.b[2580]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign66720_e103408;

        s.b[2581] = (4.0 == 2.0);
        s.v[2581] = if s.b[2581] { 1.0 } else { 0.0 };

        let (assign66740_e103427,) = {
    if ((((((!s.b[2576]) && s.b[2577]) && s.b[2578]) && s.b[2579]) && (!s.b[2580])) && s.b[2581]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign66740_e103427;

        s.b[2582] = (4.0 == 4.0);
        s.v[2582] = if s.b[2582] { 1.0 } else { 0.0 };

        let (assign66760_e103449,) = {
    if (((((((!s.b[2576]) && s.b[2577]) && s.b[2578]) && s.b[2579]) && (!s.b[2580])) && (!s.b[2581])) && s.b[2582]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign66760_e103449;

        s.b[2583] = (4.0 == 8.0);
        s.v[2583] = if s.b[2583] { 1.0 } else { 0.0 };

        let (assign66780_e103474,) = {
    if ((((((((!s.b[2576]) && s.b[2577]) && s.b[2578]) && s.b[2579]) && (!s.b[2580])) && (!s.b[2581])) && (!s.b[2582])) && s.b[2583]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign66780_e103474;

        let (assign66790_e103485,) = {
    if ((((!s.b[2576]) && s.b[2577]) && s.b[2578]) && s.b[2579]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign66790_e103485;

        let mut assign66800_loop_guard: usize = 0;
        while {
            let assign66800_cond_e103497: f64 = if (((((!s.b[2576]) && s.b[2577]) && s.b[2578]) && s.b[2579]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign66800_cond_e103497 != 0.0
        } {
            assign66800_loop_guard += 1;
            assert!(assign66800_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((!s.b[2576]) && s.b[2577]) && s.b[2578]) && s.b[2579]) {
                s.store_sqrt(726, 726);
            }
            let (assign66800_body1_e103522,) = {
    if ((((!s.b[2576]) && s.b[2577]) && s.b[2578]) && s.b[2579]) {
        let assign66800_body1_e103520: f64 = (s.v[719] + 1.0);
        (assign66800_body1_e103520,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign66800_body1_e103522;
        }

        if ((((!s.b[2576]) && s.b[2577]) && s.b[2578]) && (!s.b[2579])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 4.0)));
            }
        }

        if (((!s.b[2576]) && s.b[2577]) && s.b[2578]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_mul3_lhs(780, 781, 341, 726);
            s.store_div_scaled_product3_indices(334, 341, 725, 726, 1.0, 770, 1.0);
            s.store_sub_offset_lhs(340, 341, 1e-6, 780);
        }

        if (((!s.b[2576]) && s.b[2577]) && s.b[2578]) {
        }

        if (((!s.b[2576]) && s.b[2577]) && (!s.b[2578])) {
        }

        if (((!s.b[2576]) && s.b[2577]) && (!s.b[2578])) {
            s.store_scalar(334, 1.0);
        }

        if ((!s.b[2576]) && s.b[2577]) {
            s.store_sqrt(340, 340);
            s.store_add_ad_rhs(282, 335, A::mul_sub_from_scalar_rhs(s.ad_value(337), 1.0, s.ad_value(340)));
            s.store_div_from_scalar_offset_input(336, s.v[582], 661, s.v[582]);
            s.store_add_scaled_inputs_product_indices(283, 1435, s.v[483], 109, 1.0, 336, 282, (-1.0));
            s.store_sqrt_square_offset(782, 283, ((4.0 * 0.001) * 0.001));
            s.store_offset_scaled_div(343, 283, 782, 0.5, 0.5);
            s.store_scaled_add(283, 283, 782, 0.5);
        }

        s.b[2584] = (s.v[283] < 0.0);
        s.v[2584] = if s.b[2584] { 1.0 } else { 0.0 };

        if (((!s.b[2576]) && s.b[2577]) && s.b[2584]) {
            s.store_scalar(283, 0.0);
            s.store_scalar(343, 0.0);
        }

        if ((!s.b[2576]) && s.b[2577]) {
            s.store_offset(283, 283, 1e-25);
            s.store_offset_mul_offset_rhs(958, 957, 387, (-s.v[764]), 1.0);
        }

        if ((!s.b[2576]) && s.b[2577]) {
            if (s.v[958] <= 0.001) {
                s.store_scalar(958, 0.001);
            } else {
            }
        }

        if ((!s.b[2576]) && s.b[2577]) {
            s.store_div(339, 662, 958);
            s.store_mul(340, 663, 958);
            s.store_exp_ad(336, A::div_scaled_inputs(s.ad_value(340), -1.0, s.ad_value(283), 1.0));
            s.store_mul_product3_rhs(280, 336, s.ad_value(339), s.ad_value(283), s.ad_value(134), 1.0);
            s.store_mul3_lhs(288, 339, 283, 336);
        }

        if ((!s.b[2576]) && (!s.b[2577])) {
            s.store_scalar(280, 0.0);
        }

        s.b[2585] = (s.v[664] != 0.0);
        s.v[2585] = if s.b[2585] { 1.0 } else { 0.0 };

        if ((!s.b[2576]) && s.b[2585]) {
            s.copy_ad(334, 799);
            s.store_sqrt_square_offset(782, 334, ((4.0 * 1e-6) * 1e-6));
            s.store_offset_scaled_div(335, 334, 782, 0.5, 0.5);
            s.store_scaled_add(334, 334, 782, 0.5);
        }

        s.b[2586] = (s.v[334] < 0.0);
        s.v[2586] = if s.b[2586] { 1.0 } else { 0.0 };

        if (((!s.b[2576]) && s.b[2585]) && s.b[2586]) {
            s.store_scalar(334, 0.0);
            s.store_scalar(335, 0.0);
        }

        if ((!s.b[2576]) && s.b[2585]) {
            s.store_sqrt_offset_input(335, 127, 1e-25);
            s.store_div_from_scalar_scaled_input(337, 1.0, 335, 2.0);
            s.store_sub_ad_rhs(338, 334, A::scale_offset(s.ad_value(791), ((p.p106) * (p.p105)), p.p105));
            s.store_sqrt_square_offset(782, 338, ((4.0 * 0.01) * 0.01));
            s.store_offset_scaled_div(343, 338, 782, 0.5, 0.5);
            s.store_scaled_add(338, 338, 782, 0.5);
        }

        s.b[2587] = (s.v[338] < 0.0);
        s.v[2587] = if s.b[2587] { 1.0 } else { 0.0 };

        if (((!s.b[2576]) && s.b[2585]) && s.b[2587]) {
            s.store_scalar(338, 0.0);
            s.store_scalar(343, 0.0);
        }

        if ((!s.b[2576]) && s.b[2585]) {
            s.store_offset(338, 338, 1e-25);
            s.store_mul_ad_product_rhs(344, 450, s.ad_value(451), A::exp(A::div_from_scalar((-1.0), s.ad_value(338))));
            s.store_mul_offset_ad_rhs(345, 344, A::div_from_scalar(1.0, s.ad_value(338)), 1.0);
            s.store_mul(337, 338, 344);
            s.store_sub(334, 334, 337);
            s.store_sqrt_square_offset(782, 334, ((4.0 * 0.01) * 0.01));
            s.store_offset_scaled_div(343, 334, 782, 0.5, 0.5);
            s.store_scaled_add(334, 334, 782, 0.5);
        }

        s.b[2588] = (s.v[334] < 0.0);
        s.v[2588] = if s.b[2588] { 1.0 } else { 0.0 };

        if (((!s.b[2576]) && s.b[2585]) && s.b[2588]) {
            s.store_scalar(334, 0.0);
            s.store_scalar(343, 0.0);
        }

        if ((!s.b[2576]) && s.b[2585]) {
            s.store_offset(334, 334, 1e-25);
            s.store_div_from_scalar_mul_ad(338, 1.0, s.ad_value(334), s.ad_value(335));
            s.store_scalar(341, (s.v[165] * s.v[554]));
            s.store_exp_mul_scaled_lhs_indices(336, 341, -1.0, 338);
            s.store_mul_product3_rhs(340, 338, s.ad_value(341), s.ad_value(336), s.ad_value(338), 1.0);
            s.store_mul_product3_rhs(281, 336, s.ad_value(664), s.ad_value(134), s.ad_value(334), 1.0);
        }

        s.b[2589] = (p.p45 == 0.0);
        s.v[2589] = if s.b[2589] { 1.0 } else { 0.0 };

        if s.b[2589] {
            s.store_scalar(423, 0.0);
        }

        s.b[2590] = ((p.p45 * (s.v[796] - p.p446)) < 0.0);
        s.v[2590] = if s.b[2590] { 1.0 } else { 0.0 };

        if ((!s.b[2589]) && s.b[2590]) {
            s.copy_ad(426, 427);
        }

        if ((!s.b[2589]) && (!s.b[2590])) {
            s.store_add_scaled_inputs_ad_lhs(426, A::powf(A::offset(s.ad_value(796), (-p.p446)), 2.0), p.p445, 427, 1.0);
        }

        if (!s.b[2589]) {
            s.store_scaled_limited_exp_ad(423, A::mul(s.ad_value(154), A::sub(s.ad_value(793), s.ad_value(426))), p.p449);
        }

        s.b[2591] = (s.v[423] > 0.0);
        s.v[2591] = if s.b[2591] { 1.0 } else { 0.0 };

        s.b[2592] = ((s.v[423] > (100000.0 - 50000.0)) && (50000.0 >= 0.0));
        s.v[2592] = if s.b[2592] { 1.0 } else { 0.0 };

        if (s.b[2591] && s.b[2592]) {
            s.store_offset(781, 423, (((-100000.0)) + (50000.0)));
            s.store_square(722, 781);
            s.store_scalar(723, (50000.0 * 50000.0));
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
        }

    }

    pub(super) fn stamp_transient_block_68(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        let (assign67550_e104284,) = {
    if (s.b[2591] && s.b[2592]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign67550_e104284;

        let (assign67560_e104290,) = {
    if (s.b[2591] && s.b[2592]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign67560_e104290;

        if (s.b[2591] && s.b[2592]) {
            s.store_scalar(770, 0.0);
            s.store_scalar(726, 0.0);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_add(770, 724, 725);
            s.copy_ad(726, 770);
        }

        s.b[2593] = ((((1.0 == 1.0) || (1.0 == 2.0)) || (1.0 == 4.0)) || (1.0 == 8.0));
        s.v[2593] = if s.b[2593] { 1.0 } else { 0.0 };

        s.b[2594] = (1.0 == 1.0);
        s.v[2594] = if s.b[2594] { 1.0 } else { 0.0 };

        let (assign67650_e104360,) = {
    if (((s.b[2591] && s.b[2592]) && s.b[2593]) && s.b[2594]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign67650_e104360;

        s.b[2595] = (1.0 == 2.0);
        s.v[2595] = if s.b[2595] { 1.0 } else { 0.0 };

        let (assign67670_e104376,) = {
    if ((((s.b[2591] && s.b[2592]) && s.b[2593]) && (!s.b[2594])) && s.b[2595]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign67670_e104376;

        s.b[2596] = (1.0 == 4.0);
        s.v[2596] = if s.b[2596] { 1.0 } else { 0.0 };

        let (assign67690_e104395,) = {
    if (((((s.b[2591] && s.b[2592]) && s.b[2593]) && (!s.b[2594])) && (!s.b[2595])) && s.b[2596]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign67690_e104395;

        s.b[2597] = (1.0 == 8.0);
        s.v[2597] = if s.b[2597] { 1.0 } else { 0.0 };

        let (assign67710_e104417,) = {
    if ((((((s.b[2591] && s.b[2592]) && s.b[2593]) && (!s.b[2594])) && (!s.b[2595])) && (!s.b[2596])) && s.b[2597]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign67710_e104417;

        let (assign67720_e104425,) = {
    if ((s.b[2591] && s.b[2592]) && s.b[2593]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign67720_e104425;

        let mut assign67730_loop_guard: usize = 0;
        while {
            let assign67730_cond_e104434: f64 = if (((s.b[2591] && s.b[2592]) && s.b[2593]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign67730_cond_e104434 != 0.0
        } {
            assign67730_loop_guard += 1;
            assert!(assign67730_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((s.b[2591] && s.b[2592]) && s.b[2593]) {
                s.store_sqrt(726, 726);
            }
            let (assign67730_body1_e104453,) = {
    if ((s.b[2591] && s.b[2592]) && s.b[2593]) {
        let assign67730_body1_e104451: f64 = (s.v[719] + 1.0);
        (assign67730_body1_e104451,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign67730_body1_e104453;
        }

        if ((s.b[2591] && s.b[2592]) && (!s.b[2593])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / 2.0));
            }
        }

        if (s.b[2591] && s.b[2592]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, 50000.0);
            s.store_div_scaled_product_indices(334, 725, 726, 50000.0, 770, 1.0);
            s.store_offset(336, 780, (100000.0 - 50000.0));
        }

        if (s.b[2591] && s.b[2592]) {
        }

        if (s.b[2591] && (!s.b[2592])) {
            s.copy_ad(336, 423);
            s.store_scalar(334, 1.0);
        }

        if s.b[2591] {
            s.store_scale(422, 336, (s.v[365] * s.v[632]));
        }

        if (!s.b[2591]) {
            s.store_scalar(422, 0.0);
        }

        s.b[2598] = ((((s.v[280] + s.v[281]) > 0.0) && (s.v[523] != 0.0)) && (s.v[963] == 0.0));
        s.v[2598] = if s.b[2598] { 1.0 } else { 0.0 };

        if s.b[2598] {
            s.store_offset_scaled(334, 120, s.v[524], 1.0);
            s.store_add(335, 280, 281);
            s.store_scaled_mul(111, 334, 335, s.v[523]);
            s.store_div_from_scalar(344, 1.0, 99);
            s.store_mul3_lhs(335, 154, 111, 344);
            s.store_square(345, 344);
            s.store_div_from_scalar(344, 1.0, 102);
            s.store_mul3_lhs(336, 154, 111, 344);
            s.store_square(345, 344);
            s.store_mul_ad_rhs(112, 209, A::add_scaled_products(s.ad_value(104), s.ad_value(336), 1.0, s.ad_value(101), s.ad_value(335), (-1.0)));
            s.store_mul_add_scaled_products_indices_rhs(113, 209, 103, 336, ((-1.0) * (0.5)), 100, 335, 0.5);
            s.store_add(114, 112, 113);
            s.store_mul3_lhs(400, 115, 114, 253);
            s.store_mul(287, 288, 400);
        }

        s.b[2599] = (p.p24 != 0.0);
        s.v[2599] = if s.b[2599] { 1.0 } else { 0.0 };

        s.b[2600] = (s.v[78] == 0.0);
        s.v[2600] = if s.b[2600] { 1.0 } else { 0.0 };

        if (s.b[2599] && s.b[2600]) {
            s.store_offset_add(191, 109, 1435, (-(10.0 * 2.220446049250313e-16)));
            s.store_sub_scaled_ad_lhs(335, A::add_scaled_product(A::offset(s.ad_value(1436), (-s.v[160])), 1.0, A::sub(s.ad_value(120), s.ad_value(182)), s.ad_value(162), s.v[560]), 191, s.v[515]);
            s.store_square(335, 335);
            s.store_scalar(337, (1.0 / s.v[187]));
            s.store_mul(336, 335, 337);
            s.store_scalar(337, (1.0 / s.v[561]));
            s.store_offset_mul(341, 255, 337, 1.0);
            s.store_mul(195, 336, 341);
            s.store_sqrt_square_offset(782, 195, ((4.0 * (0.01 / 0.01)) * (0.01 / 0.01)));
            s.store_offset_scaled_div(339, 195, 782, 0.5, 0.5);
            s.store_scaled_add(195, 195, 782, 0.5);
        }

        s.b[2601] = (s.v[195] < 0.0);
        s.v[2601] = if s.b[2601] { 1.0 } else { 0.0 };

        if ((s.b[2599] && s.b[2600]) && s.b[2601]) {
            s.store_scalar(195, 0.0);
            s.store_scalar(339, 0.0);
        }

        if (s.b[2599] && s.b[2600]) {
            s.store_sqrt_square_offset(782, 1436, ((4.0 * 0.001) * 0.001));
            s.store_offset_scaled_div(338, 1436, 782, 0.5, 0.5);
            s.store_scaled_add(337, 1436, 782, 0.5);
        }

        s.b[2602] = (s.v[337] < 0.0);
        s.v[2602] = if s.b[2602] { 1.0 } else { 0.0 };

        if ((s.b[2599] && s.b[2600]) && s.b[2602]) {
            s.store_scalar(337, 0.0);
            s.store_scalar(338, 0.0);
        }

        if (s.b[2599] && s.b[2600]) {
            s.store_offset(337, 337, (-p.p262));
            s.store_scale(332, 337, 10.0);
            s.store_offset_square(336, 332, 1.0);
            s.store_sub_from_scalar_ad(335, 1.0, A::div_from_scalar(1.0, s.ad_value(336)));
            s.store_mul(195, 195, 335);
            s.store_scale(334, 162, s.v[632]);
            s.store_div_from_scalar_offset_input(341, s.v[562], 334, s.v[562]);
            s.store_scalar(340, s.v[516]);
            s.store_div_add_scaled_inputs_rhs_indices(343, 340, 340, 1.0, 1435, 1.0);
            s.store_div_from_scalar_offset_input(338, 1.0, 195, 1e-25);
            s.store_scaled_mul(335, 193, 338, (-s.v[514]));
            s.store_scaled_mul(337, 338, 338, s.v[514]);
        }

        s.b[2603] = (s.v[335] < (-34.0));
        s.v[2603] = if s.b[2603] { 1.0 } else { 0.0 };

        if ((s.b[2599] && s.b[2600]) && s.b[2603]) {
            s.store_scalar(199, 0.0);
        }

        if ((s.b[2599] && s.b[2600]) && (!s.b[2603])) {
            s.store_exp(336, 335);
            s.store_mul_scale_ad_lhs(337, A::div_from_scalar(s.v[513], s.ad_value(192)), 1.6021918e-19, 334);
            s.store_div_from_scalar(339, 1.0, 209);
            s.store_sqrt_ad(340, A::mul_offset_lhs(s.ad_value(978), (s.v[188] * 1e-12), s.ad_value(339)));
            s.store_mul3_lhs(338, 336, 337, 340);
            s.store_mul(339, 338, 195);
            s.store_mul(344, 339, 195);
            s.store_mul3_lhs(199, 341, 343, 344);
        }

        if s.b[2599] {
            s.store_offset_scaled(334, 791, (-s.v[518]), s.v[559]);
            s.store_exp_scaled_input(336, 334, s.v[187]);
            s.store_scale(334, 791, (1.0 / (s.v[187]) * 1.0 / (s.v[187])));
            s.store_mul(337, 791, 334);
            s.store_scalar(338, ((s.v[517] / 1000000.0) * s.v[632]));
            s.store_mul3_lhs(200, 338, 336, 337);
        }

        s.b[2604] = (s.v[791] >= 0.0);
        s.v[2604] = if s.b[2604] { 1.0 } else { 0.0 };

        if (s.b[2599] && s.b[2604]) {
            s.store_scale(200, 200, (-1.0));
        }

        if s.b[2599] {
            s.store_sub(335, 791, 790);
            s.store_offset_scaled(334, 335, (-s.v[518]), s.v[559]);
            s.store_exp_scaled_input(336, 334, s.v[187]);
            s.store_scale(334, 335, (1.0 / (s.v[187]) * 1.0 / (s.v[187])));
            s.store_mul(337, 335, 334);
            s.store_scalar(338, ((s.v[517] / 1000000.0) * s.v[632]));
            s.store_mul3_lhs(201, 338, 336, 337);
        }

        s.b[2605] = (s.v[335] >= 0.0);
        s.v[2605] = if s.b[2605] { 1.0 } else { 0.0 };

        if (s.b[2599] && s.b[2605]) {
            s.store_scale(201, 201, (-1.0));
        }

        if s.b[2599] {
            s.store_scaled_offset_ad(195, A::neg(A::sub(s.ad_value(791), s.ad_value(792))), ((s.v[160]) + (p.p258)), 1.0 / (s.v[187]));
            s.store_sqrt_square_offset(782, 195, ((4.0 * (0.01 / 0.01)) * (0.01 / 0.01)));
            s.store_offset_scaled_div(339, 195, 782, 0.5, 0.5);
            s.store_scaled_add(195, 195, 782, 0.5);
        }

        s.b[2606] = (s.v[195] < 0.0);
        s.v[2606] = if s.b[2606] { 1.0 } else { 0.0 };

        if (s.b[2599] && s.b[2606]) {
            s.store_scalar(195, 0.0);
            s.store_scalar(339, 0.0);
        }

        if s.b[2599] {
            s.store_offset(195, 195, 1e-25);
            s.store_div_from_scalar(335, (-s.v[520]), 195);
        }

        s.b[2607] = (s.v[335] < (-34.0));
        s.v[2607] = if s.b[2607] { 1.0 } else { 0.0 };

        if (s.b[2599] && s.b[2607]) {
            s.store_scalar(202, 0.0);
        }

        if (s.b[2599] && (!s.b[2607])) {
            s.store_exp(336, 335);
        }

    }

    pub(super) fn stamp_transient_block_69(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (s.b[2599] && (!s.b[2607])) {
            s.store_mul_div_from_scalar_ad_lhs(337, s.v[520], A::square(s.ad_value(195)), 336);
            s.store_scale(337, 162, (s.v[519] * s.v[632]));
            s.store_mul_product3_rhs(202, 336, s.ad_value(337), s.ad_value(195), s.ad_value(195), 1.0);
        }

        if s.b[2599] {
            s.copy_ad(285, 677);
            s.store_mul(286, 393, 285);
            s.store_scaled_offset_ad(336, A::add_scaled_inputs4(s.ad_value(1434), s.v[493], s.ad_value(1436), (-1.0), s.ad_value(122), 1.0, s.ad_value(174), 1.0), (-s.v[492]), (-1.0 / (s.v[187])));
            s.store_square(334, 336);
            s.store_scale(335, 286, s.v[491]);
            s.store_div_scaled_inputs(337, s.ad_value(335), -1.0, s.ad_value(336), 1.0);
        }

        s.b[2608] = (s.v[337] < (-34.0));
        s.v[2608] = if s.b[2608] { 1.0 } else { 0.0 };

        if (s.b[2599] && s.b[2608]) {
            s.store_scalar(339, 0.0);
        }

        if (s.b[2599] && (!s.b[2608])) {
            s.store_exp(339, 337);
        }

        if s.b[2599] {
            s.store_div_from_scalar(338, (((1.6021918e-19 * s.v[490]) * s.v[632]) * s.v[582]), 285);
        }

        s.b[2609] = (((2.0 * s.v[336]) + s.v[335]) < 0.0);
        s.v[2609] = if s.b[2609] { 1.0 } else { 0.0 };

        if (s.b[2599] && s.b[2609]) {
            s.store_mul3_affine_lhs(284, 338, 335, (0.25 * 7.38905609893065), 0.0, 335);
        }

        if (s.b[2599] && (!s.b[2609])) {
            s.store_mul3_lhs(284, 338, 334, 339);
        }

        if s.b[2599] {
            s.store_sub(202, 202, 284);
        }

        s.b[2610] = (p.p25 != 0.0);
        s.v[2610] = if s.b[2610] { 1.0 } else { 0.0 };

        if s.b[2610] {
            s.store_offset_ad(335, A::mul_sub_from_scalar_rhs(s.ad_value(790), 1.0, A::scale(s.ad_value(790), 100.0)), (-1e-5));
            s.store_sqrt_add_scaled_square_input(336, 335, 1.0, 790, (4.0 * 1e-5));
            s.store_add_scaled_inputs3_indices(196, 790, 1.0, 335, (-0.5), 336, (-0.5));
        }

        s.b[2611] = (p.p25 == 0.0);
        s.v[2611] = if s.b[2611] { 1.0 } else { 0.0 };

        if s.b[2611] {
            s.store_scalar(203, 0.0);
        }

        if (!s.b[2611]) {
            s.store_add_scaled_inputs4_offset_indices(335, 196, p.p242, 791, (-1.0), 122, p.p244, 174, p.p244, (p.p243 * p.p242));
            s.store_scalar(336, (1.0 / s.v[187]));
            s.store_mul(194, 335, 336);
            s.store_sqrt_square_offset(782, 194, ((4.0 * (0.01 / 0.01)) * (0.01 / 0.01)));
            s.store_offset_scaled_div(339, 194, 782, 0.5, 0.5);
            s.store_scaled_add(197, 194, 782, 0.5);
        }

        s.b[2612] = (s.v[197] < 0.0);
        s.v[2612] = if s.b[2612] { 1.0 } else { 0.0 };

        if ((!s.b[2611]) && s.b[2612]) {
            s.store_scalar(197, 0.0);
            s.store_scalar(339, 0.0);
        }

        if (!s.b[2611]) {
            s.store_div_from_scalar_offset_input(337, 1.0, 197, 1e-25);
            s.store_scaled_mul(334, 193, 337, (-s.v[512]));
        }

        s.b[2613] = (s.v[334] < (-34.0));
        s.v[2613] = if s.b[2613] { 1.0 } else { 0.0 };

        if ((!s.b[2611]) && s.b[2613]) {
            s.store_scalar(203, 0.0);
        }

        if ((!s.b[2611]) && (!s.b[2613])) {
            s.store_exp(335, 334);
            s.store_scale_ad(336, A::div_from_scalar(s.v[511], s.ad_value(192)), (1.6021918e-19 * s.v[632]));
            s.store_mul_product3_rhs(203, 335, s.ad_value(336), s.ad_value(197), s.ad_value(197), 1.0);
        }

        if (!s.b[2611]) {
            s.store_sub(205, 790, 792);
        }

        s.b[2614] = (s.v[205] > 0.0);
        s.v[2614] = if s.b[2614] { 1.0 } else { 0.0 };

        if ((!s.b[2611]) && s.b[2614]) {
            s.store_square(336, 205);
            s.store_mul(338, 336, 205);
            s.store_offset(334, 338, 0.5);
            s.store_div(339, 338, 334);
            s.store_div_ad(341, A::add_scaled_products(s.ad_value(336), s.ad_value(334), 3.0, s.ad_value(338), s.ad_value(336), (-3.0)), A::square(s.ad_value(334)));
            s.store_mul(203, 203, 339);
        }

        if ((!s.b[2611]) && (!s.b[2614])) {
            s.store_scalar(203, 0.0);
        }

        s.b[2615] = (p.p25 == 0.0);
        s.v[2615] = if s.b[2615] { 1.0 } else { 0.0 };

        if s.b[2615] {
            s.store_scalar(204, 0.0);
        }

        if (!s.b[2615]) {
            s.store_add_scaled_inputs3_mixed_aii(335, A::add_scaled_inputs3_offset(s.ad_value(196), (-p.p242), s.ad_value(791), -1.0, s.ad_value(196), 1.0, ((p.p243) * (p.p242))), 1.0, 122, p.p244, 174, p.p244);
            s.store_scalar(336, (1.0 / s.v[187]));
            s.store_mul(194, 335, 336);
            s.store_sqrt_square_offset(782, 194, ((4.0 * (0.01 / 0.01)) * (0.01 / 0.01)));
            s.store_offset_scaled_div(339, 194, 782, 0.5, 0.5);
            s.store_scaled_add(198, 194, 782, 0.5);
        }

        s.b[2616] = (s.v[198] < 0.0);
        s.v[2616] = if s.b[2616] { 1.0 } else { 0.0 };

        if ((!s.b[2615]) && s.b[2616]) {
            s.store_scalar(198, 0.0);
            s.store_scalar(339, 0.0);
        }

        if (!s.b[2615]) {
            s.store_div_from_scalar_offset_input(337, 1.0, 198, 1e-25);
            s.store_scaled_mul(334, 193, 337, (-s.v[512]));
        }

        s.b[2617] = (s.v[334] < (-34.0));
        s.v[2617] = if s.b[2617] { 1.0 } else { 0.0 };

        if ((!s.b[2615]) && s.b[2617]) {
            s.store_scalar(204, 0.0);
        }

        if ((!s.b[2615]) && (!s.b[2617])) {
            s.store_exp(335, 334);
            s.store_div_from_scalar(337, 1.0, 192);
            s.store_scale(336, 337, (s.v[511] * (1.6021918e-19 * s.v[632])));
            s.store_mul_product3_rhs(204, 335, s.ad_value(336), s.ad_value(198), s.ad_value(198), 1.0);
        }

        if (!s.b[2615]) {
            s.store_neg(206, 792);
        }

        s.b[2618] = (s.v[206] > 0.0);
        s.v[2618] = if s.b[2618] { 1.0 } else { 0.0 };

        if ((!s.b[2615]) && s.b[2618]) {
            s.store_square(336, 206);
            s.store_mul(338, 336, 206);
            s.store_offset(334, 338, 0.5);
            s.store_div(339, 338, 334);
            s.store_div_ad(341, A::add_scaled_products(s.ad_value(336), s.ad_value(334), 3.0, s.ad_value(338), s.ad_value(336), (-3.0)), A::square(s.ad_value(334)));
            s.store_mul(204, 204, 339);
        }

        if ((!s.b[2615]) && (!s.b[2618])) {
            s.store_scalar(204, 0.0);
        }

        s.v[2619] = 0.0;

        s.v[2622] = 0.0;

        s.v[2621] = 0.0;

        s.v[406] = 0.0;

        s.v[2621] = 0.0;

        s.b[2623] = (1.0 == 1.0);
        s.v[2623] = if s.b[2623] { 1.0 } else { 0.0 };

        s.b[2624] = (1.0 == 2.0);
        s.v[2624] = if s.b[2624] { 1.0 } else { 0.0 };

        s.b[2625] = (1.0 == 3.0);
        s.v[2625] = if s.b[2625] { 1.0 } else { 0.0 };

        s.b[2626] = (1.0 == 4.0);
        s.v[2626] = if s.b[2626] { 1.0 } else { 0.0 };

        s.b[2627] = (((p.p36 == 1.0) && (p.p66 > 0.0)) && (s.v[460] > 0.0));
        s.v[2627] = if s.b[2627] { 1.0 } else { 0.0 };

        let (assign69590_e106033,) = {
    if (s.b[2623] && s.b[2627]) {
        (1.0,)
    } else {
        (s.v[2621],)
    }
};
        s.v[2621] = assign69590_e106033;

        let (assign69600_e106039,) = {
    if (s.b[2623] && s.b[2627]) {
        (1.0,)
    } else {
        (s.v[2619],)
    }
};
        s.v[2619] = assign69600_e106039;

        if (s.b[2623] && s.b[2627]) {
            s.store_sub(395, 731, 728);
            s.store_neg(396, 728);
            s.store_scalar(409, s.v[460]);
            s.store_scalar(407, p.p66);
            s.store_scalar(411, 0.0);
            s.copy_ad(410, 687);
            s.store_scalar(413, s.v[188]);
        }

        s.b[2628] = (((((p.p36 == 1.0) && (p.p66 > 0.0)) && (s.v[460] > 0.0)) && (s.v[526] != 0.0)) && (p.p55 != 1.0));
        s.v[2628] = if s.b[2628] { 1.0 } else { 0.0 };

        let (assign69690_e106112,) = {
    if ((s.b[2624] && (!s.b[2623])) && s.b[2628]) {
        (1.0,)
    } else {
        (s.v[2621],)
    }
};
        s.v[2621] = assign69690_e106112;

        if ((s.b[2624] && (!s.b[2623])) && s.b[2628]) {
            s.store_sub(395, 734, 735);
            s.store_neg(396, 735);
        }

        s.b[2629] = (((p.p35 == 1.0) && (p.p63 > 0.0)) && (s.v[459] > 0.0));
        s.v[2629] = if s.b[2629] { 1.0 } else { 0.0 };

        let (assign69730_e106155,) = {
    if ((s.b[2625] && (!(s.b[2623] || s.b[2624]))) && s.b[2629]) {
        (1.0,)
    } else {
        (s.v[2621],)
    }
};
        s.v[2621] = assign69730_e106155;

        let (assign69740_e106166,) = {
    if ((s.b[2625] && (!(s.b[2623] || s.b[2624]))) && s.b[2629]) {
        (1.0,)
    } else {
        (s.v[2622],)
    }
};
        s.v[2622] = assign69740_e106166;

        if ((s.b[2625] && (!(s.b[2623] || s.b[2624]))) && s.b[2629]) {
            s.store_sub(395, 731, 728);
            s.store_sub(396, 729, 728);
            s.store_scalar(409, s.v[459]);
            s.store_scalar(407, (p.p63 + (p.p64 * p.p55)));
            s.copy_ad(411, 384);
            s.copy_ad(410, 686);
            s.copy_ad(413, 412);
            s.store_neg(407, 407);
        }

        s.b[2630] = (((s.v[407] < 0.0) && (p.p432 > 0.0)) && (p.p55 == 1.0));
        s.v[2630] = if s.b[2630] { 1.0 } else { 0.0 };

        if (((s.b[2625] && (!(s.b[2623] || s.b[2624]))) && s.b[2629]) && s.b[2630]) {
            s.store_neg(407, 407);
            s.store_scalar(335, p.p63);
            s.store_offset_div_scaled_product(996, s.ad_value(335), s.ad_value(335), 1.0, s.ad_value(651), 1.0, (-p.p137));
        }

        s.b[2631] = (p.p113 > 0.0);
        s.v[2631] = if s.b[2631] { 1.0 } else { 0.0 };

        s.b[2632] = ((s.v[396] == 0.0) || (p.p113 <= 0.0));
        s.v[2632] = if s.b[2632] { 1.0 } else { 0.0 };

        if (((((s.b[2625] && (!(s.b[2623] || s.b[2624]))) && s.b[2629]) && s.b[2630]) && s.b[2631]) && s.b[2632]) {
        }

        if (((((s.b[2625] && (!(s.b[2623] || s.b[2624]))) && s.b[2629]) && s.b[2630]) && s.b[2631]) && (!s.b[2632])) {
            s.store_scalar(783, (if (s.v[396] < 0.0) { (-1.0) } else { 1.0 }));
        }

        if (((((s.b[2625] && (!(s.b[2623] || s.b[2624]))) && s.b[2629]) && s.b[2630]) && s.b[2631]) && (!s.b[2632])) {
            s.store_mul(784, 783, 396);
            s.store_offset_powf_ad(781, A::div(s.ad_value(784), s.ad_value(996)), p.p113, 1.0);
            s.store_powf(782, 781, (1.0 / p.p113));
            s.store_div_scaled_product_indices(396, 783, 784, 1.0, 782, 1.0);
        }

        if ((((s.b[2625] && (!(s.b[2623] || s.b[2624]))) && s.b[2629]) && s.b[2630]) && s.b[2631]) {
            s.store_sqrt_offset_ad(782, A::mul_offset_lhs(s.ad_value(396), p.p137, A::offset(s.ad_value(396), p.p137)), ((4.0 * 0.1) * 0.1));
            s.store_scaled_offset_ad(343, A::div_scaled_offset_numerator(s.ad_value(396), 1.0, p.p137, s.ad_value(782), 1.0), 1.0, 0.5);
            s.store_scaled_add_ad_lhs(336, A::offset(s.ad_value(396), p.p137), 782, 0.5);
        }

        s.b[2633] = (s.v[336] < 0.0);
        s.v[2633] = if s.b[2633] { 1.0 } else { 0.0 };

        if (((((s.b[2625] && (!(s.b[2623] || s.b[2624]))) && s.b[2629]) && s.b[2630]) && s.b[2631]) && s.b[2633]) {
            s.store_scalar(336, 0.0);
            s.store_scalar(343, 0.0);
        }

        if ((((s.b[2625] && (!(s.b[2623] || s.b[2624]))) && s.b[2629]) && s.b[2630]) && s.b[2631]) {
            s.store_scaled_sqrt_ad(600, A::mul(s.ad_value(651), s.ad_value(336)), p.p432);
            s.store_sub(407, 407, 600);
        }

    }

    pub(super) fn stamp_transient_block_70(
        s: &mut Scratch,
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        s.b[2634] = (((((p.p35 == 1.0) && (p.p63 > 0.0)) && (s.v[459] > 0.0)) && (s.v[526] != 0.0)) && (p.p55 != 1.0));
        s.v[2634] = if s.b[2634] { 1.0 } else { 0.0 };

        let (assign70040_e106637,) = {
    if ((s.b[2626] && (!((s.b[2623] || s.b[2624]) || s.b[2625]))) && s.b[2634]) {
        (1.0,)
    } else {
        (s.v[2621],)
    }
};
        s.v[2621] = assign70040_e106637;

        if ((s.b[2626] && (!((s.b[2623] || s.b[2624]) || s.b[2625]))) && s.b[2634]) {
            s.store_sub(395, 734, 735);
            s.store_sub(396, 733, 735);
        }

        if (s.v[2621] != 0.0) {
            s.store_scalar(2642, 0.4);
        }

        let (assign70090_e106679,) = {
    if (s.v[2621] != 0.0) {
        (0.0,)
    } else {
        (s.v[2643],)
    }
};
        s.v[2643] = assign70090_e106679;

        if (s.v[2621] != 0.0) {
            s.store_scalar(223, 0.0);
            s.store_scalar(214, 0.0);
            s.store_scalar(216, 0.0);
            s.store_scalar(232, 0.0);
            s.store_scalar(236, 0.0);
            s.store_scalar(233, 0.0);
            s.store_scalar(217, 0.0);
            s.store_scalar(420, 0.0);
            s.store_scalar(215, 0.0);
            s.store_scalar(447, 0.0);
            s.store_scalar(445, 0.0);
            s.store_scalar(446, 0.0);
        }

        let (assign70220_e106732,) = {
    if (s.v[2621] != 0.0) {
        let assign70220_e106730: f64 = (-1.0);
        (assign70220_e106730,)
    } else {
        (s.v[79],)
    }
};
        s.v[79] = assign70220_e106732;

        if (s.v[2621] != 0.0) {
            s.store_scalar(2644, 0.0);
            s.store_scalar(2645, 0.0);
            s.store_mul_scaled_ln_ad_rhs(2640, 155, 2.0, A::div(s.ad_value(409), s.ad_value(394)));
            s.store_offset_sub_from_scalar_ad(781, 0.8, s.ad_value(2640), (-0.1));
            s.store_scalar(782, ((4.0 * 0.8) * 0.1));
        }

        if (s.v[2621] != 0.0) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if (s.v[2621] != 0.0) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(2641, 781, (-0.5), 782, (-0.5), 0.8);
        }

        s.b[2647] = (s.v[2642] > (s.v[2641] * 0.5));
        s.v[2647] = if s.b[2647] { 1.0 } else { 0.0 };

        if ((s.v[2621] != 0.0) && s.b[2647]) {
            s.store_scale(2642, 2641, 0.5);
        }

        s.b[2648] = param_given[338];
        s.v[2648] = if s.b[2648] { 1.0 } else { 0.0 };

        if ((s.v[2621] != 0.0) && s.b[2648]) {
            s.store_scalar(2641, p.p338);
        }

        s.b[2649] = param_given[339];
        s.v[2649] = if s.b[2649] { 1.0 } else { 0.0 };

        if ((s.v[2621] != 0.0) && s.b[2649]) {
            s.store_scalar(2642, p.p339);
        }

        s.b[2650] = param_given[338];
        s.v[2650] = if s.b[2650] { 1.0 } else { 0.0 };

        if (((s.v[2621] != 0.0) && (!s.b[2649])) && s.b[2650]) {
            s.store_scale(2642, 2641, 0.5);
        }

        s.b[2651] = (s.v[2642] > (s.v[2641] * 0.5));
        s.v[2651] = if s.b[2651] { 1.0 } else { 0.0 };

        if ((s.v[2621] != 0.0) && s.b[2651]) {
            s.store_scale(2642, 2641, 0.5);
        }

        s.b[2652] = (p.p38 == 1.0);
        s.v[2652] = if s.b[2652] { 1.0 } else { 0.0 };

        if ((s.v[2621] != 0.0) && s.b[2652]) {
            s.store_neg(334, 396);
        }

        s.b[2653] = (s.v[334] > s.v[2642]);
        s.v[2653] = if s.b[2653] { 1.0 } else { 0.0 };

        if (((s.v[2621] != 0.0) && s.b[2652]) && s.b[2653]) {
            s.store_sub(335, 334, 2642);
            s.store_sub(336, 2641, 2642);
            s.store_div(781, 335, 336);
            s.store_square(782, 781);
            s.store_mul(783, 782, 781);
            s.store_square(784, 782);
            s.store_div_from_scalar_ad(780, 1.0, A::add_scaled_inputs4_offset(s.ad_value(781), 1.0, s.ad_value(782), 1.0, s.ad_value(783), 1.0, s.ad_value(784), 1.0, 1.0));
            s.store_mul_ad_affine_product_lhs(345, A::add_scaled_inputs3_offset(s.ad_value(781), 2.0, s.ad_value(782), 3.0, s.ad_value(783), 4.0, 1.0), s.ad_value(780), -1.0, 0.0, 780);
            s.store_mul_sub_from_scalar_rhs(333, 336, 1.0, 780);
            s.store_add_scaled_sub_value_product_indices(334, 1.0, 780, 1.0, 781, 345, 1.0);
            s.store_neg(345, 345);
            s.store_add(344, 2642, 333);
        }

        if (((s.v[2621] != 0.0) && s.b[2652]) && (!s.b[2653])) {
            s.copy_ad(344, 334);
        }

        if ((s.v[2621] != 0.0) && s.b[2652]) {
            s.store_neg(397, 344);
        }

        if ((s.v[2621] != 0.0) && (!s.b[2652])) {
            s.copy_ad(397, 396);
        }

        if (s.v[2621] != 0.0) {
            s.store_div(212, 410, 413);
            s.store_square(213, 212);
            s.store_sub_from_scalar(402, s.v[458], 395);
        }

        let (assign70630_e107073,) = {
    if (s.v[2621] != 0.0) {
        let assign70630_e107067: f64 = (-s.v[397]);
        let assign70630_e107070: f64 = (10.0 * 2.220446049250313e-16);
        let assign70630_e107071: f64 = (assign70630_e107067 + assign70630_e107070);
        (assign70630_e107071,)
    } else {
        (s.v[403],)
    }
};
        s.v[403] = assign70630_e107073;

        if (s.v[2621] != 0.0) {
            s.store_scalar(2636, 0.0);
            s.store_scale(2637, 409, 1.6021918e-19);
            s.store_div(334, 394, 409);
            s.store_square(405, 334);
        }

        s.b[2654] = ((s.v[154] * (-s.v[397])) >= 500.0);
        s.v[2654] = if s.b[2654] { 1.0 } else { 0.0 };

        if ((s.v[2621] != 0.0) && s.b[2654]) {
            s.store_scaled_offset_ad(229, A::mul_scaled_rhs(s.ad_value(154), s.ad_value(397), -1.0), ((1.0) + ((-500.0))), 1.403592217853e217);
            s.store_scalar(334, 1.403592217853e217);
        }

        if ((s.v[2621] != 0.0) && (!s.b[2654])) {
            s.store_mul_neg_rhs(781, 154, 397);
            s.store_scalar(229, 1.0);
        }

        let mut assign70730_loop_guard: usize = 0;
        while {
            let assign70730_cond_e107147: f64 = if (((s.v[2621] != 0.0) && (!s.b[2654])) && (s.v[781] >= 60.0)) { 1.0 } else { 0.0 };
            assign70730_cond_e107147 != 0.0
        } {
            assign70730_loop_guard += 1;
            assert!(assign70730_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((s.v[2621] != 0.0) && (!s.b[2654])) {
                s.store_scale(229, 229, 1.14200738981568e26);
                s.store_offset(781, 781, (-60.0));
            }
        }

        if ((s.v[2621] != 0.0) && (!s.b[2654])) {
            s.store_mul_exp_rhs(229, 229, 781);
            s.copy_ad(334, 229);
        }

        if ((s.v[2621] != 0.0) && (s.v[406] != 0.0)) {
            s.store_offset_scaled(781, 402, (-0.5), (((-0.5)) + ((-1.0))));
            s.store_scalar(782, (4.0 * 0.5));
        }

        if ((s.v[2621] != 0.0) && (s.v[406] != 0.0)) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if ((s.v[2621] != 0.0) && (s.v[406] != 0.0)) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(335, 781, 0.5, 782, 0.5, 0.5);
        }

        s.b[2655] = (((s.v[402] + s.v[397]) > (-s.v[335])) && (s.v[335] >= 0.0));
        s.v[2655] = if s.b[2655] { 1.0 } else { 0.0 };

        if (((s.v[2621] != 0.0) && (s.v[406] != 0.0)) && s.b[2655]) {
            s.store_add_scaled_inputs3_indices(781, 402, 1.0, 397, 1.0, 335, 1.0);
            s.store_square(722, 781);
            s.store_square(723, 335);
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
        }

        let (assign70880_e107321,) = {
    if (((s.v[2621] != 0.0) && (s.v[406] != 0.0)) && s.b[2655]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign70880_e107321;

        let (assign70890_e107329,) = {
    if (((s.v[2621] != 0.0) && (s.v[406] != 0.0)) && s.b[2655]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign70890_e107329;

        if (((s.v[2621] != 0.0) && (s.v[406] != 0.0)) && s.b[2655]) {
            s.store_scalar(770, 0.0);
            s.store_scalar(726, 0.0);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_add(770, 724, 725);
            s.copy_ad(726, 770);
        }

        s.b[2656] = ((((1.0 == 1.0) || (1.0 == 2.0)) || (1.0 == 4.0)) || (1.0 == 8.0));
        s.v[2656] = if s.b[2656] { 1.0 } else { 0.0 };

        s.b[2657] = (1.0 == 1.0);
        s.v[2657] = if s.b[2657] { 1.0 } else { 0.0 };

        let (assign70980_e107413,) = {
    if (((((s.v[2621] != 0.0) && (s.v[406] != 0.0)) && s.b[2655]) && s.b[2656]) && s.b[2657]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign70980_e107413;

        s.b[2658] = (1.0 == 2.0);
        s.v[2658] = if s.b[2658] { 1.0 } else { 0.0 };

        let (assign71000_e107431,) = {
    if ((((((s.v[2621] != 0.0) && (s.v[406] != 0.0)) && s.b[2655]) && s.b[2656]) && (!s.b[2657])) && s.b[2658]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign71000_e107431;

        s.b[2659] = (1.0 == 4.0);
        s.v[2659] = if s.b[2659] { 1.0 } else { 0.0 };

        let (assign71020_e107452,) = {
    if (((((((s.v[2621] != 0.0) && (s.v[406] != 0.0)) && s.b[2655]) && s.b[2656]) && (!s.b[2657])) && (!s.b[2658])) && s.b[2659]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign71020_e107452;

        s.b[2660] = (1.0 == 8.0);
        s.v[2660] = if s.b[2660] { 1.0 } else { 0.0 };

        let (assign71040_e107476,) = {
    if ((((((((s.v[2621] != 0.0) && (s.v[406] != 0.0)) && s.b[2655]) && s.b[2656]) && (!s.b[2657])) && (!s.b[2658])) && (!s.b[2659])) && s.b[2660]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign71040_e107476;

        let (assign71050_e107486,) = {
    if ((((s.v[2621] != 0.0) && (s.v[406] != 0.0)) && s.b[2655]) && s.b[2656]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign71050_e107486;

        let mut assign71060_loop_guard: usize = 0;
        while {
            let assign71060_cond_e107497: f64 = if (((((s.v[2621] != 0.0) && (s.v[406] != 0.0)) && s.b[2655]) && s.b[2656]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign71060_cond_e107497 != 0.0
        } {
            assign71060_loop_guard += 1;
            assert!(assign71060_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.v[2621] != 0.0) && (s.v[406] != 0.0)) && s.b[2655]) && s.b[2656]) {
                s.store_sqrt(726, 726);
            }
            let (assign71060_body1_e107520,) = {
    if ((((s.v[2621] != 0.0) && (s.v[406] != 0.0)) && s.b[2655]) && s.b[2656]) {
        let assign71060_body1_e107518: f64 = (s.v[719] + 1.0);
        (assign71060_body1_e107518,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign71060_body1_e107520;
        }

        if ((((s.v[2621] != 0.0) && (s.v[406] != 0.0)) && s.b[2655]) && (!s.b[2656])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / 2.0));
            }
        }

        if (((s.v[2621] != 0.0) && (s.v[406] != 0.0)) && s.b[2655]) {
            s.store_div_from_scalar(726, 1.0, 726);
        }

    }

    pub(super) fn stamp_transient_block_71(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((s.v[2621] != 0.0) && (s.v[406] != 0.0)) && s.b[2655]) {
            s.store_mul3_lhs(780, 781, 335, 726);
            s.store_div_scaled_product3_indices(334, 335, 725, 726, 1.0, 770, 1.0);
            s.store_add_scaled_inputs(335, 335, -1.0, 780, 1.0);
        }

        if (((s.v[2621] != 0.0) && (s.v[406] != 0.0)) && s.b[2655]) {
        }

        if (((s.v[2621] != 0.0) && (s.v[406] != 0.0)) && (!s.b[2655])) {
            s.store_add(335, 402, 397);
            s.store_scalar(334, 1.0);
        }

        if ((s.v[2621] != 0.0) && (s.v[406] != 0.0)) {
            s.store_sub(397, 335, 402);
        }

        let (assign71160_e107637,) = {
    if ((s.v[2621] != 0.0) && (s.v[406] != 0.0)) {
        let assign71160_e107631: f64 = (-s.v[397]);
        let assign71160_e107634: f64 = (10.0 * 2.220446049250313e-16);
        let assign71160_e107635: f64 = (assign71160_e107631 + assign71160_e107634);
        (assign71160_e107635,)
    } else {
        (s.v[403],)
    }
};
        s.v[403] = assign71160_e107637;

        s.b[2661] = (s.v[402] < s.v[403]);
        s.v[2661] = if s.b[2661] { 1.0 } else { 0.0 };

        if ((s.v[2621] != 0.0) && s.b[2661]) {
            s.store_mul_scaled_ln_ad_rhs(271, 155, 2.0, A::div_from_scalar((-s.v[270]), s.ad_value(212)));
            s.store_mul_add_rhs(332, 154, 402, 397);
            s.store_div_from_scalar_mul_ad(335, 1.0, s.ad_value(154), s.ad_value(410));
            s.store_mul(333, 335, 413);
            s.store_offset_scaled(279, 333, (3.0 * 1.414213562373095), 2.0);
            s.store_mul3_affine_lhs(277, 279, 279, 8.0, 0.0, 279);
            s.store_sub_from_scalar_scaled_mul_ad_rhs(278, (7.0 * 1.414213562373095), 333, A::offset(s.ad_value(332), (-2.0)), 9.0);
            s.store_square(276, 278);
        }

        s.b[2662] = (s.v[277] < (s.v[276] * 1e-8));
        s.v[2662] = if s.b[2662] { 1.0 } else { 0.0 };

        if (((s.v[2621] != 0.0) && s.b[2661]) && s.b[2662]) {
            s.store_div_scaled_inputs(274, s.ad_value(277), 0.5, s.ad_value(278), 1.0);
        }

        if (((s.v[2621] != 0.0) && s.b[2661]) && (!s.b[2662])) {
            s.store_sqrt_add(275, 277, 276);
            s.store_sub(274, 275, 278);
        }

        if ((s.v[2621] != 0.0) && s.b[2661]) {
            s.store_powf(273, 274, 0.3333333333333333);
            s.store_add_scaled_inputs_product_first_ad(272, A::sub_from_scalar(((-4.0) * 1.414213562373095), A::scale(s.ad_value(333), 12.0)), 1.0, 273, 2.0, 273, 273, 1.414213562373095);
            s.store_div(116, 272, 273);
            s.store_mul(335, 116, 155);
            s.store_div(336, 335, 271);
            s.store_sqrt_square_offset(337, 336, 1.0);
            s.store_sub_ad_lhs(404, A::div(s.ad_value(335), s.ad_value(337)), 397);
            s.store_sub(336, 402, 404);
            s.store_mul(398, 413, 336);
            s.copy_ad(354, 398);
            s.copy_ad(2644, 404);
        }

        s.b[2663] = ((s.v[154] * (s.v[402] + s.v[397])) < (1.0 + (((((10.0 * 2.220446049250313e-16) - 1.0) * s.v[213]) * s.v[156]) / 4.0)));
        s.v[2663] = if s.b[2663] { 1.0 } else { 0.0 };

        if (((s.v[2621] != 0.0) && (!s.b[2661])) && s.b[2663]) {
            s.store_add_scaled_product_indices(89, 402, 1.0, 213, 154, 0.5);
        }

        if (((s.v[2621] != 0.0) && (!s.b[2661])) && (!s.b[2663])) {
            s.store_offset_div_scaled_offset_numerator(332, A::mul(s.ad_value(154), A::add(s.ad_value(402), s.ad_value(397))), 4.0, ((-1.0) * 4.0), A::mul(s.ad_value(213), s.ad_value(156)), 1.0, 1.0);
            s.store_add_ad_rhs(89, 402, A::mul3_scaled_output(s.ad_value(213), s.ad_value(154), A::sub_from_scalar(1.0, A::sqrt(s.ad_value(332))), 1.0 / (2.0)));
        }

        if ((s.v[2621] != 0.0) && (!s.b[2661])) {
            s.store_mul_add_rhs(116, 154, 89, 397);
        }

        s.b[2664] = (s.v[116] >= 3.0);
        s.v[2664] = if s.b[2664] { 1.0 } else { 0.0 };

        if (((s.v[2621] != 0.0) && (!s.b[2661])) && s.b[2664]) {
            s.store_exp_neg_input(333, 116);
            s.store_offset_div_scaled_inputs2(332, A::offset(A::mul(s.ad_value(154), A::add(s.ad_value(402), s.ad_value(397))), (-1.0)), 4.0, s.ad_value(333), 4.0, A::mul(s.ad_value(213), s.ad_value(156)), 1.0, 1.0);
            s.store_add_ad_rhs(89, 402, A::mul3_scaled_output(s.ad_value(213), s.ad_value(154), A::sub_from_scalar(1.0, A::sqrt(s.ad_value(332))), 1.0 / (2.0)));
            s.store_mul_add_rhs(116, 154, 89, 397);
            s.store_exp_neg_input(333, 116);
            s.store_offset_div_scaled_inputs2(332, A::offset(A::mul(s.ad_value(154), A::add(s.ad_value(402), s.ad_value(397))), (-1.0)), 4.0, s.ad_value(333), 4.0, A::mul(s.ad_value(213), s.ad_value(156)), 1.0, 1.0);
            s.store_add_ad_rhs(89, 402, A::mul3_scaled_output(s.ad_value(213), s.ad_value(154), A::sub_from_scalar(1.0, A::sqrt(s.ad_value(332))), 1.0 / (2.0)));
            s.store_mul_add_rhs(116, 154, 89, 397);
        }

        if (((s.v[2621] != 0.0) && (!s.b[2661])) && (!s.b[2664])) {
            s.store_scalar(434, ((1.0 / (9.0 * ((2.0) as f64).sqrt())) - ((5.0 + (7.0 * (((-3.0)) as f64).exp())) / (54.0 * (((2.0 + (((-3.0)) as f64).exp())) as f64).sqrt()))));
            s.store_scalar(435, (((1.0 + (((-3.0)) as f64).exp()) / (2.0 * (((2.0 + (((-3.0)) as f64).exp())) as f64).sqrt())) - (((2.0) as f64).sqrt() / 3.0)));
            s.store_offset_div_from_scalar_ad(436, 1.0, A::mul(s.ad_value(154), s.ad_value(212)), (1.0 / ((2.0) as f64).sqrt()));
            s.store_div_scaled_inputs2(437, s.ad_value(402), -1.0, s.ad_value(397), -1.0, s.ad_value(212), 1.0);
            s.store_add_scaled_inputs3(441, A::div_scaled_product(A::square(s.ad_value(435)), s.ad_value(435), 1.0, A::mul3_scaled_output(s.ad_value(434), s.ad_value(434), s.ad_value(434), 27.0), 1.0), 1.0, A::div_scaled_product_by_product(s.ad_value(435), s.ad_value(436), 1.0, s.ad_value(434), s.ad_value(434), 6.0), (-1.0), A::div_scaled_inputs(s.ad_value(437), 1.0, s.ad_value(434), 2.0), 1.0);
            s.store_div_ad(440, A::add_scaled_square_product(s.ad_value(435), (-1.0), s.ad_value(434), s.ad_value(436), 3.0), A::mul_scaled_lhs(s.ad_value(434), 9.0, s.ad_value(434)));
            s.store_sqrt_ad(339, A::add_scaled_square_product(s.ad_value(441), 1.0, A::square(s.ad_value(440)), s.ad_value(440), 1.0));
            s.store_powf_ad(439, A::sub(s.ad_value(339), s.ad_value(441)), 0.3333333333333333);
            s.store_neg_ad(438, A::powf(A::add(s.ad_value(441), s.ad_value(339)), 0.3333333333333333));
            s.store_add_scaled_inputs3_mixed_iia(116, 439, 1.0, 438, 1.0, A::div_scaled_inputs(s.ad_value(435), 1.0, s.ad_value(434), 3.0), -1.0);
            s.store_add_scaled_product_indices(89, 397, (-1.0), 116, 155, 1.0);
        }

        s.b[2665] = (p.p33 > 0.0);
        s.v[2665] = if s.b[2665] { 1.0 } else { 0.0 };

        if (((s.v[2621] != 0.0) && (!s.b[2661])) && s.b[2665]) {
            s.store_offset_add(442, 402, 397, 0.1);
            s.store_mul(222, 405, 229);
            s.store_mul(443, 405, 229);
            s.store_mul(334, 156, 213);
            s.store_mul(444, 154, 442);
            s.store_add_scaled_inputs_product_mixed_aaii(447, A::ln(A::add_scaled_square_product(s.ad_value(444), 1.0, s.ad_value(443), s.ad_value(334), 1.0)), 1.0, A::ln(A::mul(s.ad_value(405), s.ad_value(334))), (-1.0), 154, 397, 1.0);
        }

        s.b[2666] = (p.p33 == 2.0);
        s.v[2666] = if s.b[2666] { 1.0 } else { 0.0 };

        if ((((s.v[2621] != 0.0) && (!s.b[2661])) && s.b[2665]) && s.b[2666]) {
            s.store_offset_sub(781, 444, 447, (-1.0));
            s.store_scale(782, 444, 4.0);
        }

        if ((((s.v[2621] != 0.0) && (!s.b[2661])) && s.b[2665]) && s.b[2666]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if ((((s.v[2621] != 0.0) && (!s.b[2661])) && s.b[2665]) && s.b[2666]) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(335, 781, 782, 0.5, 0.5);
            s.store_add_scaled_inputs3_indices(447, 444, 1.0, 781, (-0.5), 782, (-0.5));
        }

        if ((((s.v[2621] != 0.0) && (!s.b[2661])) && s.b[2665]) && (!s.b[2666])) {
            if (s.v[447] <= s.v[444]) {
            } else {
                s.copy_ad(447, 444);
            }
        }

        if (((s.v[2621] != 0.0) && (!s.b[2661])) && s.b[2665]) {
            if (s.v[447] >= 0.0) {
            } else {
                s.store_scalar(447, 0.0);
            }
        }

        if (((s.v[2621] != 0.0) && (!s.b[2661])) && s.b[2665]) {
            s.store_sub(444, 444, 447);
            s.store_add_scaled_inputs(444, 444, 1.0, 154, 0.1);
            s.store_sub_ad(335, A::ln(A::add_scaled_square_product(s.ad_value(444), 1.0, s.ad_value(443), s.ad_value(334), 1.0)), A::ln(A::mul(s.ad_value(405), s.ad_value(334))));
            s.store_add_scaled_product_indices(446, 335, 1.0, 154, 397, 1.0);
        }

        if (((s.v[2621] != 0.0) && (!s.b[2661])) && s.b[2665]) {
            if (s.v[446] >= 0.0) {
            } else {
                s.store_scalar(446, 0.0);
            }
        }

        if (((s.v[2621] != 0.0) && (!s.b[2661])) && s.b[2665]) {
            s.copy_ad(445, 116);
        }

        s.b[2667] = (p.p33 == 2.0);
        s.v[2667] = if s.b[2667] { 1.0 } else { 0.0 };

        s.b[2668] = ((s.v[445] > (s.v[446] - (0.2 * s.v[446]))) && ((0.2 * s.v[446]) >= 0.0));
        s.v[2668] = if s.b[2668] { 1.0 } else { 0.0 };

        if (((((s.v[2621] != 0.0) && (!s.b[2661])) && s.b[2665]) && s.b[2667]) && s.b[2668]) {
            s.store_add_scaled_inputs3_indices(781, 445, 1.0, 446, (-1.0), 446, 0.2);
            s.store_square(722, 781);
            s.store_scaled_mul(723, 446, 446, (0.2 * 0.2));
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
        }

        let (assign71990_e108783,) = {
    if (((((s.v[2621] != 0.0) && (!s.b[2661])) && s.b[2665]) && s.b[2667]) && s.b[2668]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign71990_e108783;

        let (assign72000_e108796,) = {
    if (((((s.v[2621] != 0.0) && (!s.b[2661])) && s.b[2665]) && s.b[2667]) && s.b[2668]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign72000_e108796;

        if (((((s.v[2621] != 0.0) && (!s.b[2661])) && s.b[2665]) && s.b[2667]) && s.b[2668]) {
            s.store_scalar(770, 0.0);
            s.store_scalar(726, 0.0);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_add(770, 724, 725);
            s.copy_ad(726, 770);
        }

        s.b[2669] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.v[2669] = if s.b[2669] { 1.0 } else { 0.0 };

        s.b[2670] = (2.0 == 1.0);
        s.v[2670] = if s.b[2670] { 1.0 } else { 0.0 };

        let (assign72110_e108945,) = {
    if (((((((s.v[2621] != 0.0) && (!s.b[2661])) && s.b[2665]) && s.b[2667]) && s.b[2668]) && s.b[2669]) && s.b[2670]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign72110_e108945;

        s.b[2671] = (2.0 == 2.0);
        s.v[2671] = if s.b[2671] { 1.0 } else { 0.0 };

        let (assign72130_e108968,) = {
    if ((((((((s.v[2621] != 0.0) && (!s.b[2661])) && s.b[2665]) && s.b[2667]) && s.b[2668]) && s.b[2669]) && (!s.b[2670])) && s.b[2671]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign72130_e108968;

        s.b[2672] = (2.0 == 4.0);
        s.v[2672] = if s.b[2672] { 1.0 } else { 0.0 };

        let (assign72150_e108994,) = {
    if (((((((((s.v[2621] != 0.0) && (!s.b[2661])) && s.b[2665]) && s.b[2667]) && s.b[2668]) && s.b[2669]) && (!s.b[2670])) && (!s.b[2671])) && s.b[2672]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign72150_e108994;

        s.b[2673] = (2.0 == 8.0);
        s.v[2673] = if s.b[2673] { 1.0 } else { 0.0 };

        let (assign72170_e109023,) = {
    if ((((((((((s.v[2621] != 0.0) && (!s.b[2661])) && s.b[2665]) && s.b[2667]) && s.b[2668]) && s.b[2669]) && (!s.b[2670])) && (!s.b[2671])) && (!s.b[2672])) && s.b[2673]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign72170_e109023;

        let (assign72180_e109038,) = {
    if ((((((s.v[2621] != 0.0) && (!s.b[2661])) && s.b[2665]) && s.b[2667]) && s.b[2668]) && s.b[2669]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign72180_e109038;

        let mut assign72190_loop_guard: usize = 0;
        while {
            let assign72190_cond_e109054: f64 = if (((((((s.v[2621] != 0.0) && (!s.b[2661])) && s.b[2665]) && s.b[2667]) && s.b[2668]) && s.b[2669]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign72190_cond_e109054 != 0.0
        } {
            assign72190_loop_guard += 1;
            assert!(assign72190_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((((s.v[2621] != 0.0) && (!s.b[2661])) && s.b[2665]) && s.b[2667]) && s.b[2668]) && s.b[2669]) {
                s.store_sqrt(726, 726);
            }
            let (assign72190_body1_e109087,) = {
    if ((((((s.v[2621] != 0.0) && (!s.b[2661])) && s.b[2665]) && s.b[2667]) && s.b[2668]) && s.b[2669]) {
        let assign72190_body1_e109085: f64 = (s.v[719] + 1.0);
        (assign72190_body1_e109085,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign72190_body1_e109087;
        }

        if ((((((s.v[2621] != 0.0) && (!s.b[2661])) && s.b[2665]) && s.b[2667]) && s.b[2668]) && (!s.b[2669])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }

        if (((((s.v[2621] != 0.0) && (!s.b[2661])) && s.b[2665]) && s.b[2667]) && s.b[2668]) {
            s.store_div_from_scalar(726, 1.0, 726);
        }

    }

    pub(super) fn stamp_transient_block_72(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((((s.v[2621] != 0.0) && (!s.b[2661])) && s.b[2665]) && s.b[2667]) && s.b[2668]) {
            s.store_mul3_affine_lhs(780, 781, 446, 0.2, 0.0, 726);
            s.store_div_scaled_product3_indices(335, 446, 725, 726, 0.2, 770, 1.0);
            s.store_add_scaled_inputs3_indices(116, 446, 1.0, 446, (-0.2), 780, 1.0);
        }

        if (((((s.v[2621] != 0.0) && (!s.b[2661])) && s.b[2665]) && s.b[2667]) && s.b[2668]) {
        }

        if (((((s.v[2621] != 0.0) && (!s.b[2661])) && s.b[2665]) && s.b[2667]) && (!s.b[2668])) {
            s.copy_ad(116, 445);
            s.store_scalar(335, 1.0);
        }

        if ((((s.v[2621] != 0.0) && (!s.b[2661])) && s.b[2665]) && (!s.b[2667])) {
            if (s.v[445] <= s.v[446]) {
                s.copy_ad(116, 445);
            } else {
                s.copy_ad(116, 446);
            }
        }

        s.b[2674] = (p.p33 == 1.0);
        s.v[2674] = if s.b[2674] { 1.0 } else { 0.0 };

        if (((s.v[2621] != 0.0) && (!s.b[2661])) && s.b[2674]) {
            s.store_add_scaled_product_indices(404, 397, (-1.0), 116, 155, 1.0);
        }

        s.b[2675] = (s.v[411] > 0.0);
        s.v[2675] = if s.b[2675] { 1.0 } else { 0.0 };

        if ((((s.v[2621] != 0.0) && (!s.b[2661])) && s.b[2674]) && s.b[2675]) {
            s.store_sub_from_scalar(336, p.p334, 411);
        }

        if ((((s.v[2621] != 0.0) && (!s.b[2661])) && s.b[2674]) && (!s.b[2675])) {
            s.store_sqrt_offset_ad(782, A::mul_offset_lhs(s.ad_value(729), p.p137, A::offset(s.ad_value(729), p.p137)), ((4.0 * 0.1) * 0.1));
            s.store_scaled_offset_ad(343, A::div_scaled_offset_numerator(s.ad_value(729), 1.0, p.p137, s.ad_value(782), 1.0), 1.0, 0.5);
            s.store_scaled_add_ad_lhs(336, A::offset(s.ad_value(729), p.p137), 782, 0.5);
        }

        s.b[2676] = (s.v[336] < 0.0);
        s.v[2676] = if s.b[2676] { 1.0 } else { 0.0 };

        if (((((s.v[2621] != 0.0) && (!s.b[2661])) && s.b[2674]) && (!s.b[2675])) && s.b[2676]) {
            s.store_scalar(336, 0.0);
            s.store_scalar(343, 0.0);
        }

        if ((((s.v[2621] != 0.0) && (!s.b[2661])) && s.b[2674]) && (!s.b[2675])) {
            s.store_scaled_sqrt_ad(600, A::mul(s.ad_value(651), s.ad_value(336)), p.p432);
            s.store_sub_from_scalar(336, p.p334, 600);
        }

        if (((s.v[2621] != 0.0) && (!s.b[2661])) && s.b[2674]) {
            s.store_sqrt_square_offset(782, 336, ((4.0 * (p.p334 * 0.01)) * (p.p334 * 0.01)));
            s.store_offset_scaled_div(343, 336, 782, 0.5, 0.5);
            s.store_scaled_add(336, 336, 782, 0.5);
        }

        s.b[2677] = (s.v[336] < 0.0);
        s.v[2677] = if s.b[2677] { 1.0 } else { 0.0 };

        if ((((s.v[2621] != 0.0) && (!s.b[2661])) && s.b[2674]) && s.b[2677]) {
            s.store_scalar(336, 0.0);
            s.store_scalar(343, 0.0);
        }

        if (((s.v[2621] != 0.0) && (!s.b[2661])) && s.b[2674]) {
            s.copy_ad(386, 336);
            s.store_mul3_affine_lhs(418, 2637, 386, (0.5 * 9662367879.197212), 0.0, 386);
            s.store_sqrt_ad(334, A::mul_scaled_lhs(s.ad_value(154), 2.0, s.ad_value(418)));
            s.store_scaled_add_ad(335, A::limited_exp(s.ad_value(334)), A::limited_exp_scaled_input(s.ad_value(334), -1.0), 0.5);
            s.store_div_ad_lhs(419, A::ln(s.ad_value(335)), 418);
            s.store_add(414, 404, 397);
            s.store_mul_sub_rhs(333, 419, 414, 418);
        }

        s.b[2678] = (s.v[333] < 60.0);
        s.v[2678] = if s.b[2678] { 1.0 } else { 0.0 };

        if ((((s.v[2621] != 0.0) && (!s.b[2661])) && s.b[2674]) && s.b[2678]) {
            s.store_exp(335, 333);
            s.store_exp_mul_scaled_lhs_indices(334, 419, -1.0, 418);
            s.store_sub(336, 335, 334);
            s.store_div_ad_lhs(416, A::ln(A::offset(s.ad_value(336), 1.0)), 419);
        }

        if ((((s.v[2621] != 0.0) && (!s.b[2661])) && s.b[2674]) && (!s.b[2678])) {
            s.store_sub(416, 414, 418);
        }

        if (((s.v[2621] != 0.0) && (!s.b[2661])) && s.b[2674]) {
            s.store_mul(415, 154, 416);
        }

        s.b[2679] = ((s.v[415] > (s.v[116] / 100.0)) && (s.v[415] > 0.0));
        s.v[2679] = if s.b[2679] { 1.0 } else { 0.0 };

        let (assign72620_e109676,) = {
    if ((((s.v[2621] != 0.0) && (!s.b[2661])) && s.b[2674]) && s.b[2679]) {
        let assign72620_e109674: f64 = (s.v[2643] + 1.0);
        (assign72620_e109674,)
    } else {
        (s.v[2643],)
    }
};
        s.v[2643] = assign72620_e109676;

        if ((((s.v[2621] != 0.0) && (!s.b[2661])) && s.b[2674]) && s.b[2679]) {
            s.copy_ad(116, 447);
        }

        if ((s.v[2621] != 0.0) && (!s.b[2661])) {
            s.store_add_scaled_product_indices(404, 397, (-1.0), 116, 155, 1.0);
        }

        s.b[2680] = (((s.v[116]) as f64).abs() > 1e-6);
        s.v[2680] = if s.b[2680] { 1.0 } else { 0.0 };

        if (((s.v[2621] != 0.0) && (!s.b[2661])) && s.b[2680]) {
            s.store_add_ad(335, A::offset(s.ad_value(116), (-1.0)), A::exp_scaled_input(s.ad_value(116), -1.0));
            s.store_sqrt(336, 335);
        }

        if (((s.v[2621] != 0.0) && (!s.b[2661])) && (!s.b[2680])) {
            s.store_mul_scaled_sqrt_ad_rhs(336, 116, 0.7071067811865475, A::sub_from_scalar(1.0, A::scale(s.ad_value(116), 0.3333333333333333)));
        }

        if ((s.v[2621] != 0.0) && (!s.b[2661])) {
            s.store_mul(354, 410, 336);
            s.store_mul_sub_rhs(398, 413, 402, 404);
            s.store_div(2681, 354, 2637);
        }

        s.b[2683] = (p.p33 == 2.0);
        s.v[2683] = if s.b[2683] { 1.0 } else { 0.0 };

        s.b[2684] = ((s.v[2681] > (s.v[386] - (s.v[386] * 0.1))) && ((s.v[386] * 0.1) >= 0.0));
        s.v[2684] = if s.b[2684] { 1.0 } else { 0.0 };

        if ((((s.v[2621] != 0.0) && (!s.b[2661])) && s.b[2683]) && s.b[2684]) {
            s.store_add_scaled_inputs3_indices(781, 2681, 1.0, 386, (-1.0), 386, 0.1);
            s.store_square(722, 781);
            s.store_scaled_mul(723, 386, 386, (0.1 * 0.1));
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
        }

        let (assign72800_e109883,) = {
    if ((((s.v[2621] != 0.0) && (!s.b[2661])) && s.b[2683]) && s.b[2684]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign72800_e109883;

        let (assign72810_e109894,) = {
    if ((((s.v[2621] != 0.0) && (!s.b[2661])) && s.b[2683]) && s.b[2684]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign72810_e109894;

        if ((((s.v[2621] != 0.0) && (!s.b[2661])) && s.b[2683]) && s.b[2684]) {
            s.store_scalar(770, 0.0);
            s.store_scalar(726, 0.0);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_add(770, 724, 725);
            s.copy_ad(726, 770);
        }

        s.b[2685] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.v[2685] = if s.b[2685] { 1.0 } else { 0.0 };

        s.b[2686] = (2.0 == 1.0);
        s.v[2686] = if s.b[2686] { 1.0 } else { 0.0 };

        let (assign72920_e110025,) = {
    if ((((((s.v[2621] != 0.0) && (!s.b[2661])) && s.b[2683]) && s.b[2684]) && s.b[2685]) && s.b[2686]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign72920_e110025;

        s.b[2687] = (2.0 == 2.0);
        s.v[2687] = if s.b[2687] { 1.0 } else { 0.0 };

        let (assign72940_e110046,) = {
    if (((((((s.v[2621] != 0.0) && (!s.b[2661])) && s.b[2683]) && s.b[2684]) && s.b[2685]) && (!s.b[2686])) && s.b[2687]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign72940_e110046;

        s.b[2688] = (2.0 == 4.0);
        s.v[2688] = if s.b[2688] { 1.0 } else { 0.0 };

        let (assign72960_e110070,) = {
    if ((((((((s.v[2621] != 0.0) && (!s.b[2661])) && s.b[2683]) && s.b[2684]) && s.b[2685]) && (!s.b[2686])) && (!s.b[2687])) && s.b[2688]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign72960_e110070;

        s.b[2689] = (2.0 == 8.0);
        s.v[2689] = if s.b[2689] { 1.0 } else { 0.0 };

        let (assign72980_e110097,) = {
    if (((((((((s.v[2621] != 0.0) && (!s.b[2661])) && s.b[2683]) && s.b[2684]) && s.b[2685]) && (!s.b[2686])) && (!s.b[2687])) && (!s.b[2688])) && s.b[2689]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign72980_e110097;

        let (assign72990_e110110,) = {
    if (((((s.v[2621] != 0.0) && (!s.b[2661])) && s.b[2683]) && s.b[2684]) && s.b[2685]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign72990_e110110;

        let mut assign73000_loop_guard: usize = 0;
        while {
            let assign73000_cond_e110124: f64 = if ((((((s.v[2621] != 0.0) && (!s.b[2661])) && s.b[2683]) && s.b[2684]) && s.b[2685]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign73000_cond_e110124 != 0.0
        } {
            assign73000_loop_guard += 1;
            assert!(assign73000_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((((s.v[2621] != 0.0) && (!s.b[2661])) && s.b[2683]) && s.b[2684]) && s.b[2685]) {
                s.store_sqrt(726, 726);
            }
            let (assign73000_body1_e110153,) = {
    if (((((s.v[2621] != 0.0) && (!s.b[2661])) && s.b[2683]) && s.b[2684]) && s.b[2685]) {
        let assign73000_body1_e110151: f64 = (s.v[719] + 1.0);
        (assign73000_body1_e110151,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign73000_body1_e110153;
        }

        if (((((s.v[2621] != 0.0) && (!s.b[2661])) && s.b[2683]) && s.b[2684]) && (!s.b[2685])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }

        if ((((s.v[2621] != 0.0) && (!s.b[2661])) && s.b[2683]) && s.b[2684]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_mul3_affine_lhs(780, 781, 386, 0.1, 0.0, 726);
            s.store_div_scaled_product3_indices(334, 386, 725, 726, 0.1, 770, 1.0);
            s.store_add_scaled_inputs3_indices(335, 386, 1.0, 386, (-0.1), 780, 1.0);
        }

        if ((((s.v[2621] != 0.0) && (!s.b[2661])) && s.b[2683]) && s.b[2684]) {
        }

        if ((((s.v[2621] != 0.0) && (!s.b[2661])) && s.b[2683]) && (!s.b[2684])) {
            s.copy_ad(335, 2681);
            s.store_scalar(334, 1.0);
        }

        s.b[2690] = (s.v[334] < 1.0);
        s.v[2690] = if s.b[2690] { 1.0 } else { 0.0 };

        let (assign73100_e110295,) = {
    if ((((s.v[2621] != 0.0) && (!s.b[2661])) && s.b[2683]) && s.b[2690]) {
        let assign73100_e110293: f64 = (s.v[2643] + 2.0);
        (assign73100_e110293,)
    } else {
        (s.v[2643],)
    }
};
        s.v[2643] = assign73100_e110295;

        if (((s.v[2621] != 0.0) && (!s.b[2661])) && (!s.b[2683])) {
            if (s.v[2681] <= s.v[386]) {
                s.copy_ad(335, 2681);
            } else {
                s.copy_ad(335, 386);
            }
        }

        s.b[2691] = (s.v[2681] >= s.v[386]);
        s.v[2691] = if s.b[2691] { 1.0 } else { 0.0 };

        let (assign73130_e110327,) = {
    if ((((s.v[2621] != 0.0) && (!s.b[2661])) && (!s.b[2683])) && s.b[2691]) {
        let assign73130_e110325: f64 = (s.v[2643] + 2.0);
        (assign73130_e110325,)
    } else {
        (s.v[2643],)
    }
};
        s.v[2643] = assign73130_e110327;

        s.b[2692] = (s.v[2643] >= 2.0);
        s.v[2692] = if s.b[2692] { 1.0 } else { 0.0 };

        if (((s.v[2621] != 0.0) && (!s.b[2661])) && s.b[2692]) {
            s.copy_ad(2682, 404);
            s.store_mul(354, 335, 2637);
            s.store_sub_div_rhs_indices(404, 402, 354, 413);
        }

        s.b[2693] = (p.p33 == 2.0);
        s.v[2693] = if s.b[2693] { 1.0 } else { 0.0 };

        s.b[2694] = ((s.v[404] > (s.v[2682] - 0.1)) && (0.1 >= 0.0));
        s.v[2694] = if s.b[2694] { 1.0 } else { 0.0 };

        if (((((s.v[2621] != 0.0) && (!s.b[2661])) && s.b[2692]) && s.b[2693]) && s.b[2694]) {
            s.store_offset_sub(781, 404, 2682, 0.1);
            s.store_square(722, 781);
            s.store_scalar(723, (0.1 * 0.1));
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
        }

        let (assign73250_e110461,) = {
    if (((((s.v[2621] != 0.0) && (!s.b[2661])) && s.b[2692]) && s.b[2693]) && s.b[2694]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign73250_e110461;

        let (assign73260_e110474,) = {
    if (((((s.v[2621] != 0.0) && (!s.b[2661])) && s.b[2692]) && s.b[2693]) && s.b[2694]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign73260_e110474;

        if (((((s.v[2621] != 0.0) && (!s.b[2661])) && s.b[2692]) && s.b[2693]) && s.b[2694]) {
            s.store_scalar(770, 0.0);
            s.store_scalar(726, 0.0);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
        }

    }

    pub(super) fn stamp_transient_block_73(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((((s.v[2621] != 0.0) && (!s.b[2661])) && s.b[2692]) && s.b[2693]) && s.b[2694]) {
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_add(770, 724, 725);
            s.copy_ad(726, 770);
        }

        s.b[2695] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.v[2695] = if s.b[2695] { 1.0 } else { 0.0 };

        s.b[2696] = (2.0 == 1.0);
        s.v[2696] = if s.b[2696] { 1.0 } else { 0.0 };

        let (assign73370_e110623,) = {
    if (((((((s.v[2621] != 0.0) && (!s.b[2661])) && s.b[2692]) && s.b[2693]) && s.b[2694]) && s.b[2695]) && s.b[2696]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign73370_e110623;

        s.b[2697] = (2.0 == 2.0);
        s.v[2697] = if s.b[2697] { 1.0 } else { 0.0 };

        let (assign73390_e110646,) = {
    if ((((((((s.v[2621] != 0.0) && (!s.b[2661])) && s.b[2692]) && s.b[2693]) && s.b[2694]) && s.b[2695]) && (!s.b[2696])) && s.b[2697]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign73390_e110646;

        s.b[2698] = (2.0 == 4.0);
        s.v[2698] = if s.b[2698] { 1.0 } else { 0.0 };

        let (assign73410_e110672,) = {
    if (((((((((s.v[2621] != 0.0) && (!s.b[2661])) && s.b[2692]) && s.b[2693]) && s.b[2694]) && s.b[2695]) && (!s.b[2696])) && (!s.b[2697])) && s.b[2698]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign73410_e110672;

        s.b[2699] = (2.0 == 8.0);
        s.v[2699] = if s.b[2699] { 1.0 } else { 0.0 };

        let (assign73430_e110701,) = {
    if ((((((((((s.v[2621] != 0.0) && (!s.b[2661])) && s.b[2692]) && s.b[2693]) && s.b[2694]) && s.b[2695]) && (!s.b[2696])) && (!s.b[2697])) && (!s.b[2698])) && s.b[2699]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign73430_e110701;

        let (assign73440_e110716,) = {
    if ((((((s.v[2621] != 0.0) && (!s.b[2661])) && s.b[2692]) && s.b[2693]) && s.b[2694]) && s.b[2695]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign73440_e110716;

        let mut assign73450_loop_guard: usize = 0;
        while {
            let assign73450_cond_e110732: f64 = if (((((((s.v[2621] != 0.0) && (!s.b[2661])) && s.b[2692]) && s.b[2693]) && s.b[2694]) && s.b[2695]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign73450_cond_e110732 != 0.0
        } {
            assign73450_loop_guard += 1;
            assert!(assign73450_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((((s.v[2621] != 0.0) && (!s.b[2661])) && s.b[2692]) && s.b[2693]) && s.b[2694]) && s.b[2695]) {
                s.store_sqrt(726, 726);
            }
            let (assign73450_body1_e110765,) = {
    if ((((((s.v[2621] != 0.0) && (!s.b[2661])) && s.b[2692]) && s.b[2693]) && s.b[2694]) && s.b[2695]) {
        let assign73450_body1_e110763: f64 = (s.v[719] + 1.0);
        (assign73450_body1_e110763,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign73450_body1_e110765;
        }

        if ((((((s.v[2621] != 0.0) && (!s.b[2661])) && s.b[2692]) && s.b[2693]) && s.b[2694]) && (!s.b[2695])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }

        if (((((s.v[2621] != 0.0) && (!s.b[2661])) && s.b[2692]) && s.b[2693]) && s.b[2694]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, 0.1);
            s.store_div_scaled_product_indices(334, 725, 726, 0.1, 770, 1.0);
            s.store_add_offset_lhs(404, 2682, (-0.1), 780);
        }

        if (((((s.v[2621] != 0.0) && (!s.b[2661])) && s.b[2692]) && s.b[2693]) && s.b[2694]) {
        }

        if (((((s.v[2621] != 0.0) && (!s.b[2661])) && s.b[2692]) && s.b[2693]) && (!s.b[2694])) {
        }

        if (((((s.v[2621] != 0.0) && (!s.b[2661])) && s.b[2692]) && s.b[2693]) && (!s.b[2694])) {
            s.store_scalar(334, 1.0);
        }

        if ((((s.v[2621] != 0.0) && (!s.b[2661])) && s.b[2692]) && (!s.b[2693])) {
            if (s.v[404] <= s.v[2682]) {
            } else {
                s.copy_ad(404, 2682);
            }
        }

        if ((s.v[2621] != 0.0) && (!s.b[2661])) {
            s.copy_ad(2644, 404);
        }

        s.b[2700] = (p.p33 == 1.0);
        s.v[2700] = if s.b[2700] { 1.0 } else { 0.0 };

        let (assign73570_e110937,) = {
    if (((s.v[2621] != 0.0) && (!s.b[2661])) && s.b[2700]) {
        (0.0,)
    } else {
        (s.v[79],)
    }
};
        s.v[79] = assign73570_e110937;

        if (((s.v[2621] != 0.0) && (!s.b[2661])) && s.b[2700]) {
            s.store_sqrt_scaled_input_ad(982, A::mul(A::div_from_scalar(1.034943e-10, s.ad_value(2637)), s.ad_value(155)), 2.0);
        }

        s.b[2701] = (s.v[411] > 0.0);
        s.v[2701] = if s.b[2701] { 1.0 } else { 0.0 };

        if ((((s.v[2621] != 0.0) && (!s.b[2661])) && s.b[2700]) && s.b[2701]) {
            s.store_sub_from_scalar(336, p.p334, 411);
        }

        if ((((s.v[2621] != 0.0) && (!s.b[2661])) && s.b[2700]) && (!s.b[2701])) {
            s.store_sqrt_offset_ad(782, A::mul_offset_lhs(s.ad_value(729), p.p137, A::offset(s.ad_value(729), p.p137)), ((4.0 * 0.1) * 0.1));
            s.store_scaled_offset_ad(343, A::div_scaled_offset_numerator(s.ad_value(729), 1.0, p.p137, s.ad_value(782), 1.0), 1.0, 0.5);
            s.store_scaled_add_ad_lhs(336, A::offset(s.ad_value(729), p.p137), 782, 0.5);
        }

        s.b[2702] = (s.v[336] < 0.0);
        s.v[2702] = if s.b[2702] { 1.0 } else { 0.0 };

        if (((((s.v[2621] != 0.0) && (!s.b[2661])) && s.b[2700]) && (!s.b[2701])) && s.b[2702]) {
            s.store_scalar(336, 0.0);
            s.store_scalar(343, 0.0);
        }

        if ((((s.v[2621] != 0.0) && (!s.b[2661])) && s.b[2700]) && (!s.b[2701])) {
            s.store_scaled_sqrt_ad(600, A::mul(s.ad_value(651), s.ad_value(336)), p.p432);
            s.store_sub_from_scalar(336, p.p334, 600);
        }

        if (((s.v[2621] != 0.0) && (!s.b[2661])) && s.b[2700]) {
            s.store_sqrt_square_offset(782, 336, ((4.0 * (p.p334 * 0.01)) * (p.p334 * 0.01)));
            s.store_offset_scaled_div(343, 336, 782, 0.5, 0.5);
            s.store_scaled_add(336, 336, 782, 0.5);
        }

        s.b[2703] = (s.v[336] < 0.0);
        s.v[2703] = if s.b[2703] { 1.0 } else { 0.0 };

        if ((((s.v[2621] != 0.0) && (!s.b[2661])) && s.b[2700]) && s.b[2703]) {
            s.store_scalar(336, 0.0);
            s.store_scalar(343, 0.0);
        }

        if (((s.v[2621] != 0.0) && (!s.b[2661])) && s.b[2700]) {
            s.copy_ad(386, 336);
            s.store_mul3_affine_lhs(418, 2637, 386, (0.5 * 9662367879.197212), 0.0, 386);
            s.store_sqrt_ad(334, A::mul_scaled_lhs(s.ad_value(154), 2.0, s.ad_value(418)));
            s.store_scaled_add_ad(335, A::limited_exp(s.ad_value(334)), A::limited_exp_scaled_input(s.ad_value(334), -1.0), 0.5);
            s.store_div_ad_lhs(419, A::ln(s.ad_value(335)), 418);
        }

        let (assign73800_e111246,) = {
    if (((s.v[2621] != 0.0) && (!s.b[2661])) && s.b[2700]) {
        (1.0,)
    } else {
        (s.v[97],)
    }
};
        s.v[97] = assign73800_e111246;

        let mut assign73810_loop_guard: usize = 0;
        while {
            let assign73810_cond_e111256: f64 = (s.v[421] + 1.0);
            let assign73810_cond_e111258: f64 = if ((((s.v[2621] != 0.0) && (!s.b[2661])) && s.b[2700]) && (s.v[97] <= assign73810_cond_e111256)) { 1.0 } else { 0.0 };
            assign73810_cond_e111258 != 0.0
        } {
            assign73810_loop_guard += 1;
            assert!(assign73810_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.v[2621] != 0.0) && (!s.b[2661])) && s.b[2700]) {
                s.store_add(414, 404, 397);
                s.store_mul(116, 154, 414);
                s.store_mul_sub_rhs(333, 419, 414, 418);
            }
            s.b[2705] = (s.v[333] < 60.0);
            s.v[2705] = if s.b[2705] { 1.0 } else { 0.0 };
            if ((((s.v[2621] != 0.0) && (!s.b[2661])) && s.b[2700]) && s.b[2705]) {
                s.store_exp(335, 333);
                s.store_exp_mul_scaled_lhs_indices(334, 419, -1.0, 418);
                s.store_sub(336, 335, 334);
                s.store_div_ad_lhs(416, A::ln(A::offset(s.ad_value(336), 1.0)), 419);
                s.store_div_scaled_value_offset_denominator(417, s.ad_value(335), 1.0, s.ad_value(336), 1.0, 1.0);
            }
            if ((((s.v[2621] != 0.0) && (!s.b[2661])) && s.b[2700]) && (!s.b[2705])) {
                s.store_sub(416, 414, 418);
                s.store_scalar(417, 1.0);
            }
            if (((s.v[2621] != 0.0) && (!s.b[2661])) && s.b[2700]) {
                s.store_mul(415, 154, 416);
            }
            s.b[2706] = (s.v[116] < 0.0);
            s.v[2706] = if s.b[2706] { 1.0 } else { 0.0 };
            if ((((s.v[2621] != 0.0) && (!s.b[2661])) && s.b[2700]) && s.b[2706]) {
                s.store_scalar(334, (-0.7071067811865475));
                s.store_mul(223, 116, 334);
                s.store_mul(420, 154, 334);
            }
            s.b[2707] = (s.v[116] < 1e-6);
            s.v[2707] = if s.b[2707] { 1.0 } else { 0.0 };
            if (((((s.v[2621] != 0.0) && (!s.b[2661])) && s.b[2700]) && (!s.b[2706])) && s.b[2707]) {
                s.store_square_mul_sub_from_scalar_double_scaled_sub_rhs_scaled_output(334, 116, 1.0, 116, 1.0, 116, 1.0, 116, 0.2, 1.0 / (4.0), 1.0 / (3.0), 1.0 / (2.0));
                s.store_mul_sub_from_scalar_ad_rhs(335, 116, 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(116), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(116), 1.0, A::scale(s.ad_value(116), 0.25), 1.0 / (3.0)), 1.0 / (2.0)));
                s.store_square_mul_sub_from_scalar_double_scaled_sub_rhs_scaled_output(336, 415, 1.0, 415, 1.0, 415, 1.0, 415, 0.2, 1.0 / (4.0), 1.0 / (3.0), 1.0 / (2.0));
                s.store_mul_sub_from_scalar_ad_rhs(337, 415, 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(415), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(415), 1.0, A::scale(s.ad_value(415), 0.25), 1.0 / (3.0)), 1.0 / (2.0)));
                s.store_sub(338, 334, 336);
            }
            s.b[2708] = (s.v[338] > 0.0);
            s.v[2708] = if s.b[2708] { 1.0 } else { 0.0 };
            if ((((((s.v[2621] != 0.0) && (!s.b[2661])) && s.b[2700]) && (!s.b[2706])) && s.b[2707]) && s.b[2708]) {
                s.store_sqrt(223, 338);
                s.store_div_scaled_product_right_ad(420, 154, A::add_scaled_product(s.ad_value(335), 1.0, s.ad_value(417), s.ad_value(337), (-1.0)), 0.5, 223, 1.0);
            }
            if ((((((s.v[2621] != 0.0) && (!s.b[2661])) && s.b[2700]) && (!s.b[2706])) && s.b[2707]) && (!s.b[2708])) {
                s.store_scalar(223, 0.0);
                s.store_scalar(420, 0.0);
            }
            if (((((s.v[2621] != 0.0) && (!s.b[2661])) && s.b[2700]) && (!s.b[2706])) && (!s.b[2707])) {
                s.store_exp_neg_input(334, 116);
                s.store_exp_neg_input(335, 415);
                s.store_add_scaled_inputs4_indices(338, 116, 1.0, 415, (-1.0), 334, 1.0, 335, (-1.0));
            }
            s.b[2709] = (s.v[338] > 0.0);
            s.v[2709] = if s.b[2709] { 1.0 } else { 0.0 };
            if ((((((s.v[2621] != 0.0) && (!s.b[2661])) && s.b[2700]) && (!s.b[2706])) && (!s.b[2707])) && s.b[2709]) {
                s.store_sqrt(223, 338);
                s.store_div_scaled_product_right_ad(420, 154, A::sub(A::sub_from_scalar(1.0, s.ad_value(334)), A::mul_sub_from_scalar_rhs(s.ad_value(417), 1.0, s.ad_value(335))), 0.5, 223, 1.0);
            }
            if ((((((s.v[2621] != 0.0) && (!s.b[2661])) && s.b[2700]) && (!s.b[2706])) && (!s.b[2707])) && (!s.b[2709])) {
                s.store_scalar(223, 0.0);
                s.store_scalar(420, 0.0);
            }
            s.b[2710] = (s.v[116] < 0.0);
            s.v[2710] = if s.b[2710] { 1.0 } else { 0.0 };
            if ((((s.v[2621] != 0.0) && (!s.b[2661])) && s.b[2700]) && s.b[2710]) {
                s.store_scalar(214, 0.0);
                s.store_scalar(215, 0.0);
                s.store_neg(216, 223);
                s.store_neg(217, 420);
            }
            s.b[2711] = (s.v[116] < 60.0);
            s.v[2711] = if s.b[2711] { 1.0 } else { 0.0 };
            s.b[2712] = (s.v[116] < 5e-5);
            s.v[2712] = if s.b[2712] { 1.0 } else { 0.0 };
            if ((((((s.v[2621] != 0.0) && (!s.b[2661])) && s.b[2700]) && (!s.b[2710])) && s.b[2711]) && s.b[2712]) {
                s.store_scaled_mul_ad(334, A::square(s.ad_value(116)), A::offset(A::mul_scaled_lhs(s.ad_value(116), 1.0 / (3.0), A::offset(A::mul_scaled_lhs(s.ad_value(116), 1.0 / (4.0), A::scale_offset(s.ad_value(116), 0.2, 1.0)), 1.0)), 1.0), 1.0 / (2.0));
                s.store_mul_offset_ad_rhs(335, 116, A::mul_scaled_lhs(s.ad_value(116), 1.0 / (2.0), A::offset(A::mul_scaled_lhs(s.ad_value(116), 1.0 / (3.0), A::scale_offset(s.ad_value(116), 0.25, 1.0)), 1.0)), 1.0);
                s.store_mul(214, 222, 334);
                s.store_mul3_lhs(215, 222, 335, 154);
            }
            if ((((((s.v[2621] != 0.0) && (!s.b[2661])) && s.b[2700]) && (!s.b[2710])) && s.b[2711]) && (!s.b[2712])) {
                s.store_exp(227, 116);
                s.store_offset(335, 227, (-1.0));
                s.store_mul_sub_rhs(214, 222, 335, 116);
                s.store_mul3_lhs(215, 222, 154, 335);
            }
            if (((((s.v[2621] != 0.0) && (!s.b[2661])) && s.b[2700]) && (!s.b[2710])) && (!s.b[2711])) {
                s.store_exp_mul(231, 154, 404);
                s.store_mul_ad_rhs(214, 405, A::add_scaled_offset_product_rhs(s.ad_value(231), 1.0, s.ad_value(229), s.ad_value(116), 1.0, (-1.0)));
                s.store_mul_ad_product_rhs(215, 405, s.ad_value(154), A::sub(s.ad_value(231), s.ad_value(229)));
            }
            s.b[2713] = (s.v[214] > 0.0);
            s.v[2713] = if s.b[2713] { 1.0 } else { 0.0 };
            if (((((s.v[2621] != 0.0) && (!s.b[2661])) && s.b[2700]) && (!s.b[2710])) && s.b[2713]) {
                s.store_sqrt_square_add(216, 223, 214);
                s.store_div_scaled_add_product(217, s.ad_value(215), 0.5, s.ad_value(420), s.ad_value(223), (2.0 * 0.5), s.ad_value(216), 1.0);
            }
            if (((((s.v[2621] != 0.0) && (!s.b[2661])) && s.b[2700]) && (!s.b[2710])) && (!s.b[2713])) {
                s.copy_ad(216, 223);
                s.copy_ad(217, 420);
            }
            if (((s.v[2621] != 0.0) && (!s.b[2661])) && s.b[2700]) {
                s.store_add_scaled_inputs_product_indices(232, 404, 1.0, 402, (-1.0), 212, 216, 1.0);
                s.store_offset_mul(233, 212, 217, 1.0);
            }
            s.b[2714] = (s.v[79] == 1.0);
            s.v[2714] = if s.b[2714] { 1.0 } else { 0.0 };
            let (assign73810_body72_e112404,) = {
    if ((((s.v[2621] != 0.0) && (!s.b[2661])) && s.b[2700]) && s.b[2714]) {
        let assign73810_body72_e112402: f64 = (s.v[421] + 1.0);
        (assign73810_body72_e112402,)
    } else {
        (s.v[97],)
    }
};
            s.v[97] = assign73810_body72_e112404;
            if ((((s.v[2621] != 0.0) && (!s.b[2661])) && s.b[2700]) && (!s.b[2714])) {
                s.store_div_scaled_inputs(236, s.ad_value(232), -1.0, s.ad_value(233), 1.0);
            }
            if ((((s.v[2621] != 0.0) && (!s.b[2661])) && s.b[2700]) && (!s.b[2714])) {
                s.store_scaled_offset_ad(93, {
                    if (1.0 >= ((s.v[404]) as f64).abs()) {
                        A::constant(1.0)
                    } else {
                        A::abs(s.ad_value(404))
                    }
                }, 1.0, (0.5 * 0.1));
            }
            s.b[2715] = (((s.v[236]) as f64).abs() > s.v[93]);
            s.v[2715] = if s.b[2715] { 1.0 } else { 0.0 };
            if (((((s.v[2621] != 0.0) && (!s.b[2661])) && s.b[2700]) && (!s.b[2714])) && s.b[2715]) {
                s.store_scale(236, 93, (if (s.v[236] >= 0.0) { 1.0 } else { (-1.0) }));
            }
            if ((((s.v[2621] != 0.0) && (!s.b[2661])) && s.b[2700]) && (!s.b[2714])) {
                s.store_add(404, 404, 236);
            }
            s.b[2716] = ((((s.v[236]) as f64).abs() <= 1e-12) && (((s.v[232]) as f64).abs() <= 1e-8));
            s.v[2716] = if s.b[2716] { 1.0 } else { 0.0 };
            let (assign73810_body79_e112507,) = {
    if (((((s.v[2621] != 0.0) && (!s.b[2661])) && s.b[2700]) && (!s.b[2714])) && s.b[2716]) {
        (1.0,)
    } else {
        (s.v[79],)
    }
};
            s.v[79] = assign73810_body79_e112507;
            let (assign73810_body80_e112518,) = {
    if (((s.v[2621] != 0.0) && (!s.b[2661])) && s.b[2700]) {
        let assign73810_body80_e112516: f64 = (s.v[97] + 1.0);
        (assign73810_body80_e112516,)
    } else {
        (s.v[97],)
    }
};
            s.v[97] = assign73810_body80_e112518;
        }

        if (((s.v[2621] != 0.0) && (!s.b[2661])) && s.b[2700]) {
            s.store_mul(2635, 982, 223);
            s.store_mul(2636, 2637, 2635);
            s.store_offset_div(100, 2636, 410, (10.0 * 2.220446049250313e-16));
            s.store_mul(354, 410, 100);
            s.store_div_from_scalar_add_ad(335, 1.0, s.ad_value(216), s.ad_value(100));
            s.store_mul3_lhs(399, 410, 214, 335);
            s.store_add(398, 354, 399);
        }

    }

    pub(super) fn stamp_transient_block_74(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.b[2718] = (p.p33 == 4.0);
        s.v[2718] = if s.b[2718] { 1.0 } else { 0.0 };

        if ((s.v[2621] != 0.0) && s.b[2718]) {
            s.store_exp_ad(229, A::mul_scaled_rhs(s.ad_value(154), s.ad_value(397), -1.0));
            s.store_div(334, 394, 409);
            s.store_square(405, 334);
            s.store_mul(222, 405, 229);
            s.copy_ad(404, 2644);
        }

        let (assign73960_e112655,) = {
    if ((s.v[2621] != 0.0) && s.b[2718]) {
        (0.0,)
    } else {
        (s.v[79],)
    }
};
        s.v[79] = assign73960_e112655;

        if ((s.v[2621] != 0.0) && s.b[2718]) {
            s.store_sqrt_scaled_input_ad(982, A::mul(A::div_from_scalar(1.034943e-10, s.ad_value(2637)), s.ad_value(155)), 2.0);
        }

        s.b[2719] = (s.v[411] > 0.0);
        s.v[2719] = if s.b[2719] { 1.0 } else { 0.0 };

        if (((s.v[2621] != 0.0) && s.b[2718]) && s.b[2719]) {
            s.store_sub_from_scalar(336, p.p334, 411);
        }

        if (((s.v[2621] != 0.0) && s.b[2718]) && (!s.b[2719])) {
            s.store_sqrt_offset_ad(782, A::mul_offset_lhs(s.ad_value(729), p.p137, A::offset(s.ad_value(729), p.p137)), ((4.0 * 0.1) * 0.1));
            s.store_scaled_offset_ad(343, A::div_scaled_offset_numerator(s.ad_value(729), 1.0, p.p137, s.ad_value(782), 1.0), 1.0, 0.5);
            s.store_scaled_add_ad_lhs(336, A::offset(s.ad_value(729), p.p137), 782, 0.5);
        }

        s.b[2720] = (s.v[336] < 0.0);
        s.v[2720] = if s.b[2720] { 1.0 } else { 0.0 };

        if ((((s.v[2621] != 0.0) && s.b[2718]) && (!s.b[2719])) && s.b[2720]) {
            s.store_scalar(336, 0.0);
            s.store_scalar(343, 0.0);
        }

        if (((s.v[2621] != 0.0) && s.b[2718]) && (!s.b[2719])) {
            s.store_scaled_sqrt_ad(600, A::mul(s.ad_value(651), s.ad_value(336)), p.p432);
            s.store_sub_from_scalar(336, p.p334, 600);
        }

        if ((s.v[2621] != 0.0) && s.b[2718]) {
            s.store_sqrt_square_offset(782, 336, ((4.0 * (p.p334 * 0.01)) * (p.p334 * 0.01)));
            s.store_offset_scaled_div(343, 336, 782, 0.5, 0.5);
            s.store_scaled_add(336, 336, 782, 0.5);
        }

        s.b[2721] = (s.v[336] < 0.0);
        s.v[2721] = if s.b[2721] { 1.0 } else { 0.0 };

        if (((s.v[2621] != 0.0) && s.b[2718]) && s.b[2721]) {
            s.store_scalar(336, 0.0);
            s.store_scalar(343, 0.0);
        }

        if ((s.v[2621] != 0.0) && s.b[2718]) {
            s.copy_ad(386, 336);
            s.store_mul3_affine_lhs(418, 2637, 386, (0.5 * 9662367879.197212), 0.0, 386);
            s.store_sqrt_ad(334, A::mul_scaled_lhs(s.ad_value(154), 2.0, s.ad_value(418)));
            s.store_scaled_add_ad(335, A::limited_exp(s.ad_value(334)), A::limited_exp_scaled_input(s.ad_value(334), -1.0), 0.5);
            s.store_div_ad_lhs(419, A::ln(s.ad_value(335)), 418);
        }

        let (assign74190_e112904,) = {
    if ((s.v[2621] != 0.0) && s.b[2718]) {
        (1.0,)
    } else {
        (s.v[97],)
    }
};
        s.v[97] = assign74190_e112904;

        let mut assign74200_loop_guard: usize = 0;
        while {
            let assign74200_cond_e112911: f64 = (s.v[421] + 1.0);
            let assign74200_cond_e112913: f64 = if (((s.v[2621] != 0.0) && s.b[2718]) && (s.v[97] <= assign74200_cond_e112911)) { 1.0 } else { 0.0 };
            assign74200_cond_e112913 != 0.0
        } {
            assign74200_loop_guard += 1;
            assert!(assign74200_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((s.v[2621] != 0.0) && s.b[2718]) {
                s.store_add(414, 404, 397);
                s.store_mul(116, 154, 414);
                s.store_mul_sub_rhs(333, 419, 414, 418);
            }
            s.b[2723] = (s.v[333] < 60.0);
            s.v[2723] = if s.b[2723] { 1.0 } else { 0.0 };
            if (((s.v[2621] != 0.0) && s.b[2718]) && s.b[2723]) {
                s.store_exp(335, 333);
                s.store_exp_mul_scaled_lhs_indices(334, 419, -1.0, 418);
                s.store_sub(336, 335, 334);
                s.store_div_ad_lhs(416, A::ln(A::offset(s.ad_value(336), 1.0)), 419);
                s.store_div_scaled_value_offset_denominator(417, s.ad_value(335), 1.0, s.ad_value(336), 1.0, 1.0);
            }
            if (((s.v[2621] != 0.0) && s.b[2718]) && (!s.b[2723])) {
                s.store_sub(416, 414, 418);
                s.store_scalar(417, 1.0);
            }
            if ((s.v[2621] != 0.0) && s.b[2718]) {
                s.store_mul(415, 154, 416);
            }
            s.b[2724] = (((s.v[116]) as f64).abs() < 1e-6);
            s.v[2724] = if s.b[2724] { 1.0 } else { 0.0 };
            if (((s.v[2621] != 0.0) && s.b[2718]) && s.b[2724]) {
                s.store_square_mul_sub_from_scalar_double_scaled_sub_rhs_scaled_output(334, 116, 1.0, 116, 1.0, 116, 1.0, 116, 0.2, 1.0 / (4.0), 1.0 / (3.0), 1.0 / (2.0));
                s.store_mul_sub_from_scalar_ad_rhs(335, 116, 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(116), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(116), 1.0, A::scale(s.ad_value(116), 0.25), 1.0 / (3.0)), 1.0 / (2.0)));
                s.store_square_mul_sub_from_scalar_double_scaled_sub_rhs_scaled_output(336, 415, 1.0, 415, 1.0, 415, 1.0, 415, 0.2, 1.0 / (4.0), 1.0 / (3.0), 1.0 / (2.0));
                s.store_mul_sub_from_scalar_ad_rhs(337, 415, 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(415), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(415), 1.0, A::scale(s.ad_value(415), 0.25), 1.0 / (3.0)), 1.0 / (2.0)));
                s.store_sub(2645, 334, 336);
                s.store_mul_add_scaled_product_rhs(2646, 154, s.ad_value(335), 1.0, s.ad_value(417), s.ad_value(337), (-1.0));
            }
            if (((s.v[2621] != 0.0) && s.b[2718]) && (!s.b[2724])) {
                s.store_exp_neg_input(334, 116);
                s.store_exp_neg_input(335, 415);
                s.store_add_scaled_inputs4_indices(2645, 116, 1.0, 415, (-1.0), 334, 1.0, 335, (-1.0));
                s.store_mul_sub_ad_rhs(2646, 154, A::sub_from_scalar(1.0, s.ad_value(334)), A::mul_sub_from_scalar_rhs(s.ad_value(417), 1.0, s.ad_value(335)));
            }
            s.b[2725] = (((s.v[116]) as f64).abs() < 5e-5);
            s.v[2725] = if s.b[2725] { 1.0 } else { 0.0 };
            if (((s.v[2621] != 0.0) && s.b[2718]) && s.b[2725]) {
                s.store_scaled_mul_ad(334, A::square(s.ad_value(116)), A::offset(A::mul_scaled_lhs(s.ad_value(116), 1.0 / (3.0), A::offset(A::mul_scaled_lhs(s.ad_value(116), 1.0 / (4.0), A::scale_offset(s.ad_value(116), 0.2, 1.0)), 1.0)), 1.0), 1.0 / (2.0));
                s.store_mul_offset_ad_rhs(335, 116, A::mul_scaled_lhs(s.ad_value(116), 1.0 / (2.0), A::offset(A::mul_scaled_lhs(s.ad_value(116), 1.0 / (3.0), A::scale_offset(s.ad_value(116), 0.25, 1.0)), 1.0)), 1.0);
                s.store_mul(214, 222, 334);
                s.store_mul3_lhs(215, 222, 335, 154);
            }
            s.b[2726] = (((s.v[116]) as f64).abs() < 60.0);
            s.v[2726] = if s.b[2726] { 1.0 } else { 0.0 };
            if ((((s.v[2621] != 0.0) && s.b[2718]) && (!s.b[2725])) && s.b[2726]) {
                s.store_exp(227, 116);
                s.store_offset(335, 227, (-1.0));
                s.store_mul_sub_rhs(214, 222, 335, 116);
                s.store_mul3_lhs(215, 222, 154, 335);
            }
            if ((((s.v[2621] != 0.0) && s.b[2718]) && (!s.b[2725])) && (!s.b[2726])) {
                s.store_exp_mul(231, 154, 404);
                s.store_mul_ad_rhs(214, 405, A::add_scaled_offset_product_rhs(s.ad_value(231), 1.0, s.ad_value(229), s.ad_value(116), 1.0, (-1.0)));
                s.store_mul_ad_product_rhs(215, 405, s.ad_value(154), A::sub(s.ad_value(231), s.ad_value(229)));
            }
            s.b[2727] = (s.v[214] > 0.0);
            s.v[2727] = if s.b[2727] { 1.0 } else { 0.0 };
            if (((s.v[2621] != 0.0) && s.b[2718]) && s.b[2727]) {
                s.store_sqrt_add(216, 2645, 214);
                s.store_div_scaled_inputs2(217, s.ad_value(2646), 0.5, s.ad_value(215), 0.5, s.ad_value(216), 1.0);
            }
            s.b[2728] = (s.v[2645] > 0.0);
            s.v[2728] = if s.b[2728] { 1.0 } else { 0.0 };
            if ((((s.v[2621] != 0.0) && s.b[2718]) && (!s.b[2727])) && s.b[2728]) {
                s.store_sqrt(216, 2645);
                s.store_div_scaled_inputs(217, s.ad_value(2646), 0.5, s.ad_value(216), 1.0);
            }
            if ((((s.v[2621] != 0.0) && s.b[2718]) && (!s.b[2727])) && (!s.b[2728])) {
                s.store_scalar(216, 0.0);
                s.store_scalar(217, 0.0);
            }
            if ((s.v[2621] != 0.0) && s.b[2718]) {
                s.store_scale(216, 216, (if (s.v[116] >= 0.0) { 1.0 } else { (-1.0) }));
            }
            if ((s.v[2621] != 0.0) && s.b[2718]) {
                s.store_scale(217, 217, (if (s.v[116] >= 0.0) { 1.0 } else { (-1.0) }));
            }
            if ((s.v[2621] != 0.0) && s.b[2718]) {
                s.store_add_scaled_inputs_product_indices(232, 404, 1.0, 402, (-1.0), 212, 216, 1.0);
                s.store_offset_mul(233, 212, 217, 1.0);
            }
            s.b[2729] = (s.v[79] > 0.0);
            s.v[2729] = if s.b[2729] { 1.0 } else { 0.0 };
            let (assign74200_body56_e113653,) = {
    if (((s.v[2621] != 0.0) && s.b[2718]) && s.b[2729]) {
        let assign74200_body56_e113651: f64 = (s.v[421] + 1.0);
        (assign74200_body56_e113651,)
    } else {
        (s.v[97],)
    }
};
            s.v[97] = assign74200_body56_e113653;
            if (((s.v[2621] != 0.0) && s.b[2718]) && (!s.b[2729])) {
                s.store_div_scaled_inputs(236, s.ad_value(232), -1.0, s.ad_value(233), 1.0);
            }
            if (((s.v[2621] != 0.0) && s.b[2718]) && (!s.b[2729])) {
                s.store_scaled_offset_ad(93, {
                    if (1.0 >= ((s.v[404]) as f64).abs()) {
                        A::constant(1.0)
                    } else {
                        A::abs(s.ad_value(404))
                    }
                }, 1.0, (0.5 * 0.1));
            }
            s.b[2730] = (((s.v[236]) as f64).abs() > s.v[93]);
            s.v[2730] = if s.b[2730] { 1.0 } else { 0.0 };
            if ((((s.v[2621] != 0.0) && s.b[2718]) && (!s.b[2729])) && s.b[2730]) {
                s.store_scale(236, 93, (if (s.v[236] >= 0.0) { 1.0 } else { (-1.0) }));
            }
            if (((s.v[2621] != 0.0) && s.b[2718]) && (!s.b[2729])) {
                s.store_add(404, 404, 236);
            }
            s.b[2731] = ((((s.v[236]) as f64).abs() <= 1e-12) && (((s.v[232]) as f64).abs() <= 1e-8));
            s.v[2731] = if s.b[2731] { 1.0 } else { 0.0 };
            let (assign74200_body63_e113743,) = {
    if ((((s.v[2621] != 0.0) && s.b[2718]) && (!s.b[2729])) && s.b[2731]) {
        let assign74200_body63_e113741: f64 = (s.v[79] + 2.0);
        (assign74200_body63_e113741,)
    } else {
        (s.v[79],)
    }
};
            s.v[79] = assign74200_body63_e113743;
            let (assign74200_body64_e113751,) = {
    if ((s.v[2621] != 0.0) && s.b[2718]) {
        let assign74200_body64_e113749: f64 = (s.v[97] + 1.0);
        (assign74200_body64_e113749,)
    } else {
        (s.v[97],)
    }
};
            s.v[97] = assign74200_body64_e113751;
        }

        if ((s.v[2621] != 0.0) && s.b[2718]) {
            if (s.v[2645] >= 0.0) {
                s.store_scaled_sqrt(223, 2645, (if (s.v[116] >= 0.0) { 1.0 } else { (-1.0) }));
            } else {
                s.store_scalar(223, 0.0);
            }
        }

        if ((s.v[2621] != 0.0) && s.b[2718]) {
            s.store_mul(2635, 982, 223);
            s.store_mul(2636, 2637, 2635);
            s.store_offset_div(100, 2636, 410, (10.0 * 2.220446049250313e-16));
            s.store_mul(354, 410, 100);
            s.store_div_from_scalar_add_ad(335, 1.0, s.ad_value(216), s.ad_value(100));
            s.store_mul3_lhs(399, 410, 214, 335);
            s.store_add(398, 354, 399);
        }

        if (s.v[2621] != 0.0) {
            s.store_sub(399, 398, 354);
        }

        s.b[2733] = (s.v[407] < 0.0);
        s.v[2733] = if s.b[2733] { 1.0 } else { 0.0 };

        if ((s.v[2621] != 0.0) && s.b[2733]) {
            s.store_neg(407, 407);
        }

        s.b[2734] = (p.p55 == 0.0);
        s.v[2734] = if s.b[2734] { 1.0 } else { 0.0 };

        s.b[2735] = (p.p50 == 0.0);
        s.v[2735] = if s.b[2735] { 1.0 } else { 0.0 };

        if ((((s.v[2621] != 0.0) && s.b[2733]) && s.b[2734]) && s.b[2735]) {
            s.store_neg(2638, 404);
        }

        if ((((s.v[2621] != 0.0) && s.b[2733]) && s.b[2734]) && (!s.b[2735])) {
            s.copy_ad(2638, 396);
        }

        if (((s.v[2621] != 0.0) && s.b[2733]) && s.b[2734]) {
            s.store_sqrt_offset_ad(782, A::mul_offset_lhs(s.ad_value(2638), p.p137, A::offset(s.ad_value(2638), p.p137)), ((4.0 * 0.1) * 0.1));
            s.store_scaled_offset_ad(343, A::div_scaled_offset_numerator(s.ad_value(2638), 1.0, p.p137, s.ad_value(782), 1.0), 1.0, 0.5);
            s.store_scaled_add_ad_lhs(336, A::offset(s.ad_value(2638), p.p137), 782, 0.5);
        }

        s.b[2736] = (s.v[336] < 0.0);
        s.v[2736] = if s.b[2736] { 1.0 } else { 0.0 };

        if ((((s.v[2621] != 0.0) && s.b[2733]) && s.b[2734]) && s.b[2736]) {
            s.store_scalar(336, 0.0);
            s.store_scalar(343, 0.0);
        }

        if (((s.v[2621] != 0.0) && s.b[2733]) && s.b[2734]) {
            s.store_scaled_sqrt_ad(600, A::mul(s.ad_value(651), s.ad_value(336)), p.p432);
            s.store_add_scaled_inputs3_indices(781, 407, 1.0, 600, (-1.0), 407, (-0.1));
            s.store_scaled_mul(782, 407, 407, (4.0 * 0.1));
        }

        if (((s.v[2621] != 0.0) && s.b[2733]) && s.b[2734]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if (((s.v[2621] != 0.0) && s.b[2733]) && s.b[2734]) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_add_scaled_inputs3_indices(603, 407, 1.0, 781, (-0.5), 782, (-0.5));
            s.store_sub(407, 407, 603);
        }

        s.b[2737] = (1.0 == 1.0);
        s.v[2737] = if s.b[2737] { 1.0 } else { 0.0 };

        s.b[2738] = (1.0 == 2.0);
        s.v[2738] = if s.b[2738] { 1.0 } else { 0.0 };

        s.b[2739] = (1.0 == 3.0);
        s.v[2739] = if s.b[2739] { 1.0 } else { 0.0 };

        s.b[2740] = (1.0 == 4.0);
        s.v[2740] = if s.b[2740] { 1.0 } else { 0.0 };

        s.b[2741] = (p.p55 == 1.0);
        s.v[2741] = if s.b[2741] { 1.0 } else { 0.0 };

        if (((s.v[2621] != 0.0) && s.b[2737]) && s.b[2741]) {
            s.store_scale(338, 407, s.v[635]);
        }

        if (((s.v[2621] != 0.0) && s.b[2737]) && (!s.b[2741])) {
            s.store_scale(338, 407, (s.v[635] * (1.0 - s.v[526])));
        }

        if ((s.v[2621] != 0.0) && s.b[2737]) {
            s.store_mul(353, 338, 398);
            s.store_mul(356, 338, 354);
        }

        if ((s.v[2621] != 0.0) && (s.b[2738] && (!s.b[2737]))) {
            s.store_scale(338, 407, (s.v[635] * s.v[526]));
            s.store_mul(351, 338, 398);
            s.store_mul(359, 338, 354);
        }

        s.b[2742] = (p.p55 == 1.0);
        s.v[2742] = if s.b[2742] { 1.0 } else { 0.0 };

        if (((s.v[2621] != 0.0) && (s.b[2739] && (!(s.b[2737] || s.b[2738])))) && s.b[2742]) {
            s.store_scale(338, 407, s.v[635]);
        }

        if (((s.v[2621] != 0.0) && (s.b[2739] && (!(s.b[2737] || s.b[2738])))) && (!s.b[2742])) {
            s.store_scale(338, 407, (s.v[635] * (1.0 - s.v[526])));
        }

        if ((s.v[2621] != 0.0) && (s.b[2739] && (!(s.b[2737] || s.b[2738])))) {
            s.copy_ad(697, 404);
        }

        s.b[2743] = (p.p430 == 0.0);
        s.v[2743] = if s.b[2743] { 1.0 } else { 0.0 };

    }

    pub(super) fn stamp_transient_block_75(
        s: &mut Scratch,
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        if (((s.v[2621] != 0.0) && (s.b[2739] && (!(s.b[2737] || s.b[2738])))) && s.b[2743]) {
            s.copy_ad(698, 354);
        }

        if ((s.v[2621] != 0.0) && (s.b[2739] && (!(s.b[2737] || s.b[2738])))) {
            s.store_mul(352, 338, 398);
            s.store_mul(355, 338, 354);
            s.copy_ad(816, 355);
        }

        if ((s.v[2621] != 0.0) && (s.b[2740] && (!((s.b[2737] || s.b[2738]) || s.b[2739])))) {
            s.store_scale(338, 407, (s.v[635] * s.v[526]));
            s.store_mul(350, 338, 398);
            s.store_mul(358, 338, 354);
        }

        s.v[2621] = 0.0;

        s.b[2744] = (2.0 == 1.0);
        s.v[2744] = if s.b[2744] { 1.0 } else { 0.0 };

        s.b[2745] = (2.0 == 2.0);
        s.v[2745] = if s.b[2745] { 1.0 } else { 0.0 };

        s.b[2746] = (2.0 == 3.0);
        s.v[2746] = if s.b[2746] { 1.0 } else { 0.0 };

        s.b[2747] = (2.0 == 4.0);
        s.v[2747] = if s.b[2747] { 1.0 } else { 0.0 };

        s.b[2748] = (((p.p36 == 1.0) && (p.p66 > 0.0)) && (s.v[460] > 0.0));
        s.v[2748] = if s.b[2748] { 1.0 } else { 0.0 };

        let (assign74830_e114344,) = {
    if (s.b[2744] && s.b[2748]) {
        (1.0,)
    } else {
        (s.v[2621],)
    }
};
        s.v[2621] = assign74830_e114344;

        let (assign74840_e114350,) = {
    if (s.b[2744] && s.b[2748]) {
        (1.0,)
    } else {
        (s.v[2619],)
    }
};
        s.v[2619] = assign74840_e114350;

        if (s.b[2744] && s.b[2748]) {
            s.store_sub(395, 731, 728);
            s.store_neg(396, 728);
            s.store_scalar(409, s.v[460]);
            s.store_scalar(407, p.p66);
            s.store_scalar(411, 0.0);
            s.copy_ad(410, 687);
            s.store_scalar(413, s.v[188]);
        }

        s.b[2749] = (((((p.p36 == 1.0) && (p.p66 > 0.0)) && (s.v[460] > 0.0)) && (s.v[526] != 0.0)) && (p.p55 != 1.0));
        s.v[2749] = if s.b[2749] { 1.0 } else { 0.0 };

        let (assign74930_e114423,) = {
    if ((s.b[2745] && (!s.b[2744])) && s.b[2749]) {
        (1.0,)
    } else {
        (s.v[2621],)
    }
};
        s.v[2621] = assign74930_e114423;

        if ((s.b[2745] && (!s.b[2744])) && s.b[2749]) {
            s.store_sub(395, 734, 735);
            s.store_neg(396, 735);
        }

        s.b[2750] = (((p.p35 == 1.0) && (p.p63 > 0.0)) && (s.v[459] > 0.0));
        s.v[2750] = if s.b[2750] { 1.0 } else { 0.0 };

        let (assign74970_e114466,) = {
    if ((s.b[2746] && (!(s.b[2744] || s.b[2745]))) && s.b[2750]) {
        (1.0,)
    } else {
        (s.v[2621],)
    }
};
        s.v[2621] = assign74970_e114466;

        let (assign74980_e114477,) = {
    if ((s.b[2746] && (!(s.b[2744] || s.b[2745]))) && s.b[2750]) {
        (1.0,)
    } else {
        (s.v[2622],)
    }
};
        s.v[2622] = assign74980_e114477;

        if ((s.b[2746] && (!(s.b[2744] || s.b[2745]))) && s.b[2750]) {
            s.store_sub(395, 731, 728);
            s.store_sub(396, 729, 728);
            s.store_scalar(409, s.v[459]);
            s.store_scalar(407, (p.p63 + (p.p64 * p.p55)));
            s.copy_ad(411, 384);
            s.copy_ad(410, 686);
            s.copy_ad(413, 412);
            s.store_neg(407, 407);
        }

        s.b[2751] = (((s.v[407] < 0.0) && (p.p432 > 0.0)) && (p.p55 == 1.0));
        s.v[2751] = if s.b[2751] { 1.0 } else { 0.0 };

        if (((s.b[2746] && (!(s.b[2744] || s.b[2745]))) && s.b[2750]) && s.b[2751]) {
            s.store_neg(407, 407);
            s.store_scalar(335, p.p63);
            s.store_offset_div_scaled_product(996, s.ad_value(335), s.ad_value(335), 1.0, s.ad_value(651), 1.0, (-p.p137));
        }

        s.b[2752] = (p.p113 > 0.0);
        s.v[2752] = if s.b[2752] { 1.0 } else { 0.0 };

        s.b[2753] = ((s.v[396] == 0.0) || (p.p113 <= 0.0));
        s.v[2753] = if s.b[2753] { 1.0 } else { 0.0 };

        if (((((s.b[2746] && (!(s.b[2744] || s.b[2745]))) && s.b[2750]) && s.b[2751]) && s.b[2752]) && s.b[2753]) {
        }

        if (((((s.b[2746] && (!(s.b[2744] || s.b[2745]))) && s.b[2750]) && s.b[2751]) && s.b[2752]) && (!s.b[2753])) {
            s.store_scalar(783, (if (s.v[396] < 0.0) { (-1.0) } else { 1.0 }));
        }

        if (((((s.b[2746] && (!(s.b[2744] || s.b[2745]))) && s.b[2750]) && s.b[2751]) && s.b[2752]) && (!s.b[2753])) {
            s.store_mul(784, 783, 396);
            s.store_offset_powf_ad(781, A::div(s.ad_value(784), s.ad_value(996)), p.p113, 1.0);
            s.store_powf(782, 781, (1.0 / p.p113));
            s.store_div_scaled_product_indices(396, 783, 784, 1.0, 782, 1.0);
        }

        if ((((s.b[2746] && (!(s.b[2744] || s.b[2745]))) && s.b[2750]) && s.b[2751]) && s.b[2752]) {
            s.store_sqrt_offset_ad(782, A::mul_offset_lhs(s.ad_value(396), p.p137, A::offset(s.ad_value(396), p.p137)), ((4.0 * 0.1) * 0.1));
            s.store_scaled_offset_ad(343, A::div_scaled_offset_numerator(s.ad_value(396), 1.0, p.p137, s.ad_value(782), 1.0), 1.0, 0.5);
            s.store_scaled_add_ad_lhs(336, A::offset(s.ad_value(396), p.p137), 782, 0.5);
        }

        s.b[2754] = (s.v[336] < 0.0);
        s.v[2754] = if s.b[2754] { 1.0 } else { 0.0 };

        if (((((s.b[2746] && (!(s.b[2744] || s.b[2745]))) && s.b[2750]) && s.b[2751]) && s.b[2752]) && s.b[2754]) {
            s.store_scalar(336, 0.0);
            s.store_scalar(343, 0.0);
        }

        if ((((s.b[2746] && (!(s.b[2744] || s.b[2745]))) && s.b[2750]) && s.b[2751]) && s.b[2752]) {
            s.store_scaled_sqrt_ad(600, A::mul(s.ad_value(651), s.ad_value(336)), p.p432);
            s.store_sub(407, 407, 600);
        }

        s.b[2755] = (((((p.p35 == 1.0) && (p.p63 > 0.0)) && (s.v[459] > 0.0)) && (s.v[526] != 0.0)) && (p.p55 != 1.0));
        s.v[2755] = if s.b[2755] { 1.0 } else { 0.0 };

        let (assign75280_e114948,) = {
    if ((s.b[2747] && (!((s.b[2744] || s.b[2745]) || s.b[2746]))) && s.b[2755]) {
        (1.0,)
    } else {
        (s.v[2621],)
    }
};
        s.v[2621] = assign75280_e114948;

        if ((s.b[2747] && (!((s.b[2744] || s.b[2745]) || s.b[2746]))) && s.b[2755]) {
            s.store_sub(395, 734, 735);
            s.store_sub(396, 733, 735);
        }

        if (s.v[2621] != 0.0) {
            s.store_scalar(2763, 0.4);
        }

        let (assign75330_e114990,) = {
    if (s.v[2621] != 0.0) {
        (0.0,)
    } else {
        (s.v[2764],)
    }
};
        s.v[2764] = assign75330_e114990;

        if (s.v[2621] != 0.0) {
            s.store_scalar(223, 0.0);
            s.store_scalar(214, 0.0);
            s.store_scalar(216, 0.0);
            s.store_scalar(232, 0.0);
            s.store_scalar(236, 0.0);
            s.store_scalar(233, 0.0);
            s.store_scalar(217, 0.0);
            s.store_scalar(420, 0.0);
            s.store_scalar(215, 0.0);
            s.store_scalar(447, 0.0);
            s.store_scalar(445, 0.0);
            s.store_scalar(446, 0.0);
        }

        let (assign75460_e115043,) = {
    if (s.v[2621] != 0.0) {
        let assign75460_e115041: f64 = (-1.0);
        (assign75460_e115041,)
    } else {
        (s.v[79],)
    }
};
        s.v[79] = assign75460_e115043;

        if (s.v[2621] != 0.0) {
            s.store_scalar(2765, 0.0);
            s.store_scalar(2766, 0.0);
            s.store_mul_scaled_ln_ad_rhs(2761, 155, 2.0, A::div(s.ad_value(409), s.ad_value(394)));
            s.store_offset_sub_from_scalar_ad(781, 0.8, s.ad_value(2761), (-0.1));
            s.store_scalar(782, ((4.0 * 0.8) * 0.1));
        }

        if (s.v[2621] != 0.0) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if (s.v[2621] != 0.0) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(2762, 781, (-0.5), 782, (-0.5), 0.8);
        }

        s.b[2768] = (s.v[2763] > (s.v[2762] * 0.5));
        s.v[2768] = if s.b[2768] { 1.0 } else { 0.0 };

        if ((s.v[2621] != 0.0) && s.b[2768]) {
            s.store_scale(2763, 2762, 0.5);
        }

        s.b[2769] = param_given[338];
        s.v[2769] = if s.b[2769] { 1.0 } else { 0.0 };

        if ((s.v[2621] != 0.0) && s.b[2769]) {
            s.store_scalar(2762, p.p338);
        }

        s.b[2770] = param_given[339];
        s.v[2770] = if s.b[2770] { 1.0 } else { 0.0 };

        if ((s.v[2621] != 0.0) && s.b[2770]) {
            s.store_scalar(2763, p.p339);
        }

        s.b[2771] = param_given[338];
        s.v[2771] = if s.b[2771] { 1.0 } else { 0.0 };

        if (((s.v[2621] != 0.0) && (!s.b[2770])) && s.b[2771]) {
            s.store_scale(2763, 2762, 0.5);
        }

        s.b[2772] = (s.v[2763] > (s.v[2762] * 0.5));
        s.v[2772] = if s.b[2772] { 1.0 } else { 0.0 };

        if ((s.v[2621] != 0.0) && s.b[2772]) {
            s.store_scale(2763, 2762, 0.5);
        }

        s.b[2773] = (p.p38 == 1.0);
        s.v[2773] = if s.b[2773] { 1.0 } else { 0.0 };

        if ((s.v[2621] != 0.0) && s.b[2773]) {
            s.store_neg(334, 396);
        }

        s.b[2774] = (s.v[334] > s.v[2763]);
        s.v[2774] = if s.b[2774] { 1.0 } else { 0.0 };

        if (((s.v[2621] != 0.0) && s.b[2773]) && s.b[2774]) {
            s.store_sub(335, 334, 2763);
            s.store_sub(336, 2762, 2763);
            s.store_div(781, 335, 336);
            s.store_square(782, 781);
            s.store_mul(783, 782, 781);
            s.store_square(784, 782);
            s.store_div_from_scalar_ad(780, 1.0, A::add_scaled_inputs4_offset(s.ad_value(781), 1.0, s.ad_value(782), 1.0, s.ad_value(783), 1.0, s.ad_value(784), 1.0, 1.0));
            s.store_mul_ad_affine_product_lhs(345, A::add_scaled_inputs3_offset(s.ad_value(781), 2.0, s.ad_value(782), 3.0, s.ad_value(783), 4.0, 1.0), s.ad_value(780), -1.0, 0.0, 780);
            s.store_mul_sub_from_scalar_rhs(333, 336, 1.0, 780);
            s.store_add_scaled_sub_value_product_indices(334, 1.0, 780, 1.0, 781, 345, 1.0);
            s.store_neg(345, 345);
            s.store_add(344, 2763, 333);
        }

        if (((s.v[2621] != 0.0) && s.b[2773]) && (!s.b[2774])) {
            s.copy_ad(344, 334);
        }

        if ((s.v[2621] != 0.0) && s.b[2773]) {
            s.store_neg(397, 344);
        }

        if ((s.v[2621] != 0.0) && (!s.b[2773])) {
            s.copy_ad(397, 396);
        }

        if (s.v[2621] != 0.0) {
            s.store_div(212, 410, 413);
            s.store_square(213, 212);
            s.store_sub_from_scalar(402, s.v[458], 395);
        }

        let (assign75870_e115384,) = {
    if (s.v[2621] != 0.0) {
        let assign75870_e115378: f64 = (-s.v[397]);
        let assign75870_e115381: f64 = (10.0 * 2.220446049250313e-16);
        let assign75870_e115382: f64 = (assign75870_e115378 + assign75870_e115381);
        (assign75870_e115382,)
    } else {
        (s.v[403],)
    }
};
        s.v[403] = assign75870_e115384;

        if (s.v[2621] != 0.0) {
            s.store_scalar(2757, 0.0);
            s.store_scale(2758, 409, 1.6021918e-19);
        }

    }

    pub(super) fn stamp_transient_block_76(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (s.v[2621] != 0.0) {
            s.store_div(334, 394, 409);
            s.store_square(405, 334);
        }

        s.b[2775] = ((s.v[154] * (-s.v[397])) >= 500.0);
        s.v[2775] = if s.b[2775] { 1.0 } else { 0.0 };

        if ((s.v[2621] != 0.0) && s.b[2775]) {
            s.store_scaled_offset_ad(229, A::mul_scaled_rhs(s.ad_value(154), s.ad_value(397), -1.0), ((1.0) + ((-500.0))), 1.403592217853e217);
            s.store_scalar(334, 1.403592217853e217);
        }

        if ((s.v[2621] != 0.0) && (!s.b[2775])) {
            s.store_mul_neg_rhs(781, 154, 397);
            s.store_scalar(229, 1.0);
        }

        let mut assign75970_loop_guard: usize = 0;
        while {
            let assign75970_cond_e115458: f64 = if (((s.v[2621] != 0.0) && (!s.b[2775])) && (s.v[781] >= 60.0)) { 1.0 } else { 0.0 };
            assign75970_cond_e115458 != 0.0
        } {
            assign75970_loop_guard += 1;
            assert!(assign75970_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((s.v[2621] != 0.0) && (!s.b[2775])) {
                s.store_scale(229, 229, 1.14200738981568e26);
                s.store_offset(781, 781, (-60.0));
            }
        }

        if ((s.v[2621] != 0.0) && (!s.b[2775])) {
            s.store_mul_exp_rhs(229, 229, 781);
            s.copy_ad(334, 229);
        }

        if ((s.v[2621] != 0.0) && (s.v[406] != 0.0)) {
            s.store_offset_scaled(781, 402, (-0.5), (((-0.5)) + ((-1.0))));
            s.store_scalar(782, (4.0 * 0.5));
        }

        if ((s.v[2621] != 0.0) && (s.v[406] != 0.0)) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if ((s.v[2621] != 0.0) && (s.v[406] != 0.0)) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(335, 781, 0.5, 782, 0.5, 0.5);
        }

        s.b[2776] = (((s.v[402] + s.v[397]) > (-s.v[335])) && (s.v[335] >= 0.0));
        s.v[2776] = if s.b[2776] { 1.0 } else { 0.0 };

        if (((s.v[2621] != 0.0) && (s.v[406] != 0.0)) && s.b[2776]) {
            s.store_add_scaled_inputs3_indices(781, 402, 1.0, 397, 1.0, 335, 1.0);
            s.store_square(722, 781);
            s.store_square(723, 335);
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
        }

        let (assign76120_e115632,) = {
    if (((s.v[2621] != 0.0) && (s.v[406] != 0.0)) && s.b[2776]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign76120_e115632;

        let (assign76130_e115640,) = {
    if (((s.v[2621] != 0.0) && (s.v[406] != 0.0)) && s.b[2776]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign76130_e115640;

        if (((s.v[2621] != 0.0) && (s.v[406] != 0.0)) && s.b[2776]) {
            s.store_scalar(770, 0.0);
            s.store_scalar(726, 0.0);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_add(770, 724, 725);
            s.copy_ad(726, 770);
        }

        s.b[2777] = ((((1.0 == 1.0) || (1.0 == 2.0)) || (1.0 == 4.0)) || (1.0 == 8.0));
        s.v[2777] = if s.b[2777] { 1.0 } else { 0.0 };

        s.b[2778] = (1.0 == 1.0);
        s.v[2778] = if s.b[2778] { 1.0 } else { 0.0 };

        let (assign76220_e115724,) = {
    if (((((s.v[2621] != 0.0) && (s.v[406] != 0.0)) && s.b[2776]) && s.b[2777]) && s.b[2778]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign76220_e115724;

        s.b[2779] = (1.0 == 2.0);
        s.v[2779] = if s.b[2779] { 1.0 } else { 0.0 };

        let (assign76240_e115742,) = {
    if ((((((s.v[2621] != 0.0) && (s.v[406] != 0.0)) && s.b[2776]) && s.b[2777]) && (!s.b[2778])) && s.b[2779]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign76240_e115742;

        s.b[2780] = (1.0 == 4.0);
        s.v[2780] = if s.b[2780] { 1.0 } else { 0.0 };

        let (assign76260_e115763,) = {
    if (((((((s.v[2621] != 0.0) && (s.v[406] != 0.0)) && s.b[2776]) && s.b[2777]) && (!s.b[2778])) && (!s.b[2779])) && s.b[2780]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign76260_e115763;

        s.b[2781] = (1.0 == 8.0);
        s.v[2781] = if s.b[2781] { 1.0 } else { 0.0 };

        let (assign76280_e115787,) = {
    if ((((((((s.v[2621] != 0.0) && (s.v[406] != 0.0)) && s.b[2776]) && s.b[2777]) && (!s.b[2778])) && (!s.b[2779])) && (!s.b[2780])) && s.b[2781]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign76280_e115787;

        let (assign76290_e115797,) = {
    if ((((s.v[2621] != 0.0) && (s.v[406] != 0.0)) && s.b[2776]) && s.b[2777]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign76290_e115797;

        let mut assign76300_loop_guard: usize = 0;
        while {
            let assign76300_cond_e115808: f64 = if (((((s.v[2621] != 0.0) && (s.v[406] != 0.0)) && s.b[2776]) && s.b[2777]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign76300_cond_e115808 != 0.0
        } {
            assign76300_loop_guard += 1;
            assert!(assign76300_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.v[2621] != 0.0) && (s.v[406] != 0.0)) && s.b[2776]) && s.b[2777]) {
                s.store_sqrt(726, 726);
            }
            let (assign76300_body1_e115831,) = {
    if ((((s.v[2621] != 0.0) && (s.v[406] != 0.0)) && s.b[2776]) && s.b[2777]) {
        let assign76300_body1_e115829: f64 = (s.v[719] + 1.0);
        (assign76300_body1_e115829,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign76300_body1_e115831;
        }

        if ((((s.v[2621] != 0.0) && (s.v[406] != 0.0)) && s.b[2776]) && (!s.b[2777])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / 2.0));
            }
        }

        if (((s.v[2621] != 0.0) && (s.v[406] != 0.0)) && s.b[2776]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_mul3_lhs(780, 781, 335, 726);
            s.store_div_scaled_product3_indices(334, 335, 725, 726, 1.0, 770, 1.0);
            s.store_add_scaled_inputs(335, 335, -1.0, 780, 1.0);
        }

        if (((s.v[2621] != 0.0) && (s.v[406] != 0.0)) && s.b[2776]) {
        }

        if (((s.v[2621] != 0.0) && (s.v[406] != 0.0)) && (!s.b[2776])) {
            s.store_add(335, 402, 397);
            s.store_scalar(334, 1.0);
        }

        if ((s.v[2621] != 0.0) && (s.v[406] != 0.0)) {
            s.store_sub(397, 335, 402);
        }

        let (assign76400_e115948,) = {
    if ((s.v[2621] != 0.0) && (s.v[406] != 0.0)) {
        let assign76400_e115942: f64 = (-s.v[397]);
        let assign76400_e115945: f64 = (10.0 * 2.220446049250313e-16);
        let assign76400_e115946: f64 = (assign76400_e115942 + assign76400_e115945);
        (assign76400_e115946,)
    } else {
        (s.v[403],)
    }
};
        s.v[403] = assign76400_e115948;

        s.b[2782] = (s.v[402] < s.v[403]);
        s.v[2782] = if s.b[2782] { 1.0 } else { 0.0 };

        if ((s.v[2621] != 0.0) && s.b[2782]) {
            s.store_mul_scaled_ln_ad_rhs(271, 155, 2.0, A::div_from_scalar((-s.v[270]), s.ad_value(212)));
            s.store_mul_add_rhs(332, 154, 402, 397);
            s.store_div_from_scalar_mul_ad(335, 1.0, s.ad_value(154), s.ad_value(410));
            s.store_mul(333, 335, 413);
            s.store_offset_scaled(279, 333, (3.0 * 1.414213562373095), 2.0);
            s.store_mul3_affine_lhs(277, 279, 279, 8.0, 0.0, 279);
            s.store_sub_from_scalar_scaled_mul_ad_rhs(278, (7.0 * 1.414213562373095), 333, A::offset(s.ad_value(332), (-2.0)), 9.0);
            s.store_square(276, 278);
        }

        s.b[2783] = (s.v[277] < (s.v[276] * 1e-8));
        s.v[2783] = if s.b[2783] { 1.0 } else { 0.0 };

        if (((s.v[2621] != 0.0) && s.b[2782]) && s.b[2783]) {
            s.store_div_scaled_inputs(274, s.ad_value(277), 0.5, s.ad_value(278), 1.0);
        }

        if (((s.v[2621] != 0.0) && s.b[2782]) && (!s.b[2783])) {
            s.store_sqrt_add(275, 277, 276);
            s.store_sub(274, 275, 278);
        }

        if ((s.v[2621] != 0.0) && s.b[2782]) {
            s.store_powf(273, 274, 0.3333333333333333);
            s.store_add_scaled_inputs_product_first_ad(272, A::sub_from_scalar(((-4.0) * 1.414213562373095), A::scale(s.ad_value(333), 12.0)), 1.0, 273, 2.0, 273, 273, 1.414213562373095);
            s.store_div(116, 272, 273);
            s.store_mul(335, 116, 155);
            s.store_div(336, 335, 271);
            s.store_sqrt_square_offset(337, 336, 1.0);
            s.store_sub_ad_lhs(404, A::div(s.ad_value(335), s.ad_value(337)), 397);
            s.store_sub(336, 402, 404);
            s.store_mul(398, 413, 336);
            s.copy_ad(354, 398);
            s.copy_ad(2765, 404);
        }

        s.b[2784] = ((s.v[154] * (s.v[402] + s.v[397])) < (1.0 + (((((10.0 * 2.220446049250313e-16) - 1.0) * s.v[213]) * s.v[156]) / 4.0)));
        s.v[2784] = if s.b[2784] { 1.0 } else { 0.0 };

        if (((s.v[2621] != 0.0) && (!s.b[2782])) && s.b[2784]) {
            s.store_add_scaled_product_indices(89, 402, 1.0, 213, 154, 0.5);
        }

        if (((s.v[2621] != 0.0) && (!s.b[2782])) && (!s.b[2784])) {
            s.store_offset_div_scaled_offset_numerator(332, A::mul(s.ad_value(154), A::add(s.ad_value(402), s.ad_value(397))), 4.0, ((-1.0) * 4.0), A::mul(s.ad_value(213), s.ad_value(156)), 1.0, 1.0);
            s.store_add_ad_rhs(89, 402, A::mul3_scaled_output(s.ad_value(213), s.ad_value(154), A::sub_from_scalar(1.0, A::sqrt(s.ad_value(332))), 1.0 / (2.0)));
        }

        if ((s.v[2621] != 0.0) && (!s.b[2782])) {
            s.store_mul_add_rhs(116, 154, 89, 397);
        }

        s.b[2785] = (s.v[116] >= 3.0);
        s.v[2785] = if s.b[2785] { 1.0 } else { 0.0 };

        if (((s.v[2621] != 0.0) && (!s.b[2782])) && s.b[2785]) {
            s.store_exp_neg_input(333, 116);
            s.store_offset_div_scaled_inputs2(332, A::offset(A::mul(s.ad_value(154), A::add(s.ad_value(402), s.ad_value(397))), (-1.0)), 4.0, s.ad_value(333), 4.0, A::mul(s.ad_value(213), s.ad_value(156)), 1.0, 1.0);
            s.store_add_ad_rhs(89, 402, A::mul3_scaled_output(s.ad_value(213), s.ad_value(154), A::sub_from_scalar(1.0, A::sqrt(s.ad_value(332))), 1.0 / (2.0)));
            s.store_mul_add_rhs(116, 154, 89, 397);
            s.store_exp_neg_input(333, 116);
            s.store_offset_div_scaled_inputs2(332, A::offset(A::mul(s.ad_value(154), A::add(s.ad_value(402), s.ad_value(397))), (-1.0)), 4.0, s.ad_value(333), 4.0, A::mul(s.ad_value(213), s.ad_value(156)), 1.0, 1.0);
            s.store_add_ad_rhs(89, 402, A::mul3_scaled_output(s.ad_value(213), s.ad_value(154), A::sub_from_scalar(1.0, A::sqrt(s.ad_value(332))), 1.0 / (2.0)));
            s.store_mul_add_rhs(116, 154, 89, 397);
        }

        if (((s.v[2621] != 0.0) && (!s.b[2782])) && (!s.b[2785])) {
            s.store_scalar(434, ((1.0 / (9.0 * ((2.0) as f64).sqrt())) - ((5.0 + (7.0 * (((-3.0)) as f64).exp())) / (54.0 * (((2.0 + (((-3.0)) as f64).exp())) as f64).sqrt()))));
            s.store_scalar(435, (((1.0 + (((-3.0)) as f64).exp()) / (2.0 * (((2.0 + (((-3.0)) as f64).exp())) as f64).sqrt())) - (((2.0) as f64).sqrt() / 3.0)));
            s.store_offset_div_from_scalar_ad(436, 1.0, A::mul(s.ad_value(154), s.ad_value(212)), (1.0 / ((2.0) as f64).sqrt()));
            s.store_div_scaled_inputs2(437, s.ad_value(402), -1.0, s.ad_value(397), -1.0, s.ad_value(212), 1.0);
            s.store_add_scaled_inputs3(441, A::div_scaled_product(A::square(s.ad_value(435)), s.ad_value(435), 1.0, A::mul3_scaled_output(s.ad_value(434), s.ad_value(434), s.ad_value(434), 27.0), 1.0), 1.0, A::div_scaled_product_by_product(s.ad_value(435), s.ad_value(436), 1.0, s.ad_value(434), s.ad_value(434), 6.0), (-1.0), A::div_scaled_inputs(s.ad_value(437), 1.0, s.ad_value(434), 2.0), 1.0);
            s.store_div_ad(440, A::add_scaled_square_product(s.ad_value(435), (-1.0), s.ad_value(434), s.ad_value(436), 3.0), A::mul_scaled_lhs(s.ad_value(434), 9.0, s.ad_value(434)));
            s.store_sqrt_ad(339, A::add_scaled_square_product(s.ad_value(441), 1.0, A::square(s.ad_value(440)), s.ad_value(440), 1.0));
            s.store_powf_ad(439, A::sub(s.ad_value(339), s.ad_value(441)), 0.3333333333333333);
            s.store_neg_ad(438, A::powf(A::add(s.ad_value(441), s.ad_value(339)), 0.3333333333333333));
            s.store_add_scaled_inputs3_mixed_iia(116, 439, 1.0, 438, 1.0, A::div_scaled_inputs(s.ad_value(435), 1.0, s.ad_value(434), 3.0), -1.0);
            s.store_add_scaled_product_indices(89, 397, (-1.0), 116, 155, 1.0);
        }

        s.b[2786] = (p.p33 > 0.0);
        s.v[2786] = if s.b[2786] { 1.0 } else { 0.0 };

        if (((s.v[2621] != 0.0) && (!s.b[2782])) && s.b[2786]) {
            s.store_offset_add(442, 402, 397, 0.1);
            s.store_mul(222, 405, 229);
            s.store_mul(443, 405, 229);
            s.store_mul(334, 156, 213);
            s.store_mul(444, 154, 442);
            s.store_add_scaled_inputs_product_mixed_aaii(447, A::ln(A::add_scaled_square_product(s.ad_value(444), 1.0, s.ad_value(443), s.ad_value(334), 1.0)), 1.0, A::ln(A::mul(s.ad_value(405), s.ad_value(334))), (-1.0), 154, 397, 1.0);
        }

        s.b[2787] = (p.p33 == 2.0);
        s.v[2787] = if s.b[2787] { 1.0 } else { 0.0 };

        if ((((s.v[2621] != 0.0) && (!s.b[2782])) && s.b[2786]) && s.b[2787]) {
            s.store_offset_sub(781, 444, 447, (-1.0));
            s.store_scale(782, 444, 4.0);
        }

    }

    pub(super) fn stamp_transient_block_77(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((((s.v[2621] != 0.0) && (!s.b[2782])) && s.b[2786]) && s.b[2787]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if ((((s.v[2621] != 0.0) && (!s.b[2782])) && s.b[2786]) && s.b[2787]) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(335, 781, 782, 0.5, 0.5);
            s.store_add_scaled_inputs3_indices(447, 444, 1.0, 781, (-0.5), 782, (-0.5));
        }

        if ((((s.v[2621] != 0.0) && (!s.b[2782])) && s.b[2786]) && (!s.b[2787])) {
            if (s.v[447] <= s.v[444]) {
            } else {
                s.copy_ad(447, 444);
            }
        }

        if (((s.v[2621] != 0.0) && (!s.b[2782])) && s.b[2786]) {
            if (s.v[447] >= 0.0) {
            } else {
                s.store_scalar(447, 0.0);
            }
        }

        if (((s.v[2621] != 0.0) && (!s.b[2782])) && s.b[2786]) {
            s.store_sub(444, 444, 447);
            s.store_add_scaled_inputs(444, 444, 1.0, 154, 0.1);
            s.store_sub_ad(335, A::ln(A::add_scaled_square_product(s.ad_value(444), 1.0, s.ad_value(443), s.ad_value(334), 1.0)), A::ln(A::mul(s.ad_value(405), s.ad_value(334))));
            s.store_add_scaled_product_indices(446, 335, 1.0, 154, 397, 1.0);
        }

        if (((s.v[2621] != 0.0) && (!s.b[2782])) && s.b[2786]) {
            if (s.v[446] >= 0.0) {
            } else {
                s.store_scalar(446, 0.0);
            }
        }

        if (((s.v[2621] != 0.0) && (!s.b[2782])) && s.b[2786]) {
            s.copy_ad(445, 116);
        }

        s.b[2788] = (p.p33 == 2.0);
        s.v[2788] = if s.b[2788] { 1.0 } else { 0.0 };

        s.b[2789] = ((s.v[445] > (s.v[446] - (0.2 * s.v[446]))) && ((0.2 * s.v[446]) >= 0.0));
        s.v[2789] = if s.b[2789] { 1.0 } else { 0.0 };

        if (((((s.v[2621] != 0.0) && (!s.b[2782])) && s.b[2786]) && s.b[2788]) && s.b[2789]) {
            s.store_add_scaled_inputs3_indices(781, 445, 1.0, 446, (-1.0), 446, 0.2);
            s.store_square(722, 781);
            s.store_scaled_mul(723, 446, 446, (0.2 * 0.2));
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
        }

        let (assign77230_e117094,) = {
    if (((((s.v[2621] != 0.0) && (!s.b[2782])) && s.b[2786]) && s.b[2788]) && s.b[2789]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign77230_e117094;

        let (assign77240_e117107,) = {
    if (((((s.v[2621] != 0.0) && (!s.b[2782])) && s.b[2786]) && s.b[2788]) && s.b[2789]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign77240_e117107;

        if (((((s.v[2621] != 0.0) && (!s.b[2782])) && s.b[2786]) && s.b[2788]) && s.b[2789]) {
            s.store_scalar(770, 0.0);
            s.store_scalar(726, 0.0);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_add(770, 724, 725);
            s.copy_ad(726, 770);
        }

        s.b[2790] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.v[2790] = if s.b[2790] { 1.0 } else { 0.0 };

        s.b[2791] = (2.0 == 1.0);
        s.v[2791] = if s.b[2791] { 1.0 } else { 0.0 };

        let (assign77350_e117256,) = {
    if (((((((s.v[2621] != 0.0) && (!s.b[2782])) && s.b[2786]) && s.b[2788]) && s.b[2789]) && s.b[2790]) && s.b[2791]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign77350_e117256;

        s.b[2792] = (2.0 == 2.0);
        s.v[2792] = if s.b[2792] { 1.0 } else { 0.0 };

        let (assign77370_e117279,) = {
    if ((((((((s.v[2621] != 0.0) && (!s.b[2782])) && s.b[2786]) && s.b[2788]) && s.b[2789]) && s.b[2790]) && (!s.b[2791])) && s.b[2792]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign77370_e117279;

        s.b[2793] = (2.0 == 4.0);
        s.v[2793] = if s.b[2793] { 1.0 } else { 0.0 };

        let (assign77390_e117305,) = {
    if (((((((((s.v[2621] != 0.0) && (!s.b[2782])) && s.b[2786]) && s.b[2788]) && s.b[2789]) && s.b[2790]) && (!s.b[2791])) && (!s.b[2792])) && s.b[2793]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign77390_e117305;

        s.b[2794] = (2.0 == 8.0);
        s.v[2794] = if s.b[2794] { 1.0 } else { 0.0 };

        let (assign77410_e117334,) = {
    if ((((((((((s.v[2621] != 0.0) && (!s.b[2782])) && s.b[2786]) && s.b[2788]) && s.b[2789]) && s.b[2790]) && (!s.b[2791])) && (!s.b[2792])) && (!s.b[2793])) && s.b[2794]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign77410_e117334;

        let (assign77420_e117349,) = {
    if ((((((s.v[2621] != 0.0) && (!s.b[2782])) && s.b[2786]) && s.b[2788]) && s.b[2789]) && s.b[2790]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign77420_e117349;

        let mut assign77430_loop_guard: usize = 0;
        while {
            let assign77430_cond_e117365: f64 = if (((((((s.v[2621] != 0.0) && (!s.b[2782])) && s.b[2786]) && s.b[2788]) && s.b[2789]) && s.b[2790]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign77430_cond_e117365 != 0.0
        } {
            assign77430_loop_guard += 1;
            assert!(assign77430_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((((s.v[2621] != 0.0) && (!s.b[2782])) && s.b[2786]) && s.b[2788]) && s.b[2789]) && s.b[2790]) {
                s.store_sqrt(726, 726);
            }
            let (assign77430_body1_e117398,) = {
    if ((((((s.v[2621] != 0.0) && (!s.b[2782])) && s.b[2786]) && s.b[2788]) && s.b[2789]) && s.b[2790]) {
        let assign77430_body1_e117396: f64 = (s.v[719] + 1.0);
        (assign77430_body1_e117396,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign77430_body1_e117398;
        }

        if ((((((s.v[2621] != 0.0) && (!s.b[2782])) && s.b[2786]) && s.b[2788]) && s.b[2789]) && (!s.b[2790])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }

        if (((((s.v[2621] != 0.0) && (!s.b[2782])) && s.b[2786]) && s.b[2788]) && s.b[2789]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_mul3_affine_lhs(780, 781, 446, 0.2, 0.0, 726);
            s.store_div_scaled_product3_indices(335, 446, 725, 726, 0.2, 770, 1.0);
            s.store_add_scaled_inputs3_indices(116, 446, 1.0, 446, (-0.2), 780, 1.0);
        }

        if (((((s.v[2621] != 0.0) && (!s.b[2782])) && s.b[2786]) && s.b[2788]) && s.b[2789]) {
        }

        if (((((s.v[2621] != 0.0) && (!s.b[2782])) && s.b[2786]) && s.b[2788]) && (!s.b[2789])) {
            s.copy_ad(116, 445);
            s.store_scalar(335, 1.0);
        }

        if ((((s.v[2621] != 0.0) && (!s.b[2782])) && s.b[2786]) && (!s.b[2788])) {
            if (s.v[445] <= s.v[446]) {
                s.copy_ad(116, 445);
            } else {
                s.copy_ad(116, 446);
            }
        }

        s.b[2795] = (p.p33 == 1.0);
        s.v[2795] = if s.b[2795] { 1.0 } else { 0.0 };

        if (((s.v[2621] != 0.0) && (!s.b[2782])) && s.b[2795]) {
            s.store_add_scaled_product_indices(404, 397, (-1.0), 116, 155, 1.0);
        }

        s.b[2796] = (s.v[411] > 0.0);
        s.v[2796] = if s.b[2796] { 1.0 } else { 0.0 };

        if ((((s.v[2621] != 0.0) && (!s.b[2782])) && s.b[2795]) && s.b[2796]) {
            s.store_sub_from_scalar(336, p.p334, 411);
        }

        if ((((s.v[2621] != 0.0) && (!s.b[2782])) && s.b[2795]) && (!s.b[2796])) {
            s.store_sqrt_offset_ad(782, A::mul_offset_lhs(s.ad_value(729), p.p137, A::offset(s.ad_value(729), p.p137)), ((4.0 * 0.1) * 0.1));
            s.store_scaled_offset_ad(343, A::div_scaled_offset_numerator(s.ad_value(729), 1.0, p.p137, s.ad_value(782), 1.0), 1.0, 0.5);
            s.store_scaled_add_ad_lhs(336, A::offset(s.ad_value(729), p.p137), 782, 0.5);
        }

        s.b[2797] = (s.v[336] < 0.0);
        s.v[2797] = if s.b[2797] { 1.0 } else { 0.0 };

        if (((((s.v[2621] != 0.0) && (!s.b[2782])) && s.b[2795]) && (!s.b[2796])) && s.b[2797]) {
            s.store_scalar(336, 0.0);
            s.store_scalar(343, 0.0);
        }

        if ((((s.v[2621] != 0.0) && (!s.b[2782])) && s.b[2795]) && (!s.b[2796])) {
            s.store_scaled_sqrt_ad(600, A::mul(s.ad_value(651), s.ad_value(336)), p.p432);
            s.store_sub_from_scalar(336, p.p334, 600);
        }

        if (((s.v[2621] != 0.0) && (!s.b[2782])) && s.b[2795]) {
            s.store_sqrt_square_offset(782, 336, ((4.0 * (p.p334 * 0.01)) * (p.p334 * 0.01)));
            s.store_offset_scaled_div(343, 336, 782, 0.5, 0.5);
            s.store_scaled_add(336, 336, 782, 0.5);
        }

        s.b[2798] = (s.v[336] < 0.0);
        s.v[2798] = if s.b[2798] { 1.0 } else { 0.0 };

        if ((((s.v[2621] != 0.0) && (!s.b[2782])) && s.b[2795]) && s.b[2798]) {
            s.store_scalar(336, 0.0);
            s.store_scalar(343, 0.0);
        }

        if (((s.v[2621] != 0.0) && (!s.b[2782])) && s.b[2795]) {
            s.copy_ad(386, 336);
            s.store_mul3_affine_lhs(418, 2758, 386, (0.5 * 9662367879.197212), 0.0, 386);
            s.store_sqrt_ad(334, A::mul_scaled_lhs(s.ad_value(154), 2.0, s.ad_value(418)));
            s.store_scaled_add_ad(335, A::limited_exp(s.ad_value(334)), A::limited_exp_scaled_input(s.ad_value(334), -1.0), 0.5);
            s.store_div_ad_lhs(419, A::ln(s.ad_value(335)), 418);
            s.store_add(414, 404, 397);
            s.store_mul_sub_rhs(333, 419, 414, 418);
        }

        s.b[2799] = (s.v[333] < 60.0);
        s.v[2799] = if s.b[2799] { 1.0 } else { 0.0 };

        if ((((s.v[2621] != 0.0) && (!s.b[2782])) && s.b[2795]) && s.b[2799]) {
            s.store_exp(335, 333);
            s.store_exp_mul_scaled_lhs_indices(334, 419, -1.0, 418);
            s.store_sub(336, 335, 334);
            s.store_div_ad_lhs(416, A::ln(A::offset(s.ad_value(336), 1.0)), 419);
        }

        if ((((s.v[2621] != 0.0) && (!s.b[2782])) && s.b[2795]) && (!s.b[2799])) {
            s.store_sub(416, 414, 418);
        }

        if (((s.v[2621] != 0.0) && (!s.b[2782])) && s.b[2795]) {
            s.store_mul(415, 154, 416);
        }

        s.b[2800] = ((s.v[415] > (s.v[116] / 100.0)) && (s.v[415] > 0.0));
        s.v[2800] = if s.b[2800] { 1.0 } else { 0.0 };

        let (assign77860_e117987,) = {
    if ((((s.v[2621] != 0.0) && (!s.b[2782])) && s.b[2795]) && s.b[2800]) {
        let assign77860_e117985: f64 = (s.v[2764] + 1.0);
        (assign77860_e117985,)
    } else {
        (s.v[2764],)
    }
};
        s.v[2764] = assign77860_e117987;

        if ((((s.v[2621] != 0.0) && (!s.b[2782])) && s.b[2795]) && s.b[2800]) {
            s.copy_ad(116, 447);
        }

        if ((s.v[2621] != 0.0) && (!s.b[2782])) {
            s.store_add_scaled_product_indices(404, 397, (-1.0), 116, 155, 1.0);
        }

        s.b[2801] = (((s.v[116]) as f64).abs() > 1e-6);
        s.v[2801] = if s.b[2801] { 1.0 } else { 0.0 };

        if (((s.v[2621] != 0.0) && (!s.b[2782])) && s.b[2801]) {
            s.store_add_ad(335, A::offset(s.ad_value(116), (-1.0)), A::exp_scaled_input(s.ad_value(116), -1.0));
            s.store_sqrt(336, 335);
        }

        if (((s.v[2621] != 0.0) && (!s.b[2782])) && (!s.b[2801])) {
            s.store_mul_scaled_sqrt_ad_rhs(336, 116, 0.7071067811865475, A::sub_from_scalar(1.0, A::scale(s.ad_value(116), 0.3333333333333333)));
        }

        if ((s.v[2621] != 0.0) && (!s.b[2782])) {
            s.store_mul(354, 410, 336);
            s.store_mul_sub_rhs(398, 413, 402, 404);
            s.store_div(2802, 354, 2758);
        }

        s.b[2804] = (p.p33 == 2.0);
        s.v[2804] = if s.b[2804] { 1.0 } else { 0.0 };

        s.b[2805] = ((s.v[2802] > (s.v[386] - (s.v[386] * 0.1))) && ((s.v[386] * 0.1) >= 0.0));
        s.v[2805] = if s.b[2805] { 1.0 } else { 0.0 };

        if ((((s.v[2621] != 0.0) && (!s.b[2782])) && s.b[2804]) && s.b[2805]) {
            s.store_add_scaled_inputs3_indices(781, 2802, 1.0, 386, (-1.0), 386, 0.1);
            s.store_square(722, 781);
            s.store_scaled_mul(723, 386, 386, (0.1 * 0.1));
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
        }

        let (assign78040_e118194,) = {
    if ((((s.v[2621] != 0.0) && (!s.b[2782])) && s.b[2804]) && s.b[2805]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign78040_e118194;

        let (assign78050_e118205,) = {
    if ((((s.v[2621] != 0.0) && (!s.b[2782])) && s.b[2804]) && s.b[2805]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign78050_e118205;

        if ((((s.v[2621] != 0.0) && (!s.b[2782])) && s.b[2804]) && s.b[2805]) {
            s.store_scalar(770, 0.0);
            s.store_scalar(726, 0.0);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_mul(724, 724, 722);
        }

    }

    pub(super) fn stamp_transient_block_78(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((((s.v[2621] != 0.0) && (!s.b[2782])) && s.b[2804]) && s.b[2805]) {
            s.store_mul(725, 725, 723);
            s.store_add(770, 724, 725);
            s.copy_ad(726, 770);
        }

        s.b[2806] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.v[2806] = if s.b[2806] { 1.0 } else { 0.0 };

        s.b[2807] = (2.0 == 1.0);
        s.v[2807] = if s.b[2807] { 1.0 } else { 0.0 };

        let (assign78160_e118336,) = {
    if ((((((s.v[2621] != 0.0) && (!s.b[2782])) && s.b[2804]) && s.b[2805]) && s.b[2806]) && s.b[2807]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign78160_e118336;

        s.b[2808] = (2.0 == 2.0);
        s.v[2808] = if s.b[2808] { 1.0 } else { 0.0 };

        let (assign78180_e118357,) = {
    if (((((((s.v[2621] != 0.0) && (!s.b[2782])) && s.b[2804]) && s.b[2805]) && s.b[2806]) && (!s.b[2807])) && s.b[2808]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign78180_e118357;

        s.b[2809] = (2.0 == 4.0);
        s.v[2809] = if s.b[2809] { 1.0 } else { 0.0 };

        let (assign78200_e118381,) = {
    if ((((((((s.v[2621] != 0.0) && (!s.b[2782])) && s.b[2804]) && s.b[2805]) && s.b[2806]) && (!s.b[2807])) && (!s.b[2808])) && s.b[2809]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign78200_e118381;

        s.b[2810] = (2.0 == 8.0);
        s.v[2810] = if s.b[2810] { 1.0 } else { 0.0 };

        let (assign78220_e118408,) = {
    if (((((((((s.v[2621] != 0.0) && (!s.b[2782])) && s.b[2804]) && s.b[2805]) && s.b[2806]) && (!s.b[2807])) && (!s.b[2808])) && (!s.b[2809])) && s.b[2810]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign78220_e118408;

        let (assign78230_e118421,) = {
    if (((((s.v[2621] != 0.0) && (!s.b[2782])) && s.b[2804]) && s.b[2805]) && s.b[2806]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign78230_e118421;

        let mut assign78240_loop_guard: usize = 0;
        while {
            let assign78240_cond_e118435: f64 = if ((((((s.v[2621] != 0.0) && (!s.b[2782])) && s.b[2804]) && s.b[2805]) && s.b[2806]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign78240_cond_e118435 != 0.0
        } {
            assign78240_loop_guard += 1;
            assert!(assign78240_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((((s.v[2621] != 0.0) && (!s.b[2782])) && s.b[2804]) && s.b[2805]) && s.b[2806]) {
                s.store_sqrt(726, 726);
            }
            let (assign78240_body1_e118464,) = {
    if (((((s.v[2621] != 0.0) && (!s.b[2782])) && s.b[2804]) && s.b[2805]) && s.b[2806]) {
        let assign78240_body1_e118462: f64 = (s.v[719] + 1.0);
        (assign78240_body1_e118462,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign78240_body1_e118464;
        }

        if (((((s.v[2621] != 0.0) && (!s.b[2782])) && s.b[2804]) && s.b[2805]) && (!s.b[2806])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }

        if ((((s.v[2621] != 0.0) && (!s.b[2782])) && s.b[2804]) && s.b[2805]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_mul3_affine_lhs(780, 781, 386, 0.1, 0.0, 726);
            s.store_div_scaled_product3_indices(334, 386, 725, 726, 0.1, 770, 1.0);
            s.store_add_scaled_inputs3_indices(335, 386, 1.0, 386, (-0.1), 780, 1.0);
        }

        if ((((s.v[2621] != 0.0) && (!s.b[2782])) && s.b[2804]) && s.b[2805]) {
        }

        if ((((s.v[2621] != 0.0) && (!s.b[2782])) && s.b[2804]) && (!s.b[2805])) {
            s.copy_ad(335, 2802);
            s.store_scalar(334, 1.0);
        }

        s.b[2811] = (s.v[334] < 1.0);
        s.v[2811] = if s.b[2811] { 1.0 } else { 0.0 };

        let (assign78340_e118606,) = {
    if ((((s.v[2621] != 0.0) && (!s.b[2782])) && s.b[2804]) && s.b[2811]) {
        let assign78340_e118604: f64 = (s.v[2764] + 2.0);
        (assign78340_e118604,)
    } else {
        (s.v[2764],)
    }
};
        s.v[2764] = assign78340_e118606;

        if (((s.v[2621] != 0.0) && (!s.b[2782])) && (!s.b[2804])) {
            if (s.v[2802] <= s.v[386]) {
                s.copy_ad(335, 2802);
            } else {
                s.copy_ad(335, 386);
            }
        }

        s.b[2812] = (s.v[2802] >= s.v[386]);
        s.v[2812] = if s.b[2812] { 1.0 } else { 0.0 };

        let (assign78370_e118638,) = {
    if ((((s.v[2621] != 0.0) && (!s.b[2782])) && (!s.b[2804])) && s.b[2812]) {
        let assign78370_e118636: f64 = (s.v[2764] + 2.0);
        (assign78370_e118636,)
    } else {
        (s.v[2764],)
    }
};
        s.v[2764] = assign78370_e118638;

        s.b[2813] = (s.v[2764] >= 2.0);
        s.v[2813] = if s.b[2813] { 1.0 } else { 0.0 };

        if (((s.v[2621] != 0.0) && (!s.b[2782])) && s.b[2813]) {
            s.copy_ad(2803, 404);
            s.store_mul(354, 335, 2758);
            s.store_sub_div_rhs_indices(404, 402, 354, 413);
        }

        s.b[2814] = (p.p33 == 2.0);
        s.v[2814] = if s.b[2814] { 1.0 } else { 0.0 };

        s.b[2815] = ((s.v[404] > (s.v[2803] - 0.1)) && (0.1 >= 0.0));
        s.v[2815] = if s.b[2815] { 1.0 } else { 0.0 };

        if (((((s.v[2621] != 0.0) && (!s.b[2782])) && s.b[2813]) && s.b[2814]) && s.b[2815]) {
            s.store_offset_sub(781, 404, 2803, 0.1);
            s.store_square(722, 781);
            s.store_scalar(723, (0.1 * 0.1));
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
        }

        let (assign78490_e118772,) = {
    if (((((s.v[2621] != 0.0) && (!s.b[2782])) && s.b[2813]) && s.b[2814]) && s.b[2815]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign78490_e118772;

        let (assign78500_e118785,) = {
    if (((((s.v[2621] != 0.0) && (!s.b[2782])) && s.b[2813]) && s.b[2814]) && s.b[2815]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign78500_e118785;

        if (((((s.v[2621] != 0.0) && (!s.b[2782])) && s.b[2813]) && s.b[2814]) && s.b[2815]) {
            s.store_scalar(770, 0.0);
            s.store_scalar(726, 0.0);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_add(770, 724, 725);
            s.copy_ad(726, 770);
        }

        s.b[2816] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.v[2816] = if s.b[2816] { 1.0 } else { 0.0 };

        s.b[2817] = (2.0 == 1.0);
        s.v[2817] = if s.b[2817] { 1.0 } else { 0.0 };

        let (assign78610_e118934,) = {
    if (((((((s.v[2621] != 0.0) && (!s.b[2782])) && s.b[2813]) && s.b[2814]) && s.b[2815]) && s.b[2816]) && s.b[2817]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign78610_e118934;

        s.b[2818] = (2.0 == 2.0);
        s.v[2818] = if s.b[2818] { 1.0 } else { 0.0 };

        let (assign78630_e118957,) = {
    if ((((((((s.v[2621] != 0.0) && (!s.b[2782])) && s.b[2813]) && s.b[2814]) && s.b[2815]) && s.b[2816]) && (!s.b[2817])) && s.b[2818]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign78630_e118957;

        s.b[2819] = (2.0 == 4.0);
        s.v[2819] = if s.b[2819] { 1.0 } else { 0.0 };

        let (assign78650_e118983,) = {
    if (((((((((s.v[2621] != 0.0) && (!s.b[2782])) && s.b[2813]) && s.b[2814]) && s.b[2815]) && s.b[2816]) && (!s.b[2817])) && (!s.b[2818])) && s.b[2819]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign78650_e118983;

        s.b[2820] = (2.0 == 8.0);
        s.v[2820] = if s.b[2820] { 1.0 } else { 0.0 };

        let (assign78670_e119012,) = {
    if ((((((((((s.v[2621] != 0.0) && (!s.b[2782])) && s.b[2813]) && s.b[2814]) && s.b[2815]) && s.b[2816]) && (!s.b[2817])) && (!s.b[2818])) && (!s.b[2819])) && s.b[2820]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign78670_e119012;

        let (assign78680_e119027,) = {
    if ((((((s.v[2621] != 0.0) && (!s.b[2782])) && s.b[2813]) && s.b[2814]) && s.b[2815]) && s.b[2816]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign78680_e119027;

        let mut assign78690_loop_guard: usize = 0;
        while {
            let assign78690_cond_e119043: f64 = if (((((((s.v[2621] != 0.0) && (!s.b[2782])) && s.b[2813]) && s.b[2814]) && s.b[2815]) && s.b[2816]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign78690_cond_e119043 != 0.0
        } {
            assign78690_loop_guard += 1;
            assert!(assign78690_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((((s.v[2621] != 0.0) && (!s.b[2782])) && s.b[2813]) && s.b[2814]) && s.b[2815]) && s.b[2816]) {
                s.store_sqrt(726, 726);
            }
            let (assign78690_body1_e119076,) = {
    if ((((((s.v[2621] != 0.0) && (!s.b[2782])) && s.b[2813]) && s.b[2814]) && s.b[2815]) && s.b[2816]) {
        let assign78690_body1_e119074: f64 = (s.v[719] + 1.0);
        (assign78690_body1_e119074,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign78690_body1_e119076;
        }

        if ((((((s.v[2621] != 0.0) && (!s.b[2782])) && s.b[2813]) && s.b[2814]) && s.b[2815]) && (!s.b[2816])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }

        if (((((s.v[2621] != 0.0) && (!s.b[2782])) && s.b[2813]) && s.b[2814]) && s.b[2815]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, 0.1);
            s.store_div_scaled_product_indices(334, 725, 726, 0.1, 770, 1.0);
            s.store_add_offset_lhs(404, 2803, (-0.1), 780);
        }

        if (((((s.v[2621] != 0.0) && (!s.b[2782])) && s.b[2813]) && s.b[2814]) && s.b[2815]) {
        }

        if (((((s.v[2621] != 0.0) && (!s.b[2782])) && s.b[2813]) && s.b[2814]) && (!s.b[2815])) {
        }

        if (((((s.v[2621] != 0.0) && (!s.b[2782])) && s.b[2813]) && s.b[2814]) && (!s.b[2815])) {
            s.store_scalar(334, 1.0);
        }

        if ((((s.v[2621] != 0.0) && (!s.b[2782])) && s.b[2813]) && (!s.b[2814])) {
            if (s.v[404] <= s.v[2803]) {
            } else {
                s.copy_ad(404, 2803);
            }
        }

        if ((s.v[2621] != 0.0) && (!s.b[2782])) {
            s.copy_ad(2765, 404);
        }

        s.b[2821] = (p.p33 == 1.0);
        s.v[2821] = if s.b[2821] { 1.0 } else { 0.0 };

        let (assign78810_e119248,) = {
    if (((s.v[2621] != 0.0) && (!s.b[2782])) && s.b[2821]) {
        (0.0,)
    } else {
        (s.v[79],)
    }
};
        s.v[79] = assign78810_e119248;

        if (((s.v[2621] != 0.0) && (!s.b[2782])) && s.b[2821]) {
            s.store_sqrt_scaled_input_ad(982, A::mul(A::div_from_scalar(1.034943e-10, s.ad_value(2758)), s.ad_value(155)), 2.0);
        }

        s.b[2822] = (s.v[411] > 0.0);
        s.v[2822] = if s.b[2822] { 1.0 } else { 0.0 };

        if ((((s.v[2621] != 0.0) && (!s.b[2782])) && s.b[2821]) && s.b[2822]) {
            s.store_sub_from_scalar(336, p.p334, 411);
        }

        if ((((s.v[2621] != 0.0) && (!s.b[2782])) && s.b[2821]) && (!s.b[2822])) {
            s.store_sqrt_offset_ad(782, A::mul_offset_lhs(s.ad_value(729), p.p137, A::offset(s.ad_value(729), p.p137)), ((4.0 * 0.1) * 0.1));
            s.store_scaled_offset_ad(343, A::div_scaled_offset_numerator(s.ad_value(729), 1.0, p.p137, s.ad_value(782), 1.0), 1.0, 0.5);
            s.store_scaled_add_ad_lhs(336, A::offset(s.ad_value(729), p.p137), 782, 0.5);
        }

        s.b[2823] = (s.v[336] < 0.0);
        s.v[2823] = if s.b[2823] { 1.0 } else { 0.0 };

        if (((((s.v[2621] != 0.0) && (!s.b[2782])) && s.b[2821]) && (!s.b[2822])) && s.b[2823]) {
            s.store_scalar(336, 0.0);
            s.store_scalar(343, 0.0);
        }

        if ((((s.v[2621] != 0.0) && (!s.b[2782])) && s.b[2821]) && (!s.b[2822])) {
            s.store_scaled_sqrt_ad(600, A::mul(s.ad_value(651), s.ad_value(336)), p.p432);
            s.store_sub_from_scalar(336, p.p334, 600);
        }

        if (((s.v[2621] != 0.0) && (!s.b[2782])) && s.b[2821]) {
            s.store_sqrt_square_offset(782, 336, ((4.0 * (p.p334 * 0.01)) * (p.p334 * 0.01)));
            s.store_offset_scaled_div(343, 336, 782, 0.5, 0.5);
            s.store_scaled_add(336, 336, 782, 0.5);
        }

        s.b[2824] = (s.v[336] < 0.0);
        s.v[2824] = if s.b[2824] { 1.0 } else { 0.0 };

        if ((((s.v[2621] != 0.0) && (!s.b[2782])) && s.b[2821]) && s.b[2824]) {
            s.store_scalar(336, 0.0);
            s.store_scalar(343, 0.0);
        }

        if (((s.v[2621] != 0.0) && (!s.b[2782])) && s.b[2821]) {
            s.copy_ad(386, 336);
            s.store_mul3_affine_lhs(418, 2758, 386, (0.5 * 9662367879.197212), 0.0, 386);
            s.store_sqrt_ad(334, A::mul_scaled_lhs(s.ad_value(154), 2.0, s.ad_value(418)));
            s.store_scaled_add_ad(335, A::limited_exp(s.ad_value(334)), A::limited_exp_scaled_input(s.ad_value(334), -1.0), 0.5);
            s.store_div_ad_lhs(419, A::ln(s.ad_value(335)), 418);
        }

        let (assign79040_e119557,) = {
    if (((s.v[2621] != 0.0) && (!s.b[2782])) && s.b[2821]) {
        (1.0,)
    } else {
        (s.v[97],)
    }
};
        s.v[97] = assign79040_e119557;

    }

    pub(super) fn stamp_transient_block_79(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        let mut assign79050_loop_guard: usize = 0;
        while {
            let assign79050_cond_e119567: f64 = (s.v[421] + 1.0);
            let assign79050_cond_e119569: f64 = if ((((s.v[2621] != 0.0) && (!s.b[2782])) && s.b[2821]) && (s.v[97] <= assign79050_cond_e119567)) { 1.0 } else { 0.0 };
            assign79050_cond_e119569 != 0.0
        } {
            assign79050_loop_guard += 1;
            assert!(assign79050_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.v[2621] != 0.0) && (!s.b[2782])) && s.b[2821]) {
                s.store_add(414, 404, 397);
                s.store_mul(116, 154, 414);
                s.store_mul_sub_rhs(333, 419, 414, 418);
            }
            s.b[2826] = (s.v[333] < 60.0);
            s.v[2826] = if s.b[2826] { 1.0 } else { 0.0 };
            if ((((s.v[2621] != 0.0) && (!s.b[2782])) && s.b[2821]) && s.b[2826]) {
                s.store_exp(335, 333);
                s.store_exp_mul_scaled_lhs_indices(334, 419, -1.0, 418);
                s.store_sub(336, 335, 334);
                s.store_div_ad_lhs(416, A::ln(A::offset(s.ad_value(336), 1.0)), 419);
                s.store_div_scaled_value_offset_denominator(417, s.ad_value(335), 1.0, s.ad_value(336), 1.0, 1.0);
            }
            if ((((s.v[2621] != 0.0) && (!s.b[2782])) && s.b[2821]) && (!s.b[2826])) {
                s.store_sub(416, 414, 418);
                s.store_scalar(417, 1.0);
            }
            if (((s.v[2621] != 0.0) && (!s.b[2782])) && s.b[2821]) {
                s.store_mul(415, 154, 416);
            }
            s.b[2827] = (s.v[116] < 0.0);
            s.v[2827] = if s.b[2827] { 1.0 } else { 0.0 };
            if ((((s.v[2621] != 0.0) && (!s.b[2782])) && s.b[2821]) && s.b[2827]) {
                s.store_scalar(334, (-0.7071067811865475));
                s.store_mul(223, 116, 334);
                s.store_mul(420, 154, 334);
            }
            s.b[2828] = (s.v[116] < 1e-6);
            s.v[2828] = if s.b[2828] { 1.0 } else { 0.0 };
            if (((((s.v[2621] != 0.0) && (!s.b[2782])) && s.b[2821]) && (!s.b[2827])) && s.b[2828]) {
                s.store_square_mul_sub_from_scalar_double_scaled_sub_rhs_scaled_output(334, 116, 1.0, 116, 1.0, 116, 1.0, 116, 0.2, 1.0 / (4.0), 1.0 / (3.0), 1.0 / (2.0));
                s.store_mul_sub_from_scalar_ad_rhs(335, 116, 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(116), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(116), 1.0, A::scale(s.ad_value(116), 0.25), 1.0 / (3.0)), 1.0 / (2.0)));
                s.store_square_mul_sub_from_scalar_double_scaled_sub_rhs_scaled_output(336, 415, 1.0, 415, 1.0, 415, 1.0, 415, 0.2, 1.0 / (4.0), 1.0 / (3.0), 1.0 / (2.0));
                s.store_mul_sub_from_scalar_ad_rhs(337, 415, 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(415), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(415), 1.0, A::scale(s.ad_value(415), 0.25), 1.0 / (3.0)), 1.0 / (2.0)));
                s.store_sub(338, 334, 336);
            }
            s.b[2829] = (s.v[338] > 0.0);
            s.v[2829] = if s.b[2829] { 1.0 } else { 0.0 };
            if ((((((s.v[2621] != 0.0) && (!s.b[2782])) && s.b[2821]) && (!s.b[2827])) && s.b[2828]) && s.b[2829]) {
                s.store_sqrt(223, 338);
                s.store_div_scaled_product_right_ad(420, 154, A::add_scaled_product(s.ad_value(335), 1.0, s.ad_value(417), s.ad_value(337), (-1.0)), 0.5, 223, 1.0);
            }
            if ((((((s.v[2621] != 0.0) && (!s.b[2782])) && s.b[2821]) && (!s.b[2827])) && s.b[2828]) && (!s.b[2829])) {
                s.store_scalar(223, 0.0);
                s.store_scalar(420, 0.0);
            }
            if (((((s.v[2621] != 0.0) && (!s.b[2782])) && s.b[2821]) && (!s.b[2827])) && (!s.b[2828])) {
                s.store_exp_neg_input(334, 116);
                s.store_exp_neg_input(335, 415);
                s.store_add_scaled_inputs4_indices(338, 116, 1.0, 415, (-1.0), 334, 1.0, 335, (-1.0));
            }
            s.b[2830] = (s.v[338] > 0.0);
            s.v[2830] = if s.b[2830] { 1.0 } else { 0.0 };
            if ((((((s.v[2621] != 0.0) && (!s.b[2782])) && s.b[2821]) && (!s.b[2827])) && (!s.b[2828])) && s.b[2830]) {
                s.store_sqrt(223, 338);
                s.store_div_scaled_product_right_ad(420, 154, A::sub(A::sub_from_scalar(1.0, s.ad_value(334)), A::mul_sub_from_scalar_rhs(s.ad_value(417), 1.0, s.ad_value(335))), 0.5, 223, 1.0);
            }
            if ((((((s.v[2621] != 0.0) && (!s.b[2782])) && s.b[2821]) && (!s.b[2827])) && (!s.b[2828])) && (!s.b[2830])) {
                s.store_scalar(223, 0.0);
                s.store_scalar(420, 0.0);
            }
            s.b[2831] = (s.v[116] < 0.0);
            s.v[2831] = if s.b[2831] { 1.0 } else { 0.0 };
            if ((((s.v[2621] != 0.0) && (!s.b[2782])) && s.b[2821]) && s.b[2831]) {
                s.store_scalar(214, 0.0);
                s.store_scalar(215, 0.0);
                s.store_neg(216, 223);
                s.store_neg(217, 420);
            }
            s.b[2832] = (s.v[116] < 60.0);
            s.v[2832] = if s.b[2832] { 1.0 } else { 0.0 };
            s.b[2833] = (s.v[116] < 5e-5);
            s.v[2833] = if s.b[2833] { 1.0 } else { 0.0 };
            if ((((((s.v[2621] != 0.0) && (!s.b[2782])) && s.b[2821]) && (!s.b[2831])) && s.b[2832]) && s.b[2833]) {
                s.store_scaled_mul_ad(334, A::square(s.ad_value(116)), A::offset(A::mul_scaled_lhs(s.ad_value(116), 1.0 / (3.0), A::offset(A::mul_scaled_lhs(s.ad_value(116), 1.0 / (4.0), A::scale_offset(s.ad_value(116), 0.2, 1.0)), 1.0)), 1.0), 1.0 / (2.0));
                s.store_mul_offset_ad_rhs(335, 116, A::mul_scaled_lhs(s.ad_value(116), 1.0 / (2.0), A::offset(A::mul_scaled_lhs(s.ad_value(116), 1.0 / (3.0), A::scale_offset(s.ad_value(116), 0.25, 1.0)), 1.0)), 1.0);
                s.store_mul(214, 222, 334);
                s.store_mul3_lhs(215, 222, 335, 154);
            }
            if ((((((s.v[2621] != 0.0) && (!s.b[2782])) && s.b[2821]) && (!s.b[2831])) && s.b[2832]) && (!s.b[2833])) {
                s.store_exp(227, 116);
                s.store_offset(335, 227, (-1.0));
                s.store_mul_sub_rhs(214, 222, 335, 116);
                s.store_mul3_lhs(215, 222, 154, 335);
            }
            if (((((s.v[2621] != 0.0) && (!s.b[2782])) && s.b[2821]) && (!s.b[2831])) && (!s.b[2832])) {
                s.store_exp_mul(231, 154, 404);
                s.store_mul_ad_rhs(214, 405, A::add_scaled_offset_product_rhs(s.ad_value(231), 1.0, s.ad_value(229), s.ad_value(116), 1.0, (-1.0)));
                s.store_mul_ad_product_rhs(215, 405, s.ad_value(154), A::sub(s.ad_value(231), s.ad_value(229)));
            }
            s.b[2834] = (s.v[214] > 0.0);
            s.v[2834] = if s.b[2834] { 1.0 } else { 0.0 };
            if (((((s.v[2621] != 0.0) && (!s.b[2782])) && s.b[2821]) && (!s.b[2831])) && s.b[2834]) {
                s.store_sqrt_square_add(216, 223, 214);
                s.store_div_scaled_add_product(217, s.ad_value(215), 0.5, s.ad_value(420), s.ad_value(223), (2.0 * 0.5), s.ad_value(216), 1.0);
            }
            if (((((s.v[2621] != 0.0) && (!s.b[2782])) && s.b[2821]) && (!s.b[2831])) && (!s.b[2834])) {
                s.copy_ad(216, 223);
                s.copy_ad(217, 420);
            }
            if (((s.v[2621] != 0.0) && (!s.b[2782])) && s.b[2821]) {
                s.store_add_scaled_inputs_product_indices(232, 404, 1.0, 402, (-1.0), 212, 216, 1.0);
                s.store_offset_mul(233, 212, 217, 1.0);
            }
            s.b[2835] = (s.v[79] == 1.0);
            s.v[2835] = if s.b[2835] { 1.0 } else { 0.0 };
            let (assign79050_body72_e120715,) = {
    if ((((s.v[2621] != 0.0) && (!s.b[2782])) && s.b[2821]) && s.b[2835]) {
        let assign79050_body72_e120713: f64 = (s.v[421] + 1.0);
        (assign79050_body72_e120713,)
    } else {
        (s.v[97],)
    }
};
            s.v[97] = assign79050_body72_e120715;
            if ((((s.v[2621] != 0.0) && (!s.b[2782])) && s.b[2821]) && (!s.b[2835])) {
                s.store_div_scaled_inputs(236, s.ad_value(232), -1.0, s.ad_value(233), 1.0);
            }
            if ((((s.v[2621] != 0.0) && (!s.b[2782])) && s.b[2821]) && (!s.b[2835])) {
                s.store_scaled_offset_ad(93, {
                    if (1.0 >= ((s.v[404]) as f64).abs()) {
                        A::constant(1.0)
                    } else {
                        A::abs(s.ad_value(404))
                    }
                }, 1.0, (0.5 * 0.1));
            }
            s.b[2836] = (((s.v[236]) as f64).abs() > s.v[93]);
            s.v[2836] = if s.b[2836] { 1.0 } else { 0.0 };
            if (((((s.v[2621] != 0.0) && (!s.b[2782])) && s.b[2821]) && (!s.b[2835])) && s.b[2836]) {
                s.store_scale(236, 93, (if (s.v[236] >= 0.0) { 1.0 } else { (-1.0) }));
            }
            if ((((s.v[2621] != 0.0) && (!s.b[2782])) && s.b[2821]) && (!s.b[2835])) {
                s.store_add(404, 404, 236);
            }
            s.b[2837] = ((((s.v[236]) as f64).abs() <= 1e-12) && (((s.v[232]) as f64).abs() <= 1e-8));
            s.v[2837] = if s.b[2837] { 1.0 } else { 0.0 };
            let (assign79050_body79_e120818,) = {
    if (((((s.v[2621] != 0.0) && (!s.b[2782])) && s.b[2821]) && (!s.b[2835])) && s.b[2837]) {
        (1.0,)
    } else {
        (s.v[79],)
    }
};
            s.v[79] = assign79050_body79_e120818;
            let (assign79050_body80_e120829,) = {
    if (((s.v[2621] != 0.0) && (!s.b[2782])) && s.b[2821]) {
        let assign79050_body80_e120827: f64 = (s.v[97] + 1.0);
        (assign79050_body80_e120827,)
    } else {
        (s.v[97],)
    }
};
            s.v[97] = assign79050_body80_e120829;
        }

        if (((s.v[2621] != 0.0) && (!s.b[2782])) && s.b[2821]) {
            s.store_mul(2756, 982, 223);
            s.store_mul(2757, 2758, 2756);
            s.store_offset_div(100, 2757, 410, (10.0 * 2.220446049250313e-16));
            s.store_mul(354, 410, 100);
            s.store_div_from_scalar_add_ad(335, 1.0, s.ad_value(216), s.ad_value(100));
            s.store_mul3_lhs(399, 410, 214, 335);
            s.store_add(398, 354, 399);
        }

        s.b[2839] = (p.p33 == 4.0);
        s.v[2839] = if s.b[2839] { 1.0 } else { 0.0 };

        if ((s.v[2621] != 0.0) && s.b[2839]) {
            s.store_exp_ad(229, A::mul_scaled_rhs(s.ad_value(154), s.ad_value(397), -1.0));
            s.store_div(334, 394, 409);
            s.store_square(405, 334);
            s.store_mul(222, 405, 229);
            s.copy_ad(404, 2765);
        }

        let (assign79200_e120966,) = {
    if ((s.v[2621] != 0.0) && s.b[2839]) {
        (0.0,)
    } else {
        (s.v[79],)
    }
};
        s.v[79] = assign79200_e120966;

        if ((s.v[2621] != 0.0) && s.b[2839]) {
            s.store_sqrt_scaled_input_ad(982, A::mul(A::div_from_scalar(1.034943e-10, s.ad_value(2758)), s.ad_value(155)), 2.0);
        }

        s.b[2840] = (s.v[411] > 0.0);
        s.v[2840] = if s.b[2840] { 1.0 } else { 0.0 };

        if (((s.v[2621] != 0.0) && s.b[2839]) && s.b[2840]) {
            s.store_sub_from_scalar(336, p.p334, 411);
        }

        if (((s.v[2621] != 0.0) && s.b[2839]) && (!s.b[2840])) {
            s.store_sqrt_offset_ad(782, A::mul_offset_lhs(s.ad_value(729), p.p137, A::offset(s.ad_value(729), p.p137)), ((4.0 * 0.1) * 0.1));
            s.store_scaled_offset_ad(343, A::div_scaled_offset_numerator(s.ad_value(729), 1.0, p.p137, s.ad_value(782), 1.0), 1.0, 0.5);
            s.store_scaled_add_ad_lhs(336, A::offset(s.ad_value(729), p.p137), 782, 0.5);
        }

        s.b[2841] = (s.v[336] < 0.0);
        s.v[2841] = if s.b[2841] { 1.0 } else { 0.0 };

        if ((((s.v[2621] != 0.0) && s.b[2839]) && (!s.b[2840])) && s.b[2841]) {
            s.store_scalar(336, 0.0);
            s.store_scalar(343, 0.0);
        }

        if (((s.v[2621] != 0.0) && s.b[2839]) && (!s.b[2840])) {
            s.store_scaled_sqrt_ad(600, A::mul(s.ad_value(651), s.ad_value(336)), p.p432);
            s.store_sub_from_scalar(336, p.p334, 600);
        }

        if ((s.v[2621] != 0.0) && s.b[2839]) {
            s.store_sqrt_square_offset(782, 336, ((4.0 * (p.p334 * 0.01)) * (p.p334 * 0.01)));
            s.store_offset_scaled_div(343, 336, 782, 0.5, 0.5);
            s.store_scaled_add(336, 336, 782, 0.5);
        }

        s.b[2842] = (s.v[336] < 0.0);
        s.v[2842] = if s.b[2842] { 1.0 } else { 0.0 };

        if (((s.v[2621] != 0.0) && s.b[2839]) && s.b[2842]) {
            s.store_scalar(336, 0.0);
            s.store_scalar(343, 0.0);
        }

        if ((s.v[2621] != 0.0) && s.b[2839]) {
            s.copy_ad(386, 336);
            s.store_mul3_affine_lhs(418, 2758, 386, (0.5 * 9662367879.197212), 0.0, 386);
            s.store_sqrt_ad(334, A::mul_scaled_lhs(s.ad_value(154), 2.0, s.ad_value(418)));
            s.store_scaled_add_ad(335, A::limited_exp(s.ad_value(334)), A::limited_exp_scaled_input(s.ad_value(334), -1.0), 0.5);
            s.store_div_ad_lhs(419, A::ln(s.ad_value(335)), 418);
        }

        let (assign79430_e121215,) = {
    if ((s.v[2621] != 0.0) && s.b[2839]) {
        (1.0,)
    } else {
        (s.v[97],)
    }
};
        s.v[97] = assign79430_e121215;

    }
}
