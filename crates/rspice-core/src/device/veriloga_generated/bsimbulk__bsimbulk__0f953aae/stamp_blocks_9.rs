#![allow(dead_code, non_snake_case, unused_assignments, unused_imports, unused_parens, unused_variables)]
use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, ReactiveScratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};
impl Instance {
    #[inline(never)]
    pub(super) fn stamp_reactive_block_57(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.store_scale(318, 428, (s.v[35] * p.p2));s.store_scalar(313, ((0.1) as f64).powf((-p.p713)));s.b[1496] = (p.p713 == 1.0);s.store_scalar(1496, if s.b[1496] { 1.0 } else { 0.0 });
        if s.b[1496] {s.store_scalar(314, (1.5 - ((0.1) as f64).ln()));}
        if (!s.b[1496]) {s.store_scalar(314, ((1.0 / (1.0 - p.p713)) * (1.0 - (((0.05 * p.p713) * (1.0 + p.p713)) * s.v[313]))));}
        s.store_scalar(316, ((0.1) as f64).powf((-p.p715)));s.b[1497] = (p.p715 == 1.0);s.store_scalar(1497, if s.b[1497] { 1.0 } else { 0.0 });
        if s.b[1497] {s.store_scalar(317, (1.5 - ((0.1) as f64).ln()));}
        if (!s.b[1497]) {s.store_scalar(317, ((1.0 / (1.0 - p.p715)) * (1.0 - (((0.05 * p.p715) * (1.0 + p.p715)) * s.v[316]))));}
        s.store_scalar(319, ((0.1) as f64).powf((-p.p717)));s.b[1498] = (p.p717 == 1.0);s.store_scalar(1498, if s.b[1498] { 1.0 } else { 0.0 });
        if s.b[1498] {s.store_scalar(320, (1.5 - ((0.1) as f64).ln()));}
        if (!s.b[1498]) {s.store_scalar(320, ((1.0 / (1.0 - p.p717)) * (1.0 - (((0.05 * p.p717) * (1.0 + p.p717)) * s.v[319]))));}
        s.b[1499] = (s.v[312] > 0.0);s.store_scalar(1499, if s.b[1499] { 1.0 } else { 0.0 });
        if s.b[1499] {s.store_div(13, 306, 429);}
        s.b[1500] = (s.v[13] < 0.9);s.store_scalar(1500, if s.b[1500] { 1.0 } else { 0.0 });
        if (s.b[1499] && s.b[1500]) {s.store_sub_from_scalar(310, 1.0, 13);}
        s.b[1501] = (p.p713 != 1.0);s.store_scalar(1501, if s.b[1501] { 1.0 } else { 0.0 });s.b[1502] = (p.p713 == 0.5);s.store_scalar(1502, if s.b[1502] { 1.0 } else { 0.0 });
        if (((s.b[1499] && s.b[1500]) && s.b[1501]) && s.b[1502]) {s.store_div_from_scalar_sqrt_ad(311, 1.0, s.ad_value(310));}
        if (((s.b[1499] && s.b[1500]) && s.b[1501]) && (!s.b[1502])) {s.store_limited_exp_scaled_input_ad(311, A::ln(s.ad_value(310)), (-p.p713));}
        if ((s.b[1499] && s.b[1500]) && s.b[1501]) {s.store_mul_ad_affine_product_rhs(331, 429, s.ad_value(312), A::sub_from_scalar(1.0, A::mul(s.ad_value(310), s.ad_value(311))), 1.0 / ((1.0 - p.p713)), 0.0);}
        if ((s.b[1499] && s.b[1500]) && (!s.b[1501])) {s.store_mul_ad_affine_product_rhs(331, 429, s.ad_value(312), A::ln(s.ad_value(310)), -1.0, 0.0);}
        if (s.b[1499] && (!s.b[1500])) {s.store_mul_scale_offset_mixed_ai(14, A::offset(s.ad_value(13), (-1.0)), 13, ((5.0 * p.p713)) * (s.v[313]), ((((((-1.0)) * ((5.0 * p.p713)))) + ((1.0 + p.p713)))) * (s.v[313]));s.store_mul_ad_product_rhs_mixed_ia(331, 429, 312, A::add(s.ad_value(14), s.ad_value(314)));}
        if (!s.b[1499]) {s.store_scalar(331, 0.0);}
        s.b[1503] = (s.v[315] > 0.0);s.store_scalar(1503, if s.b[1503] { 1.0 } else { 0.0 });
        if s.b[1503] {s.store_div(13, 306, 430);}
        s.b[1504] = (s.v[13] < 0.9);s.store_scalar(1504, if s.b[1504] { 1.0 } else { 0.0 });
        if (s.b[1503] && s.b[1504]) {s.store_sub_from_scalar(310, 1.0, 13);}
        s.b[1505] = (p.p715 != 1.0);s.store_scalar(1505, if s.b[1505] { 1.0 } else { 0.0 });s.b[1506] = (p.p715 == 0.5);s.store_scalar(1506, if s.b[1506] { 1.0 } else { 0.0 });
        if (((s.b[1503] && s.b[1504]) && s.b[1505]) && s.b[1506]) {s.store_div_from_scalar_sqrt_ad(311, 1.0, s.ad_value(310));}
        if (((s.b[1503] && s.b[1504]) && s.b[1505]) && (!s.b[1506])) {s.store_limited_exp_scaled_input_ad(311, A::ln(s.ad_value(310)), (-p.p715));}
        if ((s.b[1503] && s.b[1504]) && s.b[1505]) {s.store_mul_ad_affine_product_rhs(332, 430, s.ad_value(315), A::sub_from_scalar(1.0, A::mul(s.ad_value(310), s.ad_value(311))), 1.0 / ((1.0 - p.p715)), 0.0);}
        if ((s.b[1503] && s.b[1504]) && (!s.b[1505])) {s.store_mul_ad_affine_product_rhs(332, 430, s.ad_value(315), A::ln(s.ad_value(310)), -1.0, 0.0);}
        if (s.b[1503] && (!s.b[1504])) {s.store_mul_scale_offset_mixed_ai(14, A::offset(s.ad_value(13), (-1.0)), 13, ((5.0 * p.p715)) * (s.v[316]), ((((((-1.0)) * ((5.0 * p.p715)))) + ((1.0 + p.p715)))) * (s.v[316]));s.store_mul_ad_product_rhs_mixed_ia(332, 430, 315, A::add(s.ad_value(14), s.ad_value(317)));}
        if (!s.b[1503]) {s.store_scalar(332, 0.0);}
        s.b[1507] = (s.v[318] > 0.0);s.store_scalar(1507, if s.b[1507] { 1.0 } else { 0.0 });
        if s.b[1507] {s.store_div(13, 306, 431);}
        s.b[1508] = (s.v[13] < 0.9);s.store_scalar(1508, if s.b[1508] { 1.0 } else { 0.0 });
        if (s.b[1507] && s.b[1508]) {s.store_sub_from_scalar(310, 1.0, 13);}
        s.b[1509] = (p.p717 != 1.0);s.store_scalar(1509, if s.b[1509] { 1.0 } else { 0.0 });s.b[1510] = (p.p717 == 0.5);s.store_scalar(1510, if s.b[1510] { 1.0 } else { 0.0 });
        if (((s.b[1507] && s.b[1508]) && s.b[1509]) && s.b[1510]) {s.store_div_from_scalar_sqrt_ad(311, 1.0, s.ad_value(310));}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_58(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (((s.b[1507] && s.b[1508]) && s.b[1509]) && (!s.b[1510])) {s.store_limited_exp_scaled_input_ad(311, A::ln(s.ad_value(310)), (-p.p717));}
        if ((s.b[1507] && s.b[1508]) && s.b[1509]) {s.store_mul_ad_affine_product_rhs(333, 431, s.ad_value(318), A::sub_from_scalar(1.0, A::mul(s.ad_value(310), s.ad_value(311))), 1.0 / ((1.0 - p.p717)), 0.0);}
        if ((s.b[1507] && s.b[1508]) && (!s.b[1509])) {s.store_mul_ad_affine_product_rhs(333, 431, s.ad_value(318), A::ln(s.ad_value(310)), -1.0, 0.0);}
        if (s.b[1507] && (!s.b[1508])) {s.store_mul_scale_offset_mixed_ai(14, A::offset(s.ad_value(13), (-1.0)), 13, ((5.0 * p.p717)) * (s.v[319]), ((((((-1.0)) * ((5.0 * p.p717)))) + ((1.0 + p.p717)))) * (s.v[319]));s.store_mul_ad_product_rhs_mixed_ia(333, 431, 318, A::add(s.ad_value(14), s.ad_value(320)));}
        if (!s.b[1507]) {s.store_scalar(333, 0.0);}
        s.store_add_scaled_inputs3_indices(330, 331, 1.0, 332, 1.0, 333, 1.0);s.store_mul3_lhs(321, 302, 426, 251);s.b[1511] = (s.v[301] > (s.v[35] * p.p2));s.store_scalar(1511, if s.b[1511] { 1.0 } else { 0.0 });s.b[1512] = ((p.p1128 > 0.0) && (p.p1097 == 1.0));s.store_scalar(1512, if s.b[1512] { 1.0 } else { 0.0 });
        if (s.b[1511] && s.b[1512]) {s.store_mul_ad_product_rhs_mixed_ia(324, 302, 427, A::offset(s.ad_value(301), (-(s.v[35] * p.p2))));}
        if (s.b[1511] && (!s.b[1512])) {s.store_mul3_lhs(324, 302, 427, 301);}
        if (!s.b[1511]) {s.store_mul3_lhs(324, 302, 427, 301);}
        s.store_scale(327, 425, (s.v[35] * p.p2));s.store_scalar(322, ((0.1) as f64).powf((-p.p714)));s.b[1513] = (p.p714 == 1.0);s.store_scalar(1513, if s.b[1513] { 1.0 } else { 0.0 });
        if s.b[1513] {s.store_scalar(323, (1.5 - ((0.1) as f64).ln()));}
        if (!s.b[1513]) {s.store_scalar(323, ((1.0 / (1.0 - p.p714)) * (1.0 - (((0.05 * p.p714) * (1.0 + p.p714)) * s.v[322]))));}
        s.store_scalar(325, ((0.1) as f64).powf((-p.p716)));s.b[1514] = (p.p716 == 1.0);s.store_scalar(1514, if s.b[1514] { 1.0 } else { 0.0 });
        if s.b[1514] {s.store_scalar(326, (1.5 - ((0.1) as f64).ln()));}
        if (!s.b[1514]) {s.store_scalar(326, ((1.0 / (1.0 - p.p716)) * (1.0 - (((0.05 * p.p716) * (1.0 + p.p716)) * s.v[325]))));}
        s.store_scalar(328, ((0.1) as f64).powf((-p.p718)));s.b[1515] = (p.p718 == 1.0);s.store_scalar(1515, if s.b[1515] { 1.0 } else { 0.0 });
        if s.b[1515] {s.store_scalar(329, (1.5 - ((0.1) as f64).ln()));}
        if (!s.b[1515]) {s.store_scalar(329, ((1.0 / (1.0 - p.p718)) * (1.0 - (((0.05 * p.p718) * (1.0 + p.p718)) * s.v[328]))));}
        s.b[1516] = (s.v[321] > 0.0);s.store_scalar(1516, if s.b[1516] { 1.0 } else { 0.0 });
        if s.b[1516] {s.store_div(13, 308, 432);}
        s.b[1517] = (s.v[13] < 0.9);s.store_scalar(1517, if s.b[1517] { 1.0 } else { 0.0 });
        if (s.b[1516] && s.b[1517]) {s.store_sub_from_scalar(310, 1.0, 13);}
        s.b[1518] = (p.p714 != 1.0);s.store_scalar(1518, if s.b[1518] { 1.0 } else { 0.0 });s.b[1519] = (p.p714 == 0.5);s.store_scalar(1519, if s.b[1519] { 1.0 } else { 0.0 });
        if (((s.b[1516] && s.b[1517]) && s.b[1518]) && s.b[1519]) {s.store_div_from_scalar_sqrt_ad(311, 1.0, s.ad_value(310));}
        if (((s.b[1516] && s.b[1517]) && s.b[1518]) && (!s.b[1519])) {s.store_limited_exp_scaled_input_ad(311, A::ln(s.ad_value(310)), (-p.p714));}
        if ((s.b[1516] && s.b[1517]) && s.b[1518]) {s.store_mul_ad_affine_product_rhs(335, 432, s.ad_value(321), A::sub_from_scalar(1.0, A::mul(s.ad_value(310), s.ad_value(311))), 1.0 / ((1.0 - p.p714)), 0.0);}
        if ((s.b[1516] && s.b[1517]) && (!s.b[1518])) {s.store_mul_ad_affine_product_rhs(335, 432, s.ad_value(321), A::ln(s.ad_value(310)), -1.0, 0.0);}
        if (s.b[1516] && (!s.b[1517])) {s.store_mul_scale_offset_mixed_ai(14, A::offset(s.ad_value(13), (-1.0)), 13, ((5.0 * p.p714)) * (s.v[322]), ((((((-1.0)) * ((5.0 * p.p714)))) + ((1.0 + p.p714)))) * (s.v[322]));s.store_mul_ad_product_rhs_mixed_ia(335, 432, 321, A::add(s.ad_value(14), s.ad_value(323)));}
        if (!s.b[1516]) {s.store_scalar(335, 0.0);}
        s.b[1520] = (s.v[324] > 0.0);s.store_scalar(1520, if s.b[1520] { 1.0 } else { 0.0 });
        if s.b[1520] {s.store_div(13, 308, 433);}
        s.b[1521] = (s.v[13] < 0.9);s.store_scalar(1521, if s.b[1521] { 1.0 } else { 0.0 });
        if (s.b[1520] && s.b[1521]) {s.store_sub_from_scalar(310, 1.0, 13);}
        s.b[1522] = (p.p716 != 1.0);s.store_scalar(1522, if s.b[1522] { 1.0 } else { 0.0 });s.b[1523] = (p.p716 == 0.5);s.store_scalar(1523, if s.b[1523] { 1.0 } else { 0.0 });
        if (((s.b[1520] && s.b[1521]) && s.b[1522]) && s.b[1523]) {s.store_div_from_scalar_sqrt_ad(311, 1.0, s.ad_value(310));}
        if (((s.b[1520] && s.b[1521]) && s.b[1522]) && (!s.b[1523])) {s.store_limited_exp_scaled_input_ad(311, A::ln(s.ad_value(310)), (-p.p716));}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_59(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((s.b[1520] && s.b[1521]) && s.b[1522]) {s.store_mul_ad_affine_product_rhs(336, 433, s.ad_value(324), A::sub_from_scalar(1.0, A::mul(s.ad_value(310), s.ad_value(311))), 1.0 / ((1.0 - p.p716)), 0.0);}
        if ((s.b[1520] && s.b[1521]) && (!s.b[1522])) {s.store_mul_ad_affine_product_rhs(336, 433, s.ad_value(324), A::ln(s.ad_value(310)), -1.0, 0.0);}
        if (s.b[1520] && (!s.b[1521])) {s.store_mul_scale_offset_mixed_ai(14, A::offset(s.ad_value(13), (-1.0)), 13, ((5.0 * p.p716)) * (s.v[325]), ((((((-1.0)) * ((5.0 * p.p716)))) + ((1.0 + p.p716)))) * (s.v[325]));s.store_mul_ad_product_rhs_mixed_ia(336, 433, 324, A::add(s.ad_value(14), s.ad_value(326)));}
        if (!s.b[1520]) {s.store_scalar(336, 0.0);}
        s.b[1524] = (s.v[327] > 0.0);s.store_scalar(1524, if s.b[1524] { 1.0 } else { 0.0 });
        if s.b[1524] {s.store_div(13, 308, 434);}
        s.b[1525] = (s.v[13] < 0.9);s.store_scalar(1525, if s.b[1525] { 1.0 } else { 0.0 });
        if (s.b[1524] && s.b[1525]) {s.store_sub_from_scalar(310, 1.0, 13);}
        s.b[1526] = (p.p718 != 1.0);s.store_scalar(1526, if s.b[1526] { 1.0 } else { 0.0 });s.b[1527] = (p.p718 == 0.5);s.store_scalar(1527, if s.b[1527] { 1.0 } else { 0.0 });
        if (((s.b[1524] && s.b[1525]) && s.b[1526]) && s.b[1527]) {s.store_div_from_scalar_sqrt_ad(311, 1.0, s.ad_value(310));}
        if (((s.b[1524] && s.b[1525]) && s.b[1526]) && (!s.b[1527])) {s.store_limited_exp_scaled_input_ad(311, A::ln(s.ad_value(310)), (-p.p718));}
        if ((s.b[1524] && s.b[1525]) && s.b[1526]) {s.store_mul_ad_affine_product_rhs(337, 434, s.ad_value(327), A::sub_from_scalar(1.0, A::mul(s.ad_value(310), s.ad_value(311))), 1.0 / ((1.0 - p.p718)), 0.0);}
        if ((s.b[1524] && s.b[1525]) && (!s.b[1526])) {s.store_mul_ad_affine_product_rhs(337, 434, s.ad_value(327), A::ln(s.ad_value(310)), -1.0, 0.0);}
        if (s.b[1524] && (!s.b[1525])) {s.store_mul_scale_offset_mixed_ai(14, A::offset(s.ad_value(13), (-1.0)), 13, ((5.0 * p.p718)) * (s.v[328]), ((((((-1.0)) * ((5.0 * p.p718)))) + ((1.0 + p.p718)))) * (s.v[328]));s.store_mul_ad_product_rhs_mixed_ia(337, 434, 327, A::add(s.ad_value(14), s.ad_value(329)));}
        if (!s.b[1524]) {s.store_scalar(337, 0.0);}
        s.store_add_scaled_inputs3_indices(334, 335, 1.0, 336, 1.0, 337, 1.0);s.b[1528] = ((p.p1128 > 0.0) && (p.p1097 == 1.0));s.store_scalar(1528, if s.b[1528] { 1.0 } else { 0.0 });
        if s.b[1528] {s.store_scaled_mul(321, 426, 251, p.p1128);}
        s.b[1529] = (s.v[301] > (s.v[35] * p.p2));s.store_scalar(1529, if s.b[1529] { 1.0 } else { 0.0 });
        if (s.b[1528] && s.b[1529]) {s.store_mul_scale_offset_rhs(324, 427, 301, p.p1128, (((((-(s.v[35] * p.p2))) * (p.p1128))) + ((s.v[35] * p.p2))));}
        if (s.b[1528] && (!s.b[1529])) {s.store_scaled_mul(324, 427, 301, p.p1128);}
        s.b[1530] = (s.v[321] > 0.0);s.store_scalar(1530, if s.b[1530] { 1.0 } else { 0.0 });
        if (s.b[1528] && s.b[1530]) {s.store_div(13, 309, 432);}
        s.b[1531] = (s.v[13] < 0.9);s.store_scalar(1531, if s.b[1531] { 1.0 } else { 0.0 });
        if ((s.b[1528] && s.b[1530]) && s.b[1531]) {s.store_sub_from_scalar(310, 1.0, 13);}
        s.b[1532] = (p.p714 != 1.0);s.store_scalar(1532, if s.b[1532] { 1.0 } else { 0.0 });s.b[1533] = (p.p714 == 0.5);s.store_scalar(1533, if s.b[1533] { 1.0 } else { 0.0 });
        if ((((s.b[1528] && s.b[1530]) && s.b[1531]) && s.b[1532]) && s.b[1533]) {s.store_div_from_scalar_sqrt_ad(311, 1.0, s.ad_value(310));}
        if ((((s.b[1528] && s.b[1530]) && s.b[1531]) && s.b[1532]) && (!s.b[1533])) {s.store_limited_exp_scaled_input_ad(311, A::ln(s.ad_value(310)), (-p.p714));}
        if (((s.b[1528] && s.b[1530]) && s.b[1531]) && s.b[1532]) {s.store_mul_ad_affine_product_rhs(339, 432, s.ad_value(321), A::sub_from_scalar(1.0, A::mul(s.ad_value(310), s.ad_value(311))), 1.0 / ((1.0 - p.p714)), 0.0);}
        if (((s.b[1528] && s.b[1530]) && s.b[1531]) && (!s.b[1532])) {s.store_mul_ad_affine_product_rhs(339, 432, s.ad_value(321), A::ln(s.ad_value(310)), -1.0, 0.0);}
        if ((s.b[1528] && s.b[1530]) && (!s.b[1531])) {s.store_mul_scale_offset_mixed_ai(14, A::offset(s.ad_value(13), (-1.0)), 13, ((5.0 * p.p714)) * (s.v[322]), ((((((-1.0)) * ((5.0 * p.p714)))) + ((1.0 + p.p714)))) * (s.v[322]));s.store_mul_ad_product_rhs_mixed_ia(339, 432, 321, A::add(s.ad_value(14), s.ad_value(323)));}
        if (s.b[1528] && (!s.b[1530])) {s.store_scalar(339, 0.0);}
        s.b[1534] = (s.v[324] > 0.0);s.store_scalar(1534, if s.b[1534] { 1.0 } else { 0.0 });
        if (s.b[1528] && s.b[1534]) {s.store_div(13, 309, 433);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_60(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        s.b[1535] = (s.v[13] < 0.9);s.store_scalar(1535, if s.b[1535] { 1.0 } else { 0.0 });
        if ((s.b[1528] && s.b[1534]) && s.b[1535]) {s.store_sub_from_scalar(310, 1.0, 13);}
        s.b[1536] = (p.p716 != 1.0);s.store_scalar(1536, if s.b[1536] { 1.0 } else { 0.0 });s.b[1537] = (p.p716 == 0.5);s.store_scalar(1537, if s.b[1537] { 1.0 } else { 0.0 });
        if ((((s.b[1528] && s.b[1534]) && s.b[1535]) && s.b[1536]) && s.b[1537]) {s.store_div_from_scalar_sqrt_ad(311, 1.0, s.ad_value(310));}
        if ((((s.b[1528] && s.b[1534]) && s.b[1535]) && s.b[1536]) && (!s.b[1537])) {s.store_limited_exp_scaled_input_ad(311, A::ln(s.ad_value(310)), (-p.p716));}
        if (((s.b[1528] && s.b[1534]) && s.b[1535]) && s.b[1536]) {s.store_mul_ad_affine_product_rhs(340, 433, s.ad_value(324), A::sub_from_scalar(1.0, A::mul(s.ad_value(310), s.ad_value(311))), 1.0 / ((1.0 - p.p716)), 0.0);}
        if (((s.b[1528] && s.b[1534]) && s.b[1535]) && (!s.b[1536])) {s.store_mul_ad_affine_product_rhs(340, 433, s.ad_value(324), A::ln(s.ad_value(310)), -1.0, 0.0);}
        if ((s.b[1528] && s.b[1534]) && (!s.b[1535])) {s.store_mul_scale_offset_mixed_ai(14, A::offset(s.ad_value(13), (-1.0)), 13, ((5.0 * p.p716)) * (s.v[325]), ((((((-1.0)) * ((5.0 * p.p716)))) + ((1.0 + p.p716)))) * (s.v[325]));s.store_mul_ad_product_rhs_mixed_ia(340, 433, 324, A::add(s.ad_value(14), s.ad_value(326)));}
        if (s.b[1528] && (!s.b[1534])) {s.store_scalar(340, 0.0);}
        if s.b[1528] {s.store_add(338, 339, 340);}
        if (!s.b[1528]) {s.store_scalar(338, 0.0);}
        s.b[1538] = (p.p38 != 0.0);s.store_scalar(1538, if s.b[1538] { 1.0 } else { 0.0 });
        if s.b[1538] {s.store_powf_scaled_input(13, 481, 1.0000000000000001e-23, p.p954);s.store_powf_ad(14, A::div_from_scalar(300.0, s.ad_value(391)), p.p955);s.store_div_scaled_product_mixed_iai(15, 187, A::voltage(ctx, nodes, Some(11), Some(7)), p.p953, 108, 1.0);}
        s.store_div_scaled_inputs_indices(360, 502, 2.0, 157, 1.0);s.b[1539] = (p.p784 <= 0.0);s.store_scalar(1539, if s.b[1539] { 1.0 } else { 0.0 });
        if s.b[1539] {s.store_scalar(363, 0.0);}
        if (!s.b[1539]) {s.store_div_scaled_offset_numerator_mixed_ai(12, A::div(s.ad_value(167), s.ad_value(129)), 1.0, p.p784, 360, 1.0);s.store_mul_ln_mixed_ia(363, 129, A::max_with_scalar(s.ad_value(12), 1e-38));}
        s.b[1540] = (s.v[363] < 0.0);s.store_scalar(1540, if s.b[1540] { 1.0 } else { 0.0 });
        if ((!s.b[1539]) && s.b[1540]) {s.store_scalar(363, 0.0);}
        s.store_mul_add_scaled_inputs_rhs_mixed_ai(367, 108, A::offset(s.ad_value(97), s.v[46]), 1.0 / (1.60219e-19), 483, 1.0 / (1.60219e-19));s.store_mul_ad_affine_product_lhs(366, A::mul3_scaled_output(s.ad_value(90), s.ad_value(108), s.ad_value(144), (2.0 * s.v[46])), s.ad_value(628), 6.241457005723417e18, 0.0, 611);s.store_mul_ad_affine_product_lhs(736, s.ad_value(108), A::abs(s.ad_value(188)), ((1.60219e-19 * 1.60219e-19) * 1.60219e-19), 0.0, 157);s.store_mul3_affine_lhs(737, 108, 188, 1.60219e-19, 0.0, 188);s.store_add_scaled_product_mixed_aii(738, A::scale_offset(s.ad_value(366), p.p799, p.p785), 1.0, 366, 366, p.p800);s.store_square_ad(739, A::add(s.ad_value(366), s.ad_value(367)));s.store_scale(740, 108, (p.p785 * 1.60219e-19));s.b[1541] = (p.p1065 == 1.0);s.store_scalar(1541, if s.b[1541] { 1.0 } else { 0.0 });
        if s.b[1541] {s.store_scalar(745, s.v[30]);s.store_div_scaled_inputs2_indices(712, 64, 1.0, 482, (-1.0), 108, 1.0);s.store_scaled_sqrt_ad(713, A::div_from_scalar((((2.0 * 1.60219e-19) * s.v[26]) * p.p1068), s.ad_value(108)), 1.0 / (s.v[46]));s.store_ln_ad(714, A::div_from_scalar(p.p1068, s.ad_value(28)));s.store_scalar(13, 1.0);s.store_div(204, 712, 13);s.store_div(205, 713, 13);s.store_sub_scaled_inputs_mixed_ia(13, 204, 0.5, A::scale_offset(s.ad_value(205), ((0.7071067811865475) * (3.0)), 3.0), 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_61(
        s: &mut ReactiveScratch,
    ) {
        if s.b[1541] {s.store_add_mixed_ia(14, 13, A::sqrt(A::add_scaled_inputs(A::square(s.ad_value(13)), 1.0, s.ad_value(204), 6.0)));}
        s.b[1542] = (s.v[204] < 0.0);s.store_scalar(1542, if s.b[1542] { 1.0 } else { 0.0 });
        if (s.b[1541] && s.b[1542]) {s.store_div_scaled_inputs2_indices(15, 204, 1.0, 14, (-1.0), 205, 1.0);s.store_neg_ad(715, A::ln(A::max_with_scalar(A::add(A::sub_from_scalar(1.0, s.ad_value(14)), A::square(s.ad_value(15))), 1e-38)));}
        if (s.b[1541] && (!s.b[1542])) {s.store_limited_exp_neg_input(15, 14);s.store_scale(13, 205, 0.5);s.store_sub_mixed_ai(14, A::sqrt(A::add_scaled_inputs3_offset(s.ad_value(204), 1.0, s.ad_value(15), 1.0, A::square(s.ad_value(13)), 1.0, (-1.0))), 13);s.store_sub_offset_lhs_mixed_ai(715, A::square(s.ad_value(14)), 1.0, 15);}
        if s.b[1541] {s.store_scaled_add_offset_sqrt_square_offset(20, 715, 1.0, (-1.0), ((0.25 * 2.0) * 2.0), 0.5);s.store_sqrt(96, 20);s.store_div_scaled_offset_numerator_mixed_ai(12, A::div_scaled_inputs(s.ad_value(713), 1.0, s.ad_value(96), 2.0), 1.0, 1.0, 713, 1.0);s.store_add_scaled_inputs3_indices(13, 715, 1.0, 714, (-2.0), 73, -1.0);s.store_sub_mixed_ia(14, 13, A::ln(A::max_with_scalar(A::mul_scaled_lhs(s.ad_value(12), 4.0, s.ad_value(96)), 1e-38)));s.store_scaled_sub_ad(20, A::offset(s.ad_value(14), (-0.201491)), A::sqrt_offset_rhs_product_offset(s.ad_value(14), s.ad_value(14), 0.402982, 2.446562), 0.5);s.copy_ad(94, 96);}
        s.b[1543] = (s.v[20] <= (-68.0));s.store_scalar(1543, if s.b[1543] { 1.0 } else { 0.0 });
        if (s.b[1541] && s.b[1543]) {s.store_scalar(16, (-100.0));s.store_scalar(17, 20.0);}
        s.b[1544] = (s.v[20] < (s.v[16] - (0.5 * s.v[17])));s.store_scalar(1544, if s.b[1544] { 1.0 } else { 0.0 });
        if ((s.b[1541] && s.b[1543]) && s.b[1544]) {s.store_limited_exp(15, 16);}
        s.b[1545] = (s.v[20] > (s.v[16] + (0.5 * s.v[17])));s.store_scalar(1545, if s.b[1545] { 1.0 } else { 0.0 });
        if (((s.b[1541] && s.b[1543]) && (!s.b[1544])) && s.b[1545]) {s.store_limited_exp(15, 20);}
        if (((s.b[1541] && s.b[1543]) && (!s.b[1544])) && (!s.b[1545])) {s.store_div_scaled_inputs2_indices(14, 20, 1.0, 16, (-1.0), 17, 1.0);s.store_square(18, 14);s.store_limited_exp_ad(15, A::add_scaled_product(s.ad_value(16), 1.0, s.ad_value(17), A::add(A::scale_offset(s.ad_value(14), 0.5, (5.0 / 64.0)), A::mul_sub_from_scalar_rhs(s.ad_value(18), (15.0 / 16.0), A::mul_sub_from_scalar_rhs(s.ad_value(18), 1.25, s.ad_value(18)))), 1.0));}
        if (s.b[1541] && s.b[1543]) {s.store_mul_add_scaled_inputs3_offset_rhs_mixed_iia(717, 15, 13, 1.0, 20, (-1.0), A::ln(A::max_with_scalar(A::mul_scaled_lhs(s.ad_value(12), 2.0, A::add_scaled_product(s.ad_value(94), 2.0, s.ad_value(15), s.ad_value(12), 2.0)), 1e-38)), -1.0, 1.0);}
        if (s.b[1541] && (!s.b[1543])) {s.store_limited_exp(15, 20);s.store_div_from_scalar(95, 1.0, 94);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_62(
        s: &mut ReactiveScratch,
    ) {
        if (s.b[1541] && (!s.b[1543])) {s.store_add_scaled_inputs3_mixed_iai(16, 15, 2.0, A::ln(A::max_with_scalar(A::mul3_scaled_output(s.ad_value(15), s.ad_value(12), A::add_scaled_product(s.ad_value(94), 2.0, s.ad_value(15), s.ad_value(12), 2.0), 2.0), 1e-38)), 1.0, 13, -1.0);s.store_add_ad(17, A::offset(A::div_from_scalar(1.0, s.ad_value(15)), 2.0), A::div_scaled_inputs2(s.ad_value(12), 1.0, s.ad_value(95), 1.0, A::add_scaled_product(s.ad_value(94), 1.0, s.ad_value(12), s.ad_value(15), 1.0), 1.0));s.store_sub_div_rhs_indices(15, 15, 16, 17);s.store_add_scaled_inputs3_mixed_iai(16, 15, 2.0, A::ln(A::max_with_scalar(A::mul3_scaled_output(s.ad_value(15), s.ad_value(12), A::add_scaled_product(s.ad_value(94), 2.0, s.ad_value(15), s.ad_value(12), 2.0), 2.0), 1e-38)), 1.0, 13, -1.0);s.store_add_ad(17, A::offset(A::div_from_scalar(1.0, s.ad_value(15)), 2.0), A::div_scaled_inputs2(s.ad_value(12), 1.0, s.ad_value(95), 1.0, A::add_scaled_product(s.ad_value(94), 1.0, s.ad_value(12), s.ad_value(15), 1.0), 1.0));s.store_scaled_mul_ad(18, A::div_scaled_inputs2(s.ad_value(12), 1.0, s.ad_value(95), 1.0, A::add_scaled_product(s.ad_value(94), 1.0, s.ad_value(12), s.ad_value(15), 1.0), 1.0), A::div_scaled_inputs2(s.ad_value(12), 1.0, s.ad_value(95), 1.0, A::add_scaled_product(s.ad_value(94), 1.0, s.ad_value(12), s.ad_value(15), 1.0), 1.0), 1.0);s.store_sub_mixed_ai(19, A::add_scaled_product(A::div_from_scalar(1.0, A::mul3(A::square(s.ad_value(94)), s.ad_value(94), A::add_scaled_product(s.ad_value(94), 1.0, s.ad_value(12), s.ad_value(15), 1.0))), (-1.0), A::div_from_scalar(1.0, s.ad_value(15)), A::div_from_scalar(1.0, s.ad_value(15)), (-1.0)), 18);s.store_add_scaled_offset_product_rhs_mixed_iaa(717, 15, 1.0, A::div(s.ad_value(16), s.ad_value(17)), A::div_scaled_product_by_product(s.ad_value(16), s.ad_value(19), 1.0, s.ad_value(17), s.ad_value(17), 2.0), 1.0, (-1.0));}
        s.b[1546] = ((1.0 == 0.0) && (s.v[715] < ((-2500.0) * 2.0)));s.store_scalar(1546, if s.b[1546] { 1.0 } else { 0.0 });
        if (s.b[1541] && s.b[1546]) {s.store_div_from_scalar_scaled_input(716, ((-2.0) * 2.0), 715, 16.0);}
        if (s.b[1541] && (!s.b[1546])) {s.store_scaled_add_offset_sqrt_square_offset(716, 715, 1.0, (-1.0), ((0.25 * 2.0) * 2.0), 0.5);}
        if s.b[1541] {s.store_offset_div_scaled_inputs_sqrt_rhs(718, 713, 1.0, 716, 2.0, 1.0);s.copy_ad(719, 157);s.store_scale(726, 719, (s.v[46] * s.v[29]));}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_63(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if s.b[1541] {s.store_scale(725, 157, (s.v[46] * s.v[29]));s.store_div_scaled_product_by_product_mixed_iiai(720, 188, 746, 1.0, A::mul3_scaled_output(s.ad_value(718), s.ad_value(726), s.ad_value(108), 2.0), 108, 1.0);s.store_div_scaled_product_by_product_mixed_iaai(722, 188, A::sub(s.ad_value(745), s.ad_value(746)), 1.0, A::mul3_scaled_output(s.ad_value(90), s.ad_value(725), s.ad_value(106), 2.0), 106, 1.0);s.store_add_scaled_inputs3_offset_mixed_aii(12, A::square(s.ad_value(717)), 4.0, 717, 4.0, 720, (-4.0), 1.0);s.store_offset_scaled_ad(723, A::sqrt(A::offset(A::add_scaled_inputs3(A::square(s.ad_value(144)), 4.0, s.ad_value(144), 4.0, s.ad_value(722), 4.0), 1.0)), 0.5, (-0.5));}
        s.b[1548] = (s.v[30] != s.v[746]);s.store_scalar(1548, if s.b[1548] { 1.0 } else { 0.0 });
        if (s.b[1541] && s.b[1548]) {s.store_mul3_affine_lhs(724, 90, 108, ((2.0 * s.v[46]) * 6.241457005723417e18), 0.0, 723);s.store_primal_add_scaled_inputs3_indices(361, 745, 1.0, 359, (-2.0), 746, -1.0);s.store_primal_square(362, 361);s.store_scale(13, 362, (10000000000.0 * s.v[46]));s.store_scaled_ln_ad(14, A::max_with_scalar(A::div_scaled_inputs2(s.ad_value(724), 1.0, s.ad_value(367), 1.0, A::add(s.ad_value(366), s.ad_value(367)), 1.0), 1e-38), p.p785);s.store_scaled_sub(15, 724, 366, p.p799);s.store_scaled_sub_ad(16, A::square(s.ad_value(724)), A::square(s.ad_value(366)), (0.5 * p.p800));s.store_scale(17, 362, (10000000000.0 * (s.v[29] * p.p2)));s.store_add_scaled_product(732, A::div_scaled_product3_by_product(s.ad_value(737), s.ad_value(363), s.ad_value(738), 1.0, s.ad_value(17), s.ad_value(739), 1.0), 1.0, A::div(s.ad_value(736), s.ad_value(13)), A::add_scaled_inputs3(s.ad_value(14), 1.0, s.ad_value(15), 1.0, s.ad_value(16), 1.0), 1.0);s.store_mul3_affine_lhs(18, 361, 367, ((s.v[29] * p.p2) * 10000000000.0), 0.0, 367);s.store_mul_ad_product_lhs_mixed_ai(733, A::div(s.ad_value(740), s.ad_value(18)), 188, 188);s.store_add(19, 733, 732);}
        if s.b[1541] {s.store_scale(20, 108, (p.p1067 * 1.60219e-19));s.store_mul3_affine_lhs(21, 746, 367, ((s.v[29] * p.p2) * 10000000000.0), 0.0, 367);s.store_mul_ad_product_lhs_mixed_ai(741, A::div(s.ad_value(20), s.ad_value(21)), 188, 188);s.copy_ad(22, 741);}
        s.b[1551] = (p.p801 >= (s.v[30] / 2.0));s.store_scalar(1551, if s.b[1551] { 1.0 } else { 0.0 });
        if ((!s.b[1541]) && s.b[1551]) {s.store_scalar(359, 0.0);}
        if ((!s.b[1541]) && (!s.b[1551])) {s.store_scalar(359, p.p801);}
        s.b[1552] = (((p.p785 > 0.0) || (p.p799 > 0.0)) || (p.p800 > 0.0));s.store_scalar(1552, if s.b[1552] { 1.0 } else { 0.0 });s.b[1553] = ((p.p786 != 0.0) && (p.p785 > 0.0));s.store_scalar(1553, if s.b[1553] { 1.0 } else { 0.0 });
        if (((!s.b[1541]) && s.b[1552]) && s.b[1553]) {s.store_div(13, 80, 641);s.store_offset_pow_ad(14, s.ad_value(13), s.ad_value(642), 1.0);s.store_div(15, 640, 14);s.store_scale(16, 15, 1.0 / (p.p785));s.store_scaled_add_offset_sqrt_square_offset(17, 16, 1.0, (-1.0), ((0.25 * p.p798) * p.p798), 0.5);s.store_scale(364, 17, p.p785);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_64(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (((!s.b[1541]) && s.b[1552]) && (!s.b[1553])) {s.store_scalar(364, p.p785);}
        if ((!s.b[1541]) && s.b[1552]) {s.store_primal_sub_from_scalar_scaled_input(361, s.v[30], 359, 2.0);s.store_primal_square(362, 361);s.store_scale(12, 362, (10000000000.0 * s.v[46]));s.store_mul_ad_affine_product_lhs(365, A::mul3_scaled_output(s.ad_value(90), s.ad_value(108), s.ad_value(200), (2.0 * s.v[46])), s.ad_value(628), 6.241457005723417e18, 0.0, 611);s.store_mul_ln_mixed_ia(13, 364, A::max_with_scalar(A::div_scaled_inputs2(s.ad_value(365), 1.0, s.ad_value(367), 1.0, A::add(s.ad_value(366), s.ad_value(367)), 1.0), 1e-38));s.store_scaled_sub(14, 365, 366, p.p799);s.store_scaled_sub_ad(15, A::square(s.ad_value(365)), A::square(s.ad_value(366)), (0.5 * p.p800));s.store_scale(16, 362, (10000000000.0 * (s.v[29] * p.p2)));s.store_add_scaled_product(368, A::div_scaled_product3_by_product(s.ad_value(737), s.ad_value(363), s.ad_value(738), 1.0, s.ad_value(16), s.ad_value(739), 1.0), 1.0, A::div(s.ad_value(736), s.ad_value(12)), A::add_scaled_inputs3(s.ad_value(13), 1.0, s.ad_value(14), 1.0, s.ad_value(15), 1.0), 1.0);s.store_mul3_affine_lhs(17, 361, 367, ((s.v[29] * p.p2) * 10000000000.0), 0.0, 367);s.store_scaled_mul(740, 364, 108, 1.60219e-19);s.store_mul_ad_product_lhs_mixed_ai(369, A::div(s.ad_value(740), s.ad_value(17)), 188, 188);s.store_add(18, 369, 368);}
        s.store_scaled_div(12, 80, 360, 1.0 / (s.v[30]));s.store_square(13, 12);s.store_offset_scaled(15, 13, (((p.p814 * s.v[30])) * (p.p811)), p.p811);s.store_offset_scaled(16, 13, (((p.p815 * s.v[30])) * (p.p812)), p.p812);s.store_offset_scaled(17, 13, (((p.p1044 * s.v[30])) * (p.p1043)), p.p1043);s.store_square(389, 17);s.store_square(388, 16);s.b[1555] = (p.p48 == 0.0);s.store_scalar(1555, if s.b[1555] { 1.0 } else { 0.0 });s.b[1556] = (p.p48 == 1.0);s.store_scalar(1556, if s.b[1556] { 1.0 } else { 0.0 });
        if s.b[1555] {s.store_scaled_mul(196, 108, 190, ((((-p.p2) * s.v[29]) * s.v[30]) * s.v[46]));s.store_scaled_mul(197, 108, 193, ((((-p.p2) * s.v[29]) * s.v[30]) * s.v[46]));s.store_mul_abs_mixed_ia(12, 157, A::add(s.ad_value(196), s.ad_value(197)));s.store_offset_mul(13, 12, 244, (s.v[30] * s.v[30]));}
        if (s.b[1556] && (!s.b[1555])) {s.store_scaled_mul(382, 90, 106, 2.0);s.store_mul_scale_offset_mixed_ia(12, 382, A::mul3(s.ad_value(157), s.ad_value(163), s.ad_value(175)), s.v[46], 0.0);s.store_scaled_add(13, 200, 144, 0.5);s.store_offset(15, 13, 0.5);s.store_square(16, 15);s.store_mul(17, 16, 15);s.store_sub(18, 200, 144);s.store_square(19, 18);s.store_mul(20, 19, 18);s.store_mul_scale_offset_rhs(21, 19, 13, 6.0, 0.5);s.store_scale(381, 163, s.v[30]);s.store_scale(22, 381, 1.0 / (s.v[30]));s.store_offset_ad(24, A::div_scaled_product_by_product(s.ad_value(389), s.ad_value(139), 1.0, s.ad_value(140), A::offset(s.ad_value(80), p.p1045), 1.0), 1.0);s.store_offset_scaled(24, 24, ((((-s.v[30]) / p.p1042)) as f64).exp(), (((((-1.0)) * (((((-s.v[30]) / p.p1042)) as f64).exp()))) + (1.0)));}
        s.b[1557] = ((0.0 == 0.0) && (s.v[24] < ((-2500.0) * 0.1)));s.store_scalar(1557, if s.b[1557] { 1.0 } else { 0.0 });
        if ((s.b[1556] && (!s.b[1555])) && s.b[1557]) {s.store_div_from_scalar_scaled_input(24, ((-0.1) * 0.1), 24, 16.0);}
        if ((s.b[1556] && (!s.b[1555])) && (!s.b[1557])) {s.store_scaled_add_mixed_ia(24, 24, A::sqrt_square_offset(s.ad_value(24), ((0.25 * 0.1) * 0.1)), 0.5);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_65(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (s.b[1556] && (!s.b[1555])) {s.store_div_scaled_product3_mixed_aaii(378, A::mul3(s.ad_value(381), s.ad_value(22), s.ad_value(22)), A::add_scaled_inputs3(A::div(s.ad_value(13), s.ad_value(16)), 1.0, A::div(s.ad_value(21), A::mul_scaled_lhs(s.ad_value(16), 60.0, s.ad_value(16))), (-1.0), A::div_scaled_product_by_product(s.ad_value(19), s.ad_value(19), 1.0, s.ad_value(16), s.ad_value(17), 144.0), 1.0), 388, (15.0 * 1.0 / (4.0)), 12, ((p.p2 * s.v[29]) * 12.0));}
        s.copy_ad(60, 59);s.store_scalar(218, 0.0);s.b[1562] = (p.p40 == 1.0);s.store_scalar(1562, if s.b[1562] { 1.0 } else { 0.0 });
        if s.b[1562] {s.store_offset(549, 549, p.p35);s.store_mul(65, 64, 109);s.store_mul(73, 72, 109);s.store_mul(58, 549, 109);s.store_sub(60, 65, 58);s.store_ln_ad(233, A::max_with_scalar(A::div(s.ad_value(550), s.ad_value(28)), 1e-38));s.store_scaled_sqrt_mul_scaled_lhs(234, 550, ((2.0 * 1.60219e-19) * s.v[26]), 109, 1.0 / (s.v[46]));s.store_div_from_scalar(126, 1.0, 234);s.store_div_scaled_inputs_indices(206, 479, ((2.0 * 1.60219e-19) * s.v[26]), 108, (s.v[46] * s.v[46]));}
        if s.b[1562] {
            if (s.v[479] > 0.0) {
                s.store_div_from_scalar(218, 1.0, 206);
            } else {
                s.store_scalar(218, 0.0);
            }
        }
        if s.b[1562] {
            if (s.v[479] > 0.0) {
                s.store_div(203, 550, 479);
            } else {
                s.store_scalar(203, 0.0);
            }
        }
        if s.b[1562] {s.store_offset(13, 203, 1.0);s.store_div(204, 60, 13);s.store_div(205, 234, 13);s.store_sub_scaled_inputs_mixed_ia(13, 204, 0.5, A::scale_offset(s.ad_value(205), ((0.7071067811865475) * (3.0)), 3.0), 1.0);s.store_add_mixed_ia(14, 13, A::sqrt(A::add_scaled_inputs(A::square(s.ad_value(13)), 1.0, s.ad_value(204), 6.0)));}
        s.b[1563] = (s.v[204] < 0.0);s.store_scalar(1563, if s.b[1563] { 1.0 } else { 0.0 });
        if (s.b[1562] && s.b[1563]) {s.store_div_scaled_inputs2_indices(15, 204, 1.0, 14, (-1.0), 205, 1.0);s.store_neg_ad(91, A::ln(A::max_with_scalar(A::add(A::sub_from_scalar(1.0, s.ad_value(14)), A::square(s.ad_value(15))), 1e-38)));}
        if (s.b[1562] && (!s.b[1563])) {s.store_limited_exp_neg_input(15, 14);s.store_scale(13, 205, 0.5);s.store_sub_mixed_ai(14, A::sqrt(A::add_scaled_inputs3_offset(s.ad_value(204), 1.0, s.ad_value(15), 1.0, A::square(s.ad_value(13)), 1.0, (-1.0))), 13);s.store_sub_offset_lhs_mixed_ai(91, A::square(s.ad_value(14)), 1.0, 15);}
        if s.b[1562] {s.store_scaled_add_offset_sqrt_square_offset(20, 91, 1.0, (-1.0), ((0.25 * 2.0) * 2.0), 0.5);s.store_sqrt(96, 20);s.store_div_scaled_offset_numerator_mixed_ai(12, A::div_scaled_inputs(s.ad_value(234), 1.0, s.ad_value(96), 2.0), 1.0, 1.0, 234, 1.0);s.store_add_scaled_inputs3_indices(13, 91, 1.0, 233, (-2.0), 73, -1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_66(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if s.b[1562] {s.store_sub_scaled_inputs_mixed_ia(14, 13, 1.0 / (p.p1137), A::ln(A::max_with_scalar(A::mul_scaled_lhs(s.ad_value(12), 4.0, s.ad_value(96)), 1e-38)), 1.0);s.store_scaled_sub_ad(20, A::offset(s.ad_value(14), (-0.201491)), A::sqrt_offset_rhs_product_offset(s.ad_value(14), s.ad_value(14), 0.402982, 2.446562), 0.5);s.copy_ad(94, 96);}
        s.b[1564] = (s.v[20] <= (-68.0));s.store_scalar(1564, if s.b[1564] { 1.0 } else { 0.0 });
        if (s.b[1562] && s.b[1564]) {s.store_scalar(16, (-100.0));s.store_scalar(17, 20.0);}
        s.b[1565] = (s.v[20] < (s.v[16] - (0.5 * s.v[17])));s.store_scalar(1565, if s.b[1565] { 1.0 } else { 0.0 });
        if ((s.b[1562] && s.b[1564]) && s.b[1565]) {s.store_limited_exp(15, 16);}
        s.b[1566] = (s.v[20] > (s.v[16] + (0.5 * s.v[17])));s.store_scalar(1566, if s.b[1566] { 1.0 } else { 0.0 });
        if (((s.b[1562] && s.b[1564]) && (!s.b[1565])) && s.b[1566]) {s.store_limited_exp(15, 20);}
        if (((s.b[1562] && s.b[1564]) && (!s.b[1565])) && (!s.b[1566])) {s.store_div_scaled_inputs2_indices(14, 20, 1.0, 16, (-1.0), 17, 1.0);s.store_square(18, 14);s.store_limited_exp_ad(15, A::add_scaled_product(s.ad_value(16), 1.0, s.ad_value(17), A::add(A::scale_offset(s.ad_value(14), 0.5, (5.0 / 64.0)), A::mul_sub_from_scalar_rhs(s.ad_value(18), (15.0 / 16.0), A::mul_sub_from_scalar_rhs(s.ad_value(18), 1.25, s.ad_value(18)))), 1.0));}
        if (s.b[1562] && s.b[1564]) {s.store_mul_add_scaled_inputs3_offset_rhs_mixed_iia(200, 15, 13, 1.0, 20, (-p.p1137), A::ln(A::max_with_scalar(A::mul_scaled_lhs(s.ad_value(12), 2.0, A::add_scaled_product(s.ad_value(94), 2.0, s.ad_value(15), s.ad_value(12), 2.0)), 1e-38)), (-p.p1137), 1.0);}
        if (s.b[1562] && (!s.b[1564])) {s.store_limited_exp(15, 20);s.store_div_from_scalar(95, 1.0, 94);s.store_add_scaled_inputs3_mixed_iai(16, 15, 2.0, A::ln(A::max_with_scalar(A::mul3_scaled_output(s.ad_value(15), s.ad_value(12), A::add_scaled_product(s.ad_value(94), 2.0, s.ad_value(15), s.ad_value(12), 2.0), 2.0), 1e-38)), p.p1137, 13, -1.0);s.store_add_ad(17, A::offset(A::div_from_scalar(p.p1137, s.ad_value(15)), 2.0), A::div_scaled_inputs2(s.ad_value(12), p.p1137, s.ad_value(95), p.p1137, A::add_scaled_product(s.ad_value(94), 1.0, s.ad_value(12), s.ad_value(15), 1.0), 1.0));s.store_sub_div_rhs_indices(15, 15, 16, 17);s.store_add_scaled_inputs3_mixed_iai(16, 15, 2.0, A::ln(A::max_with_scalar(A::mul3_scaled_output(s.ad_value(15), s.ad_value(12), A::add_scaled_product(s.ad_value(94), 2.0, s.ad_value(15), s.ad_value(12), 2.0), 2.0), 1e-38)), p.p1137, 13, -1.0);s.store_add_ad(17, A::offset(A::div_from_scalar(p.p1137, s.ad_value(15)), 2.0), A::div_scaled_inputs2(s.ad_value(12), p.p1137, s.ad_value(95), p.p1137, A::add_scaled_product(s.ad_value(94), 1.0, s.ad_value(12), s.ad_value(15), 1.0), 1.0));}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_67(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (s.b[1562] && (!s.b[1564])) {s.store_scaled_mul_ad(18, A::div_scaled_inputs2(s.ad_value(12), 1.0, s.ad_value(95), 1.0, A::add_scaled_product(s.ad_value(94), 1.0, s.ad_value(12), s.ad_value(15), 1.0), 1.0), A::div_scaled_inputs2(s.ad_value(12), 1.0, s.ad_value(95), 1.0, A::add_scaled_product(s.ad_value(94), 1.0, s.ad_value(12), s.ad_value(15), 1.0), 1.0), p.p1137);s.store_sub_mixed_ai(19, A::add_scaled_product(A::div_from_scalar(p.p1137, A::mul3(A::square(s.ad_value(94)), s.ad_value(94), A::add_scaled_product(s.ad_value(94), 1.0, s.ad_value(12), s.ad_value(15), 1.0))), (-1.0), A::div_from_scalar(1.0, s.ad_value(15)), A::div_from_scalar(1.0, s.ad_value(15)), (-p.p1137)), 18);s.store_add_scaled_offset_product_rhs_mixed_iaa(200, 15, 1.0, A::div(s.ad_value(16), s.ad_value(17)), A::div_scaled_product_by_product(s.ad_value(16), s.ad_value(19), 1.0, s.ad_value(17), s.ad_value(17), 2.0), 1.0, (-1.0));}
        s.b[1567] = ((1.0 == 0.0) && (s.v[91] < ((-2500.0) * 2.0)));s.store_scalar(1567, if s.b[1567] { 1.0 } else { 0.0 });
        if (s.b[1562] && s.b[1567]) {s.store_div_from_scalar_scaled_input(93, ((-2.0) * 2.0), 91, 16.0);}
        if (s.b[1562] && (!s.b[1567])) {s.store_scaled_add_offset_sqrt_square_offset(93, 91, 1.0, (-1.0), ((0.25 * 2.0) * 2.0), 0.5);}
        if s.b[1562] {s.store_sqrt(96, 93);s.store_sub_scaled_inputs(92, 91, 1.0, 200, 2.0);}
        s.b[1568] = ((1.0 == 0.0) && (s.v[92] < ((-2500.0) * 2.0)));s.store_scalar(1568, if s.b[1568] { 1.0 } else { 0.0 });
        if (s.b[1562] && s.b[1568]) {s.store_div_from_scalar_scaled_input(12, ((-2.0) * 2.0), 92, 16.0);}
        if (s.b[1562] && (!s.b[1568])) {s.store_scaled_add_offset_sqrt_square_offset(12, 92, 1.0, (-1.0), ((0.25 * 2.0) * 2.0), 0.5);}
        if s.b[1562] {s.store_offset_div_ad(90, s.ad_value(234), A::add(s.ad_value(96), A::sqrt(s.ad_value(12))), 1.0);s.store_mul_mixed_ia(12, 108, A::add_scaled_inputs_product(s.ad_value(60), 1.0, s.ad_value(91), (-1.0), s.ad_value(200), A::offset(s.ad_value(90), (-1.0)), (-2.0)));}
        s.b[1569] = ((0.0 == 0.0) && (s.v[12] < ((-2500.0) * 0.1)));s.store_scalar(1569, if s.b[1569] { 1.0 } else { 0.0 });
        if (s.b[1562] && s.b[1569]) {s.store_div_from_scalar_scaled_input(84, ((-0.1) * 0.1), 12, 16.0);}
        if (s.b[1562] && (!s.b[1569])) {s.store_scaled_add_mixed_ia(84, 12, A::sqrt_square_offset(s.ad_value(12), ((0.25 * 0.1) * 0.1)), 0.5);}
        if s.b[1562] {s.store_mul3_affine_lhs(130, 90, 108, 2.0, 0.0, 200);s.store_add_scaled_inputs(132, 84, s.v[155], 130, (s.v[158] * s.v[155]));s.store_mul_add_scaled_product_pow_rhs(15, 506, 1.0, 516, 62, 1.0, 132, 407);s.store_offset(16, 15, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_68(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.b[1570] = ((1.0 == 0.0) && (s.v[16] < ((-2500.0) * 0.0015)));s.store_scalar(1570, if s.b[1570] { 1.0 } else { 0.0 });
        if (s.b[1562] && s.b[1570]) {s.store_div_from_scalar_scaled_input(133, ((-0.0015) * 0.0015), 16, 16.0);}
        if (s.b[1562] && (!s.b[1570])) {s.store_scaled_add_offset_sqrt_square_offset(133, 16, 1.0, (-1.0), ((0.25 * 0.0015) * 0.0015), 0.5);}
        if s.b[1562] {s.store_div_scaled_product_by_product_indices(137, 499, 108, 1.0, 133, 411, s.v[34]);s.store_div_scaled_product_offset_denominator_mixed_iaa(131, 137, A::add(A::square(s.ad_value(200)), s.ad_value(200)), 1.0, A::mul_offset_rhs(s.ad_value(137), s.ad_value(200), 1.0), 1.0, 1.0);s.store_add_scaled_inputs4_mixed_iiia(145, 91, 1.0, 233, (-2.0), 131, (-2.0), A::ln(A::max_with_scalar(A::mul(A::mul3_scaled_output(s.ad_value(131), s.ad_value(90), s.ad_value(126), 2.0), A::add(A::mul3_scaled_output(s.ad_value(131), s.ad_value(90), s.ad_value(126), 2.0), A::div_scaled_value_offset_denominator(s.ad_value(234), 1.0, s.ad_value(90), (-1.0), 1.0))), 1e-38)), -1.0);s.store_mul(146, 145, 108);}
        s.b[1571] = ((0.0 == 0.0) && ((s.v[146] - s.v[72]) < ((-2500.0) * 0.001)));s.store_scalar(1571, if s.b[1571] { 1.0 } else { 0.0 });
        if (s.b[1562] && s.b[1571]) {s.store_div_from_scalar_ad(141, ((-0.001) * 0.001), A::sub_scaled_inputs(s.ad_value(146), 16.0, s.ad_value(72), 16.0));}
        if (s.b[1562] && (!s.b[1571])) {s.store_add_scaled_inputs3_sqrt_third_sub_square_offset(141, 146, 0.5, 72, ((-1.0) * 0.5), 146, 72, ((0.25 * 0.001) * 0.001), 0.5);}
        s.b[1572] = ((p.p1134 == 0.0) && (p.p1135 == 0.0));s.store_scalar(1572, if s.b[1572] { 1.0 } else { 0.0 });
        if (s.b[1562] && s.b[1572]) {s.store_scalar(783, p.p1129);}
        if (s.b[1562] && (!s.b[1572])) {s.store_div_from_scalar_offset_ad(13, s.v[30], A::sqrt(A::mul(s.ad_value(538), s.ad_value(112))), s.v[30]);s.store_offset_div_scaled_inputs2_mixed_iaa(783, 13, p.p1134, A::mul3_scaled_output(s.ad_value(13), s.ad_value(200), s.ad_value(106), p.p1135), (-1.0), A::scale_offset(s.ad_value(61), p.p1136, 1.0), 1.0, 1.0);}
        s.b[1573] = ((0.1 == 0.0) && (s.v[783] < ((-2500.0) * 0.0005)));s.store_scalar(1573, if s.b[1573] { 1.0 } else { 0.0 });
        if ((s.b[1562] && (!s.b[1572])) && s.b[1573]) {s.store_div_from_scalar_scaled_input(783, ((-0.0005) * 0.0005), 783, 16.0);}
        if ((s.b[1562] && (!s.b[1572])) && (!s.b[1573])) {s.store_scaled_add_offset_sqrt_square_offset(783, 783, 0.1, (-0.1), ((0.25 * 0.0005) * 0.0005), 0.5);}
        if s.b[1562] {s.store_div(141, 141, 783);s.store_pow_ad(19, A::offset(A::div(s.ad_value(74), s.ad_value(141)), 1e-6), A::div_from_scalar(1.0, s.ad_value(412)));s.store_pow_ad(20, A::offset(s.ad_value(19), 1.0), A::neg(s.ad_value(412)));s.store_mul(139, 75, 20);s.store_mul_add_lhs(142, 139, 72, 109);s.store_scaled_add_offset_sqrt_square_offset(20, 91, 1.0, (-1.0), ((0.25 * 2.0) * 2.0), 0.5);s.store_sqrt(96, 20);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_69(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if s.b[1562] {s.store_div_scaled_offset_numerator_mixed_ai(12, A::div_scaled_inputs(s.ad_value(234), 1.0, s.ad_value(96), 2.0), 1.0, 1.0, 234, 1.0);s.store_add_scaled_inputs3_indices(13, 91, 1.0, 233, (-2.0), 142, -1.0);s.store_sub_scaled_inputs_mixed_ia(14, 13, 1.0 / (p.p1137), A::ln(A::max_with_scalar(A::mul_scaled_lhs(s.ad_value(12), 4.0, s.ad_value(96)), 1e-38)), 1.0);s.store_scaled_sub_ad(20, A::offset(s.ad_value(14), (-0.201491)), A::sqrt_offset_rhs_product_offset(s.ad_value(14), s.ad_value(14), 0.402982, 2.446562), 0.5);s.copy_ad(94, 96);}
        s.b[1574] = (s.v[20] <= (-68.0));s.store_scalar(1574, if s.b[1574] { 1.0 } else { 0.0 });
        if (s.b[1562] && s.b[1574]) {s.store_scalar(16, (-100.0));s.store_scalar(17, 20.0);}
        s.b[1575] = (s.v[20] < (s.v[16] - (0.5 * s.v[17])));s.store_scalar(1575, if s.b[1575] { 1.0 } else { 0.0 });
        if ((s.b[1562] && s.b[1574]) && s.b[1575]) {s.store_limited_exp(15, 16);}
        s.b[1576] = (s.v[20] > (s.v[16] + (0.5 * s.v[17])));s.store_scalar(1576, if s.b[1576] { 1.0 } else { 0.0 });
        if (((s.b[1562] && s.b[1574]) && (!s.b[1575])) && s.b[1576]) {s.store_limited_exp(15, 20);}
        if (((s.b[1562] && s.b[1574]) && (!s.b[1575])) && (!s.b[1576])) {s.store_div_scaled_inputs2_indices(14, 20, 1.0, 16, (-1.0), 17, 1.0);s.store_square(18, 14);s.store_limited_exp_ad(15, A::add_scaled_product(s.ad_value(16), 1.0, s.ad_value(17), A::add(A::scale_offset(s.ad_value(14), 0.5, (5.0 / 64.0)), A::mul_sub_from_scalar_rhs(s.ad_value(18), (15.0 / 16.0), A::mul_sub_from_scalar_rhs(s.ad_value(18), 1.25, s.ad_value(18)))), 1.0));}
        if (s.b[1562] && s.b[1574]) {s.store_mul_add_scaled_inputs3_offset_rhs_mixed_iia(144, 15, 13, 1.0, 20, (-p.p1137), A::ln(A::max_with_scalar(A::mul_scaled_lhs(s.ad_value(12), 2.0, A::add_scaled_product(s.ad_value(94), 2.0, s.ad_value(15), s.ad_value(12), 2.0)), 1e-38)), (-p.p1137), 1.0);}
        if (s.b[1562] && (!s.b[1574])) {s.store_limited_exp(15, 20);s.store_div_from_scalar(95, 1.0, 94);s.store_add_scaled_inputs3_mixed_iai(16, 15, 2.0, A::ln(A::max_with_scalar(A::mul3_scaled_output(s.ad_value(15), s.ad_value(12), A::add_scaled_product(s.ad_value(94), 2.0, s.ad_value(15), s.ad_value(12), 2.0), 2.0), 1e-38)), p.p1137, 13, -1.0);s.store_add_ad(17, A::offset(A::div_from_scalar(p.p1137, s.ad_value(15)), 2.0), A::div_scaled_inputs2(s.ad_value(12), p.p1137, s.ad_value(95), p.p1137, A::add_scaled_product(s.ad_value(94), 1.0, s.ad_value(12), s.ad_value(15), 1.0), 1.0));s.store_sub_div_rhs_indices(15, 15, 16, 17);s.store_add_scaled_inputs3_mixed_iai(16, 15, 2.0, A::ln(A::max_with_scalar(A::mul3_scaled_output(s.ad_value(15), s.ad_value(12), A::add_scaled_product(s.ad_value(94), 2.0, s.ad_value(15), s.ad_value(12), 2.0), 2.0), 1e-38)), p.p1137, 13, -1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_70(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (s.b[1562] && (!s.b[1574])) {s.store_add_ad(17, A::offset(A::div_from_scalar(p.p1137, s.ad_value(15)), 2.0), A::div_scaled_inputs2(s.ad_value(12), p.p1137, s.ad_value(95), p.p1137, A::add_scaled_product(s.ad_value(94), 1.0, s.ad_value(12), s.ad_value(15), 1.0), 1.0));s.store_scaled_mul_ad(18, A::div_scaled_inputs2(s.ad_value(12), 1.0, s.ad_value(95), 1.0, A::add_scaled_product(s.ad_value(94), 1.0, s.ad_value(12), s.ad_value(15), 1.0), 1.0), A::div_scaled_inputs2(s.ad_value(12), 1.0, s.ad_value(95), 1.0, A::add_scaled_product(s.ad_value(94), 1.0, s.ad_value(12), s.ad_value(15), 1.0), 1.0), p.p1137);s.store_sub_mixed_ai(19, A::add_scaled_product(A::div_from_scalar(p.p1137, A::mul3(A::square(s.ad_value(94)), s.ad_value(94), A::add_scaled_product(s.ad_value(94), 1.0, s.ad_value(12), s.ad_value(15), 1.0))), (-1.0), A::div_from_scalar(1.0, s.ad_value(15)), A::div_from_scalar(1.0, s.ad_value(15)), (-p.p1137)), 18);s.store_add_scaled_offset_product_rhs_mixed_iaa(144, 15, 1.0, A::div(s.ad_value(16), s.ad_value(17)), A::div_scaled_product_by_product(s.ad_value(16), s.ad_value(19), 1.0, s.ad_value(17), s.ad_value(17), 2.0), 1.0, (-1.0));}
        if s.b[1562] {s.store_add_scaled_inputs3_offset_indices(92, 91, 1.0, 200, (-1.0), 144, -1.0, (-1.0));}
        s.b[1577] = ((1.0 == 0.0) && (s.v[92] < ((-2500.0) * 2.0)));s.store_scalar(1577, if s.b[1577] { 1.0 } else { 0.0 });
        if (s.b[1562] && s.b[1577]) {s.store_div_from_scalar_scaled_input(12, ((-2.0) * 2.0), 92, 16.0);}
        if (s.b[1562] && (!s.b[1577])) {s.store_scaled_add_offset_sqrt_square_offset(12, 92, 1.0, (-1.0), ((0.25 * 2.0) * 2.0), 0.5);}
        if s.b[1562] {s.store_sqrt(14, 12);s.store_add_offset_lhs_mixed_ia(15, 203, 1.0, A::div(s.ad_value(234), A::add(s.ad_value(96), s.ad_value(14))));s.store_offset_product3(16, s.ad_value(203), s.ad_value(14), s.ad_value(126), 1.0, 0.5);s.store_sqrt_add_ad(17, A::square(s.ad_value(16)), A::mul3(s.ad_value(15), A::add(s.ad_value(200), s.ad_value(144)), s.ad_value(218)));s.store_div_add_scaled_inputs_rhs_indices(90, 15, 16, 1.0, 17, 1.0);s.store_mul_mixed_ia(12, 108, A::add_scaled_inputs_product(s.ad_value(60), 1.0, s.ad_value(91), (-1.0), s.ad_value(200), A::offset(s.ad_value(90), (-1.0)), (-2.0)));}
        s.b[1578] = ((0.0 == 0.0) && (s.v[12] < ((-2500.0) * 0.1)));s.store_scalar(1578, if s.b[1578] { 1.0 } else { 0.0 });
        if (s.b[1562] && s.b[1578]) {s.store_div_from_scalar_scaled_input(84, ((-0.1) * 0.1), 12, 16.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_71(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (s.b[1562] && (!s.b[1578])) {s.store_scaled_add_mixed_ia(84, 12, A::sqrt_square_offset(s.ad_value(12), ((0.25 * 0.1) * 0.1)), 0.5);}
        if s.b[1562] {s.store_mul_mixed_ia(13, 108, A::add_scaled_inputs_product(s.ad_value(60), 1.0, s.ad_value(91), (-1.0), s.ad_value(144), A::offset(s.ad_value(90), (-1.0)), (-2.0)));}
        s.b[1579] = ((0.0 == 0.0) && (s.v[13] < ((-2500.0) * 0.1)));s.store_scalar(1579, if s.b[1579] { 1.0 } else { 0.0 });
        if (s.b[1562] && s.b[1579]) {s.store_div_from_scalar_scaled_input(85, ((-0.1) * 0.1), 13, 16.0);}
        if (s.b[1562] && (!s.b[1579])) {s.store_scaled_add_mixed_ia(85, 13, A::sqrt_square_offset(s.ad_value(13), ((0.25 * 0.1) * 0.1)), 0.5);}
        if s.b[1562] {s.store_scaled_add(86, 84, 85, 0.5);s.store_mul_ad_product_rhs_mixed_ia(80, 90, 108, A::add(s.ad_value(200), s.ad_value(144)));s.store_add_scaled_inputs(156, 86, s.v[155], 80, (s.v[158] * s.v[155]));s.store_offset(13, 203, 1.0);s.store_div_scaled_inputs2_indices(204, 60, 1.0, 109, p.p136, 13, 1.0);s.store_div(205, 234, 13);s.store_sub_scaled_inputs_mixed_ia(13, 204, 0.5, A::scale_offset(s.ad_value(205), ((0.7071067811865475) * (3.0)), 3.0), 1.0);s.store_add_mixed_ia(14, 13, A::sqrt(A::add_scaled_inputs(A::square(s.ad_value(13)), 1.0, s.ad_value(204), 6.0)));}
        s.b[1580] = (s.v[204] < 0.0);s.store_scalar(1580, if s.b[1580] { 1.0 } else { 0.0 });
        if (s.b[1562] && s.b[1580]) {s.store_div_scaled_inputs2_indices(15, 204, 1.0, 14, (-1.0), 205, 1.0);s.store_neg_ad(91, A::ln(A::max_with_scalar(A::add(A::sub_from_scalar(1.0, s.ad_value(14)), A::square(s.ad_value(15))), 1e-38)));}
        if (s.b[1562] && (!s.b[1580])) {s.store_limited_exp_neg_input(15, 14);s.store_scale(13, 205, 0.5);s.store_sub_mixed_ai(14, A::sqrt(A::add_scaled_inputs3_offset(s.ad_value(204), 1.0, s.ad_value(15), 1.0, A::square(s.ad_value(13)), 1.0, (-1.0))), 13);s.store_sub_offset_lhs_mixed_ai(91, A::square(s.ad_value(14)), 1.0, 15);}
        if s.b[1562] {s.store_mul_add_scaled_product_pow_rhs(15, 506, 1.0, 516, 62, 1.0, 156, 407);s.store_offset(16, 15, 1.0);}
        s.b[1581] = ((1.0 == 0.0) && (s.v[16] < ((-2500.0) * 0.0015)));s.store_scalar(1581, if s.b[1581] { 1.0 } else { 0.0 });
        if (s.b[1562] && s.b[1581]) {s.store_div_from_scalar_scaled_input(159, ((-0.0015) * 0.0015), 16, 16.0);}
        if (s.b[1562] && (!s.b[1581])) {s.store_scaled_add_offset_sqrt_square_offset(159, 16, 1.0, (-1.0), ((0.25 * 0.0015) * 0.0015), 0.5);}
        if s.b[1562] {s.store_div_scaled_product_by_product_indices(138, 499, 108, 2.0, 159, 411, s.v[34]);s.store_sub(87, 200, 144);s.store_mul_ad_affine_product_rhs(13, 138, s.ad_value(87), A::mul(s.ad_value(138), s.ad_value(87)), 2.0, 0.0);s.store_sqrt_offset_input(161, 13, 1.0);s.store_scaled_offset(162, 161, 1.0, 0.5);s.store_div_scaled_inputs_mixed_ia(134, 411, 2.0, A::div(s.ad_value(499), s.ad_value(159)), 1.0);s.store_scale(135, 134, s.v[34]);s.store_add(170, 141, 135);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_72(
        s: &mut ReactiveScratch,
    ) {
        if s.b[1562] {s.store_sub(167, 75, 139);}
        s.b[1582] = (s.v[542] != 0.0);s.store_scalar(1582, if s.b[1582] { 1.0 } else { 0.0 });
        if s.b[1582] {s.store_offset_mul_ad(176, s.ad_value(542), A::ln(A::max_with_scalar(A::offset(A::div_scaled_value_by_product(s.ad_value(167), 1.0, s.ad_value(542), s.ad_value(170), 1.0), 1.0), 1e-38)), 1.0);}
        if (!s.b[1582]) {s.store_scalar(176, 1.0);}
        s.store_square(207, 176);s.store_div_from_scalar(208, 1.0, 176);s.store_div_from_scalar(209, 1.0, 207);s.store_offset(210, 176, (-1.0));s.store_sub(213, 60, 91);s.store_sub(216, 200, 144);s.store_square_ad(217, A::sub(s.ad_value(200), s.ad_value(144)));s.store_add_scaled_inputs(211, 213, 1.0, 200, 2.0);s.store_add_scaled_inputs(212, 213, 1.0, 144, 2.0);s.b[1583] = ((0.0 == 0.0) && (s.v[211] < ((-2500.0) * 0.5)));s.store_scalar(1583, if s.b[1583] { 1.0 } else { 0.0 });
        if s.b[1583] {s.store_div_from_scalar_scaled_input(13, ((-0.5) * 0.5), 211, 16.0);}
        if (!s.b[1583]) {s.store_scaled_add_mixed_ia(13, 211, A::sqrt_square_offset(s.ad_value(211), ((0.25 * 0.5) * 0.5)), 0.5);}
        s.b[1584] = ((0.0 == 0.0) && (s.v[212] < ((-2500.0) * 0.5)));s.store_scalar(1584, if s.b[1584] { 1.0 } else { 0.0 });
        if s.b[1584] {s.store_div_from_scalar_scaled_input(14, ((-0.5) * 0.5), 212, 16.0);}
        if (!s.b[1584]) {s.store_scaled_add_mixed_ia(14, 212, A::sqrt_square_offset(s.ad_value(212), ((0.25 * 0.5) * 0.5)), 0.5);}
        s.store_sqrt_offset_ad(214, A::mul(s.ad_value(13), s.ad_value(218)), 0.25);s.store_sqrt_offset_ad(215, A::mul(s.ad_value(14), s.ad_value(218)), 0.25);s.store_div_mixed_ia(13, 211, A::scale_offset(s.ad_value(214), 2.0, 1.0));s.store_div_mixed_ia(14, 212, A::scale_offset(s.ad_value(215), 2.0, 1.0));s.store_add(15, 214, 215);s.store_div_scaled_value_by_product_mixed_iai(16, 217, 0.3333333333333333, A::square(s.ad_value(15)), 15, 1.0);s.store_div_scaled_product3_mixed_iiia(17, 783, 162, 208, 1.0, A::add(A::offset(s.ad_value(200), 1.0), s.ad_value(144)), 1.0);s.store_mul_scale_offset_mixed_ia(18, 17, A::add_scaled_square_product(s.ad_value(15), 1.0, s.ad_value(214), s.ad_value(215), 1.0), 0.8, 0.0);s.store_add_scaled_inputs(19, 18, 1.0, 218, 2.0);s.store_scaled_mul(20, 217, 17, 0.3333333333333333);s.store_div_scaled_product_mixed_iaa(202, 212, A::scale_offset(s.ad_value(215), 2.0, (-1.0)), 1.0, A::scale_offset(s.ad_value(215), 2.0, 1.0), 1.0);s.store_add_mixed_ai(201, A::add_scaled_offset_product_lhs(s.ad_value(213), 1.0, s.ad_value(90), (-1.0), s.ad_value(144), (-2.0)), 202);s.store_add_scaled_products_mixed_iaii(189, 208, A::add_scaled_inputs3(s.ad_value(13), 1.0, s.ad_value(14), 1.0, A::add_scaled_products(s.ad_value(16), s.ad_value(19), 1.0, s.ad_value(90), A::add_scaled_inputs3(s.ad_value(200), 1.0, s.ad_value(144), 1.0, s.ad_value(20), 1.0), (-1.0)), 1.0), 1.0, 210, 201, 1.0);s.store_add(21, 200, 144);s.store_mul3_lhs(22, 217, 17, 17);
    }
}
