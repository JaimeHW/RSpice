#![allow(dead_code, non_snake_case, unused_assignments, unused_imports, unused_parens, unused_variables)]
use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, ReactiveScratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};
impl Instance {
    #[inline(never)]
    pub(super) fn stamp_reactive_block_109(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((s.b[1508] && s.b[1518]) && s.b[1519]) {s.store_mul_product3_mixed_aiii(170, A::limited_exp(s.ad_value(169)), 766, 158, 141, 1.0);s.store_mul3_affine_lhs(172, 521, 521, -1.0, 0.0, 521);s.store_offset_add_ad(173, s.ad_value(763), A::abs(s.ad_value(172)), 1e-5);}
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
        if ((s.b[1508] && s.b[1518]) && s.b[1519]) {s.store_add_scaled_product_indices(175, 175, 1.0, 170, 174, 1.0);}
        if ((s.b[1508] && s.b[1518]) && (!s.b[1519])) {
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
        if ((s.b[1508] && s.b[1518]) && (!s.b[1519])) {s.store_div_scaled_inputs3_mixed_aiii(169, A::add_scaled_product(A::mul3(s.ad_value(767), s.ad_value(134), s.ad_value(134)), 1.0, s.ad_value(253), s.ad_value(134), (-1.0)), 1.0, 769, (-1.0), 479, 1.0, 179, 1.0);s.store_mul_product3_mixed_aiii(170, A::limited_exp(s.ad_value(169)), 766, 158, 141, 1.0);s.store_add_scaled_product_indices(175, 175, 1.0, 170, 135, -1.0);}
        s.b[1520] = (((p.p61 != 0.0) && ((p.p70 == 2.0) || (p.p70 == 3.0))) && (((p.p62 == 2.0) || (p.p62 == 3.0)) || (p.p62 == 5.0)));s.store_scalar(1520, if s.b[1520] { 1.0 } else { 0.0 });
        if (s.b[1508] && s.b[1520]) {
            s.store_mul_mixed_ia(251, 771, {
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
        s.b[1521] = ((s.v[770] <= 0.0) || (s.v[251] <= 0.0));s.store_scalar(1521, if s.b[1521] { 1.0 } else { 0.0 });
        if ((s.b[1508] && s.b[1520]) && s.b[1521]) {s.store_scalar(176, 0.0);}
        if ((s.b[1508] && s.b[1520]) && (!s.b[1521])) {s.store_div_scaled_inputs3_indices(169, 134, -1.0, 773, (-1.0), 479, 1.0, 168, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_110(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((s.b[1508] && s.b[1520]) && (!s.b[1521])) {
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
        if ((s.b[1508] && s.b[1520]) && (!s.b[1521])) {s.store_div_scaled_value_offset_denominator(170, s.ad_value(251), 1.0, s.ad_value(169), 0.001, 1.0);s.store_pow_indices(171, 169, 774);s.store_mul3_affine_lhs(172, 521, 521, -1.0, 0.0, 521);s.store_offset_add_ad(173, s.ad_value(772), A::abs(s.ad_value(172)), 1e-5);}
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
        if ((s.b[1508] && s.b[1520]) && (!s.b[1521])) {s.store_mul_ad_product_lhs(176, A::mul3(s.ad_value(770), s.ad_value(896), s.ad_value(171)), A::limited_exp_scaled_input(s.ad_value(170), -1.0), 174);}
        s.b[1523] = (p.p61 != 0.0);s.store_scalar(1523, if s.b[1523] { 1.0 } else { 0.0 });s.b[1524] = (s.v[537] > 0.0);s.store_scalar(1524, if s.b[1524] { 1.0 } else { 0.0 });s.b[1525] = (s.v[521] < s.v[543]);s.store_scalar(1525, if s.b[1525] { 1.0 } else { 0.0 });
        if ((s.b[1523] && s.b[1524]) && s.b[1525]) {s.store_div(168, 521, 539);s.store_offset_limited_exp(169, 168, (-1.0));s.store_add_scaled_product_right_sub(170, 542, 1.0, 541, 521, 543, 1.0);}
        s.b[1526] = (s.v[521] <= s.v[546]);s.store_scalar(1526, if s.b[1526] { 1.0 } else { 0.0 });
        if (((s.b[1523] && s.b[1524]) && (!s.b[1525])) && s.b[1526]) {s.store_div(168, 521, 539);s.store_div_scaled_offset_numerator_indices(169, 521, 1.0, p.p1626, 539, 1.0);s.store_limited_exp_neg_input(170, 169);}
        s.b[1527] = (s.v[281] > 0.0);s.store_scalar(1527, if s.b[1527] { 1.0 } else { 0.0 });s.b[1528] = ((p.p1643 - s.v[521]) < (p.p1643 * 0.001));s.store_scalar(1528, if s.b[1528] { 1.0 } else { 0.0 });
        if ((s.b[1523] && s.b[1527]) && s.b[1528]) {s.store_div_scaled_value_by_product_indices(168, 521, -1.0, 180, 287, 1.0);s.store_offset_ad(169, A::limited_exp_scaled_input(s.ad_value(168), 1000.0), (-1.0));}
        if ((s.b[1523] && s.b[1527]) && (!s.b[1528])) {s.store_div_scaled_value_by_product_indices(168, 521, -1.0, 180, 287, 1.0);s.store_offset_ad(169, A::limited_exp_div_scaled_inputs(s.ad_value(168), p.p1643, A::sub_from_scalar(p.p1643, s.ad_value(521)), 1.0), (-1.0));}
        s.b[1529] = (s.v[283] > 0.0);s.store_scalar(1529, if s.b[1529] { 1.0 } else { 0.0 });s.b[1530] = ((p.p1645 - s.v[521]) < (p.p1645 * 0.001));s.store_scalar(1530, if s.b[1530] { 1.0 } else { 0.0 });
        if ((s.b[1523] && s.b[1529]) && s.b[1530]) {s.store_div_scaled_value_by_product_indices(168, 521, -1.0, 180, 289, 1.0);s.store_offset_ad(169, A::limited_exp_scaled_input(s.ad_value(168), 1000.0), (-1.0));}
        if ((s.b[1523] && s.b[1529]) && (!s.b[1530])) {s.store_div_scaled_value_by_product_indices(168, 521, -1.0, 180, 289, 1.0);s.store_offset_ad(169, A::limited_exp_div_scaled_inputs(s.ad_value(168), p.p1645, A::sub_from_scalar(p.p1645, s.ad_value(521)), 1.0), (-1.0));}
        s.b[1531] = (s.v[285] > 0.0);s.store_scalar(1531, if s.b[1531] { 1.0 } else { 0.0 });s.b[1532] = ((p.p1647 - s.v[521]) < (p.p1647 * 0.001));s.store_scalar(1532, if s.b[1532] { 1.0 } else { 0.0 });
        if ((s.b[1523] && s.b[1531]) && s.b[1532]) {s.store_div_scaled_value_by_product_indices(168, 521, -1.0, 180, 291, 1.0);s.store_offset_ad(169, A::limited_exp_scaled_input(s.ad_value(168), 1000.0), (-1.0));}
        if ((s.b[1523] && s.b[1531]) && (!s.b[1532])) {s.store_div_scaled_value_by_product_indices(168, 521, -1.0, 180, 291, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_111(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((s.b[1523] && s.b[1531]) && (!s.b[1532])) {s.store_offset_ad(169, A::limited_exp_div_scaled_inputs(s.ad_value(168), p.p1647, A::sub_from_scalar(p.p1647, s.ad_value(521)), 1.0), (-1.0));}
        s.b[1533] = (s.v[538] > 0.0);s.store_scalar(1533, if s.b[1533] { 1.0 } else { 0.0 });s.b[1534] = (s.v[522] < s.v[550]);s.store_scalar(1534, if s.b[1534] { 1.0 } else { 0.0 });
        if ((s.b[1523] && s.b[1533]) && s.b[1534]) {s.store_div(168, 522, 540);s.store_offset_limited_exp(169, 168, (-1.0));s.store_add_scaled_product_right_sub(170, 549, 1.0, 548, 522, 550, 1.0);}
        s.b[1535] = (s.v[522] <= s.v[553]);s.store_scalar(1535, if s.b[1535] { 1.0 } else { 0.0 });
        if (((s.b[1523] && s.b[1533]) && (!s.b[1534])) && s.b[1535]) {s.store_div(168, 522, 540);s.store_div_scaled_offset_numerator_indices(169, 522, 1.0, p.p1627, 540, 1.0);s.store_limited_exp_neg_input(170, 169);}
        s.b[1536] = (s.v[282] > 0.0);s.store_scalar(1536, if s.b[1536] { 1.0 } else { 0.0 });s.b[1537] = ((p.p1644 - s.v[522]) < (p.p1644 * 0.001));s.store_scalar(1537, if s.b[1537] { 1.0 } else { 0.0 });
        if ((s.b[1523] && s.b[1536]) && s.b[1537]) {s.store_div_scaled_value_by_product_indices(168, 522, -1.0, 180, 288, 1.0);s.store_offset_ad(169, A::limited_exp_scaled_input(s.ad_value(168), 1000.0), (-1.0));}
        if ((s.b[1523] && s.b[1536]) && (!s.b[1537])) {s.store_div_scaled_value_by_product_indices(168, 522, -1.0, 180, 288, 1.0);s.store_offset_ad(169, A::limited_exp_div_scaled_inputs(s.ad_value(168), p.p1644, A::sub_from_scalar(p.p1644, s.ad_value(522)), 1.0), (-1.0));}
        s.b[1538] = (s.v[284] > 0.0);s.store_scalar(1538, if s.b[1538] { 1.0 } else { 0.0 });s.b[1539] = ((p.p1646 - s.v[522]) < (p.p1646 * 0.001));s.store_scalar(1539, if s.b[1539] { 1.0 } else { 0.0 });
        if ((s.b[1523] && s.b[1538]) && s.b[1539]) {s.store_div_scaled_value_by_product_indices(168, 522, -1.0, 180, 290, 1.0);s.store_offset_ad(169, A::limited_exp_scaled_input(s.ad_value(168), 1000.0), (-1.0));}
        if ((s.b[1523] && s.b[1538]) && (!s.b[1539])) {s.store_div_scaled_value_by_product_indices(168, 522, -1.0, 180, 290, 1.0);s.store_offset_ad(169, A::limited_exp_div_scaled_inputs(s.ad_value(168), p.p1646, A::sub_from_scalar(p.p1646, s.ad_value(522)), 1.0), (-1.0));}
        s.b[1540] = (s.v[286] > 0.0);s.store_scalar(1540, if s.b[1540] { 1.0 } else { 0.0 });s.b[1541] = ((p.p1648 - s.v[522]) < (p.p1648 * 0.001));s.store_scalar(1541, if s.b[1541] { 1.0 } else { 0.0 });
        if ((s.b[1523] && s.b[1540]) && s.b[1541]) {s.store_div_scaled_value_by_product_indices(168, 522, -1.0, 180, 292, 1.0);s.store_offset_ad(169, A::limited_exp_scaled_input(s.ad_value(168), 1000.0), (-1.0));}
        if ((s.b[1523] && s.b[1540]) && (!s.b[1541])) {s.store_div_scaled_value_by_product_indices(168, 522, -1.0, 180, 292, 1.0);s.store_offset_ad(169, A::limited_exp_div_scaled_inputs(s.ad_value(168), p.p1648, A::sub_from_scalar(p.p1648, s.ad_value(522)), 1.0), (-1.0));}
        s.b[1550] = (s.v[523] > 0.0);s.store_scalar(1550, if s.b[1550] { 1.0 } else { 0.0 });
        if (s.b[1523] && s.b[1550]) {s.store_div(1542, 521, 269);}
        s.b[1551] = (s.v[1542] < 0.9);s.store_scalar(1551, if s.b[1551] { 1.0 } else { 0.0 });s.b[1552] = (p.p1602 > 0.0);s.store_scalar(1552, if s.b[1552] { 1.0 } else { 0.0 });s.b[1553] = (s.v[521] > s.v[557]);s.store_scalar(1553, if s.b[1553] { 1.0 } else { 0.0 });
        if ((((s.b[1523] && s.b[1550]) && s.b[1551]) && s.b[1552]) && s.b[1553]) {s.store_sub_from_scalar(1547, 1.0, 1542);}
        s.b[1554] = (p.p1596 != 1.0);s.store_scalar(1554, if s.b[1554] { 1.0 } else { 0.0 });s.b[1555] = (p.p1596 == 0.5);s.store_scalar(1555, if s.b[1555] { 1.0 } else { 0.0 });
        if ((((((s.b[1523] && s.b[1550]) && s.b[1551]) && s.b[1552]) && s.b[1553]) && s.b[1554]) && s.b[1555]) {s.store_div_from_scalar_sqrt_ad(1548, 1.0, s.ad_value(1547));}
        if ((((((s.b[1523] && s.b[1550]) && s.b[1551]) && s.b[1552]) && s.b[1553]) && s.b[1554]) && (!s.b[1555])) {s.store_powf(1548, 1547, (-p.p1596));}
        if (((((s.b[1523] && s.b[1550]) && s.b[1551]) && s.b[1552]) && s.b[1553]) && s.b[1554]) {s.store_mul_ad_affine_product_rhs(530, 269, s.ad_value(523), A::sub_from_scalar(1.0, A::mul(s.ad_value(1547), s.ad_value(1548))), 1.0 / ((1.0 - p.p1596)), 0.0);}
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
        if ((((s.b[1523] && s.b[1550]) && s.b[1551]) && s.b[1552]) && (!s.b[1553])) {s.store_sub_from_scalar_div_indices(1547, 1.0, 557, 269);}
        s.b[1556] = (p.p1596 != 1.0);s.store_scalar(1556, if s.b[1556] { 1.0 } else { 0.0 });s.b[1557] = (p.p1596 == 0.5);s.store_scalar(1557, if s.b[1557] { 1.0 } else { 0.0 });
        if ((((((s.b[1523] && s.b[1550]) && s.b[1551]) && s.b[1552]) && (!s.b[1553])) && s.b[1556]) && s.b[1557]) {s.store_div_from_scalar_sqrt_ad(1548, 1.0, s.ad_value(1547));}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_112(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((((((s.b[1523] && s.b[1550]) && s.b[1551]) && s.b[1552]) && (!s.b[1553])) && s.b[1556]) && (!s.b[1557])) {s.store_powf(1548, 1547, (-p.p1596));}
        if (((((s.b[1523] && s.b[1550]) && s.b[1551]) && s.b[1552]) && (!s.b[1553])) && s.b[1556]) {s.store_mul_ad_affine_product_rhs(1549, 269, s.ad_value(523), A::sub_from_scalar(1.0, A::mul(s.ad_value(1547), s.ad_value(1548))), 1.0 / ((1.0 - p.p1596)), 0.0);}
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
        if ((((s.b[1523] && s.b[1550]) && s.b[1551]) && s.b[1552]) && (!s.b[1553])) {s.store_sub_from_scalar_ad(1547, 1.0, A::div_scaled_inputs2(s.ad_value(521), 1.0, s.ad_value(557), (-1.0), s.ad_value(558), 1.0));}
        s.b[1558] = (p.p1608 != 1.0);s.store_scalar(1558, if s.b[1558] { 1.0 } else { 0.0 });s.b[1559] = (p.p1608 == 0.5);s.store_scalar(1559, if s.b[1559] { 1.0 } else { 0.0 });
        if ((((((s.b[1523] && s.b[1550]) && s.b[1551]) && s.b[1552]) && (!s.b[1553])) && s.b[1558]) && s.b[1559]) {s.store_div_from_scalar_sqrt_ad(1548, 1.0, s.ad_value(1547));}
        if ((((((s.b[1523] && s.b[1550]) && s.b[1551]) && s.b[1552]) && (!s.b[1553])) && s.b[1558]) && (!s.b[1559])) {s.store_powf(1548, 1547, (-p.p1608));}
        if (((((s.b[1523] && s.b[1550]) && s.b[1551]) && s.b[1552]) && (!s.b[1553])) && s.b[1558]) {s.store_add_product3_rhs_mixed_iia(530, 1549, 558, 523, A::sub_from_scalar(1.0, A::mul(s.ad_value(1547), s.ad_value(1548))), (p.p1602 * 1.0 / ((1.0 - p.p1608))));}
        if (((((s.b[1523] && s.b[1550]) && s.b[1551]) && s.b[1552]) && (!s.b[1553])) && (!s.b[1558])) {
            s.store_sub_mixed_ia(530, 1549, A::mul3_scaled_output(s.ad_value(558), s.ad_value(523), {
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
        if (((s.b[1523] && s.b[1550]) && s.b[1551]) && (!s.b[1552])) {s.store_sub_from_scalar(1547, 1.0, 1542);}
        s.b[1560] = (p.p1596 != 1.0);s.store_scalar(1560, if s.b[1560] { 1.0 } else { 0.0 });s.b[1561] = (p.p1596 == 0.5);s.store_scalar(1561, if s.b[1561] { 1.0 } else { 0.0 });
        if (((((s.b[1523] && s.b[1550]) && s.b[1551]) && (!s.b[1552])) && s.b[1560]) && s.b[1561]) {s.store_div_from_scalar_sqrt_ad(1548, 1.0, s.ad_value(1547));}
        if (((((s.b[1523] && s.b[1550]) && s.b[1551]) && (!s.b[1552])) && s.b[1560]) && (!s.b[1561])) {s.store_powf(1548, 1547, (-p.p1596));}
        if ((((s.b[1523] && s.b[1550]) && s.b[1551]) && (!s.b[1552])) && s.b[1560]) {s.store_mul_ad_affine_product_rhs(530, 269, s.ad_value(523), A::sub_from_scalar(1.0, A::mul(s.ad_value(1547), s.ad_value(1548))), 1.0 / ((1.0 - p.p1596)), 0.0);}
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
        s.b[1562] = (p.p1596 != 1.0);s.store_scalar(1562, if s.b[1562] { 1.0 } else { 0.0 });s.b[1563] = (p.p1596 == 0.5);s.store_scalar(1563, if s.b[1563] { 1.0 } else { 0.0 });
        if ((((s.b[1523] && s.b[1550]) && (!s.b[1551])) && s.b[1562]) && s.b[1563]) {s.store_scalar(1543, (1.0 / ((0.1) as f64).sqrt()));}
        if ((((s.b[1523] && s.b[1550]) && (!s.b[1551])) && s.b[1562]) && (!s.b[1563])) {s.store_scalar(1543, ((0.1) as f64).powf((-p.p1596)));}
        if (((s.b[1523] && s.b[1550]) && (!s.b[1551])) && s.b[1562]) {s.store_scalar(1544, (1.0 / (1.0 - p.p1596)));s.store_primal_mul_scale_offset_mixed_ia(1546, 1544, A::scale(s.ad_value(1543), ((0.05 * p.p1596) * (1.0 + p.p1596))), -1.0, 1.0);}
        if (((s.b[1523] && s.b[1550]) && (!s.b[1551])) && (!s.b[1562])) {s.store_scalar(1543, 10.0);s.store_scalar(1546, (1.5 - ((0.1) as f64).ln()));}
        if ((s.b[1523] && s.b[1550]) && (!s.b[1551])) {s.store_mul_ad_product_rhs(1545, 1543, A::offset(s.ad_value(1542), (-1.0)), A::scale_offset(s.ad_value(1542), (5.0 * p.p1596), (((((-1.0)) * ((5.0 * p.p1596)))) + ((1.0 + p.p1596)))));s.store_mul_ad_product_rhs_mixed_ia(530, 269, 523, A::add(s.ad_value(1545), s.ad_value(1546)));}
        if (s.b[1523] && (!s.b[1550])) {s.store_scalar(530, 0.0);}
        s.b[1572] = (s.v[524] > 0.0);s.store_scalar(1572, if s.b[1572] { 1.0 } else { 0.0 });
        if (s.b[1523] && s.b[1572]) {s.store_div(1564, 521, 270);}
        s.b[1573] = (s.v[1564] < 0.9);s.store_scalar(1573, if s.b[1573] { 1.0 } else { 0.0 });s.b[1574] = (p.p1604 > 0.0);s.store_scalar(1574, if s.b[1574] { 1.0 } else { 0.0 });s.b[1575] = (s.v[521] > s.v[559]);s.store_scalar(1575, if s.b[1575] { 1.0 } else { 0.0 });
        if ((((s.b[1523] && s.b[1572]) && s.b[1573]) && s.b[1574]) && s.b[1575]) {s.store_sub_from_scalar(1569, 1.0, 1564);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_113(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.b[1576] = (p.p1598 != 1.0);s.store_scalar(1576, if s.b[1576] { 1.0 } else { 0.0 });s.b[1577] = (p.p1598 == 0.5);s.store_scalar(1577, if s.b[1577] { 1.0 } else { 0.0 });
        if ((((((s.b[1523] && s.b[1572]) && s.b[1573]) && s.b[1574]) && s.b[1575]) && s.b[1576]) && s.b[1577]) {s.store_div_from_scalar_sqrt_ad(1570, 1.0, s.ad_value(1569));}
        if ((((((s.b[1523] && s.b[1572]) && s.b[1573]) && s.b[1574]) && s.b[1575]) && s.b[1576]) && (!s.b[1577])) {s.store_powf(1570, 1569, (-p.p1598));}
        if (((((s.b[1523] && s.b[1572]) && s.b[1573]) && s.b[1574]) && s.b[1575]) && s.b[1576]) {s.store_mul_ad_affine_product_rhs(531, 270, s.ad_value(524), A::sub_from_scalar(1.0, A::mul(s.ad_value(1569), s.ad_value(1570))), 1.0 / ((1.0 - p.p1598)), 0.0);}
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
        if ((((s.b[1523] && s.b[1572]) && s.b[1573]) && s.b[1574]) && (!s.b[1575])) {s.store_sub_from_scalar_div_indices(1569, 1.0, 559, 270);}
        s.b[1578] = (p.p1598 != 1.0);s.store_scalar(1578, if s.b[1578] { 1.0 } else { 0.0 });s.b[1579] = (p.p1598 == 0.5);s.store_scalar(1579, if s.b[1579] { 1.0 } else { 0.0 });
        if ((((((s.b[1523] && s.b[1572]) && s.b[1573]) && s.b[1574]) && (!s.b[1575])) && s.b[1578]) && s.b[1579]) {s.store_div_from_scalar_sqrt_ad(1570, 1.0, s.ad_value(1569));}
        if ((((((s.b[1523] && s.b[1572]) && s.b[1573]) && s.b[1574]) && (!s.b[1575])) && s.b[1578]) && (!s.b[1579])) {s.store_powf(1570, 1569, (-p.p1598));}
        if (((((s.b[1523] && s.b[1572]) && s.b[1573]) && s.b[1574]) && (!s.b[1575])) && s.b[1578]) {s.store_mul_ad_affine_product_rhs(1571, 270, s.ad_value(524), A::sub_from_scalar(1.0, A::mul(s.ad_value(1569), s.ad_value(1570))), 1.0 / ((1.0 - p.p1598)), 0.0);}
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
        if ((((s.b[1523] && s.b[1572]) && s.b[1573]) && s.b[1574]) && (!s.b[1575])) {s.store_sub_from_scalar_ad(1569, 1.0, A::div_scaled_inputs2(s.ad_value(521), 1.0, s.ad_value(559), (-1.0), s.ad_value(560), 1.0));}
        s.b[1580] = (p.p1610 != 1.0);s.store_scalar(1580, if s.b[1580] { 1.0 } else { 0.0 });s.b[1581] = (p.p1610 == 0.5);s.store_scalar(1581, if s.b[1581] { 1.0 } else { 0.0 });
        if ((((((s.b[1523] && s.b[1572]) && s.b[1573]) && s.b[1574]) && (!s.b[1575])) && s.b[1580]) && s.b[1581]) {s.store_div_from_scalar_sqrt_ad(1570, 1.0, s.ad_value(1569));}
        if ((((((s.b[1523] && s.b[1572]) && s.b[1573]) && s.b[1574]) && (!s.b[1575])) && s.b[1580]) && (!s.b[1581])) {s.store_powf(1570, 1569, (-p.p1610));}
        if (((((s.b[1523] && s.b[1572]) && s.b[1573]) && s.b[1574]) && (!s.b[1575])) && s.b[1580]) {s.store_add_product3_rhs_mixed_iia(531, 1571, 560, 524, A::sub_from_scalar(1.0, A::mul(s.ad_value(1569), s.ad_value(1570))), (p.p1604 * 1.0 / ((1.0 - p.p1610))));}
        if (((((s.b[1523] && s.b[1572]) && s.b[1573]) && s.b[1574]) && (!s.b[1575])) && (!s.b[1580])) {
            s.store_sub_mixed_ia(531, 1571, A::mul3_scaled_output(s.ad_value(560), s.ad_value(524), {
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
        if (((s.b[1523] && s.b[1572]) && s.b[1573]) && (!s.b[1574])) {s.store_sub_from_scalar(1569, 1.0, 1564);}
        s.b[1582] = (p.p1598 != 1.0);s.store_scalar(1582, if s.b[1582] { 1.0 } else { 0.0 });s.b[1583] = (p.p1598 == 0.5);s.store_scalar(1583, if s.b[1583] { 1.0 } else { 0.0 });
        if (((((s.b[1523] && s.b[1572]) && s.b[1573]) && (!s.b[1574])) && s.b[1582]) && s.b[1583]) {s.store_div_from_scalar_sqrt_ad(1570, 1.0, s.ad_value(1569));}
        if (((((s.b[1523] && s.b[1572]) && s.b[1573]) && (!s.b[1574])) && s.b[1582]) && (!s.b[1583])) {s.store_powf(1570, 1569, (-p.p1598));}
        if ((((s.b[1523] && s.b[1572]) && s.b[1573]) && (!s.b[1574])) && s.b[1582]) {s.store_mul_ad_affine_product_rhs(531, 270, s.ad_value(524), A::sub_from_scalar(1.0, A::mul(s.ad_value(1569), s.ad_value(1570))), 1.0 / ((1.0 - p.p1598)), 0.0);}
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
        s.b[1584] = (p.p1598 != 1.0);s.store_scalar(1584, if s.b[1584] { 1.0 } else { 0.0 });s.b[1585] = (p.p1598 == 0.5);s.store_scalar(1585, if s.b[1585] { 1.0 } else { 0.0 });
        if ((((s.b[1523] && s.b[1572]) && (!s.b[1573])) && s.b[1584]) && s.b[1585]) {s.store_scalar(1565, (1.0 / ((0.1) as f64).sqrt()));}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_114(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((((s.b[1523] && s.b[1572]) && (!s.b[1573])) && s.b[1584]) && (!s.b[1585])) {s.store_scalar(1565, ((0.1) as f64).powf((-p.p1598)));}
        if (((s.b[1523] && s.b[1572]) && (!s.b[1573])) && s.b[1584]) {s.store_scalar(1566, (1.0 / (1.0 - p.p1598)));s.store_primal_mul_scale_offset_mixed_ia(1568, 1566, A::scale(s.ad_value(1565), ((0.05 * p.p1598) * (1.0 + p.p1598))), -1.0, 1.0);}
        if (((s.b[1523] && s.b[1572]) && (!s.b[1573])) && (!s.b[1584])) {s.store_scalar(1565, 10.0);s.store_scalar(1568, (1.5 - ((0.1) as f64).ln()));}
        if ((s.b[1523] && s.b[1572]) && (!s.b[1573])) {s.store_mul_ad_product_rhs(1567, 1565, A::offset(s.ad_value(1564), (-1.0)), A::scale_offset(s.ad_value(1564), (5.0 * p.p1598), (((((-1.0)) * ((5.0 * p.p1598)))) + ((1.0 + p.p1598)))));s.store_mul_ad_product_rhs_mixed_ia(531, 270, 524, A::add(s.ad_value(1567), s.ad_value(1568)));}
        if (s.b[1523] && (!s.b[1572])) {s.store_scalar(531, 0.0);}
        s.b[1594] = (s.v[525] > 0.0);s.store_scalar(1594, if s.b[1594] { 1.0 } else { 0.0 });
        if (s.b[1523] && s.b[1594]) {s.store_div(1586, 521, 271);}
        s.b[1595] = (s.v[1586] < 0.9);s.store_scalar(1595, if s.b[1595] { 1.0 } else { 0.0 });s.b[1596] = (p.p1606 > 0.0);s.store_scalar(1596, if s.b[1596] { 1.0 } else { 0.0 });s.b[1597] = (s.v[521] > s.v[561]);s.store_scalar(1597, if s.b[1597] { 1.0 } else { 0.0 });
        if ((((s.b[1523] && s.b[1594]) && s.b[1595]) && s.b[1596]) && s.b[1597]) {s.store_sub_from_scalar(1591, 1.0, 1586);}
        s.b[1598] = (p.p1600 != 1.0);s.store_scalar(1598, if s.b[1598] { 1.0 } else { 0.0 });s.b[1599] = (p.p1600 == 0.5);s.store_scalar(1599, if s.b[1599] { 1.0 } else { 0.0 });
        if ((((((s.b[1523] && s.b[1594]) && s.b[1595]) && s.b[1596]) && s.b[1597]) && s.b[1598]) && s.b[1599]) {s.store_div_from_scalar_sqrt_ad(1592, 1.0, s.ad_value(1591));}
        if ((((((s.b[1523] && s.b[1594]) && s.b[1595]) && s.b[1596]) && s.b[1597]) && s.b[1598]) && (!s.b[1599])) {s.store_powf(1592, 1591, (-p.p1600));}
        if (((((s.b[1523] && s.b[1594]) && s.b[1595]) && s.b[1596]) && s.b[1597]) && s.b[1598]) {s.store_mul_ad_affine_product_rhs(532, 271, s.ad_value(525), A::sub_from_scalar(1.0, A::mul(s.ad_value(1591), s.ad_value(1592))), 1.0 / ((1.0 - p.p1600)), 0.0);}
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
        if ((((s.b[1523] && s.b[1594]) && s.b[1595]) && s.b[1596]) && (!s.b[1597])) {s.store_sub_from_scalar_div_indices(1591, 1.0, 561, 271);}
        s.b[1600] = (p.p1600 != 1.0);s.store_scalar(1600, if s.b[1600] { 1.0 } else { 0.0 });s.b[1601] = (p.p1600 == 0.5);s.store_scalar(1601, if s.b[1601] { 1.0 } else { 0.0 });
        if ((((((s.b[1523] && s.b[1594]) && s.b[1595]) && s.b[1596]) && (!s.b[1597])) && s.b[1600]) && s.b[1601]) {s.store_div_from_scalar_sqrt_ad(1592, 1.0, s.ad_value(1591));}
        if ((((((s.b[1523] && s.b[1594]) && s.b[1595]) && s.b[1596]) && (!s.b[1597])) && s.b[1600]) && (!s.b[1601])) {s.store_powf(1592, 1591, (-p.p1600));}
        if (((((s.b[1523] && s.b[1594]) && s.b[1595]) && s.b[1596]) && (!s.b[1597])) && s.b[1600]) {s.store_mul_ad_affine_product_rhs(1593, 271, s.ad_value(525), A::sub_from_scalar(1.0, A::mul(s.ad_value(1591), s.ad_value(1592))), 1.0 / ((1.0 - p.p1600)), 0.0);}
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
        if ((((s.b[1523] && s.b[1594]) && s.b[1595]) && s.b[1596]) && (!s.b[1597])) {s.store_sub_from_scalar_ad(1591, 1.0, A::div_scaled_inputs2(s.ad_value(521), 1.0, s.ad_value(561), (-1.0), s.ad_value(562), 1.0));}
        s.b[1602] = (p.p1612 != 1.0);s.store_scalar(1602, if s.b[1602] { 1.0 } else { 0.0 });s.b[1603] = (p.p1612 == 0.5);s.store_scalar(1603, if s.b[1603] { 1.0 } else { 0.0 });
        if ((((((s.b[1523] && s.b[1594]) && s.b[1595]) && s.b[1596]) && (!s.b[1597])) && s.b[1602]) && s.b[1603]) {s.store_div_from_scalar_sqrt_ad(1592, 1.0, s.ad_value(1591));}
        if ((((((s.b[1523] && s.b[1594]) && s.b[1595]) && s.b[1596]) && (!s.b[1597])) && s.b[1602]) && (!s.b[1603])) {s.store_powf(1592, 1591, (-p.p1612));}
        if (((((s.b[1523] && s.b[1594]) && s.b[1595]) && s.b[1596]) && (!s.b[1597])) && s.b[1602]) {s.store_add_product3_rhs_mixed_iia(532, 1593, 562, 525, A::sub_from_scalar(1.0, A::mul(s.ad_value(1591), s.ad_value(1592))), (p.p1606 * 1.0 / ((1.0 - p.p1612))));}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_115(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (((((s.b[1523] && s.b[1594]) && s.b[1595]) && s.b[1596]) && (!s.b[1597])) && (!s.b[1602])) {
            s.store_sub_mixed_ia(532, 1593, A::mul3_scaled_output(s.ad_value(562), s.ad_value(525), {
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
        if (((s.b[1523] && s.b[1594]) && s.b[1595]) && (!s.b[1596])) {s.store_sub_from_scalar(1591, 1.0, 1586);}
        s.b[1604] = (p.p1600 != 1.0);s.store_scalar(1604, if s.b[1604] { 1.0 } else { 0.0 });s.b[1605] = (p.p1600 == 0.5);s.store_scalar(1605, if s.b[1605] { 1.0 } else { 0.0 });
        if (((((s.b[1523] && s.b[1594]) && s.b[1595]) && (!s.b[1596])) && s.b[1604]) && s.b[1605]) {s.store_div_from_scalar_sqrt_ad(1592, 1.0, s.ad_value(1591));}
        if (((((s.b[1523] && s.b[1594]) && s.b[1595]) && (!s.b[1596])) && s.b[1604]) && (!s.b[1605])) {s.store_powf(1592, 1591, (-p.p1600));}
        if ((((s.b[1523] && s.b[1594]) && s.b[1595]) && (!s.b[1596])) && s.b[1604]) {s.store_mul_ad_affine_product_rhs(532, 271, s.ad_value(525), A::sub_from_scalar(1.0, A::mul(s.ad_value(1591), s.ad_value(1592))), 1.0 / ((1.0 - p.p1600)), 0.0);}
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
        s.b[1606] = (p.p1600 != 1.0);s.store_scalar(1606, if s.b[1606] { 1.0 } else { 0.0 });s.b[1607] = (p.p1600 == 0.5);s.store_scalar(1607, if s.b[1607] { 1.0 } else { 0.0 });
        if ((((s.b[1523] && s.b[1594]) && (!s.b[1595])) && s.b[1606]) && s.b[1607]) {s.store_scalar(1587, (1.0 / ((0.1) as f64).sqrt()));}
        if ((((s.b[1523] && s.b[1594]) && (!s.b[1595])) && s.b[1606]) && (!s.b[1607])) {s.store_scalar(1587, ((0.1) as f64).powf((-p.p1600)));}
        if (((s.b[1523] && s.b[1594]) && (!s.b[1595])) && s.b[1606]) {s.store_scalar(1588, (1.0 / (1.0 - p.p1600)));s.store_primal_mul_scale_offset_mixed_ia(1590, 1588, A::scale(s.ad_value(1587), ((0.05 * p.p1600) * (1.0 + p.p1600))), -1.0, 1.0);}
        if (((s.b[1523] && s.b[1594]) && (!s.b[1595])) && (!s.b[1606])) {s.store_scalar(1587, 10.0);s.store_scalar(1590, (1.5 - ((0.1) as f64).ln()));}
        if ((s.b[1523] && s.b[1594]) && (!s.b[1595])) {s.store_mul_ad_product_rhs(1589, 1587, A::offset(s.ad_value(1586), (-1.0)), A::scale_offset(s.ad_value(1586), (5.0 * p.p1600), (((((-1.0)) * ((5.0 * p.p1600)))) + ((1.0 + p.p1600)))));s.store_mul_ad_product_rhs_mixed_ia(532, 271, 525, A::add(s.ad_value(1589), s.ad_value(1590)));}
        if (s.b[1523] && (!s.b[1594])) {s.store_scalar(532, 0.0);}
        if s.b[1523] {s.store_add_scaled_inputs3_indices(529, 530, 1.0, 531, 1.0, 532, 1.0);}
        s.b[1616] = (s.v[526] > 0.0);s.store_scalar(1616, if s.b[1616] { 1.0 } else { 0.0 });
        if (s.b[1523] && s.b[1616]) {s.store_div(1608, 522, 272);}
        s.b[1617] = (s.v[1608] < 0.9);s.store_scalar(1617, if s.b[1617] { 1.0 } else { 0.0 });s.b[1618] = (p.p1603 > 0.0);s.store_scalar(1618, if s.b[1618] { 1.0 } else { 0.0 });s.b[1619] = (s.v[522] > s.v[563]);s.store_scalar(1619, if s.b[1619] { 1.0 } else { 0.0 });
        if ((((s.b[1523] && s.b[1616]) && s.b[1617]) && s.b[1618]) && s.b[1619]) {s.store_sub_from_scalar(1613, 1.0, 1608);}
        s.b[1620] = (p.p1597 != 1.0);s.store_scalar(1620, if s.b[1620] { 1.0 } else { 0.0 });s.b[1621] = (p.p1597 == 0.5);s.store_scalar(1621, if s.b[1621] { 1.0 } else { 0.0 });
        if ((((((s.b[1523] && s.b[1616]) && s.b[1617]) && s.b[1618]) && s.b[1619]) && s.b[1620]) && s.b[1621]) {s.store_div_from_scalar_sqrt_ad(1614, 1.0, s.ad_value(1613));}
        if ((((((s.b[1523] && s.b[1616]) && s.b[1617]) && s.b[1618]) && s.b[1619]) && s.b[1620]) && (!s.b[1621])) {s.store_powf(1614, 1613, (-p.p1597));}
        if (((((s.b[1523] && s.b[1616]) && s.b[1617]) && s.b[1618]) && s.b[1619]) && s.b[1620]) {s.store_mul_ad_affine_product_rhs(534, 272, s.ad_value(526), A::sub_from_scalar(1.0, A::mul(s.ad_value(1613), s.ad_value(1614))), 1.0 / ((1.0 - p.p1597)), 0.0);}
        if (((((s.b[1523] && s.b[1616]) && s.b[1617]) && s.b[1618]) && s.b[1619]) && (!s.b[1620])) {
            s.store_mul_ad_affine_product_rhs(534, 272, s.ad_value(526), {
                if (!(s.v[1613] > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if (s.v[1613] > 1e-38) {
                            A::ln(s.ad_value(1613))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, -1.0, 0.0);
        }
        if ((((s.b[1523] && s.b[1616]) && s.b[1617]) && s.b[1618]) && (!s.b[1619])) {s.store_sub_from_scalar_div_indices(1613, 1.0, 563, 272);}
        s.b[1622] = (p.p1597 != 1.0);s.store_scalar(1622, if s.b[1622] { 1.0 } else { 0.0 });s.b[1623] = (p.p1597 == 0.5);s.store_scalar(1623, if s.b[1623] { 1.0 } else { 0.0 });
        if ((((((s.b[1523] && s.b[1616]) && s.b[1617]) && s.b[1618]) && (!s.b[1619])) && s.b[1622]) && s.b[1623]) {s.store_div_from_scalar_sqrt_ad(1614, 1.0, s.ad_value(1613));}
        if ((((((s.b[1523] && s.b[1616]) && s.b[1617]) && s.b[1618]) && (!s.b[1619])) && s.b[1622]) && (!s.b[1623])) {s.store_powf(1614, 1613, (-p.p1597));}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_116(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (((((s.b[1523] && s.b[1616]) && s.b[1617]) && s.b[1618]) && (!s.b[1619])) && s.b[1622]) {s.store_mul_ad_affine_product_rhs(1615, 272, s.ad_value(526), A::sub_from_scalar(1.0, A::mul(s.ad_value(1613), s.ad_value(1614))), 1.0 / ((1.0 - p.p1597)), 0.0);}
        if (((((s.b[1523] && s.b[1616]) && s.b[1617]) && s.b[1618]) && (!s.b[1619])) && (!s.b[1622])) {
            s.store_mul_ad_affine_product_rhs(1615, 272, s.ad_value(526), {
                if (!(s.v[1613] > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if (s.v[1613] > 1e-38) {
                            A::ln(s.ad_value(1613))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, -1.0, 0.0);
        }
        if ((((s.b[1523] && s.b[1616]) && s.b[1617]) && s.b[1618]) && (!s.b[1619])) {s.store_sub_from_scalar_ad(1613, 1.0, A::div_scaled_inputs2(s.ad_value(522), 1.0, s.ad_value(563), (-1.0), s.ad_value(564), 1.0));}
        s.b[1624] = (p.p1609 != 1.0);s.store_scalar(1624, if s.b[1624] { 1.0 } else { 0.0 });s.b[1625] = (p.p1609 == 0.5);s.store_scalar(1625, if s.b[1625] { 1.0 } else { 0.0 });
        if ((((((s.b[1523] && s.b[1616]) && s.b[1617]) && s.b[1618]) && (!s.b[1619])) && s.b[1624]) && s.b[1625]) {s.store_div_from_scalar_sqrt_ad(1614, 1.0, s.ad_value(1613));}
        if ((((((s.b[1523] && s.b[1616]) && s.b[1617]) && s.b[1618]) && (!s.b[1619])) && s.b[1624]) && (!s.b[1625])) {s.store_powf(1614, 1613, (-p.p1609));}
        if (((((s.b[1523] && s.b[1616]) && s.b[1617]) && s.b[1618]) && (!s.b[1619])) && s.b[1624]) {s.store_add_product3_rhs_mixed_iia(534, 1615, 564, 526, A::sub_from_scalar(1.0, A::mul(s.ad_value(1613), s.ad_value(1614))), (p.p1603 * 1.0 / ((1.0 - p.p1609))));}
        if (((((s.b[1523] && s.b[1616]) && s.b[1617]) && s.b[1618]) && (!s.b[1619])) && (!s.b[1624])) {
            s.store_sub_mixed_ia(534, 1615, A::mul3_scaled_output(s.ad_value(564), s.ad_value(526), {
                            if (!(s.v[1613] > 1e-38)) {
                                A::neg(A::constant(87.498233534))
                            } else {
                                {
                                    if (s.v[1613] > 1e-38) {
                                        A::ln(s.ad_value(1613))
                                    } else {
                                        A::constant(0.0)
                                    }
                                }
                            }
                        }, p.p1603));
        }
        if (((s.b[1523] && s.b[1616]) && s.b[1617]) && (!s.b[1618])) {s.store_sub_from_scalar(1613, 1.0, 1608);}
        s.b[1626] = (p.p1597 != 1.0);s.store_scalar(1626, if s.b[1626] { 1.0 } else { 0.0 });s.b[1627] = (p.p1597 == 0.5);s.store_scalar(1627, if s.b[1627] { 1.0 } else { 0.0 });
        if (((((s.b[1523] && s.b[1616]) && s.b[1617]) && (!s.b[1618])) && s.b[1626]) && s.b[1627]) {s.store_div_from_scalar_sqrt_ad(1614, 1.0, s.ad_value(1613));}
        if (((((s.b[1523] && s.b[1616]) && s.b[1617]) && (!s.b[1618])) && s.b[1626]) && (!s.b[1627])) {s.store_powf(1614, 1613, (-p.p1597));}
        if ((((s.b[1523] && s.b[1616]) && s.b[1617]) && (!s.b[1618])) && s.b[1626]) {s.store_mul_ad_affine_product_rhs(534, 272, s.ad_value(526), A::sub_from_scalar(1.0, A::mul(s.ad_value(1613), s.ad_value(1614))), 1.0 / ((1.0 - p.p1597)), 0.0);}
        if ((((s.b[1523] && s.b[1616]) && s.b[1617]) && (!s.b[1618])) && (!s.b[1626])) {
            s.store_mul_ad_affine_product_rhs(534, 272, s.ad_value(526), {
                if (!(s.v[1613] > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if (s.v[1613] > 1e-38) {
                            A::ln(s.ad_value(1613))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, -1.0, 0.0);
        }
        s.b[1628] = (p.p1597 != 1.0);s.store_scalar(1628, if s.b[1628] { 1.0 } else { 0.0 });s.b[1629] = (p.p1597 == 0.5);s.store_scalar(1629, if s.b[1629] { 1.0 } else { 0.0 });
        if ((((s.b[1523] && s.b[1616]) && (!s.b[1617])) && s.b[1628]) && s.b[1629]) {s.store_scalar(1609, (1.0 / ((0.1) as f64).sqrt()));}
        if ((((s.b[1523] && s.b[1616]) && (!s.b[1617])) && s.b[1628]) && (!s.b[1629])) {s.store_scalar(1609, ((0.1) as f64).powf((-p.p1597)));}
        if (((s.b[1523] && s.b[1616]) && (!s.b[1617])) && s.b[1628]) {s.store_scalar(1610, (1.0 / (1.0 - p.p1597)));s.store_primal_mul_scale_offset_mixed_ia(1612, 1610, A::scale(s.ad_value(1609), ((0.05 * p.p1597) * (1.0 + p.p1597))), -1.0, 1.0);}
        if (((s.b[1523] && s.b[1616]) && (!s.b[1617])) && (!s.b[1628])) {s.store_scalar(1609, 10.0);s.store_scalar(1612, (1.5 - ((0.1) as f64).ln()));}
        if ((s.b[1523] && s.b[1616]) && (!s.b[1617])) {s.store_mul_ad_product_rhs(1611, 1609, A::offset(s.ad_value(1608), (-1.0)), A::scale_offset(s.ad_value(1608), (5.0 * p.p1597), (((((-1.0)) * ((5.0 * p.p1597)))) + ((1.0 + p.p1597)))));s.store_mul_ad_product_rhs_mixed_ia(534, 272, 526, A::add(s.ad_value(1611), s.ad_value(1612)));}
        if (s.b[1523] && (!s.b[1616])) {s.store_scalar(534, 0.0);}
        s.b[1638] = (s.v[527] > 0.0);s.store_scalar(1638, if s.b[1638] { 1.0 } else { 0.0 });
        if (s.b[1523] && s.b[1638]) {s.store_div(1630, 522, 273);}
        s.b[1639] = (s.v[1630] < 0.9);s.store_scalar(1639, if s.b[1639] { 1.0 } else { 0.0 });s.b[1640] = (p.p1605 > 0.0);s.store_scalar(1640, if s.b[1640] { 1.0 } else { 0.0 });s.b[1641] = (s.v[522] > s.v[565]);s.store_scalar(1641, if s.b[1641] { 1.0 } else { 0.0 });
        if ((((s.b[1523] && s.b[1638]) && s.b[1639]) && s.b[1640]) && s.b[1641]) {s.store_sub_from_scalar(1635, 1.0, 1630);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_117(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.b[1642] = (p.p1599 != 1.0);s.store_scalar(1642, if s.b[1642] { 1.0 } else { 0.0 });s.b[1643] = (p.p1599 == 0.5);s.store_scalar(1643, if s.b[1643] { 1.0 } else { 0.0 });
        if ((((((s.b[1523] && s.b[1638]) && s.b[1639]) && s.b[1640]) && s.b[1641]) && s.b[1642]) && s.b[1643]) {s.store_div_from_scalar_sqrt_ad(1636, 1.0, s.ad_value(1635));}
        if ((((((s.b[1523] && s.b[1638]) && s.b[1639]) && s.b[1640]) && s.b[1641]) && s.b[1642]) && (!s.b[1643])) {s.store_powf(1636, 1635, (-p.p1599));}
        if (((((s.b[1523] && s.b[1638]) && s.b[1639]) && s.b[1640]) && s.b[1641]) && s.b[1642]) {s.store_mul_ad_affine_product_rhs(535, 273, s.ad_value(527), A::sub_from_scalar(1.0, A::mul(s.ad_value(1635), s.ad_value(1636))), 1.0 / ((1.0 - p.p1599)), 0.0);}
        if (((((s.b[1523] && s.b[1638]) && s.b[1639]) && s.b[1640]) && s.b[1641]) && (!s.b[1642])) {
            s.store_mul_ad_affine_product_rhs(535, 273, s.ad_value(527), {
                if (!(s.v[1635] > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if (s.v[1635] > 1e-38) {
                            A::ln(s.ad_value(1635))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, -1.0, 0.0);
        }
        if ((((s.b[1523] && s.b[1638]) && s.b[1639]) && s.b[1640]) && (!s.b[1641])) {s.store_sub_from_scalar_div_indices(1635, 1.0, 565, 273);}
        s.b[1644] = (p.p1599 != 1.0);s.store_scalar(1644, if s.b[1644] { 1.0 } else { 0.0 });s.b[1645] = (p.p1599 == 0.5);s.store_scalar(1645, if s.b[1645] { 1.0 } else { 0.0 });
        if ((((((s.b[1523] && s.b[1638]) && s.b[1639]) && s.b[1640]) && (!s.b[1641])) && s.b[1644]) && s.b[1645]) {s.store_div_from_scalar_sqrt_ad(1636, 1.0, s.ad_value(1635));}
        if ((((((s.b[1523] && s.b[1638]) && s.b[1639]) && s.b[1640]) && (!s.b[1641])) && s.b[1644]) && (!s.b[1645])) {s.store_powf(1636, 1635, (-p.p1599));}
        if (((((s.b[1523] && s.b[1638]) && s.b[1639]) && s.b[1640]) && (!s.b[1641])) && s.b[1644]) {s.store_mul_ad_affine_product_rhs(1637, 273, s.ad_value(527), A::sub_from_scalar(1.0, A::mul(s.ad_value(1635), s.ad_value(1636))), 1.0 / ((1.0 - p.p1599)), 0.0);}
        if (((((s.b[1523] && s.b[1638]) && s.b[1639]) && s.b[1640]) && (!s.b[1641])) && (!s.b[1644])) {
            s.store_mul_ad_affine_product_rhs(1637, 273, s.ad_value(527), {
                if (!(s.v[1635] > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if (s.v[1635] > 1e-38) {
                            A::ln(s.ad_value(1635))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, -1.0, 0.0);
        }
        if ((((s.b[1523] && s.b[1638]) && s.b[1639]) && s.b[1640]) && (!s.b[1641])) {s.store_sub_from_scalar_ad(1635, 1.0, A::div_scaled_inputs2(s.ad_value(522), 1.0, s.ad_value(565), (-1.0), s.ad_value(566), 1.0));}
        s.b[1646] = (p.p1611 != 1.0);s.store_scalar(1646, if s.b[1646] { 1.0 } else { 0.0 });s.b[1647] = (p.p1611 == 0.5);s.store_scalar(1647, if s.b[1647] { 1.0 } else { 0.0 });
        if ((((((s.b[1523] && s.b[1638]) && s.b[1639]) && s.b[1640]) && (!s.b[1641])) && s.b[1646]) && s.b[1647]) {s.store_div_from_scalar_sqrt_ad(1636, 1.0, s.ad_value(1635));}
        if ((((((s.b[1523] && s.b[1638]) && s.b[1639]) && s.b[1640]) && (!s.b[1641])) && s.b[1646]) && (!s.b[1647])) {s.store_powf(1636, 1635, (-p.p1611));}
        if (((((s.b[1523] && s.b[1638]) && s.b[1639]) && s.b[1640]) && (!s.b[1641])) && s.b[1646]) {s.store_add_product3_rhs_mixed_iia(535, 1637, 566, 527, A::sub_from_scalar(1.0, A::mul(s.ad_value(1635), s.ad_value(1636))), (p.p1605 * 1.0 / ((1.0 - p.p1611))));}
        if (((((s.b[1523] && s.b[1638]) && s.b[1639]) && s.b[1640]) && (!s.b[1641])) && (!s.b[1646])) {
            s.store_sub_mixed_ia(535, 1637, A::mul3_scaled_output(s.ad_value(566), s.ad_value(527), {
                            if (!(s.v[1635] > 1e-38)) {
                                A::neg(A::constant(87.498233534))
                            } else {
                                {
                                    if (s.v[1635] > 1e-38) {
                                        A::ln(s.ad_value(1635))
                                    } else {
                                        A::constant(0.0)
                                    }
                                }
                            }
                        }, p.p1605));
        }
        if (((s.b[1523] && s.b[1638]) && s.b[1639]) && (!s.b[1640])) {s.store_sub_from_scalar(1635, 1.0, 1630);}
        s.b[1648] = (p.p1599 != 1.0);s.store_scalar(1648, if s.b[1648] { 1.0 } else { 0.0 });s.b[1649] = (p.p1599 == 0.5);s.store_scalar(1649, if s.b[1649] { 1.0 } else { 0.0 });
        if (((((s.b[1523] && s.b[1638]) && s.b[1639]) && (!s.b[1640])) && s.b[1648]) && s.b[1649]) {s.store_div_from_scalar_sqrt_ad(1636, 1.0, s.ad_value(1635));}
        if (((((s.b[1523] && s.b[1638]) && s.b[1639]) && (!s.b[1640])) && s.b[1648]) && (!s.b[1649])) {s.store_powf(1636, 1635, (-p.p1599));}
        if ((((s.b[1523] && s.b[1638]) && s.b[1639]) && (!s.b[1640])) && s.b[1648]) {s.store_mul_ad_affine_product_rhs(535, 273, s.ad_value(527), A::sub_from_scalar(1.0, A::mul(s.ad_value(1635), s.ad_value(1636))), 1.0 / ((1.0 - p.p1599)), 0.0);}
        if ((((s.b[1523] && s.b[1638]) && s.b[1639]) && (!s.b[1640])) && (!s.b[1648])) {
            s.store_mul_ad_affine_product_rhs(535, 273, s.ad_value(527), {
                if (!(s.v[1635] > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if (s.v[1635] > 1e-38) {
                            A::ln(s.ad_value(1635))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, -1.0, 0.0);
        }
        s.b[1650] = (p.p1599 != 1.0);s.store_scalar(1650, if s.b[1650] { 1.0 } else { 0.0 });s.b[1651] = (p.p1599 == 0.5);s.store_scalar(1651, if s.b[1651] { 1.0 } else { 0.0 });
        if ((((s.b[1523] && s.b[1638]) && (!s.b[1639])) && s.b[1650]) && s.b[1651]) {s.store_scalar(1631, (1.0 / ((0.1) as f64).sqrt()));}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_118(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((((s.b[1523] && s.b[1638]) && (!s.b[1639])) && s.b[1650]) && (!s.b[1651])) {s.store_scalar(1631, ((0.1) as f64).powf((-p.p1599)));}
        if (((s.b[1523] && s.b[1638]) && (!s.b[1639])) && s.b[1650]) {s.store_scalar(1632, (1.0 / (1.0 - p.p1599)));s.store_primal_mul_scale_offset_mixed_ia(1634, 1632, A::scale(s.ad_value(1631), ((0.05 * p.p1599) * (1.0 + p.p1599))), -1.0, 1.0);}
        if (((s.b[1523] && s.b[1638]) && (!s.b[1639])) && (!s.b[1650])) {s.store_scalar(1631, 10.0);s.store_scalar(1634, (1.5 - ((0.1) as f64).ln()));}
        if ((s.b[1523] && s.b[1638]) && (!s.b[1639])) {s.store_mul_ad_product_rhs(1633, 1631, A::offset(s.ad_value(1630), (-1.0)), A::scale_offset(s.ad_value(1630), (5.0 * p.p1599), (((((-1.0)) * ((5.0 * p.p1599)))) + ((1.0 + p.p1599)))));s.store_mul_ad_product_rhs_mixed_ia(535, 273, 527, A::add(s.ad_value(1633), s.ad_value(1634)));}
        if (s.b[1523] && (!s.b[1638])) {s.store_scalar(535, 0.0);}
        s.b[1660] = (s.v[528] > 0.0);s.store_scalar(1660, if s.b[1660] { 1.0 } else { 0.0 });
        if (s.b[1523] && s.b[1660]) {s.store_div(1652, 522, 274);}
        s.b[1661] = (s.v[1652] < 0.9);s.store_scalar(1661, if s.b[1661] { 1.0 } else { 0.0 });s.b[1662] = (p.p1607 > 0.0);s.store_scalar(1662, if s.b[1662] { 1.0 } else { 0.0 });s.b[1663] = (s.v[522] > s.v[567]);s.store_scalar(1663, if s.b[1663] { 1.0 } else { 0.0 });
        if ((((s.b[1523] && s.b[1660]) && s.b[1661]) && s.b[1662]) && s.b[1663]) {s.store_sub_from_scalar(1657, 1.0, 1652);}
        s.b[1664] = (p.p1601 != 1.0);s.store_scalar(1664, if s.b[1664] { 1.0 } else { 0.0 });s.b[1665] = (p.p1601 == 0.5);s.store_scalar(1665, if s.b[1665] { 1.0 } else { 0.0 });
        if ((((((s.b[1523] && s.b[1660]) && s.b[1661]) && s.b[1662]) && s.b[1663]) && s.b[1664]) && s.b[1665]) {s.store_div_from_scalar_sqrt_ad(1658, 1.0, s.ad_value(1657));}
        if ((((((s.b[1523] && s.b[1660]) && s.b[1661]) && s.b[1662]) && s.b[1663]) && s.b[1664]) && (!s.b[1665])) {s.store_powf(1658, 1657, (-p.p1601));}
        if (((((s.b[1523] && s.b[1660]) && s.b[1661]) && s.b[1662]) && s.b[1663]) && s.b[1664]) {s.store_mul_ad_affine_product_rhs(536, 274, s.ad_value(528), A::sub_from_scalar(1.0, A::mul(s.ad_value(1657), s.ad_value(1658))), 1.0 / ((1.0 - p.p1601)), 0.0);}
        if (((((s.b[1523] && s.b[1660]) && s.b[1661]) && s.b[1662]) && s.b[1663]) && (!s.b[1664])) {
            s.store_mul_ad_affine_product_rhs(536, 274, s.ad_value(528), {
                if (!(s.v[1657] > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if (s.v[1657] > 1e-38) {
                            A::ln(s.ad_value(1657))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, -1.0, 0.0);
        }
        if ((((s.b[1523] && s.b[1660]) && s.b[1661]) && s.b[1662]) && (!s.b[1663])) {s.store_sub_from_scalar_div_indices(1657, 1.0, 567, 274);}
        s.b[1666] = (p.p1601 != 1.0);s.store_scalar(1666, if s.b[1666] { 1.0 } else { 0.0 });s.b[1667] = (p.p1601 == 0.5);s.store_scalar(1667, if s.b[1667] { 1.0 } else { 0.0 });
        if ((((((s.b[1523] && s.b[1660]) && s.b[1661]) && s.b[1662]) && (!s.b[1663])) && s.b[1666]) && s.b[1667]) {s.store_div_from_scalar_sqrt_ad(1658, 1.0, s.ad_value(1657));}
        if ((((((s.b[1523] && s.b[1660]) && s.b[1661]) && s.b[1662]) && (!s.b[1663])) && s.b[1666]) && (!s.b[1667])) {s.store_powf(1658, 1657, (-p.p1601));}
        if (((((s.b[1523] && s.b[1660]) && s.b[1661]) && s.b[1662]) && (!s.b[1663])) && s.b[1666]) {s.store_mul_ad_affine_product_rhs(1659, 274, s.ad_value(528), A::sub_from_scalar(1.0, A::mul(s.ad_value(1657), s.ad_value(1658))), 1.0 / ((1.0 - p.p1601)), 0.0);}
        if (((((s.b[1523] && s.b[1660]) && s.b[1661]) && s.b[1662]) && (!s.b[1663])) && (!s.b[1666])) {
            s.store_mul_ad_affine_product_rhs(1659, 274, s.ad_value(528), {
                if (!(s.v[1657] > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if (s.v[1657] > 1e-38) {
                            A::ln(s.ad_value(1657))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, -1.0, 0.0);
        }
        if ((((s.b[1523] && s.b[1660]) && s.b[1661]) && s.b[1662]) && (!s.b[1663])) {s.store_sub_from_scalar_ad(1657, 1.0, A::div_scaled_inputs2(s.ad_value(522), 1.0, s.ad_value(567), (-1.0), s.ad_value(568), 1.0));}
        s.b[1668] = (p.p1613 != 1.0);s.store_scalar(1668, if s.b[1668] { 1.0 } else { 0.0 });s.b[1669] = (p.p1613 == 0.5);s.store_scalar(1669, if s.b[1669] { 1.0 } else { 0.0 });
        if ((((((s.b[1523] && s.b[1660]) && s.b[1661]) && s.b[1662]) && (!s.b[1663])) && s.b[1668]) && s.b[1669]) {s.store_div_from_scalar_sqrt_ad(1658, 1.0, s.ad_value(1657));}
        if ((((((s.b[1523] && s.b[1660]) && s.b[1661]) && s.b[1662]) && (!s.b[1663])) && s.b[1668]) && (!s.b[1669])) {s.store_powf(1658, 1657, (-p.p1613));}
        if (((((s.b[1523] && s.b[1660]) && s.b[1661]) && s.b[1662]) && (!s.b[1663])) && s.b[1668]) {s.store_add_product3_rhs_mixed_iia(536, 1659, 568, 528, A::sub_from_scalar(1.0, A::mul(s.ad_value(1657), s.ad_value(1658))), (p.p1607 * 1.0 / ((1.0 - p.p1613))));}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_119(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        if (((((s.b[1523] && s.b[1660]) && s.b[1661]) && s.b[1662]) && (!s.b[1663])) && (!s.b[1668])) {
            s.store_sub_mixed_ia(536, 1659, A::mul3_scaled_output(s.ad_value(568), s.ad_value(528), {
                            if (!(s.v[1657] > 1e-38)) {
                                A::neg(A::constant(87.498233534))
                            } else {
                                {
                                    if (s.v[1657] > 1e-38) {
                                        A::ln(s.ad_value(1657))
                                    } else {
                                        A::constant(0.0)
                                    }
                                }
                            }
                        }, p.p1607));
        }
        if (((s.b[1523] && s.b[1660]) && s.b[1661]) && (!s.b[1662])) {s.store_sub_from_scalar(1657, 1.0, 1652);}
        s.b[1670] = (p.p1601 != 1.0);s.store_scalar(1670, if s.b[1670] { 1.0 } else { 0.0 });s.b[1671] = (p.p1601 == 0.5);s.store_scalar(1671, if s.b[1671] { 1.0 } else { 0.0 });
        if (((((s.b[1523] && s.b[1660]) && s.b[1661]) && (!s.b[1662])) && s.b[1670]) && s.b[1671]) {s.store_div_from_scalar_sqrt_ad(1658, 1.0, s.ad_value(1657));}
        if (((((s.b[1523] && s.b[1660]) && s.b[1661]) && (!s.b[1662])) && s.b[1670]) && (!s.b[1671])) {s.store_powf(1658, 1657, (-p.p1601));}
        if ((((s.b[1523] && s.b[1660]) && s.b[1661]) && (!s.b[1662])) && s.b[1670]) {s.store_mul_ad_affine_product_rhs(536, 274, s.ad_value(528), A::sub_from_scalar(1.0, A::mul(s.ad_value(1657), s.ad_value(1658))), 1.0 / ((1.0 - p.p1601)), 0.0);}
        if ((((s.b[1523] && s.b[1660]) && s.b[1661]) && (!s.b[1662])) && (!s.b[1670])) {
            s.store_mul_ad_affine_product_rhs(536, 274, s.ad_value(528), {
                if (!(s.v[1657] > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if (s.v[1657] > 1e-38) {
                            A::ln(s.ad_value(1657))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, -1.0, 0.0);
        }
        s.b[1672] = (p.p1601 != 1.0);s.store_scalar(1672, if s.b[1672] { 1.0 } else { 0.0 });s.b[1673] = (p.p1601 == 0.5);s.store_scalar(1673, if s.b[1673] { 1.0 } else { 0.0 });
        if ((((s.b[1523] && s.b[1660]) && (!s.b[1661])) && s.b[1672]) && s.b[1673]) {s.store_scalar(1653, (1.0 / ((0.1) as f64).sqrt()));}
        if ((((s.b[1523] && s.b[1660]) && (!s.b[1661])) && s.b[1672]) && (!s.b[1673])) {s.store_scalar(1653, ((0.1) as f64).powf((-p.p1601)));}
        if (((s.b[1523] && s.b[1660]) && (!s.b[1661])) && s.b[1672]) {s.store_scalar(1654, (1.0 / (1.0 - p.p1601)));s.store_primal_mul_scale_offset_mixed_ia(1656, 1654, A::scale(s.ad_value(1653), ((0.05 * p.p1601) * (1.0 + p.p1601))), -1.0, 1.0);}
        if (((s.b[1523] && s.b[1660]) && (!s.b[1661])) && (!s.b[1672])) {s.store_scalar(1653, 10.0);s.store_scalar(1656, (1.5 - ((0.1) as f64).ln()));}
        if ((s.b[1523] && s.b[1660]) && (!s.b[1661])) {s.store_mul_ad_product_rhs(1655, 1653, A::offset(s.ad_value(1652), (-1.0)), A::scale_offset(s.ad_value(1652), (5.0 * p.p1601), (((((-1.0)) * ((5.0 * p.p1601)))) + ((1.0 + p.p1601)))));s.store_mul_ad_product_rhs_mixed_ia(536, 274, 528, A::add(s.ad_value(1655), s.ad_value(1656)));}
        if (s.b[1523] && (!s.b[1660])) {s.store_scalar(536, 0.0);}
        if s.b[1523] {s.store_add_scaled_inputs3_indices(533, 534, 1.0, 535, 1.0, 536, 1.0);}
        s.store_add_scaled_inputs(507, 529, 1.0, 521, s.v[515]);s.store_add_scaled_inputs(508, 533, 1.0, 522, s.v[516]);s.store_mul_ad_product_rhs_mixed_ia(509, 517, 114, A::voltage(ctx, nodes, Some(3), Some(10)));s.b[1674] = (p.p61 != 0.0);s.store_scalar(1674, if s.b[1674] { 1.0 } else { 0.0 });
        if s.b[1674] {s.store_mul_voltage_ad(170, s.ad_value(114), ctx, nodes, Some(10), Some(3));s.store_add_scaled_inputs4_offset_indices(171, 170, 1.0, 167, (-1.0), 146, 0.5, 166, 1.0, (-p.p1529));s.store_offset(168, 171, 0.02);s.store_scaled_add_mixed_ia(512, 168, A::sqrt_square_offset(s.ad_value(168), (4.0 * 0.02)), 0.5);s.store_sub_mixed_ia(509, 509, A::mul3_scaled_output(s.ad_value(156), s.ad_value(650), A::add_scaled_inputs_product(s.ad_value(171), 1.0, s.ad_value(512), (-1.0), s.ad_value(653), A::offset(A::sqrt(A::offset(A::div_scaled_inputs(s.ad_value(512), 4.0, s.ad_value(653), 1.0), 1.0)), (-1.0)), 0.5), s.v[115]));}
        s.store_mul_add_mixed_iia(169, 126, 865, A::mul3(s.ad_value(866), s.ad_value(126), s.ad_value(126)));s.store_div_scaled_product3_indices(168, 415, 372, 158, 1.0, 153, 1.0);s.store_div_scaled_inputs_indices(579, 428, 2.0, 415, 1.0);s.b[1678] = (((p.p1682 > 0.0) || (p.p1683 > 0.0)) || (p.p1684 > 0.0));s.store_scalar(1678, if s.b[1678] { 1.0 } else { 0.0 });
        if s.b[1678] {s.store_offset(580, 153, (-(2.0 * p.p1687)));}
        s.b[1679] = (s.v[580] <= 0.0);s.store_scalar(1679, if s.b[1679] { 1.0 } else { 0.0 });
        if (s.b[1678] && s.b[1679]) {s.copy_ad(580, 153);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_120(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.b[1680] = ((p.p79 == 1.0) || (p.p79 == 0.0));s.store_scalar(1680, if s.b[1680] { 1.0 } else { 0.0 });
        if (s.b[1678] && s.b[1680]) {s.store_square(581, 580);}
        s.b[1681] = (p.p1681 > 0.0);s.store_scalar(1681, if s.b[1681] { 1.0 } else { 0.0 });
        if ((s.b[1678] && s.b[1680]) && s.b[1681]) {s.store_div_scaled_offset_numerator_indices(168, 202, 1.0 / (s.v[578]), p.p1681, 579, 1.0);}
        if ((s.b[1678] && s.b[1680]) && s.b[1681]) {
            s.store_scale_ad(582, {
                if (!(s.v[168] > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if (s.v[168] > 1e-38) {
                            A::ln(s.ad_value(168))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, s.v[578]);
        }
        if ((s.b[1678] && s.b[1680]) && (!s.b[1681])) {s.store_scalar(582, 0.0);}
        s.b[1682] = (p.p79 == 1.0);s.store_scalar(1682, if s.b[1682] { 1.0 } else { 0.0 });
        if ((s.b[1678] && s.b[1680]) && s.b[1682]) {s.store_div(169, 400, 576);s.store_offset_pow_ad(170, s.ad_value(169), s.ad_value(575), 1.0);s.store_div(171, 574, 170);s.store_scale(172, 171, 1.0 / (p.p1682));s.store_scaled_add_offset_sqrt_square_offset(174, 172, 1.0, (-1.0), ((0.25 * p.p1688) * p.p1688), 0.5);s.store_scale(573, 174, p.p1682);}
        if ((s.b[1678] && s.b[1680]) && (!s.b[1682])) {s.store_scalar(573, p.p1682);}
        if (s.b[1678] && s.b[1680]) {s.store_mul_ad_affine_product_lhs(169, s.ad_value(179), A::abs(s.ad_value(124)), ((1.60219e-19 * 1.60219e-19) * 1.60219e-19), 0.0, 415);s.store_scaled_mul(170, 372, 581, 10000000000.0);s.store_scaled_mul(583, 372, 392, 6.241457005723417e18);s.store_scaled_mul(584, 372, 393, 6.241457005723417e18);s.store_mul_add_scaled_inputs_rhs_indices(585, 179, 372, 1.0 / (1.60219e-19), 669, 1.0 / (1.60219e-19));}
        if (s.b[1678] && s.b[1680]) {
            s.store_mul_mixed_ia(171, 573, {
                            if (!(((s.v[583] + s.v[585]) / (s.v[584] + s.v[585])) > 1e-38)) {
                                A::neg(A::constant(87.498233534))
                            } else {
                                {
                                    if (((s.v[583] + s.v[585]) / (s.v[584] + s.v[585])) > 1e-38) {
                                        A::ln(A::div_scaled_inputs2(s.ad_value(583), 1.0, s.ad_value(585), 1.0, A::add(s.ad_value(584), s.ad_value(585)), 1.0))
                                    } else {
                                        A::constant(0.0)
                                    }
                                }
                            }
                        });
        }
        if (s.b[1678] && s.b[1680]) {s.store_scaled_sub(172, 583, 584, p.p1683);s.store_scaled_sub_ad(174, A::square(s.ad_value(583)), A::square(s.ad_value(584)), (0.5 * p.p1684));s.store_mul3_affine_lhs(175, 179, 124, 1.60219e-19, 0.0, 124);s.store_scaled_mul(176, 581, 158, (10000000000.0 * s.v[115]));s.store_add_scaled_inputs_product_indices(177, 573, 1.0, 584, p.p1683, 584, 584, p.p1684);s.store_square_ad(178, A::add(s.ad_value(584), s.ad_value(585)));s.store_add_scaled_product(586, A::div_scaled_product3_by_product(s.ad_value(175), s.ad_value(582), s.ad_value(177), 1.0, s.ad_value(176), s.ad_value(178), 1.0), 1.0, A::div(s.ad_value(169), s.ad_value(170)), A::add_scaled_inputs3(s.ad_value(171), 1.0, s.ad_value(172), 1.0, s.ad_value(174), 1.0), 1.0);s.store_scaled_mul(340, 573, 179, 1.60219e-19);s.store_mul_product3_indices(341, 585, 158, 580, 585, (s.v[115] * 10000000000.0));s.store_mul_ad_product_lhs_mixed_ai(587, A::div(s.ad_value(340), s.ad_value(341)), 124, 124);s.store_add(169, 587, 586);}
        s.b[1684] = (p.p79 == 2.0);s.store_scalar(1684, if s.b[1684] { 1.0 } else { 0.0 });
        if ((s.b[1678] && (!s.b[1680])) && s.b[1684]) {s.store_div(169, 400, 576);s.store_offset_pow_ad(170, s.ad_value(169), s.ad_value(575), 1.0);s.store_div(171, 574, 170);s.store_scale(172, 171, 1.0 / (p.p1682));}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_121(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((s.b[1678] && (!s.b[1680])) && s.b[1684]) {s.store_scaled_add_offset_sqrt_square_offset(174, 172, 1.0, (-1.0), ((0.25 * p.p1688) * p.p1688), 0.5);s.store_scale(573, 174, p.p1682);s.store_div_scaled_inputs_indices(589, 179, 2.0, 217, 1.0);s.store_offset_mul(169, 589, 402, 1.0);s.store_offset_scaled(170, 402, p.p1685, 1.0);}
        s.b[1685] = ((s.v[169] > 0.0) && (s.v[170] > 0.0));s.store_scalar(1685, if s.b[1685] { 1.0 } else { 0.0 });
        if (((s.b[1678] && (!s.b[1680])) && s.b[1684]) && s.b[1685]) {
            s.store_mul_scale_offset(171, {
                if (!(((s.v[392] + 0.5) / (s.v[393] + 0.5)) > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if (((s.v[392] + 0.5) / (s.v[393] + 0.5)) > 1e-38) {
                            A::ln(A::div_scaled_offset_numerator(s.ad_value(392), 1.0, 0.5, A::offset(s.ad_value(393), 0.5), 1.0))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, A::add(s.ad_value(392), s.ad_value(393)), 1.0, 1.0);
        }
        if (((s.b[1678] && (!s.b[1680])) && s.b[1684]) && s.b[1685]) {s.store_scaled_sub(172, 392, 393, 2.0);}
        s.b[1686] = (p.p72 == 0.0);s.store_scalar(1686, if s.b[1686] { 1.0 } else { 0.0 });s.b[1687] = (p.p72 == 1.0);s.store_scalar(1687, if s.b[1687] { 1.0 } else { 0.0 });
        if s.b[1686] {s.store_mul(168, 415, 592);s.store_add_scaled_square_product_indices(169, 153, 1.0, 168, 197, 1.0);}
        if (s.b[1687] && (!s.b[1686])) {s.store_div(168, 399, 217);s.store_square(168, 168);s.store_scaled_offset_ad(597, A::mul_scaled_lhs(s.ad_value(168), p.p1709, s.ad_value(153)), 1.0, p.p1708);s.store_scaled_offset_ad(598, A::mul_scaled_lhs(s.ad_value(168), p.p1711, s.ad_value(153)), 1.0, p.p1710);s.store_scaled_offset_ad(599, A::mul_scaled_lhs(s.ad_value(168), p.p1713, s.ad_value(153)), 1.0, p.p1712);s.store_scaled_offset_ad(600, A::mul_scaled_lhs(s.ad_value(168), p.p1715, s.ad_value(153)), 1.0, p.p1714);s.store_scaled_mul(169, 597, 597, 3.0);s.store_scaled_mul(170, 598, 598, 7.5);s.store_scale(171, 599, 2.5298);s.store_mul_scale_offset(601, A::div(s.ad_value(393), s.ad_value(392)), A::div(s.ad_value(390), s.ad_value(210)), -1.0, 1.0);s.store_mul_square_lhs(604, 209, 209);s.store_div_add_scaled_inputs_rhs_indices(602, 339, 339, 1.0, 399, 1.0);s.store_div_mixed_ia(172, 236, A::add_scaled_product(s.ad_value(181), 2.0, A::max_from_scalar(0.0, s.ad_value(237)), s.ad_value(392), 1.0));s.store_limited_exp_neg_input(616, 172);}
        s.b[1688] = (p.p61 == 2.0);s.store_scalar(1688, if s.b[1688] { 1.0 } else { 0.0 });
        if ((s.b[1687] && (!s.b[1686])) && s.b[1688]) {
            if (!(s.v[293] < ((-10000.0) * 1e-6))) {
                s.store_scaled_add_mixed_ia(172, 293, A::sqrt_square_offset(s.ad_value(293), ((4.0 * 1e-6) * 1e-6)), 0.5);
            } else {
                if (s.v[293] < ((-10000.0) * 1e-6)) {
                    s.store_div_from_scalar(172, ((-1e-6) * 1e-6), 293);
                } else {
                    s.store_scalar(172, 0.0);
                }
            }
        }
        if ((s.b[1687] && (!s.b[1686])) && s.b[1688]) {s.store_div_mixed_ia(174, 172, A::add_scaled_product(s.ad_value(181), 2.0, A::max_from_scalar(0.0, s.ad_value(238)), s.ad_value(392), 1.0));s.store_sub_ad(175, A::sqrt(A::sub(s.ad_value(689), s.ad_value(370))), A::sqrt(s.ad_value(689)));}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_122(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((s.b[1687] && (!s.b[1686])) && s.b[1688]) {s.store_limited_exp_ad(617, A::mul_scaled_lhs(s.ad_value(174), -1.0, s.ad_value(175)));}
        if ((s.b[1687] && (!s.b[1686])) && (!s.b[1688])) {s.store_scalar(617, 1.0);}
        if (s.b[1687] && (!s.b[1686])) {s.store_add_scaled_product_indices(615, 401, s.v[420], 407, 392, s.v[420]);s.store_pow_ad(172, A::scaled_offset(A::abs(A::div(s.ad_value(392), s.ad_value(406))), 1.0, 0.5), s.ad_value(317));}
        s.b[1689] = (p.p61 != 0.0);s.store_scalar(1689, if s.b[1689] { 1.0 } else { 0.0 });
        if ((s.b[1687] && (!s.b[1686])) && s.b[1689]) {s.store_add_scaled_product(174, A::div(s.ad_value(820), s.ad_value(172)), 1.0, A::add_scaled_product(s.ad_value(819), 1.0, s.ad_value(821), s.ad_value(370), 1.0), A::pow(A::abs(s.ad_value(615)), s.ad_value(822)), 1.0);}
        if ((s.b[1687] && (!s.b[1686])) && (!s.b[1689])) {s.store_add_scaled_product_mixed_aia(174, A::div(s.ad_value(820), s.ad_value(172)), 1.0, 819, A::pow(A::abs(s.ad_value(615)), s.ad_value(822)), 1.0);}
        if (s.b[1687] && (!s.b[1686])) {s.store_offset(618, 174, 1.0);s.store_scaled_add_offset_sqrt_square_offset(618, 618, 1.0, (-1.0), ((0.25 * p.p604) * p.p604), 0.5);s.store_scale(618, 618, 1.0 / (p.p24));s.store_scalar(619, (1.0 + (0.25 * p.p453)));s.store_div_add_scaled_inputs_rhs_indices(612, 339, 339, 1.0, 392, 1.0);s.store_mul_scale_offset_indices(172, 181, 612, -1.0, 2.0);s.store_add(613, 392, 172);}
        s.b[1690] = (p.p64 == 0.0);s.store_scalar(1690, if s.b[1690] { 1.0 } else { 0.0 });s.b[1691] = (p.p64 == 1.0);s.store_scalar(1691, if s.b[1691] { 1.0 } else { 0.0 });s.b[1692] = (p.p64 == 2.0);s.store_scalar(1692, if s.b[1692] { 1.0 } else { 0.0 });
        if ((s.b[1687] && (!s.b[1686])) && s.b[1690]) {s.store_offset_mul(172, 711, 392, 1.0);s.store_div_from_scalar(174, 1.0, 172);s.store_scaled_add_mixed_ia(175, 174, A::sqrt_square_offset(s.ad_value(174), 0.01), 0.5);s.store_mul_ad_product_lhs_mixed_ia(614, 194, A::offset(A::mul(s.ad_value(709), s.ad_value(175)), p.p908), 189);s.store_offset_mul_ad(620, A::div_scaled_product_by_product(s.ad_value(183), s.ad_value(613), s.v[115], s.ad_value(618), s.ad_value(619), 1.0), s.ad_value(614), 1.0);}
        if ((s.b[1687] && (!s.b[1686])) && (s.b[1691] && (!s.b[1690]))) {s.store_scalar(620, 1.0);}
        if ((s.b[1687] && (!s.b[1686])) && (s.b[1692] && (!(s.b[1690] || s.b[1691])))) {s.store_offset_mul(172, 711, 392, 1.0);s.store_div_from_scalar(174, 1.0, 172);s.store_scaled_add_mixed_ia(175, 174, A::sqrt_square_offset(s.ad_value(174), 0.01), 0.5);s.store_mul_scale_offset_mixed_ia(614, 189, A::mul(s.ad_value(709), s.ad_value(175)), 1.0, p.p908);s.store_mul_add_scaled_inputs3_offset_rhs_indices(614, 194, 190, 1.0, 191, 1.0, 614, 1.0, 0.0);s.store_offset_mul_ad(620, A::div_scaled_product_by_product(s.ad_value(183), s.ad_value(613), s.v[115], s.ad_value(618), s.ad_value(619), 1.0), s.ad_value(614), 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_123(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (s.b[1687] && (!s.b[1686])) {s.store_div_scaled_product_mixed_aia(603, A::mul3_scaled_output(s.ad_value(183), s.ad_value(392), s.ad_value(616), s.v[115]), 617, 1.0, A::mul3(s.ad_value(618), s.ad_value(619), s.ad_value(620)), 1.0);s.store_offset(172, 601, 1.0);s.store_sub_from_scalar(174, 1.0, 601);s.store_mul_div_scaled_inputs_indices(175, 181, 602, 2.0, 392, 1.0);s.store_add(176, 172, 175);s.store_square(605, 174);s.store_mul(606, 605, 174);s.store_mul(607, 606, 174);s.store_square(608, 176);s.store_mul(609, 608, 176);s.store_mul(610, 609, 176);s.store_mul(611, 610, 176);s.store_scale(621, 172, 0.5);s.store_div_scaled_inputs_indices(622, 605, 1.0, 176, 6.0);s.store_mul_div_scaled_inputs_mixed_aii(623, A::add(s.ad_value(621), s.ad_value(622)), 205, 1.0, 209, 1.0);s.store_div(624, 172, 608);s.store_div_scaled_product_mixed_aii(625, A::add_scaled_inputs(s.ad_value(172), 6.0, s.ad_value(175), 1.0), 605, 1.0, 610, 15.0);s.store_div_scaled_inputs_indices(626, 607, 1.0, 611, 9.0);s.store_mul_ad_affine_product_rhs(627, 205, s.ad_value(604), A::add_scaled_inputs3(s.ad_value(624), 1.0, s.ad_value(625), (-1.0), s.ad_value(626), 1.0), 1.0 / (6.0), 0.0);s.store_offset_mul_ad(177, A::div_scaled_product_offset_denominator(s.ad_value(600), s.ad_value(600), 1.0, s.ad_value(399), p.p1716, 1.0), A::div(s.ad_value(390), s.ad_value(210)), 1.0);s.store_mul_div_scaled_inputs_mixed_aii(623, A::add_scaled_products(s.ad_value(177), s.ad_value(621), 1.0, s.ad_value(169), s.ad_value(622), 1.0), 205, 1.0, 209, 1.0);s.store_mul_product3_mixed_aiii(627, A::add_scaled_inputs3(s.ad_value(624), 1.0, s.ad_value(625), (-1.0), s.ad_value(626), 1.0), 205, 604, 170, 1.0 / (6.0));s.store_div_scaled_product_mixed_aii(632, A::mul3_scaled_output(A::sqrt(A::div(s.ad_value(627), s.ad_value(623))), s.ad_value(372), s.ad_value(159), s.v[115]), 156, 1.0, 603, 1.0);}
        s.b[1696] = (p.p73 == 2.0);s.store_scalar(1696, if s.b[1696] { 1.0 } else { 0.0 });s.b[1705] = (p.p76 != 2.0);s.store_scalar(1705, if s.b[1705] { 1.0 } else { 0.0 });s.b[1706] = (p.p65 == 1.0);s.store_scalar(1706, if s.b[1706] { 1.0 } else { 0.0 });s.b[1707] = (p.p78 == 1.0);s.store_scalar(1707, if s.b[1707] { 1.0 } else { 0.0 });s.b[1708] = (p.p65 == 1.0);s.store_scalar(1708, if s.b[1708] { 1.0 } else { 0.0 });s.b[1709] = (p.p78 == 1.0);s.store_scalar(1709, if s.b[1709] { 1.0 } else { 0.0 });s.b[1710] = (p.p61 != 0.0);s.store_scalar(1710, if s.b[1710] { 1.0 } else { 0.0 });s.b[1711] = (p.p64 == 1.0);s.store_scalar(1711, if s.b[1711] { 1.0 } else { 0.0 });s.b[1712] = (p.p1910 > 0.0);s.store_scalar(1712, if s.b[1712] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_124(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        if (s.b[1711] && s.b[1712]) {
            if (!(((1.0 + (p.p1912 * s.v[232])) - 1e-6) < ((-10000.0) * 0.001))) {
                s.store_scaled_add_sqrt_square_offset_ad(1039, A::scale_offset(s.ad_value(232), p.p1912, ((1.0) + ((-1e-6)))), ((4.0 * 0.001) * 0.001), 0.5);
            } else {
                if (((1.0 + (p.p1912 * s.v[232])) - 1e-6) < ((-10000.0) * 0.001)) {
                    s.store_div_from_scalar_offset_scaled_input(1039, ((-0.001) * 0.001), 232, p.p1912, ((1.0) + ((-1e-6))));
                } else {
                    s.store_scalar(1039, 0.0);
                }
            }
        }
        s.b[1713] = (p.p75 != 0.0);s.store_scalar(1713, if s.b[1713] { 1.0 } else { 0.0 });
        if ((s.b[1711] && s.b[1712]) && s.b[1713]) {s.store_offset_add_scaled_inputs(1044, A::scale_offset(s.ad_value(232), (-p.p1913), (((-(-p.p1904))) + ((-1e-6)))), 0.5, A::sqrt_square_offset(A::scale_offset(s.ad_value(232), (-p.p1913), (((-(-p.p1904))) + ((-1e-6)))), (-((4.0 * (-p.p1904)) * 1e-6))), 0.5, (((-p.p1904)) + (p.p1904)));}
        if ((s.b[1711] && s.b[1712]) && (!s.b[1713])) {
            s.store_scale_ad(1044, {
                if (!(((1.0 + ((-p.p1913) * s.v[232])) - 1e-6) < ((-10000.0) * 0.001))) {
                    A::add_scaled_inputs(A::scale_offset(s.ad_value(232), (-p.p1913), ((1.0) + ((-1e-6)))), 0.5, A::sqrt_square_offset(A::scale_offset(s.ad_value(232), (-p.p1913), ((1.0) + ((-1e-6)))), ((4.0 * 0.001) * 0.001)), 0.5)
                } else {
                    {
                        if (((1.0 + ((-p.p1913) * s.v[232])) - 1e-6) < ((-10000.0) * 0.001)) {
                            A::div_from_scalar(((-0.001) * 0.001), A::scale_offset(s.ad_value(232), (-p.p1913), ((1.0) + ((-1e-6)))))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, p.p1904);
        }
        if (s.b[1711] && s.b[1712]) {s.store_offset(168, 392, (-p.p1906));s.store_scaled_add_offset_sqrt_square_offset(168, 168, 0.1, (-0.1), ((0.25 * 2.0) * 2.0), 0.5);s.store_div_scaled_value_offset_denominator(169, s.ad_value(168), (10.0 * p.p1907), s.ad_value(168), (10.0 * p.p1907), 1.0);s.store_mul_scale_offset_rhs(1045, 1044, 169, p.p1905, 1.0);}
        if (s.b[1711] && s.b[1712]) {
            if (!(s.v[1045] < ((-10000.0) * 10.0))) {
                s.store_scaled_add_mixed_ia(1045, 1045, A::sqrt_square_offset(s.ad_value(1045), ((4.0 * 10.0) * 10.0)), 0.5);
            } else {
                if (s.v[1045] < ((-10000.0) * 10.0)) {
                    s.store_div_from_scalar(1045, ((-10.0) * 10.0), 1045);
                } else {
                    s.store_scalar(1045, 0.0);
                }
            }
        }
        if (s.b[1711] && s.b[1712]) {s.store_scaled_mul(170, 158, 1045, (s.v[115] * 1.60219e-19));s.store_abs_voltage(174, ctx, nodes, Some(9), Some(7));}
        s.b[1714] = (p.p1917 == 0.0);s.store_scalar(1714, if s.b[1714] { 1.0 } else { 0.0 });
        if ((s.b[1711] && s.b[1712]) && s.b[1714]) {s.store_scalar(171, 1.0);}
        if ((s.b[1711] && s.b[1712]) && (!s.b[1714])) {s.store_scaled_add_sqrt_square_offset_ad(171, A::offset(s.ad_value(174), (-p.p1916)), ((0.25 * 0.5) * 0.5), 0.5);s.store_offset_scaled(171, 171, p.p1917, 1.0);}
        if (s.b[1711] && s.b[1712]) {s.store_scaled_mul(1047, 170, 171, p.p1903);s.store_scaled_mul(172, 1039, 189, p.p1910);s.store_mul(1048, 1047, 172);let t0: A = A::powf(s.ad_value(174), (4.0 - p.p1908));s.store_div_ad(1050, t0, A::add_scaled_inputs(t0, 1.0, A::powf(s.ad_value(1048), (4.0 - p.p1908)), p.p1914));s.store_div_scaled_product_mixed_aii(175, A::powf(s.ad_value(1050), (1.0 / p.p1908)), 174, 1.0, 1048, 1.0);}
        s.b[1715] = (p.p1911 > 0.0);s.store_scalar(1715, if s.b[1715] { 1.0 } else { 0.0 });s.b[1716] = (p.p1910 == 0.0);s.store_scalar(1716, if s.b[1716] { 1.0 } else { 0.0 });
    }
}
