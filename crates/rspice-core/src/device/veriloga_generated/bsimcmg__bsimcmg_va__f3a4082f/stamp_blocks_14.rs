#![allow(dead_code, non_snake_case, unused_assignments, unused_imports, unused_parens, unused_variables)]
use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, ReactiveScratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};
impl Instance {
    #[inline(never)]
    pub(super) fn stamp_reactive_block_93(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.b[1452] = ((p.p61 != 0.0) && (s.v[656] != 0.0));s.store_scalar(1452, if s.b[1452] { 1.0 } else { 0.0 });
        if s.b[1452] {s.store_offset_powf_ad(175, A::scale(s.ad_value(904), 1.0 / (p.p401)), p.p402, 1.0);s.store_div(374, 373, 175);s.store_div_from_scalar_ad(494, 1.0, A::add_scaled_product(A::div_from_scalar(1.0, s.ad_value(494)), 1.0, s.ad_value(374), s.ad_value(656), 1.0 / (s.v[143])));}
        s.store_div_scaled_product3_indices(183, 416, 163, 158, 1.0, 153, 1.0);s.store_add_scaled_product_indices(409, 396, s.v[420], 407, 400, s.v[420]);s.b[1453] = (p.p80 == 0.0);s.store_scalar(1453, if s.b[1453] { 1.0 } else { 0.0 });
        if s.b[1453] {s.store_pow_ad(170, A::scaled_offset(A::div(s.ad_value(400), s.ad_value(406)), 1.0, 0.5), s.ad_value(317));}
        if (!s.b[1453]) {s.store_scaled_square(168, 390, 1600.0);s.store_sub_from_scalar_ad(169, 1.0, A::limited_exp_scaled_input(s.ad_value(168), -1.0));s.store_mul_mixed_ai(168, A::add_scaled_products(s.ad_value(330), s.ad_value(392), 1.0, s.ad_value(331), s.ad_value(393), 1.0), 169);}
        if (!s.b[1453]) {
            if (!(s.v[168] < ((-10000.0) * 1e-12))) {
                s.store_scaled_add_mixed_ia(169, 168, A::sqrt_square_offset(s.ad_value(168), ((4.0 * 1e-12) * 1e-12)), 0.5);
            } else {
                if (s.v[168] < ((-10000.0) * 1e-12)) {
                    s.store_div_from_scalar(169, ((-1e-12) * 1e-12), 168);
                } else {
                    s.store_scalar(169, 0.0);
                }
            }
        }
        if (!s.b[1453]) {s.store_pow_ad(170, A::scaled_offset(A::div(s.ad_value(169), s.ad_value(406)), 1.0, 0.5), s.ad_value(317));}
        s.store_pow_indices(168, 409, 822);s.b[1454] = (p.p61 != 0.0);s.store_scalar(1454, if s.b[1454] { 1.0 } else { 0.0 });
        if s.b[1454] {s.store_add_scaled_product_mixed_aai(171, A::div(s.ad_value(820), s.ad_value(170)), 1.0, A::add_scaled_product(s.ad_value(819), 1.0, s.ad_value(821), s.ad_value(370), 1.0), 168, 1.0);}
        if (!s.b[1454]) {s.store_add_scaled_product_mixed_aii(171, A::div(s.ad_value(820), s.ad_value(170)), 1.0, 819, 168, 1.0);}
        s.store_offset(411, 171, 1.0);s.store_scaled_add_offset_sqrt_square_offset(411, 411, 1.0, (-1.0), ((0.25 * p.p604) * p.p604), 0.5);s.store_offset_scaled_ad(215, A::scale(A::limited_exp_scaled_input(s.ad_value(390), (-p.p888)), p.p887), (-p.p24), p.p24);s.store_div(411, 411, 215);s.store_div(415, 416, 411);s.b[1455] = (p.p67 == 1.0);s.store_scalar(1455, if s.b[1455] { 1.0 } else { 0.0 });s.b[1456] = (p.p80 == 0.0);s.store_scalar(1456, if s.b[1456] { 1.0 } else { 0.0 });
        if (s.b[1455] && s.b[1456]) {s.store_pow_ad(170, A::scaled_offset(A::div(s.ad_value(404), s.ad_value(406)), 1.0, 0.5), s.ad_value(317));}
        if (s.b[1455] && (!s.b[1456])) {s.store_add_scaled_products_indices(168, 330, 394, 1.0, 331, 395, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_94(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (s.b[1455] && (!s.b[1456])) {
            if (!(s.v[168] < ((-10000.0) * 1e-12))) {
                s.store_scaled_add_mixed_ia(169, 168, A::sqrt_square_offset(s.ad_value(168), ((4.0 * 1e-12) * 1e-12)), 0.5);
            } else {
                if (s.v[168] < ((-10000.0) * 1e-12)) {
                    s.store_div_from_scalar(169, ((-1e-12) * 1e-12), 168);
                } else {
                    s.store_scalar(169, 0.0);
                }
            }
        }
        if (s.b[1455] && (!s.b[1456])) {s.store_pow_ad(170, A::scaled_offset(A::div(s.ad_value(169), s.ad_value(406)), 1.0, 0.5), s.ad_value(317));}
        if s.b[1455] {s.store_add_scaled_product_indices(410, 396, s.v[420], 408, 404, s.v[420]);s.store_add_scaled_product_mixed_aia(171, A::div(s.ad_value(319), s.ad_value(170)), 1.0, 304, A::pow(s.ad_value(410), s.ad_value(822)), 1.0);}
        if (!s.b[1455]) {s.store_add_scaled_product_indices(410, 396, s.v[420], 408, 400, s.v[420]);s.store_add_scaled_product_mixed_aia(171, A::div(s.ad_value(820), s.ad_value(170)), 1.0, 819, A::pow(s.ad_value(410), s.ad_value(822)), 1.0);}
        s.store_offset(412, 171, 1.0);s.store_scaled_add_offset_sqrt_square_offset(412, 412, 1.0, (-1.0), ((0.25 * p.p604) * p.p604), 0.5);s.store_div(412, 412, 215);s.store_offset_div_scaled_product_indices(360, 719, 153, 1.0, 351, 1.0, 1e-6);s.b[1457] = (s.v[360] < 40.0);s.store_scalar(1457, if s.b[1457] { 1.0 } else { 0.0 });
        if s.b[1457] {s.store_add_mixed_ai(200, A::div_scaled_value_offset_denominator(s.ad_value(427), 0.5, A::cosh(s.ad_value(360)), (-1.0), 1.0), 718);}
        if (!s.b[1457]) {s.store_add_scaled_product_mixed_iia(200, 718, 1.0, 427, A::limited_exp_scaled_input(s.ad_value(360), -1.0), 1.0);}
        s.b[1458] = (s.v[720] > 0.0);s.store_scalar(1458, if s.b[1458] { 1.0 } else { 0.0 });
        if s.b[1458] {s.store_offset_div_scaled_product_indices(201, 720, 399, 1.0, 217, 1.0, 1.0);}
        if (!s.b[1458]) {s.store_div_from_scalar_sub_from_scalar_ad(201, 1.0, 1.0, A::div_scaled_product(s.ad_value(720), s.ad_value(399), 1.0, s.ad_value(217), 1.0));}
        s.store_sub(202, 126, 390);s.b[1459] = (p.p80 == 0.0);s.store_scalar(1459, if s.b[1459] { 1.0 } else { 0.0 });
        if s.b[1459] {s.store_add_scaled_inputs(204, 399, 1.0, 179, 2.0);}
        if (!s.b[1459]) {s.store_add_scaled_inputs(204, 399, 1.0, 182, 2.0);}
        s.b[1460] = (s.v[200] > 0.0);s.store_scalar(1460, if s.b[1460] { 1.0 } else { 0.0 });
        if s.b[1460] {s.copy_ad(169, 204);s.store_div_add_scaled_inputs_rhs_indices(171, 169, 210, 1.0, 169, 1.0);s.store_mul_ad_product_lhs_mixed_ai(203, A::div(s.ad_value(169), s.ad_value(200)), 171, 201);s.store_offset_div(205, 202, 203, 1.0);}
        if (!s.b[1460]) {s.store_scalar(205, 1.0);}
        s.b[1461] = (s.v[795] > 0.0);s.store_scalar(1461, if s.b[1461] { 1.0 } else { 0.0 });s.b[1462] = (s.v[793] < 0.0);s.store_scalar(1462, if s.b[1462] { 1.0 } else { 0.0 });
        if (s.b[1461] && s.b[1462]) {s.store_div_from_scalar_ad(169, 1.0, A::add_scaled_product(A::div_from_scalar(1.0, s.ad_value(795)), 1.0, s.ad_value(793), s.ad_value(399), (-1.0)));}
        if (s.b[1461] && (!s.b[1462])) {s.store_add_scaled_product_indices(169, 795, 1.0, 793, 399, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_95(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
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
        if (!s.b[1461]) {s.store_scalar(206, 1.0);}
        s.store_mul(205, 205, 206);s.store_div_scaled_inputs_indices(218, 422, 2.0, 415, 1.0);s.store_mul(219, 218, 153);
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
        }));s.store_div_from_scalar(169, 1.0, 695);
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
        s.store_div_scaled_offset_numerator_mixed_ai(209, A::limited_exp(A::mul(s.ad_value(169), {
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
        })), 1.0, 1.0, 225, 1.0);s.store_add_scaled_product_mixed_iai(209, 209, 1.0, A::mul3_scaled_output(s.ad_value(424), s.ad_value(399), s.ad_value(402), 0.5), 402, 1.0);s.store_add_div_rhs_mixed_ia(168, 241, 242, A::add_scaled_inputs(s.ad_value(399), 1.0, s.ad_value(181), 2.0));s.store_mul3_lhs(169, 168, 402, 402);s.store_offset(170, 169, ((1.0) + ((-0.001))));s.store_offset_add_scaled_inputs_mixed_ia(171, 170, 0.5, A::sqrt_square_offset(s.ad_value(170), 0.004), 0.5, (-1.0));s.store_scaled_offset_ad(214, A::sqrt(A::offset(s.ad_value(171), 1.0)), 1.0, 0.5);s.store_mul(209, 209, 214);s.store_scaled_add_offset_sqrt_square_offset(209, 209, 1.0, (-1.0), ((0.25 * p.p453) * p.p453), 0.5);s.store_div_mixed_ia(169, 236, A::add_scaled_product(s.ad_value(181), 2.0, A::max_from_scalar(0.0, A::add(s.ad_value(237), A::mul3(s.ad_value(294), s.ad_value(402), s.ad_value(402)))), s.ad_value(399), 1.0));s.store_limited_exp_neg_input(366, 169);s.b[1463] = (p.p61 == 2.0);s.store_scalar(1463, if s.b[1463] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_96(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
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
        if s.b[1463] {s.store_div_mixed_ia(169, 168, A::add_scaled_product(s.ad_value(181), 2.0, A::max_from_scalar(0.0, A::add(s.ad_value(238), A::mul3(s.ad_value(295), s.ad_value(402), s.ad_value(402)))), s.ad_value(399), 1.0));s.store_sub_ad(171, A::sqrt(A::sub(s.ad_value(689), s.ad_value(370))), A::sqrt(s.ad_value(689)));s.store_limited_exp_ad(371, A::mul_scaled_lhs(s.ad_value(169), -1.0, s.ad_value(171)));}
        if (!s.b[1463]) {s.store_scalar(371, 1.0);}
        s.b[1464] = (p.p67 == 1.0);s.store_scalar(1464, if s.b[1464] { 1.0 } else { 0.0 });
        if s.b[1464] {s.store_div_scaled_product_indices(220, 336, 412, 2.0, 414, 1.0);}
        if (!s.b[1464]) {s.store_div_scaled_product_indices(220, 336, 412, 2.0, 416, 1.0);}
        s.store_mul(221, 220, 156);s.b[1465] = (p.p67 == 1.0);s.store_scalar(1465, if s.b[1465] { 1.0 } else { 0.0 });
        if s.b[1465] {s.store_pow_ad(168, A::div(s.ad_value(405), s.ad_value(221)), s.ad_value(697));}
        if (!s.b[1465]) {s.store_pow_ad(168, A::div(s.ad_value(402), s.ad_value(221)), s.ad_value(697));}
        s.store_div_from_scalar(169, 1.0, 697);s.store_offset_pow_ad(225, s.ad_value(696), s.ad_value(169), 1.0);s.store_div_scaled_offset_numerator_mixed_ai(213, A::pow(A::add(s.ad_value(696), s.ad_value(168)), s.ad_value(169)), 1.0, 1.0, 225, 1.0);s.store_primal_scaled_add_offset_sqrt_square_offset(881, 881, 0.1, (-0.1), ((0.25 * 0.001) * 0.001), 0.5);s.store_mul(213, 213, 881);s.b[1466] = (s.v[794] != 0.0);s.store_scalar(1466, if s.b[1466] { 1.0 } else { 0.0 });
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
        if (!s.b[1466]) {s.store_scalar(207, 1.0);}
        s.store_mul3_affine_lhs(140, 640, 894, (-1.60219e-19), 0.0, 156);
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_97(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        s.store_div_add_scaled_inputs_rhs_indices(131, 339, 339, 1.0, 399, 1.0);s.store_add_mixed_ia(123, 399, A::mul_sub_from_scalar_lhs(2.0, s.ad_value(131), s.ad_value(181)));s.store_mul(122, 123, 402);s.b[1467] = (p.p64 == 0.0);s.store_scalar(1467, if s.b[1467] { 1.0 } else { 0.0 });s.b[1468] = (p.p64 == 1.0);s.store_scalar(1468, if s.b[1468] { 1.0 } else { 0.0 });s.b[1469] = (p.p64 == 2.0);s.store_scalar(1469, if s.b[1469] { 1.0 } else { 0.0 });
        if s.b[1467] {s.store_offset_mul(172, 711, 399, 1.0);s.store_div_from_scalar(169, 1.0, 172);s.store_scaled_add_mixed_ia(168, 169, A::sqrt_square_offset(s.ad_value(169), 0.01), 0.5);s.store_mul_ad_product_lhs_mixed_ia(197, 194, A::offset(A::mul(s.ad_value(709), s.ad_value(168)), p.p908), 189);s.store_offset_mul_ad(188, A::div_scaled_product_by_product(s.ad_value(183), s.ad_value(123), s.v[115], s.ad_value(411), s.ad_value(209), 1.0), s.ad_value(197), 1.0);}
        if (s.b[1468] && (!s.b[1467])) {s.store_scalar(197, 0.0);s.store_scalar(188, 1.0);s.store_add_scaled_product_mixed_iia(170, 479, (-1.0), 114, A::voltage(ctx, nodes, Some(11), Some(8)), 1.0);s.store_sqrt_square_offset(171, 170, 0.1);s.store_scaled_add(482, 170, 171, 0.5);s.store_offset_mul(172, 711, 482, 1.0);s.store_div_from_scalar(169, 1.0, 172);s.store_scaled_add_mixed_ia(168, 169, A::sqrt_square_offset(s.ad_value(169), 0.01), 0.5);s.store_mul_scale_offset_mixed_ia(174, 853, A::mul(s.ad_value(425), A::powf(A::offset(A::square(A::voltage(ctx, nodes, Some(2), Some(8))), 1e-6), (0.5 * p.p921))), 1.0, 1.0);s.store_add_scaled_product_mixed_iia(170, 479, (-1.0), 114, A::voltage(ctx, nodes, Some(11), Some(9)), 1.0);s.store_sqrt_square_offset(171, 170, 0.1);s.store_scaled_add(483, 170, 171, 0.5);s.store_offset_mul(172, 712, 483, 1.0);s.store_div_from_scalar(169, 1.0, 172);s.store_scaled_add_mixed_ia(168, 169, A::sqrt_square_offset(s.ad_value(169), 0.01), 0.5);s.store_mul_scale_offset_mixed_ia(174, 852, A::mul(s.ad_value(426), A::powf(A::offset(A::square(A::voltage(ctx, nodes, Some(0), Some(9))), 1e-6), (0.5 * p.p922))), 1.0, 1.0);}
        if (s.b[1469] && (!(s.b[1467] || s.b[1468]))) {s.store_offset_mul(172, 711, 399, 1.0);s.store_div_from_scalar(169, 1.0, 172);s.store_scaled_add_mixed_ia(168, 169, A::sqrt_square_offset(s.ad_value(169), 0.01), 0.5);s.store_mul_add_mixed_iai(197, 194, A::add_scaled_offset_product_lhs(s.ad_value(190), 1.0, A::mul(s.ad_value(709), s.ad_value(168)), p.p908, s.ad_value(189), 1.0), 191);s.store_offset_mul_ad(188, A::div_scaled_product_by_product(s.ad_value(183), s.ad_value(123), s.v[115], s.ad_value(411), s.ad_value(209), 1.0), s.ad_value(197), 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_98(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.store_div_scaled_product3_mixed_aiia(124, A::mul3_scaled_output(s.ad_value(183), s.ad_value(122), s.ad_value(205), s.v[115]), 366, 371, 1.0, A::mul3(s.ad_value(411), s.ad_value(209), s.ad_value(188)), 1.0);s.store_scale(124, 124, p.p25);s.b[1470] = (p.p67 == 1.0);s.store_scalar(1470, if s.b[1470] { 1.0 } else { 0.0 });
        if s.b[1470] {s.store_div_scaled_inputs2_indices(341, 403, 2.0, 181, 1.0, 213, 1.0);s.store_add_mixed_ia(138, 403, A::div_scaled_product(s.ad_value(405), s.ad_value(405), 1.0, s.ad_value(341), 6.0));s.store_scaled_sub_mixed_ia(137, 403, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(405), 1.0, A::mul_offset_rhs(A::div(s.ad_value(405), s.ad_value(341)), A::div_scaled_inputs(s.ad_value(405), 1.0, s.ad_value(341), 5.0), 1.0), 1.0 / (6.0)), (-0.5));}
        if (!s.b[1470]) {s.store_div_scaled_inputs2_indices(341, 399, 2.0, 181, 1.0, 213, 1.0);s.store_add_mixed_ia(138, 399, A::div_scaled_product(s.ad_value(402), s.ad_value(402), 1.0, s.ad_value(341), 6.0));s.store_scaled_sub_mixed_ia(137, 399, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(402), 1.0, A::mul_offset_rhs(A::div(s.ad_value(402), s.ad_value(341)), A::div_scaled_inputs(s.ad_value(402), 1.0, s.ad_value(341), 5.0), 1.0), 1.0 / (6.0)), (-0.5));}
        s.store_div_from_scalar(208, 1.0, 207);s.store_add_scaled_products_mixed_iiai(138, 208, 138, 1.0, A::offset(s.ad_value(207), (-1.0)), 393, 1.0);s.store_add_scaled_products_mixed_aiai(137, A::square(s.ad_value(208)), 137, 1.0, A::sub(s.ad_value(207), s.ad_value(208)), 393, 0.5);s.store_sub_scaled_inputs(139, 138, -1.0, 137, 1.0);s.store_mul3_affine_lhs(175, 159, 156, s.v[115], 0.0, 372);s.store_mul(138, 175, 138);s.store_mul(137, 175, 137);s.store_mul(139, 175, 139);s.copy_ad(592, 138);s.b[1472] = (p.p61 != 0.0);s.store_scalar(1472, if s.b[1472] { 1.0 } else { 0.0 });s.b[1473] = (p.p62 == 5.0);s.store_scalar(1473, if s.b[1473] { 1.0 } else { 0.0 });
        if (s.b[1472] && s.b[1473]) {s.store_mul3_affine_lhs(169, 160, 157, s.v[115], 0.0, 494);}
        if (s.b[1472] && (!s.b[1473])) {s.store_mul3_affine_lhs(169, 159, 157, s.v[115], 0.0, 494);}
        if s.b[1472] {s.copy_ad(176, 904);s.store_mul(340, 176, 169);s.store_neg(495, 340);s.copy_ad(496, 340);s.store_mul3_affine_lhs(169, 159, 156, s.v[115], 0.0, 163);s.store_sub(170, 401, 904);s.store_mul(340, 169, 170);s.store_sub(495, 495, 340);s.store_add(496, 496, 340);s.store_mul3_affine_lhs(169, 159, 156, s.v[115], 0.0, 163);s.store_scaled_mul_ad(170, A::offset(s.ad_value(923), (-1.0)), A::add(s.ad_value(399), A::div_scaled_product(s.ad_value(402), s.ad_value(402), 1.0, s.ad_value(341), 6.0)), 0.5);s.store_mul(340, 169, 170);s.store_sub(495, 495, 340);s.store_add(496, 496, 340);}
        s.b[1474] = (s.v[128] < 0.0);s.store_scalar(1474, if s.b[1474] { 1.0 } else { 0.0 });
        if s.b[1474] {s.copy_ad(169, 137);s.copy_ad(137, 139);s.copy_ad(139, 169);}
        s.b[1475] = (p.p78 != 1.0);s.store_scalar(1475, if s.b[1475] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_99(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        s.b[1476] = (p.p76 != 2.0);s.store_scalar(1476, if s.b[1476] { 1.0 } else { 0.0 });
        if (s.b[1475] && s.b[1476]) {s.store_scaled_mul(169, 159, 114, s.v[115]);s.store_mul_voltage_ad(170, s.ad_value(114), ctx, nodes, Some(10), Some(6));s.store_offset_sub(168, 170, 518, 0.02);s.store_scaled_sub_mixed_ia(510, 168, A::sqrt_square_offset(s.ad_value(168), (4.0 * 0.02)), 0.5);s.store_mul_mixed_ia(498, 169, A::add_scaled_products(s.ad_value(648), A::add_scaled_offset_product_rhs(A::add_scaled_inputs3(s.ad_value(170), 1.0, s.ad_value(518), (-1.0), s.ad_value(510), -1.0), 1.0, s.ad_value(651), A::sqrt(A::sub_from_scalar(1.0, A::div_scaled_inputs(s.ad_value(510), 4.0, s.ad_value(651), 1.0))), (-1.0), (-0.5)), 1.0, s.ad_value(646), s.ad_value(170), 1.0));s.store_mul_voltage_ad(170, s.ad_value(114), ctx, nodes, Some(10), Some(5));s.store_offset_sub(168, 170, 518, 0.02);s.store_scaled_sub_mixed_ia(511, 168, A::sqrt_square_offset(s.ad_value(168), (4.0 * 0.02)), 0.5);s.store_mul_mixed_ia(499, 169, A::add_scaled_products(s.ad_value(649), A::add_scaled_offset_product_rhs(A::add_scaled_inputs3(s.ad_value(170), 1.0, s.ad_value(518), (-1.0), s.ad_value(511), -1.0), 1.0, s.ad_value(652), A::sqrt(A::sub_from_scalar(1.0, A::div_scaled_inputs(s.ad_value(511), 4.0, s.ad_value(652), 1.0))), (-1.0), (-0.5)), 1.0, s.ad_value(647), s.ad_value(170), 1.0));}
        if (s.b[1475] && (!s.b[1476])) {s.store_scaled_mul(169, 159, 114, s.v[115]);s.store_mul_voltage_ad(170, s.ad_value(114), ctx, nodes, Some(13), Some(6));s.store_offset_sub(168, 170, 518, 0.02);s.store_scaled_sub_mixed_ia(510, 168, A::sqrt_square_offset(s.ad_value(168), (4.0 * 0.02)), 0.5);s.store_mul_mixed_ia(498, 169, A::add_scaled_products(s.ad_value(648), A::add_scaled_offset_product_rhs(A::add_scaled_inputs3(s.ad_value(170), 1.0, s.ad_value(518), (-1.0), s.ad_value(510), -1.0), 1.0, s.ad_value(651), A::sqrt(A::sub_from_scalar(1.0, A::div_scaled_inputs(s.ad_value(510), 4.0, s.ad_value(651), 1.0))), (-1.0), (-0.5)), 1.0, s.ad_value(646), s.ad_value(170), 1.0));s.store_mul_voltage_ad(170, s.ad_value(114), ctx, nodes, Some(14), Some(5));s.store_offset_sub(168, 170, 518, 0.02);s.store_scaled_sub_mixed_ia(511, 168, A::sqrt_square_offset(s.ad_value(168), (4.0 * 0.02)), 0.5);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_100(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        if (s.b[1475] && (!s.b[1476])) {s.store_mul_mixed_ia(499, 169, A::add_scaled_products(s.ad_value(649), A::add_scaled_offset_product_rhs(A::add_scaled_inputs3(s.ad_value(170), 1.0, s.ad_value(518), (-1.0), s.ad_value(511), -1.0), 1.0, s.ad_value(652), A::sqrt(A::sub_from_scalar(1.0, A::div_scaled_inputs(s.ad_value(511), 4.0, s.ad_value(652), 1.0))), (-1.0), (-0.5)), 1.0, s.ad_value(647), s.ad_value(170), 1.0));}
        s.b[1477] = (p.p78 == 0.0);s.store_scalar(1477, if s.b[1477] { 1.0 } else { 0.0 });s.b[1478] = (p.p76 != 2.0);s.store_scalar(1478, if s.b[1478] { 1.0 } else { 0.0 });
        if (s.b[1477] && s.b[1478]) {s.store_scale(169, 159, s.v[115]);s.store_mul_ad_product_rhs_mixed_ia(500, 169, 643, A::voltage(ctx, nodes, Some(10), Some(6)));s.store_mul_ad_product_rhs_mixed_ia(501, 169, 642, A::voltage(ctx, nodes, Some(10), Some(5)));s.store_add(505, 498, 500);s.store_add(506, 499, 501);}
        if (s.b[1477] && (!s.b[1478])) {s.store_scale(169, 159, s.v[115]);s.store_mul_ad_product_rhs_mixed_ia(500, 169, 643, A::voltage(ctx, nodes, Some(13), Some(6)));s.store_mul_ad_product_rhs_mixed_ia(501, 169, 642, A::voltage(ctx, nodes, Some(14), Some(5)));s.store_add(505, 498, 500);s.store_add(506, 499, 501);}
        s.b[1479] = (p.p78 == 1.0);s.store_scalar(1479, if s.b[1479] { 1.0 } else { 0.0 });s.b[1480] = (p.p76 != 2.0);s.store_scalar(1480, if s.b[1480] { 1.0 } else { 0.0 });s.b[1481] = (p.p63 == 1.0);s.store_scalar(1481, if s.b[1481] { 1.0 } else { 0.0 });
        if ((((!s.b[1477]) && s.b[1479]) && s.b[1480]) && s.b[1481]) {s.store_scale(168, 159, s.v[115]);s.store_mul(644, 168, 644);s.store_mul(645, 168, 645);s.store_scale(513, 168, p.p15);s.store_scale(514, 168, p.p16);}
        if ((((!s.b[1477]) && s.b[1479]) && s.b[1480]) && (!s.b[1481])) {s.store_scalar(513, p.p15);s.store_scalar(514, p.p16);}
        if (((!s.b[1477]) && s.b[1479]) && s.b[1480]) {s.store_mul_voltage_ad(498, s.ad_value(644), ctx, nodes, Some(10), Some(6));s.store_mul_voltage_ad(499, s.ad_value(645), ctx, nodes, Some(10), Some(5));s.copy_ad(505, 498);s.copy_ad(506, 499);s.store_mul_voltage_ad(500, s.ad_value(513), ctx, nodes, Some(10), Some(2));s.store_mul_voltage_ad(501, s.ad_value(514), ctx, nodes, Some(10), Some(0));}
        s.b[1482] = (p.p63 == 1.0);s.store_scalar(1482, if s.b[1482] { 1.0 } else { 0.0 });
        if ((((!s.b[1477]) && s.b[1479]) && (!s.b[1480])) && s.b[1482]) {s.store_scale(168, 159, s.v[115]);s.store_mul(644, 168, 644);s.store_mul(645, 168, 645);s.store_scale(513, 168, p.p15);s.store_scale(514, 168, p.p16);}
        if ((((!s.b[1477]) && s.b[1479]) && (!s.b[1480])) && (!s.b[1482])) {s.store_scalar(513, p.p15);s.store_scalar(514, p.p16);}
        if (((!s.b[1477]) && s.b[1479]) && (!s.b[1480])) {s.store_mul_voltage_ad(498, s.ad_value(644), ctx, nodes, Some(13), Some(6));s.store_mul_voltage_ad(499, s.ad_value(645), ctx, nodes, Some(14), Some(5));s.copy_ad(505, 498);s.copy_ad(506, 499);s.store_mul_voltage_ad(500, s.ad_value(513), ctx, nodes, Some(13), Some(2));s.store_mul_voltage_ad(501, s.ad_value(514), ctx, nodes, Some(14), Some(0));}
        s.b[1483] = (p.p76 != 2.0);s.store_scalar(1483, if s.b[1483] { 1.0 } else { 0.0 });
        if (((!s.b[1477]) && (!s.b[1479])) && s.b[1483]) {s.store_mul_voltage_ad(500, s.ad_value(453), ctx, nodes, Some(10), Some(6));s.store_mul_voltage_ad(501, s.ad_value(453), ctx, nodes, Some(10), Some(5));s.store_add(505, 498, 500);s.store_add(506, 499, 501);}
        if (((!s.b[1477]) && (!s.b[1479])) && (!s.b[1483])) {s.store_mul_voltage_ad(500, s.ad_value(453), ctx, nodes, Some(13), Some(6));s.store_mul_voltage_ad(501, s.ad_value(453), ctx, nodes, Some(14), Some(5));s.store_add(505, 498, 500);s.store_add(506, 499, 501);}
        s.b[1484] = (p.p65 == 1.0);s.store_scalar(1484, if s.b[1484] { 1.0 } else { 0.0 });
        if s.b[1484] {s.store_scalar(239, 1e-6);s.store_mul_div_scaled_inputs_mixed_aii(178, A::sqrt(s.ad_value(179)), 239, 1.0, 181, 2.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_101(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        if s.b[1484] {s.store_scale(168, 178, 0.5);s.store_div_scaled_inputs_mixed_ai(170, A::offset(s.ad_value(132), (-p.p144)), -1.0, 179, 1.0);}
        s.b[1485] = ((s.v[170] * s.v[179]) > (s.v[166] + (s.v[178] * (((s.v[166] * s.v[179])) as f64).sqrt())));s.store_scalar(1485, if s.b[1485] { 1.0 } else { 0.0 });
        if (s.b[1484] && s.b[1485]) {s.store_sub_mixed_ai(169, A::sqrt(A::add(A::offset(s.ad_value(170), (-1.0)), A::square(s.ad_value(168)))), 168);s.store_offset_square(340, 169, 1.0);s.store_offset_ad(175, A::limited_exp_scaled_input(s.ad_value(340), -1.0), (-1.0));}
        if (s.b[1484] && (!s.b[1485])) {s.store_sub_scaled_inputs_mixed_ia(171, 170, 0.5, A::scale_offset(s.ad_value(178), ((1.0 / (((2.0) as f64).sqrt())) * (3.0)), 3.0), 1.0);s.store_add_mixed_ia(340, 171, A::sqrt(A::add_scaled_inputs(A::square(s.ad_value(171)), 1.0, s.ad_value(170), 6.0)));}
        s.b[1486] = (s.v[170] < 0.0);s.store_scalar(1486, if s.b[1486] { 1.0 } else { 0.0 });
        if ((s.b[1484] && (!s.b[1485])) && s.b[1486]) {s.store_div_scaled_inputs2_indices(172, 170, 1.0, 340, (-1.0), 178, 1.0);s.store_sub_square_lhs(175, 172, 340);}
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
        if ((s.b[1484] && (!s.b[1485])) && (!s.b[1486])) {s.store_limited_exp_scaled_input(341, 340, (-1.2));s.store_sub_mixed_ai(172, A::sqrt(A::add_scaled_inputs3_offset(s.ad_value(170), 1.0, s.ad_value(341), 1.0, A::square(s.ad_value(168)), 1.0, (-1.0))), 168);s.store_add_ad(340, A::sub_from_scalar(1.0, s.ad_value(341)), A::square(s.ad_value(172)));s.store_offset_ad(175, A::limited_exp_scaled_input(s.ad_value(340), -1.0), (-1.0));}
        if s.b[1484] {s.store_sqrt_add(176, 175, 340);}
        s.b[1487] = (s.v[340] > 1e-15);s.store_scalar(1487, if s.b[1487] { 1.0 } else { 0.0 });
        if (s.b[1484] && s.b[1487]) {s.store_add_scaled_inputs_product_indices(344, 170, -1.0, 340, 1.0, 178, 176, 1.0);s.store_sub_from_scalar_ad(345, 1.0, A::div_scaled_product(s.ad_value(178), s.ad_value(175), 0.5, s.ad_value(176), 1.0));s.store_sub_div_rhs_indices(177, 340, 344, 345);}
        s.b[1488] = (s.v[340] < (-1e-15));s.store_scalar(1488, if s.b[1488] { 1.0 } else { 0.0 });
        if ((s.b[1484] && (!s.b[1487])) && s.b[1488]) {s.store_add_scaled_inputs_product_indices(344, 170, -1.0, 340, 1.0, 178, 176, (-1.0));s.store_offset_div_scaled_product_indices(345, 178, 175, 0.5, 176, 1.0, 1.0);s.store_sub_div_rhs_indices(177, 340, 344, 345);}
        if ((s.b[1484] && (!s.b[1487])) && (!s.b[1488])) {s.store_scalar(177, 0.0);}
        if s.b[1484] {s.store_mul_ad_product_lhs_mixed_ia(906, 178, A::limited_exp_scaled_input(s.ad_value(177), (-1.0 / (2.0))), 179);s.store_abs_voltage(915, ctx, nodes, Some(7), Some(6));s.store_mul_div_from_scalar_lhs_ad_indices(916, (2.0 * p.p454), 416, 397);s.store_scale(917, 916, p.p1);s.store_scalar(920, (1.0 / p.p530));s.store_add_scaled_inputs(175, 906, p.p491, 182, (2.0 * p.p491));s.store_div_scaled_product_add_scaled_denominator_indices(918, 917, 175, 1.0, 917, 1.0, 175, 1.0, 1.0);}
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
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_102(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if s.b[1484] {s.store_powf_ad(176, A::offset(A::div(s.ad_value(915), s.ad_value(918)), 1e-6), p.p530);s.store_pow_ad(177, A::offset(s.ad_value(176), 1.0), s.ad_value(920));s.store_min_ad(919, A::div(s.ad_value(915), s.ad_value(177)), s.ad_value(915));s.store_scalar(239, 1e-6);s.store_mul_div_scaled_inputs_mixed_aii(178, A::sqrt(s.ad_value(179)), 239, 1.0, 181, 2.0);s.store_scale(168, 178, 0.5);s.store_div_scaled_inputs_mixed_ai(170, A::offset(A::add(s.ad_value(133), s.ad_value(919)), (-p.p143)), -1.0, 179, 1.0);}
        s.b[1489] = ((s.v[170] * s.v[179]) > (s.v[166] + (s.v[178] * (((s.v[166] * s.v[179])) as f64).sqrt())));s.store_scalar(1489, if s.b[1489] { 1.0 } else { 0.0 });
        if (s.b[1484] && s.b[1489]) {s.store_sub_mixed_ai(169, A::sqrt(A::add(A::offset(s.ad_value(170), (-1.0)), A::square(s.ad_value(168)))), 168);s.store_offset_square(340, 169, 1.0);s.store_offset_ad(175, A::limited_exp_scaled_input(s.ad_value(340), -1.0), (-1.0));}
        if (s.b[1484] && (!s.b[1489])) {s.store_sub_scaled_inputs_mixed_ia(171, 170, 0.5, A::scale_offset(s.ad_value(178), ((1.0 / (((2.0) as f64).sqrt())) * (3.0)), 3.0), 1.0);s.store_add_mixed_ia(340, 171, A::sqrt(A::add_scaled_inputs(A::square(s.ad_value(171)), 1.0, s.ad_value(170), 6.0)));}
        s.b[1490] = (s.v[170] < 0.0);s.store_scalar(1490, if s.b[1490] { 1.0 } else { 0.0 });
        if ((s.b[1484] && (!s.b[1489])) && s.b[1490]) {s.store_div_scaled_inputs2_indices(172, 170, 1.0, 340, (-1.0), 178, 1.0);s.store_sub_square_lhs(175, 172, 340);}
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
        if ((s.b[1484] && (!s.b[1489])) && (!s.b[1490])) {s.store_limited_exp_scaled_input(341, 340, (-1.2));s.store_sub_mixed_ai(172, A::sqrt(A::add_scaled_inputs3_offset(s.ad_value(170), 1.0, s.ad_value(341), 1.0, A::square(s.ad_value(168)), 1.0, (-1.0))), 168);s.store_add_ad(340, A::sub_from_scalar(1.0, s.ad_value(341)), A::square(s.ad_value(172)));s.store_offset_ad(175, A::limited_exp_scaled_input(s.ad_value(340), -1.0), (-1.0));}
        if s.b[1484] {s.store_sqrt_add(176, 175, 340);}
        s.b[1491] = (s.v[340] > 1e-15);s.store_scalar(1491, if s.b[1491] { 1.0 } else { 0.0 });
        if (s.b[1484] && s.b[1491]) {s.store_add_scaled_inputs_product_indices(344, 170, -1.0, 340, 1.0, 178, 176, 1.0);s.store_sub_from_scalar_ad(345, 1.0, A::div_scaled_product(s.ad_value(178), s.ad_value(175), 0.5, s.ad_value(176), 1.0));s.store_sub_div_rhs_indices(177, 340, 344, 345);}
        s.b[1492] = (s.v[340] < (-1e-15));s.store_scalar(1492, if s.b[1492] { 1.0 } else { 0.0 });
        if ((s.b[1484] && (!s.b[1491])) && s.b[1492]) {s.store_add_scaled_inputs_product_indices(344, 170, -1.0, 340, 1.0, 178, 176, (-1.0));s.store_offset_div_scaled_product_indices(345, 178, 175, 0.5, 176, 1.0, 1.0);s.store_sub_div_rhs_indices(177, 340, 344, 345);}
        if ((s.b[1484] && (!s.b[1491])) && (!s.b[1492])) {s.store_scalar(177, 0.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_103(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        if s.b[1484] {s.store_mul_ad_product_lhs_mixed_ia(907, 178, A::limited_exp_scaled_input(s.ad_value(177), (-1.0 / (2.0))), 179);s.store_sub(911, 906, 907);s.store_scaled_add(910, 906, 907, 0.5);s.store_div_scaled_inputs2_indices(341, 910, 2.0, 181, 1.0, 209, 1.0);s.store_add_mixed_ia(905, 910, A::div_scaled_product(s.ad_value(911), s.ad_value(911), 1.0, s.ad_value(341), 6.0));s.store_scaled_sub_mixed_ia(909, 910, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(911), 1.0, A::mul_offset_rhs(A::div(s.ad_value(911), s.ad_value(341)), A::div_scaled_inputs(s.ad_value(911), 1.0, s.ad_value(341), 5.0), 1.0), 1.0 / (6.0)), 0.5);s.store_sub(908, 905, 909);}
        s.b[1493] = (p.p62 == 5.0);s.store_scalar(1493, if s.b[1493] { 1.0 } else { 0.0 });
        if (s.b[1484] && s.b[1493]) {s.store_scaled_mul(169, 160, 494, (s.v[115] * p.p1));}
        if (s.b[1484] && (!s.b[1493])) {s.store_scaled_mul(169, 159, 494, (s.v[115] * p.p1));}
        if s.b[1484] {s.copy_ad(176, 908);s.copy_ad(177, 909);s.store_mul(340, 176, 169);s.store_mul(341, 177, 169);s.copy_ad(908, 340);s.copy_ad(909, 341);s.copy_ad(504, 908);s.copy_ad(503, 909);}
        s.store_scaled_voltage(502, ctx, nodes, Some(0), Some(2), p.p17);s.b[1494] = (p.p71 == 1.0);s.store_scalar(1494, if s.b[1494] { 1.0 } else { 0.0 });
        if s.b[1494] {s.store_div_scaled_add_product_indices(168, 259, 1.0, 260, 153, 1.0, 153, 1.0);}
        s.b[1495] = ((s.v[168] <= 0.0) || (s.v[248] <= 0.0));s.store_scalar(1495, if s.b[1495] { 1.0 } else { 0.0 });
        if (s.b[1494] && (!s.b[1495])) {s.store_div_scaled_value_offset_denominator(169, s.ad_value(248), -1.0, s.ad_value(202), 1e-30, 1.0);}
        s.b[1496] = (p.p71 == 2.0);s.store_scalar(1496, if s.b[1496] { 1.0 } else { 0.0 });
        if ((!s.b[1494]) && s.b[1496]) {s.store_div_scaled_add_product_indices(493, 261, 1.0, 262, 153, 1.0, 153, 1.0);}
        s.b[1497] = (s.v[493] <= 0.0);s.store_scalar(1497, if s.b[1497] { 1.0 } else { 0.0 });
        if (((!s.b[1494]) && s.b[1496]) && (!s.b[1497])) {s.store_mul(168, 783, 153);s.store_div_scaled_product_offset_denominator_indices(169, 249, 168, 1.0, 168, 1.0, 1.0);}
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
        if (((!s.b[1494]) && s.b[1496]) && (!s.b[1497])) {s.store_add(171, 168, 787);}
        if (((!s.b[1494]) && s.b[1496]) && (!s.b[1497])) {
            if (!((s.v[348] * s.v[171]) < ((-10000.0) * p.p1442))) {
                s.store_add_scaled_product_mixed_aii(170, A::sqrt_square_offset(A::mul(s.ad_value(348), s.ad_value(171)), ((4.0 * p.p1442) * p.p1442)), 0.5, 348, 171, 0.5);
            } else {
                if ((s.v[348] * s.v[171]) < ((-10000.0) * p.p1442)) {
                    s.store_div_scalar_by_product_indices(170, ((-p.p1442) * p.p1442), 348, 171, 1.0);
                } else {
                    s.store_scalar(170, 0.0);
                }
            }
        }
        if (((!s.b[1494]) && s.b[1496]) && (!s.b[1497])) {s.store_div_from_scalar_offset_product(171, 1.0, 788, 126, 1.0);s.store_mul3_lhs(491, 169, 170, 171);s.store_mul_scale_offset_mixed_ia(490, 491, A::div(s.ad_value(784), s.ad_value(153)), -1.0, 1.0);s.store_sub(489, 126, 490);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_104(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (((!s.b[1494]) && s.b[1496]) && (!s.b[1497])) {s.store_add_ad(168, A::add_scaled_product(s.ad_value(782), 1.0, s.ad_value(781), s.ad_value(489), 1.0), A::mul3(s.ad_value(780), s.ad_value(489), s.ad_value(489)));s.store_sqrt_square_offset(169, 168, 1e-10);}
        s.b[1498] = (p.p69 != 0.0);s.store_scalar(1498, if s.b[1498] { 1.0 } else { 0.0 });
        if s.b[1498] {s.store_div_scaled_inputs2_by_product_indices(169, 399, 1.0, 725, (-1.0), 726, 179, 1.0);s.store_offset_add_scaled_inputs(170, A::offset(A::add_scaled_product(s.ad_value(243), 1.0, s.ad_value(723), s.ad_value(399), (-1.0)), (((-(-p.p1110))) + ((-1e-6)))), 0.5, A::sqrt_square_offset(A::offset(A::add_scaled_product(s.ad_value(243), 1.0, s.ad_value(723), s.ad_value(399), (-1.0)), (((-(-p.p1110))) + ((-1e-6)))), (-((4.0 * (-p.p1110)) * 1e-6))), 0.5, (-p.p1110));s.store_offset_mul(171, 724, 399, 1.0);s.store_scaled_mul(172, 170, 171, ((-982222000000.0) * p.p1109));s.store_limited_exp(174, 172);s.store_scalar(175, 3.75956e-7);s.store_add_scaled_inputs3_indices(468, 167, 1.0, 146, (-0.5), 166, -1.0);s.store_sub(168, 468, 497);s.store_div_scaled_value_by_product_indices(169, 168, 1.0, 731, 179, 1.0);}
        s.b[1499] = (p.p61 != 0.0);s.store_scalar(1499, if s.b[1499] { 1.0 } else { 0.0 });
        if (s.b[1498] && s.b[1499]) {s.copy_ad(466, 904);}
        s.b[1500] = (s.v[468] <= 0.0);s.store_scalar(1500, if s.b[1500] { 1.0 } else { 0.0 });
        if ((s.b[1498] && (!s.b[1499])) && s.b[1500]) {s.store_scaled_add_ad(466, A::offset(s.ad_value(168), (-0.02)), A::sqrt(A::sub_scaled_inputs(A::square(A::offset(s.ad_value(168), (-0.02))), 1.0, s.ad_value(468), 0.08)), 0.5);}
        if ((s.b[1498] && (!s.b[1499])) && (!s.b[1500])) {s.store_scaled_add_ad(466, A::offset(s.ad_value(168), (-0.02)), A::sqrt(A::add_scaled_inputs(A::square(A::offset(s.ad_value(168), (-0.02))), 1.0, s.ad_value(468), 0.08)), 0.5);}
        if s.b[1498] {s.store_offset_add_scaled_inputs(170, A::offset(A::add_scaled_product(s.ad_value(244), 1.0, s.ad_value(729), s.ad_value(466), (-1.0)), (((-(-p.p1111))) + ((-1e-6)))), 0.5, A::sqrt_square_offset(A::offset(A::add_scaled_product(s.ad_value(244), 1.0, s.ad_value(729), s.ad_value(466), (-1.0)), (((-(-p.p1111))) + ((-1e-6)))), (-((4.0 * (-p.p1111)) * 1e-6))), 0.5, (-p.p1111));s.store_offset_mul(171, 730, 466, 1.0);s.store_scaled_mul(172, 170, 171, ((-745669000000.0) * p.p1109));s.store_limited_exp(174, 172);s.store_scalar(175, 4.97232e-7);}
        s.b[1501] = (p.p68 != 0.0);s.store_scalar(1501, if s.b[1501] { 1.0 } else { 0.0 });
        if s.b[1501] {s.store_offset_add_scaled_inputs(169, A::offset(A::add_scaled_product(s.ad_value(245), 1.0, s.ad_value(734), s.ad_value(399), (-1.0)), (((-(-p.p1112))) + ((-1e-6)))), 0.5, A::sqrt_square_offset(A::offset(A::add_scaled_product(s.ad_value(245), 1.0, s.ad_value(734), s.ad_value(399), (-1.0)), (((-(-p.p1112))) + ((-1e-6)))), (-((4.0 * (-p.p1112)) * 1e-6))), 0.5, (-p.p1112));s.store_offset_mul(170, 735, 399, 1.0);s.store_mul3_affine_lhs(171, 485, 169, (-p.p1109), 0.0, 170);s.store_mul_limited_exp_rhs(172, 399, 171);s.store_add_scaled_inputs4_indices(174, 497, 1.0, 127, 0.5, 521, 0.5, 522, 0.5);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_105(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if s.b[1501] {s.store_offset_sqrt_ad(473, A::offset(A::square(s.ad_value(390)), 0.01), (-0.1));s.store_mul(169, 736, 473);s.store_limited_exp_neg_input(474, 169);s.store_offset_add(171, 169, 474, (((-1.0)) + (0.0001)));s.store_offset_sub_from_scalar_ad(172, 1.0, A::mul_offset_lhs(s.ad_value(169), 1.0, s.ad_value(474)), 0.0001);s.store_offset_square(174, 169, 0.0002);s.store_sub(168, 134, 479);s.store_sqrt_square_offset(482, 168, 0.0001);}
        s.b[1502] = (p.p82 == 1.0);s.store_scalar(1502, if s.b[1502] { 1.0 } else { 0.0 });
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
        s.b[1503] = (s.v[740] < 0.01);s.store_scalar(1503, if s.b[1503] { 1.0 } else { 0.0 });
        if ((s.b[1501] && s.b[1502]) && s.b[1503]) {s.store_scalar(740, 0.01);}
        if (s.b[1501] && (!s.b[1502])) {s.store_add_scaled_product_indices(169, 246, 1.0, 739, 482, (-1.0));}
        if s.b[1501] {s.store_offset_mul(170, 740, 482, 1.0);s.store_mul_product3_indices(171, 170, 485, 742, 169, (-p.p1109));s.store_limited_exp(172, 171);s.store_sub(168, 136, 479);s.store_sqrt_square_offset(483, 168, 0.0001);}
        s.b[1505] = (p.p82 == 1.0);s.store_scalar(1505, if s.b[1505] { 1.0 } else { 0.0 });
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
        s.b[1506] = (s.v[746] < 0.01);s.store_scalar(1506, if s.b[1506] { 1.0 } else { 0.0 });
        if ((s.b[1501] && s.b[1505]) && s.b[1506]) {s.store_scalar(746, 0.01);}
        if (s.b[1501] && (!s.b[1505])) {s.store_add_scaled_product_indices(169, 247, 1.0, 745, 483, (-1.0));}
        if s.b[1501] {s.store_offset_mul(170, 746, 483, 1.0);s.store_mul_product3_indices(171, 170, 485, 742, 169, (-p.p1109));s.store_limited_exp(172, 171);}
        s.b[1508] = (p.p70 != 0.0);s.store_scalar(1508, if s.b[1508] { 1.0 } else { 0.0 });
        if s.b[1508] {s.store_scalar(168, (s.v[145] * p.p89));}
        s.b[1509] = ((s.v[747] <= 0.0) || (s.v[252] <= 0.0));s.store_scalar(1509, if s.b[1509] { 1.0 } else { 0.0 });
        if (s.b[1508] && s.b[1509]) {s.store_scalar(175, 0.0);}
        if (s.b[1508] && (!s.b[1509])) {s.store_div_scaled_inputs3_indices(169, 136, -1.0, 750, (-1.0), 479, 1.0, 168, 1.0);}
        if (s.b[1508] && (!s.b[1509])) {
            if (!(s.v[169] < ((-10000.0) * 0.01))) {
                s.store_scaled_add_mixed_ia(169, 169, A::sqrt_square_offset(s.ad_value(169), ((4.0 * 0.01) * 0.01)), 0.5);
            } else {
                if (s.v[169] < ((-10000.0) * 0.01)) {
                    s.store_div_from_scalar(169, ((-0.01) * 0.01), 169);
                } else {
                    s.store_scalar(169, 0.0);
                }
            }
        }
        if (s.b[1508] && (!s.b[1509])) {s.store_div_scaled_value_offset_denominator(170, s.ad_value(252), 1.0, s.ad_value(169), 0.001, 1.0);s.store_pow_indices(171, 169, 751);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_106(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.b[1510] = (p.p61 != 0.0);s.store_scalar(1510, if s.b[1510] { 1.0 } else { 0.0 });
        if ((s.b[1508] && (!s.b[1509])) && s.b[1510]) {s.store_mul3_affine_lhs(172, 522, 522, -1.0, 0.0, 522);s.store_offset_add_ad(173, s.ad_value(749), A::abs(s.ad_value(172)), 1e-5);}
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
        if ((s.b[1508] && (!s.b[1509])) && s.b[1510]) {s.store_mul_ad_product_lhs(175, A::mul3(s.ad_value(747), s.ad_value(158), s.ad_value(171)), A::limited_exp_scaled_input(s.ad_value(170), -1.0), 174);}
        if ((s.b[1508] && (!s.b[1509])) && (!s.b[1510])) {s.store_mul_ad_product_lhs(175, A::mul3(s.ad_value(747), s.ad_value(158), s.ad_value(171)), A::limited_exp_scaled_input(s.ad_value(170), -1.0), 135);}
        s.b[1511] = ((p.p70 == 3.0) && (s.v[752] > 0.0));s.store_scalar(1511, if s.b[1511] { 1.0 } else { 0.0 });s.b[1512] = (p.p61 != 0.0);s.store_scalar(1512, if s.b[1512] { 1.0 } else { 0.0 });
        if ((s.b[1508] && s.b[1511]) && s.b[1512]) {
            s.store_mul_mixed_ia(254, 754, {
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
        if ((s.b[1508] && s.b[1511]) && s.b[1512]) {s.store_div_scaled_inputs3_mixed_aiii(169, A::add_scaled_product(A::mul3(s.ad_value(753), s.ad_value(136), s.ad_value(136)), 1.0, s.ad_value(254), s.ad_value(136), (-1.0)), 1.0, 755, (-1.0), 479, 1.0, 179, 1.0);s.store_mul_product3_mixed_aiii(170, A::limited_exp(s.ad_value(169)), 752, 158, 141, 1.0);s.store_mul3_affine_lhs(172, 522, 522, -1.0, 0.0, 522);s.store_offset_add_ad(173, s.ad_value(749), A::abs(s.ad_value(172)), 1e-5);}
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
        if ((s.b[1508] && s.b[1511]) && s.b[1512]) {s.store_add_scaled_product_indices(175, 175, 1.0, 170, 174, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_107(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((s.b[1508] && s.b[1511]) && (!s.b[1512])) {
            s.store_mul_mixed_ia(254, 754, {
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
        if ((s.b[1508] && s.b[1511]) && (!s.b[1512])) {s.store_div_scaled_inputs3_mixed_aiii(169, A::add_scaled_product(A::mul3(s.ad_value(753), s.ad_value(136), s.ad_value(136)), 1.0, s.ad_value(254), s.ad_value(136), (-1.0)), 1.0, 755, (-1.0), 479, 1.0, 179, 1.0);s.store_mul_product3_mixed_aiii(170, A::limited_exp(s.ad_value(169)), 752, 158, 141, 1.0);s.store_add_scaled_product_indices(175, 175, 1.0, 170, 135, 1.0);}
        s.b[1513] = (((p.p61 != 0.0) && ((p.p70 == 2.0) || (p.p70 == 3.0))) && (((p.p62 == 2.0) || (p.p62 == 3.0)) || (p.p62 == 5.0)));s.store_scalar(1513, if s.b[1513] { 1.0 } else { 0.0 });
        if (s.b[1508] && s.b[1513]) {
            s.store_mul_mixed_ia(255, 757, {
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
        s.b[1514] = ((s.v[756] <= 0.0) || (s.v[255] <= 0.0));s.store_scalar(1514, if s.b[1514] { 1.0 } else { 0.0 });
        if ((s.b[1508] && s.b[1513]) && s.b[1514]) {s.store_scalar(176, 0.0);}
        if ((s.b[1508] && s.b[1513]) && (!s.b[1514])) {s.store_div_scaled_inputs3_indices(169, 136, -1.0, 759, (-1.0), 479, 1.0, 168, 1.0);}
        if ((s.b[1508] && s.b[1513]) && (!s.b[1514])) {
            if (!(s.v[169] < ((-10000.0) * 0.01))) {
                s.store_scaled_add_mixed_ia(169, 169, A::sqrt_square_offset(s.ad_value(169), ((4.0 * 0.01) * 0.01)), 0.5);
            } else {
                if (s.v[169] < ((-10000.0) * 0.01)) {
                    s.store_div_from_scalar(169, ((-0.01) * 0.01), 169);
                } else {
                    s.store_scalar(169, 0.0);
                }
            }
        }
        if ((s.b[1508] && s.b[1513]) && (!s.b[1514])) {s.store_div_scaled_value_offset_denominator(170, s.ad_value(255), 1.0, s.ad_value(169), 0.001, 1.0);s.store_pow_indices(171, 169, 760);s.store_mul3_affine_lhs(172, 522, 522, -1.0, 0.0, 522);s.store_offset_add_ad(173, s.ad_value(758), A::abs(s.ad_value(172)), 1e-5);}
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
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_108(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((s.b[1508] && s.b[1513]) && (!s.b[1514])) {s.store_mul_ad_product_lhs(176, A::mul3(s.ad_value(756), s.ad_value(896), s.ad_value(171)), A::limited_exp_scaled_input(s.ad_value(170), -1.0), 174);}
        s.b[1516] = ((s.v[761] <= 0.0) || (s.v[250] <= 0.0));s.store_scalar(1516, if s.b[1516] { 1.0 } else { 0.0 });
        if (s.b[1508] && s.b[1516]) {s.store_scalar(175, 0.0);}
        if (s.b[1508] && (!s.b[1516])) {s.store_div_scaled_inputs3_indices(169, 134, -1.0, 764, (-1.0), 479, 1.0, 168, 1.0);}
        if (s.b[1508] && (!s.b[1516])) {
            if (!(s.v[169] < ((-10000.0) * 0.01))) {
                s.store_scaled_add_mixed_ia(169, 169, A::sqrt_square_offset(s.ad_value(169), ((4.0 * 0.01) * 0.01)), 0.5);
            } else {
                if (s.v[169] < ((-10000.0) * 0.01)) {
                    s.store_div_from_scalar(169, ((-0.01) * 0.01), 169);
                } else {
                    s.store_scalar(169, 0.0);
                }
            }
        }
        if (s.b[1508] && (!s.b[1516])) {s.store_div_scaled_value_offset_denominator(170, s.ad_value(250), 1.0, s.ad_value(169), 0.001, 1.0);s.store_pow_indices(171, 169, 765);}
        s.b[1517] = (p.p61 != 0.0);s.store_scalar(1517, if s.b[1517] { 1.0 } else { 0.0 });
        if ((s.b[1508] && (!s.b[1516])) && s.b[1517]) {s.store_mul3_affine_lhs(172, 521, 521, -1.0, 0.0, 521);s.store_offset_add_ad(173, s.ad_value(763), A::abs(s.ad_value(172)), 1e-5);}
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
        if ((s.b[1508] && (!s.b[1516])) && s.b[1517]) {s.store_mul_ad_product_lhs(175, A::mul3(s.ad_value(761), s.ad_value(158), s.ad_value(171)), A::limited_exp_scaled_input(s.ad_value(170), -1.0), 174);}
        if ((s.b[1508] && (!s.b[1516])) && (!s.b[1517])) {s.store_mul_ad_affine_product_lhs(175, A::mul3(s.ad_value(761), s.ad_value(158), s.ad_value(171)), A::limited_exp_scaled_input(s.ad_value(170), -1.0), -1.0, 0.0, 135);}
        s.b[1518] = ((p.p70 == 3.0) && (s.v[766] > 0.0));s.store_scalar(1518, if s.b[1518] { 1.0 } else { 0.0 });s.b[1519] = (p.p61 != 0.0);s.store_scalar(1519, if s.b[1519] { 1.0 } else { 0.0 });
        if ((s.b[1508] && s.b[1518]) && s.b[1519]) {
            s.store_mul_mixed_ia(253, 768, {
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
        if ((s.b[1508] && s.b[1518]) && s.b[1519]) {s.store_div_scaled_inputs3_mixed_aiii(169, A::add_scaled_product(A::mul3(s.ad_value(767), s.ad_value(134), s.ad_value(134)), 1.0, s.ad_value(253), s.ad_value(134), (-1.0)), 1.0, 769, (-1.0), 479, 1.0, 179, 1.0);}
    }
}
