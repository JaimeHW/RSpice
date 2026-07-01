#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, ReactiveScratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_reactive_block_9(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        let ctx_temp = ctx.temperature();
        if s.b[1288] {
            let assign13970_ad_e20176: A = A::pow_from_scalar((p.p1852 * 1000000000.0), A::scale(s.ad_value(994), p.p1892));
            let assign13970_ad_e20180: A = A::powf(A::scale_offset(assign13970_ad_e20176, 5.0, 1.0), 0.5);
            s.store_scaled_add_sqrt_square_offset_ad(1002, A::div(s.ad_value(993), assign13970_ad_e20180), ((0.25 * 0.1) * 0.1), 0.5);
        }

        if s.b[1288] {
            s.store_add_scaled_inputs3_indices(1003, 892, 1.0, 1001, p.p1889, 1002, (-p.p1889));
        }

        if s.b[1288] {
            let assign13990_ad_e20287: A = A::scale_offset(s.ad_value(960), ((0.5) * (2.0)), ((((1.0) + ((-1.0)))) * (2.0)));
            s.store_mul_product3_mixed_iiaa(1010, 979, 960, A::div(A::pow_from_scalar(3.14, A::scale(s.ad_value(960), 0.5)), A::offset(A::sub(A::add_scaled_inputs(A::sub_scaled_inputs(A::add_scaled_inputs(A::add_scaled_inputs4(A::exp_scaled_input(A::scale_offset(s.ad_value(960), 0.5, ((1.0) + ((-1.0)))), (-4.6)), 0.0385, A::powi(A::scale_offset(s.ad_value(960), ((0.5) * (2.0)), ((2.0) + ((-3.0)))), 8), 7.5893e-7, A::powi(assign13990_ad_e20287, 6), 6.9583e-5, A::powi(assign13990_ad_e20287, 5), (-0.0006583)), 1.0, A::powi(assign13990_ad_e20287, 4), 0.0065), 1.0, A::powi(assign13990_ad_e20287, 3), 0.026), 1.0, A::square(assign13990_ad_e20287), 0.1371), A::scale_offset(s.ad_value(960), ((0.5) * ((0.194 * 2.0))), ((((1.0) + ((-1.0)))) * ((0.194 * 2.0))))), 0.959)), A::pow(A::scale(s.ad_value(997), 1000000.0), s.ad_value(960)), (1.0 / (2.0) * 1.60219e-19));
        }

        if s.b[1288] {
            let assign14000_ad_e20421: A = A::scale_offset(s.ad_value(961), ((0.5) * (2.0)), ((((1.0) + ((-1.0)))) * (2.0)));
            s.store_mul_product3_mixed_iiaa(1011, 980, 961, A::div(A::pow_from_scalar(3.14, A::scale(s.ad_value(961), 0.5)), A::offset(A::sub(A::add_scaled_inputs(A::sub_scaled_inputs(A::add_scaled_inputs(A::add_scaled_inputs4(A::exp_scaled_input(A::scale_offset(s.ad_value(961), 0.5, ((1.0) + ((-1.0)))), (-4.6)), 0.0385, A::powi(A::scale_offset(s.ad_value(961), ((0.5) * (2.0)), ((2.0) + ((-3.0)))), 8), 7.5893e-7, A::powi(assign14000_ad_e20421, 6), 6.9583e-5, A::powi(assign14000_ad_e20421, 5), (-0.0006583)), 1.0, A::powi(assign14000_ad_e20421, 4), 0.0065), 1.0, A::powi(assign14000_ad_e20421, 3), 0.026), 1.0, A::square(assign14000_ad_e20421), 0.1371), A::scale_offset(s.ad_value(961), ((0.5) * ((0.194 * 2.0))), ((((1.0) + ((-1.0)))) * ((0.194 * 2.0))))), 0.959)), A::pow(A::scale(s.ad_value(1000), 1000000.0), s.ad_value(961)), (1.0 / (2.0) * 1.60219e-19));
        }

        if s.b[1288] {
            let assign14010_ad_e20555: A = A::scale_offset(s.ad_value(962), ((0.5) * (2.0)), ((((1.0) + ((-1.0)))) * (2.0)));
            s.store_mul_product3_mixed_iiaa(1012, 981, 962, A::div(A::pow_from_scalar(3.14, A::scale(s.ad_value(962), 0.5)), A::offset(A::sub(A::add_scaled_inputs(A::sub_scaled_inputs(A::add_scaled_inputs(A::add_scaled_inputs4(A::exp_scaled_input(A::scale_offset(s.ad_value(962), 0.5, ((1.0) + ((-1.0)))), (-4.6)), 0.0385, A::powi(A::scale_offset(s.ad_value(962), ((0.5) * (2.0)), ((2.0) + ((-3.0)))), 8), 7.5893e-7, A::powi(assign14010_ad_e20555, 6), 6.9583e-5, A::powi(assign14010_ad_e20555, 5), (-0.0006583)), 1.0, A::powi(assign14010_ad_e20555, 4), 0.0065), 1.0, A::powi(assign14010_ad_e20555, 3), 0.026), 1.0, A::square(assign14010_ad_e20555), 0.1371), A::scale_offset(s.ad_value(962), ((0.5) * ((0.194 * 2.0))), ((((1.0) + ((-1.0)))) * ((0.194 * 2.0))))), 0.959)), A::pow(A::scale(s.ad_value(1003), 1000000.0), s.ad_value(962)), (1.0 / (2.0) * 1.60219e-19));
        }

        s.b[1289] = (p.p58 == 1.0);
        s.store_scalar(1289, if s.b[1289] { 1.0 } else { 0.0 });

        if s.b[1289] {
            s.store_offset_scaled(707, 707, 1.0 / (({ let limited_exp_arg = (((p.p890 * 1000000000.0) - (p.p40 * 1000000000.0)) / p.p891); if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } } + 1.0)), (((((-p.p889)) * (1.0 / (({ let limited_exp_arg = (((p.p890 * 1000000000.0) - (p.p40 * 1000000000.0)) / p.p891); if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } } + 1.0))))) + (p.p889)));
        }

        if s.b[1289] {
            s.store_offset(1024, 807, (((-p.p892)) + ((-((p.p893 * 1000000000.0) * p.p894)))));
        }

        if s.b[1289] {
            s.store_scaled_offset(1025, 1024, ((p.p40 * 1000000000.0) * p.p894), 1.0 / ((1.0 + { let limited_exp_arg = (((p.p895 * 1000000000.0) - (p.p40 * 1000000000.0)) / p.p896); if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } })));
        }

        if s.b[1289] {
            s.store_add_scaled_inputs3_offset_mixed_iia(807, 1025, 0.5, 807, 0.5, A::sqrt_square_offset(A::sub(A::offset(s.ad_value(1025), p.p892), A::offset(s.ad_value(807), 0.2)), ((0.25 * 0.6) * 0.6)), (-0.5), ((p.p892 + 0.2) * 0.5));
        }

        if s.b[1289] {
            s.store_add_scaled_inputs3_offset_indices(1026, 811, (-(370.0 * 1.0 / ((((p.p40 * 1000000000.0)) as f64).powf(p.p898)))), 811, (-1.0 / ((1.0 + { let limited_exp_arg = (((p.p40 * 1000000000.0) - (p.p899 * 1000000000.0)) / p.p900); if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } }))), 811, 1.0, (((p.p897) * ((370.0 * 1.0 / ((((p.p40 * 1000000000.0)) as f64).powf(p.p898))))) + ((p.p897) * (1.0 / ((1.0 + { let limited_exp_arg = (((p.p40 * 1000000000.0) - (p.p899 * 1000000000.0)) / p.p900); if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } }))))));
        }

        if s.b[1289] {
            s.store_scaled_sub_offset_sqrt_square_offset(811, 1026, p.p897, (-p.p897), ((0.25 * 0.2) * 0.2), 0.5);
            s.store_scalar(1027, (p.p43 / (p.p43 + p.p40)));
            s.store_scalar(1028, ((((p.p905 * p.p40) * p.p40) * 1e18) - (p.p906 * 0.001)));
            s.store_scaled_add_ad_rhs(1029, 1028, A::powf(A::offset(A::square(s.ad_value(1028)), ((((((4.0 * p.p906) * 0.001) * (p.p905 + 0.24)) * p.p40) * p.p40) * 1e18)), 0.5), 1.0 / (((((2.0 * (p.p905 + 0.24)) * p.p40) * p.p40) * 1e18)));
            s.store_scaled_sub_offset_sqrt_square_offset_ad(1030, A::div_scalar_offset_denominator(0.0001, s.ad_value(1029), (((-0.8208)) + ((-(p.p907 * 1e-5)))), 1.0), 1.0, (-1.0), ((0.25 * 0.06) * 0.06), 0.5);
            s.store_mul_ad_product_lhs_mixed_ia(704, 704, A::add(s.ad_value(1027), A::scale_offset(s.ad_value(1027), (-p.p904), p.p904)), 1030);
            s.store_add_ad_lhs(812, A::scale_offset(s.ad_value(812), (-(((0.5 * (((p.p902 * 1000000000.0) - (p.p40 * 1000000000.0)) + ((((((p.p902 * 1000000000.0) - (p.p40 * 1000000000.0)) * ((p.p902 * 1000000000.0) - (p.p40 * 1000000000.0))) + 0.25)) as f64).sqrt()))) as f64).powf(p.p903)), ((p.p901) * ((((0.5 * (((p.p902 * 1000000000.0) - (p.p40 * 1000000000.0)) + ((((((p.p902 * 1000000000.0) - (p.p40 * 1000000000.0)) * ((p.p902 * 1000000000.0) - (p.p40 * 1000000000.0))) + 0.25)) as f64).sqrt()))) as f64).powf(p.p903)))), 812);
        }

        s.b[1290] = ((p.p74 != 0.0) && (p.p1791 > 0.0));
        s.store_scalar(1290, if s.b[1290] { 1.0 } else { 0.0 });

        if s.b[1290] {
            s.store_offset_voltage(116, ctx, nodes, Some(4), None, ((ctx_temp) + (p.p22)));
        }

        if (!s.b[1290]) {
            s.store_scalar(116, (ctx_temp + p.p22));
        }

        s.store_div(229, 116, 228);

        s.store_offset(230, 229, (-1.0));

        s.store_sub(232, 116, 228);

        s.store_scale(179, 116, 8.617087e-5);

        s.store_scale(180, 228, 8.617087e-5);

        s.store_scalar(121, p.p1786);

        s.b[1291] = (p.p80 != 0.0);
        s.store_scalar(1291, if s.b[1291] { 1.0 } else { 0.0 });

        if s.b[1291] {
            s.store_scaled_add_offset_sqrt_square_offset(119, 116, s.v[121], (-s.v[121]), ((0.25 * p.p1788) * p.p1788), 0.5);
            s.store_scaled_add_sqrt_square_offset_ad(120, A::scaled_offset(s.ad_value(116), (-p.p1787), (-p.p1790)), ((0.25 * p.p1789) * p.p1789), 0.5);
        }

        s.b[1292] = (p.p80 == 1.0);
        s.store_scalar(1292, if s.b[1292] { 1.0 } else { 0.0 });

        if (s.b[1291] && s.b[1292]) {
            s.store_scaled_add_offset_sqrt_square_offset(169, 228, s.v[121], (-s.v[121]), ((0.25 * p.p1788) * p.p1788), 0.5);
            s.store_scaled_add_sqrt_square_offset_ad(170, A::scaled_offset(s.ad_value(228), (-p.p1787), (-p.p1790)), ((0.25 * p.p1789) * p.p1789), 0.5);
        }

        s.b[1293] = (s.v[228] > s.v[121]);
        s.store_scalar(1293, if s.b[1293] { 1.0 } else { 0.0 });

        if ((s.b[1291] && s.b[1292]) && s.b[1293]) {
            s.store_add_ad_lhs(171, A::add_scaled_inputs4(s.ad_value(119), 1.0, s.ad_value(120), 1.0, s.ad_value(169), -1.0, s.ad_value(170), -1.0), 228);
        }

        if ((s.b[1291] && s.b[1292]) && (!s.b[1293])) {
            s.store_add_scaled_inputs4_offset_indices(171, 119, 1.0, 120, 1.0, 169, -1.0, 170, -1.0, s.v[121]);
        }

        if (s.b[1291] && s.b[1292]) {
            s.store_add_scaled_inputs3_sqrt_third_sub_square_offset(118, 116, 0.5, 171, 0.5, 116, 171, ((0.25 * 0.2) * 0.2), 0.5);
        }

        s.b[1294] = (s.v[121] > 210.0);
        s.store_scalar(1294, if s.b[1294] { 1.0 } else { 0.0 });

        if ((s.b[1291] && (!s.b[1292])) && s.b[1294]) {
            s.store_scalar(121, 210.0);
        }

        if (s.b[1291] && (!s.b[1292])) {
            s.store_offset_scaled_ad(312, A::tanh_scaled_input(A::offset(s.ad_value(116), (-210.0)), 0.5), 0.5, 0.5);
            s.store_sub_from_scalar(313, 1.0, 312);
        }

        s.b[1295] = (s.v[228] > 210.0);
        s.store_scalar(1295, if s.b[1295] { 1.0 } else { 0.0 });

        if ((s.b[1291] && (!s.b[1292])) && s.b[1295]) {
            s.store_scaled_add_ad(169, A::offset(s.ad_value(121), 210.0), A::sqrt_square_offset(A::sub_from_scalar(210.0, s.ad_value(121)), ((0.25 * p.p1788) * p.p1788)), 0.5);
            s.store_scalar(170, (0.5 * (((-p.p1790) * (210.0 - p.p1787)) + ((((((-p.p1790) * (210.0 - p.p1787)) * ((-p.p1790) * (210.0 - p.p1787))) + ((0.25 * p.p1789) * p.p1789))) as f64).sqrt())));
            s.store_add_scaled_inputs4_offset_indices(171, 119, 1.0, 120, 1.0, 169, -1.0, 170, -1.0, 210.0);
            s.store_add_scaled_inputs3_sqrt_third_sub_square_offset(118, 116, 0.5, 171, 0.5, 116, 171, ((0.25 * 0.2) * 0.2), 0.5);
        }

        if ((s.b[1291] && (!s.b[1292])) && (!s.b[1295])) {
            s.store_add_scaled_inputs3_sqrt_third_sub_square_offset(169, 228, 0.5, 121, 0.5, 228, 121, ((0.25 * p.p1788) * p.p1788), 0.5);
            s.store_scaled_add_sqrt_square_offset_ad(170, A::scaled_offset(s.ad_value(228), (-p.p1787), (-p.p1790)), ((0.25 * p.p1789) * p.p1789), 0.5);
        }

        s.b[1296] = (s.v[228] > s.v[121]);
        s.store_scalar(1296, if s.b[1296] { 1.0 } else { 0.0 });

        if (((s.b[1291] && (!s.b[1292])) && (!s.b[1295])) && s.b[1296]) {
            s.store_add_ad_lhs(171, A::add_scaled_inputs4(s.ad_value(119), 1.0, s.ad_value(120), 1.0, s.ad_value(169), -1.0, s.ad_value(170), -1.0), 228);
        }

        if (((s.b[1291] && (!s.b[1292])) && (!s.b[1295])) && (!s.b[1296])) {
            s.store_add_ad_lhs(171, A::add_scaled_inputs4(s.ad_value(119), 1.0, s.ad_value(120), 1.0, s.ad_value(169), -1.0, s.ad_value(170), -1.0), 121);
        }

        if ((s.b[1291] && (!s.b[1292])) && (!s.b[1295])) {
            s.store_add_scaled_inputs3_sqrt_third_sub_square_offset(172, 116, 0.5, 171, 0.5, 116, 171, ((0.25 * 0.2) * 0.2), 0.5);
            s.store_add_scaled_products_indices(118, 313, 172, 1.0, 312, 116, 1.0);
        }

        if (s.b[1291] && (!s.b[1292])) {
            s.store_scaled_sub_offset_sqrt_square_offset(117, 116, 210.0, (-210.0), ((0.25 * 0.2) * 0.2), 0.5);
            s.store_add_scaled_inputs3_offset_mixed_iia(233, 117, 1.0, 228, (-0.5), A::sqrt_square_offset(A::offset(s.ad_value(228), (-210.0)), ((0.25 * 0.2) * 0.2)), (-(-0.5)), ((-0.5) * 210.0));
            s.store_div_scaled_offset_numerator(234, s.ad_value(117), 1.0, (-210.0), s.ad_value(228), 1.0);
        }

        if s.b[1291] {
            s.store_scale(182, 118, 8.617087e-5);
        }

        s.store_sub_from_scalar_ad(146, p.p106, A::div_scaled_product_offset_denominator(s.ad_value(116), s.ad_value(116), p.p1718, s.ad_value(116), p.p1719, 1.0));

        s.store_sub_from_scalar_ad(147, p.p106, A::div_scaled_product_offset_denominator(s.ad_value(228), s.ad_value(228), p.p1718, s.ad_value(228), p.p1719, 1.0));

        s.store_mul_scaled_sqrt_scaled_input_rhs(169, 116, 1.0 / (300.15), 116, 1.0 / (300.15));

        s.store_mul_scaled_limited_exp_ad_rhs(141, 169, p.p105, A::sub_from_scalar((p.p106 / ((2.0 * 8.617087e-5) * 300.15)), A::div_scaled_inputs(s.ad_value(146), 1.0, s.ad_value(179), 2.0)));

        s.b[1297] = (p.p80 == 0.0);
        s.store_scalar(1297, if s.b[1297] { 1.0 } else { 0.0 });

        if s.b[1297] {
            s.store_scale(148, 169, p.p107);
        }

        if (!s.b[1297]) {
            s.store_mul_scaled_sqrt_scaled_input_rhs(148, 118, (1.0 / (300.15) * p.p107), 118, 1.0 / (300.15));
        }

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

        s.store_scale(389, 179, 1.60219e-19);

        s.store_div_from_scalar_ad(168, (1.05457e-34 * 3.141592653589793), A::div_scaled_inputs(s.ad_value(894), 2.0, s.ad_value(895), 1.0));

        s.store_scaled_square(377, 168, 1.0 / ((2.0 * s.v[381])));

        s.store_scaled_square(378, 168, 1.0 / ((2.0 * s.v[382])));

        s.store_scale(379, 377, 4.0);

        s.store_scale(380, 378, 4.0);

        s.store_scalar(169, ((s.v[385] * s.v[384]) / (s.v[386] * s.v[383])));

        s.store_offset_scaled_ad(387, A::limited_exp(A::div_scaled_inputs2(s.ad_value(377), 1.0, s.ad_value(378), (-1.0), s.ad_value(389), 1.0)), s.v[169], 1.0);

        s.store_add_scaled_inputs3_mixed_iaa(388, 387, 1.0, A::limited_exp(A::div_scaled_inputs2(s.ad_value(377), 1.0, s.ad_value(379), (-1.0), s.ad_value(389), 1.0)), 1.0, A::limited_exp(A::div_scaled_inputs2(s.ad_value(377), 1.0, s.ad_value(380), (-1.0), s.ad_value(389), 1.0)), s.v[169]);

        s.store_mul_scaled_ad_rhs(170, 179, -1.0, {
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
        });

        s.store_mul_add_scaled_inputs_rhs(375, 654, s.ad_value(377), 6.241457005723417e18, s.ad_value(170), 1.0);

        s.store_ln(418, 229);

        s.b[1298] = (p.p80 == 0.0);
        s.store_scalar(1298, if s.b[1298] { 1.0 } else { 0.0 });

        if s.b[1298] {
            s.store_mul_exp_ad_rhs(169, 704, A::mul(s.ad_value(836), s.ad_value(418)));
            s.store_add_scaled_inputs4_offset_mixed_iiaa(413, 169, 1.0, 169, (-0.9), A::add_scaled_product(s.ad_value(169), (-(-0.9)), s.ad_value(838), s.ad_value(232), 1.0), 0.5, A::sqrt(A::sub_scaled_inputs(A::square(A::offset(A::add_scaled_product(s.ad_value(169), (-(-0.9)), s.ad_value(838), s.ad_value(232), 1.0), (-0.0001))), 1.0, s.ad_value(169), ((-0.9) * (4.0 * 0.0001)))), 0.5, (0.5 * (-0.0001)));
        }

        s.b[1299] = (p.p66 == 1.0);
        s.store_scalar(1299, if s.b[1299] { 1.0 } else { 0.0 });

        if (s.b[1298] && s.b[1299]) {
            s.store_mul_exp_ad_rhs(169, 706, A::mul(s.ad_value(845), s.ad_value(418)));
            s.store_add_scaled_inputs4_offset_mixed_iiaa(321, 169, 1.0, 169, (-0.9), A::add_scaled_product(s.ad_value(169), (-(-0.9)), s.ad_value(846), s.ad_value(232), 1.0), 0.5, A::sqrt(A::sub_scaled_inputs(A::square(A::offset(A::add_scaled_product(s.ad_value(169), (-(-0.9)), s.ad_value(846), s.ad_value(232), 1.0), (-0.0001))), 1.0, s.ad_value(169), ((-0.9) * (4.0 * 0.0001)))), 0.5, (0.5 * (-0.0001)));
            s.copy_ad(417, 321);
        }

        if s.b[1298] {
            s.store_add_scaled_inputs4_offset_mixed_iaai(303, 807, 1.0, A::add_scaled_product(s.ad_value(807), 1.0, s.ad_value(823), s.ad_value(232), 1.0), 0.5, A::sqrt(A::sub_scaled_inputs(A::square(A::offset(A::add_scaled_product(s.ad_value(807), 1.0, s.ad_value(823), s.ad_value(232), 1.0), (-1e-6))), 1.0, s.ad_value(807), (-(4.0 * 1e-6)))), 0.5, 807, (-1.0), (0.5 * (-1e-6)));
            s.copy_ad(323, 811);
        }

        s.b[1300] = (p.p66 != 0.0);
        s.store_scalar(1300, if s.b[1300] { 1.0 } else { 0.0 });

        if (s.b[1298] && s.b[1300]) {
            s.store_add_scaled_inputs4_offset_mixed_iaai(305, 815, 1.0, A::add_scaled_product(s.ad_value(815), 1.0, s.ad_value(825), s.ad_value(232), 1.0), 0.5, A::sqrt(A::sub_scaled_inputs(A::square(A::offset(A::add_scaled_product(s.ad_value(815), 1.0, s.ad_value(825), s.ad_value(232), 1.0), (-1e-6))), 1.0, s.ad_value(815), (-(4.0 * 1e-6)))), 0.5, 815, (-1.0), (0.5 * (-1e-6)));
        }

        if s.b[1298] {
            s.store_mul_exp_ad_rhs(318, 812, A::mul(s.ad_value(830), s.ad_value(418)));
        }

        s.b[1301] = (p.p66 != 0.0);
        s.store_scalar(1301, if s.b[1301] { 1.0 } else { 0.0 });

        if (s.b[1298] && s.b[1301]) {
            s.store_mul_exp_ad_rhs(320, 818, A::mul(s.ad_value(844), s.ad_value(418)));
        }

        if s.b[1298] {
            s.store_mul_exp_ad_rhs(317, 814, A::mul(s.ad_value(834), s.ad_value(418)));
        }

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

        s.b[1302] = (p.p75 != 0.0);
        s.store_scalar(1302, if s.b[1302] { 1.0 } else { 0.0 });

        if (s.b[1298] && s.b[1302]) {
            s.store_add_scaled_inputs4_offset_mixed_iaai(332, 679, 1.0, A::add_scaled_product(s.ad_value(679), 1.0, s.ad_value(849), s.ad_value(232), -1.0), 0.5, A::sqrt(A::sub_scaled_inputs(A::square(A::offset(A::add_scaled_product(s.ad_value(679), 1.0, s.ad_value(849), s.ad_value(232), -1.0), (-1e-6))), 1.0, s.ad_value(679), (-(4.0 * 1e-6)))), 0.5, 679, (-1.0), (0.5 * (-1e-6)));
        }

        if (s.b[1298] && (!s.b[1302])) {
            s.store_mul_ad_rhs(332, 679, {
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

        s.b[1303] = (p.p66 != 0.0);
        s.store_scalar(1303, if s.b[1303] { 1.0 } else { 0.0 });

        s.b[1304] = (p.p75 != 0.0);
        s.store_scalar(1304, if s.b[1304] { 1.0 } else { 0.0 });

        if ((s.b[1298] && s.b[1303]) && s.b[1304]) {
            s.store_add_scaled_inputs4_offset_mixed_iaai(333, 680, 1.0, A::add_scaled_product(s.ad_value(680), 1.0, s.ad_value(851), s.ad_value(232), -1.0), 0.5, A::sqrt(A::sub_scaled_inputs(A::square(A::offset(A::add_scaled_product(s.ad_value(680), 1.0, s.ad_value(851), s.ad_value(232), -1.0), (-1e-6))), 1.0, s.ad_value(680), (-(4.0 * 1e-6)))), 0.5, 680, (-1.0), (0.5 * (-1e-6)));
        }

        if ((s.b[1298] && s.b[1303]) && (!s.b[1304])) {
            s.store_mul_ad_rhs(333, 680, {
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

        s.b[1305] = (s.v[333] < 1000.0);
        s.store_scalar(1305, if s.b[1305] { 1.0 } else { 0.0 });

        if ((s.b[1298] && s.b[1303]) && s.b[1305]) {
            s.store_scalar(333, 1000.0);
        }

        s.b[1306] = (p.p67 == 1.0);
        s.store_scalar(1306, if s.b[1306] { 1.0 } else { 0.0 });

        if (s.b[1298] && s.b[1306]) {
            s.store_mul_exp_ad_rhs(169, 705, A::mul(s.ad_value(839), s.ad_value(418)));
            s.store_add_scaled_inputs4_offset_mixed_iiaa(414, 169, 1.0, 169, (-0.9), A::add_scaled_product(s.ad_value(169), (-(-0.9)), s.ad_value(841), s.ad_value(232), 1.0), 0.5, A::sqrt(A::sub_scaled_inputs(A::square(A::offset(A::add_scaled_product(s.ad_value(169), (-(-0.9)), s.ad_value(841), s.ad_value(232), 1.0), (-0.0001))), 1.0, s.ad_value(169), ((-0.9) * (4.0 * 0.0001)))), 0.5, (0.5 * (-0.0001)));
            s.store_add_scaled_inputs4_offset_mixed_iaai(304, 808, 1.0, A::add_scaled_product(s.ad_value(808), 1.0, s.ad_value(826), s.ad_value(232), 1.0), 0.5, A::sqrt(A::sub_scaled_inputs(A::square(A::offset(A::add_scaled_product(s.ad_value(808), 1.0, s.ad_value(826), s.ad_value(232), 1.0), (-1e-6))), 1.0, s.ad_value(808), (-(4.0 * 1e-6)))), 0.5, 808, (-1.0), (0.5 * (-1e-6)));
            s.store_mul_exp_ad_rhs(319, 813, A::mul(s.ad_value(832), s.ad_value(418)));
        }

        s.b[1307] = (p.p75 != 0.0);
        s.store_scalar(1307, if s.b[1307] { 1.0 } else { 0.0 });

        if (s.b[1298] && s.b[1307]) {
            s.store_add_scaled_inputs4_offset_mixed_iaai(334, 698, 1.0, A::add_scaled_product(s.ad_value(698), 1.0, s.ad_value(849), s.ad_value(232), -1.0), 0.5, A::sqrt(A::sub_scaled_inputs(A::square(A::offset(A::add_scaled_product(s.ad_value(698), 1.0, s.ad_value(849), s.ad_value(232), -1.0), (-1e-6))), 1.0, s.ad_value(698), (-(4.0 * 1e-6)))), 0.5, 698, (-1.0), (0.5 * (-1e-6)));
        }

        if (s.b[1298] && (!s.b[1307])) {
            s.store_mul_ad_rhs(334, 698, {
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

        s.b[1308] = (p.p66 != 0.0);
        s.store_scalar(1308, if s.b[1308] { 1.0 } else { 0.0 });

        s.b[1309] = (p.p75 != 0.0);
        s.store_scalar(1309, if s.b[1309] { 1.0 } else { 0.0 });

        if ((s.b[1298] && s.b[1308]) && s.b[1309]) {
            s.store_add_scaled_inputs4_offset_mixed_iaai(335, 699, 1.0, A::add_scaled_product(s.ad_value(699), 1.0, s.ad_value(849), s.ad_value(232), -1.0), 0.5, A::sqrt(A::sub_scaled_inputs(A::square(A::offset(A::add_scaled_product(s.ad_value(699), 1.0, s.ad_value(849), s.ad_value(232), -1.0), (-1e-6))), 1.0, s.ad_value(699), (-(4.0 * 1e-6)))), 0.5, 699, (-1.0), (0.5 * (-1e-6)));
        }

        if ((s.b[1298] && s.b[1308]) && (!s.b[1309])) {
            s.store_mul_ad_rhs(335, 699, {
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

        s.b[1310] = (s.v[335] < 1000.0);
        s.store_scalar(1310, if s.b[1310] { 1.0 } else { 0.0 });

        if ((s.b[1298] && s.b[1308]) && s.b[1310]) {
            s.store_scalar(335, 1000.0);
        }

        s.b[1311] = (p.p75 != 0.0);
        s.store_scalar(1311, if s.b[1311] { 1.0 } else { 0.0 });

        if (s.b[1298] && s.b[1311]) {
            s.store_add_scaled_inputs4_offset_mixed_iaai(336, 702, 1.0, A::add_scaled_product(s.ad_value(702), 1.0, s.ad_value(850), s.ad_value(232), -1.0), 0.5, A::sqrt(A::sub_scaled_inputs(A::square(A::offset(A::add_scaled_product(s.ad_value(702), 1.0, s.ad_value(850), s.ad_value(232), -1.0), (-1e-6))), 1.0, s.ad_value(702), (-(4.0 * 1e-6)))), 0.5, 702, (-1.0), (0.5 * (-1e-6)));
        }

    }

    pub(super) fn stamp_reactive_block_10(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (s.b[1298] && (!s.b[1311])) {
            s.store_mul_ad_rhs(336, 702, {
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

        s.b[1312] = (p.p66 != 0.0);
        s.store_scalar(1312, if s.b[1312] { 1.0 } else { 0.0 });

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

        if s.b[1298] {
            s.copy_ad(660, 657);
            s.copy_ad(797, 792);
            s.store_mul_add_ad_lhs(231, s.ad_value(858), A::div_from_scalar(p.p1720, s.ad_value(153)), 230);
        }

        s.b[1313] = (p.p80 == 1.0);
        s.store_scalar(1313, if s.b[1313] { 1.0 } else { 0.0 });

        if ((!s.b[1298]) && s.b[1313]) {
            s.store_mul_exp_ad_rhs(169, 704, A::mul(A::add_scaled_product(s.ad_value(836), 1.0, s.ad_value(837), s.ad_value(229), 1.0), s.ad_value(418)));
            s.store_add_scaled_inputs4_offset_mixed_iiaa(413, 169, 1.0, 169, (-0.9), A::add_scaled_product(s.ad_value(169), (-(-0.9)), s.ad_value(838), s.ad_value(232), 1.0), 0.5, A::sqrt(A::sub_scaled_inputs(A::square(A::offset(A::add_scaled_product(s.ad_value(169), (-(-0.9)), s.ad_value(838), s.ad_value(232), 1.0), (-0.0001))), 1.0, s.ad_value(169), ((-0.9) * (4.0 * 0.0001)))), 0.5, (0.5 * (-0.0001)));
        }

        s.b[1314] = (p.p66 == 1.0);
        s.store_scalar(1314, if s.b[1314] { 1.0 } else { 0.0 });

        if (((!s.b[1298]) && s.b[1313]) && s.b[1314]) {
            s.store_mul_exp_ad_rhs(169, 706, A::mul(A::add_scaled_product(s.ad_value(845), 1.0, s.ad_value(837), s.ad_value(229), 1.0), s.ad_value(418)));
            s.store_add_scaled_inputs4_offset_mixed_iiaa(321, 169, 1.0, 169, (-0.9), A::add_scaled_product(s.ad_value(169), (-(-0.9)), s.ad_value(846), s.ad_value(232), 1.0), 0.5, A::sqrt(A::sub_scaled_inputs(A::square(A::offset(A::add_scaled_product(s.ad_value(169), (-(-0.9)), s.ad_value(846), s.ad_value(232), 1.0), (-0.0001))), 1.0, s.ad_value(169), ((-0.9) * (4.0 * 0.0001)))), 0.5, (0.5 * (-0.0001)));
            s.copy_ad(417, 321);
        }

        if ((!s.b[1298]) && s.b[1313]) {
            s.store_mul_exp_ad_rhs(303, 807, A::mul(A::add_scaled_product(s.ad_value(823), 1.0, s.ad_value(824), s.ad_value(229), 1.0), s.ad_value(418)));
        }

        s.b[1315] = (p.p66 != 0.0);
        s.store_scalar(1315, if s.b[1315] { 1.0 } else { 0.0 });

        if (((!s.b[1298]) && s.b[1313]) && s.b[1315]) {
            s.store_mul_exp_ad_rhs(305, 815, A::mul(A::add_scaled_product(s.ad_value(825), 1.0, s.ad_value(824), s.ad_value(229), 1.0), s.ad_value(418)));
        }

        if ((!s.b[1298]) && s.b[1313]) {
            s.store_mul_exp_ad_rhs(318, 812, A::mul(A::add_scaled_product(s.ad_value(830), 1.0, s.ad_value(831), s.ad_value(229), 1.0), s.ad_value(418)));
        }

        s.b[1316] = (p.p66 != 0.0);
        s.store_scalar(1316, if s.b[1316] { 1.0 } else { 0.0 });

        if (((!s.b[1298]) && s.b[1313]) && s.b[1316]) {
            s.store_mul_exp_ad_rhs(320, 818, A::mul(A::add_scaled_product(s.ad_value(844), 1.0, s.ad_value(831), s.ad_value(229), 1.0), s.ad_value(418)));
        }

        if ((!s.b[1298]) && s.b[1313]) {
            s.store_mul_exp_ad_rhs(317, 814, A::mul(A::add_scaled_inputs(s.ad_value(834), 1.0, s.ad_value(229), p.p881), s.ad_value(418)));
            s.store_mul_offset_ad_rhs(324, 325, A::limited_exp(A::mul(s.ad_value(326), s.ad_value(230))), (-1.0));
            s.store_mul_offset_ad_rhs(327, 328, A::limited_exp(A::mul(s.ad_value(329), s.ad_value(230))), (-1.0));
            s.store_offset(330, 324, 0.5);
            s.store_offset(331, 327, 0.5);
        }

        s.b[1317] = (p.p75 != 0.0);
        s.store_scalar(1317, if s.b[1317] { 1.0 } else { 0.0 });

        if (((!s.b[1298]) && s.b[1313]) && s.b[1317]) {
            s.store_add_scaled_inputs4_offset_mixed_iaai(323, 811, 1.0, A::add_scaled_product(s.ad_value(811), 1.0, s.ad_value(847), s.ad_value(232), 1.0), 0.5, A::sqrt(A::sub_scaled_inputs(A::square(A::offset(A::add_scaled_product(s.ad_value(811), 1.0, s.ad_value(847), s.ad_value(232), 1.0), (-1e-6))), 1.0, s.ad_value(811), (-(4.0 * 1e-6)))), 0.5, 811, (-1.0), (0.5 * (-1e-6)));
        }

        if (((!s.b[1298]) && s.b[1313]) && (!s.b[1317])) {
            s.store_mul_ad_rhs(323, 811, {
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

        s.b[1318] = (p.p67 == 1.0);
        s.store_scalar(1318, if s.b[1318] { 1.0 } else { 0.0 });

        if (((!s.b[1298]) && s.b[1313]) && s.b[1318]) {
            s.store_mul_exp_ad_rhs(169, 705, A::mul(A::add_scaled_product(s.ad_value(839), 1.0, s.ad_value(840), s.ad_value(229), 1.0), s.ad_value(418)));
            s.store_add_scaled_inputs4_offset_mixed_iiaa(414, 169, 1.0, 169, (-0.9), A::add_scaled_product(s.ad_value(169), (-(-0.9)), s.ad_value(841), s.ad_value(232), 1.0), 0.5, A::sqrt(A::sub_scaled_inputs(A::square(A::offset(A::add_scaled_product(s.ad_value(169), (-(-0.9)), s.ad_value(841), s.ad_value(232), 1.0), (-0.0001))), 1.0, s.ad_value(169), ((-0.9) * (4.0 * 0.0001)))), 0.5, (0.5 * (-0.0001)));
            s.store_mul_exp_ad_rhs(304, 808, A::mul(A::add_scaled_product(s.ad_value(826), 1.0, s.ad_value(827), s.ad_value(229), 1.0), s.ad_value(418)));
            s.store_mul_exp_ad_rhs(319, 813, A::mul(A::add_scaled_product(s.ad_value(832), 1.0, s.ad_value(833), s.ad_value(229), 1.0), s.ad_value(418)));
        }

        s.b[1319] = (s.v[854] == s.v[855]);
        s.store_scalar(1319, if s.b[1319] { 1.0 } else { 0.0 });

        if (((!s.b[1298]) && s.b[1313]) && s.b[1319]) {
            s.store_offset_mul(170, 854, 232, 1.0);
        }

        s.b[1320] = (s.v[856] < s.v[228]);
        s.store_scalar(1320, if s.b[1320] { 1.0 } else { 0.0 });

        if ((((!s.b[1298]) && s.b[1313]) && (!s.b[1319])) && s.b[1320]) {
            s.store_offset_mul(195, 854, 232, 1.0);
            s.store_add_scaled_product_mixed_aia(196, A::offset(A::mul(s.ad_value(855), A::sub(s.ad_value(116), s.ad_value(856))), 1.0), 1.0, 854, A::sub(s.ad_value(856), s.ad_value(228)), 1.0);
            s.store_mul_sub_by_sub(171, 854, 855, 856, 228);
        }

        s.b[1321] = (s.v[855] < s.v[854]);
        s.store_scalar(1321, if s.b[1321] { 1.0 } else { 0.0 });

        if (((((!s.b[1298]) && s.b[1313]) && (!s.b[1319])) && s.b[1320]) && s.b[1321]) {
            s.store_sub_ad(170, A::add_scaled_inputs3(s.ad_value(195), 0.5, s.ad_value(196), 0.5, A::sqrt(A::add_scaled_square_product(A::sub(s.ad_value(195), s.ad_value(196)), 1.0, s.ad_value(857), s.ad_value(857), 0.25)), 0.5), A::add_scaled_inputs(s.ad_value(171), 0.5, A::sqrt(A::add_scaled_square_product(s.ad_value(171), 1.0, s.ad_value(857), s.ad_value(857), 0.25)), 0.5));
        }

        if (((((!s.b[1298]) && s.b[1313]) && (!s.b[1319])) && s.b[1320]) && (!s.b[1321])) {
            s.store_sub_ad(170, A::add_scaled_inputs3(s.ad_value(195), 0.5, s.ad_value(196), 0.5, A::sqrt(A::add_scaled_square_product(A::sub(s.ad_value(195), s.ad_value(196)), 1.0, s.ad_value(857), s.ad_value(857), 0.25)), (-0.5)), A::sub_scaled_inputs(s.ad_value(171), 0.5, A::sqrt(A::add_scaled_square_product(s.ad_value(171), 1.0, s.ad_value(857), s.ad_value(857), 0.25)), 0.5));
        }

        if ((((!s.b[1298]) && s.b[1313]) && (!s.b[1319])) && (!s.b[1320])) {
            s.store_offset_mul_ad(196, s.ad_value(855), A::sub(s.ad_value(116), s.ad_value(228)), 1.0);
            s.store_add_scaled_product_mixed_aia(195, A::offset(A::mul(s.ad_value(854), A::sub(s.ad_value(116), s.ad_value(856))), 1.0), 1.0, 855, A::sub(s.ad_value(856), s.ad_value(228)), 1.0);
            s.store_mul_sub_by_sub(171, 855, 854, 856, 228);
        }

        s.b[1322] = (s.v[855] < s.v[854]);
        s.store_scalar(1322, if s.b[1322] { 1.0 } else { 0.0 });

        if (((((!s.b[1298]) && s.b[1313]) && (!s.b[1319])) && (!s.b[1320])) && s.b[1322]) {
            s.store_sub_ad(170, A::add_scaled_inputs3(s.ad_value(196), 0.5, s.ad_value(195), 0.5, A::sqrt(A::add_scaled_square_product(A::sub(s.ad_value(196), s.ad_value(195)), 1.0, s.ad_value(857), s.ad_value(857), 0.25)), 0.5), A::add_scaled_inputs(s.ad_value(171), 0.5, A::sqrt(A::add_scaled_square_product(s.ad_value(171), 1.0, s.ad_value(857), s.ad_value(857), 0.25)), 0.5));
        }

        if (((((!s.b[1298]) && s.b[1313]) && (!s.b[1319])) && (!s.b[1320])) && (!s.b[1322])) {
            s.store_sub_ad(170, A::add_scaled_inputs3(s.ad_value(196), 0.5, s.ad_value(195), 0.5, A::sqrt(A::add_scaled_square_product(A::sub(s.ad_value(196), s.ad_value(195)), 1.0, s.ad_value(857), s.ad_value(857), 0.25)), (-0.5)), A::sub_scaled_inputs(s.ad_value(171), 0.5, A::sqrt(A::add_scaled_square_product(s.ad_value(171), 1.0, s.ad_value(857), s.ad_value(857), 0.25)), 0.5));
        }

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

        s.b[1323] = (p.p75 != 0.0);
        s.store_scalar(1323, if s.b[1323] { 1.0 } else { 0.0 });

        if (((!s.b[1298]) && s.b[1313]) && s.b[1323]) {
            s.store_add_scaled_inputs3_mixed_iai(332, 679, 1.0, A::add_scaled_inputs3_offset(A::add_scaled_products(s.ad_value(849), s.ad_value(232), -1.0, s.ad_value(232), s.ad_value(232), p.p561), 0.5, s.ad_value(679), 0.5, A::sqrt(A::sub_scaled_inputs(A::square(A::offset(A::sub_scaled_inputs(A::add_scaled_products(s.ad_value(849), s.ad_value(232), -1.0, s.ad_value(232), s.ad_value(232), p.p561), 1.0, s.ad_value(679), -1.0), (-1e-6))), 1.0, s.ad_value(679), (-(4.0 * 1e-6)))), 0.5, ((-1e-6) * 0.5)), 1.0, 679, (-1.0));
        }

        if (((!s.b[1298]) && s.b[1313]) && (!s.b[1323])) {
            s.store_mul_ad_rhs(332, 679, {
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

        s.b[1324] = (p.p66 != 0.0);
        s.store_scalar(1324, if s.b[1324] { 1.0 } else { 0.0 });

        s.b[1325] = (p.p75 != 0.0);
        s.store_scalar(1325, if s.b[1325] { 1.0 } else { 0.0 });

        if ((((!s.b[1298]) && s.b[1313]) && s.b[1324]) && s.b[1325]) {
            s.store_add_scaled_inputs3_mixed_iai(333, 680, 1.0, A::add_scaled_inputs3_offset(A::add_scaled_products(s.ad_value(851), s.ad_value(232), -1.0, s.ad_value(232), s.ad_value(232), p.p561), 0.5, s.ad_value(680), 0.5, A::sqrt(A::sub_scaled_inputs(A::square(A::offset(A::sub_scaled_inputs(A::add_scaled_products(s.ad_value(851), s.ad_value(232), -1.0, s.ad_value(232), s.ad_value(232), p.p561), 1.0, s.ad_value(680), -1.0), (-1e-6))), 1.0, s.ad_value(680), (-(4.0 * 1e-6)))), 0.5, ((-1e-6) * 0.5)), 1.0, 680, (-1.0));
        }

        if ((((!s.b[1298]) && s.b[1313]) && s.b[1324]) && (!s.b[1325])) {
            s.store_mul_ad_rhs(333, 680, {
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

        s.b[1326] = (s.v[333] < 1000.0);
        s.store_scalar(1326, if s.b[1326] { 1.0 } else { 0.0 });

        if ((((!s.b[1298]) && s.b[1313]) && s.b[1324]) && s.b[1326]) {
            s.store_scalar(333, 1000.0);
        }

        s.b[1327] = (p.p75 != 0.0);
        s.store_scalar(1327, if s.b[1327] { 1.0 } else { 0.0 });

        if (((!s.b[1298]) && s.b[1313]) && s.b[1327]) {
            s.store_add_scaled_inputs3_mixed_iai(334, 698, 1.0, A::add_scaled_inputs3_offset(A::add_scaled_products(s.ad_value(849), s.ad_value(232), -1.0, s.ad_value(232), s.ad_value(232), p.p561), 0.5, s.ad_value(698), 0.5, A::sqrt(A::sub_scaled_inputs(A::square(A::offset(A::sub_scaled_inputs(A::add_scaled_products(s.ad_value(849), s.ad_value(232), -1.0, s.ad_value(232), s.ad_value(232), p.p561), 1.0, s.ad_value(698), -1.0), (-1e-6))), 1.0, s.ad_value(698), (-(4.0 * 1e-6)))), 0.5, ((-1e-6) * 0.5)), 1.0, 698, (-1.0));
        }

        if (((!s.b[1298]) && s.b[1313]) && (!s.b[1327])) {
            s.store_mul_ad_rhs(334, 698, {
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

        s.b[1328] = (p.p66 != 0.0);
        s.store_scalar(1328, if s.b[1328] { 1.0 } else { 0.0 });

        s.b[1329] = (p.p75 != 0.0);
        s.store_scalar(1329, if s.b[1329] { 1.0 } else { 0.0 });

        if ((((!s.b[1298]) && s.b[1313]) && s.b[1328]) && s.b[1329]) {
            s.store_add_scaled_inputs3_mixed_iai(335, 699, 1.0, A::add_scaled_inputs3_offset(A::add_scaled_products(s.ad_value(849), s.ad_value(232), -1.0, s.ad_value(232), s.ad_value(232), p.p561), 0.5, s.ad_value(699), 0.5, A::sqrt(A::sub_scaled_inputs(A::square(A::offset(A::sub_scaled_inputs(A::add_scaled_products(s.ad_value(849), s.ad_value(232), -1.0, s.ad_value(232), s.ad_value(232), p.p561), 1.0, s.ad_value(699), -1.0), (-1e-6))), 1.0, s.ad_value(699), (-(4.0 * 1e-6)))), 0.5, ((-1e-6) * 0.5)), 1.0, 699, (-1.0));
        }

        if ((((!s.b[1298]) && s.b[1313]) && s.b[1328]) && (!s.b[1329])) {
            s.store_mul_ad_rhs(335, 699, {
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

        s.b[1330] = (s.v[335] < 1000.0);
        s.store_scalar(1330, if s.b[1330] { 1.0 } else { 0.0 });

        if ((((!s.b[1298]) && s.b[1313]) && s.b[1328]) && s.b[1330]) {
            s.store_scalar(335, 1000.0);
        }

        s.b[1331] = (p.p75 != 0.0);
        s.store_scalar(1331, if s.b[1331] { 1.0 } else { 0.0 });

        if (((!s.b[1298]) && s.b[1313]) && s.b[1331]) {
            s.store_add_scaled_inputs3_mixed_iai(336, 702, 1.0, A::add_scaled_inputs3_offset(A::add_scaled_products(s.ad_value(850), s.ad_value(232), -1.0, s.ad_value(232), s.ad_value(232), p.p574), 0.5, s.ad_value(702), 0.5, A::sqrt(A::sub_scaled_inputs(A::square(A::offset(A::sub_scaled_inputs(A::add_scaled_products(s.ad_value(850), s.ad_value(232), -1.0, s.ad_value(232), s.ad_value(232), p.p574), 1.0, s.ad_value(702), -1.0), (-1e-6))), 1.0, s.ad_value(702), (-(4.0 * 1e-6)))), 0.5, ((-1e-6) * 0.5)), 1.0, 702, (-1.0));
        }

        if (((!s.b[1298]) && s.b[1313]) && (!s.b[1331])) {
            s.store_mul_ad_rhs(336, 702, {
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

        s.b[1332] = (p.p66 != 0.0);
        s.store_scalar(1332, if s.b[1332] { 1.0 } else { 0.0 });

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

        s.b[1333] = (p.p75 != 0.0);
        s.store_scalar(1333, if s.b[1333] { 1.0 } else { 0.0 });

        if (((!s.b[1298]) && s.b[1313]) && s.b[1333]) {
            s.store_add_scaled_inputs3_mixed_iai(660, 657, 1.0, A::add_scaled_inputs3_offset(A::add_scaled_product(s.ad_value(232), p.p498, s.ad_value(232), s.ad_value(232), p.p499), 0.5, s.ad_value(657), 0.5, A::sqrt(A::sub_scaled_inputs(A::square(A::offset(A::sub_scaled_inputs(A::add_scaled_product(s.ad_value(232), p.p498, s.ad_value(232), s.ad_value(232), p.p499), 1.0, s.ad_value(657), -1.0), (-1e-6))), 1.0, s.ad_value(657), (-(4.0 * 1e-6)))), 0.5, ((-1e-6) * 0.5)), 1.0, 657, (-1.0));
        }

        if (((!s.b[1298]) && s.b[1313]) && (!s.b[1333])) {
            s.store_mul_ad_rhs(660, 657, {
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

        s.b[1334] = (p.p75 != 0.0);
        s.store_scalar(1334, if s.b[1334] { 1.0 } else { 0.0 });

        if (((!s.b[1298]) && s.b[1313]) && s.b[1334]) {
            s.store_add_scaled_inputs3_mixed_iai(797, 792, 1.0, A::add_scaled_inputs3_offset(s.ad_value(232), (p.p1026 * 0.5), s.ad_value(792), 0.5, A::sqrt(A::sub_scaled_inputs(A::square(A::offset(A::sub_scaled_inputs(s.ad_value(232), p.p1026, s.ad_value(792), -1.0), (-1e-6))), 1.0, s.ad_value(792), (-(4.0 * 1e-6)))), 0.5, ((-1e-6) * 0.5)), 1.0, 792, (-1.0));
        }

        if (((!s.b[1298]) && s.b[1313]) && (!s.b[1334])) {
            s.store_mul_ad_rhs(797, 792, {
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

        if ((!s.b[1298]) && s.b[1313]) {
            s.store_sub_ad(231, A::add_scaled_product(A::div_scalar_offset_denominator(p.p1747, A::limited_exp_scaled_input(A::offset(s.ad_value(116), (-p.p1749)), p.p1748), 1.0, 1.0), 1.0, A::add(s.ad_value(858), A::div_from_scalar(p.p1720, s.ad_value(153))), s.ad_value(230), 1.0), A::div_scalar_offset_denominator(p.p1747, A::limited_exp_scaled_input(A::offset(s.ad_value(228), (-p.p1749)), p.p1748), 1.0, 1.0));
        }

        if ((!s.b[1298]) && (!s.b[1313])) {
            s.store_mul_exp_ad_rhs(169, 704, A::mul(A::add_scaled_product(s.ad_value(836), 1.0, s.ad_value(837), s.ad_value(234), 1.0), s.ad_value(418)));
            s.store_add_scaled_inputs4_offset_mixed_iiaa(413, 169, 1.0, 169, (-0.9), A::add_scaled_product(s.ad_value(169), (-(-0.9)), s.ad_value(838), s.ad_value(232), 1.0), 0.5, A::sqrt(A::sub_scaled_inputs(A::square(A::offset(A::add_scaled_product(s.ad_value(169), (-(-0.9)), s.ad_value(838), s.ad_value(232), 1.0), (-0.0001))), 1.0, s.ad_value(169), ((-0.9) * (4.0 * 0.0001)))), 0.5, (0.5 * (-0.0001)));
        }

        s.b[1335] = (p.p66 == 1.0);
        s.store_scalar(1335, if s.b[1335] { 1.0 } else { 0.0 });

        if (((!s.b[1298]) && (!s.b[1313])) && s.b[1335]) {
            s.store_mul_exp_ad_rhs(169, 706, A::mul(A::add_scaled_product(s.ad_value(845), 1.0, s.ad_value(837), s.ad_value(234), 1.0), s.ad_value(418)));
            s.store_add_scaled_inputs4_offset_mixed_iiaa(321, 169, 1.0, 169, (-0.9), A::add_scaled_product(s.ad_value(169), (-(-0.9)), s.ad_value(846), s.ad_value(232), 1.0), 0.5, A::sqrt(A::sub_scaled_inputs(A::square(A::offset(A::add_scaled_product(s.ad_value(169), (-(-0.9)), s.ad_value(846), s.ad_value(232), 1.0), (-0.0001))), 1.0, s.ad_value(169), ((-0.9) * (4.0 * 0.0001)))), 0.5, (0.5 * (-0.0001)));
            s.copy_ad(417, 321);
        }

        s.b[1336] = (s.v[228] > 210.0);
        s.store_scalar(1336, if s.b[1336] { 1.0 } else { 0.0 });

        if (((!s.b[1298]) && (!s.b[1313])) && s.b[1336]) {
            s.store_scaled_sub_ad(170, A::div(s.ad_value(823), A::add(s.ad_value(807), A::mul_sub_from_scalar_rhs(s.ad_value(823), 210.0, s.ad_value(228)))), A::div_scaled_product_offset_rhs(s.ad_value(824), {
                if (!((210.0 / s.v[228]) > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if ((210.0 / s.v[228]) > 1e-38) {
                            A::ln(A::div_from_scalar(210.0, s.ad_value(228)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 1.0, 1.0, s.ad_value(228), 1.0), 210.0);
        }

        if (((!s.b[1298]) && (!s.b[1313])) && s.b[1336]) {
            s.store_div_scaled_inputs2_mixed_iaa(169, 807, 1.0, A::mul_sub_from_scalar_rhs(s.ad_value(823), 210.0, s.ad_value(228)), 1.0, A::pow(A::div_from_scalar(210.0, s.ad_value(228)), A::add_scaled_product(s.ad_value(170), 1.0, s.ad_value(824), A::div_from_scalar(210.0, s.ad_value(228)), 1.0)), 1.0);
        }

    }

    pub(super) fn stamp_reactive_block_11(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (((!s.b[1298]) && (!s.b[1313])) && s.b[1336]) {
            s.store_mul_pow_ad_rhs(306, 169, s.ad_value(229), A::add_scaled_product(s.ad_value(170), 1.0, s.ad_value(824), s.ad_value(229), 1.0));
            s.store_add_scaled_product_indices(307, 807, 1.0, 823, 232, 1.0);
        }

        if (((!s.b[1298]) && (!s.b[1313])) && (!s.b[1336])) {
            s.store_mul_ad_product_rhs(170, 807, A::pow(A::div_from_scalar(210.0, s.ad_value(228)), A::add_scaled_product(s.ad_value(823), 1.0, s.ad_value(824), A::div_from_scalar(210.0, s.ad_value(228)), 1.0)), A::add_scaled_inputs(s.ad_value(823), 0.004761904761904762, A::div_scaled_product_offset_rhs(s.ad_value(824), {
                if (!((210.0 / s.v[228]) > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if ((210.0 / s.v[228]) > 1e-38) {
                            A::ln(A::div_from_scalar(210.0, s.ad_value(228)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 1.0, 1.0, s.ad_value(228), 1.0), 1.0));
        }

        if (((!s.b[1298]) && (!s.b[1313])) && (!s.b[1336])) {
            s.store_add_scaled_product_mixed_aia(169, A::mul_sub_from_scalar_rhs(s.ad_value(170), 210.0, s.ad_value(228)), (-1.0), 807, A::pow(A::div_from_scalar(210.0, s.ad_value(228)), A::add_scaled_product(s.ad_value(823), 1.0, s.ad_value(824), A::div_from_scalar(210.0, s.ad_value(228)), 1.0)), 1.0);
            s.store_mul_pow_ad_rhs(306, 807, s.ad_value(229), A::add_scaled_product(s.ad_value(823), 1.0, s.ad_value(824), s.ad_value(229), 1.0));
            s.store_add_scaled_product_indices(307, 169, 1.0, 170, 232, 1.0);
        }

        if ((!s.b[1298]) && (!s.b[1313])) {
            s.store_add_scaled_products_indices(168, 313, 306, 1.0, 312, 307, 1.0);
        }

        if ((!s.b[1298]) && (!s.b[1313])) {
            if (!(s.v[168] < ((-10000.0) * 1e-6))) {
                s.store_scaled_add_sqrt_square_offset_rhs(303, 168, 168, ((4.0 * 1e-6) * 1e-6), 0.5);
            } else {
                if (s.v[168] < ((-10000.0) * 1e-6)) {
                    s.store_div_from_scalar(303, ((-1e-6) * 1e-6), 168);
                } else {
                    s.store_scalar(303, 0.0);
                }
            }
        }

        s.b[1337] = (p.p66 != 0.0);
        s.store_scalar(1337, if s.b[1337] { 1.0 } else { 0.0 });

        s.b[1338] = (s.v[228] > 210.0);
        s.store_scalar(1338, if s.b[1338] { 1.0 } else { 0.0 });

        if ((((!s.b[1298]) && (!s.b[1313])) && s.b[1337]) && s.b[1338]) {
            s.store_scaled_sub_ad(170, A::div(s.ad_value(825), A::add(s.ad_value(815), A::mul_sub_from_scalar_rhs(s.ad_value(825), 210.0, s.ad_value(228)))), A::div_scaled_product_offset_rhs(s.ad_value(824), {
                if (!((210.0 / s.v[228]) > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if ((210.0 / s.v[228]) > 1e-38) {
                            A::ln(A::div_from_scalar(210.0, s.ad_value(228)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 1.0, 1.0, s.ad_value(228), 1.0), 210.0);
        }

        if ((((!s.b[1298]) && (!s.b[1313])) && s.b[1337]) && s.b[1338]) {
            s.store_div_scaled_inputs2_mixed_iaa(169, 815, 1.0, A::mul_sub_from_scalar_rhs(s.ad_value(825), 210.0, s.ad_value(228)), 1.0, A::pow(A::div_from_scalar(210.0, s.ad_value(228)), A::add_scaled_product(s.ad_value(170), 1.0, s.ad_value(824), A::div_from_scalar(210.0, s.ad_value(228)), 1.0)), 1.0);
            s.store_mul_pow_ad_rhs(310, 169, s.ad_value(229), A::add_scaled_product(s.ad_value(170), 1.0, s.ad_value(824), s.ad_value(229), 1.0));
            s.store_add_scaled_product_indices(311, 815, 1.0, 825, 232, 1.0);
        }

        if ((((!s.b[1298]) && (!s.b[1313])) && s.b[1337]) && (!s.b[1338])) {
            s.store_mul_ad_product_rhs(170, 815, A::pow(A::div_from_scalar(210.0, s.ad_value(228)), A::add_scaled_product(s.ad_value(825), 1.0, s.ad_value(824), A::div_from_scalar(210.0, s.ad_value(228)), 1.0)), A::add_scaled_inputs(s.ad_value(825), 0.004761904761904762, A::div_scaled_product_offset_rhs(s.ad_value(824), {
                if (!((210.0 / s.v[228]) > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if ((210.0 / s.v[228]) > 1e-38) {
                            A::ln(A::div_from_scalar(210.0, s.ad_value(228)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 1.0, 1.0, s.ad_value(228), 1.0), 1.0));
        }

        if ((((!s.b[1298]) && (!s.b[1313])) && s.b[1337]) && (!s.b[1338])) {
            s.store_add_scaled_product_mixed_aia(169, A::mul_sub_from_scalar_rhs(s.ad_value(170), 210.0, s.ad_value(228)), (-1.0), 815, A::pow(A::div_from_scalar(210.0, s.ad_value(228)), A::add_scaled_product(s.ad_value(825), 1.0, s.ad_value(824), A::div_from_scalar(210.0, s.ad_value(228)), 1.0)), 1.0);
            s.store_mul_pow_ad_rhs(310, 815, s.ad_value(229), A::add_scaled_product(s.ad_value(825), 1.0, s.ad_value(824), s.ad_value(229), 1.0));
            s.store_add_scaled_product_indices(311, 169, 1.0, 170, 232, 1.0);
        }

        if (((!s.b[1298]) && (!s.b[1313])) && s.b[1337]) {
            s.store_add_scaled_products_indices(168, 313, 310, 1.0, 312, 311, 1.0);
        }

        if (((!s.b[1298]) && (!s.b[1313])) && s.b[1337]) {
            if (!(s.v[168] < ((-10000.0) * 1e-6))) {
                s.store_scaled_add_sqrt_square_offset_rhs(305, 168, 168, ((4.0 * 1e-6) * 1e-6), 0.5);
            } else {
                if (s.v[168] < ((-10000.0) * 1e-6)) {
                    s.store_div_from_scalar(305, ((-1e-6) * 1e-6), 168);
                } else {
                    s.store_scalar(305, 0.0);
                }
            }
        }

        if ((!s.b[1298]) && (!s.b[1313])) {
            s.store_mul_exp_ad_rhs(318, 812, A::mul(A::add_scaled_product(s.ad_value(830), 1.0, s.ad_value(831), s.ad_value(234), 1.0), s.ad_value(418)));
        }

        s.b[1339] = (p.p66 != 0.0);
        s.store_scalar(1339, if s.b[1339] { 1.0 } else { 0.0 });

        if (((!s.b[1298]) && (!s.b[1313])) && s.b[1339]) {
            s.store_mul_exp_ad_rhs(320, 818, A::mul(A::add_scaled_product(s.ad_value(844), 1.0, s.ad_value(831), s.ad_value(234), 1.0), s.ad_value(418)));
        }

        if ((!s.b[1298]) && (!s.b[1313])) {
            s.store_mul_exp_ad_rhs(317, 814, A::mul(A::add_scaled_product(s.ad_value(834), 1.0, s.ad_value(835), s.ad_value(234), 1.0), s.ad_value(418)));
        }

        s.b[1340] = (((((s.v[326] * (s.v[228] - 210.0)) / s.v[228])) as f64).abs() < 1e-6);
        s.store_scalar(1340, if s.b[1340] { 1.0 } else { 0.0 });

        if (((!s.b[1298]) && (!s.b[1313])) && s.b[1340]) {
            s.store_mul_offset_ad_rhs(324, 325, A::limited_exp(A::mul(s.ad_value(326), s.ad_value(234))), (-1.0));
        }

        if (((!s.b[1298]) && (!s.b[1313])) && (!s.b[1340])) {
            s.store_div_scaled_product_offset_rhs(324, s.ad_value(325), A::limited_exp(A::mul(s.ad_value(326), s.ad_value(234))), (-1.0), 1.0, A::abs(A::offset(A::limited_exp(A::div_scaled_product_offset_rhs(s.ad_value(326), s.ad_value(228), (-210.0), 1.0, s.ad_value(228), 1.0)), (-1.0))), 1.0);
        }

        s.b[1341] = (((((s.v[329] * (s.v[228] - 210.0)) / s.v[228])) as f64).abs() < 1e-6);
        s.store_scalar(1341, if s.b[1341] { 1.0 } else { 0.0 });

        if (((!s.b[1298]) && (!s.b[1313])) && s.b[1341]) {
            s.store_mul_offset_ad_rhs(327, 328, A::limited_exp(A::mul(s.ad_value(329), s.ad_value(234))), (-1.0));
        }

        if (((!s.b[1298]) && (!s.b[1313])) && (!s.b[1341])) {
            s.store_div_scaled_product_offset_rhs(327, s.ad_value(328), A::limited_exp(A::mul(s.ad_value(329), s.ad_value(234))), (-1.0), 1.0, A::abs(A::offset(A::limited_exp(A::div_scaled_product_offset_rhs(s.ad_value(329), s.ad_value(228), (-210.0), 1.0, s.ad_value(228), 1.0)), (-1.0))), 1.0);
        }

        if ((!s.b[1298]) && (!s.b[1313])) {
            s.store_offset(330, 324, 0.5);
            s.store_offset(331, 327, 0.5);
        }

        s.b[1342] = (p.p75 != 0.0);
        s.store_scalar(1342, if s.b[1342] { 1.0 } else { 0.0 });

        if (((!s.b[1298]) && (!s.b[1313])) && s.b[1342]) {
            s.store_add_scaled_inputs4_offset_mixed_iaai(323, 811, 1.0, A::add_scaled_product(s.ad_value(811), 1.0, s.ad_value(847), s.ad_value(233), 1.0), 0.5, A::sqrt(A::sub_scaled_inputs(A::square(A::offset(A::add_scaled_product(s.ad_value(811), 1.0, s.ad_value(847), s.ad_value(233), 1.0), (-1e-6))), 1.0, s.ad_value(811), (-(4.0 * 1e-6)))), 0.5, 811, (-1.0), (0.5 * (-1e-6)));
            s.store_add_scaled_inputs3_mixed_iai(332, 679, 1.0, A::add_scaled_inputs3_offset(A::add_scaled_products(s.ad_value(849), s.ad_value(232), -1.0, s.ad_value(233), s.ad_value(233), p.p561), 0.5, s.ad_value(679), 0.5, A::sqrt(A::sub_scaled_inputs(A::square(A::offset(A::sub_scaled_inputs(A::add_scaled_products(s.ad_value(849), s.ad_value(232), -1.0, s.ad_value(233), s.ad_value(233), p.p561), 1.0, s.ad_value(679), -1.0), (-1e-6))), 1.0, s.ad_value(679), (-(4.0 * 1e-6)))), 0.5, ((-1e-6) * 0.5)), 1.0, 679, (-1.0));
        }

        s.b[1343] = (p.p66 != 0.0);
        s.store_scalar(1343, if s.b[1343] { 1.0 } else { 0.0 });

        if ((((!s.b[1298]) && (!s.b[1313])) && s.b[1342]) && s.b[1343]) {
            s.store_add_scaled_inputs3_mixed_iai(333, 680, 1.0, A::add_scaled_inputs3_offset(A::add_scaled_products(s.ad_value(851), s.ad_value(232), -1.0, s.ad_value(233), s.ad_value(233), p.p561), 0.5, s.ad_value(680), 0.5, A::sqrt(A::sub_scaled_inputs(A::square(A::offset(A::sub_scaled_inputs(A::add_scaled_products(s.ad_value(851), s.ad_value(232), -1.0, s.ad_value(233), s.ad_value(233), p.p561), 1.0, s.ad_value(680), -1.0), (-1e-6))), 1.0, s.ad_value(680), (-(4.0 * 1e-6)))), 0.5, ((-1e-6) * 0.5)), 1.0, 680, (-1.0));
        }

        s.b[1344] = (s.v[333] < 1000.0);
        s.store_scalar(1344, if s.b[1344] { 1.0 } else { 0.0 });

        if (((((!s.b[1298]) && (!s.b[1313])) && s.b[1342]) && s.b[1343]) && s.b[1344]) {
            s.store_scalar(333, 1000.0);
        }

        if (((!s.b[1298]) && (!s.b[1313])) && s.b[1342]) {
            s.store_add_scaled_inputs3_mixed_iai(334, 698, 1.0, A::add_scaled_inputs3_offset(A::add_scaled_products(s.ad_value(849), s.ad_value(232), -1.0, s.ad_value(233), s.ad_value(233), p.p561), 0.5, s.ad_value(698), 0.5, A::sqrt(A::sub_scaled_inputs(A::square(A::offset(A::sub_scaled_inputs(A::add_scaled_products(s.ad_value(849), s.ad_value(232), -1.0, s.ad_value(233), s.ad_value(233), p.p561), 1.0, s.ad_value(698), -1.0), (-1e-6))), 1.0, s.ad_value(698), (-(4.0 * 1e-6)))), 0.5, ((-1e-6) * 0.5)), 1.0, 698, (-1.0));
        }

        s.b[1345] = (p.p66 != 0.0);
        s.store_scalar(1345, if s.b[1345] { 1.0 } else { 0.0 });

        if ((((!s.b[1298]) && (!s.b[1313])) && s.b[1342]) && s.b[1345]) {
            s.store_add_scaled_inputs3_mixed_iai(335, 699, 1.0, A::add_scaled_inputs3_offset(A::add_scaled_products(s.ad_value(849), s.ad_value(232), -1.0, s.ad_value(233), s.ad_value(233), p.p561), 0.5, s.ad_value(699), 0.5, A::sqrt(A::sub_scaled_inputs(A::square(A::offset(A::sub_scaled_inputs(A::add_scaled_products(s.ad_value(849), s.ad_value(232), -1.0, s.ad_value(233), s.ad_value(233), p.p561), 1.0, s.ad_value(699), -1.0), (-1e-6))), 1.0, s.ad_value(699), (-(4.0 * 1e-6)))), 0.5, ((-1e-6) * 0.5)), 1.0, 699, (-1.0));
        }

        s.b[1346] = (s.v[335] < 1000.0);
        s.store_scalar(1346, if s.b[1346] { 1.0 } else { 0.0 });

        if (((((!s.b[1298]) && (!s.b[1313])) && s.b[1342]) && s.b[1345]) && s.b[1346]) {
            s.store_scalar(335, 1000.0);
        }

        if (((!s.b[1298]) && (!s.b[1313])) && s.b[1342]) {
            s.store_add_scaled_inputs3_mixed_iai(336, 702, 1.0, A::add_scaled_inputs3_offset(A::add_scaled_products(s.ad_value(850), s.ad_value(232), -1.0, s.ad_value(233), s.ad_value(233), p.p574), 0.5, s.ad_value(702), 0.5, A::sqrt(A::sub_scaled_inputs(A::square(A::offset(A::sub_scaled_inputs(A::add_scaled_products(s.ad_value(850), s.ad_value(232), -1.0, s.ad_value(233), s.ad_value(233), p.p574), 1.0, s.ad_value(702), -1.0), (-1e-6))), 1.0, s.ad_value(702), (-(4.0 * 1e-6)))), 0.5, ((-1e-6) * 0.5)), 1.0, 702, (-1.0));
            s.store_add_scaled_inputs3_mixed_iai(660, 657, 1.0, A::add_scaled_inputs3_offset(A::add_scaled_product(s.ad_value(233), p.p498, s.ad_value(233), s.ad_value(233), p.p499), 0.5, s.ad_value(657), 0.5, A::sqrt(A::sub_scaled_inputs(A::square(A::offset(A::sub_scaled_inputs(A::add_scaled_product(s.ad_value(233), p.p498, s.ad_value(233), s.ad_value(233), p.p499), 1.0, s.ad_value(657), -1.0), (-1e-6))), 1.0, s.ad_value(657), (-(4.0 * 1e-6)))), 0.5, ((-1e-6) * 0.5)), 1.0, 657, (-1.0));
            s.store_add_scaled_inputs3_mixed_iai(797, 792, 1.0, A::add_scaled_inputs3_offset(s.ad_value(233), (p.p1026 * 0.5), s.ad_value(792), 0.5, A::sqrt(A::sub_scaled_inputs(A::square(A::offset(A::sub_scaled_inputs(s.ad_value(233), p.p1026, s.ad_value(792), -1.0), (-1e-6))), 1.0, s.ad_value(792), (-(4.0 * 1e-6)))), 0.5, ((-1e-6) * 0.5)), 1.0, 792, (-1.0));
        }

        if (((!s.b[1298]) && (!s.b[1313])) && (!s.b[1342])) {
            s.store_mul_ad_rhs(323, 811, {
                if (!(((1.0 + (s.v[847] * s.v[233])) - 1e-6) < ((-10000.0) * 0.001))) {
                    A::add_scaled_inputs(A::offset(A::mul(s.ad_value(847), s.ad_value(233)), ((1.0) + ((-1e-6)))), 0.5, A::sqrt_square_offset(A::offset(A::mul(s.ad_value(847), s.ad_value(233)), ((1.0) + ((-1e-6)))), ((4.0 * 0.001) * 0.001)), 0.5)
                } else {
                    {
                        if (((1.0 + (s.v[847] * s.v[233])) - 1e-6) < ((-10000.0) * 0.001)) {
                            A::div_scalar_offset_denominator(((-0.001) * 0.001), A::mul(s.ad_value(847), s.ad_value(233)), ((1.0) + ((-1e-6))), 1.0)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            });
        }

        if (((!s.b[1298]) && (!s.b[1313])) && (!s.b[1342])) {
            s.store_mul_ad_rhs(332, 679, {
                if (!((((1.0 - (s.v[849] * s.v[232])) + ((p.p561 * s.v[233]) * s.v[233])) - 1e-6) < ((-10000.0) * 0.001))) {
                    A::add_scaled_inputs(A::offset(A::add_scaled_sub_value_product(1.0, A::mul(s.ad_value(849), s.ad_value(232)), 1.0, s.ad_value(233), s.ad_value(233), p.p561), (-1e-6)), 0.5, A::sqrt_square_offset(A::offset(A::add_scaled_sub_value_product(1.0, A::mul(s.ad_value(849), s.ad_value(232)), 1.0, s.ad_value(233), s.ad_value(233), p.p561), (-1e-6)), ((4.0 * 0.001) * 0.001)), 0.5)
                } else {
                    {
                        if ((((1.0 - (s.v[849] * s.v[232])) + ((p.p561 * s.v[233]) * s.v[233])) - 1e-6) < ((-10000.0) * 0.001)) {
                            A::div_scalar_offset_denominator(((-0.001) * 0.001), A::add_scaled_sub_value_product(1.0, A::mul(s.ad_value(849), s.ad_value(232)), 1.0, s.ad_value(233), s.ad_value(233), p.p561), (-1e-6), 1.0)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            });
        }

        s.b[1347] = (p.p66 != 0.0);
        s.store_scalar(1347, if s.b[1347] { 1.0 } else { 0.0 });

        if ((((!s.b[1298]) && (!s.b[1313])) && (!s.b[1342])) && s.b[1347]) {
            s.store_mul_ad_rhs(333, 680, {
                if (!((((1.0 - (s.v[851] * s.v[232])) + ((p.p561 * s.v[233]) * s.v[233])) - 1e-6) < ((-10000.0) * 0.001))) {
                    A::add_scaled_inputs(A::offset(A::add_scaled_sub_value_product(1.0, A::mul(s.ad_value(851), s.ad_value(232)), 1.0, s.ad_value(233), s.ad_value(233), p.p561), (-1e-6)), 0.5, A::sqrt_square_offset(A::offset(A::add_scaled_sub_value_product(1.0, A::mul(s.ad_value(851), s.ad_value(232)), 1.0, s.ad_value(233), s.ad_value(233), p.p561), (-1e-6)), ((4.0 * 0.001) * 0.001)), 0.5)
                } else {
                    {
                        if ((((1.0 - (s.v[851] * s.v[232])) + ((p.p561 * s.v[233]) * s.v[233])) - 1e-6) < ((-10000.0) * 0.001)) {
                            A::div_scalar_offset_denominator(((-0.001) * 0.001), A::add_scaled_sub_value_product(1.0, A::mul(s.ad_value(851), s.ad_value(232)), 1.0, s.ad_value(233), s.ad_value(233), p.p561), (-1e-6), 1.0)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            });
        }

        s.b[1348] = (s.v[333] < 1000.0);
        s.store_scalar(1348, if s.b[1348] { 1.0 } else { 0.0 });

        if (((((!s.b[1298]) && (!s.b[1313])) && (!s.b[1342])) && s.b[1347]) && s.b[1348]) {
            s.store_scalar(333, 1000.0);
        }

        if (((!s.b[1298]) && (!s.b[1313])) && (!s.b[1342])) {
            s.store_mul_ad_rhs(334, 698, {
                if (!((((1.0 - (s.v[849] * s.v[232])) + ((p.p561 * s.v[233]) * s.v[233])) - 1e-6) < ((-10000.0) * 0.001))) {
                    A::add_scaled_inputs(A::offset(A::add_scaled_sub_value_product(1.0, A::mul(s.ad_value(849), s.ad_value(232)), 1.0, s.ad_value(233), s.ad_value(233), p.p561), (-1e-6)), 0.5, A::sqrt_square_offset(A::offset(A::add_scaled_sub_value_product(1.0, A::mul(s.ad_value(849), s.ad_value(232)), 1.0, s.ad_value(233), s.ad_value(233), p.p561), (-1e-6)), ((4.0 * 0.001) * 0.001)), 0.5)
                } else {
                    {
                        if ((((1.0 - (s.v[849] * s.v[232])) + ((p.p561 * s.v[233]) * s.v[233])) - 1e-6) < ((-10000.0) * 0.001)) {
                            A::div_scalar_offset_denominator(((-0.001) * 0.001), A::add_scaled_sub_value_product(1.0, A::mul(s.ad_value(849), s.ad_value(232)), 1.0, s.ad_value(233), s.ad_value(233), p.p561), (-1e-6), 1.0)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            });
        }

        s.b[1349] = (p.p66 != 0.0);
        s.store_scalar(1349, if s.b[1349] { 1.0 } else { 0.0 });

        if ((((!s.b[1298]) && (!s.b[1313])) && (!s.b[1342])) && s.b[1349]) {
            s.store_mul_ad_rhs(335, 699, {
                if (!((((1.0 - (s.v[849] * s.v[232])) + ((p.p561 * s.v[233]) * s.v[233])) - 1e-6) < ((-10000.0) * 0.001))) {
                    A::add_scaled_inputs(A::offset(A::add_scaled_sub_value_product(1.0, A::mul(s.ad_value(849), s.ad_value(232)), 1.0, s.ad_value(233), s.ad_value(233), p.p561), (-1e-6)), 0.5, A::sqrt_square_offset(A::offset(A::add_scaled_sub_value_product(1.0, A::mul(s.ad_value(849), s.ad_value(232)), 1.0, s.ad_value(233), s.ad_value(233), p.p561), (-1e-6)), ((4.0 * 0.001) * 0.001)), 0.5)
                } else {
                    {
                        if ((((1.0 - (s.v[849] * s.v[232])) + ((p.p561 * s.v[233]) * s.v[233])) - 1e-6) < ((-10000.0) * 0.001)) {
                            A::div_scalar_offset_denominator(((-0.001) * 0.001), A::add_scaled_sub_value_product(1.0, A::mul(s.ad_value(849), s.ad_value(232)), 1.0, s.ad_value(233), s.ad_value(233), p.p561), (-1e-6), 1.0)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            });
        }

        s.b[1350] = (s.v[335] < 1000.0);
        s.store_scalar(1350, if s.b[1350] { 1.0 } else { 0.0 });

        if (((((!s.b[1298]) && (!s.b[1313])) && (!s.b[1342])) && s.b[1349]) && s.b[1350]) {
            s.store_scalar(335, 1000.0);
        }

        if (((!s.b[1298]) && (!s.b[1313])) && (!s.b[1342])) {
            s.store_mul_ad_rhs(336, 702, {
                if (!((((1.0 - (s.v[850] * s.v[232])) + ((p.p574 * s.v[233]) * s.v[233])) - 1e-6) < ((-10000.0) * 0.001))) {
                    A::add_scaled_inputs(A::offset(A::add_scaled_sub_value_product(1.0, A::mul(s.ad_value(850), s.ad_value(232)), 1.0, s.ad_value(233), s.ad_value(233), p.p574), (-1e-6)), 0.5, A::sqrt_square_offset(A::offset(A::add_scaled_sub_value_product(1.0, A::mul(s.ad_value(850), s.ad_value(232)), 1.0, s.ad_value(233), s.ad_value(233), p.p574), (-1e-6)), ((4.0 * 0.001) * 0.001)), 0.5)
                } else {
                    {
                        if ((((1.0 - (s.v[850] * s.v[232])) + ((p.p574 * s.v[233]) * s.v[233])) - 1e-6) < ((-10000.0) * 0.001)) {
                            A::div_scalar_offset_denominator(((-0.001) * 0.001), A::add_scaled_sub_value_product(1.0, A::mul(s.ad_value(850), s.ad_value(232)), 1.0, s.ad_value(233), s.ad_value(233), p.p574), (-1e-6), 1.0)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            });
        }

        if (((!s.b[1298]) && (!s.b[1313])) && (!s.b[1342])) {
            s.store_mul_ad_rhs(660, 657, {
                if (!((((1.0 + (p.p498 * s.v[233])) + ((p.p499 * s.v[233]) * s.v[233])) - 1e-6) < ((-10000.0) * 0.001))) {
                    A::add_scaled_inputs(A::offset(A::add_scaled_product(A::scale_offset(s.ad_value(233), p.p498, 1.0), 1.0, s.ad_value(233), s.ad_value(233), p.p499), (-1e-6)), 0.5, A::sqrt_square_offset(A::offset(A::add_scaled_product(A::scale_offset(s.ad_value(233), p.p498, 1.0), 1.0, s.ad_value(233), s.ad_value(233), p.p499), (-1e-6)), ((4.0 * 0.001) * 0.001)), 0.5)
                } else {
                    {
                        if ((((1.0 + (p.p498 * s.v[233])) + ((p.p499 * s.v[233]) * s.v[233])) - 1e-6) < ((-10000.0) * 0.001)) {
                            A::div_scalar_offset_denominator(((-0.001) * 0.001), A::add_scaled_product(A::scale_offset(s.ad_value(233), p.p498, 1.0), 1.0, s.ad_value(233), s.ad_value(233), p.p499), (-1e-6), 1.0)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            });
        }

        if (((!s.b[1298]) && (!s.b[1313])) && (!s.b[1342])) {
            s.store_mul_ad_rhs(797, 792, {
                if (!(((1.0 + (p.p1026 * s.v[233])) - 1e-6) < ((-10000.0) * 0.001))) {
                    A::add_scaled_inputs(A::scale_offset(s.ad_value(233), p.p1026, ((1.0) + ((-1e-6)))), 0.5, A::sqrt_square_offset(A::scale_offset(s.ad_value(233), p.p1026, ((1.0) + ((-1e-6)))), ((4.0 * 0.001) * 0.001)), 0.5)
                } else {
                    {
                        if (((1.0 + (p.p1026 * s.v[233])) - 1e-6) < ((-10000.0) * 0.001)) {
                            A::div_from_scalar(((-0.001) * 0.001), A::scale_offset(s.ad_value(233), p.p1026, ((1.0) + ((-1e-6)))))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            });
        }

        if ((!s.b[1298]) && (!s.b[1313])) {
            s.store_offset_ad(337, {
                if (!(((s.v[790] * ((1.0 + (p.p450 * s.v[232])) + ((p.p451 * s.v[233]) * s.v[233]))) - 2.0) < ((-10000.0) * 0.001))) {
                    A::add_scaled_inputs(A::offset(A::mul(s.ad_value(790), A::add_scaled_product(A::scale_offset(s.ad_value(232), p.p450, 1.0), 1.0, s.ad_value(233), s.ad_value(233), p.p451)), (-2.0)), 0.5, A::sqrt_square_offset(A::offset(A::mul(s.ad_value(790), A::add_scaled_product(A::scale_offset(s.ad_value(232), p.p450, 1.0), 1.0, s.ad_value(233), s.ad_value(233), p.p451)), (-2.0)), ((4.0 * 0.001) * 0.001)), 0.5)
                } else {
                    {
                        if (((s.v[790] * ((1.0 + (p.p450 * s.v[232])) + ((p.p451 * s.v[233]) * s.v[233]))) - 2.0) < ((-10000.0) * 0.001)) {
                            A::div_scalar_offset_denominator(((-0.001) * 0.001), A::mul(s.ad_value(790), A::add_scaled_product(A::scale_offset(s.ad_value(232), p.p450, 1.0), 1.0, s.ad_value(233), s.ad_value(233), p.p451)), (-2.0), 1.0)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 2.0);
        }

        s.b[1351] = (p.p66 != 0.0);
        s.store_scalar(1351, if s.b[1351] { 1.0 } else { 0.0 });

        if (((!s.b[1298]) && (!s.b[1313])) && s.b[1351]) {
            s.store_offset_ad(338, {
                if (!(((s.v[791] * ((1.0 + (p.p452 * s.v[232])) + ((p.p451 * s.v[233]) * s.v[233]))) - 2.0) < ((-10000.0) * 0.001))) {
                    A::add_scaled_inputs(A::offset(A::mul(s.ad_value(791), A::add_scaled_product(A::scale_offset(s.ad_value(232), p.p452, 1.0), 1.0, s.ad_value(233), s.ad_value(233), p.p451)), (-2.0)), 0.5, A::sqrt_square_offset(A::offset(A::mul(s.ad_value(791), A::add_scaled_product(A::scale_offset(s.ad_value(232), p.p452, 1.0), 1.0, s.ad_value(233), s.ad_value(233), p.p451)), (-2.0)), ((4.0 * 0.001) * 0.001)), 0.5)
                } else {
                    {
                        if (((s.v[791] * ((1.0 + (p.p452 * s.v[232])) + ((p.p451 * s.v[233]) * s.v[233]))) - 2.0) < ((-10000.0) * 0.001)) {
                            A::div_scalar_offset_denominator(((-0.001) * 0.001), A::mul(s.ad_value(791), A::add_scaled_product(A::scale_offset(s.ad_value(232), p.p452, 1.0), 1.0, s.ad_value(233), s.ad_value(233), p.p451)), (-2.0), 1.0)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 2.0);
        }

        s.b[1352] = (p.p67 == 1.0);
        s.store_scalar(1352, if s.b[1352] { 1.0 } else { 0.0 });

        if (((!s.b[1298]) && (!s.b[1313])) && s.b[1352]) {
            s.store_mul_exp_ad_rhs(169, 705, A::mul(A::add_scaled_product(s.ad_value(839), 1.0, s.ad_value(840), s.ad_value(234), 1.0), s.ad_value(418)));
            s.store_add_scaled_inputs4_offset_mixed_iiaa(414, 169, 1.0, 169, (-0.9), A::add_scaled_product(s.ad_value(169), (-(-0.9)), s.ad_value(841), s.ad_value(232), 1.0), 0.5, A::sqrt(A::sub_scaled_inputs(A::square(A::offset(A::add_scaled_product(s.ad_value(169), (-(-0.9)), s.ad_value(841), s.ad_value(232), 1.0), (-0.0001))), 1.0, s.ad_value(169), ((-0.9) * (4.0 * 0.0001)))), 0.5, (0.5 * (-0.0001)));
        }

        s.b[1353] = (s.v[228] > 210.0);
        s.store_scalar(1353, if s.b[1353] { 1.0 } else { 0.0 });

        if ((((!s.b[1298]) && (!s.b[1313])) && s.b[1352]) && s.b[1353]) {
            s.store_scaled_sub_ad(170, A::div(s.ad_value(826), A::add(s.ad_value(808), A::mul_sub_from_scalar_rhs(s.ad_value(826), 210.0, s.ad_value(228)))), A::div_scaled_product_offset_rhs(s.ad_value(827), {
                if (!((210.0 / s.v[228]) > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if ((210.0 / s.v[228]) > 1e-38) {
                            A::ln(A::div_from_scalar(210.0, s.ad_value(228)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 1.0, 1.0, s.ad_value(228), 1.0), 210.0);
        }

        if ((((!s.b[1298]) && (!s.b[1313])) && s.b[1352]) && s.b[1353]) {
            s.store_div_scaled_inputs2_mixed_iaa(169, 808, 1.0, A::mul_sub_from_scalar_rhs(s.ad_value(826), 210.0, s.ad_value(228)), 1.0, A::pow(A::div_from_scalar(210.0, s.ad_value(228)), A::add_scaled_product(s.ad_value(170), 1.0, s.ad_value(827), A::div_from_scalar(210.0, s.ad_value(228)), 1.0)), 1.0);
            s.store_mul_pow_ad_rhs(308, 169, s.ad_value(229), A::add_scaled_product(s.ad_value(170), 1.0, s.ad_value(827), s.ad_value(229), 1.0));
            s.store_add_scaled_product_indices(309, 808, 1.0, 826, 232, 1.0);
        }

        if ((((!s.b[1298]) && (!s.b[1313])) && s.b[1352]) && (!s.b[1353])) {
            s.store_mul_ad_product_rhs(170, 808, A::pow(A::div_from_scalar(210.0, s.ad_value(228)), A::add_scaled_product(s.ad_value(826), 1.0, s.ad_value(827), A::div_from_scalar(210.0, s.ad_value(228)), 1.0)), A::add_scaled_inputs(s.ad_value(826), 0.004761904761904762, A::div_scaled_product_offset_rhs(s.ad_value(827), {
                if (!((210.0 / s.v[228]) > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if ((210.0 / s.v[228]) > 1e-38) {
                            A::ln(A::div_from_scalar(210.0, s.ad_value(228)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 1.0, 1.0, s.ad_value(228), 1.0), 1.0));
        }

        if ((((!s.b[1298]) && (!s.b[1313])) && s.b[1352]) && (!s.b[1353])) {
            s.store_add_scaled_product_mixed_aia(169, A::mul_sub_from_scalar_rhs(s.ad_value(170), 210.0, s.ad_value(228)), (-1.0), 808, A::pow(A::div_from_scalar(210.0, s.ad_value(228)), A::add_scaled_product(s.ad_value(826), 1.0, s.ad_value(827), A::div_from_scalar(210.0, s.ad_value(228)), 1.0)), 1.0);
            s.store_mul_pow_ad_rhs(308, 808, s.ad_value(229), A::add_scaled_product(s.ad_value(826), 1.0, s.ad_value(827), s.ad_value(229), 1.0));
            s.store_add_scaled_product_indices(309, 169, 1.0, 170, 232, 1.0);
        }

        if (((!s.b[1298]) && (!s.b[1313])) && s.b[1352]) {
            s.store_add_scaled_products_indices(168, 313, 308, 1.0, 312, 309, 1.0);
        }

        if (((!s.b[1298]) && (!s.b[1313])) && s.b[1352]) {
            if (!(s.v[168] < ((-10000.0) * 1e-6))) {
                s.store_scaled_add_sqrt_square_offset_rhs(304, 168, 168, ((4.0 * 1e-6) * 1e-6), 0.5);
            } else {
                if (s.v[168] < ((-10000.0) * 1e-6)) {
                    s.store_div_from_scalar(304, ((-1e-6) * 1e-6), 168);
                } else {
                    s.store_scalar(304, 0.0);
                }
            }
        }

    }

    pub(super) fn stamp_reactive_block_12(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (((!s.b[1298]) && (!s.b[1313])) && s.b[1352]) {
            s.store_mul_exp_ad_rhs(319, 813, A::mul(A::add_scaled_product(s.ad_value(832), 1.0, s.ad_value(833), s.ad_value(234), 1.0), s.ad_value(418)));
        }

        s.b[1354] = (s.v[854] == s.v[855]);
        s.store_scalar(1354, if s.b[1354] { 1.0 } else { 0.0 });

        if (((!s.b[1298]) && (!s.b[1313])) && s.b[1354]) {
            s.store_offset_mul(170, 854, 232, 1.0);
        }

        s.b[1355] = (s.v[856] < 210.0);
        s.store_scalar(1355, if s.b[1355] { 1.0 } else { 0.0 });

        s.b[1356] = (s.v[228] > 210.0);
        s.store_scalar(1356, if s.b[1356] { 1.0 } else { 0.0 });

        if (((((!s.b[1298]) && (!s.b[1313])) && (!s.b[1354])) && s.b[1355]) && s.b[1356]) {
            s.store_offset_mul(195, 854, 232, 1.0);
            s.store_add_scaled_product_mixed_aia(196, A::offset(A::mul(s.ad_value(855), A::sub(s.ad_value(116), s.ad_value(856))), 1.0), 1.0, 854, A::sub(s.ad_value(856), s.ad_value(228)), 1.0);
            s.store_offset_ad(171, A::mul_sub_from_scalar_rhs(s.ad_value(854), 210.0, s.ad_value(228)), 1.0);
            s.store_add_scaled_product_mixed_aia(172, A::offset(A::mul_sub_from_scalar_rhs(s.ad_value(855), 210.0, s.ad_value(856)), 1.0), 1.0, 854, A::sub(s.ad_value(856), s.ad_value(228)), 1.0);
        }

        s.b[1357] = (s.v[855] < s.v[854]);
        s.store_scalar(1357, if s.b[1357] { 1.0 } else { 0.0 });

        if ((((((!s.b[1298]) && (!s.b[1313])) && (!s.b[1354])) && s.b[1355]) && s.b[1356]) && s.b[1357]) {
            s.store_add_ad_lhs(174, A::sub(A::add_scaled_inputs3(s.ad_value(195), 0.5, s.ad_value(196), 0.5, A::sqrt(A::add_scaled_square_product(A::sub(s.ad_value(195), s.ad_value(196)), 1.0, s.ad_value(857), s.ad_value(857), 0.25)), 0.5), A::add_scaled_inputs3(s.ad_value(171), 0.5, s.ad_value(172), 0.5, A::sqrt(A::add_scaled_square_product(A::sub(s.ad_value(171), s.ad_value(172)), 1.0, s.ad_value(857), s.ad_value(857), 0.25)), 0.5)), 171);
            s.store_add_scaled_inputs3_sqrt_third_sub_square_offset(170, 174, 0.5, 195, 0.5, 174, 195, ((0.25 * 0.001) * 0.001), 0.5);
        }

        if ((((((!s.b[1298]) && (!s.b[1313])) && (!s.b[1354])) && s.b[1355]) && s.b[1356]) && (!s.b[1357])) {
            s.store_add_ad_lhs(174, A::sub(A::add_scaled_inputs3(s.ad_value(195), 0.5, s.ad_value(196), 0.5, A::sqrt(A::add_scaled_square_product(A::sub(s.ad_value(195), s.ad_value(196)), 1.0, s.ad_value(857), s.ad_value(857), 0.25)), (-0.5)), A::add_scaled_inputs3(s.ad_value(171), 0.5, s.ad_value(172), 0.5, A::sqrt(A::add_scaled_square_product(A::sub(s.ad_value(171), s.ad_value(172)), 1.0, s.ad_value(857), s.ad_value(857), 0.25)), (-0.5))), 171);
            s.store_add_scaled_inputs3_sqrt_third_sub_square_offset(170, 174, 0.5, 195, 0.5, 174, 195, ((0.25 * 0.001) * 0.001), (-0.5));
        }

        s.b[1358] = (s.v[228] > s.v[856]);
        s.store_scalar(1358, if s.b[1358] { 1.0 } else { 0.0 });

        if ((((((!s.b[1298]) && (!s.b[1313])) && (!s.b[1354])) && s.b[1355]) && (!s.b[1356])) && s.b[1358]) {
            s.store_offset_mul(195, 854, 232, 1.0);
            s.store_add_scaled_product_mixed_aia(196, A::offset(A::mul(s.ad_value(855), A::sub(s.ad_value(116), s.ad_value(856))), 1.0), 1.0, 854, A::sub(s.ad_value(856), s.ad_value(228)), 1.0);
            s.store_mul_sub_by_sub(171, 854, 855, 856, 228);
            s.store_offset_ad(172, A::mul_sub_from_scalar_rhs(s.ad_value(854), 210.0, s.ad_value(228)), 1.0);
            s.store_add_scaled_product_mixed_aia(174, A::offset(A::mul_sub_from_scalar_rhs(s.ad_value(855), 210.0, s.ad_value(856)), 1.0), 1.0, 854, A::sub(s.ad_value(856), s.ad_value(228)), 1.0);
        }

        s.b[1359] = (s.v[855] < s.v[854]);
        s.store_scalar(1359, if s.b[1359] { 1.0 } else { 0.0 });

        if (((((((!s.b[1298]) && (!s.b[1313])) && (!s.b[1354])) && s.b[1355]) && (!s.b[1356])) && s.b[1358]) && s.b[1359]) {
            s.store_sub_ad(175, A::add_scaled_inputs3(s.ad_value(195), 0.5, s.ad_value(196), 0.5, A::sqrt(A::add_scaled_square_product(A::sub(s.ad_value(195), s.ad_value(196)), 1.0, s.ad_value(857), s.ad_value(857), 0.25)), 0.5), A::add_scaled_inputs(s.ad_value(171), 0.5, A::sqrt(A::add_scaled_square_product(s.ad_value(171), 1.0, s.ad_value(857), s.ad_value(857), 0.25)), 0.5));
            s.store_sub_ad(176, A::add_scaled_inputs3(s.ad_value(172), 0.5, s.ad_value(174), 0.5, A::sqrt(A::add_scaled_square_product(A::sub(s.ad_value(172), s.ad_value(174)), 1.0, s.ad_value(857), s.ad_value(857), 0.25)), 0.5), A::add_scaled_inputs(s.ad_value(171), 0.5, A::sqrt(A::add_scaled_square_product(s.ad_value(171), 1.0, s.ad_value(857), s.ad_value(857), 0.25)), 0.5));
            s.store_add_scaled_offset_product_rhs(177, 176, 1.0, 854, 116, (-210.0), 1.0);
            s.store_add_scaled_inputs3_sqrt_third_sub_square_offset(170, 175, 0.5, 177, 0.5, 175, 177, ((0.25 * 0.001) * 0.001), 0.5);
        }

        if (((((((!s.b[1298]) && (!s.b[1313])) && (!s.b[1354])) && s.b[1355]) && (!s.b[1356])) && s.b[1358]) && (!s.b[1359])) {
            s.store_sub_ad(175, A::add_scaled_inputs3(s.ad_value(195), 0.5, s.ad_value(196), 0.5, A::sqrt(A::add_scaled_square_product(A::sub(s.ad_value(195), s.ad_value(196)), 1.0, s.ad_value(857), s.ad_value(857), 0.25)), (-0.5)), A::sub_scaled_inputs(s.ad_value(171), 0.5, A::sqrt(A::add_scaled_square_product(s.ad_value(171), 1.0, s.ad_value(857), s.ad_value(857), 0.25)), 0.5));
            s.store_sub_ad(176, A::add_scaled_inputs3(s.ad_value(172), 0.5, s.ad_value(174), 0.5, A::sqrt(A::add_scaled_square_product(A::sub(s.ad_value(172), s.ad_value(174)), 1.0, s.ad_value(857), s.ad_value(857), 0.25)), (-0.5)), A::sub_scaled_inputs(s.ad_value(171), 0.5, A::sqrt(A::add_scaled_square_product(s.ad_value(171), 1.0, s.ad_value(857), s.ad_value(857), 0.25)), 0.5));
            s.store_add_scaled_offset_product_rhs(177, 176, 1.0, 854, 116, (-210.0), 1.0);
            s.store_add_scaled_inputs3_sqrt_third_sub_square_offset(170, 175, 0.5, 177, 0.5, 175, 177, ((0.25 * 0.001) * 0.001), (-0.5));
        }

        if ((((((!s.b[1298]) && (!s.b[1313])) && (!s.b[1354])) && s.b[1355]) && (!s.b[1356])) && (!s.b[1358])) {
            s.store_offset_mul(196, 855, 232, 1.0);
            s.store_add_scaled_product_mixed_aia(195, A::offset(A::mul(s.ad_value(854), A::sub(s.ad_value(116), s.ad_value(856))), 1.0), 1.0, 855, A::sub(s.ad_value(856), s.ad_value(228)), 1.0);
            s.store_mul_sub_by_sub(171, 855, 854, 856, 228);
            s.store_offset_ad(172, A::mul_sub_from_scalar_rhs(s.ad_value(855), 210.0, s.ad_value(228)), 1.0);
            s.store_add_scaled_product_mixed_aia(174, A::offset(A::mul_sub_from_scalar_rhs(s.ad_value(854), 210.0, s.ad_value(856)), 1.0), 1.0, 855, A::sub(s.ad_value(856), s.ad_value(228)), 1.0);
        }

        s.b[1360] = (s.v[855] < s.v[854]);
        s.store_scalar(1360, if s.b[1360] { 1.0 } else { 0.0 });

        if (((((((!s.b[1298]) && (!s.b[1313])) && (!s.b[1354])) && s.b[1355]) && (!s.b[1356])) && (!s.b[1358])) && s.b[1360]) {
            s.store_sub_ad(175, A::add_scaled_inputs3(s.ad_value(195), 0.5, s.ad_value(196), 0.5, A::sqrt(A::add_scaled_square_product(A::sub(s.ad_value(195), s.ad_value(196)), 1.0, s.ad_value(857), s.ad_value(857), 0.25)), 0.5), A::add_scaled_inputs(s.ad_value(171), 0.5, A::sqrt(A::add_scaled_square_product(s.ad_value(171), 1.0, s.ad_value(857), s.ad_value(857), 0.25)), 0.5));
            s.store_sub_ad(176, A::add_scaled_inputs3(s.ad_value(172), 0.5, s.ad_value(174), 0.5, A::sqrt(A::add_scaled_square_product(A::sub(s.ad_value(172), s.ad_value(174)), 1.0, s.ad_value(857), s.ad_value(857), 0.25)), 0.5), A::add_scaled_inputs(s.ad_value(171), 0.5, A::sqrt(A::add_scaled_square_product(s.ad_value(171), 1.0, s.ad_value(857), s.ad_value(857), 0.25)), 0.5));
            s.store_add_scaled_offset_product_rhs(177, 176, 1.0, 854, 116, (-210.0), 1.0);
            s.store_add_scaled_inputs3_sqrt_third_sub_square_offset(170, 175, 0.5, 177, 0.5, 175, 177, ((0.25 * 0.001) * 0.001), 0.5);
        }

        if (((((((!s.b[1298]) && (!s.b[1313])) && (!s.b[1354])) && s.b[1355]) && (!s.b[1356])) && (!s.b[1358])) && (!s.b[1360])) {
            s.store_sub_ad(175, A::add_scaled_inputs3(s.ad_value(195), 0.5, s.ad_value(196), 0.5, A::sqrt(A::add_scaled_square_product(A::sub(s.ad_value(195), s.ad_value(196)), 1.0, s.ad_value(857), s.ad_value(857), 0.25)), (-0.5)), A::sub_scaled_inputs(s.ad_value(171), 0.5, A::sqrt(A::add_scaled_square_product(s.ad_value(171), 1.0, s.ad_value(857), s.ad_value(857), 0.25)), 0.5));
            s.store_sub_ad(176, A::add_scaled_inputs3(s.ad_value(172), 0.5, s.ad_value(174), 0.5, A::sqrt(A::add_scaled_square_product(A::sub(s.ad_value(172), s.ad_value(174)), 1.0, s.ad_value(857), s.ad_value(857), 0.25)), (-0.5)), A::sub_scaled_inputs(s.ad_value(171), 0.5, A::sqrt(A::add_scaled_square_product(s.ad_value(171), 1.0, s.ad_value(857), s.ad_value(857), 0.25)), 0.5));
            s.store_add_scaled_offset_product_rhs(177, 176, 1.0, 854, 116, (-210.0), 1.0);
            s.store_add_scaled_inputs3_sqrt_third_sub_square_offset(170, 175, 0.5, 177, 0.5, 175, 177, ((0.25 * 0.001) * 0.001), (-0.5));
        }

        s.b[1361] = (s.v[228] > 210.0);
        s.store_scalar(1361, if s.b[1361] { 1.0 } else { 0.0 });

        if (((((!s.b[1298]) && (!s.b[1313])) && (!s.b[1354])) && (!s.b[1355])) && s.b[1361]) {
            s.store_offset_mul(195, 854, 232, 1.0);
            s.store_add_ad(196, A::offset(A::mul_offset_rhs(s.ad_value(855), s.ad_value(116), (-210.0)), 1.0), A::mul_sub_from_scalar_rhs(s.ad_value(854), 210.0, s.ad_value(228)));
        }

        s.b[1362] = (s.v[855] < s.v[854]);
        s.store_scalar(1362, if s.b[1362] { 1.0 } else { 0.0 });

        if ((((((!s.b[1298]) && (!s.b[1313])) && (!s.b[1354])) && (!s.b[1355])) && s.b[1361]) && s.b[1362]) {
            s.store_add_scaled_inputs3_sqrt_third_sub_square_offset(170, 195, 0.5, 196, 0.5, 195, 196, ((0.25 * 0.01) * 0.01), 0.5);
        }

        if ((((((!s.b[1298]) && (!s.b[1313])) && (!s.b[1354])) && (!s.b[1355])) && s.b[1361]) && (!s.b[1362])) {
            s.store_add_scaled_inputs3_sqrt_third_sub_square_offset(170, 195, 0.5, 196, 0.5, 195, 196, ((0.25 * 0.01) * 0.01), (-0.5));
        }

        if (((((!s.b[1298]) && (!s.b[1313])) && (!s.b[1354])) && (!s.b[1355])) && (!s.b[1361])) {
            s.store_offset_mul(196, 855, 232, 1.0);
            s.store_add_ad(195, A::offset(A::mul_offset_rhs(s.ad_value(854), s.ad_value(116), (-210.0)), 1.0), A::mul_sub_from_scalar_rhs(s.ad_value(855), 210.0, s.ad_value(228)));
        }

        s.b[1363] = (s.v[855] < s.v[854]);
        s.store_scalar(1363, if s.b[1363] { 1.0 } else { 0.0 });

        if ((((((!s.b[1298]) && (!s.b[1313])) && (!s.b[1354])) && (!s.b[1355])) && (!s.b[1361])) && s.b[1363]) {
            s.store_add_scaled_inputs3_sqrt_third_sub_square_offset(170, 195, 0.5, 196, 0.5, 195, 196, ((0.25 * 0.01) * 0.01), 0.5);
        }

        if ((((((!s.b[1298]) && (!s.b[1313])) && (!s.b[1354])) && (!s.b[1355])) && (!s.b[1361])) && (!s.b[1363])) {
            s.store_add_scaled_inputs3_sqrt_third_sub_square_offset(170, 195, 0.5, 196, 0.5, 195, 196, ((0.25 * 0.01) * 0.01), (-0.5));
        }

        if ((!s.b[1298]) && (!s.b[1313])) {
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

        if ((!s.b[1298]) && (!s.b[1313])) {
            s.store_scaled_sub_offset_sqrt_square_offset(172, 228, 210.0, (-210.0), ((0.25 * 0.2) * 0.2), 0.5);
            s.store_sub_ad(231, A::add_scaled_product(A::div_scalar_offset_denominator(p.p1747, A::limited_exp_scaled_input(A::offset(s.ad_value(117), (-p.p1749)), p.p1748), 1.0, 1.0), 1.0, A::add(s.ad_value(858), A::div_from_scalar(p.p1720, s.ad_value(153))), s.ad_value(230), 1.0), A::div_scalar_offset_denominator(p.p1747, A::limited_exp_scaled_input(A::offset(s.ad_value(172), (-p.p1749)), p.p1748), 1.0, 1.0));
        }

        s.b[1364] = (s.v[332] < 1000.0);
        s.store_scalar(1364, if s.b[1364] { 1.0 } else { 0.0 });

        if s.b[1364] {
            s.store_scalar(332, 1000.0);
        }

        s.b[1365] = (s.v[334] < 1000.0);
        s.store_scalar(1365, if s.b[1365] { 1.0 } else { 0.0 });

        if s.b[1365] {
            s.store_scalar(334, 1000.0);
        }

        s.b[1366] = (s.v[336] < 1000.0);
        s.store_scalar(1366, if s.b[1366] { 1.0 } else { 0.0 });

        if s.b[1366] {
            s.store_scalar(336, 1000.0);
        }

        s.b[1367] = (p.p61 != 0.0);
        s.store_scalar(1367, if s.b[1367] { 1.0 } else { 0.0 });

        s.b[1368] = (p.p75 == 0.0);
        s.store_scalar(1368, if s.b[1368] { 1.0 } else { 0.0 });

        s.b[1369] = (p.p75 != 0.0);
        s.store_scalar(1369, if s.b[1369] { 1.0 } else { 0.0 });

        if ((s.b[1367] && s.b[1368]) && s.b[1369]) {
            s.store_add_scaled_inputs4_offset_mixed_iaai(314, 809, 1.0, A::add_scaled_product(s.ad_value(809), 1.0, s.ad_value(828), s.ad_value(232), 1.0), 0.5, A::sqrt(A::sub_scaled_inputs(A::square(A::offset(A::add_scaled_product(s.ad_value(809), 1.0, s.ad_value(828), s.ad_value(232), 1.0), (-1e-6))), 1.0, s.ad_value(809), (-(4.0 * 1e-6)))), 0.5, 809, (-1.0), (0.5 * (-1e-6)));
        }

        if ((s.b[1367] && s.b[1368]) && (!s.b[1369])) {
            s.store_mul_ad_rhs(314, 809, {
                if (!(((1.0 + (s.v[828] * s.v[232])) - 1e-6) < ((-10000.0) * 0.001))) {
                    A::add_scaled_inputs(A::offset(A::mul(s.ad_value(828), s.ad_value(232)), ((1.0) + ((-1e-6)))), 0.5, A::sqrt_square_offset(A::offset(A::mul(s.ad_value(828), s.ad_value(232)), ((1.0) + ((-1e-6)))), ((4.0 * 0.001) * 0.001)), 0.5)
                } else {
                    {
                        if (((1.0 + (s.v[828] * s.v[232])) - 1e-6) < ((-10000.0) * 0.001)) {
                            A::div_scalar_offset_denominator(((-0.001) * 0.001), A::mul(s.ad_value(828), s.ad_value(232)), ((1.0) + ((-1e-6))), 1.0)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            });
        }

        s.b[1370] = (p.p67 == 1.0);
        s.store_scalar(1370, if s.b[1370] { 1.0 } else { 0.0 });

        s.b[1371] = (p.p75 != 0.0);
        s.store_scalar(1371, if s.b[1371] { 1.0 } else { 0.0 });

        if (((s.b[1367] && s.b[1368]) && s.b[1370]) && s.b[1371]) {
            s.store_add_scaled_inputs4_offset_mixed_iaai(315, 810, 1.0, A::add_scaled_product(s.ad_value(810), 1.0, s.ad_value(829), s.ad_value(232), 1.0), 0.5, A::sqrt(A::sub_scaled_inputs(A::square(A::offset(A::add_scaled_product(s.ad_value(810), 1.0, s.ad_value(829), s.ad_value(232), 1.0), (-1e-6))), 1.0, s.ad_value(810), (-(4.0 * 1e-6)))), 0.5, 810, (-1.0), (0.5 * (-1e-6)));
        }

        if (((s.b[1367] && s.b[1368]) && s.b[1370]) && (!s.b[1371])) {
            s.store_mul_ad_rhs(315, 810, {
                if (!(((1.0 + (s.v[829] * s.v[232])) - 1e-6) < ((-10000.0) * 0.001))) {
                    A::add_scaled_inputs(A::offset(A::mul(s.ad_value(829), s.ad_value(232)), ((1.0) + ((-1e-6)))), 0.5, A::sqrt_square_offset(A::offset(A::mul(s.ad_value(829), s.ad_value(232)), ((1.0) + ((-1e-6)))), ((4.0 * 0.001) * 0.001)), 0.5)
                } else {
                    {
                        if (((1.0 + (s.v[829] * s.v[232])) - 1e-6) < ((-10000.0) * 0.001)) {
                            A::div_scalar_offset_denominator(((-0.001) * 0.001), A::mul(s.ad_value(829), s.ad_value(232)), ((1.0) + ((-1e-6))), 1.0)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            });
        }

        s.b[1372] = (p.p66 != 0.0);
        s.store_scalar(1372, if s.b[1372] { 1.0 } else { 0.0 });

        s.b[1373] = (p.p75 != 0.0);
        s.store_scalar(1373, if s.b[1373] { 1.0 } else { 0.0 });

        if (((s.b[1367] && s.b[1368]) && s.b[1372]) && s.b[1373]) {
            s.store_add_scaled_inputs4_offset_mixed_iaai(316, 817, 1.0, A::add_scaled_product(s.ad_value(817), 1.0, s.ad_value(843), s.ad_value(232), 1.0), 0.5, A::sqrt(A::sub_scaled_inputs(A::square(A::offset(A::add_scaled_product(s.ad_value(817), 1.0, s.ad_value(843), s.ad_value(232), 1.0), (-1e-6))), 1.0, s.ad_value(817), (-(4.0 * 1e-6)))), 0.5, 817, (-1.0), (0.5 * (-1e-6)));
        }

        if (((s.b[1367] && s.b[1368]) && s.b[1372]) && (!s.b[1373])) {
            s.store_mul_ad_rhs(316, 817, {
                if (!(((1.0 + (s.v[843] * s.v[232])) - 1e-6) < ((-10000.0) * 0.001))) {
                    A::add_scaled_inputs(A::offset(A::mul(s.ad_value(843), s.ad_value(232)), ((1.0) + ((-1e-6)))), 0.5, A::sqrt_square_offset(A::offset(A::mul(s.ad_value(843), s.ad_value(232)), ((1.0) + ((-1e-6)))), ((4.0 * 0.001) * 0.001)), 0.5)
                } else {
                    {
                        if (((1.0 + (s.v[843] * s.v[232])) - 1e-6) < ((-10000.0) * 0.001)) {
                            A::div_scalar_offset_denominator(((-0.001) * 0.001), A::mul(s.ad_value(843), s.ad_value(232)), ((1.0) + ((-1e-6))), 1.0)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            });
        }

        if (s.b[1367] && (!s.b[1368])) {
            s.store_add_scaled_product_indices(314, 809, 1.0, 828, 232, 1.0);
        }

        s.b[1374] = (p.p67 == 1.0);
        s.store_scalar(1374, if s.b[1374] { 1.0 } else { 0.0 });

        if ((s.b[1367] && (!s.b[1368])) && s.b[1374]) {
            s.store_add_scaled_product_indices(315, 810, 1.0, 829, 232, 1.0);
        }

        s.b[1375] = (p.p66 != 0.0);
        s.store_scalar(1375, if s.b[1375] { 1.0 } else { 0.0 });

        if ((s.b[1367] && (!s.b[1368])) && s.b[1375]) {
            s.store_add_scaled_product_indices(316, 817, 1.0, 843, 232, 1.0);
        }

        s.b[1376] = (p.p75 != 0.0);
        s.store_scalar(1376, if s.b[1376] { 1.0 } else { 0.0 });

        if s.b[1376] {
            s.store_add_scaled_inputs3_mixed_iai(296, 673, 1.0, A::add_scaled_inputs3_offset(s.ad_value(232), (p.p164 * 0.5), s.ad_value(673), 0.5, A::sqrt(A::sub_scaled_inputs(A::square(A::offset(A::sub_scaled_inputs(s.ad_value(232), p.p164, s.ad_value(673), -1.0), (-1e-6))), 1.0, s.ad_value(673), (-(4.0 * 1e-6)))), 0.5, ((-1e-6) * 0.5)), 1.0, 673, (-1.0));
        }

        if (!s.b[1376]) {
            s.store_mul_ad_rhs(296, 673, {
                if (!(((1.0 + (p.p164 * s.v[232])) - 1e-6) < ((-10000.0) * 0.001))) {
                    A::add_scaled_inputs(A::scale_offset(s.ad_value(232), p.p164, ((1.0) + ((-1e-6)))), 0.5, A::sqrt_square_offset(A::scale_offset(s.ad_value(232), p.p164, ((1.0) + ((-1e-6)))), ((4.0 * 0.001) * 0.001)), 0.5)
                } else {
                    {
                        if (((1.0 + (p.p164 * s.v[232])) - 1e-6) < ((-10000.0) * 0.001)) {
                            A::div_from_scalar(((-0.001) * 0.001), A::scale_offset(s.ad_value(232), p.p164, ((1.0) + ((-1e-6)))))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            });
        }

        s.b[1377] = (p.p67 == 1.0);
        s.store_scalar(1377, if s.b[1377] { 1.0 } else { 0.0 });

        s.b[1378] = (p.p75 != 0.0);
        s.store_scalar(1378, if s.b[1378] { 1.0 } else { 0.0 });

        if (s.b[1377] && s.b[1378]) {
            s.store_add_scaled_inputs3_mixed_iai(297, 675, 1.0, A::add_scaled_inputs3_offset(s.ad_value(232), (p.p165 * 0.5), s.ad_value(675), 0.5, A::sqrt(A::sub_scaled_inputs(A::square(A::offset(A::sub_scaled_inputs(s.ad_value(232), p.p165, s.ad_value(675), -1.0), (-1e-6))), 1.0, s.ad_value(675), (-(4.0 * 1e-6)))), 0.5, ((-1e-6) * 0.5)), 1.0, 675, (-1.0));
        }

        if (s.b[1377] && (!s.b[1378])) {
            s.store_mul_ad_rhs(297, 675, {
                if (!(((1.0 + (p.p165 * s.v[232])) - 1e-6) < ((-10000.0) * 0.001))) {
                    A::add_scaled_inputs(A::scale_offset(s.ad_value(232), p.p165, ((1.0) + ((-1e-6)))), 0.5, A::sqrt_square_offset(A::scale_offset(s.ad_value(232), p.p165, ((1.0) + ((-1e-6)))), ((4.0 * 0.001) * 0.001)), 0.5)
                } else {
                    {
                        if (((1.0 + (p.p165 * s.v[232])) - 1e-6) < ((-10000.0) * 0.001)) {
                            A::div_from_scalar(((-0.001) * 0.001), A::scale_offset(s.ad_value(232), p.p165, ((1.0) + ((-1e-6)))))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            });
        }

        s.b[1379] = (p.p75 != 0.0);
        s.store_scalar(1379, if s.b[1379] { 1.0 } else { 0.0 });

        if s.b[1379] {
            s.store_add_scaled_inputs3_mixed_iai(298, 677, 1.0, A::add_scaled_inputs3_offset(s.ad_value(232), (p.p166 * 0.5), s.ad_value(677), 0.5, A::sqrt(A::sub_scaled_inputs(A::square(A::offset(A::sub_scaled_inputs(s.ad_value(232), p.p166, s.ad_value(677), -1.0), (-1e-6))), 1.0, s.ad_value(677), (-(4.0 * 1e-6)))), 0.5, ((-1e-6) * 0.5)), 1.0, 677, (-1.0));
        }

        if (!s.b[1379]) {
            s.store_mul_ad_rhs(298, 677, {
                if (!(((1.0 + (p.p166 * s.v[232])) - 1e-6) < ((-10000.0) * 0.001))) {
                    A::add_scaled_inputs(A::scale_offset(s.ad_value(232), p.p166, ((1.0) + ((-1e-6)))), 0.5, A::sqrt_square_offset(A::scale_offset(s.ad_value(232), p.p166, ((1.0) + ((-1e-6)))), ((4.0 * 0.001) * 0.001)), 0.5)
                } else {
                    {
                        if (((1.0 + (p.p166 * s.v[232])) - 1e-6) < ((-10000.0) * 0.001)) {
                            A::div_from_scalar(((-0.001) * 0.001), A::scale_offset(s.ad_value(232), p.p166, ((1.0) + ((-1e-6)))))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            });
        }

        s.b[1380] = (p.p75 != 0.0);
        s.store_scalar(1380, if s.b[1380] { 1.0 } else { 0.0 });

        if s.b[1380] {
            s.store_add_scaled_inputs4_offset_mixed_iaai(322, 707, 1.0, A::add_scaled_product(s.ad_value(707), 1.0, s.ad_value(842), s.ad_value(232), 1.0), 0.5, A::sqrt(A::sub_scaled_inputs(A::square(A::offset(A::add_scaled_product(s.ad_value(707), 1.0, s.ad_value(842), s.ad_value(232), 1.0), (-1e-6))), 1.0, s.ad_value(707), (-(4.0 * 1e-6)))), 0.5, 707, (-1.0), (0.5 * (-1e-6)));
        }

        if (!s.b[1380]) {
            s.store_mul_ad_rhs(322, 707, {
                if (!(((1.0 + (s.v[842] * s.v[232])) - 1e-6) < ((-10000.0) * 0.001))) {
                    A::add_scaled_inputs(A::offset(A::mul(s.ad_value(842), s.ad_value(232)), ((1.0) + ((-1e-6)))), 0.5, A::sqrt_square_offset(A::offset(A::mul(s.ad_value(842), s.ad_value(232)), ((1.0) + ((-1e-6)))), ((4.0 * 0.001) * 0.001)), 0.5)
                } else {
                    {
                        if (((1.0 + (s.v[842] * s.v[232])) - 1e-6) < ((-10000.0) * 0.001)) {
                            A::div_scalar_offset_denominator(((-0.001) * 0.001), A::mul(s.ad_value(842), s.ad_value(232)), ((1.0) + ((-1e-6))), 1.0)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            });
        }

        s.b[1381] = (p.p75 != 0.0);
        s.store_scalar(1381, if s.b[1381] { 1.0 } else { 0.0 });

        if s.b[1381] {
            s.store_offset_add_scaled_inputs(299, A::scale_offset(s.ad_value(232), p.p923, (((-(-p.p917))) + ((-1e-6)))), 0.5, A::sqrt_square_offset(A::scale_offset(s.ad_value(232), p.p923, (((-(-p.p917))) + ((-1e-6)))), (-((4.0 * (-p.p917)) * 1e-6))), 0.5, (((-p.p917)) + (p.p917)));
        }

        if (!s.b[1381]) {
            s.store_scale_ad(299, {
                if (!(((1.0 + (p.p923 * s.v[232])) - 1e-6) < ((-10000.0) * 0.001))) {
                    A::add_scaled_inputs(A::scale_offset(s.ad_value(232), p.p923, ((1.0) + ((-1e-6)))), 0.5, A::sqrt_square_offset(A::scale_offset(s.ad_value(232), p.p923, ((1.0) + ((-1e-6)))), ((4.0 * 0.001) * 0.001)), 0.5)
                } else {
                    {
                        if (((1.0 + (p.p923 * s.v[232])) - 1e-6) < ((-10000.0) * 0.001)) {
                            A::div_from_scalar(((-0.001) * 0.001), A::scale_offset(s.ad_value(232), p.p923, ((1.0) + ((-1e-6)))))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, p.p917);
        }

        s.b[1382] = (p.p66 != 0.0);
        s.store_scalar(1382, if s.b[1382] { 1.0 } else { 0.0 });

        s.b[1383] = (p.p75 != 0.0);
        s.store_scalar(1383, if s.b[1383] { 1.0 } else { 0.0 });

        if (s.b[1382] && s.b[1383]) {
            s.store_offset_add_scaled_inputs(300, A::scale_offset(s.ad_value(232), p.p923, (((-(-p.p918))) + ((-1e-6)))), 0.5, A::sqrt_square_offset(A::scale_offset(s.ad_value(232), p.p923, (((-(-p.p918))) + ((-1e-6)))), (-((4.0 * (-p.p918)) * 1e-6))), 0.5, (((-p.p918)) + (p.p918)));
        }

        if (s.b[1382] && (!s.b[1383])) {
            s.store_scale_ad(300, {
                if (!(((1.0 + (p.p923 * s.v[232])) - 1e-6) < ((-10000.0) * 0.001))) {
                    A::add_scaled_inputs(A::scale_offset(s.ad_value(232), p.p923, ((1.0) + ((-1e-6)))), 0.5, A::sqrt_square_offset(A::scale_offset(s.ad_value(232), p.p923, ((1.0) + ((-1e-6)))), ((4.0 * 0.001) * 0.001)), 0.5)
                } else {
                    {
                        if (((1.0 + (p.p923 * s.v[232])) - 1e-6) < ((-10000.0) * 0.001)) {
                            A::div_from_scalar(((-0.001) * 0.001), A::scale_offset(s.ad_value(232), p.p923, ((1.0) + ((-1e-6)))))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, p.p918);
        }

        s.b[1384] = (p.p75 != 0.0);
        s.store_scalar(1384, if s.b[1384] { 1.0 } else { 0.0 });

        if s.b[1384] {
            s.store_offset_add_scaled_inputs(301, A::scale_offset(s.ad_value(232), p.p924, (((-(-p.p919))) + ((-1e-6)))), 0.5, A::sqrt_square_offset(A::scale_offset(s.ad_value(232), p.p924, (((-(-p.p919))) + ((-1e-6)))), (-((4.0 * (-p.p919)) * 1e-6))), 0.5, (((-p.p919)) + (p.p919)));
        }

    }

    pub(super) fn stamp_reactive_block_13(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (!s.b[1384]) {
            s.store_scale_ad(301, {
                if (!(((1.0 + (p.p924 * s.v[232])) - 1e-6) < ((-10000.0) * 0.001))) {
                    A::add_scaled_inputs(A::scale_offset(s.ad_value(232), p.p924, ((1.0) + ((-1e-6)))), 0.5, A::sqrt_square_offset(A::scale_offset(s.ad_value(232), p.p924, ((1.0) + ((-1e-6)))), ((4.0 * 0.001) * 0.001)), 0.5)
                } else {
                    {
                        if (((1.0 + (p.p924 * s.v[232])) - 1e-6) < ((-10000.0) * 0.001)) {
                            A::div_from_scalar(((-0.001) * 0.001), A::scale_offset(s.ad_value(232), p.p924, ((1.0) + ((-1e-6)))))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, p.p919);
        }

        s.b[1385] = (p.p66 != 0.0);
        s.store_scalar(1385, if s.b[1385] { 1.0 } else { 0.0 });

        s.b[1386] = (p.p75 != 0.0);
        s.store_scalar(1386, if s.b[1386] { 1.0 } else { 0.0 });

        if (s.b[1385] && s.b[1386]) {
            s.store_offset_add_scaled_inputs(302, A::scale_offset(s.ad_value(232), p.p924, (((-(-p.p920))) + ((-1e-6)))), 0.5, A::sqrt_square_offset(A::scale_offset(s.ad_value(232), p.p924, (((-(-p.p920))) + ((-1e-6)))), (-((4.0 * (-p.p920)) * 1e-6))), 0.5, (((-p.p920)) + (p.p920)));
        }

        if (s.b[1385] && (!s.b[1386])) {
            s.store_scale_ad(302, {
                if (!(((1.0 + (p.p924 * s.v[232])) - 1e-6) < ((-10000.0) * 0.001))) {
                    A::add_scaled_inputs(A::scale_offset(s.ad_value(232), p.p924, ((1.0) + ((-1e-6)))), 0.5, A::sqrt_square_offset(A::scale_offset(s.ad_value(232), p.p924, ((1.0) + ((-1e-6)))), ((4.0 * 0.001) * 0.001)), 0.5)
                } else {
                    {
                        if (((1.0 + (p.p924 * s.v[232])) - 1e-6) < ((-10000.0) * 0.001)) {
                            A::div_from_scalar(((-0.001) * 0.001), A::scale_offset(s.ad_value(232), p.p924, ((1.0) + ((-1e-6)))))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, p.p920);
        }

        s.b[1387] = (p.p75 != 0.0);
        s.store_scalar(1387, if s.b[1387] { 1.0 } else { 0.0 });

        if s.b[1387] {
            s.store_add_scaled_inputs4_offset_mixed_iaai(257, 700, 1.0, A::add_scaled_product(s.ad_value(700), 1.0, s.ad_value(848), s.ad_value(232), -1.0), 0.5, A::sqrt(A::sub_scaled_inputs(A::square(A::offset(A::add_scaled_product(s.ad_value(700), 1.0, s.ad_value(848), s.ad_value(232), -1.0), (-1e-6))), 1.0, s.ad_value(700), (-(4.0 * 1e-6)))), 0.5, 700, (-1.0), (0.5 * (-1e-6)));
        }

        if (!s.b[1387]) {
            s.store_mul_ad_rhs(257, 700, {
                if (!(((1.0 + ((-s.v[848]) * s.v[232])) - 1e-6) < ((-10000.0) * 0.001))) {
                    A::add_scaled_inputs(A::offset(A::mul_scaled_lhs(s.ad_value(848), -1.0, s.ad_value(232)), ((1.0) + ((-1e-6)))), 0.5, A::sqrt_square_offset(A::offset(A::mul_scaled_lhs(s.ad_value(848), -1.0, s.ad_value(232)), ((1.0) + ((-1e-6)))), ((4.0 * 0.001) * 0.001)), 0.5)
                } else {
                    {
                        if (((1.0 + ((-s.v[848]) * s.v[232])) - 1e-6) < ((-10000.0) * 0.001)) {
                            A::div_scalar_offset_denominator(((-0.001) * 0.001), A::mul_scaled_lhs(s.ad_value(848), -1.0, s.ad_value(232)), ((1.0) + ((-1e-6))), 1.0)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            });
        }

        s.b[1388] = (p.p66 != 0.0);
        s.store_scalar(1388, if s.b[1388] { 1.0 } else { 0.0 });

        s.b[1389] = (p.p75 != 0.0);
        s.store_scalar(1389, if s.b[1389] { 1.0 } else { 0.0 });

        if (s.b[1388] && s.b[1389]) {
            s.store_add_scaled_inputs4_offset_mixed_iaai(258, 701, 1.0, A::add_scaled_product(s.ad_value(701), 1.0, s.ad_value(848), s.ad_value(232), -1.0), 0.5, A::sqrt(A::sub_scaled_inputs(A::square(A::offset(A::add_scaled_product(s.ad_value(701), 1.0, s.ad_value(848), s.ad_value(232), -1.0), (-1e-6))), 1.0, s.ad_value(701), (-(4.0 * 1e-6)))), 0.5, 701, (-1.0), (0.5 * (-1e-6)));
        }

        if (s.b[1388] && (!s.b[1389])) {
            s.store_mul_ad_rhs(258, 701, {
                if (!(((1.0 + ((-s.v[848]) * s.v[232])) - 1e-6) < ((-10000.0) * 0.001))) {
                    A::add_scaled_inputs(A::offset(A::mul_scaled_lhs(s.ad_value(848), -1.0, s.ad_value(232)), ((1.0) + ((-1e-6)))), 0.5, A::sqrt_square_offset(A::offset(A::mul_scaled_lhs(s.ad_value(848), -1.0, s.ad_value(232)), ((1.0) + ((-1e-6)))), ((4.0 * 0.001) * 0.001)), 0.5)
                } else {
                    {
                        if (((1.0 + ((-s.v[848]) * s.v[232])) - 1e-6) < ((-10000.0) * 0.001)) {
                            A::div_scalar_offset_denominator(((-0.001) * 0.001), A::mul_scaled_lhs(s.ad_value(848), -1.0, s.ad_value(232)), ((1.0) + ((-1e-6))), 1.0)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            });
        }

        s.store_mul_exp_ad_rhs(248, 779, A::mul(s.ad_value(860), s.ad_value(418)));

        s.store_mul_offset_ad_rhs(249, 785, {
            if (!(((1.0 + (s.v[789] * s.v[230])) - 0.01) < ((-10000.0) * 0.001))) {
                A::add_scaled_inputs(A::offset(A::mul(s.ad_value(789), s.ad_value(230)), ((1.0) + ((-0.01)))), 0.5, A::sqrt_square_offset(A::offset(A::mul(s.ad_value(789), s.ad_value(230)), ((1.0) + ((-0.01)))), ((4.0 * 0.001) * 0.001)), 0.5)
            } else {
                {
                    if (((1.0 + (s.v[789] * s.v[230])) - 0.01) < ((-10000.0) * 0.001)) {
                        A::div_scalar_offset_denominator(((-0.001) * 0.001), A::mul(s.ad_value(789), s.ad_value(230)), ((1.0) + ((-0.01))), 1.0)
                    } else {
                        A::constant(0.0)
                    }
                }
            }
        }, 0.01);

        s.store_add_scaled_product_indices(236, 683, 1.0, 684, 232, 1.0);

        s.store_add_scaled_inputs4_offset_mixed_iaai(237, 685, 1.0, A::add_scaled_product(s.ad_value(685), 1.0, s.ad_value(686), s.ad_value(232), 1.0), 0.5, A::sqrt(A::sub_scaled_inputs(A::square(A::offset(A::add_scaled_product(s.ad_value(685), 1.0, s.ad_value(686), s.ad_value(232), 1.0), (-1e-6))), 1.0, s.ad_value(685), (-(4.0 * 1e-6)))), 0.5, 685, (-1.0), (0.5 * (-1e-6)));

        s.store_add_scaled_inputs4_offset_mixed_iaai(238, 687, 1.0, A::add_scaled_product(s.ad_value(687), 1.0, s.ad_value(688), s.ad_value(232), 1.0), 0.5, A::sqrt(A::sub_scaled_inputs(A::square(A::offset(A::add_scaled_product(s.ad_value(687), 1.0, s.ad_value(688), s.ad_value(232), 1.0), (-1e-6))), 1.0, s.ad_value(687), (-(4.0 * 1e-6)))), 0.5, 687, (-1.0), (0.5 * (-1e-6)));

        s.store_add_scaled_inputs4_offset_mixed_iaai(239, 690, 1.0, A::add_scaled_product(s.ad_value(690), 1.0, s.ad_value(691), s.ad_value(232), 1.0), 0.5, A::sqrt(A::sub_scaled_inputs(A::square(A::offset(A::add_scaled_product(s.ad_value(690), 1.0, s.ad_value(691), s.ad_value(232), 1.0), (-1e-6))), 1.0, s.ad_value(690), (-(4.0 * 1e-6)))), 0.5, 690, (-1.0), (0.5 * (-1e-6)));

        s.store_add_scaled_product_indices(240, 692, 1.0, 693, 232, 1.0);

        s.store_add_scaled_product_indices(241, 798, 1.0, 800, 232, 1.0);

        s.store_add_scaled_product_indices(242, 799, 1.0, 801, 232, 1.0);

        s.store_add_scaled_inputs4_offset_mixed_iaai(293, 871, 1.0, A::add_scaled_product(s.ad_value(871), 1.0, s.ad_value(872), s.ad_value(232), 1.0), 0.5, A::sqrt(A::sub_scaled_inputs(A::square(A::offset(A::add_scaled_product(s.ad_value(871), 1.0, s.ad_value(872), s.ad_value(232), 1.0), (-1e-6))), 1.0, s.ad_value(871), (-(4.0 * 1e-6)))), 0.5, 871, (-1.0), (0.5 * (-1e-6)));

        s.store_add_scaled_product_indices(294, 867, 1.0, 868, 232, 1.0);

        s.store_add_scaled_product_indices(295, 869, 1.0, 870, 232, 1.0);

        s.store_add_scaled_inputs4_offset_mixed_iaai(243, 721, 1.0, A::add_scaled_product(s.ad_value(721), 1.0, s.ad_value(722), s.ad_value(232), 1.0), 0.5, A::sqrt(A::sub_scaled_inputs(A::square(A::offset(A::add_scaled_product(s.ad_value(721), 1.0, s.ad_value(722), s.ad_value(232), 1.0), (-1e-6))), 1.0, s.ad_value(721), (-(4.0 * 1e-6)))), 0.5, 721, (-1.0), (0.5 * (-1e-6)));

        s.store_add_scaled_inputs4_offset_mixed_iaai(244, 727, 1.0, A::add_scaled_product(s.ad_value(727), 1.0, s.ad_value(728), s.ad_value(232), 1.0), 0.5, A::sqrt(A::sub_scaled_inputs(A::square(A::offset(A::add_scaled_product(s.ad_value(727), 1.0, s.ad_value(728), s.ad_value(232), 1.0), (-1e-6))), 1.0, s.ad_value(727), (-(4.0 * 1e-6)))), 0.5, 727, (-1.0), (0.5 * (-1e-6)));

        s.store_add_scaled_inputs4_offset_mixed_iaai(245, 732, 1.0, A::add_scaled_product(s.ad_value(732), 1.0, s.ad_value(733), s.ad_value(232), 1.0), 0.5, A::sqrt(A::sub_scaled_inputs(A::square(A::offset(A::add_scaled_product(s.ad_value(732), 1.0, s.ad_value(733), s.ad_value(232), 1.0), (-1e-6))), 1.0, s.ad_value(732), (-(4.0 * 1e-6)))), 0.5, 732, (-1.0), (0.5 * (-1e-6)));

        s.store_add_scaled_inputs4_offset_mixed_iaai(246, 737, 1.0, A::add_scaled_product(s.ad_value(737), 1.0, s.ad_value(738), s.ad_value(232), 1.0), 0.5, A::sqrt(A::sub_scaled_inputs(A::square(A::offset(A::add_scaled_product(s.ad_value(737), 1.0, s.ad_value(738), s.ad_value(232), 1.0), (-1e-6))), 1.0, s.ad_value(737), (-(4.0 * 1e-6)))), 0.5, 737, (-1.0), (0.5 * (-1e-6)));

        s.store_add_scaled_inputs4_offset_mixed_iaai(247, 743, 1.0, A::add_scaled_product(s.ad_value(743), 1.0, s.ad_value(744), s.ad_value(232), 1.0), 0.5, A::sqrt(A::sub_scaled_inputs(A::square(A::offset(A::add_scaled_product(s.ad_value(743), 1.0, s.ad_value(744), s.ad_value(232), 1.0), (-1e-6))), 1.0, s.ad_value(743), (-(4.0 * 1e-6)))), 0.5, 743, (-1.0), (0.5 * (-1e-6)));

        s.store_mul_ad_rhs(252, 748, {
            if (!(((1.0 + (s.v[862] * s.v[232])) - 1e-6) < ((-10000.0) * 0.001))) {
                A::add_scaled_inputs(A::offset(A::mul(s.ad_value(862), s.ad_value(232)), ((1.0) + ((-1e-6)))), 0.5, A::sqrt_square_offset(A::offset(A::mul(s.ad_value(862), s.ad_value(232)), ((1.0) + ((-1e-6)))), ((4.0 * 0.001) * 0.001)), 0.5)
            } else {
                {
                    if (((1.0 + (s.v[862] * s.v[232])) - 1e-6) < ((-10000.0) * 0.001)) {
                        A::div_scalar_offset_denominator(((-0.001) * 0.001), A::mul(s.ad_value(862), s.ad_value(232)), ((1.0) + ((-1e-6))), 1.0)
                    } else {
                        A::constant(0.0)
                    }
                }
            }
        });

        s.store_mul_ad_rhs(250, 762, {
            if (!(((1.0 + (s.v[862] * s.v[232])) - 1e-6) < ((-10000.0) * 0.001))) {
                A::add_scaled_inputs(A::offset(A::mul(s.ad_value(862), s.ad_value(232)), ((1.0) + ((-1e-6)))), 0.5, A::sqrt_square_offset(A::offset(A::mul(s.ad_value(862), s.ad_value(232)), ((1.0) + ((-1e-6)))), ((4.0 * 0.001) * 0.001)), 0.5)
            } else {
                {
                    if (((1.0 + (s.v[862] * s.v[232])) - 1e-6) < ((-10000.0) * 0.001)) {
                        A::div_scalar_offset_denominator(((-0.001) * 0.001), A::mul(s.ad_value(862), s.ad_value(232)), ((1.0) + ((-1e-6))), 1.0)
                    } else {
                        A::constant(0.0)
                    }
                }
            }
        });

        s.store_add_scaled_inputs3_mixed_iai(259, 775, 1.0, A::add_scaled_inputs3_offset(s.ad_value(232), (p.p1437 * 0.5), s.ad_value(775), 0.5, A::sqrt(A::sub_scaled_inputs(A::square(A::offset(A::sub_scaled_inputs(s.ad_value(232), p.p1437, s.ad_value(775), -1.0), (-1e-6))), 1.0, s.ad_value(775), (-(4.0 * 1e-6)))), 0.5, ((-1e-6) * 0.5)), 1.0, 775, (-1.0));

        s.store_add_scaled_inputs3_mixed_iai(260, 776, 1.0, A::add_scaled_inputs3_offset(s.ad_value(232), (p.p1438 * 0.5), s.ad_value(776), 0.5, A::sqrt(A::sub_scaled_inputs(A::square(A::offset(A::sub_scaled_inputs(s.ad_value(232), p.p1438, s.ad_value(776), -1.0), (-1e-6))), 1.0, s.ad_value(776), (-(4.0 * 1e-6)))), 0.5, ((-1e-6) * 0.5)), 1.0, 776, (-1.0));

        s.store_add_scaled_inputs3_mixed_iai(261, 777, 1.0, A::add_scaled_inputs3_offset(s.ad_value(232), (p.p1439 * 0.5), s.ad_value(777), 0.5, A::sqrt(A::sub_scaled_inputs(A::square(A::offset(A::sub_scaled_inputs(s.ad_value(232), p.p1439, s.ad_value(777), -1.0), (-1e-25))), 1.0, s.ad_value(777), (-(4.0 * 1e-25)))), 0.5, ((-1e-25) * 0.5)), 1.0, 777, (-1.0));

        s.store_add_scaled_inputs3_mixed_iai(262, 778, 1.0, A::add_scaled_inputs3_offset(s.ad_value(232), (p.p1440 * 0.5), s.ad_value(778), 0.5, A::sqrt(A::sub_scaled_inputs(A::square(A::offset(A::sub_scaled_inputs(s.ad_value(232), p.p1440, s.ad_value(778), -1.0), (-1e-20))), 1.0, s.ad_value(778), (-(4.0 * 1e-20)))), 0.5, ((-1e-20) * 0.5)), 1.0, 778, (-1.0));

        s.b[1390] = (p.p61 != 0.0);
        s.store_scalar(1390, if s.b[1390] { 1.0 } else { 0.0 });

        s.b[1391] = (p.p75 != 0.0);
        s.store_scalar(1391, if s.b[1391] { 1.0 } else { 0.0 });

        if (s.b[1390] && s.b[1391]) {
            s.store_offset_add_scaled_inputs(263, A::scale_offset(s.ad_value(232), p.p1721, (((-(-p.p1584))) + ((-1e-6)))), 0.5, A::sqrt_square_offset(A::scale_offset(s.ad_value(232), p.p1721, (((-(-p.p1584))) + ((-1e-6)))), (-((4.0 * (-p.p1584)) * 1e-6))), 0.5, (((-p.p1584)) + (p.p1584)));
        }

        if (s.b[1390] && (!s.b[1391])) {
            s.store_scale_ad(263, {
                if (!(((1.0 + (p.p1721 * s.v[232])) - 1e-6) < ((-10000.0) * 0.001))) {
                    A::add_scaled_inputs(A::scale_offset(s.ad_value(232), p.p1721, ((1.0) + ((-1e-6)))), 0.5, A::sqrt_square_offset(A::scale_offset(s.ad_value(232), p.p1721, ((1.0) + ((-1e-6)))), ((4.0 * 0.001) * 0.001)), 0.5)
                } else {
                    {
                        if (((1.0 + (p.p1721 * s.v[232])) - 1e-6) < ((-10000.0) * 0.001)) {
                            A::div_from_scalar(((-0.001) * 0.001), A::scale_offset(s.ad_value(232), p.p1721, ((1.0) + ((-1e-6)))))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, p.p1584);
        }

        s.b[1392] = (p.p75 != 0.0);
        s.store_scalar(1392, if s.b[1392] { 1.0 } else { 0.0 });

        if (s.b[1390] && s.b[1392]) {
            s.store_offset_add_scaled_inputs(266, A::scale_offset(s.ad_value(232), p.p1721, (((-(-p.p1585))) + ((-1e-6)))), 0.5, A::sqrt_square_offset(A::scale_offset(s.ad_value(232), p.p1721, (((-(-p.p1585))) + ((-1e-6)))), (-((4.0 * (-p.p1585)) * 1e-6))), 0.5, (((-p.p1585)) + (p.p1585)));
        }

        if (s.b[1390] && (!s.b[1392])) {
            s.store_scale_ad(266, {
                if (!(((1.0 + (p.p1721 * s.v[232])) - 1e-6) < ((-10000.0) * 0.001))) {
                    A::add_scaled_inputs(A::scale_offset(s.ad_value(232), p.p1721, ((1.0) + ((-1e-6)))), 0.5, A::sqrt_square_offset(A::scale_offset(s.ad_value(232), p.p1721, ((1.0) + ((-1e-6)))), ((4.0 * 0.001) * 0.001)), 0.5)
                } else {
                    {
                        if (((1.0 + (p.p1721 * s.v[232])) - 1e-6) < ((-10000.0) * 0.001)) {
                            A::div_from_scalar(((-0.001) * 0.001), A::scale_offset(s.ad_value(232), p.p1721, ((1.0) + ((-1e-6)))))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, p.p1585);
        }

        s.b[1393] = (p.p75 != 0.0);
        s.store_scalar(1393, if s.b[1393] { 1.0 } else { 0.0 });

        if (s.b[1390] && s.b[1393]) {
            s.store_offset_add_scaled_inputs(264, A::scale_offset(s.ad_value(232), p.p1722, (((-(-p.p1586))) + ((-1e-6)))), 0.5, A::sqrt_square_offset(A::scale_offset(s.ad_value(232), p.p1722, (((-(-p.p1586))) + ((-1e-6)))), (-((4.0 * (-p.p1586)) * 1e-6))), 0.5, (((-p.p1586)) + (p.p1586)));
        }

        if (s.b[1390] && (!s.b[1393])) {
            s.store_scale_ad(264, {
                if (!(((1.0 + (p.p1722 * s.v[232])) - 1e-6) < ((-10000.0) * 0.001))) {
                    A::add_scaled_inputs(A::scale_offset(s.ad_value(232), p.p1722, ((1.0) + ((-1e-6)))), 0.5, A::sqrt_square_offset(A::scale_offset(s.ad_value(232), p.p1722, ((1.0) + ((-1e-6)))), ((4.0 * 0.001) * 0.001)), 0.5)
                } else {
                    {
                        if (((1.0 + (p.p1722 * s.v[232])) - 1e-6) < ((-10000.0) * 0.001)) {
                            A::div_from_scalar(((-0.001) * 0.001), A::scale_offset(s.ad_value(232), p.p1722, ((1.0) + ((-1e-6)))))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, p.p1586);
        }

        s.b[1394] = (p.p75 != 0.0);
        s.store_scalar(1394, if s.b[1394] { 1.0 } else { 0.0 });

        if (s.b[1390] && s.b[1394]) {
            s.store_offset_add_scaled_inputs(267, A::scale_offset(s.ad_value(232), p.p1722, (((-(-p.p1587))) + ((-1e-6)))), 0.5, A::sqrt_square_offset(A::scale_offset(s.ad_value(232), p.p1722, (((-(-p.p1587))) + ((-1e-6)))), (-((4.0 * (-p.p1587)) * 1e-6))), 0.5, (((-p.p1587)) + (p.p1587)));
        }

        if (s.b[1390] && (!s.b[1394])) {
            s.store_scale_ad(267, {
                if (!(((1.0 + (p.p1722 * s.v[232])) - 1e-6) < ((-10000.0) * 0.001))) {
                    A::add_scaled_inputs(A::scale_offset(s.ad_value(232), p.p1722, ((1.0) + ((-1e-6)))), 0.5, A::sqrt_square_offset(A::scale_offset(s.ad_value(232), p.p1722, ((1.0) + ((-1e-6)))), ((4.0 * 0.001) * 0.001)), 0.5)
                } else {
                    {
                        if (((1.0 + (p.p1722 * s.v[232])) - 1e-6) < ((-10000.0) * 0.001)) {
                            A::div_from_scalar(((-0.001) * 0.001), A::scale_offset(s.ad_value(232), p.p1722, ((1.0) + ((-1e-6)))))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, p.p1587);
        }

        s.b[1395] = (p.p75 != 0.0);
        s.store_scalar(1395, if s.b[1395] { 1.0 } else { 0.0 });

        if (s.b[1390] && s.b[1395]) {
            s.store_offset_add_scaled_inputs(268, A::scale_offset(s.ad_value(232), p.p1723, (((-(-p.p1588))) + ((-1e-6)))), 0.5, A::sqrt_square_offset(A::scale_offset(s.ad_value(232), p.p1723, (((-(-p.p1588))) + ((-1e-6)))), (-((4.0 * (-p.p1588)) * 1e-6))), 0.5, (((-p.p1588)) + (p.p1588)));
        }

        if (s.b[1390] && (!s.b[1395])) {
            s.store_scale_ad(268, {
                if (!(((1.0 + (p.p1723 * s.v[232])) - 1e-6) < ((-10000.0) * 0.001))) {
                    A::add_scaled_inputs(A::scale_offset(s.ad_value(232), p.p1723, ((1.0) + ((-1e-6)))), 0.5, A::sqrt_square_offset(A::scale_offset(s.ad_value(232), p.p1723, ((1.0) + ((-1e-6)))), ((4.0 * 0.001) * 0.001)), 0.5)
                } else {
                    {
                        if (((1.0 + (p.p1723 * s.v[232])) - 1e-6) < ((-10000.0) * 0.001)) {
                            A::div_from_scalar(((-0.001) * 0.001), A::scale_offset(s.ad_value(232), p.p1723, ((1.0) + ((-1e-6)))))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, p.p1588);
        }

        s.b[1396] = (p.p75 != 0.0);
        s.store_scalar(1396, if s.b[1396] { 1.0 } else { 0.0 });

        if (s.b[1390] && s.b[1396]) {
            s.store_offset_add_scaled_inputs(265, A::scale_offset(s.ad_value(232), p.p1723, (((-(-p.p1589))) + ((-1e-6)))), 0.5, A::sqrt_square_offset(A::scale_offset(s.ad_value(232), p.p1723, (((-(-p.p1589))) + ((-1e-6)))), (-((4.0 * (-p.p1589)) * 1e-6))), 0.5, (((-p.p1589)) + (p.p1589)));
        }

        if (s.b[1390] && (!s.b[1396])) {
            s.store_scale_ad(265, {
                if (!(((1.0 + (p.p1723 * s.v[232])) - 1e-6) < ((-10000.0) * 0.001))) {
                    A::add_scaled_inputs(A::scale_offset(s.ad_value(232), p.p1723, ((1.0) + ((-1e-6)))), 0.5, A::sqrt_square_offset(A::scale_offset(s.ad_value(232), p.p1723, ((1.0) + ((-1e-6)))), ((4.0 * 0.001) * 0.001)), 0.5)
                } else {
                    {
                        if (((1.0 + (p.p1723 * s.v[232])) - 1e-6) < ((-10000.0) * 0.001)) {
                            A::div_from_scalar(((-0.001) * 0.001), A::scale_offset(s.ad_value(232), p.p1723, ((1.0) + ((-1e-6)))))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, p.p1589);
        }

        if s.b[1390] {
            s.store_offset_ad(269, {
                if (!(((p.p1590 - (p.p1724 * s.v[232])) - 0.01) < ((-10000.0) * 0.001))) {
                    A::add_scaled_inputs(A::offset(A::sub_from_scalar(p.p1590, A::scale(s.ad_value(232), p.p1724)), (-0.01)), 0.5, A::sqrt_square_offset(A::offset(A::sub_from_scalar(p.p1590, A::scale(s.ad_value(232), p.p1724)), (-0.01)), ((4.0 * 0.001) * 0.001)), 0.5)
                } else {
                    {
                        if (((p.p1590 - (p.p1724 * s.v[232])) - 0.01) < ((-10000.0) * 0.001)) {
                            A::div_scalar_offset_denominator(((-0.001) * 0.001), A::sub_from_scalar(p.p1590, A::scale(s.ad_value(232), p.p1724)), (-0.01), 1.0)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 0.01);
        }

        if s.b[1390] {
            s.store_offset_ad(272, {
                if (!(((p.p1591 - (p.p1724 * s.v[232])) - 0.01) < ((-10000.0) * 0.001))) {
                    A::add_scaled_inputs(A::offset(A::sub_from_scalar(p.p1591, A::scale(s.ad_value(232), p.p1724)), (-0.01)), 0.5, A::sqrt_square_offset(A::offset(A::sub_from_scalar(p.p1591, A::scale(s.ad_value(232), p.p1724)), (-0.01)), ((4.0 * 0.001) * 0.001)), 0.5)
                } else {
                    {
                        if (((p.p1591 - (p.p1724 * s.v[232])) - 0.01) < ((-10000.0) * 0.001)) {
                            A::div_scalar_offset_denominator(((-0.001) * 0.001), A::sub_from_scalar(p.p1591, A::scale(s.ad_value(232), p.p1724)), (-0.01), 1.0)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 0.01);
        }

        if s.b[1390] {
            s.store_offset_ad(270, {
                if (!(((p.p1592 - (p.p1725 * s.v[232])) - 0.01) < ((-10000.0) * 0.001))) {
                    A::add_scaled_inputs(A::offset(A::sub_from_scalar(p.p1592, A::scale(s.ad_value(232), p.p1725)), (-0.01)), 0.5, A::sqrt_square_offset(A::offset(A::sub_from_scalar(p.p1592, A::scale(s.ad_value(232), p.p1725)), (-0.01)), ((4.0 * 0.001) * 0.001)), 0.5)
                } else {
                    {
                        if (((p.p1592 - (p.p1725 * s.v[232])) - 0.01) < ((-10000.0) * 0.001)) {
                            A::div_scalar_offset_denominator(((-0.001) * 0.001), A::sub_from_scalar(p.p1592, A::scale(s.ad_value(232), p.p1725)), (-0.01), 1.0)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 0.01);
        }

        if s.b[1390] {
            s.store_offset_ad(273, {
                if (!(((p.p1593 - (p.p1725 * s.v[232])) - 0.01) < ((-10000.0) * 0.001))) {
                    A::add_scaled_inputs(A::offset(A::sub_from_scalar(p.p1593, A::scale(s.ad_value(232), p.p1725)), (-0.01)), 0.5, A::sqrt_square_offset(A::offset(A::sub_from_scalar(p.p1593, A::scale(s.ad_value(232), p.p1725)), (-0.01)), ((4.0 * 0.001) * 0.001)), 0.5)
                } else {
                    {
                        if (((p.p1593 - (p.p1725 * s.v[232])) - 0.01) < ((-10000.0) * 0.001)) {
                            A::div_scalar_offset_denominator(((-0.001) * 0.001), A::sub_from_scalar(p.p1593, A::scale(s.ad_value(232), p.p1725)), (-0.01), 1.0)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 0.01);
        }

        if s.b[1390] {
            s.store_offset_ad(271, {
                if (!(((p.p1594 - (p.p1726 * s.v[232])) - 0.01) < ((-10000.0) * 0.001))) {
                    A::add_scaled_inputs(A::offset(A::sub_from_scalar(p.p1594, A::scale(s.ad_value(232), p.p1726)), (-0.01)), 0.5, A::sqrt_square_offset(A::offset(A::sub_from_scalar(p.p1594, A::scale(s.ad_value(232), p.p1726)), (-0.01)), ((4.0 * 0.001) * 0.001)), 0.5)
                } else {
                    {
                        if (((p.p1594 - (p.p1726 * s.v[232])) - 0.01) < ((-10000.0) * 0.001)) {
                            A::div_scalar_offset_denominator(((-0.001) * 0.001), A::sub_from_scalar(p.p1594, A::scale(s.ad_value(232), p.p1726)), (-0.01), 1.0)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 0.01);
        }

        if s.b[1390] {
            s.store_offset_ad(274, {
                if (!(((p.p1595 - (p.p1726 * s.v[232])) - 0.01) < ((-10000.0) * 0.001))) {
                    A::add_scaled_inputs(A::offset(A::sub_from_scalar(p.p1595, A::scale(s.ad_value(232), p.p1726)), (-0.01)), 0.5, A::sqrt_square_offset(A::offset(A::sub_from_scalar(p.p1595, A::scale(s.ad_value(232), p.p1726)), (-0.01)), ((4.0 * 0.001) * 0.001)), 0.5)
                } else {
                    {
                        if (((p.p1595 - (p.p1726 * s.v[232])) - 0.01) < ((-10000.0) * 0.001)) {
                            A::div_scalar_offset_denominator(((-0.001) * 0.001), A::sub_from_scalar(p.p1595, A::scale(s.ad_value(232), p.p1726)), (-0.01), 1.0)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 0.01);
        }

        if s.b[1390] {
            s.store_sub_ad(168, A::div(s.ad_value(147), s.ad_value(180)), A::div(s.ad_value(146), s.ad_value(179)));
            s.store_limited_exp_scaled_input_ad(171, A::add_scaled_inputs(s.ad_value(168), 1.0, s.ad_value(418), p.p1727), 1.0 / (p.p1620));
            s.store_scale(275, 171, p.p1614);
            s.store_scale(276, 171, p.p1616);
            s.store_scale(277, 171, p.p1618);
            s.store_limited_exp_scaled_input_ad(171, A::add_scaled_inputs(s.ad_value(168), 1.0, s.ad_value(418), p.p1728), 1.0 / (p.p1621));
            s.store_scale(278, 171, p.p1615);
            s.store_scale(279, 171, p.p1617);
            s.store_scale(280, 171, p.p1619);
            s.store_scaled_limited_exp_ad(281, A::div_scaled_product(s.ad_value(147), s.ad_value(230), p.p1729, s.ad_value(179), 1.0), p.p1630);
            s.store_scaled_limited_exp_ad(282, A::div_scaled_product(s.ad_value(147), s.ad_value(230), p.p1730, s.ad_value(179), 1.0), p.p1631);
            s.store_scaled_limited_exp_ad(283, A::div_scaled_product(s.ad_value(147), s.ad_value(230), p.p1731, s.ad_value(179), 1.0), p.p1632);
            s.store_scaled_limited_exp_ad(284, A::div_scaled_product(s.ad_value(147), s.ad_value(230), p.p1732, s.ad_value(179), 1.0), p.p1633);
            s.store_scaled_mul_ad(285, A::offset(A::sqrt(A::div_from_scalar(p.p1636, s.ad_value(158))), 1.0), A::limited_exp(A::div_scaled_product(s.ad_value(147), s.ad_value(230), p.p1733, s.ad_value(179), 1.0)), p.p1634);
            s.store_scaled_mul_ad(286, A::offset(A::sqrt(A::div_from_scalar(p.p1636, s.ad_value(158))), 1.0), A::limited_exp(A::div_scaled_product(s.ad_value(147), s.ad_value(230), p.p1734, s.ad_value(179), 1.0)), p.p1635);
        }

        if s.b[1390] {
            s.store_offset_ad(287, {
                if (!(((p.p1637 * (1.0 + (p.p1735 * s.v[230]))) - 0.01) < ((-10000.0) * 0.001))) {
                    A::add_scaled_inputs(A::scale_offset(s.ad_value(230), ((p.p1735) * (p.p1637)), ((p.p1637) + ((-0.01)))), 0.5, A::sqrt_square_offset(A::scale_offset(s.ad_value(230), ((p.p1735) * (p.p1637)), ((p.p1637) + ((-0.01)))), ((4.0 * 0.001) * 0.001)), 0.5)
                } else {
                    {
                        if (((p.p1637 * (1.0 + (p.p1735 * s.v[230]))) - 0.01) < ((-10000.0) * 0.001)) {
                            A::div_from_scalar(((-0.001) * 0.001), A::scale_offset(s.ad_value(230), ((p.p1735) * (p.p1637)), ((p.p1637) + ((-0.01)))))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 0.01);
        }

        if s.b[1390] {
            s.store_offset_ad(288, {
                if (!(((p.p1638 * (1.0 + (p.p1736 * s.v[230]))) - 0.01) < ((-10000.0) * 0.001))) {
                    A::add_scaled_inputs(A::scale_offset(s.ad_value(230), ((p.p1736) * (p.p1638)), ((p.p1638) + ((-0.01)))), 0.5, A::sqrt_square_offset(A::scale_offset(s.ad_value(230), ((p.p1736) * (p.p1638)), ((p.p1638) + ((-0.01)))), ((4.0 * 0.001) * 0.001)), 0.5)
                } else {
                    {
                        if (((p.p1638 * (1.0 + (p.p1736 * s.v[230]))) - 0.01) < ((-10000.0) * 0.001)) {
                            A::div_from_scalar(((-0.001) * 0.001), A::scale_offset(s.ad_value(230), ((p.p1736) * (p.p1638)), ((p.p1638) + ((-0.01)))))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 0.01);
        }

    }

    pub(super) fn stamp_reactive_block_14(
        s: &mut ReactiveScratch,
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        if s.b[1390] {
            s.store_offset_ad(289, {
                if (!(((p.p1639 * (1.0 + (p.p1737 * s.v[230]))) - 0.01) < ((-10000.0) * 0.001))) {
                    A::add_scaled_inputs(A::scale_offset(s.ad_value(230), ((p.p1737) * (p.p1639)), ((p.p1639) + ((-0.01)))), 0.5, A::sqrt_square_offset(A::scale_offset(s.ad_value(230), ((p.p1737) * (p.p1639)), ((p.p1639) + ((-0.01)))), ((4.0 * 0.001) * 0.001)), 0.5)
                } else {
                    {
                        if (((p.p1639 * (1.0 + (p.p1737 * s.v[230]))) - 0.01) < ((-10000.0) * 0.001)) {
                            A::div_from_scalar(((-0.001) * 0.001), A::scale_offset(s.ad_value(230), ((p.p1737) * (p.p1639)), ((p.p1639) + ((-0.01)))))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 0.01);
        }

        if s.b[1390] {
            s.store_offset_ad(290, {
                if (!(((p.p1640 * (1.0 + (p.p1738 * s.v[230]))) - 0.01) < ((-10000.0) * 0.001))) {
                    A::add_scaled_inputs(A::scale_offset(s.ad_value(230), ((p.p1738) * (p.p1640)), ((p.p1640) + ((-0.01)))), 0.5, A::sqrt_square_offset(A::scale_offset(s.ad_value(230), ((p.p1738) * (p.p1640)), ((p.p1640) + ((-0.01)))), ((4.0 * 0.001) * 0.001)), 0.5)
                } else {
                    {
                        if (((p.p1640 * (1.0 + (p.p1738 * s.v[230]))) - 0.01) < ((-10000.0) * 0.001)) {
                            A::div_from_scalar(((-0.001) * 0.001), A::scale_offset(s.ad_value(230), ((p.p1738) * (p.p1640)), ((p.p1640) + ((-0.01)))))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 0.01);
        }

        if s.b[1390] {
            s.store_offset_ad(291, {
                if (!(((p.p1641 * (1.0 + (p.p1739 * s.v[230]))) - 0.01) < ((-10000.0) * 0.001))) {
                    A::add_scaled_inputs(A::scale_offset(s.ad_value(230), ((p.p1739) * (p.p1641)), ((p.p1641) + ((-0.01)))), 0.5, A::sqrt_square_offset(A::scale_offset(s.ad_value(230), ((p.p1739) * (p.p1641)), ((p.p1641) + ((-0.01)))), ((4.0 * 0.001) * 0.001)), 0.5)
                } else {
                    {
                        if (((p.p1641 * (1.0 + (p.p1739 * s.v[230]))) - 0.01) < ((-10000.0) * 0.001)) {
                            A::div_from_scalar(((-0.001) * 0.001), A::scale_offset(s.ad_value(230), ((p.p1739) * (p.p1641)), ((p.p1641) + ((-0.01)))))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 0.01);
        }

        if s.b[1390] {
            s.store_offset_ad(292, {
                if (!(((p.p1642 * (1.0 + (p.p1740 * s.v[230]))) - 0.01) < ((-10000.0) * 0.001))) {
                    A::add_scaled_inputs(A::scale_offset(s.ad_value(230), ((p.p1740) * (p.p1642)), ((p.p1642) + ((-0.01)))), 0.5, A::sqrt_square_offset(A::scale_offset(s.ad_value(230), ((p.p1740) * (p.p1642)), ((p.p1642) + ((-0.01)))), ((4.0 * 0.001) * 0.001)), 0.5)
                } else {
                    {
                        if (((p.p1642 * (1.0 + (p.p1740 * s.v[230]))) - 0.01) < ((-10000.0) * 0.001)) {
                            A::div_from_scalar(((-0.001) * 0.001), A::scale_offset(s.ad_value(230), ((p.p1740) * (p.p1642)), ((p.p1642) + ((-0.01)))))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 0.01);
        }

        s.b[1397] = (!param_given[1106]);
        s.store_scalar(1397, if s.b[1397] { 1.0 } else { 0.0 });

        s.b[1398] = (p.p145 > 0.0);
        s.store_scalar(1398, if s.b[1398] { 1.0 } else { 0.0 });

        s.b[1399] = (p.p80 == 0.0);
        s.store_scalar(1399, if s.b[1399] { 1.0 } else { 0.0 });

        if ((s.b[1397] && s.b[1398]) && s.b[1399]) {
            let assign18720_ad_e35490: A = {
                if (!(((0.5 * s.v[146]) - (s.v[179] * (if (!((p.p145 / s.v[141]) > 1e-38)) { (-87.498233534) } else { (if ((p.p145 / s.v[141]) > 1e-38) { (((p.p145 / s.v[141])) as f64).ln() } else { 0.0 }) }))) < ((-10000.0) * 0.0001))) {
                    A::add_scaled_inputs(A::add_scaled_product(s.ad_value(146), 0.5, s.ad_value(179), {
                        if (!((p.p145 / s.v[141]) > 1e-38)) {
                            A::neg(A::constant(87.498233534))
                        } else {
                            {
                                if ((p.p145 / s.v[141]) > 1e-38) {
                                    A::ln(A::div_from_scalar(p.p145, s.ad_value(141)))
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }, (-1.0)), 0.5, A::sqrt_square_offset(A::add_scaled_product(s.ad_value(146), 0.5, s.ad_value(179), {
                        if (!((p.p145 / s.v[141]) > 1e-38)) {
                            A::neg(A::constant(87.498233534))
                        } else {
                            {
                                if ((p.p145 / s.v[141]) > 1e-38) {
                                    A::ln(A::div_from_scalar(p.p145, s.ad_value(141)))
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }, (-1.0)), ((4.0 * 0.0001) * 0.0001)), 0.5)
                } else {
                    {
                        if (((0.5 * s.v[146]) - (s.v[179] * (if (!((p.p145 / s.v[141]) > 1e-38)) { (-87.498233534) } else { (if ((p.p145 / s.v[141]) > 1e-38) { (((p.p145 / s.v[141])) as f64).ln() } else { 0.0 }) }))) < ((-10000.0) * 0.0001)) {
                            A::div_from_scalar(((-0.0001) * 0.0001), A::add_scaled_product(s.ad_value(146), 0.5, s.ad_value(179), {
                                if (!((p.p145 / s.v[141]) > 1e-38)) {
                                    A::neg(A::constant(87.498233534))
                                } else {
                                    {
                                        if ((p.p145 / s.v[141]) > 1e-38) {
                                            A::ln(A::div_from_scalar(p.p145, s.ad_value(141)))
                                        } else {
                                            A::constant(0.0)
                                        }
                                    }
                                }
                            }, (-1.0)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            let assign18720_ad_e35683: A = {
                if (!(((0.5 * s.v[146]) - (s.v[179] * (if (!((p.p97 / s.v[141]) > 1e-38)) { (-87.498233534) } else { (if ((p.p97 / s.v[141]) > 1e-38) { (((p.p97 / s.v[141])) as f64).ln() } else { 0.0 }) }))) < ((-10000.0) * 0.0001))) {
                    A::add_scaled_inputs(A::add_scaled_product(s.ad_value(146), 0.5, s.ad_value(179), {
                        if (!((p.p97 / s.v[141]) > 1e-38)) {
                            A::neg(A::constant(87.498233534))
                        } else {
                            {
                                if ((p.p97 / s.v[141]) > 1e-38) {
                                    A::ln(A::div_from_scalar(p.p97, s.ad_value(141)))
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }, (-1.0)), 0.5, A::sqrt_square_offset(A::add_scaled_product(s.ad_value(146), 0.5, s.ad_value(179), {
                        if (!((p.p97 / s.v[141]) > 1e-38)) {
                            A::neg(A::constant(87.498233534))
                        } else {
                            {
                                if ((p.p97 / s.v[141]) > 1e-38) {
                                    A::ln(A::div_from_scalar(p.p97, s.ad_value(141)))
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }, (-1.0)), ((4.0 * 0.0001) * 0.0001)), 0.5)
                } else {
                    {
                        if (((0.5 * s.v[146]) - (s.v[179] * (if (!((p.p97 / s.v[141]) > 1e-38)) { (-87.498233534) } else { (if ((p.p97 / s.v[141]) > 1e-38) { (((p.p97 / s.v[141])) as f64).ln() } else { 0.0 }) }))) < ((-10000.0) * 0.0001)) {
                            A::div_from_scalar(((-0.0001) * 0.0001), A::add_scaled_product(s.ad_value(146), 0.5, s.ad_value(179), {
                                if (!((p.p97 / s.v[141]) > 1e-38)) {
                                    A::neg(A::constant(87.498233534))
                                } else {
                                    {
                                        if ((p.p97 / s.v[141]) > 1e-38) {
                                            A::ln(A::div_from_scalar(p.p97, s.ad_value(141)))
                                        } else {
                                            A::constant(0.0)
                                        }
                                    }
                                }
                            }, (-1.0)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_mul_sub_ad_rhs(479, 114, assign18720_ad_e35490, A::add_scaled_product(s.ad_value(146), 0.5, s.ad_value(114), A::sub_scaled_inputs(s.ad_value(146), 0.5, assign18720_ad_e35683, 1.0), (-1.0)));
        }

        if ((s.b[1397] && s.b[1398]) && (!s.b[1399])) {
            let assign18730_ad_e36032: A = A::sub({
                if (!(((0.5 * s.v[146]) - (s.v[179] * ((if (!(p.p145 > 1e-38)) { (-87.498233534) } else { (if (p.p145 > 1e-38) { ((p.p145) as f64).ln() } else { 0.0 }) }) - s.v[142]))) < ((-10000.0) * 0.0001))) {
                    A::add_scaled_inputs3(s.ad_value(146), (0.5 * 0.5), A::mul_sub_from_scalar_rhs(s.ad_value(179), (if (!(p.p145 > 1e-38)) { (-87.498233534) } else { (if (p.p145 > 1e-38) { ((p.p145) as f64).ln() } else { 0.0 }) }), s.ad_value(142)), ((-1.0) * 0.5), A::sqrt_square_offset(A::sub_scaled_inputs(s.ad_value(146), 0.5, A::mul_sub_from_scalar_rhs(s.ad_value(179), (if (!(p.p145 > 1e-38)) { (-87.498233534) } else { (if (p.p145 > 1e-38) { ((p.p145) as f64).ln() } else { 0.0 }) }), s.ad_value(142)), 1.0), ((4.0 * 0.0001) * 0.0001)), 0.5)
                } else {
                    {
                        if (((0.5 * s.v[146]) - (s.v[179] * ((if (!(p.p145 > 1e-38)) { (-87.498233534) } else { (if (p.p145 > 1e-38) { ((p.p145) as f64).ln() } else { 0.0 }) }) - s.v[142]))) < ((-10000.0) * 0.0001)) {
                            A::div_from_scalar(((-0.0001) * 0.0001), A::sub_scaled_inputs(s.ad_value(146), 0.5, A::mul_sub_from_scalar_rhs(s.ad_value(179), (if (!(p.p145 > 1e-38)) { (-87.498233534) } else { (if (p.p145 > 1e-38) { ((p.p145) as f64).ln() } else { 0.0 }) }), s.ad_value(142)), 1.0))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, A::add_scaled_product(s.ad_value(146), 0.5, s.ad_value(114), A::sub_scaled_inputs(s.ad_value(146), 0.5, {
                if (!(((0.5 * s.v[146]) - (s.v[179] * ((if (!(p.p97 > 1e-38)) { (-87.498233534) } else { (if (p.p97 > 1e-38) { ((p.p97) as f64).ln() } else { 0.0 }) }) - s.v[142]))) < ((-10000.0) * 0.0001))) {
                    A::add_scaled_inputs3(s.ad_value(146), (0.5 * 0.5), A::mul_sub_from_scalar_rhs(s.ad_value(179), (if (!(p.p97 > 1e-38)) { (-87.498233534) } else { (if (p.p97 > 1e-38) { ((p.p97) as f64).ln() } else { 0.0 }) }), s.ad_value(142)), ((-1.0) * 0.5), A::sqrt_square_offset(A::sub_scaled_inputs(s.ad_value(146), 0.5, A::mul_sub_from_scalar_rhs(s.ad_value(179), (if (!(p.p97 > 1e-38)) { (-87.498233534) } else { (if (p.p97 > 1e-38) { ((p.p97) as f64).ln() } else { 0.0 }) }), s.ad_value(142)), 1.0), ((4.0 * 0.0001) * 0.0001)), 0.5)
                } else {
                    {
                        if (((0.5 * s.v[146]) - (s.v[179] * ((if (!(p.p97 > 1e-38)) { (-87.498233534) } else { (if (p.p97 > 1e-38) { ((p.p97) as f64).ln() } else { 0.0 }) }) - s.v[142]))) < ((-10000.0) * 0.0001)) {
                            A::div_from_scalar(((-0.0001) * 0.0001), A::sub_scaled_inputs(s.ad_value(146), 0.5, A::mul_sub_from_scalar_rhs(s.ad_value(179), (if (!(p.p97 > 1e-38)) { (-87.498233534) } else { (if (p.p97 > 1e-38) { ((p.p97) as f64).ln() } else { 0.0 }) }), s.ad_value(142)), 1.0))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 1.0), (-1.0)));
            s.store_mul_ad_rhs(479, 114, assign18730_ad_e36032);
        }

        s.b[1400] = (p.p80 == 0.0);
        s.store_scalar(1400, if s.b[1400] { 1.0 } else { 0.0 });

        if ((s.b[1397] && (!s.b[1398])) && s.b[1400]) {
            let assign18750_ad_e36241: A = {
                if (!(((0.5 * s.v[146]) - (s.v[179] * (if (!((p.p97 / s.v[141]) > 1e-38)) { (-87.498233534) } else { (if ((p.p97 / s.v[141]) > 1e-38) { (((p.p97 / s.v[141])) as f64).ln() } else { 0.0 }) }))) < ((-10000.0) * 0.0001))) {
                    A::add_scaled_inputs(A::add_scaled_product(s.ad_value(146), 0.5, s.ad_value(179), {
                        if (!((p.p97 / s.v[141]) > 1e-38)) {
                            A::neg(A::constant(87.498233534))
                        } else {
                            {
                                if ((p.p97 / s.v[141]) > 1e-38) {
                                    A::ln(A::div_from_scalar(p.p97, s.ad_value(141)))
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }, (-1.0)), 0.5, A::sqrt_square_offset(A::add_scaled_product(s.ad_value(146), 0.5, s.ad_value(179), {
                        if (!((p.p97 / s.v[141]) > 1e-38)) {
                            A::neg(A::constant(87.498233534))
                        } else {
                            {
                                if ((p.p97 / s.v[141]) > 1e-38) {
                                    A::ln(A::div_from_scalar(p.p97, s.ad_value(141)))
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }, (-1.0)), ((4.0 * 0.0001) * 0.0001)), 0.5)
                } else {
                    {
                        if (((0.5 * s.v[146]) - (s.v[179] * (if (!((p.p97 / s.v[141]) > 1e-38)) { (-87.498233534) } else { (if ((p.p97 / s.v[141]) > 1e-38) { (((p.p97 / s.v[141])) as f64).ln() } else { 0.0 }) }))) < ((-10000.0) * 0.0001)) {
                            A::div_from_scalar(((-0.0001) * 0.0001), A::add_scaled_product(s.ad_value(146), 0.5, s.ad_value(179), {
                                if (!((p.p97 / s.v[141]) > 1e-38)) {
                                    A::neg(A::constant(87.498233534))
                                } else {
                                    {
                                        if ((p.p97 / s.v[141]) > 1e-38) {
                                            A::ln(A::div_from_scalar(p.p97, s.ad_value(141)))
                                        } else {
                                            A::constant(0.0)
                                        }
                                    }
                                }
                            }, (-1.0)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_mul_sub_ad_rhs(479, 114, s.ad_value(641), A::add_scaled_product(A::scale_offset(s.ad_value(146), 0.5, p.p104), 1.0, s.ad_value(114), A::sub_scaled_inputs(s.ad_value(146), 0.5, assign18750_ad_e36241, 1.0), (-1.0)));
        }

        if ((s.b[1397] && (!s.b[1398])) && (!s.b[1400])) {
            s.store_mul_sub_ad_rhs(479, 114, s.ad_value(641), A::add_scaled_product(A::scale_offset(s.ad_value(146), 0.5, p.p104), 1.0, s.ad_value(114), A::sub_scaled_inputs(s.ad_value(146), 0.5, {
                if (!(((0.5 * s.v[146]) - (s.v[179] * ((if (!(p.p97 > 1e-38)) { (-87.498233534) } else { (if (p.p97 > 1e-38) { ((p.p97) as f64).ln() } else { 0.0 }) }) - s.v[142]))) < ((-10000.0) * 0.0001))) {
                    A::add_scaled_inputs3(s.ad_value(146), (0.5 * 0.5), A::mul_sub_from_scalar_rhs(s.ad_value(179), (if (!(p.p97 > 1e-38)) { (-87.498233534) } else { (if (p.p97 > 1e-38) { ((p.p97) as f64).ln() } else { 0.0 }) }), s.ad_value(142)), ((-1.0) * 0.5), A::sqrt_square_offset(A::sub_scaled_inputs(s.ad_value(146), 0.5, A::mul_sub_from_scalar_rhs(s.ad_value(179), (if (!(p.p97 > 1e-38)) { (-87.498233534) } else { (if (p.p97 > 1e-38) { ((p.p97) as f64).ln() } else { 0.0 }) }), s.ad_value(142)), 1.0), ((4.0 * 0.0001) * 0.0001)), 0.5)
                } else {
                    {
                        if (((0.5 * s.v[146]) - (s.v[179] * ((if (!(p.p97 > 1e-38)) { (-87.498233534) } else { (if (p.p97 > 1e-38) { ((p.p97) as f64).ln() } else { 0.0 }) }) - s.v[142]))) < ((-10000.0) * 0.0001)) {
                            A::div_from_scalar(((-0.0001) * 0.0001), A::sub_scaled_inputs(s.ad_value(146), 0.5, A::mul_sub_from_scalar_rhs(s.ad_value(179), (if (!(p.p97 > 1e-38)) { (-87.498233534) } else { (if (p.p97 > 1e-38) { ((p.p97) as f64).ln() } else { 0.0 }) }), s.ad_value(142)), 1.0))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 1.0), (-1.0)));
        }

        if (!s.b[1397]) {
            s.store_scalar(479, p.p1106);
        }

        s.b[1401] = (!param_given[1107]);
        s.store_scalar(1401, if s.b[1401] { 1.0 } else { 0.0 });

        if s.b[1401] {
            s.copy_ad(518, 479);
        }

        if (!s.b[1401]) {
            s.store_scalar(518, p.p1107);
        }

        s.b[1402] = (p.p80 == 0.0);
        s.store_scalar(1402, if s.b[1402] { 1.0 } else { 0.0 });

        if s.b[1402] {
            s.store_mul_ad_rhs(166, 179, {
                if (!((s.v[640] / s.v[141]) > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if ((s.v[640] / s.v[141]) > 1e-38) {
                            A::ln(A::div(s.ad_value(640), s.ad_value(141)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            });
        }

        if s.b[1402] {
            s.store_scaled_add_sqrt_square_offset_rhs(166, 166, 166, ((0.25 * 1e-10) * 1e-10), 0.5);
        }

        if s.b[1402] {
            s.store_mul_ad_rhs(352, 179, {
                if (!(((s.v[640] * p.p97) / (s.v[141] * s.v[141])) > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if (((s.v[640] * p.p97) / (s.v[141] * s.v[141])) > 1e-38) {
                            A::ln(A::div_scaled_inputs(s.ad_value(640), p.p97, A::square(s.ad_value(141)), 1.0))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            });
        }

        if (!s.b[1402]) {
            s.store_mul_sub_ad_rhs(166, 179, {
                if (!(s.v[640] > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if (s.v[640] > 1e-38) {
                            A::ln(s.ad_value(640))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, s.ad_value(142));
        }

        if (!s.b[1402]) {
            s.store_scaled_add_sqrt_square_offset_rhs(166, 166, 166, ((0.25 * 1e-10) * 1e-10), 0.5);
        }

        if (!s.b[1402]) {
            s.store_mul_sub_scaled_inputs_rhs(352, 179, {
                if (!((s.v[640] * p.p97) > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if ((s.v[640] * p.p97) > 1e-38) {
                            A::ln_scaled_input(s.ad_value(640), p.p97)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 1.0, s.ad_value(142), 2.0);
        }

        s.store_mul_sub_ad_rhs(167, 114, s.ad_value(641), A::offset({
            if (p.p60 == 1.0) {
                A::constant(0.0)
            } else {
                s.ad_value(146)
            }
        }, p.p104));

        s.store_scale(407, 322, 0.5);

        s.store_scalar(408, 0.5);

        s.b[1403] = (p.p60 != 1.0);
        s.store_scalar(1403, if s.b[1403] { 1.0 } else { 0.0 });

        if s.b[1403] {
            s.store_scale(407, 322, 0.333333333);
            s.store_scalar(408, 0.333333333);
        }

        s.b[1404] = (p.p61 != 0.0);
        s.store_scalar(1404, if s.b[1404] { 1.0 } else { 0.0 });

        if s.b[1404] {
            s.store_add_scaled_inputs3_indices(537, 275, p.p11, 276, p.p13, 277, (p.p3 * s.v[115]));
        }

        s.b[1405] = (s.v[537] > 0.0);
        s.store_scalar(1405, if s.b[1405] { 1.0 } else { 0.0 });

        if (s.b[1404] && s.b[1405]) {
            s.store_scale(539, 179, p.p1620);
            s.store_scaled_limited_exp_ad(547, A::div_from_scalar((-p.p1626), s.ad_value(539)), p.p1628);
            s.store_max_with_scalar_ad(170, A::div_from_scalar(p.p1622, s.ad_value(537)), 10.0);
            s.store_sub_offset_lhs(226, 170, 1.0, 547);
        }

        if (s.b[1404] && s.b[1405]) {
            s.store_mul_ad_rhs(546, 539, {
                if (!((0.5 * (s.v[226] + ((((s.v[226] * s.v[226]) + (4.0 * s.v[547]))) as f64).sqrt())) > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if ((0.5 * (s.v[226] + ((((s.v[226] * s.v[226]) + (4.0 * s.v[547]))) as f64).sqrt())) > 1e-38) {
                            A::ln_scaled_input(A::add(s.ad_value(226), A::sqrt(A::add_scaled_inputs(A::square(s.ad_value(226)), 1.0, s.ad_value(547), 4.0))), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            });
        }

        if (s.b[1404] && s.b[1405]) {
            s.store_limited_exp_div(168, 546, 539);
        }

        if (s.b[1404] && s.b[1405]) {
            s.store_offset_ad(170, {
                if (!(((p.p1624 / s.v[537]) - 10.0) < ((-10000.0) * 0.001))) {
                    A::add_scaled_inputs(A::offset(A::div_from_scalar(p.p1624, s.ad_value(537)), (-10.0)), 0.5, A::sqrt_square_offset(A::offset(A::div_from_scalar(p.p1624, s.ad_value(537)), (-10.0)), ((4.0 * 0.001) * 0.001)), 0.5)
                } else {
                    {
                        if (((p.p1624 / s.v[537]) - 10.0) < ((-10000.0) * 0.001)) {
                            A::div_scalar_offset_denominator(((-0.001) * 0.001), A::div_from_scalar(p.p1624, s.ad_value(537)), (-10.0), 1.0)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 10.0);
        }

        if (s.b[1404] && s.b[1405]) {
            s.store_sub_from_scalar_scaled_mul_ad_rhs(543, (-p.p1626), 539, {
                if (!(((s.v[170] - 1.0) / p.p1628) > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if (((s.v[170] - 1.0) / p.p1628) > 1e-38) {
                            A::ln_scaled_input(A::offset(s.ad_value(170), (-1.0)), 1.0 / (p.p1628))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 1.0);
        }

        if (s.b[1404] && s.b[1405]) {
            s.store_scale_ad(169, A::limited_exp_div_scaled_inputs(A::offset(s.ad_value(543), p.p1626), -1.0, s.ad_value(539), 1.0), p.p1628);
            s.store_mul_offset_rhs(542, 537, 169, 1.0);
            s.store_div_scaled_product_indices(541, 537, 169, -1.0, 539, 1.0);
        }

        if s.b[1404] {
            s.store_add_scaled_inputs3_indices(538, 278, p.p12, 279, p.p14, 280, (p.p3 * s.v[115]));
        }

        s.b[1406] = (s.v[538] > 0.0);
        s.store_scalar(1406, if s.b[1406] { 1.0 } else { 0.0 });

        if (s.b[1404] && s.b[1406]) {
            s.store_scale(540, 179, p.p1621);
            s.store_scaled_limited_exp_ad(554, A::div_from_scalar((-p.p1627), s.ad_value(540)), p.p1629);
            s.store_max_with_scalar_ad(170, A::div_from_scalar(p.p1623, s.ad_value(538)), 10.0);
            s.store_sub_offset_lhs(226, 170, 1.0, 554);
        }

    }

    pub(super) fn stamp_reactive_block_15(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        if (s.b[1404] && s.b[1406]) {
            s.store_mul_ad_rhs(553, 540, {
                if (!((0.5 * (s.v[226] + ((((s.v[226] * s.v[226]) + (4.0 * s.v[554]))) as f64).sqrt())) > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if ((0.5 * (s.v[226] + ((((s.v[226] * s.v[226]) + (4.0 * s.v[554]))) as f64).sqrt())) > 1e-38) {
                            A::ln_scaled_input(A::add(s.ad_value(226), A::sqrt(A::add_scaled_inputs(A::square(s.ad_value(226)), 1.0, s.ad_value(554), 4.0))), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            });
        }

        if (s.b[1404] && s.b[1406]) {
            s.store_limited_exp_div(168, 553, 540);
        }

        if (s.b[1404] && s.b[1406]) {
            s.store_offset_ad(170, {
                if (!(((p.p1625 / s.v[538]) - 10.0) < ((-10000.0) * 0.001))) {
                    A::add_scaled_inputs(A::offset(A::div_from_scalar(p.p1625, s.ad_value(538)), (-10.0)), 0.5, A::sqrt_square_offset(A::offset(A::div_from_scalar(p.p1625, s.ad_value(538)), (-10.0)), ((4.0 * 0.001) * 0.001)), 0.5)
                } else {
                    {
                        if (((p.p1625 / s.v[538]) - 10.0) < ((-10000.0) * 0.001)) {
                            A::div_scalar_offset_denominator(((-0.001) * 0.001), A::div_from_scalar(p.p1625, s.ad_value(538)), (-10.0), 1.0)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 10.0);
        }

        if (s.b[1404] && s.b[1406]) {
            s.store_sub_from_scalar_scaled_mul_ad_rhs(550, (-p.p1627), 540, {
                if (!(((s.v[170] - 1.0) / p.p1629) > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if (((s.v[170] - 1.0) / p.p1629) > 1e-38) {
                            A::ln_scaled_input(A::offset(s.ad_value(170), (-1.0)), 1.0 / (p.p1629))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 1.0);
        }

        if (s.b[1404] && s.b[1406]) {
            s.store_scale_ad(169, A::limited_exp_div_scaled_inputs(A::offset(s.ad_value(550), p.p1627), -1.0, s.ad_value(540), 1.0), p.p1629);
            s.store_mul_offset_rhs(549, 538, 169, 1.0);
            s.store_div_scaled_product_indices(548, 538, 169, -1.0, 540, 1.0);
        }

        if s.b[1404] {
            s.store_scale(523, 263, p.p11);
            s.store_scale(524, 264, p.p13);
            s.store_scaled_mul(525, 268, 158, s.v[115]);
            s.store_scale(526, 266, p.p12);
            s.store_scale(527, 267, p.p14);
            s.store_scaled_mul(528, 265, 158, s.v[115]);
        }

        s.b[1407] = (p.p1602 > 0.0);
        s.store_scalar(1407, if s.b[1407] { 1.0 } else { 0.0 });

        if (s.b[1404] && s.b[1407]) {
            s.store_scale(557, 269, (1.0 - (((1.0 / p.p1602)) as f64).powf((1.0 / p.p1596))));
            s.store_div_scaled_inputs_mixed_ia(558, 269, (p.p1602 * (p.p1608 * 1.0 / (p.p1596))), A::powf(A::sub_from_scalar(1.0, A::div(s.ad_value(557), s.ad_value(269))), (-(1.0 + p.p1596))), 1.0);
        }

        s.b[1408] = (p.p1604 > 0.0);
        s.store_scalar(1408, if s.b[1408] { 1.0 } else { 0.0 });

        if (s.b[1404] && s.b[1408]) {
            s.store_scale(559, 270, (1.0 - (((1.0 / p.p1604)) as f64).powf((1.0 / p.p1598))));
            s.store_div_scaled_inputs_mixed_ia(560, 270, (p.p1604 * (p.p1610 * 1.0 / (p.p1598))), A::powf(A::sub_from_scalar(1.0, A::div(s.ad_value(559), s.ad_value(270))), (-(1.0 + p.p1598))), 1.0);
        }

        s.b[1409] = (p.p1606 > 0.0);
        s.store_scalar(1409, if s.b[1409] { 1.0 } else { 0.0 });

        if (s.b[1404] && s.b[1409]) {
            s.store_scale(561, 271, (1.0 - (((1.0 / p.p1606)) as f64).powf((1.0 / p.p1600))));
            s.store_div_scaled_inputs_mixed_ia(562, 271, (p.p1606 * (p.p1612 * 1.0 / (p.p1600))), A::powf(A::sub_from_scalar(1.0, A::div(s.ad_value(561), s.ad_value(271))), (-(1.0 + p.p1600))), 1.0);
        }

        s.b[1410] = (p.p1603 > 0.0);
        s.store_scalar(1410, if s.b[1410] { 1.0 } else { 0.0 });

        if (s.b[1404] && s.b[1410]) {
            s.store_scale(563, 272, (1.0 - (((1.0 / p.p1603)) as f64).powf((1.0 / p.p1597))));
            s.store_div_scaled_inputs_mixed_ia(564, 272, (p.p1603 * (p.p1609 * 1.0 / (p.p1597))), A::powf(A::sub_from_scalar(1.0, A::div(s.ad_value(563), s.ad_value(272))), (-(1.0 + p.p1597))), 1.0);
        }

        s.b[1411] = (p.p1605 > 0.0);
        s.store_scalar(1411, if s.b[1411] { 1.0 } else { 0.0 });

        if (s.b[1404] && s.b[1411]) {
            s.store_scale(565, 273, (1.0 - (((1.0 / p.p1605)) as f64).powf((1.0 / p.p1599))));
            s.store_div_scaled_inputs_mixed_ia(566, 273, (p.p1605 * (p.p1611 * 1.0 / (p.p1599))), A::powf(A::sub_from_scalar(1.0, A::div(s.ad_value(565), s.ad_value(273))), (-(1.0 + p.p1599))), 1.0);
        }

        s.b[1412] = (p.p1607 > 0.0);
        s.store_scalar(1412, if s.b[1412] { 1.0 } else { 0.0 });

        if (s.b[1404] && s.b[1412]) {
            s.store_scale(567, 274, (1.0 - (((1.0 / p.p1607)) as f64).powf((1.0 / p.p1601))));
            s.store_div_scaled_inputs_mixed_ia(568, 274, (p.p1607 * (p.p1613 * 1.0 / (p.p1601))), A::powf(A::sub_from_scalar(1.0, A::div(s.ad_value(567), s.ad_value(274))), (-(1.0 + p.p1601))), 1.0);
        }

        s.store_mul_voltage_ad(134, s.ad_value(114), ctx, nodes, Some(11), Some(6));

        s.store_mul_voltage_ad(135, s.ad_value(114), ctx, nodes, Some(5), Some(6));

        s.store_mul_voltage_ad(136, s.ad_value(114), ctx, nodes, Some(11), Some(5));

        s.store_mul_voltage_ad(521, s.ad_value(114), ctx, nodes, Some(3), Some(6));

        s.store_mul_voltage_ad(522, s.ad_value(114), ctx, nodes, Some(3), Some(5));

        s.store_mul_voltage_ad(497, s.ad_value(114), ctx, nodes, Some(11), Some(3));

        s.b[1413] = (p.p76 != 2.0);
        s.store_scalar(1413, if s.b[1413] { 1.0 } else { 0.0 });

        if s.b[1413] {
            s.store_mul_voltage_ad(132, s.ad_value(114), ctx, nodes, Some(10), Some(5));
            s.store_mul_voltage_ad(133, s.ad_value(114), ctx, nodes, Some(10), Some(6));
        }

        if (!s.b[1413]) {
            s.store_mul_voltage_ad(132, s.ad_value(114), ctx, nodes, Some(14), Some(5));
            s.store_mul_voltage_ad(133, s.ad_value(114), ctx, nodes, Some(13), Some(6));
        }

        s.store_scalar(128, 1.0);

        s.b[1414] = (s.v[135] < 0.0);
        s.store_scalar(1414, if s.b[1414] { 1.0 } else { 0.0 });

        if s.b[1414] {
            s.store_scalar(128, (-1.0));
            s.store_sub(125, 134, 135);
            s.store_scale(126, 135, (-1.0));
            s.copy_ad(367, 522);
        }

        if (!s.b[1414]) {
            s.copy_ad(125, 134);
            s.copy_ad(126, 135);
            s.copy_ad(367, 521);
        }

        s.store_sub(347, 125, 167);

        s.store_offset_sqrt_ad(127, A::offset(A::square(s.ad_value(126)), 0.01), (-0.1));

        s.b[1415] = (p.p61 != 0.0);
        s.store_scalar(1415, if s.b[1415] { 1.0 } else { 0.0 });

        if s.b[1415] {
            s.store_add_scaled_inputs3_indices(368, 367, 1.0, 126, (-0.5), 127, (-(-0.5)));
            s.store_scale(369, 689, 0.95);
            s.store_offset_sub(170, 369, 368, (-0.001));
            s.store_add_scaled_inputs3_sqrt_third_mixed_iia(370, 369, 1.0, 170, (-0.5), A::add_scaled_inputs(A::square(s.ad_value(170)), 1.0, s.ad_value(369), 0.004), (-0.5));
        }

        s.store_tanh_ad(168, A::div_scaled_inputs(s.ad_value(135), 0.6, s.ad_value(179), 1.0));

        s.store_offset_scaled(186, 168, 0.5, 0.5);

        s.store_sub_from_scalar(187, 1.0, 186);

        s.b[1416] = (p.p66 != 0.0);
        s.store_scalar(1416, if s.b[1416] { 1.0 } else { 0.0 });

        if s.b[1416] {
            s.store_add_scaled_products_indices(664, 665, 187, 1.0, 663, 186, 1.0);
            s.store_add_scaled_products_indices(676, 298, 187, 1.0, 296, 186, 1.0);
            s.store_add_scaled_products_indices(427, 715, 187, 1.0, 714, 186, 1.0);
            s.store_add_scaled_products_indices(718, 717, 187, 1.0, 716, 186, 1.0);
            s.store_add_scaled_products_indices(423, 338, 187, 1.0, 337, 186, 1.0);
            s.store_add_scaled_products_indices(424, 258, 187, 1.0, 257, 186, 1.0);
            s.store_add_scaled_products_indices(422, 335, 187, 1.0, 334, 186, 1.0);
            s.store_add_scaled_products_indices(425, 300, 187, 1.0, 299, 186, 1.0);
            s.store_add_scaled_products_indices(426, 302, 187, 1.0, 301, 186, 1.0);
            s.store_add_scaled_products_indices(795, 796, 187, 1.0, 797, 186, 1.0);
            s.store_add_scaled_products_indices(428, 333, 187, 1.0, 332, 186, 1.0);
            s.store_add_scaled_products_indices(659, 658, 187, 1.0, 660, 186, 1.0);
            s.store_add_scaled_products_indices(805, 806, 187, 1.0, 804, 186, 1.0);
            s.store_add_scaled_products_indices(669, 668, 187, 1.0, 666, 186, 1.0);
            s.store_add_scaled_products_indices(416, 417, 187, 1.0, 413, 186, 1.0);
            s.store_add_scaled_products_indices(819, 305, 187, 1.0, 303, 186, 1.0);
            s.store_add_scaled_products_indices(820, 320, 187, 1.0, 318, 186, 1.0);
            s.store_add_scaled_products_indices(821, 316, 187, 1.0, 314, 186, 1.0);
            s.store_add_scaled_products_indices(822, 816, 187, 1.0, 323, 186, 1.0);
        }

        if (!s.b[1416]) {
            s.copy_ad(664, 663);
            s.copy_ad(676, 296);
            s.copy_ad(427, 714);
            s.copy_ad(718, 716);
            s.copy_ad(423, 337);
            s.copy_ad(424, 257);
            s.copy_ad(422, 334);
            s.copy_ad(425, 299);
            s.copy_ad(426, 301);
            s.copy_ad(795, 797);
            s.copy_ad(428, 332);
            s.copy_ad(659, 660);
            s.copy_ad(805, 804);
            s.copy_ad(669, 666);
            s.copy_ad(416, 413);
            s.copy_ad(819, 303);
            s.copy_ad(820, 318);
            s.copy_ad(821, 314);
            s.copy_ad(822, 323);
        }

        s.store_div_from_scalar(212, 1.0, 423);

        s.store_add_offset_lhs(353, 166, 0.4, 672);

        s.store_div_scaled_value_by_product(169, s.ad_value(893), 2.0, s.ad_value(895), A::offset(s.ad_value(898), 2.0), 1.0);

        s.store_mul_add_scaled_product_rhs(164, 362, s.ad_value(662), 1.0, s.ad_value(664), s.ad_value(127), 1.0);

        s.b[1417] = (p.p175 == 0.0);
        s.store_scalar(1417, if s.b[1417] { 1.0 } else { 0.0 });

        s.b[1418] = (p.p80 == 0.0);
        s.store_scalar(1418, if s.b[1418] { 1.0 } else { 0.0 });

        if (s.b[1417] && s.b[1418]) {
            s.store_mul_ad_product_rhs_mixed_ia(181, 179, 235, A::offset(A::div_scaled_inputs2(s.ad_value(669), 1.0, s.ad_value(164), 1.0, s.ad_value(169), 1.0), 1.0));
        }

        if (s.b[1417] && (!s.b[1418])) {
            s.store_mul_ad_product_rhs_mixed_ia(181, 182, 235, A::offset(A::div_scaled_inputs2(s.ad_value(669), 1.0, s.ad_value(164), 1.0, s.ad_value(169), 1.0), 1.0));
        }

        if (!s.b[1417]) {
            s.store_scalar(181, p.p175);
        }

        s.store_div(897, 903, 181);

        if (!(((s.v[893] * s.v[181]) / (((1.60219e-19 * s.v[148]) * 2.0) * s.v[894])) > 1e-38)) {
            s.store_scalar(900, (-87.498233534));
        } else {
            if (((s.v[893] * s.v[181]) / (((1.60219e-19 * s.v[148]) * 2.0) * s.v[894])) > 1e-38) {
                s.store_ln_ad(900, A::div_scaled_product_by_product(s.ad_value(893), s.ad_value(181), 1.0, s.ad_value(148), s.ad_value(894), (1.60219e-19 * 2.0)));
            } else {
                s.store_scalar(900, 0.0);
            }
        }

        s.store_add_ad_lhs(899, {
            if (!(A::div_scaled_product_offset_denominator(A::mul(s.ad_value(897), s.ad_value(898)), A::mul(s.ad_value(897), s.ad_value(898)), 1.0, A::add_scaled_product(A::limited_exp(A::mul(s.ad_value(897), s.ad_value(898))), 1.0, s.ad_value(897), s.ad_value(898), (-1.0)), (-1.0), 1.0).value > 1e-38)) {
                A::neg(A::constant(87.498233534))
            } else {
                {
                    if (A::div_scaled_product_offset_denominator(A::mul(s.ad_value(897), s.ad_value(898)), A::mul(s.ad_value(897), s.ad_value(898)), 1.0, A::add_scaled_product(A::limited_exp(A::mul(s.ad_value(897), s.ad_value(898))), 1.0, s.ad_value(897), s.ad_value(898), (-1.0)), (-1.0), 1.0).value > 1e-38) {
                        A::ln(A::div_scaled_product_offset_denominator(A::mul(s.ad_value(897), s.ad_value(898)), A::mul(s.ad_value(897), s.ad_value(898)), 1.0, A::add_scaled_product(A::limited_exp(A::mul(s.ad_value(897), s.ad_value(898))), 1.0, s.ad_value(897), s.ad_value(898), (-1.0)), (-1.0), 1.0))
                    } else {
                        A::constant(0.0)
                    }
                }
            }
        }, 900);

        s.store_add_scaled_ad_lhs(339, A::div_scaled_inputs(s.ad_value(181), 10.0, s.ad_value(898), 1.0), 396, 2.0);

        s.store_div_scaled_product_indices(912, 179, 893, 1.0, 895, s.v[143]);

        s.store_scalar(913, ((((((4.5 * 1.05457e-34) * 3.141592653589793) * 1.60219e-19) / (4.0 * (((2.0 * s.v[381])) as f64).sqrt()))) as f64).powf(0.666666667));

        s.store_div_scaled_inputs_mixed_ai(914, A::powf(s.ad_value(912), 0.666666667), (p.p1804 * s.v[913]), 179, 1.60219e-19);

        s.store_mul_ad_affine_product_rhs(354, 667, s.ad_value(361), A::sub(s.ad_value(352), s.ad_value(353)), -1.0, 0.0);

        s.store_add_ad(355, A::mul3_scaled_output(s.ad_value(676), s.ad_value(363), A::add_scaled_product(s.ad_value(127), 1.0, s.ad_value(674), A::sqrt(A::offset(s.ad_value(127), 0.01)), 1.0), -1.0), A::mul3(s.ad_value(681), s.ad_value(365), A::pow(A::offset(s.ad_value(127), 0.01), s.ad_value(682))));

        s.store_mul_ad_product_rhs_mixed_ia(357, 802, 364, A::sqrt(s.ad_value(353)));

        s.store_add_ad_lhs(358, A::add_scaled_inputs4(s.ad_value(354), 1.0, s.ad_value(355), 1.0, s.ad_value(357), 1.0, s.ad_value(231), 1.0), 805);

        s.store_sub(347, 347, 358);

        s.store_div_scaled_product3_indices(184, 416, 163, 158, 1.0, 153, 1.0);

        s.b[1419] = (p.p80 == 0.0);
        s.store_scalar(1419, if s.b[1419] { 1.0 } else { 0.0 });

        if s.b[1419] {
            s.store_pow_ad(171, A::div_scaled_inputs(s.ad_value(163), (2.0 * p.p108), A::mul3_scaled_output(s.ad_value(184), s.ad_value(181), s.ad_value(148), (1.60219e-19 * p.p3)), 1.0), s.ad_value(181));
        }

        if s.b[1419] {
            s.store_neg_ad(168, A::add(s.ad_value(375), {
                if (!(s.v[171] > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if (s.v[171] > 1e-38) {
                            A::ln(s.ad_value(171))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }));
        }

        if s.b[1419] {
            s.store_offset_add(169, 347, 168, p.p23);
        }

    }

    pub(super) fn stamp_reactive_block_16(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if s.b[1419] {
            s.store_sub_ad_lhs(348, {
                if (!(s.v[169] < ((-10000.0) * 0.0001))) {
                    A::add_scaled_inputs(s.ad_value(169), 0.5, A::sqrt_square_offset(s.ad_value(169), ((4.0 * 0.0001) * 0.0001)), 0.5)
                } else {
                    {
                        if (s.v[169] < ((-10000.0) * 0.0001)) {
                            A::div_from_scalar(((-0.0001) * 0.0001), s.ad_value(169))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 168);
        }

        if (!s.b[1419]) {
            s.store_mul_scaled_ad_rhs(168, 181, -1.0, {
                if (!((((2.0 * s.v[163]) * p.p108) / ((((s.v[184] * s.v[181]) * 1.60219e-19) * s.v[148]) * p.p3)) > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if ((((2.0 * s.v[163]) * p.p108) / ((((s.v[184] * s.v[181]) * 1.60219e-19) * s.v[148]) * p.p3)) > 1e-38) {
                            A::ln(A::div_scaled_inputs(s.ad_value(163), (2.0 * p.p108), A::mul3_scaled_output(s.ad_value(184), s.ad_value(181), s.ad_value(148), (1.60219e-19 * p.p3)), 1.0))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            });
        }

        if (!s.b[1419]) {
            s.store_sub_ad_lhs(169, A::add_scaled_inputs(A::offset(s.ad_value(168), 0.01), 0.5, A::sqrt_square_offset(A::offset(s.ad_value(168), (-0.01)), ((0.25 * 0.0001) * 0.0001)), 0.5), 375);
            s.store_offset_add(170, 347, 169, p.p23);
        }

        if (!s.b[1419]) {
            s.store_sub_ad_lhs(348, {
                if (!(s.v[170] < ((-10000.0) * 0.0001))) {
                    A::add_scaled_inputs(s.ad_value(170), 0.5, A::sqrt_square_offset(s.ad_value(170), ((4.0 * 0.0001) * 0.0001)), 0.5)
                } else {
                    {
                        if (s.v[170] < ((-10000.0) * 0.0001)) {
                            A::div_from_scalar(((-0.0001) * 0.0001), s.ad_value(170))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 169);
        }

        s.copy_ad(129, 375);

        s.store_powf_ad(172, A::neg(s.ad_value(897)), 0.666666667);

        s.b[1420] = (p.p61 != 0.0);
        s.store_scalar(1420, if s.b[1420] { 1.0 } else { 0.0 });

        if s.b[1420] {
            if (!((((2.0 * s.v[166]) + s.v[129]) - s.v[367]) < ((-10000.0) * 0.1))) {
                s.store_add_scaled_inputs4_mixed_iiia(169, 166, (2.0 * 0.5), 129, 0.5, 367, (-0.5), A::sqrt_square_offset(A::add_scaled_inputs3(s.ad_value(166), 2.0, s.ad_value(129), 1.0, s.ad_value(367), -1.0), ((4.0 * 0.1) * 0.1)), 0.5);
            } else {
                if ((((2.0 * s.v[166]) + s.v[129]) - s.v[367]) < ((-10000.0) * 0.1)) {
                    s.store_div_from_scalar_ad(169, ((-0.1) * 0.1), A::add_scaled_inputs3(s.ad_value(166), 2.0, s.ad_value(129), 1.0, s.ad_value(367), -1.0));
                } else {
                    s.store_scalar(169, 0.0);
                }
            }
        }

        if s.b[1420] {
            s.store_mul_div_scaled_inputs_mixed_aii(171, A::sub(A::sqrt(s.ad_value(169)), A::sqrt_scaled_input(s.ad_value(166), 2.0)), 239, -1.0, 181, 2.0);
            s.store_add_scaled_product_value_ad(168, A::add_scaled_inputs3(s.ad_value(897), -1.0, s.ad_value(171), (-1.0), s.ad_value(899), 1.0), 1.0, 914, 172, 1.0);
            s.store_add_scaled_inputs3_indices(169, 897, -1.0, 171, (-1.0), 900, 1.0);
        }

        if (!s.b[1420]) {
            s.store_add_scaled_inputs_product_indices(168, 899, 1.0, 897, (-1.0), 914, 172, 1.0);
            s.store_sub(169, 900, 897);
        }

        s.store_div_scaled_inputs2_indices(170, 348, 1.0, 129, (-1.0), 181, 1.0);

        s.store_sub(924, 169, 170);

        s.store_scaled_sub(171, 170, 168, 0.5);

        s.store_limited_exp(901, 171);

        s.b[1421] = (s.v[901] > 1e-7);
        s.store_scalar(1421, if s.b[1421] { 1.0 } else { 0.0 });

        if s.b[1421] {
            s.store_ln_offset_input(176, 901, 1.0);
            s.store_offset_scaled_ad(901, A::sqrt_square_offset(s.ad_value(176), 1.0), (-2.0), 2.0);
            s.store_mul_add_scaled_inputs_rhs(177, 898, s.ad_value(901), p.p1805, s.ad_value(897), 1.0);
            s.store_div_scaled_value_offset_denominator(172, s.ad_value(177), 1.0, A::sub(A::limited_exp(s.ad_value(177)), s.ad_value(177)), (-1.0), 1.0);
            s.store_mul(174, 177, 172);
            s.store_ln_neg_add(902, 901, 897);
        }

        if s.b[1421] {
            s.store_add_scaled_product_mixed_aia(344, A::add_scaled_inputs4(s.ad_value(924), 1.0, s.ad_value(901), (-1.0), {
                if (!((-s.v[901]) > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if ((-s.v[901]) > 1e-38) {
                            A::ln_scaled_input(s.ad_value(901), -1.0)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 1.0, {
                if (!(s.v[174] > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if (s.v[174] > 1e-38) {
                            A::ln(s.ad_value(174))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 1.0), 1.0, 914, A::exp_scaled_input(s.ad_value(902), 0.666666667), 1.0);
        }

        if s.b[1421] {
            s.store_add_scaled_offset_product_lhs_product_mixed_aaiia(345, A::offset(A::div_from_scalar(1.0, s.ad_value(901)), (-1.0)), 1.0, A::sub(A::div_from_scalar(2.0, s.ad_value(177)), s.ad_value(172)), (-1.0), 898, 1.0, 914, A::exp_scaled_input(s.ad_value(902), (-0.333333333)), (-0.666666667));
            s.store_add_scaled_product_mixed_aia(346, A::div_from_scalar((-1.0), A::square(s.ad_value(901))), 1.0, 914, A::exp_scaled_input(s.ad_value(902), (-1.333333333)), (-(2.0 / 9.0)));
            s.store_add_scaled_offset_product_rhs_mixed_iaa(901, 901, 1.0, A::div(s.ad_value(344), s.ad_value(345)), A::div_scaled_product_by_product(s.ad_value(344), s.ad_value(346), 1.0, s.ad_value(345), s.ad_value(345), 2.0), 1.0, (-1.0));
            s.store_mul_add_scaled_inputs_rhs(177, 898, s.ad_value(901), p.p1805, s.ad_value(897), 1.0);
            s.store_div_scaled_value_offset_denominator(172, s.ad_value(177), 1.0, A::sub(A::limited_exp(s.ad_value(177)), s.ad_value(177)), (-1.0), 1.0);
            s.store_mul(174, 177, 172);
            s.store_ln_neg_add(902, 901, 897);
        }

        if s.b[1421] {
            s.store_add_scaled_product_mixed_aia(344, A::add_scaled_inputs4(s.ad_value(924), 1.0, s.ad_value(901), (-1.0), {
                if (!((-s.v[901]) > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if ((-s.v[901]) > 1e-38) {
                            A::ln_scaled_input(s.ad_value(901), -1.0)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 1.0, {
                if (!(s.v[174] > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if (s.v[174] > 1e-38) {
                            A::ln(s.ad_value(174))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 1.0), 1.0, 914, A::exp_scaled_input(s.ad_value(902), 0.666666667), 1.0);
        }

        if s.b[1421] {
            s.store_add_scaled_offset_product_lhs_product_mixed_aaiia(345, A::offset(A::div_from_scalar(1.0, s.ad_value(901)), (-1.0)), 1.0, A::sub(A::div_from_scalar(2.0, s.ad_value(177)), s.ad_value(172)), (-1.0), 898, 1.0, 914, A::exp_scaled_input(s.ad_value(902), (-0.333333333)), (-0.666666667));
            s.store_add_scaled_product_mixed_aia(346, A::div_from_scalar((-1.0), A::square(s.ad_value(901))), 1.0, 914, A::exp_scaled_input(s.ad_value(902), (-1.333333333)), (-(2.0 / 9.0)));
            s.store_add_scaled_offset_product_rhs_mixed_iaa(901, 901, 1.0, A::div(s.ad_value(344), s.ad_value(345)), A::div_scaled_product_by_product(s.ad_value(344), s.ad_value(346), 1.0, s.ad_value(345), s.ad_value(345), 2.0), 1.0, (-1.0));
        }

        if (!s.b[1421]) {
            s.store_mul_neg_lhs(901, 901, 901);
        }

        s.store_mul_neg_lhs(392, 901, 181);

        s.b[1422] = (p.p57 == 1.0);
        s.store_scalar(1422, if s.b[1422] { 1.0 } else { 0.0 });

        if s.b[1422] {
            s.store_div_scaled_inputs2_indices(1015, 347, 1.0, 129, (-1.0), 181, 1.0);
            s.store_scaled_add_ad_rhs(1016, 1015, A::sqrt(A::add_scaled_square_product(s.ad_value(1015), 1.0, s.ad_value(963), s.ad_value(963), 0.25)), 0.5);
            s.store_pow_ad(1017, s.ad_value(1016), A::scale(s.ad_value(960), 0.5));
            s.store_mul_ad_product_rhs_mixed_ia(1004, 1010, 1017, A::limited_exp(A::sub(s.ad_value(1015), s.ad_value(1016))));
            s.store_div_scaled_inputs3_indices(1018, 347, 1.0, 129, (-1.0), 985, -1.0, 181, 1.0);
            s.store_scaled_add_ad_rhs(1019, 1018, A::sqrt(A::add_scaled_square_product(s.ad_value(1018), 1.0, s.ad_value(964), s.ad_value(964), 0.25)), 0.5);
            s.store_pow_ad(1020, s.ad_value(1019), A::scale(s.ad_value(961), 0.5));
            s.store_mul_ad_product_rhs_mixed_ia(1005, 1011, 1020, A::limited_exp(A::sub(s.ad_value(1018), s.ad_value(1019))));
            s.store_div_scaled_inputs3_indices(1021, 347, 1.0, 129, (-1.0), 986, -1.0, 181, 1.0);
            s.store_scaled_add_ad_rhs(1022, 1021, A::sqrt(A::add_scaled_square_product(s.ad_value(1021), 1.0, s.ad_value(965), s.ad_value(965), 0.25)), 0.5);
            s.store_pow_ad(1023, s.ad_value(1022), A::scale(s.ad_value(962), 0.5));
            s.store_mul_ad_product_rhs_mixed_ia(1006, 1012, 1023, A::limited_exp(A::sub(s.ad_value(1021), s.ad_value(1022))));
            s.store_add_scaled_products_right_right_ad(392, 983, 392, 1.0, 984, A::add_scaled_inputs3(s.ad_value(1004), 1.0, s.ad_value(1005), 1.0, s.ad_value(1006), 1.0), 1.0);
        }

        s.store_div_from_scalar(406, 0.01, 163);

        s.store_add_scaled_product_indices(419, 396, s.v[420], 407, 392, s.v[420]);

        s.store_pow_ad(170, A::scaled_offset(A::div(s.ad_value(392), s.ad_value(406)), 1.0, 0.5), s.ad_value(317));

        s.store_pow_indices(171, 419, 822);

        s.b[1423] = (p.p61 != 0.0);
        s.store_scalar(1423, if s.b[1423] { 1.0 } else { 0.0 });

        if s.b[1423] {
            s.store_add_scaled_product_mixed_aai(171, A::div(s.ad_value(820), s.ad_value(170)), 1.0, A::add_scaled_product(s.ad_value(819), 1.0, s.ad_value(821), s.ad_value(370), 1.0), 171, 1.0);
        }

        if (!s.b[1423]) {
            s.store_add_scaled_product_value_ad(171, A::div(s.ad_value(820), s.ad_value(170)), 1.0, 819, 171, 1.0);
        }

        s.store_offset(397, 171, 1.0);

        s.store_scaled_add_offset_sqrt_square_offset(397, 397, 1.0, (-1.0), ((0.25 * p.p604) * p.p604), 0.5);

        s.store_scale(397, 397, 1.0 / (p.p24));

        s.b[1424] = (p.p64 == 1.0);
        s.store_scalar(1424, if s.b[1424] { 1.0 } else { 0.0 });

        if s.b[1424] {
            s.store_scalar(198, 0.0);
        }

        s.b[1425] = (p.p64 == 0.0);
        s.store_scalar(1425, if s.b[1425] { 1.0 } else { 0.0 });

        if ((!s.b[1424]) && s.b[1425]) {
            s.store_offset_mul(172, 711, 392, 1.0);
            s.store_div_from_scalar(169, 1.0, 172);
            s.store_scaled_add_sqrt_square_offset_rhs(168, 169, 169, 0.01, 0.5);
            s.store_mul_ad_affine_product_lhs(198, A::offset(A::mul(s.ad_value(709), s.ad_value(168)), p.p908), s.ad_value(189), s.v[115], 0.0, 194);
        }

        if ((!s.b[1424]) && (!s.b[1425])) {
            s.store_offset_mul(172, 711, 392, 1.0);
            s.store_div_from_scalar(169, 1.0, 172);
            s.store_scaled_add_sqrt_square_offset_rhs(168, 169, 169, 0.01, 0.5);
            s.store_mul_ad_lhs(198, A::add_scaled_inputs_product(s.ad_value(190), 1.0, s.ad_value(191), 1.0, A::offset(A::mul(s.ad_value(709), s.ad_value(168)), p.p908), s.ad_value(189), s.v[115]), 194);
        }

        s.store_mul_div_scaled_inputs_indices(216, 397, 428, 2.0, 416, 1.0);

        s.store_mul(217, 216, 153);

        s.b[1426] = (p.p80 == 0.0);
        s.store_scalar(1426, if s.b[1426] { 1.0 } else { 0.0 });

        if s.b[1426] {
            s.store_mul_add_scaled_inputs_rhs(175, 659, s.ad_value(392), 1.0, s.ad_value(179), 2.0);
        }

        if (!s.b[1426]) {
            s.store_mul_add_scaled_inputs_rhs(175, 659, s.ad_value(392), 1.0, s.ad_value(182), 2.0);
        }

        s.b[1427] = (s.v[198] > 0.0);
        s.store_scalar(1427, if s.b[1427] { 1.0 } else { 0.0 });

        if s.b[1427] {
            s.store_mul3_lhs(224, 158, 428, 163);
            s.store_mul(168, 224, 198);
            s.store_scale(225, 168, 2.0);
            s.store_add_scaled_inputs_product_indices(226, 175, 1.0, 217, 1.0, 175, 168, 3.0);
            s.store_mul_add_scaled_product_rhs(227, 175, s.ad_value(217), 1.0, s.ad_value(175), s.ad_value(168), 2.0);
            s.store_div_scaled_inputs2(210, A::square(s.ad_value(226)), 1.0, A::add_scaled_square_product(s.ad_value(226), 1.0, s.ad_value(225), s.ad_value(227), (-2.0)), (-1.0), A::mul(A::add(s.ad_value(226), A::sqrt(A::add_scaled_square_product(s.ad_value(226), 1.0, s.ad_value(225), s.ad_value(227), (-2.0)))), s.ad_value(225)), 1.0);
        }

        if (!s.b[1427]) {
            s.store_div_scaled_product_add_scaled_denominator_indices(210, 217, 175, 1.0, 217, 1.0, 175, 1.0, 1.0);
        }

        s.store_offset_ad(210, {
            if (!((s.v[210] - 0.001) < ((-10000.0) * 1e-5))) {
                A::add_scaled_inputs(A::offset(s.ad_value(210), (-0.001)), 0.5, A::sqrt_square_offset(A::offset(s.ad_value(210), (-0.001)), ((4.0 * 1e-5) * 1e-5)), 0.5)
            } else {
                {
                    if ((s.v[210] - 0.001) < ((-10000.0) * 1e-5)) {
                        A::div_scalar_offset_denominator(((-1e-5) * 1e-5), s.ad_value(210), (-0.001), 1.0)
                    } else {
                        A::constant(0.0)
                    }
                }
            }
        }, 0.001);

        s.store_pow_ad(176, A::offset(A::div(s.ad_value(126), s.ad_value(210)), 1e-6), s.ad_value(423));

        s.store_pow_ad(177, A::offset(s.ad_value(176), 1.0), s.ad_value(212));

        s.store_min_ad(390, A::div(s.ad_value(126), s.ad_value(177)), s.ad_value(126));

        s.store_add(129, 390, 375);

        s.store_powf_ad(170, A::neg(s.ad_value(897)), 0.666666667);

        s.b[1428] = (p.p61 != 0.0);
        s.store_scalar(1428, if s.b[1428] { 1.0 } else { 0.0 });

        if s.b[1428] {
            if (!((((2.0 * s.v[166]) + s.v[129]) - s.v[367]) < ((-10000.0) * 0.1))) {
                s.store_add_scaled_inputs4_mixed_iiia(169, 166, (2.0 * 0.5), 129, 0.5, 367, (-0.5), A::sqrt_square_offset(A::add_scaled_inputs3(s.ad_value(166), 2.0, s.ad_value(129), 1.0, s.ad_value(367), -1.0), ((4.0 * 0.1) * 0.1)), 0.5);
            } else {
                if ((((2.0 * s.v[166]) + s.v[129]) - s.v[367]) < ((-10000.0) * 0.1)) {
                    s.store_div_from_scalar_ad(169, ((-0.1) * 0.1), A::add_scaled_inputs3(s.ad_value(166), 2.0, s.ad_value(129), 1.0, s.ad_value(367), -1.0));
                } else {
                    s.store_scalar(169, 0.0);
                }
            }
        }

        if s.b[1428] {
            s.store_mul_div_scaled_inputs_mixed_aii(171, A::sub(A::sqrt(s.ad_value(169)), A::sqrt_scaled_input(s.ad_value(166), 2.0)), 239, -1.0, 181, 2.0);
            s.store_add_scaled_product_value_ad(168, A::add_scaled_inputs3(s.ad_value(897), -1.0, s.ad_value(171), (-1.0), s.ad_value(899), 1.0), 1.0, 914, 170, 1.0);
            s.store_add_scaled_inputs3_indices(169, 897, -1.0, 171, (-1.0), 900, 1.0);
        }

        if (!s.b[1428]) {
            s.store_add_scaled_inputs_product_indices(168, 899, 1.0, 897, (-1.0), 914, 170, 1.0);
            s.store_sub(169, 900, 897);
        }

        s.store_div_scaled_inputs2_indices(170, 348, 1.0, 129, (-1.0), 181, 1.0);

        s.store_sub(924, 169, 170);

        s.store_scaled_sub(171, 170, 168, 0.5);

        s.store_limited_exp(901, 171);

        s.b[1429] = (s.v[901] > 1e-7);
        s.store_scalar(1429, if s.b[1429] { 1.0 } else { 0.0 });

        if s.b[1429] {
            s.store_ln_offset_input(176, 901, 1.0);
            s.store_offset_scaled_ad(901, A::sqrt_square_offset(s.ad_value(176), 1.0), (-2.0), 2.0);
            s.store_mul_add_scaled_inputs_rhs(177, 898, s.ad_value(901), p.p1805, s.ad_value(897), 1.0);
            s.store_div_scaled_value_offset_denominator(172, s.ad_value(177), 1.0, A::sub(A::limited_exp(s.ad_value(177)), s.ad_value(177)), (-1.0), 1.0);
            s.store_mul(174, 177, 172);
            s.store_ln_neg_add(902, 901, 897);
        }

    }

    pub(super) fn stamp_reactive_block_17(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if s.b[1429] {
            s.store_add_scaled_product_mixed_aia(344, A::add_scaled_inputs4(s.ad_value(924), 1.0, s.ad_value(901), (-1.0), {
                if (!((-s.v[901]) > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if ((-s.v[901]) > 1e-38) {
                            A::ln_scaled_input(s.ad_value(901), -1.0)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 1.0, {
                if (!(s.v[174] > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if (s.v[174] > 1e-38) {
                            A::ln(s.ad_value(174))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 1.0), 1.0, 914, A::exp_scaled_input(s.ad_value(902), 0.666666667), 1.0);
        }

        if s.b[1429] {
            s.store_add_scaled_offset_product_lhs_product_mixed_aaiia(345, A::offset(A::div_from_scalar(1.0, s.ad_value(901)), (-1.0)), 1.0, A::sub(A::div_from_scalar(2.0, s.ad_value(177)), s.ad_value(172)), (-1.0), 898, 1.0, 914, A::exp_scaled_input(s.ad_value(902), (-0.333333333)), (-0.666666667));
            s.store_add_scaled_product_mixed_aia(346, A::div_from_scalar((-1.0), A::square(s.ad_value(901))), 1.0, 914, A::exp_scaled_input(s.ad_value(902), (-1.333333333)), (-(2.0 / 9.0)));
            s.store_add_scaled_offset_product_rhs_mixed_iaa(901, 901, 1.0, A::div(s.ad_value(344), s.ad_value(345)), A::div_scaled_product_by_product(s.ad_value(344), s.ad_value(346), 1.0, s.ad_value(345), s.ad_value(345), 2.0), 1.0, (-1.0));
            s.store_mul_add_scaled_inputs_rhs(177, 898, s.ad_value(901), p.p1805, s.ad_value(897), 1.0);
            s.store_div_scaled_value_offset_denominator(172, s.ad_value(177), 1.0, A::sub(A::limited_exp(s.ad_value(177)), s.ad_value(177)), (-1.0), 1.0);
            s.store_mul(174, 177, 172);
            s.store_ln_neg_add(902, 901, 897);
        }

        if s.b[1429] {
            s.store_add_scaled_product_mixed_aia(344, A::add_scaled_inputs4(s.ad_value(924), 1.0, s.ad_value(901), (-1.0), {
                if (!((-s.v[901]) > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if ((-s.v[901]) > 1e-38) {
                            A::ln_scaled_input(s.ad_value(901), -1.0)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 1.0, {
                if (!(s.v[174] > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if (s.v[174] > 1e-38) {
                            A::ln(s.ad_value(174))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 1.0), 1.0, 914, A::exp_scaled_input(s.ad_value(902), 0.666666667), 1.0);
        }

        if s.b[1429] {
            s.store_add_scaled_offset_product_lhs_product_mixed_aaiia(345, A::offset(A::div_from_scalar(1.0, s.ad_value(901)), (-1.0)), 1.0, A::sub(A::div_from_scalar(2.0, s.ad_value(177)), s.ad_value(172)), (-1.0), 898, 1.0, 914, A::exp_scaled_input(s.ad_value(902), (-0.333333333)), (-0.666666667));
            s.store_add_scaled_product_mixed_aia(346, A::div_from_scalar((-1.0), A::square(s.ad_value(901))), 1.0, 914, A::exp_scaled_input(s.ad_value(902), (-1.333333333)), (-(2.0 / 9.0)));
            s.store_add_scaled_offset_product_rhs_mixed_iaa(901, 901, 1.0, A::div(s.ad_value(344), s.ad_value(345)), A::div_scaled_product_by_product(s.ad_value(344), s.ad_value(346), 1.0, s.ad_value(345), s.ad_value(345), 2.0), 1.0, (-1.0));
        }

        if (!s.b[1429]) {
            s.store_mul_neg_lhs(901, 901, 901);
        }

        s.store_mul_neg_lhs(393, 901, 181);

        s.b[1430] = (p.p57 == 1.0);
        s.store_scalar(1430, if s.b[1430] { 1.0 } else { 0.0 });

        if s.b[1430] {
            s.store_div_scaled_inputs2_indices(1015, 347, 1.0, 129, (-1.0), 181, 1.0);
            s.store_scaled_add_ad_rhs(1016, 1015, A::sqrt(A::add_scaled_square_product(s.ad_value(1015), 1.0, s.ad_value(963), s.ad_value(963), 0.25)), 0.5);
            s.store_pow_ad(1017, s.ad_value(1016), A::scale(s.ad_value(960), 0.5));
            s.store_mul_ad_product_rhs_mixed_ia(1007, 1010, 1017, A::limited_exp(A::sub(s.ad_value(1015), s.ad_value(1016))));
            s.store_div_scaled_inputs3_indices(1018, 347, 1.0, 129, (-1.0), 985, -1.0, 181, 1.0);
            s.store_scaled_add_ad_rhs(1019, 1018, A::sqrt(A::add_scaled_square_product(s.ad_value(1018), 1.0, s.ad_value(964), s.ad_value(964), 0.25)), 0.5);
            s.store_pow_ad(1020, s.ad_value(1019), A::scale(s.ad_value(961), 0.5));
            s.store_mul_ad_product_rhs_mixed_ia(1008, 1011, 1020, A::limited_exp(A::sub(s.ad_value(1018), s.ad_value(1019))));
            s.store_div_scaled_inputs3_indices(1021, 347, 1.0, 129, (-1.0), 986, -1.0, 181, 1.0);
            s.store_scaled_add_ad_rhs(1022, 1021, A::sqrt(A::add_scaled_square_product(s.ad_value(1021), 1.0, s.ad_value(965), s.ad_value(965), 0.25)), 0.5);
            s.store_pow_ad(1023, s.ad_value(1022), A::scale(s.ad_value(962), 0.5));
            s.store_mul_ad_product_rhs_mixed_ia(1009, 1012, 1023, A::limited_exp(A::sub(s.ad_value(1021), s.ad_value(1022))));
            s.store_add_scaled_products_right_right_ad(393, 983, 393, 1.0, 984, A::add_scaled_inputs3(s.ad_value(1007), 1.0, s.ad_value(1008), 1.0, s.ad_value(1009), 1.0), 1.0);
        }

        s.b[1431] = (p.p67 == 1.0);
        s.store_scalar(1431, if s.b[1431] { 1.0 } else { 0.0 });

        if s.b[1431] {
            s.store_add_ad(356, A::mul3_scaled_output(s.ad_value(297), s.ad_value(363), A::add_scaled_product(s.ad_value(127), 1.0, s.ad_value(674), A::sqrt(A::offset(s.ad_value(127), 0.01)), 1.0), -1.0), A::mul3(s.ad_value(681), s.ad_value(365), A::pow(A::offset(s.ad_value(127), 0.01), s.ad_value(682))));
            s.store_add_ad_lhs(359, A::add_scaled_inputs4(s.ad_value(354), 1.0, s.ad_value(356), 1.0, s.ad_value(357), 1.0, s.ad_value(231), 1.0), 805);
            s.store_add_scaled_inputs3_indices(349, 125, 1.0, 167, (-1.0), 359, -1.0);
            s.store_div_scaled_product3_indices(185, 414, 163, 158, 1.0, 153, 1.0);
        }

        s.b[1432] = (p.p80 == 0.0);
        s.store_scalar(1432, if s.b[1432] { 1.0 } else { 0.0 });

        if (s.b[1431] && s.b[1432]) {
            s.store_pow_ad(171, A::div_scaled_inputs(s.ad_value(163), (2.0 * p.p108), A::mul3_scaled_output(s.ad_value(185), s.ad_value(181), s.ad_value(148), (1.60219e-19 * p.p3)), 1.0), s.ad_value(181));
        }

        if (s.b[1431] && s.b[1432]) {
            s.store_neg_ad(168, A::add(s.ad_value(375), {
                if (!(s.v[171] > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if (s.v[171] > 1e-38) {
                            A::ln(s.ad_value(171))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }));
        }

        if (s.b[1431] && s.b[1432]) {
            s.store_offset_add(169, 349, 168, p.p23);
        }

        if (s.b[1431] && s.b[1432]) {
            s.store_sub_ad_lhs(350, {
                if (!(s.v[169] < ((-10000.0) * 0.0001))) {
                    A::add_scaled_inputs(s.ad_value(169), 0.5, A::sqrt_square_offset(s.ad_value(169), ((4.0 * 0.0001) * 0.0001)), 0.5)
                } else {
                    {
                        if (s.v[169] < ((-10000.0) * 0.0001)) {
                            A::div_from_scalar(((-0.0001) * 0.0001), s.ad_value(169))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 168);
        }

        if (s.b[1431] && (!s.b[1432])) {
            s.store_mul_scaled_ad_rhs(168, 181, -1.0, {
                if (!((((2.0 * s.v[163]) * p.p108) / ((((s.v[185] * s.v[181]) * 1.60219e-19) * s.v[148]) * p.p3)) > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if ((((2.0 * s.v[163]) * p.p108) / ((((s.v[185] * s.v[181]) * 1.60219e-19) * s.v[148]) * p.p3)) > 1e-38) {
                            A::ln(A::div_scaled_inputs(s.ad_value(163), (2.0 * p.p108), A::mul3_scaled_output(s.ad_value(185), s.ad_value(181), s.ad_value(148), (1.60219e-19 * p.p3)), 1.0))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            });
        }

        if (s.b[1431] && (!s.b[1432])) {
            s.store_sub_ad_lhs(169, A::add_scaled_inputs(A::offset(s.ad_value(168), 0.01), 0.5, A::sqrt_square_offset(A::offset(s.ad_value(168), (-0.01)), ((0.25 * 0.0001) * 0.0001)), 0.5), 375);
            s.store_offset_add(170, 349, 169, p.p23);
        }

        if (s.b[1431] && (!s.b[1432])) {
            s.store_sub_ad_lhs(350, {
                if (!(s.v[170] < ((-10000.0) * 0.0001))) {
                    A::add_scaled_inputs(s.ad_value(170), 0.5, A::sqrt_square_offset(s.ad_value(170), ((4.0 * 0.0001) * 0.0001)), 0.5)
                } else {
                    {
                        if (s.v[170] < ((-10000.0) * 0.0001)) {
                            A::div_from_scalar(((-0.0001) * 0.0001), s.ad_value(170))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 169);
        }

        if s.b[1431] {
            s.copy_ad(130, 375);
            s.store_powf_ad(172, A::neg(s.ad_value(897)), 0.666666667);
        }

        s.b[1433] = (p.p61 != 0.0);
        s.store_scalar(1433, if s.b[1433] { 1.0 } else { 0.0 });

        if (s.b[1431] && s.b[1433]) {
            if (!((((2.0 * s.v[166]) + s.v[130]) - s.v[367]) < ((-10000.0) * 0.1))) {
                s.store_add_scaled_inputs4_mixed_iiia(169, 166, (2.0 * 0.5), 130, 0.5, 367, (-0.5), A::sqrt_square_offset(A::add_scaled_inputs3(s.ad_value(166), 2.0, s.ad_value(130), 1.0, s.ad_value(367), -1.0), ((4.0 * 0.1) * 0.1)), 0.5);
            } else {
                if ((((2.0 * s.v[166]) + s.v[130]) - s.v[367]) < ((-10000.0) * 0.1)) {
                    s.store_div_from_scalar_ad(169, ((-0.1) * 0.1), A::add_scaled_inputs3(s.ad_value(166), 2.0, s.ad_value(130), 1.0, s.ad_value(367), -1.0));
                } else {
                    s.store_scalar(169, 0.0);
                }
            }
        }

        if (s.b[1431] && s.b[1433]) {
            s.store_mul_div_scaled_inputs_mixed_aii(171, A::sub(A::sqrt(s.ad_value(169)), A::sqrt_scaled_input(s.ad_value(166), 2.0)), 239, -1.0, 181, 2.0);
            s.store_add_scaled_product_value_ad(168, A::add_scaled_inputs3(s.ad_value(897), -1.0, s.ad_value(171), (-1.0), s.ad_value(899), 1.0), 1.0, 914, 172, 1.0);
            s.store_add_scaled_inputs3_indices(169, 897, -1.0, 171, (-1.0), 900, 1.0);
        }

        if (s.b[1431] && (!s.b[1433])) {
            s.store_add_scaled_inputs_product_indices(168, 899, 1.0, 897, (-1.0), 914, 172, 1.0);
            s.store_sub(169, 900, 897);
        }

        if s.b[1431] {
            s.store_div_scaled_inputs2_indices(170, 350, 1.0, 130, (-1.0), 181, 1.0);
            s.store_sub(924, 169, 170);
            s.store_scaled_sub(171, 170, 168, 0.5);
            s.store_limited_exp(901, 171);
        }

        s.b[1434] = (s.v[901] > 1e-7);
        s.store_scalar(1434, if s.b[1434] { 1.0 } else { 0.0 });

        if (s.b[1431] && s.b[1434]) {
            s.store_ln_offset_input(176, 901, 1.0);
            s.store_offset_scaled_ad(901, A::sqrt_square_offset(s.ad_value(176), 1.0), (-2.0), 2.0);
            s.store_mul_add_scaled_inputs_rhs(177, 898, s.ad_value(901), p.p1805, s.ad_value(897), 1.0);
            s.store_div_scaled_value_offset_denominator(172, s.ad_value(177), 1.0, A::sub(A::limited_exp(s.ad_value(177)), s.ad_value(177)), (-1.0), 1.0);
            s.store_mul(174, 177, 172);
            s.store_ln_neg_add(902, 901, 897);
        }

        if (s.b[1431] && s.b[1434]) {
            s.store_add_scaled_product_mixed_aia(344, A::add_scaled_inputs4(s.ad_value(924), 1.0, s.ad_value(901), (-1.0), {
                if (!((-s.v[901]) > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if ((-s.v[901]) > 1e-38) {
                            A::ln_scaled_input(s.ad_value(901), -1.0)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 1.0, {
                if (!(s.v[174] > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if (s.v[174] > 1e-38) {
                            A::ln(s.ad_value(174))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 1.0), 1.0, 914, A::exp_scaled_input(s.ad_value(902), 0.666666667), 1.0);
        }

        if (s.b[1431] && s.b[1434]) {
            s.store_add_scaled_offset_product_lhs_product_mixed_aaiia(345, A::offset(A::div_from_scalar(1.0, s.ad_value(901)), (-1.0)), 1.0, A::sub(A::div_from_scalar(2.0, s.ad_value(177)), s.ad_value(172)), (-1.0), 898, 1.0, 914, A::exp_scaled_input(s.ad_value(902), (-0.333333333)), (-0.666666667));
            s.store_add_scaled_product_mixed_aia(346, A::div_from_scalar((-1.0), A::square(s.ad_value(901))), 1.0, 914, A::exp_scaled_input(s.ad_value(902), (-1.333333333)), (-(2.0 / 9.0)));
            s.store_add_scaled_offset_product_rhs_mixed_iaa(901, 901, 1.0, A::div(s.ad_value(344), s.ad_value(345)), A::div_scaled_product_by_product(s.ad_value(344), s.ad_value(346), 1.0, s.ad_value(345), s.ad_value(345), 2.0), 1.0, (-1.0));
            s.store_mul_add_scaled_inputs_rhs(177, 898, s.ad_value(901), p.p1805, s.ad_value(897), 1.0);
            s.store_div_scaled_value_offset_denominator(172, s.ad_value(177), 1.0, A::sub(A::limited_exp(s.ad_value(177)), s.ad_value(177)), (-1.0), 1.0);
            s.store_mul(174, 177, 172);
            s.store_ln_neg_add(902, 901, 897);
        }

        if (s.b[1431] && s.b[1434]) {
            s.store_add_scaled_product_mixed_aia(344, A::add_scaled_inputs4(s.ad_value(924), 1.0, s.ad_value(901), (-1.0), {
                if (!((-s.v[901]) > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if ((-s.v[901]) > 1e-38) {
                            A::ln_scaled_input(s.ad_value(901), -1.0)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 1.0, {
                if (!(s.v[174] > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if (s.v[174] > 1e-38) {
                            A::ln(s.ad_value(174))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 1.0), 1.0, 914, A::exp_scaled_input(s.ad_value(902), 0.666666667), 1.0);
        }

        if (s.b[1431] && s.b[1434]) {
            s.store_add_scaled_offset_product_lhs_product_mixed_aaiia(345, A::offset(A::div_from_scalar(1.0, s.ad_value(901)), (-1.0)), 1.0, A::sub(A::div_from_scalar(2.0, s.ad_value(177)), s.ad_value(172)), (-1.0), 898, 1.0, 914, A::exp_scaled_input(s.ad_value(902), (-0.333333333)), (-0.666666667));
            s.store_add_scaled_product_mixed_aia(346, A::div_from_scalar((-1.0), A::square(s.ad_value(901))), 1.0, 914, A::exp_scaled_input(s.ad_value(902), (-1.333333333)), (-(2.0 / 9.0)));
            s.store_add_scaled_offset_product_rhs_mixed_iaa(901, 901, 1.0, A::div(s.ad_value(344), s.ad_value(345)), A::div_scaled_product_by_product(s.ad_value(344), s.ad_value(346), 1.0, s.ad_value(345), s.ad_value(345), 2.0), 1.0, (-1.0));
        }

        if (s.b[1431] && (!s.b[1434])) {
            s.store_mul_neg_lhs(901, 901, 901);
        }

        if s.b[1431] {
            s.store_mul_neg_lhs(394, 901, 181);
        }

        s.b[1435] = (p.p57 == 1.0);
        s.store_scalar(1435, if s.b[1435] { 1.0 } else { 0.0 });

        if (s.b[1431] && s.b[1435]) {
            s.store_div_scaled_inputs2_indices(1015, 349, 1.0, 130, (-1.0), 181, 1.0);
            s.store_scaled_add_ad_rhs(1016, 1015, A::sqrt(A::add_scaled_square_product(s.ad_value(1015), 1.0, s.ad_value(963), s.ad_value(963), 0.25)), 0.5);
            s.store_pow_ad(1017, s.ad_value(1016), A::scale(s.ad_value(960), 0.5));
            s.store_mul_ad_product_rhs_mixed_ia(1004, 1010, 1017, A::limited_exp(A::sub(s.ad_value(1015), s.ad_value(1016))));
            s.store_div_scaled_inputs3_indices(1018, 349, 1.0, 130, (-1.0), 985, -1.0, 181, 1.0);
            s.store_scaled_add_ad_rhs(1019, 1018, A::sqrt(A::add_scaled_square_product(s.ad_value(1018), 1.0, s.ad_value(964), s.ad_value(964), 0.25)), 0.5);
            s.store_pow_ad(1020, s.ad_value(1019), A::scale(s.ad_value(961), 0.5));
            s.store_mul_ad_product_rhs_mixed_ia(1005, 1011, 1020, A::limited_exp(A::sub(s.ad_value(1018), s.ad_value(1019))));
            s.store_div_scaled_inputs3_indices(1021, 349, 1.0, 130, (-1.0), 986, -1.0, 181, 1.0);
            s.store_scaled_add_ad_rhs(1022, 1021, A::sqrt(A::add_scaled_square_product(s.ad_value(1021), 1.0, s.ad_value(965), s.ad_value(965), 0.25)), 0.5);
            s.store_pow_ad(1023, s.ad_value(1022), A::scale(s.ad_value(962), 0.5));
            s.store_mul_ad_product_rhs_mixed_ia(1006, 1012, 1023, A::limited_exp(A::sub(s.ad_value(1021), s.ad_value(1022))));
            s.store_add_scaled_products_right_right_ad(394, 983, 394, 1.0, 984, A::add_scaled_inputs3(s.ad_value(1004), 1.0, s.ad_value(1005), 1.0, s.ad_value(1006), 1.0), 1.0);
        }

        if s.b[1431] {
            s.store_add_scaled_product_indices(421, 396, s.v[420], 407, 394, s.v[420]);
        }

    }

    pub(super) fn stamp_reactive_block_18(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if s.b[1431] {
            s.store_pow_ad(170, A::scaled_offset(A::div(s.ad_value(394), s.ad_value(406)), 1.0, 0.5), s.ad_value(317));
            s.store_pow_indices(171, 421, 822);
        }

        s.b[1436] = (p.p61 != 0.0);
        s.store_scalar(1436, if s.b[1436] { 1.0 } else { 0.0 });

        if (s.b[1431] && s.b[1436]) {
            s.store_add_scaled_product_mixed_aai(171, A::div(s.ad_value(319), s.ad_value(170)), 1.0, A::add_scaled_product(s.ad_value(304), 1.0, s.ad_value(315), s.ad_value(370), 1.0), 171, 1.0);
        }

        if (s.b[1431] && (!s.b[1436])) {
            s.store_add_scaled_product_value_ad(171, A::div(s.ad_value(319), s.ad_value(170)), 1.0, 304, 171, 1.0);
        }

        if s.b[1431] {
            s.store_offset(398, 171, 1.0);
            s.store_scaled_add_offset_sqrt_square_offset(398, 398, 1.0, (-1.0), ((0.25 * p.p604) * p.p604), 0.5);
            s.store_scale(398, 398, 1.0 / (p.p24));
        }

        s.b[1437] = (p.p64 == 1.0);
        s.store_scalar(1437, if s.b[1437] { 1.0 } else { 0.0 });

        if (s.b[1431] && s.b[1437]) {
            s.store_scalar(199, 0.0);
        }

        s.b[1438] = (p.p64 == 0.0);
        s.store_scalar(1438, if s.b[1438] { 1.0 } else { 0.0 });

        if ((s.b[1431] && (!s.b[1437])) && s.b[1438]) {
            s.store_offset_mul(172, 711, 394, 1.0);
            s.store_div_from_scalar(169, 1.0, 172);
            s.store_scaled_add_sqrt_square_offset_rhs(168, 169, 169, 0.01, 0.5);
            s.store_mul_ad_affine_product_lhs(199, A::offset(A::mul(s.ad_value(709), s.ad_value(168)), p.p908), s.ad_value(189), s.v[115], 0.0, 194);
        }

        if ((s.b[1431] && (!s.b[1437])) && (!s.b[1438])) {
            s.store_offset_mul(172, 711, 394, 1.0);
            s.store_div_from_scalar(169, 1.0, 172);
            s.store_scaled_add_sqrt_square_offset_rhs(168, 169, 169, 0.01, 0.5);
            s.store_mul_ad_lhs(199, A::add_scaled_inputs_product(s.ad_value(190), 1.0, s.ad_value(191), 1.0, A::offset(A::mul(s.ad_value(709), s.ad_value(168)), p.p908), s.ad_value(189), s.v[115]), 194);
        }

        if s.b[1431] {
            s.store_mul_div_scaled_inputs_indices(222, 398, 336, 2.0, 414, 1.0);
            s.store_mul(223, 222, 153);
        }

        s.b[1439] = (p.p80 == 0.0);
        s.store_scalar(1439, if s.b[1439] { 1.0 } else { 0.0 });

        if (s.b[1431] && s.b[1439]) {
            s.store_mul_add_scaled_inputs_rhs(175, 659, s.ad_value(394), 1.0, s.ad_value(179), 2.0);
        }

        if (s.b[1431] && (!s.b[1439])) {
            s.store_mul_add_scaled_inputs_rhs(175, 659, s.ad_value(394), 1.0, s.ad_value(182), 2.0);
        }

        s.b[1440] = (s.v[199] > 0.0);
        s.store_scalar(1440, if s.b[1440] { 1.0 } else { 0.0 });

        if (s.b[1431] && s.b[1440]) {
            s.store_mul_product3_indices(168, 199, 158, 336, 163, 1.0);
            s.store_scale(225, 168, 2.0);
            s.store_add_scaled_inputs_product_indices(226, 175, 1.0, 223, 1.0, 175, 168, 3.0);
            s.store_mul_add_scaled_product_rhs(227, 175, s.ad_value(223), 1.0, s.ad_value(175), s.ad_value(168), 2.0);
            s.store_div_scaled_inputs2(211, A::square(s.ad_value(226)), 1.0, A::add_scaled_square_product(s.ad_value(226), 1.0, s.ad_value(225), s.ad_value(227), (-2.0)), (-1.0), A::mul(A::add(s.ad_value(226), A::sqrt(A::add_scaled_square_product(s.ad_value(226), 1.0, s.ad_value(225), s.ad_value(227), (-2.0)))), s.ad_value(225)), 1.0);
        }

        if (s.b[1431] && (!s.b[1440])) {
            s.store_div_scaled_product_add_scaled_denominator_indices(211, 223, 175, 1.0, 223, 1.0, 175, 1.0, 1.0);
        }

        if s.b[1431] {
            s.store_offset_ad(211, {
                if (!((s.v[211] - 0.001) < ((-10000.0) * 1e-5))) {
                    A::add_scaled_inputs(A::offset(s.ad_value(211), (-0.001)), 0.5, A::sqrt_square_offset(A::offset(s.ad_value(211), (-0.001)), ((4.0 * 1e-5) * 1e-5)), 0.5)
                } else {
                    {
                        if ((s.v[211] - 0.001) < ((-10000.0) * 1e-5)) {
                            A::div_scalar_offset_denominator(((-1e-5) * 1e-5), s.ad_value(211), (-0.001), 1.0)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 0.001);
        }

        if s.b[1431] {
            s.store_pow_ad(176, A::offset(A::div(s.ad_value(126), s.ad_value(211)), 1e-6), s.ad_value(423));
            s.store_pow_ad(177, A::offset(s.ad_value(176), 1.0), s.ad_value(212));
            s.store_min_ad(391, A::div(s.ad_value(126), s.ad_value(177)), s.ad_value(126));
            s.store_add(130, 391, 375);
            s.store_powf_ad(170, A::neg(s.ad_value(897)), 0.666666667);
        }

        s.b[1441] = (p.p61 != 0.0);
        s.store_scalar(1441, if s.b[1441] { 1.0 } else { 0.0 });

        if (s.b[1431] && s.b[1441]) {
            if (!((((2.0 * s.v[166]) + s.v[130]) - s.v[367]) < ((-10000.0) * 0.1))) {
                s.store_add_scaled_inputs4_mixed_iiia(169, 166, (2.0 * 0.5), 130, 0.5, 367, (-0.5), A::sqrt_square_offset(A::add_scaled_inputs3(s.ad_value(166), 2.0, s.ad_value(130), 1.0, s.ad_value(367), -1.0), ((4.0 * 0.1) * 0.1)), 0.5);
            } else {
                if ((((2.0 * s.v[166]) + s.v[130]) - s.v[367]) < ((-10000.0) * 0.1)) {
                    s.store_div_from_scalar_ad(169, ((-0.1) * 0.1), A::add_scaled_inputs3(s.ad_value(166), 2.0, s.ad_value(130), 1.0, s.ad_value(367), -1.0));
                } else {
                    s.store_scalar(169, 0.0);
                }
            }
        }

        if (s.b[1431] && s.b[1441]) {
            s.store_mul_div_scaled_inputs_mixed_aii(171, A::sub(A::sqrt(s.ad_value(169)), A::sqrt_scaled_input(s.ad_value(166), 2.0)), 239, -1.0, 181, 2.0);
            s.store_add_scaled_product_value_ad(168, A::add_scaled_inputs3(s.ad_value(897), -1.0, s.ad_value(171), (-1.0), s.ad_value(899), 1.0), 1.0, 914, 170, 1.0);
            s.store_add_scaled_inputs3_indices(169, 897, -1.0, 171, (-1.0), 900, 1.0);
        }

        if (s.b[1431] && (!s.b[1441])) {
            s.store_add_scaled_inputs_product_indices(168, 899, 1.0, 897, (-1.0), 914, 170, 1.0);
            s.store_sub(169, 900, 897);
        }

        if s.b[1431] {
            s.store_div_scaled_inputs2_indices(170, 350, 1.0, 130, (-1.0), 181, 1.0);
            s.store_sub(924, 169, 170);
            s.store_scaled_sub(171, 170, 168, 0.5);
            s.store_limited_exp(901, 171);
        }

        s.b[1442] = (s.v[901] > 1e-7);
        s.store_scalar(1442, if s.b[1442] { 1.0 } else { 0.0 });

        if (s.b[1431] && s.b[1442]) {
            s.store_ln_offset_input(176, 901, 1.0);
            s.store_offset_scaled_ad(901, A::sqrt_square_offset(s.ad_value(176), 1.0), (-2.0), 2.0);
            s.store_mul_add_scaled_inputs_rhs(177, 898, s.ad_value(901), p.p1805, s.ad_value(897), 1.0);
            s.store_div_scaled_value_offset_denominator(172, s.ad_value(177), 1.0, A::sub(A::limited_exp(s.ad_value(177)), s.ad_value(177)), (-1.0), 1.0);
            s.store_mul(174, 177, 172);
            s.store_ln_neg_add(902, 901, 897);
        }

        if (s.b[1431] && s.b[1442]) {
            s.store_add_scaled_product_mixed_aia(344, A::add_scaled_inputs4(s.ad_value(924), 1.0, s.ad_value(901), (-1.0), {
                if (!((-s.v[901]) > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if ((-s.v[901]) > 1e-38) {
                            A::ln_scaled_input(s.ad_value(901), -1.0)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 1.0, {
                if (!(s.v[174] > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if (s.v[174] > 1e-38) {
                            A::ln(s.ad_value(174))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 1.0), 1.0, 914, A::exp_scaled_input(s.ad_value(902), 0.666666667), 1.0);
        }

        if (s.b[1431] && s.b[1442]) {
            s.store_add_scaled_offset_product_lhs_product_mixed_aaiia(345, A::offset(A::div_from_scalar(1.0, s.ad_value(901)), (-1.0)), 1.0, A::sub(A::div_from_scalar(2.0, s.ad_value(177)), s.ad_value(172)), (-1.0), 898, 1.0, 914, A::exp_scaled_input(s.ad_value(902), (-0.333333333)), (-0.666666667));
            s.store_add_scaled_product_mixed_aia(346, A::div_from_scalar((-1.0), A::square(s.ad_value(901))), 1.0, 914, A::exp_scaled_input(s.ad_value(902), (-1.333333333)), (-(2.0 / 9.0)));
            s.store_add_scaled_offset_product_rhs_mixed_iaa(901, 901, 1.0, A::div(s.ad_value(344), s.ad_value(345)), A::div_scaled_product_by_product(s.ad_value(344), s.ad_value(346), 1.0, s.ad_value(345), s.ad_value(345), 2.0), 1.0, (-1.0));
            s.store_mul_add_scaled_inputs_rhs(177, 898, s.ad_value(901), p.p1805, s.ad_value(897), 1.0);
            s.store_div_scaled_value_offset_denominator(172, s.ad_value(177), 1.0, A::sub(A::limited_exp(s.ad_value(177)), s.ad_value(177)), (-1.0), 1.0);
            s.store_mul(174, 177, 172);
            s.store_ln_neg_add(902, 901, 897);
        }

        if (s.b[1431] && s.b[1442]) {
            s.store_add_scaled_product_mixed_aia(344, A::add_scaled_inputs4(s.ad_value(924), 1.0, s.ad_value(901), (-1.0), {
                if (!((-s.v[901]) > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if ((-s.v[901]) > 1e-38) {
                            A::ln_scaled_input(s.ad_value(901), -1.0)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 1.0, {
                if (!(s.v[174] > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if (s.v[174] > 1e-38) {
                            A::ln(s.ad_value(174))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 1.0), 1.0, 914, A::exp_scaled_input(s.ad_value(902), 0.666666667), 1.0);
        }

        if (s.b[1431] && s.b[1442]) {
            s.store_add_scaled_offset_product_lhs_product_mixed_aaiia(345, A::offset(A::div_from_scalar(1.0, s.ad_value(901)), (-1.0)), 1.0, A::sub(A::div_from_scalar(2.0, s.ad_value(177)), s.ad_value(172)), (-1.0), 898, 1.0, 914, A::exp_scaled_input(s.ad_value(902), (-0.333333333)), (-0.666666667));
            s.store_add_scaled_product_mixed_aia(346, A::div_from_scalar((-1.0), A::square(s.ad_value(901))), 1.0, 914, A::exp_scaled_input(s.ad_value(902), (-1.333333333)), (-(2.0 / 9.0)));
            s.store_add_scaled_offset_product_rhs_mixed_iaa(901, 901, 1.0, A::div(s.ad_value(344), s.ad_value(345)), A::div_scaled_product_by_product(s.ad_value(344), s.ad_value(346), 1.0, s.ad_value(345), s.ad_value(345), 2.0), 1.0, (-1.0));
        }

        if (s.b[1431] && (!s.b[1442])) {
            s.store_mul_neg_lhs(901, 901, 901);
        }

        if s.b[1431] {
            s.store_mul_neg_lhs(395, 901, 181);
        }

        s.b[1443] = (p.p57 == 1.0);
        s.store_scalar(1443, if s.b[1443] { 1.0 } else { 0.0 });

        if (s.b[1431] && s.b[1443]) {
            s.store_div_scaled_inputs2_indices(1015, 349, 1.0, 130, (-1.0), 181, 1.0);
            s.store_scaled_add_ad_rhs(1016, 1015, A::sqrt(A::add_scaled_square_product(s.ad_value(1015), 1.0, s.ad_value(963), s.ad_value(963), 0.25)), 0.5);
            s.store_pow_ad(1017, s.ad_value(1016), A::scale(s.ad_value(960), 0.5));
            s.store_mul_ad_product_rhs_mixed_ia(1007, 1010, 1017, A::limited_exp(A::sub(s.ad_value(1015), s.ad_value(1016))));
            s.store_div_scaled_inputs3_indices(1018, 349, 1.0, 130, (-1.0), 985, -1.0, 181, 1.0);
            s.store_scaled_add_ad_rhs(1019, 1018, A::sqrt(A::add_scaled_square_product(s.ad_value(1018), 1.0, s.ad_value(964), s.ad_value(964), 0.25)), 0.5);
            s.store_pow_ad(1020, s.ad_value(1019), A::scale(s.ad_value(961), 0.5));
            s.store_mul_ad_product_rhs_mixed_ia(1008, 1011, 1020, A::limited_exp(A::sub(s.ad_value(1018), s.ad_value(1019))));
            s.store_div_scaled_inputs3_indices(1021, 349, 1.0, 130, (-1.0), 986, -1.0, 181, 1.0);
            s.store_scaled_add_ad_rhs(1022, 1021, A::sqrt(A::add_scaled_square_product(s.ad_value(1021), 1.0, s.ad_value(965), s.ad_value(965), 0.25)), 0.5);
            s.store_pow_ad(1023, s.ad_value(1022), A::scale(s.ad_value(962), 0.5));
            s.store_mul_ad_product_rhs_mixed_ia(1009, 1012, 1023, A::limited_exp(A::sub(s.ad_value(1021), s.ad_value(1022))));
            s.store_add_scaled_products_right_right_ad(395, 983, 395, 1.0, 984, A::add_scaled_inputs3(s.ad_value(1007), 1.0, s.ad_value(1008), 1.0, s.ad_value(1009), 1.0), 1.0);
        }

        if s.b[1431] {
            s.store_scaled_add(403, 394, 395, 0.5);
            s.store_sub(405, 394, 395);
            s.store_scaled_square(168, 391, 1600.0);
        }

        s.b[1444] = (p.p603 != 0.0);
        s.store_scalar(1444, if s.b[1444] { 1.0 } else { 0.0 });

        if (s.b[1431] && s.b[1444]) {
            s.store_add_scaled_inputs3_mixed_iia(404, 394, 0.5, 395, 0.5, A::mul_sub_from_scalar_lhs_scaled_output(1.0, A::limited_exp_scaled_input(s.ad_value(168), -1.0), s.ad_value(405), (p.p603 * 0.5)), 1.0);
        }

        if (s.b[1431] && (!s.b[1444])) {
            s.store_scaled_add(404, 394, 395, 0.5);
        }

        s.b[1445] = (p.p61 != 0.0);
        s.store_scalar(1445, if s.b[1445] { 1.0 } else { 0.0 });

        if s.b[1445] {
            s.store_mul_div_scaled_inputs_mixed_aii(178, A::sqrt(s.ad_value(179)), 239, 1.0, 181, 2.0);
            s.store_scale(168, 178, 0.5);
        }

        if s.b[1445] {
            s.store_div_scaled_inputs2_mixed_iai(170, 497, 1.0, A::offset(A::add_scaled_inputs_product(s.ad_value(167), 1.0, s.ad_value(146), (-1.0), s.ad_value(179), {
                if (!((s.v[640] / s.v[148]) > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if ((s.v[640] / s.v[148]) > 1e-38) {
                            A::ln(A::div(s.ad_value(640), s.ad_value(148)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, (-1.0)), p.p1529), (-1.0), 179, 1.0);
        }

        s.b[1446] = ((s.v[170] * s.v[179]) > (s.v[166] + (s.v[178] * (((s.v[166] * s.v[179])) as f64).sqrt())));
        s.store_scalar(1446, if s.b[1446] { 1.0 } else { 0.0 });

        if (s.b[1445] && s.b[1446]) {
            s.store_sub_ad_lhs(169, A::sqrt(A::add(A::offset(s.ad_value(170), (-1.0)), A::square(s.ad_value(168)))), 168);
            s.store_offset_square(340, 169, 1.0);
        }

        if (s.b[1445] && s.b[1446]) {
            if (!((((-s.v[340])) as f64).abs() < 1e-7)) {
                s.store_offset_ad(175, A::limited_exp_scaled_input(s.ad_value(340), -1.0), (-1.0));
            } else {
                if ((((-s.v[340])) as f64).abs() < 1e-7) {
                    s.store_sub_ad_lhs(175, A::mul_scaled_lhs(s.ad_value(340), (-(-0.5)), s.ad_value(340)), 340);
                } else {
                    s.store_scalar(175, 0.0);
                }
            }
        }

        if (s.b[1445] && (!s.b[1446])) {
            s.store_sub_scaled_ad_rhs(171, 170, 0.5, A::scale_offset(s.ad_value(178), ((1.0 / (((2.0) as f64).sqrt())) * (3.0)), 3.0));
            s.store_add_ad_rhs(340, 171, A::sqrt(A::add_scaled_inputs(A::square(s.ad_value(171)), 1.0, s.ad_value(170), 6.0)));
        }

        s.b[1447] = (s.v[170] < 0.0);
        s.store_scalar(1447, if s.b[1447] { 1.0 } else { 0.0 });

        if ((s.b[1445] && (!s.b[1446])) && s.b[1447]) {
            s.store_div_scaled_inputs2_indices(172, 170, 1.0, 340, (-1.0), 178, 1.0);
            s.store_sub_square_lhs(175, 172, 340);
        }

        if ((s.b[1445] && (!s.b[1446])) && s.b[1447]) {
            s.store_neg_ad(340, {
                if (!(((1.0 - s.v[340]) + (s.v[172] * s.v[172])) > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if (((1.0 - s.v[340]) + (s.v[172] * s.v[172])) > 1e-38) {
                            A::ln(A::add(A::sub_from_scalar(1.0, s.ad_value(340)), A::square(s.ad_value(172))))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            });
        }

        if ((s.b[1445] && (!s.b[1446])) && (!s.b[1447])) {
            s.store_limited_exp_neg_input(341, 340);
        }

    }

    pub(super) fn stamp_reactive_block_19(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((s.b[1445] && (!s.b[1446])) && (!s.b[1447])) {
            s.store_sub_ad_lhs(172, A::sqrt(A::add_scaled_inputs3_offset(s.ad_value(170), 1.0, s.ad_value(341), 1.0, A::square(s.ad_value(168)), 1.0, (-1.0))), 168);
            s.store_add_ad(340, A::sub_from_scalar(1.0, s.ad_value(341)), A::square(s.ad_value(172)));
        }

        if ((s.b[1445] && (!s.b[1446])) && (!s.b[1447])) {
            if (!((((-s.v[340])) as f64).abs() < 1e-7)) {
                s.store_offset_ad(175, A::limited_exp_scaled_input(s.ad_value(340), -1.0), (-1.0));
            } else {
                if ((((-s.v[340])) as f64).abs() < 1e-7) {
                    s.store_sub_ad_lhs(175, A::mul_scaled_lhs(s.ad_value(340), (-(-0.5)), s.ad_value(340)), 340);
                } else {
                    s.store_scalar(175, 0.0);
                }
            }
        }

        if s.b[1445] {
            s.store_sqrt_add(176, 175, 340);
        }

        s.b[1448] = (s.v[340] > 1e-15);
        s.store_scalar(1448, if s.b[1448] { 1.0 } else { 0.0 });

        if (s.b[1445] && s.b[1448]) {
            s.store_add_scaled_inputs_product_indices(344, 170, -1.0, 340, 1.0, 178, 176, 1.0);
            s.store_sub_from_scalar_ad(345, 1.0, A::div_scaled_product(s.ad_value(178), s.ad_value(175), 0.5, s.ad_value(176), 1.0));
            s.store_sub_div_rhs_indices(177, 340, 344, 345);
        }

        if (s.b[1445] && s.b[1448]) {
            if (!((((-s.v[177])) as f64).abs() < 1e-7)) {
                s.store_offset_ad(341, A::limited_exp_scaled_input(s.ad_value(177), -1.0), (-1.0));
            } else {
                if ((((-s.v[177])) as f64).abs() < 1e-7) {
                    s.store_sub_ad_lhs(341, A::mul_scaled_lhs(s.ad_value(177), (-(-0.5)), s.ad_value(177)), 177);
                } else {
                    s.store_scalar(341, 0.0);
                }
            }
        }

        if (s.b[1445] && s.b[1448]) {
            s.store_sqrt_add(342, 341, 177);
            s.store_mul3_affine_lhs(401, 178, 342, -1.0, 0.0, 179);
        }

        s.b[1449] = (s.v[340] < (-1e-15));
        s.store_scalar(1449, if s.b[1449] { 1.0 } else { 0.0 });

        if ((s.b[1445] && (!s.b[1448])) && s.b[1449]) {
            s.store_add_scaled_inputs_product_indices(344, 170, -1.0, 340, 1.0, 178, 176, (-1.0));
            s.store_offset_div_scaled_product(345, s.ad_value(178), s.ad_value(175), 0.5, s.ad_value(176), 1.0, 1.0);
            s.store_sub_div_rhs_indices(177, 340, 344, 345);
        }

        if ((s.b[1445] && (!s.b[1448])) && s.b[1449]) {
            s.store_add_ad_lhs(343, {
                if (!((((-s.v[177])) as f64).abs() < 1e-7)) {
                    A::offset(A::limited_exp_scaled_input(s.ad_value(177), -1.0), (-1.0))
                } else {
                    {
                        if ((((-s.v[177])) as f64).abs() < 1e-7) {
                            A::sub(A::mul_scaled_lhs(s.ad_value(177), (-(-0.5)), s.ad_value(177)), s.ad_value(177))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 177);
        }

        if ((s.b[1445] && (!s.b[1448])) && s.b[1449]) {
            s.store_mul_sqrt_rhs(342, 178, 343);
        }

        if ((s.b[1445] && (!s.b[1448])) && (!s.b[1449])) {
            s.store_scalar(177, 0.0);
            s.store_scalar(342, 0.0);
        }

        if (s.b[1445] && (!s.b[1448])) {
            s.store_mul(401, 342, 179);
        }

        if s.b[1445] {
            s.store_mul_ad_product_lhs_mixed_ia(904, 178, A::limited_exp_scaled_input(s.ad_value(177), (-1.0 / (2.0))), 179);
            s.store_scaled_add_offset_sqrt_square_offset(921, 177, 1.0, (-1.0), ((0.25 * 2.0) * 2.0), 0.5);
            s.store_sqrt(922, 921);
            s.store_offset_div(923, 178, 922, 1.0);
        }

        s.store_scaled_add(399, 392, 393, 0.5);

        s.store_sub(402, 392, 393);

        s.store_scaled_square(168, 390, 1600.0);

        s.b[1450] = (p.p603 != 0.0);
        s.store_scalar(1450, if s.b[1450] { 1.0 } else { 0.0 });

        if s.b[1450] {
            s.store_add_scaled_inputs3_mixed_iia(400, 392, 0.5, 393, 0.5, A::mul_sub_from_scalar_lhs_scaled_output(1.0, A::limited_exp_scaled_input(s.ad_value(168), -1.0), s.ad_value(402), (p.p603 * 0.5)), 1.0);
        }

        if (!s.b[1450]) {
            s.store_scaled_add(400, 392, 393, 0.5);
        }

        s.b[1451] = (s.v[655] > 0.0);
        s.store_scalar(1451, if s.b[1451] { 1.0 } else { 0.0 });

        if s.b[1451] {
            s.store_scale(172, 399, 1.0 / (p.p400));
            s.store_offset_pow_ad(174, s.ad_value(172), s.ad_value(661), 1.0);
            s.store_div(374, 373, 174);
            s.store_div_from_scalar_ad(372, 1.0, A::add_scaled_product(A::div_from_scalar(1.0, A::scale(s.ad_value(163), (p.p89 * 1.0 / (p.p90)))), 1.0, s.ad_value(374), s.ad_value(655), 1.0 / (s.v[143])));
        }

        if (!s.b[1451]) {
            s.copy_ad(372, 163);
        }

        s.b[1452] = ((p.p61 != 0.0) && (s.v[656] != 0.0));
        s.store_scalar(1452, if s.b[1452] { 1.0 } else { 0.0 });

        if s.b[1452] {
            s.store_offset_powf_ad(175, A::scale(s.ad_value(904), 1.0 / (p.p401)), p.p402, 1.0);
            s.store_div(374, 373, 175);
            s.store_div_from_scalar_ad(494, 1.0, A::add_scaled_product(A::div_from_scalar(1.0, s.ad_value(494)), 1.0, s.ad_value(374), s.ad_value(656), 1.0 / (s.v[143])));
        }

        s.store_div_scaled_product3_indices(183, 416, 163, 158, 1.0, 153, 1.0);

        s.store_add_scaled_product_indices(409, 396, s.v[420], 407, 400, s.v[420]);

        s.b[1453] = (p.p80 == 0.0);
        s.store_scalar(1453, if s.b[1453] { 1.0 } else { 0.0 });

        if s.b[1453] {
            s.store_pow_ad(170, A::scaled_offset(A::div(s.ad_value(400), s.ad_value(406)), 1.0, 0.5), s.ad_value(317));
        }

        if (!s.b[1453]) {
            s.store_scaled_square(168, 390, 1600.0);
            s.store_sub_from_scalar_ad(169, 1.0, A::limited_exp_scaled_input(s.ad_value(168), -1.0));
            s.store_mul_ad_lhs(168, A::add_scaled_products(s.ad_value(330), s.ad_value(392), 1.0, s.ad_value(331), s.ad_value(393), 1.0), 169);
        }

        if (!s.b[1453]) {
            if (!(s.v[168] < ((-10000.0) * 1e-12))) {
                s.store_scaled_add_sqrt_square_offset_rhs(169, 168, 168, ((4.0 * 1e-12) * 1e-12), 0.5);
            } else {
                if (s.v[168] < ((-10000.0) * 1e-12)) {
                    s.store_div_from_scalar(169, ((-1e-12) * 1e-12), 168);
                } else {
                    s.store_scalar(169, 0.0);
                }
            }
        }

        if (!s.b[1453]) {
            s.store_pow_ad(170, A::scaled_offset(A::div(s.ad_value(169), s.ad_value(406)), 1.0, 0.5), s.ad_value(317));
        }

        s.store_pow_indices(168, 409, 822);

        s.b[1454] = (p.p61 != 0.0);
        s.store_scalar(1454, if s.b[1454] { 1.0 } else { 0.0 });

        if s.b[1454] {
            s.store_add_scaled_product_mixed_aai(171, A::div(s.ad_value(820), s.ad_value(170)), 1.0, A::add_scaled_product(s.ad_value(819), 1.0, s.ad_value(821), s.ad_value(370), 1.0), 168, 1.0);
        }

        if (!s.b[1454]) {
            s.store_add_scaled_product_value_ad(171, A::div(s.ad_value(820), s.ad_value(170)), 1.0, 819, 168, 1.0);
        }

        s.store_offset(411, 171, 1.0);

        s.store_scaled_add_offset_sqrt_square_offset(411, 411, 1.0, (-1.0), ((0.25 * p.p604) * p.p604), 0.5);

        s.store_offset_scaled_ad(215, A::scale(A::limited_exp_scaled_input(s.ad_value(390), (-p.p888)), p.p887), (-p.p24), p.p24);

        s.store_div(411, 411, 215);

        s.store_div(415, 416, 411);

        s.b[1455] = (p.p67 == 1.0);
        s.store_scalar(1455, if s.b[1455] { 1.0 } else { 0.0 });

        s.b[1456] = (p.p80 == 0.0);
        s.store_scalar(1456, if s.b[1456] { 1.0 } else { 0.0 });

        if (s.b[1455] && s.b[1456]) {
            s.store_pow_ad(170, A::scaled_offset(A::div(s.ad_value(404), s.ad_value(406)), 1.0, 0.5), s.ad_value(317));
        }

        if (s.b[1455] && (!s.b[1456])) {
            s.store_add_scaled_products_indices(168, 330, 394, 1.0, 331, 395, 1.0);
        }

        if (s.b[1455] && (!s.b[1456])) {
            if (!(s.v[168] < ((-10000.0) * 1e-12))) {
                s.store_scaled_add_sqrt_square_offset_rhs(169, 168, 168, ((4.0 * 1e-12) * 1e-12), 0.5);
            } else {
                if (s.v[168] < ((-10000.0) * 1e-12)) {
                    s.store_div_from_scalar(169, ((-1e-12) * 1e-12), 168);
                } else {
                    s.store_scalar(169, 0.0);
                }
            }
        }

        if (s.b[1455] && (!s.b[1456])) {
            s.store_pow_ad(170, A::scaled_offset(A::div(s.ad_value(169), s.ad_value(406)), 1.0, 0.5), s.ad_value(317));
        }

        if s.b[1455] {
            s.store_add_scaled_product_indices(410, 396, s.v[420], 408, 404, s.v[420]);
            s.store_add_scaled_product_mixed_aia(171, A::div(s.ad_value(319), s.ad_value(170)), 1.0, 304, A::pow(s.ad_value(410), s.ad_value(822)), 1.0);
        }

        if (!s.b[1455]) {
            s.store_add_scaled_product_indices(410, 396, s.v[420], 408, 400, s.v[420]);
            s.store_add_scaled_product_mixed_aia(171, A::div(s.ad_value(820), s.ad_value(170)), 1.0, 819, A::pow(s.ad_value(410), s.ad_value(822)), 1.0);
        }

        s.store_offset(412, 171, 1.0);

        s.store_scaled_add_offset_sqrt_square_offset(412, 412, 1.0, (-1.0), ((0.25 * p.p604) * p.p604), 0.5);

        s.store_div(412, 412, 215);

        s.store_offset_div_scaled_product(360, s.ad_value(719), s.ad_value(153), 1.0, s.ad_value(351), 1.0, 1e-6);

        s.b[1457] = (s.v[360] < 40.0);
        s.store_scalar(1457, if s.b[1457] { 1.0 } else { 0.0 });

        if s.b[1457] {
            s.store_add_ad_lhs(200, A::div_scaled_value_offset_denominator(s.ad_value(427), 0.5, A::cosh(s.ad_value(360)), (-1.0), 1.0), 718);
        }

        if (!s.b[1457]) {
            s.store_add_scaled_product_right_ad(200, 718, 1.0, 427, A::limited_exp_scaled_input(s.ad_value(360), -1.0), 1.0);
        }

        s.b[1458] = (s.v[720] > 0.0);
        s.store_scalar(1458, if s.b[1458] { 1.0 } else { 0.0 });

        if s.b[1458] {
            s.store_offset_div_scaled_product(201, s.ad_value(720), s.ad_value(399), 1.0, s.ad_value(217), 1.0, 1.0);
        }

        if (!s.b[1458]) {
            s.store_div_from_scalar_sub_from_scalar_ad(201, 1.0, 1.0, A::div_scaled_product(s.ad_value(720), s.ad_value(399), 1.0, s.ad_value(217), 1.0));
        }

        s.store_sub(202, 126, 390);

        s.b[1459] = (p.p80 == 0.0);
        s.store_scalar(1459, if s.b[1459] { 1.0 } else { 0.0 });

        if s.b[1459] {
            s.store_add_scaled_inputs(204, 399, 1.0, 179, 2.0);
        }

        if (!s.b[1459]) {
            s.store_add_scaled_inputs(204, 399, 1.0, 182, 2.0);
        }

        s.b[1460] = (s.v[200] > 0.0);
        s.store_scalar(1460, if s.b[1460] { 1.0 } else { 0.0 });

        if s.b[1460] {
            s.copy_ad(169, 204);
            s.store_div_add_scaled_inputs_rhs_indices(171, 169, 210, 1.0, 169, 1.0);
            s.store_mul_ad_product_lhs_mixed_ai(203, A::div(s.ad_value(169), s.ad_value(200)), 171, 201);
            s.store_offset_div(205, 202, 203, 1.0);
        }

        if (!s.b[1460]) {
            s.store_scalar(205, 1.0);
        }

        s.b[1461] = (s.v[795] > 0.0);
        s.store_scalar(1461, if s.b[1461] { 1.0 } else { 0.0 });

        s.b[1462] = (s.v[793] < 0.0);
        s.store_scalar(1462, if s.b[1462] { 1.0 } else { 0.0 });

        if (s.b[1461] && s.b[1462]) {
            s.store_div_from_scalar_ad(169, 1.0, A::add_scaled_product(A::div_from_scalar(1.0, s.ad_value(795)), 1.0, s.ad_value(793), s.ad_value(399), (-1.0)));
        }

        if (s.b[1461] && (!s.b[1462])) {
            s.store_add_scaled_product_indices(169, 795, 1.0, 793, 399, 1.0);
        }

        if s.b[1461] {
            s.store_offset_mul_ad(206, s.ad_value(169), {
                if (!((1.0 + (((s.v[126] - s.v[390]) / s.v[169]) / (s.v[210] + s.v[217]))) > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if ((1.0 + (((s.v[126] - s.v[390]) / s.v[169]) / (s.v[210] + s.v[217]))) > 1e-38) {
                            A::ln(A::offset(A::div_scaled_inputs2_by_product(s.ad_value(126), 1.0, s.ad_value(390), (-1.0), s.ad_value(169), A::add(s.ad_value(210), s.ad_value(217)), 1.0), 1.0))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 1.0);
        }

        if (!s.b[1461]) {
            s.store_scalar(206, 1.0);
        }

        s.store_mul(205, 205, 206);

        s.store_div_scaled_inputs_indices(218, 422, 2.0, 415, 1.0);

        s.store_mul(219, 218, 153);

        s.store_limited_exp_ad(168, A::mul(s.ad_value(695), {
            if (!((s.v[402] / s.v[219]) > 1e-38)) {
                A::neg(A::constant(87.498233534))
            } else {
                {
                    if ((s.v[402] / s.v[219]) > 1e-38) {
                        A::ln(A::div(s.ad_value(402), s.ad_value(219)))
                    } else {
                        A::constant(0.0)
                    }
                }
            }
        }));

        s.store_div_from_scalar(169, 1.0, 695);

        s.store_offset_limited_exp_ad(225, A::mul(s.ad_value(169), {
            if (!(s.v[694] > 1e-38)) {
                A::neg(A::constant(87.498233534))
            } else {
                {
                    if (s.v[694] > 1e-38) {
                        A::ln(s.ad_value(694))
                    } else {
                        A::constant(0.0)
                    }
                }
            }
        }), 1.0);

        s.store_div_scaled_offset_numerator(209, A::limited_exp(A::mul(s.ad_value(169), {
            if (!((s.v[694] + s.v[168]) > 1e-38)) {
                A::neg(A::constant(87.498233534))
            } else {
                {
                    if ((s.v[694] + s.v[168]) > 1e-38) {
                        A::ln(A::add(s.ad_value(694), s.ad_value(168)))
                    } else {
                        A::constant(0.0)
                    }
                }
            }
        })), 1.0, 1.0, s.ad_value(225), 1.0);

        s.store_add_scaled_product_left_ad(209, 209, 1.0, A::mul3_scaled_output(s.ad_value(424), s.ad_value(399), s.ad_value(402), 0.5), 402, 1.0);

        s.store_add_div_rhs_mixed_ia(168, 241, 242, A::add_scaled_inputs(s.ad_value(399), 1.0, s.ad_value(181), 2.0));

        s.store_mul3_lhs(169, 168, 402, 402);

        s.store_offset(170, 169, ((1.0) + ((-0.001))));

        s.store_offset_add_scaled_inputs_mixed_ia(171, 170, 0.5, A::sqrt_square_offset(s.ad_value(170), 0.004), 0.5, (-1.0));

        s.store_scaled_offset_ad(214, A::sqrt(A::offset(s.ad_value(171), 1.0)), 1.0, 0.5);

        s.store_mul(209, 209, 214);

        s.store_scaled_add_offset_sqrt_square_offset(209, 209, 1.0, (-1.0), ((0.25 * p.p453) * p.p453), 0.5);

        s.store_div_ad_rhs(169, 236, A::add_scaled_product(s.ad_value(181), 2.0, A::max_from_scalar(0.0, A::add(s.ad_value(237), A::mul3(s.ad_value(294), s.ad_value(402), s.ad_value(402)))), s.ad_value(399), 1.0));

        s.store_limited_exp_neg_input(366, 169);

        s.b[1463] = (p.p61 == 2.0);
        s.store_scalar(1463, if s.b[1463] { 1.0 } else { 0.0 });

        if s.b[1463] {
            if (!((s.v[293] + (s.v[240] * s.v[127])) < ((-10000.0) * 1e-6))) {
                s.store_scaled_add_sqrt_square_offset_ad(168, A::add_scaled_product(s.ad_value(293), 1.0, s.ad_value(240), s.ad_value(127), 1.0), ((4.0 * 1e-6) * 1e-6), 0.5);
            } else {
                if ((s.v[293] + (s.v[240] * s.v[127])) < ((-10000.0) * 1e-6)) {
                    s.store_div_from_scalar_ad(168, ((-1e-6) * 1e-6), A::add_scaled_product(s.ad_value(293), 1.0, s.ad_value(240), s.ad_value(127), 1.0));
                } else {
                    s.store_scalar(168, 0.0);
                }
            }
        }

        if s.b[1463] {
            s.store_div_ad_rhs(169, 168, A::add_scaled_product(s.ad_value(181), 2.0, A::max_from_scalar(0.0, A::add(s.ad_value(238), A::mul3(s.ad_value(295), s.ad_value(402), s.ad_value(402)))), s.ad_value(399), 1.0));
            s.store_sub_ad(171, A::sqrt(A::sub(s.ad_value(689), s.ad_value(370))), A::sqrt(s.ad_value(689)));
            s.store_limited_exp_ad(371, A::mul_scaled_lhs(s.ad_value(169), -1.0, s.ad_value(171)));
        }

        if (!s.b[1463]) {
            s.store_scalar(371, 1.0);
        }

        s.b[1464] = (p.p67 == 1.0);
        s.store_scalar(1464, if s.b[1464] { 1.0 } else { 0.0 });

    }

    pub(super) fn stamp_reactive_block_20(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        if s.b[1464] {
            s.store_div_scaled_product_indices(220, 336, 412, 2.0, 414, 1.0);
        }

        if (!s.b[1464]) {
            s.store_div_scaled_product_indices(220, 336, 412, 2.0, 416, 1.0);
        }

        s.store_mul(221, 220, 156);

        s.b[1465] = (p.p67 == 1.0);
        s.store_scalar(1465, if s.b[1465] { 1.0 } else { 0.0 });

        if s.b[1465] {
            s.store_pow_ad(168, A::div(s.ad_value(405), s.ad_value(221)), s.ad_value(697));
        }

        if (!s.b[1465]) {
            s.store_pow_ad(168, A::div(s.ad_value(402), s.ad_value(221)), s.ad_value(697));
        }

        s.store_div_from_scalar(169, 1.0, 697);

        s.store_offset_pow_ad(225, s.ad_value(696), s.ad_value(169), 1.0);

        s.store_div_scaled_offset_numerator(213, A::pow(A::add(s.ad_value(696), s.ad_value(168)), s.ad_value(169)), 1.0, 1.0, s.ad_value(225), 1.0);

        s.store_scaled_add_offset_sqrt_square_offset(881, 881, 0.1, (-0.1), ((0.25 * 0.001) * 0.001), 0.5);

        s.store_mul(213, 213, 881);

        s.b[1466] = (s.v[794] != 0.0);
        s.store_scalar(1466, if s.b[1466] { 1.0 } else { 0.0 });

        if s.b[1466] {
            s.store_offset_mul_ad(207, s.ad_value(794), {
                if (!((1.0 + (((s.v[126] - s.v[390]) / s.v[794]) / (s.v[210] + s.v[221]))) > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if ((1.0 + (((s.v[126] - s.v[390]) / s.v[794]) / (s.v[210] + s.v[221]))) > 1e-38) {
                            A::ln(A::offset(A::div_scaled_inputs2_by_product(s.ad_value(126), 1.0, s.ad_value(390), (-1.0), s.ad_value(794), A::add(s.ad_value(210), s.ad_value(221)), 1.0), 1.0))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 1.0);
        }

        if (!s.b[1466]) {
            s.store_scalar(207, 1.0);
        }

        s.store_mul3_affine_lhs(140, 640, 894, (-1.60219e-19), 0.0, 156);

        s.store_div_add_scaled_inputs_rhs_indices(131, 339, 339, 1.0, 399, 1.0);

        s.store_add_ad_rhs(123, 399, A::mul_sub_from_scalar_lhs(2.0, s.ad_value(131), s.ad_value(181)));

        s.store_mul(122, 123, 402);

        s.b[1467] = (p.p64 == 0.0);
        s.store_scalar(1467, if s.b[1467] { 1.0 } else { 0.0 });

        s.b[1468] = (p.p64 == 1.0);
        s.store_scalar(1468, if s.b[1468] { 1.0 } else { 0.0 });

        s.b[1469] = (p.p64 == 2.0);
        s.store_scalar(1469, if s.b[1469] { 1.0 } else { 0.0 });

        if s.b[1467] {
            s.store_offset_mul(172, 711, 399, 1.0);
            s.store_div_from_scalar(169, 1.0, 172);
            s.store_scaled_add_sqrt_square_offset_rhs(168, 169, 169, 0.01, 0.5);
            s.store_mul_ad_product_lhs_mixed_ia(197, 194, A::offset(A::mul(s.ad_value(709), s.ad_value(168)), p.p908), 189);
            s.store_offset_mul_ad(188, A::div_scaled_product_by_product(s.ad_value(183), s.ad_value(123), s.v[115], s.ad_value(411), s.ad_value(209), 1.0), s.ad_value(197), 1.0);
        }

        if (s.b[1468] && (!s.b[1467])) {
            s.store_scalar(197, 0.0);
            s.store_scalar(188, 1.0);
            s.store_add_scaled_product_right_ad(170, 479, (-1.0), 114, A::voltage(ctx, nodes, Some(11), Some(8)), 1.0);
            s.store_sqrt_square_offset(171, 170, 0.1);
            s.store_scaled_add(482, 170, 171, 0.5);
            s.store_offset_mul(172, 711, 482, 1.0);
            s.store_div_from_scalar(169, 1.0, 172);
            s.store_scaled_add_sqrt_square_offset_rhs(168, 169, 169, 0.01, 0.5);
            s.store_mul_offset_ad_rhs(174, 853, A::mul(s.ad_value(425), A::powf(A::offset(A::square(A::voltage(ctx, nodes, Some(2), Some(8))), 1e-6), (0.5 * p.p921))), 1.0);
            s.store_add_scaled_product_right_ad(170, 479, (-1.0), 114, A::voltage(ctx, nodes, Some(11), Some(9)), 1.0);
            s.store_sqrt_square_offset(171, 170, 0.1);
            s.store_scaled_add(483, 170, 171, 0.5);
            s.store_offset_mul(172, 712, 483, 1.0);
            s.store_div_from_scalar(169, 1.0, 172);
            s.store_scaled_add_sqrt_square_offset_rhs(168, 169, 169, 0.01, 0.5);
            s.store_mul_offset_ad_rhs(174, 852, A::mul(s.ad_value(426), A::powf(A::offset(A::square(A::voltage(ctx, nodes, Some(0), Some(9))), 1e-6), (0.5 * p.p922))), 1.0);
        }

        if (s.b[1469] && (!(s.b[1467] || s.b[1468]))) {
            s.store_offset_mul(172, 711, 399, 1.0);
            s.store_div_from_scalar(169, 1.0, 172);
            s.store_scaled_add_sqrt_square_offset_rhs(168, 169, 169, 0.01, 0.5);
            s.store_mul_add_ad_rhs(197, 194, A::add_scaled_offset_product_lhs(s.ad_value(190), 1.0, A::mul(s.ad_value(709), s.ad_value(168)), p.p908, s.ad_value(189), 1.0), s.ad_value(191));
            s.store_offset_mul_ad(188, A::div_scaled_product_by_product(s.ad_value(183), s.ad_value(123), s.v[115], s.ad_value(411), s.ad_value(209), 1.0), s.ad_value(197), 1.0);
        }

        s.store_div_scaled_product3_mixed_aiia(124, A::mul3_scaled_output(s.ad_value(183), s.ad_value(122), s.ad_value(205), s.v[115]), 366, 371, 1.0, A::mul3(s.ad_value(411), s.ad_value(209), s.ad_value(188)), 1.0);

        s.store_scale(124, 124, p.p25);

        s.b[1470] = (p.p67 == 1.0);
        s.store_scalar(1470, if s.b[1470] { 1.0 } else { 0.0 });

        if s.b[1470] {
            s.store_div_scaled_inputs2_indices(341, 403, 2.0, 181, 1.0, 213, 1.0);
            s.store_add_ad_rhs(138, 403, A::div_scaled_product(s.ad_value(405), s.ad_value(405), 1.0, s.ad_value(341), 6.0));
            s.store_scaled_sub_ad_rhs(137, 403, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(405), 1.0, A::mul_offset_rhs(A::div(s.ad_value(405), s.ad_value(341)), A::div_scaled_inputs(s.ad_value(405), 1.0, s.ad_value(341), 5.0), 1.0), 1.0 / (6.0)), (-0.5));
        }

        if (!s.b[1470]) {
            s.store_div_scaled_inputs2_indices(341, 399, 2.0, 181, 1.0, 213, 1.0);
            s.store_add_ad_rhs(138, 399, A::div_scaled_product(s.ad_value(402), s.ad_value(402), 1.0, s.ad_value(341), 6.0));
            s.store_scaled_sub_ad_rhs(137, 399, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(402), 1.0, A::mul_offset_rhs(A::div(s.ad_value(402), s.ad_value(341)), A::div_scaled_inputs(s.ad_value(402), 1.0, s.ad_value(341), 5.0), 1.0), 1.0 / (6.0)), (-0.5));
        }

        s.store_div_from_scalar(208, 1.0, 207);

        s.store_add_scaled_products_right_left_ad(138, 208, 138, 1.0, A::offset(s.ad_value(207), (-1.0)), 393, 1.0);

        s.store_add_scaled_products_mixed_aiai(137, A::square(s.ad_value(208)), 137, 1.0, A::sub(s.ad_value(207), s.ad_value(208)), 393, 0.5);

        s.store_sub_scaled_inputs(139, 138, -1.0, 137, 1.0);

        s.store_mul3_affine_lhs(175, 159, 156, s.v[115], 0.0, 372);

        s.store_mul(138, 175, 138);

        s.store_mul(137, 175, 137);

        s.store_mul(139, 175, 139);

        s.copy_ad(592, 138);

        s.b[1472] = (p.p61 != 0.0);
        s.store_scalar(1472, if s.b[1472] { 1.0 } else { 0.0 });

        s.b[1473] = (p.p62 == 5.0);
        s.store_scalar(1473, if s.b[1473] { 1.0 } else { 0.0 });

        if (s.b[1472] && s.b[1473]) {
            s.store_mul3_affine_lhs(169, 160, 157, s.v[115], 0.0, 494);
        }

        if (s.b[1472] && (!s.b[1473])) {
            s.store_mul3_affine_lhs(169, 159, 157, s.v[115], 0.0, 494);
        }

        if s.b[1472] {
            s.copy_ad(176, 904);
            s.store_mul(340, 176, 169);
            s.store_neg(495, 340);
            s.copy_ad(496, 340);
            s.store_mul3_affine_lhs(169, 159, 156, s.v[115], 0.0, 163);
            s.store_sub(170, 401, 904);
            s.store_mul(340, 169, 170);
            s.store_sub(495, 495, 340);
            s.store_add(496, 496, 340);
            s.store_mul3_affine_lhs(169, 159, 156, s.v[115], 0.0, 163);
            s.store_scaled_mul_ad(170, A::offset(s.ad_value(923), (-1.0)), A::add(s.ad_value(399), A::div_scaled_product(s.ad_value(402), s.ad_value(402), 1.0, s.ad_value(341), 6.0)), 0.5);
            s.store_mul(340, 169, 170);
            s.store_sub(495, 495, 340);
            s.store_add(496, 496, 340);
        }

        s.b[1474] = (s.v[128] < 0.0);
        s.store_scalar(1474, if s.b[1474] { 1.0 } else { 0.0 });

        if s.b[1474] {
            s.copy_ad(169, 137);
            s.copy_ad(137, 139);
            s.copy_ad(139, 169);
        }

        s.b[1475] = (p.p78 != 1.0);
        s.store_scalar(1475, if s.b[1475] { 1.0 } else { 0.0 });

        s.b[1476] = (p.p76 != 2.0);
        s.store_scalar(1476, if s.b[1476] { 1.0 } else { 0.0 });

        if (s.b[1475] && s.b[1476]) {
            s.store_scaled_mul(169, 159, 114, s.v[115]);
            s.store_mul_voltage_ad(170, s.ad_value(114), ctx, nodes, Some(10), Some(6));
            s.store_offset_sub(168, 170, 518, 0.02);
            s.store_scaled_sub_sqrt_square_offset_rhs(510, 168, 168, (4.0 * 0.02), 0.5);
            s.store_mul_ad_rhs(498, 169, A::add_scaled_products(s.ad_value(648), A::add_scaled_offset_product_rhs(A::add_scaled_inputs3(s.ad_value(170), 1.0, s.ad_value(518), (-1.0), s.ad_value(510), -1.0), 1.0, s.ad_value(651), A::sqrt(A::sub_from_scalar(1.0, A::div_scaled_inputs(s.ad_value(510), 4.0, s.ad_value(651), 1.0))), (-1.0), (-0.5)), 1.0, s.ad_value(646), s.ad_value(170), 1.0));
            s.store_mul_voltage_ad(170, s.ad_value(114), ctx, nodes, Some(10), Some(5));
            s.store_offset_sub(168, 170, 518, 0.02);
            s.store_scaled_sub_sqrt_square_offset_rhs(511, 168, 168, (4.0 * 0.02), 0.5);
            s.store_mul_ad_rhs(499, 169, A::add_scaled_products(s.ad_value(649), A::add_scaled_offset_product_rhs(A::add_scaled_inputs3(s.ad_value(170), 1.0, s.ad_value(518), (-1.0), s.ad_value(511), -1.0), 1.0, s.ad_value(652), A::sqrt(A::sub_from_scalar(1.0, A::div_scaled_inputs(s.ad_value(511), 4.0, s.ad_value(652), 1.0))), (-1.0), (-0.5)), 1.0, s.ad_value(647), s.ad_value(170), 1.0));
        }

        if (s.b[1475] && (!s.b[1476])) {
            s.store_scaled_mul(169, 159, 114, s.v[115]);
            s.store_mul_voltage_ad(170, s.ad_value(114), ctx, nodes, Some(13), Some(6));
            s.store_offset_sub(168, 170, 518, 0.02);
            s.store_scaled_sub_sqrt_square_offset_rhs(510, 168, 168, (4.0 * 0.02), 0.5);
            s.store_mul_ad_rhs(498, 169, A::add_scaled_products(s.ad_value(648), A::add_scaled_offset_product_rhs(A::add_scaled_inputs3(s.ad_value(170), 1.0, s.ad_value(518), (-1.0), s.ad_value(510), -1.0), 1.0, s.ad_value(651), A::sqrt(A::sub_from_scalar(1.0, A::div_scaled_inputs(s.ad_value(510), 4.0, s.ad_value(651), 1.0))), (-1.0), (-0.5)), 1.0, s.ad_value(646), s.ad_value(170), 1.0));
            s.store_mul_voltage_ad(170, s.ad_value(114), ctx, nodes, Some(14), Some(5));
            s.store_offset_sub(168, 170, 518, 0.02);
            s.store_scaled_sub_sqrt_square_offset_rhs(511, 168, 168, (4.0 * 0.02), 0.5);
            s.store_mul_ad_rhs(499, 169, A::add_scaled_products(s.ad_value(649), A::add_scaled_offset_product_rhs(A::add_scaled_inputs3(s.ad_value(170), 1.0, s.ad_value(518), (-1.0), s.ad_value(511), -1.0), 1.0, s.ad_value(652), A::sqrt(A::sub_from_scalar(1.0, A::div_scaled_inputs(s.ad_value(511), 4.0, s.ad_value(652), 1.0))), (-1.0), (-0.5)), 1.0, s.ad_value(647), s.ad_value(170), 1.0));
        }

        s.b[1477] = (p.p78 == 0.0);
        s.store_scalar(1477, if s.b[1477] { 1.0 } else { 0.0 });

        s.b[1478] = (p.p76 != 2.0);
        s.store_scalar(1478, if s.b[1478] { 1.0 } else { 0.0 });

        if (s.b[1477] && s.b[1478]) {
            s.store_scale(169, 159, s.v[115]);
            s.store_mul_ad_product_rhs_mixed_ia(500, 169, 643, A::voltage(ctx, nodes, Some(10), Some(6)));
            s.store_mul_ad_product_rhs_mixed_ia(501, 169, 642, A::voltage(ctx, nodes, Some(10), Some(5)));
            s.store_add(505, 498, 500);
            s.store_add(506, 499, 501);
        }

        if (s.b[1477] && (!s.b[1478])) {
            s.store_scale(169, 159, s.v[115]);
            s.store_mul_ad_product_rhs_mixed_ia(500, 169, 643, A::voltage(ctx, nodes, Some(13), Some(6)));
            s.store_mul_ad_product_rhs_mixed_ia(501, 169, 642, A::voltage(ctx, nodes, Some(14), Some(5)));
            s.store_add(505, 498, 500);
            s.store_add(506, 499, 501);
        }

        s.b[1479] = (p.p78 == 1.0);
        s.store_scalar(1479, if s.b[1479] { 1.0 } else { 0.0 });

        s.b[1480] = (p.p76 != 2.0);
        s.store_scalar(1480, if s.b[1480] { 1.0 } else { 0.0 });

        s.b[1481] = (p.p63 == 1.0);
        s.store_scalar(1481, if s.b[1481] { 1.0 } else { 0.0 });

        if ((((!s.b[1477]) && s.b[1479]) && s.b[1480]) && s.b[1481]) {
            s.store_scale(168, 159, s.v[115]);
            s.store_mul(644, 168, 644);
            s.store_mul(645, 168, 645);
            s.store_scale(513, 168, p.p15);
            s.store_scale(514, 168, p.p16);
        }

        if ((((!s.b[1477]) && s.b[1479]) && s.b[1480]) && (!s.b[1481])) {
            s.store_scalar(513, p.p15);
            s.store_scalar(514, p.p16);
        }

        if (((!s.b[1477]) && s.b[1479]) && s.b[1480]) {
            s.store_mul_voltage_ad(498, s.ad_value(644), ctx, nodes, Some(10), Some(6));
            s.store_mul_voltage_ad(499, s.ad_value(645), ctx, nodes, Some(10), Some(5));
            s.copy_ad(505, 498);
            s.copy_ad(506, 499);
            s.store_mul_voltage_ad(500, s.ad_value(513), ctx, nodes, Some(10), Some(2));
            s.store_mul_voltage_ad(501, s.ad_value(514), ctx, nodes, Some(10), Some(0));
        }

        s.b[1482] = (p.p63 == 1.0);
        s.store_scalar(1482, if s.b[1482] { 1.0 } else { 0.0 });

        if ((((!s.b[1477]) && s.b[1479]) && (!s.b[1480])) && s.b[1482]) {
            s.store_scale(168, 159, s.v[115]);
            s.store_mul(644, 168, 644);
            s.store_mul(645, 168, 645);
        }

    }

    pub(super) fn stamp_reactive_block_21(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        if ((((!s.b[1477]) && s.b[1479]) && (!s.b[1480])) && s.b[1482]) {
            s.store_scale(513, 168, p.p15);
            s.store_scale(514, 168, p.p16);
        }

        if ((((!s.b[1477]) && s.b[1479]) && (!s.b[1480])) && (!s.b[1482])) {
            s.store_scalar(513, p.p15);
            s.store_scalar(514, p.p16);
        }

        if (((!s.b[1477]) && s.b[1479]) && (!s.b[1480])) {
            s.store_mul_voltage_ad(498, s.ad_value(644), ctx, nodes, Some(13), Some(6));
            s.store_mul_voltage_ad(499, s.ad_value(645), ctx, nodes, Some(14), Some(5));
            s.copy_ad(505, 498);
            s.copy_ad(506, 499);
            s.store_mul_voltage_ad(500, s.ad_value(513), ctx, nodes, Some(13), Some(2));
            s.store_mul_voltage_ad(501, s.ad_value(514), ctx, nodes, Some(14), Some(0));
        }

        s.b[1483] = (p.p76 != 2.0);
        s.store_scalar(1483, if s.b[1483] { 1.0 } else { 0.0 });

        if (((!s.b[1477]) && (!s.b[1479])) && s.b[1483]) {
            s.store_mul_voltage_ad(500, s.ad_value(453), ctx, nodes, Some(10), Some(6));
            s.store_mul_voltage_ad(501, s.ad_value(453), ctx, nodes, Some(10), Some(5));
            s.store_add(505, 498, 500);
            s.store_add(506, 499, 501);
        }

        if (((!s.b[1477]) && (!s.b[1479])) && (!s.b[1483])) {
            s.store_mul_voltage_ad(500, s.ad_value(453), ctx, nodes, Some(13), Some(6));
            s.store_mul_voltage_ad(501, s.ad_value(453), ctx, nodes, Some(14), Some(5));
            s.store_add(505, 498, 500);
            s.store_add(506, 499, 501);
        }

        s.b[1484] = (p.p65 == 1.0);
        s.store_scalar(1484, if s.b[1484] { 1.0 } else { 0.0 });

        if s.b[1484] {
            s.store_scalar(239, 1e-6);
            s.store_mul_div_scaled_inputs_mixed_aii(178, A::sqrt(s.ad_value(179)), 239, 1.0, 181, 2.0);
            s.store_scale(168, 178, 0.5);
            s.store_div_scaled_inputs_mixed_ai(170, A::offset(s.ad_value(132), (-p.p144)), -1.0, 179, 1.0);
        }

        s.b[1485] = ((s.v[170] * s.v[179]) > (s.v[166] + (s.v[178] * (((s.v[166] * s.v[179])) as f64).sqrt())));
        s.store_scalar(1485, if s.b[1485] { 1.0 } else { 0.0 });

        if (s.b[1484] && s.b[1485]) {
            s.store_sub_ad_lhs(169, A::sqrt(A::add(A::offset(s.ad_value(170), (-1.0)), A::square(s.ad_value(168)))), 168);
            s.store_offset_square(340, 169, 1.0);
            s.store_offset_ad(175, A::limited_exp_scaled_input(s.ad_value(340), -1.0), (-1.0));
        }

        if (s.b[1484] && (!s.b[1485])) {
            s.store_sub_scaled_ad_rhs(171, 170, 0.5, A::scale_offset(s.ad_value(178), ((1.0 / (((2.0) as f64).sqrt())) * (3.0)), 3.0));
            s.store_add_ad_rhs(340, 171, A::sqrt(A::add_scaled_inputs(A::square(s.ad_value(171)), 1.0, s.ad_value(170), 6.0)));
        }

        s.b[1486] = (s.v[170] < 0.0);
        s.store_scalar(1486, if s.b[1486] { 1.0 } else { 0.0 });

        if ((s.b[1484] && (!s.b[1485])) && s.b[1486]) {
            s.store_div_scaled_inputs2_indices(172, 170, 1.0, 340, (-1.0), 178, 1.0);
            s.store_sub_square_lhs(175, 172, 340);
        }

        if ((s.b[1484] && (!s.b[1485])) && s.b[1486]) {
            s.store_neg_ad(340, {
                if (!(((1.0 - s.v[340]) + (s.v[172] * s.v[172])) > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if (((1.0 - s.v[340]) + (s.v[172] * s.v[172])) > 1e-38) {
                            A::ln(A::add(A::sub_from_scalar(1.0, s.ad_value(340)), A::square(s.ad_value(172))))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            });
        }

        if ((s.b[1484] && (!s.b[1485])) && (!s.b[1486])) {
            s.store_limited_exp_scaled_input(341, 340, (-1.2));
            s.store_sub_ad_lhs(172, A::sqrt(A::add_scaled_inputs3_offset(s.ad_value(170), 1.0, s.ad_value(341), 1.0, A::square(s.ad_value(168)), 1.0, (-1.0))), 168);
            s.store_add_ad(340, A::sub_from_scalar(1.0, s.ad_value(341)), A::square(s.ad_value(172)));
            s.store_offset_ad(175, A::limited_exp_scaled_input(s.ad_value(340), -1.0), (-1.0));
        }

        if s.b[1484] {
            s.store_sqrt_add(176, 175, 340);
        }

        s.b[1487] = (s.v[340] > 1e-15);
        s.store_scalar(1487, if s.b[1487] { 1.0 } else { 0.0 });

        if (s.b[1484] && s.b[1487]) {
            s.store_add_scaled_inputs_product_indices(344, 170, -1.0, 340, 1.0, 178, 176, 1.0);
            s.store_sub_from_scalar_ad(345, 1.0, A::div_scaled_product(s.ad_value(178), s.ad_value(175), 0.5, s.ad_value(176), 1.0));
            s.store_sub_div_rhs_indices(177, 340, 344, 345);
        }

        s.b[1488] = (s.v[340] < (-1e-15));
        s.store_scalar(1488, if s.b[1488] { 1.0 } else { 0.0 });

        if ((s.b[1484] && (!s.b[1487])) && s.b[1488]) {
            s.store_add_scaled_inputs_product_indices(344, 170, -1.0, 340, 1.0, 178, 176, (-1.0));
            s.store_offset_div_scaled_product(345, s.ad_value(178), s.ad_value(175), 0.5, s.ad_value(176), 1.0, 1.0);
            s.store_sub_div_rhs_indices(177, 340, 344, 345);
        }

        if ((s.b[1484] && (!s.b[1487])) && (!s.b[1488])) {
            s.store_scalar(177, 0.0);
        }

        if s.b[1484] {
            s.store_mul_ad_product_lhs_mixed_ia(906, 178, A::limited_exp_scaled_input(s.ad_value(177), (-1.0 / (2.0))), 179);
            s.store_abs_voltage(915, ctx, nodes, Some(7), Some(6));
            s.store_mul_div_from_scalar_lhs(916, (2.0 * p.p454), 416, 397);
            s.store_scale(917, 916, p.p1);
            s.store_scalar(920, (1.0 / p.p530));
            s.store_add_scaled_inputs(175, 906, p.p491, 182, (2.0 * p.p491));
            s.store_div_scaled_product_add_scaled_denominator_indices(918, 917, 175, 1.0, 917, 1.0, 175, 1.0, 1.0);
        }

        if s.b[1484] {
            s.store_offset_ad(918, {
                if (!((s.v[918] - 0.001) < ((-10000.0) * 1e-5))) {
                    A::add_scaled_inputs(A::offset(s.ad_value(918), (-0.001)), 0.5, A::sqrt_square_offset(A::offset(s.ad_value(918), (-0.001)), ((4.0 * 1e-5) * 1e-5)), 0.5)
                } else {
                    {
                        if ((s.v[918] - 0.001) < ((-10000.0) * 1e-5)) {
                            A::div_scalar_offset_denominator(((-1e-5) * 1e-5), s.ad_value(918), (-0.001), 1.0)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 0.001);
        }

        if s.b[1484] {
            s.store_powf_ad(176, A::offset(A::div(s.ad_value(915), s.ad_value(918)), 1e-6), p.p530);
            s.store_pow_ad(177, A::offset(s.ad_value(176), 1.0), s.ad_value(920));
            s.store_min_ad(919, A::div(s.ad_value(915), s.ad_value(177)), s.ad_value(915));
            s.store_scalar(239, 1e-6);
            s.store_mul_div_scaled_inputs_mixed_aii(178, A::sqrt(s.ad_value(179)), 239, 1.0, 181, 2.0);
            s.store_scale(168, 178, 0.5);
            s.store_div_scaled_inputs_mixed_ai(170, A::offset(A::add(s.ad_value(133), s.ad_value(919)), (-p.p143)), -1.0, 179, 1.0);
        }

        s.b[1489] = ((s.v[170] * s.v[179]) > (s.v[166] + (s.v[178] * (((s.v[166] * s.v[179])) as f64).sqrt())));
        s.store_scalar(1489, if s.b[1489] { 1.0 } else { 0.0 });

        if (s.b[1484] && s.b[1489]) {
            s.store_sub_ad_lhs(169, A::sqrt(A::add(A::offset(s.ad_value(170), (-1.0)), A::square(s.ad_value(168)))), 168);
            s.store_offset_square(340, 169, 1.0);
            s.store_offset_ad(175, A::limited_exp_scaled_input(s.ad_value(340), -1.0), (-1.0));
        }

        if (s.b[1484] && (!s.b[1489])) {
            s.store_sub_scaled_ad_rhs(171, 170, 0.5, A::scale_offset(s.ad_value(178), ((1.0 / (((2.0) as f64).sqrt())) * (3.0)), 3.0));
            s.store_add_ad_rhs(340, 171, A::sqrt(A::add_scaled_inputs(A::square(s.ad_value(171)), 1.0, s.ad_value(170), 6.0)));
        }

        s.b[1490] = (s.v[170] < 0.0);
        s.store_scalar(1490, if s.b[1490] { 1.0 } else { 0.0 });

        if ((s.b[1484] && (!s.b[1489])) && s.b[1490]) {
            s.store_div_scaled_inputs2_indices(172, 170, 1.0, 340, (-1.0), 178, 1.0);
            s.store_sub_square_lhs(175, 172, 340);
        }

        if ((s.b[1484] && (!s.b[1489])) && s.b[1490]) {
            s.store_neg_ad(340, {
                if (!(((1.0 - s.v[340]) + (s.v[172] * s.v[172])) > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if (((1.0 - s.v[340]) + (s.v[172] * s.v[172])) > 1e-38) {
                            A::ln(A::add(A::sub_from_scalar(1.0, s.ad_value(340)), A::square(s.ad_value(172))))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            });
        }

        if ((s.b[1484] && (!s.b[1489])) && (!s.b[1490])) {
            s.store_limited_exp_scaled_input(341, 340, (-1.2));
            s.store_sub_ad_lhs(172, A::sqrt(A::add_scaled_inputs3_offset(s.ad_value(170), 1.0, s.ad_value(341), 1.0, A::square(s.ad_value(168)), 1.0, (-1.0))), 168);
            s.store_add_ad(340, A::sub_from_scalar(1.0, s.ad_value(341)), A::square(s.ad_value(172)));
            s.store_offset_ad(175, A::limited_exp_scaled_input(s.ad_value(340), -1.0), (-1.0));
        }

        if s.b[1484] {
            s.store_sqrt_add(176, 175, 340);
        }

        s.b[1491] = (s.v[340] > 1e-15);
        s.store_scalar(1491, if s.b[1491] { 1.0 } else { 0.0 });

        if (s.b[1484] && s.b[1491]) {
            s.store_add_scaled_inputs_product_indices(344, 170, -1.0, 340, 1.0, 178, 176, 1.0);
            s.store_sub_from_scalar_ad(345, 1.0, A::div_scaled_product(s.ad_value(178), s.ad_value(175), 0.5, s.ad_value(176), 1.0));
            s.store_sub_div_rhs_indices(177, 340, 344, 345);
        }

        s.b[1492] = (s.v[340] < (-1e-15));
        s.store_scalar(1492, if s.b[1492] { 1.0 } else { 0.0 });

        if ((s.b[1484] && (!s.b[1491])) && s.b[1492]) {
            s.store_add_scaled_inputs_product_indices(344, 170, -1.0, 340, 1.0, 178, 176, (-1.0));
            s.store_offset_div_scaled_product(345, s.ad_value(178), s.ad_value(175), 0.5, s.ad_value(176), 1.0, 1.0);
            s.store_sub_div_rhs_indices(177, 340, 344, 345);
        }

        if ((s.b[1484] && (!s.b[1491])) && (!s.b[1492])) {
            s.store_scalar(177, 0.0);
        }

        if s.b[1484] {
            s.store_mul_ad_product_lhs_mixed_ia(907, 178, A::limited_exp_scaled_input(s.ad_value(177), (-1.0 / (2.0))), 179);
            s.store_sub(911, 906, 907);
            s.store_scaled_add(910, 906, 907, 0.5);
            s.store_div_scaled_inputs2_indices(341, 910, 2.0, 181, 1.0, 209, 1.0);
            s.store_add_ad_rhs(905, 910, A::div_scaled_product(s.ad_value(911), s.ad_value(911), 1.0, s.ad_value(341), 6.0));
            s.store_scaled_sub_ad_rhs(909, 910, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(911), 1.0, A::mul_offset_rhs(A::div(s.ad_value(911), s.ad_value(341)), A::div_scaled_inputs(s.ad_value(911), 1.0, s.ad_value(341), 5.0), 1.0), 1.0 / (6.0)), 0.5);
            s.store_sub(908, 905, 909);
        }

        s.b[1493] = (p.p62 == 5.0);
        s.store_scalar(1493, if s.b[1493] { 1.0 } else { 0.0 });

        if (s.b[1484] && s.b[1493]) {
            s.store_scaled_mul(169, 160, 494, (s.v[115] * p.p1));
        }

        if (s.b[1484] && (!s.b[1493])) {
            s.store_scaled_mul(169, 159, 494, (s.v[115] * p.p1));
        }

        if s.b[1484] {
            s.copy_ad(176, 908);
            s.copy_ad(177, 909);
            s.store_mul(340, 176, 169);
            s.store_mul(341, 177, 169);
            s.copy_ad(908, 340);
            s.copy_ad(909, 341);
            s.copy_ad(504, 908);
            s.copy_ad(503, 909);
        }

        s.store_scaled_voltage(502, ctx, nodes, Some(0), Some(2), p.p17);

        s.b[1494] = (p.p71 == 1.0);
        s.store_scalar(1494, if s.b[1494] { 1.0 } else { 0.0 });

        if s.b[1494] {
            s.store_div_scaled_add_product(168, s.ad_value(259), 1.0, s.ad_value(260), s.ad_value(153), 1.0, s.ad_value(153), 1.0);
        }

        s.b[1495] = ((s.v[168] <= 0.0) || (s.v[248] <= 0.0));
        s.store_scalar(1495, if s.b[1495] { 1.0 } else { 0.0 });

        if (s.b[1494] && (!s.b[1495])) {
            s.store_div_scaled_value_offset_denominator(169, s.ad_value(248), -1.0, s.ad_value(202), 1e-30, 1.0);
        }

        s.b[1496] = (p.p71 == 2.0);
        s.store_scalar(1496, if s.b[1496] { 1.0 } else { 0.0 });

        if ((!s.b[1494]) && s.b[1496]) {
            s.store_div_scaled_add_product(493, s.ad_value(261), 1.0, s.ad_value(262), s.ad_value(153), 1.0, s.ad_value(153), 1.0);
        }

        s.b[1497] = (s.v[493] <= 0.0);
        s.store_scalar(1497, if s.b[1497] { 1.0 } else { 0.0 });

        if (((!s.b[1494]) && s.b[1496]) && (!s.b[1497])) {
            s.store_mul(168, 783, 153);
            s.store_div_scaled_product_offset_denominator(169, s.ad_value(249), s.ad_value(168), 1.0, s.ad_value(168), 1.0, 1.0);
        }

        if (((!s.b[1494]) && s.b[1496]) && (!s.b[1497])) {
            s.store_div_from_scalar_offset_ad(168, 1.0, {
                if (!((s.v[786] * s.v[348]) < ((-10000.0) * p.p1441))) {
                    A::add_scaled_product(A::sqrt_square_offset(A::mul(s.ad_value(786), s.ad_value(348)), ((4.0 * p.p1441) * p.p1441)), 0.5, s.ad_value(786), s.ad_value(348), 0.5)
                } else {
                    {
                        if ((s.v[786] * s.v[348]) < ((-10000.0) * p.p1441)) {
                            A::div_from_scalar(((-p.p1441) * p.p1441), A::mul(s.ad_value(786), s.ad_value(348)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 1.0);
        }

        if (((!s.b[1494]) && s.b[1496]) && (!s.b[1497])) {
            s.store_add(171, 168, 787);
        }

    }

    pub(super) fn stamp_reactive_block_22(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (((!s.b[1494]) && s.b[1496]) && (!s.b[1497])) {
            if (!((s.v[348] * s.v[171]) < ((-10000.0) * p.p1442))) {
                s.store_add_scaled_product_value_ad(170, A::sqrt_square_offset(A::mul(s.ad_value(348), s.ad_value(171)), ((4.0 * p.p1442) * p.p1442)), 0.5, 348, 171, 0.5);
            } else {
                if ((s.v[348] * s.v[171]) < ((-10000.0) * p.p1442)) {
                    s.store_div_from_scalar_mul_ad(170, ((-p.p1442) * p.p1442), s.ad_value(348), s.ad_value(171));
                } else {
                    s.store_scalar(170, 0.0);
                }
            }
        }

        if (((!s.b[1494]) && s.b[1496]) && (!s.b[1497])) {
            s.store_div_from_scalar_offset_product(171, 1.0, 788, 126, 1.0);
            s.store_mul3_lhs(491, 169, 170, 171);
            s.store_mul_sub_from_scalar_ad_rhs(490, 491, 1.0, A::div(s.ad_value(784), s.ad_value(153)));
            s.store_sub(489, 126, 490);
            s.store_add_ad(168, A::add_scaled_product(s.ad_value(782), 1.0, s.ad_value(781), s.ad_value(489), 1.0), A::mul3(s.ad_value(780), s.ad_value(489), s.ad_value(489)));
            s.store_sqrt_square_offset(169, 168, 1e-10);
        }

        s.b[1498] = (p.p69 != 0.0);
        s.store_scalar(1498, if s.b[1498] { 1.0 } else { 0.0 });

        if s.b[1498] {
            s.store_div_scaled_inputs2_by_product(169, s.ad_value(399), 1.0, s.ad_value(725), (-1.0), s.ad_value(726), s.ad_value(179), 1.0);
            s.store_offset_add_scaled_inputs(170, A::offset(A::add_scaled_product(s.ad_value(243), 1.0, s.ad_value(723), s.ad_value(399), (-1.0)), (((-(-p.p1110))) + ((-1e-6)))), 0.5, A::sqrt_square_offset(A::offset(A::add_scaled_product(s.ad_value(243), 1.0, s.ad_value(723), s.ad_value(399), (-1.0)), (((-(-p.p1110))) + ((-1e-6)))), (-((4.0 * (-p.p1110)) * 1e-6))), 0.5, (-p.p1110));
            s.store_offset_mul(171, 724, 399, 1.0);
            s.store_scaled_mul(172, 170, 171, ((-982222000000.0) * p.p1109));
            s.store_limited_exp(174, 172);
            s.store_scalar(175, 3.75956e-7);
            s.store_add_scaled_inputs3_indices(468, 167, 1.0, 146, (-0.5), 166, -1.0);
            s.store_sub(168, 468, 497);
            s.store_div_scaled_value_by_product(169, s.ad_value(168), 1.0, s.ad_value(731), s.ad_value(179), 1.0);
        }

        s.b[1499] = (p.p61 != 0.0);
        s.store_scalar(1499, if s.b[1499] { 1.0 } else { 0.0 });

        if (s.b[1498] && s.b[1499]) {
            s.copy_ad(466, 904);
        }

        s.b[1500] = (s.v[468] <= 0.0);
        s.store_scalar(1500, if s.b[1500] { 1.0 } else { 0.0 });

        if ((s.b[1498] && (!s.b[1499])) && s.b[1500]) {
            s.store_scaled_add_ad(466, A::offset(s.ad_value(168), (-0.02)), A::sqrt(A::sub_scaled_inputs(A::square(A::offset(s.ad_value(168), (-0.02))), 1.0, s.ad_value(468), 0.08)), 0.5);
        }

        if ((s.b[1498] && (!s.b[1499])) && (!s.b[1500])) {
            s.store_scaled_add_ad(466, A::offset(s.ad_value(168), (-0.02)), A::sqrt(A::add_scaled_inputs(A::square(A::offset(s.ad_value(168), (-0.02))), 1.0, s.ad_value(468), 0.08)), 0.5);
        }

        if s.b[1498] {
            s.store_offset_add_scaled_inputs(170, A::offset(A::add_scaled_product(s.ad_value(244), 1.0, s.ad_value(729), s.ad_value(466), (-1.0)), (((-(-p.p1111))) + ((-1e-6)))), 0.5, A::sqrt_square_offset(A::offset(A::add_scaled_product(s.ad_value(244), 1.0, s.ad_value(729), s.ad_value(466), (-1.0)), (((-(-p.p1111))) + ((-1e-6)))), (-((4.0 * (-p.p1111)) * 1e-6))), 0.5, (-p.p1111));
            s.store_offset_mul(171, 730, 466, 1.0);
            s.store_scaled_mul(172, 170, 171, ((-745669000000.0) * p.p1109));
            s.store_limited_exp(174, 172);
            s.store_scalar(175, 4.97232e-7);
        }

        s.b[1501] = (p.p68 != 0.0);
        s.store_scalar(1501, if s.b[1501] { 1.0 } else { 0.0 });

        if s.b[1501] {
            s.store_offset_add_scaled_inputs(169, A::offset(A::add_scaled_product(s.ad_value(245), 1.0, s.ad_value(734), s.ad_value(399), (-1.0)), (((-(-p.p1112))) + ((-1e-6)))), 0.5, A::sqrt_square_offset(A::offset(A::add_scaled_product(s.ad_value(245), 1.0, s.ad_value(734), s.ad_value(399), (-1.0)), (((-(-p.p1112))) + ((-1e-6)))), (-((4.0 * (-p.p1112)) * 1e-6))), 0.5, (-p.p1112));
            s.store_offset_mul(170, 735, 399, 1.0);
            s.store_mul3_affine_lhs(171, 485, 169, (-p.p1109), 0.0, 170);
            s.store_mul_limited_exp_rhs(172, 399, 171);
            s.store_add_scaled_inputs4_indices(174, 497, 1.0, 127, 0.5, 521, 0.5, 522, 0.5);
            s.store_offset_sqrt_ad(473, A::offset(A::square(s.ad_value(390)), 0.01), (-0.1));
            s.store_mul(169, 736, 473);
            s.store_limited_exp_neg_input(474, 169);
            s.store_offset_add(171, 169, 474, (((-1.0)) + (0.0001)));
            s.store_offset_sub_from_scalar_ad(172, 1.0, A::mul_offset_lhs(s.ad_value(169), 1.0, s.ad_value(474)), 0.0001);
            s.store_offset_square(174, 169, 0.0002);
            s.store_sub(168, 134, 479);
            s.store_sqrt_square_offset(482, 168, 0.0001);
        }

        s.b[1502] = (p.p82 == 1.0);
        s.store_scalar(1502, if s.b[1502] { 1.0 } else { 0.0 });

        if (s.b[1501] && s.b[1502]) {
            if (!((s.v[246] - (s.v[739] * s.v[482])) < ((-10000.0) * 1e-6))) {
                s.store_scaled_add_sqrt_square_offset_ad(169, A::add_scaled_product(s.ad_value(246), 1.0, s.ad_value(739), s.ad_value(482), (-1.0)), ((4.0 * 1e-6) * 1e-6), 0.5);
            } else {
                if ((s.v[246] - (s.v[739] * s.v[482])) < ((-10000.0) * 1e-6)) {
                    s.store_div_from_scalar_ad(169, ((-1e-6) * 1e-6), A::add_scaled_product(s.ad_value(246), 1.0, s.ad_value(739), s.ad_value(482), (-1.0)));
                } else {
                    s.store_scalar(169, 0.0);
                }
            }
        }

        s.b[1503] = (s.v[740] < 0.01);
        s.store_scalar(1503, if s.b[1503] { 1.0 } else { 0.0 });

        if ((s.b[1501] && s.b[1502]) && s.b[1503]) {
            s.store_scalar(740, 0.01);
        }

        if (s.b[1501] && (!s.b[1502])) {
            s.store_add_scaled_product_indices(169, 246, 1.0, 739, 482, (-1.0));
        }

        if s.b[1501] {
            s.store_offset_mul(170, 740, 482, 1.0);
            s.store_mul_product3_indices(171, 170, 485, 742, 169, (-p.p1109));
            s.store_limited_exp(172, 171);
            s.store_sub(168, 136, 479);
            s.store_sqrt_square_offset(483, 168, 0.0001);
        }

        s.b[1505] = (p.p82 == 1.0);
        s.store_scalar(1505, if s.b[1505] { 1.0 } else { 0.0 });

        if (s.b[1501] && s.b[1505]) {
            if (!((s.v[247] - (s.v[745] * s.v[483])) < ((-10000.0) * 1e-6))) {
                s.store_scaled_add_sqrt_square_offset_ad(169, A::add_scaled_product(s.ad_value(247), 1.0, s.ad_value(745), s.ad_value(483), (-1.0)), ((4.0 * 1e-6) * 1e-6), 0.5);
            } else {
                if ((s.v[247] - (s.v[745] * s.v[483])) < ((-10000.0) * 1e-6)) {
                    s.store_div_from_scalar_ad(169, ((-1e-6) * 1e-6), A::add_scaled_product(s.ad_value(247), 1.0, s.ad_value(745), s.ad_value(483), (-1.0)));
                } else {
                    s.store_scalar(169, 0.0);
                }
            }
        }

        s.b[1506] = (s.v[746] < 0.01);
        s.store_scalar(1506, if s.b[1506] { 1.0 } else { 0.0 });

        if ((s.b[1501] && s.b[1505]) && s.b[1506]) {
            s.store_scalar(746, 0.01);
        }

        if (s.b[1501] && (!s.b[1505])) {
            s.store_add_scaled_product_indices(169, 247, 1.0, 745, 483, (-1.0));
        }

        if s.b[1501] {
            s.store_offset_mul(170, 746, 483, 1.0);
            s.store_mul_product3_indices(171, 170, 485, 742, 169, (-p.p1109));
            s.store_limited_exp(172, 171);
        }

        s.b[1508] = (p.p70 != 0.0);
        s.store_scalar(1508, if s.b[1508] { 1.0 } else { 0.0 });

        if s.b[1508] {
            s.store_scalar(168, (s.v[145] * p.p89));
        }

        s.b[1509] = ((s.v[747] <= 0.0) || (s.v[252] <= 0.0));
        s.store_scalar(1509, if s.b[1509] { 1.0 } else { 0.0 });

        if (s.b[1508] && s.b[1509]) {
            s.store_scalar(175, 0.0);
        }

        if (s.b[1508] && (!s.b[1509])) {
            s.store_div_scaled_inputs3_indices(169, 136, -1.0, 750, (-1.0), 479, 1.0, 168, 1.0);
        }

        if (s.b[1508] && (!s.b[1509])) {
            if (!(s.v[169] < ((-10000.0) * 0.01))) {
                s.store_scaled_add_sqrt_square_offset_rhs(169, 169, 169, ((4.0 * 0.01) * 0.01), 0.5);
            } else {
                if (s.v[169] < ((-10000.0) * 0.01)) {
                    s.store_div_from_scalar(169, ((-0.01) * 0.01), 169);
                } else {
                    s.store_scalar(169, 0.0);
                }
            }
        }

        if (s.b[1508] && (!s.b[1509])) {
            s.store_div_scaled_value_offset_denominator(170, s.ad_value(252), 1.0, s.ad_value(169), 0.001, 1.0);
            s.store_pow_indices(171, 169, 751);
        }

        s.b[1510] = (p.p61 != 0.0);
        s.store_scalar(1510, if s.b[1510] { 1.0 } else { 0.0 });

        if ((s.b[1508] && (!s.b[1509])) && s.b[1510]) {
            s.store_mul3_affine_lhs(172, 522, 522, -1.0, 0.0, 522);
            s.store_offset_add_ad(173, s.ad_value(749), A::abs(s.ad_value(172)), 1e-5);
        }

        if ((s.b[1508] && (!s.b[1509])) && s.b[1510]) {
            s.store_offset_ad(174, {
                if (!((s.v[172] / s.v[173]) < ((-10000.0) * 1e-6))) {
                    A::add_scaled_inputs(A::div(s.ad_value(172), s.ad_value(173)), 0.5, A::sqrt_square_offset(A::div(s.ad_value(172), s.ad_value(173)), ((4.0 * 1e-6) * 1e-6)), 0.5)
                } else {
                    {
                        if ((s.v[172] / s.v[173]) < ((-10000.0) * 1e-6)) {
                            A::div_from_scalar(((-1e-6) * 1e-6), A::div(s.ad_value(172), s.ad_value(173)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, (-1e-6));
        }

        if ((s.b[1508] && (!s.b[1509])) && s.b[1510]) {
            s.store_mul_ad_product_lhs(175, A::mul3(s.ad_value(747), s.ad_value(158), s.ad_value(171)), A::limited_exp_scaled_input(s.ad_value(170), -1.0), 174);
        }

        if ((s.b[1508] && (!s.b[1509])) && (!s.b[1510])) {
            s.store_mul_ad_product_lhs(175, A::mul3(s.ad_value(747), s.ad_value(158), s.ad_value(171)), A::limited_exp_scaled_input(s.ad_value(170), -1.0), 135);
        }

        s.b[1511] = ((p.p70 == 3.0) && (s.v[752] > 0.0));
        s.store_scalar(1511, if s.b[1511] { 1.0 } else { 0.0 });

        s.b[1512] = (p.p61 != 0.0);
        s.store_scalar(1512, if s.b[1512] { 1.0 } else { 0.0 });

        if ((s.b[1508] && s.b[1511]) && s.b[1512]) {
            s.store_mul_ad_rhs(254, 754, {
                if (!(((1.0 + (s.v[863] * s.v[232])) - 1e-6) < ((-10000.0) * 0.001))) {
                    A::add_scaled_inputs(A::offset(A::mul(s.ad_value(863), s.ad_value(232)), ((1.0) + ((-1e-6)))), 0.5, A::sqrt_square_offset(A::offset(A::mul(s.ad_value(863), s.ad_value(232)), ((1.0) + ((-1e-6)))), ((4.0 * 0.001) * 0.001)), 0.5)
                } else {
                    {
                        if (((1.0 + (s.v[863] * s.v[232])) - 1e-6) < ((-10000.0) * 0.001)) {
                            A::div_scalar_offset_denominator(((-0.001) * 0.001), A::mul(s.ad_value(863), s.ad_value(232)), ((1.0) + ((-1e-6))), 1.0)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            });
        }

        if ((s.b[1508] && s.b[1511]) && s.b[1512]) {
            s.store_div_scaled_inputs3_mixed_aiii(169, A::add_scaled_product(A::mul3(s.ad_value(753), s.ad_value(136), s.ad_value(136)), 1.0, s.ad_value(254), s.ad_value(136), (-1.0)), 1.0, 755, (-1.0), 479, 1.0, 179, 1.0);
            s.store_mul_product3_mixed_aiii(170, A::limited_exp(s.ad_value(169)), 752, 158, 141, 1.0);
            s.store_mul3_affine_lhs(172, 522, 522, -1.0, 0.0, 522);
            s.store_offset_add_ad(173, s.ad_value(749), A::abs(s.ad_value(172)), 1e-5);
        }

        if ((s.b[1508] && s.b[1511]) && s.b[1512]) {
            s.store_offset_ad(174, {
                if (!((s.v[172] / s.v[173]) < ((-10000.0) * 1e-6))) {
                    A::add_scaled_inputs(A::div(s.ad_value(172), s.ad_value(173)), 0.5, A::sqrt_square_offset(A::div(s.ad_value(172), s.ad_value(173)), ((4.0 * 1e-6) * 1e-6)), 0.5)
                } else {
                    {
                        if ((s.v[172] / s.v[173]) < ((-10000.0) * 1e-6)) {
                            A::div_from_scalar(((-1e-6) * 1e-6), A::div(s.ad_value(172), s.ad_value(173)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, (-1e-6));
        }

        if ((s.b[1508] && s.b[1511]) && s.b[1512]) {
            s.store_add_scaled_product_indices(175, 175, 1.0, 170, 174, 1.0);
        }

        if ((s.b[1508] && s.b[1511]) && (!s.b[1512])) {
            s.store_mul_ad_rhs(254, 754, {
                if (!(((1.0 + (s.v[863] * s.v[232])) - 1e-6) < ((-10000.0) * 0.001))) {
                    A::add_scaled_inputs(A::offset(A::mul(s.ad_value(863), s.ad_value(232)), ((1.0) + ((-1e-6)))), 0.5, A::sqrt_square_offset(A::offset(A::mul(s.ad_value(863), s.ad_value(232)), ((1.0) + ((-1e-6)))), ((4.0 * 0.001) * 0.001)), 0.5)
                } else {
                    {
                        if (((1.0 + (s.v[863] * s.v[232])) - 1e-6) < ((-10000.0) * 0.001)) {
                            A::div_scalar_offset_denominator(((-0.001) * 0.001), A::mul(s.ad_value(863), s.ad_value(232)), ((1.0) + ((-1e-6))), 1.0)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            });
        }

        if ((s.b[1508] && s.b[1511]) && (!s.b[1512])) {
            s.store_div_scaled_inputs3_mixed_aiii(169, A::add_scaled_product(A::mul3(s.ad_value(753), s.ad_value(136), s.ad_value(136)), 1.0, s.ad_value(254), s.ad_value(136), (-1.0)), 1.0, 755, (-1.0), 479, 1.0, 179, 1.0);
            s.store_mul_product3_mixed_aiii(170, A::limited_exp(s.ad_value(169)), 752, 158, 141, 1.0);
            s.store_add_scaled_product_indices(175, 175, 1.0, 170, 135, 1.0);
        }

        s.b[1513] = (((p.p61 != 0.0) && ((p.p70 == 2.0) || (p.p70 == 3.0))) && (((p.p62 == 2.0) || (p.p62 == 3.0)) || (p.p62 == 5.0)));
        s.store_scalar(1513, if s.b[1513] { 1.0 } else { 0.0 });

        if (s.b[1508] && s.b[1513]) {
            s.store_mul_ad_rhs(255, 757, {
                if (!(((1.0 + (s.v[862] * s.v[232])) - 1e-6) < ((-10000.0) * 0.001))) {
                    A::add_scaled_inputs(A::offset(A::mul(s.ad_value(862), s.ad_value(232)), ((1.0) + ((-1e-6)))), 0.5, A::sqrt_square_offset(A::offset(A::mul(s.ad_value(862), s.ad_value(232)), ((1.0) + ((-1e-6)))), ((4.0 * 0.001) * 0.001)), 0.5)
                } else {
                    {
                        if (((1.0 + (s.v[862] * s.v[232])) - 1e-6) < ((-10000.0) * 0.001)) {
                            A::div_scalar_offset_denominator(((-0.001) * 0.001), A::mul(s.ad_value(862), s.ad_value(232)), ((1.0) + ((-1e-6))), 1.0)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            });
        }

        s.b[1514] = ((s.v[756] <= 0.0) || (s.v[255] <= 0.0));
        s.store_scalar(1514, if s.b[1514] { 1.0 } else { 0.0 });

        if ((s.b[1508] && s.b[1513]) && s.b[1514]) {
            s.store_scalar(176, 0.0);
        }

        if ((s.b[1508] && s.b[1513]) && (!s.b[1514])) {
            s.store_div_scaled_inputs3_indices(169, 136, -1.0, 759, (-1.0), 479, 1.0, 168, 1.0);
        }

        if ((s.b[1508] && s.b[1513]) && (!s.b[1514])) {
            if (!(s.v[169] < ((-10000.0) * 0.01))) {
                s.store_scaled_add_sqrt_square_offset_rhs(169, 169, 169, ((4.0 * 0.01) * 0.01), 0.5);
            } else {
                if (s.v[169] < ((-10000.0) * 0.01)) {
                    s.store_div_from_scalar(169, ((-0.01) * 0.01), 169);
                } else {
                    s.store_scalar(169, 0.0);
                }
            }
        }

        if ((s.b[1508] && s.b[1513]) && (!s.b[1514])) {
            s.store_div_scaled_value_offset_denominator(170, s.ad_value(255), 1.0, s.ad_value(169), 0.001, 1.0);
            s.store_pow_indices(171, 169, 760);
            s.store_mul3_affine_lhs(172, 522, 522, -1.0, 0.0, 522);
            s.store_offset_add_ad(173, s.ad_value(758), A::abs(s.ad_value(172)), 1e-5);
        }

        if ((s.b[1508] && s.b[1513]) && (!s.b[1514])) {
            s.store_offset_ad(174, {
                if (!((s.v[172] / s.v[173]) < ((-10000.0) * 1e-6))) {
                    A::add_scaled_inputs(A::div(s.ad_value(172), s.ad_value(173)), 0.5, A::sqrt_square_offset(A::div(s.ad_value(172), s.ad_value(173)), ((4.0 * 1e-6) * 1e-6)), 0.5)
                } else {
                    {
                        if ((s.v[172] / s.v[173]) < ((-10000.0) * 1e-6)) {
                            A::div_from_scalar(((-1e-6) * 1e-6), A::div(s.ad_value(172), s.ad_value(173)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, (-1e-6));
        }

        if ((s.b[1508] && s.b[1513]) && (!s.b[1514])) {
            s.store_mul_ad_product_lhs(176, A::mul3(s.ad_value(756), s.ad_value(896), s.ad_value(171)), A::limited_exp_scaled_input(s.ad_value(170), -1.0), 174);
        }

        s.b[1516] = ((s.v[761] <= 0.0) || (s.v[250] <= 0.0));
        s.store_scalar(1516, if s.b[1516] { 1.0 } else { 0.0 });

    }

    pub(super) fn stamp_reactive_block_23(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (s.b[1508] && s.b[1516]) {
            s.store_scalar(175, 0.0);
        }

        if (s.b[1508] && (!s.b[1516])) {
            s.store_div_scaled_inputs3_indices(169, 134, -1.0, 764, (-1.0), 479, 1.0, 168, 1.0);
        }

        if (s.b[1508] && (!s.b[1516])) {
            if (!(s.v[169] < ((-10000.0) * 0.01))) {
                s.store_scaled_add_sqrt_square_offset_rhs(169, 169, 169, ((4.0 * 0.01) * 0.01), 0.5);
            } else {
                if (s.v[169] < ((-10000.0) * 0.01)) {
                    s.store_div_from_scalar(169, ((-0.01) * 0.01), 169);
                } else {
                    s.store_scalar(169, 0.0);
                }
            }
        }

        if (s.b[1508] && (!s.b[1516])) {
            s.store_div_scaled_value_offset_denominator(170, s.ad_value(250), 1.0, s.ad_value(169), 0.001, 1.0);
            s.store_pow_indices(171, 169, 765);
        }

        s.b[1517] = (p.p61 != 0.0);
        s.store_scalar(1517, if s.b[1517] { 1.0 } else { 0.0 });

        if ((s.b[1508] && (!s.b[1516])) && s.b[1517]) {
            s.store_mul3_affine_lhs(172, 521, 521, -1.0, 0.0, 521);
            s.store_offset_add_ad(173, s.ad_value(763), A::abs(s.ad_value(172)), 1e-5);
        }

        if ((s.b[1508] && (!s.b[1516])) && s.b[1517]) {
            s.store_offset_ad(174, {
                if (!((s.v[172] / s.v[173]) < ((-10000.0) * 1e-6))) {
                    A::add_scaled_inputs(A::div(s.ad_value(172), s.ad_value(173)), 0.5, A::sqrt_square_offset(A::div(s.ad_value(172), s.ad_value(173)), ((4.0 * 1e-6) * 1e-6)), 0.5)
                } else {
                    {
                        if ((s.v[172] / s.v[173]) < ((-10000.0) * 1e-6)) {
                            A::div_from_scalar(((-1e-6) * 1e-6), A::div(s.ad_value(172), s.ad_value(173)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, (-1e-6));
        }

        if ((s.b[1508] && (!s.b[1516])) && s.b[1517]) {
            s.store_mul_ad_product_lhs(175, A::mul3(s.ad_value(761), s.ad_value(158), s.ad_value(171)), A::limited_exp_scaled_input(s.ad_value(170), -1.0), 174);
        }

        if ((s.b[1508] && (!s.b[1516])) && (!s.b[1517])) {
            s.store_mul_ad_affine_product_lhs(175, A::mul3(s.ad_value(761), s.ad_value(158), s.ad_value(171)), A::limited_exp_scaled_input(s.ad_value(170), -1.0), -1.0, 0.0, 135);
        }

        s.b[1518] = ((p.p70 == 3.0) && (s.v[766] > 0.0));
        s.store_scalar(1518, if s.b[1518] { 1.0 } else { 0.0 });

        s.b[1519] = (p.p61 != 0.0);
        s.store_scalar(1519, if s.b[1519] { 1.0 } else { 0.0 });

        if ((s.b[1508] && s.b[1518]) && s.b[1519]) {
            s.store_mul_ad_rhs(253, 768, {
                if (!(((1.0 + (s.v[863] * s.v[232])) - 1e-6) < ((-10000.0) * 0.001))) {
                    A::add_scaled_inputs(A::offset(A::mul(s.ad_value(863), s.ad_value(232)), ((1.0) + ((-1e-6)))), 0.5, A::sqrt_square_offset(A::offset(A::mul(s.ad_value(863), s.ad_value(232)), ((1.0) + ((-1e-6)))), ((4.0 * 0.001) * 0.001)), 0.5)
                } else {
                    {
                        if (((1.0 + (s.v[863] * s.v[232])) - 1e-6) < ((-10000.0) * 0.001)) {
                            A::div_scalar_offset_denominator(((-0.001) * 0.001), A::mul(s.ad_value(863), s.ad_value(232)), ((1.0) + ((-1e-6))), 1.0)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            });
        }

        if ((s.b[1508] && s.b[1518]) && s.b[1519]) {
            s.store_div_scaled_inputs3_mixed_aiii(169, A::add_scaled_product(A::mul3(s.ad_value(767), s.ad_value(134), s.ad_value(134)), 1.0, s.ad_value(253), s.ad_value(134), (-1.0)), 1.0, 769, (-1.0), 479, 1.0, 179, 1.0);
            s.store_mul_product3_mixed_aiii(170, A::limited_exp(s.ad_value(169)), 766, 158, 141, 1.0);
            s.store_mul3_affine_lhs(172, 521, 521, -1.0, 0.0, 521);
            s.store_offset_add_ad(173, s.ad_value(763), A::abs(s.ad_value(172)), 1e-5);
        }

        if ((s.b[1508] && s.b[1518]) && s.b[1519]) {
            s.store_offset_ad(174, {
                if (!((s.v[172] / s.v[173]) < ((-10000.0) * 1e-6))) {
                    A::add_scaled_inputs(A::div(s.ad_value(172), s.ad_value(173)), 0.5, A::sqrt_square_offset(A::div(s.ad_value(172), s.ad_value(173)), ((4.0 * 1e-6) * 1e-6)), 0.5)
                } else {
                    {
                        if ((s.v[172] / s.v[173]) < ((-10000.0) * 1e-6)) {
                            A::div_from_scalar(((-1e-6) * 1e-6), A::div(s.ad_value(172), s.ad_value(173)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, (-1e-6));
        }

        if ((s.b[1508] && s.b[1518]) && s.b[1519]) {
            s.store_add_scaled_product_indices(175, 175, 1.0, 170, 174, 1.0);
        }

        if ((s.b[1508] && s.b[1518]) && (!s.b[1519])) {
            s.store_mul_ad_rhs(253, 768, {
                if (!(((1.0 + (s.v[863] * s.v[232])) - 1e-6) < ((-10000.0) * 0.001))) {
                    A::add_scaled_inputs(A::offset(A::mul(s.ad_value(863), s.ad_value(232)), ((1.0) + ((-1e-6)))), 0.5, A::sqrt_square_offset(A::offset(A::mul(s.ad_value(863), s.ad_value(232)), ((1.0) + ((-1e-6)))), ((4.0 * 0.001) * 0.001)), 0.5)
                } else {
                    {
                        if (((1.0 + (s.v[863] * s.v[232])) - 1e-6) < ((-10000.0) * 0.001)) {
                            A::div_scalar_offset_denominator(((-0.001) * 0.001), A::mul(s.ad_value(863), s.ad_value(232)), ((1.0) + ((-1e-6))), 1.0)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            });
        }

        if ((s.b[1508] && s.b[1518]) && (!s.b[1519])) {
            s.store_div_scaled_inputs3_mixed_aiii(169, A::add_scaled_product(A::mul3(s.ad_value(767), s.ad_value(134), s.ad_value(134)), 1.0, s.ad_value(253), s.ad_value(134), (-1.0)), 1.0, 769, (-1.0), 479, 1.0, 179, 1.0);
            s.store_mul_product3_mixed_aiii(170, A::limited_exp(s.ad_value(169)), 766, 158, 141, 1.0);
            s.store_add_scaled_product_indices(175, 175, 1.0, 170, 135, -1.0);
        }

        s.b[1520] = (((p.p61 != 0.0) && ((p.p70 == 2.0) || (p.p70 == 3.0))) && (((p.p62 == 2.0) || (p.p62 == 3.0)) || (p.p62 == 5.0)));
        s.store_scalar(1520, if s.b[1520] { 1.0 } else { 0.0 });

        if (s.b[1508] && s.b[1520]) {
            s.store_mul_ad_rhs(251, 771, {
                if (!(((1.0 + (s.v[862] * s.v[232])) - 1e-6) < ((-10000.0) * 0.001))) {
                    A::add_scaled_inputs(A::offset(A::mul(s.ad_value(862), s.ad_value(232)), ((1.0) + ((-1e-6)))), 0.5, A::sqrt_square_offset(A::offset(A::mul(s.ad_value(862), s.ad_value(232)), ((1.0) + ((-1e-6)))), ((4.0 * 0.001) * 0.001)), 0.5)
                } else {
                    {
                        if (((1.0 + (s.v[862] * s.v[232])) - 1e-6) < ((-10000.0) * 0.001)) {
                            A::div_scalar_offset_denominator(((-0.001) * 0.001), A::mul(s.ad_value(862), s.ad_value(232)), ((1.0) + ((-1e-6))), 1.0)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            });
        }

        s.b[1521] = ((s.v[770] <= 0.0) || (s.v[251] <= 0.0));
        s.store_scalar(1521, if s.b[1521] { 1.0 } else { 0.0 });

        if ((s.b[1508] && s.b[1520]) && s.b[1521]) {
            s.store_scalar(176, 0.0);
        }

        if ((s.b[1508] && s.b[1520]) && (!s.b[1521])) {
            s.store_div_scaled_inputs3_indices(169, 134, -1.0, 773, (-1.0), 479, 1.0, 168, 1.0);
        }

        if ((s.b[1508] && s.b[1520]) && (!s.b[1521])) {
            if (!(s.v[169] < ((-10000.0) * 0.01))) {
                s.store_scaled_add_sqrt_square_offset_rhs(169, 169, 169, ((4.0 * 0.01) * 0.01), 0.5);
            } else {
                if (s.v[169] < ((-10000.0) * 0.01)) {
                    s.store_div_from_scalar(169, ((-0.01) * 0.01), 169);
                } else {
                    s.store_scalar(169, 0.0);
                }
            }
        }

        if ((s.b[1508] && s.b[1520]) && (!s.b[1521])) {
            s.store_div_scaled_value_offset_denominator(170, s.ad_value(251), 1.0, s.ad_value(169), 0.001, 1.0);
            s.store_pow_indices(171, 169, 774);
            s.store_mul3_affine_lhs(172, 521, 521, -1.0, 0.0, 521);
            s.store_offset_add_ad(173, s.ad_value(772), A::abs(s.ad_value(172)), 1e-5);
        }

        if ((s.b[1508] && s.b[1520]) && (!s.b[1521])) {
            s.store_offset_ad(174, {
                if (!((s.v[172] / s.v[173]) < ((-10000.0) * 1e-6))) {
                    A::add_scaled_inputs(A::div(s.ad_value(172), s.ad_value(173)), 0.5, A::sqrt_square_offset(A::div(s.ad_value(172), s.ad_value(173)), ((4.0 * 1e-6) * 1e-6)), 0.5)
                } else {
                    {
                        if ((s.v[172] / s.v[173]) < ((-10000.0) * 1e-6)) {
                            A::div_from_scalar(((-1e-6) * 1e-6), A::div(s.ad_value(172), s.ad_value(173)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, (-1e-6));
        }

        if ((s.b[1508] && s.b[1520]) && (!s.b[1521])) {
            s.store_mul_ad_product_lhs(176, A::mul3(s.ad_value(770), s.ad_value(896), s.ad_value(171)), A::limited_exp_scaled_input(s.ad_value(170), -1.0), 174);
        }

        s.b[1523] = (p.p61 != 0.0);
        s.store_scalar(1523, if s.b[1523] { 1.0 } else { 0.0 });

        s.b[1524] = (s.v[537] > 0.0);
        s.store_scalar(1524, if s.b[1524] { 1.0 } else { 0.0 });

        s.b[1525] = (s.v[521] < s.v[543]);
        s.store_scalar(1525, if s.b[1525] { 1.0 } else { 0.0 });

        if ((s.b[1523] && s.b[1524]) && s.b[1525]) {
            s.store_div(168, 521, 539);
            s.store_offset_limited_exp(169, 168, (-1.0));
            s.store_add_scaled_product_right_sub(170, 542, 1.0, 541, 521, 543, 1.0);
        }

        s.b[1526] = (s.v[521] <= s.v[546]);
        s.store_scalar(1526, if s.b[1526] { 1.0 } else { 0.0 });

        if (((s.b[1523] && s.b[1524]) && (!s.b[1525])) && s.b[1526]) {
            s.store_div(168, 521, 539);
            s.store_div_scaled_offset_numerator(169, s.ad_value(521), 1.0, p.p1626, s.ad_value(539), 1.0);
            s.store_limited_exp_neg_input(170, 169);
        }

        s.b[1527] = (s.v[281] > 0.0);
        s.store_scalar(1527, if s.b[1527] { 1.0 } else { 0.0 });

        s.b[1528] = ((p.p1643 - s.v[521]) < (p.p1643 * 0.001));
        s.store_scalar(1528, if s.b[1528] { 1.0 } else { 0.0 });

        if ((s.b[1523] && s.b[1527]) && s.b[1528]) {
            s.store_div_scaled_value_by_product(168, s.ad_value(521), -1.0, s.ad_value(180), s.ad_value(287), 1.0);
            s.store_offset_ad(169, A::limited_exp_scaled_input(s.ad_value(168), 1000.0), (-1.0));
        }

        if ((s.b[1523] && s.b[1527]) && (!s.b[1528])) {
            s.store_div_scaled_value_by_product(168, s.ad_value(521), -1.0, s.ad_value(180), s.ad_value(287), 1.0);
            s.store_offset_ad(169, A::limited_exp_div_scaled_inputs(s.ad_value(168), p.p1643, A::sub_from_scalar(p.p1643, s.ad_value(521)), 1.0), (-1.0));
        }

        s.b[1529] = (s.v[283] > 0.0);
        s.store_scalar(1529, if s.b[1529] { 1.0 } else { 0.0 });

        s.b[1530] = ((p.p1645 - s.v[521]) < (p.p1645 * 0.001));
        s.store_scalar(1530, if s.b[1530] { 1.0 } else { 0.0 });

        if ((s.b[1523] && s.b[1529]) && s.b[1530]) {
            s.store_div_scaled_value_by_product(168, s.ad_value(521), -1.0, s.ad_value(180), s.ad_value(289), 1.0);
            s.store_offset_ad(169, A::limited_exp_scaled_input(s.ad_value(168), 1000.0), (-1.0));
        }

        if ((s.b[1523] && s.b[1529]) && (!s.b[1530])) {
            s.store_div_scaled_value_by_product(168, s.ad_value(521), -1.0, s.ad_value(180), s.ad_value(289), 1.0);
            s.store_offset_ad(169, A::limited_exp_div_scaled_inputs(s.ad_value(168), p.p1645, A::sub_from_scalar(p.p1645, s.ad_value(521)), 1.0), (-1.0));
        }

        s.b[1531] = (s.v[285] > 0.0);
        s.store_scalar(1531, if s.b[1531] { 1.0 } else { 0.0 });

        s.b[1532] = ((p.p1647 - s.v[521]) < (p.p1647 * 0.001));
        s.store_scalar(1532, if s.b[1532] { 1.0 } else { 0.0 });

        if ((s.b[1523] && s.b[1531]) && s.b[1532]) {
            s.store_div_scaled_value_by_product(168, s.ad_value(521), -1.0, s.ad_value(180), s.ad_value(291), 1.0);
            s.store_offset_ad(169, A::limited_exp_scaled_input(s.ad_value(168), 1000.0), (-1.0));
        }

        if ((s.b[1523] && s.b[1531]) && (!s.b[1532])) {
            s.store_div_scaled_value_by_product(168, s.ad_value(521), -1.0, s.ad_value(180), s.ad_value(291), 1.0);
            s.store_offset_ad(169, A::limited_exp_div_scaled_inputs(s.ad_value(168), p.p1647, A::sub_from_scalar(p.p1647, s.ad_value(521)), 1.0), (-1.0));
        }

        s.b[1533] = (s.v[538] > 0.0);
        s.store_scalar(1533, if s.b[1533] { 1.0 } else { 0.0 });

        s.b[1534] = (s.v[522] < s.v[550]);
        s.store_scalar(1534, if s.b[1534] { 1.0 } else { 0.0 });

        if ((s.b[1523] && s.b[1533]) && s.b[1534]) {
            s.store_div(168, 522, 540);
            s.store_offset_limited_exp(169, 168, (-1.0));
            s.store_add_scaled_product_right_sub(170, 549, 1.0, 548, 522, 550, 1.0);
        }

        s.b[1535] = (s.v[522] <= s.v[553]);
        s.store_scalar(1535, if s.b[1535] { 1.0 } else { 0.0 });

        if (((s.b[1523] && s.b[1533]) && (!s.b[1534])) && s.b[1535]) {
            s.store_div(168, 522, 540);
            s.store_div_scaled_offset_numerator(169, s.ad_value(522), 1.0, p.p1627, s.ad_value(540), 1.0);
            s.store_limited_exp_neg_input(170, 169);
        }

        s.b[1536] = (s.v[282] > 0.0);
        s.store_scalar(1536, if s.b[1536] { 1.0 } else { 0.0 });

        s.b[1537] = ((p.p1644 - s.v[522]) < (p.p1644 * 0.001));
        s.store_scalar(1537, if s.b[1537] { 1.0 } else { 0.0 });

        if ((s.b[1523] && s.b[1536]) && s.b[1537]) {
            s.store_div_scaled_value_by_product(168, s.ad_value(522), -1.0, s.ad_value(180), s.ad_value(288), 1.0);
            s.store_offset_ad(169, A::limited_exp_scaled_input(s.ad_value(168), 1000.0), (-1.0));
        }

        if ((s.b[1523] && s.b[1536]) && (!s.b[1537])) {
            s.store_div_scaled_value_by_product(168, s.ad_value(522), -1.0, s.ad_value(180), s.ad_value(288), 1.0);
            s.store_offset_ad(169, A::limited_exp_div_scaled_inputs(s.ad_value(168), p.p1644, A::sub_from_scalar(p.p1644, s.ad_value(522)), 1.0), (-1.0));
        }

        s.b[1538] = (s.v[284] > 0.0);
        s.store_scalar(1538, if s.b[1538] { 1.0 } else { 0.0 });

        s.b[1539] = ((p.p1646 - s.v[522]) < (p.p1646 * 0.001));
        s.store_scalar(1539, if s.b[1539] { 1.0 } else { 0.0 });

        if ((s.b[1523] && s.b[1538]) && s.b[1539]) {
            s.store_div_scaled_value_by_product(168, s.ad_value(522), -1.0, s.ad_value(180), s.ad_value(290), 1.0);
            s.store_offset_ad(169, A::limited_exp_scaled_input(s.ad_value(168), 1000.0), (-1.0));
        }

        if ((s.b[1523] && s.b[1538]) && (!s.b[1539])) {
            s.store_div_scaled_value_by_product(168, s.ad_value(522), -1.0, s.ad_value(180), s.ad_value(290), 1.0);
            s.store_offset_ad(169, A::limited_exp_div_scaled_inputs(s.ad_value(168), p.p1646, A::sub_from_scalar(p.p1646, s.ad_value(522)), 1.0), (-1.0));
        }

        s.b[1540] = (s.v[286] > 0.0);
        s.store_scalar(1540, if s.b[1540] { 1.0 } else { 0.0 });

        s.b[1541] = ((p.p1648 - s.v[522]) < (p.p1648 * 0.001));
        s.store_scalar(1541, if s.b[1541] { 1.0 } else { 0.0 });

        if ((s.b[1523] && s.b[1540]) && s.b[1541]) {
            s.store_div_scaled_value_by_product(168, s.ad_value(522), -1.0, s.ad_value(180), s.ad_value(292), 1.0);
            s.store_offset_ad(169, A::limited_exp_scaled_input(s.ad_value(168), 1000.0), (-1.0));
        }

        if ((s.b[1523] && s.b[1540]) && (!s.b[1541])) {
            s.store_div_scaled_value_by_product(168, s.ad_value(522), -1.0, s.ad_value(180), s.ad_value(292), 1.0);
            s.store_offset_ad(169, A::limited_exp_div_scaled_inputs(s.ad_value(168), p.p1648, A::sub_from_scalar(p.p1648, s.ad_value(522)), 1.0), (-1.0));
        }

        s.b[1550] = (s.v[523] > 0.0);
        s.store_scalar(1550, if s.b[1550] { 1.0 } else { 0.0 });

        if (s.b[1523] && s.b[1550]) {
            s.store_div(1542, 521, 269);
        }

        s.b[1551] = (s.v[1542] < 0.9);
        s.store_scalar(1551, if s.b[1551] { 1.0 } else { 0.0 });

        s.b[1552] = (p.p1602 > 0.0);
        s.store_scalar(1552, if s.b[1552] { 1.0 } else { 0.0 });

        s.b[1553] = (s.v[521] > s.v[557]);
        s.store_scalar(1553, if s.b[1553] { 1.0 } else { 0.0 });

        if ((((s.b[1523] && s.b[1550]) && s.b[1551]) && s.b[1552]) && s.b[1553]) {
            s.store_sub_from_scalar(1547, 1.0, 1542);
        }

        s.b[1554] = (p.p1596 != 1.0);
        s.store_scalar(1554, if s.b[1554] { 1.0 } else { 0.0 });

        s.b[1555] = (p.p1596 == 0.5);
        s.store_scalar(1555, if s.b[1555] { 1.0 } else { 0.0 });

        if ((((((s.b[1523] && s.b[1550]) && s.b[1551]) && s.b[1552]) && s.b[1553]) && s.b[1554]) && s.b[1555]) {
            s.store_div_from_scalar_sqrt_ad(1548, 1.0, s.ad_value(1547));
        }

        if ((((((s.b[1523] && s.b[1550]) && s.b[1551]) && s.b[1552]) && s.b[1553]) && s.b[1554]) && (!s.b[1555])) {
            s.store_powf(1548, 1547, (-p.p1596));
        }

        if (((((s.b[1523] && s.b[1550]) && s.b[1551]) && s.b[1552]) && s.b[1553]) && s.b[1554]) {
            s.store_mul_ad_affine_product_rhs(530, 269, s.ad_value(523), A::sub_from_scalar(1.0, A::mul(s.ad_value(1547), s.ad_value(1548))), 1.0 / ((1.0 - p.p1596)), 0.0);
        }

        if (((((s.b[1523] && s.b[1550]) && s.b[1551]) && s.b[1552]) && s.b[1553]) && (!s.b[1554])) {
            s.store_mul_ad_affine_product_rhs(530, 269, s.ad_value(523), {
                if (!(s.v[1547] > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if (s.v[1547] > 1e-38) {
                            A::ln(s.ad_value(1547))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, -1.0, 0.0);
        }

        if ((((s.b[1523] && s.b[1550]) && s.b[1551]) && s.b[1552]) && (!s.b[1553])) {
            s.store_sub_from_scalar_div_indices(1547, 1.0, 557, 269);
        }

        s.b[1556] = (p.p1596 != 1.0);
        s.store_scalar(1556, if s.b[1556] { 1.0 } else { 0.0 });

        s.b[1557] = (p.p1596 == 0.5);
        s.store_scalar(1557, if s.b[1557] { 1.0 } else { 0.0 });

        if ((((((s.b[1523] && s.b[1550]) && s.b[1551]) && s.b[1552]) && (!s.b[1553])) && s.b[1556]) && s.b[1557]) {
            s.store_div_from_scalar_sqrt_ad(1548, 1.0, s.ad_value(1547));
        }

        if ((((((s.b[1523] && s.b[1550]) && s.b[1551]) && s.b[1552]) && (!s.b[1553])) && s.b[1556]) && (!s.b[1557])) {
            s.store_powf(1548, 1547, (-p.p1596));
        }

        if (((((s.b[1523] && s.b[1550]) && s.b[1551]) && s.b[1552]) && (!s.b[1553])) && s.b[1556]) {
            s.store_mul_ad_affine_product_rhs(1549, 269, s.ad_value(523), A::sub_from_scalar(1.0, A::mul(s.ad_value(1547), s.ad_value(1548))), 1.0 / ((1.0 - p.p1596)), 0.0);
        }

    }

    pub(super) fn stamp_reactive_block_24(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (((((s.b[1523] && s.b[1550]) && s.b[1551]) && s.b[1552]) && (!s.b[1553])) && (!s.b[1556])) {
            s.store_mul_ad_affine_product_rhs(1549, 269, s.ad_value(523), {
                if (!(s.v[1547] > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if (s.v[1547] > 1e-38) {
                            A::ln(s.ad_value(1547))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, -1.0, 0.0);
        }

        if ((((s.b[1523] && s.b[1550]) && s.b[1551]) && s.b[1552]) && (!s.b[1553])) {
            s.store_sub_from_scalar_ad(1547, 1.0, A::div_scaled_inputs2(s.ad_value(521), 1.0, s.ad_value(557), (-1.0), s.ad_value(558), 1.0));
        }

        s.b[1558] = (p.p1608 != 1.0);
        s.store_scalar(1558, if s.b[1558] { 1.0 } else { 0.0 });

        s.b[1559] = (p.p1608 == 0.5);
        s.store_scalar(1559, if s.b[1559] { 1.0 } else { 0.0 });

        if ((((((s.b[1523] && s.b[1550]) && s.b[1551]) && s.b[1552]) && (!s.b[1553])) && s.b[1558]) && s.b[1559]) {
            s.store_div_from_scalar_sqrt_ad(1548, 1.0, s.ad_value(1547));
        }

        if ((((((s.b[1523] && s.b[1550]) && s.b[1551]) && s.b[1552]) && (!s.b[1553])) && s.b[1558]) && (!s.b[1559])) {
            s.store_powf(1548, 1547, (-p.p1608));
        }

        if (((((s.b[1523] && s.b[1550]) && s.b[1551]) && s.b[1552]) && (!s.b[1553])) && s.b[1558]) {
            s.store_add_product3_rhs_mixed_iia(530, 1549, 558, 523, A::sub_from_scalar(1.0, A::mul(s.ad_value(1547), s.ad_value(1548))), (p.p1602 * 1.0 / ((1.0 - p.p1608))));
        }

        if (((((s.b[1523] && s.b[1550]) && s.b[1551]) && s.b[1552]) && (!s.b[1553])) && (!s.b[1558])) {
            s.store_sub_ad_rhs(530, 1549, A::mul3_scaled_output(s.ad_value(558), s.ad_value(523), {
                if (!(s.v[1547] > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if (s.v[1547] > 1e-38) {
                            A::ln(s.ad_value(1547))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, p.p1602));
        }

        if (((s.b[1523] && s.b[1550]) && s.b[1551]) && (!s.b[1552])) {
            s.store_sub_from_scalar(1547, 1.0, 1542);
        }

        s.b[1560] = (p.p1596 != 1.0);
        s.store_scalar(1560, if s.b[1560] { 1.0 } else { 0.0 });

        s.b[1561] = (p.p1596 == 0.5);
        s.store_scalar(1561, if s.b[1561] { 1.0 } else { 0.0 });

        if (((((s.b[1523] && s.b[1550]) && s.b[1551]) && (!s.b[1552])) && s.b[1560]) && s.b[1561]) {
            s.store_div_from_scalar_sqrt_ad(1548, 1.0, s.ad_value(1547));
        }

        if (((((s.b[1523] && s.b[1550]) && s.b[1551]) && (!s.b[1552])) && s.b[1560]) && (!s.b[1561])) {
            s.store_powf(1548, 1547, (-p.p1596));
        }

        if ((((s.b[1523] && s.b[1550]) && s.b[1551]) && (!s.b[1552])) && s.b[1560]) {
            s.store_mul_ad_affine_product_rhs(530, 269, s.ad_value(523), A::sub_from_scalar(1.0, A::mul(s.ad_value(1547), s.ad_value(1548))), 1.0 / ((1.0 - p.p1596)), 0.0);
        }

        if ((((s.b[1523] && s.b[1550]) && s.b[1551]) && (!s.b[1552])) && (!s.b[1560])) {
            s.store_mul_ad_affine_product_rhs(530, 269, s.ad_value(523), {
                if (!(s.v[1547] > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if (s.v[1547] > 1e-38) {
                            A::ln(s.ad_value(1547))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, -1.0, 0.0);
        }

        s.b[1562] = (p.p1596 != 1.0);
        s.store_scalar(1562, if s.b[1562] { 1.0 } else { 0.0 });

        s.b[1563] = (p.p1596 == 0.5);
        s.store_scalar(1563, if s.b[1563] { 1.0 } else { 0.0 });

        if ((((s.b[1523] && s.b[1550]) && (!s.b[1551])) && s.b[1562]) && s.b[1563]) {
            s.store_scalar(1543, (1.0 / ((0.1) as f64).sqrt()));
        }

        if ((((s.b[1523] && s.b[1550]) && (!s.b[1551])) && s.b[1562]) && (!s.b[1563])) {
            s.store_scalar(1543, ((0.1) as f64).powf((-p.p1596)));
        }

        if (((s.b[1523] && s.b[1550]) && (!s.b[1551])) && s.b[1562]) {
            s.store_scalar(1544, (1.0 / (1.0 - p.p1596)));
            s.store_mul_sub_from_scalar_ad_rhs(1546, 1544, 1.0, A::scale(s.ad_value(1543), ((0.05 * p.p1596) * (1.0 + p.p1596))));
        }

        if (((s.b[1523] && s.b[1550]) && (!s.b[1551])) && (!s.b[1562])) {
            s.store_scalar(1543, 10.0);
            s.store_scalar(1546, (1.5 - ((0.1) as f64).ln()));
        }

        if ((s.b[1523] && s.b[1550]) && (!s.b[1551])) {
            s.store_mul_ad_product_rhs(1545, 1543, A::offset(s.ad_value(1542), (-1.0)), A::scale_offset(s.ad_value(1542), (5.0 * p.p1596), (((((-1.0)) * ((5.0 * p.p1596)))) + ((1.0 + p.p1596)))));
            s.store_mul_ad_product_rhs_mixed_ia(530, 269, 523, A::add(s.ad_value(1545), s.ad_value(1546)));
        }

        if (s.b[1523] && (!s.b[1550])) {
            s.store_scalar(530, 0.0);
        }

        s.b[1572] = (s.v[524] > 0.0);
        s.store_scalar(1572, if s.b[1572] { 1.0 } else { 0.0 });

        if (s.b[1523] && s.b[1572]) {
            s.store_div(1564, 521, 270);
        }

        s.b[1573] = (s.v[1564] < 0.9);
        s.store_scalar(1573, if s.b[1573] { 1.0 } else { 0.0 });

        s.b[1574] = (p.p1604 > 0.0);
        s.store_scalar(1574, if s.b[1574] { 1.0 } else { 0.0 });

        s.b[1575] = (s.v[521] > s.v[559]);
        s.store_scalar(1575, if s.b[1575] { 1.0 } else { 0.0 });

        if ((((s.b[1523] && s.b[1572]) && s.b[1573]) && s.b[1574]) && s.b[1575]) {
            s.store_sub_from_scalar(1569, 1.0, 1564);
        }

        s.b[1576] = (p.p1598 != 1.0);
        s.store_scalar(1576, if s.b[1576] { 1.0 } else { 0.0 });

        s.b[1577] = (p.p1598 == 0.5);
        s.store_scalar(1577, if s.b[1577] { 1.0 } else { 0.0 });

        if ((((((s.b[1523] && s.b[1572]) && s.b[1573]) && s.b[1574]) && s.b[1575]) && s.b[1576]) && s.b[1577]) {
            s.store_div_from_scalar_sqrt_ad(1570, 1.0, s.ad_value(1569));
        }

        if ((((((s.b[1523] && s.b[1572]) && s.b[1573]) && s.b[1574]) && s.b[1575]) && s.b[1576]) && (!s.b[1577])) {
            s.store_powf(1570, 1569, (-p.p1598));
        }

        if (((((s.b[1523] && s.b[1572]) && s.b[1573]) && s.b[1574]) && s.b[1575]) && s.b[1576]) {
            s.store_mul_ad_affine_product_rhs(531, 270, s.ad_value(524), A::sub_from_scalar(1.0, A::mul(s.ad_value(1569), s.ad_value(1570))), 1.0 / ((1.0 - p.p1598)), 0.0);
        }

        if (((((s.b[1523] && s.b[1572]) && s.b[1573]) && s.b[1574]) && s.b[1575]) && (!s.b[1576])) {
            s.store_mul_ad_affine_product_rhs(531, 270, s.ad_value(524), {
                if (!(s.v[1569] > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if (s.v[1569] > 1e-38) {
                            A::ln(s.ad_value(1569))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, -1.0, 0.0);
        }

        if ((((s.b[1523] && s.b[1572]) && s.b[1573]) && s.b[1574]) && (!s.b[1575])) {
            s.store_sub_from_scalar_div_indices(1569, 1.0, 559, 270);
        }

        s.b[1578] = (p.p1598 != 1.0);
        s.store_scalar(1578, if s.b[1578] { 1.0 } else { 0.0 });

        s.b[1579] = (p.p1598 == 0.5);
        s.store_scalar(1579, if s.b[1579] { 1.0 } else { 0.0 });

        if ((((((s.b[1523] && s.b[1572]) && s.b[1573]) && s.b[1574]) && (!s.b[1575])) && s.b[1578]) && s.b[1579]) {
            s.store_div_from_scalar_sqrt_ad(1570, 1.0, s.ad_value(1569));
        }

        if ((((((s.b[1523] && s.b[1572]) && s.b[1573]) && s.b[1574]) && (!s.b[1575])) && s.b[1578]) && (!s.b[1579])) {
            s.store_powf(1570, 1569, (-p.p1598));
        }

        if (((((s.b[1523] && s.b[1572]) && s.b[1573]) && s.b[1574]) && (!s.b[1575])) && s.b[1578]) {
            s.store_mul_ad_affine_product_rhs(1571, 270, s.ad_value(524), A::sub_from_scalar(1.0, A::mul(s.ad_value(1569), s.ad_value(1570))), 1.0 / ((1.0 - p.p1598)), 0.0);
        }

        if (((((s.b[1523] && s.b[1572]) && s.b[1573]) && s.b[1574]) && (!s.b[1575])) && (!s.b[1578])) {
            s.store_mul_ad_affine_product_rhs(1571, 270, s.ad_value(524), {
                if (!(s.v[1569] > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if (s.v[1569] > 1e-38) {
                            A::ln(s.ad_value(1569))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, -1.0, 0.0);
        }

        if ((((s.b[1523] && s.b[1572]) && s.b[1573]) && s.b[1574]) && (!s.b[1575])) {
            s.store_sub_from_scalar_ad(1569, 1.0, A::div_scaled_inputs2(s.ad_value(521), 1.0, s.ad_value(559), (-1.0), s.ad_value(560), 1.0));
        }

        s.b[1580] = (p.p1610 != 1.0);
        s.store_scalar(1580, if s.b[1580] { 1.0 } else { 0.0 });

        s.b[1581] = (p.p1610 == 0.5);
        s.store_scalar(1581, if s.b[1581] { 1.0 } else { 0.0 });

        if ((((((s.b[1523] && s.b[1572]) && s.b[1573]) && s.b[1574]) && (!s.b[1575])) && s.b[1580]) && s.b[1581]) {
            s.store_div_from_scalar_sqrt_ad(1570, 1.0, s.ad_value(1569));
        }

        if ((((((s.b[1523] && s.b[1572]) && s.b[1573]) && s.b[1574]) && (!s.b[1575])) && s.b[1580]) && (!s.b[1581])) {
            s.store_powf(1570, 1569, (-p.p1610));
        }

        if (((((s.b[1523] && s.b[1572]) && s.b[1573]) && s.b[1574]) && (!s.b[1575])) && s.b[1580]) {
            s.store_add_product3_rhs_mixed_iia(531, 1571, 560, 524, A::sub_from_scalar(1.0, A::mul(s.ad_value(1569), s.ad_value(1570))), (p.p1604 * 1.0 / ((1.0 - p.p1610))));
        }

        if (((((s.b[1523] && s.b[1572]) && s.b[1573]) && s.b[1574]) && (!s.b[1575])) && (!s.b[1580])) {
            s.store_sub_ad_rhs(531, 1571, A::mul3_scaled_output(s.ad_value(560), s.ad_value(524), {
                if (!(s.v[1569] > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if (s.v[1569] > 1e-38) {
                            A::ln(s.ad_value(1569))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, p.p1604));
        }

        if (((s.b[1523] && s.b[1572]) && s.b[1573]) && (!s.b[1574])) {
            s.store_sub_from_scalar(1569, 1.0, 1564);
        }

        s.b[1582] = (p.p1598 != 1.0);
        s.store_scalar(1582, if s.b[1582] { 1.0 } else { 0.0 });

        s.b[1583] = (p.p1598 == 0.5);
        s.store_scalar(1583, if s.b[1583] { 1.0 } else { 0.0 });

        if (((((s.b[1523] && s.b[1572]) && s.b[1573]) && (!s.b[1574])) && s.b[1582]) && s.b[1583]) {
            s.store_div_from_scalar_sqrt_ad(1570, 1.0, s.ad_value(1569));
        }

        if (((((s.b[1523] && s.b[1572]) && s.b[1573]) && (!s.b[1574])) && s.b[1582]) && (!s.b[1583])) {
            s.store_powf(1570, 1569, (-p.p1598));
        }

        if ((((s.b[1523] && s.b[1572]) && s.b[1573]) && (!s.b[1574])) && s.b[1582]) {
            s.store_mul_ad_affine_product_rhs(531, 270, s.ad_value(524), A::sub_from_scalar(1.0, A::mul(s.ad_value(1569), s.ad_value(1570))), 1.0 / ((1.0 - p.p1598)), 0.0);
        }

        if ((((s.b[1523] && s.b[1572]) && s.b[1573]) && (!s.b[1574])) && (!s.b[1582])) {
            s.store_mul_ad_affine_product_rhs(531, 270, s.ad_value(524), {
                if (!(s.v[1569] > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if (s.v[1569] > 1e-38) {
                            A::ln(s.ad_value(1569))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, -1.0, 0.0);
        }

        s.b[1584] = (p.p1598 != 1.0);
        s.store_scalar(1584, if s.b[1584] { 1.0 } else { 0.0 });

        s.b[1585] = (p.p1598 == 0.5);
        s.store_scalar(1585, if s.b[1585] { 1.0 } else { 0.0 });

        if ((((s.b[1523] && s.b[1572]) && (!s.b[1573])) && s.b[1584]) && s.b[1585]) {
            s.store_scalar(1565, (1.0 / ((0.1) as f64).sqrt()));
        }

        if ((((s.b[1523] && s.b[1572]) && (!s.b[1573])) && s.b[1584]) && (!s.b[1585])) {
            s.store_scalar(1565, ((0.1) as f64).powf((-p.p1598)));
        }

        if (((s.b[1523] && s.b[1572]) && (!s.b[1573])) && s.b[1584]) {
            s.store_scalar(1566, (1.0 / (1.0 - p.p1598)));
            s.store_mul_sub_from_scalar_ad_rhs(1568, 1566, 1.0, A::scale(s.ad_value(1565), ((0.05 * p.p1598) * (1.0 + p.p1598))));
        }

        if (((s.b[1523] && s.b[1572]) && (!s.b[1573])) && (!s.b[1584])) {
            s.store_scalar(1565, 10.0);
            s.store_scalar(1568, (1.5 - ((0.1) as f64).ln()));
        }

        if ((s.b[1523] && s.b[1572]) && (!s.b[1573])) {
            s.store_mul_ad_product_rhs(1567, 1565, A::offset(s.ad_value(1564), (-1.0)), A::scale_offset(s.ad_value(1564), (5.0 * p.p1598), (((((-1.0)) * ((5.0 * p.p1598)))) + ((1.0 + p.p1598)))));
            s.store_mul_ad_product_rhs_mixed_ia(531, 270, 524, A::add(s.ad_value(1567), s.ad_value(1568)));
        }

        if (s.b[1523] && (!s.b[1572])) {
            s.store_scalar(531, 0.0);
        }

        s.b[1594] = (s.v[525] > 0.0);
        s.store_scalar(1594, if s.b[1594] { 1.0 } else { 0.0 });

        if (s.b[1523] && s.b[1594]) {
            s.store_div(1586, 521, 271);
        }

        s.b[1595] = (s.v[1586] < 0.9);
        s.store_scalar(1595, if s.b[1595] { 1.0 } else { 0.0 });

        s.b[1596] = (p.p1606 > 0.0);
        s.store_scalar(1596, if s.b[1596] { 1.0 } else { 0.0 });

        s.b[1597] = (s.v[521] > s.v[561]);
        s.store_scalar(1597, if s.b[1597] { 1.0 } else { 0.0 });

        if ((((s.b[1523] && s.b[1594]) && s.b[1595]) && s.b[1596]) && s.b[1597]) {
            s.store_sub_from_scalar(1591, 1.0, 1586);
        }

        s.b[1598] = (p.p1600 != 1.0);
        s.store_scalar(1598, if s.b[1598] { 1.0 } else { 0.0 });

        s.b[1599] = (p.p1600 == 0.5);
        s.store_scalar(1599, if s.b[1599] { 1.0 } else { 0.0 });

        if ((((((s.b[1523] && s.b[1594]) && s.b[1595]) && s.b[1596]) && s.b[1597]) && s.b[1598]) && s.b[1599]) {
            s.store_div_from_scalar_sqrt_ad(1592, 1.0, s.ad_value(1591));
        }

        if ((((((s.b[1523] && s.b[1594]) && s.b[1595]) && s.b[1596]) && s.b[1597]) && s.b[1598]) && (!s.b[1599])) {
            s.store_powf(1592, 1591, (-p.p1600));
        }

        if (((((s.b[1523] && s.b[1594]) && s.b[1595]) && s.b[1596]) && s.b[1597]) && s.b[1598]) {
            s.store_mul_ad_affine_product_rhs(532, 271, s.ad_value(525), A::sub_from_scalar(1.0, A::mul(s.ad_value(1591), s.ad_value(1592))), 1.0 / ((1.0 - p.p1600)), 0.0);
        }

        if (((((s.b[1523] && s.b[1594]) && s.b[1595]) && s.b[1596]) && s.b[1597]) && (!s.b[1598])) {
            s.store_mul_ad_affine_product_rhs(532, 271, s.ad_value(525), {
                if (!(s.v[1591] > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if (s.v[1591] > 1e-38) {
                            A::ln(s.ad_value(1591))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, -1.0, 0.0);
        }

        if ((((s.b[1523] && s.b[1594]) && s.b[1595]) && s.b[1596]) && (!s.b[1597])) {
            s.store_sub_from_scalar_div_indices(1591, 1.0, 561, 271);
        }

        s.b[1600] = (p.p1600 != 1.0);
        s.store_scalar(1600, if s.b[1600] { 1.0 } else { 0.0 });

        s.b[1601] = (p.p1600 == 0.5);
        s.store_scalar(1601, if s.b[1601] { 1.0 } else { 0.0 });

        if ((((((s.b[1523] && s.b[1594]) && s.b[1595]) && s.b[1596]) && (!s.b[1597])) && s.b[1600]) && s.b[1601]) {
            s.store_div_from_scalar_sqrt_ad(1592, 1.0, s.ad_value(1591));
        }

        if ((((((s.b[1523] && s.b[1594]) && s.b[1595]) && s.b[1596]) && (!s.b[1597])) && s.b[1600]) && (!s.b[1601])) {
            s.store_powf(1592, 1591, (-p.p1600));
        }

        if (((((s.b[1523] && s.b[1594]) && s.b[1595]) && s.b[1596]) && (!s.b[1597])) && s.b[1600]) {
            s.store_mul_ad_affine_product_rhs(1593, 271, s.ad_value(525), A::sub_from_scalar(1.0, A::mul(s.ad_value(1591), s.ad_value(1592))), 1.0 / ((1.0 - p.p1600)), 0.0);
        }

        if (((((s.b[1523] && s.b[1594]) && s.b[1595]) && s.b[1596]) && (!s.b[1597])) && (!s.b[1600])) {
            s.store_mul_ad_affine_product_rhs(1593, 271, s.ad_value(525), {
                if (!(s.v[1591] > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if (s.v[1591] > 1e-38) {
                            A::ln(s.ad_value(1591))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, -1.0, 0.0);
        }

        if ((((s.b[1523] && s.b[1594]) && s.b[1595]) && s.b[1596]) && (!s.b[1597])) {
            s.store_sub_from_scalar_ad(1591, 1.0, A::div_scaled_inputs2(s.ad_value(521), 1.0, s.ad_value(561), (-1.0), s.ad_value(562), 1.0));
        }

        s.b[1602] = (p.p1612 != 1.0);
        s.store_scalar(1602, if s.b[1602] { 1.0 } else { 0.0 });

        s.b[1603] = (p.p1612 == 0.5);
        s.store_scalar(1603, if s.b[1603] { 1.0 } else { 0.0 });

        if ((((((s.b[1523] && s.b[1594]) && s.b[1595]) && s.b[1596]) && (!s.b[1597])) && s.b[1602]) && s.b[1603]) {
            s.store_div_from_scalar_sqrt_ad(1592, 1.0, s.ad_value(1591));
        }

        if ((((((s.b[1523] && s.b[1594]) && s.b[1595]) && s.b[1596]) && (!s.b[1597])) && s.b[1602]) && (!s.b[1603])) {
            s.store_powf(1592, 1591, (-p.p1612));
        }

        if (((((s.b[1523] && s.b[1594]) && s.b[1595]) && s.b[1596]) && (!s.b[1597])) && s.b[1602]) {
            s.store_add_product3_rhs_mixed_iia(532, 1593, 562, 525, A::sub_from_scalar(1.0, A::mul(s.ad_value(1591), s.ad_value(1592))), (p.p1606 * 1.0 / ((1.0 - p.p1612))));
        }

        if (((((s.b[1523] && s.b[1594]) && s.b[1595]) && s.b[1596]) && (!s.b[1597])) && (!s.b[1602])) {
            s.store_sub_ad_rhs(532, 1593, A::mul3_scaled_output(s.ad_value(562), s.ad_value(525), {
                if (!(s.v[1591] > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if (s.v[1591] > 1e-38) {
                            A::ln(s.ad_value(1591))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, p.p1606));
        }

        if (((s.b[1523] && s.b[1594]) && s.b[1595]) && (!s.b[1596])) {
            s.store_sub_from_scalar(1591, 1.0, 1586);
        }

        s.b[1604] = (p.p1600 != 1.0);
        s.store_scalar(1604, if s.b[1604] { 1.0 } else { 0.0 });

        s.b[1605] = (p.p1600 == 0.5);
        s.store_scalar(1605, if s.b[1605] { 1.0 } else { 0.0 });

        if (((((s.b[1523] && s.b[1594]) && s.b[1595]) && (!s.b[1596])) && s.b[1604]) && s.b[1605]) {
            s.store_div_from_scalar_sqrt_ad(1592, 1.0, s.ad_value(1591));
        }

        if (((((s.b[1523] && s.b[1594]) && s.b[1595]) && (!s.b[1596])) && s.b[1604]) && (!s.b[1605])) {
            s.store_powf(1592, 1591, (-p.p1600));
        }

        if ((((s.b[1523] && s.b[1594]) && s.b[1595]) && (!s.b[1596])) && s.b[1604]) {
            s.store_mul_ad_affine_product_rhs(532, 271, s.ad_value(525), A::sub_from_scalar(1.0, A::mul(s.ad_value(1591), s.ad_value(1592))), 1.0 / ((1.0 - p.p1600)), 0.0);
        }

        if ((((s.b[1523] && s.b[1594]) && s.b[1595]) && (!s.b[1596])) && (!s.b[1604])) {
            s.store_mul_ad_affine_product_rhs(532, 271, s.ad_value(525), {
                if (!(s.v[1591] > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if (s.v[1591] > 1e-38) {
                            A::ln(s.ad_value(1591))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, -1.0, 0.0);
        }

    }
}
