#![allow(dead_code, non_snake_case, unused_assignments, unused_imports, unused_parens, unused_variables)]
use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, ReactiveScratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};
impl Instance {
    #[inline(never)]
    pub(super) fn stamp_reactive_block_57(
        s: &mut ReactiveScratch,
    ) {
        if (((((((!s.b[1298]) && (!s.b[1313])) && (!s.b[1354])) && s.b[1355]) && (!s.b[1356])) && s.b[1358]) && s.b[1359]) {s.store_sub_ad(175, A::add_scaled_inputs3(s.ad_value(195), 0.5, s.ad_value(196), 0.5, A::sqrt(A::add_scaled_square_product(A::sub(s.ad_value(195), s.ad_value(196)), 1.0, s.ad_value(857), s.ad_value(857), 0.25)), 0.5), A::add_scaled_inputs(s.ad_value(171), 0.5, A::sqrt(A::add_scaled_square_product(s.ad_value(171), 1.0, s.ad_value(857), s.ad_value(857), 0.25)), 0.5));s.store_sub_ad(176, A::add_scaled_inputs3(s.ad_value(172), 0.5, s.ad_value(174), 0.5, A::sqrt(A::add_scaled_square_product(A::sub(s.ad_value(172), s.ad_value(174)), 1.0, s.ad_value(857), s.ad_value(857), 0.25)), 0.5), A::add_scaled_inputs(s.ad_value(171), 0.5, A::sqrt(A::add_scaled_square_product(s.ad_value(171), 1.0, s.ad_value(857), s.ad_value(857), 0.25)), 0.5));s.store_add_scaled_offset_product_rhs(177, 176, 1.0, 854, 116, (-210.0), 1.0);s.store_add_scaled_inputs3_sqrt_third_sub_square_offset(170, 175, 0.5, 177, 0.5, 175, 177, ((0.25 * 0.001) * 0.001), 0.5);}
        if (((((((!s.b[1298]) && (!s.b[1313])) && (!s.b[1354])) && s.b[1355]) && (!s.b[1356])) && s.b[1358]) && (!s.b[1359])) {s.store_sub_ad(175, A::add_scaled_inputs3(s.ad_value(195), 0.5, s.ad_value(196), 0.5, A::sqrt(A::add_scaled_square_product(A::sub(s.ad_value(195), s.ad_value(196)), 1.0, s.ad_value(857), s.ad_value(857), 0.25)), (-0.5)), A::sub_scaled_inputs(s.ad_value(171), 0.5, A::sqrt(A::add_scaled_square_product(s.ad_value(171), 1.0, s.ad_value(857), s.ad_value(857), 0.25)), 0.5));s.store_sub_ad(176, A::add_scaled_inputs3(s.ad_value(172), 0.5, s.ad_value(174), 0.5, A::sqrt(A::add_scaled_square_product(A::sub(s.ad_value(172), s.ad_value(174)), 1.0, s.ad_value(857), s.ad_value(857), 0.25)), (-0.5)), A::sub_scaled_inputs(s.ad_value(171), 0.5, A::sqrt(A::add_scaled_square_product(s.ad_value(171), 1.0, s.ad_value(857), s.ad_value(857), 0.25)), 0.5));s.store_add_scaled_offset_product_rhs(177, 176, 1.0, 854, 116, (-210.0), 1.0);s.store_add_scaled_inputs3_sqrt_third_sub_square_offset(170, 175, 0.5, 177, 0.5, 175, 177, ((0.25 * 0.001) * 0.001), (-0.5));}
        if ((((((!s.b[1298]) && (!s.b[1313])) && (!s.b[1354])) && s.b[1355]) && (!s.b[1356])) && (!s.b[1358])) {s.store_offset_mul(196, 855, 232, 1.0);s.store_add_scaled_product_mixed_aia(195, A::offset(A::mul(s.ad_value(854), A::sub(s.ad_value(116), s.ad_value(856))), 1.0), 1.0, 855, A::sub(s.ad_value(856), s.ad_value(228)), 1.0);s.store_mul_sub_by_sub(171, 855, 854, 856, 228);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_58(
        s: &mut ReactiveScratch,
    ) {
        if ((((((!s.b[1298]) && (!s.b[1313])) && (!s.b[1354])) && s.b[1355]) && (!s.b[1356])) && (!s.b[1358])) {s.store_offset_ad(172, A::mul_sub_from_scalar_rhs(s.ad_value(855), 210.0, s.ad_value(228)), 1.0);s.store_add_scaled_product_mixed_aia(174, A::offset(A::mul_sub_from_scalar_rhs(s.ad_value(854), 210.0, s.ad_value(856)), 1.0), 1.0, 855, A::sub(s.ad_value(856), s.ad_value(228)), 1.0);}
        s.b[1360] = (s.v[855] < s.v[854]);s.store_scalar(1360, if s.b[1360] { 1.0 } else { 0.0 });
        if (((((((!s.b[1298]) && (!s.b[1313])) && (!s.b[1354])) && s.b[1355]) && (!s.b[1356])) && (!s.b[1358])) && s.b[1360]) {s.store_sub_ad(175, A::add_scaled_inputs3(s.ad_value(195), 0.5, s.ad_value(196), 0.5, A::sqrt(A::add_scaled_square_product(A::sub(s.ad_value(195), s.ad_value(196)), 1.0, s.ad_value(857), s.ad_value(857), 0.25)), 0.5), A::add_scaled_inputs(s.ad_value(171), 0.5, A::sqrt(A::add_scaled_square_product(s.ad_value(171), 1.0, s.ad_value(857), s.ad_value(857), 0.25)), 0.5));s.store_sub_ad(176, A::add_scaled_inputs3(s.ad_value(172), 0.5, s.ad_value(174), 0.5, A::sqrt(A::add_scaled_square_product(A::sub(s.ad_value(172), s.ad_value(174)), 1.0, s.ad_value(857), s.ad_value(857), 0.25)), 0.5), A::add_scaled_inputs(s.ad_value(171), 0.5, A::sqrt(A::add_scaled_square_product(s.ad_value(171), 1.0, s.ad_value(857), s.ad_value(857), 0.25)), 0.5));s.store_add_scaled_offset_product_rhs(177, 176, 1.0, 854, 116, (-210.0), 1.0);s.store_add_scaled_inputs3_sqrt_third_sub_square_offset(170, 175, 0.5, 177, 0.5, 175, 177, ((0.25 * 0.001) * 0.001), 0.5);}
        if (((((((!s.b[1298]) && (!s.b[1313])) && (!s.b[1354])) && s.b[1355]) && (!s.b[1356])) && (!s.b[1358])) && (!s.b[1360])) {s.store_sub_ad(175, A::add_scaled_inputs3(s.ad_value(195), 0.5, s.ad_value(196), 0.5, A::sqrt(A::add_scaled_square_product(A::sub(s.ad_value(195), s.ad_value(196)), 1.0, s.ad_value(857), s.ad_value(857), 0.25)), (-0.5)), A::sub_scaled_inputs(s.ad_value(171), 0.5, A::sqrt(A::add_scaled_square_product(s.ad_value(171), 1.0, s.ad_value(857), s.ad_value(857), 0.25)), 0.5));s.store_sub_ad(176, A::add_scaled_inputs3(s.ad_value(172), 0.5, s.ad_value(174), 0.5, A::sqrt(A::add_scaled_square_product(A::sub(s.ad_value(172), s.ad_value(174)), 1.0, s.ad_value(857), s.ad_value(857), 0.25)), (-0.5)), A::sub_scaled_inputs(s.ad_value(171), 0.5, A::sqrt(A::add_scaled_square_product(s.ad_value(171), 1.0, s.ad_value(857), s.ad_value(857), 0.25)), 0.5));s.store_add_scaled_offset_product_rhs(177, 176, 1.0, 854, 116, (-210.0), 1.0);s.store_add_scaled_inputs3_sqrt_third_sub_square_offset(170, 175, 0.5, 177, 0.5, 175, 177, ((0.25 * 0.001) * 0.001), (-0.5));}
        s.b[1361] = (s.v[228] > 210.0);s.store_scalar(1361, if s.b[1361] { 1.0 } else { 0.0 });
        if (((((!s.b[1298]) && (!s.b[1313])) && (!s.b[1354])) && (!s.b[1355])) && s.b[1361]) {s.store_offset_mul(195, 854, 232, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_59(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (((((!s.b[1298]) && (!s.b[1313])) && (!s.b[1354])) && (!s.b[1355])) && s.b[1361]) {s.store_add_ad(196, A::offset(A::mul_offset_rhs(s.ad_value(855), s.ad_value(116), (-210.0)), 1.0), A::mul_sub_from_scalar_rhs(s.ad_value(854), 210.0, s.ad_value(228)));}
        s.b[1362] = (s.v[855] < s.v[854]);s.store_scalar(1362, if s.b[1362] { 1.0 } else { 0.0 });
        if ((((((!s.b[1298]) && (!s.b[1313])) && (!s.b[1354])) && (!s.b[1355])) && s.b[1361]) && s.b[1362]) {s.store_add_scaled_inputs3_sqrt_third_sub_square_offset(170, 195, 0.5, 196, 0.5, 195, 196, ((0.25 * 0.01) * 0.01), 0.5);}
        if ((((((!s.b[1298]) && (!s.b[1313])) && (!s.b[1354])) && (!s.b[1355])) && s.b[1361]) && (!s.b[1362])) {s.store_add_scaled_inputs3_sqrt_third_sub_square_offset(170, 195, 0.5, 196, 0.5, 195, 196, ((0.25 * 0.01) * 0.01), (-0.5));}
        if (((((!s.b[1298]) && (!s.b[1313])) && (!s.b[1354])) && (!s.b[1355])) && (!s.b[1361])) {s.store_offset_mul(196, 855, 232, 1.0);s.store_add_ad(195, A::offset(A::mul_offset_rhs(s.ad_value(854), s.ad_value(116), (-210.0)), 1.0), A::mul_sub_from_scalar_rhs(s.ad_value(855), 210.0, s.ad_value(228)));}
        s.b[1363] = (s.v[855] < s.v[854]);s.store_scalar(1363, if s.b[1363] { 1.0 } else { 0.0 });
        if ((((((!s.b[1298]) && (!s.b[1313])) && (!s.b[1354])) && (!s.b[1355])) && (!s.b[1361])) && s.b[1363]) {s.store_add_scaled_inputs3_sqrt_third_sub_square_offset(170, 195, 0.5, 196, 0.5, 195, 196, ((0.25 * 0.01) * 0.01), 0.5);}
        if ((((((!s.b[1298]) && (!s.b[1313])) && (!s.b[1354])) && (!s.b[1355])) && (!s.b[1361])) && (!s.b[1363])) {s.store_add_scaled_inputs3_sqrt_third_sub_square_offset(170, 195, 0.5, 196, 0.5, 195, 196, ((0.25 * 0.01) * 0.01), (-0.5));}
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
        if ((!s.b[1298]) && (!s.b[1313])) {s.store_scaled_sub_offset_sqrt_square_offset(172, 228, 210.0, (-210.0), ((0.25 * 0.2) * 0.2), 0.5);s.store_sub_ad(231, A::add_scaled_product(A::div_scalar_offset_denominator(p[1747], A::limited_exp_scaled_input(A::offset(s.ad_value(117), (-p[1749])), p[1748]), 1.0, 1.0), 1.0, A::add(s.ad_value(858), A::div_from_scalar(p[1720], s.ad_value(153))), s.ad_value(230), 1.0), A::div_scalar_offset_denominator(p[1747], A::limited_exp_scaled_input(A::offset(s.ad_value(172), (-p[1749])), p[1748]), 1.0, 1.0));}
        s.b[1364] = (s.v[332] < 1000.0);s.store_scalar(1364, if s.b[1364] { 1.0 } else { 0.0 });
        if s.b[1364] {s.store_scalar(332, 1000.0);}
        s.b[1365] = (s.v[334] < 1000.0);s.store_scalar(1365, if s.b[1365] { 1.0 } else { 0.0 });
        if s.b[1365] {s.store_scalar(334, 1000.0);}
        s.b[1366] = (s.v[336] < 1000.0);s.store_scalar(1366, if s.b[1366] { 1.0 } else { 0.0 });
        if s.b[1366] {s.store_scalar(336, 1000.0);}
        s.b[1367] = (p[61] != 0.0);s.store_scalar(1367, if s.b[1367] { 1.0 } else { 0.0 });s.b[1368] = (p[75] == 0.0);s.store_scalar(1368, if s.b[1368] { 1.0 } else { 0.0 });s.b[1369] = (p[75] != 0.0);s.store_scalar(1369, if s.b[1369] { 1.0 } else { 0.0 });
        if ((s.b[1367] && s.b[1368]) && s.b[1369]) {s.store_add_scaled_inputs4_offset_mixed_iaai(314, 809, 1.0, A::add_scaled_product(s.ad_value(809), 1.0, s.ad_value(828), s.ad_value(232), 1.0), 0.5, A::sqrt(A::sub_scaled_inputs(A::square(A::offset(A::add_scaled_product(s.ad_value(809), 1.0, s.ad_value(828), s.ad_value(232), 1.0), (-1e-6))), 1.0, s.ad_value(809), (-(4.0 * 1e-6)))), 0.5, 809, (-1.0), (0.5 * (-1e-6)));}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_60(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((s.b[1367] && s.b[1368]) && (!s.b[1369])) {
            s.store_mul_mixed_ia(314, 809, {
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
        s.b[1370] = (p[67] == 1.0);s.store_scalar(1370, if s.b[1370] { 1.0 } else { 0.0 });s.b[1371] = (p[75] != 0.0);s.store_scalar(1371, if s.b[1371] { 1.0 } else { 0.0 });
        if (((s.b[1367] && s.b[1368]) && s.b[1370]) && s.b[1371]) {s.store_add_scaled_inputs4_offset_mixed_iaai(315, 810, 1.0, A::add_scaled_product(s.ad_value(810), 1.0, s.ad_value(829), s.ad_value(232), 1.0), 0.5, A::sqrt(A::sub_scaled_inputs(A::square(A::offset(A::add_scaled_product(s.ad_value(810), 1.0, s.ad_value(829), s.ad_value(232), 1.0), (-1e-6))), 1.0, s.ad_value(810), (-(4.0 * 1e-6)))), 0.5, 810, (-1.0), (0.5 * (-1e-6)));}
        if (((s.b[1367] && s.b[1368]) && s.b[1370]) && (!s.b[1371])) {
            s.store_mul_mixed_ia(315, 810, {
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
        s.b[1372] = (p[66] != 0.0);s.store_scalar(1372, if s.b[1372] { 1.0 } else { 0.0 });s.b[1373] = (p[75] != 0.0);s.store_scalar(1373, if s.b[1373] { 1.0 } else { 0.0 });
        if (((s.b[1367] && s.b[1368]) && s.b[1372]) && s.b[1373]) {s.store_add_scaled_inputs4_offset_mixed_iaai(316, 817, 1.0, A::add_scaled_product(s.ad_value(817), 1.0, s.ad_value(843), s.ad_value(232), 1.0), 0.5, A::sqrt(A::sub_scaled_inputs(A::square(A::offset(A::add_scaled_product(s.ad_value(817), 1.0, s.ad_value(843), s.ad_value(232), 1.0), (-1e-6))), 1.0, s.ad_value(817), (-(4.0 * 1e-6)))), 0.5, 817, (-1.0), (0.5 * (-1e-6)));}
        if (((s.b[1367] && s.b[1368]) && s.b[1372]) && (!s.b[1373])) {
            s.store_mul_mixed_ia(316, 817, {
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
        if (s.b[1367] && (!s.b[1368])) {s.store_add_scaled_product_indices(314, 809, 1.0, 828, 232, 1.0);}
        s.b[1374] = (p[67] == 1.0);s.store_scalar(1374, if s.b[1374] { 1.0 } else { 0.0 });
        if ((s.b[1367] && (!s.b[1368])) && s.b[1374]) {s.store_add_scaled_product_indices(315, 810, 1.0, 829, 232, 1.0);}
        s.b[1375] = (p[66] != 0.0);s.store_scalar(1375, if s.b[1375] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_61(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((s.b[1367] && (!s.b[1368])) && s.b[1375]) {s.store_add_scaled_product_indices(316, 817, 1.0, 843, 232, 1.0);}
        s.b[1376] = (p[75] != 0.0);s.store_scalar(1376, if s.b[1376] { 1.0 } else { 0.0 });
        if s.b[1376] {s.store_add_scaled_inputs3_mixed_iai(296, 673, 1.0, A::add_scaled_inputs3_offset(s.ad_value(232), (p[164] * 0.5), s.ad_value(673), 0.5, A::sqrt(A::sub_scaled_inputs(A::square(A::offset(A::sub_scaled_inputs(s.ad_value(232), p[164], s.ad_value(673), -1.0), (-1e-6))), 1.0, s.ad_value(673), (-(4.0 * 1e-6)))), 0.5, ((-1e-6) * 0.5)), 1.0, 673, (-1.0));}
        if (!s.b[1376]) {
            s.store_mul_mixed_ia(296, 673, {
                            if (!(((1.0 + (p[164] * s.v[232])) - 1e-6) < ((-10000.0) * 0.001))) {
                                A::add_scaled_inputs(A::scale_offset(s.ad_value(232), p[164], ((1.0) + ((-1e-6)))), 0.5, A::sqrt_square_offset(A::scale_offset(s.ad_value(232), p[164], ((1.0) + ((-1e-6)))), ((4.0 * 0.001) * 0.001)), 0.5)
                            } else {
                                {
                                    if (((1.0 + (p[164] * s.v[232])) - 1e-6) < ((-10000.0) * 0.001)) {
                                        A::div_from_scalar(((-0.001) * 0.001), A::scale_offset(s.ad_value(232), p[164], ((1.0) + ((-1e-6)))))
                                    } else {
                                        A::constant(0.0)
                                    }
                                }
                            }
                        });
        }
        s.b[1377] = (p[67] == 1.0);s.store_scalar(1377, if s.b[1377] { 1.0 } else { 0.0 });s.b[1378] = (p[75] != 0.0);s.store_scalar(1378, if s.b[1378] { 1.0 } else { 0.0 });
        if (s.b[1377] && s.b[1378]) {s.store_add_scaled_inputs3_mixed_iai(297, 675, 1.0, A::add_scaled_inputs3_offset(s.ad_value(232), (p[165] * 0.5), s.ad_value(675), 0.5, A::sqrt(A::sub_scaled_inputs(A::square(A::offset(A::sub_scaled_inputs(s.ad_value(232), p[165], s.ad_value(675), -1.0), (-1e-6))), 1.0, s.ad_value(675), (-(4.0 * 1e-6)))), 0.5, ((-1e-6) * 0.5)), 1.0, 675, (-1.0));}
        if (s.b[1377] && (!s.b[1378])) {
            s.store_mul_mixed_ia(297, 675, {
                            if (!(((1.0 + (p[165] * s.v[232])) - 1e-6) < ((-10000.0) * 0.001))) {
                                A::add_scaled_inputs(A::scale_offset(s.ad_value(232), p[165], ((1.0) + ((-1e-6)))), 0.5, A::sqrt_square_offset(A::scale_offset(s.ad_value(232), p[165], ((1.0) + ((-1e-6)))), ((4.0 * 0.001) * 0.001)), 0.5)
                            } else {
                                {
                                    if (((1.0 + (p[165] * s.v[232])) - 1e-6) < ((-10000.0) * 0.001)) {
                                        A::div_from_scalar(((-0.001) * 0.001), A::scale_offset(s.ad_value(232), p[165], ((1.0) + ((-1e-6)))))
                                    } else {
                                        A::constant(0.0)
                                    }
                                }
                            }
                        });
        }
        s.b[1379] = (p[75] != 0.0);s.store_scalar(1379, if s.b[1379] { 1.0 } else { 0.0 });
        if s.b[1379] {s.store_add_scaled_inputs3_mixed_iai(298, 677, 1.0, A::add_scaled_inputs3_offset(s.ad_value(232), (p[166] * 0.5), s.ad_value(677), 0.5, A::sqrt(A::sub_scaled_inputs(A::square(A::offset(A::sub_scaled_inputs(s.ad_value(232), p[166], s.ad_value(677), -1.0), (-1e-6))), 1.0, s.ad_value(677), (-(4.0 * 1e-6)))), 0.5, ((-1e-6) * 0.5)), 1.0, 677, (-1.0));}
        if (!s.b[1379]) {
            s.store_mul_mixed_ia(298, 677, {
                            if (!(((1.0 + (p[166] * s.v[232])) - 1e-6) < ((-10000.0) * 0.001))) {
                                A::add_scaled_inputs(A::scale_offset(s.ad_value(232), p[166], ((1.0) + ((-1e-6)))), 0.5, A::sqrt_square_offset(A::scale_offset(s.ad_value(232), p[166], ((1.0) + ((-1e-6)))), ((4.0 * 0.001) * 0.001)), 0.5)
                            } else {
                                {
                                    if (((1.0 + (p[166] * s.v[232])) - 1e-6) < ((-10000.0) * 0.001)) {
                                        A::div_from_scalar(((-0.001) * 0.001), A::scale_offset(s.ad_value(232), p[166], ((1.0) + ((-1e-6)))))
                                    } else {
                                        A::constant(0.0)
                                    }
                                }
                            }
                        });
        }
        s.b[1380] = (p[75] != 0.0);s.store_scalar(1380, if s.b[1380] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_62(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if s.b[1380] {s.store_add_scaled_inputs4_offset_mixed_iaai(322, 707, 1.0, A::add_scaled_product(s.ad_value(707), 1.0, s.ad_value(842), s.ad_value(232), 1.0), 0.5, A::sqrt(A::sub_scaled_inputs(A::square(A::offset(A::add_scaled_product(s.ad_value(707), 1.0, s.ad_value(842), s.ad_value(232), 1.0), (-1e-6))), 1.0, s.ad_value(707), (-(4.0 * 1e-6)))), 0.5, 707, (-1.0), (0.5 * (-1e-6)));}
        if (!s.b[1380]) {
            s.store_mul_mixed_ia(322, 707, {
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
        s.b[1381] = (p[75] != 0.0);s.store_scalar(1381, if s.b[1381] { 1.0 } else { 0.0 });
        if s.b[1381] {s.store_offset_add_scaled_inputs(299, A::scale_offset(s.ad_value(232), p[923], (((-(-p[917]))) + ((-1e-6)))), 0.5, A::sqrt_square_offset(A::scale_offset(s.ad_value(232), p[923], (((-(-p[917]))) + ((-1e-6)))), (-((4.0 * (-p[917])) * 1e-6))), 0.5, (((-p[917])) + (p[917])));}
        if (!s.b[1381]) {
            s.store_scale_ad(299, {
                if (!(((1.0 + (p[923] * s.v[232])) - 1e-6) < ((-10000.0) * 0.001))) {
                    A::add_scaled_inputs(A::scale_offset(s.ad_value(232), p[923], ((1.0) + ((-1e-6)))), 0.5, A::sqrt_square_offset(A::scale_offset(s.ad_value(232), p[923], ((1.0) + ((-1e-6)))), ((4.0 * 0.001) * 0.001)), 0.5)
                } else {
                    {
                        if (((1.0 + (p[923] * s.v[232])) - 1e-6) < ((-10000.0) * 0.001)) {
                            A::div_from_scalar(((-0.001) * 0.001), A::scale_offset(s.ad_value(232), p[923], ((1.0) + ((-1e-6)))))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, p[917]);
        }
        s.b[1382] = (p[66] != 0.0);s.store_scalar(1382, if s.b[1382] { 1.0 } else { 0.0 });s.b[1383] = (p[75] != 0.0);s.store_scalar(1383, if s.b[1383] { 1.0 } else { 0.0 });
        if (s.b[1382] && s.b[1383]) {s.store_offset_add_scaled_inputs(300, A::scale_offset(s.ad_value(232), p[923], (((-(-p[918]))) + ((-1e-6)))), 0.5, A::sqrt_square_offset(A::scale_offset(s.ad_value(232), p[923], (((-(-p[918]))) + ((-1e-6)))), (-((4.0 * (-p[918])) * 1e-6))), 0.5, (((-p[918])) + (p[918])));}
        if (s.b[1382] && (!s.b[1383])) {
            s.store_scale_ad(300, {
                if (!(((1.0 + (p[923] * s.v[232])) - 1e-6) < ((-10000.0) * 0.001))) {
                    A::add_scaled_inputs(A::scale_offset(s.ad_value(232), p[923], ((1.0) + ((-1e-6)))), 0.5, A::sqrt_square_offset(A::scale_offset(s.ad_value(232), p[923], ((1.0) + ((-1e-6)))), ((4.0 * 0.001) * 0.001)), 0.5)
                } else {
                    {
                        if (((1.0 + (p[923] * s.v[232])) - 1e-6) < ((-10000.0) * 0.001)) {
                            A::div_from_scalar(((-0.001) * 0.001), A::scale_offset(s.ad_value(232), p[923], ((1.0) + ((-1e-6)))))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, p[918]);
        }
        s.b[1384] = (p[75] != 0.0);s.store_scalar(1384, if s.b[1384] { 1.0 } else { 0.0 });
        if s.b[1384] {s.store_offset_add_scaled_inputs(301, A::scale_offset(s.ad_value(232), p[924], (((-(-p[919]))) + ((-1e-6)))), 0.5, A::sqrt_square_offset(A::scale_offset(s.ad_value(232), p[924], (((-(-p[919]))) + ((-1e-6)))), (-((4.0 * (-p[919])) * 1e-6))), 0.5, (((-p[919])) + (p[919])));}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_63(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (!s.b[1384]) {
            s.store_scale_ad(301, {
                if (!(((1.0 + (p[924] * s.v[232])) - 1e-6) < ((-10000.0) * 0.001))) {
                    A::add_scaled_inputs(A::scale_offset(s.ad_value(232), p[924], ((1.0) + ((-1e-6)))), 0.5, A::sqrt_square_offset(A::scale_offset(s.ad_value(232), p[924], ((1.0) + ((-1e-6)))), ((4.0 * 0.001) * 0.001)), 0.5)
                } else {
                    {
                        if (((1.0 + (p[924] * s.v[232])) - 1e-6) < ((-10000.0) * 0.001)) {
                            A::div_from_scalar(((-0.001) * 0.001), A::scale_offset(s.ad_value(232), p[924], ((1.0) + ((-1e-6)))))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, p[919]);
        }
        s.b[1385] = (p[66] != 0.0);s.store_scalar(1385, if s.b[1385] { 1.0 } else { 0.0 });s.b[1386] = (p[75] != 0.0);s.store_scalar(1386, if s.b[1386] { 1.0 } else { 0.0 });
        if (s.b[1385] && s.b[1386]) {s.store_offset_add_scaled_inputs(302, A::scale_offset(s.ad_value(232), p[924], (((-(-p[920]))) + ((-1e-6)))), 0.5, A::sqrt_square_offset(A::scale_offset(s.ad_value(232), p[924], (((-(-p[920]))) + ((-1e-6)))), (-((4.0 * (-p[920])) * 1e-6))), 0.5, (((-p[920])) + (p[920])));}
        if (s.b[1385] && (!s.b[1386])) {
            s.store_scale_ad(302, {
                if (!(((1.0 + (p[924] * s.v[232])) - 1e-6) < ((-10000.0) * 0.001))) {
                    A::add_scaled_inputs(A::scale_offset(s.ad_value(232), p[924], ((1.0) + ((-1e-6)))), 0.5, A::sqrt_square_offset(A::scale_offset(s.ad_value(232), p[924], ((1.0) + ((-1e-6)))), ((4.0 * 0.001) * 0.001)), 0.5)
                } else {
                    {
                        if (((1.0 + (p[924] * s.v[232])) - 1e-6) < ((-10000.0) * 0.001)) {
                            A::div_from_scalar(((-0.001) * 0.001), A::scale_offset(s.ad_value(232), p[924], ((1.0) + ((-1e-6)))))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, p[920]);
        }
        s.b[1387] = (p[75] != 0.0);s.store_scalar(1387, if s.b[1387] { 1.0 } else { 0.0 });
        if s.b[1387] {s.store_add_scaled_inputs4_offset_mixed_iaai(257, 700, 1.0, A::add_scaled_product(s.ad_value(700), 1.0, s.ad_value(848), s.ad_value(232), -1.0), 0.5, A::sqrt(A::sub_scaled_inputs(A::square(A::offset(A::add_scaled_product(s.ad_value(700), 1.0, s.ad_value(848), s.ad_value(232), -1.0), (-1e-6))), 1.0, s.ad_value(700), (-(4.0 * 1e-6)))), 0.5, 700, (-1.0), (0.5 * (-1e-6)));}
        if (!s.b[1387]) {
            s.store_mul_mixed_ia(257, 700, {
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
        s.b[1388] = (p[66] != 0.0);s.store_scalar(1388, if s.b[1388] { 1.0 } else { 0.0 });s.b[1389] = (p[75] != 0.0);s.store_scalar(1389, if s.b[1389] { 1.0 } else { 0.0 });
        if (s.b[1388] && s.b[1389]) {s.store_add_scaled_inputs4_offset_mixed_iaai(258, 701, 1.0, A::add_scaled_product(s.ad_value(701), 1.0, s.ad_value(848), s.ad_value(232), -1.0), 0.5, A::sqrt(A::sub_scaled_inputs(A::square(A::offset(A::add_scaled_product(s.ad_value(701), 1.0, s.ad_value(848), s.ad_value(232), -1.0), (-1e-6))), 1.0, s.ad_value(701), (-(4.0 * 1e-6)))), 0.5, 701, (-1.0), (0.5 * (-1e-6)));}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_64(
        s: &mut ReactiveScratch,
    ) {
        if (s.b[1388] && (!s.b[1389])) {
            s.store_mul_mixed_ia(258, 701, {
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
        s.store_mul_exp_mixed_ia(248, 779, A::mul(s.ad_value(860), s.ad_value(418)));
        s.store_mul_scale_offset_mixed_ia(249, 785, {
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
        }, 1.0, 0.01);s.store_add_scaled_product_indices(236, 683, 1.0, 684, 232, 1.0);s.store_add_scaled_inputs4_offset_mixed_iaai(237, 685, 1.0, A::add_scaled_product(s.ad_value(685), 1.0, s.ad_value(686), s.ad_value(232), 1.0), 0.5, A::sqrt(A::sub_scaled_inputs(A::square(A::offset(A::add_scaled_product(s.ad_value(685), 1.0, s.ad_value(686), s.ad_value(232), 1.0), (-1e-6))), 1.0, s.ad_value(685), (-(4.0 * 1e-6)))), 0.5, 685, (-1.0), (0.5 * (-1e-6)));s.store_add_scaled_inputs4_offset_mixed_iaai(238, 687, 1.0, A::add_scaled_product(s.ad_value(687), 1.0, s.ad_value(688), s.ad_value(232), 1.0), 0.5, A::sqrt(A::sub_scaled_inputs(A::square(A::offset(A::add_scaled_product(s.ad_value(687), 1.0, s.ad_value(688), s.ad_value(232), 1.0), (-1e-6))), 1.0, s.ad_value(687), (-(4.0 * 1e-6)))), 0.5, 687, (-1.0), (0.5 * (-1e-6)));s.store_add_scaled_inputs4_offset_mixed_iaai(239, 690, 1.0, A::add_scaled_product(s.ad_value(690), 1.0, s.ad_value(691), s.ad_value(232), 1.0), 0.5, A::sqrt(A::sub_scaled_inputs(A::square(A::offset(A::add_scaled_product(s.ad_value(690), 1.0, s.ad_value(691), s.ad_value(232), 1.0), (-1e-6))), 1.0, s.ad_value(690), (-(4.0 * 1e-6)))), 0.5, 690, (-1.0), (0.5 * (-1e-6)));s.store_add_scaled_product_indices(240, 692, 1.0, 693, 232, 1.0);s.store_add_scaled_product_indices(241, 798, 1.0, 800, 232, 1.0);s.store_add_scaled_product_indices(242, 799, 1.0, 801, 232, 1.0);
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_65(
        s: &mut ReactiveScratch,
    ) {
        s.store_add_scaled_inputs4_offset_mixed_iaai(293, 871, 1.0, A::add_scaled_product(s.ad_value(871), 1.0, s.ad_value(872), s.ad_value(232), 1.0), 0.5, A::sqrt(A::sub_scaled_inputs(A::square(A::offset(A::add_scaled_product(s.ad_value(871), 1.0, s.ad_value(872), s.ad_value(232), 1.0), (-1e-6))), 1.0, s.ad_value(871), (-(4.0 * 1e-6)))), 0.5, 871, (-1.0), (0.5 * (-1e-6)));s.store_add_scaled_product_indices(294, 867, 1.0, 868, 232, 1.0);s.store_add_scaled_product_indices(295, 869, 1.0, 870, 232, 1.0);s.store_add_scaled_inputs4_offset_mixed_iaai(243, 721, 1.0, A::add_scaled_product(s.ad_value(721), 1.0, s.ad_value(722), s.ad_value(232), 1.0), 0.5, A::sqrt(A::sub_scaled_inputs(A::square(A::offset(A::add_scaled_product(s.ad_value(721), 1.0, s.ad_value(722), s.ad_value(232), 1.0), (-1e-6))), 1.0, s.ad_value(721), (-(4.0 * 1e-6)))), 0.5, 721, (-1.0), (0.5 * (-1e-6)));s.store_add_scaled_inputs4_offset_mixed_iaai(244, 727, 1.0, A::add_scaled_product(s.ad_value(727), 1.0, s.ad_value(728), s.ad_value(232), 1.0), 0.5, A::sqrt(A::sub_scaled_inputs(A::square(A::offset(A::add_scaled_product(s.ad_value(727), 1.0, s.ad_value(728), s.ad_value(232), 1.0), (-1e-6))), 1.0, s.ad_value(727), (-(4.0 * 1e-6)))), 0.5, 727, (-1.0), (0.5 * (-1e-6)));s.store_add_scaled_inputs4_offset_mixed_iaai(245, 732, 1.0, A::add_scaled_product(s.ad_value(732), 1.0, s.ad_value(733), s.ad_value(232), 1.0), 0.5, A::sqrt(A::sub_scaled_inputs(A::square(A::offset(A::add_scaled_product(s.ad_value(732), 1.0, s.ad_value(733), s.ad_value(232), 1.0), (-1e-6))), 1.0, s.ad_value(732), (-(4.0 * 1e-6)))), 0.5, 732, (-1.0), (0.5 * (-1e-6)));s.store_add_scaled_inputs4_offset_mixed_iaai(246, 737, 1.0, A::add_scaled_product(s.ad_value(737), 1.0, s.ad_value(738), s.ad_value(232), 1.0), 0.5, A::sqrt(A::sub_scaled_inputs(A::square(A::offset(A::add_scaled_product(s.ad_value(737), 1.0, s.ad_value(738), s.ad_value(232), 1.0), (-1e-6))), 1.0, s.ad_value(737), (-(4.0 * 1e-6)))), 0.5, 737, (-1.0), (0.5 * (-1e-6)));s.store_add_scaled_inputs4_offset_mixed_iaai(247, 743, 1.0, A::add_scaled_product(s.ad_value(743), 1.0, s.ad_value(744), s.ad_value(232), 1.0), 0.5, A::sqrt(A::sub_scaled_inputs(A::square(A::offset(A::add_scaled_product(s.ad_value(743), 1.0, s.ad_value(744), s.ad_value(232), 1.0), (-1e-6))), 1.0, s.ad_value(743), (-(4.0 * 1e-6)))), 0.5, 743, (-1.0), (0.5 * (-1e-6)));
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_66(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.store_mul_mixed_ia(252, 748, {
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
        s.store_mul_mixed_ia(250, 762, {
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
        s.store_add_scaled_inputs3_mixed_iai(259, 775, 1.0, A::add_scaled_inputs3_offset(s.ad_value(232), (p[1437] * 0.5), s.ad_value(775), 0.5, A::sqrt(A::sub_scaled_inputs(A::square(A::offset(A::sub_scaled_inputs(s.ad_value(232), p[1437], s.ad_value(775), -1.0), (-1e-6))), 1.0, s.ad_value(775), (-(4.0 * 1e-6)))), 0.5, ((-1e-6) * 0.5)), 1.0, 775, (-1.0));s.store_add_scaled_inputs3_mixed_iai(260, 776, 1.0, A::add_scaled_inputs3_offset(s.ad_value(232), (p[1438] * 0.5), s.ad_value(776), 0.5, A::sqrt(A::sub_scaled_inputs(A::square(A::offset(A::sub_scaled_inputs(s.ad_value(232), p[1438], s.ad_value(776), -1.0), (-1e-6))), 1.0, s.ad_value(776), (-(4.0 * 1e-6)))), 0.5, ((-1e-6) * 0.5)), 1.0, 776, (-1.0));s.store_add_scaled_inputs3_mixed_iai(261, 777, 1.0, A::add_scaled_inputs3_offset(s.ad_value(232), (p[1439] * 0.5), s.ad_value(777), 0.5, A::sqrt(A::sub_scaled_inputs(A::square(A::offset(A::sub_scaled_inputs(s.ad_value(232), p[1439], s.ad_value(777), -1.0), (-1e-25))), 1.0, s.ad_value(777), (-(4.0 * 1e-25)))), 0.5, ((-1e-25) * 0.5)), 1.0, 777, (-1.0));s.store_add_scaled_inputs3_mixed_iai(262, 778, 1.0, A::add_scaled_inputs3_offset(s.ad_value(232), (p[1440] * 0.5), s.ad_value(778), 0.5, A::sqrt(A::sub_scaled_inputs(A::square(A::offset(A::sub_scaled_inputs(s.ad_value(232), p[1440], s.ad_value(778), -1.0), (-1e-20))), 1.0, s.ad_value(778), (-(4.0 * 1e-20)))), 0.5, ((-1e-20) * 0.5)), 1.0, 778, (-1.0));s.b[1390] = (p[61] != 0.0);s.store_scalar(1390, if s.b[1390] { 1.0 } else { 0.0 });s.b[1391] = (p[75] != 0.0);s.store_scalar(1391, if s.b[1391] { 1.0 } else { 0.0 });
        if (s.b[1390] && s.b[1391]) {s.store_offset_add_scaled_inputs(263, A::scale_offset(s.ad_value(232), p[1721], (((-(-p[1584]))) + ((-1e-6)))), 0.5, A::sqrt_square_offset(A::scale_offset(s.ad_value(232), p[1721], (((-(-p[1584]))) + ((-1e-6)))), (-((4.0 * (-p[1584])) * 1e-6))), 0.5, (((-p[1584])) + (p[1584])));}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_67(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (s.b[1390] && (!s.b[1391])) {
            s.store_scale_ad(263, {
                if (!(((1.0 + (p[1721] * s.v[232])) - 1e-6) < ((-10000.0) * 0.001))) {
                    A::add_scaled_inputs(A::scale_offset(s.ad_value(232), p[1721], ((1.0) + ((-1e-6)))), 0.5, A::sqrt_square_offset(A::scale_offset(s.ad_value(232), p[1721], ((1.0) + ((-1e-6)))), ((4.0 * 0.001) * 0.001)), 0.5)
                } else {
                    {
                        if (((1.0 + (p[1721] * s.v[232])) - 1e-6) < ((-10000.0) * 0.001)) {
                            A::div_from_scalar(((-0.001) * 0.001), A::scale_offset(s.ad_value(232), p[1721], ((1.0) + ((-1e-6)))))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, p[1584]);
        }
        s.b[1392] = (p[75] != 0.0);s.store_scalar(1392, if s.b[1392] { 1.0 } else { 0.0 });
        if (s.b[1390] && s.b[1392]) {s.store_offset_add_scaled_inputs(266, A::scale_offset(s.ad_value(232), p[1721], (((-(-p[1585]))) + ((-1e-6)))), 0.5, A::sqrt_square_offset(A::scale_offset(s.ad_value(232), p[1721], (((-(-p[1585]))) + ((-1e-6)))), (-((4.0 * (-p[1585])) * 1e-6))), 0.5, (((-p[1585])) + (p[1585])));}
        if (s.b[1390] && (!s.b[1392])) {
            s.store_scale_ad(266, {
                if (!(((1.0 + (p[1721] * s.v[232])) - 1e-6) < ((-10000.0) * 0.001))) {
                    A::add_scaled_inputs(A::scale_offset(s.ad_value(232), p[1721], ((1.0) + ((-1e-6)))), 0.5, A::sqrt_square_offset(A::scale_offset(s.ad_value(232), p[1721], ((1.0) + ((-1e-6)))), ((4.0 * 0.001) * 0.001)), 0.5)
                } else {
                    {
                        if (((1.0 + (p[1721] * s.v[232])) - 1e-6) < ((-10000.0) * 0.001)) {
                            A::div_from_scalar(((-0.001) * 0.001), A::scale_offset(s.ad_value(232), p[1721], ((1.0) + ((-1e-6)))))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, p[1585]);
        }
        s.b[1393] = (p[75] != 0.0);s.store_scalar(1393, if s.b[1393] { 1.0 } else { 0.0 });
        if (s.b[1390] && s.b[1393]) {s.store_offset_add_scaled_inputs(264, A::scale_offset(s.ad_value(232), p[1722], (((-(-p[1586]))) + ((-1e-6)))), 0.5, A::sqrt_square_offset(A::scale_offset(s.ad_value(232), p[1722], (((-(-p[1586]))) + ((-1e-6)))), (-((4.0 * (-p[1586])) * 1e-6))), 0.5, (((-p[1586])) + (p[1586])));}
        if (s.b[1390] && (!s.b[1393])) {
            s.store_scale_ad(264, {
                if (!(((1.0 + (p[1722] * s.v[232])) - 1e-6) < ((-10000.0) * 0.001))) {
                    A::add_scaled_inputs(A::scale_offset(s.ad_value(232), p[1722], ((1.0) + ((-1e-6)))), 0.5, A::sqrt_square_offset(A::scale_offset(s.ad_value(232), p[1722], ((1.0) + ((-1e-6)))), ((4.0 * 0.001) * 0.001)), 0.5)
                } else {
                    {
                        if (((1.0 + (p[1722] * s.v[232])) - 1e-6) < ((-10000.0) * 0.001)) {
                            A::div_from_scalar(((-0.001) * 0.001), A::scale_offset(s.ad_value(232), p[1722], ((1.0) + ((-1e-6)))))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, p[1586]);
        }
        s.b[1394] = (p[75] != 0.0);s.store_scalar(1394, if s.b[1394] { 1.0 } else { 0.0 });
        if (s.b[1390] && s.b[1394]) {s.store_offset_add_scaled_inputs(267, A::scale_offset(s.ad_value(232), p[1722], (((-(-p[1587]))) + ((-1e-6)))), 0.5, A::sqrt_square_offset(A::scale_offset(s.ad_value(232), p[1722], (((-(-p[1587]))) + ((-1e-6)))), (-((4.0 * (-p[1587])) * 1e-6))), 0.5, (((-p[1587])) + (p[1587])));}
        if (s.b[1390] && (!s.b[1394])) {
            s.store_scale_ad(267, {
                if (!(((1.0 + (p[1722] * s.v[232])) - 1e-6) < ((-10000.0) * 0.001))) {
                    A::add_scaled_inputs(A::scale_offset(s.ad_value(232), p[1722], ((1.0) + ((-1e-6)))), 0.5, A::sqrt_square_offset(A::scale_offset(s.ad_value(232), p[1722], ((1.0) + ((-1e-6)))), ((4.0 * 0.001) * 0.001)), 0.5)
                } else {
                    {
                        if (((1.0 + (p[1722] * s.v[232])) - 1e-6) < ((-10000.0) * 0.001)) {
                            A::div_from_scalar(((-0.001) * 0.001), A::scale_offset(s.ad_value(232), p[1722], ((1.0) + ((-1e-6)))))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, p[1587]);
        }
        s.b[1395] = (p[75] != 0.0);s.store_scalar(1395, if s.b[1395] { 1.0 } else { 0.0 });
        if (s.b[1390] && s.b[1395]) {s.store_offset_add_scaled_inputs(268, A::scale_offset(s.ad_value(232), p[1723], (((-(-p[1588]))) + ((-1e-6)))), 0.5, A::sqrt_square_offset(A::scale_offset(s.ad_value(232), p[1723], (((-(-p[1588]))) + ((-1e-6)))), (-((4.0 * (-p[1588])) * 1e-6))), 0.5, (((-p[1588])) + (p[1588])));}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_68(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (s.b[1390] && (!s.b[1395])) {
            s.store_scale_ad(268, {
                if (!(((1.0 + (p[1723] * s.v[232])) - 1e-6) < ((-10000.0) * 0.001))) {
                    A::add_scaled_inputs(A::scale_offset(s.ad_value(232), p[1723], ((1.0) + ((-1e-6)))), 0.5, A::sqrt_square_offset(A::scale_offset(s.ad_value(232), p[1723], ((1.0) + ((-1e-6)))), ((4.0 * 0.001) * 0.001)), 0.5)
                } else {
                    {
                        if (((1.0 + (p[1723] * s.v[232])) - 1e-6) < ((-10000.0) * 0.001)) {
                            A::div_from_scalar(((-0.001) * 0.001), A::scale_offset(s.ad_value(232), p[1723], ((1.0) + ((-1e-6)))))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, p[1588]);
        }
        s.b[1396] = (p[75] != 0.0);s.store_scalar(1396, if s.b[1396] { 1.0 } else { 0.0 });
        if (s.b[1390] && s.b[1396]) {s.store_offset_add_scaled_inputs(265, A::scale_offset(s.ad_value(232), p[1723], (((-(-p[1589]))) + ((-1e-6)))), 0.5, A::sqrt_square_offset(A::scale_offset(s.ad_value(232), p[1723], (((-(-p[1589]))) + ((-1e-6)))), (-((4.0 * (-p[1589])) * 1e-6))), 0.5, (((-p[1589])) + (p[1589])));}
        if (s.b[1390] && (!s.b[1396])) {
            s.store_scale_ad(265, {
                if (!(((1.0 + (p[1723] * s.v[232])) - 1e-6) < ((-10000.0) * 0.001))) {
                    A::add_scaled_inputs(A::scale_offset(s.ad_value(232), p[1723], ((1.0) + ((-1e-6)))), 0.5, A::sqrt_square_offset(A::scale_offset(s.ad_value(232), p[1723], ((1.0) + ((-1e-6)))), ((4.0 * 0.001) * 0.001)), 0.5)
                } else {
                    {
                        if (((1.0 + (p[1723] * s.v[232])) - 1e-6) < ((-10000.0) * 0.001)) {
                            A::div_from_scalar(((-0.001) * 0.001), A::scale_offset(s.ad_value(232), p[1723], ((1.0) + ((-1e-6)))))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, p[1589]);
        }
        if s.b[1390] {
            s.store_offset_ad(269, {
                if (!(((p[1590] - (p[1724] * s.v[232])) - 0.01) < ((-10000.0) * 0.001))) {
                    A::add_scaled_inputs(A::offset(A::sub_from_scalar(p[1590], A::scale(s.ad_value(232), p[1724])), (-0.01)), 0.5, A::sqrt_square_offset(A::offset(A::sub_from_scalar(p[1590], A::scale(s.ad_value(232), p[1724])), (-0.01)), ((4.0 * 0.001) * 0.001)), 0.5)
                } else {
                    {
                        if (((p[1590] - (p[1724] * s.v[232])) - 0.01) < ((-10000.0) * 0.001)) {
                            A::div_scalar_offset_denominator(((-0.001) * 0.001), A::sub_from_scalar(p[1590], A::scale(s.ad_value(232), p[1724])), (-0.01), 1.0)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 0.01);
        }
        if s.b[1390] {
            s.store_offset_ad(272, {
                if (!(((p[1591] - (p[1724] * s.v[232])) - 0.01) < ((-10000.0) * 0.001))) {
                    A::add_scaled_inputs(A::offset(A::sub_from_scalar(p[1591], A::scale(s.ad_value(232), p[1724])), (-0.01)), 0.5, A::sqrt_square_offset(A::offset(A::sub_from_scalar(p[1591], A::scale(s.ad_value(232), p[1724])), (-0.01)), ((4.0 * 0.001) * 0.001)), 0.5)
                } else {
                    {
                        if (((p[1591] - (p[1724] * s.v[232])) - 0.01) < ((-10000.0) * 0.001)) {
                            A::div_scalar_offset_denominator(((-0.001) * 0.001), A::sub_from_scalar(p[1591], A::scale(s.ad_value(232), p[1724])), (-0.01), 1.0)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 0.01);
        }
        if s.b[1390] {
            s.store_offset_ad(270, {
                if (!(((p[1592] - (p[1725] * s.v[232])) - 0.01) < ((-10000.0) * 0.001))) {
                    A::add_scaled_inputs(A::offset(A::sub_from_scalar(p[1592], A::scale(s.ad_value(232), p[1725])), (-0.01)), 0.5, A::sqrt_square_offset(A::offset(A::sub_from_scalar(p[1592], A::scale(s.ad_value(232), p[1725])), (-0.01)), ((4.0 * 0.001) * 0.001)), 0.5)
                } else {
                    {
                        if (((p[1592] - (p[1725] * s.v[232])) - 0.01) < ((-10000.0) * 0.001)) {
                            A::div_scalar_offset_denominator(((-0.001) * 0.001), A::sub_from_scalar(p[1592], A::scale(s.ad_value(232), p[1725])), (-0.01), 1.0)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 0.01);
        }
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_69(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if s.b[1390] {
            s.store_offset_ad(273, {
                if (!(((p[1593] - (p[1725] * s.v[232])) - 0.01) < ((-10000.0) * 0.001))) {
                    A::add_scaled_inputs(A::offset(A::sub_from_scalar(p[1593], A::scale(s.ad_value(232), p[1725])), (-0.01)), 0.5, A::sqrt_square_offset(A::offset(A::sub_from_scalar(p[1593], A::scale(s.ad_value(232), p[1725])), (-0.01)), ((4.0 * 0.001) * 0.001)), 0.5)
                } else {
                    {
                        if (((p[1593] - (p[1725] * s.v[232])) - 0.01) < ((-10000.0) * 0.001)) {
                            A::div_scalar_offset_denominator(((-0.001) * 0.001), A::sub_from_scalar(p[1593], A::scale(s.ad_value(232), p[1725])), (-0.01), 1.0)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 0.01);
        }
        if s.b[1390] {
            s.store_offset_ad(271, {
                if (!(((p[1594] - (p[1726] * s.v[232])) - 0.01) < ((-10000.0) * 0.001))) {
                    A::add_scaled_inputs(A::offset(A::sub_from_scalar(p[1594], A::scale(s.ad_value(232), p[1726])), (-0.01)), 0.5, A::sqrt_square_offset(A::offset(A::sub_from_scalar(p[1594], A::scale(s.ad_value(232), p[1726])), (-0.01)), ((4.0 * 0.001) * 0.001)), 0.5)
                } else {
                    {
                        if (((p[1594] - (p[1726] * s.v[232])) - 0.01) < ((-10000.0) * 0.001)) {
                            A::div_scalar_offset_denominator(((-0.001) * 0.001), A::sub_from_scalar(p[1594], A::scale(s.ad_value(232), p[1726])), (-0.01), 1.0)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 0.01);
        }
        if s.b[1390] {
            s.store_offset_ad(274, {
                if (!(((p[1595] - (p[1726] * s.v[232])) - 0.01) < ((-10000.0) * 0.001))) {
                    A::add_scaled_inputs(A::offset(A::sub_from_scalar(p[1595], A::scale(s.ad_value(232), p[1726])), (-0.01)), 0.5, A::sqrt_square_offset(A::offset(A::sub_from_scalar(p[1595], A::scale(s.ad_value(232), p[1726])), (-0.01)), ((4.0 * 0.001) * 0.001)), 0.5)
                } else {
                    {
                        if (((p[1595] - (p[1726] * s.v[232])) - 0.01) < ((-10000.0) * 0.001)) {
                            A::div_scalar_offset_denominator(((-0.001) * 0.001), A::sub_from_scalar(p[1595], A::scale(s.ad_value(232), p[1726])), (-0.01), 1.0)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 0.01);
        }
        if s.b[1390] {s.store_sub_ad(168, A::div(s.ad_value(147), s.ad_value(180)), A::div(s.ad_value(146), s.ad_value(179)));s.store_limited_exp_scaled_input_ad(171, A::add_scaled_inputs(s.ad_value(168), 1.0, s.ad_value(418), p[1727]), 1.0 / (p[1620]));s.store_scale(275, 171, p[1614]);s.store_scale(276, 171, p[1616]);s.store_scale(277, 171, p[1618]);s.store_limited_exp_scaled_input_ad(171, A::add_scaled_inputs(s.ad_value(168), 1.0, s.ad_value(418), p[1728]), 1.0 / (p[1621]));s.store_scale(278, 171, p[1615]);s.store_scale(279, 171, p[1617]);s.store_scale(280, 171, p[1619]);s.store_scaled_limited_exp_ad(281, A::div_scaled_product(s.ad_value(147), s.ad_value(230), p[1729], s.ad_value(179), 1.0), p[1630]);s.store_scaled_limited_exp_ad(282, A::div_scaled_product(s.ad_value(147), s.ad_value(230), p[1730], s.ad_value(179), 1.0), p[1631]);s.store_scaled_limited_exp_ad(283, A::div_scaled_product(s.ad_value(147), s.ad_value(230), p[1731], s.ad_value(179), 1.0), p[1632]);s.store_scaled_limited_exp_ad(284, A::div_scaled_product(s.ad_value(147), s.ad_value(230), p[1732], s.ad_value(179), 1.0), p[1633]);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_70(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if s.b[1390] {s.store_scaled_mul_ad(285, A::offset(A::sqrt(A::div_from_scalar(p[1636], s.ad_value(158))), 1.0), A::limited_exp(A::div_scaled_product(s.ad_value(147), s.ad_value(230), p[1733], s.ad_value(179), 1.0)), p[1634]);s.store_scaled_mul_ad(286, A::offset(A::sqrt(A::div_from_scalar(p[1636], s.ad_value(158))), 1.0), A::limited_exp(A::div_scaled_product(s.ad_value(147), s.ad_value(230), p[1734], s.ad_value(179), 1.0)), p[1635]);}
        if s.b[1390] {
            s.store_offset_ad(287, {
                if (!(((p[1637] * (1.0 + (p[1735] * s.v[230]))) - 0.01) < ((-10000.0) * 0.001))) {
                    A::add_scaled_inputs(A::scale_offset(s.ad_value(230), ((p[1735]) * (p[1637])), ((p[1637]) + ((-0.01)))), 0.5, A::sqrt_square_offset(A::scale_offset(s.ad_value(230), ((p[1735]) * (p[1637])), ((p[1637]) + ((-0.01)))), ((4.0 * 0.001) * 0.001)), 0.5)
                } else {
                    {
                        if (((p[1637] * (1.0 + (p[1735] * s.v[230]))) - 0.01) < ((-10000.0) * 0.001)) {
                            A::div_from_scalar(((-0.001) * 0.001), A::scale_offset(s.ad_value(230), ((p[1735]) * (p[1637])), ((p[1637]) + ((-0.01)))))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 0.01);
        }
        if s.b[1390] {
            s.store_offset_ad(288, {
                if (!(((p[1638] * (1.0 + (p[1736] * s.v[230]))) - 0.01) < ((-10000.0) * 0.001))) {
                    A::add_scaled_inputs(A::scale_offset(s.ad_value(230), ((p[1736]) * (p[1638])), ((p[1638]) + ((-0.01)))), 0.5, A::sqrt_square_offset(A::scale_offset(s.ad_value(230), ((p[1736]) * (p[1638])), ((p[1638]) + ((-0.01)))), ((4.0 * 0.001) * 0.001)), 0.5)
                } else {
                    {
                        if (((p[1638] * (1.0 + (p[1736] * s.v[230]))) - 0.01) < ((-10000.0) * 0.001)) {
                            A::div_from_scalar(((-0.001) * 0.001), A::scale_offset(s.ad_value(230), ((p[1736]) * (p[1638])), ((p[1638]) + ((-0.01)))))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 0.01);
        }
        if s.b[1390] {
            s.store_offset_ad(289, {
                if (!(((p[1639] * (1.0 + (p[1737] * s.v[230]))) - 0.01) < ((-10000.0) * 0.001))) {
                    A::add_scaled_inputs(A::scale_offset(s.ad_value(230), ((p[1737]) * (p[1639])), ((p[1639]) + ((-0.01)))), 0.5, A::sqrt_square_offset(A::scale_offset(s.ad_value(230), ((p[1737]) * (p[1639])), ((p[1639]) + ((-0.01)))), ((4.0 * 0.001) * 0.001)), 0.5)
                } else {
                    {
                        if (((p[1639] * (1.0 + (p[1737] * s.v[230]))) - 0.01) < ((-10000.0) * 0.001)) {
                            A::div_from_scalar(((-0.001) * 0.001), A::scale_offset(s.ad_value(230), ((p[1737]) * (p[1639])), ((p[1639]) + ((-0.01)))))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 0.01);
        }
        if s.b[1390] {
            s.store_offset_ad(290, {
                if (!(((p[1640] * (1.0 + (p[1738] * s.v[230]))) - 0.01) < ((-10000.0) * 0.001))) {
                    A::add_scaled_inputs(A::scale_offset(s.ad_value(230), ((p[1738]) * (p[1640])), ((p[1640]) + ((-0.01)))), 0.5, A::sqrt_square_offset(A::scale_offset(s.ad_value(230), ((p[1738]) * (p[1640])), ((p[1640]) + ((-0.01)))), ((4.0 * 0.001) * 0.001)), 0.5)
                } else {
                    {
                        if (((p[1640] * (1.0 + (p[1738] * s.v[230]))) - 0.01) < ((-10000.0) * 0.001)) {
                            A::div_from_scalar(((-0.001) * 0.001), A::scale_offset(s.ad_value(230), ((p[1738]) * (p[1640])), ((p[1640]) + ((-0.01)))))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 0.01);
        }
        if s.b[1390] {
            s.store_offset_ad(291, {
                if (!(((p[1641] * (1.0 + (p[1739] * s.v[230]))) - 0.01) < ((-10000.0) * 0.001))) {
                    A::add_scaled_inputs(A::scale_offset(s.ad_value(230), ((p[1739]) * (p[1641])), ((p[1641]) + ((-0.01)))), 0.5, A::sqrt_square_offset(A::scale_offset(s.ad_value(230), ((p[1739]) * (p[1641])), ((p[1641]) + ((-0.01)))), ((4.0 * 0.001) * 0.001)), 0.5)
                } else {
                    {
                        if (((p[1641] * (1.0 + (p[1739] * s.v[230]))) - 0.01) < ((-10000.0) * 0.001)) {
                            A::div_from_scalar(((-0.001) * 0.001), A::scale_offset(s.ad_value(230), ((p[1739]) * (p[1641])), ((p[1641]) + ((-0.01)))))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 0.01);
        }
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_71(
        s: &mut ReactiveScratch,
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        if s.b[1390] {
            s.store_offset_ad(292, {
                if (!(((p[1642] * (1.0 + (p[1740] * s.v[230]))) - 0.01) < ((-10000.0) * 0.001))) {
                    A::add_scaled_inputs(A::scale_offset(s.ad_value(230), ((p[1740]) * (p[1642])), ((p[1642]) + ((-0.01)))), 0.5, A::sqrt_square_offset(A::scale_offset(s.ad_value(230), ((p[1740]) * (p[1642])), ((p[1642]) + ((-0.01)))), ((4.0 * 0.001) * 0.001)), 0.5)
                } else {
                    {
                        if (((p[1642] * (1.0 + (p[1740] * s.v[230]))) - 0.01) < ((-10000.0) * 0.001)) {
                            A::div_from_scalar(((-0.001) * 0.001), A::scale_offset(s.ad_value(230), ((p[1740]) * (p[1642])), ((p[1642]) + ((-0.01)))))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 0.01);
        }
        s.b[1397] = (!param_given[1106]);s.store_scalar(1397, if s.b[1397] { 1.0 } else { 0.0 });s.b[1398] = (p[145] > 0.0);s.store_scalar(1398, if s.b[1398] { 1.0 } else { 0.0 });s.b[1399] = (p[80] == 0.0);s.store_scalar(1399, if s.b[1399] { 1.0 } else { 0.0 });
        if ((s.b[1397] && s.b[1398]) && s.b[1399]) {
            let t0: A = {
                if (!(((0.5 * s.v[146]) - (s.v[179] * (if (!((p[145] / s.v[141]) > 1e-38)) { (-87.498233534) } else { (if ((p[145] / s.v[141]) > 1e-38) { (((p[145] / s.v[141])) as f64).ln() } else { 0.0 }) }))) < ((-10000.0) * 0.0001))) {
                    A::add_scaled_inputs(A::add_scaled_product(s.ad_value(146), 0.5, s.ad_value(179), {
                        if (!((p[145] / s.v[141]) > 1e-38)) {
                            A::neg(A::constant(87.498233534))
                        } else {
                            {
                                if ((p[145] / s.v[141]) > 1e-38) {
                                    A::ln(A::div_from_scalar(p[145], s.ad_value(141)))
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }, (-1.0)), 0.5, A::sqrt_square_offset(A::add_scaled_product(s.ad_value(146), 0.5, s.ad_value(179), {
                        if (!((p[145] / s.v[141]) > 1e-38)) {
                            A::neg(A::constant(87.498233534))
                        } else {
                            {
                                if ((p[145] / s.v[141]) > 1e-38) {
                                    A::ln(A::div_from_scalar(p[145], s.ad_value(141)))
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }, (-1.0)), ((4.0 * 0.0001) * 0.0001)), 0.5)
                } else {
                    {
                        if (((0.5 * s.v[146]) - (s.v[179] * (if (!((p[145] / s.v[141]) > 1e-38)) { (-87.498233534) } else { (if ((p[145] / s.v[141]) > 1e-38) { (((p[145] / s.v[141])) as f64).ln() } else { 0.0 }) }))) < ((-10000.0) * 0.0001)) {
                            A::div_from_scalar(((-0.0001) * 0.0001), A::add_scaled_product(s.ad_value(146), 0.5, s.ad_value(179), {
                                if (!((p[145] / s.v[141]) > 1e-38)) {
                                    A::neg(A::constant(87.498233534))
                                } else {
                                    {
                                        if ((p[145] / s.v[141]) > 1e-38) {
                                            A::ln(A::div_from_scalar(p[145], s.ad_value(141)))
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
            let t1: A = {
                if (!(((0.5 * s.v[146]) - (s.v[179] * (if (!((p[97] / s.v[141]) > 1e-38)) { (-87.498233534) } else { (if ((p[97] / s.v[141]) > 1e-38) { (((p[97] / s.v[141])) as f64).ln() } else { 0.0 }) }))) < ((-10000.0) * 0.0001))) {
                    A::add_scaled_inputs(A::add_scaled_product(s.ad_value(146), 0.5, s.ad_value(179), {
                        if (!((p[97] / s.v[141]) > 1e-38)) {
                            A::neg(A::constant(87.498233534))
                        } else {
                            {
                                if ((p[97] / s.v[141]) > 1e-38) {
                                    A::ln(A::div_from_scalar(p[97], s.ad_value(141)))
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }, (-1.0)), 0.5, A::sqrt_square_offset(A::add_scaled_product(s.ad_value(146), 0.5, s.ad_value(179), {
                        if (!((p[97] / s.v[141]) > 1e-38)) {
                            A::neg(A::constant(87.498233534))
                        } else {
                            {
                                if ((p[97] / s.v[141]) > 1e-38) {
                                    A::ln(A::div_from_scalar(p[97], s.ad_value(141)))
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }, (-1.0)), ((4.0 * 0.0001) * 0.0001)), 0.5)
                } else {
                    {
                        if (((0.5 * s.v[146]) - (s.v[179] * (if (!((p[97] / s.v[141]) > 1e-38)) { (-87.498233534) } else { (if ((p[97] / s.v[141]) > 1e-38) { (((p[97] / s.v[141])) as f64).ln() } else { 0.0 }) }))) < ((-10000.0) * 0.0001)) {
                            A::div_from_scalar(((-0.0001) * 0.0001), A::add_scaled_product(s.ad_value(146), 0.5, s.ad_value(179), {
                                if (!((p[97] / s.v[141]) > 1e-38)) {
                                    A::neg(A::constant(87.498233534))
                                } else {
                                    {
                                        if ((p[97] / s.v[141]) > 1e-38) {
                                            A::ln(A::div_from_scalar(p[97], s.ad_value(141)))
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
            };s.store_mul_sub_mixed_iaa(479, 114, t0, A::add_scaled_product(s.ad_value(146), 0.5, s.ad_value(114), A::sub_scaled_inputs(s.ad_value(146), 0.5, t1, 1.0), (-1.0)));
        }
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_72(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((s.b[1397] && s.b[1398]) && (!s.b[1399])) {
            let t2: A = A::sub({
                if (!(((0.5 * s.v[146]) - (s.v[179] * ((if (!(p[145] > 1e-38)) { (-87.498233534) } else { (if (p[145] > 1e-38) { ((p[145]) as f64).ln() } else { 0.0 }) }) - s.v[142]))) < ((-10000.0) * 0.0001))) {
                    A::add_scaled_inputs3(s.ad_value(146), (0.5 * 0.5), A::mul_sub_from_scalar_rhs(s.ad_value(179), (if (!(p[145] > 1e-38)) { (-87.498233534) } else { (if (p[145] > 1e-38) { ((p[145]) as f64).ln() } else { 0.0 }) }), s.ad_value(142)), ((-1.0) * 0.5), A::sqrt_square_offset(A::sub_scaled_inputs(s.ad_value(146), 0.5, A::mul_sub_from_scalar_rhs(s.ad_value(179), (if (!(p[145] > 1e-38)) { (-87.498233534) } else { (if (p[145] > 1e-38) { ((p[145]) as f64).ln() } else { 0.0 }) }), s.ad_value(142)), 1.0), ((4.0 * 0.0001) * 0.0001)), 0.5)
                } else {
                    {
                        if (((0.5 * s.v[146]) - (s.v[179] * ((if (!(p[145] > 1e-38)) { (-87.498233534) } else { (if (p[145] > 1e-38) { ((p[145]) as f64).ln() } else { 0.0 }) }) - s.v[142]))) < ((-10000.0) * 0.0001)) {
                            A::div_from_scalar(((-0.0001) * 0.0001), A::sub_scaled_inputs(s.ad_value(146), 0.5, A::mul_sub_from_scalar_rhs(s.ad_value(179), (if (!(p[145] > 1e-38)) { (-87.498233534) } else { (if (p[145] > 1e-38) { ((p[145]) as f64).ln() } else { 0.0 }) }), s.ad_value(142)), 1.0))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, A::add_scaled_product(s.ad_value(146), 0.5, s.ad_value(114), A::sub_scaled_inputs(s.ad_value(146), 0.5, {
                if (!(((0.5 * s.v[146]) - (s.v[179] * ((if (!(p[97] > 1e-38)) { (-87.498233534) } else { (if (p[97] > 1e-38) { ((p[97]) as f64).ln() } else { 0.0 }) }) - s.v[142]))) < ((-10000.0) * 0.0001))) {
                    A::add_scaled_inputs3(s.ad_value(146), (0.5 * 0.5), A::mul_sub_from_scalar_rhs(s.ad_value(179), (if (!(p[97] > 1e-38)) { (-87.498233534) } else { (if (p[97] > 1e-38) { ((p[97]) as f64).ln() } else { 0.0 }) }), s.ad_value(142)), ((-1.0) * 0.5), A::sqrt_square_offset(A::sub_scaled_inputs(s.ad_value(146), 0.5, A::mul_sub_from_scalar_rhs(s.ad_value(179), (if (!(p[97] > 1e-38)) { (-87.498233534) } else { (if (p[97] > 1e-38) { ((p[97]) as f64).ln() } else { 0.0 }) }), s.ad_value(142)), 1.0), ((4.0 * 0.0001) * 0.0001)), 0.5)
                } else {
                    {
                        if (((0.5 * s.v[146]) - (s.v[179] * ((if (!(p[97] > 1e-38)) { (-87.498233534) } else { (if (p[97] > 1e-38) { ((p[97]) as f64).ln() } else { 0.0 }) }) - s.v[142]))) < ((-10000.0) * 0.0001)) {
                            A::div_from_scalar(((-0.0001) * 0.0001), A::sub_scaled_inputs(s.ad_value(146), 0.5, A::mul_sub_from_scalar_rhs(s.ad_value(179), (if (!(p[97] > 1e-38)) { (-87.498233534) } else { (if (p[97] > 1e-38) { ((p[97]) as f64).ln() } else { 0.0 }) }), s.ad_value(142)), 1.0))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 1.0), (-1.0)));s.store_mul_mixed_ia(479, 114, t2);
        }
        s.b[1400] = (p[80] == 0.0);s.store_scalar(1400, if s.b[1400] { 1.0 } else { 0.0 });
        if ((s.b[1397] && (!s.b[1398])) && s.b[1400]) {
            let t3: A = {
                if (!(((0.5 * s.v[146]) - (s.v[179] * (if (!((p[97] / s.v[141]) > 1e-38)) { (-87.498233534) } else { (if ((p[97] / s.v[141]) > 1e-38) { (((p[97] / s.v[141])) as f64).ln() } else { 0.0 }) }))) < ((-10000.0) * 0.0001))) {
                    A::add_scaled_inputs(A::add_scaled_product(s.ad_value(146), 0.5, s.ad_value(179), {
                        if (!((p[97] / s.v[141]) > 1e-38)) {
                            A::neg(A::constant(87.498233534))
                        } else {
                            {
                                if ((p[97] / s.v[141]) > 1e-38) {
                                    A::ln(A::div_from_scalar(p[97], s.ad_value(141)))
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }, (-1.0)), 0.5, A::sqrt_square_offset(A::add_scaled_product(s.ad_value(146), 0.5, s.ad_value(179), {
                        if (!((p[97] / s.v[141]) > 1e-38)) {
                            A::neg(A::constant(87.498233534))
                        } else {
                            {
                                if ((p[97] / s.v[141]) > 1e-38) {
                                    A::ln(A::div_from_scalar(p[97], s.ad_value(141)))
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }, (-1.0)), ((4.0 * 0.0001) * 0.0001)), 0.5)
                } else {
                    {
                        if (((0.5 * s.v[146]) - (s.v[179] * (if (!((p[97] / s.v[141]) > 1e-38)) { (-87.498233534) } else { (if ((p[97] / s.v[141]) > 1e-38) { (((p[97] / s.v[141])) as f64).ln() } else { 0.0 }) }))) < ((-10000.0) * 0.0001)) {
                            A::div_from_scalar(((-0.0001) * 0.0001), A::add_scaled_product(s.ad_value(146), 0.5, s.ad_value(179), {
                                if (!((p[97] / s.v[141]) > 1e-38)) {
                                    A::neg(A::constant(87.498233534))
                                } else {
                                    {
                                        if ((p[97] / s.v[141]) > 1e-38) {
                                            A::ln(A::div_from_scalar(p[97], s.ad_value(141)))
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
            };s.store_mul_sub_mixed_iia(479, 114, 641, A::add_scaled_product(A::scale_offset(s.ad_value(146), 0.5, p[104]), 1.0, s.ad_value(114), A::sub_scaled_inputs(s.ad_value(146), 0.5, t3, 1.0), (-1.0)));
        }
    }
}
