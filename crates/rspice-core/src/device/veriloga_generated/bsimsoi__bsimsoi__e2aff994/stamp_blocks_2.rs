#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, ReactiveScratch, Scratch, THERMAL_VOLTAGE_PER_K};
use super::super::state::Instance;

impl Instance {

    pub(super) fn stamp_transient_block_32(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if ((((!(s.v[1620] != 0.0)) && (s.v[1837] != 0.0)) && (!(s.v[1838] != 0.0))) && (!(s.v[1839] != 0.0))) {
            s.store_limited_exp_ad(170, A::add(s.ad_value(171), A::mul(s.ad_value(172), A::add(A::offset(A::scale(s.ad_value(169), 0.5), (5.0 / 64.0)), A::mul(s.ad_value(173), A::sub_from_scalar((15.0 / 16.0), A::mul(s.ad_value(173), A::sub_from_scalar(1.25, s.ad_value(173)))))))));
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1837] != 0.0)) {
            s.store_mul_ad_rhs(400, 170, A::sub(A::sub(A::offset(s.ad_value(168), 1.0), s.ad_value(175)), A::ln(A::max_with_scalar(A::mul(A::scale(s.ad_value(167), 2.0), A::add(A::mul(A::scale(s.ad_value(170), 2.0), s.ad_value(167)), A::scale(s.ad_value(257), 2.0))), 1e-38))));
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1837] != 0.0))) {
            s.store_ad(170, &A::limited_exp(s.ad_value(175)));
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1837] != 0.0))) {
            s.store_div_from_scalar(258, 1.0, 257);
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1837] != 0.0))) {
            s.store_sub_ad_lhs(171, A::add(A::scale(s.ad_value(170), 2.0), A::ln(A::max_with_scalar(A::mul(A::mul(A::scale(s.ad_value(170), 2.0), s.ad_value(167)), A::add(A::mul(A::scale(s.ad_value(170), 2.0), s.ad_value(167)), A::scale(s.ad_value(257), 2.0))), 1e-38))), 168);
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1837] != 0.0))) {
            s.store_add_ad(172, A::offset(A::div_from_scalar(1.0, s.ad_value(170)), 2.0), A::div(A::add(s.ad_value(167), s.ad_value(258)), A::add(A::mul(s.ad_value(167), s.ad_value(170)), s.ad_value(257))));
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1837] != 0.0))) {
            s.store_sub_ad_rhs(170, 170, A::div(s.ad_value(171), s.ad_value(172)));
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1837] != 0.0))) {
            s.store_sub_ad_lhs(171, A::add(A::scale(s.ad_value(170), 2.0), A::ln(A::max_with_scalar(A::mul(A::mul(A::scale(s.ad_value(170), 2.0), s.ad_value(167)), A::add(A::mul(A::scale(s.ad_value(170), 2.0), s.ad_value(167)), A::scale(s.ad_value(257), 2.0))), 1e-38))), 168);
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1837] != 0.0))) {
            s.store_add_ad(172, A::offset(A::div_from_scalar(1.0, s.ad_value(170)), 2.0), A::div(A::add(s.ad_value(167), s.ad_value(258)), A::add(A::mul(s.ad_value(167), s.ad_value(170)), s.ad_value(257))));
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1837] != 0.0))) {
            s.store_mul_ad(173, A::div(A::add(s.ad_value(167), s.ad_value(258)), A::add(A::mul(s.ad_value(167), s.ad_value(170)), s.ad_value(257))), A::div(A::add(s.ad_value(167), s.ad_value(258)), A::add(A::mul(s.ad_value(167), s.ad_value(170)), s.ad_value(257))));
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1837] != 0.0))) {
            s.store_sub_ad_lhs(174, A::sub(A::neg(A::mul(A::div_from_scalar(1.0, s.ad_value(170)), A::div_from_scalar(1.0, s.ad_value(170)))), A::div_from_scalar(1.0, A::mul(A::mul(A::square(s.ad_value(257)), s.ad_value(257)), A::add(A::mul(s.ad_value(167), s.ad_value(170)), s.ad_value(257))))), 173);
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1837] != 0.0))) {
            s.store_sub_ad_rhs(400, 170, A::mul(A::div(s.ad_value(171), s.ad_value(172)), A::offset(A::div(A::mul(s.ad_value(171), s.ad_value(174)), A::mul(A::scale(s.ad_value(172), 2.0), s.ad_value(172))), 1.0)));
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_scale_ad(256, A::add(A::offset(s.ad_value(254), 1.0), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(254), (-1.0)), A::offset(s.ad_value(254), (-1.0))), ((0.25 * 2.0) * 2.0)))), 0.5);
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_sqrt(259, 256);
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_sub_ad_rhs(255, 254, A::scale(s.ad_value(400), 2.0));
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_scale_ad(167, A::add(A::offset(s.ad_value(255), 1.0), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(255), (-1.0)), A::offset(s.ad_value(255), (-1.0))), ((0.25 * 2.0) * 2.0)))), 0.5);
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_offset_ad(253, A::div(s.ad_value(294), A::add(s.ad_value(259), A::sqrt(s.ad_value(167)))), 1.0);
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_scalar(335, (1e-8 / (s.v[200] * p.p76)));
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_mul_ad_rhs(167, 269, A::sub(A::sub(s.ad_value(213), s.ad_value(254)), A::mul(A::scale(s.ad_value(400), 2.0), A::offset(s.ad_value(253), (-1.0)))));
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_scale_ad(247, A::add(s.ad_value(167), A::sqrt(A::offset(A::mul(s.ad_value(167), s.ad_value(167)), ((0.25 * 0.1) * 0.1)))), 0.5);
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_mul_ad_lhs(306, A::mul(A::scale(s.ad_value(253), 2.0), s.ad_value(269)), 400);
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_mul_ad_rhs(308, 335, A::add(s.ad_value(247), A::scale(s.ad_value(306), s.v[338])));
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_ad(169, &A::pow(A::scale(A::offset(A::div(s.ad_value(306), s.ad_value(247)), 1.0), 0.5), s.ad_value(757)));
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_add_ad(170, A::mul(A::add(s.ad_value(750), A::mul(s.ad_value(760), s.ad_value(218))), A::pow(s.ad_value(308), s.ad_value(651))), A::div(s.ad_value(754), s.ad_value(169)));
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_offset(171, 170, 1.0);
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_scale_ad(309, A::add(A::offset(s.ad_value(171), 1.0), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(171), (-1.0)), A::offset(s.ad_value(171), (-1.0))), ((0.25 * 0.0015) * 0.0015)))), 0.5);
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_div_from_scalar_ad(448, 1.0, A::scale(A::pow_from_scalar((s.v[183] * 1000000.0), s.ad_value(771)), p.p2));
        }

        s.v[1840] = if (p.p33 == 1.0) { 1.0 } else { 0.0 };

        if ((!(s.v[1620] != 0.0)) && (s.v[1840] != 0.0)) {
            s.store_scalar(456, 0.0);
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1840] != 0.0))) {
            s.store_offset_ad(167, A::mul(s.ad_value(770), s.ad_value(306)), 1.0);
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1840] != 0.0))) {
            s.store_mul_ad_rhs(168, 787, A::sub(s.ad_value(274), s.ad_value(299)));
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1840] != 0.0))) {
            s.store_add_ad_lhs(169, A::div_from_scalar(1.0, s.ad_value(167)), 168);
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1840] != 0.0))) {
            s.store_add_ad_rhs(170, 169, A::sqrt(A::offset(A::square(s.ad_value(169)), 0.01)));
        }

        s.v[1841] = if (p.p33 == 0.0) { 1.0 } else { 0.0 };

        if (((!(s.v[1620] != 0.0)) && (!(s.v[1840] != 0.0))) && (s.v[1841] != 0.0)) {
            s.store_mul_ad_lhs(456, A::scale(A::mul(A::add(s.ad_value(777), A::mul(s.ad_value(776), s.ad_value(170))), s.ad_value(448)), p.p2), 652);
        }

        if (((!(s.v[1620] != 0.0)) && (!(s.v[1840] != 0.0))) && (!(s.v[1841] != 0.0))) {
            s.store_mul_ad_lhs(456, A::add(A::add(s.ad_value(452), A::scale(A::mul(A::add(s.ad_value(777), A::mul(s.ad_value(776), s.ad_value(170))), s.ad_value(448)), p.p2)), s.ad_value(453)), 652);
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_ad(167, &A::pow(s.ad_value(309), A::div_from_scalar(1.0, s.ad_value(348))));
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_mul(178, 678, 218);
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_sqrt_ad(179, A::offset(A::square(s.ad_value(178)), 0.1));
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_scale_ad(168, A::add(A::sub_from_scalar(1.0, s.ad_value(178)), A::sqrt(A::add(A::mul(A::sub_from_scalar(1.0, s.ad_value(178)), A::sub_from_scalar(1.0, s.ad_value(178))), s.ad_value(179)))), 0.5);
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_div_ad(169, A::mul(A::scale(s.ad_value(400), (10.0 * p.p497)), s.ad_value(168)), A::offset(A::mul(s.ad_value(400), s.ad_value(168)), (10.0 * p.p497)));
        }

        s.v[1842] = if (s.v[780] < 0.0) { 1.0 } else { 0.0 };

        if ((!(s.v[1620] != 0.0)) && (s.v[1842] != 0.0)) {
            s.store_mul_ad(314, A::scale(A::div(A::mul(A::div(s.ad_value(740), s.ad_value(167)), s.ad_value(269)), A::scale(s.ad_value(746), s.v[184])), 2.0), A::div_from_scalar(1.0, A::sub_from_scalar(1.0, A::mul(s.ad_value(780), s.ad_value(169)))));
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1842] != 0.0))) {
            s.store_mul_ad(314, A::scale(A::div(A::mul(A::div(s.ad_value(740), s.ad_value(167)), s.ad_value(269)), A::scale(s.ad_value(746), s.v[184])), 2.0), A::offset(A::mul(s.ad_value(780), s.ad_value(169)), 1.0));
        }

        s.v[1843] = if (s.v[456] > 0.0) { 1.0 } else { 0.0 };

        if ((!(s.v[1620] != 0.0)) && (s.v[1843] != 0.0)) {
            s.store_mul_ad_lhs(178, A::mul(A::scale(s.ad_value(253), ((s.v[183] * 2.0) * s.v[199])), s.ad_value(269)), 746);
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1843] != 0.0)) {
            s.store_div_ad(179, A::mul(A::mul(s.ad_value(178), s.ad_value(314)), s.ad_value(456)), A::scale(s.ad_value(269), 2.0));
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1843] != 0.0)) {
            s.store_div_ad(167, A::mul(A::scale(s.ad_value(314), 0.5), A::add(A::square(s.ad_value(400)), s.ad_value(400))), A::offset(A::mul(A::scale(s.ad_value(314), 0.5), A::offset(s.ad_value(400), 1.0)), 1.0));
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1843] != 0.0)) {
            s.store_mul_ad(168, A::scale(s.ad_value(314), 2.0), A::sub(s.ad_value(400), s.ad_value(167)));
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1843] != 0.0)) {
            s.store_sqrt_ad(169, A::offset(A::square(s.ad_value(168)), 1.0));
        }

        s.v[1844] = if (s.v[168] != 0.0) { 1.0 } else { 0.0 };

        if (((!(s.v[1620] != 0.0)) && (s.v[1843] != 0.0)) && (s.v[1844] != 0.0)) {
            s.store_asinh(323, 168);
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1843] != 0.0)) && (s.v[1844] != 0.0)) {
            s.store_add_ad_rhs(170, 169, A::mul(A::div_from_scalar(1.0, s.ad_value(168)), s.ad_value(323)));
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1843] != 0.0)) && (!(s.v[1844] != 0.0))) {
            s.store_add_ad_rhs(170, 169, A::div_from_scalar(1.0, s.ad_value(169)));
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1843] != 0.0)) {
            s.store_sub_ad(171, A::add(A::mul(s.ad_value(167), s.ad_value(170)), A::mul(A::mul(s.ad_value(179), s.ad_value(167)), A::offset(A::add(s.ad_value(400), s.ad_value(167)), 1.0))), A::mul(s.ad_value(314), A::sub(A::add(A::square(s.ad_value(400)), s.ad_value(400)), A::add(A::square(s.ad_value(167)), s.ad_value(167)))));
        }

        s.v[1845] = if (s.v[168] != 0.0) { 1.0 } else { 0.0 };

        if (((!(s.v[1620] != 0.0)) && (s.v[1843] != 0.0)) && (s.v[1845] != 0.0)) {
            s.store_div_ad(172, A::mul(A::scale(s.ad_value(314), (-2.0)), A::sub(A::mul(s.ad_value(168), s.ad_value(169)), s.ad_value(323))), A::square(s.ad_value(168)));
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1843] != 0.0)) && (!(s.v[1845] != 0.0))) {
            s.store_mul_ad(172, A::scale(s.ad_value(314), (-2.0)), A::div(s.ad_value(168), s.ad_value(169)));
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1843] != 0.0)) {
            s.store_add_ad(173, A::add(A::add(A::mul(s.ad_value(167), s.ad_value(172)), s.ad_value(170)), A::mul(s.ad_value(179), A::offset(A::add(s.ad_value(400), A::scale(s.ad_value(167), 2.0)), 1.0))), A::mul(s.ad_value(314), A::offset(A::scale(s.ad_value(167), 2.0), 1.0)));
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1843] != 0.0)) {
            s.store_sub_ad_rhs(167, 167, A::div(s.ad_value(171), s.ad_value(173)));
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1843] != 0.0)) {
            s.store_mul_ad(168, A::scale(s.ad_value(314), 2.0), A::sub(s.ad_value(400), s.ad_value(167)));
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1843] != 0.0)) {
            s.store_sqrt_ad(169, A::offset(A::square(s.ad_value(168)), 1.0));
        }

        s.v[1846] = if (s.v[168] != 0.0) { 1.0 } else { 0.0 };

        if (((!(s.v[1620] != 0.0)) && (s.v[1843] != 0.0)) && (s.v[1846] != 0.0)) {
            s.store_asinh(323, 168);
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1843] != 0.0)) && (s.v[1846] != 0.0)) {
            s.store_add_ad_rhs(170, 169, A::mul(A::div_from_scalar(1.0, s.ad_value(168)), s.ad_value(323)));
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1843] != 0.0)) && (!(s.v[1846] != 0.0))) {
            s.store_add_ad_rhs(170, 169, A::div_from_scalar(1.0, s.ad_value(169)));
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1843] != 0.0)) {
            s.store_sub_ad(171, A::add(A::mul(s.ad_value(167), s.ad_value(170)), A::mul(A::mul(s.ad_value(179), s.ad_value(167)), A::offset(A::add(s.ad_value(400), s.ad_value(167)), 1.0))), A::mul(s.ad_value(314), A::sub(A::add(A::square(s.ad_value(400)), s.ad_value(400)), A::add(A::square(s.ad_value(167)), s.ad_value(167)))));
        }

        s.v[1847] = if (s.v[168] != 0.0) { 1.0 } else { 0.0 };

        if (((!(s.v[1620] != 0.0)) && (s.v[1843] != 0.0)) && (s.v[1847] != 0.0)) {
            s.store_div_ad(172, A::mul(A::scale(s.ad_value(314), (-2.0)), A::sub(A::mul(s.ad_value(168), s.ad_value(169)), s.ad_value(323))), A::square(s.ad_value(168)));
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1843] != 0.0)) && (!(s.v[1847] != 0.0))) {
            s.store_mul_ad(172, A::scale(s.ad_value(314), (-2.0)), A::div(s.ad_value(168), s.ad_value(169)));
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1843] != 0.0)) {
            s.store_add_ad(173, A::add(A::add(A::mul(s.ad_value(167), s.ad_value(172)), s.ad_value(170)), A::mul(s.ad_value(179), A::offset(A::add(s.ad_value(400), A::scale(s.ad_value(167), 2.0)), 1.0))), A::mul(s.ad_value(314), A::offset(A::scale(s.ad_value(167), 2.0), 1.0)));
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1843] != 0.0)) {
            s.store_sub_ad_rhs(307, 167, A::div(s.ad_value(171), s.ad_value(173)));
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1843] != 0.0))) {
            s.store_div_ad(167, A::mul(A::scale(s.ad_value(314), 0.5), A::add(A::square(s.ad_value(400)), s.ad_value(400))), A::offset(A::mul(A::scale(s.ad_value(314), 0.5), A::offset(s.ad_value(400), 1.0)), 1.0));
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1843] != 0.0))) {
            s.store_mul_ad(168, A::scale(s.ad_value(314), 2.0), A::sub(s.ad_value(400), s.ad_value(167)));
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1843] != 0.0))) {
            s.store_sqrt_ad(169, A::offset(A::square(s.ad_value(168)), 1.0));
        }

        s.v[1848] = if (s.v[168] != 0.0) { 1.0 } else { 0.0 };

        if (((!(s.v[1620] != 0.0)) && (!(s.v[1843] != 0.0))) && (s.v[1848] != 0.0)) {
            s.store_asinh(323, 168);
        }

        if (((!(s.v[1620] != 0.0)) && (!(s.v[1843] != 0.0))) && (s.v[1848] != 0.0)) {
            s.store_add_ad_rhs(170, 169, A::mul(A::div_from_scalar(1.0, s.ad_value(168)), s.ad_value(323)));
        }

        if (((!(s.v[1620] != 0.0)) && (!(s.v[1843] != 0.0))) && (!(s.v[1848] != 0.0))) {
            s.store_add_ad_rhs(170, 169, A::div_from_scalar(1.0, s.ad_value(169)));
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1843] != 0.0))) {
            s.store_sub_ad(171, A::mul(s.ad_value(167), s.ad_value(170)), A::mul(s.ad_value(314), A::sub(A::add(A::square(s.ad_value(400)), s.ad_value(400)), A::add(A::square(s.ad_value(167)), s.ad_value(167)))));
        }

        s.v[1849] = if (s.v[168] != 0.0) { 1.0 } else { 0.0 };

        if (((!(s.v[1620] != 0.0)) && (!(s.v[1843] != 0.0))) && (s.v[1849] != 0.0)) {
            s.store_div_ad(172, A::mul(A::scale(s.ad_value(314), (-2.0)), A::sub(A::mul(s.ad_value(168), s.ad_value(169)), s.ad_value(323))), A::square(s.ad_value(168)));
        }

        if (((!(s.v[1620] != 0.0)) && (!(s.v[1843] != 0.0))) && (!(s.v[1849] != 0.0))) {
            s.store_mul_ad(172, A::scale(s.ad_value(314), (-2.0)), A::div(s.ad_value(168), s.ad_value(169)));
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1843] != 0.0))) {
            s.store_add_ad(173, A::add(A::mul(s.ad_value(167), s.ad_value(172)), s.ad_value(170)), A::mul(s.ad_value(314), A::offset(A::scale(s.ad_value(167), 2.0), 1.0)));
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1843] != 0.0))) {
            s.store_sub_ad_rhs(167, 167, A::div(s.ad_value(171), s.ad_value(173)));
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1843] != 0.0))) {
            s.store_mul_ad(168, A::scale(s.ad_value(314), 2.0), A::sub(s.ad_value(400), s.ad_value(167)));
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1843] != 0.0))) {
            s.store_sqrt_ad(169, A::offset(A::square(s.ad_value(168)), 1.0));
        }

        s.v[1850] = if (s.v[168] != 0.0) { 1.0 } else { 0.0 };

        if (((!(s.v[1620] != 0.0)) && (!(s.v[1843] != 0.0))) && (s.v[1850] != 0.0)) {
            s.store_asinh(323, 168);
        }

        if (((!(s.v[1620] != 0.0)) && (!(s.v[1843] != 0.0))) && (s.v[1850] != 0.0)) {
            s.store_add_ad_rhs(170, 169, A::mul(A::div_from_scalar(1.0, s.ad_value(168)), s.ad_value(323)));
        }

        if (((!(s.v[1620] != 0.0)) && (!(s.v[1843] != 0.0))) && (!(s.v[1850] != 0.0))) {
            s.store_add_ad_rhs(170, 169, A::div_from_scalar(1.0, s.ad_value(169)));
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1843] != 0.0))) {
            s.store_sub_ad(171, A::mul(s.ad_value(167), s.ad_value(170)), A::mul(s.ad_value(314), A::sub(A::add(A::square(s.ad_value(400)), s.ad_value(400)), A::add(A::square(s.ad_value(167)), s.ad_value(167)))));
        }

        s.v[1851] = if (s.v[168] != 0.0) { 1.0 } else { 0.0 };

        if (((!(s.v[1620] != 0.0)) && (!(s.v[1843] != 0.0))) && (s.v[1851] != 0.0)) {
            s.store_div_ad(172, A::mul(A::scale(s.ad_value(314), (-2.0)), A::sub(A::mul(s.ad_value(168), s.ad_value(169)), s.ad_value(323))), A::square(s.ad_value(168)));
        }

        if (((!(s.v[1620] != 0.0)) && (!(s.v[1843] != 0.0))) && (!(s.v[1851] != 0.0))) {
            s.store_mul_ad(172, A::scale(s.ad_value(314), (-2.0)), A::div(s.ad_value(168), s.ad_value(169)));
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1843] != 0.0))) {
            s.store_add_ad(173, A::add(A::mul(s.ad_value(167), s.ad_value(172)), s.ad_value(170)), A::mul(s.ad_value(314), A::offset(A::scale(s.ad_value(167), 2.0), 1.0)));
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1843] != 0.0))) {
            s.store_sub_ad_rhs(307, 167, A::div(s.ad_value(171), s.ad_value(173)));
        }

        if (!(s.v[1620] != 0.0)) {
            let assign46440_ad_e78990: A = A::sub(A::sub(s.ad_value(254), A::scale(s.ad_value(252), 2.0)), A::add(A::scale(s.ad_value(307), 2.0), A::ln(A::max_with_scalar(A::mul(A::mul(A::mul(A::scale(s.ad_value(307), 2.0), s.ad_value(253)), s.ad_value(295)), A::add(A::mul(A::mul(A::scale(s.ad_value(307), 2.0), s.ad_value(253)), s.ad_value(295)), A::div(s.ad_value(294), A::offset(s.ad_value(253), (-1.0))))), 1e-38))));
            s.store_ad(319, &assign46440_ad_e78990);
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_mul(312, 319, 269);
        }

        s.v[1852] = if ((p.p1349 == 0.0) && (p.p1350 == 0.0)) { 1.0 } else { 0.0 };

        if ((!(s.v[1620] != 0.0)) && (s.v[1852] != 0.0)) {
            s.store_scalar(1019, 1.0);
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1852] != 0.0))) {
            s.store_div_from_scalar_ad(168, s.v[184], A::offset(A::sqrt(A::mul(s.ad_value(782), s.ad_value(275))), s.v[184]));
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1852] != 0.0))) {
            s.store_offset_ad(1019, A::div(A::sub(A::scale(s.ad_value(168), p.p1349), A::mul(A::mul(A::scale(s.ad_value(168), p.p1350), A::powf(s.ad_value(400), p.p1351)), s.ad_value(269))), A::offset(A::scale(s.ad_value(218), p.p1352), 1.0)), 1.0);
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1852] != 0.0))) {
            s.store_scale_ad(1019, A::add(A::offset(s.ad_value(1019), 0.1), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(1019), (-0.1)), A::offset(s.ad_value(1019), (-0.1))), ((0.25 * 0.0005) * 0.0005)))), 0.5);
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_scale_ad(316, A::add(A::sub(s.ad_value(312), s.ad_value(224)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(312), s.ad_value(224)), A::sub(s.ad_value(312), s.ad_value(224))), ((0.25 * 0.001) * 0.001)))), 0.5);
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_div(316, 316, 1019);
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_pow_ad(174, A::offset(A::div(s.ad_value(226), s.ad_value(316)), 1e-6), A::div_from_scalar(1.0, s.ad_value(656)));
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_pow_ad(175, A::offset(s.ad_value(174), 1.0), A::neg(s.ad_value(656)));
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_mul(315, 226, 175);
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_mul_ad_lhs(318, A::add(s.ad_value(315), s.ad_value(224)), 270);
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_scale_ad(175, A::add(A::offset(s.ad_value(254), 1.0), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(254), (-1.0)), A::offset(s.ad_value(254), (-1.0))), ((0.25 * 2.0) * 2.0)))), 0.5);
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_sqrt(259, 175);
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_div_ad_lhs(167, A::offset(A::div(s.ad_value(294), A::scale(s.ad_value(259), 2.0)), 1.0), 294);
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_sub_ad_lhs(168, A::sub(s.ad_value(254), A::scale(s.ad_value(252), 2.0)), 318);
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_sub_ad_rhs(169, 168, A::ln(A::max_with_scalar(A::mul(A::scale(s.ad_value(167), 4.0), s.ad_value(259)), 1e-38)));
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_scale_ad(175, A::sub(A::offset(s.ad_value(169), (-0.201491)), A::sqrt(A::offset(A::mul(s.ad_value(169), A::offset(s.ad_value(169), 0.402982)), 2.446562))), 0.5);
        }

        if (!(s.v[1620] != 0.0)) {
            s.copy_ad(257, 259);
        }

        s.v[1853] = if (s.v[175] <= (-68.0)) { 1.0 } else { 0.0 };

        if ((!(s.v[1620] != 0.0)) && (s.v[1853] != 0.0)) {
            s.store_scalar(171, (-100.0));
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1853] != 0.0)) {
            s.store_scalar(172, 20.0);
        }

        s.v[1854] = if (s.v[175] < (s.v[171] - (0.5 * s.v[172]))) { 1.0 } else { 0.0 };

        if (((!(s.v[1620] != 0.0)) && (s.v[1853] != 0.0)) && (s.v[1854] != 0.0)) {
            s.store_ad(170, &A::limited_exp(s.ad_value(171)));
        }

        s.v[1855] = if (s.v[175] > (s.v[171] + (0.5 * s.v[172]))) { 1.0 } else { 0.0 };

        if ((((!(s.v[1620] != 0.0)) && (s.v[1853] != 0.0)) && (!(s.v[1854] != 0.0))) && (s.v[1855] != 0.0)) {
            s.store_ad(170, &A::limited_exp(s.ad_value(175)));
        }

        if ((((!(s.v[1620] != 0.0)) && (s.v[1853] != 0.0)) && (!(s.v[1854] != 0.0))) && (!(s.v[1855] != 0.0))) {
            s.store_div_ad_lhs(169, A::sub(s.ad_value(175), s.ad_value(171)), 172);
        }

        if ((((!(s.v[1620] != 0.0)) && (s.v[1853] != 0.0)) && (!(s.v[1854] != 0.0))) && (!(s.v[1855] != 0.0))) {
            s.store_square(173, 169);
        }

        if ((((!(s.v[1620] != 0.0)) && (s.v[1853] != 0.0)) && (!(s.v[1854] != 0.0))) && (!(s.v[1855] != 0.0))) {
            s.store_limited_exp_ad(170, A::add(s.ad_value(171), A::mul(s.ad_value(172), A::add(A::offset(A::scale(s.ad_value(169), 0.5), (5.0 / 64.0)), A::mul(s.ad_value(173), A::sub_from_scalar((15.0 / 16.0), A::mul(s.ad_value(173), A::sub_from_scalar(1.25, s.ad_value(173)))))))));
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1853] != 0.0)) {
            s.store_mul_ad_rhs(320, 170, A::sub(A::sub(A::offset(s.ad_value(168), 1.0), s.ad_value(175)), A::ln(A::max_with_scalar(A::mul(A::scale(s.ad_value(167), 2.0), A::add(A::mul(A::scale(s.ad_value(170), 2.0), s.ad_value(167)), A::scale(s.ad_value(257), 2.0))), 1e-38))));
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1853] != 0.0))) {
            s.store_ad(170, &A::limited_exp(s.ad_value(175)));
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1853] != 0.0))) {
            s.store_div_from_scalar(258, 1.0, 257);
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1853] != 0.0))) {
            s.store_sub_ad_lhs(171, A::add(A::scale(s.ad_value(170), 2.0), A::ln(A::max_with_scalar(A::mul(A::mul(A::scale(s.ad_value(170), 2.0), s.ad_value(167)), A::add(A::mul(A::scale(s.ad_value(170), 2.0), s.ad_value(167)), A::scale(s.ad_value(257), 2.0))), 1e-38))), 168);
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1853] != 0.0))) {
            s.store_add_ad(172, A::offset(A::div_from_scalar(1.0, s.ad_value(170)), 2.0), A::div(A::add(s.ad_value(167), s.ad_value(258)), A::add(A::mul(s.ad_value(167), s.ad_value(170)), s.ad_value(257))));
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1853] != 0.0))) {
            s.store_sub_ad_rhs(170, 170, A::div(s.ad_value(171), s.ad_value(172)));
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1853] != 0.0))) {
            s.store_sub_ad_lhs(171, A::add(A::scale(s.ad_value(170), 2.0), A::ln(A::max_with_scalar(A::mul(A::mul(A::scale(s.ad_value(170), 2.0), s.ad_value(167)), A::add(A::mul(A::scale(s.ad_value(170), 2.0), s.ad_value(167)), A::scale(s.ad_value(257), 2.0))), 1e-38))), 168);
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1853] != 0.0))) {
            s.store_add_ad(172, A::offset(A::div_from_scalar(1.0, s.ad_value(170)), 2.0), A::div(A::add(s.ad_value(167), s.ad_value(258)), A::add(A::mul(s.ad_value(167), s.ad_value(170)), s.ad_value(257))));
        }

    }

    pub(super) fn stamp_transient_block_33(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if ((!(s.v[1620] != 0.0)) && (!(s.v[1853] != 0.0))) {
            s.store_mul_ad(173, A::div(A::add(s.ad_value(167), s.ad_value(258)), A::add(A::mul(s.ad_value(167), s.ad_value(170)), s.ad_value(257))), A::div(A::add(s.ad_value(167), s.ad_value(258)), A::add(A::mul(s.ad_value(167), s.ad_value(170)), s.ad_value(257))));
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1853] != 0.0))) {
            s.store_sub_ad_lhs(174, A::sub(A::neg(A::mul(A::div_from_scalar(1.0, s.ad_value(170)), A::div_from_scalar(1.0, s.ad_value(170)))), A::div_from_scalar(1.0, A::mul(A::mul(A::square(s.ad_value(257)), s.ad_value(257)), A::add(A::mul(s.ad_value(167), s.ad_value(170)), s.ad_value(257))))), 173);
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1853] != 0.0))) {
            s.store_sub_ad_rhs(320, 170, A::mul(A::div(s.ad_value(171), s.ad_value(172)), A::offset(A::div(A::mul(s.ad_value(171), s.ad_value(174)), A::mul(A::scale(s.ad_value(172), 2.0), s.ad_value(172))), 1.0)));
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_offset_ad(255, A::sub(A::sub(s.ad_value(254), s.ad_value(400)), s.ad_value(320)), (-1.0));
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_scale_ad(167, A::add(A::offset(s.ad_value(255), 1.0), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(255), (-1.0)), A::offset(s.ad_value(255), (-1.0))), ((0.25 * 2.0) * 2.0)))), 0.5);
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_sqrt(169, 167);
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_offset_ad(253, A::div(s.ad_value(294), A::add(s.ad_value(259), s.ad_value(169))), 1.0);
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_mul_ad(417, A::sub(s.ad_value(400), s.ad_value(320)), A::sub(s.ad_value(400), s.ad_value(320)));
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_div_from_scalar_ad(167, 1.0, A::add(A::offset(s.ad_value(400), 1.0), s.ad_value(320)));
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_mul(168, 417, 167);
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_sub_ad(381, A::sub(s.ad_value(213), s.ad_value(254)), A::mul(A::offset(s.ad_value(253), (-1.0)), A::add(A::add(s.ad_value(400), s.ad_value(320)), A::scale(s.ad_value(168), 0.3333333333333333))));
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_scale(169, 253, 0.3333333333333333);
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_mul(170, 168, 167);
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_mul_ad_rhs(382, 169, A::add(A::add(A::scale(s.ad_value(400), 2.0), s.ad_value(320)), A::mul(A::scale(A::add(A::offset(A::scale(s.ad_value(400), 0.8), 1.0), A::scale(s.ad_value(320), 1.2)), 0.5), s.ad_value(170))));
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_mul_ad_rhs(385, 169, A::add(A::add(s.ad_value(400), A::scale(s.ad_value(320), 2.0)), A::mul(A::scale(A::add(A::offset(A::scale(s.ad_value(400), 1.2), 1.0), A::scale(s.ad_value(320), 0.8)), 0.5), s.ad_value(170))));
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_scale_ad(244, A::add(A::mul(s.ad_value(269), s.ad_value(381)), A::sqrt(A::offset(A::mul(A::mul(s.ad_value(269), s.ad_value(381)), A::mul(s.ad_value(269), s.ad_value(381))), ((0.25 * 0.1) * 0.1)))), 0.5);
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_mul_ad_rhs(243, 269, A::add(s.ad_value(382), s.ad_value(385)));
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_mul_ad_rhs(336, 335, A::add(s.ad_value(244), A::scale(s.ad_value(243), s.v[338])));
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_ad(169, &A::pow(A::scale(A::offset(A::div(s.ad_value(243), s.ad_value(244)), 1.0), 0.5), s.ad_value(757)));
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_add_ad(170, A::mul(A::add(s.ad_value(750), A::mul(s.ad_value(760), s.ad_value(218))), A::pow(s.ad_value(336), s.ad_value(651))), A::div(s.ad_value(754), s.ad_value(169)));
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_offset(171, 170, 1.0);
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_scale_ad(339, A::add(A::offset(s.ad_value(171), 1.0), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(171), (-1.0)), A::offset(s.ad_value(171), (-1.0))), ((0.25 * 0.0015) * 0.0015)))), 0.5);
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_div_ad(310, A::scale(s.ad_value(746), 2.0), A::div(s.ad_value(740), s.ad_value(339)));
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_scale(311, 310, s.v[184]);
        }

        s.v[1856] = if (s.v[781] > 0.0) { 1.0 } else { 0.0 };

        if ((!(s.v[1620] != 0.0)) && (s.v[1856] != 0.0)) {
            s.store_offset_ad(360, A::div(A::mul(s.ad_value(781), s.ad_value(243)), s.ad_value(311)), 1.0);
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1856] != 0.0))) {
            s.store_div_from_scalar_ad(360, 1.0, A::sub_from_scalar(1.0, A::div(A::mul(s.ad_value(781), s.ad_value(243)), s.ad_value(311))));
        }

        if (!(s.v[1620] != 0.0)) {
            s.copy_ad(359, 763);
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_sub(355, 226, 315);
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_add_ad_rhs(362, 243, A::scale(s.ad_value(269), 2.0));
        }

        s.v[1857] = if (s.v[359] > 0.0) { 1.0 } else { 0.0 };

        if ((!(s.v[1620] != 0.0)) && (s.v[1857] != 0.0)) {
            s.store_div_ad_rhs(170, 362, A::add(s.ad_value(316), s.ad_value(362)));
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1857] != 0.0)) {
            s.store_scale_ad(171, A::add(A::offset(A::mul(s.ad_value(764), s.ad_value(218)), 1.0), A::sqrt(A::offset(A::mul(A::offset(A::mul(s.ad_value(764), s.ad_value(218)), 1.0), A::offset(A::mul(s.ad_value(764), s.ad_value(218)), 1.0)), ((4.0 * 0.001) * 0.001)))), 0.5);
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1857] != 0.0)) {
            s.store_div_from_scalar(172, 1.0, 171);
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1857] != 0.0)) {
            s.store_mul_ad_lhs(361, A::mul(A::mul(A::div(s.ad_value(362), s.ad_value(359)), s.ad_value(170)), s.ad_value(360)), 172);
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1857] != 0.0)) {
            s.store_offset_ad(363, A::div(s.ad_value(355), s.ad_value(361)), 1.0);
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1857] != 0.0))) {
            s.store_scalar(363, 1.0);
        }

        s.v[1858] = if (s.v[769] <= 0.0) { 1.0 } else { 0.0 };

        if ((!(s.v[1620] != 0.0)) && (s.v[1858] != 0.0)) {
            s.store_scalar(268, 1.0);
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1858] != 0.0))) {
            s.store_div_ad_lhs(176, A::scale(s.ad_value(769), ((s.v[184]) as f64).sqrt()), 362);
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1858] != 0.0))) {
            s.store_div_from_scalar_ad(268, 1.0, A::offset(s.ad_value(176), 1.0));
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_add(358, 316, 311);
        }

        s.v[1859] = if (s.v[785] > 0.0) { 1.0 } else { 0.0 };

        s.v[1860] = if (p.p414 < 0.0) { 1.0 } else { 0.0 };

        if (((!(s.v[1620] != 0.0)) && (s.v[1859] != 0.0)) && (s.v[1860] != 0.0)) {
            s.store_div_ad_lhs(168, A::div(s.ad_value(785), A::sub_from_scalar(1.0, A::div(A::scale(s.ad_value(243), p.p414), s.ad_value(311)))), 268);
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1859] != 0.0)) && (!(s.v[1860] != 0.0))) {
            s.store_div_ad_lhs(168, A::mul(s.ad_value(785), A::offset(A::div(A::scale(s.ad_value(243), p.p414), s.ad_value(311)), 1.0)), 268);
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1859] != 0.0)) {
            s.store_offset_ad(364, A::mul(s.ad_value(168), A::ln(A::max_with_scalar(A::offset(A::div(A::div(s.ad_value(355), s.ad_value(168)), s.ad_value(358)), 1.0), 1e-38))), 1.0);
        }

        s.v[1861] = if (p.p414 < 0.0) { 1.0 } else { 0.0 };

        if (((!(s.v[1620] != 0.0)) && (!(s.v[1859] != 0.0))) && (s.v[1861] != 0.0)) {
            s.store_div_ad_lhs(168, A::div(s.ad_value(785), A::sub_from_scalar(1.0, A::div(A::scale(s.ad_value(243), p.p414), s.ad_value(311)))), 268);
        }

        if (((!(s.v[1620] != 0.0)) && (!(s.v[1859] != 0.0))) && (!(s.v[1861] != 0.0))) {
            s.store_div_ad_lhs(168, A::mul(s.ad_value(785), A::offset(A::div(A::scale(s.ad_value(243), p.p414), s.ad_value(311)), 1.0)), 268);
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1859] != 0.0))) {
            s.store_offset(364, 168, 1.0);
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_mul(363, 363, 364);
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_limited_exp_ad(168, A::mul(s.ad_value(768), s.ad_value(226)));
        }

        s.v[1862] = if (s.v[767] > 0.0) { 1.0 } else { 0.0 };

        if ((!(s.v[1620] != 0.0)) && (s.v[1862] != 0.0)) {
            s.store_scalar(169, (1.0 + (p.p433 * s.v[184])));
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1862] != 0.0)) {
            s.store_div_ad_lhs(356, A::offset(A::mul(s.ad_value(169), s.ad_value(168)), 1.0), 767);
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1862] != 0.0)) {
            s.store_mul(356, 356, 268);
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1862] != 0.0))) {
            s.store_scalar(356, 5.540622384e34);
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_div(171, 355, 356);
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_offset(167, 171, 1.0);
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_mul(363, 363, 167);
        }

        s.v[1863] = if (s.v[766] > 0.0) { 1.0 } else { 0.0 };

        s.v[1864] = if (s.v[355] > ((s.v[765] * s.v[300]) / 80.0)) { 1.0 } else { 0.0 };

        if (((!(s.v[1620] != 0.0)) && (s.v[1863] != 0.0)) && (s.v[1864] != 0.0)) {
            s.store_div_ad_lhs(167, A::mul(s.ad_value(765), s.ad_value(300)), 355);
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1863] != 0.0)) && (s.v[1864] != 0.0)) {
            s.store_div_ad_lhs(357, A::scale(A::limited_exp(s.ad_value(167)), s.v[184]), 766);
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1863] != 0.0)) && (!(s.v[1864] != 0.0))) {
            s.store_div_from_scalar(357, (5.540622384e34 * s.v[184]), 766);
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1863] != 0.0))) {
            s.store_scalar(357, 5.540622384e34);
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_offset_ad(365, A::div(s.ad_value(355), s.ad_value(357)), 1.0);
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_mul(363, 363, 365);
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_ad(167, &A::pow(s.ad_value(339), A::div_from_scalar(1.0, s.ad_value(348))));
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_mul(178, 678, 218);
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_sqrt_ad(179, A::offset(A::square(s.ad_value(178)), 0.1));
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_scale_ad(168, A::add(A::sub_from_scalar(1.0, s.ad_value(178)), A::sqrt(A::add(A::mul(A::sub_from_scalar(1.0, s.ad_value(178)), A::sub_from_scalar(1.0, s.ad_value(178))), s.ad_value(179)))), 0.5);
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_div_ad(169, A::mul(A::scale(s.ad_value(243), (10.0 * p.p497)), s.ad_value(168)), A::offset(A::mul(s.ad_value(243), s.ad_value(168)), (10.0 * p.p497)));
        }

        s.v[1865] = if (s.v[780] < 0.0) { 1.0 } else { 0.0 };

        if ((!(s.v[1620] != 0.0)) && (s.v[1865] != 0.0)) {
            s.store_mul_ad(314, A::scale(A::div(A::mul(A::div(s.ad_value(740), s.ad_value(167)), s.ad_value(269)), A::scale(s.ad_value(746), s.v[184])), 2.0), A::div_from_scalar(1.0, A::sub_from_scalar(1.0, A::mul(s.ad_value(780), s.ad_value(169)))));
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1865] != 0.0))) {
            s.store_mul_ad(314, A::scale(A::div(A::mul(A::div(s.ad_value(740), s.ad_value(167)), s.ad_value(269)), A::scale(s.ad_value(746), s.v[184])), 2.0), A::offset(A::mul(s.ad_value(780), s.ad_value(169)), 1.0));
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_mul_ad(168, A::scale(s.ad_value(314), 2.0), A::sub(s.ad_value(400), s.ad_value(320)));
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_sqrt_ad(169, A::offset(A::square(s.ad_value(168)), 1.0));
        }

        s.v[1866] = if (s.v[168] != 0.0) { 1.0 } else { 0.0 };

        if ((!(s.v[1620] != 0.0)) && (s.v[1866] != 0.0)) {
            s.store_scale_ad(343, A::add(s.ad_value(169), A::mul(A::div_from_scalar(1.0, s.ad_value(168)), A::asinh(s.ad_value(168)))), 0.5);
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1866] != 0.0))) {
            s.store_scale_ad(343, A::add(s.ad_value(169), A::div_from_scalar(1.0, s.ad_value(169))), 0.5);
        }

        if (!(s.v[1620] != 0.0)) {
            s.copy_ad(345, 343);
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_scalar(454, 0.0);
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_scalar(455, 0.0);
        }

        s.v[1867] = if (p.p33 == 1.0) { 1.0 } else { 0.0 };

        if ((!(s.v[1620] != 0.0)) && (s.v[1867] != 0.0)) {
            s.store_scalar(457, 0.0);
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1867] != 0.0)) {
            s.store_scalar(458, 1.0);
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1867] != 0.0)) {
            s.store_sub(169, 203, 219);
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1867] != 0.0)) {
            s.store_sqrt_ad(170, A::offset(A::square(s.ad_value(169)), 0.01));
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1867] != 0.0)) {
            s.store_scaled_add(228, 169, 170, 0.5);
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1867] != 0.0)) {
            s.store_offset_ad(172, A::mul(s.ad_value(770), s.ad_value(228)), 1.0);
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1867] != 0.0)) {
            s.store_add_ad(173, A::div_from_scalar(1.0, s.ad_value(172)), A::mul(s.ad_value(787), s.ad_value(202)));
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1867] != 0.0)) {
            s.store_scale_ad(171, A::add(s.ad_value(173), A::sqrt(A::offset(A::square(s.ad_value(173)), 0.01))), 0.5);
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1867] != 0.0)) {
            s.store_mul_ad_rhs(454, 652, A::add(s.ad_value(452), A::mul(A::add(s.ad_value(773), A::mul(s.ad_value(775), s.ad_value(171))), s.ad_value(448))));
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1867] != 0.0)) {
            s.store_sub(169, 204, 219);
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1867] != 0.0)) {
            s.store_sqrt_ad(170, A::offset(A::square(s.ad_value(169)), 0.01));
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1867] != 0.0)) {
            s.store_scaled_add(229, 169, 170, 0.5);
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1867] != 0.0)) {
            s.store_offset_ad(172, A::mul(s.ad_value(770), s.ad_value(229)), 1.0);
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1867] != 0.0)) {
            s.store_add_ad(173, A::div_from_scalar(1.0, s.ad_value(172)), A::mul(s.ad_value(787), s.ad_value(201)));
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1867] != 0.0)) {
            s.store_scale_ad(171, A::add(s.ad_value(173), A::sqrt(A::offset(A::square(s.ad_value(173)), 0.01))), 0.5);
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1867] != 0.0)) {
            s.store_mul_ad_rhs(455, 652, A::add(s.ad_value(453), A::mul(A::add(s.ad_value(772), A::mul(s.ad_value(774), s.ad_value(171))), s.ad_value(448))));
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1867] != 0.0))) {
            s.store_offset_ad(167, A::mul(s.ad_value(770), s.ad_value(243)), 1.0);
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1867] != 0.0))) {
            s.store_mul_ad_rhs(168, 787, A::sub(s.ad_value(274), s.ad_value(299)));
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1867] != 0.0))) {
            s.store_add_ad_lhs(169, A::div_from_scalar(1.0, s.ad_value(167)), 168);
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1867] != 0.0))) {
            s.store_scale_ad(170, A::add(s.ad_value(169), A::sqrt(A::offset(A::square(s.ad_value(169)), 0.01))), 0.5);
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1867] != 0.0))) {
            s.store_scale_ad(457, A::mul(A::mul(s.ad_value(652), A::add(s.ad_value(777), A::mul(s.ad_value(776), s.ad_value(170)))), s.ad_value(448)), p.p2);
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1867] != 0.0))) {
            s.copy_ad(455, 453);
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1867] != 0.0))) {
            s.copy_ad(454, 452);
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1867] != 0.0))) {
            s.store_offset_ad(458, A::mul(A::mul(A::scale(A::div(s.ad_value(740), A::mul(s.ad_value(343), s.ad_value(339))), (s.v[199] * (s.v[183] * 1.0 / (s.v[184])))), s.ad_value(243)), s.ad_value(457)), 1.0);
        }

        s.v[1868] = if (p.p33 == 2.0) { 1.0 } else { 0.0 };

        if (((!(s.v[1620] != 0.0)) && (!(s.v[1867] != 0.0))) && (s.v[1868] != 0.0)) {
            s.store_mul_ad_rhs(457, 652, A::add(A::add(s.ad_value(452), A::scale(A::mul(A::add(s.ad_value(777), A::mul(s.ad_value(776), s.ad_value(170))), s.ad_value(448)), p.p2)), s.ad_value(453)));
        }

        if (((!(s.v[1620] != 0.0)) && (!(s.v[1867] != 0.0))) && (s.v[1868] != 0.0)) {
            s.store_scalar(455, 0.0);
        }

        if (((!(s.v[1620] != 0.0)) && (!(s.v[1867] != 0.0))) && (s.v[1868] != 0.0)) {
            s.store_scalar(454, 0.0);
        }

        if (((!(s.v[1620] != 0.0)) && (!(s.v[1867] != 0.0))) && (s.v[1868] != 0.0)) {
            s.store_offset_ad(458, A::mul(A::mul(A::scale(A::div(s.ad_value(740), A::mul(s.ad_value(343), s.ad_value(339))), (s.v[199] * (s.v[183] * 1.0 / (s.v[184])))), s.ad_value(243)), s.ad_value(457)), 1.0);
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_add_ad_rhs(167, 330, A::div(s.ad_value(333), A::add(s.ad_value(243), A::mul(A::scale(s.ad_value(267), 2.0), s.ad_value(637)))));
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_sub(416, 400, 320);
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_mul_ad_lhs(168, A::mul(s.ad_value(167), s.ad_value(416)), 416);
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_offset(169, 168, ((1.0) + ((-0.001))));
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_offset_ad(170, A::scale(A::add(s.ad_value(169), A::sqrt(A::offset(A::square(s.ad_value(169)), 0.004))), 0.5), (-1.0));
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_scale_ad(334, A::offset(A::sqrt(A::offset(s.ad_value(170), 1.0)), 1.0), 0.5);
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_offset_ad(334, A::scale(A::sub(A::offset(s.ad_value(334), 1.0), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(334), (-1.0)), A::offset(s.ad_value(334), (-1.0))), ((0.25 * 0.01) * 0.01)))), 0.5), (0.25 * 0.01));
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_add(167, 400, 320);
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_sub(168, 400, 320);
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_div_ad_rhs(169, 168, A::add(s.ad_value(167), s.ad_value(833)));
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_mul_ad_lhs(170, A::mul(s.ad_value(832), s.ad_value(169)), 169);
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_offset(834, 170, 1.0);
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_div_ad_rhs(176, 858, A::add(A::mul(A::max_from_scalar(0.0, A::add(s.ad_value(861), A::mul(A::mul(s.ad_value(864), s.ad_value(168)), s.ad_value(168)))), s.ad_value(167)), A::mul(A::scale(s.ad_value(267), 2.0), s.ad_value(637))));
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_limited_exp_ad(853, A::neg(s.ad_value(176)));
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_mul_ad_lhs(340, A::mul(s.ad_value(339), s.ad_value(343)), 458);
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_div(337, 740, 340);
        }

        if (!(s.v[1620] != 0.0)) {
            let assign48130_ad_e81153: A = A::div(A::mul(A::mul(A::mul(A::mul(A::scale(A::scale(A::mul(A::scale(s.ad_value(253), (2.0 * p.p2)), s.ad_value(337)), (s.v[183] * 1.0 / (s.v[184]))), s.v[199]), s.ad_value(269)), s.ad_value(269)), A::mul(A::sub(s.ad_value(400), s.ad_value(320)), A::add(A::offset(s.ad_value(400), 1.0), s.ad_value(320)))), s.ad_value(363)), s.ad_value(334));
            s.store_mul_ad_lhs(380, A::mul(assign48130_ad_e81153, s.ad_value(834)), 853);
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_scale(380, 380, p.p26);
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_scalar(467, 0.0);
        }

    }

    pub(super) fn stamp_transient_block_34(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        s.v[1869] = if (p.p7 > 1.0) { 1.0 } else { 0.0 };

        if ((!(s.v[1620] != 0.0)) && (s.v[1869] != 0.0)) {
            s.store_mul_ad_lhs(468, A::scale(A::scale(s.ad_value(337), (s.v[183] * 1.0 / (s.v[184]))), s.v[199]), 243);
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1869] != 0.0)) {
            s.store_scale(176, 271, p.p1009);
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1869] != 0.0)) {
            s.store_scale_ad(167, A::scale(A::mul(s.ad_value(176), s.ad_value(337)), (s.v[183] * 1.0 / (s.v[184]))), s.v[199]);
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1869] != 0.0)) {
            s.store_scaled_add(467, 167, 468, (p.p1008 * p.p2));
        }

        s.v[1870] = if (p.p7 == 2.0) { 1.0 } else { 0.0 };

        if (((!(s.v[1620] != 0.0)) && (s.v[1869] != 0.0)) && (s.v[1870] != 0.0)) {
            s.store_div_from_scalar(466, 1.0, 465);
        }

        s.v[1871] = if (s.v[466] < p.p1347) { 1.0 } else { 0.0 };

        if ((((!(s.v[1620] != 0.0)) && (s.v[1869] != 0.0)) && (s.v[1870] != 0.0)) && (s.v[1871] != 0.0)) {
            s.store_scalar(466, p.p1347);
        }

        if ((((!(s.v[1620] != 0.0)) && (s.v[1869] != 0.0)) && (s.v[1870] != 0.0)) && (s.v[1871] != 0.0)) {
            s.store_div_from_scalar(465, 1.0, 466);
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1869] != 0.0)) && (s.v[1870] != 0.0)) {
            s.store_add(178, 465, 467);
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1869] != 0.0)) && (s.v[1870] != 0.0)) {
            s.store_div_ad_lhs(467, A::mul(s.ad_value(465), s.ad_value(467)), 178);
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_scalar(544, ((s.v[183] / p.p1373) + p.p1377));
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_scalar(543, ((s.v[183] / p.p1373) + p.p1378));
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_scale(545, 543, p.p74);
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_scale(546, 544, p.p74);
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_mul(593, 637, 590);
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_div(167, 498, 593);
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_ad(595, &A::limited_exp(s.ad_value(167)));
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_mul(594, 637, 590);
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_div(167, 499, 594);
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_ad(596, &A::limited_exp(s.ad_value(167)));
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_mul_ad(171, A::div_from_scalar(1.115, s.ad_value(637)), A::offset(s.ad_value(639), (-1.0)));
        }

        s.v[1872] = if (s.v[550] == 0.0) { 1.0 } else { 0.0 };

        if ((!(s.v[1620] != 0.0)) && (s.v[1872] != 0.0)) {
            s.store_scalar(535, 0.0);
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1872] != 0.0))) {
            s.store_div_ad_lhs(174, A::mul(s.ad_value(547), s.ad_value(171)), 590);
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1872] != 0.0))) {
            s.store_ad(168, &A::limited_exp(s.ad_value(174)));
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1872] != 0.0))) {
            s.store_mul(548, 550, 168);
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1872] != 0.0))) {
            s.store_mul(167, 545, 548);
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1872] != 0.0))) {
            s.store_mul_ad_rhs(535, 167, A::offset(s.ad_value(595), (-1.0)));
        }

        s.v[1873] = if (s.v[551] == 0.0) { 1.0 } else { 0.0 };

        if ((!(s.v[1620] != 0.0)) && (s.v[1873] != 0.0)) {
            s.store_scalar(536, 0.0);
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1873] != 0.0))) {
            s.store_div_ad_lhs(174, A::mul(s.ad_value(547), s.ad_value(171)), 590);
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1873] != 0.0))) {
            s.store_ad(168, &A::limited_exp(s.ad_value(174)));
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1873] != 0.0))) {
            s.store_mul(549, 551, 168);
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1873] != 0.0))) {
            s.store_mul(167, 546, 549);
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1873] != 0.0))) {
            s.store_mul_ad_rhs(536, 167, A::offset(s.ad_value(596), (-1.0)));
        }

        s.v[1874] = if (s.v[552] == 0.0) { 1.0 } else { 0.0 };

        if ((!(s.v[1620] != 0.0)) && (s.v[1874] != 0.0)) {
            s.store_scalar(537, 0.0);
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1874] != 0.0))) {
            s.store_div_ad_lhs(174, A::mul(s.ad_value(556), s.ad_value(171)), 557);
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1874] != 0.0))) {
            s.store_ad(169, &A::limited_exp(s.ad_value(174)));
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1874] != 0.0))) {
            s.store_mul(554, 552, 169);
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1874] != 0.0))) {
            s.store_mul_ad(562, A::scale(s.ad_value(557), p.p925), A::offset(A::mul(s.ad_value(565), A::offset(s.ad_value(639), (-1.0))), 1.0));
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1874] != 0.0))) {
            s.store_mul_ad(563, A::scale(s.ad_value(564), p.p925), A::offset(A::mul(s.ad_value(566), A::offset(s.ad_value(639), (-1.0))), 1.0));
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1874] != 0.0))) {
            s.store_div(167, 498, 562);
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1874] != 0.0))) {
            s.store_ad(177, &A::limited_exp(s.ad_value(167)));
        }

        s.v[1875] = if ((s.v[558] - s.v[498]) < 0.001) { 1.0 } else { 0.0 };

        if (((!(s.v[1620] != 0.0)) && (!(s.v[1874] != 0.0))) && (s.v[1875] != 0.0)) {
            s.store_scalar(168, 1000.0);
        }

        if (((!(s.v[1620] != 0.0)) && (!(s.v[1874] != 0.0))) && (s.v[1875] != 0.0)) {
            s.store_mul_ad_lhs(167, A::mul(A::div(A::neg(s.ad_value(498)), s.ad_value(563)), s.ad_value(558)), 168);
        }

        if (((!(s.v[1620] != 0.0)) && (!(s.v[1874] != 0.0))) && (s.v[1875] != 0.0)) {
            s.store_ad(178, &A::limited_exp(s.ad_value(167)));
        }

        if (((!(s.v[1620] != 0.0)) && (!(s.v[1874] != 0.0))) && (s.v[1875] != 0.0)) {
            s.store_neg(178, 178);
        }

        if (((!(s.v[1620] != 0.0)) && (!(s.v[1874] != 0.0))) && (!(s.v[1875] != 0.0))) {
            s.store_div_from_scalar_ad(168, 1.0, A::sub(s.ad_value(558), s.ad_value(498)));
        }

        if (((!(s.v[1620] != 0.0)) && (!(s.v[1874] != 0.0))) && (!(s.v[1875] != 0.0))) {
            s.store_mul_ad_lhs(167, A::mul(A::div(A::neg(s.ad_value(498)), s.ad_value(563)), s.ad_value(558)), 168);
        }

        if (((!(s.v[1620] != 0.0)) && (!(s.v[1874] != 0.0))) && (!(s.v[1875] != 0.0))) {
            s.store_ad(178, &A::limited_exp(s.ad_value(167)));
        }

        if (((!(s.v[1620] != 0.0)) && (!(s.v[1874] != 0.0))) && (!(s.v[1875] != 0.0))) {
            s.store_neg(178, 178);
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1874] != 0.0))) {
            s.store_mul(170, 545, 554);
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1874] != 0.0))) {
            s.store_mul_ad_rhs(537, 170, A::add(s.ad_value(177), s.ad_value(178)));
        }

        s.v[1876] = if (s.v[553] == 0.0) { 1.0 } else { 0.0 };

        if ((!(s.v[1620] != 0.0)) && (s.v[1876] != 0.0)) {
            s.store_scalar(538, 0.0);
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1876] != 0.0))) {
            s.store_div_ad_lhs(174, A::mul(s.ad_value(556), s.ad_value(171)), 557);
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1876] != 0.0))) {
            s.store_ad(169, &A::limited_exp(s.ad_value(174)));
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1876] != 0.0))) {
            s.store_mul(555, 553, 169);
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1876] != 0.0))) {
            s.store_mul_ad(562, A::scale(s.ad_value(557), p.p925), A::offset(A::mul(s.ad_value(565), A::offset(s.ad_value(639), (-1.0))), 1.0));
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1876] != 0.0))) {
            s.store_mul_ad(563, A::scale(s.ad_value(564), p.p925), A::offset(A::mul(s.ad_value(566), A::offset(s.ad_value(639), (-1.0))), 1.0));
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1876] != 0.0))) {
            s.store_div(167, 499, 562);
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1876] != 0.0))) {
            s.store_ad(177, &A::limited_exp(s.ad_value(167)));
        }

        s.v[1877] = if ((s.v[559] - s.v[499]) < 0.001) { 1.0 } else { 0.0 };

        if (((!(s.v[1620] != 0.0)) && (!(s.v[1876] != 0.0))) && (s.v[1877] != 0.0)) {
            s.store_scalar(168, 1000.0);
        }

        if (((!(s.v[1620] != 0.0)) && (!(s.v[1876] != 0.0))) && (s.v[1877] != 0.0)) {
            s.store_mul_ad_lhs(167, A::mul(A::div(A::neg(s.ad_value(499)), s.ad_value(563)), s.ad_value(559)), 168);
        }

        if (((!(s.v[1620] != 0.0)) && (!(s.v[1876] != 0.0))) && (s.v[1877] != 0.0)) {
            s.store_ad(178, &A::limited_exp(s.ad_value(167)));
        }

        if (((!(s.v[1620] != 0.0)) && (!(s.v[1876] != 0.0))) && (s.v[1877] != 0.0)) {
            s.store_neg(178, 178);
        }

        if (((!(s.v[1620] != 0.0)) && (!(s.v[1876] != 0.0))) && (!(s.v[1877] != 0.0))) {
            s.store_div_from_scalar_ad(168, 1.0, A::sub(s.ad_value(559), s.ad_value(499)));
        }

        if (((!(s.v[1620] != 0.0)) && (!(s.v[1876] != 0.0))) && (!(s.v[1877] != 0.0))) {
            s.store_mul_ad_lhs(167, A::mul(A::div(A::neg(s.ad_value(499)), s.ad_value(563)), s.ad_value(559)), 168);
        }

        if (((!(s.v[1620] != 0.0)) && (!(s.v[1876] != 0.0))) && (!(s.v[1877] != 0.0))) {
            s.store_ad(178, &A::limited_exp(s.ad_value(167)));
        }

        if (((!(s.v[1620] != 0.0)) && (!(s.v[1876] != 0.0))) && (!(s.v[1877] != 0.0))) {
            s.store_neg(178, 178);
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1876] != 0.0))) {
            s.store_mul(170, 546, 555);
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1876] != 0.0))) {
            s.store_mul_ad_rhs(538, 170, A::add(s.ad_value(177), s.ad_value(178)));
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_scalar(602, ((s.v[183] / p.p1373) * p.p74));
        }

        s.v[1878] = if ((s.v[598] == 0.0) && (s.v[597] == 0.0)) { 1.0 } else { 0.0 };

        if ((!(s.v[1620] != 0.0)) && (s.v[1878] != 0.0)) {
            s.store_scalar(539, 0.0);
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1878] != 0.0)) {
            s.store_scalar(540, 0.0);
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1878] != 0.0)) {
            s.store_scalar(579, 0.0);
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1878] != 0.0))) {
            s.store_div_ad_lhs(174, A::mul(s.ad_value(589), s.ad_value(171)), 590);
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1878] != 0.0))) {
            s.store_ad(167, &A::limited_exp(s.ad_value(174)));
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1878] != 0.0))) {
            s.store_mul(585, 587, 167);
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1878] != 0.0))) {
            s.store_mul(578, 598, 167);
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1878] != 0.0))) {
            s.store_div_ad_lhs(174, A::mul(s.ad_value(589), s.ad_value(171)), 590);
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1878] != 0.0))) {
            s.store_ad(167, &A::limited_exp(s.ad_value(174)));
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1878] != 0.0))) {
            s.store_mul(586, 588, 167);
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1878] != 0.0))) {
            s.store_mul(577, 597, 167);
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1878] != 0.0))) {
            s.store_mul_ad_rhs(583, 585, A::offset(s.ad_value(595), (-1.0)));
        }

        s.v[1879] = if (s.v[583] < 1e-5) { 1.0 } else { 0.0 };

        if (((!(s.v[1620] != 0.0)) && (!(s.v[1878] != 0.0))) && (s.v[1879] != 0.0)) {
            s.store_scalar(583, 0.0);
        }

        if (((!(s.v[1620] != 0.0)) && (!(s.v[1878] != 0.0))) && (s.v[1879] != 0.0)) {
            s.store_scalar(591, 1.0);
        }

        if (((!(s.v[1620] != 0.0)) && (!(s.v[1878] != 0.0))) && (!(s.v[1879] != 0.0))) {
            s.store_div_from_scalar_ad(591, 1.0, A::sqrt(A::offset(s.ad_value(583), 1.0)));
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1878] != 0.0))) {
            s.store_mul_ad_rhs(584, 586, A::offset(s.ad_value(596), (-1.0)));
        }

        s.v[1880] = if (s.v[584] < 1e-5) { 1.0 } else { 0.0 };

        if (((!(s.v[1620] != 0.0)) && (!(s.v[1878] != 0.0))) && (s.v[1880] != 0.0)) {
            s.store_scalar(584, 0.0);
        }

        if (((!(s.v[1620] != 0.0)) && (!(s.v[1878] != 0.0))) && (s.v[1880] != 0.0)) {
            s.store_scalar(592, 1.0);
        }

        if (((!(s.v[1620] != 0.0)) && (!(s.v[1878] != 0.0))) && (!(s.v[1880] != 0.0))) {
            s.store_div_from_scalar_ad(592, 1.0, A::sqrt(A::offset(s.ad_value(584), 1.0)));
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1878] != 0.0))) {
            s.store_scalar(167, (((((-0.5) * s.v[184]) * s.v[184]) / p.p595) / p.p595));
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1878] != 0.0))) {
            s.store_ad(603, &A::limited_exp(s.ad_value(167)));
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1878] != 0.0))) {
            s.store_sub_from_scalar(169, 1.0, 603);
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1878] != 0.0))) {
            s.store_scale(167, 601, ((1.0 / s.v[184]) + (1.0 / p.p595)));
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1878] != 0.0))) {
            s.store_ad(599, &A::pow(s.ad_value(167), s.ad_value(600)));
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1878] != 0.0))) {
            s.store_mul_ad_lhs(604, A::mul(s.ad_value(602), s.ad_value(578)), 599);
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1878] != 0.0))) {
            s.store_mul(168, 167, 604);
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1878] != 0.0))) {
            s.store_mul_ad_lhs(539, A::mul(s.ad_value(168), A::offset(s.ad_value(595), (-1.0))), 591);
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1878] != 0.0))) {
            s.store_mul_ad_lhs(604, A::mul(s.ad_value(602), s.ad_value(577)), 599);
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1878] != 0.0))) {
            s.store_mul(168, 167, 604);
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1878] != 0.0))) {
            s.store_mul_ad_lhs(540, A::mul(s.ad_value(168), A::offset(s.ad_value(596), (-1.0))), 592);
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1878] != 0.0))) {
            s.store_offset_ad(531, A::scale(A::pow(s.ad_value(167), s.ad_value(530)), p.p920), 1.0);
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1878] != 0.0))) {
            s.store_mul_ad_lhs(532, A::mul(s.ad_value(602), s.ad_value(578)), 531);
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1878] != 0.0))) {
            s.store_mul_ad_lhs(533, A::mul(s.ad_value(532), A::offset(s.ad_value(595), (-1.0))), 591);
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1878] != 0.0))) {
            s.store_mul_ad_lhs(532, A::mul(s.ad_value(602), s.ad_value(577)), 531);
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1878] != 0.0))) {
            s.store_mul_ad_lhs(534, A::mul(s.ad_value(532), A::offset(s.ad_value(596), (-1.0))), 592);
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1878] != 0.0))) {
            s.store_add_ad_rhs(580, 581, A::scale(s.ad_value(582), s.v[184]));
        }

        s.v[1881] = if (s.v[580] < 1.0) { 1.0 } else { 0.0 };

        if (((!(s.v[1620] != 0.0)) && (!(s.v[1878] != 0.0))) && (s.v[1881] != 0.0)) {
            s.store_scalar(580, 1.0);
        }

        s.v[1882] = if (p.p554 == 1.0) { 1.0 } else { 0.0 };

        if (((!(s.v[1620] != 0.0)) && (!(s.v[1878] != 0.0))) && (s.v[1882] != 0.0)) {
            s.store_scalar(579, 0.0);
        }

        if (((!(s.v[1620] != 0.0)) && (!(s.v[1878] != 0.0))) && (!(s.v[1882] != 0.0))) {
            s.store_offset_ad(167, A::div(A::add(s.ad_value(498), s.ad_value(499)), s.ad_value(580)), 1.0);
        }

        if (((!(s.v[1620] != 0.0)) && (!(s.v[1878] != 0.0))) && (!(s.v[1882] != 0.0))) {
            s.store_add(168, 583, 584);
        }

        if (((!(s.v[1620] != 0.0)) && (!(s.v[1878] != 0.0))) && (!(s.v[1882] != 0.0))) {
            s.store_sqrt_ad(170, A::add(A::square(s.ad_value(167)), A::scale(s.ad_value(168), 4.0)));
        }

        if (((!(s.v[1620] != 0.0)) && (!(s.v[1878] != 0.0))) && (!(s.v[1882] != 0.0))) {
            s.store_scaled_add(169, 167, 170, 0.5);
        }

        s.v[1883] = if (s.v[169] < 0.1) { 1.0 } else { 0.0 };

        if ((((!(s.v[1620] != 0.0)) && (!(s.v[1878] != 0.0))) && (!(s.v[1882] != 0.0))) && (s.v[1883] != 0.0)) {
            s.store_scalar(605, 10.0);
        }

        if ((((!(s.v[1620] != 0.0)) && (!(s.v[1878] != 0.0))) && (!(s.v[1882] != 0.0))) && (!(s.v[1883] != 0.0))) {
            s.store_div_from_scalar(605, 1.0, 169);
        }

        if (((!(s.v[1620] != 0.0)) && (!(s.v[1878] != 0.0))) && (!(s.v[1882] != 0.0))) {
            s.store_mul(167, 603, 604);
        }

        if (((!(s.v[1620] != 0.0)) && (!(s.v[1878] != 0.0))) && (!(s.v[1882] != 0.0))) {
            s.store_mul_ad_lhs(579, A::mul(A::scale(s.ad_value(167), p.p2), A::sub(s.ad_value(595), s.ad_value(596))), 605);
        }

        s.v[1884] = if ((s.v[567] == 0.0) && (s.v[568] == 0.0)) { 1.0 } else { 0.0 };

        if ((!(s.v[1620] != 0.0)) && (s.v[1884] != 0.0)) {
            s.store_scalar(541, 0.0);
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1884] != 0.0)) {
            s.store_scalar(542, 0.0);
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1884] != 0.0))) {
            s.store_mul_ad_rhs(174, 569, A::offset(s.ad_value(639), (-1.0)));
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1884] != 0.0))) {
            s.store_ad(167, &A::limited_exp(s.ad_value(174)));
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1884] != 0.0))) {
            s.store_mul(571, 567, 167);
        }

    }

    pub(super) fn stamp_transient_block_35(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if ((!(s.v[1620] != 0.0)) && (!(s.v[1884] != 0.0))) {
            s.store_mul_ad_rhs(174, 570, A::offset(s.ad_value(639), (-1.0)));
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1884] != 0.0))) {
            s.store_ad(167, &A::limited_exp(s.ad_value(174)));
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1884] != 0.0))) {
            s.store_mul(572, 568, 167);
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1884] != 0.0))) {
            s.store_scale(594, 573, p.p925);
        }

        s.v[1885] = if ((s.v[575] - s.v[498]) < 0.001) { 1.0 } else { 0.0 };

        if (((!(s.v[1620] != 0.0)) && (!(s.v[1884] != 0.0))) && (s.v[1885] != 0.0)) {
            s.store_scalar(168, 1000.0);
        }

        if (((!(s.v[1620] != 0.0)) && (!(s.v[1884] != 0.0))) && (s.v[1885] != 0.0)) {
            s.store_mul_ad_lhs(167, A::mul(A::div(A::neg(s.ad_value(498)), s.ad_value(594)), s.ad_value(575)), 168);
        }

        if (((!(s.v[1620] != 0.0)) && (!(s.v[1884] != 0.0))) && (s.v[1885] != 0.0)) {
            s.store_ad(168, &A::limited_exp(s.ad_value(167)));
        }

        if (((!(s.v[1620] != 0.0)) && (!(s.v[1884] != 0.0))) && (s.v[1885] != 0.0)) {
            s.store_mul(170, 545, 571);
        }

        if (((!(s.v[1620] != 0.0)) && (!(s.v[1884] != 0.0))) && (s.v[1885] != 0.0)) {
            s.store_mul_ad_rhs(541, 170, A::sub_from_scalar(1.0, s.ad_value(168)));
        }

        if (((!(s.v[1620] != 0.0)) && (!(s.v[1884] != 0.0))) && (!(s.v[1885] != 0.0))) {
            s.store_div_from_scalar_ad(168, 1.0, A::sub(s.ad_value(575), s.ad_value(498)));
        }

        if (((!(s.v[1620] != 0.0)) && (!(s.v[1884] != 0.0))) && (!(s.v[1885] != 0.0))) {
            s.store_mul_ad_lhs(167, A::mul(A::div(A::neg(s.ad_value(498)), s.ad_value(594)), s.ad_value(575)), 168);
        }

        if (((!(s.v[1620] != 0.0)) && (!(s.v[1884] != 0.0))) && (!(s.v[1885] != 0.0))) {
            s.store_ad(168, &A::limited_exp(s.ad_value(167)));
        }

        if (((!(s.v[1620] != 0.0)) && (!(s.v[1884] != 0.0))) && (!(s.v[1885] != 0.0))) {
            s.store_mul(170, 545, 571);
        }

        if (((!(s.v[1620] != 0.0)) && (!(s.v[1884] != 0.0))) && (!(s.v[1885] != 0.0))) {
            s.store_mul_ad_rhs(541, 170, A::sub_from_scalar(1.0, s.ad_value(168)));
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1884] != 0.0))) {
            s.store_scale(594, 574, p.p925);
        }

        s.v[1886] = if ((s.v[576] - s.v[499]) < 0.001) { 1.0 } else { 0.0 };

        if (((!(s.v[1620] != 0.0)) && (!(s.v[1884] != 0.0))) && (s.v[1886] != 0.0)) {
            s.store_scalar(168, 1000.0);
        }

        if (((!(s.v[1620] != 0.0)) && (!(s.v[1884] != 0.0))) && (s.v[1886] != 0.0)) {
            s.store_mul_ad_lhs(167, A::mul(A::div(A::neg(s.ad_value(499)), s.ad_value(594)), s.ad_value(576)), 168);
        }

        if (((!(s.v[1620] != 0.0)) && (!(s.v[1884] != 0.0))) && (s.v[1886] != 0.0)) {
            s.store_ad(168, &A::limited_exp(s.ad_value(167)));
        }

        if (((!(s.v[1620] != 0.0)) && (!(s.v[1884] != 0.0))) && (s.v[1886] != 0.0)) {
            s.store_mul(170, 545, 572);
        }

        if (((!(s.v[1620] != 0.0)) && (!(s.v[1884] != 0.0))) && (s.v[1886] != 0.0)) {
            s.store_mul_ad_rhs(542, 170, A::sub_from_scalar(1.0, s.ad_value(168)));
        }

        if (((!(s.v[1620] != 0.0)) && (!(s.v[1884] != 0.0))) && (!(s.v[1886] != 0.0))) {
            s.store_div_from_scalar_ad(168, 1.0, A::sub(s.ad_value(576), s.ad_value(499)));
        }

        if (((!(s.v[1620] != 0.0)) && (!(s.v[1884] != 0.0))) && (!(s.v[1886] != 0.0))) {
            s.store_mul_ad_lhs(167, A::mul(A::div(A::neg(s.ad_value(499)), s.ad_value(594)), s.ad_value(576)), 168);
        }

        if (((!(s.v[1620] != 0.0)) && (!(s.v[1884] != 0.0))) && (!(s.v[1886] != 0.0))) {
            s.store_ad(168, &A::limited_exp(s.ad_value(167)));
        }

        if (((!(s.v[1620] != 0.0)) && (!(s.v[1884] != 0.0))) && (!(s.v[1886] != 0.0))) {
            s.store_mul(170, 545, 572);
        }

        if (((!(s.v[1620] != 0.0)) && (!(s.v[1884] != 0.0))) && (!(s.v[1886] != 0.0))) {
            s.store_mul_ad_rhs(542, 170, A::sub_from_scalar(1.0, s.ad_value(168)));
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_scale_ad(496, A::add(A::add(A::add(s.ad_value(535), s.ad_value(537)), s.ad_value(539)), s.ad_value(541)), p.p2);
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_scale_ad(497, A::add(A::add(A::add(s.ad_value(536), s.ad_value(538)), s.ad_value(540)), s.ad_value(542)), p.p2);
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_scalar(375, 0.0);
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_scalar(374, 0.0);
        }

        s.v[1887] = if (p.p36 == 0.0) { 1.0 } else { 0.0 };

        if ((!(s.v[1620] != 0.0)) && (s.v[1887] != 0.0)) {
            s.store_scalar(167, (s.v[200] * p.p76));
        }

        s.v[1888] = if (((s.v[892] <= 0.0) || (s.v[660] <= 0.0)) || (s.v[894] < 0.0)) { 1.0 } else { 0.0 };

        if (((!(s.v[1620] != 0.0)) && (s.v[1887] != 0.0)) && (s.v[1888] != 0.0)) {
            s.store_scalar(173, 0.0);
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1887] != 0.0)) && (!(s.v[1888] != 0.0))) {
            s.store_div_ad_lhs(168, A::add(A::sub(A::neg(s.ad_value(204)), s.ad_value(895)), s.ad_value(219)), 167);
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1887] != 0.0)) && (!(s.v[1888] != 0.0))) {
            s.store_scale_ad(168, A::add(s.ad_value(168), A::sqrt(A::offset(A::square(s.ad_value(168)), ((4.0 * 0.01) * 0.01)))), 0.5);
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1887] != 0.0)) && (!(s.v[1888] != 0.0))) {
            s.store_div_ad_rhs(169, 660, A::offset(s.ad_value(168), 0.001));
        }

        s.v[1889] = if (s.v[894] != 0.0) { 1.0 } else { 0.0 };

        if ((((!(s.v[1620] != 0.0)) && (s.v[1887] != 0.0)) && (!(s.v[1888] != 0.0))) && (s.v[1889] != 0.0)) {
            s.store_mul_ad_lhs(170, A::square(s.ad_value(201)), 201);
        }

        if ((((!(s.v[1620] != 0.0)) && (s.v[1887] != 0.0)) && (!(s.v[1888] != 0.0))) && (s.v[1889] != 0.0)) {
            s.store_offset_ad(171, A::add(s.ad_value(894), A::abs(s.ad_value(170))), 0.0001);
        }

        if ((((!(s.v[1620] != 0.0)) && (s.v[1887] != 0.0)) && (!(s.v[1888] != 0.0))) && (s.v[1889] != 0.0)) {
            s.store_offset_ad(172, A::scale(A::add(A::div(s.ad_value(170), s.ad_value(171)), A::sqrt(A::offset(A::mul(A::div(s.ad_value(170), s.ad_value(171)), A::div(s.ad_value(170), s.ad_value(171))), ((4.0 * 1e-6) * 1e-6)))), 0.5), (-1e-6));
        }

        if ((((!(s.v[1620] != 0.0)) && (s.v[1887] != 0.0)) && (!(s.v[1888] != 0.0))) && (!(s.v[1889] != 0.0))) {
            s.store_scalar(172, 1.0);
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1887] != 0.0)) && (!(s.v[1888] != 0.0))) {
            s.store_mul_ad_lhs(173, A::mul(A::mul(A::mul(s.ad_value(892), s.ad_value(544)), s.ad_value(168)), A::limited_exp(A::neg(s.ad_value(169)))), 172);
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1887] != 0.0)) {
            s.copy_ad(374, 173);
        }

        s.v[1890] = if (((s.v[896] <= 0.0) || (s.v[661] <= 0.0)) || (s.v[898] < 0.0)) { 1.0 } else { 0.0 };

        if (((!(s.v[1620] != 0.0)) && (s.v[1887] != 0.0)) && (s.v[1890] != 0.0)) {
            s.store_scalar(173, 0.0);
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1887] != 0.0)) && (!(s.v[1890] != 0.0))) {
            s.store_div_ad_lhs(168, A::add(A::sub(A::neg(s.ad_value(203)), s.ad_value(899)), s.ad_value(219)), 167);
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1887] != 0.0)) && (!(s.v[1890] != 0.0))) {
            s.store_scale_ad(168, A::add(s.ad_value(168), A::sqrt(A::offset(A::square(s.ad_value(168)), ((4.0 * 0.01) * 0.01)))), 0.5);
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1887] != 0.0)) && (!(s.v[1890] != 0.0))) {
            s.store_div_ad_rhs(169, 661, A::offset(s.ad_value(168), 0.001));
        }

        s.v[1891] = if (s.v[898] != 0.0) { 1.0 } else { 0.0 };

        if ((((!(s.v[1620] != 0.0)) && (s.v[1887] != 0.0)) && (!(s.v[1890] != 0.0))) && (s.v[1891] != 0.0)) {
            s.store_mul_ad_lhs(170, A::square(s.ad_value(202)), 202);
        }

        if ((((!(s.v[1620] != 0.0)) && (s.v[1887] != 0.0)) && (!(s.v[1890] != 0.0))) && (s.v[1891] != 0.0)) {
            s.store_offset_ad(171, A::add(s.ad_value(898), A::abs(s.ad_value(170))), 0.0001);
        }

        if ((((!(s.v[1620] != 0.0)) && (s.v[1887] != 0.0)) && (!(s.v[1890] != 0.0))) && (s.v[1891] != 0.0)) {
            s.store_offset_ad(172, A::scale(A::add(A::div(s.ad_value(170), s.ad_value(171)), A::sqrt(A::offset(A::mul(A::div(s.ad_value(170), s.ad_value(171)), A::div(s.ad_value(170), s.ad_value(171))), ((4.0 * 1e-6) * 1e-6)))), 0.5), (-1e-6));
        }

        if ((((!(s.v[1620] != 0.0)) && (s.v[1887] != 0.0)) && (!(s.v[1890] != 0.0))) && (!(s.v[1891] != 0.0))) {
            s.store_scalar(172, 1.0);
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1887] != 0.0)) && (!(s.v[1890] != 0.0))) {
            s.store_mul_ad_lhs(173, A::mul(A::mul(A::mul(s.ad_value(896), s.ad_value(543)), s.ad_value(168)), A::limited_exp(A::neg(s.ad_value(169)))), 172);
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1887] != 0.0)) {
            s.copy_ad(375, 173);
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1887] != 0.0))) {
            s.store_scalar(167, (s.v[200] * p.p76));
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1887] != 0.0))) {
            s.store_sub_ad_lhs(207, A::mul(s.ad_value(905), s.ad_value(221)), 223);
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1887] != 0.0))) {
            s.store_sub_ad_lhs(206, A::mul(s.ad_value(902), s.ad_value(221)), 224);
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1887] != 0.0))) {
            s.store_sub(169, 203, 219);
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1887] != 0.0))) {
            s.store_sqrt_ad(228, A::offset(A::square(s.ad_value(169)), 0.0001));
        }

        s.v[1892] = if ((s.v[892] <= 0.0) || (s.v[660] <= 0.0)) { 1.0 } else { 0.0 };

        if (((!(s.v[1620] != 0.0)) && (!(s.v[1887] != 0.0))) && (s.v[1892] != 0.0)) {
            s.store_scalar(173, 0.0);
        }

        if (((!(s.v[1620] != 0.0)) && (!(s.v[1887] != 0.0))) && (!(s.v[1892] != 0.0))) {
            s.store_div_ad_lhs(168, A::add(A::sub(A::neg(s.ad_value(207)), s.ad_value(895)), s.ad_value(219)), 167);
        }

        if (((!(s.v[1620] != 0.0)) && (!(s.v[1887] != 0.0))) && (!(s.v[1892] != 0.0))) {
            s.store_scale_ad(168, A::add(s.ad_value(168), A::sqrt(A::offset(A::square(s.ad_value(168)), ((4.0 * 0.01) * 0.01)))), 0.5);
        }

        if (((!(s.v[1620] != 0.0)) && (!(s.v[1887] != 0.0))) && (!(s.v[1892] != 0.0))) {
            s.store_div_ad_rhs(169, 660, A::offset(s.ad_value(168), 0.001));
        }

        s.v[1893] = if (s.v[903] != 0.0) { 1.0 } else { 0.0 };

        if ((((!(s.v[1620] != 0.0)) && (!(s.v[1887] != 0.0))) && (!(s.v[1892] != 0.0))) && (s.v[1893] != 0.0)) {
            s.store_sub_ad_lhs(170, A::neg(s.ad_value(201)), 904);
        }

        if ((((!(s.v[1620] != 0.0)) && (!(s.v[1887] != 0.0))) && (!(s.v[1892] != 0.0))) && (s.v[1893] != 0.0)) {
            s.store_offset(171, 170, 0.0001);
        }

        if ((((!(s.v[1620] != 0.0)) && (!(s.v[1887] != 0.0))) && (!(s.v[1892] != 0.0))) && (s.v[1893] != 0.0)) {
            s.store_offset_ad(172, A::scale(A::add(A::div(s.ad_value(903), s.ad_value(171)), A::sqrt(A::offset(A::mul(A::div(s.ad_value(903), s.ad_value(171)), A::div(s.ad_value(903), s.ad_value(171))), ((4.0 * 1e-6) * 1e-6)))), 0.5), (-1e-6));
        }

        if ((((!(s.v[1620] != 0.0)) && (!(s.v[1887] != 0.0))) && (!(s.v[1892] != 0.0))) && (!(s.v[1893] != 0.0))) {
            s.store_scalar(172, 1.0);
        }

        if (((!(s.v[1620] != 0.0)) && (!(s.v[1887] != 0.0))) && (!(s.v[1892] != 0.0))) {
            s.store_mul_ad(173, A::mul(A::mul(A::mul(s.ad_value(892), s.ad_value(544)), s.ad_value(168)), A::limited_exp(A::neg(s.ad_value(169)))), A::limited_exp(s.ad_value(172)));
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1887] != 0.0))) {
            s.copy_ad(374, 173);
        }

        s.v[1894] = if ((s.v[896] <= 0.0) || (s.v[661] <= 0.0)) { 1.0 } else { 0.0 };

        if (((!(s.v[1620] != 0.0)) && (!(s.v[1887] != 0.0))) && (s.v[1894] != 0.0)) {
            s.store_scalar(173, 0.0);
        }

        if (((!(s.v[1620] != 0.0)) && (!(s.v[1887] != 0.0))) && (!(s.v[1894] != 0.0))) {
            s.store_div_ad_lhs(168, A::add(A::sub(A::neg(s.ad_value(206)), s.ad_value(899)), s.ad_value(219)), 167);
        }

        if (((!(s.v[1620] != 0.0)) && (!(s.v[1887] != 0.0))) && (!(s.v[1894] != 0.0))) {
            s.store_scale_ad(168, A::add(s.ad_value(168), A::sqrt(A::offset(A::square(s.ad_value(168)), ((4.0 * 0.01) * 0.01)))), 0.5);
        }

        if (((!(s.v[1620] != 0.0)) && (!(s.v[1887] != 0.0))) && (!(s.v[1894] != 0.0))) {
            s.store_div_ad_rhs(169, 661, A::offset(s.ad_value(168), 0.001));
        }

        s.v[1895] = if (s.v[906] != 0.0) { 1.0 } else { 0.0 };

        if ((((!(s.v[1620] != 0.0)) && (!(s.v[1887] != 0.0))) && (!(s.v[1894] != 0.0))) && (s.v[1895] != 0.0)) {
            s.store_sub_ad_lhs(170, A::neg(s.ad_value(202)), 907);
        }

        if ((((!(s.v[1620] != 0.0)) && (!(s.v[1887] != 0.0))) && (!(s.v[1894] != 0.0))) && (s.v[1895] != 0.0)) {
            s.store_offset(171, 170, 0.0001);
        }

        if ((((!(s.v[1620] != 0.0)) && (!(s.v[1887] != 0.0))) && (!(s.v[1894] != 0.0))) && (s.v[1895] != 0.0)) {
            s.store_offset_ad(172, A::scale(A::add(A::div(s.ad_value(906), s.ad_value(171)), A::sqrt(A::offset(A::mul(A::div(s.ad_value(906), s.ad_value(171)), A::div(s.ad_value(906), s.ad_value(171))), ((4.0 * 1e-6) * 1e-6)))), 0.5), (-1e-6));
        }

        if ((((!(s.v[1620] != 0.0)) && (!(s.v[1887] != 0.0))) && (!(s.v[1894] != 0.0))) && (!(s.v[1895] != 0.0))) {
            s.store_scalar(172, 1.0);
        }

        if (((!(s.v[1620] != 0.0)) && (!(s.v[1887] != 0.0))) && (!(s.v[1894] != 0.0))) {
            s.store_mul_ad(173, A::mul(A::mul(A::mul(s.ad_value(896), s.ad_value(543)), s.ad_value(168)), A::limited_exp(A::neg(s.ad_value(169)))), A::limited_exp(s.ad_value(172)));
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1887] != 0.0))) {
            s.copy_ad(375, 173);
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_mul_ad_lhs(1096, A::scale(s.ad_value(379), p.p2), 374);
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_mul_ad_lhs(1097, A::scale(s.ad_value(379), p.p2), 375);
        }

        s.v[1896] = if (p.p44 == 0.0) { 1.0 } else { 0.0 };

        s.v[1897] = if ((s.v[865] <= 0.0) || (s.v[659] <= 0.0)) { 1.0 } else { 0.0 };

        if (((!(s.v[1620] != 0.0)) && (s.v[1896] != 0.0)) && (s.v[1897] != 0.0)) {
            s.store_scalar(373, 0.0);
        }

        s.v[1898] = if (s.v[355] > (s.v[659] / 80.0)) { 1.0 } else { 0.0 };

        if ((((!(s.v[1620] != 0.0)) && (s.v[1896] != 0.0)) && (!(s.v[1897] != 0.0))) && (s.v[1898] != 0.0)) {
            s.store_div_ad_lhs(168, A::neg(s.ad_value(659)), 355);
        }

        if ((((!(s.v[1620] != 0.0)) && (s.v[1896] != 0.0)) && (!(s.v[1897] != 0.0))) && (s.v[1898] != 0.0)) {
            s.store_div_ad_lhs(373, A::mul(A::mul(A::mul(s.ad_value(865), s.ad_value(355)), s.ad_value(380)), A::limited_exp(s.ad_value(168))), 365);
        }

        if ((((!(s.v[1620] != 0.0)) && (s.v[1896] != 0.0)) && (!(s.v[1897] != 0.0))) && (!(s.v[1898] != 0.0))) {
            s.store_div_ad_lhs(373, A::scale(A::mul(A::mul(s.ad_value(865), s.ad_value(355)), s.ad_value(380)), 1.804851387e-35), 365);
        }

        s.v[1899] = if (p.p44 == 1.0) { 1.0 } else { 0.0 };

        s.v[1900] = if ((s.v[865] <= 0.0) || (((s.v[868] == 0.0) && (s.v[867] == 0.0)) && (s.v[659] == 0.0))) { 1.0 } else { 0.0 };

        if ((((!(s.v[1620] != 0.0)) && (!(s.v[1896] != 0.0))) && (s.v[1899] != 0.0)) && (s.v[1900] != 0.0)) {
            s.store_scalar(373, 0.0);
        }

        if ((((!(s.v[1620] != 0.0)) && (!(s.v[1896] != 0.0))) && (s.v[1899] != 0.0)) && (!(s.v[1900] != 0.0))) {
            s.store_sub_ad(370, A::mul(s.ad_value(874), A::offset(A::scale(A::offset(s.ad_value(639), (-1.0)), p.p600), 1.0)), A::scale(s.ad_value(869), 1.0 / (s.v[184])));
        }

        if ((((!(s.v[1620] != 0.0)) && (!(s.v[1896] != 0.0))) && (s.v[1899] != 0.0)) && (!(s.v[1900] != 0.0))) {
            s.store_scale(167, 875, s.v[184]);
        }

        if ((((!(s.v[1620] != 0.0)) && (!(s.v[1896] != 0.0))) && (s.v[1899] != 0.0)) && (!(s.v[1900] != 0.0))) {
            s.store_div_ad(168, A::mul(s.ad_value(870), s.ad_value(167)), A::offset(s.ad_value(167), 1.0));
        }

        if ((((!(s.v[1620] != 0.0)) && (!(s.v[1896] != 0.0))) && (s.v[1899] != 0.0)) && (!(s.v[1900] != 0.0))) {
            s.store_div_from_scalar_ad(167, 1.0, A::offset(A::scale(A::add(A::mul(A::mul(s.ad_value(871), s.ad_value(367)), s.ad_value(269)), A::sqrt(A::offset(A::mul(A::mul(A::mul(s.ad_value(871), s.ad_value(367)), s.ad_value(269)), A::mul(A::mul(s.ad_value(871), s.ad_value(367)), s.ad_value(269))), ((4.0 * p.p643) * p.p643)))), 0.5), 1.0));
        }

        if ((((!(s.v[1620] != 0.0)) && (!(s.v[1896] != 0.0))) && (s.v[1899] != 0.0)) && (!(s.v[1900] != 0.0))) {
            s.store_add(170, 167, 872);
        }

        if ((((!(s.v[1620] != 0.0)) && (!(s.v[1896] != 0.0))) && (s.v[1899] != 0.0)) && (!(s.v[1900] != 0.0))) {
            s.store_scale_ad(169, A::add(A::mul(A::mul(s.ad_value(367), s.ad_value(269)), s.ad_value(170)), A::sqrt(A::offset(A::mul(A::mul(A::mul(s.ad_value(367), s.ad_value(269)), s.ad_value(170)), A::mul(A::mul(s.ad_value(367), s.ad_value(269)), s.ad_value(170))), ((4.0 * p.p644) * p.p644)))), 0.5);
        }

        if ((((!(s.v[1620] != 0.0)) && (!(s.v[1896] != 0.0))) && (s.v[1899] != 0.0)) && (!(s.v[1900] != 0.0))) {
            s.store_div_from_scalar_ad(170, 1.0, A::offset(A::mul(s.ad_value(873), s.ad_value(227)), 1.0));
        }

        if ((((!(s.v[1620] != 0.0)) && (!(s.v[1896] != 0.0))) && (s.v[1899] != 0.0)) && (!(s.v[1900] != 0.0))) {
            s.store_mul_ad_lhs(368, A::mul(s.ad_value(168), s.ad_value(169)), 170);
        }

        if ((((!(s.v[1620] != 0.0)) && (!(s.v[1896] != 0.0))) && (s.v[1899] != 0.0)) && (!(s.v[1900] != 0.0))) {
            s.store_add(369, 370, 368);
        }

        if ((((!(s.v[1620] != 0.0)) && (!(s.v[1896] != 0.0))) && (s.v[1899] != 0.0)) && (!(s.v[1900] != 0.0))) {
            s.store_sub(371, 227, 369);
        }

        if ((((!(s.v[1620] != 0.0)) && (!(s.v[1896] != 0.0))) && (s.v[1899] != 0.0)) && (!(s.v[1900] != 0.0))) {
            s.store_add_ad(167, A::add(s.ad_value(868), A::mul(s.ad_value(867), s.ad_value(371))), A::mul(A::mul(s.ad_value(659), s.ad_value(371)), s.ad_value(371)));
        }

        if ((((!(s.v[1620] != 0.0)) && (!(s.v[1896] != 0.0))) && (s.v[1899] != 0.0)) && (!(s.v[1900] != 0.0))) {
            s.store_sqrt_ad(168, A::offset(A::square(s.ad_value(167)), 1e-10));
        }

        if ((((!(s.v[1620] != 0.0)) && (!(s.v[1896] != 0.0))) && (s.v[1899] != 0.0)) && (!(s.v[1900] != 0.0))) {
            let assign50620_ad_e84223: A = A::add(A::offset(A::offset(A::mul(A::neg(s.ad_value(865)), A::limited_exp(A::div(s.ad_value(371), s.ad_value(168)))), (-(-10.0))), (-p.p645)), A::sqrt(A::offset(A::mul(A::offset(A::offset(A::mul(A::neg(s.ad_value(865)), A::limited_exp(A::div(s.ad_value(371), s.ad_value(168)))), (-(-10.0))), (-p.p645)), A::offset(A::offset(A::mul(A::neg(s.ad_value(865)), A::limited_exp(A::div(s.ad_value(371), s.ad_value(168)))), (-(-10.0))), (-p.p645))), (-((4.0 * (-10.0)) * p.p645)))));
            s.store_neg_ad(372, A::offset(A::scale(assign50620_ad_e84223, 0.5), (-10.0)));
        }

        if ((((!(s.v[1620] != 0.0)) && (!(s.v[1896] != 0.0))) && (s.v[1899] != 0.0)) && (!(s.v[1900] != 0.0))) {
            s.store_mul_ad_rhs(373, 372, A::add(s.ad_value(380), A::mul(A::mul(s.ad_value(876), s.ad_value(211)), s.ad_value(579))));
        }

        s.v[1901] = if ((s.v[865] <= 0.0) || (((s.v[868] == 0.0) && (s.v[867] == 0.0)) && (s.v[659] == 0.0))) { 1.0 } else { 0.0 };

        if ((((!(s.v[1620] != 0.0)) && (!(s.v[1896] != 0.0))) && (!(s.v[1899] != 0.0))) && (s.v[1901] != 0.0)) {
            s.store_scalar(373, 0.0);
        }

        if ((((!(s.v[1620] != 0.0)) && (!(s.v[1896] != 0.0))) && (!(s.v[1899] != 0.0))) && (!(s.v[1901] != 0.0))) {
            s.store_sub_ad(370, A::mul(s.ad_value(874), A::offset(A::scale(A::offset(s.ad_value(639), (-1.0)), p.p600), 1.0)), A::scale(s.ad_value(869), 1.0 / (s.v[184])));
        }

        if ((((!(s.v[1620] != 0.0)) && (!(s.v[1896] != 0.0))) && (!(s.v[1899] != 0.0))) && (!(s.v[1901] != 0.0))) {
            s.store_scale(167, 875, s.v[184]);
        }

        if ((((!(s.v[1620] != 0.0)) && (!(s.v[1896] != 0.0))) && (!(s.v[1899] != 0.0))) && (!(s.v[1901] != 0.0))) {
            s.store_div_ad(168, A::mul(s.ad_value(870), s.ad_value(167)), A::offset(s.ad_value(167), 1.0));
        }

        if ((((!(s.v[1620] != 0.0)) && (!(s.v[1896] != 0.0))) && (!(s.v[1899] != 0.0))) && (!(s.v[1901] != 0.0))) {
            s.store_div_from_scalar_ad(167, 1.0, A::offset(A::scale(A::add(A::mul(A::mul(s.ad_value(871), s.ad_value(367)), s.ad_value(269)), A::sqrt(A::offset(A::mul(A::mul(A::mul(s.ad_value(871), s.ad_value(367)), s.ad_value(269)), A::mul(A::mul(s.ad_value(871), s.ad_value(367)), s.ad_value(269))), ((4.0 * p.p643) * p.p643)))), 0.5), 1.0));
        }

        if ((((!(s.v[1620] != 0.0)) && (!(s.v[1896] != 0.0))) && (!(s.v[1899] != 0.0))) && (!(s.v[1901] != 0.0))) {
            s.store_add(170, 167, 872);
        }

        if ((((!(s.v[1620] != 0.0)) && (!(s.v[1896] != 0.0))) && (!(s.v[1899] != 0.0))) && (!(s.v[1901] != 0.0))) {
            s.store_scale_ad(169, A::add(A::mul(A::mul(s.ad_value(367), s.ad_value(269)), s.ad_value(170)), A::sqrt(A::offset(A::mul(A::mul(A::mul(s.ad_value(367), s.ad_value(269)), s.ad_value(170)), A::mul(A::mul(s.ad_value(367), s.ad_value(269)), s.ad_value(170))), ((4.0 * p.p644) * p.p644)))), 0.5);
        }

        if ((((!(s.v[1620] != 0.0)) && (!(s.v[1896] != 0.0))) && (!(s.v[1899] != 0.0))) && (!(s.v[1901] != 0.0))) {
            s.store_div_from_scalar_ad(170, 1.0, A::offset(A::mul(s.ad_value(873), s.ad_value(227)), 1.0));
        }

        if ((((!(s.v[1620] != 0.0)) && (!(s.v[1896] != 0.0))) && (!(s.v[1899] != 0.0))) && (!(s.v[1901] != 0.0))) {
            s.store_mul_ad_lhs(368, A::mul(s.ad_value(168), s.ad_value(169)), 170);
        }

        if ((((!(s.v[1620] != 0.0)) && (!(s.v[1896] != 0.0))) && (!(s.v[1899] != 0.0))) && (!(s.v[1901] != 0.0))) {
            s.store_add(369, 370, 368);
        }

        if ((((!(s.v[1620] != 0.0)) && (!(s.v[1896] != 0.0))) && (!(s.v[1899] != 0.0))) && (!(s.v[1901] != 0.0))) {
            s.store_sub(371, 227, 369);
        }

        if ((((!(s.v[1620] != 0.0)) && (!(s.v[1896] != 0.0))) && (!(s.v[1899] != 0.0))) && (!(s.v[1901] != 0.0))) {
            s.store_add_ad(167, A::add(s.ad_value(868), A::mul(s.ad_value(867), s.ad_value(371))), A::mul(A::mul(s.ad_value(659), s.ad_value(371)), s.ad_value(371)));
        }

        if ((((!(s.v[1620] != 0.0)) && (!(s.v[1896] != 0.0))) && (!(s.v[1899] != 0.0))) && (!(s.v[1901] != 0.0))) {
            s.store_sqrt_ad(168, A::offset(A::square(s.ad_value(167)), 1e-10));
        }

        if ((((!(s.v[1620] != 0.0)) && (!(s.v[1896] != 0.0))) && (!(s.v[1899] != 0.0))) && (!(s.v[1901] != 0.0))) {
            let assign50780_ad_e84610: A = A::add(A::offset(A::offset(A::mul(A::neg(s.ad_value(865)), A::limited_exp(A::div(s.ad_value(371), s.ad_value(168)))), (-(-10.0))), (-p.p645)), A::sqrt(A::offset(A::mul(A::offset(A::offset(A::mul(A::neg(s.ad_value(865)), A::limited_exp(A::div(s.ad_value(371), s.ad_value(168)))), (-(-10.0))), (-p.p645)), A::offset(A::offset(A::mul(A::neg(s.ad_value(865)), A::limited_exp(A::div(s.ad_value(371), s.ad_value(168)))), (-(-10.0))), (-p.p645))), (-((4.0 * (-10.0)) * p.p645)))));
            s.store_neg_ad(372, A::offset(A::scale(assign50780_ad_e84610, 0.5), (-10.0)));
        }

        if ((((!(s.v[1620] != 0.0)) && (!(s.v[1896] != 0.0))) && (!(s.v[1899] != 0.0))) && (!(s.v[1901] != 0.0))) {
            s.store_mul(376, 372, 380);
        }

        if (((!(s.v[1620] != 0.0)) && (!(s.v[1896] != 0.0))) && (!(s.v[1899] != 0.0))) {
            s.store_scale_ad(167, A::add(s.ad_value(878), A::scale(s.ad_value(877), s.v[184])), 1.0 / (s.v[184]));
        }

        if (((!(s.v[1620] != 0.0)) && (!(s.v[1896] != 0.0))) && (!(s.v[1899] != 0.0))) {
            s.store_mul_ad_rhs(378, 880, A::offset(A::scale(A::offset(s.ad_value(639), (-1.0)), p.p666), 1.0));
        }

        s.v[1902] = if (s.v[211] > 0.0) { 1.0 } else { 0.0 };

        if ((((!(s.v[1620] != 0.0)) && (!(s.v[1896] != 0.0))) && (!(s.v[1899] != 0.0))) && (s.v[1902] != 0.0)) {
            s.store_sub(168, 378, 499);
        }

        if ((((!(s.v[1620] != 0.0)) && (!(s.v[1896] != 0.0))) && (!(s.v[1899] != 0.0))) && (!(s.v[1902] != 0.0))) {
            s.store_sub(168, 378, 498);
        }

        if (((!(s.v[1620] != 0.0)) && (!(s.v[1896] != 0.0))) && (!(s.v[1899] != 0.0))) {
            s.store_offset(169, 881, (-1.0));
        }

        s.v[1903] = if (s.v[168] > 0.0) { 1.0 } else { 0.0 };

        if ((((!(s.v[1620] != 0.0)) && (!(s.v[1896] != 0.0))) && (!(s.v[1899] != 0.0))) && (s.v[1903] != 0.0)) {
            s.store_mul_ad(170, A::neg(s.ad_value(879)), A::pow(s.ad_value(168), s.ad_value(169)));
        }

        if ((((!(s.v[1620] != 0.0)) && (!(s.v[1896] != 0.0))) && (!(s.v[1899] != 0.0))) && (!(s.v[1903] != 0.0))) {
            s.store_scalar(170, 0.0);
        }

    }

    pub(super) fn stamp_transient_block_36(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if (((!(s.v[1620] != 0.0)) && (!(s.v[1896] != 0.0))) && (!(s.v[1899] != 0.0))) {
            s.store_ad(171, &A::limited_exp(s.ad_value(170)));
        }

        if (((!(s.v[1620] != 0.0)) && (!(s.v[1896] != 0.0))) && (!(s.v[1899] != 0.0))) {
            s.store_mul_ad_lhs(377, A::mul(A::mul(A::mul(s.ad_value(167), s.ad_value(211)), s.ad_value(579)), s.ad_value(168)), 171);
        }

        if (((!(s.v[1620] != 0.0)) && (!(s.v[1896] != 0.0))) && (!(s.v[1899] != 0.0))) {
            s.store_add(373, 376, 377);
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_mul(1095, 373, 379);
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_add_ad_rhs(810, 810, A::mul(s.ad_value(813), A::offset(s.ad_value(639), (-1.0))));
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_add_ad_rhs(816, 816, A::mul(s.ad_value(814), A::offset(s.ad_value(639), (-1.0))));
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_add_ad_rhs(819, 819, A::mul(s.ad_value(815), A::offset(s.ad_value(639), (-1.0))));
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_add_ad_rhs(884, 884, A::mul(s.ad_value(886), A::offset(s.ad_value(639), (-1.0))));
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_add_ad_rhs(882, 882, A::mul(s.ad_value(887), A::offset(s.ad_value(639), (-1.0))));
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_add_ad_rhs(888, 888, A::mul(s.ad_value(891), A::offset(s.ad_value(639), (-1.0))));
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_scalar(477, 0.0);
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_scalar(479, 0.0);
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_scalar(480, 0.0);
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_scalar(483, 0.0);
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_scalar(484, 0.0);
        }

        s.v[1904] = if ((p.p37 != 0.0) || (p.p38 != 0.0)) { 1.0 } else { 0.0 };

        if ((!(s.v[1620] != 0.0)) && (s.v[1904] != 0.0)) {
            s.store_mul_ad_rhs(469, 269, A::add(A::add(A::sub(s.ad_value(213), s.ad_value(254)), s.ad_value(400)), s.ad_value(320)));
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1904] != 0.0)) {
            s.store_sqrt_ad(168, A::offset(A::square(s.ad_value(469)), 0.0001));
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1904] != 0.0)) {
            s.store_scaled_sub(471, 168, 469, 0.5);
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1904] != 0.0)) {
            s.store_scaled_add(470, 469, 168, 0.5);
        }

        s.v[1905] = if (p.p38 != 0.0) { 1.0 } else { 0.0 };

        if (((!(s.v[1620] != 0.0)) && (s.v[1904] != 0.0)) && (s.v[1905] != 0.0)) {
            s.store_scale(168, 469, 1.0 / (p.p671));
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1904] != 0.0)) && (s.v[1905] != 0.0)) {
            let assign51110_ad_e85011: A = {
                if ((!((-s.v[168]) > 37.0)) && (!((-s.v[168]) < (-37.0)))) {
                    A::ln(A::offset(A::exp(A::neg(s.ad_value(168))), 1.0))
                } else {
                    {
                        if ((!((-s.v[168]) > 37.0)) && ((-s.v[168]) < (-37.0))) {
                            A::exp(A::neg(s.ad_value(168)))
                        } else {
                            {
                                if ((-s.v[168]) > 37.0) {
                                    A::neg(s.ad_value(168))
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }
                }
            };
            s.store_scale_ad(474, assign51110_ad_e85011, p.p671);
        }

        s.v[1906] = if (p.p696 != 0.0) { 1.0 } else { 0.0 };

        if ((((!(s.v[1620] != 0.0)) && (s.v[1904] != 0.0)) && (s.v[1905] != 0.0)) && (s.v[1906] != 0.0)) {
            s.store_sub_from_scalar_ad(167, 1.0, A::scale(s.ad_value(471), 1.0 / (p.p696)));
        }

        if ((((!(s.v[1620] != 0.0)) && (s.v[1904] != 0.0)) && (s.v[1905] != 0.0)) && (!(s.v[1906] != 0.0))) {
            s.store_scalar(167, 1.0);
        }

        s.v[1907] = if (s.v[167] < 0.01) { 1.0 } else { 0.0 };

        if ((((!(s.v[1620] != 0.0)) && (s.v[1904] != 0.0)) && (s.v[1905] != 0.0)) && (s.v[1907] != 0.0)) {
            s.store_scalar(167, 0.01);
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1904] != 0.0)) && (s.v[1905] != 0.0)) {
            s.store_scale(168, 492, ((((s.v[184] * s.v[183]) / p.p1373) + (p.p1381 / p.p2)) * p.p700));
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1904] != 0.0)) && (s.v[1905] != 0.0)) {
            s.store_scalar(169, (p.p701 * p.p76));
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1904] != 0.0)) && (s.v[1905] != 0.0)) {
            s.store_div_ad_lhs(170, A::mul(s.ad_value(169), A::sub(s.ad_value(882), A::mul(s.ad_value(883), s.ad_value(471)))), 167);
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1904] != 0.0)) && (s.v[1905] != 0.0)) {
            s.store_ad(171, &A::limited_exp(s.ad_value(170)));
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1904] != 0.0)) && (s.v[1905] != 0.0)) {
            s.store_mul_ad_lhs(476, A::mul(A::mul(s.ad_value(168), s.ad_value(221)), s.ad_value(474)), 171);
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1904] != 0.0)) && (s.v[1905] != 0.0)) {
            s.store_mul(476, 476, 662);
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1904] != 0.0)) && (s.v[1905] != 0.0)) {
            s.store_scaled_sub(168, 469, 809, 1.0 / (p.p671));
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1904] != 0.0)) && (s.v[1905] != 0.0)) {
            let assign51240_ad_e85197: A = {
                if ((!(s.v[168] > 37.0)) && (!(s.v[168] < (-37.0)))) {
                    A::ln(A::offset(A::exp(s.ad_value(168)), 1.0))
                } else {
                    {
                        if ((!(s.v[168] > 37.0)) && (s.v[168] < (-37.0))) {
                            A::exp(s.ad_value(168))
                        } else {
                            {
                                if (s.v[168] > 37.0) {
                                    s.ad_value(168)
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }
                }
            };
            s.store_scale_ad(473, assign51240_ad_e85197, p.p671);
        }

        s.v[1908] = if (p.p697 != 0.0) { 1.0 } else { 0.0 };

        if ((((!(s.v[1620] != 0.0)) && (s.v[1904] != 0.0)) && (s.v[1905] != 0.0)) && (s.v[1908] != 0.0)) {
            s.store_sub_from_scalar_ad(167, 1.0, A::scale(s.ad_value(470), 1.0 / (p.p697)));
        }

        if ((((!(s.v[1620] != 0.0)) && (s.v[1904] != 0.0)) && (s.v[1905] != 0.0)) && (!(s.v[1908] != 0.0))) {
            s.store_scalar(167, 1.0);
        }

        s.v[1909] = if (s.v[167] < 0.01) { 1.0 } else { 0.0 };

        if ((((!(s.v[1620] != 0.0)) && (s.v[1904] != 0.0)) && (s.v[1905] != 0.0)) && (s.v[1909] != 0.0)) {
            s.store_scalar(167, 0.01);
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1904] != 0.0)) && (s.v[1905] != 0.0)) {
            s.store_scale(168, 492, ((((s.v[184] * s.v[183]) / p.p1373) + (p.p1381 / p.p2)) * p.p698));
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1904] != 0.0)) && (s.v[1905] != 0.0)) {
            s.store_scalar(169, (p.p699 * p.p76));
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1904] != 0.0)) && (s.v[1905] != 0.0)) {
            s.store_div_ad_lhs(170, A::mul(s.ad_value(169), A::sub(s.ad_value(884), A::mul(s.ad_value(885), s.ad_value(470)))), 167);
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1904] != 0.0)) && (s.v[1905] != 0.0)) {
            s.store_ad(171, &A::limited_exp(s.ad_value(170)));
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1904] != 0.0)) && (s.v[1905] != 0.0)) {
            s.store_mul_ad_lhs(475, A::mul(A::mul(s.ad_value(168), s.ad_value(221)), s.ad_value(473)), 171);
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1904] != 0.0)) && (s.v[1905] != 0.0)) {
            s.store_mul(475, 475, 662);
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1904] != 0.0)) && (s.v[1905] != 0.0)) {
            s.store_scaled_add(477, 476, 475, p.p2);
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1904] != 0.0)) && (s.v[1905] != 0.0)) {
            s.store_offset_ad(478, A::mul(s.ad_value(212), s.ad_value(269)), p.p1383);
        }

        s.v[1910] = if (((((p.p43 != 0.0) && (1.0 != 0.0)) && (!((p.p40 != 0.0) && (!(1.0 != 0.0))))) && (p.p45 == 1.0)) && (p.p1380 > 0.0)) { 1.0 } else { 0.0 };

        if (((!(s.v[1620] != 0.0)) && (s.v[1904] != 0.0)) && (s.v[1910] != 0.0)) {
            s.store_mul_ad_rhs(208, 379, A::voltage(ctx, &nodes, Some(8), Some(11)));
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1904] != 0.0)) && (s.v[1910] != 0.0)) {
            s.store_sub(167, 208, 478);
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1904] != 0.0)) && (s.v[1910] != 0.0)) {
            s.store_sqrt_ad(168, A::offset(A::square(s.ad_value(167)), 0.0001));
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1904] != 0.0)) && (s.v[1910] != 0.0)) {
            s.store_scale_ad(209, A::offset(A::sub(s.ad_value(168), s.ad_value(167)), (-0.01)), 0.5);
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1904] != 0.0)) && (s.v[1910] != 0.0)) {
            s.store_scalar(178, (if (p.p30 == 1.0) { p.p702 } else { p.p703 }));
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1904] != 0.0)) && (s.v[1910] != 0.0)) {
            s.store_scalar(179, (if (p.p30 == 1.0) { p.p704 } else { p.p705 }));
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1904] != 0.0)) && (s.v[1910] != 0.0)) {
            s.store_mul(169, 208, 209);
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1904] != 0.0)) && (s.v[1910] != 0.0)) {
            s.store_sub_ad_lhs(170, A::mul(s.ad_value(888), s.ad_value(890)), 889);
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1904] != 0.0)) && (s.v[1910] != 0.0)) {
            s.store_mul(171, 889, 890);
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1904] != 0.0)) && (s.v[1910] != 0.0)) {
            s.store_mul_ad(172, A::scale(A::neg(s.ad_value(179)), p.p76), A::sub(A::add(s.ad_value(888), A::mul(s.ad_value(170), s.ad_value(209))), A::mul(A::mul(s.ad_value(171), s.ad_value(209)), s.ad_value(209))));
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1904] != 0.0)) && (s.v[1910] != 0.0)) {
            s.store_ad(173, &A::limited_exp(s.ad_value(172)));
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1904] != 0.0)) && (s.v[1910] != 0.0)) {
            s.store_mul_ad_lhs(178, A::scale(s.ad_value(178), p.p1380), 492);
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1904] != 0.0)) && (s.v[1910] != 0.0)) {
            s.store_mul_ad_lhs(210, A::mul(A::mul(s.ad_value(178), s.ad_value(169)), s.ad_value(173)), 662);
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1904] != 0.0)) && (!(s.v[1910] != 0.0))) {
            s.store_scalar(210, 0.0);
        }

        s.v[1911] = if (p.p37 != 0.0) { 1.0 } else { 0.0 };

        if (((!(s.v[1620] != 0.0)) && (s.v[1904] != 0.0)) && (s.v[1911] != 0.0)) {
            s.store_sub_ad_rhs(168, 810, A::mul(s.ad_value(811), s.ad_value(470)));
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1904] != 0.0)) && (s.v[1911] != 0.0)) {
            s.store_offset_ad(169, A::mul(s.ad_value(812), s.ad_value(470)), 1.0);
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1904] != 0.0)) && (s.v[1911] != 0.0)) {
            s.store_mul_ad_lhs(170, A::scale(s.ad_value(168), s.v[488]), 169);
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1904] != 0.0)) && (s.v[1911] != 0.0)) {
            s.store_mul_ad(171, A::mul(A::mul(s.ad_value(253), s.ad_value(269)), A::add(s.ad_value(400), s.ad_value(320))), A::limited_exp(s.ad_value(170)));
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1904] != 0.0)) && (s.v[1911] != 0.0)) {
            s.store_mul_ad_lhs(481, A::mul(A::mul(A::scale(s.ad_value(487), p.p2), s.ad_value(171)), A::sub(A::add(s.ad_value(221), A::scale(s.ad_value(227), 0.5)), A::scale(A::add(s.ad_value(224), s.ad_value(223)), 0.5))), 662);
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1904] != 0.0)) && (s.v[1911] != 0.0)) {
            s.store_offset_ad(472, A::sqrt(A::offset(A::square(s.ad_value(315)), 0.01)), (-0.1));
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1904] != 0.0)) && (s.v[1911] != 0.0)) {
            s.store_scale(168, 472, s.v[823]);
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1904] != 0.0)) && (s.v[1911] != 0.0)) {
            s.store_limited_exp_ad(482, A::neg(s.ad_value(168)));
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1904] != 0.0)) && (s.v[1911] != 0.0)) {
            s.store_offset_ad(170, A::offset(A::add(s.ad_value(168), s.ad_value(482)), (-1.0)), 0.0001);
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1904] != 0.0)) && (s.v[1911] != 0.0)) {
            s.store_offset_ad(171, A::sub_from_scalar(1.0, A::mul(A::offset(s.ad_value(168), 1.0), s.ad_value(482))), 0.0001);
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1904] != 0.0)) && (s.v[1911] != 0.0)) {
            s.store_offset_ad(172, A::square(s.ad_value(168)), 0.0002);
        }

        s.v[1912] = if (s.v[211] > 0.0) { 1.0 } else { 0.0 };

        if ((((!(s.v[1620] != 0.0)) && (s.v[1904] != 0.0)) && (s.v[1911] != 0.0)) && (s.v[1912] != 0.0)) {
            s.store_div_ad_lhs(480, A::mul(s.ad_value(481), s.ad_value(171)), 172);
        }

        if ((((!(s.v[1620] != 0.0)) && (s.v[1904] != 0.0)) && (s.v[1911] != 0.0)) && (s.v[1912] != 0.0)) {
            s.store_div_ad_lhs(479, A::mul(s.ad_value(481), s.ad_value(170)), 172);
        }

        if ((((!(s.v[1620] != 0.0)) && (s.v[1904] != 0.0)) && (s.v[1911] != 0.0)) && (!(s.v[1912] != 0.0))) {
            s.store_div_ad_lhs(479, A::mul(s.ad_value(481), s.ad_value(171)), 172);
        }

        if ((((!(s.v[1620] != 0.0)) && (s.v[1904] != 0.0)) && (s.v[1911] != 0.0)) && (!(s.v[1912] != 0.0))) {
            s.store_div_ad_lhs(480, A::mul(s.ad_value(481), s.ad_value(170)), 172);
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1904] != 0.0)) && (s.v[1911] != 0.0)) {
            s.store_sub(169, 203, 219);
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1904] != 0.0)) && (s.v[1911] != 0.0)) {
            s.store_sqrt_ad(228, A::offset(A::square(s.ad_value(169)), 0.0001));
        }

        s.v[1913] = if (p.p1295 == 1.0) { 1.0 } else { 0.0 };

        if ((((!(s.v[1620] != 0.0)) && (s.v[1904] != 0.0)) && (s.v[1911] != 0.0)) && (s.v[1913] != 0.0)) {
            s.store_scale_ad(168, A::add(A::sub(s.ad_value(816), A::mul(s.ad_value(817), s.ad_value(228))), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(816), A::mul(s.ad_value(817), s.ad_value(228))), A::sub(s.ad_value(816), A::mul(s.ad_value(817), s.ad_value(228)))), ((4.0 * 1e-6) * 1e-6)))), 0.5);
        }

        s.v[1914] = if (s.v[818] < 0.01) { 1.0 } else { 0.0 };

        if (((((!(s.v[1620] != 0.0)) && (s.v[1904] != 0.0)) && (s.v[1911] != 0.0)) && (s.v[1913] != 0.0)) && (s.v[1914] != 0.0)) {
            s.store_scalar(818, 0.01);
        }

        if ((((!(s.v[1620] != 0.0)) && (s.v[1904] != 0.0)) && (s.v[1911] != 0.0)) && (!(s.v[1913] != 0.0))) {
            s.store_sub_ad_rhs(168, 816, A::mul(s.ad_value(817), s.ad_value(228)));
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1904] != 0.0)) && (s.v[1911] != 0.0)) {
            s.store_offset_ad(169, A::mul(s.ad_value(818), s.ad_value(228)), 1.0);
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1904] != 0.0)) && (s.v[1911] != 0.0)) {
            s.store_mul_ad_lhs(170, A::mul(s.ad_value(491), s.ad_value(168)), 169);
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1904] != 0.0)) && (s.v[1911] != 0.0)) {
            s.store_ad(171, &A::limited_exp(s.ad_value(170)));
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1904] != 0.0)) && (s.v[1911] != 0.0)) {
            s.store_mul_ad_lhs(485, A::mul(A::scale(s.ad_value(662), p.p2), s.ad_value(489)), 824);
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1904] != 0.0)) && (s.v[1911] != 0.0)) {
            s.store_mul_ad_lhs(483, A::mul(A::mul(s.ad_value(485), s.ad_value(203)), s.ad_value(228)), 171);
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1904] != 0.0)) && (s.v[1911] != 0.0)) {
            s.store_sub(169, 204, 219);
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1904] != 0.0)) && (s.v[1911] != 0.0)) {
            s.store_sqrt_ad(229, A::offset(A::square(s.ad_value(169)), 0.0001));
        }

        s.v[1915] = if (p.p1295 == 1.0) { 1.0 } else { 0.0 };

        if ((((!(s.v[1620] != 0.0)) && (s.v[1904] != 0.0)) && (s.v[1911] != 0.0)) && (s.v[1915] != 0.0)) {
            s.store_scale_ad(168, A::add(A::sub(s.ad_value(819), A::mul(s.ad_value(820), s.ad_value(229))), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(819), A::mul(s.ad_value(820), s.ad_value(229))), A::sub(s.ad_value(819), A::mul(s.ad_value(820), s.ad_value(229)))), ((4.0 * 1e-6) * 1e-6)))), 0.5);
        }

        s.v[1916] = if (s.v[821] < 0.01) { 1.0 } else { 0.0 };

        if (((((!(s.v[1620] != 0.0)) && (s.v[1904] != 0.0)) && (s.v[1911] != 0.0)) && (s.v[1915] != 0.0)) && (s.v[1916] != 0.0)) {
            s.store_scalar(821, 0.01);
        }

        if ((((!(s.v[1620] != 0.0)) && (s.v[1904] != 0.0)) && (s.v[1911] != 0.0)) && (!(s.v[1915] != 0.0))) {
            s.store_sub_ad_rhs(168, 819, A::mul(s.ad_value(820), s.ad_value(229)));
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1904] != 0.0)) && (s.v[1911] != 0.0)) {
            s.store_offset_ad(169, A::mul(s.ad_value(821), s.ad_value(229)), 1.0);
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1904] != 0.0)) && (s.v[1911] != 0.0)) {
            s.store_mul_ad_lhs(170, A::mul(s.ad_value(491), s.ad_value(168)), 169);
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1904] != 0.0)) && (s.v[1911] != 0.0)) {
            s.store_ad(171, &A::limited_exp(s.ad_value(170)));
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1904] != 0.0)) && (s.v[1911] != 0.0)) {
            s.store_mul_ad_lhs(486, A::mul(A::scale(s.ad_value(662), p.p2), s.ad_value(490)), 825);
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1904] != 0.0)) && (s.v[1911] != 0.0)) {
            s.store_mul_ad_lhs(484, A::mul(A::mul(s.ad_value(486), s.ad_value(204)), s.ad_value(229)), 171);
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_mul(1098, 379, 483);
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_mul(1099, 379, 484);
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_mul(1102, 379, 477);
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_mul(1100, 379, 479);
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_mul(1101, 379, 480);
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_mul(502, 666, 463);
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_mul(505, 667, 494);
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_scale(508, 671, (s.v[189] * p.p2));
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_scalar(503, ((0.1) as f64).powf((-p.p913)));
        }

        s.v[1917] = if (p.p913 == 1.0) { 1.0 } else { 0.0 };

        if ((!(s.v[1620] != 0.0)) && (s.v[1917] != 0.0)) {
            s.store_scalar(504, (1.5 - ((0.1) as f64).ln()));
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1917] != 0.0))) {
            s.store_scale_ad(504, A::sub_from_scalar(1.0, A::scale(s.ad_value(503), ((0.05 * p.p913) * (1.0 + p.p913)))), (1.0 / (1.0 - p.p913)));
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_scalar(506, ((0.1) as f64).powf((-p.p915)));
        }

        s.v[1918] = if (p.p915 == 1.0) { 1.0 } else { 0.0 };

        if ((!(s.v[1620] != 0.0)) && (s.v[1918] != 0.0)) {
            s.store_scalar(507, (1.5 - ((0.1) as f64).ln()));
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1918] != 0.0))) {
            s.store_scale_ad(507, A::sub_from_scalar(1.0, A::scale(s.ad_value(506), ((0.05 * p.p915) * (1.0 + p.p915)))), (1.0 / (1.0 - p.p915)));
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_scalar(509, ((0.1) as f64).powf((-p.p917)));
        }

        s.v[1919] = if (p.p917 == 1.0) { 1.0 } else { 0.0 };

        if ((!(s.v[1620] != 0.0)) && (s.v[1919] != 0.0)) {
            s.store_scalar(510, (1.5 - ((0.1) as f64).ln()));
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1919] != 0.0))) {
            s.store_scale_ad(510, A::sub_from_scalar(1.0, A::scale(s.ad_value(509), ((0.05 * p.p917) * (1.0 + p.p917)))), (1.0 / (1.0 - p.p917)));
        }

        s.v[1920] = if (s.v[502] > 0.0) { 1.0 } else { 0.0 };

        if ((!(s.v[1620] != 0.0)) && (s.v[1920] != 0.0)) {
            s.store_div(168, 498, 672);
        }

    }

    pub(super) fn stamp_transient_block_37(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        s.v[1921] = if (s.v[168] < 0.9) { 1.0 } else { 0.0 };

        if (((!(s.v[1620] != 0.0)) && (s.v[1920] != 0.0)) && (s.v[1921] != 0.0)) {
            s.store_sub_from_scalar(500, 1.0, 168);
        }

        s.v[1922] = if (p.p913 != 1.0) { 1.0 } else { 0.0 };

        s.v[1923] = if (p.p913 == 0.5) { 1.0 } else { 0.0 };

        if (((((!(s.v[1620] != 0.0)) && (s.v[1920] != 0.0)) && (s.v[1921] != 0.0)) && (s.v[1922] != 0.0)) && (s.v[1923] != 0.0)) {
            s.store_div_from_scalar_ad(501, 1.0, A::sqrt(s.ad_value(500)));
        }

        if (((((!(s.v[1620] != 0.0)) && (s.v[1920] != 0.0)) && (s.v[1921] != 0.0)) && (s.v[1922] != 0.0)) && (!(s.v[1923] != 0.0))) {
            s.store_limited_exp_ad(501, A::scale(A::ln(s.ad_value(500)), (-p.p913)));
        }

        if ((((!(s.v[1620] != 0.0)) && (s.v[1920] != 0.0)) && (s.v[1921] != 0.0)) && (s.v[1922] != 0.0)) {
            s.store_scale_ad(521, A::mul(A::mul(s.ad_value(672), s.ad_value(502)), A::sub_from_scalar(1.0, A::mul(s.ad_value(500), s.ad_value(501)))), 1.0 / ((1.0 - p.p913)));
        }

        if ((((!(s.v[1620] != 0.0)) && (s.v[1920] != 0.0)) && (s.v[1921] != 0.0)) && (!(s.v[1922] != 0.0))) {
            s.store_mul_ad(521, A::mul(s.ad_value(672), s.ad_value(502)), A::neg(A::ln(s.ad_value(500))));
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1920] != 0.0)) && (!(s.v[1921] != 0.0))) {
            s.store_mul_ad(169, A::mul(s.ad_value(503), A::offset(s.ad_value(168), (-1.0))), A::offset(A::scale(A::offset(s.ad_value(168), (-1.0)), (5.0 * p.p913)), (1.0 + p.p913)));
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1920] != 0.0)) && (!(s.v[1921] != 0.0))) {
            s.store_mul_ad(521, A::mul(s.ad_value(672), s.ad_value(502)), A::add(s.ad_value(169), s.ad_value(504)));
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1920] != 0.0))) {
            s.store_scalar(521, 0.0);
        }

        s.v[1924] = if (s.v[505] > 0.0) { 1.0 } else { 0.0 };

        if ((!(s.v[1620] != 0.0)) && (s.v[1924] != 0.0)) {
            s.store_div(168, 498, 673);
        }

        s.v[1925] = if (s.v[168] < 0.9) { 1.0 } else { 0.0 };

        if (((!(s.v[1620] != 0.0)) && (s.v[1924] != 0.0)) && (s.v[1925] != 0.0)) {
            s.store_sub_from_scalar(500, 1.0, 168);
        }

        s.v[1926] = if (p.p915 != 1.0) { 1.0 } else { 0.0 };

        s.v[1927] = if (p.p915 == 0.5) { 1.0 } else { 0.0 };

        if (((((!(s.v[1620] != 0.0)) && (s.v[1924] != 0.0)) && (s.v[1925] != 0.0)) && (s.v[1926] != 0.0)) && (s.v[1927] != 0.0)) {
            s.store_div_from_scalar_ad(501, 1.0, A::sqrt(s.ad_value(500)));
        }

        if (((((!(s.v[1620] != 0.0)) && (s.v[1924] != 0.0)) && (s.v[1925] != 0.0)) && (s.v[1926] != 0.0)) && (!(s.v[1927] != 0.0))) {
            s.store_limited_exp_ad(501, A::scale(A::ln(s.ad_value(500)), (-p.p915)));
        }

        if ((((!(s.v[1620] != 0.0)) && (s.v[1924] != 0.0)) && (s.v[1925] != 0.0)) && (s.v[1926] != 0.0)) {
            s.store_scale_ad(522, A::mul(A::mul(s.ad_value(673), s.ad_value(505)), A::sub_from_scalar(1.0, A::mul(s.ad_value(500), s.ad_value(501)))), 1.0 / ((1.0 - p.p915)));
        }

        if ((((!(s.v[1620] != 0.0)) && (s.v[1924] != 0.0)) && (s.v[1925] != 0.0)) && (!(s.v[1926] != 0.0))) {
            s.store_mul_ad(522, A::mul(s.ad_value(673), s.ad_value(505)), A::neg(A::ln(s.ad_value(500))));
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1924] != 0.0)) && (!(s.v[1925] != 0.0))) {
            s.store_mul_ad(169, A::mul(s.ad_value(506), A::offset(s.ad_value(168), (-1.0))), A::offset(A::scale(A::offset(s.ad_value(168), (-1.0)), (5.0 * p.p915)), (1.0 + p.p915)));
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1924] != 0.0)) && (!(s.v[1925] != 0.0))) {
            s.store_mul_ad(522, A::mul(s.ad_value(673), s.ad_value(505)), A::add(s.ad_value(169), s.ad_value(507)));
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1924] != 0.0))) {
            s.store_scalar(522, 0.0);
        }

        s.v[1928] = if (s.v[508] > 0.0) { 1.0 } else { 0.0 };

        if ((!(s.v[1620] != 0.0)) && (s.v[1928] != 0.0)) {
            s.store_div(168, 498, 674);
        }

        s.v[1929] = if (s.v[168] < 0.9) { 1.0 } else { 0.0 };

        if (((!(s.v[1620] != 0.0)) && (s.v[1928] != 0.0)) && (s.v[1929] != 0.0)) {
            s.store_sub_from_scalar(500, 1.0, 168);
        }

        s.v[1930] = if (p.p917 != 1.0) { 1.0 } else { 0.0 };

        s.v[1931] = if (p.p917 == 0.5) { 1.0 } else { 0.0 };

        if (((((!(s.v[1620] != 0.0)) && (s.v[1928] != 0.0)) && (s.v[1929] != 0.0)) && (s.v[1930] != 0.0)) && (s.v[1931] != 0.0)) {
            s.store_div_from_scalar_ad(501, 1.0, A::sqrt(s.ad_value(500)));
        }

        if (((((!(s.v[1620] != 0.0)) && (s.v[1928] != 0.0)) && (s.v[1929] != 0.0)) && (s.v[1930] != 0.0)) && (!(s.v[1931] != 0.0))) {
            s.store_limited_exp_ad(501, A::scale(A::ln(s.ad_value(500)), (-p.p917)));
        }

        if ((((!(s.v[1620] != 0.0)) && (s.v[1928] != 0.0)) && (s.v[1929] != 0.0)) && (s.v[1930] != 0.0)) {
            s.store_scale_ad(523, A::mul(A::mul(s.ad_value(674), s.ad_value(508)), A::sub_from_scalar(1.0, A::mul(s.ad_value(500), s.ad_value(501)))), 1.0 / ((1.0 - p.p917)));
        }

        if ((((!(s.v[1620] != 0.0)) && (s.v[1928] != 0.0)) && (s.v[1929] != 0.0)) && (!(s.v[1930] != 0.0))) {
            s.store_mul_ad(523, A::mul(s.ad_value(674), s.ad_value(508)), A::neg(A::ln(s.ad_value(500))));
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1928] != 0.0)) && (!(s.v[1929] != 0.0))) {
            s.store_mul_ad(169, A::mul(s.ad_value(509), A::offset(s.ad_value(168), (-1.0))), A::offset(A::scale(A::offset(s.ad_value(168), (-1.0)), (5.0 * p.p917)), (1.0 + p.p917)));
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1928] != 0.0)) && (!(s.v[1929] != 0.0))) {
            s.store_mul_ad(523, A::mul(s.ad_value(674), s.ad_value(508)), A::add(s.ad_value(169), s.ad_value(510)));
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1928] != 0.0))) {
            s.store_scalar(523, 0.0);
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_scale(524, 533, (p.p919 * p.p2));
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_add_ad_lhs(520, A::add(A::add(s.ad_value(521), s.ad_value(522)), s.ad_value(523)), 524);
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_mul(511, 669, 464);
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_mul(514, 670, 495);
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_scale(517, 668, (s.v[189] * p.p2));
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_scalar(512, ((0.1) as f64).powf((-p.p914)));
        }

        s.v[1932] = if (p.p914 == 1.0) { 1.0 } else { 0.0 };

        if ((!(s.v[1620] != 0.0)) && (s.v[1932] != 0.0)) {
            s.store_scalar(513, (1.5 - ((0.1) as f64).ln()));
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1932] != 0.0))) {
            s.store_scale_ad(513, A::sub_from_scalar(1.0, A::scale(s.ad_value(512), ((0.05 * p.p914) * (1.0 + p.p914)))), (1.0 / (1.0 - p.p914)));
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_scalar(515, ((0.1) as f64).powf((-p.p916)));
        }

        s.v[1933] = if (p.p916 == 1.0) { 1.0 } else { 0.0 };

        if ((!(s.v[1620] != 0.0)) && (s.v[1933] != 0.0)) {
            s.store_scalar(516, (1.5 - ((0.1) as f64).ln()));
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1933] != 0.0))) {
            s.store_scale_ad(516, A::sub_from_scalar(1.0, A::scale(s.ad_value(515), ((0.05 * p.p916) * (1.0 + p.p916)))), (1.0 / (1.0 - p.p916)));
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_scalar(518, ((0.1) as f64).powf((-p.p918)));
        }

        s.v[1934] = if (p.p918 == 1.0) { 1.0 } else { 0.0 };

        if ((!(s.v[1620] != 0.0)) && (s.v[1934] != 0.0)) {
            s.store_scalar(519, (1.5 - ((0.1) as f64).ln()));
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1934] != 0.0))) {
            s.store_scale_ad(519, A::sub_from_scalar(1.0, A::scale(s.ad_value(518), ((0.05 * p.p918) * (1.0 + p.p918)))), (1.0 / (1.0 - p.p918)));
        }

        s.v[1935] = if (s.v[511] > 0.0) { 1.0 } else { 0.0 };

        if ((!(s.v[1620] != 0.0)) && (s.v[1935] != 0.0)) {
            s.store_div(168, 499, 675);
        }

        s.v[1936] = if (s.v[168] < 0.9) { 1.0 } else { 0.0 };

        if (((!(s.v[1620] != 0.0)) && (s.v[1935] != 0.0)) && (s.v[1936] != 0.0)) {
            s.store_sub_from_scalar(500, 1.0, 168);
        }

        s.v[1937] = if (p.p914 != 1.0) { 1.0 } else { 0.0 };

        s.v[1938] = if (p.p914 == 0.5) { 1.0 } else { 0.0 };

        if (((((!(s.v[1620] != 0.0)) && (s.v[1935] != 0.0)) && (s.v[1936] != 0.0)) && (s.v[1937] != 0.0)) && (s.v[1938] != 0.0)) {
            s.store_div_from_scalar_ad(501, 1.0, A::sqrt(s.ad_value(500)));
        }

        if (((((!(s.v[1620] != 0.0)) && (s.v[1935] != 0.0)) && (s.v[1936] != 0.0)) && (s.v[1937] != 0.0)) && (!(s.v[1938] != 0.0))) {
            s.store_limited_exp_ad(501, A::scale(A::ln(s.ad_value(500)), (-p.p914)));
        }

        if ((((!(s.v[1620] != 0.0)) && (s.v[1935] != 0.0)) && (s.v[1936] != 0.0)) && (s.v[1937] != 0.0)) {
            s.store_scale_ad(526, A::mul(A::mul(s.ad_value(675), s.ad_value(511)), A::sub_from_scalar(1.0, A::mul(s.ad_value(500), s.ad_value(501)))), 1.0 / ((1.0 - p.p914)));
        }

        if ((((!(s.v[1620] != 0.0)) && (s.v[1935] != 0.0)) && (s.v[1936] != 0.0)) && (!(s.v[1937] != 0.0))) {
            s.store_mul_ad(526, A::mul(s.ad_value(675), s.ad_value(511)), A::neg(A::ln(s.ad_value(500))));
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1935] != 0.0)) && (!(s.v[1936] != 0.0))) {
            s.store_mul_ad(169, A::mul(s.ad_value(512), A::offset(s.ad_value(168), (-1.0))), A::offset(A::scale(A::offset(s.ad_value(168), (-1.0)), (5.0 * p.p914)), (1.0 + p.p914)));
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1935] != 0.0)) && (!(s.v[1936] != 0.0))) {
            s.store_mul_ad(526, A::mul(s.ad_value(675), s.ad_value(511)), A::add(s.ad_value(169), s.ad_value(513)));
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1935] != 0.0))) {
            s.store_scalar(526, 0.0);
        }

        s.v[1939] = if (s.v[514] > 0.0) { 1.0 } else { 0.0 };

        if ((!(s.v[1620] != 0.0)) && (s.v[1939] != 0.0)) {
            s.store_div(168, 499, 676);
        }

        s.v[1940] = if (s.v[168] < 0.9) { 1.0 } else { 0.0 };

        if (((!(s.v[1620] != 0.0)) && (s.v[1939] != 0.0)) && (s.v[1940] != 0.0)) {
            s.store_sub_from_scalar(500, 1.0, 168);
        }

        s.v[1941] = if (p.p916 != 1.0) { 1.0 } else { 0.0 };

        s.v[1942] = if (p.p916 == 0.5) { 1.0 } else { 0.0 };

        if (((((!(s.v[1620] != 0.0)) && (s.v[1939] != 0.0)) && (s.v[1940] != 0.0)) && (s.v[1941] != 0.0)) && (s.v[1942] != 0.0)) {
            s.store_div_from_scalar_ad(501, 1.0, A::sqrt(s.ad_value(500)));
        }

        if (((((!(s.v[1620] != 0.0)) && (s.v[1939] != 0.0)) && (s.v[1940] != 0.0)) && (s.v[1941] != 0.0)) && (!(s.v[1942] != 0.0))) {
            s.store_limited_exp_ad(501, A::scale(A::ln(s.ad_value(500)), (-p.p916)));
        }

        if ((((!(s.v[1620] != 0.0)) && (s.v[1939] != 0.0)) && (s.v[1940] != 0.0)) && (s.v[1941] != 0.0)) {
            s.store_scale_ad(527, A::mul(A::mul(s.ad_value(676), s.ad_value(514)), A::sub_from_scalar(1.0, A::mul(s.ad_value(500), s.ad_value(501)))), 1.0 / ((1.0 - p.p916)));
        }

        if ((((!(s.v[1620] != 0.0)) && (s.v[1939] != 0.0)) && (s.v[1940] != 0.0)) && (!(s.v[1941] != 0.0))) {
            s.store_mul_ad(527, A::mul(s.ad_value(676), s.ad_value(514)), A::neg(A::ln(s.ad_value(500))));
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1939] != 0.0)) && (!(s.v[1940] != 0.0))) {
            s.store_mul_ad(169, A::mul(s.ad_value(515), A::offset(s.ad_value(168), (-1.0))), A::offset(A::scale(A::offset(s.ad_value(168), (-1.0)), (5.0 * p.p916)), (1.0 + p.p916)));
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1939] != 0.0)) && (!(s.v[1940] != 0.0))) {
            s.store_mul_ad(527, A::mul(s.ad_value(676), s.ad_value(514)), A::add(s.ad_value(169), s.ad_value(516)));
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1939] != 0.0))) {
            s.store_scalar(527, 0.0);
        }

        s.v[1943] = if (s.v[517] > 0.0) { 1.0 } else { 0.0 };

        if ((!(s.v[1620] != 0.0)) && (s.v[1943] != 0.0)) {
            s.store_div(168, 499, 677);
        }

        s.v[1944] = if (s.v[168] < 0.9) { 1.0 } else { 0.0 };

        if (((!(s.v[1620] != 0.0)) && (s.v[1943] != 0.0)) && (s.v[1944] != 0.0)) {
            s.store_sub_from_scalar(500, 1.0, 168);
        }

        s.v[1945] = if (p.p918 != 1.0) { 1.0 } else { 0.0 };

        s.v[1946] = if (p.p918 == 0.5) { 1.0 } else { 0.0 };

        if (((((!(s.v[1620] != 0.0)) && (s.v[1943] != 0.0)) && (s.v[1944] != 0.0)) && (s.v[1945] != 0.0)) && (s.v[1946] != 0.0)) {
            s.store_div_from_scalar_ad(501, 1.0, A::sqrt(s.ad_value(500)));
        }

        if (((((!(s.v[1620] != 0.0)) && (s.v[1943] != 0.0)) && (s.v[1944] != 0.0)) && (s.v[1945] != 0.0)) && (!(s.v[1946] != 0.0))) {
            s.store_limited_exp_ad(501, A::scale(A::ln(s.ad_value(500)), (-p.p918)));
        }

        if ((((!(s.v[1620] != 0.0)) && (s.v[1943] != 0.0)) && (s.v[1944] != 0.0)) && (s.v[1945] != 0.0)) {
            s.store_scale_ad(528, A::mul(A::mul(s.ad_value(677), s.ad_value(517)), A::sub_from_scalar(1.0, A::mul(s.ad_value(500), s.ad_value(501)))), 1.0 / ((1.0 - p.p918)));
        }

        if ((((!(s.v[1620] != 0.0)) && (s.v[1943] != 0.0)) && (s.v[1944] != 0.0)) && (!(s.v[1945] != 0.0))) {
            s.store_mul_ad(528, A::mul(s.ad_value(677), s.ad_value(517)), A::neg(A::ln(s.ad_value(500))));
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1943] != 0.0)) && (!(s.v[1944] != 0.0))) {
            s.store_mul_ad(169, A::mul(s.ad_value(518), A::offset(s.ad_value(168), (-1.0))), A::offset(A::scale(A::offset(s.ad_value(168), (-1.0)), (5.0 * p.p918)), (1.0 + p.p918)));
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1943] != 0.0)) && (!(s.v[1944] != 0.0))) {
            s.store_mul_ad(528, A::mul(s.ad_value(677), s.ad_value(517)), A::add(s.ad_value(169), s.ad_value(519)));
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1943] != 0.0))) {
            s.store_scalar(528, 0.0);
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_scale(529, 534, (p.p919 * p.p2));
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_add_ad_lhs(525, A::add(A::add(s.ad_value(526), s.ad_value(527)), s.ad_value(528)), 529);
        }

        s.v[1947] = if (p.p28 != 0.0) { 1.0 } else { 0.0 };

        if ((!(s.v[1620] != 0.0)) && (s.v[1947] != 0.0)) {
            s.store_powf_ad(168, A::scale(s.ad_value(706), 1.0000000000000001e-23), p.p1144);
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1947] != 0.0)) {
            s.store_powf_ad(169, A::div_from_scalar(300.0, s.ad_value(635)), p.p1145);
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1947] != 0.0)) {
            s.store_div_ad_lhs(170, A::mul(A::scale(s.ad_value(379), p.p1143), A::voltage(ctx, &nodes, Some(10), Some(7))), 271);
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1947] != 0.0)) {
            s.store_scale_ad(975, A::limited_exp(A::mul(A::neg(s.ad_value(168)), s.ad_value(169))), p.p1138);
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1947] != 0.0)) {
            s.store_mul_ad_lhs(976, A::scale(s.ad_value(169), p.p1139), 168);
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1947] != 0.0)) {
            s.store_scale_ad(977, A::tanh(A::limited_exp(A::mul(A::scale(s.ad_value(379), p.p1142), A::sub(A::sub(A::voltage(ctx, &nodes, Some(8), Some(10)), s.ad_value(1128)), A::voltage(ctx, &nodes, Some(7), Some(10)))))), p.p1141);
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1947] != 0.0)) {
            let assign53180_ad_e87578: A = A::mul(A::mul(A::mul(A::mul(A::mul(A::scale(s.ad_value(211), (p.p2 * s.v[183])), s.ad_value(975)), A::limited_exp(s.ad_value(170))), A::limited_exp(A::scale(A::neg(s.ad_value(976)), s.v[184]))), A::limited_exp(A::div(s.ad_value(977), s.ad_value(271)))), A::offset(A::limited_exp(A::div(A::scale(s.ad_value(227), p.p1140), s.ad_value(271))), (-1.0)));
            s.store_ad(974, &assign53180_ad_e87578);
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_scale(621, 271, (4.0 * 1.602176462e-19));
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_div_ad_lhs(607, A::scale(s.ad_value(746), 2.0), 337);
        }

        s.v[1948] = if (p.p1011 <= 0.0) { 1.0 } else { 0.0 };

        if ((!(s.v[1620] != 0.0)) && (s.v[1948] != 0.0)) {
            s.store_scalar(610, 0.0);
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1948] != 0.0))) {
            s.store_div_ad_lhs(167, A::offset(A::div(s.ad_value(355), s.ad_value(300)), p.p1011), 607);
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1948] != 0.0))) {
            s.store_mul_ad_rhs(610, 300, A::ln(A::max_with_scalar(s.ad_value(167), 1e-38)));
        }

        s.v[1949] = if (s.v[610] < 0.0) { 1.0 } else { 0.0 };

        if (((!(s.v[1620] != 0.0)) && (!(s.v[1948] != 0.0))) && (s.v[1949] != 0.0)) {
            s.store_scalar(610, 0.0);
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_mul_ad(613, A::scale(s.ad_value(271), 6.241509744511525e18), A::add(A::offset(s.ad_value(260), s.v[199]), s.ad_value(709)));
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_scale_ad(612, A::mul(A::mul(A::mul(A::mul(A::scale(s.ad_value(253), (2.0 * s.v[199])), s.ad_value(271)), s.ad_value(320)), s.ad_value(853)), s.ad_value(834)), 6.241509744511525e18);
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_mul_ad_lhs(1004, A::mul(A::scale(s.ad_value(271), ((1.602176462e-19 * 1.602176462e-19) * 1.602176462e-19)), A::abs(s.ad_value(380))), 337);
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_mul_ad_lhs(1005, A::mul(A::scale(s.ad_value(271), 1.602176462e-19), s.ad_value(380)), 380);
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_add_ad(1006, A::offset(A::scale(s.ad_value(612), p.p1013), p.p1012), A::mul(A::scale(s.ad_value(612), p.p1014), s.ad_value(612)));
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_mul_ad(1007, A::add(s.ad_value(612), s.ad_value(613)), A::add(s.ad_value(612), s.ad_value(613)));
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_scale(1008, 271, (p.p1012 * 1.602176462e-19));
        }

        s.v[1950] = if (p.p1319 == 1.0) { 1.0 } else { 0.0 };

        if ((!(s.v[1620] != 0.0)) && (s.v[1950] != 0.0)) {
            s.store_scalar(1014, p.p1320);
        }

        s.v[1951] = if (s.v[184] > s.v[1014]) { 1.0 } else { 0.0 };

        if (((!(s.v[1620] != 0.0)) && (s.v[1950] != 0.0)) && (s.v[1951] != 0.0)) {
            s.store_sub_from_scalar(167, s.v[184], 1014);
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1950] != 0.0)) && (!(s.v[1951] != 0.0))) {
            s.store_scalar(1014, s.v[184]);
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1950] != 0.0)) && (!(s.v[1951] != 0.0))) {
            s.copy_ad(167, 1014);
        }

        s.v[1952] = if (p.p1015 >= (s.v[167] / 2.0)) { 1.0 } else { 0.0 };

        if (((!(s.v[1620] != 0.0)) && (s.v[1950] != 0.0)) && (s.v[1952] != 0.0)) {
            s.store_scalar(606, 0.0);
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1950] != 0.0)) && (!(s.v[1952] != 0.0))) {
            s.store_scalar(606, p.p1015);
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1950] != 0.0)) {
            s.store_scalar(1013, s.v[184]);
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1950] != 0.0)) {
            s.store_div_ad_lhs(980, A::sub(s.ad_value(221), s.ad_value(707)), 271);
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1950] != 0.0)) {
            s.store_scale_ad(981, A::sqrt(A::div_from_scalar((((2.0 * 1.602176462e-19) * s.v[180]) * p.p1322), s.ad_value(271))), 1.0 / (s.v[199]));
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1950] != 0.0)) {
            s.store_ln_ad(982, A::div_from_scalar(p.p1322, s.ad_value(182)));
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1950] != 0.0)) {
            s.store_scalar(168, 1.0);
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1950] != 0.0)) {
            s.store_div(404, 980, 168);
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1950] != 0.0)) {
            s.store_div(405, 981, 168);
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1950] != 0.0)) {
            s.store_sub_ad(168, A::scale(s.ad_value(404), 0.5), A::scale(A::offset(A::scale(s.ad_value(405), 0.7071067811865475), 1.0), 3.0));
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1950] != 0.0)) {
            s.store_add_ad_rhs(169, 168, A::sqrt(A::add(A::square(s.ad_value(168)), A::scale(s.ad_value(404), 6.0))));
        }

        s.v[1953] = if (s.v[404] < 0.0) { 1.0 } else { 0.0 };

        if (((!(s.v[1620] != 0.0)) && (s.v[1950] != 0.0)) && (s.v[1953] != 0.0)) {
            s.store_div_ad_lhs(170, A::sub(s.ad_value(404), s.ad_value(169)), 405);
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1950] != 0.0)) && (s.v[1953] != 0.0)) {
            s.store_neg_ad(983, A::ln(A::max_with_scalar(A::add(A::sub_from_scalar(1.0, s.ad_value(169)), A::square(s.ad_value(170))), 1e-38)));
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1950] != 0.0)) && (!(s.v[1953] != 0.0))) {
            s.store_limited_exp_ad(170, A::neg(s.ad_value(169)));
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1950] != 0.0)) && (!(s.v[1953] != 0.0))) {
            s.store_scale(168, 405, 0.5);
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1950] != 0.0)) && (!(s.v[1953] != 0.0))) {
            s.store_sub_ad_lhs(169, A::sqrt(A::add(A::add(A::offset(s.ad_value(404), (-1.0)), s.ad_value(170)), A::square(s.ad_value(168)))), 168);
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1950] != 0.0)) && (!(s.v[1953] != 0.0))) {
            s.store_sub_ad_lhs(983, A::offset(A::square(s.ad_value(169)), 1.0), 170);
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1950] != 0.0)) {
            s.store_scale_ad(175, A::add(A::offset(s.ad_value(983), 1.0), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(983), (-1.0)), A::offset(s.ad_value(983), (-1.0))), ((0.25 * 2.0) * 2.0)))), 0.5);
        }

    }

    pub(super) fn stamp_transient_block_38(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if ((!(s.v[1620] != 0.0)) && (s.v[1950] != 0.0)) {
            s.store_sqrt(259, 175);
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1950] != 0.0)) {
            s.store_div_ad_lhs(167, A::offset(A::div(s.ad_value(981), A::scale(s.ad_value(259), 2.0)), 1.0), 981);
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1950] != 0.0)) {
            s.store_sub_ad_lhs(168, A::sub(s.ad_value(983), A::scale(s.ad_value(982), 2.0)), 225);
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1950] != 0.0)) {
            s.store_sub_ad_rhs(169, 168, A::ln(A::max_with_scalar(A::mul(A::scale(s.ad_value(167), 4.0), s.ad_value(259)), 1e-38)));
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1950] != 0.0)) {
            s.store_scale_ad(175, A::sub(A::offset(s.ad_value(169), (-0.201491)), A::sqrt(A::offset(A::mul(s.ad_value(169), A::offset(s.ad_value(169), 0.402982)), 2.446562))), 0.5);
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1950] != 0.0)) {
            s.copy_ad(257, 259);
        }

        s.v[1954] = if (s.v[175] <= (-68.0)) { 1.0 } else { 0.0 };

        if (((!(s.v[1620] != 0.0)) && (s.v[1950] != 0.0)) && (s.v[1954] != 0.0)) {
            s.store_scalar(171, (-100.0));
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1950] != 0.0)) && (s.v[1954] != 0.0)) {
            s.store_scalar(172, 20.0);
        }

        s.v[1955] = if (s.v[175] < (s.v[171] - (0.5 * s.v[172]))) { 1.0 } else { 0.0 };

        if ((((!(s.v[1620] != 0.0)) && (s.v[1950] != 0.0)) && (s.v[1954] != 0.0)) && (s.v[1955] != 0.0)) {
            s.store_ad(170, &A::limited_exp(s.ad_value(171)));
        }

        s.v[1956] = if (s.v[175] > (s.v[171] + (0.5 * s.v[172]))) { 1.0 } else { 0.0 };

        if (((((!(s.v[1620] != 0.0)) && (s.v[1950] != 0.0)) && (s.v[1954] != 0.0)) && (!(s.v[1955] != 0.0))) && (s.v[1956] != 0.0)) {
            s.store_ad(170, &A::limited_exp(s.ad_value(175)));
        }

        if (((((!(s.v[1620] != 0.0)) && (s.v[1950] != 0.0)) && (s.v[1954] != 0.0)) && (!(s.v[1955] != 0.0))) && (!(s.v[1956] != 0.0))) {
            s.store_div_ad_lhs(169, A::sub(s.ad_value(175), s.ad_value(171)), 172);
        }

        if (((((!(s.v[1620] != 0.0)) && (s.v[1950] != 0.0)) && (s.v[1954] != 0.0)) && (!(s.v[1955] != 0.0))) && (!(s.v[1956] != 0.0))) {
            s.store_square(173, 169);
        }

        if (((((!(s.v[1620] != 0.0)) && (s.v[1950] != 0.0)) && (s.v[1954] != 0.0)) && (!(s.v[1955] != 0.0))) && (!(s.v[1956] != 0.0))) {
            s.store_limited_exp_ad(170, A::add(s.ad_value(171), A::mul(s.ad_value(172), A::add(A::offset(A::scale(s.ad_value(169), 0.5), (5.0 / 64.0)), A::mul(s.ad_value(173), A::sub_from_scalar((15.0 / 16.0), A::mul(s.ad_value(173), A::sub_from_scalar(1.25, s.ad_value(173)))))))));
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1950] != 0.0)) && (s.v[1954] != 0.0)) {
            s.store_mul_ad_rhs(985, 170, A::sub(A::sub(A::offset(s.ad_value(168), 1.0), s.ad_value(175)), A::ln(A::max_with_scalar(A::mul(A::scale(s.ad_value(167), 2.0), A::add(A::mul(A::scale(s.ad_value(170), 2.0), s.ad_value(167)), A::scale(s.ad_value(257), 2.0))), 1e-38))));
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1950] != 0.0)) && (!(s.v[1954] != 0.0))) {
            s.store_ad(170, &A::limited_exp(s.ad_value(175)));
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1950] != 0.0)) && (!(s.v[1954] != 0.0))) {
            s.store_div_from_scalar(258, 1.0, 257);
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1950] != 0.0)) && (!(s.v[1954] != 0.0))) {
            s.store_sub_ad_lhs(171, A::add(A::scale(s.ad_value(170), 2.0), A::ln(A::max_with_scalar(A::mul(A::mul(A::scale(s.ad_value(170), 2.0), s.ad_value(167)), A::add(A::mul(A::scale(s.ad_value(170), 2.0), s.ad_value(167)), A::scale(s.ad_value(257), 2.0))), 1e-38))), 168);
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1950] != 0.0)) && (!(s.v[1954] != 0.0))) {
            s.store_add_ad(172, A::offset(A::div_from_scalar(1.0, s.ad_value(170)), 2.0), A::div(A::add(s.ad_value(167), s.ad_value(258)), A::add(A::mul(s.ad_value(167), s.ad_value(170)), s.ad_value(257))));
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1950] != 0.0)) && (!(s.v[1954] != 0.0))) {
            s.store_sub_ad_rhs(170, 170, A::div(s.ad_value(171), s.ad_value(172)));
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1950] != 0.0)) && (!(s.v[1954] != 0.0))) {
            s.store_sub_ad_lhs(171, A::add(A::scale(s.ad_value(170), 2.0), A::ln(A::max_with_scalar(A::mul(A::mul(A::scale(s.ad_value(170), 2.0), s.ad_value(167)), A::add(A::mul(A::scale(s.ad_value(170), 2.0), s.ad_value(167)), A::scale(s.ad_value(257), 2.0))), 1e-38))), 168);
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1950] != 0.0)) && (!(s.v[1954] != 0.0))) {
            s.store_add_ad(172, A::offset(A::div_from_scalar(1.0, s.ad_value(170)), 2.0), A::div(A::add(s.ad_value(167), s.ad_value(258)), A::add(A::mul(s.ad_value(167), s.ad_value(170)), s.ad_value(257))));
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1950] != 0.0)) && (!(s.v[1954] != 0.0))) {
            s.store_mul_ad(173, A::div(A::add(s.ad_value(167), s.ad_value(258)), A::add(A::mul(s.ad_value(167), s.ad_value(170)), s.ad_value(257))), A::div(A::add(s.ad_value(167), s.ad_value(258)), A::add(A::mul(s.ad_value(167), s.ad_value(170)), s.ad_value(257))));
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1950] != 0.0)) && (!(s.v[1954] != 0.0))) {
            s.store_sub_ad_lhs(174, A::sub(A::neg(A::mul(A::div_from_scalar(1.0, s.ad_value(170)), A::div_from_scalar(1.0, s.ad_value(170)))), A::div_from_scalar(1.0, A::mul(A::mul(A::square(s.ad_value(257)), s.ad_value(257)), A::add(A::mul(s.ad_value(167), s.ad_value(170)), s.ad_value(257))))), 173);
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1950] != 0.0)) && (!(s.v[1954] != 0.0))) {
            s.store_sub_ad_rhs(985, 170, A::mul(A::div(s.ad_value(171), s.ad_value(172)), A::offset(A::div(A::mul(s.ad_value(171), s.ad_value(174)), A::mul(A::scale(s.ad_value(172), 2.0), s.ad_value(172))), 1.0)));
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1950] != 0.0)) {
            s.store_scale_ad(984, A::add(A::offset(s.ad_value(983), 1.0), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(983), (-1.0)), A::offset(s.ad_value(983), (-1.0))), ((0.25 * 2.0) * 2.0)))), 0.5);
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1950] != 0.0)) {
            s.store_offset_ad(986, A::div(s.ad_value(981), A::scale(A::sqrt(s.ad_value(984)), 2.0)), 1.0);
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1950] != 0.0)) {
            s.copy_ad(987, 337);
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1950] != 0.0)) {
            s.store_scale(994, 987, (s.v[199] * s.v[183]));
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1950] != 0.0)) {
            s.store_scale(993, 337, (s.v[199] * s.v[183]));
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1950] != 0.0)) {
            s.store_div_ad(988, A::mul(s.ad_value(380), s.ad_value(1014)), A::mul(A::mul(A::mul(A::scale(s.ad_value(986), 2.0), s.ad_value(994)), s.ad_value(271)), s.ad_value(271)));
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1950] != 0.0)) {
            s.store_div_ad(990, A::mul(s.ad_value(380), A::sub(s.ad_value(1013), s.ad_value(1014))), A::mul(A::mul(A::mul(A::scale(s.ad_value(253), 2.0), s.ad_value(993)), s.ad_value(269)), s.ad_value(269)));
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1950] != 0.0)) {
            s.store_offset_ad(167, A::scale(A::sub(A::add(A::square(s.ad_value(985)), s.ad_value(985)), s.ad_value(988)), 4.0), 1.0);
        }

        s.v[1957] = if (s.v[167] < 1.0) { 1.0 } else { 0.0 };

        if (((!(s.v[1620] != 0.0)) && (s.v[1950] != 0.0)) && (s.v[1957] != 0.0)) {
            s.store_scalar(989, 0.0);
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1950] != 0.0)) && (!(s.v[1957] != 0.0))) {
            s.store_offset_ad(989, A::scale(A::sqrt(s.ad_value(167)), 0.5), (-0.5));
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1950] != 0.0)) {
            s.store_offset_ad(991, A::scale(A::sqrt(A::offset(A::scale(A::add(A::add(A::square(s.ad_value(320)), s.ad_value(320)), s.ad_value(990)), 4.0), 1.0)), 0.5), (-0.5));
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1950] != 0.0)) {
            s.store_mul_ad_lhs(995, A::mul(A::mul(A::scale(s.ad_value(986), 2.0), s.ad_value(994)), s.ad_value(271)), 989);
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1950] != 0.0)) {
            s.store_mul_ad_lhs(996, A::mul(A::mul(A::scale(s.ad_value(253), 2.0), s.ad_value(993)), s.ad_value(271)), 320);
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1950] != 0.0)) {
            s.store_mul_ad(997, A::mul(A::scale(s.ad_value(993), 2.0), s.ad_value(271)), A::sub(s.ad_value(991), s.ad_value(320)));
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1950] != 0.0)) {
            s.store_mul_ad_rhs(998, 995, A::sub(s.ad_value(1013), s.ad_value(1014)));
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1950] != 0.0)) {
            s.store_add_ad(999, A::mul(s.ad_value(997), s.ad_value(1014)), A::mul(s.ad_value(996), s.ad_value(1014)));
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1950] != 0.0)) {
            s.store_div_ad(1010, A::div_from_scalar(1.0, A::add(s.ad_value(998), s.ad_value(999))), A::add(s.ad_value(998), s.ad_value(999)));
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1950] != 0.0)) {
            s.store_mul_ad_lhs(1011, A::square(s.ad_value(998)), 1010);
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1950] != 0.0)) {
            s.store_mul_ad_lhs(1012, A::square(s.ad_value(999)), 1010);
        }

        s.v[1958] = if (s.v[184] != s.v[1014]) { 1.0 } else { 0.0 };

        if (((!(s.v[1620] != 0.0)) && (s.v[1950] != 0.0)) && (s.v[1958] != 0.0)) {
            s.store_scale_ad(992, A::mul(A::mul(A::scale(s.ad_value(253), (2.0 * s.v[199])), s.ad_value(271)), s.ad_value(991)), 6.241509744511525e18);
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1950] != 0.0)) && (s.v[1958] != 0.0)) {
            s.store_sub_ad_lhs(608, A::sub(s.ad_value(1013), A::scale(s.ad_value(606), 2.0)), 1014);
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1950] != 0.0)) && (s.v[1958] != 0.0)) {
            s.store_square(609, 608);
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1950] != 0.0)) && (s.v[1958] != 0.0)) {
            s.store_scale(168, 609, (10000000000.0 * s.v[199]));
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1950] != 0.0)) && (s.v[1958] != 0.0)) {
            s.store_scale_ad(169, A::ln(A::max_with_scalar(A::div(A::add(s.ad_value(992), s.ad_value(613)), A::add(s.ad_value(612), s.ad_value(613))), 1e-38)), p.p1012);
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1950] != 0.0)) && (s.v[1958] != 0.0)) {
            s.store_scaled_sub(170, 992, 612, p.p1013);
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1950] != 0.0)) && (s.v[1958] != 0.0)) {
            s.store_scale_ad(171, A::sub(A::square(s.ad_value(992)), A::square(s.ad_value(612))), (0.5 * p.p1014));
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1950] != 0.0)) && (s.v[1958] != 0.0)) {
            s.store_scale(172, 609, (10000000000.0 * (s.v[183] * p.p2)));
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1950] != 0.0)) && (s.v[1958] != 0.0)) {
            s.store_add_ad(1000, A::mul(A::div(s.ad_value(1004), s.ad_value(168)), A::add(A::add(s.ad_value(169), s.ad_value(170)), s.ad_value(171))), A::div(A::mul(A::mul(A::div(s.ad_value(1005), s.ad_value(172)), s.ad_value(610)), s.ad_value(1006)), s.ad_value(1007)));
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1950] != 0.0)) && (s.v[1958] != 0.0)) {
            s.store_mul_ad_lhs(173, A::mul(A::scale(s.ad_value(608), ((s.v[183] * p.p2) * 10000000000.0)), s.ad_value(613)), 613);
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1950] != 0.0)) && (s.v[1958] != 0.0)) {
            s.store_mul_ad_lhs(1001, A::mul(A::div(s.ad_value(1008), s.ad_value(173)), s.ad_value(380)), 380);
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1950] != 0.0)) && (s.v[1958] != 0.0)) {
            s.store_add(174, 1001, 1000);
        }

        s.v[1959] = if (s.v[174] > 0.0) { 1.0 } else { 0.0 };

        if ((((!(s.v[1620] != 0.0)) && (s.v[1950] != 0.0)) && (s.v[1958] != 0.0)) && (s.v[1959] != 0.0)) {
            s.store_div_ad_lhs(1002, A::mul(s.ad_value(1000), s.ad_value(1001)), 174);
        }

        if ((((!(s.v[1620] != 0.0)) && (s.v[1950] != 0.0)) && (s.v[1958] != 0.0)) && (!(s.v[1959] != 0.0))) {
            s.store_scalar(1002, 0.0);
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1950] != 0.0)) && (!(s.v[1958] != 0.0))) {
            s.store_scalar(1002, 0.0);
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1950] != 0.0)) {
            s.store_scale(175, 271, (p.p1321 * 1.602176462e-19));
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1950] != 0.0)) {
            s.store_mul_ad_lhs(176, A::mul(A::scale(s.ad_value(1014), ((s.v[183] * p.p2) * 10000000000.0)), s.ad_value(613)), 613);
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1950] != 0.0)) {
            s.store_mul_ad_lhs(1009, A::mul(A::div(s.ad_value(175), s.ad_value(176)), s.ad_value(380)), 380);
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1950] != 0.0)) {
            s.copy_ad(177, 1009);
        }

        s.v[1960] = if (s.v[177] > 0.0) { 1.0 } else { 0.0 };

        if (((!(s.v[1620] != 0.0)) && (s.v[1950] != 0.0)) && (s.v[1960] != 0.0)) {
            s.copy_ad(1003, 1009);
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1950] != 0.0)) && (!(s.v[1960] != 0.0))) {
            s.store_scalar(1003, 0.0);
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1950] != 0.0)) {
            s.store_add_ad(616, A::mul(s.ad_value(1002), s.ad_value(1011)), A::mul(s.ad_value(1003), s.ad_value(1012)));
        }

        s.v[1961] = if (p.p1015 >= (s.v[184] / 2.0)) { 1.0 } else { 0.0 };

        if (((!(s.v[1620] != 0.0)) && (!(s.v[1950] != 0.0))) && (s.v[1961] != 0.0)) {
            s.store_scalar(606, 0.0);
        }

        if (((!(s.v[1620] != 0.0)) && (!(s.v[1950] != 0.0))) && (!(s.v[1961] != 0.0))) {
            s.store_scalar(606, p.p1015);
        }

        s.v[1962] = if (((p.p1012 > 0.0) || (p.p1013 > 0.0)) || (p.p1014 > 0.0)) { 1.0 } else { 0.0 };

        if (((!(s.v[1620] != 0.0)) && (!(s.v[1950] != 0.0))) && (s.v[1962] != 0.0)) {
            s.store_sub_from_scalar_ad(608, s.v[184], A::scale(s.ad_value(606), 2.0));
        }

        if (((!(s.v[1620] != 0.0)) && (!(s.v[1950] != 0.0))) && (s.v[1962] != 0.0)) {
            s.store_square(609, 608);
        }

        if (((!(s.v[1620] != 0.0)) && (!(s.v[1950] != 0.0))) && (s.v[1962] != 0.0)) {
            s.store_scale(167, 609, (10000000000.0 * s.v[199]));
        }

        if (((!(s.v[1620] != 0.0)) && (!(s.v[1950] != 0.0))) && (s.v[1962] != 0.0)) {
            s.store_scale_ad(611, A::mul(A::mul(A::mul(A::mul(A::scale(s.ad_value(253), (2.0 * s.v[199])), s.ad_value(271)), s.ad_value(400)), s.ad_value(853)), s.ad_value(834)), 6.241509744511525e18);
        }

        if (((!(s.v[1620] != 0.0)) && (!(s.v[1950] != 0.0))) && (s.v[1962] != 0.0)) {
            s.store_scale_ad(168, A::ln(A::max_with_scalar(A::div(A::add(s.ad_value(611), s.ad_value(613)), A::add(s.ad_value(612), s.ad_value(613))), 1e-38)), p.p1012);
        }

        if (((!(s.v[1620] != 0.0)) && (!(s.v[1950] != 0.0))) && (s.v[1962] != 0.0)) {
            s.store_scaled_sub(169, 611, 612, p.p1013);
        }

        if (((!(s.v[1620] != 0.0)) && (!(s.v[1950] != 0.0))) && (s.v[1962] != 0.0)) {
            s.store_scale_ad(170, A::sub(A::square(s.ad_value(611)), A::square(s.ad_value(612))), (0.5 * p.p1014));
        }

        if (((!(s.v[1620] != 0.0)) && (!(s.v[1950] != 0.0))) && (s.v[1962] != 0.0)) {
            s.store_scale(171, 609, (10000000000.0 * (s.v[183] * p.p2)));
        }

        if (((!(s.v[1620] != 0.0)) && (!(s.v[1950] != 0.0))) && (s.v[1962] != 0.0)) {
            s.store_add_ad(614, A::mul(A::div(s.ad_value(1004), s.ad_value(167)), A::add(A::add(s.ad_value(168), s.ad_value(169)), s.ad_value(170))), A::div(A::mul(A::mul(A::div(s.ad_value(1005), s.ad_value(171)), s.ad_value(610)), s.ad_value(1006)), s.ad_value(1007)));
        }

        if (((!(s.v[1620] != 0.0)) && (!(s.v[1950] != 0.0))) && (s.v[1962] != 0.0)) {
            s.store_mul_ad_lhs(172, A::mul(A::scale(s.ad_value(608), ((s.v[183] * p.p2) * 10000000000.0)), s.ad_value(613)), 613);
        }

        if (((!(s.v[1620] != 0.0)) && (!(s.v[1950] != 0.0))) && (s.v[1962] != 0.0)) {
            s.store_mul_ad_lhs(615, A::mul(A::div(s.ad_value(1008), s.ad_value(172)), s.ad_value(380)), 380);
        }

        if (((!(s.v[1620] != 0.0)) && (!(s.v[1950] != 0.0))) && (s.v[1962] != 0.0)) {
            s.store_add(173, 615, 614);
        }

        s.v[1963] = if (s.v[173] > 0.0) { 1.0 } else { 0.0 };

        if ((((!(s.v[1620] != 0.0)) && (!(s.v[1950] != 0.0))) && (s.v[1962] != 0.0)) && (s.v[1963] != 0.0)) {
            s.store_div_ad(616, A::div(A::mul(s.ad_value(614), s.ad_value(615)), s.ad_value(173)), A::offset(A::scale(A::powf(A::sub(s.ad_value(400), s.ad_value(320)), p.p1017), p.p1016), 1.0));
        }

        if ((((!(s.v[1620] != 0.0)) && (!(s.v[1950] != 0.0))) && (s.v[1962] != 0.0)) && (!(s.v[1963] != 0.0))) {
            s.store_scalar(616, 0.0);
        }

        if (((!(s.v[1620] != 0.0)) && (!(s.v[1950] != 0.0))) && (!(s.v[1962] != 0.0))) {
            s.store_scalar(616, 0.0);
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_scaled_div(167, 243, 607, 1.0 / (s.v[184]));
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_square(168, 167);
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_scale_ad(170, A::offset(A::scale(s.ad_value(168), (p.p1022 * s.v[184])), 1.0), p.p1019);
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_scale_ad(171, A::offset(A::scale(s.ad_value(168), (p.p1023 * s.v[184])), 1.0), p.p1020);
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_scale_ad(172, A::offset(A::scale(s.ad_value(168), (p.p1298 * s.v[184])), 1.0), p.p1297);
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_scale_ad(630, A::offset(A::scale(s.ad_value(168), (p.p1024 * s.v[184])), 1.0), p.p1021);
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_mul_ad_lhs(631, A::scale(s.ad_value(170), 3.0), 170);
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_offset_ad(631, A::scale(A::offset(s.ad_value(631), (-1.0)), if ((-s.v[184]) / p.p1296) > 80.0 { 5.540622384e34 * (1.0 + (((-s.v[184]) / p.p1296)) - 80.0) } else if ((-s.v[184]) / p.p1296) < -80.0 { 1.804851387e-35 } else { ((((-s.v[184]) / p.p1296)) as f64).exp() }), 1.0);
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_square(633, 172);
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_square(632, 171);
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_scalar(627, 0.0);
        }

        s.v[1964] = if (p.p39 == 0.0) { 1.0 } else { 0.0 };

        s.v[1965] = if (p.p39 == 1.0) { 1.0 } else { 0.0 };

        if ((!(s.v[1620] != 0.0)) && (s.v[1964] != 0.0)) {
            s.store_mul_ad_lhs(388, A::scale(s.ad_value(271), ((((-p.p2) * s.v[183]) * s.v[184]) * s.v[199])), 382);
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1964] != 0.0)) {
            s.store_mul_ad_lhs(389, A::scale(s.ad_value(271), ((((-p.p2) * s.v[183]) * s.v[184]) * s.v[199])), 385);
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1964] != 0.0)) {
            s.store_mul_ad_rhs(167, 337, A::abs(A::add(s.ad_value(388), s.ad_value(389))));
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1964] != 0.0)) {
            s.store_offset_ad(168, A::mul(s.ad_value(167), s.ad_value(457)), (s.v[184] * s.v[184]));
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1964] != 0.0)) {
            s.store_scaled_div(619, 167, 168, p.p1018);
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1964] != 0.0)) {
            s.store_mul(620, 621, 619);
        }

        if ((!(s.v[1620] != 0.0)) && ((s.v[1965] != 0.0) && (!(s.v[1964] != 0.0)))) {
            s.store_mul_ad_lhs(626, A::scale(s.ad_value(253), 2.0), 269);
        }

        if ((!(s.v[1620] != 0.0)) && ((s.v[1965] != 0.0) && (!(s.v[1964] != 0.0)))) {
            s.store_mul_ad_lhs(167, A::scale(A::mul(A::mul(s.ad_value(337), s.ad_value(345)), s.ad_value(363)), s.v[199]), 626);
        }

        if ((!(s.v[1620] != 0.0)) && ((s.v[1965] != 0.0) && (!(s.v[1964] != 0.0)))) {
            s.store_scaled_add(168, 400, 320, 0.5);
        }

        if ((!(s.v[1620] != 0.0)) && ((s.v[1965] != 0.0) && (!(s.v[1964] != 0.0)))) {
            s.store_offset(170, 168, 0.5);
        }

        if ((!(s.v[1620] != 0.0)) && ((s.v[1965] != 0.0) && (!(s.v[1964] != 0.0)))) {
            s.store_square(171, 170);
        }

        if ((!(s.v[1620] != 0.0)) && ((s.v[1965] != 0.0) && (!(s.v[1964] != 0.0)))) {
            s.store_mul(172, 171, 170);
        }

        if ((!(s.v[1620] != 0.0)) && ((s.v[1965] != 0.0) && (!(s.v[1964] != 0.0)))) {
            s.store_sub(173, 400, 320);
        }

        if ((!(s.v[1620] != 0.0)) && ((s.v[1965] != 0.0) && (!(s.v[1964] != 0.0)))) {
            s.store_square(174, 173);
        }

        if ((!(s.v[1620] != 0.0)) && ((s.v[1965] != 0.0) && (!(s.v[1964] != 0.0)))) {
            s.store_mul(175, 174, 173);
        }

        if ((!(s.v[1620] != 0.0)) && ((s.v[1965] != 0.0) && (!(s.v[1964] != 0.0)))) {
            s.store_mul_ad_lhs(176, A::offset(A::scale(s.ad_value(168), 6.0), 0.5), 174);
        }

        if ((!(s.v[1620] != 0.0)) && ((s.v[1965] != 0.0) && (!(s.v[1964] != 0.0)))) {
            s.store_scale(625, 345, s.v[184]);
        }

        if ((!(s.v[1620] != 0.0)) && ((s.v[1965] != 0.0) && (!(s.v[1964] != 0.0)))) {
            s.store_scale(177, 625, 1.0 / (s.v[184]));
        }

        if ((!(s.v[1620] != 0.0)) && ((s.v[1965] != 0.0) && (!(s.v[1964] != 0.0)))) {
            s.store_offset_ad(179, A::div(A::mul(s.ad_value(633), A::div(s.ad_value(315), s.ad_value(316))), A::offset(s.ad_value(243), p.p1299)), 1.0);
        }

        if ((!(s.v[1620] != 0.0)) && ((s.v[1965] != 0.0) && (!(s.v[1964] != 0.0)))) {
            s.store_offset_ad(179, A::scale(A::offset(s.ad_value(179), (-1.0)), if ((-s.v[184]) / p.p1296) > 80.0 { 5.540622384e34 * (1.0 + (((-s.v[184]) / p.p1296)) - 80.0) } else if ((-s.v[184]) / p.p1296) < -80.0 { 1.804851387e-35 } else { ((((-s.v[184]) / p.p1296)) as f64).exp() }), 1.0);
        }

        if ((!(s.v[1620] != 0.0)) && ((s.v[1965] != 0.0) && (!(s.v[1964] != 0.0)))) {
            s.store_scale_ad(179, A::add(s.ad_value(179), A::sqrt(A::offset(A::mul(s.ad_value(179), s.ad_value(179)), ((0.25 * 0.1) * 0.1)))), 0.5);
        }

        if ((!(s.v[1620] != 0.0)) && ((s.v[1965] != 0.0) && (!(s.v[1964] != 0.0)))) {
            s.store_mul_ad(624, A::div(A::scale(s.ad_value(167), (p.p2 * s.v[183])), s.ad_value(625)), A::add(A::mul(s.ad_value(168), s.ad_value(179)), A::div(A::mul(s.ad_value(174), s.ad_value(631)), A::scale(s.ad_value(170), 12.0))));
        }

        if ((!(s.v[1620] != 0.0)) && ((s.v[1965] != 0.0) && (!(s.v[1964] != 0.0)))) {
            let assign54870_ad_e89925: A = A::mul(A::scale(A::mul(A::mul(A::mul(s.ad_value(625), s.ad_value(177)), s.ad_value(177)), A::add(A::sub(A::div(s.ad_value(168), s.ad_value(171)), A::div(s.ad_value(176), A::mul(A::scale(s.ad_value(171), 60.0), s.ad_value(171)))), A::div(A::square(s.ad_value(174)), A::mul(A::scale(s.ad_value(171), 144.0), s.ad_value(172))))), (15.0 * 0.25)), s.ad_value(632));
            s.store_div_ad(622, assign54870_ad_e89925, A::scale(s.ad_value(167), ((p.p2 * s.v[183]) * 12.0)));
        }

        if ((!(s.v[1620] != 0.0)) && ((s.v[1965] != 0.0) && (!(s.v[1964] != 0.0)))) {
            s.store_scale_ad(623, A::mul(A::mul(s.ad_value(177), A::sub(A::div(s.ad_value(173), A::scale(s.ad_value(170), 12.0)), A::div(s.ad_value(175), A::scale(s.ad_value(172), 144.0)))), s.ad_value(630)), 2.531645569620253);
        }

        if ((!(s.v[1620] != 0.0)) && ((s.v[1965] != 0.0) && (!(s.v[1964] != 0.0)))) {
            s.store_sqrt_ad(628, A::mul(s.ad_value(621), s.ad_value(624)));
        }

        s.v[1966] = if (s.v[622] > 0.0) { 1.0 } else { 0.0 };

        if (((!(s.v[1620] != 0.0)) && ((s.v[1965] != 0.0) && (!(s.v[1964] != 0.0)))) && (s.v[1966] != 0.0)) {
            s.store_sqrt_ad(629, A::div(s.ad_value(621), s.ad_value(622)));
        }

        s.v[1967] = if (s.v[628] > 0.0) { 1.0 } else { 0.0 };

        if ((((!(s.v[1620] != 0.0)) && ((s.v[1965] != 0.0) && (!(s.v[1964] != 0.0)))) && (s.v[1966] != 0.0)) && (s.v[1967] != 0.0)) {
            s.store_div_ad_lhs(627, A::mul(s.ad_value(623), s.ad_value(629)), 628);
        }

    }

    pub(super) fn stamp_transient_block_39(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if ((((!(s.v[1620] != 0.0)) && ((s.v[1965] != 0.0) && (!(s.v[1964] != 0.0)))) && (s.v[1966] != 0.0)) && (!(s.v[1967] != 0.0))) {
            s.store_scalar(627, 0.0);
        }

        if (((!(s.v[1620] != 0.0)) && ((s.v[1965] != 0.0) && (!(s.v[1964] != 0.0)))) && (!(s.v[1966] != 0.0))) {
            s.store_scalar(629, 0.0);
        }

        if (((!(s.v[1620] != 0.0)) && ((s.v[1965] != 0.0) && (!(s.v[1964] != 0.0)))) && (!(s.v[1966] != 0.0))) {
            s.store_scalar(627, 0.0);
        }

        s.v[1968] = if (p.p37 != 0.0) { 1.0 } else { 0.0 };

        s.v[1969] = if (p.p38 != 0.0) { 1.0 } else { 0.0 };

        if (!(s.v[1620] != 0.0)) {
            s.copy_ad(217, 213);
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_scalar(418, 0.0);
        }

        s.v[1970] = if (p.p31 == 1.0) { 1.0 } else { 0.0 };

        if ((!(s.v[1620] != 0.0)) && (s.v[1970] != 0.0)) {
            s.store_offset(793, 793, p.p25);
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1970] != 0.0)) {
            s.store_mul(222, 221, 272);
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1970] != 0.0)) {
            s.store_mul(225, 224, 272);
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1970] != 0.0)) {
            s.store_mul(212, 793, 272);
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1970] != 0.0)) {
            s.store_sub(217, 222, 212);
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1970] != 0.0)) {
            s.store_ln_ad(432, A::max_with_scalar(A::div(s.ad_value(794), s.ad_value(182)), 1e-38));
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1970] != 0.0)) {
            s.store_scale_ad(433, A::sqrt(A::mul(A::scale(s.ad_value(794), ((2.0 * 1.602176462e-19) * s.v[180])), s.ad_value(272))), 1.0 / (s.v[199]));
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1970] != 0.0)) {
            s.store_div_from_scalar(295, 1.0, 433);
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1970] != 0.0)) {
            s.store_div_ad(406, A::scale(s.ad_value(704), ((2.0 * 1.602176462e-19) * s.v[180])), A::scale(s.ad_value(271), (s.v[199] * s.v[199])));
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1970] != 0.0)) {
            s.store_ad(418, &{
                if (s.v[704] > 0.0) {
                    A::div_from_scalar(1.0, s.ad_value(406))
                } else {
                    A::constant(0.0)
                }
            });
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1970] != 0.0)) {
            s.store_ad(403, &{
                if (s.v[704] > 0.0) {
                    A::div(s.ad_value(794), s.ad_value(704))
                } else {
                    A::constant(0.0)
                }
            });
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1970] != 0.0)) {
            s.store_offset(168, 403, 1.0);
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1970] != 0.0)) {
            s.store_div(404, 217, 168);
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1970] != 0.0)) {
            s.store_div(405, 433, 168);
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1970] != 0.0)) {
            s.store_sub_ad(168, A::scale(s.ad_value(404), 0.5), A::scale(A::offset(A::scale(s.ad_value(405), 0.7071067811865475), 1.0), 3.0));
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1970] != 0.0)) {
            s.store_add_ad_rhs(169, 168, A::sqrt(A::add(A::square(s.ad_value(168)), A::scale(s.ad_value(404), 6.0))));
        }

        s.v[1971] = if (s.v[404] < 0.0) { 1.0 } else { 0.0 };

        if (((!(s.v[1620] != 0.0)) && (s.v[1970] != 0.0)) && (s.v[1971] != 0.0)) {
            s.store_div_ad_lhs(170, A::sub(s.ad_value(404), s.ad_value(169)), 405);
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1970] != 0.0)) && (s.v[1971] != 0.0)) {
            s.store_neg_ad(254, A::ln(A::max_with_scalar(A::add(A::sub_from_scalar(1.0, s.ad_value(169)), A::square(s.ad_value(170))), 1e-38)));
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1970] != 0.0)) && (!(s.v[1971] != 0.0))) {
            s.store_limited_exp_ad(170, A::neg(s.ad_value(169)));
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1970] != 0.0)) && (!(s.v[1971] != 0.0))) {
            s.store_scale(168, 405, 0.5);
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1970] != 0.0)) && (!(s.v[1971] != 0.0))) {
            s.store_sub_ad_lhs(169, A::sqrt(A::add(A::add(A::offset(s.ad_value(404), (-1.0)), s.ad_value(170)), A::square(s.ad_value(168)))), 168);
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1970] != 0.0)) && (!(s.v[1971] != 0.0))) {
            s.store_sub_ad_lhs(254, A::offset(A::square(s.ad_value(169)), 1.0), 170);
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1970] != 0.0)) {
            s.store_scale_ad(175, A::add(A::offset(s.ad_value(254), 1.0), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(254), (-1.0)), A::offset(s.ad_value(254), (-1.0))), ((0.25 * 2.0) * 2.0)))), 0.5);
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1970] != 0.0)) {
            s.store_sqrt(259, 175);
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1970] != 0.0)) {
            s.store_div_ad_lhs(167, A::offset(A::div(s.ad_value(433), A::scale(s.ad_value(259), 2.0)), 1.0), 433);
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1970] != 0.0)) {
            s.store_sub_ad_lhs(168, A::sub(s.ad_value(254), A::scale(s.ad_value(432), 2.0)), 225);
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1970] != 0.0)) {
            s.store_sub_ad_rhs(169, 168, A::ln(A::max_with_scalar(A::mul(A::scale(s.ad_value(167), 4.0), s.ad_value(259)), 1e-38)));
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1970] != 0.0)) {
            s.store_scale_ad(175, A::sub(A::offset(s.ad_value(169), (-0.201491)), A::sqrt(A::offset(A::mul(s.ad_value(169), A::offset(s.ad_value(169), 0.402982)), 2.446562))), 0.5);
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1970] != 0.0)) {
            s.copy_ad(257, 259);
        }

        s.v[1972] = if (s.v[175] <= (-68.0)) { 1.0 } else { 0.0 };

        if (((!(s.v[1620] != 0.0)) && (s.v[1970] != 0.0)) && (s.v[1972] != 0.0)) {
            s.store_scalar(171, (-100.0));
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1970] != 0.0)) && (s.v[1972] != 0.0)) {
            s.store_scalar(172, 20.0);
        }

        s.v[1973] = if (s.v[175] < (s.v[171] - (0.5 * s.v[172]))) { 1.0 } else { 0.0 };

        if ((((!(s.v[1620] != 0.0)) && (s.v[1970] != 0.0)) && (s.v[1972] != 0.0)) && (s.v[1973] != 0.0)) {
            s.store_ad(170, &A::limited_exp(s.ad_value(171)));
        }

        s.v[1974] = if (s.v[175] > (s.v[171] + (0.5 * s.v[172]))) { 1.0 } else { 0.0 };

        if (((((!(s.v[1620] != 0.0)) && (s.v[1970] != 0.0)) && (s.v[1972] != 0.0)) && (!(s.v[1973] != 0.0))) && (s.v[1974] != 0.0)) {
            s.store_ad(170, &A::limited_exp(s.ad_value(175)));
        }

        if (((((!(s.v[1620] != 0.0)) && (s.v[1970] != 0.0)) && (s.v[1972] != 0.0)) && (!(s.v[1973] != 0.0))) && (!(s.v[1974] != 0.0))) {
            s.store_div_ad_lhs(169, A::sub(s.ad_value(175), s.ad_value(171)), 172);
        }

        if (((((!(s.v[1620] != 0.0)) && (s.v[1970] != 0.0)) && (s.v[1972] != 0.0)) && (!(s.v[1973] != 0.0))) && (!(s.v[1974] != 0.0))) {
            s.store_square(173, 169);
        }

        if (((((!(s.v[1620] != 0.0)) && (s.v[1970] != 0.0)) && (s.v[1972] != 0.0)) && (!(s.v[1973] != 0.0))) && (!(s.v[1974] != 0.0))) {
            s.store_limited_exp_ad(170, A::add(s.ad_value(171), A::mul(s.ad_value(172), A::add(A::offset(A::scale(s.ad_value(169), 0.5), (5.0 / 64.0)), A::mul(s.ad_value(173), A::sub_from_scalar((15.0 / 16.0), A::mul(s.ad_value(173), A::sub_from_scalar(1.25, s.ad_value(173)))))))));
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1970] != 0.0)) && (s.v[1972] != 0.0)) {
            s.store_mul_ad_rhs(400, 170, A::sub(A::sub(A::offset(s.ad_value(168), 1.0), s.ad_value(175)), A::ln(A::max_with_scalar(A::mul(A::scale(s.ad_value(167), 2.0), A::add(A::mul(A::scale(s.ad_value(170), 2.0), s.ad_value(167)), A::scale(s.ad_value(257), 2.0))), 1e-38))));
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1970] != 0.0)) && (!(s.v[1972] != 0.0))) {
            s.store_ad(170, &A::limited_exp(s.ad_value(175)));
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1970] != 0.0)) && (!(s.v[1972] != 0.0))) {
            s.store_div_from_scalar(258, 1.0, 257);
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1970] != 0.0)) && (!(s.v[1972] != 0.0))) {
            s.store_sub_ad_lhs(171, A::add(A::scale(s.ad_value(170), 2.0), A::ln(A::max_with_scalar(A::mul(A::mul(A::scale(s.ad_value(170), 2.0), s.ad_value(167)), A::add(A::mul(A::scale(s.ad_value(170), 2.0), s.ad_value(167)), A::scale(s.ad_value(257), 2.0))), 1e-38))), 168);
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1970] != 0.0)) && (!(s.v[1972] != 0.0))) {
            s.store_add_ad(172, A::offset(A::div_from_scalar(1.0, s.ad_value(170)), 2.0), A::div(A::add(s.ad_value(167), s.ad_value(258)), A::add(A::mul(s.ad_value(167), s.ad_value(170)), s.ad_value(257))));
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1970] != 0.0)) && (!(s.v[1972] != 0.0))) {
            s.store_sub_ad_rhs(170, 170, A::div(s.ad_value(171), s.ad_value(172)));
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1970] != 0.0)) && (!(s.v[1972] != 0.0))) {
            s.store_sub_ad_lhs(171, A::add(A::scale(s.ad_value(170), 2.0), A::ln(A::max_with_scalar(A::mul(A::mul(A::scale(s.ad_value(170), 2.0), s.ad_value(167)), A::add(A::mul(A::scale(s.ad_value(170), 2.0), s.ad_value(167)), A::scale(s.ad_value(257), 2.0))), 1e-38))), 168);
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1970] != 0.0)) && (!(s.v[1972] != 0.0))) {
            s.store_add_ad(172, A::offset(A::div_from_scalar(1.0, s.ad_value(170)), 2.0), A::div(A::add(s.ad_value(167), s.ad_value(258)), A::add(A::mul(s.ad_value(167), s.ad_value(170)), s.ad_value(257))));
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1970] != 0.0)) && (!(s.v[1972] != 0.0))) {
            s.store_mul_ad(173, A::div(A::add(s.ad_value(167), s.ad_value(258)), A::add(A::mul(s.ad_value(167), s.ad_value(170)), s.ad_value(257))), A::div(A::add(s.ad_value(167), s.ad_value(258)), A::add(A::mul(s.ad_value(167), s.ad_value(170)), s.ad_value(257))));
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1970] != 0.0)) && (!(s.v[1972] != 0.0))) {
            s.store_sub_ad_lhs(174, A::sub(A::neg(A::mul(A::div_from_scalar(1.0, s.ad_value(170)), A::div_from_scalar(1.0, s.ad_value(170)))), A::div_from_scalar(1.0, A::mul(A::mul(A::square(s.ad_value(257)), s.ad_value(257)), A::add(A::mul(s.ad_value(167), s.ad_value(170)), s.ad_value(257))))), 173);
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1970] != 0.0)) && (!(s.v[1972] != 0.0))) {
            s.store_sub_ad_rhs(400, 170, A::mul(A::div(s.ad_value(171), s.ad_value(172)), A::offset(A::div(A::mul(s.ad_value(171), s.ad_value(174)), A::mul(A::scale(s.ad_value(172), 2.0), s.ad_value(172))), 1.0)));
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1970] != 0.0)) {
            s.store_scale_ad(256, A::add(A::offset(s.ad_value(254), 1.0), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(254), (-1.0)), A::offset(s.ad_value(254), (-1.0))), ((0.25 * 2.0) * 2.0)))), 0.5);
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1970] != 0.0)) {
            s.store_sqrt(259, 256);
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1970] != 0.0)) {
            s.store_sub_ad_rhs(255, 254, A::scale(s.ad_value(400), 2.0));
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1970] != 0.0)) {
            s.store_scale_ad(167, A::add(A::offset(s.ad_value(255), 1.0), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(255), (-1.0)), A::offset(s.ad_value(255), (-1.0))), ((0.25 * 2.0) * 2.0)))), 0.5);
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1970] != 0.0)) {
            s.store_offset_ad(253, A::div(s.ad_value(433), A::add(s.ad_value(259), A::sqrt(s.ad_value(167)))), 1.0);
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1970] != 0.0)) {
            s.store_mul_ad_rhs(167, 271, A::sub(A::sub(s.ad_value(217), s.ad_value(254)), A::mul(A::scale(s.ad_value(400), 2.0), A::offset(s.ad_value(253), (-1.0)))));
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1970] != 0.0)) {
            s.store_scale_ad(247, A::add(s.ad_value(167), A::sqrt(A::offset(A::mul(s.ad_value(167), s.ad_value(167)), ((0.25 * 0.1) * 0.1)))), 0.5);
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1970] != 0.0)) {
            s.store_mul_ad_lhs(306, A::mul(A::scale(s.ad_value(253), 2.0), s.ad_value(271)), 400);
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1970] != 0.0)) {
            s.store_mul_ad_rhs(308, 335, A::add(s.ad_value(247), A::scale(s.ad_value(306), s.v[338])));
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1970] != 0.0)) {
            s.store_mul_ad(170, A::add(s.ad_value(750), A::mul(s.ad_value(760), s.ad_value(218))), A::pow(s.ad_value(308), s.ad_value(651)));
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1970] != 0.0)) {
            s.store_offset(171, 170, 1.0);
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1970] != 0.0)) {
            s.store_scale_ad(309, A::add(A::offset(s.ad_value(171), 1.0), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(171), (-1.0)), A::offset(s.ad_value(171), (-1.0))), ((0.25 * 0.0015) * 0.0015)))), 0.5);
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1970] != 0.0)) {
            s.store_div_ad(313, A::mul(A::div(s.ad_value(740), s.ad_value(309)), s.ad_value(271)), A::scale(s.ad_value(655), s.v[188]));
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1970] != 0.0)) {
            s.store_div_ad(307, A::mul(s.ad_value(313), A::add(A::square(s.ad_value(400)), s.ad_value(400))), A::offset(A::mul(s.ad_value(313), A::offset(s.ad_value(400), 1.0)), 1.0));
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1970] != 0.0)) {
            let assign55670_ad_e91153: A = A::sub(A::sub(s.ad_value(254), A::scale(s.ad_value(432), 2.0)), A::add(A::scale(s.ad_value(307), 2.0), A::ln(A::max_with_scalar(A::mul(A::mul(A::mul(A::scale(s.ad_value(307), 2.0), s.ad_value(253)), s.ad_value(295)), A::add(A::mul(A::mul(A::scale(s.ad_value(307), 2.0), s.ad_value(253)), s.ad_value(295)), A::div(s.ad_value(433), A::offset(s.ad_value(253), (-1.0))))), 1e-38))));
            s.store_ad(321, &assign55670_ad_e91153);
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1970] != 0.0)) {
            s.store_mul(322, 321, 271);
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1970] != 0.0)) {
            s.store_scale_ad(317, A::add(A::sub(s.ad_value(322), s.ad_value(224)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(322), s.ad_value(224)), A::sub(s.ad_value(322), s.ad_value(224))), ((0.25 * 0.001) * 0.001)))), 0.5);
        }

        s.v[1975] = if ((p.p1353 == 0.0) && (p.p1354 == 0.0)) { 1.0 } else { 0.0 };

        if (((!(s.v[1620] != 0.0)) && (s.v[1970] != 0.0)) && (s.v[1975] != 0.0)) {
            s.store_scalar(1020, p.p1348);
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1970] != 0.0)) && (!(s.v[1975] != 0.0))) {
            s.store_div_from_scalar_ad(168, s.v[184], A::offset(A::sqrt(A::mul(s.ad_value(782), s.ad_value(275))), s.v[184]));
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1970] != 0.0)) && (!(s.v[1975] != 0.0))) {
            s.store_offset_ad(1020, A::div(A::sub(A::scale(s.ad_value(168), p.p1353), A::mul(A::mul(A::scale(s.ad_value(168), p.p1354), s.ad_value(400)), s.ad_value(269))), A::offset(A::scale(s.ad_value(218), p.p1355), 1.0)), 1.0);
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1970] != 0.0)) && (!(s.v[1975] != 0.0))) {
            s.store_scale_ad(1020, A::add(A::offset(s.ad_value(1020), 0.1), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(1020), (-0.1)), A::offset(s.ad_value(1020), (-0.1))), ((0.25 * 0.0005) * 0.0005)))), 0.5);
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1970] != 0.0)) {
            s.store_div(317, 317, 1020);
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1970] != 0.0)) {
            s.store_pow_ad(174, A::offset(A::div(s.ad_value(226), s.ad_value(317)), 1e-6), A::div_from_scalar(1.0, s.ad_value(656)));
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1970] != 0.0)) {
            s.store_pow_ad(175, A::offset(s.ad_value(174), 1.0), A::neg(s.ad_value(656)));
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1970] != 0.0)) {
            s.store_mul(315, 226, 175);
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1970] != 0.0)) {
            s.store_mul_ad_lhs(318, A::add(s.ad_value(315), s.ad_value(224)), 272);
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1970] != 0.0)) {
            s.store_scale_ad(175, A::add(A::offset(s.ad_value(254), 1.0), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(254), (-1.0)), A::offset(s.ad_value(254), (-1.0))), ((0.25 * 2.0) * 2.0)))), 0.5);
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1970] != 0.0)) {
            s.store_sqrt(259, 175);
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1970] != 0.0)) {
            s.store_div_ad_lhs(167, A::offset(A::div(s.ad_value(433), A::scale(s.ad_value(259), 2.0)), 1.0), 433);
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1970] != 0.0)) {
            s.store_sub_ad_lhs(168, A::sub(s.ad_value(254), A::scale(s.ad_value(432), 2.0)), 318);
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1970] != 0.0)) {
            s.store_sub_ad_rhs(169, 168, A::ln(A::max_with_scalar(A::mul(A::scale(s.ad_value(167), 4.0), s.ad_value(259)), 1e-38)));
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1970] != 0.0)) {
            s.store_scale_ad(175, A::sub(A::offset(s.ad_value(169), (-0.201491)), A::sqrt(A::offset(A::mul(s.ad_value(169), A::offset(s.ad_value(169), 0.402982)), 2.446562))), 0.5);
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1970] != 0.0)) {
            s.copy_ad(257, 259);
        }

        s.v[1976] = if (s.v[175] <= (-68.0)) { 1.0 } else { 0.0 };

        if (((!(s.v[1620] != 0.0)) && (s.v[1970] != 0.0)) && (s.v[1976] != 0.0)) {
            s.store_scalar(171, (-100.0));
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1970] != 0.0)) && (s.v[1976] != 0.0)) {
            s.store_scalar(172, 20.0);
        }

        s.v[1977] = if (s.v[175] < (s.v[171] - (0.5 * s.v[172]))) { 1.0 } else { 0.0 };

        if ((((!(s.v[1620] != 0.0)) && (s.v[1970] != 0.0)) && (s.v[1976] != 0.0)) && (s.v[1977] != 0.0)) {
            s.store_ad(170, &A::limited_exp(s.ad_value(171)));
        }

        s.v[1978] = if (s.v[175] > (s.v[171] + (0.5 * s.v[172]))) { 1.0 } else { 0.0 };

        if (((((!(s.v[1620] != 0.0)) && (s.v[1970] != 0.0)) && (s.v[1976] != 0.0)) && (!(s.v[1977] != 0.0))) && (s.v[1978] != 0.0)) {
            s.store_ad(170, &A::limited_exp(s.ad_value(175)));
        }

        if (((((!(s.v[1620] != 0.0)) && (s.v[1970] != 0.0)) && (s.v[1976] != 0.0)) && (!(s.v[1977] != 0.0))) && (!(s.v[1978] != 0.0))) {
            s.store_div_ad_lhs(169, A::sub(s.ad_value(175), s.ad_value(171)), 172);
        }

        if (((((!(s.v[1620] != 0.0)) && (s.v[1970] != 0.0)) && (s.v[1976] != 0.0)) && (!(s.v[1977] != 0.0))) && (!(s.v[1978] != 0.0))) {
            s.store_square(173, 169);
        }

        if (((((!(s.v[1620] != 0.0)) && (s.v[1970] != 0.0)) && (s.v[1976] != 0.0)) && (!(s.v[1977] != 0.0))) && (!(s.v[1978] != 0.0))) {
            s.store_limited_exp_ad(170, A::add(s.ad_value(171), A::mul(s.ad_value(172), A::add(A::offset(A::scale(s.ad_value(169), 0.5), (5.0 / 64.0)), A::mul(s.ad_value(173), A::sub_from_scalar((15.0 / 16.0), A::mul(s.ad_value(173), A::sub_from_scalar(1.25, s.ad_value(173)))))))));
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1970] != 0.0)) && (s.v[1976] != 0.0)) {
            s.store_mul_ad_rhs(320, 170, A::sub(A::sub(A::offset(s.ad_value(168), 1.0), s.ad_value(175)), A::ln(A::max_with_scalar(A::mul(A::scale(s.ad_value(167), 2.0), A::add(A::mul(A::scale(s.ad_value(170), 2.0), s.ad_value(167)), A::scale(s.ad_value(257), 2.0))), 1e-38))));
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1970] != 0.0)) && (!(s.v[1976] != 0.0))) {
            s.store_ad(170, &A::limited_exp(s.ad_value(175)));
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1970] != 0.0)) && (!(s.v[1976] != 0.0))) {
            s.store_div_from_scalar(258, 1.0, 257);
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1970] != 0.0)) && (!(s.v[1976] != 0.0))) {
            s.store_sub_ad_lhs(171, A::add(A::scale(s.ad_value(170), 2.0), A::ln(A::max_with_scalar(A::mul(A::mul(A::scale(s.ad_value(170), 2.0), s.ad_value(167)), A::add(A::mul(A::scale(s.ad_value(170), 2.0), s.ad_value(167)), A::scale(s.ad_value(257), 2.0))), 1e-38))), 168);
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1970] != 0.0)) && (!(s.v[1976] != 0.0))) {
            s.store_add_ad(172, A::offset(A::div_from_scalar(1.0, s.ad_value(170)), 2.0), A::div(A::add(s.ad_value(167), s.ad_value(258)), A::add(A::mul(s.ad_value(167), s.ad_value(170)), s.ad_value(257))));
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1970] != 0.0)) && (!(s.v[1976] != 0.0))) {
            s.store_sub_ad_rhs(170, 170, A::div(s.ad_value(171), s.ad_value(172)));
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1970] != 0.0)) && (!(s.v[1976] != 0.0))) {
            s.store_sub_ad_lhs(171, A::add(A::scale(s.ad_value(170), 2.0), A::ln(A::max_with_scalar(A::mul(A::mul(A::scale(s.ad_value(170), 2.0), s.ad_value(167)), A::add(A::mul(A::scale(s.ad_value(170), 2.0), s.ad_value(167)), A::scale(s.ad_value(257), 2.0))), 1e-38))), 168);
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1970] != 0.0)) && (!(s.v[1976] != 0.0))) {
            s.store_add_ad(172, A::offset(A::div_from_scalar(1.0, s.ad_value(170)), 2.0), A::div(A::add(s.ad_value(167), s.ad_value(258)), A::add(A::mul(s.ad_value(167), s.ad_value(170)), s.ad_value(257))));
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1970] != 0.0)) && (!(s.v[1976] != 0.0))) {
            s.store_mul_ad(173, A::div(A::add(s.ad_value(167), s.ad_value(258)), A::add(A::mul(s.ad_value(167), s.ad_value(170)), s.ad_value(257))), A::div(A::add(s.ad_value(167), s.ad_value(258)), A::add(A::mul(s.ad_value(167), s.ad_value(170)), s.ad_value(257))));
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1970] != 0.0)) && (!(s.v[1976] != 0.0))) {
            s.store_sub_ad_lhs(174, A::sub(A::neg(A::mul(A::div_from_scalar(1.0, s.ad_value(170)), A::div_from_scalar(1.0, s.ad_value(170)))), A::div_from_scalar(1.0, A::mul(A::mul(A::square(s.ad_value(257)), s.ad_value(257)), A::add(A::mul(s.ad_value(167), s.ad_value(170)), s.ad_value(257))))), 173);
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1970] != 0.0)) && (!(s.v[1976] != 0.0))) {
            s.store_sub_ad_rhs(320, 170, A::mul(A::div(s.ad_value(171), s.ad_value(172)), A::offset(A::div(A::mul(s.ad_value(171), s.ad_value(174)), A::mul(A::scale(s.ad_value(172), 2.0), s.ad_value(172))), 1.0)));
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1970] != 0.0)) {
            s.store_offset_ad(255, A::sub(A::sub(s.ad_value(254), s.ad_value(400)), s.ad_value(320)), (-1.0));
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1970] != 0.0)) {
            s.store_scale_ad(167, A::add(A::offset(s.ad_value(255), 1.0), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(255), (-1.0)), A::offset(s.ad_value(255), (-1.0))), ((0.25 * 2.0) * 2.0)))), 0.5);
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1970] != 0.0)) {
            s.store_sqrt(169, 167);
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1970] != 0.0)) {
            s.store_add_ad(170, A::offset(s.ad_value(403), 1.0), A::div(s.ad_value(433), A::add(s.ad_value(259), s.ad_value(169))));
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1970] != 0.0)) {
            s.store_offset_ad(171, A::mul(A::mul(s.ad_value(403), s.ad_value(169)), s.ad_value(295)), 0.5);
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1970] != 0.0)) {
            s.store_sqrt_ad(172, A::add(A::square(s.ad_value(171)), A::mul(A::mul(s.ad_value(170), A::add(s.ad_value(400), s.ad_value(320))), s.ad_value(418))));
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1970] != 0.0)) {
            s.store_div_ad_rhs(253, 170, A::add(s.ad_value(171), s.ad_value(172)));
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1970] != 0.0)) {
            s.store_mul_ad_rhs(167, 271, A::sub(A::sub(s.ad_value(217), s.ad_value(254)), A::mul(A::scale(s.ad_value(400), 2.0), A::offset(s.ad_value(253), (-1.0)))));
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1970] != 0.0)) {
            s.store_scale_ad(247, A::add(s.ad_value(167), A::sqrt(A::offset(A::mul(s.ad_value(167), s.ad_value(167)), ((0.25 * 0.1) * 0.1)))), 0.5);
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1970] != 0.0)) {
            s.store_mul_ad_rhs(168, 271, A::sub(A::sub(s.ad_value(217), s.ad_value(254)), A::mul(A::scale(s.ad_value(320), 2.0), A::offset(s.ad_value(253), (-1.0)))));
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1970] != 0.0)) {
            s.store_scale_ad(248, A::add(s.ad_value(168), A::sqrt(A::offset(A::mul(s.ad_value(168), s.ad_value(168)), ((0.25 * 0.1) * 0.1)))), 0.5);
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1970] != 0.0)) {
            s.store_scaled_add(249, 247, 248, 0.5);
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1970] != 0.0)) {
            s.store_mul_ad(243, A::mul(s.ad_value(253), s.ad_value(271)), A::add(s.ad_value(400), s.ad_value(320)));
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1970] != 0.0)) {
            s.store_mul_ad_rhs(336, 335, A::add(s.ad_value(249), A::scale(s.ad_value(243), s.v[338])));
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1970] != 0.0)) {
            s.store_offset(168, 403, 1.0);
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1970] != 0.0)) {
            s.store_div_ad_lhs(404, A::add(s.ad_value(217), A::scale(s.ad_value(272), p.p139)), 168);
        }

    }

    pub(super) fn stamp_transient_block_40(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if ((!(s.v[1620] != 0.0)) && (s.v[1970] != 0.0)) {
            s.store_div(405, 433, 168);
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1970] != 0.0)) {
            s.store_sub_ad(168, A::scale(s.ad_value(404), 0.5), A::scale(A::offset(A::scale(s.ad_value(405), 0.7071067811865475), 1.0), 3.0));
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1970] != 0.0)) {
            s.store_add_ad_rhs(169, 168, A::sqrt(A::add(A::square(s.ad_value(168)), A::scale(s.ad_value(404), 6.0))));
        }

        s.v[1979] = if (s.v[404] < 0.0) { 1.0 } else { 0.0 };

        if (((!(s.v[1620] != 0.0)) && (s.v[1970] != 0.0)) && (s.v[1979] != 0.0)) {
            s.store_div_ad_lhs(170, A::sub(s.ad_value(404), s.ad_value(169)), 405);
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1970] != 0.0)) && (s.v[1979] != 0.0)) {
            s.store_neg_ad(254, A::ln(A::max_with_scalar(A::add(A::sub_from_scalar(1.0, s.ad_value(169)), A::square(s.ad_value(170))), 1e-38)));
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1970] != 0.0)) && (!(s.v[1979] != 0.0))) {
            s.store_limited_exp_ad(170, A::neg(s.ad_value(169)));
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1970] != 0.0)) && (!(s.v[1979] != 0.0))) {
            s.store_scale(168, 405, 0.5);
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1970] != 0.0)) && (!(s.v[1979] != 0.0))) {
            s.store_sub_ad_lhs(169, A::sqrt(A::add(A::add(A::offset(s.ad_value(404), (-1.0)), s.ad_value(170)), A::square(s.ad_value(168)))), 168);
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1970] != 0.0)) && (!(s.v[1979] != 0.0))) {
            s.store_sub_ad_lhs(254, A::offset(A::square(s.ad_value(169)), 1.0), 170);
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1970] != 0.0)) {
            s.store_mul_ad(170, A::add(s.ad_value(750), A::mul(s.ad_value(760), s.ad_value(218))), A::pow(s.ad_value(336), s.ad_value(651)));
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1970] != 0.0)) {
            s.store_offset(171, 170, 1.0);
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1970] != 0.0)) {
            s.store_scale_ad(339, A::add(A::offset(s.ad_value(171), 1.0), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(171), (-1.0)), A::offset(s.ad_value(171), (-1.0))), ((0.25 * 0.0015) * 0.0015)))), 0.5);
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1970] != 0.0)) {
            s.store_div_ad(314, A::mul(A::scale(A::div(s.ad_value(740), s.ad_value(339)), 2.0), s.ad_value(271)), A::scale(s.ad_value(655), s.v[188]));
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1970] != 0.0)) {
            s.store_sub(250, 400, 320);
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1970] != 0.0)) {
            s.store_mul_ad(168, A::scale(A::mul(s.ad_value(314), s.ad_value(250)), 2.0), A::mul(s.ad_value(314), s.ad_value(250)));
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1970] != 0.0)) {
            s.store_sqrt_ad(342, A::offset(s.ad_value(168), 1.0));
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1970] != 0.0)) {
            s.store_scaled_offset(343, 342, 1.0, 0.5);
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1970] != 0.0)) {
            s.store_div_ad(310, A::scale(s.ad_value(655), 2.0), A::div(s.ad_value(740), s.ad_value(339)));
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1970] != 0.0)) {
            s.store_scale(311, 310, s.v[188]);
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1970] != 0.0)) {
            s.store_add(358, 317, 311);
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1970] != 0.0)) {
            s.store_sub(355, 226, 315);
        }

        s.v[1980] = if (s.v[786] != 0.0) { 1.0 } else { 0.0 };

        if ((!(s.v[1620] != 0.0)) && (s.v[1980] != 0.0)) {
            s.store_offset_ad(364, A::mul(s.ad_value(786), A::ln(A::max_with_scalar(A::offset(A::div(A::div(s.ad_value(355), s.ad_value(786)), s.ad_value(358)), 1.0), 1e-38))), 1.0);
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1980] != 0.0))) {
            s.store_scalar(364, 1.0);
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_square(407, 364);
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_div_from_scalar(408, 1.0, 364);
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_div_from_scalar(409, 1.0, 407);
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_offset(410, 364, (-1.0));
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_sub(413, 217, 254);
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_sub(416, 400, 320);
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_mul_ad(417, A::sub(s.ad_value(400), s.ad_value(320)), A::sub(s.ad_value(400), s.ad_value(320)));
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_add_ad_rhs(411, 413, A::scale(s.ad_value(400), 2.0));
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_add_ad_rhs(412, 413, A::scale(s.ad_value(320), 2.0));
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_scale_ad(168, A::add(s.ad_value(411), A::sqrt(A::offset(A::mul(s.ad_value(411), s.ad_value(411)), ((0.25 * 0.5) * 0.5)))), 0.5);
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_scale_ad(169, A::add(s.ad_value(412), A::sqrt(A::offset(A::mul(s.ad_value(412), s.ad_value(412)), ((0.25 * 0.5) * 0.5)))), 0.5);
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_sqrt_ad(414, A::offset(A::mul(s.ad_value(168), s.ad_value(418)), 0.25));
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_sqrt_ad(415, A::offset(A::mul(s.ad_value(169), s.ad_value(418)), 0.25));
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_div_ad_rhs(168, 411, A::offset(A::scale(s.ad_value(414), 2.0), 1.0));
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_div_ad_rhs(169, 412, A::offset(A::scale(s.ad_value(415), 2.0), 1.0));
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_add(170, 414, 415);
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_scale_ad(171, A::div(s.ad_value(417), A::mul(A::square(s.ad_value(170)), s.ad_value(170))), 0.3333333333333333);
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_div_ad(172, A::mul(A::mul(s.ad_value(1020), s.ad_value(343)), s.ad_value(408)), A::add(A::offset(s.ad_value(400), 1.0), s.ad_value(320)));
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_mul_ad_lhs(173, A::scale(A::add(A::square(s.ad_value(170)), A::mul(s.ad_value(414), s.ad_value(415))), 0.8), 172);
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_add_ad_rhs(174, 173, A::scale(s.ad_value(418), 2.0));
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_mul_ad_lhs(175, A::scale(s.ad_value(417), 0.3333333333333333), 172);
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_div_ad(402, A::mul(s.ad_value(412), A::offset(A::scale(s.ad_value(415), 2.0), (-1.0))), A::offset(A::scale(s.ad_value(415), 2.0), 1.0));
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_add_ad_lhs(401, A::sub(s.ad_value(413), A::mul(A::scale(A::offset(s.ad_value(253), (-1.0)), 2.0), s.ad_value(320))), 402);
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_add_ad(381, A::mul(s.ad_value(408), A::add(A::add(s.ad_value(168), s.ad_value(169)), A::sub(A::mul(s.ad_value(171), s.ad_value(174)), A::mul(s.ad_value(253), A::add(A::add(s.ad_value(400), s.ad_value(320)), s.ad_value(175)))))), A::mul(s.ad_value(410), s.ad_value(401)));
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_add(176, 400, 320);
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_mul_ad_lhs(177, A::mul(s.ad_value(417), s.ad_value(172)), 172);
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_add_ad(386, A::mul(A::mul(s.ad_value(253), s.ad_value(408)), A::add(s.ad_value(176), A::mul(A::scale(s.ad_value(417), 0.3333333333333333), s.ad_value(172)))), A::mul(A::mul(A::scale(s.ad_value(253), 2.0), s.ad_value(410)), s.ad_value(320)));
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_mul_ad(383, A::mul(s.ad_value(253), s.ad_value(409)), A::sub(A::scale(s.ad_value(176), 0.5), A::mul(A::scale(s.ad_value(416), 0.16666666666666666), A::sub(A::sub_from_scalar(1.0, A::mul(s.ad_value(416), s.ad_value(172))), A::scale(s.ad_value(177), 0.2)))));
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_mul_ad_lhs(384, A::mul(s.ad_value(253), A::sub(s.ad_value(364), s.ad_value(408))), 320);
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_add(385, 383, 384);
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_sub(382, 386, 385);
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_scale_ad(246, A::add(A::mul(s.ad_value(271), s.ad_value(381)), A::sqrt(A::offset(A::mul(A::mul(s.ad_value(271), s.ad_value(381)), A::mul(s.ad_value(271), s.ad_value(381))), ((0.25 * 0.1) * 0.1)))), 0.5);
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_mul_ad_rhs(245, 271, A::add(s.ad_value(382), s.ad_value(385)));
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_scale_ad(167, A::add(s.ad_value(245), A::scale(s.ad_value(246), p.p231)), 1.0 / (p.p230));
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_offset_ad(168, A::powf(s.ad_value(167), (0.7 * p.p229)), 1.0);
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_div_from_scalar(427, (p.p228 * 1.9e-9), 168);
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_div_from_scalar_ad(428, (3.9 * 8.8541878128e-12), A::add(A::scale(s.ad_value(429), (3.9 * 1.0 / (p.p110))), A::scale(s.ad_value(427), 1.0 / (s.v[200]))));
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_mul_ad_lhs(387, A::mul(A::scale(A::div_from_scalar((8.8541878128e-12 * p.p110), s.ad_value(429)), (-(((p.p2 * s.v[187]) * s.v[188]) + p.p1379))), s.ad_value(271)), 381);
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_mul_ad_lhs(391, A::scale(s.ad_value(428), (((p.p2 * s.v[187]) * s.v[188]) + p.p1379)), 271);
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_mul_ad_lhs(389, A::neg(s.ad_value(391)), 385);
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_mul_ad_lhs(388, A::neg(s.ad_value(391)), 382);
        }

        s.v[1981] = if (p.p45 == 1.0) { 1.0 } else { 0.0 };

        if ((!(s.v[1620] != 0.0)) && (s.v[1981] != 0.0)) {
            s.store_scalar(795, (p.p140 + p.p25));
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1981] != 0.0)) {
            s.store_mul(231, 230, 272);
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1981] != 0.0)) {
            s.store_mul(233, 232, 272);
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1981] != 0.0)) {
            s.store_mul(212, 795, 272);
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1981] != 0.0)) {
            s.store_sub(434, 231, 212);
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1981] != 0.0)) {
            s.store_ln_ad(435, A::max_with_scalar(A::div_from_scalar(p.p141, s.ad_value(182)), 1e-38));
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1981] != 0.0)) {
            s.store_scale_ad(436, A::sqrt(A::scale(s.ad_value(272), (((2.0 * 1.602176462e-19) * s.v[180]) * p.p141))), 1.0 / (s.v[199]));
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1981] != 0.0)) {
            s.store_div_from_scalar(295, 1.0, 436);
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1981] != 0.0)) {
            s.store_scalar(418, 0.0);
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1981] != 0.0)) {
            s.store_scalar(403, 0.0);
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1981] != 0.0)) {
            s.store_offset(168, 403, 1.0);
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1981] != 0.0)) {
            s.store_div(404, 434, 168);
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1981] != 0.0)) {
            s.store_div(405, 436, 168);
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1981] != 0.0)) {
            s.store_sub_ad(168, A::scale(s.ad_value(404), 0.5), A::scale(A::offset(A::scale(s.ad_value(405), 0.7071067811865475), 1.0), 3.0));
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1981] != 0.0)) {
            s.store_add_ad_rhs(169, 168, A::sqrt(A::add(A::square(s.ad_value(168)), A::scale(s.ad_value(404), 6.0))));
        }

        s.v[1982] = if (s.v[404] < 0.0) { 1.0 } else { 0.0 };

        if (((!(s.v[1620] != 0.0)) && (s.v[1981] != 0.0)) && (s.v[1982] != 0.0)) {
            s.store_div_ad_lhs(170, A::sub(s.ad_value(404), s.ad_value(169)), 405);
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1981] != 0.0)) && (s.v[1982] != 0.0)) {
            s.store_neg_ad(254, A::ln(A::max_with_scalar(A::add(A::sub_from_scalar(1.0, s.ad_value(169)), A::square(s.ad_value(170))), 1e-38)));
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1981] != 0.0)) && (!(s.v[1982] != 0.0))) {
            s.store_limited_exp_ad(170, A::neg(s.ad_value(169)));
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1981] != 0.0)) && (!(s.v[1982] != 0.0))) {
            s.store_scale(168, 405, 0.5);
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1981] != 0.0)) && (!(s.v[1982] != 0.0))) {
            s.store_sub_ad_lhs(169, A::sqrt(A::add(A::add(A::offset(s.ad_value(404), (-1.0)), s.ad_value(170)), A::square(s.ad_value(168)))), 168);
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1981] != 0.0)) && (!(s.v[1982] != 0.0))) {
            s.store_sub_ad_lhs(254, A::offset(A::square(s.ad_value(169)), 1.0), 170);
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1981] != 0.0)) {
            s.store_scale_ad(175, A::add(A::offset(s.ad_value(254), 1.0), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(254), (-1.0)), A::offset(s.ad_value(254), (-1.0))), ((0.25 * 2.0) * 2.0)))), 0.5);
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1981] != 0.0)) {
            s.store_sqrt(259, 175);
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1981] != 0.0)) {
            s.store_div_ad_lhs(167, A::offset(A::div(s.ad_value(436), A::scale(s.ad_value(259), 2.0)), 1.0), 436);
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1981] != 0.0)) {
            s.store_sub_ad_lhs(168, A::sub(s.ad_value(254), A::scale(s.ad_value(435), 2.0)), 233);
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1981] != 0.0)) {
            s.store_sub_ad_rhs(169, 168, A::ln(A::max_with_scalar(A::mul(A::scale(s.ad_value(167), 4.0), s.ad_value(259)), 1e-38)));
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1981] != 0.0)) {
            s.store_scale_ad(175, A::sub(A::offset(s.ad_value(169), (-0.201491)), A::sqrt(A::offset(A::mul(s.ad_value(169), A::offset(s.ad_value(169), 0.402982)), 2.446562))), 0.5);
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1981] != 0.0)) {
            s.copy_ad(257, 259);
        }

        s.v[1983] = if (s.v[175] <= (-68.0)) { 1.0 } else { 0.0 };

        if (((!(s.v[1620] != 0.0)) && (s.v[1981] != 0.0)) && (s.v[1983] != 0.0)) {
            s.store_scalar(171, (-100.0));
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1981] != 0.0)) && (s.v[1983] != 0.0)) {
            s.store_scalar(172, 20.0);
        }

        s.v[1984] = if (s.v[175] < (s.v[171] - (0.5 * s.v[172]))) { 1.0 } else { 0.0 };

        if ((((!(s.v[1620] != 0.0)) && (s.v[1981] != 0.0)) && (s.v[1983] != 0.0)) && (s.v[1984] != 0.0)) {
            s.store_ad(170, &A::limited_exp(s.ad_value(171)));
        }

        s.v[1985] = if (s.v[175] > (s.v[171] + (0.5 * s.v[172]))) { 1.0 } else { 0.0 };

        if (((((!(s.v[1620] != 0.0)) && (s.v[1981] != 0.0)) && (s.v[1983] != 0.0)) && (!(s.v[1984] != 0.0))) && (s.v[1985] != 0.0)) {
            s.store_ad(170, &A::limited_exp(s.ad_value(175)));
        }

        if (((((!(s.v[1620] != 0.0)) && (s.v[1981] != 0.0)) && (s.v[1983] != 0.0)) && (!(s.v[1984] != 0.0))) && (!(s.v[1985] != 0.0))) {
            s.store_div_ad_lhs(169, A::sub(s.ad_value(175), s.ad_value(171)), 172);
        }

        if (((((!(s.v[1620] != 0.0)) && (s.v[1981] != 0.0)) && (s.v[1983] != 0.0)) && (!(s.v[1984] != 0.0))) && (!(s.v[1985] != 0.0))) {
            s.store_square(173, 169);
        }

        if (((((!(s.v[1620] != 0.0)) && (s.v[1981] != 0.0)) && (s.v[1983] != 0.0)) && (!(s.v[1984] != 0.0))) && (!(s.v[1985] != 0.0))) {
            s.store_limited_exp_ad(170, A::add(s.ad_value(171), A::mul(s.ad_value(172), A::add(A::offset(A::scale(s.ad_value(169), 0.5), (5.0 / 64.0)), A::mul(s.ad_value(173), A::sub_from_scalar((15.0 / 16.0), A::mul(s.ad_value(173), A::sub_from_scalar(1.25, s.ad_value(173)))))))));
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1981] != 0.0)) && (s.v[1983] != 0.0)) {
            s.store_mul_ad_rhs(400, 170, A::sub(A::sub(A::offset(s.ad_value(168), 1.0), s.ad_value(175)), A::ln(A::max_with_scalar(A::mul(A::scale(s.ad_value(167), 2.0), A::add(A::mul(A::scale(s.ad_value(170), 2.0), s.ad_value(167)), A::scale(s.ad_value(257), 2.0))), 1e-38))));
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1981] != 0.0)) && (!(s.v[1983] != 0.0))) {
            s.store_ad(170, &A::limited_exp(s.ad_value(175)));
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1981] != 0.0)) && (!(s.v[1983] != 0.0))) {
            s.store_div_from_scalar(258, 1.0, 257);
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1981] != 0.0)) && (!(s.v[1983] != 0.0))) {
            s.store_sub_ad_lhs(171, A::add(A::scale(s.ad_value(170), 2.0), A::ln(A::max_with_scalar(A::mul(A::mul(A::scale(s.ad_value(170), 2.0), s.ad_value(167)), A::add(A::mul(A::scale(s.ad_value(170), 2.0), s.ad_value(167)), A::scale(s.ad_value(257), 2.0))), 1e-38))), 168);
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1981] != 0.0)) && (!(s.v[1983] != 0.0))) {
            s.store_add_ad(172, A::offset(A::div_from_scalar(1.0, s.ad_value(170)), 2.0), A::div(A::add(s.ad_value(167), s.ad_value(258)), A::add(A::mul(s.ad_value(167), s.ad_value(170)), s.ad_value(257))));
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1981] != 0.0)) && (!(s.v[1983] != 0.0))) {
            s.store_sub_ad_rhs(170, 170, A::div(s.ad_value(171), s.ad_value(172)));
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1981] != 0.0)) && (!(s.v[1983] != 0.0))) {
            s.store_sub_ad_lhs(171, A::add(A::scale(s.ad_value(170), 2.0), A::ln(A::max_with_scalar(A::mul(A::mul(A::scale(s.ad_value(170), 2.0), s.ad_value(167)), A::add(A::mul(A::scale(s.ad_value(170), 2.0), s.ad_value(167)), A::scale(s.ad_value(257), 2.0))), 1e-38))), 168);
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1981] != 0.0)) && (!(s.v[1983] != 0.0))) {
            s.store_add_ad(172, A::offset(A::div_from_scalar(1.0, s.ad_value(170)), 2.0), A::div(A::add(s.ad_value(167), s.ad_value(258)), A::add(A::mul(s.ad_value(167), s.ad_value(170)), s.ad_value(257))));
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1981] != 0.0)) && (!(s.v[1983] != 0.0))) {
            s.store_mul_ad(173, A::div(A::add(s.ad_value(167), s.ad_value(258)), A::add(A::mul(s.ad_value(167), s.ad_value(170)), s.ad_value(257))), A::div(A::add(s.ad_value(167), s.ad_value(258)), A::add(A::mul(s.ad_value(167), s.ad_value(170)), s.ad_value(257))));
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1981] != 0.0)) && (!(s.v[1983] != 0.0))) {
            s.store_sub_ad_lhs(174, A::sub(A::neg(A::mul(A::div_from_scalar(1.0, s.ad_value(170)), A::div_from_scalar(1.0, s.ad_value(170)))), A::div_from_scalar(1.0, A::mul(A::mul(A::square(s.ad_value(257)), s.ad_value(257)), A::add(A::mul(s.ad_value(167), s.ad_value(170)), s.ad_value(257))))), 173);
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1981] != 0.0)) && (!(s.v[1983] != 0.0))) {
            s.store_sub_ad_rhs(400, 170, A::mul(A::div(s.ad_value(171), s.ad_value(172)), A::offset(A::div(A::mul(s.ad_value(171), s.ad_value(174)), A::mul(A::scale(s.ad_value(172), 2.0), s.ad_value(172))), 1.0)));
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1981] != 0.0)) {
            s.store_scale_ad(256, A::add(A::offset(s.ad_value(254), 1.0), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(254), (-1.0)), A::offset(s.ad_value(254), (-1.0))), ((0.25 * 2.0) * 2.0)))), 0.5);
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1981] != 0.0)) {
            s.store_sqrt(259, 256);
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1981] != 0.0)) {
            s.store_sub_ad_rhs(255, 254, A::scale(s.ad_value(400), 2.0));
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1981] != 0.0)) {
            s.store_scale_ad(167, A::add(A::offset(s.ad_value(255), 1.0), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(255), (-1.0)), A::offset(s.ad_value(255), (-1.0))), ((0.25 * 2.0) * 2.0)))), 0.5);
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1981] != 0.0)) {
            s.store_offset_ad(253, A::div(s.ad_value(436), A::add(s.ad_value(259), A::sqrt(s.ad_value(167)))), 1.0);
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1981] != 0.0)) {
            s.store_mul_ad_rhs(167, 271, A::sub(A::sub(s.ad_value(434), s.ad_value(254)), A::mul(A::scale(s.ad_value(400), 2.0), A::offset(s.ad_value(253), (-1.0)))));
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1981] != 0.0)) {
            s.store_scale_ad(247, A::add(s.ad_value(167), A::sqrt(A::offset(A::mul(s.ad_value(167), s.ad_value(167)), ((0.25 * 0.1) * 0.1)))), 0.5);
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1981] != 0.0)) {
            s.store_mul_ad_lhs(306, A::mul(A::scale(s.ad_value(253), 2.0), s.ad_value(271)), 400);
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1981] != 0.0)) {
            s.store_mul_ad_rhs(308, 335, A::add(s.ad_value(247), A::scale(s.ad_value(306), s.v[338])));
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1981] != 0.0)) {
            s.store_mul_ad(170, A::add(s.ad_value(750), A::mul(s.ad_value(760), s.ad_value(241))), A::pow(s.ad_value(308), s.ad_value(651)));
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1981] != 0.0)) {
            s.store_offset(171, 170, 1.0);
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1981] != 0.0)) {
            s.store_scale_ad(309, A::add(A::offset(s.ad_value(171), 1.0), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(171), (-1.0)), A::offset(s.ad_value(171), (-1.0))), ((0.25 * 0.0015) * 0.0015)))), 0.5);
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1981] != 0.0)) {
            s.store_div_ad(313, A::mul(A::div(s.ad_value(740), s.ad_value(309)), s.ad_value(271)), A::scale(s.ad_value(655), s.v[188]));
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1981] != 0.0)) {
            s.store_div_ad(307, A::mul(s.ad_value(313), A::add(A::square(s.ad_value(400)), s.ad_value(400))), A::offset(A::mul(s.ad_value(313), A::offset(s.ad_value(400), 1.0)), 1.0));
        }

    }

    pub(super) fn stamp_transient_block_41(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if ((!(s.v[1620] != 0.0)) && (s.v[1981] != 0.0)) {
            let assign57550_ad_e93994: A = A::sub(A::sub(s.ad_value(254), A::scale(s.ad_value(435), 2.0)), A::add(A::scale(s.ad_value(307), 2.0), A::ln(A::max_with_scalar(A::mul(A::mul(A::mul(A::scale(s.ad_value(307), 2.0), s.ad_value(253)), s.ad_value(295)), A::add(A::mul(A::mul(A::scale(s.ad_value(307), 2.0), s.ad_value(253)), s.ad_value(295)), A::div(s.ad_value(436), A::offset(s.ad_value(253), (-1.0))))), 1e-38))));
            s.store_ad(321, &assign57550_ad_e93994);
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1981] != 0.0)) {
            s.store_mul(322, 321, 271);
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1981] != 0.0)) {
            s.store_scale_ad(317, A::add(A::sub(s.ad_value(322), s.ad_value(232)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(322), s.ad_value(232)), A::sub(s.ad_value(322), s.ad_value(232))), ((0.25 * 0.001) * 0.001)))), 0.5);
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1981] != 0.0)) {
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1981] != 0.0)) {
            s.store_pow_ad(174, A::offset(A::div(s.ad_value(226), s.ad_value(317)), 1e-6), A::div_from_scalar(1.0, s.ad_value(656)));
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1981] != 0.0)) {
            s.store_pow_ad(175, A::offset(s.ad_value(174), 1.0), A::neg(s.ad_value(656)));
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1981] != 0.0)) {
            s.store_mul(315, 226, 175);
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1981] != 0.0)) {
            s.store_mul_ad_lhs(318, A::add(s.ad_value(315), s.ad_value(232)), 272);
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1981] != 0.0)) {
            s.store_scale_ad(175, A::add(A::offset(s.ad_value(254), 1.0), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(254), (-1.0)), A::offset(s.ad_value(254), (-1.0))), ((0.25 * 2.0) * 2.0)))), 0.5);
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1981] != 0.0)) {
            s.store_sqrt(259, 175);
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1981] != 0.0)) {
            s.store_div_ad_lhs(167, A::offset(A::div(s.ad_value(436), A::scale(s.ad_value(259), 2.0)), 1.0), 436);
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1981] != 0.0)) {
            s.store_sub_ad_lhs(168, A::sub(s.ad_value(254), A::scale(s.ad_value(435), 2.0)), 318);
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1981] != 0.0)) {
            s.store_sub_ad_rhs(169, 168, A::ln(A::max_with_scalar(A::mul(A::scale(s.ad_value(167), 4.0), s.ad_value(259)), 1e-38)));
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1981] != 0.0)) {
            s.store_scale_ad(175, A::sub(A::offset(s.ad_value(169), (-0.201491)), A::sqrt(A::offset(A::mul(s.ad_value(169), A::offset(s.ad_value(169), 0.402982)), 2.446562))), 0.5);
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1981] != 0.0)) {
            s.copy_ad(257, 259);
        }

        s.v[1986] = if (s.v[175] <= (-68.0)) { 1.0 } else { 0.0 };

        if (((!(s.v[1620] != 0.0)) && (s.v[1981] != 0.0)) && (s.v[1986] != 0.0)) {
            s.store_scalar(171, (-100.0));
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1981] != 0.0)) && (s.v[1986] != 0.0)) {
            s.store_scalar(172, 20.0);
        }

        s.v[1987] = if (s.v[175] < (s.v[171] - (0.5 * s.v[172]))) { 1.0 } else { 0.0 };

        if ((((!(s.v[1620] != 0.0)) && (s.v[1981] != 0.0)) && (s.v[1986] != 0.0)) && (s.v[1987] != 0.0)) {
            s.store_ad(170, &A::limited_exp(s.ad_value(171)));
        }

        s.v[1988] = if (s.v[175] > (s.v[171] + (0.5 * s.v[172]))) { 1.0 } else { 0.0 };

        if (((((!(s.v[1620] != 0.0)) && (s.v[1981] != 0.0)) && (s.v[1986] != 0.0)) && (!(s.v[1987] != 0.0))) && (s.v[1988] != 0.0)) {
            s.store_ad(170, &A::limited_exp(s.ad_value(175)));
        }

        if (((((!(s.v[1620] != 0.0)) && (s.v[1981] != 0.0)) && (s.v[1986] != 0.0)) && (!(s.v[1987] != 0.0))) && (!(s.v[1988] != 0.0))) {
            s.store_div_ad_lhs(169, A::sub(s.ad_value(175), s.ad_value(171)), 172);
        }

        if (((((!(s.v[1620] != 0.0)) && (s.v[1981] != 0.0)) && (s.v[1986] != 0.0)) && (!(s.v[1987] != 0.0))) && (!(s.v[1988] != 0.0))) {
            s.store_square(173, 169);
        }

        if (((((!(s.v[1620] != 0.0)) && (s.v[1981] != 0.0)) && (s.v[1986] != 0.0)) && (!(s.v[1987] != 0.0))) && (!(s.v[1988] != 0.0))) {
            s.store_limited_exp_ad(170, A::add(s.ad_value(171), A::mul(s.ad_value(172), A::add(A::offset(A::scale(s.ad_value(169), 0.5), (5.0 / 64.0)), A::mul(s.ad_value(173), A::sub_from_scalar((15.0 / 16.0), A::mul(s.ad_value(173), A::sub_from_scalar(1.25, s.ad_value(173)))))))));
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1981] != 0.0)) && (s.v[1986] != 0.0)) {
            s.store_mul_ad_rhs(320, 170, A::sub(A::sub(A::offset(s.ad_value(168), 1.0), s.ad_value(175)), A::ln(A::max_with_scalar(A::mul(A::scale(s.ad_value(167), 2.0), A::add(A::mul(A::scale(s.ad_value(170), 2.0), s.ad_value(167)), A::scale(s.ad_value(257), 2.0))), 1e-38))));
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1981] != 0.0)) && (!(s.v[1986] != 0.0))) {
            s.store_ad(170, &A::limited_exp(s.ad_value(175)));
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1981] != 0.0)) && (!(s.v[1986] != 0.0))) {
            s.store_div_from_scalar(258, 1.0, 257);
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1981] != 0.0)) && (!(s.v[1986] != 0.0))) {
            s.store_sub_ad_lhs(171, A::add(A::scale(s.ad_value(170), 2.0), A::ln(A::max_with_scalar(A::mul(A::mul(A::scale(s.ad_value(170), 2.0), s.ad_value(167)), A::add(A::mul(A::scale(s.ad_value(170), 2.0), s.ad_value(167)), A::scale(s.ad_value(257), 2.0))), 1e-38))), 168);
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1981] != 0.0)) && (!(s.v[1986] != 0.0))) {
            s.store_add_ad(172, A::offset(A::div_from_scalar(1.0, s.ad_value(170)), 2.0), A::div(A::add(s.ad_value(167), s.ad_value(258)), A::add(A::mul(s.ad_value(167), s.ad_value(170)), s.ad_value(257))));
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1981] != 0.0)) && (!(s.v[1986] != 0.0))) {
            s.store_sub_ad_rhs(170, 170, A::div(s.ad_value(171), s.ad_value(172)));
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1981] != 0.0)) && (!(s.v[1986] != 0.0))) {
            s.store_sub_ad_lhs(171, A::add(A::scale(s.ad_value(170), 2.0), A::ln(A::max_with_scalar(A::mul(A::mul(A::scale(s.ad_value(170), 2.0), s.ad_value(167)), A::add(A::mul(A::scale(s.ad_value(170), 2.0), s.ad_value(167)), A::scale(s.ad_value(257), 2.0))), 1e-38))), 168);
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1981] != 0.0)) && (!(s.v[1986] != 0.0))) {
            s.store_add_ad(172, A::offset(A::div_from_scalar(1.0, s.ad_value(170)), 2.0), A::div(A::add(s.ad_value(167), s.ad_value(258)), A::add(A::mul(s.ad_value(167), s.ad_value(170)), s.ad_value(257))));
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1981] != 0.0)) && (!(s.v[1986] != 0.0))) {
            s.store_mul_ad(173, A::div(A::add(s.ad_value(167), s.ad_value(258)), A::add(A::mul(s.ad_value(167), s.ad_value(170)), s.ad_value(257))), A::div(A::add(s.ad_value(167), s.ad_value(258)), A::add(A::mul(s.ad_value(167), s.ad_value(170)), s.ad_value(257))));
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1981] != 0.0)) && (!(s.v[1986] != 0.0))) {
            s.store_sub_ad_lhs(174, A::sub(A::neg(A::mul(A::div_from_scalar(1.0, s.ad_value(170)), A::div_from_scalar(1.0, s.ad_value(170)))), A::div_from_scalar(1.0, A::mul(A::mul(A::square(s.ad_value(257)), s.ad_value(257)), A::add(A::mul(s.ad_value(167), s.ad_value(170)), s.ad_value(257))))), 173);
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1981] != 0.0)) && (!(s.v[1986] != 0.0))) {
            s.store_sub_ad_rhs(320, 170, A::mul(A::div(s.ad_value(171), s.ad_value(172)), A::offset(A::div(A::mul(s.ad_value(171), s.ad_value(174)), A::mul(A::scale(s.ad_value(172), 2.0), s.ad_value(172))), 1.0)));
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1981] != 0.0)) {
            s.store_offset_ad(255, A::sub(A::sub(s.ad_value(254), s.ad_value(400)), s.ad_value(320)), (-1.0));
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1981] != 0.0)) {
            s.store_scale_ad(167, A::add(A::offset(s.ad_value(255), 1.0), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(255), (-1.0)), A::offset(s.ad_value(255), (-1.0))), ((0.25 * 2.0) * 2.0)))), 0.5);
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1981] != 0.0)) {
            s.store_sqrt(169, 167);
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1981] != 0.0)) {
            s.store_add_ad(170, A::offset(s.ad_value(403), 1.0), A::div(s.ad_value(436), A::add(s.ad_value(259), s.ad_value(169))));
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1981] != 0.0)) {
            s.store_offset_ad(171, A::mul(A::mul(s.ad_value(403), s.ad_value(169)), s.ad_value(295)), 0.5);
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1981] != 0.0)) {
            s.store_sqrt_ad(172, A::add(A::square(s.ad_value(171)), A::mul(A::mul(s.ad_value(170), A::add(s.ad_value(400), s.ad_value(320))), s.ad_value(418))));
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1981] != 0.0)) {
            s.store_div_ad_rhs(253, 170, A::add(s.ad_value(171), s.ad_value(172)));
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1981] != 0.0)) {
            s.store_scalar(364, 1.0);
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1981] != 0.0)) {
            s.store_square(407, 364);
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1981] != 0.0)) {
            s.store_div_from_scalar(408, 1.0, 364);
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1981] != 0.0)) {
            s.store_div_from_scalar(409, 1.0, 407);
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1981] != 0.0)) {
            s.store_offset(410, 364, (-1.0));
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1981] != 0.0)) {
            s.store_sub(413, 434, 254);
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1981] != 0.0)) {
            s.store_sub(416, 400, 320);
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1981] != 0.0)) {
            s.store_mul_ad(417, A::sub(s.ad_value(400), s.ad_value(320)), A::sub(s.ad_value(400), s.ad_value(320)));
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1981] != 0.0)) {
            s.store_add_ad_rhs(411, 413, A::scale(s.ad_value(400), 2.0));
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1981] != 0.0)) {
            s.store_add_ad_rhs(412, 413, A::scale(s.ad_value(320), 2.0));
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1981] != 0.0)) {
            s.store_scale_ad(168, A::add(s.ad_value(411), A::sqrt(A::offset(A::mul(s.ad_value(411), s.ad_value(411)), ((0.25 * 0.5) * 0.5)))), 0.5);
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1981] != 0.0)) {
            s.store_scale_ad(169, A::add(s.ad_value(412), A::sqrt(A::offset(A::mul(s.ad_value(412), s.ad_value(412)), ((0.25 * 0.5) * 0.5)))), 0.5);
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1981] != 0.0)) {
            s.store_sqrt_ad(414, A::offset(A::mul(s.ad_value(168), s.ad_value(418)), 0.25));
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1981] != 0.0)) {
            s.store_sqrt_ad(415, A::offset(A::mul(s.ad_value(169), s.ad_value(418)), 0.25));
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1981] != 0.0)) {
            s.store_div_ad_rhs(168, 411, A::offset(A::scale(s.ad_value(414), 2.0), 1.0));
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1981] != 0.0)) {
            s.store_div_ad_rhs(169, 412, A::offset(A::scale(s.ad_value(415), 2.0), 1.0));
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1981] != 0.0)) {
            s.store_add(170, 414, 415);
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1981] != 0.0)) {
            s.store_scale_ad(171, A::div(s.ad_value(417), A::mul(A::square(s.ad_value(170)), s.ad_value(170))), 0.3333333333333333);
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1981] != 0.0)) {
            s.store_scalar(343, 0.0);
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1981] != 0.0)) {
            s.store_div_ad(172, A::mul(s.ad_value(343), s.ad_value(408)), A::add(A::offset(s.ad_value(400), 1.0), s.ad_value(320)));
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1981] != 0.0)) {
            s.store_mul_ad_lhs(173, A::scale(A::add(A::square(s.ad_value(170)), A::mul(s.ad_value(414), s.ad_value(415))), 0.8), 172);
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1981] != 0.0)) {
            s.store_add_ad_rhs(174, 173, A::scale(s.ad_value(418), 2.0));
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1981] != 0.0)) {
            s.store_mul_ad_lhs(175, A::scale(s.ad_value(417), 0.3333333333333333), 172);
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1981] != 0.0)) {
            s.store_div_ad(402, A::mul(s.ad_value(412), A::offset(A::scale(s.ad_value(415), 2.0), (-1.0))), A::offset(A::scale(s.ad_value(415), 2.0), 1.0));
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1981] != 0.0)) {
            s.store_add_ad_lhs(401, A::sub(s.ad_value(413), A::mul(A::scale(A::offset(s.ad_value(253), (-1.0)), 2.0), s.ad_value(320))), 402);
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1981] != 0.0)) {
            s.store_add_ad(381, A::mul(s.ad_value(408), A::add(A::add(s.ad_value(168), s.ad_value(169)), A::sub(A::mul(s.ad_value(171), s.ad_value(174)), A::mul(s.ad_value(253), A::add(A::add(s.ad_value(400), s.ad_value(320)), s.ad_value(175)))))), A::mul(s.ad_value(410), s.ad_value(401)));
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1981] != 0.0)) {
            s.store_add(176, 400, 320);
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1981] != 0.0)) {
            s.store_mul_ad_lhs(177, A::mul(s.ad_value(417), s.ad_value(172)), 172);
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1981] != 0.0)) {
            s.store_add_ad(386, A::mul(A::mul(s.ad_value(253), s.ad_value(408)), A::add(s.ad_value(176), A::mul(A::scale(s.ad_value(417), 0.3333333333333333), s.ad_value(172)))), A::mul(A::mul(A::scale(s.ad_value(253), 2.0), s.ad_value(410)), s.ad_value(320)));
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1981] != 0.0)) {
            s.store_mul_ad(383, A::mul(s.ad_value(253), s.ad_value(409)), A::sub(A::scale(s.ad_value(176), 0.5), A::mul(A::scale(s.ad_value(416), 0.16666666666666666), A::sub(A::sub_from_scalar(1.0, A::mul(s.ad_value(416), s.ad_value(172))), A::scale(s.ad_value(177), 0.2)))));
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1981] != 0.0)) {
            s.store_mul_ad_lhs(384, A::mul(s.ad_value(253), A::sub(s.ad_value(364), s.ad_value(408))), 320);
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1981] != 0.0)) {
            s.store_add(385, 383, 384);
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1981] != 0.0)) {
            s.store_mul_ad_lhs(437, A::scale(A::div_from_scalar((8.8541878128e-12 * p.p110), s.ad_value(429)), p.p1380), 271);
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1981] != 0.0)) {
            s.store_mul(440, 437, 381);
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1981] != 0.0)) {
            s.store_mul(439, 437, 385);
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1981] != 0.0)) {
            s.store_mul(438, 437, 386);
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1981] != 0.0))) {
            s.store_scalar(440, 0.0);
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1981] != 0.0))) {
            s.store_scalar(439, 0.0);
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1981] != 0.0))) {
            s.store_scalar(438, 0.0);
        }

        if (!(s.v[1620] != 0.0)) {
            s.copy_ad(394, 389);
        }

        if (!(s.v[1620] != 0.0)) {
            s.copy_ad(395, 388);
        }

        if (!(s.v[1620] != 0.0)) {
            s.copy_ad(393, 387);
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_neg_ad(392, A::add(A::add(s.ad_value(393), s.ad_value(395)), s.ad_value(394)));
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_neg_ad(398, A::scale(s.ad_value(439), p.p45));
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_neg_ad(399, A::sub(A::scale(s.ad_value(438), p.p45), A::scale(s.ad_value(439), p.p45)));
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_neg_ad(397, A::scale(s.ad_value(440), p.p45));
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_neg_ad(396, A::add(A::add(s.ad_value(397), s.ad_value(399)), s.ad_value(398)));
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_neg_ad(389, A::sub(A::scale(s.ad_value(439), p.p45), s.ad_value(389)));
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_neg_ad(388, A::sub(A::sub(A::scale(s.ad_value(438), p.p45), s.ad_value(388)), A::scale(s.ad_value(439), p.p45)));
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_neg_ad(387, A::sub(A::scale(s.ad_value(440), p.p45), s.ad_value(387)));
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_neg_ad(390, A::add(A::add(s.ad_value(387), s.ad_value(388)), s.ad_value(389)));
        }

        s.v[1989] = if !(if self.param_given[867] { 1.0 } else { 0.0 } != 0.0) { 1.0 } else { 0.0 };

        if ((!(s.v[1620] != 0.0)) && (s.v[1989] != 0.0)) {
            s.store_scalar(788, ((((2.0 * p.p110) * 8.8541878128e-12) / 3.141592653589793) * ((((p.p871 * (1.0 + (4e-7 / p.p76)))).max(1e-38)) as f64).ln()));
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_offset(425, 788, p.p872);
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_offset(426, 788, p.p873);
        }

        s.v[1990] = if (p.p32 == 0.0) { 1.0 } else { 0.0 };

        if ((!(s.v[1620] != 0.0)) && (s.v[1990] != 0.0)) {
            s.store_mul_ad_lhs(423, A::scale(s.ad_value(425), ((-s.v[187]) * p.p2)), 431);
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1990] != 0.0)) {
            s.store_mul_ad_lhs(424, A::scale(s.ad_value(426), ((-s.v[187]) * p.p2)), 430);
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1990] != 0.0))) {
            s.store_sqrt_ad(167, A::offset(A::mul(A::offset(A::sub(s.ad_value(431), s.ad_value(219)), 0.02), A::offset(A::sub(s.ad_value(431), s.ad_value(219)), 0.02)), (4.0 * 0.02)));
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1990] != 0.0))) {
            s.store_scale_ad(419, A::sub(A::offset(A::sub(s.ad_value(431), s.ad_value(219)), 0.02), s.ad_value(167)), 0.5);
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1990] != 0.0))) {
            s.store_div_ad_rhs(173, 419, A::powf(A::offset(A::powf(A::scale(A::neg(s.ad_value(419)), 1.0 / (p.p893)), p.p894), 1.0), (1.0 / p.p894)));
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1990] != 0.0))) {
            s.store_sqrt_ad(168, A::sub_from_scalar(1.0, A::div(A::scale(s.ad_value(173), 4.0), s.ad_value(791))));
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1990] != 0.0))) {
            s.store_scale_ad(423, A::add(A::mul(s.ad_value(425), s.ad_value(431)), A::mul(s.ad_value(789), A::sub(A::sub(A::sub(s.ad_value(431), s.ad_value(219)), s.ad_value(419)), A::mul(A::scale(s.ad_value(791), 0.5), A::offset(s.ad_value(168), (-1.0)))))), ((-s.v[187]) * p.p2));
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1990] != 0.0))) {
            s.store_sqrt_ad(167, A::offset(A::mul(A::offset(A::sub(s.ad_value(430), s.ad_value(219)), 0.02), A::offset(A::sub(s.ad_value(430), s.ad_value(219)), 0.02)), (4.0 * 0.02)));
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1990] != 0.0))) {
            s.store_scale_ad(420, A::sub(A::offset(A::sub(s.ad_value(430), s.ad_value(219)), 0.02), s.ad_value(167)), 0.5);
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1990] != 0.0))) {
            s.store_div_ad_rhs(173, 420, A::powf(A::offset(A::powf(A::scale(A::neg(s.ad_value(420)), 1.0 / (p.p891)), p.p892), 1.0), (1.0 / p.p892)));
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1990] != 0.0))) {
            s.store_sqrt_ad(169, A::sub_from_scalar(1.0, A::div(A::scale(s.ad_value(173), 4.0), s.ad_value(792))));
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1990] != 0.0))) {
            s.store_scale_ad(424, A::add(A::mul(s.ad_value(426), s.ad_value(430)), A::mul(s.ad_value(790), A::sub(A::sub(A::sub(s.ad_value(430), s.ad_value(219)), s.ad_value(420)), A::mul(A::scale(s.ad_value(792), 0.5), A::offset(s.ad_value(169), (-1.0)))))), ((-s.v[187]) * p.p2));
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_ad(421, &A::mul(A::scale(A::neg(s.ad_value(379)), (p.p2 * (s.v[188] * p.p874))), A::voltage(ctx, &nodes, Some(9), Some(10))));
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_neg_ad(422, A::add(A::add(s.ad_value(423), s.ad_value(424)), s.ad_value(421)));
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_scalar(1035, ((s.v[261] - (2.0 * s.v[196])) - p.p1394));
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_offset(1036, 1035, (2.0 * p.p1393));
        }

        s.v[1991] = if (s.v[908] > 0.0) { 1.0 } else { 0.0 };

        if ((!(s.v[1620] != 0.0)) && (s.v[1991] != 0.0)) {
            s.store_ln_ad(167, A::max_with_scalar(A::div(s.ad_value(706), s.ad_value(908)), 1e-38));
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1991] != 0.0)) {
            s.store_mul_ad_lhs(215, A::mul(A::neg(s.ad_value(379)), s.ad_value(637)), 167);
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1991] != 0.0))) {
            s.store_ln_ad(167, A::max_with_scalar(A::div(A::div(A::mul(A::neg(s.ad_value(706)), s.ad_value(908)), s.ad_value(182)), s.ad_value(182)), 1e-38));
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1991] != 0.0))) {
            s.store_mul_ad_lhs(215, A::mul(A::neg(s.ad_value(379)), s.ad_value(637)), 167);
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_sub(1032, 235, 215);
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_scalar(1034, (3.453133e-11 / p.p75));
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_mul_ad(1037, A::mul(A::scale(s.ad_value(909), p.p1388), s.ad_value(1034)), A::offset(A::scale(s.ad_value(1036), ((s.v[187] / p.p1373) * p.p2)), p.p1382));
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_mul_ad_rhs(1038, 1037, A::sub(s.ad_value(1032), s.ad_value(1033)));
        }

        if (!(s.v[1620] != 0.0)) {
            s.copy_ad(1039, 1038);
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_scalar(167, (p.p1395 * ((((p.p871 * (1.0 + (p.p74 / p.p75)))).max(1e-38)) as f64).ln()));
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_scalar(168, (p.p19 - p.p1));
        }

        s.v[1992] = if (s.v[168] > 0.0) { 1.0 } else { 0.0 };

        if ((!(s.v[1620] != 0.0)) && (s.v[1992] != 0.0)) {
            s.store_mul(1040, 167, 168);
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1992] != 0.0))) {
            s.store_scalar(1040, 0.0);
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_scalar(168, (p.p20 - p.p1));
        }

        s.v[1993] = if (s.v[168] > 0.0) { 1.0 } else { 0.0 };

    }

    pub(super) fn stamp_transient_block_42(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if ((!(s.v[1620] != 0.0)) && (s.v[1993] != 0.0)) {
            s.store_mul(1041, 167, 168);
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1993] != 0.0))) {
            s.store_scalar(1041, 0.0);
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_scale(1042, 1034, p.p17);
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_scalar(1043, (p.p1396 * p.p17));
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_scale(1044, 1034, p.p18);
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_scalar(1045, (p.p1396 * p.p18));
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_mul_ad_lhs(177, A::neg(s.ad_value(379)), 236);
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_mul_ad_lhs(178, A::neg(s.ad_value(379)), 237);
        }

        s.v[1994] = if (p.p1396 != 0.0) { 1.0 } else { 0.0 };

        if ((!(s.v[1620] != 0.0)) && (s.v[1994] != 0.0)) {
            s.store_scaled_sub(168, 1044, 1045, ((-0.5) * 1.0 / (p.p1399)));
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1994] != 0.0)) {
            s.store_ln_ad(169, A::max_with_scalar(A::cosh(A::offset(A::scale(s.ad_value(178), (-p.p1399)), p.p1400)), 1e-38));
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1994] != 0.0)) {
            s.store_mul_ad_lhs(170, A::scale(A::add(s.ad_value(1044), s.ad_value(1045)), 0.5), 178);
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1994] != 0.0)) {
            s.store_add_ad_lhs(1047, A::mul(s.ad_value(168), s.ad_value(169)), 170);
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1994] != 0.0)) {
            s.store_scaled_sub(168, 1042, 1043, ((-0.5) * 1.0 / (p.p1397)));
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1994] != 0.0)) {
            s.store_ln_ad(169, A::max_with_scalar(A::cosh(A::offset(A::scale(s.ad_value(177), (-p.p1397)), p.p1398)), 1e-38));
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1994] != 0.0)) {
            s.store_mul_ad_lhs(170, A::scale(A::add(s.ad_value(1042), s.ad_value(1043)), 0.5), 177);
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1994] != 0.0)) {
            s.store_add_ad_lhs(1046, A::mul(s.ad_value(168), s.ad_value(169)), 170);
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1994] != 0.0))) {
            s.store_mul(1046, 1042, 177);
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[1994] != 0.0))) {
            s.store_mul(1047, 1044, 178);
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_add_ad_rhs(1046, 1046, A::mul(s.ad_value(1040), s.ad_value(177)));
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_add_ad_rhs(1047, 1047, A::mul(s.ad_value(1041), s.ad_value(178)));
        }

        s.v[1995] = if (p.p27 == 1.0) { 1.0 } else { 0.0 };

        if ((!(s.v[1620] != 0.0)) && (s.v[1995] != 0.0)) {
            s.store_ln_ad(951, A::max_with_scalar(A::div(s.ad_value(953), s.ad_value(182)), 1e-38));
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1995] != 0.0)) {
            s.store_max_with_scalar_ad(298, A::add(A::offset(A::mul(s.ad_value(271), s.ad_value(951)), 0.4), s.ad_value(729)), 0.4);
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1995] != 0.0)) {
            s.store_sqrt_ad(277, A::div_from_scalar((2.0 * s.v[180]), A::scale(s.ad_value(953), 1.602176462e-19)));
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1995] != 0.0)) {
            s.store_mul_ad_rhs(941, 835, A::scale(A::add(A::offset(A::mul(s.ad_value(847), A::offset(s.ad_value(639), (-1.0))), 1.0), A::sqrt(A::offset(A::mul(A::offset(A::mul(s.ad_value(847), A::offset(s.ad_value(639), (-1.0))), 1.0), A::offset(A::mul(s.ad_value(847), A::offset(s.ad_value(639), (-1.0))), 1.0)), ((4.0 * 0.001) * 0.001)))), 0.5));
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1995] != 0.0)) {
            s.store_mul_ad_rhs(940, 841, A::offset(A::mul(s.ad_value(848), A::offset(s.ad_value(639), (-1.0))), 1.0));
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1995] != 0.0)) {
            s.store_scale_ad(273, A::add(A::offset(A::sub(s.ad_value(298), s.ad_value(218)), 0.05), A::sqrt(A::offset(A::mul(A::offset(A::sub(s.ad_value(298), s.ad_value(218)), (-0.05)), A::offset(A::sub(s.ad_value(298), s.ad_value(218)), (-0.05))), ((0.25 * 0.1) * 0.1)))), 0.5);
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1995] != 0.0)) {
            s.store_sqrt(274, 273);
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1995] != 0.0)) {
            s.store_mul(275, 277, 274);
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1995] != 0.0)) {
            s.store_div_from_scalar(260, s.v[180], 275);
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1995] != 0.0)) {
            s.store_sub_ad(276, A::add(A::add(s.ad_value(836), s.ad_value(941)), A::mul(s.ad_value(838), s.ad_value(227))), A::mul(s.ad_value(840), s.ad_value(218)));
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1995] != 0.0)) {
            s.store_offset_scaled(168, 276, 1.0 / (s.v[199]), 1.0);
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1995] != 0.0)) {
            s.store_scale_ad(267, A::add(A::offset(s.ad_value(168), 1.0), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(168), (-1.0)), A::offset(s.ad_value(168), (-1.0))), ((0.25 * 0.05) * 0.05)))), 0.5);
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1995] != 0.0)) {
            s.store_mul(269, 267, 271);
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1995] != 0.0)) {
            s.store_div_from_scalar(270, 1.0, 269);
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1995] != 0.0)) {
            s.store_mul(222, 221, 270);
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1995] != 0.0)) {
            s.store_mul(225, 224, 270);
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1995] != 0.0)) {
            s.store_mul(212, 707, 270);
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1995] != 0.0)) {
            s.store_mul_ad_lhs(944, A::neg(A::add(s.ad_value(940), A::mul(s.ad_value(842), s.ad_value(218)))), 227);
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1995] != 0.0)) {
            s.store_mul_ad(293, A::add(A::add(s.ad_value(843), A::scale(s.ad_value(844), 1.0 / (s.v[184]))), A::mul(s.ad_value(845), s.ad_value(218))), A::offset(A::pow(s.ad_value(639), s.ad_value(846)), (-1.0)));
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1995] != 0.0)) {
            s.store_mul_ad_rhs(946, 300, A::offset(A::scale(s.ad_value(218), p.p1264), 1.0));
        }

        s.v[1996] = if (s.v[946] > 0.0) { 1.0 } else { 0.0 };

        if (((!(s.v[1620] != 0.0)) && (s.v[1995] != 0.0)) && (s.v[1996] != 0.0)) {
            s.store_div_from_scalar(167, (p.p1263 * s.v[184]), 946);
        }

        s.v[1997] = if (s.v[167] < 40.0) { 1.0 } else { 0.0 };

        if ((((!(s.v[1620] != 0.0)) && (s.v[1995] != 0.0)) && (s.v[1996] != 0.0)) && (s.v[1997] != 0.0)) {
            s.store_div_from_scalar_ad(943, (0.5 * p.p1262), A::offset(A::cosh(s.ad_value(167)), (-1.0)));
        }

        if ((((!(s.v[1620] != 0.0)) && (s.v[1995] != 0.0)) && (s.v[1996] != 0.0)) && (!(s.v[1997] != 0.0))) {
            s.store_scale_ad(943, A::limited_exp(A::neg(s.ad_value(167))), p.p1262);
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1995] != 0.0)) && (!(s.v[1996] != 0.0))) {
            s.store_scalar(943, 0.0);
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1995] != 0.0)) {
            s.store_mul_ad_rhs(945, 943, A::sub(s.ad_value(942), s.ad_value(298)));
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1995] != 0.0)) {
            s.store_add_ad_lhs(242, A::sub(A::add(A::offset(A::add(A::sub(s.ad_value(944), s.ad_value(293)), s.ad_value(945)), p.p1151), s.ad_value(956)), A::mul(s.ad_value(849), s.ad_value(218))), 932);
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1995] != 0.0)) {
            s.store_sub_ad(213, A::sub(s.ad_value(222), s.ad_value(212)), A::mul(s.ad_value(242), s.ad_value(270)));
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1995] != 0.0)) {
            s.store_scalar(947, (p.p1148 * (1.0 + (p.p1149 * ((s.v[184]) as f64).powf((-p.p1150))))));
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1995] != 0.0)) {
            s.store_scale_ad(954, A::sqrt(A::mul(A::scale(s.ad_value(953), ((2.0 * 1.602176462e-19) * s.v[180])), s.ad_value(270))), 1.0 / (s.v[199]));
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1995] != 0.0)) {
            s.store_mul_ad_rhs(954, 954, A::offset(s.ad_value(947), 1.0));
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1995] != 0.0)) {
            s.store_div(952, 951, 267);
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1995] != 0.0)) {
            s.store_scalar(168, 1.0);
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1995] != 0.0)) {
            s.store_div(404, 213, 168);
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1995] != 0.0)) {
            s.store_div(405, 954, 168);
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1995] != 0.0)) {
            s.store_sub_ad(168, A::scale(s.ad_value(404), 0.5), A::scale(A::offset(A::scale(s.ad_value(405), 0.7071067811865475), 1.0), 3.0));
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1995] != 0.0)) {
            s.store_add_ad_rhs(169, 168, A::sqrt(A::add(A::square(s.ad_value(168)), A::scale(s.ad_value(404), 6.0))));
        }

        s.v[1998] = if (s.v[404] < 0.0) { 1.0 } else { 0.0 };

        if (((!(s.v[1620] != 0.0)) && (s.v[1995] != 0.0)) && (s.v[1998] != 0.0)) {
            s.store_div_ad_lhs(170, A::sub(s.ad_value(404), s.ad_value(169)), 405);
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1995] != 0.0)) && (s.v[1998] != 0.0)) {
            s.store_neg_ad(254, A::ln(A::max_with_scalar(A::add(A::sub_from_scalar(1.0, s.ad_value(169)), A::square(s.ad_value(170))), 1e-38)));
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1995] != 0.0)) && (!(s.v[1998] != 0.0))) {
            s.store_limited_exp_ad(170, A::neg(s.ad_value(169)));
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1995] != 0.0)) && (!(s.v[1998] != 0.0))) {
            s.store_scale(168, 405, 0.5);
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1995] != 0.0)) && (!(s.v[1998] != 0.0))) {
            s.store_sub_ad_lhs(169, A::sqrt(A::add(A::add(A::offset(s.ad_value(404), (-1.0)), s.ad_value(170)), A::square(s.ad_value(168)))), 168);
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1995] != 0.0)) && (!(s.v[1998] != 0.0))) {
            s.store_sub_ad_lhs(254, A::offset(A::square(s.ad_value(169)), 1.0), 170);
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1995] != 0.0)) {
            s.store_scale_ad(175, A::add(A::offset(s.ad_value(254), 1.0), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(254), (-1.0)), A::offset(s.ad_value(254), (-1.0))), ((0.25 * 2.0) * 2.0)))), 0.5);
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1995] != 0.0)) {
            s.store_sqrt(259, 175);
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1995] != 0.0)) {
            s.store_div_ad_lhs(167, A::offset(A::div(s.ad_value(954), A::scale(s.ad_value(259), 2.0)), 1.0), 954);
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1995] != 0.0)) {
            s.store_sub_ad_lhs(168, A::sub(s.ad_value(254), A::scale(s.ad_value(952), 2.0)), 225);
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1995] != 0.0)) {
            s.store_sub_ad_rhs(169, 168, A::ln(A::max_with_scalar(A::mul(A::scale(s.ad_value(167), 4.0), s.ad_value(259)), 1e-38)));
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1995] != 0.0)) {
            s.store_scale_ad(175, A::sub(A::offset(s.ad_value(169), (-0.201491)), A::sqrt(A::offset(A::mul(s.ad_value(169), A::offset(s.ad_value(169), 0.402982)), 2.446562))), 0.5);
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1995] != 0.0)) {
            s.copy_ad(257, 259);
        }

        s.v[1999] = if (s.v[175] <= (-68.0)) { 1.0 } else { 0.0 };

        if (((!(s.v[1620] != 0.0)) && (s.v[1995] != 0.0)) && (s.v[1999] != 0.0)) {
            s.store_scalar(171, (-100.0));
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1995] != 0.0)) && (s.v[1999] != 0.0)) {
            s.store_scalar(172, 20.0);
        }

        s.v[2000] = if (s.v[175] < (s.v[171] - (0.5 * s.v[172]))) { 1.0 } else { 0.0 };

        if ((((!(s.v[1620] != 0.0)) && (s.v[1995] != 0.0)) && (s.v[1999] != 0.0)) && (s.v[2000] != 0.0)) {
            s.store_ad(170, &A::limited_exp(s.ad_value(171)));
        }

        s.v[2001] = if (s.v[175] > (s.v[171] + (0.5 * s.v[172]))) { 1.0 } else { 0.0 };

        if (((((!(s.v[1620] != 0.0)) && (s.v[1995] != 0.0)) && (s.v[1999] != 0.0)) && (!(s.v[2000] != 0.0))) && (s.v[2001] != 0.0)) {
            s.store_ad(170, &A::limited_exp(s.ad_value(175)));
        }

        if (((((!(s.v[1620] != 0.0)) && (s.v[1995] != 0.0)) && (s.v[1999] != 0.0)) && (!(s.v[2000] != 0.0))) && (!(s.v[2001] != 0.0))) {
            s.store_div_ad_lhs(169, A::sub(s.ad_value(175), s.ad_value(171)), 172);
        }

        if (((((!(s.v[1620] != 0.0)) && (s.v[1995] != 0.0)) && (s.v[1999] != 0.0)) && (!(s.v[2000] != 0.0))) && (!(s.v[2001] != 0.0))) {
            s.store_square(173, 169);
        }

        if (((((!(s.v[1620] != 0.0)) && (s.v[1995] != 0.0)) && (s.v[1999] != 0.0)) && (!(s.v[2000] != 0.0))) && (!(s.v[2001] != 0.0))) {
            s.store_limited_exp_ad(170, A::add(s.ad_value(171), A::mul(s.ad_value(172), A::add(A::offset(A::scale(s.ad_value(169), 0.5), (5.0 / 64.0)), A::mul(s.ad_value(173), A::sub_from_scalar((15.0 / 16.0), A::mul(s.ad_value(173), A::sub_from_scalar(1.25, s.ad_value(173)))))))));
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1995] != 0.0)) && (s.v[1999] != 0.0)) {
            s.store_mul_ad_rhs(961, 170, A::sub(A::sub(A::offset(s.ad_value(168), 1.0), s.ad_value(175)), A::ln(A::max_with_scalar(A::mul(A::scale(s.ad_value(167), 2.0), A::add(A::mul(A::scale(s.ad_value(170), 2.0), s.ad_value(167)), A::scale(s.ad_value(257), 2.0))), 1e-38))));
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1995] != 0.0)) && (!(s.v[1999] != 0.0))) {
            s.store_ad(170, &A::limited_exp(s.ad_value(175)));
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1995] != 0.0)) && (!(s.v[1999] != 0.0))) {
            s.store_div_from_scalar(258, 1.0, 257);
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1995] != 0.0)) && (!(s.v[1999] != 0.0))) {
            s.store_sub_ad_lhs(171, A::add(A::scale(s.ad_value(170), 2.0), A::ln(A::max_with_scalar(A::mul(A::mul(A::scale(s.ad_value(170), 2.0), s.ad_value(167)), A::add(A::mul(A::scale(s.ad_value(170), 2.0), s.ad_value(167)), A::scale(s.ad_value(257), 2.0))), 1e-38))), 168);
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1995] != 0.0)) && (!(s.v[1999] != 0.0))) {
            s.store_add_ad(172, A::offset(A::div_from_scalar(1.0, s.ad_value(170)), 2.0), A::div(A::add(s.ad_value(167), s.ad_value(258)), A::add(A::mul(s.ad_value(167), s.ad_value(170)), s.ad_value(257))));
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1995] != 0.0)) && (!(s.v[1999] != 0.0))) {
            s.store_sub_ad_rhs(170, 170, A::div(s.ad_value(171), s.ad_value(172)));
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1995] != 0.0)) && (!(s.v[1999] != 0.0))) {
            s.store_sub_ad_lhs(171, A::add(A::scale(s.ad_value(170), 2.0), A::ln(A::max_with_scalar(A::mul(A::mul(A::scale(s.ad_value(170), 2.0), s.ad_value(167)), A::add(A::mul(A::scale(s.ad_value(170), 2.0), s.ad_value(167)), A::scale(s.ad_value(257), 2.0))), 1e-38))), 168);
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1995] != 0.0)) && (!(s.v[1999] != 0.0))) {
            s.store_add_ad(172, A::offset(A::div_from_scalar(1.0, s.ad_value(170)), 2.0), A::div(A::add(s.ad_value(167), s.ad_value(258)), A::add(A::mul(s.ad_value(167), s.ad_value(170)), s.ad_value(257))));
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1995] != 0.0)) && (!(s.v[1999] != 0.0))) {
            s.store_mul_ad(173, A::div(A::add(s.ad_value(167), s.ad_value(258)), A::add(A::mul(s.ad_value(167), s.ad_value(170)), s.ad_value(257))), A::div(A::add(s.ad_value(167), s.ad_value(258)), A::add(A::mul(s.ad_value(167), s.ad_value(170)), s.ad_value(257))));
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1995] != 0.0)) && (!(s.v[1999] != 0.0))) {
            s.store_sub_ad_lhs(174, A::sub(A::neg(A::mul(A::div_from_scalar(1.0, s.ad_value(170)), A::div_from_scalar(1.0, s.ad_value(170)))), A::div_from_scalar(1.0, A::mul(A::mul(A::square(s.ad_value(257)), s.ad_value(257)), A::add(A::mul(s.ad_value(167), s.ad_value(170)), s.ad_value(257))))), 173);
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1995] != 0.0)) && (!(s.v[1999] != 0.0))) {
            s.store_sub_ad_rhs(961, 170, A::mul(A::div(s.ad_value(171), s.ad_value(172)), A::offset(A::div(A::mul(s.ad_value(171), s.ad_value(174)), A::mul(A::scale(s.ad_value(172), 2.0), s.ad_value(172))), 1.0)));
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1995] != 0.0)) {
            s.store_add_ad(948, A::mul(A::scale(s.ad_value(269), 2.0), s.ad_value(961)), A::scale(s.ad_value(269), 2.0));
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1995] != 0.0)) {
            s.copy_ad(949, 948);
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1995] != 0.0)) {
            s.store_add(949, 949, 224);
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1995] != 0.0)) {
            s.store_scale_ad(950, A::add(A::sub(s.ad_value(949), s.ad_value(224)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(949), s.ad_value(224)), A::sub(s.ad_value(949), s.ad_value(224))), ((0.25 * 0.001) * 0.001)))), 0.5);
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1995] != 0.0)) {
            s.store_pow_ad(174, A::offset(A::div(s.ad_value(226), s.ad_value(950)), 1e-6), A::div_from_scalar(1.0, s.ad_value(656)));
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1995] != 0.0)) {
            s.store_pow_ad(175, A::offset(s.ad_value(174), 1.0), A::neg(s.ad_value(656)));
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1995] != 0.0)) {
            s.store_mul(315, 226, 175);
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1995] != 0.0)) {
            s.store_mul_ad_lhs(318, A::add(s.ad_value(315), s.ad_value(224)), 270);
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1995] != 0.0)) {
            s.store_scale_ad(175, A::add(A::offset(s.ad_value(254), 1.0), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(254), (-1.0)), A::offset(s.ad_value(254), (-1.0))), ((0.25 * 2.0) * 2.0)))), 0.5);
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1995] != 0.0)) {
            s.store_sqrt(259, 175);
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1995] != 0.0)) {
            s.store_div_ad_lhs(167, A::offset(A::div(s.ad_value(954), A::scale(s.ad_value(259), 2.0)), 1.0), 954);
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1995] != 0.0)) {
            s.store_sub_ad_lhs(168, A::sub(s.ad_value(254), A::scale(s.ad_value(952), 2.0)), 318);
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1995] != 0.0)) {
            s.store_sub_ad_rhs(169, 168, A::ln(A::max_with_scalar(A::mul(A::scale(s.ad_value(167), 4.0), s.ad_value(259)), 1e-38)));
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1995] != 0.0)) {
            s.store_scale_ad(175, A::sub(A::offset(s.ad_value(169), (-0.201491)), A::sqrt(A::offset(A::mul(s.ad_value(169), A::offset(s.ad_value(169), 0.402982)), 2.446562))), 0.5);
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1995] != 0.0)) {
            s.copy_ad(257, 259);
        }

        s.v[2002] = if (s.v[175] <= (-68.0)) { 1.0 } else { 0.0 };

        if (((!(s.v[1620] != 0.0)) && (s.v[1995] != 0.0)) && (s.v[2002] != 0.0)) {
            s.store_scalar(171, (-100.0));
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1995] != 0.0)) && (s.v[2002] != 0.0)) {
            s.store_scalar(172, 20.0);
        }

        s.v[2003] = if (s.v[175] < (s.v[171] - (0.5 * s.v[172]))) { 1.0 } else { 0.0 };

        if ((((!(s.v[1620] != 0.0)) && (s.v[1995] != 0.0)) && (s.v[2002] != 0.0)) && (s.v[2003] != 0.0)) {
            s.store_ad(170, &A::limited_exp(s.ad_value(171)));
        }

        s.v[2004] = if (s.v[175] > (s.v[171] + (0.5 * s.v[172]))) { 1.0 } else { 0.0 };

        if (((((!(s.v[1620] != 0.0)) && (s.v[1995] != 0.0)) && (s.v[2002] != 0.0)) && (!(s.v[2003] != 0.0))) && (s.v[2004] != 0.0)) {
            s.store_ad(170, &A::limited_exp(s.ad_value(175)));
        }

        if (((((!(s.v[1620] != 0.0)) && (s.v[1995] != 0.0)) && (s.v[2002] != 0.0)) && (!(s.v[2003] != 0.0))) && (!(s.v[2004] != 0.0))) {
            s.store_div_ad_lhs(169, A::sub(s.ad_value(175), s.ad_value(171)), 172);
        }

        if (((((!(s.v[1620] != 0.0)) && (s.v[1995] != 0.0)) && (s.v[2002] != 0.0)) && (!(s.v[2003] != 0.0))) && (!(s.v[2004] != 0.0))) {
            s.store_square(173, 169);
        }

        if (((((!(s.v[1620] != 0.0)) && (s.v[1995] != 0.0)) && (s.v[2002] != 0.0)) && (!(s.v[2003] != 0.0))) && (!(s.v[2004] != 0.0))) {
            s.store_limited_exp_ad(170, A::add(s.ad_value(171), A::mul(s.ad_value(172), A::add(A::offset(A::scale(s.ad_value(169), 0.5), (5.0 / 64.0)), A::mul(s.ad_value(173), A::sub_from_scalar((15.0 / 16.0), A::mul(s.ad_value(173), A::sub_from_scalar(1.25, s.ad_value(173)))))))));
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1995] != 0.0)) && (s.v[2002] != 0.0)) {
            s.store_mul_ad_rhs(960, 170, A::sub(A::sub(A::offset(s.ad_value(168), 1.0), s.ad_value(175)), A::ln(A::max_with_scalar(A::mul(A::scale(s.ad_value(167), 2.0), A::add(A::mul(A::scale(s.ad_value(170), 2.0), s.ad_value(167)), A::scale(s.ad_value(257), 2.0))), 1e-38))));
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1995] != 0.0)) && (!(s.v[2002] != 0.0))) {
            s.store_ad(170, &A::limited_exp(s.ad_value(175)));
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1995] != 0.0)) && (!(s.v[2002] != 0.0))) {
            s.store_div_from_scalar(258, 1.0, 257);
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1995] != 0.0)) && (!(s.v[2002] != 0.0))) {
            s.store_sub_ad_lhs(171, A::add(A::scale(s.ad_value(170), 2.0), A::ln(A::max_with_scalar(A::mul(A::mul(A::scale(s.ad_value(170), 2.0), s.ad_value(167)), A::add(A::mul(A::scale(s.ad_value(170), 2.0), s.ad_value(167)), A::scale(s.ad_value(257), 2.0))), 1e-38))), 168);
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1995] != 0.0)) && (!(s.v[2002] != 0.0))) {
            s.store_add_ad(172, A::offset(A::div_from_scalar(1.0, s.ad_value(170)), 2.0), A::div(A::add(s.ad_value(167), s.ad_value(258)), A::add(A::mul(s.ad_value(167), s.ad_value(170)), s.ad_value(257))));
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1995] != 0.0)) && (!(s.v[2002] != 0.0))) {
            s.store_sub_ad_rhs(170, 170, A::div(s.ad_value(171), s.ad_value(172)));
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1995] != 0.0)) && (!(s.v[2002] != 0.0))) {
            s.store_sub_ad_lhs(171, A::add(A::scale(s.ad_value(170), 2.0), A::ln(A::max_with_scalar(A::mul(A::mul(A::scale(s.ad_value(170), 2.0), s.ad_value(167)), A::add(A::mul(A::scale(s.ad_value(170), 2.0), s.ad_value(167)), A::scale(s.ad_value(257), 2.0))), 1e-38))), 168);
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1995] != 0.0)) && (!(s.v[2002] != 0.0))) {
            s.store_add_ad(172, A::offset(A::div_from_scalar(1.0, s.ad_value(170)), 2.0), A::div(A::add(s.ad_value(167), s.ad_value(258)), A::add(A::mul(s.ad_value(167), s.ad_value(170)), s.ad_value(257))));
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1995] != 0.0)) && (!(s.v[2002] != 0.0))) {
            s.store_mul_ad(173, A::div(A::add(s.ad_value(167), s.ad_value(258)), A::add(A::mul(s.ad_value(167), s.ad_value(170)), s.ad_value(257))), A::div(A::add(s.ad_value(167), s.ad_value(258)), A::add(A::mul(s.ad_value(167), s.ad_value(170)), s.ad_value(257))));
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1995] != 0.0)) && (!(s.v[2002] != 0.0))) {
            s.store_sub_ad_lhs(174, A::sub(A::neg(A::mul(A::div_from_scalar(1.0, s.ad_value(170)), A::div_from_scalar(1.0, s.ad_value(170)))), A::div_from_scalar(1.0, A::mul(A::mul(A::square(s.ad_value(257)), s.ad_value(257)), A::add(A::mul(s.ad_value(167), s.ad_value(170)), s.ad_value(257))))), 173);
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1995] != 0.0)) && (!(s.v[2002] != 0.0))) {
            s.store_sub_ad_rhs(960, 170, A::mul(A::div(s.ad_value(171), s.ad_value(172)), A::offset(A::div(A::mul(s.ad_value(171), s.ad_value(174)), A::mul(A::scale(s.ad_value(172), 2.0), s.ad_value(172))), 1.0)));
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1995] != 0.0)) {
            s.store_scale_ad(256, A::add(A::offset(s.ad_value(254), 1.0), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(254), (-1.0)), A::offset(s.ad_value(254), (-1.0))), ((0.25 * 2.0) * 2.0)))), 0.5);
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1995] != 0.0)) {
            s.store_sqrt(259, 256);
        }

    }

    pub(super) fn stamp_transient_block_43(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if ((!(s.v[1620] != 0.0)) && (s.v[1995] != 0.0)) {
            s.store_offset_ad(255, A::sub(A::sub(s.ad_value(254), s.ad_value(961)), s.ad_value(960)), (-1.0));
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1995] != 0.0)) {
            s.store_scale_ad(167, A::add(A::offset(s.ad_value(255), 1.0), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(255), (-1.0)), A::offset(s.ad_value(255), (-1.0))), ((0.25 * 2.0) * 2.0)))), 0.5);
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1995] != 0.0)) {
            s.store_sqrt(169, 167);
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1995] != 0.0)) {
            s.store_offset_ad(959, A::div(s.ad_value(954), A::add(s.ad_value(259), s.ad_value(169))), 1.0);
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1995] != 0.0)) {
            s.store_mul_ad_lhs(939, A::mul(A::mul(A::mul(A::scale(A::scale(A::mul(A::scale(s.ad_value(959), (2.0 * p.p2)), s.ad_value(337)), (p.p1147 * 1.0 / (s.v[184]))), s.v[199]), s.ad_value(269)), s.ad_value(269)), A::mul(A::sub(s.ad_value(961), s.ad_value(960)), A::add(A::offset(s.ad_value(961), 1.0), s.ad_value(960)))), 363);
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1995] != 0.0)) {
            s.store_add(380, 939, 380);
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1995] != 0.0)) {
            s.store_scalar(964, (p.p1012 * p.p1316));
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1995] != 0.0)) {
            s.store_scalar(965, (p.p1013 * p.p1316));
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1995] != 0.0)) {
            s.store_scalar(966, (p.p1014 * p.p1316));
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1995] != 0.0)) {
            s.store_sub_from_scalar_ad(962, s.v[184], A::scale(s.ad_value(606), 2.0));
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1995] != 0.0)) {
            s.store_square(963, 962);
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1995] != 0.0)) {
            s.store_mul_ad(613, A::scale(s.ad_value(271), 6.241509744511525e18), A::add(A::offset(s.ad_value(260), s.v[199]), s.ad_value(836)));
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1995] != 0.0)) {
            s.store_scale_ad(612, A::mul(A::mul(A::scale(s.ad_value(959), (2.0 * s.v[199])), s.ad_value(271)), s.ad_value(960)), 6.241509744511525e18);
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1995] != 0.0)) {
            s.store_mul_ad_lhs(1004, A::mul(A::scale(s.ad_value(271), ((1.602176462e-19 * 1.602176462e-19) * 1.602176462e-19)), A::abs(s.ad_value(939))), 337);
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1995] != 0.0)) {
            s.store_mul_ad_lhs(1005, A::mul(A::scale(s.ad_value(271), 1.602176462e-19), s.ad_value(939)), 939);
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1995] != 0.0)) {
            s.store_add_ad(1006, A::add(s.ad_value(964), A::mul(s.ad_value(965), s.ad_value(612))), A::mul(A::mul(s.ad_value(966), s.ad_value(612)), s.ad_value(612)));
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1995] != 0.0)) {
            s.store_mul_ad(1007, A::add(s.ad_value(612), s.ad_value(613)), A::add(s.ad_value(612), s.ad_value(613)));
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1995] != 0.0)) {
            s.store_mul_ad_lhs(1008, A::scale(s.ad_value(964), 1.602176462e-19), 271);
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1995] != 0.0)) {
            s.store_scale_ad(611, A::mul(A::mul(A::scale(s.ad_value(959), (2.0 * s.v[199])), s.ad_value(271)), s.ad_value(961)), 6.241509744511525e18);
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1995] != 0.0)) {
            s.store_mul_ad_rhs(168, 964, A::ln(A::max_with_scalar(A::div(A::add(s.ad_value(611), s.ad_value(613)), A::add(s.ad_value(612), s.ad_value(613))), 1e-38)));
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1995] != 0.0)) {
            s.store_mul_ad_rhs(169, 965, A::sub(s.ad_value(611), s.ad_value(612)));
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1995] != 0.0)) {
            s.store_mul_ad(170, A::scale(s.ad_value(966), 0.5), A::sub(A::square(s.ad_value(611)), A::square(s.ad_value(612))));
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1995] != 0.0)) {
            s.store_scale(171, 963, (10000000000.0 * (p.p1147 * p.p2)));
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1995] != 0.0)) {
            s.store_add_ad(614, A::mul(A::div(s.ad_value(1004), s.ad_value(167)), A::add(A::add(s.ad_value(168), s.ad_value(169)), s.ad_value(170))), A::div(A::mul(A::mul(A::div(s.ad_value(1005), s.ad_value(171)), s.ad_value(610)), s.ad_value(1006)), s.ad_value(1007)));
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1995] != 0.0)) {
            s.store_mul_ad_lhs(172, A::mul(A::scale(s.ad_value(962), ((p.p1147 * p.p2) * 10000000000.0)), s.ad_value(613)), 613);
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1995] != 0.0)) {
            s.store_mul_ad_lhs(615, A::mul(A::div(s.ad_value(1008), s.ad_value(172)), s.ad_value(939)), 939);
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[1995] != 0.0)) {
            s.store_add(173, 615, 614);
        }

        s.v[2005] = if (s.v[173] > 0.0) { 1.0 } else { 0.0 };

        if (((!(s.v[1620] != 0.0)) && (s.v[1995] != 0.0)) && (s.v[2005] != 0.0)) {
            s.store_div_ad_lhs(174, A::mul(s.ad_value(614), s.ad_value(615)), 173);
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1995] != 0.0)) && (s.v[2005] != 0.0)) {
            s.store_offset_ad(175, A::scale(A::powf(A::sub(s.ad_value(961), s.ad_value(960)), p.p1318), p.p1317), 1.0);
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1995] != 0.0)) && (s.v[2005] != 0.0)) {
            s.store_div(967, 174, 175);
        }

        if (((!(s.v[1620] != 0.0)) && (s.v[1995] != 0.0)) && (!(s.v[2005] != 0.0))) {
            s.store_scalar(967, 0.0);
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_mul_ad_rhs(1075, 379, A::add(A::add(A::add(s.ad_value(387), s.ad_value(421)), s.ad_value(520)), s.ad_value(525)));
        }

        s.v[2006] = if (s.v[211] > 0.0) { 1.0 } else { 0.0 };

        if ((!(s.v[1620] != 0.0)) && (s.v[2006] != 0.0)) {
            s.store_mul(1050, 379, 388);
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[2006] != 0.0)) {
            s.store_mul(1051, 379, 395);
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[2006] != 0.0)) {
            s.store_mul(1052, 379, 399);
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[2006] != 0.0)) {
            s.store_mul(1053, 379, 389);
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[2006] != 0.0)) {
            s.store_mul(1054, 379, 394);
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[2006] != 0.0)) {
            s.store_mul(1055, 379, 398);
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[2006] != 0.0)) {
            s.store_mul_ad_rhs(1076, 379, A::sub(A::add(s.ad_value(388), s.ad_value(423)), s.ad_value(520)));
        }

        if ((!(s.v[1620] != 0.0)) && (s.v[2006] != 0.0)) {
            s.store_mul_ad_rhs(1077, 379, A::sub(A::add(s.ad_value(389), s.ad_value(424)), s.ad_value(525)));
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[2006] != 0.0))) {
            s.store_mul(1050, 379, 389);
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[2006] != 0.0))) {
            s.store_mul(1051, 379, 394);
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[2006] != 0.0))) {
            s.store_mul(1052, 379, 398);
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[2006] != 0.0))) {
            s.store_mul(1053, 379, 388);
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[2006] != 0.0))) {
            s.store_mul(1054, 379, 395);
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[2006] != 0.0))) {
            s.store_mul(1055, 379, 399);
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[2006] != 0.0))) {
            s.store_mul_ad_rhs(1076, 379, A::sub(A::add(s.ad_value(389), s.ad_value(423)), s.ad_value(520)));
        }

        if ((!(s.v[1620] != 0.0)) && (!(s.v[2006] != 0.0))) {
            s.store_mul_ad_rhs(1077, 379, A::sub(A::add(s.ad_value(388), s.ad_value(424)), s.ad_value(525)));
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_mul_ad_rhs(1078, 379, A::add(s.ad_value(390), s.ad_value(422)));
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_mul(1057, 379, 392);
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_mul(1058, 379, 396);
        }

        s.v[1108] = s.v[183];

        s.v[1109] = s.v[184];

        s.v[2009] = if (p.p38 != 0.0) { 1.0 } else { 0.0 };

        s.v[2010] = if (p.p37 != 0.0) { 1.0 } else { 0.0 };

        s.v[2011] = if (s.v[211] > 0.0) { 1.0 } else { 0.0 };

        s.v[2012] = if (p.p7 == 0.0) { 1.0 } else { 0.0 };

        s.v[2015] = if (p.p7 == 2.0) { 1.0 } else { 0.0 };

        if ((!(s.v[2012] != 0.0)) && (s.v[2015] != 0.0)) {
            s.copy_ad(2013, 467);
        }

        if ((!(s.v[2012] != 0.0)) && (s.v[2015] != 0.0)) {
            s.store_div_ad_lhs(2014, A::square(s.ad_value(467)), 465);
        }

        if ((!(s.v[2012] != 0.0)) && (!(s.v[2015] != 0.0))) {
            s.copy_ad(2013, 465);
        }

        if ((!(s.v[2012] != 0.0)) && (!(s.v[2015] != 0.0))) {
            s.copy_ad(2014, 465);
        }

        s.v[2016] = if ((p.p33 != 2.0) && (s.v[453] > 0.0)) { 1.0 } else { 0.0 };

        if (s.v[2016] != 0.0) {
            s.store_div_from_scalar(618, 1.0, 455);
        }

        s.v[2017] = if ((p.p33 != 2.0) && (s.v[453] > 0.0)) { 1.0 } else { 0.0 };

        s.v[2018] = if ((p.p33 != 2.0) && (s.v[452] > 0.0)) { 1.0 } else { 0.0 };

        if (s.v[2018] != 0.0) {
            s.store_div_from_scalar(617, 1.0, 454);
        }

        s.v[2019] = if ((p.p33 != 2.0) && (s.v[452] > 0.0)) { 1.0 } else { 0.0 };

        s.v[2020] = if (p.p7 == 3.0) { 1.0 } else { 0.0 };

        s.v[2021] = if ((p.p41 != 0.0) && (p.p1099 > 0.0)) { 1.0 } else { 0.0 };

        if (s.v[2021] != 0.0) {
            s.store_ad(1017, &A::mul(A::mul(A::mul(s.ad_value(379), s.ad_value(211)), s.ad_value(380)), A::voltage(ctx, &nodes, Some(6), Some(7))));
        }

        s.v[2022] = if ((p.p33 != 2.0) && (s.v[453] > 0.0)) { 1.0 } else { 0.0 };

        if ((s.v[2021] != 0.0) && (s.v[2022] != 0.0)) {
            s.store_add_ad_rhs(1017, 1017, A::div(A::square(A::voltage(ctx, &nodes, Some(0), Some(6))), s.ad_value(455)));
        }

        s.v[2023] = if ((p.p33 != 2.0) && (s.v[452] > 0.0)) { 1.0 } else { 0.0 };

        if ((s.v[2021] != 0.0) && (s.v[2023] != 0.0)) {
            s.store_add_ad_rhs(1017, 1017, A::div(A::square(A::voltage(ctx, &nodes, Some(2), Some(7))), s.ad_value(454)));
        }

        s.v[2024] = if ((p.p40 != 0.0) && (!(1.0 != 0.0))) { 1.0 } else { 0.0 };

        s.v[2025] = 1.0;

        s.v[2026] = if ((p.p40 != 0.0) && (!(1.0 != 0.0))) { 1.0 } else { 0.0 };

        s.v[2027] = 1.0;

        s.v[1024] = (p.p1359 * p.p1358);

        s.v[2028] = if ((p.p43 == 0.0) || (1.0 == 0.0)) { 1.0 } else { 0.0 };

        s.v[2029] = if ((p.p40 != 0.0) && (!(1.0 != 0.0))) { 1.0 } else { 0.0 };

        s.v[2030] = if (p.p43 == 1.0) { 1.0 } else { 0.0 };

        if (((!(s.v[2028] != 0.0)) && (!(s.v[2029] != 0.0))) && (s.v[2030] != 0.0)) {
            s.store_scalar(1025, ((((((p.p1357 * p.p1356) * p.p1360) / ((2.0 * p.p1356) + (p.p1360 * s.v[1109]))) * s.v[1108]) / p.p1373) / p.p2));
        }

        s.v[2031] = if (s.v[1025] < 0.001) { 1.0 } else { 0.0 };

        s.v[2032] = if (s.v[1024] <= 0.001) { 1.0 } else { 0.0 };

        if (((((!(s.v[2028] != 0.0)) && (!(s.v[2029] != 0.0))) && (s.v[2030] != 0.0)) && (s.v[2031] != 0.0)) && (s.v[2032] != 0.0)) {
            s.store_scalar(167, (1.0 / 0.001));
        }

        if (((((!(s.v[2028] != 0.0)) && (!(s.v[2029] != 0.0))) && (s.v[2030] != 0.0)) && (s.v[2031] != 0.0)) && (!(s.v[2032] != 0.0))) {
            s.store_scalar(167, (1.0 / s.v[1024]));
        }

        if ((((!(s.v[2028] != 0.0)) && (!(s.v[2029] != 0.0))) && (s.v[2030] != 0.0)) && (s.v[2031] != 0.0)) {
            s.copy_ad(1021, 167);
        }

        if ((((!(s.v[2028] != 0.0)) && (!(s.v[2029] != 0.0))) && (s.v[2030] != 0.0)) && (!(s.v[2031] != 0.0))) {
            s.store_div_from_scalar_ad(1021, 1.0, A::offset(s.ad_value(1025), s.v[1024]));
        }

        if (((!(s.v[2028] != 0.0)) && (!(s.v[2029] != 0.0))) && (!(s.v[2030] != 0.0))) {
            s.store_mul_ad_rhs(1027, 1028, A::pow(s.ad_value(639), s.ad_value(1029)));
        }

        if (((!(s.v[2028] != 0.0)) && (!(s.v[2029] != 0.0))) && (!(s.v[2030] != 0.0))) {
            s.store_add_ad_lhs(1026, A::neg(A::add(A::add(s.ad_value(387), s.ad_value(520)), s.ad_value(525))), 1039);
        }

        if (((!(s.v[2028] != 0.0)) && (!(s.v[2029] != 0.0))) && (!(s.v[2030] != 0.0))) {
            s.store_sub_ad_lhs(1031, A::scale(s.ad_value(1030), (1.602176462e-19 * (p.p74 * (s.v[1108] * s.v[1109])))), 1026);
        }

        if (((!(s.v[2028] != 0.0)) && (!(s.v[2029] != 0.0))) && (!(s.v[2030] != 0.0))) {
            s.store_mul(167, 1027, 1031);
        }

        if (((!(s.v[2028] != 0.0)) && (!(s.v[2029] != 0.0))) && (!(s.v[2030] != 0.0))) {
            s.store_scalar(168, (s.v[1108] * s.v[1108]));
        }

        if (((!(s.v[2028] != 0.0)) && (!(s.v[2029] != 0.0))) && (!(s.v[2030] != 0.0))) {
            s.store_div_ad_lhs(1023, A::scale(s.ad_value(167), p.p2), 168);
        }

        if (((!(s.v[2028] != 0.0)) && (!(s.v[2029] != 0.0))) && (!(s.v[2030] != 0.0))) {
            s.store_div_from_scalar(1025, 1.0, 1023);
        }

        s.v[2033] = if (s.v[1025] < 0.001) { 1.0 } else { 0.0 };

        s.v[2034] = if (s.v[1024] <= 0.001) { 1.0 } else { 0.0 };

        if (((((!(s.v[2028] != 0.0)) && (!(s.v[2029] != 0.0))) && (!(s.v[2030] != 0.0))) && (s.v[2033] != 0.0)) && (s.v[2034] != 0.0)) {
            s.store_scalar(167, (1.0 / 0.001));
        }

        if (((((!(s.v[2028] != 0.0)) && (!(s.v[2029] != 0.0))) && (!(s.v[2030] != 0.0))) && (s.v[2033] != 0.0)) && (!(s.v[2034] != 0.0))) {
            s.store_scalar(167, (1.0 / s.v[1024]));
        }

        if ((((!(s.v[2028] != 0.0)) && (!(s.v[2029] != 0.0))) && (!(s.v[2030] != 0.0))) && (s.v[2033] != 0.0)) {
            s.copy_ad(1021, 167);
        }

        if ((((!(s.v[2028] != 0.0)) && (!(s.v[2029] != 0.0))) && (!(s.v[2030] != 0.0))) && (!(s.v[2033] != 0.0))) {
            s.store_div_from_scalar_ad(1021, 1.0, A::offset(s.ad_value(1025), s.v[1024]));
        }

        s.v[2035] = if (p.p1375 == 2.0) { 1.0 } else { 0.0 };

        if (s.v[2035] != 0.0) {
            s.store_scale(1021, 1021, 2.0);
        }

        s.v[2036] = if (p.p1374 < 0.001) { 1.0 } else { 0.0 };

        if (s.v[2036] != 0.0) {
            s.store_scalar(167, (1.0 / 0.001));
        }

        if (s.v[2036] != 0.0) {
            s.copy_ad(1022, 167);
        }

        if (!(s.v[2036] != 0.0)) {
            s.store_scalar(1022, (1.0 / p.p1374));
        }

        s.v[2037] = 1.0;

        s.v[2038] = if ((p.p40 == 0.0) || (1.0 != 0.0)) { 1.0 } else { 0.0 };

        s.v[2039] = if ((((p.p43 != 0.0) && (1.0 == 1.0)) && (!((p.p40 == 1.0) && (1.0 == 0.0)))) && (p.p45 == 1.0)) { 1.0 } else { 0.0 };

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
        s.v[45] = 0.0;

        s.v[40] = 0.0;

        s.v[254] = 0.0;

        s.v[295] = 0.0;

        s.v[316] = 0.0;

        s.v[478] = 0.0;

        s.v[839] = 0.0;

        s.v[717] = 0.0;

        s.v[691] = 0.0;

        s.v[779] = 0.0;

        s.v[749] = 0.0;

        s.v[756] = 0.0;

        s.v[754] = 0.0;

        s.v[692] = 0.0;

        s.v[916] = 0.0;

        s.v[928] = 0.0;

        s.v[829] = 0.0;

        s.v[833] = 0.0;

        s.v[841] = 0.0;

        s.v[845] = 0.0;

        s.v[849] = 0.0;

        s.v[853] = 0.0;

        s.v[859] = 0.0;

        s.v[863] = 0.0;

        s.v[731] = 0.0;

        s.v[784] = 0.0;

        s.v[658] = 0.0;

        s.v[644] = 0.0;

        s.v[650] = 0.0;

        s.v[745] = 0.0;

        s.v[936] = 0.0;

        s.v[917] = 0.0;

        s.v[830] = 0.0;

        s.v[836] = 0.0;

        s.v[842] = 0.0;

        s.v[846] = 0.0;

        s.v[850] = 0.0;

        s.v[856] = 0.0;

        s.v[860] = 0.0;

        s.v[864] = 0.0;

        s.v[664] = 0.0;

        s.v[762] = 0.0;

        s.v[739] = 0.0;

        s.v[759] = 0.0;

        s.v[753] = 0.0;

        s.v[654] = 0.0;

        s.v[937] = 0.0;

        s.v[956] = 0.0;

        s.v[958] = 0.0;

        s.v[831] = 0.0;

        s.v[837] = 0.0;

        s.v[843] = 0.0;

        s.v[847] = 0.0;

        s.v[851] = 0.0;

        s.v[857] = 0.0;

        s.v[861] = 0.0;

        s.v[685] = 0.0;

        s.v[347] = 0.0;

        s.v[642] = 0.0;

        s.v[646] = 0.0;

        s.v[648] = 0.0;

        s.v[686] = 0.0;

        s.v[938] = 0.0;

        s.v[957] = 0.0;

        s.v[828] = 0.0;

        s.v[832] = 0.0;

        s.v[840] = 0.0;

        s.v[844] = 0.0;

        s.v[848] = 0.0;

        s.v[852] = 0.0;

        s.v[858] = 0.0;

        s.v[862] = 0.0;

        s.v[854] = 0.0;

        s.v[855] = 0.0;

        s.v[460] = 0.0;

        s.v[459] = 0.0;

        s.v[462] = 0.0;

        s.v[461] = 0.0;

        s.v[1019] = 1.0;

        s.v[1020] = 1.0;

        s.v[87] = 1.0;

        s.v[354] = 0.0;

        s.v[339] = 0.0;

        s.v[458] = 0.0;

        s.v[343] = 0.0;

        s.v[344] = 0.0;

        s.v[534] = 0.0;

        s.v[533] = 0.0;

        s.v[834] = 0.0;

        s.v[363] = 0.0;

        s.v[365] = 0.0;

        s.v[334] = 0.0;

        s.v[455] = 0.0;

        s.v[454] = 0.0;

        s.v[315] = 0.0;

        s.v[355] = 0.0;

        s.v[250] = 0.0;

        s.v[243] = 0.0;

        s.v[73] = 0.0;

        s.v[81] = 0.0;

        s.v[457] = 0.0;

        s.v[1048] = (1.3806503e-23 / 1.602176462e-19);

        s.v[320] = 0.0;

        s.v[400] = 0.0;

        s.v[23] = 0.0;

        s.v[22] = 0.0;

        s.v[323] = 0.0;

        s.v[74] = 0.0;

        s.v[80] = 0.0;

        s.v[84] = 0.0;

        s.v[959] = 0.0;

        s.v[960] = 0.0;

        s.v[961] = 0.0;

        s.v[1129] = if (p.p30 == 1.0) { 1.0 } else { 0.0 };

        if (s.v[1129] != 0.0) {
            s.store_scalar(379, 1.0);
        }

        if (!(s.v[1129] != 0.0)) {
            s.store_scalar(379, (-1.0));
        }

        s.v[180] = (p.p109 * 8.8541878128e-12);

        s.v[181] = (p.p110 * 8.8541878128e-12);

        s.v[199] = ((p.p110 * 8.8541878128e-12) / p.p76);

        s.v[200] = (p.p109 / p.p110);

        s.v[1130] = if !(if self.param_given[77] { 1.0 } else { 0.0 } != 0.0) { 1.0 } else { 0.0 };

        if (s.v[1130] != 0.0) {
            s.store_scalar(429, (((p.p76 * p.p110) / 3.9) - p.p78));
        }

        if (!(s.v[1130] != 0.0)) {
            s.store_scalar(429, p.p77);
        }

        s.v[262] = (p.p0 * p.p49);

        s.v[264] = (p.p1 * p.p50);

        s.v[261] = (s.v[262] + p.p51);

        s.v[681] = (s.v[264] / p.p2);

        s.v[263] = (s.v[681] + p.p53);

        s.v[682] = ((s.v[261]) as f64).powf((-p.p58));

        s.v[683] = ((s.v[263]) as f64).powf((-p.p59));

        s.v[684] = (s.v[682] * s.v[683]);

        s.v[192] = (((p.p54 + (p.p55 * s.v[682])) + (p.p56 * s.v[683])) + (p.p57 * s.v[684]));

        s.v[688] = ((s.v[261]) as f64).powf((-p.p64));

        s.v[689] = ((s.v[263]) as f64).powf((-p.p65));

        s.v[690] = (s.v[688] * s.v[689]);

        s.v[193] = (((p.p60 + (p.p61 * s.v[688])) + (p.p62 * s.v[689])) + (p.p63 * s.v[690]));

        s.v[184] = (s.v[261] - (2.0 * s.v[192]));

        s.v[183] = ((s.v[263] - (p.p1375 * p.p1376)) - ((2.0 - p.p1375) * s.v[193]));

        s.v[196] = (((p.p66 + (p.p67 * s.v[682])) + (p.p68 * s.v[683])) + (p.p69 * s.v[684]));

        s.v[197] = (((p.p70 + (p.p71 * s.v[688])) + (p.p72 * s.v[689])) + (p.p73 * s.v[690]));

        s.v[188] = (s.v[261] - (2.0 * s.v[196]));

        s.v[187] = ((s.v[263] - (p.p1375 * p.p1376)) - ((2.0 - p.p1375) * s.v[197]));

        s.v[198] = (((p.p927 + (p.p71 / ((s.v[261]) as f64).powf(p.p64))) + (p.p72 / ((s.v[263]) as f64).powf(p.p65))) + ((p.p73 / ((s.v[261]) as f64).powf(p.p64)) / ((s.v[263]) as f64).powf(p.p65)));

        s.v[189] = (s.v[263] - (2.0 * s.v[198]));

        s.v[694] = (1e-6 / s.v[184]);

        s.v[695] = (1e-6 / s.v[183]);

        s.v[697] = (1e-6 / s.v[188]);

        s.v[698] = (1e-6 / s.v[187]);

        s.v[699] = (1e-6 / p.p48);

        s.v[700] = (1e-6 / p.p52);

        s.v[696] = (s.v[694] * s.v[695]);

        s.v[685] = s.v[682];

        s.v[691] = s.v[688];

        s.v[1142] = if (p.p1026 != 0.0) { 1.0 } else { 0.0 };

        s.v[1143] = if (p.p1026 <= (-s.v[261])) { 1.0 } else { 0.0 };

        if ((s.v[1142] != 0.0) && (!(s.v[1143] != 0.0))) {
            s.store_scalar(685, (((s.v[261] + p.p1026)) as f64).powf((-p.p58)));
        }

        if ((s.v[1142] != 0.0) && (!(s.v[1143] != 0.0))) {
            s.store_scalar(691, (((s.v[261] + p.p1026)) as f64).powf((-p.p64)));
        }

        s.v[686] = s.v[683];

        s.v[692] = s.v[689];

        s.v[1144] = if (p.p1027 != 0.0) { 1.0 } else { 0.0 };

        s.v[1145] = if (p.p1027 <= (-s.v[263])) { 1.0 } else { 0.0 };

        if ((s.v[1144] != 0.0) && (!(s.v[1145] != 0.0))) {
            s.store_scalar(686, (((s.v[263] + p.p1027)) as f64).powf((-p.p59)));
        }

        if ((s.v[1144] != 0.0) && (!(s.v[1145] != 0.0))) {
            s.store_scalar(692, (((s.v[263] + p.p1027)) as f64).powf((-p.p65)));
        }

        s.store_mul(687, 685, 686);

        s.store_add_ad(194, A::add(A::offset(A::scale(s.ad_value(685), p.p55), p.p54), A::scale(s.ad_value(686), p.p56)), A::scale(s.ad_value(687), p.p57));

        s.store_mul(693, 691, 692);

        s.store_add_ad(195, A::add(A::offset(A::scale(s.ad_value(691), p.p61), p.p60), A::scale(s.ad_value(692), p.p62)), A::scale(s.ad_value(693), p.p63));

        s.store_offset_ad(186, A::sub_from_scalar(s.v[261], A::scale(s.ad_value(194), 2.0)), p.p1026);

        s.store_offset_ad(185, A::sub_from_scalar(s.v[263], A::scale(s.ad_value(195), 2.0)), p.p1027);

        s.v[1148] = if (p.p1025 == 1.0) { 1.0 } else { 0.0 };

        if (s.v[1148] != 0.0) {
            s.store_div_from_scalar(701, 1e-6, 186);
        }

        if (s.v[1148] != 0.0) {
            s.store_div_from_scalar(702, 1e-6, 185);
        }

        if (!(s.v[1148] != 0.0)) {
            s.store_div_from_scalar(701, 1.0, 186);
        }

        if (!(s.v[1148] != 0.0)) {
            s.store_div_from_scalar(702, 1.0, 185);
        }

        s.store_mul(703, 701, 702);

        s.store_add_ad(707, A::add(A::offset(A::scale(s.ad_value(701), p.p116), p.p115), A::scale(s.ad_value(702), p.p117)), A::scale(s.ad_value(703), p.p118));

        s.store_add_ad(708, A::add(A::offset(A::scale(s.ad_value(701), p.p120), p.p119), A::scale(s.ad_value(702), p.p121)), A::scale(s.ad_value(703), p.p122));

        s.store_add_ad(793, A::add(A::offset(A::scale(s.ad_value(701), p.p130), p.p129), A::scale(s.ad_value(702), p.p131)), A::scale(s.ad_value(703), p.p132));

        s.store_add_ad(705, A::add(A::offset(A::scale(s.ad_value(701), p.p143), p.p142), A::scale(s.ad_value(702), p.p144)), A::scale(s.ad_value(703), p.p145));

        s.store_add_ad(706, A::add(A::offset(A::scale(s.ad_value(701), p.p88), p.p79), A::scale(s.ad_value(702), p.p89)), A::scale(s.ad_value(703), p.p90));

        s.store_add_ad(794, A::add(A::offset(A::scale(s.ad_value(701), p.p100), p.p91), A::scale(s.ad_value(702), p.p101)), A::scale(s.ad_value(703), p.p102));

        s.store_add_ad(704, A::add(A::offset(A::scale(s.ad_value(701), p.p104), p.p103), A::scale(s.ad_value(702), p.p105)), A::scale(s.ad_value(703), p.p106));

        s.store_add_ad(709, A::add(A::offset(A::scale(s.ad_value(701), p.p233), p.p232), A::scale(s.ad_value(702), p.p234)), A::scale(s.ad_value(703), p.p235));

        s.store_add_ad(720, A::add(A::offset(A::scale(s.ad_value(701), p.p243), p.p236), A::scale(s.ad_value(702), p.p244)), A::scale(s.ad_value(703), p.p245));

        s.store_add_ad(721, A::add(A::offset(A::scale(s.ad_value(701), p.p247), p.p246), A::scale(s.ad_value(702), p.p248)), A::scale(s.ad_value(703), p.p249));

        s.store_add_ad(722, A::add(A::offset(A::scale(s.ad_value(701), p.p251), p.p250), A::scale(s.ad_value(702), p.p252)), A::scale(s.ad_value(703), p.p253));

        s.store_add_ad(725, A::add(A::offset(A::scale(s.ad_value(701), p.p171), p.p170), A::scale(s.ad_value(702), p.p172)), A::scale(s.ad_value(703), p.p173));

        s.store_add_ad(726, A::add(A::offset(A::scale(s.ad_value(701), p.p175), p.p174), A::scale(s.ad_value(702), p.p176)), A::scale(s.ad_value(703), p.p177));

        s.store_add_ad(724, A::add(A::offset(A::scale(s.ad_value(701), p.p179), p.p178), A::scale(s.ad_value(702), p.p180)), A::scale(s.ad_value(703), p.p181));

        s.store_add_ad(728, A::add(A::offset(A::scale(s.ad_value(701), p.p187), p.p186), A::scale(s.ad_value(702), p.p188)), A::scale(s.ad_value(703), p.p189));

        s.store_add_ad(727, A::add(A::offset(A::scale(s.ad_value(701), p.p183), p.p182), A::scale(s.ad_value(702), p.p184)), A::scale(s.ad_value(703), p.p185));

        s.store_add_ad(723, A::add(A::offset(A::scale(s.ad_value(701), p.p255), p.p254), A::scale(s.ad_value(702), p.p256)), A::scale(s.ad_value(703), p.p257));

        s.store_add_ad(710, A::add(A::offset(A::scale(s.ad_value(701), p.p259), p.p258), A::scale(s.ad_value(702), p.p260)), A::scale(s.ad_value(703), p.p261));

        s.store_add_ad(714, A::add(A::offset(A::scale(s.ad_value(701), p.p263), p.p262), A::scale(s.ad_value(702), p.p264)), A::scale(s.ad_value(703), p.p265));

        s.store_add_ad(715, A::add(A::offset(A::scale(s.ad_value(701), p.p1165), p.p1164), A::scale(s.ad_value(702), p.p1166)), A::scale(s.ad_value(703), p.p1167));

        s.store_add_ad(716, A::add(A::offset(A::scale(s.ad_value(701), p.p1192), p.p1191), A::scale(s.ad_value(702), p.p1193)), A::scale(s.ad_value(703), p.p1194));

        s.store_add_ad(719, A::add(A::offset(A::scale(s.ad_value(701), p.p291), p.p288), A::scale(s.ad_value(702), p.p292)), A::scale(s.ad_value(703), p.p293));

        s.store_add_ad(711, A::add(A::offset(A::scale(s.ad_value(701), p.p271), p.p270), A::scale(s.ad_value(702), p.p272)), A::scale(s.ad_value(703), p.p273));

        s.store_add_ad(712, A::add(A::offset(A::scale(s.ad_value(701), p.p1177), p.p1176), A::scale(s.ad_value(702), p.p1178)), A::scale(s.ad_value(703), p.p1179));

        s.store_add_ad(713, A::add(A::offset(A::scale(s.ad_value(701), p.p276), p.p275), A::scale(s.ad_value(702), p.p277)), A::scale(s.ad_value(703), p.p278));

        s.store_add_ad(279, A::add(A::offset(A::scale(s.ad_value(701), p.p147), p.p146), A::scale(s.ad_value(702), p.p148)), A::scale(s.ad_value(703), p.p149));

        s.store_add_ad(280, A::add(A::offset(A::scale(s.ad_value(701), p.p1239), p.p1238), A::scale(s.ad_value(702), p.p1240)), A::scale(s.ad_value(703), p.p1241));

        s.store_add_ad(281, A::add(A::offset(A::scale(s.ad_value(701), p.p151), p.p150), A::scale(s.ad_value(702), p.p152)), A::scale(s.ad_value(703), p.p153));

        s.store_add_ad(282, A::add(A::offset(A::scale(s.ad_value(701), p.p1243), p.p1242), A::scale(s.ad_value(702), p.p1244)), A::scale(s.ad_value(703), p.p1245));

        s.store_add_ad(283, A::add(A::offset(A::scale(s.ad_value(701), p.p155), p.p154), A::scale(s.ad_value(702), p.p156)), A::scale(s.ad_value(703), p.p157));

        s.store_add_ad(285, A::add(A::offset(A::scale(s.ad_value(701), p.p159), p.p158), A::scale(s.ad_value(702), p.p160)), A::scale(s.ad_value(703), p.p161));

        s.store_add_ad(287, A::add(A::offset(A::scale(s.ad_value(701), p.p163), p.p162), A::scale(s.ad_value(702), p.p164)), A::scale(s.ad_value(703), p.p165));

        s.store_add_ad(289, A::add(A::offset(A::scale(s.ad_value(701), p.p167), p.p166), A::scale(s.ad_value(702), p.p168)), A::scale(s.ad_value(703), p.p169));

        s.store_add_ad(284, A::add(A::offset(A::scale(s.ad_value(701), p.p1247), p.p1246), A::scale(s.ad_value(702), p.p1248)), A::scale(s.ad_value(703), p.p1249));

        s.store_add_ad(286, A::add(A::offset(A::scale(s.ad_value(701), p.p1251), p.p1250), A::scale(s.ad_value(702), p.p1252)), A::scale(s.ad_value(703), p.p1253));

        s.store_add_ad(288, A::add(A::offset(A::scale(s.ad_value(701), p.p1255), p.p1254), A::scale(s.ad_value(702), p.p1256)), A::scale(s.ad_value(703), p.p1257));

        s.store_add_ad(290, A::add(A::offset(A::scale(s.ad_value(701), p.p1259), p.p1258), A::scale(s.ad_value(702), p.p1260)), A::scale(s.ad_value(703), p.p1261));

        s.store_add_ad(734, A::add(A::offset(A::scale(s.ad_value(701), p.p225), p.p218), A::scale(s.ad_value(702), p.p226)), A::scale(s.ad_value(703), p.p227));

        s.store_add_ad(735, A::add(A::offset(A::scale(s.ad_value(701), p.p215), p.p208), A::scale(s.ad_value(702), p.p216)), A::scale(s.ad_value(703), p.p217));

        s.store_add_ad(736, A::add(A::offset(A::scale(s.ad_value(701), p.p1203), p.p1196), A::scale(s.ad_value(702), p.p1204)), A::scale(s.ad_value(703), p.p1205));

        s.store_add_ad(782, A::add(A::offset(A::scale(s.ad_value(701), p.p112), p.p111), A::scale(s.ad_value(702), p.p113)), A::scale(s.ad_value(703), p.p114));

        s.store_add_ad(729, A::add(A::offset(A::scale(s.ad_value(701), p.p191), p.p190), A::scale(s.ad_value(702), p.p192)), A::scale(s.ad_value(703), p.p193));

        s.store_add_ad(730, A::add(A::offset(A::scale(s.ad_value(701), p.p195), p.p194), A::scale(s.ad_value(702), p.p196)), A::scale(s.ad_value(703), p.p197));

        s.store_add_ad(733, A::add(A::offset(A::scale(s.ad_value(701), p.p205), p.p203), A::scale(s.ad_value(702), p.p206)), A::scale(s.ad_value(703), p.p207));

        s.store_add_ad(737, A::add(A::offset(A::scale(s.ad_value(701), p.p310), p.p309), A::scale(s.ad_value(702), p.p311)), A::scale(s.ad_value(703), p.p312));

        s.store_add_ad(738, A::add(A::offset(A::scale(s.ad_value(701), p.p340), p.p337), A::scale(s.ad_value(702), p.p341)), A::scale(s.ad_value(703), p.p342));

        s.store_add_ad(748, A::add(A::offset(A::scale(s.ad_value(701), p.p355), p.p348), A::scale(s.ad_value(702), p.p356)), A::scale(s.ad_value(703), p.p357));

        s.store_add_ad(752, A::add(A::offset(A::scale(s.ad_value(701), p.p375), p.p372), A::scale(s.ad_value(702), p.p376)), A::scale(s.ad_value(703), p.p377));

        s.store_add_ad(751, A::add(A::offset(A::scale(s.ad_value(701), p.p363), p.p362), A::scale(s.ad_value(702), p.p364)), A::scale(s.ad_value(703), p.p365));

        s.store_add_ad(755, A::add(A::offset(A::scale(s.ad_value(701), p.p383), p.p382), A::scale(s.ad_value(702), p.p384)), A::scale(s.ad_value(703), p.p385));

        s.store_add_ad(758, A::add(A::offset(A::scale(s.ad_value(701), p.p397), p.p390), A::scale(s.ad_value(702), p.p398)), A::scale(s.ad_value(703), p.p399));

        s.store_add_ad(783, A::add(A::offset(A::scale(s.ad_value(701), p.p407), p.p404), A::scale(s.ad_value(702), p.p408)), A::scale(s.ad_value(703), p.p409));

        s.store_add_ad(786, A::add(A::offset(A::scale(s.ad_value(701), p.p418), p.p415), A::scale(s.ad_value(702), p.p419)), A::scale(s.ad_value(703), p.p420));

        s.store_add_ad(775, A::add(A::offset(A::scale(s.ad_value(701), p.p458), p.p457), A::scale(s.ad_value(702), p.p459)), A::scale(s.ad_value(703), p.p460));

        s.store_add_ad(774, A::add(A::offset(A::scale(s.ad_value(701), p.p468), p.p467), A::scale(s.ad_value(702), p.p469)), A::scale(s.ad_value(703), p.p470));

        s.store_add_ad(770, A::add(A::offset(A::scale(s.ad_value(701), p.p440), p.p439), A::scale(s.ad_value(702), p.p441)), A::scale(s.ad_value(703), p.p442));

        s.store_add_ad(787, A::add(A::offset(A::scale(s.ad_value(701), p.p444), p.p443), A::scale(s.ad_value(702), p.p445)), A::scale(s.ad_value(703), p.p446));

        s.store_add_ad(771, A::add(A::offset(A::scale(s.ad_value(701), p.p450), p.p449), A::scale(s.ad_value(702), p.p451)), A::scale(s.ad_value(703), p.p452));

        s.store_add_ad(773, A::add(A::offset(A::scale(s.ad_value(701), p.p454), p.p453), A::scale(s.ad_value(702), p.p455)), A::scale(s.ad_value(703), p.p456));

        s.store_add_ad(772, A::add(A::offset(A::scale(s.ad_value(701), p.p464), p.p463), A::scale(s.ad_value(702), p.p465)), A::scale(s.ad_value(703), p.p466));

        s.store_add_ad(776, A::add(A::offset(A::scale(s.ad_value(701), p.p480), p.p477), A::scale(s.ad_value(702), p.p481)), A::scale(s.ad_value(703), p.p482));

        s.store_add_ad(777, A::add(A::offset(A::scale(s.ad_value(701), p.p474), p.p473), A::scale(s.ad_value(702), p.p475)), A::scale(s.ad_value(703), p.p476));

        s.store_add_ad(778, A::add(A::offset(A::scale(s.ad_value(701), p.p499), p.p498), A::scale(s.ad_value(702), p.p500)), A::scale(s.ad_value(703), p.p501));

        s.store_add_ad(761, A::add(A::offset(A::scale(s.ad_value(701), p.p533), p.p530), A::scale(s.ad_value(702), p.p534)), A::scale(s.ad_value(703), p.p535));

        s.store_add_ad(764, A::add(A::offset(A::scale(s.ad_value(701), p.p541), p.p540), A::scale(s.ad_value(702), p.p542)), A::scale(s.ad_value(703), p.p543));

        s.store_add_ad(765, A::add(A::offset(A::scale(s.ad_value(701), p.p422), p.p421), A::scale(s.ad_value(702), p.p423)), A::scale(s.ad_value(703), p.p424));

        s.store_add_ad(766, A::add(A::offset(A::scale(s.ad_value(701), p.p426), p.p425), A::scale(s.ad_value(702), p.p427)), A::scale(s.ad_value(703), p.p428));

        s.store_add_ad(767, A::add(A::offset(A::scale(s.ad_value(701), p.p430), p.p429), A::scale(s.ad_value(702), p.p431)), A::scale(s.ad_value(703), p.p432));

        s.store_add_ad(768, A::add(A::offset(A::scale(s.ad_value(701), p.p435), p.p434), A::scale(s.ad_value(702), p.p436)), A::scale(s.ad_value(703), p.p437));

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
        s.store_add_ad(769, A::add(A::offset(A::scale(s.ad_value(701), p.p551), p.p548), A::scale(s.ad_value(702), p.p552)), A::scale(s.ad_value(703), p.p553));

        s.store_add_ad(781, A::add(A::offset(A::scale(s.ad_value(701), p.p545), p.p544), A::scale(s.ad_value(702), p.p546)), A::scale(s.ad_value(703), p.p547));

        s.store_add_ad(741, A::add(A::offset(A::scale(s.ad_value(701), p.p296), p.p295), A::scale(s.ad_value(702), p.p297)), A::scale(s.ad_value(703), p.p298));

        s.store_add_ad(742, A::add(A::offset(A::scale(s.ad_value(701), p.p511), p.p510), A::scale(s.ad_value(702), p.p512)), A::scale(s.ad_value(703), p.p513));

        s.store_add_ad(744, A::add(A::offset(A::scale(s.ad_value(701), p.p326), p.p325), A::scale(s.ad_value(702), p.p327)), A::scale(s.ad_value(703), p.p328));

        s.store_add_ad(743, A::add(A::offset(A::scale(s.ad_value(701), p.p330), p.p329), A::scale(s.ad_value(702), p.p331)), A::scale(s.ad_value(703), p.p332));

        s.store_add_ad(346, A::add(A::offset(A::scale(s.ad_value(701), p.p484), p.p483), A::scale(s.ad_value(702), p.p485)), A::scale(s.ad_value(703), p.p486));

        s.store_add_ad(747, A::add(A::offset(A::scale(s.ad_value(701), p.p316), p.p315), A::scale(s.ad_value(702), p.p317)), A::scale(s.ad_value(703), p.p318));

        s.store_add_ad(788, A::add(A::offset(A::scale(s.ad_value(701), p.p868), p.p867), A::scale(s.ad_value(702), p.p869)), A::scale(s.ad_value(703), p.p870));

        s.store_add_ad(789, A::add(A::offset(A::scale(s.ad_value(701), p.p876), p.p875), A::scale(s.ad_value(702), p.p877)), A::scale(s.ad_value(703), p.p878));

        s.store_add_ad(790, A::add(A::offset(A::scale(s.ad_value(701), p.p880), p.p879), A::scale(s.ad_value(702), p.p881)), A::scale(s.ad_value(703), p.p882));

        s.store_add_ad(791, A::add(A::offset(A::scale(s.ad_value(701), p.p884), p.p883), A::scale(s.ad_value(702), p.p885)), A::scale(s.ad_value(703), p.p886));

        s.store_add_ad(792, A::add(A::offset(A::scale(s.ad_value(701), p.p888), p.p887), A::scale(s.ad_value(702), p.p889)), A::scale(s.ad_value(703), p.p890));

        s.store_add_ad(865, A::add(A::offset(A::scale(s.ad_value(701), p.p604), p.p601), A::scale(s.ad_value(702), p.p605)), A::scale(s.ad_value(703), p.p606));

        s.store_add_ad(866, A::add(A::offset(A::scale(s.ad_value(701), p.p608), p.p607), A::scale(s.ad_value(702), p.p609)), A::scale(s.ad_value(703), p.p610));

        s.store_add_ad(867, A::add(A::offset(A::scale(s.ad_value(701), p.p612), p.p611), A::scale(s.ad_value(702), p.p613)), A::scale(s.ad_value(703), p.p614));

        s.store_add_ad(868, A::add(A::offset(A::scale(s.ad_value(701), p.p616), p.p615), A::scale(s.ad_value(702), p.p617)), A::scale(s.ad_value(703), p.p618));

        s.store_add_ad(869, A::add(A::offset(A::scale(s.ad_value(701), p.p620), p.p619), A::scale(s.ad_value(702), p.p621)), A::scale(s.ad_value(703), p.p622));

        s.store_add_ad(870, A::add(A::offset(A::scale(s.ad_value(701), p.p624), p.p623), A::scale(s.ad_value(702), p.p625)), A::scale(s.ad_value(703), p.p626));

        s.store_add_ad(871, A::add(A::offset(A::scale(s.ad_value(701), p.p628), p.p627), A::scale(s.ad_value(702), p.p629)), A::scale(s.ad_value(703), p.p630));

        s.store_add_ad(872, A::add(A::offset(A::scale(s.ad_value(701), p.p632), p.p631), A::scale(s.ad_value(702), p.p633)), A::scale(s.ad_value(703), p.p634));

        s.store_add_ad(873, A::add(A::offset(A::scale(s.ad_value(701), p.p636), p.p635), A::scale(s.ad_value(702), p.p637)), A::scale(s.ad_value(703), p.p638));

        s.store_add_ad(874, A::add(A::offset(A::scale(s.ad_value(701), p.p597), p.p596), A::scale(s.ad_value(702), p.p598)), A::scale(s.ad_value(703), p.p599));

        s.store_add_ad(875, A::add(A::offset(A::scale(s.ad_value(701), p.p640), p.p639), A::scale(s.ad_value(702), p.p641)), A::scale(s.ad_value(703), p.p642));

        s.store_add_ad(877, A::add(A::offset(A::scale(s.ad_value(701), p.p655), p.p650), A::scale(s.ad_value(702), p.p658)), A::scale(s.ad_value(703), p.p661));

        s.store_add_ad(878, A::add(A::offset(A::scale(s.ad_value(701), p.p654), p.p651), A::scale(s.ad_value(702), p.p657)), A::scale(s.ad_value(703), p.p660));

        s.store_add_ad(879, A::add(A::offset(A::scale(s.ad_value(701), p.p653), p.p652), A::scale(s.ad_value(702), p.p656)), A::scale(s.ad_value(703), p.p659));

        s.store_add_ad(880, A::add(A::offset(A::scale(s.ad_value(701), p.p663), p.p662), A::scale(s.ad_value(702), p.p664)), A::scale(s.ad_value(703), p.p665));

        s.store_add_ad(881, A::add(A::offset(A::scale(s.ad_value(701), p.p668), p.p667), A::scale(s.ad_value(702), p.p669)), A::scale(s.ad_value(703), p.p670));

        s.store_add_ad(1028, A::add(A::offset(A::scale(s.ad_value(701), p.p1362), p.p1361), A::scale(s.ad_value(702), p.p1363)), A::scale(s.ad_value(703), p.p1364));

        s.store_add_ad(1029, A::add(A::offset(A::scale(s.ad_value(701), p.p1366), p.p1365), A::scale(s.ad_value(702), p.p1367)), A::scale(s.ad_value(703), p.p1368));

        s.store_add_ad(1030, A::add(A::offset(A::scale(s.ad_value(701), p.p1370), p.p1369), A::scale(s.ad_value(702), p.p1371)), A::scale(s.ad_value(703), p.p1372));

        s.store_add_ad(547, A::add(A::offset(A::scale(s.ad_value(701), p.p929), p.p928), A::scale(s.ad_value(702), p.p930)), A::scale(s.ad_value(703), p.p931));

        s.store_add_ad(550, A::add(A::offset(A::scale(s.ad_value(701), p.p934), p.p932), A::scale(s.ad_value(702), p.p936)), A::scale(s.ad_value(703), p.p938));

        s.store_add_ad(551, A::add(A::offset(A::scale(s.ad_value(701), p.p935), p.p933), A::scale(s.ad_value(702), p.p937)), A::scale(s.ad_value(703), p.p939));

        s.store_add_ad(557, A::add(A::offset(A::scale(s.ad_value(701), p.p941), p.p940), A::scale(s.ad_value(702), p.p942)), A::scale(s.ad_value(703), p.p943));

        s.store_add_ad(564, A::add(A::offset(A::scale(s.ad_value(701), p.p945), p.p944), A::scale(s.ad_value(702), p.p946)), A::scale(s.ad_value(703), p.p947));

        s.store_add_ad(556, A::add(A::offset(A::scale(s.ad_value(701), p.p949), p.p948), A::scale(s.ad_value(702), p.p950)), A::scale(s.ad_value(703), p.p951));

        s.store_add_ad(552, A::add(A::offset(A::scale(s.ad_value(701), p.p954), p.p952), A::scale(s.ad_value(702), p.p956)), A::scale(s.ad_value(703), p.p958));

        s.store_add_ad(553, A::add(A::offset(A::scale(s.ad_value(701), p.p955), p.p953), A::scale(s.ad_value(702), p.p957)), A::scale(s.ad_value(703), p.p959));

        s.store_add_ad(565, A::add(A::offset(A::scale(s.ad_value(701), p.p962), p.p960), A::scale(s.ad_value(702), p.p964)), A::scale(s.ad_value(703), p.p966));

        s.store_add_ad(566, A::add(A::offset(A::scale(s.ad_value(701), p.p963), p.p961), A::scale(s.ad_value(702), p.p965)), A::scale(s.ad_value(703), p.p967));

        s.store_add_ad(567, A::add(A::offset(A::scale(s.ad_value(701), p.p970), p.p968), A::scale(s.ad_value(702), p.p972)), A::scale(s.ad_value(703), p.p974));

        s.store_add_ad(568, A::add(A::offset(A::scale(s.ad_value(701), p.p971), p.p969), A::scale(s.ad_value(702), p.p973)), A::scale(s.ad_value(703), p.p975));

        s.store_add_ad(569, A::add(A::offset(A::scale(s.ad_value(701), p.p978), p.p976), A::scale(s.ad_value(702), p.p980)), A::scale(s.ad_value(703), p.p982));

        s.store_add_ad(570, A::add(A::offset(A::scale(s.ad_value(701), p.p979), p.p977), A::scale(s.ad_value(702), p.p981)), A::scale(s.ad_value(703), p.p983));

        s.store_add_ad(573, A::add(A::offset(A::scale(s.ad_value(701), p.p986), p.p984), A::scale(s.ad_value(702), p.p988)), A::scale(s.ad_value(703), p.p990));

        s.store_add_ad(574, A::add(A::offset(A::scale(s.ad_value(701), p.p987), p.p985), A::scale(s.ad_value(702), p.p989)), A::scale(s.ad_value(703), p.p991));

        s.store_add_ad(575, A::add(A::offset(A::scale(s.ad_value(701), p.p994), p.p992), A::scale(s.ad_value(702), p.p996)), A::scale(s.ad_value(703), p.p998));

        s.store_add_ad(576, A::add(A::offset(A::scale(s.ad_value(701), p.p995), p.p993), A::scale(s.ad_value(702), p.p997)), A::scale(s.ad_value(703), p.p999));

        s.store_add_ad(558, A::add(A::offset(A::scale(s.ad_value(701), p.p1002), p.p1000), A::scale(s.ad_value(702), p.p1004)), A::scale(s.ad_value(703), p.p1006));

        s.store_add_ad(559, A::add(A::offset(A::scale(s.ad_value(701), p.p1003), p.p1001), A::scale(s.ad_value(702), p.p1005)), A::scale(s.ad_value(703), p.p1007));

        s.store_add_ad(581, A::add(A::offset(A::scale(s.ad_value(701), p.p556), p.p555), A::scale(s.ad_value(702), p.p557)), A::scale(s.ad_value(703), p.p558));

        s.store_add_ad(582, A::add(A::offset(A::scale(s.ad_value(701), p.p560), p.p559), A::scale(s.ad_value(702), p.p561)), A::scale(s.ad_value(703), p.p562));

        s.store_add_ad(587, A::add(A::offset(A::scale(s.ad_value(701), p.p565), p.p563), A::scale(s.ad_value(702), p.p567)), A::scale(s.ad_value(703), p.p569));

        s.store_add_ad(588, A::add(A::offset(A::scale(s.ad_value(701), p.p566), p.p564), A::scale(s.ad_value(702), p.p568)), A::scale(s.ad_value(703), p.p570));

        s.store_add_ad(589, A::add(A::offset(A::scale(s.ad_value(701), p.p572), p.p571), A::scale(s.ad_value(702), p.p573)), A::scale(s.ad_value(703), p.p574));

        s.store_add_ad(590, A::add(A::offset(A::scale(s.ad_value(701), p.p576), p.p575), A::scale(s.ad_value(702), p.p577)), A::scale(s.ad_value(703), p.p578));

        s.store_add_ad(598, A::add(A::offset(A::scale(s.ad_value(701), p.p582), p.p579), A::scale(s.ad_value(702), p.p581)), A::scale(s.ad_value(703), p.p580));

        s.store_add_ad(597, A::add(A::offset(A::scale(s.ad_value(701), p.p584), p.p583), A::scale(s.ad_value(702), p.p585)), A::scale(s.ad_value(703), p.p586));

        s.store_add_ad(600, A::add(A::offset(A::scale(s.ad_value(701), p.p588), p.p587), A::scale(s.ad_value(702), p.p590)), A::scale(s.ad_value(703), p.p592));

        s.store_add_ad(601, A::add(A::offset(A::scale(s.ad_value(701), p.p589), p.p594), A::scale(s.ad_value(702), p.p591)), A::scale(s.ad_value(703), p.p593));

        s.store_add_ad(530, A::add(A::offset(A::scale(s.ad_value(701), p.p922), p.p921), A::scale(s.ad_value(702), p.p923)), A::scale(s.ad_value(703), p.p924));

        s.store_add_ad(806, A::add(A::offset(A::scale(s.ad_value(701), p.p1126), p.p1125), A::scale(s.ad_value(702), p.p1127)), A::scale(s.ad_value(703), p.p1128));

        s.store_add_ad(807, A::add(A::offset(A::scale(s.ad_value(701), p.p1130), p.p1129), A::scale(s.ad_value(702), p.p1131)), A::scale(s.ad_value(703), p.p1132));

        s.store_add_ad(808, A::add(A::offset(A::scale(s.ad_value(701), p.p1134), p.p1133), A::scale(s.ad_value(702), p.p1135)), A::scale(s.ad_value(703), p.p1136));

        s.store_add_ad(892, A::add(A::offset(A::scale(s.ad_value(701), p.p802), p.p799), A::scale(s.ad_value(702), p.p803)), A::scale(s.ad_value(703), p.p804));

        s.store_add_ad(893, A::add(A::offset(A::scale(s.ad_value(701), p.p807), p.p805), A::scale(s.ad_value(702), p.p808)), A::scale(s.ad_value(703), p.p809));

        s.store_add_ad(900, A::add(A::offset(A::scale(s.ad_value(701), p.p810), p.p806), A::scale(s.ad_value(702), p.p811)), A::scale(s.ad_value(703), p.p812));

        s.store_add_ad(894, A::add(A::offset(A::scale(s.ad_value(701), p.p814), p.p813), A::scale(s.ad_value(702), p.p815)), A::scale(s.ad_value(703), p.p816));

        s.store_add_ad(895, A::add(A::offset(A::scale(s.ad_value(701), p.p818), p.p817), A::scale(s.ad_value(702), p.p819)), A::scale(s.ad_value(703), p.p820));

        s.store_add_ad(896, A::add(A::offset(A::scale(s.ad_value(701), p.p824), p.p821), A::scale(s.ad_value(702), p.p825)), A::scale(s.ad_value(703), p.p826));

        s.store_add_ad(897, A::add(A::offset(A::scale(s.ad_value(701), p.p829), p.p827), A::scale(s.ad_value(702), p.p830)), A::scale(s.ad_value(703), p.p831));

        s.store_add_ad(901, A::add(A::offset(A::scale(s.ad_value(701), p.p832), p.p828), A::scale(s.ad_value(702), p.p833)), A::scale(s.ad_value(703), p.p834));

        s.store_add_ad(898, A::add(A::offset(A::scale(s.ad_value(701), p.p836), p.p835), A::scale(s.ad_value(702), p.p837)), A::scale(s.ad_value(703), p.p838));

        s.store_add_ad(899, A::add(A::offset(A::scale(s.ad_value(701), p.p840), p.p839), A::scale(s.ad_value(702), p.p841)), A::scale(s.ad_value(703), p.p842));

        s.store_add_ad(905, A::add(A::offset(A::scale(s.ad_value(701), p.p856), p.p855), A::scale(s.ad_value(702), p.p857)), A::scale(s.ad_value(703), p.p858));

        s.store_add_ad(902, A::add(A::offset(A::scale(s.ad_value(701), p.p844), p.p843), A::scale(s.ad_value(702), p.p845)), A::scale(s.ad_value(703), p.p846));

        s.store_add_ad(906, A::add(A::offset(A::scale(s.ad_value(701), p.p860), p.p859), A::scale(s.ad_value(702), p.p861)), A::scale(s.ad_value(703), p.p862));

        s.store_add_ad(903, A::add(A::offset(A::scale(s.ad_value(701), p.p848), p.p847), A::scale(s.ad_value(702), p.p849)), A::scale(s.ad_value(703), p.p850));

        s.store_add_ad(907, A::add(A::offset(A::scale(s.ad_value(701), p.p864), p.p863), A::scale(s.ad_value(702), p.p865)), A::scale(s.ad_value(703), p.p866));

        s.store_add_ad(904, A::add(A::offset(A::scale(s.ad_value(701), p.p852), p.p851), A::scale(s.ad_value(702), p.p853)), A::scale(s.ad_value(703), p.p854));

        s.store_add_ad(796, A::add(A::offset(A::scale(s.ad_value(701), p.p1033), p.p1032), A::scale(s.ad_value(702), p.p1034)), A::scale(s.ad_value(703), p.p1035));

        s.store_add_ad(797, A::add(A::offset(A::scale(s.ad_value(701), p.p1038), p.p1037), A::scale(s.ad_value(702), p.p1039)), A::scale(s.ad_value(703), p.p1040));

        s.store_add_ad(798, A::add(A::offset(A::scale(s.ad_value(701), p.p1043), p.p1042), A::scale(s.ad_value(702), p.p1044)), A::scale(s.ad_value(703), p.p1045));

        s.store_add_ad(799, A::add(A::offset(A::scale(s.ad_value(701), p.p1047), p.p1046), A::scale(s.ad_value(702), p.p1048)), A::scale(s.ad_value(703), p.p1049));

        s.store_add_ad(805, A::add(A::offset(A::scale(s.ad_value(701), p.p1052), p.p1051), A::scale(s.ad_value(702), p.p1053)), A::scale(s.ad_value(703), p.p1054));

        s.store_add_ad(800, A::add(A::offset(A::scale(s.ad_value(701), p.p1056), p.p1055), A::scale(s.ad_value(702), p.p1057)), A::scale(s.ad_value(703), p.p1058));

        s.store_add_ad(801, A::add(A::offset(A::scale(s.ad_value(701), p.p1061), p.p1060), A::scale(s.ad_value(702), p.p1062)), A::scale(s.ad_value(703), p.p1063));

        s.store_add_ad(802, A::add(A::offset(A::scale(s.ad_value(701), p.p1065), p.p1064), A::scale(s.ad_value(702), p.p1066)), A::scale(s.ad_value(703), p.p1067));

        s.store_add_ad(803, A::add(A::offset(A::scale(s.ad_value(701), p.p1071), p.p1070), A::scale(s.ad_value(702), p.p1072)), A::scale(s.ad_value(703), p.p1073));

        s.store_add_ad(804, A::add(A::offset(A::scale(s.ad_value(701), p.p1086), p.p1085), A::scale(s.ad_value(702), p.p1087)), A::scale(s.ad_value(703), p.p1088));

        s.store_add_ad(809, A::add(A::offset(A::scale(s.ad_value(701), p.p732), p.p706), A::scale(s.ad_value(702), p.p733)), A::scale(s.ad_value(703), p.p734));

        s.store_add_ad(882, A::add(A::offset(A::scale(s.ad_value(701), p.p685), p.p684), A::scale(s.ad_value(702), p.p686)), A::scale(s.ad_value(703), p.p687));

        s.store_add_ad(887, A::add(A::offset(A::scale(s.ad_value(701), p.p689), p.p688), A::scale(s.ad_value(702), p.p690)), A::scale(s.ad_value(703), p.p691));

        s.store_add_ad(883, A::add(A::offset(A::scale(s.ad_value(701), p.p693), p.p692), A::scale(s.ad_value(702), p.p694)), A::scale(s.ad_value(703), p.p695));

        s.store_add_ad(884, A::add(A::offset(A::scale(s.ad_value(701), p.p673), p.p672), A::scale(s.ad_value(702), p.p674)), A::scale(s.ad_value(703), p.p675));

        s.store_add_ad(886, A::add(A::offset(A::scale(s.ad_value(701), p.p677), p.p676), A::scale(s.ad_value(702), p.p678)), A::scale(s.ad_value(703), p.p679));

        s.store_add_ad(885, A::add(A::offset(A::scale(s.ad_value(701), p.p681), p.p680), A::scale(s.ad_value(702), p.p682)), A::scale(s.ad_value(703), p.p683));

        s.store_add_ad(810, A::add(A::offset(A::scale(s.ad_value(701), p.p735), p.p707), A::scale(s.ad_value(702), p.p737)), A::scale(s.ad_value(703), p.p739));

        s.store_add_ad(813, A::add(A::offset(A::scale(s.ad_value(701), p.p736), p.p726), A::scale(s.ad_value(702), p.p738)), A::scale(s.ad_value(703), p.p740));

        s.store_add_ad(811, A::add(A::offset(A::scale(s.ad_value(701), p.p741), p.p708), A::scale(s.ad_value(702), p.p742)), A::scale(s.ad_value(703), p.p743));

        s.store_add_ad(812, A::add(A::offset(A::scale(s.ad_value(701), p.p744), p.p709), A::scale(s.ad_value(702), p.p745)), A::scale(s.ad_value(703), p.p746));

        s.store_add_ad(816, A::add(A::offset(A::scale(s.ad_value(701), p.p747), p.p710), A::scale(s.ad_value(702), p.p749)), A::scale(s.ad_value(703), p.p751));

        s.store_add_ad(814, A::add(A::offset(A::scale(s.ad_value(701), p.p748), p.p711), A::scale(s.ad_value(702), p.p750)), A::scale(s.ad_value(703), p.p752));

        s.store_add_ad(817, A::add(A::offset(A::scale(s.ad_value(701), p.p753), p.p712), A::scale(s.ad_value(702), p.p754)), A::scale(s.ad_value(703), p.p755));

        s.store_add_ad(818, A::add(A::offset(A::scale(s.ad_value(701), p.p756), p.p713), A::scale(s.ad_value(702), p.p757)), A::scale(s.ad_value(703), p.p758));

        s.store_add_ad(819, A::add(A::offset(A::scale(s.ad_value(701), p.p759), p.p714), A::scale(s.ad_value(702), p.p761)), A::scale(s.ad_value(703), p.p763));

        s.store_add_ad(815, A::add(A::offset(A::scale(s.ad_value(701), p.p760), p.p715), A::scale(s.ad_value(702), p.p762)), A::scale(s.ad_value(703), p.p764));

        s.store_add_ad(820, A::add(A::offset(A::scale(s.ad_value(701), p.p765), p.p716), A::scale(s.ad_value(702), p.p766)), A::scale(s.ad_value(703), p.p767));

        s.store_add_ad(821, A::add(A::offset(A::scale(s.ad_value(701), p.p768), p.p717), A::scale(s.ad_value(702), p.p769)), A::scale(s.ad_value(703), p.p770));

        s.store_add_ad(822, A::add(A::offset(A::scale(s.ad_value(701), p.p771), p.p720), A::scale(s.ad_value(702), p.p772)), A::scale(s.ad_value(703), p.p773));

        s.store_add_ad(826, A::add(A::offset(A::scale(s.ad_value(701), p.p780), p.p721), A::scale(s.ad_value(702), p.p781)), A::scale(s.ad_value(703), p.p782));

        s.store_add_ad(679, A::add(A::offset(A::scale(s.ad_value(701), p.p1078), p.p1075), A::scale(s.ad_value(702), p.p1079)), A::scale(s.ad_value(703), p.p1080));

        s.store_add_ad(680, A::add(A::offset(A::scale(s.ad_value(701), p.p1082), p.p1081), A::scale(s.ad_value(702), p.p1083)), A::scale(s.ad_value(703), p.p1084));

        s.store_add_ad(678, A::add(A::offset(A::scale(s.ad_value(701), p.p494), p.p489), A::scale(s.ad_value(702), p.p495)), A::scale(s.ad_value(703), p.p496));

        s.store_add_ad(328, A::add(A::offset(A::scale(s.ad_value(701), p.p515), p.p514), A::scale(s.ad_value(702), p.p516)), A::scale(s.ad_value(703), p.p517));

        s.store_add_ad(329, A::add(A::offset(A::scale(s.ad_value(701), p.p519), p.p518), A::scale(s.ad_value(702), p.p520)), A::scale(s.ad_value(703), p.p521));

        s.store_add_ad(331, A::add(A::offset(A::scale(s.ad_value(701), p.p523), p.p522), A::scale(s.ad_value(702), p.p524)), A::scale(s.ad_value(703), p.p525));

        s.store_add_ad(332, A::add(A::offset(A::scale(s.ad_value(701), p.p527), p.p526), A::scale(s.ad_value(702), p.p528)), A::scale(s.ad_value(703), p.p529));

        s.store_add_ad(828, A::add(A::offset(A::scale(s.ad_value(701), p.p1301), p.p1300), A::scale(s.ad_value(702), p.p1302)), A::scale(s.ad_value(703), p.p1303));

        s.store_add_ad(829, A::add(A::offset(A::scale(s.ad_value(701), p.p1309), p.p1308), A::scale(s.ad_value(702), p.p1310)), A::scale(s.ad_value(703), p.p1311));

        s.store_add_ad(830, A::add(A::offset(A::scale(s.ad_value(701), p.p1305), p.p1304), A::scale(s.ad_value(702), p.p1306)), A::scale(s.ad_value(703), p.p1307));

        s.store_add_ad(831, A::add(A::offset(A::scale(s.ad_value(701), p.p1313), p.p1312), A::scale(s.ad_value(702), p.p1314)), A::scale(s.ad_value(703), p.p1315));

        s.store_add_ad(835, A::add(A::offset(A::scale(s.ad_value(701), p.p1157), p.p1156), A::scale(s.ad_value(702), p.p1158)), A::scale(s.ad_value(703), p.p1159));

        s.store_add_ad(953, A::add(A::offset(A::scale(s.ad_value(701), p.p1153), p.p1152), A::scale(s.ad_value(702), p.p1154)), A::scale(s.ad_value(703), p.p1155));

        s.store_add_ad(836, A::add(A::offset(A::scale(s.ad_value(701), p.p1161), p.p1160), A::scale(s.ad_value(702), p.p1162)), A::scale(s.ad_value(703), p.p1163));

        s.store_add_ad(837, A::add(A::offset(A::scale(s.ad_value(701), p.p1169), p.p1168), A::scale(s.ad_value(702), p.p1170)), A::scale(s.ad_value(703), p.p1171));

        s.store_add_ad(840, A::add(A::offset(A::scale(s.ad_value(701), p.p1187), p.p1186), A::scale(s.ad_value(702), p.p1188)), A::scale(s.ad_value(703), p.p1189));

        s.store_add_ad(841, A::add(A::offset(A::scale(s.ad_value(701), p.p1207), p.p1206), A::scale(s.ad_value(702), p.p1208)), A::scale(s.ad_value(703), p.p1209));

        s.store_add_ad(842, A::add(A::offset(A::scale(s.ad_value(701), p.p1211), p.p1210), A::scale(s.ad_value(702), p.p1212)), A::scale(s.ad_value(703), p.p1213));

        s.store_add_ad(843, A::add(A::offset(A::scale(s.ad_value(701), p.p1215), p.p1214), A::scale(s.ad_value(702), p.p1216)), A::scale(s.ad_value(703), p.p1217));

        s.store_add_ad(844, A::add(A::offset(A::scale(s.ad_value(701), p.p1219), p.p1218), A::scale(s.ad_value(702), p.p1220)), A::scale(s.ad_value(703), p.p1221));

        s.store_add_ad(845, A::add(A::offset(A::scale(s.ad_value(701), p.p1223), p.p1222), A::scale(s.ad_value(702), p.p1224)), A::scale(s.ad_value(703), p.p1225));

        s.store_add_ad(846, A::add(A::offset(A::scale(s.ad_value(701), p.p1227), p.p1226), A::scale(s.ad_value(702), p.p1228)), A::scale(s.ad_value(703), p.p1229));

        s.store_add_ad(847, A::add(A::offset(A::scale(s.ad_value(701), p.p1231), p.p1230), A::scale(s.ad_value(702), p.p1232)), A::scale(s.ad_value(703), p.p1233));

        s.store_add_ad(848, A::add(A::offset(A::scale(s.ad_value(701), p.p1235), p.p1234), A::scale(s.ad_value(702), p.p1236)), A::scale(s.ad_value(703), p.p1237));

        s.store_add_ad(849, A::add(A::offset(A::scale(s.ad_value(701), p.p1272), p.p1265), A::scale(s.ad_value(702), p.p1273)), A::scale(s.ad_value(703), p.p1274));

        s.store_add_ad(850, A::add(A::offset(A::scale(s.ad_value(701), p.p1276), p.p1275), A::scale(s.ad_value(702), p.p1277)), A::scale(s.ad_value(703), p.p1278));

        s.store_add_ad(854, A::add(A::offset(A::scale(s.ad_value(701), p.p1284), p.p1283), A::scale(s.ad_value(702), p.p1285)), A::scale(s.ad_value(703), p.p1286));

        s.store_add_ad(855, A::add(A::offset(A::scale(s.ad_value(701), p.p1280), p.p1279), A::scale(s.ad_value(702), p.p1281)), A::scale(s.ad_value(703), p.p1282));

        s.store_add_ad(851, A::add(A::offset(A::scale(s.ad_value(701), p.p1288), p.p1287), A::scale(s.ad_value(702), p.p1289)), A::scale(s.ad_value(703), p.p1290));

        s.store_add_ad(852, A::add(A::offset(A::scale(s.ad_value(701), p.p1292), p.p1291), A::scale(s.ad_value(702), p.p1293)), A::scale(s.ad_value(703), p.p1294));

        s.store_add_ad(856, A::add(A::offset(A::scale(s.ad_value(701), p.p1324), p.p1323), A::scale(s.ad_value(702), p.p1325)), A::scale(s.ad_value(703), p.p1326));

        s.store_add_ad(857, A::add(A::offset(A::scale(s.ad_value(701), p.p1328), p.p1327), A::scale(s.ad_value(702), p.p1329)), A::scale(s.ad_value(703), p.p1330));

        s.store_add_ad(859, A::add(A::offset(A::scale(s.ad_value(701), p.p1332), p.p1331), A::scale(s.ad_value(702), p.p1333)), A::scale(s.ad_value(703), p.p1334));

        s.store_add_ad(860, A::add(A::offset(A::scale(s.ad_value(701), p.p1336), p.p1335), A::scale(s.ad_value(702), p.p1337)), A::scale(s.ad_value(703), p.p1338));

        s.store_add_ad(862, A::add(A::offset(A::scale(s.ad_value(701), p.p1340), p.p1339), A::scale(s.ad_value(702), p.p1341)), A::scale(s.ad_value(703), p.p1342));

        s.store_add_ad(863, A::add(A::offset(A::scale(s.ad_value(701), p.p1344), p.p1343), A::scale(s.ad_value(702), p.p1345)), A::scale(s.ad_value(703), p.p1346));

        s.store_add_ad(888, A::add(A::offset(A::scale(s.ad_value(701), p.p787), p.p783), A::scale(s.ad_value(702), p.p791)), A::scale(s.ad_value(703), p.p795));

        s.store_add_ad(891, A::add(A::offset(A::scale(s.ad_value(701), p.p788), p.p784), A::scale(s.ad_value(702), p.p792)), A::scale(s.ad_value(703), p.p796));

        s.store_add_ad(889, A::add(A::offset(A::scale(s.ad_value(701), p.p789), p.p785), A::scale(s.ad_value(702), p.p793)), A::scale(s.ad_value(703), p.p797));

        s.store_add_ad(890, A::add(A::offset(A::scale(s.ad_value(701), p.p790), p.p786), A::scale(s.ad_value(702), p.p794)), A::scale(s.ad_value(703), p.p798));

        s.store_add_ad(908, A::add(A::offset(A::scale(s.ad_value(701), p.p1385), p.p1384), A::scale(s.ad_value(702), p.p1386)), A::scale(s.ad_value(703), p.p1387));

        s.store_add_ad(909, A::add(A::offset(A::scale(s.ad_value(701), p.p1390), p.p1389), A::scale(s.ad_value(702), p.p1391)), A::scale(s.ad_value(703), p.p1392));

        s.v[1149] = if (p.p35 != 0.0) { 1.0 } else { 0.0 };

        if (s.v[1149] != 0.0) {
            s.store_add_ad(839, A::add(A::offset(A::scale(s.ad_value(701), p.p1173), p.p1172), A::scale(s.ad_value(702), p.p1174)), A::scale(s.ad_value(703), p.p1175));
        }

        if (s.v[1149] != 0.0) {
            s.store_add_ad(717, A::add(A::offset(A::scale(s.ad_value(701), p.p285), p.p284), A::scale(s.ad_value(702), p.p286)), A::scale(s.ad_value(703), p.p287));
        }

        if (s.v[1149] != 0.0) {
            s.store_add_ad(731, A::add(A::offset(A::scale(s.ad_value(701), p.p199), p.p198), A::scale(s.ad_value(702), p.p200)), A::scale(s.ad_value(703), p.p201));
        }

        if (s.v[1149] != 0.0) {
            s.store_add_ad(739, A::add(A::offset(A::scale(s.ad_value(701), p.p344), p.p343), A::scale(s.ad_value(702), p.p345)), A::scale(s.ad_value(703), p.p346));
        }

        if (s.v[1149] != 0.0) {
            s.store_add_ad(749, A::add(A::offset(A::scale(s.ad_value(701), p.p359), p.p358), A::scale(s.ad_value(702), p.p360)), A::scale(s.ad_value(703), p.p361));
        }

        if (s.v[1149] != 0.0) {
            s.store_add_ad(753, A::add(A::offset(A::scale(s.ad_value(701), p.p379), p.p378), A::scale(s.ad_value(702), p.p380)), A::scale(s.ad_value(703), p.p381));
        }

        if (s.v[1149] != 0.0) {
            s.store_add_ad(756, A::add(A::offset(A::scale(s.ad_value(701), p.p387), p.p386), A::scale(s.ad_value(702), p.p388)), A::scale(s.ad_value(703), p.p389));
        }

        if (s.v[1149] != 0.0) {
            s.store_add_ad(759, A::add(A::offset(A::scale(s.ad_value(701), p.p401), p.p400), A::scale(s.ad_value(702), p.p402)), A::scale(s.ad_value(703), p.p403));
        }

        if (s.v[1149] != 0.0) {
            s.store_add_ad(784, A::add(A::offset(A::scale(s.ad_value(701), p.p411), p.p410), A::scale(s.ad_value(702), p.p412)), A::scale(s.ad_value(703), p.p413));
        }

        if (s.v[1149] != 0.0) {
            s.store_add_ad(762, A::add(A::offset(A::scale(s.ad_value(701), p.p537), p.p536), A::scale(s.ad_value(702), p.p538)), A::scale(s.ad_value(703), p.p539));
        }

        if (s.v[1149] != 0.0) {
            s.store_add_ad(745, A::add(A::offset(A::scale(s.ad_value(701), p.p306), p.p305), A::scale(s.ad_value(702), p.p307)), A::scale(s.ad_value(703), p.p308));
        }

        if (s.v[1149] != 0.0) {
            s.store_add_ad(347, A::add(A::offset(A::scale(s.ad_value(701), p.p491), p.p490), A::scale(s.ad_value(702), p.p492)), A::scale(s.ad_value(703), p.p493));
        }

        if (s.v[1149] != 0.0) {
            s.store_add_ad(779, A::add(A::offset(A::scale(s.ad_value(701), p.p507), p.p506), A::scale(s.ad_value(702), p.p508)), A::scale(s.ad_value(703), p.p509));
        }

        s.v[167] = ((p.p80 * ((((s.v[694]) as f64).powf(p.p81) - ((s.v[699]) as f64).powf(p.p81))).max(0.0)) + (p.p82 * ((((s.v[694]) as f64).powf(p.p83) - ((s.v[699]) as f64).powf(p.p83))).max(0.0)));

        s.v[168] = ((p.p84 * ((((s.v[695]) as f64).powf(p.p85) - ((s.v[700]) as f64).powf(p.p85))).max(0.0)) + (p.p86 * (((s.v[695] * s.v[694])) as f64).powf(p.p87)));

        s.store_scale(706, 706, ((1.0 + s.v[167]) + s.v[168]));

        s.v[167] = (p.p237 * ((((s.v[694]) as f64).powf(p.p238) - ((s.v[699]) as f64).powf(p.p238))).max(0.0));

        s.v[168] = ((p.p239 * ((((s.v[695]) as f64).powf(p.p240) - ((s.v[700]) as f64).powf(p.p240))).max(0.0)) + (p.p241 * ((s.v[696]) as f64).powf(p.p242)));

        s.store_scale(720, 720, ((1.0 + s.v[167]) + s.v[168]));

        s.v[167] = (1.0 + (p.p282 * ((((s.v[694]) as f64).powf(p.p283) - ((s.v[699]) as f64).powf(p.p283))).max(0.0)));

        s.store_scale(710, 710, s.v[167]);

        s.v[1150] = if (p.p35 != 0.0) { 1.0 } else { 0.0 };

        if (s.v[1150] != 0.0) {
            s.store_scale(839, 839, s.v[167]);
        }

        if (s.v[1150] != 0.0) {
            s.store_scale(717, 717, s.v[167]);
        }

        s.store_scale(719, 719, (1.0 + (p.p289 * ((((s.v[694]) as f64).powf(p.p290) - ((s.v[699]) as f64).powf(p.p290))).max(0.0))));

        s.store_scale(738, 738, p.p24);

        s.v[1151] = if (p.p42 != 1.0) { 1.0 } else { 0.0 };

        s.v[1152] = if (p.p339 > 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1151] != 0.0) && (s.v[1152] != 0.0)) {
            s.store_scale(738, 738, (1.0 - (p.p338 * ((((s.v[694]) as f64).powf(p.p339) - ((s.v[699]) as f64).powf(p.p339))).max(0.0))));
        }

        s.v[1153] = if (p.p35 != 0.0) { 1.0 } else { 0.0 };

        if (((s.v[1151] != 0.0) && (s.v[1152] != 0.0)) && (s.v[1153] != 0.0)) {
            s.store_scale(739, 739, (1.0 - (p.p338 * ((((s.v[694]) as f64).powf(p.p339) - ((s.v[699]) as f64).powf(p.p339))).max(0.0))));
        }

        if ((s.v[1151] != 0.0) && (!(s.v[1152] != 0.0))) {
            s.store_scale(738, 738, (1.0 - p.p338));
        }

        s.v[1154] = if (p.p35 != 0.0) { 1.0 } else { 0.0 };

        if (((s.v[1151] != 0.0) && (!(s.v[1152] != 0.0))) && (s.v[1154] != 0.0)) {
            s.store_scale(739, 739, (1.0 - p.p338));
        }

        if (!(s.v[1151] != 0.0)) {
            let assign4590_ad_e6159: A = A::scale(s.ad_value(738), ((1.0 - (p.p333 * if ((-s.v[184]) / p.p334) > 80.0 { 5.540622384e34 * (1.0 + (((-s.v[184]) / p.p334)) - 80.0) } else if ((-s.v[184]) / p.p334) < -80.0 { 1.804851387e-35 } else { ((((-s.v[184]) / p.p334)) as f64).exp() })) - (p.p335 * if ((-s.v[184]) / p.p336) > 80.0 { 5.540622384e34 * (1.0 + (((-s.v[184]) / p.p336)) - 80.0) } else if ((-s.v[184]) / p.p336) < -80.0 { 1.804851387e-35 } else { ((((-s.v[184]) / p.p336)) as f64).exp() })));
            s.store_ad(738, &assign4590_ad_e6159);
        }

        s.v[1155] = if (p.p35 != 0.0) { 1.0 } else { 0.0 };

        if ((!(s.v[1151] != 0.0)) && (s.v[1155] != 0.0)) {
            let assign4610_ad_e6187: A = A::scale(s.ad_value(739), ((1.0 - (p.p333 * if ((-s.v[184]) / p.p334) > 80.0 { 5.540622384e34 * (1.0 + (((-s.v[184]) / p.p334)) - 80.0) } else if ((-s.v[184]) / p.p334) < -80.0 { 1.804851387e-35 } else { ((((-s.v[184]) / p.p334)) as f64).exp() })) - (p.p335 * if ((-s.v[184]) / p.p336) > 80.0 { 5.540622384e34 * (1.0 + (((-s.v[184]) / p.p336)) - 80.0) } else if ((-s.v[184]) / p.p336) < -80.0 { 1.804851387e-35 } else { ((((-s.v[184]) / p.p336)) as f64).exp() })));
            s.store_ad(739, &assign4610_ad_e6187);
        }

        s.v[167] = (p.p349 * ((((s.v[694]) as f64).powf(p.p350) - ((s.v[699]) as f64).powf(p.p350))).max(0.0));

        s.v[168] = ((p.p351 * ((((s.v[695]) as f64).powf(p.p352) - ((s.v[700]) as f64).powf(p.p352))).max(0.0)) + (p.p353 * ((s.v[696]) as f64).powf(p.p354)));

        s.store_scale(748, 748, ((1.0 + s.v[167]) + s.v[168]));

        s.v[1156] = if (p.p35 != 0.0) { 1.0 } else { 0.0 };

        if (s.v[1156] != 0.0) {
            s.store_scale(749, 749, ((1.0 + s.v[167]) + s.v[168]));
        }

        s.v[167] = (p.p366 * ((((s.v[694]) as f64).powf(p.p367) - ((s.v[699]) as f64).powf(p.p367))).max(0.0));

        s.v[168] = ((p.p368 * ((((s.v[695]) as f64).powf(p.p369) - ((s.v[700]) as f64).powf(p.p369))).max(0.0)) + (p.p370 * ((s.v[696]) as f64).powf(p.p371)));

        s.store_scale(751, 751, ((1.0 + s.v[167]) + s.v[168]));

        s.v[167] = (1.0 + (p.p373 * ((((s.v[694]) as f64).powf(p.p374) - ((s.v[699]) as f64).powf(p.p374))).max(0.0)));

        s.store_scale(752, 752, s.v[167]);

        s.v[1157] = if (p.p35 != 0.0) { 1.0 } else { 0.0 };

        if (s.v[1157] != 0.0) {
            s.store_scale(753, 753, s.v[167]);
        }

        s.v[167] = (p.p391 * ((((s.v[694]) as f64).powf(p.p392) - ((s.v[699]) as f64).powf(p.p392))).max(0.0));

        s.v[168] = ((p.p393 * ((((s.v[695]) as f64).powf(p.p394) - ((s.v[700]) as f64).powf(p.p394))).max(0.0)) + (p.p395 * ((s.v[696]) as f64).powf(p.p396)));

        s.store_scale(758, 758, ((1.0 + s.v[167]) + s.v[168]));

        s.v[1158] = if (p.p35 != 0.0) { 1.0 } else { 0.0 };

        if (s.v[1158] != 0.0) {
            s.store_scale(759, 759, ((1.0 + s.v[167]) + s.v[168]));
        }

        s.v[167] = ((((s.v[694]) as f64).powf(p.p202) - ((s.v[699]) as f64).powf(p.p202))).max(0.0);

        s.store_scale(730, 730, s.v[167]);

        s.v[1159] = if (p.p35 != 0.0) { 1.0 } else { 0.0 };

        if (s.v[1159] != 0.0) {
            s.store_scale(731, 731, s.v[167]);
        }

        s.store_scale(733, 733, ((((s.v[694]) as f64).powf(p.p204) - ((s.v[699]) as f64).powf(p.p204))).max(0.0));

        s.v[167] = (1.0 + (p.p531 * ((((s.v[694]) as f64).powf(p.p532) - ((s.v[699]) as f64).powf(p.p532))).max(0.0)));

        s.store_scale(761, 761, s.v[167]);

        s.v[1160] = if (p.p35 != 0.0) { 1.0 } else { 0.0 };

        if (s.v[1160] != 0.0) {
            s.store_scale(762, 762, s.v[167]);
        }

        s.store_scale(167, 737, (1.0 + (p.p313 * ((((s.v[694]) as f64).powf(p.p314) - ((s.v[699]) as f64).powf(p.p314))).max(0.0))));

        s.store_ad(737, &A::min_with_scalar(s.ad_value(167), 0.5));

        s.store_scale(769, 769, (1.0 + (p.p549 * ((((s.v[694]) as f64).powf(p.p550) - ((s.v[699]) as f64).powf(p.p550))).max(0.0))));

        s.v[167] = (1.0 + (p.p405 * ((((s.v[694]) as f64).powf(p.p406) - ((s.v[699]) as f64).powf(p.p406))).max(0.0)));

        s.store_scale(783, 783, s.v[167]);

        s.store_ad(783, &A::max_with_scalar(s.ad_value(783), 0.0));

        s.v[1161] = if (p.p35 != 0.0) { 1.0 } else { 0.0 };

        if (s.v[1161] != 0.0) {
            s.store_scale(784, 784, s.v[167]);
        }

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
        if (s.v[1161] != 0.0) {
            s.store_ad(784, &A::max_with_scalar(s.ad_value(784), 0.0));
        }

        s.v[167] = (p.p299 * ((((s.v[694]) as f64).powf(p.p300) - ((s.v[699]) as f64).powf(p.p300))).max(0.0));

        s.v[168] = ((p.p301 * ((((s.v[695]) as f64).powf(p.p302) - ((s.v[700]) as f64).powf(p.p302))).max(0.0)) + (p.p303 * ((s.v[696]) as f64).powf(p.p304)));

        s.store_scale(741, 741, ((1.0 + s.v[167]) + s.v[168]));

        s.v[1162] = if (p.p35 != 0.0) { 1.0 } else { 0.0 };

        if (s.v[1162] != 0.0) {
            s.store_scale(745, 745, ((1.0 + s.v[167]) + s.v[168]));
        }

        s.store_max_with_scalar_ad(346, A::scale(s.ad_value(346), (1.0 + (p.p487 * ((((s.v[694]) as f64).powf(p.p488) - ((s.v[699]) as f64).powf(p.p488))).max(0.0)))), 0.25);

        s.v[1163] = if (p.p35 != 0.0) { 1.0 } else { 0.0 };

        if (s.v[1163] != 0.0) {
            s.store_max_with_scalar_ad(347, A::scale(s.ad_value(347), (1.0 + (p.p487 * ((((s.v[694]) as f64).powf(p.p488) - ((s.v[699]) as f64).powf(p.p488))).max(0.0)))), 0.25);
        }

        s.v[167] = (1.0 + (p.p502 * ((((s.v[694]) as f64).powf(p.p505) - ((s.v[699]) as f64).powf(p.p505))).max(0.0)));

        s.store_scale(778, 778, s.v[167]);

        s.v[1164] = if (p.p35 != 0.0) { 1.0 } else { 0.0 };

        if (s.v[1164] != 0.0) {
            s.store_scale(779, 779, s.v[167]);
        }

        s.store_scale(865, 865, (1.0 + (p.p602 * ((((s.v[694]) as f64).powf(p.p603) - ((s.v[699]) as f64).powf(p.p603))).max(0.0))));

        s.store_scale(892, 892, ((1.0 + (p.p800 * s.v[694])) + (p.p801 * s.v[695])));

        s.store_scale(896, 896, ((1.0 + (p.p822 * s.v[694])) + (p.p823 * s.v[695])));

        s.store_scale(810, 810, ((1.0 + (p.p724 * s.v[694])) + (p.p725 * s.v[695])));

        s.store_scale(816, 816, ((1.0 + (p.p727 * s.v[694])) + (p.p728 * s.v[695])));

        s.store_scale(819, 819, ((1.0 + (p.p729 * s.v[694])) + (p.p730 * s.v[695])));

        s.v[823] = (p.p723 * (1.0 + (p.p731 * s.v[694])));

        s.v[167] = ((p.p92 * ((((s.v[697]) as f64).powf(p.p93) - ((s.v[699]) as f64).powf(p.p93))).max(0.0)) + (p.p94 * ((((s.v[697]) as f64).powf(p.p95) - ((s.v[699]) as f64).powf(p.p95))).max(0.0)));

        s.v[168] = ((p.p96 * ((((s.v[698]) as f64).powf(p.p97) - ((s.v[700]) as f64).powf(p.p97))).max(0.0)) + (p.p98 * (((s.v[698] * s.v[697])) as f64).powf(p.p99)));

        s.store_scale(794, 794, ((1.0 + s.v[167]) + s.v[168]));

        s.v[1165] = if (p.p29 == 1.0) { 1.0 } else { 0.0 };

        if (s.v[1165] != 0.0) {
            s.copy_ad(794, 706);
        }

        if (!(s.v[1165] != 0.0)) {
        }

        s.v[167] = (p.p123 * ((((s.v[694]) as f64).powf(p.p124) - ((s.v[699]) as f64).powf(p.p124))).max(0.0));

        s.v[168] = ((p.p125 * ((((s.v[695]) as f64).powf(p.p126) - ((s.v[700]) as f64).powf(p.p126))).max(0.0)) + (p.p127 * ((s.v[696]) as f64).powf(p.p128)));

        s.store_scale(707, 707, ((1.0 + s.v[167]) + s.v[168]));

        s.v[167] = (p.p133 * ((((s.v[697]) as f64).powf(p.p134) - ((s.v[699]) as f64).powf(p.p134))).max(0.0));

        s.v[168] = ((p.p135 * ((((s.v[698]) as f64).powf(p.p136) - ((s.v[700]) as f64).powf(p.p136))).max(0.0)) + (p.p137 * (((s.v[698] * s.v[697])) as f64).powf(p.p138)));

        s.store_scale(793, 793, ((1.0 + s.v[167]) + s.v[168]));

        s.v[167] = (p.p319 * ((((s.v[697]) as f64).powf(p.p320) - ((s.v[699]) as f64).powf(p.p320))).max(0.0));

        s.v[168] = ((p.p321 * ((((s.v[698]) as f64).powf(p.p322) - ((s.v[700]) as f64).powf(p.p322))).max(0.0)) + (p.p323 * (((s.v[698] * s.v[697])) as f64).powf(p.p324)));

        s.store_scale(747, 747, ((1.0 + s.v[167]) + s.v[168]));

        s.store_scale(786, 786, (1.0 + (p.p416 * ((((s.v[697]) as f64).powf(p.p417) - ((s.v[699]) as f64).powf(p.p417))).max(0.0))));

        s.store_ad(786, &A::max_with_scalar(s.ad_value(786), 0.0));

        s.v[167] = (p.p209 * ((((s.v[694]) as f64).powf(p.p210) - ((s.v[699]) as f64).powf(p.p210))).max(0.0));

        s.v[168] = ((p.p211 * ((((s.v[695]) as f64).powf(p.p212) - ((s.v[700]) as f64).powf(p.p212))).max(0.0)) + (p.p213 * ((s.v[696]) as f64).powf(p.p214)));

        s.store_scale(735, 735, ((1.0 + s.v[167]) + s.v[168]));

        s.v[167] = (p.p1197 * ((((s.v[694]) as f64).powf(p.p1198) - ((s.v[699]) as f64).powf(p.p1198))).max(0.0));

        s.v[168] = ((p.p1199 * ((((s.v[695]) as f64).powf(p.p1200) - ((s.v[700]) as f64).powf(p.p1200))).max(0.0)) + (p.p1201 * ((s.v[696]) as f64).powf(p.p1202)));

        s.store_scale(736, 736, ((1.0 + s.v[167]) + s.v[168]));

        s.v[167] = (p.p219 * ((((s.v[694]) as f64).powf(p.p220) - ((s.v[699]) as f64).powf(p.p220))).max(0.0));

        s.v[168] = ((p.p221 * ((((s.v[695]) as f64).powf(p.p222) - ((s.v[700]) as f64).powf(p.p222))).max(0.0)) + (p.p223 * ((s.v[696]) as f64).powf(p.p224)));

        s.store_scale(734, 734, ((1.0 + s.v[167]) + s.v[168]));

        s.v[167] = (p.p1266 * ((((s.v[694]) as f64).powf(p.p1267) - ((s.v[699]) as f64).powf(p.p1267))).max(0.0));

        s.v[168] = ((p.p1268 * ((((s.v[695]) as f64).powf(p.p1269) - ((s.v[700]) as f64).powf(p.p1269))).max(0.0)) + (p.p1270 * ((s.v[696]) as f64).powf(p.p1271)));

        s.store_scale(849, 849, ((1.0 + s.v[167]) + s.v[168]));

        s.store_scale(787, 787, (1.0 + (p.p447 * ((((s.v[694]) as f64).powf(p.p448) - ((s.v[699]) as f64).powf(p.p448))).max(0.0))));

        s.store_scale(796, 796, (1.0 + (s.v[694] * p.p1036)));

        s.store_scale(797, 797, (1.0 + (s.v[694] * p.p1041)));

        s.store_scale(799, 799, (1.0 + (s.v[694] * p.p1050)));

        s.store_scale(802, 802, (1.0 + (s.v[694] * p.p1068)));

        s.store_scale(803, 803, (1.0 + (s.v[694] * p.p1074)));

        s.v[1166] = if (p.p33 == 1.0) { 1.0 } else { 0.0 };

        if (s.v[1166] != 0.0) {
            s.store_scale(775, 775, (1.0 + (p.p461 * ((((s.v[694]) as f64).powf(p.p462) - ((s.v[699]) as f64).powf(p.p462))).max(0.0))));
        }

        if (s.v[1166] != 0.0) {
            s.store_scale(774, 774, (1.0 + (p.p471 * ((((s.v[694]) as f64).powf(p.p472) - ((s.v[699]) as f64).powf(p.p472))).max(0.0))));
        }

        if (!(s.v[1166] != 0.0)) {
            s.store_scale(776, 776, (1.0 + (p.p478 * ((((s.v[694]) as f64).powf(p.p479) - ((s.v[699]) as f64).powf(p.p479))).max(0.0))));
        }

        s.v[1167] = if (s.v[755] < 1.0) { 1.0 } else { 0.0 };

        if (s.v[1167] != 0.0) {
            s.store_scalar(755, 1.0);
        }

        s.v[1168] = if (s.v[755] > 2.0) { 1.0 } else { 0.0 };

        if ((!(s.v[1167] != 0.0)) && (s.v[1168] != 0.0)) {
            s.store_scalar(755, 2.0);
        }

        s.v[1169] = if (p.p35 != 0.0) { 1.0 } else { 0.0 };

        s.v[1170] = if (s.v[756] < 1.0) { 1.0 } else { 0.0 };

        if ((s.v[1169] != 0.0) && (s.v[1170] != 0.0)) {
            s.store_scalar(756, 1.0);
        }

        s.v[1171] = if (s.v[756] > 2.0) { 1.0 } else { 0.0 };

        if (((s.v[1169] != 0.0) && (!(s.v[1170] != 0.0))) && (s.v[1171] != 0.0)) {
            s.store_scalar(756, 2.0);
        }

        s.v[1196] = if (s.v[829] < 0.0) { 1.0 } else { 0.0 };

        if (s.v[1196] != 0.0) {
            s.store_scalar(829, 0.0);
        }

        s.v[1197] = if (s.v[738] <= 0.0) { 1.0 } else { 0.0 };

        if (s.v[1197] != 0.0) {
            s.store_scalar(738, 0.067);
        }

        s.v[1198] = if (s.v[748] < 0.0) { 1.0 } else { 0.0 };

        if (s.v[1198] != 0.0) {
            s.store_scalar(748, 0.0);
        }

        s.v[1199] = if (s.v[751] < 0.0) { 1.0 } else { 0.0 };

        if (s.v[1199] != 0.0) {
            s.store_scalar(751, 0.0);
        }

        s.v[1200] = if (s.v[752] < 0.0) { 1.0 } else { 0.0 };

        if (s.v[1200] != 0.0) {
            s.store_scalar(752, 0.0);
        }

        s.v[1201] = if (s.v[755] < 0.0) { 1.0 } else { 0.0 };

        if (s.v[1201] != 0.0) {
            s.store_scalar(755, 0.0);
        }

        s.v[1202] = if (s.v[590] <= 0.0) { 1.0 } else { 0.0 };

        if (s.v[1202] != 0.0) {
            s.store_scalar(590, 1.0);
        }

        s.v[1203] = if (s.v[564] <= 0.0) { 1.0 } else { 0.0 };

        if (s.v[1203] != 0.0) {
            s.store_scalar(564, 10.0);
        }

        s.v[1204] = if (s.v[557] <= 0.0) { 1.0 } else { 0.0 };

        if (s.v[1204] != 0.0) {
            s.store_scalar(557, 2.0);
        }

        s.v[969] = 0.0;

        s.v[971] = 0.0;

        s.v[968] = 0.0;

        s.v[970] = 0.0;

        s.v[973] = 0.0;

        s.v[972] = 0.0;

        s.v[449] = (p.p895 - p.p898);

        s.v[451] = p.p896;

        s.v[450] = (p.p897 - p.p898);

        s.v[1206] = if self.param_given[3] { 1.0 } else { 0.0 };

        if (s.v[1206] != 0.0) {
            s.store_scalar(452, (p.p438 * p.p3));
        }

        s.v[1207] = if ((p.p9 > 0.0) && (p.p438 > 0.0)) { 1.0 } else { 0.0 };

        s.v[1208] = if (p.p8 < 9.0) { 1.0 } else { 0.0 };

        s.v[1209] = if ((p.p2 % 2.0) != 0.0) { 1.0 } else { 0.0 };

        if ((((!(s.v[1206] != 0.0)) && (s.v[1207] != 0.0)) && (s.v[1208] != 0.0)) && (s.v[1209] != 0.0)) {
            s.store_scalar(969, 1.0);
        }

        if ((((!(s.v[1206] != 0.0)) && (s.v[1207] != 0.0)) && (s.v[1208] != 0.0)) && (s.v[1209] != 0.0)) {
            s.store_scalar(971, 1.0);
        }

        if ((((!(s.v[1206] != 0.0)) && (s.v[1207] != 0.0)) && (s.v[1208] != 0.0)) && (s.v[1209] != 0.0)) {
            s.store_scalar(968, (2.0 * (((p.p2 - 1.0) / 2.0)).max(0.0)));
        }

        if ((((!(s.v[1206] != 0.0)) && (s.v[1207] != 0.0)) && (s.v[1208] != 0.0)) && (s.v[1209] != 0.0)) {
            s.copy_ad(970, 968);
        }

        s.v[1210] = if (p.p6 == 1.0) { 1.0 } else { 0.0 };

        if (((((!(s.v[1206] != 0.0)) && (s.v[1207] != 0.0)) && (s.v[1208] != 0.0)) && (!(s.v[1209] != 0.0))) && (s.v[1210] != 0.0)) {
            s.store_scalar(969, 2.0);
        }

        if (((((!(s.v[1206] != 0.0)) && (s.v[1207] != 0.0)) && (s.v[1208] != 0.0)) && (!(s.v[1209] != 0.0))) && (s.v[1210] != 0.0)) {
            s.store_scalar(968, (2.0 * (((p.p2 / 2.0) - 1.0)).max(0.0)));
        }

        if (((((!(s.v[1206] != 0.0)) && (s.v[1207] != 0.0)) && (s.v[1208] != 0.0)) && (!(s.v[1209] != 0.0))) && (s.v[1210] != 0.0)) {
            s.store_scalar(971, 0.0);
        }

        if (((((!(s.v[1206] != 0.0)) && (s.v[1207] != 0.0)) && (s.v[1208] != 0.0)) && (!(s.v[1209] != 0.0))) && (s.v[1210] != 0.0)) {
            s.store_scalar(970, p.p2);
        }

        if (((((!(s.v[1206] != 0.0)) && (s.v[1207] != 0.0)) && (s.v[1208] != 0.0)) && (!(s.v[1209] != 0.0))) && (!(s.v[1210] != 0.0))) {
            s.store_scalar(969, 0.0);
        }

        if (((((!(s.v[1206] != 0.0)) && (s.v[1207] != 0.0)) && (s.v[1208] != 0.0)) && (!(s.v[1209] != 0.0))) && (!(s.v[1210] != 0.0))) {
            s.store_scalar(968, p.p2);
        }

        if (((((!(s.v[1206] != 0.0)) && (s.v[1207] != 0.0)) && (s.v[1208] != 0.0)) && (!(s.v[1209] != 0.0))) && (!(s.v[1210] != 0.0))) {
            s.store_scalar(971, 2.0);
        }

        if (((((!(s.v[1206] != 0.0)) && (s.v[1207] != 0.0)) && (s.v[1208] != 0.0)) && (!(s.v[1209] != 0.0))) && (!(s.v[1210] != 0.0))) {
            s.store_scalar(970, (2.0 * (((p.p2 / 2.0) - 1.0)).max(0.0)));
        }

        s.v[1211] = if (1.0 == 1.0) { 1.0 } else { 0.0 };

        s.v[1212] = if (s.v[970] == 0.0) { 1.0 } else { 0.0 };

        if (((((!(s.v[1206] != 0.0)) && (s.v[1207] != 0.0)) && (s.v[1208] != 0.0)) && (s.v[1211] != 0.0)) && (s.v[1212] != 0.0)) {
            s.store_scalar(972, 0.0);
        }

        if (((((!(s.v[1206] != 0.0)) && (s.v[1207] != 0.0)) && (s.v[1208] != 0.0)) && (s.v[1211] != 0.0)) && (!(s.v[1212] != 0.0))) {
            s.store_div_from_scalar_ad(972, (p.p438 * s.v[449]), A::scale(s.ad_value(970), s.v[183]));
        }

        s.v[1213] = if (s.v[968] == 0.0) { 1.0 } else { 0.0 };

        if (((((!(s.v[1206] != 0.0)) && (s.v[1207] != 0.0)) && (s.v[1208] != 0.0)) && (!(s.v[1211] != 0.0))) && (s.v[1213] != 0.0)) {
            s.store_scalar(972, 0.0);
        }

        if (((((!(s.v[1206] != 0.0)) && (s.v[1207] != 0.0)) && (s.v[1208] != 0.0)) && (!(s.v[1211] != 0.0))) && (!(s.v[1213] != 0.0))) {
            s.store_div_from_scalar_ad(972, (p.p438 * s.v[449]), A::scale(s.ad_value(968), s.v[183]));
        }

        s.v[1214] = if (p.p8 == 0.0) { 1.0 } else { 0.0 };

        s.v[1215] = if (p.p8 == 1.0) { 1.0 } else { 0.0 };

        s.v[1216] = if (p.p8 == 2.0) { 1.0 } else { 0.0 };

        s.v[1217] = if (p.p8 == 3.0) { 1.0 } else { 0.0 };

        s.v[1218] = if (p.p8 == 4.0) { 1.0 } else { 0.0 };

        s.v[1219] = if (p.p8 == 5.0) { 1.0 } else { 0.0 };

        s.v[1220] = if (p.p8 == 6.0) { 1.0 } else { 0.0 };

        s.v[1221] = if (p.p8 == 7.0) { 1.0 } else { 0.0 };

        s.v[1222] = if (p.p8 == 8.0) { 1.0 } else { 0.0 };

        s.v[1223] = if (p.p8 == 9.0) { 1.0 } else { 0.0 };

        s.v[1224] = if (p.p8 == 10.0) { 1.0 } else { 0.0 };

        s.v[1225] = if (1.0 == 1.0) { 1.0 } else { 0.0 };

        s.v[1226] = if (1.0 == 1.0) { 1.0 } else { 0.0 };

        s.v[1227] = if (((p.p9 == 1.0) || (p.p9 == 2.0)) || (p.p9 == 5.0)) { 1.0 } else { 0.0 };

        s.v[1228] = if (((p.p9 == 3.0) || (p.p9 == 4.0)) || (p.p9 == 6.0)) { 1.0 } else { 0.0 };

        s.v[1229] = if (s.v[971] == 0.0) { 1.0 } else { 0.0 };

        if (((((((!(s.v[1206] != 0.0)) && (s.v[1207] != 0.0)) && (s.v[1214] != 0.0)) && (s.v[1225] != 0.0)) && (s.v[1226] != 0.0)) && (s.v[1227] != 0.0)) && (s.v[1229] != 0.0)) {
            s.store_scalar(973, 0.0);
        }

        if (((((((!(s.v[1206] != 0.0)) && (s.v[1207] != 0.0)) && (s.v[1214] != 0.0)) && (s.v[1225] != 0.0)) && (s.v[1226] != 0.0)) && (s.v[1227] != 0.0)) && (!(s.v[1229] != 0.0))) {
            s.store_div_from_scalar_ad(973, (p.p438 * s.v[449]), A::scale(s.ad_value(971), s.v[183]));
        }

        s.v[1231] = if ((s.v[971] == 0.0) || ((s.v[449] + s.v[451]) == 0.0)) { 1.0 } else { 0.0 };

        if (((((((!(s.v[1206] != 0.0)) && (s.v[1207] != 0.0)) && (s.v[1214] != 0.0)) && (s.v[1225] != 0.0)) && (s.v[1226] != 0.0)) && ((s.v[1228] != 0.0) && (!(s.v[1227] != 0.0)))) && (s.v[1231] != 0.0)) {
            s.store_scalar(973, 0.0);
        }

        if (((((((!(s.v[1206] != 0.0)) && (s.v[1207] != 0.0)) && (s.v[1214] != 0.0)) && (s.v[1225] != 0.0)) && (s.v[1226] != 0.0)) && ((s.v[1228] != 0.0) && (!(s.v[1227] != 0.0)))) && (!(s.v[1231] != 0.0))) {
            s.store_div_from_scalar_ad(973, (p.p438 * s.v[183]), A::scale(s.ad_value(971), (3.0 * (s.v[449] + s.v[451]))));
        }

        if ((((((!(s.v[1206] != 0.0)) && (s.v[1207] != 0.0)) && (s.v[1214] != 0.0)) && (s.v[1225] != 0.0)) && (s.v[1226] != 0.0)) && (!((s.v[1227] != 0.0) || (s.v[1228] != 0.0)))) {
            s.store_scalar(973, 0.0);
        }

        s.v[1232] = if (((p.p9 == 1.0) || (p.p9 == 3.0)) || (p.p9 == 7.0)) { 1.0 } else { 0.0 };

        s.v[1233] = if (((p.p9 == 2.0) || (p.p9 == 4.0)) || (p.p9 == 8.0)) { 1.0 } else { 0.0 };

        s.v[1234] = if (s.v[971] == 0.0) { 1.0 } else { 0.0 };

        if (((((((!(s.v[1206] != 0.0)) && (s.v[1207] != 0.0)) && (s.v[1214] != 0.0)) && (s.v[1225] != 0.0)) && (!(s.v[1226] != 0.0))) && (s.v[1232] != 0.0)) && (s.v[1234] != 0.0)) {
            s.store_scalar(973, 0.0);
        }

        if (((((((!(s.v[1206] != 0.0)) && (s.v[1207] != 0.0)) && (s.v[1214] != 0.0)) && (s.v[1225] != 0.0)) && (!(s.v[1226] != 0.0))) && (s.v[1232] != 0.0)) && (!(s.v[1234] != 0.0))) {
            s.store_div_from_scalar_ad(973, (p.p438 * s.v[449]), A::scale(s.ad_value(971), s.v[183]));
        }

        s.v[1236] = if ((s.v[971] == 0.0) || ((s.v[449] + s.v[451]) == 0.0)) { 1.0 } else { 0.0 };

        if (((((((!(s.v[1206] != 0.0)) && (s.v[1207] != 0.0)) && (s.v[1214] != 0.0)) && (s.v[1225] != 0.0)) && (!(s.v[1226] != 0.0))) && ((s.v[1233] != 0.0) && (!(s.v[1232] != 0.0)))) && (s.v[1236] != 0.0)) {
            s.store_scalar(973, 0.0);
        }

        if (((((((!(s.v[1206] != 0.0)) && (s.v[1207] != 0.0)) && (s.v[1214] != 0.0)) && (s.v[1225] != 0.0)) && (!(s.v[1226] != 0.0))) && ((s.v[1233] != 0.0) && (!(s.v[1232] != 0.0)))) && (!(s.v[1236] != 0.0))) {
            s.store_div_from_scalar_ad(973, (p.p438 * s.v[183]), A::scale(s.ad_value(971), (3.0 * (s.v[449] + s.v[451]))));
        }

        if ((((((!(s.v[1206] != 0.0)) && (s.v[1207] != 0.0)) && (s.v[1214] != 0.0)) && (s.v[1225] != 0.0)) && (!(s.v[1226] != 0.0))) && (!((s.v[1232] != 0.0) || (s.v[1233] != 0.0)))) {
            s.store_scalar(973, 0.0);
        }

        s.v[1237] = if (0.0 == 1.0) { 1.0 } else { 0.0 };

        s.v[1238] = if (((p.p9 == 1.0) || (p.p9 == 2.0)) || (p.p9 == 5.0)) { 1.0 } else { 0.0 };

        s.v[1239] = if (((p.p9 == 3.0) || (p.p9 == 4.0)) || (p.p9 == 6.0)) { 1.0 } else { 0.0 };

        s.v[1240] = if (s.v[969] == 0.0) { 1.0 } else { 0.0 };

        if (((((((!(s.v[1206] != 0.0)) && (s.v[1207] != 0.0)) && (s.v[1214] != 0.0)) && (!(s.v[1225] != 0.0))) && (s.v[1237] != 0.0)) && (s.v[1238] != 0.0)) && (s.v[1240] != 0.0)) {
            s.store_scalar(973, 0.0);
        }

        if (((((((!(s.v[1206] != 0.0)) && (s.v[1207] != 0.0)) && (s.v[1214] != 0.0)) && (!(s.v[1225] != 0.0))) && (s.v[1237] != 0.0)) && (s.v[1238] != 0.0)) && (!(s.v[1240] != 0.0))) {
            s.store_div_from_scalar_ad(973, (p.p438 * s.v[449]), A::scale(s.ad_value(969), s.v[183]));
        }

        s.v[1242] = if ((s.v[969] == 0.0) || ((s.v[449] + s.v[451]) == 0.0)) { 1.0 } else { 0.0 };

        if (((((((!(s.v[1206] != 0.0)) && (s.v[1207] != 0.0)) && (s.v[1214] != 0.0)) && (!(s.v[1225] != 0.0))) && (s.v[1237] != 0.0)) && ((s.v[1239] != 0.0) && (!(s.v[1238] != 0.0)))) && (s.v[1242] != 0.0)) {
            s.store_scalar(973, 0.0);
        }

        if (((((((!(s.v[1206] != 0.0)) && (s.v[1207] != 0.0)) && (s.v[1214] != 0.0)) && (!(s.v[1225] != 0.0))) && (s.v[1237] != 0.0)) && ((s.v[1239] != 0.0) && (!(s.v[1238] != 0.0)))) && (!(s.v[1242] != 0.0))) {
            s.store_div_from_scalar_ad(973, (p.p438 * s.v[183]), A::scale(s.ad_value(969), (3.0 * (s.v[449] + s.v[451]))));
        }

        if ((((((!(s.v[1206] != 0.0)) && (s.v[1207] != 0.0)) && (s.v[1214] != 0.0)) && (!(s.v[1225] != 0.0))) && (s.v[1237] != 0.0)) && (!((s.v[1238] != 0.0) || (s.v[1239] != 0.0)))) {
            s.store_scalar(973, 0.0);
        }

        s.v[1243] = if (((p.p9 == 1.0) || (p.p9 == 3.0)) || (p.p9 == 7.0)) { 1.0 } else { 0.0 };

        s.v[1244] = if (((p.p9 == 2.0) || (p.p9 == 4.0)) || (p.p9 == 8.0)) { 1.0 } else { 0.0 };

        s.v[1245] = if (s.v[969] == 0.0) { 1.0 } else { 0.0 };

        if (((((((!(s.v[1206] != 0.0)) && (s.v[1207] != 0.0)) && (s.v[1214] != 0.0)) && (!(s.v[1225] != 0.0))) && (!(s.v[1237] != 0.0))) && (s.v[1243] != 0.0)) && (s.v[1245] != 0.0)) {
            s.store_scalar(973, 0.0);
        }

        if (((((((!(s.v[1206] != 0.0)) && (s.v[1207] != 0.0)) && (s.v[1214] != 0.0)) && (!(s.v[1225] != 0.0))) && (!(s.v[1237] != 0.0))) && (s.v[1243] != 0.0)) && (!(s.v[1245] != 0.0))) {
            s.store_div_from_scalar_ad(973, (p.p438 * s.v[449]), A::scale(s.ad_value(969), s.v[183]));
        }

        s.v[1247] = if ((s.v[969] == 0.0) || ((s.v[449] + s.v[451]) == 0.0)) { 1.0 } else { 0.0 };

        if (((((((!(s.v[1206] != 0.0)) && (s.v[1207] != 0.0)) && (s.v[1214] != 0.0)) && (!(s.v[1225] != 0.0))) && (!(s.v[1237] != 0.0))) && ((s.v[1244] != 0.0) && (!(s.v[1243] != 0.0)))) && (s.v[1247] != 0.0)) {
            s.store_scalar(973, 0.0);
        }

        if (((((((!(s.v[1206] != 0.0)) && (s.v[1207] != 0.0)) && (s.v[1214] != 0.0)) && (!(s.v[1225] != 0.0))) && (!(s.v[1237] != 0.0))) && ((s.v[1244] != 0.0) && (!(s.v[1243] != 0.0)))) && (!(s.v[1247] != 0.0))) {
            s.store_div_from_scalar_ad(973, (p.p438 * s.v[183]), A::scale(s.ad_value(969), (3.0 * (s.v[449] + s.v[451]))));
        }

        if ((((((!(s.v[1206] != 0.0)) && (s.v[1207] != 0.0)) && (s.v[1214] != 0.0)) && (!(s.v[1225] != 0.0))) && (!(s.v[1237] != 0.0))) && (!((s.v[1243] != 0.0) || (s.v[1244] != 0.0)))) {
            s.store_scalar(973, 0.0);
        }

        s.v[1248] = if (1.0 == 1.0) { 1.0 } else { 0.0 };

        s.v[1249] = if (1.0 == 1.0) { 1.0 } else { 0.0 };

        s.v[1250] = if (((p.p9 == 1.0) || (p.p9 == 2.0)) || (p.p9 == 5.0)) { 1.0 } else { 0.0 };

        s.v[1251] = if (((p.p9 == 3.0) || (p.p9 == 4.0)) || (p.p9 == 6.0)) { 1.0 } else { 0.0 };

        s.v[1252] = if (s.v[971] == 0.0) { 1.0 } else { 0.0 };

        if (((((((!(s.v[1206] != 0.0)) && (s.v[1207] != 0.0)) && ((s.v[1215] != 0.0) && (!(s.v[1214] != 0.0)))) && (s.v[1248] != 0.0)) && (s.v[1249] != 0.0)) && (s.v[1250] != 0.0)) && (s.v[1252] != 0.0)) {
            s.store_scalar(973, 0.0);
        }

        if (((((((!(s.v[1206] != 0.0)) && (s.v[1207] != 0.0)) && ((s.v[1215] != 0.0) && (!(s.v[1214] != 0.0)))) && (s.v[1248] != 0.0)) && (s.v[1249] != 0.0)) && (s.v[1250] != 0.0)) && (!(s.v[1252] != 0.0))) {
            s.store_div_from_scalar_ad(973, (p.p438 * s.v[449]), A::scale(s.ad_value(971), s.v[183]));
        }

        s.v[1254] = if ((s.v[971] == 0.0) || ((s.v[449] + s.v[451]) == 0.0)) { 1.0 } else { 0.0 };

        if (((((((!(s.v[1206] != 0.0)) && (s.v[1207] != 0.0)) && ((s.v[1215] != 0.0) && (!(s.v[1214] != 0.0)))) && (s.v[1248] != 0.0)) && (s.v[1249] != 0.0)) && ((s.v[1251] != 0.0) && (!(s.v[1250] != 0.0)))) && (s.v[1254] != 0.0)) {
            s.store_scalar(973, 0.0);
        }

        if (((((((!(s.v[1206] != 0.0)) && (s.v[1207] != 0.0)) && ((s.v[1215] != 0.0) && (!(s.v[1214] != 0.0)))) && (s.v[1248] != 0.0)) && (s.v[1249] != 0.0)) && ((s.v[1251] != 0.0) && (!(s.v[1250] != 0.0)))) && (!(s.v[1254] != 0.0))) {
            s.store_div_from_scalar_ad(973, (p.p438 * s.v[183]), A::scale(s.ad_value(971), (3.0 * (s.v[449] + s.v[451]))));
        }

        if ((((((!(s.v[1206] != 0.0)) && (s.v[1207] != 0.0)) && ((s.v[1215] != 0.0) && (!(s.v[1214] != 0.0)))) && (s.v[1248] != 0.0)) && (s.v[1249] != 0.0)) && (!((s.v[1250] != 0.0) || (s.v[1251] != 0.0)))) {
            s.store_scalar(973, 0.0);
        }

        s.v[1255] = if (((p.p9 == 1.0) || (p.p9 == 3.0)) || (p.p9 == 7.0)) { 1.0 } else { 0.0 };

        s.v[1256] = if (((p.p9 == 2.0) || (p.p9 == 4.0)) || (p.p9 == 8.0)) { 1.0 } else { 0.0 };

        s.v[1257] = if (s.v[971] == 0.0) { 1.0 } else { 0.0 };

        if (((((((!(s.v[1206] != 0.0)) && (s.v[1207] != 0.0)) && ((s.v[1215] != 0.0) && (!(s.v[1214] != 0.0)))) && (s.v[1248] != 0.0)) && (!(s.v[1249] != 0.0))) && (s.v[1255] != 0.0)) && (s.v[1257] != 0.0)) {
            s.store_scalar(973, 0.0);
        }

        if (((((((!(s.v[1206] != 0.0)) && (s.v[1207] != 0.0)) && ((s.v[1215] != 0.0) && (!(s.v[1214] != 0.0)))) && (s.v[1248] != 0.0)) && (!(s.v[1249] != 0.0))) && (s.v[1255] != 0.0)) && (!(s.v[1257] != 0.0))) {
            s.store_div_from_scalar_ad(973, (p.p438 * s.v[449]), A::scale(s.ad_value(971), s.v[183]));
        }

        s.v[1259] = if ((s.v[971] == 0.0) || ((s.v[449] + s.v[451]) == 0.0)) { 1.0 } else { 0.0 };

        if (((((((!(s.v[1206] != 0.0)) && (s.v[1207] != 0.0)) && ((s.v[1215] != 0.0) && (!(s.v[1214] != 0.0)))) && (s.v[1248] != 0.0)) && (!(s.v[1249] != 0.0))) && ((s.v[1256] != 0.0) && (!(s.v[1255] != 0.0)))) && (s.v[1259] != 0.0)) {
            s.store_scalar(973, 0.0);
        }

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
        if (((((((!(s.v[1206] != 0.0)) && (s.v[1207] != 0.0)) && ((s.v[1215] != 0.0) && (!(s.v[1214] != 0.0)))) && (s.v[1248] != 0.0)) && (!(s.v[1249] != 0.0))) && ((s.v[1256] != 0.0) && (!(s.v[1255] != 0.0)))) && (!(s.v[1259] != 0.0))) {
            s.store_div_from_scalar_ad(973, (p.p438 * s.v[183]), A::scale(s.ad_value(971), (3.0 * (s.v[449] + s.v[451]))));
        }

        if ((((((!(s.v[1206] != 0.0)) && (s.v[1207] != 0.0)) && ((s.v[1215] != 0.0) && (!(s.v[1214] != 0.0)))) && (s.v[1248] != 0.0)) && (!(s.v[1249] != 0.0))) && (!((s.v[1255] != 0.0) || (s.v[1256] != 0.0)))) {
            s.store_scalar(973, 0.0);
        }

        s.v[1260] = if (0.0 == 1.0) { 1.0 } else { 0.0 };

        s.v[1261] = if (((p.p9 == 1.0) || (p.p9 == 2.0)) || (p.p9 == 5.0)) { 1.0 } else { 0.0 };

        s.v[1262] = if (((p.p9 == 3.0) || (p.p9 == 4.0)) || (p.p9 == 6.0)) { 1.0 } else { 0.0 };

        s.v[1263] = if (s.v[969] == 0.0) { 1.0 } else { 0.0 };

        if (((((((!(s.v[1206] != 0.0)) && (s.v[1207] != 0.0)) && ((s.v[1215] != 0.0) && (!(s.v[1214] != 0.0)))) && (!(s.v[1248] != 0.0))) && (s.v[1260] != 0.0)) && (s.v[1261] != 0.0)) && (s.v[1263] != 0.0)) {
            s.store_scalar(973, 0.0);
        }

        if (((((((!(s.v[1206] != 0.0)) && (s.v[1207] != 0.0)) && ((s.v[1215] != 0.0) && (!(s.v[1214] != 0.0)))) && (!(s.v[1248] != 0.0))) && (s.v[1260] != 0.0)) && (s.v[1261] != 0.0)) && (!(s.v[1263] != 0.0))) {
            s.store_div_from_scalar_ad(973, (p.p438 * s.v[449]), A::scale(s.ad_value(969), s.v[183]));
        }

        s.v[1265] = if ((s.v[969] == 0.0) || (s.v[449] == 0.0)) { 1.0 } else { 0.0 };

        if (((((((!(s.v[1206] != 0.0)) && (s.v[1207] != 0.0)) && ((s.v[1215] != 0.0) && (!(s.v[1214] != 0.0)))) && (!(s.v[1248] != 0.0))) && (s.v[1260] != 0.0)) && ((s.v[1262] != 0.0) && (!(s.v[1261] != 0.0)))) && (s.v[1265] != 0.0)) {
            s.store_scalar(973, 0.0);
        }

        if (((((((!(s.v[1206] != 0.0)) && (s.v[1207] != 0.0)) && ((s.v[1215] != 0.0) && (!(s.v[1214] != 0.0)))) && (!(s.v[1248] != 0.0))) && (s.v[1260] != 0.0)) && ((s.v[1262] != 0.0) && (!(s.v[1261] != 0.0)))) && (!(s.v[1265] != 0.0))) {
            s.store_div_from_scalar_ad(973, (p.p438 * s.v[183]), A::scale(s.ad_value(969), (6.0 * s.v[449])));
        }

        if ((((((!(s.v[1206] != 0.0)) && (s.v[1207] != 0.0)) && ((s.v[1215] != 0.0) && (!(s.v[1214] != 0.0)))) && (!(s.v[1248] != 0.0))) && (s.v[1260] != 0.0)) && (!((s.v[1261] != 0.0) || (s.v[1262] != 0.0)))) {
            s.store_scalar(973, 0.0);
        }

        s.v[1266] = if (((p.p9 == 1.0) || (p.p9 == 3.0)) || (p.p9 == 7.0)) { 1.0 } else { 0.0 };

        s.v[1267] = if (((p.p9 == 2.0) || (p.p9 == 4.0)) || (p.p9 == 8.0)) { 1.0 } else { 0.0 };

        s.v[1268] = if (s.v[969] == 0.0) { 1.0 } else { 0.0 };

        if (((((((!(s.v[1206] != 0.0)) && (s.v[1207] != 0.0)) && ((s.v[1215] != 0.0) && (!(s.v[1214] != 0.0)))) && (!(s.v[1248] != 0.0))) && (!(s.v[1260] != 0.0))) && (s.v[1266] != 0.0)) && (s.v[1268] != 0.0)) {
            s.store_scalar(973, 0.0);
        }

        if (((((((!(s.v[1206] != 0.0)) && (s.v[1207] != 0.0)) && ((s.v[1215] != 0.0) && (!(s.v[1214] != 0.0)))) && (!(s.v[1248] != 0.0))) && (!(s.v[1260] != 0.0))) && (s.v[1266] != 0.0)) && (!(s.v[1268] != 0.0))) {
            s.store_div_from_scalar_ad(973, (p.p438 * s.v[449]), A::scale(s.ad_value(969), s.v[183]));
        }

        s.v[1270] = if ((s.v[969] == 0.0) || (s.v[449] == 0.0)) { 1.0 } else { 0.0 };

        if (((((((!(s.v[1206] != 0.0)) && (s.v[1207] != 0.0)) && ((s.v[1215] != 0.0) && (!(s.v[1214] != 0.0)))) && (!(s.v[1248] != 0.0))) && (!(s.v[1260] != 0.0))) && ((s.v[1267] != 0.0) && (!(s.v[1266] != 0.0)))) && (s.v[1270] != 0.0)) {
            s.store_scalar(973, 0.0);
        }

        if (((((((!(s.v[1206] != 0.0)) && (s.v[1207] != 0.0)) && ((s.v[1215] != 0.0) && (!(s.v[1214] != 0.0)))) && (!(s.v[1248] != 0.0))) && (!(s.v[1260] != 0.0))) && ((s.v[1267] != 0.0) && (!(s.v[1266] != 0.0)))) && (!(s.v[1270] != 0.0))) {
            s.store_div_from_scalar_ad(973, (p.p438 * s.v[183]), A::scale(s.ad_value(969), (6.0 * s.v[449])));
        }

        if ((((((!(s.v[1206] != 0.0)) && (s.v[1207] != 0.0)) && ((s.v[1215] != 0.0) && (!(s.v[1214] != 0.0)))) && (!(s.v[1248] != 0.0))) && (!(s.v[1260] != 0.0))) && (!((s.v[1266] != 0.0) || (s.v[1267] != 0.0)))) {
            s.store_scalar(973, 0.0);
        }

        s.v[1271] = if (1.0 == 1.0) { 1.0 } else { 0.0 };

        s.v[1272] = if (1.0 == 1.0) { 1.0 } else { 0.0 };

        s.v[1273] = if (((p.p9 == 1.0) || (p.p9 == 2.0)) || (p.p9 == 5.0)) { 1.0 } else { 0.0 };

        s.v[1274] = if (((p.p9 == 3.0) || (p.p9 == 4.0)) || (p.p9 == 6.0)) { 1.0 } else { 0.0 };

        s.v[1275] = if (s.v[971] == 0.0) { 1.0 } else { 0.0 };

        if (((((((!(s.v[1206] != 0.0)) && (s.v[1207] != 0.0)) && ((s.v[1216] != 0.0) && (!((s.v[1214] != 0.0) || (s.v[1215] != 0.0))))) && (s.v[1271] != 0.0)) && (s.v[1272] != 0.0)) && (s.v[1273] != 0.0)) && (s.v[1275] != 0.0)) {
            s.store_scalar(973, 0.0);
        }

        if (((((((!(s.v[1206] != 0.0)) && (s.v[1207] != 0.0)) && ((s.v[1216] != 0.0) && (!((s.v[1214] != 0.0) || (s.v[1215] != 0.0))))) && (s.v[1271] != 0.0)) && (s.v[1272] != 0.0)) && (s.v[1273] != 0.0)) && (!(s.v[1275] != 0.0))) {
            s.store_div_from_scalar_ad(973, (p.p438 * s.v[449]), A::scale(s.ad_value(971), s.v[183]));
        }

        s.v[1277] = if ((s.v[971] == 0.0) || (s.v[449] == 0.0)) { 1.0 } else { 0.0 };

        if (((((((!(s.v[1206] != 0.0)) && (s.v[1207] != 0.0)) && ((s.v[1216] != 0.0) && (!((s.v[1214] != 0.0) || (s.v[1215] != 0.0))))) && (s.v[1271] != 0.0)) && (s.v[1272] != 0.0)) && ((s.v[1274] != 0.0) && (!(s.v[1273] != 0.0)))) && (s.v[1277] != 0.0)) {
            s.store_scalar(973, 0.0);
        }

        if (((((((!(s.v[1206] != 0.0)) && (s.v[1207] != 0.0)) && ((s.v[1216] != 0.0) && (!((s.v[1214] != 0.0) || (s.v[1215] != 0.0))))) && (s.v[1271] != 0.0)) && (s.v[1272] != 0.0)) && ((s.v[1274] != 0.0) && (!(s.v[1273] != 0.0)))) && (!(s.v[1277] != 0.0))) {
            s.store_div_from_scalar_ad(973, (p.p438 * s.v[183]), A::scale(s.ad_value(971), (6.0 * s.v[449])));
        }

        if ((((((!(s.v[1206] != 0.0)) && (s.v[1207] != 0.0)) && ((s.v[1216] != 0.0) && (!((s.v[1214] != 0.0) || (s.v[1215] != 0.0))))) && (s.v[1271] != 0.0)) && (s.v[1272] != 0.0)) && (!((s.v[1273] != 0.0) || (s.v[1274] != 0.0)))) {
            s.store_scalar(973, 0.0);
        }

        s.v[1278] = if (((p.p9 == 1.0) || (p.p9 == 3.0)) || (p.p9 == 7.0)) { 1.0 } else { 0.0 };

        s.v[1279] = if (((p.p9 == 2.0) || (p.p9 == 4.0)) || (p.p9 == 8.0)) { 1.0 } else { 0.0 };

        s.v[1280] = if (s.v[971] == 0.0) { 1.0 } else { 0.0 };

        if (((((((!(s.v[1206] != 0.0)) && (s.v[1207] != 0.0)) && ((s.v[1216] != 0.0) && (!((s.v[1214] != 0.0) || (s.v[1215] != 0.0))))) && (s.v[1271] != 0.0)) && (!(s.v[1272] != 0.0))) && (s.v[1278] != 0.0)) && (s.v[1280] != 0.0)) {
            s.store_scalar(973, 0.0);
        }

        if (((((((!(s.v[1206] != 0.0)) && (s.v[1207] != 0.0)) && ((s.v[1216] != 0.0) && (!((s.v[1214] != 0.0) || (s.v[1215] != 0.0))))) && (s.v[1271] != 0.0)) && (!(s.v[1272] != 0.0))) && (s.v[1278] != 0.0)) && (!(s.v[1280] != 0.0))) {
            s.store_div_from_scalar_ad(973, (p.p438 * s.v[449]), A::scale(s.ad_value(971), s.v[183]));
        }

        s.v[1282] = if ((s.v[971] == 0.0) || (s.v[449] == 0.0)) { 1.0 } else { 0.0 };

        if (((((((!(s.v[1206] != 0.0)) && (s.v[1207] != 0.0)) && ((s.v[1216] != 0.0) && (!((s.v[1214] != 0.0) || (s.v[1215] != 0.0))))) && (s.v[1271] != 0.0)) && (!(s.v[1272] != 0.0))) && ((s.v[1279] != 0.0) && (!(s.v[1278] != 0.0)))) && (s.v[1282] != 0.0)) {
            s.store_scalar(973, 0.0);
        }

        if (((((((!(s.v[1206] != 0.0)) && (s.v[1207] != 0.0)) && ((s.v[1216] != 0.0) && (!((s.v[1214] != 0.0) || (s.v[1215] != 0.0))))) && (s.v[1271] != 0.0)) && (!(s.v[1272] != 0.0))) && ((s.v[1279] != 0.0) && (!(s.v[1278] != 0.0)))) && (!(s.v[1282] != 0.0))) {
            s.store_div_from_scalar_ad(973, (p.p438 * s.v[183]), A::scale(s.ad_value(971), (6.0 * s.v[449])));
        }

        if ((((((!(s.v[1206] != 0.0)) && (s.v[1207] != 0.0)) && ((s.v[1216] != 0.0) && (!((s.v[1214] != 0.0) || (s.v[1215] != 0.0))))) && (s.v[1271] != 0.0)) && (!(s.v[1272] != 0.0))) && (!((s.v[1278] != 0.0) || (s.v[1279] != 0.0)))) {
            s.store_scalar(973, 0.0);
        }

        s.v[1283] = if (0.0 == 1.0) { 1.0 } else { 0.0 };

        s.v[1284] = if (((p.p9 == 1.0) || (p.p9 == 2.0)) || (p.p9 == 5.0)) { 1.0 } else { 0.0 };

        s.v[1285] = if (((p.p9 == 3.0) || (p.p9 == 4.0)) || (p.p9 == 6.0)) { 1.0 } else { 0.0 };

        s.v[1286] = if (s.v[969] == 0.0) { 1.0 } else { 0.0 };

        if (((((((!(s.v[1206] != 0.0)) && (s.v[1207] != 0.0)) && ((s.v[1216] != 0.0) && (!((s.v[1214] != 0.0) || (s.v[1215] != 0.0))))) && (!(s.v[1271] != 0.0))) && (s.v[1283] != 0.0)) && (s.v[1284] != 0.0)) && (s.v[1286] != 0.0)) {
            s.store_scalar(973, 0.0);
        }

        if (((((((!(s.v[1206] != 0.0)) && (s.v[1207] != 0.0)) && ((s.v[1216] != 0.0) && (!((s.v[1214] != 0.0) || (s.v[1215] != 0.0))))) && (!(s.v[1271] != 0.0))) && (s.v[1283] != 0.0)) && (s.v[1284] != 0.0)) && (!(s.v[1286] != 0.0))) {
            s.store_div_from_scalar_ad(973, (p.p438 * s.v[449]), A::scale(s.ad_value(969), s.v[183]));
        }

        s.v[1288] = if ((s.v[969] == 0.0) || ((s.v[449] + s.v[451]) == 0.0)) { 1.0 } else { 0.0 };

        if (((((((!(s.v[1206] != 0.0)) && (s.v[1207] != 0.0)) && ((s.v[1216] != 0.0) && (!((s.v[1214] != 0.0) || (s.v[1215] != 0.0))))) && (!(s.v[1271] != 0.0))) && (s.v[1283] != 0.0)) && ((s.v[1285] != 0.0) && (!(s.v[1284] != 0.0)))) && (s.v[1288] != 0.0)) {
            s.store_scalar(973, 0.0);
        }

        if (((((((!(s.v[1206] != 0.0)) && (s.v[1207] != 0.0)) && ((s.v[1216] != 0.0) && (!((s.v[1214] != 0.0) || (s.v[1215] != 0.0))))) && (!(s.v[1271] != 0.0))) && (s.v[1283] != 0.0)) && ((s.v[1285] != 0.0) && (!(s.v[1284] != 0.0)))) && (!(s.v[1288] != 0.0))) {
            s.store_div_from_scalar_ad(973, (p.p438 * s.v[183]), A::scale(s.ad_value(969), (3.0 * (s.v[449] + s.v[451]))));
        }

        if ((((((!(s.v[1206] != 0.0)) && (s.v[1207] != 0.0)) && ((s.v[1216] != 0.0) && (!((s.v[1214] != 0.0) || (s.v[1215] != 0.0))))) && (!(s.v[1271] != 0.0))) && (s.v[1283] != 0.0)) && (!((s.v[1284] != 0.0) || (s.v[1285] != 0.0)))) {
            s.store_scalar(973, 0.0);
        }

        s.v[1289] = if (((p.p9 == 1.0) || (p.p9 == 3.0)) || (p.p9 == 7.0)) { 1.0 } else { 0.0 };

        s.v[1290] = if (((p.p9 == 2.0) || (p.p9 == 4.0)) || (p.p9 == 8.0)) { 1.0 } else { 0.0 };

        s.v[1291] = if (s.v[969] == 0.0) { 1.0 } else { 0.0 };

        if (((((((!(s.v[1206] != 0.0)) && (s.v[1207] != 0.0)) && ((s.v[1216] != 0.0) && (!((s.v[1214] != 0.0) || (s.v[1215] != 0.0))))) && (!(s.v[1271] != 0.0))) && (!(s.v[1283] != 0.0))) && (s.v[1289] != 0.0)) && (s.v[1291] != 0.0)) {
            s.store_scalar(973, 0.0);
        }

        if (((((((!(s.v[1206] != 0.0)) && (s.v[1207] != 0.0)) && ((s.v[1216] != 0.0) && (!((s.v[1214] != 0.0) || (s.v[1215] != 0.0))))) && (!(s.v[1271] != 0.0))) && (!(s.v[1283] != 0.0))) && (s.v[1289] != 0.0)) && (!(s.v[1291] != 0.0))) {
            s.store_div_from_scalar_ad(973, (p.p438 * s.v[449]), A::scale(s.ad_value(969), s.v[183]));
        }

        s.v[1293] = if ((s.v[969] == 0.0) || ((s.v[449] + s.v[451]) == 0.0)) { 1.0 } else { 0.0 };

        if (((((((!(s.v[1206] != 0.0)) && (s.v[1207] != 0.0)) && ((s.v[1216] != 0.0) && (!((s.v[1214] != 0.0) || (s.v[1215] != 0.0))))) && (!(s.v[1271] != 0.0))) && (!(s.v[1283] != 0.0))) && ((s.v[1290] != 0.0) && (!(s.v[1289] != 0.0)))) && (s.v[1293] != 0.0)) {
            s.store_scalar(973, 0.0);
        }

        if (((((((!(s.v[1206] != 0.0)) && (s.v[1207] != 0.0)) && ((s.v[1216] != 0.0) && (!((s.v[1214] != 0.0) || (s.v[1215] != 0.0))))) && (!(s.v[1271] != 0.0))) && (!(s.v[1283] != 0.0))) && ((s.v[1290] != 0.0) && (!(s.v[1289] != 0.0)))) && (!(s.v[1293] != 0.0))) {
            s.store_div_from_scalar_ad(973, (p.p438 * s.v[183]), A::scale(s.ad_value(969), (3.0 * (s.v[449] + s.v[451]))));
        }

        if ((((((!(s.v[1206] != 0.0)) && (s.v[1207] != 0.0)) && ((s.v[1216] != 0.0) && (!((s.v[1214] != 0.0) || (s.v[1215] != 0.0))))) && (!(s.v[1271] != 0.0))) && (!(s.v[1283] != 0.0))) && (!((s.v[1289] != 0.0) || (s.v[1290] != 0.0)))) {
            s.store_scalar(973, 0.0);
        }

        s.v[1294] = if (1.0 == 1.0) { 1.0 } else { 0.0 };

        s.v[1295] = if (1.0 == 1.0) { 1.0 } else { 0.0 };

        s.v[1296] = if (((p.p9 == 1.0) || (p.p9 == 2.0)) || (p.p9 == 5.0)) { 1.0 } else { 0.0 };

        s.v[1297] = if (((p.p9 == 3.0) || (p.p9 == 4.0)) || (p.p9 == 6.0)) { 1.0 } else { 0.0 };

        s.v[1298] = if (s.v[971] == 0.0) { 1.0 } else { 0.0 };

        if (((((((!(s.v[1206] != 0.0)) && (s.v[1207] != 0.0)) && ((s.v[1217] != 0.0) && (!(((s.v[1214] != 0.0) || (s.v[1215] != 0.0)) || (s.v[1216] != 0.0))))) && (s.v[1294] != 0.0)) && (s.v[1295] != 0.0)) && (s.v[1296] != 0.0)) && (s.v[1298] != 0.0)) {
            s.store_scalar(973, 0.0);
        }

        if (((((((!(s.v[1206] != 0.0)) && (s.v[1207] != 0.0)) && ((s.v[1217] != 0.0) && (!(((s.v[1214] != 0.0) || (s.v[1215] != 0.0)) || (s.v[1216] != 0.0))))) && (s.v[1294] != 0.0)) && (s.v[1295] != 0.0)) && (s.v[1296] != 0.0)) && (!(s.v[1298] != 0.0))) {
            s.store_div_from_scalar_ad(973, (p.p438 * s.v[449]), A::scale(s.ad_value(971), s.v[183]));
        }

        s.v[1300] = if ((s.v[971] == 0.0) || (s.v[449] == 0.0)) { 1.0 } else { 0.0 };

        if (((((((!(s.v[1206] != 0.0)) && (s.v[1207] != 0.0)) && ((s.v[1217] != 0.0) && (!(((s.v[1214] != 0.0) || (s.v[1215] != 0.0)) || (s.v[1216] != 0.0))))) && (s.v[1294] != 0.0)) && (s.v[1295] != 0.0)) && ((s.v[1297] != 0.0) && (!(s.v[1296] != 0.0)))) && (s.v[1300] != 0.0)) {
            s.store_scalar(973, 0.0);
        }

        if (((((((!(s.v[1206] != 0.0)) && (s.v[1207] != 0.0)) && ((s.v[1217] != 0.0) && (!(((s.v[1214] != 0.0) || (s.v[1215] != 0.0)) || (s.v[1216] != 0.0))))) && (s.v[1294] != 0.0)) && (s.v[1295] != 0.0)) && ((s.v[1297] != 0.0) && (!(s.v[1296] != 0.0)))) && (!(s.v[1300] != 0.0))) {
            s.store_div_from_scalar_ad(973, (p.p438 * s.v[183]), A::scale(s.ad_value(971), (6.0 * s.v[449])));
        }

        if ((((((!(s.v[1206] != 0.0)) && (s.v[1207] != 0.0)) && ((s.v[1217] != 0.0) && (!(((s.v[1214] != 0.0) || (s.v[1215] != 0.0)) || (s.v[1216] != 0.0))))) && (s.v[1294] != 0.0)) && (s.v[1295] != 0.0)) && (!((s.v[1296] != 0.0) || (s.v[1297] != 0.0)))) {
            s.store_scalar(973, 0.0);
        }

        s.v[1301] = if (((p.p9 == 1.0) || (p.p9 == 3.0)) || (p.p9 == 7.0)) { 1.0 } else { 0.0 };

        s.v[1302] = if (((p.p9 == 2.0) || (p.p9 == 4.0)) || (p.p9 == 8.0)) { 1.0 } else { 0.0 };

        s.v[1303] = if (s.v[971] == 0.0) { 1.0 } else { 0.0 };

        if (((((((!(s.v[1206] != 0.0)) && (s.v[1207] != 0.0)) && ((s.v[1217] != 0.0) && (!(((s.v[1214] != 0.0) || (s.v[1215] != 0.0)) || (s.v[1216] != 0.0))))) && (s.v[1294] != 0.0)) && (!(s.v[1295] != 0.0))) && (s.v[1301] != 0.0)) && (s.v[1303] != 0.0)) {
            s.store_scalar(973, 0.0);
        }

        if (((((((!(s.v[1206] != 0.0)) && (s.v[1207] != 0.0)) && ((s.v[1217] != 0.0) && (!(((s.v[1214] != 0.0) || (s.v[1215] != 0.0)) || (s.v[1216] != 0.0))))) && (s.v[1294] != 0.0)) && (!(s.v[1295] != 0.0))) && (s.v[1301] != 0.0)) && (!(s.v[1303] != 0.0))) {
            s.store_div_from_scalar_ad(973, (p.p438 * s.v[449]), A::scale(s.ad_value(971), s.v[183]));
        }

        s.v[1305] = if ((s.v[971] == 0.0) || (s.v[449] == 0.0)) { 1.0 } else { 0.0 };

        if (((((((!(s.v[1206] != 0.0)) && (s.v[1207] != 0.0)) && ((s.v[1217] != 0.0) && (!(((s.v[1214] != 0.0) || (s.v[1215] != 0.0)) || (s.v[1216] != 0.0))))) && (s.v[1294] != 0.0)) && (!(s.v[1295] != 0.0))) && ((s.v[1302] != 0.0) && (!(s.v[1301] != 0.0)))) && (s.v[1305] != 0.0)) {
            s.store_scalar(973, 0.0);
        }

        if (((((((!(s.v[1206] != 0.0)) && (s.v[1207] != 0.0)) && ((s.v[1217] != 0.0) && (!(((s.v[1214] != 0.0) || (s.v[1215] != 0.0)) || (s.v[1216] != 0.0))))) && (s.v[1294] != 0.0)) && (!(s.v[1295] != 0.0))) && ((s.v[1302] != 0.0) && (!(s.v[1301] != 0.0)))) && (!(s.v[1305] != 0.0))) {
            s.store_div_from_scalar_ad(973, (p.p438 * s.v[183]), A::scale(s.ad_value(971), (6.0 * s.v[449])));
        }

        if ((((((!(s.v[1206] != 0.0)) && (s.v[1207] != 0.0)) && ((s.v[1217] != 0.0) && (!(((s.v[1214] != 0.0) || (s.v[1215] != 0.0)) || (s.v[1216] != 0.0))))) && (s.v[1294] != 0.0)) && (!(s.v[1295] != 0.0))) && (!((s.v[1301] != 0.0) || (s.v[1302] != 0.0)))) {
            s.store_scalar(973, 0.0);
        }

        s.v[1306] = if (0.0 == 1.0) { 1.0 } else { 0.0 };

        s.v[1307] = if (((p.p9 == 1.0) || (p.p9 == 2.0)) || (p.p9 == 5.0)) { 1.0 } else { 0.0 };

        s.v[1308] = if (((p.p9 == 3.0) || (p.p9 == 4.0)) || (p.p9 == 6.0)) { 1.0 } else { 0.0 };

        s.v[1309] = if (s.v[969] == 0.0) { 1.0 } else { 0.0 };

        if (((((((!(s.v[1206] != 0.0)) && (s.v[1207] != 0.0)) && ((s.v[1217] != 0.0) && (!(((s.v[1214] != 0.0) || (s.v[1215] != 0.0)) || (s.v[1216] != 0.0))))) && (!(s.v[1294] != 0.0))) && (s.v[1306] != 0.0)) && (s.v[1307] != 0.0)) && (s.v[1309] != 0.0)) {
            s.store_scalar(973, 0.0);
        }

        if (((((((!(s.v[1206] != 0.0)) && (s.v[1207] != 0.0)) && ((s.v[1217] != 0.0) && (!(((s.v[1214] != 0.0) || (s.v[1215] != 0.0)) || (s.v[1216] != 0.0))))) && (!(s.v[1294] != 0.0))) && (s.v[1306] != 0.0)) && (s.v[1307] != 0.0)) && (!(s.v[1309] != 0.0))) {
            s.store_div_from_scalar_ad(973, (p.p438 * s.v[449]), A::scale(s.ad_value(969), s.v[183]));
        }

        s.v[1311] = if ((s.v[969] == 0.0) || (s.v[449] == 0.0)) { 1.0 } else { 0.0 };

        if (((((((!(s.v[1206] != 0.0)) && (s.v[1207] != 0.0)) && ((s.v[1217] != 0.0) && (!(((s.v[1214] != 0.0) || (s.v[1215] != 0.0)) || (s.v[1216] != 0.0))))) && (!(s.v[1294] != 0.0))) && (s.v[1306] != 0.0)) && ((s.v[1308] != 0.0) && (!(s.v[1307] != 0.0)))) && (s.v[1311] != 0.0)) {
            s.store_scalar(973, 0.0);
        }

        if (((((((!(s.v[1206] != 0.0)) && (s.v[1207] != 0.0)) && ((s.v[1217] != 0.0) && (!(((s.v[1214] != 0.0) || (s.v[1215] != 0.0)) || (s.v[1216] != 0.0))))) && (!(s.v[1294] != 0.0))) && (s.v[1306] != 0.0)) && ((s.v[1308] != 0.0) && (!(s.v[1307] != 0.0)))) && (!(s.v[1311] != 0.0))) {
            s.store_div_from_scalar_ad(973, (p.p438 * s.v[183]), A::scale(s.ad_value(969), (6.0 * s.v[449])));
        }

        if ((((((!(s.v[1206] != 0.0)) && (s.v[1207] != 0.0)) && ((s.v[1217] != 0.0) && (!(((s.v[1214] != 0.0) || (s.v[1215] != 0.0)) || (s.v[1216] != 0.0))))) && (!(s.v[1294] != 0.0))) && (s.v[1306] != 0.0)) && (!((s.v[1307] != 0.0) || (s.v[1308] != 0.0)))) {
            s.store_scalar(973, 0.0);
        }

        s.v[1312] = if (((p.p9 == 1.0) || (p.p9 == 3.0)) || (p.p9 == 7.0)) { 1.0 } else { 0.0 };

        s.v[1313] = if (((p.p9 == 2.0) || (p.p9 == 4.0)) || (p.p9 == 8.0)) { 1.0 } else { 0.0 };

        s.v[1314] = if (s.v[969] == 0.0) { 1.0 } else { 0.0 };

        if (((((((!(s.v[1206] != 0.0)) && (s.v[1207] != 0.0)) && ((s.v[1217] != 0.0) && (!(((s.v[1214] != 0.0) || (s.v[1215] != 0.0)) || (s.v[1216] != 0.0))))) && (!(s.v[1294] != 0.0))) && (!(s.v[1306] != 0.0))) && (s.v[1312] != 0.0)) && (s.v[1314] != 0.0)) {
            s.store_scalar(973, 0.0);
        }

        if (((((((!(s.v[1206] != 0.0)) && (s.v[1207] != 0.0)) && ((s.v[1217] != 0.0) && (!(((s.v[1214] != 0.0) || (s.v[1215] != 0.0)) || (s.v[1216] != 0.0))))) && (!(s.v[1294] != 0.0))) && (!(s.v[1306] != 0.0))) && (s.v[1312] != 0.0)) && (!(s.v[1314] != 0.0))) {
            s.store_div_from_scalar_ad(973, (p.p438 * s.v[449]), A::scale(s.ad_value(969), s.v[183]));
        }

        s.v[1316] = if ((s.v[969] == 0.0) || (s.v[449] == 0.0)) { 1.0 } else { 0.0 };

        if (((((((!(s.v[1206] != 0.0)) && (s.v[1207] != 0.0)) && ((s.v[1217] != 0.0) && (!(((s.v[1214] != 0.0) || (s.v[1215] != 0.0)) || (s.v[1216] != 0.0))))) && (!(s.v[1294] != 0.0))) && (!(s.v[1306] != 0.0))) && ((s.v[1313] != 0.0) && (!(s.v[1312] != 0.0)))) && (s.v[1316] != 0.0)) {
            s.store_scalar(973, 0.0);
        }

        if (((((((!(s.v[1206] != 0.0)) && (s.v[1207] != 0.0)) && ((s.v[1217] != 0.0) && (!(((s.v[1214] != 0.0) || (s.v[1215] != 0.0)) || (s.v[1216] != 0.0))))) && (!(s.v[1294] != 0.0))) && (!(s.v[1306] != 0.0))) && ((s.v[1313] != 0.0) && (!(s.v[1312] != 0.0)))) && (!(s.v[1316] != 0.0))) {
            s.store_div_from_scalar_ad(973, (p.p438 * s.v[183]), A::scale(s.ad_value(969), (6.0 * s.v[449])));
        }

        if ((((((!(s.v[1206] != 0.0)) && (s.v[1207] != 0.0)) && ((s.v[1217] != 0.0) && (!(((s.v[1214] != 0.0) || (s.v[1215] != 0.0)) || (s.v[1216] != 0.0))))) && (!(s.v[1294] != 0.0))) && (!(s.v[1306] != 0.0))) && (!((s.v[1312] != 0.0) || (s.v[1313] != 0.0)))) {
            s.store_scalar(973, 0.0);
        }

        s.v[1317] = if (1.0 == 1.0) { 1.0 } else { 0.0 };

        s.v[1318] = if (1.0 == 1.0) { 1.0 } else { 0.0 };

        s.v[1319] = if (((p.p9 == 1.0) || (p.p9 == 2.0)) || (p.p9 == 5.0)) { 1.0 } else { 0.0 };

        s.v[1320] = if (((p.p9 == 3.0) || (p.p9 == 4.0)) || (p.p9 == 6.0)) { 1.0 } else { 0.0 };

        s.v[1321] = if (s.v[971] == 0.0) { 1.0 } else { 0.0 };

        if (((((((!(s.v[1206] != 0.0)) && (s.v[1207] != 0.0)) && ((s.v[1218] != 0.0) && (!((((s.v[1214] != 0.0) || (s.v[1215] != 0.0)) || (s.v[1216] != 0.0)) || (s.v[1217] != 0.0))))) && (s.v[1317] != 0.0)) && (s.v[1318] != 0.0)) && (s.v[1319] != 0.0)) && (s.v[1321] != 0.0)) {
            s.store_scalar(973, 0.0);
        }

        if (((((((!(s.v[1206] != 0.0)) && (s.v[1207] != 0.0)) && ((s.v[1218] != 0.0) && (!((((s.v[1214] != 0.0) || (s.v[1215] != 0.0)) || (s.v[1216] != 0.0)) || (s.v[1217] != 0.0))))) && (s.v[1317] != 0.0)) && (s.v[1318] != 0.0)) && (s.v[1319] != 0.0)) && (!(s.v[1321] != 0.0))) {
            s.store_div_from_scalar_ad(973, (p.p438 * s.v[449]), A::scale(s.ad_value(971), s.v[183]));
        }

        s.v[1323] = if ((s.v[971] == 0.0) || ((s.v[449] + s.v[451]) == 0.0)) { 1.0 } else { 0.0 };

        if (((((((!(s.v[1206] != 0.0)) && (s.v[1207] != 0.0)) && ((s.v[1218] != 0.0) && (!((((s.v[1214] != 0.0) || (s.v[1215] != 0.0)) || (s.v[1216] != 0.0)) || (s.v[1217] != 0.0))))) && (s.v[1317] != 0.0)) && (s.v[1318] != 0.0)) && ((s.v[1320] != 0.0) && (!(s.v[1319] != 0.0)))) && (s.v[1323] != 0.0)) {
            s.store_scalar(973, 0.0);
        }

        if (((((((!(s.v[1206] != 0.0)) && (s.v[1207] != 0.0)) && ((s.v[1218] != 0.0) && (!((((s.v[1214] != 0.0) || (s.v[1215] != 0.0)) || (s.v[1216] != 0.0)) || (s.v[1217] != 0.0))))) && (s.v[1317] != 0.0)) && (s.v[1318] != 0.0)) && ((s.v[1320] != 0.0) && (!(s.v[1319] != 0.0)))) && (!(s.v[1323] != 0.0))) {
            s.store_div_from_scalar_ad(973, (p.p438 * s.v[183]), A::scale(s.ad_value(971), (3.0 * (s.v[449] + s.v[451]))));
        }

        if ((((((!(s.v[1206] != 0.0)) && (s.v[1207] != 0.0)) && ((s.v[1218] != 0.0) && (!((((s.v[1214] != 0.0) || (s.v[1215] != 0.0)) || (s.v[1216] != 0.0)) || (s.v[1217] != 0.0))))) && (s.v[1317] != 0.0)) && (s.v[1318] != 0.0)) && (!((s.v[1319] != 0.0) || (s.v[1320] != 0.0)))) {
            s.store_scalar(973, 0.0);
        }

        s.v[1324] = if (((p.p9 == 1.0) || (p.p9 == 3.0)) || (p.p9 == 7.0)) { 1.0 } else { 0.0 };

        s.v[1325] = if (((p.p9 == 2.0) || (p.p9 == 4.0)) || (p.p9 == 8.0)) { 1.0 } else { 0.0 };

        s.v[1326] = if (s.v[971] == 0.0) { 1.0 } else { 0.0 };

        if (((((((!(s.v[1206] != 0.0)) && (s.v[1207] != 0.0)) && ((s.v[1218] != 0.0) && (!((((s.v[1214] != 0.0) || (s.v[1215] != 0.0)) || (s.v[1216] != 0.0)) || (s.v[1217] != 0.0))))) && (s.v[1317] != 0.0)) && (!(s.v[1318] != 0.0))) && (s.v[1324] != 0.0)) && (s.v[1326] != 0.0)) {
            s.store_scalar(973, 0.0);
        }

        if (((((((!(s.v[1206] != 0.0)) && (s.v[1207] != 0.0)) && ((s.v[1218] != 0.0) && (!((((s.v[1214] != 0.0) || (s.v[1215] != 0.0)) || (s.v[1216] != 0.0)) || (s.v[1217] != 0.0))))) && (s.v[1317] != 0.0)) && (!(s.v[1318] != 0.0))) && (s.v[1324] != 0.0)) && (!(s.v[1326] != 0.0))) {
            s.store_div_from_scalar_ad(973, (p.p438 * s.v[449]), A::scale(s.ad_value(971), s.v[183]));
        }

        s.v[1328] = if ((s.v[971] == 0.0) || ((s.v[449] + s.v[451]) == 0.0)) { 1.0 } else { 0.0 };

        if (((((((!(s.v[1206] != 0.0)) && (s.v[1207] != 0.0)) && ((s.v[1218] != 0.0) && (!((((s.v[1214] != 0.0) || (s.v[1215] != 0.0)) || (s.v[1216] != 0.0)) || (s.v[1217] != 0.0))))) && (s.v[1317] != 0.0)) && (!(s.v[1318] != 0.0))) && ((s.v[1325] != 0.0) && (!(s.v[1324] != 0.0)))) && (s.v[1328] != 0.0)) {
            s.store_scalar(973, 0.0);
        }

        if (((((((!(s.v[1206] != 0.0)) && (s.v[1207] != 0.0)) && ((s.v[1218] != 0.0) && (!((((s.v[1214] != 0.0) || (s.v[1215] != 0.0)) || (s.v[1216] != 0.0)) || (s.v[1217] != 0.0))))) && (s.v[1317] != 0.0)) && (!(s.v[1318] != 0.0))) && ((s.v[1325] != 0.0) && (!(s.v[1324] != 0.0)))) && (!(s.v[1328] != 0.0))) {
            s.store_div_from_scalar_ad(973, (p.p438 * s.v[183]), A::scale(s.ad_value(971), (3.0 * (s.v[449] + s.v[451]))));
        }

        if ((((((!(s.v[1206] != 0.0)) && (s.v[1207] != 0.0)) && ((s.v[1218] != 0.0) && (!((((s.v[1214] != 0.0) || (s.v[1215] != 0.0)) || (s.v[1216] != 0.0)) || (s.v[1217] != 0.0))))) && (s.v[1317] != 0.0)) && (!(s.v[1318] != 0.0))) && (!((s.v[1324] != 0.0) || (s.v[1325] != 0.0)))) {
            s.store_scalar(973, 0.0);
        }

        if ((((!(s.v[1206] != 0.0)) && (s.v[1207] != 0.0)) && ((s.v[1218] != 0.0) && (!((((s.v[1214] != 0.0) || (s.v[1215] != 0.0)) || (s.v[1216] != 0.0)) || (s.v[1217] != 0.0))))) && (!(s.v[1317] != 0.0))) {
            s.store_scalar(973, ((p.p438 * s.v[450]) / s.v[183]));
        }

        s.v[1329] = if (1.0 == 1.0) { 1.0 } else { 0.0 };

        s.v[1330] = if (1.0 == 1.0) { 1.0 } else { 0.0 };

        s.v[1331] = if (((p.p9 == 1.0) || (p.p9 == 2.0)) || (p.p9 == 5.0)) { 1.0 } else { 0.0 };

        s.v[1332] = if (((p.p9 == 3.0) || (p.p9 == 4.0)) || (p.p9 == 6.0)) { 1.0 } else { 0.0 };

        s.v[1333] = if (s.v[971] == 0.0) { 1.0 } else { 0.0 };

        if (((((((!(s.v[1206] != 0.0)) && (s.v[1207] != 0.0)) && ((s.v[1219] != 0.0) && (!(((((s.v[1214] != 0.0) || (s.v[1215] != 0.0)) || (s.v[1216] != 0.0)) || (s.v[1217] != 0.0)) || (s.v[1218] != 0.0))))) && (s.v[1329] != 0.0)) && (s.v[1330] != 0.0)) && (s.v[1331] != 0.0)) && (s.v[1333] != 0.0)) {
            s.store_scalar(973, 0.0);
        }

        if (((((((!(s.v[1206] != 0.0)) && (s.v[1207] != 0.0)) && ((s.v[1219] != 0.0) && (!(((((s.v[1214] != 0.0) || (s.v[1215] != 0.0)) || (s.v[1216] != 0.0)) || (s.v[1217] != 0.0)) || (s.v[1218] != 0.0))))) && (s.v[1329] != 0.0)) && (s.v[1330] != 0.0)) && (s.v[1331] != 0.0)) && (!(s.v[1333] != 0.0))) {
            s.store_div_from_scalar_ad(973, (p.p438 * s.v[449]), A::scale(s.ad_value(971), s.v[183]));
        }

        s.v[1335] = if ((s.v[971] == 0.0) || (s.v[449] == 0.0)) { 1.0 } else { 0.0 };

        if (((((((!(s.v[1206] != 0.0)) && (s.v[1207] != 0.0)) && ((s.v[1219] != 0.0) && (!(((((s.v[1214] != 0.0) || (s.v[1215] != 0.0)) || (s.v[1216] != 0.0)) || (s.v[1217] != 0.0)) || (s.v[1218] != 0.0))))) && (s.v[1329] != 0.0)) && (s.v[1330] != 0.0)) && ((s.v[1332] != 0.0) && (!(s.v[1331] != 0.0)))) && (s.v[1335] != 0.0)) {
            s.store_scalar(973, 0.0);
        }

        if (((((((!(s.v[1206] != 0.0)) && (s.v[1207] != 0.0)) && ((s.v[1219] != 0.0) && (!(((((s.v[1214] != 0.0) || (s.v[1215] != 0.0)) || (s.v[1216] != 0.0)) || (s.v[1217] != 0.0)) || (s.v[1218] != 0.0))))) && (s.v[1329] != 0.0)) && (s.v[1330] != 0.0)) && ((s.v[1332] != 0.0) && (!(s.v[1331] != 0.0)))) && (!(s.v[1335] != 0.0))) {
            s.store_div_from_scalar_ad(973, (p.p438 * s.v[183]), A::scale(s.ad_value(971), (6.0 * s.v[449])));
        }

        if ((((((!(s.v[1206] != 0.0)) && (s.v[1207] != 0.0)) && ((s.v[1219] != 0.0) && (!(((((s.v[1214] != 0.0) || (s.v[1215] != 0.0)) || (s.v[1216] != 0.0)) || (s.v[1217] != 0.0)) || (s.v[1218] != 0.0))))) && (s.v[1329] != 0.0)) && (s.v[1330] != 0.0)) && (!((s.v[1331] != 0.0) || (s.v[1332] != 0.0)))) {
            s.store_scalar(973, 0.0);
        }

        s.v[1336] = if (((p.p9 == 1.0) || (p.p9 == 3.0)) || (p.p9 == 7.0)) { 1.0 } else { 0.0 };

        s.v[1337] = if (((p.p9 == 2.0) || (p.p9 == 4.0)) || (p.p9 == 8.0)) { 1.0 } else { 0.0 };

        s.v[1338] = if (s.v[971] == 0.0) { 1.0 } else { 0.0 };

        if (((((((!(s.v[1206] != 0.0)) && (s.v[1207] != 0.0)) && ((s.v[1219] != 0.0) && (!(((((s.v[1214] != 0.0) || (s.v[1215] != 0.0)) || (s.v[1216] != 0.0)) || (s.v[1217] != 0.0)) || (s.v[1218] != 0.0))))) && (s.v[1329] != 0.0)) && (!(s.v[1330] != 0.0))) && (s.v[1336] != 0.0)) && (s.v[1338] != 0.0)) {
            s.store_scalar(973, 0.0);
        }

        if (((((((!(s.v[1206] != 0.0)) && (s.v[1207] != 0.0)) && ((s.v[1219] != 0.0) && (!(((((s.v[1214] != 0.0) || (s.v[1215] != 0.0)) || (s.v[1216] != 0.0)) || (s.v[1217] != 0.0)) || (s.v[1218] != 0.0))))) && (s.v[1329] != 0.0)) && (!(s.v[1330] != 0.0))) && (s.v[1336] != 0.0)) && (!(s.v[1338] != 0.0))) {
            s.store_div_from_scalar_ad(973, (p.p438 * s.v[449]), A::scale(s.ad_value(971), s.v[183]));
        }

        s.v[1340] = if ((s.v[971] == 0.0) || (s.v[449] == 0.0)) { 1.0 } else { 0.0 };

        if (((((((!(s.v[1206] != 0.0)) && (s.v[1207] != 0.0)) && ((s.v[1219] != 0.0) && (!(((((s.v[1214] != 0.0) || (s.v[1215] != 0.0)) || (s.v[1216] != 0.0)) || (s.v[1217] != 0.0)) || (s.v[1218] != 0.0))))) && (s.v[1329] != 0.0)) && (!(s.v[1330] != 0.0))) && ((s.v[1337] != 0.0) && (!(s.v[1336] != 0.0)))) && (s.v[1340] != 0.0)) {
            s.store_scalar(973, 0.0);
        }

        if (((((((!(s.v[1206] != 0.0)) && (s.v[1207] != 0.0)) && ((s.v[1219] != 0.0) && (!(((((s.v[1214] != 0.0) || (s.v[1215] != 0.0)) || (s.v[1216] != 0.0)) || (s.v[1217] != 0.0)) || (s.v[1218] != 0.0))))) && (s.v[1329] != 0.0)) && (!(s.v[1330] != 0.0))) && ((s.v[1337] != 0.0) && (!(s.v[1336] != 0.0)))) && (!(s.v[1340] != 0.0))) {
            s.store_div_from_scalar_ad(973, (p.p438 * s.v[183]), A::scale(s.ad_value(971), (6.0 * s.v[449])));
        }

        if ((((((!(s.v[1206] != 0.0)) && (s.v[1207] != 0.0)) && ((s.v[1219] != 0.0) && (!(((((s.v[1214] != 0.0) || (s.v[1215] != 0.0)) || (s.v[1216] != 0.0)) || (s.v[1217] != 0.0)) || (s.v[1218] != 0.0))))) && (s.v[1329] != 0.0)) && (!(s.v[1330] != 0.0))) && (!((s.v[1336] != 0.0) || (s.v[1337] != 0.0)))) {
            s.store_scalar(973, 0.0);
        }

        s.v[1341] = if (s.v[969] == 0.0) { 1.0 } else { 0.0 };

        if (((((!(s.v[1206] != 0.0)) && (s.v[1207] != 0.0)) && ((s.v[1219] != 0.0) && (!(((((s.v[1214] != 0.0) || (s.v[1215] != 0.0)) || (s.v[1216] != 0.0)) || (s.v[1217] != 0.0)) || (s.v[1218] != 0.0))))) && (!(s.v[1329] != 0.0))) && (s.v[1341] != 0.0)) {
            s.store_scalar(973, 0.0);
        }

        if (((((!(s.v[1206] != 0.0)) && (s.v[1207] != 0.0)) && ((s.v[1219] != 0.0) && (!(((((s.v[1214] != 0.0) || (s.v[1215] != 0.0)) || (s.v[1216] != 0.0)) || (s.v[1217] != 0.0)) || (s.v[1218] != 0.0))))) && (!(s.v[1329] != 0.0))) && (!(s.v[1341] != 0.0))) {
            s.store_div_from_scalar_ad(973, (p.p438 * s.v[450]), A::scale(s.ad_value(969), s.v[183]));
        }

        s.v[1342] = if (1.0 == 1.0) { 1.0 } else { 0.0 };

        if ((((!(s.v[1206] != 0.0)) && (s.v[1207] != 0.0)) && ((s.v[1220] != 0.0) && (!((((((s.v[1214] != 0.0) || (s.v[1215] != 0.0)) || (s.v[1216] != 0.0)) || (s.v[1217] != 0.0)) || (s.v[1218] != 0.0)) || (s.v[1219] != 0.0))))) && (s.v[1342] != 0.0)) {
            s.store_scalar(973, ((p.p438 * s.v[450]) / s.v[183]));
        }

        s.v[1343] = if (0.0 == 1.0) { 1.0 } else { 0.0 };

        s.v[1344] = if (((p.p9 == 1.0) || (p.p9 == 2.0)) || (p.p9 == 5.0)) { 1.0 } else { 0.0 };

        s.v[1345] = if (((p.p9 == 3.0) || (p.p9 == 4.0)) || (p.p9 == 6.0)) { 1.0 } else { 0.0 };

        s.v[1346] = if (s.v[969] == 0.0) { 1.0 } else { 0.0 };

        if (((((((!(s.v[1206] != 0.0)) && (s.v[1207] != 0.0)) && ((s.v[1220] != 0.0) && (!((((((s.v[1214] != 0.0) || (s.v[1215] != 0.0)) || (s.v[1216] != 0.0)) || (s.v[1217] != 0.0)) || (s.v[1218] != 0.0)) || (s.v[1219] != 0.0))))) && (!(s.v[1342] != 0.0))) && (s.v[1343] != 0.0)) && (s.v[1344] != 0.0)) && (s.v[1346] != 0.0)) {
            s.store_scalar(973, 0.0);
        }

        if (((((((!(s.v[1206] != 0.0)) && (s.v[1207] != 0.0)) && ((s.v[1220] != 0.0) && (!((((((s.v[1214] != 0.0) || (s.v[1215] != 0.0)) || (s.v[1216] != 0.0)) || (s.v[1217] != 0.0)) || (s.v[1218] != 0.0)) || (s.v[1219] != 0.0))))) && (!(s.v[1342] != 0.0))) && (s.v[1343] != 0.0)) && (s.v[1344] != 0.0)) && (!(s.v[1346] != 0.0))) {
            s.store_div_from_scalar_ad(973, (p.p438 * s.v[449]), A::scale(s.ad_value(969), s.v[183]));
        }

        s.v[1348] = if ((s.v[969] == 0.0) || ((s.v[449] + s.v[451]) == 0.0)) { 1.0 } else { 0.0 };

        if (((((((!(s.v[1206] != 0.0)) && (s.v[1207] != 0.0)) && ((s.v[1220] != 0.0) && (!((((((s.v[1214] != 0.0) || (s.v[1215] != 0.0)) || (s.v[1216] != 0.0)) || (s.v[1217] != 0.0)) || (s.v[1218] != 0.0)) || (s.v[1219] != 0.0))))) && (!(s.v[1342] != 0.0))) && (s.v[1343] != 0.0)) && ((s.v[1345] != 0.0) && (!(s.v[1344] != 0.0)))) && (s.v[1348] != 0.0)) {
            s.store_scalar(973, 0.0);
        }

        if (((((((!(s.v[1206] != 0.0)) && (s.v[1207] != 0.0)) && ((s.v[1220] != 0.0) && (!((((((s.v[1214] != 0.0) || (s.v[1215] != 0.0)) || (s.v[1216] != 0.0)) || (s.v[1217] != 0.0)) || (s.v[1218] != 0.0)) || (s.v[1219] != 0.0))))) && (!(s.v[1342] != 0.0))) && (s.v[1343] != 0.0)) && ((s.v[1345] != 0.0) && (!(s.v[1344] != 0.0)))) && (!(s.v[1348] != 0.0))) {
            s.store_div_from_scalar_ad(973, (p.p438 * s.v[183]), A::scale(s.ad_value(969), (3.0 * (s.v[449] + s.v[451]))));
        }

        if ((((((!(s.v[1206] != 0.0)) && (s.v[1207] != 0.0)) && ((s.v[1220] != 0.0) && (!((((((s.v[1214] != 0.0) || (s.v[1215] != 0.0)) || (s.v[1216] != 0.0)) || (s.v[1217] != 0.0)) || (s.v[1218] != 0.0)) || (s.v[1219] != 0.0))))) && (!(s.v[1342] != 0.0))) && (s.v[1343] != 0.0)) && (!((s.v[1344] != 0.0) || (s.v[1345] != 0.0)))) {
            s.store_scalar(973, 0.0);
        }

        s.v[1349] = if (((p.p9 == 1.0) || (p.p9 == 3.0)) || (p.p9 == 7.0)) { 1.0 } else { 0.0 };

        s.v[1350] = if (((p.p9 == 2.0) || (p.p9 == 4.0)) || (p.p9 == 8.0)) { 1.0 } else { 0.0 };

        s.v[1351] = if (s.v[969] == 0.0) { 1.0 } else { 0.0 };

        if (((((((!(s.v[1206] != 0.0)) && (s.v[1207] != 0.0)) && ((s.v[1220] != 0.0) && (!((((((s.v[1214] != 0.0) || (s.v[1215] != 0.0)) || (s.v[1216] != 0.0)) || (s.v[1217] != 0.0)) || (s.v[1218] != 0.0)) || (s.v[1219] != 0.0))))) && (!(s.v[1342] != 0.0))) && (!(s.v[1343] != 0.0))) && (s.v[1349] != 0.0)) && (s.v[1351] != 0.0)) {
            s.store_scalar(973, 0.0);
        }

        if (((((((!(s.v[1206] != 0.0)) && (s.v[1207] != 0.0)) && ((s.v[1220] != 0.0) && (!((((((s.v[1214] != 0.0) || (s.v[1215] != 0.0)) || (s.v[1216] != 0.0)) || (s.v[1217] != 0.0)) || (s.v[1218] != 0.0)) || (s.v[1219] != 0.0))))) && (!(s.v[1342] != 0.0))) && (!(s.v[1343] != 0.0))) && (s.v[1349] != 0.0)) && (!(s.v[1351] != 0.0))) {
            s.store_div_from_scalar_ad(973, (p.p438 * s.v[449]), A::scale(s.ad_value(969), s.v[183]));
        }

        s.v[1353] = if ((s.v[969] == 0.0) || ((s.v[449] + s.v[451]) == 0.0)) { 1.0 } else { 0.0 };

        if (((((((!(s.v[1206] != 0.0)) && (s.v[1207] != 0.0)) && ((s.v[1220] != 0.0) && (!((((((s.v[1214] != 0.0) || (s.v[1215] != 0.0)) || (s.v[1216] != 0.0)) || (s.v[1217] != 0.0)) || (s.v[1218] != 0.0)) || (s.v[1219] != 0.0))))) && (!(s.v[1342] != 0.0))) && (!(s.v[1343] != 0.0))) && ((s.v[1350] != 0.0) && (!(s.v[1349] != 0.0)))) && (s.v[1353] != 0.0)) {
            s.store_scalar(973, 0.0);
        }

        if (((((((!(s.v[1206] != 0.0)) && (s.v[1207] != 0.0)) && ((s.v[1220] != 0.0) && (!((((((s.v[1214] != 0.0) || (s.v[1215] != 0.0)) || (s.v[1216] != 0.0)) || (s.v[1217] != 0.0)) || (s.v[1218] != 0.0)) || (s.v[1219] != 0.0))))) && (!(s.v[1342] != 0.0))) && (!(s.v[1343] != 0.0))) && ((s.v[1350] != 0.0) && (!(s.v[1349] != 0.0)))) && (!(s.v[1353] != 0.0))) {
            s.store_div_from_scalar_ad(973, (p.p438 * s.v[183]), A::scale(s.ad_value(969), (3.0 * (s.v[449] + s.v[451]))));
        }

        if ((((((!(s.v[1206] != 0.0)) && (s.v[1207] != 0.0)) && ((s.v[1220] != 0.0) && (!((((((s.v[1214] != 0.0) || (s.v[1215] != 0.0)) || (s.v[1216] != 0.0)) || (s.v[1217] != 0.0)) || (s.v[1218] != 0.0)) || (s.v[1219] != 0.0))))) && (!(s.v[1342] != 0.0))) && (!(s.v[1343] != 0.0))) && (!((s.v[1349] != 0.0) || (s.v[1350] != 0.0)))) {
            s.store_scalar(973, 0.0);
        }

        s.v[1354] = if (1.0 == 1.0) { 1.0 } else { 0.0 };

        s.v[1355] = if (s.v[971] == 0.0) { 1.0 } else { 0.0 };

        if (((((!(s.v[1206] != 0.0)) && (s.v[1207] != 0.0)) && ((s.v[1221] != 0.0) && (!(((((((s.v[1214] != 0.0) || (s.v[1215] != 0.0)) || (s.v[1216] != 0.0)) || (s.v[1217] != 0.0)) || (s.v[1218] != 0.0)) || (s.v[1219] != 0.0)) || (s.v[1220] != 0.0))))) && (s.v[1354] != 0.0)) && (s.v[1355] != 0.0)) {
            s.store_scalar(973, 0.0);
        }

        if (((((!(s.v[1206] != 0.0)) && (s.v[1207] != 0.0)) && ((s.v[1221] != 0.0) && (!(((((((s.v[1214] != 0.0) || (s.v[1215] != 0.0)) || (s.v[1216] != 0.0)) || (s.v[1217] != 0.0)) || (s.v[1218] != 0.0)) || (s.v[1219] != 0.0)) || (s.v[1220] != 0.0))))) && (s.v[1354] != 0.0)) && (!(s.v[1355] != 0.0))) {
            s.store_div_from_scalar_ad(973, (p.p438 * s.v[450]), A::scale(s.ad_value(971), s.v[183]));
        }

    }
}
