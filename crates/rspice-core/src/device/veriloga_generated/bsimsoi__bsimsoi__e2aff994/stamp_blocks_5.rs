#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, ReactiveScratch, Scratch, THERMAL_VOLTAGE_PER_K};
use super::super::state::Instance;

impl Instance {

    pub(super) fn stamp_reactive_block_36(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
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

        if ((!(s.v[1620] != 0.0)) && (s.v[1950] != 0.0)) {
            s.store_offset_ad(991, A::scale(A::sqrt(A::offset(A::scale(A::add(A::add(A::square(s.ad_value(320)), s.ad_value(320)), s.ad_value(990)), 4.0), 1.0)), 0.5), (-0.5));
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
            s.store_square(633, 172);
        }

        if (!(s.v[1620] != 0.0)) {
            s.store_square(632, 171);
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

        if ((!(s.v[1620] != 0.0)) && ((s.v[1965] != 0.0) && (!(s.v[1964] != 0.0)))) {
            s.store_mul_ad_lhs(626, A::scale(s.ad_value(253), 2.0), 269);
        }

        if ((!(s.v[1620] != 0.0)) && ((s.v[1965] != 0.0) && (!(s.v[1964] != 0.0)))) {
            s.store_mul_ad_lhs(167, A::scale(A::mul(A::mul(s.ad_value(337), s.ad_value(345)), s.ad_value(363)), s.v[199]), 626);
        }

    }

    pub(super) fn stamp_reactive_block_37(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
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
            let assign54870_ad_e89925: A = A::mul(A::scale(A::mul(A::mul(A::mul(s.ad_value(625), s.ad_value(177)), s.ad_value(177)), A::add(A::sub(A::div(s.ad_value(168), s.ad_value(171)), A::div(s.ad_value(176), A::mul(A::scale(s.ad_value(171), 60.0), s.ad_value(171)))), A::div(A::square(s.ad_value(174)), A::mul(A::scale(s.ad_value(171), 144.0), s.ad_value(172))))), (15.0 * 0.25)), s.ad_value(632));
            s.store_div_ad(622, assign54870_ad_e89925, A::scale(s.ad_value(167), ((p.p2 * s.v[183]) * 12.0)));
        }

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

    }

    pub(super) fn stamp_reactive_block_38(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
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

    }

    pub(super) fn stamp_reactive_block_39(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
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

    }

    pub(super) fn stamp_reactive_block_40(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
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

    }

    pub(super) fn stamp_reactive_block_41(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
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

        s.v[2036] = if (p.p1374 < 0.001) { 1.0 } else { 0.0 };

        if (s.v[2036] != 0.0) {
            s.store_scalar(167, (1.0 / 0.001));
        }

    }

