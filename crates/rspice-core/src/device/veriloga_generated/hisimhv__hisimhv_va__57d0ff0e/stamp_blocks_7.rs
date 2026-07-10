#![allow(dead_code, non_snake_case, unused_assignments, unused_imports, unused_parens, unused_variables)]
use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, Scratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};
impl Instance {
    #[inline(never)]
    pub(super) fn stamp_transient_block_112(
        s: &mut Scratch,
    ) {
        s.b[2431] = (s.v[77] == 0.0);s.store_scalar(2431, if s.b[2431] { 1.0 } else { 0.0 });
        if (((!s.b[1443]) && s.b[2430]) && s.b[2431]) {s.store_mul_sub_rhs(116, 154, 89, 1435);}
        s.b[2432] = (s.v[116] < 3.0);s.store_scalar(2432, if s.b[2432] { 1.0 } else { 0.0 });
        if ((((!s.b[1443]) && s.b[2430]) && s.b[2431]) && s.b[2432]) {s.store_mul_sub_rhs(333, 154, 85, 1435);s.store_div_scalar_by_product_indices(335, 1.0, 154, 212, (1.414213562373095 / 108.0));s.store_offset_scaled(336, 335, 3.0, 81.0);s.store_add_scaled_sub_value_product_mixed_aii(337, (-2916.0), A::scale(s.ad_value(335), 81.0), 1.0, 335, 333, 27.0);s.store_add_scaled_sub_value_product_mixed_aii(338, 1458.0, A::scaled_offset(s.ad_value(335), 54.0, 81.0), 1.0, 335, 333, 27.0);s.store_square(338, 338);}
        if ((((!s.b[1443]) && s.b[2430]) && s.b[2431]) && s.b[2432]) {
            if ((s.v[337] + ((((((4.0 * s.v[336]) * s.v[336]) * s.v[336]) + s.v[338])) as f64).sqrt()) == 0.0) {
                s.store_scalar(339, 0.0);
            } else {
                s.store_powf_ad(339, A::add(s.ad_value(337), A::sqrt(A::add(A::mul3_scaled_output(s.ad_value(336), s.ad_value(336), s.ad_value(336), 4.0), s.ad_value(338)))), 0.3333333333333333);
            }
        }
        if ((((!s.b[1443]) && s.b[2430]) && s.b[2431]) && s.b[2432]) {s.store_add_scaled_inputs_mixed_ai(332, A::sub_from_scalar(3.0, A::div_scaled_inputs(s.ad_value(336), 1.259921049894873, s.ad_value(339), 3.0)), 1.0, 339, (1.0 / (3.0 * 1.259921049894873)));s.store_add_scaled_product_indices(89, 1435, 1.0, 332, 155, 1.0);s.copy_ad(88, 89);}
        s.b[2433] = (s.v[791] <= s.v[118]);s.store_scalar(2433, if s.b[2433] { 1.0 } else { 0.0 });
        if (((((!s.b[1443]) && s.b[2430]) && s.b[2431]) && (!s.b[2432])) && s.b[2433]) {s.copy_ad(88, 89);}
        if (((((!s.b[1443]) && s.b[2430]) && s.b[2431]) && (!s.b[2432])) && (!s.b[2433])) {s.store_div_scalar_by_product_indices(335, 1.0, 210, 211, 1.0);s.store_mul3_lhs(336, 335, 85, 85);s.store_add_div_from_scalar_rhs(337, 154, 2.0, 85);s.store_div_ln_lhs(90, 336, 337);s.store_offset_sub(781, 90, 89, (-0.0008));s.store_scale(782, 90, (4.0 * 0.0008));}
        if (((((!s.b[1443]) && s.b[2430]) && s.b[2431]) && (!s.b[2432])) && (!s.b[2433])) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }
        if (((((!s.b[1443]) && s.b[2430]) && s.b[2431]) && (!s.b[2432])) && (!s.b[2433])) {s.store_sqrt_square_add(782, 781, 782);s.store_offset_scaled_div(335, 781, 782, 0.5, 0.5);s.store_add_scaled_inputs3_indices(88, 90, 1.0, 781, (-0.5), 782, (-0.5));}
        if ((!s.b[1443]) && s.b[2430]) {s.store_offset(332, 1435, (1e-12 / 2.0));}
        s.b[2434] = (s.v[88] < s.v[332]);s.store_scalar(2434, if s.b[2434] { 1.0 } else { 0.0 });
        if (((!s.b[1443]) && s.b[2430]) && s.b[2434]) {s.copy_ad(88, 332);}
        if ((!s.b[1443]) && s.b[2430]) {s.copy_ad(87, 88);s.copy_ad(92, 89);s.store_exp_mul(229, 154, 1435);s.store_mul(222, 210, 229);}
        let (t0,) = {
    if ((!s.b[1443]) && s.b[2430]) {
        (0.0,)
    } else {
        (s.v[79],)
    }
};
        s.store_scalar(79, t0);
        let (t1,) = {
    if ((!s.b[1443]) && s.b[2430]) {
        (1.0,)
    } else {
        (s.v[97],)
    }
};
        s.store_scalar(97, t1);
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_113(
        s: &mut Scratch,
    ) {
        let mut tb: usize = 0;
        while {
            let t9: f64 = (s.v[421] + 1.0);let ta: f64 = if (((!s.b[1443]) && s.b[2430]) && (s.v[97] <= t9)) { 1.0 } else { 0.0 };
            ta != 0.0
        } {
            tb += 1;assert!(tb <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((!s.b[1443]) && s.b[2430]) {s.store_mul_sub_rhs(116, 154, 87, 1435);}
            s.b[2435] = (s.v[116] < 5.0);s.store_scalar(2435, if s.b[2435] { 1.0 } else { 0.0 });
            if (((!s.b[1443]) && s.b[2430]) && s.b[2435]) {s.store_mul3_ad_middle(225, A::square(s.ad_value(116)), 116, A::offset(A::mul(s.ad_value(116), A::scale_offset(s.ad_value(116), 0.006115288895133179, (-0.07053654284009761))), 0.29693154855771));s.store_mul_scale_offset(226, A::square(s.ad_value(116)), A::mul(s.ad_value(116), A::scale_offset(s.ad_value(116), (5.0 * 0.006115288895133179), (4.0 * (-0.07053654284009761)))), 1.0, (3.0 * 0.29693154855771));s.store_mul3_lhs(214, 222, 225, 225);s.store_mul_product3_indices(215, 226, 222, 154, 225, 2.0);s.store_mul_scale_offset_mixed_ia(223, 116, A::mul_offset_rhs(s.ad_value(116), A::mul_offset_rhs(s.ad_value(116), A::mul(s.ad_value(116), A::scale_offset(s.ad_value(116), 6.36964918866352e-5, (-0.00163730162779191))), 0.0178800506338833), (-0.117851130197758)), 1.0, 0.707106781186548);s.store_offset_mul_offset_rhs_mixed_ia(224, 116, A::mul_offset_rhs(s.ad_value(116), A::mul(s.ad_value(116), A::scale_offset(s.ad_value(116), (5.0 * 6.36964918866352e-5), (4.0 * (-0.00163730162779191)))), (3.0 * 0.0178800506338833)), (2.0 * (-0.117851130197758)), 0.707106781186548);s.store_sqrt_square_add(216, 223, 214);s.store_div_scaled_inputs2_mixed_aii(217, A::mul3_scaled_output(s.ad_value(154), s.ad_value(224), s.ad_value(223), 2.0), 1.0, 215, 1.0, 216, 2.0);}
            s.b[2436] = (s.v[116] < 60.0);s.store_scalar(2436, if s.b[2436] { 1.0 } else { 0.0 });
            if ((((!s.b[1443]) && s.b[2430]) && (!s.b[2435])) && s.b[2436]) {s.store_exp(227, 116);s.store_mul_scale_offset_indices(214, 222, 227, 1.0, (-1.0));s.store_mul3_lhs(215, 222, 154, 227);}
            if ((((!s.b[1443]) && s.b[2430]) && (!s.b[2435])) && (!s.b[2436])) {s.store_exp_mul(231, 154, 87);s.store_mul_sub_rhs(214, 210, 231, 229);s.store_mul3_lhs(215, 210, 154, 231);}
            if (((!s.b[1443]) && s.b[2430]) && (!s.b[2435])) {s.store_sqrt_add_ad(216, A::offset(s.ad_value(116), (-1.0)), s.ad_value(214));s.store_div_scaled_inputs2_indices(217, 154, 1.0, 215, 1.0, 216, 2.0);}
            if ((!s.b[1443]) && s.b[2430]) {s.store_add_scaled_inputs_product_indices(232, 85, 1.0, 87, (-1.0), 212, 216, (-1.0));s.store_sub_from_scalar_scaled_mul(233, (-1.0), 212, 217, 1.0);}
            s.b[2437] = (s.v[79] == 1.0);s.store_scalar(2437, if s.b[2437] { 1.0 } else { 0.0 });
            let (t2,) = {
    if (((!s.b[1443]) && s.b[2430]) && s.b[2437]) {
        (1.0,)
    } else {
        (s.v[944],)
    }
};
            s.store_scalar(944, t2);s.b[2438] = (s.v[944] == 0.0);s.store_scalar(2438, if s.b[2438] { 1.0 } else { 0.0 });
            if (((!s.b[1443]) && s.b[2430]) && s.b[2438]) {s.store_div_scaled_inputs_indices(236, 232, -1.0, 233, 1.0);}
            if (((!s.b[1443]) && s.b[2430]) && s.b[2438]) {
                s.store_scaled_offset_ad(93, {
                    if (1.0 >= ((s.v[87]) as f64).abs()) {
                        A::constant(1.0)
                    } else {
                        A::abs(s.ad_value(87))
                    }
                }, 1.0, (0.5 * 0.1));
            }
            s.b[2439] = (((s.v[236]) as f64).abs() > s.v[93]);s.store_scalar(2439, if s.b[2439] { 1.0 } else { 0.0 });
            if ((((!s.b[1443]) && s.b[2430]) && s.b[2438]) && s.b[2439]) {s.store_scale(236, 93, (if (s.v[236] >= 0.0) { 1.0 } else { (-1.0) }));}
            if (((!s.b[1443]) && s.b[2430]) && s.b[2438]) {s.store_add(87, 87, 236);}
            s.b[2440] = ((((s.v[236]) as f64).abs() <= 1e-12) && (((s.v[232]) as f64).abs() <= 1e-8));s.store_scalar(2440, if s.b[2440] { 1.0 } else { 0.0 });
            let (t3,) = {
    if ((((!s.b[1443]) && s.b[2430]) && s.b[2438]) && s.b[2440]) {
        (1.0,)
    } else {
        (s.v[79],)
    }
};
            s.store_scalar(79, t3);
            let (t5,) = {
    if (((!s.b[1443]) && s.b[2430]) && (s.v[944] != 0.0)) {
        let t4: f64 = (s.v[421] + 1.0);
        (t4,)
    } else {
        (s.v[97],)
    }
};
            s.store_scalar(97, t5);
            let (t6,) = {
    if ((!s.b[1443]) && s.b[2430]) {
        (0.0,)
    } else {
        (s.v[944],)
    }
};
            s.store_scalar(944, t6);
            let (t8,) = {
    if ((!s.b[1443]) && s.b[2430]) {
        let t7: f64 = (s.v[97] + 1.0);
        (t7,)
    } else {
        (s.v[97],)
    }
};
            s.store_scalar(97, t8);
        }
        let (td,) = {
    if ((!s.b[1443]) && s.b[2430]) {
        let tc: f64 = (s.v[97] - 1.0);
        (tc,)
    } else {
        (s.v[97],)
    }
};
        s.store_scalar(97, td);s.b[2442] = (s.v[116] < 5.0);s.store_scalar(2442, if s.b[2442] { 1.0 } else { 0.0 });
        if (((!s.b[1443]) && s.b[2430]) && s.b[2442]) {s.store_offset_square(99, 223, (10.0 * 2.220446049250313e-16));s.store_offset(100, 223, (10.0 * 2.220446049250313e-16));}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_114(
        s: &mut Scratch,
    ) {
        if (((!s.b[1443]) && s.b[2430]) && s.b[2442]) {s.store_offset_mul_ad(101, A::square(s.ad_value(223)), s.ad_value(223), (10.0 * 2.220446049250313e-16));}
        let (te,) = {
    if (((!s.b[1443]) && s.b[2430]) && (!s.b[2442])) {
        (3.0,)
    } else {
        (s.v[347],)
    }
};
        s.store_scalar(347, te);
        let (tf,) = {
    if (((!s.b[1443]) && s.b[2430]) && (!s.b[2442])) {
        (0.0,)
    } else {
        (s.v[78],)
    }
};
        s.store_scalar(78, tf);
        if (((!s.b[1443]) && s.b[2430]) && (!s.b[2442])) {s.store_offset(99, 116, (-1.0));s.store_sqrt(100, 99);s.store_mul(101, 99, 100);}
        if ((!s.b[1443]) && s.b[2430]) {s.store_mul(239, 209, 100);s.store_div_from_scalar_add_ad(335, 1.0, s.ad_value(216), s.ad_value(100));s.store_offset_product3(238, s.ad_value(209), s.ad_value(214), s.ad_value(335), 1.0, 1e-25);}
        s.b[2443] = (s.v[116] < 5.0);s.store_scalar(2443, if s.b[2443] { 1.0 } else { 0.0 });s.b[2444] = (s.v[116] < 3.0);s.store_scalar(2444, if s.b[2444] { 1.0 } else { 0.0 });
        let (t10,) = {
    if ((((!s.b[1443]) && s.b[2430]) && s.b[2443]) && s.b[2444]) {
        (1.0,)
    } else {
        (s.v[347],)
    }
};
        s.store_scalar(347, t10);
        let (t11,) = {
    if ((((!s.b[1443]) && s.b[2430]) && s.b[2443]) && s.b[2444]) {
        (1.0,)
    } else {
        (s.v[78],)
    }
};
        s.store_scalar(78, t11);
        if ((((!s.b[1443]) && s.b[2430]) && s.b[2443]) && s.b[2444]) {s.copy_ad(133, 238);s.copy_ad(131, 239);s.store_scalar(247, 0.5);s.store_scalar(169, 0.0);}
        let (t12,) = {
    if ((((!s.b[1443]) && s.b[2430]) && s.b[2443]) && (!s.b[2444])) {
        (2.0,)
    } else {
        (s.v[347],)
    }
};
        s.store_scalar(347, t12);
        let (t13,) = {
    if ((((!s.b[1443]) && s.b[2430]) && s.b[2443]) && (!s.b[2444])) {
        (0.0,)
    } else {
        (s.v[78],)
    }
};
        s.store_scalar(78, t13);
        if ((((!s.b[1443]) && s.b[2430]) && s.b[2443]) && (!s.b[2444])) {s.store_scalar(335, (1.0 / (5.0 - 3.0)));s.store_mul_scale_offset_indices(332, 335, 116, 1.0, (-3.0));s.store_mul3_ad_middle(207, A::square(s.ad_value(332)), 332, A::offset(A::mul(s.ad_value(332), A::scale_offset(s.ad_value(332), 6.0, (-15.0))), 10.0));}
        if ((!s.b[1443]) && s.b[2430]) {s.store_mul(127, 238, 186);s.copy_ad(349, 790);s.store_div_square_rhs(336, 636, 185);s.store_add_scaled_inputs3_indices(334, 85, 1.0, 155, (-1.0), 1438, -1.0);s.store_offset_mul_ad(335, A::div_from_scalar(2.0, s.ad_value(336)), s.ad_value(334), 1.0);s.store_sqrt_square_offset(782, 335, ((4.0 * 0.001) * 0.001));s.store_offset_scaled_div(332, 335, 782, 0.5, 0.5);s.store_scaled_add(343, 335, 782, 0.5);}
        s.b[2445] = (s.v[343] < 0.0);s.store_scalar(2445, if s.b[2445] { 1.0 } else { 0.0 });
        if (((!s.b[1443]) && s.b[2430]) && s.b[2445]) {s.store_scalar(343, 0.0);s.store_scalar(332, 0.0);}
        if ((!s.b[1443]) && s.b[2430]) {s.store_offset(343, 343, 1e-25);s.store_sqrt(337, 343);s.store_add_mul_sub_from_scalar_rhs_indices(344, 85, 336, 1.0, 337);s.store_sqrt_square_offset(782, 344, ((4.0 * 0.01) * 0.01));s.store_offset_scaled_div(334, 344, 782, 0.5, 0.5);s.store_scaled_add(344, 344, 782, 0.5);}
        s.b[2446] = (s.v[344] < 0.0);s.store_scalar(2446, if s.b[2446] { 1.0 } else { 0.0 });
        if (((!s.b[1443]) && s.b[2430]) && s.b[2446]) {s.store_scalar(344, 0.0);s.store_scalar(334, 0.0);}
        if ((!s.b[1443]) && s.b[2430]) {s.store_offset(344, 344, (10.0 * 2.220446049250313e-16));s.store_div(335, 790, 344);}
        if ((!s.b[1443]) && s.b[2430]) {
            if (s.v[335] == 0.0) {
                s.store_scalar(336, 0.0);
            } else {
                s.store_pow_offset_rhs(336, 335, 658, (-1.0));
            }
        }
        if ((!s.b[1443]) && s.b[2430]) {s.store_offset_mul(337, 336, 335, 1.0);}
        if ((!s.b[1443]) && s.b[2430]) {
            if (s.v[337] == 0.0) {
                s.store_scalar(338, 0.0);
            } else {
                s.store_pow_ad(338, s.ad_value(337), A::offset(A::div_from_scalar(1.0, s.ad_value(658)), (-1.0)));
            }
        }
        if ((!s.b[1443]) && s.b[2430]) {s.store_mul(340, 338, 337);s.store_div(348, 790, 340);s.copy_ad(790, 348);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_115(
        s: &mut Scratch,
    ) {
        if ((!s.b[1443]) && s.b[2430]) {s.store_exp_ad(230, A::mul(s.ad_value(154), A::sub(s.ad_value(1435), s.ad_value(790))));}
        s.b[2447] = (s.v[790] < 0.0);s.store_scalar(2447, if s.b[2447] { 1.0 } else { 0.0 });
        if (((!s.b[1443]) && s.b[2430]) && s.b[2447]) {s.store_scalar(94, 0.0);s.copy_ad(91, 87);}
        let (t14,) = {
    if (((!s.b[1443]) && s.b[2430]) && s.b[2447]) {
        (1.0,)
    } else {
        (s.v[947],)
    }
};
        s.store_scalar(947, t14);s.b[2448] = (s.v[947] == 0.0);s.store_scalar(2448, if s.b[2448] { 1.0 } else { 0.0 });s.b[2449] = (s.v[77] == 0.0);s.store_scalar(2449, if s.b[2449] { 1.0 } else { 0.0 });
        if ((((!s.b[1443]) && s.b[2430]) && s.b[2448]) && s.b[2449]) {
            if ((s.v[92] - s.v[87]) >= 0.0) {
                s.store_sub(96, 92, 87);
            } else {
                s.store_scalar(96, 0.0);
            }
        }
        s.b[2450] = (((1.0 + 0.3) * s.v[96]) > 0.03);s.store_scalar(2450, if s.b[2450] { 1.0 } else { 0.0 });
        if (((((!s.b[1443]) && s.b[2430]) && s.b[2448]) && s.b[2449]) && s.b[2450]) {s.store_offset_sub_scaled_inputs_indices(781, 96, (1.0 + 0.3), 790, 1.0, (-0.03));s.store_scale(782, 96, ((1.0 + 0.3) * (4.0 * 0.03)));}
        if (((((!s.b[1443]) && s.b[2430]) && s.b[2448]) && s.b[2449]) && s.b[2450]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }
        if (((((!s.b[1443]) && s.b[2430]) && s.b[2448]) && s.b[2449]) && s.b[2450]) {s.store_sqrt_square_add(782, 781, 782);s.store_offset_scaled_div(335, 781, 782, 0.5, 0.5);s.store_add_scaled_inputs3_indices(95, 96, (1.0 + 0.3), 781, (-0.5), 782, (-0.5));}
        if (((((!s.b[1443]) && s.b[2430]) && s.b[2448]) && s.b[2449]) && (!s.b[2450])) {s.store_scale(95, 96, (1.0 + 0.3));}
        if ((((!s.b[1443]) && s.b[2430]) && s.b[2448]) && s.b[2449]) {
            if (s.v[95] <= s.v[96]) {
            } else {
                s.copy_ad(95, 96);
            }
        }
        s.b[2451] = (s.v[95] < 0.0);s.store_scalar(2451, if s.b[2451] { 1.0 } else { 0.0 });
        if ((((!s.b[1443]) && s.b[2430]) && s.b[2448]) && s.b[2451]) {s.store_scalar(95, 0.0);}
        s.b[2452] = (s.v[95] > s.v[790]);s.store_scalar(2452, if s.b[2452] { 1.0 } else { 0.0 });
        if (((((!s.b[1443]) && s.b[2430]) && s.b[2448]) && (!s.b[2451])) && s.b[2452]) {s.copy_ad(95, 790);}
        if (((!s.b[1443]) && s.b[2430]) && s.b[2448]) {s.copy_ad(94, 95);s.store_add(91, 87, 94);}
        let (t15,) = {
    if (((!s.b[1443]) && s.b[2430]) && s.b[2448]) {
        (0.0,)
    } else {
        (s.v[79],)
    }
};
        s.store_scalar(79, t15);
        let (t16,) = {
    if (((!s.b[1443]) && s.b[2430]) && (s.v[947] != 0.0)) {
        (0.0,)
    } else {
        (s.v[947],)
    }
};
        s.store_scalar(947, t16);
        let (t17,) = {
    if ((!s.b[1443]) && s.b[2430]) {
        (1.0,)
    } else {
        (s.v[98],)
    }
};
        s.store_scalar(98, t17);
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_116(
        s: &mut Scratch,
    ) {
        let mut t21: usize = 0;
        while {
            let t1f: f64 = (40.0 + 1.0);let t20: f64 = if (((!s.b[1443]) && s.b[2430]) && (s.v[98] <= t1f)) { 1.0 } else { 0.0 };
            t20 != 0.0
        } {
            t21 += 1;assert!(t21 <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((!s.b[1443]) && s.b[2430]) {s.store_mul_sub_rhs(116, 154, 91, 1435);}
            s.b[2453] = (s.v[116] < 5.0);s.store_scalar(2453, if s.b[2453] { 1.0 } else { 0.0 });
            if (((!s.b[1443]) && s.b[2430]) && s.b[2453]) {s.store_mul3_ad_middle(225, A::square(s.ad_value(116)), 116, A::offset(A::mul(s.ad_value(116), A::scale_offset(s.ad_value(116), 0.006115288895133179, (-0.07053654284009761))), 0.29693154855771));s.store_mul_scale_offset(226, A::square(s.ad_value(116)), A::mul(s.ad_value(116), A::scale_offset(s.ad_value(116), (5.0 * 0.006115288895133179), (4.0 * (-0.07053654284009761)))), 1.0, (3.0 * 0.29693154855771));s.store_mul(222, 210, 230);s.store_mul3_lhs(218, 222, 225, 225);s.store_mul_product3_indices(219, 226, 222, 154, 225, 2.0);s.store_mul_scale_offset_mixed_ia(223, 116, A::mul_offset_rhs(s.ad_value(116), A::mul_offset_rhs(s.ad_value(116), A::mul(s.ad_value(116), A::scale_offset(s.ad_value(116), 6.36964918866352e-5, (-0.00163730162779191))), 0.0178800506338833), (-0.117851130197758)), 1.0, 0.707106781186548);s.store_offset_mul_offset_rhs_mixed_ia(224, 116, A::mul_offset_rhs(s.ad_value(116), A::mul(s.ad_value(116), A::scale_offset(s.ad_value(116), (5.0 * 6.36964918866352e-5), (4.0 * (-0.00163730162779191)))), (3.0 * 0.0178800506338833)), (2.0 * (-0.117851130197758)), 0.707106781186548);s.store_sqrt_square_add(220, 223, 218);s.store_div_scaled_inputs2_mixed_aii(221, A::mul3_scaled_output(s.ad_value(154), s.ad_value(224), s.ad_value(223), 2.0), 1.0, 219, 1.0, 220, 2.0);}
            if (((!s.b[1443]) && s.b[2430]) && (!s.b[2453])) {s.store_mul_sub_rhs(117, 154, 91, 790);s.store_exp(228, 117);s.store_mul_sub_rhs(218, 210, 228, 230);s.store_mul3_lhs(219, 210, 154, 228);s.store_offset(102, 116, (-1.0));s.store_sqrt_add(220, 102, 218);s.store_div_scaled_inputs2_indices(221, 154, 1.0, 219, 1.0, 220, 2.0);}
            if ((!s.b[1443]) && s.b[2430]) {s.store_add_scaled_inputs_product_indices(234, 85, 1.0, 91, (-1.0), 212, 220, (-1.0));s.store_sub_from_scalar_scaled_mul(235, (-1.0), 212, 221, 1.0);}
            s.b[2454] = (s.v[79] == 1.0);s.store_scalar(2454, if s.b[2454] { 1.0 } else { 0.0 });
            let (t18,) = {
    if (((!s.b[1443]) && s.b[2430]) && s.b[2454]) {
        (1.0,)
    } else {
        (s.v[945],)
    }
};
            s.store_scalar(945, t18);s.b[2455] = (s.v[945] == 0.0);s.store_scalar(2455, if s.b[2455] { 1.0 } else { 0.0 });
            if (((!s.b[1443]) && s.b[2430]) && s.b[2455]) {s.store_div_scaled_inputs_indices(237, 234, -1.0, 235, 1.0);}
            if (((!s.b[1443]) && s.b[2430]) && s.b[2455]) {
                s.store_scaled_offset_ad(93, {
                    if (1.0 >= ((s.v[91]) as f64).abs()) {
                        A::constant(1.0)
                    } else {
                        A::abs(s.ad_value(91))
                    }
                }, 1.0, (0.5 * 0.1));
            }
            s.b[2456] = (((s.v[237]) as f64).abs() > s.v[93]);s.store_scalar(2456, if s.b[2456] { 1.0 } else { 0.0 });
            if ((((!s.b[1443]) && s.b[2430]) && s.b[2455]) && s.b[2456]) {s.store_scale(237, 93, (if (s.v[237] >= 0.0) { 1.0 } else { (-1.0) }));}
            if (((!s.b[1443]) && s.b[2430]) && s.b[2455]) {s.store_add(91, 91, 237);}
            s.b[2457] = ((((s.v[237]) as f64).abs() <= 1e-12) && (((s.v[234]) as f64).abs() <= 1e-8));s.store_scalar(2457, if s.b[2457] { 1.0 } else { 0.0 });
            let (t19,) = {
    if ((((!s.b[1443]) && s.b[2430]) && s.b[2455]) && s.b[2457]) {
        (1.0,)
    } else {
        (s.v[79],)
    }
};
            s.store_scalar(79, t19);
            let (t1b,) = {
    if (((!s.b[1443]) && s.b[2430]) && (s.v[945] != 0.0)) {
        let t1a: f64 = (40.0 + 1.0);
        (t1a,)
    } else {
        (s.v[98],)
    }
};
            s.store_scalar(98, t1b);
            let (t1c,) = {
    if ((!s.b[1443]) && s.b[2430]) {
        (0.0,)
    } else {
        (s.v[945],)
    }
};
            s.store_scalar(945, t1c);
            let (t1e,) = {
    if ((!s.b[1443]) && s.b[2430]) {
        let t1d: f64 = (s.v[98] + 1.0);
        (t1d,)
    } else {
        (s.v[98],)
    }
};
            s.store_scalar(98, t1e);
        }
        let (t23,) = {
    if ((!s.b[1443]) && s.b[2430]) {
        let t22: f64 = (s.v[98] - 1.0);
        (t22,)
    } else {
        (s.v[98],)
    }
};
        s.store_scalar(98, t23);s.b[2459] = (s.v[116] < 5.0);s.store_scalar(2459, if s.b[2459] { 1.0 } else { 0.0 });
        if (((!s.b[1443]) && s.b[2430]) && s.b[2459]) {s.store_offset_square(102, 223, (10.0 * 2.220446049250313e-16));s.store_offset(103, 223, (10.0 * 2.220446049250313e-16));s.store_offset_mul_ad(104, A::square(s.ad_value(223)), s.ad_value(223), (10.0 * 2.220446049250313e-16));}
        if (((!s.b[1443]) && s.b[2430]) && (!s.b[2459])) {s.store_offset(102, 116, (-1.0));}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_117(
        s: &mut Scratch,
    ) {
        if (((!s.b[1443]) && s.b[2430]) && (!s.b[2459])) {s.store_sqrt(103, 102);s.store_mul(104, 102, 103);}
        if ((!s.b[1443]) && s.b[2430]) {s.store_sub(94, 91, 87);s.copy_ad(790, 349);s.store_div(335, 154, 99);s.store_mul(258, 335, 94);s.store_offset(259, 258, 1.0);s.store_sqrt(260, 259);s.store_mul(261, 260, 259);s.store_mul(262, 261, 259);s.store_div_from_scalar_offset_input(263, 1.0, 260, 1.0);s.store_div_from_scalar_offset_input(264, 1.0, 261, 1.0);s.store_div_from_scalar_offset_input(265, 1.0, 262, 1.0);s.store_div(266, 263, 100);s.store_offset_mul_offset_rhs(335, 258, 258, 3.0, 3.0);s.store_mul3_affine_lhs(267, 100, 264, 0.6666666666666667, 0.0, 335);s.store_offset_mul_offset_rhs_mixed_ia(335, 258, A::mul_offset_rhs(s.ad_value(258), A::mul_offset_rhs(s.ad_value(258), s.ad_value(258), 5.0), 10.0), 10.0, 5.0);s.store_mul_product3_mixed_iaii(268, 335, A::div_from_scalar(4.0, A::scale(s.ad_value(154), 15.0)), 101, 265, 1.0);s.store_sub_mixed_ai(269, A::add_scaled_products(s.ad_value(87), s.ad_value(267), 1.0, s.ad_value(155), s.ad_value(104), 0.6666666666666667), 268);s.store_add_scaled_inputs4_indices(335, 85, 1.0, 155, 1.0, 87, (-(2.0 * 0.5)), 94, (-0.5));s.store_sub(336, 266, 267);s.store_mul(337, 154, 185);s.store_mul(338, 154, 209);s.store_add_scaled_products_indices(250, 337, 335, 1.0, 338, 336, 1.0);s.store_mul(248, 94, 250);}
        s.b[2460] = (s.v[347] == 1.0);s.store_scalar(2460, if s.b[2460] { 1.0 } else { 0.0 });
        let (t24,) = {
    if (((!s.b[1443]) && s.b[2430]) && s.b[2460]) {
        (1.0,)
    } else {
        (s.v[948],)
    }
};
        s.store_scalar(948, t24);s.b[2461] = (s.v[948] == 0.0);s.store_scalar(2461, if s.b[2461] { 1.0 } else { 0.0 });s.b[2462] = ((s.v[508] < (10.0 * 2.220446049250313e-16)) && (s.v[509] < (10.0 * 2.220446049250313e-16)));s.store_scalar(2462, if s.b[2462] { 1.0 } else { 0.0 });
        if ((((!s.b[1443]) && s.b[2430]) && s.b[2461]) && s.b[2462]) {s.store_scalar(169, 0.0);s.copy_ad(168, 91);}
        s.b[2463] = ((s.v[168] > (((s.v[87] + s.v[790]) - (10.0 * 2.220446049250313e-16)) - (10.0 * 2.220446049250313e-16))) && ((10.0 * 2.220446049250313e-16) >= 0.0));s.store_scalar(2463, if s.b[2463] { 1.0 } else { 0.0 });
        if (((((!s.b[1443]) && s.b[2430]) && s.b[2461]) && s.b[2462]) && s.b[2463]) {s.store_offset_add_scaled_inputs3_offset_indices(781, 168, 1.0, 87, -1.0, 790, -1.0, (-(-(10.0 * 2.220446049250313e-16))), (10.0 * 2.220446049250313e-16));s.store_square(722, 781);s.store_scalar(723, ((10.0 * 2.220446049250313e-16) * (10.0 * 2.220446049250313e-16)));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);}
        let (t25,) = {
    if (((((!s.b[1443]) && s.b[2430]) && s.b[2461]) && s.b[2462]) && s.b[2463]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, t25);
        let (t26,) = {
    if (((((!s.b[1443]) && s.b[2430]) && s.b[2461]) && s.b[2462]) && s.b[2463]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t26);
        if (((((!s.b[1443]) && s.b[2430]) && s.b[2461]) && s.b[2462]) && s.b[2463]) {s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[2464] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(2464, if s.b[2464] { 1.0 } else { 0.0 });s.b[2465] = (2.0 == 1.0);s.store_scalar(2465, if s.b[2465] { 1.0 } else { 0.0 });
        let (t27,) = {
    if (((((((!s.b[1443]) && s.b[2430]) && s.b[2461]) && s.b[2462]) && s.b[2463]) && s.b[2464]) && s.b[2465]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t27);s.b[2466] = (2.0 == 2.0);s.store_scalar(2466, if s.b[2466] { 1.0 } else { 0.0 });
        let (t28,) = {
    if ((((((((!s.b[1443]) && s.b[2430]) && s.b[2461]) && s.b[2462]) && s.b[2463]) && s.b[2464]) && (!s.b[2465])) && s.b[2466]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t28);s.b[2467] = (2.0 == 4.0);s.store_scalar(2467, if s.b[2467] { 1.0 } else { 0.0 });
        let (t29,) = {
    if (((((((((!s.b[1443]) && s.b[2430]) && s.b[2461]) && s.b[2462]) && s.b[2463]) && s.b[2464]) && (!s.b[2465])) && (!s.b[2466])) && s.b[2467]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t29);s.b[2468] = (2.0 == 8.0);s.store_scalar(2468, if s.b[2468] { 1.0 } else { 0.0 });
        let (t2a,) = {
    if ((((((((((!s.b[1443]) && s.b[2430]) && s.b[2461]) && s.b[2462]) && s.b[2463]) && s.b[2464]) && (!s.b[2465])) && (!s.b[2466])) && (!s.b[2467])) && s.b[2468]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t2a);
        let (t2b,) = {
    if ((((((!s.b[1443]) && s.b[2430]) && s.b[2461]) && s.b[2462]) && s.b[2463]) && s.b[2464]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, t2b);let mut t2f: usize = 0;
        while {
            let t2e: f64 = if (((((((!s.b[1443]) && s.b[2430]) && s.b[2461]) && s.b[2462]) && s.b[2463]) && s.b[2464]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            t2e != 0.0
        } {
            t2f += 1;assert!(t2f <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((((!s.b[1443]) && s.b[2430]) && s.b[2461]) && s.b[2462]) && s.b[2463]) && s.b[2464]) {s.store_sqrt(726, 726);}
            let (t2d,) = {
    if ((((((!s.b[1443]) && s.b[2430]) && s.b[2461]) && s.b[2462]) && s.b[2463]) && s.b[2464]) {
        let t2c: f64 = (s.v[719] + 1.0);
        (t2c,)
    } else {
        (s.v[719],)
    }
};
            s.store_scalar(719, t2d);
        }
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_118(
        s: &mut Scratch,
    ) {
        if ((((((!s.b[1443]) && s.b[2430]) && s.b[2461]) && s.b[2462]) && s.b[2463]) && (!s.b[2464])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }
        if (((((!s.b[1443]) && s.b[2430]) && s.b[2461]) && s.b[2462]) && s.b[2463]) {s.store_div_from_scalar(726, 1.0, 726);s.store_scaled_mul(780, 781, 726, (10.0 * 2.220446049250313e-16));s.store_div_scaled_product_indices(334, 725, 726, (10.0 * 2.220446049250313e-16), 770, 1.0);s.store_add_scaled_inputs3_offset_indices(168, 87, 1.0, 790, 1.0, 780, 1.0, (((-(10.0 * 2.220446049250313e-16))) + ((-(10.0 * 2.220446049250313e-16)))));}
        if (((((!s.b[1443]) && s.b[2430]) && s.b[2461]) && s.b[2462]) && s.b[2463]) {
        }
        if (((((!s.b[1443]) && s.b[2430]) && s.b[2461]) && s.b[2462]) && (!s.b[2463])) {
        }
        if (((((!s.b[1443]) && s.b[2430]) && s.b[2461]) && s.b[2462]) && (!s.b[2463])) {s.store_scalar(334, 1.0);}
        if ((((!s.b[1443]) && s.b[2430]) && s.b[2461]) && (!s.b[2462])) {s.copy_ad(335, 684);s.store_sqrt_sub(342, 91, 1435);s.store_mul(171, 335, 342);s.store_div_scaled_inputs_indices(343, 335, 0.5, 342, 1.0);s.store_div_from_scalar(334, 1.0, 171);s.store_mul(335, 238, 334);s.store_scale(336, 335, s.v[509]);s.store_scale(337, 334, s.v[509]);s.store_add_scaled_product_indices(339, 336, 1.0, 508, 166, 1.0);s.store_div_from_scalar(335, 1.0, 339);s.store_scale(338, 335, 1.034943e-10);s.store_scalar(335, (1.0 - s.v[507]));s.store_add_scaled_inputs_product_indices(168, 790, s.v[507], 87, s.v[507], 335, 91, 1.0);}
        s.b[2469] = ((s.v[168] > (((s.v[87] + s.v[790]) - (10.0 * 2.220446049250313e-16)) - (10.0 * 2.220446049250313e-16))) && ((10.0 * 2.220446049250313e-16) >= 0.0));s.store_scalar(2469, if s.b[2469] { 1.0 } else { 0.0 });
        if (((((!s.b[1443]) && s.b[2430]) && s.b[2461]) && (!s.b[2462])) && s.b[2469]) {s.store_offset_add_scaled_inputs3_offset_indices(781, 168, 1.0, 87, -1.0, 790, -1.0, (-(-(10.0 * 2.220446049250313e-16))), (10.0 * 2.220446049250313e-16));s.store_square(722, 781);s.store_scalar(723, ((10.0 * 2.220446049250313e-16) * (10.0 * 2.220446049250313e-16)));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);}
        let (t30,) = {
    if (((((!s.b[1443]) && s.b[2430]) && s.b[2461]) && (!s.b[2462])) && s.b[2469]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, t30);
        let (t31,) = {
    if (((((!s.b[1443]) && s.b[2430]) && s.b[2461]) && (!s.b[2462])) && s.b[2469]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t31);
        if (((((!s.b[1443]) && s.b[2430]) && s.b[2461]) && (!s.b[2462])) && s.b[2469]) {s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[2470] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(2470, if s.b[2470] { 1.0 } else { 0.0 });s.b[2471] = (2.0 == 1.0);s.store_scalar(2471, if s.b[2471] { 1.0 } else { 0.0 });
        let (t32,) = {
    if (((((((!s.b[1443]) && s.b[2430]) && s.b[2461]) && (!s.b[2462])) && s.b[2469]) && s.b[2470]) && s.b[2471]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t32);s.b[2472] = (2.0 == 2.0);s.store_scalar(2472, if s.b[2472] { 1.0 } else { 0.0 });
        let (t33,) = {
    if ((((((((!s.b[1443]) && s.b[2430]) && s.b[2461]) && (!s.b[2462])) && s.b[2469]) && s.b[2470]) && (!s.b[2471])) && s.b[2472]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t33);s.b[2473] = (2.0 == 4.0);s.store_scalar(2473, if s.b[2473] { 1.0 } else { 0.0 });
        let (t34,) = {
    if (((((((((!s.b[1443]) && s.b[2430]) && s.b[2461]) && (!s.b[2462])) && s.b[2469]) && s.b[2470]) && (!s.b[2471])) && (!s.b[2472])) && s.b[2473]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t34);s.b[2474] = (2.0 == 8.0);s.store_scalar(2474, if s.b[2474] { 1.0 } else { 0.0 });
        let (t35,) = {
    if ((((((((((!s.b[1443]) && s.b[2430]) && s.b[2461]) && (!s.b[2462])) && s.b[2469]) && s.b[2470]) && (!s.b[2471])) && (!s.b[2472])) && (!s.b[2473])) && s.b[2474]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t35);
        let (t36,) = {
    if ((((((!s.b[1443]) && s.b[2430]) && s.b[2461]) && (!s.b[2462])) && s.b[2469]) && s.b[2470]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, t36);let mut t3a: usize = 0;
        while {
            let t39: f64 = if (((((((!s.b[1443]) && s.b[2430]) && s.b[2461]) && (!s.b[2462])) && s.b[2469]) && s.b[2470]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            t39 != 0.0
        } {
            t3a += 1;assert!(t3a <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((((!s.b[1443]) && s.b[2430]) && s.b[2461]) && (!s.b[2462])) && s.b[2469]) && s.b[2470]) {s.store_sqrt(726, 726);}
            let (t38,) = {
    if ((((((!s.b[1443]) && s.b[2430]) && s.b[2461]) && (!s.b[2462])) && s.b[2469]) && s.b[2470]) {
        let t37: f64 = (s.v[719] + 1.0);
        (t37,)
    } else {
        (s.v[719],)
    }
};
            s.store_scalar(719, t38);
        }
        if ((((((!s.b[1443]) && s.b[2430]) && s.b[2461]) && (!s.b[2462])) && s.b[2469]) && (!s.b[2470])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }
        if (((((!s.b[1443]) && s.b[2430]) && s.b[2461]) && (!s.b[2462])) && s.b[2469]) {s.store_div_from_scalar(726, 1.0, 726);s.store_scaled_mul(780, 781, 726, (10.0 * 2.220446049250313e-16));s.store_div_scaled_product_indices(334, 725, 726, (10.0 * 2.220446049250313e-16), 770, 1.0);s.store_add_scaled_inputs3_offset_indices(168, 87, 1.0, 790, 1.0, 780, 1.0, (((-(10.0 * 2.220446049250313e-16))) + ((-(10.0 * 2.220446049250313e-16)))));}
        if (((((!s.b[1443]) && s.b[2430]) && s.b[2461]) && (!s.b[2462])) && s.b[2469]) {
        }
        if (((((!s.b[1443]) && s.b[2430]) && s.b[2461]) && (!s.b[2462])) && (!s.b[2469])) {
        }
        if (((((!s.b[1443]) && s.b[2430]) && s.b[2461]) && (!s.b[2462])) && (!s.b[2469])) {s.store_scalar(334, 1.0);}
        if ((((!s.b[1443]) && s.b[2430]) && s.b[2461]) && (!s.b[2462])) {s.store_sub(340, 168, 91);s.store_mul(337, 154, 238);s.store_div_from_scalar(335, 1.0, 337);s.store_mul_ad_product_lhs_mixed_ai(339, A::offset(s.ad_value(94), (10.0 * 2.220446049250313e-16)), 250, 335);s.store_mul(336, 339, 154);s.store_scale(344, 166, 9662367879.197212);s.store_scalar(335, 100000.0);s.store_div_from_scalar(336, 1.0, 162);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_119(
        s: &mut Scratch,
    ) {
        if ((((!s.b[1443]) && s.b[2430]) && s.b[2461]) && (!s.b[2462])) {s.store_mul_mixed_ai(345, A::add_scaled_inputs_product(s.ad_value(339), 2.0, A::mul3_scaled_output(s.ad_value(344), s.ad_value(340), s.ad_value(338), 2.0), 1.0, s.ad_value(335), s.ad_value(338), 1.0), 336);s.store_mul(337, 336, 338);s.store_mul(341, 345, 338);s.store_add_scaled_product_indices(345, 335, 4.0, 344, 340, (2.0 * 4.0));s.store_mul3_affine_lhs(335, 344, 338, 8.0, 0.0, 338);s.store_scaled_mul(336, 345, 338, 2.0);s.store_mul3_lhs(342, 345, 338, 338);s.store_sqrt_square_add(343, 341, 342);s.store_scaled_sub(169, 343, 341, 0.5);s.copy_ad(335, 169);s.store_mul(169, 208, 335);}
        if (((!s.b[1443]) && s.b[2430]) && s.b[2461]) {s.store_scale(169, 169, s.v[619]);s.store_add(335, 85, 155);s.store_add_scaled_product_indices(336, 269, (-1.0), 335, 267, 1.0);s.store_mul_mixed_ia(240, 209, A::add_scaled_products(s.ad_value(209), A::add_scaled_sub_value_product(1.5, A::offset(s.ad_value(99), 1.0), 1.0, s.ad_value(154), s.ad_value(94), (-0.5)), 1.0, s.ad_value(185), s.ad_value(336), 1.0));s.copy_ad(335, 154);s.store_div_scaled_product_indices(131, 335, 240, 1.0, 250, 1.0);s.store_scale(335, 212, 2.0);s.store_mul_sub_rhs(241, 335, 267, 100);s.store_scaled_sub(336, 267, 100, 2.0);s.store_add(126, 94, 241);s.store_div_from_scalar(335, 1.0, 127);s.store_mul(336, 126, 335);s.store_sub_from_scalar(337, 1.0, 336);s.store_sub_from_scalar(332, 1.0, 337);s.store_square(722, 332);s.store_scalar(723, 1.0);s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);}
        let (t3b,) = {
    if (((!s.b[1443]) && s.b[2430]) && s.b[2461]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, t3b);
        let (t3c,) = {
    if (((!s.b[1443]) && s.b[2430]) && s.b[2461]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t3c);
        if (((!s.b[1443]) && s.b[2430]) && s.b[2461]) {s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[2475] = ((((4.0 == 1.0) || (4.0 == 2.0)) || (4.0 == 4.0)) || (4.0 == 8.0));s.store_scalar(2475, if s.b[2475] { 1.0 } else { 0.0 });s.b[2476] = (4.0 == 1.0);s.store_scalar(2476, if s.b[2476] { 1.0 } else { 0.0 });
        let (t3d,) = {
    if (((((!s.b[1443]) && s.b[2430]) && s.b[2461]) && s.b[2475]) && s.b[2476]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t3d);s.b[2477] = (4.0 == 2.0);s.store_scalar(2477, if s.b[2477] { 1.0 } else { 0.0 });
        let (t3e,) = {
    if ((((((!s.b[1443]) && s.b[2430]) && s.b[2461]) && s.b[2475]) && (!s.b[2476])) && s.b[2477]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t3e);s.b[2478] = (4.0 == 4.0);s.store_scalar(2478, if s.b[2478] { 1.0 } else { 0.0 });
        let (t3f,) = {
    if (((((((!s.b[1443]) && s.b[2430]) && s.b[2461]) && s.b[2475]) && (!s.b[2476])) && (!s.b[2477])) && s.b[2478]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t3f);s.b[2479] = (4.0 == 8.0);s.store_scalar(2479, if s.b[2479] { 1.0 } else { 0.0 });
        let (t40,) = {
    if ((((((((!s.b[1443]) && s.b[2430]) && s.b[2461]) && s.b[2475]) && (!s.b[2476])) && (!s.b[2477])) && (!s.b[2478])) && s.b[2479]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t40);
        let (t41,) = {
    if ((((!s.b[1443]) && s.b[2430]) && s.b[2461]) && s.b[2475]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, t41);let mut t45: usize = 0;
        while {
            let t44: f64 = if (((((!s.b[1443]) && s.b[2430]) && s.b[2461]) && s.b[2475]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            t44 != 0.0
        } {
            t45 += 1;assert!(t45 <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((!s.b[1443]) && s.b[2430]) && s.b[2461]) && s.b[2475]) {s.store_sqrt(726, 726);}
            let (t43,) = {
    if ((((!s.b[1443]) && s.b[2430]) && s.b[2461]) && s.b[2475]) {
        let t42: f64 = (s.v[719] + 1.0);
        (t42,)
    } else {
        (s.v[719],)
    }
};
            s.store_scalar(719, t43);
        }
        if ((((!s.b[1443]) && s.b[2430]) && s.b[2461]) && (!s.b[2475])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 4.0)));
            }
        }
        if (((!s.b[1443]) && s.b[2430]) && s.b[2461]) {s.store_div_from_scalar(726, 1.0, 726);s.store_scaled_mul(333, 332, 726, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_120(
        s: &mut Scratch,
    ) {
        if (((!s.b[1443]) && s.b[2430]) && s.b[2461]) {s.store_div_scaled_product_indices(338, 725, 726, 1.0, 770, 1.0);s.store_sub_from_scalar(125, 1.0, 333);s.store_offset_mul_offset_rhs(242, 125, 125, 1.0, 1.0);}
        s.b[2480] = (((1.0 + s.v[125]) < ((10.0 * 2.220446049250313e-16) + (10.0 * 2.220446049250313e-16))) && ((10.0 * 2.220446049250313e-16) >= 0.0));s.store_scalar(2480, if s.b[2480] { 1.0 } else { 0.0 });
        if ((((!s.b[1443]) && s.b[2430]) && s.b[2461]) && s.b[2480]) {s.store_sub_from_scalar_ad(781, ((10.0 * 2.220446049250313e-16) + (10.0 * 2.220446049250313e-16)), A::offset(s.ad_value(125), 1.0));s.store_square(722, 781);s.store_scalar(723, ((10.0 * 2.220446049250313e-16) * (10.0 * 2.220446049250313e-16)));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);}
        let (t46,) = {
    if ((((!s.b[1443]) && s.b[2430]) && s.b[2461]) && s.b[2480]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, t46);
        let (t47,) = {
    if ((((!s.b[1443]) && s.b[2430]) && s.b[2461]) && s.b[2480]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t47);
        if ((((!s.b[1443]) && s.b[2430]) && s.b[2461]) && s.b[2480]) {s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[2481] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(2481, if s.b[2481] { 1.0 } else { 0.0 });s.b[2482] = (2.0 == 1.0);s.store_scalar(2482, if s.b[2482] { 1.0 } else { 0.0 });
        let (t48,) = {
    if ((((((!s.b[1443]) && s.b[2430]) && s.b[2461]) && s.b[2480]) && s.b[2481]) && s.b[2482]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t48);s.b[2483] = (2.0 == 2.0);s.store_scalar(2483, if s.b[2483] { 1.0 } else { 0.0 });
        let (t49,) = {
    if (((((((!s.b[1443]) && s.b[2430]) && s.b[2461]) && s.b[2480]) && s.b[2481]) && (!s.b[2482])) && s.b[2483]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t49);s.b[2484] = (2.0 == 4.0);s.store_scalar(2484, if s.b[2484] { 1.0 } else { 0.0 });
        let (t4a,) = {
    if ((((((((!s.b[1443]) && s.b[2430]) && s.b[2461]) && s.b[2480]) && s.b[2481]) && (!s.b[2482])) && (!s.b[2483])) && s.b[2484]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t4a);s.b[2485] = (2.0 == 8.0);s.store_scalar(2485, if s.b[2485] { 1.0 } else { 0.0 });
        let (t4b,) = {
    if (((((((((!s.b[1443]) && s.b[2430]) && s.b[2461]) && s.b[2480]) && s.b[2481]) && (!s.b[2482])) && (!s.b[2483])) && (!s.b[2484])) && s.b[2485]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t4b);
        let (t4c,) = {
    if (((((!s.b[1443]) && s.b[2430]) && s.b[2461]) && s.b[2480]) && s.b[2481]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, t4c);let mut t50: usize = 0;
        while {
            let t4f: f64 = if ((((((!s.b[1443]) && s.b[2430]) && s.b[2461]) && s.b[2480]) && s.b[2481]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            t4f != 0.0
        } {
            t50 += 1;assert!(t50 <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((((!s.b[1443]) && s.b[2430]) && s.b[2461]) && s.b[2480]) && s.b[2481]) {s.store_sqrt(726, 726);}
            let (t4e,) = {
    if (((((!s.b[1443]) && s.b[2430]) && s.b[2461]) && s.b[2480]) && s.b[2481]) {
        let t4d: f64 = (s.v[719] + 1.0);
        (t4d,)
    } else {
        (s.v[719],)
    }
};
            s.store_scalar(719, t4e);
        }
        if (((((!s.b[1443]) && s.b[2430]) && s.b[2461]) && s.b[2480]) && (!s.b[2481])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }
        if ((((!s.b[1443]) && s.b[2430]) && s.b[2461]) && s.b[2480]) {s.store_div_from_scalar(726, 1.0, 726);s.store_scaled_mul(780, 781, 726, (10.0 * 2.220446049250313e-16));s.store_div_scaled_product_indices(334, 725, 726, (10.0 * 2.220446049250313e-16), 770, 1.0);s.store_sub_from_scalar(243, ((10.0 * 2.220446049250313e-16) + (10.0 * 2.220446049250313e-16)), 780);}
        if ((((!s.b[1443]) && s.b[2430]) && s.b[2461]) && s.b[2480]) {
        }
        if ((((!s.b[1443]) && s.b[2430]) && s.b[2461]) && (!s.b[2480])) {s.store_offset(243, 125, 1.0);s.store_scalar(334, 1.0);}
        if (((!s.b[1443]) && s.b[2430]) && s.b[2461]) {s.store_div_scaled_product_indices(335, 127, 242, 0.6666666666666667, 243, 1.0);s.store_mul(133, 335, 185);s.store_offset(244, 125, 0.5);s.store_mul(245, 243, 242);s.store_div_scaled_inputs_indices(246, 244, 0.4, 245, 1.0);s.store_sub_from_scalar(247, 0.6, 246);}
        s.b[2486] = (s.v[247] > 0.5);s.store_scalar(2486, if s.b[2486] { 1.0 } else { 0.0 });
        if ((((!s.b[1443]) && s.b[2430]) && s.b[2461]) && s.b[2486]) {s.store_scalar(247, 0.5);}
        s.b[2487] = (s.v[347] == 2.0);s.store_scalar(2487, if s.b[2487] { 1.0 } else { 0.0 });
        if ((((!s.b[1443]) && s.b[2430]) && s.b[2461]) && s.b[2487]) {s.copy_ad(335, 131);s.store_add_scaled_product_mixed_aii(131, A::mul_sub_from_scalar_lhs(1.0, s.ad_value(207), s.ad_value(239)), 1.0, 207, 131, 1.0);}
        s.b[2488] = (s.v[131] < 0.0);s.store_scalar(2488, if s.b[2488] { 1.0 } else { 0.0 });
        if (((((!s.b[1443]) && s.b[2430]) && s.b[2461]) && s.b[2487]) && s.b[2488]) {s.store_scalar(131, 0.0);}
        if ((((!s.b[1443]) && s.b[2430]) && s.b[2461]) && s.b[2487]) {s.copy_ad(335, 133);s.store_add_scaled_product_mixed_aii(133, A::mul_sub_from_scalar_lhs(1.0, s.ad_value(207), s.ad_value(238)), 1.0, 207, 133, 1.0);}
        s.b[2489] = (s.v[133] < 0.0);s.store_scalar(2489, if s.b[2489] { 1.0 } else { 0.0 });
        if (((((!s.b[1443]) && s.b[2430]) && s.b[2461]) && s.b[2487]) && s.b[2489]) {s.store_scalar(133, 0.0);}
        if ((((!s.b[1443]) && s.b[2430]) && s.b[2461]) && s.b[2487]) {s.copy_ad(335, 247);s.store_add_scaled_product_mixed_aii(247, A::scale_offset(s.ad_value(207), (-0.5), 0.5), 1.0, 207, 247, 1.0);s.copy_ad(335, 169);s.store_mul(169, 207, 169);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_121(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        let (t51,) = {
    if (((!s.b[1443]) && s.b[2430]) && (s.v[948] != 0.0)) {
        (0.0,)
    } else {
        (s.v[948],)
    }
};
        s.store_scalar(948, t51);
        if ((!s.b[1443]) && s.b[2430]) {s.store_sub(170, 162, 169);}
        s.b[2490] = (s.v[170] < 1e-9);s.store_scalar(2490, if s.b[2490] { 1.0 } else { 0.0 });
        if (((!s.b[1443]) && s.b[2430]) && s.b[2490]) {s.store_scalar(170, 1e-9);}
        if ((!s.b[1443]) && s.b[2430]) {s.store_scalar(335, (s.v[625] / 100.0));s.store_scalar(336, (s.v[626] / 100.0));s.copy_ad(334, 682);s.store_offset_mul_ad(338, A::sub(s.ad_value(91), s.ad_value(87)), s.ad_value(334), 1.0);s.store_add_scaled_products_indices(339, 335, 131, 1.0, 336, 133, 1.0);s.store_div(337, 339, 338);s.store_mul_scale_offset_rhs(251, 337, 1438, p.p166, 1.0);}
        if ((!s.b[1443]) && s.b[2430]) {
            if (s.v[251] == 0.0) {
                s.store_scalar(339, 0.0);
            } else {
                s.store_powf(339, 251, (p.p160 - 1.0));
            }
        }
        if ((!s.b[1443]) && s.b[2430]) {s.store_mul(342, 339, 251);}
        if ((!s.b[1443]) && s.b[2430]) {
            if (s.v[251] == 0.0) {
                s.store_scalar(341, 0.0);
            } else {
                s.store_pow_offset_rhs(341, 251, 624, (-1.0));
            }
        }
        if ((!s.b[1443]) && s.b[2430]) {s.store_mul(340, 341, 251);s.store_scalar(343, (1.6021918e-19 * 10000.0));s.store_div(252, 133, 343);s.store_add_scaled_inputs_mixed_ai(335, A::add_scaled_product(A::div_from_scalar(1.0, A::scale_offset(s.ad_value(252), (s.v[475] * 1e-11), s.v[474])), 1.0, s.ad_value(679), s.ad_value(342), 1.0), 1.0, 340, 1.0 / (s.v[479]));s.store_div_from_scalar(254, 1.0, 335);s.store_scale(254, 254, 0.0001);s.store_mul_ad_product_lhs_mixed_ia(336, 154, A::offset(s.ad_value(238), 1e-25), 170);s.store_div_from_scalar(335, 1.0, 336);s.store_square(337, 335);s.store_mul_scale_offset_indices(338, 337, 154, -1.0, 0.0);s.store_mul(339, 338, 170);s.store_mul_scale_offset_indices(340, 338, 238, 1.0, 1e-25);s.store_mul_ad_product_lhs_mixed_ai(333, A::offset(s.ad_value(94), (10.0 * 2.220446049250313e-16)), 250, 335);s.store_div_scaled_inputs_indices(336, 257, 0.2, 254, 1.0);s.store_div_scaled_inputs_indices(337, 336, -1.0, 254, 1.0);s.store_sqrt_square_sum(255, 333, 336);s.store_div_from_scalar(338, 1.0, 255);s.store_mul(256, 254, 255);s.store_div(335, 256, 257);}
        s.b[2491] = (((1.0 - (10.0 * 2.220446049250313e-16)) <= p.p178) && (p.p178 <= (1.0 + (10.0 * 2.220446049250313e-16))));s.store_scalar(2491, if s.b[2491] { 1.0 } else { 0.0 });
        if (((!s.b[1443]) && s.b[2430]) && s.b[2491]) {s.store_scalar(337, 1.0);}
        s.b[2492] = (((2.0 - (10.0 * 2.220446049250313e-16)) <= p.p178) && (p.p178 <= (2.0 + (10.0 * 2.220446049250313e-16))));s.store_scalar(2492, if s.b[2492] { 1.0 } else { 0.0 });
        if ((((!s.b[1443]) && s.b[2430]) && (!s.b[2491])) && s.b[2492]) {s.copy_ad(337, 335);}
        if ((((!s.b[1443]) && s.b[2430]) && (!s.b[2491])) && (!s.b[2492])) {
            if (s.v[335] == 0.0) {
                s.store_scalar(337, 0.0);
            } else {
                s.store_powf(337, 335, (p.p178 - 1.0));
            }
        }
        if ((!s.b[1443]) && s.b[2430]) {s.store_mul(336, 335, 337);s.store_offset(338, 336, 1.0);}
        s.b[2493] = (((1.0 - (10.0 * 2.220446049250313e-16)) <= p.p178) && (p.p178 <= (1.0 + (10.0 * 2.220446049250313e-16))));s.store_scalar(2493, if s.b[2493] { 1.0 } else { 0.0 });
        if (((!s.b[1443]) && s.b[2430]) && s.b[2493]) {s.store_div_from_scalar(339, 1.0, 338);}
        s.b[2494] = (((2.0 - (10.0 * 2.220446049250313e-16)) <= p.p178) && (p.p178 <= (2.0 + (10.0 * 2.220446049250313e-16))));s.store_scalar(2494, if s.b[2494] { 1.0 } else { 0.0 });
        if ((((!s.b[1443]) && s.b[2430]) && (!s.b[2493])) && s.b[2494]) {s.store_div_from_scalar_sqrt_ad(339, 1.0, s.ad_value(338));}
        if ((((!s.b[1443]) && s.b[2430]) && (!s.b[2493])) && (!s.b[2494])) {
            if (s.v[338] == 0.0) {
                s.store_scalar(340, 0.0);
            } else {
                s.store_powf(340, 338, (((-1.0) / p.p178) - 1.0));
            }
        }
        if ((((!s.b[1443]) && s.b[2430]) && (!s.b[2493])) && (!s.b[2494])) {s.store_mul(339, 338, 340);}
        if ((!s.b[1443]) && s.b[2430]) {s.store_mul(253, 254, 339);s.store_div_scaled_inputs_indices(115, 155, s.v[632], 170, 1.0);s.store_div_scaled_inputs_indices(335, 115, -1.0, 170, 1.0);s.store_mul3_lhs(135, 115, 248, 253);}
        s.b[2495] = (p.p283 != 0.0);s.store_scalar(2495, if s.b[2495] { 1.0 } else { 0.0 });
        if (((!s.b[1443]) && s.b[2430]) && s.b[2495]) {s.store_scaled_sub(335, 790, 94, 0.5);s.store_scale(781, 335, (2.0 * 100.0));}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_122(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((!s.b[1443]) && s.b[2430]) && s.b[2495]) {s.store_offset_mul_offset_rhs_mixed_ia(782, 781, A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul(s.ad_value(781), A::scale_offset(s.ad_value(781), (1.0 / 5040.0), (1.0 / 720.0))), (1.0 / 120.0)), (1.0 / 24.0)), (1.0 / 6.0)), (1.0 / 2.0), 1.0);s.store_offset_mul_offset_rhs_mixed_ia(783, 781, A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul(s.ad_value(781), A::scale_offset(s.ad_value(781), (1.0 / 840.0), (1.0 / 144.0))), (1.0 / 30.0)), (1.0 / 8.0)), (1.0 / 3.0), (1.0 / 2.0));s.store_div_from_scalar(340, 0.01, 782);s.store_div_scaled_inputs_square_rhs(336, 783, (-2.0), 782, 1.0);s.store_scale(336, 336, 0.5);s.store_sub_from_scalar_ad(335, 1.1, A::add(s.ad_value(87), s.ad_value(340)));s.store_sqrt_square_offset(782, 335, ((4.0 * 0.05) * 0.05));s.store_offset_scaled_div(334, 335, 782, 0.5, 0.5);s.store_scaled_add(336, 335, 782, 0.5);}
        s.b[2496] = (s.v[336] < 0.0);s.store_scalar(2496, if s.b[2496] { 1.0 } else { 0.0 });
        if ((((!s.b[1443]) && s.b[2430]) && s.b[2495]) && s.b[2496]) {s.store_scalar(336, 0.0);s.store_scalar(334, 0.0);}
        if (((!s.b[1443]) && s.b[2430]) && s.b[2495]) {s.store_offset(336, 336, 1e-25);s.store_scale(334, 154, s.v[672]);s.store_mul(337, 185, 334);s.store_powf(334, 336, p.p284);s.store_mul(343, 337, 334);s.store_offset_scaled(338, 1439, p.p285, 1.0);s.store_scalar(334, s.v[673]);s.store_add_scaled_inputs3_indices(339, 87, 1.0, 340, 1.0, 1438, -1.0);s.store_add_product3_rhs_indices(338, 338, 1439, 334, 339, 1.0);s.store_mul(340, 343, 338);s.copy_ad(343, 340);}
        if (((!s.b[1443]) && s.b[2430]) && (!s.b[2495])) {s.store_scalar(343, 0.0);}
        s.b[2497] = (p.p287 != 0.0);s.store_scalar(2497, if s.b[2497] { 1.0 } else { 0.0 });
        if (((!s.b[1443]) && s.b[2430]) && s.b[2497]) {s.store_scale(335, 154, s.v[674]);s.store_mul(336, 185, 335);s.store_mul(342, 336, 1439);}
        if (((!s.b[1443]) && s.b[2430]) && (!s.b[2497])) {s.store_scalar(342, 0.0);}
        s.b[2498] = ((s.v[343] + s.v[342]) > 0.0);s.store_scalar(2498, if s.b[2498] { 1.0 } else { 0.0 });
        if (((!s.b[1443]) && s.b[2430]) && s.b[2498]) {s.store_mul_add_rhs(249, 94, 343, 342);s.store_mul3_lhs(45, 115, 249, 253);s.store_add(135, 135, 45);}
        if (((!s.b[1443]) && s.b[2430]) && (!s.b[2498])) {s.store_scalar(45, 0.0);}
        s.b[2499] = ((s.v[74] == 2.0) || (s.v[74] == 3.0));s.store_scalar(2499, if s.b[2499] { 1.0 } else { 0.0 });s.b[2500] = (p.p296 > 0.0);s.store_scalar(2500, if s.b[2500] { 1.0 } else { 0.0 });
        if ((((!s.b[1443]) && s.b[2430]) && s.b[2499]) && s.b[2500]) {s.copy_ad(338, 647);s.store_scaled_offset(335, 796, (-p.p300), s.v[533]);s.store_offset_sub(781, 335, 338, (-(0.01 * 0.01)));s.store_scale(782, 338, (4.0 * (0.01 * 0.01)));}
        if ((((!s.b[1443]) && s.b[2430]) && s.b[2499]) && s.b[2500]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }
        if ((((!s.b[1443]) && s.b[2430]) && s.b[2499]) && s.b[2500]) {s.store_sqrt_square_add(782, 781, 782);s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);s.store_add_scaled_inputs3_indices(336, 338, 1.0, 781, 0.5, 782, 0.5);s.store_scale(337, 338, (p.p296 + 1.0));s.store_offset_sub(781, 337, 336, (-(0.01 * 0.01)));s.store_scale(782, 337, (4.0 * (0.01 * 0.01)));}
        if ((((!s.b[1443]) && s.b[2430]) && s.b[2499]) && s.b[2500]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }
        if ((((!s.b[1443]) && s.b[2430]) && s.b[2499]) && s.b[2500]) {s.store_sqrt_square_add(782, 781, 782);s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);s.store_add_scaled_inputs3_indices(341, 337, 1.0, 781, (-0.5), 782, (-0.5));}
        if ((((!s.b[1443]) && s.b[2430]) && s.b[2499]) && (!s.b[2500])) {s.copy_ad(341, 647);}
        s.b[2501] = (s.v[793] >= 0.0);s.store_scalar(2501, if s.b[2501] { 1.0 } else { 0.0 });
        if ((((!s.b[1443]) && s.b[2430]) && s.b[2499]) && s.b[2501]) {s.copy_ad(369, 793);}
        if ((((!s.b[1443]) && s.b[2430]) && s.b[2499]) && (!s.b[2501])) {s.store_scalar(369, 0.0);}
        s.b[2502] = (s.v[369] < (20.0 * 1e-12));s.store_scalar(2502, if s.b[2502] { 1.0 } else { 0.0 });
        if ((((!s.b[1443]) && s.b[2430]) && s.b[2499]) && s.b[2502]) {s.store_scalar(378, (((((20.0 + 1.0)) as f64).powf((p.p297 - 1.0)) * ((20.0 + 1.0) - ((0.5 * p.p297) * 20.0))) * ((1e-12) as f64).powf(p.p297)));s.store_scalar(379, ((((0.5 * p.p297) * (((20.0 + 1.0)) as f64).powf((p.p297 - 1.0))) / 20.0) * ((1e-12) as f64).powf((p.p297 - 2.0))));}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_123(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((((!s.b[1443]) && s.b[2430]) && s.b[2499]) && s.b[2502]) {s.store_add_product3_rhs_indices(335, 378, 379, 369, 369, 1.0);}
        if ((((!s.b[1443]) && s.b[2430]) && s.b[2499]) && (!s.b[2502])) {s.store_powf_offset_input(335, 369, 1e-12, p.p297);}
        if (((!s.b[1443]) && s.b[2430]) && s.b[2499]) {s.store_powf_offset_input(343, 369, 1e-12, p.p299);s.store_add_scaled_products_indices(368, 341, 335, 1.0 / (s.v[632]), 797, 343, (s.v[531] * 1.0 / (s.v[632])));s.store_mul(334, 368, 135);s.store_offset(335, 790, 1e-12);s.store_div_from_scalar(336, 1.0, 335);s.store_offset_mul(337, 334, 336, 1.0);s.store_div_from_scalar(338, 1.0, 337);s.store_mul(134, 135, 338);}
        if (((!s.b[1443]) && s.b[2430]) && (!s.b[2499])) {s.copy_ad(134, 135);s.store_scalar(368, 0.0);}
        s.b[2503] = (p.p27 != 0.0);s.store_scalar(2503, if s.b[2503] { 1.0 } else { 0.0 });
        if (((!s.b[1443]) && s.b[2430]) && s.b[2503]) {s.store_scale(335, 186, 1.034943e-10);s.copy_ad(336, 684);s.store_scalar(337, (s.v[628] - p.p139));s.store_div_from_scalar_square_ad(338, 1.0, s.ad_value(337));s.store_mul_ad_product_lhs_mixed_ai(339, A::mul_sub_from_scalar_lhs_scaled_output(p.p137, s.ad_value(123), s.ad_value(335), 2.0), 336, 338);s.store_mul(121, 339, 181);s.store_div_scaled_inputs_indices(340, 339, 0.5, 181, 1.0);s.store_mul_ad_product_lhs_mixed_ai(341, A::mul_sub_from_scalar_lhs_scaled_output(p.p137, s.ad_value(123), s.ad_value(336), (2.0 * 1.034943e-10)), 338, 181);s.store_mul_product3_indices(342, 181, 335, 336, 338, (-2.0));s.store_scalar(338, s.v[496]);s.store_scalar(340, s.v[497]);s.store_add_scaled_product_indices(335, 338, 1.0, 340, 1439, 1.0);s.store_mul(137, 121, 335);s.store_sub_from_scalar_scaled_input(335, s.v[498], 790, p.p213);s.store_add_scaled_inputs3_offset_indices(138, 1440, 1.0, 335, 1.0, 137, 1.0, (-s.v[160]));s.store_mul3_lhs(141, 694, 186, 186);s.store_scaled_mul(142, 141, 154, 0.5);s.store_scaled_mul(143, 142, 154, 2.0);s.store_scale(345, 154, 0.25);s.store_offset_sub_ad(344, A::offset(A::add_scaled_product(s.ad_value(155), 1.0, s.ad_value(141), s.ad_value(345), (-1.0)), ((s.v[160]) + ((-s.v[498])))), s.ad_value(137), 1e-25);s.store_offset_sub(335, 1440, 344, (-0.005));}
        if (((!s.b[1443]) && s.b[2430]) && s.b[2503]) {s.store_scalar(334, (if (s.v[344] >= 0.0) { 1.0 } else { (-1.0) }));}
        if (((!s.b[1443]) && s.b[2430]) && s.b[2503]) {s.store_sqrt_add_scaled_square_product(336, 335, 1.0, 334, 344, (4.0 * 0.005));s.store_sub_mixed_ai(337, A::add_scaled_inputs4_offset(s.ad_value(344), 1.0, s.ad_value(335), 0.5, s.ad_value(336), 0.5, s.ad_value(137), 1.0, (((-s.v[160])) + (s.v[498]))), 1438);s.store_offset_mul(338, 154, 337, (-1.0));s.store_div_from_scalar(339, 4.0, 143);s.store_offset_mul(335, 338, 339, 1.0);s.store_mul(340, 154, 339);s.store_mul(341, 338, 339);s.store_sqrt_square_offset(782, 335, ((4.0 * 0.01) * 0.01));s.store_offset_scaled_div(336, 335, 782, 0.5, 0.5);s.store_scaled_add(335, 335, 782, 0.5);}
        s.b[2504] = (s.v[335] < 0.0);s.store_scalar(2504, if s.b[2504] { 1.0 } else { 0.0 });
        if ((((!s.b[1443]) && s.b[2430]) && s.b[2503]) && s.b[2504]) {s.store_scalar(335, 0.0);s.store_scalar(336, 0.0);}
        if (((!s.b[1443]) && s.b[2430]) && s.b[2503]) {s.store_offset(335, 335, 1e-25);s.store_sqrt(144, 335);s.store_mul_scale_offset_indices(334, 142, 144, -1.0, 1.0);s.store_add(146, 138, 334);s.store_div_from_scalar_add_ad(334, 1.0, s.ad_value(154), A::div_scalar_offset_denominator(2.0, s.ad_value(138), 1e-25, 1.0));s.store_mul_ln_mixed_ia(147, 334, A::mul(A::div_scalar_by_product(1.0, s.ad_value(140), s.ad_value(141), 1.0), A::square(s.ad_value(138))));s.store_offset_sub(148, 147, 146, (-0.002));s.store_sqrt_add_scaled_square_input(334, 148, 1.0, 147, (4.0 * 0.002));s.store_add_scaled_inputs3_indices(149, 147, 1.0, 148, (-0.5), 334, (-0.5));s.store_mul_exp_mixed_ia(334, 140, A::mul(s.ad_value(154), s.ad_value(149)));}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_124(
        s: &mut Scratch,
    ) {
        if (((!s.b[1443]) && s.b[2430]) && s.b[2503]) {s.store_add_offset_lhs_mixed_ai(335, A::mul(s.ad_value(154), A::sub(s.ad_value(149), s.ad_value(1438))), (-1.0), 334);s.store_sqrt_square_offset(782, 335, ((4.0 * 0.01) * 0.01));s.store_offset_scaled_div(334, 335, 782, 0.5, 0.5);s.store_scaled_add(335, 335, 782, 0.5);}
        s.b[2505] = (s.v[335] < 0.0);s.store_scalar(2505, if s.b[2505] { 1.0 } else { 0.0 });
        if ((((!s.b[1443]) && s.b[2430]) && s.b[2503]) && s.b[2505]) {s.store_scalar(335, 0.0);s.store_scalar(334, 0.0);}
        if (((!s.b[1443]) && s.b[2430]) && s.b[2503]) {s.store_offset(335, 335, 1e-25);s.store_sqrt(150, 335);s.store_offset_mul_ad(335, s.ad_value(154), A::sub(s.ad_value(149), s.ad_value(1438)), (-1.0));s.store_sqrt_square_offset(782, 335, ((4.0 * 0.01) * 0.01));s.store_offset_scaled_div(334, 335, 782, 0.5, 0.5);s.store_scaled_add(335, 335, 782, 0.5);}
        s.b[2506] = (s.v[335] < 0.0);s.store_scalar(2506, if s.b[2506] { 1.0 } else { 0.0 });
        if ((((!s.b[1443]) && s.b[2430]) && s.b[2503]) && s.b[2506]) {s.store_scalar(335, 0.0);s.store_scalar(334, 0.0);}
        if (((!s.b[1443]) && s.b[2430]) && s.b[2503]) {s.store_offset(335, 335, 1e-25);s.store_sqrt(151, 335);s.store_div_from_scalar(336, 0.5, 151);s.store_mul_sub_rhs(152, 139, 150, 151);s.store_sub(335, 146, 149);s.store_sqrt_square_offset(782, 335, ((4.0 * 0.1) * 0.1));s.store_offset_scaled_div(336, 335, 782, 0.5, 0.5);s.store_scaled_add(335, 335, 782, 0.5);}
        s.b[2507] = (s.v[335] < 0.0);s.store_scalar(2507, if s.b[2507] { 1.0 } else { 0.0 });
        if ((((!s.b[1443]) && s.b[2430]) && s.b[2503]) && s.b[2507]) {s.store_scalar(335, 0.0);s.store_scalar(336, 0.0);}
        if (((!s.b[1443]) && s.b[2430]) && s.b[2503]) {s.store_offset(335, 335, 1e-25);s.store_div(332, 790, 335);s.store_div_from_scalar_square_ad(336, 1.0, s.ad_value(335));s.store_square(722, 332);s.store_scalar(723, 1.0);s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);}
        let (t52,) = {
    if (((!s.b[1443]) && s.b[2430]) && s.b[2503]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, t52);
        let (t53,) = {
    if (((!s.b[1443]) && s.b[2430]) && s.b[2503]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t53);
        if (((!s.b[1443]) && s.b[2430]) && s.b[2503]) {s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[2508] = ((((4.0 == 1.0) || (4.0 == 2.0)) || (4.0 == 4.0)) || (4.0 == 8.0));s.store_scalar(2508, if s.b[2508] { 1.0 } else { 0.0 });s.b[2509] = (4.0 == 1.0);s.store_scalar(2509, if s.b[2509] { 1.0 } else { 0.0 });
        let (t54,) = {
    if (((((!s.b[1443]) && s.b[2430]) && s.b[2503]) && s.b[2508]) && s.b[2509]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t54);s.b[2510] = (4.0 == 2.0);s.store_scalar(2510, if s.b[2510] { 1.0 } else { 0.0 });
        let (t55,) = {
    if ((((((!s.b[1443]) && s.b[2430]) && s.b[2503]) && s.b[2508]) && (!s.b[2509])) && s.b[2510]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t55);s.b[2511] = (4.0 == 4.0);s.store_scalar(2511, if s.b[2511] { 1.0 } else { 0.0 });
        let (t56,) = {
    if (((((((!s.b[1443]) && s.b[2430]) && s.b[2503]) && s.b[2508]) && (!s.b[2509])) && (!s.b[2510])) && s.b[2511]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t56);s.b[2512] = (4.0 == 8.0);s.store_scalar(2512, if s.b[2512] { 1.0 } else { 0.0 });
        let (t57,) = {
    if ((((((((!s.b[1443]) && s.b[2430]) && s.b[2503]) && s.b[2508]) && (!s.b[2509])) && (!s.b[2510])) && (!s.b[2511])) && s.b[2512]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t57);
        let (t58,) = {
    if ((((!s.b[1443]) && s.b[2430]) && s.b[2503]) && s.b[2508]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, t58);let mut t5c: usize = 0;
        while {
            let t5b: f64 = if (((((!s.b[1443]) && s.b[2430]) && s.b[2503]) && s.b[2508]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            t5b != 0.0
        } {
            t5c += 1;assert!(t5c <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((!s.b[1443]) && s.b[2430]) && s.b[2503]) && s.b[2508]) {s.store_sqrt(726, 726);}
            let (t5a,) = {
    if ((((!s.b[1443]) && s.b[2430]) && s.b[2503]) && s.b[2508]) {
        let t59: f64 = (s.v[719] + 1.0);
        (t59,)
    } else {
        (s.v[719],)
    }
};
            s.store_scalar(719, t5a);
        }
        if ((((!s.b[1443]) && s.b[2430]) && s.b[2503]) && (!s.b[2508])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 4.0)));
            }
        }
        if (((!s.b[1443]) && s.b[2430]) && s.b[2503]) {s.store_div_from_scalar(726, 1.0, 726);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_125(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((!s.b[1443]) && s.b[2430]) && s.b[2503]) {s.store_scaled_mul(333, 332, 726, 1.0);s.store_div_scaled_product_indices(336, 725, 726, 1.0, 770, 1.0);s.store_scale(145, 155, ((2.0 * s.v[495]) * p.p7));s.copy_ad(335, 170);s.store_div_scaled_product_mixed_aii(153, A::mul3(s.ad_value(145), s.ad_value(253), s.ad_value(152)), 333, 1.0, 335, 1.0);s.store_add(134, 134, 153);}
        s.b[2513] = (((p.p31 != 0.0) && (p.p30 != 0.0)) && (s.v[963] == 0.0));s.store_scalar(2513, if s.b[2513] { 1.0 } else { 0.0 });
        if (((!s.b[1443]) && s.b[2430]) && s.b[2513]) {s.store_square(317, 127);s.store_mul3_affine_lhs(318, 155, 186, 2.0, 0.0, 248);s.store_sub(319, 317, 318);s.store_sqrt_square_offset(782, 317, ((4.0 * 0.001) * 0.001));s.store_offset_scaled_div(334, 317, 782, 0.5, 0.5);s.store_scaled_add(317, 317, 782, 0.5);}
        s.b[2514] = (s.v[317] < 0.0);s.store_scalar(2514, if s.b[2514] { 1.0 } else { 0.0 });
        if ((((!s.b[1443]) && s.b[2430]) && s.b[2513]) && s.b[2514]) {s.store_scalar(317, 0.0);s.store_scalar(334, 0.0);}
        if (((!s.b[1443]) && s.b[2430]) && s.b[2513]) {s.store_sqrt_square_offset(782, 319, ((4.0 * 0.001) * 0.001));s.store_offset_scaled_div(334, 319, 782, 0.5, 0.5);s.store_scaled_add(319, 319, 782, 0.5);}
        s.b[2515] = (s.v[319] < 0.0);s.store_scalar(2515, if s.b[2515] { 1.0 } else { 0.0 });
        if ((((!s.b[1443]) && s.b[2430]) && s.b[2513]) && s.b[2515]) {s.store_scalar(319, 0.0);s.store_scalar(334, 0.0);}
        if (((!s.b[1443]) && s.b[2430]) && s.b[2513]) {s.store_sub(320, 317, 319);}
        s.b[2516] = ((s.v[238] < (10.0 * 2.220446049250313e-16)) || (s.v[320] < (10.0 * 2.220446049250313e-16)));s.store_scalar(2516, if s.b[2516] { 1.0 } else { 0.0 });
        let (t5d,) = {
    if ((((!s.b[1443]) && s.b[2430]) && s.b[2513]) && s.b[2516]) {
        (0.0,)
    } else {
        (s.v[321],)
    }
};
        s.store_scalar(321, t5d);
        let (t5e,) = {
    if ((((!s.b[1443]) && s.b[2430]) && s.b[2513]) && (!s.b[2516])) {
        (1.0,)
    } else {
        (s.v[321],)
    }
};
        s.store_scalar(321, t5e);
        let (t5f,) = {
    if ((!s.b[1443]) && (s.v[946] != 0.0)) {
        (0.0,)
    } else {
        (s.v[946],)
    }
};
        s.store_scalar(946, t5f);s.b[2517] = ((s.v[78] == 0.0) && (s.v[127] > 1e-12));s.store_scalar(2517, if s.b[2517] { 1.0 } else { 0.0 });
        if ((!s.b[1443]) && s.b[2517]) {s.store_div_scaled_product_indices(130, 212, 154, 1.0, 100, 2.0);s.store_add_mixed_ai(128, A::div_scaled_value_offset_denominator(s.ad_value(127), 1.0, s.ad_value(130), 1.0, 1.0), 87);}
        if ((!s.b[1443]) && (!s.b[2517])) {s.store_scalar(128, 0.0);}
        if (!s.b[1443]) {s.copy_ad(136, 134);s.store_scalar(46, 0.0);}
        s.b[2519] = ((p.p450 > 0.0) && (p.p454 > 0.0));s.store_scalar(2519, if s.b[2519] { 1.0 } else { 0.0 });
        if ((!s.b[1443]) && s.b[2519]) {s.store_scalar(2524, 1e-5);s.store_offset_add_scaled_inputs3_offset_indices(2525, 791, 1.0, 120, 1.0, 182, -1.0, (-s.v[160]), (-p.p455));}
        let (t61,) = {
    if ((!s.b[1443]) && s.b[2519]) {
        let t60: f64 = (s.v[118] + p.p455);
        (t60,)
    } else {
        (s.v[2526],)
    }
};
        s.store_scalar(2526, t61);
        if ((!s.b[1443]) && s.b[2519]) {s.store_sqrt_offset_ad(781, A::square(A::sub(s.ad_value(960), s.ad_value(1435))), ((4.0 * 0.01) * 0.01));s.store_add_scaled_inputs3_indices(2536, 960, 0.5, 1435, ((-1.0) * 0.5), 781, 0.5);s.store_sqrt_ad(2520, A::div_scaled_product_offset_denominator(s.ad_value(2536), s.ad_value(586), (((2.0 * 1.6021918e-19) * 1.0 / (1.034943e-10)) * s.v[489]), s.ad_value(586), s.v[489], 1.0));s.store_mul(2522, 2520, 162);s.store_div_scaled_product_add_scaled_denominator_indices(993, 2522, 2522, (-0.25), 790, 1.0, 2522, 1.0, 1.0);}
        s.b[2538] = (p.p457 > 0.0);s.store_scalar(2538, if s.b[2538] { 1.0 } else { 0.0 });
        if (((!s.b[1443]) && s.b[2519]) && s.b[2538]) {s.store_scalar(2523, p.p457);}
        if (((!s.b[1443]) && s.b[2519]) && (!s.b[2538])) {s.copy_ad(2539, 993);}
        let (t62,) = {
    if (((!s.b[1443]) && s.b[2519]) && (!s.b[2538])) {
        (s.v[2526],)
    } else {
        (s.v[2540],)
    }
};
        s.store_scalar(2540, t62);
        if (((!s.b[1443]) && s.b[2519]) && (!s.b[2538])) {s.store_offset_div_scaled_offset_numerator(332, A::mul(s.ad_value(154), A::sub(s.ad_value(2525), s.ad_value(2539))), 4.0, ((-1.0) * 4.0), A::mul(s.ad_value(213), s.ad_value(156)), 1.0, 1.0);}
        if (((!s.b[1443]) && s.b[2519]) && (!s.b[2538])) {
            if (s.v[332] >= (10.0 * 2.220446049250313e-16)) {
            } else {
                s.store_scalar(332, (10.0 * 2.220446049250313e-16));
            }
        }
        if (((!s.b[1443]) && s.b[2519]) && (!s.b[2538])) {s.store_add_product3_rhs_mixed_iia(89, 2525, 213, 154, A::sub_from_scalar(1.0, A::sqrt(s.ad_value(332))), 0.5);s.store_mul_sub_rhs(116, 154, 89, 2539);}
        s.b[2541] = (s.v[116] < 3.0);s.store_scalar(2541, if s.b[2541] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_126(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((((!s.b[1443]) && s.b[2519]) && (!s.b[2538])) && s.b[2541]) {s.store_mul_sub_rhs(333, 154, 2525, 2539);s.store_div_scalar_by_product_indices(335, 1.0, 154, 212, (1.414213562373095 / 108.0));s.store_offset_scaled(336, 335, 3.0, 81.0);s.store_add_scaled_sub_value_product_mixed_aii(337, (-2916.0), A::scale(s.ad_value(335), 81.0), 1.0, 335, 333, 27.0);s.store_add_scaled_sub_value_product_mixed_aii(338, 1458.0, A::scaled_offset(s.ad_value(335), 54.0, 81.0), 1.0, 335, 333, 27.0);s.store_square(338, 338);}
        if ((((!s.b[1443]) && s.b[2519]) && (!s.b[2538])) && s.b[2541]) {
            if ((s.v[337] + ((((((4.0 * s.v[336]) * s.v[336]) * s.v[336]) + s.v[338])) as f64).sqrt()) == 0.0) {
                s.store_scalar(339, 0.0);
            } else {
                s.store_powf_ad(339, A::add(s.ad_value(337), A::sqrt(A::add(A::mul3_scaled_output(s.ad_value(336), s.ad_value(336), s.ad_value(336), 4.0), s.ad_value(338)))), 0.3333333333333333);
            }
        }
        if ((((!s.b[1443]) && s.b[2519]) && (!s.b[2538])) && s.b[2541]) {s.store_add_scaled_inputs_mixed_ai(332, A::sub_from_scalar(3.0, A::div_scaled_inputs(s.ad_value(336), 1.259921049894873, s.ad_value(339), 3.0)), 1.0, 339, (1.0 / (3.0 * 1.259921049894873)));s.store_add_scaled_product_indices(89, 2539, 1.0, 332, 155, 1.0);s.copy_ad(88, 89);}
        s.b[2542] = (s.v[791] <= s.v[2540]);s.store_scalar(2542, if s.b[2542] { 1.0 } else { 0.0 });
        if (((((!s.b[1443]) && s.b[2519]) && (!s.b[2538])) && (!s.b[2541])) && s.b[2542]) {s.copy_ad(88, 89);}
        if (((((!s.b[1443]) && s.b[2519]) && (!s.b[2538])) && (!s.b[2541])) && (!s.b[2542])) {s.store_div_scalar_by_product_indices(335, 1.0, 210, 211, 1.0);s.store_mul3_lhs(336, 335, 2525, 2525);s.store_add_div_from_scalar_rhs(337, 154, 2.0, 2525);s.store_offset_div_ad(90, A::ln(s.ad_value(336)), s.ad_value(337), p.p456);s.store_offset_sub(781, 90, 89, (-0.0008));s.store_scale(782, 90, (4.0 * 0.0008));}
        if (((((!s.b[1443]) && s.b[2519]) && (!s.b[2538])) && (!s.b[2541])) && (!s.b[2542])) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }
        if (((((!s.b[1443]) && s.b[2519]) && (!s.b[2538])) && (!s.b[2541])) && (!s.b[2542])) {s.store_sqrt_square_add(782, 781, 782);s.store_add_scaled_inputs3_indices(88, 90, 1.0, 781, (-0.5), 782, (-0.5));}
        if (((!s.b[1443]) && s.b[2519]) && (!s.b[2538])) {s.store_offset(332, 2539, (1e-12 / 2.0));}
        s.b[2543] = (s.v[88] < s.v[332]);s.store_scalar(2543, if s.b[2543] { 1.0 } else { 0.0 });
        if ((((!s.b[1443]) && s.b[2519]) && (!s.b[2538])) && s.b[2543]) {s.copy_ad(88, 332);}
        if (((!s.b[1443]) && s.b[2519]) && (!s.b[2538])) {s.copy_ad(2523, 88);}
        s.b[2544] = (p.p451 == 1.0);s.store_scalar(2544, if s.b[2544] { 1.0 } else { 0.0 });
        if ((((!s.b[1443]) && s.b[2519]) && (!s.b[2538])) && s.b[2544]) {s.copy_ad(88, 2523);s.copy_ad(2545, 993);}
        let (t67,) = {
    if ((((!s.b[1443]) && s.b[2519]) && (!s.b[2538])) && s.b[2544]) {
        let t63: f64 = (s.v[160] - s.v[120]);let t64: f64 = (t63 + s.v[182]);let t65: f64 = (t64 + s.v[2545]);let t66: f64 = (t65 + p.p455);
        (t66,)
    } else {
        (s.v[86],)
    }
};
        s.store_scalar(86, t67);s.b[2554] = (s.v[791] < s.v[86]);s.store_scalar(2554, if s.b[2554] { 1.0 } else { 0.0 });
        let (t69,) = {
    if (((((!s.b[1443]) && s.b[2519]) && (!s.b[2538])) && s.b[2544]) && s.b[2554]) {
        let t68: f64 = (-1.0);
        (t68,)
    } else {
        (s.v[347],)
    }
};
        s.store_scalar(347, t69);
        if (((((!s.b[1443]) && s.b[2519]) && (!s.b[2538])) && s.b[2544]) && s.b[2554]) {s.store_mul_scaled_ln_ad_rhs(271, 155, 2.0, A::div_from_scalar((-s.v[270]), s.ad_value(212)));s.store_mul_sub_rhs(332, 154, 2525, 2545);s.store_div_scalar_by_product_indices(335, 1.0, 154, 209, 1.0);s.store_mul(333, 335, 185);s.store_offset_scaled(279, 333, (3.0 * 1.414213562373095), 2.0);s.store_mul3_affine_lhs(277, 279, 279, 8.0, 0.0, 279);s.store_offset(338, 332, (-2.0));s.store_scaled_mul(339, 333, 338, 9.0);s.store_sub_from_scalar(278, (7.0 * 1.414213562373095), 339);s.store_square(276, 278);}
        s.b[2555] = (s.v[277] < (s.v[276] * 1e-8));s.store_scalar(2555, if s.b[2555] { 1.0 } else { 0.0 });
        if ((((((!s.b[1443]) && s.b[2519]) && (!s.b[2538])) && s.b[2544]) && s.b[2554]) && s.b[2555]) {s.store_add_scaled_inputs3_offset_mixed_iai(274, 278, 1.0, A::div_scaled_inputs(s.ad_value(277), 0.5, s.ad_value(278), 1.0), 1.0, 339, 1.0, ((-7.0) * 1.414213562373095));}
        if ((((((!s.b[1443]) && s.b[2519]) && (!s.b[2538])) && s.b[2544]) && s.b[2554]) && (!s.b[2555])) {s.store_sqrt_add(275, 277, 276);s.store_add_offset_lhs(274, 275, ((-7.0) * 1.414213562373095), 339);}
        if (((((!s.b[1443]) && s.b[2519]) && (!s.b[2538])) && s.b[2544]) && s.b[2554]) {
            if (s.v[274] == 0.0) {
                s.store_scalar(273, 0.0);
            } else {
                s.store_powf(273, 274, 0.3333333333333333);
            }
        }
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_127(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((((!s.b[1443]) && s.b[2519]) && (!s.b[2538])) && s.b[2544]) && s.b[2554]) {s.store_add_scaled_inputs_product_mixed_aiii(272, A::sub_from_scalar(((-4.0) * 1.414213562373095), A::scale(s.ad_value(333), 12.0)), 1.0, 273, 2.0, 273, 273, 1.414213562373095);s.store_div_from_scalar(335, 1.0, 273);s.store_mul(116, 272, 335);s.store_add_scaled_product_indices(167, 2545, 1.0, 116, 155, 1.0);s.store_sub(335, 167, 2545);s.store_div(336, 335, 271);s.store_sqrt_square_offset(337, 336, 1.0);s.store_add_div_lhs_indices(2523, 335, 337, 2545);}
        if (((((!s.b[1443]) && s.b[2519]) && (!s.b[2538])) && s.b[2544]) && (!s.b[2554])) {s.store_exp_ad(230, A::mul_offset_rhs(s.ad_value(154), s.ad_value(2545), (-p.p456)));}
        let (t6a,) = {
    if (((((!s.b[1443]) && s.b[2519]) && (!s.b[2538])) && s.b[2544]) && (!s.b[2554])) {
        (0.0,)
    } else {
        (s.v[79],)
    }
};
        s.store_scalar(79, t6a);
        if (((((!s.b[1443]) && s.b[2519]) && (!s.b[2538])) && s.b[2544]) && (!s.b[2554])) {s.copy_ad(2546, 88);s.store_mul3_affine_lhs(2547, 166, 2524, (0.5 * 9662367879.197212), 0.0, 2524);s.store_sqrt_mul_scaled_lhs(334, 154, 2.0, 2547);s.store_scaled_add_ad(335, A::limited_exp(s.ad_value(334)), A::limited_exp_scaled_input(s.ad_value(334), -1.0), 0.5);s.store_div_ln_lhs(2548, 335, 2547);}
        let (t6b,) = {
    if (((((!s.b[1443]) && s.b[2519]) && (!s.b[2538])) && s.b[2544]) && (!s.b[2554])) {
        (1.0,)
    } else {
        (s.v[97],)
    }
};
        s.store_scalar(97, t6b);
    }
}
