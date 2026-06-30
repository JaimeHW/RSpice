#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, Scratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_transient_block_64(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((!s.b[1441]) && s.b[2428]) && s.b[2501]) {
            s.store_sqrt_add_scaled_square_input(334, 148, 1.0, 147, (4.0 * 0.002));
            s.store_add_scaled_inputs3_indices(149, 147, 1.0, 148, (-0.5), 334, (-0.5));
            s.store_mul_exp_ad_rhs(334, 140, A::mul(s.ad_value(154), s.ad_value(149)));
            s.store_add_offset_ad_lhs(335, A::mul(s.ad_value(154), A::sub(s.ad_value(149), s.ad_value(1436))), (-1.0), 334);
            s.store_sqrt_square_offset(782, 335, ((4.0 * 0.01) * 0.01));
            s.store_offset_scaled_div(334, 335, 782, 0.5, 0.5);
            s.store_scaled_add(335, 335, 782, 0.5);
        }

        s.b[2503] = (s.v[335] < 0.0);
        s.store_scalar(2503, if s.b[2503] { 1.0 } else { 0.0 });

        if ((((!s.b[1441]) && s.b[2428]) && s.b[2501]) && s.b[2503]) {
            s.store_scalar(335, 0.0);
            s.store_scalar(334, 0.0);
        }

        if (((!s.b[1441]) && s.b[2428]) && s.b[2501]) {
            s.store_offset(335, 335, 1e-25);
            s.store_sqrt(150, 335);
            s.store_offset_mul_ad(335, s.ad_value(154), A::sub(s.ad_value(149), s.ad_value(1436)), (-1.0));
            s.store_sqrt_square_offset(782, 335, ((4.0 * 0.01) * 0.01));
            s.store_offset_scaled_div(334, 335, 782, 0.5, 0.5);
            s.store_scaled_add(335, 335, 782, 0.5);
        }

        s.b[2504] = (s.v[335] < 0.0);
        s.store_scalar(2504, if s.b[2504] { 1.0 } else { 0.0 });

        if ((((!s.b[1441]) && s.b[2428]) && s.b[2501]) && s.b[2504]) {
            s.store_scalar(335, 0.0);
            s.store_scalar(334, 0.0);
        }

        if (((!s.b[1441]) && s.b[2428]) && s.b[2501]) {
            s.store_offset(335, 335, 1e-25);
            s.store_sqrt(151, 335);
            s.store_div_from_scalar(336, 0.5, 151);
            s.store_mul_sub_rhs(152, 139, 150, 151);
            s.store_sub(335, 146, 149);
            s.store_sqrt_square_offset(782, 335, ((4.0 * 0.1) * 0.1));
            s.store_offset_scaled_div(336, 335, 782, 0.5, 0.5);
            s.store_scaled_add(335, 335, 782, 0.5);
        }

        s.b[2505] = (s.v[335] < 0.0);
        s.store_scalar(2505, if s.b[2505] { 1.0 } else { 0.0 });

        if ((((!s.b[1441]) && s.b[2428]) && s.b[2501]) && s.b[2505]) {
            s.store_scalar(335, 0.0);
            s.store_scalar(336, 0.0);
        }

        if (((!s.b[1441]) && s.b[2428]) && s.b[2501]) {
            s.store_offset(335, 335, 1e-25);
            s.store_div(332, 790, 335);
            s.store_div_from_scalar_square_ad(336, 1.0, s.ad_value(335));
            s.store_square(722, 332);
            s.store_scalar(723, 1.0);
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
        }

        let (assign63860_e98761,) = {
    if (((!s.b[1441]) && s.b[2428]) && s.b[2501]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, assign63860_e98761);

        let (assign63870_e98770,) = {
    if (((!s.b[1441]) && s.b[2428]) && s.b[2501]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, assign63870_e98770);

        if (((!s.b[1441]) && s.b[2428]) && s.b[2501]) {
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

        s.b[2506] = ((((4.0 == 1.0) || (4.0 == 2.0)) || (4.0 == 4.0)) || (4.0 == 8.0));
        s.store_scalar(2506, if s.b[2506] { 1.0 } else { 0.0 });

        s.b[2507] = (4.0 == 1.0);
        s.store_scalar(2507, if s.b[2507] { 1.0 } else { 0.0 });

        let (assign64020_e98927,) = {
    if (((((!s.b[1441]) && s.b[2428]) && s.b[2501]) && s.b[2506]) && s.b[2507]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, assign64020_e98927);

        s.b[2508] = (4.0 == 2.0);
        s.store_scalar(2508, if s.b[2508] { 1.0 } else { 0.0 });

        let (assign64040_e98946,) = {
    if ((((((!s.b[1441]) && s.b[2428]) && s.b[2501]) && s.b[2506]) && (!s.b[2507])) && s.b[2508]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, assign64040_e98946);

        s.b[2509] = (4.0 == 4.0);
        s.store_scalar(2509, if s.b[2509] { 1.0 } else { 0.0 });

        let (assign64060_e98968,) = {
    if (((((((!s.b[1441]) && s.b[2428]) && s.b[2501]) && s.b[2506]) && (!s.b[2507])) && (!s.b[2508])) && s.b[2509]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, assign64060_e98968);

        s.b[2510] = (4.0 == 8.0);
        s.store_scalar(2510, if s.b[2510] { 1.0 } else { 0.0 });

        let (assign64080_e98993,) = {
    if ((((((((!s.b[1441]) && s.b[2428]) && s.b[2501]) && s.b[2506]) && (!s.b[2507])) && (!s.b[2508])) && (!s.b[2509])) && s.b[2510]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, assign64080_e98993);

        let (assign64090_e99004,) = {
    if ((((!s.b[1441]) && s.b[2428]) && s.b[2501]) && s.b[2506]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, assign64090_e99004);

        let mut assign64100_loop_guard: usize = 0;
        while {
            let assign64100_cond_e99016: f64 = if (((((!s.b[1441]) && s.b[2428]) && s.b[2501]) && s.b[2506]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign64100_cond_e99016 != 0.0
        } {
            assign64100_loop_guard += 1;
            assert!(assign64100_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((!s.b[1441]) && s.b[2428]) && s.b[2501]) && s.b[2506]) {
                s.store_sqrt(726, 726);
            }
            let (assign64100_body1_e99041,) = {
    if ((((!s.b[1441]) && s.b[2428]) && s.b[2501]) && s.b[2506]) {
        let assign64100_body1_e99039: f64 = (s.v[719] + 1.0);
        (assign64100_body1_e99039,)
    } else {
        (s.v[719],)
    }
};
            s.store_scalar(719, assign64100_body1_e99041);
        }

        if ((((!s.b[1441]) && s.b[2428]) && s.b[2501]) && (!s.b[2506])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 4.0)));
            }
        }

        if (((!s.b[1441]) && s.b[2428]) && s.b[2501]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(333, 332, 726, 1.0);
            s.store_div_scaled_product_indices(336, 725, 726, 1.0, 770, 1.0);
            s.store_scale(145, 155, ((2.0 * s.v[495]) * p.p7));
            s.copy_ad(335, 170);
            s.store_div_scaled_product_left_ad(153, A::mul3(s.ad_value(145), s.ad_value(253), s.ad_value(152)), 333, 1.0, 335, 1.0);
            s.store_add(134, 134, 153);
        }

        s.b[2511] = (((p.p31 != 0.0) && (p.p30 != 0.0)) && (s.v[963] == 0.0));
        s.store_scalar(2511, if s.b[2511] { 1.0 } else { 0.0 });

        if (((!s.b[1441]) && s.b[2428]) && s.b[2511]) {
            s.store_square(317, 127);
            s.store_mul3_affine_lhs(318, 155, 186, 2.0, 0.0, 248);
            s.store_sub(319, 317, 318);
            s.store_sqrt_square_offset(782, 317, ((4.0 * 0.001) * 0.001));
            s.store_offset_scaled_div(334, 317, 782, 0.5, 0.5);
            s.store_scaled_add(317, 317, 782, 0.5);
        }

        s.b[2512] = (s.v[317] < 0.0);
        s.store_scalar(2512, if s.b[2512] { 1.0 } else { 0.0 });

        if ((((!s.b[1441]) && s.b[2428]) && s.b[2511]) && s.b[2512]) {
            s.store_scalar(317, 0.0);
            s.store_scalar(334, 0.0);
        }

        if (((!s.b[1441]) && s.b[2428]) && s.b[2511]) {
            s.store_sqrt_square_offset(782, 319, ((4.0 * 0.001) * 0.001));
            s.store_offset_scaled_div(334, 319, 782, 0.5, 0.5);
            s.store_scaled_add(319, 319, 782, 0.5);
        }

        s.b[2513] = (s.v[319] < 0.0);
        s.store_scalar(2513, if s.b[2513] { 1.0 } else { 0.0 });

        if ((((!s.b[1441]) && s.b[2428]) && s.b[2511]) && s.b[2513]) {
            s.store_scalar(319, 0.0);
            s.store_scalar(334, 0.0);
        }

        if (((!s.b[1441]) && s.b[2428]) && s.b[2511]) {
            s.store_sub(320, 317, 319);
        }

        s.b[2514] = ((s.v[238] < (10.0 * 2.220446049250313e-16)) || (s.v[320] < (10.0 * 2.220446049250313e-16)));
        s.store_scalar(2514, if s.b[2514] { 1.0 } else { 0.0 });

        let (assign64370_e99378,) = {
    if ((((!s.b[1441]) && s.b[2428]) && s.b[2511]) && s.b[2514]) {
        (0.0,)
    } else {
        (s.v[321],)
    }
};
        s.store_scalar(321, assign64370_e99378);

        let (assign64380_e99390,) = {
    if ((((!s.b[1441]) && s.b[2428]) && s.b[2511]) && (!s.b[2514])) {
        (1.0,)
    } else {
        (s.v[321],)
    }
};
        s.store_scalar(321, assign64380_e99390);

        let (assign64390_e99397,) = {
    if ((!s.b[1441]) && (s.v[946] != 0.0)) {
        (0.0,)
    } else {
        (s.v[946],)
    }
};
        s.store_scalar(946, assign64390_e99397);

        s.b[2515] = ((s.v[78] == 0.0) && (s.v[127] > 1e-12));
        s.store_scalar(2515, if s.b[2515] { 1.0 } else { 0.0 });

        if ((!s.b[1441]) && s.b[2515]) {
            s.store_div_scaled_product_indices(130, 212, 154, 1.0, 100, 2.0);
            s.store_add_ad_lhs(128, A::div_scaled_value_offset_denominator(s.ad_value(127), 1.0, s.ad_value(130), 1.0, 1.0), 87);
        }

        if ((!s.b[1441]) && (!s.b[2515])) {
            s.store_scalar(128, 0.0);
        }

        if (!s.b[1441]) {
            s.copy_ad(136, 134);
            s.store_scalar(46, 0.0);
        }

        s.b[2517] = ((p.p450 > 0.0) && (p.p454 > 0.0));
        s.store_scalar(2517, if s.b[2517] { 1.0 } else { 0.0 });

        if ((!s.b[1441]) && s.b[2517]) {
            s.store_scalar(2522, 1e-5);
            s.store_offset_add_scaled_inputs3_offset_indices(2523, 791, 1.0, 120, 1.0, 182, -1.0, (-s.v[160]), (-p.p455));
        }

        let (assign64520_e99503,) = {
    if ((!s.b[1441]) && s.b[2517]) {
        let assign64520_e99501: f64 = (s.v[118] + p.p455);
        (assign64520_e99501,)
    } else {
        (s.v[2524],)
    }
};
        s.store_scalar(2524, assign64520_e99503);

        if ((!s.b[1441]) && s.b[2517]) {
            s.store_sqrt_offset_ad(781, A::square(A::sub(s.ad_value(960), s.ad_value(1433))), ((4.0 * 0.01) * 0.01));
            s.store_add_scaled_inputs3_indices(2534, 960, 0.5, 1433, ((-1.0) * 0.5), 781, 0.5);
            s.store_sqrt_ad(2518, A::div_scaled_product_offset_denominator(s.ad_value(2534), s.ad_value(586), (((2.0 * 1.6021918e-19) * 1.0 / (1.034943e-10)) * s.v[489]), s.ad_value(586), s.v[489], 1.0));
            s.store_mul(2520, 2518, 162);
            s.store_div_scaled_product_add_scaled_denominator_indices(993, 2520, 2520, (-0.25), 790, 1.0, 2520, 1.0, 1.0);
        }

        s.b[2536] = (p.p457 > 0.0);
        s.store_scalar(2536, if s.b[2536] { 1.0 } else { 0.0 });

        if (((!s.b[1441]) && s.b[2517]) && s.b[2536]) {
            s.store_scalar(2521, p.p457);
        }

        if (((!s.b[1441]) && s.b[2517]) && (!s.b[2536])) {
            s.copy_ad(2537, 993);
        }

        let (assign64610_e99615,) = {
    if (((!s.b[1441]) && s.b[2517]) && (!s.b[2536])) {
        (s.v[2524],)
    } else {
        (s.v[2538],)
    }
};
        s.store_scalar(2538, assign64610_e99615);

    }

    pub(super) fn stamp_transient_block_65(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((!s.b[1441]) && s.b[2517]) && (!s.b[2536])) {
            s.store_offset_div_scaled_offset_numerator(332, A::mul(s.ad_value(154), A::sub(s.ad_value(2523), s.ad_value(2537))), 4.0, ((-1.0) * 4.0), A::mul(s.ad_value(213), s.ad_value(156)), 1.0, 1.0);
        }

        if (((!s.b[1441]) && s.b[2517]) && (!s.b[2536])) {
            if (s.v[332] >= (10.0 * 2.220446049250313e-16)) {
            } else {
                s.store_scalar(332, (10.0 * 2.220446049250313e-16));
            }
        }

        if (((!s.b[1441]) && s.b[2517]) && (!s.b[2536])) {
            s.store_add_product3_rhs_mixed_iia(89, 2523, 213, 154, A::sub_from_scalar(1.0, A::sqrt(s.ad_value(332))), 0.5);
            s.store_mul_sub_rhs(116, 154, 89, 2537);
        }

        s.b[2539] = (s.v[116] < 3.0);
        s.store_scalar(2539, if s.b[2539] { 1.0 } else { 0.0 });

        if ((((!s.b[1441]) && s.b[2517]) && (!s.b[2536])) && s.b[2539]) {
            s.store_mul_sub_rhs(333, 154, 2523, 2537);
            s.store_div_from_scalar_scaled_mul(335, 1.0, 154, 212, (1.414213562373095 / 108.0));
            s.store_offset_scaled(336, 335, 3.0, 81.0);
            s.store_add_scaled_sub_value_product_mixed_aii(337, (-2916.0), A::scale(s.ad_value(335), 81.0), 1.0, 335, 333, 27.0);
            s.store_add_scaled_sub_value_product_mixed_aii(338, 1458.0, A::scaled_offset(s.ad_value(335), 54.0, 81.0), 1.0, 335, 333, 27.0);
            s.store_square(338, 338);
        }

        if ((((!s.b[1441]) && s.b[2517]) && (!s.b[2536])) && s.b[2539]) {
            if ((s.v[337] + ((((((4.0 * s.v[336]) * s.v[336]) * s.v[336]) + s.v[338])) as f64).sqrt()) == 0.0) {
                s.store_scalar(339, 0.0);
            } else {
                s.store_powf_ad(339, A::add(s.ad_value(337), A::sqrt(A::add(A::mul3_scaled_output(s.ad_value(336), s.ad_value(336), s.ad_value(336), 4.0), s.ad_value(338)))), 0.3333333333333333);
            }
        }

        if ((((!s.b[1441]) && s.b[2517]) && (!s.b[2536])) && s.b[2539]) {
            s.store_add_scaled_ad_lhs(332, A::sub_from_scalar(3.0, A::div_scaled_inputs(s.ad_value(336), 1.259921049894873, s.ad_value(339), 3.0)), 339, (1.0 / (3.0 * 1.259921049894873)));
            s.store_add_scaled_product_indices(89, 2537, 1.0, 332, 155, 1.0);
            s.copy_ad(88, 89);
        }

        s.b[2540] = (s.v[791] <= s.v[2538]);
        s.store_scalar(2540, if s.b[2540] { 1.0 } else { 0.0 });

        if (((((!s.b[1441]) && s.b[2517]) && (!s.b[2536])) && (!s.b[2539])) && s.b[2540]) {
            s.copy_ad(88, 89);
        }

        if (((((!s.b[1441]) && s.b[2517]) && (!s.b[2536])) && (!s.b[2539])) && (!s.b[2540])) {
            s.store_div_scalar_by_product(335, 1.0, s.ad_value(210), s.ad_value(211), 1.0);
            s.store_mul3_lhs(336, 335, 2523, 2523);
            s.store_add_div_from_scalar_rhs(337, 154, 2.0, 2523);
            s.store_offset_div_ad(90, A::ln(s.ad_value(336)), s.ad_value(337), p.p456);
            s.store_offset_sub(781, 90, 89, (-0.0008));
            s.store_scale(782, 90, (4.0 * 0.0008));
        }

        if (((((!s.b[1441]) && s.b[2517]) && (!s.b[2536])) && (!s.b[2539])) && (!s.b[2540])) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if (((((!s.b[1441]) && s.b[2517]) && (!s.b[2536])) && (!s.b[2539])) && (!s.b[2540])) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_add_scaled_inputs3_indices(88, 90, 1.0, 781, (-0.5), 782, (-0.5));
        }

        if (((!s.b[1441]) && s.b[2517]) && (!s.b[2536])) {
            s.store_offset(332, 2537, (1e-12 / 2.0));
        }

        s.b[2541] = (s.v[88] < s.v[332]);
        s.store_scalar(2541, if s.b[2541] { 1.0 } else { 0.0 });

        if ((((!s.b[1441]) && s.b[2517]) && (!s.b[2536])) && s.b[2541]) {
            s.copy_ad(88, 332);
        }

        if (((!s.b[1441]) && s.b[2517]) && (!s.b[2536])) {
            s.copy_ad(2521, 88);
        }

        s.b[2542] = (p.p451 == 1.0);
        s.store_scalar(2542, if s.b[2542] { 1.0 } else { 0.0 });

        if ((((!s.b[1441]) && s.b[2517]) && (!s.b[2536])) && s.b[2542]) {
            s.copy_ad(88, 2521);
            s.copy_ad(2543, 993);
        }

        let (assign64950_e100196,) = {
    if ((((!s.b[1441]) && s.b[2517]) && (!s.b[2536])) && s.b[2542]) {
        let assign64950_e100188: f64 = (s.v[160] - s.v[120]);
        let assign64950_e100190: f64 = (assign64950_e100188 + s.v[182]);
        let assign64950_e100192: f64 = (assign64950_e100190 + s.v[2543]);
        let assign64950_e100194: f64 = (assign64950_e100192 + p.p455);
        (assign64950_e100194,)
    } else {
        (s.v[86],)
    }
};
        s.store_scalar(86, assign64950_e100196);

        s.b[2552] = (s.v[791] < s.v[86]);
        s.store_scalar(2552, if s.b[2552] { 1.0 } else { 0.0 });

        let (assign64970_e100214,) = {
    if (((((!s.b[1441]) && s.b[2517]) && (!s.b[2536])) && s.b[2542]) && s.b[2552]) {
        let assign64970_e100212: f64 = (-1.0);
        (assign64970_e100212,)
    } else {
        (s.v[347],)
    }
};
        s.store_scalar(347, assign64970_e100214);

        if (((((!s.b[1441]) && s.b[2517]) && (!s.b[2536])) && s.b[2542]) && s.b[2552]) {
            s.store_mul_scaled_ln_ad_rhs(271, 155, 2.0, A::div_from_scalar((-s.v[270]), s.ad_value(212)));
            s.store_mul_sub_rhs(332, 154, 2523, 2543);
            s.store_div_from_scalar_mul_ad(335, 1.0, s.ad_value(154), s.ad_value(209));
            s.store_mul(333, 335, 185);
            s.store_offset_scaled(279, 333, (3.0 * 1.414213562373095), 2.0);
            s.store_mul3_affine_lhs(277, 279, 279, 8.0, 0.0, 279);
            s.store_offset(338, 332, (-2.0));
            s.store_scaled_mul(339, 333, 338, 9.0);
            s.store_sub_from_scalar(278, (7.0 * 1.414213562373095), 339);
            s.store_square(276, 278);
        }

        s.b[2553] = (s.v[277] < (s.v[276] * 1e-8));
        s.store_scalar(2553, if s.b[2553] { 1.0 } else { 0.0 });

        if ((((((!s.b[1441]) && s.b[2517]) && (!s.b[2536])) && s.b[2542]) && s.b[2552]) && s.b[2553]) {
            s.store_add_scaled_inputs3_offset_mixed_iai(274, 278, 1.0, A::div_scaled_inputs(s.ad_value(277), 0.5, s.ad_value(278), 1.0), 1.0, 339, 1.0, ((-7.0) * 1.414213562373095));
        }

        if ((((((!s.b[1441]) && s.b[2517]) && (!s.b[2536])) && s.b[2542]) && s.b[2552]) && (!s.b[2553])) {
            s.store_sqrt_add(275, 277, 276);
            s.store_add_offset_lhs(274, 275, ((-7.0) * 1.414213562373095), 339);
        }

        if (((((!s.b[1441]) && s.b[2517]) && (!s.b[2536])) && s.b[2542]) && s.b[2552]) {
            if (s.v[274] == 0.0) {
                s.store_scalar(273, 0.0);
            } else {
                s.store_powf(273, 274, 0.3333333333333333);
            }
        }

        if (((((!s.b[1441]) && s.b[2517]) && (!s.b[2536])) && s.b[2542]) && s.b[2552]) {
            s.store_add_scaled_inputs_product_first_ad(272, A::sub_from_scalar(((-4.0) * 1.414213562373095), A::scale(s.ad_value(333), 12.0)), 1.0, 273, 2.0, 273, 273, 1.414213562373095);
            s.store_div_from_scalar(335, 1.0, 273);
            s.store_mul(116, 272, 335);
            s.store_add_scaled_product_indices(167, 2543, 1.0, 116, 155, 1.0);
            s.store_sub(335, 167, 2543);
            s.store_div(336, 335, 271);
            s.store_sqrt_square_offset(337, 336, 1.0);
            s.store_add_div_lhs_indices(2521, 335, 337, 2543);
        }

        if (((((!s.b[1441]) && s.b[2517]) && (!s.b[2536])) && s.b[2542]) && (!s.b[2552])) {
            s.store_exp_ad(230, A::mul_offset_rhs(s.ad_value(154), s.ad_value(2543), (-p.p456)));
        }

        let (assign65220_e100680,) = {
    if (((((!s.b[1441]) && s.b[2517]) && (!s.b[2536])) && s.b[2542]) && (!s.b[2552])) {
        (0.0,)
    } else {
        (s.v[79],)
    }
};
        s.store_scalar(79, assign65220_e100680);

        if (((((!s.b[1441]) && s.b[2517]) && (!s.b[2536])) && s.b[2542]) && (!s.b[2552])) {
            s.copy_ad(2544, 88);
            s.store_mul3_affine_lhs(2545, 166, 2522, (0.5 * 9662367879.197212), 0.0, 2522);
            s.store_sqrt_mul_scaled_lhs(334, 154, 2.0, 2545);
            s.store_scaled_add_ad(335, A::limited_exp(s.ad_value(334)), A::limited_exp_scaled_input(s.ad_value(334), -1.0), 0.5);
            s.store_div_ln_lhs(2546, 335, 2545);
        }

        let (assign65280_e100793,) = {
    if (((((!s.b[1441]) && s.b[2517]) && (!s.b[2536])) && s.b[2542]) && (!s.b[2552])) {
        (1.0,)
    } else {
        (s.v[97],)
    }
};
        s.store_scalar(97, assign65280_e100793);

        let mut assign65290_loop_guard: usize = 0;
        while {
            let assign65290_cond_e100809: f64 = (s.v[421] + 1.0);
            let assign65290_cond_e100811: f64 = if ((((((!s.b[1441]) && s.b[2517]) && (!s.b[2536])) && s.b[2542]) && (!s.b[2552])) && (s.v[97] <= assign65290_cond_e100809)) { 1.0 } else { 0.0 };
            assign65290_cond_e100811 != 0.0
        } {
            assign65290_loop_guard += 1;
            assert!(assign65290_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((((!s.b[1441]) && s.b[2517]) && (!s.b[2536])) && s.b[2542]) && (!s.b[2552])) {
                s.store_sub(2547, 2544, 2543);
                s.store_mul(116, 154, 2547);
                s.store_mul_sub_rhs(333, 2546, 2547, 2545);
            }
            s.b[2554] = (s.v[333] < 60.0);
            s.store_scalar(2554, if s.b[2554] { 1.0 } else { 0.0 });
            if ((((((!s.b[1441]) && s.b[2517]) && (!s.b[2536])) && s.b[2542]) && (!s.b[2552])) && s.b[2554]) {
                s.store_exp(335, 333);
                s.store_exp_mul_scaled_lhs_indices(334, 2546, -1.0, 2545);
                s.store_sub(336, 335, 334);
                s.store_div_ln_offset_lhs(2549, 336, 1.0, 2546);
                s.store_div_scaled_value_offset_denominator(2550, s.ad_value(335), 1.0, s.ad_value(336), 1.0, 1.0);
            }
            if ((((((!s.b[1441]) && s.b[2517]) && (!s.b[2536])) && s.b[2542]) && (!s.b[2552])) && (!s.b[2554])) {
                s.store_sub(2549, 2547, 2545);
                s.store_scalar(2550, 1.0);
            }
            if (((((!s.b[1441]) && s.b[2517]) && (!s.b[2536])) && s.b[2542]) && (!s.b[2552])) {
                s.store_mul(2548, 154, 2549);
            }
            s.b[2555] = (((s.v[116]) as f64).abs() < 1e-16);
            s.store_scalar(2555, if s.b[2555] { 1.0 } else { 0.0 });
            if ((((((!s.b[1441]) && s.b[2517]) && (!s.b[2536])) && s.b[2542]) && (!s.b[2552])) && s.b[2555]) {
                s.store_sqrt_scaled_input_ad(334, A::sub_from_scalar(1.0, A::square(s.ad_value(2550))), 1.0 / (2.0));
                s.store_mul(223, 116, 334);
                s.store_mul(2551, 154, 334);
            }
            s.b[2556] = (s.v[116] < 0.0);
            s.store_scalar(2556, if s.b[2556] { 1.0 } else { 0.0 });
            if (((((((!s.b[1441]) && s.b[2517]) && (!s.b[2536])) && s.b[2542]) && (!s.b[2552])) && s.b[2555]) && s.b[2556]) {
                s.store_neg(223, 223);
                s.store_neg(2551, 2551);
            }
            s.b[2557] = (((s.v[116]) as f64).abs() < 0.005);
            s.store_scalar(2557, if s.b[2557] { 1.0 } else { 0.0 });
            if (((((((!s.b[1441]) && s.b[2517]) && (!s.b[2536])) && s.b[2542]) && (!s.b[2552])) && (!s.b[2555])) && s.b[2557]) {
                s.store_square_mul_sub_from_scalar_double_scaled_sub_rhs_scaled_output(334, 116, 1.0, 116, 1.0, 116, 1.0, 116, 0.2, 1.0 / (4.0), 1.0 / (3.0), 1.0 / (2.0));
                s.store_mul_sub_from_scalar_ad_rhs(335, 116, 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(116), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(116), 1.0, A::scale(s.ad_value(116), 0.25), 1.0 / (3.0)), 1.0 / (2.0)));
                s.store_square_mul_sub_from_scalar_double_scaled_sub_rhs_scaled_output(336, 2548, 1.0, 2548, 1.0, 2548, 1.0, 2548, 0.2, 1.0 / (4.0), 1.0 / (3.0), 1.0 / (2.0));
                s.store_mul_sub_from_scalar_ad_rhs(337, 2548, 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(2548), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(2548), 1.0, A::scale(s.ad_value(2548), 0.25), 1.0 / (3.0)), 1.0 / (2.0)));
                s.store_sqrt_sub(223, 334, 336);
                s.store_div_scaled_product_right_ad(2551, 154, A::add_scaled_product(s.ad_value(335), 1.0, s.ad_value(2550), s.ad_value(337), (-1.0)), 0.5, 223, 1.0);
            }
            if (((((((!s.b[1441]) && s.b[2517]) && (!s.b[2536])) && s.b[2542]) && (!s.b[2552])) && (!s.b[2555])) && (!s.b[2557])) {
                s.store_exp_neg_input(334, 116);
                s.store_exp_neg_input(335, 2548);
                s.store_sqrt_ad(223, A::add_scaled_inputs4(s.ad_value(116), 1.0, s.ad_value(2548), (-1.0), s.ad_value(334), 1.0, s.ad_value(335), (-1.0)));
                s.store_div_scaled_product_right_ad(2551, 154, A::sub(A::sub_from_scalar(1.0, s.ad_value(334)), A::mul_sub_from_scalar_rhs(s.ad_value(2550), 1.0, s.ad_value(335))), 0.5, 223, 1.0);
            }
            s.b[2558] = ((s.v[79] == 1.0) && (s.v[116] < 0.0));
            s.store_scalar(2558, if s.b[2558] { 1.0 } else { 0.0 });
            let (assign65290_body31_e101483,) = {
    if ((((((!s.b[1441]) && s.b[2517]) && (!s.b[2536])) && s.b[2542]) && (!s.b[2552])) && s.b[2558]) {
        let assign65290_body31_e101481: f64 = (-1.0);
        (assign65290_body31_e101481,)
    } else {
        (s.v[347],)
    }
};
            s.store_scalar(347, assign65290_body31_e101483);
            s.b[2559] = (s.v[116] < 0.0);
            s.store_scalar(2559, if s.b[2559] { 1.0 } else { 0.0 });
            if ((((((!s.b[1441]) && s.b[2517]) && (!s.b[2536])) && s.b[2542]) && (!s.b[2552])) && s.b[2559]) {
                s.store_neg(216, 223);
                s.store_neg(217, 2551);
            }
            s.b[2560] = (s.v[116] < 1e-7);
            s.store_scalar(2560, if s.b[2560] { 1.0 } else { 0.0 });
            if (((((((!s.b[1441]) && s.b[2517]) && (!s.b[2536])) && s.b[2542]) && (!s.b[2552])) && (!s.b[2559])) && s.b[2560]) {
                s.copy_ad(216, 223);
                s.copy_ad(217, 2551);
            }
            if (((((((!s.b[1441]) && s.b[2517]) && (!s.b[2536])) && s.b[2542]) && (!s.b[2552])) && (!s.b[2559])) && (!s.b[2560])) {
                s.store_mul_offset_rhs(117, 154, 2544, (-p.p456));
                s.store_exp(228, 117);
                s.store_mul_ad_rhs(214, 210, A::add_scaled_offset_product_rhs(s.ad_value(228), 1.0, s.ad_value(230), s.ad_value(116), 1.0, (-1.0)));
                s.store_mul_ad_product_rhs_mixed_ia(215, 210, 154, A::sub(s.ad_value(228), s.ad_value(230)));
                s.store_sqrt_square_add(216, 223, 214);
                s.store_div_scaled_add_product(217, s.ad_value(215), 0.5, s.ad_value(2551), s.ad_value(223), (2.0 * 0.5), s.ad_value(216), 1.0);
            }
            if (((((!s.b[1441]) && s.b[2517]) && (!s.b[2536])) && s.b[2542]) && (!s.b[2552])) {
                s.store_add_scaled_inputs_product_indices(232, 2544, 1.0, 2523, (-1.0), 212, 216, 1.0);
                s.store_offset_mul(233, 212, 217, 1.0);
            }
            s.b[2561] = (s.v[79] == 1.0);
            s.store_scalar(2561, if s.b[2561] { 1.0 } else { 0.0 });
            let (assign65290_body47_e101788,) = {
    if ((((((!s.b[1441]) && s.b[2517]) && (!s.b[2536])) && s.b[2542]) && (!s.b[2552])) && s.b[2561]) {
        let assign65290_body47_e101786: f64 = (s.v[421] + 1.0);
        (assign65290_body47_e101786,)
    } else {
        (s.v[97],)
    }
};
            s.store_scalar(97, assign65290_body47_e101788);
            if ((((((!s.b[1441]) && s.b[2517]) && (!s.b[2536])) && s.b[2542]) && (!s.b[2552])) && (!s.b[2561])) {
                s.store_div_scaled_inputs_indices(236, 232, -1.0, 233, 1.0);
            }
            if ((((((!s.b[1441]) && s.b[2517]) && (!s.b[2536])) && s.b[2542]) && (!s.b[2552])) && (!s.b[2561])) {
                s.store_scaled_offset_ad(93, {
                    if (1.0 >= ((s.v[2544]) as f64).abs()) {
                        A::constant(1.0)
                    } else {
                        A::abs(s.ad_value(2544))
                    }
                }, 1.0, (0.5 * 0.1));
            }
            s.b[2562] = (((s.v[236]) as f64).abs() > s.v[93]);
            s.store_scalar(2562, if s.b[2562] { 1.0 } else { 0.0 });
            if (((((((!s.b[1441]) && s.b[2517]) && (!s.b[2536])) && s.b[2542]) && (!s.b[2552])) && (!s.b[2561])) && s.b[2562]) {
                s.store_scale(236, 93, (if (s.v[236] >= 0.0) { 1.0 } else { (-1.0) }));
            }
            if ((((((!s.b[1441]) && s.b[2517]) && (!s.b[2536])) && s.b[2542]) && (!s.b[2552])) && (!s.b[2561])) {
                s.store_add(2544, 2544, 236);
            }
            s.b[2563] = ((((s.v[236]) as f64).abs() <= 1e-12) && (((s.v[232]) as f64).abs() <= 1e-8));
            s.store_scalar(2563, if s.b[2563] { 1.0 } else { 0.0 });
            let (assign65290_body54_e101921,) = {
    if (((((((!s.b[1441]) && s.b[2517]) && (!s.b[2536])) && s.b[2542]) && (!s.b[2552])) && (!s.b[2561])) && s.b[2563]) {
        (1.0,)
    } else {
        (s.v[79],)
    }
};
            s.store_scalar(79, assign65290_body54_e101921);
            let (assign65290_body55_e101938,) = {
    if (((((!s.b[1441]) && s.b[2517]) && (!s.b[2536])) && s.b[2542]) && (!s.b[2552])) {
        let assign65290_body55_e101936: f64 = (s.v[97] + 1.0);
        (assign65290_body55_e101936,)
    } else {
        (s.v[97],)
    }
};
            s.store_scalar(97, assign65290_body55_e101938);
        }

        if (((((!s.b[1441]) && s.b[2517]) && (!s.b[2536])) && s.b[2542]) && (!s.b[2552])) {
            s.copy_ad(2521, 2544);
        }

    }

    pub(super) fn stamp_transient_block_66(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((!s.b[1441]) && s.b[2517]) {
            s.store_mul_sub_scaled_inputs_rhs(339, 154, s.ad_value(2521), -1.0, s.ad_value(993), -1.0);
            s.store_abs(2533, 339);
            s.store_exp(340, 339);
            s.store_sub_offset_lhs(341, 340, (-1.0), 339);
        }

        s.b[2564] = (s.v[339] > 1e-7);
        s.store_scalar(2564, if s.b[2564] { 1.0 } else { 0.0 });

        if (((!s.b[1441]) && s.b[2517]) && s.b[2564]) {
            s.store_mul_scaled_sqrt_rhs(2535, 209, -1.0, 341);
        }

        s.b[2565] = (s.v[2533] > 1e-7);
        s.store_scalar(2565, if s.b[2565] { 1.0 } else { 0.0 });

        if ((((!s.b[1441]) && s.b[2517]) && (!s.b[2564])) && s.b[2565]) {
            s.store_mul_sqrt_rhs(2535, 209, 341);
        }

        if ((((!s.b[1441]) && s.b[2517]) && (!s.b[2564])) && (!s.b[2565])) {
            s.store_mul_scaled_sqrt_ad_rhs(2535, 339, (-0.7071067811865475), A::offset(A::mul_scaled_lhs(s.ad_value(2533), 0.3333333333333333, A::scale_offset(s.ad_value(2533), 0.25, 1.0)), 1.0));
        }

        if ((!s.b[1441]) && s.b[2517]) {
            s.store_sqrt_square_offset(781, 2535, ((4.0 * 1e-6) * 1e-6));
            s.store_scaled_add(2530, 2535, 781, 0.5);
            s.store_div_scaled_inputs_indices(2531, 2530, 1.0, 586, 1.6021918e-19);
            s.store_offset(335, 2531, (-p.p452));
            s.store_scale(2532, 2531, 0.01);
            s.store_sqrt_add_scaled_square_product(781, 335, 1.0, 2532, 2532, 4.0);
            s.store_scaled_add(336, 335, 781, 0.5);
            s.store_div_scaled_product_by_product(2529, s.ad_value(336), s.ad_value(336), 1.0, s.ad_value(2531), s.ad_value(2531), 1.0);
            s.store_add_scaled_product_left_ad(994, 993, 1.0, A::sub(s.ad_value(2521), s.ad_value(993)), 2529, 1.0);
            s.store_mul_sub_from_scalar_rhs_ad(333, A::exp(A::mul(s.ad_value(154), A::add_scaled_inputs3(s.ad_value(994), 1.0, s.ad_value(960), -1.0, s.ad_value(1433), 1.0))), 1.0, A::exp(A::mul_scaled_lhs(s.ad_value(154), -1.0, s.ad_value(790))));
            s.store_scalar(2525, (((((2.0 * 1.6021918e-19) * s.v[489]) * 1.034943e-10)) as f64).sqrt());
            s.store_mul_sqrt_rhs(2526, 2525, 155);
            s.store_mul_sub_rhs(2519, 154, 994, 993);
        }

        s.b[2566] = ((s.v[2519] < (0.2 * s.v[154])) && ((0.2 * s.v[154]) >= 0.0));
        s.store_scalar(2566, if s.b[2566] { 1.0 } else { 0.0 });

        if (((!s.b[1441]) && s.b[2517]) && s.b[2566]) {
            s.store_sub_scaled_inputs(781, 154, 0.2, 2519, 1.0);
            s.store_square(722, 781);
            s.store_scaled_mul(723, 154, 154, (0.2 * 0.2));
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
        }

        let (assign65590_e102302,) = {
    if (((!s.b[1441]) && s.b[2517]) && s.b[2566]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, assign65590_e102302);

        let (assign65600_e102311,) = {
    if (((!s.b[1441]) && s.b[2517]) && s.b[2566]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, assign65600_e102311);

        if (((!s.b[1441]) && s.b[2517]) && s.b[2566]) {
            s.store_scalar(770, 0.0);
            s.store_scalar(726, 0.0);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_add(770, 724, 725);
            s.copy_ad(726, 770);
        }

        s.b[2567] = ((((1.0 == 1.0) || (1.0 == 2.0)) || (1.0 == 4.0)) || (1.0 == 8.0));
        s.store_scalar(2567, if s.b[2567] { 1.0 } else { 0.0 });

        s.b[2568] = (1.0 == 1.0);
        s.store_scalar(2568, if s.b[2568] { 1.0 } else { 0.0 });

        let (assign65690_e102402,) = {
    if (((((!s.b[1441]) && s.b[2517]) && s.b[2566]) && s.b[2567]) && s.b[2568]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, assign65690_e102402);

        s.b[2569] = (1.0 == 2.0);
        s.store_scalar(2569, if s.b[2569] { 1.0 } else { 0.0 });

        let (assign65710_e102421,) = {
    if ((((((!s.b[1441]) && s.b[2517]) && s.b[2566]) && s.b[2567]) && (!s.b[2568])) && s.b[2569]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, assign65710_e102421);

        s.b[2570] = (1.0 == 4.0);
        s.store_scalar(2570, if s.b[2570] { 1.0 } else { 0.0 });

        let (assign65730_e102443,) = {
    if (((((((!s.b[1441]) && s.b[2517]) && s.b[2566]) && s.b[2567]) && (!s.b[2568])) && (!s.b[2569])) && s.b[2570]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, assign65730_e102443);

        s.b[2571] = (1.0 == 8.0);
        s.store_scalar(2571, if s.b[2571] { 1.0 } else { 0.0 });

        let (assign65750_e102468,) = {
    if ((((((((!s.b[1441]) && s.b[2517]) && s.b[2566]) && s.b[2567]) && (!s.b[2568])) && (!s.b[2569])) && (!s.b[2570])) && s.b[2571]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, assign65750_e102468);

        let (assign65760_e102479,) = {
    if ((((!s.b[1441]) && s.b[2517]) && s.b[2566]) && s.b[2567]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, assign65760_e102479);

        let mut assign65770_loop_guard: usize = 0;
        while {
            let assign65770_cond_e102491: f64 = if (((((!s.b[1441]) && s.b[2517]) && s.b[2566]) && s.b[2567]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign65770_cond_e102491 != 0.0
        } {
            assign65770_loop_guard += 1;
            assert!(assign65770_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((!s.b[1441]) && s.b[2517]) && s.b[2566]) && s.b[2567]) {
                s.store_sqrt(726, 726);
            }
            let (assign65770_body1_e102516,) = {
    if ((((!s.b[1441]) && s.b[2517]) && s.b[2566]) && s.b[2567]) {
        let assign65770_body1_e102514: f64 = (s.v[719] + 1.0);
        (assign65770_body1_e102514,)
    } else {
        (s.v[719],)
    }
};
            s.store_scalar(719, assign65770_body1_e102516);
        }

        if ((((!s.b[1441]) && s.b[2517]) && s.b[2566]) && (!s.b[2567])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / 2.0));
            }
        }

        if (((!s.b[1441]) && s.b[2517]) && s.b[2566]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_mul3_affine_lhs(780, 781, 154, 0.2, 0.0, 726);
            s.store_div_scaled_product3_indices(334, 154, 725, 726, 0.2, 770, 1.0);
            s.store_sub_scaled_inputs(335, 154, 0.2, 780, 1.0);
        }

        if (((!s.b[1441]) && s.b[2517]) && s.b[2566]) {
        }

        if (((!s.b[1441]) && s.b[2517]) && (!s.b[2566])) {
            s.copy_ad(335, 2519);
            s.store_scalar(334, 1.0);
        }

        if ((!s.b[1441]) && s.b[2517]) {
            s.store_sqrt_offset_input(2527, 335, (10.0 * 2.220446049250313e-16));
            s.store_mul(2528, 2526, 2527);
            s.store_mul_scaled_ad_lhs(995, A::div_scaled_inputs(s.ad_value(155), 2.0, s.ad_value(162), 1.0), 2528, p.p454);
            s.store_scaled_mul(46, 995, 333, s.v[632]);
            s.store_add(134, 136, 46);
        }

        if (!s.b[1441]) {
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

        s.store_div_scaled_inputs_square_rhs(336, 783, (-2.0), 782, 1.0);

        s.b[2572] = ((s.v[110] < ((10.0 * 2.220446049250313e-16) + (10.0 * 2.220446049250313e-16))) && ((10.0 * 2.220446049250313e-16) >= 0.0));
        s.store_scalar(2572, if s.b[2572] { 1.0 } else { 0.0 });

        if s.b[2572] {
            s.store_sub_from_scalar(781, ((10.0 * 2.220446049250313e-16) + (10.0 * 2.220446049250313e-16)), 110);
            s.store_square(722, 781);
            s.store_scalar(723, ((10.0 * 2.220446049250313e-16) * (10.0 * 2.220446049250313e-16)));
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
        }

        let (assign66110_e102863,) = {
    if s.b[2572] {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, assign66110_e102863);

        let (assign66120_e102867,) = {
    if s.b[2572] {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, assign66120_e102867);

        if s.b[2572] {
            s.store_scalar(770, 0.0);
            s.store_scalar(726, 0.0);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_add(770, 724, 725);
            s.copy_ad(726, 770);
        }

        s.b[2573] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.store_scalar(2573, if s.b[2573] { 1.0 } else { 0.0 });

        s.b[2574] = (2.0 == 1.0);
        s.store_scalar(2574, if s.b[2574] { 1.0 } else { 0.0 });

        let (assign66230_e102935,) = {
    if ((s.b[2572] && s.b[2573]) && s.b[2574]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, assign66230_e102935);

        s.b[2575] = (2.0 == 2.0);
        s.store_scalar(2575, if s.b[2575] { 1.0 } else { 0.0 });

        let (assign66250_e102949,) = {
    if (((s.b[2572] && s.b[2573]) && (!s.b[2574])) && s.b[2575]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, assign66250_e102949);

        s.b[2576] = (2.0 == 4.0);
        s.store_scalar(2576, if s.b[2576] { 1.0 } else { 0.0 });

        let (assign66270_e102966,) = {
    if ((((s.b[2572] && s.b[2573]) && (!s.b[2574])) && (!s.b[2575])) && s.b[2576]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, assign66270_e102966);

        s.b[2577] = (2.0 == 8.0);
        s.store_scalar(2577, if s.b[2577] { 1.0 } else { 0.0 });

        let (assign66290_e102986,) = {
    if (((((s.b[2572] && s.b[2573]) && (!s.b[2574])) && (!s.b[2575])) && (!s.b[2576])) && s.b[2577]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, assign66290_e102986);

        let (assign66300_e102992,) = {
    if (s.b[2572] && s.b[2573]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, assign66300_e102992);

        let mut assign66310_loop_guard: usize = 0;
        while {
            let assign66310_cond_e102999: f64 = if ((s.b[2572] && s.b[2573]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign66310_cond_e102999 != 0.0
        } {
            assign66310_loop_guard += 1;
            assert!(assign66310_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (s.b[2572] && s.b[2573]) {
                s.store_sqrt(726, 726);
            }
            let (assign66310_body1_e103014,) = {
    if (s.b[2572] && s.b[2573]) {
        let assign66310_body1_e103012: f64 = (s.v[719] + 1.0);
        (assign66310_body1_e103012,)
    } else {
        (s.v[719],)
    }
};
            s.store_scalar(719, assign66310_body1_e103014);
        }

        if (s.b[2572] && (!s.b[2573])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }

        if s.b[2572] {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, (10.0 * 2.220446049250313e-16));
            s.store_div_scaled_product_indices(334, 725, 726, (10.0 * 2.220446049250313e-16), 770, 1.0);
            s.store_sub_from_scalar(110, ((10.0 * 2.220446049250313e-16) + (10.0 * 2.220446049250313e-16)), 780);
        }

        if s.b[2572] {
        }

        if (!s.b[2572]) {
        }

        if (!s.b[2572]) {
            s.store_scalar(334, 1.0);
        }

        s.store_add(109, 87, 110);

    }

    pub(super) fn stamp_transient_block_67(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.store_add_scaled_product_left_ad(134, 134, 1.0, A::div_from_scalar(s.v[163], s.ad_value(162)), 790, p.p435);

        s.b[2578] = (p.p23 == 0.0);
        s.store_scalar(2578, if s.b[2578] { 1.0 } else { 0.0 });

        if s.b[2578] {
            s.store_scalar(280, 0.0);
            s.store_scalar(288, 0.0);
        }

        s.b[2579] = ((s.v[481] > 0.0) && (s.v[454] > 0.0));
        s.store_scalar(2579, if s.b[2579] { 1.0 } else { 0.0 });

        if ((!s.b[2578]) && s.b[2579]) {
            s.store_mul(335, 659, 85);
            s.store_scale(337, 636, 1.0 / ((s.v[188] * s.v[188])));
            s.store_scale_ad(338, A::div_from_scalar(2.0, s.ad_value(636)), (s.v[188] * s.v[188]));
            s.store_add_scaled_inputs_product_indices(339, 335, 1.0, 155, (-1.0), 660, 1436, (-1.0));
            s.store_offset_mul(340, 338, 339, 1.0);
            s.store_scaled_offset(341, 338, 1.0, 2.0);
        }

        s.b[2580] = ((s.v[340] < (1e-6 + s.v[341])) && (s.v[341] >= 0.0));
        s.store_scalar(2580, if s.b[2580] { 1.0 } else { 0.0 });

        if (((!s.b[2578]) && s.b[2579]) && s.b[2580]) {
            s.store_sub_offset_lhs(781, 341, 1e-6, 340);
            s.store_square(722, 781);
            s.store_square(723, 341);
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
        }

        let (assign66580_e103255,) = {
    if (((!s.b[2578]) && s.b[2579]) && s.b[2580]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, assign66580_e103255);

        let (assign66590_e103264,) = {
    if (((!s.b[2578]) && s.b[2579]) && s.b[2580]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, assign66590_e103264);

        if (((!s.b[2578]) && s.b[2579]) && s.b[2580]) {
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

        s.b[2581] = ((((4.0 == 1.0) || (4.0 == 2.0)) || (4.0 == 4.0)) || (4.0 == 8.0));
        s.store_scalar(2581, if s.b[2581] { 1.0 } else { 0.0 });

        s.b[2582] = (4.0 == 1.0);
        s.store_scalar(2582, if s.b[2582] { 1.0 } else { 0.0 });

        let (assign66740_e103421,) = {
    if (((((!s.b[2578]) && s.b[2579]) && s.b[2580]) && s.b[2581]) && s.b[2582]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, assign66740_e103421);

        s.b[2583] = (4.0 == 2.0);
        s.store_scalar(2583, if s.b[2583] { 1.0 } else { 0.0 });

        let (assign66760_e103440,) = {
    if ((((((!s.b[2578]) && s.b[2579]) && s.b[2580]) && s.b[2581]) && (!s.b[2582])) && s.b[2583]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, assign66760_e103440);

        s.b[2584] = (4.0 == 4.0);
        s.store_scalar(2584, if s.b[2584] { 1.0 } else { 0.0 });

        let (assign66780_e103462,) = {
    if (((((((!s.b[2578]) && s.b[2579]) && s.b[2580]) && s.b[2581]) && (!s.b[2582])) && (!s.b[2583])) && s.b[2584]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, assign66780_e103462);

        s.b[2585] = (4.0 == 8.0);
        s.store_scalar(2585, if s.b[2585] { 1.0 } else { 0.0 });

        let (assign66800_e103487,) = {
    if ((((((((!s.b[2578]) && s.b[2579]) && s.b[2580]) && s.b[2581]) && (!s.b[2582])) && (!s.b[2583])) && (!s.b[2584])) && s.b[2585]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, assign66800_e103487);

        let (assign66810_e103498,) = {
    if ((((!s.b[2578]) && s.b[2579]) && s.b[2580]) && s.b[2581]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, assign66810_e103498);

        let mut assign66820_loop_guard: usize = 0;
        while {
            let assign66820_cond_e103510: f64 = if (((((!s.b[2578]) && s.b[2579]) && s.b[2580]) && s.b[2581]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign66820_cond_e103510 != 0.0
        } {
            assign66820_loop_guard += 1;
            assert!(assign66820_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((!s.b[2578]) && s.b[2579]) && s.b[2580]) && s.b[2581]) {
                s.store_sqrt(726, 726);
            }
            let (assign66820_body1_e103535,) = {
    if ((((!s.b[2578]) && s.b[2579]) && s.b[2580]) && s.b[2581]) {
        let assign66820_body1_e103533: f64 = (s.v[719] + 1.0);
        (assign66820_body1_e103533,)
    } else {
        (s.v[719],)
    }
};
            s.store_scalar(719, assign66820_body1_e103535);
        }

        if ((((!s.b[2578]) && s.b[2579]) && s.b[2580]) && (!s.b[2581])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 4.0)));
            }
        }

        if (((!s.b[2578]) && s.b[2579]) && s.b[2580]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_mul3_lhs(780, 781, 341, 726);
            s.store_div_scaled_product3_indices(334, 341, 725, 726, 1.0, 770, 1.0);
            s.store_sub_offset_lhs(340, 341, 1e-6, 780);
        }

        if (((!s.b[2578]) && s.b[2579]) && s.b[2580]) {
        }

        if (((!s.b[2578]) && s.b[2579]) && (!s.b[2580])) {
        }

        if (((!s.b[2578]) && s.b[2579]) && (!s.b[2580])) {
            s.store_scalar(334, 1.0);
        }

        if ((!s.b[2578]) && s.b[2579]) {
            s.store_sqrt(340, 340);
            s.store_add_mul_sub_from_scalar_rhs_indices(282, 335, 337, 1.0, 340);
            s.store_div_from_scalar_offset_input(336, s.v[582], 661, s.v[582]);
            s.store_add_scaled_inputs_product_indices(283, 1437, s.v[483], 109, 1.0, 336, 282, (-1.0));
            s.store_sqrt_square_offset(782, 283, ((4.0 * 0.001) * 0.001));
            s.store_offset_scaled_div(343, 283, 782, 0.5, 0.5);
            s.store_scaled_add(283, 283, 782, 0.5);
        }

        s.b[2586] = (s.v[283] < 0.0);
        s.store_scalar(2586, if s.b[2586] { 1.0 } else { 0.0 });

        if (((!s.b[2578]) && s.b[2579]) && s.b[2586]) {
            s.store_scalar(283, 0.0);
            s.store_scalar(343, 0.0);
        }

        if ((!s.b[2578]) && s.b[2579]) {
            s.store_offset(283, 283, 1e-25);
            s.store_offset_mul_offset_rhs(958, 957, 387, (-s.v[764]), 1.0);
        }

        if ((!s.b[2578]) && s.b[2579]) {
            if (s.v[958] <= 0.001) {
                s.store_scalar(958, 0.001);
            } else {
            }
        }

        if ((!s.b[2578]) && s.b[2579]) {
            s.store_div(339, 662, 958);
            s.store_mul(340, 663, 958);
            s.store_ad_value(336, A::exp_div_scaled_inputs(s.ad_value(340), -1.0, s.ad_value(283), 1.0));
            s.store_mul_product3_indices(280, 336, 339, 283, 134, 1.0);
            s.store_mul3_lhs(288, 339, 283, 336);
        }

        if ((!s.b[2578]) && (!s.b[2579])) {
            s.store_scalar(280, 0.0);
        }

        s.b[2587] = (s.v[664] != 0.0);
        s.store_scalar(2587, if s.b[2587] { 1.0 } else { 0.0 });

        if ((!s.b[2578]) && s.b[2587]) {
            s.copy_ad(334, 799);
            s.store_sqrt_square_offset(782, 334, ((4.0 * 1e-6) * 1e-6));
            s.store_offset_scaled_div(335, 334, 782, 0.5, 0.5);
            s.store_scaled_add(334, 334, 782, 0.5);
        }

        s.b[2588] = (s.v[334] < 0.0);
        s.store_scalar(2588, if s.b[2588] { 1.0 } else { 0.0 });

        if (((!s.b[2578]) && s.b[2587]) && s.b[2588]) {
            s.store_scalar(334, 0.0);
            s.store_scalar(335, 0.0);
        }

        if ((!s.b[2578]) && s.b[2587]) {
            s.store_sqrt_offset_input(335, 127, 1e-25);
            s.store_div_from_scalar_scaled_input(337, 1.0, 335, 2.0);
            s.store_sub_ad_rhs(338, 334, A::scale_offset(s.ad_value(791), ((p.p106) * (p.p105)), p.p105));
            s.store_sqrt_square_offset(782, 338, ((4.0 * 0.01) * 0.01));
            s.store_offset_scaled_div(343, 338, 782, 0.5, 0.5);
            s.store_scaled_add(338, 338, 782, 0.5);
        }

        s.b[2589] = (s.v[338] < 0.0);
        s.store_scalar(2589, if s.b[2589] { 1.0 } else { 0.0 });

        if (((!s.b[2578]) && s.b[2587]) && s.b[2589]) {
            s.store_scalar(338, 0.0);
            s.store_scalar(343, 0.0);
        }

        if ((!s.b[2578]) && s.b[2587]) {
            s.store_offset(338, 338, 1e-25);
            s.store_mul_ad_product_rhs_mixed_ia(344, 450, 451, A::exp(A::div_from_scalar((-1.0), s.ad_value(338))));
            s.store_mul_offset_ad_rhs(345, 344, A::div_from_scalar(1.0, s.ad_value(338)), 1.0);
            s.store_mul(337, 338, 344);
            s.store_sub(334, 334, 337);
            s.store_sqrt_square_offset(782, 334, ((4.0 * 0.01) * 0.01));
            s.store_offset_scaled_div(343, 334, 782, 0.5, 0.5);
            s.store_scaled_add(334, 334, 782, 0.5);
        }

        s.b[2590] = (s.v[334] < 0.0);
        s.store_scalar(2590, if s.b[2590] { 1.0 } else { 0.0 });

        if (((!s.b[2578]) && s.b[2587]) && s.b[2590]) {
            s.store_scalar(334, 0.0);
            s.store_scalar(343, 0.0);
        }

        if ((!s.b[2578]) && s.b[2587]) {
            s.store_offset(334, 334, 1e-25);
            s.store_div_from_scalar_mul_ad(338, 1.0, s.ad_value(334), s.ad_value(335));
            s.store_scalar(341, (s.v[165] * s.v[554]));
            s.store_exp_mul_scaled_lhs_indices(336, 341, -1.0, 338);
            s.store_mul_product3_indices(340, 338, 341, 336, 338, 1.0);
            s.store_mul_product3_indices(281, 336, 664, 134, 334, 1.0);
        }

        s.b[2591] = (p.p45 == 0.0);
        s.store_scalar(2591, if s.b[2591] { 1.0 } else { 0.0 });

        if s.b[2591] {
            s.store_scalar(423, 0.0);
        }

        s.b[2592] = ((p.p45 * (s.v[796] - p.p446)) < 0.0);
        s.store_scalar(2592, if s.b[2592] { 1.0 } else { 0.0 });

        if ((!s.b[2591]) && s.b[2592]) {
            s.copy_ad(426, 427);
        }

        if ((!s.b[2591]) && (!s.b[2592])) {
            s.store_add_scaled_inputs_ad_lhs(426, A::square(A::offset(s.ad_value(796), (-p.p446))), p.p445, 427, 1.0);
        }

        if (!s.b[2591]) {
            s.store_scaled_limited_exp_ad(423, A::mul(s.ad_value(154), A::sub(s.ad_value(793), s.ad_value(426))), p.p449);
        }

        s.b[2593] = (s.v[423] > 0.0);
        s.store_scalar(2593, if s.b[2593] { 1.0 } else { 0.0 });

        s.b[2594] = ((s.v[423] > (100000.0 - 50000.0)) && (50000.0 >= 0.0));
        s.store_scalar(2594, if s.b[2594] { 1.0 } else { 0.0 });

        if (s.b[2593] && s.b[2594]) {
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
        let (assign67570_e104297,) = {
    if (s.b[2593] && s.b[2594]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, assign67570_e104297);

        let (assign67580_e104303,) = {
    if (s.b[2593] && s.b[2594]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, assign67580_e104303);

        if (s.b[2593] && s.b[2594]) {
            s.store_scalar(770, 0.0);
            s.store_scalar(726, 0.0);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_add(770, 724, 725);
            s.copy_ad(726, 770);
        }

        s.b[2595] = ((((1.0 == 1.0) || (1.0 == 2.0)) || (1.0 == 4.0)) || (1.0 == 8.0));
        s.store_scalar(2595, if s.b[2595] { 1.0 } else { 0.0 });

        s.b[2596] = (1.0 == 1.0);
        s.store_scalar(2596, if s.b[2596] { 1.0 } else { 0.0 });

        let (assign67670_e104373,) = {
    if (((s.b[2593] && s.b[2594]) && s.b[2595]) && s.b[2596]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, assign67670_e104373);

        s.b[2597] = (1.0 == 2.0);
        s.store_scalar(2597, if s.b[2597] { 1.0 } else { 0.0 });

        let (assign67690_e104389,) = {
    if ((((s.b[2593] && s.b[2594]) && s.b[2595]) && (!s.b[2596])) && s.b[2597]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, assign67690_e104389);

        s.b[2598] = (1.0 == 4.0);
        s.store_scalar(2598, if s.b[2598] { 1.0 } else { 0.0 });

        let (assign67710_e104408,) = {
    if (((((s.b[2593] && s.b[2594]) && s.b[2595]) && (!s.b[2596])) && (!s.b[2597])) && s.b[2598]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, assign67710_e104408);

        s.b[2599] = (1.0 == 8.0);
        s.store_scalar(2599, if s.b[2599] { 1.0 } else { 0.0 });

        let (assign67730_e104430,) = {
    if ((((((s.b[2593] && s.b[2594]) && s.b[2595]) && (!s.b[2596])) && (!s.b[2597])) && (!s.b[2598])) && s.b[2599]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, assign67730_e104430);

        let (assign67740_e104438,) = {
    if ((s.b[2593] && s.b[2594]) && s.b[2595]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, assign67740_e104438);

        let mut assign67750_loop_guard: usize = 0;
        while {
            let assign67750_cond_e104447: f64 = if (((s.b[2593] && s.b[2594]) && s.b[2595]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign67750_cond_e104447 != 0.0
        } {
            assign67750_loop_guard += 1;
            assert!(assign67750_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((s.b[2593] && s.b[2594]) && s.b[2595]) {
                s.store_sqrt(726, 726);
            }
            let (assign67750_body1_e104466,) = {
    if ((s.b[2593] && s.b[2594]) && s.b[2595]) {
        let assign67750_body1_e104464: f64 = (s.v[719] + 1.0);
        (assign67750_body1_e104464,)
    } else {
        (s.v[719],)
    }
};
            s.store_scalar(719, assign67750_body1_e104466);
        }

        if ((s.b[2593] && s.b[2594]) && (!s.b[2595])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / 2.0));
            }
        }

        if (s.b[2593] && s.b[2594]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, 50000.0);
            s.store_div_scaled_product_indices(334, 725, 726, 50000.0, 770, 1.0);
            s.store_offset(336, 780, (100000.0 - 50000.0));
        }

        if (s.b[2593] && s.b[2594]) {
        }

        if (s.b[2593] && (!s.b[2594])) {
            s.copy_ad(336, 423);
            s.store_scalar(334, 1.0);
        }

        if s.b[2593] {
            s.store_scale(422, 336, (s.v[365] * s.v[632]));
        }

        if (!s.b[2593]) {
            s.store_scalar(422, 0.0);
        }

        s.b[2600] = ((((s.v[280] + s.v[281]) > 0.0) && (s.v[523] != 0.0)) && (s.v[963] == 0.0));
        s.store_scalar(2600, if s.b[2600] { 1.0 } else { 0.0 });

        if s.b[2600] {
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

        s.b[2601] = (p.p24 != 0.0);
        s.store_scalar(2601, if s.b[2601] { 1.0 } else { 0.0 });

        s.b[2602] = (s.v[78] == 0.0);
        s.store_scalar(2602, if s.b[2602] { 1.0 } else { 0.0 });

        if (s.b[2601] && s.b[2602]) {
            s.store_offset_add(191, 109, 1437, (-(10.0 * 2.220446049250313e-16)));
            s.store_sub_scaled_ad_lhs(335, A::add_scaled_product(A::offset(s.ad_value(1438), (-s.v[160])), 1.0, A::sub(s.ad_value(120), s.ad_value(182)), s.ad_value(162), s.v[560]), 191, s.v[515]);
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

        s.b[2603] = (s.v[195] < 0.0);
        s.store_scalar(2603, if s.b[2603] { 1.0 } else { 0.0 });

        if ((s.b[2601] && s.b[2602]) && s.b[2603]) {
            s.store_scalar(195, 0.0);
            s.store_scalar(339, 0.0);
        }

        if (s.b[2601] && s.b[2602]) {
            s.store_sqrt_square_offset(782, 1438, ((4.0 * 0.001) * 0.001));
            s.store_offset_scaled_div(338, 1438, 782, 0.5, 0.5);
            s.store_scaled_add(337, 1438, 782, 0.5);
        }

        s.b[2604] = (s.v[337] < 0.0);
        s.store_scalar(2604, if s.b[2604] { 1.0 } else { 0.0 });

        if ((s.b[2601] && s.b[2602]) && s.b[2604]) {
            s.store_scalar(337, 0.0);
            s.store_scalar(338, 0.0);
        }

        if (s.b[2601] && s.b[2602]) {
            s.store_offset(337, 337, (-p.p262));
            s.store_scale(332, 337, 10.0);
            s.store_offset_square(336, 332, 1.0);
            s.store_sub_from_scalar_ad(335, 1.0, A::div_from_scalar(1.0, s.ad_value(336)));
            s.store_mul(195, 195, 335);
            s.store_scale(334, 162, s.v[632]);
            s.store_div_from_scalar_offset_input(341, s.v[562], 334, s.v[562]);
            s.store_scalar(340, s.v[516]);
            s.store_div_add_scaled_inputs_rhs_indices(343, 340, 340, 1.0, 1437, 1.0);
            s.store_div_from_scalar_offset_input(338, 1.0, 195, 1e-25);
            s.store_scaled_mul(335, 193, 338, (-s.v[514]));
            s.store_scaled_mul(337, 338, 338, s.v[514]);
        }

        s.b[2605] = (s.v[335] < (-34.0));
        s.store_scalar(2605, if s.b[2605] { 1.0 } else { 0.0 });

        if ((s.b[2601] && s.b[2602]) && s.b[2605]) {
            s.store_scalar(199, 0.0);
        }

        if ((s.b[2601] && s.b[2602]) && (!s.b[2605])) {
            s.store_exp(336, 335);
            s.store_mul_scale_ad_lhs(337, A::div_from_scalar(s.v[513], s.ad_value(192)), 1.6021918e-19, 334);
            s.store_div_from_scalar(339, 1.0, 209);
            s.store_sqrt_ad(340, A::mul_offset_lhs(s.ad_value(978), (s.v[188] * 1e-12), s.ad_value(339)));
            s.store_mul3_lhs(338, 336, 337, 340);
            s.store_mul(339, 338, 195);
            s.store_mul(344, 339, 195);
            s.store_mul3_lhs(199, 341, 343, 344);
        }

        if s.b[2601] {
            s.store_offset_scaled(334, 791, (-s.v[518]), s.v[559]);
            s.store_exp_scaled_input(336, 334, s.v[187]);
            s.store_scale(334, 791, (1.0 / (s.v[187]) * 1.0 / (s.v[187])));
            s.store_mul(337, 791, 334);
            s.store_scalar(338, ((s.v[517] / 1000000.0) * s.v[632]));
            s.store_mul3_lhs(200, 338, 336, 337);
        }

        s.b[2606] = (s.v[791] >= 0.0);
        s.store_scalar(2606, if s.b[2606] { 1.0 } else { 0.0 });

        if (s.b[2601] && s.b[2606]) {
            s.store_scale(200, 200, (-1.0));
        }

        if s.b[2601] {
            s.store_sub(335, 791, 790);
            s.store_offset_scaled(334, 335, (-s.v[518]), s.v[559]);
            s.store_exp_scaled_input(336, 334, s.v[187]);
            s.store_scale(334, 335, (1.0 / (s.v[187]) * 1.0 / (s.v[187])));
            s.store_mul(337, 335, 334);
            s.store_scalar(338, ((s.v[517] / 1000000.0) * s.v[632]));
            s.store_mul3_lhs(201, 338, 336, 337);
        }

        s.b[2607] = (s.v[335] >= 0.0);
        s.store_scalar(2607, if s.b[2607] { 1.0 } else { 0.0 });

        if (s.b[2601] && s.b[2607]) {
            s.store_scale(201, 201, (-1.0));
        }

        if s.b[2601] {
            s.store_scaled_offset_ad(195, A::neg(A::sub(s.ad_value(791), s.ad_value(792))), ((s.v[160]) + (p.p258)), 1.0 / (s.v[187]));
            s.store_sqrt_square_offset(782, 195, ((4.0 * (0.01 / 0.01)) * (0.01 / 0.01)));
            s.store_offset_scaled_div(339, 195, 782, 0.5, 0.5);
            s.store_scaled_add(195, 195, 782, 0.5);
        }

        s.b[2608] = (s.v[195] < 0.0);
        s.store_scalar(2608, if s.b[2608] { 1.0 } else { 0.0 });

        if (s.b[2601] && s.b[2608]) {
            s.store_scalar(195, 0.0);
            s.store_scalar(339, 0.0);
        }

        if s.b[2601] {
            s.store_offset(195, 195, 1e-25);
            s.store_div_from_scalar(335, (-s.v[520]), 195);
        }

        s.b[2609] = (s.v[335] < (-34.0));
        s.store_scalar(2609, if s.b[2609] { 1.0 } else { 0.0 });

        if (s.b[2601] && s.b[2609]) {
            s.store_scalar(202, 0.0);
        }

        if (s.b[2601] && (!s.b[2609])) {
            s.store_exp(336, 335);
        }

    }

    pub(super) fn stamp_transient_block_69(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (s.b[2601] && (!s.b[2609])) {
            s.store_mul_div_from_scalar_ad_lhs(337, s.v[520], A::square(s.ad_value(195)), 336);
            s.store_scale(337, 162, (s.v[519] * s.v[632]));
            s.store_mul_product3_indices(202, 336, 337, 195, 195, 1.0);
        }

        if s.b[2601] {
            s.copy_ad(285, 677);
            s.store_mul(286, 393, 285);
            s.store_scaled_offset_ad(336, A::add_scaled_inputs4(s.ad_value(1436), s.v[493], s.ad_value(1438), (-1.0), s.ad_value(122), 1.0, s.ad_value(174), 1.0), (-s.v[492]), (-1.0 / (s.v[187])));
            s.store_square(334, 336);
            s.store_scale(335, 286, s.v[491]);
            s.store_div_scaled_inputs_indices(337, 335, -1.0, 336, 1.0);
        }

        s.b[2610] = (s.v[337] < (-34.0));
        s.store_scalar(2610, if s.b[2610] { 1.0 } else { 0.0 });

        if (s.b[2601] && s.b[2610]) {
            s.store_scalar(339, 0.0);
        }

        if (s.b[2601] && (!s.b[2610])) {
            s.store_exp(339, 337);
        }

        if s.b[2601] {
            s.store_div_from_scalar(338, (((1.6021918e-19 * s.v[490]) * s.v[632]) * s.v[582]), 285);
        }

        s.b[2611] = (((2.0 * s.v[336]) + s.v[335]) < 0.0);
        s.store_scalar(2611, if s.b[2611] { 1.0 } else { 0.0 });

        if (s.b[2601] && s.b[2611]) {
            s.store_mul3_affine_lhs(284, 338, 335, (0.25 * 7.38905609893065), 0.0, 335);
        }

        if (s.b[2601] && (!s.b[2611])) {
            s.store_mul3_lhs(284, 338, 334, 339);
        }

        if s.b[2601] {
            s.store_sub(202, 202, 284);
        }

        s.b[2612] = (p.p25 != 0.0);
        s.store_scalar(2612, if s.b[2612] { 1.0 } else { 0.0 });

        if s.b[2612] {
            s.store_offset_ad(335, A::mul_sub_from_scalar_rhs(s.ad_value(790), 1.0, A::scale(s.ad_value(790), 100.0)), (-1e-5));
            s.store_sqrt_add_scaled_square_input(336, 335, 1.0, 790, (4.0 * 1e-5));
            s.store_add_scaled_inputs3_indices(196, 790, 1.0, 335, (-0.5), 336, (-0.5));
        }

        s.b[2613] = (p.p25 == 0.0);
        s.store_scalar(2613, if s.b[2613] { 1.0 } else { 0.0 });

        if s.b[2613] {
            s.store_scalar(203, 0.0);
        }

        if (!s.b[2613]) {
            s.store_add_scaled_inputs4_offset_indices(335, 196, p.p242, 791, (-1.0), 122, p.p244, 174, p.p244, (p.p243 * p.p242));
            s.store_scalar(336, (1.0 / s.v[187]));
            s.store_mul(194, 335, 336);
            s.store_sqrt_square_offset(782, 194, ((4.0 * (0.01 / 0.01)) * (0.01 / 0.01)));
            s.store_offset_scaled_div(339, 194, 782, 0.5, 0.5);
            s.store_scaled_add(197, 194, 782, 0.5);
        }

        s.b[2614] = (s.v[197] < 0.0);
        s.store_scalar(2614, if s.b[2614] { 1.0 } else { 0.0 });

        if ((!s.b[2613]) && s.b[2614]) {
            s.store_scalar(197, 0.0);
            s.store_scalar(339, 0.0);
        }

        if (!s.b[2613]) {
            s.store_div_from_scalar_offset_input(337, 1.0, 197, 1e-25);
            s.store_scaled_mul(334, 193, 337, (-s.v[512]));
        }

        s.b[2615] = (s.v[334] < (-34.0));
        s.store_scalar(2615, if s.b[2615] { 1.0 } else { 0.0 });

        if ((!s.b[2613]) && s.b[2615]) {
            s.store_scalar(203, 0.0);
        }

        if ((!s.b[2613]) && (!s.b[2615])) {
            s.store_exp(335, 334);
            s.store_scale_ad(336, A::div_from_scalar(s.v[511], s.ad_value(192)), (1.6021918e-19 * s.v[632]));
            s.store_mul_product3_indices(203, 335, 336, 197, 197, 1.0);
        }

        if (!s.b[2613]) {
            s.store_sub(205, 790, 792);
        }

        s.b[2616] = (s.v[205] > 0.0);
        s.store_scalar(2616, if s.b[2616] { 1.0 } else { 0.0 });

        if ((!s.b[2613]) && s.b[2616]) {
            s.store_square(336, 205);
            s.store_mul(338, 336, 205);
            s.store_offset(334, 338, 0.5);
            s.store_div(339, 338, 334);
            s.store_div_ad(341, A::add_scaled_products(s.ad_value(336), s.ad_value(334), 3.0, s.ad_value(338), s.ad_value(336), (-3.0)), A::square(s.ad_value(334)));
            s.store_mul(203, 203, 339);
        }

        if ((!s.b[2613]) && (!s.b[2616])) {
            s.store_scalar(203, 0.0);
        }

        s.b[2617] = (p.p25 == 0.0);
        s.store_scalar(2617, if s.b[2617] { 1.0 } else { 0.0 });

        if s.b[2617] {
            s.store_scalar(204, 0.0);
        }

        if (!s.b[2617]) {
            s.store_add_scaled_inputs3_mixed_aii(335, A::add_scaled_inputs3_offset(s.ad_value(196), (-p.p242), s.ad_value(791), -1.0, s.ad_value(196), 1.0, ((p.p243) * (p.p242))), 1.0, 122, p.p244, 174, p.p244);
            s.store_scalar(336, (1.0 / s.v[187]));
            s.store_mul(194, 335, 336);
            s.store_sqrt_square_offset(782, 194, ((4.0 * (0.01 / 0.01)) * (0.01 / 0.01)));
            s.store_offset_scaled_div(339, 194, 782, 0.5, 0.5);
            s.store_scaled_add(198, 194, 782, 0.5);
        }

        s.b[2618] = (s.v[198] < 0.0);
        s.store_scalar(2618, if s.b[2618] { 1.0 } else { 0.0 });

        if ((!s.b[2617]) && s.b[2618]) {
            s.store_scalar(198, 0.0);
            s.store_scalar(339, 0.0);
        }

        if (!s.b[2617]) {
            s.store_div_from_scalar_offset_input(337, 1.0, 198, 1e-25);
            s.store_scaled_mul(334, 193, 337, (-s.v[512]));
        }

        s.b[2619] = (s.v[334] < (-34.0));
        s.store_scalar(2619, if s.b[2619] { 1.0 } else { 0.0 });

        if ((!s.b[2617]) && s.b[2619]) {
            s.store_scalar(204, 0.0);
        }

        if ((!s.b[2617]) && (!s.b[2619])) {
            s.store_exp(335, 334);
            s.store_div_from_scalar(337, 1.0, 192);
            s.store_scale(336, 337, (s.v[511] * (1.6021918e-19 * s.v[632])));
            s.store_mul_product3_indices(204, 335, 336, 198, 198, 1.0);
        }

        if (!s.b[2617]) {
            s.store_neg(206, 792);
        }

        s.b[2620] = (s.v[206] > 0.0);
        s.store_scalar(2620, if s.b[2620] { 1.0 } else { 0.0 });

        if ((!s.b[2617]) && s.b[2620]) {
            s.store_square(336, 206);
            s.store_mul(338, 336, 206);
            s.store_offset(334, 338, 0.5);
            s.store_div(339, 338, 334);
            s.store_div_ad(341, A::add_scaled_products(s.ad_value(336), s.ad_value(334), 3.0, s.ad_value(338), s.ad_value(336), (-3.0)), A::square(s.ad_value(334)));
            s.store_mul(204, 204, 339);
        }

        if ((!s.b[2617]) && (!s.b[2620])) {
            s.store_scalar(204, 0.0);
        }

        s.store_scalar(2621, 0.0);

        s.store_scalar(2624, 0.0);

        s.store_scalar(2623, 0.0);

        s.store_scalar(406, 0.0);

        s.store_scalar(2623, 0.0);

        s.b[2625] = (1.0 == 1.0);
        s.store_scalar(2625, if s.b[2625] { 1.0 } else { 0.0 });

        s.b[2626] = (1.0 == 2.0);
        s.store_scalar(2626, if s.b[2626] { 1.0 } else { 0.0 });

        s.b[2627] = (1.0 == 3.0);
        s.store_scalar(2627, if s.b[2627] { 1.0 } else { 0.0 });

        s.b[2628] = (1.0 == 4.0);
        s.store_scalar(2628, if s.b[2628] { 1.0 } else { 0.0 });

        s.b[2629] = (((p.p36 == 1.0) && (p.p66 > 0.0)) && (s.v[460] > 0.0));
        s.store_scalar(2629, if s.b[2629] { 1.0 } else { 0.0 });

        let (assign69610_e106046,) = {
    if (s.b[2625] && s.b[2629]) {
        (1.0,)
    } else {
        (s.v[2623],)
    }
};
        s.store_scalar(2623, assign69610_e106046);

        let (assign69620_e106052,) = {
    if (s.b[2625] && s.b[2629]) {
        (1.0,)
    } else {
        (s.v[2621],)
    }
};
        s.store_scalar(2621, assign69620_e106052);

        if (s.b[2625] && s.b[2629]) {
            s.store_sub(395, 731, 728);
            s.store_neg(396, 728);
            s.store_scalar(409, s.v[460]);
            s.store_scalar(407, p.p66);
            s.store_scalar(411, 0.0);
            s.copy_ad(410, 687);
            s.store_scalar(413, s.v[188]);
        }

        s.b[2630] = (((((p.p36 == 1.0) && (p.p66 > 0.0)) && (s.v[460] > 0.0)) && (s.v[526] != 0.0)) && (p.p55 != 1.0));
        s.store_scalar(2630, if s.b[2630] { 1.0 } else { 0.0 });

        let (assign69710_e106125,) = {
    if ((s.b[2626] && (!s.b[2625])) && s.b[2630]) {
        (1.0,)
    } else {
        (s.v[2623],)
    }
};
        s.store_scalar(2623, assign69710_e106125);

        if ((s.b[2626] && (!s.b[2625])) && s.b[2630]) {
            s.store_sub(395, 734, 735);
            s.store_neg(396, 735);
        }

        s.b[2631] = (((p.p35 == 1.0) && (p.p63 > 0.0)) && (s.v[459] > 0.0));
        s.store_scalar(2631, if s.b[2631] { 1.0 } else { 0.0 });

        let (assign69750_e106168,) = {
    if ((s.b[2627] && (!(s.b[2625] || s.b[2626]))) && s.b[2631]) {
        (1.0,)
    } else {
        (s.v[2623],)
    }
};
        s.store_scalar(2623, assign69750_e106168);

        let (assign69760_e106179,) = {
    if ((s.b[2627] && (!(s.b[2625] || s.b[2626]))) && s.b[2631]) {
        (1.0,)
    } else {
        (s.v[2624],)
    }
};
        s.store_scalar(2624, assign69760_e106179);

        if ((s.b[2627] && (!(s.b[2625] || s.b[2626]))) && s.b[2631]) {
            s.store_sub(395, 731, 728);
            s.store_sub(396, 729, 728);
            s.store_scalar(409, s.v[459]);
            s.store_scalar(407, (p.p63 + (p.p64 * p.p55)));
            s.copy_ad(411, 384);
            s.copy_ad(410, 686);
            s.copy_ad(413, 412);
            s.store_neg(407, 407);
        }

        s.b[2632] = (((s.v[407] < 0.0) && (p.p432 > 0.0)) && (p.p55 == 1.0));
        s.store_scalar(2632, if s.b[2632] { 1.0 } else { 0.0 });

        if (((s.b[2627] && (!(s.b[2625] || s.b[2626]))) && s.b[2631]) && s.b[2632]) {
            s.store_neg(407, 407);
            s.store_scalar(335, p.p63);
            s.store_offset_div_scaled_product(996, s.ad_value(335), s.ad_value(335), 1.0, s.ad_value(651), 1.0, (-p.p137));
        }

        s.b[2633] = (p.p113 > 0.0);
        s.store_scalar(2633, if s.b[2633] { 1.0 } else { 0.0 });

        s.b[2634] = ((s.v[396] == 0.0) || (p.p113 <= 0.0));
        s.store_scalar(2634, if s.b[2634] { 1.0 } else { 0.0 });

        if (((((s.b[2627] && (!(s.b[2625] || s.b[2626]))) && s.b[2631]) && s.b[2632]) && s.b[2633]) && s.b[2634]) {
        }

        if (((((s.b[2627] && (!(s.b[2625] || s.b[2626]))) && s.b[2631]) && s.b[2632]) && s.b[2633]) && (!s.b[2634])) {
            s.store_scalar(783, (if (s.v[396] < 0.0) { (-1.0) } else { 1.0 }));
        }

        if (((((s.b[2627] && (!(s.b[2625] || s.b[2626]))) && s.b[2631]) && s.b[2632]) && s.b[2633]) && (!s.b[2634])) {
            s.store_mul(784, 783, 396);
            s.store_offset_powf_ad(781, A::div(s.ad_value(784), s.ad_value(996)), p.p113, 1.0);
            s.store_powf(782, 781, (1.0 / p.p113));
            s.store_div_scaled_product_indices(396, 783, 784, 1.0, 782, 1.0);
        }

        if ((((s.b[2627] && (!(s.b[2625] || s.b[2626]))) && s.b[2631]) && s.b[2632]) && s.b[2633]) {
            s.store_sqrt_offset_square_offset(782, 396, p.p137, ((4.0 * 0.1) * 0.1));
            s.store_scaled_offset_ad(343, A::div_scaled_offset_numerator(s.ad_value(396), 1.0, p.p137, s.ad_value(782), 1.0), 1.0, 0.5);
            s.store_scaled_add_ad_lhs(336, A::offset(s.ad_value(396), p.p137), 782, 0.5);
        }

        s.b[2635] = (s.v[336] < 0.0);
        s.store_scalar(2635, if s.b[2635] { 1.0 } else { 0.0 });

        if (((((s.b[2627] && (!(s.b[2625] || s.b[2626]))) && s.b[2631]) && s.b[2632]) && s.b[2633]) && s.b[2635]) {
            s.store_scalar(336, 0.0);
            s.store_scalar(343, 0.0);
        }

        if ((((s.b[2627] && (!(s.b[2625] || s.b[2626]))) && s.b[2631]) && s.b[2632]) && s.b[2633]) {
            s.store_scaled_sqrt_mul(600, 651, 336, p.p432);
            s.store_sub(407, 407, 600);
        }

    }

    pub(super) fn stamp_transient_block_70(
        s: &mut Scratch,
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        s.b[2636] = (((((p.p35 == 1.0) && (p.p63 > 0.0)) && (s.v[459] > 0.0)) && (s.v[526] != 0.0)) && (p.p55 != 1.0));
        s.store_scalar(2636, if s.b[2636] { 1.0 } else { 0.0 });

        let (assign70060_e106650,) = {
    if ((s.b[2628] && (!((s.b[2625] || s.b[2626]) || s.b[2627]))) && s.b[2636]) {
        (1.0,)
    } else {
        (s.v[2623],)
    }
};
        s.store_scalar(2623, assign70060_e106650);

        if ((s.b[2628] && (!((s.b[2625] || s.b[2626]) || s.b[2627]))) && s.b[2636]) {
            s.store_sub(395, 734, 735);
            s.store_sub(396, 733, 735);
        }

        if (s.v[2623] != 0.0) {
            s.store_scalar(2644, 0.4);
        }

        let (assign70110_e106692,) = {
    if (s.v[2623] != 0.0) {
        (0.0,)
    } else {
        (s.v[2645],)
    }
};
        s.store_scalar(2645, assign70110_e106692);

        if (s.v[2623] != 0.0) {
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

        let (assign70240_e106745,) = {
    if (s.v[2623] != 0.0) {
        let assign70240_e106743: f64 = (-1.0);
        (assign70240_e106743,)
    } else {
        (s.v[79],)
    }
};
        s.store_scalar(79, assign70240_e106745);

        if (s.v[2623] != 0.0) {
            s.store_scalar(2646, 0.0);
            s.store_scalar(2647, 0.0);
            s.store_mul_scaled_ln_ad_rhs(2642, 155, 2.0, A::div(s.ad_value(409), s.ad_value(394)));
            s.store_offset_sub_from_scalar_ad(781, 0.8, s.ad_value(2642), (-0.1));
            s.store_scalar(782, ((4.0 * 0.8) * 0.1));
        }

        if (s.v[2623] != 0.0) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if (s.v[2623] != 0.0) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(2643, 781, (-0.5), 782, (-0.5), 0.8);
        }

        s.b[2649] = (s.v[2644] > (s.v[2643] * 0.5));
        s.store_scalar(2649, if s.b[2649] { 1.0 } else { 0.0 });

        if ((s.v[2623] != 0.0) && s.b[2649]) {
            s.store_scale(2644, 2643, 0.5);
        }

        s.b[2650] = param_given[338];
        s.store_scalar(2650, if s.b[2650] { 1.0 } else { 0.0 });

        if ((s.v[2623] != 0.0) && s.b[2650]) {
            s.store_scalar(2643, p.p338);
        }

        s.b[2651] = param_given[339];
        s.store_scalar(2651, if s.b[2651] { 1.0 } else { 0.0 });

        if ((s.v[2623] != 0.0) && s.b[2651]) {
            s.store_scalar(2644, p.p339);
        }

        s.b[2652] = param_given[338];
        s.store_scalar(2652, if s.b[2652] { 1.0 } else { 0.0 });

        if (((s.v[2623] != 0.0) && (!s.b[2651])) && s.b[2652]) {
            s.store_scale(2644, 2643, 0.5);
        }

        s.b[2653] = (s.v[2644] > (s.v[2643] * 0.5));
        s.store_scalar(2653, if s.b[2653] { 1.0 } else { 0.0 });

        if ((s.v[2623] != 0.0) && s.b[2653]) {
            s.store_scale(2644, 2643, 0.5);
        }

        s.b[2654] = (p.p38 == 1.0);
        s.store_scalar(2654, if s.b[2654] { 1.0 } else { 0.0 });

        if ((s.v[2623] != 0.0) && s.b[2654]) {
            s.store_neg(334, 396);
        }

        s.b[2655] = (s.v[334] > s.v[2644]);
        s.store_scalar(2655, if s.b[2655] { 1.0 } else { 0.0 });

        if (((s.v[2623] != 0.0) && s.b[2654]) && s.b[2655]) {
            s.store_sub(335, 334, 2644);
            s.store_sub(336, 2643, 2644);
            s.store_div(781, 335, 336);
            s.store_square(782, 781);
            s.store_mul(783, 782, 781);
            s.store_square(784, 782);
            s.store_div_from_scalar_ad(780, 1.0, A::add_scaled_inputs4_offset(s.ad_value(781), 1.0, s.ad_value(782), 1.0, s.ad_value(783), 1.0, s.ad_value(784), 1.0, 1.0));
            s.store_mul_ad_affine_product_lhs(345, A::add_scaled_inputs3_offset(s.ad_value(781), 2.0, s.ad_value(782), 3.0, s.ad_value(783), 4.0, 1.0), s.ad_value(780), -1.0, 0.0, 780);
            s.store_mul_sub_from_scalar_rhs(333, 336, 1.0, 780);
            s.store_add_scaled_sub_value_product_indices(334, 1.0, 780, 1.0, 781, 345, 1.0);
            s.store_neg(345, 345);
            s.store_add(344, 2644, 333);
        }

        if (((s.v[2623] != 0.0) && s.b[2654]) && (!s.b[2655])) {
            s.copy_ad(344, 334);
        }

        if ((s.v[2623] != 0.0) && s.b[2654]) {
            s.store_neg(397, 344);
        }

        if ((s.v[2623] != 0.0) && (!s.b[2654])) {
            s.copy_ad(397, 396);
        }

        if (s.v[2623] != 0.0) {
            s.store_div(212, 410, 413);
            s.store_square(213, 212);
            s.store_sub_from_scalar(402, s.v[458], 395);
        }

        let (assign70650_e107086,) = {
    if (s.v[2623] != 0.0) {
        let assign70650_e107080: f64 = (-s.v[397]);
        let assign70650_e107083: f64 = (10.0 * 2.220446049250313e-16);
        let assign70650_e107084: f64 = (assign70650_e107080 + assign70650_e107083);
        (assign70650_e107084,)
    } else {
        (s.v[403],)
    }
};
        s.store_scalar(403, assign70650_e107086);

        if (s.v[2623] != 0.0) {
            s.store_scalar(2638, 0.0);
            s.store_scale(2639, 409, 1.6021918e-19);
            s.store_div(334, 394, 409);
            s.store_square(405, 334);
        }

        s.b[2656] = ((s.v[154] * (-s.v[397])) >= 500.0);
        s.store_scalar(2656, if s.b[2656] { 1.0 } else { 0.0 });

        if ((s.v[2623] != 0.0) && s.b[2656]) {
            s.store_scaled_offset_ad(229, A::mul_scaled_rhs(s.ad_value(154), s.ad_value(397), -1.0), ((1.0) + ((-500.0))), 1.403592217853e217);
            s.store_scalar(334, 1.403592217853e217);
        }

        if ((s.v[2623] != 0.0) && (!s.b[2656])) {
            s.store_mul_neg_rhs(781, 154, 397);
            s.store_scalar(229, 1.0);
        }

        let mut assign70750_loop_guard: usize = 0;
        while {
            let assign70750_cond_e107160: f64 = if (((s.v[2623] != 0.0) && (!s.b[2656])) && (s.v[781] >= 60.0)) { 1.0 } else { 0.0 };
            assign70750_cond_e107160 != 0.0
        } {
            assign70750_loop_guard += 1;
            assert!(assign70750_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((s.v[2623] != 0.0) && (!s.b[2656])) {
                s.store_scale(229, 229, 1.14200738981568e26);
                s.store_offset(781, 781, (-60.0));
            }
        }

        if ((s.v[2623] != 0.0) && (!s.b[2656])) {
            s.store_mul_exp_rhs(229, 229, 781);
            s.copy_ad(334, 229);
        }

        if ((s.v[2623] != 0.0) && (s.v[406] != 0.0)) {
            s.store_offset_scaled(781, 402, (-0.5), (((-0.5)) + ((-1.0))));
            s.store_scalar(782, (4.0 * 0.5));
        }

        if ((s.v[2623] != 0.0) && (s.v[406] != 0.0)) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if ((s.v[2623] != 0.0) && (s.v[406] != 0.0)) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(335, 781, 0.5, 782, 0.5, 0.5);
        }

        s.b[2657] = (((s.v[402] + s.v[397]) > (-s.v[335])) && (s.v[335] >= 0.0));
        s.store_scalar(2657, if s.b[2657] { 1.0 } else { 0.0 });

        if (((s.v[2623] != 0.0) && (s.v[406] != 0.0)) && s.b[2657]) {
            s.store_add_scaled_inputs3_indices(781, 402, 1.0, 397, 1.0, 335, 1.0);
            s.store_square(722, 781);
            s.store_square(723, 335);
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
        }

        let (assign70900_e107334,) = {
    if (((s.v[2623] != 0.0) && (s.v[406] != 0.0)) && s.b[2657]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, assign70900_e107334);

        let (assign70910_e107342,) = {
    if (((s.v[2623] != 0.0) && (s.v[406] != 0.0)) && s.b[2657]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, assign70910_e107342);

        if (((s.v[2623] != 0.0) && (s.v[406] != 0.0)) && s.b[2657]) {
            s.store_scalar(770, 0.0);
            s.store_scalar(726, 0.0);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_add(770, 724, 725);
            s.copy_ad(726, 770);
        }

        s.b[2658] = ((((1.0 == 1.0) || (1.0 == 2.0)) || (1.0 == 4.0)) || (1.0 == 8.0));
        s.store_scalar(2658, if s.b[2658] { 1.0 } else { 0.0 });

        s.b[2659] = (1.0 == 1.0);
        s.store_scalar(2659, if s.b[2659] { 1.0 } else { 0.0 });

        let (assign71000_e107426,) = {
    if (((((s.v[2623] != 0.0) && (s.v[406] != 0.0)) && s.b[2657]) && s.b[2658]) && s.b[2659]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, assign71000_e107426);

        s.b[2660] = (1.0 == 2.0);
        s.store_scalar(2660, if s.b[2660] { 1.0 } else { 0.0 });

        let (assign71020_e107444,) = {
    if ((((((s.v[2623] != 0.0) && (s.v[406] != 0.0)) && s.b[2657]) && s.b[2658]) && (!s.b[2659])) && s.b[2660]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, assign71020_e107444);

        s.b[2661] = (1.0 == 4.0);
        s.store_scalar(2661, if s.b[2661] { 1.0 } else { 0.0 });

        let (assign71040_e107465,) = {
    if (((((((s.v[2623] != 0.0) && (s.v[406] != 0.0)) && s.b[2657]) && s.b[2658]) && (!s.b[2659])) && (!s.b[2660])) && s.b[2661]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, assign71040_e107465);

        s.b[2662] = (1.0 == 8.0);
        s.store_scalar(2662, if s.b[2662] { 1.0 } else { 0.0 });

        let (assign71060_e107489,) = {
    if ((((((((s.v[2623] != 0.0) && (s.v[406] != 0.0)) && s.b[2657]) && s.b[2658]) && (!s.b[2659])) && (!s.b[2660])) && (!s.b[2661])) && s.b[2662]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, assign71060_e107489);

        let (assign71070_e107499,) = {
    if ((((s.v[2623] != 0.0) && (s.v[406] != 0.0)) && s.b[2657]) && s.b[2658]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, assign71070_e107499);

        let mut assign71080_loop_guard: usize = 0;
        while {
            let assign71080_cond_e107510: f64 = if (((((s.v[2623] != 0.0) && (s.v[406] != 0.0)) && s.b[2657]) && s.b[2658]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign71080_cond_e107510 != 0.0
        } {
            assign71080_loop_guard += 1;
            assert!(assign71080_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.v[2623] != 0.0) && (s.v[406] != 0.0)) && s.b[2657]) && s.b[2658]) {
                s.store_sqrt(726, 726);
            }
            let (assign71080_body1_e107533,) = {
    if ((((s.v[2623] != 0.0) && (s.v[406] != 0.0)) && s.b[2657]) && s.b[2658]) {
        let assign71080_body1_e107531: f64 = (s.v[719] + 1.0);
        (assign71080_body1_e107531,)
    } else {
        (s.v[719],)
    }
};
            s.store_scalar(719, assign71080_body1_e107533);
        }

        if ((((s.v[2623] != 0.0) && (s.v[406] != 0.0)) && s.b[2657]) && (!s.b[2658])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / 2.0));
            }
        }

        if (((s.v[2623] != 0.0) && (s.v[406] != 0.0)) && s.b[2657]) {
            s.store_div_from_scalar(726, 1.0, 726);
        }

    }

    pub(super) fn stamp_transient_block_71(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((s.v[2623] != 0.0) && (s.v[406] != 0.0)) && s.b[2657]) {
            s.store_mul3_lhs(780, 781, 335, 726);
            s.store_div_scaled_product3_indices(334, 335, 725, 726, 1.0, 770, 1.0);
            s.store_add_scaled_inputs(335, 335, -1.0, 780, 1.0);
        }

        if (((s.v[2623] != 0.0) && (s.v[406] != 0.0)) && s.b[2657]) {
        }

        if (((s.v[2623] != 0.0) && (s.v[406] != 0.0)) && (!s.b[2657])) {
            s.store_add(335, 402, 397);
            s.store_scalar(334, 1.0);
        }

        if ((s.v[2623] != 0.0) && (s.v[406] != 0.0)) {
            s.store_sub(397, 335, 402);
        }

        let (assign71180_e107650,) = {
    if ((s.v[2623] != 0.0) && (s.v[406] != 0.0)) {
        let assign71180_e107644: f64 = (-s.v[397]);
        let assign71180_e107647: f64 = (10.0 * 2.220446049250313e-16);
        let assign71180_e107648: f64 = (assign71180_e107644 + assign71180_e107647);
        (assign71180_e107648,)
    } else {
        (s.v[403],)
    }
};
        s.store_scalar(403, assign71180_e107650);

        s.b[2663] = (s.v[402] < s.v[403]);
        s.store_scalar(2663, if s.b[2663] { 1.0 } else { 0.0 });

        if ((s.v[2623] != 0.0) && s.b[2663]) {
            s.store_mul_scaled_ln_ad_rhs(271, 155, 2.0, A::div_from_scalar((-s.v[270]), s.ad_value(212)));
            s.store_mul_add_rhs(332, 154, 402, 397);
            s.store_div_from_scalar_mul_ad(335, 1.0, s.ad_value(154), s.ad_value(410));
            s.store_mul(333, 335, 413);
            s.store_offset_scaled(279, 333, (3.0 * 1.414213562373095), 2.0);
            s.store_mul3_affine_lhs(277, 279, 279, 8.0, 0.0, 279);
            s.store_sub_from_scalar_scaled_mul_ad_rhs(278, (7.0 * 1.414213562373095), 333, A::offset(s.ad_value(332), (-2.0)), 9.0);
            s.store_square(276, 278);
        }

        s.b[2664] = (s.v[277] < (s.v[276] * 1e-8));
        s.store_scalar(2664, if s.b[2664] { 1.0 } else { 0.0 });

        if (((s.v[2623] != 0.0) && s.b[2663]) && s.b[2664]) {
            s.store_div_scaled_inputs_indices(274, 277, 0.5, 278, 1.0);
        }

        if (((s.v[2623] != 0.0) && s.b[2663]) && (!s.b[2664])) {
            s.store_sqrt_add(275, 277, 276);
            s.store_sub(274, 275, 278);
        }

        if ((s.v[2623] != 0.0) && s.b[2663]) {
            s.store_powf(273, 274, 0.3333333333333333);
            s.store_add_scaled_inputs_product_first_ad(272, A::sub_from_scalar(((-4.0) * 1.414213562373095), A::scale(s.ad_value(333), 12.0)), 1.0, 273, 2.0, 273, 273, 1.414213562373095);
            s.store_div(116, 272, 273);
            s.store_mul(335, 116, 155);
            s.store_div(336, 335, 271);
            s.store_sqrt_square_offset(337, 336, 1.0);
            s.store_sub_div_lhs_indices(404, 335, 337, 397);
            s.store_sub(336, 402, 404);
            s.store_mul(398, 413, 336);
            s.copy_ad(354, 398);
            s.copy_ad(2646, 404);
        }

        s.b[2665] = ((s.v[154] * (s.v[402] + s.v[397])) < (1.0 + (((((10.0 * 2.220446049250313e-16) - 1.0) * s.v[213]) * s.v[156]) / 4.0)));
        s.store_scalar(2665, if s.b[2665] { 1.0 } else { 0.0 });

        if (((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2665]) {
            s.store_add_scaled_product_indices(89, 402, 1.0, 213, 154, 0.5);
        }

        if (((s.v[2623] != 0.0) && (!s.b[2663])) && (!s.b[2665])) {
            s.store_offset_div_scaled_offset_numerator(332, A::mul(s.ad_value(154), A::add(s.ad_value(402), s.ad_value(397))), 4.0, ((-1.0) * 4.0), A::mul(s.ad_value(213), s.ad_value(156)), 1.0, 1.0);
            s.store_add_product3_rhs_mixed_iia(89, 402, 213, 154, A::sub_from_scalar(1.0, A::sqrt(s.ad_value(332))), 1.0 / (2.0));
        }

        if ((s.v[2623] != 0.0) && (!s.b[2663])) {
            s.store_mul_add_rhs(116, 154, 89, 397);
        }

        s.b[2666] = (s.v[116] >= 3.0);
        s.store_scalar(2666, if s.b[2666] { 1.0 } else { 0.0 });

        if (((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2666]) {
            s.store_exp_neg_input(333, 116);
            s.store_offset_div_scaled_inputs2_mixed_aia(332, A::offset(A::mul(s.ad_value(154), A::add(s.ad_value(402), s.ad_value(397))), (-1.0)), 4.0, 333, 4.0, A::mul(s.ad_value(213), s.ad_value(156)), 1.0, 1.0);
            s.store_add_product3_rhs_mixed_iia(89, 402, 213, 154, A::sub_from_scalar(1.0, A::sqrt(s.ad_value(332))), 1.0 / (2.0));
            s.store_mul_add_rhs(116, 154, 89, 397);
            s.store_exp_neg_input(333, 116);
            s.store_offset_div_scaled_inputs2_mixed_aia(332, A::offset(A::mul(s.ad_value(154), A::add(s.ad_value(402), s.ad_value(397))), (-1.0)), 4.0, 333, 4.0, A::mul(s.ad_value(213), s.ad_value(156)), 1.0, 1.0);
            s.store_add_product3_rhs_mixed_iia(89, 402, 213, 154, A::sub_from_scalar(1.0, A::sqrt(s.ad_value(332))), 1.0 / (2.0));
            s.store_mul_add_rhs(116, 154, 89, 397);
        }

        if (((s.v[2623] != 0.0) && (!s.b[2663])) && (!s.b[2666])) {
            s.store_scalar(434, ((1.0 / (9.0 * ((2.0) as f64).sqrt())) - ((5.0 + (7.0 * (((-3.0)) as f64).exp())) / (54.0 * (((2.0 + (((-3.0)) as f64).exp())) as f64).sqrt()))));
            s.store_scalar(435, (((1.0 + (((-3.0)) as f64).exp()) / (2.0 * (((2.0 + (((-3.0)) as f64).exp())) as f64).sqrt())) - (((2.0) as f64).sqrt() / 3.0)));
            s.store_offset_div_from_scalar_ad(436, 1.0, A::mul(s.ad_value(154), s.ad_value(212)), (1.0 / ((2.0) as f64).sqrt()));
            s.store_div_scaled_inputs2_indices(437, 402, -1.0, 397, -1.0, 212, 1.0);
            s.store_add_scaled_inputs3_div_scaled_third_mixed_aaii(441, A::div_scaled_product(A::square(s.ad_value(435)), s.ad_value(435), 1.0, A::mul3_scaled_output(s.ad_value(434), s.ad_value(434), s.ad_value(434), 27.0), 1.0), 1.0, A::div_scaled_product_by_product(s.ad_value(435), s.ad_value(436), 1.0, s.ad_value(434), s.ad_value(434), 6.0), (-1.0), 437, 1.0, 434, 2.0, 1.0);
            s.store_div_ad(440, A::add_scaled_square_product(s.ad_value(435), (-1.0), s.ad_value(434), s.ad_value(436), 3.0), A::mul_scaled_lhs(s.ad_value(434), 9.0, s.ad_value(434)));
            s.store_sqrt_add_scaled_square_cube_product(339, 441, 1.0, 440, 1.0);
            s.store_powf_ad(439, A::sub(s.ad_value(339), s.ad_value(441)), 0.3333333333333333);
            s.store_neg_powf_add_input(438, 441, 339, 0.3333333333333333);
            s.store_add_scaled_inputs3_div_scaled_third_indices(116, 439, 1.0, 438, 1.0, 435, 1.0, 434, 3.0, -1.0);
            s.store_add_scaled_product_indices(89, 397, (-1.0), 116, 155, 1.0);
        }

        s.b[2667] = (p.p33 > 0.0);
        s.store_scalar(2667, if s.b[2667] { 1.0 } else { 0.0 });

        if (((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2667]) {
            s.store_offset_add(442, 402, 397, 0.1);
            s.store_mul(222, 405, 229);
            s.store_mul(443, 405, 229);
            s.store_mul(334, 156, 213);
            s.store_mul(444, 154, 442);
            s.store_add_scaled_inputs_product_mixed_aaii(447, A::ln(A::add_scaled_square_product(s.ad_value(444), 1.0, s.ad_value(443), s.ad_value(334), 1.0)), 1.0, A::ln(A::mul(s.ad_value(405), s.ad_value(334))), (-1.0), 154, 397, 1.0);
        }

        s.b[2668] = (p.p33 == 2.0);
        s.store_scalar(2668, if s.b[2668] { 1.0 } else { 0.0 });

        if ((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2667]) && s.b[2668]) {
            s.store_offset_sub(781, 444, 447, (-1.0));
            s.store_scale(782, 444, 4.0);
        }

        if ((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2667]) && s.b[2668]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if ((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2667]) && s.b[2668]) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(335, 781, 782, 0.5, 0.5);
            s.store_add_scaled_inputs3_indices(447, 444, 1.0, 781, (-0.5), 782, (-0.5));
        }

        if ((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2667]) && (!s.b[2668])) {
            if (s.v[447] <= s.v[444]) {
            } else {
                s.copy_ad(447, 444);
            }
        }

        if (((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2667]) {
            if (s.v[447] >= 0.0) {
            } else {
                s.store_scalar(447, 0.0);
            }
        }

        if (((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2667]) {
            s.store_sub(444, 444, 447);
            s.store_add_scaled_inputs(444, 444, 1.0, 154, 0.1);
            s.store_sub_ad(335, A::ln(A::add_scaled_square_product(s.ad_value(444), 1.0, s.ad_value(443), s.ad_value(334), 1.0)), A::ln(A::mul(s.ad_value(405), s.ad_value(334))));
            s.store_add_scaled_product_indices(446, 335, 1.0, 154, 397, 1.0);
        }

        if (((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2667]) {
            if (s.v[446] >= 0.0) {
            } else {
                s.store_scalar(446, 0.0);
            }
        }

        if (((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2667]) {
            s.copy_ad(445, 116);
        }

        s.b[2669] = (p.p33 == 2.0);
        s.store_scalar(2669, if s.b[2669] { 1.0 } else { 0.0 });

        s.b[2670] = ((s.v[445] > (s.v[446] - (0.2 * s.v[446]))) && ((0.2 * s.v[446]) >= 0.0));
        s.store_scalar(2670, if s.b[2670] { 1.0 } else { 0.0 });

        if (((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2667]) && s.b[2669]) && s.b[2670]) {
            s.store_add_scaled_inputs3_indices(781, 445, 1.0, 446, (-1.0), 446, 0.2);
            s.store_square(722, 781);
            s.store_scaled_mul(723, 446, 446, (0.2 * 0.2));
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
        }

        let (assign72010_e108796,) = {
    if (((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2667]) && s.b[2669]) && s.b[2670]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, assign72010_e108796);

        let (assign72020_e108809,) = {
    if (((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2667]) && s.b[2669]) && s.b[2670]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, assign72020_e108809);

        if (((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2667]) && s.b[2669]) && s.b[2670]) {
            s.store_scalar(770, 0.0);
            s.store_scalar(726, 0.0);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_add(770, 724, 725);
            s.copy_ad(726, 770);
        }

        s.b[2671] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.store_scalar(2671, if s.b[2671] { 1.0 } else { 0.0 });

        s.b[2672] = (2.0 == 1.0);
        s.store_scalar(2672, if s.b[2672] { 1.0 } else { 0.0 });

        let (assign72130_e108958,) = {
    if (((((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2667]) && s.b[2669]) && s.b[2670]) && s.b[2671]) && s.b[2672]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, assign72130_e108958);

        s.b[2673] = (2.0 == 2.0);
        s.store_scalar(2673, if s.b[2673] { 1.0 } else { 0.0 });

        let (assign72150_e108981,) = {
    if ((((((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2667]) && s.b[2669]) && s.b[2670]) && s.b[2671]) && (!s.b[2672])) && s.b[2673]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, assign72150_e108981);

        s.b[2674] = (2.0 == 4.0);
        s.store_scalar(2674, if s.b[2674] { 1.0 } else { 0.0 });

        let (assign72170_e109007,) = {
    if (((((((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2667]) && s.b[2669]) && s.b[2670]) && s.b[2671]) && (!s.b[2672])) && (!s.b[2673])) && s.b[2674]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, assign72170_e109007);

        s.b[2675] = (2.0 == 8.0);
        s.store_scalar(2675, if s.b[2675] { 1.0 } else { 0.0 });

        let (assign72190_e109036,) = {
    if ((((((((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2667]) && s.b[2669]) && s.b[2670]) && s.b[2671]) && (!s.b[2672])) && (!s.b[2673])) && (!s.b[2674])) && s.b[2675]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, assign72190_e109036);

        let (assign72200_e109051,) = {
    if ((((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2667]) && s.b[2669]) && s.b[2670]) && s.b[2671]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, assign72200_e109051);

        let mut assign72210_loop_guard: usize = 0;
        while {
            let assign72210_cond_e109067: f64 = if (((((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2667]) && s.b[2669]) && s.b[2670]) && s.b[2671]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign72210_cond_e109067 != 0.0
        } {
            assign72210_loop_guard += 1;
            assert!(assign72210_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2667]) && s.b[2669]) && s.b[2670]) && s.b[2671]) {
                s.store_sqrt(726, 726);
            }
            let (assign72210_body1_e109100,) = {
    if ((((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2667]) && s.b[2669]) && s.b[2670]) && s.b[2671]) {
        let assign72210_body1_e109098: f64 = (s.v[719] + 1.0);
        (assign72210_body1_e109098,)
    } else {
        (s.v[719],)
    }
};
            s.store_scalar(719, assign72210_body1_e109100);
        }

        if ((((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2667]) && s.b[2669]) && s.b[2670]) && (!s.b[2671])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }

        if (((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2667]) && s.b[2669]) && s.b[2670]) {
            s.store_div_from_scalar(726, 1.0, 726);
        }

    }

    pub(super) fn stamp_transient_block_72(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2667]) && s.b[2669]) && s.b[2670]) {
            s.store_mul3_affine_lhs(780, 781, 446, 0.2, 0.0, 726);
            s.store_div_scaled_product3_indices(335, 446, 725, 726, 0.2, 770, 1.0);
            s.store_add_scaled_inputs3_indices(116, 446, 1.0, 446, (-0.2), 780, 1.0);
        }

        if (((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2667]) && s.b[2669]) && s.b[2670]) {
        }

        if (((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2667]) && s.b[2669]) && (!s.b[2670])) {
            s.copy_ad(116, 445);
            s.store_scalar(335, 1.0);
        }

        if ((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2667]) && (!s.b[2669])) {
            if (s.v[445] <= s.v[446]) {
                s.copy_ad(116, 445);
            } else {
                s.copy_ad(116, 446);
            }
        }

        s.b[2676] = (p.p33 == 1.0);
        s.store_scalar(2676, if s.b[2676] { 1.0 } else { 0.0 });

        if (((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2676]) {
            s.store_add_scaled_product_indices(404, 397, (-1.0), 116, 155, 1.0);
        }

        s.b[2677] = (s.v[411] > 0.0);
        s.store_scalar(2677, if s.b[2677] { 1.0 } else { 0.0 });

        if ((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2676]) && s.b[2677]) {
            s.store_sub_from_scalar(336, p.p334, 411);
        }

        if ((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2676]) && (!s.b[2677])) {
            s.store_sqrt_offset_square_offset(782, 729, p.p137, ((4.0 * 0.1) * 0.1));
            s.store_scaled_offset_ad(343, A::div_scaled_offset_numerator(s.ad_value(729), 1.0, p.p137, s.ad_value(782), 1.0), 1.0, 0.5);
            s.store_scaled_add_ad_lhs(336, A::offset(s.ad_value(729), p.p137), 782, 0.5);
        }

        s.b[2678] = (s.v[336] < 0.0);
        s.store_scalar(2678, if s.b[2678] { 1.0 } else { 0.0 });

        if (((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2676]) && (!s.b[2677])) && s.b[2678]) {
            s.store_scalar(336, 0.0);
            s.store_scalar(343, 0.0);
        }

        if ((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2676]) && (!s.b[2677])) {
            s.store_scaled_sqrt_mul(600, 651, 336, p.p432);
            s.store_sub_from_scalar(336, p.p334, 600);
        }

        if (((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2676]) {
            s.store_sqrt_square_offset(782, 336, ((4.0 * (p.p334 * 0.01)) * (p.p334 * 0.01)));
            s.store_offset_scaled_div(343, 336, 782, 0.5, 0.5);
            s.store_scaled_add(336, 336, 782, 0.5);
        }

        s.b[2679] = (s.v[336] < 0.0);
        s.store_scalar(2679, if s.b[2679] { 1.0 } else { 0.0 });

        if ((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2676]) && s.b[2679]) {
            s.store_scalar(336, 0.0);
            s.store_scalar(343, 0.0);
        }

        if (((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2676]) {
            s.copy_ad(386, 336);
            s.store_mul3_affine_lhs(418, 2639, 386, (0.5 * 9662367879.197212), 0.0, 386);
            s.store_sqrt_mul_scaled_lhs(334, 154, 2.0, 418);
            s.store_scaled_add_ad(335, A::limited_exp(s.ad_value(334)), A::limited_exp_scaled_input(s.ad_value(334), -1.0), 0.5);
            s.store_div_ln_lhs(419, 335, 418);
            s.store_add(414, 404, 397);
            s.store_mul_sub_rhs(333, 419, 414, 418);
        }

        s.b[2680] = (s.v[333] < 60.0);
        s.store_scalar(2680, if s.b[2680] { 1.0 } else { 0.0 });

        if ((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2676]) && s.b[2680]) {
            s.store_exp(335, 333);
            s.store_exp_mul_scaled_lhs_indices(334, 419, -1.0, 418);
            s.store_sub(336, 335, 334);
            s.store_div_ln_offset_lhs(416, 336, 1.0, 419);
        }

        if ((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2676]) && (!s.b[2680])) {
            s.store_sub(416, 414, 418);
        }

        if (((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2676]) {
            s.store_mul(415, 154, 416);
        }

        s.b[2681] = ((s.v[415] > (s.v[116] / 100.0)) && (s.v[415] > 0.0));
        s.store_scalar(2681, if s.b[2681] { 1.0 } else { 0.0 });

        let (assign72640_e109689,) = {
    if ((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2676]) && s.b[2681]) {
        let assign72640_e109687: f64 = (s.v[2645] + 1.0);
        (assign72640_e109687,)
    } else {
        (s.v[2645],)
    }
};
        s.store_scalar(2645, assign72640_e109689);

        if ((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2676]) && s.b[2681]) {
            s.copy_ad(116, 447);
        }

        if ((s.v[2623] != 0.0) && (!s.b[2663])) {
            s.store_add_scaled_product_indices(404, 397, (-1.0), 116, 155, 1.0);
        }

        s.b[2682] = (((s.v[116]) as f64).abs() > 1e-6);
        s.store_scalar(2682, if s.b[2682] { 1.0 } else { 0.0 });

        if (((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2682]) {
            s.store_add_offset_lhs_ad_rhs(335, 116, (-1.0), A::exp_scaled_input(s.ad_value(116), -1.0));
            s.store_sqrt(336, 335);
        }

        if (((s.v[2623] != 0.0) && (!s.b[2663])) && (!s.b[2682])) {
            s.store_mul_scaled_sqrt_ad_rhs(336, 116, 0.7071067811865475, A::sub_from_scalar(1.0, A::scale(s.ad_value(116), 0.3333333333333333)));
        }

        if ((s.v[2623] != 0.0) && (!s.b[2663])) {
            s.store_mul(354, 410, 336);
            s.store_mul_sub_rhs(398, 413, 402, 404);
            s.store_div(2683, 354, 2639);
        }

        s.b[2685] = (p.p33 == 2.0);
        s.store_scalar(2685, if s.b[2685] { 1.0 } else { 0.0 });

        s.b[2686] = ((s.v[2683] > (s.v[386] - (s.v[386] * 0.1))) && ((s.v[386] * 0.1) >= 0.0));
        s.store_scalar(2686, if s.b[2686] { 1.0 } else { 0.0 });

        if ((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2685]) && s.b[2686]) {
            s.store_add_scaled_inputs3_indices(781, 2683, 1.0, 386, (-1.0), 386, 0.1);
            s.store_square(722, 781);
            s.store_scaled_mul(723, 386, 386, (0.1 * 0.1));
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
        }

        let (assign72820_e109896,) = {
    if ((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2685]) && s.b[2686]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, assign72820_e109896);

        let (assign72830_e109907,) = {
    if ((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2685]) && s.b[2686]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, assign72830_e109907);

        if ((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2685]) && s.b[2686]) {
            s.store_scalar(770, 0.0);
            s.store_scalar(726, 0.0);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_add(770, 724, 725);
            s.copy_ad(726, 770);
        }

        s.b[2687] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.store_scalar(2687, if s.b[2687] { 1.0 } else { 0.0 });

        s.b[2688] = (2.0 == 1.0);
        s.store_scalar(2688, if s.b[2688] { 1.0 } else { 0.0 });

        let (assign72940_e110038,) = {
    if ((((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2685]) && s.b[2686]) && s.b[2687]) && s.b[2688]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, assign72940_e110038);

        s.b[2689] = (2.0 == 2.0);
        s.store_scalar(2689, if s.b[2689] { 1.0 } else { 0.0 });

        let (assign72960_e110059,) = {
    if (((((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2685]) && s.b[2686]) && s.b[2687]) && (!s.b[2688])) && s.b[2689]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, assign72960_e110059);

        s.b[2690] = (2.0 == 4.0);
        s.store_scalar(2690, if s.b[2690] { 1.0 } else { 0.0 });

        let (assign72980_e110083,) = {
    if ((((((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2685]) && s.b[2686]) && s.b[2687]) && (!s.b[2688])) && (!s.b[2689])) && s.b[2690]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, assign72980_e110083);

        s.b[2691] = (2.0 == 8.0);
        s.store_scalar(2691, if s.b[2691] { 1.0 } else { 0.0 });

        let (assign73000_e110110,) = {
    if (((((((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2685]) && s.b[2686]) && s.b[2687]) && (!s.b[2688])) && (!s.b[2689])) && (!s.b[2690])) && s.b[2691]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, assign73000_e110110);

        let (assign73010_e110123,) = {
    if (((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2685]) && s.b[2686]) && s.b[2687]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, assign73010_e110123);

        let mut assign73020_loop_guard: usize = 0;
        while {
            let assign73020_cond_e110137: f64 = if ((((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2685]) && s.b[2686]) && s.b[2687]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign73020_cond_e110137 != 0.0
        } {
            assign73020_loop_guard += 1;
            assert!(assign73020_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2685]) && s.b[2686]) && s.b[2687]) {
                s.store_sqrt(726, 726);
            }
            let (assign73020_body1_e110166,) = {
    if (((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2685]) && s.b[2686]) && s.b[2687]) {
        let assign73020_body1_e110164: f64 = (s.v[719] + 1.0);
        (assign73020_body1_e110164,)
    } else {
        (s.v[719],)
    }
};
            s.store_scalar(719, assign73020_body1_e110166);
        }

        if (((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2685]) && s.b[2686]) && (!s.b[2687])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }

        if ((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2685]) && s.b[2686]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_mul3_affine_lhs(780, 781, 386, 0.1, 0.0, 726);
            s.store_div_scaled_product3_indices(334, 386, 725, 726, 0.1, 770, 1.0);
            s.store_add_scaled_inputs3_indices(335, 386, 1.0, 386, (-0.1), 780, 1.0);
        }

        if ((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2685]) && s.b[2686]) {
        }

        if ((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2685]) && (!s.b[2686])) {
            s.copy_ad(335, 2683);
            s.store_scalar(334, 1.0);
        }

        s.b[2692] = (s.v[334] < 1.0);
        s.store_scalar(2692, if s.b[2692] { 1.0 } else { 0.0 });

        let (assign73120_e110308,) = {
    if ((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2685]) && s.b[2692]) {
        let assign73120_e110306: f64 = (s.v[2645] + 2.0);
        (assign73120_e110306,)
    } else {
        (s.v[2645],)
    }
};
        s.store_scalar(2645, assign73120_e110308);

        if (((s.v[2623] != 0.0) && (!s.b[2663])) && (!s.b[2685])) {
            if (s.v[2683] <= s.v[386]) {
                s.copy_ad(335, 2683);
            } else {
                s.copy_ad(335, 386);
            }
        }

        s.b[2693] = (s.v[2683] >= s.v[386]);
        s.store_scalar(2693, if s.b[2693] { 1.0 } else { 0.0 });

        let (assign73150_e110340,) = {
    if ((((s.v[2623] != 0.0) && (!s.b[2663])) && (!s.b[2685])) && s.b[2693]) {
        let assign73150_e110338: f64 = (s.v[2645] + 2.0);
        (assign73150_e110338,)
    } else {
        (s.v[2645],)
    }
};
        s.store_scalar(2645, assign73150_e110340);

        s.b[2694] = (s.v[2645] >= 2.0);
        s.store_scalar(2694, if s.b[2694] { 1.0 } else { 0.0 });

        if (((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2694]) {
            s.copy_ad(2684, 404);
            s.store_mul(354, 335, 2639);
            s.store_sub_div_rhs_indices(404, 402, 354, 413);
        }

        s.b[2695] = (p.p33 == 2.0);
        s.store_scalar(2695, if s.b[2695] { 1.0 } else { 0.0 });

        s.b[2696] = ((s.v[404] > (s.v[2684] - 0.1)) && (0.1 >= 0.0));
        s.store_scalar(2696, if s.b[2696] { 1.0 } else { 0.0 });

        if (((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2694]) && s.b[2695]) && s.b[2696]) {
            s.store_offset_sub(781, 404, 2684, 0.1);
            s.store_square(722, 781);
            s.store_scalar(723, (0.1 * 0.1));
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
        }

        let (assign73270_e110474,) = {
    if (((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2694]) && s.b[2695]) && s.b[2696]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, assign73270_e110474);

        let (assign73280_e110487,) = {
    if (((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2694]) && s.b[2695]) && s.b[2696]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, assign73280_e110487);

        if (((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2694]) && s.b[2695]) && s.b[2696]) {
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
        if (((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2694]) && s.b[2695]) && s.b[2696]) {
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_add(770, 724, 725);
            s.copy_ad(726, 770);
        }

        s.b[2697] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.store_scalar(2697, if s.b[2697] { 1.0 } else { 0.0 });

        s.b[2698] = (2.0 == 1.0);
        s.store_scalar(2698, if s.b[2698] { 1.0 } else { 0.0 });

        let (assign73390_e110636,) = {
    if (((((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2694]) && s.b[2695]) && s.b[2696]) && s.b[2697]) && s.b[2698]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, assign73390_e110636);

        s.b[2699] = (2.0 == 2.0);
        s.store_scalar(2699, if s.b[2699] { 1.0 } else { 0.0 });

        let (assign73410_e110659,) = {
    if ((((((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2694]) && s.b[2695]) && s.b[2696]) && s.b[2697]) && (!s.b[2698])) && s.b[2699]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, assign73410_e110659);

        s.b[2700] = (2.0 == 4.0);
        s.store_scalar(2700, if s.b[2700] { 1.0 } else { 0.0 });

        let (assign73430_e110685,) = {
    if (((((((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2694]) && s.b[2695]) && s.b[2696]) && s.b[2697]) && (!s.b[2698])) && (!s.b[2699])) && s.b[2700]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, assign73430_e110685);

        s.b[2701] = (2.0 == 8.0);
        s.store_scalar(2701, if s.b[2701] { 1.0 } else { 0.0 });

        let (assign73450_e110714,) = {
    if ((((((((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2694]) && s.b[2695]) && s.b[2696]) && s.b[2697]) && (!s.b[2698])) && (!s.b[2699])) && (!s.b[2700])) && s.b[2701]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, assign73450_e110714);

        let (assign73460_e110729,) = {
    if ((((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2694]) && s.b[2695]) && s.b[2696]) && s.b[2697]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, assign73460_e110729);

        let mut assign73470_loop_guard: usize = 0;
        while {
            let assign73470_cond_e110745: f64 = if (((((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2694]) && s.b[2695]) && s.b[2696]) && s.b[2697]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign73470_cond_e110745 != 0.0
        } {
            assign73470_loop_guard += 1;
            assert!(assign73470_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2694]) && s.b[2695]) && s.b[2696]) && s.b[2697]) {
                s.store_sqrt(726, 726);
            }
            let (assign73470_body1_e110778,) = {
    if ((((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2694]) && s.b[2695]) && s.b[2696]) && s.b[2697]) {
        let assign73470_body1_e110776: f64 = (s.v[719] + 1.0);
        (assign73470_body1_e110776,)
    } else {
        (s.v[719],)
    }
};
            s.store_scalar(719, assign73470_body1_e110778);
        }

        if ((((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2694]) && s.b[2695]) && s.b[2696]) && (!s.b[2697])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }

        if (((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2694]) && s.b[2695]) && s.b[2696]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, 0.1);
            s.store_div_scaled_product_indices(334, 725, 726, 0.1, 770, 1.0);
            s.store_add_offset_lhs(404, 2684, (-0.1), 780);
        }

        if (((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2694]) && s.b[2695]) && s.b[2696]) {
        }

        if (((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2694]) && s.b[2695]) && (!s.b[2696])) {
        }

        if (((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2694]) && s.b[2695]) && (!s.b[2696])) {
            s.store_scalar(334, 1.0);
        }

        if ((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2694]) && (!s.b[2695])) {
            if (s.v[404] <= s.v[2684]) {
            } else {
                s.copy_ad(404, 2684);
            }
        }

        if ((s.v[2623] != 0.0) && (!s.b[2663])) {
            s.copy_ad(2646, 404);
        }

        s.b[2702] = (p.p33 == 1.0);
        s.store_scalar(2702, if s.b[2702] { 1.0 } else { 0.0 });

        let (assign73590_e110950,) = {
    if (((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2702]) {
        (0.0,)
    } else {
        (s.v[79],)
    }
};
        s.store_scalar(79, assign73590_e110950);

        if (((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2702]) {
            s.store_sqrt_scaled_input_ad(982, A::mul(A::div_from_scalar(1.034943e-10, s.ad_value(2639)), s.ad_value(155)), 2.0);
        }

        s.b[2703] = (s.v[411] > 0.0);
        s.store_scalar(2703, if s.b[2703] { 1.0 } else { 0.0 });

        if ((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2702]) && s.b[2703]) {
            s.store_sub_from_scalar(336, p.p334, 411);
        }

        if ((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2702]) && (!s.b[2703])) {
            s.store_sqrt_offset_square_offset(782, 729, p.p137, ((4.0 * 0.1) * 0.1));
            s.store_scaled_offset_ad(343, A::div_scaled_offset_numerator(s.ad_value(729), 1.0, p.p137, s.ad_value(782), 1.0), 1.0, 0.5);
            s.store_scaled_add_ad_lhs(336, A::offset(s.ad_value(729), p.p137), 782, 0.5);
        }

        s.b[2704] = (s.v[336] < 0.0);
        s.store_scalar(2704, if s.b[2704] { 1.0 } else { 0.0 });

        if (((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2702]) && (!s.b[2703])) && s.b[2704]) {
            s.store_scalar(336, 0.0);
            s.store_scalar(343, 0.0);
        }

        if ((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2702]) && (!s.b[2703])) {
            s.store_scaled_sqrt_mul(600, 651, 336, p.p432);
            s.store_sub_from_scalar(336, p.p334, 600);
        }

        if (((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2702]) {
            s.store_sqrt_square_offset(782, 336, ((4.0 * (p.p334 * 0.01)) * (p.p334 * 0.01)));
            s.store_offset_scaled_div(343, 336, 782, 0.5, 0.5);
            s.store_scaled_add(336, 336, 782, 0.5);
        }

        s.b[2705] = (s.v[336] < 0.0);
        s.store_scalar(2705, if s.b[2705] { 1.0 } else { 0.0 });

        if ((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2702]) && s.b[2705]) {
            s.store_scalar(336, 0.0);
            s.store_scalar(343, 0.0);
        }

        if (((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2702]) {
            s.copy_ad(386, 336);
            s.store_mul3_affine_lhs(418, 2639, 386, (0.5 * 9662367879.197212), 0.0, 386);
            s.store_sqrt_mul_scaled_lhs(334, 154, 2.0, 418);
            s.store_scaled_add_ad(335, A::limited_exp(s.ad_value(334)), A::limited_exp_scaled_input(s.ad_value(334), -1.0), 0.5);
            s.store_div_ln_lhs(419, 335, 418);
        }

        let (assign73820_e111259,) = {
    if (((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2702]) {
        (1.0,)
    } else {
        (s.v[97],)
    }
};
        s.store_scalar(97, assign73820_e111259);

        let mut assign73830_loop_guard: usize = 0;
        while {
            let assign73830_cond_e111269: f64 = (s.v[421] + 1.0);
            let assign73830_cond_e111271: f64 = if ((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2702]) && (s.v[97] <= assign73830_cond_e111269)) { 1.0 } else { 0.0 };
            assign73830_cond_e111271 != 0.0
        } {
            assign73830_loop_guard += 1;
            assert!(assign73830_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2702]) {
                s.store_add(414, 404, 397);
                s.store_mul(116, 154, 414);
                s.store_mul_sub_rhs(333, 419, 414, 418);
            }
            s.b[2707] = (s.v[333] < 60.0);
            s.store_scalar(2707, if s.b[2707] { 1.0 } else { 0.0 });
            if ((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2702]) && s.b[2707]) {
                s.store_exp(335, 333);
                s.store_exp_mul_scaled_lhs_indices(334, 419, -1.0, 418);
                s.store_sub(336, 335, 334);
                s.store_div_ln_offset_lhs(416, 336, 1.0, 419);
                s.store_div_scaled_value_offset_denominator(417, s.ad_value(335), 1.0, s.ad_value(336), 1.0, 1.0);
            }
            if ((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2702]) && (!s.b[2707])) {
                s.store_sub(416, 414, 418);
                s.store_scalar(417, 1.0);
            }
            if (((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2702]) {
                s.store_mul(415, 154, 416);
            }
            s.b[2708] = (s.v[116] < 0.0);
            s.store_scalar(2708, if s.b[2708] { 1.0 } else { 0.0 });
            if ((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2702]) && s.b[2708]) {
                s.store_scalar(334, (-0.7071067811865475));
                s.store_mul(223, 116, 334);
                s.store_mul(420, 154, 334);
            }
            s.b[2709] = (s.v[116] < 1e-6);
            s.store_scalar(2709, if s.b[2709] { 1.0 } else { 0.0 });
            if (((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2702]) && (!s.b[2708])) && s.b[2709]) {
                s.store_square_mul_sub_from_scalar_double_scaled_sub_rhs_scaled_output(334, 116, 1.0, 116, 1.0, 116, 1.0, 116, 0.2, 1.0 / (4.0), 1.0 / (3.0), 1.0 / (2.0));
                s.store_mul_sub_from_scalar_ad_rhs(335, 116, 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(116), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(116), 1.0, A::scale(s.ad_value(116), 0.25), 1.0 / (3.0)), 1.0 / (2.0)));
                s.store_square_mul_sub_from_scalar_double_scaled_sub_rhs_scaled_output(336, 415, 1.0, 415, 1.0, 415, 1.0, 415, 0.2, 1.0 / (4.0), 1.0 / (3.0), 1.0 / (2.0));
                s.store_mul_sub_from_scalar_ad_rhs(337, 415, 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(415), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(415), 1.0, A::scale(s.ad_value(415), 0.25), 1.0 / (3.0)), 1.0 / (2.0)));
                s.store_sub(338, 334, 336);
            }
            s.b[2710] = (s.v[338] > 0.0);
            s.store_scalar(2710, if s.b[2710] { 1.0 } else { 0.0 });
            if ((((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2702]) && (!s.b[2708])) && s.b[2709]) && s.b[2710]) {
                s.store_sqrt(223, 338);
                s.store_div_scaled_product_right_ad(420, 154, A::add_scaled_product(s.ad_value(335), 1.0, s.ad_value(417), s.ad_value(337), (-1.0)), 0.5, 223, 1.0);
            }
            if ((((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2702]) && (!s.b[2708])) && s.b[2709]) && (!s.b[2710])) {
                s.store_scalar(223, 0.0);
                s.store_scalar(420, 0.0);
            }
            if (((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2702]) && (!s.b[2708])) && (!s.b[2709])) {
                s.store_exp_neg_input(334, 116);
                s.store_exp_neg_input(335, 415);
                s.store_add_scaled_inputs4_indices(338, 116, 1.0, 415, (-1.0), 334, 1.0, 335, (-1.0));
            }
            s.b[2711] = (s.v[338] > 0.0);
            s.store_scalar(2711, if s.b[2711] { 1.0 } else { 0.0 });
            if ((((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2702]) && (!s.b[2708])) && (!s.b[2709])) && s.b[2711]) {
                s.store_sqrt(223, 338);
                s.store_div_scaled_product_right_ad(420, 154, A::sub(A::sub_from_scalar(1.0, s.ad_value(334)), A::mul_sub_from_scalar_rhs(s.ad_value(417), 1.0, s.ad_value(335))), 0.5, 223, 1.0);
            }
            if ((((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2702]) && (!s.b[2708])) && (!s.b[2709])) && (!s.b[2711])) {
                s.store_scalar(223, 0.0);
                s.store_scalar(420, 0.0);
            }
            s.b[2712] = (s.v[116] < 0.0);
            s.store_scalar(2712, if s.b[2712] { 1.0 } else { 0.0 });
            if ((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2702]) && s.b[2712]) {
                s.store_scalar(214, 0.0);
                s.store_scalar(215, 0.0);
                s.store_neg(216, 223);
                s.store_neg(217, 420);
            }
            s.b[2713] = (s.v[116] < 60.0);
            s.store_scalar(2713, if s.b[2713] { 1.0 } else { 0.0 });
            s.b[2714] = (s.v[116] < 5e-5);
            s.store_scalar(2714, if s.b[2714] { 1.0 } else { 0.0 });
            if ((((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2702]) && (!s.b[2712])) && s.b[2713]) && s.b[2714]) {
                s.store_scaled_mul_ad(334, A::square(s.ad_value(116)), A::offset(A::mul_scaled_lhs(s.ad_value(116), 1.0 / (3.0), A::offset(A::mul_scaled_lhs(s.ad_value(116), 1.0 / (4.0), A::scale_offset(s.ad_value(116), 0.2, 1.0)), 1.0)), 1.0), 1.0 / (2.0));
                s.store_mul_offset_ad_rhs(335, 116, A::mul_scaled_lhs(s.ad_value(116), 1.0 / (2.0), A::offset(A::mul_scaled_lhs(s.ad_value(116), 1.0 / (3.0), A::scale_offset(s.ad_value(116), 0.25, 1.0)), 1.0)), 1.0);
                s.store_mul(214, 222, 334);
                s.store_mul3_lhs(215, 222, 335, 154);
            }
            if ((((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2702]) && (!s.b[2712])) && s.b[2713]) && (!s.b[2714])) {
                s.store_exp(227, 116);
                s.store_offset(335, 227, (-1.0));
                s.store_mul_sub_rhs(214, 222, 335, 116);
                s.store_mul3_lhs(215, 222, 154, 335);
            }
            if (((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2702]) && (!s.b[2712])) && (!s.b[2713])) {
                s.store_exp_mul(231, 154, 404);
                s.store_mul_ad_rhs(214, 405, A::add_scaled_offset_product_rhs(s.ad_value(231), 1.0, s.ad_value(229), s.ad_value(116), 1.0, (-1.0)));
                s.store_mul_ad_product_rhs_mixed_ia(215, 405, 154, A::sub(s.ad_value(231), s.ad_value(229)));
            }
            s.b[2715] = (s.v[214] > 0.0);
            s.store_scalar(2715, if s.b[2715] { 1.0 } else { 0.0 });
            if (((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2702]) && (!s.b[2712])) && s.b[2715]) {
                s.store_sqrt_square_add(216, 223, 214);
                s.store_div_scaled_add_product(217, s.ad_value(215), 0.5, s.ad_value(420), s.ad_value(223), (2.0 * 0.5), s.ad_value(216), 1.0);
            }
            if (((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2702]) && (!s.b[2712])) && (!s.b[2715])) {
                s.copy_ad(216, 223);
                s.copy_ad(217, 420);
            }
            if (((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2702]) {
                s.store_add_scaled_inputs_product_indices(232, 404, 1.0, 402, (-1.0), 212, 216, 1.0);
                s.store_offset_mul(233, 212, 217, 1.0);
            }
            s.b[2716] = (s.v[79] == 1.0);
            s.store_scalar(2716, if s.b[2716] { 1.0 } else { 0.0 });
            let (assign73830_body72_e112417,) = {
    if ((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2702]) && s.b[2716]) {
        let assign73830_body72_e112415: f64 = (s.v[421] + 1.0);
        (assign73830_body72_e112415,)
    } else {
        (s.v[97],)
    }
};
            s.store_scalar(97, assign73830_body72_e112417);
            if ((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2702]) && (!s.b[2716])) {
                s.store_div_scaled_inputs_indices(236, 232, -1.0, 233, 1.0);
            }
            if ((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2702]) && (!s.b[2716])) {
                s.store_scaled_offset_ad(93, {
                    if (1.0 >= ((s.v[404]) as f64).abs()) {
                        A::constant(1.0)
                    } else {
                        A::abs(s.ad_value(404))
                    }
                }, 1.0, (0.5 * 0.1));
            }
            s.b[2717] = (((s.v[236]) as f64).abs() > s.v[93]);
            s.store_scalar(2717, if s.b[2717] { 1.0 } else { 0.0 });
            if (((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2702]) && (!s.b[2716])) && s.b[2717]) {
                s.store_scale(236, 93, (if (s.v[236] >= 0.0) { 1.0 } else { (-1.0) }));
            }
            if ((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2702]) && (!s.b[2716])) {
                s.store_add(404, 404, 236);
            }
            s.b[2718] = ((((s.v[236]) as f64).abs() <= 1e-12) && (((s.v[232]) as f64).abs() <= 1e-8));
            s.store_scalar(2718, if s.b[2718] { 1.0 } else { 0.0 });
            let (assign73830_body79_e112520,) = {
    if (((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2702]) && (!s.b[2716])) && s.b[2718]) {
        (1.0,)
    } else {
        (s.v[79],)
    }
};
            s.store_scalar(79, assign73830_body79_e112520);
            let (assign73830_body80_e112531,) = {
    if (((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2702]) {
        let assign73830_body80_e112529: f64 = (s.v[97] + 1.0);
        (assign73830_body80_e112529,)
    } else {
        (s.v[97],)
    }
};
            s.store_scalar(97, assign73830_body80_e112531);
        }

        if (((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2702]) {
            s.store_mul(2637, 982, 223);
            s.store_mul(2638, 2639, 2637);
            s.store_offset_div(100, 2638, 410, (10.0 * 2.220446049250313e-16));
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
        s.b[2720] = (p.p33 == 4.0);
        s.store_scalar(2720, if s.b[2720] { 1.0 } else { 0.0 });

        if ((s.v[2623] != 0.0) && s.b[2720]) {
            s.store_exp_ad(229, A::mul_scaled_rhs(s.ad_value(154), s.ad_value(397), -1.0));
            s.store_div(334, 394, 409);
            s.store_square(405, 334);
            s.store_mul(222, 405, 229);
            s.copy_ad(404, 2646);
        }

        let (assign73980_e112668,) = {
    if ((s.v[2623] != 0.0) && s.b[2720]) {
        (0.0,)
    } else {
        (s.v[79],)
    }
};
        s.store_scalar(79, assign73980_e112668);

        if ((s.v[2623] != 0.0) && s.b[2720]) {
            s.store_sqrt_scaled_input_ad(982, A::mul(A::div_from_scalar(1.034943e-10, s.ad_value(2639)), s.ad_value(155)), 2.0);
        }

        s.b[2721] = (s.v[411] > 0.0);
        s.store_scalar(2721, if s.b[2721] { 1.0 } else { 0.0 });

        if (((s.v[2623] != 0.0) && s.b[2720]) && s.b[2721]) {
            s.store_sub_from_scalar(336, p.p334, 411);
        }

        if (((s.v[2623] != 0.0) && s.b[2720]) && (!s.b[2721])) {
            s.store_sqrt_offset_square_offset(782, 729, p.p137, ((4.0 * 0.1) * 0.1));
            s.store_scaled_offset_ad(343, A::div_scaled_offset_numerator(s.ad_value(729), 1.0, p.p137, s.ad_value(782), 1.0), 1.0, 0.5);
            s.store_scaled_add_ad_lhs(336, A::offset(s.ad_value(729), p.p137), 782, 0.5);
        }

        s.b[2722] = (s.v[336] < 0.0);
        s.store_scalar(2722, if s.b[2722] { 1.0 } else { 0.0 });

        if ((((s.v[2623] != 0.0) && s.b[2720]) && (!s.b[2721])) && s.b[2722]) {
            s.store_scalar(336, 0.0);
            s.store_scalar(343, 0.0);
        }

        if (((s.v[2623] != 0.0) && s.b[2720]) && (!s.b[2721])) {
            s.store_scaled_sqrt_mul(600, 651, 336, p.p432);
            s.store_sub_from_scalar(336, p.p334, 600);
        }

        if ((s.v[2623] != 0.0) && s.b[2720]) {
            s.store_sqrt_square_offset(782, 336, ((4.0 * (p.p334 * 0.01)) * (p.p334 * 0.01)));
            s.store_offset_scaled_div(343, 336, 782, 0.5, 0.5);
            s.store_scaled_add(336, 336, 782, 0.5);
        }

        s.b[2723] = (s.v[336] < 0.0);
        s.store_scalar(2723, if s.b[2723] { 1.0 } else { 0.0 });

        if (((s.v[2623] != 0.0) && s.b[2720]) && s.b[2723]) {
            s.store_scalar(336, 0.0);
            s.store_scalar(343, 0.0);
        }

        if ((s.v[2623] != 0.0) && s.b[2720]) {
            s.copy_ad(386, 336);
            s.store_mul3_affine_lhs(418, 2639, 386, (0.5 * 9662367879.197212), 0.0, 386);
            s.store_sqrt_mul_scaled_lhs(334, 154, 2.0, 418);
            s.store_scaled_add_ad(335, A::limited_exp(s.ad_value(334)), A::limited_exp_scaled_input(s.ad_value(334), -1.0), 0.5);
            s.store_div_ln_lhs(419, 335, 418);
        }

        let (assign74210_e112917,) = {
    if ((s.v[2623] != 0.0) && s.b[2720]) {
        (1.0,)
    } else {
        (s.v[97],)
    }
};
        s.store_scalar(97, assign74210_e112917);

        let mut assign74220_loop_guard: usize = 0;
        while {
            let assign74220_cond_e112924: f64 = (s.v[421] + 1.0);
            let assign74220_cond_e112926: f64 = if (((s.v[2623] != 0.0) && s.b[2720]) && (s.v[97] <= assign74220_cond_e112924)) { 1.0 } else { 0.0 };
            assign74220_cond_e112926 != 0.0
        } {
            assign74220_loop_guard += 1;
            assert!(assign74220_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((s.v[2623] != 0.0) && s.b[2720]) {
                s.store_add(414, 404, 397);
                s.store_mul(116, 154, 414);
                s.store_mul_sub_rhs(333, 419, 414, 418);
            }
            s.b[2725] = (s.v[333] < 60.0);
            s.store_scalar(2725, if s.b[2725] { 1.0 } else { 0.0 });
            if (((s.v[2623] != 0.0) && s.b[2720]) && s.b[2725]) {
                s.store_exp(335, 333);
                s.store_exp_mul_scaled_lhs_indices(334, 419, -1.0, 418);
                s.store_sub(336, 335, 334);
                s.store_div_ln_offset_lhs(416, 336, 1.0, 419);
                s.store_div_scaled_value_offset_denominator(417, s.ad_value(335), 1.0, s.ad_value(336), 1.0, 1.0);
            }
            if (((s.v[2623] != 0.0) && s.b[2720]) && (!s.b[2725])) {
                s.store_sub(416, 414, 418);
                s.store_scalar(417, 1.0);
            }
            if ((s.v[2623] != 0.0) && s.b[2720]) {
                s.store_mul(415, 154, 416);
            }
            s.b[2726] = (((s.v[116]) as f64).abs() < 1e-6);
            s.store_scalar(2726, if s.b[2726] { 1.0 } else { 0.0 });
            if (((s.v[2623] != 0.0) && s.b[2720]) && s.b[2726]) {
                s.store_square_mul_sub_from_scalar_double_scaled_sub_rhs_scaled_output(334, 116, 1.0, 116, 1.0, 116, 1.0, 116, 0.2, 1.0 / (4.0), 1.0 / (3.0), 1.0 / (2.0));
                s.store_mul_sub_from_scalar_ad_rhs(335, 116, 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(116), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(116), 1.0, A::scale(s.ad_value(116), 0.25), 1.0 / (3.0)), 1.0 / (2.0)));
                s.store_square_mul_sub_from_scalar_double_scaled_sub_rhs_scaled_output(336, 415, 1.0, 415, 1.0, 415, 1.0, 415, 0.2, 1.0 / (4.0), 1.0 / (3.0), 1.0 / (2.0));
                s.store_mul_sub_from_scalar_ad_rhs(337, 415, 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(415), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(415), 1.0, A::scale(s.ad_value(415), 0.25), 1.0 / (3.0)), 1.0 / (2.0)));
                s.store_sub(2647, 334, 336);
                s.store_mul_add_scaled_product_rhs(2648, 154, s.ad_value(335), 1.0, s.ad_value(417), s.ad_value(337), (-1.0));
            }
            if (((s.v[2623] != 0.0) && s.b[2720]) && (!s.b[2726])) {
                s.store_exp_neg_input(334, 116);
                s.store_exp_neg_input(335, 415);
                s.store_add_scaled_inputs4_indices(2647, 116, 1.0, 415, (-1.0), 334, 1.0, 335, (-1.0));
                s.store_mul_sub_ad_rhs(2648, 154, A::sub_from_scalar(1.0, s.ad_value(334)), A::mul_sub_from_scalar_rhs(s.ad_value(417), 1.0, s.ad_value(335)));
            }
            s.b[2727] = (((s.v[116]) as f64).abs() < 5e-5);
            s.store_scalar(2727, if s.b[2727] { 1.0 } else { 0.0 });
            if (((s.v[2623] != 0.0) && s.b[2720]) && s.b[2727]) {
                s.store_scaled_mul_ad(334, A::square(s.ad_value(116)), A::offset(A::mul_scaled_lhs(s.ad_value(116), 1.0 / (3.0), A::offset(A::mul_scaled_lhs(s.ad_value(116), 1.0 / (4.0), A::scale_offset(s.ad_value(116), 0.2, 1.0)), 1.0)), 1.0), 1.0 / (2.0));
                s.store_mul_offset_ad_rhs(335, 116, A::mul_scaled_lhs(s.ad_value(116), 1.0 / (2.0), A::offset(A::mul_scaled_lhs(s.ad_value(116), 1.0 / (3.0), A::scale_offset(s.ad_value(116), 0.25, 1.0)), 1.0)), 1.0);
                s.store_mul(214, 222, 334);
                s.store_mul3_lhs(215, 222, 335, 154);
            }
            s.b[2728] = (((s.v[116]) as f64).abs() < 60.0);
            s.store_scalar(2728, if s.b[2728] { 1.0 } else { 0.0 });
            if ((((s.v[2623] != 0.0) && s.b[2720]) && (!s.b[2727])) && s.b[2728]) {
                s.store_exp(227, 116);
                s.store_offset(335, 227, (-1.0));
                s.store_mul_sub_rhs(214, 222, 335, 116);
                s.store_mul3_lhs(215, 222, 154, 335);
            }
            if ((((s.v[2623] != 0.0) && s.b[2720]) && (!s.b[2727])) && (!s.b[2728])) {
                s.store_exp_mul(231, 154, 404);
                s.store_mul_ad_rhs(214, 405, A::add_scaled_offset_product_rhs(s.ad_value(231), 1.0, s.ad_value(229), s.ad_value(116), 1.0, (-1.0)));
                s.store_mul_ad_product_rhs_mixed_ia(215, 405, 154, A::sub(s.ad_value(231), s.ad_value(229)));
            }
            s.b[2729] = (s.v[214] > 0.0);
            s.store_scalar(2729, if s.b[2729] { 1.0 } else { 0.0 });
            if (((s.v[2623] != 0.0) && s.b[2720]) && s.b[2729]) {
                s.store_sqrt_add(216, 2647, 214);
                s.store_div_scaled_inputs2_indices(217, 2648, 0.5, 215, 0.5, 216, 1.0);
            }
            s.b[2730] = (s.v[2647] > 0.0);
            s.store_scalar(2730, if s.b[2730] { 1.0 } else { 0.0 });
            if ((((s.v[2623] != 0.0) && s.b[2720]) && (!s.b[2729])) && s.b[2730]) {
                s.store_sqrt(216, 2647);
                s.store_div_scaled_inputs_indices(217, 2648, 0.5, 216, 1.0);
            }
            if ((((s.v[2623] != 0.0) && s.b[2720]) && (!s.b[2729])) && (!s.b[2730])) {
                s.store_scalar(216, 0.0);
                s.store_scalar(217, 0.0);
            }
            if ((s.v[2623] != 0.0) && s.b[2720]) {
                s.store_scale(216, 216, (if (s.v[116] >= 0.0) { 1.0 } else { (-1.0) }));
            }
            if ((s.v[2623] != 0.0) && s.b[2720]) {
                s.store_scale(217, 217, (if (s.v[116] >= 0.0) { 1.0 } else { (-1.0) }));
            }
            if ((s.v[2623] != 0.0) && s.b[2720]) {
                s.store_add_scaled_inputs_product_indices(232, 404, 1.0, 402, (-1.0), 212, 216, 1.0);
                s.store_offset_mul(233, 212, 217, 1.0);
            }
            s.b[2731] = (s.v[79] > 0.0);
            s.store_scalar(2731, if s.b[2731] { 1.0 } else { 0.0 });
            let (assign74220_body56_e113666,) = {
    if (((s.v[2623] != 0.0) && s.b[2720]) && s.b[2731]) {
        let assign74220_body56_e113664: f64 = (s.v[421] + 1.0);
        (assign74220_body56_e113664,)
    } else {
        (s.v[97],)
    }
};
            s.store_scalar(97, assign74220_body56_e113666);
            if (((s.v[2623] != 0.0) && s.b[2720]) && (!s.b[2731])) {
                s.store_div_scaled_inputs_indices(236, 232, -1.0, 233, 1.0);
            }
            if (((s.v[2623] != 0.0) && s.b[2720]) && (!s.b[2731])) {
                s.store_scaled_offset_ad(93, {
                    if (1.0 >= ((s.v[404]) as f64).abs()) {
                        A::constant(1.0)
                    } else {
                        A::abs(s.ad_value(404))
                    }
                }, 1.0, (0.5 * 0.1));
            }
            s.b[2732] = (((s.v[236]) as f64).abs() > s.v[93]);
            s.store_scalar(2732, if s.b[2732] { 1.0 } else { 0.0 });
            if ((((s.v[2623] != 0.0) && s.b[2720]) && (!s.b[2731])) && s.b[2732]) {
                s.store_scale(236, 93, (if (s.v[236] >= 0.0) { 1.0 } else { (-1.0) }));
            }
            if (((s.v[2623] != 0.0) && s.b[2720]) && (!s.b[2731])) {
                s.store_add(404, 404, 236);
            }
            s.b[2733] = ((((s.v[236]) as f64).abs() <= 1e-12) && (((s.v[232]) as f64).abs() <= 1e-8));
            s.store_scalar(2733, if s.b[2733] { 1.0 } else { 0.0 });
            let (assign74220_body63_e113756,) = {
    if ((((s.v[2623] != 0.0) && s.b[2720]) && (!s.b[2731])) && s.b[2733]) {
        let assign74220_body63_e113754: f64 = (s.v[79] + 2.0);
        (assign74220_body63_e113754,)
    } else {
        (s.v[79],)
    }
};
            s.store_scalar(79, assign74220_body63_e113756);
            let (assign74220_body64_e113764,) = {
    if ((s.v[2623] != 0.0) && s.b[2720]) {
        let assign74220_body64_e113762: f64 = (s.v[97] + 1.0);
        (assign74220_body64_e113762,)
    } else {
        (s.v[97],)
    }
};
            s.store_scalar(97, assign74220_body64_e113764);
        }

        if ((s.v[2623] != 0.0) && s.b[2720]) {
            if (s.v[2647] >= 0.0) {
                s.store_scaled_sqrt(223, 2647, (if (s.v[116] >= 0.0) { 1.0 } else { (-1.0) }));
            } else {
                s.store_scalar(223, 0.0);
            }
        }

        if ((s.v[2623] != 0.0) && s.b[2720]) {
            s.store_mul(2637, 982, 223);
            s.store_mul(2638, 2639, 2637);
            s.store_offset_div(100, 2638, 410, (10.0 * 2.220446049250313e-16));
            s.store_mul(354, 410, 100);
            s.store_div_from_scalar_add_ad(335, 1.0, s.ad_value(216), s.ad_value(100));
            s.store_mul3_lhs(399, 410, 214, 335);
            s.store_add(398, 354, 399);
        }

        if (s.v[2623] != 0.0) {
            s.store_sub(399, 398, 354);
        }

        s.b[2735] = (s.v[407] < 0.0);
        s.store_scalar(2735, if s.b[2735] { 1.0 } else { 0.0 });

        if ((s.v[2623] != 0.0) && s.b[2735]) {
            s.store_neg(407, 407);
        }

        s.b[2736] = (p.p55 == 0.0);
        s.store_scalar(2736, if s.b[2736] { 1.0 } else { 0.0 });

        s.b[2737] = (p.p50 == 0.0);
        s.store_scalar(2737, if s.b[2737] { 1.0 } else { 0.0 });

        if ((((s.v[2623] != 0.0) && s.b[2735]) && s.b[2736]) && s.b[2737]) {
            s.store_neg(2640, 404);
        }

        if ((((s.v[2623] != 0.0) && s.b[2735]) && s.b[2736]) && (!s.b[2737])) {
            s.copy_ad(2640, 396);
        }

        if (((s.v[2623] != 0.0) && s.b[2735]) && s.b[2736]) {
            s.store_sqrt_offset_square_offset(782, 2640, p.p137, ((4.0 * 0.1) * 0.1));
            s.store_scaled_offset_ad(343, A::div_scaled_offset_numerator(s.ad_value(2640), 1.0, p.p137, s.ad_value(782), 1.0), 1.0, 0.5);
            s.store_scaled_add_ad_lhs(336, A::offset(s.ad_value(2640), p.p137), 782, 0.5);
        }

        s.b[2738] = (s.v[336] < 0.0);
        s.store_scalar(2738, if s.b[2738] { 1.0 } else { 0.0 });

        if ((((s.v[2623] != 0.0) && s.b[2735]) && s.b[2736]) && s.b[2738]) {
            s.store_scalar(336, 0.0);
            s.store_scalar(343, 0.0);
        }

        if (((s.v[2623] != 0.0) && s.b[2735]) && s.b[2736]) {
            s.store_scaled_sqrt_mul(600, 651, 336, p.p432);
            s.store_add_scaled_inputs3_indices(781, 407, 1.0, 600, (-1.0), 407, (-0.1));
            s.store_scaled_mul(782, 407, 407, (4.0 * 0.1));
        }

        if (((s.v[2623] != 0.0) && s.b[2735]) && s.b[2736]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if (((s.v[2623] != 0.0) && s.b[2735]) && s.b[2736]) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_add_scaled_inputs3_indices(603, 407, 1.0, 781, (-0.5), 782, (-0.5));
            s.store_sub(407, 407, 603);
        }

        s.b[2739] = (1.0 == 1.0);
        s.store_scalar(2739, if s.b[2739] { 1.0 } else { 0.0 });

        s.b[2740] = (1.0 == 2.0);
        s.store_scalar(2740, if s.b[2740] { 1.0 } else { 0.0 });

        s.b[2741] = (1.0 == 3.0);
        s.store_scalar(2741, if s.b[2741] { 1.0 } else { 0.0 });

        s.b[2742] = (1.0 == 4.0);
        s.store_scalar(2742, if s.b[2742] { 1.0 } else { 0.0 });

        s.b[2743] = (p.p55 == 1.0);
        s.store_scalar(2743, if s.b[2743] { 1.0 } else { 0.0 });

        if (((s.v[2623] != 0.0) && s.b[2739]) && s.b[2743]) {
            s.store_scale(338, 407, s.v[635]);
        }

        if (((s.v[2623] != 0.0) && s.b[2739]) && (!s.b[2743])) {
            s.store_scale(338, 407, (s.v[635] * (1.0 - s.v[526])));
        }

        if ((s.v[2623] != 0.0) && s.b[2739]) {
            s.store_mul(353, 338, 398);
            s.store_mul(356, 338, 354);
        }

        if ((s.v[2623] != 0.0) && (s.b[2740] && (!s.b[2739]))) {
            s.store_scale(338, 407, (s.v[635] * s.v[526]));
            s.store_mul(351, 338, 398);
            s.store_mul(359, 338, 354);
        }

        s.b[2744] = (p.p55 == 1.0);
        s.store_scalar(2744, if s.b[2744] { 1.0 } else { 0.0 });

        if (((s.v[2623] != 0.0) && (s.b[2741] && (!(s.b[2739] || s.b[2740])))) && s.b[2744]) {
            s.store_scale(338, 407, s.v[635]);
        }

        if (((s.v[2623] != 0.0) && (s.b[2741] && (!(s.b[2739] || s.b[2740])))) && (!s.b[2744])) {
            s.store_scale(338, 407, (s.v[635] * (1.0 - s.v[526])));
        }

        if ((s.v[2623] != 0.0) && (s.b[2741] && (!(s.b[2739] || s.b[2740])))) {
            s.copy_ad(697, 404);
        }

        s.b[2745] = (p.p430 == 0.0);
        s.store_scalar(2745, if s.b[2745] { 1.0 } else { 0.0 });

    }

    pub(super) fn stamp_transient_block_75(
        s: &mut Scratch,
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        if (((s.v[2623] != 0.0) && (s.b[2741] && (!(s.b[2739] || s.b[2740])))) && s.b[2745]) {
            s.copy_ad(698, 354);
        }

        if ((s.v[2623] != 0.0) && (s.b[2741] && (!(s.b[2739] || s.b[2740])))) {
            s.store_mul(352, 338, 398);
            s.store_mul(355, 338, 354);
            s.copy_ad(816, 355);
        }

        if ((s.v[2623] != 0.0) && (s.b[2742] && (!((s.b[2739] || s.b[2740]) || s.b[2741])))) {
            s.store_scale(338, 407, (s.v[635] * s.v[526]));
            s.store_mul(350, 338, 398);
            s.store_mul(358, 338, 354);
        }

        s.store_scalar(2623, 0.0);

        s.b[2746] = (2.0 == 1.0);
        s.store_scalar(2746, if s.b[2746] { 1.0 } else { 0.0 });

        s.b[2747] = (2.0 == 2.0);
        s.store_scalar(2747, if s.b[2747] { 1.0 } else { 0.0 });

        s.b[2748] = (2.0 == 3.0);
        s.store_scalar(2748, if s.b[2748] { 1.0 } else { 0.0 });

        s.b[2749] = (2.0 == 4.0);
        s.store_scalar(2749, if s.b[2749] { 1.0 } else { 0.0 });

        s.b[2750] = (((p.p36 == 1.0) && (p.p66 > 0.0)) && (s.v[460] > 0.0));
        s.store_scalar(2750, if s.b[2750] { 1.0 } else { 0.0 });

        let (assign74850_e114357,) = {
    if (s.b[2746] && s.b[2750]) {
        (1.0,)
    } else {
        (s.v[2623],)
    }
};
        s.store_scalar(2623, assign74850_e114357);

        let (assign74860_e114363,) = {
    if (s.b[2746] && s.b[2750]) {
        (1.0,)
    } else {
        (s.v[2621],)
    }
};
        s.store_scalar(2621, assign74860_e114363);

        if (s.b[2746] && s.b[2750]) {
            s.store_sub(395, 731, 728);
            s.store_neg(396, 728);
            s.store_scalar(409, s.v[460]);
            s.store_scalar(407, p.p66);
            s.store_scalar(411, 0.0);
            s.copy_ad(410, 687);
            s.store_scalar(413, s.v[188]);
        }

        s.b[2751] = (((((p.p36 == 1.0) && (p.p66 > 0.0)) && (s.v[460] > 0.0)) && (s.v[526] != 0.0)) && (p.p55 != 1.0));
        s.store_scalar(2751, if s.b[2751] { 1.0 } else { 0.0 });

        let (assign74950_e114436,) = {
    if ((s.b[2747] && (!s.b[2746])) && s.b[2751]) {
        (1.0,)
    } else {
        (s.v[2623],)
    }
};
        s.store_scalar(2623, assign74950_e114436);

        if ((s.b[2747] && (!s.b[2746])) && s.b[2751]) {
            s.store_sub(395, 734, 735);
            s.store_neg(396, 735);
        }

        s.b[2752] = (((p.p35 == 1.0) && (p.p63 > 0.0)) && (s.v[459] > 0.0));
        s.store_scalar(2752, if s.b[2752] { 1.0 } else { 0.0 });

        let (assign74990_e114479,) = {
    if ((s.b[2748] && (!(s.b[2746] || s.b[2747]))) && s.b[2752]) {
        (1.0,)
    } else {
        (s.v[2623],)
    }
};
        s.store_scalar(2623, assign74990_e114479);

        let (assign75000_e114490,) = {
    if ((s.b[2748] && (!(s.b[2746] || s.b[2747]))) && s.b[2752]) {
        (1.0,)
    } else {
        (s.v[2624],)
    }
};
        s.store_scalar(2624, assign75000_e114490);

        if ((s.b[2748] && (!(s.b[2746] || s.b[2747]))) && s.b[2752]) {
            s.store_sub(395, 731, 728);
            s.store_sub(396, 729, 728);
            s.store_scalar(409, s.v[459]);
            s.store_scalar(407, (p.p63 + (p.p64 * p.p55)));
            s.copy_ad(411, 384);
            s.copy_ad(410, 686);
            s.copy_ad(413, 412);
            s.store_neg(407, 407);
        }

        s.b[2753] = (((s.v[407] < 0.0) && (p.p432 > 0.0)) && (p.p55 == 1.0));
        s.store_scalar(2753, if s.b[2753] { 1.0 } else { 0.0 });

        if (((s.b[2748] && (!(s.b[2746] || s.b[2747]))) && s.b[2752]) && s.b[2753]) {
            s.store_neg(407, 407);
            s.store_scalar(335, p.p63);
            s.store_offset_div_scaled_product(996, s.ad_value(335), s.ad_value(335), 1.0, s.ad_value(651), 1.0, (-p.p137));
        }

        s.b[2754] = (p.p113 > 0.0);
        s.store_scalar(2754, if s.b[2754] { 1.0 } else { 0.0 });

        s.b[2755] = ((s.v[396] == 0.0) || (p.p113 <= 0.0));
        s.store_scalar(2755, if s.b[2755] { 1.0 } else { 0.0 });

        if (((((s.b[2748] && (!(s.b[2746] || s.b[2747]))) && s.b[2752]) && s.b[2753]) && s.b[2754]) && s.b[2755]) {
        }

        if (((((s.b[2748] && (!(s.b[2746] || s.b[2747]))) && s.b[2752]) && s.b[2753]) && s.b[2754]) && (!s.b[2755])) {
            s.store_scalar(783, (if (s.v[396] < 0.0) { (-1.0) } else { 1.0 }));
        }

        if (((((s.b[2748] && (!(s.b[2746] || s.b[2747]))) && s.b[2752]) && s.b[2753]) && s.b[2754]) && (!s.b[2755])) {
            s.store_mul(784, 783, 396);
            s.store_offset_powf_ad(781, A::div(s.ad_value(784), s.ad_value(996)), p.p113, 1.0);
            s.store_powf(782, 781, (1.0 / p.p113));
            s.store_div_scaled_product_indices(396, 783, 784, 1.0, 782, 1.0);
        }

        if ((((s.b[2748] && (!(s.b[2746] || s.b[2747]))) && s.b[2752]) && s.b[2753]) && s.b[2754]) {
            s.store_sqrt_offset_square_offset(782, 396, p.p137, ((4.0 * 0.1) * 0.1));
            s.store_scaled_offset_ad(343, A::div_scaled_offset_numerator(s.ad_value(396), 1.0, p.p137, s.ad_value(782), 1.0), 1.0, 0.5);
            s.store_scaled_add_ad_lhs(336, A::offset(s.ad_value(396), p.p137), 782, 0.5);
        }

        s.b[2756] = (s.v[336] < 0.0);
        s.store_scalar(2756, if s.b[2756] { 1.0 } else { 0.0 });

        if (((((s.b[2748] && (!(s.b[2746] || s.b[2747]))) && s.b[2752]) && s.b[2753]) && s.b[2754]) && s.b[2756]) {
            s.store_scalar(336, 0.0);
            s.store_scalar(343, 0.0);
        }

        if ((((s.b[2748] && (!(s.b[2746] || s.b[2747]))) && s.b[2752]) && s.b[2753]) && s.b[2754]) {
            s.store_scaled_sqrt_mul(600, 651, 336, p.p432);
            s.store_sub(407, 407, 600);
        }

        s.b[2757] = (((((p.p35 == 1.0) && (p.p63 > 0.0)) && (s.v[459] > 0.0)) && (s.v[526] != 0.0)) && (p.p55 != 1.0));
        s.store_scalar(2757, if s.b[2757] { 1.0 } else { 0.0 });

        let (assign75300_e114961,) = {
    if ((s.b[2749] && (!((s.b[2746] || s.b[2747]) || s.b[2748]))) && s.b[2757]) {
        (1.0,)
    } else {
        (s.v[2623],)
    }
};
        s.store_scalar(2623, assign75300_e114961);

        if ((s.b[2749] && (!((s.b[2746] || s.b[2747]) || s.b[2748]))) && s.b[2757]) {
            s.store_sub(395, 734, 735);
            s.store_sub(396, 733, 735);
        }

        if (s.v[2623] != 0.0) {
            s.store_scalar(2765, 0.4);
        }

        let (assign75350_e115003,) = {
    if (s.v[2623] != 0.0) {
        (0.0,)
    } else {
        (s.v[2766],)
    }
};
        s.store_scalar(2766, assign75350_e115003);

        if (s.v[2623] != 0.0) {
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

        let (assign75480_e115056,) = {
    if (s.v[2623] != 0.0) {
        let assign75480_e115054: f64 = (-1.0);
        (assign75480_e115054,)
    } else {
        (s.v[79],)
    }
};
        s.store_scalar(79, assign75480_e115056);

        if (s.v[2623] != 0.0) {
            s.store_scalar(2767, 0.0);
            s.store_scalar(2768, 0.0);
            s.store_mul_scaled_ln_ad_rhs(2763, 155, 2.0, A::div(s.ad_value(409), s.ad_value(394)));
            s.store_offset_sub_from_scalar_ad(781, 0.8, s.ad_value(2763), (-0.1));
            s.store_scalar(782, ((4.0 * 0.8) * 0.1));
        }

        if (s.v[2623] != 0.0) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if (s.v[2623] != 0.0) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(2764, 781, (-0.5), 782, (-0.5), 0.8);
        }

        s.b[2770] = (s.v[2765] > (s.v[2764] * 0.5));
        s.store_scalar(2770, if s.b[2770] { 1.0 } else { 0.0 });

        if ((s.v[2623] != 0.0) && s.b[2770]) {
            s.store_scale(2765, 2764, 0.5);
        }

        s.b[2771] = param_given[338];
        s.store_scalar(2771, if s.b[2771] { 1.0 } else { 0.0 });

        if ((s.v[2623] != 0.0) && s.b[2771]) {
            s.store_scalar(2764, p.p338);
        }

        s.b[2772] = param_given[339];
        s.store_scalar(2772, if s.b[2772] { 1.0 } else { 0.0 });

        if ((s.v[2623] != 0.0) && s.b[2772]) {
            s.store_scalar(2765, p.p339);
        }

        s.b[2773] = param_given[338];
        s.store_scalar(2773, if s.b[2773] { 1.0 } else { 0.0 });

        if (((s.v[2623] != 0.0) && (!s.b[2772])) && s.b[2773]) {
            s.store_scale(2765, 2764, 0.5);
        }

        s.b[2774] = (s.v[2765] > (s.v[2764] * 0.5));
        s.store_scalar(2774, if s.b[2774] { 1.0 } else { 0.0 });

        if ((s.v[2623] != 0.0) && s.b[2774]) {
            s.store_scale(2765, 2764, 0.5);
        }

        s.b[2775] = (p.p38 == 1.0);
        s.store_scalar(2775, if s.b[2775] { 1.0 } else { 0.0 });

        if ((s.v[2623] != 0.0) && s.b[2775]) {
            s.store_neg(334, 396);
        }

        s.b[2776] = (s.v[334] > s.v[2765]);
        s.store_scalar(2776, if s.b[2776] { 1.0 } else { 0.0 });

        if (((s.v[2623] != 0.0) && s.b[2775]) && s.b[2776]) {
            s.store_sub(335, 334, 2765);
            s.store_sub(336, 2764, 2765);
            s.store_div(781, 335, 336);
            s.store_square(782, 781);
            s.store_mul(783, 782, 781);
            s.store_square(784, 782);
            s.store_div_from_scalar_ad(780, 1.0, A::add_scaled_inputs4_offset(s.ad_value(781), 1.0, s.ad_value(782), 1.0, s.ad_value(783), 1.0, s.ad_value(784), 1.0, 1.0));
            s.store_mul_ad_affine_product_lhs(345, A::add_scaled_inputs3_offset(s.ad_value(781), 2.0, s.ad_value(782), 3.0, s.ad_value(783), 4.0, 1.0), s.ad_value(780), -1.0, 0.0, 780);
            s.store_mul_sub_from_scalar_rhs(333, 336, 1.0, 780);
            s.store_add_scaled_sub_value_product_indices(334, 1.0, 780, 1.0, 781, 345, 1.0);
            s.store_neg(345, 345);
            s.store_add(344, 2765, 333);
        }

        if (((s.v[2623] != 0.0) && s.b[2775]) && (!s.b[2776])) {
            s.copy_ad(344, 334);
        }

        if ((s.v[2623] != 0.0) && s.b[2775]) {
            s.store_neg(397, 344);
        }

        if ((s.v[2623] != 0.0) && (!s.b[2775])) {
            s.copy_ad(397, 396);
        }

        if (s.v[2623] != 0.0) {
            s.store_div(212, 410, 413);
            s.store_square(213, 212);
            s.store_sub_from_scalar(402, s.v[458], 395);
        }

        let (assign75890_e115397,) = {
    if (s.v[2623] != 0.0) {
        let assign75890_e115391: f64 = (-s.v[397]);
        let assign75890_e115394: f64 = (10.0 * 2.220446049250313e-16);
        let assign75890_e115395: f64 = (assign75890_e115391 + assign75890_e115394);
        (assign75890_e115395,)
    } else {
        (s.v[403],)
    }
};
        s.store_scalar(403, assign75890_e115397);

        if (s.v[2623] != 0.0) {
            s.store_scalar(2759, 0.0);
            s.store_scale(2760, 409, 1.6021918e-19);
        }

    }

    pub(super) fn stamp_transient_block_76(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (s.v[2623] != 0.0) {
            s.store_div(334, 394, 409);
            s.store_square(405, 334);
        }

        s.b[2777] = ((s.v[154] * (-s.v[397])) >= 500.0);
        s.store_scalar(2777, if s.b[2777] { 1.0 } else { 0.0 });

        if ((s.v[2623] != 0.0) && s.b[2777]) {
            s.store_scaled_offset_ad(229, A::mul_scaled_rhs(s.ad_value(154), s.ad_value(397), -1.0), ((1.0) + ((-500.0))), 1.403592217853e217);
            s.store_scalar(334, 1.403592217853e217);
        }

        if ((s.v[2623] != 0.0) && (!s.b[2777])) {
            s.store_mul_neg_rhs(781, 154, 397);
            s.store_scalar(229, 1.0);
        }

        let mut assign75990_loop_guard: usize = 0;
        while {
            let assign75990_cond_e115471: f64 = if (((s.v[2623] != 0.0) && (!s.b[2777])) && (s.v[781] >= 60.0)) { 1.0 } else { 0.0 };
            assign75990_cond_e115471 != 0.0
        } {
            assign75990_loop_guard += 1;
            assert!(assign75990_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((s.v[2623] != 0.0) && (!s.b[2777])) {
                s.store_scale(229, 229, 1.14200738981568e26);
                s.store_offset(781, 781, (-60.0));
            }
        }

        if ((s.v[2623] != 0.0) && (!s.b[2777])) {
            s.store_mul_exp_rhs(229, 229, 781);
            s.copy_ad(334, 229);
        }

        if ((s.v[2623] != 0.0) && (s.v[406] != 0.0)) {
            s.store_offset_scaled(781, 402, (-0.5), (((-0.5)) + ((-1.0))));
            s.store_scalar(782, (4.0 * 0.5));
        }

        if ((s.v[2623] != 0.0) && (s.v[406] != 0.0)) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if ((s.v[2623] != 0.0) && (s.v[406] != 0.0)) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(335, 781, 0.5, 782, 0.5, 0.5);
        }

        s.b[2778] = (((s.v[402] + s.v[397]) > (-s.v[335])) && (s.v[335] >= 0.0));
        s.store_scalar(2778, if s.b[2778] { 1.0 } else { 0.0 });

        if (((s.v[2623] != 0.0) && (s.v[406] != 0.0)) && s.b[2778]) {
            s.store_add_scaled_inputs3_indices(781, 402, 1.0, 397, 1.0, 335, 1.0);
            s.store_square(722, 781);
            s.store_square(723, 335);
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
        }

        let (assign76140_e115645,) = {
    if (((s.v[2623] != 0.0) && (s.v[406] != 0.0)) && s.b[2778]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, assign76140_e115645);

        let (assign76150_e115653,) = {
    if (((s.v[2623] != 0.0) && (s.v[406] != 0.0)) && s.b[2778]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, assign76150_e115653);

        if (((s.v[2623] != 0.0) && (s.v[406] != 0.0)) && s.b[2778]) {
            s.store_scalar(770, 0.0);
            s.store_scalar(726, 0.0);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_add(770, 724, 725);
            s.copy_ad(726, 770);
        }

        s.b[2779] = ((((1.0 == 1.0) || (1.0 == 2.0)) || (1.0 == 4.0)) || (1.0 == 8.0));
        s.store_scalar(2779, if s.b[2779] { 1.0 } else { 0.0 });

        s.b[2780] = (1.0 == 1.0);
        s.store_scalar(2780, if s.b[2780] { 1.0 } else { 0.0 });

        let (assign76240_e115737,) = {
    if (((((s.v[2623] != 0.0) && (s.v[406] != 0.0)) && s.b[2778]) && s.b[2779]) && s.b[2780]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, assign76240_e115737);

        s.b[2781] = (1.0 == 2.0);
        s.store_scalar(2781, if s.b[2781] { 1.0 } else { 0.0 });

        let (assign76260_e115755,) = {
    if ((((((s.v[2623] != 0.0) && (s.v[406] != 0.0)) && s.b[2778]) && s.b[2779]) && (!s.b[2780])) && s.b[2781]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, assign76260_e115755);

        s.b[2782] = (1.0 == 4.0);
        s.store_scalar(2782, if s.b[2782] { 1.0 } else { 0.0 });

        let (assign76280_e115776,) = {
    if (((((((s.v[2623] != 0.0) && (s.v[406] != 0.0)) && s.b[2778]) && s.b[2779]) && (!s.b[2780])) && (!s.b[2781])) && s.b[2782]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, assign76280_e115776);

        s.b[2783] = (1.0 == 8.0);
        s.store_scalar(2783, if s.b[2783] { 1.0 } else { 0.0 });

        let (assign76300_e115800,) = {
    if ((((((((s.v[2623] != 0.0) && (s.v[406] != 0.0)) && s.b[2778]) && s.b[2779]) && (!s.b[2780])) && (!s.b[2781])) && (!s.b[2782])) && s.b[2783]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, assign76300_e115800);

        let (assign76310_e115810,) = {
    if ((((s.v[2623] != 0.0) && (s.v[406] != 0.0)) && s.b[2778]) && s.b[2779]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, assign76310_e115810);

        let mut assign76320_loop_guard: usize = 0;
        while {
            let assign76320_cond_e115821: f64 = if (((((s.v[2623] != 0.0) && (s.v[406] != 0.0)) && s.b[2778]) && s.b[2779]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign76320_cond_e115821 != 0.0
        } {
            assign76320_loop_guard += 1;
            assert!(assign76320_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.v[2623] != 0.0) && (s.v[406] != 0.0)) && s.b[2778]) && s.b[2779]) {
                s.store_sqrt(726, 726);
            }
            let (assign76320_body1_e115844,) = {
    if ((((s.v[2623] != 0.0) && (s.v[406] != 0.0)) && s.b[2778]) && s.b[2779]) {
        let assign76320_body1_e115842: f64 = (s.v[719] + 1.0);
        (assign76320_body1_e115842,)
    } else {
        (s.v[719],)
    }
};
            s.store_scalar(719, assign76320_body1_e115844);
        }

        if ((((s.v[2623] != 0.0) && (s.v[406] != 0.0)) && s.b[2778]) && (!s.b[2779])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / 2.0));
            }
        }

        if (((s.v[2623] != 0.0) && (s.v[406] != 0.0)) && s.b[2778]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_mul3_lhs(780, 781, 335, 726);
            s.store_div_scaled_product3_indices(334, 335, 725, 726, 1.0, 770, 1.0);
            s.store_add_scaled_inputs(335, 335, -1.0, 780, 1.0);
        }

        if (((s.v[2623] != 0.0) && (s.v[406] != 0.0)) && s.b[2778]) {
        }

        if (((s.v[2623] != 0.0) && (s.v[406] != 0.0)) && (!s.b[2778])) {
            s.store_add(335, 402, 397);
            s.store_scalar(334, 1.0);
        }

        if ((s.v[2623] != 0.0) && (s.v[406] != 0.0)) {
            s.store_sub(397, 335, 402);
        }

        let (assign76420_e115961,) = {
    if ((s.v[2623] != 0.0) && (s.v[406] != 0.0)) {
        let assign76420_e115955: f64 = (-s.v[397]);
        let assign76420_e115958: f64 = (10.0 * 2.220446049250313e-16);
        let assign76420_e115959: f64 = (assign76420_e115955 + assign76420_e115958);
        (assign76420_e115959,)
    } else {
        (s.v[403],)
    }
};
        s.store_scalar(403, assign76420_e115961);

        s.b[2784] = (s.v[402] < s.v[403]);
        s.store_scalar(2784, if s.b[2784] { 1.0 } else { 0.0 });

        if ((s.v[2623] != 0.0) && s.b[2784]) {
            s.store_mul_scaled_ln_ad_rhs(271, 155, 2.0, A::div_from_scalar((-s.v[270]), s.ad_value(212)));
            s.store_mul_add_rhs(332, 154, 402, 397);
            s.store_div_from_scalar_mul_ad(335, 1.0, s.ad_value(154), s.ad_value(410));
            s.store_mul(333, 335, 413);
            s.store_offset_scaled(279, 333, (3.0 * 1.414213562373095), 2.0);
            s.store_mul3_affine_lhs(277, 279, 279, 8.0, 0.0, 279);
            s.store_sub_from_scalar_scaled_mul_ad_rhs(278, (7.0 * 1.414213562373095), 333, A::offset(s.ad_value(332), (-2.0)), 9.0);
            s.store_square(276, 278);
        }

        s.b[2785] = (s.v[277] < (s.v[276] * 1e-8));
        s.store_scalar(2785, if s.b[2785] { 1.0 } else { 0.0 });

        if (((s.v[2623] != 0.0) && s.b[2784]) && s.b[2785]) {
            s.store_div_scaled_inputs_indices(274, 277, 0.5, 278, 1.0);
        }

        if (((s.v[2623] != 0.0) && s.b[2784]) && (!s.b[2785])) {
            s.store_sqrt_add(275, 277, 276);
            s.store_sub(274, 275, 278);
        }

        if ((s.v[2623] != 0.0) && s.b[2784]) {
            s.store_powf(273, 274, 0.3333333333333333);
            s.store_add_scaled_inputs_product_first_ad(272, A::sub_from_scalar(((-4.0) * 1.414213562373095), A::scale(s.ad_value(333), 12.0)), 1.0, 273, 2.0, 273, 273, 1.414213562373095);
            s.store_div(116, 272, 273);
            s.store_mul(335, 116, 155);
            s.store_div(336, 335, 271);
            s.store_sqrt_square_offset(337, 336, 1.0);
            s.store_sub_div_lhs_indices(404, 335, 337, 397);
            s.store_sub(336, 402, 404);
            s.store_mul(398, 413, 336);
            s.copy_ad(354, 398);
            s.copy_ad(2767, 404);
        }

        s.b[2786] = ((s.v[154] * (s.v[402] + s.v[397])) < (1.0 + (((((10.0 * 2.220446049250313e-16) - 1.0) * s.v[213]) * s.v[156]) / 4.0)));
        s.store_scalar(2786, if s.b[2786] { 1.0 } else { 0.0 });

        if (((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2786]) {
            s.store_add_scaled_product_indices(89, 402, 1.0, 213, 154, 0.5);
        }

        if (((s.v[2623] != 0.0) && (!s.b[2784])) && (!s.b[2786])) {
            s.store_offset_div_scaled_offset_numerator(332, A::mul(s.ad_value(154), A::add(s.ad_value(402), s.ad_value(397))), 4.0, ((-1.0) * 4.0), A::mul(s.ad_value(213), s.ad_value(156)), 1.0, 1.0);
            s.store_add_product3_rhs_mixed_iia(89, 402, 213, 154, A::sub_from_scalar(1.0, A::sqrt(s.ad_value(332))), 1.0 / (2.0));
        }

        if ((s.v[2623] != 0.0) && (!s.b[2784])) {
            s.store_mul_add_rhs(116, 154, 89, 397);
        }

        s.b[2787] = (s.v[116] >= 3.0);
        s.store_scalar(2787, if s.b[2787] { 1.0 } else { 0.0 });

        if (((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2787]) {
            s.store_exp_neg_input(333, 116);
            s.store_offset_div_scaled_inputs2_mixed_aia(332, A::offset(A::mul(s.ad_value(154), A::add(s.ad_value(402), s.ad_value(397))), (-1.0)), 4.0, 333, 4.0, A::mul(s.ad_value(213), s.ad_value(156)), 1.0, 1.0);
            s.store_add_product3_rhs_mixed_iia(89, 402, 213, 154, A::sub_from_scalar(1.0, A::sqrt(s.ad_value(332))), 1.0 / (2.0));
            s.store_mul_add_rhs(116, 154, 89, 397);
            s.store_exp_neg_input(333, 116);
            s.store_offset_div_scaled_inputs2_mixed_aia(332, A::offset(A::mul(s.ad_value(154), A::add(s.ad_value(402), s.ad_value(397))), (-1.0)), 4.0, 333, 4.0, A::mul(s.ad_value(213), s.ad_value(156)), 1.0, 1.0);
            s.store_add_product3_rhs_mixed_iia(89, 402, 213, 154, A::sub_from_scalar(1.0, A::sqrt(s.ad_value(332))), 1.0 / (2.0));
            s.store_mul_add_rhs(116, 154, 89, 397);
        }

        if (((s.v[2623] != 0.0) && (!s.b[2784])) && (!s.b[2787])) {
            s.store_scalar(434, ((1.0 / (9.0 * ((2.0) as f64).sqrt())) - ((5.0 + (7.0 * (((-3.0)) as f64).exp())) / (54.0 * (((2.0 + (((-3.0)) as f64).exp())) as f64).sqrt()))));
            s.store_scalar(435, (((1.0 + (((-3.0)) as f64).exp()) / (2.0 * (((2.0 + (((-3.0)) as f64).exp())) as f64).sqrt())) - (((2.0) as f64).sqrt() / 3.0)));
            s.store_offset_div_from_scalar_ad(436, 1.0, A::mul(s.ad_value(154), s.ad_value(212)), (1.0 / ((2.0) as f64).sqrt()));
            s.store_div_scaled_inputs2_indices(437, 402, -1.0, 397, -1.0, 212, 1.0);
            s.store_add_scaled_inputs3_div_scaled_third_mixed_aaii(441, A::div_scaled_product(A::square(s.ad_value(435)), s.ad_value(435), 1.0, A::mul3_scaled_output(s.ad_value(434), s.ad_value(434), s.ad_value(434), 27.0), 1.0), 1.0, A::div_scaled_product_by_product(s.ad_value(435), s.ad_value(436), 1.0, s.ad_value(434), s.ad_value(434), 6.0), (-1.0), 437, 1.0, 434, 2.0, 1.0);
            s.store_div_ad(440, A::add_scaled_square_product(s.ad_value(435), (-1.0), s.ad_value(434), s.ad_value(436), 3.0), A::mul_scaled_lhs(s.ad_value(434), 9.0, s.ad_value(434)));
            s.store_sqrt_add_scaled_square_cube_product(339, 441, 1.0, 440, 1.0);
            s.store_powf_ad(439, A::sub(s.ad_value(339), s.ad_value(441)), 0.3333333333333333);
            s.store_neg_powf_add_input(438, 441, 339, 0.3333333333333333);
            s.store_add_scaled_inputs3_div_scaled_third_indices(116, 439, 1.0, 438, 1.0, 435, 1.0, 434, 3.0, -1.0);
            s.store_add_scaled_product_indices(89, 397, (-1.0), 116, 155, 1.0);
        }

        s.b[2788] = (p.p33 > 0.0);
        s.store_scalar(2788, if s.b[2788] { 1.0 } else { 0.0 });

        if (((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2788]) {
            s.store_offset_add(442, 402, 397, 0.1);
            s.store_mul(222, 405, 229);
            s.store_mul(443, 405, 229);
            s.store_mul(334, 156, 213);
            s.store_mul(444, 154, 442);
            s.store_add_scaled_inputs_product_mixed_aaii(447, A::ln(A::add_scaled_square_product(s.ad_value(444), 1.0, s.ad_value(443), s.ad_value(334), 1.0)), 1.0, A::ln(A::mul(s.ad_value(405), s.ad_value(334))), (-1.0), 154, 397, 1.0);
        }

        s.b[2789] = (p.p33 == 2.0);
        s.store_scalar(2789, if s.b[2789] { 1.0 } else { 0.0 });

        if ((((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2788]) && s.b[2789]) {
            s.store_offset_sub(781, 444, 447, (-1.0));
            s.store_scale(782, 444, 4.0);
        }

    }

    pub(super) fn stamp_transient_block_77(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2788]) && s.b[2789]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if ((((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2788]) && s.b[2789]) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(335, 781, 782, 0.5, 0.5);
            s.store_add_scaled_inputs3_indices(447, 444, 1.0, 781, (-0.5), 782, (-0.5));
        }

        if ((((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2788]) && (!s.b[2789])) {
            if (s.v[447] <= s.v[444]) {
            } else {
                s.copy_ad(447, 444);
            }
        }

        if (((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2788]) {
            if (s.v[447] >= 0.0) {
            } else {
                s.store_scalar(447, 0.0);
            }
        }

        if (((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2788]) {
            s.store_sub(444, 444, 447);
            s.store_add_scaled_inputs(444, 444, 1.0, 154, 0.1);
            s.store_sub_ad(335, A::ln(A::add_scaled_square_product(s.ad_value(444), 1.0, s.ad_value(443), s.ad_value(334), 1.0)), A::ln(A::mul(s.ad_value(405), s.ad_value(334))));
            s.store_add_scaled_product_indices(446, 335, 1.0, 154, 397, 1.0);
        }

        if (((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2788]) {
            if (s.v[446] >= 0.0) {
            } else {
                s.store_scalar(446, 0.0);
            }
        }

        if (((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2788]) {
            s.copy_ad(445, 116);
        }

        s.b[2790] = (p.p33 == 2.0);
        s.store_scalar(2790, if s.b[2790] { 1.0 } else { 0.0 });

        s.b[2791] = ((s.v[445] > (s.v[446] - (0.2 * s.v[446]))) && ((0.2 * s.v[446]) >= 0.0));
        s.store_scalar(2791, if s.b[2791] { 1.0 } else { 0.0 });

        if (((((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2788]) && s.b[2790]) && s.b[2791]) {
            s.store_add_scaled_inputs3_indices(781, 445, 1.0, 446, (-1.0), 446, 0.2);
            s.store_square(722, 781);
            s.store_scaled_mul(723, 446, 446, (0.2 * 0.2));
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
        }

        let (assign77250_e117107,) = {
    if (((((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2788]) && s.b[2790]) && s.b[2791]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, assign77250_e117107);

        let (assign77260_e117120,) = {
    if (((((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2788]) && s.b[2790]) && s.b[2791]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, assign77260_e117120);

        if (((((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2788]) && s.b[2790]) && s.b[2791]) {
            s.store_scalar(770, 0.0);
            s.store_scalar(726, 0.0);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_add(770, 724, 725);
            s.copy_ad(726, 770);
        }

        s.b[2792] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.store_scalar(2792, if s.b[2792] { 1.0 } else { 0.0 });

        s.b[2793] = (2.0 == 1.0);
        s.store_scalar(2793, if s.b[2793] { 1.0 } else { 0.0 });

        let (assign77370_e117269,) = {
    if (((((((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2788]) && s.b[2790]) && s.b[2791]) && s.b[2792]) && s.b[2793]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, assign77370_e117269);

        s.b[2794] = (2.0 == 2.0);
        s.store_scalar(2794, if s.b[2794] { 1.0 } else { 0.0 });

        let (assign77390_e117292,) = {
    if ((((((((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2788]) && s.b[2790]) && s.b[2791]) && s.b[2792]) && (!s.b[2793])) && s.b[2794]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, assign77390_e117292);

        s.b[2795] = (2.0 == 4.0);
        s.store_scalar(2795, if s.b[2795] { 1.0 } else { 0.0 });

        let (assign77410_e117318,) = {
    if (((((((((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2788]) && s.b[2790]) && s.b[2791]) && s.b[2792]) && (!s.b[2793])) && (!s.b[2794])) && s.b[2795]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, assign77410_e117318);

        s.b[2796] = (2.0 == 8.0);
        s.store_scalar(2796, if s.b[2796] { 1.0 } else { 0.0 });

        let (assign77430_e117347,) = {
    if ((((((((((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2788]) && s.b[2790]) && s.b[2791]) && s.b[2792]) && (!s.b[2793])) && (!s.b[2794])) && (!s.b[2795])) && s.b[2796]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, assign77430_e117347);

        let (assign77440_e117362,) = {
    if ((((((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2788]) && s.b[2790]) && s.b[2791]) && s.b[2792]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, assign77440_e117362);

        let mut assign77450_loop_guard: usize = 0;
        while {
            let assign77450_cond_e117378: f64 = if (((((((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2788]) && s.b[2790]) && s.b[2791]) && s.b[2792]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign77450_cond_e117378 != 0.0
        } {
            assign77450_loop_guard += 1;
            assert!(assign77450_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2788]) && s.b[2790]) && s.b[2791]) && s.b[2792]) {
                s.store_sqrt(726, 726);
            }
            let (assign77450_body1_e117411,) = {
    if ((((((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2788]) && s.b[2790]) && s.b[2791]) && s.b[2792]) {
        let assign77450_body1_e117409: f64 = (s.v[719] + 1.0);
        (assign77450_body1_e117409,)
    } else {
        (s.v[719],)
    }
};
            s.store_scalar(719, assign77450_body1_e117411);
        }

        if ((((((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2788]) && s.b[2790]) && s.b[2791]) && (!s.b[2792])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }

        if (((((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2788]) && s.b[2790]) && s.b[2791]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_mul3_affine_lhs(780, 781, 446, 0.2, 0.0, 726);
            s.store_div_scaled_product3_indices(335, 446, 725, 726, 0.2, 770, 1.0);
            s.store_add_scaled_inputs3_indices(116, 446, 1.0, 446, (-0.2), 780, 1.0);
        }

        if (((((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2788]) && s.b[2790]) && s.b[2791]) {
        }

        if (((((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2788]) && s.b[2790]) && (!s.b[2791])) {
            s.copy_ad(116, 445);
            s.store_scalar(335, 1.0);
        }

        if ((((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2788]) && (!s.b[2790])) {
            if (s.v[445] <= s.v[446]) {
                s.copy_ad(116, 445);
            } else {
                s.copy_ad(116, 446);
            }
        }

        s.b[2797] = (p.p33 == 1.0);
        s.store_scalar(2797, if s.b[2797] { 1.0 } else { 0.0 });

        if (((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2797]) {
            s.store_add_scaled_product_indices(404, 397, (-1.0), 116, 155, 1.0);
        }

        s.b[2798] = (s.v[411] > 0.0);
        s.store_scalar(2798, if s.b[2798] { 1.0 } else { 0.0 });

        if ((((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2797]) && s.b[2798]) {
            s.store_sub_from_scalar(336, p.p334, 411);
        }

        if ((((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2797]) && (!s.b[2798])) {
            s.store_sqrt_offset_square_offset(782, 729, p.p137, ((4.0 * 0.1) * 0.1));
            s.store_scaled_offset_ad(343, A::div_scaled_offset_numerator(s.ad_value(729), 1.0, p.p137, s.ad_value(782), 1.0), 1.0, 0.5);
            s.store_scaled_add_ad_lhs(336, A::offset(s.ad_value(729), p.p137), 782, 0.5);
        }

        s.b[2799] = (s.v[336] < 0.0);
        s.store_scalar(2799, if s.b[2799] { 1.0 } else { 0.0 });

        if (((((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2797]) && (!s.b[2798])) && s.b[2799]) {
            s.store_scalar(336, 0.0);
            s.store_scalar(343, 0.0);
        }

        if ((((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2797]) && (!s.b[2798])) {
            s.store_scaled_sqrt_mul(600, 651, 336, p.p432);
            s.store_sub_from_scalar(336, p.p334, 600);
        }

        if (((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2797]) {
            s.store_sqrt_square_offset(782, 336, ((4.0 * (p.p334 * 0.01)) * (p.p334 * 0.01)));
            s.store_offset_scaled_div(343, 336, 782, 0.5, 0.5);
            s.store_scaled_add(336, 336, 782, 0.5);
        }

        s.b[2800] = (s.v[336] < 0.0);
        s.store_scalar(2800, if s.b[2800] { 1.0 } else { 0.0 });

        if ((((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2797]) && s.b[2800]) {
            s.store_scalar(336, 0.0);
            s.store_scalar(343, 0.0);
        }

        if (((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2797]) {
            s.copy_ad(386, 336);
            s.store_mul3_affine_lhs(418, 2760, 386, (0.5 * 9662367879.197212), 0.0, 386);
            s.store_sqrt_mul_scaled_lhs(334, 154, 2.0, 418);
            s.store_scaled_add_ad(335, A::limited_exp(s.ad_value(334)), A::limited_exp_scaled_input(s.ad_value(334), -1.0), 0.5);
            s.store_div_ln_lhs(419, 335, 418);
            s.store_add(414, 404, 397);
            s.store_mul_sub_rhs(333, 419, 414, 418);
        }

        s.b[2801] = (s.v[333] < 60.0);
        s.store_scalar(2801, if s.b[2801] { 1.0 } else { 0.0 });

        if ((((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2797]) && s.b[2801]) {
            s.store_exp(335, 333);
            s.store_exp_mul_scaled_lhs_indices(334, 419, -1.0, 418);
            s.store_sub(336, 335, 334);
            s.store_div_ln_offset_lhs(416, 336, 1.0, 419);
        }

        if ((((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2797]) && (!s.b[2801])) {
            s.store_sub(416, 414, 418);
        }

        if (((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2797]) {
            s.store_mul(415, 154, 416);
        }

        s.b[2802] = ((s.v[415] > (s.v[116] / 100.0)) && (s.v[415] > 0.0));
        s.store_scalar(2802, if s.b[2802] { 1.0 } else { 0.0 });

        let (assign77880_e118000,) = {
    if ((((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2797]) && s.b[2802]) {
        let assign77880_e117998: f64 = (s.v[2766] + 1.0);
        (assign77880_e117998,)
    } else {
        (s.v[2766],)
    }
};
        s.store_scalar(2766, assign77880_e118000);

        if ((((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2797]) && s.b[2802]) {
            s.copy_ad(116, 447);
        }

        if ((s.v[2623] != 0.0) && (!s.b[2784])) {
            s.store_add_scaled_product_indices(404, 397, (-1.0), 116, 155, 1.0);
        }

        s.b[2803] = (((s.v[116]) as f64).abs() > 1e-6);
        s.store_scalar(2803, if s.b[2803] { 1.0 } else { 0.0 });

        if (((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2803]) {
            s.store_add_offset_lhs_ad_rhs(335, 116, (-1.0), A::exp_scaled_input(s.ad_value(116), -1.0));
            s.store_sqrt(336, 335);
        }

        if (((s.v[2623] != 0.0) && (!s.b[2784])) && (!s.b[2803])) {
            s.store_mul_scaled_sqrt_ad_rhs(336, 116, 0.7071067811865475, A::sub_from_scalar(1.0, A::scale(s.ad_value(116), 0.3333333333333333)));
        }

        if ((s.v[2623] != 0.0) && (!s.b[2784])) {
            s.store_mul(354, 410, 336);
            s.store_mul_sub_rhs(398, 413, 402, 404);
            s.store_div(2804, 354, 2760);
        }

        s.b[2806] = (p.p33 == 2.0);
        s.store_scalar(2806, if s.b[2806] { 1.0 } else { 0.0 });

        s.b[2807] = ((s.v[2804] > (s.v[386] - (s.v[386] * 0.1))) && ((s.v[386] * 0.1) >= 0.0));
        s.store_scalar(2807, if s.b[2807] { 1.0 } else { 0.0 });

        if ((((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2806]) && s.b[2807]) {
            s.store_add_scaled_inputs3_indices(781, 2804, 1.0, 386, (-1.0), 386, 0.1);
            s.store_square(722, 781);
            s.store_scaled_mul(723, 386, 386, (0.1 * 0.1));
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
        }

        let (assign78060_e118207,) = {
    if ((((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2806]) && s.b[2807]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, assign78060_e118207);

        let (assign78070_e118218,) = {
    if ((((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2806]) && s.b[2807]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, assign78070_e118218);

        if ((((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2806]) && s.b[2807]) {
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
        if ((((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2806]) && s.b[2807]) {
            s.store_mul(725, 725, 723);
            s.store_add(770, 724, 725);
            s.copy_ad(726, 770);
        }

        s.b[2808] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.store_scalar(2808, if s.b[2808] { 1.0 } else { 0.0 });

        s.b[2809] = (2.0 == 1.0);
        s.store_scalar(2809, if s.b[2809] { 1.0 } else { 0.0 });

        let (assign78180_e118349,) = {
    if ((((((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2806]) && s.b[2807]) && s.b[2808]) && s.b[2809]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, assign78180_e118349);

        s.b[2810] = (2.0 == 2.0);
        s.store_scalar(2810, if s.b[2810] { 1.0 } else { 0.0 });

        let (assign78200_e118370,) = {
    if (((((((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2806]) && s.b[2807]) && s.b[2808]) && (!s.b[2809])) && s.b[2810]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, assign78200_e118370);

        s.b[2811] = (2.0 == 4.0);
        s.store_scalar(2811, if s.b[2811] { 1.0 } else { 0.0 });

        let (assign78220_e118394,) = {
    if ((((((((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2806]) && s.b[2807]) && s.b[2808]) && (!s.b[2809])) && (!s.b[2810])) && s.b[2811]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, assign78220_e118394);

        s.b[2812] = (2.0 == 8.0);
        s.store_scalar(2812, if s.b[2812] { 1.0 } else { 0.0 });

        let (assign78240_e118421,) = {
    if (((((((((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2806]) && s.b[2807]) && s.b[2808]) && (!s.b[2809])) && (!s.b[2810])) && (!s.b[2811])) && s.b[2812]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, assign78240_e118421);

        let (assign78250_e118434,) = {
    if (((((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2806]) && s.b[2807]) && s.b[2808]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, assign78250_e118434);

        let mut assign78260_loop_guard: usize = 0;
        while {
            let assign78260_cond_e118448: f64 = if ((((((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2806]) && s.b[2807]) && s.b[2808]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign78260_cond_e118448 != 0.0
        } {
            assign78260_loop_guard += 1;
            assert!(assign78260_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2806]) && s.b[2807]) && s.b[2808]) {
                s.store_sqrt(726, 726);
            }
            let (assign78260_body1_e118477,) = {
    if (((((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2806]) && s.b[2807]) && s.b[2808]) {
        let assign78260_body1_e118475: f64 = (s.v[719] + 1.0);
        (assign78260_body1_e118475,)
    } else {
        (s.v[719],)
    }
};
            s.store_scalar(719, assign78260_body1_e118477);
        }

        if (((((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2806]) && s.b[2807]) && (!s.b[2808])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }

        if ((((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2806]) && s.b[2807]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_mul3_affine_lhs(780, 781, 386, 0.1, 0.0, 726);
            s.store_div_scaled_product3_indices(334, 386, 725, 726, 0.1, 770, 1.0);
            s.store_add_scaled_inputs3_indices(335, 386, 1.0, 386, (-0.1), 780, 1.0);
        }

        if ((((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2806]) && s.b[2807]) {
        }

        if ((((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2806]) && (!s.b[2807])) {
            s.copy_ad(335, 2804);
            s.store_scalar(334, 1.0);
        }

        s.b[2813] = (s.v[334] < 1.0);
        s.store_scalar(2813, if s.b[2813] { 1.0 } else { 0.0 });

        let (assign78360_e118619,) = {
    if ((((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2806]) && s.b[2813]) {
        let assign78360_e118617: f64 = (s.v[2766] + 2.0);
        (assign78360_e118617,)
    } else {
        (s.v[2766],)
    }
};
        s.store_scalar(2766, assign78360_e118619);

        if (((s.v[2623] != 0.0) && (!s.b[2784])) && (!s.b[2806])) {
            if (s.v[2804] <= s.v[386]) {
                s.copy_ad(335, 2804);
            } else {
                s.copy_ad(335, 386);
            }
        }

        s.b[2814] = (s.v[2804] >= s.v[386]);
        s.store_scalar(2814, if s.b[2814] { 1.0 } else { 0.0 });

        let (assign78390_e118651,) = {
    if ((((s.v[2623] != 0.0) && (!s.b[2784])) && (!s.b[2806])) && s.b[2814]) {
        let assign78390_e118649: f64 = (s.v[2766] + 2.0);
        (assign78390_e118649,)
    } else {
        (s.v[2766],)
    }
};
        s.store_scalar(2766, assign78390_e118651);

        s.b[2815] = (s.v[2766] >= 2.0);
        s.store_scalar(2815, if s.b[2815] { 1.0 } else { 0.0 });

        if (((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2815]) {
            s.copy_ad(2805, 404);
            s.store_mul(354, 335, 2760);
            s.store_sub_div_rhs_indices(404, 402, 354, 413);
        }

        s.b[2816] = (p.p33 == 2.0);
        s.store_scalar(2816, if s.b[2816] { 1.0 } else { 0.0 });

        s.b[2817] = ((s.v[404] > (s.v[2805] - 0.1)) && (0.1 >= 0.0));
        s.store_scalar(2817, if s.b[2817] { 1.0 } else { 0.0 });

        if (((((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2815]) && s.b[2816]) && s.b[2817]) {
            s.store_offset_sub(781, 404, 2805, 0.1);
            s.store_square(722, 781);
            s.store_scalar(723, (0.1 * 0.1));
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
        }

        let (assign78510_e118785,) = {
    if (((((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2815]) && s.b[2816]) && s.b[2817]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, assign78510_e118785);

        let (assign78520_e118798,) = {
    if (((((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2815]) && s.b[2816]) && s.b[2817]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, assign78520_e118798);

        if (((((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2815]) && s.b[2816]) && s.b[2817]) {
            s.store_scalar(770, 0.0);
            s.store_scalar(726, 0.0);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_add(770, 724, 725);
            s.copy_ad(726, 770);
        }

        s.b[2818] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.store_scalar(2818, if s.b[2818] { 1.0 } else { 0.0 });

        s.b[2819] = (2.0 == 1.0);
        s.store_scalar(2819, if s.b[2819] { 1.0 } else { 0.0 });

        let (assign78630_e118947,) = {
    if (((((((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2815]) && s.b[2816]) && s.b[2817]) && s.b[2818]) && s.b[2819]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, assign78630_e118947);

        s.b[2820] = (2.0 == 2.0);
        s.store_scalar(2820, if s.b[2820] { 1.0 } else { 0.0 });

        let (assign78650_e118970,) = {
    if ((((((((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2815]) && s.b[2816]) && s.b[2817]) && s.b[2818]) && (!s.b[2819])) && s.b[2820]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, assign78650_e118970);

        s.b[2821] = (2.0 == 4.0);
        s.store_scalar(2821, if s.b[2821] { 1.0 } else { 0.0 });

        let (assign78670_e118996,) = {
    if (((((((((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2815]) && s.b[2816]) && s.b[2817]) && s.b[2818]) && (!s.b[2819])) && (!s.b[2820])) && s.b[2821]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, assign78670_e118996);

        s.b[2822] = (2.0 == 8.0);
        s.store_scalar(2822, if s.b[2822] { 1.0 } else { 0.0 });

        let (assign78690_e119025,) = {
    if ((((((((((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2815]) && s.b[2816]) && s.b[2817]) && s.b[2818]) && (!s.b[2819])) && (!s.b[2820])) && (!s.b[2821])) && s.b[2822]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, assign78690_e119025);

        let (assign78700_e119040,) = {
    if ((((((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2815]) && s.b[2816]) && s.b[2817]) && s.b[2818]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, assign78700_e119040);

        let mut assign78710_loop_guard: usize = 0;
        while {
            let assign78710_cond_e119056: f64 = if (((((((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2815]) && s.b[2816]) && s.b[2817]) && s.b[2818]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign78710_cond_e119056 != 0.0
        } {
            assign78710_loop_guard += 1;
            assert!(assign78710_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2815]) && s.b[2816]) && s.b[2817]) && s.b[2818]) {
                s.store_sqrt(726, 726);
            }
            let (assign78710_body1_e119089,) = {
    if ((((((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2815]) && s.b[2816]) && s.b[2817]) && s.b[2818]) {
        let assign78710_body1_e119087: f64 = (s.v[719] + 1.0);
        (assign78710_body1_e119087,)
    } else {
        (s.v[719],)
    }
};
            s.store_scalar(719, assign78710_body1_e119089);
        }

        if ((((((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2815]) && s.b[2816]) && s.b[2817]) && (!s.b[2818])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }

        if (((((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2815]) && s.b[2816]) && s.b[2817]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, 0.1);
            s.store_div_scaled_product_indices(334, 725, 726, 0.1, 770, 1.0);
            s.store_add_offset_lhs(404, 2805, (-0.1), 780);
        }

        if (((((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2815]) && s.b[2816]) && s.b[2817]) {
        }

        if (((((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2815]) && s.b[2816]) && (!s.b[2817])) {
        }

        if (((((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2815]) && s.b[2816]) && (!s.b[2817])) {
            s.store_scalar(334, 1.0);
        }

        if ((((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2815]) && (!s.b[2816])) {
            if (s.v[404] <= s.v[2805]) {
            } else {
                s.copy_ad(404, 2805);
            }
        }

        if ((s.v[2623] != 0.0) && (!s.b[2784])) {
            s.copy_ad(2767, 404);
        }

        s.b[2823] = (p.p33 == 1.0);
        s.store_scalar(2823, if s.b[2823] { 1.0 } else { 0.0 });

        let (assign78830_e119261,) = {
    if (((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2823]) {
        (0.0,)
    } else {
        (s.v[79],)
    }
};
        s.store_scalar(79, assign78830_e119261);

        if (((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2823]) {
            s.store_sqrt_scaled_input_ad(982, A::mul(A::div_from_scalar(1.034943e-10, s.ad_value(2760)), s.ad_value(155)), 2.0);
        }

        s.b[2824] = (s.v[411] > 0.0);
        s.store_scalar(2824, if s.b[2824] { 1.0 } else { 0.0 });

        if ((((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2823]) && s.b[2824]) {
            s.store_sub_from_scalar(336, p.p334, 411);
        }

        if ((((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2823]) && (!s.b[2824])) {
            s.store_sqrt_offset_square_offset(782, 729, p.p137, ((4.0 * 0.1) * 0.1));
            s.store_scaled_offset_ad(343, A::div_scaled_offset_numerator(s.ad_value(729), 1.0, p.p137, s.ad_value(782), 1.0), 1.0, 0.5);
            s.store_scaled_add_ad_lhs(336, A::offset(s.ad_value(729), p.p137), 782, 0.5);
        }

        s.b[2825] = (s.v[336] < 0.0);
        s.store_scalar(2825, if s.b[2825] { 1.0 } else { 0.0 });

        if (((((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2823]) && (!s.b[2824])) && s.b[2825]) {
            s.store_scalar(336, 0.0);
            s.store_scalar(343, 0.0);
        }

        if ((((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2823]) && (!s.b[2824])) {
            s.store_scaled_sqrt_mul(600, 651, 336, p.p432);
            s.store_sub_from_scalar(336, p.p334, 600);
        }

        if (((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2823]) {
            s.store_sqrt_square_offset(782, 336, ((4.0 * (p.p334 * 0.01)) * (p.p334 * 0.01)));
            s.store_offset_scaled_div(343, 336, 782, 0.5, 0.5);
            s.store_scaled_add(336, 336, 782, 0.5);
        }

        s.b[2826] = (s.v[336] < 0.0);
        s.store_scalar(2826, if s.b[2826] { 1.0 } else { 0.0 });

        if ((((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2823]) && s.b[2826]) {
            s.store_scalar(336, 0.0);
            s.store_scalar(343, 0.0);
        }

        if (((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2823]) {
            s.copy_ad(386, 336);
            s.store_mul3_affine_lhs(418, 2760, 386, (0.5 * 9662367879.197212), 0.0, 386);
            s.store_sqrt_mul_scaled_lhs(334, 154, 2.0, 418);
            s.store_scaled_add_ad(335, A::limited_exp(s.ad_value(334)), A::limited_exp_scaled_input(s.ad_value(334), -1.0), 0.5);
            s.store_div_ln_lhs(419, 335, 418);
        }

        let (assign79060_e119570,) = {
    if (((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2823]) {
        (1.0,)
    } else {
        (s.v[97],)
    }
};
        s.store_scalar(97, assign79060_e119570);

    }

    pub(super) fn stamp_transient_block_79(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        let mut assign79070_loop_guard: usize = 0;
        while {
            let assign79070_cond_e119580: f64 = (s.v[421] + 1.0);
            let assign79070_cond_e119582: f64 = if ((((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2823]) && (s.v[97] <= assign79070_cond_e119580)) { 1.0 } else { 0.0 };
            assign79070_cond_e119582 != 0.0
        } {
            assign79070_loop_guard += 1;
            assert!(assign79070_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2823]) {
                s.store_add(414, 404, 397);
                s.store_mul(116, 154, 414);
                s.store_mul_sub_rhs(333, 419, 414, 418);
            }
            s.b[2828] = (s.v[333] < 60.0);
            s.store_scalar(2828, if s.b[2828] { 1.0 } else { 0.0 });
            if ((((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2823]) && s.b[2828]) {
                s.store_exp(335, 333);
                s.store_exp_mul_scaled_lhs_indices(334, 419, -1.0, 418);
                s.store_sub(336, 335, 334);
                s.store_div_ln_offset_lhs(416, 336, 1.0, 419);
                s.store_div_scaled_value_offset_denominator(417, s.ad_value(335), 1.0, s.ad_value(336), 1.0, 1.0);
            }
            if ((((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2823]) && (!s.b[2828])) {
                s.store_sub(416, 414, 418);
                s.store_scalar(417, 1.0);
            }
            if (((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2823]) {
                s.store_mul(415, 154, 416);
            }
            s.b[2829] = (s.v[116] < 0.0);
            s.store_scalar(2829, if s.b[2829] { 1.0 } else { 0.0 });
            if ((((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2823]) && s.b[2829]) {
                s.store_scalar(334, (-0.7071067811865475));
                s.store_mul(223, 116, 334);
                s.store_mul(420, 154, 334);
            }
            s.b[2830] = (s.v[116] < 1e-6);
            s.store_scalar(2830, if s.b[2830] { 1.0 } else { 0.0 });
            if (((((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2823]) && (!s.b[2829])) && s.b[2830]) {
                s.store_square_mul_sub_from_scalar_double_scaled_sub_rhs_scaled_output(334, 116, 1.0, 116, 1.0, 116, 1.0, 116, 0.2, 1.0 / (4.0), 1.0 / (3.0), 1.0 / (2.0));
                s.store_mul_sub_from_scalar_ad_rhs(335, 116, 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(116), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(116), 1.0, A::scale(s.ad_value(116), 0.25), 1.0 / (3.0)), 1.0 / (2.0)));
                s.store_square_mul_sub_from_scalar_double_scaled_sub_rhs_scaled_output(336, 415, 1.0, 415, 1.0, 415, 1.0, 415, 0.2, 1.0 / (4.0), 1.0 / (3.0), 1.0 / (2.0));
                s.store_mul_sub_from_scalar_ad_rhs(337, 415, 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(415), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(415), 1.0, A::scale(s.ad_value(415), 0.25), 1.0 / (3.0)), 1.0 / (2.0)));
                s.store_sub(338, 334, 336);
            }
            s.b[2831] = (s.v[338] > 0.0);
            s.store_scalar(2831, if s.b[2831] { 1.0 } else { 0.0 });
            if ((((((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2823]) && (!s.b[2829])) && s.b[2830]) && s.b[2831]) {
                s.store_sqrt(223, 338);
                s.store_div_scaled_product_right_ad(420, 154, A::add_scaled_product(s.ad_value(335), 1.0, s.ad_value(417), s.ad_value(337), (-1.0)), 0.5, 223, 1.0);
            }
            if ((((((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2823]) && (!s.b[2829])) && s.b[2830]) && (!s.b[2831])) {
                s.store_scalar(223, 0.0);
                s.store_scalar(420, 0.0);
            }
            if (((((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2823]) && (!s.b[2829])) && (!s.b[2830])) {
                s.store_exp_neg_input(334, 116);
                s.store_exp_neg_input(335, 415);
                s.store_add_scaled_inputs4_indices(338, 116, 1.0, 415, (-1.0), 334, 1.0, 335, (-1.0));
            }
            s.b[2832] = (s.v[338] > 0.0);
            s.store_scalar(2832, if s.b[2832] { 1.0 } else { 0.0 });
            if ((((((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2823]) && (!s.b[2829])) && (!s.b[2830])) && s.b[2832]) {
                s.store_sqrt(223, 338);
                s.store_div_scaled_product_right_ad(420, 154, A::sub(A::sub_from_scalar(1.0, s.ad_value(334)), A::mul_sub_from_scalar_rhs(s.ad_value(417), 1.0, s.ad_value(335))), 0.5, 223, 1.0);
            }
            if ((((((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2823]) && (!s.b[2829])) && (!s.b[2830])) && (!s.b[2832])) {
                s.store_scalar(223, 0.0);
                s.store_scalar(420, 0.0);
            }
            s.b[2833] = (s.v[116] < 0.0);
            s.store_scalar(2833, if s.b[2833] { 1.0 } else { 0.0 });
            if ((((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2823]) && s.b[2833]) {
                s.store_scalar(214, 0.0);
                s.store_scalar(215, 0.0);
                s.store_neg(216, 223);
                s.store_neg(217, 420);
            }
            s.b[2834] = (s.v[116] < 60.0);
            s.store_scalar(2834, if s.b[2834] { 1.0 } else { 0.0 });
            s.b[2835] = (s.v[116] < 5e-5);
            s.store_scalar(2835, if s.b[2835] { 1.0 } else { 0.0 });
            if ((((((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2823]) && (!s.b[2833])) && s.b[2834]) && s.b[2835]) {
                s.store_scaled_mul_ad(334, A::square(s.ad_value(116)), A::offset(A::mul_scaled_lhs(s.ad_value(116), 1.0 / (3.0), A::offset(A::mul_scaled_lhs(s.ad_value(116), 1.0 / (4.0), A::scale_offset(s.ad_value(116), 0.2, 1.0)), 1.0)), 1.0), 1.0 / (2.0));
                s.store_mul_offset_ad_rhs(335, 116, A::mul_scaled_lhs(s.ad_value(116), 1.0 / (2.0), A::offset(A::mul_scaled_lhs(s.ad_value(116), 1.0 / (3.0), A::scale_offset(s.ad_value(116), 0.25, 1.0)), 1.0)), 1.0);
                s.store_mul(214, 222, 334);
                s.store_mul3_lhs(215, 222, 335, 154);
            }
            if ((((((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2823]) && (!s.b[2833])) && s.b[2834]) && (!s.b[2835])) {
                s.store_exp(227, 116);
                s.store_offset(335, 227, (-1.0));
                s.store_mul_sub_rhs(214, 222, 335, 116);
                s.store_mul3_lhs(215, 222, 154, 335);
            }
            if (((((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2823]) && (!s.b[2833])) && (!s.b[2834])) {
                s.store_exp_mul(231, 154, 404);
                s.store_mul_ad_rhs(214, 405, A::add_scaled_offset_product_rhs(s.ad_value(231), 1.0, s.ad_value(229), s.ad_value(116), 1.0, (-1.0)));
                s.store_mul_ad_product_rhs_mixed_ia(215, 405, 154, A::sub(s.ad_value(231), s.ad_value(229)));
            }
            s.b[2836] = (s.v[214] > 0.0);
            s.store_scalar(2836, if s.b[2836] { 1.0 } else { 0.0 });
            if (((((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2823]) && (!s.b[2833])) && s.b[2836]) {
                s.store_sqrt_square_add(216, 223, 214);
                s.store_div_scaled_add_product(217, s.ad_value(215), 0.5, s.ad_value(420), s.ad_value(223), (2.0 * 0.5), s.ad_value(216), 1.0);
            }
            if (((((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2823]) && (!s.b[2833])) && (!s.b[2836])) {
                s.copy_ad(216, 223);
                s.copy_ad(217, 420);
            }
            if (((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2823]) {
                s.store_add_scaled_inputs_product_indices(232, 404, 1.0, 402, (-1.0), 212, 216, 1.0);
                s.store_offset_mul(233, 212, 217, 1.0);
            }
            s.b[2837] = (s.v[79] == 1.0);
            s.store_scalar(2837, if s.b[2837] { 1.0 } else { 0.0 });
            let (assign79070_body72_e120728,) = {
    if ((((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2823]) && s.b[2837]) {
        let assign79070_body72_e120726: f64 = (s.v[421] + 1.0);
        (assign79070_body72_e120726,)
    } else {
        (s.v[97],)
    }
};
            s.store_scalar(97, assign79070_body72_e120728);
            if ((((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2823]) && (!s.b[2837])) {
                s.store_div_scaled_inputs_indices(236, 232, -1.0, 233, 1.0);
            }
            if ((((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2823]) && (!s.b[2837])) {
                s.store_scaled_offset_ad(93, {
                    if (1.0 >= ((s.v[404]) as f64).abs()) {
                        A::constant(1.0)
                    } else {
                        A::abs(s.ad_value(404))
                    }
                }, 1.0, (0.5 * 0.1));
            }
            s.b[2838] = (((s.v[236]) as f64).abs() > s.v[93]);
            s.store_scalar(2838, if s.b[2838] { 1.0 } else { 0.0 });
            if (((((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2823]) && (!s.b[2837])) && s.b[2838]) {
                s.store_scale(236, 93, (if (s.v[236] >= 0.0) { 1.0 } else { (-1.0) }));
            }
            if ((((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2823]) && (!s.b[2837])) {
                s.store_add(404, 404, 236);
            }
            s.b[2839] = ((((s.v[236]) as f64).abs() <= 1e-12) && (((s.v[232]) as f64).abs() <= 1e-8));
            s.store_scalar(2839, if s.b[2839] { 1.0 } else { 0.0 });
            let (assign79070_body79_e120831,) = {
    if (((((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2823]) && (!s.b[2837])) && s.b[2839]) {
        (1.0,)
    } else {
        (s.v[79],)
    }
};
            s.store_scalar(79, assign79070_body79_e120831);
            let (assign79070_body80_e120842,) = {
    if (((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2823]) {
        let assign79070_body80_e120840: f64 = (s.v[97] + 1.0);
        (assign79070_body80_e120840,)
    } else {
        (s.v[97],)
    }
};
            s.store_scalar(97, assign79070_body80_e120842);
        }

        if (((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2823]) {
            s.store_mul(2758, 982, 223);
            s.store_mul(2759, 2760, 2758);
            s.store_offset_div(100, 2759, 410, (10.0 * 2.220446049250313e-16));
            s.store_mul(354, 410, 100);
            s.store_div_from_scalar_add_ad(335, 1.0, s.ad_value(216), s.ad_value(100));
            s.store_mul3_lhs(399, 410, 214, 335);
            s.store_add(398, 354, 399);
        }

        s.b[2841] = (p.p33 == 4.0);
        s.store_scalar(2841, if s.b[2841] { 1.0 } else { 0.0 });

        if ((s.v[2623] != 0.0) && s.b[2841]) {
            s.store_exp_ad(229, A::mul_scaled_rhs(s.ad_value(154), s.ad_value(397), -1.0));
            s.store_div(334, 394, 409);
            s.store_square(405, 334);
            s.store_mul(222, 405, 229);
            s.copy_ad(404, 2767);
        }

        let (assign79220_e120979,) = {
    if ((s.v[2623] != 0.0) && s.b[2841]) {
        (0.0,)
    } else {
        (s.v[79],)
    }
};
        s.store_scalar(79, assign79220_e120979);

        if ((s.v[2623] != 0.0) && s.b[2841]) {
            s.store_sqrt_scaled_input_ad(982, A::mul(A::div_from_scalar(1.034943e-10, s.ad_value(2760)), s.ad_value(155)), 2.0);
        }

        s.b[2842] = (s.v[411] > 0.0);
        s.store_scalar(2842, if s.b[2842] { 1.0 } else { 0.0 });

        if (((s.v[2623] != 0.0) && s.b[2841]) && s.b[2842]) {
            s.store_sub_from_scalar(336, p.p334, 411);
        }

        if (((s.v[2623] != 0.0) && s.b[2841]) && (!s.b[2842])) {
            s.store_sqrt_offset_square_offset(782, 729, p.p137, ((4.0 * 0.1) * 0.1));
            s.store_scaled_offset_ad(343, A::div_scaled_offset_numerator(s.ad_value(729), 1.0, p.p137, s.ad_value(782), 1.0), 1.0, 0.5);
            s.store_scaled_add_ad_lhs(336, A::offset(s.ad_value(729), p.p137), 782, 0.5);
        }

        s.b[2843] = (s.v[336] < 0.0);
        s.store_scalar(2843, if s.b[2843] { 1.0 } else { 0.0 });

        if ((((s.v[2623] != 0.0) && s.b[2841]) && (!s.b[2842])) && s.b[2843]) {
            s.store_scalar(336, 0.0);
            s.store_scalar(343, 0.0);
        }

        if (((s.v[2623] != 0.0) && s.b[2841]) && (!s.b[2842])) {
            s.store_scaled_sqrt_mul(600, 651, 336, p.p432);
            s.store_sub_from_scalar(336, p.p334, 600);
        }

        if ((s.v[2623] != 0.0) && s.b[2841]) {
            s.store_sqrt_square_offset(782, 336, ((4.0 * (p.p334 * 0.01)) * (p.p334 * 0.01)));
            s.store_offset_scaled_div(343, 336, 782, 0.5, 0.5);
            s.store_scaled_add(336, 336, 782, 0.5);
        }

        s.b[2844] = (s.v[336] < 0.0);
        s.store_scalar(2844, if s.b[2844] { 1.0 } else { 0.0 });

        if (((s.v[2623] != 0.0) && s.b[2841]) && s.b[2844]) {
            s.store_scalar(336, 0.0);
            s.store_scalar(343, 0.0);
        }

        if ((s.v[2623] != 0.0) && s.b[2841]) {
            s.copy_ad(386, 336);
            s.store_mul3_affine_lhs(418, 2760, 386, (0.5 * 9662367879.197212), 0.0, 386);
            s.store_sqrt_mul_scaled_lhs(334, 154, 2.0, 418);
            s.store_scaled_add_ad(335, A::limited_exp(s.ad_value(334)), A::limited_exp_scaled_input(s.ad_value(334), -1.0), 0.5);
            s.store_div_ln_lhs(419, 335, 418);
        }

        let (assign79450_e121228,) = {
    if ((s.v[2623] != 0.0) && s.b[2841]) {
        (1.0,)
    } else {
        (s.v[97],)
    }
};
        s.store_scalar(97, assign79450_e121228);

    }
}