    pub(super) fn stamp_transient_equation_0_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq0_e1463,) = {
    if (!(s.v[1556] != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq0_value: f64 = eq0_e1463;
        stamper.stamp_potential(
            branches[0],
            eq0_value,
            &[
            ],
        );
    }

    pub(super) fn stamp_transient_equation_1_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq1_e1472,) = {
    if (s.v[1620] != 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq1_value: f64 = eq1_e1472;
        stamper.stamp_current(
            Some(nodes[6]),
            Some(nodes[7]),
            self.multiplicity * (eq1_value),
            &[
            ],
        );
    }

    pub(super) fn stamp_transient_equation_2_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq2_e1480,) = {
    if ((s.v[1620] != 0.0) && (s.v[1793] != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq2_value: f64 = eq2_e1480;
        stamper.stamp_current(
            Some(nodes[6]),
            Some(nodes[7]),
            self.multiplicity * (eq2_value),
            &[
            ],
        );
    }

    pub(super) fn stamp_transient_equation_3_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq3_e1486,) = {
    if ((s.v[1620] != 0.0) && (s.v[1793] != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq3_value: f64 = eq3_e1486;
        stamper.stamp_potential(
            branches[1],
            eq3_value,
            &[
            ],
        );
    }

    pub(super) fn stamp_transient_equation_4_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq4_e1497,) = {
    if ((s.v[1620] != 0.0) && ((s.v[1794] != 0.0) && (!(s.v[1793] != 0.0)))) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq4_value: f64 = eq4_e1497;
        stamper.stamp_current(
            Some(nodes[13]),
            None,
            self.multiplicity * (eq4_value),
            &[
            ],
        );
    }

    pub(super) fn stamp_transient_equation_5_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq5_e1514,) = {
    if ((s.v[1620] != 0.0) && ((s.v[1794] != 0.0) && (!(s.v[1793] != 0.0)))) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq5_value: f64 = eq5_e1514;
        stamper.stamp_current(
            Some(nodes[12]),
            None,
            self.multiplicity * (eq5_value),
            &[
            ],
        );
    }

    pub(super) fn stamp_transient_equation_6_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let nv13 = ctx.node_voltage(nodes[13]);
        let (eq6_e1526, eq6_e1526_d_n0, eq6_e1526_d_n1, eq6_e1526_d_n2, eq6_e1526_d_n3, eq6_e1526_d_n4, eq6_e1526_d_n5, eq6_e1526_d_n6, eq6_e1526_d_n7, eq6_e1526_d_n8, eq6_e1526_d_n9, eq6_e1526_d_n10, eq6_e1526_d_n11, eq6_e1526_d_n12, eq6_e1526_d_n13,) = {
    if ((s.v[1620] != 0.0) && ((s.v[1794] != 0.0) && (!(s.v[1793] != 0.0)))) {
        let eq6_e1522: f64 = (-s.v[629]);
        let eq6_e1522_d_n0: f64 = (-s.dn[629][0]);
        let eq6_e1522_d_n1: f64 = (-s.dn[629][1]);
        let eq6_e1522_d_n2: f64 = (-s.dn[629][2]);
        let eq6_e1522_d_n3: f64 = (-s.dn[629][3]);
        let eq6_e1522_d_n4: f64 = (-s.dn[629][4]);
        let eq6_e1522_d_n5: f64 = (-s.dn[629][5]);
        let eq6_e1522_d_n6: f64 = (-s.dn[629][6]);
        let eq6_e1522_d_n7: f64 = (-s.dn[629][7]);
        let eq6_e1522_d_n8: f64 = (-s.dn[629][8]);
        let eq6_e1522_d_n9: f64 = (-s.dn[629][9]);
        let eq6_e1522_d_n10: f64 = (-s.dn[629][10]);
        let eq6_e1522_d_n11: f64 = (-s.dn[629][11]);
        let eq6_e1522_d_n12: f64 = (-s.dn[629][12]);
        let eq6_e1522_d_n13: f64 = (-s.dn[629][13]);
        let eq6_e1524: f64 = (eq6_e1522 * (nv13 - 0.0));
        let eq6_e1524_d_n0: f64 = (eq6_e1522_d_n0 * (nv13 - 0.0));
        let eq6_e1524_d_n1: f64 = (eq6_e1522_d_n1 * (nv13 - 0.0));
        let eq6_e1524_d_n2: f64 = (eq6_e1522_d_n2 * (nv13 - 0.0));
        let eq6_e1524_d_n3: f64 = (eq6_e1522_d_n3 * (nv13 - 0.0));
        let eq6_e1524_d_n4: f64 = (eq6_e1522_d_n4 * (nv13 - 0.0));
        let eq6_e1524_d_n5: f64 = (eq6_e1522_d_n5 * (nv13 - 0.0));
        let eq6_e1524_d_n6: f64 = (eq6_e1522_d_n6 * (nv13 - 0.0));
        let eq6_e1524_d_n7: f64 = (eq6_e1522_d_n7 * (nv13 - 0.0));
        let eq6_e1524_d_n8: f64 = (eq6_e1522_d_n8 * (nv13 - 0.0));
        let eq6_e1524_d_n9: f64 = (eq6_e1522_d_n9 * (nv13 - 0.0));
        let eq6_e1524_d_n10: f64 = (eq6_e1522_d_n10 * (nv13 - 0.0));
        let eq6_e1524_d_n11: f64 = (eq6_e1522_d_n11 * (nv13 - 0.0));
        let eq6_e1524_d_n12: f64 = (eq6_e1522_d_n12 * (nv13 - 0.0));
        let eq6_e1524_d_n13: f64 = ((eq6_e1522_d_n13 * (nv13 - 0.0)) + eq6_e1522);
        (eq6_e1524, eq6_e1524_d_n0, eq6_e1524_d_n1, eq6_e1524_d_n2, eq6_e1524_d_n3, eq6_e1524_d_n4, eq6_e1524_d_n5, eq6_e1524_d_n6, eq6_e1524_d_n7, eq6_e1524_d_n8, eq6_e1524_d_n9, eq6_e1524_d_n10, eq6_e1524_d_n11, eq6_e1524_d_n12, eq6_e1524_d_n13,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq6_value: f64 = eq6_e1526;
        let eq6_node_derivatives: [f64; 14] = [eq6_e1526_d_n0, eq6_e1526_d_n1, eq6_e1526_d_n2, eq6_e1526_d_n3, eq6_e1526_d_n4, eq6_e1526_d_n5, eq6_e1526_d_n6, eq6_e1526_d_n7, eq6_e1526_d_n8, eq6_e1526_d_n9, eq6_e1526_d_n10, eq6_e1526_d_n11, eq6_e1526_d_n12, eq6_e1526_d_n13];
        let eq6_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[12]),
            None,
            self.multiplicity * (eq6_value),
            &nodes,
            &eq6_node_derivatives,
            &branches,
            &eq6_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_7_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let nv12 = ctx.node_voltage(nodes[12]);
        let (eq7_e1546, eq7_e1546_d_n0, eq7_e1546_d_n1, eq7_e1546_d_n2, eq7_e1546_d_n3, eq7_e1546_d_n4, eq7_e1546_d_n5, eq7_e1546_d_n6, eq7_e1546_d_n7, eq7_e1546_d_n8, eq7_e1546_d_n9, eq7_e1546_d_n10, eq7_e1546_d_n11, eq7_e1546_d_n12, eq7_e1546_d_n13,) = {
    if ((s.v[1620] != 0.0) && ((s.v[1794] != 0.0) && (!(s.v[1793] != 0.0)))) {
        let eq7_e1535: f64 = (s.v[622] * s.v[199]);
        let eq7_e1535_d_n0: f64 = (s.dn[622][0] * s.v[199]);
        let eq7_e1535_d_n1: f64 = (s.dn[622][1] * s.v[199]);
        let eq7_e1535_d_n2: f64 = (s.dn[622][2] * s.v[199]);
        let eq7_e1535_d_n3: f64 = (s.dn[622][3] * s.v[199]);
        let eq7_e1535_d_n4: f64 = (s.dn[622][4] * s.v[199]);
        let eq7_e1535_d_n5: f64 = (s.dn[622][5] * s.v[199]);
        let eq7_e1535_d_n6: f64 = (s.dn[622][6] * s.v[199]);
        let eq7_e1535_d_n7: f64 = (s.dn[622][7] * s.v[199]);
        let eq7_e1535_d_n8: f64 = (s.dn[622][8] * s.v[199]);
        let eq7_e1535_d_n9: f64 = (s.dn[622][9] * s.v[199]);
        let eq7_e1535_d_n10: f64 = (s.dn[622][10] * s.v[199]);
        let eq7_e1535_d_n11: f64 = (s.dn[622][11] * s.v[199]);
        let eq7_e1535_d_n12: f64 = (s.dn[622][12] * s.v[199]);
        let eq7_e1535_d_n13: f64 = (s.dn[622][13] * s.v[199]);
        let eq7_e1537: f64 = (eq7_e1535 * s.v[183]);
        let eq7_e1537_d_n0: f64 = (eq7_e1535_d_n0 * s.v[183]);
        let eq7_e1537_d_n1: f64 = (eq7_e1535_d_n1 * s.v[183]);
        let eq7_e1537_d_n2: f64 = (eq7_e1535_d_n2 * s.v[183]);
        let eq7_e1537_d_n3: f64 = (eq7_e1535_d_n3 * s.v[183]);
        let eq7_e1537_d_n4: f64 = (eq7_e1535_d_n4 * s.v[183]);
        let eq7_e1537_d_n5: f64 = (eq7_e1535_d_n5 * s.v[183]);
        let eq7_e1537_d_n6: f64 = (eq7_e1535_d_n6 * s.v[183]);
        let eq7_e1537_d_n7: f64 = (eq7_e1535_d_n7 * s.v[183]);
        let eq7_e1537_d_n8: f64 = (eq7_e1535_d_n8 * s.v[183]);
        let eq7_e1537_d_n9: f64 = (eq7_e1535_d_n9 * s.v[183]);
        let eq7_e1537_d_n10: f64 = (eq7_e1535_d_n10 * s.v[183]);
        let eq7_e1537_d_n11: f64 = (eq7_e1535_d_n11 * s.v[183]);
        let eq7_e1537_d_n12: f64 = (eq7_e1535_d_n12 * s.v[183]);
        let eq7_e1537_d_n13: f64 = (eq7_e1535_d_n13 * s.v[183]);
        let eq7_e1539: f64 = (eq7_e1537 * p.p2);
        let eq7_e1539_d_n0: f64 = (eq7_e1537_d_n0 * p.p2);
        let eq7_e1539_d_n1: f64 = (eq7_e1537_d_n1 * p.p2);
        let eq7_e1539_d_n2: f64 = (eq7_e1537_d_n2 * p.p2);
        let eq7_e1539_d_n3: f64 = (eq7_e1537_d_n3 * p.p2);
        let eq7_e1539_d_n4: f64 = (eq7_e1537_d_n4 * p.p2);
        let eq7_e1539_d_n5: f64 = (eq7_e1537_d_n5 * p.p2);
        let eq7_e1539_d_n6: f64 = (eq7_e1537_d_n6 * p.p2);
        let eq7_e1539_d_n7: f64 = (eq7_e1537_d_n7 * p.p2);
        let eq7_e1539_d_n8: f64 = (eq7_e1537_d_n8 * p.p2);
        let eq7_e1539_d_n9: f64 = (eq7_e1537_d_n9 * p.p2);
        let eq7_e1539_d_n10: f64 = (eq7_e1537_d_n10 * p.p2);
        let eq7_e1539_d_n11: f64 = (eq7_e1537_d_n11 * p.p2);
        let eq7_e1539_d_n12: f64 = (eq7_e1537_d_n12 * p.p2);
        let eq7_e1539_d_n13: f64 = (eq7_e1537_d_n13 * p.p2);
        let eq7_e1541: f64 = (eq7_e1539 * s.v[184]);
        let eq7_e1541_d_n0: f64 = (eq7_e1539_d_n0 * s.v[184]);
        let eq7_e1541_d_n1: f64 = (eq7_e1539_d_n1 * s.v[184]);
        let eq7_e1541_d_n2: f64 = (eq7_e1539_d_n2 * s.v[184]);
        let eq7_e1541_d_n3: f64 = (eq7_e1539_d_n3 * s.v[184]);
        let eq7_e1541_d_n4: f64 = (eq7_e1539_d_n4 * s.v[184]);
        let eq7_e1541_d_n5: f64 = (eq7_e1539_d_n5 * s.v[184]);
        let eq7_e1541_d_n6: f64 = (eq7_e1539_d_n6 * s.v[184]);
        let eq7_e1541_d_n7: f64 = (eq7_e1539_d_n7 * s.v[184]);
        let eq7_e1541_d_n8: f64 = (eq7_e1539_d_n8 * s.v[184]);
        let eq7_e1541_d_n9: f64 = (eq7_e1539_d_n9 * s.v[184]);
        let eq7_e1541_d_n10: f64 = (eq7_e1539_d_n10 * s.v[184]);
        let eq7_e1541_d_n11: f64 = (eq7_e1539_d_n11 * s.v[184]);
        let eq7_e1541_d_n12: f64 = (eq7_e1539_d_n12 * s.v[184]);
        let eq7_e1541_d_n13: f64 = (eq7_e1539_d_n13 * s.v[184]);
        let eq7_e1543: f64 = (eq7_e1541 * (nv12 - 0.0));
        let eq7_e1543_d_n0: f64 = (eq7_e1541_d_n0 * (nv12 - 0.0));
        let eq7_e1543_d_n1: f64 = (eq7_e1541_d_n1 * (nv12 - 0.0));
        let eq7_e1543_d_n2: f64 = (eq7_e1541_d_n2 * (nv12 - 0.0));
        let eq7_e1543_d_n3: f64 = (eq7_e1541_d_n3 * (nv12 - 0.0));
        let eq7_e1543_d_n4: f64 = (eq7_e1541_d_n4 * (nv12 - 0.0));
        let eq7_e1543_d_n5: f64 = (eq7_e1541_d_n5 * (nv12 - 0.0));
        let eq7_e1543_d_n6: f64 = (eq7_e1541_d_n6 * (nv12 - 0.0));
        let eq7_e1543_d_n7: f64 = (eq7_e1541_d_n7 * (nv12 - 0.0));
        let eq7_e1543_d_n8: f64 = (eq7_e1541_d_n8 * (nv12 - 0.0));
        let eq7_e1543_d_n9: f64 = (eq7_e1541_d_n9 * (nv12 - 0.0));
        let eq7_e1543_d_n10: f64 = (eq7_e1541_d_n10 * (nv12 - 0.0));
        let eq7_e1543_d_n11: f64 = (eq7_e1541_d_n11 * (nv12 - 0.0));
        let eq7_e1543_d_n12: f64 = ((eq7_e1541_d_n12 * (nv12 - 0.0)) + eq7_e1541);
        let eq7_e1543_d_n13: f64 = (eq7_e1541_d_n13 * (nv12 - 0.0));
        let eq7_e1544: f64 = self.eval_ddt(0, eq7_e1543);
        let eq7_e1544_d_n0: f64 = self.ddt_jacobian(eq7_e1543_d_n0);
        let eq7_e1544_d_n1: f64 = self.ddt_jacobian(eq7_e1543_d_n1);
        let eq7_e1544_d_n2: f64 = self.ddt_jacobian(eq7_e1543_d_n2);
        let eq7_e1544_d_n3: f64 = self.ddt_jacobian(eq7_e1543_d_n3);
        let eq7_e1544_d_n4: f64 = self.ddt_jacobian(eq7_e1543_d_n4);
        let eq7_e1544_d_n5: f64 = self.ddt_jacobian(eq7_e1543_d_n5);
        let eq7_e1544_d_n6: f64 = self.ddt_jacobian(eq7_e1543_d_n6);
        let eq7_e1544_d_n7: f64 = self.ddt_jacobian(eq7_e1543_d_n7);
        let eq7_e1544_d_n8: f64 = self.ddt_jacobian(eq7_e1543_d_n8);
        let eq7_e1544_d_n9: f64 = self.ddt_jacobian(eq7_e1543_d_n9);
        let eq7_e1544_d_n10: f64 = self.ddt_jacobian(eq7_e1543_d_n10);
        let eq7_e1544_d_n11: f64 = self.ddt_jacobian(eq7_e1543_d_n11);
        let eq7_e1544_d_n12: f64 = self.ddt_jacobian(eq7_e1543_d_n12);
        let eq7_e1544_d_n13: f64 = self.ddt_jacobian(eq7_e1543_d_n13);
        (eq7_e1544, eq7_e1544_d_n0, eq7_e1544_d_n1, eq7_e1544_d_n2, eq7_e1544_d_n3, eq7_e1544_d_n4, eq7_e1544_d_n5, eq7_e1544_d_n6, eq7_e1544_d_n7, eq7_e1544_d_n8, eq7_e1544_d_n9, eq7_e1544_d_n10, eq7_e1544_d_n11, eq7_e1544_d_n12, eq7_e1544_d_n13,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq7_value: f64 = eq7_e1546;
        let eq7_node_derivatives: [f64; 14] = [eq7_e1546_d_n0, eq7_e1546_d_n1, eq7_e1546_d_n2, eq7_e1546_d_n3, eq7_e1546_d_n4, eq7_e1546_d_n5, eq7_e1546_d_n6, eq7_e1546_d_n7, eq7_e1546_d_n8, eq7_e1546_d_n9, eq7_e1546_d_n10, eq7_e1546_d_n11, eq7_e1546_d_n12, eq7_e1546_d_n13];
        let eq7_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[12]),
            None,
            self.multiplicity * (eq7_value),
            &nodes,
            &eq7_node_derivatives,
            &branches,
            &eq7_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_8_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq8_e1563,) = {
    if ((s.v[1620] != 0.0) && ((s.v[1794] != 0.0) && (!(s.v[1793] != 0.0)))) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq8_value: f64 = eq8_e1563;
        stamper.stamp_current(
            Some(nodes[6]),
            Some(nodes[7]),
            self.multiplicity * (eq8_value),
            &[
            ],
        );
    }

    pub(super) fn stamp_transient_equation_9_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let nv13 = ctx.node_voltage(nodes[13]);
        let (eq9_e1574, eq9_e1574_d_n0, eq9_e1574_d_n1, eq9_e1574_d_n2, eq9_e1574_d_n3, eq9_e1574_d_n4, eq9_e1574_d_n5, eq9_e1574_d_n6, eq9_e1574_d_n7, eq9_e1574_d_n8, eq9_e1574_d_n9, eq9_e1574_d_n10, eq9_e1574_d_n11, eq9_e1574_d_n12, eq9_e1574_d_n13,) = {
    if ((s.v[1620] != 0.0) && ((s.v[1794] != 0.0) && (!(s.v[1793] != 0.0)))) {
        let eq9_e1572: f64 = (s.v[628] * (nv13 - 0.0));
        let eq9_e1572_d_n0: f64 = (s.dn[628][0] * (nv13 - 0.0));
        let eq9_e1572_d_n1: f64 = (s.dn[628][1] * (nv13 - 0.0));
        let eq9_e1572_d_n2: f64 = (s.dn[628][2] * (nv13 - 0.0));
        let eq9_e1572_d_n3: f64 = (s.dn[628][3] * (nv13 - 0.0));
        let eq9_e1572_d_n4: f64 = (s.dn[628][4] * (nv13 - 0.0));
        let eq9_e1572_d_n5: f64 = (s.dn[628][5] * (nv13 - 0.0));
        let eq9_e1572_d_n6: f64 = (s.dn[628][6] * (nv13 - 0.0));
        let eq9_e1572_d_n7: f64 = (s.dn[628][7] * (nv13 - 0.0));
        let eq9_e1572_d_n8: f64 = (s.dn[628][8] * (nv13 - 0.0));
        let eq9_e1572_d_n9: f64 = (s.dn[628][9] * (nv13 - 0.0));
        let eq9_e1572_d_n10: f64 = (s.dn[628][10] * (nv13 - 0.0));
        let eq9_e1572_d_n11: f64 = (s.dn[628][11] * (nv13 - 0.0));
        let eq9_e1572_d_n12: f64 = (s.dn[628][12] * (nv13 - 0.0));
        let eq9_e1572_d_n13: f64 = ((s.dn[628][13] * (nv13 - 0.0)) + s.v[628]);
        (eq9_e1572, eq9_e1572_d_n0, eq9_e1572_d_n1, eq9_e1572_d_n2, eq9_e1572_d_n3, eq9_e1572_d_n4, eq9_e1572_d_n5, eq9_e1572_d_n6, eq9_e1572_d_n7, eq9_e1572_d_n8, eq9_e1572_d_n9, eq9_e1572_d_n10, eq9_e1572_d_n11, eq9_e1572_d_n12, eq9_e1572_d_n13,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq9_value: f64 = eq9_e1574;
        let eq9_node_derivatives: [f64; 14] = [eq9_e1574_d_n0, eq9_e1574_d_n1, eq9_e1574_d_n2, eq9_e1574_d_n3, eq9_e1574_d_n4, eq9_e1574_d_n5, eq9_e1574_d_n6, eq9_e1574_d_n7, eq9_e1574_d_n8, eq9_e1574_d_n9, eq9_e1574_d_n10, eq9_e1574_d_n11, eq9_e1574_d_n12, eq9_e1574_d_n13];
        let eq9_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[6]),
            Some(nodes[7]),
            self.multiplicity * (eq9_value),
            &nodes,
            &eq9_node_derivatives,
            &branches,
            &eq9_branch_derivatives,
            self.multiplicity,
        );
    }
}
