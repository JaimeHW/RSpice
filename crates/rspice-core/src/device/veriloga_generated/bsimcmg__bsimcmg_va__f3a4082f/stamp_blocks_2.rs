#![allow(dead_code, non_snake_case, unused_assignments, unused_imports, unused_parens, unused_variables)]
use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, Scratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};
impl Instance {
    #[inline(never)]
    pub(super) fn stamp_transient_block_32(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        let ctx_temp = ctx.temperature();
        if s.b[1288] {let t0: A = A::scale_offset(s.ad_value(962), ((0.5) * (2.0)), ((((1.0) + ((-1.0)))) * (2.0)));s.store_primal_mul_product3_mixed_iiaa(1012, 981, 962, A::div(A::pow_from_scalar(3.14, A::scale(s.ad_value(962), 0.5)), A::offset(A::sub(A::add_scaled_inputs(A::sub_scaled_inputs(A::add_scaled_inputs(A::add_scaled_inputs4(A::exp_scaled_input(A::scale_offset(s.ad_value(962), 0.5, ((1.0) + ((-1.0)))), (-4.6)), 0.0385, A::powi(A::scale_offset(s.ad_value(962), ((0.5) * (2.0)), ((2.0) + ((-3.0)))), 8), 7.5893e-7, A::powi(t0, 6), 6.9583e-5, A::powi(t0, 5), (-0.0006583)), 1.0, A::pow4(t0), 0.0065), 1.0, A::cube(t0), 0.026), 1.0, A::square(t0), 0.1371), A::scale_offset(s.ad_value(962), ((0.5) * ((0.194 * 2.0))), ((((1.0) + ((-1.0)))) * ((0.194 * 2.0))))), 0.959)), A::pow(A::scale(s.ad_value(1003), 1000000.0), s.ad_value(962)), (1.0 / (2.0) * 1.60219e-19));}
        s.b[1289] = (p.p58 == 1.0);s.store_scalar(1289, if s.b[1289] { 1.0 } else { 0.0 });
        if s.b[1289] {s.store_primal_offset_scaled(707, 707, 1.0 / (({ let limited_exp_arg = (((p.p890 * 1000000000.0) - (p.p40 * 1000000000.0)) / p.p891); if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } } + 1.0)), (((((-p.p889)) * (1.0 / (({ let limited_exp_arg = (((p.p890 * 1000000000.0) - (p.p40 * 1000000000.0)) / p.p891); if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } } + 1.0))))) + (p.p889)));}
        if s.b[1289] {s.store_offset(1024, 807, (((-p.p892)) + ((-((p.p893 * 1000000000.0) * p.p894)))));}
        if s.b[1289] {s.store_scaled_offset(1025, 1024, ((p.p40 * 1000000000.0) * p.p894), 1.0 / ((1.0 + { let limited_exp_arg = (((p.p895 * 1000000000.0) - (p.p40 * 1000000000.0)) / p.p896); if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } })));}
        if s.b[1289] {s.store_add_scaled_inputs3_offset_mixed_iia(807, 1025, 0.5, 807, 0.5, A::sqrt_square_offset(A::sub(A::offset(s.ad_value(1025), p.p892), A::offset(s.ad_value(807), 0.2)), ((0.25 * 0.6) * 0.6)), (-0.5), ((p.p892 + 0.2) * 0.5));}
        if s.b[1289] {s.store_add_scaled_inputs3_offset_indices(1026, 811, (-(370.0 * 1.0 / ((((p.p40 * 1000000000.0)) as f64).powf(p.p898)))), 811, (-1.0 / ((1.0 + { let limited_exp_arg = (((p.p40 * 1000000000.0) - (p.p899 * 1000000000.0)) / p.p900); if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } }))), 811, 1.0, (((p.p897) * ((370.0 * 1.0 / ((((p.p40 * 1000000000.0)) as f64).powf(p.p898))))) + ((p.p897) * (1.0 / ((1.0 + { let limited_exp_arg = (((p.p40 * 1000000000.0) - (p.p899 * 1000000000.0)) / p.p900); if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } }))))));}
        if s.b[1289] {s.store_scaled_sub_offset_sqrt_square_offset(811, 1026, p.p897, (-p.p897), ((0.25 * 0.2) * 0.2), 0.5);s.store_scalar(1027, (p.p43 / (p.p43 + p.p40)));s.store_scalar(1028, ((((p.p905 * p.p40) * p.p40) * 1e18) - (p.p906 * 0.001)));s.store_primal_scaled_add_mixed_ia(1029, 1028, A::powf(A::offset(A::square(s.ad_value(1028)), ((((((4.0 * p.p906) * 0.001) * (p.p905 + 0.24)) * p.p40) * p.p40) * 1e18)), 0.5), 1.0 / (((((2.0 * (p.p905 + 0.24)) * p.p40) * p.p40) * 1e18)));s.store_primal_scaled_sub_offset_sqrt_square_offset_ad(1030, A::div_scalar_offset_denominator(0.0001, s.ad_value(1029), (((-0.8208)) + ((-(p.p907 * 1e-5)))), 1.0), 1.0, (-1.0), ((0.25 * 0.06) * 0.06), 0.5);s.store_mul_ad_product_lhs_mixed_ia(704, 704, A::add(s.ad_value(1027), A::scale_offset(s.ad_value(1027), (-p.p904), p.p904)), 1030);s.store_add_mixed_ai(812, A::scale_offset(s.ad_value(812), (-(((0.5 * (((p.p902 * 1000000000.0) - (p.p40 * 1000000000.0)) + ((((((p.p902 * 1000000000.0) - (p.p40 * 1000000000.0)) * ((p.p902 * 1000000000.0) - (p.p40 * 1000000000.0))) + 0.25)) as f64).sqrt()))) as f64).powf(p.p903)), ((p.p901) * ((((0.5 * (((p.p902 * 1000000000.0) - (p.p40 * 1000000000.0)) + ((((((p.p902 * 1000000000.0) - (p.p40 * 1000000000.0)) * ((p.p902 * 1000000000.0) - (p.p40 * 1000000000.0))) + 0.25)) as f64).sqrt()))) as f64).powf(p.p903)))), 812);}
        s.b[1290] = ((p.p74 != 0.0) && (p.p1791 > 0.0));s.store_scalar(1290, if s.b[1290] { 1.0 } else { 0.0 });
        if s.b[1290] {s.store_offset_voltage(116, ctx, nodes, Some(4), None, ((ctx_temp) + (p.p22)));}
        if (!s.b[1290]) {s.store_scalar(116, (ctx_temp + p.p22));}
        s.store_div(229, 116, 228);s.store_offset(230, 229, (-1.0));s.store_sub(232, 116, 228);s.store_scale(179, 116, 8.617087e-5);s.store_primal_scale(180, 228, 8.617087e-5);s.store_scalar(121, p.p1786);s.b[1291] = (p.p80 != 0.0);s.store_scalar(1291, if s.b[1291] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_33(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if s.b[1291] {s.store_scaled_add_offset_sqrt_square_offset(119, 116, s.v[121], (-s.v[121]), ((0.25 * p.p1788) * p.p1788), 0.5);s.store_scaled_add_sqrt_square_offset_ad(120, A::scaled_offset(s.ad_value(116), (-p.p1787), (-p.p1790)), ((0.25 * p.p1789) * p.p1789), 0.5);}
        s.b[1292] = (p.p80 == 1.0);s.store_scalar(1292, if s.b[1292] { 1.0 } else { 0.0 });
        if (s.b[1291] && s.b[1292]) {s.store_scaled_add_offset_sqrt_square_offset(169, 228, s.v[121], (-s.v[121]), ((0.25 * p.p1788) * p.p1788), 0.5);s.store_scaled_add_sqrt_square_offset_ad(170, A::scaled_offset(s.ad_value(228), (-p.p1787), (-p.p1790)), ((0.25 * p.p1789) * p.p1789), 0.5);}
        s.b[1293] = (s.v[228] > s.v[121]);s.store_scalar(1293, if s.b[1293] { 1.0 } else { 0.0 });
        if ((s.b[1291] && s.b[1292]) && s.b[1293]) {s.store_add_mixed_ai(171, A::add_scaled_inputs4(s.ad_value(119), 1.0, s.ad_value(120), 1.0, s.ad_value(169), -1.0, s.ad_value(170), -1.0), 228);}
        if ((s.b[1291] && s.b[1292]) && (!s.b[1293])) {s.store_add_scaled_inputs4_offset_indices(171, 119, 1.0, 120, 1.0, 169, -1.0, 170, -1.0, s.v[121]);}
        if (s.b[1291] && s.b[1292]) {s.store_add_scaled_inputs3_sqrt_third_sub_square_offset(118, 116, 0.5, 171, 0.5, 116, 171, ((0.25 * 0.2) * 0.2), 0.5);}
        s.b[1294] = (s.v[121] > 210.0);s.store_scalar(1294, if s.b[1294] { 1.0 } else { 0.0 });
        if ((s.b[1291] && (!s.b[1292])) && s.b[1294]) {s.store_scalar(121, 210.0);}
        if (s.b[1291] && (!s.b[1292])) {s.store_offset_scaled_ad(312, A::tanh_scaled_input(A::offset(s.ad_value(116), (-210.0)), 0.5), 0.5, 0.5);s.store_sub_from_scalar(313, 1.0, 312);}
        s.b[1295] = (s.v[228] > 210.0);s.store_scalar(1295, if s.b[1295] { 1.0 } else { 0.0 });
        if ((s.b[1291] && (!s.b[1292])) && s.b[1295]) {s.store_scaled_add_ad(169, A::offset(s.ad_value(121), 210.0), A::sqrt_square_offset(A::sub_from_scalar(210.0, s.ad_value(121)), ((0.25 * p.p1788) * p.p1788)), 0.5);s.store_scalar(170, (0.5 * (((-p.p1790) * (210.0 - p.p1787)) + ((((((-p.p1790) * (210.0 - p.p1787)) * ((-p.p1790) * (210.0 - p.p1787))) + ((0.25 * p.p1789) * p.p1789))) as f64).sqrt())));s.store_add_scaled_inputs4_offset_indices(171, 119, 1.0, 120, 1.0, 169, -1.0, 170, -1.0, 210.0);s.store_add_scaled_inputs3_sqrt_third_sub_square_offset(118, 116, 0.5, 171, 0.5, 116, 171, ((0.25 * 0.2) * 0.2), 0.5);}
        if ((s.b[1291] && (!s.b[1292])) && (!s.b[1295])) {s.store_add_scaled_inputs3_sqrt_third_sub_square_offset(169, 228, 0.5, 121, 0.5, 228, 121, ((0.25 * p.p1788) * p.p1788), 0.5);s.store_scaled_add_sqrt_square_offset_ad(170, A::scaled_offset(s.ad_value(228), (-p.p1787), (-p.p1790)), ((0.25 * p.p1789) * p.p1789), 0.5);}
        s.b[1296] = (s.v[228] > s.v[121]);s.store_scalar(1296, if s.b[1296] { 1.0 } else { 0.0 });
        if (((s.b[1291] && (!s.b[1292])) && (!s.b[1295])) && s.b[1296]) {s.store_add_mixed_ai(171, A::add_scaled_inputs4(s.ad_value(119), 1.0, s.ad_value(120), 1.0, s.ad_value(169), -1.0, s.ad_value(170), -1.0), 228);}
        if (((s.b[1291] && (!s.b[1292])) && (!s.b[1295])) && (!s.b[1296])) {s.store_add_mixed_ai(171, A::add_scaled_inputs4(s.ad_value(119), 1.0, s.ad_value(120), 1.0, s.ad_value(169), -1.0, s.ad_value(170), -1.0), 121);}
        if ((s.b[1291] && (!s.b[1292])) && (!s.b[1295])) {s.store_add_scaled_inputs3_sqrt_third_sub_square_offset(172, 116, 0.5, 171, 0.5, 116, 171, ((0.25 * 0.2) * 0.2), 0.5);s.store_add_scaled_products_indices(118, 313, 172, 1.0, 312, 116, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_34(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (s.b[1291] && (!s.b[1292])) {s.store_scaled_sub_offset_sqrt_square_offset(117, 116, 210.0, (-210.0), ((0.25 * 0.2) * 0.2), 0.5);s.store_add_scaled_inputs3_offset_mixed_iia(233, 117, 1.0, 228, (-0.5), A::sqrt_square_offset(A::offset(s.ad_value(228), (-210.0)), ((0.25 * 0.2) * 0.2)), (-(-0.5)), ((-0.5) * 210.0));s.store_div_scaled_offset_numerator_indices(234, 117, 1.0, (-210.0), 228, 1.0);}
        if s.b[1291] {s.store_scale(182, 118, 8.617087e-5);}
        s.store_sub_from_scalar_ad(146, p.p106, A::div_scaled_product_offset_denominator(s.ad_value(116), s.ad_value(116), p.p1718, s.ad_value(116), p.p1719, 1.0));s.store_primal_sub_from_scalar_ad(147, p.p106, A::div_scaled_product_offset_denominator(s.ad_value(228), s.ad_value(228), p.p1718, s.ad_value(228), p.p1719, 1.0));s.store_mul_scaled_sqrt_scaled_input_rhs(169, 116, 1.0 / (300.15), 116, 1.0 / (300.15));s.store_mul_scaled_limited_exp_ad_rhs(141, 169, p.p105, A::sub_from_scalar((p.p106 / ((2.0 * 8.617087e-5) * 300.15)), A::div_scaled_inputs(s.ad_value(146), 1.0, s.ad_value(179), 2.0)));s.b[1297] = (p.p80 == 0.0);s.store_scalar(1297, if s.b[1297] { 1.0 } else { 0.0 });
        if s.b[1297] {s.store_scale(148, 169, p.p107);}
        if (!s.b[1297]) {s.store_mul_scaled_sqrt_scaled_input_rhs(148, 118, (1.0 / (300.15) * p.p107), 118, 1.0 / (300.15));}
        if (!s.b[1297]) {
            s.store_sub_ad(142, A::offset({
                if (!((p.p105 * s.v[169]) > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if ((p.p105 * s.v[169]) > 1e-38) {
                            A::ln_scaled_input(s.ad_value(169), p.p105)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, (p.p106 / ((2.0 * 8.617087e-5) * 300.15))), A::div_scaled_inputs(s.ad_value(146), 1.0, s.ad_value(179), 2.0));
        }
        if (!(((1.0 + (s.v[859] * s.v[232])) - 1e-6) < ((-10000.0) * 0.001))) {
            s.store_scaled_add_sqrt_square_offset_ad(235, A::offset(A::mul(s.ad_value(859), s.ad_value(232)), ((1.0) + ((-1e-6)))), ((4.0 * 0.001) * 0.001), 0.5);
        } else {
            if (((1.0 + (s.v[859] * s.v[232])) - 1e-6) < ((-10000.0) * 0.001)) {
                s.store_div_from_scalar_offset_product(235, ((-0.001) * 0.001), 859, 232, ((1.0) + ((-1e-6))));
            } else {
                s.store_scalar(235, 0.0);
            }
        }
        s.store_scale(389, 179, 1.60219e-19);s.store_div_from_scalar_ad(168, (1.05457e-34 * 3.141592653589793), A::div_scaled_inputs(s.ad_value(894), 2.0, s.ad_value(895), 1.0));s.store_scaled_square(377, 168, 1.0 / ((2.0 * s.v[381])));s.store_scaled_square(378, 168, 1.0 / ((2.0 * s.v[382])));s.store_scale(379, 377, 4.0);s.store_scale(380, 378, 4.0);s.store_scalar(169, ((s.v[385] * s.v[384]) / (s.v[386] * s.v[383])));s.store_offset_scaled_ad(387, A::limited_exp(A::div_scaled_inputs2(s.ad_value(377), 1.0, s.ad_value(378), (-1.0), s.ad_value(389), 1.0)), s.v[169], 1.0);s.store_add_scaled_inputs3_mixed_iaa(388, 387, 1.0, A::limited_exp(A::div_scaled_inputs2(s.ad_value(377), 1.0, s.ad_value(379), (-1.0), s.ad_value(389), 1.0)), 1.0, A::limited_exp(A::div_scaled_inputs2(s.ad_value(377), 1.0, s.ad_value(380), (-1.0), s.ad_value(389), 1.0)), s.v[169]);
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_35(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.store_mul_scale_offset_mixed_ia(170, 179, {
            if (!((((((s.v[386] * s.v[383]) / (((3.141592653589793 * 1.05457e-34) * 1.05457e-34) * s.v[148])) * s.v[389]) / ((2.0 * s.v[894]) / s.v[895])) * s.v[388]) > 1e-38)) {
                A::neg(A::constant(87.498233534))
            } else {
                {
                    if ((((((s.v[386] * s.v[383]) / (((3.141592653589793 * 1.05457e-34) * 1.05457e-34) * s.v[148])) * s.v[389]) / ((2.0 * s.v[894]) / s.v[895])) * s.v[388]) > 1e-38) {
                        A::ln(A::mul(A::div_scaled_value_by_product(s.ad_value(389), (s.v[386] * s.v[383]), A::scale(s.ad_value(148), ((3.141592653589793 * 1.05457e-34) * 1.05457e-34)), A::div_scaled_inputs(s.ad_value(894), 2.0, s.ad_value(895), 1.0), 1.0), s.ad_value(388)))
                    } else {
                        A::constant(0.0)
                    }
                }
            }
        }, -1.0, 0.0);s.store_mul_add_scaled_inputs_rhs_indices(375, 654, 377, 6.241457005723417e18, 170, 1.0);s.store_ln(418, 229);s.b[1298] = (p.p80 == 0.0);s.store_scalar(1298, if s.b[1298] { 1.0 } else { 0.0 });
        if s.b[1298] {s.store_mul_exp_mixed_ia(169, 704, A::mul(s.ad_value(836), s.ad_value(418)));s.store_add_scaled_inputs4_offset_mixed_iiaa(413, 169, 1.0, 169, (-0.9), A::add_scaled_product(s.ad_value(169), (-(-0.9)), s.ad_value(838), s.ad_value(232), 1.0), 0.5, A::sqrt(A::sub_scaled_inputs(A::square(A::offset(A::add_scaled_product(s.ad_value(169), (-(-0.9)), s.ad_value(838), s.ad_value(232), 1.0), (-0.0001))), 1.0, s.ad_value(169), ((-0.9) * (4.0 * 0.0001)))), 0.5, (0.5 * (-0.0001)));}
        s.b[1299] = (p.p66 == 1.0);s.store_scalar(1299, if s.b[1299] { 1.0 } else { 0.0 });
        if (s.b[1298] && s.b[1299]) {s.store_mul_exp_mixed_ia(169, 706, A::mul(s.ad_value(845), s.ad_value(418)));s.store_add_scaled_inputs4_offset_mixed_iiaa(321, 169, 1.0, 169, (-0.9), A::add_scaled_product(s.ad_value(169), (-(-0.9)), s.ad_value(846), s.ad_value(232), 1.0), 0.5, A::sqrt(A::sub_scaled_inputs(A::square(A::offset(A::add_scaled_product(s.ad_value(169), (-(-0.9)), s.ad_value(846), s.ad_value(232), 1.0), (-0.0001))), 1.0, s.ad_value(169), ((-0.9) * (4.0 * 0.0001)))), 0.5, (0.5 * (-0.0001)));s.copy_ad(417, 321);}
        if s.b[1298] {s.store_add_scaled_inputs4_offset_mixed_iaai(303, 807, 1.0, A::add_scaled_product(s.ad_value(807), 1.0, s.ad_value(823), s.ad_value(232), 1.0), 0.5, A::sqrt(A::sub_scaled_inputs(A::square(A::offset(A::add_scaled_product(s.ad_value(807), 1.0, s.ad_value(823), s.ad_value(232), 1.0), (-1e-6))), 1.0, s.ad_value(807), (-(4.0 * 1e-6)))), 0.5, 807, (-1.0), (0.5 * (-1e-6)));s.copy_ad(323, 811);}
        s.b[1300] = (p.p66 != 0.0);s.store_scalar(1300, if s.b[1300] { 1.0 } else { 0.0 });
        if (s.b[1298] && s.b[1300]) {s.store_add_scaled_inputs4_offset_mixed_iaai(305, 815, 1.0, A::add_scaled_product(s.ad_value(815), 1.0, s.ad_value(825), s.ad_value(232), 1.0), 0.5, A::sqrt(A::sub_scaled_inputs(A::square(A::offset(A::add_scaled_product(s.ad_value(815), 1.0, s.ad_value(825), s.ad_value(232), 1.0), (-1e-6))), 1.0, s.ad_value(815), (-(4.0 * 1e-6)))), 0.5, 815, (-1.0), (0.5 * (-1e-6)));}
        if s.b[1298] {s.store_mul_exp_mixed_ia(318, 812, A::mul(s.ad_value(830), s.ad_value(418)));}
        s.b[1301] = (p.p66 != 0.0);s.store_scalar(1301, if s.b[1301] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_36(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (s.b[1298] && s.b[1301]) {s.store_mul_exp_mixed_ia(320, 818, A::mul(s.ad_value(844), s.ad_value(418)));}
        if s.b[1298] {s.store_mul_exp_mixed_ia(317, 814, A::mul(s.ad_value(834), s.ad_value(418)));}
        if s.b[1298] {
            if (!(((1.0 + (s.v[854] * s.v[232])) - 1e-6) < ((-10000.0) * 0.001))) {
                s.store_scaled_add_sqrt_square_offset_ad(194, A::offset(A::mul(s.ad_value(854), s.ad_value(232)), ((1.0) + ((-1e-6)))), ((4.0 * 0.001) * 0.001), 0.5);
            } else {
                if (((1.0 + (s.v[854] * s.v[232])) - 1e-6) < ((-10000.0) * 0.001)) {
                    s.store_div_from_scalar_offset_product(194, ((-0.001) * 0.001), 854, 232, ((1.0) + ((-1e-6))));
                } else {
                    s.store_scalar(194, 0.0);
                }
            }
        }
        s.b[1302] = (p.p75 != 0.0);s.store_scalar(1302, if s.b[1302] { 1.0 } else { 0.0 });
        if (s.b[1298] && s.b[1302]) {s.store_add_scaled_inputs4_offset_mixed_iaai(332, 679, 1.0, A::add_scaled_product(s.ad_value(679), 1.0, s.ad_value(849), s.ad_value(232), -1.0), 0.5, A::sqrt(A::sub_scaled_inputs(A::square(A::offset(A::add_scaled_product(s.ad_value(679), 1.0, s.ad_value(849), s.ad_value(232), -1.0), (-1e-6))), 1.0, s.ad_value(679), (-(4.0 * 1e-6)))), 0.5, 679, (-1.0), (0.5 * (-1e-6)));}
        if (s.b[1298] && (!s.b[1302])) {
            s.store_mul_mixed_ia(332, 679, {
                            if (!(((1.0 + ((-s.v[849]) * s.v[232])) - 1e-6) < ((-10000.0) * 0.001))) {
                                A::add_scaled_inputs(A::offset(A::mul_scaled_lhs(s.ad_value(849), -1.0, s.ad_value(232)), ((1.0) + ((-1e-6)))), 0.5, A::sqrt_square_offset(A::offset(A::mul_scaled_lhs(s.ad_value(849), -1.0, s.ad_value(232)), ((1.0) + ((-1e-6)))), ((4.0 * 0.001) * 0.001)), 0.5)
                            } else {
                                {
                                    if (((1.0 + ((-s.v[849]) * s.v[232])) - 1e-6) < ((-10000.0) * 0.001)) {
                                        A::div_scalar_offset_denominator(((-0.001) * 0.001), A::mul_scaled_lhs(s.ad_value(849), -1.0, s.ad_value(232)), ((1.0) + ((-1e-6))), 1.0)
                                    } else {
                                        A::constant(0.0)
                                    }
                                }
                            }
                        });
        }
        s.b[1303] = (p.p66 != 0.0);s.store_scalar(1303, if s.b[1303] { 1.0 } else { 0.0 });s.b[1304] = (p.p75 != 0.0);s.store_scalar(1304, if s.b[1304] { 1.0 } else { 0.0 });
        if ((s.b[1298] && s.b[1303]) && s.b[1304]) {s.store_add_scaled_inputs4_offset_mixed_iaai(333, 680, 1.0, A::add_scaled_product(s.ad_value(680), 1.0, s.ad_value(851), s.ad_value(232), -1.0), 0.5, A::sqrt(A::sub_scaled_inputs(A::square(A::offset(A::add_scaled_product(s.ad_value(680), 1.0, s.ad_value(851), s.ad_value(232), -1.0), (-1e-6))), 1.0, s.ad_value(680), (-(4.0 * 1e-6)))), 0.5, 680, (-1.0), (0.5 * (-1e-6)));}
        if ((s.b[1298] && s.b[1303]) && (!s.b[1304])) {
            s.store_mul_mixed_ia(333, 680, {
                            if (!(((1.0 + ((-s.v[851]) * s.v[232])) - 1e-6) < ((-10000.0) * 0.001))) {
                                A::add_scaled_inputs(A::offset(A::mul_scaled_lhs(s.ad_value(851), -1.0, s.ad_value(232)), ((1.0) + ((-1e-6)))), 0.5, A::sqrt_square_offset(A::offset(A::mul_scaled_lhs(s.ad_value(851), -1.0, s.ad_value(232)), ((1.0) + ((-1e-6)))), ((4.0 * 0.001) * 0.001)), 0.5)
                            } else {
                                {
                                    if (((1.0 + ((-s.v[851]) * s.v[232])) - 1e-6) < ((-10000.0) * 0.001)) {
                                        A::div_scalar_offset_denominator(((-0.001) * 0.001), A::mul_scaled_lhs(s.ad_value(851), -1.0, s.ad_value(232)), ((1.0) + ((-1e-6))), 1.0)
                                    } else {
                                        A::constant(0.0)
                                    }
                                }
                            }
                        });
        }
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_37(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.b[1305] = (s.v[333] < 1000.0);s.store_scalar(1305, if s.b[1305] { 1.0 } else { 0.0 });
        if ((s.b[1298] && s.b[1303]) && s.b[1305]) {s.store_scalar(333, 1000.0);}
        s.b[1306] = (p.p67 == 1.0);s.store_scalar(1306, if s.b[1306] { 1.0 } else { 0.0 });
        if (s.b[1298] && s.b[1306]) {s.store_mul_exp_mixed_ia(169, 705, A::mul(s.ad_value(839), s.ad_value(418)));s.store_add_scaled_inputs4_offset_mixed_iiaa(414, 169, 1.0, 169, (-0.9), A::add_scaled_product(s.ad_value(169), (-(-0.9)), s.ad_value(841), s.ad_value(232), 1.0), 0.5, A::sqrt(A::sub_scaled_inputs(A::square(A::offset(A::add_scaled_product(s.ad_value(169), (-(-0.9)), s.ad_value(841), s.ad_value(232), 1.0), (-0.0001))), 1.0, s.ad_value(169), ((-0.9) * (4.0 * 0.0001)))), 0.5, (0.5 * (-0.0001)));s.store_add_scaled_inputs4_offset_mixed_iaai(304, 808, 1.0, A::add_scaled_product(s.ad_value(808), 1.0, s.ad_value(826), s.ad_value(232), 1.0), 0.5, A::sqrt(A::sub_scaled_inputs(A::square(A::offset(A::add_scaled_product(s.ad_value(808), 1.0, s.ad_value(826), s.ad_value(232), 1.0), (-1e-6))), 1.0, s.ad_value(808), (-(4.0 * 1e-6)))), 0.5, 808, (-1.0), (0.5 * (-1e-6)));s.store_mul_exp_mixed_ia(319, 813, A::mul(s.ad_value(832), s.ad_value(418)));}
        s.b[1307] = (p.p75 != 0.0);s.store_scalar(1307, if s.b[1307] { 1.0 } else { 0.0 });
        if (s.b[1298] && s.b[1307]) {s.store_add_scaled_inputs4_offset_mixed_iaai(334, 698, 1.0, A::add_scaled_product(s.ad_value(698), 1.0, s.ad_value(849), s.ad_value(232), -1.0), 0.5, A::sqrt(A::sub_scaled_inputs(A::square(A::offset(A::add_scaled_product(s.ad_value(698), 1.0, s.ad_value(849), s.ad_value(232), -1.0), (-1e-6))), 1.0, s.ad_value(698), (-(4.0 * 1e-6)))), 0.5, 698, (-1.0), (0.5 * (-1e-6)));}
        if (s.b[1298] && (!s.b[1307])) {
            s.store_mul_mixed_ia(334, 698, {
                            if (!(((1.0 + ((-s.v[849]) * s.v[232])) - 1e-6) < ((-10000.0) * 0.001))) {
                                A::add_scaled_inputs(A::offset(A::mul_scaled_lhs(s.ad_value(849), -1.0, s.ad_value(232)), ((1.0) + ((-1e-6)))), 0.5, A::sqrt_square_offset(A::offset(A::mul_scaled_lhs(s.ad_value(849), -1.0, s.ad_value(232)), ((1.0) + ((-1e-6)))), ((4.0 * 0.001) * 0.001)), 0.5)
                            } else {
                                {
                                    if (((1.0 + ((-s.v[849]) * s.v[232])) - 1e-6) < ((-10000.0) * 0.001)) {
                                        A::div_scalar_offset_denominator(((-0.001) * 0.001), A::mul_scaled_lhs(s.ad_value(849), -1.0, s.ad_value(232)), ((1.0) + ((-1e-6))), 1.0)
                                    } else {
                                        A::constant(0.0)
                                    }
                                }
                            }
                        });
        }
        s.b[1308] = (p.p66 != 0.0);s.store_scalar(1308, if s.b[1308] { 1.0 } else { 0.0 });s.b[1309] = (p.p75 != 0.0);s.store_scalar(1309, if s.b[1309] { 1.0 } else { 0.0 });
        if ((s.b[1298] && s.b[1308]) && s.b[1309]) {s.store_add_scaled_inputs4_offset_mixed_iaai(335, 699, 1.0, A::add_scaled_product(s.ad_value(699), 1.0, s.ad_value(849), s.ad_value(232), -1.0), 0.5, A::sqrt(A::sub_scaled_inputs(A::square(A::offset(A::add_scaled_product(s.ad_value(699), 1.0, s.ad_value(849), s.ad_value(232), -1.0), (-1e-6))), 1.0, s.ad_value(699), (-(4.0 * 1e-6)))), 0.5, 699, (-1.0), (0.5 * (-1e-6)));}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_38(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((s.b[1298] && s.b[1308]) && (!s.b[1309])) {
            s.store_mul_mixed_ia(335, 699, {
                            if (!(((1.0 + ((-s.v[849]) * s.v[232])) - 1e-6) < ((-10000.0) * 0.001))) {
                                A::add_scaled_inputs(A::offset(A::mul_scaled_lhs(s.ad_value(849), -1.0, s.ad_value(232)), ((1.0) + ((-1e-6)))), 0.5, A::sqrt_square_offset(A::offset(A::mul_scaled_lhs(s.ad_value(849), -1.0, s.ad_value(232)), ((1.0) + ((-1e-6)))), ((4.0 * 0.001) * 0.001)), 0.5)
                            } else {
                                {
                                    if (((1.0 + ((-s.v[849]) * s.v[232])) - 1e-6) < ((-10000.0) * 0.001)) {
                                        A::div_scalar_offset_denominator(((-0.001) * 0.001), A::mul_scaled_lhs(s.ad_value(849), -1.0, s.ad_value(232)), ((1.0) + ((-1e-6))), 1.0)
                                    } else {
                                        A::constant(0.0)
                                    }
                                }
                            }
                        });
        }
        s.b[1310] = (s.v[335] < 1000.0);s.store_scalar(1310, if s.b[1310] { 1.0 } else { 0.0 });
        if ((s.b[1298] && s.b[1308]) && s.b[1310]) {s.store_scalar(335, 1000.0);}
        s.b[1311] = (p.p75 != 0.0);s.store_scalar(1311, if s.b[1311] { 1.0 } else { 0.0 });
        if (s.b[1298] && s.b[1311]) {s.store_add_scaled_inputs4_offset_mixed_iaai(336, 702, 1.0, A::add_scaled_product(s.ad_value(702), 1.0, s.ad_value(850), s.ad_value(232), -1.0), 0.5, A::sqrt(A::sub_scaled_inputs(A::square(A::offset(A::add_scaled_product(s.ad_value(702), 1.0, s.ad_value(850), s.ad_value(232), -1.0), (-1e-6))), 1.0, s.ad_value(702), (-(4.0 * 1e-6)))), 0.5, 702, (-1.0), (0.5 * (-1e-6)));}
        if (s.b[1298] && (!s.b[1311])) {
            s.store_mul_mixed_ia(336, 702, {
                            if (!(((1.0 + ((-s.v[850]) * s.v[232])) - 1e-6) < ((-10000.0) * 0.001))) {
                                A::add_scaled_inputs(A::offset(A::mul_scaled_lhs(s.ad_value(850), -1.0, s.ad_value(232)), ((1.0) + ((-1e-6)))), 0.5, A::sqrt_square_offset(A::offset(A::mul_scaled_lhs(s.ad_value(850), -1.0, s.ad_value(232)), ((1.0) + ((-1e-6)))), ((4.0 * 0.001) * 0.001)), 0.5)
                            } else {
                                {
                                    if (((1.0 + ((-s.v[850]) * s.v[232])) - 1e-6) < ((-10000.0) * 0.001)) {
                                        A::div_scalar_offset_denominator(((-0.001) * 0.001), A::mul_scaled_lhs(s.ad_value(850), -1.0, s.ad_value(232)), ((1.0) + ((-1e-6))), 1.0)
                                    } else {
                                        A::constant(0.0)
                                    }
                                }
                            }
                        });
        }
        if s.b[1298] {
            s.store_offset_ad(337, {
                if (!(((s.v[790] * (1.0 + (p.p450 * s.v[232]))) - 2.0) < ((-10000.0) * 0.001))) {
                    A::add_scaled_inputs(A::offset(A::mul(s.ad_value(790), A::scale_offset(s.ad_value(232), p.p450, 1.0)), (-2.0)), 0.5, A::sqrt_square_offset(A::offset(A::mul(s.ad_value(790), A::scale_offset(s.ad_value(232), p.p450, 1.0)), (-2.0)), ((4.0 * 0.001) * 0.001)), 0.5)
                } else {
                    {
                        if (((s.v[790] * (1.0 + (p.p450 * s.v[232]))) - 2.0) < ((-10000.0) * 0.001)) {
                            A::div_scalar_offset_denominator(((-0.001) * 0.001), A::mul(s.ad_value(790), A::scale_offset(s.ad_value(232), p.p450, 1.0)), (-2.0), 1.0)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 2.0);
        }
        s.b[1312] = (p.p66 != 0.0);s.store_scalar(1312, if s.b[1312] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_39(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (s.b[1298] && s.b[1312]) {
            s.store_offset_ad(338, {
                if (!(((s.v[791] * (1.0 + (p.p452 * s.v[232]))) - 2.0) < ((-10000.0) * 0.001))) {
                    A::add_scaled_inputs(A::offset(A::mul(s.ad_value(791), A::scale_offset(s.ad_value(232), p.p452, 1.0)), (-2.0)), 0.5, A::sqrt_square_offset(A::offset(A::mul(s.ad_value(791), A::scale_offset(s.ad_value(232), p.p452, 1.0)), (-2.0)), ((4.0 * 0.001) * 0.001)), 0.5)
                } else {
                    {
                        if (((s.v[791] * (1.0 + (p.p452 * s.v[232]))) - 2.0) < ((-10000.0) * 0.001)) {
                            A::div_scalar_offset_denominator(((-0.001) * 0.001), A::mul(s.ad_value(791), A::scale_offset(s.ad_value(232), p.p452, 1.0)), (-2.0), 1.0)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 2.0);
        }
        if s.b[1298] {s.copy_ad(660, 657);s.copy_ad(797, 792);s.store_mul_add_mixed_iia(231, 230, 858, A::div_from_scalar(p.p1720, s.ad_value(153)));}
        s.b[1313] = (p.p80 == 1.0);s.store_scalar(1313, if s.b[1313] { 1.0 } else { 0.0 });
        if ((!s.b[1298]) && s.b[1313]) {s.store_mul_exp_mixed_ia(169, 704, A::mul(A::add_scaled_product(s.ad_value(836), 1.0, s.ad_value(837), s.ad_value(229), 1.0), s.ad_value(418)));s.store_add_scaled_inputs4_offset_mixed_iiaa(413, 169, 1.0, 169, (-0.9), A::add_scaled_product(s.ad_value(169), (-(-0.9)), s.ad_value(838), s.ad_value(232), 1.0), 0.5, A::sqrt(A::sub_scaled_inputs(A::square(A::offset(A::add_scaled_product(s.ad_value(169), (-(-0.9)), s.ad_value(838), s.ad_value(232), 1.0), (-0.0001))), 1.0, s.ad_value(169), ((-0.9) * (4.0 * 0.0001)))), 0.5, (0.5 * (-0.0001)));}
        s.b[1314] = (p.p66 == 1.0);s.store_scalar(1314, if s.b[1314] { 1.0 } else { 0.0 });
        if (((!s.b[1298]) && s.b[1313]) && s.b[1314]) {s.store_mul_exp_mixed_ia(169, 706, A::mul(A::add_scaled_product(s.ad_value(845), 1.0, s.ad_value(837), s.ad_value(229), 1.0), s.ad_value(418)));s.store_add_scaled_inputs4_offset_mixed_iiaa(321, 169, 1.0, 169, (-0.9), A::add_scaled_product(s.ad_value(169), (-(-0.9)), s.ad_value(846), s.ad_value(232), 1.0), 0.5, A::sqrt(A::sub_scaled_inputs(A::square(A::offset(A::add_scaled_product(s.ad_value(169), (-(-0.9)), s.ad_value(846), s.ad_value(232), 1.0), (-0.0001))), 1.0, s.ad_value(169), ((-0.9) * (4.0 * 0.0001)))), 0.5, (0.5 * (-0.0001)));s.copy_ad(417, 321);}
        if ((!s.b[1298]) && s.b[1313]) {s.store_mul_exp_mixed_ia(303, 807, A::mul(A::add_scaled_product(s.ad_value(823), 1.0, s.ad_value(824), s.ad_value(229), 1.0), s.ad_value(418)));}
        s.b[1315] = (p.p66 != 0.0);s.store_scalar(1315, if s.b[1315] { 1.0 } else { 0.0 });
        if (((!s.b[1298]) && s.b[1313]) && s.b[1315]) {s.store_mul_exp_mixed_ia(305, 815, A::mul(A::add_scaled_product(s.ad_value(825), 1.0, s.ad_value(824), s.ad_value(229), 1.0), s.ad_value(418)));}
        if ((!s.b[1298]) && s.b[1313]) {s.store_mul_exp_mixed_ia(318, 812, A::mul(A::add_scaled_product(s.ad_value(830), 1.0, s.ad_value(831), s.ad_value(229), 1.0), s.ad_value(418)));}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_40(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.b[1316] = (p.p66 != 0.0);s.store_scalar(1316, if s.b[1316] { 1.0 } else { 0.0 });
        if (((!s.b[1298]) && s.b[1313]) && s.b[1316]) {s.store_mul_exp_mixed_ia(320, 818, A::mul(A::add_scaled_product(s.ad_value(844), 1.0, s.ad_value(831), s.ad_value(229), 1.0), s.ad_value(418)));}
        if ((!s.b[1298]) && s.b[1313]) {s.store_mul_exp_mixed_ia(317, 814, A::mul(A::add_scaled_inputs(s.ad_value(834), 1.0, s.ad_value(229), p.p881), s.ad_value(418)));s.store_mul_scale_offset_mixed_ia(324, 325, A::limited_exp(A::mul(s.ad_value(326), s.ad_value(230))), 1.0, (-1.0));s.store_mul_scale_offset_mixed_ia(327, 328, A::limited_exp(A::mul(s.ad_value(329), s.ad_value(230))), 1.0, (-1.0));s.store_offset(330, 324, 0.5);s.store_offset(331, 327, 0.5);}
        s.b[1317] = (p.p75 != 0.0);s.store_scalar(1317, if s.b[1317] { 1.0 } else { 0.0 });
        if (((!s.b[1298]) && s.b[1313]) && s.b[1317]) {s.store_add_scaled_inputs4_offset_mixed_iaai(323, 811, 1.0, A::add_scaled_product(s.ad_value(811), 1.0, s.ad_value(847), s.ad_value(232), 1.0), 0.5, A::sqrt(A::sub_scaled_inputs(A::square(A::offset(A::add_scaled_product(s.ad_value(811), 1.0, s.ad_value(847), s.ad_value(232), 1.0), (-1e-6))), 1.0, s.ad_value(811), (-(4.0 * 1e-6)))), 0.5, 811, (-1.0), (0.5 * (-1e-6)));}
        if (((!s.b[1298]) && s.b[1313]) && (!s.b[1317])) {
            s.store_mul_mixed_ia(323, 811, {
                            if (!(((1.0 + (s.v[847] * s.v[232])) - 1e-6) < ((-10000.0) * 0.001))) {
                                A::add_scaled_inputs(A::offset(A::mul(s.ad_value(847), s.ad_value(232)), ((1.0) + ((-1e-6)))), 0.5, A::sqrt_square_offset(A::offset(A::mul(s.ad_value(847), s.ad_value(232)), ((1.0) + ((-1e-6)))), ((4.0 * 0.001) * 0.001)), 0.5)
                            } else {
                                {
                                    if (((1.0 + (s.v[847] * s.v[232])) - 1e-6) < ((-10000.0) * 0.001)) {
                                        A::div_scalar_offset_denominator(((-0.001) * 0.001), A::mul(s.ad_value(847), s.ad_value(232)), ((1.0) + ((-1e-6))), 1.0)
                                    } else {
                                        A::constant(0.0)
                                    }
                                }
                            }
                        });
        }
        s.b[1318] = (p.p67 == 1.0);s.store_scalar(1318, if s.b[1318] { 1.0 } else { 0.0 });
        if (((!s.b[1298]) && s.b[1313]) && s.b[1318]) {s.store_mul_exp_mixed_ia(169, 705, A::mul(A::add_scaled_product(s.ad_value(839), 1.0, s.ad_value(840), s.ad_value(229), 1.0), s.ad_value(418)));s.store_add_scaled_inputs4_offset_mixed_iiaa(414, 169, 1.0, 169, (-0.9), A::add_scaled_product(s.ad_value(169), (-(-0.9)), s.ad_value(841), s.ad_value(232), 1.0), 0.5, A::sqrt(A::sub_scaled_inputs(A::square(A::offset(A::add_scaled_product(s.ad_value(169), (-(-0.9)), s.ad_value(841), s.ad_value(232), 1.0), (-0.0001))), 1.0, s.ad_value(169), ((-0.9) * (4.0 * 0.0001)))), 0.5, (0.5 * (-0.0001)));s.store_mul_exp_mixed_ia(304, 808, A::mul(A::add_scaled_product(s.ad_value(826), 1.0, s.ad_value(827), s.ad_value(229), 1.0), s.ad_value(418)));}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_41(
        s: &mut Scratch,
    ) {
        if (((!s.b[1298]) && s.b[1313]) && s.b[1318]) {s.store_mul_exp_mixed_ia(319, 813, A::mul(A::add_scaled_product(s.ad_value(832), 1.0, s.ad_value(833), s.ad_value(229), 1.0), s.ad_value(418)));}
        s.b[1319] = (s.v[854] == s.v[855]);s.store_scalar(1319, if s.b[1319] { 1.0 } else { 0.0 });
        if (((!s.b[1298]) && s.b[1313]) && s.b[1319]) {s.store_offset_mul(170, 854, 232, 1.0);}
        s.b[1320] = (s.v[856] < s.v[228]);s.store_scalar(1320, if s.b[1320] { 1.0 } else { 0.0 });
        if ((((!s.b[1298]) && s.b[1313]) && (!s.b[1319])) && s.b[1320]) {s.store_offset_mul(195, 854, 232, 1.0);s.store_add_scaled_product_mixed_aia(196, A::offset(A::mul(s.ad_value(855), A::sub(s.ad_value(116), s.ad_value(856))), 1.0), 1.0, 854, A::sub(s.ad_value(856), s.ad_value(228)), 1.0);s.store_mul_sub_by_sub(171, 854, 855, 856, 228);}
        s.b[1321] = (s.v[855] < s.v[854]);s.store_scalar(1321, if s.b[1321] { 1.0 } else { 0.0 });
        if (((((!s.b[1298]) && s.b[1313]) && (!s.b[1319])) && s.b[1320]) && s.b[1321]) {s.store_sub_ad(170, A::add_scaled_inputs3(s.ad_value(195), 0.5, s.ad_value(196), 0.5, A::sqrt(A::add_scaled_square_product(A::sub(s.ad_value(195), s.ad_value(196)), 1.0, s.ad_value(857), s.ad_value(857), 0.25)), 0.5), A::add_scaled_inputs(s.ad_value(171), 0.5, A::sqrt(A::add_scaled_square_product(s.ad_value(171), 1.0, s.ad_value(857), s.ad_value(857), 0.25)), 0.5));}
        if (((((!s.b[1298]) && s.b[1313]) && (!s.b[1319])) && s.b[1320]) && (!s.b[1321])) {s.store_sub_ad(170, A::add_scaled_inputs3(s.ad_value(195), 0.5, s.ad_value(196), 0.5, A::sqrt(A::add_scaled_square_product(A::sub(s.ad_value(195), s.ad_value(196)), 1.0, s.ad_value(857), s.ad_value(857), 0.25)), (-0.5)), A::sub_scaled_inputs(s.ad_value(171), 0.5, A::sqrt(A::add_scaled_square_product(s.ad_value(171), 1.0, s.ad_value(857), s.ad_value(857), 0.25)), 0.5));}
        if ((((!s.b[1298]) && s.b[1313]) && (!s.b[1319])) && (!s.b[1320])) {s.store_offset_mul_ad(196, s.ad_value(855), A::sub(s.ad_value(116), s.ad_value(228)), 1.0);s.store_add_scaled_product_mixed_aia(195, A::offset(A::mul(s.ad_value(854), A::sub(s.ad_value(116), s.ad_value(856))), 1.0), 1.0, 855, A::sub(s.ad_value(856), s.ad_value(228)), 1.0);s.store_mul_sub_by_sub(171, 855, 854, 856, 228);}
        s.b[1322] = (s.v[855] < s.v[854]);s.store_scalar(1322, if s.b[1322] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_42(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((((!s.b[1298]) && s.b[1313]) && (!s.b[1319])) && (!s.b[1320])) && s.b[1322]) {s.store_sub_ad(170, A::add_scaled_inputs3(s.ad_value(196), 0.5, s.ad_value(195), 0.5, A::sqrt(A::add_scaled_square_product(A::sub(s.ad_value(196), s.ad_value(195)), 1.0, s.ad_value(857), s.ad_value(857), 0.25)), 0.5), A::add_scaled_inputs(s.ad_value(171), 0.5, A::sqrt(A::add_scaled_square_product(s.ad_value(171), 1.0, s.ad_value(857), s.ad_value(857), 0.25)), 0.5));}
        if (((((!s.b[1298]) && s.b[1313]) && (!s.b[1319])) && (!s.b[1320])) && (!s.b[1322])) {s.store_sub_ad(170, A::add_scaled_inputs3(s.ad_value(196), 0.5, s.ad_value(195), 0.5, A::sqrt(A::add_scaled_square_product(A::sub(s.ad_value(196), s.ad_value(195)), 1.0, s.ad_value(857), s.ad_value(857), 0.25)), (-0.5)), A::sub_scaled_inputs(s.ad_value(171), 0.5, A::sqrt(A::add_scaled_square_product(s.ad_value(171), 1.0, s.ad_value(857), s.ad_value(857), 0.25)), 0.5));}
        if ((!s.b[1298]) && s.b[1313]) {
            if (!((s.v[170] - 1e-6) < ((-10000.0) * 0.001))) {
                s.store_scaled_add_sqrt_square_offset_ad(194, A::offset(s.ad_value(170), (-1e-6)), ((4.0 * 0.001) * 0.001), 0.5);
            } else {
                if ((s.v[170] - 1e-6) < ((-10000.0) * 0.001)) {
                    s.store_div_from_scalar_offset_input(194, ((-0.001) * 0.001), 170, (-1e-6));
                } else {
                    s.store_scalar(194, 0.0);
                }
            }
        }
        s.b[1323] = (p.p75 != 0.0);s.store_scalar(1323, if s.b[1323] { 1.0 } else { 0.0 });
        if (((!s.b[1298]) && s.b[1313]) && s.b[1323]) {s.store_add_scaled_inputs3_mixed_iai(332, 679, 1.0, A::add_scaled_inputs3_offset(A::add_scaled_products(s.ad_value(849), s.ad_value(232), -1.0, s.ad_value(232), s.ad_value(232), p.p561), 0.5, s.ad_value(679), 0.5, A::sqrt(A::sub_scaled_inputs(A::square(A::offset(A::sub_scaled_inputs(A::add_scaled_products(s.ad_value(849), s.ad_value(232), -1.0, s.ad_value(232), s.ad_value(232), p.p561), 1.0, s.ad_value(679), -1.0), (-1e-6))), 1.0, s.ad_value(679), (-(4.0 * 1e-6)))), 0.5, ((-1e-6) * 0.5)), 1.0, 679, (-1.0));}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_43(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((!s.b[1298]) && s.b[1313]) && (!s.b[1323])) {
            s.store_mul_mixed_ia(332, 679, {
                            if (!((((1.0 + ((-s.v[849]) * s.v[232])) + ((p.p561 * s.v[232]) * s.v[232])) - 1e-6) < ((-10000.0) * 0.001))) {
                                A::add_scaled_inputs(A::offset(A::add_scaled_product(A::offset(A::mul_scaled_lhs(s.ad_value(849), -1.0, s.ad_value(232)), 1.0), 1.0, s.ad_value(232), s.ad_value(232), p.p561), (-1e-6)), 0.5, A::sqrt_square_offset(A::offset(A::add_scaled_product(A::offset(A::mul_scaled_lhs(s.ad_value(849), -1.0, s.ad_value(232)), 1.0), 1.0, s.ad_value(232), s.ad_value(232), p.p561), (-1e-6)), ((4.0 * 0.001) * 0.001)), 0.5)
                            } else {
                                {
                                    if ((((1.0 + ((-s.v[849]) * s.v[232])) + ((p.p561 * s.v[232]) * s.v[232])) - 1e-6) < ((-10000.0) * 0.001)) {
                                        A::div_scalar_offset_denominator(((-0.001) * 0.001), A::add_scaled_product(A::offset(A::mul_scaled_lhs(s.ad_value(849), -1.0, s.ad_value(232)), 1.0), 1.0, s.ad_value(232), s.ad_value(232), p.p561), (-1e-6), 1.0)
                                    } else {
                                        A::constant(0.0)
                                    }
                                }
                            }
                        });
        }
        s.b[1324] = (p.p66 != 0.0);s.store_scalar(1324, if s.b[1324] { 1.0 } else { 0.0 });s.b[1325] = (p.p75 != 0.0);s.store_scalar(1325, if s.b[1325] { 1.0 } else { 0.0 });
        if ((((!s.b[1298]) && s.b[1313]) && s.b[1324]) && s.b[1325]) {s.store_add_scaled_inputs3_mixed_iai(333, 680, 1.0, A::add_scaled_inputs3_offset(A::add_scaled_products(s.ad_value(851), s.ad_value(232), -1.0, s.ad_value(232), s.ad_value(232), p.p561), 0.5, s.ad_value(680), 0.5, A::sqrt(A::sub_scaled_inputs(A::square(A::offset(A::sub_scaled_inputs(A::add_scaled_products(s.ad_value(851), s.ad_value(232), -1.0, s.ad_value(232), s.ad_value(232), p.p561), 1.0, s.ad_value(680), -1.0), (-1e-6))), 1.0, s.ad_value(680), (-(4.0 * 1e-6)))), 0.5, ((-1e-6) * 0.5)), 1.0, 680, (-1.0));}
        if ((((!s.b[1298]) && s.b[1313]) && s.b[1324]) && (!s.b[1325])) {
            s.store_mul_mixed_ia(333, 680, {
                            if (!((((1.0 + ((-s.v[851]) * s.v[232])) + ((p.p561 * s.v[232]) * s.v[232])) - 1e-6) < ((-10000.0) * 0.001))) {
                                A::add_scaled_inputs(A::offset(A::add_scaled_product(A::offset(A::mul_scaled_lhs(s.ad_value(851), -1.0, s.ad_value(232)), 1.0), 1.0, s.ad_value(232), s.ad_value(232), p.p561), (-1e-6)), 0.5, A::sqrt_square_offset(A::offset(A::add_scaled_product(A::offset(A::mul_scaled_lhs(s.ad_value(851), -1.0, s.ad_value(232)), 1.0), 1.0, s.ad_value(232), s.ad_value(232), p.p561), (-1e-6)), ((4.0 * 0.001) * 0.001)), 0.5)
                            } else {
                                {
                                    if ((((1.0 + ((-s.v[851]) * s.v[232])) + ((p.p561 * s.v[232]) * s.v[232])) - 1e-6) < ((-10000.0) * 0.001)) {
                                        A::div_scalar_offset_denominator(((-0.001) * 0.001), A::add_scaled_product(A::offset(A::mul_scaled_lhs(s.ad_value(851), -1.0, s.ad_value(232)), 1.0), 1.0, s.ad_value(232), s.ad_value(232), p.p561), (-1e-6), 1.0)
                                    } else {
                                        A::constant(0.0)
                                    }
                                }
                            }
                        });
        }
        s.b[1326] = (s.v[333] < 1000.0);s.store_scalar(1326, if s.b[1326] { 1.0 } else { 0.0 });
        if ((((!s.b[1298]) && s.b[1313]) && s.b[1324]) && s.b[1326]) {s.store_scalar(333, 1000.0);}
        s.b[1327] = (p.p75 != 0.0);s.store_scalar(1327, if s.b[1327] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_44(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((!s.b[1298]) && s.b[1313]) && s.b[1327]) {s.store_add_scaled_inputs3_mixed_iai(334, 698, 1.0, A::add_scaled_inputs3_offset(A::add_scaled_products(s.ad_value(849), s.ad_value(232), -1.0, s.ad_value(232), s.ad_value(232), p.p561), 0.5, s.ad_value(698), 0.5, A::sqrt(A::sub_scaled_inputs(A::square(A::offset(A::sub_scaled_inputs(A::add_scaled_products(s.ad_value(849), s.ad_value(232), -1.0, s.ad_value(232), s.ad_value(232), p.p561), 1.0, s.ad_value(698), -1.0), (-1e-6))), 1.0, s.ad_value(698), (-(4.0 * 1e-6)))), 0.5, ((-1e-6) * 0.5)), 1.0, 698, (-1.0));}
        if (((!s.b[1298]) && s.b[1313]) && (!s.b[1327])) {
            s.store_mul_mixed_ia(334, 698, {
                            if (!((((1.0 + ((-s.v[849]) * s.v[232])) + ((p.p561 * s.v[232]) * s.v[232])) - 1e-6) < ((-10000.0) * 0.001))) {
                                A::add_scaled_inputs(A::offset(A::add_scaled_product(A::offset(A::mul_scaled_lhs(s.ad_value(849), -1.0, s.ad_value(232)), 1.0), 1.0, s.ad_value(232), s.ad_value(232), p.p561), (-1e-6)), 0.5, A::sqrt_square_offset(A::offset(A::add_scaled_product(A::offset(A::mul_scaled_lhs(s.ad_value(849), -1.0, s.ad_value(232)), 1.0), 1.0, s.ad_value(232), s.ad_value(232), p.p561), (-1e-6)), ((4.0 * 0.001) * 0.001)), 0.5)
                            } else {
                                {
                                    if ((((1.0 + ((-s.v[849]) * s.v[232])) + ((p.p561 * s.v[232]) * s.v[232])) - 1e-6) < ((-10000.0) * 0.001)) {
                                        A::div_scalar_offset_denominator(((-0.001) * 0.001), A::add_scaled_product(A::offset(A::mul_scaled_lhs(s.ad_value(849), -1.0, s.ad_value(232)), 1.0), 1.0, s.ad_value(232), s.ad_value(232), p.p561), (-1e-6), 1.0)
                                    } else {
                                        A::constant(0.0)
                                    }
                                }
                            }
                        });
        }
        s.b[1328] = (p.p66 != 0.0);s.store_scalar(1328, if s.b[1328] { 1.0 } else { 0.0 });s.b[1329] = (p.p75 != 0.0);s.store_scalar(1329, if s.b[1329] { 1.0 } else { 0.0 });
        if ((((!s.b[1298]) && s.b[1313]) && s.b[1328]) && s.b[1329]) {s.store_add_scaled_inputs3_mixed_iai(335, 699, 1.0, A::add_scaled_inputs3_offset(A::add_scaled_products(s.ad_value(849), s.ad_value(232), -1.0, s.ad_value(232), s.ad_value(232), p.p561), 0.5, s.ad_value(699), 0.5, A::sqrt(A::sub_scaled_inputs(A::square(A::offset(A::sub_scaled_inputs(A::add_scaled_products(s.ad_value(849), s.ad_value(232), -1.0, s.ad_value(232), s.ad_value(232), p.p561), 1.0, s.ad_value(699), -1.0), (-1e-6))), 1.0, s.ad_value(699), (-(4.0 * 1e-6)))), 0.5, ((-1e-6) * 0.5)), 1.0, 699, (-1.0));}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_45(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((((!s.b[1298]) && s.b[1313]) && s.b[1328]) && (!s.b[1329])) {
            s.store_mul_mixed_ia(335, 699, {
                            if (!((((1.0 + ((-s.v[849]) * s.v[232])) + ((p.p561 * s.v[232]) * s.v[232])) - 1e-6) < ((-10000.0) * 0.001))) {
                                A::add_scaled_inputs(A::offset(A::add_scaled_product(A::offset(A::mul_scaled_lhs(s.ad_value(849), -1.0, s.ad_value(232)), 1.0), 1.0, s.ad_value(232), s.ad_value(232), p.p561), (-1e-6)), 0.5, A::sqrt_square_offset(A::offset(A::add_scaled_product(A::offset(A::mul_scaled_lhs(s.ad_value(849), -1.0, s.ad_value(232)), 1.0), 1.0, s.ad_value(232), s.ad_value(232), p.p561), (-1e-6)), ((4.0 * 0.001) * 0.001)), 0.5)
                            } else {
                                {
                                    if ((((1.0 + ((-s.v[849]) * s.v[232])) + ((p.p561 * s.v[232]) * s.v[232])) - 1e-6) < ((-10000.0) * 0.001)) {
                                        A::div_scalar_offset_denominator(((-0.001) * 0.001), A::add_scaled_product(A::offset(A::mul_scaled_lhs(s.ad_value(849), -1.0, s.ad_value(232)), 1.0), 1.0, s.ad_value(232), s.ad_value(232), p.p561), (-1e-6), 1.0)
                                    } else {
                                        A::constant(0.0)
                                    }
                                }
                            }
                        });
        }
        s.b[1330] = (s.v[335] < 1000.0);s.store_scalar(1330, if s.b[1330] { 1.0 } else { 0.0 });
        if ((((!s.b[1298]) && s.b[1313]) && s.b[1328]) && s.b[1330]) {s.store_scalar(335, 1000.0);}
        s.b[1331] = (p.p75 != 0.0);s.store_scalar(1331, if s.b[1331] { 1.0 } else { 0.0 });
        if (((!s.b[1298]) && s.b[1313]) && s.b[1331]) {s.store_add_scaled_inputs3_mixed_iai(336, 702, 1.0, A::add_scaled_inputs3_offset(A::add_scaled_products(s.ad_value(850), s.ad_value(232), -1.0, s.ad_value(232), s.ad_value(232), p.p574), 0.5, s.ad_value(702), 0.5, A::sqrt(A::sub_scaled_inputs(A::square(A::offset(A::sub_scaled_inputs(A::add_scaled_products(s.ad_value(850), s.ad_value(232), -1.0, s.ad_value(232), s.ad_value(232), p.p574), 1.0, s.ad_value(702), -1.0), (-1e-6))), 1.0, s.ad_value(702), (-(4.0 * 1e-6)))), 0.5, ((-1e-6) * 0.5)), 1.0, 702, (-1.0));}
        if (((!s.b[1298]) && s.b[1313]) && (!s.b[1331])) {
            s.store_mul_mixed_ia(336, 702, {
                            if (!((((1.0 + ((-s.v[850]) * s.v[232])) + ((p.p574 * s.v[232]) * s.v[232])) - 1e-6) < ((-10000.0) * 0.001))) {
                                A::add_scaled_inputs(A::offset(A::add_scaled_product(A::offset(A::mul_scaled_lhs(s.ad_value(850), -1.0, s.ad_value(232)), 1.0), 1.0, s.ad_value(232), s.ad_value(232), p.p574), (-1e-6)), 0.5, A::sqrt_square_offset(A::offset(A::add_scaled_product(A::offset(A::mul_scaled_lhs(s.ad_value(850), -1.0, s.ad_value(232)), 1.0), 1.0, s.ad_value(232), s.ad_value(232), p.p574), (-1e-6)), ((4.0 * 0.001) * 0.001)), 0.5)
                            } else {
                                {
                                    if ((((1.0 + ((-s.v[850]) * s.v[232])) + ((p.p574 * s.v[232]) * s.v[232])) - 1e-6) < ((-10000.0) * 0.001)) {
                                        A::div_scalar_offset_denominator(((-0.001) * 0.001), A::add_scaled_product(A::offset(A::mul_scaled_lhs(s.ad_value(850), -1.0, s.ad_value(232)), 1.0), 1.0, s.ad_value(232), s.ad_value(232), p.p574), (-1e-6), 1.0)
                                    } else {
                                        A::constant(0.0)
                                    }
                                }
                            }
                        });
        }
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_46(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((!s.b[1298]) && s.b[1313]) {
            s.store_offset_ad(337, {
                if (!(((s.v[790] * ((1.0 + (p.p450 * s.v[232])) + ((p.p451 * s.v[232]) * s.v[232]))) - 2.0) < ((-10000.0) * 0.001))) {
                    A::add_scaled_inputs(A::offset(A::mul(s.ad_value(790), A::add_scaled_product(A::scale_offset(s.ad_value(232), p.p450, 1.0), 1.0, s.ad_value(232), s.ad_value(232), p.p451)), (-2.0)), 0.5, A::sqrt_square_offset(A::offset(A::mul(s.ad_value(790), A::add_scaled_product(A::scale_offset(s.ad_value(232), p.p450, 1.0), 1.0, s.ad_value(232), s.ad_value(232), p.p451)), (-2.0)), ((4.0 * 0.001) * 0.001)), 0.5)
                } else {
                    {
                        if (((s.v[790] * ((1.0 + (p.p450 * s.v[232])) + ((p.p451 * s.v[232]) * s.v[232]))) - 2.0) < ((-10000.0) * 0.001)) {
                            A::div_scalar_offset_denominator(((-0.001) * 0.001), A::mul(s.ad_value(790), A::add_scaled_product(A::scale_offset(s.ad_value(232), p.p450, 1.0), 1.0, s.ad_value(232), s.ad_value(232), p.p451)), (-2.0), 1.0)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 2.0);
        }
        s.b[1332] = (p.p66 != 0.0);s.store_scalar(1332, if s.b[1332] { 1.0 } else { 0.0 });
        if (((!s.b[1298]) && s.b[1313]) && s.b[1332]) {
            s.store_offset_ad(338, {
                if (!(((s.v[791] * ((1.0 + (p.p452 * s.v[232])) + ((p.p451 * s.v[232]) * s.v[232]))) - 2.0) < ((-10000.0) * 0.001))) {
                    A::add_scaled_inputs(A::offset(A::mul(s.ad_value(791), A::add_scaled_product(A::scale_offset(s.ad_value(232), p.p452, 1.0), 1.0, s.ad_value(232), s.ad_value(232), p.p451)), (-2.0)), 0.5, A::sqrt_square_offset(A::offset(A::mul(s.ad_value(791), A::add_scaled_product(A::scale_offset(s.ad_value(232), p.p452, 1.0), 1.0, s.ad_value(232), s.ad_value(232), p.p451)), (-2.0)), ((4.0 * 0.001) * 0.001)), 0.5)
                } else {
                    {
                        if (((s.v[791] * ((1.0 + (p.p452 * s.v[232])) + ((p.p451 * s.v[232]) * s.v[232]))) - 2.0) < ((-10000.0) * 0.001)) {
                            A::div_scalar_offset_denominator(((-0.001) * 0.001), A::mul(s.ad_value(791), A::add_scaled_product(A::scale_offset(s.ad_value(232), p.p452, 1.0), 1.0, s.ad_value(232), s.ad_value(232), p.p451)), (-2.0), 1.0)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 2.0);
        }
        s.b[1333] = (p.p75 != 0.0);s.store_scalar(1333, if s.b[1333] { 1.0 } else { 0.0 });
        if (((!s.b[1298]) && s.b[1313]) && s.b[1333]) {s.store_add_scaled_inputs3_mixed_iai(660, 657, 1.0, A::add_scaled_inputs3_offset(A::add_scaled_product(s.ad_value(232), p.p498, s.ad_value(232), s.ad_value(232), p.p499), 0.5, s.ad_value(657), 0.5, A::sqrt(A::sub_scaled_inputs(A::square(A::offset(A::sub_scaled_inputs(A::add_scaled_product(s.ad_value(232), p.p498, s.ad_value(232), s.ad_value(232), p.p499), 1.0, s.ad_value(657), -1.0), (-1e-6))), 1.0, s.ad_value(657), (-(4.0 * 1e-6)))), 0.5, ((-1e-6) * 0.5)), 1.0, 657, (-1.0));}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_47(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((!s.b[1298]) && s.b[1313]) && (!s.b[1333])) {
            s.store_mul_mixed_ia(660, 657, {
                            if (!((((1.0 + (p.p498 * s.v[232])) + ((p.p499 * s.v[232]) * s.v[232])) - 1e-6) < ((-10000.0) * 0.001))) {
                                A::add_scaled_inputs(A::offset(A::add_scaled_product(A::scale_offset(s.ad_value(232), p.p498, 1.0), 1.0, s.ad_value(232), s.ad_value(232), p.p499), (-1e-6)), 0.5, A::sqrt_square_offset(A::offset(A::add_scaled_product(A::scale_offset(s.ad_value(232), p.p498, 1.0), 1.0, s.ad_value(232), s.ad_value(232), p.p499), (-1e-6)), ((4.0 * 0.001) * 0.001)), 0.5)
                            } else {
                                {
                                    if ((((1.0 + (p.p498 * s.v[232])) + ((p.p499 * s.v[232]) * s.v[232])) - 1e-6) < ((-10000.0) * 0.001)) {
                                        A::div_scalar_offset_denominator(((-0.001) * 0.001), A::add_scaled_product(A::scale_offset(s.ad_value(232), p.p498, 1.0), 1.0, s.ad_value(232), s.ad_value(232), p.p499), (-1e-6), 1.0)
                                    } else {
                                        A::constant(0.0)
                                    }
                                }
                            }
                        });
        }
        s.b[1334] = (p.p75 != 0.0);s.store_scalar(1334, if s.b[1334] { 1.0 } else { 0.0 });
        if (((!s.b[1298]) && s.b[1313]) && s.b[1334]) {s.store_add_scaled_inputs3_mixed_iai(797, 792, 1.0, A::add_scaled_inputs3_offset(s.ad_value(232), (p.p1026 * 0.5), s.ad_value(792), 0.5, A::sqrt(A::sub_scaled_inputs(A::square(A::offset(A::sub_scaled_inputs(s.ad_value(232), p.p1026, s.ad_value(792), -1.0), (-1e-6))), 1.0, s.ad_value(792), (-(4.0 * 1e-6)))), 0.5, ((-1e-6) * 0.5)), 1.0, 792, (-1.0));}
        if (((!s.b[1298]) && s.b[1313]) && (!s.b[1334])) {
            s.store_mul_mixed_ia(797, 792, {
                            if (!(((1.0 + (p.p1026 * s.v[232])) - 1e-6) < ((-10000.0) * 0.001))) {
                                A::add_scaled_inputs(A::scale_offset(s.ad_value(232), p.p1026, ((1.0) + ((-1e-6)))), 0.5, A::sqrt_square_offset(A::scale_offset(s.ad_value(232), p.p1026, ((1.0) + ((-1e-6)))), ((4.0 * 0.001) * 0.001)), 0.5)
                            } else {
                                {
                                    if (((1.0 + (p.p1026 * s.v[232])) - 1e-6) < ((-10000.0) * 0.001)) {
                                        A::div_from_scalar(((-0.001) * 0.001), A::scale_offset(s.ad_value(232), p.p1026, ((1.0) + ((-1e-6)))))
                                    } else {
                                        A::constant(0.0)
                                    }
                                }
                            }
                        });
        }
        if ((!s.b[1298]) && s.b[1313]) {s.store_sub_ad(231, A::add_scaled_product(A::div_scalar_offset_denominator(p.p1747, A::limited_exp_scaled_input(A::offset(s.ad_value(116), (-p.p1749)), p.p1748), 1.0, 1.0), 1.0, A::add(s.ad_value(858), A::div_from_scalar(p.p1720, s.ad_value(153))), s.ad_value(230), 1.0), A::div_scalar_offset_denominator(p.p1747, A::limited_exp_scaled_input(A::offset(s.ad_value(228), (-p.p1749)), p.p1748), 1.0, 1.0));}
        if ((!s.b[1298]) && (!s.b[1313])) {s.store_mul_exp_mixed_ia(169, 704, A::mul(A::add_scaled_product(s.ad_value(836), 1.0, s.ad_value(837), s.ad_value(234), 1.0), s.ad_value(418)));s.store_add_scaled_inputs4_offset_mixed_iiaa(413, 169, 1.0, 169, (-0.9), A::add_scaled_product(s.ad_value(169), (-(-0.9)), s.ad_value(838), s.ad_value(232), 1.0), 0.5, A::sqrt(A::sub_scaled_inputs(A::square(A::offset(A::add_scaled_product(s.ad_value(169), (-(-0.9)), s.ad_value(838), s.ad_value(232), 1.0), (-0.0001))), 1.0, s.ad_value(169), ((-0.9) * (4.0 * 0.0001)))), 0.5, (0.5 * (-0.0001)));}
        s.b[1335] = (p.p66 == 1.0);s.store_scalar(1335, if s.b[1335] { 1.0 } else { 0.0 });
        if (((!s.b[1298]) && (!s.b[1313])) && s.b[1335]) {s.store_mul_exp_mixed_ia(169, 706, A::mul(A::add_scaled_product(s.ad_value(845), 1.0, s.ad_value(837), s.ad_value(234), 1.0), s.ad_value(418)));}
    }
}
