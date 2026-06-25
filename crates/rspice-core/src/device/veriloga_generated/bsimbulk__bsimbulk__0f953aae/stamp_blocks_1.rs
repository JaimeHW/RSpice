#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, ReactiveScratch, Scratch, THERMAL_VOLTAGE_PER_K};
use super::super::state::Instance;

impl Instance {

    pub(super) fn stamp_transient_block_16(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if (((s.v[1490] != 0.0) && (s.v[1493] != 0.0)) && (!(s.v[1495] != 0.0))) {
            s.store_offset_ad(13, A::limited_exp(A::div(A::scale(s.ad_value(12), p.p751), A::sub_from_scalar(p.p751, s.ad_value(309)))), (-1.0));
        }

        if (((s.v[1490] != 0.0) && (s.v[1493] != 0.0)) && (!(s.v[1495] != 0.0))) {
            s.store_sub_ad_rhs(305, 305, A::mul(s.ad_value(14), s.ad_value(13)));
        }

        s.store_mul(312, 423, 250);

        s.store_mul(315, 424, 300);

        s.store_scale(318, 428, (s.v[35] * p.p2));

        s.v[313] = ((0.1) as f64).powf((-p.p713));

        s.v[1496] = if (p.p713 == 1.0) { 1.0 } else { 0.0 };

        if (s.v[1496] != 0.0) {
            s.store_scalar(314, (1.5 - ((0.1) as f64).ln()));
        }

        if (!(s.v[1496] != 0.0)) {
            s.store_scalar(314, ((1.0 / (1.0 - p.p713)) * (1.0 - (((0.05 * p.p713) * (1.0 + p.p713)) * s.v[313]))));
        }

        s.v[316] = ((0.1) as f64).powf((-p.p715));

        s.v[1497] = if (p.p715 == 1.0) { 1.0 } else { 0.0 };

        if (s.v[1497] != 0.0) {
            s.store_scalar(317, (1.5 - ((0.1) as f64).ln()));
        }

        if (!(s.v[1497] != 0.0)) {
            s.store_scalar(317, ((1.0 / (1.0 - p.p715)) * (1.0 - (((0.05 * p.p715) * (1.0 + p.p715)) * s.v[316]))));
        }

        s.v[319] = ((0.1) as f64).powf((-p.p717));

        s.v[1498] = if (p.p717 == 1.0) { 1.0 } else { 0.0 };

        if (s.v[1498] != 0.0) {
            s.store_scalar(320, (1.5 - ((0.1) as f64).ln()));
        }

        if (!(s.v[1498] != 0.0)) {
            s.store_scalar(320, ((1.0 / (1.0 - p.p717)) * (1.0 - (((0.05 * p.p717) * (1.0 + p.p717)) * s.v[319]))));
        }

        s.v[1499] = if (s.v[312] > 0.0) { 1.0 } else { 0.0 };

        if (s.v[1499] != 0.0) {
            s.store_div(13, 306, 429);
        }

        s.v[1500] = if (s.v[13] < 0.9) { 1.0 } else { 0.0 };

        if ((s.v[1499] != 0.0) && (s.v[1500] != 0.0)) {
            s.store_sub_from_scalar(310, 1.0, 13);
        }

        s.v[1501] = if (p.p713 != 1.0) { 1.0 } else { 0.0 };

        s.v[1502] = if (p.p713 == 0.5) { 1.0 } else { 0.0 };

        if ((((s.v[1499] != 0.0) && (s.v[1500] != 0.0)) && (s.v[1501] != 0.0)) && (s.v[1502] != 0.0)) {
            s.store_div_from_scalar_ad(311, 1.0, A::sqrt(s.ad_value(310)));
        }

        if ((((s.v[1499] != 0.0) && (s.v[1500] != 0.0)) && (s.v[1501] != 0.0)) && (!(s.v[1502] != 0.0))) {
            s.store_limited_exp_ad(311, A::scale(A::ln(s.ad_value(310)), (-p.p713)));
        }

        if (((s.v[1499] != 0.0) && (s.v[1500] != 0.0)) && (s.v[1501] != 0.0)) {
            s.store_scale_ad(331, A::mul(A::mul(s.ad_value(429), s.ad_value(312)), A::sub_from_scalar(1.0, A::mul(s.ad_value(310), s.ad_value(311)))), 1.0 / ((1.0 - p.p713)));
        }

        if (((s.v[1499] != 0.0) && (s.v[1500] != 0.0)) && (!(s.v[1501] != 0.0))) {
            s.store_mul_ad(331, A::mul(s.ad_value(429), s.ad_value(312)), A::neg(A::ln(s.ad_value(310))));
        }

        if ((s.v[1499] != 0.0) && (!(s.v[1500] != 0.0))) {
            s.store_mul_ad(14, A::scale(A::offset(s.ad_value(13), (-1.0)), s.v[313]), A::offset(A::scale(A::offset(s.ad_value(13), (-1.0)), (5.0 * p.p713)), (1.0 + p.p713)));
        }

        if ((s.v[1499] != 0.0) && (!(s.v[1500] != 0.0))) {
            s.store_mul_ad(331, A::mul(s.ad_value(429), s.ad_value(312)), A::add(s.ad_value(14), s.ad_value(314)));
        }

        if (!(s.v[1499] != 0.0)) {
            s.store_scalar(331, 0.0);
        }

        s.v[1503] = if (s.v[315] > 0.0) { 1.0 } else { 0.0 };

        if (s.v[1503] != 0.0) {
            s.store_div(13, 306, 430);
        }

        s.v[1504] = if (s.v[13] < 0.9) { 1.0 } else { 0.0 };

        if ((s.v[1503] != 0.0) && (s.v[1504] != 0.0)) {
            s.store_sub_from_scalar(310, 1.0, 13);
        }

        s.v[1505] = if (p.p715 != 1.0) { 1.0 } else { 0.0 };

        s.v[1506] = if (p.p715 == 0.5) { 1.0 } else { 0.0 };

        if ((((s.v[1503] != 0.0) && (s.v[1504] != 0.0)) && (s.v[1505] != 0.0)) && (s.v[1506] != 0.0)) {
            s.store_div_from_scalar_ad(311, 1.0, A::sqrt(s.ad_value(310)));
        }

        if ((((s.v[1503] != 0.0) && (s.v[1504] != 0.0)) && (s.v[1505] != 0.0)) && (!(s.v[1506] != 0.0))) {
            s.store_limited_exp_ad(311, A::scale(A::ln(s.ad_value(310)), (-p.p715)));
        }

        if (((s.v[1503] != 0.0) && (s.v[1504] != 0.0)) && (s.v[1505] != 0.0)) {
            s.store_scale_ad(332, A::mul(A::mul(s.ad_value(430), s.ad_value(315)), A::sub_from_scalar(1.0, A::mul(s.ad_value(310), s.ad_value(311)))), 1.0 / ((1.0 - p.p715)));
        }

        if (((s.v[1503] != 0.0) && (s.v[1504] != 0.0)) && (!(s.v[1505] != 0.0))) {
            s.store_mul_ad(332, A::mul(s.ad_value(430), s.ad_value(315)), A::neg(A::ln(s.ad_value(310))));
        }

        if ((s.v[1503] != 0.0) && (!(s.v[1504] != 0.0))) {
            s.store_mul_ad(14, A::scale(A::offset(s.ad_value(13), (-1.0)), s.v[316]), A::offset(A::scale(A::offset(s.ad_value(13), (-1.0)), (5.0 * p.p715)), (1.0 + p.p715)));
        }

        if ((s.v[1503] != 0.0) && (!(s.v[1504] != 0.0))) {
            s.store_mul_ad(332, A::mul(s.ad_value(430), s.ad_value(315)), A::add(s.ad_value(14), s.ad_value(317)));
        }

        if (!(s.v[1503] != 0.0)) {
            s.store_scalar(332, 0.0);
        }

        s.v[1507] = if (s.v[318] > 0.0) { 1.0 } else { 0.0 };

        if (s.v[1507] != 0.0) {
            s.store_div(13, 306, 431);
        }

        s.v[1508] = if (s.v[13] < 0.9) { 1.0 } else { 0.0 };

        if ((s.v[1507] != 0.0) && (s.v[1508] != 0.0)) {
            s.store_sub_from_scalar(310, 1.0, 13);
        }

        s.v[1509] = if (p.p717 != 1.0) { 1.0 } else { 0.0 };

        s.v[1510] = if (p.p717 == 0.5) { 1.0 } else { 0.0 };

        if ((((s.v[1507] != 0.0) && (s.v[1508] != 0.0)) && (s.v[1509] != 0.0)) && (s.v[1510] != 0.0)) {
            s.store_div_from_scalar_ad(311, 1.0, A::sqrt(s.ad_value(310)));
        }

        if ((((s.v[1507] != 0.0) && (s.v[1508] != 0.0)) && (s.v[1509] != 0.0)) && (!(s.v[1510] != 0.0))) {
            s.store_limited_exp_ad(311, A::scale(A::ln(s.ad_value(310)), (-p.p717)));
        }

        if (((s.v[1507] != 0.0) && (s.v[1508] != 0.0)) && (s.v[1509] != 0.0)) {
            s.store_scale_ad(333, A::mul(A::mul(s.ad_value(431), s.ad_value(318)), A::sub_from_scalar(1.0, A::mul(s.ad_value(310), s.ad_value(311)))), 1.0 / ((1.0 - p.p717)));
        }

        if (((s.v[1507] != 0.0) && (s.v[1508] != 0.0)) && (!(s.v[1509] != 0.0))) {
            s.store_mul_ad(333, A::mul(s.ad_value(431), s.ad_value(318)), A::neg(A::ln(s.ad_value(310))));
        }

        if ((s.v[1507] != 0.0) && (!(s.v[1508] != 0.0))) {
            s.store_mul_ad(14, A::scale(A::offset(s.ad_value(13), (-1.0)), s.v[319]), A::offset(A::scale(A::offset(s.ad_value(13), (-1.0)), (5.0 * p.p717)), (1.0 + p.p717)));
        }

        if ((s.v[1507] != 0.0) && (!(s.v[1508] != 0.0))) {
            s.store_mul_ad(333, A::mul(s.ad_value(431), s.ad_value(318)), A::add(s.ad_value(14), s.ad_value(320)));
        }

        if (!(s.v[1507] != 0.0)) {
            s.store_scalar(333, 0.0);
        }

        s.store_add_ad_lhs(330, A::add(s.ad_value(331), s.ad_value(332)), 333);

        s.store_mul_ad_lhs(321, A::mul(s.ad_value(302), s.ad_value(426)), 251);

        s.v[1511] = if (s.v[301] > (s.v[35] * p.p2)) { 1.0 } else { 0.0 };

        s.v[1512] = if ((p.p1128 > 0.0) && (p.p1097 == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1511] != 0.0) && (s.v[1512] != 0.0)) {
            s.store_mul_ad(324, A::mul(s.ad_value(302), s.ad_value(427)), A::offset(s.ad_value(301), (-(s.v[35] * p.p2))));
        }

        if ((s.v[1511] != 0.0) && (!(s.v[1512] != 0.0))) {
            s.store_mul_ad_lhs(324, A::mul(s.ad_value(302), s.ad_value(427)), 301);
        }

        if (!(s.v[1511] != 0.0)) {
            s.store_mul_ad_lhs(324, A::mul(s.ad_value(302), s.ad_value(427)), 301);
        }

        s.store_scale(327, 425, (s.v[35] * p.p2));

        s.v[322] = ((0.1) as f64).powf((-p.p714));

        s.v[1513] = if (p.p714 == 1.0) { 1.0 } else { 0.0 };

        if (s.v[1513] != 0.0) {
            s.store_scalar(323, (1.5 - ((0.1) as f64).ln()));
        }

        if (!(s.v[1513] != 0.0)) {
            s.store_scalar(323, ((1.0 / (1.0 - p.p714)) * (1.0 - (((0.05 * p.p714) * (1.0 + p.p714)) * s.v[322]))));
        }

        s.v[325] = ((0.1) as f64).powf((-p.p716));

        s.v[1514] = if (p.p716 == 1.0) { 1.0 } else { 0.0 };

        if (s.v[1514] != 0.0) {
            s.store_scalar(326, (1.5 - ((0.1) as f64).ln()));
        }

        if (!(s.v[1514] != 0.0)) {
            s.store_scalar(326, ((1.0 / (1.0 - p.p716)) * (1.0 - (((0.05 * p.p716) * (1.0 + p.p716)) * s.v[325]))));
        }

        s.v[328] = ((0.1) as f64).powf((-p.p718));

        s.v[1515] = if (p.p718 == 1.0) { 1.0 } else { 0.0 };

        if (s.v[1515] != 0.0) {
            s.store_scalar(329, (1.5 - ((0.1) as f64).ln()));
        }

        if (!(s.v[1515] != 0.0)) {
            s.store_scalar(329, ((1.0 / (1.0 - p.p718)) * (1.0 - (((0.05 * p.p718) * (1.0 + p.p718)) * s.v[328]))));
        }

        s.v[1516] = if (s.v[321] > 0.0) { 1.0 } else { 0.0 };

        if (s.v[1516] != 0.0) {
            s.store_div(13, 308, 432);
        }

        s.v[1517] = if (s.v[13] < 0.9) { 1.0 } else { 0.0 };

        if ((s.v[1516] != 0.0) && (s.v[1517] != 0.0)) {
            s.store_sub_from_scalar(310, 1.0, 13);
        }

        s.v[1518] = if (p.p714 != 1.0) { 1.0 } else { 0.0 };

        s.v[1519] = if (p.p714 == 0.5) { 1.0 } else { 0.0 };

        if ((((s.v[1516] != 0.0) && (s.v[1517] != 0.0)) && (s.v[1518] != 0.0)) && (s.v[1519] != 0.0)) {
            s.store_div_from_scalar_ad(311, 1.0, A::sqrt(s.ad_value(310)));
        }

        if ((((s.v[1516] != 0.0) && (s.v[1517] != 0.0)) && (s.v[1518] != 0.0)) && (!(s.v[1519] != 0.0))) {
            s.store_limited_exp_ad(311, A::scale(A::ln(s.ad_value(310)), (-p.p714)));
        }

        if (((s.v[1516] != 0.0) && (s.v[1517] != 0.0)) && (s.v[1518] != 0.0)) {
            s.store_scale_ad(335, A::mul(A::mul(s.ad_value(432), s.ad_value(321)), A::sub_from_scalar(1.0, A::mul(s.ad_value(310), s.ad_value(311)))), 1.0 / ((1.0 - p.p714)));
        }

        if (((s.v[1516] != 0.0) && (s.v[1517] != 0.0)) && (!(s.v[1518] != 0.0))) {
            s.store_mul_ad(335, A::mul(s.ad_value(432), s.ad_value(321)), A::neg(A::ln(s.ad_value(310))));
        }

        if ((s.v[1516] != 0.0) && (!(s.v[1517] != 0.0))) {
            s.store_mul_ad(14, A::scale(A::offset(s.ad_value(13), (-1.0)), s.v[322]), A::offset(A::scale(A::offset(s.ad_value(13), (-1.0)), (5.0 * p.p714)), (1.0 + p.p714)));
        }

        if ((s.v[1516] != 0.0) && (!(s.v[1517] != 0.0))) {
            s.store_mul_ad(335, A::mul(s.ad_value(432), s.ad_value(321)), A::add(s.ad_value(14), s.ad_value(323)));
        }

        if (!(s.v[1516] != 0.0)) {
            s.store_scalar(335, 0.0);
        }

        s.v[1520] = if (s.v[324] > 0.0) { 1.0 } else { 0.0 };

        if (s.v[1520] != 0.0) {
            s.store_div(13, 308, 433);
        }

        s.v[1521] = if (s.v[13] < 0.9) { 1.0 } else { 0.0 };

        if ((s.v[1520] != 0.0) && (s.v[1521] != 0.0)) {
            s.store_sub_from_scalar(310, 1.0, 13);
        }

        s.v[1522] = if (p.p716 != 1.0) { 1.0 } else { 0.0 };

        s.v[1523] = if (p.p716 == 0.5) { 1.0 } else { 0.0 };

        if ((((s.v[1520] != 0.0) && (s.v[1521] != 0.0)) && (s.v[1522] != 0.0)) && (s.v[1523] != 0.0)) {
            s.store_div_from_scalar_ad(311, 1.0, A::sqrt(s.ad_value(310)));
        }

        if ((((s.v[1520] != 0.0) && (s.v[1521] != 0.0)) && (s.v[1522] != 0.0)) && (!(s.v[1523] != 0.0))) {
            s.store_limited_exp_ad(311, A::scale(A::ln(s.ad_value(310)), (-p.p716)));
        }

        if (((s.v[1520] != 0.0) && (s.v[1521] != 0.0)) && (s.v[1522] != 0.0)) {
            s.store_scale_ad(336, A::mul(A::mul(s.ad_value(433), s.ad_value(324)), A::sub_from_scalar(1.0, A::mul(s.ad_value(310), s.ad_value(311)))), 1.0 / ((1.0 - p.p716)));
        }

        if (((s.v[1520] != 0.0) && (s.v[1521] != 0.0)) && (!(s.v[1522] != 0.0))) {
            s.store_mul_ad(336, A::mul(s.ad_value(433), s.ad_value(324)), A::neg(A::ln(s.ad_value(310))));
        }

        if ((s.v[1520] != 0.0) && (!(s.v[1521] != 0.0))) {
            s.store_mul_ad(14, A::scale(A::offset(s.ad_value(13), (-1.0)), s.v[325]), A::offset(A::scale(A::offset(s.ad_value(13), (-1.0)), (5.0 * p.p716)), (1.0 + p.p716)));
        }

        if ((s.v[1520] != 0.0) && (!(s.v[1521] != 0.0))) {
            s.store_mul_ad(336, A::mul(s.ad_value(433), s.ad_value(324)), A::add(s.ad_value(14), s.ad_value(326)));
        }

        if (!(s.v[1520] != 0.0)) {
            s.store_scalar(336, 0.0);
        }

        s.v[1524] = if (s.v[327] > 0.0) { 1.0 } else { 0.0 };

        if (s.v[1524] != 0.0) {
            s.store_div(13, 308, 434);
        }

        s.v[1525] = if (s.v[13] < 0.9) { 1.0 } else { 0.0 };

        if ((s.v[1524] != 0.0) && (s.v[1525] != 0.0)) {
            s.store_sub_from_scalar(310, 1.0, 13);
        }

        s.v[1526] = if (p.p718 != 1.0) { 1.0 } else { 0.0 };

        s.v[1527] = if (p.p718 == 0.5) { 1.0 } else { 0.0 };

        if ((((s.v[1524] != 0.0) && (s.v[1525] != 0.0)) && (s.v[1526] != 0.0)) && (s.v[1527] != 0.0)) {
            s.store_div_from_scalar_ad(311, 1.0, A::sqrt(s.ad_value(310)));
        }

        if ((((s.v[1524] != 0.0) && (s.v[1525] != 0.0)) && (s.v[1526] != 0.0)) && (!(s.v[1527] != 0.0))) {
            s.store_limited_exp_ad(311, A::scale(A::ln(s.ad_value(310)), (-p.p718)));
        }

        if (((s.v[1524] != 0.0) && (s.v[1525] != 0.0)) && (s.v[1526] != 0.0)) {
            s.store_scale_ad(337, A::mul(A::mul(s.ad_value(434), s.ad_value(327)), A::sub_from_scalar(1.0, A::mul(s.ad_value(310), s.ad_value(311)))), 1.0 / ((1.0 - p.p718)));
        }

        if (((s.v[1524] != 0.0) && (s.v[1525] != 0.0)) && (!(s.v[1526] != 0.0))) {
            s.store_mul_ad(337, A::mul(s.ad_value(434), s.ad_value(327)), A::neg(A::ln(s.ad_value(310))));
        }

        if ((s.v[1524] != 0.0) && (!(s.v[1525] != 0.0))) {
            s.store_mul_ad(14, A::scale(A::offset(s.ad_value(13), (-1.0)), s.v[328]), A::offset(A::scale(A::offset(s.ad_value(13), (-1.0)), (5.0 * p.p718)), (1.0 + p.p718)));
        }

        if ((s.v[1524] != 0.0) && (!(s.v[1525] != 0.0))) {
            s.store_mul_ad(337, A::mul(s.ad_value(434), s.ad_value(327)), A::add(s.ad_value(14), s.ad_value(329)));
        }

        if (!(s.v[1524] != 0.0)) {
            s.store_scalar(337, 0.0);
        }

        s.store_add_ad_lhs(334, A::add(s.ad_value(335), s.ad_value(336)), 337);

        s.v[1528] = if ((p.p1128 > 0.0) && (p.p1097 == 1.0)) { 1.0 } else { 0.0 };

        if (s.v[1528] != 0.0) {
            s.store_mul_ad_lhs(321, A::scale(s.ad_value(426), p.p1128), 251);
        }

        s.v[1529] = if (s.v[301] > (s.v[35] * p.p2)) { 1.0 } else { 0.0 };

        if ((s.v[1528] != 0.0) && (s.v[1529] != 0.0)) {
            s.store_mul_ad_rhs(324, 427, A::offset(A::scale(A::offset(s.ad_value(301), (-(s.v[35] * p.p2))), p.p1128), (s.v[35] * p.p2)));
        }

        if ((s.v[1528] != 0.0) && (!(s.v[1529] != 0.0))) {
            s.store_mul_ad_lhs(324, A::scale(s.ad_value(427), p.p1128), 301);
        }

        s.v[1530] = if (s.v[321] > 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1528] != 0.0) && (s.v[1530] != 0.0)) {
            s.store_div(13, 309, 432);
        }

        s.v[1531] = if (s.v[13] < 0.9) { 1.0 } else { 0.0 };

        if (((s.v[1528] != 0.0) && (s.v[1530] != 0.0)) && (s.v[1531] != 0.0)) {
            s.store_sub_from_scalar(310, 1.0, 13);
        }

        s.v[1532] = if (p.p714 != 1.0) { 1.0 } else { 0.0 };

        s.v[1533] = if (p.p714 == 0.5) { 1.0 } else { 0.0 };

        if (((((s.v[1528] != 0.0) && (s.v[1530] != 0.0)) && (s.v[1531] != 0.0)) && (s.v[1532] != 0.0)) && (s.v[1533] != 0.0)) {
            s.store_div_from_scalar_ad(311, 1.0, A::sqrt(s.ad_value(310)));
        }

        if (((((s.v[1528] != 0.0) && (s.v[1530] != 0.0)) && (s.v[1531] != 0.0)) && (s.v[1532] != 0.0)) && (!(s.v[1533] != 0.0))) {
            s.store_limited_exp_ad(311, A::scale(A::ln(s.ad_value(310)), (-p.p714)));
        }

        if ((((s.v[1528] != 0.0) && (s.v[1530] != 0.0)) && (s.v[1531] != 0.0)) && (s.v[1532] != 0.0)) {
            s.store_scale_ad(339, A::mul(A::mul(s.ad_value(432), s.ad_value(321)), A::sub_from_scalar(1.0, A::mul(s.ad_value(310), s.ad_value(311)))), 1.0 / ((1.0 - p.p714)));
        }

        if ((((s.v[1528] != 0.0) && (s.v[1530] != 0.0)) && (s.v[1531] != 0.0)) && (!(s.v[1532] != 0.0))) {
            s.store_mul_ad(339, A::mul(s.ad_value(432), s.ad_value(321)), A::neg(A::ln(s.ad_value(310))));
        }

        if (((s.v[1528] != 0.0) && (s.v[1530] != 0.0)) && (!(s.v[1531] != 0.0))) {
            s.store_mul_ad(14, A::scale(A::offset(s.ad_value(13), (-1.0)), s.v[322]), A::offset(A::scale(A::offset(s.ad_value(13), (-1.0)), (5.0 * p.p714)), (1.0 + p.p714)));
        }

        if (((s.v[1528] != 0.0) && (s.v[1530] != 0.0)) && (!(s.v[1531] != 0.0))) {
            s.store_mul_ad(339, A::mul(s.ad_value(432), s.ad_value(321)), A::add(s.ad_value(14), s.ad_value(323)));
        }

        if ((s.v[1528] != 0.0) && (!(s.v[1530] != 0.0))) {
            s.store_scalar(339, 0.0);
        }

        s.v[1534] = if (s.v[324] > 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1528] != 0.0) && (s.v[1534] != 0.0)) {
            s.store_div(13, 309, 433);
        }

        s.v[1535] = if (s.v[13] < 0.9) { 1.0 } else { 0.0 };

        if (((s.v[1528] != 0.0) && (s.v[1534] != 0.0)) && (s.v[1535] != 0.0)) {
            s.store_sub_from_scalar(310, 1.0, 13);
        }

        s.v[1536] = if (p.p716 != 1.0) { 1.0 } else { 0.0 };

        s.v[1537] = if (p.p716 == 0.5) { 1.0 } else { 0.0 };

        if (((((s.v[1528] != 0.0) && (s.v[1534] != 0.0)) && (s.v[1535] != 0.0)) && (s.v[1536] != 0.0)) && (s.v[1537] != 0.0)) {
            s.store_div_from_scalar_ad(311, 1.0, A::sqrt(s.ad_value(310)));
        }

        if (((((s.v[1528] != 0.0) && (s.v[1534] != 0.0)) && (s.v[1535] != 0.0)) && (s.v[1536] != 0.0)) && (!(s.v[1537] != 0.0))) {
            s.store_limited_exp_ad(311, A::scale(A::ln(s.ad_value(310)), (-p.p716)));
        }

        if ((((s.v[1528] != 0.0) && (s.v[1534] != 0.0)) && (s.v[1535] != 0.0)) && (s.v[1536] != 0.0)) {
            s.store_scale_ad(340, A::mul(A::mul(s.ad_value(433), s.ad_value(324)), A::sub_from_scalar(1.0, A::mul(s.ad_value(310), s.ad_value(311)))), 1.0 / ((1.0 - p.p716)));
        }

        if ((((s.v[1528] != 0.0) && (s.v[1534] != 0.0)) && (s.v[1535] != 0.0)) && (!(s.v[1536] != 0.0))) {
            s.store_mul_ad(340, A::mul(s.ad_value(433), s.ad_value(324)), A::neg(A::ln(s.ad_value(310))));
        }

        if (((s.v[1528] != 0.0) && (s.v[1534] != 0.0)) && (!(s.v[1535] != 0.0))) {
            s.store_mul_ad(14, A::scale(A::offset(s.ad_value(13), (-1.0)), s.v[325]), A::offset(A::scale(A::offset(s.ad_value(13), (-1.0)), (5.0 * p.p716)), (1.0 + p.p716)));
        }

        if (((s.v[1528] != 0.0) && (s.v[1534] != 0.0)) && (!(s.v[1535] != 0.0))) {
            s.store_mul_ad(340, A::mul(s.ad_value(433), s.ad_value(324)), A::add(s.ad_value(14), s.ad_value(326)));
        }

        if ((s.v[1528] != 0.0) && (!(s.v[1534] != 0.0))) {
            s.store_scalar(340, 0.0);
        }

        if (s.v[1528] != 0.0) {
            s.store_add(338, 339, 340);
        }

        if (!(s.v[1528] != 0.0)) {
            s.store_scalar(338, 0.0);
        }

        s.v[1538] = if (p.p38 != 0.0) { 1.0 } else { 0.0 };

        if (s.v[1538] != 0.0) {
            s.store_powf_ad(13, A::scale(s.ad_value(481), 1.0000000000000001e-23), p.p954);
        }

        if (s.v[1538] != 0.0) {
            s.store_powf_ad(14, A::div_from_scalar(300.0, s.ad_value(391)), p.p955);
        }

        if (s.v[1538] != 0.0) {
            s.store_div_ad_lhs(15, A::mul(A::scale(s.ad_value(187), p.p953), A::voltage(ctx, &nodes, Some(11), Some(7))), 108);
        }

        if (s.v[1538] != 0.0) {
            s.store_scale_ad(707, A::limited_exp(A::mul(A::neg(s.ad_value(13)), s.ad_value(14))), p.p948);
        }

        if (s.v[1538] != 0.0) {
            s.store_mul_ad_lhs(708, A::scale(s.ad_value(14), p.p949), 13);
        }

        if (s.v[1538] != 0.0) {
            s.store_scale_ad(709, A::tanh(A::limited_exp(A::mul(A::scale(s.ad_value(187), p.p952), A::sub(A::sub(A::voltage(ctx, &nodes, Some(9), Some(11)), s.ad_value(857)), A::voltage(ctx, &nodes, Some(7), Some(11)))))), p.p951);
        }

    }

    pub(super) fn stamp_transient_block_17(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if (s.v[1538] != 0.0) {
            let assign27280_ad_e36587: A = A::mul(A::mul(A::mul(A::mul(A::mul(A::scale(s.ad_value(57), (p.p2 * s.v[29])), s.ad_value(707)), A::limited_exp(s.ad_value(15))), A::limited_exp(A::scale(A::neg(s.ad_value(708)), s.v[30]))), A::limited_exp(A::div(s.ad_value(709), s.ad_value(108)))), A::offset(A::limited_exp(A::div(A::scale(s.ad_value(76), p.p950), s.ad_value(108))), (-1.0)));
            s.store_ad(706, &assign27280_ad_e36587);
        }

        s.store_scale(377, 108, (4.0 * 1.60219e-19));

        s.store_div_ad_lhs(360, A::scale(s.ad_value(502), 2.0), 157);

        s.v[1539] = if (p.p784 <= 0.0) { 1.0 } else { 0.0 };

        if (s.v[1539] != 0.0) {
            s.store_scalar(363, 0.0);
        }

        if (!(s.v[1539] != 0.0)) {
            s.store_div_ad_lhs(12, A::offset(A::div(s.ad_value(167), s.ad_value(129)), p.p784), 360);
        }

        if (!(s.v[1539] != 0.0)) {
            s.store_mul_ad_rhs(363, 129, A::ln(A::max_with_scalar(s.ad_value(12), 1e-38)));
        }

        s.v[1540] = if (s.v[363] < 0.0) { 1.0 } else { 0.0 };

        if ((!(s.v[1539] != 0.0)) && (s.v[1540] != 0.0)) {
            s.store_scalar(363, 0.0);
        }

        s.store_mul_ad(367, A::scale(s.ad_value(108), 6.241457005723417e18), A::add(A::offset(s.ad_value(97), s.v[46]), s.ad_value(483)));

        s.store_scale_ad(366, A::mul(A::mul(A::mul(A::mul(A::scale(s.ad_value(90), (2.0 * s.v[46])), s.ad_value(108)), s.ad_value(144)), s.ad_value(628)), s.ad_value(611)), 6.241457005723417e18);

        s.store_mul_ad_lhs(736, A::mul(A::scale(s.ad_value(108), ((1.60219e-19 * 1.60219e-19) * 1.60219e-19)), A::abs(s.ad_value(188))), 157);

        s.store_mul_ad_lhs(737, A::mul(A::scale(s.ad_value(108), 1.60219e-19), s.ad_value(188)), 188);

        s.store_add_ad(738, A::offset(A::scale(s.ad_value(366), p.p799), p.p785), A::mul(A::scale(s.ad_value(366), p.p800), s.ad_value(366)));

        s.store_mul_ad(739, A::add(s.ad_value(366), s.ad_value(367)), A::add(s.ad_value(366), s.ad_value(367)));

        s.store_scale(740, 108, (p.p785 * 1.60219e-19));

        s.v[1541] = if (p.p1065 == 1.0) { 1.0 } else { 0.0 };

        if (s.v[1541] != 0.0) {
            s.store_scalar(745, s.v[30]);
        }

        if (s.v[1541] != 0.0) {
            s.store_div_ad_lhs(712, A::sub(s.ad_value(64), s.ad_value(482)), 108);
        }

        if (s.v[1541] != 0.0) {
            s.store_scale_ad(713, A::sqrt(A::div_from_scalar((((2.0 * 1.60219e-19) * s.v[26]) * p.p1068), s.ad_value(108))), 1.0 / (s.v[46]));
        }

        if (s.v[1541] != 0.0) {
            s.store_ln_ad(714, A::div_from_scalar(p.p1068, s.ad_value(28)));
        }

        if (s.v[1541] != 0.0) {
            s.store_scalar(13, 1.0);
        }

        if (s.v[1541] != 0.0) {
            s.store_div(204, 712, 13);
        }

        if (s.v[1541] != 0.0) {
            s.store_div(205, 713, 13);
        }

        if (s.v[1541] != 0.0) {
            s.store_sub_ad(13, A::scale(s.ad_value(204), 0.5), A::scale(A::offset(A::scale(s.ad_value(205), 0.7071067811865475), 1.0), 3.0));
        }

        if (s.v[1541] != 0.0) {
            s.store_add_ad_rhs(14, 13, A::sqrt(A::add(A::square(s.ad_value(13)), A::scale(s.ad_value(204), 6.0))));
        }

        s.v[1542] = if (s.v[204] < 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1541] != 0.0) && (s.v[1542] != 0.0)) {
            s.store_div_ad_lhs(15, A::sub(s.ad_value(204), s.ad_value(14)), 205);
        }

        if ((s.v[1541] != 0.0) && (s.v[1542] != 0.0)) {
            s.store_neg_ad(715, A::ln(A::max_with_scalar(A::add(A::sub_from_scalar(1.0, s.ad_value(14)), A::square(s.ad_value(15))), 1e-38)));
        }

        if ((s.v[1541] != 0.0) && (!(s.v[1542] != 0.0))) {
            s.store_limited_exp_ad(15, A::neg(s.ad_value(14)));
        }

        if ((s.v[1541] != 0.0) && (!(s.v[1542] != 0.0))) {
            s.store_scale(13, 205, 0.5);
        }

        if ((s.v[1541] != 0.0) && (!(s.v[1542] != 0.0))) {
            s.store_sub_ad_lhs(14, A::sqrt(A::add(A::add(A::offset(s.ad_value(204), (-1.0)), s.ad_value(15)), A::square(s.ad_value(13)))), 13);
        }

        if ((s.v[1541] != 0.0) && (!(s.v[1542] != 0.0))) {
            s.store_sub_ad_lhs(715, A::offset(A::square(s.ad_value(14)), 1.0), 15);
        }

        if (s.v[1541] != 0.0) {
            s.store_scale_ad(20, A::add(A::offset(s.ad_value(715), 1.0), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(715), (-1.0)), A::offset(s.ad_value(715), (-1.0))), ((0.25 * 2.0) * 2.0)))), 0.5);
        }

        if (s.v[1541] != 0.0) {
            s.store_sqrt(96, 20);
        }

        if (s.v[1541] != 0.0) {
            s.store_div_ad_lhs(12, A::offset(A::div(s.ad_value(713), A::scale(s.ad_value(96), 2.0)), 1.0), 713);
        }

        if (s.v[1541] != 0.0) {
            s.store_sub_ad_lhs(13, A::sub(s.ad_value(715), A::scale(s.ad_value(714), 2.0)), 73);
        }

        if (s.v[1541] != 0.0) {
            s.store_sub_ad_rhs(14, 13, A::ln(A::max_with_scalar(A::mul(A::scale(s.ad_value(12), 4.0), s.ad_value(96)), 1e-38)));
        }

        if (s.v[1541] != 0.0) {
            s.store_scale_ad(20, A::sub(A::offset(s.ad_value(14), (-0.201491)), A::sqrt(A::offset(A::mul(s.ad_value(14), A::offset(s.ad_value(14), 0.402982)), 2.446562))), 0.5);
        }

        if (s.v[1541] != 0.0) {
            s.copy_ad(94, 96);
        }

        s.v[1543] = if (s.v[20] <= (-68.0)) { 1.0 } else { 0.0 };

        if ((s.v[1541] != 0.0) && (s.v[1543] != 0.0)) {
            s.store_scalar(16, (-100.0));
        }

        if ((s.v[1541] != 0.0) && (s.v[1543] != 0.0)) {
            s.store_scalar(17, 20.0);
        }

        s.v[1544] = if (s.v[20] < (s.v[16] - (0.5 * s.v[17]))) { 1.0 } else { 0.0 };

        if (((s.v[1541] != 0.0) && (s.v[1543] != 0.0)) && (s.v[1544] != 0.0)) {
            s.store_ad(15, &A::limited_exp(s.ad_value(16)));
        }

        s.v[1545] = if (s.v[20] > (s.v[16] + (0.5 * s.v[17]))) { 1.0 } else { 0.0 };

        if ((((s.v[1541] != 0.0) && (s.v[1543] != 0.0)) && (!(s.v[1544] != 0.0))) && (s.v[1545] != 0.0)) {
            s.store_ad(15, &A::limited_exp(s.ad_value(20)));
        }

        if ((((s.v[1541] != 0.0) && (s.v[1543] != 0.0)) && (!(s.v[1544] != 0.0))) && (!(s.v[1545] != 0.0))) {
            s.store_div_ad_lhs(14, A::sub(s.ad_value(20), s.ad_value(16)), 17);
        }

        if ((((s.v[1541] != 0.0) && (s.v[1543] != 0.0)) && (!(s.v[1544] != 0.0))) && (!(s.v[1545] != 0.0))) {
            s.store_square(18, 14);
        }

        if ((((s.v[1541] != 0.0) && (s.v[1543] != 0.0)) && (!(s.v[1544] != 0.0))) && (!(s.v[1545] != 0.0))) {
            s.store_limited_exp_ad(15, A::add(s.ad_value(16), A::mul(s.ad_value(17), A::add(A::offset(A::scale(s.ad_value(14), 0.5), (5.0 / 64.0)), A::mul(s.ad_value(18), A::sub_from_scalar((15.0 / 16.0), A::mul(s.ad_value(18), A::sub_from_scalar(1.25, s.ad_value(18)))))))));
        }

        if ((s.v[1541] != 0.0) && (s.v[1543] != 0.0)) {
            s.store_mul_ad_rhs(717, 15, A::sub(A::sub(A::offset(s.ad_value(13), 1.0), s.ad_value(20)), A::ln(A::max_with_scalar(A::mul(A::scale(s.ad_value(12), 2.0), A::add(A::mul(A::scale(s.ad_value(15), 2.0), s.ad_value(12)), A::scale(s.ad_value(94), 2.0))), 1e-38))));
        }

        if ((s.v[1541] != 0.0) && (!(s.v[1543] != 0.0))) {
            s.store_ad(15, &A::limited_exp(s.ad_value(20)));
        }

        if ((s.v[1541] != 0.0) && (!(s.v[1543] != 0.0))) {
            s.store_div_from_scalar(95, 1.0, 94);
        }

        if ((s.v[1541] != 0.0) && (!(s.v[1543] != 0.0))) {
            s.store_sub_ad_lhs(16, A::add(A::scale(s.ad_value(15), 2.0), A::ln(A::max_with_scalar(A::mul(A::mul(A::scale(s.ad_value(15), 2.0), s.ad_value(12)), A::add(A::mul(A::scale(s.ad_value(15), 2.0), s.ad_value(12)), A::scale(s.ad_value(94), 2.0))), 1e-38))), 13);
        }

        if ((s.v[1541] != 0.0) && (!(s.v[1543] != 0.0))) {
            s.store_add_ad(17, A::offset(A::div_from_scalar(1.0, s.ad_value(15)), 2.0), A::div(A::add(s.ad_value(12), s.ad_value(95)), A::add(A::mul(s.ad_value(12), s.ad_value(15)), s.ad_value(94))));
        }

        if ((s.v[1541] != 0.0) && (!(s.v[1543] != 0.0))) {
            s.store_sub_ad_rhs(15, 15, A::div(s.ad_value(16), s.ad_value(17)));
        }

        if ((s.v[1541] != 0.0) && (!(s.v[1543] != 0.0))) {
            s.store_sub_ad_lhs(16, A::add(A::scale(s.ad_value(15), 2.0), A::ln(A::max_with_scalar(A::mul(A::mul(A::scale(s.ad_value(15), 2.0), s.ad_value(12)), A::add(A::mul(A::scale(s.ad_value(15), 2.0), s.ad_value(12)), A::scale(s.ad_value(94), 2.0))), 1e-38))), 13);
        }

        if ((s.v[1541] != 0.0) && (!(s.v[1543] != 0.0))) {
            s.store_add_ad(17, A::offset(A::div_from_scalar(1.0, s.ad_value(15)), 2.0), A::div(A::add(s.ad_value(12), s.ad_value(95)), A::add(A::mul(s.ad_value(12), s.ad_value(15)), s.ad_value(94))));
        }

        if ((s.v[1541] != 0.0) && (!(s.v[1543] != 0.0))) {
            s.store_mul_ad(18, A::div(A::add(s.ad_value(12), s.ad_value(95)), A::add(A::mul(s.ad_value(12), s.ad_value(15)), s.ad_value(94))), A::div(A::add(s.ad_value(12), s.ad_value(95)), A::add(A::mul(s.ad_value(12), s.ad_value(15)), s.ad_value(94))));
        }

        if ((s.v[1541] != 0.0) && (!(s.v[1543] != 0.0))) {
            s.store_sub_ad_lhs(19, A::sub(A::scale(A::mul(A::div_from_scalar(1.0, s.ad_value(15)), A::div_from_scalar(1.0, s.ad_value(15))), (-1.0)), A::div_from_scalar(1.0, A::mul(A::mul(A::square(s.ad_value(94)), s.ad_value(94)), A::add(A::mul(s.ad_value(12), s.ad_value(15)), s.ad_value(94))))), 18);
        }

        if ((s.v[1541] != 0.0) && (!(s.v[1543] != 0.0))) {
            s.store_sub_ad_rhs(717, 15, A::mul(A::div(s.ad_value(16), s.ad_value(17)), A::offset(A::div(A::mul(s.ad_value(16), s.ad_value(19)), A::mul(A::scale(s.ad_value(17), 2.0), s.ad_value(17))), 1.0)));
        }

        s.v[1546] = if ((1.0 == 0.0) && (s.v[715] < ((-2500.0) * 2.0))) { 1.0 } else { 0.0 };

        if ((s.v[1541] != 0.0) && (s.v[1546] != 0.0)) {
            s.store_div_from_scalar_ad(716, ((-2.0) * 2.0), A::scale(s.ad_value(715), 16.0));
        }

        if ((s.v[1541] != 0.0) && (!(s.v[1546] != 0.0))) {
            s.store_scale_ad(716, A::add(A::offset(s.ad_value(715), 1.0), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(715), (-1.0)), A::offset(s.ad_value(715), (-1.0))), ((0.25 * 2.0) * 2.0)))), 0.5);
        }

        if (s.v[1541] != 0.0) {
            s.store_offset_ad(718, A::div(s.ad_value(713), A::scale(A::sqrt(s.ad_value(716)), 2.0)), 1.0);
        }

        if (s.v[1541] != 0.0) {
            s.copy_ad(719, 157);
        }

        if (s.v[1541] != 0.0) {
            s.store_scale(726, 719, (s.v[46] * s.v[29]));
        }

        if (s.v[1541] != 0.0) {
            s.store_scale(725, 157, (s.v[46] * s.v[29]));
        }

        if (s.v[1541] != 0.0) {
            s.store_div_ad(720, A::mul(s.ad_value(188), s.ad_value(746)), A::mul(A::mul(A::mul(A::scale(s.ad_value(718), 2.0), s.ad_value(726)), s.ad_value(108)), s.ad_value(108)));
        }

        if (s.v[1541] != 0.0) {
            s.store_div_ad(722, A::mul(s.ad_value(188), A::sub(s.ad_value(745), s.ad_value(746))), A::mul(A::mul(A::mul(A::scale(s.ad_value(90), 2.0), s.ad_value(725)), s.ad_value(106)), s.ad_value(106)));
        }

        if (s.v[1541] != 0.0) {
            s.store_offset_ad(12, A::scale(A::sub(A::add(A::square(s.ad_value(717)), s.ad_value(717)), s.ad_value(720)), 4.0), 1.0);
        }

        s.v[1547] = if (s.v[12] < 1.0) { 1.0 } else { 0.0 };

        if ((s.v[1541] != 0.0) && (s.v[1547] != 0.0)) {
            s.store_scalar(721, 0.0);
        }

        if ((s.v[1541] != 0.0) && (!(s.v[1547] != 0.0))) {
            s.store_offset_ad(721, A::scale(A::sqrt(s.ad_value(12)), 0.5), (-0.5));
        }

        if (s.v[1541] != 0.0) {
            s.store_offset_ad(723, A::scale(A::sqrt(A::offset(A::scale(A::add(A::add(A::square(s.ad_value(144)), s.ad_value(144)), s.ad_value(722)), 4.0), 1.0)), 0.5), (-0.5));
        }

        if (s.v[1541] != 0.0) {
            s.store_mul_ad_lhs(727, A::mul(A::mul(A::scale(s.ad_value(718), 2.0), s.ad_value(726)), s.ad_value(108)), 721);
        }

        if (s.v[1541] != 0.0) {
            s.store_mul_ad_lhs(728, A::mul(A::mul(A::scale(s.ad_value(90), 2.0), s.ad_value(725)), s.ad_value(108)), 144);
        }

        if (s.v[1541] != 0.0) {
            s.store_mul_ad(729, A::mul(A::scale(s.ad_value(725), 2.0), s.ad_value(108)), A::sub(s.ad_value(723), s.ad_value(144)));
        }

        if (s.v[1541] != 0.0) {
            s.store_mul_ad_rhs(730, 727, A::sub(s.ad_value(745), s.ad_value(746)));
        }

        if (s.v[1541] != 0.0) {
            s.store_add_ad(731, A::mul(s.ad_value(729), s.ad_value(746)), A::mul(s.ad_value(728), s.ad_value(746)));
        }

        if (s.v[1541] != 0.0) {
            s.store_div_ad(742, A::div_from_scalar(1.0, A::add(s.ad_value(730), s.ad_value(731))), A::add(s.ad_value(730), s.ad_value(731)));
        }

        if (s.v[1541] != 0.0) {
            s.store_mul_ad_lhs(743, A::square(s.ad_value(730)), 742);
        }

        if (s.v[1541] != 0.0) {
            s.store_mul_ad_lhs(744, A::square(s.ad_value(731)), 742);
        }

        s.v[1548] = if (s.v[30] != s.v[746]) { 1.0 } else { 0.0 };

        if ((s.v[1541] != 0.0) && (s.v[1548] != 0.0)) {
            s.store_scale_ad(724, A::mul(A::mul(A::scale(s.ad_value(90), (2.0 * s.v[46])), s.ad_value(108)), s.ad_value(723)), 6.241457005723417e18);
        }

        if ((s.v[1541] != 0.0) && (s.v[1548] != 0.0)) {
            s.store_sub_ad_lhs(361, A::sub(s.ad_value(745), A::scale(s.ad_value(359), 2.0)), 746);
        }

        if ((s.v[1541] != 0.0) && (s.v[1548] != 0.0)) {
            s.store_square(362, 361);
        }

        if ((s.v[1541] != 0.0) && (s.v[1548] != 0.0)) {
            s.store_scale(13, 362, (10000000000.0 * s.v[46]));
        }

        if ((s.v[1541] != 0.0) && (s.v[1548] != 0.0)) {
            s.store_scale_ad(14, A::ln(A::max_with_scalar(A::div(A::add(s.ad_value(724), s.ad_value(367)), A::add(s.ad_value(366), s.ad_value(367))), 1e-38)), p.p785);
        }

        if ((s.v[1541] != 0.0) && (s.v[1548] != 0.0)) {
            s.store_scaled_sub(15, 724, 366, p.p799);
        }

        if ((s.v[1541] != 0.0) && (s.v[1548] != 0.0)) {
            s.store_scale_ad(16, A::sub(A::square(s.ad_value(724)), A::square(s.ad_value(366))), (0.5 * p.p800));
        }

        if ((s.v[1541] != 0.0) && (s.v[1548] != 0.0)) {
            s.store_scale(17, 362, (10000000000.0 * (s.v[29] * p.p2)));
        }

        if ((s.v[1541] != 0.0) && (s.v[1548] != 0.0)) {
            s.store_add_ad(732, A::mul(A::div(s.ad_value(736), s.ad_value(13)), A::add(A::add(s.ad_value(14), s.ad_value(15)), s.ad_value(16))), A::div(A::mul(A::mul(A::div(s.ad_value(737), s.ad_value(17)), s.ad_value(363)), s.ad_value(738)), s.ad_value(739)));
        }

        if ((s.v[1541] != 0.0) && (s.v[1548] != 0.0)) {
            s.store_mul_ad_lhs(18, A::mul(A::scale(s.ad_value(361), ((s.v[29] * p.p2) * 10000000000.0)), s.ad_value(367)), 367);
        }

        if ((s.v[1541] != 0.0) && (s.v[1548] != 0.0)) {
            s.store_mul_ad_lhs(733, A::mul(A::div(s.ad_value(740), s.ad_value(18)), s.ad_value(188)), 188);
        }

        if ((s.v[1541] != 0.0) && (s.v[1548] != 0.0)) {
            s.store_add(19, 733, 732);
        }

        s.v[1549] = if (s.v[19] > 0.0) { 1.0 } else { 0.0 };

        if (((s.v[1541] != 0.0) && (s.v[1548] != 0.0)) && (s.v[1549] != 0.0)) {
            s.store_div_ad_lhs(734, A::mul(s.ad_value(732), s.ad_value(733)), 19);
        }

        if (((s.v[1541] != 0.0) && (s.v[1548] != 0.0)) && (!(s.v[1549] != 0.0))) {
            s.store_scalar(734, 0.0);
        }

        if ((s.v[1541] != 0.0) && (!(s.v[1548] != 0.0))) {
            s.store_scalar(734, 0.0);
        }

        if (s.v[1541] != 0.0) {
            s.store_scale(20, 108, (p.p1067 * 1.60219e-19));
        }

        if (s.v[1541] != 0.0) {
            s.store_mul_ad_lhs(21, A::mul(A::scale(s.ad_value(746), ((s.v[29] * p.p2) * 10000000000.0)), s.ad_value(367)), 367);
        }

        if (s.v[1541] != 0.0) {
            s.store_mul_ad_lhs(741, A::mul(A::div(s.ad_value(20), s.ad_value(21)), s.ad_value(188)), 188);
        }

        if (s.v[1541] != 0.0) {
            s.copy_ad(22, 741);
        }

        s.v[1550] = if (s.v[22] > 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1541] != 0.0) && (s.v[1550] != 0.0)) {
            s.copy_ad(735, 741);
        }

        if ((s.v[1541] != 0.0) && (!(s.v[1550] != 0.0))) {
            s.store_scalar(735, 0.0);
        }

        if (s.v[1541] != 0.0) {
            s.store_add_ad(370, A::mul(s.ad_value(734), s.ad_value(743)), A::mul(s.ad_value(735), s.ad_value(744)));
        }

        s.v[1551] = if (p.p801 >= (s.v[30] / 2.0)) { 1.0 } else { 0.0 };

        if ((!(s.v[1541] != 0.0)) && (s.v[1551] != 0.0)) {
            s.store_scalar(359, 0.0);
        }

        if ((!(s.v[1541] != 0.0)) && (!(s.v[1551] != 0.0))) {
            s.store_scalar(359, p.p801);
        }

        s.v[1552] = if (((p.p785 > 0.0) || (p.p799 > 0.0)) || (p.p800 > 0.0)) { 1.0 } else { 0.0 };

        s.v[1553] = if ((p.p786 != 0.0) && (p.p785 > 0.0)) { 1.0 } else { 0.0 };

        if (((!(s.v[1541] != 0.0)) && (s.v[1552] != 0.0)) && (s.v[1553] != 0.0)) {
            s.store_div(13, 80, 641);
        }

        if (((!(s.v[1541] != 0.0)) && (s.v[1552] != 0.0)) && (s.v[1553] != 0.0)) {
            s.store_offset_ad(14, A::pow(s.ad_value(13), s.ad_value(642)), 1.0);
        }

        if (((!(s.v[1541] != 0.0)) && (s.v[1552] != 0.0)) && (s.v[1553] != 0.0)) {
            s.store_div(15, 640, 14);
        }

        if (((!(s.v[1541] != 0.0)) && (s.v[1552] != 0.0)) && (s.v[1553] != 0.0)) {
            s.store_scale(16, 15, 1.0 / (p.p785));
        }

        if (((!(s.v[1541] != 0.0)) && (s.v[1552] != 0.0)) && (s.v[1553] != 0.0)) {
            s.store_scale_ad(17, A::add(A::offset(s.ad_value(16), 1.0), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(16), (-1.0)), A::offset(s.ad_value(16), (-1.0))), ((0.25 * p.p798) * p.p798)))), 0.5);
        }

        if (((!(s.v[1541] != 0.0)) && (s.v[1552] != 0.0)) && (s.v[1553] != 0.0)) {
            s.store_scale(364, 17, p.p785);
        }

        if (((!(s.v[1541] != 0.0)) && (s.v[1552] != 0.0)) && (!(s.v[1553] != 0.0))) {
            s.store_scalar(364, p.p785);
        }

        if ((!(s.v[1541] != 0.0)) && (s.v[1552] != 0.0)) {
            s.store_sub_from_scalar_ad(361, s.v[30], A::scale(s.ad_value(359), 2.0));
        }

        if ((!(s.v[1541] != 0.0)) && (s.v[1552] != 0.0)) {
            s.store_square(362, 361);
        }

        if ((!(s.v[1541] != 0.0)) && (s.v[1552] != 0.0)) {
            s.store_scale(12, 362, (10000000000.0 * s.v[46]));
        }

        if ((!(s.v[1541] != 0.0)) && (s.v[1552] != 0.0)) {
            s.store_scale_ad(365, A::mul(A::mul(A::mul(A::mul(A::scale(s.ad_value(90), (2.0 * s.v[46])), s.ad_value(108)), s.ad_value(200)), s.ad_value(628)), s.ad_value(611)), 6.241457005723417e18);
        }

        if ((!(s.v[1541] != 0.0)) && (s.v[1552] != 0.0)) {
            s.store_mul_ad_rhs(13, 364, A::ln(A::max_with_scalar(A::div(A::add(s.ad_value(365), s.ad_value(367)), A::add(s.ad_value(366), s.ad_value(367))), 1e-38)));
        }

        if ((!(s.v[1541] != 0.0)) && (s.v[1552] != 0.0)) {
            s.store_scaled_sub(14, 365, 366, p.p799);
        }

        if ((!(s.v[1541] != 0.0)) && (s.v[1552] != 0.0)) {
            s.store_scale_ad(15, A::sub(A::square(s.ad_value(365)), A::square(s.ad_value(366))), (0.5 * p.p800));
        }

        if ((!(s.v[1541] != 0.0)) && (s.v[1552] != 0.0)) {
            s.store_scale(16, 362, (10000000000.0 * (s.v[29] * p.p2)));
        }

        if ((!(s.v[1541] != 0.0)) && (s.v[1552] != 0.0)) {
            s.store_add_ad(368, A::mul(A::div(s.ad_value(736), s.ad_value(12)), A::add(A::add(s.ad_value(13), s.ad_value(14)), s.ad_value(15))), A::div(A::mul(A::mul(A::div(s.ad_value(737), s.ad_value(16)), s.ad_value(363)), s.ad_value(738)), s.ad_value(739)));
        }

        if ((!(s.v[1541] != 0.0)) && (s.v[1552] != 0.0)) {
            s.store_mul_ad_lhs(17, A::mul(A::scale(s.ad_value(361), ((s.v[29] * p.p2) * 10000000000.0)), s.ad_value(367)), 367);
        }

        if ((!(s.v[1541] != 0.0)) && (s.v[1552] != 0.0)) {
            s.store_mul_ad_lhs(740, A::scale(s.ad_value(364), 1.60219e-19), 108);
        }

        if ((!(s.v[1541] != 0.0)) && (s.v[1552] != 0.0)) {
            s.store_mul_ad_lhs(369, A::mul(A::div(s.ad_value(740), s.ad_value(17)), s.ad_value(188)), 188);
        }

        if ((!(s.v[1541] != 0.0)) && (s.v[1552] != 0.0)) {
            s.store_add(18, 369, 368);
        }

        s.v[1554] = if (s.v[18] > 0.0) { 1.0 } else { 0.0 };

        if (((!(s.v[1541] != 0.0)) && (s.v[1552] != 0.0)) && (s.v[1554] != 0.0)) {
            s.store_div_ad(370, A::div(A::mul(s.ad_value(368), s.ad_value(369)), s.ad_value(18)), A::offset(A::scale(A::powf(A::sub(s.ad_value(200), s.ad_value(144)), p.p803), p.p802), 1.0));
        }

        if (((!(s.v[1541] != 0.0)) && (s.v[1552] != 0.0)) && (!(s.v[1554] != 0.0))) {
            s.store_scalar(370, 0.0);
        }

        if ((!(s.v[1541] != 0.0)) && (!(s.v[1552] != 0.0))) {
            s.store_scalar(370, 0.0);
        }

        s.store_scaled_div(12, 80, 360, 1.0 / (s.v[30]));

        s.store_square(13, 12);

        s.store_scale_ad(15, A::offset(A::scale(s.ad_value(13), (p.p814 * s.v[30])), 1.0), p.p811);

        s.store_scale_ad(16, A::offset(A::scale(s.ad_value(13), (p.p815 * s.v[30])), 1.0), p.p812);

        s.store_scale_ad(17, A::offset(A::scale(s.ad_value(13), (p.p1044 * s.v[30])), 1.0), p.p1043);

        s.store_scale_ad(386, A::offset(A::scale(s.ad_value(13), (p.p816 * s.v[30])), 1.0), p.p813);

    }

    pub(super) fn stamp_transient_block_18(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        s.store_mul_ad_lhs(387, A::scale(s.ad_value(15), 3.0), 15);

        s.store_offset_ad(387, A::scale(A::offset(s.ad_value(387), (-1.0)), ((((-s.v[30]) / p.p1042)) as f64).exp()), 1.0);

        s.store_square(389, 17);

        s.store_square(388, 16);

        s.v[383] = 0.0;

        s.v[1555] = if (p.p48 == 0.0) { 1.0 } else { 0.0 };

        s.v[1556] = if (p.p48 == 1.0) { 1.0 } else { 0.0 };

        if (s.v[1555] != 0.0) {
            s.store_mul_ad_lhs(196, A::scale(s.ad_value(108), ((((-p.p2) * s.v[29]) * s.v[30]) * s.v[46])), 190);
        }

        if (s.v[1555] != 0.0) {
            s.store_mul_ad_lhs(197, A::scale(s.ad_value(108), ((((-p.p2) * s.v[29]) * s.v[30]) * s.v[46])), 193);
        }

        if (s.v[1555] != 0.0) {
            s.store_mul_ad_rhs(12, 157, A::abs(A::add(s.ad_value(196), s.ad_value(197))));
        }

        if (s.v[1555] != 0.0) {
            s.store_offset_ad(13, A::mul(s.ad_value(12), s.ad_value(244)), (s.v[30] * s.v[30]));
        }

        if (s.v[1555] != 0.0) {
            s.store_scaled_div(375, 12, 13, p.p810);
        }

        if (s.v[1555] != 0.0) {
            s.store_mul(376, 377, 375);
        }

        if ((s.v[1556] != 0.0) && (!(s.v[1555] != 0.0))) {
            s.store_mul_ad_lhs(382, A::scale(s.ad_value(90), 2.0), 106);
        }

        if ((s.v[1556] != 0.0) && (!(s.v[1555] != 0.0))) {
            s.store_mul_ad_lhs(12, A::scale(A::mul(A::mul(s.ad_value(157), s.ad_value(163)), s.ad_value(175)), s.v[46]), 382);
        }

        if ((s.v[1556] != 0.0) && (!(s.v[1555] != 0.0))) {
            s.store_scaled_add(13, 200, 144, 0.5);
        }

        if ((s.v[1556] != 0.0) && (!(s.v[1555] != 0.0))) {
            s.store_offset(15, 13, 0.5);
        }

        if ((s.v[1556] != 0.0) && (!(s.v[1555] != 0.0))) {
            s.store_square(16, 15);
        }

        if ((s.v[1556] != 0.0) && (!(s.v[1555] != 0.0))) {
            s.store_mul(17, 16, 15);
        }

        if ((s.v[1556] != 0.0) && (!(s.v[1555] != 0.0))) {
            s.store_sub(18, 200, 144);
        }

        if ((s.v[1556] != 0.0) && (!(s.v[1555] != 0.0))) {
            s.store_square(19, 18);
        }

        if ((s.v[1556] != 0.0) && (!(s.v[1555] != 0.0))) {
            s.store_mul(20, 19, 18);
        }

        if ((s.v[1556] != 0.0) && (!(s.v[1555] != 0.0))) {
            s.store_mul_ad_lhs(21, A::offset(A::scale(s.ad_value(13), 6.0), 0.5), 19);
        }

        if ((s.v[1556] != 0.0) && (!(s.v[1555] != 0.0))) {
            s.store_scale(381, 163, s.v[30]);
        }

        if ((s.v[1556] != 0.0) && (!(s.v[1555] != 0.0))) {
            s.store_scale(22, 381, 1.0 / (s.v[30]));
        }

        if ((s.v[1556] != 0.0) && (!(s.v[1555] != 0.0))) {
            s.store_offset_ad(24, A::div(A::mul(s.ad_value(389), A::div(s.ad_value(139), s.ad_value(140))), A::offset(s.ad_value(80), p.p1045)), 1.0);
        }

        if ((s.v[1556] != 0.0) && (!(s.v[1555] != 0.0))) {
            s.store_offset_ad(24, A::scale(A::offset(s.ad_value(24), (-1.0)), ((((-s.v[30]) / p.p1042)) as f64).exp()), 1.0);
        }

        s.v[1557] = if ((0.0 == 0.0) && (s.v[24] < ((-2500.0) * 0.1))) { 1.0 } else { 0.0 };

        if (((s.v[1556] != 0.0) && (!(s.v[1555] != 0.0))) && (s.v[1557] != 0.0)) {
            s.store_div_from_scalar_ad(24, ((-0.1) * 0.1), A::scale(s.ad_value(24), 16.0));
        }

        if (((s.v[1556] != 0.0) && (!(s.v[1555] != 0.0))) && (!(s.v[1557] != 0.0))) {
            s.store_scale_ad(24, A::add(s.ad_value(24), A::sqrt(A::offset(A::mul(s.ad_value(24), s.ad_value(24)), ((0.25 * 0.1) * 0.1)))), 0.5);
        }

        if ((s.v[1556] != 0.0) && (!(s.v[1555] != 0.0))) {
            s.store_mul_ad(380, A::div(A::scale(s.ad_value(12), (p.p2 * s.v[29])), s.ad_value(381)), A::add(A::mul(s.ad_value(13), s.ad_value(24)), A::div(A::mul(s.ad_value(19), s.ad_value(387)), A::scale(s.ad_value(15), 12.0))));
        }

        if ((s.v[1556] != 0.0) && (!(s.v[1555] != 0.0))) {
            let assign29020_ad_e38614: A = A::div(A::mul(A::scale(A::mul(A::mul(A::mul(s.ad_value(381), s.ad_value(22)), s.ad_value(22)), A::add(A::sub(A::div(s.ad_value(13), s.ad_value(16)), A::div(s.ad_value(21), A::mul(A::scale(s.ad_value(16), 60.0), s.ad_value(16)))), A::div(A::square(s.ad_value(19)), A::mul(A::scale(s.ad_value(16), 144.0), s.ad_value(17))))), (15.0 * 0.25)), s.ad_value(388)), A::scale(s.ad_value(12), ((p.p2 * s.v[29]) * 12.0)));
            s.store_ad(378, &assign29020_ad_e38614);
        }

        if ((s.v[1556] != 0.0) && (!(s.v[1555] != 0.0))) {
            s.store_scale_ad(379, A::mul(A::mul(s.ad_value(22), A::sub(A::div(s.ad_value(18), A::scale(s.ad_value(15), 12.0)), A::div(s.ad_value(20), A::scale(s.ad_value(17), 144.0)))), s.ad_value(386)), 2.531645569620253);
        }

        if ((s.v[1556] != 0.0) && (!(s.v[1555] != 0.0))) {
            s.store_sqrt_ad(384, A::mul(s.ad_value(377), s.ad_value(380)));
        }

        s.v[1558] = if (s.v[378] > 0.0) { 1.0 } else { 0.0 };

        if (((s.v[1556] != 0.0) && (!(s.v[1555] != 0.0))) && (s.v[1558] != 0.0)) {
            s.store_sqrt_ad(385, A::div(s.ad_value(377), s.ad_value(378)));
        }

        s.v[1559] = if (s.v[384] > 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[1556] != 0.0) && (!(s.v[1555] != 0.0))) && (s.v[1558] != 0.0)) && (s.v[1559] != 0.0)) {
            s.store_div_ad_lhs(383, A::mul(s.ad_value(379), s.ad_value(385)), 384);
        }

        if ((((s.v[1556] != 0.0) && (!(s.v[1555] != 0.0))) && (s.v[1558] != 0.0)) && (!(s.v[1559] != 0.0))) {
            s.store_scalar(383, 0.0);
        }

        if (((s.v[1556] != 0.0) && (!(s.v[1555] != 0.0))) && (!(s.v[1558] != 0.0))) {
            s.store_scalar(385, 0.0);
        }

        if (((s.v[1556] != 0.0) && (!(s.v[1555] != 0.0))) && (!(s.v[1558] != 0.0))) {
            s.store_scalar(383, 0.0);
        }

        s.v[1560] = if (p.p46 != 0.0) { 1.0 } else { 0.0 };

        s.v[1561] = if (p.p47 != 0.0) { 1.0 } else { 0.0 };

        s.copy_ad(60, 59);

        s.v[218] = 0.0;

        s.v[1562] = if (p.p40 == 1.0) { 1.0 } else { 0.0 };

        if (s.v[1562] != 0.0) {
            s.store_offset(549, 549, p.p35);
        }

        if (s.v[1562] != 0.0) {
            s.store_mul(65, 64, 109);
        }

        if (s.v[1562] != 0.0) {
            s.store_mul(73, 72, 109);
        }

        if (s.v[1562] != 0.0) {
            s.store_mul(58, 549, 109);
        }

        if (s.v[1562] != 0.0) {
            s.store_sub(60, 65, 58);
        }

        if (s.v[1562] != 0.0) {
            s.store_ln_ad(233, A::max_with_scalar(A::div(s.ad_value(550), s.ad_value(28)), 1e-38));
        }

        if (s.v[1562] != 0.0) {
            s.store_scale_ad(234, A::sqrt(A::mul(A::scale(s.ad_value(550), ((2.0 * 1.60219e-19) * s.v[26])), s.ad_value(109))), 1.0 / (s.v[46]));
        }

        if (s.v[1562] != 0.0) {
            s.store_div_from_scalar(126, 1.0, 234);
        }

        if (s.v[1562] != 0.0) {
            s.store_div_ad(206, A::scale(s.ad_value(479), ((2.0 * 1.60219e-19) * s.v[26])), A::scale(s.ad_value(108), (s.v[46] * s.v[46])));
        }

        if (s.v[1562] != 0.0) {
            s.store_ad(218, &{
                if (s.v[479] > 0.0) {
                    A::div_from_scalar(1.0, s.ad_value(206))
                } else {
                    A::constant(0.0)
                }
            });
        }

        if (s.v[1562] != 0.0) {
            s.store_ad(203, &{
                if (s.v[479] > 0.0) {
                    A::div(s.ad_value(550), s.ad_value(479))
                } else {
                    A::constant(0.0)
                }
            });
        }

        if (s.v[1562] != 0.0) {
            s.store_offset(13, 203, 1.0);
        }

        if (s.v[1562] != 0.0) {
            s.store_div(204, 60, 13);
        }

        if (s.v[1562] != 0.0) {
            s.store_div(205, 234, 13);
        }

        if (s.v[1562] != 0.0) {
            s.store_sub_ad(13, A::scale(s.ad_value(204), 0.5), A::scale(A::offset(A::scale(s.ad_value(205), 0.7071067811865475), 1.0), 3.0));
        }

        if (s.v[1562] != 0.0) {
            s.store_add_ad_rhs(14, 13, A::sqrt(A::add(A::square(s.ad_value(13)), A::scale(s.ad_value(204), 6.0))));
        }

        s.v[1563] = if (s.v[204] < 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1562] != 0.0) && (s.v[1563] != 0.0)) {
            s.store_div_ad_lhs(15, A::sub(s.ad_value(204), s.ad_value(14)), 205);
        }

        if ((s.v[1562] != 0.0) && (s.v[1563] != 0.0)) {
            s.store_neg_ad(91, A::ln(A::max_with_scalar(A::add(A::sub_from_scalar(1.0, s.ad_value(14)), A::square(s.ad_value(15))), 1e-38)));
        }

        if ((s.v[1562] != 0.0) && (!(s.v[1563] != 0.0))) {
            s.store_limited_exp_ad(15, A::neg(s.ad_value(14)));
        }

        if ((s.v[1562] != 0.0) && (!(s.v[1563] != 0.0))) {
            s.store_scale(13, 205, 0.5);
        }

        if ((s.v[1562] != 0.0) && (!(s.v[1563] != 0.0))) {
            s.store_sub_ad_lhs(14, A::sqrt(A::add(A::add(A::offset(s.ad_value(204), (-1.0)), s.ad_value(15)), A::square(s.ad_value(13)))), 13);
        }

        if ((s.v[1562] != 0.0) && (!(s.v[1563] != 0.0))) {
            s.store_sub_ad_lhs(91, A::offset(A::square(s.ad_value(14)), 1.0), 15);
        }

        if (s.v[1562] != 0.0) {
            s.store_scale_ad(20, A::add(A::offset(s.ad_value(91), 1.0), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(91), (-1.0)), A::offset(s.ad_value(91), (-1.0))), ((0.25 * 2.0) * 2.0)))), 0.5);
        }

        if (s.v[1562] != 0.0) {
            s.store_sqrt(96, 20);
        }

        if (s.v[1562] != 0.0) {
            s.store_div_ad_lhs(12, A::offset(A::div(s.ad_value(234), A::scale(s.ad_value(96), 2.0)), 1.0), 234);
        }

        if (s.v[1562] != 0.0) {
            s.store_sub_ad_lhs(13, A::sub(s.ad_value(91), A::scale(s.ad_value(233), 2.0)), 73);
        }

        if (s.v[1562] != 0.0) {
            s.store_sub_ad(14, A::scale(s.ad_value(13), 1.0 / (p.p1137)), A::ln(A::max_with_scalar(A::mul(A::scale(s.ad_value(12), 4.0), s.ad_value(96)), 1e-38)));
        }

        if (s.v[1562] != 0.0) {
            s.store_scale_ad(20, A::sub(A::offset(s.ad_value(14), (-0.201491)), A::sqrt(A::offset(A::mul(s.ad_value(14), A::offset(s.ad_value(14), 0.402982)), 2.446562))), 0.5);
        }

        if (s.v[1562] != 0.0) {
            s.copy_ad(94, 96);
        }

        s.v[1564] = if (s.v[20] <= (-68.0)) { 1.0 } else { 0.0 };

        if ((s.v[1562] != 0.0) && (s.v[1564] != 0.0)) {
            s.store_scalar(16, (-100.0));
        }

        if ((s.v[1562] != 0.0) && (s.v[1564] != 0.0)) {
            s.store_scalar(17, 20.0);
        }

        s.v[1565] = if (s.v[20] < (s.v[16] - (0.5 * s.v[17]))) { 1.0 } else { 0.0 };

        if (((s.v[1562] != 0.0) && (s.v[1564] != 0.0)) && (s.v[1565] != 0.0)) {
            s.store_ad(15, &A::limited_exp(s.ad_value(16)));
        }

        s.v[1566] = if (s.v[20] > (s.v[16] + (0.5 * s.v[17]))) { 1.0 } else { 0.0 };

        if ((((s.v[1562] != 0.0) && (s.v[1564] != 0.0)) && (!(s.v[1565] != 0.0))) && (s.v[1566] != 0.0)) {
            s.store_ad(15, &A::limited_exp(s.ad_value(20)));
        }

        if ((((s.v[1562] != 0.0) && (s.v[1564] != 0.0)) && (!(s.v[1565] != 0.0))) && (!(s.v[1566] != 0.0))) {
            s.store_div_ad_lhs(14, A::sub(s.ad_value(20), s.ad_value(16)), 17);
        }

        if ((((s.v[1562] != 0.0) && (s.v[1564] != 0.0)) && (!(s.v[1565] != 0.0))) && (!(s.v[1566] != 0.0))) {
            s.store_square(18, 14);
        }

        if ((((s.v[1562] != 0.0) && (s.v[1564] != 0.0)) && (!(s.v[1565] != 0.0))) && (!(s.v[1566] != 0.0))) {
            s.store_limited_exp_ad(15, A::add(s.ad_value(16), A::mul(s.ad_value(17), A::add(A::offset(A::scale(s.ad_value(14), 0.5), (5.0 / 64.0)), A::mul(s.ad_value(18), A::sub_from_scalar((15.0 / 16.0), A::mul(s.ad_value(18), A::sub_from_scalar(1.25, s.ad_value(18)))))))));
        }

        if ((s.v[1562] != 0.0) && (s.v[1564] != 0.0)) {
            s.store_mul_ad_rhs(200, 15, A::sub(A::sub(A::offset(s.ad_value(13), 1.0), A::scale(s.ad_value(20), p.p1137)), A::scale(A::ln(A::max_with_scalar(A::mul(A::scale(s.ad_value(12), 2.0), A::add(A::mul(A::scale(s.ad_value(15), 2.0), s.ad_value(12)), A::scale(s.ad_value(94), 2.0))), 1e-38)), p.p1137)));
        }

        if ((s.v[1562] != 0.0) && (!(s.v[1564] != 0.0))) {
            s.store_ad(15, &A::limited_exp(s.ad_value(20)));
        }

        if ((s.v[1562] != 0.0) && (!(s.v[1564] != 0.0))) {
            s.store_div_from_scalar(95, 1.0, 94);
        }

        if ((s.v[1562] != 0.0) && (!(s.v[1564] != 0.0))) {
            s.store_sub_ad_lhs(16, A::add(A::scale(s.ad_value(15), 2.0), A::scale(A::ln(A::max_with_scalar(A::mul(A::mul(A::scale(s.ad_value(15), 2.0), s.ad_value(12)), A::add(A::mul(A::scale(s.ad_value(15), 2.0), s.ad_value(12)), A::scale(s.ad_value(94), 2.0))), 1e-38)), p.p1137)), 13);
        }

        if ((s.v[1562] != 0.0) && (!(s.v[1564] != 0.0))) {
            s.store_add_ad(17, A::offset(A::div_from_scalar(p.p1137, s.ad_value(15)), 2.0), A::div(A::scale(A::add(s.ad_value(12), s.ad_value(95)), p.p1137), A::add(A::mul(s.ad_value(12), s.ad_value(15)), s.ad_value(94))));
        }

        if ((s.v[1562] != 0.0) && (!(s.v[1564] != 0.0))) {
            s.store_sub_ad_rhs(15, 15, A::div(s.ad_value(16), s.ad_value(17)));
        }

        if ((s.v[1562] != 0.0) && (!(s.v[1564] != 0.0))) {
            s.store_sub_ad_lhs(16, A::add(A::scale(s.ad_value(15), 2.0), A::scale(A::ln(A::max_with_scalar(A::mul(A::mul(A::scale(s.ad_value(15), 2.0), s.ad_value(12)), A::add(A::mul(A::scale(s.ad_value(15), 2.0), s.ad_value(12)), A::scale(s.ad_value(94), 2.0))), 1e-38)), p.p1137)), 13);
        }

        if ((s.v[1562] != 0.0) && (!(s.v[1564] != 0.0))) {
            s.store_add_ad(17, A::offset(A::div_from_scalar(p.p1137, s.ad_value(15)), 2.0), A::div(A::scale(A::add(s.ad_value(12), s.ad_value(95)), p.p1137), A::add(A::mul(s.ad_value(12), s.ad_value(15)), s.ad_value(94))));
        }

        if ((s.v[1562] != 0.0) && (!(s.v[1564] != 0.0))) {
            s.store_mul_ad(18, A::scale(A::div(A::add(s.ad_value(12), s.ad_value(95)), A::add(A::mul(s.ad_value(12), s.ad_value(15)), s.ad_value(94))), p.p1137), A::div(A::add(s.ad_value(12), s.ad_value(95)), A::add(A::mul(s.ad_value(12), s.ad_value(15)), s.ad_value(94))));
        }

        if ((s.v[1562] != 0.0) && (!(s.v[1564] != 0.0))) {
            s.store_sub_ad_lhs(19, A::sub(A::scale(A::mul(A::div_from_scalar(1.0, s.ad_value(15)), A::div_from_scalar(1.0, s.ad_value(15))), (-p.p1137)), A::div_from_scalar(p.p1137, A::mul(A::mul(A::square(s.ad_value(94)), s.ad_value(94)), A::add(A::mul(s.ad_value(12), s.ad_value(15)), s.ad_value(94))))), 18);
        }

        if ((s.v[1562] != 0.0) && (!(s.v[1564] != 0.0))) {
            s.store_sub_ad_rhs(200, 15, A::mul(A::div(s.ad_value(16), s.ad_value(17)), A::offset(A::div(A::mul(s.ad_value(16), s.ad_value(19)), A::mul(A::scale(s.ad_value(17), 2.0), s.ad_value(17))), 1.0)));
        }

        s.v[1567] = if ((1.0 == 0.0) && (s.v[91] < ((-2500.0) * 2.0))) { 1.0 } else { 0.0 };

        if ((s.v[1562] != 0.0) && (s.v[1567] != 0.0)) {
            s.store_div_from_scalar_ad(93, ((-2.0) * 2.0), A::scale(s.ad_value(91), 16.0));
        }

        if ((s.v[1562] != 0.0) && (!(s.v[1567] != 0.0))) {
            s.store_scale_ad(93, A::add(A::offset(s.ad_value(91), 1.0), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(91), (-1.0)), A::offset(s.ad_value(91), (-1.0))), ((0.25 * 2.0) * 2.0)))), 0.5);
        }

        if (s.v[1562] != 0.0) {
            s.store_sqrt(96, 93);
        }

        if (s.v[1562] != 0.0) {
            s.store_sub_ad_rhs(92, 91, A::scale(s.ad_value(200), 2.0));
        }

        s.v[1568] = if ((1.0 == 0.0) && (s.v[92] < ((-2500.0) * 2.0))) { 1.0 } else { 0.0 };

        if ((s.v[1562] != 0.0) && (s.v[1568] != 0.0)) {
            s.store_div_from_scalar_ad(12, ((-2.0) * 2.0), A::scale(s.ad_value(92), 16.0));
        }

        if ((s.v[1562] != 0.0) && (!(s.v[1568] != 0.0))) {
            s.store_scale_ad(12, A::add(A::offset(s.ad_value(92), 1.0), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(92), (-1.0)), A::offset(s.ad_value(92), (-1.0))), ((0.25 * 2.0) * 2.0)))), 0.5);
        }

        if (s.v[1562] != 0.0) {
            s.store_offset_ad(90, A::div(s.ad_value(234), A::add(s.ad_value(96), A::sqrt(s.ad_value(12)))), 1.0);
        }

        if (s.v[1562] != 0.0) {
            s.store_mul_ad_rhs(12, 108, A::sub(A::sub(s.ad_value(60), s.ad_value(91)), A::mul(A::scale(s.ad_value(200), 2.0), A::offset(s.ad_value(90), (-1.0)))));
        }

        s.v[1569] = if ((0.0 == 0.0) && (s.v[12] < ((-2500.0) * 0.1))) { 1.0 } else { 0.0 };

        if ((s.v[1562] != 0.0) && (s.v[1569] != 0.0)) {
            s.store_div_from_scalar_ad(84, ((-0.1) * 0.1), A::scale(s.ad_value(12), 16.0));
        }

        if ((s.v[1562] != 0.0) && (!(s.v[1569] != 0.0))) {
            s.store_scale_ad(84, A::add(s.ad_value(12), A::sqrt(A::offset(A::mul(s.ad_value(12), s.ad_value(12)), ((0.25 * 0.1) * 0.1)))), 0.5);
        }

        if (s.v[1562] != 0.0) {
            s.store_mul_ad_lhs(130, A::mul(A::scale(s.ad_value(90), 2.0), s.ad_value(108)), 200);
        }

        if (s.v[1562] != 0.0) {
            s.store_scale_ad(132, A::add(s.ad_value(84), A::scale(s.ad_value(130), s.v[158])), s.v[155]);
        }

        if (s.v[1562] != 0.0) {
            s.store_mul_ad(15, A::add(s.ad_value(506), A::mul(s.ad_value(516), s.ad_value(62))), A::pow(s.ad_value(132), s.ad_value(407)));
        }

        if (s.v[1562] != 0.0) {
            s.store_offset(16, 15, 1.0);
        }

        s.v[1570] = if ((1.0 == 0.0) && (s.v[16] < ((-2500.0) * 0.0015))) { 1.0 } else { 0.0 };

        if ((s.v[1562] != 0.0) && (s.v[1570] != 0.0)) {
            s.store_div_from_scalar_ad(133, ((-0.0015) * 0.0015), A::scale(s.ad_value(16), 16.0));
        }

        if ((s.v[1562] != 0.0) && (!(s.v[1570] != 0.0))) {
            s.store_scale_ad(133, A::add(A::offset(s.ad_value(16), 1.0), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(16), (-1.0)), A::offset(s.ad_value(16), (-1.0))), ((0.25 * 0.0015) * 0.0015)))), 0.5);
        }

        if (s.v[1562] != 0.0) {
            s.store_div_ad(137, A::mul(A::div(s.ad_value(499), s.ad_value(133)), s.ad_value(108)), A::scale(s.ad_value(411), s.v[34]));
        }

        if (s.v[1562] != 0.0) {
            s.store_div_ad(131, A::mul(s.ad_value(137), A::add(A::square(s.ad_value(200)), s.ad_value(200))), A::offset(A::mul(s.ad_value(137), A::offset(s.ad_value(200), 1.0)), 1.0));
        }

        if (s.v[1562] != 0.0) {
            let assign29900_ad_e39747: A = A::sub(A::sub(s.ad_value(91), A::scale(s.ad_value(233), 2.0)), A::add(A::scale(s.ad_value(131), 2.0), A::ln(A::max_with_scalar(A::mul(A::mul(A::mul(A::scale(s.ad_value(131), 2.0), s.ad_value(90)), s.ad_value(126)), A::add(A::mul(A::mul(A::scale(s.ad_value(131), 2.0), s.ad_value(90)), s.ad_value(126)), A::div(s.ad_value(234), A::offset(s.ad_value(90), (-1.0))))), 1e-38))));
            s.store_ad(145, &assign29900_ad_e39747);
        }

        if (s.v[1562] != 0.0) {
            s.store_mul(146, 145, 108);
        }

        s.v[1571] = if ((0.0 == 0.0) && ((s.v[146] - s.v[72]) < ((-2500.0) * 0.001))) { 1.0 } else { 0.0 };

        if ((s.v[1562] != 0.0) && (s.v[1571] != 0.0)) {
            s.store_div_from_scalar_ad(141, ((-0.001) * 0.001), A::scale(A::sub(s.ad_value(146), s.ad_value(72)), 16.0));
        }

        if ((s.v[1562] != 0.0) && (!(s.v[1571] != 0.0))) {
            s.store_scale_ad(141, A::add(A::sub(s.ad_value(146), s.ad_value(72)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(146), s.ad_value(72)), A::sub(s.ad_value(146), s.ad_value(72))), ((0.25 * 0.001) * 0.001)))), 0.5);
        }

        s.v[1572] = if ((p.p1134 == 0.0) && (p.p1135 == 0.0)) { 1.0 } else { 0.0 };

        if ((s.v[1562] != 0.0) && (s.v[1572] != 0.0)) {
            s.store_scalar(783, p.p1129);
        }

        if ((s.v[1562] != 0.0) && (!(s.v[1572] != 0.0))) {
            s.store_div_from_scalar_ad(13, s.v[30], A::offset(A::sqrt(A::mul(s.ad_value(538), s.ad_value(112))), s.v[30]));
        }

        if ((s.v[1562] != 0.0) && (!(s.v[1572] != 0.0))) {
            s.store_offset_ad(783, A::div(A::sub(A::scale(s.ad_value(13), p.p1134), A::mul(A::mul(A::scale(s.ad_value(13), p.p1135), s.ad_value(200)), s.ad_value(106))), A::offset(A::scale(s.ad_value(61), p.p1136), 1.0)), 1.0);
        }

        s.v[1573] = if ((0.1 == 0.0) && (s.v[783] < ((-2500.0) * 0.0005))) { 1.0 } else { 0.0 };

        if (((s.v[1562] != 0.0) && (!(s.v[1572] != 0.0))) && (s.v[1573] != 0.0)) {
            s.store_div_from_scalar_ad(783, ((-0.0005) * 0.0005), A::scale(s.ad_value(783), 16.0));
        }

        if (((s.v[1562] != 0.0) && (!(s.v[1572] != 0.0))) && (!(s.v[1573] != 0.0))) {
            s.store_scale_ad(783, A::add(A::offset(s.ad_value(783), 0.1), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(783), (-0.1)), A::offset(s.ad_value(783), (-0.1))), ((0.25 * 0.0005) * 0.0005)))), 0.5);
        }

        if (s.v[1562] != 0.0) {
            s.store_div(141, 141, 783);
        }

        if (s.v[1562] != 0.0) {
            s.store_pow_ad(19, A::offset(A::div(s.ad_value(74), s.ad_value(141)), 1e-6), A::div_from_scalar(1.0, s.ad_value(412)));
        }

        if (s.v[1562] != 0.0) {
            s.store_pow_ad(20, A::offset(s.ad_value(19), 1.0), A::neg(s.ad_value(412)));
        }

        if (s.v[1562] != 0.0) {
            s.store_mul(139, 75, 20);
        }

        if (s.v[1562] != 0.0) {
            s.store_mul_ad_lhs(142, A::add(s.ad_value(139), s.ad_value(72)), 109);
        }

        if (s.v[1562] != 0.0) {
            s.store_scale_ad(20, A::add(A::offset(s.ad_value(91), 1.0), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(91), (-1.0)), A::offset(s.ad_value(91), (-1.0))), ((0.25 * 2.0) * 2.0)))), 0.5);
        }

    }

    pub(super) fn stamp_transient_block_19(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if (s.v[1562] != 0.0) {
            s.store_sqrt(96, 20);
        }

        if (s.v[1562] != 0.0) {
            s.store_div_ad_lhs(12, A::offset(A::div(s.ad_value(234), A::scale(s.ad_value(96), 2.0)), 1.0), 234);
        }

        if (s.v[1562] != 0.0) {
            s.store_sub_ad_lhs(13, A::sub(s.ad_value(91), A::scale(s.ad_value(233), 2.0)), 142);
        }

        if (s.v[1562] != 0.0) {
            s.store_sub_ad(14, A::scale(s.ad_value(13), 1.0 / (p.p1137)), A::ln(A::max_with_scalar(A::mul(A::scale(s.ad_value(12), 4.0), s.ad_value(96)), 1e-38)));
        }

        if (s.v[1562] != 0.0) {
            s.store_scale_ad(20, A::sub(A::offset(s.ad_value(14), (-0.201491)), A::sqrt(A::offset(A::mul(s.ad_value(14), A::offset(s.ad_value(14), 0.402982)), 2.446562))), 0.5);
        }

        if (s.v[1562] != 0.0) {
            s.copy_ad(94, 96);
        }

        s.v[1574] = if (s.v[20] <= (-68.0)) { 1.0 } else { 0.0 };

        if ((s.v[1562] != 0.0) && (s.v[1574] != 0.0)) {
            s.store_scalar(16, (-100.0));
        }

        if ((s.v[1562] != 0.0) && (s.v[1574] != 0.0)) {
            s.store_scalar(17, 20.0);
        }

        s.v[1575] = if (s.v[20] < (s.v[16] - (0.5 * s.v[17]))) { 1.0 } else { 0.0 };

        if (((s.v[1562] != 0.0) && (s.v[1574] != 0.0)) && (s.v[1575] != 0.0)) {
            s.store_ad(15, &A::limited_exp(s.ad_value(16)));
        }

        s.v[1576] = if (s.v[20] > (s.v[16] + (0.5 * s.v[17]))) { 1.0 } else { 0.0 };

        if ((((s.v[1562] != 0.0) && (s.v[1574] != 0.0)) && (!(s.v[1575] != 0.0))) && (s.v[1576] != 0.0)) {
            s.store_ad(15, &A::limited_exp(s.ad_value(20)));
        }

        if ((((s.v[1562] != 0.0) && (s.v[1574] != 0.0)) && (!(s.v[1575] != 0.0))) && (!(s.v[1576] != 0.0))) {
            s.store_div_ad_lhs(14, A::sub(s.ad_value(20), s.ad_value(16)), 17);
        }

        if ((((s.v[1562] != 0.0) && (s.v[1574] != 0.0)) && (!(s.v[1575] != 0.0))) && (!(s.v[1576] != 0.0))) {
            s.store_square(18, 14);
        }

        if ((((s.v[1562] != 0.0) && (s.v[1574] != 0.0)) && (!(s.v[1575] != 0.0))) && (!(s.v[1576] != 0.0))) {
            s.store_limited_exp_ad(15, A::add(s.ad_value(16), A::mul(s.ad_value(17), A::add(A::offset(A::scale(s.ad_value(14), 0.5), (5.0 / 64.0)), A::mul(s.ad_value(18), A::sub_from_scalar((15.0 / 16.0), A::mul(s.ad_value(18), A::sub_from_scalar(1.25, s.ad_value(18)))))))));
        }

        if ((s.v[1562] != 0.0) && (s.v[1574] != 0.0)) {
            s.store_mul_ad_rhs(144, 15, A::sub(A::sub(A::offset(s.ad_value(13), 1.0), A::scale(s.ad_value(20), p.p1137)), A::scale(A::ln(A::max_with_scalar(A::mul(A::scale(s.ad_value(12), 2.0), A::add(A::mul(A::scale(s.ad_value(15), 2.0), s.ad_value(12)), A::scale(s.ad_value(94), 2.0))), 1e-38)), p.p1137)));
        }

        if ((s.v[1562] != 0.0) && (!(s.v[1574] != 0.0))) {
            s.store_ad(15, &A::limited_exp(s.ad_value(20)));
        }

        if ((s.v[1562] != 0.0) && (!(s.v[1574] != 0.0))) {
            s.store_div_from_scalar(95, 1.0, 94);
        }

        if ((s.v[1562] != 0.0) && (!(s.v[1574] != 0.0))) {
            s.store_sub_ad_lhs(16, A::add(A::scale(s.ad_value(15), 2.0), A::scale(A::ln(A::max_with_scalar(A::mul(A::mul(A::scale(s.ad_value(15), 2.0), s.ad_value(12)), A::add(A::mul(A::scale(s.ad_value(15), 2.0), s.ad_value(12)), A::scale(s.ad_value(94), 2.0))), 1e-38)), p.p1137)), 13);
        }

        if ((s.v[1562] != 0.0) && (!(s.v[1574] != 0.0))) {
            s.store_add_ad(17, A::offset(A::div_from_scalar(p.p1137, s.ad_value(15)), 2.0), A::div(A::scale(A::add(s.ad_value(12), s.ad_value(95)), p.p1137), A::add(A::mul(s.ad_value(12), s.ad_value(15)), s.ad_value(94))));
        }

        if ((s.v[1562] != 0.0) && (!(s.v[1574] != 0.0))) {
            s.store_sub_ad_rhs(15, 15, A::div(s.ad_value(16), s.ad_value(17)));
        }

        if ((s.v[1562] != 0.0) && (!(s.v[1574] != 0.0))) {
            s.store_sub_ad_lhs(16, A::add(A::scale(s.ad_value(15), 2.0), A::scale(A::ln(A::max_with_scalar(A::mul(A::mul(A::scale(s.ad_value(15), 2.0), s.ad_value(12)), A::add(A::mul(A::scale(s.ad_value(15), 2.0), s.ad_value(12)), A::scale(s.ad_value(94), 2.0))), 1e-38)), p.p1137)), 13);
        }

        if ((s.v[1562] != 0.0) && (!(s.v[1574] != 0.0))) {
            s.store_add_ad(17, A::offset(A::div_from_scalar(p.p1137, s.ad_value(15)), 2.0), A::div(A::scale(A::add(s.ad_value(12), s.ad_value(95)), p.p1137), A::add(A::mul(s.ad_value(12), s.ad_value(15)), s.ad_value(94))));
        }

        if ((s.v[1562] != 0.0) && (!(s.v[1574] != 0.0))) {
            s.store_mul_ad(18, A::scale(A::div(A::add(s.ad_value(12), s.ad_value(95)), A::add(A::mul(s.ad_value(12), s.ad_value(15)), s.ad_value(94))), p.p1137), A::div(A::add(s.ad_value(12), s.ad_value(95)), A::add(A::mul(s.ad_value(12), s.ad_value(15)), s.ad_value(94))));
        }

        if ((s.v[1562] != 0.0) && (!(s.v[1574] != 0.0))) {
            s.store_sub_ad_lhs(19, A::sub(A::scale(A::mul(A::div_from_scalar(1.0, s.ad_value(15)), A::div_from_scalar(1.0, s.ad_value(15))), (-p.p1137)), A::div_from_scalar(p.p1137, A::mul(A::mul(A::square(s.ad_value(94)), s.ad_value(94)), A::add(A::mul(s.ad_value(12), s.ad_value(15)), s.ad_value(94))))), 18);
        }

        if ((s.v[1562] != 0.0) && (!(s.v[1574] != 0.0))) {
            s.store_sub_ad_rhs(144, 15, A::mul(A::div(s.ad_value(16), s.ad_value(17)), A::offset(A::div(A::mul(s.ad_value(16), s.ad_value(19)), A::mul(A::scale(s.ad_value(17), 2.0), s.ad_value(17))), 1.0)));
        }

        if (s.v[1562] != 0.0) {
            s.store_offset_ad(92, A::sub(A::sub(s.ad_value(91), s.ad_value(200)), s.ad_value(144)), (-1.0));
        }

        s.v[1577] = if ((1.0 == 0.0) && (s.v[92] < ((-2500.0) * 2.0))) { 1.0 } else { 0.0 };

        if ((s.v[1562] != 0.0) && (s.v[1577] != 0.0)) {
            s.store_div_from_scalar_ad(12, ((-2.0) * 2.0), A::scale(s.ad_value(92), 16.0));
        }

        if ((s.v[1562] != 0.0) && (!(s.v[1577] != 0.0))) {
            s.store_scale_ad(12, A::add(A::offset(s.ad_value(92), 1.0), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(92), (-1.0)), A::offset(s.ad_value(92), (-1.0))), ((0.25 * 2.0) * 2.0)))), 0.5);
        }

        if (s.v[1562] != 0.0) {
            s.store_sqrt(14, 12);
        }

        if (s.v[1562] != 0.0) {
            s.store_add_ad(15, A::offset(s.ad_value(203), 1.0), A::div(s.ad_value(234), A::add(s.ad_value(96), s.ad_value(14))));
        }

        if (s.v[1562] != 0.0) {
            s.store_offset_ad(16, A::mul(A::mul(s.ad_value(203), s.ad_value(14)), s.ad_value(126)), 0.5);
        }

        if (s.v[1562] != 0.0) {
            s.store_sqrt_ad(17, A::add(A::square(s.ad_value(16)), A::mul(A::mul(s.ad_value(15), A::add(s.ad_value(200), s.ad_value(144))), s.ad_value(218))));
        }

        if (s.v[1562] != 0.0) {
            s.store_div_ad_rhs(90, 15, A::add(s.ad_value(16), s.ad_value(17)));
        }

        if (s.v[1562] != 0.0) {
            s.store_mul_ad_rhs(12, 108, A::sub(A::sub(s.ad_value(60), s.ad_value(91)), A::mul(A::scale(s.ad_value(200), 2.0), A::offset(s.ad_value(90), (-1.0)))));
        }

        s.v[1578] = if ((0.0 == 0.0) && (s.v[12] < ((-2500.0) * 0.1))) { 1.0 } else { 0.0 };

        if ((s.v[1562] != 0.0) && (s.v[1578] != 0.0)) {
            s.store_div_from_scalar_ad(84, ((-0.1) * 0.1), A::scale(s.ad_value(12), 16.0));
        }

        if ((s.v[1562] != 0.0) && (!(s.v[1578] != 0.0))) {
            s.store_scale_ad(84, A::add(s.ad_value(12), A::sqrt(A::offset(A::mul(s.ad_value(12), s.ad_value(12)), ((0.25 * 0.1) * 0.1)))), 0.5);
        }

        if (s.v[1562] != 0.0) {
            s.store_mul_ad_rhs(13, 108, A::sub(A::sub(s.ad_value(60), s.ad_value(91)), A::mul(A::scale(s.ad_value(144), 2.0), A::offset(s.ad_value(90), (-1.0)))));
        }

        s.v[1579] = if ((0.0 == 0.0) && (s.v[13] < ((-2500.0) * 0.1))) { 1.0 } else { 0.0 };

        if ((s.v[1562] != 0.0) && (s.v[1579] != 0.0)) {
            s.store_div_from_scalar_ad(85, ((-0.1) * 0.1), A::scale(s.ad_value(13), 16.0));
        }

        if ((s.v[1562] != 0.0) && (!(s.v[1579] != 0.0))) {
            s.store_scale_ad(85, A::add(s.ad_value(13), A::sqrt(A::offset(A::mul(s.ad_value(13), s.ad_value(13)), ((0.25 * 0.1) * 0.1)))), 0.5);
        }

        if (s.v[1562] != 0.0) {
            s.store_scaled_add(86, 84, 85, 0.5);
        }

        if (s.v[1562] != 0.0) {
            s.store_mul_ad(80, A::mul(s.ad_value(90), s.ad_value(108)), A::add(s.ad_value(200), s.ad_value(144)));
        }

        if (s.v[1562] != 0.0) {
            s.store_scale_ad(156, A::add(s.ad_value(86), A::scale(s.ad_value(80), s.v[158])), s.v[155]);
        }

        if (s.v[1562] != 0.0) {
            s.store_offset(13, 203, 1.0);
        }

        if (s.v[1562] != 0.0) {
            s.store_div_ad_lhs(204, A::add(s.ad_value(60), A::scale(s.ad_value(109), p.p136)), 13);
        }

        if (s.v[1562] != 0.0) {
            s.store_div(205, 234, 13);
        }

        if (s.v[1562] != 0.0) {
            s.store_sub_ad(13, A::scale(s.ad_value(204), 0.5), A::scale(A::offset(A::scale(s.ad_value(205), 0.7071067811865475), 1.0), 3.0));
        }

        if (s.v[1562] != 0.0) {
            s.store_add_ad_rhs(14, 13, A::sqrt(A::add(A::square(s.ad_value(13)), A::scale(s.ad_value(204), 6.0))));
        }

        s.v[1580] = if (s.v[204] < 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1562] != 0.0) && (s.v[1580] != 0.0)) {
            s.store_div_ad_lhs(15, A::sub(s.ad_value(204), s.ad_value(14)), 205);
        }

        if ((s.v[1562] != 0.0) && (s.v[1580] != 0.0)) {
            s.store_neg_ad(91, A::ln(A::max_with_scalar(A::add(A::sub_from_scalar(1.0, s.ad_value(14)), A::square(s.ad_value(15))), 1e-38)));
        }

        if ((s.v[1562] != 0.0) && (!(s.v[1580] != 0.0))) {
            s.store_limited_exp_ad(15, A::neg(s.ad_value(14)));
        }

        if ((s.v[1562] != 0.0) && (!(s.v[1580] != 0.0))) {
            s.store_scale(13, 205, 0.5);
        }

        if ((s.v[1562] != 0.0) && (!(s.v[1580] != 0.0))) {
            s.store_sub_ad_lhs(14, A::sqrt(A::add(A::add(A::offset(s.ad_value(204), (-1.0)), s.ad_value(15)), A::square(s.ad_value(13)))), 13);
        }

        if ((s.v[1562] != 0.0) && (!(s.v[1580] != 0.0))) {
            s.store_sub_ad_lhs(91, A::offset(A::square(s.ad_value(14)), 1.0), 15);
        }

        if (s.v[1562] != 0.0) {
            s.store_mul_ad(15, A::add(s.ad_value(506), A::mul(s.ad_value(516), s.ad_value(62))), A::pow(s.ad_value(156), s.ad_value(407)));
        }

        if (s.v[1562] != 0.0) {
            s.store_offset(16, 15, 1.0);
        }

        s.v[1581] = if ((1.0 == 0.0) && (s.v[16] < ((-2500.0) * 0.0015))) { 1.0 } else { 0.0 };

        if ((s.v[1562] != 0.0) && (s.v[1581] != 0.0)) {
            s.store_div_from_scalar_ad(159, ((-0.0015) * 0.0015), A::scale(s.ad_value(16), 16.0));
        }

        if ((s.v[1562] != 0.0) && (!(s.v[1581] != 0.0))) {
            s.store_scale_ad(159, A::add(A::offset(s.ad_value(16), 1.0), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(16), (-1.0)), A::offset(s.ad_value(16), (-1.0))), ((0.25 * 0.0015) * 0.0015)))), 0.5);
        }

        if (s.v[1562] != 0.0) {
            s.store_div_ad(138, A::mul(A::scale(A::div(s.ad_value(499), s.ad_value(159)), 2.0), s.ad_value(108)), A::scale(s.ad_value(411), s.v[34]));
        }

        if (s.v[1562] != 0.0) {
            s.store_sub(87, 200, 144);
        }

        if (s.v[1562] != 0.0) {
            s.store_mul_ad(13, A::scale(A::mul(s.ad_value(138), s.ad_value(87)), 2.0), A::mul(s.ad_value(138), s.ad_value(87)));
        }

        if (s.v[1562] != 0.0) {
            s.store_sqrt_ad(161, A::offset(s.ad_value(13), 1.0));
        }

        if (s.v[1562] != 0.0) {
            s.store_scaled_offset(162, 161, 1.0, 0.5);
        }

        if (s.v[1562] != 0.0) {
            s.store_div_ad(134, A::scale(s.ad_value(411), 2.0), A::div(s.ad_value(499), s.ad_value(159)));
        }

        if (s.v[1562] != 0.0) {
            s.store_scale(135, 134, s.v[34]);
        }

        if (s.v[1562] != 0.0) {
            s.store_add(170, 141, 135);
        }

        if (s.v[1562] != 0.0) {
            s.store_sub(167, 75, 139);
        }

        s.v[1582] = if (s.v[542] != 0.0) { 1.0 } else { 0.0 };

        if (s.v[1582] != 0.0) {
            s.store_offset_ad(176, A::mul(s.ad_value(542), A::ln(A::max_with_scalar(A::offset(A::div(A::div(s.ad_value(167), s.ad_value(542)), s.ad_value(170)), 1.0), 1e-38))), 1.0);
        }

        if (!(s.v[1582] != 0.0)) {
            s.store_scalar(176, 1.0);
        }

        s.store_square(207, 176);

        s.store_div_from_scalar(208, 1.0, 176);

        s.store_div_from_scalar(209, 1.0, 207);

        s.store_offset(210, 176, (-1.0));

        s.store_sub(213, 60, 91);

        s.store_sub(216, 200, 144);

        s.store_mul_ad(217, A::sub(s.ad_value(200), s.ad_value(144)), A::sub(s.ad_value(200), s.ad_value(144)));

        s.store_add_ad_rhs(211, 213, A::scale(s.ad_value(200), 2.0));

        s.store_add_ad_rhs(212, 213, A::scale(s.ad_value(144), 2.0));

        s.v[1583] = if ((0.0 == 0.0) && (s.v[211] < ((-2500.0) * 0.5))) { 1.0 } else { 0.0 };

        if (s.v[1583] != 0.0) {
            s.store_div_from_scalar_ad(13, ((-0.5) * 0.5), A::scale(s.ad_value(211), 16.0));
        }

        if (!(s.v[1583] != 0.0)) {
            s.store_scale_ad(13, A::add(s.ad_value(211), A::sqrt(A::offset(A::mul(s.ad_value(211), s.ad_value(211)), ((0.25 * 0.5) * 0.5)))), 0.5);
        }

        s.v[1584] = if ((0.0 == 0.0) && (s.v[212] < ((-2500.0) * 0.5))) { 1.0 } else { 0.0 };

        if (s.v[1584] != 0.0) {
            s.store_div_from_scalar_ad(14, ((-0.5) * 0.5), A::scale(s.ad_value(212), 16.0));
        }

        if (!(s.v[1584] != 0.0)) {
            s.store_scale_ad(14, A::add(s.ad_value(212), A::sqrt(A::offset(A::mul(s.ad_value(212), s.ad_value(212)), ((0.25 * 0.5) * 0.5)))), 0.5);
        }

        s.store_sqrt_ad(214, A::offset(A::mul(s.ad_value(13), s.ad_value(218)), 0.25));

        s.store_sqrt_ad(215, A::offset(A::mul(s.ad_value(14), s.ad_value(218)), 0.25));

        s.store_div_ad_rhs(13, 211, A::offset(A::scale(s.ad_value(214), 2.0), 1.0));

        s.store_div_ad_rhs(14, 212, A::offset(A::scale(s.ad_value(215), 2.0), 1.0));

        s.store_add(15, 214, 215);

        s.store_scale_ad(16, A::div(s.ad_value(217), A::mul(A::square(s.ad_value(15)), s.ad_value(15))), 0.3333333333333333);

        s.store_div_ad(17, A::mul(A::mul(s.ad_value(783), s.ad_value(162)), s.ad_value(208)), A::add(A::offset(s.ad_value(200), 1.0), s.ad_value(144)));

        s.store_mul_ad_lhs(18, A::scale(A::add(A::square(s.ad_value(15)), A::mul(s.ad_value(214), s.ad_value(215))), 0.8), 17);

        s.store_add_ad_rhs(19, 18, A::scale(s.ad_value(218), 2.0));

        s.store_mul_ad_lhs(20, A::scale(s.ad_value(217), 0.3333333333333333), 17);

        s.store_div_ad(202, A::mul(s.ad_value(212), A::offset(A::scale(s.ad_value(215), 2.0), (-1.0))), A::offset(A::scale(s.ad_value(215), 2.0), 1.0));

        s.store_add_ad_lhs(201, A::sub(s.ad_value(213), A::mul(A::scale(A::offset(s.ad_value(90), (-1.0)), 2.0), s.ad_value(144))), 202);

        s.store_add_ad(189, A::mul(s.ad_value(208), A::add(A::add(s.ad_value(13), s.ad_value(14)), A::sub(A::mul(s.ad_value(16), s.ad_value(19)), A::mul(s.ad_value(90), A::add(A::add(s.ad_value(200), s.ad_value(144)), s.ad_value(20)))))), A::mul(s.ad_value(210), s.ad_value(201)));

        s.store_add(21, 200, 144);

        s.store_mul_ad_lhs(22, A::mul(s.ad_value(217), s.ad_value(17)), 17);

        s.store_add_ad(194, A::mul(A::mul(s.ad_value(90), s.ad_value(208)), A::add(s.ad_value(21), A::mul(A::scale(s.ad_value(217), 0.3333333333333333), s.ad_value(17)))), A::mul(A::mul(A::scale(s.ad_value(90), 2.0), s.ad_value(210)), s.ad_value(144)));

        s.store_mul_ad(191, A::mul(s.ad_value(90), s.ad_value(209)), A::sub(A::scale(s.ad_value(21), 0.5), A::mul(A::scale(s.ad_value(216), 0.16666666666666666), A::sub(A::sub_from_scalar(1.0, A::mul(s.ad_value(216), s.ad_value(17))), A::scale(s.ad_value(22), 0.2)))));

        s.store_mul_ad_lhs(192, A::mul(s.ad_value(90), A::sub(s.ad_value(176), s.ad_value(208))), 144);

        s.store_add(193, 191, 192);

        s.store_sub(190, 194, 193);

        s.v[1585] = if ((0.0 == 0.0) && ((s.v[108] * s.v[189]) < ((-2500.0) * p.p694))) { 1.0 } else { 0.0 };

        if (s.v[1585] != 0.0) {
            s.store_div_from_scalar_ad(83, ((-p.p694) * p.p694), A::scale(A::mul(s.ad_value(108), s.ad_value(189)), 16.0));
        }

        if (!(s.v[1585] != 0.0)) {
            s.store_scale_ad(83, A::add(A::mul(s.ad_value(108), s.ad_value(189)), A::sqrt(A::offset(A::mul(A::mul(s.ad_value(108), s.ad_value(189)), A::mul(s.ad_value(108), s.ad_value(189))), ((0.25 * p.p694) * p.p694)))), 0.5);
        }

        s.store_mul_ad_rhs(82, 108, A::add(s.ad_value(190), s.ad_value(193)));

        s.store_scale_ad(12, A::add(s.ad_value(82), A::scale(s.ad_value(83), p.p208)), 1.0 / (p.p207));

        s.store_offset_ad(13, A::powf(s.ad_value(12), (0.7 * p.p206)), 1.0);

        s.store_div_from_scalar(227, (p.p205 * 1.9e-9), 13);

        s.store_div_from_scalar_ad(228, (3.9 * 8.85418e-12), A::add(A::scale(s.ad_value(229), (3.9 * 1.0 / (p.p111))), A::scale(s.ad_value(227), 1.0 / (s.v[47]))));

        s.store_mul_ad_lhs(195, A::mul(A::scale(A::div_from_scalar((8.85418e-12 * p.p111), s.ad_value(229)), (((-p.p2) * s.v[33]) * s.v[34])), s.ad_value(108)), 189);

        s.store_mul_ad_lhs(199, A::scale(s.ad_value(228), ((p.p2 * s.v[33]) * s.v[34])), 108);

        s.store_mul_ad_lhs(196, A::neg(s.ad_value(199)), 190);

        s.store_mul_ad_lhs(197, A::neg(s.ad_value(199)), 193);

        s.store_neg_ad(198, A::add(A::add(s.ad_value(195), s.ad_value(196)), s.ad_value(197)));

        s.v[1586] = if !(if self.param_given[666] { 1.0 } else { 0.0 } != 0.0) { 1.0 } else { 0.0 };

        if (s.v[1586] != 0.0) {
            s.store_scalar(544, ((((2.0 * p.p111) * 8.85418e-12) / 3.141592653589793) * ((((p.p670 * (1.0 + (4e-7 / p.p77)))).max(1e-38)) as f64).ln()));
        }

        s.store_offset(225, 544, p.p671);

        s.store_offset(226, 544, p.p672);

        s.v[1587] = if (p.p41 == 0.0) { 1.0 } else { 0.0 };

        if (s.v[1587] != 0.0) {
            s.store_mul_ad_lhs(223, A::scale(s.ad_value(225), ((-s.v[33]) * p.p2)), 231);
        }

        if (s.v[1587] != 0.0) {
            s.store_mul_ad_lhs(224, A::scale(s.ad_value(226), ((-s.v[33]) * p.p2)), 232);
        }

        if (!(s.v[1587] != 0.0)) {
            s.store_sqrt_ad(12, A::offset(A::mul(A::offset(A::sub(s.ad_value(231), s.ad_value(63)), 0.02), A::offset(A::sub(s.ad_value(231), s.ad_value(63)), 0.02)), (4.0 * 0.02)));
        }

        if (!(s.v[1587] != 0.0)) {
            s.store_scale_ad(219, A::sub(A::offset(A::sub(s.ad_value(231), s.ad_value(63)), 0.02), s.ad_value(12)), 0.5);
        }

        if (!(s.v[1587] != 0.0)) {
            s.store_div_ad_rhs(18, 219, A::powf(A::offset(A::powf(A::scale(A::neg(s.ad_value(219)), 1.0 / (p.p692)), p.p693), 1.0), (1.0 / p.p693)));
        }

        if (!(s.v[1587] != 0.0)) {
            s.store_sqrt_ad(13, A::sub_from_scalar(1.0, A::div(A::scale(s.ad_value(18), 4.0), s.ad_value(547))));
        }

        if (!(s.v[1587] != 0.0)) {
            s.store_scale_ad(223, A::add(A::mul(s.ad_value(225), s.ad_value(231)), A::mul(s.ad_value(545), A::sub(A::sub(A::sub(s.ad_value(231), s.ad_value(63)), s.ad_value(219)), A::mul(A::scale(s.ad_value(547), 0.5), A::offset(s.ad_value(13), (-1.0)))))), ((-s.v[33]) * p.p2));
        }

        if (!(s.v[1587] != 0.0)) {
            s.store_sqrt_ad(12, A::offset(A::mul(A::offset(A::sub(s.ad_value(232), s.ad_value(63)), 0.02), A::offset(A::sub(s.ad_value(232), s.ad_value(63)), 0.02)), (4.0 * 0.02)));
        }

        if (!(s.v[1587] != 0.0)) {
            s.store_scale_ad(220, A::sub(A::offset(A::sub(s.ad_value(232), s.ad_value(63)), 0.02), s.ad_value(12)), 0.5);
        }

        if (!(s.v[1587] != 0.0)) {
            s.store_div_ad_rhs(18, 220, A::powf(A::offset(A::powf(A::scale(A::neg(s.ad_value(220)), 1.0 / (p.p690)), p.p691), 1.0), (1.0 / p.p691)));
        }

        if (!(s.v[1587] != 0.0)) {
            s.store_sqrt_ad(14, A::sub_from_scalar(1.0, A::div(A::scale(s.ad_value(18), 4.0), s.ad_value(548))));
        }

        if (!(s.v[1587] != 0.0)) {
            s.store_scale_ad(224, A::add(A::mul(s.ad_value(226), s.ad_value(232)), A::mul(s.ad_value(546), A::sub(A::sub(A::sub(s.ad_value(232), s.ad_value(63)), s.ad_value(220)), A::mul(A::scale(s.ad_value(548), 0.5), A::offset(s.ad_value(14), (-1.0)))))), ((-s.v[33]) * p.p2));
        }

        s.store_ad(221, &A::mul(A::scale(A::neg(s.ad_value(187)), (p.p2 * (s.v[34] * p.p673))), A::voltage(ctx, &nodes, Some(10), Some(11))));

        s.v[1588] = if (p.p37 == 1.0) { 1.0 } else { 0.0 };

        if (s.v[1588] != 0.0) {
            s.store_ln_ad(684, A::max_with_scalar(A::div(s.ad_value(686), s.ad_value(28)), 1e-38));
        }

        if (s.v[1588] != 0.0) {
            s.store_max_with_scalar_ad(127, A::add(A::offset(A::mul(s.ad_value(108), s.ad_value(684)), 0.4), s.ad_value(489)), 0.4);
        }

        if (s.v[1588] != 0.0) {
            s.store_sqrt_ad(114, A::div_from_scalar((2.0 * s.v[26]), A::scale(s.ad_value(686), 1.60219e-19)));
        }

        if (s.v[1588] != 0.0) {
            let assign31550_ad_e41781: A = {
                if (!((1.0 + (s.v[622] * (s.v[395] - 1.0))) < ((-10000.0) * 0.001))) {
                    A::scale(A::add(A::offset(A::mul(s.ad_value(622), A::offset(s.ad_value(395), (-1.0))), 1.0), A::sqrt(A::offset(A::mul(A::offset(A::mul(s.ad_value(622), A::offset(s.ad_value(395), (-1.0))), 1.0), A::offset(A::mul(s.ad_value(622), A::offset(s.ad_value(395), (-1.0))), 1.0)), ((4.0 * 0.001) * 0.001)))), 0.5)
                } else {
                    {
                        if ((1.0 + (s.v[622] * (s.v[395] - 1.0))) < ((-10000.0) * 0.001)) {
                            A::div_from_scalar(((-0.001) * 0.001), A::offset(A::mul(s.ad_value(622), A::offset(s.ad_value(395), (-1.0))), 1.0))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_mul_ad_rhs(674, 612, assign31550_ad_e41781);
        }

        if (s.v[1588] != 0.0) {
            s.store_mul_ad_rhs(673, 616, A::offset(A::mul(s.ad_value(623), A::offset(s.ad_value(395), (-1.0))), 1.0));
        }

        s.v[1589] = if ((0.05 == 0.0) && ((s.v[127] - s.v[61]) < ((-2500.0) * 0.1))) { 1.0 } else { 0.0 };

        if ((s.v[1588] != 0.0) && (s.v[1589] != 0.0)) {
            s.store_div_from_scalar_ad(110, ((-0.1) * 0.1), A::scale(A::sub(s.ad_value(127), s.ad_value(61)), 16.0));
        }

        if ((s.v[1588] != 0.0) && (!(s.v[1589] != 0.0))) {
            s.store_scale_ad(110, A::add(A::offset(A::sub(s.ad_value(127), s.ad_value(61)), 0.05), A::sqrt(A::offset(A::mul(A::offset(A::sub(s.ad_value(127), s.ad_value(61)), (-0.05)), A::offset(A::sub(s.ad_value(127), s.ad_value(61)), (-0.05))), ((0.25 * 0.1) * 0.1)))), 0.5);
        }

        if (s.v[1588] != 0.0) {
            s.store_sqrt(111, 110);
        }

        if (s.v[1588] != 0.0) {
            s.store_mul(112, 114, 111);
        }

    }

    pub(super) fn stamp_transient_block_20(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if (s.v[1588] != 0.0) {
            s.store_div_from_scalar(97, s.v[26], 112);
        }

        if (s.v[1588] != 0.0) {
            s.store_sub_ad(113, A::add(A::add(s.ad_value(613), s.ad_value(674)), A::mul(s.ad_value(614), s.ad_value(76))), A::mul(s.ad_value(615), s.ad_value(61)));
        }

        if (s.v[1588] != 0.0) {
            s.store_offset_scaled(13, 113, 1.0 / (s.v[46]), 1.0);
        }

        s.v[1590] = if ((1.0 == 0.0) && (s.v[13] < ((-2500.0) * 0.05))) { 1.0 } else { 0.0 };

        if ((s.v[1588] != 0.0) && (s.v[1590] != 0.0)) {
            s.store_div_from_scalar_ad(104, ((-0.05) * 0.05), A::scale(s.ad_value(13), 16.0));
        }

        if ((s.v[1588] != 0.0) && (!(s.v[1590] != 0.0))) {
            s.store_scale_ad(104, A::add(A::offset(s.ad_value(13), 1.0), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(13), (-1.0)), A::offset(s.ad_value(13), (-1.0))), ((0.25 * 0.05) * 0.05)))), 0.5);
        }

        if (s.v[1588] != 0.0) {
            s.store_mul(106, 104, 108);
        }

        if (s.v[1588] != 0.0) {
            s.store_div_from_scalar(107, 1.0, 106);
        }

        if (s.v[1588] != 0.0) {
            s.store_mul(65, 64, 107);
        }

        if (s.v[1588] != 0.0) {
            s.store_mul(73, 70, 107);
        }

        if (s.v[1588] != 0.0) {
            s.store_mul(58, 482, 107);
        }

        if (s.v[1588] != 0.0) {
            s.store_mul_ad_lhs(677, A::neg(A::add(s.ad_value(673), A::mul(s.ad_value(617), s.ad_value(61)))), 76);
        }

        if (s.v[1588] != 0.0) {
            s.store_mul_ad(124, A::add(A::add(s.ad_value(618), A::scale(s.ad_value(619), 1.0 / (s.v[30]))), A::mul(s.ad_value(620), s.ad_value(61))), A::offset(A::pow(s.ad_value(395), s.ad_value(621)), (-1.0)));
        }

        if (s.v[1588] != 0.0) {
            s.store_mul_ad_rhs(679, 129, A::offset(A::scale(s.ad_value(61), p.p1016), 1.0));
        }

        s.v[1591] = if (s.v[679] > 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1588] != 0.0) && (s.v[1591] != 0.0)) {
            s.store_div_from_scalar(12, (p.p1015 * s.v[30]), 679);
        }

        s.v[1592] = if (s.v[12] < 40.0) { 1.0 } else { 0.0 };

        if (((s.v[1588] != 0.0) && (s.v[1591] != 0.0)) && (s.v[1592] != 0.0)) {
            s.store_div_from_scalar_ad(676, (0.5 * p.p1014), A::offset(A::cosh(s.ad_value(12)), (-1.0)));
        }

        if (((s.v[1588] != 0.0) && (s.v[1591] != 0.0)) && (!(s.v[1592] != 0.0))) {
            s.store_scale_ad(676, A::limited_exp(A::neg(s.ad_value(12))), p.p1014);
        }

        if ((s.v[1588] != 0.0) && (!(s.v[1591] != 0.0))) {
            s.store_scalar(676, 0.0);
        }

        if (s.v[1588] != 0.0) {
            s.store_mul_ad_rhs(678, 676, A::sub(s.ad_value(675), s.ad_value(127)));
        }

        if (s.v[1588] != 0.0) {
            s.store_add_ad_lhs(79, A::sub(A::add(A::offset(A::add(A::sub(s.ad_value(677), s.ad_value(124)), s.ad_value(678)), p.p961), s.ad_value(688)), A::mul(A::add(s.ad_value(624), s.ad_value(666)), s.ad_value(61))), 665);
        }

        if (s.v[1588] != 0.0) {
            s.store_sub_ad(59, A::sub(s.ad_value(65), s.ad_value(58)), A::mul(s.ad_value(79), s.ad_value(107)));
        }

        if (s.v[1588] != 0.0) {
            s.store_scalar(680, (p.p958 * (1.0 + (p.p959 * ((s.v[30]) as f64).powf((-p.p960))))));
        }

        if (s.v[1588] != 0.0) {
            s.store_scale_ad(687, A::sqrt(A::mul(A::scale(s.ad_value(686), ((2.0 * 1.60219e-19) * s.v[26])), s.ad_value(107))), 1.0 / (s.v[46]));
        }

        if (s.v[1588] != 0.0) {
            s.store_mul_ad_rhs(687, 687, A::offset(s.ad_value(680), 1.0));
        }

        if (s.v[1588] != 0.0) {
            s.store_div(685, 684, 104);
        }

        if (s.v[1588] != 0.0) {
            s.store_scalar(13, 1.0);
        }

        if (s.v[1588] != 0.0) {
            s.store_div(204, 59, 13);
        }

        if (s.v[1588] != 0.0) {
            s.store_div(205, 687, 13);
        }

        if (s.v[1588] != 0.0) {
            s.store_sub_ad(13, A::scale(s.ad_value(204), 0.5), A::scale(A::offset(A::scale(s.ad_value(205), 0.7071067811865475), 1.0), 3.0));
        }

        if (s.v[1588] != 0.0) {
            s.store_add_ad_rhs(14, 13, A::sqrt(A::add(A::square(s.ad_value(13)), A::scale(s.ad_value(204), 6.0))));
        }

        s.v[1593] = if (s.v[204] < 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1588] != 0.0) && (s.v[1593] != 0.0)) {
            s.store_div_ad_lhs(15, A::sub(s.ad_value(204), s.ad_value(14)), 205);
        }

        if ((s.v[1588] != 0.0) && (s.v[1593] != 0.0)) {
            s.store_neg_ad(91, A::ln(A::max_with_scalar(A::add(A::sub_from_scalar(1.0, s.ad_value(14)), A::square(s.ad_value(15))), 1e-38)));
        }

        if ((s.v[1588] != 0.0) && (!(s.v[1593] != 0.0))) {
            s.store_limited_exp_ad(15, A::neg(s.ad_value(14)));
        }

        if ((s.v[1588] != 0.0) && (!(s.v[1593] != 0.0))) {
            s.store_scale(13, 205, 0.5);
        }

        if ((s.v[1588] != 0.0) && (!(s.v[1593] != 0.0))) {
            s.store_sub_ad_lhs(14, A::sqrt(A::add(A::add(A::offset(s.ad_value(204), (-1.0)), s.ad_value(15)), A::square(s.ad_value(13)))), 13);
        }

        if ((s.v[1588] != 0.0) && (!(s.v[1593] != 0.0))) {
            s.store_sub_ad_lhs(91, A::offset(A::square(s.ad_value(14)), 1.0), 15);
        }

        if (s.v[1588] != 0.0) {
            s.store_scale_ad(20, A::add(A::offset(s.ad_value(91), 1.0), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(91), (-1.0)), A::offset(s.ad_value(91), (-1.0))), ((0.25 * 2.0) * 2.0)))), 0.5);
        }

        if (s.v[1588] != 0.0) {
            s.store_sqrt(96, 20);
        }

        if (s.v[1588] != 0.0) {
            s.store_div_ad_lhs(12, A::offset(A::div(s.ad_value(687), A::scale(s.ad_value(96), 2.0)), 1.0), 687);
        }

        if (s.v[1588] != 0.0) {
            s.store_sub_ad_lhs(13, A::sub(s.ad_value(91), A::scale(s.ad_value(685), 2.0)), 73);
        }

        if (s.v[1588] != 0.0) {
            s.store_sub_ad_rhs(14, 13, A::ln(A::max_with_scalar(A::mul(A::scale(s.ad_value(12), 4.0), s.ad_value(96)), 1e-38)));
        }

        if (s.v[1588] != 0.0) {
            s.store_scale_ad(20, A::sub(A::offset(s.ad_value(14), (-0.201491)), A::sqrt(A::offset(A::mul(s.ad_value(14), A::offset(s.ad_value(14), 0.402982)), 2.446562))), 0.5);
        }

        if (s.v[1588] != 0.0) {
            s.copy_ad(94, 96);
        }

        s.v[1594] = if (s.v[20] <= (-68.0)) { 1.0 } else { 0.0 };

        if ((s.v[1588] != 0.0) && (s.v[1594] != 0.0)) {
            s.store_scalar(16, (-100.0));
        }

        if ((s.v[1588] != 0.0) && (s.v[1594] != 0.0)) {
            s.store_scalar(17, 20.0);
        }

        s.v[1595] = if (s.v[20] < (s.v[16] - (0.5 * s.v[17]))) { 1.0 } else { 0.0 };

        if (((s.v[1588] != 0.0) && (s.v[1594] != 0.0)) && (s.v[1595] != 0.0)) {
            s.store_ad(15, &A::limited_exp(s.ad_value(16)));
        }

        s.v[1596] = if (s.v[20] > (s.v[16] + (0.5 * s.v[17]))) { 1.0 } else { 0.0 };

        if ((((s.v[1588] != 0.0) && (s.v[1594] != 0.0)) && (!(s.v[1595] != 0.0))) && (s.v[1596] != 0.0)) {
            s.store_ad(15, &A::limited_exp(s.ad_value(20)));
        }

        if ((((s.v[1588] != 0.0) && (s.v[1594] != 0.0)) && (!(s.v[1595] != 0.0))) && (!(s.v[1596] != 0.0))) {
            s.store_div_ad_lhs(14, A::sub(s.ad_value(20), s.ad_value(16)), 17);
        }

        if ((((s.v[1588] != 0.0) && (s.v[1594] != 0.0)) && (!(s.v[1595] != 0.0))) && (!(s.v[1596] != 0.0))) {
            s.store_square(18, 14);
        }

        if ((((s.v[1588] != 0.0) && (s.v[1594] != 0.0)) && (!(s.v[1595] != 0.0))) && (!(s.v[1596] != 0.0))) {
            s.store_limited_exp_ad(15, A::add(s.ad_value(16), A::mul(s.ad_value(17), A::add(A::offset(A::scale(s.ad_value(14), 0.5), (5.0 / 64.0)), A::mul(s.ad_value(18), A::sub_from_scalar((15.0 / 16.0), A::mul(s.ad_value(18), A::sub_from_scalar(1.25, s.ad_value(18)))))))));
        }

        if ((s.v[1588] != 0.0) && (s.v[1594] != 0.0)) {
            s.store_mul_ad_rhs(693, 15, A::sub(A::sub(A::offset(s.ad_value(13), 1.0), s.ad_value(20)), A::ln(A::max_with_scalar(A::mul(A::scale(s.ad_value(12), 2.0), A::add(A::mul(A::scale(s.ad_value(15), 2.0), s.ad_value(12)), A::scale(s.ad_value(94), 2.0))), 1e-38))));
        }

        if ((s.v[1588] != 0.0) && (!(s.v[1594] != 0.0))) {
            s.store_ad(15, &A::limited_exp(s.ad_value(20)));
        }

        if ((s.v[1588] != 0.0) && (!(s.v[1594] != 0.0))) {
            s.store_div_from_scalar(95, 1.0, 94);
        }

        if ((s.v[1588] != 0.0) && (!(s.v[1594] != 0.0))) {
            s.store_sub_ad_lhs(16, A::add(A::scale(s.ad_value(15), 2.0), A::ln(A::max_with_scalar(A::mul(A::mul(A::scale(s.ad_value(15), 2.0), s.ad_value(12)), A::add(A::mul(A::scale(s.ad_value(15), 2.0), s.ad_value(12)), A::scale(s.ad_value(94), 2.0))), 1e-38))), 13);
        }

        if ((s.v[1588] != 0.0) && (!(s.v[1594] != 0.0))) {
            s.store_add_ad(17, A::offset(A::div_from_scalar(1.0, s.ad_value(15)), 2.0), A::div(A::add(s.ad_value(12), s.ad_value(95)), A::add(A::mul(s.ad_value(12), s.ad_value(15)), s.ad_value(94))));
        }

        if ((s.v[1588] != 0.0) && (!(s.v[1594] != 0.0))) {
            s.store_sub_ad_rhs(15, 15, A::div(s.ad_value(16), s.ad_value(17)));
        }

        if ((s.v[1588] != 0.0) && (!(s.v[1594] != 0.0))) {
            s.store_sub_ad_lhs(16, A::add(A::scale(s.ad_value(15), 2.0), A::ln(A::max_with_scalar(A::mul(A::mul(A::scale(s.ad_value(15), 2.0), s.ad_value(12)), A::add(A::mul(A::scale(s.ad_value(15), 2.0), s.ad_value(12)), A::scale(s.ad_value(94), 2.0))), 1e-38))), 13);
        }

        if ((s.v[1588] != 0.0) && (!(s.v[1594] != 0.0))) {
            s.store_add_ad(17, A::offset(A::div_from_scalar(1.0, s.ad_value(15)), 2.0), A::div(A::add(s.ad_value(12), s.ad_value(95)), A::add(A::mul(s.ad_value(12), s.ad_value(15)), s.ad_value(94))));
        }

        if ((s.v[1588] != 0.0) && (!(s.v[1594] != 0.0))) {
            s.store_mul_ad(18, A::div(A::add(s.ad_value(12), s.ad_value(95)), A::add(A::mul(s.ad_value(12), s.ad_value(15)), s.ad_value(94))), A::div(A::add(s.ad_value(12), s.ad_value(95)), A::add(A::mul(s.ad_value(12), s.ad_value(15)), s.ad_value(94))));
        }

        if ((s.v[1588] != 0.0) && (!(s.v[1594] != 0.0))) {
            s.store_sub_ad_lhs(19, A::sub(A::scale(A::mul(A::div_from_scalar(1.0, s.ad_value(15)), A::div_from_scalar(1.0, s.ad_value(15))), (-1.0)), A::div_from_scalar(1.0, A::mul(A::mul(A::square(s.ad_value(94)), s.ad_value(94)), A::add(A::mul(s.ad_value(12), s.ad_value(15)), s.ad_value(94))))), 18);
        }

        if ((s.v[1588] != 0.0) && (!(s.v[1594] != 0.0))) {
            s.store_sub_ad_rhs(693, 15, A::mul(A::div(s.ad_value(16), s.ad_value(17)), A::offset(A::div(A::mul(s.ad_value(16), s.ad_value(19)), A::mul(A::scale(s.ad_value(17), 2.0), s.ad_value(17))), 1.0)));
        }

        if (s.v[1588] != 0.0) {
            s.store_add_ad(681, A::mul(A::scale(s.ad_value(106), 2.0), s.ad_value(693)), A::scale(s.ad_value(106), 2.0));
        }

        if (s.v[1588] != 0.0) {
            s.copy_ad(682, 681);
        }

        if (s.v[1588] != 0.0) {
            s.store_add(682, 682, 70);
        }

        s.v[1597] = if ((0.0 == 0.0) && ((s.v[682] - s.v[70]) < ((-2500.0) * 0.001))) { 1.0 } else { 0.0 };

        if ((s.v[1588] != 0.0) && (s.v[1597] != 0.0)) {
            s.store_div_from_scalar_ad(683, ((-0.001) * 0.001), A::scale(A::sub(s.ad_value(682), s.ad_value(70)), 16.0));
        }

        if ((s.v[1588] != 0.0) && (!(s.v[1597] != 0.0))) {
            s.store_scale_ad(683, A::add(A::sub(s.ad_value(682), s.ad_value(70)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(682), s.ad_value(70)), A::sub(s.ad_value(682), s.ad_value(70))), ((0.25 * 0.001) * 0.001)))), 0.5);
        }

        if (s.v[1588] != 0.0) {
            s.store_pow_ad(19, A::div(s.ad_value(74), s.ad_value(683)), A::div_from_scalar(1.0, s.ad_value(412)));
        }

        if (s.v[1588] != 0.0) {
            s.store_pow_ad(20, A::offset(s.ad_value(19), 1.0), A::neg(s.ad_value(412)));
        }

        if (s.v[1588] != 0.0) {
            s.store_mul(139, 74, 20);
        }

        if (s.v[1588] != 0.0) {
            s.store_mul_ad_lhs(142, A::add(s.ad_value(139), s.ad_value(70)), 107);
        }

        if (s.v[1588] != 0.0) {
            s.store_scale_ad(20, A::add(A::offset(s.ad_value(91), 1.0), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(91), (-1.0)), A::offset(s.ad_value(91), (-1.0))), ((0.25 * 2.0) * 2.0)))), 0.5);
        }

        if (s.v[1588] != 0.0) {
            s.store_sqrt(96, 20);
        }

        if (s.v[1588] != 0.0) {
            s.store_div_ad_lhs(12, A::offset(A::div(s.ad_value(687), A::scale(s.ad_value(96), 2.0)), 1.0), 687);
        }

        if (s.v[1588] != 0.0) {
            s.store_sub_ad_lhs(13, A::sub(s.ad_value(91), A::scale(s.ad_value(685), 2.0)), 142);
        }

        if (s.v[1588] != 0.0) {
            s.store_sub_ad_rhs(14, 13, A::ln(A::max_with_scalar(A::mul(A::scale(s.ad_value(12), 4.0), s.ad_value(96)), 1e-38)));
        }

        if (s.v[1588] != 0.0) {
            s.store_scale_ad(20, A::sub(A::offset(s.ad_value(14), (-0.201491)), A::sqrt(A::offset(A::mul(s.ad_value(14), A::offset(s.ad_value(14), 0.402982)), 2.446562))), 0.5);
        }

        if (s.v[1588] != 0.0) {
            s.copy_ad(94, 96);
        }

        s.v[1598] = if (s.v[20] <= (-68.0)) { 1.0 } else { 0.0 };

        if ((s.v[1588] != 0.0) && (s.v[1598] != 0.0)) {
            s.store_scalar(16, (-100.0));
        }

        if ((s.v[1588] != 0.0) && (s.v[1598] != 0.0)) {
            s.store_scalar(17, 20.0);
        }

        s.v[1599] = if (s.v[20] < (s.v[16] - (0.5 * s.v[17]))) { 1.0 } else { 0.0 };

        if (((s.v[1588] != 0.0) && (s.v[1598] != 0.0)) && (s.v[1599] != 0.0)) {
            s.store_ad(15, &A::limited_exp(s.ad_value(16)));
        }

        s.v[1600] = if (s.v[20] > (s.v[16] + (0.5 * s.v[17]))) { 1.0 } else { 0.0 };

        if ((((s.v[1588] != 0.0) && (s.v[1598] != 0.0)) && (!(s.v[1599] != 0.0))) && (s.v[1600] != 0.0)) {
            s.store_ad(15, &A::limited_exp(s.ad_value(20)));
        }

        if ((((s.v[1588] != 0.0) && (s.v[1598] != 0.0)) && (!(s.v[1599] != 0.0))) && (!(s.v[1600] != 0.0))) {
            s.store_div_ad_lhs(14, A::sub(s.ad_value(20), s.ad_value(16)), 17);
        }

        if ((((s.v[1588] != 0.0) && (s.v[1598] != 0.0)) && (!(s.v[1599] != 0.0))) && (!(s.v[1600] != 0.0))) {
            s.store_square(18, 14);
        }

        if ((((s.v[1588] != 0.0) && (s.v[1598] != 0.0)) && (!(s.v[1599] != 0.0))) && (!(s.v[1600] != 0.0))) {
            s.store_limited_exp_ad(15, A::add(s.ad_value(16), A::mul(s.ad_value(17), A::add(A::offset(A::scale(s.ad_value(14), 0.5), (5.0 / 64.0)), A::mul(s.ad_value(18), A::sub_from_scalar((15.0 / 16.0), A::mul(s.ad_value(18), A::sub_from_scalar(1.25, s.ad_value(18)))))))));
        }

        if ((s.v[1588] != 0.0) && (s.v[1598] != 0.0)) {
            s.store_mul_ad_rhs(692, 15, A::sub(A::sub(A::offset(s.ad_value(13), 1.0), s.ad_value(20)), A::ln(A::max_with_scalar(A::mul(A::scale(s.ad_value(12), 2.0), A::add(A::mul(A::scale(s.ad_value(15), 2.0), s.ad_value(12)), A::scale(s.ad_value(94), 2.0))), 1e-38))));
        }

        if ((s.v[1588] != 0.0) && (!(s.v[1598] != 0.0))) {
            s.store_ad(15, &A::limited_exp(s.ad_value(20)));
        }

        if ((s.v[1588] != 0.0) && (!(s.v[1598] != 0.0))) {
            s.store_div_from_scalar(95, 1.0, 94);
        }

        if ((s.v[1588] != 0.0) && (!(s.v[1598] != 0.0))) {
            s.store_sub_ad_lhs(16, A::add(A::scale(s.ad_value(15), 2.0), A::ln(A::max_with_scalar(A::mul(A::mul(A::scale(s.ad_value(15), 2.0), s.ad_value(12)), A::add(A::mul(A::scale(s.ad_value(15), 2.0), s.ad_value(12)), A::scale(s.ad_value(94), 2.0))), 1e-38))), 13);
        }

        if ((s.v[1588] != 0.0) && (!(s.v[1598] != 0.0))) {
            s.store_add_ad(17, A::offset(A::div_from_scalar(1.0, s.ad_value(15)), 2.0), A::div(A::add(s.ad_value(12), s.ad_value(95)), A::add(A::mul(s.ad_value(12), s.ad_value(15)), s.ad_value(94))));
        }

        if ((s.v[1588] != 0.0) && (!(s.v[1598] != 0.0))) {
            s.store_sub_ad_rhs(15, 15, A::div(s.ad_value(16), s.ad_value(17)));
        }

        if ((s.v[1588] != 0.0) && (!(s.v[1598] != 0.0))) {
            s.store_sub_ad_lhs(16, A::add(A::scale(s.ad_value(15), 2.0), A::ln(A::max_with_scalar(A::mul(A::mul(A::scale(s.ad_value(15), 2.0), s.ad_value(12)), A::add(A::mul(A::scale(s.ad_value(15), 2.0), s.ad_value(12)), A::scale(s.ad_value(94), 2.0))), 1e-38))), 13);
        }

        if ((s.v[1588] != 0.0) && (!(s.v[1598] != 0.0))) {
            s.store_add_ad(17, A::offset(A::div_from_scalar(1.0, s.ad_value(15)), 2.0), A::div(A::add(s.ad_value(12), s.ad_value(95)), A::add(A::mul(s.ad_value(12), s.ad_value(15)), s.ad_value(94))));
        }

        if ((s.v[1588] != 0.0) && (!(s.v[1598] != 0.0))) {
            s.store_mul_ad(18, A::div(A::add(s.ad_value(12), s.ad_value(95)), A::add(A::mul(s.ad_value(12), s.ad_value(15)), s.ad_value(94))), A::div(A::add(s.ad_value(12), s.ad_value(95)), A::add(A::mul(s.ad_value(12), s.ad_value(15)), s.ad_value(94))));
        }

        if ((s.v[1588] != 0.0) && (!(s.v[1598] != 0.0))) {
            s.store_sub_ad_lhs(19, A::sub(A::scale(A::mul(A::div_from_scalar(1.0, s.ad_value(15)), A::div_from_scalar(1.0, s.ad_value(15))), (-1.0)), A::div_from_scalar(1.0, A::mul(A::mul(A::square(s.ad_value(94)), s.ad_value(94)), A::add(A::mul(s.ad_value(12), s.ad_value(15)), s.ad_value(94))))), 18);
        }

        if ((s.v[1588] != 0.0) && (!(s.v[1598] != 0.0))) {
            s.store_sub_ad_rhs(692, 15, A::mul(A::div(s.ad_value(16), s.ad_value(17)), A::offset(A::div(A::mul(s.ad_value(16), s.ad_value(19)), A::mul(A::scale(s.ad_value(17), 2.0), s.ad_value(17))), 1.0)));
        }

        s.v[1601] = if ((1.0 == 0.0) && (s.v[91] < ((-2500.0) * 2.0))) { 1.0 } else { 0.0 };

        if ((s.v[1588] != 0.0) && (s.v[1601] != 0.0)) {
            s.store_div_from_scalar_ad(93, ((-2.0) * 2.0), A::scale(s.ad_value(91), 16.0));
        }

        if ((s.v[1588] != 0.0) && (!(s.v[1601] != 0.0))) {
            s.store_scale_ad(93, A::add(A::offset(s.ad_value(91), 1.0), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(91), (-1.0)), A::offset(s.ad_value(91), (-1.0))), ((0.25 * 2.0) * 2.0)))), 0.5);
        }

        if (s.v[1588] != 0.0) {
            s.store_sqrt(96, 93);
        }

        if (s.v[1588] != 0.0) {
            s.store_offset_ad(92, A::sub(A::sub(s.ad_value(91), s.ad_value(693)), s.ad_value(692)), (-1.0));
        }

        s.v[1602] = if ((1.0 == 0.0) && (s.v[92] < ((-2500.0) * 2.0))) { 1.0 } else { 0.0 };

        if ((s.v[1588] != 0.0) && (s.v[1602] != 0.0)) {
            s.store_div_from_scalar_ad(12, ((-2.0) * 2.0), A::scale(s.ad_value(92), 16.0));
        }

        if ((s.v[1588] != 0.0) && (!(s.v[1602] != 0.0))) {
            s.store_scale_ad(12, A::add(A::offset(s.ad_value(92), 1.0), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(92), (-1.0)), A::offset(s.ad_value(92), (-1.0))), ((0.25 * 2.0) * 2.0)))), 0.5);
        }

        if (s.v[1588] != 0.0) {
            s.store_sqrt(14, 12);
        }

        if (s.v[1588] != 0.0) {
            s.store_offset_ad(691, A::div(s.ad_value(687), A::add(s.ad_value(96), s.ad_value(14))), 1.0);
        }

        if (s.v[1588] != 0.0) {
            s.store_mul_ad_lhs(672, A::mul(A::mul(A::mul(A::scale(A::scale(A::mul(A::scale(s.ad_value(691), (2.0 * p.p2)), s.ad_value(157)), (p.p957 * 1.0 / (s.v[30]))), s.v[46]), s.ad_value(106)), s.ad_value(106)), A::mul(A::sub(s.ad_value(693), s.ad_value(692)), A::add(A::offset(s.ad_value(693), 1.0), s.ad_value(692)))), 175);
        }

        if (s.v[1588] != 0.0) {
            s.store_add(188, 672, 188);
        }

        if (s.v[1588] != 0.0) {
            s.store_scalar(696, (p.p785 * p.p1062));
        }

        if (s.v[1588] != 0.0) {
            s.store_scalar(697, (p.p799 * p.p1062));
        }

        if (s.v[1588] != 0.0) {
            s.store_scalar(698, (p.p800 * p.p1062));
        }

        if (s.v[1588] != 0.0) {
            s.store_sub_from_scalar_ad(694, s.v[30], A::scale(s.ad_value(359), 2.0));
        }

        if (s.v[1588] != 0.0) {
            s.store_square(695, 694);
        }

        if (s.v[1588] != 0.0) {
            s.store_mul_ad(367, A::scale(s.ad_value(108), 6.241457005723417e18), A::add(A::offset(s.ad_value(97), s.v[46]), s.ad_value(613)));
        }

        if (s.v[1588] != 0.0) {
            s.store_scale_ad(366, A::mul(A::mul(A::scale(s.ad_value(691), (2.0 * s.v[46])), s.ad_value(108)), s.ad_value(692)), 6.241457005723417e18);
        }

        if (s.v[1588] != 0.0) {
            s.store_mul_ad_lhs(736, A::mul(A::scale(s.ad_value(108), ((1.60219e-19 * 1.60219e-19) * 1.60219e-19)), A::abs(s.ad_value(672))), 157);
        }

        if (s.v[1588] != 0.0) {
            s.store_mul_ad_lhs(737, A::mul(A::scale(s.ad_value(108), 1.60219e-19), s.ad_value(672)), 672);
        }

        if (s.v[1588] != 0.0) {
            s.store_add_ad(738, A::add(s.ad_value(696), A::mul(s.ad_value(697), s.ad_value(366))), A::mul(A::mul(s.ad_value(698), s.ad_value(366)), s.ad_value(366)));
        }

        if (s.v[1588] != 0.0) {
            s.store_mul_ad(739, A::add(s.ad_value(366), s.ad_value(367)), A::add(s.ad_value(366), s.ad_value(367)));
        }

        if (s.v[1588] != 0.0) {
            s.store_mul_ad_lhs(740, A::scale(s.ad_value(696), 1.60219e-19), 108);
        }

        if (s.v[1588] != 0.0) {
            s.store_scale_ad(365, A::mul(A::mul(A::scale(s.ad_value(691), (2.0 * s.v[46])), s.ad_value(108)), s.ad_value(693)), 6.241457005723417e18);
        }

        if (s.v[1588] != 0.0) {
            s.store_mul_ad_rhs(13, 696, A::ln(A::max_with_scalar(A::div(A::add(s.ad_value(365), s.ad_value(367)), A::add(s.ad_value(366), s.ad_value(367))), 1e-38)));
        }

        if (s.v[1588] != 0.0) {
            s.store_mul_ad_rhs(14, 697, A::sub(s.ad_value(365), s.ad_value(366)));
        }

        if (s.v[1588] != 0.0) {
            s.store_mul_ad(15, A::scale(s.ad_value(698), 0.5), A::sub(A::square(s.ad_value(365)), A::square(s.ad_value(366))));
        }

        if (s.v[1588] != 0.0) {
            s.store_scale(16, 695, (10000000000.0 * (p.p957 * p.p2)));
        }

    }

    pub(super) fn stamp_transient_block_21(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if (s.v[1588] != 0.0) {
            s.store_add_ad(368, A::mul(A::div(s.ad_value(736), s.ad_value(12)), A::add(A::add(s.ad_value(13), s.ad_value(14)), s.ad_value(15))), A::div(A::mul(A::mul(A::div(s.ad_value(737), s.ad_value(16)), s.ad_value(363)), s.ad_value(738)), s.ad_value(739)));
        }

        if (s.v[1588] != 0.0) {
            s.store_mul_ad_lhs(17, A::mul(A::scale(s.ad_value(694), ((p.p957 * p.p2) * 10000000000.0)), s.ad_value(367)), 367);
        }

        if (s.v[1588] != 0.0) {
            s.store_mul_ad_lhs(369, A::mul(A::div(s.ad_value(740), s.ad_value(17)), s.ad_value(672)), 672);
        }

        if (s.v[1588] != 0.0) {
            s.store_add(18, 369, 368);
        }

        s.v[1603] = if (s.v[18] > 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1588] != 0.0) && (s.v[1603] != 0.0)) {
            s.store_div_ad_lhs(19, A::mul(s.ad_value(368), s.ad_value(369)), 18);
        }

        if ((s.v[1588] != 0.0) && (s.v[1603] != 0.0)) {
            s.store_offset_ad(20, A::scale(A::powf(A::sub(s.ad_value(693), s.ad_value(692)), p.p1064), p.p1063), 1.0);
        }

        if ((s.v[1588] != 0.0) && (s.v[1603] != 0.0)) {
            s.store_div(699, 19, 20);
        }

        if ((s.v[1588] != 0.0) && (!(s.v[1603] != 0.0))) {
            s.store_scalar(699, 0.0);
        }

        s.v[1604] = if (s.v[57] > 0.0) { 1.0 } else { 0.0 };

        if (s.v[1604] != 0.0) {
            s.store_mul_ad_lhs(785, A::scale(s.ad_value(187), p.p29), 196);
        }

        if (s.v[1604] != 0.0) {
            s.store_mul_ad_lhs(786, A::scale(s.ad_value(187), p.p29), 197);
        }

        if (!(s.v[1604] != 0.0)) {
            s.store_mul_ad_lhs(785, A::scale(s.ad_value(187), p.p29), 197);
        }

        if (!(s.v[1604] != 0.0)) {
            s.store_mul_ad_lhs(786, A::scale(s.ad_value(187), p.p29), 196);
        }

        s.v[1605] = if ((p.p1094 == 1.0) && (p.p1095 == 1.0)) { 1.0 } else { 0.0 };

        if (s.v[1605] != 0.0) {
            s.store_add(221, 221, 774);
        }

        if (s.v[1605] != 0.0) {
            s.store_add(224, 224, 775);
        }

        s.v[1606] = if (p.p1096 == 1.0) { 1.0 } else { 0.0 };

        if ((s.v[1605] != 0.0) && (s.v[1606] != 0.0)) {
            s.store_add(221, 221, 776);
        }

        if ((s.v[1605] != 0.0) && (s.v[1606] != 0.0)) {
            s.store_add(223, 223, 777);
        }

        s.store_mul_ad_lhs(787, A::scale(s.ad_value(187), p.p29), 198);

        s.v[1609] = if (p.p47 != 0.0) { 1.0 } else { 0.0 };

        s.v[1610] = if (p.p46 != 0.0) { 1.0 } else { 0.0 };

        s.v[1611] = if (s.v[57] > 0.0) { 1.0 } else { 0.0 };

        s.v[1612] = if ((p.p42 != 2.0) && (s.v[240] > 0.0)) { 1.0 } else { 0.0 };

        if (s.v[1612] != 0.0) {
            s.store_div_from_scalar(372, 1.0, 242);
        }

        s.v[1613] = if (((p.p42 == 1.0) && (p.p1094 == 1.0)) && (p.p1110 > 0.0)) { 1.0 } else { 0.0 };

        if ((s.v[1612] != 0.0) && (s.v[1613] != 0.0)) {
            s.store_div_from_scalar(374, 1.0, 759);
        }

        s.v[1614] = if ((p.p42 != 2.0) && (s.v[239] > 0.0)) { 1.0 } else { 0.0 };

        if (s.v[1614] != 0.0) {
            s.store_div_from_scalar(371, 1.0, 241);
        }

        s.v[1615] = if (((p.p42 == 1.0) && (p.p1094 == 1.0)) && (p.p1112 > 0.0)) { 1.0 } else { 0.0 };

        if ((s.v[1614] != 0.0) && (s.v[1615] != 0.0)) {
            s.store_div_from_scalar(373, 1.0, 761);
        }

        s.v[1616] = if (p.p7 == 0.0) { 1.0 } else { 0.0 };

        s.v[1619] = if (p.p7 == 2.0) { 1.0 } else { 0.0 };

        if ((!(s.v[1616] != 0.0)) && (s.v[1619] != 0.0)) {
            s.copy_ad(1617, 254);
        }

        if ((!(s.v[1616] != 0.0)) && (s.v[1619] != 0.0)) {
            s.store_div_ad_lhs(1618, A::square(s.ad_value(254)), 252);
        }

        if ((!(s.v[1616] != 0.0)) && (!(s.v[1619] != 0.0))) {
            s.copy_ad(1617, 252);
        }

        if ((!(s.v[1616] != 0.0)) && (!(s.v[1619] != 0.0))) {
            s.copy_ad(1618, 252);
        }

        s.v[1620] = if (p.p7 == 3.0) { 1.0 } else { 0.0 };

        s.v[1621] = if ((p.p49 != 0.0) && (p.p909 > 0.0)) { 1.0 } else { 0.0 };

        if (s.v[1621] != 0.0) {
            s.store_ad(749, &A::mul(A::mul(A::mul(s.ad_value(187), s.ad_value(57)), s.ad_value(188)), A::voltage(ctx, &nodes, Some(5), Some(7))));
        }

        s.v[1622] = if ((p.p42 != 2.0) && (s.v[240] > 0.0)) { 1.0 } else { 0.0 };

        s.v[1623] = if (((p.p42 == 1.0) && (p.p1094 == 1.0)) && (p.p1110 > 0.0)) { 1.0 } else { 0.0 };

        if (((s.v[1621] != 0.0) && (s.v[1622] != 0.0)) && (s.v[1623] != 0.0)) {
            s.store_add_ad(749, A::add(s.ad_value(749), A::mul(A::square(A::voltage(ctx, &nodes, Some(0), Some(6))), s.ad_value(372))), A::mul(A::square(A::voltage(ctx, &nodes, Some(6), Some(5))), s.ad_value(374)));
        }

        if (((s.v[1621] != 0.0) && (s.v[1622] != 0.0)) && (!(s.v[1623] != 0.0))) {
            s.store_add_ad_rhs(749, 749, A::mul(A::square(A::voltage(ctx, &nodes, Some(0), Some(6))), s.ad_value(372)));
        }

        s.v[1624] = if ((p.p42 != 2.0) && (s.v[239] > 0.0)) { 1.0 } else { 0.0 };

        s.v[1625] = if (((p.p42 == 1.0) && (p.p1094 == 1.0)) && (p.p1112 > 0.0)) { 1.0 } else { 0.0 };

        if (((s.v[1621] != 0.0) && (s.v[1624] != 0.0)) && (s.v[1625] != 0.0)) {
            s.store_add_ad(749, A::add(s.ad_value(749), A::mul(A::square(A::voltage(ctx, &nodes, Some(2), Some(8))), s.ad_value(371))), A::mul(A::square(A::voltage(ctx, &nodes, Some(8), Some(7))), s.ad_value(373)));
        }

        if (((s.v[1621] != 0.0) && (s.v[1624] != 0.0)) && (!(s.v[1625] != 0.0))) {
            s.store_add_ad_rhs(749, 749, A::mul(A::square(A::voltage(ctx, &nodes, Some(2), Some(8))), s.ad_value(371)));
        }

        s.v[1626] = if (p.p8 != 0.0) { 1.0 } else { 0.0 };

        s.v[1627] = if (p.p8 != 0.0) { 1.0 } else { 0.0 };

        s.v[1628] = if (p.p1097 == 0.0) { 1.0 } else { 0.0 };

        s.v[1629] = if ((p.p8 != 0.0) && (p.p1097 == 1.0)) { 1.0 } else { 0.0 };

        s.v[1630] = if ((p.p8 != 0.0) && (p.p1097 == 1.0)) { 1.0 } else { 0.0 };

    }

    pub(super) fn stamp_reactive_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        s.v[485] = 0.0;

        s.v[466] = 0.0;

        s.v[535] = 0.0;

        s.v[505] = 0.0;

        s.v[512] = 0.0;

        s.v[510] = 0.0;

        s.v[467] = 0.0;

        s.v[649] = 0.0;

        s.v[661] = 0.0;

        s.v[669] = 0.0;

        s.v[606] = 0.0;

        s.v[610] = 0.0;

        s.v[616] = 0.0;

        s.v[620] = 0.0;

        s.v[624] = 0.0;

        s.v[628] = 0.0;

        s.v[634] = 0.0;

        s.v[638] = 0.0;

        s.v[491] = 0.0;

        s.v[540] = 0.0;

        s.v[414] = 0.0;

        s.v[400] = 0.0;

        s.v[406] = 0.0;

        s.v[501] = 0.0;

        s.v[650] = 0.0;

        s.v[670] = 0.0;

        s.v[607] = 0.0;

        s.v[613] = 0.0;

        s.v[617] = 0.0;

        s.v[621] = 0.0;

        s.v[625] = 0.0;

        s.v[631] = 0.0;

        s.v[635] = 0.0;

        s.v[639] = 0.0;

        s.v[762] = 1.0;

        s.v[421] = 0.0;

        s.v[518] = 0.0;

        s.v[498] = 0.0;

        s.v[515] = 0.0;

        s.v[509] = 0.0;

        s.v[410] = 0.0;

        s.v[688] = 0.0;

        s.v[690] = 0.0;

        s.v[671] = 0.0;

        s.v[608] = 0.0;

        s.v[614] = 0.0;

        s.v[618] = 0.0;

        s.v[622] = 0.0;

        s.v[626] = 0.0;

        s.v[632] = 0.0;

        s.v[636] = 0.0;

        s.v[759] = 0.0;

        s.v[763] = 1.0;

        s.v[460] = 0.0;

        s.v[165] = 0.0;

        s.v[398] = 0.0;

        s.v[402] = 0.0;

        s.v[404] = 0.0;

        s.v[461] = 0.0;

        s.v[689] = 0.0;

        s.v[605] = 0.0;

        s.v[609] = 0.0;

        s.v[615] = 0.0;

        s.v[619] = 0.0;

        s.v[623] = 0.0;

        s.v[627] = 0.0;

        s.v[633] = 0.0;

        s.v[637] = 0.0;

        s.v[761] = 0.0;

        s.v[629] = 0.0;

        s.v[630] = 0.0;

        s.v[247] = 0.0;

        s.v[246] = 0.0;

        s.v[249] = 0.0;

        s.v[248] = 0.0;

        s.v[782] = 1.0;

        s.v[783] = 1.0;

        s.v[372] = 0.0;

        s.v[371] = 0.0;

        s.v[374] = 0.0;

        s.v[373] = 0.0;

        s.v[67] = 0.0;

        s.v[71] = 0.0;

        s.v[750] = 0.0;

        s.v[147] = 0.0;

        s.v[183] = 0.0;

        s.v[416] = 0.0;

        s.v[552] = 0.0;

        s.v[557] = 0.0;

        s.v[760] = 0.0;

        s.v[859] = if (p.p39 == 1.0) { 1.0 } else { 0.0 };

        if (s.v[859] != 0.0) {
            s.store_scalar(187, 1.0);
        }

        if (!(s.v[859] != 0.0)) {
            s.store_scalar(187, (-1.0));
        }

        s.v[26] = (p.p110 * 8.85418e-12);

        s.v[27] = (p.p111 * 8.85418e-12);

        s.v[46] = ((p.p111 * 8.85418e-12) / p.p77);

        s.v[47] = (p.p110 / p.p111);

        s.v[860] = if !(if self.param_given[78] { 1.0 } else { 0.0 } != 0.0) { 1.0 } else { 0.0 };

        if (s.v[860] != 0.0) {
            s.store_scalar(229, (((p.p77 * p.p111) / 3.9) - p.p79));
        }

        if (!(s.v[860] != 0.0)) {
            s.store_scalar(229, p.p78);
        }

        s.v[99] = (p.p0 * p.p52);

        s.v[101] = (p.p1 * p.p53);

        s.v[98] = (s.v[99] + p.p54);

        s.v[456] = (s.v[101] / p.p2);

        s.v[100] = (s.v[456] + p.p56);

        s.v[457] = ((s.v[98]) as f64).powf((-p.p61));

        s.v[458] = ((s.v[100]) as f64).powf((-p.p62));

        s.v[459] = (s.v[457] * s.v[458]);

        s.v[39] = (((p.p57 + (p.p58 * s.v[457])) + (p.p59 * s.v[458])) + (p.p60 * s.v[459]));

        s.v[463] = ((s.v[98]) as f64).powf((-p.p67));

        s.v[464] = ((s.v[100]) as f64).powf((-p.p68));

        s.v[465] = (s.v[463] * s.v[464]);

        s.v[40] = (((p.p63 + (p.p64 * s.v[463])) + (p.p65 * s.v[464])) + (p.p66 * s.v[465]));

        s.v[30] = (s.v[98] - (2.0 * s.v[39]));

        s.v[29] = (s.v[100] - (2.0 * s.v[40]));

        s.v[43] = (((p.p69 + (p.p70 * s.v[457])) + (p.p71 * s.v[458])) + (p.p72 * s.v[459]));

        s.v[44] = (((p.p73 + (p.p74 * s.v[463])) + (p.p75 * s.v[464])) + (p.p76 * s.v[465]));

        s.v[34] = (s.v[98] - (2.0 * s.v[43]));

        s.v[33] = (s.v[100] - (2.0 * s.v[44]));

        s.v[45] = (((p.p138 + (p.p74 / ((s.v[98]) as f64).powf(p.p67))) + (p.p75 / ((s.v[100]) as f64).powf(p.p68))) + ((p.p76 / ((s.v[98]) as f64).powf(p.p67)) / ((s.v[100]) as f64).powf(p.p68)));

        s.v[35] = (s.v[100] - (2.0 * s.v[45]));

        s.v[469] = (1e-6 / s.v[30]);

        s.v[470] = (1e-6 / s.v[29]);

        s.v[472] = (1e-6 / s.v[34]);

        s.v[473] = (1e-6 / s.v[33]);

        s.v[474] = (1e-6 / p.p51);

        s.v[475] = (1e-6 / p.p55);

        s.v[471] = (s.v[469] * s.v[470]);

        s.v[460] = s.v[457];

        s.v[466] = s.v[463];

        s.v[872] = if (p.p818 != 0.0) { 1.0 } else { 0.0 };

        s.v[873] = if (p.p818 <= (-s.v[98])) { 1.0 } else { 0.0 };

        if ((s.v[872] != 0.0) && (!(s.v[873] != 0.0))) {
            s.store_scalar(460, (((s.v[98] + p.p818)) as f64).powf((-p.p61)));
        }

        if ((s.v[872] != 0.0) && (!(s.v[873] != 0.0))) {
            s.store_scalar(466, (((s.v[98] + p.p818)) as f64).powf((-p.p67)));
        }

        s.v[461] = s.v[458];

        s.v[467] = s.v[464];

        s.v[874] = if (p.p819 != 0.0) { 1.0 } else { 0.0 };

        s.v[875] = if (p.p819 <= (-s.v[100])) { 1.0 } else { 0.0 };

        if ((s.v[874] != 0.0) && (!(s.v[875] != 0.0))) {
            s.store_scalar(461, (((s.v[100] + p.p819)) as f64).powf((-p.p62)));
        }

        if ((s.v[874] != 0.0) && (!(s.v[875] != 0.0))) {
            s.store_scalar(467, (((s.v[100] + p.p819)) as f64).powf((-p.p68)));
        }

        s.store_mul(462, 460, 461);

        s.store_add_ad(41, A::add(A::offset(A::scale(s.ad_value(460), p.p58), p.p57), A::scale(s.ad_value(461), p.p59)), A::scale(s.ad_value(462), p.p60));

        s.store_mul(468, 466, 467);

        s.store_add_ad(42, A::add(A::offset(A::scale(s.ad_value(466), p.p64), p.p63), A::scale(s.ad_value(467), p.p65)), A::scale(s.ad_value(468), p.p66));

        s.store_offset_ad(32, A::sub_from_scalar(s.v[98], A::scale(s.ad_value(41), 2.0)), p.p818);

        s.store_offset_ad(31, A::sub_from_scalar(s.v[100], A::scale(s.ad_value(42), 2.0)), p.p819);

        s.v[878] = if (p.p817 == 1.0) { 1.0 } else { 0.0 };

        if (s.v[878] != 0.0) {
            s.store_div_from_scalar(476, 1e-6, 32);
        }

        if (s.v[878] != 0.0) {
            s.store_div_from_scalar(477, 1e-6, 31);
        }

        if (!(s.v[878] != 0.0)) {
            s.store_div_from_scalar(476, 1.0, 32);
        }

        if (!(s.v[878] != 0.0)) {
            s.store_div_from_scalar(477, 1.0, 31);
        }

        s.store_mul(478, 476, 477);

        s.store_add_ad(482, A::add(A::offset(A::scale(s.ad_value(476), p.p117), p.p116), A::scale(s.ad_value(477), p.p118)), A::scale(s.ad_value(478), p.p119));

        s.store_add_ad(549, A::add(A::offset(A::scale(s.ad_value(476), p.p127), p.p126), A::scale(s.ad_value(477), p.p128)), A::scale(s.ad_value(478), p.p129));

        s.store_add_ad(480, A::add(A::offset(A::scale(s.ad_value(476), p.p140), p.p139), A::scale(s.ad_value(477), p.p141)), A::scale(s.ad_value(478), p.p142));

        s.store_add_ad(481, A::add(A::offset(A::scale(s.ad_value(476), p.p89), p.p80), A::scale(s.ad_value(477), p.p90)), A::scale(s.ad_value(478), p.p91));

        s.store_add_ad(550, A::add(A::offset(A::scale(s.ad_value(476), p.p101), p.p92), A::scale(s.ad_value(477), p.p102)), A::scale(s.ad_value(478), p.p103));

        s.store_add_ad(479, A::add(A::offset(A::scale(s.ad_value(476), p.p105), p.p104), A::scale(s.ad_value(477), p.p106)), A::scale(s.ad_value(478), p.p107));

        s.store_add_ad(483, A::add(A::offset(A::scale(s.ad_value(476), p.p210), p.p209), A::scale(s.ad_value(477), p.p211)), A::scale(s.ad_value(478), p.p212));

        s.store_add_ad(488, A::add(A::offset(A::scale(s.ad_value(476), p.p220), p.p213), A::scale(s.ad_value(477), p.p221)), A::scale(s.ad_value(478), p.p222));

        s.store_add_ad(484, A::add(A::offset(A::scale(s.ad_value(476), p.p226), p.p223), A::scale(s.ad_value(477), p.p227)), A::scale(s.ad_value(478), p.p228));

        s.store_add_ad(487, A::add(A::offset(A::scale(s.ad_value(476), p.p236), p.p233), A::scale(s.ad_value(477), p.p237)), A::scale(s.ad_value(478), p.p238));

        s.store_add_ad(116, A::add(A::offset(A::scale(s.ad_value(476), p.p144), p.p143), A::scale(s.ad_value(477), p.p145)), A::scale(s.ad_value(478), p.p146));

        s.store_add_ad(117, A::add(A::offset(A::scale(s.ad_value(476), p.p148), p.p147), A::scale(s.ad_value(477), p.p149)), A::scale(s.ad_value(478), p.p150));

        s.store_add_ad(118, A::add(A::offset(A::scale(s.ad_value(476), p.p152), p.p151), A::scale(s.ad_value(477), p.p153)), A::scale(s.ad_value(478), p.p154));

        s.store_add_ad(119, A::add(A::offset(A::scale(s.ad_value(476), p.p156), p.p155), A::scale(s.ad_value(477), p.p157)), A::scale(s.ad_value(478), p.p158));

        s.store_add_ad(120, A::add(A::offset(A::scale(s.ad_value(476), p.p160), p.p159), A::scale(s.ad_value(477), p.p161)), A::scale(s.ad_value(478), p.p162));

        s.store_add_ad(121, A::add(A::offset(A::scale(s.ad_value(476), p.p164), p.p163), A::scale(s.ad_value(477), p.p165)), A::scale(s.ad_value(478), p.p166));

        s.store_add_ad(494, A::add(A::offset(A::scale(s.ad_value(476), p.p202), p.p195), A::scale(s.ad_value(477), p.p203)), A::scale(s.ad_value(478), p.p204));

        s.store_add_ad(495, A::add(A::offset(A::scale(s.ad_value(476), p.p192), p.p185), A::scale(s.ad_value(477), p.p193)), A::scale(s.ad_value(478), p.p194));

        s.store_add_ad(538, A::add(A::offset(A::scale(s.ad_value(476), p.p113), p.p112), A::scale(s.ad_value(477), p.p114)), A::scale(s.ad_value(478), p.p115));

        s.store_add_ad(489, A::add(A::offset(A::scale(s.ad_value(476), p.p168), p.p167), A::scale(s.ad_value(477), p.p169)), A::scale(s.ad_value(478), p.p170));

        s.store_add_ad(490, A::add(A::offset(A::scale(s.ad_value(476), p.p172), p.p171), A::scale(s.ad_value(477), p.p173)), A::scale(s.ad_value(478), p.p174));

        s.store_add_ad(493, A::add(A::offset(A::scale(s.ad_value(476), p.p182), p.p180), A::scale(s.ad_value(477), p.p183)), A::scale(s.ad_value(478), p.p184));

        s.store_add_ad(496, A::add(A::offset(A::scale(s.ad_value(476), p.p254), p.p253), A::scale(s.ad_value(477), p.p255)), A::scale(s.ad_value(478), p.p256));

        s.store_add_ad(497, A::add(A::offset(A::scale(s.ad_value(476), p.p276), p.p273), A::scale(s.ad_value(477), p.p277)), A::scale(s.ad_value(478), p.p278));

        s.store_add_ad(504, A::add(A::offset(A::scale(s.ad_value(476), p.p291), p.p284), A::scale(s.ad_value(477), p.p292)), A::scale(s.ad_value(478), p.p293));

        s.store_add_ad(508, A::add(A::offset(A::scale(s.ad_value(476), p.p311), p.p308), A::scale(s.ad_value(477), p.p312)), A::scale(s.ad_value(478), p.p313));

        s.store_add_ad(507, A::add(A::offset(A::scale(s.ad_value(476), p.p299), p.p298), A::scale(s.ad_value(477), p.p300)), A::scale(s.ad_value(478), p.p301));

        s.store_add_ad(511, A::add(A::offset(A::scale(s.ad_value(476), p.p319), p.p318), A::scale(s.ad_value(477), p.p320)), A::scale(s.ad_value(478), p.p321));

        s.store_add_ad(514, A::add(A::offset(A::scale(s.ad_value(476), p.p333), p.p326), A::scale(s.ad_value(477), p.p334)), A::scale(s.ad_value(478), p.p335));

        s.store_add_ad(539, A::add(A::offset(A::scale(s.ad_value(476), p.p343), p.p340), A::scale(s.ad_value(477), p.p344)), A::scale(s.ad_value(478), p.p345));

        s.store_add_ad(542, A::add(A::offset(A::scale(s.ad_value(476), p.p354), p.p351), A::scale(s.ad_value(477), p.p355)), A::scale(s.ad_value(478), p.p356));

        s.store_add_ad(531, A::add(A::offset(A::scale(s.ad_value(476), p.p394), p.p393), A::scale(s.ad_value(477), p.p395)), A::scale(s.ad_value(478), p.p396));

        s.store_add_ad(530, A::add(A::offset(A::scale(s.ad_value(476), p.p404), p.p403), A::scale(s.ad_value(477), p.p405)), A::scale(s.ad_value(478), p.p406));

        s.store_add_ad(526, A::add(A::offset(A::scale(s.ad_value(476), p.p376), p.p375), A::scale(s.ad_value(477), p.p377)), A::scale(s.ad_value(478), p.p378));

        s.store_add_ad(543, A::add(A::offset(A::scale(s.ad_value(476), p.p380), p.p379), A::scale(s.ad_value(477), p.p381)), A::scale(s.ad_value(478), p.p382));

        s.store_add_ad(527, A::add(A::offset(A::scale(s.ad_value(476), p.p386), p.p385), A::scale(s.ad_value(477), p.p387)), A::scale(s.ad_value(478), p.p388));

        s.store_add_ad(529, A::add(A::offset(A::scale(s.ad_value(476), p.p390), p.p389), A::scale(s.ad_value(477), p.p391)), A::scale(s.ad_value(478), p.p392));

        s.store_add_ad(528, A::add(A::offset(A::scale(s.ad_value(476), p.p400), p.p399), A::scale(s.ad_value(477), p.p401)), A::scale(s.ad_value(478), p.p402));

        s.store_add_ad(532, A::add(A::offset(A::scale(s.ad_value(476), p.p416), p.p413), A::scale(s.ad_value(477), p.p417)), A::scale(s.ad_value(478), p.p418));

        s.store_add_ad(533, A::add(A::offset(A::scale(s.ad_value(476), p.p410), p.p409), A::scale(s.ad_value(477), p.p411)), A::scale(s.ad_value(478), p.p412));

        s.store_add_ad(534, A::add(A::offset(A::scale(s.ad_value(476), p.p435), p.p434), A::scale(s.ad_value(477), p.p436)), A::scale(s.ad_value(478), p.p437));

        s.store_add_ad(517, A::add(A::offset(A::scale(s.ad_value(476), p.p463), p.p460), A::scale(s.ad_value(477), p.p464)), A::scale(s.ad_value(478), p.p465));

        s.store_add_ad(520, A::add(A::offset(A::scale(s.ad_value(476), p.p471), p.p470), A::scale(s.ad_value(477), p.p472)), A::scale(s.ad_value(478), p.p473));

        s.store_add_ad(521, A::add(A::offset(A::scale(s.ad_value(476), p.p358), p.p357), A::scale(s.ad_value(477), p.p359)), A::scale(s.ad_value(478), p.p360));

        s.store_add_ad(522, A::add(A::offset(A::scale(s.ad_value(476), p.p362), p.p361), A::scale(s.ad_value(477), p.p363)), A::scale(s.ad_value(478), p.p364));

        s.store_add_ad(523, A::add(A::offset(A::scale(s.ad_value(476), p.p366), p.p365), A::scale(s.ad_value(477), p.p367)), A::scale(s.ad_value(478), p.p368));

        s.store_add_ad(524, A::add(A::offset(A::scale(s.ad_value(476), p.p371), p.p370), A::scale(s.ad_value(477), p.p372)), A::scale(s.ad_value(478), p.p373));

        s.store_add_ad(525, A::add(A::offset(A::scale(s.ad_value(476), p.p481), p.p478), A::scale(s.ad_value(477), p.p482)), A::scale(s.ad_value(478), p.p483));

        s.store_add_ad(537, A::add(A::offset(A::scale(s.ad_value(476), p.p475), p.p474), A::scale(s.ad_value(477), p.p476)), A::scale(s.ad_value(478), p.p477));

        s.store_add_ad(500, A::add(A::offset(A::scale(s.ad_value(476), p.p240), p.p239), A::scale(s.ad_value(477), p.p241)), A::scale(s.ad_value(478), p.p242));

        s.store_add_ad(164, A::add(A::offset(A::scale(s.ad_value(476), p.p420), p.p419), A::scale(s.ad_value(477), p.p421)), A::scale(s.ad_value(478), p.p422));

        s.store_add_ad(503, A::add(A::offset(A::scale(s.ad_value(476), p.p260), p.p259), A::scale(s.ad_value(477), p.p261)), A::scale(s.ad_value(478), p.p262));

        s.store_add_ad(544, A::add(A::offset(A::scale(s.ad_value(476), p.p667), p.p666), A::scale(s.ad_value(477), p.p668)), A::scale(s.ad_value(478), p.p669));

        s.store_add_ad(545, A::add(A::offset(A::scale(s.ad_value(476), p.p675), p.p674), A::scale(s.ad_value(477), p.p676)), A::scale(s.ad_value(478), p.p677));

        s.store_add_ad(546, A::add(A::offset(A::scale(s.ad_value(476), p.p679), p.p678), A::scale(s.ad_value(477), p.p680)), A::scale(s.ad_value(478), p.p681));

        s.store_add_ad(547, A::add(A::offset(A::scale(s.ad_value(476), p.p683), p.p682), A::scale(s.ad_value(477), p.p684)), A::scale(s.ad_value(478), p.p685));

        s.store_add_ad(548, A::add(A::offset(A::scale(s.ad_value(476), p.p687), p.p686), A::scale(s.ad_value(477), p.p688)), A::scale(s.ad_value(478), p.p689));

        s.store_add_ad(551, A::add(A::offset(A::scale(s.ad_value(476), p.p489), p.p484), A::scale(s.ad_value(477), p.p490)), A::scale(s.ad_value(478), p.p491));

        s.store_add_ad(554, A::add(A::offset(A::scale(s.ad_value(476), p.p497), p.p494), A::scale(s.ad_value(477), p.p498)), A::scale(s.ad_value(478), p.p499));

        s.store_add_ad(578, A::add(A::offset(A::scale(s.ad_value(476), p.p936), p.p935), A::scale(s.ad_value(477), p.p937)), A::scale(s.ad_value(478), p.p938));

        s.store_add_ad(579, A::add(A::offset(A::scale(s.ad_value(476), p.p940), p.p939), A::scale(s.ad_value(477), p.p941)), A::scale(s.ad_value(478), p.p942));

        s.store_add_ad(580, A::add(A::offset(A::scale(s.ad_value(476), p.p944), p.p943), A::scale(s.ad_value(477), p.p945)), A::scale(s.ad_value(478), p.p946));

        s.store_add_ad(559, A::add(A::offset(A::scale(s.ad_value(476), p.p633), p.p630), A::scale(s.ad_value(477), p.p634)), A::scale(s.ad_value(478), p.p635));

        s.store_add_ad(560, A::add(A::offset(A::scale(s.ad_value(476), p.p637), p.p636), A::scale(s.ad_value(477), p.p638)), A::scale(s.ad_value(478), p.p639));

        s.store_add_ad(561, A::add(A::offset(A::scale(s.ad_value(476), p.p641), p.p640), A::scale(s.ad_value(477), p.p642)), A::scale(s.ad_value(478), p.p643));

        s.store_add_ad(562, A::add(A::offset(A::scale(s.ad_value(476), p.p645), p.p644), A::scale(s.ad_value(477), p.p646)), A::scale(s.ad_value(478), p.p647));

        s.store_add_ad(563, A::add(A::offset(A::scale(s.ad_value(476), p.p651), p.p648), A::scale(s.ad_value(477), p.p652)), A::scale(s.ad_value(478), p.p653));

        s.store_add_ad(564, A::add(A::offset(A::scale(s.ad_value(476), p.p655), p.p654), A::scale(s.ad_value(477), p.p656)), A::scale(s.ad_value(478), p.p657));

        s.store_add_ad(565, A::add(A::offset(A::scale(s.ad_value(476), p.p659), p.p658), A::scale(s.ad_value(477), p.p660)), A::scale(s.ad_value(478), p.p661));

        s.store_add_ad(566, A::add(A::offset(A::scale(s.ad_value(476), p.p663), p.p662), A::scale(s.ad_value(477), p.p664)), A::scale(s.ad_value(478), p.p665));

        s.store_add_ad(567, A::add(A::offset(A::scale(s.ad_value(476), p.p825), p.p824), A::scale(s.ad_value(477), p.p826)), A::scale(s.ad_value(478), p.p827));

        s.store_add_ad(568, A::add(A::offset(A::scale(s.ad_value(476), p.p830), p.p829), A::scale(s.ad_value(477), p.p831)), A::scale(s.ad_value(478), p.p832));

        s.store_add_ad(569, A::add(A::offset(A::scale(s.ad_value(476), p.p835), p.p834), A::scale(s.ad_value(477), p.p836)), A::scale(s.ad_value(478), p.p837));

        s.store_add_ad(570, A::add(A::offset(A::scale(s.ad_value(476), p.p839), p.p838), A::scale(s.ad_value(477), p.p840)), A::scale(s.ad_value(478), p.p841));

        s.store_add_ad(577, A::add(A::offset(A::scale(s.ad_value(476), p.p844), p.p843), A::scale(s.ad_value(477), p.p845)), A::scale(s.ad_value(478), p.p846));

        s.store_add_ad(571, A::add(A::offset(A::scale(s.ad_value(476), p.p848), p.p847), A::scale(s.ad_value(477), p.p849)), A::scale(s.ad_value(478), p.p850));

        s.store_add_ad(572, A::add(A::offset(A::scale(s.ad_value(476), p.p853), p.p852), A::scale(s.ad_value(477), p.p854)), A::scale(s.ad_value(478), p.p855));

        s.store_add_ad(573, A::add(A::offset(A::scale(s.ad_value(476), p.p857), p.p856), A::scale(s.ad_value(477), p.p858)), A::scale(s.ad_value(478), p.p859));

        s.store_add_ad(574, A::add(A::offset(A::scale(s.ad_value(476), p.p863), p.p862), A::scale(s.ad_value(477), p.p864)), A::scale(s.ad_value(478), p.p865));

        s.store_add_ad(575, A::add(A::offset(A::scale(s.ad_value(476), p.p878), p.p877), A::scale(s.ad_value(477), p.p879)), A::scale(s.ad_value(478), p.p880));

        s.store_add_ad(576, A::add(A::offset(A::scale(s.ad_value(476), p.p886), p.p885), A::scale(s.ad_value(477), p.p887)), A::scale(s.ad_value(478), p.p888));

        s.store_add_ad(581, A::add(A::offset(A::scale(s.ad_value(476), p.p564), p.p537), A::scale(s.ad_value(477), p.p565)), A::scale(s.ad_value(478), p.p566));

        s.store_add_ad(582, A::add(A::offset(A::scale(s.ad_value(476), p.p567), p.p538), A::scale(s.ad_value(477), p.p568)), A::scale(s.ad_value(478), p.p569));

        s.store_add_ad(583, A::add(A::offset(A::scale(s.ad_value(476), p.p570), p.p539), A::scale(s.ad_value(477), p.p571)), A::scale(s.ad_value(478), p.p572));

        s.store_add_ad(584, A::add(A::offset(A::scale(s.ad_value(476), p.p573), p.p540), A::scale(s.ad_value(477), p.p574)), A::scale(s.ad_value(478), p.p575));

        s.store_add_ad(585, A::add(A::offset(A::scale(s.ad_value(476), p.p576), p.p541), A::scale(s.ad_value(477), p.p577)), A::scale(s.ad_value(478), p.p578));

        s.store_add_ad(586, A::add(A::offset(A::scale(s.ad_value(476), p.p579), p.p533), A::scale(s.ad_value(477), p.p580)), A::scale(s.ad_value(478), p.p581));

        s.store_add_ad(587, A::add(A::offset(A::scale(s.ad_value(476), p.p582), p.p534), A::scale(s.ad_value(477), p.p583)), A::scale(s.ad_value(478), p.p584));

        s.store_add_ad(588, A::add(A::offset(A::scale(s.ad_value(476), p.p585), p.p535), A::scale(s.ad_value(477), p.p586)), A::scale(s.ad_value(478), p.p587));

        s.store_add_ad(589, A::add(A::offset(A::scale(s.ad_value(476), p.p588), p.p536), A::scale(s.ad_value(477), p.p589)), A::scale(s.ad_value(478), p.p590));

        s.store_add_ad(590, A::add(A::offset(A::scale(s.ad_value(476), p.p591), p.p542), A::scale(s.ad_value(477), p.p592)), A::scale(s.ad_value(478), p.p593));

        s.store_add_ad(591, A::add(A::offset(A::scale(s.ad_value(476), p.p594), p.p543), A::scale(s.ad_value(477), p.p595)), A::scale(s.ad_value(478), p.p596));

    }

    pub(super) fn stamp_reactive_block_1(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        s.store_add_ad(592, A::add(A::offset(A::scale(s.ad_value(476), p.p597), p.p544), A::scale(s.ad_value(477), p.p598)), A::scale(s.ad_value(478), p.p599));

        s.store_add_ad(593, A::add(A::offset(A::scale(s.ad_value(476), p.p600), p.p545), A::scale(s.ad_value(477), p.p601)), A::scale(s.ad_value(478), p.p602));

        s.store_add_ad(594, A::add(A::offset(A::scale(s.ad_value(476), p.p603), p.p546), A::scale(s.ad_value(477), p.p604)), A::scale(s.ad_value(478), p.p605));

        s.store_add_ad(595, A::add(A::offset(A::scale(s.ad_value(476), p.p606), p.p547), A::scale(s.ad_value(477), p.p607)), A::scale(s.ad_value(478), p.p608));

        s.store_add_ad(596, A::add(A::offset(A::scale(s.ad_value(476), p.p609), p.p548), A::scale(s.ad_value(477), p.p610)), A::scale(s.ad_value(478), p.p611));

        s.store_add_ad(597, A::add(A::offset(A::scale(s.ad_value(476), p.p612), p.p549), A::scale(s.ad_value(477), p.p613)), A::scale(s.ad_value(478), p.p614));

        s.store_add_ad(598, A::add(A::offset(A::scale(s.ad_value(476), p.p615), p.p550), A::scale(s.ad_value(477), p.p616)), A::scale(s.ad_value(478), p.p617));

        s.store_add_ad(599, A::add(A::offset(A::scale(s.ad_value(476), p.p618), p.p553), A::scale(s.ad_value(477), p.p619)), A::scale(s.ad_value(478), p.p620));

        s.store_add_ad(454, A::add(A::offset(A::scale(s.ad_value(476), p.p870), p.p867), A::scale(s.ad_value(477), p.p871)), A::scale(s.ad_value(478), p.p872));

        s.store_add_ad(455, A::add(A::offset(A::scale(s.ad_value(476), p.p874), p.p873), A::scale(s.ad_value(477), p.p875)), A::scale(s.ad_value(478), p.p876));

        s.store_add_ad(453, A::add(A::offset(A::scale(s.ad_value(476), p.p430), p.p425), A::scale(s.ad_value(477), p.p431)), A::scale(s.ad_value(478), p.p432));

        s.store_add_ad(148, A::add(A::offset(A::scale(s.ad_value(476), p.p445), p.p444), A::scale(s.ad_value(477), p.p446)), A::scale(s.ad_value(478), p.p447));

        s.store_add_ad(149, A::add(A::offset(A::scale(s.ad_value(476), p.p449), p.p448), A::scale(s.ad_value(477), p.p450)), A::scale(s.ad_value(478), p.p451));

        s.store_add_ad(151, A::add(A::offset(A::scale(s.ad_value(476), p.p453), p.p452), A::scale(s.ad_value(477), p.p454)), A::scale(s.ad_value(478), p.p455));

        s.store_add_ad(152, A::add(A::offset(A::scale(s.ad_value(476), p.p457), p.p456), A::scale(s.ad_value(477), p.p458)), A::scale(s.ad_value(478), p.p459));

        s.store_add_ad(605, A::add(A::offset(A::scale(s.ad_value(476), p.p1047), p.p1046), A::scale(s.ad_value(477), p.p1048)), A::scale(s.ad_value(478), p.p1049));

        s.store_add_ad(606, A::add(A::offset(A::scale(s.ad_value(476), p.p1055), p.p1054), A::scale(s.ad_value(477), p.p1056)), A::scale(s.ad_value(478), p.p1057));

        s.store_add_ad(607, A::add(A::offset(A::scale(s.ad_value(476), p.p1051), p.p1050), A::scale(s.ad_value(477), p.p1052)), A::scale(s.ad_value(478), p.p1053));

        s.store_add_ad(608, A::add(A::offset(A::scale(s.ad_value(476), p.p1059), p.p1058), A::scale(s.ad_value(477), p.p1060)), A::scale(s.ad_value(478), p.p1061));

        s.store_add_ad(612, A::add(A::offset(A::scale(s.ad_value(476), p.p967), p.p966), A::scale(s.ad_value(477), p.p968)), A::scale(s.ad_value(478), p.p969));

        s.store_add_ad(686, A::add(A::offset(A::scale(s.ad_value(476), p.p963), p.p962), A::scale(s.ad_value(477), p.p964)), A::scale(s.ad_value(478), p.p965));

        s.store_add_ad(613, A::add(A::offset(A::scale(s.ad_value(476), p.p971), p.p970), A::scale(s.ad_value(477), p.p972)), A::scale(s.ad_value(478), p.p973));

        s.store_add_ad(614, A::add(A::offset(A::scale(s.ad_value(476), p.p975), p.p974), A::scale(s.ad_value(477), p.p976)), A::scale(s.ad_value(478), p.p977));

        s.store_add_ad(615, A::add(A::offset(A::scale(s.ad_value(476), p.p979), p.p978), A::scale(s.ad_value(477), p.p980)), A::scale(s.ad_value(478), p.p981));

        s.store_add_ad(616, A::add(A::offset(A::scale(s.ad_value(476), p.p983), p.p982), A::scale(s.ad_value(477), p.p984)), A::scale(s.ad_value(478), p.p985));

        s.store_add_ad(617, A::add(A::offset(A::scale(s.ad_value(476), p.p987), p.p986), A::scale(s.ad_value(477), p.p988)), A::scale(s.ad_value(478), p.p989));

        s.store_add_ad(618, A::add(A::offset(A::scale(s.ad_value(476), p.p991), p.p990), A::scale(s.ad_value(477), p.p992)), A::scale(s.ad_value(478), p.p993));

        s.store_add_ad(619, A::add(A::offset(A::scale(s.ad_value(476), p.p995), p.p994), A::scale(s.ad_value(477), p.p996)), A::scale(s.ad_value(478), p.p997));

        s.store_add_ad(620, A::add(A::offset(A::scale(s.ad_value(476), p.p999), p.p998), A::scale(s.ad_value(477), p.p1000)), A::scale(s.ad_value(478), p.p1001));

        s.store_add_ad(621, A::add(A::offset(A::scale(s.ad_value(476), p.p1003), p.p1002), A::scale(s.ad_value(477), p.p1004)), A::scale(s.ad_value(478), p.p1005));

        s.store_add_ad(622, A::add(A::offset(A::scale(s.ad_value(476), p.p1007), p.p1006), A::scale(s.ad_value(477), p.p1008)), A::scale(s.ad_value(478), p.p1009));

        s.store_add_ad(623, A::add(A::offset(A::scale(s.ad_value(476), p.p1011), p.p1010), A::scale(s.ad_value(477), p.p1012)), A::scale(s.ad_value(478), p.p1013));

        s.store_add_ad(624, A::add(A::offset(A::scale(s.ad_value(476), p.p1018), p.p1017), A::scale(s.ad_value(477), p.p1019)), A::scale(s.ad_value(478), p.p1020));

        s.store_add_ad(625, A::add(A::offset(A::scale(s.ad_value(476), p.p1022), p.p1021), A::scale(s.ad_value(477), p.p1023)), A::scale(s.ad_value(478), p.p1024));

        s.store_add_ad(629, A::add(A::offset(A::scale(s.ad_value(476), p.p1030), p.p1029), A::scale(s.ad_value(477), p.p1031)), A::scale(s.ad_value(478), p.p1032));

        s.store_add_ad(630, A::add(A::offset(A::scale(s.ad_value(476), p.p1026), p.p1025), A::scale(s.ad_value(477), p.p1027)), A::scale(s.ad_value(478), p.p1028));

        s.store_add_ad(626, A::add(A::offset(A::scale(s.ad_value(476), p.p1034), p.p1033), A::scale(s.ad_value(477), p.p1035)), A::scale(s.ad_value(478), p.p1036));

        s.store_add_ad(627, A::add(A::offset(A::scale(s.ad_value(476), p.p1038), p.p1037), A::scale(s.ad_value(477), p.p1039)), A::scale(s.ad_value(478), p.p1040));

        s.store_add_ad(631, A::add(A::offset(A::scale(s.ad_value(476), p.p1070), p.p1069), A::scale(s.ad_value(477), p.p1071)), A::scale(s.ad_value(478), p.p1072));

        s.store_add_ad(632, A::add(A::offset(A::scale(s.ad_value(476), p.p1074), p.p1073), A::scale(s.ad_value(477), p.p1075)), A::scale(s.ad_value(478), p.p1076));

        s.store_add_ad(634, A::add(A::offset(A::scale(s.ad_value(476), p.p1078), p.p1077), A::scale(s.ad_value(477), p.p1079)), A::scale(s.ad_value(478), p.p1080));

        s.store_add_ad(635, A::add(A::offset(A::scale(s.ad_value(476), p.p1082), p.p1081), A::scale(s.ad_value(477), p.p1083)), A::scale(s.ad_value(478), p.p1084));

        s.store_add_ad(637, A::add(A::offset(A::scale(s.ad_value(476), p.p1086), p.p1085), A::scale(s.ad_value(477), p.p1087)), A::scale(s.ad_value(478), p.p1088));

        s.store_add_ad(638, A::add(A::offset(A::scale(s.ad_value(476), p.p1090), p.p1089), A::scale(s.ad_value(477), p.p1091)), A::scale(s.ad_value(478), p.p1092));

        s.store_add_ad(640, A::add(A::offset(A::scale(s.ad_value(476), p.p787), p.p786), A::scale(s.ad_value(477), p.p788)), A::scale(s.ad_value(478), p.p789));

        s.store_add_ad(641, A::add(A::offset(A::scale(s.ad_value(476), p.p795), p.p794), A::scale(s.ad_value(477), p.p796)), A::scale(s.ad_value(478), p.p797));

        s.store_add_ad(642, A::add(A::offset(A::scale(s.ad_value(476), p.p791), p.p790), A::scale(s.ad_value(477), p.p792)), A::scale(s.ad_value(478), p.p793));

        s.v[879] = if (p.p44 != 0.0) { 1.0 } else { 0.0 };

        if (s.v[879] != 0.0) {
            s.store_add_ad(485, A::add(A::offset(A::scale(s.ad_value(476), p.p230), p.p229), A::scale(s.ad_value(477), p.p231)), A::scale(s.ad_value(478), p.p232));
        }

        if (s.v[879] != 0.0) {
            s.store_add_ad(491, A::add(A::offset(A::scale(s.ad_value(476), p.p176), p.p175), A::scale(s.ad_value(477), p.p177)), A::scale(s.ad_value(478), p.p178));
        }

        if (s.v[879] != 0.0) {
            s.store_add_ad(498, A::add(A::offset(A::scale(s.ad_value(476), p.p280), p.p279), A::scale(s.ad_value(477), p.p281)), A::scale(s.ad_value(478), p.p282));
        }

        if (s.v[879] != 0.0) {
            s.store_add_ad(505, A::add(A::offset(A::scale(s.ad_value(476), p.p295), p.p294), A::scale(s.ad_value(477), p.p296)), A::scale(s.ad_value(478), p.p297));
        }

        if (s.v[879] != 0.0) {
            s.store_add_ad(509, A::add(A::offset(A::scale(s.ad_value(476), p.p315), p.p314), A::scale(s.ad_value(477), p.p316)), A::scale(s.ad_value(478), p.p317));
        }

        if (s.v[879] != 0.0) {
            s.store_add_ad(512, A::add(A::offset(A::scale(s.ad_value(476), p.p323), p.p322), A::scale(s.ad_value(477), p.p324)), A::scale(s.ad_value(478), p.p325));
        }

        if (s.v[879] != 0.0) {
            s.store_add_ad(515, A::add(A::offset(A::scale(s.ad_value(476), p.p337), p.p336), A::scale(s.ad_value(477), p.p338)), A::scale(s.ad_value(478), p.p339));
        }

        if (s.v[879] != 0.0) {
            s.store_add_ad(540, A::add(A::offset(A::scale(s.ad_value(476), p.p347), p.p346), A::scale(s.ad_value(477), p.p348)), A::scale(s.ad_value(478), p.p349));
        }

        if (s.v[879] != 0.0) {
            s.store_add_ad(518, A::add(A::offset(A::scale(s.ad_value(476), p.p467), p.p466), A::scale(s.ad_value(477), p.p468)), A::scale(s.ad_value(478), p.p469));
        }

        if (s.v[879] != 0.0) {
            s.store_add_ad(501, A::add(A::offset(A::scale(s.ad_value(476), p.p250), p.p249), A::scale(s.ad_value(477), p.p251)), A::scale(s.ad_value(478), p.p252));
        }

        if (s.v[879] != 0.0) {
            s.store_add_ad(165, A::add(A::offset(A::scale(s.ad_value(476), p.p427), p.p426), A::scale(s.ad_value(477), p.p428)), A::scale(s.ad_value(478), p.p429));
        }

        if (s.v[879] != 0.0) {
            s.store_add_ad(535, A::add(A::offset(A::scale(s.ad_value(476), p.p441), p.p440), A::scale(s.ad_value(477), p.p442)), A::scale(s.ad_value(478), p.p443));
        }

        if (s.v[879] != 0.0) {
            s.store_add_ad(552, A::add(A::offset(A::scale(s.ad_value(476), p.p526), p.p525), A::scale(s.ad_value(477), p.p527)), A::scale(s.ad_value(478), p.p528));
        }

        if (s.v[879] != 0.0) {
            s.store_add_ad(557, A::add(A::offset(A::scale(s.ad_value(476), p.p530), p.p529), A::scale(s.ad_value(477), p.p531)), A::scale(s.ad_value(478), p.p532));
        }

        s.v[12] = ((p.p81 * ((((s.v[469]) as f64).powf(p.p82) - ((s.v[474]) as f64).powf(p.p82))).max(0.0)) + (p.p83 * ((((s.v[469]) as f64).powf(p.p84) - ((s.v[474]) as f64).powf(p.p84))).max(0.0)));

        s.v[13] = ((p.p85 * ((((s.v[470]) as f64).powf(p.p86) - ((s.v[475]) as f64).powf(p.p86))).max(0.0)) + (p.p87 * (((s.v[470] * s.v[469])) as f64).powf(p.p88)));

        s.store_scale(481, 481, ((1.0 + s.v[12]) + s.v[13]));

        s.v[12] = (p.p214 * ((((s.v[469]) as f64).powf(p.p215) - ((s.v[474]) as f64).powf(p.p215))).max(0.0));

        s.v[13] = ((p.p216 * ((((s.v[470]) as f64).powf(p.p217) - ((s.v[475]) as f64).powf(p.p217))).max(0.0)) + (p.p218 * ((s.v[471]) as f64).powf(p.p219)));

        s.store_scale(488, 488, ((1.0 + s.v[12]) + s.v[13]));

        s.v[12] = (1.0 + (p.p224 * ((((s.v[469]) as f64).powf(p.p225) - ((s.v[474]) as f64).powf(p.p225))).max(0.0)));

        s.store_scale(484, 484, s.v[12]);

        s.v[880] = if (p.p44 != 0.0) { 1.0 } else { 0.0 };

        if (s.v[880] != 0.0) {
            s.store_scale(485, 485, s.v[12]);
        }

        s.store_scale(487, 487, (1.0 + (p.p234 * ((((s.v[469]) as f64).powf(p.p235) - ((s.v[474]) as f64).powf(p.p235))).max(0.0))));

        s.store_scale(497, 497, p.p34);

        s.v[881] = if (p.p50 != 1.0) { 1.0 } else { 0.0 };

        s.v[882] = if (p.p275 > 0.0) { 1.0 } else { 0.0 };

        if ((s.v[881] != 0.0) && (s.v[882] != 0.0)) {
            s.store_scale(497, 497, (1.0 - (p.p274 * ((((s.v[469]) as f64).powf(p.p275) - ((s.v[474]) as f64).powf(p.p275))).max(0.0))));
        }

        s.v[883] = if (p.p44 != 0.0) { 1.0 } else { 0.0 };

        if (((s.v[881] != 0.0) && (s.v[882] != 0.0)) && (s.v[883] != 0.0)) {
            s.store_scale(498, 498, (1.0 - (p.p274 * ((((s.v[469]) as f64).powf(p.p275) - ((s.v[474]) as f64).powf(p.p275))).max(0.0))));
        }

        if ((s.v[881] != 0.0) && (!(s.v[882] != 0.0))) {
            s.store_scale(497, 497, (1.0 - p.p274));
        }

        s.v[884] = if (p.p44 != 0.0) { 1.0 } else { 0.0 };

        if (((s.v[881] != 0.0) && (!(s.v[882] != 0.0))) && (s.v[884] != 0.0)) {
            s.store_scale(498, 498, (1.0 - p.p274));
        }

        if (!(s.v[881] != 0.0)) {
            let assign3470_ad_e4787: A = A::scale(s.ad_value(497), ((1.0 - (p.p269 * if ((-s.v[30]) / p.p270) > 80.0 { 5.540622384e34 * (1.0 + (((-s.v[30]) / p.p270)) - 80.0) } else if ((-s.v[30]) / p.p270) < -80.0 { 1.804851387e-35 } else { ((((-s.v[30]) / p.p270)) as f64).exp() })) - (p.p271 * if ((-s.v[30]) / p.p272) > 80.0 { 5.540622384e34 * (1.0 + (((-s.v[30]) / p.p272)) - 80.0) } else if ((-s.v[30]) / p.p272) < -80.0 { 1.804851387e-35 } else { ((((-s.v[30]) / p.p272)) as f64).exp() })));
            s.store_ad(497, &assign3470_ad_e4787);
        }

        s.v[885] = if (p.p44 != 0.0) { 1.0 } else { 0.0 };

        if ((!(s.v[881] != 0.0)) && (s.v[885] != 0.0)) {
            let assign3490_ad_e4815: A = A::scale(s.ad_value(498), ((1.0 - (p.p269 * if ((-s.v[30]) / p.p270) > 80.0 { 5.540622384e34 * (1.0 + (((-s.v[30]) / p.p270)) - 80.0) } else if ((-s.v[30]) / p.p270) < -80.0 { 1.804851387e-35 } else { ((((-s.v[30]) / p.p270)) as f64).exp() })) - (p.p271 * if ((-s.v[30]) / p.p272) > 80.0 { 5.540622384e34 * (1.0 + (((-s.v[30]) / p.p272)) - 80.0) } else if ((-s.v[30]) / p.p272) < -80.0 { 1.804851387e-35 } else { ((((-s.v[30]) / p.p272)) as f64).exp() })));
            s.store_ad(498, &assign3490_ad_e4815);
        }

        s.v[12] = (p.p285 * ((((s.v[469]) as f64).powf(p.p286) - ((s.v[474]) as f64).powf(p.p286))).max(0.0));

        s.v[13] = ((p.p287 * ((((s.v[470]) as f64).powf(p.p288) - ((s.v[475]) as f64).powf(p.p288))).max(0.0)) + (p.p289 * ((s.v[471]) as f64).powf(p.p290)));

        s.store_scale(504, 504, ((1.0 + s.v[12]) + s.v[13]));

        s.v[886] = if (p.p44 != 0.0) { 1.0 } else { 0.0 };

        if (s.v[886] != 0.0) {
            s.store_scale(505, 505, ((1.0 + s.v[12]) + s.v[13]));
        }

        s.v[12] = (p.p302 * ((((s.v[469]) as f64).powf(p.p303) - ((s.v[474]) as f64).powf(p.p303))).max(0.0));

        s.v[13] = ((p.p304 * ((((s.v[470]) as f64).powf(p.p305) - ((s.v[475]) as f64).powf(p.p305))).max(0.0)) + (p.p306 * ((s.v[471]) as f64).powf(p.p307)));

        s.store_scale(507, 507, ((1.0 + s.v[12]) + s.v[13]));

        s.v[12] = (1.0 + (p.p309 * ((((s.v[469]) as f64).powf(p.p310) - ((s.v[474]) as f64).powf(p.p310))).max(0.0)));

        s.store_scale(508, 508, s.v[12]);

        s.v[887] = if (p.p44 != 0.0) { 1.0 } else { 0.0 };

        if (s.v[887] != 0.0) {
            s.store_scale(509, 509, s.v[12]);
        }

        s.v[12] = (p.p327 * ((((s.v[469]) as f64).powf(p.p328) - ((s.v[474]) as f64).powf(p.p328))).max(0.0));

        s.v[13] = ((p.p329 * ((((s.v[470]) as f64).powf(p.p330) - ((s.v[475]) as f64).powf(p.p330))).max(0.0)) + (p.p331 * ((s.v[471]) as f64).powf(p.p332)));

        s.store_scale(514, 514, ((1.0 + s.v[12]) + s.v[13]));

        s.v[888] = if (p.p44 != 0.0) { 1.0 } else { 0.0 };

        if (s.v[888] != 0.0) {
            s.store_scale(515, 515, ((1.0 + s.v[12]) + s.v[13]));
        }

        s.v[12] = ((((s.v[469]) as f64).powf(p.p179) - ((s.v[474]) as f64).powf(p.p179))).max(0.0);

        s.store_scale(490, 490, s.v[12]);

        s.v[889] = if (p.p44 != 0.0) { 1.0 } else { 0.0 };

        if (s.v[889] != 0.0) {
            s.store_scale(491, 491, s.v[12]);
        }

        s.store_scale(493, 493, ((((s.v[469]) as f64).powf(p.p181) - ((s.v[474]) as f64).powf(p.p181))).max(0.0));

        s.v[12] = (1.0 + (p.p461 * ((((s.v[469]) as f64).powf(p.p462) - ((s.v[474]) as f64).powf(p.p462))).max(0.0)));

        s.store_scale(517, 517, s.v[12]);

        s.v[890] = if (p.p44 != 0.0) { 1.0 } else { 0.0 };

        if (s.v[890] != 0.0) {
            s.store_scale(518, 518, s.v[12]);
        }

        s.store_scale(12, 496, (1.0 + (p.p257 * ((((s.v[469]) as f64).powf(p.p258) - ((s.v[474]) as f64).powf(p.p258))).max(0.0))));

        s.store_ad(496, &A::min_with_scalar(s.ad_value(12), 0.5));

        s.store_scale(525, 525, (1.0 + (p.p479 * ((((s.v[469]) as f64).powf(p.p480) - ((s.v[474]) as f64).powf(p.p480))).max(0.0))));

        s.v[12] = (1.0 + (p.p341 * ((((s.v[469]) as f64).powf(p.p342) - ((s.v[474]) as f64).powf(p.p342))).max(0.0)));

        s.store_scale(539, 539, s.v[12]);

        s.store_ad(539, &A::max_with_scalar(s.ad_value(539), 0.0));

        s.v[891] = if (p.p44 != 0.0) { 1.0 } else { 0.0 };

        if (s.v[891] != 0.0) {
            s.store_scale(540, 540, s.v[12]);
        }

        if (s.v[891] != 0.0) {
            s.store_ad(540, &A::max_with_scalar(s.ad_value(540), 0.0));
        }

        s.v[12] = (p.p243 * ((((s.v[469]) as f64).powf(p.p244) - ((s.v[474]) as f64).powf(p.p244))).max(0.0));

        s.v[13] = ((p.p245 * ((((s.v[470]) as f64).powf(p.p246) - ((s.v[475]) as f64).powf(p.p246))).max(0.0)) + (p.p247 * ((s.v[471]) as f64).powf(p.p248)));

        s.store_scale(500, 500, ((1.0 + s.v[12]) + s.v[13]));

        s.v[892] = if (p.p44 != 0.0) { 1.0 } else { 0.0 };

        if (s.v[892] != 0.0) {
            s.store_scale(501, 501, ((1.0 + s.v[12]) + s.v[13]));
        }

        s.store_max_with_scalar_ad(164, A::scale(s.ad_value(164), (1.0 + (p.p423 * ((((s.v[469]) as f64).powf(p.p424) - ((s.v[474]) as f64).powf(p.p424))).max(0.0)))), 0.25);

        s.v[893] = if (p.p44 != 0.0) { 1.0 } else { 0.0 };

        if (s.v[893] != 0.0) {
            s.store_max_with_scalar_ad(165, A::scale(s.ad_value(165), (1.0 + (p.p423 * ((((s.v[469]) as f64).powf(p.p424) - ((s.v[474]) as f64).powf(p.p424))).max(0.0)))), 0.25);
        }

        s.v[12] = (1.0 + (p.p438 * ((((s.v[469]) as f64).powf(p.p439) - ((s.v[474]) as f64).powf(p.p439))).max(0.0)));

        s.store_scale(534, 534, s.v[12]);

        s.v[894] = if (p.p44 != 0.0) { 1.0 } else { 0.0 };

        if (s.v[894] != 0.0) {
            s.store_scale(535, 535, s.v[12]);
        }

        s.v[12] = (p.p485 * ((((s.v[469]) as f64).powf(p.p486) - ((s.v[474]) as f64).powf(p.p486))).max(0.0));

        s.v[13] = (p.p487 * ((((s.v[470]) as f64).powf(p.p488) - ((s.v[475]) as f64).powf(p.p488))).max(0.0));

        s.store_scale(551, 551, ((1.0 + s.v[12]) + s.v[13]));

        s.v[895] = if (p.p44 != 0.0) { 1.0 } else { 0.0 };

        if (s.v[895] != 0.0) {
            s.store_scale(552, 552, ((1.0 + s.v[12]) + s.v[13]));
        }

        s.v[13] = (p.p495 * ((((s.v[470]) as f64).powf(p.p496) - ((s.v[475]) as f64).powf(p.p496))).max(0.0));

        s.store_scale(554, 554, (1.0 + s.v[13]));

        s.v[13] = (p.p519 * ((((s.v[470]) as f64).powf(p.p520) - ((s.v[475]) as f64).powf(p.p520))).max(0.0));

        s.v[555] = p.p518;

        s.v[555] = (s.v[555] * (1.0 + s.v[13]));

        s.v[13] = (p.p522 * ((((s.v[470]) as f64).powf(p.p523) - ((s.v[475]) as f64).powf(p.p523))).max(0.0));

        s.v[556] = p.p521;

        s.v[556] = (s.v[556] * (1.0 + s.v[13]));

        s.store_scale(559, 559, ((1.0 + (p.p631 * s.v[469])) + (p.p632 * s.v[470])));

        s.store_scale(563, 563, ((1.0 + (p.p649 * s.v[469])) + (p.p650 * s.v[470])));

        s.store_scale(590, 590, ((1.0 + (p.p557 * s.v[469])) + (p.p558 * s.v[470])));

        s.store_scale(593, 593, ((1.0 + (p.p559 * s.v[469])) + (p.p560 * s.v[470])));

        s.store_scale(596, 596, ((1.0 + (p.p561 * s.v[469])) + (p.p562 * s.v[470])));

        s.v[600] = (p.p556 * (1.0 + (p.p563 * s.v[469])));

        s.v[12] = ((p.p93 * ((((s.v[472]) as f64).powf(p.p94) - ((s.v[474]) as f64).powf(p.p94))).max(0.0)) + (p.p95 * ((((s.v[472]) as f64).powf(p.p96) - ((s.v[474]) as f64).powf(p.p96))).max(0.0)));

        s.v[13] = ((p.p97 * ((((s.v[473]) as f64).powf(p.p98) - ((s.v[475]) as f64).powf(p.p98))).max(0.0)) + (p.p99 * (((s.v[473] * s.v[472])) as f64).powf(p.p100)));

        s.store_scale(550, 550, ((1.0 + s.v[12]) + s.v[13]));

        s.v[12] = (p.p120 * ((((s.v[472]) as f64).powf(p.p121) - ((s.v[474]) as f64).powf(p.p121))).max(0.0));

        s.v[13] = ((p.p122 * ((((s.v[473]) as f64).powf(p.p123) - ((s.v[475]) as f64).powf(p.p123))).max(0.0)) + (p.p124 * ((s.v[471]) as f64).powf(p.p125)));

        s.store_scale(482, 482, ((1.0 + s.v[12]) + s.v[13]));

        s.v[12] = (p.p130 * ((((s.v[472]) as f64).powf(p.p131) - ((s.v[474]) as f64).powf(p.p131))).max(0.0));

        s.v[13] = ((p.p132 * ((((s.v[473]) as f64).powf(p.p133) - ((s.v[475]) as f64).powf(p.p133))).max(0.0)) + (p.p134 * ((s.v[471]) as f64).powf(p.p135)));

        s.store_scale(549, 549, ((1.0 + s.v[12]) + s.v[13]));

        s.v[12] = (p.p263 * ((((s.v[472]) as f64).powf(p.p264) - ((s.v[474]) as f64).powf(p.p264))).max(0.0));

        s.v[13] = ((p.p265 * ((((s.v[470]) as f64).powf(p.p266) - ((s.v[475]) as f64).powf(p.p266))).max(0.0)) + (p.p267 * ((s.v[471]) as f64).powf(p.p268)));

        s.store_scale(503, 503, ((1.0 + s.v[12]) + s.v[13]));

        s.store_scale(542, 542, (1.0 + (p.p352 * ((((s.v[472]) as f64).powf(p.p353) - ((s.v[474]) as f64).powf(p.p353))).max(0.0))));

        s.store_ad(542, &A::max_with_scalar(s.ad_value(542), 0.0));

        s.v[12] = (p.p186 * ((((s.v[469]) as f64).powf(p.p187) - ((s.v[474]) as f64).powf(p.p187))).max(0.0));

        s.v[13] = ((p.p188 * ((((s.v[470]) as f64).powf(p.p189) - ((s.v[475]) as f64).powf(p.p189))).max(0.0)) + (p.p190 * ((s.v[471]) as f64).powf(p.p191)));

        s.store_scale(495, 495, ((1.0 + s.v[12]) + s.v[13]));

        s.v[12] = (p.p196 * ((((s.v[469]) as f64).powf(p.p197) - ((s.v[474]) as f64).powf(p.p197))).max(0.0));

        s.v[13] = ((p.p198 * ((((s.v[470]) as f64).powf(p.p199) - ((s.v[475]) as f64).powf(p.p199))).max(0.0)) + (p.p200 * ((s.v[471]) as f64).powf(p.p201)));

        s.store_scale(494, 494, ((1.0 + s.v[12]) + s.v[13]));

        s.store_scale(543, 543, (1.0 + (p.p383 * ((((s.v[469]) as f64).powf(p.p384) - ((s.v[474]) as f64).powf(p.p384))).max(0.0))));

        s.store_scale(567, 567, (1.0 + (s.v[469] * p.p828)));

        s.store_scale(568, 568, (1.0 + (s.v[469] * p.p833)));

        s.store_scale(570, 570, (1.0 + (s.v[469] * p.p842)));

        s.store_scale(573, 573, (1.0 + (s.v[469] * p.p860)));

        s.store_scale(574, 574, (1.0 + (s.v[469] * p.p866)));

        s.v[898] = if (p.p42 == 1.0) { 1.0 } else { 0.0 };

        if (s.v[898] != 0.0) {
            s.store_scale(531, 531, (1.0 + (p.p397 * ((((s.v[469]) as f64).powf(p.p398) - ((s.v[474]) as f64).powf(p.p398))).max(0.0))));
        }

        if (s.v[898] != 0.0) {
            s.store_scale(530, 530, (1.0 + (p.p407 * ((((s.v[469]) as f64).powf(p.p408) - ((s.v[474]) as f64).powf(p.p408))).max(0.0))));
        }

        if (!(s.v[898] != 0.0)) {
            s.store_scale(532, 532, (1.0 + (p.p414 * ((((s.v[469]) as f64).powf(p.p415) - ((s.v[474]) as f64).powf(p.p415))).max(0.0))));
        }

        s.v[899] = if (s.v[511] < 1.0) { 1.0 } else { 0.0 };

        if (s.v[899] != 0.0) {
            s.store_scalar(511, 1.0);
        }

        s.v[900] = if (s.v[511] > 2.0) { 1.0 } else { 0.0 };

        if ((!(s.v[899] != 0.0)) && (s.v[900] != 0.0)) {
            s.store_scalar(511, 2.0);
        }

        s.v[901] = if (p.p44 != 0.0) { 1.0 } else { 0.0 };

        s.v[902] = if (s.v[512] < 1.0) { 1.0 } else { 0.0 };

        if ((s.v[901] != 0.0) && (s.v[902] != 0.0)) {
            s.store_scalar(512, 1.0);
        }

        s.v[903] = if (s.v[512] > 2.0) { 1.0 } else { 0.0 };

        if (((s.v[901] != 0.0) && (!(s.v[902] != 0.0))) && (s.v[903] != 0.0)) {
            s.store_scalar(512, 2.0);
        }

        s.v[925] = if (s.v[606] < 0.0) { 1.0 } else { 0.0 };

        if (s.v[925] != 0.0) {
            s.store_scalar(606, 0.0);
        }

        s.v[926] = if (s.v[497] <= 0.0) { 1.0 } else { 0.0 };

        if (s.v[926] != 0.0) {
            s.store_scalar(497, 0.067);
        }

        s.v[927] = if (s.v[504] < 0.0) { 1.0 } else { 0.0 };

        if (s.v[927] != 0.0) {
            s.store_scalar(504, 0.0);
        }

        s.v[928] = if (s.v[507] < 0.0) { 1.0 } else { 0.0 };

        if (s.v[928] != 0.0) {
            s.store_scalar(507, 0.0);
        }

        s.v[929] = if (s.v[508] < 0.0) { 1.0 } else { 0.0 };

        if (s.v[929] != 0.0) {
            s.store_scalar(508, 0.0);
        }

        s.v[930] = if (s.v[511] < 0.0) { 1.0 } else { 0.0 };

        if (s.v[930] != 0.0) {
            s.store_scalar(511, 0.0);
        }

        s.v[931] = if (s.v[555] < 0.0) { 1.0 } else { 0.0 };

        if (s.v[931] != 0.0) {
            s.store_scalar(555, 0.0);
        }

        s.v[932] = if (p.p1065 == 1.0) { 1.0 } else { 0.0 };

        if (s.v[932] != 0.0) {
            s.store_scalar(746, p.p1066);
        }

        s.v[933] = if (s.v[30] > s.v[746]) { 1.0 } else { 0.0 };

    }

    pub(super) fn stamp_reactive_block_2(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if ((s.v[932] != 0.0) && (s.v[933] != 0.0)) {
            s.store_sub_from_scalar(12, s.v[30], 746);
        }

        if ((s.v[932] != 0.0) && (!(s.v[933] != 0.0))) {
            s.store_scalar(746, s.v[30]);
        }

        if ((s.v[932] != 0.0) && (!(s.v[933] != 0.0))) {
            s.copy_ad(12, 746);
        }

        s.v[934] = if (p.p801 >= (s.v[12] / 2.0)) { 1.0 } else { 0.0 };

        if ((s.v[932] != 0.0) && (s.v[934] != 0.0)) {
            s.store_scalar(359, 0.0);
        }

        if ((s.v[932] != 0.0) && (!(s.v[934] != 0.0))) {
            s.store_scalar(359, p.p801);
        }

        s.v[701] = 0.0;

        s.v[703] = 0.0;

        s.v[700] = 0.0;

        s.v[702] = 0.0;

        s.v[705] = 0.0;

        s.v[704] = 0.0;

        s.v[236] = (p.p695 - p.p698);

        s.v[238] = p.p696;

        s.v[237] = (p.p697 - p.p698);

        s.v[935] = if self.param_given[3] { 1.0 } else { 0.0 };

        if (s.v[935] != 0.0) {
            s.store_scalar(239, (p.p374 * p.p3));
        }

        s.v[936] = if ((p.p10 > 0.0) && (p.p374 > 0.0)) { 1.0 } else { 0.0 };

        s.v[937] = if (p.p9 < 9.0) { 1.0 } else { 0.0 };

        s.v[938] = if ((p.p2 % 2.0) != 0.0) { 1.0 } else { 0.0 };

        if ((((!(s.v[935] != 0.0)) && (s.v[936] != 0.0)) && (s.v[937] != 0.0)) && (s.v[938] != 0.0)) {
            s.store_scalar(701, 1.0);
        }

        if ((((!(s.v[935] != 0.0)) && (s.v[936] != 0.0)) && (s.v[937] != 0.0)) && (s.v[938] != 0.0)) {
            s.store_scalar(703, 1.0);
        }

        if ((((!(s.v[935] != 0.0)) && (s.v[936] != 0.0)) && (s.v[937] != 0.0)) && (s.v[938] != 0.0)) {
            s.store_scalar(700, (2.0 * (((p.p2 - 1.0) / 2.0)).max(0.0)));
        }

        if ((((!(s.v[935] != 0.0)) && (s.v[936] != 0.0)) && (s.v[937] != 0.0)) && (s.v[938] != 0.0)) {
            s.copy_ad(702, 700);
        }

        s.v[939] = if (p.p6 == 1.0) { 1.0 } else { 0.0 };

        if (((((!(s.v[935] != 0.0)) && (s.v[936] != 0.0)) && (s.v[937] != 0.0)) && (!(s.v[938] != 0.0))) && (s.v[939] != 0.0)) {
            s.store_scalar(701, 2.0);
        }

        if (((((!(s.v[935] != 0.0)) && (s.v[936] != 0.0)) && (s.v[937] != 0.0)) && (!(s.v[938] != 0.0))) && (s.v[939] != 0.0)) {
            s.store_scalar(700, (2.0 * (((p.p2 / 2.0) - 1.0)).max(0.0)));
        }

        if (((((!(s.v[935] != 0.0)) && (s.v[936] != 0.0)) && (s.v[937] != 0.0)) && (!(s.v[938] != 0.0))) && (s.v[939] != 0.0)) {
            s.store_scalar(703, 0.0);
        }

        if (((((!(s.v[935] != 0.0)) && (s.v[936] != 0.0)) && (s.v[937] != 0.0)) && (!(s.v[938] != 0.0))) && (s.v[939] != 0.0)) {
            s.store_scalar(702, p.p2);
        }

        if (((((!(s.v[935] != 0.0)) && (s.v[936] != 0.0)) && (s.v[937] != 0.0)) && (!(s.v[938] != 0.0))) && (!(s.v[939] != 0.0))) {
            s.store_scalar(701, 0.0);
        }

        if (((((!(s.v[935] != 0.0)) && (s.v[936] != 0.0)) && (s.v[937] != 0.0)) && (!(s.v[938] != 0.0))) && (!(s.v[939] != 0.0))) {
            s.store_scalar(700, p.p2);
        }

        if (((((!(s.v[935] != 0.0)) && (s.v[936] != 0.0)) && (s.v[937] != 0.0)) && (!(s.v[938] != 0.0))) && (!(s.v[939] != 0.0))) {
            s.store_scalar(703, 2.0);
        }

        if (((((!(s.v[935] != 0.0)) && (s.v[936] != 0.0)) && (s.v[937] != 0.0)) && (!(s.v[938] != 0.0))) && (!(s.v[939] != 0.0))) {
            s.store_scalar(702, (2.0 * (((p.p2 / 2.0) - 1.0)).max(0.0)));
        }

        s.v[940] = if (1.0 == 1.0) { 1.0 } else { 0.0 };

        s.v[941] = if (s.v[702] == 0.0) { 1.0 } else { 0.0 };

        if (((((!(s.v[935] != 0.0)) && (s.v[936] != 0.0)) && (s.v[937] != 0.0)) && (s.v[940] != 0.0)) && (s.v[941] != 0.0)) {
            s.store_scalar(704, 0.0);
        }

        if (((((!(s.v[935] != 0.0)) && (s.v[936] != 0.0)) && (s.v[937] != 0.0)) && (s.v[940] != 0.0)) && (!(s.v[941] != 0.0))) {
            s.store_div_from_scalar_ad(704, (p.p374 * s.v[236]), A::scale(s.ad_value(702), s.v[29]));
        }

        s.v[942] = if (s.v[700] == 0.0) { 1.0 } else { 0.0 };

        if (((((!(s.v[935] != 0.0)) && (s.v[936] != 0.0)) && (s.v[937] != 0.0)) && (!(s.v[940] != 0.0))) && (s.v[942] != 0.0)) {
            s.store_scalar(704, 0.0);
        }

        if (((((!(s.v[935] != 0.0)) && (s.v[936] != 0.0)) && (s.v[937] != 0.0)) && (!(s.v[940] != 0.0))) && (!(s.v[942] != 0.0))) {
            s.store_div_from_scalar_ad(704, (p.p374 * s.v[236]), A::scale(s.ad_value(700), s.v[29]));
        }

        s.v[943] = if (p.p9 == 0.0) { 1.0 } else { 0.0 };

        s.v[944] = if (p.p9 == 1.0) { 1.0 } else { 0.0 };

        s.v[945] = if (p.p9 == 2.0) { 1.0 } else { 0.0 };

        s.v[946] = if (p.p9 == 3.0) { 1.0 } else { 0.0 };

        s.v[947] = if (p.p9 == 4.0) { 1.0 } else { 0.0 };

        s.v[948] = if (p.p9 == 5.0) { 1.0 } else { 0.0 };

        s.v[949] = if (p.p9 == 6.0) { 1.0 } else { 0.0 };

        s.v[950] = if (p.p9 == 7.0) { 1.0 } else { 0.0 };

        s.v[951] = if (p.p9 == 8.0) { 1.0 } else { 0.0 };

        s.v[952] = if (p.p9 == 9.0) { 1.0 } else { 0.0 };

        s.v[953] = if (p.p9 == 10.0) { 1.0 } else { 0.0 };

        s.v[954] = if (1.0 == 1.0) { 1.0 } else { 0.0 };

        s.v[955] = if (1.0 == 1.0) { 1.0 } else { 0.0 };

        s.v[956] = if (((p.p10 == 1.0) || (p.p10 == 2.0)) || (p.p10 == 5.0)) { 1.0 } else { 0.0 };

        s.v[957] = if (((p.p10 == 3.0) || (p.p10 == 4.0)) || (p.p10 == 6.0)) { 1.0 } else { 0.0 };

        s.v[958] = if (s.v[703] == 0.0) { 1.0 } else { 0.0 };

        if (((((((!(s.v[935] != 0.0)) && (s.v[936] != 0.0)) && (s.v[943] != 0.0)) && (s.v[954] != 0.0)) && (s.v[955] != 0.0)) && (s.v[956] != 0.0)) && (s.v[958] != 0.0)) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!(s.v[935] != 0.0)) && (s.v[936] != 0.0)) && (s.v[943] != 0.0)) && (s.v[954] != 0.0)) && (s.v[955] != 0.0)) && (s.v[956] != 0.0)) && (!(s.v[958] != 0.0))) {
            s.store_div_from_scalar_ad(705, (p.p374 * s.v[236]), A::scale(s.ad_value(703), s.v[29]));
        }

        s.v[960] = if ((s.v[703] == 0.0) || ((s.v[236] + s.v[238]) == 0.0)) { 1.0 } else { 0.0 };

        if (((((((!(s.v[935] != 0.0)) && (s.v[936] != 0.0)) && (s.v[943] != 0.0)) && (s.v[954] != 0.0)) && (s.v[955] != 0.0)) && ((s.v[957] != 0.0) && (!(s.v[956] != 0.0)))) && (s.v[960] != 0.0)) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!(s.v[935] != 0.0)) && (s.v[936] != 0.0)) && (s.v[943] != 0.0)) && (s.v[954] != 0.0)) && (s.v[955] != 0.0)) && ((s.v[957] != 0.0) && (!(s.v[956] != 0.0)))) && (!(s.v[960] != 0.0))) {
            s.store_div_from_scalar_ad(705, (p.p374 * s.v[29]), A::scale(s.ad_value(703), (3.0 * (s.v[236] + s.v[238]))));
        }

        if ((((((!(s.v[935] != 0.0)) && (s.v[936] != 0.0)) && (s.v[943] != 0.0)) && (s.v[954] != 0.0)) && (s.v[955] != 0.0)) && (!((s.v[956] != 0.0) || (s.v[957] != 0.0)))) {
            s.store_scalar(705, 0.0);
        }

        s.v[961] = if (((p.p10 == 1.0) || (p.p10 == 3.0)) || (p.p10 == 7.0)) { 1.0 } else { 0.0 };

        s.v[962] = if (((p.p10 == 2.0) || (p.p10 == 4.0)) || (p.p10 == 8.0)) { 1.0 } else { 0.0 };

        s.v[963] = if (s.v[703] == 0.0) { 1.0 } else { 0.0 };

        if (((((((!(s.v[935] != 0.0)) && (s.v[936] != 0.0)) && (s.v[943] != 0.0)) && (s.v[954] != 0.0)) && (!(s.v[955] != 0.0))) && (s.v[961] != 0.0)) && (s.v[963] != 0.0)) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!(s.v[935] != 0.0)) && (s.v[936] != 0.0)) && (s.v[943] != 0.0)) && (s.v[954] != 0.0)) && (!(s.v[955] != 0.0))) && (s.v[961] != 0.0)) && (!(s.v[963] != 0.0))) {
            s.store_div_from_scalar_ad(705, (p.p374 * s.v[236]), A::scale(s.ad_value(703), s.v[29]));
        }

        s.v[965] = if ((s.v[703] == 0.0) || ((s.v[236] + s.v[238]) == 0.0)) { 1.0 } else { 0.0 };

        if (((((((!(s.v[935] != 0.0)) && (s.v[936] != 0.0)) && (s.v[943] != 0.0)) && (s.v[954] != 0.0)) && (!(s.v[955] != 0.0))) && ((s.v[962] != 0.0) && (!(s.v[961] != 0.0)))) && (s.v[965] != 0.0)) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!(s.v[935] != 0.0)) && (s.v[936] != 0.0)) && (s.v[943] != 0.0)) && (s.v[954] != 0.0)) && (!(s.v[955] != 0.0))) && ((s.v[962] != 0.0) && (!(s.v[961] != 0.0)))) && (!(s.v[965] != 0.0))) {
            s.store_div_from_scalar_ad(705, (p.p374 * s.v[29]), A::scale(s.ad_value(703), (3.0 * (s.v[236] + s.v[238]))));
        }

        if ((((((!(s.v[935] != 0.0)) && (s.v[936] != 0.0)) && (s.v[943] != 0.0)) && (s.v[954] != 0.0)) && (!(s.v[955] != 0.0))) && (!((s.v[961] != 0.0) || (s.v[962] != 0.0)))) {
            s.store_scalar(705, 0.0);
        }

        s.v[966] = if (0.0 == 1.0) { 1.0 } else { 0.0 };

        s.v[967] = if (((p.p10 == 1.0) || (p.p10 == 2.0)) || (p.p10 == 5.0)) { 1.0 } else { 0.0 };

        s.v[968] = if (((p.p10 == 3.0) || (p.p10 == 4.0)) || (p.p10 == 6.0)) { 1.0 } else { 0.0 };

        s.v[969] = if (s.v[701] == 0.0) { 1.0 } else { 0.0 };

        if (((((((!(s.v[935] != 0.0)) && (s.v[936] != 0.0)) && (s.v[943] != 0.0)) && (!(s.v[954] != 0.0))) && (s.v[966] != 0.0)) && (s.v[967] != 0.0)) && (s.v[969] != 0.0)) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!(s.v[935] != 0.0)) && (s.v[936] != 0.0)) && (s.v[943] != 0.0)) && (!(s.v[954] != 0.0))) && (s.v[966] != 0.0)) && (s.v[967] != 0.0)) && (!(s.v[969] != 0.0))) {
            s.store_div_from_scalar_ad(705, (p.p374 * s.v[236]), A::scale(s.ad_value(701), s.v[29]));
        }

        s.v[971] = if ((s.v[701] == 0.0) || ((s.v[236] + s.v[238]) == 0.0)) { 1.0 } else { 0.0 };

        if (((((((!(s.v[935] != 0.0)) && (s.v[936] != 0.0)) && (s.v[943] != 0.0)) && (!(s.v[954] != 0.0))) && (s.v[966] != 0.0)) && ((s.v[968] != 0.0) && (!(s.v[967] != 0.0)))) && (s.v[971] != 0.0)) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!(s.v[935] != 0.0)) && (s.v[936] != 0.0)) && (s.v[943] != 0.0)) && (!(s.v[954] != 0.0))) && (s.v[966] != 0.0)) && ((s.v[968] != 0.0) && (!(s.v[967] != 0.0)))) && (!(s.v[971] != 0.0))) {
            s.store_div_from_scalar_ad(705, (p.p374 * s.v[29]), A::scale(s.ad_value(701), (3.0 * (s.v[236] + s.v[238]))));
        }

        if ((((((!(s.v[935] != 0.0)) && (s.v[936] != 0.0)) && (s.v[943] != 0.0)) && (!(s.v[954] != 0.0))) && (s.v[966] != 0.0)) && (!((s.v[967] != 0.0) || (s.v[968] != 0.0)))) {
            s.store_scalar(705, 0.0);
        }

        s.v[972] = if (((p.p10 == 1.0) || (p.p10 == 3.0)) || (p.p10 == 7.0)) { 1.0 } else { 0.0 };

        s.v[973] = if (((p.p10 == 2.0) || (p.p10 == 4.0)) || (p.p10 == 8.0)) { 1.0 } else { 0.0 };

        s.v[974] = if (s.v[701] == 0.0) { 1.0 } else { 0.0 };

        if (((((((!(s.v[935] != 0.0)) && (s.v[936] != 0.0)) && (s.v[943] != 0.0)) && (!(s.v[954] != 0.0))) && (!(s.v[966] != 0.0))) && (s.v[972] != 0.0)) && (s.v[974] != 0.0)) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!(s.v[935] != 0.0)) && (s.v[936] != 0.0)) && (s.v[943] != 0.0)) && (!(s.v[954] != 0.0))) && (!(s.v[966] != 0.0))) && (s.v[972] != 0.0)) && (!(s.v[974] != 0.0))) {
            s.store_div_from_scalar_ad(705, (p.p374 * s.v[236]), A::scale(s.ad_value(701), s.v[29]));
        }

        s.v[976] = if ((s.v[701] == 0.0) || ((s.v[236] + s.v[238]) == 0.0)) { 1.0 } else { 0.0 };

        if (((((((!(s.v[935] != 0.0)) && (s.v[936] != 0.0)) && (s.v[943] != 0.0)) && (!(s.v[954] != 0.0))) && (!(s.v[966] != 0.0))) && ((s.v[973] != 0.0) && (!(s.v[972] != 0.0)))) && (s.v[976] != 0.0)) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!(s.v[935] != 0.0)) && (s.v[936] != 0.0)) && (s.v[943] != 0.0)) && (!(s.v[954] != 0.0))) && (!(s.v[966] != 0.0))) && ((s.v[973] != 0.0) && (!(s.v[972] != 0.0)))) && (!(s.v[976] != 0.0))) {
            s.store_div_from_scalar_ad(705, (p.p374 * s.v[29]), A::scale(s.ad_value(701), (3.0 * (s.v[236] + s.v[238]))));
        }

        if ((((((!(s.v[935] != 0.0)) && (s.v[936] != 0.0)) && (s.v[943] != 0.0)) && (!(s.v[954] != 0.0))) && (!(s.v[966] != 0.0))) && (!((s.v[972] != 0.0) || (s.v[973] != 0.0)))) {
            s.store_scalar(705, 0.0);
        }

        s.v[977] = if (1.0 == 1.0) { 1.0 } else { 0.0 };

        s.v[978] = if (1.0 == 1.0) { 1.0 } else { 0.0 };

        s.v[979] = if (((p.p10 == 1.0) || (p.p10 == 2.0)) || (p.p10 == 5.0)) { 1.0 } else { 0.0 };

        s.v[980] = if (((p.p10 == 3.0) || (p.p10 == 4.0)) || (p.p10 == 6.0)) { 1.0 } else { 0.0 };

        s.v[981] = if (s.v[703] == 0.0) { 1.0 } else { 0.0 };

        if (((((((!(s.v[935] != 0.0)) && (s.v[936] != 0.0)) && ((s.v[944] != 0.0) && (!(s.v[943] != 0.0)))) && (s.v[977] != 0.0)) && (s.v[978] != 0.0)) && (s.v[979] != 0.0)) && (s.v[981] != 0.0)) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!(s.v[935] != 0.0)) && (s.v[936] != 0.0)) && ((s.v[944] != 0.0) && (!(s.v[943] != 0.0)))) && (s.v[977] != 0.0)) && (s.v[978] != 0.0)) && (s.v[979] != 0.0)) && (!(s.v[981] != 0.0))) {
            s.store_div_from_scalar_ad(705, (p.p374 * s.v[236]), A::scale(s.ad_value(703), s.v[29]));
        }

        s.v[983] = if ((s.v[703] == 0.0) || ((s.v[236] + s.v[238]) == 0.0)) { 1.0 } else { 0.0 };

        if (((((((!(s.v[935] != 0.0)) && (s.v[936] != 0.0)) && ((s.v[944] != 0.0) && (!(s.v[943] != 0.0)))) && (s.v[977] != 0.0)) && (s.v[978] != 0.0)) && ((s.v[980] != 0.0) && (!(s.v[979] != 0.0)))) && (s.v[983] != 0.0)) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!(s.v[935] != 0.0)) && (s.v[936] != 0.0)) && ((s.v[944] != 0.0) && (!(s.v[943] != 0.0)))) && (s.v[977] != 0.0)) && (s.v[978] != 0.0)) && ((s.v[980] != 0.0) && (!(s.v[979] != 0.0)))) && (!(s.v[983] != 0.0))) {
            s.store_div_from_scalar_ad(705, (p.p374 * s.v[29]), A::scale(s.ad_value(703), (3.0 * (s.v[236] + s.v[238]))));
        }

        if ((((((!(s.v[935] != 0.0)) && (s.v[936] != 0.0)) && ((s.v[944] != 0.0) && (!(s.v[943] != 0.0)))) && (s.v[977] != 0.0)) && (s.v[978] != 0.0)) && (!((s.v[979] != 0.0) || (s.v[980] != 0.0)))) {
            s.store_scalar(705, 0.0);
        }

        s.v[984] = if (((p.p10 == 1.0) || (p.p10 == 3.0)) || (p.p10 == 7.0)) { 1.0 } else { 0.0 };

        s.v[985] = if (((p.p10 == 2.0) || (p.p10 == 4.0)) || (p.p10 == 8.0)) { 1.0 } else { 0.0 };

        s.v[986] = if (s.v[703] == 0.0) { 1.0 } else { 0.0 };

        if (((((((!(s.v[935] != 0.0)) && (s.v[936] != 0.0)) && ((s.v[944] != 0.0) && (!(s.v[943] != 0.0)))) && (s.v[977] != 0.0)) && (!(s.v[978] != 0.0))) && (s.v[984] != 0.0)) && (s.v[986] != 0.0)) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!(s.v[935] != 0.0)) && (s.v[936] != 0.0)) && ((s.v[944] != 0.0) && (!(s.v[943] != 0.0)))) && (s.v[977] != 0.0)) && (!(s.v[978] != 0.0))) && (s.v[984] != 0.0)) && (!(s.v[986] != 0.0))) {
            s.store_div_from_scalar_ad(705, (p.p374 * s.v[236]), A::scale(s.ad_value(703), s.v[29]));
        }

        s.v[988] = if ((s.v[703] == 0.0) || ((s.v[236] + s.v[238]) == 0.0)) { 1.0 } else { 0.0 };

        if (((((((!(s.v[935] != 0.0)) && (s.v[936] != 0.0)) && ((s.v[944] != 0.0) && (!(s.v[943] != 0.0)))) && (s.v[977] != 0.0)) && (!(s.v[978] != 0.0))) && ((s.v[985] != 0.0) && (!(s.v[984] != 0.0)))) && (s.v[988] != 0.0)) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!(s.v[935] != 0.0)) && (s.v[936] != 0.0)) && ((s.v[944] != 0.0) && (!(s.v[943] != 0.0)))) && (s.v[977] != 0.0)) && (!(s.v[978] != 0.0))) && ((s.v[985] != 0.0) && (!(s.v[984] != 0.0)))) && (!(s.v[988] != 0.0))) {
            s.store_div_from_scalar_ad(705, (p.p374 * s.v[29]), A::scale(s.ad_value(703), (3.0 * (s.v[236] + s.v[238]))));
        }

        if ((((((!(s.v[935] != 0.0)) && (s.v[936] != 0.0)) && ((s.v[944] != 0.0) && (!(s.v[943] != 0.0)))) && (s.v[977] != 0.0)) && (!(s.v[978] != 0.0))) && (!((s.v[984] != 0.0) || (s.v[985] != 0.0)))) {
            s.store_scalar(705, 0.0);
        }

        s.v[989] = if (0.0 == 1.0) { 1.0 } else { 0.0 };

        s.v[990] = if (((p.p10 == 1.0) || (p.p10 == 2.0)) || (p.p10 == 5.0)) { 1.0 } else { 0.0 };

        s.v[991] = if (((p.p10 == 3.0) || (p.p10 == 4.0)) || (p.p10 == 6.0)) { 1.0 } else { 0.0 };

        s.v[992] = if (s.v[701] == 0.0) { 1.0 } else { 0.0 };

        if (((((((!(s.v[935] != 0.0)) && (s.v[936] != 0.0)) && ((s.v[944] != 0.0) && (!(s.v[943] != 0.0)))) && (!(s.v[977] != 0.0))) && (s.v[989] != 0.0)) && (s.v[990] != 0.0)) && (s.v[992] != 0.0)) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!(s.v[935] != 0.0)) && (s.v[936] != 0.0)) && ((s.v[944] != 0.0) && (!(s.v[943] != 0.0)))) && (!(s.v[977] != 0.0))) && (s.v[989] != 0.0)) && (s.v[990] != 0.0)) && (!(s.v[992] != 0.0))) {
            s.store_div_from_scalar_ad(705, (p.p374 * s.v[236]), A::scale(s.ad_value(701), s.v[29]));
        }

        s.v[994] = if ((s.v[701] == 0.0) || (s.v[236] == 0.0)) { 1.0 } else { 0.0 };

        if (((((((!(s.v[935] != 0.0)) && (s.v[936] != 0.0)) && ((s.v[944] != 0.0) && (!(s.v[943] != 0.0)))) && (!(s.v[977] != 0.0))) && (s.v[989] != 0.0)) && ((s.v[991] != 0.0) && (!(s.v[990] != 0.0)))) && (s.v[994] != 0.0)) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!(s.v[935] != 0.0)) && (s.v[936] != 0.0)) && ((s.v[944] != 0.0) && (!(s.v[943] != 0.0)))) && (!(s.v[977] != 0.0))) && (s.v[989] != 0.0)) && ((s.v[991] != 0.0) && (!(s.v[990] != 0.0)))) && (!(s.v[994] != 0.0))) {
            s.store_div_from_scalar_ad(705, (p.p374 * s.v[29]), A::scale(s.ad_value(701), (6.0 * s.v[236])));
        }

        if ((((((!(s.v[935] != 0.0)) && (s.v[936] != 0.0)) && ((s.v[944] != 0.0) && (!(s.v[943] != 0.0)))) && (!(s.v[977] != 0.0))) && (s.v[989] != 0.0)) && (!((s.v[990] != 0.0) || (s.v[991] != 0.0)))) {
            s.store_scalar(705, 0.0);
        }

        s.v[995] = if (((p.p10 == 1.0) || (p.p10 == 3.0)) || (p.p10 == 7.0)) { 1.0 } else { 0.0 };

        s.v[996] = if (((p.p10 == 2.0) || (p.p10 == 4.0)) || (p.p10 == 8.0)) { 1.0 } else { 0.0 };

        s.v[997] = if (s.v[701] == 0.0) { 1.0 } else { 0.0 };

        if (((((((!(s.v[935] != 0.0)) && (s.v[936] != 0.0)) && ((s.v[944] != 0.0) && (!(s.v[943] != 0.0)))) && (!(s.v[977] != 0.0))) && (!(s.v[989] != 0.0))) && (s.v[995] != 0.0)) && (s.v[997] != 0.0)) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!(s.v[935] != 0.0)) && (s.v[936] != 0.0)) && ((s.v[944] != 0.0) && (!(s.v[943] != 0.0)))) && (!(s.v[977] != 0.0))) && (!(s.v[989] != 0.0))) && (s.v[995] != 0.0)) && (!(s.v[997] != 0.0))) {
            s.store_div_from_scalar_ad(705, (p.p374 * s.v[236]), A::scale(s.ad_value(701), s.v[29]));
        }

        s.v[999] = if ((s.v[701] == 0.0) || (s.v[236] == 0.0)) { 1.0 } else { 0.0 };

        if (((((((!(s.v[935] != 0.0)) && (s.v[936] != 0.0)) && ((s.v[944] != 0.0) && (!(s.v[943] != 0.0)))) && (!(s.v[977] != 0.0))) && (!(s.v[989] != 0.0))) && ((s.v[996] != 0.0) && (!(s.v[995] != 0.0)))) && (s.v[999] != 0.0)) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!(s.v[935] != 0.0)) && (s.v[936] != 0.0)) && ((s.v[944] != 0.0) && (!(s.v[943] != 0.0)))) && (!(s.v[977] != 0.0))) && (!(s.v[989] != 0.0))) && ((s.v[996] != 0.0) && (!(s.v[995] != 0.0)))) && (!(s.v[999] != 0.0))) {
            s.store_div_from_scalar_ad(705, (p.p374 * s.v[29]), A::scale(s.ad_value(701), (6.0 * s.v[236])));
        }

        if ((((((!(s.v[935] != 0.0)) && (s.v[936] != 0.0)) && ((s.v[944] != 0.0) && (!(s.v[943] != 0.0)))) && (!(s.v[977] != 0.0))) && (!(s.v[989] != 0.0))) && (!((s.v[995] != 0.0) || (s.v[996] != 0.0)))) {
            s.store_scalar(705, 0.0);
        }

        s.v[1000] = if (1.0 == 1.0) { 1.0 } else { 0.0 };

        s.v[1001] = if (1.0 == 1.0) { 1.0 } else { 0.0 };

        s.v[1002] = if (((p.p10 == 1.0) || (p.p10 == 2.0)) || (p.p10 == 5.0)) { 1.0 } else { 0.0 };

        s.v[1003] = if (((p.p10 == 3.0) || (p.p10 == 4.0)) || (p.p10 == 6.0)) { 1.0 } else { 0.0 };

        s.v[1004] = if (s.v[703] == 0.0) { 1.0 } else { 0.0 };

        if (((((((!(s.v[935] != 0.0)) && (s.v[936] != 0.0)) && ((s.v[945] != 0.0) && (!((s.v[943] != 0.0) || (s.v[944] != 0.0))))) && (s.v[1000] != 0.0)) && (s.v[1001] != 0.0)) && (s.v[1002] != 0.0)) && (s.v[1004] != 0.0)) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!(s.v[935] != 0.0)) && (s.v[936] != 0.0)) && ((s.v[945] != 0.0) && (!((s.v[943] != 0.0) || (s.v[944] != 0.0))))) && (s.v[1000] != 0.0)) && (s.v[1001] != 0.0)) && (s.v[1002] != 0.0)) && (!(s.v[1004] != 0.0))) {
            s.store_div_from_scalar_ad(705, (p.p374 * s.v[236]), A::scale(s.ad_value(703), s.v[29]));
        }

        s.v[1006] = if ((s.v[703] == 0.0) || (s.v[236] == 0.0)) { 1.0 } else { 0.0 };

        if (((((((!(s.v[935] != 0.0)) && (s.v[936] != 0.0)) && ((s.v[945] != 0.0) && (!((s.v[943] != 0.0) || (s.v[944] != 0.0))))) && (s.v[1000] != 0.0)) && (s.v[1001] != 0.0)) && ((s.v[1003] != 0.0) && (!(s.v[1002] != 0.0)))) && (s.v[1006] != 0.0)) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!(s.v[935] != 0.0)) && (s.v[936] != 0.0)) && ((s.v[945] != 0.0) && (!((s.v[943] != 0.0) || (s.v[944] != 0.0))))) && (s.v[1000] != 0.0)) && (s.v[1001] != 0.0)) && ((s.v[1003] != 0.0) && (!(s.v[1002] != 0.0)))) && (!(s.v[1006] != 0.0))) {
            s.store_div_from_scalar_ad(705, (p.p374 * s.v[29]), A::scale(s.ad_value(703), (6.0 * s.v[236])));
        }

        if ((((((!(s.v[935] != 0.0)) && (s.v[936] != 0.0)) && ((s.v[945] != 0.0) && (!((s.v[943] != 0.0) || (s.v[944] != 0.0))))) && (s.v[1000] != 0.0)) && (s.v[1001] != 0.0)) && (!((s.v[1002] != 0.0) || (s.v[1003] != 0.0)))) {
            s.store_scalar(705, 0.0);
        }

        s.v[1007] = if (((p.p10 == 1.0) || (p.p10 == 3.0)) || (p.p10 == 7.0)) { 1.0 } else { 0.0 };

        s.v[1008] = if (((p.p10 == 2.0) || (p.p10 == 4.0)) || (p.p10 == 8.0)) { 1.0 } else { 0.0 };

        s.v[1009] = if (s.v[703] == 0.0) { 1.0 } else { 0.0 };

        if (((((((!(s.v[935] != 0.0)) && (s.v[936] != 0.0)) && ((s.v[945] != 0.0) && (!((s.v[943] != 0.0) || (s.v[944] != 0.0))))) && (s.v[1000] != 0.0)) && (!(s.v[1001] != 0.0))) && (s.v[1007] != 0.0)) && (s.v[1009] != 0.0)) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!(s.v[935] != 0.0)) && (s.v[936] != 0.0)) && ((s.v[945] != 0.0) && (!((s.v[943] != 0.0) || (s.v[944] != 0.0))))) && (s.v[1000] != 0.0)) && (!(s.v[1001] != 0.0))) && (s.v[1007] != 0.0)) && (!(s.v[1009] != 0.0))) {
            s.store_div_from_scalar_ad(705, (p.p374 * s.v[236]), A::scale(s.ad_value(703), s.v[29]));
        }

        s.v[1011] = if ((s.v[703] == 0.0) || (s.v[236] == 0.0)) { 1.0 } else { 0.0 };

        if (((((((!(s.v[935] != 0.0)) && (s.v[936] != 0.0)) && ((s.v[945] != 0.0) && (!((s.v[943] != 0.0) || (s.v[944] != 0.0))))) && (s.v[1000] != 0.0)) && (!(s.v[1001] != 0.0))) && ((s.v[1008] != 0.0) && (!(s.v[1007] != 0.0)))) && (s.v[1011] != 0.0)) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!(s.v[935] != 0.0)) && (s.v[936] != 0.0)) && ((s.v[945] != 0.0) && (!((s.v[943] != 0.0) || (s.v[944] != 0.0))))) && (s.v[1000] != 0.0)) && (!(s.v[1001] != 0.0))) && ((s.v[1008] != 0.0) && (!(s.v[1007] != 0.0)))) && (!(s.v[1011] != 0.0))) {
            s.store_div_from_scalar_ad(705, (p.p374 * s.v[29]), A::scale(s.ad_value(703), (6.0 * s.v[236])));
        }

        if ((((((!(s.v[935] != 0.0)) && (s.v[936] != 0.0)) && ((s.v[945] != 0.0) && (!((s.v[943] != 0.0) || (s.v[944] != 0.0))))) && (s.v[1000] != 0.0)) && (!(s.v[1001] != 0.0))) && (!((s.v[1007] != 0.0) || (s.v[1008] != 0.0)))) {
            s.store_scalar(705, 0.0);
        }

        s.v[1012] = if (0.0 == 1.0) { 1.0 } else { 0.0 };

        s.v[1013] = if (((p.p10 == 1.0) || (p.p10 == 2.0)) || (p.p10 == 5.0)) { 1.0 } else { 0.0 };

        s.v[1014] = if (((p.p10 == 3.0) || (p.p10 == 4.0)) || (p.p10 == 6.0)) { 1.0 } else { 0.0 };

        s.v[1015] = if (s.v[701] == 0.0) { 1.0 } else { 0.0 };

        if (((((((!(s.v[935] != 0.0)) && (s.v[936] != 0.0)) && ((s.v[945] != 0.0) && (!((s.v[943] != 0.0) || (s.v[944] != 0.0))))) && (!(s.v[1000] != 0.0))) && (s.v[1012] != 0.0)) && (s.v[1013] != 0.0)) && (s.v[1015] != 0.0)) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!(s.v[935] != 0.0)) && (s.v[936] != 0.0)) && ((s.v[945] != 0.0) && (!((s.v[943] != 0.0) || (s.v[944] != 0.0))))) && (!(s.v[1000] != 0.0))) && (s.v[1012] != 0.0)) && (s.v[1013] != 0.0)) && (!(s.v[1015] != 0.0))) {
            s.store_div_from_scalar_ad(705, (p.p374 * s.v[236]), A::scale(s.ad_value(701), s.v[29]));
        }

        s.v[1017] = if ((s.v[701] == 0.0) || ((s.v[236] + s.v[238]) == 0.0)) { 1.0 } else { 0.0 };

        if (((((((!(s.v[935] != 0.0)) && (s.v[936] != 0.0)) && ((s.v[945] != 0.0) && (!((s.v[943] != 0.0) || (s.v[944] != 0.0))))) && (!(s.v[1000] != 0.0))) && (s.v[1012] != 0.0)) && ((s.v[1014] != 0.0) && (!(s.v[1013] != 0.0)))) && (s.v[1017] != 0.0)) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!(s.v[935] != 0.0)) && (s.v[936] != 0.0)) && ((s.v[945] != 0.0) && (!((s.v[943] != 0.0) || (s.v[944] != 0.0))))) && (!(s.v[1000] != 0.0))) && (s.v[1012] != 0.0)) && ((s.v[1014] != 0.0) && (!(s.v[1013] != 0.0)))) && (!(s.v[1017] != 0.0))) {
            s.store_div_from_scalar_ad(705, (p.p374 * s.v[29]), A::scale(s.ad_value(701), (3.0 * (s.v[236] + s.v[238]))));
        }

        if ((((((!(s.v[935] != 0.0)) && (s.v[936] != 0.0)) && ((s.v[945] != 0.0) && (!((s.v[943] != 0.0) || (s.v[944] != 0.0))))) && (!(s.v[1000] != 0.0))) && (s.v[1012] != 0.0)) && (!((s.v[1013] != 0.0) || (s.v[1014] != 0.0)))) {
            s.store_scalar(705, 0.0);
        }

        s.v[1018] = if (((p.p10 == 1.0) || (p.p10 == 3.0)) || (p.p10 == 7.0)) { 1.0 } else { 0.0 };

        s.v[1019] = if (((p.p10 == 2.0) || (p.p10 == 4.0)) || (p.p10 == 8.0)) { 1.0 } else { 0.0 };

        s.v[1020] = if (s.v[701] == 0.0) { 1.0 } else { 0.0 };

        if (((((((!(s.v[935] != 0.0)) && (s.v[936] != 0.0)) && ((s.v[945] != 0.0) && (!((s.v[943] != 0.0) || (s.v[944] != 0.0))))) && (!(s.v[1000] != 0.0))) && (!(s.v[1012] != 0.0))) && (s.v[1018] != 0.0)) && (s.v[1020] != 0.0)) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!(s.v[935] != 0.0)) && (s.v[936] != 0.0)) && ((s.v[945] != 0.0) && (!((s.v[943] != 0.0) || (s.v[944] != 0.0))))) && (!(s.v[1000] != 0.0))) && (!(s.v[1012] != 0.0))) && (s.v[1018] != 0.0)) && (!(s.v[1020] != 0.0))) {
            s.store_div_from_scalar_ad(705, (p.p374 * s.v[236]), A::scale(s.ad_value(701), s.v[29]));
        }

        s.v[1022] = if ((s.v[701] == 0.0) || ((s.v[236] + s.v[238]) == 0.0)) { 1.0 } else { 0.0 };

        if (((((((!(s.v[935] != 0.0)) && (s.v[936] != 0.0)) && ((s.v[945] != 0.0) && (!((s.v[943] != 0.0) || (s.v[944] != 0.0))))) && (!(s.v[1000] != 0.0))) && (!(s.v[1012] != 0.0))) && ((s.v[1019] != 0.0) && (!(s.v[1018] != 0.0)))) && (s.v[1022] != 0.0)) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!(s.v[935] != 0.0)) && (s.v[936] != 0.0)) && ((s.v[945] != 0.0) && (!((s.v[943] != 0.0) || (s.v[944] != 0.0))))) && (!(s.v[1000] != 0.0))) && (!(s.v[1012] != 0.0))) && ((s.v[1019] != 0.0) && (!(s.v[1018] != 0.0)))) && (!(s.v[1022] != 0.0))) {
            s.store_div_from_scalar_ad(705, (p.p374 * s.v[29]), A::scale(s.ad_value(701), (3.0 * (s.v[236] + s.v[238]))));
        }

        if ((((((!(s.v[935] != 0.0)) && (s.v[936] != 0.0)) && ((s.v[945] != 0.0) && (!((s.v[943] != 0.0) || (s.v[944] != 0.0))))) && (!(s.v[1000] != 0.0))) && (!(s.v[1012] != 0.0))) && (!((s.v[1018] != 0.0) || (s.v[1019] != 0.0)))) {
            s.store_scalar(705, 0.0);
        }

        s.v[1023] = if (1.0 == 1.0) { 1.0 } else { 0.0 };

        s.v[1024] = if (1.0 == 1.0) { 1.0 } else { 0.0 };

        s.v[1025] = if (((p.p10 == 1.0) || (p.p10 == 2.0)) || (p.p10 == 5.0)) { 1.0 } else { 0.0 };

        s.v[1026] = if (((p.p10 == 3.0) || (p.p10 == 4.0)) || (p.p10 == 6.0)) { 1.0 } else { 0.0 };

        s.v[1027] = if (s.v[703] == 0.0) { 1.0 } else { 0.0 };

    }

    pub(super) fn stamp_reactive_block_3(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if (((((((!(s.v[935] != 0.0)) && (s.v[936] != 0.0)) && ((s.v[946] != 0.0) && (!(((s.v[943] != 0.0) || (s.v[944] != 0.0)) || (s.v[945] != 0.0))))) && (s.v[1023] != 0.0)) && (s.v[1024] != 0.0)) && (s.v[1025] != 0.0)) && (s.v[1027] != 0.0)) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!(s.v[935] != 0.0)) && (s.v[936] != 0.0)) && ((s.v[946] != 0.0) && (!(((s.v[943] != 0.0) || (s.v[944] != 0.0)) || (s.v[945] != 0.0))))) && (s.v[1023] != 0.0)) && (s.v[1024] != 0.0)) && (s.v[1025] != 0.0)) && (!(s.v[1027] != 0.0))) {
            s.store_div_from_scalar_ad(705, (p.p374 * s.v[236]), A::scale(s.ad_value(703), s.v[29]));
        }

        s.v[1029] = if ((s.v[703] == 0.0) || (s.v[236] == 0.0)) { 1.0 } else { 0.0 };

        if (((((((!(s.v[935] != 0.0)) && (s.v[936] != 0.0)) && ((s.v[946] != 0.0) && (!(((s.v[943] != 0.0) || (s.v[944] != 0.0)) || (s.v[945] != 0.0))))) && (s.v[1023] != 0.0)) && (s.v[1024] != 0.0)) && ((s.v[1026] != 0.0) && (!(s.v[1025] != 0.0)))) && (s.v[1029] != 0.0)) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!(s.v[935] != 0.0)) && (s.v[936] != 0.0)) && ((s.v[946] != 0.0) && (!(((s.v[943] != 0.0) || (s.v[944] != 0.0)) || (s.v[945] != 0.0))))) && (s.v[1023] != 0.0)) && (s.v[1024] != 0.0)) && ((s.v[1026] != 0.0) && (!(s.v[1025] != 0.0)))) && (!(s.v[1029] != 0.0))) {
            s.store_div_from_scalar_ad(705, (p.p374 * s.v[29]), A::scale(s.ad_value(703), (6.0 * s.v[236])));
        }

        if ((((((!(s.v[935] != 0.0)) && (s.v[936] != 0.0)) && ((s.v[946] != 0.0) && (!(((s.v[943] != 0.0) || (s.v[944] != 0.0)) || (s.v[945] != 0.0))))) && (s.v[1023] != 0.0)) && (s.v[1024] != 0.0)) && (!((s.v[1025] != 0.0) || (s.v[1026] != 0.0)))) {
            s.store_scalar(705, 0.0);
        }

        s.v[1030] = if (((p.p10 == 1.0) || (p.p10 == 3.0)) || (p.p10 == 7.0)) { 1.0 } else { 0.0 };

        s.v[1031] = if (((p.p10 == 2.0) || (p.p10 == 4.0)) || (p.p10 == 8.0)) { 1.0 } else { 0.0 };

        s.v[1032] = if (s.v[703] == 0.0) { 1.0 } else { 0.0 };

        if (((((((!(s.v[935] != 0.0)) && (s.v[936] != 0.0)) && ((s.v[946] != 0.0) && (!(((s.v[943] != 0.0) || (s.v[944] != 0.0)) || (s.v[945] != 0.0))))) && (s.v[1023] != 0.0)) && (!(s.v[1024] != 0.0))) && (s.v[1030] != 0.0)) && (s.v[1032] != 0.0)) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!(s.v[935] != 0.0)) && (s.v[936] != 0.0)) && ((s.v[946] != 0.0) && (!(((s.v[943] != 0.0) || (s.v[944] != 0.0)) || (s.v[945] != 0.0))))) && (s.v[1023] != 0.0)) && (!(s.v[1024] != 0.0))) && (s.v[1030] != 0.0)) && (!(s.v[1032] != 0.0))) {
            s.store_div_from_scalar_ad(705, (p.p374 * s.v[236]), A::scale(s.ad_value(703), s.v[29]));
        }

        s.v[1034] = if ((s.v[703] == 0.0) || (s.v[236] == 0.0)) { 1.0 } else { 0.0 };

        if (((((((!(s.v[935] != 0.0)) && (s.v[936] != 0.0)) && ((s.v[946] != 0.0) && (!(((s.v[943] != 0.0) || (s.v[944] != 0.0)) || (s.v[945] != 0.0))))) && (s.v[1023] != 0.0)) && (!(s.v[1024] != 0.0))) && ((s.v[1031] != 0.0) && (!(s.v[1030] != 0.0)))) && (s.v[1034] != 0.0)) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!(s.v[935] != 0.0)) && (s.v[936] != 0.0)) && ((s.v[946] != 0.0) && (!(((s.v[943] != 0.0) || (s.v[944] != 0.0)) || (s.v[945] != 0.0))))) && (s.v[1023] != 0.0)) && (!(s.v[1024] != 0.0))) && ((s.v[1031] != 0.0) && (!(s.v[1030] != 0.0)))) && (!(s.v[1034] != 0.0))) {
            s.store_div_from_scalar_ad(705, (p.p374 * s.v[29]), A::scale(s.ad_value(703), (6.0 * s.v[236])));
        }

        if ((((((!(s.v[935] != 0.0)) && (s.v[936] != 0.0)) && ((s.v[946] != 0.0) && (!(((s.v[943] != 0.0) || (s.v[944] != 0.0)) || (s.v[945] != 0.0))))) && (s.v[1023] != 0.0)) && (!(s.v[1024] != 0.0))) && (!((s.v[1030] != 0.0) || (s.v[1031] != 0.0)))) {
            s.store_scalar(705, 0.0);
        }

        s.v[1035] = if (0.0 == 1.0) { 1.0 } else { 0.0 };

        s.v[1036] = if (((p.p10 == 1.0) || (p.p10 == 2.0)) || (p.p10 == 5.0)) { 1.0 } else { 0.0 };

        s.v[1037] = if (((p.p10 == 3.0) || (p.p10 == 4.0)) || (p.p10 == 6.0)) { 1.0 } else { 0.0 };

        s.v[1038] = if (s.v[701] == 0.0) { 1.0 } else { 0.0 };

        if (((((((!(s.v[935] != 0.0)) && (s.v[936] != 0.0)) && ((s.v[946] != 0.0) && (!(((s.v[943] != 0.0) || (s.v[944] != 0.0)) || (s.v[945] != 0.0))))) && (!(s.v[1023] != 0.0))) && (s.v[1035] != 0.0)) && (s.v[1036] != 0.0)) && (s.v[1038] != 0.0)) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!(s.v[935] != 0.0)) && (s.v[936] != 0.0)) && ((s.v[946] != 0.0) && (!(((s.v[943] != 0.0) || (s.v[944] != 0.0)) || (s.v[945] != 0.0))))) && (!(s.v[1023] != 0.0))) && (s.v[1035] != 0.0)) && (s.v[1036] != 0.0)) && (!(s.v[1038] != 0.0))) {
            s.store_div_from_scalar_ad(705, (p.p374 * s.v[236]), A::scale(s.ad_value(701), s.v[29]));
        }

        s.v[1040] = if ((s.v[701] == 0.0) || (s.v[236] == 0.0)) { 1.0 } else { 0.0 };

        if (((((((!(s.v[935] != 0.0)) && (s.v[936] != 0.0)) && ((s.v[946] != 0.0) && (!(((s.v[943] != 0.0) || (s.v[944] != 0.0)) || (s.v[945] != 0.0))))) && (!(s.v[1023] != 0.0))) && (s.v[1035] != 0.0)) && ((s.v[1037] != 0.0) && (!(s.v[1036] != 0.0)))) && (s.v[1040] != 0.0)) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!(s.v[935] != 0.0)) && (s.v[936] != 0.0)) && ((s.v[946] != 0.0) && (!(((s.v[943] != 0.0) || (s.v[944] != 0.0)) || (s.v[945] != 0.0))))) && (!(s.v[1023] != 0.0))) && (s.v[1035] != 0.0)) && ((s.v[1037] != 0.0) && (!(s.v[1036] != 0.0)))) && (!(s.v[1040] != 0.0))) {
            s.store_div_from_scalar_ad(705, (p.p374 * s.v[29]), A::scale(s.ad_value(701), (6.0 * s.v[236])));
        }

        if ((((((!(s.v[935] != 0.0)) && (s.v[936] != 0.0)) && ((s.v[946] != 0.0) && (!(((s.v[943] != 0.0) || (s.v[944] != 0.0)) || (s.v[945] != 0.0))))) && (!(s.v[1023] != 0.0))) && (s.v[1035] != 0.0)) && (!((s.v[1036] != 0.0) || (s.v[1037] != 0.0)))) {
            s.store_scalar(705, 0.0);
        }

        s.v[1041] = if (((p.p10 == 1.0) || (p.p10 == 3.0)) || (p.p10 == 7.0)) { 1.0 } else { 0.0 };

        s.v[1042] = if (((p.p10 == 2.0) || (p.p10 == 4.0)) || (p.p10 == 8.0)) { 1.0 } else { 0.0 };

        s.v[1043] = if (s.v[701] == 0.0) { 1.0 } else { 0.0 };

        if (((((((!(s.v[935] != 0.0)) && (s.v[936] != 0.0)) && ((s.v[946] != 0.0) && (!(((s.v[943] != 0.0) || (s.v[944] != 0.0)) || (s.v[945] != 0.0))))) && (!(s.v[1023] != 0.0))) && (!(s.v[1035] != 0.0))) && (s.v[1041] != 0.0)) && (s.v[1043] != 0.0)) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!(s.v[935] != 0.0)) && (s.v[936] != 0.0)) && ((s.v[946] != 0.0) && (!(((s.v[943] != 0.0) || (s.v[944] != 0.0)) || (s.v[945] != 0.0))))) && (!(s.v[1023] != 0.0))) && (!(s.v[1035] != 0.0))) && (s.v[1041] != 0.0)) && (!(s.v[1043] != 0.0))) {
            s.store_div_from_scalar_ad(705, (p.p374 * s.v[236]), A::scale(s.ad_value(701), s.v[29]));
        }

        s.v[1045] = if ((s.v[701] == 0.0) || (s.v[236] == 0.0)) { 1.0 } else { 0.0 };

        if (((((((!(s.v[935] != 0.0)) && (s.v[936] != 0.0)) && ((s.v[946] != 0.0) && (!(((s.v[943] != 0.0) || (s.v[944] != 0.0)) || (s.v[945] != 0.0))))) && (!(s.v[1023] != 0.0))) && (!(s.v[1035] != 0.0))) && ((s.v[1042] != 0.0) && (!(s.v[1041] != 0.0)))) && (s.v[1045] != 0.0)) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!(s.v[935] != 0.0)) && (s.v[936] != 0.0)) && ((s.v[946] != 0.0) && (!(((s.v[943] != 0.0) || (s.v[944] != 0.0)) || (s.v[945] != 0.0))))) && (!(s.v[1023] != 0.0))) && (!(s.v[1035] != 0.0))) && ((s.v[1042] != 0.0) && (!(s.v[1041] != 0.0)))) && (!(s.v[1045] != 0.0))) {
            s.store_div_from_scalar_ad(705, (p.p374 * s.v[29]), A::scale(s.ad_value(701), (6.0 * s.v[236])));
        }

        if ((((((!(s.v[935] != 0.0)) && (s.v[936] != 0.0)) && ((s.v[946] != 0.0) && (!(((s.v[943] != 0.0) || (s.v[944] != 0.0)) || (s.v[945] != 0.0))))) && (!(s.v[1023] != 0.0))) && (!(s.v[1035] != 0.0))) && (!((s.v[1041] != 0.0) || (s.v[1042] != 0.0)))) {
            s.store_scalar(705, 0.0);
        }

        s.v[1046] = if (1.0 == 1.0) { 1.0 } else { 0.0 };

        s.v[1047] = if (1.0 == 1.0) { 1.0 } else { 0.0 };

        s.v[1048] = if (((p.p10 == 1.0) || (p.p10 == 2.0)) || (p.p10 == 5.0)) { 1.0 } else { 0.0 };

        s.v[1049] = if (((p.p10 == 3.0) || (p.p10 == 4.0)) || (p.p10 == 6.0)) { 1.0 } else { 0.0 };

        s.v[1050] = if (s.v[703] == 0.0) { 1.0 } else { 0.0 };

        if (((((((!(s.v[935] != 0.0)) && (s.v[936] != 0.0)) && ((s.v[947] != 0.0) && (!((((s.v[943] != 0.0) || (s.v[944] != 0.0)) || (s.v[945] != 0.0)) || (s.v[946] != 0.0))))) && (s.v[1046] != 0.0)) && (s.v[1047] != 0.0)) && (s.v[1048] != 0.0)) && (s.v[1050] != 0.0)) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!(s.v[935] != 0.0)) && (s.v[936] != 0.0)) && ((s.v[947] != 0.0) && (!((((s.v[943] != 0.0) || (s.v[944] != 0.0)) || (s.v[945] != 0.0)) || (s.v[946] != 0.0))))) && (s.v[1046] != 0.0)) && (s.v[1047] != 0.0)) && (s.v[1048] != 0.0)) && (!(s.v[1050] != 0.0))) {
            s.store_div_from_scalar_ad(705, (p.p374 * s.v[236]), A::scale(s.ad_value(703), s.v[29]));
        }

        s.v[1052] = if ((s.v[703] == 0.0) || ((s.v[236] + s.v[238]) == 0.0)) { 1.0 } else { 0.0 };

        if (((((((!(s.v[935] != 0.0)) && (s.v[936] != 0.0)) && ((s.v[947] != 0.0) && (!((((s.v[943] != 0.0) || (s.v[944] != 0.0)) || (s.v[945] != 0.0)) || (s.v[946] != 0.0))))) && (s.v[1046] != 0.0)) && (s.v[1047] != 0.0)) && ((s.v[1049] != 0.0) && (!(s.v[1048] != 0.0)))) && (s.v[1052] != 0.0)) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!(s.v[935] != 0.0)) && (s.v[936] != 0.0)) && ((s.v[947] != 0.0) && (!((((s.v[943] != 0.0) || (s.v[944] != 0.0)) || (s.v[945] != 0.0)) || (s.v[946] != 0.0))))) && (s.v[1046] != 0.0)) && (s.v[1047] != 0.0)) && ((s.v[1049] != 0.0) && (!(s.v[1048] != 0.0)))) && (!(s.v[1052] != 0.0))) {
            s.store_div_from_scalar_ad(705, (p.p374 * s.v[29]), A::scale(s.ad_value(703), (3.0 * (s.v[236] + s.v[238]))));
        }

        if ((((((!(s.v[935] != 0.0)) && (s.v[936] != 0.0)) && ((s.v[947] != 0.0) && (!((((s.v[943] != 0.0) || (s.v[944] != 0.0)) || (s.v[945] != 0.0)) || (s.v[946] != 0.0))))) && (s.v[1046] != 0.0)) && (s.v[1047] != 0.0)) && (!((s.v[1048] != 0.0) || (s.v[1049] != 0.0)))) {
            s.store_scalar(705, 0.0);
        }

        s.v[1053] = if (((p.p10 == 1.0) || (p.p10 == 3.0)) || (p.p10 == 7.0)) { 1.0 } else { 0.0 };

        s.v[1054] = if (((p.p10 == 2.0) || (p.p10 == 4.0)) || (p.p10 == 8.0)) { 1.0 } else { 0.0 };

        s.v[1055] = if (s.v[703] == 0.0) { 1.0 } else { 0.0 };

        if (((((((!(s.v[935] != 0.0)) && (s.v[936] != 0.0)) && ((s.v[947] != 0.0) && (!((((s.v[943] != 0.0) || (s.v[944] != 0.0)) || (s.v[945] != 0.0)) || (s.v[946] != 0.0))))) && (s.v[1046] != 0.0)) && (!(s.v[1047] != 0.0))) && (s.v[1053] != 0.0)) && (s.v[1055] != 0.0)) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!(s.v[935] != 0.0)) && (s.v[936] != 0.0)) && ((s.v[947] != 0.0) && (!((((s.v[943] != 0.0) || (s.v[944] != 0.0)) || (s.v[945] != 0.0)) || (s.v[946] != 0.0))))) && (s.v[1046] != 0.0)) && (!(s.v[1047] != 0.0))) && (s.v[1053] != 0.0)) && (!(s.v[1055] != 0.0))) {
            s.store_div_from_scalar_ad(705, (p.p374 * s.v[236]), A::scale(s.ad_value(703), s.v[29]));
        }

        s.v[1057] = if ((s.v[703] == 0.0) || ((s.v[236] + s.v[238]) == 0.0)) { 1.0 } else { 0.0 };

        if (((((((!(s.v[935] != 0.0)) && (s.v[936] != 0.0)) && ((s.v[947] != 0.0) && (!((((s.v[943] != 0.0) || (s.v[944] != 0.0)) || (s.v[945] != 0.0)) || (s.v[946] != 0.0))))) && (s.v[1046] != 0.0)) && (!(s.v[1047] != 0.0))) && ((s.v[1054] != 0.0) && (!(s.v[1053] != 0.0)))) && (s.v[1057] != 0.0)) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!(s.v[935] != 0.0)) && (s.v[936] != 0.0)) && ((s.v[947] != 0.0) && (!((((s.v[943] != 0.0) || (s.v[944] != 0.0)) || (s.v[945] != 0.0)) || (s.v[946] != 0.0))))) && (s.v[1046] != 0.0)) && (!(s.v[1047] != 0.0))) && ((s.v[1054] != 0.0) && (!(s.v[1053] != 0.0)))) && (!(s.v[1057] != 0.0))) {
            s.store_div_from_scalar_ad(705, (p.p374 * s.v[29]), A::scale(s.ad_value(703), (3.0 * (s.v[236] + s.v[238]))));
        }

        if ((((((!(s.v[935] != 0.0)) && (s.v[936] != 0.0)) && ((s.v[947] != 0.0) && (!((((s.v[943] != 0.0) || (s.v[944] != 0.0)) || (s.v[945] != 0.0)) || (s.v[946] != 0.0))))) && (s.v[1046] != 0.0)) && (!(s.v[1047] != 0.0))) && (!((s.v[1053] != 0.0) || (s.v[1054] != 0.0)))) {
            s.store_scalar(705, 0.0);
        }

        if ((((!(s.v[935] != 0.0)) && (s.v[936] != 0.0)) && ((s.v[947] != 0.0) && (!((((s.v[943] != 0.0) || (s.v[944] != 0.0)) || (s.v[945] != 0.0)) || (s.v[946] != 0.0))))) && (!(s.v[1046] != 0.0))) {
            s.store_scalar(705, ((p.p374 * s.v[237]) / s.v[29]));
        }

        s.v[1058] = if (1.0 == 1.0) { 1.0 } else { 0.0 };

        s.v[1059] = if (1.0 == 1.0) { 1.0 } else { 0.0 };

        s.v[1060] = if (((p.p10 == 1.0) || (p.p10 == 2.0)) || (p.p10 == 5.0)) { 1.0 } else { 0.0 };

        s.v[1061] = if (((p.p10 == 3.0) || (p.p10 == 4.0)) || (p.p10 == 6.0)) { 1.0 } else { 0.0 };

        s.v[1062] = if (s.v[703] == 0.0) { 1.0 } else { 0.0 };

        if (((((((!(s.v[935] != 0.0)) && (s.v[936] != 0.0)) && ((s.v[948] != 0.0) && (!(((((s.v[943] != 0.0) || (s.v[944] != 0.0)) || (s.v[945] != 0.0)) || (s.v[946] != 0.0)) || (s.v[947] != 0.0))))) && (s.v[1058] != 0.0)) && (s.v[1059] != 0.0)) && (s.v[1060] != 0.0)) && (s.v[1062] != 0.0)) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!(s.v[935] != 0.0)) && (s.v[936] != 0.0)) && ((s.v[948] != 0.0) && (!(((((s.v[943] != 0.0) || (s.v[944] != 0.0)) || (s.v[945] != 0.0)) || (s.v[946] != 0.0)) || (s.v[947] != 0.0))))) && (s.v[1058] != 0.0)) && (s.v[1059] != 0.0)) && (s.v[1060] != 0.0)) && (!(s.v[1062] != 0.0))) {
            s.store_div_from_scalar_ad(705, (p.p374 * s.v[236]), A::scale(s.ad_value(703), s.v[29]));
        }

        s.v[1064] = if ((s.v[703] == 0.0) || (s.v[236] == 0.0)) { 1.0 } else { 0.0 };

        if (((((((!(s.v[935] != 0.0)) && (s.v[936] != 0.0)) && ((s.v[948] != 0.0) && (!(((((s.v[943] != 0.0) || (s.v[944] != 0.0)) || (s.v[945] != 0.0)) || (s.v[946] != 0.0)) || (s.v[947] != 0.0))))) && (s.v[1058] != 0.0)) && (s.v[1059] != 0.0)) && ((s.v[1061] != 0.0) && (!(s.v[1060] != 0.0)))) && (s.v[1064] != 0.0)) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!(s.v[935] != 0.0)) && (s.v[936] != 0.0)) && ((s.v[948] != 0.0) && (!(((((s.v[943] != 0.0) || (s.v[944] != 0.0)) || (s.v[945] != 0.0)) || (s.v[946] != 0.0)) || (s.v[947] != 0.0))))) && (s.v[1058] != 0.0)) && (s.v[1059] != 0.0)) && ((s.v[1061] != 0.0) && (!(s.v[1060] != 0.0)))) && (!(s.v[1064] != 0.0))) {
            s.store_div_from_scalar_ad(705, (p.p374 * s.v[29]), A::scale(s.ad_value(703), (6.0 * s.v[236])));
        }

        if ((((((!(s.v[935] != 0.0)) && (s.v[936] != 0.0)) && ((s.v[948] != 0.0) && (!(((((s.v[943] != 0.0) || (s.v[944] != 0.0)) || (s.v[945] != 0.0)) || (s.v[946] != 0.0)) || (s.v[947] != 0.0))))) && (s.v[1058] != 0.0)) && (s.v[1059] != 0.0)) && (!((s.v[1060] != 0.0) || (s.v[1061] != 0.0)))) {
            s.store_scalar(705, 0.0);
        }

        s.v[1065] = if (((p.p10 == 1.0) || (p.p10 == 3.0)) || (p.p10 == 7.0)) { 1.0 } else { 0.0 };

        s.v[1066] = if (((p.p10 == 2.0) || (p.p10 == 4.0)) || (p.p10 == 8.0)) { 1.0 } else { 0.0 };

        s.v[1067] = if (s.v[703] == 0.0) { 1.0 } else { 0.0 };

        if (((((((!(s.v[935] != 0.0)) && (s.v[936] != 0.0)) && ((s.v[948] != 0.0) && (!(((((s.v[943] != 0.0) || (s.v[944] != 0.0)) || (s.v[945] != 0.0)) || (s.v[946] != 0.0)) || (s.v[947] != 0.0))))) && (s.v[1058] != 0.0)) && (!(s.v[1059] != 0.0))) && (s.v[1065] != 0.0)) && (s.v[1067] != 0.0)) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!(s.v[935] != 0.0)) && (s.v[936] != 0.0)) && ((s.v[948] != 0.0) && (!(((((s.v[943] != 0.0) || (s.v[944] != 0.0)) || (s.v[945] != 0.0)) || (s.v[946] != 0.0)) || (s.v[947] != 0.0))))) && (s.v[1058] != 0.0)) && (!(s.v[1059] != 0.0))) && (s.v[1065] != 0.0)) && (!(s.v[1067] != 0.0))) {
            s.store_div_from_scalar_ad(705, (p.p374 * s.v[236]), A::scale(s.ad_value(703), s.v[29]));
        }

        s.v[1069] = if ((s.v[703] == 0.0) || (s.v[236] == 0.0)) { 1.0 } else { 0.0 };

        if (((((((!(s.v[935] != 0.0)) && (s.v[936] != 0.0)) && ((s.v[948] != 0.0) && (!(((((s.v[943] != 0.0) || (s.v[944] != 0.0)) || (s.v[945] != 0.0)) || (s.v[946] != 0.0)) || (s.v[947] != 0.0))))) && (s.v[1058] != 0.0)) && (!(s.v[1059] != 0.0))) && ((s.v[1066] != 0.0) && (!(s.v[1065] != 0.0)))) && (s.v[1069] != 0.0)) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!(s.v[935] != 0.0)) && (s.v[936] != 0.0)) && ((s.v[948] != 0.0) && (!(((((s.v[943] != 0.0) || (s.v[944] != 0.0)) || (s.v[945] != 0.0)) || (s.v[946] != 0.0)) || (s.v[947] != 0.0))))) && (s.v[1058] != 0.0)) && (!(s.v[1059] != 0.0))) && ((s.v[1066] != 0.0) && (!(s.v[1065] != 0.0)))) && (!(s.v[1069] != 0.0))) {
            s.store_div_from_scalar_ad(705, (p.p374 * s.v[29]), A::scale(s.ad_value(703), (6.0 * s.v[236])));
        }

        if ((((((!(s.v[935] != 0.0)) && (s.v[936] != 0.0)) && ((s.v[948] != 0.0) && (!(((((s.v[943] != 0.0) || (s.v[944] != 0.0)) || (s.v[945] != 0.0)) || (s.v[946] != 0.0)) || (s.v[947] != 0.0))))) && (s.v[1058] != 0.0)) && (!(s.v[1059] != 0.0))) && (!((s.v[1065] != 0.0) || (s.v[1066] != 0.0)))) {
            s.store_scalar(705, 0.0);
        }

        s.v[1070] = if (s.v[701] == 0.0) { 1.0 } else { 0.0 };

        if (((((!(s.v[935] != 0.0)) && (s.v[936] != 0.0)) && ((s.v[948] != 0.0) && (!(((((s.v[943] != 0.0) || (s.v[944] != 0.0)) || (s.v[945] != 0.0)) || (s.v[946] != 0.0)) || (s.v[947] != 0.0))))) && (!(s.v[1058] != 0.0))) && (s.v[1070] != 0.0)) {
            s.store_scalar(705, 0.0);
        }

        if (((((!(s.v[935] != 0.0)) && (s.v[936] != 0.0)) && ((s.v[948] != 0.0) && (!(((((s.v[943] != 0.0) || (s.v[944] != 0.0)) || (s.v[945] != 0.0)) || (s.v[946] != 0.0)) || (s.v[947] != 0.0))))) && (!(s.v[1058] != 0.0))) && (!(s.v[1070] != 0.0))) {
            s.store_div_from_scalar_ad(705, (p.p374 * s.v[237]), A::scale(s.ad_value(701), s.v[29]));
        }

        s.v[1071] = if (1.0 == 1.0) { 1.0 } else { 0.0 };

        if ((((!(s.v[935] != 0.0)) && (s.v[936] != 0.0)) && ((s.v[949] != 0.0) && (!((((((s.v[943] != 0.0) || (s.v[944] != 0.0)) || (s.v[945] != 0.0)) || (s.v[946] != 0.0)) || (s.v[947] != 0.0)) || (s.v[948] != 0.0))))) && (s.v[1071] != 0.0)) {
            s.store_scalar(705, ((p.p374 * s.v[237]) / s.v[29]));
        }

        s.v[1072] = if (0.0 == 1.0) { 1.0 } else { 0.0 };

        s.v[1073] = if (((p.p10 == 1.0) || (p.p10 == 2.0)) || (p.p10 == 5.0)) { 1.0 } else { 0.0 };

        s.v[1074] = if (((p.p10 == 3.0) || (p.p10 == 4.0)) || (p.p10 == 6.0)) { 1.0 } else { 0.0 };

        s.v[1075] = if (s.v[701] == 0.0) { 1.0 } else { 0.0 };

        if (((((((!(s.v[935] != 0.0)) && (s.v[936] != 0.0)) && ((s.v[949] != 0.0) && (!((((((s.v[943] != 0.0) || (s.v[944] != 0.0)) || (s.v[945] != 0.0)) || (s.v[946] != 0.0)) || (s.v[947] != 0.0)) || (s.v[948] != 0.0))))) && (!(s.v[1071] != 0.0))) && (s.v[1072] != 0.0)) && (s.v[1073] != 0.0)) && (s.v[1075] != 0.0)) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!(s.v[935] != 0.0)) && (s.v[936] != 0.0)) && ((s.v[949] != 0.0) && (!((((((s.v[943] != 0.0) || (s.v[944] != 0.0)) || (s.v[945] != 0.0)) || (s.v[946] != 0.0)) || (s.v[947] != 0.0)) || (s.v[948] != 0.0))))) && (!(s.v[1071] != 0.0))) && (s.v[1072] != 0.0)) && (s.v[1073] != 0.0)) && (!(s.v[1075] != 0.0))) {
            s.store_div_from_scalar_ad(705, (p.p374 * s.v[236]), A::scale(s.ad_value(701), s.v[29]));
        }

        s.v[1077] = if ((s.v[701] == 0.0) || ((s.v[236] + s.v[238]) == 0.0)) { 1.0 } else { 0.0 };

        if (((((((!(s.v[935] != 0.0)) && (s.v[936] != 0.0)) && ((s.v[949] != 0.0) && (!((((((s.v[943] != 0.0) || (s.v[944] != 0.0)) || (s.v[945] != 0.0)) || (s.v[946] != 0.0)) || (s.v[947] != 0.0)) || (s.v[948] != 0.0))))) && (!(s.v[1071] != 0.0))) && (s.v[1072] != 0.0)) && ((s.v[1074] != 0.0) && (!(s.v[1073] != 0.0)))) && (s.v[1077] != 0.0)) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!(s.v[935] != 0.0)) && (s.v[936] != 0.0)) && ((s.v[949] != 0.0) && (!((((((s.v[943] != 0.0) || (s.v[944] != 0.0)) || (s.v[945] != 0.0)) || (s.v[946] != 0.0)) || (s.v[947] != 0.0)) || (s.v[948] != 0.0))))) && (!(s.v[1071] != 0.0))) && (s.v[1072] != 0.0)) && ((s.v[1074] != 0.0) && (!(s.v[1073] != 0.0)))) && (!(s.v[1077] != 0.0))) {
            s.store_div_from_scalar_ad(705, (p.p374 * s.v[29]), A::scale(s.ad_value(701), (3.0 * (s.v[236] + s.v[238]))));
        }

        if ((((((!(s.v[935] != 0.0)) && (s.v[936] != 0.0)) && ((s.v[949] != 0.0) && (!((((((s.v[943] != 0.0) || (s.v[944] != 0.0)) || (s.v[945] != 0.0)) || (s.v[946] != 0.0)) || (s.v[947] != 0.0)) || (s.v[948] != 0.0))))) && (!(s.v[1071] != 0.0))) && (s.v[1072] != 0.0)) && (!((s.v[1073] != 0.0) || (s.v[1074] != 0.0)))) {
            s.store_scalar(705, 0.0);
        }

        s.v[1078] = if (((p.p10 == 1.0) || (p.p10 == 3.0)) || (p.p10 == 7.0)) { 1.0 } else { 0.0 };

        s.v[1079] = if (((p.p10 == 2.0) || (p.p10 == 4.0)) || (p.p10 == 8.0)) { 1.0 } else { 0.0 };

        s.v[1080] = if (s.v[701] == 0.0) { 1.0 } else { 0.0 };

        if (((((((!(s.v[935] != 0.0)) && (s.v[936] != 0.0)) && ((s.v[949] != 0.0) && (!((((((s.v[943] != 0.0) || (s.v[944] != 0.0)) || (s.v[945] != 0.0)) || (s.v[946] != 0.0)) || (s.v[947] != 0.0)) || (s.v[948] != 0.0))))) && (!(s.v[1071] != 0.0))) && (!(s.v[1072] != 0.0))) && (s.v[1078] != 0.0)) && (s.v[1080] != 0.0)) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!(s.v[935] != 0.0)) && (s.v[936] != 0.0)) && ((s.v[949] != 0.0) && (!((((((s.v[943] != 0.0) || (s.v[944] != 0.0)) || (s.v[945] != 0.0)) || (s.v[946] != 0.0)) || (s.v[947] != 0.0)) || (s.v[948] != 0.0))))) && (!(s.v[1071] != 0.0))) && (!(s.v[1072] != 0.0))) && (s.v[1078] != 0.0)) && (!(s.v[1080] != 0.0))) {
            s.store_div_from_scalar_ad(705, (p.p374 * s.v[236]), A::scale(s.ad_value(701), s.v[29]));
        }

        s.v[1082] = if ((s.v[701] == 0.0) || ((s.v[236] + s.v[238]) == 0.0)) { 1.0 } else { 0.0 };

        if (((((((!(s.v[935] != 0.0)) && (s.v[936] != 0.0)) && ((s.v[949] != 0.0) && (!((((((s.v[943] != 0.0) || (s.v[944] != 0.0)) || (s.v[945] != 0.0)) || (s.v[946] != 0.0)) || (s.v[947] != 0.0)) || (s.v[948] != 0.0))))) && (!(s.v[1071] != 0.0))) && (!(s.v[1072] != 0.0))) && ((s.v[1079] != 0.0) && (!(s.v[1078] != 0.0)))) && (s.v[1082] != 0.0)) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!(s.v[935] != 0.0)) && (s.v[936] != 0.0)) && ((s.v[949] != 0.0) && (!((((((s.v[943] != 0.0) || (s.v[944] != 0.0)) || (s.v[945] != 0.0)) || (s.v[946] != 0.0)) || (s.v[947] != 0.0)) || (s.v[948] != 0.0))))) && (!(s.v[1071] != 0.0))) && (!(s.v[1072] != 0.0))) && ((s.v[1079] != 0.0) && (!(s.v[1078] != 0.0)))) && (!(s.v[1082] != 0.0))) {
            s.store_div_from_scalar_ad(705, (p.p374 * s.v[29]), A::scale(s.ad_value(701), (3.0 * (s.v[236] + s.v[238]))));
        }

        if ((((((!(s.v[935] != 0.0)) && (s.v[936] != 0.0)) && ((s.v[949] != 0.0) && (!((((((s.v[943] != 0.0) || (s.v[944] != 0.0)) || (s.v[945] != 0.0)) || (s.v[946] != 0.0)) || (s.v[947] != 0.0)) || (s.v[948] != 0.0))))) && (!(s.v[1071] != 0.0))) && (!(s.v[1072] != 0.0))) && (!((s.v[1078] != 0.0) || (s.v[1079] != 0.0)))) {
            s.store_scalar(705, 0.0);
        }

        s.v[1083] = if (1.0 == 1.0) { 1.0 } else { 0.0 };

        s.v[1084] = if (s.v[703] == 0.0) { 1.0 } else { 0.0 };

        if (((((!(s.v[935] != 0.0)) && (s.v[936] != 0.0)) && ((s.v[950] != 0.0) && (!(((((((s.v[943] != 0.0) || (s.v[944] != 0.0)) || (s.v[945] != 0.0)) || (s.v[946] != 0.0)) || (s.v[947] != 0.0)) || (s.v[948] != 0.0)) || (s.v[949] != 0.0))))) && (s.v[1083] != 0.0)) && (s.v[1084] != 0.0)) {
            s.store_scalar(705, 0.0);
        }

        if (((((!(s.v[935] != 0.0)) && (s.v[936] != 0.0)) && ((s.v[950] != 0.0) && (!(((((((s.v[943] != 0.0) || (s.v[944] != 0.0)) || (s.v[945] != 0.0)) || (s.v[946] != 0.0)) || (s.v[947] != 0.0)) || (s.v[948] != 0.0)) || (s.v[949] != 0.0))))) && (s.v[1083] != 0.0)) && (!(s.v[1084] != 0.0))) {
            s.store_div_from_scalar_ad(705, (p.p374 * s.v[237]), A::scale(s.ad_value(703), s.v[29]));
        }

        s.v[1085] = if (0.0 == 1.0) { 1.0 } else { 0.0 };

        s.v[1086] = if (((p.p10 == 1.0) || (p.p10 == 2.0)) || (p.p10 == 5.0)) { 1.0 } else { 0.0 };

        s.v[1087] = if (((p.p10 == 3.0) || (p.p10 == 4.0)) || (p.p10 == 6.0)) { 1.0 } else { 0.0 };

        s.v[1088] = if (s.v[701] == 0.0) { 1.0 } else { 0.0 };

        if (((((((!(s.v[935] != 0.0)) && (s.v[936] != 0.0)) && ((s.v[950] != 0.0) && (!(((((((s.v[943] != 0.0) || (s.v[944] != 0.0)) || (s.v[945] != 0.0)) || (s.v[946] != 0.0)) || (s.v[947] != 0.0)) || (s.v[948] != 0.0)) || (s.v[949] != 0.0))))) && (!(s.v[1083] != 0.0))) && (s.v[1085] != 0.0)) && (s.v[1086] != 0.0)) && (s.v[1088] != 0.0)) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!(s.v[935] != 0.0)) && (s.v[936] != 0.0)) && ((s.v[950] != 0.0) && (!(((((((s.v[943] != 0.0) || (s.v[944] != 0.0)) || (s.v[945] != 0.0)) || (s.v[946] != 0.0)) || (s.v[947] != 0.0)) || (s.v[948] != 0.0)) || (s.v[949] != 0.0))))) && (!(s.v[1083] != 0.0))) && (s.v[1085] != 0.0)) && (s.v[1086] != 0.0)) && (!(s.v[1088] != 0.0))) {
            s.store_div_from_scalar_ad(705, (p.p374 * s.v[236]), A::scale(s.ad_value(701), s.v[29]));
        }

        s.v[1090] = if ((s.v[701] == 0.0) || (s.v[236] == 0.0)) { 1.0 } else { 0.0 };

        if (((((((!(s.v[935] != 0.0)) && (s.v[936] != 0.0)) && ((s.v[950] != 0.0) && (!(((((((s.v[943] != 0.0) || (s.v[944] != 0.0)) || (s.v[945] != 0.0)) || (s.v[946] != 0.0)) || (s.v[947] != 0.0)) || (s.v[948] != 0.0)) || (s.v[949] != 0.0))))) && (!(s.v[1083] != 0.0))) && (s.v[1085] != 0.0)) && ((s.v[1087] != 0.0) && (!(s.v[1086] != 0.0)))) && (s.v[1090] != 0.0)) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!(s.v[935] != 0.0)) && (s.v[936] != 0.0)) && ((s.v[950] != 0.0) && (!(((((((s.v[943] != 0.0) || (s.v[944] != 0.0)) || (s.v[945] != 0.0)) || (s.v[946] != 0.0)) || (s.v[947] != 0.0)) || (s.v[948] != 0.0)) || (s.v[949] != 0.0))))) && (!(s.v[1083] != 0.0))) && (s.v[1085] != 0.0)) && ((s.v[1087] != 0.0) && (!(s.v[1086] != 0.0)))) && (!(s.v[1090] != 0.0))) {
            s.store_div_from_scalar_ad(705, (p.p374 * s.v[29]), A::scale(s.ad_value(701), (6.0 * s.v[236])));
        }

        if ((((((!(s.v[935] != 0.0)) && (s.v[936] != 0.0)) && ((s.v[950] != 0.0) && (!(((((((s.v[943] != 0.0) || (s.v[944] != 0.0)) || (s.v[945] != 0.0)) || (s.v[946] != 0.0)) || (s.v[947] != 0.0)) || (s.v[948] != 0.0)) || (s.v[949] != 0.0))))) && (!(s.v[1083] != 0.0))) && (s.v[1085] != 0.0)) && (!((s.v[1086] != 0.0) || (s.v[1087] != 0.0)))) {
            s.store_scalar(705, 0.0);
        }

        s.v[1091] = if (((p.p10 == 1.0) || (p.p10 == 3.0)) || (p.p10 == 7.0)) { 1.0 } else { 0.0 };

        s.v[1092] = if (((p.p10 == 2.0) || (p.p10 == 4.0)) || (p.p10 == 8.0)) { 1.0 } else { 0.0 };

        s.v[1093] = if (s.v[701] == 0.0) { 1.0 } else { 0.0 };

        if (((((((!(s.v[935] != 0.0)) && (s.v[936] != 0.0)) && ((s.v[950] != 0.0) && (!(((((((s.v[943] != 0.0) || (s.v[944] != 0.0)) || (s.v[945] != 0.0)) || (s.v[946] != 0.0)) || (s.v[947] != 0.0)) || (s.v[948] != 0.0)) || (s.v[949] != 0.0))))) && (!(s.v[1083] != 0.0))) && (!(s.v[1085] != 0.0))) && (s.v[1091] != 0.0)) && (s.v[1093] != 0.0)) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!(s.v[935] != 0.0)) && (s.v[936] != 0.0)) && ((s.v[950] != 0.0) && (!(((((((s.v[943] != 0.0) || (s.v[944] != 0.0)) || (s.v[945] != 0.0)) || (s.v[946] != 0.0)) || (s.v[947] != 0.0)) || (s.v[948] != 0.0)) || (s.v[949] != 0.0))))) && (!(s.v[1083] != 0.0))) && (!(s.v[1085] != 0.0))) && (s.v[1091] != 0.0)) && (!(s.v[1093] != 0.0))) {
            s.store_div_from_scalar_ad(705, (p.p374 * s.v[236]), A::scale(s.ad_value(701), s.v[29]));
        }

        s.v[1095] = if ((s.v[701] == 0.0) || (s.v[236] == 0.0)) { 1.0 } else { 0.0 };

        if (((((((!(s.v[935] != 0.0)) && (s.v[936] != 0.0)) && ((s.v[950] != 0.0) && (!(((((((s.v[943] != 0.0) || (s.v[944] != 0.0)) || (s.v[945] != 0.0)) || (s.v[946] != 0.0)) || (s.v[947] != 0.0)) || (s.v[948] != 0.0)) || (s.v[949] != 0.0))))) && (!(s.v[1083] != 0.0))) && (!(s.v[1085] != 0.0))) && ((s.v[1092] != 0.0) && (!(s.v[1091] != 0.0)))) && (s.v[1095] != 0.0)) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!(s.v[935] != 0.0)) && (s.v[936] != 0.0)) && ((s.v[950] != 0.0) && (!(((((((s.v[943] != 0.0) || (s.v[944] != 0.0)) || (s.v[945] != 0.0)) || (s.v[946] != 0.0)) || (s.v[947] != 0.0)) || (s.v[948] != 0.0)) || (s.v[949] != 0.0))))) && (!(s.v[1083] != 0.0))) && (!(s.v[1085] != 0.0))) && ((s.v[1092] != 0.0) && (!(s.v[1091] != 0.0)))) && (!(s.v[1095] != 0.0))) {
            s.store_div_from_scalar_ad(705, (p.p374 * s.v[29]), A::scale(s.ad_value(701), (6.0 * s.v[236])));
        }

        if ((((((!(s.v[935] != 0.0)) && (s.v[936] != 0.0)) && ((s.v[950] != 0.0) && (!(((((((s.v[943] != 0.0) || (s.v[944] != 0.0)) || (s.v[945] != 0.0)) || (s.v[946] != 0.0)) || (s.v[947] != 0.0)) || (s.v[948] != 0.0)) || (s.v[949] != 0.0))))) && (!(s.v[1083] != 0.0))) && (!(s.v[1085] != 0.0))) && (!((s.v[1091] != 0.0) || (s.v[1092] != 0.0)))) {
            s.store_scalar(705, 0.0);
        }

        if (((!(s.v[935] != 0.0)) && (s.v[936] != 0.0)) && ((s.v[951] != 0.0) && (!((((((((s.v[943] != 0.0) || (s.v[944] != 0.0)) || (s.v[945] != 0.0)) || (s.v[946] != 0.0)) || (s.v[947] != 0.0)) || (s.v[948] != 0.0)) || (s.v[949] != 0.0)) || (s.v[950] != 0.0))))) {
            s.store_scalar(705, ((p.p374 * s.v[237]) / s.v[29]));
        }

        s.v[1096] = if (1.0 == 1.0) { 1.0 } else { 0.0 };

        if ((((!(s.v[935] != 0.0)) && (s.v[936] != 0.0)) && ((s.v[952] != 0.0) && (!(((((((((s.v[943] != 0.0) || (s.v[944] != 0.0)) || (s.v[945] != 0.0)) || (s.v[946] != 0.0)) || (s.v[947] != 0.0)) || (s.v[948] != 0.0)) || (s.v[949] != 0.0)) || (s.v[950] != 0.0)) || (s.v[951] != 0.0))))) && (s.v[1096] != 0.0)) {
            s.store_scalar(705, (((0.5 * p.p374) * s.v[236]) / s.v[29]));
        }

        s.v[1097] = if (p.p2 == 2.0) { 1.0 } else { 0.0 };

        if (((((!(s.v[935] != 0.0)) && (s.v[936] != 0.0)) && ((s.v[952] != 0.0) && (!(((((((((s.v[943] != 0.0) || (s.v[944] != 0.0)) || (s.v[945] != 0.0)) || (s.v[946] != 0.0)) || (s.v[947] != 0.0)) || (s.v[948] != 0.0)) || (s.v[949] != 0.0)) || (s.v[950] != 0.0)) || (s.v[951] != 0.0))))) && (s.v[1096] != 0.0)) && (s.v[1097] != 0.0)) {
            s.store_scalar(704, 0.0);
        }

        if (((((!(s.v[935] != 0.0)) && (s.v[936] != 0.0)) && ((s.v[952] != 0.0) && (!(((((((((s.v[943] != 0.0) || (s.v[944] != 0.0)) || (s.v[945] != 0.0)) || (s.v[946] != 0.0)) || (s.v[947] != 0.0)) || (s.v[948] != 0.0)) || (s.v[949] != 0.0)) || (s.v[950] != 0.0)) || (s.v[951] != 0.0))))) && (s.v[1096] != 0.0)) && (!(s.v[1097] != 0.0))) {
            s.store_scalar(704, ((p.p374 * s.v[236]) / (s.v[29] * (p.p2 - 2.0))));
        }

        if ((((!(s.v[935] != 0.0)) && (s.v[936] != 0.0)) && ((s.v[952] != 0.0) && (!(((((((((s.v[943] != 0.0) || (s.v[944] != 0.0)) || (s.v[945] != 0.0)) || (s.v[946] != 0.0)) || (s.v[947] != 0.0)) || (s.v[948] != 0.0)) || (s.v[949] != 0.0)) || (s.v[950] != 0.0)) || (s.v[951] != 0.0))))) && (!(s.v[1096] != 0.0))) {
            s.store_scalar(705, 0.0);
        }

        if ((((!(s.v[935] != 0.0)) && (s.v[936] != 0.0)) && ((s.v[952] != 0.0) && (!(((((((((s.v[943] != 0.0) || (s.v[944] != 0.0)) || (s.v[945] != 0.0)) || (s.v[946] != 0.0)) || (s.v[947] != 0.0)) || (s.v[948] != 0.0)) || (s.v[949] != 0.0)) || (s.v[950] != 0.0)) || (s.v[951] != 0.0))))) && (!(s.v[1096] != 0.0))) {
            s.store_scalar(704, ((p.p374 * s.v[236]) / (s.v[29] * p.p2)));
        }

        s.v[1098] = if (1.0 == 1.0) { 1.0 } else { 0.0 };

        if ((((!(s.v[935] != 0.0)) && (s.v[936] != 0.0)) && ((s.v[953] != 0.0) && (!((((((((((s.v[943] != 0.0) || (s.v[944] != 0.0)) || (s.v[945] != 0.0)) || (s.v[946] != 0.0)) || (s.v[947] != 0.0)) || (s.v[948] != 0.0)) || (s.v[949] != 0.0)) || (s.v[950] != 0.0)) || (s.v[951] != 0.0)) || (s.v[952] != 0.0))))) && (s.v[1098] != 0.0)) {
            s.store_scalar(705, 0.0);
        }

        if ((((!(s.v[935] != 0.0)) && (s.v[936] != 0.0)) && ((s.v[953] != 0.0) && (!((((((((((s.v[943] != 0.0) || (s.v[944] != 0.0)) || (s.v[945] != 0.0)) || (s.v[946] != 0.0)) || (s.v[947] != 0.0)) || (s.v[948] != 0.0)) || (s.v[949] != 0.0)) || (s.v[950] != 0.0)) || (s.v[951] != 0.0)) || (s.v[952] != 0.0))))) && (s.v[1098] != 0.0)) {
            s.store_scalar(704, ((p.p374 * s.v[236]) / (s.v[29] * p.p2)));
        }

        if ((((!(s.v[935] != 0.0)) && (s.v[936] != 0.0)) && ((s.v[953] != 0.0) && (!((((((((((s.v[943] != 0.0) || (s.v[944] != 0.0)) || (s.v[945] != 0.0)) || (s.v[946] != 0.0)) || (s.v[947] != 0.0)) || (s.v[948] != 0.0)) || (s.v[949] != 0.0)) || (s.v[950] != 0.0)) || (s.v[951] != 0.0)) || (s.v[952] != 0.0))))) && (!(s.v[1098] != 0.0))) {
            s.store_scalar(705, (((0.5 * p.p374) * s.v[236]) / s.v[29]));
        }

        s.v[1099] = if (p.p2 == 2.0) { 1.0 } else { 0.0 };

        if (((((!(s.v[935] != 0.0)) && (s.v[936] != 0.0)) && ((s.v[953] != 0.0) && (!((((((((((s.v[943] != 0.0) || (s.v[944] != 0.0)) || (s.v[945] != 0.0)) || (s.v[946] != 0.0)) || (s.v[947] != 0.0)) || (s.v[948] != 0.0)) || (s.v[949] != 0.0)) || (s.v[950] != 0.0)) || (s.v[951] != 0.0)) || (s.v[952] != 0.0))))) && (!(s.v[1098] != 0.0))) && (s.v[1099] != 0.0)) {
            s.store_scalar(704, 0.0);
        }

        if (((((!(s.v[935] != 0.0)) && (s.v[936] != 0.0)) && ((s.v[953] != 0.0) && (!((((((((((s.v[943] != 0.0) || (s.v[944] != 0.0)) || (s.v[945] != 0.0)) || (s.v[946] != 0.0)) || (s.v[947] != 0.0)) || (s.v[948] != 0.0)) || (s.v[949] != 0.0)) || (s.v[950] != 0.0)) || (s.v[951] != 0.0)) || (s.v[952] != 0.0))))) && (!(s.v[1098] != 0.0))) && (!(s.v[1099] != 0.0))) {
            s.store_scalar(704, ((p.p374 * s.v[236]) / (s.v[29] * (p.p2 - 2.0))));
        }

        if (((!(s.v[935] != 0.0)) && (s.v[936] != 0.0)) && (!(((((((((((s.v[943] != 0.0) || (s.v[944] != 0.0)) || (s.v[945] != 0.0)) || (s.v[946] != 0.0)) || (s.v[947] != 0.0)) || (s.v[948] != 0.0)) || (s.v[949] != 0.0)) || (s.v[950] != 0.0)) || (s.v[951] != 0.0)) || (s.v[952] != 0.0)) || (s.v[953] != 0.0)))) {
            s.store_scalar(704, 0.0);
        }

        s.v[1100] = if (s.v[704] <= 0.0) { 1.0 } else { 0.0 };

        if (((!(s.v[935] != 0.0)) && (s.v[936] != 0.0)) && (s.v[1100] != 0.0)) {
            s.copy_ad(239, 705);
        }

        s.v[1101] = if (s.v[705] <= 0.0) { 1.0 } else { 0.0 };

        if ((((!(s.v[935] != 0.0)) && (s.v[936] != 0.0)) && (!(s.v[1100] != 0.0))) && (s.v[1101] != 0.0)) {
            s.copy_ad(239, 704);
        }

        if ((((!(s.v[935] != 0.0)) && (s.v[936] != 0.0)) && (!(s.v[1100] != 0.0))) && (!(s.v[1101] != 0.0))) {
            s.store_div_ad(239, A::mul(s.ad_value(704), s.ad_value(705)), A::add(s.ad_value(704), s.ad_value(705)));
        }

        if ((!(s.v[935] != 0.0)) && (!(s.v[936] != 0.0))) {
            s.store_scalar(239, 0.0);
        }

        s.v[1103] = if self.param_given[4] { 1.0 } else { 0.0 };

        if (s.v[1103] != 0.0) {
            s.store_scalar(240, (p.p374 * p.p4));
        }

        s.v[1104] = if ((p.p10 > 0.0) && (p.p374 > 0.0)) { 1.0 } else { 0.0 };

        s.v[1105] = if (p.p9 < 9.0) { 1.0 } else { 0.0 };

        s.v[1106] = if ((p.p2 % 2.0) != 0.0) { 1.0 } else { 0.0 };

        if ((((!(s.v[1103] != 0.0)) && (s.v[1104] != 0.0)) && (s.v[1105] != 0.0)) && (s.v[1106] != 0.0)) {
            s.store_scalar(701, 1.0);
        }

        if ((((!(s.v[1103] != 0.0)) && (s.v[1104] != 0.0)) && (s.v[1105] != 0.0)) && (s.v[1106] != 0.0)) {
            s.store_scalar(703, 1.0);
        }

        if ((((!(s.v[1103] != 0.0)) && (s.v[1104] != 0.0)) && (s.v[1105] != 0.0)) && (s.v[1106] != 0.0)) {
            s.store_scalar(700, (2.0 * (((p.p2 - 1.0) / 2.0)).max(0.0)));
        }

        if ((((!(s.v[1103] != 0.0)) && (s.v[1104] != 0.0)) && (s.v[1105] != 0.0)) && (s.v[1106] != 0.0)) {
            s.copy_ad(702, 700);
        }

        s.v[1107] = if (p.p6 == 1.0) { 1.0 } else { 0.0 };

        if (((((!(s.v[1103] != 0.0)) && (s.v[1104] != 0.0)) && (s.v[1105] != 0.0)) && (!(s.v[1106] != 0.0))) && (s.v[1107] != 0.0)) {
            s.store_scalar(701, 2.0);
        }

        if (((((!(s.v[1103] != 0.0)) && (s.v[1104] != 0.0)) && (s.v[1105] != 0.0)) && (!(s.v[1106] != 0.0))) && (s.v[1107] != 0.0)) {
            s.store_scalar(700, (2.0 * (((p.p2 / 2.0) - 1.0)).max(0.0)));
        }

        if (((((!(s.v[1103] != 0.0)) && (s.v[1104] != 0.0)) && (s.v[1105] != 0.0)) && (!(s.v[1106] != 0.0))) && (s.v[1107] != 0.0)) {
            s.store_scalar(703, 0.0);
        }

        if (((((!(s.v[1103] != 0.0)) && (s.v[1104] != 0.0)) && (s.v[1105] != 0.0)) && (!(s.v[1106] != 0.0))) && (s.v[1107] != 0.0)) {
            s.store_scalar(702, p.p2);
        }

        if (((((!(s.v[1103] != 0.0)) && (s.v[1104] != 0.0)) && (s.v[1105] != 0.0)) && (!(s.v[1106] != 0.0))) && (!(s.v[1107] != 0.0))) {
            s.store_scalar(701, 0.0);
        }

        if (((((!(s.v[1103] != 0.0)) && (s.v[1104] != 0.0)) && (s.v[1105] != 0.0)) && (!(s.v[1106] != 0.0))) && (!(s.v[1107] != 0.0))) {
            s.store_scalar(700, p.p2);
        }

        if (((((!(s.v[1103] != 0.0)) && (s.v[1104] != 0.0)) && (s.v[1105] != 0.0)) && (!(s.v[1106] != 0.0))) && (!(s.v[1107] != 0.0))) {
            s.store_scalar(703, 2.0);
        }

    }

    pub(super) fn stamp_reactive_block_4(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if (((((!(s.v[1103] != 0.0)) && (s.v[1104] != 0.0)) && (s.v[1105] != 0.0)) && (!(s.v[1106] != 0.0))) && (!(s.v[1107] != 0.0))) {
            s.store_scalar(702, (2.0 * (((p.p2 / 2.0) - 1.0)).max(0.0)));
        }

        s.v[1108] = if (0.0 == 1.0) { 1.0 } else { 0.0 };

        s.v[1109] = if (s.v[702] == 0.0) { 1.0 } else { 0.0 };

        if (((((!(s.v[1103] != 0.0)) && (s.v[1104] != 0.0)) && (s.v[1105] != 0.0)) && (s.v[1108] != 0.0)) && (s.v[1109] != 0.0)) {
            s.store_scalar(704, 0.0);
        }

        if (((((!(s.v[1103] != 0.0)) && (s.v[1104] != 0.0)) && (s.v[1105] != 0.0)) && (s.v[1108] != 0.0)) && (!(s.v[1109] != 0.0))) {
            s.store_div_from_scalar_ad(704, (p.p374 * s.v[236]), A::scale(s.ad_value(702), s.v[29]));
        }

        s.v[1110] = if (s.v[700] == 0.0) { 1.0 } else { 0.0 };

        if (((((!(s.v[1103] != 0.0)) && (s.v[1104] != 0.0)) && (s.v[1105] != 0.0)) && (!(s.v[1108] != 0.0))) && (s.v[1110] != 0.0)) {
            s.store_scalar(704, 0.0);
        }

        if (((((!(s.v[1103] != 0.0)) && (s.v[1104] != 0.0)) && (s.v[1105] != 0.0)) && (!(s.v[1108] != 0.0))) && (!(s.v[1110] != 0.0))) {
            s.store_div_from_scalar_ad(704, (p.p374 * s.v[236]), A::scale(s.ad_value(700), s.v[29]));
        }

        s.v[1111] = if (p.p9 == 0.0) { 1.0 } else { 0.0 };

        s.v[1112] = if (p.p9 == 1.0) { 1.0 } else { 0.0 };

        s.v[1113] = if (p.p9 == 2.0) { 1.0 } else { 0.0 };

        s.v[1114] = if (p.p9 == 3.0) { 1.0 } else { 0.0 };

        s.v[1115] = if (p.p9 == 4.0) { 1.0 } else { 0.0 };

        s.v[1116] = if (p.p9 == 5.0) { 1.0 } else { 0.0 };

        s.v[1117] = if (p.p9 == 6.0) { 1.0 } else { 0.0 };

        s.v[1118] = if (p.p9 == 7.0) { 1.0 } else { 0.0 };

        s.v[1119] = if (p.p9 == 8.0) { 1.0 } else { 0.0 };

        s.v[1120] = if (p.p9 == 9.0) { 1.0 } else { 0.0 };

        s.v[1121] = if (p.p9 == 10.0) { 1.0 } else { 0.0 };

        s.v[1122] = if (0.0 == 1.0) { 1.0 } else { 0.0 };

        s.v[1123] = if (1.0 == 1.0) { 1.0 } else { 0.0 };

        s.v[1124] = if (((p.p10 == 1.0) || (p.p10 == 2.0)) || (p.p10 == 5.0)) { 1.0 } else { 0.0 };

        s.v[1125] = if (((p.p10 == 3.0) || (p.p10 == 4.0)) || (p.p10 == 6.0)) { 1.0 } else { 0.0 };

        s.v[1126] = if (s.v[703] == 0.0) { 1.0 } else { 0.0 };

        if (((((((!(s.v[1103] != 0.0)) && (s.v[1104] != 0.0)) && (s.v[1111] != 0.0)) && (s.v[1122] != 0.0)) && (s.v[1123] != 0.0)) && (s.v[1124] != 0.0)) && (s.v[1126] != 0.0)) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!(s.v[1103] != 0.0)) && (s.v[1104] != 0.0)) && (s.v[1111] != 0.0)) && (s.v[1122] != 0.0)) && (s.v[1123] != 0.0)) && (s.v[1124] != 0.0)) && (!(s.v[1126] != 0.0))) {
            s.store_div_from_scalar_ad(705, (p.p374 * s.v[236]), A::scale(s.ad_value(703), s.v[29]));
        }

        s.v[1128] = if ((s.v[703] == 0.0) || ((s.v[236] + s.v[238]) == 0.0)) { 1.0 } else { 0.0 };

        if (((((((!(s.v[1103] != 0.0)) && (s.v[1104] != 0.0)) && (s.v[1111] != 0.0)) && (s.v[1122] != 0.0)) && (s.v[1123] != 0.0)) && ((s.v[1125] != 0.0) && (!(s.v[1124] != 0.0)))) && (s.v[1128] != 0.0)) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!(s.v[1103] != 0.0)) && (s.v[1104] != 0.0)) && (s.v[1111] != 0.0)) && (s.v[1122] != 0.0)) && (s.v[1123] != 0.0)) && ((s.v[1125] != 0.0) && (!(s.v[1124] != 0.0)))) && (!(s.v[1128] != 0.0))) {
            s.store_div_from_scalar_ad(705, (p.p374 * s.v[29]), A::scale(s.ad_value(703), (3.0 * (s.v[236] + s.v[238]))));
        }

        if ((((((!(s.v[1103] != 0.0)) && (s.v[1104] != 0.0)) && (s.v[1111] != 0.0)) && (s.v[1122] != 0.0)) && (s.v[1123] != 0.0)) && (!((s.v[1124] != 0.0) || (s.v[1125] != 0.0)))) {
            s.store_scalar(705, 0.0);
        }

        s.v[1129] = if (((p.p10 == 1.0) || (p.p10 == 3.0)) || (p.p10 == 7.0)) { 1.0 } else { 0.0 };

        s.v[1130] = if (((p.p10 == 2.0) || (p.p10 == 4.0)) || (p.p10 == 8.0)) { 1.0 } else { 0.0 };

        s.v[1131] = if (s.v[703] == 0.0) { 1.0 } else { 0.0 };

        if (((((((!(s.v[1103] != 0.0)) && (s.v[1104] != 0.0)) && (s.v[1111] != 0.0)) && (s.v[1122] != 0.0)) && (!(s.v[1123] != 0.0))) && (s.v[1129] != 0.0)) && (s.v[1131] != 0.0)) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!(s.v[1103] != 0.0)) && (s.v[1104] != 0.0)) && (s.v[1111] != 0.0)) && (s.v[1122] != 0.0)) && (!(s.v[1123] != 0.0))) && (s.v[1129] != 0.0)) && (!(s.v[1131] != 0.0))) {
            s.store_div_from_scalar_ad(705, (p.p374 * s.v[236]), A::scale(s.ad_value(703), s.v[29]));
        }

        s.v[1133] = if ((s.v[703] == 0.0) || ((s.v[236] + s.v[238]) == 0.0)) { 1.0 } else { 0.0 };

        if (((((((!(s.v[1103] != 0.0)) && (s.v[1104] != 0.0)) && (s.v[1111] != 0.0)) && (s.v[1122] != 0.0)) && (!(s.v[1123] != 0.0))) && ((s.v[1130] != 0.0) && (!(s.v[1129] != 0.0)))) && (s.v[1133] != 0.0)) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!(s.v[1103] != 0.0)) && (s.v[1104] != 0.0)) && (s.v[1111] != 0.0)) && (s.v[1122] != 0.0)) && (!(s.v[1123] != 0.0))) && ((s.v[1130] != 0.0) && (!(s.v[1129] != 0.0)))) && (!(s.v[1133] != 0.0))) {
            s.store_div_from_scalar_ad(705, (p.p374 * s.v[29]), A::scale(s.ad_value(703), (3.0 * (s.v[236] + s.v[238]))));
        }

        if ((((((!(s.v[1103] != 0.0)) && (s.v[1104] != 0.0)) && (s.v[1111] != 0.0)) && (s.v[1122] != 0.0)) && (!(s.v[1123] != 0.0))) && (!((s.v[1129] != 0.0) || (s.v[1130] != 0.0)))) {
            s.store_scalar(705, 0.0);
        }

        s.v[1134] = if (0.0 == 1.0) { 1.0 } else { 0.0 };

        s.v[1135] = if (((p.p10 == 1.0) || (p.p10 == 2.0)) || (p.p10 == 5.0)) { 1.0 } else { 0.0 };

        s.v[1136] = if (((p.p10 == 3.0) || (p.p10 == 4.0)) || (p.p10 == 6.0)) { 1.0 } else { 0.0 };

        s.v[1137] = if (s.v[701] == 0.0) { 1.0 } else { 0.0 };

        if (((((((!(s.v[1103] != 0.0)) && (s.v[1104] != 0.0)) && (s.v[1111] != 0.0)) && (!(s.v[1122] != 0.0))) && (s.v[1134] != 0.0)) && (s.v[1135] != 0.0)) && (s.v[1137] != 0.0)) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!(s.v[1103] != 0.0)) && (s.v[1104] != 0.0)) && (s.v[1111] != 0.0)) && (!(s.v[1122] != 0.0))) && (s.v[1134] != 0.0)) && (s.v[1135] != 0.0)) && (!(s.v[1137] != 0.0))) {
            s.store_div_from_scalar_ad(705, (p.p374 * s.v[236]), A::scale(s.ad_value(701), s.v[29]));
        }

        s.v[1139] = if ((s.v[701] == 0.0) || ((s.v[236] + s.v[238]) == 0.0)) { 1.0 } else { 0.0 };

        if (((((((!(s.v[1103] != 0.0)) && (s.v[1104] != 0.0)) && (s.v[1111] != 0.0)) && (!(s.v[1122] != 0.0))) && (s.v[1134] != 0.0)) && ((s.v[1136] != 0.0) && (!(s.v[1135] != 0.0)))) && (s.v[1139] != 0.0)) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!(s.v[1103] != 0.0)) && (s.v[1104] != 0.0)) && (s.v[1111] != 0.0)) && (!(s.v[1122] != 0.0))) && (s.v[1134] != 0.0)) && ((s.v[1136] != 0.0) && (!(s.v[1135] != 0.0)))) && (!(s.v[1139] != 0.0))) {
            s.store_div_from_scalar_ad(705, (p.p374 * s.v[29]), A::scale(s.ad_value(701), (3.0 * (s.v[236] + s.v[238]))));
        }

        if ((((((!(s.v[1103] != 0.0)) && (s.v[1104] != 0.0)) && (s.v[1111] != 0.0)) && (!(s.v[1122] != 0.0))) && (s.v[1134] != 0.0)) && (!((s.v[1135] != 0.0) || (s.v[1136] != 0.0)))) {
            s.store_scalar(705, 0.0);
        }

        s.v[1140] = if (((p.p10 == 1.0) || (p.p10 == 3.0)) || (p.p10 == 7.0)) { 1.0 } else { 0.0 };

        s.v[1141] = if (((p.p10 == 2.0) || (p.p10 == 4.0)) || (p.p10 == 8.0)) { 1.0 } else { 0.0 };

        s.v[1142] = if (s.v[701] == 0.0) { 1.0 } else { 0.0 };

        if (((((((!(s.v[1103] != 0.0)) && (s.v[1104] != 0.0)) && (s.v[1111] != 0.0)) && (!(s.v[1122] != 0.0))) && (!(s.v[1134] != 0.0))) && (s.v[1140] != 0.0)) && (s.v[1142] != 0.0)) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!(s.v[1103] != 0.0)) && (s.v[1104] != 0.0)) && (s.v[1111] != 0.0)) && (!(s.v[1122] != 0.0))) && (!(s.v[1134] != 0.0))) && (s.v[1140] != 0.0)) && (!(s.v[1142] != 0.0))) {
            s.store_div_from_scalar_ad(705, (p.p374 * s.v[236]), A::scale(s.ad_value(701), s.v[29]));
        }

        s.v[1144] = if ((s.v[701] == 0.0) || ((s.v[236] + s.v[238]) == 0.0)) { 1.0 } else { 0.0 };

        if (((((((!(s.v[1103] != 0.0)) && (s.v[1104] != 0.0)) && (s.v[1111] != 0.0)) && (!(s.v[1122] != 0.0))) && (!(s.v[1134] != 0.0))) && ((s.v[1141] != 0.0) && (!(s.v[1140] != 0.0)))) && (s.v[1144] != 0.0)) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!(s.v[1103] != 0.0)) && (s.v[1104] != 0.0)) && (s.v[1111] != 0.0)) && (!(s.v[1122] != 0.0))) && (!(s.v[1134] != 0.0))) && ((s.v[1141] != 0.0) && (!(s.v[1140] != 0.0)))) && (!(s.v[1144] != 0.0))) {
            s.store_div_from_scalar_ad(705, (p.p374 * s.v[29]), A::scale(s.ad_value(701), (3.0 * (s.v[236] + s.v[238]))));
        }

        if ((((((!(s.v[1103] != 0.0)) && (s.v[1104] != 0.0)) && (s.v[1111] != 0.0)) && (!(s.v[1122] != 0.0))) && (!(s.v[1134] != 0.0))) && (!((s.v[1140] != 0.0) || (s.v[1141] != 0.0)))) {
            s.store_scalar(705, 0.0);
        }

        s.v[1145] = if (0.0 == 1.0) { 1.0 } else { 0.0 };

        s.v[1146] = if (1.0 == 1.0) { 1.0 } else { 0.0 };

        s.v[1147] = if (((p.p10 == 1.0) || (p.p10 == 2.0)) || (p.p10 == 5.0)) { 1.0 } else { 0.0 };

        s.v[1148] = if (((p.p10 == 3.0) || (p.p10 == 4.0)) || (p.p10 == 6.0)) { 1.0 } else { 0.0 };

        s.v[1149] = if (s.v[703] == 0.0) { 1.0 } else { 0.0 };

        if (((((((!(s.v[1103] != 0.0)) && (s.v[1104] != 0.0)) && ((s.v[1112] != 0.0) && (!(s.v[1111] != 0.0)))) && (s.v[1145] != 0.0)) && (s.v[1146] != 0.0)) && (s.v[1147] != 0.0)) && (s.v[1149] != 0.0)) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!(s.v[1103] != 0.0)) && (s.v[1104] != 0.0)) && ((s.v[1112] != 0.0) && (!(s.v[1111] != 0.0)))) && (s.v[1145] != 0.0)) && (s.v[1146] != 0.0)) && (s.v[1147] != 0.0)) && (!(s.v[1149] != 0.0))) {
            s.store_div_from_scalar_ad(705, (p.p374 * s.v[236]), A::scale(s.ad_value(703), s.v[29]));
        }

        s.v[1151] = if ((s.v[703] == 0.0) || ((s.v[236] + s.v[238]) == 0.0)) { 1.0 } else { 0.0 };

        if (((((((!(s.v[1103] != 0.0)) && (s.v[1104] != 0.0)) && ((s.v[1112] != 0.0) && (!(s.v[1111] != 0.0)))) && (s.v[1145] != 0.0)) && (s.v[1146] != 0.0)) && ((s.v[1148] != 0.0) && (!(s.v[1147] != 0.0)))) && (s.v[1151] != 0.0)) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!(s.v[1103] != 0.0)) && (s.v[1104] != 0.0)) && ((s.v[1112] != 0.0) && (!(s.v[1111] != 0.0)))) && (s.v[1145] != 0.0)) && (s.v[1146] != 0.0)) && ((s.v[1148] != 0.0) && (!(s.v[1147] != 0.0)))) && (!(s.v[1151] != 0.0))) {
            s.store_div_from_scalar_ad(705, (p.p374 * s.v[29]), A::scale(s.ad_value(703), (3.0 * (s.v[236] + s.v[238]))));
        }

        if ((((((!(s.v[1103] != 0.0)) && (s.v[1104] != 0.0)) && ((s.v[1112] != 0.0) && (!(s.v[1111] != 0.0)))) && (s.v[1145] != 0.0)) && (s.v[1146] != 0.0)) && (!((s.v[1147] != 0.0) || (s.v[1148] != 0.0)))) {
            s.store_scalar(705, 0.0);
        }

        s.v[1152] = if (((p.p10 == 1.0) || (p.p10 == 3.0)) || (p.p10 == 7.0)) { 1.0 } else { 0.0 };

        s.v[1153] = if (((p.p10 == 2.0) || (p.p10 == 4.0)) || (p.p10 == 8.0)) { 1.0 } else { 0.0 };

        s.v[1154] = if (s.v[703] == 0.0) { 1.0 } else { 0.0 };

        if (((((((!(s.v[1103] != 0.0)) && (s.v[1104] != 0.0)) && ((s.v[1112] != 0.0) && (!(s.v[1111] != 0.0)))) && (s.v[1145] != 0.0)) && (!(s.v[1146] != 0.0))) && (s.v[1152] != 0.0)) && (s.v[1154] != 0.0)) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!(s.v[1103] != 0.0)) && (s.v[1104] != 0.0)) && ((s.v[1112] != 0.0) && (!(s.v[1111] != 0.0)))) && (s.v[1145] != 0.0)) && (!(s.v[1146] != 0.0))) && (s.v[1152] != 0.0)) && (!(s.v[1154] != 0.0))) {
            s.store_div_from_scalar_ad(705, (p.p374 * s.v[236]), A::scale(s.ad_value(703), s.v[29]));
        }

        s.v[1156] = if ((s.v[703] == 0.0) || ((s.v[236] + s.v[238]) == 0.0)) { 1.0 } else { 0.0 };

        if (((((((!(s.v[1103] != 0.0)) && (s.v[1104] != 0.0)) && ((s.v[1112] != 0.0) && (!(s.v[1111] != 0.0)))) && (s.v[1145] != 0.0)) && (!(s.v[1146] != 0.0))) && ((s.v[1153] != 0.0) && (!(s.v[1152] != 0.0)))) && (s.v[1156] != 0.0)) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!(s.v[1103] != 0.0)) && (s.v[1104] != 0.0)) && ((s.v[1112] != 0.0) && (!(s.v[1111] != 0.0)))) && (s.v[1145] != 0.0)) && (!(s.v[1146] != 0.0))) && ((s.v[1153] != 0.0) && (!(s.v[1152] != 0.0)))) && (!(s.v[1156] != 0.0))) {
            s.store_div_from_scalar_ad(705, (p.p374 * s.v[29]), A::scale(s.ad_value(703), (3.0 * (s.v[236] + s.v[238]))));
        }

        if ((((((!(s.v[1103] != 0.0)) && (s.v[1104] != 0.0)) && ((s.v[1112] != 0.0) && (!(s.v[1111] != 0.0)))) && (s.v[1145] != 0.0)) && (!(s.v[1146] != 0.0))) && (!((s.v[1152] != 0.0) || (s.v[1153] != 0.0)))) {
            s.store_scalar(705, 0.0);
        }

        s.v[1157] = if (0.0 == 1.0) { 1.0 } else { 0.0 };

        s.v[1158] = if (((p.p10 == 1.0) || (p.p10 == 2.0)) || (p.p10 == 5.0)) { 1.0 } else { 0.0 };

        s.v[1159] = if (((p.p10 == 3.0) || (p.p10 == 4.0)) || (p.p10 == 6.0)) { 1.0 } else { 0.0 };

        s.v[1160] = if (s.v[701] == 0.0) { 1.0 } else { 0.0 };

        if (((((((!(s.v[1103] != 0.0)) && (s.v[1104] != 0.0)) && ((s.v[1112] != 0.0) && (!(s.v[1111] != 0.0)))) && (!(s.v[1145] != 0.0))) && (s.v[1157] != 0.0)) && (s.v[1158] != 0.0)) && (s.v[1160] != 0.0)) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!(s.v[1103] != 0.0)) && (s.v[1104] != 0.0)) && ((s.v[1112] != 0.0) && (!(s.v[1111] != 0.0)))) && (!(s.v[1145] != 0.0))) && (s.v[1157] != 0.0)) && (s.v[1158] != 0.0)) && (!(s.v[1160] != 0.0))) {
            s.store_div_from_scalar_ad(705, (p.p374 * s.v[236]), A::scale(s.ad_value(701), s.v[29]));
        }

        s.v[1162] = if ((s.v[701] == 0.0) || (s.v[236] == 0.0)) { 1.0 } else { 0.0 };

        if (((((((!(s.v[1103] != 0.0)) && (s.v[1104] != 0.0)) && ((s.v[1112] != 0.0) && (!(s.v[1111] != 0.0)))) && (!(s.v[1145] != 0.0))) && (s.v[1157] != 0.0)) && ((s.v[1159] != 0.0) && (!(s.v[1158] != 0.0)))) && (s.v[1162] != 0.0)) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!(s.v[1103] != 0.0)) && (s.v[1104] != 0.0)) && ((s.v[1112] != 0.0) && (!(s.v[1111] != 0.0)))) && (!(s.v[1145] != 0.0))) && (s.v[1157] != 0.0)) && ((s.v[1159] != 0.0) && (!(s.v[1158] != 0.0)))) && (!(s.v[1162] != 0.0))) {
            s.store_div_from_scalar_ad(705, (p.p374 * s.v[29]), A::scale(s.ad_value(701), (6.0 * s.v[236])));
        }

        if ((((((!(s.v[1103] != 0.0)) && (s.v[1104] != 0.0)) && ((s.v[1112] != 0.0) && (!(s.v[1111] != 0.0)))) && (!(s.v[1145] != 0.0))) && (s.v[1157] != 0.0)) && (!((s.v[1158] != 0.0) || (s.v[1159] != 0.0)))) {
            s.store_scalar(705, 0.0);
        }

        s.v[1163] = if (((p.p10 == 1.0) || (p.p10 == 3.0)) || (p.p10 == 7.0)) { 1.0 } else { 0.0 };

        s.v[1164] = if (((p.p10 == 2.0) || (p.p10 == 4.0)) || (p.p10 == 8.0)) { 1.0 } else { 0.0 };

        s.v[1165] = if (s.v[701] == 0.0) { 1.0 } else { 0.0 };

        if (((((((!(s.v[1103] != 0.0)) && (s.v[1104] != 0.0)) && ((s.v[1112] != 0.0) && (!(s.v[1111] != 0.0)))) && (!(s.v[1145] != 0.0))) && (!(s.v[1157] != 0.0))) && (s.v[1163] != 0.0)) && (s.v[1165] != 0.0)) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!(s.v[1103] != 0.0)) && (s.v[1104] != 0.0)) && ((s.v[1112] != 0.0) && (!(s.v[1111] != 0.0)))) && (!(s.v[1145] != 0.0))) && (!(s.v[1157] != 0.0))) && (s.v[1163] != 0.0)) && (!(s.v[1165] != 0.0))) {
            s.store_div_from_scalar_ad(705, (p.p374 * s.v[236]), A::scale(s.ad_value(701), s.v[29]));
        }

        s.v[1167] = if ((s.v[701] == 0.0) || (s.v[236] == 0.0)) { 1.0 } else { 0.0 };

        if (((((((!(s.v[1103] != 0.0)) && (s.v[1104] != 0.0)) && ((s.v[1112] != 0.0) && (!(s.v[1111] != 0.0)))) && (!(s.v[1145] != 0.0))) && (!(s.v[1157] != 0.0))) && ((s.v[1164] != 0.0) && (!(s.v[1163] != 0.0)))) && (s.v[1167] != 0.0)) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!(s.v[1103] != 0.0)) && (s.v[1104] != 0.0)) && ((s.v[1112] != 0.0) && (!(s.v[1111] != 0.0)))) && (!(s.v[1145] != 0.0))) && (!(s.v[1157] != 0.0))) && ((s.v[1164] != 0.0) && (!(s.v[1163] != 0.0)))) && (!(s.v[1167] != 0.0))) {
            s.store_div_from_scalar_ad(705, (p.p374 * s.v[29]), A::scale(s.ad_value(701), (6.0 * s.v[236])));
        }

        if ((((((!(s.v[1103] != 0.0)) && (s.v[1104] != 0.0)) && ((s.v[1112] != 0.0) && (!(s.v[1111] != 0.0)))) && (!(s.v[1145] != 0.0))) && (!(s.v[1157] != 0.0))) && (!((s.v[1163] != 0.0) || (s.v[1164] != 0.0)))) {
            s.store_scalar(705, 0.0);
        }

        s.v[1168] = if (0.0 == 1.0) { 1.0 } else { 0.0 };

        s.v[1169] = if (1.0 == 1.0) { 1.0 } else { 0.0 };

        s.v[1170] = if (((p.p10 == 1.0) || (p.p10 == 2.0)) || (p.p10 == 5.0)) { 1.0 } else { 0.0 };

        s.v[1171] = if (((p.p10 == 3.0) || (p.p10 == 4.0)) || (p.p10 == 6.0)) { 1.0 } else { 0.0 };

        s.v[1172] = if (s.v[703] == 0.0) { 1.0 } else { 0.0 };

        if (((((((!(s.v[1103] != 0.0)) && (s.v[1104] != 0.0)) && ((s.v[1113] != 0.0) && (!((s.v[1111] != 0.0) || (s.v[1112] != 0.0))))) && (s.v[1168] != 0.0)) && (s.v[1169] != 0.0)) && (s.v[1170] != 0.0)) && (s.v[1172] != 0.0)) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!(s.v[1103] != 0.0)) && (s.v[1104] != 0.0)) && ((s.v[1113] != 0.0) && (!((s.v[1111] != 0.0) || (s.v[1112] != 0.0))))) && (s.v[1168] != 0.0)) && (s.v[1169] != 0.0)) && (s.v[1170] != 0.0)) && (!(s.v[1172] != 0.0))) {
            s.store_div_from_scalar_ad(705, (p.p374 * s.v[236]), A::scale(s.ad_value(703), s.v[29]));
        }

        s.v[1174] = if ((s.v[703] == 0.0) || (s.v[236] == 0.0)) { 1.0 } else { 0.0 };

        if (((((((!(s.v[1103] != 0.0)) && (s.v[1104] != 0.0)) && ((s.v[1113] != 0.0) && (!((s.v[1111] != 0.0) || (s.v[1112] != 0.0))))) && (s.v[1168] != 0.0)) && (s.v[1169] != 0.0)) && ((s.v[1171] != 0.0) && (!(s.v[1170] != 0.0)))) && (s.v[1174] != 0.0)) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!(s.v[1103] != 0.0)) && (s.v[1104] != 0.0)) && ((s.v[1113] != 0.0) && (!((s.v[1111] != 0.0) || (s.v[1112] != 0.0))))) && (s.v[1168] != 0.0)) && (s.v[1169] != 0.0)) && ((s.v[1171] != 0.0) && (!(s.v[1170] != 0.0)))) && (!(s.v[1174] != 0.0))) {
            s.store_div_from_scalar_ad(705, (p.p374 * s.v[29]), A::scale(s.ad_value(703), (6.0 * s.v[236])));
        }

        if ((((((!(s.v[1103] != 0.0)) && (s.v[1104] != 0.0)) && ((s.v[1113] != 0.0) && (!((s.v[1111] != 0.0) || (s.v[1112] != 0.0))))) && (s.v[1168] != 0.0)) && (s.v[1169] != 0.0)) && (!((s.v[1170] != 0.0) || (s.v[1171] != 0.0)))) {
            s.store_scalar(705, 0.0);
        }

        s.v[1175] = if (((p.p10 == 1.0) || (p.p10 == 3.0)) || (p.p10 == 7.0)) { 1.0 } else { 0.0 };

        s.v[1176] = if (((p.p10 == 2.0) || (p.p10 == 4.0)) || (p.p10 == 8.0)) { 1.0 } else { 0.0 };

        s.v[1177] = if (s.v[703] == 0.0) { 1.0 } else { 0.0 };

        if (((((((!(s.v[1103] != 0.0)) && (s.v[1104] != 0.0)) && ((s.v[1113] != 0.0) && (!((s.v[1111] != 0.0) || (s.v[1112] != 0.0))))) && (s.v[1168] != 0.0)) && (!(s.v[1169] != 0.0))) && (s.v[1175] != 0.0)) && (s.v[1177] != 0.0)) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!(s.v[1103] != 0.0)) && (s.v[1104] != 0.0)) && ((s.v[1113] != 0.0) && (!((s.v[1111] != 0.0) || (s.v[1112] != 0.0))))) && (s.v[1168] != 0.0)) && (!(s.v[1169] != 0.0))) && (s.v[1175] != 0.0)) && (!(s.v[1177] != 0.0))) {
            s.store_div_from_scalar_ad(705, (p.p374 * s.v[236]), A::scale(s.ad_value(703), s.v[29]));
        }

        s.v[1179] = if ((s.v[703] == 0.0) || (s.v[236] == 0.0)) { 1.0 } else { 0.0 };

        if (((((((!(s.v[1103] != 0.0)) && (s.v[1104] != 0.0)) && ((s.v[1113] != 0.0) && (!((s.v[1111] != 0.0) || (s.v[1112] != 0.0))))) && (s.v[1168] != 0.0)) && (!(s.v[1169] != 0.0))) && ((s.v[1176] != 0.0) && (!(s.v[1175] != 0.0)))) && (s.v[1179] != 0.0)) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!(s.v[1103] != 0.0)) && (s.v[1104] != 0.0)) && ((s.v[1113] != 0.0) && (!((s.v[1111] != 0.0) || (s.v[1112] != 0.0))))) && (s.v[1168] != 0.0)) && (!(s.v[1169] != 0.0))) && ((s.v[1176] != 0.0) && (!(s.v[1175] != 0.0)))) && (!(s.v[1179] != 0.0))) {
            s.store_div_from_scalar_ad(705, (p.p374 * s.v[29]), A::scale(s.ad_value(703), (6.0 * s.v[236])));
        }

        if ((((((!(s.v[1103] != 0.0)) && (s.v[1104] != 0.0)) && ((s.v[1113] != 0.0) && (!((s.v[1111] != 0.0) || (s.v[1112] != 0.0))))) && (s.v[1168] != 0.0)) && (!(s.v[1169] != 0.0))) && (!((s.v[1175] != 0.0) || (s.v[1176] != 0.0)))) {
            s.store_scalar(705, 0.0);
        }

        s.v[1180] = if (0.0 == 1.0) { 1.0 } else { 0.0 };

        s.v[1181] = if (((p.p10 == 1.0) || (p.p10 == 2.0)) || (p.p10 == 5.0)) { 1.0 } else { 0.0 };

        s.v[1182] = if (((p.p10 == 3.0) || (p.p10 == 4.0)) || (p.p10 == 6.0)) { 1.0 } else { 0.0 };

        s.v[1183] = if (s.v[701] == 0.0) { 1.0 } else { 0.0 };

        if (((((((!(s.v[1103] != 0.0)) && (s.v[1104] != 0.0)) && ((s.v[1113] != 0.0) && (!((s.v[1111] != 0.0) || (s.v[1112] != 0.0))))) && (!(s.v[1168] != 0.0))) && (s.v[1180] != 0.0)) && (s.v[1181] != 0.0)) && (s.v[1183] != 0.0)) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!(s.v[1103] != 0.0)) && (s.v[1104] != 0.0)) && ((s.v[1113] != 0.0) && (!((s.v[1111] != 0.0) || (s.v[1112] != 0.0))))) && (!(s.v[1168] != 0.0))) && (s.v[1180] != 0.0)) && (s.v[1181] != 0.0)) && (!(s.v[1183] != 0.0))) {
            s.store_div_from_scalar_ad(705, (p.p374 * s.v[236]), A::scale(s.ad_value(701), s.v[29]));
        }

        s.v[1185] = if ((s.v[701] == 0.0) || ((s.v[236] + s.v[238]) == 0.0)) { 1.0 } else { 0.0 };

        if (((((((!(s.v[1103] != 0.0)) && (s.v[1104] != 0.0)) && ((s.v[1113] != 0.0) && (!((s.v[1111] != 0.0) || (s.v[1112] != 0.0))))) && (!(s.v[1168] != 0.0))) && (s.v[1180] != 0.0)) && ((s.v[1182] != 0.0) && (!(s.v[1181] != 0.0)))) && (s.v[1185] != 0.0)) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!(s.v[1103] != 0.0)) && (s.v[1104] != 0.0)) && ((s.v[1113] != 0.0) && (!((s.v[1111] != 0.0) || (s.v[1112] != 0.0))))) && (!(s.v[1168] != 0.0))) && (s.v[1180] != 0.0)) && ((s.v[1182] != 0.0) && (!(s.v[1181] != 0.0)))) && (!(s.v[1185] != 0.0))) {
            s.store_div_from_scalar_ad(705, (p.p374 * s.v[29]), A::scale(s.ad_value(701), (3.0 * (s.v[236] + s.v[238]))));
        }

        if ((((((!(s.v[1103] != 0.0)) && (s.v[1104] != 0.0)) && ((s.v[1113] != 0.0) && (!((s.v[1111] != 0.0) || (s.v[1112] != 0.0))))) && (!(s.v[1168] != 0.0))) && (s.v[1180] != 0.0)) && (!((s.v[1181] != 0.0) || (s.v[1182] != 0.0)))) {
            s.store_scalar(705, 0.0);
        }

        s.v[1186] = if (((p.p10 == 1.0) || (p.p10 == 3.0)) || (p.p10 == 7.0)) { 1.0 } else { 0.0 };

        s.v[1187] = if (((p.p10 == 2.0) || (p.p10 == 4.0)) || (p.p10 == 8.0)) { 1.0 } else { 0.0 };

        s.v[1188] = if (s.v[701] == 0.0) { 1.0 } else { 0.0 };

        if (((((((!(s.v[1103] != 0.0)) && (s.v[1104] != 0.0)) && ((s.v[1113] != 0.0) && (!((s.v[1111] != 0.0) || (s.v[1112] != 0.0))))) && (!(s.v[1168] != 0.0))) && (!(s.v[1180] != 0.0))) && (s.v[1186] != 0.0)) && (s.v[1188] != 0.0)) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!(s.v[1103] != 0.0)) && (s.v[1104] != 0.0)) && ((s.v[1113] != 0.0) && (!((s.v[1111] != 0.0) || (s.v[1112] != 0.0))))) && (!(s.v[1168] != 0.0))) && (!(s.v[1180] != 0.0))) && (s.v[1186] != 0.0)) && (!(s.v[1188] != 0.0))) {
            s.store_div_from_scalar_ad(705, (p.p374 * s.v[236]), A::scale(s.ad_value(701), s.v[29]));
        }

        s.v[1190] = if ((s.v[701] == 0.0) || ((s.v[236] + s.v[238]) == 0.0)) { 1.0 } else { 0.0 };

        if (((((((!(s.v[1103] != 0.0)) && (s.v[1104] != 0.0)) && ((s.v[1113] != 0.0) && (!((s.v[1111] != 0.0) || (s.v[1112] != 0.0))))) && (!(s.v[1168] != 0.0))) && (!(s.v[1180] != 0.0))) && ((s.v[1187] != 0.0) && (!(s.v[1186] != 0.0)))) && (s.v[1190] != 0.0)) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!(s.v[1103] != 0.0)) && (s.v[1104] != 0.0)) && ((s.v[1113] != 0.0) && (!((s.v[1111] != 0.0) || (s.v[1112] != 0.0))))) && (!(s.v[1168] != 0.0))) && (!(s.v[1180] != 0.0))) && ((s.v[1187] != 0.0) && (!(s.v[1186] != 0.0)))) && (!(s.v[1190] != 0.0))) {
            s.store_div_from_scalar_ad(705, (p.p374 * s.v[29]), A::scale(s.ad_value(701), (3.0 * (s.v[236] + s.v[238]))));
        }

        if ((((((!(s.v[1103] != 0.0)) && (s.v[1104] != 0.0)) && ((s.v[1113] != 0.0) && (!((s.v[1111] != 0.0) || (s.v[1112] != 0.0))))) && (!(s.v[1168] != 0.0))) && (!(s.v[1180] != 0.0))) && (!((s.v[1186] != 0.0) || (s.v[1187] != 0.0)))) {
            s.store_scalar(705, 0.0);
        }

        s.v[1191] = if (0.0 == 1.0) { 1.0 } else { 0.0 };

        s.v[1192] = if (1.0 == 1.0) { 1.0 } else { 0.0 };

        s.v[1193] = if (((p.p10 == 1.0) || (p.p10 == 2.0)) || (p.p10 == 5.0)) { 1.0 } else { 0.0 };

        s.v[1194] = if (((p.p10 == 3.0) || (p.p10 == 4.0)) || (p.p10 == 6.0)) { 1.0 } else { 0.0 };

        s.v[1195] = if (s.v[703] == 0.0) { 1.0 } else { 0.0 };

        if (((((((!(s.v[1103] != 0.0)) && (s.v[1104] != 0.0)) && ((s.v[1114] != 0.0) && (!(((s.v[1111] != 0.0) || (s.v[1112] != 0.0)) || (s.v[1113] != 0.0))))) && (s.v[1191] != 0.0)) && (s.v[1192] != 0.0)) && (s.v[1193] != 0.0)) && (s.v[1195] != 0.0)) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!(s.v[1103] != 0.0)) && (s.v[1104] != 0.0)) && ((s.v[1114] != 0.0) && (!(((s.v[1111] != 0.0) || (s.v[1112] != 0.0)) || (s.v[1113] != 0.0))))) && (s.v[1191] != 0.0)) && (s.v[1192] != 0.0)) && (s.v[1193] != 0.0)) && (!(s.v[1195] != 0.0))) {
            s.store_div_from_scalar_ad(705, (p.p374 * s.v[236]), A::scale(s.ad_value(703), s.v[29]));
        }

        s.v[1197] = if ((s.v[703] == 0.0) || (s.v[236] == 0.0)) { 1.0 } else { 0.0 };

        if (((((((!(s.v[1103] != 0.0)) && (s.v[1104] != 0.0)) && ((s.v[1114] != 0.0) && (!(((s.v[1111] != 0.0) || (s.v[1112] != 0.0)) || (s.v[1113] != 0.0))))) && (s.v[1191] != 0.0)) && (s.v[1192] != 0.0)) && ((s.v[1194] != 0.0) && (!(s.v[1193] != 0.0)))) && (s.v[1197] != 0.0)) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!(s.v[1103] != 0.0)) && (s.v[1104] != 0.0)) && ((s.v[1114] != 0.0) && (!(((s.v[1111] != 0.0) || (s.v[1112] != 0.0)) || (s.v[1113] != 0.0))))) && (s.v[1191] != 0.0)) && (s.v[1192] != 0.0)) && ((s.v[1194] != 0.0) && (!(s.v[1193] != 0.0)))) && (!(s.v[1197] != 0.0))) {
            s.store_div_from_scalar_ad(705, (p.p374 * s.v[29]), A::scale(s.ad_value(703), (6.0 * s.v[236])));
        }

        if ((((((!(s.v[1103] != 0.0)) && (s.v[1104] != 0.0)) && ((s.v[1114] != 0.0) && (!(((s.v[1111] != 0.0) || (s.v[1112] != 0.0)) || (s.v[1113] != 0.0))))) && (s.v[1191] != 0.0)) && (s.v[1192] != 0.0)) && (!((s.v[1193] != 0.0) || (s.v[1194] != 0.0)))) {
            s.store_scalar(705, 0.0);
        }

        s.v[1198] = if (((p.p10 == 1.0) || (p.p10 == 3.0)) || (p.p10 == 7.0)) { 1.0 } else { 0.0 };

        s.v[1199] = if (((p.p10 == 2.0) || (p.p10 == 4.0)) || (p.p10 == 8.0)) { 1.0 } else { 0.0 };

        s.v[1200] = if (s.v[703] == 0.0) { 1.0 } else { 0.0 };

        if (((((((!(s.v[1103] != 0.0)) && (s.v[1104] != 0.0)) && ((s.v[1114] != 0.0) && (!(((s.v[1111] != 0.0) || (s.v[1112] != 0.0)) || (s.v[1113] != 0.0))))) && (s.v[1191] != 0.0)) && (!(s.v[1192] != 0.0))) && (s.v[1198] != 0.0)) && (s.v[1200] != 0.0)) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!(s.v[1103] != 0.0)) && (s.v[1104] != 0.0)) && ((s.v[1114] != 0.0) && (!(((s.v[1111] != 0.0) || (s.v[1112] != 0.0)) || (s.v[1113] != 0.0))))) && (s.v[1191] != 0.0)) && (!(s.v[1192] != 0.0))) && (s.v[1198] != 0.0)) && (!(s.v[1200] != 0.0))) {
            s.store_div_from_scalar_ad(705, (p.p374 * s.v[236]), A::scale(s.ad_value(703), s.v[29]));
        }

        s.v[1202] = if ((s.v[703] == 0.0) || (s.v[236] == 0.0)) { 1.0 } else { 0.0 };

        if (((((((!(s.v[1103] != 0.0)) && (s.v[1104] != 0.0)) && ((s.v[1114] != 0.0) && (!(((s.v[1111] != 0.0) || (s.v[1112] != 0.0)) || (s.v[1113] != 0.0))))) && (s.v[1191] != 0.0)) && (!(s.v[1192] != 0.0))) && ((s.v[1199] != 0.0) && (!(s.v[1198] != 0.0)))) && (s.v[1202] != 0.0)) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!(s.v[1103] != 0.0)) && (s.v[1104] != 0.0)) && ((s.v[1114] != 0.0) && (!(((s.v[1111] != 0.0) || (s.v[1112] != 0.0)) || (s.v[1113] != 0.0))))) && (s.v[1191] != 0.0)) && (!(s.v[1192] != 0.0))) && ((s.v[1199] != 0.0) && (!(s.v[1198] != 0.0)))) && (!(s.v[1202] != 0.0))) {
            s.store_div_from_scalar_ad(705, (p.p374 * s.v[29]), A::scale(s.ad_value(703), (6.0 * s.v[236])));
        }

        if ((((((!(s.v[1103] != 0.0)) && (s.v[1104] != 0.0)) && ((s.v[1114] != 0.0) && (!(((s.v[1111] != 0.0) || (s.v[1112] != 0.0)) || (s.v[1113] != 0.0))))) && (s.v[1191] != 0.0)) && (!(s.v[1192] != 0.0))) && (!((s.v[1198] != 0.0) || (s.v[1199] != 0.0)))) {
            s.store_scalar(705, 0.0);
        }

        s.v[1203] = if (0.0 == 1.0) { 1.0 } else { 0.0 };

        s.v[1204] = if (((p.p10 == 1.0) || (p.p10 == 2.0)) || (p.p10 == 5.0)) { 1.0 } else { 0.0 };

        s.v[1205] = if (((p.p10 == 3.0) || (p.p10 == 4.0)) || (p.p10 == 6.0)) { 1.0 } else { 0.0 };

        s.v[1206] = if (s.v[701] == 0.0) { 1.0 } else { 0.0 };

        if (((((((!(s.v[1103] != 0.0)) && (s.v[1104] != 0.0)) && ((s.v[1114] != 0.0) && (!(((s.v[1111] != 0.0) || (s.v[1112] != 0.0)) || (s.v[1113] != 0.0))))) && (!(s.v[1191] != 0.0))) && (s.v[1203] != 0.0)) && (s.v[1204] != 0.0)) && (s.v[1206] != 0.0)) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!(s.v[1103] != 0.0)) && (s.v[1104] != 0.0)) && ((s.v[1114] != 0.0) && (!(((s.v[1111] != 0.0) || (s.v[1112] != 0.0)) || (s.v[1113] != 0.0))))) && (!(s.v[1191] != 0.0))) && (s.v[1203] != 0.0)) && (s.v[1204] != 0.0)) && (!(s.v[1206] != 0.0))) {
            s.store_div_from_scalar_ad(705, (p.p374 * s.v[236]), A::scale(s.ad_value(701), s.v[29]));
        }

        s.v[1208] = if ((s.v[701] == 0.0) || (s.v[236] == 0.0)) { 1.0 } else { 0.0 };

        if (((((((!(s.v[1103] != 0.0)) && (s.v[1104] != 0.0)) && ((s.v[1114] != 0.0) && (!(((s.v[1111] != 0.0) || (s.v[1112] != 0.0)) || (s.v[1113] != 0.0))))) && (!(s.v[1191] != 0.0))) && (s.v[1203] != 0.0)) && ((s.v[1205] != 0.0) && (!(s.v[1204] != 0.0)))) && (s.v[1208] != 0.0)) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!(s.v[1103] != 0.0)) && (s.v[1104] != 0.0)) && ((s.v[1114] != 0.0) && (!(((s.v[1111] != 0.0) || (s.v[1112] != 0.0)) || (s.v[1113] != 0.0))))) && (!(s.v[1191] != 0.0))) && (s.v[1203] != 0.0)) && ((s.v[1205] != 0.0) && (!(s.v[1204] != 0.0)))) && (!(s.v[1208] != 0.0))) {
            s.store_div_from_scalar_ad(705, (p.p374 * s.v[29]), A::scale(s.ad_value(701), (6.0 * s.v[236])));
        }

        if ((((((!(s.v[1103] != 0.0)) && (s.v[1104] != 0.0)) && ((s.v[1114] != 0.0) && (!(((s.v[1111] != 0.0) || (s.v[1112] != 0.0)) || (s.v[1113] != 0.0))))) && (!(s.v[1191] != 0.0))) && (s.v[1203] != 0.0)) && (!((s.v[1204] != 0.0) || (s.v[1205] != 0.0)))) {
            s.store_scalar(705, 0.0);
        }

        s.v[1209] = if (((p.p10 == 1.0) || (p.p10 == 3.0)) || (p.p10 == 7.0)) { 1.0 } else { 0.0 };

        s.v[1210] = if (((p.p10 == 2.0) || (p.p10 == 4.0)) || (p.p10 == 8.0)) { 1.0 } else { 0.0 };

        s.v[1211] = if (s.v[701] == 0.0) { 1.0 } else { 0.0 };

        if (((((((!(s.v[1103] != 0.0)) && (s.v[1104] != 0.0)) && ((s.v[1114] != 0.0) && (!(((s.v[1111] != 0.0) || (s.v[1112] != 0.0)) || (s.v[1113] != 0.0))))) && (!(s.v[1191] != 0.0))) && (!(s.v[1203] != 0.0))) && (s.v[1209] != 0.0)) && (s.v[1211] != 0.0)) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!(s.v[1103] != 0.0)) && (s.v[1104] != 0.0)) && ((s.v[1114] != 0.0) && (!(((s.v[1111] != 0.0) || (s.v[1112] != 0.0)) || (s.v[1113] != 0.0))))) && (!(s.v[1191] != 0.0))) && (!(s.v[1203] != 0.0))) && (s.v[1209] != 0.0)) && (!(s.v[1211] != 0.0))) {
            s.store_div_from_scalar_ad(705, (p.p374 * s.v[236]), A::scale(s.ad_value(701), s.v[29]));
        }

        s.v[1213] = if ((s.v[701] == 0.0) || (s.v[236] == 0.0)) { 1.0 } else { 0.0 };

        if (((((((!(s.v[1103] != 0.0)) && (s.v[1104] != 0.0)) && ((s.v[1114] != 0.0) && (!(((s.v[1111] != 0.0) || (s.v[1112] != 0.0)) || (s.v[1113] != 0.0))))) && (!(s.v[1191] != 0.0))) && (!(s.v[1203] != 0.0))) && ((s.v[1210] != 0.0) && (!(s.v[1209] != 0.0)))) && (s.v[1213] != 0.0)) {
            s.store_scalar(705, 0.0);
        }

    }

    pub(super) fn stamp_reactive_block_5(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if (((((((!(s.v[1103] != 0.0)) && (s.v[1104] != 0.0)) && ((s.v[1114] != 0.0) && (!(((s.v[1111] != 0.0) || (s.v[1112] != 0.0)) || (s.v[1113] != 0.0))))) && (!(s.v[1191] != 0.0))) && (!(s.v[1203] != 0.0))) && ((s.v[1210] != 0.0) && (!(s.v[1209] != 0.0)))) && (!(s.v[1213] != 0.0))) {
            s.store_div_from_scalar_ad(705, (p.p374 * s.v[29]), A::scale(s.ad_value(701), (6.0 * s.v[236])));
        }

        if ((((((!(s.v[1103] != 0.0)) && (s.v[1104] != 0.0)) && ((s.v[1114] != 0.0) && (!(((s.v[1111] != 0.0) || (s.v[1112] != 0.0)) || (s.v[1113] != 0.0))))) && (!(s.v[1191] != 0.0))) && (!(s.v[1203] != 0.0))) && (!((s.v[1209] != 0.0) || (s.v[1210] != 0.0)))) {
            s.store_scalar(705, 0.0);
        }

        s.v[1214] = if (0.0 == 1.0) { 1.0 } else { 0.0 };

        s.v[1215] = if (1.0 == 1.0) { 1.0 } else { 0.0 };

        s.v[1216] = if (((p.p10 == 1.0) || (p.p10 == 2.0)) || (p.p10 == 5.0)) { 1.0 } else { 0.0 };

        s.v[1217] = if (((p.p10 == 3.0) || (p.p10 == 4.0)) || (p.p10 == 6.0)) { 1.0 } else { 0.0 };

        s.v[1218] = if (s.v[703] == 0.0) { 1.0 } else { 0.0 };

        if (((((((!(s.v[1103] != 0.0)) && (s.v[1104] != 0.0)) && ((s.v[1115] != 0.0) && (!((((s.v[1111] != 0.0) || (s.v[1112] != 0.0)) || (s.v[1113] != 0.0)) || (s.v[1114] != 0.0))))) && (s.v[1214] != 0.0)) && (s.v[1215] != 0.0)) && (s.v[1216] != 0.0)) && (s.v[1218] != 0.0)) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!(s.v[1103] != 0.0)) && (s.v[1104] != 0.0)) && ((s.v[1115] != 0.0) && (!((((s.v[1111] != 0.0) || (s.v[1112] != 0.0)) || (s.v[1113] != 0.0)) || (s.v[1114] != 0.0))))) && (s.v[1214] != 0.0)) && (s.v[1215] != 0.0)) && (s.v[1216] != 0.0)) && (!(s.v[1218] != 0.0))) {
            s.store_div_from_scalar_ad(705, (p.p374 * s.v[236]), A::scale(s.ad_value(703), s.v[29]));
        }

        s.v[1220] = if ((s.v[703] == 0.0) || ((s.v[236] + s.v[238]) == 0.0)) { 1.0 } else { 0.0 };

        if (((((((!(s.v[1103] != 0.0)) && (s.v[1104] != 0.0)) && ((s.v[1115] != 0.0) && (!((((s.v[1111] != 0.0) || (s.v[1112] != 0.0)) || (s.v[1113] != 0.0)) || (s.v[1114] != 0.0))))) && (s.v[1214] != 0.0)) && (s.v[1215] != 0.0)) && ((s.v[1217] != 0.0) && (!(s.v[1216] != 0.0)))) && (s.v[1220] != 0.0)) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!(s.v[1103] != 0.0)) && (s.v[1104] != 0.0)) && ((s.v[1115] != 0.0) && (!((((s.v[1111] != 0.0) || (s.v[1112] != 0.0)) || (s.v[1113] != 0.0)) || (s.v[1114] != 0.0))))) && (s.v[1214] != 0.0)) && (s.v[1215] != 0.0)) && ((s.v[1217] != 0.0) && (!(s.v[1216] != 0.0)))) && (!(s.v[1220] != 0.0))) {
            s.store_div_from_scalar_ad(705, (p.p374 * s.v[29]), A::scale(s.ad_value(703), (3.0 * (s.v[236] + s.v[238]))));
        }

        if ((((((!(s.v[1103] != 0.0)) && (s.v[1104] != 0.0)) && ((s.v[1115] != 0.0) && (!((((s.v[1111] != 0.0) || (s.v[1112] != 0.0)) || (s.v[1113] != 0.0)) || (s.v[1114] != 0.0))))) && (s.v[1214] != 0.0)) && (s.v[1215] != 0.0)) && (!((s.v[1216] != 0.0) || (s.v[1217] != 0.0)))) {
            s.store_scalar(705, 0.0);
        }

        s.v[1221] = if (((p.p10 == 1.0) || (p.p10 == 3.0)) || (p.p10 == 7.0)) { 1.0 } else { 0.0 };

        s.v[1222] = if (((p.p10 == 2.0) || (p.p10 == 4.0)) || (p.p10 == 8.0)) { 1.0 } else { 0.0 };

        s.v[1223] = if (s.v[703] == 0.0) { 1.0 } else { 0.0 };

        if (((((((!(s.v[1103] != 0.0)) && (s.v[1104] != 0.0)) && ((s.v[1115] != 0.0) && (!((((s.v[1111] != 0.0) || (s.v[1112] != 0.0)) || (s.v[1113] != 0.0)) || (s.v[1114] != 0.0))))) && (s.v[1214] != 0.0)) && (!(s.v[1215] != 0.0))) && (s.v[1221] != 0.0)) && (s.v[1223] != 0.0)) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!(s.v[1103] != 0.0)) && (s.v[1104] != 0.0)) && ((s.v[1115] != 0.0) && (!((((s.v[1111] != 0.0) || (s.v[1112] != 0.0)) || (s.v[1113] != 0.0)) || (s.v[1114] != 0.0))))) && (s.v[1214] != 0.0)) && (!(s.v[1215] != 0.0))) && (s.v[1221] != 0.0)) && (!(s.v[1223] != 0.0))) {
            s.store_div_from_scalar_ad(705, (p.p374 * s.v[236]), A::scale(s.ad_value(703), s.v[29]));
        }

        s.v[1225] = if ((s.v[703] == 0.0) || ((s.v[236] + s.v[238]) == 0.0)) { 1.0 } else { 0.0 };

        if (((((((!(s.v[1103] != 0.0)) && (s.v[1104] != 0.0)) && ((s.v[1115] != 0.0) && (!((((s.v[1111] != 0.0) || (s.v[1112] != 0.0)) || (s.v[1113] != 0.0)) || (s.v[1114] != 0.0))))) && (s.v[1214] != 0.0)) && (!(s.v[1215] != 0.0))) && ((s.v[1222] != 0.0) && (!(s.v[1221] != 0.0)))) && (s.v[1225] != 0.0)) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!(s.v[1103] != 0.0)) && (s.v[1104] != 0.0)) && ((s.v[1115] != 0.0) && (!((((s.v[1111] != 0.0) || (s.v[1112] != 0.0)) || (s.v[1113] != 0.0)) || (s.v[1114] != 0.0))))) && (s.v[1214] != 0.0)) && (!(s.v[1215] != 0.0))) && ((s.v[1222] != 0.0) && (!(s.v[1221] != 0.0)))) && (!(s.v[1225] != 0.0))) {
            s.store_div_from_scalar_ad(705, (p.p374 * s.v[29]), A::scale(s.ad_value(703), (3.0 * (s.v[236] + s.v[238]))));
        }

        if ((((((!(s.v[1103] != 0.0)) && (s.v[1104] != 0.0)) && ((s.v[1115] != 0.0) && (!((((s.v[1111] != 0.0) || (s.v[1112] != 0.0)) || (s.v[1113] != 0.0)) || (s.v[1114] != 0.0))))) && (s.v[1214] != 0.0)) && (!(s.v[1215] != 0.0))) && (!((s.v[1221] != 0.0) || (s.v[1222] != 0.0)))) {
            s.store_scalar(705, 0.0);
        }

        if ((((!(s.v[1103] != 0.0)) && (s.v[1104] != 0.0)) && ((s.v[1115] != 0.0) && (!((((s.v[1111] != 0.0) || (s.v[1112] != 0.0)) || (s.v[1113] != 0.0)) || (s.v[1114] != 0.0))))) && (!(s.v[1214] != 0.0))) {
            s.store_scalar(705, ((p.p374 * s.v[237]) / s.v[29]));
        }

        s.v[1226] = if (0.0 == 1.0) { 1.0 } else { 0.0 };

        s.v[1227] = if (1.0 == 1.0) { 1.0 } else { 0.0 };

        s.v[1228] = if (((p.p10 == 1.0) || (p.p10 == 2.0)) || (p.p10 == 5.0)) { 1.0 } else { 0.0 };

        s.v[1229] = if (((p.p10 == 3.0) || (p.p10 == 4.0)) || (p.p10 == 6.0)) { 1.0 } else { 0.0 };

        s.v[1230] = if (s.v[703] == 0.0) { 1.0 } else { 0.0 };

        if (((((((!(s.v[1103] != 0.0)) && (s.v[1104] != 0.0)) && ((s.v[1116] != 0.0) && (!(((((s.v[1111] != 0.0) || (s.v[1112] != 0.0)) || (s.v[1113] != 0.0)) || (s.v[1114] != 0.0)) || (s.v[1115] != 0.0))))) && (s.v[1226] != 0.0)) && (s.v[1227] != 0.0)) && (s.v[1228] != 0.0)) && (s.v[1230] != 0.0)) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!(s.v[1103] != 0.0)) && (s.v[1104] != 0.0)) && ((s.v[1116] != 0.0) && (!(((((s.v[1111] != 0.0) || (s.v[1112] != 0.0)) || (s.v[1113] != 0.0)) || (s.v[1114] != 0.0)) || (s.v[1115] != 0.0))))) && (s.v[1226] != 0.0)) && (s.v[1227] != 0.0)) && (s.v[1228] != 0.0)) && (!(s.v[1230] != 0.0))) {
            s.store_div_from_scalar_ad(705, (p.p374 * s.v[236]), A::scale(s.ad_value(703), s.v[29]));
        }

        s.v[1232] = if ((s.v[703] == 0.0) || (s.v[236] == 0.0)) { 1.0 } else { 0.0 };

        if (((((((!(s.v[1103] != 0.0)) && (s.v[1104] != 0.0)) && ((s.v[1116] != 0.0) && (!(((((s.v[1111] != 0.0) || (s.v[1112] != 0.0)) || (s.v[1113] != 0.0)) || (s.v[1114] != 0.0)) || (s.v[1115] != 0.0))))) && (s.v[1226] != 0.0)) && (s.v[1227] != 0.0)) && ((s.v[1229] != 0.0) && (!(s.v[1228] != 0.0)))) && (s.v[1232] != 0.0)) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!(s.v[1103] != 0.0)) && (s.v[1104] != 0.0)) && ((s.v[1116] != 0.0) && (!(((((s.v[1111] != 0.0) || (s.v[1112] != 0.0)) || (s.v[1113] != 0.0)) || (s.v[1114] != 0.0)) || (s.v[1115] != 0.0))))) && (s.v[1226] != 0.0)) && (s.v[1227] != 0.0)) && ((s.v[1229] != 0.0) && (!(s.v[1228] != 0.0)))) && (!(s.v[1232] != 0.0))) {
            s.store_div_from_scalar_ad(705, (p.p374 * s.v[29]), A::scale(s.ad_value(703), (6.0 * s.v[236])));
        }

        if ((((((!(s.v[1103] != 0.0)) && (s.v[1104] != 0.0)) && ((s.v[1116] != 0.0) && (!(((((s.v[1111] != 0.0) || (s.v[1112] != 0.0)) || (s.v[1113] != 0.0)) || (s.v[1114] != 0.0)) || (s.v[1115] != 0.0))))) && (s.v[1226] != 0.0)) && (s.v[1227] != 0.0)) && (!((s.v[1228] != 0.0) || (s.v[1229] != 0.0)))) {
            s.store_scalar(705, 0.0);
        }

        s.v[1233] = if (((p.p10 == 1.0) || (p.p10 == 3.0)) || (p.p10 == 7.0)) { 1.0 } else { 0.0 };

        s.v[1234] = if (((p.p10 == 2.0) || (p.p10 == 4.0)) || (p.p10 == 8.0)) { 1.0 } else { 0.0 };

        s.v[1235] = if (s.v[703] == 0.0) { 1.0 } else { 0.0 };

        if (((((((!(s.v[1103] != 0.0)) && (s.v[1104] != 0.0)) && ((s.v[1116] != 0.0) && (!(((((s.v[1111] != 0.0) || (s.v[1112] != 0.0)) || (s.v[1113] != 0.0)) || (s.v[1114] != 0.0)) || (s.v[1115] != 0.0))))) && (s.v[1226] != 0.0)) && (!(s.v[1227] != 0.0))) && (s.v[1233] != 0.0)) && (s.v[1235] != 0.0)) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!(s.v[1103] != 0.0)) && (s.v[1104] != 0.0)) && ((s.v[1116] != 0.0) && (!(((((s.v[1111] != 0.0) || (s.v[1112] != 0.0)) || (s.v[1113] != 0.0)) || (s.v[1114] != 0.0)) || (s.v[1115] != 0.0))))) && (s.v[1226] != 0.0)) && (!(s.v[1227] != 0.0))) && (s.v[1233] != 0.0)) && (!(s.v[1235] != 0.0))) {
            s.store_div_from_scalar_ad(705, (p.p374 * s.v[236]), A::scale(s.ad_value(703), s.v[29]));
        }

        s.v[1237] = if ((s.v[703] == 0.0) || (s.v[236] == 0.0)) { 1.0 } else { 0.0 };

        if (((((((!(s.v[1103] != 0.0)) && (s.v[1104] != 0.0)) && ((s.v[1116] != 0.0) && (!(((((s.v[1111] != 0.0) || (s.v[1112] != 0.0)) || (s.v[1113] != 0.0)) || (s.v[1114] != 0.0)) || (s.v[1115] != 0.0))))) && (s.v[1226] != 0.0)) && (!(s.v[1227] != 0.0))) && ((s.v[1234] != 0.0) && (!(s.v[1233] != 0.0)))) && (s.v[1237] != 0.0)) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!(s.v[1103] != 0.0)) && (s.v[1104] != 0.0)) && ((s.v[1116] != 0.0) && (!(((((s.v[1111] != 0.0) || (s.v[1112] != 0.0)) || (s.v[1113] != 0.0)) || (s.v[1114] != 0.0)) || (s.v[1115] != 0.0))))) && (s.v[1226] != 0.0)) && (!(s.v[1227] != 0.0))) && ((s.v[1234] != 0.0) && (!(s.v[1233] != 0.0)))) && (!(s.v[1237] != 0.0))) {
            s.store_div_from_scalar_ad(705, (p.p374 * s.v[29]), A::scale(s.ad_value(703), (6.0 * s.v[236])));
        }

        if ((((((!(s.v[1103] != 0.0)) && (s.v[1104] != 0.0)) && ((s.v[1116] != 0.0) && (!(((((s.v[1111] != 0.0) || (s.v[1112] != 0.0)) || (s.v[1113] != 0.0)) || (s.v[1114] != 0.0)) || (s.v[1115] != 0.0))))) && (s.v[1226] != 0.0)) && (!(s.v[1227] != 0.0))) && (!((s.v[1233] != 0.0) || (s.v[1234] != 0.0)))) {
            s.store_scalar(705, 0.0);
        }

        s.v[1238] = if (s.v[701] == 0.0) { 1.0 } else { 0.0 };

        if (((((!(s.v[1103] != 0.0)) && (s.v[1104] != 0.0)) && ((s.v[1116] != 0.0) && (!(((((s.v[1111] != 0.0) || (s.v[1112] != 0.0)) || (s.v[1113] != 0.0)) || (s.v[1114] != 0.0)) || (s.v[1115] != 0.0))))) && (!(s.v[1226] != 0.0))) && (s.v[1238] != 0.0)) {
            s.store_scalar(705, 0.0);
        }

        if (((((!(s.v[1103] != 0.0)) && (s.v[1104] != 0.0)) && ((s.v[1116] != 0.0) && (!(((((s.v[1111] != 0.0) || (s.v[1112] != 0.0)) || (s.v[1113] != 0.0)) || (s.v[1114] != 0.0)) || (s.v[1115] != 0.0))))) && (!(s.v[1226] != 0.0))) && (!(s.v[1238] != 0.0))) {
            s.store_div_from_scalar_ad(705, (p.p374 * s.v[237]), A::scale(s.ad_value(701), s.v[29]));
        }

        s.v[1239] = if (0.0 == 1.0) { 1.0 } else { 0.0 };

        if ((((!(s.v[1103] != 0.0)) && (s.v[1104] != 0.0)) && ((s.v[1117] != 0.0) && (!((((((s.v[1111] != 0.0) || (s.v[1112] != 0.0)) || (s.v[1113] != 0.0)) || (s.v[1114] != 0.0)) || (s.v[1115] != 0.0)) || (s.v[1116] != 0.0))))) && (s.v[1239] != 0.0)) {
            s.store_scalar(705, ((p.p374 * s.v[237]) / s.v[29]));
        }

        s.v[1240] = if (0.0 == 1.0) { 1.0 } else { 0.0 };

        s.v[1241] = if (((p.p10 == 1.0) || (p.p10 == 2.0)) || (p.p10 == 5.0)) { 1.0 } else { 0.0 };

        s.v[1242] = if (((p.p10 == 3.0) || (p.p10 == 4.0)) || (p.p10 == 6.0)) { 1.0 } else { 0.0 };

        s.v[1243] = if (s.v[701] == 0.0) { 1.0 } else { 0.0 };

        if (((((((!(s.v[1103] != 0.0)) && (s.v[1104] != 0.0)) && ((s.v[1117] != 0.0) && (!((((((s.v[1111] != 0.0) || (s.v[1112] != 0.0)) || (s.v[1113] != 0.0)) || (s.v[1114] != 0.0)) || (s.v[1115] != 0.0)) || (s.v[1116] != 0.0))))) && (!(s.v[1239] != 0.0))) && (s.v[1240] != 0.0)) && (s.v[1241] != 0.0)) && (s.v[1243] != 0.0)) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!(s.v[1103] != 0.0)) && (s.v[1104] != 0.0)) && ((s.v[1117] != 0.0) && (!((((((s.v[1111] != 0.0) || (s.v[1112] != 0.0)) || (s.v[1113] != 0.0)) || (s.v[1114] != 0.0)) || (s.v[1115] != 0.0)) || (s.v[1116] != 0.0))))) && (!(s.v[1239] != 0.0))) && (s.v[1240] != 0.0)) && (s.v[1241] != 0.0)) && (!(s.v[1243] != 0.0))) {
            s.store_div_from_scalar_ad(705, (p.p374 * s.v[236]), A::scale(s.ad_value(701), s.v[29]));
        }

        s.v[1245] = if ((s.v[701] == 0.0) || ((s.v[236] + s.v[238]) == 0.0)) { 1.0 } else { 0.0 };

        if (((((((!(s.v[1103] != 0.0)) && (s.v[1104] != 0.0)) && ((s.v[1117] != 0.0) && (!((((((s.v[1111] != 0.0) || (s.v[1112] != 0.0)) || (s.v[1113] != 0.0)) || (s.v[1114] != 0.0)) || (s.v[1115] != 0.0)) || (s.v[1116] != 0.0))))) && (!(s.v[1239] != 0.0))) && (s.v[1240] != 0.0)) && ((s.v[1242] != 0.0) && (!(s.v[1241] != 0.0)))) && (s.v[1245] != 0.0)) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!(s.v[1103] != 0.0)) && (s.v[1104] != 0.0)) && ((s.v[1117] != 0.0) && (!((((((s.v[1111] != 0.0) || (s.v[1112] != 0.0)) || (s.v[1113] != 0.0)) || (s.v[1114] != 0.0)) || (s.v[1115] != 0.0)) || (s.v[1116] != 0.0))))) && (!(s.v[1239] != 0.0))) && (s.v[1240] != 0.0)) && ((s.v[1242] != 0.0) && (!(s.v[1241] != 0.0)))) && (!(s.v[1245] != 0.0))) {
            s.store_div_from_scalar_ad(705, (p.p374 * s.v[29]), A::scale(s.ad_value(701), (3.0 * (s.v[236] + s.v[238]))));
        }

        if ((((((!(s.v[1103] != 0.0)) && (s.v[1104] != 0.0)) && ((s.v[1117] != 0.0) && (!((((((s.v[1111] != 0.0) || (s.v[1112] != 0.0)) || (s.v[1113] != 0.0)) || (s.v[1114] != 0.0)) || (s.v[1115] != 0.0)) || (s.v[1116] != 0.0))))) && (!(s.v[1239] != 0.0))) && (s.v[1240] != 0.0)) && (!((s.v[1241] != 0.0) || (s.v[1242] != 0.0)))) {
            s.store_scalar(705, 0.0);
        }

        s.v[1246] = if (((p.p10 == 1.0) || (p.p10 == 3.0)) || (p.p10 == 7.0)) { 1.0 } else { 0.0 };

        s.v[1247] = if (((p.p10 == 2.0) || (p.p10 == 4.0)) || (p.p10 == 8.0)) { 1.0 } else { 0.0 };

        s.v[1248] = if (s.v[701] == 0.0) { 1.0 } else { 0.0 };

        if (((((((!(s.v[1103] != 0.0)) && (s.v[1104] != 0.0)) && ((s.v[1117] != 0.0) && (!((((((s.v[1111] != 0.0) || (s.v[1112] != 0.0)) || (s.v[1113] != 0.0)) || (s.v[1114] != 0.0)) || (s.v[1115] != 0.0)) || (s.v[1116] != 0.0))))) && (!(s.v[1239] != 0.0))) && (!(s.v[1240] != 0.0))) && (s.v[1246] != 0.0)) && (s.v[1248] != 0.0)) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!(s.v[1103] != 0.0)) && (s.v[1104] != 0.0)) && ((s.v[1117] != 0.0) && (!((((((s.v[1111] != 0.0) || (s.v[1112] != 0.0)) || (s.v[1113] != 0.0)) || (s.v[1114] != 0.0)) || (s.v[1115] != 0.0)) || (s.v[1116] != 0.0))))) && (!(s.v[1239] != 0.0))) && (!(s.v[1240] != 0.0))) && (s.v[1246] != 0.0)) && (!(s.v[1248] != 0.0))) {
            s.store_div_from_scalar_ad(705, (p.p374 * s.v[236]), A::scale(s.ad_value(701), s.v[29]));
        }

        s.v[1250] = if ((s.v[701] == 0.0) || ((s.v[236] + s.v[238]) == 0.0)) { 1.0 } else { 0.0 };

        if (((((((!(s.v[1103] != 0.0)) && (s.v[1104] != 0.0)) && ((s.v[1117] != 0.0) && (!((((((s.v[1111] != 0.0) || (s.v[1112] != 0.0)) || (s.v[1113] != 0.0)) || (s.v[1114] != 0.0)) || (s.v[1115] != 0.0)) || (s.v[1116] != 0.0))))) && (!(s.v[1239] != 0.0))) && (!(s.v[1240] != 0.0))) && ((s.v[1247] != 0.0) && (!(s.v[1246] != 0.0)))) && (s.v[1250] != 0.0)) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!(s.v[1103] != 0.0)) && (s.v[1104] != 0.0)) && ((s.v[1117] != 0.0) && (!((((((s.v[1111] != 0.0) || (s.v[1112] != 0.0)) || (s.v[1113] != 0.0)) || (s.v[1114] != 0.0)) || (s.v[1115] != 0.0)) || (s.v[1116] != 0.0))))) && (!(s.v[1239] != 0.0))) && (!(s.v[1240] != 0.0))) && ((s.v[1247] != 0.0) && (!(s.v[1246] != 0.0)))) && (!(s.v[1250] != 0.0))) {
            s.store_div_from_scalar_ad(705, (p.p374 * s.v[29]), A::scale(s.ad_value(701), (3.0 * (s.v[236] + s.v[238]))));
        }

        if ((((((!(s.v[1103] != 0.0)) && (s.v[1104] != 0.0)) && ((s.v[1117] != 0.0) && (!((((((s.v[1111] != 0.0) || (s.v[1112] != 0.0)) || (s.v[1113] != 0.0)) || (s.v[1114] != 0.0)) || (s.v[1115] != 0.0)) || (s.v[1116] != 0.0))))) && (!(s.v[1239] != 0.0))) && (!(s.v[1240] != 0.0))) && (!((s.v[1246] != 0.0) || (s.v[1247] != 0.0)))) {
            s.store_scalar(705, 0.0);
        }

        s.v[1251] = if (0.0 == 1.0) { 1.0 } else { 0.0 };

        s.v[1252] = if (s.v[703] == 0.0) { 1.0 } else { 0.0 };

        if (((((!(s.v[1103] != 0.0)) && (s.v[1104] != 0.0)) && ((s.v[1118] != 0.0) && (!(((((((s.v[1111] != 0.0) || (s.v[1112] != 0.0)) || (s.v[1113] != 0.0)) || (s.v[1114] != 0.0)) || (s.v[1115] != 0.0)) || (s.v[1116] != 0.0)) || (s.v[1117] != 0.0))))) && (s.v[1251] != 0.0)) && (s.v[1252] != 0.0)) {
            s.store_scalar(705, 0.0);
        }

        if (((((!(s.v[1103] != 0.0)) && (s.v[1104] != 0.0)) && ((s.v[1118] != 0.0) && (!(((((((s.v[1111] != 0.0) || (s.v[1112] != 0.0)) || (s.v[1113] != 0.0)) || (s.v[1114] != 0.0)) || (s.v[1115] != 0.0)) || (s.v[1116] != 0.0)) || (s.v[1117] != 0.0))))) && (s.v[1251] != 0.0)) && (!(s.v[1252] != 0.0))) {
            s.store_div_from_scalar_ad(705, (p.p374 * s.v[237]), A::scale(s.ad_value(703), s.v[29]));
        }

        s.v[1253] = if (0.0 == 1.0) { 1.0 } else { 0.0 };

        s.v[1254] = if (((p.p10 == 1.0) || (p.p10 == 2.0)) || (p.p10 == 5.0)) { 1.0 } else { 0.0 };

        s.v[1255] = if (((p.p10 == 3.0) || (p.p10 == 4.0)) || (p.p10 == 6.0)) { 1.0 } else { 0.0 };

        s.v[1256] = if (s.v[701] == 0.0) { 1.0 } else { 0.0 };

        if (((((((!(s.v[1103] != 0.0)) && (s.v[1104] != 0.0)) && ((s.v[1118] != 0.0) && (!(((((((s.v[1111] != 0.0) || (s.v[1112] != 0.0)) || (s.v[1113] != 0.0)) || (s.v[1114] != 0.0)) || (s.v[1115] != 0.0)) || (s.v[1116] != 0.0)) || (s.v[1117] != 0.0))))) && (!(s.v[1251] != 0.0))) && (s.v[1253] != 0.0)) && (s.v[1254] != 0.0)) && (s.v[1256] != 0.0)) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!(s.v[1103] != 0.0)) && (s.v[1104] != 0.0)) && ((s.v[1118] != 0.0) && (!(((((((s.v[1111] != 0.0) || (s.v[1112] != 0.0)) || (s.v[1113] != 0.0)) || (s.v[1114] != 0.0)) || (s.v[1115] != 0.0)) || (s.v[1116] != 0.0)) || (s.v[1117] != 0.0))))) && (!(s.v[1251] != 0.0))) && (s.v[1253] != 0.0)) && (s.v[1254] != 0.0)) && (!(s.v[1256] != 0.0))) {
            s.store_div_from_scalar_ad(705, (p.p374 * s.v[236]), A::scale(s.ad_value(701), s.v[29]));
        }

        s.v[1258] = if ((s.v[701] == 0.0) || (s.v[236] == 0.0)) { 1.0 } else { 0.0 };

        if (((((((!(s.v[1103] != 0.0)) && (s.v[1104] != 0.0)) && ((s.v[1118] != 0.0) && (!(((((((s.v[1111] != 0.0) || (s.v[1112] != 0.0)) || (s.v[1113] != 0.0)) || (s.v[1114] != 0.0)) || (s.v[1115] != 0.0)) || (s.v[1116] != 0.0)) || (s.v[1117] != 0.0))))) && (!(s.v[1251] != 0.0))) && (s.v[1253] != 0.0)) && ((s.v[1255] != 0.0) && (!(s.v[1254] != 0.0)))) && (s.v[1258] != 0.0)) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!(s.v[1103] != 0.0)) && (s.v[1104] != 0.0)) && ((s.v[1118] != 0.0) && (!(((((((s.v[1111] != 0.0) || (s.v[1112] != 0.0)) || (s.v[1113] != 0.0)) || (s.v[1114] != 0.0)) || (s.v[1115] != 0.0)) || (s.v[1116] != 0.0)) || (s.v[1117] != 0.0))))) && (!(s.v[1251] != 0.0))) && (s.v[1253] != 0.0)) && ((s.v[1255] != 0.0) && (!(s.v[1254] != 0.0)))) && (!(s.v[1258] != 0.0))) {
            s.store_div_from_scalar_ad(705, (p.p374 * s.v[29]), A::scale(s.ad_value(701), (6.0 * s.v[236])));
        }

        if ((((((!(s.v[1103] != 0.0)) && (s.v[1104] != 0.0)) && ((s.v[1118] != 0.0) && (!(((((((s.v[1111] != 0.0) || (s.v[1112] != 0.0)) || (s.v[1113] != 0.0)) || (s.v[1114] != 0.0)) || (s.v[1115] != 0.0)) || (s.v[1116] != 0.0)) || (s.v[1117] != 0.0))))) && (!(s.v[1251] != 0.0))) && (s.v[1253] != 0.0)) && (!((s.v[1254] != 0.0) || (s.v[1255] != 0.0)))) {
            s.store_scalar(705, 0.0);
        }

        s.v[1259] = if (((p.p10 == 1.0) || (p.p10 == 3.0)) || (p.p10 == 7.0)) { 1.0 } else { 0.0 };

        s.v[1260] = if (((p.p10 == 2.0) || (p.p10 == 4.0)) || (p.p10 == 8.0)) { 1.0 } else { 0.0 };

        s.v[1261] = if (s.v[701] == 0.0) { 1.0 } else { 0.0 };

        if (((((((!(s.v[1103] != 0.0)) && (s.v[1104] != 0.0)) && ((s.v[1118] != 0.0) && (!(((((((s.v[1111] != 0.0) || (s.v[1112] != 0.0)) || (s.v[1113] != 0.0)) || (s.v[1114] != 0.0)) || (s.v[1115] != 0.0)) || (s.v[1116] != 0.0)) || (s.v[1117] != 0.0))))) && (!(s.v[1251] != 0.0))) && (!(s.v[1253] != 0.0))) && (s.v[1259] != 0.0)) && (s.v[1261] != 0.0)) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!(s.v[1103] != 0.0)) && (s.v[1104] != 0.0)) && ((s.v[1118] != 0.0) && (!(((((((s.v[1111] != 0.0) || (s.v[1112] != 0.0)) || (s.v[1113] != 0.0)) || (s.v[1114] != 0.0)) || (s.v[1115] != 0.0)) || (s.v[1116] != 0.0)) || (s.v[1117] != 0.0))))) && (!(s.v[1251] != 0.0))) && (!(s.v[1253] != 0.0))) && (s.v[1259] != 0.0)) && (!(s.v[1261] != 0.0))) {
            s.store_div_from_scalar_ad(705, (p.p374 * s.v[236]), A::scale(s.ad_value(701), s.v[29]));
        }

        s.v[1263] = if ((s.v[701] == 0.0) || (s.v[236] == 0.0)) { 1.0 } else { 0.0 };

        if (((((((!(s.v[1103] != 0.0)) && (s.v[1104] != 0.0)) && ((s.v[1118] != 0.0) && (!(((((((s.v[1111] != 0.0) || (s.v[1112] != 0.0)) || (s.v[1113] != 0.0)) || (s.v[1114] != 0.0)) || (s.v[1115] != 0.0)) || (s.v[1116] != 0.0)) || (s.v[1117] != 0.0))))) && (!(s.v[1251] != 0.0))) && (!(s.v[1253] != 0.0))) && ((s.v[1260] != 0.0) && (!(s.v[1259] != 0.0)))) && (s.v[1263] != 0.0)) {
            s.store_scalar(705, 0.0);
        }

        if (((((((!(s.v[1103] != 0.0)) && (s.v[1104] != 0.0)) && ((s.v[1118] != 0.0) && (!(((((((s.v[1111] != 0.0) || (s.v[1112] != 0.0)) || (s.v[1113] != 0.0)) || (s.v[1114] != 0.0)) || (s.v[1115] != 0.0)) || (s.v[1116] != 0.0)) || (s.v[1117] != 0.0))))) && (!(s.v[1251] != 0.0))) && (!(s.v[1253] != 0.0))) && ((s.v[1260] != 0.0) && (!(s.v[1259] != 0.0)))) && (!(s.v[1263] != 0.0))) {
            s.store_div_from_scalar_ad(705, (p.p374 * s.v[29]), A::scale(s.ad_value(701), (6.0 * s.v[236])));
        }

        if ((((((!(s.v[1103] != 0.0)) && (s.v[1104] != 0.0)) && ((s.v[1118] != 0.0) && (!(((((((s.v[1111] != 0.0) || (s.v[1112] != 0.0)) || (s.v[1113] != 0.0)) || (s.v[1114] != 0.0)) || (s.v[1115] != 0.0)) || (s.v[1116] != 0.0)) || (s.v[1117] != 0.0))))) && (!(s.v[1251] != 0.0))) && (!(s.v[1253] != 0.0))) && (!((s.v[1259] != 0.0) || (s.v[1260] != 0.0)))) {
            s.store_scalar(705, 0.0);
        }

        if (((!(s.v[1103] != 0.0)) && (s.v[1104] != 0.0)) && ((s.v[1119] != 0.0) && (!((((((((s.v[1111] != 0.0) || (s.v[1112] != 0.0)) || (s.v[1113] != 0.0)) || (s.v[1114] != 0.0)) || (s.v[1115] != 0.0)) || (s.v[1116] != 0.0)) || (s.v[1117] != 0.0)) || (s.v[1118] != 0.0))))) {
            s.store_scalar(705, ((p.p374 * s.v[237]) / s.v[29]));
        }

        s.v[1264] = if (0.0 == 1.0) { 1.0 } else { 0.0 };

        if ((((!(s.v[1103] != 0.0)) && (s.v[1104] != 0.0)) && ((s.v[1120] != 0.0) && (!(((((((((s.v[1111] != 0.0) || (s.v[1112] != 0.0)) || (s.v[1113] != 0.0)) || (s.v[1114] != 0.0)) || (s.v[1115] != 0.0)) || (s.v[1116] != 0.0)) || (s.v[1117] != 0.0)) || (s.v[1118] != 0.0)) || (s.v[1119] != 0.0))))) && (s.v[1264] != 0.0)) {
            s.store_scalar(705, (((0.5 * p.p374) * s.v[236]) / s.v[29]));
        }

        s.v[1265] = if (p.p2 == 2.0) { 1.0 } else { 0.0 };

        if (((((!(s.v[1103] != 0.0)) && (s.v[1104] != 0.0)) && ((s.v[1120] != 0.0) && (!(((((((((s.v[1111] != 0.0) || (s.v[1112] != 0.0)) || (s.v[1113] != 0.0)) || (s.v[1114] != 0.0)) || (s.v[1115] != 0.0)) || (s.v[1116] != 0.0)) || (s.v[1117] != 0.0)) || (s.v[1118] != 0.0)) || (s.v[1119] != 0.0))))) && (s.v[1264] != 0.0)) && (s.v[1265] != 0.0)) {
            s.store_scalar(704, 0.0);
        }

        if (((((!(s.v[1103] != 0.0)) && (s.v[1104] != 0.0)) && ((s.v[1120] != 0.0) && (!(((((((((s.v[1111] != 0.0) || (s.v[1112] != 0.0)) || (s.v[1113] != 0.0)) || (s.v[1114] != 0.0)) || (s.v[1115] != 0.0)) || (s.v[1116] != 0.0)) || (s.v[1117] != 0.0)) || (s.v[1118] != 0.0)) || (s.v[1119] != 0.0))))) && (s.v[1264] != 0.0)) && (!(s.v[1265] != 0.0))) {
            s.store_scalar(704, ((p.p374 * s.v[236]) / (s.v[29] * (p.p2 - 2.0))));
        }

        if ((((!(s.v[1103] != 0.0)) && (s.v[1104] != 0.0)) && ((s.v[1120] != 0.0) && (!(((((((((s.v[1111] != 0.0) || (s.v[1112] != 0.0)) || (s.v[1113] != 0.0)) || (s.v[1114] != 0.0)) || (s.v[1115] != 0.0)) || (s.v[1116] != 0.0)) || (s.v[1117] != 0.0)) || (s.v[1118] != 0.0)) || (s.v[1119] != 0.0))))) && (!(s.v[1264] != 0.0))) {
            s.store_scalar(705, 0.0);
        }

        if ((((!(s.v[1103] != 0.0)) && (s.v[1104] != 0.0)) && ((s.v[1120] != 0.0) && (!(((((((((s.v[1111] != 0.0) || (s.v[1112] != 0.0)) || (s.v[1113] != 0.0)) || (s.v[1114] != 0.0)) || (s.v[1115] != 0.0)) || (s.v[1116] != 0.0)) || (s.v[1117] != 0.0)) || (s.v[1118] != 0.0)) || (s.v[1119] != 0.0))))) && (!(s.v[1264] != 0.0))) {
            s.store_scalar(704, ((p.p374 * s.v[236]) / (s.v[29] * p.p2)));
        }

        s.v[1266] = if (0.0 == 1.0) { 1.0 } else { 0.0 };

        if ((((!(s.v[1103] != 0.0)) && (s.v[1104] != 0.0)) && ((s.v[1121] != 0.0) && (!((((((((((s.v[1111] != 0.0) || (s.v[1112] != 0.0)) || (s.v[1113] != 0.0)) || (s.v[1114] != 0.0)) || (s.v[1115] != 0.0)) || (s.v[1116] != 0.0)) || (s.v[1117] != 0.0)) || (s.v[1118] != 0.0)) || (s.v[1119] != 0.0)) || (s.v[1120] != 0.0))))) && (s.v[1266] != 0.0)) {
            s.store_scalar(705, 0.0);
        }

        if ((((!(s.v[1103] != 0.0)) && (s.v[1104] != 0.0)) && ((s.v[1121] != 0.0) && (!((((((((((s.v[1111] != 0.0) || (s.v[1112] != 0.0)) || (s.v[1113] != 0.0)) || (s.v[1114] != 0.0)) || (s.v[1115] != 0.0)) || (s.v[1116] != 0.0)) || (s.v[1117] != 0.0)) || (s.v[1118] != 0.0)) || (s.v[1119] != 0.0)) || (s.v[1120] != 0.0))))) && (s.v[1266] != 0.0)) {
            s.store_scalar(704, ((p.p374 * s.v[236]) / (s.v[29] * p.p2)));
        }

        if ((((!(s.v[1103] != 0.0)) && (s.v[1104] != 0.0)) && ((s.v[1121] != 0.0) && (!((((((((((s.v[1111] != 0.0) || (s.v[1112] != 0.0)) || (s.v[1113] != 0.0)) || (s.v[1114] != 0.0)) || (s.v[1115] != 0.0)) || (s.v[1116] != 0.0)) || (s.v[1117] != 0.0)) || (s.v[1118] != 0.0)) || (s.v[1119] != 0.0)) || (s.v[1120] != 0.0))))) && (!(s.v[1266] != 0.0))) {
            s.store_scalar(705, (((0.5 * p.p374) * s.v[236]) / s.v[29]));
        }

        s.v[1267] = if (p.p2 == 2.0) { 1.0 } else { 0.0 };

        if (((((!(s.v[1103] != 0.0)) && (s.v[1104] != 0.0)) && ((s.v[1121] != 0.0) && (!((((((((((s.v[1111] != 0.0) || (s.v[1112] != 0.0)) || (s.v[1113] != 0.0)) || (s.v[1114] != 0.0)) || (s.v[1115] != 0.0)) || (s.v[1116] != 0.0)) || (s.v[1117] != 0.0)) || (s.v[1118] != 0.0)) || (s.v[1119] != 0.0)) || (s.v[1120] != 0.0))))) && (!(s.v[1266] != 0.0))) && (s.v[1267] != 0.0)) {
            s.store_scalar(704, 0.0);
        }

        if (((((!(s.v[1103] != 0.0)) && (s.v[1104] != 0.0)) && ((s.v[1121] != 0.0) && (!((((((((((s.v[1111] != 0.0) || (s.v[1112] != 0.0)) || (s.v[1113] != 0.0)) || (s.v[1114] != 0.0)) || (s.v[1115] != 0.0)) || (s.v[1116] != 0.0)) || (s.v[1117] != 0.0)) || (s.v[1118] != 0.0)) || (s.v[1119] != 0.0)) || (s.v[1120] != 0.0))))) && (!(s.v[1266] != 0.0))) && (!(s.v[1267] != 0.0))) {
            s.store_scalar(704, ((p.p374 * s.v[236]) / (s.v[29] * (p.p2 - 2.0))));
        }

        if (((!(s.v[1103] != 0.0)) && (s.v[1104] != 0.0)) && (!(((((((((((s.v[1111] != 0.0) || (s.v[1112] != 0.0)) || (s.v[1113] != 0.0)) || (s.v[1114] != 0.0)) || (s.v[1115] != 0.0)) || (s.v[1116] != 0.0)) || (s.v[1117] != 0.0)) || (s.v[1118] != 0.0)) || (s.v[1119] != 0.0)) || (s.v[1120] != 0.0)) || (s.v[1121] != 0.0)))) {
            s.store_scalar(704, 0.0);
        }

        s.v[1268] = if (s.v[704] <= 0.0) { 1.0 } else { 0.0 };

        if (((!(s.v[1103] != 0.0)) && (s.v[1104] != 0.0)) && (s.v[1268] != 0.0)) {
            s.copy_ad(240, 705);
        }

        s.v[1269] = if (s.v[705] <= 0.0) { 1.0 } else { 0.0 };

        if ((((!(s.v[1103] != 0.0)) && (s.v[1104] != 0.0)) && (!(s.v[1268] != 0.0))) && (s.v[1269] != 0.0)) {
            s.copy_ad(240, 704);
        }

        if ((((!(s.v[1103] != 0.0)) && (s.v[1104] != 0.0)) && (!(s.v[1268] != 0.0))) && (!(s.v[1269] != 0.0))) {
            s.store_div_ad(240, A::mul(s.ad_value(704), s.ad_value(705)), A::add(s.ad_value(704), s.ad_value(705)));
        }

        if ((!(s.v[1103] != 0.0)) && (!(s.v[1104] != 0.0))) {
            s.store_scalar(240, 0.0);
        }

        s.v[1271] = if (p.p42 == 0.0) { 1.0 } else { 0.0 };

        s.v[1272] = if (s.v[239] < p.p1093) { 1.0 } else { 0.0 };

        if ((s.v[1271] != 0.0) && (s.v[1272] != 0.0)) {
            s.store_scalar(239, 0.0);
        }

        s.v[1273] = if (s.v[240] < p.p1093) { 1.0 } else { 0.0 };

        if ((s.v[1271] != 0.0) && (s.v[1273] != 0.0)) {
            s.store_scalar(240, 0.0);
        }

        s.v[1274] = if (s.v[239] <= p.p1093) { 1.0 } else { 0.0 };

        if ((!(s.v[1271] != 0.0)) && (s.v[1274] != 0.0)) {
            s.store_scalar(239, p.p1093);
        }

        s.v[1275] = if (s.v[240] <= p.p1093) { 1.0 } else { 0.0 };

        if ((!(s.v[1271] != 0.0)) && (s.v[1275] != 0.0)) {
            s.store_scalar(240, p.p1093);
        }

        s.v[1276] = if (p.p42 == 1.0) { 1.0 } else { 0.0 };

        s.v[1277] = if (s.v[529] <= 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1276] != 0.0) && (s.v[1277] != 0.0)) {
            s.store_scalar(529, 0.0);
        }

        s.v[1278] = if (s.v[528] <= 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1276] != 0.0) && (s.v[1278] != 0.0)) {
            s.store_scalar(528, 0.0);
        }

        s.v[1279] = if (s.v[531] <= 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1276] != 0.0) && (s.v[1279] != 0.0)) {
            s.store_scalar(531, 0.0);
        }

        s.v[1280] = if (s.v[530] <= 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1276] != 0.0) && (s.v[1280] != 0.0)) {
            s.store_scalar(530, 0.0);
        }

        s.v[1281] = if (s.v[533] <= 0.0) { 1.0 } else { 0.0 };

        if ((!(s.v[1276] != 0.0)) && (s.v[1281] != 0.0)) {
            s.store_scalar(533, 0.0);
        }

        s.v[1282] = if (s.v[532] <= 0.0) { 1.0 } else { 0.0 };

        if ((!(s.v[1276] != 0.0)) && (s.v[1282] != 0.0)) {
            s.store_scalar(532, 0.0);
        }

        s.v[1301] = if (p.p1097 == 1.0) { 1.0 } else { 0.0 };

        if (s.v[1301] != 0.0) {
            s.store_scalar(302, (1.0 - p.p1128));
        }

        if (!(s.v[1301] != 0.0)) {
            s.store_scalar(302, 1.0);
        }

        s.v[252] = ((p.p700 * (p.p31 + ((s.v[35] / 3.0) / p.p32))) / ((p.p32 * p.p2) * (s.v[98] - p.p699)));

        s.v[1303] = if (s.v[252] > 0.0) { 1.0 } else { 0.0 };

        if (s.v[1303] != 0.0) {
            s.store_scalar(252, (1.0 / s.v[252]));
        }

        if (!(s.v[1303] != 0.0)) {
            s.store_scalar(252, 1000.0);
        }

        s.v[12] = (p.p77 * p.p77);

        s.store_scale(13, 599, p.p77);

        s.store_square(14, 13);

        s.v[295] = (if (p.p39 == 1.0) { 745669000000.0 } else { 1166450000000.0 });

        s.store_scale(297, 599, ((-s.v[295]) * p.p77));

        s.v[295] = ((-s.v[295]) * p.p77);

        s.v[38] = (p.p911 + s.v[29]);

        s.v[1305] = if (((p.p49 != 0.0) && (p.p909 > 0.0)) && (s.v[38] > 0.0)) { 1.0 } else { 0.0 };

        if (s.v[1305] != 0.0) {
            s.store_scalar(747, ((s.v[38] * p.p2) / p.p909));
        }

        if (s.v[1305] != 0.0) {
            s.store_scalar(748, ((p.p910 * s.v[38]) * p.p2));
        }

        if (!(s.v[1305] != 0.0)) {
            s.store_scalar(747, 1.0);
        }

        if (!(s.v[1305] != 0.0)) {
            s.store_scalar(748, 0.0);
        }

        s.v[1306] = if (p.p820 <= (-273.15)) { 1.0 } else { 0.0 };

        if (s.v[1306] != 0.0) {
            s.store_scalar(12, (300.15 - 273.15));
        }

        if (s.v[1306] != 0.0) {
            s.store_scalar(392, 300.15);
        }

        if (!(s.v[1306] != 0.0)) {
            s.store_scalar(392, (p.p820 + 273.15));
        }

        s.v[391] = (ctx.temperature() + p.p33);

        s.v[1307] = if (((p.p49 != 0.0) && (p.p909 > 0.0)) && (s.v[38] > 0.0)) { 1.0 } else { 0.0 };

        if (s.v[1307] != 0.0) {
            s.store_ad(390, &A::voltage(ctx, &nodes, Some(4), None));
        }

        if (!(s.v[1307] != 0.0)) {
            s.store_scalar(390, 0.0);
        }

        s.store_offset(391, 390, s.v[391]);

        s.store_scale(108, 391, 8.617087e-5);

        s.store_div_from_scalar(109, 1.0, 108);

        s.store_div(395, 391, 392);

        s.store_sub(396, 391, 392);

        s.store_scale(393, 391, 8.617087e-5);

        s.store_scale(394, 392, 8.617087e-5);

        s.store_sub_from_scalar_ad(36, p.p109, A::div(A::mul(A::scale(s.ad_value(391), p.p821), s.ad_value(391)), A::offset(s.ad_value(391), p.p822)));

    }

    pub(super) fn stamp_reactive_block_6(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        s.store_sub_from_scalar_ad(37, p.p109, A::div(A::mul(A::scale(s.ad_value(392), p.p821), s.ad_value(392)), A::offset(s.ad_value(392), p.p822)));

        s.store_mul_ad(13, A::div(s.ad_value(391), s.ad_value(392)), A::sqrt(A::div(s.ad_value(391), s.ad_value(392))));

        s.store_mul_ad(28, A::scale(s.ad_value(13), p.p108), A::limited_exp(A::sub(A::div(s.ad_value(36), A::scale(s.ad_value(394), 2.0)), A::div(s.ad_value(36), A::scale(s.ad_value(393), 2.0)))));

        s.v[1308] = if (((p.p49 != 0.0) && (p.p909 > 0.0)) && (s.v[38] > 0.0)) { 1.0 } else { 0.0 };

        if (s.v[1308] != 0.0) {
            s.store_ln_ad(12, A::max_with_scalar(A::div(s.ad_value(481), s.ad_value(28)), 1e-38));
        }

        if (s.v[1308] != 0.0) {
            s.store_sqrt_ad(88, A::offset(A::square(s.ad_value(12)), 1e-6));
        }

        if (!(s.v[1308] != 0.0)) {
            s.store_ln_ad(88, A::max_with_scalar(A::div(s.ad_value(481), s.ad_value(28)), 1e-38));
        }

        s.v[1309] = if (((p.p49 != 0.0) && (p.p909 > 0.0)) && (s.v[38] > 0.0)) { 1.0 } else { 0.0 };

        if (s.v[1309] != 0.0) {
            s.store_ln_ad(12, A::max_with_scalar(A::div(A::mul(s.ad_value(686), s.ad_value(480)), A::square(s.ad_value(28))), 1e-38));
        }

        if (s.v[1309] != 0.0) {
            s.store_sqrt_ad(675, A::offset(A::square(s.ad_value(12)), 1e-6));
        }

        if (!(s.v[1309] != 0.0)) {
            s.store_ln_ad(675, A::max_with_scalar(A::div(A::mul(s.ad_value(686), s.ad_value(480)), A::square(s.ad_value(28))), 1e-38));
        }

        s.v[1310] = if (s.v[479] > 0.0) { 1.0 } else { 0.0 };

        if (s.v[1310] != 0.0) {
            s.store_offset_ad(63, A::mul(A::mul(A::neg(s.ad_value(187)), s.ad_value(108)), A::ln(A::max_with_scalar(A::div(s.ad_value(479), s.ad_value(480)), 1e-38))), p.p5);
        }

        if (!(s.v[1310] != 0.0)) {
            s.store_scalar(63, 0.0);
        }

        s.store_max_with_scalar_ad(127, A::add(A::offset(A::mul(s.ad_value(108), s.ad_value(88)), 0.4), s.ad_value(489)), 0.4);

        s.store_sqrt(128, 127);

        s.store_sqrt_ad(114, A::div_from_scalar((2.0 * s.v[26]), A::scale(s.ad_value(481), 1.60219e-19)));

        s.store_sqrt_ad(129, A::scale(s.ad_value(538), ((s.v[26] / s.v[27]) * p.p77)));

        let assign13230_ad_e18111: A = {
    if (!((1.0 + (p.p823 * (s.v[395] - 1.0))) < ((-10000.0) * 0.001))) {
        A::scale(A::add(A::offset(A::scale(A::offset(s.ad_value(395), (-1.0)), p.p823), 1.0), A::sqrt(A::offset(A::mul(A::offset(A::scale(A::offset(s.ad_value(395), (-1.0)), p.p823), 1.0), A::offset(A::scale(A::offset(s.ad_value(395), (-1.0)), p.p823), 1.0)), ((4.0 * 0.001) * 0.001)))), 0.5)
    } else {
        {
            if ((1.0 + (p.p823 * (s.v[395] - 1.0))) < ((-10000.0) * 0.001)) {
                A::div_from_scalar(((-0.001) * 0.001), A::offset(A::scale(A::offset(s.ad_value(395), (-1.0)), p.p823), 1.0))
            } else {
                A::constant(0.0)
            }
        }
    }
};
        s.store_mul_ad_rhs(422, 488, assign13230_ad_e18111);

        s.store_mul_ad_rhs(420, 490, A::offset(A::scale(A::offset(s.ad_value(395), (-1.0)), p.p851), 1.0));

        s.v[1311] = if (p.p44 != 0.0) { 1.0 } else { 0.0 };

        if (s.v[1311] != 0.0) {
            s.store_mul_ad_rhs(421, 491, A::offset(A::scale(A::offset(s.ad_value(395), (-1.0)), p.p851), 1.0));
        }

        s.v[158] = (if (p.p39 != 1.0) { (0.3333333333333333 * p.p283) } else { (0.5 * p.p283) });

        s.store_mul_ad_rhs(397, 497, A::pow(s.ad_value(395), s.ad_value(567)));

        let assign13290_ad_e18224: A = {
    if (!(((1.0 + (s.v[568] * s.v[396])) - 1e-6) < ((-10000.0) * 0.001))) {
        A::scale(A::add(A::offset(A::offset(A::mul(s.ad_value(568), s.ad_value(396)), 1.0), (-1e-6)), A::sqrt(A::offset(A::mul(A::offset(A::offset(A::mul(s.ad_value(568), s.ad_value(396)), 1.0), (-1e-6)), A::offset(A::offset(A::mul(s.ad_value(568), s.ad_value(396)), 1.0), (-1e-6))), ((4.0 * 0.001) * 0.001)))), 0.5)
    } else {
        {
            if (((1.0 + (s.v[568] * s.v[396])) - 1e-6) < ((-10000.0) * 0.001)) {
                A::div_from_scalar(((-0.001) * 0.001), A::offset(A::offset(A::mul(s.ad_value(568), s.ad_value(396)), 1.0), (-1e-6)))
            } else {
                A::constant(0.0)
            }
        }
    }
};
        s.store_mul_ad_rhs(399, 504, assign13290_ad_e18224);

        let assign13300_ad_e18298: A = {
    if (!(((1.0 + (s.v[569] * s.v[396])) - 1e-6) < ((-10000.0) * 0.001))) {
        A::scale(A::add(A::offset(A::offset(A::mul(s.ad_value(569), s.ad_value(396)), 1.0), (-1e-6)), A::sqrt(A::offset(A::mul(A::offset(A::offset(A::mul(s.ad_value(569), s.ad_value(396)), 1.0), (-1e-6)), A::offset(A::offset(A::mul(s.ad_value(569), s.ad_value(396)), 1.0), (-1e-6))), ((4.0 * 0.001) * 0.001)))), 0.5)
    } else {
        {
            if (((1.0 + (s.v[569] * s.v[396])) - 1e-6) < ((-10000.0) * 0.001)) {
                A::div_from_scalar(((-0.001) * 0.001), A::offset(A::offset(A::mul(s.ad_value(569), s.ad_value(396)), 1.0), (-1e-6)))
            } else {
                A::constant(0.0)
            }
        }
    }
};
        s.store_mul_ad_rhs(401, 514, assign13300_ad_e18298);

        s.store_mul_ad_rhs(403, 508, A::pow(s.ad_value(395), s.ad_value(570)));

        s.store_mul_ad_rhs(405, 511, A::pow(s.ad_value(395), s.ad_value(571)));

        let assign13330_ad_e18382: A = {
    if (!((1.0 + (s.v[577] * (s.v[395] - 1.0))) < ((-10000.0) * 0.001))) {
        A::scale(A::add(A::offset(A::mul(s.ad_value(577), A::offset(s.ad_value(395), (-1.0))), 1.0), A::sqrt(A::offset(A::mul(A::offset(A::mul(s.ad_value(577), A::offset(s.ad_value(395), (-1.0))), 1.0), A::offset(A::mul(s.ad_value(577), A::offset(s.ad_value(395), (-1.0))), 1.0)), ((4.0 * 0.001) * 0.001)))), 0.5)
    } else {
        {
            if ((1.0 + (s.v[577] * (s.v[395] - 1.0))) < ((-10000.0) * 0.001)) {
                A::div_from_scalar(((-0.001) * 0.001), A::offset(A::mul(s.ad_value(577), A::offset(s.ad_value(395), (-1.0))), 1.0))
            } else {
                A::constant(0.0)
            }
        }
    }
};
        s.store_mul_ad_rhs(407, 507, assign13330_ad_e18382);

        s.v[1312] = if (p.p44 != 0.0) { 1.0 } else { 0.0 };

        if (s.v[1312] != 0.0) {
            s.store_mul_ad_rhs(398, 498, A::pow(s.ad_value(395), s.ad_value(567)));
        }

        if (s.v[1312] != 0.0) {
            let assign13360_ad_e18468: A = {
                if (!(((1.0 + (s.v[568] * s.v[396])) - 1e-6) < ((-10000.0) * 0.001))) {
                    A::scale(A::add(A::offset(A::offset(A::mul(s.ad_value(568), s.ad_value(396)), 1.0), (-1e-6)), A::sqrt(A::offset(A::mul(A::offset(A::offset(A::mul(s.ad_value(568), s.ad_value(396)), 1.0), (-1e-6)), A::offset(A::offset(A::mul(s.ad_value(568), s.ad_value(396)), 1.0), (-1e-6))), ((4.0 * 0.001) * 0.001)))), 0.5)
                } else {
                    {
                        if (((1.0 + (s.v[568] * s.v[396])) - 1e-6) < ((-10000.0) * 0.001)) {
                            A::div_from_scalar(((-0.001) * 0.001), A::offset(A::offset(A::mul(s.ad_value(568), s.ad_value(396)), 1.0), (-1e-6)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_mul_ad_rhs(400, 505, assign13360_ad_e18468);
        }

        if (s.v[1312] != 0.0) {
            let assign13370_ad_e18545: A = {
                if (!(((1.0 + (s.v[569] * s.v[396])) - 1e-6) < ((-10000.0) * 0.001))) {
                    A::scale(A::add(A::offset(A::offset(A::mul(s.ad_value(569), s.ad_value(396)), 1.0), (-1e-6)), A::sqrt(A::offset(A::mul(A::offset(A::offset(A::mul(s.ad_value(569), s.ad_value(396)), 1.0), (-1e-6)), A::offset(A::offset(A::mul(s.ad_value(569), s.ad_value(396)), 1.0), (-1e-6))), ((4.0 * 0.001) * 0.001)))), 0.5)
                } else {
                    {
                        if (((1.0 + (s.v[569] * s.v[396])) - 1e-6) < ((-10000.0) * 0.001)) {
                            A::div_from_scalar(((-0.001) * 0.001), A::offset(A::offset(A::mul(s.ad_value(569), s.ad_value(396)), 1.0), (-1e-6)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_mul_ad_rhs(402, 515, assign13370_ad_e18545);
        }

        if (s.v[1312] != 0.0) {
            s.store_mul_ad_rhs(404, 509, A::pow(s.ad_value(395), s.ad_value(570)));
        }

        if (s.v[1312] != 0.0) {
            s.store_mul_ad_rhs(406, 512, A::pow(s.ad_value(395), s.ad_value(571)));
        }

        s.store_ad(408, &A::pow(s.ad_value(395), s.ad_value(572)));

        s.store_mul_ad_rhs(409, 500, A::pow(s.ad_value(395), A::neg(s.ad_value(573))));

        s.v[1313] = if (s.v[409] < 100.0) { 1.0 } else { 0.0 };

        if (s.v[1313] != 0.0) {
            s.store_scalar(409, 100.0);
        }

        s.v[1314] = if (p.p1094 == 1.0) { 1.0 } else { 0.0 };

        if (s.v[1314] != 0.0) {
            s.store_powf(762, 395, p.p1120);
        }

        if (s.v[1314] != 0.0) {
            s.store_scale_ad(763, A::powf(s.ad_value(395), (-p.p1121)), p.p1100);
        }

        s.v[1315] = if (p.p44 != 0.0) { 1.0 } else { 0.0 };

        if (s.v[1315] != 0.0) {
            s.store_mul_ad_rhs(410, 501, A::pow(s.ad_value(395), A::neg(s.ad_value(573))));
        }

        s.v[1316] = if (s.v[410] < 100.0) { 1.0 } else { 0.0 };

        if ((s.v[1315] != 0.0) && (s.v[1316] != 0.0)) {
            s.store_scalar(410, 100.0);
        }

        s.store_mul_ad_rhs(411, 503, A::pow(s.ad_value(395), A::neg(s.ad_value(573))));

        s.v[1317] = if (s.v[411] < 100.0) { 1.0 } else { 0.0 };

        if (s.v[1317] != 0.0) {
            s.store_scalar(411, 100.0);
        }

        let assign13540_ad_e18729: A = {
    if (!((((1.0 / s.v[496]) * (1.0 + (p.p861 * s.v[396]))) - 2.0) < ((-10000.0) * 0.001))) {
        let assign13540_ad_e18693: A = A::add(A::offset(A::mul(A::div_from_scalar(1.0, s.ad_value(496)), A::offset(A::scale(s.ad_value(396), p.p861), 1.0)), (-2.0)), A::sqrt(A::offset(A::mul(A::offset(A::mul(A::div_from_scalar(1.0, s.ad_value(496)), A::offset(A::scale(s.ad_value(396), p.p861), 1.0)), (-2.0)), A::offset(A::mul(A::div_from_scalar(1.0, s.ad_value(496)), A::offset(A::scale(s.ad_value(396), p.p861), 1.0)), (-2.0))), ((4.0 * 0.001) * 0.001))));
        A::scale(assign13540_ad_e18693, 0.5)
    } else {
        {
            if ((((1.0 / s.v[496]) * (1.0 + (p.p861 * s.v[396]))) - 2.0) < ((-10000.0) * 0.001)) {
                A::div_from_scalar(((-0.001) * 0.001), A::offset(A::mul(A::div_from_scalar(1.0, s.ad_value(496)), A::offset(A::scale(s.ad_value(396), p.p861), 1.0)), (-2.0)))
            } else {
                A::constant(0.0)
            }
        }
    }
};
        s.store_div_from_scalar_ad(412, 1.0, A::offset(assign13540_ad_e18729, 2.0));

        let assign13550_ad_e18805: A = {
    if (!(((1.0 - (s.v[574] * s.v[396])) - 1e-6) < ((-10000.0) * 0.001))) {
        A::scale(A::add(A::offset(A::sub_from_scalar(1.0, A::mul(s.ad_value(574), s.ad_value(396))), (-1e-6)), A::sqrt(A::offset(A::mul(A::offset(A::sub_from_scalar(1.0, A::mul(s.ad_value(574), s.ad_value(396))), (-1e-6)), A::offset(A::sub_from_scalar(1.0, A::mul(s.ad_value(574), s.ad_value(396))), (-1e-6))), ((4.0 * 0.001) * 0.001)))), 0.5)
    } else {
        {
            if (((1.0 - (s.v[574] * s.v[396])) - 1e-6) < ((-10000.0) * 0.001)) {
                A::div_from_scalar(((-0.001) * 0.001), A::offset(A::sub_from_scalar(1.0, A::mul(s.ad_value(574), s.ad_value(396))), (-1e-6)))
            } else {
                A::constant(0.0)
            }
        }
    }
};
        s.store_mul_ad_rhs(413, 534, assign13550_ad_e18805);

        s.v[1318] = if (p.p44 != 0.0) { 1.0 } else { 0.0 };

        if (s.v[1318] != 0.0) {
            let assign13570_ad_e18883: A = {
                if (!(((1.0 - (s.v[574] * s.v[396])) - 1e-6) < ((-10000.0) * 0.001))) {
                    A::scale(A::add(A::offset(A::sub_from_scalar(1.0, A::mul(s.ad_value(574), s.ad_value(396))), (-1e-6)), A::sqrt(A::offset(A::mul(A::offset(A::sub_from_scalar(1.0, A::mul(s.ad_value(574), s.ad_value(396))), (-1e-6)), A::offset(A::sub_from_scalar(1.0, A::mul(s.ad_value(574), s.ad_value(396))), (-1e-6))), ((4.0 * 0.001) * 0.001)))), 0.5)
                } else {
                    {
                        if (((1.0 - (s.v[574] * s.v[396])) - 1e-6) < ((-10000.0) * 0.001)) {
                            A::div_from_scalar(((-0.001) * 0.001), A::offset(A::sub_from_scalar(1.0, A::mul(s.ad_value(574), s.ad_value(396))), (-1e-6)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_mul_ad_rhs(414, 535, assign13570_ad_e18883);
        }

        let assign13580_ad_e18959: A = {
    if (!(((1.0 + (s.v[149] * s.v[396])) - 1e-6) < ((-10000.0) * 0.001))) {
        A::scale(A::add(A::offset(A::offset(A::mul(s.ad_value(149), s.ad_value(396)), 1.0), (-1e-6)), A::sqrt(A::offset(A::mul(A::offset(A::offset(A::mul(s.ad_value(149), s.ad_value(396)), 1.0), (-1e-6)), A::offset(A::offset(A::mul(s.ad_value(149), s.ad_value(396)), 1.0), (-1e-6))), ((4.0 * 0.001) * 0.001)))), 0.5)
    } else {
        {
            if (((1.0 + (s.v[149] * s.v[396])) - 1e-6) < ((-10000.0) * 0.001)) {
                A::div_from_scalar(((-0.001) * 0.001), A::offset(A::offset(A::mul(s.ad_value(149), s.ad_value(396)), 1.0), (-1e-6)))
            } else {
                A::constant(0.0)
            }
        }
    }
};
        s.store_mul_ad_rhs(150, 148, assign13580_ad_e18959);

        let assign13590_ad_e19033: A = {
    if (!(((1.0 + (s.v[152] * s.v[396])) - 1e-6) < ((-10000.0) * 0.001))) {
        A::scale(A::add(A::offset(A::offset(A::mul(s.ad_value(152), s.ad_value(396)), 1.0), (-1e-6)), A::sqrt(A::offset(A::mul(A::offset(A::offset(A::mul(s.ad_value(152), s.ad_value(396)), 1.0), (-1e-6)), A::offset(A::offset(A::mul(s.ad_value(152), s.ad_value(396)), 1.0), (-1e-6))), ((4.0 * 0.001) * 0.001)))), 0.5)
    } else {
        {
            if (((1.0 + (s.v[152] * s.v[396])) - 1e-6) < ((-10000.0) * 0.001)) {
                A::div_from_scalar(((-0.001) * 0.001), A::offset(A::offset(A::mul(s.ad_value(152), s.ad_value(396)), 1.0), (-1e-6)))
            } else {
                A::constant(0.0)
            }
        }
    }
};
        s.store_mul_ad_rhs(153, 151, assign13590_ad_e19033);

        s.store_mul_ad_rhs(415, 554, A::pow(s.ad_value(395), s.ad_value(575)));

        s.v[1319] = if (p.p44 != 0.0) { 1.0 } else { 0.0 };

        if (s.v[1319] != 0.0) {
            s.store_mul_ad_rhs(416, 557, A::pow(s.ad_value(395), s.ad_value(575)));
        }

        let assign13630_ad_e19123: A = {
    if (!(((1.0 + (s.v[576] * s.v[396])) - 1e-6) < ((-10000.0) * 0.001))) {
        A::scale(A::add(A::offset(A::offset(A::mul(s.ad_value(576), s.ad_value(396)), 1.0), (-1e-6)), A::sqrt(A::offset(A::mul(A::offset(A::offset(A::mul(s.ad_value(576), s.ad_value(396)), 1.0), (-1e-6)), A::offset(A::offset(A::mul(s.ad_value(576), s.ad_value(396)), 1.0), (-1e-6))), ((4.0 * 0.001) * 0.001)))), 0.5)
    } else {
        {
            if (((1.0 + (s.v[576] * s.v[396])) - 1e-6) < ((-10000.0) * 0.001)) {
                A::div_from_scalar(((-0.001) * 0.001), A::offset(A::offset(A::mul(s.ad_value(576), s.ad_value(396)), 1.0), (-1e-6)))
            } else {
                A::constant(0.0)
            }
        }
    }
};
        s.store_mul_ad_rhs(417, 560, assign13630_ad_e19123);

        let assign13640_ad_e19197: A = {
    if (!(((1.0 + (s.v[576] * s.v[396])) - 1e-6) < ((-10000.0) * 0.001))) {
        A::scale(A::add(A::offset(A::offset(A::mul(s.ad_value(576), s.ad_value(396)), 1.0), (-1e-6)), A::sqrt(A::offset(A::mul(A::offset(A::offset(A::mul(s.ad_value(576), s.ad_value(396)), 1.0), (-1e-6)), A::offset(A::offset(A::mul(s.ad_value(576), s.ad_value(396)), 1.0), (-1e-6))), ((4.0 * 0.001) * 0.001)))), 0.5)
    } else {
        {
            if (((1.0 + (s.v[576] * s.v[396])) - 1e-6) < ((-10000.0) * 0.001)) {
                A::div_from_scalar(((-0.001) * 0.001), A::offset(A::offset(A::mul(s.ad_value(576), s.ad_value(396)), 1.0), (-1e-6)))
            } else {
                A::constant(0.0)
            }
        }
    }
};
        s.store_mul_ad_rhs(418, 564, assign13640_ad_e19197);

        let assign13660_ad_e19278: A = {
    if (!(((1.0 + (s.v[607] * s.v[396])) - 1e-6) < ((-10000.0) * 0.001))) {
        A::scale(A::add(A::offset(A::offset(A::mul(s.ad_value(607), s.ad_value(396)), 1.0), (-1e-6)), A::sqrt(A::offset(A::mul(A::offset(A::offset(A::mul(s.ad_value(607), s.ad_value(396)), 1.0), (-1e-6)), A::offset(A::offset(A::mul(s.ad_value(607), s.ad_value(396)), 1.0), (-1e-6))), ((4.0 * 0.001) * 0.001)))), 0.5)
    } else {
        {
            if (((1.0 + (s.v[607] * s.v[396])) - 1e-6) < ((-10000.0) * 0.001)) {
                A::div_from_scalar(((-0.001) * 0.001), A::offset(A::offset(A::mul(s.ad_value(607), s.ad_value(396)), 1.0), (-1e-6)))
            } else {
                A::constant(0.0)
            }
        }
    }
};
        s.store_mul_ad_rhs(609, 605, assign13660_ad_e19278);

        let assign13670_ad_e19352: A = {
    if (!(((1.0 + (s.v[608] * s.v[396])) - 1e-6) < ((-10000.0) * 0.001))) {
        A::scale(A::add(A::offset(A::offset(A::mul(s.ad_value(608), s.ad_value(396)), 1.0), (-1e-6)), A::sqrt(A::offset(A::mul(A::offset(A::offset(A::mul(s.ad_value(608), s.ad_value(396)), 1.0), (-1e-6)), A::offset(A::offset(A::mul(s.ad_value(608), s.ad_value(396)), 1.0), (-1e-6))), ((4.0 * 0.001) * 0.001)))), 0.5)
    } else {
        {
            if (((1.0 + (s.v[608] * s.v[396])) - 1e-6) < ((-10000.0) * 0.001)) {
                A::div_from_scalar(((-0.001) * 0.001), A::offset(A::offset(A::mul(s.ad_value(608), s.ad_value(396)), 1.0), (-1e-6)))
            } else {
                A::constant(0.0)
            }
        }
    }
};
        s.store_mul_ad_rhs(610, 606, assign13670_ad_e19352);

        let assign13680_ad_e19426: A = {
    if (!(((1.0 + (s.v[632] * s.v[396])) - 1e-6) < ((-10000.0) * 0.001))) {
        A::scale(A::add(A::offset(A::offset(A::mul(s.ad_value(632), s.ad_value(396)), 1.0), (-1e-6)), A::sqrt(A::offset(A::mul(A::offset(A::offset(A::mul(s.ad_value(632), s.ad_value(396)), 1.0), (-1e-6)), A::offset(A::offset(A::mul(s.ad_value(632), s.ad_value(396)), 1.0), (-1e-6))), ((4.0 * 0.001) * 0.001)))), 0.5)
    } else {
        {
            if (((1.0 + (s.v[632] * s.v[396])) - 1e-6) < ((-10000.0) * 0.001)) {
                A::div_from_scalar(((-0.001) * 0.001), A::offset(A::offset(A::mul(s.ad_value(632), s.ad_value(396)), 1.0), (-1e-6)))
            } else {
                A::constant(0.0)
            }
        }
    }
};
        s.store_mul_ad_rhs(633, 631, assign13680_ad_e19426);

        let assign13690_ad_e19500: A = {
    if (!(((1.0 + (s.v[635] * s.v[396])) - 1e-6) < ((-10000.0) * 0.001))) {
        A::scale(A::add(A::offset(A::offset(A::mul(s.ad_value(635), s.ad_value(396)), 1.0), (-1e-6)), A::sqrt(A::offset(A::mul(A::offset(A::offset(A::mul(s.ad_value(635), s.ad_value(396)), 1.0), (-1e-6)), A::offset(A::offset(A::mul(s.ad_value(635), s.ad_value(396)), 1.0), (-1e-6))), ((4.0 * 0.001) * 0.001)))), 0.5)
    } else {
        {
            if (((1.0 + (s.v[635] * s.v[396])) - 1e-6) < ((-10000.0) * 0.001)) {
                A::div_from_scalar(((-0.001) * 0.001), A::offset(A::offset(A::mul(s.ad_value(635), s.ad_value(396)), 1.0), (-1e-6)))
            } else {
                A::constant(0.0)
            }
        }
    }
};
        s.store_mul_ad_rhs(636, 634, assign13690_ad_e19500);

        let assign13700_ad_e19574: A = {
    if (!(((1.0 + (s.v[638] * s.v[396])) - 1e-6) < ((-10000.0) * 0.001))) {
        A::scale(A::add(A::offset(A::offset(A::mul(s.ad_value(638), s.ad_value(396)), 1.0), (-1e-6)), A::sqrt(A::offset(A::mul(A::offset(A::offset(A::mul(s.ad_value(638), s.ad_value(396)), 1.0), (-1e-6)), A::offset(A::offset(A::mul(s.ad_value(638), s.ad_value(396)), 1.0), (-1e-6))), ((4.0 * 0.001) * 0.001)))), 0.5)
    } else {
        {
            if (((1.0 + (s.v[638] * s.v[396])) - 1e-6) < ((-10000.0) * 0.001)) {
                A::div_from_scalar(((-0.001) * 0.001), A::offset(A::offset(A::mul(s.ad_value(638), s.ad_value(396)), 1.0), (-1e-6)))
            } else {
                A::constant(0.0)
            }
        }
    }
};
        s.store_mul_ad_rhs(639, 637, assign13700_ad_e19574);

        let assign13710_ad_e19648: A = {
    if (!(((1.0 + (p.p889 * s.v[396])) - 1e-6) < ((-10000.0) * 0.001))) {
        A::scale(A::add(A::offset(A::offset(A::scale(s.ad_value(396), p.p889), 1.0), (-1e-6)), A::sqrt(A::offset(A::mul(A::offset(A::offset(A::scale(s.ad_value(396), p.p889), 1.0), (-1e-6)), A::offset(A::offset(A::scale(s.ad_value(396), p.p889), 1.0), (-1e-6))), ((4.0 * 0.001) * 0.001)))), 0.5)
    } else {
        {
            if (((1.0 + (p.p889 * s.v[396])) - 1e-6) < ((-10000.0) * 0.001)) {
                A::div_from_scalar(((-0.001) * 0.001), A::offset(A::offset(A::scale(s.ad_value(396), p.p889), 1.0), (-1e-6)))
            } else {
                A::constant(0.0)
            }
        }
    }
};
        s.store_scale_ad(423, assign13710_ad_e19648, p.p701);

        let assign13720_ad_e19722: A = {
    if (!(((1.0 + (p.p889 * s.v[396])) - 1e-6) < ((-10000.0) * 0.001))) {
        A::scale(A::add(A::offset(A::offset(A::scale(s.ad_value(396), p.p889), 1.0), (-1e-6)), A::sqrt(A::offset(A::mul(A::offset(A::offset(A::scale(s.ad_value(396), p.p889), 1.0), (-1e-6)), A::offset(A::offset(A::scale(s.ad_value(396), p.p889), 1.0), (-1e-6))), ((4.0 * 0.001) * 0.001)))), 0.5)
    } else {
        {
            if (((1.0 + (p.p889 * s.v[396])) - 1e-6) < ((-10000.0) * 0.001)) {
                A::div_from_scalar(((-0.001) * 0.001), A::offset(A::offset(A::scale(s.ad_value(396), p.p889), 1.0), (-1e-6)))
            } else {
                A::constant(0.0)
            }
        }
    }
};
        s.store_scale_ad(426, assign13720_ad_e19722, p.p702);

        let assign13730_ad_e19796: A = {
    if (!(((1.0 + (p.p890 * s.v[396])) - 1e-6) < ((-10000.0) * 0.001))) {
        A::scale(A::add(A::offset(A::offset(A::scale(s.ad_value(396), p.p890), 1.0), (-1e-6)), A::sqrt(A::offset(A::mul(A::offset(A::offset(A::scale(s.ad_value(396), p.p890), 1.0), (-1e-6)), A::offset(A::offset(A::scale(s.ad_value(396), p.p890), 1.0), (-1e-6))), ((4.0 * 0.001) * 0.001)))), 0.5)
    } else {
        {
            if (((1.0 + (p.p890 * s.v[396])) - 1e-6) < ((-10000.0) * 0.001)) {
                A::div_from_scalar(((-0.001) * 0.001), A::offset(A::offset(A::scale(s.ad_value(396), p.p890), 1.0), (-1e-6)))
            } else {
                A::constant(0.0)
            }
        }
    }
};
        s.store_scale_ad(424, assign13730_ad_e19796, p.p703);

        let assign13740_ad_e19870: A = {
    if (!(((1.0 + (p.p890 * s.v[396])) - 1e-6) < ((-10000.0) * 0.001))) {
        A::scale(A::add(A::offset(A::offset(A::scale(s.ad_value(396), p.p890), 1.0), (-1e-6)), A::sqrt(A::offset(A::mul(A::offset(A::offset(A::scale(s.ad_value(396), p.p890), 1.0), (-1e-6)), A::offset(A::offset(A::scale(s.ad_value(396), p.p890), 1.0), (-1e-6))), ((4.0 * 0.001) * 0.001)))), 0.5)
    } else {
        {
            if (((1.0 + (p.p890 * s.v[396])) - 1e-6) < ((-10000.0) * 0.001)) {
                A::div_from_scalar(((-0.001) * 0.001), A::offset(A::offset(A::scale(s.ad_value(396), p.p890), 1.0), (-1e-6)))
            } else {
                A::constant(0.0)
            }
        }
    }
};
        s.store_scale_ad(427, assign13740_ad_e19870, p.p704);

        let assign13750_ad_e19944: A = {
    if (!(((1.0 + (p.p891 * s.v[396])) - 1e-6) < ((-10000.0) * 0.001))) {
        A::scale(A::add(A::offset(A::offset(A::scale(s.ad_value(396), p.p891), 1.0), (-1e-6)), A::sqrt(A::offset(A::mul(A::offset(A::offset(A::scale(s.ad_value(396), p.p891), 1.0), (-1e-6)), A::offset(A::offset(A::scale(s.ad_value(396), p.p891), 1.0), (-1e-6))), ((4.0 * 0.001) * 0.001)))), 0.5)
    } else {
        {
            if (((1.0 + (p.p891 * s.v[396])) - 1e-6) < ((-10000.0) * 0.001)) {
                A::div_from_scalar(((-0.001) * 0.001), A::offset(A::offset(A::scale(s.ad_value(396), p.p891), 1.0), (-1e-6)))
            } else {
                A::constant(0.0)
            }
        }
    }
};
        s.store_scale_ad(428, assign13750_ad_e19944, p.p705);

        let assign13760_ad_e20018: A = {
    if (!(((1.0 + (p.p891 * s.v[396])) - 1e-6) < ((-10000.0) * 0.001))) {
        A::scale(A::add(A::offset(A::offset(A::scale(s.ad_value(396), p.p891), 1.0), (-1e-6)), A::sqrt(A::offset(A::mul(A::offset(A::offset(A::scale(s.ad_value(396), p.p891), 1.0), (-1e-6)), A::offset(A::offset(A::scale(s.ad_value(396), p.p891), 1.0), (-1e-6))), ((4.0 * 0.001) * 0.001)))), 0.5)
    } else {
        {
            if (((1.0 + (p.p891 * s.v[396])) - 1e-6) < ((-10000.0) * 0.001)) {
                A::div_from_scalar(((-0.001) * 0.001), A::offset(A::offset(A::scale(s.ad_value(396), p.p891), 1.0), (-1e-6)))
            } else {
                A::constant(0.0)
            }
        }
    }
};
        s.store_scale_ad(425, assign13760_ad_e20018, p.p706);

    }

    pub(super) fn stamp_reactive_block_7(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let assign13770_ad_e20091: A = {
    if (!(((p.p707 - (p.p892 * s.v[396])) - 0.01) < ((-10000.0) * 0.001))) {
        A::scale(A::add(A::offset(A::sub_from_scalar(p.p707, A::scale(s.ad_value(396), p.p892)), (-0.01)), A::sqrt(A::offset(A::mul(A::offset(A::sub_from_scalar(p.p707, A::scale(s.ad_value(396), p.p892)), (-0.01)), A::offset(A::sub_from_scalar(p.p707, A::scale(s.ad_value(396), p.p892)), (-0.01))), ((4.0 * 0.001) * 0.001)))), 0.5)
    } else {
        {
            if (((p.p707 - (p.p892 * s.v[396])) - 0.01) < ((-10000.0) * 0.001)) {
                A::div_from_scalar(((-0.001) * 0.001), A::offset(A::sub_from_scalar(p.p707, A::scale(s.ad_value(396), p.p892)), (-0.01)))
            } else {
                A::constant(0.0)
            }
        }
    }
};
        s.store_offset_ad(429, assign13770_ad_e20091, 0.01);

        let assign13780_ad_e20165: A = {
    if (!(((p.p708 - (p.p892 * s.v[396])) - 0.01) < ((-10000.0) * 0.001))) {
        A::scale(A::add(A::offset(A::sub_from_scalar(p.p708, A::scale(s.ad_value(396), p.p892)), (-0.01)), A::sqrt(A::offset(A::mul(A::offset(A::sub_from_scalar(p.p708, A::scale(s.ad_value(396), p.p892)), (-0.01)), A::offset(A::sub_from_scalar(p.p708, A::scale(s.ad_value(396), p.p892)), (-0.01))), ((4.0 * 0.001) * 0.001)))), 0.5)
    } else {
        {
            if (((p.p708 - (p.p892 * s.v[396])) - 0.01) < ((-10000.0) * 0.001)) {
                A::div_from_scalar(((-0.001) * 0.001), A::offset(A::sub_from_scalar(p.p708, A::scale(s.ad_value(396), p.p892)), (-0.01)))
            } else {
                A::constant(0.0)
            }
        }
    }
};
        s.store_offset_ad(432, assign13780_ad_e20165, 0.01);

        let assign13790_ad_e20239: A = {
    if (!(((p.p709 - (p.p893 * s.v[396])) - 0.01) < ((-10000.0) * 0.001))) {
        A::scale(A::add(A::offset(A::sub_from_scalar(p.p709, A::scale(s.ad_value(396), p.p893)), (-0.01)), A::sqrt(A::offset(A::mul(A::offset(A::sub_from_scalar(p.p709, A::scale(s.ad_value(396), p.p893)), (-0.01)), A::offset(A::sub_from_scalar(p.p709, A::scale(s.ad_value(396), p.p893)), (-0.01))), ((4.0 * 0.001) * 0.001)))), 0.5)
    } else {
        {
            if (((p.p709 - (p.p893 * s.v[396])) - 0.01) < ((-10000.0) * 0.001)) {
                A::div_from_scalar(((-0.001) * 0.001), A::offset(A::sub_from_scalar(p.p709, A::scale(s.ad_value(396), p.p893)), (-0.01)))
            } else {
                A::constant(0.0)
            }
        }
    }
};
        s.store_offset_ad(430, assign13790_ad_e20239, 0.01);

        let assign13800_ad_e20313: A = {
    if (!(((p.p710 - (p.p893 * s.v[396])) - 0.01) < ((-10000.0) * 0.001))) {
        A::scale(A::add(A::offset(A::sub_from_scalar(p.p710, A::scale(s.ad_value(396), p.p893)), (-0.01)), A::sqrt(A::offset(A::mul(A::offset(A::sub_from_scalar(p.p710, A::scale(s.ad_value(396), p.p893)), (-0.01)), A::offset(A::sub_from_scalar(p.p710, A::scale(s.ad_value(396), p.p893)), (-0.01))), ((4.0 * 0.001) * 0.001)))), 0.5)
    } else {
        {
            if (((p.p710 - (p.p893 * s.v[396])) - 0.01) < ((-10000.0) * 0.001)) {
                A::div_from_scalar(((-0.001) * 0.001), A::offset(A::sub_from_scalar(p.p710, A::scale(s.ad_value(396), p.p893)), (-0.01)))
            } else {
                A::constant(0.0)
            }
        }
    }
};
        s.store_offset_ad(433, assign13800_ad_e20313, 0.01);

        let assign13810_ad_e20387: A = {
    if (!(((p.p711 - (p.p894 * s.v[396])) - 0.01) < ((-10000.0) * 0.001))) {
        A::scale(A::add(A::offset(A::sub_from_scalar(p.p711, A::scale(s.ad_value(396), p.p894)), (-0.01)), A::sqrt(A::offset(A::mul(A::offset(A::sub_from_scalar(p.p711, A::scale(s.ad_value(396), p.p894)), (-0.01)), A::offset(A::sub_from_scalar(p.p711, A::scale(s.ad_value(396), p.p894)), (-0.01))), ((4.0 * 0.001) * 0.001)))), 0.5)
    } else {
        {
            if (((p.p711 - (p.p894 * s.v[396])) - 0.01) < ((-10000.0) * 0.001)) {
                A::div_from_scalar(((-0.001) * 0.001), A::offset(A::sub_from_scalar(p.p711, A::scale(s.ad_value(396), p.p894)), (-0.01)))
            } else {
                A::constant(0.0)
            }
        }
    }
};
        s.store_offset_ad(431, assign13810_ad_e20387, 0.01);

        let assign13820_ad_e20461: A = {
    if (!(((p.p712 - (p.p894 * s.v[396])) - 0.01) < ((-10000.0) * 0.001))) {
        A::scale(A::add(A::offset(A::sub_from_scalar(p.p712, A::scale(s.ad_value(396), p.p894)), (-0.01)), A::sqrt(A::offset(A::mul(A::offset(A::sub_from_scalar(p.p712, A::scale(s.ad_value(396), p.p894)), (-0.01)), A::offset(A::sub_from_scalar(p.p712, A::scale(s.ad_value(396), p.p894)), (-0.01))), ((4.0 * 0.001) * 0.001)))), 0.5)
    } else {
        {
            if (((p.p712 - (p.p894 * s.v[396])) - 0.01) < ((-10000.0) * 0.001)) {
                A::div_from_scalar(((-0.001) * 0.001), A::offset(A::sub_from_scalar(p.p712, A::scale(s.ad_value(396), p.p894)), (-0.01)))
            } else {
                A::constant(0.0)
            }
        }
    }
};
        s.store_offset_ad(434, assign13820_ad_e20461, 0.01);

        s.store_sub_ad(12, A::div(s.ad_value(37), s.ad_value(394)), A::div(s.ad_value(36), s.ad_value(393)));

        s.store_ln_ad(13, A::max_with_scalar(s.ad_value(395), 1e-38));

        s.store_limited_exp_ad(15, A::scale(A::add(s.ad_value(12), A::scale(s.ad_value(13), p.p895)), 1.0 / (p.p725)));

        s.store_scale(435, 15, p.p719);

        s.store_scale(436, 15, p.p721);

        s.store_scale(437, 15, p.p723);

        s.store_limited_exp_ad(15, A::scale(A::add(s.ad_value(12), A::scale(s.ad_value(13), p.p896)), 1.0 / (p.p726)));

        s.store_scale(438, 15, p.p720);

        s.store_scale(439, 15, p.p722);

        s.store_scale(440, 15, p.p724);

        s.store_scale_ad(441, A::limited_exp(A::div(A::mul(A::scale(s.ad_value(37), p.p897), A::offset(s.ad_value(395), (-1.0))), s.ad_value(393))), p.p735);

        s.store_scale_ad(443, A::limited_exp(A::div(A::mul(A::scale(s.ad_value(37), p.p899), A::offset(s.ad_value(395), (-1.0))), s.ad_value(393))), p.p737);

        s.store_scale_ad(445, A::limited_exp(A::div(A::mul(A::scale(s.ad_value(37), p.p901), A::offset(s.ad_value(395), (-1.0))), s.ad_value(393))), (p.p739 * ((((p.p741 / s.v[35])) as f64).sqrt() + 1.0)));

        s.store_scale_ad(442, A::limited_exp(A::div(A::mul(A::scale(s.ad_value(37), p.p898), A::offset(s.ad_value(395), (-1.0))), s.ad_value(393))), p.p736);

        s.store_scale_ad(444, A::limited_exp(A::div(A::mul(A::scale(s.ad_value(37), p.p900), A::offset(s.ad_value(395), (-1.0))), s.ad_value(393))), p.p738);

        s.store_scale_ad(446, A::limited_exp(A::div(A::mul(A::scale(s.ad_value(37), p.p902), A::offset(s.ad_value(395), (-1.0))), s.ad_value(393))), (p.p740 * ((((p.p741 / s.v[35])) as f64).sqrt() + 1.0)));

        let assign13990_ad_e20690: A = {
    if (!(((p.p742 * (1.0 + (p.p903 * (s.v[395] - 1.0)))) - 0.01) < ((-10000.0) * 0.001))) {
        let assign13990_ad_e20654: A = A::add(A::offset(A::scale(A::offset(A::scale(A::offset(s.ad_value(395), (-1.0)), p.p903), 1.0), p.p742), (-0.01)), A::sqrt(A::offset(A::mul(A::offset(A::scale(A::offset(A::scale(A::offset(s.ad_value(395), (-1.0)), p.p903), 1.0), p.p742), (-0.01)), A::offset(A::scale(A::offset(A::scale(A::offset(s.ad_value(395), (-1.0)), p.p903), 1.0), p.p742), (-0.01))), ((4.0 * 0.001) * 0.001))));
        A::scale(assign13990_ad_e20654, 0.5)
    } else {
        {
            if (((p.p742 * (1.0 + (p.p903 * (s.v[395] - 1.0)))) - 0.01) < ((-10000.0) * 0.001)) {
                A::div_from_scalar(((-0.001) * 0.001), A::offset(A::scale(A::offset(A::scale(A::offset(s.ad_value(395), (-1.0)), p.p903), 1.0), p.p742), (-0.01)))
            } else {
                A::constant(0.0)
            }
        }
    }
};
        s.store_offset_ad(447, assign13990_ad_e20690, 0.01);

        let assign14000_ad_e20788: A = {
    if (!(((p.p744 * (1.0 + (p.p905 * (s.v[395] - 1.0)))) - 0.01) < ((-10000.0) * 0.001))) {
        let assign14000_ad_e20752: A = A::add(A::offset(A::scale(A::offset(A::scale(A::offset(s.ad_value(395), (-1.0)), p.p905), 1.0), p.p744), (-0.01)), A::sqrt(A::offset(A::mul(A::offset(A::scale(A::offset(A::scale(A::offset(s.ad_value(395), (-1.0)), p.p905), 1.0), p.p744), (-0.01)), A::offset(A::scale(A::offset(A::scale(A::offset(s.ad_value(395), (-1.0)), p.p905), 1.0), p.p744), (-0.01))), ((4.0 * 0.001) * 0.001))));
        A::scale(assign14000_ad_e20752, 0.5)
    } else {
        {
            if (((p.p744 * (1.0 + (p.p905 * (s.v[395] - 1.0)))) - 0.01) < ((-10000.0) * 0.001)) {
                A::div_from_scalar(((-0.001) * 0.001), A::offset(A::scale(A::offset(A::scale(A::offset(s.ad_value(395), (-1.0)), p.p905), 1.0), p.p744), (-0.01)))
            } else {
                A::constant(0.0)
            }
        }
    }
};
        s.store_offset_ad(449, assign14000_ad_e20788, 0.01);

        let assign14010_ad_e20886: A = {
    if (!(((p.p746 * (1.0 + (p.p907 * (s.v[395] - 1.0)))) - 0.01) < ((-10000.0) * 0.001))) {
        let assign14010_ad_e20850: A = A::add(A::offset(A::scale(A::offset(A::scale(A::offset(s.ad_value(395), (-1.0)), p.p907), 1.0), p.p746), (-0.01)), A::sqrt(A::offset(A::mul(A::offset(A::scale(A::offset(A::scale(A::offset(s.ad_value(395), (-1.0)), p.p907), 1.0), p.p746), (-0.01)), A::offset(A::scale(A::offset(A::scale(A::offset(s.ad_value(395), (-1.0)), p.p907), 1.0), p.p746), (-0.01))), ((4.0 * 0.001) * 0.001))));
        A::scale(assign14010_ad_e20850, 0.5)
    } else {
        {
            if (((p.p746 * (1.0 + (p.p907 * (s.v[395] - 1.0)))) - 0.01) < ((-10000.0) * 0.001)) {
                A::div_from_scalar(((-0.001) * 0.001), A::offset(A::scale(A::offset(A::scale(A::offset(s.ad_value(395), (-1.0)), p.p907), 1.0), p.p746), (-0.01)))
            } else {
                A::constant(0.0)
            }
        }
    }
};
        s.store_offset_ad(451, assign14010_ad_e20886, 0.01);

        let assign14020_ad_e20984: A = {
    if (!(((p.p743 * (1.0 + (p.p904 * (s.v[395] - 1.0)))) - 0.01) < ((-10000.0) * 0.001))) {
        let assign14020_ad_e20948: A = A::add(A::offset(A::scale(A::offset(A::scale(A::offset(s.ad_value(395), (-1.0)), p.p904), 1.0), p.p743), (-0.01)), A::sqrt(A::offset(A::mul(A::offset(A::scale(A::offset(A::scale(A::offset(s.ad_value(395), (-1.0)), p.p904), 1.0), p.p743), (-0.01)), A::offset(A::scale(A::offset(A::scale(A::offset(s.ad_value(395), (-1.0)), p.p904), 1.0), p.p743), (-0.01))), ((4.0 * 0.001) * 0.001))));
        A::scale(assign14020_ad_e20948, 0.5)
    } else {
        {
            if (((p.p743 * (1.0 + (p.p904 * (s.v[395] - 1.0)))) - 0.01) < ((-10000.0) * 0.001)) {
                A::div_from_scalar(((-0.001) * 0.001), A::offset(A::scale(A::offset(A::scale(A::offset(s.ad_value(395), (-1.0)), p.p904), 1.0), p.p743), (-0.01)))
            } else {
                A::constant(0.0)
            }
        }
    }
};
        s.store_offset_ad(448, assign14020_ad_e20984, 0.01);

        let assign14030_ad_e21082: A = {
    if (!(((p.p745 * (1.0 + (p.p906 * (s.v[395] - 1.0)))) - 0.01) < ((-10000.0) * 0.001))) {
        let assign14030_ad_e21046: A = A::add(A::offset(A::scale(A::offset(A::scale(A::offset(s.ad_value(395), (-1.0)), p.p906), 1.0), p.p745), (-0.01)), A::sqrt(A::offset(A::mul(A::offset(A::scale(A::offset(A::scale(A::offset(s.ad_value(395), (-1.0)), p.p906), 1.0), p.p745), (-0.01)), A::offset(A::scale(A::offset(A::scale(A::offset(s.ad_value(395), (-1.0)), p.p906), 1.0), p.p745), (-0.01))), ((4.0 * 0.001) * 0.001))));
        A::scale(assign14030_ad_e21046, 0.5)
    } else {
        {
            if (((p.p745 * (1.0 + (p.p906 * (s.v[395] - 1.0)))) - 0.01) < ((-10000.0) * 0.001)) {
                A::div_from_scalar(((-0.001) * 0.001), A::offset(A::scale(A::offset(A::scale(A::offset(s.ad_value(395), (-1.0)), p.p906), 1.0), p.p745), (-0.01)))
            } else {
                A::constant(0.0)
            }
        }
    }
};
        s.store_offset_ad(450, assign14030_ad_e21082, 0.01);

        let assign14040_ad_e21180: A = {
    if (!(((p.p747 * (1.0 + (p.p908 * (s.v[395] - 1.0)))) - 0.01) < ((-10000.0) * 0.001))) {
        let assign14040_ad_e21144: A = A::add(A::offset(A::scale(A::offset(A::scale(A::offset(s.ad_value(395), (-1.0)), p.p908), 1.0), p.p747), (-0.01)), A::sqrt(A::offset(A::mul(A::offset(A::scale(A::offset(A::scale(A::offset(s.ad_value(395), (-1.0)), p.p908), 1.0), p.p747), (-0.01)), A::offset(A::scale(A::offset(A::scale(A::offset(s.ad_value(395), (-1.0)), p.p908), 1.0), p.p747), (-0.01))), ((4.0 * 0.001) * 0.001))));
        A::scale(assign14040_ad_e21144, 0.5)
    } else {
        {
            if (((p.p747 * (1.0 + (p.p908 * (s.v[395] - 1.0)))) - 0.01) < ((-10000.0) * 0.001)) {
                A::div_from_scalar(((-0.001) * 0.001), A::offset(A::scale(A::offset(A::scale(A::offset(s.ad_value(395), (-1.0)), p.p908), 1.0), p.p747), (-0.01)))
            } else {
                A::constant(0.0)
            }
        }
    }
};
        s.store_offset_ad(452, assign14040_ad_e21180, 0.01);

        s.v[1320] = if (p.p9 < 9.0) { 1.0 } else { 0.0 };

        s.v[1321] = if ((p.p2 % 2.0) != 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1320] != 0.0) && (s.v[1321] != 0.0)) {
            s.store_scalar(701, 1.0);
        }

        if ((s.v[1320] != 0.0) && (s.v[1321] != 0.0)) {
            s.store_scalar(703, 1.0);
        }

        if ((s.v[1320] != 0.0) && (s.v[1321] != 0.0)) {
            s.store_scalar(700, (2.0 * (((p.p2 - 1.0) / 2.0)).max(0.0)));
        }

        if ((s.v[1320] != 0.0) && (s.v[1321] != 0.0)) {
            s.copy_ad(702, 700);
        }

        s.v[1322] = if (p.p6 == 1.0) { 1.0 } else { 0.0 };

        if (((s.v[1320] != 0.0) && (!(s.v[1321] != 0.0))) && (s.v[1322] != 0.0)) {
            s.store_scalar(701, 2.0);
        }

        if (((s.v[1320] != 0.0) && (!(s.v[1321] != 0.0))) && (s.v[1322] != 0.0)) {
            s.store_scalar(700, (2.0 * (((p.p2 / 2.0) - 1.0)).max(0.0)));
        }

        if (((s.v[1320] != 0.0) && (!(s.v[1321] != 0.0))) && (s.v[1322] != 0.0)) {
            s.store_scalar(703, 0.0);
        }

        if (((s.v[1320] != 0.0) && (!(s.v[1321] != 0.0))) && (s.v[1322] != 0.0)) {
            s.store_scalar(702, p.p2);
        }

        if (((s.v[1320] != 0.0) && (!(s.v[1321] != 0.0))) && (!(s.v[1322] != 0.0))) {
            s.store_scalar(701, 0.0);
        }

        if (((s.v[1320] != 0.0) && (!(s.v[1321] != 0.0))) && (!(s.v[1322] != 0.0))) {
            s.store_scalar(700, p.p2);
        }

        if (((s.v[1320] != 0.0) && (!(s.v[1321] != 0.0))) && (!(s.v[1322] != 0.0))) {
            s.store_scalar(703, 2.0);
        }

        if (((s.v[1320] != 0.0) && (!(s.v[1321] != 0.0))) && (!(s.v[1322] != 0.0))) {
            s.store_scalar(702, (2.0 * (((p.p2 / 2.0) - 1.0)).max(0.0)));
        }

        s.v[12] = (s.v[236] + s.v[238]);

        s.v[13] = (s.v[236] + s.v[236]);

        s.v[14] = (s.v[237] + s.v[237]);

        s.v[0] = ((s.v[12] + s.v[12]) + s.v[35]);

        s.v[1] = ((s.v[12] + s.v[12]) + s.v[35]);

        s.v[2] = s.v[13];

        s.v[3] = s.v[13];

        s.v[4] = s.v[14];

        s.v[5] = s.v[14];

        s.v[6] = (s.v[12] * s.v[35]);

        s.v[7] = (s.v[12] * s.v[35]);

        s.v[8] = (s.v[236] * s.v[35]);

        s.v[9] = (s.v[236] * s.v[35]);

        s.v[10] = (s.v[237] * s.v[35]);

        s.v[11] = (s.v[237] * s.v[35]);

        s.v[1323] = if (p.p9 == 0.0) { 1.0 } else { 0.0 };

        s.v[1324] = if (p.p9 == 1.0) { 1.0 } else { 0.0 };

        s.v[1325] = if (p.p9 == 2.0) { 1.0 } else { 0.0 };

        s.v[1326] = if (p.p9 == 3.0) { 1.0 } else { 0.0 };

        s.v[1327] = if (p.p9 == 4.0) { 1.0 } else { 0.0 };

        s.v[1328] = if (p.p9 == 5.0) { 1.0 } else { 0.0 };

        s.v[1329] = if (p.p9 == 6.0) { 1.0 } else { 0.0 };

        s.v[1330] = if (p.p9 == 7.0) { 1.0 } else { 0.0 };

        s.v[1331] = if (p.p9 == 8.0) { 1.0 } else { 0.0 };

        s.v[1332] = if (p.p9 == 9.0) { 1.0 } else { 0.0 };

        s.v[1333] = if (p.p9 == 10.0) { 1.0 } else { 0.0 };

        if (s.v[1323] != 0.0) {
            s.store_add_ad(248, A::scale(s.ad_value(703), s.v[0]), A::scale(s.ad_value(702), s.v[2]));
        }

        if (s.v[1323] != 0.0) {
            s.store_add_ad(249, A::scale(s.ad_value(701), s.v[1]), A::scale(s.ad_value(700), s.v[3]));
        }

        if (s.v[1323] != 0.0) {
            s.store_add_ad(246, A::scale(s.ad_value(703), s.v[6]), A::scale(s.ad_value(702), s.v[8]));
        }

        if (s.v[1323] != 0.0) {
            s.store_add_ad(247, A::scale(s.ad_value(701), s.v[7]), A::scale(s.ad_value(700), s.v[9]));
        }

        if ((s.v[1324] != 0.0) && (!(s.v[1323] != 0.0))) {
            s.store_add_ad(248, A::scale(s.ad_value(703), s.v[0]), A::scale(s.ad_value(702), s.v[2]));
        }

        if ((s.v[1324] != 0.0) && (!(s.v[1323] != 0.0))) {
            s.store_scaled_add(249, 701, 700, s.v[3]);
        }

        if ((s.v[1324] != 0.0) && (!(s.v[1323] != 0.0))) {
            s.store_add_ad(246, A::scale(s.ad_value(703), s.v[6]), A::scale(s.ad_value(702), s.v[8]));
        }

        if ((s.v[1324] != 0.0) && (!(s.v[1323] != 0.0))) {
            s.store_scaled_add(247, 701, 700, s.v[9]);
        }

        if ((s.v[1325] != 0.0) && (!((s.v[1323] != 0.0) || (s.v[1324] != 0.0)))) {
            s.store_scaled_add(248, 703, 702, s.v[2]);
        }

        if ((s.v[1325] != 0.0) && (!((s.v[1323] != 0.0) || (s.v[1324] != 0.0)))) {
            s.store_add_ad(249, A::scale(s.ad_value(701), s.v[1]), A::scale(s.ad_value(700), s.v[3]));
        }

        if ((s.v[1325] != 0.0) && (!((s.v[1323] != 0.0) || (s.v[1324] != 0.0)))) {
            s.store_scaled_add(246, 703, 702, s.v[8]);
        }

        if ((s.v[1325] != 0.0) && (!((s.v[1323] != 0.0) || (s.v[1324] != 0.0)))) {
            s.store_add_ad(247, A::scale(s.ad_value(701), s.v[7]), A::scale(s.ad_value(700), s.v[9]));
        }

        if ((s.v[1326] != 0.0) && (!(((s.v[1323] != 0.0) || (s.v[1324] != 0.0)) || (s.v[1325] != 0.0)))) {
            s.store_scaled_add(248, 703, 702, s.v[2]);
        }

        if ((s.v[1326] != 0.0) && (!(((s.v[1323] != 0.0) || (s.v[1324] != 0.0)) || (s.v[1325] != 0.0)))) {
            s.store_scaled_add(249, 701, 700, s.v[3]);
        }

        if ((s.v[1326] != 0.0) && (!(((s.v[1323] != 0.0) || (s.v[1324] != 0.0)) || (s.v[1325] != 0.0)))) {
            s.store_scaled_add(246, 703, 702, s.v[8]);
        }

        if ((s.v[1326] != 0.0) && (!(((s.v[1323] != 0.0) || (s.v[1324] != 0.0)) || (s.v[1325] != 0.0)))) {
            s.store_scaled_add(247, 701, 700, s.v[9]);
        }

        if ((s.v[1327] != 0.0) && (!((((s.v[1323] != 0.0) || (s.v[1324] != 0.0)) || (s.v[1325] != 0.0)) || (s.v[1326] != 0.0)))) {
            s.store_add_ad(248, A::scale(s.ad_value(703), s.v[0]), A::scale(s.ad_value(702), s.v[2]));
        }

        if ((s.v[1327] != 0.0) && (!((((s.v[1323] != 0.0) || (s.v[1324] != 0.0)) || (s.v[1325] != 0.0)) || (s.v[1326] != 0.0)))) {
            s.store_add_ad(249, A::scale(s.ad_value(701), s.v[5]), A::scale(s.ad_value(700), s.v[3]));
        }

        if ((s.v[1327] != 0.0) && (!((((s.v[1323] != 0.0) || (s.v[1324] != 0.0)) || (s.v[1325] != 0.0)) || (s.v[1326] != 0.0)))) {
            s.store_add_ad(246, A::scale(s.ad_value(703), s.v[6]), A::scale(s.ad_value(702), s.v[8]));
        }

        if ((s.v[1327] != 0.0) && (!((((s.v[1323] != 0.0) || (s.v[1324] != 0.0)) || (s.v[1325] != 0.0)) || (s.v[1326] != 0.0)))) {
            s.store_add_ad(247, A::scale(s.ad_value(701), s.v[11]), A::scale(s.ad_value(700), s.v[9]));
        }

        if ((s.v[1328] != 0.0) && (!(((((s.v[1323] != 0.0) || (s.v[1324] != 0.0)) || (s.v[1325] != 0.0)) || (s.v[1326] != 0.0)) || (s.v[1327] != 0.0)))) {
            s.store_scaled_add(248, 703, 702, s.v[2]);
        }

        if ((s.v[1328] != 0.0) && (!(((((s.v[1323] != 0.0) || (s.v[1324] != 0.0)) || (s.v[1325] != 0.0)) || (s.v[1326] != 0.0)) || (s.v[1327] != 0.0)))) {
            s.store_add_ad(249, A::scale(s.ad_value(701), s.v[5]), A::scale(s.ad_value(700), s.v[3]));
        }

        if ((s.v[1328] != 0.0) && (!(((((s.v[1323] != 0.0) || (s.v[1324] != 0.0)) || (s.v[1325] != 0.0)) || (s.v[1326] != 0.0)) || (s.v[1327] != 0.0)))) {
            s.store_scaled_add(246, 703, 702, s.v[8]);
        }

        if ((s.v[1328] != 0.0) && (!(((((s.v[1323] != 0.0) || (s.v[1324] != 0.0)) || (s.v[1325] != 0.0)) || (s.v[1326] != 0.0)) || (s.v[1327] != 0.0)))) {
            s.store_add_ad(247, A::scale(s.ad_value(701), s.v[11]), A::scale(s.ad_value(700), s.v[9]));
        }

        if ((s.v[1329] != 0.0) && (!((((((s.v[1323] != 0.0) || (s.v[1324] != 0.0)) || (s.v[1325] != 0.0)) || (s.v[1326] != 0.0)) || (s.v[1327] != 0.0)) || (s.v[1328] != 0.0)))) {
            s.store_add_ad(248, A::scale(s.ad_value(703), s.v[4]), A::scale(s.ad_value(702), s.v[2]));
        }

        if ((s.v[1329] != 0.0) && (!((((((s.v[1323] != 0.0) || (s.v[1324] != 0.0)) || (s.v[1325] != 0.0)) || (s.v[1326] != 0.0)) || (s.v[1327] != 0.0)) || (s.v[1328] != 0.0)))) {
            s.store_add_ad(249, A::scale(s.ad_value(701), s.v[1]), A::scale(s.ad_value(700), s.v[3]));
        }

        if ((s.v[1329] != 0.0) && (!((((((s.v[1323] != 0.0) || (s.v[1324] != 0.0)) || (s.v[1325] != 0.0)) || (s.v[1326] != 0.0)) || (s.v[1327] != 0.0)) || (s.v[1328] != 0.0)))) {
            s.store_add_ad(246, A::scale(s.ad_value(703), s.v[10]), A::scale(s.ad_value(702), s.v[8]));
        }

        if ((s.v[1329] != 0.0) && (!((((((s.v[1323] != 0.0) || (s.v[1324] != 0.0)) || (s.v[1325] != 0.0)) || (s.v[1326] != 0.0)) || (s.v[1327] != 0.0)) || (s.v[1328] != 0.0)))) {
            s.store_add_ad(247, A::scale(s.ad_value(701), s.v[7]), A::scale(s.ad_value(700), s.v[9]));
        }

        if ((s.v[1330] != 0.0) && (!(((((((s.v[1323] != 0.0) || (s.v[1324] != 0.0)) || (s.v[1325] != 0.0)) || (s.v[1326] != 0.0)) || (s.v[1327] != 0.0)) || (s.v[1328] != 0.0)) || (s.v[1329] != 0.0)))) {
            s.store_add_ad(248, A::scale(s.ad_value(703), s.v[4]), A::scale(s.ad_value(702), s.v[2]));
        }

        if ((s.v[1330] != 0.0) && (!(((((((s.v[1323] != 0.0) || (s.v[1324] != 0.0)) || (s.v[1325] != 0.0)) || (s.v[1326] != 0.0)) || (s.v[1327] != 0.0)) || (s.v[1328] != 0.0)) || (s.v[1329] != 0.0)))) {
            s.store_scaled_add(249, 701, 700, s.v[3]);
        }

        if ((s.v[1330] != 0.0) && (!(((((((s.v[1323] != 0.0) || (s.v[1324] != 0.0)) || (s.v[1325] != 0.0)) || (s.v[1326] != 0.0)) || (s.v[1327] != 0.0)) || (s.v[1328] != 0.0)) || (s.v[1329] != 0.0)))) {
            s.store_add_ad(246, A::scale(s.ad_value(703), s.v[10]), A::scale(s.ad_value(702), s.v[8]));
        }

        if ((s.v[1330] != 0.0) && (!(((((((s.v[1323] != 0.0) || (s.v[1324] != 0.0)) || (s.v[1325] != 0.0)) || (s.v[1326] != 0.0)) || (s.v[1327] != 0.0)) || (s.v[1328] != 0.0)) || (s.v[1329] != 0.0)))) {
            s.store_scaled_add(247, 701, 700, s.v[9]);
        }

        if ((s.v[1331] != 0.0) && (!((((((((s.v[1323] != 0.0) || (s.v[1324] != 0.0)) || (s.v[1325] != 0.0)) || (s.v[1326] != 0.0)) || (s.v[1327] != 0.0)) || (s.v[1328] != 0.0)) || (s.v[1329] != 0.0)) || (s.v[1330] != 0.0)))) {
            s.store_add_ad(248, A::scale(s.ad_value(703), s.v[4]), A::scale(s.ad_value(702), s.v[2]));
        }

        if ((s.v[1331] != 0.0) && (!((((((((s.v[1323] != 0.0) || (s.v[1324] != 0.0)) || (s.v[1325] != 0.0)) || (s.v[1326] != 0.0)) || (s.v[1327] != 0.0)) || (s.v[1328] != 0.0)) || (s.v[1329] != 0.0)) || (s.v[1330] != 0.0)))) {
            s.store_add_ad(249, A::scale(s.ad_value(701), s.v[5]), A::scale(s.ad_value(700), s.v[3]));
        }

        if ((s.v[1331] != 0.0) && (!((((((((s.v[1323] != 0.0) || (s.v[1324] != 0.0)) || (s.v[1325] != 0.0)) || (s.v[1326] != 0.0)) || (s.v[1327] != 0.0)) || (s.v[1328] != 0.0)) || (s.v[1329] != 0.0)) || (s.v[1330] != 0.0)))) {
            s.store_add_ad(246, A::scale(s.ad_value(703), s.v[10]), A::scale(s.ad_value(702), s.v[8]));
        }

        if ((s.v[1331] != 0.0) && (!((((((((s.v[1323] != 0.0) || (s.v[1324] != 0.0)) || (s.v[1325] != 0.0)) || (s.v[1326] != 0.0)) || (s.v[1327] != 0.0)) || (s.v[1328] != 0.0)) || (s.v[1329] != 0.0)) || (s.v[1330] != 0.0)))) {
            s.store_add_ad(247, A::scale(s.ad_value(701), s.v[11]), A::scale(s.ad_value(700), s.v[9]));
        }

        if ((s.v[1332] != 0.0) && (!(((((((((s.v[1323] != 0.0) || (s.v[1324] != 0.0)) || (s.v[1325] != 0.0)) || (s.v[1326] != 0.0)) || (s.v[1327] != 0.0)) || (s.v[1328] != 0.0)) || (s.v[1329] != 0.0)) || (s.v[1330] != 0.0)) || (s.v[1331] != 0.0)))) {
            s.store_scalar(248, (s.v[0] + ((p.p2 - 1.0) * s.v[2])));
        }

        if ((s.v[1332] != 0.0) && (!(((((((((s.v[1323] != 0.0) || (s.v[1324] != 0.0)) || (s.v[1325] != 0.0)) || (s.v[1326] != 0.0)) || (s.v[1327] != 0.0)) || (s.v[1328] != 0.0)) || (s.v[1329] != 0.0)) || (s.v[1330] != 0.0)) || (s.v[1331] != 0.0)))) {
            s.store_scalar(249, (p.p2 * s.v[3]));
        }

        if ((s.v[1332] != 0.0) && (!(((((((((s.v[1323] != 0.0) || (s.v[1324] != 0.0)) || (s.v[1325] != 0.0)) || (s.v[1326] != 0.0)) || (s.v[1327] != 0.0)) || (s.v[1328] != 0.0)) || (s.v[1329] != 0.0)) || (s.v[1330] != 0.0)) || (s.v[1331] != 0.0)))) {
            s.store_scalar(246, (s.v[6] + ((p.p2 - 1.0) * s.v[8])));
        }

        if ((s.v[1332] != 0.0) && (!(((((((((s.v[1323] != 0.0) || (s.v[1324] != 0.0)) || (s.v[1325] != 0.0)) || (s.v[1326] != 0.0)) || (s.v[1327] != 0.0)) || (s.v[1328] != 0.0)) || (s.v[1329] != 0.0)) || (s.v[1330] != 0.0)) || (s.v[1331] != 0.0)))) {
            s.store_scalar(247, (p.p2 * s.v[9]));
        }

        if ((s.v[1333] != 0.0) && (!((((((((((s.v[1323] != 0.0) || (s.v[1324] != 0.0)) || (s.v[1325] != 0.0)) || (s.v[1326] != 0.0)) || (s.v[1327] != 0.0)) || (s.v[1328] != 0.0)) || (s.v[1329] != 0.0)) || (s.v[1330] != 0.0)) || (s.v[1331] != 0.0)) || (s.v[1332] != 0.0)))) {
            s.store_scalar(248, (p.p2 * s.v[2]));
        }

        if ((s.v[1333] != 0.0) && (!((((((((((s.v[1323] != 0.0) || (s.v[1324] != 0.0)) || (s.v[1325] != 0.0)) || (s.v[1326] != 0.0)) || (s.v[1327] != 0.0)) || (s.v[1328] != 0.0)) || (s.v[1329] != 0.0)) || (s.v[1330] != 0.0)) || (s.v[1331] != 0.0)) || (s.v[1332] != 0.0)))) {
            s.store_scalar(249, (s.v[1] + ((p.p2 - 1.0) * s.v[3])));
        }

        if ((s.v[1333] != 0.0) && (!((((((((((s.v[1323] != 0.0) || (s.v[1324] != 0.0)) || (s.v[1325] != 0.0)) || (s.v[1326] != 0.0)) || (s.v[1327] != 0.0)) || (s.v[1328] != 0.0)) || (s.v[1329] != 0.0)) || (s.v[1330] != 0.0)) || (s.v[1331] != 0.0)) || (s.v[1332] != 0.0)))) {
            s.store_scalar(246, (p.p2 * s.v[8]));
        }

        if ((s.v[1333] != 0.0) && (!((((((((((s.v[1323] != 0.0) || (s.v[1324] != 0.0)) || (s.v[1325] != 0.0)) || (s.v[1326] != 0.0)) || (s.v[1327] != 0.0)) || (s.v[1328] != 0.0)) || (s.v[1329] != 0.0)) || (s.v[1330] != 0.0)) || (s.v[1331] != 0.0)) || (s.v[1332] != 0.0)))) {
            s.store_scalar(247, (s.v[7] + ((p.p2 - 1.0) * s.v[9])));
        }

        if (!(((((((((((s.v[1323] != 0.0) || (s.v[1324] != 0.0)) || (s.v[1325] != 0.0)) || (s.v[1326] != 0.0)) || (s.v[1327] != 0.0)) || (s.v[1328] != 0.0)) || (s.v[1329] != 0.0)) || (s.v[1330] != 0.0)) || (s.v[1331] != 0.0)) || (s.v[1332] != 0.0)) || (s.v[1333] != 0.0))) {
            s.store_scalar(248, 0.0);
        }

        if (!(((((((((((s.v[1323] != 0.0) || (s.v[1324] != 0.0)) || (s.v[1325] != 0.0)) || (s.v[1326] != 0.0)) || (s.v[1327] != 0.0)) || (s.v[1328] != 0.0)) || (s.v[1329] != 0.0)) || (s.v[1330] != 0.0)) || (s.v[1331] != 0.0)) || (s.v[1332] != 0.0)) || (s.v[1333] != 0.0))) {
            s.store_scalar(249, 0.0);
        }

        if (!(((((((((((s.v[1323] != 0.0) || (s.v[1324] != 0.0)) || (s.v[1325] != 0.0)) || (s.v[1326] != 0.0)) || (s.v[1327] != 0.0)) || (s.v[1328] != 0.0)) || (s.v[1329] != 0.0)) || (s.v[1330] != 0.0)) || (s.v[1331] != 0.0)) || (s.v[1332] != 0.0)) || (s.v[1333] != 0.0))) {
            s.store_scalar(246, 0.0);
        }

    }

    pub(super) fn stamp_reactive_block_8(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if (!(((((((((((s.v[1323] != 0.0) || (s.v[1324] != 0.0)) || (s.v[1325] != 0.0)) || (s.v[1326] != 0.0)) || (s.v[1327] != 0.0)) || (s.v[1328] != 0.0)) || (s.v[1329] != 0.0)) || (s.v[1330] != 0.0)) || (s.v[1331] != 0.0)) || (s.v[1332] != 0.0)) || (s.v[1333] != 0.0))) {
            s.store_scalar(247, 0.0);
        }

        s.v[1334] = if self.param_given[24] { 1.0 } else { 0.0 };

        if (s.v[1334] != 0.0) {
            s.store_scalar(250, ((p.p24 * p.p53) * p.p52));
        }

        if (!(s.v[1334] != 0.0)) {
            s.copy_ad(250, 246);
        }

        s.v[1335] = if (s.v[250] < 0.0) { 1.0 } else { 0.0 };

        if (s.v[1335] != 0.0) {
            s.store_scalar(250, 0.0);
        }

        s.v[1336] = if self.param_given[25] { 1.0 } else { 0.0 };

        if (s.v[1336] != 0.0) {
            s.store_scalar(251, ((p.p25 * p.p53) * p.p52));
        }

        if (!(s.v[1336] != 0.0)) {
            s.copy_ad(251, 247);
        }

        s.v[1337] = if (s.v[251] < 0.0) { 1.0 } else { 0.0 };

        if (s.v[1337] != 0.0) {
            s.store_scalar(251, 0.0);
        }

        s.v[1338] = if self.param_given[26] { 1.0 } else { 0.0 };

        s.v[1339] = if (p.p137 == 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1338] != 0.0) && (s.v[1339] != 0.0)) {
            s.store_scalar(300, (p.p26 * p.p53));
        }

        if ((s.v[1338] != 0.0) && (!(s.v[1339] != 0.0))) {
            s.store_scalar(300, (((p.p26 * p.p53) - (s.v[35] * p.p2))).max(0.0));
        }

        if (!(s.v[1338] != 0.0)) {
            s.copy_ad(300, 248);
        }

        s.v[1340] = if (s.v[300] < 0.0) { 1.0 } else { 0.0 };

        if ((!(s.v[1338] != 0.0)) && (s.v[1340] != 0.0)) {
            s.store_scalar(300, 0.0);
        }

        s.v[1341] = if self.param_given[27] { 1.0 } else { 0.0 };

        s.v[1342] = if (p.p137 == 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1341] != 0.0) && (s.v[1342] != 0.0)) {
            s.store_scalar(301, (p.p27 * p.p53));
        }

        if ((s.v[1341] != 0.0) && (!(s.v[1342] != 0.0))) {
            s.store_scalar(301, (((p.p27 * p.p53) - (s.v[35] * p.p2))).max(0.0));
        }

        if (!(s.v[1341] != 0.0)) {
            s.copy_ad(301, 249);
        }

        s.v[1343] = if (s.v[301] < 0.0) { 1.0 } else { 0.0 };

        if ((!(s.v[1341] != 0.0)) && (s.v[1343] != 0.0)) {
            s.store_scalar(301, 0.0);
        }

        s.store_add_ad(341, A::add(A::mul(s.ad_value(250), s.ad_value(435)), A::mul(s.ad_value(300), s.ad_value(436))), A::scale(s.ad_value(437), (s.v[35] * p.p2)));

        s.v[1344] = if (s.v[341] > 0.0) { 1.0 } else { 0.0 };

        if (s.v[1344] != 0.0) {
            s.store_scale(343, 393, p.p725);
        }

        if (s.v[1344] != 0.0) {
            s.store_scale_ad(351, A::limited_exp(A::div_from_scalar((-p.p731), s.ad_value(343))), p.p733);
        }

        if (s.v[1344] != 0.0) {
            s.store_max_with_scalar_ad(14, A::div_from_scalar(p.p727, s.ad_value(341)), 10.0);
        }

        if (s.v[1344] != 0.0) {
            s.store_sub_ad_lhs(25, A::offset(s.ad_value(14), 1.0), 351);
        }

        if (s.v[1344] != 0.0) {
            s.store_mul_ad_rhs(350, 343, A::ln(A::max_with_scalar(A::scale(A::add(s.ad_value(25), A::sqrt(A::add(A::square(s.ad_value(25)), A::scale(s.ad_value(351), 4.0)))), 0.5), 1e-38)));
        }

        if (s.v[1344] != 0.0) {
            s.store_limited_exp_ad(12, A::div(s.ad_value(350), s.ad_value(343)));
        }

        if (s.v[1344] != 0.0) {
            s.store_mul_ad_rhs(349, 341, A::offset(A::add(A::sub(s.ad_value(12), A::div(s.ad_value(351), s.ad_value(12))), s.ad_value(351)), (-1.0)));
        }

        if (s.v[1344] != 0.0) {
            s.store_div_ad_lhs(348, A::mul(s.ad_value(341), A::add(s.ad_value(12), A::div(s.ad_value(351), s.ad_value(12)))), 343);
        }

        if (s.v[1344] != 0.0) {
            let assign15280_ad_e22663: A = {
                if (!(((p.p729 / s.v[341]) - 10.0) < ((-10000.0) * 0.001))) {
                    A::scale(A::add(A::offset(A::div_from_scalar(p.p729, s.ad_value(341)), (-10.0)), A::sqrt(A::offset(A::mul(A::offset(A::div_from_scalar(p.p729, s.ad_value(341)), (-10.0)), A::offset(A::div_from_scalar(p.p729, s.ad_value(341)), (-10.0))), ((4.0 * 0.001) * 0.001)))), 0.5)
                } else {
                    {
                        if (((p.p729 / s.v[341]) - 10.0) < ((-10000.0) * 0.001)) {
                            A::div_from_scalar(((-0.001) * 0.001), A::offset(A::div_from_scalar(p.p729, s.ad_value(341)), (-10.0)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_offset_ad(14, assign15280_ad_e22663, 10.0);
        }

        if (s.v[1344] != 0.0) {
            s.store_sub_from_scalar_ad(347, (-p.p731), A::mul(s.ad_value(343), A::ln(A::max_with_scalar(A::scale(A::offset(s.ad_value(14), (-1.0)), 1.0 / (p.p733)), 1e-38))));
        }

        if (s.v[1344] != 0.0) {
            s.store_scale_ad(13, A::limited_exp(A::div(A::neg(A::offset(s.ad_value(347), p.p731)), s.ad_value(343))), p.p733);
        }

        if (s.v[1344] != 0.0) {
            s.store_mul_ad_rhs(346, 341, A::offset(s.ad_value(13), 1.0));
        }

        if (s.v[1344] != 0.0) {
            s.store_div_ad_lhs(345, A::mul(A::neg(s.ad_value(341)), s.ad_value(13)), 343);
        }

        if (!(s.v[1344] != 0.0)) {
            s.store_scalar(343, 0.0);
        }

        if (!(s.v[1344] != 0.0)) {
            s.store_scalar(351, 0.0);
        }

        if (!(s.v[1344] != 0.0)) {
            s.store_scalar(350, 0.0);
        }

        if (!(s.v[1344] != 0.0)) {
            s.store_scalar(349, 0.0);
        }

        if (!(s.v[1344] != 0.0)) {
            s.store_scalar(348, 0.0);
        }

        if (!(s.v[1344] != 0.0)) {
            s.store_scalar(347, 0.0);
        }

        if (!(s.v[1344] != 0.0)) {
            s.store_scalar(346, 0.0);
        }

        if (!(s.v[1344] != 0.0)) {
            s.store_scalar(345, 0.0);
        }

        s.store_add_ad(342, A::add(A::mul(s.ad_value(251), s.ad_value(438)), A::mul(s.ad_value(301), s.ad_value(439))), A::scale(s.ad_value(440), (s.v[35] * p.p2)));

        s.v[1345] = if (s.v[342] > 0.0) { 1.0 } else { 0.0 };

        if (s.v[1345] != 0.0) {
            s.store_scale(344, 393, p.p726);
        }

        if (s.v[1345] != 0.0) {
            s.store_scale_ad(358, A::limited_exp(A::div_from_scalar((-p.p732), s.ad_value(344))), p.p734);
        }

        if (s.v[1345] != 0.0) {
            s.store_max_with_scalar_ad(14, A::div_from_scalar(p.p728, s.ad_value(342)), 10.0);
        }

        if (s.v[1345] != 0.0) {
            s.store_sub_ad_lhs(25, A::offset(s.ad_value(14), 1.0), 358);
        }

        if (s.v[1345] != 0.0) {
            s.store_mul_ad_rhs(357, 344, A::ln(A::max_with_scalar(A::scale(A::add(s.ad_value(25), A::sqrt(A::add(A::square(s.ad_value(25)), A::scale(s.ad_value(358), 4.0)))), 0.5), 1e-38)));
        }

        if (s.v[1345] != 0.0) {
            s.store_limited_exp_ad(12, A::div(s.ad_value(357), s.ad_value(344)));
        }

        if (s.v[1345] != 0.0) {
            s.store_mul_ad_rhs(356, 342, A::offset(A::add(A::sub(s.ad_value(12), A::div(s.ad_value(358), s.ad_value(12))), s.ad_value(358)), (-1.0)));
        }

        if (s.v[1345] != 0.0) {
            s.store_div_ad_lhs(355, A::mul(s.ad_value(342), A::add(s.ad_value(12), A::div(s.ad_value(358), s.ad_value(12)))), 344);
        }

        if (s.v[1345] != 0.0) {
            let assign15510_ad_e22914: A = {
                if (!(((p.p730 / s.v[342]) - 10.0) < ((-10000.0) * 0.001))) {
                    A::scale(A::add(A::offset(A::div_from_scalar(p.p730, s.ad_value(342)), (-10.0)), A::sqrt(A::offset(A::mul(A::offset(A::div_from_scalar(p.p730, s.ad_value(342)), (-10.0)), A::offset(A::div_from_scalar(p.p730, s.ad_value(342)), (-10.0))), ((4.0 * 0.001) * 0.001)))), 0.5)
                } else {
                    {
                        if (((p.p730 / s.v[342]) - 10.0) < ((-10000.0) * 0.001)) {
                            A::div_from_scalar(((-0.001) * 0.001), A::offset(A::div_from_scalar(p.p730, s.ad_value(342)), (-10.0)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_offset_ad(14, assign15510_ad_e22914, 10.0);
        }

        if (s.v[1345] != 0.0) {
            s.store_sub_from_scalar_ad(354, (-p.p732), A::mul(s.ad_value(344), A::ln(A::max_with_scalar(A::scale(A::offset(s.ad_value(14), (-1.0)), 1.0 / (p.p734)), 1e-38))));
        }

        if (s.v[1345] != 0.0) {
            s.store_scale_ad(13, A::limited_exp(A::div(A::neg(A::offset(s.ad_value(354), p.p732)), s.ad_value(344))), p.p734);
        }

        if (s.v[1345] != 0.0) {
            s.store_mul_ad_rhs(353, 342, A::offset(s.ad_value(13), 1.0));
        }

        if (s.v[1345] != 0.0) {
            s.store_div_ad_lhs(352, A::mul(A::neg(s.ad_value(342)), s.ad_value(13)), 344);
        }

        if (!(s.v[1345] != 0.0)) {
            s.store_scalar(344, 0.0);
        }

        if (!(s.v[1345] != 0.0)) {
            s.store_scalar(358, 0.0);
        }

        if (!(s.v[1345] != 0.0)) {
            s.store_scalar(357, 0.0);
        }

        if (!(s.v[1345] != 0.0)) {
            s.store_scalar(356, 0.0);
        }

        if (!(s.v[1345] != 0.0)) {
            s.store_scalar(355, 0.0);
        }

        if (!(s.v[1345] != 0.0)) {
            s.store_scalar(354, 0.0);
        }

        if (!(s.v[1345] != 0.0)) {
            s.store_scalar(353, 0.0);
        }

        if (!(s.v[1345] != 0.0)) {
            s.store_scalar(352, 0.0);
        }

        s.v[1346] = if (((p.p17 > 0.0) && (p.p18 > 0.0)) && ((p.p2 == 1.0) || ((p.p2 > 1.0) && (p.p19 > 0.0)))) { 1.0 } else { 0.0 };

        if (s.v[1346] != 0.0) {
            s.store_scalar(12, ((s.v[98]) as f64).powf(p.p921));
        }

        if (s.v[1346] != 0.0) {
            s.store_scalar(643, (s.v[100] + p.p914));
        }

        if (s.v[1346] != 0.0) {
            s.store_powf(13, 643, p.p922);
        }

        if (s.v[1346] != 0.0) {
            s.store_add_ad(644, A::add(A::div_from_scalar(p.p918, s.ad_value(12)), A::div_from_scalar(p.p919, s.ad_value(13))), A::div_from_scalar(p.p920, A::mul(s.ad_value(12), s.ad_value(13))));
        }

        if (s.v[1346] != 0.0) {
            s.store_offset(645, 644, 1.0);
        }

        if (s.v[1346] != 0.0) {
            s.store_scalar(12, ((s.v[98]) as f64).powf(p.p927));
        }

        if (s.v[1346] != 0.0) {
            s.store_powf(13, 643, p.p928);
        }

        if (s.v[1346] != 0.0) {
            s.store_add_ad(646, A::add(A::div_from_scalar(p.p924, s.ad_value(12)), A::div_from_scalar(p.p925, s.ad_value(13))), A::div_from_scalar(p.p926, A::mul(s.ad_value(12), s.ad_value(13))));
        }

        if (s.v[1346] != 0.0) {
            s.store_offset(647, 646, 1.0);
        }

        if (s.v[1346] != 0.0) {
            s.store_offset(12, 395, (-1.0));
        }

        if (s.v[1346] != 0.0) {
            s.store_offset_ad(648, A::mul(s.ad_value(645), A::offset(A::scale(s.ad_value(12), p.p917), 1.0)), 1e-9);
        }

        if (s.v[1346] != 0.0) {
            s.store_scalar(662, 0.0);
        }

        let mut assign15770_loop_guard: usize = 0;
        while {
            let assign15770_cond_e23123: f64 = if ((s.v[1346] != 0.0) && (s.v[662] < p.p2)) { 1.0 } else { 0.0 };
            assign15770_cond_e23123 != 0.0
        } {
            assign15770_loop_guard += 1;
            assert!(assign15770_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (s.v[1346] != 0.0) {
                s.store_div_from_scalar_ad(12, (1.0 / p.p2), A::offset(A::scale(s.ad_value(662), (p.p19 + s.v[99])), (p.p17 + (0.5 * s.v[99]))));
            }
            if (s.v[1346] != 0.0) {
                s.store_div_from_scalar_ad(13, (1.0 / p.p2), A::offset(A::scale(s.ad_value(662), (p.p19 + s.v[99])), (p.p18 + (0.5 * s.v[99]))));
            }
            if (s.v[1346] != 0.0) {
                s.store_offset(649, 12, s.v[649]);
            }
            if (s.v[1346] != 0.0) {
                s.store_offset(650, 13, s.v[650]);
            }
            if (s.v[1346] != 0.0) {
                s.store_offset(662, 662, 1.0);
            }
        }

        if (s.v[1346] != 0.0) {
            s.store_scalar(651, (1.0 / (p.p912 + (0.5 * s.v[99]))));
        }

        if (s.v[1346] != 0.0) {
            s.store_scalar(652, (1.0 / (p.p913 + (0.5 * s.v[99]))));
        }

        if (s.v[1346] != 0.0) {
            s.store_add(653, 651, 652);
        }

        if (s.v[1346] != 0.0) {
            s.store_mul_ad_lhs(654, A::div_from_scalar(p.p915, s.ad_value(648)), 653);
        }

        if (s.v[1346] != 0.0) {
            s.store_add(655, 649, 650);
        }

        if (s.v[1346] != 0.0) {
            s.store_mul_ad_lhs(656, A::div_from_scalar(p.p915, s.ad_value(648)), 655);
        }

        if (s.v[1346] != 0.0) {
            s.store_div_ad(657, A::offset(s.ad_value(656), 1.0), A::offset(s.ad_value(654), 1.0));
        }

        if (s.v[1346] != 0.0) {
            s.store_div_ad(658, A::offset(A::scale(s.ad_value(656), p.p916), 1.0), A::offset(A::scale(s.ad_value(654), p.p916), 1.0));
        }

        if (s.v[1346] != 0.0) {
            s.store_mul_ad(659, A::div_from_scalar(p.p923, s.ad_value(647)), A::sub(s.ad_value(655), s.ad_value(653)));
        }

        if (s.v[1346] != 0.0) {
            s.store_mul_ad(660, A::div_from_scalar(p.p929, A::powf(s.ad_value(647), p.p930)), A::sub(s.ad_value(655), s.ad_value(653)));
        }

        if (s.v[1346] != 0.0) {
            s.store_mul_ad(661, A::div_from_scalar(p.p931, A::powf(s.ad_value(647), p.p932)), A::sub(s.ad_value(655), s.ad_value(653)));
        }

        if (s.v[1346] != 0.0) {
            s.store_mul(397, 397, 657);
        }

        if (s.v[1346] != 0.0) {
            s.store_mul(409, 409, 658);
        }

        if (s.v[1346] != 0.0) {
            s.store_add(494, 494, 660);
        }

        if (s.v[1346] != 0.0) {
            s.store_add(420, 420, 661);
        }

        s.v[1347] = if (p.p37 == 1.0) { 1.0 } else { 0.0 };

        if ((s.v[1346] != 0.0) && (s.v[1347] != 0.0)) {
            s.store_mul_ad(688, A::div(s.ad_value(625), s.ad_value(647)), A::sub(s.ad_value(655), s.ad_value(653)));
        }

        if ((s.v[1346] != 0.0) && (s.v[1347] != 0.0)) {
            s.store_mul_ad(689, A::div(s.ad_value(626), A::powf(s.ad_value(647), p.p930)), A::sub(s.ad_value(655), s.ad_value(653)));
        }

        if ((s.v[1346] != 0.0) && (s.v[1347] != 0.0)) {
            s.store_mul_ad(690, A::div(s.ad_value(627), A::powf(s.ad_value(647), p.p932)), A::sub(s.ad_value(655), s.ad_value(653)));
        }

        if (s.v[1346] != 0.0) {
            s.store_add(624, 624, 689);
        }

        if (s.v[1346] != 0.0) {
            s.store_add(616, 616, 690);
        }

        if (!(s.v[1346] != 0.0)) {
            s.store_scalar(659, 0.0);
        }

        if (!(s.v[1346] != 0.0)) {
            s.store_scalar(688, 0.0);
        }

        s.v[1348] = if (p.p43 == 1.0) { 1.0 } else { 0.0 };

        if (s.v[1348] != 0.0) {
            s.store_scalar(668, (p.p1 / p.p2));
        }

        if (s.v[1348] != 0.0) {
            s.store_scalar(669, p.p20);
        }

        if (s.v[1348] != 0.0) {
            s.store_scalar(670, p.p21);
        }

        if (s.v[1348] != 0.0) {
            s.store_scalar(671, p.p22);
        }

        s.v[1349] = if (((!(if self.param_given[20] { 1.0 } else { 0.0 } != 0.0)) && (!(if self.param_given[21] { 1.0 } else { 0.0 } != 0.0))) && (!(if self.param_given[22] { 1.0 } else { 0.0 } != 0.0))) { 1.0 } else { 0.0 };

        s.v[1350] = if ((if self.param_given[23] { 1.0 } else { 0.0 } != 0.0) && (p.p23 > 0.0)) { 1.0 } else { 0.0 };

        if (((s.v[1348] != 0.0) && (s.v[1349] != 0.0)) && (s.v[1350] != 0.0)) {
            s.store_offset(13, 668, p.p23);
        }

        if (((s.v[1348] != 0.0) && (s.v[1349] != 0.0)) && (s.v[1350] != 0.0)) {
            s.store_scalar(14, (1.0 / p.p947));
        }

        if (((s.v[1348] != 0.0) && (s.v[1349] != 0.0)) && (s.v[1350] != 0.0)) {
            s.store_div_from_scalar_ad(669, (p.p947 * p.p947), A::scale(s.ad_value(13), p.p23));
        }

        if (((s.v[1348] != 0.0) && (s.v[1349] != 0.0)) && (s.v[1350] != 0.0)) {
            s.store_div_ad_lhs(670, A::sub(A::scale(A::limited_exp(A::scale(s.ad_value(14), ((-10.0) * p.p23))), ((0.1 * p.p23) + (0.01 * p.p947))), A::mul(A::offset(A::scale(s.ad_value(13), 0.1), (0.01 * p.p947)), A::limited_exp(A::mul(A::scale(s.ad_value(13), (-10.0)), s.ad_value(14))))), 668);
        }

        if (((s.v[1348] != 0.0) && (s.v[1349] != 0.0)) && (s.v[1350] != 0.0)) {
            s.store_div_ad_lhs(671, A::sub(A::scale(A::limited_exp(A::scale(s.ad_value(14), ((-20.0) * p.p23))), ((0.05 * p.p23) + (0.0025 * p.p947))), A::mul(A::offset(A::scale(s.ad_value(13), 0.05), (0.0025 * p.p947)), A::limited_exp(A::mul(A::scale(s.ad_value(13), (-20.0)), s.ad_value(14))))), 668);
        }

        s.store_mul_ad_rhs(663, 578, A::add(A::add(s.ad_value(669), A::scale(s.ad_value(670), p.p933)), A::scale(s.ad_value(671), p.p934)));

        s.store_mul_ad_rhs(664, 579, A::add(A::add(s.ad_value(669), A::scale(s.ad_value(670), p.p933)), A::scale(s.ad_value(671), p.p934)));

        s.store_mul_ad_rhs(665, 630, A::add(A::add(s.ad_value(669), A::scale(s.ad_value(670), p.p933)), A::scale(s.ad_value(671), p.p934)));

        s.store_mul_ad_rhs(666, 629, A::add(A::add(s.ad_value(669), A::scale(s.ad_value(670), p.p933)), A::scale(s.ad_value(671), p.p934)));

        s.store_offset_ad(667, A::mul(s.ad_value(580), A::add(A::add(s.ad_value(669), A::scale(s.ad_value(670), p.p933)), A::scale(s.ad_value(671), p.p934))), 1.0);

        s.store_mul(397, 397, 667);

        s.store_add(494, 494, 664);

        s.store_mul_ad_rhs(64, 187, A::voltage(ctx, &nodes, Some(9), Some(11)));

        s.store_mul_ad_rhs(66, 187, A::voltage(ctx, &nodes, Some(5), Some(11)));

        s.store_mul_ad_rhs(70, 187, A::voltage(ctx, &nodes, Some(7), Some(11)));

        s.store_sub(74, 66, 70);

        s.copy_ad(68, 66);

    }

    pub(super) fn stamp_reactive_block_9(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        s.copy_ad(56, 74);

        s.copy_ad(50, 70);

        s.copy_ad(48, 66);

        s.store_mul_ad_rhs(306, 187, A::voltage(ctx, &nodes, Some(12), Some(7)));

        s.store_mul_ad_rhs(307, 187, A::voltage(ctx, &nodes, Some(13), Some(5)));

        s.store_mul_ad_rhs(308, 187, A::voltage(ctx, &nodes, Some(13), Some(5)));

        s.store_mul_ad_rhs(309, 187, A::voltage(ctx, &nodes, Some(13), Some(14)));

        s.store_sub(54, 64, 66);

        s.store_sub(52, 64, 70);

        s.store_mul_ad_rhs(230, 187, A::voltage(ctx, &nodes, Some(10), Some(5)));

        s.store_mul_ad_rhs(231, 187, A::voltage(ctx, &nodes, Some(10), Some(7)));

        s.copy_ad(232, 230);

        s.v[1351] = if ((((p.p1110 != 0.0) && (p.p42 == 1.0)) && (p.p1095 == 1.0)) && (p.p1094 == 1.0)) { 1.0 } else { 0.0 };

        if (s.v[1351] != 0.0) {
            s.store_add_ad_rhs(68, 66, A::mul(A::scale(s.ad_value(187), (1.0 - (p.p1111 / p.p1110))), A::voltage(ctx, &nodes, Some(6), Some(5))));
        }

        if (s.v[1351] != 0.0) {
            s.store_sub_ad_lhs(308, A::add(s.ad_value(307), s.ad_value(66)), 68);
        }

        if (s.v[1351] != 0.0) {
            s.store_sub_ad_lhs(232, A::add(s.ad_value(230), s.ad_value(66)), 68);
        }

        s.copy_ad(69, 68);

        s.store_mul_ad_rhs(72, 187, A::voltage(ctx, &nodes, Some(7), Some(11)));

        s.v[57] = 1.0;

        s.v[1352] = if (s.v[74] < 0.0) { 1.0 } else { 0.0 };

        if (s.v[1352] != 0.0) {
            s.store_scalar(57, (-1.0));
        }

        if (s.v[1352] != 0.0) {
            s.store_mul_ad_rhs(66, 187, A::voltage(ctx, &nodes, Some(7), Some(11)));
        }

        if (s.v[1352] != 0.0) {
            s.store_mul_ad_rhs(70, 187, A::voltage(ctx, &nodes, Some(5), Some(11)));
        }

        if (s.v[1352] != 0.0) {
            s.copy_ad(72, 69);
        }

        if (s.v[1352] != 0.0) {
            s.store_mul_ad_rhs(68, 187, A::voltage(ctx, &nodes, Some(7), Some(11)));
        }

        s.store_sub(74, 66, 70);

        s.store_sub(75, 68, 72);

        s.store_scale(12, 75, p.p956);

        if ((!(s.v[12] > 37.0)) && (!(s.v[12] < (-37.0)))) {
            s.store_ln_ad(13, A::offset(A::exp(s.ad_value(12)), 1.0));
        } else {
            if ((!(s.v[12] > 37.0)) && (s.v[12] < (-37.0))) {
                s.store_exp(13, 12);
            } else {
                if (s.v[12] > 37.0) {
                    s.copy_ad(13, 12);
                } else {
                    s.store_scalar(13, 0.0);
                }
            }
        }

        s.store_offset_ad(76, A::sub(A::scale(s.ad_value(13), (2.0 / p.p956)), s.ad_value(75)), (-((2.0 / p.p956) * ((2.0) as f64).ln())));

        s.store_neg_ad(62, A::add(s.ad_value(72), A::scale(A::sub(s.ad_value(75), s.ad_value(76)), 0.5)));

        s.store_scale(12, 74, p.p956);

        if ((!(s.v[12] > 37.0)) && (!(s.v[12] < (-37.0)))) {
            s.store_ln_ad(13, A::offset(A::exp(s.ad_value(12)), 1.0));
        } else {
            if ((!(s.v[12] > 37.0)) && (s.v[12] < (-37.0))) {
                s.store_exp(13, 12);
            } else {
                if (s.v[12] > 37.0) {
                    s.copy_ad(13, 12);
                } else {
                    s.store_scalar(13, 0.0);
                }
            }
        }

        s.store_offset_ad(76, A::sub(A::scale(s.ad_value(13), (2.0 / p.p956)), s.ad_value(74)), (-((2.0 / p.p956) * ((2.0) as f64).ln())));

        s.store_neg_ad(61, A::add(s.ad_value(70), A::scale(A::sub(s.ad_value(74), s.ad_value(76)), 0.5)));

        s.store_tanh_ad(12, A::div(A::scale(s.ad_value(56), p.p1123), s.ad_value(393)));

        s.store_offset_scaled(102, 12, 0.5, 0.5);

        s.store_sub_from_scalar(103, 1.0, 102);

        s.v[1353] = if (p.p44 != 0.0) { 1.0 } else { 0.0 };

        if (s.v[1353] != 0.0) {
            s.store_add_ad(486, A::mul(s.ad_value(485), s.ad_value(103)), A::mul(s.ad_value(484), s.ad_value(102)));
        }

        if (s.v[1353] != 0.0) {
            s.store_add_ad(492, A::mul(s.ad_value(421), s.ad_value(103)), A::mul(s.ad_value(420), s.ad_value(102)));
        }

        if (s.v[1353] != 0.0) {
            s.store_add_ad(519, A::mul(s.ad_value(518), s.ad_value(103)), A::mul(s.ad_value(517), s.ad_value(102)));
        }

        if (s.v[1353] != 0.0) {
            s.store_add_ad(541, A::mul(s.ad_value(540), s.ad_value(103)), A::mul(s.ad_value(539), s.ad_value(102)));
        }

        if (s.v[1353] != 0.0) {
            s.store_add_ad(166, A::mul(s.ad_value(165), s.ad_value(103)), A::mul(s.ad_value(164), s.ad_value(102)));
        }

        if (s.v[1353] != 0.0) {
            s.store_add_ad(502, A::mul(s.ad_value(410), s.ad_value(103)), A::mul(s.ad_value(409), s.ad_value(102)));
        }

        if (s.v[1353] != 0.0) {
            s.store_add_ad(536, A::mul(s.ad_value(414), s.ad_value(103)), A::mul(s.ad_value(413), s.ad_value(102)));
        }

        if (s.v[1353] != 0.0) {
            s.store_add_ad(499, A::mul(s.ad_value(398), s.ad_value(103)), A::mul(s.ad_value(397), s.ad_value(102)));
        }

        if (s.v[1353] != 0.0) {
            s.store_add_ad(506, A::mul(s.ad_value(400), s.ad_value(103)), A::mul(s.ad_value(399), s.ad_value(102)));
        }

        if (s.v[1353] != 0.0) {
            s.store_add_ad(516, A::mul(s.ad_value(402), s.ad_value(103)), A::mul(s.ad_value(401), s.ad_value(102)));
        }

        if (s.v[1353] != 0.0) {
            s.store_add_ad(510, A::mul(s.ad_value(404), s.ad_value(103)), A::mul(s.ad_value(403), s.ad_value(102)));
        }

        if (s.v[1353] != 0.0) {
            s.store_add_ad(513, A::mul(s.ad_value(406), s.ad_value(103)), A::mul(s.ad_value(405), s.ad_value(102)));
        }

        if (s.v[1353] != 0.0) {
            s.store_add_ad(553, A::mul(s.ad_value(552), s.ad_value(103)), A::mul(s.ad_value(551), s.ad_value(102)));
        }

        if (s.v[1353] != 0.0) {
            s.store_add_ad(558, A::mul(s.ad_value(416), s.ad_value(103)), A::mul(s.ad_value(415), s.ad_value(102)));
        }

        if (!(s.v[1353] != 0.0)) {
            s.copy_ad(486, 484);
        }

        if (!(s.v[1353] != 0.0)) {
            s.copy_ad(492, 420);
        }

        if (!(s.v[1353] != 0.0)) {
            s.copy_ad(519, 517);
        }

        if (!(s.v[1353] != 0.0)) {
            s.copy_ad(541, 539);
        }

        if (!(s.v[1353] != 0.0)) {
            s.copy_ad(166, 164);
        }

        if (!(s.v[1353] != 0.0)) {
            s.copy_ad(502, 409);
        }

        if (!(s.v[1353] != 0.0)) {
            s.copy_ad(536, 413);
        }

        if (!(s.v[1353] != 0.0)) {
            s.copy_ad(499, 397);
        }

        if (!(s.v[1353] != 0.0)) {
            s.copy_ad(506, 399);
        }

        if (!(s.v[1353] != 0.0)) {
            s.copy_ad(516, 401);
        }

        if (!(s.v[1353] != 0.0)) {
            s.copy_ad(510, 403);
        }

        if (!(s.v[1353] != 0.0)) {
            s.copy_ad(513, 405);
        }

        if (!(s.v[1353] != 0.0)) {
            s.copy_ad(553, 551);
        }

        if (!(s.v[1353] != 0.0)) {
            s.copy_ad(558, 415);
        }

        s.v[1354] = if ((0.05 == 0.0) && ((s.v[127] - s.v[61]) < ((-2500.0) * 0.1))) { 1.0 } else { 0.0 };

        if (s.v[1354] != 0.0) {
            s.store_div_from_scalar_ad(110, ((-0.1) * 0.1), A::scale(A::sub(s.ad_value(127), s.ad_value(61)), 16.0));
        }

        if (!(s.v[1354] != 0.0)) {
            s.store_scale_ad(110, A::add(A::offset(A::sub(s.ad_value(127), s.ad_value(61)), 0.05), A::sqrt(A::offset(A::mul(A::offset(A::sub(s.ad_value(127), s.ad_value(61)), (-0.05)), A::offset(A::sub(s.ad_value(127), s.ad_value(61)), (-0.05))), ((0.25 * 0.1) * 0.1)))), 0.5);
        }

        s.store_sqrt(111, 110);

        s.store_mul(112, 114, 111);

        s.store_div_from_scalar(97, s.v[26], 112);

        s.store_sub_ad(113, A::add(A::add(s.ad_value(483), s.ad_value(422)), A::mul(s.ad_value(486), s.ad_value(76))), A::mul(s.ad_value(487), s.ad_value(61)));

        s.store_offset_scaled(13, 113, 1.0 / (s.v[46]), 1.0);

        s.v[1355] = if ((1.0 == 0.0) && (s.v[13] < ((-2500.0) * 0.05))) { 1.0 } else { 0.0 };

        if (s.v[1355] != 0.0) {
            s.store_div_from_scalar_ad(104, ((-0.05) * 0.05), A::scale(s.ad_value(13), 16.0));
        }

        if (!(s.v[1355] != 0.0)) {
            s.store_scale_ad(104, A::add(A::offset(s.ad_value(13), 1.0), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(13), (-1.0)), A::offset(s.ad_value(13), (-1.0))), ((0.25 * 0.05) * 0.05)))), 0.5);
        }

        s.store_mul(106, 104, 108);

        s.store_div_from_scalar(107, 1.0, 106);

        s.store_mul_ad_lhs(123, A::neg(A::add(s.ad_value(492), A::mul(s.ad_value(493), s.ad_value(61)))), 76);

        s.store_offset_ad(123, A::scale(A::sub(s.ad_value(123), A::sqrt(A::offset(A::mul(s.ad_value(123), s.ad_value(123)), ((0.25 * 0.005) * 0.005)))), 0.5), (0.25 * 0.005));

        s.store_mul_ad(124, A::add(A::offset(s.ad_value(454), (p.p869 / s.v[30])), A::mul(s.ad_value(455), s.ad_value(61))), A::offset(A::powf(s.ad_value(395), p.p868), (-1.0)));

        s.v[1356] = if (s.v[116] > 0.0) { 1.0 } else { 0.0 };

        if (s.v[1356] != 0.0) {
            s.store_mul_ad_lhs(12, A::neg(s.ad_value(117)), 76);
        }

        s.v[1357] = if (s.v[12] < (-80.0)) { 1.0 } else { 0.0 };

        if ((s.v[1356] != 0.0) && (s.v[1357] != 0.0)) {
            s.store_scalar(14, 1.804851387e-35);
        }

        if ((s.v[1356] != 0.0) && (!(s.v[1357] != 0.0))) {
            s.store_ad(14, &A::limited_exp(s.ad_value(12)));
        }

        if (s.v[1356] != 0.0) {
            s.store_offset_ad(15, A::mul(s.ad_value(116), A::offset(s.ad_value(14), 1.0)), s.v[30]);
        }

        if (s.v[1356] != 0.0) {
            s.store_mul_ad(115, A::neg(s.ad_value(106)), A::ln(A::max_with_scalar(A::div_from_scalar(s.v[30], s.ad_value(15)), 1e-38)));
        }

        if (!(s.v[1356] != 0.0)) {
            s.store_scalar(115, 0.0);
        }

        s.store_add_ad_rhs(16, 121, A::div(s.ad_value(118), A::pow_from_scalar(s.v[30], s.ad_value(119))));

        s.store_sub_ad_rhs(115, 115, A::mul(s.ad_value(16), A::tanh(A::mul(s.ad_value(120), s.ad_value(76)))));

        s.store_offset(482, 482, p.p35);

        s.store_mul(65, 64, 107);

        s.store_mul(73, 70, 107);

        s.store_mul(58, 482, 107);

        s.store_sub_ad(122, A::mul(s.ad_value(495), A::sub(s.ad_value(111), s.ad_value(128))), A::mul(s.ad_value(494), s.ad_value(61)));

        s.store_add_ad_lhs(79, A::add(A::sub(A::add(A::add(s.ad_value(123), s.ad_value(115)), s.ad_value(122)), s.ad_value(124)), s.ad_value(659)), 663);

        s.store_sub_ad(59, A::sub(s.ad_value(65), s.ad_value(58)), A::mul(s.ad_value(79), s.ad_value(107)));

        s.store_scale_ad(125, A::sqrt(A::mul(A::scale(s.ad_value(481), ((2.0 * 1.60219e-19) * s.v[26])), s.ad_value(109))), 1.0 / (s.v[46]));

        if (!(((2.0 * s.v[88]) + (s.v[70] * s.v[109])) < ((-10000.0) * 0.001))) {
            s.store_scale_ad(12, A::add(A::add(A::scale(s.ad_value(88), 2.0), A::mul(s.ad_value(70), s.ad_value(109))), A::sqrt(A::offset(A::mul(A::add(A::scale(s.ad_value(88), 2.0), A::mul(s.ad_value(70), s.ad_value(109))), A::add(A::scale(s.ad_value(88), 2.0), A::mul(s.ad_value(70), s.ad_value(109)))), ((4.0 * 0.001) * 0.001)))), 0.5);
        } else {
            if (((2.0 * s.v[88]) + (s.v[70] * s.v[109])) < ((-10000.0) * 0.001)) {
                s.store_div_from_scalar_ad(12, ((-0.001) * 0.001), A::add(A::scale(s.ad_value(88), 2.0), A::mul(s.ad_value(70), s.ad_value(109))));
            } else {
                s.store_scalar(12, 0.0);
            }
        }

        s.store_offset_ad(90, A::div(s.ad_value(125), A::scale(A::sqrt(s.ad_value(12)), 2.0)), 1.0);

        s.store_scale_ad(125, A::sqrt(A::mul(A::scale(s.ad_value(481), ((2.0 * 1.60219e-19) * s.v[26])), s.ad_value(107))), 1.0 / (s.v[46]));

        s.store_div_from_scalar(126, 1.0, 125);

        s.store_div(89, 88, 104);

        s.v[13] = 1.0;

        s.store_scale(204, 59, 1.0 / (s.v[13]));

        s.store_scale(205, 125, 1.0 / (s.v[13]));

        s.store_sub_ad(13, A::scale(s.ad_value(204), 0.5), A::scale(A::offset(A::scale(s.ad_value(205), 0.7071067811865475), 1.0), 3.0));

        s.store_add_ad_rhs(14, 13, A::sqrt(A::add(A::square(s.ad_value(13)), A::scale(s.ad_value(204), 6.0))));

        s.v[1358] = if (s.v[204] < 0.0) { 1.0 } else { 0.0 };

        if (s.v[1358] != 0.0) {
            s.store_div_ad_lhs(15, A::sub(s.ad_value(204), s.ad_value(14)), 205);
        }

        if (s.v[1358] != 0.0) {
            s.store_neg_ad(91, A::ln(A::max_with_scalar(A::add(A::sub_from_scalar(1.0, s.ad_value(14)), A::square(s.ad_value(15))), 1e-38)));
        }

        if (!(s.v[1358] != 0.0)) {
            s.store_limited_exp_ad(15, A::neg(s.ad_value(14)));
        }

        if (!(s.v[1358] != 0.0)) {
            s.store_scale(13, 205, 0.5);
        }

        if (!(s.v[1358] != 0.0)) {
            s.store_sub_ad_lhs(14, A::sqrt(A::add(A::add(A::offset(s.ad_value(204), (-1.0)), s.ad_value(15)), A::square(s.ad_value(13)))), 13);
        }

        if (!(s.v[1358] != 0.0)) {
            s.store_sub_ad_lhs(91, A::offset(A::square(s.ad_value(14)), 1.0), 15);
        }

        s.store_scale_ad(20, A::add(A::offset(s.ad_value(91), 1.0), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(91), (-1.0)), A::offset(s.ad_value(91), (-1.0))), ((0.25 * 2.0) * 2.0)))), 0.5);

        s.store_sqrt(96, 20);

        s.store_div_ad_lhs(12, A::offset(A::div(s.ad_value(125), A::scale(s.ad_value(96), 2.0)), 1.0), 125);

        s.store_sub_ad_lhs(13, A::sub(s.ad_value(91), A::scale(s.ad_value(89), 2.0)), 73);

        s.store_sub_ad_rhs(14, 13, A::ln(A::max_with_scalar(A::mul(A::scale(s.ad_value(12), 4.0), s.ad_value(96)), 1e-38)));

        s.store_scale_ad(20, A::sub(A::offset(s.ad_value(14), (-0.201491)), A::sqrt(A::offset(A::mul(s.ad_value(14), A::offset(s.ad_value(14), 0.402982)), 2.446562))), 0.5);

        s.copy_ad(94, 96);

        s.v[1359] = if (s.v[20] <= (-68.0)) { 1.0 } else { 0.0 };

        if (s.v[1359] != 0.0) {
            s.store_scalar(16, (-100.0));
        }

        if (s.v[1359] != 0.0) {
            s.store_scalar(17, 20.0);
        }

        s.v[1360] = if (s.v[20] < (s.v[16] - (0.5 * s.v[17]))) { 1.0 } else { 0.0 };

        if ((s.v[1359] != 0.0) && (s.v[1360] != 0.0)) {
            s.store_ad(15, &A::limited_exp(s.ad_value(16)));
        }

        s.v[1361] = if (s.v[20] > (s.v[16] + (0.5 * s.v[17]))) { 1.0 } else { 0.0 };

        if (((s.v[1359] != 0.0) && (!(s.v[1360] != 0.0))) && (s.v[1361] != 0.0)) {
            s.store_ad(15, &A::limited_exp(s.ad_value(20)));
        }

        if (((s.v[1359] != 0.0) && (!(s.v[1360] != 0.0))) && (!(s.v[1361] != 0.0))) {
            s.store_div_ad_lhs(14, A::sub(s.ad_value(20), s.ad_value(16)), 17);
        }

        if (((s.v[1359] != 0.0) && (!(s.v[1360] != 0.0))) && (!(s.v[1361] != 0.0))) {
            s.store_square(18, 14);
        }

        if (((s.v[1359] != 0.0) && (!(s.v[1360] != 0.0))) && (!(s.v[1361] != 0.0))) {
            s.store_limited_exp_ad(15, A::add(s.ad_value(16), A::mul(s.ad_value(17), A::add(A::offset(A::scale(s.ad_value(14), 0.5), (5.0 / 64.0)), A::mul(s.ad_value(18), A::sub_from_scalar((15.0 / 16.0), A::mul(s.ad_value(18), A::sub_from_scalar(1.25, s.ad_value(18)))))))));
        }

        if (s.v[1359] != 0.0) {
            s.store_mul_ad_rhs(200, 15, A::sub(A::sub(A::offset(s.ad_value(13), 1.0), s.ad_value(20)), A::ln(A::max_with_scalar(A::mul(A::scale(s.ad_value(12), 2.0), A::add(A::mul(A::scale(s.ad_value(15), 2.0), s.ad_value(12)), A::scale(s.ad_value(94), 2.0))), 1e-38))));
        }

        if (!(s.v[1359] != 0.0)) {
            s.store_ad(15, &A::limited_exp(s.ad_value(20)));
        }

        if (!(s.v[1359] != 0.0)) {
            s.store_div_from_scalar(95, 1.0, 94);
        }

        if (!(s.v[1359] != 0.0)) {
            s.store_sub_ad_lhs(16, A::add(A::scale(s.ad_value(15), 2.0), A::ln(A::max_with_scalar(A::mul(A::mul(A::scale(s.ad_value(15), 2.0), s.ad_value(12)), A::add(A::mul(A::scale(s.ad_value(15), 2.0), s.ad_value(12)), A::scale(s.ad_value(94), 2.0))), 1e-38))), 13);
        }

        if (!(s.v[1359] != 0.0)) {
            s.store_add_ad(17, A::offset(A::div_from_scalar(1.0, s.ad_value(15)), 2.0), A::div(A::add(s.ad_value(12), s.ad_value(95)), A::add(A::mul(s.ad_value(12), s.ad_value(15)), s.ad_value(94))));
        }

        if (!(s.v[1359] != 0.0)) {
            s.store_sub_ad_rhs(15, 15, A::div(s.ad_value(16), s.ad_value(17)));
        }

        if (!(s.v[1359] != 0.0)) {
            s.store_sub_ad_lhs(16, A::add(A::scale(s.ad_value(15), 2.0), A::ln(A::max_with_scalar(A::mul(A::mul(A::scale(s.ad_value(15), 2.0), s.ad_value(12)), A::add(A::mul(A::scale(s.ad_value(15), 2.0), s.ad_value(12)), A::scale(s.ad_value(94), 2.0))), 1e-38))), 13);
        }

        if (!(s.v[1359] != 0.0)) {
            s.store_add_ad(17, A::offset(A::div_from_scalar(1.0, s.ad_value(15)), 2.0), A::div(A::add(s.ad_value(12), s.ad_value(95)), A::add(A::mul(s.ad_value(12), s.ad_value(15)), s.ad_value(94))));
        }

        if (!(s.v[1359] != 0.0)) {
            s.store_mul_ad(18, A::div(A::add(s.ad_value(12), s.ad_value(95)), A::add(A::mul(s.ad_value(12), s.ad_value(15)), s.ad_value(94))), A::div(A::add(s.ad_value(12), s.ad_value(95)), A::add(A::mul(s.ad_value(12), s.ad_value(15)), s.ad_value(94))));
        }

        if (!(s.v[1359] != 0.0)) {
            s.store_sub_ad_lhs(19, A::sub(A::scale(A::mul(A::div_from_scalar(1.0, s.ad_value(15)), A::div_from_scalar(1.0, s.ad_value(15))), (-1.0)), A::div_from_scalar(1.0, A::mul(A::mul(A::square(s.ad_value(94)), s.ad_value(94)), A::add(A::mul(s.ad_value(12), s.ad_value(15)), s.ad_value(94))))), 18);
        }

        if (!(s.v[1359] != 0.0)) {
            s.store_sub_ad_rhs(200, 15, A::mul(A::div(s.ad_value(16), s.ad_value(17)), A::offset(A::div(A::mul(s.ad_value(16), s.ad_value(19)), A::mul(A::scale(s.ad_value(17), 2.0), s.ad_value(17))), 1.0)));
        }

        s.v[1362] = if ((1.0 == 0.0) && (s.v[91] < ((-2500.0) * 2.0))) { 1.0 } else { 0.0 };

        if (s.v[1362] != 0.0) {
            s.store_div_from_scalar_ad(93, ((-2.0) * 2.0), A::scale(s.ad_value(91), 16.0));
        }

        if (!(s.v[1362] != 0.0)) {
            s.store_scale_ad(93, A::add(A::offset(s.ad_value(91), 1.0), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(91), (-1.0)), A::offset(s.ad_value(91), (-1.0))), ((0.25 * 2.0) * 2.0)))), 0.5);
        }

        s.store_sqrt(96, 93);

        s.store_sub_ad_rhs(92, 91, A::scale(s.ad_value(200), 2.0));

        s.v[1363] = if ((1.0 == 0.0) && (s.v[92] < ((-2500.0) * 2.0))) { 1.0 } else { 0.0 };

        if (s.v[1363] != 0.0) {
            s.store_div_from_scalar_ad(12, ((-2.0) * 2.0), A::scale(s.ad_value(92), 16.0));
        }

        if (!(s.v[1363] != 0.0)) {
            s.store_scale_ad(12, A::add(A::offset(s.ad_value(92), 1.0), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(92), (-1.0)), A::offset(s.ad_value(92), (-1.0))), ((0.25 * 2.0) * 2.0)))), 0.5);
        }

        s.store_offset_ad(90, A::div(s.ad_value(125), A::add(s.ad_value(96), A::sqrt(s.ad_value(12)))), 1.0);

        s.v[155] = (1e-8 / (s.v[47] * p.p77));

        s.store_mul_ad_rhs(12, 106, A::sub(A::sub(s.ad_value(59), s.ad_value(91)), A::mul(A::scale(s.ad_value(200), 2.0), A::offset(s.ad_value(90), (-1.0)))));

        s.v[1364] = if ((0.0 == 0.0) && (s.v[12] < ((-2500.0) * 0.1))) { 1.0 } else { 0.0 };

        if (s.v[1364] != 0.0) {
            s.store_div_from_scalar_ad(84, ((-0.1) * 0.1), A::scale(s.ad_value(12), 16.0));
        }

        if (!(s.v[1364] != 0.0)) {
            s.store_scale_ad(84, A::add(s.ad_value(12), A::sqrt(A::offset(A::mul(s.ad_value(12), s.ad_value(12)), ((0.25 * 0.1) * 0.1)))), 0.5);
        }

        s.store_mul_ad_lhs(130, A::mul(A::scale(s.ad_value(90), 2.0), s.ad_value(106)), 200);

        s.store_scale_ad(132, A::add(s.ad_value(84), A::scale(s.ad_value(130), s.v[158])), s.v[155]);

        s.store_ad(14, &A::pow(A::scale(A::offset(A::div(s.ad_value(130), s.ad_value(84)), 1.0), 0.5), s.ad_value(513)));

        s.store_add_ad(15, A::mul(A::add(s.ad_value(506), A::mul(s.ad_value(516), s.ad_value(61))), A::pow(s.ad_value(132), s.ad_value(407))), A::div(s.ad_value(510), s.ad_value(14)));

    }
}
