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
        if ((!(s.v[971] != 0.0)) && (!(s.v[973] != 0.0))) {
            s.store_scale_ad(280, A::div_from_scalar(s.v[628], s.ad_value(245)), (1.6021918e-19 * s.v[466]));
        }

        if ((!(s.v[971] != 0.0)) && (!(s.v[973] != 0.0))) {
            s.store_mul_ad(258, A::mul(A::mul(s.ad_value(280), s.ad_value(249)), s.ad_value(249)), A::exp(s.ad_value(278)));
        }

        if ((!(s.v[971] != 0.0)) && (!(s.v[973] != 0.0))) {
            s.store_div_ad_rhs(258, 258, A::offset(A::exp(A::mul(s.ad_value(120), s.ad_value(51))), 1.0));
        }

        if ((!(s.v[971] != 0.0)) && (!(s.v[973] != 0.0))) {
            s.store_div_ad_rhs(258, 258, A::sub_from_scalar(1.0, A::exp(A::div(A::neg(s.ad_value(123)), s.ad_value(629)))));
        }

        s.v[264] = p.p176;

        s.v[261] = 0.0;

        s.v[974] = if (s.v[34] != 0.0) { 1.0 } else { 0.0 };

        if (s.v[974] != 0.0) {
            s.store_add(280, 51, 56);
        }

        if (s.v[974] != 0.0) {
            s.store_add_ad(260, A::scale(s.ad_value(280), s.v[264]), A::scale(s.ad_value(57), (1.0 - s.v[264])));
        }

        s.v[975] = if (s.v[260] > ((s.v[56] + s.v[51]) - (10.0 * 2.220446049250313e-16))) { 1.0 } else { 0.0 };

        if ((s.v[974] != 0.0) && (s.v[975] != 0.0)) {
            s.store_offset_ad(260, A::add(s.ad_value(56), s.ad_value(51)), (-(10.0 * 2.220446049250313e-16)));
        }

        s.v[976] = if (p.p45 != 0.0) { 1.0 } else { 0.0 };

        s.v[977] = if (s.v[151] > 1e-15) { 1.0 } else { 0.0 };

        if (((!(s.v[974] != 0.0)) && (s.v[976] != 0.0)) && (s.v[977] != 0.0)) {
            s.store_div_ad_lhs(261, A::div(A::mul(s.ad_value(151), s.ad_value(122)), s.ad_value(123)), 149);
        }

        s.v[435] = s.v[273];

        s.v[436] = (1.0 / s.v[435]);

        s.v[978] = if (((p.p19 >= 1.0) && (p.p175 > 0.0)) && (s.v[624] > 0.0)) { 1.0 } else { 0.0 };

        if (s.v[978] != 0.0) {
            s.store_scalar(195, p.p175);
        }

        if (s.v[978] != 0.0) {
            s.store_mul_ad_rhs(437, 141, A::sqrt(A::div_from_scalar(s.v[624], s.ad_value(457))));
        }

        if (s.v[978] != 0.0) {
            s.store_scalar(399, ((1.0 - -1.0) / 2.0));
        }

        if (s.v[978] != 0.0) {
            s.store_scalar(400, ((1.0 + -1.0) / 2.0));
        }

        if (s.v[978] != 0.0) {
            s.store_add_ad(402, A::mul(s.ad_value(399), s.ad_value(412)), A::mul(s.ad_value(400), s.ad_value(413)));
        }

        if (s.v[978] != 0.0) {
            s.store_add_ad(403, A::mul(s.ad_value(399), s.ad_value(413)), A::mul(s.ad_value(400), s.ad_value(412)));
        }

        if ((s.v[978] != 0.0) && (s.v[399] != 0.0)) {
            s.store_add_ad(414, A::mul(s.ad_value(412), s.ad_value(42)), A::mul(s.ad_value(413), A::sub(s.ad_value(42), s.ad_value(41))));
        }

        if ((s.v[978] != 0.0) && (s.v[400] != 0.0)) {
            s.store_add_ad(414, A::mul(s.ad_value(413), s.ad_value(42)), A::mul(s.ad_value(412), A::sub(s.ad_value(42), s.ad_value(41))));
        }

        if (s.v[978] != 0.0) {
            s.store_scalar(415, 0.0);
        }

        if (s.v[978] != 0.0) {
            s.store_neg(278, 415);
        }

        s.v[979] = if (s.v[278] > s.v[31]) { 1.0 } else { 0.0 };

        if ((s.v[978] != 0.0) && (s.v[979] != 0.0)) {
            s.store_sub(279, 278, 31);
        }

        if ((s.v[978] != 0.0) && (s.v[979] != 0.0)) {
            s.store_sub_from_scalar(280, s.v[30], 31);
        }

        if ((s.v[978] != 0.0) && (s.v[979] != 0.0)) {
            s.store_div(638, 279, 280);
        }

        if ((s.v[978] != 0.0) && (s.v[979] != 0.0)) {
            s.store_square(639, 638);
        }

        if ((s.v[978] != 0.0) && (s.v[979] != 0.0)) {
            s.store_mul(640, 639, 638);
        }

        if ((s.v[978] != 0.0) && (s.v[979] != 0.0)) {
            s.store_square(641, 639);
        }

        if ((s.v[978] != 0.0) && (s.v[979] != 0.0)) {
            s.store_div_from_scalar_ad(291, 1.0, A::add(A::add(A::add(A::offset(s.ad_value(638), 1.0), s.ad_value(639)), s.ad_value(640)), s.ad_value(641)));
        }

        if ((s.v[978] != 0.0) && (s.v[979] != 0.0)) {
            s.store_mul_ad_lhs(387, A::mul(A::neg(A::add(A::add(A::offset(A::scale(s.ad_value(638), 2.0), 1.0), A::scale(s.ad_value(639), 3.0)), A::scale(s.ad_value(640), 4.0))), s.ad_value(291)), 291);
        }

        if ((s.v[978] != 0.0) && (s.v[979] != 0.0)) {
            s.store_mul_ad_rhs(291, 280, A::sub_from_scalar(1.0, s.ad_value(291)));
        }

        if ((s.v[978] != 0.0) && (s.v[979] != 0.0)) {
            s.store_neg(387, 387);
        }

        if ((s.v[978] != 0.0) && (s.v[979] != 0.0)) {
            s.store_add(288, 31, 291);
        }

        if ((s.v[978] != 0.0) && (!(s.v[979] != 0.0))) {
            s.copy_ad(288, 278);
        }

        if (s.v[978] != 0.0) {
            s.store_offset_ad(416, A::neg(s.ad_value(288)), (-1e-12));
        }

        if (s.v[978] != 0.0) {
            s.store_scale(144, 437, s.v[436]);
        }

        if (s.v[978] != 0.0) {
            s.store_square(145, 144);
        }

        if (s.v[978] != 0.0) {
            s.store_sub_from_scalar(404, p.p39, 414);
        }

        if (s.v[978] != 0.0) {
            s.store_mul_ad(417, A::div_from_scalar(2.0, s.ad_value(120)), A::ln(A::div_from_scalar(s.v[624], s.ad_value(127))));
        }

        if (s.v[978] != 0.0) {
            s.store_neg(419, 416);
        }

        s.v[980] = if (s.v[404] < s.v[419]) { 1.0 } else { 0.0 };

        if ((s.v[978] != 0.0) && (s.v[980] != 0.0)) {
            s.store_div_from_scalar_ad(291, s.v[435], A::mul(s.ad_value(120), s.ad_value(437)));
        }

        if ((s.v[978] != 0.0) && (s.v[980] != 0.0)) {
            s.store_offset_scaled(184, 291, (3.0 * 1.414213562373095), 2.0);
        }

        if ((s.v[978] != 0.0) && (s.v[980] != 0.0)) {
            s.store_mul_ad_lhs(182, A::mul(A::scale(s.ad_value(184), 8.0), s.ad_value(184)), 184);
        }

        if ((s.v[978] != 0.0) && (s.v[980] != 0.0)) {
            s.store_sub(176, 137, 417);
        }

        if ((s.v[978] != 0.0) && (s.v[980] != 0.0)) {
            s.store_mul_ad_rhs(290, 120, A::add(s.ad_value(404), s.ad_value(416)));
        }

        if ((s.v[978] != 0.0) && (s.v[980] != 0.0)) {
            s.store_sub_from_scalar_ad(183, (7.0 * 1.414213562373095), A::mul(A::scale(s.ad_value(291), 9.0), A::offset(s.ad_value(290), (-2.0))));
        }

        if ((s.v[978] != 0.0) && (s.v[980] != 0.0)) {
            s.store_square(181, 183);
        }

        s.v[981] = if (s.v[182] < (s.v[181] * 1e-8)) { 1.0 } else { 0.0 };

        if (((s.v[978] != 0.0) && (s.v[980] != 0.0)) && (s.v[981] != 0.0)) {
            s.store_add_ad(179, A::add(A::offset(s.ad_value(183), ((-7.0) * 1.414213562373095)), A::div(A::scale(s.ad_value(182), 0.5), s.ad_value(183))), A::mul(A::scale(s.ad_value(291), 9.0), A::offset(s.ad_value(290), (-2.0))));
        }

        if (((s.v[978] != 0.0) && (s.v[980] != 0.0)) && (!(s.v[981] != 0.0))) {
            s.store_sqrt_ad(180, A::add(s.ad_value(182), s.ad_value(181)));
        }

        if (((s.v[978] != 0.0) && (s.v[980] != 0.0)) && (!(s.v[981] != 0.0))) {
            s.store_add_ad(179, A::offset(s.ad_value(180), ((-7.0) * 1.414213562373095)), A::mul(A::scale(s.ad_value(291), 9.0), A::offset(s.ad_value(290), (-2.0))));
        }

        if ((s.v[978] != 0.0) && (s.v[980] != 0.0)) {
            s.store_powf(178, 179, 0.3333333333333333);
        }

        if ((s.v[978] != 0.0) && (s.v[980] != 0.0)) {
            s.store_add_ad(177, A::add(A::sub_from_scalar(((-4.0) * 1.414213562373095), A::scale(s.ad_value(291), 12.0)), A::scale(s.ad_value(178), 2.0)), A::mul(A::scale(s.ad_value(178), 1.414213562373095), s.ad_value(178)));
        }

        if ((s.v[978] != 0.0) && (s.v[980] != 0.0)) {
            s.store_div(77, 177, 178);
        }

        if ((s.v[978] != 0.0) && (s.v[980] != 0.0)) {
            s.store_sub_ad_lhs(259, A::mul(s.ad_value(77), s.ad_value(122)), 416);
        }

        if ((s.v[978] != 0.0) && (s.v[980] != 0.0)) {
            s.store_add(279, 259, 416);
        }

        if ((s.v[978] != 0.0) && (s.v[980] != 0.0)) {
            s.store_div(280, 279, 176);
        }

        if ((s.v[978] != 0.0) && (s.v[980] != 0.0)) {
            s.store_sub_ad_lhs(410, A::div(s.ad_value(279), A::sqrt(A::offset(A::square(s.ad_value(280)), 1.0))), 416);
        }

        if ((s.v[978] != 0.0) && (s.v[980] != 0.0)) {
            s.store_scaled_sub(408, 404, 410, s.v[435]);
        }

        if ((s.v[978] != 0.0) && (s.v[980] != 0.0)) {
            s.copy_ad(407, 408);
        }

        if ((s.v[978] != 0.0) && (!(s.v[980] != 0.0))) {
            s.store_scalar(77, 3.0);
        }

        if ((s.v[978] != 0.0) && (!(s.v[980] != 0.0))) {
            s.store_sub_ad_lhs(319, A::div(s.ad_value(77), s.ad_value(120)), 416);
        }

        if ((s.v[978] != 0.0) && (!(s.v[980] != 0.0))) {
            s.store_offset_ad(290, A::div(A::scale(A::add(A::offset(A::mul(s.ad_value(120), A::add(s.ad_value(404), s.ad_value(416))), (-1.0)), A::exp(A::neg(s.ad_value(77)))), 4.0), A::mul(s.ad_value(145), s.ad_value(121))), 1.0);
        }

        s.v[982] = if (s.v[290] < (10.0 * 2.220446049250313e-16)) { 1.0 } else { 0.0 };

        if (((s.v[978] != 0.0) && (!(s.v[980] != 0.0))) && (s.v[982] != 0.0)) {
            s.store_scalar(290, (10.0 * 2.220446049250313e-16));
        }

        if ((s.v[978] != 0.0) && (!(s.v[980] != 0.0))) {
            s.store_add_ad_rhs(319, 404, A::mul(A::scale(A::mul(s.ad_value(145), s.ad_value(120)), 0.5), A::sub_from_scalar(1.0, A::sqrt(s.ad_value(290)))));
        }

        if ((s.v[978] != 0.0) && (!(s.v[980] != 0.0))) {
            s.store_mul_ad_rhs(77, 120, A::add(s.ad_value(319), s.ad_value(416)));
        }

        if ((s.v[978] != 0.0) && (!(s.v[980] != 0.0))) {
            s.store_offset_ad(290, A::div(A::scale(A::add(A::offset(A::mul(s.ad_value(120), A::add(s.ad_value(404), s.ad_value(416))), (-1.0)), A::exp(A::neg(s.ad_value(77)))), 4.0), A::mul(s.ad_value(145), s.ad_value(121))), 1.0);
        }

        s.v[983] = if (s.v[290] < (10.0 * 2.220446049250313e-16)) { 1.0 } else { 0.0 };

        if (((s.v[978] != 0.0) && (!(s.v[980] != 0.0))) && (s.v[983] != 0.0)) {
            s.store_scalar(290, (10.0 * 2.220446049250313e-16));
        }

        if ((s.v[978] != 0.0) && (!(s.v[980] != 0.0))) {
            s.store_add_ad_rhs(319, 404, A::mul(A::scale(A::mul(s.ad_value(145), s.ad_value(120)), 0.5), A::sub_from_scalar(1.0, A::sqrt(s.ad_value(290)))));
        }

        if ((s.v[978] != 0.0) && (!(s.v[980] != 0.0))) {
            s.store_mul_ad_rhs(77, 120, A::add(s.ad_value(319), s.ad_value(416)));
        }

        s.v[984] = if (s.v[77] < 3.0) { 1.0 } else { 0.0 };

        if (((s.v[978] != 0.0) && (!(s.v[980] != 0.0))) && (s.v[984] != 0.0)) {
            s.store_scalar(421, ((1.0 / (9.0 * 1.414213562373095)) - ((5.0 + (7.0 * 0.049787068367863944)) / (54.0 * (((2.0 + 0.049787068367863944)) as f64).sqrt()))));
        }

        if (((s.v[978] != 0.0) && (!(s.v[980] != 0.0))) && (s.v[984] != 0.0)) {
            s.store_scalar(422, (((1.0 + 0.049787068367863944) / (2.0 * (((2.0 + 0.049787068367863944)) as f64).sqrt())) - (1.414213562373095 / 3.0)));
        }

        if (((s.v[978] != 0.0) && (!(s.v[980] != 0.0))) && (s.v[984] != 0.0)) {
            s.store_offset_ad(423, A::div_from_scalar(1.0, A::mul(s.ad_value(120), s.ad_value(144))), (1.0 / 1.414213562373095));
        }

        if (((s.v[978] != 0.0) && (!(s.v[980] != 0.0))) && (s.v[984] != 0.0)) {
            s.store_div_ad_lhs(425, A::neg(A::add(s.ad_value(404), s.ad_value(416))), 144);
        }

        if (((s.v[978] != 0.0) && (!(s.v[980] != 0.0))) && (s.v[984] != 0.0)) {
            s.store_add_ad(426, A::sub(A::div(A::mul(A::square(s.ad_value(422)), s.ad_value(422)), A::mul(A::mul(A::scale(s.ad_value(421), 27.0), s.ad_value(421)), s.ad_value(421))), A::div(A::mul(s.ad_value(422), s.ad_value(423)), A::mul(A::scale(s.ad_value(421), 6.0), s.ad_value(421)))), A::div(s.ad_value(425), A::scale(s.ad_value(421), 2.0)));
        }

        if (((s.v[978] != 0.0) && (!(s.v[980] != 0.0))) && (s.v[984] != 0.0)) {
            s.store_div_ad(424, A::sub(A::mul(A::scale(s.ad_value(421), 3.0), s.ad_value(423)), A::square(s.ad_value(422))), A::mul(A::scale(s.ad_value(421), 9.0), s.ad_value(421)));
        }

        if (((s.v[978] != 0.0) && (!(s.v[980] != 0.0))) && (s.v[984] != 0.0)) {
            s.store_sqrt_ad(283, A::add(A::square(s.ad_value(426)), A::mul(A::square(s.ad_value(424)), s.ad_value(424))));
        }

        if (((s.v[978] != 0.0) && (!(s.v[980] != 0.0))) && (s.v[984] != 0.0)) {
            s.store_powf_ad(427, A::sub(s.ad_value(283), s.ad_value(426)), 0.3333333333333333);
        }

        if (((s.v[978] != 0.0) && (!(s.v[980] != 0.0))) && (s.v[984] != 0.0)) {
            s.store_neg_ad(428, A::powf(A::add(s.ad_value(426), s.ad_value(283)), 0.3333333333333333));
        }

        if (((s.v[978] != 0.0) && (!(s.v[980] != 0.0))) && (s.v[984] != 0.0)) {
            s.store_sub_ad(290, A::add(s.ad_value(427), s.ad_value(428)), A::div(s.ad_value(422), A::scale(s.ad_value(421), 3.0)));
        }

        if (((s.v[978] != 0.0) && (!(s.v[980] != 0.0))) && (s.v[984] != 0.0)) {
            s.store_sub_ad_lhs(319, A::mul(s.ad_value(290), s.ad_value(122)), 416);
        }

        if (((s.v[978] != 0.0) && (!(s.v[980] != 0.0))) && (s.v[984] != 0.0)) {
            s.store_mul_ad_rhs(77, 120, A::add(s.ad_value(319), s.ad_value(416)));
        }

        s.v[985] = if (p.p30 > 0.0) { 1.0 } else { 0.0 };

        if (((s.v[978] != 0.0) && (!(s.v[980] != 0.0))) && (s.v[985] != 0.0)) {
            s.store_offset_ad(420, A::add(s.ad_value(404), s.ad_value(416)), 0.1);
        }

        if (((s.v[978] != 0.0) && (!(s.v[980] != 0.0))) && (s.v[985] != 0.0)) {
            s.store_offset_ad(203, A::exp(A::mul(s.ad_value(120), A::neg(s.ad_value(416)))), 1e-50);
        }

        if (((s.v[978] != 0.0) && (!(s.v[980] != 0.0))) && (s.v[985] != 0.0)) {
            s.store_scale(278, 127, 1.0 / (s.v[624]));
        }

        if (((s.v[978] != 0.0) && (!(s.v[980] != 0.0))) && (s.v[985] != 0.0)) {
            s.store_square(429, 278);
        }

        if (((s.v[978] != 0.0) && (!(s.v[980] != 0.0))) && (s.v[985] != 0.0)) {
            s.store_mul(430, 429, 203);
        }

        if (((s.v[978] != 0.0) && (!(s.v[980] != 0.0))) && (s.v[985] != 0.0)) {
            s.store_mul(278, 121, 145);
        }

        if (((s.v[978] != 0.0) && (!(s.v[980] != 0.0))) && (s.v[985] != 0.0)) {
            s.store_mul(434, 120, 420);
        }

        if (((s.v[978] != 0.0) && (!(s.v[980] != 0.0))) && (s.v[985] != 0.0)) {
            s.store_add_ad(433, A::sub(A::ln(A::add(A::mul(s.ad_value(430), s.ad_value(278)), A::square(s.ad_value(434)))), A::ln(A::mul(s.ad_value(429), s.ad_value(278)))), A::mul(s.ad_value(120), s.ad_value(416)));
        }

        if (((s.v[978] != 0.0) && (!(s.v[980] != 0.0))) && (s.v[985] != 0.0)) {
            s.store_offset_ad(638, A::sub(s.ad_value(434), s.ad_value(433)), (-1.0));
        }

        if (((s.v[978] != 0.0) && (!(s.v[980] != 0.0))) && (s.v[985] != 0.0)) {
            s.store_scale(639, 434, 4.0);
        }

        if (((s.v[978] != 0.0) && (!(s.v[980] != 0.0))) && (s.v[985] != 0.0)) {
            s.store_ad(639, &{
                if (s.v[639] > 0.0) {
                    s.ad_value(639)
                } else {
                    A::neg(s.ad_value(639))
                }
            });
        }

        if (((s.v[978] != 0.0) && (!(s.v[980] != 0.0))) && (s.v[985] != 0.0)) {
            s.store_sqrt_ad(639, A::add(A::square(s.ad_value(638)), s.ad_value(639)));
        }

        if (((s.v[978] != 0.0) && (!(s.v[980] != 0.0))) && (s.v[985] != 0.0)) {
            s.store_scale_ad(279, A::offset(A::div(s.ad_value(638), s.ad_value(639)), 1.0), 0.5);
        }

        if (((s.v[978] != 0.0) && (!(s.v[980] != 0.0))) && (s.v[985] != 0.0)) {
            s.store_scale_ad(280, A::sub_from_scalar(1.0, A::div(A::offset(s.ad_value(638), 2.0), s.ad_value(639))), 0.5);
        }

        if (((s.v[978] != 0.0) && (!(s.v[980] != 0.0))) && (s.v[985] != 0.0)) {
            s.store_sub_ad_rhs(433, 434, A::scale(A::add(s.ad_value(638), s.ad_value(639)), 0.5));
        }

        if (((s.v[978] != 0.0) && (!(s.v[980] != 0.0))) && (s.v[985] != 0.0)) {
            s.store_sub(434, 434, 433);
        }

        if (((s.v[978] != 0.0) && (!(s.v[980] != 0.0))) && (s.v[985] != 0.0)) {
            s.store_add_ad_rhs(434, 434, A::scale(s.ad_value(120), 0.1));
        }

        if (((s.v[978] != 0.0) && (!(s.v[980] != 0.0))) && (s.v[985] != 0.0)) {
            s.store_add_ad(432, A::sub(A::ln(A::add(A::mul(s.ad_value(430), s.ad_value(278)), A::square(s.ad_value(434)))), A::ln(A::mul(s.ad_value(429), s.ad_value(278)))), A::mul(s.ad_value(120), s.ad_value(416)));
        }

        if (((s.v[978] != 0.0) && (!(s.v[980] != 0.0))) && (s.v[985] != 0.0)) {
            s.store_sub_ad_lhs(320, A::div(s.ad_value(432), s.ad_value(120)), 416);
        }

        if (((s.v[978] != 0.0) && (!(s.v[980] != 0.0))) && (s.v[985] != 0.0)) {
            s.copy_ad(431, 77);
        }

        if (((s.v[978] != 0.0) && (!(s.v[980] != 0.0))) && (s.v[985] != 0.0)) {
            s.store_offset_ad(638, A::sub(s.ad_value(432), s.ad_value(431)), (-(0.0008 * 75.0)));
        }

        if (((s.v[978] != 0.0) && (!(s.v[980] != 0.0))) && (s.v[985] != 0.0)) {
            s.store_scale(639, 432, (4.0 * (0.0008 * 75.0)));
        }

        if (((s.v[978] != 0.0) && (!(s.v[980] != 0.0))) && (s.v[985] != 0.0)) {
            s.store_ad(639, &{
                if (s.v[639] > 0.0) {
                    s.ad_value(639)
                } else {
                    A::neg(s.ad_value(639))
                }
            });
        }

        if (((s.v[978] != 0.0) && (!(s.v[980] != 0.0))) && (s.v[985] != 0.0)) {
            s.store_sqrt_ad(639, A::add(A::square(s.ad_value(638)), s.ad_value(639)));
        }

        if (((s.v[978] != 0.0) && (!(s.v[980] != 0.0))) && (s.v[985] != 0.0)) {
            s.store_scale_ad(279, A::offset(A::div(s.ad_value(638), s.ad_value(639)), 1.0), 0.5);
        }

        if (((s.v[978] != 0.0) && (!(s.v[980] != 0.0))) && (s.v[985] != 0.0)) {
            s.store_scale_ad(280, A::sub_from_scalar(1.0, A::div(A::offset(s.ad_value(638), ((2.0 * 0.0008) * 75.0)), s.ad_value(639))), 0.5);
        }

        if (((s.v[978] != 0.0) && (!(s.v[980] != 0.0))) && (s.v[985] != 0.0)) {
            s.store_sub_ad_rhs(77, 432, A::scale(A::add(s.ad_value(638), s.ad_value(639)), 0.5));
        }

        if ((s.v[978] != 0.0) && (!(s.v[980] != 0.0))) {
            s.store_sub_ad_lhs(410, A::div(s.ad_value(77), s.ad_value(120)), 416);
        }

        if ((s.v[978] != 0.0) && (!(s.v[980] != 0.0))) {
            s.store_add_ad(279, A::offset(s.ad_value(77), (-1.0)), A::exp(A::neg(s.ad_value(77))));
        }

        s.v[986] = if (s.v[279] < (10.0 * 2.220446049250313e-16)) { 1.0 } else { 0.0 };

        if (((s.v[978] != 0.0) && (!(s.v[980] != 0.0))) && (s.v[986] != 0.0)) {
            s.store_scalar(279, (10.0 * 2.220446049250313e-16));
        }

        if ((s.v[978] != 0.0) && (!(s.v[980] != 0.0))) {
            s.store_mul_ad_rhs(407, 437, A::sqrt(s.ad_value(279)));
        }

        if ((s.v[978] != 0.0) && (!(s.v[980] != 0.0))) {
            s.store_scaled_sub(408, 404, 410, s.v[435]);
        }

        s.v[987] = if (p.p30 == 1.0) { 1.0 } else { 0.0 };

        if (((s.v[978] != 0.0) && (!(s.v[980] != 0.0))) && (s.v[987] != 0.0)) {
            s.store_exp_ad(203, A::mul(s.ad_value(120), A::neg(s.ad_value(416))));
        }

        if (((s.v[978] != 0.0) && (!(s.v[980] != 0.0))) && (s.v[987] != 0.0)) {
            s.store_scale(278, 127, 1.0 / (s.v[624]));
        }

        if (((s.v[978] != 0.0) && (!(s.v[980] != 0.0))) && (s.v[987] != 0.0)) {
            s.store_square(429, 278);
        }

        if (((s.v[978] != 0.0) && (!(s.v[980] != 0.0))) && (s.v[987] != 0.0)) {
            s.store_mul(204, 429, 203);
        }

        if (((s.v[978] != 0.0) && (!(s.v[980] != 0.0))) && (s.v[987] != 0.0)) {
            s.store_scalar(379, 0.0);
        }

        if (((s.v[978] != 0.0) && (!(s.v[980] != 0.0))) && (s.v[987] != 0.0)) {
            s.store_scalar(62, 1.0);
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
        let mut assign19230_loop_guard: usize = 0;
        while {
            let assign19230_cond_e23449: f64 = (40.0 + 1.0);
            let assign19230_cond_e23451: f64 = if ((((s.v[978] != 0.0) && (!(s.v[980] != 0.0))) && (s.v[987] != 0.0)) && (s.v[62] <= assign19230_cond_e23449)) { 1.0 } else { 0.0 };
            assign19230_cond_e23451 != 0.0
        } {
            assign19230_loop_guard += 1;
            assert!(assign19230_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.v[978] != 0.0) && (!(s.v[980] != 0.0))) && (s.v[987] != 0.0)) {
                s.store_mul_ad_rhs(77, 120, A::add(s.ad_value(410), s.ad_value(416)));
            }
            s.v[988] = if (s.v[77] < 5.0) { 1.0 } else { 0.0 };
            if ((((s.v[978] != 0.0) && (!(s.v[980] != 0.0))) && (s.v[987] != 0.0)) && (s.v[988] != 0.0)) {
                s.store_mul_ad(205, A::mul(A::square(s.ad_value(77)), s.ad_value(77)), A::offset(A::mul(s.ad_value(77), A::offset(A::scale(s.ad_value(77), 0.006115288895133179), (-0.07053654284009761))), 0.29693154855771));
            }
            if ((((s.v[978] != 0.0) && (!(s.v[980] != 0.0))) && (s.v[987] != 0.0)) && (s.v[988] != 0.0)) {
                s.store_mul_ad(206, A::square(s.ad_value(77)), A::offset(A::mul(s.ad_value(77), A::offset(A::scale(s.ad_value(77), (5.0 * 0.006115288895133179)), (4.0 * (-0.07053654284009761)))), (3.0 * 0.29693154855771)));
            }
            if ((((s.v[978] != 0.0) && (!(s.v[980] != 0.0))) && (s.v[987] != 0.0)) && (s.v[988] != 0.0)) {
                s.store_mul_ad_lhs(207, A::mul(s.ad_value(204), s.ad_value(205)), 205);
            }
            if ((((s.v[978] != 0.0) && (!(s.v[980] != 0.0))) && (s.v[987] != 0.0)) && (s.v[988] != 0.0)) {
                s.store_mul_ad_lhs(208, A::mul(A::scale(A::mul(s.ad_value(204), s.ad_value(120)), 2.0), s.ad_value(205)), 206);
            }
            if ((((s.v[978] != 0.0) && (!(s.v[980] != 0.0))) && (s.v[987] != 0.0)) && (s.v[988] != 0.0)) {
                s.store_mul_ad_rhs(146, 77, A::offset(A::mul(s.ad_value(77), A::offset(A::mul(s.ad_value(77), A::offset(A::mul(s.ad_value(77), A::offset(A::scale(s.ad_value(77), 6.36964918866352e-5), (-0.00163730162779191))), 0.0178800506338833)), (-0.117851130197758))), 0.707106781186548));
            }
            if ((((s.v[978] != 0.0) && (!(s.v[980] != 0.0))) && (s.v[987] != 0.0)) && (s.v[988] != 0.0)) {
                s.store_offset_ad(148, A::mul(s.ad_value(77), A::offset(A::mul(s.ad_value(77), A::offset(A::mul(s.ad_value(77), A::offset(A::scale(s.ad_value(77), (5.0 * 6.36964918866352e-5)), (4.0 * (-0.00163730162779191)))), (3.0 * 0.0178800506338833))), (2.0 * (-0.117851130197758)))), 0.707106781186548);
            }
            if ((((s.v[978] != 0.0) && (!(s.v[980] != 0.0))) && (s.v[987] != 0.0)) && (s.v[988] != 0.0)) {
                s.store_sqrt_ad(209, A::offset(A::add(A::square(s.ad_value(146)), s.ad_value(207)), 1e-50));
            }
            if ((((s.v[978] != 0.0) && (!(s.v[980] != 0.0))) && (s.v[987] != 0.0)) && (s.v[988] != 0.0)) {
                s.store_div_ad(210, A::add(A::mul(A::scale(A::mul(s.ad_value(120), s.ad_value(148)), 2.0), s.ad_value(146)), s.ad_value(208)), A::scale(s.ad_value(209), 2.0));
            }
            s.v[989] = if (s.v[77] < 80.0) { 1.0 } else { 0.0 };
            if (((((s.v[978] != 0.0) && (!(s.v[980] != 0.0))) && (s.v[987] != 0.0)) && (!(s.v[988] != 0.0))) && (s.v[989] != 0.0)) {
                s.store_exp(147, 77);
            }
            if (((((s.v[978] != 0.0) && (!(s.v[980] != 0.0))) && (s.v[987] != 0.0)) && (!(s.v[988] != 0.0))) && (s.v[989] != 0.0)) {
                s.store_mul_ad_rhs(207, 204, A::offset(s.ad_value(147), (-1.0)));
            }
            if (((((s.v[978] != 0.0) && (!(s.v[980] != 0.0))) && (s.v[987] != 0.0)) && (!(s.v[988] != 0.0))) && (s.v[989] != 0.0)) {
                s.store_mul_ad_lhs(208, A::mul(s.ad_value(204), s.ad_value(120)), 147);
            }
            if (((((s.v[978] != 0.0) && (!(s.v[980] != 0.0))) && (s.v[987] != 0.0)) && (!(s.v[988] != 0.0))) && (!(s.v[989] != 0.0))) {
                s.store_exp_ad(202, A::mul(s.ad_value(120), s.ad_value(410)));
            }
            if (((((s.v[978] != 0.0) && (!(s.v[980] != 0.0))) && (s.v[987] != 0.0)) && (!(s.v[988] != 0.0))) && (!(s.v[989] != 0.0))) {
                s.store_mul_ad_rhs(207, 429, A::sub(s.ad_value(202), s.ad_value(203)));
            }
            if (((((s.v[978] != 0.0) && (!(s.v[980] != 0.0))) && (s.v[987] != 0.0)) && (!(s.v[988] != 0.0))) && (!(s.v[989] != 0.0))) {
                s.store_mul_ad_lhs(208, A::mul(s.ad_value(429), s.ad_value(120)), 202);
            }
            if ((((s.v[978] != 0.0) && (!(s.v[980] != 0.0))) && (s.v[987] != 0.0)) && (!(s.v[988] != 0.0))) {
                s.store_sqrt_ad(209, A::add(A::offset(s.ad_value(77), (-1.0)), s.ad_value(207)));
            }
            if ((((s.v[978] != 0.0) && (!(s.v[980] != 0.0))) && (s.v[987] != 0.0)) && (!(s.v[988] != 0.0))) {
                s.store_scale_ad(210, A::div(A::add(s.ad_value(120), s.ad_value(208)), s.ad_value(209)), 0.5);
            }
            if (((s.v[978] != 0.0) && (!(s.v[980] != 0.0))) && (s.v[987] != 0.0)) {
                s.store_sub_ad(211, A::sub(s.ad_value(404), s.ad_value(410)), A::mul(s.ad_value(144), s.ad_value(209)));
            }
            if (((s.v[978] != 0.0) && (!(s.v[980] != 0.0))) && (s.v[987] != 0.0)) {
                s.store_sub_from_scalar_ad(212, (-1.0), A::mul(s.ad_value(144), s.ad_value(210)));
            }
            s.v[990] = if (s.v[379] == 1.0) { 1.0 } else { 0.0 };
            if ((((s.v[978] != 0.0) && (!(s.v[980] != 0.0))) && (s.v[987] != 0.0)) && (s.v[990] != 0.0)) {
                s.store_scalar(62, (40.0 + 1.0));
            }
            if ((((s.v[978] != 0.0) && (!(s.v[980] != 0.0))) && (s.v[987] != 0.0)) && (!(s.v[990] != 0.0))) {
                s.store_div_ad_lhs(213, A::neg(s.ad_value(211)), 212);
            }
            if ((((s.v[978] != 0.0) && (!(s.v[980] != 0.0))) && (s.v[987] != 0.0)) && (!(s.v[990] != 0.0))) {
                s.store_scale_ad(214, A::offset({
                    if (1.0 >= ((s.v[410]) as f64).abs()) {
                        A::constant(1.0)
                    } else {
                        A::abs(s.ad_value(410))
                    }
                }, 1.0), (0.5 * 0.1));
            }
            s.v[991] = if (((s.v[213]) as f64).abs() > s.v[214]) { 1.0 } else { 0.0 };
            if (((((s.v[978] != 0.0) && (!(s.v[980] != 0.0))) && (s.v[987] != 0.0)) && (!(s.v[990] != 0.0))) && (s.v[991] != 0.0)) {
                s.store_scale(213, 214, (if (s.v[213] >= 0.0) { 1.0 } else { (-1.0) }));
            }
            if ((((s.v[978] != 0.0) && (!(s.v[980] != 0.0))) && (s.v[987] != 0.0)) && (!(s.v[990] != 0.0))) {
                s.store_add(410, 410, 213);
            }
            s.v[992] = if ((((s.v[213]) as f64).abs() <= 1e-12) && (((s.v[211]) as f64).abs() <= 1e-8)) { 1.0 } else { 0.0 };
            if (((((s.v[978] != 0.0) && (!(s.v[980] != 0.0))) && (s.v[987] != 0.0)) && (!(s.v[990] != 0.0))) && (s.v[992] != 0.0)) {
                s.store_scalar(379, 1.0);
            }
            if (((s.v[978] != 0.0) && (!(s.v[980] != 0.0))) && (s.v[987] != 0.0)) {
                s.store_offset(62, 62, 1.0);
            }
        }

        s.v[994] = if (s.v[77] < 5.0) { 1.0 } else { 0.0 };

        if ((((s.v[978] != 0.0) && (!(s.v[980] != 0.0))) && (s.v[987] != 0.0)) && (s.v[994] != 0.0)) {
            s.store_offset_ad(64, A::square(s.ad_value(146)), (10.0 * 2.220446049250313e-16));
        }

        if ((((s.v[978] != 0.0) && (!(s.v[980] != 0.0))) && (s.v[987] != 0.0)) && (s.v[994] != 0.0)) {
            s.store_offset(65, 146, (10.0 * 2.220446049250313e-16));
        }

        if ((((s.v[978] != 0.0) && (!(s.v[980] != 0.0))) && (s.v[987] != 0.0)) && (!(s.v[994] != 0.0))) {
            s.store_offset(64, 77, (-1.0));
        }

        if ((((s.v[978] != 0.0) && (!(s.v[980] != 0.0))) && (s.v[987] != 0.0)) && (!(s.v[994] != 0.0))) {
            s.store_sqrt(65, 64);
        }

        if (((s.v[978] != 0.0) && (!(s.v[980] != 0.0))) && (s.v[987] != 0.0)) {
            s.store_mul(407, 437, 65);
        }

        if (((s.v[978] != 0.0) && (!(s.v[980] != 0.0))) && (s.v[987] != 0.0)) {
            s.store_div_from_scalar_ad(279, 1.0, A::add(s.ad_value(209), s.ad_value(65)));
        }

        if (((s.v[978] != 0.0) && (!(s.v[980] != 0.0))) && (s.v[987] != 0.0)) {
            s.store_mul_ad_lhs(409, A::mul(s.ad_value(437), s.ad_value(207)), 279);
        }

        if (((s.v[978] != 0.0) && (!(s.v[980] != 0.0))) && (s.v[987] != 0.0)) {
            s.store_add(408, 407, 409);
        }

        if (s.v[978] != 0.0) {
            s.store_sub(409, 408, 407);
        }

        if (s.v[978] != 0.0) {
            s.store_scale(282, 195, s.v[513]);
        }

        if ((s.v[978] != 0.0) && (s.v[402] != 0.0)) {
            s.store_mul(398, 282, 408);
        }

        if ((s.v[978] != 0.0) && (s.v[402] != 0.0)) {
            s.store_mul(406, 282, 407);
        }

        if ((s.v[978] != 0.0) && (s.v[403] != 0.0)) {
            s.store_mul(397, 282, 408);
        }

        if ((s.v[978] != 0.0) && (s.v[403] != 0.0)) {
            s.store_mul(405, 282, 407);
        }

        if (s.v[978] != 0.0) {
            s.store_scalar(399, ((1.0 - 1.0) / 2.0));
        }

        if (s.v[978] != 0.0) {
            s.store_scalar(400, ((1.0 + 1.0) / 2.0));
        }

        if (s.v[978] != 0.0) {
            s.store_add_ad(402, A::mul(s.ad_value(399), s.ad_value(412)), A::mul(s.ad_value(400), s.ad_value(413)));
        }

        if (s.v[978] != 0.0) {
            s.store_add_ad(403, A::mul(s.ad_value(399), s.ad_value(413)), A::mul(s.ad_value(400), s.ad_value(412)));
        }

        if ((s.v[978] != 0.0) && (s.v[399] != 0.0)) {
            s.store_add_ad(414, A::mul(s.ad_value(412), s.ad_value(42)), A::mul(s.ad_value(413), A::sub(s.ad_value(42), s.ad_value(41))));
        }

        if ((s.v[978] != 0.0) && (s.v[400] != 0.0)) {
            s.store_add_ad(414, A::mul(s.ad_value(413), s.ad_value(42)), A::mul(s.ad_value(412), A::sub(s.ad_value(42), s.ad_value(41))));
        }

        if (s.v[978] != 0.0) {
            s.store_scalar(415, 0.0);
        }

        if (s.v[978] != 0.0) {
            s.store_neg(278, 415);
        }

        s.v[996] = if (s.v[278] > s.v[31]) { 1.0 } else { 0.0 };

        if ((s.v[978] != 0.0) && (s.v[996] != 0.0)) {
            s.store_sub(279, 278, 31);
        }

        if ((s.v[978] != 0.0) && (s.v[996] != 0.0)) {
            s.store_sub_from_scalar(280, s.v[30], 31);
        }

        if ((s.v[978] != 0.0) && (s.v[996] != 0.0)) {
            s.store_div(638, 279, 280);
        }

        if ((s.v[978] != 0.0) && (s.v[996] != 0.0)) {
            s.store_square(639, 638);
        }

        if ((s.v[978] != 0.0) && (s.v[996] != 0.0)) {
            s.store_mul(640, 639, 638);
        }

        if ((s.v[978] != 0.0) && (s.v[996] != 0.0)) {
            s.store_square(641, 639);
        }

        if ((s.v[978] != 0.0) && (s.v[996] != 0.0)) {
            s.store_div_from_scalar_ad(291, 1.0, A::add(A::add(A::add(A::offset(s.ad_value(638), 1.0), s.ad_value(639)), s.ad_value(640)), s.ad_value(641)));
        }

        if ((s.v[978] != 0.0) && (s.v[996] != 0.0)) {
            s.store_mul_ad_lhs(387, A::mul(A::neg(A::add(A::add(A::offset(A::scale(s.ad_value(638), 2.0), 1.0), A::scale(s.ad_value(639), 3.0)), A::scale(s.ad_value(640), 4.0))), s.ad_value(291)), 291);
        }

        if ((s.v[978] != 0.0) && (s.v[996] != 0.0)) {
            s.store_mul_ad_rhs(291, 280, A::sub_from_scalar(1.0, s.ad_value(291)));
        }

        if ((s.v[978] != 0.0) && (s.v[996] != 0.0)) {
            s.store_neg(387, 387);
        }

        if ((s.v[978] != 0.0) && (s.v[996] != 0.0)) {
            s.store_add(288, 31, 291);
        }

        if ((s.v[978] != 0.0) && (!(s.v[996] != 0.0))) {
            s.copy_ad(288, 278);
        }

        if (s.v[978] != 0.0) {
            s.store_offset_ad(416, A::neg(s.ad_value(288)), (-1e-12));
        }

        if (s.v[978] != 0.0) {
            s.store_scale(144, 437, s.v[436]);
        }

        if (s.v[978] != 0.0) {
            s.store_square(145, 144);
        }

        if (s.v[978] != 0.0) {
            s.store_sub_from_scalar(404, p.p39, 414);
        }

        if (s.v[978] != 0.0) {
            s.store_mul_ad(417, A::div_from_scalar(2.0, s.ad_value(120)), A::ln(A::div_from_scalar(s.v[624], s.ad_value(127))));
        }

        if (s.v[978] != 0.0) {
            s.store_neg(419, 416);
        }

        s.v[997] = if (s.v[404] < s.v[419]) { 1.0 } else { 0.0 };

        if ((s.v[978] != 0.0) && (s.v[997] != 0.0)) {
            s.store_div_from_scalar_ad(291, s.v[435], A::mul(s.ad_value(120), s.ad_value(437)));
        }

        if ((s.v[978] != 0.0) && (s.v[997] != 0.0)) {
            s.store_offset_scaled(184, 291, (3.0 * 1.414213562373095), 2.0);
        }

        if ((s.v[978] != 0.0) && (s.v[997] != 0.0)) {
            s.store_mul_ad_lhs(182, A::mul(A::scale(s.ad_value(184), 8.0), s.ad_value(184)), 184);
        }

        if ((s.v[978] != 0.0) && (s.v[997] != 0.0)) {
            s.store_sub(176, 137, 417);
        }

        if ((s.v[978] != 0.0) && (s.v[997] != 0.0)) {
            s.store_mul_ad_rhs(290, 120, A::add(s.ad_value(404), s.ad_value(416)));
        }

        if ((s.v[978] != 0.0) && (s.v[997] != 0.0)) {
            s.store_sub_from_scalar_ad(183, (7.0 * 1.414213562373095), A::mul(A::scale(s.ad_value(291), 9.0), A::offset(s.ad_value(290), (-2.0))));
        }

        if ((s.v[978] != 0.0) && (s.v[997] != 0.0)) {
            s.store_square(181, 183);
        }

        s.v[998] = if (s.v[182] < (s.v[181] * 1e-8)) { 1.0 } else { 0.0 };

        if (((s.v[978] != 0.0) && (s.v[997] != 0.0)) && (s.v[998] != 0.0)) {
            s.store_add_ad(179, A::add(A::offset(s.ad_value(183), ((-7.0) * 1.414213562373095)), A::div(A::scale(s.ad_value(182), 0.5), s.ad_value(183))), A::mul(A::scale(s.ad_value(291), 9.0), A::offset(s.ad_value(290), (-2.0))));
        }

        if (((s.v[978] != 0.0) && (s.v[997] != 0.0)) && (!(s.v[998] != 0.0))) {
            s.store_sqrt_ad(180, A::add(s.ad_value(182), s.ad_value(181)));
        }

        if (((s.v[978] != 0.0) && (s.v[997] != 0.0)) && (!(s.v[998] != 0.0))) {
            s.store_add_ad(179, A::offset(s.ad_value(180), ((-7.0) * 1.414213562373095)), A::mul(A::scale(s.ad_value(291), 9.0), A::offset(s.ad_value(290), (-2.0))));
        }

        if ((s.v[978] != 0.0) && (s.v[997] != 0.0)) {
            s.store_powf(178, 179, 0.3333333333333333);
        }

        if ((s.v[978] != 0.0) && (s.v[997] != 0.0)) {
            s.store_add_ad(177, A::add(A::sub_from_scalar(((-4.0) * 1.414213562373095), A::scale(s.ad_value(291), 12.0)), A::scale(s.ad_value(178), 2.0)), A::mul(A::scale(s.ad_value(178), 1.414213562373095), s.ad_value(178)));
        }

        if ((s.v[978] != 0.0) && (s.v[997] != 0.0)) {
            s.store_div(77, 177, 178);
        }

        if ((s.v[978] != 0.0) && (s.v[997] != 0.0)) {
            s.store_sub_ad_lhs(259, A::mul(s.ad_value(77), s.ad_value(122)), 416);
        }

        if ((s.v[978] != 0.0) && (s.v[997] != 0.0)) {
            s.store_add(279, 259, 416);
        }

        if ((s.v[978] != 0.0) && (s.v[997] != 0.0)) {
            s.store_div(280, 279, 176);
        }

        if ((s.v[978] != 0.0) && (s.v[997] != 0.0)) {
            s.store_sub_ad_lhs(410, A::div(s.ad_value(279), A::sqrt(A::offset(A::square(s.ad_value(280)), 1.0))), 416);
        }

        if ((s.v[978] != 0.0) && (s.v[997] != 0.0)) {
            s.store_scaled_sub(408, 404, 410, s.v[435]);
        }

        if ((s.v[978] != 0.0) && (s.v[997] != 0.0)) {
            s.copy_ad(407, 408);
        }

        if ((s.v[978] != 0.0) && (!(s.v[997] != 0.0))) {
            s.store_scalar(77, 3.0);
        }

        if ((s.v[978] != 0.0) && (!(s.v[997] != 0.0))) {
            s.store_sub_ad_lhs(319, A::div(s.ad_value(77), s.ad_value(120)), 416);
        }

        if ((s.v[978] != 0.0) && (!(s.v[997] != 0.0))) {
            s.store_offset_ad(290, A::div(A::scale(A::add(A::offset(A::mul(s.ad_value(120), A::add(s.ad_value(404), s.ad_value(416))), (-1.0)), A::exp(A::neg(s.ad_value(77)))), 4.0), A::mul(s.ad_value(145), s.ad_value(121))), 1.0);
        }

        s.v[999] = if (s.v[290] < (10.0 * 2.220446049250313e-16)) { 1.0 } else { 0.0 };

        if (((s.v[978] != 0.0) && (!(s.v[997] != 0.0))) && (s.v[999] != 0.0)) {
            s.store_scalar(290, (10.0 * 2.220446049250313e-16));
        }

        if ((s.v[978] != 0.0) && (!(s.v[997] != 0.0))) {
            s.store_add_ad_rhs(319, 404, A::mul(A::scale(A::mul(s.ad_value(145), s.ad_value(120)), 0.5), A::sub_from_scalar(1.0, A::sqrt(s.ad_value(290)))));
        }

        if ((s.v[978] != 0.0) && (!(s.v[997] != 0.0))) {
            s.store_mul_ad_rhs(77, 120, A::add(s.ad_value(319), s.ad_value(416)));
        }

        if ((s.v[978] != 0.0) && (!(s.v[997] != 0.0))) {
            s.store_offset_ad(290, A::div(A::scale(A::add(A::offset(A::mul(s.ad_value(120), A::add(s.ad_value(404), s.ad_value(416))), (-1.0)), A::exp(A::neg(s.ad_value(77)))), 4.0), A::mul(s.ad_value(145), s.ad_value(121))), 1.0);
        }

        s.v[1000] = if (s.v[290] < (10.0 * 2.220446049250313e-16)) { 1.0 } else { 0.0 };

        if (((s.v[978] != 0.0) && (!(s.v[997] != 0.0))) && (s.v[1000] != 0.0)) {
            s.store_scalar(290, (10.0 * 2.220446049250313e-16));
        }

        if ((s.v[978] != 0.0) && (!(s.v[997] != 0.0))) {
            s.store_add_ad_rhs(319, 404, A::mul(A::scale(A::mul(s.ad_value(145), s.ad_value(120)), 0.5), A::sub_from_scalar(1.0, A::sqrt(s.ad_value(290)))));
        }

        if ((s.v[978] != 0.0) && (!(s.v[997] != 0.0))) {
            s.store_mul_ad_rhs(77, 120, A::add(s.ad_value(319), s.ad_value(416)));
        }

        s.v[1001] = if (s.v[77] < 3.0) { 1.0 } else { 0.0 };

        if (((s.v[978] != 0.0) && (!(s.v[997] != 0.0))) && (s.v[1001] != 0.0)) {
            s.store_scalar(421, ((1.0 / (9.0 * 1.414213562373095)) - ((5.0 + (7.0 * 0.049787068367863944)) / (54.0 * (((2.0 + 0.049787068367863944)) as f64).sqrt()))));
        }

        if (((s.v[978] != 0.0) && (!(s.v[997] != 0.0))) && (s.v[1001] != 0.0)) {
            s.store_scalar(422, (((1.0 + 0.049787068367863944) / (2.0 * (((2.0 + 0.049787068367863944)) as f64).sqrt())) - (1.414213562373095 / 3.0)));
        }

        if (((s.v[978] != 0.0) && (!(s.v[997] != 0.0))) && (s.v[1001] != 0.0)) {
            s.store_offset_ad(423, A::div_from_scalar(1.0, A::mul(s.ad_value(120), s.ad_value(144))), (1.0 / 1.414213562373095));
        }

        if (((s.v[978] != 0.0) && (!(s.v[997] != 0.0))) && (s.v[1001] != 0.0)) {
            s.store_div_ad_lhs(425, A::neg(A::add(s.ad_value(404), s.ad_value(416))), 144);
        }

        if (((s.v[978] != 0.0) && (!(s.v[997] != 0.0))) && (s.v[1001] != 0.0)) {
            s.store_add_ad(426, A::sub(A::div(A::mul(A::square(s.ad_value(422)), s.ad_value(422)), A::mul(A::mul(A::scale(s.ad_value(421), 27.0), s.ad_value(421)), s.ad_value(421))), A::div(A::mul(s.ad_value(422), s.ad_value(423)), A::mul(A::scale(s.ad_value(421), 6.0), s.ad_value(421)))), A::div(s.ad_value(425), A::scale(s.ad_value(421), 2.0)));
        }

        if (((s.v[978] != 0.0) && (!(s.v[997] != 0.0))) && (s.v[1001] != 0.0)) {
            s.store_div_ad(424, A::sub(A::mul(A::scale(s.ad_value(421), 3.0), s.ad_value(423)), A::square(s.ad_value(422))), A::mul(A::scale(s.ad_value(421), 9.0), s.ad_value(421)));
        }

        if (((s.v[978] != 0.0) && (!(s.v[997] != 0.0))) && (s.v[1001] != 0.0)) {
            s.store_sqrt_ad(283, A::add(A::square(s.ad_value(426)), A::mul(A::square(s.ad_value(424)), s.ad_value(424))));
        }

        if (((s.v[978] != 0.0) && (!(s.v[997] != 0.0))) && (s.v[1001] != 0.0)) {
            s.store_powf_ad(427, A::sub(s.ad_value(283), s.ad_value(426)), 0.3333333333333333);
        }

        if (((s.v[978] != 0.0) && (!(s.v[997] != 0.0))) && (s.v[1001] != 0.0)) {
            s.store_neg_ad(428, A::powf(A::add(s.ad_value(426), s.ad_value(283)), 0.3333333333333333));
        }

        if (((s.v[978] != 0.0) && (!(s.v[997] != 0.0))) && (s.v[1001] != 0.0)) {
            s.store_sub_ad(290, A::add(s.ad_value(427), s.ad_value(428)), A::div(s.ad_value(422), A::scale(s.ad_value(421), 3.0)));
        }

        if (((s.v[978] != 0.0) && (!(s.v[997] != 0.0))) && (s.v[1001] != 0.0)) {
            s.store_sub_ad_lhs(319, A::mul(s.ad_value(290), s.ad_value(122)), 416);
        }

        if (((s.v[978] != 0.0) && (!(s.v[997] != 0.0))) && (s.v[1001] != 0.0)) {
            s.store_mul_ad_rhs(77, 120, A::add(s.ad_value(319), s.ad_value(416)));
        }

        s.v[1002] = if (p.p30 > 0.0) { 1.0 } else { 0.0 };

        if (((s.v[978] != 0.0) && (!(s.v[997] != 0.0))) && (s.v[1002] != 0.0)) {
            s.store_offset_ad(420, A::add(s.ad_value(404), s.ad_value(416)), 0.1);
        }

        if (((s.v[978] != 0.0) && (!(s.v[997] != 0.0))) && (s.v[1002] != 0.0)) {
            s.store_offset_ad(203, A::exp(A::mul(s.ad_value(120), A::neg(s.ad_value(416)))), 1e-50);
        }

        if (((s.v[978] != 0.0) && (!(s.v[997] != 0.0))) && (s.v[1002] != 0.0)) {
            s.store_scale(278, 127, 1.0 / (s.v[624]));
        }

        if (((s.v[978] != 0.0) && (!(s.v[997] != 0.0))) && (s.v[1002] != 0.0)) {
            s.store_square(429, 278);
        }

        if (((s.v[978] != 0.0) && (!(s.v[997] != 0.0))) && (s.v[1002] != 0.0)) {
            s.store_mul(430, 429, 203);
        }

        if (((s.v[978] != 0.0) && (!(s.v[997] != 0.0))) && (s.v[1002] != 0.0)) {
            s.store_mul(278, 121, 145);
        }

        if (((s.v[978] != 0.0) && (!(s.v[997] != 0.0))) && (s.v[1002] != 0.0)) {
            s.store_mul(434, 120, 420);
        }

        if (((s.v[978] != 0.0) && (!(s.v[997] != 0.0))) && (s.v[1002] != 0.0)) {
            s.store_add_ad(433, A::sub(A::ln(A::add(A::mul(s.ad_value(430), s.ad_value(278)), A::square(s.ad_value(434)))), A::ln(A::mul(s.ad_value(429), s.ad_value(278)))), A::mul(s.ad_value(120), s.ad_value(416)));
        }

        if (((s.v[978] != 0.0) && (!(s.v[997] != 0.0))) && (s.v[1002] != 0.0)) {
            s.store_offset_ad(638, A::sub(s.ad_value(434), s.ad_value(433)), (-1.0));
        }

        if (((s.v[978] != 0.0) && (!(s.v[997] != 0.0))) && (s.v[1002] != 0.0)) {
            s.store_scale(639, 434, 4.0);
        }

        if (((s.v[978] != 0.0) && (!(s.v[997] != 0.0))) && (s.v[1002] != 0.0)) {
            s.store_ad(639, &{
                if (s.v[639] > 0.0) {
                    s.ad_value(639)
                } else {
                    A::neg(s.ad_value(639))
                }
            });
        }

        if (((s.v[978] != 0.0) && (!(s.v[997] != 0.0))) && (s.v[1002] != 0.0)) {
            s.store_sqrt_ad(639, A::add(A::square(s.ad_value(638)), s.ad_value(639)));
        }

        if (((s.v[978] != 0.0) && (!(s.v[997] != 0.0))) && (s.v[1002] != 0.0)) {
            s.store_scale_ad(279, A::offset(A::div(s.ad_value(638), s.ad_value(639)), 1.0), 0.5);
        }

        if (((s.v[978] != 0.0) && (!(s.v[997] != 0.0))) && (s.v[1002] != 0.0)) {
            s.store_scale_ad(280, A::sub_from_scalar(1.0, A::div(A::offset(s.ad_value(638), 2.0), s.ad_value(639))), 0.5);
        }

        if (((s.v[978] != 0.0) && (!(s.v[997] != 0.0))) && (s.v[1002] != 0.0)) {
            s.store_sub_ad_rhs(433, 434, A::scale(A::add(s.ad_value(638), s.ad_value(639)), 0.5));
        }

        if (((s.v[978] != 0.0) && (!(s.v[997] != 0.0))) && (s.v[1002] != 0.0)) {
            s.store_sub(434, 434, 433);
        }

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
        if (((s.v[978] != 0.0) && (!(s.v[997] != 0.0))) && (s.v[1002] != 0.0)) {
            s.store_add_ad_rhs(434, 434, A::scale(s.ad_value(120), 0.1));
        }

        if (((s.v[978] != 0.0) && (!(s.v[997] != 0.0))) && (s.v[1002] != 0.0)) {
            s.store_add_ad(432, A::sub(A::ln(A::add(A::mul(s.ad_value(430), s.ad_value(278)), A::square(s.ad_value(434)))), A::ln(A::mul(s.ad_value(429), s.ad_value(278)))), A::mul(s.ad_value(120), s.ad_value(416)));
        }

        if (((s.v[978] != 0.0) && (!(s.v[997] != 0.0))) && (s.v[1002] != 0.0)) {
            s.store_sub_ad_lhs(320, A::div(s.ad_value(432), s.ad_value(120)), 416);
        }

        if (((s.v[978] != 0.0) && (!(s.v[997] != 0.0))) && (s.v[1002] != 0.0)) {
            s.copy_ad(431, 77);
        }

        if (((s.v[978] != 0.0) && (!(s.v[997] != 0.0))) && (s.v[1002] != 0.0)) {
            s.store_offset_ad(638, A::sub(s.ad_value(432), s.ad_value(431)), (-(0.0008 * 75.0)));
        }

        if (((s.v[978] != 0.0) && (!(s.v[997] != 0.0))) && (s.v[1002] != 0.0)) {
            s.store_scale(639, 432, (4.0 * (0.0008 * 75.0)));
        }

        if (((s.v[978] != 0.0) && (!(s.v[997] != 0.0))) && (s.v[1002] != 0.0)) {
            s.store_ad(639, &{
                if (s.v[639] > 0.0) {
                    s.ad_value(639)
                } else {
                    A::neg(s.ad_value(639))
                }
            });
        }

        if (((s.v[978] != 0.0) && (!(s.v[997] != 0.0))) && (s.v[1002] != 0.0)) {
            s.store_sqrt_ad(639, A::add(A::square(s.ad_value(638)), s.ad_value(639)));
        }

        if (((s.v[978] != 0.0) && (!(s.v[997] != 0.0))) && (s.v[1002] != 0.0)) {
            s.store_scale_ad(279, A::offset(A::div(s.ad_value(638), s.ad_value(639)), 1.0), 0.5);
        }

        if (((s.v[978] != 0.0) && (!(s.v[997] != 0.0))) && (s.v[1002] != 0.0)) {
            s.store_scale_ad(280, A::sub_from_scalar(1.0, A::div(A::offset(s.ad_value(638), ((2.0 * 0.0008) * 75.0)), s.ad_value(639))), 0.5);
        }

        if (((s.v[978] != 0.0) && (!(s.v[997] != 0.0))) && (s.v[1002] != 0.0)) {
            s.store_sub_ad_rhs(77, 432, A::scale(A::add(s.ad_value(638), s.ad_value(639)), 0.5));
        }

        if ((s.v[978] != 0.0) && (!(s.v[997] != 0.0))) {
            s.store_sub_ad_lhs(410, A::div(s.ad_value(77), s.ad_value(120)), 416);
        }

        if ((s.v[978] != 0.0) && (!(s.v[997] != 0.0))) {
            s.store_add_ad(279, A::offset(s.ad_value(77), (-1.0)), A::exp(A::neg(s.ad_value(77))));
        }

        s.v[1003] = if (s.v[279] < (10.0 * 2.220446049250313e-16)) { 1.0 } else { 0.0 };

        if (((s.v[978] != 0.0) && (!(s.v[997] != 0.0))) && (s.v[1003] != 0.0)) {
            s.store_scalar(279, (10.0 * 2.220446049250313e-16));
        }

        if ((s.v[978] != 0.0) && (!(s.v[997] != 0.0))) {
            s.store_mul_ad_rhs(407, 437, A::sqrt(s.ad_value(279)));
        }

        if ((s.v[978] != 0.0) && (!(s.v[997] != 0.0))) {
            s.store_scaled_sub(408, 404, 410, s.v[435]);
        }

        s.v[1004] = if (p.p30 == 1.0) { 1.0 } else { 0.0 };

        if (((s.v[978] != 0.0) && (!(s.v[997] != 0.0))) && (s.v[1004] != 0.0)) {
            s.store_exp_ad(203, A::mul(s.ad_value(120), A::neg(s.ad_value(416))));
        }

        if (((s.v[978] != 0.0) && (!(s.v[997] != 0.0))) && (s.v[1004] != 0.0)) {
            s.store_scale(278, 127, 1.0 / (s.v[624]));
        }

        if (((s.v[978] != 0.0) && (!(s.v[997] != 0.0))) && (s.v[1004] != 0.0)) {
            s.store_square(429, 278);
        }

        if (((s.v[978] != 0.0) && (!(s.v[997] != 0.0))) && (s.v[1004] != 0.0)) {
            s.store_mul(204, 429, 203);
        }

        if (((s.v[978] != 0.0) && (!(s.v[997] != 0.0))) && (s.v[1004] != 0.0)) {
            s.store_scalar(379, 0.0);
        }

        if (((s.v[978] != 0.0) && (!(s.v[997] != 0.0))) && (s.v[1004] != 0.0)) {
            s.store_scalar(62, 1.0);
        }

        let mut assign20620_loop_guard: usize = 0;
        while {
            let assign20620_cond_e25604: f64 = (40.0 + 1.0);
            let assign20620_cond_e25606: f64 = if ((((s.v[978] != 0.0) && (!(s.v[997] != 0.0))) && (s.v[1004] != 0.0)) && (s.v[62] <= assign20620_cond_e25604)) { 1.0 } else { 0.0 };
            assign20620_cond_e25606 != 0.0
        } {
            assign20620_loop_guard += 1;
            assert!(assign20620_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.v[978] != 0.0) && (!(s.v[997] != 0.0))) && (s.v[1004] != 0.0)) {
                s.store_mul_ad_rhs(77, 120, A::add(s.ad_value(410), s.ad_value(416)));
            }
            s.v[1005] = if (s.v[77] < 5.0) { 1.0 } else { 0.0 };
            if ((((s.v[978] != 0.0) && (!(s.v[997] != 0.0))) && (s.v[1004] != 0.0)) && (s.v[1005] != 0.0)) {
                s.store_mul_ad(205, A::mul(A::square(s.ad_value(77)), s.ad_value(77)), A::offset(A::mul(s.ad_value(77), A::offset(A::scale(s.ad_value(77), 0.006115288895133179), (-0.07053654284009761))), 0.29693154855771));
            }
            if ((((s.v[978] != 0.0) && (!(s.v[997] != 0.0))) && (s.v[1004] != 0.0)) && (s.v[1005] != 0.0)) {
                s.store_mul_ad(206, A::square(s.ad_value(77)), A::offset(A::mul(s.ad_value(77), A::offset(A::scale(s.ad_value(77), (5.0 * 0.006115288895133179)), (4.0 * (-0.07053654284009761)))), (3.0 * 0.29693154855771)));
            }
            if ((((s.v[978] != 0.0) && (!(s.v[997] != 0.0))) && (s.v[1004] != 0.0)) && (s.v[1005] != 0.0)) {
                s.store_mul_ad_lhs(207, A::mul(s.ad_value(204), s.ad_value(205)), 205);
            }
            if ((((s.v[978] != 0.0) && (!(s.v[997] != 0.0))) && (s.v[1004] != 0.0)) && (s.v[1005] != 0.0)) {
                s.store_mul_ad_lhs(208, A::mul(A::scale(A::mul(s.ad_value(204), s.ad_value(120)), 2.0), s.ad_value(205)), 206);
            }
            if ((((s.v[978] != 0.0) && (!(s.v[997] != 0.0))) && (s.v[1004] != 0.0)) && (s.v[1005] != 0.0)) {
                s.store_mul_ad_rhs(146, 77, A::offset(A::mul(s.ad_value(77), A::offset(A::mul(s.ad_value(77), A::offset(A::mul(s.ad_value(77), A::offset(A::scale(s.ad_value(77), 6.36964918866352e-5), (-0.00163730162779191))), 0.0178800506338833)), (-0.117851130197758))), 0.707106781186548));
            }
            if ((((s.v[978] != 0.0) && (!(s.v[997] != 0.0))) && (s.v[1004] != 0.0)) && (s.v[1005] != 0.0)) {
                s.store_offset_ad(148, A::mul(s.ad_value(77), A::offset(A::mul(s.ad_value(77), A::offset(A::mul(s.ad_value(77), A::offset(A::scale(s.ad_value(77), (5.0 * 6.36964918866352e-5)), (4.0 * (-0.00163730162779191)))), (3.0 * 0.0178800506338833))), (2.0 * (-0.117851130197758)))), 0.707106781186548);
            }
            if ((((s.v[978] != 0.0) && (!(s.v[997] != 0.0))) && (s.v[1004] != 0.0)) && (s.v[1005] != 0.0)) {
                s.store_sqrt_ad(209, A::offset(A::add(A::square(s.ad_value(146)), s.ad_value(207)), 1e-50));
            }
            if ((((s.v[978] != 0.0) && (!(s.v[997] != 0.0))) && (s.v[1004] != 0.0)) && (s.v[1005] != 0.0)) {
                s.store_div_ad(210, A::add(A::mul(A::scale(A::mul(s.ad_value(120), s.ad_value(148)), 2.0), s.ad_value(146)), s.ad_value(208)), A::scale(s.ad_value(209), 2.0));
            }
            s.v[1006] = if (s.v[77] < 80.0) { 1.0 } else { 0.0 };
            if (((((s.v[978] != 0.0) && (!(s.v[997] != 0.0))) && (s.v[1004] != 0.0)) && (!(s.v[1005] != 0.0))) && (s.v[1006] != 0.0)) {
                s.store_exp(147, 77);
            }
            if (((((s.v[978] != 0.0) && (!(s.v[997] != 0.0))) && (s.v[1004] != 0.0)) && (!(s.v[1005] != 0.0))) && (s.v[1006] != 0.0)) {
                s.store_mul_ad_rhs(207, 204, A::offset(s.ad_value(147), (-1.0)));
            }
            if (((((s.v[978] != 0.0) && (!(s.v[997] != 0.0))) && (s.v[1004] != 0.0)) && (!(s.v[1005] != 0.0))) && (s.v[1006] != 0.0)) {
                s.store_mul_ad_lhs(208, A::mul(s.ad_value(204), s.ad_value(120)), 147);
            }
            if (((((s.v[978] != 0.0) && (!(s.v[997] != 0.0))) && (s.v[1004] != 0.0)) && (!(s.v[1005] != 0.0))) && (!(s.v[1006] != 0.0))) {
                s.store_exp_ad(202, A::mul(s.ad_value(120), s.ad_value(410)));
            }
            if (((((s.v[978] != 0.0) && (!(s.v[997] != 0.0))) && (s.v[1004] != 0.0)) && (!(s.v[1005] != 0.0))) && (!(s.v[1006] != 0.0))) {
                s.store_mul_ad_rhs(207, 429, A::sub(s.ad_value(202), s.ad_value(203)));
            }
            if (((((s.v[978] != 0.0) && (!(s.v[997] != 0.0))) && (s.v[1004] != 0.0)) && (!(s.v[1005] != 0.0))) && (!(s.v[1006] != 0.0))) {
                s.store_mul_ad_lhs(208, A::mul(s.ad_value(429), s.ad_value(120)), 202);
            }
            if ((((s.v[978] != 0.0) && (!(s.v[997] != 0.0))) && (s.v[1004] != 0.0)) && (!(s.v[1005] != 0.0))) {
                s.store_sqrt_ad(209, A::add(A::offset(s.ad_value(77), (-1.0)), s.ad_value(207)));
            }
            if ((((s.v[978] != 0.0) && (!(s.v[997] != 0.0))) && (s.v[1004] != 0.0)) && (!(s.v[1005] != 0.0))) {
                s.store_scale_ad(210, A::div(A::add(s.ad_value(120), s.ad_value(208)), s.ad_value(209)), 0.5);
            }
            if (((s.v[978] != 0.0) && (!(s.v[997] != 0.0))) && (s.v[1004] != 0.0)) {
                s.store_sub_ad(211, A::sub(s.ad_value(404), s.ad_value(410)), A::mul(s.ad_value(144), s.ad_value(209)));
            }
            if (((s.v[978] != 0.0) && (!(s.v[997] != 0.0))) && (s.v[1004] != 0.0)) {
                s.store_sub_from_scalar_ad(212, (-1.0), A::mul(s.ad_value(144), s.ad_value(210)));
            }
            s.v[1007] = if (s.v[379] == 1.0) { 1.0 } else { 0.0 };
            if ((((s.v[978] != 0.0) && (!(s.v[997] != 0.0))) && (s.v[1004] != 0.0)) && (s.v[1007] != 0.0)) {
                s.store_scalar(62, (40.0 + 1.0));
            }
            if ((((s.v[978] != 0.0) && (!(s.v[997] != 0.0))) && (s.v[1004] != 0.0)) && (!(s.v[1007] != 0.0))) {
                s.store_div_ad_lhs(213, A::neg(s.ad_value(211)), 212);
            }
            if ((((s.v[978] != 0.0) && (!(s.v[997] != 0.0))) && (s.v[1004] != 0.0)) && (!(s.v[1007] != 0.0))) {
                s.store_scale_ad(214, A::offset({
                    if (1.0 >= ((s.v[410]) as f64).abs()) {
                        A::constant(1.0)
                    } else {
                        A::abs(s.ad_value(410))
                    }
                }, 1.0), (0.5 * 0.1));
            }
            s.v[1008] = if (((s.v[213]) as f64).abs() > s.v[214]) { 1.0 } else { 0.0 };
            if (((((s.v[978] != 0.0) && (!(s.v[997] != 0.0))) && (s.v[1004] != 0.0)) && (!(s.v[1007] != 0.0))) && (s.v[1008] != 0.0)) {
                s.store_scale(213, 214, (if (s.v[213] >= 0.0) { 1.0 } else { (-1.0) }));
            }
            if ((((s.v[978] != 0.0) && (!(s.v[997] != 0.0))) && (s.v[1004] != 0.0)) && (!(s.v[1007] != 0.0))) {
                s.store_add(410, 410, 213);
            }
            s.v[1009] = if ((((s.v[213]) as f64).abs() <= 1e-12) && (((s.v[211]) as f64).abs() <= 1e-8)) { 1.0 } else { 0.0 };
            if (((((s.v[978] != 0.0) && (!(s.v[997] != 0.0))) && (s.v[1004] != 0.0)) && (!(s.v[1007] != 0.0))) && (s.v[1009] != 0.0)) {
                s.store_scalar(379, 1.0);
            }
            if (((s.v[978] != 0.0) && (!(s.v[997] != 0.0))) && (s.v[1004] != 0.0)) {
                s.store_offset(62, 62, 1.0);
            }
        }

        s.v[1011] = if (s.v[77] < 5.0) { 1.0 } else { 0.0 };

        if ((((s.v[978] != 0.0) && (!(s.v[997] != 0.0))) && (s.v[1004] != 0.0)) && (s.v[1011] != 0.0)) {
            s.store_offset_ad(64, A::square(s.ad_value(146)), (10.0 * 2.220446049250313e-16));
        }

        if ((((s.v[978] != 0.0) && (!(s.v[997] != 0.0))) && (s.v[1004] != 0.0)) && (s.v[1011] != 0.0)) {
            s.store_offset(65, 146, (10.0 * 2.220446049250313e-16));
        }

        if ((((s.v[978] != 0.0) && (!(s.v[997] != 0.0))) && (s.v[1004] != 0.0)) && (!(s.v[1011] != 0.0))) {
            s.store_offset(64, 77, (-1.0));
        }

        if ((((s.v[978] != 0.0) && (!(s.v[997] != 0.0))) && (s.v[1004] != 0.0)) && (!(s.v[1011] != 0.0))) {
            s.store_sqrt(65, 64);
        }

        if (((s.v[978] != 0.0) && (!(s.v[997] != 0.0))) && (s.v[1004] != 0.0)) {
            s.store_mul(407, 437, 65);
        }

        if (((s.v[978] != 0.0) && (!(s.v[997] != 0.0))) && (s.v[1004] != 0.0)) {
            s.store_div_from_scalar_ad(279, 1.0, A::add(s.ad_value(209), s.ad_value(65)));
        }

        if (((s.v[978] != 0.0) && (!(s.v[997] != 0.0))) && (s.v[1004] != 0.0)) {
            s.store_mul_ad_lhs(409, A::mul(s.ad_value(437), s.ad_value(207)), 279);
        }

        if (((s.v[978] != 0.0) && (!(s.v[997] != 0.0))) && (s.v[1004] != 0.0)) {
            s.store_add(408, 407, 409);
        }

        if (s.v[978] != 0.0) {
            s.store_sub(409, 408, 407);
        }

        if (s.v[978] != 0.0) {
            s.store_scale(282, 195, s.v[513]);
        }

        if ((s.v[978] != 0.0) && (s.v[402] != 0.0)) {
            s.store_mul(398, 282, 408);
        }

        if ((s.v[978] != 0.0) && (s.v[402] != 0.0)) {
            s.store_mul(406, 282, 407);
        }

        if ((s.v[978] != 0.0) && (s.v[403] != 0.0)) {
            s.store_mul(397, 282, 408);
        }

        if ((s.v[978] != 0.0) && (s.v[403] != 0.0)) {
            s.store_mul(405, 282, 407);
        }

        if (s.v[978] != 0.0) {
            s.store_add_ad(194, A::scale(s.ad_value(413), s.v[519]), A::scale(s.ad_value(412), s.v[518]));
        }

        if ((s.v[978] != 0.0) && (s.v[194] != 0.0)) {
            s.store_add_ad(198, A::scale(s.ad_value(413), p.p174), A::scale(s.ad_value(412), p.p173));
        }

        if ((s.v[978] != 0.0) && (s.v[194] != 0.0)) {
            s.store_scale(198, 198, (-s.v[513]));
        }

        if ((s.v[978] != 0.0) && (s.v[194] != 0.0)) {
            s.store_offset_ad(197, A::mul(A::neg(s.ad_value(198)), A::sub(s.ad_value(52), s.ad_value(51))), s.v[197]);
        }

        if (s.v[978] != 0.0) {
            s.store_add_ad(194, A::scale(s.ad_value(412), s.v[519]), A::scale(s.ad_value(413), s.v[518]));
        }

        if ((s.v[978] != 0.0) && (s.v[194] != 0.0)) {
            s.store_add_ad(199, A::scale(s.ad_value(412), p.p174), A::scale(s.ad_value(413), p.p173));
        }

        if ((s.v[978] != 0.0) && (s.v[194] != 0.0)) {
            s.store_scale(199, 199, (-s.v[513]));
        }

        if ((s.v[978] != 0.0) && (s.v[194] != 0.0)) {
            s.store_offset_ad(196, A::mul(A::neg(s.ad_value(199)), s.ad_value(52)), s.v[196]);
        }

        s.v[1013] = if (((s.v[575] == 1.0) && (!(s.v[518] != 0.0))) || ((s.v[575] != 1.0) && (!(s.v[519] != 0.0)))) { 1.0 } else { 0.0 };

        s.v[1014] = if (p.p175 > 0.0) { 1.0 } else { 0.0 };

        if (((!(s.v[978] != 0.0)) && (s.v[1013] != 0.0)) && (s.v[1014] != 0.0)) {
            s.store_scalar(198, (((-s.v[435]) * p.p175) * s.v[513]));
        }

        if (((!(s.v[978] != 0.0)) && (s.v[1013] != 0.0)) && (!(s.v[1014] != 0.0))) {
            s.store_scalar(198, 0.0);
        }

        if ((!(s.v[978] != 0.0)) && (!(s.v[1013] != 0.0))) {
            s.store_add_ad(198, A::scale(s.ad_value(413), p.p174), A::scale(s.ad_value(412), p.p173));
        }

        if ((!(s.v[978] != 0.0)) && (!(s.v[1013] != 0.0))) {
            s.store_scale(198, 198, (-s.v[513]));
        }

        if (!(s.v[978] != 0.0)) {
            s.store_mul_ad(197, A::neg(s.ad_value(198)), A::sub(s.ad_value(52), s.ad_value(51)));
        }

        s.v[1015] = if (((s.v[575] == 1.0) && (!(s.v[519] != 0.0))) || ((s.v[575] != 1.0) && (!(s.v[518] != 0.0)))) { 1.0 } else { 0.0 };

        if ((!(s.v[978] != 0.0)) && (s.v[1015] != 0.0)) {
            s.store_scalar(199, (((-s.v[435]) * p.p175) * s.v[513]));
        }

        if ((!(s.v[978] != 0.0)) && (!(s.v[1015] != 0.0))) {
            s.store_add_ad(199, A::scale(s.ad_value(412), p.p174), A::scale(s.ad_value(413), p.p173));
        }

        if ((!(s.v[978] != 0.0)) && (!(s.v[1015] != 0.0))) {
            s.store_scale(199, 199, (-s.v[513]));
        }

        if (!(s.v[978] != 0.0)) {
            s.store_mul_ad_lhs(196, A::neg(s.ad_value(199)), 52);
        }

        s.v[1016] = if (s.v[34] == 0.0) { 1.0 } else { 0.0 };

        if ((s.v[38] != 0.0) && (s.v[1016] != 0.0)) {
            s.store_mul_ad_lhs(279, A::scale(s.ad_value(386), (p.p223 * p.p224)), 386);
        }

        if ((s.v[38] != 0.0) && (s.v[1016] != 0.0)) {
            s.store_offset_ad(280, A::add(A::scale(A::mul(s.ad_value(158), s.ad_value(86)), p.p223), A::mul(A::scale(s.ad_value(386), p.p224), s.ad_value(386))), 1e-50);
        }

        if ((s.v[38] != 0.0) && (s.v[1016] != 0.0)) {
            s.store_div(221, 279, 280);
        }

        if ((s.v[38] != 0.0) && (!(s.v[1016] != 0.0))) {
            s.store_scalar(221, (p.p223 + 1e-50));
        }

        if (s.v[38] != 0.0) {
            s.store_scale(222, 270, (p.p225 * 0.0001));
        }

        s.v[1017] = if ((p.p21 != 0.0) && (!(s.v[34] != 0.0))) { 1.0 } else { 0.0 };

        if (s.v[1017] != 0.0) {
            s.store_scalar(223, s.v[617]);
        }

        if (s.v[1017] != 0.0) {
            s.store_scalar(224, s.v[618]);
        }

        if (s.v[1017] != 0.0) {
            s.store_scalar(225, s.v[619]);
        }

        if (s.v[1017] != 0.0) {
            s.store_scale(279, 149, 6.241449993689894e18);
        }

        if (s.v[1017] != 0.0) {
            s.store_scale_ad(280, A::mul(A::add(A::add(s.ad_value(270), A::div(s.ad_value(149), A::sub(s.ad_value(56), s.ad_value(50)))), s.ad_value(225)), s.ad_value(122)), 6.241449993689894e18);
        }

        if (s.v[1017] != 0.0) {
            s.store_sub_ad_lhs(281, A::scale(A::div(A::scale(s.ad_value(91), ((-2.0) * 6.241449993689894e18)), s.ad_value(386)), 1.0 / (s.v[513])), 279);
        }

        s.v[1018] = if ((((s.v[281] - s.v[279])) as f64).abs() > (10.0 * 2.220446049250313e-16)) { 1.0 } else { 0.0 };

        if ((s.v[1017] != 0.0) && (s.v[1018] != 0.0)) {
            let assign21170_ad_e26697: A = A::add(A::div(A::div_from_scalar(1.0, A::add(s.ad_value(279), s.ad_value(280))), A::add(s.ad_value(281), s.ad_value(280))), A::mul(A::div(A::mul(A::mul(A::scale(s.ad_value(223), 2.0), s.ad_value(160)), s.ad_value(158)), A::sub(s.ad_value(281), s.ad_value(279))), A::ln(A::div(A::add(s.ad_value(281), s.ad_value(280)), A::add(s.ad_value(279), s.ad_value(280))))));
            s.store_add_ad(282, assign21170_ad_e26697, A::mul(A::mul(A::mul(A::mul(A::mul(s.ad_value(223), s.ad_value(160)), s.ad_value(158)), s.ad_value(223)), s.ad_value(160)), s.ad_value(158)));
        }

        if ((s.v[1017] != 0.0) && (!(s.v[1018] != 0.0))) {
            let assign21180_ad_e26748: A = A::add(A::add(A::div(A::div_from_scalar(1.0, A::add(s.ad_value(279), s.ad_value(280))), A::add(s.ad_value(281), s.ad_value(280))), A::div(A::mul(A::mul(A::scale(s.ad_value(223), 2.0), s.ad_value(160)), s.ad_value(158)), A::add(s.ad_value(279), s.ad_value(280)))), A::mul(A::mul(A::mul(A::mul(A::mul(s.ad_value(223), s.ad_value(160)), s.ad_value(158)), s.ad_value(223)), s.ad_value(160)), s.ad_value(158)));
            s.store_ad(282, &assign21180_ad_e26748);
        }

        if (s.v[1017] != 0.0) {
            s.store_mul_ad_lhs(226, A::div(A::mul(A::square(s.ad_value(94)), s.ad_value(224)), A::scale(A::mul(s.ad_value(386), s.ad_value(120)), s.v[466])), 282);
        }

        if (!(s.v[1017] != 0.0)) {
            s.store_scalar(226, 0.0);
        }

        s.v[1019] = if ((p.p23 != 0.0) && (!(s.v[34] != 0.0))) { 1.0 } else { 0.0 };

        if (s.v[1019] != 0.0) {
            s.store_div_ad_lhs(227, A::sub(s.ad_value(260), s.ad_value(56)), 386);
        }

        if (s.v[1019] != 0.0) {
            s.store_scaled_mul(289, 159, 227, 1.0 / ((10000000.0 * 0.01)));
        }

        s.v[1020] = if (((1.0 - (10.0 * 2.220446049250313e-16)) <= p.p114) && (p.p114 <= (1.0 + (10.0 * 2.220446049250313e-16)))) { 1.0 } else { 0.0 };

        if ((s.v[1019] != 0.0) && (s.v[1020] != 0.0)) {
            s.store_scalar(285, 1.0);
        }

        s.v[1021] = if (((2.0 - (10.0 * 2.220446049250313e-16)) <= p.p114) && (p.p114 <= (2.0 + (10.0 * 2.220446049250313e-16)))) { 1.0 } else { 0.0 };

        if (((s.v[1019] != 0.0) && (!(s.v[1020] != 0.0))) && (s.v[1021] != 0.0)) {
            s.copy_ad(285, 289);
        }

        if (((s.v[1019] != 0.0) && (!(s.v[1020] != 0.0))) && (!(s.v[1021] != 0.0))) {
            s.store_powf(285, 289, (p.p114 - 1.0));
        }

        if (s.v[1019] != 0.0) {
            s.store_offset_ad(287, A::mul(s.ad_value(289), s.ad_value(285)), 1.0);
        }

        if (s.v[1019] != 0.0) {
            s.store_powf(288, 287, (((-1.0) / p.p114) - 1.0));
        }

        if (s.v[1019] != 0.0) {
            s.store_mul_ad_lhs(230, A::mul(s.ad_value(159), s.ad_value(287)), 288);
        }

        if (s.v[1019] != 0.0) {
            s.store_scaled_add(228, 158, 230, 0.5);
        }

        if (s.v[1019] != 0.0) {
            s.store_square(278, 85);
        }

        if (s.v[1019] != 0.0) {
            let assign21340_ad_e26942: A = A::add(A::add(A::mul(A::mul(A::add(A::offset(A::scale(s.ad_value(85), 3.0), 1.0), A::scale(s.ad_value(278), 6.0)), s.ad_value(230)), s.ad_value(230)), A::mul(A::mul(A::add(A::offset(A::scale(s.ad_value(85), 4.0), 3.0), A::scale(s.ad_value(278), 3.0)), s.ad_value(230)), s.ad_value(158))), A::mul(A::mul(A::add(A::offset(A::scale(s.ad_value(85), 3.0), 6.0), s.ad_value(278)), s.ad_value(158)), s.ad_value(158)));
            s.store_div_ad(229, A::mul(A::mul(A::mul(A::scale(s.ad_value(270), s.v[466]), s.ad_value(86)), s.ad_value(158)), assign21340_ad_e26942), A::mul(A::mul(A::mul(A::scale(s.ad_value(386), 15.0), A::offset(s.ad_value(85), 1.0)), s.ad_value(228)), s.ad_value(228)));
        }

        if (!(s.v[1019] != 0.0)) {
            s.store_scalar(229, 0.0);
        }

        s.v[1022] = if ((((p.p20 != 0.0) && (p.p23 != 0.0)) && (s.v[35] == 1.0)) && (!(s.v[34] != 0.0))) { 1.0 } else { 0.0 };

        if (s.v[1022] != 0.0) {
            s.store_sqrt(235, 233);
        }

        if (s.v[1022] != 0.0) {
            s.store_add(280, 86, 235);
        }

        if (s.v[1022] != 0.0) {
            s.store_square(281, 231);
        }

        if (s.v[1022] != 0.0) {
            s.store_square(282, 233);
        }

        if (s.v[1022] != 0.0) {
            s.store_mul_ad_lhs(283, A::scale(s.ad_value(231), 42.0), 233);
        }

        if (s.v[1022] != 0.0) {
            s.store_add_ad_rhs(283, 283, A::scale(A::add(s.ad_value(281), s.ad_value(282)), 4.0));
        }

        if (s.v[1022] != 0.0) {
            s.store_add_ad_rhs(283, 283, A::mul(A::mul(A::scale(s.ad_value(235), 20.0), s.ad_value(86)), A::add(s.ad_value(231), s.ad_value(233))));
        }

        if (s.v[1022] != 0.0) {
            s.store_square(288, 280);
        }

        if (s.v[1022] != 0.0) {
            s.store_div_ad_rhs(236, 283, A::mul(A::square(s.ad_value(288)), s.ad_value(280)));
        }

        if (s.v[1022] != 0.0) {
            s.store_mul_ad_lhs(237, A::mul(A::div_from_scalar(s.v[466], s.ad_value(386)), s.ad_value(158)), 270);
        }

        if (s.v[1022] != 0.0) {
            s.store_mul(238, 237, 86);
        }

        if (s.v[1022] != 0.0) {
            s.store_div(239, 229, 238);
        }

        if (s.v[1022] != 0.0) {
            s.store_add_ad_lhs(285, A::add(s.ad_value(231), A::mul(A::scale(s.ad_value(86), 4.0), s.ad_value(235))), 233);
        }

        if (s.v[1022] != 0.0) {
            s.store_div_ad(240, A::mul(A::scale(s.ad_value(234), 3.872983346207417), s.ad_value(285)), A::mul(A::scale(s.ad_value(280), 6.0), A::sqrt(A::mul(A::mul(A::mul(s.ad_value(239), s.ad_value(280)), s.ad_value(86)), s.ad_value(283)))));
        }

        s.store_add(94, 94, 193);

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
        if (s.v[517] != 0.0) {
            s.store_scalar(200, ((-p.p172) * s.v[277]));
        }

        if (s.v[517] != 0.0) {
            s.store_mul_ad_rhs(201, 200, A::sub(s.ad_value(42), s.ad_value(40)));
        }

        if (!(s.v[517] != 0.0)) {
            s.store_scalar(200, 0.0);
        }

        if (!(s.v[517] != 0.0)) {
            s.store_scalar(201, 0.0);
        }

        s.v[215] = 0.0;

        s.store_scaled_sub(216, 42, 41, s.v[215]);

        s.store_scale(217, 42, s.v[215]);

        s.store_add(197, 197, 216);

        s.store_add(196, 196, 217);

        s.store_scale(0, 94, s.v[394]);

        s.store_scale(279, 123, (-s.v[513]));

        s.store_scaled_add(280, 523, 576, (-0.5));

        s.store_scaled_add(281, 531, 585, (-0.5));

        s.store_mul_ad_lhs(444, A::scale(s.ad_value(279), (0.1 * s.v[294])), 40);

        s.store_mul_ad(443, A::scale(s.ad_value(279), (0.1 * s.v[294])), A::sub(s.ad_value(40), s.ad_value(41)));

        s.store_mul(441, 279, 280);

        s.store_mul(442, 279, 281);

        if (p.p303 != 0.0) {
            s.store_scalar(336, 0.0);
        }

        if (p.p303 != 0.0) {
            s.copy_ad(92, 91);
        }

        if (!(p.p303 != 0.0)) {
            s.store_add_ad_lhs(92, A::add(s.ad_value(91), s.ad_value(441)), 442);
        }

        s.store_scale(93, 92, s.v[385]);

        if (s.v[38] != 0.0) {
            s.store_scalar(15, 0.0);
        }

        if (s.v[38] != 0.0) {
            s.store_scalar(14, 0.0);
        }

        if (s.v[38] != 0.0) {
            s.store_scalar(492, 0.0);
        }

        if (s.v[38] != 0.0) {
            s.store_scale(556, 336, s.v[394]);
        }

        if (s.v[38] != 0.0) {
            s.store_scale(555, 92, s.v[394]);
        }

        if (!(s.v[38] != 0.0)) {
            s.store_scale_ad(14, A::sub(A::neg(s.ad_value(336)), s.ad_value(92)), s.v[394]);
        }

        if (!(s.v[38] != 0.0)) {
            s.store_scaled_add(15, 93, 443, s.v[394]);
        }

        if (!(s.v[38] != 0.0)) {
            s.store_scale_ad(16, A::add(A::sub(s.ad_value(92), s.ad_value(93)), s.ad_value(444)), s.v[394]);
        }

        s.v[1023] = if (p.p45 == 0.0) { 1.0 } else { 0.0 };

        if (s.v[1023] != 0.0) {
            s.store_scalar(219, 0.0);
        }

        if (!(s.v[1023] != 0.0)) {
            s.store_add_ad_lhs(218, A::mul(s.ad_value(261), s.ad_value(123)), 56);
        }

        s.v[1024] = if (s.v[218] > s.v[260]) { 1.0 } else { 0.0 };

        if ((!(s.v[1023] != 0.0)) && (s.v[1024] != 0.0)) {
            s.copy_ad(218, 260);
        }

        if (!(s.v[1023] != 0.0)) {
            s.store_add_ad(279, A::scale(A::add(s.ad_value(51), s.ad_value(56)), s.v[264]), A::scale(s.ad_value(218), (1.0 - s.v[264])));
        }

        if (!(s.v[1023] != 0.0)) {
            s.store_sqrt_ad(288, A::div_from_scalar((2.0 * 1.034943e-10), s.ad_value(126)));
        }

        if (!(s.v[1023] != 0.0)) {
            s.store_scale(281, 288, 1.3);
        }

        if (!(s.v[1023] != 0.0)) {
            s.store_scale(280, 281, (1.034943e-10 * s.v[513]));
        }

        if (!(s.v[1023] != 0.0)) {
            s.store_mul_ad_lhs(219, A::sub(A::scale(A::sub(A::add(s.ad_value(56), s.ad_value(51)), s.ad_value(279)), 1.0 / (p.p45)), s.ad_value(261)), 280);
        }

        s.v[1025] = if (p.p46 != 0.0) { 1.0 } else { 0.0 };

        if (s.v[1025] != 0.0) {
            s.store_add_ad_rhs(219, 219, A::scale(s.ad_value(50), s.v[490]));
        }

        s.v[1026] = if (p.p14 == 1.0) { 1.0 } else { 0.0 };

        if (s.v[1026] != 0.0) {
            s.store_add_ad_rhs(14, 14, A::scale(A::sub(A::sub(A::sub(A::sub(A::add(s.ad_value(197), s.ad_value(196)), s.ad_value(201)), s.ad_value(219)), s.ad_value(398)), s.ad_value(397)), s.v[394]));
        }

        if (s.v[1026] != 0.0) {
            s.store_add_ad_rhs(15, 15, A::scale(A::add(A::sub(s.ad_value(219), s.ad_value(197)), s.ad_value(405)), s.v[394]));
        }

        if (s.v[1026] != 0.0) {
            s.store_add_ad_rhs(16, 16, A::scale(A::sub(s.ad_value(406), s.ad_value(196)), s.v[394]));
        }

        s.store_scale(494, 185, s.v[394]);

        s.store_scale_ad(6, A::neg(s.ad_value(254)), s.v[394]);

        s.v[1027] = if (s.v[575] == 1.0) { 1.0 } else { 0.0 };

        if (s.v[1027] != 0.0) {
            s.store_scale_ad(4, A::sub(A::mul(s.ad_value(256), s.ad_value(255)), s.ad_value(251)), s.v[394]);
        }

        if (!(s.v[1027] != 0.0)) {
            s.store_sub_from_scalar(279, 1.0, 256);
        }

        if (!(s.v[1027] != 0.0)) {
            s.store_scale_ad(4, A::sub(A::mul(s.ad_value(279), s.ad_value(255)), s.ad_value(250)), s.v[394]);
        }

        s.v[1028] = if (s.v[575] == 1.0) { 1.0 } else { 0.0 };

        if (s.v[1028] != 0.0) {
            s.store_sub_from_scalar(279, 1.0, 256);
        }

        if (s.v[1028] != 0.0) {
            s.store_scale_ad(5, A::sub(A::mul(s.ad_value(279), s.ad_value(255)), s.ad_value(250)), s.v[394]);
        }

        if (!(s.v[1028] != 0.0)) {
            s.store_scale_ad(5, A::sub(A::mul(s.ad_value(256), s.ad_value(255)), s.ad_value(251)), s.v[394]);
        }

        if (s.v[575] == 1.0) {
            s.store_scale(2, 257, s.v[394]);
        } else {
            s.store_scale(2, 258, s.v[394]);
        }

        if (s.v[575] == 1.0) {
            s.store_scale(3, 258, s.v[394]);
        } else {
            s.store_scale(3, 257, s.v[394]);
        }

        s.store_scale(573, 374, (4.0 * 1.3806226e-23));

        s.store_scale(563, 226, s.v[394]);

        s.store_scale(564, 229, s.v[394]);

        s.store_scalar(18, A::ddx_projection(&s.ad_value(14), Some(11), None));

        s.store_scale(18, 18, p.p33);

        s.store_scalar(19, A::ddx_projection(&s.ad_value(14), Some(12), None));

        s.store_scale(19, 19, p.p33);

        if (s.v[575] > 0.0) {
            s.copy_ad(493, 19);
        } else {
            s.copy_ad(493, 18);
        }

        s.v[1029] = if ((((p.p20 != 0.0) && (p.p23 != 0.0)) && (s.v[35] == 1.0)) && (!(s.v[34] != 0.0))) { 1.0 } else { 0.0 };

        if (s.v[1029] != 0.0) {
            s.store_mul_ad_lhs(278, A::scale(s.ad_value(270), (1e-6 * s.v[513])), 123);
        }

        if (s.v[1029] != 0.0) {
            s.store_scale(288, 493, 1.0 / (s.v[394]));
        }

        if (s.v[1029] != 0.0) {
            s.store_div_ad_lhs(241, A::mul(A::mul(A::scale(s.ad_value(122), (0.1185185185185185 * 1.6021918e-19)), s.ad_value(288)), s.ad_value(288)), 237);
        }

        s.v[1030] = if ((s.v[234] > (10.0 * 2.220446049250313e-16)) && (s.v[51] > (10.0 * 2.220446049250313e-16))) { 1.0 } else { 0.0 };

        if ((s.v[1029] != 0.0) && (s.v[1030] != 0.0)) {
            s.store_div(242, 159, 158);
        }

        if ((s.v[1029] != 0.0) && (s.v[1030] != 0.0)) {
            s.store_div_ad_lhs(243, A::sub(A::div(s.ad_value(159), s.ad_value(230)), s.ad_value(242)), 51);
        }

        if ((s.v[1029] != 0.0) && (s.v[1030] != 0.0)) {
            s.store_add_ad_rhs(244, 242, A::div(A::mul(A::scale(s.ad_value(243), 0.6666666666666667), A::add(A::add(s.ad_value(231), A::mul(s.ad_value(86), s.ad_value(235))), s.ad_value(233))), A::add(s.ad_value(86), s.ad_value(235))));
        }

        if ((s.v[1029] != 0.0) && (!(s.v[1030] != 0.0))) {
            s.store_div(244, 159, 230);
        }

        if (s.v[1029] != 0.0) {
            s.store_mul_ad_lhs(495, A::mul(A::scale(s.ad_value(241), s.v[394]), s.ad_value(236)), 244);
        }

        if (s.v[1029] != 0.0) {
            s.copy_ad(496, 240);
        }

        if (s.v[1029] != 0.0) {
            s.store_ad(495, &{
                if (s.v[495] < 0.0) {
                    A::constant(0.0)
                } else {
                    s.ad_value(495)
                }
            });
        }

        if (s.v[1029] != 0.0) {
            s.store_ad(495, &{
                if ((-s.v[288]) > s.v[278]) {
                    s.ad_value(495)
                } else {
                    A::constant(0.0)
                }
            });
        }

        if (s.v[1029] != 0.0) {
            s.store_ad(496, &{
                if ((-s.v[288]) > s.v[278]) {
                    s.ad_value(496)
                } else {
                    A::constant(0.0)
                }
            });
        }

        if (!(s.v[1029] != 0.0)) {
            s.store_scalar(495, 0.0);
        }

        if (!(s.v[1029] != 0.0)) {
            s.store_scalar(496, 0.0);
        }

        s.store_mul(608, 573, 564);

        s.copy_ad(609, 496);

        if ((s.v[608] > 0.0) && (s.v[495] > 0.0)) {
            s.store_sqrt_ad(610, A::div(s.ad_value(495), s.ad_value(608)));
        } else {
            s.store_scalar(610, 0.0);
        }

        if (s.v[575] > 0.0) {
            s.store_scale(611, 610, (1.0 - s.v[385]));
        } else {
            s.store_scale(611, 610, s.v[385]);
        }

        if (s.v[575] > 0.0) {
            s.store_scale(612, 610, s.v[385]);
        } else {
            s.store_scale(612, 610, (1.0 - s.v[385]));
        }

        s.v[632] = 0.0;

        s.v[633] = 0.0;

        s.v[635] = 0.0;

        s.v[636] = 0.0;

        s.v[1031] = if (p.p312 == 1.0) { 1.0 } else { 0.0 };

        if (s.v[1031] != 0.0) {
            s.store_scalar(1042, (p.p315 / 1e-6));
        }

        if (s.v[1031] != 0.0) {
            s.store_scalar(1035, p.p317);
        }

        if (s.v[1031] != 0.0) {
            s.store_scalar(1036, p.p319);
        }

        if (s.v[1031] != 0.0) {
            s.store_scalar(1037, p.p324);
        }

        if (s.v[1031] != 0.0) {
            s.store_scalar(1038, (if (p.p314 > 0.0) { (p.p314 * p.p308) } else { 0.0 }));
        }

        if (s.v[1031] != 0.0) {
            s.store_scalar(1041, p.p311);
        }

        if (s.v[1031] != 0.0) {
            s.store_ad(1039, &A::scale(A::voltage(ctx, &nodes, Some(12), Some(2)), p.p33));
        }

        if (s.v[1031] != 0.0) {
            s.store_scalar(1048, ((((p.p322 * p.p322) + (p.p38 * p.p38))) as f64).sqrt());
        }

        if (s.v[1031] != 0.0) {
            s.store_scalar(1050, (s.v[124] * p.p5));
        }

        if (s.v[1031] != 0.0) {
            s.store_scale(1035, 1035, 0.0001);
        }

        if (s.v[1031] != 0.0) {
            s.store_scale(1036, 1036, 0.01);
        }

        if (s.v[1031] != 0.0) {
            s.store_scale(1040, 374, 1.0 / (s.v[445]));
        }

        if (s.v[1031] != 0.0) {
            s.store_powf(279, 1040, p.p320);
        }

        if (s.v[1031] != 0.0) {
            s.store_div(1043, 1035, 279);
        }

        if (s.v[1031] != 0.0) {
            s.store_sub_ad(278, A::add(A::offset(A::scale(s.ad_value(1040), 0.4), 1.8), A::mul(A::scale(s.ad_value(1040), 0.1), s.ad_value(1040))), A::scale(A::sub_from_scalar(1.0, s.ad_value(1040)), p.p321));
        }

        if (s.v[1031] != 0.0) {
            s.store_div(1044, 1036, 278);
        }

        if (s.v[1031] != 0.0) {
            s.store_add_ad_rhs(1037, 1037, A::scale(A::offset(s.ad_value(374), (-s.v[445])), p.p325));
        }

        if (s.v[1031] != 0.0) {
            s.store_scalar(1032, (1.0 + (p.p330 / ((s.v[375]) as f64).powf(p.p331))));
        }

        if (s.v[1031] != 0.0) {
            s.store_scalar(1034, (1.0 + (p.p328 / ((s.v[375]) as f64).powf(p.p329))));
        }

        if (s.v[1031] != 0.0) {
            s.store_scalar(1033, (1.0 + (p.p326 / ((s.v[376]) as f64).powf(p.p327))));
        }

        if (s.v[1031] != 0.0) {
            s.store_mul(1043, 1043, 1032);
        }

        if (s.v[1031] != 0.0) {
            s.store_offset_ad(1044, A::mul(A::mul(s.ad_value(1044), s.ad_value(1033)), s.ad_value(1034)), 1e-50);
        }

        if (s.v[1031] != 0.0) {
            s.store_div(1045, 1039, 1041);
        }

        if (s.v[1031] != 0.0) {
            s.store_mul(1046, 1043, 1045);
        }

        s.v[1051] = if (s.v[1039] >= 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1031] != 0.0) && (s.v[1051] != 0.0)) {
            s.store_div(279, 1046, 1044);
        }

        if ((s.v[1031] != 0.0) && (!(s.v[1051] != 0.0))) {
            s.store_div_ad_lhs(279, A::neg(s.ad_value(1046)), 1044);
        }

        s.v[1052] = if (((1.0 - (10.0 * 2.220446049250313e-16)) <= s.v[1037]) && (s.v[1037] <= (1.0 + (10.0 * 2.220446049250313e-16)))) { 1.0 } else { 0.0 };

        if ((s.v[1031] != 0.0) && (s.v[1052] != 0.0)) {
            s.store_scalar(281, 1.0);
        }

        s.v[1053] = if (((2.0 - (10.0 * 2.220446049250313e-16)) <= s.v[1037]) && (s.v[1037] <= (2.0 + (10.0 * 2.220446049250313e-16)))) { 1.0 } else { 0.0 };

        if (((s.v[1031] != 0.0) && (!(s.v[1052] != 0.0))) && (s.v[1053] != 0.0)) {
            s.copy_ad(281, 279);
        }

        if (((s.v[1031] != 0.0) && (!(s.v[1052] != 0.0))) && (!(s.v[1053] != 0.0))) {
            s.store_ad(281, &A::pow(s.ad_value(279), A::offset(s.ad_value(1037), (-1.0))));
        }

        if (s.v[1031] != 0.0) {
            s.store_mul(280, 279, 281);
        }

        if (s.v[1031] != 0.0) {
            s.store_offset(282, 280, 1.0);
        }

        s.v[1054] = if (((1.0 - (10.0 * 2.220446049250313e-16)) <= s.v[1037]) && (s.v[1037] <= (1.0 + (10.0 * 2.220446049250313e-16)))) { 1.0 } else { 0.0 };

        if ((s.v[1031] != 0.0) && (s.v[1054] != 0.0)) {
            s.store_div_from_scalar(283, 1.0, 282);
        }

        s.v[1055] = if (((2.0 - (10.0 * 2.220446049250313e-16)) <= s.v[1037]) && (s.v[1037] <= (2.0 + (10.0 * 2.220446049250313e-16)))) { 1.0 } else { 0.0 };

        if (((s.v[1031] != 0.0) && (!(s.v[1054] != 0.0))) && (s.v[1055] != 0.0)) {
            s.store_div_from_scalar_ad(283, 1.0, A::sqrt(s.ad_value(282)));
        }

        if (((s.v[1031] != 0.0) && (!(s.v[1054] != 0.0))) && (!(s.v[1055] != 0.0))) {
            s.store_ad(284, &A::pow(s.ad_value(282), A::offset(A::div_from_scalar((-1.0), s.ad_value(1037)), (-1.0))));
        }

        if (((s.v[1031] != 0.0) && (!(s.v[1054] != 0.0))) && (!(s.v[1055] != 0.0))) {
            s.store_mul(283, 282, 284);
        }

        if (s.v[1031] != 0.0) {
            s.store_mul(1047, 1043, 283);
        }

        if (s.v[1031] != 0.0) {
            s.store_div_from_scalar(279, 1.6021918e-19, 1041);
        }

        if (s.v[1031] != 0.0) {
            s.store_mul_ad_lhs(1049, A::mul(A::mul(s.ad_value(279), s.ad_value(1048)), s.ad_value(1047)), 1042);
        }

        s.v[1056] = if (s.v[1049] <= 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1031] != 0.0) && (s.v[1056] != 0.0)) {
            s.store_scalar(1049, 1e-50);
        }

        if (s.v[1031] != 0.0) {
            s.store_div_from_scalar(27, 1.0, 1049);
        }

        if (s.v[1031] != 0.0) {
            s.store_div(27, 27, 1050);
        }

        if (s.v[1031] != 0.0) {
            s.store_add(27, 27, 1038);
        }

        if (s.v[1031] != 0.0) {
            s.store_ad(634, &{
                if ((s.v[27] > 0.0001) && (p.p23 != 0.0)) {
                    A::div_from_scalar(s.v[394], s.ad_value(27))
                } else {
                    A::constant(0.0)
                }
            });
        }

        s.v[1057] = if (s.v[27] < 0.0001) { 1.0 } else { 0.0 };

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
        if ((s.v[1031] != 0.0) && (s.v[1057] != 0.0)) {
            s.store_scalar(27, 0.0001);
        }

        if (s.v[1031] != 0.0) {
            s.store_scale(633, 27, 1.0 / (s.v[394]));
        }

        if (s.v[1031] != 0.0) {
            s.copy_ad(636, 634);
        }

        s.v[1058] = if (p.p313 == 1.0) { 1.0 } else { 0.0 };

        if (s.v[1058] != 0.0) {
            s.store_scalar(1069, (p.p40 / 1e-6));
        }

        if (s.v[1058] != 0.0) {
            s.store_scalar(1062, p.p316);
        }

        if (s.v[1058] != 0.0) {
            s.store_scalar(1063, p.p318);
        }

        if (s.v[1058] != 0.0) {
            s.store_scalar(1064, p.p323);
        }

        if (s.v[1058] != 0.0) {
            s.store_scalar(1065, (if (p.p314 > 0.0) { (p.p314 * p.p309) } else { 0.0 }));
        }

        if (s.v[1058] != 0.0) {
            s.store_scalar(1068, p.p310);
        }

        if (s.v[1058] != 0.0) {
            s.store_ad(1066, &A::scale(A::voltage(ctx, &nodes, Some(0), Some(11)), p.p33));
        }

        if (s.v[1058] != 0.0) {
            s.store_scalar(1075, ((((p.p322 * p.p322) + (p.p38 * p.p38))) as f64).sqrt());
        }

        if (s.v[1058] != 0.0) {
            s.store_scalar(1077, (s.v[124] * p.p5));
        }

        if (s.v[1058] != 0.0) {
            s.store_scale(1062, 1062, 0.0001);
        }

        if (s.v[1058] != 0.0) {
            s.store_scale(1063, 1063, 0.01);
        }

        if (s.v[1058] != 0.0) {
            s.store_scale(1067, 374, 1.0 / (s.v[445]));
        }

        if (s.v[1058] != 0.0) {
            s.store_powf(279, 1067, p.p320);
        }

        if (s.v[1058] != 0.0) {
            s.store_div(1070, 1062, 279);
        }

        if (s.v[1058] != 0.0) {
            s.store_sub_ad(278, A::add(A::offset(A::scale(s.ad_value(1067), 0.4), 1.8), A::mul(A::scale(s.ad_value(1067), 0.1), s.ad_value(1067))), A::scale(A::sub_from_scalar(1.0, s.ad_value(1067)), p.p321));
        }

        if (s.v[1058] != 0.0) {
            s.store_div(1071, 1063, 278);
        }

        if (s.v[1058] != 0.0) {
            s.store_add_ad_rhs(1064, 1064, A::scale(A::offset(s.ad_value(374), (-s.v[445])), p.p325));
        }

        if (s.v[1058] != 0.0) {
            s.store_scalar(1059, (1.0 + (p.p330 / ((s.v[375]) as f64).powf(p.p331))));
        }

        if (s.v[1058] != 0.0) {
            s.store_scalar(1061, (1.0 + (p.p328 / ((s.v[375]) as f64).powf(p.p329))));
        }

        if (s.v[1058] != 0.0) {
            s.store_scalar(1060, (1.0 + (p.p326 / ((s.v[376]) as f64).powf(p.p327))));
        }

        if (s.v[1058] != 0.0) {
            s.store_mul(1070, 1070, 1059);
        }

        if (s.v[1058] != 0.0) {
            s.store_offset_ad(1071, A::mul(A::mul(s.ad_value(1071), s.ad_value(1060)), s.ad_value(1061)), 1e-50);
        }

        if (s.v[1058] != 0.0) {
            s.store_div(1072, 1066, 1068);
        }

        if (s.v[1058] != 0.0) {
            s.store_mul(1073, 1070, 1072);
        }

        s.v[1078] = if (s.v[1066] >= 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1058] != 0.0) && (s.v[1078] != 0.0)) {
            s.store_div(279, 1073, 1071);
        }

        if ((s.v[1058] != 0.0) && (!(s.v[1078] != 0.0))) {
            s.store_div_ad_lhs(279, A::neg(s.ad_value(1073)), 1071);
        }

        s.v[1079] = if (((1.0 - (10.0 * 2.220446049250313e-16)) <= s.v[1064]) && (s.v[1064] <= (1.0 + (10.0 * 2.220446049250313e-16)))) { 1.0 } else { 0.0 };

        if ((s.v[1058] != 0.0) && (s.v[1079] != 0.0)) {
            s.store_scalar(281, 1.0);
        }

        s.v[1080] = if (((2.0 - (10.0 * 2.220446049250313e-16)) <= s.v[1064]) && (s.v[1064] <= (2.0 + (10.0 * 2.220446049250313e-16)))) { 1.0 } else { 0.0 };

        if (((s.v[1058] != 0.0) && (!(s.v[1079] != 0.0))) && (s.v[1080] != 0.0)) {
            s.copy_ad(281, 279);
        }

        if (((s.v[1058] != 0.0) && (!(s.v[1079] != 0.0))) && (!(s.v[1080] != 0.0))) {
            s.store_ad(281, &A::pow(s.ad_value(279), A::offset(s.ad_value(1064), (-1.0))));
        }

        if (s.v[1058] != 0.0) {
            s.store_mul(280, 279, 281);
        }

        if (s.v[1058] != 0.0) {
            s.store_offset(282, 280, 1.0);
        }

        s.v[1081] = if (((1.0 - (10.0 * 2.220446049250313e-16)) <= s.v[1064]) && (s.v[1064] <= (1.0 + (10.0 * 2.220446049250313e-16)))) { 1.0 } else { 0.0 };

        if ((s.v[1058] != 0.0) && (s.v[1081] != 0.0)) {
            s.store_div_from_scalar(283, 1.0, 282);
        }

        s.v[1082] = if (((2.0 - (10.0 * 2.220446049250313e-16)) <= s.v[1064]) && (s.v[1064] <= (2.0 + (10.0 * 2.220446049250313e-16)))) { 1.0 } else { 0.0 };

        if (((s.v[1058] != 0.0) && (!(s.v[1081] != 0.0))) && (s.v[1082] != 0.0)) {
            s.store_div_from_scalar_ad(283, 1.0, A::sqrt(s.ad_value(282)));
        }

        if (((s.v[1058] != 0.0) && (!(s.v[1081] != 0.0))) && (!(s.v[1082] != 0.0))) {
            s.store_ad(284, &A::pow(s.ad_value(282), A::offset(A::div_from_scalar((-1.0), s.ad_value(1064)), (-1.0))));
        }

        if (((s.v[1058] != 0.0) && (!(s.v[1081] != 0.0))) && (!(s.v[1082] != 0.0))) {
            s.store_mul(283, 282, 284);
        }

        if (s.v[1058] != 0.0) {
            s.store_mul(1074, 1070, 283);
        }

        if (s.v[1058] != 0.0) {
            s.store_div_from_scalar(279, 1.6021918e-19, 1068);
        }

        if (s.v[1058] != 0.0) {
            s.store_mul_ad_lhs(1076, A::mul(A::mul(s.ad_value(279), s.ad_value(1075)), s.ad_value(1074)), 1069);
        }

        s.v[1083] = if (s.v[1076] <= 0.0) { 1.0 } else { 0.0 };

        if ((s.v[1058] != 0.0) && (s.v[1083] != 0.0)) {
            s.store_scalar(1076, 1e-50);
        }

        if (s.v[1058] != 0.0) {
            s.store_div_from_scalar(27, 1.0, 1076);
        }

        if (s.v[1058] != 0.0) {
            s.store_div(27, 27, 1077);
        }

        if (s.v[1058] != 0.0) {
            s.store_add(27, 27, 1065);
        }

        if (s.v[1058] != 0.0) {
            s.store_ad(634, &{
                if ((s.v[27] > 0.0001) && (p.p23 != 0.0)) {
                    A::div_from_scalar(s.v[394], s.ad_value(27))
                } else {
                    A::constant(0.0)
                }
            });
        }

        s.v[1084] = if (s.v[27] < 0.0001) { 1.0 } else { 0.0 };

        if ((s.v[1058] != 0.0) && (s.v[1084] != 0.0)) {
            s.store_scalar(27, 0.0001);
        }

        if (s.v[1058] != 0.0) {
            s.store_scale(632, 27, 1.0 / (s.v[394]));
        }

        if (s.v[1058] != 0.0) {
            s.copy_ad(635, 634);
        }

        s.v[1085] = if (s.v[221] < 1e-18) { 1.0 } else { 0.0 };

        if ((s.v[38] != 0.0) && (s.v[1085] != 0.0)) {
            s.store_scalar(221, 1e-18);
        }

        s.v[1086] = if (s.v[222] < 1e-18) { 1.0 } else { 0.0 };

        if ((s.v[38] != 0.0) && (s.v[1086] != 0.0)) {
            s.store_scalar(222, 1e-18);
        }

        if (s.v[38] != 0.0) {
            s.store_div_ad_lhs(549, A::sub(s.ad_value(551), s.ad_value(555)), 221);
        }

        if (s.v[38] != 0.0) {
            s.store_div_ad_lhs(550, A::sub(s.ad_value(548), s.ad_value(556)), 222);
        }

        if (s.v[38] != 0.0) {
            s.store_sub_ad_lhs(554, A::neg(s.ad_value(551)), 548);
        }

        if (s.v[38] != 0.0) {
            s.store_scale(552, 551, s.v[385]);
        }

        if (s.v[38] != 0.0) {
            s.store_scale(553, 551, (1.0 - s.v[385]));
        }

        if (!(s.v[38] != 0.0)) {
            s.store_scalar(549, 0.0);
        }

        if (!(s.v[38] != 0.0)) {
            s.store_scalar(550, 0.0);
        }

        if (!(s.v[38] != 0.0)) {
            s.store_scalar(552, 0.0);
        }

        if (!(s.v[38] != 0.0)) {
            s.store_scalar(553, 0.0);
        }

        if (!(s.v[38] != 0.0)) {
            s.store_scalar(554, 0.0);
        }

        if (!(s.v[38] != 0.0)) {
            s.store_scalar(548, 0.0);
        }

        s.copy_ad(26, 632);

        s.copy_ad(27, 633);

        s.v[1087] = if (s.v[575] == 1.0) { 1.0 } else { 0.0 };

        if (s.v[1087] != 0.0) {
            s.copy_ad(94, 0);
        }

        if (s.v[1087] != 0.0) {
            s.copy_ad(185, 494);
        }

        if (s.v[1087] != 0.0) {
            s.store_scalar(546, 0.0);
        }

        if (s.v[1087] != 0.0) {
            s.copy_ad(561, 14);
        }

        if (s.v[1087] != 0.0) {
            s.copy_ad(93, 15);
        }

        if (s.v[1087] != 0.0) {
            s.store_neg_ad(492, A::add(A::add(s.ad_value(14), s.ad_value(15)), s.ad_value(16)));
        }

        if (s.v[1087] != 0.0) {
            s.copy_ad(90, 492);
        }

        if (!(s.v[1087] != 0.0)) {
            s.store_neg(94, 0);
        }

        if (!(s.v[1087] != 0.0)) {
            s.copy_ad(546, 494);
        }

        if (!(s.v[1087] != 0.0)) {
            s.store_scalar(185, 0.0);
        }

        if (!(s.v[1087] != 0.0)) {
            s.copy_ad(561, 14);
        }

        if (!(s.v[1087] != 0.0)) {
            s.copy_ad(93, 16);
        }

        if (!(s.v[1087] != 0.0)) {
            s.store_neg_ad(492, A::add(A::add(s.ad_value(14), s.ad_value(15)), s.ad_value(16)));
        }

        if (!(s.v[1087] != 0.0)) {
            s.copy_ad(90, 492);
        }

        if (!(s.v[1087] != 0.0)) {
            s.copy_ad(16, 15);
        }

        if (!(s.v[1087] != 0.0)) {
            s.copy_ad(15, 93);
        }

        if ((!(s.v[1087] != 0.0)) && (s.v[38] != 0.0)) {
            s.copy_ad(279, 552);
        }

        if ((!(s.v[1087] != 0.0)) && (s.v[38] != 0.0)) {
            s.copy_ad(552, 553);
        }

        if ((!(s.v[1087] != 0.0)) && (s.v[38] != 0.0)) {
            s.copy_ad(553, 279);
        }

        s.v[1088] = if ((p.p28 != 0.0) && (p.p237 > 0.0)) { 1.0 } else { 0.0 };

        if (s.v[1088] != 0.0) {
            s.store_mul(547, 0, 51);
        }

        if (s.v[1088] != 0.0) {
            s.store_scalar(516, s.v[468]);
        }

        if (s.v[1088] != 0.0) {
            s.store_scalar(557, (1.0 / s.v[467]));
        }

        if (!(s.v[1088] != 0.0)) {
            s.store_scalar(547, 0.0);
        }

        if (!(s.v[1088] != 0.0)) {
            s.store_scalar(516, 0.0);
        }

        if (!(s.v[1088] != 0.0)) {
            s.store_scalar(557, 0.0);
        }

        s.copy_ad(251, 4);

        s.copy_ad(250, 5);

        s.copy_ad(254, 6);

        s.copy_ad(257, 2);

        s.copy_ad(258, 3);

        s.copy_ad(0, 94);

        s.store_scalar(18, A::ddx_projection(&s.ad_value(14), Some(11), None));

        s.store_scale(18, 18, p.p33);

        s.store_scalar(19, A::ddx_projection(&s.ad_value(14), Some(12), None));

        s.store_scale(19, 19, p.p33);

        s.v[1094] = if ((p.p28 != 0.0) && (p.p237 > 0.0)) { 1.0 } else { 0.0 };

        s.v[1095] = if (((p.p27 != 0.0) && (p.p15 != 0.0)) && (p.p16 != 0.0)) { 1.0 } else { 0.0 };

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
        s.v[649] = 2.0;

        s.v[650] = 0.1;

        s.v[651] = 0.1;

        s.v[514] = 0.0;

        s.v[574] = 0.0;

        s.v[237] = 1e-12;

        s.v[28] = 500.0;

        s.v[29] = 200.0;

        s.v[32] = 0.002;

        s.v[38] = p.p24;

        s.v[46] = 1.0;

        s.v[36] = 1.0;

        s.v[305] = 0.0;

        s.v[306] = 0.0;

        s.v[307] = 0.0;

        s.v[308] = 0.0;

        s.v[309] = 0.0;

        s.v[310] = 0.0;

        s.v[312] = 0.0;

        s.v[314] = 0.0;

        s.v[311] = 0.0;

        s.v[313] = 0.0;

        s.v[207] = 0.0;

        s.v[209] = 0.0;

        s.v[531] = 0.0;

        s.v[528] = 0.0;

        s.v[585] = 0.0;

        s.v[588] = 0.0;

        s.v[523] = 0.0;

        s.v[576] = 0.0;

        s.v[555] = 0.0;

        s.v[556] = 0.0;

        s.v[322] = 0.0;

        s.v[327] = 0.0;

        s.v[329] = 0.0;

        s.v[330] = 0.0;

        s.v[331] = 0.0;

        s.v[334] = 0.0;

        s.v[336] = 0.0;

        s.v[337] = 0.0;

        s.v[345] = 0.0;

        s.v[383] = 0.0;

        s.v[385] = 0.5;

        s.v[441] = 0.0;

        s.v[442] = 0.0;

        s.v[558] = 0.0;

        s.v[405] = 0.0;

        s.v[406] = 0.0;

        s.v[397] = 0.0;

        s.v[398] = 0.0;

        s.v[414] = 0.0;

        s.v[34] = 0.0;

        s.v[35] = 0.0;

        s.v[292] = 0.0;

        s.v[16] = 0.0;

        s.v[60] = 0.0;

        s.v[58] = 0.0;

        s.v[74] = 1.0;

        s.v[85] = 0.0;

        s.v[91] = 0.0;

        s.v[93] = 0.0;

        s.v[94] = 0.0;

        s.v[151] = 0.0;

        s.v[158] = 0.0;

        s.v[159] = 0.0;

        s.v[160] = 0.0;

        s.v[185] = 0.0;

        s.v[189] = 1.0;

        s.v[193] = 0.0;

        s.v[196] = 0.0;

        s.v[197] = 0.0;

        s.v[221] = 0.0;

        s.v[222] = 0.0;

        s.v[146] = 0.0;

        s.v[260] = 0.0;

        s.v[89] = 0.0;

        s.v[230] = 0.0;

        s.v[231] = 0.0;

        s.v[233] = 0.0;

        s.v[234] = 0.0;

        s.v[235] = 0.0;

        s.v[236] = 0.0;

        s.v[55] = 0.0;

        s.v[77] = 0.0;

        s.v[339] = 0.0;

        s.v[388] = 0.0;

        s.v[316] = 0.0;

        s.v[517] = if self.param_given[172] { 1.0 } else { 0.0 };

        s.v[518] = if self.param_given[173] { 1.0 } else { 0.0 };

        s.v[519] = if self.param_given[174] { 1.0 } else { 0.0 };

        s.v[463] = if self.param_given[9] { 1.0 } else { 0.0 };

        s.v[394] = 1.0;

        s.v[446] = (if (if self.param_given[177] { 1.0 } else { 0.0 } != 0.0) { p.p177 } else { (5000000000.0 / (p.p227 * p.p230)) });

        s.v[660] = if ((s.v[446] < (2.0 + 0.1)) && (0.1 >= 0.0)) { 1.0 } else { 0.0 };

        if (s.v[660] != 0.0) {
            s.store_scalar(638, ((2.0 + 0.1) - s.v[446]));
        }

        if (s.v[660] != 0.0) {
            s.store_square(642, 638);
        }

        if (s.v[660] != 0.0) {
            s.store_scalar(643, (0.1 * 0.1));
        }

        if (s.v[660] != 0.0) {
            s.store_scalar(644, 1.0);
        }

        if (s.v[660] != 0.0) {
            s.store_scalar(645, 1.0);
        }

        if (s.v[660] != 0.0) {
            s.store_scalar(647, 0.0);
        }

        if (s.v[660] != 0.0) {
            s.store_scalar(648, 0.0);
        }

        if (s.v[660] != 0.0) {
            s.store_scalar(220, 0.0);
        }

        if (s.v[660] != 0.0) {
            s.store_scalar(646, 0.0);
        }

        if (s.v[660] != 0.0) {
            s.store_mul(644, 644, 642);
        }

        if (s.v[660] != 0.0) {
            s.store_mul(645, 645, 643);
        }

        if (s.v[660] != 0.0) {
            s.store_mul(644, 644, 642);
        }

        if (s.v[660] != 0.0) {
            s.store_mul(645, 645, 643);
        }

        if (s.v[660] != 0.0) {
            s.store_add(220, 644, 645);
        }

        if (s.v[660] != 0.0) {
            s.copy_ad(646, 220);
        }

        s.v[661] = if ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0)) { 1.0 } else { 0.0 };

        s.v[662] = if (2.0 == 1.0) { 1.0 } else { 0.0 };

        if (((s.v[660] != 0.0) && (s.v[661] != 0.0)) && (s.v[662] != 0.0)) {
            s.store_scalar(648, 1.0);
        }

        s.v[663] = if (2.0 == 2.0) { 1.0 } else { 0.0 };

        if ((((s.v[660] != 0.0) && (s.v[661] != 0.0)) && (!(s.v[662] != 0.0))) && (s.v[663] != 0.0)) {
            s.store_scalar(648, 2.0);
        }

        s.v[664] = if (2.0 == 4.0) { 1.0 } else { 0.0 };

        if (((((s.v[660] != 0.0) && (s.v[661] != 0.0)) && (!(s.v[662] != 0.0))) && (!(s.v[663] != 0.0))) && (s.v[664] != 0.0)) {
            s.store_scalar(648, 3.0);
        }

        s.v[665] = if (2.0 == 8.0) { 1.0 } else { 0.0 };

        if ((((((s.v[660] != 0.0) && (s.v[661] != 0.0)) && (!(s.v[662] != 0.0))) && (!(s.v[663] != 0.0))) && (!(s.v[664] != 0.0))) && (s.v[665] != 0.0)) {
            s.store_scalar(648, 4.0);
        }

        if ((s.v[660] != 0.0) && (s.v[661] != 0.0)) {
            s.store_scalar(647, 0.0);
        }

        let mut assign1360_loop_guard: usize = 0;
        while {
            let assign1360_cond_e892: f64 = if (((s.v[660] != 0.0) && (s.v[661] != 0.0)) && (s.v[647] < s.v[648])) { 1.0 } else { 0.0 };
            assign1360_cond_e892 != 0.0
        } {
            assign1360_loop_guard += 1;
            assert!(assign1360_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((s.v[660] != 0.0) && (s.v[661] != 0.0)) {
                s.store_sqrt(646, 646);
            }
            if ((s.v[660] != 0.0) && (s.v[661] != 0.0)) {
                s.store_offset(647, 647, 1.0);
            }
        }

        if ((s.v[660] != 0.0) && (!(s.v[661] != 0.0))) {
            s.store_powf(646, 646, (1.0 / (2.0 * 2.0)));
        }

        if (s.v[660] != 0.0) {
            s.store_div_from_scalar_ad(646, 1.0, A::offset(s.ad_value(646), 1e-50));
        }

        if (s.v[660] != 0.0) {
            s.store_mul_ad_lhs(637, A::scale(s.ad_value(638), 0.1), 646);
        }

        if (s.v[660] != 0.0) {
            s.store_div_ad(278, A::mul(A::scale(s.ad_value(645), 0.1), s.ad_value(646)), A::offset(s.ad_value(220), 1e-50));
        }

        if (s.v[660] != 0.0) {
            s.store_sub_from_scalar(446, (2.0 + 0.1), 637);
        }

        if (s.v[660] != 0.0) {
        }

        if (!(s.v[660] != 0.0)) {
        }

        if (!(s.v[660] != 0.0)) {
            s.store_scalar(278, 1.0);
        }

        s.v[613] = (p.p34 * 0.01);

        s.v[614] = (p.p59 / 1e-6);

        s.v[615] = (p.p101 * 0.01);

        s.v[616] = (p.p192 / 1e-6);

        s.v[617] = (p.p219 * 0.01);

        s.v[619] = (p.p220 / 0.0001);

        s.v[620] = (p.p230 / 1e-6);

        s.v[621] = (p.p231 / 1e-6);

        s.v[622] = (p.p237 * 0.01);

        s.v[623] = (p.p238 / 0.01);

        s.v[624] = (p.p40 / 1e-6);

        s.v[625] = (p.p236 / 1e-6);

        s.v[627] = (p.p197 / 0.01);

        s.v[630] = (p.p306 / 1e-6);

        s.v[631] = (p.p307 / 1e-6);

        s.v[626] = (p.p189 * 10000.0);

        s.v[452] = (p.p147 / 1e-6);

        s.v[628] = (p.p196 / 10.0);

        s.v[445] = (p.p222 + 273.15);

        s.v[447] = (p.p9 + 273.15);

        s.v[509] = p.p41;

        s.v[510] = p.p42;

        s.v[277] = p.p0;

        s.v[456] = (p.p1 / p.p5);

        s.v[375] = (s.v[277] * 1000000.0);

        s.v[376] = (s.v[456] * 1000000.0);

        s.v[377] = (s.v[376] * s.v[375]);

        s.v[279] = (p.p62 / ((s.v[377]) as f64).powf(p.p63));

        s.v[133] = (s.v[277] + s.v[279]);

        s.v[134] = (s.v[456] + s.v[279]);

        s.v[482] = (p.p64 / ((s.v[377]) as f64).powf(p.p65));

        s.v[279] = (1.0 + (p.p148 / (((s.v[133] * 1000000.0)) as f64).powf(p.p149)));

        s.v[280] = (1.0 + (p.p150 / (((s.v[134] * 1000000.0)) as f64).powf(p.p151)));

        s.v[452] = ((s.v[452] * s.v[279]) * s.v[280]);

        s.v[279] = (1.0 + (p.p154 / (((s.v[133] * 1000000.0)) as f64).powf(p.p155)));

        s.v[280] = (1.0 + (p.p156 / (((s.v[134] * 1000000.0)) as f64).powf(p.p157)));

        s.v[453] = ((p.p152 * s.v[279]) * s.v[280]);

        s.v[511] = ((2.0 * s.v[453]) * p.p153);

        s.v[124] = ((s.v[456] - (2.0 * s.v[509])) - s.v[511]);

        s.v[512] = ((s.v[456] - (2.0 * s.v[510])) - s.v[511]);

        s.v[466] = (s.v[124] * p.p5);

        s.v[513] = (s.v[512] * p.p5);

        s.v[467] = (s.v[622] / (s.v[394] * s.v[466]));

        s.v[468] = (s.v[623] * (s.v[394] * s.v[513]));

        s.v[278] = (s.v[630] * ((p.p11 + (p.p304 * p.p12)) + (p.p305 * p.p13)));

        s.v[620] = (s.v[620] + s.v[278]);

        s.v[638] = ((s.v[620] - (1000000000000000.0 / 1e-6)) - (0.01 / 1e-6));

        s.v[639] = ((4.0 * (1000000000000000.0 / 1e-6)) * (0.01 / 1e-6));

        if !(s.v[639] > 0.0) {
            s.store_scalar(639, (-s.v[639]));
        }

        s.store_sqrt_ad(639, A::offset(s.ad_value(639), (s.v[638] * s.v[638])));

        s.store_offset_ad(620, A::scale(A::offset(s.ad_value(639), s.v[638]), 0.5), (1000000000000000.0 / 1e-6));

        s.v[278] = (s.v[631] * ((p.p11 + (p.p304 * p.p12)) + (p.p305 * p.p13)));

        s.v[614] = (s.v[614] + s.v[278]);

        s.v[638] = ((s.v[614] - (1000000000000000.0 / 1e-6)) - (0.01 / 1e-6));

        s.v[639] = ((4.0 * (1000000000000000.0 / 1e-6)) * (0.01 / 1e-6));

        if !(s.v[639] > 0.0) {
            s.store_scalar(639, (-s.v[639]));
        }

        s.store_sqrt_ad(639, A::offset(s.ad_value(639), (s.v[638] * s.v[638])));

        s.store_offset_ad(614, A::scale(A::offset(s.ad_value(639), s.v[638]), 0.5), (1000000000000000.0 / 1e-6));

        s.v[448] = ((p.p86 * ((s.v[375]) as f64).powf(p.p88)) * (1.0 + (p.p90 / ((s.v[375]) as f64).powf(p.p91))));

        s.v[449] = ((p.p87 * ((s.v[375]) as f64).powf(p.p89)) * (1.0 + (p.p92 / ((s.v[375]) as f64).powf(p.p93))));

        s.v[450] = ((p.p289 * ((s.v[375]) as f64).powf(p.p291)) * (1.0 + (p.p293 / ((s.v[375]) as f64).powf(p.p294))));

        s.v[451] = ((p.p290 * ((s.v[375]) as f64).powf(p.p292)) * (1.0 + (p.p295 / ((s.v[375]) as f64).powf(p.p296))));

        s.v[470] = ((p.p106 * (1.0 + (p.p107 / ((s.v[375]) as f64).powf(p.p110)))) * (1.0 + (p.p108 / ((s.v[376]) as f64).powf(p.p109))));

        s.v[594] = ((p.p283 * (1.0 + (p.p285 / ((s.v[375]) as f64).powf(p.p286)))) * (1.0 + (p.p287 / ((s.v[376]) as f64).powf(p.p288))));

        s.v[279] = (s.v[621] * (1.0 + (p.p232 / ((s.v[375]) as f64).powf(p.p233))));

        s.v[638] = ((s.v[279] - s.v[625]) - (s.v[621] * 0.001));

        s.v[639] = ((4.0 * s.v[625]) * (s.v[621] * 0.001));

        if !(s.v[639] > 0.0) {
            s.store_scalar(639, (-s.v[639]));
        }

        s.store_sqrt_ad(639, A::offset(s.ad_value(639), (s.v[638] * s.v[638])));

        s.store_offset_ad(462, A::scale(A::offset(s.ad_value(639), s.v[638]), 0.5), s.v[625]);

        if (p.p32 != 0.0) {
            s.store_scale(279, 462, (1.0 + (p.p234 / ((s.v[376]) as f64).powf(p.p235))));
        }

        if (p.p32 != 0.0) {
            s.store_offset(638, 279, (((-s.v[625])) + ((-(s.v[621] * 0.001)))));
        }

        if (p.p32 != 0.0) {
            s.store_scalar(639, ((4.0 * s.v[625]) * (s.v[621] * 0.001)));
        }

        if (p.p32 != 0.0) {
            s.store_ad(639, &{
                if (s.v[639] > 0.0) {
                    s.ad_value(639)
                } else {
                    A::neg(s.ad_value(639))
                }
            });
        }

        if (p.p32 != 0.0) {
            s.store_sqrt_ad(639, A::add(A::square(s.ad_value(638)), s.ad_value(639)));
        }

        if (p.p32 != 0.0) {
            s.store_offset_ad(462, A::scale(A::add(s.ad_value(638), s.ad_value(639)), 0.5), s.v[625]);
        }

        s.store_scale(460, 614, (1.0 + (p.p60 / ((s.v[376]) as f64).powf(p.p61))));

        s.copy_ad(461, 460);

        s.v[279] = ((1.0 / (p.p43 + (0.5 * p.p0))) + (1.0 / (p.p44 + (0.5 * p.p0))));

        s.v[459] = (2.0 / s.v[279]);

        s.v[666] = if (((p.p6 > 0.0) && (p.p7 > 0.0)) && ((p.p5 == 1.0) || ((p.p5 > 1.0) && (p.p8 > 0.0)))) { 1.0 } else { 0.0 };

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
        let nv4 = ctx.node_voltage(nodes[4]);
        if (s.v[666] != 0.0) {
            s.store_scalar(279, 0.0);
        }

        if (s.v[666] != 0.0) {
            s.store_scalar(514, 0.0);
        }

        let mut assign2290_loop_guard: usize = 0;
        while {
            let assign2290_cond_e1503: f64 = if ((s.v[666] != 0.0) && (s.v[514] < p.p5)) { 1.0 } else { 0.0 };
            assign2290_cond_e1503 != 0.0
        } {
            assign2290_loop_guard += 1;
            assert!(assign2290_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (s.v[666] != 0.0) {
                s.store_add_ad(279, A::add(s.ad_value(279), A::div_from_scalar(1.0, A::offset(A::scale(s.ad_value(514), (p.p8 + p.p0)), (p.p6 + (0.5 * p.p0))))), A::div_from_scalar(1.0, A::offset(A::scale(s.ad_value(514), (p.p8 + p.p0)), (p.p7 + (0.5 * p.p0)))));
            }
            if (s.v[666] != 0.0) {
                s.store_offset(514, 514, 1.0);
            }
        }

        if (s.v[666] != 0.0) {
            s.store_div_from_scalar(458, (2.0 * p.p5), 279);
        }

        if (!(s.v[666] != 0.0)) {
            s.store_scalar(458, 0.0);
        }

        s.v[667] = if (s.v[458] > 0.0) { 1.0 } else { 0.0 };

        if (s.v[667] != 0.0) {
            s.store_scalar(279, (1.0 / (1.0 + p.p166)));
        }

        if (s.v[667] != 0.0) {
            s.store_scalar(280, 0.0);
        }

        if (s.v[667] != 0.0) {
            s.store_scalar(281, 0.0);
        }

        if (s.v[667] != 0.0) {
            s.store_div_ad(461, A::mul(s.ad_value(460), A::offset(A::mul(s.ad_value(279), s.ad_value(280)), 1.0)), A::offset(A::mul(s.ad_value(279), s.ad_value(281)), 1.0));
        }

        if (s.v[667] != 0.0) {
            s.store_scalar(279, (1.0 / (1.0 + p.p169)));
        }

        if (s.v[667] != 0.0) {
            s.store_powf_ad(280, A::div_from_scalar(p.p168, s.ad_value(458)), p.p170);
        }

        if (s.v[667] != 0.0) {
            s.store_scalar(281, (((p.p168 / s.v[459])) as f64).powf(p.p170));
        }

        if (s.v[667] != 0.0) {
            s.store_div_ad(620, A::mul(s.ad_value(620), A::offset(A::mul(s.ad_value(279), s.ad_value(280)), 1.0)), A::offset(A::mul(s.ad_value(279), s.ad_value(281)), 1.0));
        }

        if (!(s.v[667] != 0.0)) {
            s.copy_ad(461, 460);
        }

        s.v[280] = (1.0 + (p.p190 / ((s.v[376]) as f64).powf(p.p191)));

        s.store_div_from_scalar(281, s.v[616], 620);

        s.store_offset(638, 281, (((-s.v[280])) + ((-0.01))));

        s.store_scale(639, 281, (4.0 * 0.01));

        if !(s.v[639] > 0.0) {
            s.store_neg(639, 639);
        }

        s.store_sqrt_ad(639, A::add(A::square(s.ad_value(638)), s.ad_value(639)));

        s.store_sub_ad_rhs(279, 281, A::scale(A::add(s.ad_value(638), s.ad_value(639)), 0.5));

        s.store_mul(471, 620, 279);

        s.v[668] = if ((s.v[277] > p.p58) || (p.p58 <= 0.0)) { 1.0 } else { 0.0 };

        if (s.v[668] != 0.0) {
            s.store_scale_ad(457, A::add(A::scale(s.ad_value(471), (s.v[277] - p.p58)), A::scale(s.ad_value(461), p.p58)), 1.0 / (s.v[277]));
        }

        if (!(s.v[668] != 0.0)) {
            s.store_add_ad_rhs(457, 461, A::scale(A::sub(s.ad_value(461), s.ad_value(471)), ((p.p58 - s.v[277]) * 1.0 / (p.p58))));
        }

        s.store_scale(126, 457, 1.6021918e-19);

        s.store_scale(472, 126, 1.034943e-10);

        s.store_scale(473, 472, 2.0);

        s.store_scale(474, 462, (1.6021918e-19 * 1.034943e-10));

        s.v[475] = (p.p239 * ((s.v[375]) as f64).powf((-p.p242)));

        s.v[476] = (p.p243 * ((s.v[375]) as f64).powf((-p.p244)));

        s.v[477] = (p.p246 * (((s.v[375] + p.p248)) as f64).powf((-p.p247)));

        s.v[669] = if ((s.v[277] <= (2.0 * p.p58)) && (p.p58 > 0.0)) { 1.0 } else { 0.0 };

        if (s.v[669] != 0.0) {
            s.store_sub_ad_lhs(560, A::sub(A::scale(s.ad_value(461), 2.0), A::scale(A::sub(s.ad_value(461), s.ad_value(471)), (s.v[277] * 1.0 / (p.p58)))), 471);
        }

        if (s.v[669] != 0.0) {
            s.store_ln_ad(478, A::div(s.ad_value(560), s.ad_value(471)));
        }

        if (!(s.v[669] != 0.0)) {
            s.store_scalar(478, 0.0);
        }

        s.store_scale_ad(129, A::ln(A::scale(s.ad_value(457), 9.615384615384616e-17)), (2.0 / 38.68283));

        s.store_scale_ad(136, A::ln(A::scale(s.ad_value(471), 9.615384615384616e-17)), (2.0 / 38.68283));

        s.v[479] = ((((1.0 + (1.0 / s.v[375]))) as f64).powf(p.p77) * p.p75);

        s.v[279] = (p.p116 * s.v[375]);

        s.v[481] = ((((s.v[279] * p.p115) / (s.v[279] + p.p115)) + p.p117) + 1e-50);

        s.v[483] = (1.0 + (((s.v[375]) as f64).powf(p.p179) * p.p180));

        s.v[670] = if (p.p25 == 1.0) { 1.0 } else { 0.0 };

        if (s.v[670] != 0.0) {
            s.store_scalar(279, (p.p3 + (s.v[124] / (3.0 * p.p2))));
        }

        s.v[485] = (1.0 + (p.p131 / ((s.v[376]) as f64).powf(p.p132)));

        s.v[486] = (p.p125 * (1.0 + (p.p126 / ((s.v[375]) as f64).powf(p.p127))));

        s.v[487] = (s.v[375] / (s.v[375] + p.p124));

        s.v[488] = (p.p118 * (1.0 + (p.p120 / ((s.v[375]) as f64).powf(p.p121))));

        s.v[489] = (p.p119 * (1.0 + (p.p122 / s.v[375])));

        s.v[490] = (((10000.0 * s.v[513]) * p.p46) / ((s.v[375]) as f64).powf(p.p47));

        s.v[559] = (p.p133 * (1.0 + (p.p134 / ((s.v[375]) as f64).powf(p.p135))));

        s.v[491] = (p.p128 * (1.0 + (p.p129 / ((s.v[375]) as f64).powf(p.p130))));

        s.v[279] = ((2.0 * 1.034943e-10) / 1.6021918e-19);

        s.store_sqrt_ad(132, A::div_from_scalar(s.v[279], s.ad_value(457)));

        s.store_ad(540, &A::scale(A::voltage(ctx, &nodes, Some(5), Some(12)), p.p33));

        s.store_ad(541, &A::scale(A::voltage(ctx, &nodes, Some(11), Some(12)), p.p33));

        s.store_ad(542, &A::scale(A::voltage(ctx, &nodes, Some(6), Some(12)), p.p33));

        s.store_ad(543, &A::scale(A::voltage(ctx, &nodes, Some(5), Some(2)), p.p33));

        s.store_ad(544, &A::scale(A::voltage(ctx, &nodes, Some(0), Some(2)), p.p33));

        s.store_ad(545, &A::scale(A::voltage(ctx, &nodes, Some(6), Some(2)), p.p33));

        s.v[672] = if ((p.p28 != 0.0) && (p.p237 > 0.0)) { 1.0 } else { 0.0 };

        if (s.v[672] != 0.0) {
            s.store_ad(11, &{
                if (nv4 > 0.0) {
                    A::voltage(ctx, &nodes, Some(4), None)
                } else {
                    A::constant(0.0)
                }
            });
        }

        if (!(s.v[672] != 0.0)) {
            s.store_scalar(11, 0.0);
        }

        if (s.v[38] != 0.0) {
            s.store_ad(551, &A::scale(A::voltage(ctx, &nodes, Some(8), None), 1e-9));
        }

        if (s.v[38] != 0.0) {
            s.store_ad(548, &A::scale(A::voltage(ctx, &nodes, Some(9), None), 1e-9));
        }

        if (!(s.v[38] != 0.0)) {
            s.store_scalar(551, 0.0);
        }

        if (!(s.v[38] != 0.0)) {
            s.store_scalar(548, 0.0);
        }

        s.v[673] = if (s.v[541] >= 0.0) { 1.0 } else { 0.0 };

        if (s.v[673] != 0.0) {
            s.store_scalar(575, 1.0);
        }

        if (s.v[673] != 0.0) {
            s.store_scalar(412, 1.0);
        }

        if (s.v[673] != 0.0) {
            s.store_scalar(413, 0.0);
        }

        if (s.v[673] != 0.0) {
            s.copy_ad(49, 540);
        }

        if (s.v[673] != 0.0) {
            s.copy_ad(48, 541);
        }

        if (s.v[673] != 0.0) {
            s.copy_ad(47, 542);
        }

        if (s.v[673] != 0.0) {
            s.copy_ad(42, 543);
        }

        if (s.v[673] != 0.0) {
            s.copy_ad(41, 544);
        }

        if (s.v[673] != 0.0) {
            s.copy_ad(40, 545);
        }

        if (!(s.v[673] != 0.0)) {
            s.store_scalar(575, (-1.0));
        }

        if (!(s.v[673] != 0.0)) {
            s.store_scalar(412, 0.0);
        }

        if (!(s.v[673] != 0.0)) {
            s.store_scalar(413, 1.0);
        }

        if (!(s.v[673] != 0.0)) {
            s.store_sub(49, 540, 541);
        }

        if (!(s.v[673] != 0.0)) {
            s.store_neg(48, 541);
        }

        if (!(s.v[673] != 0.0)) {
            s.store_sub(47, 542, 541);
        }

        if (!(s.v[673] != 0.0)) {
            s.store_sub(42, 543, 544);
        }

        if (!(s.v[673] != 0.0)) {
            s.store_neg(41, 544);
        }

        if (!(s.v[673] != 0.0)) {
            s.store_sub(40, 545, 544);
        }

        s.v[374] = ctx.temperature();

        if (s.v[463] != 0.0) {
            s.store_scalar(374, s.v[447]);
        }

        s.store_add_ad_lhs(374, A::offset(s.ad_value(374), p.p10), 11);

        s.v[465] = (p.p37 - (s.v[445] * (9.025e-5 + (s.v[445] * 1e-7))));

        s.store_offset_ad(279, A::square(s.ad_value(374)), (-(s.v[445] * s.v[445])));

        s.store_sub_ad(137, A::sub_from_scalar(s.v[465], A::scale(A::offset(s.ad_value(374), (-s.v[445])), p.p35)), A::scale(s.ad_value(279), p.p36));

        s.store_div_from_scalar_ad(120, 1.6021918e-19, A::scale(s.ad_value(374), 1.3806226e-23));

        s.store_square(121, 120);

        s.store_div_from_scalar(122, 1.0, 120);

        s.v[464] = (1.6021918e-19 / (1.3806226e-23 * s.v[445]));

        s.v[676] = (((p.p249 * (1.0 + (p.p95 / ((s.v[376]) as f64).powf(p.p96)))) * (1.0 + (p.p97 / ((s.v[375]) as f64).powf(p.p98)))) * (1.0 + (p.p99 / ((s.v[377]) as f64).powf(p.p100))));

        s.v[677] = (((p.p276 * (1.0 + (p.p277 / ((s.v[376]) as f64).powf(p.p278)))) * (1.0 + (p.p281 / ((s.v[375]) as f64).powf(p.p282)))) * (1.0 + (p.p279 / ((s.v[377]) as f64).powf(p.p280))));

        s.v[681] = if (s.v[458] > 0.0) { 1.0 } else { 0.0 };

        if (s.v[681] != 0.0) {
            s.store_scalar(678, (1.0 / (1.0 + p.p163)));
        }

        if (s.v[681] != 0.0) {
            s.store_powf_ad(679, A::div_from_scalar(p.p162, s.ad_value(458)), p.p164);
        }

        if (s.v[681] != 0.0) {
            s.store_scalar(680, (((p.p162 / s.v[459])) as f64).powf(p.p164));
        }

        if (s.v[681] != 0.0) {
            s.store_div_ad(676, A::scale(A::offset(A::mul(s.ad_value(678), s.ad_value(679)), 1.0), s.v[676]), A::offset(A::mul(s.ad_value(678), s.ad_value(680)), 1.0));
        }

        if (s.v[681] != 0.0) {
            s.store_div_ad(677, A::scale(A::offset(A::mul(s.ad_value(678), s.ad_value(679)), 1.0), s.v[677]), A::offset(A::mul(s.ad_value(678), s.ad_value(680)), 1.0));
        }

        s.v[678] = (1.0 + (p.p112 / ((s.v[375]) as f64).powf(p.p113)));

        s.store_offset_ad(378, A::mul(A::scale(A::offset(A::scale(s.ad_value(374), 1.0 / (s.v[445])), (-1.0)), p.p253), A::offset(A::scale(s.ad_value(374), 1.0 / (s.v[445])), (-1.0))), (p.p111 * s.v[678]));

        s.store_ad(678, &A::pow(A::scale(s.ad_value(374), 1.0 / (s.v[445])), s.ad_value(378)));

        s.store_div(469, 678, 676);

        s.store_div(595, 678, 677);

        s.store_mul(380, 478, 122);

        s.v[279] = ((((1.0 + (p.p181 / ((s.v[375]) as f64).powf(p.p182))) * (1.0 + (p.p185 / ((s.v[375]) as f64).powf(p.p186)))) * (1.0 + (p.p187 / ((s.v[376]) as f64).powf(p.p188)))) * (1.0 + (p.p183 / ((s.v[377]) as f64).powf(p.p184))));

        s.v[639] = ((((s.v[279] * s.v[279]) + ((4.0 * 0.001) * 0.001))) as f64).sqrt();

        s.v[280] = (0.5 * (1.0 + (s.v[279] / s.v[639])));

        s.v[480] = ((0.5 * (s.v[279] + s.v[639])) + (1e-10 * 0.001));

        s.v[682] = if (s.v[480] < 0.0) { 1.0 } else { 0.0 };

        if (s.v[682] != 0.0) {
            s.store_scalar(480, 0.0);
        }

        if (s.v[682] != 0.0) {
            s.store_scalar(280, 0.0);
        }

        s.store_scale(279, 374, 1.0 / (s.v[445]));

        s.v[280] = (1.0 + (p.p102 / ((s.v[375]) as f64).powf(p.p103)));

        s.store_scale_ad(162, A::div(A::scale(s.ad_value(480), s.v[613]), A::sub(A::add(A::offset(A::scale(s.ad_value(279), (0.4 * 0.01)), (1.8 * 0.01)), A::scale(A::mul(A::scale(s.ad_value(279), 0.1), s.ad_value(279)), 0.01)), A::scale(A::sub_from_scalar(1.0, s.ad_value(279)), (s.v[615] * s.v[280])))), 0.01);

        s.store_sqrt(245, 137);

        s.store_mul(246, 137, 245);

        s.store_mul_ad(127, A::scale(A::powf(A::scale(s.ad_value(374), 1.0 / (s.v[445])), 1.5), 1.04e16), A::exp(A::offset(A::mul(A::scale(A::neg(s.ad_value(137)), 0.5), s.ad_value(120)), ((s.v[465] / 2.0) * s.v[464]))));

        s.v[117] = (((((2.0 * 1.6021918e-19) * s.v[452]) * 1.034943e-10)) as f64).sqrt();

        s.v[118] = (1.0 / (s.v[452] * s.v[452]));

        s.store_scaled_sqrt(100, 122, s.v[117]);

        s.store_square(119, 100);

        s.store_scale_ad(101, A::square(s.ad_value(127)), s.v[118]);

        s.v[279] = ((p.p38 / (p.p251 + p.p252)) * p.p0);

        s.v[281] = ((((p.p38 * 0.001) + ((10.0 * 2.220446049250313e-16) / 100.0))) as f64).abs();

        s.v[683] = if (p.p38 > 0.0) { 1.0 } else { 0.0 };

        if (s.v[683] != 0.0) {
            s.store_scalar(638, ((p.p38 - s.v[279]) - s.v[281]));
        }

        if (s.v[683] != 0.0) {
            s.store_scalar(639, ((4.0 * p.p38) * s.v[281]));
        }

        if (s.v[683] != 0.0) {
            s.store_ad(639, &{
                if (s.v[639] > 0.0) {
                    s.ad_value(639)
                } else {
                    A::neg(s.ad_value(639))
                }
            });
        }

        if (s.v[683] != 0.0) {
            s.store_sqrt_ad(639, A::add(A::square(s.ad_value(638)), s.ad_value(639)));
        }

        if (s.v[683] != 0.0) {
            s.store_scale_ad(279, A::offset(A::div(s.ad_value(638), s.ad_value(639)), 1.0), 0.5);
        }

        if (s.v[683] != 0.0) {
            s.store_sub_from_scalar_ad(280, p.p38, A::scale(A::add(s.ad_value(638), s.ad_value(639)), 0.5));
        }

        if (!(s.v[683] != 0.0)) {
            s.store_offset(638, 279, (((-p.p38)) + ((-s.v[281]))));
        }

        if (!(s.v[683] != 0.0)) {
            s.store_scalar(639, ((4.0 * p.p38) * s.v[281]));
        }

        if (!(s.v[683] != 0.0)) {
            s.store_ad(639, &{
                if (s.v[639] > 0.0) {
                    s.ad_value(639)
                } else {
                    A::neg(s.ad_value(639))
                }
            });
        }

        if (!(s.v[683] != 0.0)) {
            s.store_sqrt_ad(639, A::add(A::square(s.ad_value(638)), s.ad_value(639)));
        }

        if (!(s.v[683] != 0.0)) {
            s.store_scale_ad(279, A::offset(A::div(s.ad_value(638), s.ad_value(639)), 1.0), 0.5);
        }

        if (!(s.v[683] != 0.0)) {
            s.store_offset_ad(280, A::scale(A::add(s.ad_value(638), s.ad_value(639)), 0.5), p.p38);
        }

        s.store_sub_from_scalar_ad(123, p.p0, A::scale(s.ad_value(280), 2.0));

        s.v[279] = ((-p.p49) * (1.0 + (p.p50 / ((s.v[375]) as f64).powf(p.p51))));

        s.v[280] = ((-p.p49) * (1.0 + (p.p52 / ((s.v[375]) as f64).powf(p.p53))));

        s.v[281] = (-(p.p49 + (p.p54 * s.v[375])));

        s.v[638] = ((s.v[279] - s.v[280]) - 1e-12);

        s.v[639] = ((4.0 * s.v[280]) * 1e-12);

        if !(s.v[639] > 0.0) {
            s.store_scalar(639, (-s.v[639]));
        }

        s.store_sqrt_ad(639, A::offset(s.ad_value(639), (s.v[638] * s.v[638])));

        s.store_scale_ad(279, A::offset(A::div_from_scalar(s.v[638], s.ad_value(639)), 1.0), 0.5);

        s.store_offset_ad(138, A::scale(A::offset(s.ad_value(639), s.v[638]), 0.5), s.v[280]);

        s.store_offset(638, 138, (((-s.v[281])) + ((-1e-12))));

        s.v[639] = ((4.0 * s.v[281]) * 1e-12);

        if !(s.v[639] > 0.0) {
            s.store_scalar(639, (-s.v[639]));
        }

        s.store_sqrt_ad(639, A::add(A::square(s.ad_value(638)), s.ad_value(639)));

        s.store_scale_ad(279, A::offset(A::div(s.ad_value(638), s.ad_value(639)), 1.0), 0.5);

        s.store_offset_ad(138, A::scale(A::add(s.ad_value(638), s.ad_value(639)), 0.5), s.v[281]);

        s.store_neg(138, 138);

        s.store_mul_ad(128, A::scale(s.ad_value(122), 2.0), A::ln(A::div(s.ad_value(471), s.ad_value(127))));

        s.store_sqrt_ad(125, A::mul(A::div_from_scalar(1.034943e-10, s.ad_value(126)), s.ad_value(122)));

        s.store_mul_ad_lhs(141, A::scale(s.ad_value(126), 1.414213562373095), 125);

        s.copy_ad(438, 474);

        s.store_sqrt_ad(439, A::mul(A::scale(s.ad_value(438), 2.0), s.ad_value(122)));

        s.store_div(279, 127, 471);

        s.store_square(142, 279);

        s.store_div(279, 127, 462);

        s.store_square(143, 279);

        s.v[272] = p.p226;

        s.v[273] = (3.453133e-11 / s.v[272]);

        s.v[274] = (s.v[272] / 3.453133e-11);

        s.v[294] = (3.453133e-11 / p.p229);

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
        s.v[295] = (p.p229 / 3.453133e-11);

        s.store_scale(296, 471, ((-1.6021918e-19) * p.p227));

        s.v[535] = (1.034943e-10 / p.p227);

        s.v[536] = (1.0 / s.v[535]);

        s.v[293] = (s.v[295] + s.v[536]);

        s.v[31] = p.p254;

        s.v[30] = p.p255;

        s.v[688] = if (s.v[31] > (s.v[30] * 0.5)) { 1.0 } else { 0.0 };

        if (s.v[688] != 0.0) {
            s.store_scalar(31, (0.5 * s.v[30]));
        }

        s.v[689] = if (s.v[47] > s.v[31]) { 1.0 } else { 0.0 };

        if (s.v[689] != 0.0) {
            s.store_sub(280, 47, 31);
        }

        if (s.v[689] != 0.0) {
            s.store_sub_from_scalar(281, s.v[30], 31);
        }

        if (s.v[689] != 0.0) {
            s.store_square(642, 280);
        }

        if (s.v[689] != 0.0) {
            s.store_square(643, 281);
        }

        if (s.v[689] != 0.0) {
            s.store_scalar(644, 1.0);
        }

        if (s.v[689] != 0.0) {
            s.store_scalar(645, 1.0);
        }

        if (s.v[689] != 0.0) {
            s.store_scalar(647, 0.0);
        }

        if (s.v[689] != 0.0) {
            s.store_scalar(648, 0.0);
        }

        if (s.v[689] != 0.0) {
            s.store_scalar(220, 0.0);
        }

        if (s.v[689] != 0.0) {
            s.store_scalar(646, 0.0);
        }

        if (s.v[689] != 0.0) {
            s.store_mul(644, 644, 642);
        }

        if (s.v[689] != 0.0) {
            s.store_mul(645, 645, 643);
        }

        if (s.v[689] != 0.0) {
            s.store_mul(644, 644, 642);
        }

        if (s.v[689] != 0.0) {
            s.store_mul(645, 645, 643);
        }

        if (s.v[689] != 0.0) {
            s.store_mul(644, 644, 642);
        }

        if (s.v[689] != 0.0) {
            s.store_mul(645, 645, 643);
        }

        if (s.v[689] != 0.0) {
            s.store_mul(644, 644, 642);
        }

        if (s.v[689] != 0.0) {
            s.store_mul(645, 645, 643);
        }

        if (s.v[689] != 0.0) {
            s.store_add(220, 644, 645);
        }

        if (s.v[689] != 0.0) {
            s.copy_ad(646, 220);
        }

        s.v[690] = if ((((4.0 == 1.0) || (4.0 == 2.0)) || (4.0 == 4.0)) || (4.0 == 8.0)) { 1.0 } else { 0.0 };

        s.v[691] = if (4.0 == 1.0) { 1.0 } else { 0.0 };

        if (((s.v[689] != 0.0) && (s.v[690] != 0.0)) && (s.v[691] != 0.0)) {
            s.store_scalar(648, 1.0);
        }

        s.v[692] = if (4.0 == 2.0) { 1.0 } else { 0.0 };

        if ((((s.v[689] != 0.0) && (s.v[690] != 0.0)) && (!(s.v[691] != 0.0))) && (s.v[692] != 0.0)) {
            s.store_scalar(648, 2.0);
        }

        s.v[693] = if (4.0 == 4.0) { 1.0 } else { 0.0 };

        if (((((s.v[689] != 0.0) && (s.v[690] != 0.0)) && (!(s.v[691] != 0.0))) && (!(s.v[692] != 0.0))) && (s.v[693] != 0.0)) {
            s.store_scalar(648, 3.0);
        }

        s.v[694] = if (4.0 == 8.0) { 1.0 } else { 0.0 };

        if ((((((s.v[689] != 0.0) && (s.v[690] != 0.0)) && (!(s.v[691] != 0.0))) && (!(s.v[692] != 0.0))) && (!(s.v[693] != 0.0))) && (s.v[694] != 0.0)) {
            s.store_scalar(648, 4.0);
        }

        if ((s.v[689] != 0.0) && (s.v[690] != 0.0)) {
            s.store_scalar(647, 0.0);
        }

        let mut assign4560_loop_guard: usize = 0;
        while {
            let assign4560_cond_e3027: f64 = if (((s.v[689] != 0.0) && (s.v[690] != 0.0)) && (s.v[647] < s.v[648])) { 1.0 } else { 0.0 };
            assign4560_cond_e3027 != 0.0
        } {
            assign4560_loop_guard += 1;
            assert!(assign4560_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((s.v[689] != 0.0) && (s.v[690] != 0.0)) {
                s.store_sqrt(646, 646);
            }
            if ((s.v[689] != 0.0) && (s.v[690] != 0.0)) {
                s.store_offset(647, 647, 1.0);
            }
        }

        if ((s.v[689] != 0.0) && (!(s.v[690] != 0.0))) {
            s.store_powf(646, 646, (1.0 / (2.0 * 4.0)));
        }

        if (s.v[689] != 0.0) {
            s.store_div_from_scalar_ad(646, 1.0, A::offset(s.ad_value(646), 1e-50));
        }

        if (s.v[689] != 0.0) {
            s.store_mul_ad_lhs(282, A::mul(s.ad_value(280), s.ad_value(281)), 646);
        }

        if (s.v[689] != 0.0) {
            s.store_div_ad(286, A::mul(A::mul(s.ad_value(281), s.ad_value(645)), s.ad_value(646)), A::offset(s.ad_value(220), 1e-50));
        }

        if (s.v[689] != 0.0) {
            s.store_add(43, 31, 282);
        }

        if (s.v[689] != 0.0) {
            s.copy_ad(46, 286);
        }

        if (!(s.v[689] != 0.0)) {
            s.copy_ad(43, 47);
        }

        if (!(s.v[689] != 0.0)) {
            s.store_scalar(46, 1.0);
        }

        s.copy_ad(44, 48);

        s.copy_ad(45, 49);

        s.v[33] = 0.0;

        s.v[695] = 0.0;

        s.v[696] = 0.0;

        s.v[697] = 0.0;

        s.v[698] = 0.0;

        s.v[699] = 0.0;

        s.v[700] = 0.0;

        s.copy_ad(50, 43);

        s.copy_ad(51, 44);

        s.copy_ad(52, 45);

        s.v[62] = 0.0;

        s.v[63] = 0.0;

        s.store_scaled_mul(279, 46, 51, 0.5);

        s.store_scale(638, 279, (2.0 * 1.0 / (p.p216)));

        s.store_offset_ad(639, A::mul(s.ad_value(638), A::offset(A::mul(s.ad_value(638), A::offset(A::mul(s.ad_value(638), A::offset(A::mul(s.ad_value(638), A::offset(A::mul(s.ad_value(638), A::offset(A::scale(s.ad_value(638), (1.0 / 5040.0)), (1.0 / 720.0))), (1.0 / 120.0))), (1.0 / 24.0))), (1.0 / 6.0))), (1.0 / 2.0))), 1.0);

        s.store_offset_ad(640, A::mul(s.ad_value(638), A::offset(A::mul(s.ad_value(638), A::offset(A::mul(s.ad_value(638), A::offset(A::mul(s.ad_value(638), A::offset(A::scale(s.ad_value(638), (1.0 / 840.0)), (1.0 / 144.0))), (1.0 / 30.0))), (1.0 / 8.0))), (1.0 / 3.0))), (1.0 / 2.0));

        s.store_div_from_scalar(73, p.p216, 639);

        s.store_div_ad(280, A::scale(s.ad_value(640), (-2.0)), A::square(s.ad_value(639)));

        s.v[701] = if (s.v[73] < 1e-12) { 1.0 } else { 0.0 };

        if (s.v[701] != 0.0) {
            s.store_scalar(73, 1e-12);
        }

        s.store_add(70, 50, 73);

        s.store_add_ad_rhs(71, 51, A::scale(s.ad_value(73), 2.0));

        s.store_add(72, 52, 73);

        s.store_scale(279, 126, (2.0 * (1.034943e-10 * (s.v[274] * s.v[274]))));

        s.store_sub(280, 52, 138);

        s.store_offset_ad(281, A::mul(A::div_from_scalar(2.0, s.ad_value(279)), A::sub(A::sub(s.ad_value(280), s.ad_value(122)), s.ad_value(50))), 1.0);

        s.store_sqrt_ad(639, A::offset(A::square(s.ad_value(281)), ((4.0 * 0.001) * 0.001)));

        s.store_scale_ad(283, A::offset(A::div(s.ad_value(281), s.ad_value(639)), 1.0), 0.5);

        s.store_offset_ad(282, A::scale(A::add(s.ad_value(281), s.ad_value(639)), 0.5), (1e-10 * 0.001));

        s.v[702] = if (s.v[282] < 0.0) { 1.0 } else { 0.0 };

        if (s.v[702] != 0.0) {
            s.store_scalar(282, 0.0);
        }

        if (s.v[702] != 0.0) {
            s.store_scalar(283, 0.0);
        }

        s.store_sqrt_ad(290, A::offset(s.ad_value(282), 1e-50));

        s.store_add_ad_rhs(87, 280, A::mul(s.ad_value(279), A::sub_from_scalar(1.0, s.ad_value(290))));

        s.store_sub(88, 87, 128);

        s.store_offset(638, 88, (((-0.1)) + ((-0.05))));

        s.v[639] = ((4.0 * 0.1) * 0.05);

        if !(s.v[639] > 0.0) {
            s.store_scalar(639, (-s.v[639]));
        }

        s.store_sqrt_ad(639, A::add(A::square(s.ad_value(638)), s.ad_value(639)));

        s.store_scale_ad(284, A::offset(A::div(s.ad_value(638), s.ad_value(639)), 1.0), 0.5);

        s.store_offset_ad(88, A::scale(A::add(s.ad_value(638), s.ad_value(639)), 0.5), 0.1);

        s.store_div(279, 51, 88);

        s.copy_ad(638, 279);

        s.store_square(639, 638);

        s.store_mul(640, 639, 638);

        s.store_square(641, 639);

        s.store_div_from_scalar_ad(290, 1.0, A::add(A::add(A::add(A::offset(s.ad_value(638), 1.0), s.ad_value(639)), s.ad_value(640)), s.ad_value(641)));

        s.store_mul_ad_lhs(278, A::mul(A::neg(A::add(A::add(A::offset(A::scale(s.ad_value(638), 2.0), 1.0), A::scale(s.ad_value(639), 3.0)), A::scale(s.ad_value(640), 4.0))), s.ad_value(290)), 290);

        s.store_sub_from_scalar(290, 1.0, 290);

        s.store_neg(278, 278);

        s.store_square(276, 290);

        s.v[703] = if (((p.p193 == 0.0) && (p.p195 == 0.0)) || (p.p194 == 0.0)) { 1.0 } else { 0.0 };

        if (s.v[703] != 0.0) {
            s.store_scalar(37, 0.0);
        }

        if (!(s.v[703] != 0.0)) {
            s.store_scalar(37, 1.0);
        }

        s.store_add_ad(275, A::add(s.ad_value(129), s.ad_value(138)), A::scale(A::sqrt(A::mul(A::scale(s.ad_value(126), (2.0 * 1.034943e-10)), s.ad_value(129))), 1.0 / (s.v[273])));

        s.v[704] = if (s.v[37] == 0.0) { 1.0 } else { 0.0 };

        if (s.v[704] != 0.0) {
            s.store_scalar(268, s.v[272]);
        }

        if (s.v[704] != 0.0) {
            s.store_scalar(270, s.v[273]);
        }

        if (s.v[704] != 0.0) {
            s.store_scalar(271, s.v[274]);
        }

        if (s.v[704] != 0.0) {
            s.store_scale(278, 141, (s.v[274] * s.v[274]));
        }

        if (s.v[704] != 0.0) {
            s.store_mul(381, 278, 141);
        }

        if (!(s.v[704] != 0.0)) {
            s.store_offset_ad(283, A::sub(A::sub(s.ad_value(52), s.ad_value(50)), s.ad_value(275)), p.p194);
        }

        if (!(s.v[704] != 0.0)) {
            s.store_sqrt_ad(639, A::offset(A::square(s.ad_value(283)), ((4.0 * 0.0001) * 0.0001)));
        }

        if (!(s.v[704] != 0.0)) {
            s.store_scale_ad(281, A::offset(A::div(s.ad_value(283), s.ad_value(639)), 1.0), 0.5);
        }

        if (!(s.v[704] != 0.0)) {
            s.store_offset_ad(280, A::scale(A::add(s.ad_value(283), s.ad_value(639)), 0.5), (1e-10 * 0.0001));
        }

        s.v[705] = if (s.v[280] < 0.0) { 1.0 } else { 0.0 };

        if ((!(s.v[704] != 0.0)) && (s.v[705] != 0.0)) {
            s.store_scalar(280, 0.0);
        }

        if ((!(s.v[704] != 0.0)) && (s.v[705] != 0.0)) {
            s.store_scalar(281, 0.0);
        }

        if (!(s.v[704] != 0.0)) {
            s.store_div_from_scalar(281, 1.0, 280);
        }

        if (!(s.v[704] != 0.0)) {
            s.store_scale_ad(282, A::abs(s.ad_value(275)), 2.0);
        }

        if (!(s.v[704] != 0.0)) {
            s.store_offset_ad(284, A::sub(s.ad_value(138), s.ad_value(275)), p.p194);
        }

        s.v[706] = if (s.v[284] > s.v[282]) { 1.0 } else { 0.0 };

        if ((!(s.v[704] != 0.0)) && (s.v[706] != 0.0)) {
            s.copy_ad(282, 284);
        }

        if (!(s.v[704] != 0.0)) {
            s.store_offset_ad(638, A::sub(A::div_from_scalar(1.0, s.ad_value(282)), s.ad_value(281)), (-0.0001));
        }

        if (!(s.v[704] != 0.0)) {
            s.store_scale_ad(639, A::div_from_scalar(1.0, s.ad_value(282)), (4.0 * 0.0001));
        }

        if (!(s.v[704] != 0.0)) {
            s.store_ad(639, &{
                if (s.v[639] > 0.0) {
                    s.ad_value(639)
                } else {
                    A::neg(s.ad_value(639))
                }
            });
        }

        if (!(s.v[704] != 0.0)) {
            s.store_sqrt_ad(639, A::add(A::square(s.ad_value(638)), s.ad_value(639)));
        }

        if (!(s.v[704] != 0.0)) {
            s.store_scale_ad(284, A::offset(A::div(s.ad_value(638), s.ad_value(639)), 1.0), 0.5);
        }

        if (!(s.v[704] != 0.0)) {
            s.store_sub_ad(280, A::div_from_scalar(1.0, s.ad_value(282)), A::scale(A::add(s.ad_value(638), s.ad_value(639)), 0.5));
        }

        if (!(s.v[704] != 0.0)) {
            s.store_offset_scaled(269, 280, p.p193, p.p195);
        }

        s.v[707] = if ((s.v[269] * 1000000000000.0) < s.v[272]) { 1.0 } else { 0.0 };

        if ((!(s.v[704] != 0.0)) && (s.v[707] != 0.0)) {
            s.store_scalar(269, 0.0);
        }

        if ((!(s.v[704] != 0.0)) && (s.v[707] != 0.0)) {
            s.store_scalar(37, 0.0);
        }

        if (!(s.v[704] != 0.0)) {
            s.store_offset(268, 269, s.v[272]);
        }

        if (!(s.v[704] != 0.0)) {
            s.store_div_from_scalar(270, 3.453133e-11, 268);
        }

        if (!(s.v[704] != 0.0)) {
            s.store_scale(271, 268, 28959208927.08158);
        }

        if (!(s.v[704] != 0.0)) {
            s.store_mul_ad_lhs(381, A::mul(A::square(s.ad_value(141)), s.ad_value(271)), 271);
        }

        s.store_offset_ad(638, A::sub_from_scalar(0.5, s.ad_value(70)), (-0.001));

        s.v[639] = ((4.0 * 0.5) * 0.001);

        if !(s.v[639] > 0.0) {
            s.store_scalar(639, (-s.v[639]));
        }

        s.store_sqrt_ad(639, A::add(A::square(s.ad_value(638)), s.ad_value(639)));

        s.store_scale_ad(278, A::offset(A::div(s.ad_value(638), s.ad_value(639)), 1.0), 0.5);

        s.store_sub_from_scalar_ad(382, 0.5, A::scale(A::add(s.ad_value(638), s.ad_value(639)), 0.5));

        s.store_sqrt_ad(150, A::mul(s.ad_value(473), s.ad_value(129)));

        s.store_add_ad_lhs(265, A::add(A::add(s.ad_value(129), s.ad_value(138)), A::mul(s.ad_value(150), s.ad_value(271))), 380);

        s.copy_ad(130, 129);

        s.v[278] = 0.95;

        s.store_offset_ad(279, A::sub(A::scale(s.ad_value(130), s.v[278]), s.ad_value(382)), (-0.001));

        s.store_sqrt_ad(280, A::add(A::square(s.ad_value(279)), A::scale(s.ad_value(130), ((4.0 * s.v[278]) * 0.001))));

        s.store_sub_ad_rhs(131, 130, A::sub(A::scale(s.ad_value(130), s.v[278]), A::scale(A::add(s.ad_value(279), s.ad_value(280)), 0.5)));

        s.store_sqrt(135, 131);

        s.v[708] = if (p.p58 != 0.0) { 1.0 } else { 0.0 };

        if (s.v[708] != 0.0) {
            s.store_sqrt_ad(278, A::mul(A::scale(s.ad_value(471), ((2.0 * 1.6021918e-19) * 1.034943e-10)), s.ad_value(136)));
        }

        if (s.v[708] != 0.0) {
            s.store_add_ad(79, A::add(s.ad_value(136), s.ad_value(138)), A::mul(s.ad_value(278), s.ad_value(271)));
        }

        if (s.v[708] != 0.0) {
            s.store_scalar(278, ((2.0 * p.p227) / (p.p58 * p.p58)));
        }

        if (s.v[708] != 0.0) {
            s.store_mul_ad(81, A::mul(A::scale(s.ad_value(271), 1.034943e-10), s.ad_value(278)), A::sub_from_scalar(p.p55, s.ad_value(130)));
        }

        if (s.v[708] != 0.0) {
            s.store_add_ad(278, A::offset(A::scale(s.ad_value(131), (p.p68 / p.p58)), p.p66), A::scale(s.ad_value(71), p.p67));
        }

        if (s.v[708] != 0.0) {
            s.store_mul_ad_lhs(266, A::mul(A::sub(s.ad_value(265), s.ad_value(79)), s.ad_value(81)), 278);
        }

        if (!(s.v[708] != 0.0)) {
            s.store_scalar(266, 0.0);
        }

        s.v[709] = if (p.p297 != 0.0) { 1.0 } else { 0.0 };

        if (s.v[709] != 0.0) {
            s.store_offset_ad(288, A::add(A::sub(s.ad_value(122), A::scale(A::mul(s.ad_value(381), s.ad_value(120)), 0.25)), s.ad_value(138)), 1e-50);
        }

        if (s.v[709] != 0.0) {
            s.store_offset_ad(279, A::sub(s.ad_value(72), s.ad_value(288)), (-0.005));
        }

        if (s.v[709] != 0.0) {
            s.store_scalar(278, (if (s.v[288] >= 0.0) { 1.0 } else { (-1.0) }));
        }

        if (s.v[709] != 0.0) {
            s.store_sqrt_ad(280, A::add(A::square(s.ad_value(279)), A::scale(A::mul(A::scale(s.ad_value(278), 4.0), s.ad_value(288)), 0.005)));
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
        if (s.v[709] != 0.0) {
            s.store_sub_ad_lhs(281, A::add(s.ad_value(288), A::scale(A::add(s.ad_value(279), s.ad_value(280)), 0.5)), 138);
        }

        if (s.v[709] != 0.0) {
            s.store_mul_ad_lhs(282, A::mul(A::div_from_scalar(4.0, s.ad_value(381)), s.ad_value(122)), 122);
        }

        if (s.v[709] != 0.0) {
            s.store_offset_ad(283, A::mul(s.ad_value(120), s.ad_value(281)), (-1.0));
        }

        if (s.v[709] != 0.0) {
            s.store_offset_ad(279, A::mul(s.ad_value(283), s.ad_value(282)), 1.0);
        }

        if (s.v[709] != 0.0) {
            s.store_sqrt_ad(639, A::offset(A::square(s.ad_value(279)), ((4.0 * 0.001) * 0.001)));
        }

        if (s.v[709] != 0.0) {
            s.store_scale_ad(285, A::offset(A::div(s.ad_value(279), s.ad_value(639)), 1.0), 0.5);
        }

        if (s.v[709] != 0.0) {
            s.store_offset_ad(279, A::scale(A::add(s.ad_value(279), s.ad_value(639)), 0.5), (1e-10 * 0.001));
        }

        s.v[710] = if (s.v[279] < 0.0) { 1.0 } else { 0.0 };

        if ((s.v[709] != 0.0) && (s.v[710] != 0.0)) {
            s.store_scalar(279, 0.0);
        }

        if ((s.v[709] != 0.0) && (s.v[710] != 0.0)) {
            s.store_scalar(285, 0.0);
        }

        if (s.v[709] != 0.0) {
            s.store_sqrt_ad(280, A::offset(s.ad_value(279), (10.0 * 2.220446049250313e-16)));
        }

        if (s.v[709] != 0.0) {
            s.store_add_ad_rhs(139, 281, A::mul(A::mul(A::scale(s.ad_value(381), 0.5), s.ad_value(120)), A::sub_from_scalar(1.0, s.ad_value(280))));
        }

        if (s.v[709] != 0.0) {
            s.store_offset_ad(638, A::sub(s.ad_value(129), s.ad_value(139)), (-0.005));
        }

        if (s.v[709] != 0.0) {
            s.store_scale(639, 129, (4.0 * 0.005));
        }

        if (s.v[709] != 0.0) {
            s.store_ad(639, &{
                if (s.v[639] > 0.0) {
                    s.ad_value(639)
                } else {
                    A::neg(s.ad_value(639))
                }
            });
        }

        if (s.v[709] != 0.0) {
            s.store_sqrt_ad(639, A::add(A::square(s.ad_value(638)), s.ad_value(639)));
        }

        if (s.v[709] != 0.0) {
            s.store_scale_ad(280, A::offset(A::div(s.ad_value(638), s.ad_value(639)), 1.0), 0.5);
        }

        if (s.v[709] != 0.0) {
            s.store_sub_ad_rhs(140, 129, A::scale(A::add(s.ad_value(638), s.ad_value(639)), 0.5));
        }

        if (s.v[709] != 0.0) {
            s.store_add_ad_rhs(130, 129, A::scale(A::sub(s.ad_value(140), s.ad_value(129)), p.p297));
        }

        s.store_scale(279, 271, (1.034943e-10 * (p.p227 * 2.0)));

        s.store_sub_from_scalar(280, p.p55, 130);

        s.v[281] = (s.v[277] - p.p57);

        s.store_scaled_mul(81, 279, 280, 1.0 / ((s.v[281] * s.v[281])));

        s.store_sqrt_ad(639, A::offset(A::square(s.ad_value(50)), ((4.0 * 0.001) * 0.001)));

        s.store_scale_ad(278, A::offset(A::div(s.ad_value(50), s.ad_value(639)), 1.0), 0.5);

        s.store_offset_ad(593, A::scale(A::add(s.ad_value(50), s.ad_value(639)), 0.5), (1e-10 * 0.001));

        s.v[711] = if (s.v[593] < 0.0) { 1.0 } else { 0.0 };

        if (s.v[711] != 0.0) {
            s.store_scalar(593, 0.0);
        }

        if (s.v[711] != 0.0) {
            s.store_scalar(278, 0.0);
        }

        s.store_add_ad(283, A::add(A::offset(A::scale(s.ad_value(131), (p.p71 / s.v[277])), p.p69), A::scale(s.ad_value(71), p.p70)), A::scale(s.ad_value(593), p.p250));

        s.store_mul(82, 81, 283);

        s.v[712] = if (p.p72 > 0.0) { 1.0 } else { 0.0 };

        if (s.v[712] != 0.0) {
            s.store_add_ad(279, A::offset(A::add(s.ad_value(137), s.ad_value(128)), (-(2.0 * p.p74))), A::scale(s.ad_value(71), p.p73));
        }

        if (s.v[712] != 0.0) {
            s.store_scalar(280, ((s.v[277] * 0.5) + p.p56));
        }

        if (s.v[712] != 0.0) {
            s.store_div_from_scalar(281, (p.p72 * p.p227), 280);
        }

        if (s.v[712] != 0.0) {
            s.store_mul(83, 279, 281);
        }

        if (!(s.v[712] != 0.0)) {
            s.store_scalar(83, 0.0);
        }

        s.store_div_from_scalar_ad(281, 1.0, A::offset(s.ad_value(270), (s.v[626] / s.v[124])));

        s.store_sub(283, 271, 281);

        s.store_offset_ad(84, A::mul(s.ad_value(150), s.ad_value(283)), (p.p104 / s.v[376]));

        s.store_offset_ad(80, A::add(A::add(A::add(s.ad_value(82), s.ad_value(266)), s.ad_value(84)), s.ad_value(83)), s.v[482]);

        s.store_sub(78, 265, 80);

        s.v[713] = if (p.p75 == 0.0) { 1.0 } else { 0.0 };

        if (s.v[713] != 0.0) {
            s.store_scalar(36, 0.0);
        }

        if (!(s.v[713] != 0.0)) {
            s.store_scalar(36, 1.0);
        }

        s.v[714] = if (s.v[36] == 0.0) { 1.0 } else { 0.0 };

        if (s.v[714] != 0.0) {
            s.store_scalar(267, 0.0);
        }

        if (!(s.v[714] != 0.0)) {
            s.store_offset(281, 72, (-p.p76));
        }

        s.v[715] = if (s.v[281] < (-3.0)) { 1.0 } else { 0.0 };

        if ((!(s.v[714] != 0.0)) && (s.v[715] != 0.0)) {
            s.store_scalar(284, 0.0);
        }

        if ((!(s.v[714] != 0.0)) && (s.v[715] != 0.0)) {
            s.store_scalar(267, 0.0);
        }

        s.v[716] = if (s.v[281] < 0.0) { 1.0 } else { 0.0 };

        if (((!(s.v[714] != 0.0)) && (!(s.v[715] != 0.0))) && (s.v[716] != 0.0)) {
            s.store_offset_ad(284, A::mul(s.ad_value(281), A::offset(A::scale(s.ad_value(281), (3.0 * (1.0 / 27.0))), (2.0 * (1.0 / 3.0)))), 1.0);
        }

        if (((!(s.v[714] != 0.0)) && (!(s.v[715] != 0.0))) && (s.v[716] != 0.0)) {
            s.store_offset_ad(267, A::mul(s.ad_value(281), A::offset(A::mul(s.ad_value(281), A::offset(A::scale(s.ad_value(281), (1.0 / 27.0)), (1.0 / 3.0))), 1.0)), 1.0);
        }

        if (((!(s.v[714] != 0.0)) && (!(s.v[715] != 0.0))) && (!(s.v[716] != 0.0))) {
            s.store_offset_ad(284, A::mul(s.ad_value(281), A::offset(A::mul(s.ad_value(281), A::offset(A::scale(s.ad_value(281), (4.0 * 0.148148111111111)), (3.0 * 0.0402052934513951))), (2.0 * (1.0 / 3.0)))), 1.0);
        }

        if (((!(s.v[714] != 0.0)) && (!(s.v[715] != 0.0))) && (!(s.v[716] != 0.0))) {
            s.store_offset_ad(267, A::mul(s.ad_value(281), A::offset(A::mul(s.ad_value(281), A::offset(A::mul(s.ad_value(281), A::offset(A::scale(s.ad_value(281), 0.148148111111111), 0.0402052934513951)), (1.0 / 3.0))), 1.0)), 1.0);
        }

        if (!(s.v[714] != 0.0)) {
            s.store_sqrt_ad(639, A::offset(A::mul(A::offset(s.ad_value(267), (-1.0)), A::offset(s.ad_value(267), (-1.0))), ((4.0 * 0.1) * 0.1)));
        }

        if (!(s.v[714] != 0.0)) {
            s.store_scale_ad(284, A::offset(A::div(A::offset(s.ad_value(267), (-1.0)), s.ad_value(639)), 1.0), 0.5);
        }

        if (!(s.v[714] != 0.0)) {
            s.store_offset_ad(267, A::scale(A::add(A::offset(s.ad_value(267), (-1.0)), s.ad_value(639)), 0.5), (1e-10 * 0.1));
        }

        s.v[717] = if (s.v[267] < 0.0) { 1.0 } else { 0.0 };

        if ((!(s.v[714] != 0.0)) && (s.v[717] != 0.0)) {
            s.store_scalar(267, 0.0);
        }

        if ((!(s.v[714] != 0.0)) && (s.v[717] != 0.0)) {
            s.store_scalar(284, 0.0);
        }

        if (!(s.v[714] != 0.0)) {
            s.store_scale(267, 267, s.v[479]);
        }

        if (!(s.v[714] != 0.0)) {
            s.store_offset_ad(638, A::sub_from_scalar(1.0, s.ad_value(267)), (-0.05));
        }

        if (!(s.v[714] != 0.0)) {
            s.store_scalar(639, (4.0 * 0.05));
        }

        if (!(s.v[714] != 0.0)) {
            s.store_ad(639, &{
                if (s.v[639] > 0.0) {
                    s.ad_value(639)
                } else {
                    A::neg(s.ad_value(639))
                }
            });
        }

        if (!(s.v[714] != 0.0)) {
            s.store_sqrt_ad(639, A::add(A::square(s.ad_value(638)), s.ad_value(639)));
        }

        if (!(s.v[714] != 0.0)) {
            s.store_scale_ad(287, A::offset(A::div(s.ad_value(638), s.ad_value(639)), 1.0), 0.5);
        }

        if (!(s.v[714] != 0.0)) {
            s.store_sub_from_scalar_ad(267, 1.0, A::scale(A::add(s.ad_value(638), s.ad_value(639)), 0.5));
        }

        s.store_sub_ad_lhs(53, A::add(A::sub(s.ad_value(52), s.ad_value(138)), s.ad_value(80)), 267);

        s.copy_ad(76, 53);

        s.store_mul_ad_rhs(298, 122, A::ln(A::div(s.ad_value(471), s.ad_value(462))));

        s.store_add_ad_lhs(54, A::sub(s.ad_value(138), s.ad_value(80)), 267);

        s.store_mul(144, 141, 271);

        s.store_square(145, 144);

        if (p.p29 != 0.0) {
            s.store_add(440, 70, 298);
        }

        if (!(p.p29 != 0.0)) {
            s.store_add(440, 50, 298);
        }

        s.v[718] = if (s.v[440] < 0.0) { 1.0 } else { 0.0 };

        if (s.v[718] != 0.0) {
            s.store_div(278, 462, 471);
        }

        if (s.v[718] != 0.0) {
            s.store_offset(279, 278, 1.0);
        }

        if (s.v[718] != 0.0) {
            s.store_add_ad(280, A::sub(s.ad_value(122), s.ad_value(440)), A::mul(s.ad_value(278), A::add(s.ad_value(122), s.ad_value(440))));
        }

        if (s.v[718] != 0.0) {
            s.store_scale_ad(281, A::square(s.ad_value(439)), (s.v[295] * s.v[295]));
        }

        if (s.v[718] != 0.0) {
            s.store_sub_ad(282, A::mul(A::scale(s.ad_value(280), 2.0), s.ad_value(279)), A::mul(s.ad_value(281), s.ad_value(120)));
        }

        if (s.v[718] != 0.0) {
            s.store_add_ad_lhs(283, A::add(A::square(s.ad_value(280)), A::mul(A::mul(s.ad_value(281), s.ad_value(120)), s.ad_value(440))), 281);
        }

        if (s.v[718] != 0.0) {
            s.store_ad(285, &{
                if (((s.v[282] * s.v[282]) - (((4.0 * s.v[279]) * s.v[279]) * s.v[283])) >= 1e-50) {
                    A::sub(A::square(s.ad_value(282)), A::mul(A::mul(A::scale(s.ad_value(279), 4.0), s.ad_value(279)), s.ad_value(283)))
                } else {
                    A::constant(1e-50)
                }
            });
        }

        if (s.v[718] != 0.0) {
            s.store_div_ad(331, A::add(s.ad_value(282), A::sqrt(s.ad_value(285))), A::offset(A::square(s.ad_value(279)), 2.0));
        }

        if (!(s.v[718] != 0.0)) {
            s.store_mul_ad_lhs(279, A::square(s.ad_value(439)), 120);
        }

        if (!(s.v[718] != 0.0)) {
            s.store_mul_ad_lhs(280, A::square(s.ad_value(141)), 120);
        }

        if (!(s.v[718] != 0.0)) {
            s.store_neg_ad(281, A::add(s.ad_value(122), A::scale(s.ad_value(440), 2.0)));
        }

        if (!(s.v[718] != 0.0)) {
            s.store_offset_ad(282, A::div(s.ad_value(280), s.ad_value(279)), 1.0);
        }

        if (!(s.v[718] != 0.0)) {
            s.store_scale_ad(283, A::square(s.ad_value(141)), (s.v[295] * s.v[295]));
        }

        if (!(s.v[718] != 0.0)) {
            s.store_sub_ad(284, A::mul(s.ad_value(283), s.ad_value(120)), A::mul(A::scale(s.ad_value(281), 2.0), s.ad_value(282)));
        }

        if (!(s.v[718] != 0.0)) {
            s.store_ad(285, &{
                if (((s.v[284] * s.v[284]) - ((((4.0 * s.v[282]) * s.v[282]) * s.v[281]) * s.v[281])) >= 1e-50) {
                    A::sub(A::square(s.ad_value(284)), A::mul(A::mul(A::mul(A::scale(s.ad_value(282), 4.0), s.ad_value(282)), s.ad_value(281)), s.ad_value(281)))
                } else {
                    A::constant(1e-50)
                }
            });
        }

        if (!(s.v[718] != 0.0)) {
            s.store_div_ad(331, A::add(s.ad_value(284), A::sqrt(s.ad_value(285))), A::mul(A::scale(s.ad_value(282), 2.0), s.ad_value(282)));
        }

        s.store_mul_ad(326, A::div_from_scalar(2.0, s.ad_value(120)), A::ln(A::div(s.ad_value(462), s.ad_value(127))));

        s.store_scale_ad(278, A::square(s.ad_value(439)), (s.v[293] * s.v[293]));

        s.store_neg(279, 440);

        s.store_sub_ad(280, A::mul(A::add(A::scale(s.ad_value(279), 2.0), A::mul(s.ad_value(278), s.ad_value(120))), A::add(A::scale(s.ad_value(279), 2.0), A::mul(s.ad_value(278), s.ad_value(120)))), A::scale(A::add(A::square(s.ad_value(279)), s.ad_value(278)), 4.0));

        if !(s.v[280] >= (10.0 * 2.220446049250313e-16)) {
            s.store_scalar(280, (10.0 * 2.220446049250313e-16));
        }

        s.store_sqrt(280, 280);

        s.store_add_ad(281, A::scale(s.ad_value(279), 2.0), A::mul(s.ad_value(278), s.ad_value(120)));

        s.store_scaled_sub(324, 281, 280, 0.5);

        s.store_div_ad(325, A::ln(A::div(A::div(A::square(s.ad_value(279)), s.ad_value(278)), s.ad_value(143))), A::add(s.ad_value(120), A::div_from_scalar(2.0, s.ad_value(279))));

        s.v[719] = if (s.v[324] < s.v[326]) { 1.0 } else { 0.0 };

        if (s.v[719] != 0.0) {
            s.copy_ad(331, 324);
        }

        if (!(s.v[719] != 0.0)) {
            s.store_offset_ad(638, A::sub(s.ad_value(325), s.ad_value(324)), (-0.0008));
        }

        if (!(s.v[719] != 0.0)) {
            s.store_scale(639, 325, (4.0 * 0.0008));
        }

        if (!(s.v[719] != 0.0)) {
            s.store_ad(639, &{
                if (s.v[639] > 0.0) {
                    s.ad_value(639)
                } else {
                    A::neg(s.ad_value(639))
                }
            });
        }

        if (!(s.v[719] != 0.0)) {
            s.store_sqrt_ad(639, A::add(A::square(s.ad_value(638)), s.ad_value(639)));
        }

        if (!(s.v[719] != 0.0)) {
            s.store_scale_ad(279, A::offset(A::div(s.ad_value(638), s.ad_value(639)), 1.0), 0.5);
        }

        if (!(s.v[719] != 0.0)) {
            s.store_sub_ad_rhs(331, 325, A::scale(A::add(s.ad_value(638), s.ad_value(639)), 0.5));
        }

        s.v[62] = 0.0;

        let mut assign6930_loop_guard: usize = 0;
        while {
            let assign6930_cond_e4908: f64 = if s.v[62] < s.v[28] { 1.0 } else { 0.0 };
            assign6930_cond_e4908 != 0.0
        } {
            assign6930_loop_guard += 1;
            assert!(assign6930_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            s.copy_ad(279, 439);
            s.store_mul(280, 120, 331);
            s.store_exp_ad(281, A::neg(s.ad_value(280)));
            s.v[720] = if (s.v[331] > 1e-8) { 1.0 } else { 0.0 };
            if (s.v[720] != 0.0) {
                s.store_exp_ad(278, A::mul(s.ad_value(120), s.ad_value(331)));
            }
            if (s.v[720] != 0.0) {
                s.store_mul_ad(282, A::neg(s.ad_value(279)), A::sqrt(A::add(A::offset(A::add(s.ad_value(281), s.ad_value(280)), (-1.0)), A::mul(s.ad_value(143), A::offset(s.ad_value(278), (-1.0))))));
            }
            if (s.v[720] != 0.0) {
                s.store_mul_ad(283, A::div(s.ad_value(438), s.ad_value(282)), A::add(A::sub_from_scalar(1.0, s.ad_value(281)), A::mul(s.ad_value(143), s.ad_value(278))));
            }
            s.v[721] = if (s.v[331] < (-1e-8)) { 1.0 } else { 0.0 };
            if ((!(s.v[720] != 0.0)) && (s.v[721] != 0.0)) {
                s.store_mul_ad_rhs(282, 279, A::sqrt(A::offset(A::add(s.ad_value(281), s.ad_value(280)), (-1.0))));
            }
            if ((!(s.v[720] != 0.0)) && (s.v[721] != 0.0)) {
                s.store_mul_ad(283, A::div(s.ad_value(438), s.ad_value(282)), A::sub_from_scalar(1.0, s.ad_value(281)));
            }
            if ((!(s.v[720] != 0.0)) && (!(s.v[721] != 0.0))) {
                s.store_mul_ad_lhs(282, A::mul(A::neg(A::sqrt(A::div(s.ad_value(438), s.ad_value(120)))), s.ad_value(120)), 331);
            }
            if ((!(s.v[720] != 0.0)) && (!(s.v[721] != 0.0))) {
                s.store_neg_ad(283, A::sqrt(A::mul(s.ad_value(438), s.ad_value(120))));
            }
            s.store_sqrt_ad(639, A::offset(A::square(s.ad_value(282)), ((4.0 * 1e-6) * 1e-6)));
            s.store_scale_ad(285, A::offset(A::div(s.ad_value(282), s.ad_value(639)), 1.0), 0.5);
            s.store_offset_ad(284, A::scale(A::add(s.ad_value(282), s.ad_value(639)), 0.5), (1e-10 * 1e-6));
            s.v[722] = if (s.v[284] < 0.0) { 1.0 } else { 0.0 };
            if (s.v[722] != 0.0) {
                s.store_scalar(284, 0.0);
            }
            if (s.v[722] != 0.0) {
                s.store_scalar(285, 0.0);
            }
            s.store_offset_ad(638, A::sub(A::neg(s.ad_value(296)), s.ad_value(284)), (-1e-9));
            s.store_scale_ad(639, A::neg(s.ad_value(296)), (4.0 * 1e-9));
            if !(s.v[639] > 0.0) {
                s.store_neg(639, 639);
            }
            s.store_sqrt_ad(639, A::add(A::square(s.ad_value(638)), s.ad_value(639)));
            s.store_scale_ad(286, A::offset(A::div(s.ad_value(638), s.ad_value(639)), 1.0), 0.5);
            s.store_sub_ad(284, A::neg(s.ad_value(296)), A::scale(A::add(s.ad_value(638), s.ad_value(639)), 0.5));
            s.store_mul_ad_rhs(285, 285, A::mul(s.ad_value(283), s.ad_value(286)));
            s.store_div_ad_lhs(334, A::scale(A::scale(A::scale(A::square(s.ad_value(284)), 0.5), 9662367879.197212), 6.241449993689894e18), 471);
            s.store_div_ad_lhs(335, A::mul(A::scale(s.ad_value(334), 2.0), s.ad_value(285)), 284);
            s.store_sub_ad_rhs(284, 331, A::div(A::add(A::sub(A::sub(A::scale(s.ad_value(282), 1.0 / (s.v[294])), s.ad_value(331)), s.ad_value(440)), s.ad_value(334)), A::add(A::offset(A::scale(s.ad_value(283), 1.0 / (s.v[294])), (-1.0)), s.ad_value(335))));
            s.v[723] = if ((((s.v[284] - s.v[331])) as f64).abs() < 0.001) { 1.0 } else { 0.0 };
            if (s.v[723] != 0.0) {
                s.store_scalar(62, s.v[28]);
            }
            s.copy_ad(331, 284);
            s.copy_ad(330, 282);
            s.store_offset(62, 62, 1.0);
        }

        s.copy_ad(332, 334);

        s.store_sqrt_ad(279, A::div(A::scale(s.ad_value(332), ((2.0 * 1.034943e-10) / 1.6021918e-19)), s.ad_value(471)));

        s.v[724] = if (s.v[279] > (0.99 * p.p227)) { 1.0 } else { 0.0 };

        if (s.v[724] != 0.0) {
            s.store_div_from_scalar(278, 1.0, 270);
        }

        if (s.v[724] != 0.0) {
            s.store_scalar(280, (1.0 / s.v[294]));
        }

        if (s.v[724] != 0.0) {
            s.store_div_from_scalar_ad(281, 1.0, A::add(A::offset(s.ad_value(278), s.v[536]), s.ad_value(280)));
        }

        if (s.v[724] != 0.0) {
            s.store_sub_from_scalar_ad(282, 1.0, A::mul(s.ad_value(281), s.ad_value(278)));
        }

        if (s.v[724] != 0.0) {
            s.store_mul_ad_rhs(283, 278, A::mul(s.ad_value(281), A::sub(A::mul(A::offset(s.ad_value(280), (0.5 * s.v[536])), A::neg(s.ad_value(296))), s.ad_value(440))));
        }

        if (s.v[724] != 0.0) {
            s.store_div(327, 283, 282);
        }

        if (s.v[724] != 0.0) {
            s.store_add(54, 54, 327);
        }

        if (s.v[724] != 0.0) {
            s.store_sub_ad_rhs(53, 53, A::scale(s.ad_value(327), p.p298));
        }

        if (s.v[724] != 0.0) {
            s.copy_ad(76, 53);
        }

        s.v[725] = if (s.v[33] >= 1.0) { 1.0 } else { 0.0 };

        if (s.v[725] != 0.0) {
            s.store_scalar(305, s.v[695]);
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
        if (s.v[725] != 0.0) {
            s.store_scalar(306, s.v[696]);
        }

        if (s.v[725] != 0.0) {
            s.store_offset(307, 440, s.v[697]);
        }

        if (s.v[725] != 0.0) {
            s.store_add_ad_lhs(328, A::scale(A::neg(s.ad_value(296)), (s.v[536] * 0.5)), 122);
        }

        if (s.v[725] != 0.0) {
            s.store_sub_ad_rhs(329, 328, A::scale(s.ad_value(330), s.v[536]));
        }

        s.v[726] = if (s.v[440] < 0.0) { 1.0 } else { 0.0 };

        if ((!(s.v[725] != 0.0)) && (s.v[726] != 0.0)) {
            s.store_scalar(55, 0.0);
        }

        if ((!(s.v[725] != 0.0)) && (s.v[726] != 0.0)) {
            s.store_scalar(62, 1.0);
        }

        let mut assign7150_loop_guard: usize = 0;
        while {
            let assign7150_cond_e5303: f64 = if (((!(s.v[725] != 0.0)) && (s.v[726] != 0.0)) && (s.v[62] <= s.v[28])) { 1.0 } else { 0.0 };
            assign7150_cond_e5303 != 0.0
        } {
            assign7150_loop_guard += 1;
            assert!(assign7150_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((!(s.v[725] != 0.0)) && (s.v[726] != 0.0)) {
                s.store_div_from_scalar_ad(278, s.v[294], A::scale(s.ad_value(462), ((2.0 * 1.6021918e-19) * 1.034943e-10)));
            }
            if ((!(s.v[725] != 0.0)) && (s.v[726] != 0.0)) {
                s.store_scalar(279, (1.0 + (s.v[294] * s.v[536])));
            }
            if ((!(s.v[725] != 0.0)) && (s.v[726] != 0.0)) {
                s.store_scale_ad(280, A::add(A::add(A::scale(A::neg(s.ad_value(296)), (0.5 * s.v[536])), s.ad_value(122)), s.ad_value(440)), s.v[294]);
            }
            if ((!(s.v[725] != 0.0)) && (s.v[726] != 0.0)) {
                s.store_mul_ad_lhs(285, A::mul(A::scale(s.ad_value(278), 2.0), s.ad_value(270)), 270);
            }
            if ((!(s.v[725] != 0.0)) && (s.v[726] != 0.0)) {
                s.store_add_ad(282, A::add(A::offset(A::mul(s.ad_value(279), s.ad_value(270)), s.v[294]), A::mul(A::mul(A::scale(s.ad_value(278), 2.0), s.ad_value(270)), s.ad_value(296))), A::mul(s.ad_value(285), s.ad_value(55)));
            }
            if ((!(s.v[725] != 0.0)) && (s.v[726] != 0.0)) {
                s.store_mul_ad_lhs(286, A::mul(A::scale(s.ad_value(270), ((2.0 * s.v[294]) * 2.0)), s.ad_value(278)), 270);
            }
            if ((!(s.v[725] != 0.0)) && (s.v[726] != 0.0)) {
                let assign7150_body6_ad_e5439: A = A::add(A::add(A::offset(A::mul(A::mul(A::sub(A::square(s.ad_value(279)), A::mul(A::scale(s.ad_value(278), 4.0), s.ad_value(280))), s.ad_value(270)), s.ad_value(270)), (s.v[294] * s.v[294])), A::mul(A::scale(s.ad_value(270), (2.0 * s.v[294])), A::add(s.ad_value(279), A::mul(A::scale(s.ad_value(278), 2.0), s.ad_value(296))))), A::mul(s.ad_value(286), s.ad_value(55)));
                s.store_ad(283, &assign7150_body6_ad_e5439);
            }
            if ((!(s.v[725] != 0.0)) && (s.v[726] != 0.0)) {
                s.store_sqrt(283, 283);
            }
            if ((!(s.v[725] != 0.0)) && (s.v[726] != 0.0)) {
                s.store_div_ad_rhs(286, 286, A::scale(s.ad_value(283), 2.0));
            }
            if ((!(s.v[725] != 0.0)) && (s.v[726] != 0.0)) {
                s.store_div_from_scalar_ad(284, 1.0, A::mul(A::mul(A::scale(s.ad_value(278), 2.0), s.ad_value(270)), s.ad_value(270)));
            }
            if ((!(s.v[725] != 0.0)) && (s.v[726] != 0.0)) {
                s.store_mul_ad_rhs(346, 284, A::sub(s.ad_value(282), s.ad_value(283)));
            }
            if ((!(s.v[725] != 0.0)) && (s.v[726] != 0.0)) {
                s.store_mul_ad_rhs(347, 284, A::sub(s.ad_value(285), s.ad_value(286)));
            }
            if ((!(s.v[725] != 0.0)) && (s.v[726] != 0.0)) {
                s.store_div_ad_lhs(370, A::neg(s.ad_value(346)), 347);
            }
            s.v[727] = if (((s.v[370]) as f64).abs() < 1e-12) { 1.0 } else { 0.0 };
            if (((!(s.v[725] != 0.0)) && (s.v[726] != 0.0)) && (s.v[727] != 0.0)) {
                s.store_scalar(62, s.v[28]);
            }
            s.v[728] = if (s.v[370] > 0.1) { 1.0 } else { 0.0 };
            if ((((!(s.v[725] != 0.0)) && (s.v[726] != 0.0)) && (!(s.v[727] != 0.0))) && (s.v[728] != 0.0)) {
                s.store_scalar(370, 0.1);
            }
            s.v[729] = if (s.v[370] < (-0.1)) { 1.0 } else { 0.0 };
            if (((((!(s.v[725] != 0.0)) && (s.v[726] != 0.0)) && (!(s.v[727] != 0.0))) && (!(s.v[728] != 0.0))) && (s.v[729] != 0.0)) {
                s.store_scalar(370, (-0.1));
            }
            if ((!(s.v[725] != 0.0)) && (s.v[726] != 0.0)) {
                s.store_add(55, 55, 370);
            }
            if ((!(s.v[725] != 0.0)) && (s.v[726] != 0.0)) {
                s.store_offset(62, 62, 1.0);
            }
        }

        s.v[730] = if (s.v[52] < (s.v[54] + s.v[55])) { 1.0 } else { 0.0 };

        if ((!(s.v[725] != 0.0)) && (s.v[730] != 0.0)) {
            s.store_scalar(39, 1.0);
        }

        if ((!(s.v[725] != 0.0)) && (s.v[730] != 0.0)) {
            s.store_scalar(292, (-1.0));
        }

        if ((!(s.v[725] != 0.0)) && (s.v[730] != 0.0)) {
            s.copy_ad(332, 334);
        }

        if ((!(s.v[725] != 0.0)) && (s.v[730] != 0.0)) {
            s.store_sqrt_ad(279, A::div(A::scale(s.ad_value(332), ((2.0 * 1.034943e-10) / 1.6021918e-19)), s.ad_value(471)));
        }

        if ((!(s.v[725] != 0.0)) && (s.v[730] != 0.0)) {
            s.store_scale_ad(278, A::square(s.ad_value(439)), (s.v[293] * s.v[293]));
        }

        s.v[731] = if ((s.v[345] + s.v[279]) < p.p227) { 1.0 } else { 0.0 };

        if (((!(s.v[725] != 0.0)) && (s.v[730] != 0.0)) && (s.v[731] != 0.0)) {
            s.store_sub_from_scalar(279, (10.0 * 2.220446049250313e-16), 440);
        }

        if (((!(s.v[725] != 0.0)) && (s.v[730] != 0.0)) && (s.v[731] != 0.0)) {
            s.store_sub_ad(280, A::mul(A::add(A::scale(s.ad_value(279), 2.0), A::mul(s.ad_value(278), s.ad_value(120))), A::add(A::scale(s.ad_value(279), 2.0), A::mul(s.ad_value(278), s.ad_value(120)))), A::scale(A::add(A::square(s.ad_value(279)), s.ad_value(278)), 4.0));
        }

        if (((!(s.v[725] != 0.0)) && (s.v[730] != 0.0)) && (s.v[731] != 0.0)) {
            s.store_ad(280, &{
                if (s.v[280] >= (10.0 * 2.220446049250313e-16)) {
                    s.ad_value(280)
                } else {
                    A::constant((10.0 * 2.220446049250313e-16))
                }
            });
        }

        if (((!(s.v[725] != 0.0)) && (s.v[730] != 0.0)) && (s.v[731] != 0.0)) {
            s.store_sqrt(280, 280);
        }

        if (((!(s.v[725] != 0.0)) && (s.v[730] != 0.0)) && (s.v[731] != 0.0)) {
            s.store_add_ad(281, A::scale(s.ad_value(279), 2.0), A::mul(s.ad_value(278), s.ad_value(120)));
        }

        if (((!(s.v[725] != 0.0)) && (s.v[730] != 0.0)) && (s.v[731] != 0.0)) {
            s.store_scaled_sub(324, 281, 280, 0.5);
        }

        if (((!(s.v[725] != 0.0)) && (s.v[730] != 0.0)) && (s.v[731] != 0.0)) {
            s.store_div_ad(325, A::ln(A::div(A::div(A::square(s.ad_value(279)), s.ad_value(278)), s.ad_value(143))), A::add(s.ad_value(120), A::div_from_scalar(2.0, s.ad_value(279))));
        }

        s.v[732] = if (s.v[324] < s.v[326]) { 1.0 } else { 0.0 };

        if ((((!(s.v[725] != 0.0)) && (s.v[730] != 0.0)) && (s.v[731] != 0.0)) && (s.v[732] != 0.0)) {
            s.copy_ad(307, 324);
        }

        if ((((!(s.v[725] != 0.0)) && (s.v[730] != 0.0)) && (s.v[731] != 0.0)) && (!(s.v[732] != 0.0))) {
            s.store_offset_ad(638, A::sub(s.ad_value(325), s.ad_value(324)), (-0.0008));
        }

        if ((((!(s.v[725] != 0.0)) && (s.v[730] != 0.0)) && (s.v[731] != 0.0)) && (!(s.v[732] != 0.0))) {
            s.store_scale(639, 325, (4.0 * 0.0008));
        }

        if ((((!(s.v[725] != 0.0)) && (s.v[730] != 0.0)) && (s.v[731] != 0.0)) && (!(s.v[732] != 0.0))) {
            s.store_ad(639, &{
                if (s.v[639] > 0.0) {
                    s.ad_value(639)
                } else {
                    A::neg(s.ad_value(639))
                }
            });
        }

        if ((((!(s.v[725] != 0.0)) && (s.v[730] != 0.0)) && (s.v[731] != 0.0)) && (!(s.v[732] != 0.0))) {
            s.store_sqrt_ad(639, A::add(A::square(s.ad_value(638)), s.ad_value(639)));
        }

        if ((((!(s.v[725] != 0.0)) && (s.v[730] != 0.0)) && (s.v[731] != 0.0)) && (!(s.v[732] != 0.0))) {
            s.store_scale_ad(279, A::offset(A::div(s.ad_value(638), s.ad_value(639)), 1.0), 0.5);
        }

        if ((((!(s.v[725] != 0.0)) && (s.v[730] != 0.0)) && (s.v[731] != 0.0)) && (!(s.v[732] != 0.0))) {
            s.store_sub_ad_rhs(307, 325, A::scale(A::add(s.ad_value(638), s.ad_value(639)), 0.5));
        }

        if (((!(s.v[725] != 0.0)) && (s.v[730] != 0.0)) && (!(s.v[731] != 0.0))) {
            s.store_neg_ad(279, A::sub(A::sub(s.ad_value(440), s.ad_value(305)), A::scale(A::scale(s.ad_value(296), 0.5), (p.p227 * 9662367879.197212))));
        }

        if (((!(s.v[725] != 0.0)) && (s.v[730] != 0.0)) && (!(s.v[731] != 0.0))) {
            s.store_sub_ad(280, A::mul(A::add(A::scale(s.ad_value(279), 2.0), A::mul(s.ad_value(278), s.ad_value(120))), A::add(A::scale(s.ad_value(279), 2.0), A::mul(s.ad_value(278), s.ad_value(120)))), A::scale(A::add(A::square(s.ad_value(279)), s.ad_value(278)), 4.0));
        }

        if (((!(s.v[725] != 0.0)) && (s.v[730] != 0.0)) && (!(s.v[731] != 0.0))) {
            s.store_ad(280, &{
                if (s.v[280] >= (10.0 * 2.220446049250313e-16)) {
                    s.ad_value(280)
                } else {
                    A::constant((10.0 * 2.220446049250313e-16))
                }
            });
        }

        if (((!(s.v[725] != 0.0)) && (s.v[730] != 0.0)) && (!(s.v[731] != 0.0))) {
            s.store_sqrt(280, 280);
        }

        if (((!(s.v[725] != 0.0)) && (s.v[730] != 0.0)) && (!(s.v[731] != 0.0))) {
            s.store_add_ad(281, A::scale(s.ad_value(279), 2.0), A::mul(s.ad_value(278), s.ad_value(120)));
        }

        if (((!(s.v[725] != 0.0)) && (s.v[730] != 0.0)) && (!(s.v[731] != 0.0))) {
            s.store_scaled_sub(324, 281, 280, 0.5);
        }

        if (((!(s.v[725] != 0.0)) && (s.v[730] != 0.0)) && (!(s.v[731] != 0.0))) {
            s.store_div_ad(325, A::ln(A::div(A::div(A::square(s.ad_value(279)), s.ad_value(278)), s.ad_value(143))), A::add(s.ad_value(120), A::div_from_scalar(2.0, s.ad_value(279))));
        }

        s.v[733] = if (s.v[324] < s.v[326]) { 1.0 } else { 0.0 };

        if ((((!(s.v[725] != 0.0)) && (s.v[730] != 0.0)) && (!(s.v[731] != 0.0))) && (s.v[733] != 0.0)) {
            s.copy_ad(307, 324);
        }

        if ((((!(s.v[725] != 0.0)) && (s.v[730] != 0.0)) && (!(s.v[731] != 0.0))) && (!(s.v[733] != 0.0))) {
            s.store_offset_ad(638, A::sub(s.ad_value(325), s.ad_value(324)), (-0.0008));
        }

        if ((((!(s.v[725] != 0.0)) && (s.v[730] != 0.0)) && (!(s.v[731] != 0.0))) && (!(s.v[733] != 0.0))) {
            s.store_scale(639, 325, (4.0 * 0.0008));
        }

        if ((((!(s.v[725] != 0.0)) && (s.v[730] != 0.0)) && (!(s.v[731] != 0.0))) && (!(s.v[733] != 0.0))) {
            s.store_ad(639, &{
                if (s.v[639] > 0.0) {
                    s.ad_value(639)
                } else {
                    A::neg(s.ad_value(639))
                }
            });
        }

        if ((((!(s.v[725] != 0.0)) && (s.v[730] != 0.0)) && (!(s.v[731] != 0.0))) && (!(s.v[733] != 0.0))) {
            s.store_sqrt_ad(639, A::add(A::square(s.ad_value(638)), s.ad_value(639)));
        }

        if ((((!(s.v[725] != 0.0)) && (s.v[730] != 0.0)) && (!(s.v[731] != 0.0))) && (!(s.v[733] != 0.0))) {
            s.store_scale_ad(279, A::offset(A::div(s.ad_value(638), s.ad_value(639)), 1.0), 0.5);
        }

        if ((((!(s.v[725] != 0.0)) && (s.v[730] != 0.0)) && (!(s.v[731] != 0.0))) && (!(s.v[733] != 0.0))) {
            s.store_sub_ad_rhs(307, 325, A::scale(A::add(s.ad_value(638), s.ad_value(639)), 0.5));
        }

        if ((!(s.v[725] != 0.0)) && (s.v[730] != 0.0)) {
            s.store_sqrt_ad(279, A::div(A::scale(s.ad_value(332), ((2.0 * 1.034943e-10) / 1.6021918e-19)), s.ad_value(471)));
        }

        s.v[734] = if ((s.v[345] + s.v[279]) < p.p227) { 1.0 } else { 0.0 };

        if (((!(s.v[725] != 0.0)) && (s.v[730] != 0.0)) && (s.v[734] != 0.0)) {
            s.store_scalar(62, 0.0);
        }

        let mut assign7560_loop_guard: usize = 0;
        while {
            let assign7560_cond_e6174: f64 = if ((((!(s.v[725] != 0.0)) && (s.v[730] != 0.0)) && (s.v[734] != 0.0)) && (s.v[62] < s.v[28])) { 1.0 } else { 0.0 };
            assign7560_cond_e6174 != 0.0
        } {
            assign7560_loop_guard += 1;
            assert!(assign7560_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((!(s.v[725] != 0.0)) && (s.v[730] != 0.0)) && (s.v[734] != 0.0)) {
                s.copy_ad(279, 439);
            }
            if (((!(s.v[725] != 0.0)) && (s.v[730] != 0.0)) && (s.v[734] != 0.0)) {
                s.store_mul(280, 120, 307);
            }
            if (((!(s.v[725] != 0.0)) && (s.v[730] != 0.0)) && (s.v[734] != 0.0)) {
                s.store_exp_ad(281, A::neg(s.ad_value(280)));
            }
            s.v[735] = if (s.v[307] > 1e-8) { 1.0 } else { 0.0 };
            if ((((!(s.v[725] != 0.0)) && (s.v[730] != 0.0)) && (s.v[734] != 0.0)) && (s.v[735] != 0.0)) {
                s.store_exp_ad(278, A::mul(s.ad_value(120), s.ad_value(307)));
            }
            if ((((!(s.v[725] != 0.0)) && (s.v[730] != 0.0)) && (s.v[734] != 0.0)) && (s.v[735] != 0.0)) {
                s.store_mul_ad(282, A::neg(s.ad_value(279)), A::sqrt(A::add(A::offset(A::add(s.ad_value(281), s.ad_value(280)), (-1.0)), A::mul(s.ad_value(143), A::offset(s.ad_value(278), (-1.0))))));
            }
            if ((((!(s.v[725] != 0.0)) && (s.v[730] != 0.0)) && (s.v[734] != 0.0)) && (s.v[735] != 0.0)) {
                s.store_mul_ad(283, A::div(s.ad_value(438), s.ad_value(282)), A::add(A::sub_from_scalar(1.0, s.ad_value(281)), A::mul(s.ad_value(143), s.ad_value(278))));
            }
            s.v[736] = if (s.v[307] < (-1e-8)) { 1.0 } else { 0.0 };
            if (((((!(s.v[725] != 0.0)) && (s.v[730] != 0.0)) && (s.v[734] != 0.0)) && (!(s.v[735] != 0.0))) && (s.v[736] != 0.0)) {
                s.store_mul_ad_rhs(282, 279, A::sqrt(A::offset(A::add(s.ad_value(281), s.ad_value(280)), (-1.0))));
            }
            if (((((!(s.v[725] != 0.0)) && (s.v[730] != 0.0)) && (s.v[734] != 0.0)) && (!(s.v[735] != 0.0))) && (s.v[736] != 0.0)) {
                s.store_mul_ad(283, A::div(s.ad_value(438), s.ad_value(282)), A::sub_from_scalar(1.0, s.ad_value(281)));
            }
            if (((((!(s.v[725] != 0.0)) && (s.v[730] != 0.0)) && (s.v[734] != 0.0)) && (!(s.v[735] != 0.0))) && (!(s.v[736] != 0.0))) {
                s.store_mul_ad_lhs(282, A::mul(A::neg(A::sqrt(A::div(s.ad_value(438), s.ad_value(120)))), s.ad_value(120)), 307);
            }
            if (((((!(s.v[725] != 0.0)) && (s.v[730] != 0.0)) && (s.v[734] != 0.0)) && (!(s.v[735] != 0.0))) && (!(s.v[736] != 0.0))) {
                s.store_neg_ad(283, A::sqrt(A::mul(s.ad_value(438), s.ad_value(120))));
            }
            if (((!(s.v[725] != 0.0)) && (s.v[730] != 0.0)) && (s.v[734] != 0.0)) {
                s.store_sqrt_ad(639, A::offset(A::square(s.ad_value(282)), ((4.0 * 1e-10) * 1e-10)));
            }
            if (((!(s.v[725] != 0.0)) && (s.v[730] != 0.0)) && (s.v[734] != 0.0)) {
                s.store_scale_ad(285, A::offset(A::div(s.ad_value(282), s.ad_value(639)), 1.0), 0.5);
            }
            if (((!(s.v[725] != 0.0)) && (s.v[730] != 0.0)) && (s.v[734] != 0.0)) {
                s.store_offset_ad(284, A::scale(A::add(s.ad_value(282), s.ad_value(639)), 0.5), (1e-10 * 1e-10));
            }
            s.v[737] = if (s.v[284] < 0.0) { 1.0 } else { 0.0 };
            if ((((!(s.v[725] != 0.0)) && (s.v[730] != 0.0)) && (s.v[734] != 0.0)) && (s.v[737] != 0.0)) {
                s.store_scalar(284, 0.0);
            }
            if ((((!(s.v[725] != 0.0)) && (s.v[730] != 0.0)) && (s.v[734] != 0.0)) && (s.v[737] != 0.0)) {
                s.store_scalar(285, 0.0);
            }
            if (((!(s.v[725] != 0.0)) && (s.v[730] != 0.0)) && (s.v[734] != 0.0)) {
                s.store_offset_ad(638, A::sub(A::neg(s.ad_value(296)), s.ad_value(284)), (-1e-13));
            }
            if (((!(s.v[725] != 0.0)) && (s.v[730] != 0.0)) && (s.v[734] != 0.0)) {
                s.store_scale_ad(639, A::neg(s.ad_value(296)), (4.0 * 1e-13));
            }
            if (((!(s.v[725] != 0.0)) && (s.v[730] != 0.0)) && (s.v[734] != 0.0)) {
                s.store_ad(639, &{
                    if (s.v[639] > 0.0) {
                        s.ad_value(639)
                    } else {
                        A::neg(s.ad_value(639))
                    }
                });
            }
            if (((!(s.v[725] != 0.0)) && (s.v[730] != 0.0)) && (s.v[734] != 0.0)) {
                s.store_sqrt_ad(639, A::add(A::square(s.ad_value(638)), s.ad_value(639)));
            }
            if (((!(s.v[725] != 0.0)) && (s.v[730] != 0.0)) && (s.v[734] != 0.0)) {
                s.store_scale_ad(286, A::offset(A::div(s.ad_value(638), s.ad_value(639)), 1.0), 0.5);
            }
            if (((!(s.v[725] != 0.0)) && (s.v[730] != 0.0)) && (s.v[734] != 0.0)) {
                s.store_sub_ad(284, A::neg(s.ad_value(296)), A::scale(A::add(s.ad_value(638), s.ad_value(639)), 0.5));
            }
            if (((!(s.v[725] != 0.0)) && (s.v[730] != 0.0)) && (s.v[734] != 0.0)) {
                s.store_mul_ad_rhs(285, 285, A::mul(s.ad_value(283), s.ad_value(286)));
            }
            if (((!(s.v[725] != 0.0)) && (s.v[730] != 0.0)) && (s.v[734] != 0.0)) {
                s.store_div_ad_lhs(332, A::scale(A::scale(A::scale(A::square(s.ad_value(284)), 0.5), 9662367879.197212), 6.241449993689894e18), 471);
            }
            if (((!(s.v[725] != 0.0)) && (s.v[730] != 0.0)) && (s.v[734] != 0.0)) {
                s.store_div_ad_lhs(333, A::mul(A::scale(s.ad_value(332), 2.0), s.ad_value(285)), 284);
            }
            if (((!(s.v[725] != 0.0)) && (s.v[730] != 0.0)) && (s.v[734] != 0.0)) {
                s.store_sub_ad_rhs(284, 307, A::div(A::add(A::sub(A::sub(A::scale(s.ad_value(282), 1.0 / (s.v[294])), s.ad_value(307)), s.ad_value(440)), s.ad_value(332)), A::add(A::offset(A::scale(s.ad_value(283), 1.0 / (s.v[294])), (-1.0)), s.ad_value(333))));
            }
            s.v[738] = if ((((s.v[284] - s.v[307])) as f64).abs() < 0.001) { 1.0 } else { 0.0 };
            if ((((!(s.v[725] != 0.0)) && (s.v[730] != 0.0)) && (s.v[734] != 0.0)) && (s.v[738] != 0.0)) {
                s.store_scalar(62, s.v[28]);
            }
            if (((!(s.v[725] != 0.0)) && (s.v[730] != 0.0)) && (s.v[734] != 0.0)) {
                s.copy_ad(307, 284);
            }
            if (((!(s.v[725] != 0.0)) && (s.v[730] != 0.0)) && (s.v[734] != 0.0)) {
                s.copy_ad(312, 282);
            }
            if (((!(s.v[725] != 0.0)) && (s.v[730] != 0.0)) && (s.v[734] != 0.0)) {
                s.store_offset(62, 62, 1.0);
            }
        }

        if (((!(s.v[725] != 0.0)) && (s.v[730] != 0.0)) && (!(s.v[734] != 0.0))) {
            s.store_scalar(62, 0.0);
        }

        let mut assign7580_loop_guard: usize = 0;
        while {
            let assign7580_cond_e6663: f64 = if ((((!(s.v[725] != 0.0)) && (s.v[730] != 0.0)) && (!(s.v[734] != 0.0))) && (s.v[62] < s.v[28])) { 1.0 } else { 0.0 };
            assign7580_cond_e6663 != 0.0
        } {
            assign7580_loop_guard += 1;
            assert!(assign7580_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((!(s.v[725] != 0.0)) && (s.v[730] != 0.0)) && (!(s.v[734] != 0.0))) {
                s.copy_ad(279, 439);
            }
            if (((!(s.v[725] != 0.0)) && (s.v[730] != 0.0)) && (!(s.v[734] != 0.0))) {
                s.store_mul(280, 120, 307);
            }
            if (((!(s.v[725] != 0.0)) && (s.v[730] != 0.0)) && (!(s.v[734] != 0.0))) {
                s.store_exp_ad(281, A::neg(s.ad_value(280)));
            }
            s.v[739] = if (s.v[307] > 1e-8) { 1.0 } else { 0.0 };
            if ((((!(s.v[725] != 0.0)) && (s.v[730] != 0.0)) && (!(s.v[734] != 0.0))) && (s.v[739] != 0.0)) {
                s.store_exp_ad(278, A::mul(s.ad_value(120), s.ad_value(307)));
            }
            if ((((!(s.v[725] != 0.0)) && (s.v[730] != 0.0)) && (!(s.v[734] != 0.0))) && (s.v[739] != 0.0)) {
                s.store_mul_ad(282, A::neg(s.ad_value(279)), A::sqrt(A::add(A::offset(A::add(s.ad_value(281), s.ad_value(280)), (-1.0)), A::mul(s.ad_value(143), A::offset(s.ad_value(278), (-1.0))))));
            }
            if ((((!(s.v[725] != 0.0)) && (s.v[730] != 0.0)) && (!(s.v[734] != 0.0))) && (s.v[739] != 0.0)) {
                s.store_mul_ad(283, A::div(s.ad_value(438), s.ad_value(282)), A::add(A::sub_from_scalar(1.0, s.ad_value(281)), A::mul(s.ad_value(143), s.ad_value(278))));
            }
            s.v[740] = if (s.v[307] < (-1e-8)) { 1.0 } else { 0.0 };
            if (((((!(s.v[725] != 0.0)) && (s.v[730] != 0.0)) && (!(s.v[734] != 0.0))) && (!(s.v[739] != 0.0))) && (s.v[740] != 0.0)) {
                s.store_mul_ad_rhs(282, 279, A::sqrt(A::offset(A::add(s.ad_value(281), s.ad_value(280)), (-1.0))));
            }
            if (((((!(s.v[725] != 0.0)) && (s.v[730] != 0.0)) && (!(s.v[734] != 0.0))) && (!(s.v[739] != 0.0))) && (s.v[740] != 0.0)) {
                s.store_mul_ad(283, A::div(s.ad_value(438), s.ad_value(282)), A::sub_from_scalar(1.0, s.ad_value(281)));
            }
            if (((((!(s.v[725] != 0.0)) && (s.v[730] != 0.0)) && (!(s.v[734] != 0.0))) && (!(s.v[739] != 0.0))) && (!(s.v[740] != 0.0))) {
                s.store_mul_ad_lhs(282, A::mul(A::neg(A::sqrt(A::div(s.ad_value(438), s.ad_value(120)))), s.ad_value(120)), 307);
            }
            if (((((!(s.v[725] != 0.0)) && (s.v[730] != 0.0)) && (!(s.v[734] != 0.0))) && (!(s.v[739] != 0.0))) && (!(s.v[740] != 0.0))) {
                s.store_neg_ad(283, A::sqrt(A::mul(s.ad_value(438), s.ad_value(120))));
            }
            if (((!(s.v[725] != 0.0)) && (s.v[730] != 0.0)) && (!(s.v[734] != 0.0))) {
                s.store_sqrt_ad(639, A::offset(A::square(s.ad_value(282)), ((4.0 * 1e-10) * 1e-10)));
            }
            if (((!(s.v[725] != 0.0)) && (s.v[730] != 0.0)) && (!(s.v[734] != 0.0))) {
                s.store_scale_ad(285, A::offset(A::div(s.ad_value(282), s.ad_value(639)), 1.0), 0.5);
            }
            if (((!(s.v[725] != 0.0)) && (s.v[730] != 0.0)) && (!(s.v[734] != 0.0))) {
                s.store_offset_ad(284, A::scale(A::add(s.ad_value(282), s.ad_value(639)), 0.5), (1e-10 * 1e-10));
            }
            s.v[741] = if (s.v[284] < 0.0) { 1.0 } else { 0.0 };
            if ((((!(s.v[725] != 0.0)) && (s.v[730] != 0.0)) && (!(s.v[734] != 0.0))) && (s.v[741] != 0.0)) {
                s.store_scalar(284, 0.0);
            }
            if ((((!(s.v[725] != 0.0)) && (s.v[730] != 0.0)) && (!(s.v[734] != 0.0))) && (s.v[741] != 0.0)) {
                s.store_scalar(285, 0.0);
            }
            if (((!(s.v[725] != 0.0)) && (s.v[730] != 0.0)) && (!(s.v[734] != 0.0))) {
                s.store_offset_ad(638, A::sub(A::neg(s.ad_value(296)), s.ad_value(284)), (-1e-13));
            }
            if (((!(s.v[725] != 0.0)) && (s.v[730] != 0.0)) && (!(s.v[734] != 0.0))) {
                s.store_scale_ad(639, A::neg(s.ad_value(296)), (4.0 * 1e-13));
            }
            if (((!(s.v[725] != 0.0)) && (s.v[730] != 0.0)) && (!(s.v[734] != 0.0))) {
                s.store_ad(639, &{
                    if (s.v[639] > 0.0) {
                        s.ad_value(639)
                    } else {
                        A::neg(s.ad_value(639))
                    }
                });
            }
            if (((!(s.v[725] != 0.0)) && (s.v[730] != 0.0)) && (!(s.v[734] != 0.0))) {
                s.store_sqrt_ad(639, A::add(A::square(s.ad_value(638)), s.ad_value(639)));
            }
            if (((!(s.v[725] != 0.0)) && (s.v[730] != 0.0)) && (!(s.v[734] != 0.0))) {
                s.store_scale_ad(286, A::offset(A::div(s.ad_value(638), s.ad_value(639)), 1.0), 0.5);
            }
            if (((!(s.v[725] != 0.0)) && (s.v[730] != 0.0)) && (!(s.v[734] != 0.0))) {
                s.store_sub_ad(284, A::neg(s.ad_value(296)), A::scale(A::add(s.ad_value(638), s.ad_value(639)), 0.5));
            }
            if (((!(s.v[725] != 0.0)) && (s.v[730] != 0.0)) && (!(s.v[734] != 0.0))) {
                s.store_mul_ad_rhs(285, 285, A::mul(s.ad_value(283), s.ad_value(286)));
            }
            if (((!(s.v[725] != 0.0)) && (s.v[730] != 0.0)) && (!(s.v[734] != 0.0))) {
                s.store_div_ad_lhs(332, A::scale(A::scale(A::scale(A::square(s.ad_value(284)), 0.5), 9662367879.197212), 6.241449993689894e18), 471);
            }
            if (((!(s.v[725] != 0.0)) && (s.v[730] != 0.0)) && (!(s.v[734] != 0.0))) {
                s.store_div_ad_lhs(333, A::mul(A::scale(s.ad_value(332), 2.0), s.ad_value(285)), 284);
            }
            if (((!(s.v[725] != 0.0)) && (s.v[730] != 0.0)) && (!(s.v[734] != 0.0))) {
                let assign7580_body27_ad_e7124: A = A::div(A::add(A::sub(A::add(A::add(A::sub(s.ad_value(305), s.ad_value(307)), A::scale(s.ad_value(282), 1.0 / (s.v[294]))), A::scale(A::add(s.ad_value(282), A::scale(s.ad_value(296), 0.5)), (p.p227 * 9662367879.197212))), s.ad_value(440)), s.ad_value(332)), A::add(A::add(A::offset(A::scale(s.ad_value(283), 1.0 / (s.v[294])), (-1.0)), A::scale(s.ad_value(283), (p.p227 * 9662367879.197212))), s.ad_value(333)));
                s.store_sub_ad_rhs(284, 307, assign7580_body27_ad_e7124);
            }
            s.v[742] = if ((((s.v[284] - s.v[307])) as f64).abs() < 0.001) { 1.0 } else { 0.0 };
            if ((((!(s.v[725] != 0.0)) && (s.v[730] != 0.0)) && (!(s.v[734] != 0.0))) && (s.v[742] != 0.0)) {
                s.store_scalar(62, s.v[28]);
            }
            if (((!(s.v[725] != 0.0)) && (s.v[730] != 0.0)) && (!(s.v[734] != 0.0))) {
                s.copy_ad(307, 284);
            }
            if (((!(s.v[725] != 0.0)) && (s.v[730] != 0.0)) && (!(s.v[734] != 0.0))) {
                s.copy_ad(312, 282);
            }
            if (((!(s.v[725] != 0.0)) && (s.v[730] != 0.0)) && (!(s.v[734] != 0.0))) {
                s.store_offset(62, 62, 1.0);
            }
        }

        if ((!(s.v[725] != 0.0)) && (s.v[730] != 0.0)) {
            s.store_add(307, 440, 307);
        }

        if ((!(s.v[725] != 0.0)) && (s.v[730] != 0.0)) {
            s.store_sub_ad_rhs(306, 307, A::scale(s.ad_value(312), 1.0 / (s.v[294])));
        }

        if (!(s.v[725] != 0.0)) {
            s.store_offset_ad(290, A::div(A::scale(A::offset(A::mul(s.ad_value(120), A::sub(s.ad_value(76), s.ad_value(50))), (-1.0)), 4.0), A::mul(s.ad_value(145), s.ad_value(121))), 1.0);
        }

        if (!(s.v[725] != 0.0)) {
            s.store_ad(290, &{
                if (s.v[290] >= (10.0 * 2.220446049250313e-16)) {
                    s.ad_value(290)
                } else {
                    A::constant((10.0 * 2.220446049250313e-16))
                }
            });
        }

        if (!(s.v[725] != 0.0)) {
            s.store_add_ad_rhs(319, 76, A::mul(A::scale(A::mul(s.ad_value(145), s.ad_value(120)), 0.5), A::sub_from_scalar(1.0, A::sqrt(s.ad_value(290)))));
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
        if (!(s.v[725] != 0.0)) {
            s.store_div_from_scalar(278, 1.0, 270);
        }

        if (!(s.v[725] != 0.0)) {
            s.store_scalar(279, (p.p227 / 1.034943e-10));
        }

        if (!(s.v[725] != 0.0)) {
            s.store_scalar(280, (1.0 / s.v[294]));
        }

        if (!(s.v[725] != 0.0)) {
            s.store_div_from_scalar_ad(281, 1.0, A::add(A::add(s.ad_value(278), s.ad_value(279)), s.ad_value(280)));
        }

        s.v[743] = if ((s.v[52] - s.v[327]) <= s.v[78]) { 1.0 } else { 0.0 };

        if ((!(s.v[725] != 0.0)) && (s.v[743] != 0.0)) {
            s.store_ad(283, &{
                if (s.v[319] > 0.0) {
                    A::sqrt(A::mul(A::scale(s.ad_value(471), (1.6021918e-19 * (2.0 * 1.034943e-10))), s.ad_value(319)))
                } else {
                    A::constant(0.0)
                }
            });
        }

        if ((!(s.v[725] != 0.0)) && (s.v[743] != 0.0)) {
            s.store_ad(283, &{
                if (s.v[296] <= s.v[283]) {
                    s.ad_value(296)
                } else {
                    s.ad_value(283)
                }
            });
        }

        if ((!(s.v[725] != 0.0)) && (s.v[743] != 0.0)) {
            s.store_mul_ad_rhs(282, 281, A::add(A::sub(s.ad_value(76), s.ad_value(440)), A::mul(A::add(s.ad_value(280), A::scale(s.ad_value(279), 0.5)), A::neg(s.ad_value(283)))));
        }

        if ((!(s.v[725] != 0.0)) && (!(s.v[743] != 0.0))) {
            s.store_mul_ad_rhs(282, 281, A::add(A::sub(s.ad_value(76), s.ad_value(440)), A::mul(A::add(s.ad_value(280), A::scale(s.ad_value(279), 0.5)), A::neg(s.ad_value(296)))));
        }

        if (!(s.v[725] != 0.0)) {
            s.store_sub_ad_rhs(319, 76, A::div(s.ad_value(282), s.ad_value(270)));
        }

        if (!(s.v[725] != 0.0)) {
            s.copy_ad(321, 319);
        }

        s.v[744] = if ((s.v[52] - s.v[327]) > s.v[78]) { 1.0 } else { 0.0 };

        if ((!(s.v[725] != 0.0)) && (s.v[744] != 0.0)) {
            s.store_div_ad_lhs(279, A::div_from_scalar(1.0, s.ad_value(142)), 381);
        }

        if ((!(s.v[725] != 0.0)) && (s.v[744] != 0.0)) {
            s.store_mul_ad(280, A::mul(s.ad_value(279), A::sub(s.ad_value(76), s.ad_value(327))), A::sub(s.ad_value(76), s.ad_value(327)));
        }

        if ((!(s.v[725] != 0.0)) && (s.v[744] != 0.0)) {
            s.store_add_ad_rhs(281, 120, A::div_from_scalar(2.0, A::sub(s.ad_value(76), s.ad_value(327))));
        }

        if ((!(s.v[725] != 0.0)) && (s.v[744] != 0.0)) {
            s.store_div_ad_lhs(320, A::ln(s.ad_value(280)), 281);
        }

        s.v[745] = if ((s.v[319] > (s.v[320] - 0.15)) && (0.15 >= 0.0)) { 1.0 } else { 0.0 };

        if (((!(s.v[725] != 0.0)) && (s.v[744] != 0.0)) && (s.v[745] != 0.0)) {
            s.store_offset_ad(638, A::sub(s.ad_value(319), s.ad_value(320)), 0.15);
        }

        if (((!(s.v[725] != 0.0)) && (s.v[744] != 0.0)) && (s.v[745] != 0.0)) {
            s.store_square(642, 638);
        }

        if (((!(s.v[725] != 0.0)) && (s.v[744] != 0.0)) && (s.v[745] != 0.0)) {
            s.store_scalar(643, (0.15 * 0.15));
        }

        if (((!(s.v[725] != 0.0)) && (s.v[744] != 0.0)) && (s.v[745] != 0.0)) {
            s.store_scalar(644, 1.0);
        }

        if (((!(s.v[725] != 0.0)) && (s.v[744] != 0.0)) && (s.v[745] != 0.0)) {
            s.store_scalar(645, 1.0);
        }

        if (((!(s.v[725] != 0.0)) && (s.v[744] != 0.0)) && (s.v[745] != 0.0)) {
            s.store_scalar(647, 0.0);
        }

        if (((!(s.v[725] != 0.0)) && (s.v[744] != 0.0)) && (s.v[745] != 0.0)) {
            s.store_scalar(648, 0.0);
        }

        if (((!(s.v[725] != 0.0)) && (s.v[744] != 0.0)) && (s.v[745] != 0.0)) {
            s.store_scalar(220, 0.0);
        }

        if (((!(s.v[725] != 0.0)) && (s.v[744] != 0.0)) && (s.v[745] != 0.0)) {
            s.store_scalar(646, 0.0);
        }

        if (((!(s.v[725] != 0.0)) && (s.v[744] != 0.0)) && (s.v[745] != 0.0)) {
            s.store_mul(644, 644, 642);
        }

        if (((!(s.v[725] != 0.0)) && (s.v[744] != 0.0)) && (s.v[745] != 0.0)) {
            s.store_mul(645, 645, 643);
        }

        if (((!(s.v[725] != 0.0)) && (s.v[744] != 0.0)) && (s.v[745] != 0.0)) {
            s.store_add(220, 644, 645);
        }

        if (((!(s.v[725] != 0.0)) && (s.v[744] != 0.0)) && (s.v[745] != 0.0)) {
            s.copy_ad(646, 220);
        }

        s.v[746] = if ((((1.0 == 1.0) || (1.0 == 2.0)) || (1.0 == 4.0)) || (1.0 == 8.0)) { 1.0 } else { 0.0 };

        s.v[747] = if (1.0 == 1.0) { 1.0 } else { 0.0 };

        if (((((!(s.v[725] != 0.0)) && (s.v[744] != 0.0)) && (s.v[745] != 0.0)) && (s.v[746] != 0.0)) && (s.v[747] != 0.0)) {
            s.store_scalar(648, 1.0);
        }

        s.v[748] = if (1.0 == 2.0) { 1.0 } else { 0.0 };

        if ((((((!(s.v[725] != 0.0)) && (s.v[744] != 0.0)) && (s.v[745] != 0.0)) && (s.v[746] != 0.0)) && (!(s.v[747] != 0.0))) && (s.v[748] != 0.0)) {
            s.store_scalar(648, 2.0);
        }

        s.v[749] = if (1.0 == 4.0) { 1.0 } else { 0.0 };

        if (((((((!(s.v[725] != 0.0)) && (s.v[744] != 0.0)) && (s.v[745] != 0.0)) && (s.v[746] != 0.0)) && (!(s.v[747] != 0.0))) && (!(s.v[748] != 0.0))) && (s.v[749] != 0.0)) {
            s.store_scalar(648, 3.0);
        }

        s.v[750] = if (1.0 == 8.0) { 1.0 } else { 0.0 };

        if ((((((((!(s.v[725] != 0.0)) && (s.v[744] != 0.0)) && (s.v[745] != 0.0)) && (s.v[746] != 0.0)) && (!(s.v[747] != 0.0))) && (!(s.v[748] != 0.0))) && (!(s.v[749] != 0.0))) && (s.v[750] != 0.0)) {
            s.store_scalar(648, 4.0);
        }

        if ((((!(s.v[725] != 0.0)) && (s.v[744] != 0.0)) && (s.v[745] != 0.0)) && (s.v[746] != 0.0)) {
            s.store_scalar(647, 0.0);
        }

        let mut assign8040_loop_guard: usize = 0;
        while {
            let assign8040_cond_e7685: f64 = if (((((!(s.v[725] != 0.0)) && (s.v[744] != 0.0)) && (s.v[745] != 0.0)) && (s.v[746] != 0.0)) && (s.v[647] < s.v[648])) { 1.0 } else { 0.0 };
            assign8040_cond_e7685 != 0.0
        } {
            assign8040_loop_guard += 1;
            assert!(assign8040_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((!(s.v[725] != 0.0)) && (s.v[744] != 0.0)) && (s.v[745] != 0.0)) && (s.v[746] != 0.0)) {
                s.store_sqrt(646, 646);
            }
            if ((((!(s.v[725] != 0.0)) && (s.v[744] != 0.0)) && (s.v[745] != 0.0)) && (s.v[746] != 0.0)) {
                s.store_offset(647, 647, 1.0);
            }
        }

        if ((((!(s.v[725] != 0.0)) && (s.v[744] != 0.0)) && (s.v[745] != 0.0)) && (!(s.v[746] != 0.0))) {
            s.store_powf(646, 646, (1.0 / 2.0));
        }

        if (((!(s.v[725] != 0.0)) && (s.v[744] != 0.0)) && (s.v[745] != 0.0)) {
            s.store_div_from_scalar_ad(646, 1.0, A::offset(s.ad_value(646), 1e-50));
        }

        if (((!(s.v[725] != 0.0)) && (s.v[744] != 0.0)) && (s.v[745] != 0.0)) {
            s.store_mul_ad_lhs(637, A::scale(s.ad_value(638), 0.15), 646);
        }

        if (((!(s.v[725] != 0.0)) && (s.v[744] != 0.0)) && (s.v[745] != 0.0)) {
            s.store_div_ad(279, A::mul(A::scale(s.ad_value(645), 0.15), s.ad_value(646)), A::offset(s.ad_value(220), 1e-50));
        }

        if (((!(s.v[725] != 0.0)) && (s.v[744] != 0.0)) && (s.v[745] != 0.0)) {
            s.store_add_ad_lhs(321, A::offset(s.ad_value(320), (-0.15)), 637);
        }

        if (((!(s.v[725] != 0.0)) && (s.v[744] != 0.0)) && (s.v[745] != 0.0)) {
        }

        if (((!(s.v[725] != 0.0)) && (s.v[744] != 0.0)) && (!(s.v[745] != 0.0))) {
            s.copy_ad(321, 319);
        }

        if (((!(s.v[725] != 0.0)) && (s.v[744] != 0.0)) && (!(s.v[745] != 0.0))) {
            s.store_scalar(279, 1.0);
        }

        if (!(s.v[725] != 0.0)) {
            s.store_ad(345, &{
                if (s.v[321] > 0.0) {
                    A::sqrt(A::div(A::scale(s.ad_value(321), ((2.0 * 1.034943e-10) / 1.6021918e-19)), s.ad_value(471)))
                } else {
                    A::constant(0.0)
                }
            });
        }

        s.v[751] = if (s.v[345] < p.p227) { 1.0 } else { 0.0 };

        if ((!(s.v[725] != 0.0)) && (s.v[751] != 0.0)) {
            s.store_scalar(39, 1.0);
        }

        if ((!(s.v[725] != 0.0)) && (!(s.v[751] != 0.0))) {
            s.store_scalar(39, 2.0);
        }

        if (!(s.v[725] != 0.0)) {
            s.copy_ad(305, 321);
        }

        if (!(s.v[725] != 0.0)) {
            s.copy_ad(58, 319);
        }

        if (!(s.v[725] != 0.0)) {
            s.store_scale_ad(278, A::square(s.ad_value(439)), (s.v[293] * s.v[293]));
        }

        s.v[752] = if (s.v[39] == 1.0) { 1.0 } else { 0.0 };

        if ((!(s.v[725] != 0.0)) && (s.v[752] != 0.0)) {
            s.store_neg(279, 440);
        }

        if ((!(s.v[725] != 0.0)) && (s.v[752] != 0.0)) {
            s.store_sub_ad(280, A::mul(A::add(A::scale(s.ad_value(279), 2.0), A::mul(s.ad_value(278), s.ad_value(120))), A::add(A::scale(s.ad_value(279), 2.0), A::mul(s.ad_value(278), s.ad_value(120)))), A::scale(A::add(A::square(s.ad_value(279)), s.ad_value(278)), 4.0));
        }

        if ((!(s.v[725] != 0.0)) && (s.v[752] != 0.0)) {
            s.store_ad(280, &{
                if (s.v[280] >= (10.0 * 2.220446049250313e-16)) {
                    s.ad_value(280)
                } else {
                    A::constant((10.0 * 2.220446049250313e-16))
                }
            });
        }

        if ((!(s.v[725] != 0.0)) && (s.v[752] != 0.0)) {
            s.store_sqrt(280, 280);
        }

        if ((!(s.v[725] != 0.0)) && (s.v[752] != 0.0)) {
            s.store_add_ad(281, A::scale(s.ad_value(279), 2.0), A::mul(s.ad_value(278), s.ad_value(120)));
        }

        if ((!(s.v[725] != 0.0)) && (s.v[752] != 0.0)) {
            s.store_scaled_sub(324, 281, 280, 0.5);
        }

        if ((!(s.v[725] != 0.0)) && (s.v[752] != 0.0)) {
            s.store_div_ad(325, A::ln(A::div(A::div(A::square(s.ad_value(279)), s.ad_value(278)), s.ad_value(143))), A::add(s.ad_value(120), A::div_from_scalar(2.0, s.ad_value(279))));
        }

        s.v[753] = if (s.v[324] < s.v[326]) { 1.0 } else { 0.0 };

        if (((!(s.v[725] != 0.0)) && (s.v[752] != 0.0)) && (s.v[753] != 0.0)) {
            s.copy_ad(307, 324);
        }

        if (((!(s.v[725] != 0.0)) && (s.v[752] != 0.0)) && (!(s.v[753] != 0.0))) {
            s.store_offset_ad(638, A::sub(s.ad_value(325), s.ad_value(324)), (-0.0008));
        }

        if (((!(s.v[725] != 0.0)) && (s.v[752] != 0.0)) && (!(s.v[753] != 0.0))) {
            s.store_scale(639, 325, (4.0 * 0.0008));
        }

        if (((!(s.v[725] != 0.0)) && (s.v[752] != 0.0)) && (!(s.v[753] != 0.0))) {
            s.store_ad(639, &{
                if (s.v[639] > 0.0) {
                    s.ad_value(639)
                } else {
                    A::neg(s.ad_value(639))
                }
            });
        }

        if (((!(s.v[725] != 0.0)) && (s.v[752] != 0.0)) && (!(s.v[753] != 0.0))) {
            s.store_sqrt_ad(639, A::add(A::square(s.ad_value(638)), s.ad_value(639)));
        }

        if (((!(s.v[725] != 0.0)) && (s.v[752] != 0.0)) && (!(s.v[753] != 0.0))) {
            s.store_scale_ad(279, A::offset(A::div(s.ad_value(638), s.ad_value(639)), 1.0), 0.5);
        }

        if (((!(s.v[725] != 0.0)) && (s.v[752] != 0.0)) && (!(s.v[753] != 0.0))) {
            s.store_sub_ad_rhs(307, 325, A::scale(A::add(s.ad_value(638), s.ad_value(639)), 0.5));
        }

        if ((!(s.v[725] != 0.0)) && (!(s.v[752] != 0.0))) {
            s.store_neg_ad(279, A::sub(A::sub(s.ad_value(440), s.ad_value(305)), A::scale(A::scale(s.ad_value(296), 0.5), (p.p227 * 9662367879.197212))));
        }

        if ((!(s.v[725] != 0.0)) && (!(s.v[752] != 0.0))) {
            s.store_sub_ad(280, A::mul(A::add(A::scale(s.ad_value(279), 2.0), A::mul(s.ad_value(278), s.ad_value(120))), A::add(A::scale(s.ad_value(279), 2.0), A::mul(s.ad_value(278), s.ad_value(120)))), A::scale(A::add(A::square(s.ad_value(279)), s.ad_value(278)), 4.0));
        }

        if ((!(s.v[725] != 0.0)) && (!(s.v[752] != 0.0))) {
            s.store_ad(280, &{
                if (s.v[280] >= (10.0 * 2.220446049250313e-16)) {
                    s.ad_value(280)
                } else {
                    A::constant((10.0 * 2.220446049250313e-16))
                }
            });
        }

        if ((!(s.v[725] != 0.0)) && (!(s.v[752] != 0.0))) {
            s.store_sqrt(280, 280);
        }

        if ((!(s.v[725] != 0.0)) && (!(s.v[752] != 0.0))) {
            s.store_add_ad(281, A::scale(s.ad_value(279), 2.0), A::mul(s.ad_value(278), s.ad_value(120)));
        }

        if ((!(s.v[725] != 0.0)) && (!(s.v[752] != 0.0))) {
            s.store_scaled_sub(324, 281, 280, 0.5);
        }

        if ((!(s.v[725] != 0.0)) && (!(s.v[752] != 0.0))) {
            s.store_div_ad(325, A::ln(A::div(A::div(A::square(s.ad_value(279)), s.ad_value(278)), s.ad_value(143))), A::add(s.ad_value(120), A::div_from_scalar(2.0, s.ad_value(279))));
        }

        s.v[754] = if (s.v[324] < s.v[326]) { 1.0 } else { 0.0 };

        if (((!(s.v[725] != 0.0)) && (!(s.v[752] != 0.0))) && (s.v[754] != 0.0)) {
            s.copy_ad(307, 324);
        }

        if (((!(s.v[725] != 0.0)) && (!(s.v[752] != 0.0))) && (!(s.v[754] != 0.0))) {
            s.store_offset_ad(638, A::sub(s.ad_value(325), s.ad_value(324)), (-0.0008));
        }

        if (((!(s.v[725] != 0.0)) && (!(s.v[752] != 0.0))) && (!(s.v[754] != 0.0))) {
            s.store_scale(639, 325, (4.0 * 0.0008));
        }

        if (((!(s.v[725] != 0.0)) && (!(s.v[752] != 0.0))) && (!(s.v[754] != 0.0))) {
            s.store_ad(639, &{
                if (s.v[639] > 0.0) {
                    s.ad_value(639)
                } else {
                    A::neg(s.ad_value(639))
                }
            });
        }

        if (((!(s.v[725] != 0.0)) && (!(s.v[752] != 0.0))) && (!(s.v[754] != 0.0))) {
            s.store_sqrt_ad(639, A::add(A::square(s.ad_value(638)), s.ad_value(639)));
        }

        if (((!(s.v[725] != 0.0)) && (!(s.v[752] != 0.0))) && (!(s.v[754] != 0.0))) {
            s.store_scale_ad(279, A::offset(A::div(s.ad_value(638), s.ad_value(639)), 1.0), 0.5);
        }

        if (((!(s.v[725] != 0.0)) && (!(s.v[752] != 0.0))) && (!(s.v[754] != 0.0))) {
            s.store_sub_ad_rhs(307, 325, A::scale(A::add(s.ad_value(638), s.ad_value(639)), 0.5));
        }

        s.v[755] = if ((s.v[39] == 1.0) && (0.0 != 0.0)) { 1.0 } else { 0.0 };

        if ((!(s.v[725] != 0.0)) && (s.v[755] != 0.0)) {
            s.store_scalar(39, 1.0);
        }

        if ((!(s.v[725] != 0.0)) && (s.v[755] != 0.0)) {
            s.store_scalar(62, 0.0);
        }

        let mut assign8540_loop_guard: usize = 0;
        while {
            let assign8540_cond_e8341: f64 = if (((!(s.v[725] != 0.0)) && (s.v[755] != 0.0)) && (s.v[62] < s.v[28])) { 1.0 } else { 0.0 };
            assign8540_cond_e8341 != 0.0
        } {
            assign8540_loop_guard += 1;
            assert!(assign8540_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((!(s.v[725] != 0.0)) && (s.v[755] != 0.0)) {
                s.copy_ad(279, 439);
            }
            if ((!(s.v[725] != 0.0)) && (s.v[755] != 0.0)) {
                s.store_mul(280, 120, 307);
            }
            if ((!(s.v[725] != 0.0)) && (s.v[755] != 0.0)) {
                s.store_exp_ad(281, A::neg(s.ad_value(280)));
            }
            s.v[756] = if (s.v[307] > 1e-8) { 1.0 } else { 0.0 };
            if (((!(s.v[725] != 0.0)) && (s.v[755] != 0.0)) && (s.v[756] != 0.0)) {
                s.store_exp_ad(278, A::mul(s.ad_value(120), s.ad_value(307)));
            }
            if (((!(s.v[725] != 0.0)) && (s.v[755] != 0.0)) && (s.v[756] != 0.0)) {
                s.store_mul_ad(282, A::neg(s.ad_value(279)), A::sqrt(A::add(A::offset(A::add(s.ad_value(281), s.ad_value(280)), (-1.0)), A::mul(s.ad_value(143), A::offset(s.ad_value(278), (-1.0))))));
            }
            if (((!(s.v[725] != 0.0)) && (s.v[755] != 0.0)) && (s.v[756] != 0.0)) {
                s.store_mul_ad(283, A::div(s.ad_value(438), s.ad_value(282)), A::add(A::sub_from_scalar(1.0, s.ad_value(281)), A::mul(s.ad_value(143), s.ad_value(278))));
            }
            s.v[757] = if (s.v[307] < (-1e-8)) { 1.0 } else { 0.0 };
            if ((((!(s.v[725] != 0.0)) && (s.v[755] != 0.0)) && (!(s.v[756] != 0.0))) && (s.v[757] != 0.0)) {
                s.store_mul_ad_rhs(282, 279, A::sqrt(A::offset(A::add(s.ad_value(281), s.ad_value(280)), (-1.0))));
            }
            if ((((!(s.v[725] != 0.0)) && (s.v[755] != 0.0)) && (!(s.v[756] != 0.0))) && (s.v[757] != 0.0)) {
                s.store_mul_ad(283, A::div(s.ad_value(438), s.ad_value(282)), A::sub_from_scalar(1.0, s.ad_value(281)));
            }
            if ((((!(s.v[725] != 0.0)) && (s.v[755] != 0.0)) && (!(s.v[756] != 0.0))) && (!(s.v[757] != 0.0))) {
                s.store_mul_ad_lhs(282, A::mul(A::neg(A::sqrt(A::div(s.ad_value(438), s.ad_value(120)))), s.ad_value(120)), 307);
            }
            if ((((!(s.v[725] != 0.0)) && (s.v[755] != 0.0)) && (!(s.v[756] != 0.0))) && (!(s.v[757] != 0.0))) {
                s.store_neg_ad(283, A::sqrt(A::mul(s.ad_value(438), s.ad_value(120))));
            }
            if ((!(s.v[725] != 0.0)) && (s.v[755] != 0.0)) {
                s.store_sub_ad_rhs(284, 307, A::div(A::sub(A::sub(A::scale(s.ad_value(282), 1.0 / (s.v[294])), s.ad_value(307)), s.ad_value(440)), A::offset(A::scale(s.ad_value(283), 1.0 / (s.v[294])), (-1.0))));
            }
            s.v[758] = if ((((s.v[284] - s.v[307])) as f64).abs() < 0.001) { 1.0 } else { 0.0 };
            if (((!(s.v[725] != 0.0)) && (s.v[755] != 0.0)) && (s.v[758] != 0.0)) {
                s.copy_ad(285, 62);
            }
            if (((!(s.v[725] != 0.0)) && (s.v[755] != 0.0)) && (s.v[758] != 0.0)) {
                s.store_scalar(62, s.v[28]);
            }
            if ((!(s.v[725] != 0.0)) && (s.v[755] != 0.0)) {
                s.copy_ad(307, 284);
            }
            if ((!(s.v[725] != 0.0)) && (s.v[755] != 0.0)) {
                s.copy_ad(312, 282);
            }
            if ((!(s.v[725] != 0.0)) && (s.v[755] != 0.0)) {
                s.store_offset(62, 62, 1.0);
            }
        }

        if ((!(s.v[725] != 0.0)) && (s.v[755] != 0.0)) {
            s.store_add(307, 440, 307);
        }

        if ((!(s.v[725] != 0.0)) && (s.v[755] != 0.0)) {
            s.store_sub_ad_rhs(306, 307, A::scale(s.ad_value(312), 1.0 / (s.v[294])));
        }

        if ((!(s.v[725] != 0.0)) && (!(s.v[755] != 0.0))) {
            s.store_scalar(39, 2.0);
        }

        s.v[759] = if (0.0 == 0.0) { 1.0 } else { 0.0 };

        if (((!(s.v[725] != 0.0)) && (!(s.v[755] != 0.0))) && (s.v[759] != 0.0)) {
            s.store_scalar(315, (1e-12 * 100.0));
        }

        if (((!(s.v[725] != 0.0)) && (!(s.v[755] != 0.0))) && (s.v[759] != 0.0)) {
            s.copy_ad(56, 319);
        }

        if (((!(s.v[725] != 0.0)) && (!(s.v[755] != 0.0))) && (!(s.v[759] != 0.0))) {
            s.store_scalar(315, 0.001);
        }

        if (((!(s.v[725] != 0.0)) && (!(s.v[755] != 0.0))) && (!(s.v[759] != 0.0))) {
            s.copy_ad(56, 305);
        }

        if ((!(s.v[725] != 0.0)) && (!(s.v[755] != 0.0))) {
            s.store_scalar(62, 0.0);
        }

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
        let mut assign8640_loop_guard: usize = 0;
        while {
            let assign8640_cond_e8666: f64 = if (((!(s.v[725] != 0.0)) && (!(s.v[755] != 0.0))) && (s.v[62] < s.v[28])) { 1.0 } else { 0.0 };
            assign8640_cond_e8666 != 0.0
        } {
            assign8640_loop_guard += 1;
            assert!(assign8640_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((!(s.v[725] != 0.0)) && (!(s.v[755] != 0.0))) {
                s.copy_ad(279, 439);
            }
            if ((!(s.v[725] != 0.0)) && (!(s.v[755] != 0.0))) {
                s.store_mul(280, 120, 307);
            }
            if ((!(s.v[725] != 0.0)) && (!(s.v[755] != 0.0))) {
                s.store_exp_ad(281, A::neg(s.ad_value(280)));
            }
            s.v[760] = if (s.v[307] > 1e-8) { 1.0 } else { 0.0 };
            if (((!(s.v[725] != 0.0)) && (!(s.v[755] != 0.0))) && (s.v[760] != 0.0)) {
                s.store_exp_ad(278, A::mul(s.ad_value(120), s.ad_value(307)));
            }
            if (((!(s.v[725] != 0.0)) && (!(s.v[755] != 0.0))) && (s.v[760] != 0.0)) {
                s.store_mul_ad(282, A::neg(s.ad_value(279)), A::sqrt(A::add(A::offset(A::add(s.ad_value(281), s.ad_value(280)), (-1.0)), A::mul(s.ad_value(143), A::offset(s.ad_value(278), (-1.0))))));
            }
            if (((!(s.v[725] != 0.0)) && (!(s.v[755] != 0.0))) && (s.v[760] != 0.0)) {
                s.store_mul_ad(283, A::div(s.ad_value(438), s.ad_value(282)), A::add(A::sub_from_scalar(1.0, s.ad_value(281)), A::mul(s.ad_value(143), s.ad_value(278))));
            }
            s.v[761] = if (s.v[307] < (-1e-8)) { 1.0 } else { 0.0 };
            if ((((!(s.v[725] != 0.0)) && (!(s.v[755] != 0.0))) && (!(s.v[760] != 0.0))) && (s.v[761] != 0.0)) {
                s.store_mul_ad_rhs(282, 279, A::sqrt(A::offset(A::add(s.ad_value(281), s.ad_value(280)), (-1.0))));
            }
            if ((((!(s.v[725] != 0.0)) && (!(s.v[755] != 0.0))) && (!(s.v[760] != 0.0))) && (s.v[761] != 0.0)) {
                s.store_mul_ad(283, A::div(s.ad_value(438), s.ad_value(282)), A::sub_from_scalar(1.0, s.ad_value(281)));
            }
            if ((((!(s.v[725] != 0.0)) && (!(s.v[755] != 0.0))) && (!(s.v[760] != 0.0))) && (!(s.v[761] != 0.0))) {
                s.store_mul_ad_lhs(282, A::mul(A::neg(A::sqrt(A::div(s.ad_value(438), s.ad_value(120)))), s.ad_value(120)), 307);
            }
            if ((((!(s.v[725] != 0.0)) && (!(s.v[755] != 0.0))) && (!(s.v[760] != 0.0))) && (!(s.v[761] != 0.0))) {
                s.store_neg_ad(283, A::sqrt(A::mul(s.ad_value(438), s.ad_value(120))));
            }
            if ((!(s.v[725] != 0.0)) && (!(s.v[755] != 0.0))) {
                let assign8640_body12_ad_e8877: A = A::div(A::sub(A::add(A::add(A::sub(s.ad_value(56), s.ad_value(307)), A::scale(s.ad_value(282), 1.0 / (s.v[294]))), A::scale(A::add(s.ad_value(282), A::scale(s.ad_value(296), 0.5)), (p.p227 * 9662367879.197212))), s.ad_value(440)), A::add(A::offset(A::scale(s.ad_value(283), 1.0 / (s.v[294])), (-1.0)), A::scale(s.ad_value(283), (p.p227 * 9662367879.197212))));
                s.store_sub_ad_rhs(284, 307, assign8640_body12_ad_e8877);
            }
            s.v[762] = if ((((s.v[284] - s.v[307])) as f64).abs() < s.v[315]) { 1.0 } else { 0.0 };
            if (((!(s.v[725] != 0.0)) && (!(s.v[755] != 0.0))) && (s.v[762] != 0.0)) {
                s.copy_ad(285, 62);
            }
            if (((!(s.v[725] != 0.0)) && (!(s.v[755] != 0.0))) && (s.v[762] != 0.0)) {
                s.store_scalar(62, s.v[28]);
            }
            if ((!(s.v[725] != 0.0)) && (!(s.v[755] != 0.0))) {
                s.copy_ad(307, 284);
            }
            if ((!(s.v[725] != 0.0)) && (!(s.v[755] != 0.0))) {
                s.copy_ad(312, 282);
            }
            if ((!(s.v[725] != 0.0)) && (!(s.v[755] != 0.0))) {
                s.store_offset(62, 62, 1.0);
            }
        }

        s.v[763] = if (0.0 == 0.0) { 1.0 } else { 0.0 };

        if (((!(s.v[725] != 0.0)) && (!(s.v[755] != 0.0))) && (s.v[763] != 0.0)) {
            s.copy_ad(316, 312);
        }

        s.v[764] = if (1.0 == 0.0) { 1.0 } else { 0.0 };

        if (((!(s.v[725] != 0.0)) && (!(s.v[755] != 0.0))) && (s.v[764] != 0.0)) {
            s.store_scalar(315, (1e-12 * 100.0));
        }

        if (((!(s.v[725] != 0.0)) && (!(s.v[755] != 0.0))) && (s.v[764] != 0.0)) {
            s.copy_ad(56, 319);
        }

        if (((!(s.v[725] != 0.0)) && (!(s.v[755] != 0.0))) && (!(s.v[764] != 0.0))) {
            s.store_scalar(315, 0.001);
        }

        if (((!(s.v[725] != 0.0)) && (!(s.v[755] != 0.0))) && (!(s.v[764] != 0.0))) {
            s.copy_ad(56, 305);
        }

        if ((!(s.v[725] != 0.0)) && (!(s.v[755] != 0.0))) {
            s.store_scalar(62, 0.0);
        }

        let mut assign8730_loop_guard: usize = 0;
        while {
            let assign8730_cond_e9009: f64 = if (((!(s.v[725] != 0.0)) && (!(s.v[755] != 0.0))) && (s.v[62] < s.v[28])) { 1.0 } else { 0.0 };
            assign8730_cond_e9009 != 0.0
        } {
            assign8730_loop_guard += 1;
            assert!(assign8730_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((!(s.v[725] != 0.0)) && (!(s.v[755] != 0.0))) {
                s.copy_ad(279, 439);
            }
            if ((!(s.v[725] != 0.0)) && (!(s.v[755] != 0.0))) {
                s.store_mul(280, 120, 307);
            }
            if ((!(s.v[725] != 0.0)) && (!(s.v[755] != 0.0))) {
                s.store_exp_ad(281, A::neg(s.ad_value(280)));
            }
            s.v[765] = if (s.v[307] > 1e-8) { 1.0 } else { 0.0 };
            if (((!(s.v[725] != 0.0)) && (!(s.v[755] != 0.0))) && (s.v[765] != 0.0)) {
                s.store_exp_ad(278, A::mul(s.ad_value(120), s.ad_value(307)));
            }
            if (((!(s.v[725] != 0.0)) && (!(s.v[755] != 0.0))) && (s.v[765] != 0.0)) {
                s.store_mul_ad(282, A::neg(s.ad_value(279)), A::sqrt(A::add(A::offset(A::add(s.ad_value(281), s.ad_value(280)), (-1.0)), A::mul(s.ad_value(143), A::offset(s.ad_value(278), (-1.0))))));
            }
            if (((!(s.v[725] != 0.0)) && (!(s.v[755] != 0.0))) && (s.v[765] != 0.0)) {
                s.store_mul_ad(283, A::div(s.ad_value(438), s.ad_value(282)), A::add(A::sub_from_scalar(1.0, s.ad_value(281)), A::mul(s.ad_value(143), s.ad_value(278))));
            }
            s.v[766] = if (s.v[307] < (-1e-8)) { 1.0 } else { 0.0 };
            if ((((!(s.v[725] != 0.0)) && (!(s.v[755] != 0.0))) && (!(s.v[765] != 0.0))) && (s.v[766] != 0.0)) {
                s.store_mul_ad_rhs(282, 279, A::sqrt(A::offset(A::add(s.ad_value(281), s.ad_value(280)), (-1.0))));
            }
            if ((((!(s.v[725] != 0.0)) && (!(s.v[755] != 0.0))) && (!(s.v[765] != 0.0))) && (s.v[766] != 0.0)) {
                s.store_mul_ad(283, A::div(s.ad_value(438), s.ad_value(282)), A::sub_from_scalar(1.0, s.ad_value(281)));
            }
            if ((((!(s.v[725] != 0.0)) && (!(s.v[755] != 0.0))) && (!(s.v[765] != 0.0))) && (!(s.v[766] != 0.0))) {
                s.store_mul_ad_lhs(282, A::mul(A::neg(A::sqrt(A::div(s.ad_value(438), s.ad_value(120)))), s.ad_value(120)), 307);
            }
            if ((((!(s.v[725] != 0.0)) && (!(s.v[755] != 0.0))) && (!(s.v[765] != 0.0))) && (!(s.v[766] != 0.0))) {
                s.store_neg_ad(283, A::sqrt(A::mul(s.ad_value(438), s.ad_value(120))));
            }
            if ((!(s.v[725] != 0.0)) && (!(s.v[755] != 0.0))) {
                let assign8730_body12_ad_e9220: A = A::div(A::sub(A::add(A::add(A::sub(s.ad_value(56), s.ad_value(307)), A::scale(s.ad_value(282), 1.0 / (s.v[294]))), A::scale(A::add(s.ad_value(282), A::scale(s.ad_value(296), 0.5)), (p.p227 * 9662367879.197212))), s.ad_value(440)), A::add(A::offset(A::scale(s.ad_value(283), 1.0 / (s.v[294])), (-1.0)), A::scale(s.ad_value(283), (p.p227 * 9662367879.197212))));
                s.store_sub_ad_rhs(284, 307, assign8730_body12_ad_e9220);
            }
            s.v[767] = if ((((s.v[284] - s.v[307])) as f64).abs() < s.v[315]) { 1.0 } else { 0.0 };
            if (((!(s.v[725] != 0.0)) && (!(s.v[755] != 0.0))) && (s.v[767] != 0.0)) {
                s.copy_ad(285, 62);
            }
            if (((!(s.v[725] != 0.0)) && (!(s.v[755] != 0.0))) && (s.v[767] != 0.0)) {
                s.store_scalar(62, s.v[28]);
            }
            if ((!(s.v[725] != 0.0)) && (!(s.v[755] != 0.0))) {
                s.copy_ad(307, 284);
            }
            if ((!(s.v[725] != 0.0)) && (!(s.v[755] != 0.0))) {
                s.copy_ad(312, 282);
            }
            if ((!(s.v[725] != 0.0)) && (!(s.v[755] != 0.0))) {
                s.store_offset(62, 62, 1.0);
            }
        }

        s.v[768] = if (1.0 == 0.0) { 1.0 } else { 0.0 };

        if (((!(s.v[725] != 0.0)) && (!(s.v[755] != 0.0))) && (s.v[768] != 0.0)) {
            s.copy_ad(316, 312);
        }

        if ((!(s.v[725] != 0.0)) && (!(s.v[755] != 0.0))) {
            s.store_scalar(63, 0.0);
        }

        if (!(s.v[725] != 0.0)) {
            s.store_offset_ad(307, A::add(s.ad_value(440), s.ad_value(307)), (-0.01));
        }

        if (!(s.v[725] != 0.0)) {
            s.store_sub_ad_rhs(306, 307, A::scale(s.ad_value(312), 1.0 / (s.v[294])));
        }

        s.v[769] = if ((s.v[306] > (s.v[305] - 0.15)) && (0.15 >= 0.0)) { 1.0 } else { 0.0 };

        if ((!(s.v[725] != 0.0)) && (s.v[769] != 0.0)) {
            s.store_offset_ad(638, A::sub(s.ad_value(306), s.ad_value(305)), 0.15);
        }

        if ((!(s.v[725] != 0.0)) && (s.v[769] != 0.0)) {
            s.store_square(642, 638);
        }

        if ((!(s.v[725] != 0.0)) && (s.v[769] != 0.0)) {
            s.store_scalar(643, (0.15 * 0.15));
        }

        if ((!(s.v[725] != 0.0)) && (s.v[769] != 0.0)) {
            s.store_scalar(644, 1.0);
        }

        if ((!(s.v[725] != 0.0)) && (s.v[769] != 0.0)) {
            s.store_scalar(645, 1.0);
        }

        if ((!(s.v[725] != 0.0)) && (s.v[769] != 0.0)) {
            s.store_scalar(647, 0.0);
        }

        if ((!(s.v[725] != 0.0)) && (s.v[769] != 0.0)) {
            s.store_scalar(648, 0.0);
        }

        if ((!(s.v[725] != 0.0)) && (s.v[769] != 0.0)) {
            s.store_scalar(220, 0.0);
        }

        if ((!(s.v[725] != 0.0)) && (s.v[769] != 0.0)) {
            s.store_scalar(646, 0.0);
        }

        if ((!(s.v[725] != 0.0)) && (s.v[769] != 0.0)) {
            s.store_mul(644, 644, 642);
        }

        if ((!(s.v[725] != 0.0)) && (s.v[769] != 0.0)) {
            s.store_mul(645, 645, 643);
        }

        if ((!(s.v[725] != 0.0)) && (s.v[769] != 0.0)) {
            s.store_add(220, 644, 645);
        }

        if ((!(s.v[725] != 0.0)) && (s.v[769] != 0.0)) {
            s.copy_ad(646, 220);
        }

        s.v[770] = if ((((1.0 == 1.0) || (1.0 == 2.0)) || (1.0 == 4.0)) || (1.0 == 8.0)) { 1.0 } else { 0.0 };

        s.v[771] = if (1.0 == 1.0) { 1.0 } else { 0.0 };

        if ((((!(s.v[725] != 0.0)) && (s.v[769] != 0.0)) && (s.v[770] != 0.0)) && (s.v[771] != 0.0)) {
            s.store_scalar(648, 1.0);
        }

        s.v[772] = if (1.0 == 2.0) { 1.0 } else { 0.0 };

        if (((((!(s.v[725] != 0.0)) && (s.v[769] != 0.0)) && (s.v[770] != 0.0)) && (!(s.v[771] != 0.0))) && (s.v[772] != 0.0)) {
            s.store_scalar(648, 2.0);
        }

        s.v[773] = if (1.0 == 4.0) { 1.0 } else { 0.0 };

        if ((((((!(s.v[725] != 0.0)) && (s.v[769] != 0.0)) && (s.v[770] != 0.0)) && (!(s.v[771] != 0.0))) && (!(s.v[772] != 0.0))) && (s.v[773] != 0.0)) {
            s.store_scalar(648, 3.0);
        }

        s.v[774] = if (1.0 == 8.0) { 1.0 } else { 0.0 };

        if (((((((!(s.v[725] != 0.0)) && (s.v[769] != 0.0)) && (s.v[770] != 0.0)) && (!(s.v[771] != 0.0))) && (!(s.v[772] != 0.0))) && (!(s.v[773] != 0.0))) && (s.v[774] != 0.0)) {
            s.store_scalar(648, 4.0);
        }

        if (((!(s.v[725] != 0.0)) && (s.v[769] != 0.0)) && (s.v[770] != 0.0)) {
            s.store_scalar(647, 0.0);
        }

        let mut assign9030_loop_guard: usize = 0;
        while {
            let assign9030_cond_e9536: f64 = if ((((!(s.v[725] != 0.0)) && (s.v[769] != 0.0)) && (s.v[770] != 0.0)) && (s.v[647] < s.v[648])) { 1.0 } else { 0.0 };
            assign9030_cond_e9536 != 0.0
        } {
            assign9030_loop_guard += 1;
            assert!(assign9030_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((!(s.v[725] != 0.0)) && (s.v[769] != 0.0)) && (s.v[770] != 0.0)) {
                s.store_sqrt(646, 646);
            }
            if (((!(s.v[725] != 0.0)) && (s.v[769] != 0.0)) && (s.v[770] != 0.0)) {
                s.store_offset(647, 647, 1.0);
            }
        }

        if (((!(s.v[725] != 0.0)) && (s.v[769] != 0.0)) && (!(s.v[770] != 0.0))) {
            s.store_powf(646, 646, (1.0 / 2.0));
        }

        if ((!(s.v[725] != 0.0)) && (s.v[769] != 0.0)) {
            s.store_div_from_scalar_ad(646, 1.0, A::offset(s.ad_value(646), 1e-50));
        }

        if ((!(s.v[725] != 0.0)) && (s.v[769] != 0.0)) {
            s.store_mul_ad_lhs(637, A::scale(s.ad_value(638), 0.15), 646);
        }

        if ((!(s.v[725] != 0.0)) && (s.v[769] != 0.0)) {
            s.store_div_ad(278, A::mul(A::scale(s.ad_value(645), 0.15), s.ad_value(646)), A::offset(s.ad_value(220), 1e-50));
        }

        if ((!(s.v[725] != 0.0)) && (s.v[769] != 0.0)) {
            s.store_add_ad_lhs(306, A::offset(s.ad_value(305), (-0.15)), 637);
        }

        if ((!(s.v[725] != 0.0)) && (s.v[769] != 0.0)) {
        }

        if ((!(s.v[725] != 0.0)) && (!(s.v[769] != 0.0))) {
        }

        if ((!(s.v[725] != 0.0)) && (!(s.v[769] != 0.0))) {
            s.store_scalar(278, 1.0);
        }

        if (!(s.v[725] != 0.0)) {
            s.copy_ad(522, 306);
        }

        s.v[775] = if ((p.p15 == 1.0) && (s.v[52] > (s.v[54] + 0.2))) { 1.0 } else { 0.0 };

        if (s.v[775] != 0.0) {
            s.store_scalar(389, s.v[559]);
        }

        if (s.v[775] != 0.0) {
            s.store_sub_ad_lhs(388, A::add(A::sub(s.ad_value(72), s.ad_value(389)), s.ad_value(80)), 267);
        }

        if (s.v[775] != 0.0) {
            s.store_scalar(32, p.p136);
        }

        if (s.v[775] != 0.0) {
            s.copy_ad(99, 388);
        }

        if (s.v[775] != 0.0) {
            s.store_sqrt_ad(100, A::div(A::scale(s.ad_value(471), ((2.0 * 1.6021918e-19) * 1.034943e-10)), s.ad_value(120)));
        }

        if (s.v[775] != 0.0) {
            s.store_div_ad_lhs(101, A::div(A::square(s.ad_value(127)), s.ad_value(471)), 471);
        }

        if (s.v[775] != 0.0) {
            s.store_div_ad_lhs(102, A::div(A::square(s.ad_value(100)), s.ad_value(270)), 270);
        }

        if (s.v[775] != 0.0) {
            s.store_scaled_mul(103, 102, 120, 0.5);
        }

        if (s.v[775] != 0.0) {
            s.store_scaled_mul(104, 103, 120, 2.0);
        }

        if (s.v[775] != 0.0) {
            s.store_sqrt_ad(105, A::offset(A::div(A::scale(A::offset(A::mul(s.ad_value(120), s.ad_value(99)), (-1.0)), 4.0), s.ad_value(104)), 1.0));
        }

        if (s.v[775] != 0.0) {
            s.store_add_ad_rhs(107, 99, A::mul(s.ad_value(103), A::sub_from_scalar(1.0, s.ad_value(105))));
        }

        if (s.v[775] != 0.0) {
            s.store_div_ad_lhs(108, A::div_from_scalar(1.0, s.ad_value(101)), 102);
        }

        if (s.v[775] != 0.0) {
            s.store_div_ad(109, A::ln(A::mul(s.ad_value(108), A::square(s.ad_value(99)))), A::add(s.ad_value(120), A::div_from_scalar(2.0, s.ad_value(99))));
        }

        if (s.v[775] != 0.0) {
            s.store_sub_ad_lhs(110, A::sub(s.ad_value(109), s.ad_value(107)), 32);
        }

        if (s.v[775] != 0.0) {
            s.store_sub_ad_rhs(111, 109, A::scale(A::add(s.ad_value(110), A::sqrt(A::add(A::square(s.ad_value(110)), A::mul(A::scale(s.ad_value(32), 4.0), s.ad_value(109))))), 0.5));
        }

        if (s.v[775] != 0.0) {
            s.store_exp_ad(112, A::mul(s.ad_value(120), s.ad_value(111)));
        }

        if (s.v[775] != 0.0) {
            s.store_add_ad(113, A::offset(A::mul(s.ad_value(120), s.ad_value(111)), (-1.0)), A::mul(s.ad_value(101), s.ad_value(112)));
        }

        if (s.v[775] != 0.0) {
            s.store_offset_ad(114, A::mul(s.ad_value(120), s.ad_value(111)), (-1.0));
        }

        s.v[776] = if ((s.v[113] > 0.0) && (s.v[114] > 0.0)) { 1.0 } else { 0.0 };

        if ((s.v[775] != 0.0) && (s.v[776] != 0.0)) {
            s.store_sqrt_ad(113, A::add(A::offset(A::mul(s.ad_value(120), s.ad_value(111)), (-1.0)), A::mul(s.ad_value(101), s.ad_value(112))));
        }

        if ((s.v[775] != 0.0) && (s.v[776] != 0.0)) {
            s.store_sqrt_ad(114, A::offset(A::mul(s.ad_value(120), s.ad_value(111)), (-1.0)));
        }

        if ((s.v[775] != 0.0) && (s.v[776] != 0.0)) {
            s.store_mul_ad_rhs(115, 100, A::sub(s.ad_value(113), s.ad_value(114)));
        }

        if ((s.v[775] != 0.0) && (s.v[776] != 0.0)) {
            s.store_div_from_scalar(106, (2.0 * s.v[124]), 120);
        }

        if ((s.v[775] != 0.0) && (s.v[776] != 0.0)) {
            s.store_scalar(158, (300.0 * 0.0001));
        }

        if ((s.v[775] != 0.0) && (s.v[776] != 0.0)) {
            s.store_scalar(262, 0.0);
        }

        if ((s.v[775] != 0.0) && (s.v[776] != 0.0)) {
            s.store_scalar(279, 0.0);
        }

        if ((s.v[775] != 0.0) && (s.v[776] != 0.0)) {
            s.store_div_ad(116, A::mul(A::mul(A::mul(s.ad_value(106), s.ad_value(158)), s.ad_value(115)), s.ad_value(279)), A::sub(s.ad_value(123), s.ad_value(262)));
        }

        if ((s.v[775] != 0.0) && (s.v[776] != 0.0)) {
            s.copy_ad(338, 116);
        }

        if ((s.v[775] != 0.0) && (s.v[776] != 0.0)) {
            s.copy_ad(339, 111);
        }

        if ((s.v[775] != 0.0) && (s.v[776] != 0.0)) {
            s.store_offset_ad(290, A::div(A::scale(A::offset(A::mul(s.ad_value(120), s.ad_value(76)), (-1.0)), 4.0), A::mul(s.ad_value(145), s.ad_value(121))), 1.0);
        }

        s.v[777] = if (s.v[290] < (10.0 * 2.220446049250313e-16)) { 1.0 } else { 0.0 };

        if (((s.v[775] != 0.0) && (s.v[776] != 0.0)) && (s.v[777] != 0.0)) {
            s.store_scalar(290, (10.0 * 2.220446049250313e-16));
        }

        if ((s.v[775] != 0.0) && (s.v[776] != 0.0)) {
            s.store_add_ad_rhs(319, 76, A::mul(A::scale(A::mul(s.ad_value(145), s.ad_value(120)), 0.5), A::sub_from_scalar(1.0, A::sqrt(s.ad_value(290)))));
        }

        if ((s.v[775] != 0.0) && (s.v[776] != 0.0)) {
            s.copy_ad(58, 319);
        }

        if ((s.v[775] != 0.0) && (s.v[776] != 0.0)) {
            s.store_sub(61, 319, 339);
        }

        s.v[778] = if (s.v[61] < 0.0) { 1.0 } else { 0.0 };

        if (((s.v[775] != 0.0) && (s.v[776] != 0.0)) && (s.v[778] != 0.0)) {
            s.store_scalar(61, 0.0);
        }

        if ((s.v[775] != 0.0) && (s.v[776] != 0.0)) {
            s.store_scale(283, 61, (1.0 + 0.3));
        }

        if ((s.v[775] != 0.0) && (s.v[776] != 0.0)) {
            s.store_offset_ad(284, A::sub(s.ad_value(283), s.ad_value(71)), (-0.03));
        }

        if ((s.v[775] != 0.0) && (s.v[776] != 0.0)) {
            s.store_sqrt_ad(285, A::add(A::square(s.ad_value(284)), A::scale(s.ad_value(283), (4.0 * 0.03))));
        }

        if ((s.v[775] != 0.0) && (s.v[776] != 0.0)) {
            s.store_sub_ad_rhs(60, 283, A::scale(A::add(s.ad_value(284), s.ad_value(285)), 0.5));
        }

        s.v[779] = if (s.v[60] > s.v[61]) { 1.0 } else { 0.0 };

        if (((s.v[775] != 0.0) && (s.v[776] != 0.0)) && (s.v[779] != 0.0)) {
            s.copy_ad(60, 61);
        }

        if ((s.v[775] != 0.0) && (s.v[776] != 0.0)) {
            s.copy_ad(392, 60);
        }

        if ((s.v[775] != 0.0) && (s.v[776] != 0.0)) {
            s.store_scalar(796, (s.v[272] * 100.0));
        }

        if ((s.v[775] != 0.0) && (s.v[776] != 0.0)) {
            s.store_scalar(797, (s.v[466] * 100.0));
        }

        if ((s.v[775] != 0.0) && (s.v[776] != 0.0)) {
            s.store_scale(798, 123, 100.0);
        }

        s.v[799] = if (p.p26 == 0.0) { 1.0 } else { 0.0 };

        if (((s.v[775] != 0.0) && (s.v[776] != 0.0)) && (!(s.v[799] != 0.0))) {
            s.store_scalar(391, 4.12);
        }

        if (((s.v[775] != 0.0) && (s.v[776] != 0.0)) && (!(s.v[799] != 0.0))) {
            s.store_mul_ad_lhs(780, A::scale(s.ad_value(797), (p.p141 * 1.6021918e-19)), 798);
        }

        if (((s.v[775] != 0.0) && (s.v[776] != 0.0)) && (!(s.v[799] != 0.0))) {
            s.store_div(781, 780, 245);
        }

        if (((s.v[775] != 0.0) && (s.v[776] != 0.0)) && (!(s.v[799] != 0.0))) {
            s.store_div_ad_lhs(782, A::neg(A::offset(A::add(A::add(A::add(A::scale(s.ad_value(70), p.p144), s.ad_value(82)), s.ad_value(266)), s.ad_value(137)), p.p143)), 796);
        }

        if (((s.v[775] != 0.0) && (s.v[776] != 0.0)) && (!(s.v[799] != 0.0))) {
            s.store_scalar(514, 0.0);
        }

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
        let mut assign9680_loop_guard: usize = 0;
        while {
            let assign9680_cond_e10183: f64 = (100.0 - 1.0);
            let assign9680_cond_e10185: f64 = if ((((s.v[775] != 0.0) && (s.v[776] != 0.0)) && (!(s.v[799] != 0.0))) && (s.v[514] <= assign9680_cond_e10183)) { 1.0 } else { 0.0 };
            assign9680_cond_e10185 != 0.0
        } {
            assign9680_loop_guard += 1;
            assert!(assign9680_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.v[775] != 0.0) && (s.v[776] != 0.0)) && (!(s.v[799] != 0.0))) {
                s.copy_ad(783, 514);
            }
            if (((s.v[775] != 0.0) && (s.v[776] != 0.0)) && (!(s.v[799] != 0.0))) {
                s.store_scalar(784, 100.0);
            }
            if (((s.v[775] != 0.0) && (s.v[776] != 0.0)) && (!(s.v[799] != 0.0))) {
                s.store_div(785, 783, 784);
            }
            if (((s.v[775] != 0.0) && (s.v[776] != 0.0)) && (!(s.v[799] != 0.0))) {
                s.store_sub_ad(786, A::add(s.ad_value(53), s.ad_value(73)), A::add(A::mul(s.ad_value(392), s.ad_value(785)), s.ad_value(339)));
            }
            if (((s.v[775] != 0.0) && (s.v[776] != 0.0)) && (!(s.v[799] != 0.0))) {
                s.store_sub_from_scalar_ad(787, 1.0, A::div(s.ad_value(786), s.ad_value(391)));
            }
            if (((s.v[775] != 0.0) && (s.v[776] != 0.0)) && (!(s.v[799] != 0.0))) {
                s.store_add_ad_rhs(790, 782, A::div(s.ad_value(786), s.ad_value(796)));
            }
            if (((s.v[775] != 0.0) && (s.v[776] != 0.0)) && (!(s.v[799] != 0.0))) {
                s.store_square(788, 790);
            }
            if (((s.v[775] != 0.0) && (s.v[776] != 0.0)) && (!(s.v[799] != 0.0))) {
                s.store_sqrt_ad(639, A::offset(A::square(s.ad_value(787)), ((4.0 * 0.001) * 0.001)));
            }
            if (((s.v[775] != 0.0) && (s.v[776] != 0.0)) && (!(s.v[799] != 0.0))) {
                s.store_offset_ad(787, A::scale(A::add(s.ad_value(787), s.ad_value(639)), 0.5), (1e-10 * 0.001));
            }
            s.v[800] = if (s.v[787] < 0.0) { 1.0 } else { 0.0 };
            if ((((s.v[775] != 0.0) && (s.v[776] != 0.0)) && (!(s.v[799] != 0.0))) && (s.v[800] != 0.0)) {
                s.store_scalar(787, 0.0);
            }
            if (((s.v[775] != 0.0) && (s.v[776] != 0.0)) && (!(s.v[799] != 0.0))) {
                s.store_scale_ad(789, A::sub_from_scalar(1.0, A::mul(A::sqrt(s.ad_value(787)), s.ad_value(787))), p.p142);
            }
            if (((s.v[775] != 0.0) && (s.v[776] != 0.0)) && (!(s.v[799] != 0.0))) {
                s.store_div_ad_lhs(791, A::neg(s.ad_value(789)), 790);
            }
            s.v[801] = if (s.v[791] < (-34.0)) { 1.0 } else { 0.0 };
            if ((((s.v[775] != 0.0) && (s.v[776] != 0.0)) && (!(s.v[799] != 0.0))) && (s.v[801] != 0.0)) {
                s.store_scalar(792, 0.0);
            }
            if ((((s.v[775] != 0.0) && (s.v[776] != 0.0)) && (!(s.v[799] != 0.0))) && (!(s.v[801] != 0.0))) {
                s.store_exp(792, 791);
            }
            if (((s.v[775] != 0.0) && (s.v[776] != 0.0)) && (!(s.v[799] != 0.0))) {
                s.copy_ad(793, 781);
            }
            if (((s.v[775] != 0.0) && (s.v[776] != 0.0)) && (!(s.v[799] != 0.0))) {
                s.store_scale_ad(794, A::mul(A::mul(A::scale(s.ad_value(793), 0.25), s.ad_value(789)), s.ad_value(789)), 7.38905609893065);
            }
            s.v[802] = if (((2.0 * s.v[790]) + s.v[789]) < 0.0) { 1.0 } else { 0.0 };
            if ((((s.v[775] != 0.0) && (s.v[776] != 0.0)) && (!(s.v[799] != 0.0))) && (s.v[802] != 0.0)) {
                s.copy_ad(393, 794);
            }
            if ((((s.v[775] != 0.0) && (s.v[776] != 0.0)) && (!(s.v[799] != 0.0))) && (!(s.v[802] != 0.0))) {
                s.store_mul_ad_lhs(795, A::mul(s.ad_value(780), s.ad_value(788)), 792);
            }
            s.v[803] = if ((s.v[795] < s.v[794]) || (s.v[790] < 0.0)) { 1.0 } else { 0.0 };
            if (((((s.v[775] != 0.0) && (s.v[776] != 0.0)) && (!(s.v[799] != 0.0))) && (!(s.v[802] != 0.0))) && (s.v[803] != 0.0)) {
                s.copy_ad(393, 794);
            }
            if (((((s.v[775] != 0.0) && (s.v[776] != 0.0)) && (!(s.v[799] != 0.0))) && (!(s.v[802] != 0.0))) && (!(s.v[803] != 0.0))) {
                s.copy_ad(393, 795);
            }
            s.v[804] = if (s.v[393] < 1e-9) { 1.0 } else { 0.0 };
            if ((((s.v[775] != 0.0) && (s.v[776] != 0.0)) && (!(s.v[799] != 0.0))) && (s.v[804] != 0.0)) {
                s.store_scalar(514, 100.0);
            }
            if ((((s.v[775] != 0.0) && (s.v[776] != 0.0)) && (!(s.v[799] != 0.0))) && (s.v[804] != 0.0)) {
                s.store_scalar(62, s.v[28]);
            }
            if (((s.v[775] != 0.0) && (s.v[776] != 0.0)) && (!(s.v[799] != 0.0))) {
                s.store_offset(514, 514, 1.0);
            }
        }

        s.v[805] = if ((s.v[488] <= 0.0) || (s.v[162] <= 0.0)) { 1.0 } else { 0.0 };

        if (((s.v[775] != 0.0) && (s.v[776] != 0.0)) && (s.v[805] != 0.0)) {
            s.store_scalar(185, 0.0);
        }

        if (((s.v[775] != 0.0) && (s.v[776] != 0.0)) && (!(s.v[805] != 0.0))) {
            s.copy_ad(279, 388);
        }

        if (((s.v[775] != 0.0) && (s.v[776] != 0.0)) && (!(s.v[805] != 0.0))) {
            s.store_square(285, 270);
        }

        if (((s.v[775] != 0.0) && (s.v[776] != 0.0)) && (!(s.v[805] != 0.0))) {
            s.store_mul_ad_lhs(282, A::div_from_scalar(2.0, s.ad_value(472)), 285);
        }

        if (((s.v[775] != 0.0) && (s.v[776] != 0.0)) && (!(s.v[805] != 0.0))) {
            s.store_sub_ad(283, A::sub(s.ad_value(279), s.ad_value(122)), A::scale(s.ad_value(70), s.v[486]));
        }

        if (((s.v[775] != 0.0) && (s.v[776] != 0.0)) && (!(s.v[805] != 0.0))) {
            s.store_offset_ad(284, A::mul(s.ad_value(282), s.ad_value(283)), 1.0);
        }

        if (((s.v[775] != 0.0) && (s.v[776] != 0.0)) && (!(s.v[805] != 0.0))) {
            s.store_sqrt_ad(639, A::offset(A::square(s.ad_value(284)), ((4.0 * 0.001) * 0.001)));
        }

        if (((s.v[775] != 0.0) && (s.v[776] != 0.0)) && (!(s.v[805] != 0.0))) {
            s.store_scale_ad(287, A::offset(A::div(s.ad_value(284), s.ad_value(639)), 1.0), 0.5);
        }

        if (((s.v[775] != 0.0) && (s.v[776] != 0.0)) && (!(s.v[805] != 0.0))) {
            s.store_offset_ad(284, A::scale(A::add(s.ad_value(284), s.ad_value(639)), 0.5), (1e-10 * 0.001));
        }

        s.v[806] = if (s.v[284] < 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[775] != 0.0) && (s.v[776] != 0.0)) && (!(s.v[805] != 0.0))) && (s.v[806] != 0.0)) {
            s.store_scalar(284, 0.0);
        }

        if ((((s.v[775] != 0.0) && (s.v[776] != 0.0)) && (!(s.v[805] != 0.0))) && (s.v[806] != 0.0)) {
            s.store_scalar(287, 0.0);
        }

        if (((s.v[775] != 0.0) && (s.v[776] != 0.0)) && (!(s.v[805] != 0.0))) {
            s.store_offset(284, 284, 1e-50);
        }

        if (((s.v[775] != 0.0) && (s.v[776] != 0.0)) && (!(s.v[805] != 0.0))) {
            s.store_add_ad(186, A::scale(s.ad_value(279), s.v[491]), A::mul(A::div(s.ad_value(472), s.ad_value(285)), A::sub_from_scalar(1.0, A::sqrt(s.ad_value(284)))));
        }

        if (((s.v[775] != 0.0) && (s.v[776] != 0.0)) && (!(s.v[805] != 0.0))) {
            s.store_sub_ad(187, A::add(A::scale(s.ad_value(71), p.p123), s.ad_value(339)), A::scale(s.ad_value(186), (s.v[487] * s.v[485])));
        }

        if (((s.v[775] != 0.0) && (s.v[776] != 0.0)) && (!(s.v[805] != 0.0))) {
            s.store_sqrt_ad(639, A::offset(A::square(s.ad_value(187)), ((4.0 * 0.01) * 0.01)));
        }

        if (((s.v[775] != 0.0) && (s.v[776] != 0.0)) && (!(s.v[805] != 0.0))) {
            s.store_scale_ad(287, A::offset(A::div(s.ad_value(187), s.ad_value(639)), 1.0), 0.5);
        }

        if (((s.v[775] != 0.0) && (s.v[776] != 0.0)) && (!(s.v[805] != 0.0))) {
            s.store_offset_ad(187, A::scale(A::add(s.ad_value(187), s.ad_value(639)), 0.5), (1e-10 * 0.01));
        }

        s.v[807] = if (s.v[187] < 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[775] != 0.0) && (s.v[776] != 0.0)) && (!(s.v[805] != 0.0))) && (s.v[807] != 0.0)) {
            s.store_scalar(187, 0.0);
        }

        if ((((s.v[775] != 0.0) && (s.v[776] != 0.0)) && (!(s.v[805] != 0.0))) && (s.v[807] != 0.0)) {
            s.store_scalar(287, 0.0);
        }

        if (((s.v[775] != 0.0) && (s.v[776] != 0.0)) && (!(s.v[805] != 0.0))) {
            s.store_offset(187, 187, 1e-50);
        }

        if (((s.v[775] != 0.0) && (s.v[776] != 0.0)) && (!(s.v[805] != 0.0))) {
            s.store_exp_ad(280, A::div_from_scalar((-s.v[489]), s.ad_value(187)));
        }

        if (((s.v[775] != 0.0) && (s.v[776] != 0.0)) && (!(s.v[805] != 0.0))) {
            s.store_mul_ad_lhs(185, A::mul(A::scale(s.ad_value(187), s.v[488]), s.ad_value(338)), 280);
        }

        s.v[808] = if (p.p16 == 1.0) { 1.0 } else { 0.0 };

        if (((s.v[775] != 0.0) && (s.v[776] != 0.0)) && (s.v[808] != 0.0)) {
            s.store_scale_ad(279, A::exp(A::scale(A::neg(s.ad_value(120)), p.p140)), ((1.6021918e-19 * p.p227) * s.v[466]));
        }

        if (((s.v[775] != 0.0) && (s.v[776] != 0.0)) && (s.v[808] != 0.0)) {
            s.store_offset_scaled(280, 471, (((((36.0 * 1e-7) / 0.0001)) as f64).sqrt() * 13.0), ((((((13.0 * 1e-7) / 0.0001)) as f64).sqrt() * 36.0) * (1e20 / 1e-6)));
        }

        if (((s.v[775] != 0.0) && (s.v[776] != 0.0)) && (s.v[808] != 0.0)) {
            s.store_div_from_scalar_ad(282, (((((13.0 * 1e-7) / 0.0001)) as f64).sqrt() * ((((36.0 * 1e-7) / 0.0001)) as f64).sqrt()), A::mul(s.ad_value(279), s.ad_value(280)));
        }

        if (((s.v[775] != 0.0) && (s.v[776] != 0.0)) && (s.v[808] != 0.0)) {
            s.store_scale(283, 122, 0.0);
        }

        if (((s.v[775] != 0.0) && (s.v[776] != 0.0)) && (s.v[808] != 0.0)) {
            s.store_sqrt_ad(284, A::mul(A::scale(s.ad_value(471), ((2.0 * 1.034943e-10) * 1.6021918e-19)), s.ad_value(122)));
        }

        if (((s.v[775] != 0.0) && (s.v[776] != 0.0)) && (s.v[808] != 0.0)) {
            s.store_sqrt_ad(285, A::mul(s.ad_value(120), A::sub(s.ad_value(339), s.ad_value(283))));
        }

        if (((s.v[775] != 0.0) && (s.v[776] != 0.0)) && (s.v[808] != 0.0)) {
            s.store_sqrt_ad(286, A::mul(s.ad_value(120), s.ad_value(339)));
        }

        if (((s.v[775] != 0.0) && (s.v[776] != 0.0)) && (s.v[808] != 0.0)) {
            s.store_mul_ad(337, A::neg(s.ad_value(284)), A::sub(s.ad_value(285), s.ad_value(286)));
        }

        if ((((s.v[775] != 0.0) && (s.v[776] != 0.0)) && (s.v[808] != 0.0)) && (p.p27 != 0.0)) {
            s.store_div_from_scalar_ad(342, p.p137, A::offset(s.ad_value(185), p.p138));
        }

        if ((((s.v[775] != 0.0) && (s.v[776] != 0.0)) && (s.v[808] != 0.0)) && (p.p27 != 0.0)) {
            s.store_mul(341, 342, 270);
        }

        if ((((s.v[775] != 0.0) && (s.v[776] != 0.0)) && (s.v[808] != 0.0)) && (p.p27 != 0.0)) {
            s.copy_ad(340, 337);
        }

        if ((((s.v[775] != 0.0) && (s.v[776] != 0.0)) && (s.v[808] != 0.0)) && (p.p27 != 0.0)) {
            s.store_ad(562, &A::scale(A::voltage(ctx, &nodes, Some(10), None), 1e-9));
        }

        if ((((s.v[775] != 0.0) && (s.v[776] != 0.0)) && (s.v[808] != 0.0)) && (p.p27 != 0.0)) {
            s.copy_ad(337, 562);
        }

        if ((((s.v[775] != 0.0) && (s.v[776] != 0.0)) && (s.v[808] != 0.0)) && (p.p27 != 0.0)) {
            s.store_div_ad_lhs(558, A::sub(s.ad_value(562), s.ad_value(340)), 341);
        }

        if (((s.v[775] != 0.0) && (s.v[776] != 0.0)) && (!(s.v[808] != 0.0))) {
            s.store_scalar(337, 0.0);
        }

        if ((s.v[775] != 0.0) && (!(s.v[776] != 0.0))) {
            s.store_scalar(185, 0.0);
        }

        if ((s.v[775] != 0.0) && (!(s.v[776] != 0.0))) {
            s.store_scalar(337, 0.0);
        }

        if (!(s.v[775] != 0.0)) {
            s.store_scalar(185, 0.0);
        }

        if (!(s.v[775] != 0.0)) {
            s.store_scalar(337, 0.0);
        }

        s.copy_ad(299, 305);

        s.copy_ad(300, 306);

        s.store_sub(301, 307, 440);

        s.v[379] = 0.0;

        s.v[606] = 1.0;

        s.v[604] = 0.0;

        s.v[605] = 0.0;

        s.v[809] = if (s.v[649] < 4.0) { 1.0 } else { 0.0 };

        if (s.v[809] != 0.0) {
            s.copy_ad(599, 296);
        }

        if (s.v[809] != 0.0) {
            s.store_neg(600, 599);
        }

        if (s.v[809] != 0.0) {
            s.store_div_from_scalar_ad(601, 0.004832, A::mul(A::square(s.ad_value(296)), s.ad_value(296)));
        }

        if (s.v[809] != 0.0) {
            s.store_scale(603, 296, (-3.7477));
        }

        if (s.v[809] != 0.0) {
            s.store_scale(602, 296, 4.3495);
        }

        if (!(s.v[809] != 0.0)) {
            s.store_scale(599, 296, 1.5);
        }

        if (!(s.v[809] != 0.0)) {
            s.store_neg(600, 599);
        }

        if (!(s.v[809] != 0.0)) {
            s.store_div_from_scalar_ad(601, 0.001765, A::mul(A::square(s.ad_value(296)), s.ad_value(296)));
        }

        if (!(s.v[809] != 0.0)) {
            s.store_scale(603, 296, (-4.8303));
        }

        if (!(s.v[809] != 0.0)) {
            s.store_scale(602, 296, 5.9661);
        }

        s.copy_ad(306, 300);

        s.copy_ad(534, 300);

        s.copy_ad(522, 534);

        s.copy_ad(307, 301);

        s.v[62] = 1.0;

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
        let mut assign10390_loop_guard: usize = 0;
        while {
            let assign10390_cond_e11185: f64 = if s.v[62] <= s.v[28] { 1.0 } else { 0.0 };
            assign10390_cond_e11185 != 0.0
        } {
            assign10390_loop_guard += 1;
            assert!(assign10390_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            s.copy_ad(279, 307);
            s.store_mul(297, 120, 279);
            s.store_exp_ad(278, A::neg(s.ad_value(297)));
            s.v[810] = if (s.v[279] < (-1e-8)) { 1.0 } else { 0.0 };
            if (s.v[810] != 0.0) {
                s.store_exp_ad(280, A::mul(s.ad_value(120), s.ad_value(307)));
            }
            if (s.v[810] != 0.0) {
                s.store_mul_ad_rhs(312, 439, A::sqrt(A::add(A::offset(A::add(s.ad_value(278), s.ad_value(297)), (-1.0)), A::mul(s.ad_value(143), A::offset(s.ad_value(280), (-1.0))))));
            }
            if (s.v[810] != 0.0) {
                s.store_div_ad_lhs(343, A::mul(s.ad_value(438), A::add(A::sub_from_scalar(1.0, s.ad_value(278)), A::mul(s.ad_value(143), s.ad_value(280)))), 312);
            }
            s.v[811] = if (s.v[279] > (1e-8 / 10.0)) { 1.0 } else { 0.0 };
            if ((!(s.v[810] != 0.0)) && (s.v[811] != 0.0)) {
                s.store_exp_ad(280, A::mul(s.ad_value(120), s.ad_value(307)));
            }
            if ((!(s.v[810] != 0.0)) && (s.v[811] != 0.0)) {
                s.store_mul_ad(312, A::neg(s.ad_value(439)), A::sqrt(A::add(A::offset(A::add(s.ad_value(278), s.ad_value(297)), (-1.0)), A::mul(s.ad_value(143), A::offset(A::sub(s.ad_value(280), s.ad_value(297)), (-1.0))))));
            }
            if ((!(s.v[810] != 0.0)) && (s.v[811] != 0.0)) {
                s.store_div_ad_lhs(343, A::mul(s.ad_value(438), A::add(A::sub_from_scalar(1.0, s.ad_value(278)), A::mul(s.ad_value(143), A::offset(s.ad_value(280), (-1.0))))), 312);
            }
            if ((!(s.v[810] != 0.0)) && (!(s.v[811] != 0.0))) {
                s.store_scale_ad(312, A::mul(A::neg(s.ad_value(439)), s.ad_value(297)), 1.0 / (((2.0) as f64).sqrt()));
            }
            if ((!(s.v[810] != 0.0)) && (!(s.v[811] != 0.0))) {
                s.store_scale_ad(343, A::mul(A::neg(s.ad_value(439)), s.ad_value(120)), 1.0 / (((2.0) as f64).sqrt()));
            }
            s.store_add_ad_lhs(306, A::add(A::sub(s.ad_value(307), A::scale(s.ad_value(312), 1.0 / (s.v[294]))), s.ad_value(50)), 298);
            s.store_sub_from_scalar_ad(583, 1.0, A::scale(s.ad_value(343), 1.0 / (s.v[294])));
            s.store_sub(279, 305, 522);
            s.store_mul(297, 120, 279);
            s.v[812] = if ((-s.v[297]) >= 80.0) { 1.0 } else { 0.0 };
            if (s.v[812] != 0.0) {
                s.store_scale_ad(278, A::offset(A::sub_from_scalar(1.0, s.ad_value(297)), (-80.0)), 5.540622384e34);
            }
            if (s.v[812] != 0.0) {
                s.store_scalar(284, 5.540622384e34);
            }
            if (!(s.v[812] != 0.0)) {
                s.store_exp_ad(278, A::neg(s.ad_value(297)));
            }
            if (!(s.v[812] != 0.0)) {
                s.copy_ad(284, 278);
            }
            s.v[813] = if (s.v[279] < (-1e-8)) { 1.0 } else { 0.0 };
            if (s.v[813] != 0.0) {
                s.store_sqrt_ad(280, A::offset(A::add(s.ad_value(278), s.ad_value(297)), (-1.0)));
            }
            if (s.v[813] != 0.0) {
                s.store_mul(523, 141, 280);
            }
            if (s.v[813] != 0.0) {
                s.store_div_ad(524, A::mul(A::mul(s.ad_value(141), s.ad_value(120)), A::sub_from_scalar(1.0, s.ad_value(284))), A::scale(s.ad_value(280), 2.0));
            }
            if (s.v[813] != 0.0) {
                s.store_neg(525, 524);
            }
            if (s.v[813] != 0.0) {
                s.store_scalar(311, 0.0);
            }
            if (s.v[813] != 0.0) {
                s.store_scalar(526, 0.0);
            }
            if (s.v[813] != 0.0) {
                s.store_scalar(527, 0.0);
            }
            s.v[814] = if (s.v[279] > 1e-8) { 1.0 } else { 0.0 };
            if ((!(s.v[813] != 0.0)) && (s.v[814] != 0.0)) {
                s.store_sqrt_ad(280, A::offset(A::add(s.ad_value(278), s.ad_value(297)), (-1.0)));
            }
            if ((!(s.v[813] != 0.0)) && (s.v[814] != 0.0)) {
                s.store_mul_ad_lhs(523, A::neg(s.ad_value(141)), 280);
            }
            if ((!(s.v[813] != 0.0)) && (s.v[814] != 0.0)) {
                s.store_div_ad(524, A::mul(A::mul(A::neg(s.ad_value(141)), s.ad_value(120)), A::sub_from_scalar(1.0, s.ad_value(284))), A::scale(s.ad_value(280), 2.0));
            }
            if ((!(s.v[813] != 0.0)) && (s.v[814] != 0.0)) {
                s.store_neg(525, 524);
            }
            if ((!(s.v[813] != 0.0)) && (s.v[814] != 0.0)) {
                s.store_exp(278, 297);
            }
            if ((!(s.v[813] != 0.0)) && (s.v[814] != 0.0)) {
                s.store_exp_ad(281, A::mul(s.ad_value(120), s.ad_value(522)));
            }
            if ((!(s.v[813] != 0.0)) && (s.v[814] != 0.0)) {
                s.store_sqrt_ad(282, A::add(A::div(A::square(s.ad_value(523)), A::square(s.ad_value(141))), A::mul(A::mul(A::scale(s.ad_value(142), 2.0), s.ad_value(281)), A::offset(A::sub(s.ad_value(278), s.ad_value(297)), (-1.0)))));
            }
            if ((!(s.v[813] != 0.0)) && (s.v[814] != 0.0)) {
                s.store_div_ad(537, A::add(A::div(A::mul(A::scale(s.ad_value(523), 2.0), s.ad_value(524)), A::square(s.ad_value(141))), A::mul(A::mul(A::mul(A::scale(s.ad_value(120), 2.0), s.ad_value(142)), s.ad_value(281)), A::offset(s.ad_value(278), (-1.0)))), A::scale(s.ad_value(282), 2.0));
            }
            if ((!(s.v[813] != 0.0)) && (s.v[814] != 0.0)) {
                s.store_div_ad(538, A::sub(A::div(A::mul(A::scale(s.ad_value(523), 2.0), s.ad_value(525)), A::square(s.ad_value(141))), A::mul(A::mul(A::mul(A::scale(s.ad_value(120), 2.0), s.ad_value(142)), s.ad_value(281)), s.ad_value(297))), A::scale(s.ad_value(282), 2.0));
            }
            if ((!(s.v[813] != 0.0)) && (s.v[814] != 0.0)) {
                s.store_sub_ad_lhs(311, A::mul(A::neg(s.ad_value(141)), s.ad_value(282)), 523);
            }
            if ((!(s.v[813] != 0.0)) && (s.v[814] != 0.0)) {
                s.store_sub_ad_lhs(526, A::mul(A::neg(s.ad_value(141)), s.ad_value(537)), 524);
            }
            if ((!(s.v[813] != 0.0)) && (s.v[814] != 0.0)) {
                s.store_sub_ad_lhs(527, A::mul(A::neg(s.ad_value(141)), s.ad_value(538)), 525);
            }
            if ((!(s.v[813] != 0.0)) && (!(s.v[814] != 0.0))) {
                s.store_scale_ad(523, A::mul(A::neg(s.ad_value(141)), s.ad_value(297)), 1.0 / (((2.0) as f64).sqrt()));
            }
            if ((!(s.v[813] != 0.0)) && (!(s.v[814] != 0.0))) {
                s.store_scale_ad(524, A::mul(A::neg(s.ad_value(141)), s.ad_value(120)), 1.0 / (((2.0) as f64).sqrt()));
            }
            if ((!(s.v[813] != 0.0)) && (!(s.v[814] != 0.0))) {
                s.store_neg(525, 524);
            }
            if ((!(s.v[813] != 0.0)) && (!(s.v[814] != 0.0))) {
                s.store_scalar(311, 0.0);
            }
            if ((!(s.v[813] != 0.0)) && (!(s.v[814] != 0.0))) {
                s.store_scalar(526, 0.0);
            }
            if ((!(s.v[813] != 0.0)) && (!(s.v[814] != 0.0))) {
                s.store_scalar(527, 0.0);
            }
            s.store_sub(279, 306, 522);
            s.store_mul(297, 120, 279);
            s.v[815] = if ((-s.v[297]) >= 80.0) { 1.0 } else { 0.0 };
            if (s.v[815] != 0.0) {
                s.store_scale_ad(278, A::offset(A::sub_from_scalar(1.0, s.ad_value(297)), (-80.0)), 5.540622384e34);
            }
            if (s.v[815] != 0.0) {
                s.store_scalar(284, 5.540622384e34);
            }
            if (!(s.v[815] != 0.0)) {
                s.store_exp_ad(278, A::neg(s.ad_value(297)));
            }
            if (!(s.v[815] != 0.0)) {
                s.copy_ad(284, 278);
            }
            s.v[816] = if (s.v[279] < (-1e-8)) { 1.0 } else { 0.0 };
            if (s.v[816] != 0.0) {
                s.store_sqrt_ad(280, A::offset(A::add(s.ad_value(278), s.ad_value(297)), (-1.0)));
            }
            if (s.v[816] != 0.0) {
                s.store_mul(531, 141, 280);
            }
            if (s.v[816] != 0.0) {
                s.store_div_ad(532, A::mul(A::mul(s.ad_value(141), s.ad_value(120)), A::sub_from_scalar(1.0, s.ad_value(284))), A::scale(s.ad_value(280), 2.0));
            }
            if (s.v[816] != 0.0) {
                s.store_neg(533, 532);
            }
            if (s.v[816] != 0.0) {
                s.store_scalar(528, 0.0);
            }
            if (s.v[816] != 0.0) {
                s.store_scalar(529, 0.0);
            }
            if (s.v[816] != 0.0) {
                s.store_scalar(530, 0.0);
            }
            s.v[817] = if (s.v[279] > 1e-8) { 1.0 } else { 0.0 };
            if ((!(s.v[816] != 0.0)) && (s.v[817] != 0.0)) {
                s.store_sqrt_ad(280, A::offset(A::add(s.ad_value(278), s.ad_value(297)), (-1.0)));
            }
            if ((!(s.v[816] != 0.0)) && (s.v[817] != 0.0)) {
                s.store_mul_ad_lhs(531, A::neg(s.ad_value(141)), 280);
            }
            if ((!(s.v[816] != 0.0)) && (s.v[817] != 0.0)) {
                s.store_div_ad(532, A::mul(A::mul(A::neg(s.ad_value(141)), s.ad_value(120)), A::sub_from_scalar(1.0, s.ad_value(284))), A::scale(s.ad_value(280), 2.0));
            }
            if ((!(s.v[816] != 0.0)) && (s.v[817] != 0.0)) {
                s.store_neg(533, 532);
            }
            if ((!(s.v[816] != 0.0)) && (s.v[817] != 0.0)) {
                s.store_exp(278, 297);
            }
            if ((!(s.v[816] != 0.0)) && (s.v[817] != 0.0)) {
                s.store_exp_ad(281, A::mul(s.ad_value(120), s.ad_value(522)));
            }
            if ((!(s.v[816] != 0.0)) && (s.v[817] != 0.0)) {
                s.store_sqrt_ad(282, A::add(A::div(A::square(s.ad_value(531)), A::square(s.ad_value(141))), A::mul(A::mul(A::scale(s.ad_value(142), 2.0), s.ad_value(281)), A::offset(A::sub(s.ad_value(278), s.ad_value(297)), (-1.0)))));
            }
            if ((!(s.v[816] != 0.0)) && (s.v[817] != 0.0)) {
                s.store_div_ad(539, A::add(A::div(A::mul(A::scale(s.ad_value(531), 2.0), s.ad_value(532)), A::square(s.ad_value(141))), A::mul(A::mul(A::mul(A::scale(s.ad_value(120), 2.0), s.ad_value(142)), s.ad_value(281)), A::offset(s.ad_value(278), (-1.0)))), A::scale(s.ad_value(282), 2.0));
            }
            if ((!(s.v[816] != 0.0)) && (s.v[817] != 0.0)) {
                s.store_div_ad(538, A::sub(A::div(A::mul(A::scale(s.ad_value(531), 2.0), s.ad_value(533)), A::square(s.ad_value(141))), A::mul(A::mul(A::mul(A::scale(s.ad_value(120), 2.0), s.ad_value(142)), s.ad_value(281)), s.ad_value(297))), A::scale(s.ad_value(282), 2.0));
            }
            if ((!(s.v[816] != 0.0)) && (s.v[817] != 0.0)) {
                s.store_sub_ad_lhs(528, A::mul(A::neg(s.ad_value(141)), s.ad_value(282)), 531);
            }
            if ((!(s.v[816] != 0.0)) && (s.v[817] != 0.0)) {
                s.store_sub_ad_lhs(529, A::mul(A::neg(s.ad_value(141)), s.ad_value(539)), 532);
            }
            if ((!(s.v[816] != 0.0)) && (s.v[817] != 0.0)) {
                s.store_sub_ad_lhs(530, A::mul(A::neg(s.ad_value(141)), s.ad_value(538)), 533);
            }
            if ((!(s.v[816] != 0.0)) && (!(s.v[817] != 0.0))) {
                s.store_scale_ad(531, A::mul(A::neg(s.ad_value(141)), s.ad_value(297)), 1.0 / (((2.0) as f64).sqrt()));
            }
            if ((!(s.v[816] != 0.0)) && (!(s.v[817] != 0.0))) {
                s.store_scale_ad(532, A::mul(A::neg(s.ad_value(141)), s.ad_value(120)), 1.0 / (((2.0) as f64).sqrt()));
            }
            if ((!(s.v[816] != 0.0)) && (!(s.v[817] != 0.0))) {
                s.store_neg(533, 532);
            }
            if ((!(s.v[816] != 0.0)) && (!(s.v[817] != 0.0))) {
                s.store_scalar(528, 0.0);
            }
            if ((!(s.v[816] != 0.0)) && (!(s.v[817] != 0.0))) {
                s.store_scalar(529, 0.0);
            }
            if ((!(s.v[816] != 0.0)) && (!(s.v[817] != 0.0))) {
                s.store_scalar(530, 0.0);
            }
            s.v[818] = if (s.v[379] == 1.0) { 1.0 } else { 0.0 };
            if (s.v[818] != 0.0) {
                s.store_scalar(574, s.v[62]);
            }
            if (s.v[818] != 0.0) {
                s.store_scalar(62, s.v[28]);
            }
            if (!(s.v[818] != 0.0)) {
                s.store_sub_ad(346, A::sub(s.ad_value(305), s.ad_value(76)), A::div(A::add(A::add(A::add(A::add(A::add(s.ad_value(312), s.ad_value(311)), s.ad_value(523)), s.ad_value(528)), s.ad_value(531)), s.ad_value(337)), s.ad_value(270)));
            }
            if (!(s.v[818] != 0.0)) {
                s.store_sub_from_scalar_ad(347, 1.0, A::div(A::add(s.ad_value(526), s.ad_value(524)), s.ad_value(270)));
            }
            if (!(s.v[818] != 0.0)) {
                s.store_div_ad_lhs(348, A::neg(A::add(A::add(A::add(s.ad_value(527), s.ad_value(525)), s.ad_value(530)), s.ad_value(533))), 270);
            }
            if (!(s.v[818] != 0.0)) {
                s.store_div_ad_lhs(349, A::neg(A::add(s.ad_value(343), A::mul(A::add(s.ad_value(529), s.ad_value(532)), s.ad_value(583)))), 270);
            }
            s.v[819] = if (s.v[312] <= s.v[599]) { 1.0 } else { 0.0 };
            if ((!(s.v[818] != 0.0)) && (s.v[819] != 0.0)) {
                s.store_sqrt_ad(279, A::mul(s.ad_value(296), A::add(A::scale(s.ad_value(312), 2.0), s.ad_value(296))));
            }
            if ((!(s.v[818] != 0.0)) && (s.v[819] != 0.0)) {
                s.store_div_ad_lhs(604, A::mul(s.ad_value(296), s.ad_value(343)), 279);
            }
            s.v[820] = if (s.v[312] <= s.v[603]) { 1.0 } else { 0.0 };
            if (((!(s.v[818] != 0.0)) && (!(s.v[819] != 0.0))) && (s.v[820] != 0.0)) {
                s.store_mul_ad(279, A::mul(A::mul(A::mul(s.ad_value(601), A::sub(s.ad_value(312), s.ad_value(603))), A::sub(s.ad_value(312), s.ad_value(603))), A::sub(s.ad_value(312), s.ad_value(603))), A::sub(s.ad_value(312), s.ad_value(602)));
            }
            if (((!(s.v[818] != 0.0)) && (!(s.v[819] != 0.0))) && (s.v[820] != 0.0)) {
                s.store_mul_ad_lhs(604, A::mul(A::mul(A::mul(s.ad_value(601), A::sub(s.ad_value(312), s.ad_value(603))), A::sub(s.ad_value(312), s.ad_value(603))), A::add(A::scale(A::sub(s.ad_value(312), s.ad_value(602)), 3.0), A::sub(s.ad_value(312), s.ad_value(603)))), 343);
            }
            if (((!(s.v[818] != 0.0)) && (!(s.v[819] != 0.0))) && (!(s.v[820] != 0.0))) {
                s.store_scalar(279, 0.0);
            }
            if (((!(s.v[818] != 0.0)) && (!(s.v[819] != 0.0))) && (!(s.v[820] != 0.0))) {
                s.store_scalar(604, 0.0);
            }
            if (!(s.v[818] != 0.0)) {
                s.store_scale_ad(281, A::div(A::neg(s.ad_value(316)), s.ad_value(296)), s.v[650]);
            }
            if (!(s.v[818] != 0.0)) {
                s.store_div_from_scalar_ad(280, 1.0, A::offset(A::exp(A::neg(s.ad_value(281))), 1.0));
            }
            if (!(s.v[818] != 0.0)) {
                s.store_mul_ad(278, A::square(s.ad_value(280)), A::exp(A::neg(s.ad_value(281))));
            }
            if (!(s.v[818] != 0.0)) {
                s.store_mul(280, 280, 600);
            }
            if (!(s.v[818] != 0.0)) {
                s.store_neg_ad(279, A::add(s.ad_value(296), s.ad_value(280)));
            }
            if (!(s.v[818] != 0.0)) {
                s.store_scalar(604, 0.0);
            }
            if (!(s.v[818] != 0.0)) {
                s.store_scaled_add(350, 523, 279, 1.0 / (s.v[535]));
            }
            if (!(s.v[818] != 0.0)) {
                s.store_scale(351, 524, 1.0 / (s.v[535]));
            }
            if (!(s.v[818] != 0.0)) {
                s.store_scale(352, 525, 1.0 / (s.v[535]));
            }
            if (!(s.v[818] != 0.0)) {
                s.store_scale(353, 604, 1.0 / (s.v[535]));
            }
            if (!(s.v[818] != 0.0)) {
                s.store_scale_ad(281, A::div(A::neg(s.ad_value(316)), s.ad_value(296)), s.v[651]);
            }
            if (!(s.v[818] != 0.0)) {
                s.store_div_from_scalar_ad(280, 1.0, A::offset(A::exp(A::neg(s.ad_value(281))), 1.0));
            }
            if (!(s.v[818] != 0.0)) {
                s.store_mul_ad(278, A::square(s.ad_value(280)), A::exp(A::neg(s.ad_value(281))));
            }
            if (!(s.v[818] != 0.0)) {
                s.store_mul(280, 280, 600);
            }
            if (!(s.v[818] != 0.0)) {
                s.store_scalar(605, 0.0);
            }
            if (!(s.v[818] != 0.0)) {
                s.store_scaled_add(354, 531, 280, 1.0 / (s.v[535]));
            }
            if (!(s.v[818] != 0.0)) {
                s.store_scale(355, 533, 1.0 / (s.v[535]));
            }
            if (!(s.v[818] != 0.0)) {
                s.store_scale_ad(356, A::add(A::mul(s.ad_value(532), s.ad_value(583)), s.ad_value(605)), 1.0 / (s.v[535]));
            }
            if (!(s.v[818] != 0.0)) {
                s.store_add_ad(357, A::sub(A::sub(A::mul(A::mul(s.ad_value(347), s.ad_value(352)), s.ad_value(356)), A::mul(A::mul(s.ad_value(347), s.ad_value(353)), s.ad_value(355))), A::mul(A::mul(s.ad_value(348), s.ad_value(351)), s.ad_value(356))), A::mul(A::mul(s.ad_value(349), s.ad_value(351)), s.ad_value(355)));
            }
            s.v[821] = if (s.v[357] > 0.0) { 1.0 } else { 0.0 };
            if ((!(s.v[818] != 0.0)) && (s.v[821] != 0.0)) {
                s.store_div_from_scalar_ad(358, 1.0, A::offset(s.ad_value(357), 1e-50));
            }
            if ((!(s.v[818] != 0.0)) && (!(s.v[821] != 0.0))) {
                s.store_div_from_scalar_ad(358, 1.0, A::offset(s.ad_value(357), (-1e-50)));
            }
            if (!(s.v[818] != 0.0)) {
                s.store_sub_ad(359, A::mul(s.ad_value(352), s.ad_value(356)), A::mul(s.ad_value(353), s.ad_value(355)));
            }
            if (!(s.v[818] != 0.0)) {
                s.store_sub_ad(360, A::mul(s.ad_value(349), s.ad_value(355)), A::mul(s.ad_value(348), s.ad_value(356)));
            }
            if (!(s.v[818] != 0.0)) {
                s.store_sub_ad(361, A::mul(s.ad_value(348), s.ad_value(353)), A::mul(s.ad_value(349), s.ad_value(352)));
            }
            if (!(s.v[818] != 0.0)) {
                s.store_mul_ad_lhs(362, A::neg(s.ad_value(351)), 356);
            }
            if (!(s.v[818] != 0.0)) {
                s.store_mul(363, 347, 356);
            }
            if (!(s.v[818] != 0.0)) {
                s.store_sub_ad(364, A::mul(s.ad_value(349), s.ad_value(351)), A::mul(s.ad_value(347), s.ad_value(353)));
            }
            if (!(s.v[818] != 0.0)) {
                s.store_mul(365, 351, 355);
            }
            if (!(s.v[818] != 0.0)) {
                s.store_mul_ad_lhs(366, A::neg(s.ad_value(347)), 355);
            }
            if (!(s.v[818] != 0.0)) {
                s.store_sub_ad(367, A::mul(s.ad_value(347), s.ad_value(352)), A::mul(s.ad_value(348), s.ad_value(351)));
            }
            if (!(s.v[818] != 0.0)) {
                s.store_mul_ad(368, A::neg(s.ad_value(358)), A::add(A::add(A::mul(s.ad_value(359), s.ad_value(346)), A::mul(s.ad_value(360), s.ad_value(350))), A::mul(s.ad_value(361), s.ad_value(354))));
            }
            if (!(s.v[818] != 0.0)) {
                s.store_mul_ad(369, A::neg(s.ad_value(358)), A::add(A::add(A::mul(s.ad_value(362), s.ad_value(346)), A::mul(s.ad_value(363), s.ad_value(350))), A::mul(s.ad_value(364), s.ad_value(354))));
            }
            if (!(s.v[818] != 0.0)) {
                s.store_mul_ad(370, A::neg(s.ad_value(358)), A::add(A::add(A::mul(s.ad_value(365), s.ad_value(346)), A::mul(s.ad_value(366), s.ad_value(350))), A::mul(s.ad_value(367), s.ad_value(354))));
            }
            if (!(s.v[818] != 0.0)) {
                s.store_ad(279, &A::abs(s.ad_value(368)));
            }
            s.v[822] = if (s.v[279] < ((s.v[369]) as f64).abs()) { 1.0 } else { 0.0 };
            if ((!(s.v[818] != 0.0)) && (s.v[822] != 0.0)) {
                s.store_ad(279, &A::abs(s.ad_value(369)));
            }
            s.v[823] = if (s.v[279] < ((s.v[370]) as f64).abs()) { 1.0 } else { 0.0 };
            if ((!(s.v[818] != 0.0)) && (s.v[823] != 0.0)) {
                s.store_ad(279, &A::abs(s.ad_value(370)));
            }
            if (!(s.v[818] != 0.0)) {
                s.store_scalar(606, 1.0);
            }
            s.v[824] = if (s.v[62] > 80.0) { 1.0 } else { 0.0 };
            if ((!(s.v[818] != 0.0)) && (s.v[824] != 0.0)) {
                s.store_scalar(606, 25.0);
            }
            s.v[825] = if (s.v[62] > 40.0) { 1.0 } else { 0.0 };
            if (((!(s.v[818] != 0.0)) && (!(s.v[824] != 0.0))) && (s.v[825] != 0.0)) {
                s.store_scalar(606, 25.0);
            }
            s.v[826] = if (s.v[62] > 20.0) { 1.0 } else { 0.0 };
            if ((((!(s.v[818] != 0.0)) && (!(s.v[824] != 0.0))) && (!(s.v[825] != 0.0))) && (s.v[826] != 0.0)) {
                s.store_scalar(606, 25.0);
            }
            s.v[827] = if (s.v[62] > 10.0) { 1.0 } else { 0.0 };
            if (((((!(s.v[818] != 0.0)) && (!(s.v[824] != 0.0))) && (!(s.v[825] != 0.0))) && (!(s.v[826] != 0.0))) && (s.v[827] != 0.0)) {
                s.store_scalar(606, 5.0);
            }
            s.v[828] = if (s.v[279] > (0.1 / s.v[606])) { 1.0 } else { 0.0 };
            if ((!(s.v[818] != 0.0)) && (s.v[828] != 0.0)) {
                s.store_mul_ad_rhs(368, 368, A::div(A::div_from_scalar(0.1, s.ad_value(606)), s.ad_value(279)));
            }
            if ((!(s.v[818] != 0.0)) && (s.v[828] != 0.0)) {
                s.store_mul_ad_rhs(369, 369, A::div(A::div_from_scalar(0.1, s.ad_value(606)), s.ad_value(279)));
            }
            if ((!(s.v[818] != 0.0)) && (s.v[828] != 0.0)) {
                s.store_mul_ad_rhs(370, 370, A::div(A::div_from_scalar(0.1, s.ad_value(606)), s.ad_value(279)));
            }
            if (!(s.v[818] != 0.0)) {
                s.store_add(305, 305, 368);
            }
            if (!(s.v[818] != 0.0)) {
                s.store_add(522, 522, 369);
            }
            if (!(s.v[818] != 0.0)) {
                s.store_add(307, 307, 370);
            }
            if (!(s.v[818] != 0.0)) {
                s.store_scale(607, 606, 1e-12);
            }
            s.v[829] = if (s.v[279] < s.v[607]) { 1.0 } else { 0.0 };
            if ((!(s.v[818] != 0.0)) && (s.v[829] != 0.0)) {
                s.store_scalar(379, 1.0);
            }
            s.store_offset(62, 62, 1.0);
        }

        s.v[830] = if (s.v[574] > 0.0) { 1.0 } else { 0.0 };

        if (s.v[830] != 0.0) {
            s.copy_ad(62, 574);
        }

        if (s.v[830] != 0.0) {
            s.store_scalar(574, 0.0);
        }

        s.v[831] = if (s.v[62] > s.v[28]) { 1.0 } else { 0.0 };

        if (s.v[831] != 0.0) {
            s.copy_ad(305, 299);
        }

        if (s.v[831] != 0.0) {
            s.copy_ad(306, 300);
        }

        if (s.v[831] != 0.0) {
            s.copy_ad(307, 301);
        }

        if (s.v[831] != 0.0) {
            s.copy_ad(522, 534);
        }

        s.copy_ad(56, 305);

        s.store_neg(149, 311);

        s.v[833] = if (s.v[149] <= 1e-50) { 1.0 } else { 0.0 };

        if (s.v[833] != 0.0) {
            s.store_scalar(149, 1e-50);
        }

        if (s.v[833] != 0.0) {
            s.store_scalar(34, 1.0);
        }

        s.store_neg(150, 528);

        s.v[834] = if (s.v[150] <= 1e-50) { 1.0 } else { 0.0 };

        if (s.v[834] != 0.0) {
            s.store_scalar(150, 1e-50);
        }

        s.store_mul(86, 149, 271);

        s.copy_ad(396, 51);

        s.store_div_ad_rhs(280, 472, A::square(s.ad_value(270)));

        s.store_sub(278, 76, 122);

        s.store_offset_ad(287, A::mul(A::div_from_scalar(2.0, s.ad_value(280)), s.ad_value(278)), 1.0);

        s.store_sqrt_ad(639, A::offset(A::square(s.ad_value(287)), ((4.0 * 0.05) * 0.05)));

        s.store_scale_ad(284, A::offset(A::div(s.ad_value(287), s.ad_value(639)), 1.0), 0.5);

        s.store_offset_ad(287, A::scale(A::add(s.ad_value(287), s.ad_value(639)), 0.5), (1e-10 * 0.05));

        s.v[835] = if (s.v[287] < 0.0) { 1.0 } else { 0.0 };

        if (s.v[835] != 0.0) {
            s.store_scalar(287, 0.0);
        }

        if (s.v[835] != 0.0) {
            s.store_scalar(284, 0.0);
        }

        s.store_sqrt(281, 287);

        s.store_add_ad_rhs(288, 76, A::mul(s.ad_value(280), A::sub_from_scalar(1.0, s.ad_value(281))));

        s.store_sqrt_ad(639, A::offset(A::square(s.ad_value(288)), ((4.0 * 0.01) * 0.01)));

        s.store_scale_ad(278, A::offset(A::div(s.ad_value(288), s.ad_value(639)), 1.0), 0.5);

        s.store_offset_ad(288, A::scale(A::add(s.ad_value(288), s.ad_value(639)), 0.5), (1e-10 * 0.01));

        s.v[836] = if (s.v[288] < 0.0) { 1.0 } else { 0.0 };

        if (s.v[836] != 0.0) {
            s.store_scalar(288, 0.0);
        }

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
        if (s.v[836] != 0.0) {
            s.store_scalar(278, 0.0);
        }

        s.copy_ad(89, 288);

        s.store_offset_ad(279, A::div(s.ad_value(51), s.ad_value(89)), 1e-50);

        s.store_powf(280, 279, (s.v[481] - 1.0));

        s.store_offset_ad(281, A::mul(s.ad_value(280), s.ad_value(279)), 1.0);

        s.store_powf(282, 281, ((1.0 / s.v[481]) - 1.0));

        s.store_mul(284, 282, 281);

        s.store_div(395, 51, 284);

        s.copy_ad(51, 395);

        s.v[837] = if (s.v[51] < 0.0) { 1.0 } else { 0.0 };

        if (s.v[837] != 0.0) {
            s.copy_ad(57, 56);
        }

        if (s.v[837] != 0.0) {
            s.store_sub(59, 57, 56);
        }

        if (s.v[837] != 0.0) {
            s.copy_ad(308, 57);
        }

        if (s.v[837] != 0.0) {
            s.copy_ad(309, 306);
        }

        if (s.v[837] != 0.0) {
            s.copy_ad(584, 522);
        }

        if (s.v[837] != 0.0) {
            s.copy_ad(310, 307);
        }

        if (s.v[837] != 0.0) {
            s.store_scalar(379, 1.0);
        }

        s.v[838] = if ((s.v[33] >= 1.0) || (s.v[86] < 1e-12)) { 1.0 } else { 0.0 };

        if ((!(s.v[837] != 0.0)) && (s.v[838] != 0.0)) {
            s.store_scalar(308, s.v[698]);
        }

        if ((!(s.v[837] != 0.0)) && (s.v[838] != 0.0)) {
            s.store_scalar(309, s.v[699]);
        }

        if ((!(s.v[837] != 0.0)) && (s.v[838] != 0.0)) {
            s.store_offset(310, 440, s.v[700]);
        }

        if ((!(s.v[837] != 0.0)) && (!(s.v[838] != 0.0))) {
            s.store_ad(61, &{
                if ((s.v[58] - s.v[305]) >= 0.0) {
                    A::sub(s.ad_value(58), s.ad_value(305))
                } else {
                    A::constant(0.0)
                }
            });
        }

        if ((!(s.v[837] != 0.0)) && (!(s.v[838] != 0.0))) {
            s.store_offset_ad(638, A::sub(A::scale(s.ad_value(61), (1.0 + (0.3 * 0.5))), s.ad_value(51)), (-0.03));
        }

        if ((!(s.v[837] != 0.0)) && (!(s.v[838] != 0.0))) {
            s.store_scale(639, 61, ((1.0 + (0.3 * 0.5)) * (4.0 * 0.03)));
        }

        if ((!(s.v[837] != 0.0)) && (!(s.v[838] != 0.0))) {
            s.store_ad(639, &{
                if (s.v[639] > 0.0) {
                    s.ad_value(639)
                } else {
                    A::neg(s.ad_value(639))
                }
            });
        }

        if ((!(s.v[837] != 0.0)) && (!(s.v[838] != 0.0))) {
            s.store_sqrt_ad(639, A::add(A::square(s.ad_value(638)), s.ad_value(639)));
        }

        if ((!(s.v[837] != 0.0)) && (!(s.v[838] != 0.0))) {
            s.store_scale_ad(279, A::offset(A::div(s.ad_value(638), s.ad_value(639)), 1.0), 0.5);
        }

        if ((!(s.v[837] != 0.0)) && (!(s.v[838] != 0.0))) {
            s.store_sub_ad(60, A::scale(s.ad_value(61), (1.0 + (0.3 * 0.5))), A::scale(A::add(s.ad_value(638), s.ad_value(639)), 0.5));
        }

        if ((!(s.v[837] != 0.0)) && (!(s.v[838] != 0.0))) {
            s.store_ad(60, &{
                if (s.v[60] <= s.v[61]) {
                    s.ad_value(60)
                } else {
                    s.ad_value(61)
                }
            });
        }

        s.v[839] = if (s.v[60] < 0.0) { 1.0 } else { 0.0 };

        if (((!(s.v[837] != 0.0)) && (!(s.v[838] != 0.0))) && (s.v[839] != 0.0)) {
            s.store_scalar(60, 0.0);
        }

        s.v[840] = if (s.v[60] > s.v[51]) { 1.0 } else { 0.0 };

        if ((((!(s.v[837] != 0.0)) && (!(s.v[838] != 0.0))) && (!(s.v[839] != 0.0))) && (s.v[840] != 0.0)) {
            s.copy_ad(60, 51);
        }

        if ((!(s.v[837] != 0.0)) && (!(s.v[838] != 0.0))) {
            s.copy_ad(59, 60);
        }

        if ((!(s.v[837] != 0.0)) && (!(s.v[838] != 0.0))) {
            s.store_add(57, 305, 59);
        }

        if ((!(s.v[837] != 0.0)) && (!(s.v[838] != 0.0))) {
            s.store_scalar(290, (1e-12 / 2.0));
        }

        s.v[841] = if (s.v[57] < s.v[290]) { 1.0 } else { 0.0 };

        if (((!(s.v[837] != 0.0)) && (!(s.v[838] != 0.0))) && (s.v[841] != 0.0)) {
            s.copy_ad(57, 290);
        }

        if ((!(s.v[837] != 0.0)) && (!(s.v[838] != 0.0))) {
            s.copy_ad(308, 57);
        }

        if ((!(s.v[837] != 0.0)) && (!(s.v[838] != 0.0))) {
            s.store_ad(308, &{
                if (s.v[292] == (-1.0)) {
                    s.ad_value(305)
                } else {
                    s.ad_value(57)
                }
            });
        }

        if ((!(s.v[837] != 0.0)) && (!(s.v[838] != 0.0))) {
            s.store_scale_ad(278, A::square(s.ad_value(439)), (s.v[293] * s.v[293]));
        }

        s.v[842] = if (s.v[308] < s.v[329]) { 1.0 } else { 0.0 };

        if (((!(s.v[837] != 0.0)) && (!(s.v[838] != 0.0))) && (s.v[842] != 0.0)) {
            s.store_neg(279, 440);
        }

        if (((!(s.v[837] != 0.0)) && (!(s.v[838] != 0.0))) && (s.v[842] != 0.0)) {
            s.store_sub_ad(280, A::mul(A::add(A::scale(s.ad_value(279), 2.0), A::mul(s.ad_value(278), s.ad_value(120))), A::add(A::scale(s.ad_value(279), 2.0), A::mul(s.ad_value(278), s.ad_value(120)))), A::scale(A::add(A::square(s.ad_value(279)), s.ad_value(278)), 4.0));
        }

        if (((!(s.v[837] != 0.0)) && (!(s.v[838] != 0.0))) && (s.v[842] != 0.0)) {
            s.store_ad(280, &{
                if (s.v[280] >= (10.0 * 2.220446049250313e-16)) {
                    s.ad_value(280)
                } else {
                    A::constant((10.0 * 2.220446049250313e-16))
                }
            });
        }

        if (((!(s.v[837] != 0.0)) && (!(s.v[838] != 0.0))) && (s.v[842] != 0.0)) {
            s.store_scale_ad(324, A::sub(A::add(A::scale(s.ad_value(279), 2.0), A::mul(s.ad_value(278), s.ad_value(120))), A::sqrt(s.ad_value(280))), 0.5);
        }

        if (((!(s.v[837] != 0.0)) && (!(s.v[838] != 0.0))) && (s.v[842] != 0.0)) {
            s.store_div_ad(325, A::ln(A::div(A::div(A::square(s.ad_value(279)), s.ad_value(278)), s.ad_value(143))), A::add(s.ad_value(120), A::div_from_scalar(2.0, s.ad_value(279))));
        }

        s.v[843] = if (s.v[324] < s.v[326]) { 1.0 } else { 0.0 };

        if ((((!(s.v[837] != 0.0)) && (!(s.v[838] != 0.0))) && (s.v[842] != 0.0)) && (s.v[843] != 0.0)) {
            s.copy_ad(310, 324);
        }

        if ((((!(s.v[837] != 0.0)) && (!(s.v[838] != 0.0))) && (s.v[842] != 0.0)) && (!(s.v[843] != 0.0))) {
            s.store_offset_ad(638, A::sub(s.ad_value(325), s.ad_value(324)), (-0.0008));
        }

        if ((((!(s.v[837] != 0.0)) && (!(s.v[838] != 0.0))) && (s.v[842] != 0.0)) && (!(s.v[843] != 0.0))) {
            s.store_scale(639, 325, (4.0 * 0.0008));
        }

        if ((((!(s.v[837] != 0.0)) && (!(s.v[838] != 0.0))) && (s.v[842] != 0.0)) && (!(s.v[843] != 0.0))) {
            s.store_ad(639, &{
                if (s.v[639] > 0.0) {
                    s.ad_value(639)
                } else {
                    A::neg(s.ad_value(639))
                }
            });
        }

        if ((((!(s.v[837] != 0.0)) && (!(s.v[838] != 0.0))) && (s.v[842] != 0.0)) && (!(s.v[843] != 0.0))) {
            s.store_sqrt_ad(639, A::add(A::square(s.ad_value(638)), s.ad_value(639)));
        }

        if ((((!(s.v[837] != 0.0)) && (!(s.v[838] != 0.0))) && (s.v[842] != 0.0)) && (!(s.v[843] != 0.0))) {
            s.store_scale_ad(279, A::offset(A::div(s.ad_value(638), s.ad_value(639)), 1.0), 0.5);
        }

        if ((((!(s.v[837] != 0.0)) && (!(s.v[838] != 0.0))) && (s.v[842] != 0.0)) && (!(s.v[843] != 0.0))) {
            s.store_sub_ad_rhs(310, 325, A::scale(A::add(s.ad_value(638), s.ad_value(639)), 0.5));
        }

        if (((!(s.v[837] != 0.0)) && (!(s.v[838] != 0.0))) && (!(s.v[842] != 0.0))) {
            s.store_neg_ad(279, A::sub(A::sub(s.ad_value(440), s.ad_value(308)), A::scale(A::scale(s.ad_value(296), 0.5), s.v[536])));
        }

        if (((!(s.v[837] != 0.0)) && (!(s.v[838] != 0.0))) && (!(s.v[842] != 0.0))) {
            s.store_sub_ad(280, A::mul(A::add(A::scale(s.ad_value(279), 2.0), A::mul(s.ad_value(278), s.ad_value(120))), A::add(A::scale(s.ad_value(279), 2.0), A::mul(s.ad_value(278), s.ad_value(120)))), A::scale(A::add(A::square(s.ad_value(279)), s.ad_value(278)), 4.0));
        }

        if (((!(s.v[837] != 0.0)) && (!(s.v[838] != 0.0))) && (!(s.v[842] != 0.0))) {
            s.store_ad(280, &{
                if (s.v[280] >= (10.0 * 2.220446049250313e-16)) {
                    s.ad_value(280)
                } else {
                    A::constant((10.0 * 2.220446049250313e-16))
                }
            });
        }

        if (((!(s.v[837] != 0.0)) && (!(s.v[838] != 0.0))) && (!(s.v[842] != 0.0))) {
            s.store_scale_ad(324, A::sub(A::add(A::scale(s.ad_value(279), 2.0), A::mul(s.ad_value(278), s.ad_value(120))), A::sqrt(s.ad_value(280))), 0.5);
        }

        if (((!(s.v[837] != 0.0)) && (!(s.v[838] != 0.0))) && (!(s.v[842] != 0.0))) {
            s.store_div_ad(325, A::ln(A::div(A::div(A::square(s.ad_value(279)), s.ad_value(278)), s.ad_value(143))), A::add(s.ad_value(120), A::div_from_scalar(2.0, s.ad_value(279))));
        }

        s.v[844] = if (s.v[324] < s.v[326]) { 1.0 } else { 0.0 };

        if ((((!(s.v[837] != 0.0)) && (!(s.v[838] != 0.0))) && (!(s.v[842] != 0.0))) && (s.v[844] != 0.0)) {
            s.copy_ad(310, 324);
        }

        if ((((!(s.v[837] != 0.0)) && (!(s.v[838] != 0.0))) && (!(s.v[842] != 0.0))) && (!(s.v[844] != 0.0))) {
            s.store_offset_ad(638, A::sub(s.ad_value(325), s.ad_value(324)), (-0.0008));
        }

        if ((((!(s.v[837] != 0.0)) && (!(s.v[838] != 0.0))) && (!(s.v[842] != 0.0))) && (!(s.v[844] != 0.0))) {
            s.store_scale(639, 325, (4.0 * 0.0008));
        }

        if ((((!(s.v[837] != 0.0)) && (!(s.v[838] != 0.0))) && (!(s.v[842] != 0.0))) && (!(s.v[844] != 0.0))) {
            s.store_ad(639, &{
                if (s.v[639] > 0.0) {
                    s.ad_value(639)
                } else {
                    A::neg(s.ad_value(639))
                }
            });
        }

        if ((((!(s.v[837] != 0.0)) && (!(s.v[838] != 0.0))) && (!(s.v[842] != 0.0))) && (!(s.v[844] != 0.0))) {
            s.store_sqrt_ad(639, A::add(A::square(s.ad_value(638)), s.ad_value(639)));
        }

        if ((((!(s.v[837] != 0.0)) && (!(s.v[838] != 0.0))) && (!(s.v[842] != 0.0))) && (!(s.v[844] != 0.0))) {
            s.store_scale_ad(279, A::offset(A::div(s.ad_value(638), s.ad_value(639)), 1.0), 0.5);
        }

        if ((((!(s.v[837] != 0.0)) && (!(s.v[838] != 0.0))) && (!(s.v[842] != 0.0))) && (!(s.v[844] != 0.0))) {
            s.store_sub_ad_rhs(310, 325, A::scale(A::add(s.ad_value(638), s.ad_value(639)), 0.5));
        }

        s.v[845] = if ((s.v[308] < s.v[329]) && (0.0 != 0.0)) { 1.0 } else { 0.0 };

        if (((!(s.v[837] != 0.0)) && (!(s.v[838] != 0.0))) && (s.v[845] != 0.0)) {
            s.store_scalar(63, 0.0);
        }

        let mut assign11450_loop_guard: usize = 0;
        while {
            let assign11450_cond_e13817: f64 = if ((((!(s.v[837] != 0.0)) && (!(s.v[838] != 0.0))) && (s.v[845] != 0.0)) && (s.v[63] < s.v[29])) { 1.0 } else { 0.0 };
            assign11450_cond_e13817 != 0.0
        } {
            assign11450_loop_guard += 1;
            assert!(assign11450_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((!(s.v[837] != 0.0)) && (!(s.v[838] != 0.0))) && (s.v[845] != 0.0)) {
                s.store_mul(280, 120, 310);
            }
            if (((!(s.v[837] != 0.0)) && (!(s.v[838] != 0.0))) && (s.v[845] != 0.0)) {
                s.store_exp_ad(281, A::neg(s.ad_value(280)));
            }
            s.v[846] = if (s.v[310] > 1e-8) { 1.0 } else { 0.0 };
            if ((((!(s.v[837] != 0.0)) && (!(s.v[838] != 0.0))) && (s.v[845] != 0.0)) && (s.v[846] != 0.0)) {
                s.store_exp_ad(278, A::mul(s.ad_value(120), s.ad_value(310)));
            }
            if ((((!(s.v[837] != 0.0)) && (!(s.v[838] != 0.0))) && (s.v[845] != 0.0)) && (s.v[846] != 0.0)) {
                s.store_mul_ad(282, A::neg(s.ad_value(439)), A::sqrt(A::add(A::offset(A::add(s.ad_value(281), s.ad_value(280)), (-1.0)), A::mul(s.ad_value(143), A::offset(s.ad_value(278), (-1.0))))));
            }
            if ((((!(s.v[837] != 0.0)) && (!(s.v[838] != 0.0))) && (s.v[845] != 0.0)) && (s.v[846] != 0.0)) {
                s.store_mul_ad(283, A::div(s.ad_value(438), s.ad_value(282)), A::add(A::sub_from_scalar(1.0, s.ad_value(281)), A::mul(s.ad_value(143), s.ad_value(278))));
            }
            s.v[847] = if (s.v[310] < (-1e-8)) { 1.0 } else { 0.0 };
            if (((((!(s.v[837] != 0.0)) && (!(s.v[838] != 0.0))) && (s.v[845] != 0.0)) && (!(s.v[846] != 0.0))) && (s.v[847] != 0.0)) {
                s.store_mul_ad_rhs(282, 439, A::sqrt(A::offset(A::add(s.ad_value(281), s.ad_value(280)), (-1.0))));
            }
            if (((((!(s.v[837] != 0.0)) && (!(s.v[838] != 0.0))) && (s.v[845] != 0.0)) && (!(s.v[846] != 0.0))) && (s.v[847] != 0.0)) {
                s.store_mul_ad(283, A::div(s.ad_value(438), s.ad_value(282)), A::sub_from_scalar(1.0, s.ad_value(281)));
            }
            if (((((!(s.v[837] != 0.0)) && (!(s.v[838] != 0.0))) && (s.v[845] != 0.0)) && (!(s.v[846] != 0.0))) && (!(s.v[847] != 0.0))) {
                s.store_mul_ad_lhs(282, A::mul(A::neg(A::sqrt(A::div(s.ad_value(438), s.ad_value(120)))), s.ad_value(120)), 310);
            }
            if (((((!(s.v[837] != 0.0)) && (!(s.v[838] != 0.0))) && (s.v[845] != 0.0)) && (!(s.v[846] != 0.0))) && (!(s.v[847] != 0.0))) {
                s.store_neg_ad(283, A::sqrt(A::mul(s.ad_value(438), s.ad_value(120))));
            }
            if (((!(s.v[837] != 0.0)) && (!(s.v[838] != 0.0))) && (s.v[845] != 0.0)) {
                s.store_sqrt_ad(639, A::offset(A::square(s.ad_value(282)), ((4.0 * 1e-6) * 1e-6)));
            }
            if (((!(s.v[837] != 0.0)) && (!(s.v[838] != 0.0))) && (s.v[845] != 0.0)) {
                s.store_scale_ad(285, A::offset(A::div(s.ad_value(282), s.ad_value(639)), 1.0), 0.5);
            }
            if (((!(s.v[837] != 0.0)) && (!(s.v[838] != 0.0))) && (s.v[845] != 0.0)) {
                s.store_offset_ad(284, A::scale(A::add(s.ad_value(282), s.ad_value(639)), 0.5), (1e-10 * 1e-6));
            }
            s.v[848] = if (s.v[284] < 0.0) { 1.0 } else { 0.0 };
            if ((((!(s.v[837] != 0.0)) && (!(s.v[838] != 0.0))) && (s.v[845] != 0.0)) && (s.v[848] != 0.0)) {
                s.store_scalar(284, 0.0);
            }
            if ((((!(s.v[837] != 0.0)) && (!(s.v[838] != 0.0))) && (s.v[845] != 0.0)) && (s.v[848] != 0.0)) {
                s.store_scalar(285, 0.0);
            }
            if (((!(s.v[837] != 0.0)) && (!(s.v[838] != 0.0))) && (s.v[845] != 0.0)) {
                s.store_offset_ad(638, A::sub(A::neg(s.ad_value(296)), s.ad_value(284)), (-1e-9));
            }
            if (((!(s.v[837] != 0.0)) && (!(s.v[838] != 0.0))) && (s.v[845] != 0.0)) {
                s.store_scale_ad(639, A::neg(s.ad_value(296)), (4.0 * 1e-9));
            }
            if (((!(s.v[837] != 0.0)) && (!(s.v[838] != 0.0))) && (s.v[845] != 0.0)) {
                s.store_ad(639, &{
                    if (s.v[639] > 0.0) {
                        s.ad_value(639)
                    } else {
                        A::neg(s.ad_value(639))
                    }
                });
            }
            if (((!(s.v[837] != 0.0)) && (!(s.v[838] != 0.0))) && (s.v[845] != 0.0)) {
                s.store_sqrt_ad(639, A::add(A::square(s.ad_value(638)), s.ad_value(639)));
            }
            if (((!(s.v[837] != 0.0)) && (!(s.v[838] != 0.0))) && (s.v[845] != 0.0)) {
                s.store_scale_ad(286, A::offset(A::div(s.ad_value(638), s.ad_value(639)), 1.0), 0.5);
            }
            if (((!(s.v[837] != 0.0)) && (!(s.v[838] != 0.0))) && (s.v[845] != 0.0)) {
                s.store_sub_ad(284, A::neg(s.ad_value(296)), A::scale(A::add(s.ad_value(638), s.ad_value(639)), 0.5));
            }
            if (((!(s.v[837] != 0.0)) && (!(s.v[838] != 0.0))) && (s.v[845] != 0.0)) {
                s.store_mul_ad_rhs(285, 285, A::mul(s.ad_value(283), s.ad_value(286)));
            }
            if (((!(s.v[837] != 0.0)) && (!(s.v[838] != 0.0))) && (s.v[845] != 0.0)) {
                s.store_div_ad_lhs(332, A::scale(A::scale(A::scale(A::square(s.ad_value(284)), 0.5), 9662367879.197212), 6.241449993689894e18), 471);
            }
            if (((!(s.v[837] != 0.0)) && (!(s.v[838] != 0.0))) && (s.v[845] != 0.0)) {
                s.store_div_ad_lhs(333, A::mul(A::scale(s.ad_value(332), 2.0), s.ad_value(285)), 284);
            }
            if (((!(s.v[837] != 0.0)) && (!(s.v[838] != 0.0))) && (s.v[845] != 0.0)) {
                s.store_sub_ad_rhs(284, 310, A::div(A::add(A::sub(A::sub(A::scale(s.ad_value(282), 1.0 / (s.v[294])), s.ad_value(310)), s.ad_value(440)), s.ad_value(332)), A::add(A::offset(A::scale(s.ad_value(283), 1.0 / (s.v[294])), (-1.0)), s.ad_value(333))));
            }
            s.v[849] = if ((((s.v[284] - s.v[310])) as f64).abs() < 1e-12) { 1.0 } else { 0.0 };
            if ((((!(s.v[837] != 0.0)) && (!(s.v[838] != 0.0))) && (s.v[845] != 0.0)) && (s.v[849] != 0.0)) {
                s.store_scalar(63, s.v[29]);
            }
            if (((!(s.v[837] != 0.0)) && (!(s.v[838] != 0.0))) && (s.v[845] != 0.0)) {
                s.copy_ad(310, 284);
            }
            if (((!(s.v[837] != 0.0)) && (!(s.v[838] != 0.0))) && (s.v[845] != 0.0)) {
                s.copy_ad(314, 282);
            }
            if (((!(s.v[837] != 0.0)) && (!(s.v[838] != 0.0))) && (s.v[845] != 0.0)) {
                s.store_offset(63, 63, 1.0);
            }
        }

        if (((!(s.v[837] != 0.0)) && (!(s.v[838] != 0.0))) && (s.v[845] != 0.0)) {
            s.store_add(310, 440, 310);
        }

        if (((!(s.v[837] != 0.0)) && (!(s.v[838] != 0.0))) && (s.v[845] != 0.0)) {
            s.store_sub_ad_rhs(309, 310, A::scale(s.ad_value(314), 1.0 / (s.v[294])));
        }

        if (((!(s.v[837] != 0.0)) && (!(s.v[838] != 0.0))) && (!(s.v[845] != 0.0))) {
            s.store_scalar(63, 0.0);
        }

    }

    pub(super) fn stamp_reactive_block_10(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let mut assign11490_loop_guard: usize = 0;
        while {
            let assign11490_cond_e14353: f64 = if ((((!(s.v[837] != 0.0)) && (!(s.v[838] != 0.0))) && (!(s.v[845] != 0.0))) && (s.v[63] < s.v[29])) { 1.0 } else { 0.0 };
            assign11490_cond_e14353 != 0.0
        } {
            assign11490_loop_guard += 1;
            assert!(assign11490_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((!(s.v[837] != 0.0)) && (!(s.v[838] != 0.0))) && (!(s.v[845] != 0.0))) {
                s.copy_ad(279, 439);
            }
            if (((!(s.v[837] != 0.0)) && (!(s.v[838] != 0.0))) && (!(s.v[845] != 0.0))) {
                s.store_mul(280, 120, 310);
            }
            if (((!(s.v[837] != 0.0)) && (!(s.v[838] != 0.0))) && (!(s.v[845] != 0.0))) {
                s.store_exp_ad(281, A::neg(s.ad_value(280)));
            }
            s.v[850] = if (s.v[310] > 1e-8) { 1.0 } else { 0.0 };
            if ((((!(s.v[837] != 0.0)) && (!(s.v[838] != 0.0))) && (!(s.v[845] != 0.0))) && (s.v[850] != 0.0)) {
                s.store_exp_ad(278, A::mul(s.ad_value(120), s.ad_value(310)));
            }
            if ((((!(s.v[837] != 0.0)) && (!(s.v[838] != 0.0))) && (!(s.v[845] != 0.0))) && (s.v[850] != 0.0)) {
                s.store_mul_ad(282, A::neg(s.ad_value(279)), A::sqrt(A::add(A::offset(A::add(s.ad_value(281), s.ad_value(280)), (-1.0)), A::mul(s.ad_value(143), A::offset(s.ad_value(278), (-1.0))))));
            }
            if ((((!(s.v[837] != 0.0)) && (!(s.v[838] != 0.0))) && (!(s.v[845] != 0.0))) && (s.v[850] != 0.0)) {
                s.store_mul_ad(283, A::div(s.ad_value(438), s.ad_value(282)), A::add(A::sub_from_scalar(1.0, s.ad_value(281)), A::mul(s.ad_value(143), s.ad_value(278))));
            }
            s.v[851] = if (s.v[310] < (-1e-8)) { 1.0 } else { 0.0 };
            if (((((!(s.v[837] != 0.0)) && (!(s.v[838] != 0.0))) && (!(s.v[845] != 0.0))) && (!(s.v[850] != 0.0))) && (s.v[851] != 0.0)) {
                s.store_mul_ad_rhs(282, 279, A::sqrt(A::offset(A::add(s.ad_value(281), s.ad_value(280)), (-1.0))));
            }
            if (((((!(s.v[837] != 0.0)) && (!(s.v[838] != 0.0))) && (!(s.v[845] != 0.0))) && (!(s.v[850] != 0.0))) && (s.v[851] != 0.0)) {
                s.store_mul_ad(283, A::div(s.ad_value(438), s.ad_value(282)), A::sub_from_scalar(1.0, s.ad_value(281)));
            }
            if (((((!(s.v[837] != 0.0)) && (!(s.v[838] != 0.0))) && (!(s.v[845] != 0.0))) && (!(s.v[850] != 0.0))) && (!(s.v[851] != 0.0))) {
                s.store_mul_ad_lhs(282, A::mul(A::neg(A::sqrt(A::div(s.ad_value(438), s.ad_value(120)))), s.ad_value(120)), 310);
            }
            if (((((!(s.v[837] != 0.0)) && (!(s.v[838] != 0.0))) && (!(s.v[845] != 0.0))) && (!(s.v[850] != 0.0))) && (!(s.v[851] != 0.0))) {
                s.store_neg_ad(283, A::sqrt(A::mul(s.ad_value(438), s.ad_value(120))));
            }
            if (((!(s.v[837] != 0.0)) && (!(s.v[838] != 0.0))) && (!(s.v[845] != 0.0))) {
                s.store_sqrt_ad(639, A::offset(A::square(s.ad_value(282)), ((4.0 * 1e-6) * 1e-6)));
            }
            if (((!(s.v[837] != 0.0)) && (!(s.v[838] != 0.0))) && (!(s.v[845] != 0.0))) {
                s.store_scale_ad(285, A::offset(A::div(s.ad_value(282), s.ad_value(639)), 1.0), 0.5);
            }
            if (((!(s.v[837] != 0.0)) && (!(s.v[838] != 0.0))) && (!(s.v[845] != 0.0))) {
                s.store_offset_ad(284, A::scale(A::add(s.ad_value(282), s.ad_value(639)), 0.5), (1e-10 * 1e-6));
            }
            s.v[852] = if (s.v[284] < 0.0) { 1.0 } else { 0.0 };
            if ((((!(s.v[837] != 0.0)) && (!(s.v[838] != 0.0))) && (!(s.v[845] != 0.0))) && (s.v[852] != 0.0)) {
                s.store_scalar(284, 0.0);
            }
            if ((((!(s.v[837] != 0.0)) && (!(s.v[838] != 0.0))) && (!(s.v[845] != 0.0))) && (s.v[852] != 0.0)) {
                s.store_scalar(285, 0.0);
            }
            if (((!(s.v[837] != 0.0)) && (!(s.v[838] != 0.0))) && (!(s.v[845] != 0.0))) {
                s.store_offset_ad(638, A::sub(A::neg(s.ad_value(296)), s.ad_value(284)), (-1e-9));
            }
            if (((!(s.v[837] != 0.0)) && (!(s.v[838] != 0.0))) && (!(s.v[845] != 0.0))) {
                s.store_scale_ad(639, A::neg(s.ad_value(296)), (4.0 * 1e-9));
            }
            if (((!(s.v[837] != 0.0)) && (!(s.v[838] != 0.0))) && (!(s.v[845] != 0.0))) {
                s.store_ad(639, &{
                    if (s.v[639] > 0.0) {
                        s.ad_value(639)
                    } else {
                        A::neg(s.ad_value(639))
                    }
                });
            }
            if (((!(s.v[837] != 0.0)) && (!(s.v[838] != 0.0))) && (!(s.v[845] != 0.0))) {
                s.store_sqrt_ad(639, A::add(A::square(s.ad_value(638)), s.ad_value(639)));
            }
            if (((!(s.v[837] != 0.0)) && (!(s.v[838] != 0.0))) && (!(s.v[845] != 0.0))) {
                s.store_scale_ad(286, A::offset(A::div(s.ad_value(638), s.ad_value(639)), 1.0), 0.5);
            }
            if (((!(s.v[837] != 0.0)) && (!(s.v[838] != 0.0))) && (!(s.v[845] != 0.0))) {
                s.store_sub_ad(284, A::neg(s.ad_value(296)), A::scale(A::add(s.ad_value(638), s.ad_value(639)), 0.5));
            }
            if (((!(s.v[837] != 0.0)) && (!(s.v[838] != 0.0))) && (!(s.v[845] != 0.0))) {
                s.store_mul_ad_rhs(285, 285, A::mul(s.ad_value(283), s.ad_value(286)));
            }
            if (((!(s.v[837] != 0.0)) && (!(s.v[838] != 0.0))) && (!(s.v[845] != 0.0))) {
                s.store_div_ad_lhs(332, A::scale(A::scale(A::scale(A::square(s.ad_value(284)), 0.5), 9662367879.197212), 6.241449993689894e18), 471);
            }
            if (((!(s.v[837] != 0.0)) && (!(s.v[838] != 0.0))) && (!(s.v[845] != 0.0))) {
                s.store_div_ad_lhs(333, A::mul(A::scale(s.ad_value(332), 2.0), s.ad_value(285)), 284);
            }
            if (((!(s.v[837] != 0.0)) && (!(s.v[838] != 0.0))) && (!(s.v[845] != 0.0))) {
                let assign11490_body27_ad_e14835: A = A::div(A::add(A::sub(A::add(A::add(A::sub(s.ad_value(308), s.ad_value(310)), A::scale(s.ad_value(282), 1.0 / (s.v[294]))), A::scale(A::add(s.ad_value(282), A::scale(s.ad_value(296), 0.5)), s.v[536])), s.ad_value(440)), s.ad_value(332)), A::add(A::add(A::offset(A::scale(s.ad_value(283), 1.0 / (s.v[294])), (-1.0)), A::scale(s.ad_value(283), s.v[536])), s.ad_value(333)));
                s.store_sub_ad_rhs(284, 310, assign11490_body27_ad_e14835);
            }
            s.v[853] = if ((((s.v[284] - s.v[310])) as f64).abs() < 1e-12) { 1.0 } else { 0.0 };
            if ((((!(s.v[837] != 0.0)) && (!(s.v[838] != 0.0))) && (!(s.v[845] != 0.0))) && (s.v[853] != 0.0)) {
                s.store_scalar(63, s.v[29]);
            }
            if (((!(s.v[837] != 0.0)) && (!(s.v[838] != 0.0))) && (!(s.v[845] != 0.0))) {
                s.copy_ad(310, 284);
            }
            if (((!(s.v[837] != 0.0)) && (!(s.v[838] != 0.0))) && (!(s.v[845] != 0.0))) {
                s.copy_ad(314, 282);
            }
            if (((!(s.v[837] != 0.0)) && (!(s.v[838] != 0.0))) && (!(s.v[845] != 0.0))) {
                s.store_offset(63, 63, 1.0);
            }
        }

        if (((!(s.v[837] != 0.0)) && (!(s.v[838] != 0.0))) && (!(s.v[845] != 0.0))) {
            s.store_add(310, 440, 310);
        }

        if (((!(s.v[837] != 0.0)) && (!(s.v[838] != 0.0))) && (!(s.v[845] != 0.0))) {
            s.store_sub_ad_rhs(309, 310, A::scale(s.ad_value(314), 1.0 / (s.v[294])));
        }

        if ((!(s.v[837] != 0.0)) && (!(s.v[838] != 0.0))) {
            s.copy_ad(584, 309);
        }

        s.v[854] = if (s.v[86] < 1e-12) { 1.0 } else { 0.0 };

        if (s.v[854] != 0.0) {
            s.copy_ad(302, 305);
        }

        if (s.v[854] != 0.0) {
            s.copy_ad(303, 306);
        }

        if (s.v[854] != 0.0) {
            s.copy_ad(304, 307);
        }

        if (s.v[854] != 0.0) {
            s.copy_ad(581, 522);
        }

        if (!(s.v[854] != 0.0)) {
            s.copy_ad(302, 308);
        }

        if (!(s.v[854] != 0.0)) {
            s.copy_ad(303, 309);
        }

        if (!(s.v[854] != 0.0)) {
            s.store_sub(304, 310, 440);
        }

        if (!(s.v[854] != 0.0)) {
            s.store_ad(581, &{
                if (s.v[303] < s.v[302]) {
                    s.ad_value(303)
                } else {
                    s.ad_value(302)
                }
            });
        }

        s.v[379] = (if (s.v[292] < 0.0) { 1.0 } else { 0.0 });

        s.copy_ad(308, 302);

        s.copy_ad(309, 303);

        s.copy_ad(310, 304);

        s.copy_ad(584, 581);

        s.v[63] = 1.0;

    }
}
