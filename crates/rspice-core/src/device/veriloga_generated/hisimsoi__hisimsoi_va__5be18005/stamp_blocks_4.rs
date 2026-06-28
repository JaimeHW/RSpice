#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, ReactiveScratch, Scratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_reactive_block_28(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && (!s.b[1630])) && s.b[1637]) {
            s.store_mul(458, 1556, 1605);
            s.store_div_from_scalar_add_ad(1524, 1.0, s.ad_value(1597), s.ad_value(1605));
            s.store_mul3_lhs(460, 1556, 1593, 1524);
            s.store_add(459, 458, 460);
        }

        if (((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) {
            s.store_sub(460, 459, 458);
        }

        if (((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) {
            if (p.p43 == 1.0) {
                s.store_mul(1527, 287, 1536);
            } else {
                s.store_mul(1527, 108, 1536);
            }
        }

        s.b[1646] = (((s.v[1542] != 0.0) && (p.p43 == 0.0)) || ((s.v[1540] != 0.0) && (p.p43 == 1.0)));
        s.v[1646] = if s.b[1646] { 1.0 } else { 0.0 };

        if ((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && s.b[1646]) {
            s.store_mul(455, 1527, 459);
            s.store_mul(457, 1527, 458);
        }

        s.b[1647] = (((s.v[1543] != 0.0) && (p.p43 == 0.0)) || ((s.v[1541] != 0.0) && (p.p43 == 1.0)));
        s.v[1647] = if s.b[1647] { 1.0 } else { 0.0 };

        if ((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && s.b[1647]) {
            s.store_mul(454, 1527, 459);
            s.store_mul(456, 1527, 458);
        }

        if ((p.p24 != 0.0) && s.b[1606]) {
            s.store_add_scaled_inputs(266, 462, s.v[566], 461, s.v[565]);
        }

        if (((p.p24 != 0.0) && s.b[1606]) && (s.v[266] != 0.0)) {
            s.store_add_scaled_inputs(269, 462, p.p170, 461, p.p169);
        }

        s.b[1648] = (p.p43 == 1.0);
        s.v[1648] = if s.b[1648] { 1.0 } else { 0.0 };

        if ((((p.p24 != 0.0) && s.b[1606]) && (s.v[266] != 0.0)) && s.b[1648]) {
            s.store_add_scaled_products_indices(1524, 462, 287, 1.0, 461, 288, 1.0);
            s.store_mul_neg_rhs(269, 269, 1524);
        }

        if ((((p.p24 != 0.0) && s.b[1606]) && (s.v[266] != 0.0)) && (!s.b[1648])) {
            s.store_mul_neg_rhs(269, 269, 108);
        }

        if (((p.p24 != 0.0) && s.b[1606]) && (s.v[266] != 0.0)) {
            s.store_add_scaled_product_right_sub(268, 268, 1.0, 269, 158, 157, -1.0);
        }

        if ((p.p24 != 0.0) && s.b[1606]) {
            s.store_add_scaled_inputs(266, 461, s.v[566], 462, s.v[565]);
        }

        if (((p.p24 != 0.0) && s.b[1606]) && (s.v[266] != 0.0)) {
            s.store_add_scaled_inputs(270, 461, p.p170, 462, p.p169);
        }

        s.b[1649] = (p.p43 == 1.0);
        s.v[1649] = if s.b[1649] { 1.0 } else { 0.0 };

        if ((((p.p24 != 0.0) && s.b[1606]) && (s.v[266] != 0.0)) && s.b[1649]) {
            s.store_add_scaled_products_indices(1524, 461, 287, 1.0, 462, 288, 1.0);
            s.store_mul_neg_rhs(270, 270, 1524);
        }

        if ((((p.p24 != 0.0) && s.b[1606]) && (s.v[266] != 0.0)) && (!s.b[1649])) {
            s.store_mul_neg_rhs(270, 270, 108);
        }

        if (((p.p24 != 0.0) && s.b[1606]) && (s.v[266] != 0.0)) {
            s.store_add_scaled_product_indices(267, 267, 1.0, 270, 158, -1.0);
        }

        s.b[1650] = (((s.v[613] == 1.0) && (!s.b[565])) || ((s.v[613] != 1.0) && (!s.b[566])));
        s.v[1650] = if s.b[1650] { 1.0 } else { 0.0 };

        s.b[1651] = (p.p43 == 1.0);
        s.v[1651] = if s.b[1651] { 1.0 } else { 0.0 };

        if ((((p.p24 != 0.0) && (!s.b[1606])) && s.b[1650]) && s.b[1651]) {
            s.store_scale(269, 288, ((-s.v[1534]) * p.p188));
        }

        if ((((p.p24 != 0.0) && (!s.b[1606])) && s.b[1650]) && (!s.b[1651])) {
            s.store_scale(269, 108, ((-s.v[1534]) * p.p188));
        }

        if (((p.p24 != 0.0) && (!s.b[1606])) && (!s.b[1650])) {
            s.store_add_scaled_inputs(269, 462, p.p170, 461, p.p169);
        }

        s.b[1652] = (p.p43 == 1.0);
        s.v[1652] = if s.b[1652] { 1.0 } else { 0.0 };

        if ((((p.p24 != 0.0) && (!s.b[1606])) && (!s.b[1650])) && s.b[1652]) {
            s.store_add_scaled_products_indices(1524, 462, 287, 1.0, 461, 288, 1.0);
            s.store_mul_neg_rhs(269, 269, 1524);
        }

        if ((((p.p24 != 0.0) && (!s.b[1606])) && (!s.b[1650])) && (!s.b[1652])) {
            s.store_mul_neg_rhs(269, 269, 108);
        }

        if ((p.p24 != 0.0) && (!s.b[1606])) {
            s.store_mul_sub_scaled_inputs_rhs(268, 269, s.ad_value(158), -1.0, s.ad_value(157), -1.0);
        }

        s.b[1653] = (((s.v[613] == 1.0) && (!s.b[566])) || ((s.v[613] != 1.0) && (!s.b[565])));
        s.v[1653] = if s.b[1653] { 1.0 } else { 0.0 };

        s.b[1654] = (p.p43 == 1.0);
        s.v[1654] = if s.b[1654] { 1.0 } else { 0.0 };

        if ((((p.p24 != 0.0) && (!s.b[1606])) && s.b[1653]) && s.b[1654]) {
            s.store_scale(270, 287, ((-s.v[1534]) * p.p188));
        }

        if ((((p.p24 != 0.0) && (!s.b[1606])) && s.b[1653]) && (!s.b[1654])) {
            s.store_scale(270, 108, ((-s.v[1534]) * p.p188));
        }

        if (((p.p24 != 0.0) && (!s.b[1606])) && (!s.b[1653])) {
            s.store_add_scaled_inputs(270, 461, p.p170, 462, p.p169);
        }

        s.b[1655] = (p.p43 == 1.0);
        s.v[1655] = if s.b[1655] { 1.0 } else { 0.0 };

        if ((((p.p24 != 0.0) && (!s.b[1606])) && (!s.b[1653])) && s.b[1655]) {
            s.store_add_scaled_products_indices(1524, 461, 287, 1.0, 462, 288, 1.0);
            s.store_mul_neg_rhs(270, 270, 1524);
        }

        if ((((p.p24 != 0.0) && (!s.b[1606])) && (!s.b[1653])) && (!s.b[1655])) {
            s.store_mul_neg_rhs(270, 270, 108);
        }

        if ((p.p24 != 0.0) && (!s.b[1606])) {
            s.store_mul_neg_lhs(267, 270, 158);
        }

        s.b[1656] = (p.p43 == 1.0);
        s.v[1656] = if s.b[1656] { 1.0 } else { 0.0 };

        if s.b[1656] {
            s.copy_ad(1672, 590);
            s.copy_ad(1673, 591);
            s.store_scale_ad(1674, A::exp_scaled_input(A::add_scaled_inputs(A::sub_from_scalar((s.v[87] * s.v[114]), A::mul(s.ad_value(237), s.ad_value(225))), 1.0, A::ln_scaled_input(s.ad_value(429), 1.0 / (s.v[81])), p.p175), 1.0 / (p.p174)), p.p173);
            s.store_scale_ad(1675, A::exp_scaled_input(A::add_scaled_inputs(A::sub_from_scalar((s.v[87] * s.v[114]), A::mul(s.ad_value(237), s.ad_value(225))), 1.0, A::ln_scaled_input(s.ad_value(429), 1.0 / (s.v[81])), p.p176), 1.0 / (p.p174)), p.p173);
            s.store_scaled_mul(1679, 286, 1674, p.p237);
            s.store_scaled_mul(1681, 286, 1675, p.p237);
            s.store_scaled_mul(1680, 285, 1674, p.p237);
            s.store_scaled_mul(1682, 285, 1675, p.p237);
            s.store_scale(1658, 429, 1.0 / (s.v[81]));
            s.store_offset(1659, 1679, 1e-50);
            s.store_scale_ad(1677, A::div_from_scalar(p.p174, s.ad_value(225)), 0.0);
            s.store_scale_ad(1678, A::div_from_scalar(p.p174, s.ad_value(225)), 0.0);
            s.store_scale(1676, 227, p.p174);
        }

        s.b[1685] = (s.v[1672] < s.v[1677]);
        s.v[1685] = if s.b[1685] { 1.0 } else { 0.0 };

        if (s.b[1656] && s.b[1685]) {
            s.store_exp_div(1658, 1672, 1676);
            s.store_mul_offset_rhs(282, 1679, 1658, (-1.0));
        }

        if (s.b[1656] && (!s.b[1685])) {
            s.store_exp_div(1658, 1677, 1676);
            s.store_add_scaled_offset_product_rhs_mixed_aii(282, A::mul3(A::div(s.ad_value(1679), s.ad_value(1676)), s.ad_value(1658), A::sub(s.ad_value(1672), s.ad_value(1677))), 1.0, 1679, 1658, (-1.0), 1.0);
        }

        if s.b[1656] {
            s.store_add_scaled_product_indices(282, 282, 1.0, 1672, 1681, p.p178);
        }

        s.b[1686] = (s.v[1673] < s.v[1678]);
        s.v[1686] = if s.b[1686] { 1.0 } else { 0.0 };

        if (s.b[1656] && s.b[1686]) {
            s.store_exp_div(1658, 1673, 1676);
            s.store_mul_offset_rhs(281, 1680, 1658, (-1.0));
        }

        if (s.b[1656] && (!s.b[1686])) {
            s.store_exp_div(1658, 1678, 1676);
            s.store_add_scaled_offset_product_rhs_mixed_aii(281, A::mul3(A::div(s.ad_value(1680), s.ad_value(1676)), s.ad_value(1658), A::sub(s.ad_value(1673), s.ad_value(1678))), 1.0, 1680, 1658, (-1.0), 1.0);
        }

        if s.b[1656] {
            s.store_add_scaled_product_indices(281, 281, 1.0, 1673, 1682, p.p178);
            s.store_add_scaled_inputs(282, 282, 1.0, 1672, s.v[142]);
            s.store_add_scaled_inputs(281, 281, 1.0, 1673, s.v[142]);
            s.store_scalar(1666, (p.p179 * p.p2));
            s.store_scalar(1667, (p.p179 * p.p3));
            s.store_scalar(1665, (p.p237 - p.p238));
        }

        s.b[1687] = (s.v[1665] <= 0.0);
        s.v[1687] = if s.b[1687] { 1.0 } else { 0.0 };

        if (s.b[1656] && s.b[1687]) {
            s.store_scalar(1666, 0.0);
            s.store_scalar(1667, 0.0);
        }

        s.b[1688] = (p.p5 > s.v[287]);
        s.v[1688] = if s.b[1688] { 1.0 } else { 0.0 };

        if (s.b[1656] && s.b[1688]) {
            s.store_offset_scaled(1669, 287, (-p.p180), ((p.p5) * (p.p180)));
            s.store_scale(1671, 287, p.p181);
        }

        s.b[1689] = (s.v[1673] < 0.0);
        s.v[1689] = if s.b[1689] { 1.0 } else { 0.0 };

        s.b[1690] = (s.v[1667] > 0.0);
        s.v[1690] = if s.b[1690] { 1.0 } else { 0.0 };

        if (((s.b[1656] && s.b[1688]) && s.b[1689]) && s.b[1690]) {
            s.store_sub_from_scalar_scaled_input(1683, 1.0, 1673, 1.0 / (p.p185));
        }

        s.b[1691] = (p.p182 == 0.5);
        s.v[1691] = if s.b[1691] { 1.0 } else { 0.0 };

        if ((((s.b[1656] && s.b[1688]) && s.b[1689]) && s.b[1690]) && s.b[1691]) {
            s.store_div_from_scalar_sqrt_ad(1684, 1.0, s.ad_value(1683));
        }

        if ((((s.b[1656] && s.b[1688]) && s.b[1689]) && s.b[1690]) && (!s.b[1691])) {
            s.store_powf(1684, 1683, (-p.p182));
        }

        if (((s.b[1656] && s.b[1688]) && s.b[1689]) && s.b[1690]) {
            s.store_mul_sub_from_scalar_ad_rhs_scaled_output(283, 1667, 1.0, A::mul(s.ad_value(1683), s.ad_value(1684)), (p.p185 * 1.0 / ((1.0 - p.p182))));
        }

        if (((s.b[1656] && s.b[1688]) && s.b[1689]) && (!s.b[1690])) {
            s.store_scalar(283, 0.0);
        }

        s.b[1692] = (s.v[1669] > 0.0);
        s.v[1692] = if s.b[1692] { 1.0 } else { 0.0 };

        if (((s.b[1656] && s.b[1688]) && s.b[1689]) && s.b[1692]) {
            s.store_sub_from_scalar_scaled_input(1683, 1.0, 1673, 1.0 / (p.p186));
        }

        s.b[1693] = (p.p183 == 0.5);
        s.v[1693] = if s.b[1693] { 1.0 } else { 0.0 };

        if ((((s.b[1656] && s.b[1688]) && s.b[1689]) && s.b[1692]) && s.b[1693]) {
            s.store_div_from_scalar_sqrt_ad(1684, 1.0, s.ad_value(1683));
        }

        if ((((s.b[1656] && s.b[1688]) && s.b[1689]) && s.b[1692]) && (!s.b[1693])) {
            s.store_powf(1684, 1683, (-p.p183));
        }

        if (((s.b[1656] && s.b[1688]) && s.b[1689]) && s.b[1692]) {
            s.store_add_ad_rhs(283, 283, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1669), 1.0, A::mul(s.ad_value(1683), s.ad_value(1684)), (p.p186 * 1.0 / ((1.0 - p.p183)))));
        }

        s.b[1694] = (s.v[1671] > 0.0);
        s.v[1694] = if s.b[1694] { 1.0 } else { 0.0 };

        if (((s.b[1656] && s.b[1688]) && s.b[1689]) && s.b[1694]) {
            s.store_sub_from_scalar_scaled_input(1683, 1.0, 1673, 1.0 / (p.p187));
        }

        s.b[1695] = (p.p184 == 0.5);
        s.v[1695] = if s.b[1695] { 1.0 } else { 0.0 };

        if ((((s.b[1656] && s.b[1688]) && s.b[1689]) && s.b[1694]) && s.b[1695]) {
            s.store_div_from_scalar_sqrt_ad(1684, 1.0, s.ad_value(1683));
        }

        if ((((s.b[1656] && s.b[1688]) && s.b[1689]) && s.b[1694]) && (!s.b[1695])) {
            s.store_powf(1684, 1683, (-p.p184));
        }

        if (((s.b[1656] && s.b[1688]) && s.b[1689]) && s.b[1694]) {
            s.store_add_ad_rhs(283, 283, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1671), 1.0, A::mul(s.ad_value(1683), s.ad_value(1684)), (p.p187 * 1.0 / ((1.0 - p.p184)))));
        }

        if ((s.b[1656] && s.b[1688]) && (!s.b[1689])) {
            s.store_add_scaled_inputs3_indices(1658, 1667, 1.0, 1669, 1.0, 1671, 1.0);
            s.store_add_scaled_inputs3_indices(1659, 1667, (p.p182 * 1.0 / (p.p185)), 1669, (p.p183 * 1.0 / (p.p186)), 1671, (p.p184 * 1.0 / (p.p187)));
            s.store_mul_add_scaled_product_rhs(283, 1673, s.ad_value(1658), 1.0, s.ad_value(1673), s.ad_value(1659), 0.5);
        }

        if (s.b[1656] && (!s.b[1688])) {
            s.store_scalar(1671, (p.p181 * p.p5));
        }

        s.b[1696] = (s.v[1673] < 0.0);
        s.v[1696] = if s.b[1696] { 1.0 } else { 0.0 };

        s.b[1697] = (s.v[1667] > 0.0);
        s.v[1697] = if s.b[1697] { 1.0 } else { 0.0 };

        if (((s.b[1656] && (!s.b[1688])) && s.b[1696]) && s.b[1697]) {
            s.store_sub_from_scalar_scaled_input(1683, 1.0, 1673, 1.0 / (p.p185));
        }

        s.b[1698] = (p.p182 == 0.5);
        s.v[1698] = if s.b[1698] { 1.0 } else { 0.0 };

        if ((((s.b[1656] && (!s.b[1688])) && s.b[1696]) && s.b[1697]) && s.b[1698]) {
            s.store_div_from_scalar_sqrt_ad(1684, 1.0, s.ad_value(1683));
        }

        if ((((s.b[1656] && (!s.b[1688])) && s.b[1696]) && s.b[1697]) && (!s.b[1698])) {
            s.store_powf(1684, 1683, (-p.p182));
        }

        if (((s.b[1656] && (!s.b[1688])) && s.b[1696]) && s.b[1697]) {
            s.store_mul_sub_from_scalar_ad_rhs_scaled_output(283, 1667, 1.0, A::mul(s.ad_value(1683), s.ad_value(1684)), (p.p185 * 1.0 / ((1.0 - p.p182))));
        }

        if (((s.b[1656] && (!s.b[1688])) && s.b[1696]) && (!s.b[1697])) {
            s.store_scalar(283, 0.0);
        }

        s.b[1699] = (s.v[1671] > 0.0);
        s.v[1699] = if s.b[1699] { 1.0 } else { 0.0 };

        if (((s.b[1656] && (!s.b[1688])) && s.b[1696]) && s.b[1699]) {
            s.store_sub_from_scalar_scaled_input(1683, 1.0, 1673, 1.0 / (p.p187));
        }

        s.b[1700] = (p.p184 == 0.5);
        s.v[1700] = if s.b[1700] { 1.0 } else { 0.0 };

        if ((((s.b[1656] && (!s.b[1688])) && s.b[1696]) && s.b[1699]) && s.b[1700]) {
            s.store_div_from_scalar_sqrt_ad(1684, 1.0, s.ad_value(1683));
        }

        if ((((s.b[1656] && (!s.b[1688])) && s.b[1696]) && s.b[1699]) && (!s.b[1700])) {
            s.store_powf(1684, 1683, (-p.p184));
        }

        if (((s.b[1656] && (!s.b[1688])) && s.b[1696]) && s.b[1699]) {
            s.store_add_ad_rhs(283, 283, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1671), 1.0, A::mul(s.ad_value(1683), s.ad_value(1684)), (p.p187 * 1.0 / ((1.0 - p.p184)))));
        }

        if ((s.b[1656] && (!s.b[1688])) && (!s.b[1696])) {
            s.store_add(1658, 1667, 1671);
            s.store_add_scaled_inputs(1659, 1667, (p.p182 * 1.0 / (p.p185)), 1671, (p.p184 * 1.0 / (p.p187)));
            s.store_mul_add_scaled_product_rhs(283, 1673, s.ad_value(1658), 1.0, s.ad_value(1673), s.ad_value(1659), 0.5);
        }

        s.b[1701] = (p.p4 > s.v[288]);
        s.v[1701] = if s.b[1701] { 1.0 } else { 0.0 };

        if (s.b[1656] && s.b[1701]) {
            s.store_offset_scaled(1668, 288, (-p.p180), ((p.p4) * (p.p180)));
            s.store_scale(1670, 288, p.p181);
        }

        s.b[1702] = (s.v[1672] < 0.0);
        s.v[1702] = if s.b[1702] { 1.0 } else { 0.0 };

        s.b[1703] = (s.v[1666] > 0.0);
        s.v[1703] = if s.b[1703] { 1.0 } else { 0.0 };

        if (((s.b[1656] && s.b[1701]) && s.b[1702]) && s.b[1703]) {
            s.store_sub_from_scalar_scaled_input(1683, 1.0, 1672, 1.0 / (p.p185));
        }

        s.b[1704] = (p.p182 == 0.5);
        s.v[1704] = if s.b[1704] { 1.0 } else { 0.0 };

        if ((((s.b[1656] && s.b[1701]) && s.b[1702]) && s.b[1703]) && s.b[1704]) {
            s.store_div_from_scalar_sqrt_ad(1684, 1.0, s.ad_value(1683));
        }

        if ((((s.b[1656] && s.b[1701]) && s.b[1702]) && s.b[1703]) && (!s.b[1704])) {
            s.store_powf(1684, 1683, (-p.p182));
        }

        if (((s.b[1656] && s.b[1701]) && s.b[1702]) && s.b[1703]) {
            s.store_mul_sub_from_scalar_ad_rhs_scaled_output(284, 1666, 1.0, A::mul(s.ad_value(1683), s.ad_value(1684)), (p.p185 * 1.0 / ((1.0 - p.p182))));
        }

    }

    pub(super) fn stamp_reactive_block_29(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (((s.b[1656] && s.b[1701]) && s.b[1702]) && (!s.b[1703])) {
            s.store_scalar(284, 0.0);
        }

        s.b[1705] = (s.v[1668] > 0.0);
        s.v[1705] = if s.b[1705] { 1.0 } else { 0.0 };

        if (((s.b[1656] && s.b[1701]) && s.b[1702]) && s.b[1705]) {
            s.store_sub_from_scalar_scaled_input(1683, 1.0, 1672, 1.0 / (p.p186));
        }

        s.b[1706] = (p.p183 == 0.5);
        s.v[1706] = if s.b[1706] { 1.0 } else { 0.0 };

        if ((((s.b[1656] && s.b[1701]) && s.b[1702]) && s.b[1705]) && s.b[1706]) {
            s.store_div_from_scalar_sqrt_ad(1684, 1.0, s.ad_value(1683));
        }

        if ((((s.b[1656] && s.b[1701]) && s.b[1702]) && s.b[1705]) && (!s.b[1706])) {
            s.store_powf(1684, 1683, (-p.p183));
        }

        if (((s.b[1656] && s.b[1701]) && s.b[1702]) && s.b[1705]) {
            s.store_add_ad_rhs(284, 284, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1668), 1.0, A::mul(s.ad_value(1683), s.ad_value(1684)), (p.p186 * 1.0 / ((1.0 - p.p183)))));
        }

        s.b[1707] = (s.v[1670] > 0.0);
        s.v[1707] = if s.b[1707] { 1.0 } else { 0.0 };

        if (((s.b[1656] && s.b[1701]) && s.b[1702]) && s.b[1707]) {
            s.store_sub_from_scalar_scaled_input(1683, 1.0, 1672, 1.0 / (p.p187));
        }

        s.b[1708] = (p.p184 == 0.5);
        s.v[1708] = if s.b[1708] { 1.0 } else { 0.0 };

        if ((((s.b[1656] && s.b[1701]) && s.b[1702]) && s.b[1707]) && s.b[1708]) {
            s.store_div_from_scalar_sqrt_ad(1684, 1.0, s.ad_value(1683));
        }

        if ((((s.b[1656] && s.b[1701]) && s.b[1702]) && s.b[1707]) && (!s.b[1708])) {
            s.store_powf(1684, 1683, (-p.p184));
        }

        if (((s.b[1656] && s.b[1701]) && s.b[1702]) && s.b[1707]) {
            s.store_add_ad_rhs(284, 284, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1670), 1.0, A::mul(s.ad_value(1683), s.ad_value(1684)), (p.p187 * 1.0 / ((1.0 - p.p184)))));
        }

        if ((s.b[1656] && s.b[1701]) && (!s.b[1702])) {
            s.store_add_scaled_inputs3_indices(1658, 1666, 1.0, 1668, 1.0, 1670, 1.0);
            s.store_add_scaled_inputs3_indices(1659, 1666, (p.p182 * 1.0 / (p.p185)), 1668, (p.p183 * 1.0 / (p.p186)), 1670, (p.p184 * 1.0 / (p.p187)));
            s.store_mul_add_scaled_product_rhs(284, 1672, s.ad_value(1658), 1.0, s.ad_value(1672), s.ad_value(1659), 0.5);
        }

        if (s.b[1656] && (!s.b[1701])) {
            s.store_scalar(1670, (p.p181 * p.p4));
        }

        s.b[1709] = (s.v[1672] < 0.0);
        s.v[1709] = if s.b[1709] { 1.0 } else { 0.0 };

        s.b[1710] = (s.v[1666] > 0.0);
        s.v[1710] = if s.b[1710] { 1.0 } else { 0.0 };

        if (((s.b[1656] && (!s.b[1701])) && s.b[1709]) && s.b[1710]) {
            s.store_sub_from_scalar_scaled_input(1683, 1.0, 1672, 1.0 / (p.p185));
        }

        s.b[1711] = (p.p182 == 0.5);
        s.v[1711] = if s.b[1711] { 1.0 } else { 0.0 };

        if ((((s.b[1656] && (!s.b[1701])) && s.b[1709]) && s.b[1710]) && s.b[1711]) {
            s.store_div_from_scalar_sqrt_ad(1684, 1.0, s.ad_value(1683));
        }

        if ((((s.b[1656] && (!s.b[1701])) && s.b[1709]) && s.b[1710]) && (!s.b[1711])) {
            s.store_powf(1684, 1683, (-p.p182));
        }

        if (((s.b[1656] && (!s.b[1701])) && s.b[1709]) && s.b[1710]) {
            s.store_mul_sub_from_scalar_ad_rhs_scaled_output(284, 1666, 1.0, A::mul(s.ad_value(1683), s.ad_value(1684)), (p.p185 * 1.0 / ((1.0 - p.p182))));
        }

        if (((s.b[1656] && (!s.b[1701])) && s.b[1709]) && (!s.b[1710])) {
            s.store_scalar(284, 0.0);
        }

        s.b[1712] = (s.v[1670] > 0.0);
        s.v[1712] = if s.b[1712] { 1.0 } else { 0.0 };

        if (((s.b[1656] && (!s.b[1701])) && s.b[1709]) && s.b[1712]) {
            s.store_sub_from_scalar_scaled_input(1683, 1.0, 1672, 1.0 / (p.p187));
        }

        s.b[1713] = (p.p184 == 0.5);
        s.v[1713] = if s.b[1713] { 1.0 } else { 0.0 };

        if ((((s.b[1656] && (!s.b[1701])) && s.b[1709]) && s.b[1712]) && s.b[1713]) {
            s.store_div_from_scalar_sqrt_ad(1684, 1.0, s.ad_value(1683));
        }

        if ((((s.b[1656] && (!s.b[1701])) && s.b[1709]) && s.b[1712]) && (!s.b[1713])) {
            s.store_powf(1684, 1683, (-p.p184));
        }

        if (((s.b[1656] && (!s.b[1701])) && s.b[1709]) && s.b[1712]) {
            s.store_add_ad_rhs(284, 284, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1670), 1.0, A::mul(s.ad_value(1683), s.ad_value(1684)), (p.p187 * 1.0 / ((1.0 - p.p184)))));
        }

        if ((s.b[1656] && (!s.b[1701])) && (!s.b[1709])) {
            s.store_add(1658, 1666, 1670);
            s.store_add_scaled_inputs(1659, 1666, (p.p182 * 1.0 / (p.p185)), 1670, (p.p184 * 1.0 / (p.p187)));
            s.store_mul_add_scaled_product_rhs(284, 1672, s.ad_value(1658), 1.0, s.ad_value(1672), s.ad_value(1659), 0.5);
        }

        s.b[1714] = (s.v[1667] > 0.0);
        s.v[1714] = if s.b[1714] { 1.0 } else { 0.0 };

        if (s.b[1656] && s.b[1714]) {
            s.store_scaled_mul(1661, 544, 1665, ((-1.6021918e-19) * p.p3));
            s.store_scale(1663, 1661, (-0.001));
            s.store_add_scaled_inputs3_indices(44, 1661, -1.0, 283, 1.0, 1663, -1.0);
            s.store_scaled_mul(45, 1661, 1663, (-4.0));
        }

        if (s.b[1656] && s.b[1714]) {
            if (s.v[45] > 0.0) {
            } else {
                s.store_neg(45, 45);
            }
        }

        if (s.b[1656] && s.b[1714]) {
            s.store_sqrt_square_add(45, 44, 45);
            s.store_add_scaled_inputs3_indices(283, 1661, -1.0, 44, (-0.5), 45, (-0.5));
            s.store_scale(283, 283, (-1.0));
        }

        s.b[1715] = (s.v[1666] > 0.0);
        s.v[1715] = if s.b[1715] { 1.0 } else { 0.0 };

        if (s.b[1656] && s.b[1715]) {
            s.store_scaled_mul(1662, 544, 1665, ((-1.6021918e-19) * p.p2));
            s.store_scale(1664, 1662, (-0.001));
            s.store_add_scaled_inputs3_indices(44, 1662, -1.0, 284, 1.0, 1664, -1.0);
            s.store_scaled_mul(45, 1662, 1664, (-4.0));
        }

        if (s.b[1656] && s.b[1715]) {
            if (s.v[45] > 0.0) {
            } else {
                s.store_neg(45, 45);
            }
        }

        if (s.b[1656] && s.b[1715]) {
            s.store_sqrt_square_add(45, 44, 45);
            s.store_add_scaled_inputs3_indices(284, 1662, -1.0, 44, (-0.5), 45, (-0.5));
            s.store_scale(284, 284, (-1.0));
        }

        s.b[1748] = ((p.p32 != 0.0) && (s.v[145] == 0.0));
        s.v[1748] = if s.b[1748] { 1.0 } else { 0.0 };

        if s.b[1748] {
            s.store_div_scaled_inputs2_indices(1731, 314, 1.0, 161, (-1.0), 441, 1.0);
            s.store_scaled_mul(1732, 251, 1731, 1e-5);
        }

        s.b[1749] = (((1.0 - (10.0 * 2.220446049250313e-16)) <= p.p113) && (p.p113 <= (1.0 + (10.0 * 2.220446049250313e-16))));
        s.v[1749] = if s.b[1749] { 1.0 } else { 0.0 };

        if (s.b[1748] && s.b[1749]) {
            s.store_scalar(1733, 1.0);
        }

        s.b[1750] = (((2.0 - (10.0 * 2.220446049250313e-16)) <= p.p113) && (p.p113 <= (2.0 + (10.0 * 2.220446049250313e-16))));
        s.v[1750] = if s.b[1750] { 1.0 } else { 0.0 };

        if ((s.b[1748] && (!s.b[1749])) && s.b[1750]) {
            s.copy_ad(1733, 1732);
        }

        if ((s.b[1748] && (!s.b[1749])) && (!s.b[1750])) {
            s.store_powf(1733, 1732, (p.p113 - 1.0));
        }

        if s.b[1748] {
            s.store_mul(1734, 1732, 1733);
            s.store_offset(1735, 1734, 1.0);
            s.store_powf(1736, 1735, (((-1.0) / p.p113) - 1.0));
            s.store_mul(1737, 1735, 1736);
            s.store_mul(293, 251, 1737);
            s.store_scaled_add(1739, 250, 293, 0.5);
            s.store_square(1738, 190);
        }

        if s.b[1748] {
            let assign33750_ad_e48938: A = A::add_scaled_inputs3(A::mul3(A::add_scaled_inputs(A::scale_offset(s.ad_value(190), 3.0, 1.0), 1.0, s.ad_value(1738), 6.0), s.ad_value(293), s.ad_value(293)), 1.0, A::mul3(A::add_scaled_inputs(A::scale_offset(s.ad_value(190), 4.0, 3.0), 1.0, s.ad_value(1738), 3.0), s.ad_value(293), s.ad_value(250)), 1.0, A::mul3(A::add(A::scale_offset(s.ad_value(190), 3.0, 6.0), s.ad_value(1738)), s.ad_value(250), s.ad_value(250)), 1.0);
            s.store_div_scaled_product3_by_product(292, A::mul3(s.ad_value(107), s.ad_value(323), s.ad_value(192)), s.ad_value(250), assign33750_ad_e48938, 1.0, A::mul3_scaled_output(s.ad_value(441), A::offset(s.ad_value(190), 1.0), s.ad_value(1739), 15.0), s.ad_value(1739), 1.0);
        }

        if (!s.b[1748]) {
            s.store_scalar(292, 0.0);
        }

        s.b[1751] = ((((p.p30 != 0.0) && (p.p32 != 0.0)) && (s.v[146] == 1.0)) && (s.v[145] == 0.0));
        s.v[1751] = if s.b[1751] { 1.0 } else { 0.0 };

        if s.b[1751] {
            s.store_sqrt(298, 296);
            s.store_add(1740, 192, 298);
            s.store_square(1741, 294);
            s.store_square(1742, 296);
            s.store_scaled_mul(1743, 294, 296, 42.0);
            s.store_add_scaled_inputs3_indices(1743, 1743, 1.0, 1741, 4.0, 1742, 4.0);
            s.store_add_ad_rhs(1743, 1743, A::mul3_scaled_output(s.ad_value(298), s.ad_value(192), A::add(s.ad_value(294), s.ad_value(296)), 20.0));
            s.store_square(1744, 1740);
            s.store_square(1736, 1744);
            s.store_div_ad_rhs(299, 1743, A::mul(s.ad_value(1736), s.ad_value(1740)));
            s.store_mul_ad_product_lhs(300, A::div(s.ad_value(107), s.ad_value(441)), s.ad_value(250), 323);
        }

        s.store_add(199, 199, 265);

        s.b[1752] = (p.p43 == 1.0);
        s.v[1752] = if s.b[1752] { 1.0 } else { 0.0 };

        if s.b[1752] {
            s.store_add(271, 531, 532);
        }

        if (s.b[1752] && s.b[564]) {
            s.store_offset(271, 271, (-(p.p168 * s.v[99])));
        }

        if s.b[1752] {
            s.store_mul_sub_scaled_inputs_rhs(272, 271, s.ad_value(158), -1.0, s.ad_value(513), -1.0);
            s.store_scalar(276, 0.0);
            s.store_mul_scaled_offset_rhs(274, 276, p.p9, 518, s.v[101]);
            s.store_mul_scaled_offset_rhs(275, 276, p.p9, 519, s.v[101]);
            s.store_mul_sub_rhs(277, 274, 158, 157);
            s.store_mul(278, 275, 158);
            s.store_mul_sub_scaled_inputs_rhs(279, 276, s.ad_value(158), (p.p19 * p.p9), s.ad_value(513), (p.p19 * p.p9));
            s.store_add(268, 268, 277);
            s.store_add(267, 267, 278);
            s.store_add(272, 272, 279);
        }

        if ((!s.b[1752]) && s.b[564]) {
            s.store_scalar(271, ((-p.p168) * s.v[99]));
            s.store_mul_sub_scaled_inputs_rhs(272, 271, s.ad_value(158), -1.0, s.ad_value(513), -1.0);
        }

        if ((!s.b[1752]) && (!s.b[564])) {
            s.store_scalar(271, 0.0);
            s.store_scalar(272, 0.0);
        }

        if (!s.b[1752]) {
            s.store_scalar(273, 0.0);
            s.copy_ad(274, 273);
            s.copy_ad(275, 273);
            s.store_mul_sub_rhs(277, 274, 158, 157);
            s.store_mul(278, 275, 158);
            s.store_add(268, 268, 277);
            s.store_add(267, 267, 278);
        }

        s.store_scale(9, 199, s.v[451]);

        if (s.v[85] != 0.0) {
            s.store_scalar(24, 0.0);
            s.store_scalar(23, 0.0);
        }

        s.b[1753] = (p.p43 == 1.0);
        s.v[1753] = if s.b[1753] { 1.0 } else { 0.0 };

        if ((s.v[85] != 0.0) && s.b[1753]) {
            s.store_scalar(25, 0.0);
            s.copy_ad(556, 438);
        }

        if ((s.v[85] != 0.0) && (!s.b[1753])) {
            s.store_scalar(554, 0.0);
        }

        s.b[1754] = (p.p43 == 1.0);
        s.v[1754] = if s.b[1754] { 1.0 } else { 0.0 };

        if ((s.v[85] == 0.0) && s.b[1754]) {
            s.store_sub_scaled_inputs(23, 196, (-s.v[451]), 197, s.v[451]);
            s.store_scale(24, 198, s.v[451]);
            s.store_scaled_sub(25, 197, 198, s.v[451]);
        }

        if ((s.v[85] == 0.0) && (!s.b[1754])) {
            s.store_add_scaled_inputs4_indices(23, 392, (-s.v[451]), 197, ((-1.0) * s.v[451]), 476, (-s.v[451]), 477, (-s.v[451]));
            s.store_scaled_add(24, 198, 477, s.v[451]);
            s.store_add_scaled_inputs3_indices(25, 197, s.v[451], 198, ((-1.0) * s.v[451]), 476, s.v[451]);
        }

        s.b[1760] = (p.p64 == 0.0);
        s.v[1760] = if s.b[1760] { 1.0 } else { 0.0 };

        if s.b[1760] {
            s.store_scalar(280, 0.0);
        }

        if (!s.b[1760]) {
            s.store_add_scaled_inputs(1755, 315, s.v[97], 161, 1.0);
        }

        s.b[1761] = (s.v[1755] > s.v[314]);
        s.v[1761] = if s.b[1761] { 1.0 } else { 0.0 };

        if ((!s.b[1760]) && s.b[1761]) {
            s.copy_ad(1755, 314);
        }

        if (!s.b[1760]) {
            s.store_add_scaled_inputs3_indices(1756, 157, s.v[317], 161, s.v[317], 1755, (1.0 - s.v[317]));
            s.store_sqrt_div_from_scalar_ad(1757, (2.0 * 1.034943e-10), s.ad_value(229));
            s.store_scale(1758, 1757, 1.3);
            s.store_scaled_mul(1759, 108, 1758, 1.034943e-10);
            s.store_mul_ad_lhs(280, A::add_scaled_inputs4(s.ad_value(161), 1.0 / (p.p64), s.ad_value(157), 1.0 / (p.p64), s.ad_value(1756), (-1.0 / (p.p64)), s.ad_value(315), -1.0), 1759);
        }

        s.b[1762] = (p.p65 != 0.0);
        s.v[1762] = if s.b[1762] { 1.0 } else { 0.0 };

    }

    pub(super) fn stamp_reactive_block_30(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        if s.b[1762] {
            s.store_add_scaled_product_indices(280, 280, 1.0, 135, 513, 1.0);
        }

        s.b[1763] = (p.p24 == 1.0);
        s.v[1763] = if s.b[1763] { 1.0 } else { 0.0 };

        s.b[1764] = (p.p43 == 1.0);
        s.v[1764] = if s.b[1764] { 1.0 } else { 0.0 };

        if (s.b[1763] && s.b[1764]) {
            s.store_add_scaled_inputs4_indices(471, 463, -1.0, 464, (-1.0), 467, -1.0, 468, -1.0);
            s.store_add(472, 466, 470);
            s.store_add(473, 465, 469);
            s.store_add_ad_rhs(23, 23, A::add_scaled_inputs(A::sub(A::sub(A::add_scaled_inputs4(s.ad_value(268), 1.0, s.ad_value(267), 1.0, s.ad_value(272), 1.0, s.ad_value(280), -1.0), s.ad_value(455)), s.ad_value(454)), s.v[451], s.ad_value(471), s.v[451]));
            s.store_add_ad_rhs(24, 24, A::add_scaled_inputs4(s.ad_value(280), s.v[451], s.ad_value(268), ((-1.0) * s.v[451]), s.ad_value(456), s.v[451], s.ad_value(472), s.v[451]));
            s.store_add_scaled_inputs4_indices(25, 25, 1.0, 457, s.v[451], 267, ((-1.0) * s.v[451]), 473, s.v[451]);
        }

        if (s.b[1763] && (!s.b[1764])) {
            s.store_add_ad_rhs(23, 23, A::sub_scaled_inputs(A::sub(A::add_scaled_inputs4(s.ad_value(268), 1.0, s.ad_value(267), 1.0, s.ad_value(272), 1.0, s.ad_value(280), -1.0), s.ad_value(455)), s.v[451], s.ad_value(454), s.v[451]));
            s.store_add_scaled_inputs4_indices(24, 24, 1.0, 280, s.v[451], 268, ((-1.0) * s.v[451]), 456, s.v[451]);
            s.store_add_scaled_inputs3_indices(25, 25, 1.0, 457, s.v[451], 267, (-s.v[451]));
        }

        s.b[1765] = (p.p43 == 1.0);
        s.v[1765] = if s.b[1765] { 1.0 } else { 0.0 };

        if s.b[1765] {
            s.store_scale(36, 281, s.v[451]);
            s.store_scale(35, 282, s.v[451]);
            s.store_scale(560, 284, s.v[451]);
            s.store_scale(561, 283, s.v[451]);
        }

        if (!s.b[1765]) {
            s.store_scalar(36, 0.0);
            s.store_scalar(35, 0.0);
            s.store_scalar(560, 0.0);
            s.store_scalar(561, 0.0);
        }

        s.b[1766] = (p.p25 != 1.0);
        s.v[1766] = if s.b[1766] { 1.0 } else { 0.0 };

        if s.b[1766] {
            s.store_scalar(557, 0.0);
        }

        if (!s.b[1766]) {
            s.store_scale(557, 263, s.v[451]);
        }

        s.store_scale(598, 292, s.v[451]);

        s.store_scalar(27, A::ddx_projection(&s.ad_value(23), Some(6), None));

        s.store_scale(27, 27, p.p50);

        s.store_scalar(28, A::ddx_projection(&s.ad_value(23), Some(7), None));

        s.store_scale(28, 28, p.p50);

        if (s.v[613] > 0.0) {
            s.copy_ad(555, 28);
        } else {
            s.copy_ad(555, 27);
        }

        s.b[1775] = ((((p.p30 != 0.0) && (p.p32 != 0.0)) && (s.v[146] == 1.0)) && (s.v[145] == 0.0));
        s.v[1775] = if s.b[1775] { 1.0 } else { 0.0 };

        if s.b[1775] {
            s.store_scaled_mul(1769, 323, 108, (1e-6 * s.v[98]));
            s.store_scale(1770, 555, 1.0 / (s.v[451]));
            s.store_div_scaled_product3_indices(1771, 227, 1770, 1770, (0.1185185185185185 * 1.6021918e-19), 300, 1.0);
        }

        s.b[1776] = ((s.v[297] > (10.0 * 2.220446049250313e-16)) && (s.v[157] > (10.0 * 2.220446049250313e-16)));
        s.v[1776] = if s.b[1776] { 1.0 } else { 0.0 };

        if (s.b[1775] && s.b[1776]) {
            s.store_div(1772, 251, 250);
            s.store_div_scaled_inputs2_mixed_aii(1773, A::div(s.ad_value(251), s.ad_value(293)), 1.0, 1772, (-1.0), 157, 1.0);
            s.store_add_ad_rhs(1774, 1772, A::div_scaled_product(s.ad_value(1773), A::add(A::add_scaled_product(s.ad_value(294), 1.0, s.ad_value(192), s.ad_value(298), 1.0), s.ad_value(296)), 0.6666666666666667, A::add(s.ad_value(192), s.ad_value(298)), 1.0));
        }

        if (s.b[1775] && (!s.b[1776])) {
            s.store_div(1774, 251, 293);
        }

        if s.b[1775] {
            s.store_mul3_affine_lhs(558, 1771, 299, s.v[451], 0.0, 1774);
        }

        if s.b[1775] {
            if (((-s.v[1770]) > s.v[1769]) && (s.v[558] > 0.0)) {
            } else {
                s.store_scalar(558, 0.0);
            }
        }

        if (!s.b[1775]) {
            s.store_scalar(558, 0.0);
        }

        s.b[1777] = (p.p259 == 1.0);
        s.v[1777] = if s.b[1777] { 1.0 } else { 0.0 };

        if s.b[1777] {
            s.store_scalar(3, 1.0);
        }

        s.b[1797] = (s.v[3] == 1.0);
        s.v[1797] = if s.b[1797] { 1.0 } else { 0.0 };

        if (s.b[1777] && s.b[1797]) {
            s.store_scalar(1781, p.p266);
            s.store_scalar(1782, p.p268);
            s.store_scalar(1783, p.p273);
            s.store_scalar(1787, p.p258);
            s.store_scaled_voltage(1785, ctx, nodes, Some(7), Some(2), p.p50);
        }

        if (s.b[1777] && (!s.b[1797])) {
            s.store_scalar(1781, p.p265);
            s.store_scalar(1782, p.p267);
            s.store_scalar(1783, p.p272);
            s.store_scalar(1787, p.p257);
            s.store_scaled_voltage(1785, ctx, nodes, Some(0), Some(6), p.p50);
        }

        if s.b[1777] {
            s.store_scale(1781, 1781, 0.0001);
            s.store_scale(1782, 1782, 0.01);
            s.store_scale(1786, 429, 1.0 / (s.v[81]));
            s.store_powf(328, 1786, p.p269);
            s.store_div(1789, 1781, 328);
            s.store_sub_ad(327, A::add_scaled_product(A::scale_offset(s.ad_value(1786), 0.4, 1.8), 1.0, s.ad_value(1786), s.ad_value(1786), 0.1), A::scale_offset(s.ad_value(1786), (-p.p270), p.p270));
            s.store_div(1790, 1782, 327);
            s.store_add_ad_rhs(1783, 1783, A::scaled_offset(s.ad_value(429), (-s.v[81]), p.p274));
            s.store_scalar(1778, (1.0 + (p.p279 / ((s.v[100]) as f64).powf(p.p280))));
            s.store_scalar(1780, (1.0 + (p.p277 / ((s.v[100]) as f64).powf(p.p278))));
            s.store_scalar(1779, (1.0 + (p.p275 / ((s.v[109]) as f64).powf(p.p276))));
            s.store_mul(1789, 1789, 1778);
            s.store_offset_product3(1790, s.ad_value(1790), s.ad_value(1779), s.ad_value(1780), 1.0, 1e-50);
            s.store_div(1791, 1785, 1787);
            s.store_mul(1792, 1789, 1791);
        }

        s.b[1798] = (s.v[1785] >= 0.0);
        s.v[1798] = if s.b[1798] { 1.0 } else { 0.0 };

        if (s.b[1777] && s.b[1798]) {
            s.store_div(328, 1792, 1790);
        }

        if (s.b[1777] && (!s.b[1798])) {
            s.store_div_scaled_inputs_indices(328, 1792, -1.0, 1790, 1.0);
        }

        s.b[1799] = (((1.0 - (10.0 * 2.220446049250313e-16)) <= s.v[1783]) && (s.v[1783] <= (1.0 + (10.0 * 2.220446049250313e-16))));
        s.v[1799] = if s.b[1799] { 1.0 } else { 0.0 };

        if (s.b[1777] && s.b[1799]) {
            s.store_scalar(330, 1.0);
        }

        s.b[1800] = (((2.0 - (10.0 * 2.220446049250313e-16)) <= s.v[1783]) && (s.v[1783] <= (2.0 + (10.0 * 2.220446049250313e-16))));
        s.v[1800] = if s.b[1800] { 1.0 } else { 0.0 };

        if ((s.b[1777] && (!s.b[1799])) && s.b[1800]) {
            s.copy_ad(330, 328);
        }

        if ((s.b[1777] && (!s.b[1799])) && (!s.b[1800])) {
            s.store_pow_ad(330, s.ad_value(328), A::offset(s.ad_value(1783), (-1.0)));
        }

        if s.b[1777] {
            s.store_mul(329, 328, 330);
            s.store_offset(331, 329, 1.0);
        }

        s.b[1801] = (((1.0 - (10.0 * 2.220446049250313e-16)) <= s.v[1783]) && (s.v[1783] <= (1.0 + (10.0 * 2.220446049250313e-16))));
        s.v[1801] = if s.b[1801] { 1.0 } else { 0.0 };

        if (s.b[1777] && s.b[1801]) {
            s.store_div_from_scalar(332, 1.0, 331);
        }

        s.b[1802] = (((2.0 - (10.0 * 2.220446049250313e-16)) <= s.v[1783]) && (s.v[1783] <= (2.0 + (10.0 * 2.220446049250313e-16))));
        s.v[1802] = if s.b[1802] { 1.0 } else { 0.0 };

        if ((s.b[1777] && (!s.b[1801])) && s.b[1802]) {
            s.store_div_from_scalar_sqrt_ad(332, 1.0, s.ad_value(331));
        }

        if ((s.b[1777] && (!s.b[1801])) && (!s.b[1802])) {
            s.store_pow_ad(333, s.ad_value(331), A::offset(A::div_from_scalar((-1.0), s.ad_value(1783)), (-1.0)));
            s.store_mul(332, 331, 333);
        }

        if s.b[1777] {
            s.store_div_from_scalar(328, 1.6021918e-19, 1787);
        }

        s.b[1805] = (p.p260 == 1.0);
        s.v[1805] = if s.b[1805] { 1.0 } else { 0.0 };

        if s.b[1805] {
            s.store_scalar(3, 2.0);
        }

        s.b[1825] = (s.v[3] == 1.0);
        s.v[1825] = if s.b[1825] { 1.0 } else { 0.0 };

        if (s.b[1805] && s.b[1825]) {
            s.store_scalar(1809, p.p266);
            s.store_scalar(1810, p.p268);
            s.store_scalar(1811, p.p273);
            s.store_scalar(1815, p.p258);
            s.store_scaled_voltage(1813, ctx, nodes, Some(7), Some(2), p.p50);
        }

        if (s.b[1805] && (!s.b[1825])) {
            s.store_scalar(1809, p.p265);
            s.store_scalar(1810, p.p267);
            s.store_scalar(1811, p.p272);
            s.store_scalar(1815, p.p257);
            s.store_scaled_voltage(1813, ctx, nodes, Some(0), Some(6), p.p50);
        }

        if s.b[1805] {
            s.store_scale(1809, 1809, 0.0001);
            s.store_scale(1810, 1810, 0.01);
            s.store_scale(1814, 429, 1.0 / (s.v[81]));
            s.store_powf(328, 1814, p.p269);
            s.store_div(1817, 1809, 328);
            s.store_sub_ad(327, A::add_scaled_product(A::scale_offset(s.ad_value(1814), 0.4, 1.8), 1.0, s.ad_value(1814), s.ad_value(1814), 0.1), A::scale_offset(s.ad_value(1814), (-p.p270), p.p270));
            s.store_div(1818, 1810, 327);
            s.store_add_ad_rhs(1811, 1811, A::scaled_offset(s.ad_value(429), (-s.v[81]), p.p274));
            s.store_scalar(1806, (1.0 + (p.p279 / ((s.v[100]) as f64).powf(p.p280))));
            s.store_scalar(1808, (1.0 + (p.p277 / ((s.v[100]) as f64).powf(p.p278))));
            s.store_scalar(1807, (1.0 + (p.p275 / ((s.v[109]) as f64).powf(p.p276))));
            s.store_mul(1817, 1817, 1806);
            s.store_offset_product3(1818, s.ad_value(1818), s.ad_value(1807), s.ad_value(1808), 1.0, 1e-50);
            s.store_div(1819, 1813, 1815);
            s.store_mul(1820, 1817, 1819);
        }

        s.b[1826] = (s.v[1813] >= 0.0);
        s.v[1826] = if s.b[1826] { 1.0 } else { 0.0 };

        if (s.b[1805] && s.b[1826]) {
            s.store_div(328, 1820, 1818);
        }

        if (s.b[1805] && (!s.b[1826])) {
            s.store_div_scaled_inputs_indices(328, 1820, -1.0, 1818, 1.0);
        }

        s.b[1827] = (((1.0 - (10.0 * 2.220446049250313e-16)) <= s.v[1811]) && (s.v[1811] <= (1.0 + (10.0 * 2.220446049250313e-16))));
        s.v[1827] = if s.b[1827] { 1.0 } else { 0.0 };

        if (s.b[1805] && s.b[1827]) {
            s.store_scalar(330, 1.0);
        }

        s.b[1828] = (((2.0 - (10.0 * 2.220446049250313e-16)) <= s.v[1811]) && (s.v[1811] <= (2.0 + (10.0 * 2.220446049250313e-16))));
        s.v[1828] = if s.b[1828] { 1.0 } else { 0.0 };

        if ((s.b[1805] && (!s.b[1827])) && s.b[1828]) {
            s.copy_ad(330, 328);
        }

        if ((s.b[1805] && (!s.b[1827])) && (!s.b[1828])) {
            s.store_pow_ad(330, s.ad_value(328), A::offset(s.ad_value(1811), (-1.0)));
        }

        if s.b[1805] {
            s.store_mul(329, 328, 330);
            s.store_offset(331, 329, 1.0);
        }

        s.b[1829] = (((1.0 - (10.0 * 2.220446049250313e-16)) <= s.v[1811]) && (s.v[1811] <= (1.0 + (10.0 * 2.220446049250313e-16))));
        s.v[1829] = if s.b[1829] { 1.0 } else { 0.0 };

        if (s.b[1805] && s.b[1829]) {
            s.store_div_from_scalar(332, 1.0, 331);
        }

        s.b[1830] = (((2.0 - (10.0 * 2.220446049250313e-16)) <= s.v[1811]) && (s.v[1811] <= (2.0 + (10.0 * 2.220446049250313e-16))));
        s.v[1830] = if s.b[1830] { 1.0 } else { 0.0 };

        if ((s.b[1805] && (!s.b[1829])) && s.b[1830]) {
            s.store_div_from_scalar_sqrt_ad(332, 1.0, s.ad_value(331));
        }

        if ((s.b[1805] && (!s.b[1829])) && (!s.b[1830])) {
            s.store_pow_ad(333, s.ad_value(331), A::offset(A::div_from_scalar((-1.0), s.ad_value(1811)), (-1.0)));
            s.store_mul(332, 331, 333);
        }

        if s.b[1805] {
            s.store_div_from_scalar(328, 1.6021918e-19, 1815);
        }

        s.b[1833] = (p.p43 == 1.0);
        s.v[1833] = if s.b[1833] { 1.0 } else { 0.0 };

    }

    pub(super) fn stamp_reactive_block_31(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (s.b[1833] && (s.v[85] != 0.0)) {
            if (s.v[613] == 1.0) {
                s.copy_ad(438, 556);
            } else {
                s.store_sub_from_scalar(438, 1.0, 556);
            }
        }

        if (s.b[1833] && (s.v[85] != 0.0)) {
            s.store_add_scaled_product_indices(584, 473, 1.0, 580, 438, 1.0);
            s.store_add_ad_lhs(585, A::mul_sub_from_scalar_rhs(s.ad_value(580), 1.0, s.ad_value(438)), 473);
            s.store_add_scaled_inputs3_indices(586, 580, -1.0, 581, (-1.0), 471, 1.0);
        }

        if (s.b[1833] && (s.v[85] == 0.0)) {
            s.store_scalar(584, 0.0);
            s.store_scalar(585, 0.0);
            s.store_scalar(586, 0.0);
            s.store_scalar(581, 0.0);
        }

        if ((!s.b[1833]) && (s.v[85] != 0.0)) {
            s.store_add_scaled_inputs3_indices(586, 584, -1.0, 585, (-1.0), 581, -1.0);
        }

        if ((!s.b[1833]) && (s.v[85] == 0.0)) {
            s.store_scalar(584, 0.0);
            s.store_scalar(585, 0.0);
            s.store_scalar(586, 0.0);
            s.store_scalar(581, 0.0);
        }

        s.b[1838] = (s.v[613] == 1.0);
        s.v[1838] = if s.b[1838] { 1.0 } else { 0.0 };

        if s.b[1838] {
            s.copy_ad(199, 9);
            s.copy_ad(263, 557);
            s.store_add(594, 23, 586);
            s.store_add(198, 24, 584);
            s.store_add_scaled_inputs3_indices(554, 23, (-1.0), 24, (-1.0), 25, (-1.0));
            s.store_add(196, 554, 581);
        }

        if (!s.b[1838]) {
            s.store_neg(199, 9);
            s.store_scalar(263, 0.0);
            s.store_add(594, 23, 586);
            s.store_add(198, 25, 585);
            s.store_add_scaled_inputs3_indices(554, 23, (-1.0), 24, (-1.0), 25, (-1.0));
            s.store_add(196, 554, 581);
        }

        s.b[1839] = (p.p43 == 1.0);
        s.v[1839] = if s.b[1839] { 1.0 } else { 0.0 };

        if s.b[1839] {
            s.copy_ad(282, 35);
            s.copy_ad(284, 560);
            s.copy_ad(281, 36);
            s.copy_ad(283, 561);
        }

        s.b[1840] = ((p.p38 == 1.0) && (s.v[67] > 0.0));
        s.v[1840] = if s.b[1840] { 1.0 } else { 0.0 };

        if s.b[1840] {
            s.copy_ad(563, 542);
        }

        if (!s.b[1840]) {
            s.store_scalar(563, 0.0);
        }

        s.copy_ad(9, 199);

        s.store_scalar(27, A::ddx_projection(&s.ad_value(594), Some(6), None));

        s.store_scale(27, 27, p.p50);

        s.store_scalar(28, A::ddx_projection(&s.ad_value(594), Some(7), None));

        s.store_scale(28, 28, p.p50);

        s.b[1842] = (p.p43 == 1.0);
        s.v[1842] = if s.b[1842] { 1.0 } else { 0.0 };

        if s.b[1842] {
            s.store_scale(35, 282, p.p50);
            s.store_scale(36, 281, p.p50);
        }

        s.store_scale(610, 429, (4.0 * 1.3806226e-23));

        s.copy_ad(438, 439);

        s.store_mul(615, 610, 598);

        if ((s.v[615] > 0.0) && (s.v[558] > 0.0)) {
            s.store_sqrt_div(616, 558, 615);
        } else {
            s.store_scalar(616, 0.0);
        }

        if (s.v[613] > 0.0) {
            s.store_mul_sub_from_scalar_rhs(617, 616, 1.0, 438);
        } else {
            s.store_mul(617, 616, 438);
        }

        if (s.v[613] > 0.0) {
            s.store_mul(618, 616, 438);
        } else {
            s.store_mul_sub_from_scalar_rhs(618, 616, 1.0, 438);
        }

        s.b[1850] = ((p.p38 > 0.0) && (p.p242 > 0.0));
        s.v[1850] = if s.b[1850] { 1.0 } else { 0.0 };

        s.b[1851] = (p.p43 == 1.0);
        s.v[1851] = if s.b[1851] { 1.0 } else { 0.0 };

        s.b[1852] = ((p.p37 != 0.0) || ((p.p25 == 1.0) && (p.p26 == 2.0)));
        s.v[1852] = if s.b[1852] { 1.0 } else { 0.0 };

    }

    pub(super) fn stamp_transient_equations_block_0(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        multiplicity: f64,
        ddt_active: bool,
        ddt_scale: f64,
        ddt_state_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_initialized: &mut [bool; Instance::DDT_STATE_COUNT],
    ) {
        let nv0 = ctx.node_voltage(nodes[0]);
        let nv2 = ctx.node_voltage(nodes[2]);
        let nv6 = ctx.node_voltage(nodes[6]);
        let nv7 = ctx.node_voltage(nodes[7]);
        let eq3_e324: f64 = (p.p50 * s.v[199]);
        let eq3_e324_d_n0: f64 = (p.p50 * s.dn[199][0]);
        let eq3_e324_d_n1: f64 = (p.p50 * s.dn[199][1]);
        let eq3_e324_d_n2: f64 = (p.p50 * s.dn[199][2]);
        let eq3_e324_d_n3: f64 = (p.p50 * s.dn[199][3]);
        let eq3_e324_d_n4: f64 = (p.p50 * s.dn[199][4]);
        let eq3_e324_d_n5: f64 = (p.p50 * s.dn[199][5]);
        let eq3_e324_d_n6: f64 = (p.p50 * s.dn[199][6]);
        let eq3_e324_d_n7: f64 = (p.p50 * s.dn[199][7]);
        let eq3_e324_d_n8: f64 = (p.p50 * s.dn[199][8]);
        let eq3_e324_d_n9: f64 = (p.p50 * s.dn[199][9]);
        let eq3_e324_d_n10: f64 = (p.p50 * s.dn[199][10]);
        let eq3_e324_d_n11: f64 = (p.p50 * s.dn[199][11]);
        let eq3_e324_d_n12: f64 = (p.p50 * s.dn[199][12]);
        let eq3_e324_d_n13: f64 = (p.p50 * s.dn[199][13]);
        let eq3_e324_d_n14: f64 = (p.p50 * s.dn[199][14]);
        let eq3_e324_d_n15: f64 = (p.p50 * s.dn[199][15]);
        let eq3_e324_d_n16: f64 = (p.p50 * s.dn[199][16]);
        let eq3_e324_d_n17: f64 = (p.p50 * s.dn[199][17]);
        let eq3_e324_d_n18: f64 = (p.p50 * s.dn[199][18]);
        let eq3_e324_d_b0: f64 = (p.p50 * s.db[199][0]);
        let eq3_e324_d_b1: f64 = (p.p50 * s.db[199][1]);
        let eq3_e324_d_b2: f64 = (p.p50 * s.db[199][2]);
        let eq3_e324_d_b3: f64 = (p.p50 * s.db[199][3]);
        let eq3_e324_d_b4: f64 = (p.p50 * s.db[199][4]);
        let eq3_e324_d_b5: f64 = (p.p50 * s.db[199][5]);
        let eq3_e324_d_b6: f64 = (p.p50 * s.db[199][6]);
        let eq3_e324_d_b7: f64 = (p.p50 * s.db[199][7]);
        let eq3_e324_d_b8: f64 = (p.p50 * s.db[199][8]);
        let eq3_e324_d_b9: f64 = (p.p50 * s.db[199][9]);
        let eq3_e324_d_b10: f64 = (p.p50 * s.db[199][10]);
        let eq3_e324_d_b11: f64 = (p.p50 * s.db[199][11]);
        let eq3_e324_d_b12: f64 = (p.p50 * s.db[199][12]);
        let eq3_e324_d_b13: f64 = (p.p50 * s.db[199][13]);
        let eq3_e324_d_b14: f64 = (p.p50 * s.db[199][14]);
        let eq3_e324_d_b15: f64 = (p.p50 * s.db[199][15]);
        let eq3_value: f64 = eq3_e324;
        let eq3_node_derivatives: [f64; 19] = [eq3_e324_d_n0, eq3_e324_d_n1, eq3_e324_d_n2, eq3_e324_d_n3, eq3_e324_d_n4, eq3_e324_d_n5, eq3_e324_d_n6, eq3_e324_d_n7, eq3_e324_d_n8, eq3_e324_d_n9, eq3_e324_d_n10, eq3_e324_d_n11, eq3_e324_d_n12, eq3_e324_d_n13, eq3_e324_d_n14, eq3_e324_d_n15, eq3_e324_d_n16, eq3_e324_d_n17, eq3_e324_d_n18];
        let eq3_branch_derivatives: [f64; 16] = [eq3_e324_d_b0, eq3_e324_d_b1, eq3_e324_d_b2, eq3_e324_d_b3, eq3_e324_d_b4, eq3_e324_d_b5, eq3_e324_d_b6, eq3_e324_d_b7, eq3_e324_d_b8, eq3_e324_d_b9, eq3_e324_d_b10, eq3_e324_d_b11, eq3_e324_d_b12, eq3_e324_d_b13, eq3_e324_d_b14, eq3_e324_d_b15];
        stamper.stamp_current_dense_local(
            Some(6),
            Some(7),
            multiplicity * (eq3_value),
            &eq3_node_derivatives,
            &eq3_branch_derivatives,
            multiplicity,
        );
        let (eq4_e330, eq4_e330_d_n0, eq4_e330_d_n1, eq4_e330_d_n2, eq4_e330_d_n3, eq4_e330_d_n4, eq4_e330_d_n5, eq4_e330_d_n6, eq4_e330_d_n7, eq4_e330_d_n8, eq4_e330_d_n9, eq4_e330_d_n10, eq4_e330_d_n11, eq4_e330_d_n12, eq4_e330_d_n13, eq4_e330_d_n14, eq4_e330_d_n15, eq4_e330_d_n16, eq4_e330_d_n17, eq4_e330_d_n18, eq4_e330_d_b0, eq4_e330_d_b1, eq4_e330_d_b2, eq4_e330_d_b3, eq4_e330_d_b4, eq4_e330_d_b5, eq4_e330_d_b6, eq4_e330_d_b7, eq4_e330_d_b8, eq4_e330_d_b9, eq4_e330_d_b10, eq4_e330_d_b11, eq4_e330_d_b12, eq4_e330_d_b13, eq4_e330_d_b14, eq4_e330_d_b15,) = {
    if s.b[1848] {
        let eq4_e328: f64 = (p.p50 * s.v[306]);
        let eq4_e328_d_n0: f64 = (p.p50 * s.dn[306][0]);
        let eq4_e328_d_n1: f64 = (p.p50 * s.dn[306][1]);
        let eq4_e328_d_n2: f64 = (p.p50 * s.dn[306][2]);
        let eq4_e328_d_n3: f64 = (p.p50 * s.dn[306][3]);
        let eq4_e328_d_n4: f64 = (p.p50 * s.dn[306][4]);
        let eq4_e328_d_n5: f64 = (p.p50 * s.dn[306][5]);
        let eq4_e328_d_n6: f64 = (p.p50 * s.dn[306][6]);
        let eq4_e328_d_n7: f64 = (p.p50 * s.dn[306][7]);
        let eq4_e328_d_n8: f64 = (p.p50 * s.dn[306][8]);
        let eq4_e328_d_n9: f64 = (p.p50 * s.dn[306][9]);
        let eq4_e328_d_n10: f64 = (p.p50 * s.dn[306][10]);
        let eq4_e328_d_n11: f64 = (p.p50 * s.dn[306][11]);
        let eq4_e328_d_n12: f64 = (p.p50 * s.dn[306][12]);
        let eq4_e328_d_n13: f64 = (p.p50 * s.dn[306][13]);
        let eq4_e328_d_n14: f64 = (p.p50 * s.dn[306][14]);
        let eq4_e328_d_n15: f64 = (p.p50 * s.dn[306][15]);
        let eq4_e328_d_n16: f64 = (p.p50 * s.dn[306][16]);
        let eq4_e328_d_n17: f64 = (p.p50 * s.dn[306][17]);
        let eq4_e328_d_n18: f64 = (p.p50 * s.dn[306][18]);
        let eq4_e328_d_b0: f64 = (p.p50 * s.db[306][0]);
        let eq4_e328_d_b1: f64 = (p.p50 * s.db[306][1]);
        let eq4_e328_d_b2: f64 = (p.p50 * s.db[306][2]);
        let eq4_e328_d_b3: f64 = (p.p50 * s.db[306][3]);
        let eq4_e328_d_b4: f64 = (p.p50 * s.db[306][4]);
        let eq4_e328_d_b5: f64 = (p.p50 * s.db[306][5]);
        let eq4_e328_d_b6: f64 = (p.p50 * s.db[306][6]);
        let eq4_e328_d_b7: f64 = (p.p50 * s.db[306][7]);
        let eq4_e328_d_b8: f64 = (p.p50 * s.db[306][8]);
        let eq4_e328_d_b9: f64 = (p.p50 * s.db[306][9]);
        let eq4_e328_d_b10: f64 = (p.p50 * s.db[306][10]);
        let eq4_e328_d_b11: f64 = (p.p50 * s.db[306][11]);
        let eq4_e328_d_b12: f64 = (p.p50 * s.db[306][12]);
        let eq4_e328_d_b13: f64 = (p.p50 * s.db[306][13]);
        let eq4_e328_d_b14: f64 = (p.p50 * s.db[306][14]);
        let eq4_e328_d_b15: f64 = (p.p50 * s.db[306][15]);
        (eq4_e328, eq4_e328_d_n0, eq4_e328_d_n1, eq4_e328_d_n2, eq4_e328_d_n3, eq4_e328_d_n4, eq4_e328_d_n5, eq4_e328_d_n6, eq4_e328_d_n7, eq4_e328_d_n8, eq4_e328_d_n9, eq4_e328_d_n10, eq4_e328_d_n11, eq4_e328_d_n12, eq4_e328_d_n13, eq4_e328_d_n14, eq4_e328_d_n15, eq4_e328_d_n16, eq4_e328_d_n17, eq4_e328_d_n18, eq4_e328_d_b0, eq4_e328_d_b1, eq4_e328_d_b2, eq4_e328_d_b3, eq4_e328_d_b4, eq4_e328_d_b5, eq4_e328_d_b6, eq4_e328_d_b7, eq4_e328_d_b8, eq4_e328_d_b9, eq4_e328_d_b10, eq4_e328_d_b11, eq4_e328_d_b12, eq4_e328_d_b13, eq4_e328_d_b14, eq4_e328_d_b15,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq4_value: f64 = eq4_e330;
        let eq4_node_derivatives: [f64; 19] = [eq4_e330_d_n0, eq4_e330_d_n1, eq4_e330_d_n2, eq4_e330_d_n3, eq4_e330_d_n4, eq4_e330_d_n5, eq4_e330_d_n6, eq4_e330_d_n7, eq4_e330_d_n8, eq4_e330_d_n9, eq4_e330_d_n10, eq4_e330_d_n11, eq4_e330_d_n12, eq4_e330_d_n13, eq4_e330_d_n14, eq4_e330_d_n15, eq4_e330_d_n16, eq4_e330_d_n17, eq4_e330_d_n18];
        let eq4_branch_derivatives: [f64; 16] = [eq4_e330_d_b0, eq4_e330_d_b1, eq4_e330_d_b2, eq4_e330_d_b3, eq4_e330_d_b4, eq4_e330_d_b5, eq4_e330_d_b6, eq4_e330_d_b7, eq4_e330_d_b8, eq4_e330_d_b9, eq4_e330_d_b10, eq4_e330_d_b11, eq4_e330_d_b12, eq4_e330_d_b13, eq4_e330_d_b14, eq4_e330_d_b15];
        stamper.stamp_current_dense_local(
            Some(11),
            Some(7),
            multiplicity * (eq4_value),
            &eq4_node_derivatives,
            &eq4_branch_derivatives,
            multiplicity,
        );
        let (eq5_e336, eq5_e336_d_n0, eq5_e336_d_n1, eq5_e336_d_n2, eq5_e336_d_n3, eq5_e336_d_n4, eq5_e336_d_n5, eq5_e336_d_n6, eq5_e336_d_n7, eq5_e336_d_n8, eq5_e336_d_n9, eq5_e336_d_n10, eq5_e336_d_n11, eq5_e336_d_n12, eq5_e336_d_n13, eq5_e336_d_n14, eq5_e336_d_n15, eq5_e336_d_n16, eq5_e336_d_n17, eq5_e336_d_n18, eq5_e336_d_b0, eq5_e336_d_b1, eq5_e336_d_b2, eq5_e336_d_b3, eq5_e336_d_b4, eq5_e336_d_b5, eq5_e336_d_b6, eq5_e336_d_b7, eq5_e336_d_b8, eq5_e336_d_b9, eq5_e336_d_b10, eq5_e336_d_b11, eq5_e336_d_b12, eq5_e336_d_b13, eq5_e336_d_b14, eq5_e336_d_b15,) = {
    if s.b[1848] {
        let eq5_e334: f64 = (p.p50 * s.v[307]);
        let eq5_e334_d_n0: f64 = (p.p50 * s.dn[307][0]);
        let eq5_e334_d_n1: f64 = (p.p50 * s.dn[307][1]);
        let eq5_e334_d_n2: f64 = (p.p50 * s.dn[307][2]);
        let eq5_e334_d_n3: f64 = (p.p50 * s.dn[307][3]);
        let eq5_e334_d_n4: f64 = (p.p50 * s.dn[307][4]);
        let eq5_e334_d_n5: f64 = (p.p50 * s.dn[307][5]);
        let eq5_e334_d_n6: f64 = (p.p50 * s.dn[307][6]);
        let eq5_e334_d_n7: f64 = (p.p50 * s.dn[307][7]);
        let eq5_e334_d_n8: f64 = (p.p50 * s.dn[307][8]);
        let eq5_e334_d_n9: f64 = (p.p50 * s.dn[307][9]);
        let eq5_e334_d_n10: f64 = (p.p50 * s.dn[307][10]);
        let eq5_e334_d_n11: f64 = (p.p50 * s.dn[307][11]);
        let eq5_e334_d_n12: f64 = (p.p50 * s.dn[307][12]);
        let eq5_e334_d_n13: f64 = (p.p50 * s.dn[307][13]);
        let eq5_e334_d_n14: f64 = (p.p50 * s.dn[307][14]);
        let eq5_e334_d_n15: f64 = (p.p50 * s.dn[307][15]);
        let eq5_e334_d_n16: f64 = (p.p50 * s.dn[307][16]);
        let eq5_e334_d_n17: f64 = (p.p50 * s.dn[307][17]);
        let eq5_e334_d_n18: f64 = (p.p50 * s.dn[307][18]);
        let eq5_e334_d_b0: f64 = (p.p50 * s.db[307][0]);
        let eq5_e334_d_b1: f64 = (p.p50 * s.db[307][1]);
        let eq5_e334_d_b2: f64 = (p.p50 * s.db[307][2]);
        let eq5_e334_d_b3: f64 = (p.p50 * s.db[307][3]);
        let eq5_e334_d_b4: f64 = (p.p50 * s.db[307][4]);
        let eq5_e334_d_b5: f64 = (p.p50 * s.db[307][5]);
        let eq5_e334_d_b6: f64 = (p.p50 * s.db[307][6]);
        let eq5_e334_d_b7: f64 = (p.p50 * s.db[307][7]);
        let eq5_e334_d_b8: f64 = (p.p50 * s.db[307][8]);
        let eq5_e334_d_b9: f64 = (p.p50 * s.db[307][9]);
        let eq5_e334_d_b10: f64 = (p.p50 * s.db[307][10]);
        let eq5_e334_d_b11: f64 = (p.p50 * s.db[307][11]);
        let eq5_e334_d_b12: f64 = (p.p50 * s.db[307][12]);
        let eq5_e334_d_b13: f64 = (p.p50 * s.db[307][13]);
        let eq5_e334_d_b14: f64 = (p.p50 * s.db[307][14]);
        let eq5_e334_d_b15: f64 = (p.p50 * s.db[307][15]);
        (eq5_e334, eq5_e334_d_n0, eq5_e334_d_n1, eq5_e334_d_n2, eq5_e334_d_n3, eq5_e334_d_n4, eq5_e334_d_n5, eq5_e334_d_n6, eq5_e334_d_n7, eq5_e334_d_n8, eq5_e334_d_n9, eq5_e334_d_n10, eq5_e334_d_n11, eq5_e334_d_n12, eq5_e334_d_n13, eq5_e334_d_n14, eq5_e334_d_n15, eq5_e334_d_n16, eq5_e334_d_n17, eq5_e334_d_n18, eq5_e334_d_b0, eq5_e334_d_b1, eq5_e334_d_b2, eq5_e334_d_b3, eq5_e334_d_b4, eq5_e334_d_b5, eq5_e334_d_b6, eq5_e334_d_b7, eq5_e334_d_b8, eq5_e334_d_b9, eq5_e334_d_b10, eq5_e334_d_b11, eq5_e334_d_b12, eq5_e334_d_b13, eq5_e334_d_b14, eq5_e334_d_b15,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq5_value: f64 = eq5_e336;
        let eq5_node_derivatives: [f64; 19] = [eq5_e336_d_n0, eq5_e336_d_n1, eq5_e336_d_n2, eq5_e336_d_n3, eq5_e336_d_n4, eq5_e336_d_n5, eq5_e336_d_n6, eq5_e336_d_n7, eq5_e336_d_n8, eq5_e336_d_n9, eq5_e336_d_n10, eq5_e336_d_n11, eq5_e336_d_n12, eq5_e336_d_n13, eq5_e336_d_n14, eq5_e336_d_n15, eq5_e336_d_n16, eq5_e336_d_n17, eq5_e336_d_n18];
        let eq5_branch_derivatives: [f64; 16] = [eq5_e336_d_b0, eq5_e336_d_b1, eq5_e336_d_b2, eq5_e336_d_b3, eq5_e336_d_b4, eq5_e336_d_b5, eq5_e336_d_b6, eq5_e336_d_b7, eq5_e336_d_b8, eq5_e336_d_b9, eq5_e336_d_b10, eq5_e336_d_b11, eq5_e336_d_b12, eq5_e336_d_b13, eq5_e336_d_b14, eq5_e336_d_b15];
        stamper.stamp_current_dense_local(
            Some(11),
            Some(6),
            multiplicity * (eq5_value),
            &eq5_node_derivatives,
            &eq5_branch_derivatives,
            multiplicity,
        );
        let (eq6_e342, eq6_e342_d_n0, eq6_e342_d_n1, eq6_e342_d_n2, eq6_e342_d_n3, eq6_e342_d_n4, eq6_e342_d_n5, eq6_e342_d_n6, eq6_e342_d_n7, eq6_e342_d_n8, eq6_e342_d_n9, eq6_e342_d_n10, eq6_e342_d_n11, eq6_e342_d_n12, eq6_e342_d_n13, eq6_e342_d_n14, eq6_e342_d_n15, eq6_e342_d_n16, eq6_e342_d_n17, eq6_e342_d_n18, eq6_e342_d_b0, eq6_e342_d_b1, eq6_e342_d_b2, eq6_e342_d_b3, eq6_e342_d_b4, eq6_e342_d_b5, eq6_e342_d_b6, eq6_e342_d_b7, eq6_e342_d_b8, eq6_e342_d_b9, eq6_e342_d_b10, eq6_e342_d_b11, eq6_e342_d_b12, eq6_e342_d_b13, eq6_e342_d_b14, eq6_e342_d_b15,) = {
    if s.b[1848] {
        let eq6_e340: f64 = (p.p50 * s.v[308]);
        let eq6_e340_d_n0: f64 = (p.p50 * s.dn[308][0]);
        let eq6_e340_d_n1: f64 = (p.p50 * s.dn[308][1]);
        let eq6_e340_d_n2: f64 = (p.p50 * s.dn[308][2]);
        let eq6_e340_d_n3: f64 = (p.p50 * s.dn[308][3]);
        let eq6_e340_d_n4: f64 = (p.p50 * s.dn[308][4]);
        let eq6_e340_d_n5: f64 = (p.p50 * s.dn[308][5]);
        let eq6_e340_d_n6: f64 = (p.p50 * s.dn[308][6]);
        let eq6_e340_d_n7: f64 = (p.p50 * s.dn[308][7]);
        let eq6_e340_d_n8: f64 = (p.p50 * s.dn[308][8]);
        let eq6_e340_d_n9: f64 = (p.p50 * s.dn[308][9]);
        let eq6_e340_d_n10: f64 = (p.p50 * s.dn[308][10]);
        let eq6_e340_d_n11: f64 = (p.p50 * s.dn[308][11]);
        let eq6_e340_d_n12: f64 = (p.p50 * s.dn[308][12]);
        let eq6_e340_d_n13: f64 = (p.p50 * s.dn[308][13]);
        let eq6_e340_d_n14: f64 = (p.p50 * s.dn[308][14]);
        let eq6_e340_d_n15: f64 = (p.p50 * s.dn[308][15]);
        let eq6_e340_d_n16: f64 = (p.p50 * s.dn[308][16]);
        let eq6_e340_d_n17: f64 = (p.p50 * s.dn[308][17]);
        let eq6_e340_d_n18: f64 = (p.p50 * s.dn[308][18]);
        let eq6_e340_d_b0: f64 = (p.p50 * s.db[308][0]);
        let eq6_e340_d_b1: f64 = (p.p50 * s.db[308][1]);
        let eq6_e340_d_b2: f64 = (p.p50 * s.db[308][2]);
        let eq6_e340_d_b3: f64 = (p.p50 * s.db[308][3]);
        let eq6_e340_d_b4: f64 = (p.p50 * s.db[308][4]);
        let eq6_e340_d_b5: f64 = (p.p50 * s.db[308][5]);
        let eq6_e340_d_b6: f64 = (p.p50 * s.db[308][6]);
        let eq6_e340_d_b7: f64 = (p.p50 * s.db[308][7]);
        let eq6_e340_d_b8: f64 = (p.p50 * s.db[308][8]);
        let eq6_e340_d_b9: f64 = (p.p50 * s.db[308][9]);
        let eq6_e340_d_b10: f64 = (p.p50 * s.db[308][10]);
        let eq6_e340_d_b11: f64 = (p.p50 * s.db[308][11]);
        let eq6_e340_d_b12: f64 = (p.p50 * s.db[308][12]);
        let eq6_e340_d_b13: f64 = (p.p50 * s.db[308][13]);
        let eq6_e340_d_b14: f64 = (p.p50 * s.db[308][14]);
        let eq6_e340_d_b15: f64 = (p.p50 * s.db[308][15]);
        (eq6_e340, eq6_e340_d_n0, eq6_e340_d_n1, eq6_e340_d_n2, eq6_e340_d_n3, eq6_e340_d_n4, eq6_e340_d_n5, eq6_e340_d_n6, eq6_e340_d_n7, eq6_e340_d_n8, eq6_e340_d_n9, eq6_e340_d_n10, eq6_e340_d_n11, eq6_e340_d_n12, eq6_e340_d_n13, eq6_e340_d_n14, eq6_e340_d_n15, eq6_e340_d_n16, eq6_e340_d_n17, eq6_e340_d_n18, eq6_e340_d_b0, eq6_e340_d_b1, eq6_e340_d_b2, eq6_e340_d_b3, eq6_e340_d_b4, eq6_e340_d_b5, eq6_e340_d_b6, eq6_e340_d_b7, eq6_e340_d_b8, eq6_e340_d_b9, eq6_e340_d_b10, eq6_e340_d_b11, eq6_e340_d_b12, eq6_e340_d_b13, eq6_e340_d_b14, eq6_e340_d_b15,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq6_value: f64 = eq6_e342;
        let eq6_node_derivatives: [f64; 19] = [eq6_e342_d_n0, eq6_e342_d_n1, eq6_e342_d_n2, eq6_e342_d_n3, eq6_e342_d_n4, eq6_e342_d_n5, eq6_e342_d_n6, eq6_e342_d_n7, eq6_e342_d_n8, eq6_e342_d_n9, eq6_e342_d_n10, eq6_e342_d_n11, eq6_e342_d_n12, eq6_e342_d_n13, eq6_e342_d_n14, eq6_e342_d_n15, eq6_e342_d_n16, eq6_e342_d_n17, eq6_e342_d_n18];
        let eq6_branch_derivatives: [f64; 16] = [eq6_e342_d_b0, eq6_e342_d_b1, eq6_e342_d_b2, eq6_e342_d_b3, eq6_e342_d_b4, eq6_e342_d_b5, eq6_e342_d_b6, eq6_e342_d_b7, eq6_e342_d_b8, eq6_e342_d_b9, eq6_e342_d_b10, eq6_e342_d_b11, eq6_e342_d_b12, eq6_e342_d_b13, eq6_e342_d_b14, eq6_e342_d_b15];
        stamper.stamp_current_dense_local(
            Some(11),
            Some(12),
            multiplicity * (eq6_value),
            &eq6_node_derivatives,
            &eq6_branch_derivatives,
            multiplicity,
        );
        let (eq7_e348, eq7_e348_d_n0, eq7_e348_d_n1, eq7_e348_d_n2, eq7_e348_d_n3, eq7_e348_d_n4, eq7_e348_d_n5, eq7_e348_d_n6, eq7_e348_d_n7, eq7_e348_d_n8, eq7_e348_d_n9, eq7_e348_d_n10, eq7_e348_d_n11, eq7_e348_d_n12, eq7_e348_d_n13, eq7_e348_d_n14, eq7_e348_d_n15, eq7_e348_d_n16, eq7_e348_d_n17, eq7_e348_d_n18, eq7_e348_d_b0, eq7_e348_d_b1, eq7_e348_d_b2, eq7_e348_d_b3, eq7_e348_d_b4, eq7_e348_d_b5, eq7_e348_d_b6, eq7_e348_d_b7, eq7_e348_d_b8, eq7_e348_d_b9, eq7_e348_d_b10, eq7_e348_d_b11, eq7_e348_d_b12, eq7_e348_d_b13, eq7_e348_d_b14, eq7_e348_d_b15,) = {
    if (p.p259 != 0.0) {
        let eq7_e346: f64 = ((nv7 - nv2) / s.v[1]);
        let eq7_e346_d_n0: f64 = (-(((nv7 - nv2) * s.dn[1][0]) / (s.v[1] * s.v[1])));
        let eq7_e346_d_n1: f64 = (-(((nv7 - nv2) * s.dn[1][1]) / (s.v[1] * s.v[1])));
        let eq7_e346_d_n2: f64 = (((-s.v[1]) - ((nv7 - nv2) * s.dn[1][2])) / (s.v[1] * s.v[1]));
        let eq7_e346_d_n3: f64 = (-(((nv7 - nv2) * s.dn[1][3]) / (s.v[1] * s.v[1])));
        let eq7_e346_d_n4: f64 = (-(((nv7 - nv2) * s.dn[1][4]) / (s.v[1] * s.v[1])));
        let eq7_e346_d_n5: f64 = (-(((nv7 - nv2) * s.dn[1][5]) / (s.v[1] * s.v[1])));
        let eq7_e346_d_n6: f64 = (-(((nv7 - nv2) * s.dn[1][6]) / (s.v[1] * s.v[1])));
        let eq7_e346_d_n7: f64 = ((s.v[1] - ((nv7 - nv2) * s.dn[1][7])) / (s.v[1] * s.v[1]));
        let eq7_e346_d_n8: f64 = (-(((nv7 - nv2) * s.dn[1][8]) / (s.v[1] * s.v[1])));
        let eq7_e346_d_n9: f64 = (-(((nv7 - nv2) * s.dn[1][9]) / (s.v[1] * s.v[1])));
        let eq7_e346_d_n10: f64 = (-(((nv7 - nv2) * s.dn[1][10]) / (s.v[1] * s.v[1])));
        let eq7_e346_d_n11: f64 = (-(((nv7 - nv2) * s.dn[1][11]) / (s.v[1] * s.v[1])));
        let eq7_e346_d_n12: f64 = (-(((nv7 - nv2) * s.dn[1][12]) / (s.v[1] * s.v[1])));
        let eq7_e346_d_n13: f64 = (-(((nv7 - nv2) * s.dn[1][13]) / (s.v[1] * s.v[1])));
        let eq7_e346_d_n14: f64 = (-(((nv7 - nv2) * s.dn[1][14]) / (s.v[1] * s.v[1])));
        let eq7_e346_d_n15: f64 = (-(((nv7 - nv2) * s.dn[1][15]) / (s.v[1] * s.v[1])));
        let eq7_e346_d_n16: f64 = (-(((nv7 - nv2) * s.dn[1][16]) / (s.v[1] * s.v[1])));
        let eq7_e346_d_n17: f64 = (-(((nv7 - nv2) * s.dn[1][17]) / (s.v[1] * s.v[1])));
        let eq7_e346_d_n18: f64 = (-(((nv7 - nv2) * s.dn[1][18]) / (s.v[1] * s.v[1])));
        let eq7_e346_d_b0: f64 = (-(((nv7 - nv2) * s.db[1][0]) / (s.v[1] * s.v[1])));
        let eq7_e346_d_b1: f64 = (-(((nv7 - nv2) * s.db[1][1]) / (s.v[1] * s.v[1])));
        let eq7_e346_d_b2: f64 = (-(((nv7 - nv2) * s.db[1][2]) / (s.v[1] * s.v[1])));
        let eq7_e346_d_b3: f64 = (-(((nv7 - nv2) * s.db[1][3]) / (s.v[1] * s.v[1])));
        let eq7_e346_d_b4: f64 = (-(((nv7 - nv2) * s.db[1][4]) / (s.v[1] * s.v[1])));
        let eq7_e346_d_b5: f64 = (-(((nv7 - nv2) * s.db[1][5]) / (s.v[1] * s.v[1])));
        let eq7_e346_d_b6: f64 = (-(((nv7 - nv2) * s.db[1][6]) / (s.v[1] * s.v[1])));
        let eq7_e346_d_b7: f64 = (-(((nv7 - nv2) * s.db[1][7]) / (s.v[1] * s.v[1])));
        let eq7_e346_d_b8: f64 = (-(((nv7 - nv2) * s.db[1][8]) / (s.v[1] * s.v[1])));
        let eq7_e346_d_b9: f64 = (-(((nv7 - nv2) * s.db[1][9]) / (s.v[1] * s.v[1])));
        let eq7_e346_d_b10: f64 = (-(((nv7 - nv2) * s.db[1][10]) / (s.v[1] * s.v[1])));
        let eq7_e346_d_b11: f64 = (-(((nv7 - nv2) * s.db[1][11]) / (s.v[1] * s.v[1])));
        let eq7_e346_d_b12: f64 = (-(((nv7 - nv2) * s.db[1][12]) / (s.v[1] * s.v[1])));
        let eq7_e346_d_b13: f64 = (-(((nv7 - nv2) * s.db[1][13]) / (s.v[1] * s.v[1])));
        let eq7_e346_d_b14: f64 = (-(((nv7 - nv2) * s.db[1][14]) / (s.v[1] * s.v[1])));
        let eq7_e346_d_b15: f64 = (-(((nv7 - nv2) * s.db[1][15]) / (s.v[1] * s.v[1])));
        (eq7_e346, eq7_e346_d_n0, eq7_e346_d_n1, eq7_e346_d_n2, eq7_e346_d_n3, eq7_e346_d_n4, eq7_e346_d_n5, eq7_e346_d_n6, eq7_e346_d_n7, eq7_e346_d_n8, eq7_e346_d_n9, eq7_e346_d_n10, eq7_e346_d_n11, eq7_e346_d_n12, eq7_e346_d_n13, eq7_e346_d_n14, eq7_e346_d_n15, eq7_e346_d_n16, eq7_e346_d_n17, eq7_e346_d_n18, eq7_e346_d_b0, eq7_e346_d_b1, eq7_e346_d_b2, eq7_e346_d_b3, eq7_e346_d_b4, eq7_e346_d_b5, eq7_e346_d_b6, eq7_e346_d_b7, eq7_e346_d_b8, eq7_e346_d_b9, eq7_e346_d_b10, eq7_e346_d_b11, eq7_e346_d_b12, eq7_e346_d_b13, eq7_e346_d_b14, eq7_e346_d_b15,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq7_value: f64 = eq7_e348;
        let eq7_node_derivatives: [f64; 19] = [eq7_e348_d_n0, eq7_e348_d_n1, eq7_e348_d_n2, eq7_e348_d_n3, eq7_e348_d_n4, eq7_e348_d_n5, eq7_e348_d_n6, eq7_e348_d_n7, eq7_e348_d_n8, eq7_e348_d_n9, eq7_e348_d_n10, eq7_e348_d_n11, eq7_e348_d_n12, eq7_e348_d_n13, eq7_e348_d_n14, eq7_e348_d_n15, eq7_e348_d_n16, eq7_e348_d_n17, eq7_e348_d_n18];
        let eq7_branch_derivatives: [f64; 16] = [eq7_e348_d_b0, eq7_e348_d_b1, eq7_e348_d_b2, eq7_e348_d_b3, eq7_e348_d_b4, eq7_e348_d_b5, eq7_e348_d_b6, eq7_e348_d_b7, eq7_e348_d_b8, eq7_e348_d_b9, eq7_e348_d_b10, eq7_e348_d_b11, eq7_e348_d_b12, eq7_e348_d_b13, eq7_e348_d_b14, eq7_e348_d_b15];
        stamper.stamp_current_dense_local(
            Some(7),
            Some(2),
            multiplicity * (eq7_value),
            &eq7_node_derivatives,
            &eq7_branch_derivatives,
            multiplicity,
        );
        let (eq9_e359, eq9_e359_d_n0, eq9_e359_d_n1, eq9_e359_d_n2, eq9_e359_d_n3, eq9_e359_d_n4, eq9_e359_d_n5, eq9_e359_d_n6, eq9_e359_d_n7, eq9_e359_d_n8, eq9_e359_d_n9, eq9_e359_d_n10, eq9_e359_d_n11, eq9_e359_d_n12, eq9_e359_d_n13, eq9_e359_d_n14, eq9_e359_d_n15, eq9_e359_d_n16, eq9_e359_d_n17, eq9_e359_d_n18, eq9_e359_d_b0, eq9_e359_d_b1, eq9_e359_d_b2, eq9_e359_d_b3, eq9_e359_d_b4, eq9_e359_d_b5, eq9_e359_d_b6, eq9_e359_d_b7, eq9_e359_d_b8, eq9_e359_d_b9, eq9_e359_d_b10, eq9_e359_d_b11, eq9_e359_d_b12, eq9_e359_d_b13, eq9_e359_d_b14, eq9_e359_d_b15,) = {
    if (p.p260 != 0.0) {
        let eq9_e357: f64 = ((nv0 - nv6) / s.v[0]);
        let eq9_e357_d_n0: f64 = ((s.v[0] - ((nv0 - nv6) * s.dn[0][0])) / (s.v[0] * s.v[0]));
        let eq9_e357_d_n1: f64 = (-(((nv0 - nv6) * s.dn[0][1]) / (s.v[0] * s.v[0])));
        let eq9_e357_d_n2: f64 = (-(((nv0 - nv6) * s.dn[0][2]) / (s.v[0] * s.v[0])));
        let eq9_e357_d_n3: f64 = (-(((nv0 - nv6) * s.dn[0][3]) / (s.v[0] * s.v[0])));
        let eq9_e357_d_n4: f64 = (-(((nv0 - nv6) * s.dn[0][4]) / (s.v[0] * s.v[0])));
        let eq9_e357_d_n5: f64 = (-(((nv0 - nv6) * s.dn[0][5]) / (s.v[0] * s.v[0])));
        let eq9_e357_d_n6: f64 = (((-s.v[0]) - ((nv0 - nv6) * s.dn[0][6])) / (s.v[0] * s.v[0]));
        let eq9_e357_d_n7: f64 = (-(((nv0 - nv6) * s.dn[0][7]) / (s.v[0] * s.v[0])));
        let eq9_e357_d_n8: f64 = (-(((nv0 - nv6) * s.dn[0][8]) / (s.v[0] * s.v[0])));
        let eq9_e357_d_n9: f64 = (-(((nv0 - nv6) * s.dn[0][9]) / (s.v[0] * s.v[0])));
        let eq9_e357_d_n10: f64 = (-(((nv0 - nv6) * s.dn[0][10]) / (s.v[0] * s.v[0])));
        let eq9_e357_d_n11: f64 = (-(((nv0 - nv6) * s.dn[0][11]) / (s.v[0] * s.v[0])));
        let eq9_e357_d_n12: f64 = (-(((nv0 - nv6) * s.dn[0][12]) / (s.v[0] * s.v[0])));
        let eq9_e357_d_n13: f64 = (-(((nv0 - nv6) * s.dn[0][13]) / (s.v[0] * s.v[0])));
        let eq9_e357_d_n14: f64 = (-(((nv0 - nv6) * s.dn[0][14]) / (s.v[0] * s.v[0])));
        let eq9_e357_d_n15: f64 = (-(((nv0 - nv6) * s.dn[0][15]) / (s.v[0] * s.v[0])));
        let eq9_e357_d_n16: f64 = (-(((nv0 - nv6) * s.dn[0][16]) / (s.v[0] * s.v[0])));
        let eq9_e357_d_n17: f64 = (-(((nv0 - nv6) * s.dn[0][17]) / (s.v[0] * s.v[0])));
        let eq9_e357_d_n18: f64 = (-(((nv0 - nv6) * s.dn[0][18]) / (s.v[0] * s.v[0])));
        let eq9_e357_d_b0: f64 = (-(((nv0 - nv6) * s.db[0][0]) / (s.v[0] * s.v[0])));
        let eq9_e357_d_b1: f64 = (-(((nv0 - nv6) * s.db[0][1]) / (s.v[0] * s.v[0])));
        let eq9_e357_d_b2: f64 = (-(((nv0 - nv6) * s.db[0][2]) / (s.v[0] * s.v[0])));
        let eq9_e357_d_b3: f64 = (-(((nv0 - nv6) * s.db[0][3]) / (s.v[0] * s.v[0])));
        let eq9_e357_d_b4: f64 = (-(((nv0 - nv6) * s.db[0][4]) / (s.v[0] * s.v[0])));
        let eq9_e357_d_b5: f64 = (-(((nv0 - nv6) * s.db[0][5]) / (s.v[0] * s.v[0])));
        let eq9_e357_d_b6: f64 = (-(((nv0 - nv6) * s.db[0][6]) / (s.v[0] * s.v[0])));
        let eq9_e357_d_b7: f64 = (-(((nv0 - nv6) * s.db[0][7]) / (s.v[0] * s.v[0])));
        let eq9_e357_d_b8: f64 = (-(((nv0 - nv6) * s.db[0][8]) / (s.v[0] * s.v[0])));
        let eq9_e357_d_b9: f64 = (-(((nv0 - nv6) * s.db[0][9]) / (s.v[0] * s.v[0])));
        let eq9_e357_d_b10: f64 = (-(((nv0 - nv6) * s.db[0][10]) / (s.v[0] * s.v[0])));
        let eq9_e357_d_b11: f64 = (-(((nv0 - nv6) * s.db[0][11]) / (s.v[0] * s.v[0])));
        let eq9_e357_d_b12: f64 = (-(((nv0 - nv6) * s.db[0][12]) / (s.v[0] * s.v[0])));
        let eq9_e357_d_b13: f64 = (-(((nv0 - nv6) * s.db[0][13]) / (s.v[0] * s.v[0])));
        let eq9_e357_d_b14: f64 = (-(((nv0 - nv6) * s.db[0][14]) / (s.v[0] * s.v[0])));
        let eq9_e357_d_b15: f64 = (-(((nv0 - nv6) * s.db[0][15]) / (s.v[0] * s.v[0])));
        (eq9_e357, eq9_e357_d_n0, eq9_e357_d_n1, eq9_e357_d_n2, eq9_e357_d_n3, eq9_e357_d_n4, eq9_e357_d_n5, eq9_e357_d_n6, eq9_e357_d_n7, eq9_e357_d_n8, eq9_e357_d_n9, eq9_e357_d_n10, eq9_e357_d_n11, eq9_e357_d_n12, eq9_e357_d_n13, eq9_e357_d_n14, eq9_e357_d_n15, eq9_e357_d_n16, eq9_e357_d_n17, eq9_e357_d_n18, eq9_e357_d_b0, eq9_e357_d_b1, eq9_e357_d_b2, eq9_e357_d_b3, eq9_e357_d_b4, eq9_e357_d_b5, eq9_e357_d_b6, eq9_e357_d_b7, eq9_e357_d_b8, eq9_e357_d_b9, eq9_e357_d_b10, eq9_e357_d_b11, eq9_e357_d_b12, eq9_e357_d_b13, eq9_e357_d_b14, eq9_e357_d_b15,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq9_value: f64 = eq9_e359;
        let eq9_node_derivatives: [f64; 19] = [eq9_e359_d_n0, eq9_e359_d_n1, eq9_e359_d_n2, eq9_e359_d_n3, eq9_e359_d_n4, eq9_e359_d_n5, eq9_e359_d_n6, eq9_e359_d_n7, eq9_e359_d_n8, eq9_e359_d_n9, eq9_e359_d_n10, eq9_e359_d_n11, eq9_e359_d_n12, eq9_e359_d_n13, eq9_e359_d_n14, eq9_e359_d_n15, eq9_e359_d_n16, eq9_e359_d_n17, eq9_e359_d_n18];
        let eq9_branch_derivatives: [f64; 16] = [eq9_e359_d_b0, eq9_e359_d_b1, eq9_e359_d_b2, eq9_e359_d_b3, eq9_e359_d_b4, eq9_e359_d_b5, eq9_e359_d_b6, eq9_e359_d_b7, eq9_e359_d_b8, eq9_e359_d_b9, eq9_e359_d_b10, eq9_e359_d_b11, eq9_e359_d_b12, eq9_e359_d_b13, eq9_e359_d_b14, eq9_e359_d_b15];
        stamper.stamp_current_dense_local(
            Some(0),
            Some(6),
            multiplicity * (eq9_value),
            &eq9_node_derivatives,
            &eq9_branch_derivatives,
            multiplicity,
        );
        let eq11_e367: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 0, s.v[594]);
        let eq11_e367_d_n0: f64 = (s.dn[594][0] * ddt_scale);
        let eq11_e367_d_n1: f64 = (s.dn[594][1] * ddt_scale);
        let eq11_e367_d_n2: f64 = (s.dn[594][2] * ddt_scale);
        let eq11_e367_d_n3: f64 = (s.dn[594][3] * ddt_scale);
        let eq11_e367_d_n4: f64 = (s.dn[594][4] * ddt_scale);
        let eq11_e367_d_n5: f64 = (s.dn[594][5] * ddt_scale);
        let eq11_e367_d_n6: f64 = (s.dn[594][6] * ddt_scale);
        let eq11_e367_d_n7: f64 = (s.dn[594][7] * ddt_scale);
        let eq11_e367_d_n8: f64 = (s.dn[594][8] * ddt_scale);
        let eq11_e367_d_n9: f64 = (s.dn[594][9] * ddt_scale);
        let eq11_e367_d_n10: f64 = (s.dn[594][10] * ddt_scale);
        let eq11_e367_d_n11: f64 = (s.dn[594][11] * ddt_scale);
        let eq11_e367_d_n12: f64 = (s.dn[594][12] * ddt_scale);
        let eq11_e367_d_n13: f64 = (s.dn[594][13] * ddt_scale);
        let eq11_e367_d_n14: f64 = (s.dn[594][14] * ddt_scale);
        let eq11_e367_d_n15: f64 = (s.dn[594][15] * ddt_scale);
        let eq11_e367_d_n16: f64 = (s.dn[594][16] * ddt_scale);
        let eq11_e367_d_n17: f64 = (s.dn[594][17] * ddt_scale);
        let eq11_e367_d_n18: f64 = (s.dn[594][18] * ddt_scale);
        let eq11_e367_d_b0: f64 = (s.db[594][0] * ddt_scale);
        let eq11_e367_d_b1: f64 = (s.db[594][1] * ddt_scale);
        let eq11_e367_d_b2: f64 = (s.db[594][2] * ddt_scale);
        let eq11_e367_d_b3: f64 = (s.db[594][3] * ddt_scale);
        let eq11_e367_d_b4: f64 = (s.db[594][4] * ddt_scale);
        let eq11_e367_d_b5: f64 = (s.db[594][5] * ddt_scale);
        let eq11_e367_d_b6: f64 = (s.db[594][6] * ddt_scale);
        let eq11_e367_d_b7: f64 = (s.db[594][7] * ddt_scale);
        let eq11_e367_d_b8: f64 = (s.db[594][8] * ddt_scale);
        let eq11_e367_d_b9: f64 = (s.db[594][9] * ddt_scale);
        let eq11_e367_d_b10: f64 = (s.db[594][10] * ddt_scale);
        let eq11_e367_d_b11: f64 = (s.db[594][11] * ddt_scale);
        let eq11_e367_d_b12: f64 = (s.db[594][12] * ddt_scale);
        let eq11_e367_d_b13: f64 = (s.db[594][13] * ddt_scale);
        let eq11_e367_d_b14: f64 = (s.db[594][14] * ddt_scale);
        let eq11_e367_d_b15: f64 = (s.db[594][15] * ddt_scale);
        let eq11_e368: f64 = (p.p50 * eq11_e367);
        let eq11_e368_d_n0: f64 = (p.p50 * eq11_e367_d_n0);
        let eq11_e368_d_n1: f64 = (p.p50 * eq11_e367_d_n1);
        let eq11_e368_d_n2: f64 = (p.p50 * eq11_e367_d_n2);
        let eq11_e368_d_n3: f64 = (p.p50 * eq11_e367_d_n3);
        let eq11_e368_d_n4: f64 = (p.p50 * eq11_e367_d_n4);
        let eq11_e368_d_n5: f64 = (p.p50 * eq11_e367_d_n5);
        let eq11_e368_d_n6: f64 = (p.p50 * eq11_e367_d_n6);
        let eq11_e368_d_n7: f64 = (p.p50 * eq11_e367_d_n7);
        let eq11_e368_d_n8: f64 = (p.p50 * eq11_e367_d_n8);
        let eq11_e368_d_n9: f64 = (p.p50 * eq11_e367_d_n9);
        let eq11_e368_d_n10: f64 = (p.p50 * eq11_e367_d_n10);
        let eq11_e368_d_n11: f64 = (p.p50 * eq11_e367_d_n11);
        let eq11_e368_d_n12: f64 = (p.p50 * eq11_e367_d_n12);
        let eq11_e368_d_n13: f64 = (p.p50 * eq11_e367_d_n13);
        let eq11_e368_d_n14: f64 = (p.p50 * eq11_e367_d_n14);
        let eq11_e368_d_n15: f64 = (p.p50 * eq11_e367_d_n15);
        let eq11_e368_d_n16: f64 = (p.p50 * eq11_e367_d_n16);
        let eq11_e368_d_n17: f64 = (p.p50 * eq11_e367_d_n17);
        let eq11_e368_d_n18: f64 = (p.p50 * eq11_e367_d_n18);
        let eq11_e368_d_b0: f64 = (p.p50 * eq11_e367_d_b0);
        let eq11_e368_d_b1: f64 = (p.p50 * eq11_e367_d_b1);
        let eq11_e368_d_b2: f64 = (p.p50 * eq11_e367_d_b2);
        let eq11_e368_d_b3: f64 = (p.p50 * eq11_e367_d_b3);
        let eq11_e368_d_b4: f64 = (p.p50 * eq11_e367_d_b4);
        let eq11_e368_d_b5: f64 = (p.p50 * eq11_e367_d_b5);
        let eq11_e368_d_b6: f64 = (p.p50 * eq11_e367_d_b6);
        let eq11_e368_d_b7: f64 = (p.p50 * eq11_e367_d_b7);
        let eq11_e368_d_b8: f64 = (p.p50 * eq11_e367_d_b8);
        let eq11_e368_d_b9: f64 = (p.p50 * eq11_e367_d_b9);
        let eq11_e368_d_b10: f64 = (p.p50 * eq11_e367_d_b10);
        let eq11_e368_d_b11: f64 = (p.p50 * eq11_e367_d_b11);
        let eq11_e368_d_b12: f64 = (p.p50 * eq11_e367_d_b12);
        let eq11_e368_d_b13: f64 = (p.p50 * eq11_e367_d_b13);
        let eq11_e368_d_b14: f64 = (p.p50 * eq11_e367_d_b14);
        let eq11_e368_d_b15: f64 = (p.p50 * eq11_e367_d_b15);
        let eq11_value: f64 = eq11_e368;
        let eq11_node_derivatives: [f64; 19] = [eq11_e368_d_n0, eq11_e368_d_n1, eq11_e368_d_n2, eq11_e368_d_n3, eq11_e368_d_n4, eq11_e368_d_n5, eq11_e368_d_n6, eq11_e368_d_n7, eq11_e368_d_n8, eq11_e368_d_n9, eq11_e368_d_n10, eq11_e368_d_n11, eq11_e368_d_n12, eq11_e368_d_n13, eq11_e368_d_n14, eq11_e368_d_n15, eq11_e368_d_n16, eq11_e368_d_n17, eq11_e368_d_n18];
        let eq11_branch_derivatives: [f64; 16] = [eq11_e368_d_b0, eq11_e368_d_b1, eq11_e368_d_b2, eq11_e368_d_b3, eq11_e368_d_b4, eq11_e368_d_b5, eq11_e368_d_b6, eq11_e368_d_b7, eq11_e368_d_b8, eq11_e368_d_b9, eq11_e368_d_b10, eq11_e368_d_b11, eq11_e368_d_b12, eq11_e368_d_b13, eq11_e368_d_b14, eq11_e368_d_b15];
        stamper.stamp_current_dense_local(
            Some(11),
            Some(7),
            multiplicity * (eq11_value),
            &eq11_node_derivatives,
            &eq11_branch_derivatives,
            multiplicity,
        );
        let eq12_e371: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 1, s.v[198]);
        let eq12_e371_d_n0: f64 = (s.dn[198][0] * ddt_scale);
        let eq12_e371_d_n1: f64 = (s.dn[198][1] * ddt_scale);
        let eq12_e371_d_n2: f64 = (s.dn[198][2] * ddt_scale);
        let eq12_e371_d_n3: f64 = (s.dn[198][3] * ddt_scale);
        let eq12_e371_d_n4: f64 = (s.dn[198][4] * ddt_scale);
        let eq12_e371_d_n5: f64 = (s.dn[198][5] * ddt_scale);
        let eq12_e371_d_n6: f64 = (s.dn[198][6] * ddt_scale);
        let eq12_e371_d_n7: f64 = (s.dn[198][7] * ddt_scale);
        let eq12_e371_d_n8: f64 = (s.dn[198][8] * ddt_scale);
        let eq12_e371_d_n9: f64 = (s.dn[198][9] * ddt_scale);
        let eq12_e371_d_n10: f64 = (s.dn[198][10] * ddt_scale);
        let eq12_e371_d_n11: f64 = (s.dn[198][11] * ddt_scale);
        let eq12_e371_d_n12: f64 = (s.dn[198][12] * ddt_scale);
        let eq12_e371_d_n13: f64 = (s.dn[198][13] * ddt_scale);
        let eq12_e371_d_n14: f64 = (s.dn[198][14] * ddt_scale);
        let eq12_e371_d_n15: f64 = (s.dn[198][15] * ddt_scale);
        let eq12_e371_d_n16: f64 = (s.dn[198][16] * ddt_scale);
        let eq12_e371_d_n17: f64 = (s.dn[198][17] * ddt_scale);
        let eq12_e371_d_n18: f64 = (s.dn[198][18] * ddt_scale);
        let eq12_e371_d_b0: f64 = (s.db[198][0] * ddt_scale);
        let eq12_e371_d_b1: f64 = (s.db[198][1] * ddt_scale);
        let eq12_e371_d_b2: f64 = (s.db[198][2] * ddt_scale);
        let eq12_e371_d_b3: f64 = (s.db[198][3] * ddt_scale);
        let eq12_e371_d_b4: f64 = (s.db[198][4] * ddt_scale);
        let eq12_e371_d_b5: f64 = (s.db[198][5] * ddt_scale);
        let eq12_e371_d_b6: f64 = (s.db[198][6] * ddt_scale);
        let eq12_e371_d_b7: f64 = (s.db[198][7] * ddt_scale);
        let eq12_e371_d_b8: f64 = (s.db[198][8] * ddt_scale);
        let eq12_e371_d_b9: f64 = (s.db[198][9] * ddt_scale);
        let eq12_e371_d_b10: f64 = (s.db[198][10] * ddt_scale);
        let eq12_e371_d_b11: f64 = (s.db[198][11] * ddt_scale);
        let eq12_e371_d_b12: f64 = (s.db[198][12] * ddt_scale);
        let eq12_e371_d_b13: f64 = (s.db[198][13] * ddt_scale);
        let eq12_e371_d_b14: f64 = (s.db[198][14] * ddt_scale);
        let eq12_e371_d_b15: f64 = (s.db[198][15] * ddt_scale);
        let eq12_e372: f64 = (p.p50 * eq12_e371);
        let eq12_e372_d_n0: f64 = (p.p50 * eq12_e371_d_n0);
        let eq12_e372_d_n1: f64 = (p.p50 * eq12_e371_d_n1);
        let eq12_e372_d_n2: f64 = (p.p50 * eq12_e371_d_n2);
        let eq12_e372_d_n3: f64 = (p.p50 * eq12_e371_d_n3);
        let eq12_e372_d_n4: f64 = (p.p50 * eq12_e371_d_n4);
        let eq12_e372_d_n5: f64 = (p.p50 * eq12_e371_d_n5);
        let eq12_e372_d_n6: f64 = (p.p50 * eq12_e371_d_n6);
        let eq12_e372_d_n7: f64 = (p.p50 * eq12_e371_d_n7);
        let eq12_e372_d_n8: f64 = (p.p50 * eq12_e371_d_n8);
        let eq12_e372_d_n9: f64 = (p.p50 * eq12_e371_d_n9);
        let eq12_e372_d_n10: f64 = (p.p50 * eq12_e371_d_n10);
        let eq12_e372_d_n11: f64 = (p.p50 * eq12_e371_d_n11);
        let eq12_e372_d_n12: f64 = (p.p50 * eq12_e371_d_n12);
        let eq12_e372_d_n13: f64 = (p.p50 * eq12_e371_d_n13);
        let eq12_e372_d_n14: f64 = (p.p50 * eq12_e371_d_n14);
        let eq12_e372_d_n15: f64 = (p.p50 * eq12_e371_d_n15);
        let eq12_e372_d_n16: f64 = (p.p50 * eq12_e371_d_n16);
        let eq12_e372_d_n17: f64 = (p.p50 * eq12_e371_d_n17);
        let eq12_e372_d_n18: f64 = (p.p50 * eq12_e371_d_n18);
        let eq12_e372_d_b0: f64 = (p.p50 * eq12_e371_d_b0);
        let eq12_e372_d_b1: f64 = (p.p50 * eq12_e371_d_b1);
        let eq12_e372_d_b2: f64 = (p.p50 * eq12_e371_d_b2);
        let eq12_e372_d_b3: f64 = (p.p50 * eq12_e371_d_b3);
        let eq12_e372_d_b4: f64 = (p.p50 * eq12_e371_d_b4);
        let eq12_e372_d_b5: f64 = (p.p50 * eq12_e371_d_b5);
        let eq12_e372_d_b6: f64 = (p.p50 * eq12_e371_d_b6);
        let eq12_e372_d_b7: f64 = (p.p50 * eq12_e371_d_b7);
        let eq12_e372_d_b8: f64 = (p.p50 * eq12_e371_d_b8);
        let eq12_e372_d_b9: f64 = (p.p50 * eq12_e371_d_b9);
        let eq12_e372_d_b10: f64 = (p.p50 * eq12_e371_d_b10);
        let eq12_e372_d_b11: f64 = (p.p50 * eq12_e371_d_b11);
        let eq12_e372_d_b12: f64 = (p.p50 * eq12_e371_d_b12);
        let eq12_e372_d_b13: f64 = (p.p50 * eq12_e371_d_b13);
        let eq12_e372_d_b14: f64 = (p.p50 * eq12_e371_d_b14);
        let eq12_e372_d_b15: f64 = (p.p50 * eq12_e371_d_b15);
        let eq12_value: f64 = eq12_e372;
        let eq12_node_derivatives: [f64; 19] = [eq12_e372_d_n0, eq12_e372_d_n1, eq12_e372_d_n2, eq12_e372_d_n3, eq12_e372_d_n4, eq12_e372_d_n5, eq12_e372_d_n6, eq12_e372_d_n7, eq12_e372_d_n8, eq12_e372_d_n9, eq12_e372_d_n10, eq12_e372_d_n11, eq12_e372_d_n12, eq12_e372_d_n13, eq12_e372_d_n14, eq12_e372_d_n15, eq12_e372_d_n16, eq12_e372_d_n17, eq12_e372_d_n18];
        let eq12_branch_derivatives: [f64; 16] = [eq12_e372_d_b0, eq12_e372_d_b1, eq12_e372_d_b2, eq12_e372_d_b3, eq12_e372_d_b4, eq12_e372_d_b5, eq12_e372_d_b6, eq12_e372_d_b7, eq12_e372_d_b8, eq12_e372_d_b9, eq12_e372_d_b10, eq12_e372_d_b11, eq12_e372_d_b12, eq12_e372_d_b13, eq12_e372_d_b14, eq12_e372_d_b15];
        stamper.stamp_current_dense_local(
            Some(6),
            Some(7),
            multiplicity * (eq12_value),
            &eq12_node_derivatives,
            &eq12_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_transient_equations_block_1(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        multiplicity: f64,
        ddt_active: bool,
        ddt_scale: f64,
        ddt_state_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_initialized: &mut [bool; Instance::DDT_STATE_COUNT],
    ) {
        let nv1 = ctx.node_voltage(nodes[1]);
        let nv10 = ctx.node_voltage(nodes[10]);
        let nv11 = ctx.node_voltage(nodes[11]);
        let nv14 = ctx.node_voltage(nodes[14]);
        let eq13_e375: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 2, s.v[196]);
        let eq13_e375_d_n0: f64 = (s.dn[196][0] * ddt_scale);
        let eq13_e375_d_n1: f64 = (s.dn[196][1] * ddt_scale);
        let eq13_e375_d_n2: f64 = (s.dn[196][2] * ddt_scale);
        let eq13_e375_d_n3: f64 = (s.dn[196][3] * ddt_scale);
        let eq13_e375_d_n4: f64 = (s.dn[196][4] * ddt_scale);
        let eq13_e375_d_n5: f64 = (s.dn[196][5] * ddt_scale);
        let eq13_e375_d_n6: f64 = (s.dn[196][6] * ddt_scale);
        let eq13_e375_d_n7: f64 = (s.dn[196][7] * ddt_scale);
        let eq13_e375_d_n8: f64 = (s.dn[196][8] * ddt_scale);
        let eq13_e375_d_n9: f64 = (s.dn[196][9] * ddt_scale);
        let eq13_e375_d_n10: f64 = (s.dn[196][10] * ddt_scale);
        let eq13_e375_d_n11: f64 = (s.dn[196][11] * ddt_scale);
        let eq13_e375_d_n12: f64 = (s.dn[196][12] * ddt_scale);
        let eq13_e375_d_n13: f64 = (s.dn[196][13] * ddt_scale);
        let eq13_e375_d_n14: f64 = (s.dn[196][14] * ddt_scale);
        let eq13_e375_d_n15: f64 = (s.dn[196][15] * ddt_scale);
        let eq13_e375_d_n16: f64 = (s.dn[196][16] * ddt_scale);
        let eq13_e375_d_n17: f64 = (s.dn[196][17] * ddt_scale);
        let eq13_e375_d_n18: f64 = (s.dn[196][18] * ddt_scale);
        let eq13_e375_d_b0: f64 = (s.db[196][0] * ddt_scale);
        let eq13_e375_d_b1: f64 = (s.db[196][1] * ddt_scale);
        let eq13_e375_d_b2: f64 = (s.db[196][2] * ddt_scale);
        let eq13_e375_d_b3: f64 = (s.db[196][3] * ddt_scale);
        let eq13_e375_d_b4: f64 = (s.db[196][4] * ddt_scale);
        let eq13_e375_d_b5: f64 = (s.db[196][5] * ddt_scale);
        let eq13_e375_d_b6: f64 = (s.db[196][6] * ddt_scale);
        let eq13_e375_d_b7: f64 = (s.db[196][7] * ddt_scale);
        let eq13_e375_d_b8: f64 = (s.db[196][8] * ddt_scale);
        let eq13_e375_d_b9: f64 = (s.db[196][9] * ddt_scale);
        let eq13_e375_d_b10: f64 = (s.db[196][10] * ddt_scale);
        let eq13_e375_d_b11: f64 = (s.db[196][11] * ddt_scale);
        let eq13_e375_d_b12: f64 = (s.db[196][12] * ddt_scale);
        let eq13_e375_d_b13: f64 = (s.db[196][13] * ddt_scale);
        let eq13_e375_d_b14: f64 = (s.db[196][14] * ddt_scale);
        let eq13_e375_d_b15: f64 = (s.db[196][15] * ddt_scale);
        let eq13_e376: f64 = (p.p50 * eq13_e375);
        let eq13_e376_d_n0: f64 = (p.p50 * eq13_e375_d_n0);
        let eq13_e376_d_n1: f64 = (p.p50 * eq13_e375_d_n1);
        let eq13_e376_d_n2: f64 = (p.p50 * eq13_e375_d_n2);
        let eq13_e376_d_n3: f64 = (p.p50 * eq13_e375_d_n3);
        let eq13_e376_d_n4: f64 = (p.p50 * eq13_e375_d_n4);
        let eq13_e376_d_n5: f64 = (p.p50 * eq13_e375_d_n5);
        let eq13_e376_d_n6: f64 = (p.p50 * eq13_e375_d_n6);
        let eq13_e376_d_n7: f64 = (p.p50 * eq13_e375_d_n7);
        let eq13_e376_d_n8: f64 = (p.p50 * eq13_e375_d_n8);
        let eq13_e376_d_n9: f64 = (p.p50 * eq13_e375_d_n9);
        let eq13_e376_d_n10: f64 = (p.p50 * eq13_e375_d_n10);
        let eq13_e376_d_n11: f64 = (p.p50 * eq13_e375_d_n11);
        let eq13_e376_d_n12: f64 = (p.p50 * eq13_e375_d_n12);
        let eq13_e376_d_n13: f64 = (p.p50 * eq13_e375_d_n13);
        let eq13_e376_d_n14: f64 = (p.p50 * eq13_e375_d_n14);
        let eq13_e376_d_n15: f64 = (p.p50 * eq13_e375_d_n15);
        let eq13_e376_d_n16: f64 = (p.p50 * eq13_e375_d_n16);
        let eq13_e376_d_n17: f64 = (p.p50 * eq13_e375_d_n17);
        let eq13_e376_d_n18: f64 = (p.p50 * eq13_e375_d_n18);
        let eq13_e376_d_b0: f64 = (p.p50 * eq13_e375_d_b0);
        let eq13_e376_d_b1: f64 = (p.p50 * eq13_e375_d_b1);
        let eq13_e376_d_b2: f64 = (p.p50 * eq13_e375_d_b2);
        let eq13_e376_d_b3: f64 = (p.p50 * eq13_e375_d_b3);
        let eq13_e376_d_b4: f64 = (p.p50 * eq13_e375_d_b4);
        let eq13_e376_d_b5: f64 = (p.p50 * eq13_e375_d_b5);
        let eq13_e376_d_b6: f64 = (p.p50 * eq13_e375_d_b6);
        let eq13_e376_d_b7: f64 = (p.p50 * eq13_e375_d_b7);
        let eq13_e376_d_b8: f64 = (p.p50 * eq13_e375_d_b8);
        let eq13_e376_d_b9: f64 = (p.p50 * eq13_e375_d_b9);
        let eq13_e376_d_b10: f64 = (p.p50 * eq13_e375_d_b10);
        let eq13_e376_d_b11: f64 = (p.p50 * eq13_e375_d_b11);
        let eq13_e376_d_b12: f64 = (p.p50 * eq13_e375_d_b12);
        let eq13_e376_d_b13: f64 = (p.p50 * eq13_e375_d_b13);
        let eq13_e376_d_b14: f64 = (p.p50 * eq13_e375_d_b14);
        let eq13_e376_d_b15: f64 = (p.p50 * eq13_e375_d_b15);
        let eq13_value: f64 = eq13_e376;
        let eq13_node_derivatives: [f64; 19] = [eq13_e376_d_n0, eq13_e376_d_n1, eq13_e376_d_n2, eq13_e376_d_n3, eq13_e376_d_n4, eq13_e376_d_n5, eq13_e376_d_n6, eq13_e376_d_n7, eq13_e376_d_n8, eq13_e376_d_n9, eq13_e376_d_n10, eq13_e376_d_n11, eq13_e376_d_n12, eq13_e376_d_n13, eq13_e376_d_n14, eq13_e376_d_n15, eq13_e376_d_n16, eq13_e376_d_n17, eq13_e376_d_n18];
        let eq13_branch_derivatives: [f64; 16] = [eq13_e376_d_b0, eq13_e376_d_b1, eq13_e376_d_b2, eq13_e376_d_b3, eq13_e376_d_b4, eq13_e376_d_b5, eq13_e376_d_b6, eq13_e376_d_b7, eq13_e376_d_b8, eq13_e376_d_b9, eq13_e376_d_b10, eq13_e376_d_b11, eq13_e376_d_b12, eq13_e376_d_b13, eq13_e376_d_b14, eq13_e376_d_b15];
        stamper.stamp_current_dense_local(
            Some(12),
            Some(7),
            multiplicity * (eq13_value),
            &eq13_node_derivatives,
            &eq13_branch_derivatives,
            multiplicity,
        );
        let eq18_e402: f64 = (s.v[614] * (nv14 - 0.0));
        let eq18_e402_d_n0: f64 = (s.dn[614][0] * (nv14 - 0.0));
        let eq18_e402_d_n1: f64 = (s.dn[614][1] * (nv14 - 0.0));
        let eq18_e402_d_n2: f64 = (s.dn[614][2] * (nv14 - 0.0));
        let eq18_e402_d_n3: f64 = (s.dn[614][3] * (nv14 - 0.0));
        let eq18_e402_d_n4: f64 = (s.dn[614][4] * (nv14 - 0.0));
        let eq18_e402_d_n5: f64 = (s.dn[614][5] * (nv14 - 0.0));
        let eq18_e402_d_n6: f64 = (s.dn[614][6] * (nv14 - 0.0));
        let eq18_e402_d_n7: f64 = (s.dn[614][7] * (nv14 - 0.0));
        let eq18_e402_d_n8: f64 = (s.dn[614][8] * (nv14 - 0.0));
        let eq18_e402_d_n9: f64 = (s.dn[614][9] * (nv14 - 0.0));
        let eq18_e402_d_n10: f64 = (s.dn[614][10] * (nv14 - 0.0));
        let eq18_e402_d_n11: f64 = (s.dn[614][11] * (nv14 - 0.0));
        let eq18_e402_d_n12: f64 = (s.dn[614][12] * (nv14 - 0.0));
        let eq18_e402_d_n13: f64 = (s.dn[614][13] * (nv14 - 0.0));
        let eq18_e402_d_n14: f64 = ((s.dn[614][14] * (nv14 - 0.0)) + s.v[614]);
        let eq18_e402_d_n15: f64 = (s.dn[614][15] * (nv14 - 0.0));
        let eq18_e402_d_n16: f64 = (s.dn[614][16] * (nv14 - 0.0));
        let eq18_e402_d_n17: f64 = (s.dn[614][17] * (nv14 - 0.0));
        let eq18_e402_d_n18: f64 = (s.dn[614][18] * (nv14 - 0.0));
        let eq18_e402_d_b0: f64 = (s.db[614][0] * (nv14 - 0.0));
        let eq18_e402_d_b1: f64 = (s.db[614][1] * (nv14 - 0.0));
        let eq18_e402_d_b2: f64 = (s.db[614][2] * (nv14 - 0.0));
        let eq18_e402_d_b3: f64 = (s.db[614][3] * (nv14 - 0.0));
        let eq18_e402_d_b4: f64 = (s.db[614][4] * (nv14 - 0.0));
        let eq18_e402_d_b5: f64 = (s.db[614][5] * (nv14 - 0.0));
        let eq18_e402_d_b6: f64 = (s.db[614][6] * (nv14 - 0.0));
        let eq18_e402_d_b7: f64 = (s.db[614][7] * (nv14 - 0.0));
        let eq18_e402_d_b8: f64 = (s.db[614][8] * (nv14 - 0.0));
        let eq18_e402_d_b9: f64 = (s.db[614][9] * (nv14 - 0.0));
        let eq18_e402_d_b10: f64 = (s.db[614][10] * (nv14 - 0.0));
        let eq18_e402_d_b11: f64 = (s.db[614][11] * (nv14 - 0.0));
        let eq18_e402_d_b12: f64 = (s.db[614][12] * (nv14 - 0.0));
        let eq18_e402_d_b13: f64 = (s.db[614][13] * (nv14 - 0.0));
        let eq18_e402_d_b14: f64 = (s.db[614][14] * (nv14 - 0.0));
        let eq18_e402_d_b15: f64 = (s.db[614][15] * (nv14 - 0.0));
        let eq18_value: f64 = eq18_e402;
        let eq18_node_derivatives: [f64; 19] = [eq18_e402_d_n0, eq18_e402_d_n1, eq18_e402_d_n2, eq18_e402_d_n3, eq18_e402_d_n4, eq18_e402_d_n5, eq18_e402_d_n6, eq18_e402_d_n7, eq18_e402_d_n8, eq18_e402_d_n9, eq18_e402_d_n10, eq18_e402_d_n11, eq18_e402_d_n12, eq18_e402_d_n13, eq18_e402_d_n14, eq18_e402_d_n15, eq18_e402_d_n16, eq18_e402_d_n17, eq18_e402_d_n18];
        let eq18_branch_derivatives: [f64; 16] = [eq18_e402_d_b0, eq18_e402_d_b1, eq18_e402_d_b2, eq18_e402_d_b3, eq18_e402_d_b4, eq18_e402_d_b5, eq18_e402_d_b6, eq18_e402_d_b7, eq18_e402_d_b8, eq18_e402_d_b9, eq18_e402_d_b10, eq18_e402_d_b11, eq18_e402_d_b12, eq18_e402_d_b13, eq18_e402_d_b14, eq18_e402_d_b15];
        stamper.stamp_current_dense_local(
            Some(6),
            Some(7),
            multiplicity * (eq18_value),
            &eq18_node_derivatives,
            &eq18_branch_derivatives,
            multiplicity,
        );
        let eq19_e405: f64 = ((nv14 - 0.0) * s.v[617]);
        let eq19_e405_d_n0: f64 = ((nv14 - 0.0) * s.dn[617][0]);
        let eq19_e405_d_n1: f64 = ((nv14 - 0.0) * s.dn[617][1]);
        let eq19_e405_d_n2: f64 = ((nv14 - 0.0) * s.dn[617][2]);
        let eq19_e405_d_n3: f64 = ((nv14 - 0.0) * s.dn[617][3]);
        let eq19_e405_d_n4: f64 = ((nv14 - 0.0) * s.dn[617][4]);
        let eq19_e405_d_n5: f64 = ((nv14 - 0.0) * s.dn[617][5]);
        let eq19_e405_d_n6: f64 = ((nv14 - 0.0) * s.dn[617][6]);
        let eq19_e405_d_n7: f64 = ((nv14 - 0.0) * s.dn[617][7]);
        let eq19_e405_d_n8: f64 = ((nv14 - 0.0) * s.dn[617][8]);
        let eq19_e405_d_n9: f64 = ((nv14 - 0.0) * s.dn[617][9]);
        let eq19_e405_d_n10: f64 = ((nv14 - 0.0) * s.dn[617][10]);
        let eq19_e405_d_n11: f64 = ((nv14 - 0.0) * s.dn[617][11]);
        let eq19_e405_d_n12: f64 = ((nv14 - 0.0) * s.dn[617][12]);
        let eq19_e405_d_n13: f64 = ((nv14 - 0.0) * s.dn[617][13]);
        let eq19_e405_d_n14: f64 = (s.v[617] + ((nv14 - 0.0) * s.dn[617][14]));
        let eq19_e405_d_n15: f64 = ((nv14 - 0.0) * s.dn[617][15]);
        let eq19_e405_d_n16: f64 = ((nv14 - 0.0) * s.dn[617][16]);
        let eq19_e405_d_n17: f64 = ((nv14 - 0.0) * s.dn[617][17]);
        let eq19_e405_d_n18: f64 = ((nv14 - 0.0) * s.dn[617][18]);
        let eq19_e405_d_b0: f64 = ((nv14 - 0.0) * s.db[617][0]);
        let eq19_e405_d_b1: f64 = ((nv14 - 0.0) * s.db[617][1]);
        let eq19_e405_d_b2: f64 = ((nv14 - 0.0) * s.db[617][2]);
        let eq19_e405_d_b3: f64 = ((nv14 - 0.0) * s.db[617][3]);
        let eq19_e405_d_b4: f64 = ((nv14 - 0.0) * s.db[617][4]);
        let eq19_e405_d_b5: f64 = ((nv14 - 0.0) * s.db[617][5]);
        let eq19_e405_d_b6: f64 = ((nv14 - 0.0) * s.db[617][6]);
        let eq19_e405_d_b7: f64 = ((nv14 - 0.0) * s.db[617][7]);
        let eq19_e405_d_b8: f64 = ((nv14 - 0.0) * s.db[617][8]);
        let eq19_e405_d_b9: f64 = ((nv14 - 0.0) * s.db[617][9]);
        let eq19_e405_d_b10: f64 = ((nv14 - 0.0) * s.db[617][10]);
        let eq19_e405_d_b11: f64 = ((nv14 - 0.0) * s.db[617][11]);
        let eq19_e405_d_b12: f64 = ((nv14 - 0.0) * s.db[617][12]);
        let eq19_e405_d_b13: f64 = ((nv14 - 0.0) * s.db[617][13]);
        let eq19_e405_d_b14: f64 = ((nv14 - 0.0) * s.db[617][14]);
        let eq19_e405_d_b15: f64 = ((nv14 - 0.0) * s.db[617][15]);
        let eq19_e406: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 3, eq19_e405);
        let eq19_e406_d_n0: f64 = (eq19_e405_d_n0 * ddt_scale);
        let eq19_e406_d_n1: f64 = (eq19_e405_d_n1 * ddt_scale);
        let eq19_e406_d_n2: f64 = (eq19_e405_d_n2 * ddt_scale);
        let eq19_e406_d_n3: f64 = (eq19_e405_d_n3 * ddt_scale);
        let eq19_e406_d_n4: f64 = (eq19_e405_d_n4 * ddt_scale);
        let eq19_e406_d_n5: f64 = (eq19_e405_d_n5 * ddt_scale);
        let eq19_e406_d_n6: f64 = (eq19_e405_d_n6 * ddt_scale);
        let eq19_e406_d_n7: f64 = (eq19_e405_d_n7 * ddt_scale);
        let eq19_e406_d_n8: f64 = (eq19_e405_d_n8 * ddt_scale);
        let eq19_e406_d_n9: f64 = (eq19_e405_d_n9 * ddt_scale);
        let eq19_e406_d_n10: f64 = (eq19_e405_d_n10 * ddt_scale);
        let eq19_e406_d_n11: f64 = (eq19_e405_d_n11 * ddt_scale);
        let eq19_e406_d_n12: f64 = (eq19_e405_d_n12 * ddt_scale);
        let eq19_e406_d_n13: f64 = (eq19_e405_d_n13 * ddt_scale);
        let eq19_e406_d_n14: f64 = (eq19_e405_d_n14 * ddt_scale);
        let eq19_e406_d_n15: f64 = (eq19_e405_d_n15 * ddt_scale);
        let eq19_e406_d_n16: f64 = (eq19_e405_d_n16 * ddt_scale);
        let eq19_e406_d_n17: f64 = (eq19_e405_d_n17 * ddt_scale);
        let eq19_e406_d_n18: f64 = (eq19_e405_d_n18 * ddt_scale);
        let eq19_e406_d_b0: f64 = (eq19_e405_d_b0 * ddt_scale);
        let eq19_e406_d_b1: f64 = (eq19_e405_d_b1 * ddt_scale);
        let eq19_e406_d_b2: f64 = (eq19_e405_d_b2 * ddt_scale);
        let eq19_e406_d_b3: f64 = (eq19_e405_d_b3 * ddt_scale);
        let eq19_e406_d_b4: f64 = (eq19_e405_d_b4 * ddt_scale);
        let eq19_e406_d_b5: f64 = (eq19_e405_d_b5 * ddt_scale);
        let eq19_e406_d_b6: f64 = (eq19_e405_d_b6 * ddt_scale);
        let eq19_e406_d_b7: f64 = (eq19_e405_d_b7 * ddt_scale);
        let eq19_e406_d_b8: f64 = (eq19_e405_d_b8 * ddt_scale);
        let eq19_e406_d_b9: f64 = (eq19_e405_d_b9 * ddt_scale);
        let eq19_e406_d_b10: f64 = (eq19_e405_d_b10 * ddt_scale);
        let eq19_e406_d_b11: f64 = (eq19_e405_d_b11 * ddt_scale);
        let eq19_e406_d_b12: f64 = (eq19_e405_d_b12 * ddt_scale);
        let eq19_e406_d_b13: f64 = (eq19_e405_d_b13 * ddt_scale);
        let eq19_e406_d_b14: f64 = (eq19_e405_d_b14 * ddt_scale);
        let eq19_e406_d_b15: f64 = (eq19_e405_d_b15 * ddt_scale);
        let eq19_value: f64 = eq19_e406;
        let eq19_node_derivatives: [f64; 19] = [eq19_e406_d_n0, eq19_e406_d_n1, eq19_e406_d_n2, eq19_e406_d_n3, eq19_e406_d_n4, eq19_e406_d_n5, eq19_e406_d_n6, eq19_e406_d_n7, eq19_e406_d_n8, eq19_e406_d_n9, eq19_e406_d_n10, eq19_e406_d_n11, eq19_e406_d_n12, eq19_e406_d_n13, eq19_e406_d_n14, eq19_e406_d_n15, eq19_e406_d_n16, eq19_e406_d_n17, eq19_e406_d_n18];
        let eq19_branch_derivatives: [f64; 16] = [eq19_e406_d_b0, eq19_e406_d_b1, eq19_e406_d_b2, eq19_e406_d_b3, eq19_e406_d_b4, eq19_e406_d_b5, eq19_e406_d_b6, eq19_e406_d_b7, eq19_e406_d_b8, eq19_e406_d_b9, eq19_e406_d_b10, eq19_e406_d_b11, eq19_e406_d_b12, eq19_e406_d_b13, eq19_e406_d_b14, eq19_e406_d_b15];
        stamper.stamp_current_dense_local(
            Some(11),
            Some(7),
            multiplicity * (eq19_value),
            &eq19_node_derivatives,
            &eq19_branch_derivatives,
            multiplicity,
        );
        let eq20_e409: f64 = ((nv14 - 0.0) * s.v[618]);
        let eq20_e409_d_n0: f64 = ((nv14 - 0.0) * s.dn[618][0]);
        let eq20_e409_d_n1: f64 = ((nv14 - 0.0) * s.dn[618][1]);
        let eq20_e409_d_n2: f64 = ((nv14 - 0.0) * s.dn[618][2]);
        let eq20_e409_d_n3: f64 = ((nv14 - 0.0) * s.dn[618][3]);
        let eq20_e409_d_n4: f64 = ((nv14 - 0.0) * s.dn[618][4]);
        let eq20_e409_d_n5: f64 = ((nv14 - 0.0) * s.dn[618][5]);
        let eq20_e409_d_n6: f64 = ((nv14 - 0.0) * s.dn[618][6]);
        let eq20_e409_d_n7: f64 = ((nv14 - 0.0) * s.dn[618][7]);
        let eq20_e409_d_n8: f64 = ((nv14 - 0.0) * s.dn[618][8]);
        let eq20_e409_d_n9: f64 = ((nv14 - 0.0) * s.dn[618][9]);
        let eq20_e409_d_n10: f64 = ((nv14 - 0.0) * s.dn[618][10]);
        let eq20_e409_d_n11: f64 = ((nv14 - 0.0) * s.dn[618][11]);
        let eq20_e409_d_n12: f64 = ((nv14 - 0.0) * s.dn[618][12]);
        let eq20_e409_d_n13: f64 = ((nv14 - 0.0) * s.dn[618][13]);
        let eq20_e409_d_n14: f64 = (s.v[618] + ((nv14 - 0.0) * s.dn[618][14]));
        let eq20_e409_d_n15: f64 = ((nv14 - 0.0) * s.dn[618][15]);
        let eq20_e409_d_n16: f64 = ((nv14 - 0.0) * s.dn[618][16]);
        let eq20_e409_d_n17: f64 = ((nv14 - 0.0) * s.dn[618][17]);
        let eq20_e409_d_n18: f64 = ((nv14 - 0.0) * s.dn[618][18]);
        let eq20_e409_d_b0: f64 = ((nv14 - 0.0) * s.db[618][0]);
        let eq20_e409_d_b1: f64 = ((nv14 - 0.0) * s.db[618][1]);
        let eq20_e409_d_b2: f64 = ((nv14 - 0.0) * s.db[618][2]);
        let eq20_e409_d_b3: f64 = ((nv14 - 0.0) * s.db[618][3]);
        let eq20_e409_d_b4: f64 = ((nv14 - 0.0) * s.db[618][4]);
        let eq20_e409_d_b5: f64 = ((nv14 - 0.0) * s.db[618][5]);
        let eq20_e409_d_b6: f64 = ((nv14 - 0.0) * s.db[618][6]);
        let eq20_e409_d_b7: f64 = ((nv14 - 0.0) * s.db[618][7]);
        let eq20_e409_d_b8: f64 = ((nv14 - 0.0) * s.db[618][8]);
        let eq20_e409_d_b9: f64 = ((nv14 - 0.0) * s.db[618][9]);
        let eq20_e409_d_b10: f64 = ((nv14 - 0.0) * s.db[618][10]);
        let eq20_e409_d_b11: f64 = ((nv14 - 0.0) * s.db[618][11]);
        let eq20_e409_d_b12: f64 = ((nv14 - 0.0) * s.db[618][12]);
        let eq20_e409_d_b13: f64 = ((nv14 - 0.0) * s.db[618][13]);
        let eq20_e409_d_b14: f64 = ((nv14 - 0.0) * s.db[618][14]);
        let eq20_e409_d_b15: f64 = ((nv14 - 0.0) * s.db[618][15]);
        let eq20_e410: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 4, eq20_e409);
        let eq20_e410_d_n0: f64 = (eq20_e409_d_n0 * ddt_scale);
        let eq20_e410_d_n1: f64 = (eq20_e409_d_n1 * ddt_scale);
        let eq20_e410_d_n2: f64 = (eq20_e409_d_n2 * ddt_scale);
        let eq20_e410_d_n3: f64 = (eq20_e409_d_n3 * ddt_scale);
        let eq20_e410_d_n4: f64 = (eq20_e409_d_n4 * ddt_scale);
        let eq20_e410_d_n5: f64 = (eq20_e409_d_n5 * ddt_scale);
        let eq20_e410_d_n6: f64 = (eq20_e409_d_n6 * ddt_scale);
        let eq20_e410_d_n7: f64 = (eq20_e409_d_n7 * ddt_scale);
        let eq20_e410_d_n8: f64 = (eq20_e409_d_n8 * ddt_scale);
        let eq20_e410_d_n9: f64 = (eq20_e409_d_n9 * ddt_scale);
        let eq20_e410_d_n10: f64 = (eq20_e409_d_n10 * ddt_scale);
        let eq20_e410_d_n11: f64 = (eq20_e409_d_n11 * ddt_scale);
        let eq20_e410_d_n12: f64 = (eq20_e409_d_n12 * ddt_scale);
        let eq20_e410_d_n13: f64 = (eq20_e409_d_n13 * ddt_scale);
        let eq20_e410_d_n14: f64 = (eq20_e409_d_n14 * ddt_scale);
        let eq20_e410_d_n15: f64 = (eq20_e409_d_n15 * ddt_scale);
        let eq20_e410_d_n16: f64 = (eq20_e409_d_n16 * ddt_scale);
        let eq20_e410_d_n17: f64 = (eq20_e409_d_n17 * ddt_scale);
        let eq20_e410_d_n18: f64 = (eq20_e409_d_n18 * ddt_scale);
        let eq20_e410_d_b0: f64 = (eq20_e409_d_b0 * ddt_scale);
        let eq20_e410_d_b1: f64 = (eq20_e409_d_b1 * ddt_scale);
        let eq20_e410_d_b2: f64 = (eq20_e409_d_b2 * ddt_scale);
        let eq20_e410_d_b3: f64 = (eq20_e409_d_b3 * ddt_scale);
        let eq20_e410_d_b4: f64 = (eq20_e409_d_b4 * ddt_scale);
        let eq20_e410_d_b5: f64 = (eq20_e409_d_b5 * ddt_scale);
        let eq20_e410_d_b6: f64 = (eq20_e409_d_b6 * ddt_scale);
        let eq20_e410_d_b7: f64 = (eq20_e409_d_b7 * ddt_scale);
        let eq20_e410_d_b8: f64 = (eq20_e409_d_b8 * ddt_scale);
        let eq20_e410_d_b9: f64 = (eq20_e409_d_b9 * ddt_scale);
        let eq20_e410_d_b10: f64 = (eq20_e409_d_b10 * ddt_scale);
        let eq20_e410_d_b11: f64 = (eq20_e409_d_b11 * ddt_scale);
        let eq20_e410_d_b12: f64 = (eq20_e409_d_b12 * ddt_scale);
        let eq20_e410_d_b13: f64 = (eq20_e409_d_b13 * ddt_scale);
        let eq20_e410_d_b14: f64 = (eq20_e409_d_b14 * ddt_scale);
        let eq20_e410_d_b15: f64 = (eq20_e409_d_b15 * ddt_scale);
        let eq20_value: f64 = eq20_e410;
        let eq20_node_derivatives: [f64; 19] = [eq20_e410_d_n0, eq20_e410_d_n1, eq20_e410_d_n2, eq20_e410_d_n3, eq20_e410_d_n4, eq20_e410_d_n5, eq20_e410_d_n6, eq20_e410_d_n7, eq20_e410_d_n8, eq20_e410_d_n9, eq20_e410_d_n10, eq20_e410_d_n11, eq20_e410_d_n12, eq20_e410_d_n13, eq20_e410_d_n14, eq20_e410_d_n15, eq20_e410_d_n16, eq20_e410_d_n17, eq20_e410_d_n18];
        let eq20_branch_derivatives: [f64; 16] = [eq20_e410_d_b0, eq20_e410_d_b1, eq20_e410_d_b2, eq20_e410_d_b3, eq20_e410_d_b4, eq20_e410_d_b5, eq20_e410_d_b6, eq20_e410_d_b7, eq20_e410_d_b8, eq20_e410_d_b9, eq20_e410_d_b10, eq20_e410_d_b11, eq20_e410_d_b12, eq20_e410_d_b13, eq20_e410_d_b14, eq20_e410_d_b15];
        stamper.stamp_current_dense_local(
            Some(11),
            Some(6),
            multiplicity * (eq20_value),
            &eq20_node_derivatives,
            &eq20_branch_derivatives,
            multiplicity,
        );
        let (eq26_e462, eq26_e462_d_n0, eq26_e462_d_n1, eq26_e462_d_n2, eq26_e462_d_n3, eq26_e462_d_n4, eq26_e462_d_n5, eq26_e462_d_n6, eq26_e462_d_n7, eq26_e462_d_n8, eq26_e462_d_n9, eq26_e462_d_n10, eq26_e462_d_n11, eq26_e462_d_n12, eq26_e462_d_n13, eq26_e462_d_n14, eq26_e462_d_n15, eq26_e462_d_n16, eq26_e462_d_n17, eq26_e462_d_n18, eq26_e462_d_b0, eq26_e462_d_b1, eq26_e462_d_b2, eq26_e462_d_b3, eq26_e462_d_b4, eq26_e462_d_b5, eq26_e462_d_b6, eq26_e462_d_b7, eq26_e462_d_b8, eq26_e462_d_b9, eq26_e462_d_b10, eq26_e462_d_b11, eq26_e462_d_b12, eq26_e462_d_b13, eq26_e462_d_b14, eq26_e462_d_b15,) = {
    if (p.p35 != 0.0) {
        let eq26_e460: f64 = (s.v[551] * (nv1 - nv11));
        let eq26_e460_d_n0: f64 = (s.dn[551][0] * (nv1 - nv11));
        let eq26_e460_d_n1: f64 = ((s.dn[551][1] * (nv1 - nv11)) + s.v[551]);
        let eq26_e460_d_n2: f64 = (s.dn[551][2] * (nv1 - nv11));
        let eq26_e460_d_n3: f64 = (s.dn[551][3] * (nv1 - nv11));
        let eq26_e460_d_n4: f64 = (s.dn[551][4] * (nv1 - nv11));
        let eq26_e460_d_n5: f64 = (s.dn[551][5] * (nv1 - nv11));
        let eq26_e460_d_n6: f64 = (s.dn[551][6] * (nv1 - nv11));
        let eq26_e460_d_n7: f64 = (s.dn[551][7] * (nv1 - nv11));
        let eq26_e460_d_n8: f64 = (s.dn[551][8] * (nv1 - nv11));
        let eq26_e460_d_n9: f64 = (s.dn[551][9] * (nv1 - nv11));
        let eq26_e460_d_n10: f64 = (s.dn[551][10] * (nv1 - nv11));
        let eq26_e460_d_n11: f64 = ((s.dn[551][11] * (nv1 - nv11)) + (-s.v[551]));
        let eq26_e460_d_n12: f64 = (s.dn[551][12] * (nv1 - nv11));
        let eq26_e460_d_n13: f64 = (s.dn[551][13] * (nv1 - nv11));
        let eq26_e460_d_n14: f64 = (s.dn[551][14] * (nv1 - nv11));
        let eq26_e460_d_n15: f64 = (s.dn[551][15] * (nv1 - nv11));
        let eq26_e460_d_n16: f64 = (s.dn[551][16] * (nv1 - nv11));
        let eq26_e460_d_n17: f64 = (s.dn[551][17] * (nv1 - nv11));
        let eq26_e460_d_n18: f64 = (s.dn[551][18] * (nv1 - nv11));
        let eq26_e460_d_b0: f64 = (s.db[551][0] * (nv1 - nv11));
        let eq26_e460_d_b1: f64 = (s.db[551][1] * (nv1 - nv11));
        let eq26_e460_d_b2: f64 = (s.db[551][2] * (nv1 - nv11));
        let eq26_e460_d_b3: f64 = (s.db[551][3] * (nv1 - nv11));
        let eq26_e460_d_b4: f64 = (s.db[551][4] * (nv1 - nv11));
        let eq26_e460_d_b5: f64 = (s.db[551][5] * (nv1 - nv11));
        let eq26_e460_d_b6: f64 = (s.db[551][6] * (nv1 - nv11));
        let eq26_e460_d_b7: f64 = (s.db[551][7] * (nv1 - nv11));
        let eq26_e460_d_b8: f64 = (s.db[551][8] * (nv1 - nv11));
        let eq26_e460_d_b9: f64 = (s.db[551][9] * (nv1 - nv11));
        let eq26_e460_d_b10: f64 = (s.db[551][10] * (nv1 - nv11));
        let eq26_e460_d_b11: f64 = (s.db[551][11] * (nv1 - nv11));
        let eq26_e460_d_b12: f64 = (s.db[551][12] * (nv1 - nv11));
        let eq26_e460_d_b13: f64 = (s.db[551][13] * (nv1 - nv11));
        let eq26_e460_d_b14: f64 = (s.db[551][14] * (nv1 - nv11));
        let eq26_e460_d_b15: f64 = (s.db[551][15] * (nv1 - nv11));
        (eq26_e460, eq26_e460_d_n0, eq26_e460_d_n1, eq26_e460_d_n2, eq26_e460_d_n3, eq26_e460_d_n4, eq26_e460_d_n5, eq26_e460_d_n6, eq26_e460_d_n7, eq26_e460_d_n8, eq26_e460_d_n9, eq26_e460_d_n10, eq26_e460_d_n11, eq26_e460_d_n12, eq26_e460_d_n13, eq26_e460_d_n14, eq26_e460_d_n15, eq26_e460_d_n16, eq26_e460_d_n17, eq26_e460_d_n18, eq26_e460_d_b0, eq26_e460_d_b1, eq26_e460_d_b2, eq26_e460_d_b3, eq26_e460_d_b4, eq26_e460_d_b5, eq26_e460_d_b6, eq26_e460_d_b7, eq26_e460_d_b8, eq26_e460_d_b9, eq26_e460_d_b10, eq26_e460_d_b11, eq26_e460_d_b12, eq26_e460_d_b13, eq26_e460_d_b14, eq26_e460_d_b15,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq26_value: f64 = eq26_e462;
        let eq26_node_derivatives: [f64; 19] = [eq26_e462_d_n0, eq26_e462_d_n1, eq26_e462_d_n2, eq26_e462_d_n3, eq26_e462_d_n4, eq26_e462_d_n5, eq26_e462_d_n6, eq26_e462_d_n7, eq26_e462_d_n8, eq26_e462_d_n9, eq26_e462_d_n10, eq26_e462_d_n11, eq26_e462_d_n12, eq26_e462_d_n13, eq26_e462_d_n14, eq26_e462_d_n15, eq26_e462_d_n16, eq26_e462_d_n17, eq26_e462_d_n18];
        let eq26_branch_derivatives: [f64; 16] = [eq26_e462_d_b0, eq26_e462_d_b1, eq26_e462_d_b2, eq26_e462_d_b3, eq26_e462_d_b4, eq26_e462_d_b5, eq26_e462_d_b6, eq26_e462_d_b7, eq26_e462_d_b8, eq26_e462_d_b9, eq26_e462_d_b10, eq26_e462_d_b11, eq26_e462_d_b12, eq26_e462_d_b13, eq26_e462_d_b14, eq26_e462_d_b15];
        stamper.stamp_current_dense_local(
            Some(1),
            Some(11),
            multiplicity * (eq26_value),
            &eq26_node_derivatives,
            &eq26_branch_derivatives,
            multiplicity,
        );
        let (eq28_e473, eq28_e473_d_n0, eq28_e473_d_n1, eq28_e473_d_n2, eq28_e473_d_n3, eq28_e473_d_n4, eq28_e473_d_n5, eq28_e473_d_n6, eq28_e473_d_n7, eq28_e473_d_n8, eq28_e473_d_n9, eq28_e473_d_n10, eq28_e473_d_n11, eq28_e473_d_n12, eq28_e473_d_n13, eq28_e473_d_n14, eq28_e473_d_n15, eq28_e473_d_n16, eq28_e473_d_n17, eq28_e473_d_n18, eq28_e473_d_b0, eq28_e473_d_b1, eq28_e473_d_b2, eq28_e473_d_b3, eq28_e473_d_b4, eq28_e473_d_b5, eq28_e473_d_b6, eq28_e473_d_b7, eq28_e473_d_b8, eq28_e473_d_b9, eq28_e473_d_b10, eq28_e473_d_b11, eq28_e473_d_b12, eq28_e473_d_b13, eq28_e473_d_b14, eq28_e473_d_b15,) = {
    if s.b[1850] {
        let eq28_e471: f64 = ((nv10 - 0.0) * s.v[589]);
        let eq28_e471_d_n0: f64 = ((nv10 - 0.0) * s.dn[589][0]);
        let eq28_e471_d_n1: f64 = ((nv10 - 0.0) * s.dn[589][1]);
        let eq28_e471_d_n2: f64 = ((nv10 - 0.0) * s.dn[589][2]);
        let eq28_e471_d_n3: f64 = ((nv10 - 0.0) * s.dn[589][3]);
        let eq28_e471_d_n4: f64 = ((nv10 - 0.0) * s.dn[589][4]);
        let eq28_e471_d_n5: f64 = ((nv10 - 0.0) * s.dn[589][5]);
        let eq28_e471_d_n6: f64 = ((nv10 - 0.0) * s.dn[589][6]);
        let eq28_e471_d_n7: f64 = ((nv10 - 0.0) * s.dn[589][7]);
        let eq28_e471_d_n8: f64 = ((nv10 - 0.0) * s.dn[589][8]);
        let eq28_e471_d_n9: f64 = ((nv10 - 0.0) * s.dn[589][9]);
        let eq28_e471_d_n10: f64 = (s.v[589] + ((nv10 - 0.0) * s.dn[589][10]));
        let eq28_e471_d_n11: f64 = ((nv10 - 0.0) * s.dn[589][11]);
        let eq28_e471_d_n12: f64 = ((nv10 - 0.0) * s.dn[589][12]);
        let eq28_e471_d_n13: f64 = ((nv10 - 0.0) * s.dn[589][13]);
        let eq28_e471_d_n14: f64 = ((nv10 - 0.0) * s.dn[589][14]);
        let eq28_e471_d_n15: f64 = ((nv10 - 0.0) * s.dn[589][15]);
        let eq28_e471_d_n16: f64 = ((nv10 - 0.0) * s.dn[589][16]);
        let eq28_e471_d_n17: f64 = ((nv10 - 0.0) * s.dn[589][17]);
        let eq28_e471_d_n18: f64 = ((nv10 - 0.0) * s.dn[589][18]);
        let eq28_e471_d_b0: f64 = ((nv10 - 0.0) * s.db[589][0]);
        let eq28_e471_d_b1: f64 = ((nv10 - 0.0) * s.db[589][1]);
        let eq28_e471_d_b2: f64 = ((nv10 - 0.0) * s.db[589][2]);
        let eq28_e471_d_b3: f64 = ((nv10 - 0.0) * s.db[589][3]);
        let eq28_e471_d_b4: f64 = ((nv10 - 0.0) * s.db[589][4]);
        let eq28_e471_d_b5: f64 = ((nv10 - 0.0) * s.db[589][5]);
        let eq28_e471_d_b6: f64 = ((nv10 - 0.0) * s.db[589][6]);
        let eq28_e471_d_b7: f64 = ((nv10 - 0.0) * s.db[589][7]);
        let eq28_e471_d_b8: f64 = ((nv10 - 0.0) * s.db[589][8]);
        let eq28_e471_d_b9: f64 = ((nv10 - 0.0) * s.db[589][9]);
        let eq28_e471_d_b10: f64 = ((nv10 - 0.0) * s.db[589][10]);
        let eq28_e471_d_b11: f64 = ((nv10 - 0.0) * s.db[589][11]);
        let eq28_e471_d_b12: f64 = ((nv10 - 0.0) * s.db[589][12]);
        let eq28_e471_d_b13: f64 = ((nv10 - 0.0) * s.db[589][13]);
        let eq28_e471_d_b14: f64 = ((nv10 - 0.0) * s.db[589][14]);
        let eq28_e471_d_b15: f64 = ((nv10 - 0.0) * s.db[589][15]);
        (eq28_e471, eq28_e471_d_n0, eq28_e471_d_n1, eq28_e471_d_n2, eq28_e471_d_n3, eq28_e471_d_n4, eq28_e471_d_n5, eq28_e471_d_n6, eq28_e471_d_n7, eq28_e471_d_n8, eq28_e471_d_n9, eq28_e471_d_n10, eq28_e471_d_n11, eq28_e471_d_n12, eq28_e471_d_n13, eq28_e471_d_n14, eq28_e471_d_n15, eq28_e471_d_n16, eq28_e471_d_n17, eq28_e471_d_n18, eq28_e471_d_b0, eq28_e471_d_b1, eq28_e471_d_b2, eq28_e471_d_b3, eq28_e471_d_b4, eq28_e471_d_b5, eq28_e471_d_b6, eq28_e471_d_b7, eq28_e471_d_b8, eq28_e471_d_b9, eq28_e471_d_b10, eq28_e471_d_b11, eq28_e471_d_b12, eq28_e471_d_b13, eq28_e471_d_b14, eq28_e471_d_b15,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq28_value: f64 = eq28_e473;
        let eq28_node_derivatives: [f64; 19] = [eq28_e473_d_n0, eq28_e473_d_n1, eq28_e473_d_n2, eq28_e473_d_n3, eq28_e473_d_n4, eq28_e473_d_n5, eq28_e473_d_n6, eq28_e473_d_n7, eq28_e473_d_n8, eq28_e473_d_n9, eq28_e473_d_n10, eq28_e473_d_n11, eq28_e473_d_n12, eq28_e473_d_n13, eq28_e473_d_n14, eq28_e473_d_n15, eq28_e473_d_n16, eq28_e473_d_n17, eq28_e473_d_n18];
        let eq28_branch_derivatives: [f64; 16] = [eq28_e473_d_b0, eq28_e473_d_b1, eq28_e473_d_b2, eq28_e473_d_b3, eq28_e473_d_b4, eq28_e473_d_b5, eq28_e473_d_b6, eq28_e473_d_b7, eq28_e473_d_b8, eq28_e473_d_b9, eq28_e473_d_b10, eq28_e473_d_b11, eq28_e473_d_b12, eq28_e473_d_b13, eq28_e473_d_b14, eq28_e473_d_b15];
        stamper.stamp_current_dense_local(
            Some(10),
            None,
            multiplicity * (eq28_value),
            &eq28_node_derivatives,
            &eq28_branch_derivatives,
            multiplicity,
        );
        let (eq29_e478, eq29_e478_d_n0, eq29_e478_d_n1, eq29_e478_d_n2, eq29_e478_d_n3, eq29_e478_d_n4, eq29_e478_d_n5, eq29_e478_d_n6, eq29_e478_d_n7, eq29_e478_d_n8, eq29_e478_d_n9, eq29_e478_d_n10, eq29_e478_d_n11, eq29_e478_d_n12, eq29_e478_d_n13, eq29_e478_d_n14, eq29_e478_d_n15, eq29_e478_d_n16, eq29_e478_d_n17, eq29_e478_d_n18, eq29_e478_d_b0, eq29_e478_d_b1, eq29_e478_d_b2, eq29_e478_d_b3, eq29_e478_d_b4, eq29_e478_d_b5, eq29_e478_d_b6, eq29_e478_d_b7, eq29_e478_d_b8, eq29_e478_d_b9, eq29_e478_d_b10, eq29_e478_d_b11, eq29_e478_d_b12, eq29_e478_d_b13, eq29_e478_d_b14, eq29_e478_d_b15,) = {
    if s.b[1850] {
        let eq29_e476: f64 = (-s.v[595]);
        let eq29_e476_d_n0: f64 = (-s.dn[595][0]);
        let eq29_e476_d_n1: f64 = (-s.dn[595][1]);
        let eq29_e476_d_n2: f64 = (-s.dn[595][2]);
        let eq29_e476_d_n3: f64 = (-s.dn[595][3]);
        let eq29_e476_d_n4: f64 = (-s.dn[595][4]);
        let eq29_e476_d_n5: f64 = (-s.dn[595][5]);
        let eq29_e476_d_n6: f64 = (-s.dn[595][6]);
        let eq29_e476_d_n7: f64 = (-s.dn[595][7]);
        let eq29_e476_d_n8: f64 = (-s.dn[595][8]);
        let eq29_e476_d_n9: f64 = (-s.dn[595][9]);
        let eq29_e476_d_n10: f64 = (-s.dn[595][10]);
        let eq29_e476_d_n11: f64 = (-s.dn[595][11]);
        let eq29_e476_d_n12: f64 = (-s.dn[595][12]);
        let eq29_e476_d_n13: f64 = (-s.dn[595][13]);
        let eq29_e476_d_n14: f64 = (-s.dn[595][14]);
        let eq29_e476_d_n15: f64 = (-s.dn[595][15]);
        let eq29_e476_d_n16: f64 = (-s.dn[595][16]);
        let eq29_e476_d_n17: f64 = (-s.dn[595][17]);
        let eq29_e476_d_n18: f64 = (-s.dn[595][18]);
        let eq29_e476_d_b0: f64 = (-s.db[595][0]);
        let eq29_e476_d_b1: f64 = (-s.db[595][1]);
        let eq29_e476_d_b2: f64 = (-s.db[595][2]);
        let eq29_e476_d_b3: f64 = (-s.db[595][3]);
        let eq29_e476_d_b4: f64 = (-s.db[595][4]);
        let eq29_e476_d_b5: f64 = (-s.db[595][5]);
        let eq29_e476_d_b6: f64 = (-s.db[595][6]);
        let eq29_e476_d_b7: f64 = (-s.db[595][7]);
        let eq29_e476_d_b8: f64 = (-s.db[595][8]);
        let eq29_e476_d_b9: f64 = (-s.db[595][9]);
        let eq29_e476_d_b10: f64 = (-s.db[595][10]);
        let eq29_e476_d_b11: f64 = (-s.db[595][11]);
        let eq29_e476_d_b12: f64 = (-s.db[595][12]);
        let eq29_e476_d_b13: f64 = (-s.db[595][13]);
        let eq29_e476_d_b14: f64 = (-s.db[595][14]);
        let eq29_e476_d_b15: f64 = (-s.db[595][15]);
        (eq29_e476, eq29_e476_d_n0, eq29_e476_d_n1, eq29_e476_d_n2, eq29_e476_d_n3, eq29_e476_d_n4, eq29_e476_d_n5, eq29_e476_d_n6, eq29_e476_d_n7, eq29_e476_d_n8, eq29_e476_d_n9, eq29_e476_d_n10, eq29_e476_d_n11, eq29_e476_d_n12, eq29_e476_d_n13, eq29_e476_d_n14, eq29_e476_d_n15, eq29_e476_d_n16, eq29_e476_d_n17, eq29_e476_d_n18, eq29_e476_d_b0, eq29_e476_d_b1, eq29_e476_d_b2, eq29_e476_d_b3, eq29_e476_d_b4, eq29_e476_d_b5, eq29_e476_d_b6, eq29_e476_d_b7, eq29_e476_d_b8, eq29_e476_d_b9, eq29_e476_d_b10, eq29_e476_d_b11, eq29_e476_d_b12, eq29_e476_d_b13, eq29_e476_d_b14, eq29_e476_d_b15,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq29_value: f64 = eq29_e478;
        let eq29_node_derivatives: [f64; 19] = [eq29_e478_d_n0, eq29_e478_d_n1, eq29_e478_d_n2, eq29_e478_d_n3, eq29_e478_d_n4, eq29_e478_d_n5, eq29_e478_d_n6, eq29_e478_d_n7, eq29_e478_d_n8, eq29_e478_d_n9, eq29_e478_d_n10, eq29_e478_d_n11, eq29_e478_d_n12, eq29_e478_d_n13, eq29_e478_d_n14, eq29_e478_d_n15, eq29_e478_d_n16, eq29_e478_d_n17, eq29_e478_d_n18];
        let eq29_branch_derivatives: [f64; 16] = [eq29_e478_d_b0, eq29_e478_d_b1, eq29_e478_d_b2, eq29_e478_d_b3, eq29_e478_d_b4, eq29_e478_d_b5, eq29_e478_d_b6, eq29_e478_d_b7, eq29_e478_d_b8, eq29_e478_d_b9, eq29_e478_d_b10, eq29_e478_d_b11, eq29_e478_d_b12, eq29_e478_d_b13, eq29_e478_d_b14, eq29_e478_d_b15];
        stamper.stamp_current_dense_local(
            Some(10),
            None,
            multiplicity * (eq29_value),
            &eq29_node_derivatives,
            &eq29_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_transient_equations_block_2(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        multiplicity: f64,
        ddt_active: bool,
        ddt_scale: f64,
        ddt_state_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_initialized: &mut [bool; Instance::DDT_STATE_COUNT],
    ) {
        let nv10 = ctx.node_voltage(nodes[10]);
        let (eq31_e491, eq31_e491_d_n0, eq31_e491_d_n1, eq31_e491_d_n2, eq31_e491_d_n3, eq31_e491_d_n4, eq31_e491_d_n5, eq31_e491_d_n6, eq31_e491_d_n7, eq31_e491_d_n8, eq31_e491_d_n9, eq31_e491_d_n10, eq31_e491_d_n11, eq31_e491_d_n12, eq31_e491_d_n13, eq31_e491_d_n14, eq31_e491_d_n15, eq31_e491_d_n16, eq31_e491_d_n17, eq31_e491_d_n18, eq31_e491_d_b0, eq31_e491_d_b1, eq31_e491_d_b2, eq31_e491_d_b3, eq31_e491_d_b4, eq31_e491_d_b5, eq31_e491_d_b6, eq31_e491_d_b7, eq31_e491_d_b8, eq31_e491_d_b9, eq31_e491_d_b10, eq31_e491_d_b11, eq31_e491_d_b12, eq31_e491_d_b13, eq31_e491_d_b14, eq31_e491_d_b15,) = {
    if s.b[1850] {
        let eq31_e488: f64 = (s.v[563] * (nv10 - 0.0));
        let eq31_e488_d_n0: f64 = (s.dn[563][0] * (nv10 - 0.0));
        let eq31_e488_d_n1: f64 = (s.dn[563][1] * (nv10 - 0.0));
        let eq31_e488_d_n2: f64 = (s.dn[563][2] * (nv10 - 0.0));
        let eq31_e488_d_n3: f64 = (s.dn[563][3] * (nv10 - 0.0));
        let eq31_e488_d_n4: f64 = (s.dn[563][4] * (nv10 - 0.0));
        let eq31_e488_d_n5: f64 = (s.dn[563][5] * (nv10 - 0.0));
        let eq31_e488_d_n6: f64 = (s.dn[563][6] * (nv10 - 0.0));
        let eq31_e488_d_n7: f64 = (s.dn[563][7] * (nv10 - 0.0));
        let eq31_e488_d_n8: f64 = (s.dn[563][8] * (nv10 - 0.0));
        let eq31_e488_d_n9: f64 = (s.dn[563][9] * (nv10 - 0.0));
        let eq31_e488_d_n10: f64 = ((s.dn[563][10] * (nv10 - 0.0)) + s.v[563]);
        let eq31_e488_d_n11: f64 = (s.dn[563][11] * (nv10 - 0.0));
        let eq31_e488_d_n12: f64 = (s.dn[563][12] * (nv10 - 0.0));
        let eq31_e488_d_n13: f64 = (s.dn[563][13] * (nv10 - 0.0));
        let eq31_e488_d_n14: f64 = (s.dn[563][14] * (nv10 - 0.0));
        let eq31_e488_d_n15: f64 = (s.dn[563][15] * (nv10 - 0.0));
        let eq31_e488_d_n16: f64 = (s.dn[563][16] * (nv10 - 0.0));
        let eq31_e488_d_n17: f64 = (s.dn[563][17] * (nv10 - 0.0));
        let eq31_e488_d_n18: f64 = (s.dn[563][18] * (nv10 - 0.0));
        let eq31_e488_d_b0: f64 = (s.db[563][0] * (nv10 - 0.0));
        let eq31_e488_d_b1: f64 = (s.db[563][1] * (nv10 - 0.0));
        let eq31_e488_d_b2: f64 = (s.db[563][2] * (nv10 - 0.0));
        let eq31_e488_d_b3: f64 = (s.db[563][3] * (nv10 - 0.0));
        let eq31_e488_d_b4: f64 = (s.db[563][4] * (nv10 - 0.0));
        let eq31_e488_d_b5: f64 = (s.db[563][5] * (nv10 - 0.0));
        let eq31_e488_d_b6: f64 = (s.db[563][6] * (nv10 - 0.0));
        let eq31_e488_d_b7: f64 = (s.db[563][7] * (nv10 - 0.0));
        let eq31_e488_d_b8: f64 = (s.db[563][8] * (nv10 - 0.0));
        let eq31_e488_d_b9: f64 = (s.db[563][9] * (nv10 - 0.0));
        let eq31_e488_d_b10: f64 = (s.db[563][10] * (nv10 - 0.0));
        let eq31_e488_d_b11: f64 = (s.db[563][11] * (nv10 - 0.0));
        let eq31_e488_d_b12: f64 = (s.db[563][12] * (nv10 - 0.0));
        let eq31_e488_d_b13: f64 = (s.db[563][13] * (nv10 - 0.0));
        let eq31_e488_d_b14: f64 = (s.db[563][14] * (nv10 - 0.0));
        let eq31_e488_d_b15: f64 = (s.db[563][15] * (nv10 - 0.0));
        let eq31_e489: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 5, eq31_e488);
        let eq31_e489_d_n0: f64 = (eq31_e488_d_n0 * ddt_scale);
        let eq31_e489_d_n1: f64 = (eq31_e488_d_n1 * ddt_scale);
        let eq31_e489_d_n2: f64 = (eq31_e488_d_n2 * ddt_scale);
        let eq31_e489_d_n3: f64 = (eq31_e488_d_n3 * ddt_scale);
        let eq31_e489_d_n4: f64 = (eq31_e488_d_n4 * ddt_scale);
        let eq31_e489_d_n5: f64 = (eq31_e488_d_n5 * ddt_scale);
        let eq31_e489_d_n6: f64 = (eq31_e488_d_n6 * ddt_scale);
        let eq31_e489_d_n7: f64 = (eq31_e488_d_n7 * ddt_scale);
        let eq31_e489_d_n8: f64 = (eq31_e488_d_n8 * ddt_scale);
        let eq31_e489_d_n9: f64 = (eq31_e488_d_n9 * ddt_scale);
        let eq31_e489_d_n10: f64 = (eq31_e488_d_n10 * ddt_scale);
        let eq31_e489_d_n11: f64 = (eq31_e488_d_n11 * ddt_scale);
        let eq31_e489_d_n12: f64 = (eq31_e488_d_n12 * ddt_scale);
        let eq31_e489_d_n13: f64 = (eq31_e488_d_n13 * ddt_scale);
        let eq31_e489_d_n14: f64 = (eq31_e488_d_n14 * ddt_scale);
        let eq31_e489_d_n15: f64 = (eq31_e488_d_n15 * ddt_scale);
        let eq31_e489_d_n16: f64 = (eq31_e488_d_n16 * ddt_scale);
        let eq31_e489_d_n17: f64 = (eq31_e488_d_n17 * ddt_scale);
        let eq31_e489_d_n18: f64 = (eq31_e488_d_n18 * ddt_scale);
        let eq31_e489_d_b0: f64 = (eq31_e488_d_b0 * ddt_scale);
        let eq31_e489_d_b1: f64 = (eq31_e488_d_b1 * ddt_scale);
        let eq31_e489_d_b2: f64 = (eq31_e488_d_b2 * ddt_scale);
        let eq31_e489_d_b3: f64 = (eq31_e488_d_b3 * ddt_scale);
        let eq31_e489_d_b4: f64 = (eq31_e488_d_b4 * ddt_scale);
        let eq31_e489_d_b5: f64 = (eq31_e488_d_b5 * ddt_scale);
        let eq31_e489_d_b6: f64 = (eq31_e488_d_b6 * ddt_scale);
        let eq31_e489_d_b7: f64 = (eq31_e488_d_b7 * ddt_scale);
        let eq31_e489_d_b8: f64 = (eq31_e488_d_b8 * ddt_scale);
        let eq31_e489_d_b9: f64 = (eq31_e488_d_b9 * ddt_scale);
        let eq31_e489_d_b10: f64 = (eq31_e488_d_b10 * ddt_scale);
        let eq31_e489_d_b11: f64 = (eq31_e488_d_b11 * ddt_scale);
        let eq31_e489_d_b12: f64 = (eq31_e488_d_b12 * ddt_scale);
        let eq31_e489_d_b13: f64 = (eq31_e488_d_b13 * ddt_scale);
        let eq31_e489_d_b14: f64 = (eq31_e488_d_b14 * ddt_scale);
        let eq31_e489_d_b15: f64 = (eq31_e488_d_b15 * ddt_scale);
        (eq31_e489, eq31_e489_d_n0, eq31_e489_d_n1, eq31_e489_d_n2, eq31_e489_d_n3, eq31_e489_d_n4, eq31_e489_d_n5, eq31_e489_d_n6, eq31_e489_d_n7, eq31_e489_d_n8, eq31_e489_d_n9, eq31_e489_d_n10, eq31_e489_d_n11, eq31_e489_d_n12, eq31_e489_d_n13, eq31_e489_d_n14, eq31_e489_d_n15, eq31_e489_d_n16, eq31_e489_d_n17, eq31_e489_d_n18, eq31_e489_d_b0, eq31_e489_d_b1, eq31_e489_d_b2, eq31_e489_d_b3, eq31_e489_d_b4, eq31_e489_d_b5, eq31_e489_d_b6, eq31_e489_d_b7, eq31_e489_d_b8, eq31_e489_d_b9, eq31_e489_d_b10, eq31_e489_d_b11, eq31_e489_d_b12, eq31_e489_d_b13, eq31_e489_d_b14, eq31_e489_d_b15,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq31_value: f64 = eq31_e491;
        let eq31_node_derivatives: [f64; 19] = [eq31_e491_d_n0, eq31_e491_d_n1, eq31_e491_d_n2, eq31_e491_d_n3, eq31_e491_d_n4, eq31_e491_d_n5, eq31_e491_d_n6, eq31_e491_d_n7, eq31_e491_d_n8, eq31_e491_d_n9, eq31_e491_d_n10, eq31_e491_d_n11, eq31_e491_d_n12, eq31_e491_d_n13, eq31_e491_d_n14, eq31_e491_d_n15, eq31_e491_d_n16, eq31_e491_d_n17, eq31_e491_d_n18];
        let eq31_branch_derivatives: [f64; 16] = [eq31_e491_d_b0, eq31_e491_d_b1, eq31_e491_d_b2, eq31_e491_d_b3, eq31_e491_d_b4, eq31_e491_d_b5, eq31_e491_d_b6, eq31_e491_d_b7, eq31_e491_d_b8, eq31_e491_d_b9, eq31_e491_d_b10, eq31_e491_d_b11, eq31_e491_d_b12, eq31_e491_d_b13, eq31_e491_d_b14, eq31_e491_d_b15];
        stamper.stamp_current_dense_local(
            Some(10),
            None,
            multiplicity * (eq31_value),
            &eq31_node_derivatives,
            &eq31_branch_derivatives,
            multiplicity,
        );
        let (eq33_e506, eq33_e506_d_n0, eq33_e506_d_n1, eq33_e506_d_n2, eq33_e506_d_n3, eq33_e506_d_n4, eq33_e506_d_n5, eq33_e506_d_n6, eq33_e506_d_n7, eq33_e506_d_n8, eq33_e506_d_n9, eq33_e506_d_n10, eq33_e506_d_n11, eq33_e506_d_n12, eq33_e506_d_n13, eq33_e506_d_n14, eq33_e506_d_n15, eq33_e506_d_n16, eq33_e506_d_n17, eq33_e506_d_n18, eq33_e506_d_b0, eq33_e506_d_b1, eq33_e506_d_b2, eq33_e506_d_b3, eq33_e506_d_b4, eq33_e506_d_b5, eq33_e506_d_b6, eq33_e506_d_b7, eq33_e506_d_b8, eq33_e506_d_b9, eq33_e506_d_b10, eq33_e506_d_b11, eq33_e506_d_b12, eq33_e506_d_b13, eq33_e506_d_b14, eq33_e506_d_b15,) = {
    if s.b[1851] {
        let eq33_e503: f64 = (s.v[311] + s.v[263]);
        let eq33_e503_d_n0: f64 = (s.dn[311][0] + s.dn[263][0]);
        let eq33_e503_d_n1: f64 = (s.dn[311][1] + s.dn[263][1]);
        let eq33_e503_d_n2: f64 = (s.dn[311][2] + s.dn[263][2]);
        let eq33_e503_d_n3: f64 = (s.dn[311][3] + s.dn[263][3]);
        let eq33_e503_d_n4: f64 = (s.dn[311][4] + s.dn[263][4]);
        let eq33_e503_d_n5: f64 = (s.dn[311][5] + s.dn[263][5]);
        let eq33_e503_d_n6: f64 = (s.dn[311][6] + s.dn[263][6]);
        let eq33_e503_d_n7: f64 = (s.dn[311][7] + s.dn[263][7]);
        let eq33_e503_d_n8: f64 = (s.dn[311][8] + s.dn[263][8]);
        let eq33_e503_d_n9: f64 = (s.dn[311][9] + s.dn[263][9]);
        let eq33_e503_d_n10: f64 = (s.dn[311][10] + s.dn[263][10]);
        let eq33_e503_d_n11: f64 = (s.dn[311][11] + s.dn[263][11]);
        let eq33_e503_d_n12: f64 = (s.dn[311][12] + s.dn[263][12]);
        let eq33_e503_d_n13: f64 = (s.dn[311][13] + s.dn[263][13]);
        let eq33_e503_d_n14: f64 = (s.dn[311][14] + s.dn[263][14]);
        let eq33_e503_d_n15: f64 = (s.dn[311][15] + s.dn[263][15]);
        let eq33_e503_d_n16: f64 = (s.dn[311][16] + s.dn[263][16]);
        let eq33_e503_d_n17: f64 = (s.dn[311][17] + s.dn[263][17]);
        let eq33_e503_d_n18: f64 = (s.dn[311][18] + s.dn[263][18]);
        let eq33_e503_d_b0: f64 = (s.db[311][0] + s.db[263][0]);
        let eq33_e503_d_b1: f64 = (s.db[311][1] + s.db[263][1]);
        let eq33_e503_d_b2: f64 = (s.db[311][2] + s.db[263][2]);
        let eq33_e503_d_b3: f64 = (s.db[311][3] + s.db[263][3]);
        let eq33_e503_d_b4: f64 = (s.db[311][4] + s.db[263][4]);
        let eq33_e503_d_b5: f64 = (s.db[311][5] + s.db[263][5]);
        let eq33_e503_d_b6: f64 = (s.db[311][6] + s.db[263][6]);
        let eq33_e503_d_b7: f64 = (s.db[311][7] + s.db[263][7]);
        let eq33_e503_d_b8: f64 = (s.db[311][8] + s.db[263][8]);
        let eq33_e503_d_b9: f64 = (s.db[311][9] + s.db[263][9]);
        let eq33_e503_d_b10: f64 = (s.db[311][10] + s.db[263][10]);
        let eq33_e503_d_b11: f64 = (s.db[311][11] + s.db[263][11]);
        let eq33_e503_d_b12: f64 = (s.db[311][12] + s.db[263][12]);
        let eq33_e503_d_b13: f64 = (s.db[311][13] + s.db[263][13]);
        let eq33_e503_d_b14: f64 = (s.db[311][14] + s.db[263][14]);
        let eq33_e503_d_b15: f64 = (s.db[311][15] + s.db[263][15]);
        let eq33_e504: f64 = (p.p50 * eq33_e503);
        let eq33_e504_d_n0: f64 = (p.p50 * eq33_e503_d_n0);
        let eq33_e504_d_n1: f64 = (p.p50 * eq33_e503_d_n1);
        let eq33_e504_d_n2: f64 = (p.p50 * eq33_e503_d_n2);
        let eq33_e504_d_n3: f64 = (p.p50 * eq33_e503_d_n3);
        let eq33_e504_d_n4: f64 = (p.p50 * eq33_e503_d_n4);
        let eq33_e504_d_n5: f64 = (p.p50 * eq33_e503_d_n5);
        let eq33_e504_d_n6: f64 = (p.p50 * eq33_e503_d_n6);
        let eq33_e504_d_n7: f64 = (p.p50 * eq33_e503_d_n7);
        let eq33_e504_d_n8: f64 = (p.p50 * eq33_e503_d_n8);
        let eq33_e504_d_n9: f64 = (p.p50 * eq33_e503_d_n9);
        let eq33_e504_d_n10: f64 = (p.p50 * eq33_e503_d_n10);
        let eq33_e504_d_n11: f64 = (p.p50 * eq33_e503_d_n11);
        let eq33_e504_d_n12: f64 = (p.p50 * eq33_e503_d_n12);
        let eq33_e504_d_n13: f64 = (p.p50 * eq33_e503_d_n13);
        let eq33_e504_d_n14: f64 = (p.p50 * eq33_e503_d_n14);
        let eq33_e504_d_n15: f64 = (p.p50 * eq33_e503_d_n15);
        let eq33_e504_d_n16: f64 = (p.p50 * eq33_e503_d_n16);
        let eq33_e504_d_n17: f64 = (p.p50 * eq33_e503_d_n17);
        let eq33_e504_d_n18: f64 = (p.p50 * eq33_e503_d_n18);
        let eq33_e504_d_b0: f64 = (p.p50 * eq33_e503_d_b0);
        let eq33_e504_d_b1: f64 = (p.p50 * eq33_e503_d_b1);
        let eq33_e504_d_b2: f64 = (p.p50 * eq33_e503_d_b2);
        let eq33_e504_d_b3: f64 = (p.p50 * eq33_e503_d_b3);
        let eq33_e504_d_b4: f64 = (p.p50 * eq33_e503_d_b4);
        let eq33_e504_d_b5: f64 = (p.p50 * eq33_e503_d_b5);
        let eq33_e504_d_b6: f64 = (p.p50 * eq33_e503_d_b6);
        let eq33_e504_d_b7: f64 = (p.p50 * eq33_e503_d_b7);
        let eq33_e504_d_b8: f64 = (p.p50 * eq33_e503_d_b8);
        let eq33_e504_d_b9: f64 = (p.p50 * eq33_e503_d_b9);
        let eq33_e504_d_b10: f64 = (p.p50 * eq33_e503_d_b10);
        let eq33_e504_d_b11: f64 = (p.p50 * eq33_e503_d_b11);
        let eq33_e504_d_b12: f64 = (p.p50 * eq33_e503_d_b12);
        let eq33_e504_d_b13: f64 = (p.p50 * eq33_e503_d_b13);
        let eq33_e504_d_b14: f64 = (p.p50 * eq33_e503_d_b14);
        let eq33_e504_d_b15: f64 = (p.p50 * eq33_e503_d_b15);
        (eq33_e504, eq33_e504_d_n0, eq33_e504_d_n1, eq33_e504_d_n2, eq33_e504_d_n3, eq33_e504_d_n4, eq33_e504_d_n5, eq33_e504_d_n6, eq33_e504_d_n7, eq33_e504_d_n8, eq33_e504_d_n9, eq33_e504_d_n10, eq33_e504_d_n11, eq33_e504_d_n12, eq33_e504_d_n13, eq33_e504_d_n14, eq33_e504_d_n15, eq33_e504_d_n16, eq33_e504_d_n17, eq33_e504_d_n18, eq33_e504_d_b0, eq33_e504_d_b1, eq33_e504_d_b2, eq33_e504_d_b3, eq33_e504_d_b4, eq33_e504_d_b5, eq33_e504_d_b6, eq33_e504_d_b7, eq33_e504_d_b8, eq33_e504_d_b9, eq33_e504_d_b10, eq33_e504_d_b11, eq33_e504_d_b12, eq33_e504_d_b13, eq33_e504_d_b14, eq33_e504_d_b15,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq33_value: f64 = eq33_e506;
        let eq33_node_derivatives: [f64; 19] = [eq33_e506_d_n0, eq33_e506_d_n1, eq33_e506_d_n2, eq33_e506_d_n3, eq33_e506_d_n4, eq33_e506_d_n5, eq33_e506_d_n6, eq33_e506_d_n7, eq33_e506_d_n8, eq33_e506_d_n9, eq33_e506_d_n10, eq33_e506_d_n11, eq33_e506_d_n12, eq33_e506_d_n13, eq33_e506_d_n14, eq33_e506_d_n15, eq33_e506_d_n16, eq33_e506_d_n17, eq33_e506_d_n18];
        let eq33_branch_derivatives: [f64; 16] = [eq33_e506_d_b0, eq33_e506_d_b1, eq33_e506_d_b2, eq33_e506_d_b3, eq33_e506_d_b4, eq33_e506_d_b5, eq33_e506_d_b6, eq33_e506_d_b7, eq33_e506_d_b8, eq33_e506_d_b9, eq33_e506_d_b10, eq33_e506_d_b11, eq33_e506_d_b12, eq33_e506_d_b13, eq33_e506_d_b14, eq33_e506_d_b15];
        stamper.stamp_current_dense_local(
            Some(6),
            Some(12),
            multiplicity * (eq33_value),
            &eq33_node_derivatives,
            &eq33_branch_derivatives,
            multiplicity,
        );
        let (eq34_e514, eq34_e514_d_n0, eq34_e514_d_n1, eq34_e514_d_n2, eq34_e514_d_n3, eq34_e514_d_n4, eq34_e514_d_n5, eq34_e514_d_n6, eq34_e514_d_n7, eq34_e514_d_n8, eq34_e514_d_n9, eq34_e514_d_n10, eq34_e514_d_n11, eq34_e514_d_n12, eq34_e514_d_n13, eq34_e514_d_n14, eq34_e514_d_n15, eq34_e514_d_n16, eq34_e514_d_n17, eq34_e514_d_n18, eq34_e514_d_b0, eq34_e514_d_b1, eq34_e514_d_b2, eq34_e514_d_b3, eq34_e514_d_b4, eq34_e514_d_b5, eq34_e514_d_b6, eq34_e514_d_b7, eq34_e514_d_b8, eq34_e514_d_b9, eq34_e514_d_b10, eq34_e514_d_b11, eq34_e514_d_b12, eq34_e514_d_b13, eq34_e514_d_b14, eq34_e514_d_b15,) = {
    if s.b[1851] {
        let eq34_e511: f64 = (s.v[312] + s.v[573]);
        let eq34_e511_d_n0: f64 = (s.dn[312][0] + s.dn[573][0]);
        let eq34_e511_d_n1: f64 = (s.dn[312][1] + s.dn[573][1]);
        let eq34_e511_d_n2: f64 = (s.dn[312][2] + s.dn[573][2]);
        let eq34_e511_d_n3: f64 = (s.dn[312][3] + s.dn[573][3]);
        let eq34_e511_d_n4: f64 = (s.dn[312][4] + s.dn[573][4]);
        let eq34_e511_d_n5: f64 = (s.dn[312][5] + s.dn[573][5]);
        let eq34_e511_d_n6: f64 = (s.dn[312][6] + s.dn[573][6]);
        let eq34_e511_d_n7: f64 = (s.dn[312][7] + s.dn[573][7]);
        let eq34_e511_d_n8: f64 = (s.dn[312][8] + s.dn[573][8]);
        let eq34_e511_d_n9: f64 = (s.dn[312][9] + s.dn[573][9]);
        let eq34_e511_d_n10: f64 = (s.dn[312][10] + s.dn[573][10]);
        let eq34_e511_d_n11: f64 = (s.dn[312][11] + s.dn[573][11]);
        let eq34_e511_d_n12: f64 = (s.dn[312][12] + s.dn[573][12]);
        let eq34_e511_d_n13: f64 = (s.dn[312][13] + s.dn[573][13]);
        let eq34_e511_d_n14: f64 = (s.dn[312][14] + s.dn[573][14]);
        let eq34_e511_d_n15: f64 = (s.dn[312][15] + s.dn[573][15]);
        let eq34_e511_d_n16: f64 = (s.dn[312][16] + s.dn[573][16]);
        let eq34_e511_d_n17: f64 = (s.dn[312][17] + s.dn[573][17]);
        let eq34_e511_d_n18: f64 = (s.dn[312][18] + s.dn[573][18]);
        let eq34_e511_d_b0: f64 = (s.db[312][0] + s.db[573][0]);
        let eq34_e511_d_b1: f64 = (s.db[312][1] + s.db[573][1]);
        let eq34_e511_d_b2: f64 = (s.db[312][2] + s.db[573][2]);
        let eq34_e511_d_b3: f64 = (s.db[312][3] + s.db[573][3]);
        let eq34_e511_d_b4: f64 = (s.db[312][4] + s.db[573][4]);
        let eq34_e511_d_b5: f64 = (s.db[312][5] + s.db[573][5]);
        let eq34_e511_d_b6: f64 = (s.db[312][6] + s.db[573][6]);
        let eq34_e511_d_b7: f64 = (s.db[312][7] + s.db[573][7]);
        let eq34_e511_d_b8: f64 = (s.db[312][8] + s.db[573][8]);
        let eq34_e511_d_b9: f64 = (s.db[312][9] + s.db[573][9]);
        let eq34_e511_d_b10: f64 = (s.db[312][10] + s.db[573][10]);
        let eq34_e511_d_b11: f64 = (s.db[312][11] + s.db[573][11]);
        let eq34_e511_d_b12: f64 = (s.db[312][12] + s.db[573][12]);
        let eq34_e511_d_b13: f64 = (s.db[312][13] + s.db[573][13]);
        let eq34_e511_d_b14: f64 = (s.db[312][14] + s.db[573][14]);
        let eq34_e511_d_b15: f64 = (s.db[312][15] + s.db[573][15]);
        let eq34_e512: f64 = (p.p50 * eq34_e511);
        let eq34_e512_d_n0: f64 = (p.p50 * eq34_e511_d_n0);
        let eq34_e512_d_n1: f64 = (p.p50 * eq34_e511_d_n1);
        let eq34_e512_d_n2: f64 = (p.p50 * eq34_e511_d_n2);
        let eq34_e512_d_n3: f64 = (p.p50 * eq34_e511_d_n3);
        let eq34_e512_d_n4: f64 = (p.p50 * eq34_e511_d_n4);
        let eq34_e512_d_n5: f64 = (p.p50 * eq34_e511_d_n5);
        let eq34_e512_d_n6: f64 = (p.p50 * eq34_e511_d_n6);
        let eq34_e512_d_n7: f64 = (p.p50 * eq34_e511_d_n7);
        let eq34_e512_d_n8: f64 = (p.p50 * eq34_e511_d_n8);
        let eq34_e512_d_n9: f64 = (p.p50 * eq34_e511_d_n9);
        let eq34_e512_d_n10: f64 = (p.p50 * eq34_e511_d_n10);
        let eq34_e512_d_n11: f64 = (p.p50 * eq34_e511_d_n11);
        let eq34_e512_d_n12: f64 = (p.p50 * eq34_e511_d_n12);
        let eq34_e512_d_n13: f64 = (p.p50 * eq34_e511_d_n13);
        let eq34_e512_d_n14: f64 = (p.p50 * eq34_e511_d_n14);
        let eq34_e512_d_n15: f64 = (p.p50 * eq34_e511_d_n15);
        let eq34_e512_d_n16: f64 = (p.p50 * eq34_e511_d_n16);
        let eq34_e512_d_n17: f64 = (p.p50 * eq34_e511_d_n17);
        let eq34_e512_d_n18: f64 = (p.p50 * eq34_e511_d_n18);
        let eq34_e512_d_b0: f64 = (p.p50 * eq34_e511_d_b0);
        let eq34_e512_d_b1: f64 = (p.p50 * eq34_e511_d_b1);
        let eq34_e512_d_b2: f64 = (p.p50 * eq34_e511_d_b2);
        let eq34_e512_d_b3: f64 = (p.p50 * eq34_e511_d_b3);
        let eq34_e512_d_b4: f64 = (p.p50 * eq34_e511_d_b4);
        let eq34_e512_d_b5: f64 = (p.p50 * eq34_e511_d_b5);
        let eq34_e512_d_b6: f64 = (p.p50 * eq34_e511_d_b6);
        let eq34_e512_d_b7: f64 = (p.p50 * eq34_e511_d_b7);
        let eq34_e512_d_b8: f64 = (p.p50 * eq34_e511_d_b8);
        let eq34_e512_d_b9: f64 = (p.p50 * eq34_e511_d_b9);
        let eq34_e512_d_b10: f64 = (p.p50 * eq34_e511_d_b10);
        let eq34_e512_d_b11: f64 = (p.p50 * eq34_e511_d_b11);
        let eq34_e512_d_b12: f64 = (p.p50 * eq34_e511_d_b12);
        let eq34_e512_d_b13: f64 = (p.p50 * eq34_e511_d_b13);
        let eq34_e512_d_b14: f64 = (p.p50 * eq34_e511_d_b14);
        let eq34_e512_d_b15: f64 = (p.p50 * eq34_e511_d_b15);
        (eq34_e512, eq34_e512_d_n0, eq34_e512_d_n1, eq34_e512_d_n2, eq34_e512_d_n3, eq34_e512_d_n4, eq34_e512_d_n5, eq34_e512_d_n6, eq34_e512_d_n7, eq34_e512_d_n8, eq34_e512_d_n9, eq34_e512_d_n10, eq34_e512_d_n11, eq34_e512_d_n12, eq34_e512_d_n13, eq34_e512_d_n14, eq34_e512_d_n15, eq34_e512_d_n16, eq34_e512_d_n17, eq34_e512_d_n18, eq34_e512_d_b0, eq34_e512_d_b1, eq34_e512_d_b2, eq34_e512_d_b3, eq34_e512_d_b4, eq34_e512_d_b5, eq34_e512_d_b6, eq34_e512_d_b7, eq34_e512_d_b8, eq34_e512_d_b9, eq34_e512_d_b10, eq34_e512_d_b11, eq34_e512_d_b12, eq34_e512_d_b13, eq34_e512_d_b14, eq34_e512_d_b15,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq34_value: f64 = eq34_e514;
        let eq34_node_derivatives: [f64; 19] = [eq34_e514_d_n0, eq34_e514_d_n1, eq34_e514_d_n2, eq34_e514_d_n3, eq34_e514_d_n4, eq34_e514_d_n5, eq34_e514_d_n6, eq34_e514_d_n7, eq34_e514_d_n8, eq34_e514_d_n9, eq34_e514_d_n10, eq34_e514_d_n11, eq34_e514_d_n12, eq34_e514_d_n13, eq34_e514_d_n14, eq34_e514_d_n15, eq34_e514_d_n16, eq34_e514_d_n17, eq34_e514_d_n18];
        let eq34_branch_derivatives: [f64; 16] = [eq34_e514_d_b0, eq34_e514_d_b1, eq34_e514_d_b2, eq34_e514_d_b3, eq34_e514_d_b4, eq34_e514_d_b5, eq34_e514_d_b6, eq34_e514_d_b7, eq34_e514_d_b8, eq34_e514_d_b9, eq34_e514_d_b10, eq34_e514_d_b11, eq34_e514_d_b12, eq34_e514_d_b13, eq34_e514_d_b14, eq34_e514_d_b15];
        stamper.stamp_current_dense_local(
            Some(7),
            Some(12),
            multiplicity * (eq34_value),
            &eq34_node_derivatives,
            &eq34_branch_derivatives,
            multiplicity,
        );
        let (eq35_e523, eq35_e523_d_n0, eq35_e523_d_n1, eq35_e523_d_n2, eq35_e523_d_n3, eq35_e523_d_n4, eq35_e523_d_n5, eq35_e523_d_n6, eq35_e523_d_n7, eq35_e523_d_n8, eq35_e523_d_n9, eq35_e523_d_n10, eq35_e523_d_n11, eq35_e523_d_n12, eq35_e523_d_n13, eq35_e523_d_n14, eq35_e523_d_n15, eq35_e523_d_n16, eq35_e523_d_n17, eq35_e523_d_n18, eq35_e523_d_b0, eq35_e523_d_b1, eq35_e523_d_b2, eq35_e523_d_b3, eq35_e523_d_b4, eq35_e523_d_b5, eq35_e523_d_b6, eq35_e523_d_b7, eq35_e523_d_b8, eq35_e523_d_b9, eq35_e523_d_b10, eq35_e523_d_b11, eq35_e523_d_b12, eq35_e523_d_b13, eq35_e523_d_b14, eq35_e523_d_b15,) = {
    if s.b[1851] {
        let eq35_e519: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 6, s.v[283]);
        let eq35_e519_d_n0: f64 = (s.dn[283][0] * ddt_scale);
        let eq35_e519_d_n1: f64 = (s.dn[283][1] * ddt_scale);
        let eq35_e519_d_n2: f64 = (s.dn[283][2] * ddt_scale);
        let eq35_e519_d_n3: f64 = (s.dn[283][3] * ddt_scale);
        let eq35_e519_d_n4: f64 = (s.dn[283][4] * ddt_scale);
        let eq35_e519_d_n5: f64 = (s.dn[283][5] * ddt_scale);
        let eq35_e519_d_n6: f64 = (s.dn[283][6] * ddt_scale);
        let eq35_e519_d_n7: f64 = (s.dn[283][7] * ddt_scale);
        let eq35_e519_d_n8: f64 = (s.dn[283][8] * ddt_scale);
        let eq35_e519_d_n9: f64 = (s.dn[283][9] * ddt_scale);
        let eq35_e519_d_n10: f64 = (s.dn[283][10] * ddt_scale);
        let eq35_e519_d_n11: f64 = (s.dn[283][11] * ddt_scale);
        let eq35_e519_d_n12: f64 = (s.dn[283][12] * ddt_scale);
        let eq35_e519_d_n13: f64 = (s.dn[283][13] * ddt_scale);
        let eq35_e519_d_n14: f64 = (s.dn[283][14] * ddt_scale);
        let eq35_e519_d_n15: f64 = (s.dn[283][15] * ddt_scale);
        let eq35_e519_d_n16: f64 = (s.dn[283][16] * ddt_scale);
        let eq35_e519_d_n17: f64 = (s.dn[283][17] * ddt_scale);
        let eq35_e519_d_n18: f64 = (s.dn[283][18] * ddt_scale);
        let eq35_e519_d_b0: f64 = (s.db[283][0] * ddt_scale);
        let eq35_e519_d_b1: f64 = (s.db[283][1] * ddt_scale);
        let eq35_e519_d_b2: f64 = (s.db[283][2] * ddt_scale);
        let eq35_e519_d_b3: f64 = (s.db[283][3] * ddt_scale);
        let eq35_e519_d_b4: f64 = (s.db[283][4] * ddt_scale);
        let eq35_e519_d_b5: f64 = (s.db[283][5] * ddt_scale);
        let eq35_e519_d_b6: f64 = (s.db[283][6] * ddt_scale);
        let eq35_e519_d_b7: f64 = (s.db[283][7] * ddt_scale);
        let eq35_e519_d_b8: f64 = (s.db[283][8] * ddt_scale);
        let eq35_e519_d_b9: f64 = (s.db[283][9] * ddt_scale);
        let eq35_e519_d_b10: f64 = (s.db[283][10] * ddt_scale);
        let eq35_e519_d_b11: f64 = (s.db[283][11] * ddt_scale);
        let eq35_e519_d_b12: f64 = (s.db[283][12] * ddt_scale);
        let eq35_e519_d_b13: f64 = (s.db[283][13] * ddt_scale);
        let eq35_e519_d_b14: f64 = (s.db[283][14] * ddt_scale);
        let eq35_e519_d_b15: f64 = (s.db[283][15] * ddt_scale);
        let eq35_e520: f64 = (s.v[281] + eq35_e519);
        let eq35_e520_d_n0: f64 = (s.dn[281][0] + eq35_e519_d_n0);
        let eq35_e520_d_n1: f64 = (s.dn[281][1] + eq35_e519_d_n1);
        let eq35_e520_d_n2: f64 = (s.dn[281][2] + eq35_e519_d_n2);
        let eq35_e520_d_n3: f64 = (s.dn[281][3] + eq35_e519_d_n3);
        let eq35_e520_d_n4: f64 = (s.dn[281][4] + eq35_e519_d_n4);
        let eq35_e520_d_n5: f64 = (s.dn[281][5] + eq35_e519_d_n5);
        let eq35_e520_d_n6: f64 = (s.dn[281][6] + eq35_e519_d_n6);
        let eq35_e520_d_n7: f64 = (s.dn[281][7] + eq35_e519_d_n7);
        let eq35_e520_d_n8: f64 = (s.dn[281][8] + eq35_e519_d_n8);
        let eq35_e520_d_n9: f64 = (s.dn[281][9] + eq35_e519_d_n9);
        let eq35_e520_d_n10: f64 = (s.dn[281][10] + eq35_e519_d_n10);
        let eq35_e520_d_n11: f64 = (s.dn[281][11] + eq35_e519_d_n11);
        let eq35_e520_d_n12: f64 = (s.dn[281][12] + eq35_e519_d_n12);
        let eq35_e520_d_n13: f64 = (s.dn[281][13] + eq35_e519_d_n13);
        let eq35_e520_d_n14: f64 = (s.dn[281][14] + eq35_e519_d_n14);
        let eq35_e520_d_n15: f64 = (s.dn[281][15] + eq35_e519_d_n15);
        let eq35_e520_d_n16: f64 = (s.dn[281][16] + eq35_e519_d_n16);
        let eq35_e520_d_n17: f64 = (s.dn[281][17] + eq35_e519_d_n17);
        let eq35_e520_d_n18: f64 = (s.dn[281][18] + eq35_e519_d_n18);
        let eq35_e520_d_b0: f64 = (s.db[281][0] + eq35_e519_d_b0);
        let eq35_e520_d_b1: f64 = (s.db[281][1] + eq35_e519_d_b1);
        let eq35_e520_d_b2: f64 = (s.db[281][2] + eq35_e519_d_b2);
        let eq35_e520_d_b3: f64 = (s.db[281][3] + eq35_e519_d_b3);
        let eq35_e520_d_b4: f64 = (s.db[281][4] + eq35_e519_d_b4);
        let eq35_e520_d_b5: f64 = (s.db[281][5] + eq35_e519_d_b5);
        let eq35_e520_d_b6: f64 = (s.db[281][6] + eq35_e519_d_b6);
        let eq35_e520_d_b7: f64 = (s.db[281][7] + eq35_e519_d_b7);
        let eq35_e520_d_b8: f64 = (s.db[281][8] + eq35_e519_d_b8);
        let eq35_e520_d_b9: f64 = (s.db[281][9] + eq35_e519_d_b9);
        let eq35_e520_d_b10: f64 = (s.db[281][10] + eq35_e519_d_b10);
        let eq35_e520_d_b11: f64 = (s.db[281][11] + eq35_e519_d_b11);
        let eq35_e520_d_b12: f64 = (s.db[281][12] + eq35_e519_d_b12);
        let eq35_e520_d_b13: f64 = (s.db[281][13] + eq35_e519_d_b13);
        let eq35_e520_d_b14: f64 = (s.db[281][14] + eq35_e519_d_b14);
        let eq35_e520_d_b15: f64 = (s.db[281][15] + eq35_e519_d_b15);
        let eq35_e521: f64 = (p.p50 * eq35_e520);
        let eq35_e521_d_n0: f64 = (p.p50 * eq35_e520_d_n0);
        let eq35_e521_d_n1: f64 = (p.p50 * eq35_e520_d_n1);
        let eq35_e521_d_n2: f64 = (p.p50 * eq35_e520_d_n2);
        let eq35_e521_d_n3: f64 = (p.p50 * eq35_e520_d_n3);
        let eq35_e521_d_n4: f64 = (p.p50 * eq35_e520_d_n4);
        let eq35_e521_d_n5: f64 = (p.p50 * eq35_e520_d_n5);
        let eq35_e521_d_n6: f64 = (p.p50 * eq35_e520_d_n6);
        let eq35_e521_d_n7: f64 = (p.p50 * eq35_e520_d_n7);
        let eq35_e521_d_n8: f64 = (p.p50 * eq35_e520_d_n8);
        let eq35_e521_d_n9: f64 = (p.p50 * eq35_e520_d_n9);
        let eq35_e521_d_n10: f64 = (p.p50 * eq35_e520_d_n10);
        let eq35_e521_d_n11: f64 = (p.p50 * eq35_e520_d_n11);
        let eq35_e521_d_n12: f64 = (p.p50 * eq35_e520_d_n12);
        let eq35_e521_d_n13: f64 = (p.p50 * eq35_e520_d_n13);
        let eq35_e521_d_n14: f64 = (p.p50 * eq35_e520_d_n14);
        let eq35_e521_d_n15: f64 = (p.p50 * eq35_e520_d_n15);
        let eq35_e521_d_n16: f64 = (p.p50 * eq35_e520_d_n16);
        let eq35_e521_d_n17: f64 = (p.p50 * eq35_e520_d_n17);
        let eq35_e521_d_n18: f64 = (p.p50 * eq35_e520_d_n18);
        let eq35_e521_d_b0: f64 = (p.p50 * eq35_e520_d_b0);
        let eq35_e521_d_b1: f64 = (p.p50 * eq35_e520_d_b1);
        let eq35_e521_d_b2: f64 = (p.p50 * eq35_e520_d_b2);
        let eq35_e521_d_b3: f64 = (p.p50 * eq35_e520_d_b3);
        let eq35_e521_d_b4: f64 = (p.p50 * eq35_e520_d_b4);
        let eq35_e521_d_b5: f64 = (p.p50 * eq35_e520_d_b5);
        let eq35_e521_d_b6: f64 = (p.p50 * eq35_e520_d_b6);
        let eq35_e521_d_b7: f64 = (p.p50 * eq35_e520_d_b7);
        let eq35_e521_d_b8: f64 = (p.p50 * eq35_e520_d_b8);
        let eq35_e521_d_b9: f64 = (p.p50 * eq35_e520_d_b9);
        let eq35_e521_d_b10: f64 = (p.p50 * eq35_e520_d_b10);
        let eq35_e521_d_b11: f64 = (p.p50 * eq35_e520_d_b11);
        let eq35_e521_d_b12: f64 = (p.p50 * eq35_e520_d_b12);
        let eq35_e521_d_b13: f64 = (p.p50 * eq35_e520_d_b13);
        let eq35_e521_d_b14: f64 = (p.p50 * eq35_e520_d_b14);
        let eq35_e521_d_b15: f64 = (p.p50 * eq35_e520_d_b15);
        (eq35_e521, eq35_e521_d_n0, eq35_e521_d_n1, eq35_e521_d_n2, eq35_e521_d_n3, eq35_e521_d_n4, eq35_e521_d_n5, eq35_e521_d_n6, eq35_e521_d_n7, eq35_e521_d_n8, eq35_e521_d_n9, eq35_e521_d_n10, eq35_e521_d_n11, eq35_e521_d_n12, eq35_e521_d_n13, eq35_e521_d_n14, eq35_e521_d_n15, eq35_e521_d_n16, eq35_e521_d_n17, eq35_e521_d_n18, eq35_e521_d_b0, eq35_e521_d_b1, eq35_e521_d_b2, eq35_e521_d_b3, eq35_e521_d_b4, eq35_e521_d_b5, eq35_e521_d_b6, eq35_e521_d_b7, eq35_e521_d_b8, eq35_e521_d_b9, eq35_e521_d_b10, eq35_e521_d_b11, eq35_e521_d_b12, eq35_e521_d_b13, eq35_e521_d_b14, eq35_e521_d_b15,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq35_value: f64 = eq35_e523;
        let eq35_node_derivatives: [f64; 19] = [eq35_e523_d_n0, eq35_e523_d_n1, eq35_e523_d_n2, eq35_e523_d_n3, eq35_e523_d_n4, eq35_e523_d_n5, eq35_e523_d_n6, eq35_e523_d_n7, eq35_e523_d_n8, eq35_e523_d_n9, eq35_e523_d_n10, eq35_e523_d_n11, eq35_e523_d_n12, eq35_e523_d_n13, eq35_e523_d_n14, eq35_e523_d_n15, eq35_e523_d_n16, eq35_e523_d_n17, eq35_e523_d_n18];
        let eq35_branch_derivatives: [f64; 16] = [eq35_e523_d_b0, eq35_e523_d_b1, eq35_e523_d_b2, eq35_e523_d_b3, eq35_e523_d_b4, eq35_e523_d_b5, eq35_e523_d_b6, eq35_e523_d_b7, eq35_e523_d_b8, eq35_e523_d_b9, eq35_e523_d_b10, eq35_e523_d_b11, eq35_e523_d_b12, eq35_e523_d_b13, eq35_e523_d_b14, eq35_e523_d_b15];
        stamper.stamp_current_dense_local(
            Some(9),
            Some(7),
            multiplicity * (eq35_value),
            &eq35_node_derivatives,
            &eq35_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_transient_equations_block_3(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        multiplicity: f64,
        ddt_active: bool,
        ddt_scale: f64,
        ddt_state_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_initialized: &mut [bool; Instance::DDT_STATE_COUNT],
    ) {
        let nv4 = ctx.node_voltage(nodes[4]);
        let nv12 = ctx.node_voltage(nodes[12]);
        let nv13 = ctx.node_voltage(nodes[13]);
        let nv17 = ctx.node_voltage(nodes[17]);
        let nv18 = ctx.node_voltage(nodes[18]);
        let (eq36_e532, eq36_e532_d_n0, eq36_e532_d_n1, eq36_e532_d_n2, eq36_e532_d_n3, eq36_e532_d_n4, eq36_e532_d_n5, eq36_e532_d_n6, eq36_e532_d_n7, eq36_e532_d_n8, eq36_e532_d_n9, eq36_e532_d_n10, eq36_e532_d_n11, eq36_e532_d_n12, eq36_e532_d_n13, eq36_e532_d_n14, eq36_e532_d_n15, eq36_e532_d_n16, eq36_e532_d_n17, eq36_e532_d_n18, eq36_e532_d_b0, eq36_e532_d_b1, eq36_e532_d_b2, eq36_e532_d_b3, eq36_e532_d_b4, eq36_e532_d_b5, eq36_e532_d_b6, eq36_e532_d_b7, eq36_e532_d_b8, eq36_e532_d_b9, eq36_e532_d_b10, eq36_e532_d_b11, eq36_e532_d_b12, eq36_e532_d_b13, eq36_e532_d_b14, eq36_e532_d_b15,) = {
    if s.b[1851] {
        let eq36_e528: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 7, s.v[284]);
        let eq36_e528_d_n0: f64 = (s.dn[284][0] * ddt_scale);
        let eq36_e528_d_n1: f64 = (s.dn[284][1] * ddt_scale);
        let eq36_e528_d_n2: f64 = (s.dn[284][2] * ddt_scale);
        let eq36_e528_d_n3: f64 = (s.dn[284][3] * ddt_scale);
        let eq36_e528_d_n4: f64 = (s.dn[284][4] * ddt_scale);
        let eq36_e528_d_n5: f64 = (s.dn[284][5] * ddt_scale);
        let eq36_e528_d_n6: f64 = (s.dn[284][6] * ddt_scale);
        let eq36_e528_d_n7: f64 = (s.dn[284][7] * ddt_scale);
        let eq36_e528_d_n8: f64 = (s.dn[284][8] * ddt_scale);
        let eq36_e528_d_n9: f64 = (s.dn[284][9] * ddt_scale);
        let eq36_e528_d_n10: f64 = (s.dn[284][10] * ddt_scale);
        let eq36_e528_d_n11: f64 = (s.dn[284][11] * ddt_scale);
        let eq36_e528_d_n12: f64 = (s.dn[284][12] * ddt_scale);
        let eq36_e528_d_n13: f64 = (s.dn[284][13] * ddt_scale);
        let eq36_e528_d_n14: f64 = (s.dn[284][14] * ddt_scale);
        let eq36_e528_d_n15: f64 = (s.dn[284][15] * ddt_scale);
        let eq36_e528_d_n16: f64 = (s.dn[284][16] * ddt_scale);
        let eq36_e528_d_n17: f64 = (s.dn[284][17] * ddt_scale);
        let eq36_e528_d_n18: f64 = (s.dn[284][18] * ddt_scale);
        let eq36_e528_d_b0: f64 = (s.db[284][0] * ddt_scale);
        let eq36_e528_d_b1: f64 = (s.db[284][1] * ddt_scale);
        let eq36_e528_d_b2: f64 = (s.db[284][2] * ddt_scale);
        let eq36_e528_d_b3: f64 = (s.db[284][3] * ddt_scale);
        let eq36_e528_d_b4: f64 = (s.db[284][4] * ddt_scale);
        let eq36_e528_d_b5: f64 = (s.db[284][5] * ddt_scale);
        let eq36_e528_d_b6: f64 = (s.db[284][6] * ddt_scale);
        let eq36_e528_d_b7: f64 = (s.db[284][7] * ddt_scale);
        let eq36_e528_d_b8: f64 = (s.db[284][8] * ddt_scale);
        let eq36_e528_d_b9: f64 = (s.db[284][9] * ddt_scale);
        let eq36_e528_d_b10: f64 = (s.db[284][10] * ddt_scale);
        let eq36_e528_d_b11: f64 = (s.db[284][11] * ddt_scale);
        let eq36_e528_d_b12: f64 = (s.db[284][12] * ddt_scale);
        let eq36_e528_d_b13: f64 = (s.db[284][13] * ddt_scale);
        let eq36_e528_d_b14: f64 = (s.db[284][14] * ddt_scale);
        let eq36_e528_d_b15: f64 = (s.db[284][15] * ddt_scale);
        let eq36_e529: f64 = (s.v[282] + eq36_e528);
        let eq36_e529_d_n0: f64 = (s.dn[282][0] + eq36_e528_d_n0);
        let eq36_e529_d_n1: f64 = (s.dn[282][1] + eq36_e528_d_n1);
        let eq36_e529_d_n2: f64 = (s.dn[282][2] + eq36_e528_d_n2);
        let eq36_e529_d_n3: f64 = (s.dn[282][3] + eq36_e528_d_n3);
        let eq36_e529_d_n4: f64 = (s.dn[282][4] + eq36_e528_d_n4);
        let eq36_e529_d_n5: f64 = (s.dn[282][5] + eq36_e528_d_n5);
        let eq36_e529_d_n6: f64 = (s.dn[282][6] + eq36_e528_d_n6);
        let eq36_e529_d_n7: f64 = (s.dn[282][7] + eq36_e528_d_n7);
        let eq36_e529_d_n8: f64 = (s.dn[282][8] + eq36_e528_d_n8);
        let eq36_e529_d_n9: f64 = (s.dn[282][9] + eq36_e528_d_n9);
        let eq36_e529_d_n10: f64 = (s.dn[282][10] + eq36_e528_d_n10);
        let eq36_e529_d_n11: f64 = (s.dn[282][11] + eq36_e528_d_n11);
        let eq36_e529_d_n12: f64 = (s.dn[282][12] + eq36_e528_d_n12);
        let eq36_e529_d_n13: f64 = (s.dn[282][13] + eq36_e528_d_n13);
        let eq36_e529_d_n14: f64 = (s.dn[282][14] + eq36_e528_d_n14);
        let eq36_e529_d_n15: f64 = (s.dn[282][15] + eq36_e528_d_n15);
        let eq36_e529_d_n16: f64 = (s.dn[282][16] + eq36_e528_d_n16);
        let eq36_e529_d_n17: f64 = (s.dn[282][17] + eq36_e528_d_n17);
        let eq36_e529_d_n18: f64 = (s.dn[282][18] + eq36_e528_d_n18);
        let eq36_e529_d_b0: f64 = (s.db[282][0] + eq36_e528_d_b0);
        let eq36_e529_d_b1: f64 = (s.db[282][1] + eq36_e528_d_b1);
        let eq36_e529_d_b2: f64 = (s.db[282][2] + eq36_e528_d_b2);
        let eq36_e529_d_b3: f64 = (s.db[282][3] + eq36_e528_d_b3);
        let eq36_e529_d_b4: f64 = (s.db[282][4] + eq36_e528_d_b4);
        let eq36_e529_d_b5: f64 = (s.db[282][5] + eq36_e528_d_b5);
        let eq36_e529_d_b6: f64 = (s.db[282][6] + eq36_e528_d_b6);
        let eq36_e529_d_b7: f64 = (s.db[282][7] + eq36_e528_d_b7);
        let eq36_e529_d_b8: f64 = (s.db[282][8] + eq36_e528_d_b8);
        let eq36_e529_d_b9: f64 = (s.db[282][9] + eq36_e528_d_b9);
        let eq36_e529_d_b10: f64 = (s.db[282][10] + eq36_e528_d_b10);
        let eq36_e529_d_b11: f64 = (s.db[282][11] + eq36_e528_d_b11);
        let eq36_e529_d_b12: f64 = (s.db[282][12] + eq36_e528_d_b12);
        let eq36_e529_d_b13: f64 = (s.db[282][13] + eq36_e528_d_b13);
        let eq36_e529_d_b14: f64 = (s.db[282][14] + eq36_e528_d_b14);
        let eq36_e529_d_b15: f64 = (s.db[282][15] + eq36_e528_d_b15);
        let eq36_e530: f64 = (p.p50 * eq36_e529);
        let eq36_e530_d_n0: f64 = (p.p50 * eq36_e529_d_n0);
        let eq36_e530_d_n1: f64 = (p.p50 * eq36_e529_d_n1);
        let eq36_e530_d_n2: f64 = (p.p50 * eq36_e529_d_n2);
        let eq36_e530_d_n3: f64 = (p.p50 * eq36_e529_d_n3);
        let eq36_e530_d_n4: f64 = (p.p50 * eq36_e529_d_n4);
        let eq36_e530_d_n5: f64 = (p.p50 * eq36_e529_d_n5);
        let eq36_e530_d_n6: f64 = (p.p50 * eq36_e529_d_n6);
        let eq36_e530_d_n7: f64 = (p.p50 * eq36_e529_d_n7);
        let eq36_e530_d_n8: f64 = (p.p50 * eq36_e529_d_n8);
        let eq36_e530_d_n9: f64 = (p.p50 * eq36_e529_d_n9);
        let eq36_e530_d_n10: f64 = (p.p50 * eq36_e529_d_n10);
        let eq36_e530_d_n11: f64 = (p.p50 * eq36_e529_d_n11);
        let eq36_e530_d_n12: f64 = (p.p50 * eq36_e529_d_n12);
        let eq36_e530_d_n13: f64 = (p.p50 * eq36_e529_d_n13);
        let eq36_e530_d_n14: f64 = (p.p50 * eq36_e529_d_n14);
        let eq36_e530_d_n15: f64 = (p.p50 * eq36_e529_d_n15);
        let eq36_e530_d_n16: f64 = (p.p50 * eq36_e529_d_n16);
        let eq36_e530_d_n17: f64 = (p.p50 * eq36_e529_d_n17);
        let eq36_e530_d_n18: f64 = (p.p50 * eq36_e529_d_n18);
        let eq36_e530_d_b0: f64 = (p.p50 * eq36_e529_d_b0);
        let eq36_e530_d_b1: f64 = (p.p50 * eq36_e529_d_b1);
        let eq36_e530_d_b2: f64 = (p.p50 * eq36_e529_d_b2);
        let eq36_e530_d_b3: f64 = (p.p50 * eq36_e529_d_b3);
        let eq36_e530_d_b4: f64 = (p.p50 * eq36_e529_d_b4);
        let eq36_e530_d_b5: f64 = (p.p50 * eq36_e529_d_b5);
        let eq36_e530_d_b6: f64 = (p.p50 * eq36_e529_d_b6);
        let eq36_e530_d_b7: f64 = (p.p50 * eq36_e529_d_b7);
        let eq36_e530_d_b8: f64 = (p.p50 * eq36_e529_d_b8);
        let eq36_e530_d_b9: f64 = (p.p50 * eq36_e529_d_b9);
        let eq36_e530_d_b10: f64 = (p.p50 * eq36_e529_d_b10);
        let eq36_e530_d_b11: f64 = (p.p50 * eq36_e529_d_b11);
        let eq36_e530_d_b12: f64 = (p.p50 * eq36_e529_d_b12);
        let eq36_e530_d_b13: f64 = (p.p50 * eq36_e529_d_b13);
        let eq36_e530_d_b14: f64 = (p.p50 * eq36_e529_d_b14);
        let eq36_e530_d_b15: f64 = (p.p50 * eq36_e529_d_b15);
        (eq36_e530, eq36_e530_d_n0, eq36_e530_d_n1, eq36_e530_d_n2, eq36_e530_d_n3, eq36_e530_d_n4, eq36_e530_d_n5, eq36_e530_d_n6, eq36_e530_d_n7, eq36_e530_d_n8, eq36_e530_d_n9, eq36_e530_d_n10, eq36_e530_d_n11, eq36_e530_d_n12, eq36_e530_d_n13, eq36_e530_d_n14, eq36_e530_d_n15, eq36_e530_d_n16, eq36_e530_d_n17, eq36_e530_d_n18, eq36_e530_d_b0, eq36_e530_d_b1, eq36_e530_d_b2, eq36_e530_d_b3, eq36_e530_d_b4, eq36_e530_d_b5, eq36_e530_d_b6, eq36_e530_d_b7, eq36_e530_d_b8, eq36_e530_d_b9, eq36_e530_d_b10, eq36_e530_d_b11, eq36_e530_d_b12, eq36_e530_d_b13, eq36_e530_d_b14, eq36_e530_d_b15,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq36_value: f64 = eq36_e532;
        let eq36_node_derivatives: [f64; 19] = [eq36_e532_d_n0, eq36_e532_d_n1, eq36_e532_d_n2, eq36_e532_d_n3, eq36_e532_d_n4, eq36_e532_d_n5, eq36_e532_d_n6, eq36_e532_d_n7, eq36_e532_d_n8, eq36_e532_d_n9, eq36_e532_d_n10, eq36_e532_d_n11, eq36_e532_d_n12, eq36_e532_d_n13, eq36_e532_d_n14, eq36_e532_d_n15, eq36_e532_d_n16, eq36_e532_d_n17, eq36_e532_d_n18];
        let eq36_branch_derivatives: [f64; 16] = [eq36_e532_d_b0, eq36_e532_d_b1, eq36_e532_d_b2, eq36_e532_d_b3, eq36_e532_d_b4, eq36_e532_d_b5, eq36_e532_d_b6, eq36_e532_d_b7, eq36_e532_d_b8, eq36_e532_d_b9, eq36_e532_d_b10, eq36_e532_d_b11, eq36_e532_d_b12, eq36_e532_d_b13, eq36_e532_d_b14, eq36_e532_d_b15];
        stamper.stamp_current_dense_local(
            Some(8),
            Some(6),
            multiplicity * (eq36_value),
            &eq36_node_derivatives,
            &eq36_branch_derivatives,
            multiplicity,
        );
        let (eq37_e540, eq37_e540_d_n0, eq37_e540_d_n1, eq37_e540_d_n2, eq37_e540_d_n3, eq37_e540_d_n4, eq37_e540_d_n5, eq37_e540_d_n6, eq37_e540_d_n7, eq37_e540_d_n8, eq37_e540_d_n9, eq37_e540_d_n10, eq37_e540_d_n11, eq37_e540_d_n12, eq37_e540_d_n13, eq37_e540_d_n14, eq37_e540_d_n15, eq37_e540_d_n16, eq37_e540_d_n17, eq37_e540_d_n18, eq37_e540_d_b0, eq37_e540_d_b1, eq37_e540_d_b2, eq37_e540_d_b3, eq37_e540_d_b4, eq37_e540_d_b5, eq37_e540_d_b6, eq37_e540_d_b7, eq37_e540_d_b8, eq37_e540_d_b9, eq37_e540_d_b10, eq37_e540_d_b11, eq37_e540_d_b12, eq37_e540_d_b13, eq37_e540_d_b14, eq37_e540_d_b15,) = {
    if (s.b[1851] && (p.p261 != 0.0)) {
        let eq37_e538: f64 = ((nv4 - nv12) / s.v[2]);
        let eq37_e538_d_n0: f64 = (-(((nv4 - nv12) * s.dn[2][0]) / (s.v[2] * s.v[2])));
        let eq37_e538_d_n1: f64 = (-(((nv4 - nv12) * s.dn[2][1]) / (s.v[2] * s.v[2])));
        let eq37_e538_d_n2: f64 = (-(((nv4 - nv12) * s.dn[2][2]) / (s.v[2] * s.v[2])));
        let eq37_e538_d_n3: f64 = (-(((nv4 - nv12) * s.dn[2][3]) / (s.v[2] * s.v[2])));
        let eq37_e538_d_n4: f64 = ((s.v[2] - ((nv4 - nv12) * s.dn[2][4])) / (s.v[2] * s.v[2]));
        let eq37_e538_d_n5: f64 = (-(((nv4 - nv12) * s.dn[2][5]) / (s.v[2] * s.v[2])));
        let eq37_e538_d_n6: f64 = (-(((nv4 - nv12) * s.dn[2][6]) / (s.v[2] * s.v[2])));
        let eq37_e538_d_n7: f64 = (-(((nv4 - nv12) * s.dn[2][7]) / (s.v[2] * s.v[2])));
        let eq37_e538_d_n8: f64 = (-(((nv4 - nv12) * s.dn[2][8]) / (s.v[2] * s.v[2])));
        let eq37_e538_d_n9: f64 = (-(((nv4 - nv12) * s.dn[2][9]) / (s.v[2] * s.v[2])));
        let eq37_e538_d_n10: f64 = (-(((nv4 - nv12) * s.dn[2][10]) / (s.v[2] * s.v[2])));
        let eq37_e538_d_n11: f64 = (-(((nv4 - nv12) * s.dn[2][11]) / (s.v[2] * s.v[2])));
        let eq37_e538_d_n12: f64 = (((-s.v[2]) - ((nv4 - nv12) * s.dn[2][12])) / (s.v[2] * s.v[2]));
        let eq37_e538_d_n13: f64 = (-(((nv4 - nv12) * s.dn[2][13]) / (s.v[2] * s.v[2])));
        let eq37_e538_d_n14: f64 = (-(((nv4 - nv12) * s.dn[2][14]) / (s.v[2] * s.v[2])));
        let eq37_e538_d_n15: f64 = (-(((nv4 - nv12) * s.dn[2][15]) / (s.v[2] * s.v[2])));
        let eq37_e538_d_n16: f64 = (-(((nv4 - nv12) * s.dn[2][16]) / (s.v[2] * s.v[2])));
        let eq37_e538_d_n17: f64 = (-(((nv4 - nv12) * s.dn[2][17]) / (s.v[2] * s.v[2])));
        let eq37_e538_d_n18: f64 = (-(((nv4 - nv12) * s.dn[2][18]) / (s.v[2] * s.v[2])));
        let eq37_e538_d_b0: f64 = (-(((nv4 - nv12) * s.db[2][0]) / (s.v[2] * s.v[2])));
        let eq37_e538_d_b1: f64 = (-(((nv4 - nv12) * s.db[2][1]) / (s.v[2] * s.v[2])));
        let eq37_e538_d_b2: f64 = (-(((nv4 - nv12) * s.db[2][2]) / (s.v[2] * s.v[2])));
        let eq37_e538_d_b3: f64 = (-(((nv4 - nv12) * s.db[2][3]) / (s.v[2] * s.v[2])));
        let eq37_e538_d_b4: f64 = (-(((nv4 - nv12) * s.db[2][4]) / (s.v[2] * s.v[2])));
        let eq37_e538_d_b5: f64 = (-(((nv4 - nv12) * s.db[2][5]) / (s.v[2] * s.v[2])));
        let eq37_e538_d_b6: f64 = (-(((nv4 - nv12) * s.db[2][6]) / (s.v[2] * s.v[2])));
        let eq37_e538_d_b7: f64 = (-(((nv4 - nv12) * s.db[2][7]) / (s.v[2] * s.v[2])));
        let eq37_e538_d_b8: f64 = (-(((nv4 - nv12) * s.db[2][8]) / (s.v[2] * s.v[2])));
        let eq37_e538_d_b9: f64 = (-(((nv4 - nv12) * s.db[2][9]) / (s.v[2] * s.v[2])));
        let eq37_e538_d_b10: f64 = (-(((nv4 - nv12) * s.db[2][10]) / (s.v[2] * s.v[2])));
        let eq37_e538_d_b11: f64 = (-(((nv4 - nv12) * s.db[2][11]) / (s.v[2] * s.v[2])));
        let eq37_e538_d_b12: f64 = (-(((nv4 - nv12) * s.db[2][12]) / (s.v[2] * s.v[2])));
        let eq37_e538_d_b13: f64 = (-(((nv4 - nv12) * s.db[2][13]) / (s.v[2] * s.v[2])));
        let eq37_e538_d_b14: f64 = (-(((nv4 - nv12) * s.db[2][14]) / (s.v[2] * s.v[2])));
        let eq37_e538_d_b15: f64 = (-(((nv4 - nv12) * s.db[2][15]) / (s.v[2] * s.v[2])));
        (eq37_e538, eq37_e538_d_n0, eq37_e538_d_n1, eq37_e538_d_n2, eq37_e538_d_n3, eq37_e538_d_n4, eq37_e538_d_n5, eq37_e538_d_n6, eq37_e538_d_n7, eq37_e538_d_n8, eq37_e538_d_n9, eq37_e538_d_n10, eq37_e538_d_n11, eq37_e538_d_n12, eq37_e538_d_n13, eq37_e538_d_n14, eq37_e538_d_n15, eq37_e538_d_n16, eq37_e538_d_n17, eq37_e538_d_n18, eq37_e538_d_b0, eq37_e538_d_b1, eq37_e538_d_b2, eq37_e538_d_b3, eq37_e538_d_b4, eq37_e538_d_b5, eq37_e538_d_b6, eq37_e538_d_b7, eq37_e538_d_b8, eq37_e538_d_b9, eq37_e538_d_b10, eq37_e538_d_b11, eq37_e538_d_b12, eq37_e538_d_b13, eq37_e538_d_b14, eq37_e538_d_b15,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq37_value: f64 = eq37_e540;
        let eq37_node_derivatives: [f64; 19] = [eq37_e540_d_n0, eq37_e540_d_n1, eq37_e540_d_n2, eq37_e540_d_n3, eq37_e540_d_n4, eq37_e540_d_n5, eq37_e540_d_n6, eq37_e540_d_n7, eq37_e540_d_n8, eq37_e540_d_n9, eq37_e540_d_n10, eq37_e540_d_n11, eq37_e540_d_n12, eq37_e540_d_n13, eq37_e540_d_n14, eq37_e540_d_n15, eq37_e540_d_n16, eq37_e540_d_n17, eq37_e540_d_n18];
        let eq37_branch_derivatives: [f64; 16] = [eq37_e540_d_b0, eq37_e540_d_b1, eq37_e540_d_b2, eq37_e540_d_b3, eq37_e540_d_b4, eq37_e540_d_b5, eq37_e540_d_b6, eq37_e540_d_b7, eq37_e540_d_b8, eq37_e540_d_b9, eq37_e540_d_b10, eq37_e540_d_b11, eq37_e540_d_b12, eq37_e540_d_b13, eq37_e540_d_b14, eq37_e540_d_b15];
        stamper.stamp_current_dense_local(
            Some(4),
            Some(12),
            multiplicity * (eq37_value),
            &eq37_node_derivatives,
            &eq37_branch_derivatives,
            multiplicity,
        );
        let (eq43_e583, eq43_e583_d_n0, eq43_e583_d_n1, eq43_e583_d_n2, eq43_e583_d_n3, eq43_e583_d_n4, eq43_e583_d_n5, eq43_e583_d_n6, eq43_e583_d_n7, eq43_e583_d_n8, eq43_e583_d_n9, eq43_e583_d_n10, eq43_e583_d_n11, eq43_e583_d_n12, eq43_e583_d_n13, eq43_e583_d_n14, eq43_e583_d_n15, eq43_e583_d_n16, eq43_e583_d_n17, eq43_e583_d_n18, eq43_e583_d_b0, eq43_e583_d_b1, eq43_e583_d_b2, eq43_e583_d_b3, eq43_e583_d_b4, eq43_e583_d_b5, eq43_e583_d_b6, eq43_e583_d_b7, eq43_e583_d_b8, eq43_e583_d_b9, eq43_e583_d_b10, eq43_e583_d_b11, eq43_e583_d_b12, eq43_e583_d_b13, eq43_e583_d_b14, eq43_e583_d_b15,) = {
    if (s.b[1851] && (p.p34 != 0.0)) {
        (s.v[582], s.dn[582][0], s.dn[582][1], s.dn[582][2], s.dn[582][3], s.dn[582][4], s.dn[582][5], s.dn[582][6], s.dn[582][7], s.dn[582][8], s.dn[582][9], s.dn[582][10], s.dn[582][11], s.dn[582][12], s.dn[582][13], s.dn[582][14], s.dn[582][15], s.dn[582][16], s.dn[582][17], s.dn[582][18], s.db[582][0], s.db[582][1], s.db[582][2], s.db[582][3], s.db[582][4], s.db[582][5], s.db[582][6], s.db[582][7], s.db[582][8], s.db[582][9], s.db[582][10], s.db[582][11], s.db[582][12], s.db[582][13], s.db[582][14], s.db[582][15],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq43_value: f64 = eq43_e583;
        let eq43_node_derivatives: [f64; 19] = [eq43_e583_d_n0, eq43_e583_d_n1, eq43_e583_d_n2, eq43_e583_d_n3, eq43_e583_d_n4, eq43_e583_d_n5, eq43_e583_d_n6, eq43_e583_d_n7, eq43_e583_d_n8, eq43_e583_d_n9, eq43_e583_d_n10, eq43_e583_d_n11, eq43_e583_d_n12, eq43_e583_d_n13, eq43_e583_d_n14, eq43_e583_d_n15, eq43_e583_d_n16, eq43_e583_d_n17, eq43_e583_d_n18];
        let eq43_branch_derivatives: [f64; 16] = [eq43_e583_d_b0, eq43_e583_d_b1, eq43_e583_d_b2, eq43_e583_d_b3, eq43_e583_d_b4, eq43_e583_d_b5, eq43_e583_d_b6, eq43_e583_d_b7, eq43_e583_d_b8, eq43_e583_d_b9, eq43_e583_d_b10, eq43_e583_d_b11, eq43_e583_d_b12, eq43_e583_d_b13, eq43_e583_d_b14, eq43_e583_d_b15];
        stamper.stamp_current_dense_local(
            Some(18),
            None,
            multiplicity * (eq43_value),
            &eq43_node_derivatives,
            &eq43_branch_derivatives,
            multiplicity,
        );
        let (eq44_e589, eq44_e589_d_n0, eq44_e589_d_n1, eq44_e589_d_n2, eq44_e589_d_n3, eq44_e589_d_n4, eq44_e589_d_n5, eq44_e589_d_n6, eq44_e589_d_n7, eq44_e589_d_n8, eq44_e589_d_n9, eq44_e589_d_n10, eq44_e589_d_n11, eq44_e589_d_n12, eq44_e589_d_n13, eq44_e589_d_n14, eq44_e589_d_n15, eq44_e589_d_n16, eq44_e589_d_n17, eq44_e589_d_n18, eq44_e589_d_b0, eq44_e589_d_b1, eq44_e589_d_b2, eq44_e589_d_b3, eq44_e589_d_b4, eq44_e589_d_b5, eq44_e589_d_b6, eq44_e589_d_b7, eq44_e589_d_b8, eq44_e589_d_b9, eq44_e589_d_b10, eq44_e589_d_b11, eq44_e589_d_b12, eq44_e589_d_b13, eq44_e589_d_b14, eq44_e589_d_b15,) = {
    if (s.b[1851] && (p.p34 != 0.0)) {
        (s.v[583], s.dn[583][0], s.dn[583][1], s.dn[583][2], s.dn[583][3], s.dn[583][4], s.dn[583][5], s.dn[583][6], s.dn[583][7], s.dn[583][8], s.dn[583][9], s.dn[583][10], s.dn[583][11], s.dn[583][12], s.dn[583][13], s.dn[583][14], s.dn[583][15], s.dn[583][16], s.dn[583][17], s.dn[583][18], s.db[583][0], s.db[583][1], s.db[583][2], s.db[583][3], s.db[583][4], s.db[583][5], s.db[583][6], s.db[583][7], s.db[583][8], s.db[583][9], s.db[583][10], s.db[583][11], s.db[583][12], s.db[583][13], s.db[583][14], s.db[583][15],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq44_value: f64 = eq44_e589;
        let eq44_node_derivatives: [f64; 19] = [eq44_e589_d_n0, eq44_e589_d_n1, eq44_e589_d_n2, eq44_e589_d_n3, eq44_e589_d_n4, eq44_e589_d_n5, eq44_e589_d_n6, eq44_e589_d_n7, eq44_e589_d_n8, eq44_e589_d_n9, eq44_e589_d_n10, eq44_e589_d_n11, eq44_e589_d_n12, eq44_e589_d_n13, eq44_e589_d_n14, eq44_e589_d_n15, eq44_e589_d_n16, eq44_e589_d_n17, eq44_e589_d_n18];
        let eq44_branch_derivatives: [f64; 16] = [eq44_e589_d_b0, eq44_e589_d_b1, eq44_e589_d_b2, eq44_e589_d_b3, eq44_e589_d_b4, eq44_e589_d_b5, eq44_e589_d_b6, eq44_e589_d_b7, eq44_e589_d_b8, eq44_e589_d_b9, eq44_e589_d_b10, eq44_e589_d_b11, eq44_e589_d_b12, eq44_e589_d_b13, eq44_e589_d_b14, eq44_e589_d_b15];
        stamper.stamp_current_dense_local(
            Some(13),
            None,
            multiplicity * (eq44_value),
            &eq44_node_derivatives,
            &eq44_branch_derivatives,
            multiplicity,
        );
        let (eq47_e616, eq47_e616_d_n18,) = {
    if (s.b[1851] && (p.p34 != 0.0)) {
        let eq47_e611: f64 = (1e-9 / 0.0001);
        let eq47_e613: f64 = (eq47_e611 * (nv18 - 0.0));
        let eq47_e614: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 8, eq47_e613);
        let eq47_e614_d_n18: f64 = (eq47_e611 * ddt_scale);
        (eq47_e614, eq47_e614_d_n18,)
    } else {
        (0.0, 0.0,)
    }
};
        let eq47_value: f64 = eq47_e616;
        stamper.stamp_current_node1_local(
            Some(18),
            None,
            multiplicity * (eq47_value),
            18,
            multiplicity * (eq47_e616_d_n18),
        );
        let (eq48_e627, eq48_e627_d_n13,) = {
    if (s.b[1851] && (p.p34 != 0.0)) {
        let eq48_e622: f64 = (1e-9 / 0.0001);
        let eq48_e624: f64 = (eq48_e622 * (nv13 - 0.0));
        let eq48_e625: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 9, eq48_e624);
        let eq48_e625_d_n13: f64 = (eq48_e622 * ddt_scale);
        (eq48_e625, eq48_e625_d_n13,)
    } else {
        (0.0, 0.0,)
    }
};
        let eq48_value: f64 = eq48_e627;
        stamper.stamp_current_node1_local(
            Some(13),
            None,
            multiplicity * (eq48_value),
            13,
            multiplicity * (eq48_e627_d_n13),
        );
        let (eq51_e647, eq51_e647_d_n0, eq51_e647_d_n1, eq51_e647_d_n2, eq51_e647_d_n3, eq51_e647_d_n4, eq51_e647_d_n5, eq51_e647_d_n6, eq51_e647_d_n7, eq51_e647_d_n8, eq51_e647_d_n9, eq51_e647_d_n10, eq51_e647_d_n11, eq51_e647_d_n12, eq51_e647_d_n13, eq51_e647_d_n14, eq51_e647_d_n15, eq51_e647_d_n16, eq51_e647_d_n17, eq51_e647_d_n18, eq51_e647_d_b0, eq51_e647_d_b1, eq51_e647_d_b2, eq51_e647_d_b3, eq51_e647_d_b4, eq51_e647_d_b5, eq51_e647_d_b6, eq51_e647_d_b7, eq51_e647_d_b8, eq51_e647_d_b9, eq51_e647_d_b10, eq51_e647_d_b11, eq51_e647_d_b12, eq51_e647_d_b13, eq51_e647_d_b14, eq51_e647_d_b15,) = {
    if (s.b[1851] && s.b[1852]) {
        (s.v[592], s.dn[592][0], s.dn[592][1], s.dn[592][2], s.dn[592][3], s.dn[592][4], s.dn[592][5], s.dn[592][6], s.dn[592][7], s.dn[592][8], s.dn[592][9], s.dn[592][10], s.dn[592][11], s.dn[592][12], s.dn[592][13], s.dn[592][14], s.dn[592][15], s.dn[592][16], s.dn[592][17], s.dn[592][18], s.db[592][0], s.db[592][1], s.db[592][2], s.db[592][3], s.db[592][4], s.db[592][5], s.db[592][6], s.db[592][7], s.db[592][8], s.db[592][9], s.db[592][10], s.db[592][11], s.db[592][12], s.db[592][13], s.db[592][14], s.db[592][15],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq51_value: f64 = eq51_e647;
        let eq51_node_derivatives: [f64; 19] = [eq51_e647_d_n0, eq51_e647_d_n1, eq51_e647_d_n2, eq51_e647_d_n3, eq51_e647_d_n4, eq51_e647_d_n5, eq51_e647_d_n6, eq51_e647_d_n7, eq51_e647_d_n8, eq51_e647_d_n9, eq51_e647_d_n10, eq51_e647_d_n11, eq51_e647_d_n12, eq51_e647_d_n13, eq51_e647_d_n14, eq51_e647_d_n15, eq51_e647_d_n16, eq51_e647_d_n17, eq51_e647_d_n18];
        let eq51_branch_derivatives: [f64; 16] = [eq51_e647_d_b0, eq51_e647_d_b1, eq51_e647_d_b2, eq51_e647_d_b3, eq51_e647_d_b4, eq51_e647_d_b5, eq51_e647_d_b6, eq51_e647_d_b7, eq51_e647_d_b8, eq51_e647_d_b9, eq51_e647_d_b10, eq51_e647_d_b11, eq51_e647_d_b12, eq51_e647_d_b13, eq51_e647_d_b14, eq51_e647_d_b15];
        stamper.stamp_current_dense_local(
            Some(17),
            None,
            multiplicity * (eq51_value),
            &eq51_node_derivatives,
            &eq51_branch_derivatives,
            multiplicity,
        );
        let (eq53_e666, eq53_e666_d_n17,) = {
    if (s.b[1851] && s.b[1852]) {
        let eq53_e661: f64 = (1e-9 / 0.0001);
        let eq53_e663: f64 = (eq53_e661 * (nv17 - 0.0));
        let eq53_e664: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 10, eq53_e663);
        let eq53_e664_d_n17: f64 = (eq53_e661 * ddt_scale);
        (eq53_e664, eq53_e664_d_n17,)
    } else {
        (0.0, 0.0,)
    }
};
        let eq53_value: f64 = eq53_e666;
        stamper.stamp_current_node1_local(
            Some(17),
            None,
            multiplicity * (eq53_value),
            17,
            multiplicity * (eq53_e666_d_n17),
        );
        let (eq55_e682, eq55_e682_d_n0, eq55_e682_d_n1, eq55_e682_d_n2, eq55_e682_d_n3, eq55_e682_d_n4, eq55_e682_d_n5, eq55_e682_d_n6, eq55_e682_d_n7, eq55_e682_d_n8, eq55_e682_d_n9, eq55_e682_d_n10, eq55_e682_d_n11, eq55_e682_d_n12, eq55_e682_d_n13, eq55_e682_d_n14, eq55_e682_d_n15, eq55_e682_d_n16, eq55_e682_d_n17, eq55_e682_d_n18, eq55_e682_d_b0, eq55_e682_d_b1, eq55_e682_d_b2, eq55_e682_d_b3, eq55_e682_d_b4, eq55_e682_d_b5, eq55_e682_d_b6, eq55_e682_d_b7, eq55_e682_d_b8, eq55_e682_d_b9, eq55_e682_d_b10, eq55_e682_d_b11, eq55_e682_d_b12, eq55_e682_d_b13, eq55_e682_d_b14, eq55_e682_d_b15,) = {
    if (!s.b[1851]) {
        let eq55_e679: f64 = (s.v[311] + s.v[263]);
        let eq55_e679_d_n0: f64 = (s.dn[311][0] + s.dn[263][0]);
        let eq55_e679_d_n1: f64 = (s.dn[311][1] + s.dn[263][1]);
        let eq55_e679_d_n2: f64 = (s.dn[311][2] + s.dn[263][2]);
        let eq55_e679_d_n3: f64 = (s.dn[311][3] + s.dn[263][3]);
        let eq55_e679_d_n4: f64 = (s.dn[311][4] + s.dn[263][4]);
        let eq55_e679_d_n5: f64 = (s.dn[311][5] + s.dn[263][5]);
        let eq55_e679_d_n6: f64 = (s.dn[311][6] + s.dn[263][6]);
        let eq55_e679_d_n7: f64 = (s.dn[311][7] + s.dn[263][7]);
        let eq55_e679_d_n8: f64 = (s.dn[311][8] + s.dn[263][8]);
        let eq55_e679_d_n9: f64 = (s.dn[311][9] + s.dn[263][9]);
        let eq55_e679_d_n10: f64 = (s.dn[311][10] + s.dn[263][10]);
        let eq55_e679_d_n11: f64 = (s.dn[311][11] + s.dn[263][11]);
        let eq55_e679_d_n12: f64 = (s.dn[311][12] + s.dn[263][12]);
        let eq55_e679_d_n13: f64 = (s.dn[311][13] + s.dn[263][13]);
        let eq55_e679_d_n14: f64 = (s.dn[311][14] + s.dn[263][14]);
        let eq55_e679_d_n15: f64 = (s.dn[311][15] + s.dn[263][15]);
        let eq55_e679_d_n16: f64 = (s.dn[311][16] + s.dn[263][16]);
        let eq55_e679_d_n17: f64 = (s.dn[311][17] + s.dn[263][17]);
        let eq55_e679_d_n18: f64 = (s.dn[311][18] + s.dn[263][18]);
        let eq55_e679_d_b0: f64 = (s.db[311][0] + s.db[263][0]);
        let eq55_e679_d_b1: f64 = (s.db[311][1] + s.db[263][1]);
        let eq55_e679_d_b2: f64 = (s.db[311][2] + s.db[263][2]);
        let eq55_e679_d_b3: f64 = (s.db[311][3] + s.db[263][3]);
        let eq55_e679_d_b4: f64 = (s.db[311][4] + s.db[263][4]);
        let eq55_e679_d_b5: f64 = (s.db[311][5] + s.db[263][5]);
        let eq55_e679_d_b6: f64 = (s.db[311][6] + s.db[263][6]);
        let eq55_e679_d_b7: f64 = (s.db[311][7] + s.db[263][7]);
        let eq55_e679_d_b8: f64 = (s.db[311][8] + s.db[263][8]);
        let eq55_e679_d_b9: f64 = (s.db[311][9] + s.db[263][9]);
        let eq55_e679_d_b10: f64 = (s.db[311][10] + s.db[263][10]);
        let eq55_e679_d_b11: f64 = (s.db[311][11] + s.db[263][11]);
        let eq55_e679_d_b12: f64 = (s.db[311][12] + s.db[263][12]);
        let eq55_e679_d_b13: f64 = (s.db[311][13] + s.db[263][13]);
        let eq55_e679_d_b14: f64 = (s.db[311][14] + s.db[263][14]);
        let eq55_e679_d_b15: f64 = (s.db[311][15] + s.db[263][15]);
        let eq55_e680: f64 = (p.p50 * eq55_e679);
        let eq55_e680_d_n0: f64 = (p.p50 * eq55_e679_d_n0);
        let eq55_e680_d_n1: f64 = (p.p50 * eq55_e679_d_n1);
        let eq55_e680_d_n2: f64 = (p.p50 * eq55_e679_d_n2);
        let eq55_e680_d_n3: f64 = (p.p50 * eq55_e679_d_n3);
        let eq55_e680_d_n4: f64 = (p.p50 * eq55_e679_d_n4);
        let eq55_e680_d_n5: f64 = (p.p50 * eq55_e679_d_n5);
        let eq55_e680_d_n6: f64 = (p.p50 * eq55_e679_d_n6);
        let eq55_e680_d_n7: f64 = (p.p50 * eq55_e679_d_n7);
        let eq55_e680_d_n8: f64 = (p.p50 * eq55_e679_d_n8);
        let eq55_e680_d_n9: f64 = (p.p50 * eq55_e679_d_n9);
        let eq55_e680_d_n10: f64 = (p.p50 * eq55_e679_d_n10);
        let eq55_e680_d_n11: f64 = (p.p50 * eq55_e679_d_n11);
        let eq55_e680_d_n12: f64 = (p.p50 * eq55_e679_d_n12);
        let eq55_e680_d_n13: f64 = (p.p50 * eq55_e679_d_n13);
        let eq55_e680_d_n14: f64 = (p.p50 * eq55_e679_d_n14);
        let eq55_e680_d_n15: f64 = (p.p50 * eq55_e679_d_n15);
        let eq55_e680_d_n16: f64 = (p.p50 * eq55_e679_d_n16);
        let eq55_e680_d_n17: f64 = (p.p50 * eq55_e679_d_n17);
        let eq55_e680_d_n18: f64 = (p.p50 * eq55_e679_d_n18);
        let eq55_e680_d_b0: f64 = (p.p50 * eq55_e679_d_b0);
        let eq55_e680_d_b1: f64 = (p.p50 * eq55_e679_d_b1);
        let eq55_e680_d_b2: f64 = (p.p50 * eq55_e679_d_b2);
        let eq55_e680_d_b3: f64 = (p.p50 * eq55_e679_d_b3);
        let eq55_e680_d_b4: f64 = (p.p50 * eq55_e679_d_b4);
        let eq55_e680_d_b5: f64 = (p.p50 * eq55_e679_d_b5);
        let eq55_e680_d_b6: f64 = (p.p50 * eq55_e679_d_b6);
        let eq55_e680_d_b7: f64 = (p.p50 * eq55_e679_d_b7);
        let eq55_e680_d_b8: f64 = (p.p50 * eq55_e679_d_b8);
        let eq55_e680_d_b9: f64 = (p.p50 * eq55_e679_d_b9);
        let eq55_e680_d_b10: f64 = (p.p50 * eq55_e679_d_b10);
        let eq55_e680_d_b11: f64 = (p.p50 * eq55_e679_d_b11);
        let eq55_e680_d_b12: f64 = (p.p50 * eq55_e679_d_b12);
        let eq55_e680_d_b13: f64 = (p.p50 * eq55_e679_d_b13);
        let eq55_e680_d_b14: f64 = (p.p50 * eq55_e679_d_b14);
        let eq55_e680_d_b15: f64 = (p.p50 * eq55_e679_d_b15);
        (eq55_e680, eq55_e680_d_n0, eq55_e680_d_n1, eq55_e680_d_n2, eq55_e680_d_n3, eq55_e680_d_n4, eq55_e680_d_n5, eq55_e680_d_n6, eq55_e680_d_n7, eq55_e680_d_n8, eq55_e680_d_n9, eq55_e680_d_n10, eq55_e680_d_n11, eq55_e680_d_n12, eq55_e680_d_n13, eq55_e680_d_n14, eq55_e680_d_n15, eq55_e680_d_n16, eq55_e680_d_n17, eq55_e680_d_n18, eq55_e680_d_b0, eq55_e680_d_b1, eq55_e680_d_b2, eq55_e680_d_b3, eq55_e680_d_b4, eq55_e680_d_b5, eq55_e680_d_b6, eq55_e680_d_b7, eq55_e680_d_b8, eq55_e680_d_b9, eq55_e680_d_b10, eq55_e680_d_b11, eq55_e680_d_b12, eq55_e680_d_b13, eq55_e680_d_b14, eq55_e680_d_b15,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq55_value: f64 = eq55_e682;
        let eq55_node_derivatives: [f64; 19] = [eq55_e682_d_n0, eq55_e682_d_n1, eq55_e682_d_n2, eq55_e682_d_n3, eq55_e682_d_n4, eq55_e682_d_n5, eq55_e682_d_n6, eq55_e682_d_n7, eq55_e682_d_n8, eq55_e682_d_n9, eq55_e682_d_n10, eq55_e682_d_n11, eq55_e682_d_n12, eq55_e682_d_n13, eq55_e682_d_n14, eq55_e682_d_n15, eq55_e682_d_n16, eq55_e682_d_n17, eq55_e682_d_n18];
        let eq55_branch_derivatives: [f64; 16] = [eq55_e682_d_b0, eq55_e682_d_b1, eq55_e682_d_b2, eq55_e682_d_b3, eq55_e682_d_b4, eq55_e682_d_b5, eq55_e682_d_b6, eq55_e682_d_b7, eq55_e682_d_b8, eq55_e682_d_b9, eq55_e682_d_b10, eq55_e682_d_b11, eq55_e682_d_b12, eq55_e682_d_b13, eq55_e682_d_b14, eq55_e682_d_b15];
        stamper.stamp_current_dense_local(
            Some(6),
            Some(7),
            multiplicity * (eq55_value),
            &eq55_node_derivatives,
            &eq55_branch_derivatives,
            multiplicity,
        );
        let (eq56_e691, eq56_e691_d_n0, eq56_e691_d_n1, eq56_e691_d_n2, eq56_e691_d_n3, eq56_e691_d_n4, eq56_e691_d_n5, eq56_e691_d_n6, eq56_e691_d_n7, eq56_e691_d_n8, eq56_e691_d_n9, eq56_e691_d_n10, eq56_e691_d_n11, eq56_e691_d_n12, eq56_e691_d_n13, eq56_e691_d_n14, eq56_e691_d_n15, eq56_e691_d_n16, eq56_e691_d_n17, eq56_e691_d_n18, eq56_e691_d_b0, eq56_e691_d_b1, eq56_e691_d_b2, eq56_e691_d_b3, eq56_e691_d_b4, eq56_e691_d_b5, eq56_e691_d_b6, eq56_e691_d_b7, eq56_e691_d_b8, eq56_e691_d_b9, eq56_e691_d_b10, eq56_e691_d_b11, eq56_e691_d_b12, eq56_e691_d_b13, eq56_e691_d_b14, eq56_e691_d_b15,) = {
    if (!s.b[1851]) {
        let eq56_e688: f64 = (s.v[312] + s.v[573]);
        let eq56_e688_d_n0: f64 = (s.dn[312][0] + s.dn[573][0]);
        let eq56_e688_d_n1: f64 = (s.dn[312][1] + s.dn[573][1]);
        let eq56_e688_d_n2: f64 = (s.dn[312][2] + s.dn[573][2]);
        let eq56_e688_d_n3: f64 = (s.dn[312][3] + s.dn[573][3]);
        let eq56_e688_d_n4: f64 = (s.dn[312][4] + s.dn[573][4]);
        let eq56_e688_d_n5: f64 = (s.dn[312][5] + s.dn[573][5]);
        let eq56_e688_d_n6: f64 = (s.dn[312][6] + s.dn[573][6]);
        let eq56_e688_d_n7: f64 = (s.dn[312][7] + s.dn[573][7]);
        let eq56_e688_d_n8: f64 = (s.dn[312][8] + s.dn[573][8]);
        let eq56_e688_d_n9: f64 = (s.dn[312][9] + s.dn[573][9]);
        let eq56_e688_d_n10: f64 = (s.dn[312][10] + s.dn[573][10]);
        let eq56_e688_d_n11: f64 = (s.dn[312][11] + s.dn[573][11]);
        let eq56_e688_d_n12: f64 = (s.dn[312][12] + s.dn[573][12]);
        let eq56_e688_d_n13: f64 = (s.dn[312][13] + s.dn[573][13]);
        let eq56_e688_d_n14: f64 = (s.dn[312][14] + s.dn[573][14]);
        let eq56_e688_d_n15: f64 = (s.dn[312][15] + s.dn[573][15]);
        let eq56_e688_d_n16: f64 = (s.dn[312][16] + s.dn[573][16]);
        let eq56_e688_d_n17: f64 = (s.dn[312][17] + s.dn[573][17]);
        let eq56_e688_d_n18: f64 = (s.dn[312][18] + s.dn[573][18]);
        let eq56_e688_d_b0: f64 = (s.db[312][0] + s.db[573][0]);
        let eq56_e688_d_b1: f64 = (s.db[312][1] + s.db[573][1]);
        let eq56_e688_d_b2: f64 = (s.db[312][2] + s.db[573][2]);
        let eq56_e688_d_b3: f64 = (s.db[312][3] + s.db[573][3]);
        let eq56_e688_d_b4: f64 = (s.db[312][4] + s.db[573][4]);
        let eq56_e688_d_b5: f64 = (s.db[312][5] + s.db[573][5]);
        let eq56_e688_d_b6: f64 = (s.db[312][6] + s.db[573][6]);
        let eq56_e688_d_b7: f64 = (s.db[312][7] + s.db[573][7]);
        let eq56_e688_d_b8: f64 = (s.db[312][8] + s.db[573][8]);
        let eq56_e688_d_b9: f64 = (s.db[312][9] + s.db[573][9]);
        let eq56_e688_d_b10: f64 = (s.db[312][10] + s.db[573][10]);
        let eq56_e688_d_b11: f64 = (s.db[312][11] + s.db[573][11]);
        let eq56_e688_d_b12: f64 = (s.db[312][12] + s.db[573][12]);
        let eq56_e688_d_b13: f64 = (s.db[312][13] + s.db[573][13]);
        let eq56_e688_d_b14: f64 = (s.db[312][14] + s.db[573][14]);
        let eq56_e688_d_b15: f64 = (s.db[312][15] + s.db[573][15]);
        let eq56_e689: f64 = (p.p50 * eq56_e688);
        let eq56_e689_d_n0: f64 = (p.p50 * eq56_e688_d_n0);
        let eq56_e689_d_n1: f64 = (p.p50 * eq56_e688_d_n1);
        let eq56_e689_d_n2: f64 = (p.p50 * eq56_e688_d_n2);
        let eq56_e689_d_n3: f64 = (p.p50 * eq56_e688_d_n3);
        let eq56_e689_d_n4: f64 = (p.p50 * eq56_e688_d_n4);
        let eq56_e689_d_n5: f64 = (p.p50 * eq56_e688_d_n5);
        let eq56_e689_d_n6: f64 = (p.p50 * eq56_e688_d_n6);
        let eq56_e689_d_n7: f64 = (p.p50 * eq56_e688_d_n7);
        let eq56_e689_d_n8: f64 = (p.p50 * eq56_e688_d_n8);
        let eq56_e689_d_n9: f64 = (p.p50 * eq56_e688_d_n9);
        let eq56_e689_d_n10: f64 = (p.p50 * eq56_e688_d_n10);
        let eq56_e689_d_n11: f64 = (p.p50 * eq56_e688_d_n11);
        let eq56_e689_d_n12: f64 = (p.p50 * eq56_e688_d_n12);
        let eq56_e689_d_n13: f64 = (p.p50 * eq56_e688_d_n13);
        let eq56_e689_d_n14: f64 = (p.p50 * eq56_e688_d_n14);
        let eq56_e689_d_n15: f64 = (p.p50 * eq56_e688_d_n15);
        let eq56_e689_d_n16: f64 = (p.p50 * eq56_e688_d_n16);
        let eq56_e689_d_n17: f64 = (p.p50 * eq56_e688_d_n17);
        let eq56_e689_d_n18: f64 = (p.p50 * eq56_e688_d_n18);
        let eq56_e689_d_b0: f64 = (p.p50 * eq56_e688_d_b0);
        let eq56_e689_d_b1: f64 = (p.p50 * eq56_e688_d_b1);
        let eq56_e689_d_b2: f64 = (p.p50 * eq56_e688_d_b2);
        let eq56_e689_d_b3: f64 = (p.p50 * eq56_e688_d_b3);
        let eq56_e689_d_b4: f64 = (p.p50 * eq56_e688_d_b4);
        let eq56_e689_d_b5: f64 = (p.p50 * eq56_e688_d_b5);
        let eq56_e689_d_b6: f64 = (p.p50 * eq56_e688_d_b6);
        let eq56_e689_d_b7: f64 = (p.p50 * eq56_e688_d_b7);
        let eq56_e689_d_b8: f64 = (p.p50 * eq56_e688_d_b8);
        let eq56_e689_d_b9: f64 = (p.p50 * eq56_e688_d_b9);
        let eq56_e689_d_b10: f64 = (p.p50 * eq56_e688_d_b10);
        let eq56_e689_d_b11: f64 = (p.p50 * eq56_e688_d_b11);
        let eq56_e689_d_b12: f64 = (p.p50 * eq56_e688_d_b12);
        let eq56_e689_d_b13: f64 = (p.p50 * eq56_e688_d_b13);
        let eq56_e689_d_b14: f64 = (p.p50 * eq56_e688_d_b14);
        let eq56_e689_d_b15: f64 = (p.p50 * eq56_e688_d_b15);
        (eq56_e689, eq56_e689_d_n0, eq56_e689_d_n1, eq56_e689_d_n2, eq56_e689_d_n3, eq56_e689_d_n4, eq56_e689_d_n5, eq56_e689_d_n6, eq56_e689_d_n7, eq56_e689_d_n8, eq56_e689_d_n9, eq56_e689_d_n10, eq56_e689_d_n11, eq56_e689_d_n12, eq56_e689_d_n13, eq56_e689_d_n14, eq56_e689_d_n15, eq56_e689_d_n16, eq56_e689_d_n17, eq56_e689_d_n18, eq56_e689_d_b0, eq56_e689_d_b1, eq56_e689_d_b2, eq56_e689_d_b3, eq56_e689_d_b4, eq56_e689_d_b5, eq56_e689_d_b6, eq56_e689_d_b7, eq56_e689_d_b8, eq56_e689_d_b9, eq56_e689_d_b10, eq56_e689_d_b11, eq56_e689_d_b12, eq56_e689_d_b13, eq56_e689_d_b14, eq56_e689_d_b15,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq56_value: f64 = eq56_e691;
        let eq56_node_derivatives: [f64; 19] = [eq56_e691_d_n0, eq56_e691_d_n1, eq56_e691_d_n2, eq56_e691_d_n3, eq56_e691_d_n4, eq56_e691_d_n5, eq56_e691_d_n6, eq56_e691_d_n7, eq56_e691_d_n8, eq56_e691_d_n9, eq56_e691_d_n10, eq56_e691_d_n11, eq56_e691_d_n12, eq56_e691_d_n13, eq56_e691_d_n14, eq56_e691_d_n15, eq56_e691_d_n16, eq56_e691_d_n17, eq56_e691_d_n18];
        let eq56_branch_derivatives: [f64; 16] = [eq56_e691_d_b0, eq56_e691_d_b1, eq56_e691_d_b2, eq56_e691_d_b3, eq56_e691_d_b4, eq56_e691_d_b5, eq56_e691_d_b6, eq56_e691_d_b7, eq56_e691_d_b8, eq56_e691_d_b9, eq56_e691_d_b10, eq56_e691_d_b11, eq56_e691_d_b12, eq56_e691_d_b13, eq56_e691_d_b14, eq56_e691_d_b15];
        stamper.stamp_current_dense_local(
            Some(7),
            Some(6),
            multiplicity * (eq56_value),
            &eq56_node_derivatives,
            &eq56_branch_derivatives,
            multiplicity,
        );
        let (eq58_e703, eq58_e703_d_n0, eq58_e703_d_n1, eq58_e703_d_n2, eq58_e703_d_n3, eq58_e703_d_n4, eq58_e703_d_n5, eq58_e703_d_n6, eq58_e703_d_n7, eq58_e703_d_n8, eq58_e703_d_n9, eq58_e703_d_n10, eq58_e703_d_n11, eq58_e703_d_n12, eq58_e703_d_n13, eq58_e703_d_n14, eq58_e703_d_n15, eq58_e703_d_n16, eq58_e703_d_n17, eq58_e703_d_n18, eq58_e703_d_b0, eq58_e703_d_b1, eq58_e703_d_b2, eq58_e703_d_b3, eq58_e703_d_b4, eq58_e703_d_b5, eq58_e703_d_b6, eq58_e703_d_b7, eq58_e703_d_b8, eq58_e703_d_b9, eq58_e703_d_b10, eq58_e703_d_b11, eq58_e703_d_b12, eq58_e703_d_b13, eq58_e703_d_b14, eq58_e703_d_b15,) = {
    if ((!s.b[1851]) && (p.p37 != 0.0)) {
        (s.v[592], s.dn[592][0], s.dn[592][1], s.dn[592][2], s.dn[592][3], s.dn[592][4], s.dn[592][5], s.dn[592][6], s.dn[592][7], s.dn[592][8], s.dn[592][9], s.dn[592][10], s.dn[592][11], s.dn[592][12], s.dn[592][13], s.dn[592][14], s.dn[592][15], s.dn[592][16], s.dn[592][17], s.dn[592][18], s.db[592][0], s.db[592][1], s.db[592][2], s.db[592][3], s.db[592][4], s.db[592][5], s.db[592][6], s.db[592][7], s.db[592][8], s.db[592][9], s.db[592][10], s.db[592][11], s.db[592][12], s.db[592][13], s.db[592][14], s.db[592][15],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq58_value: f64 = eq58_e703;
        let eq58_node_derivatives: [f64; 19] = [eq58_e703_d_n0, eq58_e703_d_n1, eq58_e703_d_n2, eq58_e703_d_n3, eq58_e703_d_n4, eq58_e703_d_n5, eq58_e703_d_n6, eq58_e703_d_n7, eq58_e703_d_n8, eq58_e703_d_n9, eq58_e703_d_n10, eq58_e703_d_n11, eq58_e703_d_n12, eq58_e703_d_n13, eq58_e703_d_n14, eq58_e703_d_n15, eq58_e703_d_n16, eq58_e703_d_n17, eq58_e703_d_n18];
        let eq58_branch_derivatives: [f64; 16] = [eq58_e703_d_b0, eq58_e703_d_b1, eq58_e703_d_b2, eq58_e703_d_b3, eq58_e703_d_b4, eq58_e703_d_b5, eq58_e703_d_b6, eq58_e703_d_b7, eq58_e703_d_b8, eq58_e703_d_b9, eq58_e703_d_b10, eq58_e703_d_b11, eq58_e703_d_b12, eq58_e703_d_b13, eq58_e703_d_b14, eq58_e703_d_b15];
        stamper.stamp_current_dense_local(
            Some(17),
            None,
            multiplicity * (eq58_value),
            &eq58_node_derivatives,
            &eq58_branch_derivatives,
            multiplicity,
        );
        let (eq60_e724, eq60_e724_d_n17,) = {
    if ((!s.b[1851]) && (p.p37 != 0.0)) {
        let eq60_e719: f64 = (1e-9 / 0.0001);
        let eq60_e721: f64 = (eq60_e719 * (nv17 - 0.0));
        let eq60_e722: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 11, eq60_e721);
        let eq60_e722_d_n17: f64 = (eq60_e719 * ddt_scale);
        (eq60_e722, eq60_e722_d_n17,)
    } else {
        (0.0, 0.0,)
    }
};
        let eq60_value: f64 = eq60_e724;
        stamper.stamp_current_node1_local(
            Some(17),
            None,
            multiplicity * (eq60_value),
            17,
            multiplicity * (eq60_e724_d_n17),
        );
    }

    pub(super) fn stamp_transient_equations_block_4(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        multiplicity: f64,
        ddt_active: bool,
        ddt_scale: f64,
        ddt_state_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_initialized: &mut [bool; Instance::DDT_STATE_COUNT],
    ) {
        let nv13 = ctx.node_voltage(nodes[13]);
        let nv15 = ctx.node_voltage(nodes[15]);
        let nv16 = ctx.node_voltage(nodes[16]);
        let (eq62_e739, eq62_e739_d_n0, eq62_e739_d_n1, eq62_e739_d_n2, eq62_e739_d_n3, eq62_e739_d_n4, eq62_e739_d_n5, eq62_e739_d_n6, eq62_e739_d_n7, eq62_e739_d_n8, eq62_e739_d_n9, eq62_e739_d_n10, eq62_e739_d_n11, eq62_e739_d_n12, eq62_e739_d_n13, eq62_e739_d_n14, eq62_e739_d_n15, eq62_e739_d_n16, eq62_e739_d_n17, eq62_e739_d_n18, eq62_e739_d_b0, eq62_e739_d_b1, eq62_e739_d_b2, eq62_e739_d_b3, eq62_e739_d_b4, eq62_e739_d_b5, eq62_e739_d_b6, eq62_e739_d_b7, eq62_e739_d_b8, eq62_e739_d_b9, eq62_e739_d_b10, eq62_e739_d_b11, eq62_e739_d_b12, eq62_e739_d_b13, eq62_e739_d_b14, eq62_e739_d_b15,) = {
    if ((!s.b[1851]) && (p.p34 != 0.0)) {
        (s.v[574], s.dn[574][0], s.dn[574][1], s.dn[574][2], s.dn[574][3], s.dn[574][4], s.dn[574][5], s.dn[574][6], s.dn[574][7], s.dn[574][8], s.dn[574][9], s.dn[574][10], s.dn[574][11], s.dn[574][12], s.dn[574][13], s.dn[574][14], s.dn[574][15], s.dn[574][16], s.dn[574][17], s.dn[574][18], s.db[574][0], s.db[574][1], s.db[574][2], s.db[574][3], s.db[574][4], s.db[574][5], s.db[574][6], s.db[574][7], s.db[574][8], s.db[574][9], s.db[574][10], s.db[574][11], s.db[574][12], s.db[574][13], s.db[574][14], s.db[574][15],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq62_value: f64 = eq62_e739;
        let eq62_node_derivatives: [f64; 19] = [eq62_e739_d_n0, eq62_e739_d_n1, eq62_e739_d_n2, eq62_e739_d_n3, eq62_e739_d_n4, eq62_e739_d_n5, eq62_e739_d_n6, eq62_e739_d_n7, eq62_e739_d_n8, eq62_e739_d_n9, eq62_e739_d_n10, eq62_e739_d_n11, eq62_e739_d_n12, eq62_e739_d_n13, eq62_e739_d_n14, eq62_e739_d_n15, eq62_e739_d_n16, eq62_e739_d_n17, eq62_e739_d_n18];
        let eq62_branch_derivatives: [f64; 16] = [eq62_e739_d_b0, eq62_e739_d_b1, eq62_e739_d_b2, eq62_e739_d_b3, eq62_e739_d_b4, eq62_e739_d_b5, eq62_e739_d_b6, eq62_e739_d_b7, eq62_e739_d_b8, eq62_e739_d_b9, eq62_e739_d_b10, eq62_e739_d_b11, eq62_e739_d_b12, eq62_e739_d_b13, eq62_e739_d_b14, eq62_e739_d_b15];
        stamper.stamp_current_dense_local(
            Some(15),
            None,
            multiplicity * (eq62_value),
            &eq62_node_derivatives,
            &eq62_branch_derivatives,
            multiplicity,
        );
        let (eq63_e746, eq63_e746_d_n0, eq63_e746_d_n1, eq63_e746_d_n2, eq63_e746_d_n3, eq63_e746_d_n4, eq63_e746_d_n5, eq63_e746_d_n6, eq63_e746_d_n7, eq63_e746_d_n8, eq63_e746_d_n9, eq63_e746_d_n10, eq63_e746_d_n11, eq63_e746_d_n12, eq63_e746_d_n13, eq63_e746_d_n14, eq63_e746_d_n15, eq63_e746_d_n16, eq63_e746_d_n17, eq63_e746_d_n18, eq63_e746_d_b0, eq63_e746_d_b1, eq63_e746_d_b2, eq63_e746_d_b3, eq63_e746_d_b4, eq63_e746_d_b5, eq63_e746_d_b6, eq63_e746_d_b7, eq63_e746_d_b8, eq63_e746_d_b9, eq63_e746_d_b10, eq63_e746_d_b11, eq63_e746_d_b12, eq63_e746_d_b13, eq63_e746_d_b14, eq63_e746_d_b15,) = {
    if ((!s.b[1851]) && (p.p34 != 0.0)) {
        (s.v[575], s.dn[575][0], s.dn[575][1], s.dn[575][2], s.dn[575][3], s.dn[575][4], s.dn[575][5], s.dn[575][6], s.dn[575][7], s.dn[575][8], s.dn[575][9], s.dn[575][10], s.dn[575][11], s.dn[575][12], s.dn[575][13], s.dn[575][14], s.dn[575][15], s.dn[575][16], s.dn[575][17], s.dn[575][18], s.db[575][0], s.db[575][1], s.db[575][2], s.db[575][3], s.db[575][4], s.db[575][5], s.db[575][6], s.db[575][7], s.db[575][8], s.db[575][9], s.db[575][10], s.db[575][11], s.db[575][12], s.db[575][13], s.db[575][14], s.db[575][15],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq63_value: f64 = eq63_e746;
        let eq63_node_derivatives: [f64; 19] = [eq63_e746_d_n0, eq63_e746_d_n1, eq63_e746_d_n2, eq63_e746_d_n3, eq63_e746_d_n4, eq63_e746_d_n5, eq63_e746_d_n6, eq63_e746_d_n7, eq63_e746_d_n8, eq63_e746_d_n9, eq63_e746_d_n10, eq63_e746_d_n11, eq63_e746_d_n12, eq63_e746_d_n13, eq63_e746_d_n14, eq63_e746_d_n15, eq63_e746_d_n16, eq63_e746_d_n17, eq63_e746_d_n18];
        let eq63_branch_derivatives: [f64; 16] = [eq63_e746_d_b0, eq63_e746_d_b1, eq63_e746_d_b2, eq63_e746_d_b3, eq63_e746_d_b4, eq63_e746_d_b5, eq63_e746_d_b6, eq63_e746_d_b7, eq63_e746_d_b8, eq63_e746_d_b9, eq63_e746_d_b10, eq63_e746_d_b11, eq63_e746_d_b12, eq63_e746_d_b13, eq63_e746_d_b14, eq63_e746_d_b15];
        stamper.stamp_current_dense_local(
            Some(16),
            None,
            multiplicity * (eq63_value),
            &eq63_node_derivatives,
            &eq63_branch_derivatives,
            multiplicity,
        );
        let (eq64_e753, eq64_e753_d_n0, eq64_e753_d_n1, eq64_e753_d_n2, eq64_e753_d_n3, eq64_e753_d_n4, eq64_e753_d_n5, eq64_e753_d_n6, eq64_e753_d_n7, eq64_e753_d_n8, eq64_e753_d_n9, eq64_e753_d_n10, eq64_e753_d_n11, eq64_e753_d_n12, eq64_e753_d_n13, eq64_e753_d_n14, eq64_e753_d_n15, eq64_e753_d_n16, eq64_e753_d_n17, eq64_e753_d_n18, eq64_e753_d_b0, eq64_e753_d_b1, eq64_e753_d_b2, eq64_e753_d_b3, eq64_e753_d_b4, eq64_e753_d_b5, eq64_e753_d_b6, eq64_e753_d_b7, eq64_e753_d_b8, eq64_e753_d_b9, eq64_e753_d_b10, eq64_e753_d_b11, eq64_e753_d_b12, eq64_e753_d_b13, eq64_e753_d_b14, eq64_e753_d_b15,) = {
    if ((!s.b[1851]) && (p.p34 != 0.0)) {
        (s.v[583], s.dn[583][0], s.dn[583][1], s.dn[583][2], s.dn[583][3], s.dn[583][4], s.dn[583][5], s.dn[583][6], s.dn[583][7], s.dn[583][8], s.dn[583][9], s.dn[583][10], s.dn[583][11], s.dn[583][12], s.dn[583][13], s.dn[583][14], s.dn[583][15], s.dn[583][16], s.dn[583][17], s.dn[583][18], s.db[583][0], s.db[583][1], s.db[583][2], s.db[583][3], s.db[583][4], s.db[583][5], s.db[583][6], s.db[583][7], s.db[583][8], s.db[583][9], s.db[583][10], s.db[583][11], s.db[583][12], s.db[583][13], s.db[583][14], s.db[583][15],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq64_value: f64 = eq64_e753;
        let eq64_node_derivatives: [f64; 19] = [eq64_e753_d_n0, eq64_e753_d_n1, eq64_e753_d_n2, eq64_e753_d_n3, eq64_e753_d_n4, eq64_e753_d_n5, eq64_e753_d_n6, eq64_e753_d_n7, eq64_e753_d_n8, eq64_e753_d_n9, eq64_e753_d_n10, eq64_e753_d_n11, eq64_e753_d_n12, eq64_e753_d_n13, eq64_e753_d_n14, eq64_e753_d_n15, eq64_e753_d_n16, eq64_e753_d_n17, eq64_e753_d_n18];
        let eq64_branch_derivatives: [f64; 16] = [eq64_e753_d_b0, eq64_e753_d_b1, eq64_e753_d_b2, eq64_e753_d_b3, eq64_e753_d_b4, eq64_e753_d_b5, eq64_e753_d_b6, eq64_e753_d_b7, eq64_e753_d_b8, eq64_e753_d_b9, eq64_e753_d_b10, eq64_e753_d_b11, eq64_e753_d_b12, eq64_e753_d_b13, eq64_e753_d_b14, eq64_e753_d_b15];
        stamper.stamp_current_dense_local(
            Some(13),
            None,
            multiplicity * (eq64_value),
            &eq64_node_derivatives,
            &eq64_branch_derivatives,
            multiplicity,
        );
        let (eq68_e792, eq68_e792_d_n15,) = {
    if ((!s.b[1851]) && (p.p34 != 0.0)) {
        let eq68_e787: f64 = (1e-9 / 0.0001);
        let eq68_e789: f64 = (eq68_e787 * (nv15 - 0.0));
        let eq68_e790: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 12, eq68_e789);
        let eq68_e790_d_n15: f64 = (eq68_e787 * ddt_scale);
        (eq68_e790, eq68_e790_d_n15,)
    } else {
        (0.0, 0.0,)
    }
};
        let eq68_value: f64 = eq68_e792;
        stamper.stamp_current_node1_local(
            Some(15),
            None,
            multiplicity * (eq68_value),
            15,
            multiplicity * (eq68_e792_d_n15),
        );
        let (eq69_e804, eq69_e804_d_n16,) = {
    if ((!s.b[1851]) && (p.p34 != 0.0)) {
        let eq69_e799: f64 = (1e-9 / 0.0001);
        let eq69_e801: f64 = (eq69_e799 * (nv16 - 0.0));
        let eq69_e802: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 13, eq69_e801);
        let eq69_e802_d_n16: f64 = (eq69_e799 * ddt_scale);
        (eq69_e802, eq69_e802_d_n16,)
    } else {
        (0.0, 0.0,)
    }
};
        let eq69_value: f64 = eq69_e804;
        stamper.stamp_current_node1_local(
            Some(16),
            None,
            multiplicity * (eq69_value),
            16,
            multiplicity * (eq69_e804_d_n16),
        );
        let (eq70_e816, eq70_e816_d_n13,) = {
    if ((!s.b[1851]) && (p.p34 != 0.0)) {
        let eq70_e811: f64 = (1e-9 / 0.0001);
        let eq70_e813: f64 = (eq70_e811 * (nv13 - 0.0));
        let eq70_e814: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 14, eq70_e813);
        let eq70_e814_d_n13: f64 = (eq70_e811 * ddt_scale);
        (eq70_e814, eq70_e814_d_n13,)
    } else {
        (0.0, 0.0,)
    }
};
        let eq70_value: f64 = eq70_e816;
        stamper.stamp_current_node1_local(
            Some(13),
            None,
            multiplicity * (eq70_value),
            13,
            multiplicity * (eq70_e816_d_n13),
        );
    }

    pub(super) fn stamp_reactive_equations_block_0(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        branches: &[usize; Instance::BRANCH_COUNT],
        multiplicity: f64,
    ) {
        let nv10 = ctx.node_voltage(nodes[10]);
        let nv14 = ctx.node_voltage(nodes[14]);
        let eq11_e367_q: f64 = s.v[594];
        let eq11_e368: f64 = (p.p50 * s.v[594]);
        let eq11_e368_d_n0: f64 = (p.p50 * s.dn[594][0]);
        let eq11_e368_d_n1: f64 = (p.p50 * s.dn[594][1]);
        let eq11_e368_d_n2: f64 = (p.p50 * s.dn[594][2]);
        let eq11_e368_d_n3: f64 = (p.p50 * s.dn[594][3]);
        let eq11_e368_d_n4: f64 = (p.p50 * s.dn[594][4]);
        let eq11_e368_d_n5: f64 = (p.p50 * s.dn[594][5]);
        let eq11_e368_d_n6: f64 = (p.p50 * s.dn[594][6]);
        let eq11_e368_d_n7: f64 = (p.p50 * s.dn[594][7]);
        let eq11_e368_d_n8: f64 = (p.p50 * s.dn[594][8]);
        let eq11_e368_d_n9: f64 = (p.p50 * s.dn[594][9]);
        let eq11_e368_d_n10: f64 = (p.p50 * s.dn[594][10]);
        let eq11_e368_d_n11: f64 = (p.p50 * s.dn[594][11]);
        let eq11_e368_d_n12: f64 = (p.p50 * s.dn[594][12]);
        let eq11_e368_d_n13: f64 = (p.p50 * s.dn[594][13]);
        let eq11_e368_d_n14: f64 = (p.p50 * s.dn[594][14]);
        let eq11_e368_d_n15: f64 = (p.p50 * s.dn[594][15]);
        let eq11_e368_d_n16: f64 = (p.p50 * s.dn[594][16]);
        let eq11_e368_d_n17: f64 = (p.p50 * s.dn[594][17]);
        let eq11_e368_d_n18: f64 = (p.p50 * s.dn[594][18]);
        let eq11_e368_d_b0: f64 = (p.p50 * s.db[594][0]);
        let eq11_e368_d_b1: f64 = (p.p50 * s.db[594][1]);
        let eq11_e368_d_b2: f64 = (p.p50 * s.db[594][2]);
        let eq11_e368_d_b3: f64 = (p.p50 * s.db[594][3]);
        let eq11_e368_d_b4: f64 = (p.p50 * s.db[594][4]);
        let eq11_e368_d_b5: f64 = (p.p50 * s.db[594][5]);
        let eq11_e368_d_b6: f64 = (p.p50 * s.db[594][6]);
        let eq11_e368_d_b7: f64 = (p.p50 * s.db[594][7]);
        let eq11_e368_d_b8: f64 = (p.p50 * s.db[594][8]);
        let eq11_e368_d_b9: f64 = (p.p50 * s.db[594][9]);
        let eq11_e368_d_b10: f64 = (p.p50 * s.db[594][10]);
        let eq11_e368_d_b11: f64 = (p.p50 * s.db[594][11]);
        let eq11_e368_d_b12: f64 = (p.p50 * s.db[594][12]);
        let eq11_e368_d_b13: f64 = (p.p50 * s.db[594][13]);
        let eq11_e368_d_b14: f64 = (p.p50 * s.db[594][14]);
        let eq11_e368_d_b15: f64 = (p.p50 * s.db[594][15]);
        let eq11_e368_q: f64 = (p.p50 * eq11_e367_q);
        let eq11_e368_q_d_n0: f64 = (p.p50 * s.dn[594][0]);
        let eq11_e368_q_d_n1: f64 = (p.p50 * s.dn[594][1]);
        let eq11_e368_q_d_n2: f64 = (p.p50 * s.dn[594][2]);
        let eq11_e368_q_d_n3: f64 = (p.p50 * s.dn[594][3]);
        let eq11_e368_q_d_n4: f64 = (p.p50 * s.dn[594][4]);
        let eq11_e368_q_d_n5: f64 = (p.p50 * s.dn[594][5]);
        let eq11_e368_q_d_n6: f64 = (p.p50 * s.dn[594][6]);
        let eq11_e368_q_d_n7: f64 = (p.p50 * s.dn[594][7]);
        let eq11_e368_q_d_n8: f64 = (p.p50 * s.dn[594][8]);
        let eq11_e368_q_d_n9: f64 = (p.p50 * s.dn[594][9]);
        let eq11_e368_q_d_n10: f64 = (p.p50 * s.dn[594][10]);
        let eq11_e368_q_d_n11: f64 = (p.p50 * s.dn[594][11]);
        let eq11_e368_q_d_n12: f64 = (p.p50 * s.dn[594][12]);
        let eq11_e368_q_d_n13: f64 = (p.p50 * s.dn[594][13]);
        let eq11_e368_q_d_n14: f64 = (p.p50 * s.dn[594][14]);
        let eq11_e368_q_d_n15: f64 = (p.p50 * s.dn[594][15]);
        let eq11_e368_q_d_n16: f64 = (p.p50 * s.dn[594][16]);
        let eq11_e368_q_d_n17: f64 = (p.p50 * s.dn[594][17]);
        let eq11_e368_q_d_n18: f64 = (p.p50 * s.dn[594][18]);
        let eq11_e368_q_d_b0: f64 = (p.p50 * s.db[594][0]);
        let eq11_e368_q_d_b1: f64 = (p.p50 * s.db[594][1]);
        let eq11_e368_q_d_b2: f64 = (p.p50 * s.db[594][2]);
        let eq11_e368_q_d_b3: f64 = (p.p50 * s.db[594][3]);
        let eq11_e368_q_d_b4: f64 = (p.p50 * s.db[594][4]);
        let eq11_e368_q_d_b5: f64 = (p.p50 * s.db[594][5]);
        let eq11_e368_q_d_b6: f64 = (p.p50 * s.db[594][6]);
        let eq11_e368_q_d_b7: f64 = (p.p50 * s.db[594][7]);
        let eq11_e368_q_d_b8: f64 = (p.p50 * s.db[594][8]);
        let eq11_e368_q_d_b9: f64 = (p.p50 * s.db[594][9]);
        let eq11_e368_q_d_b10: f64 = (p.p50 * s.db[594][10]);
        let eq11_e368_q_d_b11: f64 = (p.p50 * s.db[594][11]);
        let eq11_e368_q_d_b12: f64 = (p.p50 * s.db[594][12]);
        let eq11_e368_q_d_b13: f64 = (p.p50 * s.db[594][13]);
        let eq11_e368_q_d_b14: f64 = (p.p50 * s.db[594][14]);
        let eq11_e368_q_d_b15: f64 = (p.p50 * s.db[594][15]);
        let eq11_reactive_node_derivatives: [f64; 19] = [eq11_e368_q_d_n0, eq11_e368_q_d_n1, eq11_e368_q_d_n2, eq11_e368_q_d_n3, eq11_e368_q_d_n4, eq11_e368_q_d_n5, eq11_e368_q_d_n6, eq11_e368_q_d_n7, eq11_e368_q_d_n8, eq11_e368_q_d_n9, eq11_e368_q_d_n10, eq11_e368_q_d_n11, eq11_e368_q_d_n12, eq11_e368_q_d_n13, eq11_e368_q_d_n14, eq11_e368_q_d_n15, eq11_e368_q_d_n16, eq11_e368_q_d_n17, eq11_e368_q_d_n18];
        let eq11_reactive_branch_derivatives: [f64; 16] = [eq11_e368_q_d_b0, eq11_e368_q_d_b1, eq11_e368_q_d_b2, eq11_e368_q_d_b3, eq11_e368_q_d_b4, eq11_e368_q_d_b5, eq11_e368_q_d_b6, eq11_e368_q_d_b7, eq11_e368_q_d_b8, eq11_e368_q_d_b9, eq11_e368_q_d_b10, eq11_e368_q_d_b11, eq11_e368_q_d_b12, eq11_e368_q_d_b13, eq11_e368_q_d_b14, eq11_e368_q_d_b15];
        stamper.stamp_current_reactive_dense(
            Some(nodes[11]),
            Some(nodes[7]),
            nodes,
            &eq11_reactive_node_derivatives,
            branches,
            &eq11_reactive_branch_derivatives,
            multiplicity,
        );
        let eq12_e371_q: f64 = s.v[198];
        let eq12_e372: f64 = (p.p50 * s.v[198]);
        let eq12_e372_d_n0: f64 = (p.p50 * s.dn[198][0]);
        let eq12_e372_d_n1: f64 = (p.p50 * s.dn[198][1]);
        let eq12_e372_d_n2: f64 = (p.p50 * s.dn[198][2]);
        let eq12_e372_d_n3: f64 = (p.p50 * s.dn[198][3]);
        let eq12_e372_d_n4: f64 = (p.p50 * s.dn[198][4]);
        let eq12_e372_d_n5: f64 = (p.p50 * s.dn[198][5]);
        let eq12_e372_d_n6: f64 = (p.p50 * s.dn[198][6]);
        let eq12_e372_d_n7: f64 = (p.p50 * s.dn[198][7]);
        let eq12_e372_d_n8: f64 = (p.p50 * s.dn[198][8]);
        let eq12_e372_d_n9: f64 = (p.p50 * s.dn[198][9]);
        let eq12_e372_d_n10: f64 = (p.p50 * s.dn[198][10]);
        let eq12_e372_d_n11: f64 = (p.p50 * s.dn[198][11]);
        let eq12_e372_d_n12: f64 = (p.p50 * s.dn[198][12]);
        let eq12_e372_d_n13: f64 = (p.p50 * s.dn[198][13]);
        let eq12_e372_d_n14: f64 = (p.p50 * s.dn[198][14]);
        let eq12_e372_d_n15: f64 = (p.p50 * s.dn[198][15]);
        let eq12_e372_d_n16: f64 = (p.p50 * s.dn[198][16]);
        let eq12_e372_d_n17: f64 = (p.p50 * s.dn[198][17]);
        let eq12_e372_d_n18: f64 = (p.p50 * s.dn[198][18]);
        let eq12_e372_d_b0: f64 = (p.p50 * s.db[198][0]);
        let eq12_e372_d_b1: f64 = (p.p50 * s.db[198][1]);
        let eq12_e372_d_b2: f64 = (p.p50 * s.db[198][2]);
        let eq12_e372_d_b3: f64 = (p.p50 * s.db[198][3]);
        let eq12_e372_d_b4: f64 = (p.p50 * s.db[198][4]);
        let eq12_e372_d_b5: f64 = (p.p50 * s.db[198][5]);
        let eq12_e372_d_b6: f64 = (p.p50 * s.db[198][6]);
        let eq12_e372_d_b7: f64 = (p.p50 * s.db[198][7]);
        let eq12_e372_d_b8: f64 = (p.p50 * s.db[198][8]);
        let eq12_e372_d_b9: f64 = (p.p50 * s.db[198][9]);
        let eq12_e372_d_b10: f64 = (p.p50 * s.db[198][10]);
        let eq12_e372_d_b11: f64 = (p.p50 * s.db[198][11]);
        let eq12_e372_d_b12: f64 = (p.p50 * s.db[198][12]);
        let eq12_e372_d_b13: f64 = (p.p50 * s.db[198][13]);
        let eq12_e372_d_b14: f64 = (p.p50 * s.db[198][14]);
        let eq12_e372_d_b15: f64 = (p.p50 * s.db[198][15]);
        let eq12_e372_q: f64 = (p.p50 * eq12_e371_q);
        let eq12_e372_q_d_n0: f64 = (p.p50 * s.dn[198][0]);
        let eq12_e372_q_d_n1: f64 = (p.p50 * s.dn[198][1]);
        let eq12_e372_q_d_n2: f64 = (p.p50 * s.dn[198][2]);
        let eq12_e372_q_d_n3: f64 = (p.p50 * s.dn[198][3]);
        let eq12_e372_q_d_n4: f64 = (p.p50 * s.dn[198][4]);
        let eq12_e372_q_d_n5: f64 = (p.p50 * s.dn[198][5]);
        let eq12_e372_q_d_n6: f64 = (p.p50 * s.dn[198][6]);
        let eq12_e372_q_d_n7: f64 = (p.p50 * s.dn[198][7]);
        let eq12_e372_q_d_n8: f64 = (p.p50 * s.dn[198][8]);
        let eq12_e372_q_d_n9: f64 = (p.p50 * s.dn[198][9]);
        let eq12_e372_q_d_n10: f64 = (p.p50 * s.dn[198][10]);
        let eq12_e372_q_d_n11: f64 = (p.p50 * s.dn[198][11]);
        let eq12_e372_q_d_n12: f64 = (p.p50 * s.dn[198][12]);
        let eq12_e372_q_d_n13: f64 = (p.p50 * s.dn[198][13]);
        let eq12_e372_q_d_n14: f64 = (p.p50 * s.dn[198][14]);
        let eq12_e372_q_d_n15: f64 = (p.p50 * s.dn[198][15]);
        let eq12_e372_q_d_n16: f64 = (p.p50 * s.dn[198][16]);
        let eq12_e372_q_d_n17: f64 = (p.p50 * s.dn[198][17]);
        let eq12_e372_q_d_n18: f64 = (p.p50 * s.dn[198][18]);
        let eq12_e372_q_d_b0: f64 = (p.p50 * s.db[198][0]);
        let eq12_e372_q_d_b1: f64 = (p.p50 * s.db[198][1]);
        let eq12_e372_q_d_b2: f64 = (p.p50 * s.db[198][2]);
        let eq12_e372_q_d_b3: f64 = (p.p50 * s.db[198][3]);
        let eq12_e372_q_d_b4: f64 = (p.p50 * s.db[198][4]);
        let eq12_e372_q_d_b5: f64 = (p.p50 * s.db[198][5]);
        let eq12_e372_q_d_b6: f64 = (p.p50 * s.db[198][6]);
        let eq12_e372_q_d_b7: f64 = (p.p50 * s.db[198][7]);
        let eq12_e372_q_d_b8: f64 = (p.p50 * s.db[198][8]);
        let eq12_e372_q_d_b9: f64 = (p.p50 * s.db[198][9]);
        let eq12_e372_q_d_b10: f64 = (p.p50 * s.db[198][10]);
        let eq12_e372_q_d_b11: f64 = (p.p50 * s.db[198][11]);
        let eq12_e372_q_d_b12: f64 = (p.p50 * s.db[198][12]);
        let eq12_e372_q_d_b13: f64 = (p.p50 * s.db[198][13]);
        let eq12_e372_q_d_b14: f64 = (p.p50 * s.db[198][14]);
        let eq12_e372_q_d_b15: f64 = (p.p50 * s.db[198][15]);
        let eq12_reactive_node_derivatives: [f64; 19] = [eq12_e372_q_d_n0, eq12_e372_q_d_n1, eq12_e372_q_d_n2, eq12_e372_q_d_n3, eq12_e372_q_d_n4, eq12_e372_q_d_n5, eq12_e372_q_d_n6, eq12_e372_q_d_n7, eq12_e372_q_d_n8, eq12_e372_q_d_n9, eq12_e372_q_d_n10, eq12_e372_q_d_n11, eq12_e372_q_d_n12, eq12_e372_q_d_n13, eq12_e372_q_d_n14, eq12_e372_q_d_n15, eq12_e372_q_d_n16, eq12_e372_q_d_n17, eq12_e372_q_d_n18];
        let eq12_reactive_branch_derivatives: [f64; 16] = [eq12_e372_q_d_b0, eq12_e372_q_d_b1, eq12_e372_q_d_b2, eq12_e372_q_d_b3, eq12_e372_q_d_b4, eq12_e372_q_d_b5, eq12_e372_q_d_b6, eq12_e372_q_d_b7, eq12_e372_q_d_b8, eq12_e372_q_d_b9, eq12_e372_q_d_b10, eq12_e372_q_d_b11, eq12_e372_q_d_b12, eq12_e372_q_d_b13, eq12_e372_q_d_b14, eq12_e372_q_d_b15];
        stamper.stamp_current_reactive_dense(
            Some(nodes[6]),
            Some(nodes[7]),
            nodes,
            &eq12_reactive_node_derivatives,
            branches,
            &eq12_reactive_branch_derivatives,
            multiplicity,
        );
        let eq13_e375_q: f64 = s.v[196];
        let eq13_e376: f64 = (p.p50 * s.v[196]);
        let eq13_e376_d_n0: f64 = (p.p50 * s.dn[196][0]);
        let eq13_e376_d_n1: f64 = (p.p50 * s.dn[196][1]);
        let eq13_e376_d_n2: f64 = (p.p50 * s.dn[196][2]);
        let eq13_e376_d_n3: f64 = (p.p50 * s.dn[196][3]);
        let eq13_e376_d_n4: f64 = (p.p50 * s.dn[196][4]);
        let eq13_e376_d_n5: f64 = (p.p50 * s.dn[196][5]);
        let eq13_e376_d_n6: f64 = (p.p50 * s.dn[196][6]);
        let eq13_e376_d_n7: f64 = (p.p50 * s.dn[196][7]);
        let eq13_e376_d_n8: f64 = (p.p50 * s.dn[196][8]);
        let eq13_e376_d_n9: f64 = (p.p50 * s.dn[196][9]);
        let eq13_e376_d_n10: f64 = (p.p50 * s.dn[196][10]);
        let eq13_e376_d_n11: f64 = (p.p50 * s.dn[196][11]);
        let eq13_e376_d_n12: f64 = (p.p50 * s.dn[196][12]);
        let eq13_e376_d_n13: f64 = (p.p50 * s.dn[196][13]);
        let eq13_e376_d_n14: f64 = (p.p50 * s.dn[196][14]);
        let eq13_e376_d_n15: f64 = (p.p50 * s.dn[196][15]);
        let eq13_e376_d_n16: f64 = (p.p50 * s.dn[196][16]);
        let eq13_e376_d_n17: f64 = (p.p50 * s.dn[196][17]);
        let eq13_e376_d_n18: f64 = (p.p50 * s.dn[196][18]);
        let eq13_e376_d_b0: f64 = (p.p50 * s.db[196][0]);
        let eq13_e376_d_b1: f64 = (p.p50 * s.db[196][1]);
        let eq13_e376_d_b2: f64 = (p.p50 * s.db[196][2]);
        let eq13_e376_d_b3: f64 = (p.p50 * s.db[196][3]);
        let eq13_e376_d_b4: f64 = (p.p50 * s.db[196][4]);
        let eq13_e376_d_b5: f64 = (p.p50 * s.db[196][5]);
        let eq13_e376_d_b6: f64 = (p.p50 * s.db[196][6]);
        let eq13_e376_d_b7: f64 = (p.p50 * s.db[196][7]);
        let eq13_e376_d_b8: f64 = (p.p50 * s.db[196][8]);
        let eq13_e376_d_b9: f64 = (p.p50 * s.db[196][9]);
        let eq13_e376_d_b10: f64 = (p.p50 * s.db[196][10]);
        let eq13_e376_d_b11: f64 = (p.p50 * s.db[196][11]);
        let eq13_e376_d_b12: f64 = (p.p50 * s.db[196][12]);
        let eq13_e376_d_b13: f64 = (p.p50 * s.db[196][13]);
        let eq13_e376_d_b14: f64 = (p.p50 * s.db[196][14]);
        let eq13_e376_d_b15: f64 = (p.p50 * s.db[196][15]);
        let eq13_e376_q: f64 = (p.p50 * eq13_e375_q);
        let eq13_e376_q_d_n0: f64 = (p.p50 * s.dn[196][0]);
        let eq13_e376_q_d_n1: f64 = (p.p50 * s.dn[196][1]);
        let eq13_e376_q_d_n2: f64 = (p.p50 * s.dn[196][2]);
        let eq13_e376_q_d_n3: f64 = (p.p50 * s.dn[196][3]);
        let eq13_e376_q_d_n4: f64 = (p.p50 * s.dn[196][4]);
        let eq13_e376_q_d_n5: f64 = (p.p50 * s.dn[196][5]);
        let eq13_e376_q_d_n6: f64 = (p.p50 * s.dn[196][6]);
        let eq13_e376_q_d_n7: f64 = (p.p50 * s.dn[196][7]);
        let eq13_e376_q_d_n8: f64 = (p.p50 * s.dn[196][8]);
        let eq13_e376_q_d_n9: f64 = (p.p50 * s.dn[196][9]);
        let eq13_e376_q_d_n10: f64 = (p.p50 * s.dn[196][10]);
        let eq13_e376_q_d_n11: f64 = (p.p50 * s.dn[196][11]);
        let eq13_e376_q_d_n12: f64 = (p.p50 * s.dn[196][12]);
        let eq13_e376_q_d_n13: f64 = (p.p50 * s.dn[196][13]);
        let eq13_e376_q_d_n14: f64 = (p.p50 * s.dn[196][14]);
        let eq13_e376_q_d_n15: f64 = (p.p50 * s.dn[196][15]);
        let eq13_e376_q_d_n16: f64 = (p.p50 * s.dn[196][16]);
        let eq13_e376_q_d_n17: f64 = (p.p50 * s.dn[196][17]);
        let eq13_e376_q_d_n18: f64 = (p.p50 * s.dn[196][18]);
        let eq13_e376_q_d_b0: f64 = (p.p50 * s.db[196][0]);
        let eq13_e376_q_d_b1: f64 = (p.p50 * s.db[196][1]);
        let eq13_e376_q_d_b2: f64 = (p.p50 * s.db[196][2]);
        let eq13_e376_q_d_b3: f64 = (p.p50 * s.db[196][3]);
        let eq13_e376_q_d_b4: f64 = (p.p50 * s.db[196][4]);
        let eq13_e376_q_d_b5: f64 = (p.p50 * s.db[196][5]);
        let eq13_e376_q_d_b6: f64 = (p.p50 * s.db[196][6]);
        let eq13_e376_q_d_b7: f64 = (p.p50 * s.db[196][7]);
        let eq13_e376_q_d_b8: f64 = (p.p50 * s.db[196][8]);
        let eq13_e376_q_d_b9: f64 = (p.p50 * s.db[196][9]);
        let eq13_e376_q_d_b10: f64 = (p.p50 * s.db[196][10]);
        let eq13_e376_q_d_b11: f64 = (p.p50 * s.db[196][11]);
        let eq13_e376_q_d_b12: f64 = (p.p50 * s.db[196][12]);
        let eq13_e376_q_d_b13: f64 = (p.p50 * s.db[196][13]);
        let eq13_e376_q_d_b14: f64 = (p.p50 * s.db[196][14]);
        let eq13_e376_q_d_b15: f64 = (p.p50 * s.db[196][15]);
        let eq13_reactive_node_derivatives: [f64; 19] = [eq13_e376_q_d_n0, eq13_e376_q_d_n1, eq13_e376_q_d_n2, eq13_e376_q_d_n3, eq13_e376_q_d_n4, eq13_e376_q_d_n5, eq13_e376_q_d_n6, eq13_e376_q_d_n7, eq13_e376_q_d_n8, eq13_e376_q_d_n9, eq13_e376_q_d_n10, eq13_e376_q_d_n11, eq13_e376_q_d_n12, eq13_e376_q_d_n13, eq13_e376_q_d_n14, eq13_e376_q_d_n15, eq13_e376_q_d_n16, eq13_e376_q_d_n17, eq13_e376_q_d_n18];
        let eq13_reactive_branch_derivatives: [f64; 16] = [eq13_e376_q_d_b0, eq13_e376_q_d_b1, eq13_e376_q_d_b2, eq13_e376_q_d_b3, eq13_e376_q_d_b4, eq13_e376_q_d_b5, eq13_e376_q_d_b6, eq13_e376_q_d_b7, eq13_e376_q_d_b8, eq13_e376_q_d_b9, eq13_e376_q_d_b10, eq13_e376_q_d_b11, eq13_e376_q_d_b12, eq13_e376_q_d_b13, eq13_e376_q_d_b14, eq13_e376_q_d_b15];
        stamper.stamp_current_reactive_dense(
            Some(nodes[12]),
            Some(nodes[7]),
            nodes,
            &eq13_reactive_node_derivatives,
            branches,
            &eq13_reactive_branch_derivatives,
            multiplicity,
        );
        let eq19_e405: f64 = ((nv14 - 0.0) * s.v[617]);
        let eq19_e405_d_n0: f64 = ((nv14 - 0.0) * s.dn[617][0]);
        let eq19_e405_d_n1: f64 = ((nv14 - 0.0) * s.dn[617][1]);
        let eq19_e405_d_n2: f64 = ((nv14 - 0.0) * s.dn[617][2]);
        let eq19_e405_d_n3: f64 = ((nv14 - 0.0) * s.dn[617][3]);
        let eq19_e405_d_n4: f64 = ((nv14 - 0.0) * s.dn[617][4]);
        let eq19_e405_d_n5: f64 = ((nv14 - 0.0) * s.dn[617][5]);
        let eq19_e405_d_n6: f64 = ((nv14 - 0.0) * s.dn[617][6]);
        let eq19_e405_d_n7: f64 = ((nv14 - 0.0) * s.dn[617][7]);
        let eq19_e405_d_n8: f64 = ((nv14 - 0.0) * s.dn[617][8]);
        let eq19_e405_d_n9: f64 = ((nv14 - 0.0) * s.dn[617][9]);
        let eq19_e405_d_n10: f64 = ((nv14 - 0.0) * s.dn[617][10]);
        let eq19_e405_d_n11: f64 = ((nv14 - 0.0) * s.dn[617][11]);
        let eq19_e405_d_n12: f64 = ((nv14 - 0.0) * s.dn[617][12]);
        let eq19_e405_d_n13: f64 = ((nv14 - 0.0) * s.dn[617][13]);
        let eq19_e405_d_n14: f64 = (s.v[617] + ((nv14 - 0.0) * s.dn[617][14]));
        let eq19_e405_d_n15: f64 = ((nv14 - 0.0) * s.dn[617][15]);
        let eq19_e405_d_n16: f64 = ((nv14 - 0.0) * s.dn[617][16]);
        let eq19_e405_d_n17: f64 = ((nv14 - 0.0) * s.dn[617][17]);
        let eq19_e405_d_n18: f64 = ((nv14 - 0.0) * s.dn[617][18]);
        let eq19_e405_d_b0: f64 = ((nv14 - 0.0) * s.db[617][0]);
        let eq19_e405_d_b1: f64 = ((nv14 - 0.0) * s.db[617][1]);
        let eq19_e405_d_b2: f64 = ((nv14 - 0.0) * s.db[617][2]);
        let eq19_e405_d_b3: f64 = ((nv14 - 0.0) * s.db[617][3]);
        let eq19_e405_d_b4: f64 = ((nv14 - 0.0) * s.db[617][4]);
        let eq19_e405_d_b5: f64 = ((nv14 - 0.0) * s.db[617][5]);
        let eq19_e405_d_b6: f64 = ((nv14 - 0.0) * s.db[617][6]);
        let eq19_e405_d_b7: f64 = ((nv14 - 0.0) * s.db[617][7]);
        let eq19_e405_d_b8: f64 = ((nv14 - 0.0) * s.db[617][8]);
        let eq19_e405_d_b9: f64 = ((nv14 - 0.0) * s.db[617][9]);
        let eq19_e405_d_b10: f64 = ((nv14 - 0.0) * s.db[617][10]);
        let eq19_e405_d_b11: f64 = ((nv14 - 0.0) * s.db[617][11]);
        let eq19_e405_d_b12: f64 = ((nv14 - 0.0) * s.db[617][12]);
        let eq19_e405_d_b13: f64 = ((nv14 - 0.0) * s.db[617][13]);
        let eq19_e405_d_b14: f64 = ((nv14 - 0.0) * s.db[617][14]);
        let eq19_e405_d_b15: f64 = ((nv14 - 0.0) * s.db[617][15]);
        let eq19_e406_q: f64 = eq19_e405;
        let eq19_reactive_node_derivatives: [f64; 19] = [eq19_e405_d_n0, eq19_e405_d_n1, eq19_e405_d_n2, eq19_e405_d_n3, eq19_e405_d_n4, eq19_e405_d_n5, eq19_e405_d_n6, eq19_e405_d_n7, eq19_e405_d_n8, eq19_e405_d_n9, eq19_e405_d_n10, eq19_e405_d_n11, eq19_e405_d_n12, eq19_e405_d_n13, eq19_e405_d_n14, eq19_e405_d_n15, eq19_e405_d_n16, eq19_e405_d_n17, eq19_e405_d_n18];
        let eq19_reactive_branch_derivatives: [f64; 16] = [eq19_e405_d_b0, eq19_e405_d_b1, eq19_e405_d_b2, eq19_e405_d_b3, eq19_e405_d_b4, eq19_e405_d_b5, eq19_e405_d_b6, eq19_e405_d_b7, eq19_e405_d_b8, eq19_e405_d_b9, eq19_e405_d_b10, eq19_e405_d_b11, eq19_e405_d_b12, eq19_e405_d_b13, eq19_e405_d_b14, eq19_e405_d_b15];
        stamper.stamp_current_reactive_dense(
            Some(nodes[11]),
            Some(nodes[7]),
            nodes,
            &eq19_reactive_node_derivatives,
            branches,
            &eq19_reactive_branch_derivatives,
            multiplicity,
        );
        let eq20_e409: f64 = ((nv14 - 0.0) * s.v[618]);
        let eq20_e409_d_n0: f64 = ((nv14 - 0.0) * s.dn[618][0]);
        let eq20_e409_d_n1: f64 = ((nv14 - 0.0) * s.dn[618][1]);
        let eq20_e409_d_n2: f64 = ((nv14 - 0.0) * s.dn[618][2]);
        let eq20_e409_d_n3: f64 = ((nv14 - 0.0) * s.dn[618][3]);
        let eq20_e409_d_n4: f64 = ((nv14 - 0.0) * s.dn[618][4]);
        let eq20_e409_d_n5: f64 = ((nv14 - 0.0) * s.dn[618][5]);
        let eq20_e409_d_n6: f64 = ((nv14 - 0.0) * s.dn[618][6]);
        let eq20_e409_d_n7: f64 = ((nv14 - 0.0) * s.dn[618][7]);
        let eq20_e409_d_n8: f64 = ((nv14 - 0.0) * s.dn[618][8]);
        let eq20_e409_d_n9: f64 = ((nv14 - 0.0) * s.dn[618][9]);
        let eq20_e409_d_n10: f64 = ((nv14 - 0.0) * s.dn[618][10]);
        let eq20_e409_d_n11: f64 = ((nv14 - 0.0) * s.dn[618][11]);
        let eq20_e409_d_n12: f64 = ((nv14 - 0.0) * s.dn[618][12]);
        let eq20_e409_d_n13: f64 = ((nv14 - 0.0) * s.dn[618][13]);
        let eq20_e409_d_n14: f64 = (s.v[618] + ((nv14 - 0.0) * s.dn[618][14]));
        let eq20_e409_d_n15: f64 = ((nv14 - 0.0) * s.dn[618][15]);
        let eq20_e409_d_n16: f64 = ((nv14 - 0.0) * s.dn[618][16]);
        let eq20_e409_d_n17: f64 = ((nv14 - 0.0) * s.dn[618][17]);
        let eq20_e409_d_n18: f64 = ((nv14 - 0.0) * s.dn[618][18]);
        let eq20_e409_d_b0: f64 = ((nv14 - 0.0) * s.db[618][0]);
        let eq20_e409_d_b1: f64 = ((nv14 - 0.0) * s.db[618][1]);
        let eq20_e409_d_b2: f64 = ((nv14 - 0.0) * s.db[618][2]);
        let eq20_e409_d_b3: f64 = ((nv14 - 0.0) * s.db[618][3]);
        let eq20_e409_d_b4: f64 = ((nv14 - 0.0) * s.db[618][4]);
        let eq20_e409_d_b5: f64 = ((nv14 - 0.0) * s.db[618][5]);
        let eq20_e409_d_b6: f64 = ((nv14 - 0.0) * s.db[618][6]);
        let eq20_e409_d_b7: f64 = ((nv14 - 0.0) * s.db[618][7]);
        let eq20_e409_d_b8: f64 = ((nv14 - 0.0) * s.db[618][8]);
        let eq20_e409_d_b9: f64 = ((nv14 - 0.0) * s.db[618][9]);
        let eq20_e409_d_b10: f64 = ((nv14 - 0.0) * s.db[618][10]);
        let eq20_e409_d_b11: f64 = ((nv14 - 0.0) * s.db[618][11]);
        let eq20_e409_d_b12: f64 = ((nv14 - 0.0) * s.db[618][12]);
        let eq20_e409_d_b13: f64 = ((nv14 - 0.0) * s.db[618][13]);
        let eq20_e409_d_b14: f64 = ((nv14 - 0.0) * s.db[618][14]);
        let eq20_e409_d_b15: f64 = ((nv14 - 0.0) * s.db[618][15]);
        let eq20_e410_q: f64 = eq20_e409;
        let eq20_reactive_node_derivatives: [f64; 19] = [eq20_e409_d_n0, eq20_e409_d_n1, eq20_e409_d_n2, eq20_e409_d_n3, eq20_e409_d_n4, eq20_e409_d_n5, eq20_e409_d_n6, eq20_e409_d_n7, eq20_e409_d_n8, eq20_e409_d_n9, eq20_e409_d_n10, eq20_e409_d_n11, eq20_e409_d_n12, eq20_e409_d_n13, eq20_e409_d_n14, eq20_e409_d_n15, eq20_e409_d_n16, eq20_e409_d_n17, eq20_e409_d_n18];
        let eq20_reactive_branch_derivatives: [f64; 16] = [eq20_e409_d_b0, eq20_e409_d_b1, eq20_e409_d_b2, eq20_e409_d_b3, eq20_e409_d_b4, eq20_e409_d_b5, eq20_e409_d_b6, eq20_e409_d_b7, eq20_e409_d_b8, eq20_e409_d_b9, eq20_e409_d_b10, eq20_e409_d_b11, eq20_e409_d_b12, eq20_e409_d_b13, eq20_e409_d_b14, eq20_e409_d_b15];
        stamper.stamp_current_reactive_dense(
            Some(nodes[11]),
            Some(nodes[6]),
            nodes,
            &eq20_reactive_node_derivatives,
            branches,
            &eq20_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq31_e491, eq31_e491_d_n0, eq31_e491_d_n1, eq31_e491_d_n2, eq31_e491_d_n3, eq31_e491_d_n4, eq31_e491_d_n5, eq31_e491_d_n6, eq31_e491_d_n7, eq31_e491_d_n8, eq31_e491_d_n9, eq31_e491_d_n10, eq31_e491_d_n11, eq31_e491_d_n12, eq31_e491_d_n13, eq31_e491_d_n14, eq31_e491_d_n15, eq31_e491_d_n16, eq31_e491_d_n17, eq31_e491_d_n18, eq31_e491_d_b0, eq31_e491_d_b1, eq31_e491_d_b2, eq31_e491_d_b3, eq31_e491_d_b4, eq31_e491_d_b5, eq31_e491_d_b6, eq31_e491_d_b7, eq31_e491_d_b8, eq31_e491_d_b9, eq31_e491_d_b10, eq31_e491_d_b11, eq31_e491_d_b12, eq31_e491_d_b13, eq31_e491_d_b14, eq31_e491_d_b15, eq31_e491_q, eq31_e491_q_d_n0, eq31_e491_q_d_n1, eq31_e491_q_d_n2, eq31_e491_q_d_n3, eq31_e491_q_d_n4, eq31_e491_q_d_n5, eq31_e491_q_d_n6, eq31_e491_q_d_n7, eq31_e491_q_d_n8, eq31_e491_q_d_n9, eq31_e491_q_d_n10, eq31_e491_q_d_n11, eq31_e491_q_d_n12, eq31_e491_q_d_n13, eq31_e491_q_d_n14, eq31_e491_q_d_n15, eq31_e491_q_d_n16, eq31_e491_q_d_n17, eq31_e491_q_d_n18, eq31_e491_q_d_b0, eq31_e491_q_d_b1, eq31_e491_q_d_b2, eq31_e491_q_d_b3, eq31_e491_q_d_b4, eq31_e491_q_d_b5, eq31_e491_q_d_b6, eq31_e491_q_d_b7, eq31_e491_q_d_b8, eq31_e491_q_d_b9, eq31_e491_q_d_b10, eq31_e491_q_d_b11, eq31_e491_q_d_b12, eq31_e491_q_d_b13, eq31_e491_q_d_b14, eq31_e491_q_d_b15,) = {
    if s.b[1850] {
        let eq31_e488: f64 = (s.v[563] * (nv10 - 0.0));
        let eq31_e488_d_n0: f64 = (s.dn[563][0] * (nv10 - 0.0));
        let eq31_e488_d_n1: f64 = (s.dn[563][1] * (nv10 - 0.0));
        let eq31_e488_d_n2: f64 = (s.dn[563][2] * (nv10 - 0.0));
        let eq31_e488_d_n3: f64 = (s.dn[563][3] * (nv10 - 0.0));
        let eq31_e488_d_n4: f64 = (s.dn[563][4] * (nv10 - 0.0));
        let eq31_e488_d_n5: f64 = (s.dn[563][5] * (nv10 - 0.0));
        let eq31_e488_d_n6: f64 = (s.dn[563][6] * (nv10 - 0.0));
        let eq31_e488_d_n7: f64 = (s.dn[563][7] * (nv10 - 0.0));
        let eq31_e488_d_n8: f64 = (s.dn[563][8] * (nv10 - 0.0));
        let eq31_e488_d_n9: f64 = (s.dn[563][9] * (nv10 - 0.0));
        let eq31_e488_d_n10: f64 = ((s.dn[563][10] * (nv10 - 0.0)) + s.v[563]);
        let eq31_e488_d_n11: f64 = (s.dn[563][11] * (nv10 - 0.0));
        let eq31_e488_d_n12: f64 = (s.dn[563][12] * (nv10 - 0.0));
        let eq31_e488_d_n13: f64 = (s.dn[563][13] * (nv10 - 0.0));
        let eq31_e488_d_n14: f64 = (s.dn[563][14] * (nv10 - 0.0));
        let eq31_e488_d_n15: f64 = (s.dn[563][15] * (nv10 - 0.0));
        let eq31_e488_d_n16: f64 = (s.dn[563][16] * (nv10 - 0.0));
        let eq31_e488_d_n17: f64 = (s.dn[563][17] * (nv10 - 0.0));
        let eq31_e488_d_n18: f64 = (s.dn[563][18] * (nv10 - 0.0));
        let eq31_e488_d_b0: f64 = (s.db[563][0] * (nv10 - 0.0));
        let eq31_e488_d_b1: f64 = (s.db[563][1] * (nv10 - 0.0));
        let eq31_e488_d_b2: f64 = (s.db[563][2] * (nv10 - 0.0));
        let eq31_e488_d_b3: f64 = (s.db[563][3] * (nv10 - 0.0));
        let eq31_e488_d_b4: f64 = (s.db[563][4] * (nv10 - 0.0));
        let eq31_e488_d_b5: f64 = (s.db[563][5] * (nv10 - 0.0));
        let eq31_e488_d_b6: f64 = (s.db[563][6] * (nv10 - 0.0));
        let eq31_e488_d_b7: f64 = (s.db[563][7] * (nv10 - 0.0));
        let eq31_e488_d_b8: f64 = (s.db[563][8] * (nv10 - 0.0));
        let eq31_e488_d_b9: f64 = (s.db[563][9] * (nv10 - 0.0));
        let eq31_e488_d_b10: f64 = (s.db[563][10] * (nv10 - 0.0));
        let eq31_e488_d_b11: f64 = (s.db[563][11] * (nv10 - 0.0));
        let eq31_e488_d_b12: f64 = (s.db[563][12] * (nv10 - 0.0));
        let eq31_e488_d_b13: f64 = (s.db[563][13] * (nv10 - 0.0));
        let eq31_e488_d_b14: f64 = (s.db[563][14] * (nv10 - 0.0));
        let eq31_e488_d_b15: f64 = (s.db[563][15] * (nv10 - 0.0));
        let eq31_e489_q: f64 = eq31_e488;
        (eq31_e488, eq31_e488_d_n0, eq31_e488_d_n1, eq31_e488_d_n2, eq31_e488_d_n3, eq31_e488_d_n4, eq31_e488_d_n5, eq31_e488_d_n6, eq31_e488_d_n7, eq31_e488_d_n8, eq31_e488_d_n9, eq31_e488_d_n10, eq31_e488_d_n11, eq31_e488_d_n12, eq31_e488_d_n13, eq31_e488_d_n14, eq31_e488_d_n15, eq31_e488_d_n16, eq31_e488_d_n17, eq31_e488_d_n18, eq31_e488_d_b0, eq31_e488_d_b1, eq31_e488_d_b2, eq31_e488_d_b3, eq31_e488_d_b4, eq31_e488_d_b5, eq31_e488_d_b6, eq31_e488_d_b7, eq31_e488_d_b8, eq31_e488_d_b9, eq31_e488_d_b10, eq31_e488_d_b11, eq31_e488_d_b12, eq31_e488_d_b13, eq31_e488_d_b14, eq31_e488_d_b15, eq31_e489_q, eq31_e488_d_n0, eq31_e488_d_n1, eq31_e488_d_n2, eq31_e488_d_n3, eq31_e488_d_n4, eq31_e488_d_n5, eq31_e488_d_n6, eq31_e488_d_n7, eq31_e488_d_n8, eq31_e488_d_n9, eq31_e488_d_n10, eq31_e488_d_n11, eq31_e488_d_n12, eq31_e488_d_n13, eq31_e488_d_n14, eq31_e488_d_n15, eq31_e488_d_n16, eq31_e488_d_n17, eq31_e488_d_n18, eq31_e488_d_b0, eq31_e488_d_b1, eq31_e488_d_b2, eq31_e488_d_b3, eq31_e488_d_b4, eq31_e488_d_b5, eq31_e488_d_b6, eq31_e488_d_b7, eq31_e488_d_b8, eq31_e488_d_b9, eq31_e488_d_b10, eq31_e488_d_b11, eq31_e488_d_b12, eq31_e488_d_b13, eq31_e488_d_b14, eq31_e488_d_b15,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq31_reactive_node_derivatives: [f64; 19] = [eq31_e491_q_d_n0, eq31_e491_q_d_n1, eq31_e491_q_d_n2, eq31_e491_q_d_n3, eq31_e491_q_d_n4, eq31_e491_q_d_n5, eq31_e491_q_d_n6, eq31_e491_q_d_n7, eq31_e491_q_d_n8, eq31_e491_q_d_n9, eq31_e491_q_d_n10, eq31_e491_q_d_n11, eq31_e491_q_d_n12, eq31_e491_q_d_n13, eq31_e491_q_d_n14, eq31_e491_q_d_n15, eq31_e491_q_d_n16, eq31_e491_q_d_n17, eq31_e491_q_d_n18];
        let eq31_reactive_branch_derivatives: [f64; 16] = [eq31_e491_q_d_b0, eq31_e491_q_d_b1, eq31_e491_q_d_b2, eq31_e491_q_d_b3, eq31_e491_q_d_b4, eq31_e491_q_d_b5, eq31_e491_q_d_b6, eq31_e491_q_d_b7, eq31_e491_q_d_b8, eq31_e491_q_d_b9, eq31_e491_q_d_b10, eq31_e491_q_d_b11, eq31_e491_q_d_b12, eq31_e491_q_d_b13, eq31_e491_q_d_b14, eq31_e491_q_d_b15];
        stamper.stamp_current_reactive_dense(
            Some(nodes[10]),
            None,
            nodes,
            &eq31_reactive_node_derivatives,
            branches,
            &eq31_reactive_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equations_block_1(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        branches: &[usize; Instance::BRANCH_COUNT],
        multiplicity: f64,
    ) {
        let nv13 = ctx.node_voltage(nodes[13]);
        let nv15 = ctx.node_voltage(nodes[15]);
        let nv16 = ctx.node_voltage(nodes[16]);
        let nv17 = ctx.node_voltage(nodes[17]);
        let nv18 = ctx.node_voltage(nodes[18]);
        let (eq35_e523, eq35_e523_d_n0, eq35_e523_d_n1, eq35_e523_d_n2, eq35_e523_d_n3, eq35_e523_d_n4, eq35_e523_d_n5, eq35_e523_d_n6, eq35_e523_d_n7, eq35_e523_d_n8, eq35_e523_d_n9, eq35_e523_d_n10, eq35_e523_d_n11, eq35_e523_d_n12, eq35_e523_d_n13, eq35_e523_d_n14, eq35_e523_d_n15, eq35_e523_d_n16, eq35_e523_d_n17, eq35_e523_d_n18, eq35_e523_d_b0, eq35_e523_d_b1, eq35_e523_d_b2, eq35_e523_d_b3, eq35_e523_d_b4, eq35_e523_d_b5, eq35_e523_d_b6, eq35_e523_d_b7, eq35_e523_d_b8, eq35_e523_d_b9, eq35_e523_d_b10, eq35_e523_d_b11, eq35_e523_d_b12, eq35_e523_d_b13, eq35_e523_d_b14, eq35_e523_d_b15, eq35_e523_q, eq35_e523_q_d_n0, eq35_e523_q_d_n1, eq35_e523_q_d_n2, eq35_e523_q_d_n3, eq35_e523_q_d_n4, eq35_e523_q_d_n5, eq35_e523_q_d_n6, eq35_e523_q_d_n7, eq35_e523_q_d_n8, eq35_e523_q_d_n9, eq35_e523_q_d_n10, eq35_e523_q_d_n11, eq35_e523_q_d_n12, eq35_e523_q_d_n13, eq35_e523_q_d_n14, eq35_e523_q_d_n15, eq35_e523_q_d_n16, eq35_e523_q_d_n17, eq35_e523_q_d_n18, eq35_e523_q_d_b0, eq35_e523_q_d_b1, eq35_e523_q_d_b2, eq35_e523_q_d_b3, eq35_e523_q_d_b4, eq35_e523_q_d_b5, eq35_e523_q_d_b6, eq35_e523_q_d_b7, eq35_e523_q_d_b8, eq35_e523_q_d_b9, eq35_e523_q_d_b10, eq35_e523_q_d_b11, eq35_e523_q_d_b12, eq35_e523_q_d_b13, eq35_e523_q_d_b14, eq35_e523_q_d_b15,) = {
    if s.b[1851] {
        let eq35_e519_q: f64 = s.v[283];
        let eq35_e520: f64 = (s.v[281] + s.v[283]);
        let eq35_e520_d_n0: f64 = (s.dn[281][0] + s.dn[283][0]);
        let eq35_e520_d_n1: f64 = (s.dn[281][1] + s.dn[283][1]);
        let eq35_e520_d_n2: f64 = (s.dn[281][2] + s.dn[283][2]);
        let eq35_e520_d_n3: f64 = (s.dn[281][3] + s.dn[283][3]);
        let eq35_e520_d_n4: f64 = (s.dn[281][4] + s.dn[283][4]);
        let eq35_e520_d_n5: f64 = (s.dn[281][5] + s.dn[283][5]);
        let eq35_e520_d_n6: f64 = (s.dn[281][6] + s.dn[283][6]);
        let eq35_e520_d_n7: f64 = (s.dn[281][7] + s.dn[283][7]);
        let eq35_e520_d_n8: f64 = (s.dn[281][8] + s.dn[283][8]);
        let eq35_e520_d_n9: f64 = (s.dn[281][9] + s.dn[283][9]);
        let eq35_e520_d_n10: f64 = (s.dn[281][10] + s.dn[283][10]);
        let eq35_e520_d_n11: f64 = (s.dn[281][11] + s.dn[283][11]);
        let eq35_e520_d_n12: f64 = (s.dn[281][12] + s.dn[283][12]);
        let eq35_e520_d_n13: f64 = (s.dn[281][13] + s.dn[283][13]);
        let eq35_e520_d_n14: f64 = (s.dn[281][14] + s.dn[283][14]);
        let eq35_e520_d_n15: f64 = (s.dn[281][15] + s.dn[283][15]);
        let eq35_e520_d_n16: f64 = (s.dn[281][16] + s.dn[283][16]);
        let eq35_e520_d_n17: f64 = (s.dn[281][17] + s.dn[283][17]);
        let eq35_e520_d_n18: f64 = (s.dn[281][18] + s.dn[283][18]);
        let eq35_e520_d_b0: f64 = (s.db[281][0] + s.db[283][0]);
        let eq35_e520_d_b1: f64 = (s.db[281][1] + s.db[283][1]);
        let eq35_e520_d_b2: f64 = (s.db[281][2] + s.db[283][2]);
        let eq35_e520_d_b3: f64 = (s.db[281][3] + s.db[283][3]);
        let eq35_e520_d_b4: f64 = (s.db[281][4] + s.db[283][4]);
        let eq35_e520_d_b5: f64 = (s.db[281][5] + s.db[283][5]);
        let eq35_e520_d_b6: f64 = (s.db[281][6] + s.db[283][6]);
        let eq35_e520_d_b7: f64 = (s.db[281][7] + s.db[283][7]);
        let eq35_e520_d_b8: f64 = (s.db[281][8] + s.db[283][8]);
        let eq35_e520_d_b9: f64 = (s.db[281][9] + s.db[283][9]);
        let eq35_e520_d_b10: f64 = (s.db[281][10] + s.db[283][10]);
        let eq35_e520_d_b11: f64 = (s.db[281][11] + s.db[283][11]);
        let eq35_e520_d_b12: f64 = (s.db[281][12] + s.db[283][12]);
        let eq35_e520_d_b13: f64 = (s.db[281][13] + s.db[283][13]);
        let eq35_e520_d_b14: f64 = (s.db[281][14] + s.db[283][14]);
        let eq35_e520_d_b15: f64 = (s.db[281][15] + s.db[283][15]);
        let eq35_e520_q: f64 = eq35_e519_q;
        let eq35_e521: f64 = (p.p50 * eq35_e520);
        let eq35_e521_d_n0: f64 = (p.p50 * eq35_e520_d_n0);
        let eq35_e521_d_n1: f64 = (p.p50 * eq35_e520_d_n1);
        let eq35_e521_d_n2: f64 = (p.p50 * eq35_e520_d_n2);
        let eq35_e521_d_n3: f64 = (p.p50 * eq35_e520_d_n3);
        let eq35_e521_d_n4: f64 = (p.p50 * eq35_e520_d_n4);
        let eq35_e521_d_n5: f64 = (p.p50 * eq35_e520_d_n5);
        let eq35_e521_d_n6: f64 = (p.p50 * eq35_e520_d_n6);
        let eq35_e521_d_n7: f64 = (p.p50 * eq35_e520_d_n7);
        let eq35_e521_d_n8: f64 = (p.p50 * eq35_e520_d_n8);
        let eq35_e521_d_n9: f64 = (p.p50 * eq35_e520_d_n9);
        let eq35_e521_d_n10: f64 = (p.p50 * eq35_e520_d_n10);
        let eq35_e521_d_n11: f64 = (p.p50 * eq35_e520_d_n11);
        let eq35_e521_d_n12: f64 = (p.p50 * eq35_e520_d_n12);
        let eq35_e521_d_n13: f64 = (p.p50 * eq35_e520_d_n13);
        let eq35_e521_d_n14: f64 = (p.p50 * eq35_e520_d_n14);
        let eq35_e521_d_n15: f64 = (p.p50 * eq35_e520_d_n15);
        let eq35_e521_d_n16: f64 = (p.p50 * eq35_e520_d_n16);
        let eq35_e521_d_n17: f64 = (p.p50 * eq35_e520_d_n17);
        let eq35_e521_d_n18: f64 = (p.p50 * eq35_e520_d_n18);
        let eq35_e521_d_b0: f64 = (p.p50 * eq35_e520_d_b0);
        let eq35_e521_d_b1: f64 = (p.p50 * eq35_e520_d_b1);
        let eq35_e521_d_b2: f64 = (p.p50 * eq35_e520_d_b2);
        let eq35_e521_d_b3: f64 = (p.p50 * eq35_e520_d_b3);
        let eq35_e521_d_b4: f64 = (p.p50 * eq35_e520_d_b4);
        let eq35_e521_d_b5: f64 = (p.p50 * eq35_e520_d_b5);
        let eq35_e521_d_b6: f64 = (p.p50 * eq35_e520_d_b6);
        let eq35_e521_d_b7: f64 = (p.p50 * eq35_e520_d_b7);
        let eq35_e521_d_b8: f64 = (p.p50 * eq35_e520_d_b8);
        let eq35_e521_d_b9: f64 = (p.p50 * eq35_e520_d_b9);
        let eq35_e521_d_b10: f64 = (p.p50 * eq35_e520_d_b10);
        let eq35_e521_d_b11: f64 = (p.p50 * eq35_e520_d_b11);
        let eq35_e521_d_b12: f64 = (p.p50 * eq35_e520_d_b12);
        let eq35_e521_d_b13: f64 = (p.p50 * eq35_e520_d_b13);
        let eq35_e521_d_b14: f64 = (p.p50 * eq35_e520_d_b14);
        let eq35_e521_d_b15: f64 = (p.p50 * eq35_e520_d_b15);
        let eq35_e521_q: f64 = (p.p50 * eq35_e520_q);
        let eq35_e521_q_d_n0: f64 = (p.p50 * s.dn[283][0]);
        let eq35_e521_q_d_n1: f64 = (p.p50 * s.dn[283][1]);
        let eq35_e521_q_d_n2: f64 = (p.p50 * s.dn[283][2]);
        let eq35_e521_q_d_n3: f64 = (p.p50 * s.dn[283][3]);
        let eq35_e521_q_d_n4: f64 = (p.p50 * s.dn[283][4]);
        let eq35_e521_q_d_n5: f64 = (p.p50 * s.dn[283][5]);
        let eq35_e521_q_d_n6: f64 = (p.p50 * s.dn[283][6]);
        let eq35_e521_q_d_n7: f64 = (p.p50 * s.dn[283][7]);
        let eq35_e521_q_d_n8: f64 = (p.p50 * s.dn[283][8]);
        let eq35_e521_q_d_n9: f64 = (p.p50 * s.dn[283][9]);
        let eq35_e521_q_d_n10: f64 = (p.p50 * s.dn[283][10]);
        let eq35_e521_q_d_n11: f64 = (p.p50 * s.dn[283][11]);
        let eq35_e521_q_d_n12: f64 = (p.p50 * s.dn[283][12]);
        let eq35_e521_q_d_n13: f64 = (p.p50 * s.dn[283][13]);
        let eq35_e521_q_d_n14: f64 = (p.p50 * s.dn[283][14]);
        let eq35_e521_q_d_n15: f64 = (p.p50 * s.dn[283][15]);
        let eq35_e521_q_d_n16: f64 = (p.p50 * s.dn[283][16]);
        let eq35_e521_q_d_n17: f64 = (p.p50 * s.dn[283][17]);
        let eq35_e521_q_d_n18: f64 = (p.p50 * s.dn[283][18]);
        let eq35_e521_q_d_b0: f64 = (p.p50 * s.db[283][0]);
        let eq35_e521_q_d_b1: f64 = (p.p50 * s.db[283][1]);
        let eq35_e521_q_d_b2: f64 = (p.p50 * s.db[283][2]);
        let eq35_e521_q_d_b3: f64 = (p.p50 * s.db[283][3]);
        let eq35_e521_q_d_b4: f64 = (p.p50 * s.db[283][4]);
        let eq35_e521_q_d_b5: f64 = (p.p50 * s.db[283][5]);
        let eq35_e521_q_d_b6: f64 = (p.p50 * s.db[283][6]);
        let eq35_e521_q_d_b7: f64 = (p.p50 * s.db[283][7]);
        let eq35_e521_q_d_b8: f64 = (p.p50 * s.db[283][8]);
        let eq35_e521_q_d_b9: f64 = (p.p50 * s.db[283][9]);
        let eq35_e521_q_d_b10: f64 = (p.p50 * s.db[283][10]);
        let eq35_e521_q_d_b11: f64 = (p.p50 * s.db[283][11]);
        let eq35_e521_q_d_b12: f64 = (p.p50 * s.db[283][12]);
        let eq35_e521_q_d_b13: f64 = (p.p50 * s.db[283][13]);
        let eq35_e521_q_d_b14: f64 = (p.p50 * s.db[283][14]);
        let eq35_e521_q_d_b15: f64 = (p.p50 * s.db[283][15]);
        (eq35_e521, eq35_e521_d_n0, eq35_e521_d_n1, eq35_e521_d_n2, eq35_e521_d_n3, eq35_e521_d_n4, eq35_e521_d_n5, eq35_e521_d_n6, eq35_e521_d_n7, eq35_e521_d_n8, eq35_e521_d_n9, eq35_e521_d_n10, eq35_e521_d_n11, eq35_e521_d_n12, eq35_e521_d_n13, eq35_e521_d_n14, eq35_e521_d_n15, eq35_e521_d_n16, eq35_e521_d_n17, eq35_e521_d_n18, eq35_e521_d_b0, eq35_e521_d_b1, eq35_e521_d_b2, eq35_e521_d_b3, eq35_e521_d_b4, eq35_e521_d_b5, eq35_e521_d_b6, eq35_e521_d_b7, eq35_e521_d_b8, eq35_e521_d_b9, eq35_e521_d_b10, eq35_e521_d_b11, eq35_e521_d_b12, eq35_e521_d_b13, eq35_e521_d_b14, eq35_e521_d_b15, eq35_e521_q, eq35_e521_q_d_n0, eq35_e521_q_d_n1, eq35_e521_q_d_n2, eq35_e521_q_d_n3, eq35_e521_q_d_n4, eq35_e521_q_d_n5, eq35_e521_q_d_n6, eq35_e521_q_d_n7, eq35_e521_q_d_n8, eq35_e521_q_d_n9, eq35_e521_q_d_n10, eq35_e521_q_d_n11, eq35_e521_q_d_n12, eq35_e521_q_d_n13, eq35_e521_q_d_n14, eq35_e521_q_d_n15, eq35_e521_q_d_n16, eq35_e521_q_d_n17, eq35_e521_q_d_n18, eq35_e521_q_d_b0, eq35_e521_q_d_b1, eq35_e521_q_d_b2, eq35_e521_q_d_b3, eq35_e521_q_d_b4, eq35_e521_q_d_b5, eq35_e521_q_d_b6, eq35_e521_q_d_b7, eq35_e521_q_d_b8, eq35_e521_q_d_b9, eq35_e521_q_d_b10, eq35_e521_q_d_b11, eq35_e521_q_d_b12, eq35_e521_q_d_b13, eq35_e521_q_d_b14, eq35_e521_q_d_b15,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq35_reactive_node_derivatives: [f64; 19] = [eq35_e523_q_d_n0, eq35_e523_q_d_n1, eq35_e523_q_d_n2, eq35_e523_q_d_n3, eq35_e523_q_d_n4, eq35_e523_q_d_n5, eq35_e523_q_d_n6, eq35_e523_q_d_n7, eq35_e523_q_d_n8, eq35_e523_q_d_n9, eq35_e523_q_d_n10, eq35_e523_q_d_n11, eq35_e523_q_d_n12, eq35_e523_q_d_n13, eq35_e523_q_d_n14, eq35_e523_q_d_n15, eq35_e523_q_d_n16, eq35_e523_q_d_n17, eq35_e523_q_d_n18];
        let eq35_reactive_branch_derivatives: [f64; 16] = [eq35_e523_q_d_b0, eq35_e523_q_d_b1, eq35_e523_q_d_b2, eq35_e523_q_d_b3, eq35_e523_q_d_b4, eq35_e523_q_d_b5, eq35_e523_q_d_b6, eq35_e523_q_d_b7, eq35_e523_q_d_b8, eq35_e523_q_d_b9, eq35_e523_q_d_b10, eq35_e523_q_d_b11, eq35_e523_q_d_b12, eq35_e523_q_d_b13, eq35_e523_q_d_b14, eq35_e523_q_d_b15];
        stamper.stamp_current_reactive_dense(
            Some(nodes[9]),
            Some(nodes[7]),
            nodes,
            &eq35_reactive_node_derivatives,
            branches,
            &eq35_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq36_e532, eq36_e532_d_n0, eq36_e532_d_n1, eq36_e532_d_n2, eq36_e532_d_n3, eq36_e532_d_n4, eq36_e532_d_n5, eq36_e532_d_n6, eq36_e532_d_n7, eq36_e532_d_n8, eq36_e532_d_n9, eq36_e532_d_n10, eq36_e532_d_n11, eq36_e532_d_n12, eq36_e532_d_n13, eq36_e532_d_n14, eq36_e532_d_n15, eq36_e532_d_n16, eq36_e532_d_n17, eq36_e532_d_n18, eq36_e532_d_b0, eq36_e532_d_b1, eq36_e532_d_b2, eq36_e532_d_b3, eq36_e532_d_b4, eq36_e532_d_b5, eq36_e532_d_b6, eq36_e532_d_b7, eq36_e532_d_b8, eq36_e532_d_b9, eq36_e532_d_b10, eq36_e532_d_b11, eq36_e532_d_b12, eq36_e532_d_b13, eq36_e532_d_b14, eq36_e532_d_b15, eq36_e532_q, eq36_e532_q_d_n0, eq36_e532_q_d_n1, eq36_e532_q_d_n2, eq36_e532_q_d_n3, eq36_e532_q_d_n4, eq36_e532_q_d_n5, eq36_e532_q_d_n6, eq36_e532_q_d_n7, eq36_e532_q_d_n8, eq36_e532_q_d_n9, eq36_e532_q_d_n10, eq36_e532_q_d_n11, eq36_e532_q_d_n12, eq36_e532_q_d_n13, eq36_e532_q_d_n14, eq36_e532_q_d_n15, eq36_e532_q_d_n16, eq36_e532_q_d_n17, eq36_e532_q_d_n18, eq36_e532_q_d_b0, eq36_e532_q_d_b1, eq36_e532_q_d_b2, eq36_e532_q_d_b3, eq36_e532_q_d_b4, eq36_e532_q_d_b5, eq36_e532_q_d_b6, eq36_e532_q_d_b7, eq36_e532_q_d_b8, eq36_e532_q_d_b9, eq36_e532_q_d_b10, eq36_e532_q_d_b11, eq36_e532_q_d_b12, eq36_e532_q_d_b13, eq36_e532_q_d_b14, eq36_e532_q_d_b15,) = {
    if s.b[1851] {
        let eq36_e528_q: f64 = s.v[284];
        let eq36_e529: f64 = (s.v[282] + s.v[284]);
        let eq36_e529_d_n0: f64 = (s.dn[282][0] + s.dn[284][0]);
        let eq36_e529_d_n1: f64 = (s.dn[282][1] + s.dn[284][1]);
        let eq36_e529_d_n2: f64 = (s.dn[282][2] + s.dn[284][2]);
        let eq36_e529_d_n3: f64 = (s.dn[282][3] + s.dn[284][3]);
        let eq36_e529_d_n4: f64 = (s.dn[282][4] + s.dn[284][4]);
        let eq36_e529_d_n5: f64 = (s.dn[282][5] + s.dn[284][5]);
        let eq36_e529_d_n6: f64 = (s.dn[282][6] + s.dn[284][6]);
        let eq36_e529_d_n7: f64 = (s.dn[282][7] + s.dn[284][7]);
        let eq36_e529_d_n8: f64 = (s.dn[282][8] + s.dn[284][8]);
        let eq36_e529_d_n9: f64 = (s.dn[282][9] + s.dn[284][9]);
        let eq36_e529_d_n10: f64 = (s.dn[282][10] + s.dn[284][10]);
        let eq36_e529_d_n11: f64 = (s.dn[282][11] + s.dn[284][11]);
        let eq36_e529_d_n12: f64 = (s.dn[282][12] + s.dn[284][12]);
        let eq36_e529_d_n13: f64 = (s.dn[282][13] + s.dn[284][13]);
        let eq36_e529_d_n14: f64 = (s.dn[282][14] + s.dn[284][14]);
        let eq36_e529_d_n15: f64 = (s.dn[282][15] + s.dn[284][15]);
        let eq36_e529_d_n16: f64 = (s.dn[282][16] + s.dn[284][16]);
        let eq36_e529_d_n17: f64 = (s.dn[282][17] + s.dn[284][17]);
        let eq36_e529_d_n18: f64 = (s.dn[282][18] + s.dn[284][18]);
        let eq36_e529_d_b0: f64 = (s.db[282][0] + s.db[284][0]);
        let eq36_e529_d_b1: f64 = (s.db[282][1] + s.db[284][1]);
        let eq36_e529_d_b2: f64 = (s.db[282][2] + s.db[284][2]);
        let eq36_e529_d_b3: f64 = (s.db[282][3] + s.db[284][3]);
        let eq36_e529_d_b4: f64 = (s.db[282][4] + s.db[284][4]);
        let eq36_e529_d_b5: f64 = (s.db[282][5] + s.db[284][5]);
        let eq36_e529_d_b6: f64 = (s.db[282][6] + s.db[284][6]);
        let eq36_e529_d_b7: f64 = (s.db[282][7] + s.db[284][7]);
        let eq36_e529_d_b8: f64 = (s.db[282][8] + s.db[284][8]);
        let eq36_e529_d_b9: f64 = (s.db[282][9] + s.db[284][9]);
        let eq36_e529_d_b10: f64 = (s.db[282][10] + s.db[284][10]);
        let eq36_e529_d_b11: f64 = (s.db[282][11] + s.db[284][11]);
        let eq36_e529_d_b12: f64 = (s.db[282][12] + s.db[284][12]);
        let eq36_e529_d_b13: f64 = (s.db[282][13] + s.db[284][13]);
        let eq36_e529_d_b14: f64 = (s.db[282][14] + s.db[284][14]);
        let eq36_e529_d_b15: f64 = (s.db[282][15] + s.db[284][15]);
        let eq36_e529_q: f64 = eq36_e528_q;
        let eq36_e530: f64 = (p.p50 * eq36_e529);
        let eq36_e530_d_n0: f64 = (p.p50 * eq36_e529_d_n0);
        let eq36_e530_d_n1: f64 = (p.p50 * eq36_e529_d_n1);
        let eq36_e530_d_n2: f64 = (p.p50 * eq36_e529_d_n2);
        let eq36_e530_d_n3: f64 = (p.p50 * eq36_e529_d_n3);
        let eq36_e530_d_n4: f64 = (p.p50 * eq36_e529_d_n4);
        let eq36_e530_d_n5: f64 = (p.p50 * eq36_e529_d_n5);
        let eq36_e530_d_n6: f64 = (p.p50 * eq36_e529_d_n6);
        let eq36_e530_d_n7: f64 = (p.p50 * eq36_e529_d_n7);
        let eq36_e530_d_n8: f64 = (p.p50 * eq36_e529_d_n8);
        let eq36_e530_d_n9: f64 = (p.p50 * eq36_e529_d_n9);
        let eq36_e530_d_n10: f64 = (p.p50 * eq36_e529_d_n10);
        let eq36_e530_d_n11: f64 = (p.p50 * eq36_e529_d_n11);
        let eq36_e530_d_n12: f64 = (p.p50 * eq36_e529_d_n12);
        let eq36_e530_d_n13: f64 = (p.p50 * eq36_e529_d_n13);
        let eq36_e530_d_n14: f64 = (p.p50 * eq36_e529_d_n14);
        let eq36_e530_d_n15: f64 = (p.p50 * eq36_e529_d_n15);
        let eq36_e530_d_n16: f64 = (p.p50 * eq36_e529_d_n16);
        let eq36_e530_d_n17: f64 = (p.p50 * eq36_e529_d_n17);
        let eq36_e530_d_n18: f64 = (p.p50 * eq36_e529_d_n18);
        let eq36_e530_d_b0: f64 = (p.p50 * eq36_e529_d_b0);
        let eq36_e530_d_b1: f64 = (p.p50 * eq36_e529_d_b1);
        let eq36_e530_d_b2: f64 = (p.p50 * eq36_e529_d_b2);
        let eq36_e530_d_b3: f64 = (p.p50 * eq36_e529_d_b3);
        let eq36_e530_d_b4: f64 = (p.p50 * eq36_e529_d_b4);
        let eq36_e530_d_b5: f64 = (p.p50 * eq36_e529_d_b5);
        let eq36_e530_d_b6: f64 = (p.p50 * eq36_e529_d_b6);
        let eq36_e530_d_b7: f64 = (p.p50 * eq36_e529_d_b7);
        let eq36_e530_d_b8: f64 = (p.p50 * eq36_e529_d_b8);
        let eq36_e530_d_b9: f64 = (p.p50 * eq36_e529_d_b9);
        let eq36_e530_d_b10: f64 = (p.p50 * eq36_e529_d_b10);
        let eq36_e530_d_b11: f64 = (p.p50 * eq36_e529_d_b11);
        let eq36_e530_d_b12: f64 = (p.p50 * eq36_e529_d_b12);
        let eq36_e530_d_b13: f64 = (p.p50 * eq36_e529_d_b13);
        let eq36_e530_d_b14: f64 = (p.p50 * eq36_e529_d_b14);
        let eq36_e530_d_b15: f64 = (p.p50 * eq36_e529_d_b15);
        let eq36_e530_q: f64 = (p.p50 * eq36_e529_q);
        let eq36_e530_q_d_n0: f64 = (p.p50 * s.dn[284][0]);
        let eq36_e530_q_d_n1: f64 = (p.p50 * s.dn[284][1]);
        let eq36_e530_q_d_n2: f64 = (p.p50 * s.dn[284][2]);
        let eq36_e530_q_d_n3: f64 = (p.p50 * s.dn[284][3]);
        let eq36_e530_q_d_n4: f64 = (p.p50 * s.dn[284][4]);
        let eq36_e530_q_d_n5: f64 = (p.p50 * s.dn[284][5]);
        let eq36_e530_q_d_n6: f64 = (p.p50 * s.dn[284][6]);
        let eq36_e530_q_d_n7: f64 = (p.p50 * s.dn[284][7]);
        let eq36_e530_q_d_n8: f64 = (p.p50 * s.dn[284][8]);
        let eq36_e530_q_d_n9: f64 = (p.p50 * s.dn[284][9]);
        let eq36_e530_q_d_n10: f64 = (p.p50 * s.dn[284][10]);
        let eq36_e530_q_d_n11: f64 = (p.p50 * s.dn[284][11]);
        let eq36_e530_q_d_n12: f64 = (p.p50 * s.dn[284][12]);
        let eq36_e530_q_d_n13: f64 = (p.p50 * s.dn[284][13]);
        let eq36_e530_q_d_n14: f64 = (p.p50 * s.dn[284][14]);
        let eq36_e530_q_d_n15: f64 = (p.p50 * s.dn[284][15]);
        let eq36_e530_q_d_n16: f64 = (p.p50 * s.dn[284][16]);
        let eq36_e530_q_d_n17: f64 = (p.p50 * s.dn[284][17]);
        let eq36_e530_q_d_n18: f64 = (p.p50 * s.dn[284][18]);
        let eq36_e530_q_d_b0: f64 = (p.p50 * s.db[284][0]);
        let eq36_e530_q_d_b1: f64 = (p.p50 * s.db[284][1]);
        let eq36_e530_q_d_b2: f64 = (p.p50 * s.db[284][2]);
        let eq36_e530_q_d_b3: f64 = (p.p50 * s.db[284][3]);
        let eq36_e530_q_d_b4: f64 = (p.p50 * s.db[284][4]);
        let eq36_e530_q_d_b5: f64 = (p.p50 * s.db[284][5]);
        let eq36_e530_q_d_b6: f64 = (p.p50 * s.db[284][6]);
        let eq36_e530_q_d_b7: f64 = (p.p50 * s.db[284][7]);
        let eq36_e530_q_d_b8: f64 = (p.p50 * s.db[284][8]);
        let eq36_e530_q_d_b9: f64 = (p.p50 * s.db[284][9]);
        let eq36_e530_q_d_b10: f64 = (p.p50 * s.db[284][10]);
        let eq36_e530_q_d_b11: f64 = (p.p50 * s.db[284][11]);
        let eq36_e530_q_d_b12: f64 = (p.p50 * s.db[284][12]);
        let eq36_e530_q_d_b13: f64 = (p.p50 * s.db[284][13]);
        let eq36_e530_q_d_b14: f64 = (p.p50 * s.db[284][14]);
        let eq36_e530_q_d_b15: f64 = (p.p50 * s.db[284][15]);
        (eq36_e530, eq36_e530_d_n0, eq36_e530_d_n1, eq36_e530_d_n2, eq36_e530_d_n3, eq36_e530_d_n4, eq36_e530_d_n5, eq36_e530_d_n6, eq36_e530_d_n7, eq36_e530_d_n8, eq36_e530_d_n9, eq36_e530_d_n10, eq36_e530_d_n11, eq36_e530_d_n12, eq36_e530_d_n13, eq36_e530_d_n14, eq36_e530_d_n15, eq36_e530_d_n16, eq36_e530_d_n17, eq36_e530_d_n18, eq36_e530_d_b0, eq36_e530_d_b1, eq36_e530_d_b2, eq36_e530_d_b3, eq36_e530_d_b4, eq36_e530_d_b5, eq36_e530_d_b6, eq36_e530_d_b7, eq36_e530_d_b8, eq36_e530_d_b9, eq36_e530_d_b10, eq36_e530_d_b11, eq36_e530_d_b12, eq36_e530_d_b13, eq36_e530_d_b14, eq36_e530_d_b15, eq36_e530_q, eq36_e530_q_d_n0, eq36_e530_q_d_n1, eq36_e530_q_d_n2, eq36_e530_q_d_n3, eq36_e530_q_d_n4, eq36_e530_q_d_n5, eq36_e530_q_d_n6, eq36_e530_q_d_n7, eq36_e530_q_d_n8, eq36_e530_q_d_n9, eq36_e530_q_d_n10, eq36_e530_q_d_n11, eq36_e530_q_d_n12, eq36_e530_q_d_n13, eq36_e530_q_d_n14, eq36_e530_q_d_n15, eq36_e530_q_d_n16, eq36_e530_q_d_n17, eq36_e530_q_d_n18, eq36_e530_q_d_b0, eq36_e530_q_d_b1, eq36_e530_q_d_b2, eq36_e530_q_d_b3, eq36_e530_q_d_b4, eq36_e530_q_d_b5, eq36_e530_q_d_b6, eq36_e530_q_d_b7, eq36_e530_q_d_b8, eq36_e530_q_d_b9, eq36_e530_q_d_b10, eq36_e530_q_d_b11, eq36_e530_q_d_b12, eq36_e530_q_d_b13, eq36_e530_q_d_b14, eq36_e530_q_d_b15,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq36_reactive_node_derivatives: [f64; 19] = [eq36_e532_q_d_n0, eq36_e532_q_d_n1, eq36_e532_q_d_n2, eq36_e532_q_d_n3, eq36_e532_q_d_n4, eq36_e532_q_d_n5, eq36_e532_q_d_n6, eq36_e532_q_d_n7, eq36_e532_q_d_n8, eq36_e532_q_d_n9, eq36_e532_q_d_n10, eq36_e532_q_d_n11, eq36_e532_q_d_n12, eq36_e532_q_d_n13, eq36_e532_q_d_n14, eq36_e532_q_d_n15, eq36_e532_q_d_n16, eq36_e532_q_d_n17, eq36_e532_q_d_n18];
        let eq36_reactive_branch_derivatives: [f64; 16] = [eq36_e532_q_d_b0, eq36_e532_q_d_b1, eq36_e532_q_d_b2, eq36_e532_q_d_b3, eq36_e532_q_d_b4, eq36_e532_q_d_b5, eq36_e532_q_d_b6, eq36_e532_q_d_b7, eq36_e532_q_d_b8, eq36_e532_q_d_b9, eq36_e532_q_d_b10, eq36_e532_q_d_b11, eq36_e532_q_d_b12, eq36_e532_q_d_b13, eq36_e532_q_d_b14, eq36_e532_q_d_b15];
        stamper.stamp_current_reactive_dense(
            Some(nodes[8]),
            Some(nodes[6]),
            nodes,
            &eq36_reactive_node_derivatives,
            branches,
            &eq36_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq47_e616, eq47_e616_d_n18, eq47_e616_q, eq47_e616_q_d_n18,) = {
    if (s.b[1851] && (p.p34 != 0.0)) {
        let eq47_e611: f64 = (1e-9 / 0.0001);
        let eq47_e613: f64 = (eq47_e611 * (nv18 - 0.0));
        let eq47_e614_q: f64 = eq47_e613;
        (eq47_e613, eq47_e611, eq47_e614_q, eq47_e611,)
    } else {
        (0.0, 0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node1(
            Some(nodes[18]),
            None,
            nodes[18],
            multiplicity * (eq47_e616_q_d_n18),
        );
        let (eq48_e627, eq48_e627_d_n13, eq48_e627_q, eq48_e627_q_d_n13,) = {
    if (s.b[1851] && (p.p34 != 0.0)) {
        let eq48_e622: f64 = (1e-9 / 0.0001);
        let eq48_e624: f64 = (eq48_e622 * (nv13 - 0.0));
        let eq48_e625_q: f64 = eq48_e624;
        (eq48_e624, eq48_e622, eq48_e625_q, eq48_e622,)
    } else {
        (0.0, 0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node1(
            Some(nodes[13]),
            None,
            nodes[13],
            multiplicity * (eq48_e627_q_d_n13),
        );
        let (eq53_e666, eq53_e666_d_n17, eq53_e666_q, eq53_e666_q_d_n17,) = {
    if (s.b[1851] && s.b[1852]) {
        let eq53_e661: f64 = (1e-9 / 0.0001);
        let eq53_e663: f64 = (eq53_e661 * (nv17 - 0.0));
        let eq53_e664_q: f64 = eq53_e663;
        (eq53_e663, eq53_e661, eq53_e664_q, eq53_e661,)
    } else {
        (0.0, 0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node1(
            Some(nodes[17]),
            None,
            nodes[17],
            multiplicity * (eq53_e666_q_d_n17),
        );
        let (eq60_e724, eq60_e724_d_n17, eq60_e724_q, eq60_e724_q_d_n17,) = {
    if ((!s.b[1851]) && (p.p37 != 0.0)) {
        let eq60_e719: f64 = (1e-9 / 0.0001);
        let eq60_e721: f64 = (eq60_e719 * (nv17 - 0.0));
        let eq60_e722_q: f64 = eq60_e721;
        (eq60_e721, eq60_e719, eq60_e722_q, eq60_e719,)
    } else {
        (0.0, 0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node1(
            Some(nodes[17]),
            None,
            nodes[17],
            multiplicity * (eq60_e724_q_d_n17),
        );
        let (eq68_e792, eq68_e792_d_n15, eq68_e792_q, eq68_e792_q_d_n15,) = {
    if ((!s.b[1851]) && (p.p34 != 0.0)) {
        let eq68_e787: f64 = (1e-9 / 0.0001);
        let eq68_e789: f64 = (eq68_e787 * (nv15 - 0.0));
        let eq68_e790_q: f64 = eq68_e789;
        (eq68_e789, eq68_e787, eq68_e790_q, eq68_e787,)
    } else {
        (0.0, 0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node1(
            Some(nodes[15]),
            None,
            nodes[15],
            multiplicity * (eq68_e792_q_d_n15),
        );
        let (eq69_e804, eq69_e804_d_n16, eq69_e804_q, eq69_e804_q_d_n16,) = {
    if ((!s.b[1851]) && (p.p34 != 0.0)) {
        let eq69_e799: f64 = (1e-9 / 0.0001);
        let eq69_e801: f64 = (eq69_e799 * (nv16 - 0.0));
        let eq69_e802_q: f64 = eq69_e801;
        (eq69_e801, eq69_e799, eq69_e802_q, eq69_e799,)
    } else {
        (0.0, 0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node1(
            Some(nodes[16]),
            None,
            nodes[16],
            multiplicity * (eq69_e804_q_d_n16),
        );
        let (eq70_e816, eq70_e816_d_n13, eq70_e816_q, eq70_e816_q_d_n13,) = {
    if ((!s.b[1851]) && (p.p34 != 0.0)) {
        let eq70_e811: f64 = (1e-9 / 0.0001);
        let eq70_e813: f64 = (eq70_e811 * (nv13 - 0.0));
        let eq70_e814_q: f64 = eq70_e813;
        (eq70_e813, eq70_e811, eq70_e814_q, eq70_e811,)
    } else {
        (0.0, 0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node1(
            Some(nodes[13]),
            None,
            nodes[13],
            multiplicity * (eq70_e816_q_d_n13),
        );
    }
}
