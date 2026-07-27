#![allow(dead_code, non_snake_case, unused_assignments, unused_imports, unused_parens, unused_variables)]
use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, ReactiveScratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};
impl Instance {
    #[inline(never)]
    pub(super) fn stamp_reactive_block_73(
        s: &mut ReactiveScratch,
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        if ((s.b[1397] && (!s.b[1398])) && (!s.b[1400])) {
            s.store_mul_sub_mixed_iia(479, 114, 641, A::add_scaled_product(A::scale_offset(s.ad_value(146), 0.5, p[104]), 1.0, s.ad_value(114), A::sub_scaled_inputs(s.ad_value(146), 0.5, {
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
            }, 1.0), (-1.0)));
        }
        if (!s.b[1397]) {s.store_scalar(479, p[1106]);}
        s.b[1401] = (!param_given[1107]);s.store_scalar(1401, if s.b[1401] { 1.0 } else { 0.0 });
        if s.b[1401] {s.copy_ad(518, 479);}
        if (!s.b[1401]) {s.store_scalar(518, p[1107]);}
        s.b[1402] = (p[80] == 0.0);s.store_scalar(1402, if s.b[1402] { 1.0 } else { 0.0 });
        if s.b[1402] {
            s.store_mul_mixed_ia(166, 179, {
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
        if s.b[1402] {s.store_scaled_add_mixed_ia(166, 166, A::sqrt_square_offset(s.ad_value(166), ((0.25 * 1e-10) * 1e-10)), 0.5);}
        if s.b[1402] {
            s.store_mul_mixed_ia(352, 179, {
                            if (!(((s.v[640] * p[97]) / (s.v[141] * s.v[141])) > 1e-38)) {
                                A::neg(A::constant(87.498233534))
                            } else {
                                {
                                    if (((s.v[640] * p[97]) / (s.v[141] * s.v[141])) > 1e-38) {
                                        A::ln(A::div_scaled_inputs(s.ad_value(640), p[97], A::square(s.ad_value(141)), 1.0))
                                    } else {
                                        A::constant(0.0)
                                    }
                                }
                            }
                        });
        }
        if (!s.b[1402]) {
            s.store_mul_sub_mixed_iai(166, 179, {
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
            }, 142);
        }
        if (!s.b[1402]) {s.store_scaled_add_mixed_ia(166, 166, A::sqrt_square_offset(s.ad_value(166), ((0.25 * 1e-10) * 1e-10)), 0.5);}
        if (!s.b[1402]) {
            s.store_mul_sub_scaled_inputs_rhs_mixed_ai(352, 179, {
                if (!((s.v[640] * p[97]) > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if ((s.v[640] * p[97]) > 1e-38) {
                            A::ln_scaled_input(s.ad_value(640), p[97])
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 1.0, 142, 2.0);
        }
        s.store_mul_sub_mixed_iia(167, 114, 641, A::offset({
            if (p[60] == 1.0) {
                A::constant(0.0)
            } else {
                s.ad_value(146)
            }
        }, p[104]));s.store_scale(407, 322, 0.5);s.store_scalar(408, 0.5);s.b[1403] = (p[60] != 1.0);s.store_scalar(1403, if s.b[1403] { 1.0 } else { 0.0 });
        if s.b[1403] {s.store_scale(407, 322, 0.333333333);s.store_scalar(408, 0.333333333);}
        s.b[1404] = (p[61] != 0.0);s.store_scalar(1404, if s.b[1404] { 1.0 } else { 0.0 });
        if s.b[1404] {s.store_add_scaled_inputs3_indices(537, 275, p[11], 276, p[13], 277, (p[3] * s.v[115]));}
        s.b[1405] = (s.v[537] > 0.0);s.store_scalar(1405, if s.b[1405] { 1.0 } else { 0.0 });
        if (s.b[1404] && s.b[1405]) {s.store_scale(539, 179, p[1620]);s.store_scaled_limited_exp_ad(547, A::div_from_scalar((-p[1626]), s.ad_value(539)), p[1628]);s.store_max_with_scalar_ad(170, A::div_from_scalar(p[1622], s.ad_value(537)), 10.0);s.store_sub_offset_lhs(226, 170, 1.0, 547);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_74(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (s.b[1404] && s.b[1405]) {
            s.store_mul_mixed_ia(546, 539, {
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
        if (s.b[1404] && s.b[1405]) {s.store_limited_exp_div(168, 546, 539);}
        if (s.b[1404] && s.b[1405]) {
            s.store_offset_ad(170, {
                if (!(((p[1624] / s.v[537]) - 10.0) < ((-10000.0) * 0.001))) {
                    A::add_scaled_inputs(A::offset(A::div_from_scalar(p[1624], s.ad_value(537)), (-10.0)), 0.5, A::sqrt_square_offset(A::offset(A::div_from_scalar(p[1624], s.ad_value(537)), (-10.0)), ((4.0 * 0.001) * 0.001)), 0.5)
                } else {
                    {
                        if (((p[1624] / s.v[537]) - 10.0) < ((-10000.0) * 0.001)) {
                            A::div_scalar_offset_denominator(((-0.001) * 0.001), A::div_from_scalar(p[1624], s.ad_value(537)), (-10.0), 1.0)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 10.0);
        }
        if (s.b[1404] && s.b[1405]) {
            s.store_sub_from_scalar_scaled_mul_mixed_ia(543, (-p[1626]), 539, {
                if (!(((s.v[170] - 1.0) / p[1628]) > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if (((s.v[170] - 1.0) / p[1628]) > 1e-38) {
                            A::ln_scaled_input(A::offset(s.ad_value(170), (-1.0)), 1.0 / (p[1628]))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 1.0);
        }
        if (s.b[1404] && s.b[1405]) {s.store_scale_ad(169, A::limited_exp_div_scaled_inputs(A::offset(s.ad_value(543), p[1626]), -1.0, s.ad_value(539), 1.0), p[1628]);s.store_mul_scale_offset_indices(542, 537, 169, 1.0, 1.0);s.store_div_scaled_product_indices(541, 537, 169, -1.0, 539, 1.0);}
        if s.b[1404] {s.store_add_scaled_inputs3_indices(538, 278, p[12], 279, p[14], 280, (p[3] * s.v[115]));}
        s.b[1406] = (s.v[538] > 0.0);s.store_scalar(1406, if s.b[1406] { 1.0 } else { 0.0 });
        if (s.b[1404] && s.b[1406]) {s.store_scale(540, 179, p[1621]);s.store_scaled_limited_exp_ad(554, A::div_from_scalar((-p[1627]), s.ad_value(540)), p[1629]);s.store_max_with_scalar_ad(170, A::div_from_scalar(p[1623], s.ad_value(538)), 10.0);s.store_sub_offset_lhs(226, 170, 1.0, 554);}
        if (s.b[1404] && s.b[1406]) {
            s.store_mul_mixed_ia(553, 540, {
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
        if (s.b[1404] && s.b[1406]) {s.store_limited_exp_div(168, 553, 540);}
        if (s.b[1404] && s.b[1406]) {
            s.store_offset_ad(170, {
                if (!(((p[1625] / s.v[538]) - 10.0) < ((-10000.0) * 0.001))) {
                    A::add_scaled_inputs(A::offset(A::div_from_scalar(p[1625], s.ad_value(538)), (-10.0)), 0.5, A::sqrt_square_offset(A::offset(A::div_from_scalar(p[1625], s.ad_value(538)), (-10.0)), ((4.0 * 0.001) * 0.001)), 0.5)
                } else {
                    {
                        if (((p[1625] / s.v[538]) - 10.0) < ((-10000.0) * 0.001)) {
                            A::div_scalar_offset_denominator(((-0.001) * 0.001), A::div_from_scalar(p[1625], s.ad_value(538)), (-10.0), 1.0)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 10.0);
        }
        if (s.b[1404] && s.b[1406]) {
            s.store_sub_from_scalar_scaled_mul_mixed_ia(550, (-p[1627]), 540, {
                if (!(((s.v[170] - 1.0) / p[1629]) > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if (((s.v[170] - 1.0) / p[1629]) > 1e-38) {
                            A::ln_scaled_input(A::offset(s.ad_value(170), (-1.0)), 1.0 / (p[1629]))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 1.0);
        }
        if (s.b[1404] && s.b[1406]) {s.store_scale_ad(169, A::limited_exp_div_scaled_inputs(A::offset(s.ad_value(550), p[1627]), -1.0, s.ad_value(540), 1.0), p[1629]);s.store_mul_scale_offset_indices(549, 538, 169, 1.0, 1.0);s.store_div_scaled_product_indices(548, 538, 169, -1.0, 540, 1.0);}
        if s.b[1404] {s.store_scale(523, 263, p[11]);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_75(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        if s.b[1404] {s.store_scale(524, 264, p[13]);s.store_scaled_mul(525, 268, 158, s.v[115]);s.store_scale(526, 266, p[12]);s.store_scale(527, 267, p[14]);s.store_scaled_mul(528, 265, 158, s.v[115]);}
        s.b[1407] = (p[1602] > 0.0);s.store_scalar(1407, if s.b[1407] { 1.0 } else { 0.0 });
        if (s.b[1404] && s.b[1407]) {s.store_scale(557, 269, (1.0 - (((1.0 / p[1602])) as f64).powf((1.0 / p[1596]))));s.store_div_scaled_inputs_mixed_ia(558, 269, (p[1602] * (p[1608] * 1.0 / (p[1596]))), A::powf(A::sub_from_scalar(1.0, A::div(s.ad_value(557), s.ad_value(269))), (-(1.0 + p[1596]))), 1.0);}
        s.b[1408] = (p[1604] > 0.0);s.store_scalar(1408, if s.b[1408] { 1.0 } else { 0.0 });
        if (s.b[1404] && s.b[1408]) {s.store_scale(559, 270, (1.0 - (((1.0 / p[1604])) as f64).powf((1.0 / p[1598]))));s.store_div_scaled_inputs_mixed_ia(560, 270, (p[1604] * (p[1610] * 1.0 / (p[1598]))), A::powf(A::sub_from_scalar(1.0, A::div(s.ad_value(559), s.ad_value(270))), (-(1.0 + p[1598]))), 1.0);}
        s.b[1409] = (p[1606] > 0.0);s.store_scalar(1409, if s.b[1409] { 1.0 } else { 0.0 });
        if (s.b[1404] && s.b[1409]) {s.store_scale(561, 271, (1.0 - (((1.0 / p[1606])) as f64).powf((1.0 / p[1600]))));s.store_div_scaled_inputs_mixed_ia(562, 271, (p[1606] * (p[1612] * 1.0 / (p[1600]))), A::powf(A::sub_from_scalar(1.0, A::div(s.ad_value(561), s.ad_value(271))), (-(1.0 + p[1600]))), 1.0);}
        s.b[1410] = (p[1603] > 0.0);s.store_scalar(1410, if s.b[1410] { 1.0 } else { 0.0 });
        if (s.b[1404] && s.b[1410]) {s.store_scale(563, 272, (1.0 - (((1.0 / p[1603])) as f64).powf((1.0 / p[1597]))));s.store_div_scaled_inputs_mixed_ia(564, 272, (p[1603] * (p[1609] * 1.0 / (p[1597]))), A::powf(A::sub_from_scalar(1.0, A::div(s.ad_value(563), s.ad_value(272))), (-(1.0 + p[1597]))), 1.0);}
        s.b[1411] = (p[1605] > 0.0);s.store_scalar(1411, if s.b[1411] { 1.0 } else { 0.0 });
        if (s.b[1404] && s.b[1411]) {s.store_scale(565, 273, (1.0 - (((1.0 / p[1605])) as f64).powf((1.0 / p[1599]))));s.store_div_scaled_inputs_mixed_ia(566, 273, (p[1605] * (p[1611] * 1.0 / (p[1599]))), A::powf(A::sub_from_scalar(1.0, A::div(s.ad_value(565), s.ad_value(273))), (-(1.0 + p[1599]))), 1.0);}
        s.b[1412] = (p[1607] > 0.0);s.store_scalar(1412, if s.b[1412] { 1.0 } else { 0.0 });
        if (s.b[1404] && s.b[1412]) {s.store_scale(567, 274, (1.0 - (((1.0 / p[1607])) as f64).powf((1.0 / p[1601]))));s.store_div_scaled_inputs_mixed_ia(568, 274, (p[1607] * (p[1613] * 1.0 / (p[1601]))), A::powf(A::sub_from_scalar(1.0, A::div(s.ad_value(567), s.ad_value(274))), (-(1.0 + p[1601]))), 1.0);}
        s.store_mul_voltage_ad(134, s.ad_value(114), ctx, nodes, Some(11), Some(6));s.store_mul_voltage_ad(135, s.ad_value(114), ctx, nodes, Some(5), Some(6));s.store_mul_voltage_ad(136, s.ad_value(114), ctx, nodes, Some(11), Some(5));s.store_mul_voltage_ad(521, s.ad_value(114), ctx, nodes, Some(3), Some(6));s.store_mul_voltage_ad(522, s.ad_value(114), ctx, nodes, Some(3), Some(5));s.store_mul_voltage_ad(497, s.ad_value(114), ctx, nodes, Some(11), Some(3));s.b[1413] = (p[76] != 2.0);s.store_scalar(1413, if s.b[1413] { 1.0 } else { 0.0 });
        if s.b[1413] {s.store_mul_voltage_ad(132, s.ad_value(114), ctx, nodes, Some(10), Some(5));s.store_mul_voltage_ad(133, s.ad_value(114), ctx, nodes, Some(10), Some(6));}
        if (!s.b[1413]) {s.store_mul_voltage_ad(132, s.ad_value(114), ctx, nodes, Some(14), Some(5));s.store_mul_voltage_ad(133, s.ad_value(114), ctx, nodes, Some(13), Some(6));}
        s.store_scalar(128, 1.0);s.b[1414] = (s.v[135] < 0.0);s.store_scalar(1414, if s.b[1414] { 1.0 } else { 0.0 });
        if s.b[1414] {s.store_scalar(128, (-1.0));s.store_sub(125, 134, 135);s.store_scale(126, 135, (-1.0));s.copy_ad(367, 522);}
        if (!s.b[1414]) {s.copy_ad(125, 134);s.copy_ad(126, 135);s.copy_ad(367, 521);}
        s.store_sub(347, 125, 167);s.store_offset_sqrt_ad(127, A::offset(A::square(s.ad_value(126)), 0.01), (-0.1));
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_76(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.b[1415] = (p[61] != 0.0);s.store_scalar(1415, if s.b[1415] { 1.0 } else { 0.0 });
        if s.b[1415] {s.store_add_scaled_inputs3_indices(368, 367, 1.0, 126, (-0.5), 127, (-(-0.5)));s.store_primal_scale(369, 689, 0.95);s.store_offset_sub(170, 369, 368, (-0.001));s.store_add_scaled_inputs3_sqrt_third_mixed_iia(370, 369, 1.0, 170, (-0.5), A::add_scaled_inputs(A::square(s.ad_value(170)), 1.0, s.ad_value(369), 0.004), (-0.5));}
        s.store_tanh_ad(168, A::div_scaled_inputs(s.ad_value(135), 0.6, s.ad_value(179), 1.0));s.store_offset_scaled(186, 168, 0.5, 0.5);s.store_sub_from_scalar(187, 1.0, 186);s.b[1416] = (p[66] != 0.0);s.store_scalar(1416, if s.b[1416] { 1.0 } else { 0.0 });
        if s.b[1416] {s.store_add_scaled_products_indices(664, 665, 187, 1.0, 663, 186, 1.0);s.store_add_scaled_products_indices(676, 298, 187, 1.0, 296, 186, 1.0);s.store_add_scaled_products_indices(427, 715, 187, 1.0, 714, 186, 1.0);s.store_add_scaled_products_indices(718, 717, 187, 1.0, 716, 186, 1.0);s.store_add_scaled_products_indices(423, 338, 187, 1.0, 337, 186, 1.0);s.store_add_scaled_products_indices(424, 258, 187, 1.0, 257, 186, 1.0);s.store_add_scaled_products_indices(422, 335, 187, 1.0, 334, 186, 1.0);s.store_add_scaled_products_indices(425, 300, 187, 1.0, 299, 186, 1.0);s.store_add_scaled_products_indices(426, 302, 187, 1.0, 301, 186, 1.0);s.store_add_scaled_products_indices(795, 796, 187, 1.0, 797, 186, 1.0);s.store_add_scaled_products_indices(428, 333, 187, 1.0, 332, 186, 1.0);s.store_add_scaled_products_indices(659, 658, 187, 1.0, 660, 186, 1.0);s.store_add_scaled_products_indices(805, 806, 187, 1.0, 804, 186, 1.0);s.store_add_scaled_products_indices(669, 668, 187, 1.0, 666, 186, 1.0);s.store_add_scaled_products_indices(416, 417, 187, 1.0, 413, 186, 1.0);s.store_add_scaled_products_indices(819, 305, 187, 1.0, 303, 186, 1.0);s.store_add_scaled_products_indices(820, 320, 187, 1.0, 318, 186, 1.0);s.store_add_scaled_products_indices(821, 316, 187, 1.0, 314, 186, 1.0);s.store_add_scaled_products_indices(822, 816, 187, 1.0, 323, 186, 1.0);}
        if (!s.b[1416]) {s.copy_ad(664, 663);s.copy_ad(676, 296);s.copy_ad(427, 714);s.copy_ad(718, 716);s.copy_ad(423, 337);s.copy_ad(424, 257);s.copy_ad(422, 334);s.copy_ad(425, 299);s.copy_ad(426, 301);s.copy_ad(795, 797);s.copy_ad(428, 332);s.copy_ad(659, 660);s.copy_ad(805, 804);s.copy_ad(669, 666);s.copy_ad(416, 413);s.copy_ad(819, 303);s.copy_ad(820, 318);s.copy_ad(821, 314);s.copy_ad(822, 323);}
        s.store_div_from_scalar(212, 1.0, 423);s.store_add_offset_lhs(353, 166, 0.4, 672);s.store_div_scaled_value_by_product_mixed_iia(169, 893, 2.0, 895, A::offset(s.ad_value(898), 2.0), 1.0);s.store_mul_add_scaled_product_rhs_indices(164, 362, 662, 1.0, 664, 127, 1.0);s.b[1417] = (p[175] == 0.0);s.store_scalar(1417, if s.b[1417] { 1.0 } else { 0.0 });s.b[1418] = (p[80] == 0.0);s.store_scalar(1418, if s.b[1418] { 1.0 } else { 0.0 });
        if (s.b[1417] && s.b[1418]) {s.store_mul_ad_product_rhs_mixed_ia(181, 179, 235, A::offset(A::div_scaled_inputs2(s.ad_value(669), 1.0, s.ad_value(164), 1.0, s.ad_value(169), 1.0), 1.0));}
        if (s.b[1417] && (!s.b[1418])) {s.store_mul_ad_product_rhs_mixed_ia(181, 182, 235, A::offset(A::div_scaled_inputs2(s.ad_value(669), 1.0, s.ad_value(164), 1.0, s.ad_value(169), 1.0), 1.0));}
        if (!s.b[1417]) {s.store_scalar(181, p[175]);}
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
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_77(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.store_add_mixed_ai(899, {
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
        s.store_add_scaled_inputs_mixed_ai(339, A::div_scaled_inputs(s.ad_value(181), 10.0, s.ad_value(898), 1.0), 1.0, 396, 2.0);s.store_div_scaled_product_indices(912, 179, 893, 1.0, 895, s.v[143]);s.store_scalar(913, ((((((4.5 * 1.05457e-34) * 3.141592653589793) * 1.60219e-19) / (4.0 * (((2.0 * s.v[381])) as f64).sqrt()))) as f64).powf(0.666666667));s.store_div_scaled_inputs_mixed_ai(914, A::powf(s.ad_value(912), 0.666666667), (p[1804] * s.v[913]), 179, 1.60219e-19);s.store_mul_ad_affine_product_rhs(354, 667, s.ad_value(361), A::sub(s.ad_value(352), s.ad_value(353)), -1.0, 0.0);s.store_add_ad(355, A::mul3_scaled_output(s.ad_value(676), s.ad_value(363), A::add_scaled_product(s.ad_value(127), 1.0, s.ad_value(674), A::sqrt(A::offset(s.ad_value(127), 0.01)), 1.0), -1.0), A::mul3(s.ad_value(681), s.ad_value(365), A::pow(A::offset(s.ad_value(127), 0.01), s.ad_value(682))));s.store_mul_ad_product_rhs_mixed_ia(357, 802, 364, A::sqrt(s.ad_value(353)));s.store_add_mixed_ai(358, A::add_scaled_inputs4(s.ad_value(354), 1.0, s.ad_value(355), 1.0, s.ad_value(357), 1.0, s.ad_value(231), 1.0), 805);s.store_sub(347, 347, 358);s.store_div_scaled_product3_indices(184, 416, 163, 158, 1.0, 153, 1.0);s.b[1419] = (p[80] == 0.0);s.store_scalar(1419, if s.b[1419] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_78(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if s.b[1419] {s.store_pow_ad(171, A::div_scaled_inputs(s.ad_value(163), (2.0 * p[108]), A::mul3_scaled_output(s.ad_value(184), s.ad_value(181), s.ad_value(148), (1.60219e-19 * p[3])), 1.0), s.ad_value(181));}
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
        if s.b[1419] {s.store_offset_add(169, 347, 168, p[23]);}
        if s.b[1419] {
            s.store_sub_mixed_ai(348, {
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
            s.store_mul_scale_offset_mixed_ia(168, 181, {
                if (!((((2.0 * s.v[163]) * p[108]) / ((((s.v[184] * s.v[181]) * 1.60219e-19) * s.v[148]) * p[3])) > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if ((((2.0 * s.v[163]) * p[108]) / ((((s.v[184] * s.v[181]) * 1.60219e-19) * s.v[148]) * p[3])) > 1e-38) {
                            A::ln(A::div_scaled_inputs(s.ad_value(163), (2.0 * p[108]), A::mul3_scaled_output(s.ad_value(184), s.ad_value(181), s.ad_value(148), (1.60219e-19 * p[3])), 1.0))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, -1.0, 0.0);
        }
        if (!s.b[1419]) {s.store_sub_mixed_ai(169, A::add_scaled_inputs(A::offset(s.ad_value(168), 0.01), 0.5, A::sqrt_square_offset(A::offset(s.ad_value(168), (-0.01)), ((0.25 * 0.0001) * 0.0001)), 0.5), 375);s.store_offset_add(170, 347, 169, p[23]);}
        if (!s.b[1419]) {
            s.store_sub_mixed_ai(348, {
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
        s.copy_ad(129, 375);s.store_powf_ad(172, A::neg(s.ad_value(897)), 0.666666667);s.b[1420] = (p[61] != 0.0);s.store_scalar(1420, if s.b[1420] { 1.0 } else { 0.0 });
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
        if s.b[1420] {s.store_mul_div_scaled_inputs_mixed_aii(171, A::sub(A::sqrt(s.ad_value(169)), A::sqrt_scaled_input(s.ad_value(166), 2.0)), 239, -1.0, 181, 2.0);s.store_add_scaled_product_mixed_aii(168, A::add_scaled_inputs3(s.ad_value(897), -1.0, s.ad_value(171), (-1.0), s.ad_value(899), 1.0), 1.0, 914, 172, 1.0);s.store_add_scaled_inputs3_indices(169, 897, -1.0, 171, (-1.0), 900, 1.0);}
        if (!s.b[1420]) {s.store_add_scaled_inputs_product_indices(168, 899, 1.0, 897, (-1.0), 914, 172, 1.0);s.store_sub(169, 900, 897);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_79(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.store_div_scaled_inputs2_indices(170, 348, 1.0, 129, (-1.0), 181, 1.0);s.store_sub(924, 169, 170);s.store_scaled_sub(171, 170, 168, 0.5);s.store_limited_exp(901, 171);s.b[1421] = (s.v[901] > 1e-7);s.store_scalar(1421, if s.b[1421] { 1.0 } else { 0.0 });
        if s.b[1421] {s.store_ln_offset_input(176, 901, 1.0);s.store_offset_scaled_ad(901, A::sqrt_square_offset(s.ad_value(176), 1.0), (-2.0), 2.0);s.store_mul_add_scaled_inputs_rhs_indices(177, 898, 901, p[1805], 897, 1.0);s.store_div_scaled_value_offset_denominator(172, s.ad_value(177), 1.0, A::sub(A::limited_exp(s.ad_value(177)), s.ad_value(177)), (-1.0), 1.0);s.store_mul(174, 177, 172);s.store_ln_neg_add(902, 901, 897);}
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
        if s.b[1421] {s.store_add_scaled_offset_product_lhs_product_mixed_aaiia(345, A::offset(A::div_from_scalar(1.0, s.ad_value(901)), (-1.0)), 1.0, A::sub(A::div_from_scalar(2.0, s.ad_value(177)), s.ad_value(172)), (-1.0), 898, 1.0, 914, A::exp_scaled_input(s.ad_value(902), (-0.333333333)), (-0.666666667));s.store_add_scaled_product_mixed_aia(346, A::div_from_scalar((-1.0), A::square(s.ad_value(901))), 1.0, 914, A::exp_scaled_input(s.ad_value(902), (-1.333333333)), (-(2.0 / 9.0)));s.store_add_scaled_offset_product_rhs_mixed_iaa(901, 901, 1.0, A::div(s.ad_value(344), s.ad_value(345)), A::div_scaled_product_by_product(s.ad_value(344), s.ad_value(346), 1.0, s.ad_value(345), s.ad_value(345), 2.0), 1.0, (-1.0));s.store_mul_add_scaled_inputs_rhs_indices(177, 898, 901, p[1805], 897, 1.0);s.store_div_scaled_value_offset_denominator(172, s.ad_value(177), 1.0, A::sub(A::limited_exp(s.ad_value(177)), s.ad_value(177)), (-1.0), 1.0);s.store_mul(174, 177, 172);s.store_ln_neg_add(902, 901, 897);}
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
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_80(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if s.b[1421] {s.store_add_scaled_offset_product_lhs_product_mixed_aaiia(345, A::offset(A::div_from_scalar(1.0, s.ad_value(901)), (-1.0)), 1.0, A::sub(A::div_from_scalar(2.0, s.ad_value(177)), s.ad_value(172)), (-1.0), 898, 1.0, 914, A::exp_scaled_input(s.ad_value(902), (-0.333333333)), (-0.666666667));s.store_add_scaled_product_mixed_aia(346, A::div_from_scalar((-1.0), A::square(s.ad_value(901))), 1.0, 914, A::exp_scaled_input(s.ad_value(902), (-1.333333333)), (-(2.0 / 9.0)));s.store_add_scaled_offset_product_rhs_mixed_iaa(901, 901, 1.0, A::div(s.ad_value(344), s.ad_value(345)), A::div_scaled_product_by_product(s.ad_value(344), s.ad_value(346), 1.0, s.ad_value(345), s.ad_value(345), 2.0), 1.0, (-1.0));}
        if (!s.b[1421]) {s.store_mul_scale_offset_indices(901, 901, 901, -1.0, 0.0);}
        s.store_mul_scale_offset_indices(392, 181, 901, -1.0, 0.0);s.b[1422] = (p[57] == 1.0);s.store_scalar(1422, if s.b[1422] { 1.0 } else { 0.0 });
        if s.b[1422] {s.store_div_scaled_inputs2_indices(1015, 347, 1.0, 129, (-1.0), 181, 1.0);s.store_scaled_add_mixed_ia(1016, 1015, A::sqrt(A::add_scaled_square_product(s.ad_value(1015), 1.0, s.ad_value(963), s.ad_value(963), 0.25)), 0.5);s.store_pow_ad(1017, s.ad_value(1016), A::scale(s.ad_value(960), 0.5));s.store_mul_ad_product_rhs_mixed_ia(1004, 1010, 1017, A::limited_exp(A::sub(s.ad_value(1015), s.ad_value(1016))));s.store_div_scaled_inputs3_indices(1018, 347, 1.0, 129, (-1.0), 985, -1.0, 181, 1.0);s.store_scaled_add_mixed_ia(1019, 1018, A::sqrt(A::add_scaled_square_product(s.ad_value(1018), 1.0, s.ad_value(964), s.ad_value(964), 0.25)), 0.5);s.store_pow_ad(1020, s.ad_value(1019), A::scale(s.ad_value(961), 0.5));s.store_mul_ad_product_rhs_mixed_ia(1005, 1011, 1020, A::limited_exp(A::sub(s.ad_value(1018), s.ad_value(1019))));s.store_div_scaled_inputs3_indices(1021, 347, 1.0, 129, (-1.0), 986, -1.0, 181, 1.0);s.store_scaled_add_mixed_ia(1022, 1021, A::sqrt(A::add_scaled_square_product(s.ad_value(1021), 1.0, s.ad_value(965), s.ad_value(965), 0.25)), 0.5);s.store_pow_ad(1023, s.ad_value(1022), A::scale(s.ad_value(962), 0.5));s.store_mul_ad_product_rhs_mixed_ia(1006, 1012, 1023, A::limited_exp(A::sub(s.ad_value(1021), s.ad_value(1022))));s.store_add_scaled_products_mixed_iiia(392, 983, 392, 1.0, 984, A::add_scaled_inputs3(s.ad_value(1004), 1.0, s.ad_value(1005), 1.0, s.ad_value(1006), 1.0), 1.0);}
        s.store_primal_div_from_scalar(406, 0.01, 163);s.store_add_scaled_product_indices(419, 396, s.v[420], 407, 392, s.v[420]);s.store_pow_ad(170, A::scaled_offset(A::div(s.ad_value(392), s.ad_value(406)), 1.0, 0.5), s.ad_value(317));s.store_pow_indices(171, 419, 822);s.b[1423] = (p[61] != 0.0);s.store_scalar(1423, if s.b[1423] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_81(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if s.b[1423] {s.store_add_scaled_product_mixed_aai(171, A::div(s.ad_value(820), s.ad_value(170)), 1.0, A::add_scaled_product(s.ad_value(819), 1.0, s.ad_value(821), s.ad_value(370), 1.0), 171, 1.0);}
        if (!s.b[1423]) {s.store_add_scaled_product_mixed_aii(171, A::div(s.ad_value(820), s.ad_value(170)), 1.0, 819, 171, 1.0);}
        s.store_offset(397, 171, 1.0);s.store_scaled_add_offset_sqrt_square_offset(397, 397, 1.0, (-1.0), ((0.25 * p[604]) * p[604]), 0.5);s.store_scale(397, 397, 1.0 / (p[24]));s.b[1424] = (p[64] == 1.0);s.store_scalar(1424, if s.b[1424] { 1.0 } else { 0.0 });
        if s.b[1424] {s.store_scalar(198, 0.0);}
        s.b[1425] = (p[64] == 0.0);s.store_scalar(1425, if s.b[1425] { 1.0 } else { 0.0 });
        if ((!s.b[1424]) && s.b[1425]) {s.store_offset_mul(172, 711, 392, 1.0);s.store_div_from_scalar(169, 1.0, 172);s.store_scaled_add_mixed_ia(168, 169, A::sqrt_square_offset(s.ad_value(169), 0.01), 0.5);s.store_mul_ad_affine_product_lhs(198, A::offset(A::mul(s.ad_value(709), s.ad_value(168)), p[908]), s.ad_value(189), s.v[115], 0.0, 194);}
        if ((!s.b[1424]) && (!s.b[1425])) {s.store_offset_mul(172, 711, 392, 1.0);s.store_div_from_scalar(169, 1.0, 172);s.store_scaled_add_mixed_ia(168, 169, A::sqrt_square_offset(s.ad_value(169), 0.01), 0.5);s.store_mul_mixed_ai(198, A::add_scaled_inputs_product(s.ad_value(190), 1.0, s.ad_value(191), 1.0, A::offset(A::mul(s.ad_value(709), s.ad_value(168)), p[908]), s.ad_value(189), s.v[115]), 194);}
        s.store_mul_div_scaled_inputs_indices(216, 397, 428, 2.0, 416, 1.0);s.store_mul(217, 216, 153);s.b[1426] = (p[80] == 0.0);s.store_scalar(1426, if s.b[1426] { 1.0 } else { 0.0 });
        if s.b[1426] {s.store_mul_add_scaled_inputs_rhs_indices(175, 659, 392, 1.0, 179, 2.0);}
        if (!s.b[1426]) {s.store_mul_add_scaled_inputs_rhs_indices(175, 659, 392, 1.0, 182, 2.0);}
        s.b[1427] = (s.v[198] > 0.0);s.store_scalar(1427, if s.b[1427] { 1.0 } else { 0.0 });
        if s.b[1427] {s.store_mul3_lhs(224, 158, 428, 163);s.store_mul(168, 224, 198);s.store_scale(225, 168, 2.0);s.store_add_scaled_inputs_product_indices(226, 175, 1.0, 217, 1.0, 175, 168, 3.0);s.store_mul_add_scaled_product_rhs_indices(227, 175, 217, 1.0, 175, 168, 2.0);s.store_div_scaled_inputs2_by_product_mixed_aaai(210, A::square(s.ad_value(226)), 1.0, A::add_scaled_square_product(s.ad_value(226), 1.0, s.ad_value(225), s.ad_value(227), (-2.0)), (-1.0), A::add(s.ad_value(226), A::sqrt(A::add_scaled_square_product(s.ad_value(226), 1.0, s.ad_value(225), s.ad_value(227), (-2.0)))), 225, 1.0);}
        if (!s.b[1427]) {s.store_div_scaled_product_add_scaled_denominator_indices(210, 217, 175, 1.0, 217, 1.0, 175, 1.0, 1.0);}
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
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_82(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.store_pow_ad(176, A::offset(A::div(s.ad_value(126), s.ad_value(210)), 1e-6), s.ad_value(423));s.store_pow_ad(177, A::offset(s.ad_value(176), 1.0), s.ad_value(212));s.store_min_ad(390, A::div(s.ad_value(126), s.ad_value(177)), s.ad_value(126));s.store_add(129, 390, 375);s.store_powf_ad(170, A::neg(s.ad_value(897)), 0.666666667);s.b[1428] = (p[61] != 0.0);s.store_scalar(1428, if s.b[1428] { 1.0 } else { 0.0 });
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
        if s.b[1428] {s.store_mul_div_scaled_inputs_mixed_aii(171, A::sub(A::sqrt(s.ad_value(169)), A::sqrt_scaled_input(s.ad_value(166), 2.0)), 239, -1.0, 181, 2.0);s.store_add_scaled_product_mixed_aii(168, A::add_scaled_inputs3(s.ad_value(897), -1.0, s.ad_value(171), (-1.0), s.ad_value(899), 1.0), 1.0, 914, 170, 1.0);s.store_add_scaled_inputs3_indices(169, 897, -1.0, 171, (-1.0), 900, 1.0);}
        if (!s.b[1428]) {s.store_add_scaled_inputs_product_indices(168, 899, 1.0, 897, (-1.0), 914, 170, 1.0);s.store_sub(169, 900, 897);}
        s.store_div_scaled_inputs2_indices(170, 348, 1.0, 129, (-1.0), 181, 1.0);s.store_sub(924, 169, 170);s.store_scaled_sub(171, 170, 168, 0.5);s.store_limited_exp(901, 171);s.b[1429] = (s.v[901] > 1e-7);s.store_scalar(1429, if s.b[1429] { 1.0 } else { 0.0 });
        if s.b[1429] {s.store_ln_offset_input(176, 901, 1.0);s.store_offset_scaled_ad(901, A::sqrt_square_offset(s.ad_value(176), 1.0), (-2.0), 2.0);s.store_mul_add_scaled_inputs_rhs_indices(177, 898, 901, p[1805], 897, 1.0);s.store_div_scaled_value_offset_denominator(172, s.ad_value(177), 1.0, A::sub(A::limited_exp(s.ad_value(177)), s.ad_value(177)), (-1.0), 1.0);s.store_mul(174, 177, 172);s.store_ln_neg_add(902, 901, 897);}
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
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_83(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if s.b[1429] {s.store_add_scaled_offset_product_lhs_product_mixed_aaiia(345, A::offset(A::div_from_scalar(1.0, s.ad_value(901)), (-1.0)), 1.0, A::sub(A::div_from_scalar(2.0, s.ad_value(177)), s.ad_value(172)), (-1.0), 898, 1.0, 914, A::exp_scaled_input(s.ad_value(902), (-0.333333333)), (-0.666666667));s.store_add_scaled_product_mixed_aia(346, A::div_from_scalar((-1.0), A::square(s.ad_value(901))), 1.0, 914, A::exp_scaled_input(s.ad_value(902), (-1.333333333)), (-(2.0 / 9.0)));s.store_add_scaled_offset_product_rhs_mixed_iaa(901, 901, 1.0, A::div(s.ad_value(344), s.ad_value(345)), A::div_scaled_product_by_product(s.ad_value(344), s.ad_value(346), 1.0, s.ad_value(345), s.ad_value(345), 2.0), 1.0, (-1.0));s.store_mul_add_scaled_inputs_rhs_indices(177, 898, 901, p[1805], 897, 1.0);s.store_div_scaled_value_offset_denominator(172, s.ad_value(177), 1.0, A::sub(A::limited_exp(s.ad_value(177)), s.ad_value(177)), (-1.0), 1.0);s.store_mul(174, 177, 172);s.store_ln_neg_add(902, 901, 897);}
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
        if s.b[1429] {s.store_add_scaled_offset_product_lhs_product_mixed_aaiia(345, A::offset(A::div_from_scalar(1.0, s.ad_value(901)), (-1.0)), 1.0, A::sub(A::div_from_scalar(2.0, s.ad_value(177)), s.ad_value(172)), (-1.0), 898, 1.0, 914, A::exp_scaled_input(s.ad_value(902), (-0.333333333)), (-0.666666667));s.store_add_scaled_product_mixed_aia(346, A::div_from_scalar((-1.0), A::square(s.ad_value(901))), 1.0, 914, A::exp_scaled_input(s.ad_value(902), (-1.333333333)), (-(2.0 / 9.0)));s.store_add_scaled_offset_product_rhs_mixed_iaa(901, 901, 1.0, A::div(s.ad_value(344), s.ad_value(345)), A::div_scaled_product_by_product(s.ad_value(344), s.ad_value(346), 1.0, s.ad_value(345), s.ad_value(345), 2.0), 1.0, (-1.0));}
        if (!s.b[1429]) {s.store_mul_scale_offset_indices(901, 901, 901, -1.0, 0.0);}
        s.store_mul_scale_offset_indices(393, 181, 901, -1.0, 0.0);s.b[1430] = (p[57] == 1.0);s.store_scalar(1430, if s.b[1430] { 1.0 } else { 0.0 });
        if s.b[1430] {s.store_div_scaled_inputs2_indices(1015, 347, 1.0, 129, (-1.0), 181, 1.0);s.store_scaled_add_mixed_ia(1016, 1015, A::sqrt(A::add_scaled_square_product(s.ad_value(1015), 1.0, s.ad_value(963), s.ad_value(963), 0.25)), 0.5);s.store_pow_ad(1017, s.ad_value(1016), A::scale(s.ad_value(960), 0.5));s.store_mul_ad_product_rhs_mixed_ia(1007, 1010, 1017, A::limited_exp(A::sub(s.ad_value(1015), s.ad_value(1016))));s.store_div_scaled_inputs3_indices(1018, 347, 1.0, 129, (-1.0), 985, -1.0, 181, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_84(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if s.b[1430] {s.store_scaled_add_mixed_ia(1019, 1018, A::sqrt(A::add_scaled_square_product(s.ad_value(1018), 1.0, s.ad_value(964), s.ad_value(964), 0.25)), 0.5);s.store_pow_ad(1020, s.ad_value(1019), A::scale(s.ad_value(961), 0.5));s.store_mul_ad_product_rhs_mixed_ia(1008, 1011, 1020, A::limited_exp(A::sub(s.ad_value(1018), s.ad_value(1019))));s.store_div_scaled_inputs3_indices(1021, 347, 1.0, 129, (-1.0), 986, -1.0, 181, 1.0);s.store_scaled_add_mixed_ia(1022, 1021, A::sqrt(A::add_scaled_square_product(s.ad_value(1021), 1.0, s.ad_value(965), s.ad_value(965), 0.25)), 0.5);s.store_pow_ad(1023, s.ad_value(1022), A::scale(s.ad_value(962), 0.5));s.store_mul_ad_product_rhs_mixed_ia(1009, 1012, 1023, A::limited_exp(A::sub(s.ad_value(1021), s.ad_value(1022))));s.store_add_scaled_products_mixed_iiia(393, 983, 393, 1.0, 984, A::add_scaled_inputs3(s.ad_value(1007), 1.0, s.ad_value(1008), 1.0, s.ad_value(1009), 1.0), 1.0);}
        s.b[1431] = (p[67] == 1.0);s.store_scalar(1431, if s.b[1431] { 1.0 } else { 0.0 });
        if s.b[1431] {s.store_add_ad(356, A::mul3_scaled_output(s.ad_value(297), s.ad_value(363), A::add_scaled_product(s.ad_value(127), 1.0, s.ad_value(674), A::sqrt(A::offset(s.ad_value(127), 0.01)), 1.0), -1.0), A::mul3(s.ad_value(681), s.ad_value(365), A::pow(A::offset(s.ad_value(127), 0.01), s.ad_value(682))));s.store_add_mixed_ai(359, A::add_scaled_inputs4(s.ad_value(354), 1.0, s.ad_value(356), 1.0, s.ad_value(357), 1.0, s.ad_value(231), 1.0), 805);s.store_add_scaled_inputs3_indices(349, 125, 1.0, 167, (-1.0), 359, -1.0);s.store_div_scaled_product3_indices(185, 414, 163, 158, 1.0, 153, 1.0);}
        s.b[1432] = (p[80] == 0.0);s.store_scalar(1432, if s.b[1432] { 1.0 } else { 0.0 });
        if (s.b[1431] && s.b[1432]) {s.store_pow_ad(171, A::div_scaled_inputs(s.ad_value(163), (2.0 * p[108]), A::mul3_scaled_output(s.ad_value(185), s.ad_value(181), s.ad_value(148), (1.60219e-19 * p[3])), 1.0), s.ad_value(181));}
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
        if (s.b[1431] && s.b[1432]) {s.store_offset_add(169, 349, 168, p[23]);}
        if (s.b[1431] && s.b[1432]) {
            s.store_sub_mixed_ai(350, {
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
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_85(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (s.b[1431] && (!s.b[1432])) {
            s.store_mul_scale_offset_mixed_ia(168, 181, {
                if (!((((2.0 * s.v[163]) * p[108]) / ((((s.v[185] * s.v[181]) * 1.60219e-19) * s.v[148]) * p[3])) > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if ((((2.0 * s.v[163]) * p[108]) / ((((s.v[185] * s.v[181]) * 1.60219e-19) * s.v[148]) * p[3])) > 1e-38) {
                            A::ln(A::div_scaled_inputs(s.ad_value(163), (2.0 * p[108]), A::mul3_scaled_output(s.ad_value(185), s.ad_value(181), s.ad_value(148), (1.60219e-19 * p[3])), 1.0))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, -1.0, 0.0);
        }
        if (s.b[1431] && (!s.b[1432])) {s.store_sub_mixed_ai(169, A::add_scaled_inputs(A::offset(s.ad_value(168), 0.01), 0.5, A::sqrt_square_offset(A::offset(s.ad_value(168), (-0.01)), ((0.25 * 0.0001) * 0.0001)), 0.5), 375);s.store_offset_add(170, 349, 169, p[23]);}
        if (s.b[1431] && (!s.b[1432])) {
            s.store_sub_mixed_ai(350, {
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
        if s.b[1431] {s.copy_ad(130, 375);s.store_powf_ad(172, A::neg(s.ad_value(897)), 0.666666667);}
        s.b[1433] = (p[61] != 0.0);s.store_scalar(1433, if s.b[1433] { 1.0 } else { 0.0 });
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
        if (s.b[1431] && s.b[1433]) {s.store_mul_div_scaled_inputs_mixed_aii(171, A::sub(A::sqrt(s.ad_value(169)), A::sqrt_scaled_input(s.ad_value(166), 2.0)), 239, -1.0, 181, 2.0);s.store_add_scaled_product_mixed_aii(168, A::add_scaled_inputs3(s.ad_value(897), -1.0, s.ad_value(171), (-1.0), s.ad_value(899), 1.0), 1.0, 914, 172, 1.0);s.store_add_scaled_inputs3_indices(169, 897, -1.0, 171, (-1.0), 900, 1.0);}
        if (s.b[1431] && (!s.b[1433])) {s.store_add_scaled_inputs_product_indices(168, 899, 1.0, 897, (-1.0), 914, 172, 1.0);s.store_sub(169, 900, 897);}
        if s.b[1431] {s.store_div_scaled_inputs2_indices(170, 350, 1.0, 130, (-1.0), 181, 1.0);s.store_sub(924, 169, 170);s.store_scaled_sub(171, 170, 168, 0.5);s.store_limited_exp(901, 171);}
        s.b[1434] = (s.v[901] > 1e-7);s.store_scalar(1434, if s.b[1434] { 1.0 } else { 0.0 });
        if (s.b[1431] && s.b[1434]) {s.store_ln_offset_input(176, 901, 1.0);s.store_offset_scaled_ad(901, A::sqrt_square_offset(s.ad_value(176), 1.0), (-2.0), 2.0);s.store_mul_add_scaled_inputs_rhs_indices(177, 898, 901, p[1805], 897, 1.0);s.store_div_scaled_value_offset_denominator(172, s.ad_value(177), 1.0, A::sub(A::limited_exp(s.ad_value(177)), s.ad_value(177)), (-1.0), 1.0);s.store_mul(174, 177, 172);s.store_ln_neg_add(902, 901, 897);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_86(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
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
        if (s.b[1431] && s.b[1434]) {s.store_add_scaled_offset_product_lhs_product_mixed_aaiia(345, A::offset(A::div_from_scalar(1.0, s.ad_value(901)), (-1.0)), 1.0, A::sub(A::div_from_scalar(2.0, s.ad_value(177)), s.ad_value(172)), (-1.0), 898, 1.0, 914, A::exp_scaled_input(s.ad_value(902), (-0.333333333)), (-0.666666667));s.store_add_scaled_product_mixed_aia(346, A::div_from_scalar((-1.0), A::square(s.ad_value(901))), 1.0, 914, A::exp_scaled_input(s.ad_value(902), (-1.333333333)), (-(2.0 / 9.0)));s.store_add_scaled_offset_product_rhs_mixed_iaa(901, 901, 1.0, A::div(s.ad_value(344), s.ad_value(345)), A::div_scaled_product_by_product(s.ad_value(344), s.ad_value(346), 1.0, s.ad_value(345), s.ad_value(345), 2.0), 1.0, (-1.0));s.store_mul_add_scaled_inputs_rhs_indices(177, 898, 901, p[1805], 897, 1.0);s.store_div_scaled_value_offset_denominator(172, s.ad_value(177), 1.0, A::sub(A::limited_exp(s.ad_value(177)), s.ad_value(177)), (-1.0), 1.0);s.store_mul(174, 177, 172);s.store_ln_neg_add(902, 901, 897);}
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
        if (s.b[1431] && s.b[1434]) {s.store_add_scaled_offset_product_lhs_product_mixed_aaiia(345, A::offset(A::div_from_scalar(1.0, s.ad_value(901)), (-1.0)), 1.0, A::sub(A::div_from_scalar(2.0, s.ad_value(177)), s.ad_value(172)), (-1.0), 898, 1.0, 914, A::exp_scaled_input(s.ad_value(902), (-0.333333333)), (-0.666666667));s.store_add_scaled_product_mixed_aia(346, A::div_from_scalar((-1.0), A::square(s.ad_value(901))), 1.0, 914, A::exp_scaled_input(s.ad_value(902), (-1.333333333)), (-(2.0 / 9.0)));s.store_add_scaled_offset_product_rhs_mixed_iaa(901, 901, 1.0, A::div(s.ad_value(344), s.ad_value(345)), A::div_scaled_product_by_product(s.ad_value(344), s.ad_value(346), 1.0, s.ad_value(345), s.ad_value(345), 2.0), 1.0, (-1.0));}
        if (s.b[1431] && (!s.b[1434])) {s.store_mul_scale_offset_indices(901, 901, 901, -1.0, 0.0);}
        if s.b[1431] {s.store_mul_scale_offset_indices(394, 181, 901, -1.0, 0.0);}
        s.b[1435] = (p[57] == 1.0);s.store_scalar(1435, if s.b[1435] { 1.0 } else { 0.0 });
        if (s.b[1431] && s.b[1435]) {s.store_div_scaled_inputs2_indices(1015, 349, 1.0, 130, (-1.0), 181, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_87(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (s.b[1431] && s.b[1435]) {s.store_scaled_add_mixed_ia(1016, 1015, A::sqrt(A::add_scaled_square_product(s.ad_value(1015), 1.0, s.ad_value(963), s.ad_value(963), 0.25)), 0.5);s.store_pow_ad(1017, s.ad_value(1016), A::scale(s.ad_value(960), 0.5));s.store_mul_ad_product_rhs_mixed_ia(1004, 1010, 1017, A::limited_exp(A::sub(s.ad_value(1015), s.ad_value(1016))));s.store_div_scaled_inputs3_indices(1018, 349, 1.0, 130, (-1.0), 985, -1.0, 181, 1.0);s.store_scaled_add_mixed_ia(1019, 1018, A::sqrt(A::add_scaled_square_product(s.ad_value(1018), 1.0, s.ad_value(964), s.ad_value(964), 0.25)), 0.5);s.store_pow_ad(1020, s.ad_value(1019), A::scale(s.ad_value(961), 0.5));s.store_mul_ad_product_rhs_mixed_ia(1005, 1011, 1020, A::limited_exp(A::sub(s.ad_value(1018), s.ad_value(1019))));s.store_div_scaled_inputs3_indices(1021, 349, 1.0, 130, (-1.0), 986, -1.0, 181, 1.0);s.store_scaled_add_mixed_ia(1022, 1021, A::sqrt(A::add_scaled_square_product(s.ad_value(1021), 1.0, s.ad_value(965), s.ad_value(965), 0.25)), 0.5);s.store_pow_ad(1023, s.ad_value(1022), A::scale(s.ad_value(962), 0.5));s.store_mul_ad_product_rhs_mixed_ia(1006, 1012, 1023, A::limited_exp(A::sub(s.ad_value(1021), s.ad_value(1022))));s.store_add_scaled_products_mixed_iiia(394, 983, 394, 1.0, 984, A::add_scaled_inputs3(s.ad_value(1004), 1.0, s.ad_value(1005), 1.0, s.ad_value(1006), 1.0), 1.0);}
        if s.b[1431] {s.store_add_scaled_product_indices(421, 396, s.v[420], 407, 394, s.v[420]);s.store_pow_ad(170, A::scaled_offset(A::div(s.ad_value(394), s.ad_value(406)), 1.0, 0.5), s.ad_value(317));s.store_pow_indices(171, 421, 822);}
        s.b[1436] = (p[61] != 0.0);s.store_scalar(1436, if s.b[1436] { 1.0 } else { 0.0 });
        if (s.b[1431] && s.b[1436]) {s.store_add_scaled_product_mixed_aai(171, A::div(s.ad_value(319), s.ad_value(170)), 1.0, A::add_scaled_product(s.ad_value(304), 1.0, s.ad_value(315), s.ad_value(370), 1.0), 171, 1.0);}
        if (s.b[1431] && (!s.b[1436])) {s.store_add_scaled_product_mixed_aii(171, A::div(s.ad_value(319), s.ad_value(170)), 1.0, 304, 171, 1.0);}
        if s.b[1431] {s.store_offset(398, 171, 1.0);s.store_scaled_add_offset_sqrt_square_offset(398, 398, 1.0, (-1.0), ((0.25 * p[604]) * p[604]), 0.5);s.store_scale(398, 398, 1.0 / (p[24]));}
        s.b[1437] = (p[64] == 1.0);s.store_scalar(1437, if s.b[1437] { 1.0 } else { 0.0 });
        if (s.b[1431] && s.b[1437]) {s.store_scalar(199, 0.0);}
        s.b[1438] = (p[64] == 0.0);s.store_scalar(1438, if s.b[1438] { 1.0 } else { 0.0 });
        if ((s.b[1431] && (!s.b[1437])) && s.b[1438]) {s.store_offset_mul(172, 711, 394, 1.0);s.store_div_from_scalar(169, 1.0, 172);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_88(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((s.b[1431] && (!s.b[1437])) && s.b[1438]) {s.store_scaled_add_mixed_ia(168, 169, A::sqrt_square_offset(s.ad_value(169), 0.01), 0.5);s.store_mul_ad_affine_product_lhs(199, A::offset(A::mul(s.ad_value(709), s.ad_value(168)), p[908]), s.ad_value(189), s.v[115], 0.0, 194);}
        if ((s.b[1431] && (!s.b[1437])) && (!s.b[1438])) {s.store_offset_mul(172, 711, 394, 1.0);s.store_div_from_scalar(169, 1.0, 172);s.store_scaled_add_mixed_ia(168, 169, A::sqrt_square_offset(s.ad_value(169), 0.01), 0.5);s.store_mul_mixed_ai(199, A::add_scaled_inputs_product(s.ad_value(190), 1.0, s.ad_value(191), 1.0, A::offset(A::mul(s.ad_value(709), s.ad_value(168)), p[908]), s.ad_value(189), s.v[115]), 194);}
        if s.b[1431] {s.store_mul_div_scaled_inputs_indices(222, 398, 336, 2.0, 414, 1.0);s.store_mul(223, 222, 153);}
        s.b[1439] = (p[80] == 0.0);s.store_scalar(1439, if s.b[1439] { 1.0 } else { 0.0 });
        if (s.b[1431] && s.b[1439]) {s.store_mul_add_scaled_inputs_rhs_indices(175, 659, 394, 1.0, 179, 2.0);}
        if (s.b[1431] && (!s.b[1439])) {s.store_mul_add_scaled_inputs_rhs_indices(175, 659, 394, 1.0, 182, 2.0);}
        s.b[1440] = (s.v[199] > 0.0);s.store_scalar(1440, if s.b[1440] { 1.0 } else { 0.0 });
        if (s.b[1431] && s.b[1440]) {s.store_mul_product3_indices(168, 199, 158, 336, 163, 1.0);s.store_scale(225, 168, 2.0);s.store_add_scaled_inputs_product_indices(226, 175, 1.0, 223, 1.0, 175, 168, 3.0);s.store_mul_add_scaled_product_rhs_indices(227, 175, 223, 1.0, 175, 168, 2.0);s.store_div_scaled_inputs2_by_product_mixed_aaai(211, A::square(s.ad_value(226)), 1.0, A::add_scaled_square_product(s.ad_value(226), 1.0, s.ad_value(225), s.ad_value(227), (-2.0)), (-1.0), A::add(s.ad_value(226), A::sqrt(A::add_scaled_square_product(s.ad_value(226), 1.0, s.ad_value(225), s.ad_value(227), (-2.0)))), 225, 1.0);}
        if (s.b[1431] && (!s.b[1440])) {s.store_div_scaled_product_add_scaled_denominator_indices(211, 223, 175, 1.0, 223, 1.0, 175, 1.0, 1.0);}
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
        if s.b[1431] {s.store_pow_ad(176, A::offset(A::div(s.ad_value(126), s.ad_value(211)), 1e-6), s.ad_value(423));s.store_pow_ad(177, A::offset(s.ad_value(176), 1.0), s.ad_value(212));s.store_min_ad(391, A::div(s.ad_value(126), s.ad_value(177)), s.ad_value(126));s.store_add(130, 391, 375);s.store_powf_ad(170, A::neg(s.ad_value(897)), 0.666666667);}
        s.b[1441] = (p[61] != 0.0);s.store_scalar(1441, if s.b[1441] { 1.0 } else { 0.0 });
    }
}
