#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, ReactiveScratch, THERMAL_VOLTAGE_PER_K};
use super::super::state::Instance;

impl Instance {

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
        if (((s.v[418] != 0.0) && (!(s.v[522] != 0.0))) && (s.v[523] != 0.0)) {
            s.store_sqrt_ad(436, A::sub_from_scalar(1.0, A::mul(s.ad_value(428), s.ad_value(121))));
        }

        if (((s.v[418] != 0.0) && (!(s.v[522] != 0.0))) && (!(s.v[523] != 0.0))) {
            s.store_powf_ad(436, A::sub_from_scalar(1.0, A::mul(s.ad_value(428), s.ad_value(121))), s.v[124]);
        }

        if ((s.v[418] != 0.0) && (!(s.v[522] != 0.0))) {
            s.store_add_ad(273, A::mul(s.ad_value(133), A::sub_from_scalar(1.0, s.ad_value(436))), A::mul(s.ad_value(136), A::sub(s.ad_value(190), s.ad_value(428))));
        }

        if ((s.v[418] != 0.0) && (!(s.v[522] != 0.0))) {
            s.store_mul(437, 103, 372);
        }

        s.v[524] = if ((s.v[22] == 0.0) && (s.v[25] == 0.0)) { 1.0 } else { 0.0 };

        if (((s.v[418] != 0.0) && (!(s.v[522] != 0.0))) && (s.v[524] != 0.0)) {
            s.store_scalar(439, 0.0);
        }

        if (((s.v[418] != 0.0) && (!(s.v[522] != 0.0))) && (s.v[524] != 0.0)) {
            s.store_scalar(442, 0.0);
        }

        if (((s.v[418] != 0.0) && (!(s.v[522] != 0.0))) && (s.v[524] != 0.0)) {
            s.store_scalar(443, 0.0);
        }

        if (((s.v[418] != 0.0) && (!(s.v[522] != 0.0))) && (s.v[524] != 0.0)) {
            s.store_scalar(444, 0.0);
        }

        if (((s.v[418] != 0.0) && (!(s.v[522] != 0.0))) && (s.v[524] != 0.0)) {
            s.store_scalar(438, 0.0);
        }

        if (((s.v[418] != 0.0) && (!(s.v[522] != 0.0))) && (!(s.v[524] != 0.0))) {
            s.store_sub(439, 109, 433);
        }

        if (((s.v[418] != 0.0) && (!(s.v[522] != 0.0))) && (!(s.v[524] != 0.0))) {
            s.store_sub_from_scalar_ad(440, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(431), s.ad_value(439)))));
        }

        s.v[525] = if (s.v[11] == 0.5) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (!(s.v[522] != 0.0))) && (!(s.v[524] != 0.0))) && (s.v[525] != 0.0)) {
            s.store_scalar(441, 0.0);
        }

        if ((((s.v[418] != 0.0) && (!(s.v[522] != 0.0))) && (!(s.v[524] != 0.0))) && (!(s.v[525] != 0.0))) {
            s.store_scale_ad(441, A::add(A::div(A::mul(A::square(s.ad_value(440)), A::ln(s.ad_value(440))), A::sub_from_scalar(1.0, s.ad_value(440))), s.ad_value(440)), (1.0 - (2.0 * s.v[11])));
        }

        if (((s.v[418] != 0.0) && (!(s.v[522] != 0.0))) && (!(s.v[524] != 0.0))) {
            s.store_add(442, 440, 441);
        }

        s.v[526] = if (s.v[11] == 0.5) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (!(s.v[522] != 0.0))) && (!(s.v[524] != 0.0))) && (s.v[526] != 0.0)) {
            s.store_sqrt_ad(436, A::scale(s.ad_value(439), s.v[145]));
        }

        if ((((s.v[418] != 0.0) && (!(s.v[522] != 0.0))) && (!(s.v[524] != 0.0))) && (!(s.v[526] != 0.0))) {
            s.store_powf_ad(436, A::scale(s.ad_value(439), s.v[145]), s.v[11]);
        }

        if (((s.v[418] != 0.0) && (!(s.v[522] != 0.0))) && (!(s.v[524] != 0.0))) {
            s.store_scale(443, 436, s.v[139]);
        }

        if (((s.v[418] != 0.0) && (!(s.v[522] != 0.0))) && (!(s.v[524] != 0.0))) {
            s.store_mul_ad_rhs(444, 100, A::mul(A::offset(s.ad_value(430), (-1.0)), s.ad_value(443)));
        }

        if (((s.v[418] != 0.0) && (!(s.v[522] != 0.0))) && (!(s.v[524] != 0.0))) {
            s.store_scaled_mul(438, 444, 442, s.v[22]);
        }

        s.v[527] = if (s.v[25] == 0.0) { 1.0 } else { 0.0 };

        if (((s.v[418] != 0.0) && (!(s.v[522] != 0.0))) && (s.v[527] != 0.0)) {
            s.store_scalar(445, 0.0);
        }

        if (((s.v[418] != 0.0) && (!(s.v[522] != 0.0))) && (!(s.v[527] != 0.0))) {
            s.store_scale_ad(446, A::div(A::scale(s.ad_value(443), s.v[124]), s.ad_value(439)), s.v[154]);
        }

        if (((s.v[418] != 0.0) && (!(s.v[522] != 0.0))) && (!(s.v[527] != 0.0))) {
            s.store_div_from_scalar(447, (0.666666666666667 * s.v[151]), 446);
        }

        if (((s.v[418] != 0.0) && (!(s.v[522] != 0.0))) && (!(s.v[527] != 0.0))) {
            s.store_square(448, 447);
        }

        if (((s.v[418] != 0.0) && (!(s.v[522] != 0.0))) && (!(s.v[527] != 0.0))) {
            s.store_sqrt_ad(449, A::div(A::square(s.ad_value(448)), A::offset(A::square(s.ad_value(448)), 1.0)));
        }

        if (((s.v[418] != 0.0) && (!(s.v[522] != 0.0))) && (!(s.v[527] != 0.0))) {
            s.store_sqrt_ad(450, A::abs(s.ad_value(449)));
        }

        if (((s.v[418] != 0.0) && (!(s.v[522] != 0.0))) && (!(s.v[527] != 0.0))) {
            s.store_mul(451, 449, 450);
        }

        s.v[528] = if (((-s.v[11]) * s.v[127]) == (-1.0)) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (!(s.v[522] != 0.0))) && (!(s.v[527] != 0.0))) && (s.v[528] != 0.0)) {
            s.store_div_from_scalar_ad(452, 1.0, A::offset(A::mul(s.ad_value(446), s.ad_value(451)), 1.0));
        }

        if ((((s.v[418] != 0.0) && (!(s.v[522] != 0.0))) && (!(s.v[527] != 0.0))) && (!(s.v[528] != 0.0))) {
            s.store_powf_ad(452, A::offset(A::mul(s.ad_value(446), s.ad_value(451)), 1.0), ((-s.v[11]) * s.v[127]));
        }

        if (((s.v[418] != 0.0) && (!(s.v[522] != 0.0))) && (!(s.v[527] != 0.0))) {
            s.store_div_ad(453, A::mul(s.ad_value(442), s.ad_value(452)), A::add(s.ad_value(442), s.ad_value(452)));
        }

        if (((s.v[418] != 0.0) && (!(s.v[522] != 0.0))) && (!(s.v[527] != 0.0))) {
            s.store_sqrt_ad(454, A::scale(A::div(s.ad_value(446), s.ad_value(450)), 0.375));
        }

        if (((s.v[418] != 0.0) && (!(s.v[522] != 0.0))) && (!(s.v[527] != 0.0))) {
            s.store_sub_ad_lhs(455, A::scale(A::mul(s.ad_value(447), s.ad_value(450)), 2.0), 449);
        }

        if (((s.v[418] != 0.0) && (!(s.v[522] != 0.0))) && (!(s.v[527] != 0.0))) {
            s.store_add_ad(456, A::sub(A::mul(A::scale(s.ad_value(447), s.v[151]), s.ad_value(450)), A::scale(s.ad_value(449), s.v[151])), A::scale(A::mul(s.ad_value(446), s.ad_value(451)), 0.5));
        }

        if (((s.v[418] != 0.0) && (!(s.v[522] != 0.0))) && (!(s.v[527] != 0.0))) {
            s.store_mul_ad_lhs(457, A::offset(s.ad_value(455), (-1.0)), 454);
        }

        if (((s.v[418] != 0.0) && (!(s.v[522] != 0.0))) && (!(s.v[527] != 0.0))) {
            s.store_square(419, 457);
        }

        s.v[529] = if (s.v[457] > 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (!(s.v[522] != 0.0))) && (!(s.v[527] != 0.0))) && (s.v[529] != 0.0)) {
            s.store_div_from_scalar_ad(420, 1.0, A::offset(A::scale(s.ad_value(457), s.v[86]), 1.0));
        }

        if ((((s.v[418] != 0.0) && (!(s.v[522] != 0.0))) && (!(s.v[527] != 0.0))) && (!(s.v[529] != 0.0))) {
            s.store_div_from_scalar_ad(420, 1.0, A::sub_from_scalar(1.0, A::scale(s.ad_value(457), s.v[86])));
        }

        s.v[530] = if (((-s.v[419]) + s.v[456]) > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (!(s.v[522] != 0.0))) && (!(s.v[527] != 0.0))) && (s.v[530] != 0.0)) {
            s.store_exp_ad(436, A::sub(s.ad_value(456), s.ad_value(419)));
        }

        if ((((s.v[418] != 0.0) && (!(s.v[522] != 0.0))) && (!(s.v[527] != 0.0))) && (!(s.v[530] != 0.0))) {
            s.store_div_from_scalar_ad(436, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(456), s.ad_value(419))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(456), s.ad_value(419))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(456), s.ad_value(419))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if (((s.v[418] != 0.0) && (!(s.v[522] != 0.0))) && (!(s.v[527] != 0.0))) {
            s.store_mul_ad_lhs(421, A::add(A::add(A::scale(s.ad_value(420), 0.29214664), A::scale(A::square(s.ad_value(420)), s.v[87])), A::scale(A::mul(A::square(s.ad_value(420)), s.ad_value(420)), s.v[88])), 436);
        }

        s.v[531] = if (s.v[457] > 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (!(s.v[522] != 0.0))) && (!(s.v[527] != 0.0))) && (s.v[531] != 0.0)) {
            s.copy_ad(458, 421);
        }

        s.v[532] = if (s.v[456] > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((s.v[418] != 0.0) && (!(s.v[522] != 0.0))) && (!(s.v[527] != 0.0))) && (!(s.v[531] != 0.0))) && (s.v[532] != 0.0)) {
            s.store_exp(436, 456);
        }

        if (((((s.v[418] != 0.0) && (!(s.v[522] != 0.0))) && (!(s.v[527] != 0.0))) && (!(s.v[531] != 0.0))) && (!(s.v[532] != 0.0))) {
            s.store_div_from_scalar_ad(436, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(456)), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(456)), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), s.ad_value(456)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if ((((s.v[418] != 0.0) && (!(s.v[522] != 0.0))) && (!(s.v[527] != 0.0))) && (!(s.v[531] != 0.0))) {
            s.store_sub_ad_lhs(458, A::scale(s.ad_value(436), 2.0), 421);
        }

        if (((s.v[418] != 0.0) && (!(s.v[522] != 0.0))) && (!(s.v[527] != 0.0))) {
            s.store_scale_ad(459, A::div(A::scale(s.ad_value(458), s.v[151]), s.ad_value(454)), (1.772453850905516 * 0.5));
        }

        if (((s.v[418] != 0.0) && (!(s.v[522] != 0.0))) && (!(s.v[527] != 0.0))) {
            s.store_scale_ad(445, A::mul(A::mul(s.ad_value(444), s.ad_value(459)), s.ad_value(453)), s.v[25]);
        }

        s.v[533] = if (s.v[31] == 0.0) { 1.0 } else { 0.0 };

        if (((s.v[418] != 0.0) && (!(s.v[522] != 0.0))) && (s.v[533] != 0.0)) {
            s.store_scalar(460, 0.0);
        }

        s.v[534] = if (s.v[11] == 0.5) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (!(s.v[522] != 0.0))) && (!(s.v[533] != 0.0))) && (s.v[534] != 0.0)) {
            s.store_sqrt_ad(436, A::scale(A::sub_from_scalar(s.v[8], s.ad_value(434)), s.v[145]));
        }

        if ((((s.v[418] != 0.0) && (!(s.v[522] != 0.0))) && (!(s.v[533] != 0.0))) && (!(s.v[534] != 0.0))) {
            s.store_powf_ad(436, A::scale(A::sub_from_scalar(s.v[8], s.ad_value(434)), s.v[145]), s.v[11]);
        }

        if (((s.v[418] != 0.0) && (!(s.v[522] != 0.0))) && (!(s.v[533] != 0.0))) {
            s.store_scale_ad(461, A::div(A::scale(A::sub_from_scalar(s.v[8], s.ad_value(434)), s.v[142]), s.ad_value(436)), s.v[127]);
        }

        s.v[535] = if (((((-s.v[157]) / s.v[461])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (!(s.v[522] != 0.0))) && (!(s.v[533] != 0.0))) && (s.v[535] != 0.0)) {
            s.store_exp_ad(436, A::div(A::neg(s.ad_value(157)), s.ad_value(461)));
        }

        s.v[536] = if (((-s.v[157]) / s.v[461]) < (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((s.v[418] != 0.0) && (!(s.v[522] != 0.0))) && (!(s.v[533] != 0.0))) && (!(s.v[535] != 0.0))) && (s.v[536] != 0.0)) {
            let assign8600_ad_e9956: A = A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(157)), s.ad_value(461))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(157)), s.ad_value(461))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(157)), s.ad_value(461))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
            s.store_div_from_scalar_ad(436, 1e-100, assign8600_ad_e9956);
        }

        if (((((s.v[418] != 0.0) && (!(s.v[522] != 0.0))) && (!(s.v[533] != 0.0))) && (!(s.v[535] != 0.0))) && (!(s.v[536] != 0.0))) {
            let assign8610_ad_e10004: A = A::scale(A::offset(A::mul(A::offset(A::div(A::neg(s.ad_value(157)), s.ad_value(461)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::div(A::neg(s.ad_value(157)), s.ad_value(461)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::div(A::neg(s.ad_value(157)), s.ad_value(461)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
            s.store_ad(436, &assign8610_ad_e10004);
        }

        if (((s.v[418] != 0.0) && (!(s.v[522] != 0.0))) && (!(s.v[533] != 0.0))) {
            s.store_scale_ad(460, A::mul(A::mul(A::mul(s.ad_value(190), s.ad_value(461)), s.ad_value(461)), s.ad_value(436)), s.v[31]);
        }

        s.v[537] = if ((s.v[40] > 1000000.0) || (p.p80 == 0.0)) { 1.0 } else { 0.0 };

        if (((s.v[418] != 0.0) && (!(s.v[522] != 0.0))) && (s.v[537] != 0.0)) {
            s.store_scalar(462, 1.0);
        }

        s.v[538] = if (s.v[435] > ((-s.v[158]) * s.v[40])) { 1.0 } else { 0.0 };

        s.v[539] = if (s.v[43] == 4.0) { 1.0 } else { 0.0 };

        if (((((s.v[418] != 0.0) && (!(s.v[522] != 0.0))) && (!(s.v[537] != 0.0))) && (s.v[538] != 0.0)) && (s.v[539] != 0.0)) {
            s.store_mul_ad(436, A::mul(A::mul(A::abs(A::mul(s.ad_value(435), s.ad_value(164))), A::abs(A::mul(s.ad_value(435), s.ad_value(164)))), A::abs(A::mul(s.ad_value(435), s.ad_value(164)))), A::abs(A::mul(s.ad_value(435), s.ad_value(164))));
        }

        if (((((s.v[418] != 0.0) && (!(s.v[522] != 0.0))) && (!(s.v[537] != 0.0))) && (s.v[538] != 0.0)) && (!(s.v[539] != 0.0))) {
            s.store_powf_ad(436, A::abs(A::mul(s.ad_value(435), s.ad_value(164))), s.v[43]);
        }

        if ((((s.v[418] != 0.0) && (!(s.v[522] != 0.0))) && (!(s.v[537] != 0.0))) && (s.v[538] != 0.0)) {
            s.store_div_from_scalar_ad(462, 1.0, A::sub_from_scalar(1.0, s.ad_value(436)));
        }

        if ((((s.v[418] != 0.0) && (!(s.v[522] != 0.0))) && (!(s.v[537] != 0.0))) && (!(s.v[538] != 0.0))) {
            s.store_offset_ad(462, A::mul(A::add(s.ad_value(435), A::scale(s.ad_value(40), s.v[158])), s.ad_value(167)), s.v[161]);
        }

        if ((s.v[418] != 0.0) && (!(s.v[522] != 0.0))) {
            s.store_mul_ad_lhs(272, A::add(A::add(A::add(s.ad_value(437), s.ad_value(438)), s.ad_value(445)), s.ad_value(460)), 462);
        }

        if ((s.v[418] != 0.0) && (!(s.v[522] != 0.0))) {
            s.store_mul_ad_lhs(293, A::add(A::add(s.ad_value(438), s.ad_value(445)), s.ad_value(460)), 462);
        }

        if (s.v[418] != 0.0) {
            s.store_add_ad(180, A::add(A::scale(s.ad_value(268), s.v[256]), A::scale(s.ad_value(270), s.v[257])), A::scale(s.ad_value(272), s.v[258]));
        }

        s.v[540] = if !(((s.v[256] == 0.0) && (s.v[257] == 0.0)) && (s.v[258] == 0.0)) { 1.0 } else { 0.0 };

        if ((s.v[418] != 0.0) && (s.v[540] != 0.0)) {
            s.store_mul_ad_lhs(422, A::scale(s.ad_value(265), 4.0), 265);
        }

        if ((s.v[418] != 0.0) && (s.v[540] != 0.0)) {
            s.store_div(423, 265, 266);
        }

        if ((s.v[418] != 0.0) && (s.v[540] != 0.0)) {
            s.store_add_ad_rhs(424, 191, A::mul(s.ad_value(265), s.ad_value(423)));
        }

        if ((s.v[418] != 0.0) && (s.v[540] != 0.0)) {
            s.store_add(425, 266, 424);
        }

        if ((s.v[418] != 0.0) && (s.v[540] != 0.0)) {
            s.store_sub(426, 266, 424);
        }

        if ((s.v[418] != 0.0) && (s.v[540] != 0.0)) {
            s.store_sqrt_ad(427, A::add(A::square(s.ad_value(426)), s.ad_value(422)));
        }

        if ((s.v[418] != 0.0) && (s.v[540] != 0.0)) {
            s.store_scale_ad(428, A::div(A::mul(s.ad_value(191), s.ad_value(266)), A::add(s.ad_value(425), s.ad_value(427))), 2.0);
        }

        s.v[541] = if (s.v[191] < s.v[262]) { 1.0 } else { 0.0 };

        s.v[542] = if ((((0.5 * (s.v[191] * s.v[85]))) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (s.v[541] != 0.0)) && (s.v[542] != 0.0)) {
            s.store_exp_ad(430, A::scale(s.ad_value(191), (s.v[85] * 0.5)));
        }

        s.v[543] = if ((0.5 * (s.v[191] * s.v[85])) < (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (s.v[541] != 0.0)) && (!(s.v[542] != 0.0))) && (s.v[543] != 0.0)) {
            let assign8860_ad_e10343: A = A::div_from_scalar(1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(s.ad_value(191), (s.v[85] * 0.5))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(s.ad_value(191), (s.v[85] * 0.5))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::scale(s.ad_value(191), (s.v[85] * 0.5))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
            s.store_ad(430, &assign8860_ad_e10343);
        }

        if (((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (s.v[541] != 0.0)) && (!(s.v[542] != 0.0))) && (!(s.v[543] != 0.0))) {
            s.store_scale_ad(430, A::offset(A::mul(A::offset(A::scale(s.ad_value(191), (s.v[85] * 0.5)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::scale(s.ad_value(191), (s.v[85] * 0.5)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::scale(s.ad_value(191), (s.v[85] * 0.5)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
        }

        if (((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (s.v[541] != 0.0)) {
            s.store_scale_ad(363, A::square(s.ad_value(318)), 1.0 / (s.v[308]));
        }

        if (((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (s.v[541] != 0.0)) {
            s.store_scale_ad(362, A::ln(A::div_from_scalar(s.v[308], s.ad_value(363))), (s.v[62] / s.v[85]));
        }

        s.v[544] = if (s.v[62] < p.p85) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (s.v[541] != 0.0)) && (s.v[544] != 0.0)) {
            s.store_offset_ad(360, A::scale(A::sub(s.ad_value(191), s.ad_value(362)), p.p86), s.v[62]);
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (s.v[541] != 0.0)) && (s.v[544] != 0.0)) {
            s.store_sub_from_scalar_ad(350, s.v[62], A::scale(s.ad_value(362), p.p86));
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (s.v[541] != 0.0)) && (s.v[544] != 0.0)) {
            s.store_offset_ad(314, A::sub_from_scalar(p.p85, s.ad_value(360)), (-0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (s.v[541] != 0.0)) && (s.v[544] != 0.0)) {
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (s.v[541] != 0.0)) && (s.v[544] != 0.0)) {
            s.store_ad(315, &{
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (s.v[541] != 0.0)) && (s.v[544] != 0.0)) {
            s.store_sqrt_ad(315, A::add(A::square(s.ad_value(314)), s.ad_value(315)));
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (s.v[541] != 0.0)) && (s.v[544] != 0.0)) {
            s.store_sub_from_scalar_ad(361, p.p85, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5));
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (s.v[541] != 0.0)) && (s.v[544] != 0.0)) {
            s.store_offset(314, 361, (((-s.v[62])) + ((-0.01))));
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (s.v[541] != 0.0)) && (s.v[544] != 0.0)) {
            s.store_scalar(315, ((4.0 * s.v[62]) * 0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (s.v[541] != 0.0)) && (s.v[544] != 0.0)) {
            s.store_ad(315, &{
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (s.v[541] != 0.0)) && (s.v[544] != 0.0)) {
            s.store_sqrt_ad(315, A::add(A::square(s.ad_value(314)), s.ad_value(315)));
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (s.v[541] != 0.0)) && (s.v[544] != 0.0)) {
            s.store_offset_ad(359, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5), s.v[62]);
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (s.v[541] != 0.0)) && (s.v[544] != 0.0)) {
            s.store_offset_ad(314, A::sub_from_scalar(p.p85, s.ad_value(350)), (-0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (s.v[541] != 0.0)) && (s.v[544] != 0.0)) {
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (s.v[541] != 0.0)) && (s.v[544] != 0.0)) {
            s.store_ad(315, &{
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (s.v[541] != 0.0)) && (s.v[544] != 0.0)) {
            s.store_sqrt_ad(315, A::add(A::square(s.ad_value(314)), s.ad_value(315)));
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (s.v[541] != 0.0)) && (s.v[544] != 0.0)) {
            s.store_sub_from_scalar_ad(350, p.p85, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5));
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (s.v[541] != 0.0)) && (s.v[544] != 0.0)) {
            s.store_offset(314, 350, (((-s.v[62])) + ((-0.01))));
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (s.v[541] != 0.0)) && (s.v[544] != 0.0)) {
            s.store_scalar(315, ((4.0 * s.v[62]) * 0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (s.v[541] != 0.0)) && (s.v[544] != 0.0)) {
            s.store_ad(315, &{
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (s.v[541] != 0.0)) && (s.v[544] != 0.0)) {
            s.store_sqrt_ad(315, A::add(A::square(s.ad_value(314)), s.ad_value(315)));
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (s.v[541] != 0.0)) && (s.v[544] != 0.0)) {
            s.store_offset_ad(350, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5), s.v[62]);
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (s.v[541] != 0.0)) && (!(s.v[544] != 0.0))) {
            s.store_scalar(350, s.v[62]);
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (s.v[541] != 0.0)) && (!(s.v[544] != 0.0))) {
            s.store_scalar(359, s.v[62]);
        }

        s.v[545] = if ((((s.v[85] * ((s.v[191] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85))))) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (s.v[541] != 0.0)) && (s.v[545] != 0.0)) {
            s.store_exp_ad(370, A::scale(A::add(A::div(s.ad_value(191), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]));
        }

        s.v[546] = if ((s.v[85] * ((s.v[191] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85)))) < (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (s.v[541] != 0.0)) && (!(s.v[545] != 0.0))) && (s.v[546] != 0.0)) {
            let assign9180_ad_e10909: A = A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(A::add(A::div(s.ad_value(191), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85])), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::scale(A::add(A::div(s.ad_value(191), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85])), 0.3333333333333333), 1.0));
            s.store_div_from_scalar_ad(370, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(A::add(A::div(s.ad_value(191), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85])), A::offset(A::scale(assign9180_ad_e10909, 0.5), 1.0)), 1.0));
        }

        if (((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (s.v[541] != 0.0)) && (!(s.v[545] != 0.0))) && (!(s.v[546] != 0.0))) {
            let assign9190_ad_e10987: A = A::mul(A::offset(A::scale(A::add(A::div(s.ad_value(191), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]), (-230.25850929940458)), A::offset(A::scale(A::offset(A::scale(A::add(A::div(s.ad_value(191), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]), (-230.25850929940458)), 0.3333333333333333), 1.0));
            s.store_scale_ad(370, A::offset(A::mul(A::offset(A::scale(A::add(A::div(s.ad_value(191), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]), (-230.25850929940458)), A::offset(A::scale(assign9190_ad_e10987, 0.5), 1.0)), 1.0), 1e100);
        }

        if (((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (s.v[541] != 0.0)) {
            s.store_scale_ad(363, A::square(s.ad_value(318)), 1.0 / (s.v[310]));
        }

        if (((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (s.v[541] != 0.0)) {
            s.store_scale_ad(362, A::ln(A::div_from_scalar(s.v[310], s.ad_value(363))), (s.v[64] / s.v[85]));
        }

        s.v[547] = if (s.v[64] < p.p85) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (s.v[541] != 0.0)) && (s.v[547] != 0.0)) {
            s.store_offset_ad(360, A::scale(A::sub(s.ad_value(191), s.ad_value(362)), p.p86), s.v[64]);
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (s.v[541] != 0.0)) && (s.v[547] != 0.0)) {
            s.store_sub_from_scalar_ad(350, s.v[64], A::scale(s.ad_value(362), p.p86));
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (s.v[541] != 0.0)) && (s.v[547] != 0.0)) {
            s.store_offset_ad(314, A::sub_from_scalar(p.p85, s.ad_value(360)), (-0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (s.v[541] != 0.0)) && (s.v[547] != 0.0)) {
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (s.v[541] != 0.0)) && (s.v[547] != 0.0)) {
            s.store_ad(315, &{
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
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
        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (s.v[541] != 0.0)) && (s.v[547] != 0.0)) {
            s.store_sqrt_ad(315, A::add(A::square(s.ad_value(314)), s.ad_value(315)));
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (s.v[541] != 0.0)) && (s.v[547] != 0.0)) {
            s.store_sub_from_scalar_ad(361, p.p85, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5));
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (s.v[541] != 0.0)) && (s.v[547] != 0.0)) {
            s.store_offset(314, 361, (((-s.v[64])) + ((-0.01))));
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (s.v[541] != 0.0)) && (s.v[547] != 0.0)) {
            s.store_scalar(315, ((4.0 * s.v[64]) * 0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (s.v[541] != 0.0)) && (s.v[547] != 0.0)) {
            s.store_ad(315, &{
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (s.v[541] != 0.0)) && (s.v[547] != 0.0)) {
            s.store_sqrt_ad(315, A::add(A::square(s.ad_value(314)), s.ad_value(315)));
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (s.v[541] != 0.0)) && (s.v[547] != 0.0)) {
            s.store_offset_ad(359, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5), s.v[64]);
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (s.v[541] != 0.0)) && (s.v[547] != 0.0)) {
            s.store_offset_ad(314, A::sub_from_scalar(p.p85, s.ad_value(350)), (-0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (s.v[541] != 0.0)) && (s.v[547] != 0.0)) {
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (s.v[541] != 0.0)) && (s.v[547] != 0.0)) {
            s.store_ad(315, &{
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (s.v[541] != 0.0)) && (s.v[547] != 0.0)) {
            s.store_sqrt_ad(315, A::add(A::square(s.ad_value(314)), s.ad_value(315)));
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (s.v[541] != 0.0)) && (s.v[547] != 0.0)) {
            s.store_sub_from_scalar_ad(350, p.p85, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5));
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (s.v[541] != 0.0)) && (s.v[547] != 0.0)) {
            s.store_offset(314, 350, (((-s.v[64])) + ((-0.01))));
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (s.v[541] != 0.0)) && (s.v[547] != 0.0)) {
            s.store_scalar(315, ((4.0 * s.v[64]) * 0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (s.v[541] != 0.0)) && (s.v[547] != 0.0)) {
            s.store_ad(315, &{
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (s.v[541] != 0.0)) && (s.v[547] != 0.0)) {
            s.store_sqrt_ad(315, A::add(A::square(s.ad_value(314)), s.ad_value(315)));
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (s.v[541] != 0.0)) && (s.v[547] != 0.0)) {
            s.store_offset_ad(350, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5), s.v[64]);
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (s.v[541] != 0.0)) && (!(s.v[547] != 0.0))) {
            s.store_scalar(350, s.v[64]);
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (s.v[541] != 0.0)) && (!(s.v[547] != 0.0))) {
            s.store_scalar(359, s.v[64]);
        }

        s.v[548] = if ((((s.v[85] * ((s.v[191] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85))))) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (s.v[541] != 0.0)) && (s.v[548] != 0.0)) {
            s.store_exp_ad(371, A::scale(A::add(A::div(s.ad_value(191), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]));
        }

        s.v[549] = if ((s.v[85] * ((s.v[191] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85)))) < (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (s.v[541] != 0.0)) && (!(s.v[548] != 0.0))) && (s.v[549] != 0.0)) {
            let assign9500_ad_e11510: A = A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(A::add(A::div(s.ad_value(191), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85])), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::scale(A::add(A::div(s.ad_value(191), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85])), 0.3333333333333333), 1.0));
            s.store_div_from_scalar_ad(371, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(A::add(A::div(s.ad_value(191), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85])), A::offset(A::scale(assign9500_ad_e11510, 0.5), 1.0)), 1.0));
        }

        if (((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (s.v[541] != 0.0)) && (!(s.v[548] != 0.0))) && (!(s.v[549] != 0.0))) {
            let assign9510_ad_e11588: A = A::mul(A::offset(A::scale(A::add(A::div(s.ad_value(191), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]), (-230.25850929940458)), A::offset(A::scale(A::offset(A::scale(A::add(A::div(s.ad_value(191), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]), (-230.25850929940458)), 0.3333333333333333), 1.0));
            s.store_scale_ad(371, A::offset(A::mul(A::offset(A::scale(A::add(A::div(s.ad_value(191), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]), (-230.25850929940458)), A::offset(A::scale(assign9510_ad_e11588, 0.5), 1.0)), 1.0), 1e100);
        }

        if (((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (s.v[541] != 0.0)) {
            s.store_scale_ad(363, A::square(s.ad_value(318)), 1.0 / (s.v[309]));
        }

        if (((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (s.v[541] != 0.0)) {
            s.store_scale_ad(362, A::ln(A::div_from_scalar(s.v[309], s.ad_value(363))), (s.v[63] / s.v[85]));
        }

        s.v[550] = if (s.v[63] < p.p85) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (s.v[541] != 0.0)) && (s.v[550] != 0.0)) {
            s.store_offset_ad(360, A::scale(A::sub(s.ad_value(191), s.ad_value(362)), p.p86), s.v[63]);
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (s.v[541] != 0.0)) && (s.v[550] != 0.0)) {
            s.store_sub_from_scalar_ad(350, s.v[63], A::scale(s.ad_value(362), p.p86));
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (s.v[541] != 0.0)) && (s.v[550] != 0.0)) {
            s.store_offset_ad(314, A::sub_from_scalar(p.p85, s.ad_value(360)), (-0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (s.v[541] != 0.0)) && (s.v[550] != 0.0)) {
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (s.v[541] != 0.0)) && (s.v[550] != 0.0)) {
            s.store_ad(315, &{
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (s.v[541] != 0.0)) && (s.v[550] != 0.0)) {
            s.store_sqrt_ad(315, A::add(A::square(s.ad_value(314)), s.ad_value(315)));
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (s.v[541] != 0.0)) && (s.v[550] != 0.0)) {
            s.store_sub_from_scalar_ad(361, p.p85, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5));
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (s.v[541] != 0.0)) && (s.v[550] != 0.0)) {
            s.store_offset(314, 361, (((-s.v[63])) + ((-0.01))));
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (s.v[541] != 0.0)) && (s.v[550] != 0.0)) {
            s.store_scalar(315, ((4.0 * s.v[63]) * 0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (s.v[541] != 0.0)) && (s.v[550] != 0.0)) {
            s.store_ad(315, &{
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (s.v[541] != 0.0)) && (s.v[550] != 0.0)) {
            s.store_sqrt_ad(315, A::add(A::square(s.ad_value(314)), s.ad_value(315)));
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (s.v[541] != 0.0)) && (s.v[550] != 0.0)) {
            s.store_offset_ad(359, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5), s.v[63]);
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (s.v[541] != 0.0)) && (s.v[550] != 0.0)) {
            s.store_offset_ad(314, A::sub_from_scalar(p.p85, s.ad_value(350)), (-0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (s.v[541] != 0.0)) && (s.v[550] != 0.0)) {
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (s.v[541] != 0.0)) && (s.v[550] != 0.0)) {
            s.store_ad(315, &{
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (s.v[541] != 0.0)) && (s.v[550] != 0.0)) {
            s.store_sqrt_ad(315, A::add(A::square(s.ad_value(314)), s.ad_value(315)));
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (s.v[541] != 0.0)) && (s.v[550] != 0.0)) {
            s.store_sub_from_scalar_ad(350, p.p85, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5));
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (s.v[541] != 0.0)) && (s.v[550] != 0.0)) {
            s.store_offset(314, 350, (((-s.v[63])) + ((-0.01))));
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (s.v[541] != 0.0)) && (s.v[550] != 0.0)) {
            s.store_scalar(315, ((4.0 * s.v[63]) * 0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (s.v[541] != 0.0)) && (s.v[550] != 0.0)) {
            s.store_ad(315, &{
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (s.v[541] != 0.0)) && (s.v[550] != 0.0)) {
            s.store_sqrt_ad(315, A::add(A::square(s.ad_value(314)), s.ad_value(315)));
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (s.v[541] != 0.0)) && (s.v[550] != 0.0)) {
            s.store_offset_ad(350, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5), s.v[63]);
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (s.v[541] != 0.0)) && (!(s.v[550] != 0.0))) {
            s.store_scalar(350, s.v[63]);
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (s.v[541] != 0.0)) && (!(s.v[550] != 0.0))) {
            s.store_scalar(359, s.v[63]);
        }

        s.v[551] = if ((((s.v[85] * ((s.v[191] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85))))) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (s.v[541] != 0.0)) && (s.v[551] != 0.0)) {
            s.store_exp_ad(372, A::scale(A::add(A::div(s.ad_value(191), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]));
        }

        s.v[552] = if ((s.v[85] * ((s.v[191] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85)))) < (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (s.v[541] != 0.0)) && (!(s.v[551] != 0.0))) && (s.v[552] != 0.0)) {
            let assign9820_ad_e12111: A = A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(A::add(A::div(s.ad_value(191), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85])), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::scale(A::add(A::div(s.ad_value(191), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85])), 0.3333333333333333), 1.0));
            s.store_div_from_scalar_ad(372, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(A::add(A::div(s.ad_value(191), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85])), A::offset(A::scale(assign9820_ad_e12111, 0.5), 1.0)), 1.0));
        }

        if (((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (s.v[541] != 0.0)) && (!(s.v[551] != 0.0))) && (!(s.v[552] != 0.0))) {
            let assign9830_ad_e12189: A = A::mul(A::offset(A::scale(A::add(A::div(s.ad_value(191), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]), (-230.25850929940458)), A::offset(A::scale(A::offset(A::scale(A::add(A::div(s.ad_value(191), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]), (-230.25850929940458)), 0.3333333333333333), 1.0));
            s.store_scale_ad(372, A::offset(A::mul(A::offset(A::scale(A::add(A::div(s.ad_value(191), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]), (-230.25850929940458)), A::offset(A::scale(assign9830_ad_e12189, 0.5), 1.0)), 1.0), 1e100);
        }

        if (((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (!(s.v[541] != 0.0))) {
            s.store_sqrt_ad(430, A::mul(A::offset(A::scale(A::sub(s.ad_value(191), s.ad_value(262)), s.v[85]), 1.0), s.ad_value(263)));
        }

        if (((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (!(s.v[541] != 0.0))) {
            s.store_scale_ad(363, A::square(s.ad_value(318)), 1.0 / (s.v[308]));
        }

        if (((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (!(s.v[541] != 0.0))) {
            s.store_scale_ad(362, A::ln(A::div_from_scalar(s.v[308], s.ad_value(363))), (s.v[62] / s.v[85]));
        }

        s.v[553] = if (s.v[62] < p.p85) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (!(s.v[541] != 0.0))) && (s.v[553] != 0.0)) {
            s.store_offset_ad(360, A::scale(A::sub(s.ad_value(262), s.ad_value(362)), p.p86), s.v[62]);
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (!(s.v[541] != 0.0))) && (s.v[553] != 0.0)) {
            s.store_sub_from_scalar_ad(350, s.v[62], A::scale(s.ad_value(362), p.p86));
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (!(s.v[541] != 0.0))) && (s.v[553] != 0.0)) {
            s.store_offset_ad(314, A::sub_from_scalar(p.p85, s.ad_value(360)), (-0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (!(s.v[541] != 0.0))) && (s.v[553] != 0.0)) {
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (!(s.v[541] != 0.0))) && (s.v[553] != 0.0)) {
            s.store_ad(315, &{
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (!(s.v[541] != 0.0))) && (s.v[553] != 0.0)) {
            s.store_sqrt_ad(315, A::add(A::square(s.ad_value(314)), s.ad_value(315)));
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (!(s.v[541] != 0.0))) && (s.v[553] != 0.0)) {
            s.store_scale_ad(364, A::offset(A::div(s.ad_value(314), s.ad_value(315)), 1.0), 0.5);
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (!(s.v[541] != 0.0))) && (s.v[553] != 0.0)) {
            s.store_sub_from_scalar_ad(361, p.p85, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5));
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (!(s.v[541] != 0.0))) && (s.v[553] != 0.0)) {
            s.store_offset(314, 361, (((-s.v[62])) + ((-0.01))));
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (!(s.v[541] != 0.0))) && (s.v[553] != 0.0)) {
            s.store_scalar(315, ((4.0 * s.v[62]) * 0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (!(s.v[541] != 0.0))) && (s.v[553] != 0.0)) {
            s.store_ad(315, &{
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (!(s.v[541] != 0.0))) && (s.v[553] != 0.0)) {
            s.store_sqrt_ad(315, A::add(A::square(s.ad_value(314)), s.ad_value(315)));
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (!(s.v[541] != 0.0))) && (s.v[553] != 0.0)) {
            s.store_scale_ad(365, A::offset(A::div(s.ad_value(314), s.ad_value(315)), 1.0), 0.5);
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (!(s.v[541] != 0.0))) && (s.v[553] != 0.0)) {
            s.store_offset_ad(359, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5), s.v[62]);
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (!(s.v[541] != 0.0))) && (s.v[553] != 0.0)) {
            s.store_offset_ad(314, A::sub_from_scalar(p.p85, s.ad_value(350)), (-0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (!(s.v[541] != 0.0))) && (s.v[553] != 0.0)) {
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (!(s.v[541] != 0.0))) && (s.v[553] != 0.0)) {
            s.store_ad(315, &{
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (!(s.v[541] != 0.0))) && (s.v[553] != 0.0)) {
            s.store_sqrt_ad(315, A::add(A::square(s.ad_value(314)), s.ad_value(315)));
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (!(s.v[541] != 0.0))) && (s.v[553] != 0.0)) {
            s.store_sub_from_scalar_ad(350, p.p85, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5));
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (!(s.v[541] != 0.0))) && (s.v[553] != 0.0)) {
            s.store_offset(314, 350, (((-s.v[62])) + ((-0.01))));
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (!(s.v[541] != 0.0))) && (s.v[553] != 0.0)) {
            s.store_scalar(315, ((4.0 * s.v[62]) * 0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (!(s.v[541] != 0.0))) && (s.v[553] != 0.0)) {
            s.store_ad(315, &{
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (!(s.v[541] != 0.0))) && (s.v[553] != 0.0)) {
            s.store_sqrt_ad(315, A::add(A::square(s.ad_value(314)), s.ad_value(315)));
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (!(s.v[541] != 0.0))) && (s.v[553] != 0.0)) {
            s.store_offset_ad(350, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5), s.v[62]);
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (!(s.v[541] != 0.0))) && (s.v[553] != 0.0)) {
            s.store_mul_ad_lhs(366, A::scale(s.ad_value(364), p.p86), 365);
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (!(s.v[541] != 0.0))) && (!(s.v[553] != 0.0))) {
            s.store_scalar(350, s.v[62]);
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (!(s.v[541] != 0.0))) && (!(s.v[553] != 0.0))) {
            s.store_scalar(359, s.v[62]);
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (!(s.v[541] != 0.0))) && (!(s.v[553] != 0.0))) {
            s.store_scalar(366, 0.0);
        }

        s.v[554] = if ((((s.v[85] * ((s.v[262] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85))))) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (!(s.v[541] != 0.0))) && (s.v[554] != 0.0)) {
            s.store_exp_ad(281, A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]));
        }

        s.v[555] = if ((s.v[85] * ((s.v[262] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85)))) < (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (!(s.v[541] != 0.0))) && (!(s.v[554] != 0.0))) && (s.v[555] != 0.0)) {
            let assign10190_ad_e12819: A = A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85])), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85])), 0.3333333333333333), 1.0));
            s.store_div_from_scalar_ad(281, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85])), A::offset(A::scale(assign10190_ad_e12819, 0.5), 1.0)), 1.0));
        }

        if (((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (!(s.v[541] != 0.0))) && (!(s.v[554] != 0.0))) && (!(s.v[555] != 0.0))) {
            let assign10200_ad_e12898: A = A::mul(A::offset(A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]), (-230.25850929940458)), A::offset(A::scale(A::offset(A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]), (-230.25850929940458)), 0.3333333333333333), 1.0));
            s.store_scale_ad(281, A::offset(A::mul(A::offset(A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]), (-230.25850929940458)), A::offset(A::scale(assign10200_ad_e12898, 0.5), 1.0)), 1.0), 1e100);
        }

        if (((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (!(s.v[541] != 0.0))) {
            s.store_scale_ad(367, A::add(A::div(A::sub(s.ad_value(359), A::mul(s.ad_value(262), s.ad_value(366))), A::square(s.ad_value(359))), A::div(A::mul(s.ad_value(362), s.ad_value(366)), A::scale(s.ad_value(350), p.p85))), s.v[85]);
        }

        if (((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (!(s.v[541] != 0.0))) {
            s.store_mul_ad_lhs(370, A::offset(A::mul(A::sub(s.ad_value(191), s.ad_value(262)), s.ad_value(367)), 1.0), 281);
        }

        if (((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (!(s.v[541] != 0.0))) {
            s.store_scale_ad(363, A::square(s.ad_value(318)), 1.0 / (s.v[310]));
        }

        if (((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (!(s.v[541] != 0.0))) {
            s.store_scale_ad(362, A::ln(A::div_from_scalar(s.v[310], s.ad_value(363))), (s.v[64] / s.v[85]));
        }

        s.v[556] = if (s.v[64] < p.p85) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (!(s.v[541] != 0.0))) && (s.v[556] != 0.0)) {
            s.store_offset_ad(360, A::scale(A::sub(s.ad_value(262), s.ad_value(362)), p.p86), s.v[64]);
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (!(s.v[541] != 0.0))) && (s.v[556] != 0.0)) {
            s.store_sub_from_scalar_ad(350, s.v[64], A::scale(s.ad_value(362), p.p86));
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (!(s.v[541] != 0.0))) && (s.v[556] != 0.0)) {
            s.store_offset_ad(314, A::sub_from_scalar(p.p85, s.ad_value(360)), (-0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (!(s.v[541] != 0.0))) && (s.v[556] != 0.0)) {
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (!(s.v[541] != 0.0))) && (s.v[556] != 0.0)) {
            s.store_ad(315, &{
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (!(s.v[541] != 0.0))) && (s.v[556] != 0.0)) {
            s.store_sqrt_ad(315, A::add(A::square(s.ad_value(314)), s.ad_value(315)));
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (!(s.v[541] != 0.0))) && (s.v[556] != 0.0)) {
            s.store_scale_ad(364, A::offset(A::div(s.ad_value(314), s.ad_value(315)), 1.0), 0.5);
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (!(s.v[541] != 0.0))) && (s.v[556] != 0.0)) {
            s.store_sub_from_scalar_ad(361, p.p85, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5));
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (!(s.v[541] != 0.0))) && (s.v[556] != 0.0)) {
            s.store_offset(314, 361, (((-s.v[64])) + ((-0.01))));
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (!(s.v[541] != 0.0))) && (s.v[556] != 0.0)) {
            s.store_scalar(315, ((4.0 * s.v[64]) * 0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (!(s.v[541] != 0.0))) && (s.v[556] != 0.0)) {
            s.store_ad(315, &{
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (!(s.v[541] != 0.0))) && (s.v[556] != 0.0)) {
            s.store_sqrt_ad(315, A::add(A::square(s.ad_value(314)), s.ad_value(315)));
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (!(s.v[541] != 0.0))) && (s.v[556] != 0.0)) {
            s.store_scale_ad(365, A::offset(A::div(s.ad_value(314), s.ad_value(315)), 1.0), 0.5);
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
        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (!(s.v[541] != 0.0))) && (s.v[556] != 0.0)) {
            s.store_offset_ad(359, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5), s.v[64]);
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (!(s.v[541] != 0.0))) && (s.v[556] != 0.0)) {
            s.store_offset_ad(314, A::sub_from_scalar(p.p85, s.ad_value(350)), (-0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (!(s.v[541] != 0.0))) && (s.v[556] != 0.0)) {
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (!(s.v[541] != 0.0))) && (s.v[556] != 0.0)) {
            s.store_ad(315, &{
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (!(s.v[541] != 0.0))) && (s.v[556] != 0.0)) {
            s.store_sqrt_ad(315, A::add(A::square(s.ad_value(314)), s.ad_value(315)));
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (!(s.v[541] != 0.0))) && (s.v[556] != 0.0)) {
            s.store_sub_from_scalar_ad(350, p.p85, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5));
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (!(s.v[541] != 0.0))) && (s.v[556] != 0.0)) {
            s.store_offset(314, 350, (((-s.v[64])) + ((-0.01))));
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (!(s.v[541] != 0.0))) && (s.v[556] != 0.0)) {
            s.store_scalar(315, ((4.0 * s.v[64]) * 0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (!(s.v[541] != 0.0))) && (s.v[556] != 0.0)) {
            s.store_ad(315, &{
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (!(s.v[541] != 0.0))) && (s.v[556] != 0.0)) {
            s.store_sqrt_ad(315, A::add(A::square(s.ad_value(314)), s.ad_value(315)));
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (!(s.v[541] != 0.0))) && (s.v[556] != 0.0)) {
            s.store_offset_ad(350, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5), s.v[64]);
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (!(s.v[541] != 0.0))) && (s.v[556] != 0.0)) {
            s.store_mul_ad_lhs(366, A::scale(s.ad_value(364), p.p86), 365);
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (!(s.v[541] != 0.0))) && (!(s.v[556] != 0.0))) {
            s.store_scalar(350, s.v[64]);
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (!(s.v[541] != 0.0))) && (!(s.v[556] != 0.0))) {
            s.store_scalar(359, s.v[64]);
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (!(s.v[541] != 0.0))) && (!(s.v[556] != 0.0))) {
            s.store_scalar(366, 0.0);
        }

        s.v[557] = if ((((s.v[85] * ((s.v[262] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85))))) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (!(s.v[541] != 0.0))) && (s.v[557] != 0.0)) {
            s.store_exp_ad(282, A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]));
        }

        s.v[558] = if ((s.v[85] * ((s.v[262] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85)))) < (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (!(s.v[541] != 0.0))) && (!(s.v[557] != 0.0))) && (s.v[558] != 0.0)) {
            let assign10570_ad_e13554: A = A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85])), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85])), 0.3333333333333333), 1.0));
            s.store_div_from_scalar_ad(282, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85])), A::offset(A::scale(assign10570_ad_e13554, 0.5), 1.0)), 1.0));
        }

        if (((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (!(s.v[541] != 0.0))) && (!(s.v[557] != 0.0))) && (!(s.v[558] != 0.0))) {
            let assign10580_ad_e13633: A = A::mul(A::offset(A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]), (-230.25850929940458)), A::offset(A::scale(A::offset(A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]), (-230.25850929940458)), 0.3333333333333333), 1.0));
            s.store_scale_ad(282, A::offset(A::mul(A::offset(A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]), (-230.25850929940458)), A::offset(A::scale(assign10580_ad_e13633, 0.5), 1.0)), 1.0), 1e100);
        }

        if (((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (!(s.v[541] != 0.0))) {
            s.store_scale_ad(367, A::add(A::div(A::sub(s.ad_value(359), A::mul(s.ad_value(262), s.ad_value(366))), A::square(s.ad_value(359))), A::div(A::mul(s.ad_value(362), s.ad_value(366)), A::scale(s.ad_value(350), p.p85))), s.v[85]);
        }

        if (((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (!(s.v[541] != 0.0))) {
            s.store_mul_ad_lhs(371, A::offset(A::mul(A::sub(s.ad_value(191), s.ad_value(262)), s.ad_value(367)), 1.0), 282);
        }

        if (((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (!(s.v[541] != 0.0))) {
            s.store_scale_ad(363, A::square(s.ad_value(318)), 1.0 / (s.v[309]));
        }

        if (((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (!(s.v[541] != 0.0))) {
            s.store_scale_ad(362, A::ln(A::div_from_scalar(s.v[309], s.ad_value(363))), (s.v[63] / s.v[85]));
        }

        s.v[559] = if (s.v[63] < p.p85) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (!(s.v[541] != 0.0))) && (s.v[559] != 0.0)) {
            s.store_offset_ad(360, A::scale(A::sub(s.ad_value(262), s.ad_value(362)), p.p86), s.v[63]);
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (!(s.v[541] != 0.0))) && (s.v[559] != 0.0)) {
            s.store_sub_from_scalar_ad(350, s.v[63], A::scale(s.ad_value(362), p.p86));
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (!(s.v[541] != 0.0))) && (s.v[559] != 0.0)) {
            s.store_offset_ad(314, A::sub_from_scalar(p.p85, s.ad_value(360)), (-0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (!(s.v[541] != 0.0))) && (s.v[559] != 0.0)) {
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (!(s.v[541] != 0.0))) && (s.v[559] != 0.0)) {
            s.store_ad(315, &{
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (!(s.v[541] != 0.0))) && (s.v[559] != 0.0)) {
            s.store_sqrt_ad(315, A::add(A::square(s.ad_value(314)), s.ad_value(315)));
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (!(s.v[541] != 0.0))) && (s.v[559] != 0.0)) {
            s.store_scale_ad(364, A::offset(A::div(s.ad_value(314), s.ad_value(315)), 1.0), 0.5);
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (!(s.v[541] != 0.0))) && (s.v[559] != 0.0)) {
            s.store_sub_from_scalar_ad(361, p.p85, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5));
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (!(s.v[541] != 0.0))) && (s.v[559] != 0.0)) {
            s.store_offset(314, 361, (((-s.v[63])) + ((-0.01))));
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (!(s.v[541] != 0.0))) && (s.v[559] != 0.0)) {
            s.store_scalar(315, ((4.0 * s.v[63]) * 0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (!(s.v[541] != 0.0))) && (s.v[559] != 0.0)) {
            s.store_ad(315, &{
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (!(s.v[541] != 0.0))) && (s.v[559] != 0.0)) {
            s.store_sqrt_ad(315, A::add(A::square(s.ad_value(314)), s.ad_value(315)));
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (!(s.v[541] != 0.0))) && (s.v[559] != 0.0)) {
            s.store_scale_ad(365, A::offset(A::div(s.ad_value(314), s.ad_value(315)), 1.0), 0.5);
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (!(s.v[541] != 0.0))) && (s.v[559] != 0.0)) {
            s.store_offset_ad(359, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5), s.v[63]);
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (!(s.v[541] != 0.0))) && (s.v[559] != 0.0)) {
            s.store_offset_ad(314, A::sub_from_scalar(p.p85, s.ad_value(350)), (-0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (!(s.v[541] != 0.0))) && (s.v[559] != 0.0)) {
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (!(s.v[541] != 0.0))) && (s.v[559] != 0.0)) {
            s.store_ad(315, &{
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (!(s.v[541] != 0.0))) && (s.v[559] != 0.0)) {
            s.store_sqrt_ad(315, A::add(A::square(s.ad_value(314)), s.ad_value(315)));
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (!(s.v[541] != 0.0))) && (s.v[559] != 0.0)) {
            s.store_sub_from_scalar_ad(350, p.p85, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5));
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (!(s.v[541] != 0.0))) && (s.v[559] != 0.0)) {
            s.store_offset(314, 350, (((-s.v[63])) + ((-0.01))));
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (!(s.v[541] != 0.0))) && (s.v[559] != 0.0)) {
            s.store_scalar(315, ((4.0 * s.v[63]) * 0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (!(s.v[541] != 0.0))) && (s.v[559] != 0.0)) {
            s.store_ad(315, &{
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (!(s.v[541] != 0.0))) && (s.v[559] != 0.0)) {
            s.store_sqrt_ad(315, A::add(A::square(s.ad_value(314)), s.ad_value(315)));
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (!(s.v[541] != 0.0))) && (s.v[559] != 0.0)) {
            s.store_offset_ad(350, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5), s.v[63]);
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (!(s.v[541] != 0.0))) && (s.v[559] != 0.0)) {
            s.store_mul_ad_lhs(366, A::scale(s.ad_value(364), p.p86), 365);
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (!(s.v[541] != 0.0))) && (!(s.v[559] != 0.0))) {
            s.store_scalar(350, s.v[63]);
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (!(s.v[541] != 0.0))) && (!(s.v[559] != 0.0))) {
            s.store_scalar(359, s.v[63]);
        }

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (!(s.v[541] != 0.0))) && (!(s.v[559] != 0.0))) {
            s.store_scalar(366, 0.0);
        }

        s.v[560] = if ((((s.v[85] * ((s.v[262] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85))))) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (!(s.v[541] != 0.0))) && (s.v[560] != 0.0)) {
            s.store_exp_ad(283, A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]));
        }

        s.v[561] = if ((s.v[85] * ((s.v[262] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85)))) < (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (!(s.v[541] != 0.0))) && (!(s.v[560] != 0.0))) && (s.v[561] != 0.0)) {
            let assign10950_ad_e14289: A = A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85])), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85])), 0.3333333333333333), 1.0));
            s.store_div_from_scalar_ad(283, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85])), A::offset(A::scale(assign10950_ad_e14289, 0.5), 1.0)), 1.0));
        }

        if (((((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (!(s.v[541] != 0.0))) && (!(s.v[560] != 0.0))) && (!(s.v[561] != 0.0))) {
            let assign10960_ad_e14368: A = A::mul(A::offset(A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]), (-230.25850929940458)), A::offset(A::scale(A::offset(A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]), (-230.25850929940458)), 0.3333333333333333), 1.0));
            s.store_scale_ad(283, A::offset(A::mul(A::offset(A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]), (-230.25850929940458)), A::offset(A::scale(assign10960_ad_e14368, 0.5), 1.0)), 1.0), 1e100);
        }

        if (((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (!(s.v[541] != 0.0))) {
            s.store_scale_ad(367, A::add(A::div(A::sub(s.ad_value(359), A::mul(s.ad_value(262), s.ad_value(366))), A::square(s.ad_value(359))), A::div(A::mul(s.ad_value(362), s.ad_value(366)), A::scale(s.ad_value(350), p.p85))), s.v[85]);
        }

        if (((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (!(s.v[541] != 0.0))) {
            s.store_mul_ad_lhs(372, A::offset(A::mul(A::sub(s.ad_value(191), s.ad_value(262)), s.ad_value(367)), 1.0), 283);
        }

        if ((s.v[418] != 0.0) && (s.v[540] != 0.0)) {
            s.store_offset(370, 370, (-1.0));
        }

        if ((s.v[418] != 0.0) && (s.v[540] != 0.0)) {
            s.store_offset(371, 371, (-1.0));
        }

        if ((s.v[418] != 0.0) && (s.v[540] != 0.0)) {
            s.store_offset(372, 372, (-1.0));
        }

        if ((s.v[418] != 0.0) && (s.v[540] != 0.0)) {
            s.store_div_from_scalar(429, 1.0, 430);
        }

        s.v[562] = if (s.v[191] > 0.0) { 1.0 } else { 0.0 };

        if (((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (s.v[562] != 0.0)) {
            s.store_scale_ad(431, A::ln(A::add(A::offset(s.ad_value(429), 2.0), A::sqrt(A::mul(A::offset(s.ad_value(429), 1.0), A::offset(s.ad_value(429), 3.0))))), (s.v[84] * 2.0));
        }

        if (((s.v[418] != 0.0) && (s.v[540] != 0.0)) && (!(s.v[562] != 0.0))) {
            s.store_sub_ad_lhs(431, A::scale(A::ln(A::add(A::offset(A::scale(s.ad_value(430), 2.0), 1.0), A::sqrt(A::mul(A::offset(s.ad_value(430), 1.0), A::offset(A::scale(s.ad_value(430), 3.0), 1.0))))), (s.v[84] * 2.0)), 191);
        }

        if ((s.v[418] != 0.0) && (s.v[540] != 0.0)) {
            s.store_sub(432, 264, 431);
        }

        if ((s.v[418] != 0.0) && (s.v[540] != 0.0)) {
            s.store_scale_ad(433, A::sub(A::add(s.ad_value(191), s.ad_value(432)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(191), s.ad_value(432)), A::sub(s.ad_value(191), s.ad_value(432))), ((4.0 * s.v[84]) * s.v[84])))), 0.5);
        }

        if ((s.v[418] != 0.0) && (s.v[540] != 0.0)) {
            s.store_scale_ad(434, A::sub(A::add(s.ad_value(191), s.ad_value(267)), A::sqrt(A::add(A::mul(A::sub(s.ad_value(191), s.ad_value(267)), A::sub(s.ad_value(191), s.ad_value(267))), A::mul(A::scale(s.ad_value(82), 4.0), s.ad_value(82))))), 0.5);
        }

        if ((s.v[418] != 0.0) && (s.v[540] != 0.0)) {
            s.store_scale_ad(435, A::sub(s.ad_value(191), A::sqrt(A::offset(A::mul(s.ad_value(191), s.ad_value(191)), ((4.0 * 1e-6) * 1e-6)))), 0.5);
        }

        if ((s.v[418] != 0.0) && (!(s.v[540] != 0.0))) {
            s.store_scalar(370, 0.0);
        }

        if ((s.v[418] != 0.0) && (!(s.v[540] != 0.0))) {
            s.store_scalar(371, 0.0);
        }

        if ((s.v[418] != 0.0) && (!(s.v[540] != 0.0))) {
            s.store_scalar(372, 0.0);
        }

        if ((s.v[418] != 0.0) && (!(s.v[540] != 0.0))) {
            s.store_scalar(431, 0.0);
        }

        if ((s.v[418] != 0.0) && (!(s.v[540] != 0.0))) {
            s.store_scalar(428, 0.0);
        }

        if ((s.v[418] != 0.0) && (!(s.v[540] != 0.0))) {
            s.store_scalar(430, 0.0);
        }

        if ((s.v[418] != 0.0) && (!(s.v[540] != 0.0))) {
            s.store_scalar(433, 0.0);
        }

        if ((s.v[418] != 0.0) && (!(s.v[540] != 0.0))) {
            s.store_scalar(434, 0.0);
        }

        if ((s.v[418] != 0.0) && (!(s.v[540] != 0.0))) {
            s.store_scalar(435, 0.0);
        }

        s.v[563] = if (s.v[256] == 0.0) { 1.0 } else { 0.0 };

        if ((s.v[418] != 0.0) && (s.v[563] != 0.0)) {
            s.store_scalar(268, 0.0);
        }

        if ((s.v[418] != 0.0) && (s.v[563] != 0.0)) {
            s.store_scalar(291, 0.0);
        }

        if ((s.v[418] != 0.0) && (s.v[563] != 0.0)) {
            s.store_scalar(269, 0.0);
        }

        s.v[564] = if (s.v[122] == 0.5) { 1.0 } else { 0.0 };

        if (((s.v[418] != 0.0) && (!(s.v[563] != 0.0))) && (s.v[564] != 0.0)) {
            s.store_sqrt_ad(436, A::sub_from_scalar(1.0, A::mul(s.ad_value(428), s.ad_value(119))));
        }

        if (((s.v[418] != 0.0) && (!(s.v[563] != 0.0))) && (!(s.v[564] != 0.0))) {
            s.store_powf_ad(436, A::sub_from_scalar(1.0, A::mul(s.ad_value(428), s.ad_value(119))), s.v[122]);
        }

        if ((s.v[418] != 0.0) && (!(s.v[563] != 0.0))) {
            s.store_add_ad(269, A::mul(s.ad_value(131), A::sub_from_scalar(1.0, s.ad_value(436))), A::mul(s.ad_value(134), A::sub(s.ad_value(191), s.ad_value(428))));
        }

        if ((s.v[418] != 0.0) && (!(s.v[563] != 0.0))) {
            s.store_mul(437, 101, 370);
        }

        s.v[565] = if ((s.v[20] == 0.0) && (s.v[23] == 0.0)) { 1.0 } else { 0.0 };

        if (((s.v[418] != 0.0) && (!(s.v[563] != 0.0))) && (s.v[565] != 0.0)) {
            s.store_scalar(439, 0.0);
        }

        if (((s.v[418] != 0.0) && (!(s.v[563] != 0.0))) && (s.v[565] != 0.0)) {
            s.store_scalar(442, 0.0);
        }

        if (((s.v[418] != 0.0) && (!(s.v[563] != 0.0))) && (s.v[565] != 0.0)) {
            s.store_scalar(443, 0.0);
        }

        if (((s.v[418] != 0.0) && (!(s.v[563] != 0.0))) && (s.v[565] != 0.0)) {
            s.store_scalar(444, 0.0);
        }

        if (((s.v[418] != 0.0) && (!(s.v[563] != 0.0))) && (s.v[565] != 0.0)) {
            s.store_scalar(438, 0.0);
        }

        if (((s.v[418] != 0.0) && (!(s.v[563] != 0.0))) && (!(s.v[565] != 0.0))) {
            s.store_sub(439, 107, 433);
        }

        if (((s.v[418] != 0.0) && (!(s.v[563] != 0.0))) && (!(s.v[565] != 0.0))) {
            s.store_sub_from_scalar_ad(440, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(431), s.ad_value(439)))));
        }

        s.v[566] = if (s.v[9] == 0.5) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (!(s.v[563] != 0.0))) && (!(s.v[565] != 0.0))) && (s.v[566] != 0.0)) {
            s.store_scalar(441, 0.0);
        }

        if ((((s.v[418] != 0.0) && (!(s.v[563] != 0.0))) && (!(s.v[565] != 0.0))) && (!(s.v[566] != 0.0))) {
            s.store_scale_ad(441, A::add(A::div(A::mul(A::square(s.ad_value(440)), A::ln(s.ad_value(440))), A::sub_from_scalar(1.0, s.ad_value(440))), s.ad_value(440)), (1.0 - (2.0 * s.v[9])));
        }

        if (((s.v[418] != 0.0) && (!(s.v[563] != 0.0))) && (!(s.v[565] != 0.0))) {
            s.store_add(442, 440, 441);
        }

        s.v[567] = if (s.v[9] == 0.5) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (!(s.v[563] != 0.0))) && (!(s.v[565] != 0.0))) && (s.v[567] != 0.0)) {
            s.store_sqrt_ad(436, A::scale(s.ad_value(439), s.v[143]));
        }

        if ((((s.v[418] != 0.0) && (!(s.v[563] != 0.0))) && (!(s.v[565] != 0.0))) && (!(s.v[567] != 0.0))) {
            s.store_powf_ad(436, A::scale(s.ad_value(439), s.v[143]), s.v[9]);
        }

        if (((s.v[418] != 0.0) && (!(s.v[563] != 0.0))) && (!(s.v[565] != 0.0))) {
            s.store_scale(443, 436, s.v[137]);
        }

        if (((s.v[418] != 0.0) && (!(s.v[563] != 0.0))) && (!(s.v[565] != 0.0))) {
            s.store_mul_ad_rhs(444, 98, A::mul(A::offset(s.ad_value(430), (-1.0)), s.ad_value(443)));
        }

        if (((s.v[418] != 0.0) && (!(s.v[563] != 0.0))) && (!(s.v[565] != 0.0))) {
            s.store_scaled_mul(438, 444, 442, s.v[20]);
        }

        s.v[568] = if (s.v[23] == 0.0) { 1.0 } else { 0.0 };

        if (((s.v[418] != 0.0) && (!(s.v[563] != 0.0))) && (s.v[568] != 0.0)) {
            s.store_scalar(445, 0.0);
        }

        if (((s.v[418] != 0.0) && (!(s.v[563] != 0.0))) && (!(s.v[568] != 0.0))) {
            s.store_scale_ad(446, A::div(A::scale(s.ad_value(443), s.v[122]), s.ad_value(439)), s.v[152]);
        }

        if (((s.v[418] != 0.0) && (!(s.v[563] != 0.0))) && (!(s.v[568] != 0.0))) {
            s.store_div_from_scalar(447, (0.666666666666667 * s.v[149]), 446);
        }

        if (((s.v[418] != 0.0) && (!(s.v[563] != 0.0))) && (!(s.v[568] != 0.0))) {
            s.store_square(448, 447);
        }

        if (((s.v[418] != 0.0) && (!(s.v[563] != 0.0))) && (!(s.v[568] != 0.0))) {
            s.store_sqrt_ad(449, A::div(A::square(s.ad_value(448)), A::offset(A::square(s.ad_value(448)), 1.0)));
        }

        if (((s.v[418] != 0.0) && (!(s.v[563] != 0.0))) && (!(s.v[568] != 0.0))) {
            s.store_sqrt_ad(450, A::abs(s.ad_value(449)));
        }

        if (((s.v[418] != 0.0) && (!(s.v[563] != 0.0))) && (!(s.v[568] != 0.0))) {
            s.store_mul(451, 449, 450);
        }

        s.v[569] = if (((-s.v[9]) * s.v[125]) == (-1.0)) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (!(s.v[563] != 0.0))) && (!(s.v[568] != 0.0))) && (s.v[569] != 0.0)) {
            s.store_div_from_scalar_ad(452, 1.0, A::offset(A::mul(s.ad_value(446), s.ad_value(451)), 1.0));
        }

        if ((((s.v[418] != 0.0) && (!(s.v[563] != 0.0))) && (!(s.v[568] != 0.0))) && (!(s.v[569] != 0.0))) {
            s.store_powf_ad(452, A::offset(A::mul(s.ad_value(446), s.ad_value(451)), 1.0), ((-s.v[9]) * s.v[125]));
        }

        if (((s.v[418] != 0.0) && (!(s.v[563] != 0.0))) && (!(s.v[568] != 0.0))) {
            s.store_div_ad(453, A::mul(s.ad_value(442), s.ad_value(452)), A::add(s.ad_value(442), s.ad_value(452)));
        }

        if (((s.v[418] != 0.0) && (!(s.v[563] != 0.0))) && (!(s.v[568] != 0.0))) {
            s.store_sqrt_ad(454, A::scale(A::div(s.ad_value(446), s.ad_value(450)), 0.375));
        }

        if (((s.v[418] != 0.0) && (!(s.v[563] != 0.0))) && (!(s.v[568] != 0.0))) {
            s.store_sub_ad_lhs(455, A::scale(A::mul(s.ad_value(447), s.ad_value(450)), 2.0), 449);
        }

        if (((s.v[418] != 0.0) && (!(s.v[563] != 0.0))) && (!(s.v[568] != 0.0))) {
            s.store_add_ad(456, A::sub(A::mul(A::scale(s.ad_value(447), s.v[149]), s.ad_value(450)), A::scale(s.ad_value(449), s.v[149])), A::scale(A::mul(s.ad_value(446), s.ad_value(451)), 0.5));
        }

        if (((s.v[418] != 0.0) && (!(s.v[563] != 0.0))) && (!(s.v[568] != 0.0))) {
            s.store_mul_ad_lhs(457, A::offset(s.ad_value(455), (-1.0)), 454);
        }

        if (((s.v[418] != 0.0) && (!(s.v[563] != 0.0))) && (!(s.v[568] != 0.0))) {
            s.store_square(419, 457);
        }

        s.v[570] = if (s.v[457] > 0.0) { 1.0 } else { 0.0 };

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
        if ((((s.v[418] != 0.0) && (!(s.v[563] != 0.0))) && (!(s.v[568] != 0.0))) && (s.v[570] != 0.0)) {
            s.store_div_from_scalar_ad(420, 1.0, A::offset(A::scale(s.ad_value(457), s.v[86]), 1.0));
        }

        if ((((s.v[418] != 0.0) && (!(s.v[563] != 0.0))) && (!(s.v[568] != 0.0))) && (!(s.v[570] != 0.0))) {
            s.store_div_from_scalar_ad(420, 1.0, A::sub_from_scalar(1.0, A::scale(s.ad_value(457), s.v[86])));
        }

        s.v[571] = if (((-s.v[419]) + s.v[456]) > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (!(s.v[563] != 0.0))) && (!(s.v[568] != 0.0))) && (s.v[571] != 0.0)) {
            s.store_exp_ad(436, A::sub(s.ad_value(456), s.ad_value(419)));
        }

        if ((((s.v[418] != 0.0) && (!(s.v[563] != 0.0))) && (!(s.v[568] != 0.0))) && (!(s.v[571] != 0.0))) {
            s.store_div_from_scalar_ad(436, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(456), s.ad_value(419))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(456), s.ad_value(419))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(456), s.ad_value(419))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if (((s.v[418] != 0.0) && (!(s.v[563] != 0.0))) && (!(s.v[568] != 0.0))) {
            s.store_mul_ad_lhs(421, A::add(A::add(A::scale(s.ad_value(420), 0.29214664), A::scale(A::square(s.ad_value(420)), s.v[87])), A::scale(A::mul(A::square(s.ad_value(420)), s.ad_value(420)), s.v[88])), 436);
        }

        s.v[572] = if (s.v[457] > 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (!(s.v[563] != 0.0))) && (!(s.v[568] != 0.0))) && (s.v[572] != 0.0)) {
            s.copy_ad(458, 421);
        }

        s.v[573] = if (s.v[456] > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((s.v[418] != 0.0) && (!(s.v[563] != 0.0))) && (!(s.v[568] != 0.0))) && (!(s.v[572] != 0.0))) && (s.v[573] != 0.0)) {
            s.store_exp(436, 456);
        }

        if (((((s.v[418] != 0.0) && (!(s.v[563] != 0.0))) && (!(s.v[568] != 0.0))) && (!(s.v[572] != 0.0))) && (!(s.v[573] != 0.0))) {
            s.store_div_from_scalar_ad(436, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(456)), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(456)), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), s.ad_value(456)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if ((((s.v[418] != 0.0) && (!(s.v[563] != 0.0))) && (!(s.v[568] != 0.0))) && (!(s.v[572] != 0.0))) {
            s.store_sub_ad_lhs(458, A::scale(s.ad_value(436), 2.0), 421);
        }

        if (((s.v[418] != 0.0) && (!(s.v[563] != 0.0))) && (!(s.v[568] != 0.0))) {
            s.store_scale_ad(459, A::div(A::scale(s.ad_value(458), s.v[149]), s.ad_value(454)), (1.772453850905516 * 0.5));
        }

        if (((s.v[418] != 0.0) && (!(s.v[563] != 0.0))) && (!(s.v[568] != 0.0))) {
            s.store_scale_ad(445, A::mul(A::mul(s.ad_value(444), s.ad_value(459)), s.ad_value(453)), s.v[23]);
        }

        s.v[574] = if (s.v[29] == 0.0) { 1.0 } else { 0.0 };

        if (((s.v[418] != 0.0) && (!(s.v[563] != 0.0))) && (s.v[574] != 0.0)) {
            s.store_scalar(460, 0.0);
        }

        s.v[575] = if (s.v[9] == 0.5) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (!(s.v[563] != 0.0))) && (!(s.v[574] != 0.0))) && (s.v[575] != 0.0)) {
            s.store_sqrt_ad(436, A::scale(A::sub_from_scalar(s.v[6], s.ad_value(434)), s.v[143]));
        }

        if ((((s.v[418] != 0.0) && (!(s.v[563] != 0.0))) && (!(s.v[574] != 0.0))) && (!(s.v[575] != 0.0))) {
            s.store_powf_ad(436, A::scale(A::sub_from_scalar(s.v[6], s.ad_value(434)), s.v[143]), s.v[9]);
        }

        if (((s.v[418] != 0.0) && (!(s.v[563] != 0.0))) && (!(s.v[574] != 0.0))) {
            s.store_scale_ad(461, A::div(A::scale(A::sub_from_scalar(s.v[6], s.ad_value(434)), s.v[140]), s.ad_value(436)), s.v[125]);
        }

        s.v[576] = if (((((-s.v[155]) / s.v[461])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (!(s.v[563] != 0.0))) && (!(s.v[574] != 0.0))) && (s.v[576] != 0.0)) {
            s.store_exp_ad(436, A::div(A::neg(s.ad_value(155)), s.ad_value(461)));
        }

        s.v[577] = if (((-s.v[155]) / s.v[461]) < (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((s.v[418] != 0.0) && (!(s.v[563] != 0.0))) && (!(s.v[574] != 0.0))) && (!(s.v[576] != 0.0))) && (s.v[577] != 0.0)) {
            let assign11870_ad_e15602: A = A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(155)), s.ad_value(461))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(155)), s.ad_value(461))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(155)), s.ad_value(461))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
            s.store_div_from_scalar_ad(436, 1e-100, assign11870_ad_e15602);
        }

        if (((((s.v[418] != 0.0) && (!(s.v[563] != 0.0))) && (!(s.v[574] != 0.0))) && (!(s.v[576] != 0.0))) && (!(s.v[577] != 0.0))) {
            let assign11880_ad_e15650: A = A::scale(A::offset(A::mul(A::offset(A::div(A::neg(s.ad_value(155)), s.ad_value(461)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::div(A::neg(s.ad_value(155)), s.ad_value(461)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::div(A::neg(s.ad_value(155)), s.ad_value(461)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
            s.store_ad(436, &assign11880_ad_e15650);
        }

        if (((s.v[418] != 0.0) && (!(s.v[563] != 0.0))) && (!(s.v[574] != 0.0))) {
            s.store_scale_ad(460, A::mul(A::mul(A::mul(s.ad_value(191), s.ad_value(461)), s.ad_value(461)), s.ad_value(436)), s.v[29]);
        }

        s.v[578] = if ((s.v[38] > 1000000.0) || (p.p80 == 0.0)) { 1.0 } else { 0.0 };

        if (((s.v[418] != 0.0) && (!(s.v[563] != 0.0))) && (s.v[578] != 0.0)) {
            s.store_scalar(462, 1.0);
        }

        s.v[579] = if (s.v[435] > ((-s.v[158]) * s.v[38])) { 1.0 } else { 0.0 };

        s.v[580] = if (s.v[41] == 4.0) { 1.0 } else { 0.0 };

        if (((((s.v[418] != 0.0) && (!(s.v[563] != 0.0))) && (!(s.v[578] != 0.0))) && (s.v[579] != 0.0)) && (s.v[580] != 0.0)) {
            s.store_mul_ad(436, A::mul(A::mul(A::abs(A::mul(s.ad_value(435), s.ad_value(162))), A::abs(A::mul(s.ad_value(435), s.ad_value(162)))), A::abs(A::mul(s.ad_value(435), s.ad_value(162)))), A::abs(A::mul(s.ad_value(435), s.ad_value(162))));
        }

        if (((((s.v[418] != 0.0) && (!(s.v[563] != 0.0))) && (!(s.v[578] != 0.0))) && (s.v[579] != 0.0)) && (!(s.v[580] != 0.0))) {
            s.store_powf_ad(436, A::abs(A::mul(s.ad_value(435), s.ad_value(162))), s.v[41]);
        }

        if ((((s.v[418] != 0.0) && (!(s.v[563] != 0.0))) && (!(s.v[578] != 0.0))) && (s.v[579] != 0.0)) {
            s.store_div_from_scalar_ad(462, 1.0, A::sub_from_scalar(1.0, s.ad_value(436)));
        }

        if ((((s.v[418] != 0.0) && (!(s.v[563] != 0.0))) && (!(s.v[578] != 0.0))) && (!(s.v[579] != 0.0))) {
            s.store_offset_ad(462, A::mul(A::add(s.ad_value(435), A::scale(s.ad_value(38), s.v[158])), s.ad_value(165)), s.v[159]);
        }

        if ((s.v[418] != 0.0) && (!(s.v[563] != 0.0))) {
            s.store_mul_ad_lhs(268, A::add(A::add(A::add(s.ad_value(437), s.ad_value(438)), s.ad_value(445)), s.ad_value(460)), 462);
        }

        if ((s.v[418] != 0.0) && (!(s.v[563] != 0.0))) {
            s.store_mul_ad_lhs(291, A::add(A::add(s.ad_value(438), s.ad_value(445)), s.ad_value(460)), 462);
        }

        s.v[581] = if (s.v[257] == 0.0) { 1.0 } else { 0.0 };

        if ((s.v[418] != 0.0) && (s.v[581] != 0.0)) {
            s.store_scalar(270, 0.0);
        }

        if ((s.v[418] != 0.0) && (s.v[581] != 0.0)) {
            s.store_scalar(292, 0.0);
        }

        if ((s.v[418] != 0.0) && (s.v[581] != 0.0)) {
            s.store_scalar(271, 0.0);
        }

        s.v[582] = if (s.v[123] == 0.5) { 1.0 } else { 0.0 };

        if (((s.v[418] != 0.0) && (!(s.v[581] != 0.0))) && (s.v[582] != 0.0)) {
            s.store_sqrt_ad(436, A::sub_from_scalar(1.0, A::mul(s.ad_value(428), s.ad_value(120))));
        }

        if (((s.v[418] != 0.0) && (!(s.v[581] != 0.0))) && (!(s.v[582] != 0.0))) {
            s.store_powf_ad(436, A::sub_from_scalar(1.0, A::mul(s.ad_value(428), s.ad_value(120))), s.v[123]);
        }

        if ((s.v[418] != 0.0) && (!(s.v[581] != 0.0))) {
            s.store_add_ad(271, A::mul(s.ad_value(132), A::sub_from_scalar(1.0, s.ad_value(436))), A::mul(s.ad_value(135), A::sub(s.ad_value(191), s.ad_value(428))));
        }

        if ((s.v[418] != 0.0) && (!(s.v[581] != 0.0))) {
            s.store_mul(437, 102, 371);
        }

        s.v[583] = if ((s.v[21] == 0.0) && (s.v[24] == 0.0)) { 1.0 } else { 0.0 };

        if (((s.v[418] != 0.0) && (!(s.v[581] != 0.0))) && (s.v[583] != 0.0)) {
            s.store_scalar(439, 0.0);
        }

        if (((s.v[418] != 0.0) && (!(s.v[581] != 0.0))) && (s.v[583] != 0.0)) {
            s.store_scalar(442, 0.0);
        }

        if (((s.v[418] != 0.0) && (!(s.v[581] != 0.0))) && (s.v[583] != 0.0)) {
            s.store_scalar(443, 0.0);
        }

        if (((s.v[418] != 0.0) && (!(s.v[581] != 0.0))) && (s.v[583] != 0.0)) {
            s.store_scalar(444, 0.0);
        }

        if (((s.v[418] != 0.0) && (!(s.v[581] != 0.0))) && (s.v[583] != 0.0)) {
            s.store_scalar(438, 0.0);
        }

        if (((s.v[418] != 0.0) && (!(s.v[581] != 0.0))) && (!(s.v[583] != 0.0))) {
            s.store_sub(439, 108, 433);
        }

        if (((s.v[418] != 0.0) && (!(s.v[581] != 0.0))) && (!(s.v[583] != 0.0))) {
            s.store_sub_from_scalar_ad(440, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(431), s.ad_value(439)))));
        }

        s.v[584] = if (s.v[10] == 0.5) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (!(s.v[581] != 0.0))) && (!(s.v[583] != 0.0))) && (s.v[584] != 0.0)) {
            s.store_scalar(441, 0.0);
        }

        if ((((s.v[418] != 0.0) && (!(s.v[581] != 0.0))) && (!(s.v[583] != 0.0))) && (!(s.v[584] != 0.0))) {
            s.store_scale_ad(441, A::add(A::div(A::mul(A::square(s.ad_value(440)), A::ln(s.ad_value(440))), A::sub_from_scalar(1.0, s.ad_value(440))), s.ad_value(440)), (1.0 - (2.0 * s.v[10])));
        }

        if (((s.v[418] != 0.0) && (!(s.v[581] != 0.0))) && (!(s.v[583] != 0.0))) {
            s.store_add(442, 440, 441);
        }

        s.v[585] = if (s.v[10] == 0.5) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (!(s.v[581] != 0.0))) && (!(s.v[583] != 0.0))) && (s.v[585] != 0.0)) {
            s.store_sqrt_ad(436, A::scale(s.ad_value(439), s.v[144]));
        }

        if ((((s.v[418] != 0.0) && (!(s.v[581] != 0.0))) && (!(s.v[583] != 0.0))) && (!(s.v[585] != 0.0))) {
            s.store_powf_ad(436, A::scale(s.ad_value(439), s.v[144]), s.v[10]);
        }

        if (((s.v[418] != 0.0) && (!(s.v[581] != 0.0))) && (!(s.v[583] != 0.0))) {
            s.store_scale(443, 436, s.v[138]);
        }

        if (((s.v[418] != 0.0) && (!(s.v[581] != 0.0))) && (!(s.v[583] != 0.0))) {
            s.store_mul_ad_rhs(444, 99, A::mul(A::offset(s.ad_value(430), (-1.0)), s.ad_value(443)));
        }

        if (((s.v[418] != 0.0) && (!(s.v[581] != 0.0))) && (!(s.v[583] != 0.0))) {
            s.store_scaled_mul(438, 444, 442, s.v[21]);
        }

        s.v[586] = if (s.v[24] == 0.0) { 1.0 } else { 0.0 };

        if (((s.v[418] != 0.0) && (!(s.v[581] != 0.0))) && (s.v[586] != 0.0)) {
            s.store_scalar(445, 0.0);
        }

        if (((s.v[418] != 0.0) && (!(s.v[581] != 0.0))) && (!(s.v[586] != 0.0))) {
            s.store_scale_ad(446, A::div(A::scale(s.ad_value(443), s.v[123]), s.ad_value(439)), s.v[153]);
        }

        if (((s.v[418] != 0.0) && (!(s.v[581] != 0.0))) && (!(s.v[586] != 0.0))) {
            s.store_div_from_scalar(447, (0.666666666666667 * s.v[150]), 446);
        }

        if (((s.v[418] != 0.0) && (!(s.v[581] != 0.0))) && (!(s.v[586] != 0.0))) {
            s.store_square(448, 447);
        }

        if (((s.v[418] != 0.0) && (!(s.v[581] != 0.0))) && (!(s.v[586] != 0.0))) {
            s.store_sqrt_ad(449, A::div(A::square(s.ad_value(448)), A::offset(A::square(s.ad_value(448)), 1.0)));
        }

        if (((s.v[418] != 0.0) && (!(s.v[581] != 0.0))) && (!(s.v[586] != 0.0))) {
            s.store_sqrt_ad(450, A::abs(s.ad_value(449)));
        }

        if (((s.v[418] != 0.0) && (!(s.v[581] != 0.0))) && (!(s.v[586] != 0.0))) {
            s.store_mul(451, 449, 450);
        }

        s.v[587] = if (((-s.v[10]) * s.v[126]) == (-1.0)) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (!(s.v[581] != 0.0))) && (!(s.v[586] != 0.0))) && (s.v[587] != 0.0)) {
            s.store_div_from_scalar_ad(452, 1.0, A::offset(A::mul(s.ad_value(446), s.ad_value(451)), 1.0));
        }

        if ((((s.v[418] != 0.0) && (!(s.v[581] != 0.0))) && (!(s.v[586] != 0.0))) && (!(s.v[587] != 0.0))) {
            s.store_powf_ad(452, A::offset(A::mul(s.ad_value(446), s.ad_value(451)), 1.0), ((-s.v[10]) * s.v[126]));
        }

        if (((s.v[418] != 0.0) && (!(s.v[581] != 0.0))) && (!(s.v[586] != 0.0))) {
            s.store_div_ad(453, A::mul(s.ad_value(442), s.ad_value(452)), A::add(s.ad_value(442), s.ad_value(452)));
        }

        if (((s.v[418] != 0.0) && (!(s.v[581] != 0.0))) && (!(s.v[586] != 0.0))) {
            s.store_sqrt_ad(454, A::scale(A::div(s.ad_value(446), s.ad_value(450)), 0.375));
        }

        if (((s.v[418] != 0.0) && (!(s.v[581] != 0.0))) && (!(s.v[586] != 0.0))) {
            s.store_sub_ad_lhs(455, A::scale(A::mul(s.ad_value(447), s.ad_value(450)), 2.0), 449);
        }

        if (((s.v[418] != 0.0) && (!(s.v[581] != 0.0))) && (!(s.v[586] != 0.0))) {
            s.store_add_ad(456, A::sub(A::mul(A::scale(s.ad_value(447), s.v[150]), s.ad_value(450)), A::scale(s.ad_value(449), s.v[150])), A::scale(A::mul(s.ad_value(446), s.ad_value(451)), 0.5));
        }

        if (((s.v[418] != 0.0) && (!(s.v[581] != 0.0))) && (!(s.v[586] != 0.0))) {
            s.store_mul_ad_lhs(457, A::offset(s.ad_value(455), (-1.0)), 454);
        }

        if (((s.v[418] != 0.0) && (!(s.v[581] != 0.0))) && (!(s.v[586] != 0.0))) {
            s.store_square(419, 457);
        }

        s.v[588] = if (s.v[457] > 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (!(s.v[581] != 0.0))) && (!(s.v[586] != 0.0))) && (s.v[588] != 0.0)) {
            s.store_div_from_scalar_ad(420, 1.0, A::offset(A::scale(s.ad_value(457), s.v[86]), 1.0));
        }

        if ((((s.v[418] != 0.0) && (!(s.v[581] != 0.0))) && (!(s.v[586] != 0.0))) && (!(s.v[588] != 0.0))) {
            s.store_div_from_scalar_ad(420, 1.0, A::sub_from_scalar(1.0, A::scale(s.ad_value(457), s.v[86])));
        }

        s.v[589] = if (((-s.v[419]) + s.v[456]) > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (!(s.v[581] != 0.0))) && (!(s.v[586] != 0.0))) && (s.v[589] != 0.0)) {
            s.store_exp_ad(436, A::sub(s.ad_value(456), s.ad_value(419)));
        }

        if ((((s.v[418] != 0.0) && (!(s.v[581] != 0.0))) && (!(s.v[586] != 0.0))) && (!(s.v[589] != 0.0))) {
            s.store_div_from_scalar_ad(436, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(456), s.ad_value(419))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(456), s.ad_value(419))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(456), s.ad_value(419))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if (((s.v[418] != 0.0) && (!(s.v[581] != 0.0))) && (!(s.v[586] != 0.0))) {
            s.store_mul_ad_lhs(421, A::add(A::add(A::scale(s.ad_value(420), 0.29214664), A::scale(A::square(s.ad_value(420)), s.v[87])), A::scale(A::mul(A::square(s.ad_value(420)), s.ad_value(420)), s.v[88])), 436);
        }

        s.v[590] = if (s.v[457] > 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (!(s.v[581] != 0.0))) && (!(s.v[586] != 0.0))) && (s.v[590] != 0.0)) {
            s.copy_ad(458, 421);
        }

        s.v[591] = if (s.v[456] > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((s.v[418] != 0.0) && (!(s.v[581] != 0.0))) && (!(s.v[586] != 0.0))) && (!(s.v[590] != 0.0))) && (s.v[591] != 0.0)) {
            s.store_exp(436, 456);
        }

        if (((((s.v[418] != 0.0) && (!(s.v[581] != 0.0))) && (!(s.v[586] != 0.0))) && (!(s.v[590] != 0.0))) && (!(s.v[591] != 0.0))) {
            s.store_div_from_scalar_ad(436, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(456)), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(456)), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), s.ad_value(456)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if ((((s.v[418] != 0.0) && (!(s.v[581] != 0.0))) && (!(s.v[586] != 0.0))) && (!(s.v[590] != 0.0))) {
            s.store_sub_ad_lhs(458, A::scale(s.ad_value(436), 2.0), 421);
        }

        if (((s.v[418] != 0.0) && (!(s.v[581] != 0.0))) && (!(s.v[586] != 0.0))) {
            s.store_scale_ad(459, A::div(A::scale(s.ad_value(458), s.v[150]), s.ad_value(454)), (1.772453850905516 * 0.5));
        }

        if (((s.v[418] != 0.0) && (!(s.v[581] != 0.0))) && (!(s.v[586] != 0.0))) {
            s.store_scale_ad(445, A::mul(A::mul(s.ad_value(444), s.ad_value(459)), s.ad_value(453)), s.v[24]);
        }

        s.v[592] = if (s.v[30] == 0.0) { 1.0 } else { 0.0 };

        if (((s.v[418] != 0.0) && (!(s.v[581] != 0.0))) && (s.v[592] != 0.0)) {
            s.store_scalar(460, 0.0);
        }

        s.v[593] = if (s.v[10] == 0.5) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (!(s.v[581] != 0.0))) && (!(s.v[592] != 0.0))) && (s.v[593] != 0.0)) {
            s.store_sqrt_ad(436, A::scale(A::sub_from_scalar(s.v[7], s.ad_value(434)), s.v[144]));
        }

        if ((((s.v[418] != 0.0) && (!(s.v[581] != 0.0))) && (!(s.v[592] != 0.0))) && (!(s.v[593] != 0.0))) {
            s.store_powf_ad(436, A::scale(A::sub_from_scalar(s.v[7], s.ad_value(434)), s.v[144]), s.v[10]);
        }

        if (((s.v[418] != 0.0) && (!(s.v[581] != 0.0))) && (!(s.v[592] != 0.0))) {
            s.store_scale_ad(461, A::div(A::scale(A::sub_from_scalar(s.v[7], s.ad_value(434)), s.v[141]), s.ad_value(436)), s.v[126]);
        }

        s.v[594] = if (((((-s.v[156]) / s.v[461])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (!(s.v[581] != 0.0))) && (!(s.v[592] != 0.0))) && (s.v[594] != 0.0)) {
            s.store_exp_ad(436, A::div(A::neg(s.ad_value(156)), s.ad_value(461)));
        }

        s.v[595] = if (((-s.v[156]) / s.v[461]) < (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((s.v[418] != 0.0) && (!(s.v[581] != 0.0))) && (!(s.v[592] != 0.0))) && (!(s.v[594] != 0.0))) && (s.v[595] != 0.0)) {
            let assign12680_ad_e16758: A = A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(156)), s.ad_value(461))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(156)), s.ad_value(461))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(156)), s.ad_value(461))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
            s.store_div_from_scalar_ad(436, 1e-100, assign12680_ad_e16758);
        }

        if (((((s.v[418] != 0.0) && (!(s.v[581] != 0.0))) && (!(s.v[592] != 0.0))) && (!(s.v[594] != 0.0))) && (!(s.v[595] != 0.0))) {
            let assign12690_ad_e16806: A = A::scale(A::offset(A::mul(A::offset(A::div(A::neg(s.ad_value(156)), s.ad_value(461)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::div(A::neg(s.ad_value(156)), s.ad_value(461)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::div(A::neg(s.ad_value(156)), s.ad_value(461)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
            s.store_ad(436, &assign12690_ad_e16806);
        }

        if (((s.v[418] != 0.0) && (!(s.v[581] != 0.0))) && (!(s.v[592] != 0.0))) {
            s.store_scale_ad(460, A::mul(A::mul(A::mul(s.ad_value(191), s.ad_value(461)), s.ad_value(461)), s.ad_value(436)), s.v[30]);
        }

        s.v[596] = if ((s.v[39] > 1000000.0) || (p.p80 == 0.0)) { 1.0 } else { 0.0 };

        if (((s.v[418] != 0.0) && (!(s.v[581] != 0.0))) && (s.v[596] != 0.0)) {
            s.store_scalar(462, 1.0);
        }

        s.v[597] = if (s.v[435] > ((-s.v[158]) * s.v[39])) { 1.0 } else { 0.0 };

        s.v[598] = if (s.v[42] == 4.0) { 1.0 } else { 0.0 };

        if (((((s.v[418] != 0.0) && (!(s.v[581] != 0.0))) && (!(s.v[596] != 0.0))) && (s.v[597] != 0.0)) && (s.v[598] != 0.0)) {
            s.store_mul_ad(436, A::mul(A::mul(A::abs(A::mul(s.ad_value(435), s.ad_value(163))), A::abs(A::mul(s.ad_value(435), s.ad_value(163)))), A::abs(A::mul(s.ad_value(435), s.ad_value(163)))), A::abs(A::mul(s.ad_value(435), s.ad_value(163))));
        }

        if (((((s.v[418] != 0.0) && (!(s.v[581] != 0.0))) && (!(s.v[596] != 0.0))) && (s.v[597] != 0.0)) && (!(s.v[598] != 0.0))) {
            s.store_powf_ad(436, A::abs(A::mul(s.ad_value(435), s.ad_value(163))), s.v[42]);
        }

        if ((((s.v[418] != 0.0) && (!(s.v[581] != 0.0))) && (!(s.v[596] != 0.0))) && (s.v[597] != 0.0)) {
            s.store_div_from_scalar_ad(462, 1.0, A::sub_from_scalar(1.0, s.ad_value(436)));
        }

        if ((((s.v[418] != 0.0) && (!(s.v[581] != 0.0))) && (!(s.v[596] != 0.0))) && (!(s.v[597] != 0.0))) {
            s.store_offset_ad(462, A::mul(A::add(s.ad_value(435), A::scale(s.ad_value(39), s.v[158])), s.ad_value(166)), s.v[160]);
        }

        if ((s.v[418] != 0.0) && (!(s.v[581] != 0.0))) {
            s.store_mul_ad_lhs(270, A::add(A::add(A::add(s.ad_value(437), s.ad_value(438)), s.ad_value(445)), s.ad_value(460)), 462);
        }

        if ((s.v[418] != 0.0) && (!(s.v[581] != 0.0))) {
            s.store_mul_ad_lhs(292, A::add(A::add(s.ad_value(438), s.ad_value(445)), s.ad_value(460)), 462);
        }

        s.v[599] = if (s.v[258] == 0.0) { 1.0 } else { 0.0 };

        if ((s.v[418] != 0.0) && (s.v[599] != 0.0)) {
            s.store_scalar(272, 0.0);
        }

        if ((s.v[418] != 0.0) && (s.v[599] != 0.0)) {
            s.store_scalar(293, 0.0);
        }

        if ((s.v[418] != 0.0) && (s.v[599] != 0.0)) {
            s.store_scalar(273, 0.0);
        }

        s.v[600] = if (s.v[124] == 0.5) { 1.0 } else { 0.0 };

        if (((s.v[418] != 0.0) && (!(s.v[599] != 0.0))) && (s.v[600] != 0.0)) {
            s.store_sqrt_ad(436, A::sub_from_scalar(1.0, A::mul(s.ad_value(428), s.ad_value(121))));
        }

        if (((s.v[418] != 0.0) && (!(s.v[599] != 0.0))) && (!(s.v[600] != 0.0))) {
            s.store_powf_ad(436, A::sub_from_scalar(1.0, A::mul(s.ad_value(428), s.ad_value(121))), s.v[124]);
        }

        if ((s.v[418] != 0.0) && (!(s.v[599] != 0.0))) {
            s.store_add_ad(273, A::mul(s.ad_value(133), A::sub_from_scalar(1.0, s.ad_value(436))), A::mul(s.ad_value(136), A::sub(s.ad_value(191), s.ad_value(428))));
        }

        if ((s.v[418] != 0.0) && (!(s.v[599] != 0.0))) {
            s.store_mul(437, 103, 372);
        }

        s.v[601] = if ((s.v[22] == 0.0) && (s.v[25] == 0.0)) { 1.0 } else { 0.0 };

        if (((s.v[418] != 0.0) && (!(s.v[599] != 0.0))) && (s.v[601] != 0.0)) {
            s.store_scalar(439, 0.0);
        }

        if (((s.v[418] != 0.0) && (!(s.v[599] != 0.0))) && (s.v[601] != 0.0)) {
            s.store_scalar(442, 0.0);
        }

        if (((s.v[418] != 0.0) && (!(s.v[599] != 0.0))) && (s.v[601] != 0.0)) {
            s.store_scalar(443, 0.0);
        }

        if (((s.v[418] != 0.0) && (!(s.v[599] != 0.0))) && (s.v[601] != 0.0)) {
            s.store_scalar(444, 0.0);
        }

        if (((s.v[418] != 0.0) && (!(s.v[599] != 0.0))) && (s.v[601] != 0.0)) {
            s.store_scalar(438, 0.0);
        }

        if (((s.v[418] != 0.0) && (!(s.v[599] != 0.0))) && (!(s.v[601] != 0.0))) {
            s.store_sub(439, 109, 433);
        }

        if (((s.v[418] != 0.0) && (!(s.v[599] != 0.0))) && (!(s.v[601] != 0.0))) {
            s.store_sub_from_scalar_ad(440, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(431), s.ad_value(439)))));
        }

        s.v[602] = if (s.v[11] == 0.5) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (!(s.v[599] != 0.0))) && (!(s.v[601] != 0.0))) && (s.v[602] != 0.0)) {
            s.store_scalar(441, 0.0);
        }

        if ((((s.v[418] != 0.0) && (!(s.v[599] != 0.0))) && (!(s.v[601] != 0.0))) && (!(s.v[602] != 0.0))) {
            s.store_scale_ad(441, A::add(A::div(A::mul(A::square(s.ad_value(440)), A::ln(s.ad_value(440))), A::sub_from_scalar(1.0, s.ad_value(440))), s.ad_value(440)), (1.0 - (2.0 * s.v[11])));
        }

        if (((s.v[418] != 0.0) && (!(s.v[599] != 0.0))) && (!(s.v[601] != 0.0))) {
            s.store_add(442, 440, 441);
        }

        s.v[603] = if (s.v[11] == 0.5) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (!(s.v[599] != 0.0))) && (!(s.v[601] != 0.0))) && (s.v[603] != 0.0)) {
            s.store_sqrt_ad(436, A::scale(s.ad_value(439), s.v[145]));
        }

        if ((((s.v[418] != 0.0) && (!(s.v[599] != 0.0))) && (!(s.v[601] != 0.0))) && (!(s.v[603] != 0.0))) {
            s.store_powf_ad(436, A::scale(s.ad_value(439), s.v[145]), s.v[11]);
        }

        if (((s.v[418] != 0.0) && (!(s.v[599] != 0.0))) && (!(s.v[601] != 0.0))) {
            s.store_scale(443, 436, s.v[139]);
        }

        if (((s.v[418] != 0.0) && (!(s.v[599] != 0.0))) && (!(s.v[601] != 0.0))) {
            s.store_mul_ad_rhs(444, 100, A::mul(A::offset(s.ad_value(430), (-1.0)), s.ad_value(443)));
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
        if (((s.v[418] != 0.0) && (!(s.v[599] != 0.0))) && (!(s.v[601] != 0.0))) {
            s.store_scaled_mul(438, 444, 442, s.v[22]);
        }

        s.v[604] = if (s.v[25] == 0.0) { 1.0 } else { 0.0 };

        if (((s.v[418] != 0.0) && (!(s.v[599] != 0.0))) && (s.v[604] != 0.0)) {
            s.store_scalar(445, 0.0);
        }

        if (((s.v[418] != 0.0) && (!(s.v[599] != 0.0))) && (!(s.v[604] != 0.0))) {
            s.store_scale_ad(446, A::div(A::scale(s.ad_value(443), s.v[124]), s.ad_value(439)), s.v[154]);
        }

        if (((s.v[418] != 0.0) && (!(s.v[599] != 0.0))) && (!(s.v[604] != 0.0))) {
            s.store_div_from_scalar(447, (0.666666666666667 * s.v[151]), 446);
        }

        if (((s.v[418] != 0.0) && (!(s.v[599] != 0.0))) && (!(s.v[604] != 0.0))) {
            s.store_square(448, 447);
        }

        if (((s.v[418] != 0.0) && (!(s.v[599] != 0.0))) && (!(s.v[604] != 0.0))) {
            s.store_sqrt_ad(449, A::div(A::square(s.ad_value(448)), A::offset(A::square(s.ad_value(448)), 1.0)));
        }

        if (((s.v[418] != 0.0) && (!(s.v[599] != 0.0))) && (!(s.v[604] != 0.0))) {
            s.store_sqrt_ad(450, A::abs(s.ad_value(449)));
        }

        if (((s.v[418] != 0.0) && (!(s.v[599] != 0.0))) && (!(s.v[604] != 0.0))) {
            s.store_mul(451, 449, 450);
        }

        s.v[605] = if (((-s.v[11]) * s.v[127]) == (-1.0)) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (!(s.v[599] != 0.0))) && (!(s.v[604] != 0.0))) && (s.v[605] != 0.0)) {
            s.store_div_from_scalar_ad(452, 1.0, A::offset(A::mul(s.ad_value(446), s.ad_value(451)), 1.0));
        }

        if ((((s.v[418] != 0.0) && (!(s.v[599] != 0.0))) && (!(s.v[604] != 0.0))) && (!(s.v[605] != 0.0))) {
            s.store_powf_ad(452, A::offset(A::mul(s.ad_value(446), s.ad_value(451)), 1.0), ((-s.v[11]) * s.v[127]));
        }

        if (((s.v[418] != 0.0) && (!(s.v[599] != 0.0))) && (!(s.v[604] != 0.0))) {
            s.store_div_ad(453, A::mul(s.ad_value(442), s.ad_value(452)), A::add(s.ad_value(442), s.ad_value(452)));
        }

        if (((s.v[418] != 0.0) && (!(s.v[599] != 0.0))) && (!(s.v[604] != 0.0))) {
            s.store_sqrt_ad(454, A::scale(A::div(s.ad_value(446), s.ad_value(450)), 0.375));
        }

        if (((s.v[418] != 0.0) && (!(s.v[599] != 0.0))) && (!(s.v[604] != 0.0))) {
            s.store_sub_ad_lhs(455, A::scale(A::mul(s.ad_value(447), s.ad_value(450)), 2.0), 449);
        }

        if (((s.v[418] != 0.0) && (!(s.v[599] != 0.0))) && (!(s.v[604] != 0.0))) {
            s.store_add_ad(456, A::sub(A::mul(A::scale(s.ad_value(447), s.v[151]), s.ad_value(450)), A::scale(s.ad_value(449), s.v[151])), A::scale(A::mul(s.ad_value(446), s.ad_value(451)), 0.5));
        }

        if (((s.v[418] != 0.0) && (!(s.v[599] != 0.0))) && (!(s.v[604] != 0.0))) {
            s.store_mul_ad_lhs(457, A::offset(s.ad_value(455), (-1.0)), 454);
        }

        if (((s.v[418] != 0.0) && (!(s.v[599] != 0.0))) && (!(s.v[604] != 0.0))) {
            s.store_square(419, 457);
        }

        s.v[606] = if (s.v[457] > 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (!(s.v[599] != 0.0))) && (!(s.v[604] != 0.0))) && (s.v[606] != 0.0)) {
            s.store_div_from_scalar_ad(420, 1.0, A::offset(A::scale(s.ad_value(457), s.v[86]), 1.0));
        }

        if ((((s.v[418] != 0.0) && (!(s.v[599] != 0.0))) && (!(s.v[604] != 0.0))) && (!(s.v[606] != 0.0))) {
            s.store_div_from_scalar_ad(420, 1.0, A::sub_from_scalar(1.0, A::scale(s.ad_value(457), s.v[86])));
        }

        s.v[607] = if (((-s.v[419]) + s.v[456]) > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (!(s.v[599] != 0.0))) && (!(s.v[604] != 0.0))) && (s.v[607] != 0.0)) {
            s.store_exp_ad(436, A::sub(s.ad_value(456), s.ad_value(419)));
        }

        if ((((s.v[418] != 0.0) && (!(s.v[599] != 0.0))) && (!(s.v[604] != 0.0))) && (!(s.v[607] != 0.0))) {
            s.store_div_from_scalar_ad(436, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(456), s.ad_value(419))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(456), s.ad_value(419))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(456), s.ad_value(419))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if (((s.v[418] != 0.0) && (!(s.v[599] != 0.0))) && (!(s.v[604] != 0.0))) {
            s.store_mul_ad_lhs(421, A::add(A::add(A::scale(s.ad_value(420), 0.29214664), A::scale(A::square(s.ad_value(420)), s.v[87])), A::scale(A::mul(A::square(s.ad_value(420)), s.ad_value(420)), s.v[88])), 436);
        }

        s.v[608] = if (s.v[457] > 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (!(s.v[599] != 0.0))) && (!(s.v[604] != 0.0))) && (s.v[608] != 0.0)) {
            s.copy_ad(458, 421);
        }

        s.v[609] = if (s.v[456] > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((s.v[418] != 0.0) && (!(s.v[599] != 0.0))) && (!(s.v[604] != 0.0))) && (!(s.v[608] != 0.0))) && (s.v[609] != 0.0)) {
            s.store_exp(436, 456);
        }

        if (((((s.v[418] != 0.0) && (!(s.v[599] != 0.0))) && (!(s.v[604] != 0.0))) && (!(s.v[608] != 0.0))) && (!(s.v[609] != 0.0))) {
            s.store_div_from_scalar_ad(436, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(456)), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(456)), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), s.ad_value(456)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if ((((s.v[418] != 0.0) && (!(s.v[599] != 0.0))) && (!(s.v[604] != 0.0))) && (!(s.v[608] != 0.0))) {
            s.store_sub_ad_lhs(458, A::scale(s.ad_value(436), 2.0), 421);
        }

        if (((s.v[418] != 0.0) && (!(s.v[599] != 0.0))) && (!(s.v[604] != 0.0))) {
            s.store_scale_ad(459, A::div(A::scale(s.ad_value(458), s.v[151]), s.ad_value(454)), (1.772453850905516 * 0.5));
        }

        if (((s.v[418] != 0.0) && (!(s.v[599] != 0.0))) && (!(s.v[604] != 0.0))) {
            s.store_scale_ad(445, A::mul(A::mul(s.ad_value(444), s.ad_value(459)), s.ad_value(453)), s.v[25]);
        }

        s.v[610] = if (s.v[31] == 0.0) { 1.0 } else { 0.0 };

        if (((s.v[418] != 0.0) && (!(s.v[599] != 0.0))) && (s.v[610] != 0.0)) {
            s.store_scalar(460, 0.0);
        }

        s.v[611] = if (s.v[11] == 0.5) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (!(s.v[599] != 0.0))) && (!(s.v[610] != 0.0))) && (s.v[611] != 0.0)) {
            s.store_sqrt_ad(436, A::scale(A::sub_from_scalar(s.v[8], s.ad_value(434)), s.v[145]));
        }

        if ((((s.v[418] != 0.0) && (!(s.v[599] != 0.0))) && (!(s.v[610] != 0.0))) && (!(s.v[611] != 0.0))) {
            s.store_powf_ad(436, A::scale(A::sub_from_scalar(s.v[8], s.ad_value(434)), s.v[145]), s.v[11]);
        }

        if (((s.v[418] != 0.0) && (!(s.v[599] != 0.0))) && (!(s.v[610] != 0.0))) {
            s.store_scale_ad(461, A::div(A::scale(A::sub_from_scalar(s.v[8], s.ad_value(434)), s.v[142]), s.ad_value(436)), s.v[127]);
        }

        s.v[612] = if (((((-s.v[157]) / s.v[461])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (!(s.v[599] != 0.0))) && (!(s.v[610] != 0.0))) && (s.v[612] != 0.0)) {
            s.store_exp_ad(436, A::div(A::neg(s.ad_value(157)), s.ad_value(461)));
        }

        s.v[613] = if (((-s.v[157]) / s.v[461]) < (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((s.v[418] != 0.0) && (!(s.v[599] != 0.0))) && (!(s.v[610] != 0.0))) && (!(s.v[612] != 0.0))) && (s.v[613] != 0.0)) {
            let assign13490_ad_e17914: A = A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(157)), s.ad_value(461))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(157)), s.ad_value(461))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(157)), s.ad_value(461))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
            s.store_div_from_scalar_ad(436, 1e-100, assign13490_ad_e17914);
        }

        if (((((s.v[418] != 0.0) && (!(s.v[599] != 0.0))) && (!(s.v[610] != 0.0))) && (!(s.v[612] != 0.0))) && (!(s.v[613] != 0.0))) {
            let assign13500_ad_e17962: A = A::scale(A::offset(A::mul(A::offset(A::div(A::neg(s.ad_value(157)), s.ad_value(461)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::div(A::neg(s.ad_value(157)), s.ad_value(461)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::div(A::neg(s.ad_value(157)), s.ad_value(461)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
            s.store_ad(436, &assign13500_ad_e17962);
        }

        if (((s.v[418] != 0.0) && (!(s.v[599] != 0.0))) && (!(s.v[610] != 0.0))) {
            s.store_scale_ad(460, A::mul(A::mul(A::mul(s.ad_value(191), s.ad_value(461)), s.ad_value(461)), s.ad_value(436)), s.v[31]);
        }

        s.v[614] = if ((s.v[40] > 1000000.0) || (p.p80 == 0.0)) { 1.0 } else { 0.0 };

        if (((s.v[418] != 0.0) && (!(s.v[599] != 0.0))) && (s.v[614] != 0.0)) {
            s.store_scalar(462, 1.0);
        }

        s.v[615] = if (s.v[435] > ((-s.v[158]) * s.v[40])) { 1.0 } else { 0.0 };

        s.v[616] = if (s.v[43] == 4.0) { 1.0 } else { 0.0 };

        if (((((s.v[418] != 0.0) && (!(s.v[599] != 0.0))) && (!(s.v[614] != 0.0))) && (s.v[615] != 0.0)) && (s.v[616] != 0.0)) {
            s.store_mul_ad(436, A::mul(A::mul(A::abs(A::mul(s.ad_value(435), s.ad_value(164))), A::abs(A::mul(s.ad_value(435), s.ad_value(164)))), A::abs(A::mul(s.ad_value(435), s.ad_value(164)))), A::abs(A::mul(s.ad_value(435), s.ad_value(164))));
        }

        if (((((s.v[418] != 0.0) && (!(s.v[599] != 0.0))) && (!(s.v[614] != 0.0))) && (s.v[615] != 0.0)) && (!(s.v[616] != 0.0))) {
            s.store_powf_ad(436, A::abs(A::mul(s.ad_value(435), s.ad_value(164))), s.v[43]);
        }

        if ((((s.v[418] != 0.0) && (!(s.v[599] != 0.0))) && (!(s.v[614] != 0.0))) && (s.v[615] != 0.0)) {
            s.store_div_from_scalar_ad(462, 1.0, A::sub_from_scalar(1.0, s.ad_value(436)));
        }

        if ((((s.v[418] != 0.0) && (!(s.v[599] != 0.0))) && (!(s.v[614] != 0.0))) && (!(s.v[615] != 0.0))) {
            s.store_offset_ad(462, A::mul(A::add(s.ad_value(435), A::scale(s.ad_value(40), s.v[158])), s.ad_value(167)), s.v[161]);
        }

        if ((s.v[418] != 0.0) && (!(s.v[599] != 0.0))) {
            s.store_mul_ad_lhs(272, A::add(A::add(A::add(s.ad_value(437), s.ad_value(438)), s.ad_value(445)), s.ad_value(460)), 462);
        }

        if ((s.v[418] != 0.0) && (!(s.v[599] != 0.0))) {
            s.store_mul_ad_lhs(293, A::add(A::add(s.ad_value(438), s.ad_value(445)), s.ad_value(460)), 462);
        }

        if (s.v[418] != 0.0) {
            s.store_add_ad(181, A::add(A::scale(s.ad_value(268), s.v[256]), A::scale(s.ad_value(270), s.v[257])), A::scale(s.ad_value(272), s.v[258]));
        }

        s.v[617] = if !(((s.v[256] == 0.0) && (s.v[257] == 0.0)) && (s.v[258] == 0.0)) { 1.0 } else { 0.0 };

        if ((s.v[418] != 0.0) && (s.v[617] != 0.0)) {
            s.store_mul_ad_lhs(422, A::scale(s.ad_value(265), 4.0), 265);
        }

        if ((s.v[418] != 0.0) && (s.v[617] != 0.0)) {
            s.store_div(423, 265, 266);
        }

        if ((s.v[418] != 0.0) && (s.v[617] != 0.0)) {
            s.store_add_ad_rhs(424, 192, A::mul(s.ad_value(265), s.ad_value(423)));
        }

        if ((s.v[418] != 0.0) && (s.v[617] != 0.0)) {
            s.store_add(425, 266, 424);
        }

        if ((s.v[418] != 0.0) && (s.v[617] != 0.0)) {
            s.store_sub(426, 266, 424);
        }

        if ((s.v[418] != 0.0) && (s.v[617] != 0.0)) {
            s.store_sqrt_ad(427, A::add(A::square(s.ad_value(426)), s.ad_value(422)));
        }

        if ((s.v[418] != 0.0) && (s.v[617] != 0.0)) {
            s.store_scale_ad(428, A::div(A::mul(s.ad_value(192), s.ad_value(266)), A::add(s.ad_value(425), s.ad_value(427))), 2.0);
        }

        s.v[618] = if (s.v[192] < s.v[262]) { 1.0 } else { 0.0 };

        s.v[619] = if ((((0.5 * (s.v[192] * s.v[85]))) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (s.v[618] != 0.0)) && (s.v[619] != 0.0)) {
            s.store_exp_ad(430, A::scale(s.ad_value(192), (s.v[85] * 0.5)));
        }

        s.v[620] = if ((0.5 * (s.v[192] * s.v[85])) < (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (s.v[618] != 0.0)) && (!(s.v[619] != 0.0))) && (s.v[620] != 0.0)) {
            let assign13750_ad_e18301: A = A::div_from_scalar(1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(s.ad_value(192), (s.v[85] * 0.5))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(s.ad_value(192), (s.v[85] * 0.5))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::scale(s.ad_value(192), (s.v[85] * 0.5))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
            s.store_ad(430, &assign13750_ad_e18301);
        }

        if (((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (s.v[618] != 0.0)) && (!(s.v[619] != 0.0))) && (!(s.v[620] != 0.0))) {
            s.store_scale_ad(430, A::offset(A::mul(A::offset(A::scale(s.ad_value(192), (s.v[85] * 0.5)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::scale(s.ad_value(192), (s.v[85] * 0.5)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::scale(s.ad_value(192), (s.v[85] * 0.5)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
        }

        if (((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (s.v[618] != 0.0)) {
            s.store_scale_ad(363, A::square(s.ad_value(318)), 1.0 / (s.v[308]));
        }

        if (((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (s.v[618] != 0.0)) {
            s.store_scale_ad(362, A::ln(A::div_from_scalar(s.v[308], s.ad_value(363))), (s.v[62] / s.v[85]));
        }

        s.v[621] = if (s.v[62] < p.p85) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (s.v[618] != 0.0)) && (s.v[621] != 0.0)) {
            s.store_offset_ad(360, A::scale(A::sub(s.ad_value(192), s.ad_value(362)), p.p86), s.v[62]);
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (s.v[618] != 0.0)) && (s.v[621] != 0.0)) {
            s.store_sub_from_scalar_ad(350, s.v[62], A::scale(s.ad_value(362), p.p86));
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (s.v[618] != 0.0)) && (s.v[621] != 0.0)) {
            s.store_offset_ad(314, A::sub_from_scalar(p.p85, s.ad_value(360)), (-0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (s.v[618] != 0.0)) && (s.v[621] != 0.0)) {
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (s.v[618] != 0.0)) && (s.v[621] != 0.0)) {
            s.store_ad(315, &{
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (s.v[618] != 0.0)) && (s.v[621] != 0.0)) {
            s.store_sqrt_ad(315, A::add(A::square(s.ad_value(314)), s.ad_value(315)));
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (s.v[618] != 0.0)) && (s.v[621] != 0.0)) {
            s.store_sub_from_scalar_ad(361, p.p85, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5));
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (s.v[618] != 0.0)) && (s.v[621] != 0.0)) {
            s.store_offset(314, 361, (((-s.v[62])) + ((-0.01))));
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (s.v[618] != 0.0)) && (s.v[621] != 0.0)) {
            s.store_scalar(315, ((4.0 * s.v[62]) * 0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (s.v[618] != 0.0)) && (s.v[621] != 0.0)) {
            s.store_ad(315, &{
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (s.v[618] != 0.0)) && (s.v[621] != 0.0)) {
            s.store_sqrt_ad(315, A::add(A::square(s.ad_value(314)), s.ad_value(315)));
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (s.v[618] != 0.0)) && (s.v[621] != 0.0)) {
            s.store_offset_ad(359, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5), s.v[62]);
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (s.v[618] != 0.0)) && (s.v[621] != 0.0)) {
            s.store_offset_ad(314, A::sub_from_scalar(p.p85, s.ad_value(350)), (-0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (s.v[618] != 0.0)) && (s.v[621] != 0.0)) {
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (s.v[618] != 0.0)) && (s.v[621] != 0.0)) {
            s.store_ad(315, &{
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (s.v[618] != 0.0)) && (s.v[621] != 0.0)) {
            s.store_sqrt_ad(315, A::add(A::square(s.ad_value(314)), s.ad_value(315)));
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (s.v[618] != 0.0)) && (s.v[621] != 0.0)) {
            s.store_sub_from_scalar_ad(350, p.p85, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5));
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (s.v[618] != 0.0)) && (s.v[621] != 0.0)) {
            s.store_offset(314, 350, (((-s.v[62])) + ((-0.01))));
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (s.v[618] != 0.0)) && (s.v[621] != 0.0)) {
            s.store_scalar(315, ((4.0 * s.v[62]) * 0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (s.v[618] != 0.0)) && (s.v[621] != 0.0)) {
            s.store_ad(315, &{
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (s.v[618] != 0.0)) && (s.v[621] != 0.0)) {
            s.store_sqrt_ad(315, A::add(A::square(s.ad_value(314)), s.ad_value(315)));
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (s.v[618] != 0.0)) && (s.v[621] != 0.0)) {
            s.store_offset_ad(350, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5), s.v[62]);
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (s.v[618] != 0.0)) && (!(s.v[621] != 0.0))) {
            s.store_scalar(350, s.v[62]);
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (s.v[618] != 0.0)) && (!(s.v[621] != 0.0))) {
            s.store_scalar(359, s.v[62]);
        }

        s.v[622] = if ((((s.v[85] * ((s.v[192] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85))))) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (s.v[618] != 0.0)) && (s.v[622] != 0.0)) {
            s.store_exp_ad(370, A::scale(A::add(A::div(s.ad_value(192), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]));
        }

        s.v[623] = if ((s.v[85] * ((s.v[192] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85)))) < (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (s.v[618] != 0.0)) && (!(s.v[622] != 0.0))) && (s.v[623] != 0.0)) {
            let assign14070_ad_e18867: A = A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(A::add(A::div(s.ad_value(192), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85])), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::scale(A::add(A::div(s.ad_value(192), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85])), 0.3333333333333333), 1.0));
            s.store_div_from_scalar_ad(370, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(A::add(A::div(s.ad_value(192), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85])), A::offset(A::scale(assign14070_ad_e18867, 0.5), 1.0)), 1.0));
        }

        if (((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (s.v[618] != 0.0)) && (!(s.v[622] != 0.0))) && (!(s.v[623] != 0.0))) {
            let assign14080_ad_e18945: A = A::mul(A::offset(A::scale(A::add(A::div(s.ad_value(192), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]), (-230.25850929940458)), A::offset(A::scale(A::offset(A::scale(A::add(A::div(s.ad_value(192), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]), (-230.25850929940458)), 0.3333333333333333), 1.0));
            s.store_scale_ad(370, A::offset(A::mul(A::offset(A::scale(A::add(A::div(s.ad_value(192), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]), (-230.25850929940458)), A::offset(A::scale(assign14080_ad_e18945, 0.5), 1.0)), 1.0), 1e100);
        }

        if (((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (s.v[618] != 0.0)) {
            s.store_scale_ad(363, A::square(s.ad_value(318)), 1.0 / (s.v[310]));
        }

        if (((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (s.v[618] != 0.0)) {
            s.store_scale_ad(362, A::ln(A::div_from_scalar(s.v[310], s.ad_value(363))), (s.v[64] / s.v[85]));
        }

        s.v[624] = if (s.v[64] < p.p85) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (s.v[618] != 0.0)) && (s.v[624] != 0.0)) {
            s.store_offset_ad(360, A::scale(A::sub(s.ad_value(192), s.ad_value(362)), p.p86), s.v[64]);
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (s.v[618] != 0.0)) && (s.v[624] != 0.0)) {
            s.store_sub_from_scalar_ad(350, s.v[64], A::scale(s.ad_value(362), p.p86));
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (s.v[618] != 0.0)) && (s.v[624] != 0.0)) {
            s.store_offset_ad(314, A::sub_from_scalar(p.p85, s.ad_value(360)), (-0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (s.v[618] != 0.0)) && (s.v[624] != 0.0)) {
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (s.v[618] != 0.0)) && (s.v[624] != 0.0)) {
            s.store_ad(315, &{
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (s.v[618] != 0.0)) && (s.v[624] != 0.0)) {
            s.store_sqrt_ad(315, A::add(A::square(s.ad_value(314)), s.ad_value(315)));
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (s.v[618] != 0.0)) && (s.v[624] != 0.0)) {
            s.store_sub_from_scalar_ad(361, p.p85, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5));
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (s.v[618] != 0.0)) && (s.v[624] != 0.0)) {
            s.store_offset(314, 361, (((-s.v[64])) + ((-0.01))));
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (s.v[618] != 0.0)) && (s.v[624] != 0.0)) {
            s.store_scalar(315, ((4.0 * s.v[64]) * 0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (s.v[618] != 0.0)) && (s.v[624] != 0.0)) {
            s.store_ad(315, &{
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (s.v[618] != 0.0)) && (s.v[624] != 0.0)) {
            s.store_sqrt_ad(315, A::add(A::square(s.ad_value(314)), s.ad_value(315)));
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (s.v[618] != 0.0)) && (s.v[624] != 0.0)) {
            s.store_offset_ad(359, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5), s.v[64]);
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (s.v[618] != 0.0)) && (s.v[624] != 0.0)) {
            s.store_offset_ad(314, A::sub_from_scalar(p.p85, s.ad_value(350)), (-0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (s.v[618] != 0.0)) && (s.v[624] != 0.0)) {
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (s.v[618] != 0.0)) && (s.v[624] != 0.0)) {
            s.store_ad(315, &{
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (s.v[618] != 0.0)) && (s.v[624] != 0.0)) {
            s.store_sqrt_ad(315, A::add(A::square(s.ad_value(314)), s.ad_value(315)));
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (s.v[618] != 0.0)) && (s.v[624] != 0.0)) {
            s.store_sub_from_scalar_ad(350, p.p85, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5));
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (s.v[618] != 0.0)) && (s.v[624] != 0.0)) {
            s.store_offset(314, 350, (((-s.v[64])) + ((-0.01))));
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (s.v[618] != 0.0)) && (s.v[624] != 0.0)) {
            s.store_scalar(315, ((4.0 * s.v[64]) * 0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (s.v[618] != 0.0)) && (s.v[624] != 0.0)) {
            s.store_ad(315, &{
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
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
        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (s.v[618] != 0.0)) && (s.v[624] != 0.0)) {
            s.store_sqrt_ad(315, A::add(A::square(s.ad_value(314)), s.ad_value(315)));
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (s.v[618] != 0.0)) && (s.v[624] != 0.0)) {
            s.store_offset_ad(350, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5), s.v[64]);
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (s.v[618] != 0.0)) && (!(s.v[624] != 0.0))) {
            s.store_scalar(350, s.v[64]);
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (s.v[618] != 0.0)) && (!(s.v[624] != 0.0))) {
            s.store_scalar(359, s.v[64]);
        }

        s.v[625] = if ((((s.v[85] * ((s.v[192] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85))))) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (s.v[618] != 0.0)) && (s.v[625] != 0.0)) {
            s.store_exp_ad(371, A::scale(A::add(A::div(s.ad_value(192), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]));
        }

        s.v[626] = if ((s.v[85] * ((s.v[192] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85)))) < (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (s.v[618] != 0.0)) && (!(s.v[625] != 0.0))) && (s.v[626] != 0.0)) {
            let assign14390_ad_e19468: A = A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(A::add(A::div(s.ad_value(192), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85])), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::scale(A::add(A::div(s.ad_value(192), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85])), 0.3333333333333333), 1.0));
            s.store_div_from_scalar_ad(371, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(A::add(A::div(s.ad_value(192), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85])), A::offset(A::scale(assign14390_ad_e19468, 0.5), 1.0)), 1.0));
        }

        if (((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (s.v[618] != 0.0)) && (!(s.v[625] != 0.0))) && (!(s.v[626] != 0.0))) {
            let assign14400_ad_e19546: A = A::mul(A::offset(A::scale(A::add(A::div(s.ad_value(192), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]), (-230.25850929940458)), A::offset(A::scale(A::offset(A::scale(A::add(A::div(s.ad_value(192), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]), (-230.25850929940458)), 0.3333333333333333), 1.0));
            s.store_scale_ad(371, A::offset(A::mul(A::offset(A::scale(A::add(A::div(s.ad_value(192), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]), (-230.25850929940458)), A::offset(A::scale(assign14400_ad_e19546, 0.5), 1.0)), 1.0), 1e100);
        }

        if (((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (s.v[618] != 0.0)) {
            s.store_scale_ad(363, A::square(s.ad_value(318)), 1.0 / (s.v[309]));
        }

        if (((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (s.v[618] != 0.0)) {
            s.store_scale_ad(362, A::ln(A::div_from_scalar(s.v[309], s.ad_value(363))), (s.v[63] / s.v[85]));
        }

        s.v[627] = if (s.v[63] < p.p85) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (s.v[618] != 0.0)) && (s.v[627] != 0.0)) {
            s.store_offset_ad(360, A::scale(A::sub(s.ad_value(192), s.ad_value(362)), p.p86), s.v[63]);
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (s.v[618] != 0.0)) && (s.v[627] != 0.0)) {
            s.store_sub_from_scalar_ad(350, s.v[63], A::scale(s.ad_value(362), p.p86));
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (s.v[618] != 0.0)) && (s.v[627] != 0.0)) {
            s.store_offset_ad(314, A::sub_from_scalar(p.p85, s.ad_value(360)), (-0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (s.v[618] != 0.0)) && (s.v[627] != 0.0)) {
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (s.v[618] != 0.0)) && (s.v[627] != 0.0)) {
            s.store_ad(315, &{
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (s.v[618] != 0.0)) && (s.v[627] != 0.0)) {
            s.store_sqrt_ad(315, A::add(A::square(s.ad_value(314)), s.ad_value(315)));
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (s.v[618] != 0.0)) && (s.v[627] != 0.0)) {
            s.store_sub_from_scalar_ad(361, p.p85, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5));
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (s.v[618] != 0.0)) && (s.v[627] != 0.0)) {
            s.store_offset(314, 361, (((-s.v[63])) + ((-0.01))));
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (s.v[618] != 0.0)) && (s.v[627] != 0.0)) {
            s.store_scalar(315, ((4.0 * s.v[63]) * 0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (s.v[618] != 0.0)) && (s.v[627] != 0.0)) {
            s.store_ad(315, &{
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (s.v[618] != 0.0)) && (s.v[627] != 0.0)) {
            s.store_sqrt_ad(315, A::add(A::square(s.ad_value(314)), s.ad_value(315)));
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (s.v[618] != 0.0)) && (s.v[627] != 0.0)) {
            s.store_offset_ad(359, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5), s.v[63]);
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (s.v[618] != 0.0)) && (s.v[627] != 0.0)) {
            s.store_offset_ad(314, A::sub_from_scalar(p.p85, s.ad_value(350)), (-0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (s.v[618] != 0.0)) && (s.v[627] != 0.0)) {
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (s.v[618] != 0.0)) && (s.v[627] != 0.0)) {
            s.store_ad(315, &{
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (s.v[618] != 0.0)) && (s.v[627] != 0.0)) {
            s.store_sqrt_ad(315, A::add(A::square(s.ad_value(314)), s.ad_value(315)));
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (s.v[618] != 0.0)) && (s.v[627] != 0.0)) {
            s.store_sub_from_scalar_ad(350, p.p85, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5));
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (s.v[618] != 0.0)) && (s.v[627] != 0.0)) {
            s.store_offset(314, 350, (((-s.v[63])) + ((-0.01))));
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (s.v[618] != 0.0)) && (s.v[627] != 0.0)) {
            s.store_scalar(315, ((4.0 * s.v[63]) * 0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (s.v[618] != 0.0)) && (s.v[627] != 0.0)) {
            s.store_ad(315, &{
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (s.v[618] != 0.0)) && (s.v[627] != 0.0)) {
            s.store_sqrt_ad(315, A::add(A::square(s.ad_value(314)), s.ad_value(315)));
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (s.v[618] != 0.0)) && (s.v[627] != 0.0)) {
            s.store_offset_ad(350, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5), s.v[63]);
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (s.v[618] != 0.0)) && (!(s.v[627] != 0.0))) {
            s.store_scalar(350, s.v[63]);
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (s.v[618] != 0.0)) && (!(s.v[627] != 0.0))) {
            s.store_scalar(359, s.v[63]);
        }

        s.v[628] = if ((((s.v[85] * ((s.v[192] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85))))) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (s.v[618] != 0.0)) && (s.v[628] != 0.0)) {
            s.store_exp_ad(372, A::scale(A::add(A::div(s.ad_value(192), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]));
        }

        s.v[629] = if ((s.v[85] * ((s.v[192] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85)))) < (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (s.v[618] != 0.0)) && (!(s.v[628] != 0.0))) && (s.v[629] != 0.0)) {
            let assign14710_ad_e20069: A = A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(A::add(A::div(s.ad_value(192), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85])), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::scale(A::add(A::div(s.ad_value(192), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85])), 0.3333333333333333), 1.0));
            s.store_div_from_scalar_ad(372, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(A::add(A::div(s.ad_value(192), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85])), A::offset(A::scale(assign14710_ad_e20069, 0.5), 1.0)), 1.0));
        }

        if (((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (s.v[618] != 0.0)) && (!(s.v[628] != 0.0))) && (!(s.v[629] != 0.0))) {
            let assign14720_ad_e20147: A = A::mul(A::offset(A::scale(A::add(A::div(s.ad_value(192), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]), (-230.25850929940458)), A::offset(A::scale(A::offset(A::scale(A::add(A::div(s.ad_value(192), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]), (-230.25850929940458)), 0.3333333333333333), 1.0));
            s.store_scale_ad(372, A::offset(A::mul(A::offset(A::scale(A::add(A::div(s.ad_value(192), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]), (-230.25850929940458)), A::offset(A::scale(assign14720_ad_e20147, 0.5), 1.0)), 1.0), 1e100);
        }

        if (((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (!(s.v[618] != 0.0))) {
            s.store_sqrt_ad(430, A::mul(A::offset(A::scale(A::sub(s.ad_value(192), s.ad_value(262)), s.v[85]), 1.0), s.ad_value(263)));
        }

        if (((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (!(s.v[618] != 0.0))) {
            s.store_scale_ad(363, A::square(s.ad_value(318)), 1.0 / (s.v[308]));
        }

        if (((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (!(s.v[618] != 0.0))) {
            s.store_scale_ad(362, A::ln(A::div_from_scalar(s.v[308], s.ad_value(363))), (s.v[62] / s.v[85]));
        }

        s.v[630] = if (s.v[62] < p.p85) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (!(s.v[618] != 0.0))) && (s.v[630] != 0.0)) {
            s.store_offset_ad(360, A::scale(A::sub(s.ad_value(262), s.ad_value(362)), p.p86), s.v[62]);
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (!(s.v[618] != 0.0))) && (s.v[630] != 0.0)) {
            s.store_sub_from_scalar_ad(350, s.v[62], A::scale(s.ad_value(362), p.p86));
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (!(s.v[618] != 0.0))) && (s.v[630] != 0.0)) {
            s.store_offset_ad(314, A::sub_from_scalar(p.p85, s.ad_value(360)), (-0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (!(s.v[618] != 0.0))) && (s.v[630] != 0.0)) {
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (!(s.v[618] != 0.0))) && (s.v[630] != 0.0)) {
            s.store_ad(315, &{
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (!(s.v[618] != 0.0))) && (s.v[630] != 0.0)) {
            s.store_sqrt_ad(315, A::add(A::square(s.ad_value(314)), s.ad_value(315)));
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (!(s.v[618] != 0.0))) && (s.v[630] != 0.0)) {
            s.store_scale_ad(364, A::offset(A::div(s.ad_value(314), s.ad_value(315)), 1.0), 0.5);
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (!(s.v[618] != 0.0))) && (s.v[630] != 0.0)) {
            s.store_sub_from_scalar_ad(361, p.p85, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5));
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (!(s.v[618] != 0.0))) && (s.v[630] != 0.0)) {
            s.store_offset(314, 361, (((-s.v[62])) + ((-0.01))));
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (!(s.v[618] != 0.0))) && (s.v[630] != 0.0)) {
            s.store_scalar(315, ((4.0 * s.v[62]) * 0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (!(s.v[618] != 0.0))) && (s.v[630] != 0.0)) {
            s.store_ad(315, &{
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (!(s.v[618] != 0.0))) && (s.v[630] != 0.0)) {
            s.store_sqrt_ad(315, A::add(A::square(s.ad_value(314)), s.ad_value(315)));
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (!(s.v[618] != 0.0))) && (s.v[630] != 0.0)) {
            s.store_scale_ad(365, A::offset(A::div(s.ad_value(314), s.ad_value(315)), 1.0), 0.5);
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (!(s.v[618] != 0.0))) && (s.v[630] != 0.0)) {
            s.store_offset_ad(359, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5), s.v[62]);
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (!(s.v[618] != 0.0))) && (s.v[630] != 0.0)) {
            s.store_offset_ad(314, A::sub_from_scalar(p.p85, s.ad_value(350)), (-0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (!(s.v[618] != 0.0))) && (s.v[630] != 0.0)) {
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (!(s.v[618] != 0.0))) && (s.v[630] != 0.0)) {
            s.store_ad(315, &{
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (!(s.v[618] != 0.0))) && (s.v[630] != 0.0)) {
            s.store_sqrt_ad(315, A::add(A::square(s.ad_value(314)), s.ad_value(315)));
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (!(s.v[618] != 0.0))) && (s.v[630] != 0.0)) {
            s.store_sub_from_scalar_ad(350, p.p85, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5));
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (!(s.v[618] != 0.0))) && (s.v[630] != 0.0)) {
            s.store_offset(314, 350, (((-s.v[62])) + ((-0.01))));
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (!(s.v[618] != 0.0))) && (s.v[630] != 0.0)) {
            s.store_scalar(315, ((4.0 * s.v[62]) * 0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (!(s.v[618] != 0.0))) && (s.v[630] != 0.0)) {
            s.store_ad(315, &{
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (!(s.v[618] != 0.0))) && (s.v[630] != 0.0)) {
            s.store_sqrt_ad(315, A::add(A::square(s.ad_value(314)), s.ad_value(315)));
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (!(s.v[618] != 0.0))) && (s.v[630] != 0.0)) {
            s.store_offset_ad(350, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5), s.v[62]);
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (!(s.v[618] != 0.0))) && (s.v[630] != 0.0)) {
            s.store_mul_ad_lhs(366, A::scale(s.ad_value(364), p.p86), 365);
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (!(s.v[618] != 0.0))) && (!(s.v[630] != 0.0))) {
            s.store_scalar(350, s.v[62]);
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (!(s.v[618] != 0.0))) && (!(s.v[630] != 0.0))) {
            s.store_scalar(359, s.v[62]);
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (!(s.v[618] != 0.0))) && (!(s.v[630] != 0.0))) {
            s.store_scalar(366, 0.0);
        }

        s.v[631] = if ((((s.v[85] * ((s.v[262] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85))))) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (!(s.v[618] != 0.0))) && (s.v[631] != 0.0)) {
            s.store_exp_ad(281, A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]));
        }

        s.v[632] = if ((s.v[85] * ((s.v[262] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85)))) < (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (!(s.v[618] != 0.0))) && (!(s.v[631] != 0.0))) && (s.v[632] != 0.0)) {
            let assign15080_ad_e20777: A = A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85])), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85])), 0.3333333333333333), 1.0));
            s.store_div_from_scalar_ad(281, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85])), A::offset(A::scale(assign15080_ad_e20777, 0.5), 1.0)), 1.0));
        }

        if (((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (!(s.v[618] != 0.0))) && (!(s.v[631] != 0.0))) && (!(s.v[632] != 0.0))) {
            let assign15090_ad_e20856: A = A::mul(A::offset(A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]), (-230.25850929940458)), A::offset(A::scale(A::offset(A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]), (-230.25850929940458)), 0.3333333333333333), 1.0));
            s.store_scale_ad(281, A::offset(A::mul(A::offset(A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]), (-230.25850929940458)), A::offset(A::scale(assign15090_ad_e20856, 0.5), 1.0)), 1.0), 1e100);
        }

        if (((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (!(s.v[618] != 0.0))) {
            s.store_scale_ad(367, A::add(A::div(A::sub(s.ad_value(359), A::mul(s.ad_value(262), s.ad_value(366))), A::square(s.ad_value(359))), A::div(A::mul(s.ad_value(362), s.ad_value(366)), A::scale(s.ad_value(350), p.p85))), s.v[85]);
        }

        if (((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (!(s.v[618] != 0.0))) {
            s.store_mul_ad_lhs(370, A::offset(A::mul(A::sub(s.ad_value(192), s.ad_value(262)), s.ad_value(367)), 1.0), 281);
        }

        if (((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (!(s.v[618] != 0.0))) {
            s.store_scale_ad(363, A::square(s.ad_value(318)), 1.0 / (s.v[310]));
        }

        if (((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (!(s.v[618] != 0.0))) {
            s.store_scale_ad(362, A::ln(A::div_from_scalar(s.v[310], s.ad_value(363))), (s.v[64] / s.v[85]));
        }

        s.v[633] = if (s.v[64] < p.p85) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (!(s.v[618] != 0.0))) && (s.v[633] != 0.0)) {
            s.store_offset_ad(360, A::scale(A::sub(s.ad_value(262), s.ad_value(362)), p.p86), s.v[64]);
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (!(s.v[618] != 0.0))) && (s.v[633] != 0.0)) {
            s.store_sub_from_scalar_ad(350, s.v[64], A::scale(s.ad_value(362), p.p86));
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (!(s.v[618] != 0.0))) && (s.v[633] != 0.0)) {
            s.store_offset_ad(314, A::sub_from_scalar(p.p85, s.ad_value(360)), (-0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (!(s.v[618] != 0.0))) && (s.v[633] != 0.0)) {
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (!(s.v[618] != 0.0))) && (s.v[633] != 0.0)) {
            s.store_ad(315, &{
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (!(s.v[618] != 0.0))) && (s.v[633] != 0.0)) {
            s.store_sqrt_ad(315, A::add(A::square(s.ad_value(314)), s.ad_value(315)));
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (!(s.v[618] != 0.0))) && (s.v[633] != 0.0)) {
            s.store_scale_ad(364, A::offset(A::div(s.ad_value(314), s.ad_value(315)), 1.0), 0.5);
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (!(s.v[618] != 0.0))) && (s.v[633] != 0.0)) {
            s.store_sub_from_scalar_ad(361, p.p85, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5));
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (!(s.v[618] != 0.0))) && (s.v[633] != 0.0)) {
            s.store_offset(314, 361, (((-s.v[64])) + ((-0.01))));
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (!(s.v[618] != 0.0))) && (s.v[633] != 0.0)) {
            s.store_scalar(315, ((4.0 * s.v[64]) * 0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (!(s.v[618] != 0.0))) && (s.v[633] != 0.0)) {
            s.store_ad(315, &{
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (!(s.v[618] != 0.0))) && (s.v[633] != 0.0)) {
            s.store_sqrt_ad(315, A::add(A::square(s.ad_value(314)), s.ad_value(315)));
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (!(s.v[618] != 0.0))) && (s.v[633] != 0.0)) {
            s.store_scale_ad(365, A::offset(A::div(s.ad_value(314), s.ad_value(315)), 1.0), 0.5);
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (!(s.v[618] != 0.0))) && (s.v[633] != 0.0)) {
            s.store_offset_ad(359, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5), s.v[64]);
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (!(s.v[618] != 0.0))) && (s.v[633] != 0.0)) {
            s.store_offset_ad(314, A::sub_from_scalar(p.p85, s.ad_value(350)), (-0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (!(s.v[618] != 0.0))) && (s.v[633] != 0.0)) {
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (!(s.v[618] != 0.0))) && (s.v[633] != 0.0)) {
            s.store_ad(315, &{
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (!(s.v[618] != 0.0))) && (s.v[633] != 0.0)) {
            s.store_sqrt_ad(315, A::add(A::square(s.ad_value(314)), s.ad_value(315)));
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (!(s.v[618] != 0.0))) && (s.v[633] != 0.0)) {
            s.store_sub_from_scalar_ad(350, p.p85, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5));
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (!(s.v[618] != 0.0))) && (s.v[633] != 0.0)) {
            s.store_offset(314, 350, (((-s.v[64])) + ((-0.01))));
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (!(s.v[618] != 0.0))) && (s.v[633] != 0.0)) {
            s.store_scalar(315, ((4.0 * s.v[64]) * 0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (!(s.v[618] != 0.0))) && (s.v[633] != 0.0)) {
            s.store_ad(315, &{
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (!(s.v[618] != 0.0))) && (s.v[633] != 0.0)) {
            s.store_sqrt_ad(315, A::add(A::square(s.ad_value(314)), s.ad_value(315)));
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (!(s.v[618] != 0.0))) && (s.v[633] != 0.0)) {
            s.store_offset_ad(350, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5), s.v[64]);
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (!(s.v[618] != 0.0))) && (s.v[633] != 0.0)) {
            s.store_mul_ad_lhs(366, A::scale(s.ad_value(364), p.p86), 365);
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (!(s.v[618] != 0.0))) && (!(s.v[633] != 0.0))) {
            s.store_scalar(350, s.v[64]);
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (!(s.v[618] != 0.0))) && (!(s.v[633] != 0.0))) {
            s.store_scalar(359, s.v[64]);
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (!(s.v[618] != 0.0))) && (!(s.v[633] != 0.0))) {
            s.store_scalar(366, 0.0);
        }

        s.v[634] = if ((((s.v[85] * ((s.v[262] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85))))) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (!(s.v[618] != 0.0))) && (s.v[634] != 0.0)) {
            s.store_exp_ad(282, A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]));
        }

        s.v[635] = if ((s.v[85] * ((s.v[262] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85)))) < (-230.25850929940458)) { 1.0 } else { 0.0 };

    }

    pub(super) fn stamp_reactive_block_11(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if (((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (!(s.v[618] != 0.0))) && (!(s.v[634] != 0.0))) && (s.v[635] != 0.0)) {
            let assign15460_ad_e21512: A = A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85])), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85])), 0.3333333333333333), 1.0));
            s.store_div_from_scalar_ad(282, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85])), A::offset(A::scale(assign15460_ad_e21512, 0.5), 1.0)), 1.0));
        }

        if (((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (!(s.v[618] != 0.0))) && (!(s.v[634] != 0.0))) && (!(s.v[635] != 0.0))) {
            let assign15470_ad_e21591: A = A::mul(A::offset(A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]), (-230.25850929940458)), A::offset(A::scale(A::offset(A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]), (-230.25850929940458)), 0.3333333333333333), 1.0));
            s.store_scale_ad(282, A::offset(A::mul(A::offset(A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]), (-230.25850929940458)), A::offset(A::scale(assign15470_ad_e21591, 0.5), 1.0)), 1.0), 1e100);
        }

        if (((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (!(s.v[618] != 0.0))) {
            s.store_scale_ad(367, A::add(A::div(A::sub(s.ad_value(359), A::mul(s.ad_value(262), s.ad_value(366))), A::square(s.ad_value(359))), A::div(A::mul(s.ad_value(362), s.ad_value(366)), A::scale(s.ad_value(350), p.p85))), s.v[85]);
        }

        if (((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (!(s.v[618] != 0.0))) {
            s.store_mul_ad_lhs(371, A::offset(A::mul(A::sub(s.ad_value(192), s.ad_value(262)), s.ad_value(367)), 1.0), 282);
        }

        if (((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (!(s.v[618] != 0.0))) {
            s.store_scale_ad(363, A::square(s.ad_value(318)), 1.0 / (s.v[309]));
        }

        if (((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (!(s.v[618] != 0.0))) {
            s.store_scale_ad(362, A::ln(A::div_from_scalar(s.v[309], s.ad_value(363))), (s.v[63] / s.v[85]));
        }

        s.v[636] = if (s.v[63] < p.p85) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (!(s.v[618] != 0.0))) && (s.v[636] != 0.0)) {
            s.store_offset_ad(360, A::scale(A::sub(s.ad_value(262), s.ad_value(362)), p.p86), s.v[63]);
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (!(s.v[618] != 0.0))) && (s.v[636] != 0.0)) {
            s.store_sub_from_scalar_ad(350, s.v[63], A::scale(s.ad_value(362), p.p86));
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (!(s.v[618] != 0.0))) && (s.v[636] != 0.0)) {
            s.store_offset_ad(314, A::sub_from_scalar(p.p85, s.ad_value(360)), (-0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (!(s.v[618] != 0.0))) && (s.v[636] != 0.0)) {
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (!(s.v[618] != 0.0))) && (s.v[636] != 0.0)) {
            s.store_ad(315, &{
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (!(s.v[618] != 0.0))) && (s.v[636] != 0.0)) {
            s.store_sqrt_ad(315, A::add(A::square(s.ad_value(314)), s.ad_value(315)));
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (!(s.v[618] != 0.0))) && (s.v[636] != 0.0)) {
            s.store_scale_ad(364, A::offset(A::div(s.ad_value(314), s.ad_value(315)), 1.0), 0.5);
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (!(s.v[618] != 0.0))) && (s.v[636] != 0.0)) {
            s.store_sub_from_scalar_ad(361, p.p85, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5));
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (!(s.v[618] != 0.0))) && (s.v[636] != 0.0)) {
            s.store_offset(314, 361, (((-s.v[63])) + ((-0.01))));
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (!(s.v[618] != 0.0))) && (s.v[636] != 0.0)) {
            s.store_scalar(315, ((4.0 * s.v[63]) * 0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (!(s.v[618] != 0.0))) && (s.v[636] != 0.0)) {
            s.store_ad(315, &{
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (!(s.v[618] != 0.0))) && (s.v[636] != 0.0)) {
            s.store_sqrt_ad(315, A::add(A::square(s.ad_value(314)), s.ad_value(315)));
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (!(s.v[618] != 0.0))) && (s.v[636] != 0.0)) {
            s.store_scale_ad(365, A::offset(A::div(s.ad_value(314), s.ad_value(315)), 1.0), 0.5);
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (!(s.v[618] != 0.0))) && (s.v[636] != 0.0)) {
            s.store_offset_ad(359, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5), s.v[63]);
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (!(s.v[618] != 0.0))) && (s.v[636] != 0.0)) {
            s.store_offset_ad(314, A::sub_from_scalar(p.p85, s.ad_value(350)), (-0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (!(s.v[618] != 0.0))) && (s.v[636] != 0.0)) {
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (!(s.v[618] != 0.0))) && (s.v[636] != 0.0)) {
            s.store_ad(315, &{
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (!(s.v[618] != 0.0))) && (s.v[636] != 0.0)) {
            s.store_sqrt_ad(315, A::add(A::square(s.ad_value(314)), s.ad_value(315)));
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (!(s.v[618] != 0.0))) && (s.v[636] != 0.0)) {
            s.store_sub_from_scalar_ad(350, p.p85, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5));
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (!(s.v[618] != 0.0))) && (s.v[636] != 0.0)) {
            s.store_offset(314, 350, (((-s.v[63])) + ((-0.01))));
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (!(s.v[618] != 0.0))) && (s.v[636] != 0.0)) {
            s.store_scalar(315, ((4.0 * s.v[63]) * 0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (!(s.v[618] != 0.0))) && (s.v[636] != 0.0)) {
            s.store_ad(315, &{
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (!(s.v[618] != 0.0))) && (s.v[636] != 0.0)) {
            s.store_sqrt_ad(315, A::add(A::square(s.ad_value(314)), s.ad_value(315)));
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (!(s.v[618] != 0.0))) && (s.v[636] != 0.0)) {
            s.store_offset_ad(350, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5), s.v[63]);
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (!(s.v[618] != 0.0))) && (s.v[636] != 0.0)) {
            s.store_mul_ad_lhs(366, A::scale(s.ad_value(364), p.p86), 365);
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (!(s.v[618] != 0.0))) && (!(s.v[636] != 0.0))) {
            s.store_scalar(350, s.v[63]);
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (!(s.v[618] != 0.0))) && (!(s.v[636] != 0.0))) {
            s.store_scalar(359, s.v[63]);
        }

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (!(s.v[618] != 0.0))) && (!(s.v[636] != 0.0))) {
            s.store_scalar(366, 0.0);
        }

        s.v[637] = if ((((s.v[85] * ((s.v[262] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85))))) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (!(s.v[618] != 0.0))) && (s.v[637] != 0.0)) {
            s.store_exp_ad(283, A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]));
        }

        s.v[638] = if ((s.v[85] * ((s.v[262] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85)))) < (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (!(s.v[618] != 0.0))) && (!(s.v[637] != 0.0))) && (s.v[638] != 0.0)) {
            let assign15840_ad_e22247: A = A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85])), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85])), 0.3333333333333333), 1.0));
            s.store_div_from_scalar_ad(283, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85])), A::offset(A::scale(assign15840_ad_e22247, 0.5), 1.0)), 1.0));
        }

        if (((((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (!(s.v[618] != 0.0))) && (!(s.v[637] != 0.0))) && (!(s.v[638] != 0.0))) {
            let assign15850_ad_e22326: A = A::mul(A::offset(A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]), (-230.25850929940458)), A::offset(A::scale(A::offset(A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]), (-230.25850929940458)), 0.3333333333333333), 1.0));
            s.store_scale_ad(283, A::offset(A::mul(A::offset(A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]), (-230.25850929940458)), A::offset(A::scale(assign15850_ad_e22326, 0.5), 1.0)), 1.0), 1e100);
        }

        if (((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (!(s.v[618] != 0.0))) {
            s.store_scale_ad(367, A::add(A::div(A::sub(s.ad_value(359), A::mul(s.ad_value(262), s.ad_value(366))), A::square(s.ad_value(359))), A::div(A::mul(s.ad_value(362), s.ad_value(366)), A::scale(s.ad_value(350), p.p85))), s.v[85]);
        }

        if (((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (!(s.v[618] != 0.0))) {
            s.store_mul_ad_lhs(372, A::offset(A::mul(A::sub(s.ad_value(192), s.ad_value(262)), s.ad_value(367)), 1.0), 283);
        }

        if ((s.v[418] != 0.0) && (s.v[617] != 0.0)) {
            s.store_offset(370, 370, (-1.0));
        }

        if ((s.v[418] != 0.0) && (s.v[617] != 0.0)) {
            s.store_offset(371, 371, (-1.0));
        }

        if ((s.v[418] != 0.0) && (s.v[617] != 0.0)) {
            s.store_offset(372, 372, (-1.0));
        }

        if ((s.v[418] != 0.0) && (s.v[617] != 0.0)) {
            s.store_div_from_scalar(429, 1.0, 430);
        }

        s.v[639] = if (s.v[192] > 0.0) { 1.0 } else { 0.0 };

        if (((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (s.v[639] != 0.0)) {
            s.store_scale_ad(431, A::ln(A::add(A::offset(s.ad_value(429), 2.0), A::sqrt(A::mul(A::offset(s.ad_value(429), 1.0), A::offset(s.ad_value(429), 3.0))))), (s.v[84] * 2.0));
        }

        if (((s.v[418] != 0.0) && (s.v[617] != 0.0)) && (!(s.v[639] != 0.0))) {
            s.store_sub_ad_lhs(431, A::scale(A::ln(A::add(A::offset(A::scale(s.ad_value(430), 2.0), 1.0), A::sqrt(A::mul(A::offset(s.ad_value(430), 1.0), A::offset(A::scale(s.ad_value(430), 3.0), 1.0))))), (s.v[84] * 2.0)), 192);
        }

        if ((s.v[418] != 0.0) && (s.v[617] != 0.0)) {
            s.store_sub(432, 264, 431);
        }

        if ((s.v[418] != 0.0) && (s.v[617] != 0.0)) {
            s.store_scale_ad(433, A::sub(A::add(s.ad_value(192), s.ad_value(432)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(192), s.ad_value(432)), A::sub(s.ad_value(192), s.ad_value(432))), ((4.0 * s.v[84]) * s.v[84])))), 0.5);
        }

        if ((s.v[418] != 0.0) && (s.v[617] != 0.0)) {
            s.store_scale_ad(434, A::sub(A::add(s.ad_value(192), s.ad_value(267)), A::sqrt(A::add(A::mul(A::sub(s.ad_value(192), s.ad_value(267)), A::sub(s.ad_value(192), s.ad_value(267))), A::mul(A::scale(s.ad_value(82), 4.0), s.ad_value(82))))), 0.5);
        }

        if ((s.v[418] != 0.0) && (s.v[617] != 0.0)) {
            s.store_scale_ad(435, A::sub(s.ad_value(192), A::sqrt(A::offset(A::mul(s.ad_value(192), s.ad_value(192)), ((4.0 * 1e-6) * 1e-6)))), 0.5);
        }

        if ((s.v[418] != 0.0) && (!(s.v[617] != 0.0))) {
            s.store_scalar(370, 0.0);
        }

        if ((s.v[418] != 0.0) && (!(s.v[617] != 0.0))) {
            s.store_scalar(371, 0.0);
        }

        if ((s.v[418] != 0.0) && (!(s.v[617] != 0.0))) {
            s.store_scalar(372, 0.0);
        }

        if ((s.v[418] != 0.0) && (!(s.v[617] != 0.0))) {
            s.store_scalar(431, 0.0);
        }

        if ((s.v[418] != 0.0) && (!(s.v[617] != 0.0))) {
            s.store_scalar(428, 0.0);
        }

        if ((s.v[418] != 0.0) && (!(s.v[617] != 0.0))) {
            s.store_scalar(430, 0.0);
        }

        if ((s.v[418] != 0.0) && (!(s.v[617] != 0.0))) {
            s.store_scalar(433, 0.0);
        }

        if ((s.v[418] != 0.0) && (!(s.v[617] != 0.0))) {
            s.store_scalar(434, 0.0);
        }

        if ((s.v[418] != 0.0) && (!(s.v[617] != 0.0))) {
            s.store_scalar(435, 0.0);
        }

        s.v[640] = if (s.v[256] == 0.0) { 1.0 } else { 0.0 };

        if ((s.v[418] != 0.0) && (s.v[640] != 0.0)) {
            s.store_scalar(268, 0.0);
        }

        if ((s.v[418] != 0.0) && (s.v[640] != 0.0)) {
            s.store_scalar(291, 0.0);
        }

        if ((s.v[418] != 0.0) && (s.v[640] != 0.0)) {
            s.store_scalar(269, 0.0);
        }

        s.v[641] = if (s.v[122] == 0.5) { 1.0 } else { 0.0 };

        if (((s.v[418] != 0.0) && (!(s.v[640] != 0.0))) && (s.v[641] != 0.0)) {
            s.store_sqrt_ad(436, A::sub_from_scalar(1.0, A::mul(s.ad_value(428), s.ad_value(119))));
        }

        if (((s.v[418] != 0.0) && (!(s.v[640] != 0.0))) && (!(s.v[641] != 0.0))) {
            s.store_powf_ad(436, A::sub_from_scalar(1.0, A::mul(s.ad_value(428), s.ad_value(119))), s.v[122]);
        }

        if ((s.v[418] != 0.0) && (!(s.v[640] != 0.0))) {
            s.store_add_ad(269, A::mul(s.ad_value(131), A::sub_from_scalar(1.0, s.ad_value(436))), A::mul(s.ad_value(134), A::sub(s.ad_value(192), s.ad_value(428))));
        }

        if ((s.v[418] != 0.0) && (!(s.v[640] != 0.0))) {
            s.store_mul(437, 101, 370);
        }

        s.v[642] = if ((s.v[20] == 0.0) && (s.v[23] == 0.0)) { 1.0 } else { 0.0 };

        if (((s.v[418] != 0.0) && (!(s.v[640] != 0.0))) && (s.v[642] != 0.0)) {
            s.store_scalar(439, 0.0);
        }

        if (((s.v[418] != 0.0) && (!(s.v[640] != 0.0))) && (s.v[642] != 0.0)) {
            s.store_scalar(442, 0.0);
        }

        if (((s.v[418] != 0.0) && (!(s.v[640] != 0.0))) && (s.v[642] != 0.0)) {
            s.store_scalar(443, 0.0);
        }

        if (((s.v[418] != 0.0) && (!(s.v[640] != 0.0))) && (s.v[642] != 0.0)) {
            s.store_scalar(444, 0.0);
        }

        if (((s.v[418] != 0.0) && (!(s.v[640] != 0.0))) && (s.v[642] != 0.0)) {
            s.store_scalar(438, 0.0);
        }

        if (((s.v[418] != 0.0) && (!(s.v[640] != 0.0))) && (!(s.v[642] != 0.0))) {
            s.store_sub(439, 107, 433);
        }

        if (((s.v[418] != 0.0) && (!(s.v[640] != 0.0))) && (!(s.v[642] != 0.0))) {
            s.store_sub_from_scalar_ad(440, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(431), s.ad_value(439)))));
        }

        s.v[643] = if (s.v[9] == 0.5) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (!(s.v[640] != 0.0))) && (!(s.v[642] != 0.0))) && (s.v[643] != 0.0)) {
            s.store_scalar(441, 0.0);
        }

        if ((((s.v[418] != 0.0) && (!(s.v[640] != 0.0))) && (!(s.v[642] != 0.0))) && (!(s.v[643] != 0.0))) {
            s.store_scale_ad(441, A::add(A::div(A::mul(A::square(s.ad_value(440)), A::ln(s.ad_value(440))), A::sub_from_scalar(1.0, s.ad_value(440))), s.ad_value(440)), (1.0 - (2.0 * s.v[9])));
        }

        if (((s.v[418] != 0.0) && (!(s.v[640] != 0.0))) && (!(s.v[642] != 0.0))) {
            s.store_add(442, 440, 441);
        }

        s.v[644] = if (s.v[9] == 0.5) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (!(s.v[640] != 0.0))) && (!(s.v[642] != 0.0))) && (s.v[644] != 0.0)) {
            s.store_sqrt_ad(436, A::scale(s.ad_value(439), s.v[143]));
        }

        if ((((s.v[418] != 0.0) && (!(s.v[640] != 0.0))) && (!(s.v[642] != 0.0))) && (!(s.v[644] != 0.0))) {
            s.store_powf_ad(436, A::scale(s.ad_value(439), s.v[143]), s.v[9]);
        }

        if (((s.v[418] != 0.0) && (!(s.v[640] != 0.0))) && (!(s.v[642] != 0.0))) {
            s.store_scale(443, 436, s.v[137]);
        }

        if (((s.v[418] != 0.0) && (!(s.v[640] != 0.0))) && (!(s.v[642] != 0.0))) {
            s.store_mul_ad_rhs(444, 98, A::mul(A::offset(s.ad_value(430), (-1.0)), s.ad_value(443)));
        }

        if (((s.v[418] != 0.0) && (!(s.v[640] != 0.0))) && (!(s.v[642] != 0.0))) {
            s.store_scaled_mul(438, 444, 442, s.v[20]);
        }

        s.v[645] = if (s.v[23] == 0.0) { 1.0 } else { 0.0 };

        if (((s.v[418] != 0.0) && (!(s.v[640] != 0.0))) && (s.v[645] != 0.0)) {
            s.store_scalar(445, 0.0);
        }

        if (((s.v[418] != 0.0) && (!(s.v[640] != 0.0))) && (!(s.v[645] != 0.0))) {
            s.store_scale_ad(446, A::div(A::scale(s.ad_value(443), s.v[122]), s.ad_value(439)), s.v[152]);
        }

        if (((s.v[418] != 0.0) && (!(s.v[640] != 0.0))) && (!(s.v[645] != 0.0))) {
            s.store_div_from_scalar(447, (0.666666666666667 * s.v[149]), 446);
        }

        if (((s.v[418] != 0.0) && (!(s.v[640] != 0.0))) && (!(s.v[645] != 0.0))) {
            s.store_square(448, 447);
        }

        if (((s.v[418] != 0.0) && (!(s.v[640] != 0.0))) && (!(s.v[645] != 0.0))) {
            s.store_sqrt_ad(449, A::div(A::square(s.ad_value(448)), A::offset(A::square(s.ad_value(448)), 1.0)));
        }

        if (((s.v[418] != 0.0) && (!(s.v[640] != 0.0))) && (!(s.v[645] != 0.0))) {
            s.store_sqrt_ad(450, A::abs(s.ad_value(449)));
        }

        if (((s.v[418] != 0.0) && (!(s.v[640] != 0.0))) && (!(s.v[645] != 0.0))) {
            s.store_mul(451, 449, 450);
        }

        s.v[646] = if (((-s.v[9]) * s.v[125]) == (-1.0)) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (!(s.v[640] != 0.0))) && (!(s.v[645] != 0.0))) && (s.v[646] != 0.0)) {
            s.store_div_from_scalar_ad(452, 1.0, A::offset(A::mul(s.ad_value(446), s.ad_value(451)), 1.0));
        }

        if ((((s.v[418] != 0.0) && (!(s.v[640] != 0.0))) && (!(s.v[645] != 0.0))) && (!(s.v[646] != 0.0))) {
            s.store_powf_ad(452, A::offset(A::mul(s.ad_value(446), s.ad_value(451)), 1.0), ((-s.v[9]) * s.v[125]));
        }

        if (((s.v[418] != 0.0) && (!(s.v[640] != 0.0))) && (!(s.v[645] != 0.0))) {
            s.store_div_ad(453, A::mul(s.ad_value(442), s.ad_value(452)), A::add(s.ad_value(442), s.ad_value(452)));
        }

        if (((s.v[418] != 0.0) && (!(s.v[640] != 0.0))) && (!(s.v[645] != 0.0))) {
            s.store_sqrt_ad(454, A::scale(A::div(s.ad_value(446), s.ad_value(450)), 0.375));
        }

        if (((s.v[418] != 0.0) && (!(s.v[640] != 0.0))) && (!(s.v[645] != 0.0))) {
            s.store_sub_ad_lhs(455, A::scale(A::mul(s.ad_value(447), s.ad_value(450)), 2.0), 449);
        }

        if (((s.v[418] != 0.0) && (!(s.v[640] != 0.0))) && (!(s.v[645] != 0.0))) {
            s.store_add_ad(456, A::sub(A::mul(A::scale(s.ad_value(447), s.v[149]), s.ad_value(450)), A::scale(s.ad_value(449), s.v[149])), A::scale(A::mul(s.ad_value(446), s.ad_value(451)), 0.5));
        }

        if (((s.v[418] != 0.0) && (!(s.v[640] != 0.0))) && (!(s.v[645] != 0.0))) {
            s.store_mul_ad_lhs(457, A::offset(s.ad_value(455), (-1.0)), 454);
        }

        if (((s.v[418] != 0.0) && (!(s.v[640] != 0.0))) && (!(s.v[645] != 0.0))) {
            s.store_square(419, 457);
        }

        s.v[647] = if (s.v[457] > 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (!(s.v[640] != 0.0))) && (!(s.v[645] != 0.0))) && (s.v[647] != 0.0)) {
            s.store_div_from_scalar_ad(420, 1.0, A::offset(A::scale(s.ad_value(457), s.v[86]), 1.0));
        }

        if ((((s.v[418] != 0.0) && (!(s.v[640] != 0.0))) && (!(s.v[645] != 0.0))) && (!(s.v[647] != 0.0))) {
            s.store_div_from_scalar_ad(420, 1.0, A::sub_from_scalar(1.0, A::scale(s.ad_value(457), s.v[86])));
        }

        s.v[648] = if (((-s.v[419]) + s.v[456]) > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (!(s.v[640] != 0.0))) && (!(s.v[645] != 0.0))) && (s.v[648] != 0.0)) {
            s.store_exp_ad(436, A::sub(s.ad_value(456), s.ad_value(419)));
        }

        if ((((s.v[418] != 0.0) && (!(s.v[640] != 0.0))) && (!(s.v[645] != 0.0))) && (!(s.v[648] != 0.0))) {
            s.store_div_from_scalar_ad(436, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(456), s.ad_value(419))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(456), s.ad_value(419))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(456), s.ad_value(419))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if (((s.v[418] != 0.0) && (!(s.v[640] != 0.0))) && (!(s.v[645] != 0.0))) {
            s.store_mul_ad_lhs(421, A::add(A::add(A::scale(s.ad_value(420), 0.29214664), A::scale(A::square(s.ad_value(420)), s.v[87])), A::scale(A::mul(A::square(s.ad_value(420)), s.ad_value(420)), s.v[88])), 436);
        }

        s.v[649] = if (s.v[457] > 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (!(s.v[640] != 0.0))) && (!(s.v[645] != 0.0))) && (s.v[649] != 0.0)) {
            s.copy_ad(458, 421);
        }

        s.v[650] = if (s.v[456] > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((s.v[418] != 0.0) && (!(s.v[640] != 0.0))) && (!(s.v[645] != 0.0))) && (!(s.v[649] != 0.0))) && (s.v[650] != 0.0)) {
            s.store_exp(436, 456);
        }

        if (((((s.v[418] != 0.0) && (!(s.v[640] != 0.0))) && (!(s.v[645] != 0.0))) && (!(s.v[649] != 0.0))) && (!(s.v[650] != 0.0))) {
            s.store_div_from_scalar_ad(436, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(456)), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(456)), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), s.ad_value(456)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if ((((s.v[418] != 0.0) && (!(s.v[640] != 0.0))) && (!(s.v[645] != 0.0))) && (!(s.v[649] != 0.0))) {
            s.store_sub_ad_lhs(458, A::scale(s.ad_value(436), 2.0), 421);
        }

        if (((s.v[418] != 0.0) && (!(s.v[640] != 0.0))) && (!(s.v[645] != 0.0))) {
            s.store_scale_ad(459, A::div(A::scale(s.ad_value(458), s.v[149]), s.ad_value(454)), (1.772453850905516 * 0.5));
        }

        if (((s.v[418] != 0.0) && (!(s.v[640] != 0.0))) && (!(s.v[645] != 0.0))) {
            s.store_scale_ad(445, A::mul(A::mul(s.ad_value(444), s.ad_value(459)), s.ad_value(453)), s.v[23]);
        }

        s.v[651] = if (s.v[29] == 0.0) { 1.0 } else { 0.0 };

        if (((s.v[418] != 0.0) && (!(s.v[640] != 0.0))) && (s.v[651] != 0.0)) {
            s.store_scalar(460, 0.0);
        }

        s.v[652] = if (s.v[9] == 0.5) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (!(s.v[640] != 0.0))) && (!(s.v[651] != 0.0))) && (s.v[652] != 0.0)) {
            s.store_sqrt_ad(436, A::scale(A::sub_from_scalar(s.v[6], s.ad_value(434)), s.v[143]));
        }

        if ((((s.v[418] != 0.0) && (!(s.v[640] != 0.0))) && (!(s.v[651] != 0.0))) && (!(s.v[652] != 0.0))) {
            s.store_powf_ad(436, A::scale(A::sub_from_scalar(s.v[6], s.ad_value(434)), s.v[143]), s.v[9]);
        }

        if (((s.v[418] != 0.0) && (!(s.v[640] != 0.0))) && (!(s.v[651] != 0.0))) {
            s.store_scale_ad(461, A::div(A::scale(A::sub_from_scalar(s.v[6], s.ad_value(434)), s.v[140]), s.ad_value(436)), s.v[125]);
        }

        s.v[653] = if (((((-s.v[155]) / s.v[461])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (!(s.v[640] != 0.0))) && (!(s.v[651] != 0.0))) && (s.v[653] != 0.0)) {
            s.store_exp_ad(436, A::div(A::neg(s.ad_value(155)), s.ad_value(461)));
        }

        s.v[654] = if (((-s.v[155]) / s.v[461]) < (-230.25850929940458)) { 1.0 } else { 0.0 };

    }

    pub(super) fn stamp_reactive_block_12(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if (((((s.v[418] != 0.0) && (!(s.v[640] != 0.0))) && (!(s.v[651] != 0.0))) && (!(s.v[653] != 0.0))) && (s.v[654] != 0.0)) {
            let assign16760_ad_e23560: A = A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(155)), s.ad_value(461))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(155)), s.ad_value(461))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(155)), s.ad_value(461))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
            s.store_div_from_scalar_ad(436, 1e-100, assign16760_ad_e23560);
        }

        if (((((s.v[418] != 0.0) && (!(s.v[640] != 0.0))) && (!(s.v[651] != 0.0))) && (!(s.v[653] != 0.0))) && (!(s.v[654] != 0.0))) {
            let assign16770_ad_e23608: A = A::scale(A::offset(A::mul(A::offset(A::div(A::neg(s.ad_value(155)), s.ad_value(461)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::div(A::neg(s.ad_value(155)), s.ad_value(461)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::div(A::neg(s.ad_value(155)), s.ad_value(461)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
            s.store_ad(436, &assign16770_ad_e23608);
        }

        if (((s.v[418] != 0.0) && (!(s.v[640] != 0.0))) && (!(s.v[651] != 0.0))) {
            s.store_scale_ad(460, A::mul(A::mul(A::mul(s.ad_value(192), s.ad_value(461)), s.ad_value(461)), s.ad_value(436)), s.v[29]);
        }

        s.v[655] = if ((s.v[38] > 1000000.0) || (p.p80 == 0.0)) { 1.0 } else { 0.0 };

        if (((s.v[418] != 0.0) && (!(s.v[640] != 0.0))) && (s.v[655] != 0.0)) {
            s.store_scalar(462, 1.0);
        }

        s.v[656] = if (s.v[435] > ((-s.v[158]) * s.v[38])) { 1.0 } else { 0.0 };

        s.v[657] = if (s.v[41] == 4.0) { 1.0 } else { 0.0 };

        if (((((s.v[418] != 0.0) && (!(s.v[640] != 0.0))) && (!(s.v[655] != 0.0))) && (s.v[656] != 0.0)) && (s.v[657] != 0.0)) {
            s.store_mul_ad(436, A::mul(A::mul(A::abs(A::mul(s.ad_value(435), s.ad_value(162))), A::abs(A::mul(s.ad_value(435), s.ad_value(162)))), A::abs(A::mul(s.ad_value(435), s.ad_value(162)))), A::abs(A::mul(s.ad_value(435), s.ad_value(162))));
        }

        if (((((s.v[418] != 0.0) && (!(s.v[640] != 0.0))) && (!(s.v[655] != 0.0))) && (s.v[656] != 0.0)) && (!(s.v[657] != 0.0))) {
            s.store_powf_ad(436, A::abs(A::mul(s.ad_value(435), s.ad_value(162))), s.v[41]);
        }

        if ((((s.v[418] != 0.0) && (!(s.v[640] != 0.0))) && (!(s.v[655] != 0.0))) && (s.v[656] != 0.0)) {
            s.store_div_from_scalar_ad(462, 1.0, A::sub_from_scalar(1.0, s.ad_value(436)));
        }

        if ((((s.v[418] != 0.0) && (!(s.v[640] != 0.0))) && (!(s.v[655] != 0.0))) && (!(s.v[656] != 0.0))) {
            s.store_offset_ad(462, A::mul(A::add(s.ad_value(435), A::scale(s.ad_value(38), s.v[158])), s.ad_value(165)), s.v[159]);
        }

        if ((s.v[418] != 0.0) && (!(s.v[640] != 0.0))) {
            s.store_mul_ad_lhs(268, A::add(A::add(A::add(s.ad_value(437), s.ad_value(438)), s.ad_value(445)), s.ad_value(460)), 462);
        }

        if ((s.v[418] != 0.0) && (!(s.v[640] != 0.0))) {
            s.store_mul_ad_lhs(291, A::add(A::add(s.ad_value(438), s.ad_value(445)), s.ad_value(460)), 462);
        }

        s.v[658] = if (s.v[257] == 0.0) { 1.0 } else { 0.0 };

        if ((s.v[418] != 0.0) && (s.v[658] != 0.0)) {
            s.store_scalar(270, 0.0);
        }

        if ((s.v[418] != 0.0) && (s.v[658] != 0.0)) {
            s.store_scalar(292, 0.0);
        }

        if ((s.v[418] != 0.0) && (s.v[658] != 0.0)) {
            s.store_scalar(271, 0.0);
        }

        s.v[659] = if (s.v[123] == 0.5) { 1.0 } else { 0.0 };

        if (((s.v[418] != 0.0) && (!(s.v[658] != 0.0))) && (s.v[659] != 0.0)) {
            s.store_sqrt_ad(436, A::sub_from_scalar(1.0, A::mul(s.ad_value(428), s.ad_value(120))));
        }

        if (((s.v[418] != 0.0) && (!(s.v[658] != 0.0))) && (!(s.v[659] != 0.0))) {
            s.store_powf_ad(436, A::sub_from_scalar(1.0, A::mul(s.ad_value(428), s.ad_value(120))), s.v[123]);
        }

        if ((s.v[418] != 0.0) && (!(s.v[658] != 0.0))) {
            s.store_add_ad(271, A::mul(s.ad_value(132), A::sub_from_scalar(1.0, s.ad_value(436))), A::mul(s.ad_value(135), A::sub(s.ad_value(192), s.ad_value(428))));
        }

        if ((s.v[418] != 0.0) && (!(s.v[658] != 0.0))) {
            s.store_mul(437, 102, 371);
        }

        s.v[660] = if ((s.v[21] == 0.0) && (s.v[24] == 0.0)) { 1.0 } else { 0.0 };

        if (((s.v[418] != 0.0) && (!(s.v[658] != 0.0))) && (s.v[660] != 0.0)) {
            s.store_scalar(439, 0.0);
        }

        if (((s.v[418] != 0.0) && (!(s.v[658] != 0.0))) && (s.v[660] != 0.0)) {
            s.store_scalar(442, 0.0);
        }

        if (((s.v[418] != 0.0) && (!(s.v[658] != 0.0))) && (s.v[660] != 0.0)) {
            s.store_scalar(443, 0.0);
        }

        if (((s.v[418] != 0.0) && (!(s.v[658] != 0.0))) && (s.v[660] != 0.0)) {
            s.store_scalar(444, 0.0);
        }

        if (((s.v[418] != 0.0) && (!(s.v[658] != 0.0))) && (s.v[660] != 0.0)) {
            s.store_scalar(438, 0.0);
        }

        if (((s.v[418] != 0.0) && (!(s.v[658] != 0.0))) && (!(s.v[660] != 0.0))) {
            s.store_sub(439, 108, 433);
        }

        if (((s.v[418] != 0.0) && (!(s.v[658] != 0.0))) && (!(s.v[660] != 0.0))) {
            s.store_sub_from_scalar_ad(440, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(431), s.ad_value(439)))));
        }

        s.v[661] = if (s.v[10] == 0.5) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (!(s.v[658] != 0.0))) && (!(s.v[660] != 0.0))) && (s.v[661] != 0.0)) {
            s.store_scalar(441, 0.0);
        }

        if ((((s.v[418] != 0.0) && (!(s.v[658] != 0.0))) && (!(s.v[660] != 0.0))) && (!(s.v[661] != 0.0))) {
            s.store_scale_ad(441, A::add(A::div(A::mul(A::square(s.ad_value(440)), A::ln(s.ad_value(440))), A::sub_from_scalar(1.0, s.ad_value(440))), s.ad_value(440)), (1.0 - (2.0 * s.v[10])));
        }

        if (((s.v[418] != 0.0) && (!(s.v[658] != 0.0))) && (!(s.v[660] != 0.0))) {
            s.store_add(442, 440, 441);
        }

        s.v[662] = if (s.v[10] == 0.5) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (!(s.v[658] != 0.0))) && (!(s.v[660] != 0.0))) && (s.v[662] != 0.0)) {
            s.store_sqrt_ad(436, A::scale(s.ad_value(439), s.v[144]));
        }

        if ((((s.v[418] != 0.0) && (!(s.v[658] != 0.0))) && (!(s.v[660] != 0.0))) && (!(s.v[662] != 0.0))) {
            s.store_powf_ad(436, A::scale(s.ad_value(439), s.v[144]), s.v[10]);
        }

        if (((s.v[418] != 0.0) && (!(s.v[658] != 0.0))) && (!(s.v[660] != 0.0))) {
            s.store_scale(443, 436, s.v[138]);
        }

        if (((s.v[418] != 0.0) && (!(s.v[658] != 0.0))) && (!(s.v[660] != 0.0))) {
            s.store_mul_ad_rhs(444, 99, A::mul(A::offset(s.ad_value(430), (-1.0)), s.ad_value(443)));
        }

        if (((s.v[418] != 0.0) && (!(s.v[658] != 0.0))) && (!(s.v[660] != 0.0))) {
            s.store_scaled_mul(438, 444, 442, s.v[21]);
        }

        s.v[663] = if (s.v[24] == 0.0) { 1.0 } else { 0.0 };

        if (((s.v[418] != 0.0) && (!(s.v[658] != 0.0))) && (s.v[663] != 0.0)) {
            s.store_scalar(445, 0.0);
        }

        if (((s.v[418] != 0.0) && (!(s.v[658] != 0.0))) && (!(s.v[663] != 0.0))) {
            s.store_scale_ad(446, A::div(A::scale(s.ad_value(443), s.v[123]), s.ad_value(439)), s.v[153]);
        }

        if (((s.v[418] != 0.0) && (!(s.v[658] != 0.0))) && (!(s.v[663] != 0.0))) {
            s.store_div_from_scalar(447, (0.666666666666667 * s.v[150]), 446);
        }

        if (((s.v[418] != 0.0) && (!(s.v[658] != 0.0))) && (!(s.v[663] != 0.0))) {
            s.store_square(448, 447);
        }

        if (((s.v[418] != 0.0) && (!(s.v[658] != 0.0))) && (!(s.v[663] != 0.0))) {
            s.store_sqrt_ad(449, A::div(A::square(s.ad_value(448)), A::offset(A::square(s.ad_value(448)), 1.0)));
        }

        if (((s.v[418] != 0.0) && (!(s.v[658] != 0.0))) && (!(s.v[663] != 0.0))) {
            s.store_sqrt_ad(450, A::abs(s.ad_value(449)));
        }

        if (((s.v[418] != 0.0) && (!(s.v[658] != 0.0))) && (!(s.v[663] != 0.0))) {
            s.store_mul(451, 449, 450);
        }

        s.v[664] = if (((-s.v[10]) * s.v[126]) == (-1.0)) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (!(s.v[658] != 0.0))) && (!(s.v[663] != 0.0))) && (s.v[664] != 0.0)) {
            s.store_div_from_scalar_ad(452, 1.0, A::offset(A::mul(s.ad_value(446), s.ad_value(451)), 1.0));
        }

        if ((((s.v[418] != 0.0) && (!(s.v[658] != 0.0))) && (!(s.v[663] != 0.0))) && (!(s.v[664] != 0.0))) {
            s.store_powf_ad(452, A::offset(A::mul(s.ad_value(446), s.ad_value(451)), 1.0), ((-s.v[10]) * s.v[126]));
        }

        if (((s.v[418] != 0.0) && (!(s.v[658] != 0.0))) && (!(s.v[663] != 0.0))) {
            s.store_div_ad(453, A::mul(s.ad_value(442), s.ad_value(452)), A::add(s.ad_value(442), s.ad_value(452)));
        }

        if (((s.v[418] != 0.0) && (!(s.v[658] != 0.0))) && (!(s.v[663] != 0.0))) {
            s.store_sqrt_ad(454, A::scale(A::div(s.ad_value(446), s.ad_value(450)), 0.375));
        }

        if (((s.v[418] != 0.0) && (!(s.v[658] != 0.0))) && (!(s.v[663] != 0.0))) {
            s.store_sub_ad_lhs(455, A::scale(A::mul(s.ad_value(447), s.ad_value(450)), 2.0), 449);
        }

        if (((s.v[418] != 0.0) && (!(s.v[658] != 0.0))) && (!(s.v[663] != 0.0))) {
            s.store_add_ad(456, A::sub(A::mul(A::scale(s.ad_value(447), s.v[150]), s.ad_value(450)), A::scale(s.ad_value(449), s.v[150])), A::scale(A::mul(s.ad_value(446), s.ad_value(451)), 0.5));
        }

        if (((s.v[418] != 0.0) && (!(s.v[658] != 0.0))) && (!(s.v[663] != 0.0))) {
            s.store_mul_ad_lhs(457, A::offset(s.ad_value(455), (-1.0)), 454);
        }

        if (((s.v[418] != 0.0) && (!(s.v[658] != 0.0))) && (!(s.v[663] != 0.0))) {
            s.store_square(419, 457);
        }

        s.v[665] = if (s.v[457] > 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (!(s.v[658] != 0.0))) && (!(s.v[663] != 0.0))) && (s.v[665] != 0.0)) {
            s.store_div_from_scalar_ad(420, 1.0, A::offset(A::scale(s.ad_value(457), s.v[86]), 1.0));
        }

        if ((((s.v[418] != 0.0) && (!(s.v[658] != 0.0))) && (!(s.v[663] != 0.0))) && (!(s.v[665] != 0.0))) {
            s.store_div_from_scalar_ad(420, 1.0, A::sub_from_scalar(1.0, A::scale(s.ad_value(457), s.v[86])));
        }

        s.v[666] = if (((-s.v[419]) + s.v[456]) > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (!(s.v[658] != 0.0))) && (!(s.v[663] != 0.0))) && (s.v[666] != 0.0)) {
            s.store_exp_ad(436, A::sub(s.ad_value(456), s.ad_value(419)));
        }

        if ((((s.v[418] != 0.0) && (!(s.v[658] != 0.0))) && (!(s.v[663] != 0.0))) && (!(s.v[666] != 0.0))) {
            s.store_div_from_scalar_ad(436, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(456), s.ad_value(419))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(456), s.ad_value(419))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(456), s.ad_value(419))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if (((s.v[418] != 0.0) && (!(s.v[658] != 0.0))) && (!(s.v[663] != 0.0))) {
            s.store_mul_ad_lhs(421, A::add(A::add(A::scale(s.ad_value(420), 0.29214664), A::scale(A::square(s.ad_value(420)), s.v[87])), A::scale(A::mul(A::square(s.ad_value(420)), s.ad_value(420)), s.v[88])), 436);
        }

        s.v[667] = if (s.v[457] > 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (!(s.v[658] != 0.0))) && (!(s.v[663] != 0.0))) && (s.v[667] != 0.0)) {
            s.copy_ad(458, 421);
        }

        s.v[668] = if (s.v[456] > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((s.v[418] != 0.0) && (!(s.v[658] != 0.0))) && (!(s.v[663] != 0.0))) && (!(s.v[667] != 0.0))) && (s.v[668] != 0.0)) {
            s.store_exp(436, 456);
        }

        if (((((s.v[418] != 0.0) && (!(s.v[658] != 0.0))) && (!(s.v[663] != 0.0))) && (!(s.v[667] != 0.0))) && (!(s.v[668] != 0.0))) {
            s.store_div_from_scalar_ad(436, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(456)), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(456)), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), s.ad_value(456)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if ((((s.v[418] != 0.0) && (!(s.v[658] != 0.0))) && (!(s.v[663] != 0.0))) && (!(s.v[667] != 0.0))) {
            s.store_sub_ad_lhs(458, A::scale(s.ad_value(436), 2.0), 421);
        }

        if (((s.v[418] != 0.0) && (!(s.v[658] != 0.0))) && (!(s.v[663] != 0.0))) {
            s.store_scale_ad(459, A::div(A::scale(s.ad_value(458), s.v[150]), s.ad_value(454)), (1.772453850905516 * 0.5));
        }

        if (((s.v[418] != 0.0) && (!(s.v[658] != 0.0))) && (!(s.v[663] != 0.0))) {
            s.store_scale_ad(445, A::mul(A::mul(s.ad_value(444), s.ad_value(459)), s.ad_value(453)), s.v[24]);
        }

        s.v[669] = if (s.v[30] == 0.0) { 1.0 } else { 0.0 };

        if (((s.v[418] != 0.0) && (!(s.v[658] != 0.0))) && (s.v[669] != 0.0)) {
            s.store_scalar(460, 0.0);
        }

        s.v[670] = if (s.v[10] == 0.5) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (!(s.v[658] != 0.0))) && (!(s.v[669] != 0.0))) && (s.v[670] != 0.0)) {
            s.store_sqrt_ad(436, A::scale(A::sub_from_scalar(s.v[7], s.ad_value(434)), s.v[144]));
        }

        if ((((s.v[418] != 0.0) && (!(s.v[658] != 0.0))) && (!(s.v[669] != 0.0))) && (!(s.v[670] != 0.0))) {
            s.store_powf_ad(436, A::scale(A::sub_from_scalar(s.v[7], s.ad_value(434)), s.v[144]), s.v[10]);
        }

        if (((s.v[418] != 0.0) && (!(s.v[658] != 0.0))) && (!(s.v[669] != 0.0))) {
            s.store_scale_ad(461, A::div(A::scale(A::sub_from_scalar(s.v[7], s.ad_value(434)), s.v[141]), s.ad_value(436)), s.v[126]);
        }

        s.v[671] = if (((((-s.v[156]) / s.v[461])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (!(s.v[658] != 0.0))) && (!(s.v[669] != 0.0))) && (s.v[671] != 0.0)) {
            s.store_exp_ad(436, A::div(A::neg(s.ad_value(156)), s.ad_value(461)));
        }

        s.v[672] = if (((-s.v[156]) / s.v[461]) < (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((s.v[418] != 0.0) && (!(s.v[658] != 0.0))) && (!(s.v[669] != 0.0))) && (!(s.v[671] != 0.0))) && (s.v[672] != 0.0)) {
            let assign17570_ad_e24716: A = A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(156)), s.ad_value(461))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(156)), s.ad_value(461))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(156)), s.ad_value(461))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
            s.store_div_from_scalar_ad(436, 1e-100, assign17570_ad_e24716);
        }

        if (((((s.v[418] != 0.0) && (!(s.v[658] != 0.0))) && (!(s.v[669] != 0.0))) && (!(s.v[671] != 0.0))) && (!(s.v[672] != 0.0))) {
            let assign17580_ad_e24764: A = A::scale(A::offset(A::mul(A::offset(A::div(A::neg(s.ad_value(156)), s.ad_value(461)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::div(A::neg(s.ad_value(156)), s.ad_value(461)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::div(A::neg(s.ad_value(156)), s.ad_value(461)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
            s.store_ad(436, &assign17580_ad_e24764);
        }

        if (((s.v[418] != 0.0) && (!(s.v[658] != 0.0))) && (!(s.v[669] != 0.0))) {
            s.store_scale_ad(460, A::mul(A::mul(A::mul(s.ad_value(192), s.ad_value(461)), s.ad_value(461)), s.ad_value(436)), s.v[30]);
        }

        s.v[673] = if ((s.v[39] > 1000000.0) || (p.p80 == 0.0)) { 1.0 } else { 0.0 };

        if (((s.v[418] != 0.0) && (!(s.v[658] != 0.0))) && (s.v[673] != 0.0)) {
            s.store_scalar(462, 1.0);
        }

        s.v[674] = if (s.v[435] > ((-s.v[158]) * s.v[39])) { 1.0 } else { 0.0 };

        s.v[675] = if (s.v[42] == 4.0) { 1.0 } else { 0.0 };

        if (((((s.v[418] != 0.0) && (!(s.v[658] != 0.0))) && (!(s.v[673] != 0.0))) && (s.v[674] != 0.0)) && (s.v[675] != 0.0)) {
            s.store_mul_ad(436, A::mul(A::mul(A::abs(A::mul(s.ad_value(435), s.ad_value(163))), A::abs(A::mul(s.ad_value(435), s.ad_value(163)))), A::abs(A::mul(s.ad_value(435), s.ad_value(163)))), A::abs(A::mul(s.ad_value(435), s.ad_value(163))));
        }

        if (((((s.v[418] != 0.0) && (!(s.v[658] != 0.0))) && (!(s.v[673] != 0.0))) && (s.v[674] != 0.0)) && (!(s.v[675] != 0.0))) {
            s.store_powf_ad(436, A::abs(A::mul(s.ad_value(435), s.ad_value(163))), s.v[42]);
        }

        if ((((s.v[418] != 0.0) && (!(s.v[658] != 0.0))) && (!(s.v[673] != 0.0))) && (s.v[674] != 0.0)) {
            s.store_div_from_scalar_ad(462, 1.0, A::sub_from_scalar(1.0, s.ad_value(436)));
        }

        if ((((s.v[418] != 0.0) && (!(s.v[658] != 0.0))) && (!(s.v[673] != 0.0))) && (!(s.v[674] != 0.0))) {
            s.store_offset_ad(462, A::mul(A::add(s.ad_value(435), A::scale(s.ad_value(39), s.v[158])), s.ad_value(166)), s.v[160]);
        }

        if ((s.v[418] != 0.0) && (!(s.v[658] != 0.0))) {
            s.store_mul_ad_lhs(270, A::add(A::add(A::add(s.ad_value(437), s.ad_value(438)), s.ad_value(445)), s.ad_value(460)), 462);
        }

        if ((s.v[418] != 0.0) && (!(s.v[658] != 0.0))) {
            s.store_mul_ad_lhs(292, A::add(A::add(s.ad_value(438), s.ad_value(445)), s.ad_value(460)), 462);
        }

        s.v[676] = if (s.v[258] == 0.0) { 1.0 } else { 0.0 };

        if ((s.v[418] != 0.0) && (s.v[676] != 0.0)) {
            s.store_scalar(272, 0.0);
        }

        if ((s.v[418] != 0.0) && (s.v[676] != 0.0)) {
            s.store_scalar(293, 0.0);
        }

        if ((s.v[418] != 0.0) && (s.v[676] != 0.0)) {
            s.store_scalar(273, 0.0);
        }

        s.v[677] = if (s.v[124] == 0.5) { 1.0 } else { 0.0 };

        if (((s.v[418] != 0.0) && (!(s.v[676] != 0.0))) && (s.v[677] != 0.0)) {
            s.store_sqrt_ad(436, A::sub_from_scalar(1.0, A::mul(s.ad_value(428), s.ad_value(121))));
        }

        if (((s.v[418] != 0.0) && (!(s.v[676] != 0.0))) && (!(s.v[677] != 0.0))) {
            s.store_powf_ad(436, A::sub_from_scalar(1.0, A::mul(s.ad_value(428), s.ad_value(121))), s.v[124]);
        }

        if ((s.v[418] != 0.0) && (!(s.v[676] != 0.0))) {
            s.store_add_ad(273, A::mul(s.ad_value(133), A::sub_from_scalar(1.0, s.ad_value(436))), A::mul(s.ad_value(136), A::sub(s.ad_value(192), s.ad_value(428))));
        }

        if ((s.v[418] != 0.0) && (!(s.v[676] != 0.0))) {
            s.store_mul(437, 103, 372);
        }

        s.v[678] = if ((s.v[22] == 0.0) && (s.v[25] == 0.0)) { 1.0 } else { 0.0 };

        if (((s.v[418] != 0.0) && (!(s.v[676] != 0.0))) && (s.v[678] != 0.0)) {
            s.store_scalar(439, 0.0);
        }

        if (((s.v[418] != 0.0) && (!(s.v[676] != 0.0))) && (s.v[678] != 0.0)) {
            s.store_scalar(442, 0.0);
        }

        if (((s.v[418] != 0.0) && (!(s.v[676] != 0.0))) && (s.v[678] != 0.0)) {
            s.store_scalar(443, 0.0);
        }

        if (((s.v[418] != 0.0) && (!(s.v[676] != 0.0))) && (s.v[678] != 0.0)) {
            s.store_scalar(444, 0.0);
        }

        if (((s.v[418] != 0.0) && (!(s.v[676] != 0.0))) && (s.v[678] != 0.0)) {
            s.store_scalar(438, 0.0);
        }

        if (((s.v[418] != 0.0) && (!(s.v[676] != 0.0))) && (!(s.v[678] != 0.0))) {
            s.store_sub(439, 109, 433);
        }

        if (((s.v[418] != 0.0) && (!(s.v[676] != 0.0))) && (!(s.v[678] != 0.0))) {
            s.store_sub_from_scalar_ad(440, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(431), s.ad_value(439)))));
        }

        s.v[679] = if (s.v[11] == 0.5) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (!(s.v[676] != 0.0))) && (!(s.v[678] != 0.0))) && (s.v[679] != 0.0)) {
            s.store_scalar(441, 0.0);
        }

        if ((((s.v[418] != 0.0) && (!(s.v[676] != 0.0))) && (!(s.v[678] != 0.0))) && (!(s.v[679] != 0.0))) {
            s.store_scale_ad(441, A::add(A::div(A::mul(A::square(s.ad_value(440)), A::ln(s.ad_value(440))), A::sub_from_scalar(1.0, s.ad_value(440))), s.ad_value(440)), (1.0 - (2.0 * s.v[11])));
        }

        if (((s.v[418] != 0.0) && (!(s.v[676] != 0.0))) && (!(s.v[678] != 0.0))) {
            s.store_add(442, 440, 441);
        }

        s.v[680] = if (s.v[11] == 0.5) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (!(s.v[676] != 0.0))) && (!(s.v[678] != 0.0))) && (s.v[680] != 0.0)) {
            s.store_sqrt_ad(436, A::scale(s.ad_value(439), s.v[145]));
        }

        if ((((s.v[418] != 0.0) && (!(s.v[676] != 0.0))) && (!(s.v[678] != 0.0))) && (!(s.v[680] != 0.0))) {
            s.store_powf_ad(436, A::scale(s.ad_value(439), s.v[145]), s.v[11]);
        }

        if (((s.v[418] != 0.0) && (!(s.v[676] != 0.0))) && (!(s.v[678] != 0.0))) {
            s.store_scale(443, 436, s.v[139]);
        }

        if (((s.v[418] != 0.0) && (!(s.v[676] != 0.0))) && (!(s.v[678] != 0.0))) {
            s.store_mul_ad_rhs(444, 100, A::mul(A::offset(s.ad_value(430), (-1.0)), s.ad_value(443)));
        }

        if (((s.v[418] != 0.0) && (!(s.v[676] != 0.0))) && (!(s.v[678] != 0.0))) {
            s.store_scaled_mul(438, 444, 442, s.v[22]);
        }

        s.v[681] = if (s.v[25] == 0.0) { 1.0 } else { 0.0 };

        if (((s.v[418] != 0.0) && (!(s.v[676] != 0.0))) && (s.v[681] != 0.0)) {
            s.store_scalar(445, 0.0);
        }

        if (((s.v[418] != 0.0) && (!(s.v[676] != 0.0))) && (!(s.v[681] != 0.0))) {
            s.store_scale_ad(446, A::div(A::scale(s.ad_value(443), s.v[124]), s.ad_value(439)), s.v[154]);
        }

        if (((s.v[418] != 0.0) && (!(s.v[676] != 0.0))) && (!(s.v[681] != 0.0))) {
            s.store_div_from_scalar(447, (0.666666666666667 * s.v[151]), 446);
        }

        if (((s.v[418] != 0.0) && (!(s.v[676] != 0.0))) && (!(s.v[681] != 0.0))) {
            s.store_square(448, 447);
        }

        if (((s.v[418] != 0.0) && (!(s.v[676] != 0.0))) && (!(s.v[681] != 0.0))) {
            s.store_sqrt_ad(449, A::div(A::square(s.ad_value(448)), A::offset(A::square(s.ad_value(448)), 1.0)));
        }

        if (((s.v[418] != 0.0) && (!(s.v[676] != 0.0))) && (!(s.v[681] != 0.0))) {
            s.store_sqrt_ad(450, A::abs(s.ad_value(449)));
        }

        if (((s.v[418] != 0.0) && (!(s.v[676] != 0.0))) && (!(s.v[681] != 0.0))) {
            s.store_mul(451, 449, 450);
        }

        s.v[682] = if (((-s.v[11]) * s.v[127]) == (-1.0)) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (!(s.v[676] != 0.0))) && (!(s.v[681] != 0.0))) && (s.v[682] != 0.0)) {
            s.store_div_from_scalar_ad(452, 1.0, A::offset(A::mul(s.ad_value(446), s.ad_value(451)), 1.0));
        }

        if ((((s.v[418] != 0.0) && (!(s.v[676] != 0.0))) && (!(s.v[681] != 0.0))) && (!(s.v[682] != 0.0))) {
            s.store_powf_ad(452, A::offset(A::mul(s.ad_value(446), s.ad_value(451)), 1.0), ((-s.v[11]) * s.v[127]));
        }

        if (((s.v[418] != 0.0) && (!(s.v[676] != 0.0))) && (!(s.v[681] != 0.0))) {
            s.store_div_ad(453, A::mul(s.ad_value(442), s.ad_value(452)), A::add(s.ad_value(442), s.ad_value(452)));
        }

        if (((s.v[418] != 0.0) && (!(s.v[676] != 0.0))) && (!(s.v[681] != 0.0))) {
            s.store_sqrt_ad(454, A::scale(A::div(s.ad_value(446), s.ad_value(450)), 0.375));
        }

        if (((s.v[418] != 0.0) && (!(s.v[676] != 0.0))) && (!(s.v[681] != 0.0))) {
            s.store_sub_ad_lhs(455, A::scale(A::mul(s.ad_value(447), s.ad_value(450)), 2.0), 449);
        }

        if (((s.v[418] != 0.0) && (!(s.v[676] != 0.0))) && (!(s.v[681] != 0.0))) {
            s.store_add_ad(456, A::sub(A::mul(A::scale(s.ad_value(447), s.v[151]), s.ad_value(450)), A::scale(s.ad_value(449), s.v[151])), A::scale(A::mul(s.ad_value(446), s.ad_value(451)), 0.5));
        }

        if (((s.v[418] != 0.0) && (!(s.v[676] != 0.0))) && (!(s.v[681] != 0.0))) {
            s.store_mul_ad_lhs(457, A::offset(s.ad_value(455), (-1.0)), 454);
        }

        if (((s.v[418] != 0.0) && (!(s.v[676] != 0.0))) && (!(s.v[681] != 0.0))) {
            s.store_square(419, 457);
        }

        s.v[683] = if (s.v[457] > 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (!(s.v[676] != 0.0))) && (!(s.v[681] != 0.0))) && (s.v[683] != 0.0)) {
            s.store_div_from_scalar_ad(420, 1.0, A::offset(A::scale(s.ad_value(457), s.v[86]), 1.0));
        }

        if ((((s.v[418] != 0.0) && (!(s.v[676] != 0.0))) && (!(s.v[681] != 0.0))) && (!(s.v[683] != 0.0))) {
            s.store_div_from_scalar_ad(420, 1.0, A::sub_from_scalar(1.0, A::scale(s.ad_value(457), s.v[86])));
        }

        s.v[684] = if (((-s.v[419]) + s.v[456]) > (-230.25850929940458)) { 1.0 } else { 0.0 };

    }

    pub(super) fn stamp_reactive_block_13(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if ((((s.v[418] != 0.0) && (!(s.v[676] != 0.0))) && (!(s.v[681] != 0.0))) && (s.v[684] != 0.0)) {
            s.store_exp_ad(436, A::sub(s.ad_value(456), s.ad_value(419)));
        }

        if ((((s.v[418] != 0.0) && (!(s.v[676] != 0.0))) && (!(s.v[681] != 0.0))) && (!(s.v[684] != 0.0))) {
            s.store_div_from_scalar_ad(436, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(456), s.ad_value(419))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(456), s.ad_value(419))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(456), s.ad_value(419))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if (((s.v[418] != 0.0) && (!(s.v[676] != 0.0))) && (!(s.v[681] != 0.0))) {
            s.store_mul_ad_lhs(421, A::add(A::add(A::scale(s.ad_value(420), 0.29214664), A::scale(A::square(s.ad_value(420)), s.v[87])), A::scale(A::mul(A::square(s.ad_value(420)), s.ad_value(420)), s.v[88])), 436);
        }

        s.v[685] = if (s.v[457] > 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (!(s.v[676] != 0.0))) && (!(s.v[681] != 0.0))) && (s.v[685] != 0.0)) {
            s.copy_ad(458, 421);
        }

        s.v[686] = if (s.v[456] > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((s.v[418] != 0.0) && (!(s.v[676] != 0.0))) && (!(s.v[681] != 0.0))) && (!(s.v[685] != 0.0))) && (s.v[686] != 0.0)) {
            s.store_exp(436, 456);
        }

        if (((((s.v[418] != 0.0) && (!(s.v[676] != 0.0))) && (!(s.v[681] != 0.0))) && (!(s.v[685] != 0.0))) && (!(s.v[686] != 0.0))) {
            s.store_div_from_scalar_ad(436, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(456)), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(456)), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), s.ad_value(456)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if ((((s.v[418] != 0.0) && (!(s.v[676] != 0.0))) && (!(s.v[681] != 0.0))) && (!(s.v[685] != 0.0))) {
            s.store_sub_ad_lhs(458, A::scale(s.ad_value(436), 2.0), 421);
        }

        if (((s.v[418] != 0.0) && (!(s.v[676] != 0.0))) && (!(s.v[681] != 0.0))) {
            s.store_scale_ad(459, A::div(A::scale(s.ad_value(458), s.v[151]), s.ad_value(454)), (1.772453850905516 * 0.5));
        }

        if (((s.v[418] != 0.0) && (!(s.v[676] != 0.0))) && (!(s.v[681] != 0.0))) {
            s.store_scale_ad(445, A::mul(A::mul(s.ad_value(444), s.ad_value(459)), s.ad_value(453)), s.v[25]);
        }

        s.v[687] = if (s.v[31] == 0.0) { 1.0 } else { 0.0 };

        if (((s.v[418] != 0.0) && (!(s.v[676] != 0.0))) && (s.v[687] != 0.0)) {
            s.store_scalar(460, 0.0);
        }

        s.v[688] = if (s.v[11] == 0.5) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (!(s.v[676] != 0.0))) && (!(s.v[687] != 0.0))) && (s.v[688] != 0.0)) {
            s.store_sqrt_ad(436, A::scale(A::sub_from_scalar(s.v[8], s.ad_value(434)), s.v[145]));
        }

        if ((((s.v[418] != 0.0) && (!(s.v[676] != 0.0))) && (!(s.v[687] != 0.0))) && (!(s.v[688] != 0.0))) {
            s.store_powf_ad(436, A::scale(A::sub_from_scalar(s.v[8], s.ad_value(434)), s.v[145]), s.v[11]);
        }

        if (((s.v[418] != 0.0) && (!(s.v[676] != 0.0))) && (!(s.v[687] != 0.0))) {
            s.store_scale_ad(461, A::div(A::scale(A::sub_from_scalar(s.v[8], s.ad_value(434)), s.v[142]), s.ad_value(436)), s.v[127]);
        }

        s.v[689] = if (((((-s.v[157]) / s.v[461])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (!(s.v[676] != 0.0))) && (!(s.v[687] != 0.0))) && (s.v[689] != 0.0)) {
            s.store_exp_ad(436, A::div(A::neg(s.ad_value(157)), s.ad_value(461)));
        }

        s.v[690] = if (((-s.v[157]) / s.v[461]) < (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((s.v[418] != 0.0) && (!(s.v[676] != 0.0))) && (!(s.v[687] != 0.0))) && (!(s.v[689] != 0.0))) && (s.v[690] != 0.0)) {
            let assign18380_ad_e25872: A = A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(157)), s.ad_value(461))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(157)), s.ad_value(461))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(157)), s.ad_value(461))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
            s.store_div_from_scalar_ad(436, 1e-100, assign18380_ad_e25872);
        }

        if (((((s.v[418] != 0.0) && (!(s.v[676] != 0.0))) && (!(s.v[687] != 0.0))) && (!(s.v[689] != 0.0))) && (!(s.v[690] != 0.0))) {
            let assign18390_ad_e25920: A = A::scale(A::offset(A::mul(A::offset(A::div(A::neg(s.ad_value(157)), s.ad_value(461)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::div(A::neg(s.ad_value(157)), s.ad_value(461)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::div(A::neg(s.ad_value(157)), s.ad_value(461)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
            s.store_ad(436, &assign18390_ad_e25920);
        }

        if (((s.v[418] != 0.0) && (!(s.v[676] != 0.0))) && (!(s.v[687] != 0.0))) {
            s.store_scale_ad(460, A::mul(A::mul(A::mul(s.ad_value(192), s.ad_value(461)), s.ad_value(461)), s.ad_value(436)), s.v[31]);
        }

        s.v[691] = if ((s.v[40] > 1000000.0) || (p.p80 == 0.0)) { 1.0 } else { 0.0 };

        if (((s.v[418] != 0.0) && (!(s.v[676] != 0.0))) && (s.v[691] != 0.0)) {
            s.store_scalar(462, 1.0);
        }

        s.v[692] = if (s.v[435] > ((-s.v[158]) * s.v[40])) { 1.0 } else { 0.0 };

        s.v[693] = if (s.v[43] == 4.0) { 1.0 } else { 0.0 };

        if (((((s.v[418] != 0.0) && (!(s.v[676] != 0.0))) && (!(s.v[691] != 0.0))) && (s.v[692] != 0.0)) && (s.v[693] != 0.0)) {
            s.store_mul_ad(436, A::mul(A::mul(A::abs(A::mul(s.ad_value(435), s.ad_value(164))), A::abs(A::mul(s.ad_value(435), s.ad_value(164)))), A::abs(A::mul(s.ad_value(435), s.ad_value(164)))), A::abs(A::mul(s.ad_value(435), s.ad_value(164))));
        }

        if (((((s.v[418] != 0.0) && (!(s.v[676] != 0.0))) && (!(s.v[691] != 0.0))) && (s.v[692] != 0.0)) && (!(s.v[693] != 0.0))) {
            s.store_powf_ad(436, A::abs(A::mul(s.ad_value(435), s.ad_value(164))), s.v[43]);
        }

        if ((((s.v[418] != 0.0) && (!(s.v[676] != 0.0))) && (!(s.v[691] != 0.0))) && (s.v[692] != 0.0)) {
            s.store_div_from_scalar_ad(462, 1.0, A::sub_from_scalar(1.0, s.ad_value(436)));
        }

        if ((((s.v[418] != 0.0) && (!(s.v[676] != 0.0))) && (!(s.v[691] != 0.0))) && (!(s.v[692] != 0.0))) {
            s.store_offset_ad(462, A::mul(A::add(s.ad_value(435), A::scale(s.ad_value(40), s.v[158])), s.ad_value(167)), s.v[161]);
        }

        if ((s.v[418] != 0.0) && (!(s.v[676] != 0.0))) {
            s.store_mul_ad_lhs(272, A::add(A::add(A::add(s.ad_value(437), s.ad_value(438)), s.ad_value(445)), s.ad_value(460)), 462);
        }

        if ((s.v[418] != 0.0) && (!(s.v[676] != 0.0))) {
            s.store_mul_ad_lhs(293, A::add(A::add(s.ad_value(438), s.ad_value(445)), s.ad_value(460)), 462);
        }

        if (s.v[418] != 0.0) {
            s.store_add_ad(182, A::add(A::scale(s.ad_value(268), s.v[256]), A::scale(s.ad_value(270), s.v[257])), A::scale(s.ad_value(272), s.v[258]));
        }

        s.v[694] = if !(((s.v[256] == 0.0) && (s.v[257] == 0.0)) && (s.v[258] == 0.0)) { 1.0 } else { 0.0 };

        if ((s.v[418] != 0.0) && (s.v[694] != 0.0)) {
            s.store_mul_ad_lhs(422, A::scale(s.ad_value(265), 4.0), 265);
        }

        if ((s.v[418] != 0.0) && (s.v[694] != 0.0)) {
            s.store_div(423, 265, 266);
        }

        if ((s.v[418] != 0.0) && (s.v[694] != 0.0)) {
            s.store_add_ad_rhs(424, 193, A::mul(s.ad_value(265), s.ad_value(423)));
        }

        if ((s.v[418] != 0.0) && (s.v[694] != 0.0)) {
            s.store_add(425, 266, 424);
        }

        if ((s.v[418] != 0.0) && (s.v[694] != 0.0)) {
            s.store_sub(426, 266, 424);
        }

        if ((s.v[418] != 0.0) && (s.v[694] != 0.0)) {
            s.store_sqrt_ad(427, A::add(A::square(s.ad_value(426)), s.ad_value(422)));
        }

        if ((s.v[418] != 0.0) && (s.v[694] != 0.0)) {
            s.store_scale_ad(428, A::div(A::mul(s.ad_value(193), s.ad_value(266)), A::add(s.ad_value(425), s.ad_value(427))), 2.0);
        }

        s.v[695] = if (s.v[193] < s.v[262]) { 1.0 } else { 0.0 };

        s.v[696] = if ((((0.5 * (s.v[193] * s.v[85]))) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (s.v[695] != 0.0)) && (s.v[696] != 0.0)) {
            s.store_exp_ad(430, A::scale(s.ad_value(193), (s.v[85] * 0.5)));
        }

        s.v[697] = if ((0.5 * (s.v[193] * s.v[85])) < (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (s.v[695] != 0.0)) && (!(s.v[696] != 0.0))) && (s.v[697] != 0.0)) {
            let assign18640_ad_e26259: A = A::div_from_scalar(1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(s.ad_value(193), (s.v[85] * 0.5))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(s.ad_value(193), (s.v[85] * 0.5))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::scale(s.ad_value(193), (s.v[85] * 0.5))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
            s.store_ad(430, &assign18640_ad_e26259);
        }

        if (((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (s.v[695] != 0.0)) && (!(s.v[696] != 0.0))) && (!(s.v[697] != 0.0))) {
            s.store_scale_ad(430, A::offset(A::mul(A::offset(A::scale(s.ad_value(193), (s.v[85] * 0.5)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::scale(s.ad_value(193), (s.v[85] * 0.5)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::scale(s.ad_value(193), (s.v[85] * 0.5)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
        }

        if (((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (s.v[695] != 0.0)) {
            s.store_scale_ad(363, A::square(s.ad_value(318)), 1.0 / (s.v[308]));
        }

        if (((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (s.v[695] != 0.0)) {
            s.store_scale_ad(362, A::ln(A::div_from_scalar(s.v[308], s.ad_value(363))), (s.v[62] / s.v[85]));
        }

        s.v[698] = if (s.v[62] < p.p85) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (s.v[695] != 0.0)) && (s.v[698] != 0.0)) {
            s.store_offset_ad(360, A::scale(A::sub(s.ad_value(193), s.ad_value(362)), p.p86), s.v[62]);
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (s.v[695] != 0.0)) && (s.v[698] != 0.0)) {
            s.store_sub_from_scalar_ad(350, s.v[62], A::scale(s.ad_value(362), p.p86));
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (s.v[695] != 0.0)) && (s.v[698] != 0.0)) {
            s.store_offset_ad(314, A::sub_from_scalar(p.p85, s.ad_value(360)), (-0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (s.v[695] != 0.0)) && (s.v[698] != 0.0)) {
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (s.v[695] != 0.0)) && (s.v[698] != 0.0)) {
            s.store_ad(315, &{
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (s.v[695] != 0.0)) && (s.v[698] != 0.0)) {
            s.store_sqrt_ad(315, A::add(A::square(s.ad_value(314)), s.ad_value(315)));
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (s.v[695] != 0.0)) && (s.v[698] != 0.0)) {
            s.store_sub_from_scalar_ad(361, p.p85, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5));
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (s.v[695] != 0.0)) && (s.v[698] != 0.0)) {
            s.store_offset(314, 361, (((-s.v[62])) + ((-0.01))));
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (s.v[695] != 0.0)) && (s.v[698] != 0.0)) {
            s.store_scalar(315, ((4.0 * s.v[62]) * 0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (s.v[695] != 0.0)) && (s.v[698] != 0.0)) {
            s.store_ad(315, &{
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (s.v[695] != 0.0)) && (s.v[698] != 0.0)) {
            s.store_sqrt_ad(315, A::add(A::square(s.ad_value(314)), s.ad_value(315)));
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (s.v[695] != 0.0)) && (s.v[698] != 0.0)) {
            s.store_offset_ad(359, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5), s.v[62]);
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (s.v[695] != 0.0)) && (s.v[698] != 0.0)) {
            s.store_offset_ad(314, A::sub_from_scalar(p.p85, s.ad_value(350)), (-0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (s.v[695] != 0.0)) && (s.v[698] != 0.0)) {
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (s.v[695] != 0.0)) && (s.v[698] != 0.0)) {
            s.store_ad(315, &{
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (s.v[695] != 0.0)) && (s.v[698] != 0.0)) {
            s.store_sqrt_ad(315, A::add(A::square(s.ad_value(314)), s.ad_value(315)));
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (s.v[695] != 0.0)) && (s.v[698] != 0.0)) {
            s.store_sub_from_scalar_ad(350, p.p85, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5));
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (s.v[695] != 0.0)) && (s.v[698] != 0.0)) {
            s.store_offset(314, 350, (((-s.v[62])) + ((-0.01))));
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (s.v[695] != 0.0)) && (s.v[698] != 0.0)) {
            s.store_scalar(315, ((4.0 * s.v[62]) * 0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (s.v[695] != 0.0)) && (s.v[698] != 0.0)) {
            s.store_ad(315, &{
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (s.v[695] != 0.0)) && (s.v[698] != 0.0)) {
            s.store_sqrt_ad(315, A::add(A::square(s.ad_value(314)), s.ad_value(315)));
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (s.v[695] != 0.0)) && (s.v[698] != 0.0)) {
            s.store_offset_ad(350, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5), s.v[62]);
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (s.v[695] != 0.0)) && (!(s.v[698] != 0.0))) {
            s.store_scalar(350, s.v[62]);
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (s.v[695] != 0.0)) && (!(s.v[698] != 0.0))) {
            s.store_scalar(359, s.v[62]);
        }

        s.v[699] = if ((((s.v[85] * ((s.v[193] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85))))) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (s.v[695] != 0.0)) && (s.v[699] != 0.0)) {
            s.store_exp_ad(370, A::scale(A::add(A::div(s.ad_value(193), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]));
        }

        s.v[700] = if ((s.v[85] * ((s.v[193] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85)))) < (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (s.v[695] != 0.0)) && (!(s.v[699] != 0.0))) && (s.v[700] != 0.0)) {
            let assign18960_ad_e26825: A = A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(A::add(A::div(s.ad_value(193), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85])), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::scale(A::add(A::div(s.ad_value(193), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85])), 0.3333333333333333), 1.0));
            s.store_div_from_scalar_ad(370, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(A::add(A::div(s.ad_value(193), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85])), A::offset(A::scale(assign18960_ad_e26825, 0.5), 1.0)), 1.0));
        }

        if (((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (s.v[695] != 0.0)) && (!(s.v[699] != 0.0))) && (!(s.v[700] != 0.0))) {
            let assign18970_ad_e26903: A = A::mul(A::offset(A::scale(A::add(A::div(s.ad_value(193), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]), (-230.25850929940458)), A::offset(A::scale(A::offset(A::scale(A::add(A::div(s.ad_value(193), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]), (-230.25850929940458)), 0.3333333333333333), 1.0));
            s.store_scale_ad(370, A::offset(A::mul(A::offset(A::scale(A::add(A::div(s.ad_value(193), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]), (-230.25850929940458)), A::offset(A::scale(assign18970_ad_e26903, 0.5), 1.0)), 1.0), 1e100);
        }

        if (((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (s.v[695] != 0.0)) {
            s.store_scale_ad(363, A::square(s.ad_value(318)), 1.0 / (s.v[310]));
        }

        if (((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (s.v[695] != 0.0)) {
            s.store_scale_ad(362, A::ln(A::div_from_scalar(s.v[310], s.ad_value(363))), (s.v[64] / s.v[85]));
        }

        s.v[701] = if (s.v[64] < p.p85) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (s.v[695] != 0.0)) && (s.v[701] != 0.0)) {
            s.store_offset_ad(360, A::scale(A::sub(s.ad_value(193), s.ad_value(362)), p.p86), s.v[64]);
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (s.v[695] != 0.0)) && (s.v[701] != 0.0)) {
            s.store_sub_from_scalar_ad(350, s.v[64], A::scale(s.ad_value(362), p.p86));
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (s.v[695] != 0.0)) && (s.v[701] != 0.0)) {
            s.store_offset_ad(314, A::sub_from_scalar(p.p85, s.ad_value(360)), (-0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (s.v[695] != 0.0)) && (s.v[701] != 0.0)) {
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (s.v[695] != 0.0)) && (s.v[701] != 0.0)) {
            s.store_ad(315, &{
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (s.v[695] != 0.0)) && (s.v[701] != 0.0)) {
            s.store_sqrt_ad(315, A::add(A::square(s.ad_value(314)), s.ad_value(315)));
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (s.v[695] != 0.0)) && (s.v[701] != 0.0)) {
            s.store_sub_from_scalar_ad(361, p.p85, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5));
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (s.v[695] != 0.0)) && (s.v[701] != 0.0)) {
            s.store_offset(314, 361, (((-s.v[64])) + ((-0.01))));
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (s.v[695] != 0.0)) && (s.v[701] != 0.0)) {
            s.store_scalar(315, ((4.0 * s.v[64]) * 0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (s.v[695] != 0.0)) && (s.v[701] != 0.0)) {
            s.store_ad(315, &{
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (s.v[695] != 0.0)) && (s.v[701] != 0.0)) {
            s.store_sqrt_ad(315, A::add(A::square(s.ad_value(314)), s.ad_value(315)));
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (s.v[695] != 0.0)) && (s.v[701] != 0.0)) {
            s.store_offset_ad(359, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5), s.v[64]);
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (s.v[695] != 0.0)) && (s.v[701] != 0.0)) {
            s.store_offset_ad(314, A::sub_from_scalar(p.p85, s.ad_value(350)), (-0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (s.v[695] != 0.0)) && (s.v[701] != 0.0)) {
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (s.v[695] != 0.0)) && (s.v[701] != 0.0)) {
            s.store_ad(315, &{
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (s.v[695] != 0.0)) && (s.v[701] != 0.0)) {
            s.store_sqrt_ad(315, A::add(A::square(s.ad_value(314)), s.ad_value(315)));
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (s.v[695] != 0.0)) && (s.v[701] != 0.0)) {
            s.store_sub_from_scalar_ad(350, p.p85, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5));
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (s.v[695] != 0.0)) && (s.v[701] != 0.0)) {
            s.store_offset(314, 350, (((-s.v[64])) + ((-0.01))));
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (s.v[695] != 0.0)) && (s.v[701] != 0.0)) {
            s.store_scalar(315, ((4.0 * s.v[64]) * 0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (s.v[695] != 0.0)) && (s.v[701] != 0.0)) {
            s.store_ad(315, &{
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (s.v[695] != 0.0)) && (s.v[701] != 0.0)) {
            s.store_sqrt_ad(315, A::add(A::square(s.ad_value(314)), s.ad_value(315)));
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (s.v[695] != 0.0)) && (s.v[701] != 0.0)) {
            s.store_offset_ad(350, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5), s.v[64]);
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (s.v[695] != 0.0)) && (!(s.v[701] != 0.0))) {
            s.store_scalar(350, s.v[64]);
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (s.v[695] != 0.0)) && (!(s.v[701] != 0.0))) {
            s.store_scalar(359, s.v[64]);
        }

        s.v[702] = if ((((s.v[85] * ((s.v[193] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85))))) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (s.v[695] != 0.0)) && (s.v[702] != 0.0)) {
            s.store_exp_ad(371, A::scale(A::add(A::div(s.ad_value(193), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]));
        }

        s.v[703] = if ((s.v[85] * ((s.v[193] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85)))) < (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (s.v[695] != 0.0)) && (!(s.v[702] != 0.0))) && (s.v[703] != 0.0)) {
            let assign19280_ad_e27426: A = A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(A::add(A::div(s.ad_value(193), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85])), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::scale(A::add(A::div(s.ad_value(193), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85])), 0.3333333333333333), 1.0));
            s.store_div_from_scalar_ad(371, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(A::add(A::div(s.ad_value(193), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85])), A::offset(A::scale(assign19280_ad_e27426, 0.5), 1.0)), 1.0));
        }

        if (((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (s.v[695] != 0.0)) && (!(s.v[702] != 0.0))) && (!(s.v[703] != 0.0))) {
            let assign19290_ad_e27504: A = A::mul(A::offset(A::scale(A::add(A::div(s.ad_value(193), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]), (-230.25850929940458)), A::offset(A::scale(A::offset(A::scale(A::add(A::div(s.ad_value(193), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]), (-230.25850929940458)), 0.3333333333333333), 1.0));
            s.store_scale_ad(371, A::offset(A::mul(A::offset(A::scale(A::add(A::div(s.ad_value(193), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]), (-230.25850929940458)), A::offset(A::scale(assign19290_ad_e27504, 0.5), 1.0)), 1.0), 1e100);
        }

        if (((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (s.v[695] != 0.0)) {
            s.store_scale_ad(363, A::square(s.ad_value(318)), 1.0 / (s.v[309]));
        }

        if (((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (s.v[695] != 0.0)) {
            s.store_scale_ad(362, A::ln(A::div_from_scalar(s.v[309], s.ad_value(363))), (s.v[63] / s.v[85]));
        }

        s.v[704] = if (s.v[63] < p.p85) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (s.v[695] != 0.0)) && (s.v[704] != 0.0)) {
            s.store_offset_ad(360, A::scale(A::sub(s.ad_value(193), s.ad_value(362)), p.p86), s.v[63]);
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (s.v[695] != 0.0)) && (s.v[704] != 0.0)) {
            s.store_sub_from_scalar_ad(350, s.v[63], A::scale(s.ad_value(362), p.p86));
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (s.v[695] != 0.0)) && (s.v[704] != 0.0)) {
            s.store_offset_ad(314, A::sub_from_scalar(p.p85, s.ad_value(360)), (-0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (s.v[695] != 0.0)) && (s.v[704] != 0.0)) {
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (s.v[695] != 0.0)) && (s.v[704] != 0.0)) {
            s.store_ad(315, &{
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (s.v[695] != 0.0)) && (s.v[704] != 0.0)) {
            s.store_sqrt_ad(315, A::add(A::square(s.ad_value(314)), s.ad_value(315)));
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (s.v[695] != 0.0)) && (s.v[704] != 0.0)) {
            s.store_sub_from_scalar_ad(361, p.p85, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5));
        }

    }

    pub(super) fn stamp_reactive_block_14(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (s.v[695] != 0.0)) && (s.v[704] != 0.0)) {
            s.store_offset(314, 361, (((-s.v[63])) + ((-0.01))));
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (s.v[695] != 0.0)) && (s.v[704] != 0.0)) {
            s.store_scalar(315, ((4.0 * s.v[63]) * 0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (s.v[695] != 0.0)) && (s.v[704] != 0.0)) {
            s.store_ad(315, &{
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (s.v[695] != 0.0)) && (s.v[704] != 0.0)) {
            s.store_sqrt_ad(315, A::add(A::square(s.ad_value(314)), s.ad_value(315)));
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (s.v[695] != 0.0)) && (s.v[704] != 0.0)) {
            s.store_offset_ad(359, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5), s.v[63]);
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (s.v[695] != 0.0)) && (s.v[704] != 0.0)) {
            s.store_offset_ad(314, A::sub_from_scalar(p.p85, s.ad_value(350)), (-0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (s.v[695] != 0.0)) && (s.v[704] != 0.0)) {
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (s.v[695] != 0.0)) && (s.v[704] != 0.0)) {
            s.store_ad(315, &{
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (s.v[695] != 0.0)) && (s.v[704] != 0.0)) {
            s.store_sqrt_ad(315, A::add(A::square(s.ad_value(314)), s.ad_value(315)));
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (s.v[695] != 0.0)) && (s.v[704] != 0.0)) {
            s.store_sub_from_scalar_ad(350, p.p85, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5));
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (s.v[695] != 0.0)) && (s.v[704] != 0.0)) {
            s.store_offset(314, 350, (((-s.v[63])) + ((-0.01))));
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (s.v[695] != 0.0)) && (s.v[704] != 0.0)) {
            s.store_scalar(315, ((4.0 * s.v[63]) * 0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (s.v[695] != 0.0)) && (s.v[704] != 0.0)) {
            s.store_ad(315, &{
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (s.v[695] != 0.0)) && (s.v[704] != 0.0)) {
            s.store_sqrt_ad(315, A::add(A::square(s.ad_value(314)), s.ad_value(315)));
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (s.v[695] != 0.0)) && (s.v[704] != 0.0)) {
            s.store_offset_ad(350, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5), s.v[63]);
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (s.v[695] != 0.0)) && (!(s.v[704] != 0.0))) {
            s.store_scalar(350, s.v[63]);
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (s.v[695] != 0.0)) && (!(s.v[704] != 0.0))) {
            s.store_scalar(359, s.v[63]);
        }

        s.v[705] = if ((((s.v[85] * ((s.v[193] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85))))) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (s.v[695] != 0.0)) && (s.v[705] != 0.0)) {
            s.store_exp_ad(372, A::scale(A::add(A::div(s.ad_value(193), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]));
        }

        s.v[706] = if ((s.v[85] * ((s.v[193] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85)))) < (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (s.v[695] != 0.0)) && (!(s.v[705] != 0.0))) && (s.v[706] != 0.0)) {
            let assign19600_ad_e28027: A = A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(A::add(A::div(s.ad_value(193), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85])), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::scale(A::add(A::div(s.ad_value(193), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85])), 0.3333333333333333), 1.0));
            s.store_div_from_scalar_ad(372, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(A::add(A::div(s.ad_value(193), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85])), A::offset(A::scale(assign19600_ad_e28027, 0.5), 1.0)), 1.0));
        }

        if (((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (s.v[695] != 0.0)) && (!(s.v[705] != 0.0))) && (!(s.v[706] != 0.0))) {
            let assign19610_ad_e28105: A = A::mul(A::offset(A::scale(A::add(A::div(s.ad_value(193), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]), (-230.25850929940458)), A::offset(A::scale(A::offset(A::scale(A::add(A::div(s.ad_value(193), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]), (-230.25850929940458)), 0.3333333333333333), 1.0));
            s.store_scale_ad(372, A::offset(A::mul(A::offset(A::scale(A::add(A::div(s.ad_value(193), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]), (-230.25850929940458)), A::offset(A::scale(assign19610_ad_e28105, 0.5), 1.0)), 1.0), 1e100);
        }

        if (((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (!(s.v[695] != 0.0))) {
            s.store_sqrt_ad(430, A::mul(A::offset(A::scale(A::sub(s.ad_value(193), s.ad_value(262)), s.v[85]), 1.0), s.ad_value(263)));
        }

        if (((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (!(s.v[695] != 0.0))) {
            s.store_scale_ad(363, A::square(s.ad_value(318)), 1.0 / (s.v[308]));
        }

        if (((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (!(s.v[695] != 0.0))) {
            s.store_scale_ad(362, A::ln(A::div_from_scalar(s.v[308], s.ad_value(363))), (s.v[62] / s.v[85]));
        }

        s.v[707] = if (s.v[62] < p.p85) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (!(s.v[695] != 0.0))) && (s.v[707] != 0.0)) {
            s.store_offset_ad(360, A::scale(A::sub(s.ad_value(262), s.ad_value(362)), p.p86), s.v[62]);
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (!(s.v[695] != 0.0))) && (s.v[707] != 0.0)) {
            s.store_sub_from_scalar_ad(350, s.v[62], A::scale(s.ad_value(362), p.p86));
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (!(s.v[695] != 0.0))) && (s.v[707] != 0.0)) {
            s.store_offset_ad(314, A::sub_from_scalar(p.p85, s.ad_value(360)), (-0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (!(s.v[695] != 0.0))) && (s.v[707] != 0.0)) {
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (!(s.v[695] != 0.0))) && (s.v[707] != 0.0)) {
            s.store_ad(315, &{
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (!(s.v[695] != 0.0))) && (s.v[707] != 0.0)) {
            s.store_sqrt_ad(315, A::add(A::square(s.ad_value(314)), s.ad_value(315)));
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (!(s.v[695] != 0.0))) && (s.v[707] != 0.0)) {
            s.store_scale_ad(364, A::offset(A::div(s.ad_value(314), s.ad_value(315)), 1.0), 0.5);
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (!(s.v[695] != 0.0))) && (s.v[707] != 0.0)) {
            s.store_sub_from_scalar_ad(361, p.p85, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5));
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (!(s.v[695] != 0.0))) && (s.v[707] != 0.0)) {
            s.store_offset(314, 361, (((-s.v[62])) + ((-0.01))));
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (!(s.v[695] != 0.0))) && (s.v[707] != 0.0)) {
            s.store_scalar(315, ((4.0 * s.v[62]) * 0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (!(s.v[695] != 0.0))) && (s.v[707] != 0.0)) {
            s.store_ad(315, &{
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (!(s.v[695] != 0.0))) && (s.v[707] != 0.0)) {
            s.store_sqrt_ad(315, A::add(A::square(s.ad_value(314)), s.ad_value(315)));
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (!(s.v[695] != 0.0))) && (s.v[707] != 0.0)) {
            s.store_scale_ad(365, A::offset(A::div(s.ad_value(314), s.ad_value(315)), 1.0), 0.5);
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (!(s.v[695] != 0.0))) && (s.v[707] != 0.0)) {
            s.store_offset_ad(359, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5), s.v[62]);
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (!(s.v[695] != 0.0))) && (s.v[707] != 0.0)) {
            s.store_offset_ad(314, A::sub_from_scalar(p.p85, s.ad_value(350)), (-0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (!(s.v[695] != 0.0))) && (s.v[707] != 0.0)) {
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (!(s.v[695] != 0.0))) && (s.v[707] != 0.0)) {
            s.store_ad(315, &{
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (!(s.v[695] != 0.0))) && (s.v[707] != 0.0)) {
            s.store_sqrt_ad(315, A::add(A::square(s.ad_value(314)), s.ad_value(315)));
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (!(s.v[695] != 0.0))) && (s.v[707] != 0.0)) {
            s.store_sub_from_scalar_ad(350, p.p85, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5));
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (!(s.v[695] != 0.0))) && (s.v[707] != 0.0)) {
            s.store_offset(314, 350, (((-s.v[62])) + ((-0.01))));
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (!(s.v[695] != 0.0))) && (s.v[707] != 0.0)) {
            s.store_scalar(315, ((4.0 * s.v[62]) * 0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (!(s.v[695] != 0.0))) && (s.v[707] != 0.0)) {
            s.store_ad(315, &{
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (!(s.v[695] != 0.0))) && (s.v[707] != 0.0)) {
            s.store_sqrt_ad(315, A::add(A::square(s.ad_value(314)), s.ad_value(315)));
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (!(s.v[695] != 0.0))) && (s.v[707] != 0.0)) {
            s.store_offset_ad(350, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5), s.v[62]);
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (!(s.v[695] != 0.0))) && (s.v[707] != 0.0)) {
            s.store_mul_ad_lhs(366, A::scale(s.ad_value(364), p.p86), 365);
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (!(s.v[695] != 0.0))) && (!(s.v[707] != 0.0))) {
            s.store_scalar(350, s.v[62]);
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (!(s.v[695] != 0.0))) && (!(s.v[707] != 0.0))) {
            s.store_scalar(359, s.v[62]);
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (!(s.v[695] != 0.0))) && (!(s.v[707] != 0.0))) {
            s.store_scalar(366, 0.0);
        }

        s.v[708] = if ((((s.v[85] * ((s.v[262] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85))))) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (!(s.v[695] != 0.0))) && (s.v[708] != 0.0)) {
            s.store_exp_ad(281, A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]));
        }

        s.v[709] = if ((s.v[85] * ((s.v[262] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85)))) < (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (!(s.v[695] != 0.0))) && (!(s.v[708] != 0.0))) && (s.v[709] != 0.0)) {
            let assign19970_ad_e28735: A = A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85])), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85])), 0.3333333333333333), 1.0));
            s.store_div_from_scalar_ad(281, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85])), A::offset(A::scale(assign19970_ad_e28735, 0.5), 1.0)), 1.0));
        }

        if (((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (!(s.v[695] != 0.0))) && (!(s.v[708] != 0.0))) && (!(s.v[709] != 0.0))) {
            let assign19980_ad_e28814: A = A::mul(A::offset(A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]), (-230.25850929940458)), A::offset(A::scale(A::offset(A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]), (-230.25850929940458)), 0.3333333333333333), 1.0));
            s.store_scale_ad(281, A::offset(A::mul(A::offset(A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]), (-230.25850929940458)), A::offset(A::scale(assign19980_ad_e28814, 0.5), 1.0)), 1.0), 1e100);
        }

        if (((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (!(s.v[695] != 0.0))) {
            s.store_scale_ad(367, A::add(A::div(A::sub(s.ad_value(359), A::mul(s.ad_value(262), s.ad_value(366))), A::square(s.ad_value(359))), A::div(A::mul(s.ad_value(362), s.ad_value(366)), A::scale(s.ad_value(350), p.p85))), s.v[85]);
        }

        if (((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (!(s.v[695] != 0.0))) {
            s.store_mul_ad_lhs(370, A::offset(A::mul(A::sub(s.ad_value(193), s.ad_value(262)), s.ad_value(367)), 1.0), 281);
        }

        if (((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (!(s.v[695] != 0.0))) {
            s.store_scale_ad(363, A::square(s.ad_value(318)), 1.0 / (s.v[310]));
        }

        if (((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (!(s.v[695] != 0.0))) {
            s.store_scale_ad(362, A::ln(A::div_from_scalar(s.v[310], s.ad_value(363))), (s.v[64] / s.v[85]));
        }

        s.v[710] = if (s.v[64] < p.p85) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (!(s.v[695] != 0.0))) && (s.v[710] != 0.0)) {
            s.store_offset_ad(360, A::scale(A::sub(s.ad_value(262), s.ad_value(362)), p.p86), s.v[64]);
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (!(s.v[695] != 0.0))) && (s.v[710] != 0.0)) {
            s.store_sub_from_scalar_ad(350, s.v[64], A::scale(s.ad_value(362), p.p86));
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (!(s.v[695] != 0.0))) && (s.v[710] != 0.0)) {
            s.store_offset_ad(314, A::sub_from_scalar(p.p85, s.ad_value(360)), (-0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (!(s.v[695] != 0.0))) && (s.v[710] != 0.0)) {
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (!(s.v[695] != 0.0))) && (s.v[710] != 0.0)) {
            s.store_ad(315, &{
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (!(s.v[695] != 0.0))) && (s.v[710] != 0.0)) {
            s.store_sqrt_ad(315, A::add(A::square(s.ad_value(314)), s.ad_value(315)));
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (!(s.v[695] != 0.0))) && (s.v[710] != 0.0)) {
            s.store_scale_ad(364, A::offset(A::div(s.ad_value(314), s.ad_value(315)), 1.0), 0.5);
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (!(s.v[695] != 0.0))) && (s.v[710] != 0.0)) {
            s.store_sub_from_scalar_ad(361, p.p85, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5));
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (!(s.v[695] != 0.0))) && (s.v[710] != 0.0)) {
            s.store_offset(314, 361, (((-s.v[64])) + ((-0.01))));
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (!(s.v[695] != 0.0))) && (s.v[710] != 0.0)) {
            s.store_scalar(315, ((4.0 * s.v[64]) * 0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (!(s.v[695] != 0.0))) && (s.v[710] != 0.0)) {
            s.store_ad(315, &{
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (!(s.v[695] != 0.0))) && (s.v[710] != 0.0)) {
            s.store_sqrt_ad(315, A::add(A::square(s.ad_value(314)), s.ad_value(315)));
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (!(s.v[695] != 0.0))) && (s.v[710] != 0.0)) {
            s.store_scale_ad(365, A::offset(A::div(s.ad_value(314), s.ad_value(315)), 1.0), 0.5);
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (!(s.v[695] != 0.0))) && (s.v[710] != 0.0)) {
            s.store_offset_ad(359, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5), s.v[64]);
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (!(s.v[695] != 0.0))) && (s.v[710] != 0.0)) {
            s.store_offset_ad(314, A::sub_from_scalar(p.p85, s.ad_value(350)), (-0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (!(s.v[695] != 0.0))) && (s.v[710] != 0.0)) {
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (!(s.v[695] != 0.0))) && (s.v[710] != 0.0)) {
            s.store_ad(315, &{
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (!(s.v[695] != 0.0))) && (s.v[710] != 0.0)) {
            s.store_sqrt_ad(315, A::add(A::square(s.ad_value(314)), s.ad_value(315)));
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (!(s.v[695] != 0.0))) && (s.v[710] != 0.0)) {
            s.store_sub_from_scalar_ad(350, p.p85, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5));
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (!(s.v[695] != 0.0))) && (s.v[710] != 0.0)) {
            s.store_offset(314, 350, (((-s.v[64])) + ((-0.01))));
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (!(s.v[695] != 0.0))) && (s.v[710] != 0.0)) {
            s.store_scalar(315, ((4.0 * s.v[64]) * 0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (!(s.v[695] != 0.0))) && (s.v[710] != 0.0)) {
            s.store_ad(315, &{
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (!(s.v[695] != 0.0))) && (s.v[710] != 0.0)) {
            s.store_sqrt_ad(315, A::add(A::square(s.ad_value(314)), s.ad_value(315)));
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (!(s.v[695] != 0.0))) && (s.v[710] != 0.0)) {
            s.store_offset_ad(350, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5), s.v[64]);
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (!(s.v[695] != 0.0))) && (s.v[710] != 0.0)) {
            s.store_mul_ad_lhs(366, A::scale(s.ad_value(364), p.p86), 365);
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (!(s.v[695] != 0.0))) && (!(s.v[710] != 0.0))) {
            s.store_scalar(350, s.v[64]);
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (!(s.v[695] != 0.0))) && (!(s.v[710] != 0.0))) {
            s.store_scalar(359, s.v[64]);
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (!(s.v[695] != 0.0))) && (!(s.v[710] != 0.0))) {
            s.store_scalar(366, 0.0);
        }

        s.v[711] = if ((((s.v[85] * ((s.v[262] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85))))) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (!(s.v[695] != 0.0))) && (s.v[711] != 0.0)) {
            s.store_exp_ad(282, A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]));
        }

        s.v[712] = if ((s.v[85] * ((s.v[262] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85)))) < (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (!(s.v[695] != 0.0))) && (!(s.v[711] != 0.0))) && (s.v[712] != 0.0)) {
            let assign20350_ad_e29470: A = A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85])), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85])), 0.3333333333333333), 1.0));
            s.store_div_from_scalar_ad(282, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85])), A::offset(A::scale(assign20350_ad_e29470, 0.5), 1.0)), 1.0));
        }

        if (((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (!(s.v[695] != 0.0))) && (!(s.v[711] != 0.0))) && (!(s.v[712] != 0.0))) {
            let assign20360_ad_e29549: A = A::mul(A::offset(A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]), (-230.25850929940458)), A::offset(A::scale(A::offset(A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]), (-230.25850929940458)), 0.3333333333333333), 1.0));
            s.store_scale_ad(282, A::offset(A::mul(A::offset(A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]), (-230.25850929940458)), A::offset(A::scale(assign20360_ad_e29549, 0.5), 1.0)), 1.0), 1e100);
        }

        if (((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (!(s.v[695] != 0.0))) {
            s.store_scale_ad(367, A::add(A::div(A::sub(s.ad_value(359), A::mul(s.ad_value(262), s.ad_value(366))), A::square(s.ad_value(359))), A::div(A::mul(s.ad_value(362), s.ad_value(366)), A::scale(s.ad_value(350), p.p85))), s.v[85]);
        }

        if (((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (!(s.v[695] != 0.0))) {
            s.store_mul_ad_lhs(371, A::offset(A::mul(A::sub(s.ad_value(193), s.ad_value(262)), s.ad_value(367)), 1.0), 282);
        }

        if (((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (!(s.v[695] != 0.0))) {
            s.store_scale_ad(363, A::square(s.ad_value(318)), 1.0 / (s.v[309]));
        }

        if (((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (!(s.v[695] != 0.0))) {
            s.store_scale_ad(362, A::ln(A::div_from_scalar(s.v[309], s.ad_value(363))), (s.v[63] / s.v[85]));
        }

        s.v[713] = if (s.v[63] < p.p85) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (!(s.v[695] != 0.0))) && (s.v[713] != 0.0)) {
            s.store_offset_ad(360, A::scale(A::sub(s.ad_value(262), s.ad_value(362)), p.p86), s.v[63]);
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (!(s.v[695] != 0.0))) && (s.v[713] != 0.0)) {
            s.store_sub_from_scalar_ad(350, s.v[63], A::scale(s.ad_value(362), p.p86));
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (!(s.v[695] != 0.0))) && (s.v[713] != 0.0)) {
            s.store_offset_ad(314, A::sub_from_scalar(p.p85, s.ad_value(360)), (-0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (!(s.v[695] != 0.0))) && (s.v[713] != 0.0)) {
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (!(s.v[695] != 0.0))) && (s.v[713] != 0.0)) {
            s.store_ad(315, &{
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (!(s.v[695] != 0.0))) && (s.v[713] != 0.0)) {
            s.store_sqrt_ad(315, A::add(A::square(s.ad_value(314)), s.ad_value(315)));
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (!(s.v[695] != 0.0))) && (s.v[713] != 0.0)) {
            s.store_scale_ad(364, A::offset(A::div(s.ad_value(314), s.ad_value(315)), 1.0), 0.5);
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (!(s.v[695] != 0.0))) && (s.v[713] != 0.0)) {
            s.store_sub_from_scalar_ad(361, p.p85, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5));
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (!(s.v[695] != 0.0))) && (s.v[713] != 0.0)) {
            s.store_offset(314, 361, (((-s.v[63])) + ((-0.01))));
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (!(s.v[695] != 0.0))) && (s.v[713] != 0.0)) {
            s.store_scalar(315, ((4.0 * s.v[63]) * 0.01));
        }

    }

    pub(super) fn stamp_reactive_block_15(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (!(s.v[695] != 0.0))) && (s.v[713] != 0.0)) {
            s.store_ad(315, &{
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (!(s.v[695] != 0.0))) && (s.v[713] != 0.0)) {
            s.store_sqrt_ad(315, A::add(A::square(s.ad_value(314)), s.ad_value(315)));
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (!(s.v[695] != 0.0))) && (s.v[713] != 0.0)) {
            s.store_scale_ad(365, A::offset(A::div(s.ad_value(314), s.ad_value(315)), 1.0), 0.5);
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (!(s.v[695] != 0.0))) && (s.v[713] != 0.0)) {
            s.store_offset_ad(359, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5), s.v[63]);
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (!(s.v[695] != 0.0))) && (s.v[713] != 0.0)) {
            s.store_offset_ad(314, A::sub_from_scalar(p.p85, s.ad_value(350)), (-0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (!(s.v[695] != 0.0))) && (s.v[713] != 0.0)) {
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (!(s.v[695] != 0.0))) && (s.v[713] != 0.0)) {
            s.store_ad(315, &{
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (!(s.v[695] != 0.0))) && (s.v[713] != 0.0)) {
            s.store_sqrt_ad(315, A::add(A::square(s.ad_value(314)), s.ad_value(315)));
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (!(s.v[695] != 0.0))) && (s.v[713] != 0.0)) {
            s.store_sub_from_scalar_ad(350, p.p85, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5));
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (!(s.v[695] != 0.0))) && (s.v[713] != 0.0)) {
            s.store_offset(314, 350, (((-s.v[63])) + ((-0.01))));
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (!(s.v[695] != 0.0))) && (s.v[713] != 0.0)) {
            s.store_scalar(315, ((4.0 * s.v[63]) * 0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (!(s.v[695] != 0.0))) && (s.v[713] != 0.0)) {
            s.store_ad(315, &{
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (!(s.v[695] != 0.0))) && (s.v[713] != 0.0)) {
            s.store_sqrt_ad(315, A::add(A::square(s.ad_value(314)), s.ad_value(315)));
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (!(s.v[695] != 0.0))) && (s.v[713] != 0.0)) {
            s.store_offset_ad(350, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5), s.v[63]);
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (!(s.v[695] != 0.0))) && (s.v[713] != 0.0)) {
            s.store_mul_ad_lhs(366, A::scale(s.ad_value(364), p.p86), 365);
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (!(s.v[695] != 0.0))) && (!(s.v[713] != 0.0))) {
            s.store_scalar(350, s.v[63]);
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (!(s.v[695] != 0.0))) && (!(s.v[713] != 0.0))) {
            s.store_scalar(359, s.v[63]);
        }

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (!(s.v[695] != 0.0))) && (!(s.v[713] != 0.0))) {
            s.store_scalar(366, 0.0);
        }

        s.v[714] = if ((((s.v[85] * ((s.v[262] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85))))) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (!(s.v[695] != 0.0))) && (s.v[714] != 0.0)) {
            s.store_exp_ad(283, A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]));
        }

        s.v[715] = if ((s.v[85] * ((s.v[262] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85)))) < (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (!(s.v[695] != 0.0))) && (!(s.v[714] != 0.0))) && (s.v[715] != 0.0)) {
            let assign20730_ad_e30205: A = A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85])), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85])), 0.3333333333333333), 1.0));
            s.store_div_from_scalar_ad(283, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85])), A::offset(A::scale(assign20730_ad_e30205, 0.5), 1.0)), 1.0));
        }

        if (((((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (!(s.v[695] != 0.0))) && (!(s.v[714] != 0.0))) && (!(s.v[715] != 0.0))) {
            let assign20740_ad_e30284: A = A::mul(A::offset(A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]), (-230.25850929940458)), A::offset(A::scale(A::offset(A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]), (-230.25850929940458)), 0.3333333333333333), 1.0));
            s.store_scale_ad(283, A::offset(A::mul(A::offset(A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]), (-230.25850929940458)), A::offset(A::scale(assign20740_ad_e30284, 0.5), 1.0)), 1.0), 1e100);
        }

        if (((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (!(s.v[695] != 0.0))) {
            s.store_scale_ad(367, A::add(A::div(A::sub(s.ad_value(359), A::mul(s.ad_value(262), s.ad_value(366))), A::square(s.ad_value(359))), A::div(A::mul(s.ad_value(362), s.ad_value(366)), A::scale(s.ad_value(350), p.p85))), s.v[85]);
        }

        if (((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (!(s.v[695] != 0.0))) {
            s.store_mul_ad_lhs(372, A::offset(A::mul(A::sub(s.ad_value(193), s.ad_value(262)), s.ad_value(367)), 1.0), 283);
        }

        if ((s.v[418] != 0.0) && (s.v[694] != 0.0)) {
            s.store_offset(370, 370, (-1.0));
        }

        if ((s.v[418] != 0.0) && (s.v[694] != 0.0)) {
            s.store_offset(371, 371, (-1.0));
        }

        if ((s.v[418] != 0.0) && (s.v[694] != 0.0)) {
            s.store_offset(372, 372, (-1.0));
        }

        if ((s.v[418] != 0.0) && (s.v[694] != 0.0)) {
            s.store_div_from_scalar(429, 1.0, 430);
        }

        s.v[716] = if (s.v[193] > 0.0) { 1.0 } else { 0.0 };

        if (((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (s.v[716] != 0.0)) {
            s.store_scale_ad(431, A::ln(A::add(A::offset(s.ad_value(429), 2.0), A::sqrt(A::mul(A::offset(s.ad_value(429), 1.0), A::offset(s.ad_value(429), 3.0))))), (s.v[84] * 2.0));
        }

        if (((s.v[418] != 0.0) && (s.v[694] != 0.0)) && (!(s.v[716] != 0.0))) {
            s.store_sub_ad_lhs(431, A::scale(A::ln(A::add(A::offset(A::scale(s.ad_value(430), 2.0), 1.0), A::sqrt(A::mul(A::offset(s.ad_value(430), 1.0), A::offset(A::scale(s.ad_value(430), 3.0), 1.0))))), (s.v[84] * 2.0)), 193);
        }

        if ((s.v[418] != 0.0) && (s.v[694] != 0.0)) {
            s.store_sub(432, 264, 431);
        }

        if ((s.v[418] != 0.0) && (s.v[694] != 0.0)) {
            s.store_scale_ad(433, A::sub(A::add(s.ad_value(193), s.ad_value(432)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(193), s.ad_value(432)), A::sub(s.ad_value(193), s.ad_value(432))), ((4.0 * s.v[84]) * s.v[84])))), 0.5);
        }

        if ((s.v[418] != 0.0) && (s.v[694] != 0.0)) {
            s.store_scale_ad(434, A::sub(A::add(s.ad_value(193), s.ad_value(267)), A::sqrt(A::add(A::mul(A::sub(s.ad_value(193), s.ad_value(267)), A::sub(s.ad_value(193), s.ad_value(267))), A::mul(A::scale(s.ad_value(82), 4.0), s.ad_value(82))))), 0.5);
        }

        if ((s.v[418] != 0.0) && (s.v[694] != 0.0)) {
            s.store_scale_ad(435, A::sub(s.ad_value(193), A::sqrt(A::offset(A::mul(s.ad_value(193), s.ad_value(193)), ((4.0 * 1e-6) * 1e-6)))), 0.5);
        }

        if ((s.v[418] != 0.0) && (!(s.v[694] != 0.0))) {
            s.store_scalar(370, 0.0);
        }

        if ((s.v[418] != 0.0) && (!(s.v[694] != 0.0))) {
            s.store_scalar(371, 0.0);
        }

        if ((s.v[418] != 0.0) && (!(s.v[694] != 0.0))) {
            s.store_scalar(372, 0.0);
        }

        if ((s.v[418] != 0.0) && (!(s.v[694] != 0.0))) {
            s.store_scalar(431, 0.0);
        }

        if ((s.v[418] != 0.0) && (!(s.v[694] != 0.0))) {
            s.store_scalar(428, 0.0);
        }

        if ((s.v[418] != 0.0) && (!(s.v[694] != 0.0))) {
            s.store_scalar(430, 0.0);
        }

        if ((s.v[418] != 0.0) && (!(s.v[694] != 0.0))) {
            s.store_scalar(433, 0.0);
        }

        if ((s.v[418] != 0.0) && (!(s.v[694] != 0.0))) {
            s.store_scalar(434, 0.0);
        }

        if ((s.v[418] != 0.0) && (!(s.v[694] != 0.0))) {
            s.store_scalar(435, 0.0);
        }

        s.v[717] = if (s.v[256] == 0.0) { 1.0 } else { 0.0 };

        if ((s.v[418] != 0.0) && (s.v[717] != 0.0)) {
            s.store_scalar(268, 0.0);
        }

        if ((s.v[418] != 0.0) && (s.v[717] != 0.0)) {
            s.store_scalar(291, 0.0);
        }

        if ((s.v[418] != 0.0) && (s.v[717] != 0.0)) {
            s.store_scalar(269, 0.0);
        }

        s.v[718] = if (s.v[122] == 0.5) { 1.0 } else { 0.0 };

        if (((s.v[418] != 0.0) && (!(s.v[717] != 0.0))) && (s.v[718] != 0.0)) {
            s.store_sqrt_ad(436, A::sub_from_scalar(1.0, A::mul(s.ad_value(428), s.ad_value(119))));
        }

        if (((s.v[418] != 0.0) && (!(s.v[717] != 0.0))) && (!(s.v[718] != 0.0))) {
            s.store_powf_ad(436, A::sub_from_scalar(1.0, A::mul(s.ad_value(428), s.ad_value(119))), s.v[122]);
        }

        if ((s.v[418] != 0.0) && (!(s.v[717] != 0.0))) {
            s.store_add_ad(269, A::mul(s.ad_value(131), A::sub_from_scalar(1.0, s.ad_value(436))), A::mul(s.ad_value(134), A::sub(s.ad_value(193), s.ad_value(428))));
        }

        if ((s.v[418] != 0.0) && (!(s.v[717] != 0.0))) {
            s.store_mul(437, 101, 370);
        }

        s.v[719] = if ((s.v[20] == 0.0) && (s.v[23] == 0.0)) { 1.0 } else { 0.0 };

        if (((s.v[418] != 0.0) && (!(s.v[717] != 0.0))) && (s.v[719] != 0.0)) {
            s.store_scalar(439, 0.0);
        }

        if (((s.v[418] != 0.0) && (!(s.v[717] != 0.0))) && (s.v[719] != 0.0)) {
            s.store_scalar(442, 0.0);
        }

        if (((s.v[418] != 0.0) && (!(s.v[717] != 0.0))) && (s.v[719] != 0.0)) {
            s.store_scalar(443, 0.0);
        }

        if (((s.v[418] != 0.0) && (!(s.v[717] != 0.0))) && (s.v[719] != 0.0)) {
            s.store_scalar(444, 0.0);
        }

        if (((s.v[418] != 0.0) && (!(s.v[717] != 0.0))) && (s.v[719] != 0.0)) {
            s.store_scalar(438, 0.0);
        }

        if (((s.v[418] != 0.0) && (!(s.v[717] != 0.0))) && (!(s.v[719] != 0.0))) {
            s.store_sub(439, 107, 433);
        }

        if (((s.v[418] != 0.0) && (!(s.v[717] != 0.0))) && (!(s.v[719] != 0.0))) {
            s.store_sub_from_scalar_ad(440, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(431), s.ad_value(439)))));
        }

        s.v[720] = if (s.v[9] == 0.5) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (!(s.v[717] != 0.0))) && (!(s.v[719] != 0.0))) && (s.v[720] != 0.0)) {
            s.store_scalar(441, 0.0);
        }

        if ((((s.v[418] != 0.0) && (!(s.v[717] != 0.0))) && (!(s.v[719] != 0.0))) && (!(s.v[720] != 0.0))) {
            s.store_scale_ad(441, A::add(A::div(A::mul(A::square(s.ad_value(440)), A::ln(s.ad_value(440))), A::sub_from_scalar(1.0, s.ad_value(440))), s.ad_value(440)), (1.0 - (2.0 * s.v[9])));
        }

        if (((s.v[418] != 0.0) && (!(s.v[717] != 0.0))) && (!(s.v[719] != 0.0))) {
            s.store_add(442, 440, 441);
        }

        s.v[721] = if (s.v[9] == 0.5) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (!(s.v[717] != 0.0))) && (!(s.v[719] != 0.0))) && (s.v[721] != 0.0)) {
            s.store_sqrt_ad(436, A::scale(s.ad_value(439), s.v[143]));
        }

        if ((((s.v[418] != 0.0) && (!(s.v[717] != 0.0))) && (!(s.v[719] != 0.0))) && (!(s.v[721] != 0.0))) {
            s.store_powf_ad(436, A::scale(s.ad_value(439), s.v[143]), s.v[9]);
        }

        if (((s.v[418] != 0.0) && (!(s.v[717] != 0.0))) && (!(s.v[719] != 0.0))) {
            s.store_scale(443, 436, s.v[137]);
        }

        if (((s.v[418] != 0.0) && (!(s.v[717] != 0.0))) && (!(s.v[719] != 0.0))) {
            s.store_mul_ad_rhs(444, 98, A::mul(A::offset(s.ad_value(430), (-1.0)), s.ad_value(443)));
        }

        if (((s.v[418] != 0.0) && (!(s.v[717] != 0.0))) && (!(s.v[719] != 0.0))) {
            s.store_scaled_mul(438, 444, 442, s.v[20]);
        }

        s.v[722] = if (s.v[23] == 0.0) { 1.0 } else { 0.0 };

        if (((s.v[418] != 0.0) && (!(s.v[717] != 0.0))) && (s.v[722] != 0.0)) {
            s.store_scalar(445, 0.0);
        }

        if (((s.v[418] != 0.0) && (!(s.v[717] != 0.0))) && (!(s.v[722] != 0.0))) {
            s.store_scale_ad(446, A::div(A::scale(s.ad_value(443), s.v[122]), s.ad_value(439)), s.v[152]);
        }

        if (((s.v[418] != 0.0) && (!(s.v[717] != 0.0))) && (!(s.v[722] != 0.0))) {
            s.store_div_from_scalar(447, (0.666666666666667 * s.v[149]), 446);
        }

        if (((s.v[418] != 0.0) && (!(s.v[717] != 0.0))) && (!(s.v[722] != 0.0))) {
            s.store_square(448, 447);
        }

        if (((s.v[418] != 0.0) && (!(s.v[717] != 0.0))) && (!(s.v[722] != 0.0))) {
            s.store_sqrt_ad(449, A::div(A::square(s.ad_value(448)), A::offset(A::square(s.ad_value(448)), 1.0)));
        }

        if (((s.v[418] != 0.0) && (!(s.v[717] != 0.0))) && (!(s.v[722] != 0.0))) {
            s.store_sqrt_ad(450, A::abs(s.ad_value(449)));
        }

        if (((s.v[418] != 0.0) && (!(s.v[717] != 0.0))) && (!(s.v[722] != 0.0))) {
            s.store_mul(451, 449, 450);
        }

        s.v[723] = if (((-s.v[9]) * s.v[125]) == (-1.0)) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (!(s.v[717] != 0.0))) && (!(s.v[722] != 0.0))) && (s.v[723] != 0.0)) {
            s.store_div_from_scalar_ad(452, 1.0, A::offset(A::mul(s.ad_value(446), s.ad_value(451)), 1.0));
        }

        if ((((s.v[418] != 0.0) && (!(s.v[717] != 0.0))) && (!(s.v[722] != 0.0))) && (!(s.v[723] != 0.0))) {
            s.store_powf_ad(452, A::offset(A::mul(s.ad_value(446), s.ad_value(451)), 1.0), ((-s.v[9]) * s.v[125]));
        }

        if (((s.v[418] != 0.0) && (!(s.v[717] != 0.0))) && (!(s.v[722] != 0.0))) {
            s.store_div_ad(453, A::mul(s.ad_value(442), s.ad_value(452)), A::add(s.ad_value(442), s.ad_value(452)));
        }

        if (((s.v[418] != 0.0) && (!(s.v[717] != 0.0))) && (!(s.v[722] != 0.0))) {
            s.store_sqrt_ad(454, A::scale(A::div(s.ad_value(446), s.ad_value(450)), 0.375));
        }

        if (((s.v[418] != 0.0) && (!(s.v[717] != 0.0))) && (!(s.v[722] != 0.0))) {
            s.store_sub_ad_lhs(455, A::scale(A::mul(s.ad_value(447), s.ad_value(450)), 2.0), 449);
        }

        if (((s.v[418] != 0.0) && (!(s.v[717] != 0.0))) && (!(s.v[722] != 0.0))) {
            s.store_add_ad(456, A::sub(A::mul(A::scale(s.ad_value(447), s.v[149]), s.ad_value(450)), A::scale(s.ad_value(449), s.v[149])), A::scale(A::mul(s.ad_value(446), s.ad_value(451)), 0.5));
        }

        if (((s.v[418] != 0.0) && (!(s.v[717] != 0.0))) && (!(s.v[722] != 0.0))) {
            s.store_mul_ad_lhs(457, A::offset(s.ad_value(455), (-1.0)), 454);
        }

        if (((s.v[418] != 0.0) && (!(s.v[717] != 0.0))) && (!(s.v[722] != 0.0))) {
            s.store_square(419, 457);
        }

        s.v[724] = if (s.v[457] > 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (!(s.v[717] != 0.0))) && (!(s.v[722] != 0.0))) && (s.v[724] != 0.0)) {
            s.store_div_from_scalar_ad(420, 1.0, A::offset(A::scale(s.ad_value(457), s.v[86]), 1.0));
        }

        if ((((s.v[418] != 0.0) && (!(s.v[717] != 0.0))) && (!(s.v[722] != 0.0))) && (!(s.v[724] != 0.0))) {
            s.store_div_from_scalar_ad(420, 1.0, A::sub_from_scalar(1.0, A::scale(s.ad_value(457), s.v[86])));
        }

        s.v[725] = if (((-s.v[419]) + s.v[456]) > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (!(s.v[717] != 0.0))) && (!(s.v[722] != 0.0))) && (s.v[725] != 0.0)) {
            s.store_exp_ad(436, A::sub(s.ad_value(456), s.ad_value(419)));
        }

        if ((((s.v[418] != 0.0) && (!(s.v[717] != 0.0))) && (!(s.v[722] != 0.0))) && (!(s.v[725] != 0.0))) {
            s.store_div_from_scalar_ad(436, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(456), s.ad_value(419))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(456), s.ad_value(419))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(456), s.ad_value(419))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if (((s.v[418] != 0.0) && (!(s.v[717] != 0.0))) && (!(s.v[722] != 0.0))) {
            s.store_mul_ad_lhs(421, A::add(A::add(A::scale(s.ad_value(420), 0.29214664), A::scale(A::square(s.ad_value(420)), s.v[87])), A::scale(A::mul(A::square(s.ad_value(420)), s.ad_value(420)), s.v[88])), 436);
        }

        s.v[726] = if (s.v[457] > 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (!(s.v[717] != 0.0))) && (!(s.v[722] != 0.0))) && (s.v[726] != 0.0)) {
            s.copy_ad(458, 421);
        }

        s.v[727] = if (s.v[456] > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((s.v[418] != 0.0) && (!(s.v[717] != 0.0))) && (!(s.v[722] != 0.0))) && (!(s.v[726] != 0.0))) && (s.v[727] != 0.0)) {
            s.store_exp(436, 456);
        }

        if (((((s.v[418] != 0.0) && (!(s.v[717] != 0.0))) && (!(s.v[722] != 0.0))) && (!(s.v[726] != 0.0))) && (!(s.v[727] != 0.0))) {
            s.store_div_from_scalar_ad(436, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(456)), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(456)), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), s.ad_value(456)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if ((((s.v[418] != 0.0) && (!(s.v[717] != 0.0))) && (!(s.v[722] != 0.0))) && (!(s.v[726] != 0.0))) {
            s.store_sub_ad_lhs(458, A::scale(s.ad_value(436), 2.0), 421);
        }

        if (((s.v[418] != 0.0) && (!(s.v[717] != 0.0))) && (!(s.v[722] != 0.0))) {
            s.store_scale_ad(459, A::div(A::scale(s.ad_value(458), s.v[149]), s.ad_value(454)), (1.772453850905516 * 0.5));
        }

        if (((s.v[418] != 0.0) && (!(s.v[717] != 0.0))) && (!(s.v[722] != 0.0))) {
            s.store_scale_ad(445, A::mul(A::mul(s.ad_value(444), s.ad_value(459)), s.ad_value(453)), s.v[23]);
        }

        s.v[728] = if (s.v[29] == 0.0) { 1.0 } else { 0.0 };

        if (((s.v[418] != 0.0) && (!(s.v[717] != 0.0))) && (s.v[728] != 0.0)) {
            s.store_scalar(460, 0.0);
        }

        s.v[729] = if (s.v[9] == 0.5) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (!(s.v[717] != 0.0))) && (!(s.v[728] != 0.0))) && (s.v[729] != 0.0)) {
            s.store_sqrt_ad(436, A::scale(A::sub_from_scalar(s.v[6], s.ad_value(434)), s.v[143]));
        }

        if ((((s.v[418] != 0.0) && (!(s.v[717] != 0.0))) && (!(s.v[728] != 0.0))) && (!(s.v[729] != 0.0))) {
            s.store_powf_ad(436, A::scale(A::sub_from_scalar(s.v[6], s.ad_value(434)), s.v[143]), s.v[9]);
        }

        if (((s.v[418] != 0.0) && (!(s.v[717] != 0.0))) && (!(s.v[728] != 0.0))) {
            s.store_scale_ad(461, A::div(A::scale(A::sub_from_scalar(s.v[6], s.ad_value(434)), s.v[140]), s.ad_value(436)), s.v[125]);
        }

        s.v[730] = if (((((-s.v[155]) / s.v[461])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (!(s.v[717] != 0.0))) && (!(s.v[728] != 0.0))) && (s.v[730] != 0.0)) {
            s.store_exp_ad(436, A::div(A::neg(s.ad_value(155)), s.ad_value(461)));
        }

        s.v[731] = if (((-s.v[155]) / s.v[461]) < (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((s.v[418] != 0.0) && (!(s.v[717] != 0.0))) && (!(s.v[728] != 0.0))) && (!(s.v[730] != 0.0))) && (s.v[731] != 0.0)) {
            let assign21650_ad_e31518: A = A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(155)), s.ad_value(461))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(155)), s.ad_value(461))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(155)), s.ad_value(461))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
            s.store_div_from_scalar_ad(436, 1e-100, assign21650_ad_e31518);
        }

        if (((((s.v[418] != 0.0) && (!(s.v[717] != 0.0))) && (!(s.v[728] != 0.0))) && (!(s.v[730] != 0.0))) && (!(s.v[731] != 0.0))) {
            let assign21660_ad_e31566: A = A::scale(A::offset(A::mul(A::offset(A::div(A::neg(s.ad_value(155)), s.ad_value(461)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::div(A::neg(s.ad_value(155)), s.ad_value(461)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::div(A::neg(s.ad_value(155)), s.ad_value(461)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
            s.store_ad(436, &assign21660_ad_e31566);
        }

        if (((s.v[418] != 0.0) && (!(s.v[717] != 0.0))) && (!(s.v[728] != 0.0))) {
            s.store_scale_ad(460, A::mul(A::mul(A::mul(s.ad_value(193), s.ad_value(461)), s.ad_value(461)), s.ad_value(436)), s.v[29]);
        }

        s.v[732] = if ((s.v[38] > 1000000.0) || (p.p80 == 0.0)) { 1.0 } else { 0.0 };

        if (((s.v[418] != 0.0) && (!(s.v[717] != 0.0))) && (s.v[732] != 0.0)) {
            s.store_scalar(462, 1.0);
        }

        s.v[733] = if (s.v[435] > ((-s.v[158]) * s.v[38])) { 1.0 } else { 0.0 };

        s.v[734] = if (s.v[41] == 4.0) { 1.0 } else { 0.0 };

        if (((((s.v[418] != 0.0) && (!(s.v[717] != 0.0))) && (!(s.v[732] != 0.0))) && (s.v[733] != 0.0)) && (s.v[734] != 0.0)) {
            s.store_mul_ad(436, A::mul(A::mul(A::abs(A::mul(s.ad_value(435), s.ad_value(162))), A::abs(A::mul(s.ad_value(435), s.ad_value(162)))), A::abs(A::mul(s.ad_value(435), s.ad_value(162)))), A::abs(A::mul(s.ad_value(435), s.ad_value(162))));
        }

        if (((((s.v[418] != 0.0) && (!(s.v[717] != 0.0))) && (!(s.v[732] != 0.0))) && (s.v[733] != 0.0)) && (!(s.v[734] != 0.0))) {
            s.store_powf_ad(436, A::abs(A::mul(s.ad_value(435), s.ad_value(162))), s.v[41]);
        }

        if ((((s.v[418] != 0.0) && (!(s.v[717] != 0.0))) && (!(s.v[732] != 0.0))) && (s.v[733] != 0.0)) {
            s.store_div_from_scalar_ad(462, 1.0, A::sub_from_scalar(1.0, s.ad_value(436)));
        }

        if ((((s.v[418] != 0.0) && (!(s.v[717] != 0.0))) && (!(s.v[732] != 0.0))) && (!(s.v[733] != 0.0))) {
            s.store_offset_ad(462, A::mul(A::add(s.ad_value(435), A::scale(s.ad_value(38), s.v[158])), s.ad_value(165)), s.v[159]);
        }

        if ((s.v[418] != 0.0) && (!(s.v[717] != 0.0))) {
            s.store_mul_ad_lhs(268, A::add(A::add(A::add(s.ad_value(437), s.ad_value(438)), s.ad_value(445)), s.ad_value(460)), 462);
        }

        if ((s.v[418] != 0.0) && (!(s.v[717] != 0.0))) {
            s.store_mul_ad_lhs(291, A::add(A::add(s.ad_value(438), s.ad_value(445)), s.ad_value(460)), 462);
        }

        s.v[735] = if (s.v[257] == 0.0) { 1.0 } else { 0.0 };

        if ((s.v[418] != 0.0) && (s.v[735] != 0.0)) {
            s.store_scalar(270, 0.0);
        }

        if ((s.v[418] != 0.0) && (s.v[735] != 0.0)) {
            s.store_scalar(292, 0.0);
        }

        if ((s.v[418] != 0.0) && (s.v[735] != 0.0)) {
            s.store_scalar(271, 0.0);
        }

        s.v[736] = if (s.v[123] == 0.5) { 1.0 } else { 0.0 };

        if (((s.v[418] != 0.0) && (!(s.v[735] != 0.0))) && (s.v[736] != 0.0)) {
            s.store_sqrt_ad(436, A::sub_from_scalar(1.0, A::mul(s.ad_value(428), s.ad_value(120))));
        }

        if (((s.v[418] != 0.0) && (!(s.v[735] != 0.0))) && (!(s.v[736] != 0.0))) {
            s.store_powf_ad(436, A::sub_from_scalar(1.0, A::mul(s.ad_value(428), s.ad_value(120))), s.v[123]);
        }

        if ((s.v[418] != 0.0) && (!(s.v[735] != 0.0))) {
            s.store_add_ad(271, A::mul(s.ad_value(132), A::sub_from_scalar(1.0, s.ad_value(436))), A::mul(s.ad_value(135), A::sub(s.ad_value(193), s.ad_value(428))));
        }

    }

    pub(super) fn stamp_reactive_block_16(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if ((s.v[418] != 0.0) && (!(s.v[735] != 0.0))) {
            s.store_mul(437, 102, 371);
        }

        s.v[737] = if ((s.v[21] == 0.0) && (s.v[24] == 0.0)) { 1.0 } else { 0.0 };

        if (((s.v[418] != 0.0) && (!(s.v[735] != 0.0))) && (s.v[737] != 0.0)) {
            s.store_scalar(439, 0.0);
        }

        if (((s.v[418] != 0.0) && (!(s.v[735] != 0.0))) && (s.v[737] != 0.0)) {
            s.store_scalar(442, 0.0);
        }

        if (((s.v[418] != 0.0) && (!(s.v[735] != 0.0))) && (s.v[737] != 0.0)) {
            s.store_scalar(443, 0.0);
        }

        if (((s.v[418] != 0.0) && (!(s.v[735] != 0.0))) && (s.v[737] != 0.0)) {
            s.store_scalar(444, 0.0);
        }

        if (((s.v[418] != 0.0) && (!(s.v[735] != 0.0))) && (s.v[737] != 0.0)) {
            s.store_scalar(438, 0.0);
        }

        if (((s.v[418] != 0.0) && (!(s.v[735] != 0.0))) && (!(s.v[737] != 0.0))) {
            s.store_sub(439, 108, 433);
        }

        if (((s.v[418] != 0.0) && (!(s.v[735] != 0.0))) && (!(s.v[737] != 0.0))) {
            s.store_sub_from_scalar_ad(440, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(431), s.ad_value(439)))));
        }

        s.v[738] = if (s.v[10] == 0.5) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (!(s.v[735] != 0.0))) && (!(s.v[737] != 0.0))) && (s.v[738] != 0.0)) {
            s.store_scalar(441, 0.0);
        }

        if ((((s.v[418] != 0.0) && (!(s.v[735] != 0.0))) && (!(s.v[737] != 0.0))) && (!(s.v[738] != 0.0))) {
            s.store_scale_ad(441, A::add(A::div(A::mul(A::square(s.ad_value(440)), A::ln(s.ad_value(440))), A::sub_from_scalar(1.0, s.ad_value(440))), s.ad_value(440)), (1.0 - (2.0 * s.v[10])));
        }

        if (((s.v[418] != 0.0) && (!(s.v[735] != 0.0))) && (!(s.v[737] != 0.0))) {
            s.store_add(442, 440, 441);
        }

        s.v[739] = if (s.v[10] == 0.5) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (!(s.v[735] != 0.0))) && (!(s.v[737] != 0.0))) && (s.v[739] != 0.0)) {
            s.store_sqrt_ad(436, A::scale(s.ad_value(439), s.v[144]));
        }

        if ((((s.v[418] != 0.0) && (!(s.v[735] != 0.0))) && (!(s.v[737] != 0.0))) && (!(s.v[739] != 0.0))) {
            s.store_powf_ad(436, A::scale(s.ad_value(439), s.v[144]), s.v[10]);
        }

        if (((s.v[418] != 0.0) && (!(s.v[735] != 0.0))) && (!(s.v[737] != 0.0))) {
            s.store_scale(443, 436, s.v[138]);
        }

        if (((s.v[418] != 0.0) && (!(s.v[735] != 0.0))) && (!(s.v[737] != 0.0))) {
            s.store_mul_ad_rhs(444, 99, A::mul(A::offset(s.ad_value(430), (-1.0)), s.ad_value(443)));
        }

        if (((s.v[418] != 0.0) && (!(s.v[735] != 0.0))) && (!(s.v[737] != 0.0))) {
            s.store_scaled_mul(438, 444, 442, s.v[21]);
        }

        s.v[740] = if (s.v[24] == 0.0) { 1.0 } else { 0.0 };

        if (((s.v[418] != 0.0) && (!(s.v[735] != 0.0))) && (s.v[740] != 0.0)) {
            s.store_scalar(445, 0.0);
        }

        if (((s.v[418] != 0.0) && (!(s.v[735] != 0.0))) && (!(s.v[740] != 0.0))) {
            s.store_scale_ad(446, A::div(A::scale(s.ad_value(443), s.v[123]), s.ad_value(439)), s.v[153]);
        }

        if (((s.v[418] != 0.0) && (!(s.v[735] != 0.0))) && (!(s.v[740] != 0.0))) {
            s.store_div_from_scalar(447, (0.666666666666667 * s.v[150]), 446);
        }

        if (((s.v[418] != 0.0) && (!(s.v[735] != 0.0))) && (!(s.v[740] != 0.0))) {
            s.store_square(448, 447);
        }

        if (((s.v[418] != 0.0) && (!(s.v[735] != 0.0))) && (!(s.v[740] != 0.0))) {
            s.store_sqrt_ad(449, A::div(A::square(s.ad_value(448)), A::offset(A::square(s.ad_value(448)), 1.0)));
        }

        if (((s.v[418] != 0.0) && (!(s.v[735] != 0.0))) && (!(s.v[740] != 0.0))) {
            s.store_sqrt_ad(450, A::abs(s.ad_value(449)));
        }

        if (((s.v[418] != 0.0) && (!(s.v[735] != 0.0))) && (!(s.v[740] != 0.0))) {
            s.store_mul(451, 449, 450);
        }

        s.v[741] = if (((-s.v[10]) * s.v[126]) == (-1.0)) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (!(s.v[735] != 0.0))) && (!(s.v[740] != 0.0))) && (s.v[741] != 0.0)) {
            s.store_div_from_scalar_ad(452, 1.0, A::offset(A::mul(s.ad_value(446), s.ad_value(451)), 1.0));
        }

        if ((((s.v[418] != 0.0) && (!(s.v[735] != 0.0))) && (!(s.v[740] != 0.0))) && (!(s.v[741] != 0.0))) {
            s.store_powf_ad(452, A::offset(A::mul(s.ad_value(446), s.ad_value(451)), 1.0), ((-s.v[10]) * s.v[126]));
        }

        if (((s.v[418] != 0.0) && (!(s.v[735] != 0.0))) && (!(s.v[740] != 0.0))) {
            s.store_div_ad(453, A::mul(s.ad_value(442), s.ad_value(452)), A::add(s.ad_value(442), s.ad_value(452)));
        }

        if (((s.v[418] != 0.0) && (!(s.v[735] != 0.0))) && (!(s.v[740] != 0.0))) {
            s.store_sqrt_ad(454, A::scale(A::div(s.ad_value(446), s.ad_value(450)), 0.375));
        }

        if (((s.v[418] != 0.0) && (!(s.v[735] != 0.0))) && (!(s.v[740] != 0.0))) {
            s.store_sub_ad_lhs(455, A::scale(A::mul(s.ad_value(447), s.ad_value(450)), 2.0), 449);
        }

        if (((s.v[418] != 0.0) && (!(s.v[735] != 0.0))) && (!(s.v[740] != 0.0))) {
            s.store_add_ad(456, A::sub(A::mul(A::scale(s.ad_value(447), s.v[150]), s.ad_value(450)), A::scale(s.ad_value(449), s.v[150])), A::scale(A::mul(s.ad_value(446), s.ad_value(451)), 0.5));
        }

        if (((s.v[418] != 0.0) && (!(s.v[735] != 0.0))) && (!(s.v[740] != 0.0))) {
            s.store_mul_ad_lhs(457, A::offset(s.ad_value(455), (-1.0)), 454);
        }

        if (((s.v[418] != 0.0) && (!(s.v[735] != 0.0))) && (!(s.v[740] != 0.0))) {
            s.store_square(419, 457);
        }

        s.v[742] = if (s.v[457] > 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (!(s.v[735] != 0.0))) && (!(s.v[740] != 0.0))) && (s.v[742] != 0.0)) {
            s.store_div_from_scalar_ad(420, 1.0, A::offset(A::scale(s.ad_value(457), s.v[86]), 1.0));
        }

        if ((((s.v[418] != 0.0) && (!(s.v[735] != 0.0))) && (!(s.v[740] != 0.0))) && (!(s.v[742] != 0.0))) {
            s.store_div_from_scalar_ad(420, 1.0, A::sub_from_scalar(1.0, A::scale(s.ad_value(457), s.v[86])));
        }

        s.v[743] = if (((-s.v[419]) + s.v[456]) > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (!(s.v[735] != 0.0))) && (!(s.v[740] != 0.0))) && (s.v[743] != 0.0)) {
            s.store_exp_ad(436, A::sub(s.ad_value(456), s.ad_value(419)));
        }

        if ((((s.v[418] != 0.0) && (!(s.v[735] != 0.0))) && (!(s.v[740] != 0.0))) && (!(s.v[743] != 0.0))) {
            s.store_div_from_scalar_ad(436, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(456), s.ad_value(419))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(456), s.ad_value(419))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(456), s.ad_value(419))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if (((s.v[418] != 0.0) && (!(s.v[735] != 0.0))) && (!(s.v[740] != 0.0))) {
            s.store_mul_ad_lhs(421, A::add(A::add(A::scale(s.ad_value(420), 0.29214664), A::scale(A::square(s.ad_value(420)), s.v[87])), A::scale(A::mul(A::square(s.ad_value(420)), s.ad_value(420)), s.v[88])), 436);
        }

        s.v[744] = if (s.v[457] > 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (!(s.v[735] != 0.0))) && (!(s.v[740] != 0.0))) && (s.v[744] != 0.0)) {
            s.copy_ad(458, 421);
        }

        s.v[745] = if (s.v[456] > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((s.v[418] != 0.0) && (!(s.v[735] != 0.0))) && (!(s.v[740] != 0.0))) && (!(s.v[744] != 0.0))) && (s.v[745] != 0.0)) {
            s.store_exp(436, 456);
        }

        if (((((s.v[418] != 0.0) && (!(s.v[735] != 0.0))) && (!(s.v[740] != 0.0))) && (!(s.v[744] != 0.0))) && (!(s.v[745] != 0.0))) {
            s.store_div_from_scalar_ad(436, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(456)), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(456)), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), s.ad_value(456)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if ((((s.v[418] != 0.0) && (!(s.v[735] != 0.0))) && (!(s.v[740] != 0.0))) && (!(s.v[744] != 0.0))) {
            s.store_sub_ad_lhs(458, A::scale(s.ad_value(436), 2.0), 421);
        }

        if (((s.v[418] != 0.0) && (!(s.v[735] != 0.0))) && (!(s.v[740] != 0.0))) {
            s.store_scale_ad(459, A::div(A::scale(s.ad_value(458), s.v[150]), s.ad_value(454)), (1.772453850905516 * 0.5));
        }

        if (((s.v[418] != 0.0) && (!(s.v[735] != 0.0))) && (!(s.v[740] != 0.0))) {
            s.store_scale_ad(445, A::mul(A::mul(s.ad_value(444), s.ad_value(459)), s.ad_value(453)), s.v[24]);
        }

        s.v[746] = if (s.v[30] == 0.0) { 1.0 } else { 0.0 };

        if (((s.v[418] != 0.0) && (!(s.v[735] != 0.0))) && (s.v[746] != 0.0)) {
            s.store_scalar(460, 0.0);
        }

        s.v[747] = if (s.v[10] == 0.5) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (!(s.v[735] != 0.0))) && (!(s.v[746] != 0.0))) && (s.v[747] != 0.0)) {
            s.store_sqrt_ad(436, A::scale(A::sub_from_scalar(s.v[7], s.ad_value(434)), s.v[144]));
        }

        if ((((s.v[418] != 0.0) && (!(s.v[735] != 0.0))) && (!(s.v[746] != 0.0))) && (!(s.v[747] != 0.0))) {
            s.store_powf_ad(436, A::scale(A::sub_from_scalar(s.v[7], s.ad_value(434)), s.v[144]), s.v[10]);
        }

        if (((s.v[418] != 0.0) && (!(s.v[735] != 0.0))) && (!(s.v[746] != 0.0))) {
            s.store_scale_ad(461, A::div(A::scale(A::sub_from_scalar(s.v[7], s.ad_value(434)), s.v[141]), s.ad_value(436)), s.v[126]);
        }

        s.v[748] = if (((((-s.v[156]) / s.v[461])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (!(s.v[735] != 0.0))) && (!(s.v[746] != 0.0))) && (s.v[748] != 0.0)) {
            s.store_exp_ad(436, A::div(A::neg(s.ad_value(156)), s.ad_value(461)));
        }

        s.v[749] = if (((-s.v[156]) / s.v[461]) < (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((s.v[418] != 0.0) && (!(s.v[735] != 0.0))) && (!(s.v[746] != 0.0))) && (!(s.v[748] != 0.0))) && (s.v[749] != 0.0)) {
            let assign22460_ad_e32674: A = A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(156)), s.ad_value(461))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(156)), s.ad_value(461))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(156)), s.ad_value(461))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
            s.store_div_from_scalar_ad(436, 1e-100, assign22460_ad_e32674);
        }

        if (((((s.v[418] != 0.0) && (!(s.v[735] != 0.0))) && (!(s.v[746] != 0.0))) && (!(s.v[748] != 0.0))) && (!(s.v[749] != 0.0))) {
            let assign22470_ad_e32722: A = A::scale(A::offset(A::mul(A::offset(A::div(A::neg(s.ad_value(156)), s.ad_value(461)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::div(A::neg(s.ad_value(156)), s.ad_value(461)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::div(A::neg(s.ad_value(156)), s.ad_value(461)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
            s.store_ad(436, &assign22470_ad_e32722);
        }

        if (((s.v[418] != 0.0) && (!(s.v[735] != 0.0))) && (!(s.v[746] != 0.0))) {
            s.store_scale_ad(460, A::mul(A::mul(A::mul(s.ad_value(193), s.ad_value(461)), s.ad_value(461)), s.ad_value(436)), s.v[30]);
        }

        s.v[750] = if ((s.v[39] > 1000000.0) || (p.p80 == 0.0)) { 1.0 } else { 0.0 };

        if (((s.v[418] != 0.0) && (!(s.v[735] != 0.0))) && (s.v[750] != 0.0)) {
            s.store_scalar(462, 1.0);
        }

        s.v[751] = if (s.v[435] > ((-s.v[158]) * s.v[39])) { 1.0 } else { 0.0 };

        s.v[752] = if (s.v[42] == 4.0) { 1.0 } else { 0.0 };

        if (((((s.v[418] != 0.0) && (!(s.v[735] != 0.0))) && (!(s.v[750] != 0.0))) && (s.v[751] != 0.0)) && (s.v[752] != 0.0)) {
            s.store_mul_ad(436, A::mul(A::mul(A::abs(A::mul(s.ad_value(435), s.ad_value(163))), A::abs(A::mul(s.ad_value(435), s.ad_value(163)))), A::abs(A::mul(s.ad_value(435), s.ad_value(163)))), A::abs(A::mul(s.ad_value(435), s.ad_value(163))));
        }

        if (((((s.v[418] != 0.0) && (!(s.v[735] != 0.0))) && (!(s.v[750] != 0.0))) && (s.v[751] != 0.0)) && (!(s.v[752] != 0.0))) {
            s.store_powf_ad(436, A::abs(A::mul(s.ad_value(435), s.ad_value(163))), s.v[42]);
        }

        if ((((s.v[418] != 0.0) && (!(s.v[735] != 0.0))) && (!(s.v[750] != 0.0))) && (s.v[751] != 0.0)) {
            s.store_div_from_scalar_ad(462, 1.0, A::sub_from_scalar(1.0, s.ad_value(436)));
        }

        if ((((s.v[418] != 0.0) && (!(s.v[735] != 0.0))) && (!(s.v[750] != 0.0))) && (!(s.v[751] != 0.0))) {
            s.store_offset_ad(462, A::mul(A::add(s.ad_value(435), A::scale(s.ad_value(39), s.v[158])), s.ad_value(166)), s.v[160]);
        }

        if ((s.v[418] != 0.0) && (!(s.v[735] != 0.0))) {
            s.store_mul_ad_lhs(270, A::add(A::add(A::add(s.ad_value(437), s.ad_value(438)), s.ad_value(445)), s.ad_value(460)), 462);
        }

        if ((s.v[418] != 0.0) && (!(s.v[735] != 0.0))) {
            s.store_mul_ad_lhs(292, A::add(A::add(s.ad_value(438), s.ad_value(445)), s.ad_value(460)), 462);
        }

        s.v[753] = if (s.v[258] == 0.0) { 1.0 } else { 0.0 };

        if ((s.v[418] != 0.0) && (s.v[753] != 0.0)) {
            s.store_scalar(272, 0.0);
        }

        if ((s.v[418] != 0.0) && (s.v[753] != 0.0)) {
            s.store_scalar(293, 0.0);
        }

        if ((s.v[418] != 0.0) && (s.v[753] != 0.0)) {
            s.store_scalar(273, 0.0);
        }

        s.v[754] = if (s.v[124] == 0.5) { 1.0 } else { 0.0 };

        if (((s.v[418] != 0.0) && (!(s.v[753] != 0.0))) && (s.v[754] != 0.0)) {
            s.store_sqrt_ad(436, A::sub_from_scalar(1.0, A::mul(s.ad_value(428), s.ad_value(121))));
        }

        if (((s.v[418] != 0.0) && (!(s.v[753] != 0.0))) && (!(s.v[754] != 0.0))) {
            s.store_powf_ad(436, A::sub_from_scalar(1.0, A::mul(s.ad_value(428), s.ad_value(121))), s.v[124]);
        }

        if ((s.v[418] != 0.0) && (!(s.v[753] != 0.0))) {
            s.store_add_ad(273, A::mul(s.ad_value(133), A::sub_from_scalar(1.0, s.ad_value(436))), A::mul(s.ad_value(136), A::sub(s.ad_value(193), s.ad_value(428))));
        }

        if ((s.v[418] != 0.0) && (!(s.v[753] != 0.0))) {
            s.store_mul(437, 103, 372);
        }

        s.v[755] = if ((s.v[22] == 0.0) && (s.v[25] == 0.0)) { 1.0 } else { 0.0 };

        if (((s.v[418] != 0.0) && (!(s.v[753] != 0.0))) && (s.v[755] != 0.0)) {
            s.store_scalar(439, 0.0);
        }

        if (((s.v[418] != 0.0) && (!(s.v[753] != 0.0))) && (s.v[755] != 0.0)) {
            s.store_scalar(442, 0.0);
        }

        if (((s.v[418] != 0.0) && (!(s.v[753] != 0.0))) && (s.v[755] != 0.0)) {
            s.store_scalar(443, 0.0);
        }

        if (((s.v[418] != 0.0) && (!(s.v[753] != 0.0))) && (s.v[755] != 0.0)) {
            s.store_scalar(444, 0.0);
        }

        if (((s.v[418] != 0.0) && (!(s.v[753] != 0.0))) && (s.v[755] != 0.0)) {
            s.store_scalar(438, 0.0);
        }

        if (((s.v[418] != 0.0) && (!(s.v[753] != 0.0))) && (!(s.v[755] != 0.0))) {
            s.store_sub(439, 109, 433);
        }

        if (((s.v[418] != 0.0) && (!(s.v[753] != 0.0))) && (!(s.v[755] != 0.0))) {
            s.store_sub_from_scalar_ad(440, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(431), s.ad_value(439)))));
        }

        s.v[756] = if (s.v[11] == 0.5) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (!(s.v[753] != 0.0))) && (!(s.v[755] != 0.0))) && (s.v[756] != 0.0)) {
            s.store_scalar(441, 0.0);
        }

        if ((((s.v[418] != 0.0) && (!(s.v[753] != 0.0))) && (!(s.v[755] != 0.0))) && (!(s.v[756] != 0.0))) {
            s.store_scale_ad(441, A::add(A::div(A::mul(A::square(s.ad_value(440)), A::ln(s.ad_value(440))), A::sub_from_scalar(1.0, s.ad_value(440))), s.ad_value(440)), (1.0 - (2.0 * s.v[11])));
        }

        if (((s.v[418] != 0.0) && (!(s.v[753] != 0.0))) && (!(s.v[755] != 0.0))) {
            s.store_add(442, 440, 441);
        }

        s.v[757] = if (s.v[11] == 0.5) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (!(s.v[753] != 0.0))) && (!(s.v[755] != 0.0))) && (s.v[757] != 0.0)) {
            s.store_sqrt_ad(436, A::scale(s.ad_value(439), s.v[145]));
        }

        if ((((s.v[418] != 0.0) && (!(s.v[753] != 0.0))) && (!(s.v[755] != 0.0))) && (!(s.v[757] != 0.0))) {
            s.store_powf_ad(436, A::scale(s.ad_value(439), s.v[145]), s.v[11]);
        }

        if (((s.v[418] != 0.0) && (!(s.v[753] != 0.0))) && (!(s.v[755] != 0.0))) {
            s.store_scale(443, 436, s.v[139]);
        }

        if (((s.v[418] != 0.0) && (!(s.v[753] != 0.0))) && (!(s.v[755] != 0.0))) {
            s.store_mul_ad_rhs(444, 100, A::mul(A::offset(s.ad_value(430), (-1.0)), s.ad_value(443)));
        }

        if (((s.v[418] != 0.0) && (!(s.v[753] != 0.0))) && (!(s.v[755] != 0.0))) {
            s.store_scaled_mul(438, 444, 442, s.v[22]);
        }

        s.v[758] = if (s.v[25] == 0.0) { 1.0 } else { 0.0 };

        if (((s.v[418] != 0.0) && (!(s.v[753] != 0.0))) && (s.v[758] != 0.0)) {
            s.store_scalar(445, 0.0);
        }

        if (((s.v[418] != 0.0) && (!(s.v[753] != 0.0))) && (!(s.v[758] != 0.0))) {
            s.store_scale_ad(446, A::div(A::scale(s.ad_value(443), s.v[124]), s.ad_value(439)), s.v[154]);
        }

        if (((s.v[418] != 0.0) && (!(s.v[753] != 0.0))) && (!(s.v[758] != 0.0))) {
            s.store_div_from_scalar(447, (0.666666666666667 * s.v[151]), 446);
        }

        if (((s.v[418] != 0.0) && (!(s.v[753] != 0.0))) && (!(s.v[758] != 0.0))) {
            s.store_square(448, 447);
        }

        if (((s.v[418] != 0.0) && (!(s.v[753] != 0.0))) && (!(s.v[758] != 0.0))) {
            s.store_sqrt_ad(449, A::div(A::square(s.ad_value(448)), A::offset(A::square(s.ad_value(448)), 1.0)));
        }

        if (((s.v[418] != 0.0) && (!(s.v[753] != 0.0))) && (!(s.v[758] != 0.0))) {
            s.store_sqrt_ad(450, A::abs(s.ad_value(449)));
        }

        if (((s.v[418] != 0.0) && (!(s.v[753] != 0.0))) && (!(s.v[758] != 0.0))) {
            s.store_mul(451, 449, 450);
        }

        s.v[759] = if (((-s.v[11]) * s.v[127]) == (-1.0)) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (!(s.v[753] != 0.0))) && (!(s.v[758] != 0.0))) && (s.v[759] != 0.0)) {
            s.store_div_from_scalar_ad(452, 1.0, A::offset(A::mul(s.ad_value(446), s.ad_value(451)), 1.0));
        }

        if ((((s.v[418] != 0.0) && (!(s.v[753] != 0.0))) && (!(s.v[758] != 0.0))) && (!(s.v[759] != 0.0))) {
            s.store_powf_ad(452, A::offset(A::mul(s.ad_value(446), s.ad_value(451)), 1.0), ((-s.v[11]) * s.v[127]));
        }

        if (((s.v[418] != 0.0) && (!(s.v[753] != 0.0))) && (!(s.v[758] != 0.0))) {
            s.store_div_ad(453, A::mul(s.ad_value(442), s.ad_value(452)), A::add(s.ad_value(442), s.ad_value(452)));
        }

        if (((s.v[418] != 0.0) && (!(s.v[753] != 0.0))) && (!(s.v[758] != 0.0))) {
            s.store_sqrt_ad(454, A::scale(A::div(s.ad_value(446), s.ad_value(450)), 0.375));
        }

        if (((s.v[418] != 0.0) && (!(s.v[753] != 0.0))) && (!(s.v[758] != 0.0))) {
            s.store_sub_ad_lhs(455, A::scale(A::mul(s.ad_value(447), s.ad_value(450)), 2.0), 449);
        }

        if (((s.v[418] != 0.0) && (!(s.v[753] != 0.0))) && (!(s.v[758] != 0.0))) {
            s.store_add_ad(456, A::sub(A::mul(A::scale(s.ad_value(447), s.v[151]), s.ad_value(450)), A::scale(s.ad_value(449), s.v[151])), A::scale(A::mul(s.ad_value(446), s.ad_value(451)), 0.5));
        }

        if (((s.v[418] != 0.0) && (!(s.v[753] != 0.0))) && (!(s.v[758] != 0.0))) {
            s.store_mul_ad_lhs(457, A::offset(s.ad_value(455), (-1.0)), 454);
        }

        if (((s.v[418] != 0.0) && (!(s.v[753] != 0.0))) && (!(s.v[758] != 0.0))) {
            s.store_square(419, 457);
        }

        s.v[760] = if (s.v[457] > 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (!(s.v[753] != 0.0))) && (!(s.v[758] != 0.0))) && (s.v[760] != 0.0)) {
            s.store_div_from_scalar_ad(420, 1.0, A::offset(A::scale(s.ad_value(457), s.v[86]), 1.0));
        }

        if ((((s.v[418] != 0.0) && (!(s.v[753] != 0.0))) && (!(s.v[758] != 0.0))) && (!(s.v[760] != 0.0))) {
            s.store_div_from_scalar_ad(420, 1.0, A::sub_from_scalar(1.0, A::scale(s.ad_value(457), s.v[86])));
        }

        s.v[761] = if (((-s.v[419]) + s.v[456]) > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (!(s.v[753] != 0.0))) && (!(s.v[758] != 0.0))) && (s.v[761] != 0.0)) {
            s.store_exp_ad(436, A::sub(s.ad_value(456), s.ad_value(419)));
        }

        if ((((s.v[418] != 0.0) && (!(s.v[753] != 0.0))) && (!(s.v[758] != 0.0))) && (!(s.v[761] != 0.0))) {
            s.store_div_from_scalar_ad(436, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(456), s.ad_value(419))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(456), s.ad_value(419))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(456), s.ad_value(419))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if (((s.v[418] != 0.0) && (!(s.v[753] != 0.0))) && (!(s.v[758] != 0.0))) {
            s.store_mul_ad_lhs(421, A::add(A::add(A::scale(s.ad_value(420), 0.29214664), A::scale(A::square(s.ad_value(420)), s.v[87])), A::scale(A::mul(A::square(s.ad_value(420)), s.ad_value(420)), s.v[88])), 436);
        }

        s.v[762] = if (s.v[457] > 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (!(s.v[753] != 0.0))) && (!(s.v[758] != 0.0))) && (s.v[762] != 0.0)) {
            s.copy_ad(458, 421);
        }

        s.v[763] = if (s.v[456] > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((s.v[418] != 0.0) && (!(s.v[753] != 0.0))) && (!(s.v[758] != 0.0))) && (!(s.v[762] != 0.0))) && (s.v[763] != 0.0)) {
            s.store_exp(436, 456);
        }

        if (((((s.v[418] != 0.0) && (!(s.v[753] != 0.0))) && (!(s.v[758] != 0.0))) && (!(s.v[762] != 0.0))) && (!(s.v[763] != 0.0))) {
            s.store_div_from_scalar_ad(436, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(456)), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(456)), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), s.ad_value(456)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if ((((s.v[418] != 0.0) && (!(s.v[753] != 0.0))) && (!(s.v[758] != 0.0))) && (!(s.v[762] != 0.0))) {
            s.store_sub_ad_lhs(458, A::scale(s.ad_value(436), 2.0), 421);
        }

        if (((s.v[418] != 0.0) && (!(s.v[753] != 0.0))) && (!(s.v[758] != 0.0))) {
            s.store_scale_ad(459, A::div(A::scale(s.ad_value(458), s.v[151]), s.ad_value(454)), (1.772453850905516 * 0.5));
        }

        if (((s.v[418] != 0.0) && (!(s.v[753] != 0.0))) && (!(s.v[758] != 0.0))) {
            s.store_scale_ad(445, A::mul(A::mul(s.ad_value(444), s.ad_value(459)), s.ad_value(453)), s.v[25]);
        }

        s.v[764] = if (s.v[31] == 0.0) { 1.0 } else { 0.0 };

        if (((s.v[418] != 0.0) && (!(s.v[753] != 0.0))) && (s.v[764] != 0.0)) {
            s.store_scalar(460, 0.0);
        }

        s.v[765] = if (s.v[11] == 0.5) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (!(s.v[753] != 0.0))) && (!(s.v[764] != 0.0))) && (s.v[765] != 0.0)) {
            s.store_sqrt_ad(436, A::scale(A::sub_from_scalar(s.v[8], s.ad_value(434)), s.v[145]));
        }

        if ((((s.v[418] != 0.0) && (!(s.v[753] != 0.0))) && (!(s.v[764] != 0.0))) && (!(s.v[765] != 0.0))) {
            s.store_powf_ad(436, A::scale(A::sub_from_scalar(s.v[8], s.ad_value(434)), s.v[145]), s.v[11]);
        }

        if (((s.v[418] != 0.0) && (!(s.v[753] != 0.0))) && (!(s.v[764] != 0.0))) {
            s.store_scale_ad(461, A::div(A::scale(A::sub_from_scalar(s.v[8], s.ad_value(434)), s.v[142]), s.ad_value(436)), s.v[127]);
        }

        s.v[766] = if (((((-s.v[157]) / s.v[461])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (!(s.v[753] != 0.0))) && (!(s.v[764] != 0.0))) && (s.v[766] != 0.0)) {
            s.store_exp_ad(436, A::div(A::neg(s.ad_value(157)), s.ad_value(461)));
        }

        s.v[767] = if (((-s.v[157]) / s.v[461]) < (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((s.v[418] != 0.0) && (!(s.v[753] != 0.0))) && (!(s.v[764] != 0.0))) && (!(s.v[766] != 0.0))) && (s.v[767] != 0.0)) {
            let assign23270_ad_e33830: A = A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(157)), s.ad_value(461))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(157)), s.ad_value(461))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(157)), s.ad_value(461))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
            s.store_div_from_scalar_ad(436, 1e-100, assign23270_ad_e33830);
        }

    }

    pub(super) fn stamp_reactive_block_17(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if (((((s.v[418] != 0.0) && (!(s.v[753] != 0.0))) && (!(s.v[764] != 0.0))) && (!(s.v[766] != 0.0))) && (!(s.v[767] != 0.0))) {
            let assign23280_ad_e33878: A = A::scale(A::offset(A::mul(A::offset(A::div(A::neg(s.ad_value(157)), s.ad_value(461)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::div(A::neg(s.ad_value(157)), s.ad_value(461)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::div(A::neg(s.ad_value(157)), s.ad_value(461)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
            s.store_ad(436, &assign23280_ad_e33878);
        }

        if (((s.v[418] != 0.0) && (!(s.v[753] != 0.0))) && (!(s.v[764] != 0.0))) {
            s.store_scale_ad(460, A::mul(A::mul(A::mul(s.ad_value(193), s.ad_value(461)), s.ad_value(461)), s.ad_value(436)), s.v[31]);
        }

        s.v[768] = if ((s.v[40] > 1000000.0) || (p.p80 == 0.0)) { 1.0 } else { 0.0 };

        if (((s.v[418] != 0.0) && (!(s.v[753] != 0.0))) && (s.v[768] != 0.0)) {
            s.store_scalar(462, 1.0);
        }

        s.v[769] = if (s.v[435] > ((-s.v[158]) * s.v[40])) { 1.0 } else { 0.0 };

        s.v[770] = if (s.v[43] == 4.0) { 1.0 } else { 0.0 };

        if (((((s.v[418] != 0.0) && (!(s.v[753] != 0.0))) && (!(s.v[768] != 0.0))) && (s.v[769] != 0.0)) && (s.v[770] != 0.0)) {
            s.store_mul_ad(436, A::mul(A::mul(A::abs(A::mul(s.ad_value(435), s.ad_value(164))), A::abs(A::mul(s.ad_value(435), s.ad_value(164)))), A::abs(A::mul(s.ad_value(435), s.ad_value(164)))), A::abs(A::mul(s.ad_value(435), s.ad_value(164))));
        }

        if (((((s.v[418] != 0.0) && (!(s.v[753] != 0.0))) && (!(s.v[768] != 0.0))) && (s.v[769] != 0.0)) && (!(s.v[770] != 0.0))) {
            s.store_powf_ad(436, A::abs(A::mul(s.ad_value(435), s.ad_value(164))), s.v[43]);
        }

        if ((((s.v[418] != 0.0) && (!(s.v[753] != 0.0))) && (!(s.v[768] != 0.0))) && (s.v[769] != 0.0)) {
            s.store_div_from_scalar_ad(462, 1.0, A::sub_from_scalar(1.0, s.ad_value(436)));
        }

        if ((((s.v[418] != 0.0) && (!(s.v[753] != 0.0))) && (!(s.v[768] != 0.0))) && (!(s.v[769] != 0.0))) {
            s.store_offset_ad(462, A::mul(A::add(s.ad_value(435), A::scale(s.ad_value(40), s.v[158])), s.ad_value(167)), s.v[161]);
        }

        if ((s.v[418] != 0.0) && (!(s.v[753] != 0.0))) {
            s.store_mul_ad_lhs(272, A::add(A::add(A::add(s.ad_value(437), s.ad_value(438)), s.ad_value(445)), s.ad_value(460)), 462);
        }

        if ((s.v[418] != 0.0) && (!(s.v[753] != 0.0))) {
            s.store_mul_ad_lhs(293, A::add(A::add(s.ad_value(438), s.ad_value(445)), s.ad_value(460)), 462);
        }

        if (s.v[418] != 0.0) {
            s.store_add_ad(183, A::add(A::scale(s.ad_value(268), s.v[256]), A::scale(s.ad_value(270), s.v[257])), A::scale(s.ad_value(272), s.v[258]));
        }

        s.v[771] = if !(((s.v[256] == 0.0) && (s.v[257] == 0.0)) && (s.v[258] == 0.0)) { 1.0 } else { 0.0 };

        if ((s.v[418] != 0.0) && (s.v[771] != 0.0)) {
            s.store_mul_ad_lhs(422, A::scale(s.ad_value(265), 4.0), 265);
        }

        if ((s.v[418] != 0.0) && (s.v[771] != 0.0)) {
            s.store_div(423, 265, 266);
        }

        if ((s.v[418] != 0.0) && (s.v[771] != 0.0)) {
            s.store_add_ad_rhs(424, 194, A::mul(s.ad_value(265), s.ad_value(423)));
        }

        if ((s.v[418] != 0.0) && (s.v[771] != 0.0)) {
            s.store_add(425, 266, 424);
        }

        if ((s.v[418] != 0.0) && (s.v[771] != 0.0)) {
            s.store_sub(426, 266, 424);
        }

        if ((s.v[418] != 0.0) && (s.v[771] != 0.0)) {
            s.store_sqrt_ad(427, A::add(A::square(s.ad_value(426)), s.ad_value(422)));
        }

        if ((s.v[418] != 0.0) && (s.v[771] != 0.0)) {
            s.store_scale_ad(428, A::div(A::mul(s.ad_value(194), s.ad_value(266)), A::add(s.ad_value(425), s.ad_value(427))), 2.0);
        }

        s.v[772] = if (s.v[194] < s.v[262]) { 1.0 } else { 0.0 };

        s.v[773] = if ((((0.5 * (s.v[194] * s.v[85]))) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (s.v[771] != 0.0)) && (s.v[772] != 0.0)) && (s.v[773] != 0.0)) {
            s.store_exp_ad(430, A::scale(s.ad_value(194), (s.v[85] * 0.5)));
        }

        s.v[774] = if ((0.5 * (s.v[194] * s.v[85])) < (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((s.v[418] != 0.0) && (s.v[771] != 0.0)) && (s.v[772] != 0.0)) && (!(s.v[773] != 0.0))) && (s.v[774] != 0.0)) {
            let assign23530_ad_e34217: A = A::div_from_scalar(1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(s.ad_value(194), (s.v[85] * 0.5))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(s.ad_value(194), (s.v[85] * 0.5))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::scale(s.ad_value(194), (s.v[85] * 0.5))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
            s.store_ad(430, &assign23530_ad_e34217);
        }

        if (((((s.v[418] != 0.0) && (s.v[771] != 0.0)) && (s.v[772] != 0.0)) && (!(s.v[773] != 0.0))) && (!(s.v[774] != 0.0))) {
            s.store_scale_ad(430, A::offset(A::mul(A::offset(A::scale(s.ad_value(194), (s.v[85] * 0.5)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::scale(s.ad_value(194), (s.v[85] * 0.5)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::scale(s.ad_value(194), (s.v[85] * 0.5)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
        }

        if (((s.v[418] != 0.0) && (s.v[771] != 0.0)) && (s.v[772] != 0.0)) {
            s.store_scale_ad(363, A::square(s.ad_value(318)), 1.0 / (s.v[308]));
        }

        if (((s.v[418] != 0.0) && (s.v[771] != 0.0)) && (s.v[772] != 0.0)) {
            s.store_scale_ad(362, A::ln(A::div_from_scalar(s.v[308], s.ad_value(363))), (s.v[62] / s.v[85]));
        }

        s.v[775] = if (s.v[62] < p.p85) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (s.v[771] != 0.0)) && (s.v[772] != 0.0)) && (s.v[775] != 0.0)) {
            s.store_offset_ad(360, A::scale(A::sub(s.ad_value(194), s.ad_value(362)), p.p86), s.v[62]);
        }

        if ((((s.v[418] != 0.0) && (s.v[771] != 0.0)) && (s.v[772] != 0.0)) && (s.v[775] != 0.0)) {
            s.store_sub_from_scalar_ad(350, s.v[62], A::scale(s.ad_value(362), p.p86));
        }

        if ((((s.v[418] != 0.0) && (s.v[771] != 0.0)) && (s.v[772] != 0.0)) && (s.v[775] != 0.0)) {
            s.store_offset_ad(314, A::sub_from_scalar(p.p85, s.ad_value(360)), (-0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[771] != 0.0)) && (s.v[772] != 0.0)) && (s.v[775] != 0.0)) {
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[771] != 0.0)) && (s.v[772] != 0.0)) && (s.v[775] != 0.0)) {
            s.store_ad(315, &{
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((s.v[418] != 0.0) && (s.v[771] != 0.0)) && (s.v[772] != 0.0)) && (s.v[775] != 0.0)) {
            s.store_sqrt_ad(315, A::add(A::square(s.ad_value(314)), s.ad_value(315)));
        }

        if ((((s.v[418] != 0.0) && (s.v[771] != 0.0)) && (s.v[772] != 0.0)) && (s.v[775] != 0.0)) {
            s.store_sub_from_scalar_ad(361, p.p85, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5));
        }

        if ((((s.v[418] != 0.0) && (s.v[771] != 0.0)) && (s.v[772] != 0.0)) && (s.v[775] != 0.0)) {
            s.store_offset(314, 361, (((-s.v[62])) + ((-0.01))));
        }

        if ((((s.v[418] != 0.0) && (s.v[771] != 0.0)) && (s.v[772] != 0.0)) && (s.v[775] != 0.0)) {
            s.store_scalar(315, ((4.0 * s.v[62]) * 0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[771] != 0.0)) && (s.v[772] != 0.0)) && (s.v[775] != 0.0)) {
            s.store_ad(315, &{
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((s.v[418] != 0.0) && (s.v[771] != 0.0)) && (s.v[772] != 0.0)) && (s.v[775] != 0.0)) {
            s.store_sqrt_ad(315, A::add(A::square(s.ad_value(314)), s.ad_value(315)));
        }

        if ((((s.v[418] != 0.0) && (s.v[771] != 0.0)) && (s.v[772] != 0.0)) && (s.v[775] != 0.0)) {
            s.store_offset_ad(359, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5), s.v[62]);
        }

        if ((((s.v[418] != 0.0) && (s.v[771] != 0.0)) && (s.v[772] != 0.0)) && (s.v[775] != 0.0)) {
            s.store_offset_ad(314, A::sub_from_scalar(p.p85, s.ad_value(350)), (-0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[771] != 0.0)) && (s.v[772] != 0.0)) && (s.v[775] != 0.0)) {
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[771] != 0.0)) && (s.v[772] != 0.0)) && (s.v[775] != 0.0)) {
            s.store_ad(315, &{
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((s.v[418] != 0.0) && (s.v[771] != 0.0)) && (s.v[772] != 0.0)) && (s.v[775] != 0.0)) {
            s.store_sqrt_ad(315, A::add(A::square(s.ad_value(314)), s.ad_value(315)));
        }

        if ((((s.v[418] != 0.0) && (s.v[771] != 0.0)) && (s.v[772] != 0.0)) && (s.v[775] != 0.0)) {
            s.store_sub_from_scalar_ad(350, p.p85, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5));
        }

        if ((((s.v[418] != 0.0) && (s.v[771] != 0.0)) && (s.v[772] != 0.0)) && (s.v[775] != 0.0)) {
            s.store_offset(314, 350, (((-s.v[62])) + ((-0.01))));
        }

        if ((((s.v[418] != 0.0) && (s.v[771] != 0.0)) && (s.v[772] != 0.0)) && (s.v[775] != 0.0)) {
            s.store_scalar(315, ((4.0 * s.v[62]) * 0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[771] != 0.0)) && (s.v[772] != 0.0)) && (s.v[775] != 0.0)) {
            s.store_ad(315, &{
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((s.v[418] != 0.0) && (s.v[771] != 0.0)) && (s.v[772] != 0.0)) && (s.v[775] != 0.0)) {
            s.store_sqrt_ad(315, A::add(A::square(s.ad_value(314)), s.ad_value(315)));
        }

        if ((((s.v[418] != 0.0) && (s.v[771] != 0.0)) && (s.v[772] != 0.0)) && (s.v[775] != 0.0)) {
            s.store_offset_ad(350, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5), s.v[62]);
        }

        if ((((s.v[418] != 0.0) && (s.v[771] != 0.0)) && (s.v[772] != 0.0)) && (!(s.v[775] != 0.0))) {
            s.store_scalar(350, s.v[62]);
        }

        if ((((s.v[418] != 0.0) && (s.v[771] != 0.0)) && (s.v[772] != 0.0)) && (!(s.v[775] != 0.0))) {
            s.store_scalar(359, s.v[62]);
        }

        s.v[776] = if ((((s.v[85] * ((s.v[194] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85))))) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (s.v[771] != 0.0)) && (s.v[772] != 0.0)) && (s.v[776] != 0.0)) {
            s.store_exp_ad(370, A::scale(A::add(A::div(s.ad_value(194), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]));
        }

        s.v[777] = if ((s.v[85] * ((s.v[194] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85)))) < (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((s.v[418] != 0.0) && (s.v[771] != 0.0)) && (s.v[772] != 0.0)) && (!(s.v[776] != 0.0))) && (s.v[777] != 0.0)) {
            let assign23850_ad_e34783: A = A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(A::add(A::div(s.ad_value(194), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85])), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::scale(A::add(A::div(s.ad_value(194), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85])), 0.3333333333333333), 1.0));
            s.store_div_from_scalar_ad(370, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(A::add(A::div(s.ad_value(194), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85])), A::offset(A::scale(assign23850_ad_e34783, 0.5), 1.0)), 1.0));
        }

        if (((((s.v[418] != 0.0) && (s.v[771] != 0.0)) && (s.v[772] != 0.0)) && (!(s.v[776] != 0.0))) && (!(s.v[777] != 0.0))) {
            let assign23860_ad_e34861: A = A::mul(A::offset(A::scale(A::add(A::div(s.ad_value(194), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]), (-230.25850929940458)), A::offset(A::scale(A::offset(A::scale(A::add(A::div(s.ad_value(194), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]), (-230.25850929940458)), 0.3333333333333333), 1.0));
            s.store_scale_ad(370, A::offset(A::mul(A::offset(A::scale(A::add(A::div(s.ad_value(194), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]), (-230.25850929940458)), A::offset(A::scale(assign23860_ad_e34861, 0.5), 1.0)), 1.0), 1e100);
        }

        if (((s.v[418] != 0.0) && (s.v[771] != 0.0)) && (s.v[772] != 0.0)) {
            s.store_scale_ad(363, A::square(s.ad_value(318)), 1.0 / (s.v[310]));
        }

        if (((s.v[418] != 0.0) && (s.v[771] != 0.0)) && (s.v[772] != 0.0)) {
            s.store_scale_ad(362, A::ln(A::div_from_scalar(s.v[310], s.ad_value(363))), (s.v[64] / s.v[85]));
        }

        s.v[778] = if (s.v[64] < p.p85) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (s.v[771] != 0.0)) && (s.v[772] != 0.0)) && (s.v[778] != 0.0)) {
            s.store_offset_ad(360, A::scale(A::sub(s.ad_value(194), s.ad_value(362)), p.p86), s.v[64]);
        }

        if ((((s.v[418] != 0.0) && (s.v[771] != 0.0)) && (s.v[772] != 0.0)) && (s.v[778] != 0.0)) {
            s.store_sub_from_scalar_ad(350, s.v[64], A::scale(s.ad_value(362), p.p86));
        }

        if ((((s.v[418] != 0.0) && (s.v[771] != 0.0)) && (s.v[772] != 0.0)) && (s.v[778] != 0.0)) {
            s.store_offset_ad(314, A::sub_from_scalar(p.p85, s.ad_value(360)), (-0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[771] != 0.0)) && (s.v[772] != 0.0)) && (s.v[778] != 0.0)) {
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[771] != 0.0)) && (s.v[772] != 0.0)) && (s.v[778] != 0.0)) {
            s.store_ad(315, &{
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((s.v[418] != 0.0) && (s.v[771] != 0.0)) && (s.v[772] != 0.0)) && (s.v[778] != 0.0)) {
            s.store_sqrt_ad(315, A::add(A::square(s.ad_value(314)), s.ad_value(315)));
        }

        if ((((s.v[418] != 0.0) && (s.v[771] != 0.0)) && (s.v[772] != 0.0)) && (s.v[778] != 0.0)) {
            s.store_sub_from_scalar_ad(361, p.p85, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5));
        }

        if ((((s.v[418] != 0.0) && (s.v[771] != 0.0)) && (s.v[772] != 0.0)) && (s.v[778] != 0.0)) {
            s.store_offset(314, 361, (((-s.v[64])) + ((-0.01))));
        }

        if ((((s.v[418] != 0.0) && (s.v[771] != 0.0)) && (s.v[772] != 0.0)) && (s.v[778] != 0.0)) {
            s.store_scalar(315, ((4.0 * s.v[64]) * 0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[771] != 0.0)) && (s.v[772] != 0.0)) && (s.v[778] != 0.0)) {
            s.store_ad(315, &{
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((s.v[418] != 0.0) && (s.v[771] != 0.0)) && (s.v[772] != 0.0)) && (s.v[778] != 0.0)) {
            s.store_sqrt_ad(315, A::add(A::square(s.ad_value(314)), s.ad_value(315)));
        }

        if ((((s.v[418] != 0.0) && (s.v[771] != 0.0)) && (s.v[772] != 0.0)) && (s.v[778] != 0.0)) {
            s.store_offset_ad(359, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5), s.v[64]);
        }

        if ((((s.v[418] != 0.0) && (s.v[771] != 0.0)) && (s.v[772] != 0.0)) && (s.v[778] != 0.0)) {
            s.store_offset_ad(314, A::sub_from_scalar(p.p85, s.ad_value(350)), (-0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[771] != 0.0)) && (s.v[772] != 0.0)) && (s.v[778] != 0.0)) {
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[771] != 0.0)) && (s.v[772] != 0.0)) && (s.v[778] != 0.0)) {
            s.store_ad(315, &{
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((s.v[418] != 0.0) && (s.v[771] != 0.0)) && (s.v[772] != 0.0)) && (s.v[778] != 0.0)) {
            s.store_sqrt_ad(315, A::add(A::square(s.ad_value(314)), s.ad_value(315)));
        }

        if ((((s.v[418] != 0.0) && (s.v[771] != 0.0)) && (s.v[772] != 0.0)) && (s.v[778] != 0.0)) {
            s.store_sub_from_scalar_ad(350, p.p85, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5));
        }

        if ((((s.v[418] != 0.0) && (s.v[771] != 0.0)) && (s.v[772] != 0.0)) && (s.v[778] != 0.0)) {
            s.store_offset(314, 350, (((-s.v[64])) + ((-0.01))));
        }

        if ((((s.v[418] != 0.0) && (s.v[771] != 0.0)) && (s.v[772] != 0.0)) && (s.v[778] != 0.0)) {
            s.store_scalar(315, ((4.0 * s.v[64]) * 0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[771] != 0.0)) && (s.v[772] != 0.0)) && (s.v[778] != 0.0)) {
            s.store_ad(315, &{
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((s.v[418] != 0.0) && (s.v[771] != 0.0)) && (s.v[772] != 0.0)) && (s.v[778] != 0.0)) {
            s.store_sqrt_ad(315, A::add(A::square(s.ad_value(314)), s.ad_value(315)));
        }

        if ((((s.v[418] != 0.0) && (s.v[771] != 0.0)) && (s.v[772] != 0.0)) && (s.v[778] != 0.0)) {
            s.store_offset_ad(350, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5), s.v[64]);
        }

        if ((((s.v[418] != 0.0) && (s.v[771] != 0.0)) && (s.v[772] != 0.0)) && (!(s.v[778] != 0.0))) {
            s.store_scalar(350, s.v[64]);
        }

        if ((((s.v[418] != 0.0) && (s.v[771] != 0.0)) && (s.v[772] != 0.0)) && (!(s.v[778] != 0.0))) {
            s.store_scalar(359, s.v[64]);
        }

        s.v[779] = if ((((s.v[85] * ((s.v[194] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85))))) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (s.v[771] != 0.0)) && (s.v[772] != 0.0)) && (s.v[779] != 0.0)) {
            s.store_exp_ad(371, A::scale(A::add(A::div(s.ad_value(194), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]));
        }

        s.v[780] = if ((s.v[85] * ((s.v[194] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85)))) < (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((s.v[418] != 0.0) && (s.v[771] != 0.0)) && (s.v[772] != 0.0)) && (!(s.v[779] != 0.0))) && (s.v[780] != 0.0)) {
            let assign24170_ad_e35384: A = A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(A::add(A::div(s.ad_value(194), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85])), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::scale(A::add(A::div(s.ad_value(194), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85])), 0.3333333333333333), 1.0));
            s.store_div_from_scalar_ad(371, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(A::add(A::div(s.ad_value(194), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85])), A::offset(A::scale(assign24170_ad_e35384, 0.5), 1.0)), 1.0));
        }

        if (((((s.v[418] != 0.0) && (s.v[771] != 0.0)) && (s.v[772] != 0.0)) && (!(s.v[779] != 0.0))) && (!(s.v[780] != 0.0))) {
            let assign24180_ad_e35462: A = A::mul(A::offset(A::scale(A::add(A::div(s.ad_value(194), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]), (-230.25850929940458)), A::offset(A::scale(A::offset(A::scale(A::add(A::div(s.ad_value(194), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]), (-230.25850929940458)), 0.3333333333333333), 1.0));
            s.store_scale_ad(371, A::offset(A::mul(A::offset(A::scale(A::add(A::div(s.ad_value(194), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]), (-230.25850929940458)), A::offset(A::scale(assign24180_ad_e35462, 0.5), 1.0)), 1.0), 1e100);
        }

        if (((s.v[418] != 0.0) && (s.v[771] != 0.0)) && (s.v[772] != 0.0)) {
            s.store_scale_ad(363, A::square(s.ad_value(318)), 1.0 / (s.v[309]));
        }

        if (((s.v[418] != 0.0) && (s.v[771] != 0.0)) && (s.v[772] != 0.0)) {
            s.store_scale_ad(362, A::ln(A::div_from_scalar(s.v[309], s.ad_value(363))), (s.v[63] / s.v[85]));
        }

        s.v[781] = if (s.v[63] < p.p85) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (s.v[771] != 0.0)) && (s.v[772] != 0.0)) && (s.v[781] != 0.0)) {
            s.store_offset_ad(360, A::scale(A::sub(s.ad_value(194), s.ad_value(362)), p.p86), s.v[63]);
        }

        if ((((s.v[418] != 0.0) && (s.v[771] != 0.0)) && (s.v[772] != 0.0)) && (s.v[781] != 0.0)) {
            s.store_sub_from_scalar_ad(350, s.v[63], A::scale(s.ad_value(362), p.p86));
        }

        if ((((s.v[418] != 0.0) && (s.v[771] != 0.0)) && (s.v[772] != 0.0)) && (s.v[781] != 0.0)) {
            s.store_offset_ad(314, A::sub_from_scalar(p.p85, s.ad_value(360)), (-0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[771] != 0.0)) && (s.v[772] != 0.0)) && (s.v[781] != 0.0)) {
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[771] != 0.0)) && (s.v[772] != 0.0)) && (s.v[781] != 0.0)) {
            s.store_ad(315, &{
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((s.v[418] != 0.0) && (s.v[771] != 0.0)) && (s.v[772] != 0.0)) && (s.v[781] != 0.0)) {
            s.store_sqrt_ad(315, A::add(A::square(s.ad_value(314)), s.ad_value(315)));
        }

        if ((((s.v[418] != 0.0) && (s.v[771] != 0.0)) && (s.v[772] != 0.0)) && (s.v[781] != 0.0)) {
            s.store_sub_from_scalar_ad(361, p.p85, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5));
        }

        if ((((s.v[418] != 0.0) && (s.v[771] != 0.0)) && (s.v[772] != 0.0)) && (s.v[781] != 0.0)) {
            s.store_offset(314, 361, (((-s.v[63])) + ((-0.01))));
        }

        if ((((s.v[418] != 0.0) && (s.v[771] != 0.0)) && (s.v[772] != 0.0)) && (s.v[781] != 0.0)) {
            s.store_scalar(315, ((4.0 * s.v[63]) * 0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[771] != 0.0)) && (s.v[772] != 0.0)) && (s.v[781] != 0.0)) {
            s.store_ad(315, &{
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((s.v[418] != 0.0) && (s.v[771] != 0.0)) && (s.v[772] != 0.0)) && (s.v[781] != 0.0)) {
            s.store_sqrt_ad(315, A::add(A::square(s.ad_value(314)), s.ad_value(315)));
        }

        if ((((s.v[418] != 0.0) && (s.v[771] != 0.0)) && (s.v[772] != 0.0)) && (s.v[781] != 0.0)) {
            s.store_offset_ad(359, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5), s.v[63]);
        }

        if ((((s.v[418] != 0.0) && (s.v[771] != 0.0)) && (s.v[772] != 0.0)) && (s.v[781] != 0.0)) {
            s.store_offset_ad(314, A::sub_from_scalar(p.p85, s.ad_value(350)), (-0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[771] != 0.0)) && (s.v[772] != 0.0)) && (s.v[781] != 0.0)) {
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[771] != 0.0)) && (s.v[772] != 0.0)) && (s.v[781] != 0.0)) {
            s.store_ad(315, &{
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((s.v[418] != 0.0) && (s.v[771] != 0.0)) && (s.v[772] != 0.0)) && (s.v[781] != 0.0)) {
            s.store_sqrt_ad(315, A::add(A::square(s.ad_value(314)), s.ad_value(315)));
        }

        if ((((s.v[418] != 0.0) && (s.v[771] != 0.0)) && (s.v[772] != 0.0)) && (s.v[781] != 0.0)) {
            s.store_sub_from_scalar_ad(350, p.p85, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5));
        }

        if ((((s.v[418] != 0.0) && (s.v[771] != 0.0)) && (s.v[772] != 0.0)) && (s.v[781] != 0.0)) {
            s.store_offset(314, 350, (((-s.v[63])) + ((-0.01))));
        }

        if ((((s.v[418] != 0.0) && (s.v[771] != 0.0)) && (s.v[772] != 0.0)) && (s.v[781] != 0.0)) {
            s.store_scalar(315, ((4.0 * s.v[63]) * 0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[771] != 0.0)) && (s.v[772] != 0.0)) && (s.v[781] != 0.0)) {
            s.store_ad(315, &{
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((s.v[418] != 0.0) && (s.v[771] != 0.0)) && (s.v[772] != 0.0)) && (s.v[781] != 0.0)) {
            s.store_sqrt_ad(315, A::add(A::square(s.ad_value(314)), s.ad_value(315)));
        }

    }

    pub(super) fn stamp_reactive_block_18(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if ((((s.v[418] != 0.0) && (s.v[771] != 0.0)) && (s.v[772] != 0.0)) && (s.v[781] != 0.0)) {
            s.store_offset_ad(350, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5), s.v[63]);
        }

        if ((((s.v[418] != 0.0) && (s.v[771] != 0.0)) && (s.v[772] != 0.0)) && (!(s.v[781] != 0.0))) {
            s.store_scalar(350, s.v[63]);
        }

        if ((((s.v[418] != 0.0) && (s.v[771] != 0.0)) && (s.v[772] != 0.0)) && (!(s.v[781] != 0.0))) {
            s.store_scalar(359, s.v[63]);
        }

        s.v[782] = if ((((s.v[85] * ((s.v[194] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85))))) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (s.v[771] != 0.0)) && (s.v[772] != 0.0)) && (s.v[782] != 0.0)) {
            s.store_exp_ad(372, A::scale(A::add(A::div(s.ad_value(194), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]));
        }

        s.v[783] = if ((s.v[85] * ((s.v[194] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85)))) < (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((s.v[418] != 0.0) && (s.v[771] != 0.0)) && (s.v[772] != 0.0)) && (!(s.v[782] != 0.0))) && (s.v[783] != 0.0)) {
            let assign24490_ad_e35985: A = A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(A::add(A::div(s.ad_value(194), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85])), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::scale(A::add(A::div(s.ad_value(194), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85])), 0.3333333333333333), 1.0));
            s.store_div_from_scalar_ad(372, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(A::add(A::div(s.ad_value(194), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85])), A::offset(A::scale(assign24490_ad_e35985, 0.5), 1.0)), 1.0));
        }

        if (((((s.v[418] != 0.0) && (s.v[771] != 0.0)) && (s.v[772] != 0.0)) && (!(s.v[782] != 0.0))) && (!(s.v[783] != 0.0))) {
            let assign24500_ad_e36063: A = A::mul(A::offset(A::scale(A::add(A::div(s.ad_value(194), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]), (-230.25850929940458)), A::offset(A::scale(A::offset(A::scale(A::add(A::div(s.ad_value(194), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]), (-230.25850929940458)), 0.3333333333333333), 1.0));
            s.store_scale_ad(372, A::offset(A::mul(A::offset(A::scale(A::add(A::div(s.ad_value(194), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]), (-230.25850929940458)), A::offset(A::scale(assign24500_ad_e36063, 0.5), 1.0)), 1.0), 1e100);
        }

        if (((s.v[418] != 0.0) && (s.v[771] != 0.0)) && (!(s.v[772] != 0.0))) {
            s.store_sqrt_ad(430, A::mul(A::offset(A::scale(A::sub(s.ad_value(194), s.ad_value(262)), s.v[85]), 1.0), s.ad_value(263)));
        }

        if (((s.v[418] != 0.0) && (s.v[771] != 0.0)) && (!(s.v[772] != 0.0))) {
            s.store_scale_ad(363, A::square(s.ad_value(318)), 1.0 / (s.v[308]));
        }

        if (((s.v[418] != 0.0) && (s.v[771] != 0.0)) && (!(s.v[772] != 0.0))) {
            s.store_scale_ad(362, A::ln(A::div_from_scalar(s.v[308], s.ad_value(363))), (s.v[62] / s.v[85]));
        }

        s.v[784] = if (s.v[62] < p.p85) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (s.v[771] != 0.0)) && (!(s.v[772] != 0.0))) && (s.v[784] != 0.0)) {
            s.store_offset_ad(360, A::scale(A::sub(s.ad_value(262), s.ad_value(362)), p.p86), s.v[62]);
        }

        if ((((s.v[418] != 0.0) && (s.v[771] != 0.0)) && (!(s.v[772] != 0.0))) && (s.v[784] != 0.0)) {
            s.store_sub_from_scalar_ad(350, s.v[62], A::scale(s.ad_value(362), p.p86));
        }

        if ((((s.v[418] != 0.0) && (s.v[771] != 0.0)) && (!(s.v[772] != 0.0))) && (s.v[784] != 0.0)) {
            s.store_offset_ad(314, A::sub_from_scalar(p.p85, s.ad_value(360)), (-0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[771] != 0.0)) && (!(s.v[772] != 0.0))) && (s.v[784] != 0.0)) {
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[771] != 0.0)) && (!(s.v[772] != 0.0))) && (s.v[784] != 0.0)) {
            s.store_ad(315, &{
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((s.v[418] != 0.0) && (s.v[771] != 0.0)) && (!(s.v[772] != 0.0))) && (s.v[784] != 0.0)) {
            s.store_sqrt_ad(315, A::add(A::square(s.ad_value(314)), s.ad_value(315)));
        }

        if ((((s.v[418] != 0.0) && (s.v[771] != 0.0)) && (!(s.v[772] != 0.0))) && (s.v[784] != 0.0)) {
            s.store_scale_ad(364, A::offset(A::div(s.ad_value(314), s.ad_value(315)), 1.0), 0.5);
        }

        if ((((s.v[418] != 0.0) && (s.v[771] != 0.0)) && (!(s.v[772] != 0.0))) && (s.v[784] != 0.0)) {
            s.store_sub_from_scalar_ad(361, p.p85, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5));
        }

        if ((((s.v[418] != 0.0) && (s.v[771] != 0.0)) && (!(s.v[772] != 0.0))) && (s.v[784] != 0.0)) {
            s.store_offset(314, 361, (((-s.v[62])) + ((-0.01))));
        }

        if ((((s.v[418] != 0.0) && (s.v[771] != 0.0)) && (!(s.v[772] != 0.0))) && (s.v[784] != 0.0)) {
            s.store_scalar(315, ((4.0 * s.v[62]) * 0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[771] != 0.0)) && (!(s.v[772] != 0.0))) && (s.v[784] != 0.0)) {
            s.store_ad(315, &{
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((s.v[418] != 0.0) && (s.v[771] != 0.0)) && (!(s.v[772] != 0.0))) && (s.v[784] != 0.0)) {
            s.store_sqrt_ad(315, A::add(A::square(s.ad_value(314)), s.ad_value(315)));
        }

        if ((((s.v[418] != 0.0) && (s.v[771] != 0.0)) && (!(s.v[772] != 0.0))) && (s.v[784] != 0.0)) {
            s.store_scale_ad(365, A::offset(A::div(s.ad_value(314), s.ad_value(315)), 1.0), 0.5);
        }

        if ((((s.v[418] != 0.0) && (s.v[771] != 0.0)) && (!(s.v[772] != 0.0))) && (s.v[784] != 0.0)) {
            s.store_offset_ad(359, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5), s.v[62]);
        }

        if ((((s.v[418] != 0.0) && (s.v[771] != 0.0)) && (!(s.v[772] != 0.0))) && (s.v[784] != 0.0)) {
            s.store_offset_ad(314, A::sub_from_scalar(p.p85, s.ad_value(350)), (-0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[771] != 0.0)) && (!(s.v[772] != 0.0))) && (s.v[784] != 0.0)) {
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[771] != 0.0)) && (!(s.v[772] != 0.0))) && (s.v[784] != 0.0)) {
            s.store_ad(315, &{
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((s.v[418] != 0.0) && (s.v[771] != 0.0)) && (!(s.v[772] != 0.0))) && (s.v[784] != 0.0)) {
            s.store_sqrt_ad(315, A::add(A::square(s.ad_value(314)), s.ad_value(315)));
        }

        if ((((s.v[418] != 0.0) && (s.v[771] != 0.0)) && (!(s.v[772] != 0.0))) && (s.v[784] != 0.0)) {
            s.store_sub_from_scalar_ad(350, p.p85, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5));
        }

        if ((((s.v[418] != 0.0) && (s.v[771] != 0.0)) && (!(s.v[772] != 0.0))) && (s.v[784] != 0.0)) {
            s.store_offset(314, 350, (((-s.v[62])) + ((-0.01))));
        }

        if ((((s.v[418] != 0.0) && (s.v[771] != 0.0)) && (!(s.v[772] != 0.0))) && (s.v[784] != 0.0)) {
            s.store_scalar(315, ((4.0 * s.v[62]) * 0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[771] != 0.0)) && (!(s.v[772] != 0.0))) && (s.v[784] != 0.0)) {
            s.store_ad(315, &{
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((s.v[418] != 0.0) && (s.v[771] != 0.0)) && (!(s.v[772] != 0.0))) && (s.v[784] != 0.0)) {
            s.store_sqrt_ad(315, A::add(A::square(s.ad_value(314)), s.ad_value(315)));
        }

        if ((((s.v[418] != 0.0) && (s.v[771] != 0.0)) && (!(s.v[772] != 0.0))) && (s.v[784] != 0.0)) {
            s.store_offset_ad(350, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5), s.v[62]);
        }

        if ((((s.v[418] != 0.0) && (s.v[771] != 0.0)) && (!(s.v[772] != 0.0))) && (s.v[784] != 0.0)) {
            s.store_mul_ad_lhs(366, A::scale(s.ad_value(364), p.p86), 365);
        }

        if ((((s.v[418] != 0.0) && (s.v[771] != 0.0)) && (!(s.v[772] != 0.0))) && (!(s.v[784] != 0.0))) {
            s.store_scalar(350, s.v[62]);
        }

        if ((((s.v[418] != 0.0) && (s.v[771] != 0.0)) && (!(s.v[772] != 0.0))) && (!(s.v[784] != 0.0))) {
            s.store_scalar(359, s.v[62]);
        }

        if ((((s.v[418] != 0.0) && (s.v[771] != 0.0)) && (!(s.v[772] != 0.0))) && (!(s.v[784] != 0.0))) {
            s.store_scalar(366, 0.0);
        }

        s.v[785] = if ((((s.v[85] * ((s.v[262] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85))))) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (s.v[771] != 0.0)) && (!(s.v[772] != 0.0))) && (s.v[785] != 0.0)) {
            s.store_exp_ad(281, A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]));
        }

        s.v[786] = if ((s.v[85] * ((s.v[262] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85)))) < (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((s.v[418] != 0.0) && (s.v[771] != 0.0)) && (!(s.v[772] != 0.0))) && (!(s.v[785] != 0.0))) && (s.v[786] != 0.0)) {
            let assign24860_ad_e36693: A = A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85])), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85])), 0.3333333333333333), 1.0));
            s.store_div_from_scalar_ad(281, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85])), A::offset(A::scale(assign24860_ad_e36693, 0.5), 1.0)), 1.0));
        }

        if (((((s.v[418] != 0.0) && (s.v[771] != 0.0)) && (!(s.v[772] != 0.0))) && (!(s.v[785] != 0.0))) && (!(s.v[786] != 0.0))) {
            let assign24870_ad_e36772: A = A::mul(A::offset(A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]), (-230.25850929940458)), A::offset(A::scale(A::offset(A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]), (-230.25850929940458)), 0.3333333333333333), 1.0));
            s.store_scale_ad(281, A::offset(A::mul(A::offset(A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]), (-230.25850929940458)), A::offset(A::scale(assign24870_ad_e36772, 0.5), 1.0)), 1.0), 1e100);
        }

        if (((s.v[418] != 0.0) && (s.v[771] != 0.0)) && (!(s.v[772] != 0.0))) {
            s.store_scale_ad(367, A::add(A::div(A::sub(s.ad_value(359), A::mul(s.ad_value(262), s.ad_value(366))), A::square(s.ad_value(359))), A::div(A::mul(s.ad_value(362), s.ad_value(366)), A::scale(s.ad_value(350), p.p85))), s.v[85]);
        }

        if (((s.v[418] != 0.0) && (s.v[771] != 0.0)) && (!(s.v[772] != 0.0))) {
            s.store_mul_ad_lhs(370, A::offset(A::mul(A::sub(s.ad_value(194), s.ad_value(262)), s.ad_value(367)), 1.0), 281);
        }

        if (((s.v[418] != 0.0) && (s.v[771] != 0.0)) && (!(s.v[772] != 0.0))) {
            s.store_scale_ad(363, A::square(s.ad_value(318)), 1.0 / (s.v[310]));
        }

        if (((s.v[418] != 0.0) && (s.v[771] != 0.0)) && (!(s.v[772] != 0.0))) {
            s.store_scale_ad(362, A::ln(A::div_from_scalar(s.v[310], s.ad_value(363))), (s.v[64] / s.v[85]));
        }

        s.v[787] = if (s.v[64] < p.p85) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (s.v[771] != 0.0)) && (!(s.v[772] != 0.0))) && (s.v[787] != 0.0)) {
            s.store_offset_ad(360, A::scale(A::sub(s.ad_value(262), s.ad_value(362)), p.p86), s.v[64]);
        }

        if ((((s.v[418] != 0.0) && (s.v[771] != 0.0)) && (!(s.v[772] != 0.0))) && (s.v[787] != 0.0)) {
            s.store_sub_from_scalar_ad(350, s.v[64], A::scale(s.ad_value(362), p.p86));
        }

        if ((((s.v[418] != 0.0) && (s.v[771] != 0.0)) && (!(s.v[772] != 0.0))) && (s.v[787] != 0.0)) {
            s.store_offset_ad(314, A::sub_from_scalar(p.p85, s.ad_value(360)), (-0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[771] != 0.0)) && (!(s.v[772] != 0.0))) && (s.v[787] != 0.0)) {
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[771] != 0.0)) && (!(s.v[772] != 0.0))) && (s.v[787] != 0.0)) {
            s.store_ad(315, &{
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((s.v[418] != 0.0) && (s.v[771] != 0.0)) && (!(s.v[772] != 0.0))) && (s.v[787] != 0.0)) {
            s.store_sqrt_ad(315, A::add(A::square(s.ad_value(314)), s.ad_value(315)));
        }

        if ((((s.v[418] != 0.0) && (s.v[771] != 0.0)) && (!(s.v[772] != 0.0))) && (s.v[787] != 0.0)) {
            s.store_scale_ad(364, A::offset(A::div(s.ad_value(314), s.ad_value(315)), 1.0), 0.5);
        }

        if ((((s.v[418] != 0.0) && (s.v[771] != 0.0)) && (!(s.v[772] != 0.0))) && (s.v[787] != 0.0)) {
            s.store_sub_from_scalar_ad(361, p.p85, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5));
        }

        if ((((s.v[418] != 0.0) && (s.v[771] != 0.0)) && (!(s.v[772] != 0.0))) && (s.v[787] != 0.0)) {
            s.store_offset(314, 361, (((-s.v[64])) + ((-0.01))));
        }

        if ((((s.v[418] != 0.0) && (s.v[771] != 0.0)) && (!(s.v[772] != 0.0))) && (s.v[787] != 0.0)) {
            s.store_scalar(315, ((4.0 * s.v[64]) * 0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[771] != 0.0)) && (!(s.v[772] != 0.0))) && (s.v[787] != 0.0)) {
            s.store_ad(315, &{
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((s.v[418] != 0.0) && (s.v[771] != 0.0)) && (!(s.v[772] != 0.0))) && (s.v[787] != 0.0)) {
            s.store_sqrt_ad(315, A::add(A::square(s.ad_value(314)), s.ad_value(315)));
        }

        if ((((s.v[418] != 0.0) && (s.v[771] != 0.0)) && (!(s.v[772] != 0.0))) && (s.v[787] != 0.0)) {
            s.store_scale_ad(365, A::offset(A::div(s.ad_value(314), s.ad_value(315)), 1.0), 0.5);
        }

        if ((((s.v[418] != 0.0) && (s.v[771] != 0.0)) && (!(s.v[772] != 0.0))) && (s.v[787] != 0.0)) {
            s.store_offset_ad(359, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5), s.v[64]);
        }

        if ((((s.v[418] != 0.0) && (s.v[771] != 0.0)) && (!(s.v[772] != 0.0))) && (s.v[787] != 0.0)) {
            s.store_offset_ad(314, A::sub_from_scalar(p.p85, s.ad_value(350)), (-0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[771] != 0.0)) && (!(s.v[772] != 0.0))) && (s.v[787] != 0.0)) {
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[771] != 0.0)) && (!(s.v[772] != 0.0))) && (s.v[787] != 0.0)) {
            s.store_ad(315, &{
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((s.v[418] != 0.0) && (s.v[771] != 0.0)) && (!(s.v[772] != 0.0))) && (s.v[787] != 0.0)) {
            s.store_sqrt_ad(315, A::add(A::square(s.ad_value(314)), s.ad_value(315)));
        }

        if ((((s.v[418] != 0.0) && (s.v[771] != 0.0)) && (!(s.v[772] != 0.0))) && (s.v[787] != 0.0)) {
            s.store_sub_from_scalar_ad(350, p.p85, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5));
        }

        if ((((s.v[418] != 0.0) && (s.v[771] != 0.0)) && (!(s.v[772] != 0.0))) && (s.v[787] != 0.0)) {
            s.store_offset(314, 350, (((-s.v[64])) + ((-0.01))));
        }

        if ((((s.v[418] != 0.0) && (s.v[771] != 0.0)) && (!(s.v[772] != 0.0))) && (s.v[787] != 0.0)) {
            s.store_scalar(315, ((4.0 * s.v[64]) * 0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[771] != 0.0)) && (!(s.v[772] != 0.0))) && (s.v[787] != 0.0)) {
            s.store_ad(315, &{
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((s.v[418] != 0.0) && (s.v[771] != 0.0)) && (!(s.v[772] != 0.0))) && (s.v[787] != 0.0)) {
            s.store_sqrt_ad(315, A::add(A::square(s.ad_value(314)), s.ad_value(315)));
        }

        if ((((s.v[418] != 0.0) && (s.v[771] != 0.0)) && (!(s.v[772] != 0.0))) && (s.v[787] != 0.0)) {
            s.store_offset_ad(350, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5), s.v[64]);
        }

        if ((((s.v[418] != 0.0) && (s.v[771] != 0.0)) && (!(s.v[772] != 0.0))) && (s.v[787] != 0.0)) {
            s.store_mul_ad_lhs(366, A::scale(s.ad_value(364), p.p86), 365);
        }

        if ((((s.v[418] != 0.0) && (s.v[771] != 0.0)) && (!(s.v[772] != 0.0))) && (!(s.v[787] != 0.0))) {
            s.store_scalar(350, s.v[64]);
        }

        if ((((s.v[418] != 0.0) && (s.v[771] != 0.0)) && (!(s.v[772] != 0.0))) && (!(s.v[787] != 0.0))) {
            s.store_scalar(359, s.v[64]);
        }

        if ((((s.v[418] != 0.0) && (s.v[771] != 0.0)) && (!(s.v[772] != 0.0))) && (!(s.v[787] != 0.0))) {
            s.store_scalar(366, 0.0);
        }

        s.v[788] = if ((((s.v[85] * ((s.v[262] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85))))) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (s.v[771] != 0.0)) && (!(s.v[772] != 0.0))) && (s.v[788] != 0.0)) {
            s.store_exp_ad(282, A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]));
        }

        s.v[789] = if ((s.v[85] * ((s.v[262] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85)))) < (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((s.v[418] != 0.0) && (s.v[771] != 0.0)) && (!(s.v[772] != 0.0))) && (!(s.v[788] != 0.0))) && (s.v[789] != 0.0)) {
            let assign25240_ad_e37428: A = A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85])), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85])), 0.3333333333333333), 1.0));
            s.store_div_from_scalar_ad(282, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85])), A::offset(A::scale(assign25240_ad_e37428, 0.5), 1.0)), 1.0));
        }

        if (((((s.v[418] != 0.0) && (s.v[771] != 0.0)) && (!(s.v[772] != 0.0))) && (!(s.v[788] != 0.0))) && (!(s.v[789] != 0.0))) {
            let assign25250_ad_e37507: A = A::mul(A::offset(A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]), (-230.25850929940458)), A::offset(A::scale(A::offset(A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]), (-230.25850929940458)), 0.3333333333333333), 1.0));
            s.store_scale_ad(282, A::offset(A::mul(A::offset(A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]), (-230.25850929940458)), A::offset(A::scale(assign25250_ad_e37507, 0.5), 1.0)), 1.0), 1e100);
        }

        if (((s.v[418] != 0.0) && (s.v[771] != 0.0)) && (!(s.v[772] != 0.0))) {
            s.store_scale_ad(367, A::add(A::div(A::sub(s.ad_value(359), A::mul(s.ad_value(262), s.ad_value(366))), A::square(s.ad_value(359))), A::div(A::mul(s.ad_value(362), s.ad_value(366)), A::scale(s.ad_value(350), p.p85))), s.v[85]);
        }

        if (((s.v[418] != 0.0) && (s.v[771] != 0.0)) && (!(s.v[772] != 0.0))) {
            s.store_mul_ad_lhs(371, A::offset(A::mul(A::sub(s.ad_value(194), s.ad_value(262)), s.ad_value(367)), 1.0), 282);
        }

        if (((s.v[418] != 0.0) && (s.v[771] != 0.0)) && (!(s.v[772] != 0.0))) {
            s.store_scale_ad(363, A::square(s.ad_value(318)), 1.0 / (s.v[309]));
        }

        if (((s.v[418] != 0.0) && (s.v[771] != 0.0)) && (!(s.v[772] != 0.0))) {
            s.store_scale_ad(362, A::ln(A::div_from_scalar(s.v[309], s.ad_value(363))), (s.v[63] / s.v[85]));
        }

        s.v[790] = if (s.v[63] < p.p85) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (s.v[771] != 0.0)) && (!(s.v[772] != 0.0))) && (s.v[790] != 0.0)) {
            s.store_offset_ad(360, A::scale(A::sub(s.ad_value(262), s.ad_value(362)), p.p86), s.v[63]);
        }

        if ((((s.v[418] != 0.0) && (s.v[771] != 0.0)) && (!(s.v[772] != 0.0))) && (s.v[790] != 0.0)) {
            s.store_sub_from_scalar_ad(350, s.v[63], A::scale(s.ad_value(362), p.p86));
        }

        if ((((s.v[418] != 0.0) && (s.v[771] != 0.0)) && (!(s.v[772] != 0.0))) && (s.v[790] != 0.0)) {
            s.store_offset_ad(314, A::sub_from_scalar(p.p85, s.ad_value(360)), (-0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[771] != 0.0)) && (!(s.v[772] != 0.0))) && (s.v[790] != 0.0)) {
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[771] != 0.0)) && (!(s.v[772] != 0.0))) && (s.v[790] != 0.0)) {
            s.store_ad(315, &{
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((s.v[418] != 0.0) && (s.v[771] != 0.0)) && (!(s.v[772] != 0.0))) && (s.v[790] != 0.0)) {
            s.store_sqrt_ad(315, A::add(A::square(s.ad_value(314)), s.ad_value(315)));
        }

        if ((((s.v[418] != 0.0) && (s.v[771] != 0.0)) && (!(s.v[772] != 0.0))) && (s.v[790] != 0.0)) {
            s.store_scale_ad(364, A::offset(A::div(s.ad_value(314), s.ad_value(315)), 1.0), 0.5);
        }

        if ((((s.v[418] != 0.0) && (s.v[771] != 0.0)) && (!(s.v[772] != 0.0))) && (s.v[790] != 0.0)) {
            s.store_sub_from_scalar_ad(361, p.p85, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5));
        }

        if ((((s.v[418] != 0.0) && (s.v[771] != 0.0)) && (!(s.v[772] != 0.0))) && (s.v[790] != 0.0)) {
            s.store_offset(314, 361, (((-s.v[63])) + ((-0.01))));
        }

        if ((((s.v[418] != 0.0) && (s.v[771] != 0.0)) && (!(s.v[772] != 0.0))) && (s.v[790] != 0.0)) {
            s.store_scalar(315, ((4.0 * s.v[63]) * 0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[771] != 0.0)) && (!(s.v[772] != 0.0))) && (s.v[790] != 0.0)) {
            s.store_ad(315, &{
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((s.v[418] != 0.0) && (s.v[771] != 0.0)) && (!(s.v[772] != 0.0))) && (s.v[790] != 0.0)) {
            s.store_sqrt_ad(315, A::add(A::square(s.ad_value(314)), s.ad_value(315)));
        }

        if ((((s.v[418] != 0.0) && (s.v[771] != 0.0)) && (!(s.v[772] != 0.0))) && (s.v[790] != 0.0)) {
            s.store_scale_ad(365, A::offset(A::div(s.ad_value(314), s.ad_value(315)), 1.0), 0.5);
        }

        if ((((s.v[418] != 0.0) && (s.v[771] != 0.0)) && (!(s.v[772] != 0.0))) && (s.v[790] != 0.0)) {
            s.store_offset_ad(359, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5), s.v[63]);
        }

        if ((((s.v[418] != 0.0) && (s.v[771] != 0.0)) && (!(s.v[772] != 0.0))) && (s.v[790] != 0.0)) {
            s.store_offset_ad(314, A::sub_from_scalar(p.p85, s.ad_value(350)), (-0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[771] != 0.0)) && (!(s.v[772] != 0.0))) && (s.v[790] != 0.0)) {
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[771] != 0.0)) && (!(s.v[772] != 0.0))) && (s.v[790] != 0.0)) {
            s.store_ad(315, &{
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((s.v[418] != 0.0) && (s.v[771] != 0.0)) && (!(s.v[772] != 0.0))) && (s.v[790] != 0.0)) {
            s.store_sqrt_ad(315, A::add(A::square(s.ad_value(314)), s.ad_value(315)));
        }

        if ((((s.v[418] != 0.0) && (s.v[771] != 0.0)) && (!(s.v[772] != 0.0))) && (s.v[790] != 0.0)) {
            s.store_sub_from_scalar_ad(350, p.p85, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5));
        }

        if ((((s.v[418] != 0.0) && (s.v[771] != 0.0)) && (!(s.v[772] != 0.0))) && (s.v[790] != 0.0)) {
            s.store_offset(314, 350, (((-s.v[63])) + ((-0.01))));
        }

        if ((((s.v[418] != 0.0) && (s.v[771] != 0.0)) && (!(s.v[772] != 0.0))) && (s.v[790] != 0.0)) {
            s.store_scalar(315, ((4.0 * s.v[63]) * 0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[771] != 0.0)) && (!(s.v[772] != 0.0))) && (s.v[790] != 0.0)) {
            s.store_ad(315, &{
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((s.v[418] != 0.0) && (s.v[771] != 0.0)) && (!(s.v[772] != 0.0))) && (s.v[790] != 0.0)) {
            s.store_sqrt_ad(315, A::add(A::square(s.ad_value(314)), s.ad_value(315)));
        }

        if ((((s.v[418] != 0.0) && (s.v[771] != 0.0)) && (!(s.v[772] != 0.0))) && (s.v[790] != 0.0)) {
            s.store_offset_ad(350, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5), s.v[63]);
        }

        if ((((s.v[418] != 0.0) && (s.v[771] != 0.0)) && (!(s.v[772] != 0.0))) && (s.v[790] != 0.0)) {
            s.store_mul_ad_lhs(366, A::scale(s.ad_value(364), p.p86), 365);
        }

    }

    pub(super) fn stamp_reactive_block_19(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if ((((s.v[418] != 0.0) && (s.v[771] != 0.0)) && (!(s.v[772] != 0.0))) && (!(s.v[790] != 0.0))) {
            s.store_scalar(350, s.v[63]);
        }

        if ((((s.v[418] != 0.0) && (s.v[771] != 0.0)) && (!(s.v[772] != 0.0))) && (!(s.v[790] != 0.0))) {
            s.store_scalar(359, s.v[63]);
        }

        if ((((s.v[418] != 0.0) && (s.v[771] != 0.0)) && (!(s.v[772] != 0.0))) && (!(s.v[790] != 0.0))) {
            s.store_scalar(366, 0.0);
        }

        s.v[791] = if ((((s.v[85] * ((s.v[262] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85))))) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (s.v[771] != 0.0)) && (!(s.v[772] != 0.0))) && (s.v[791] != 0.0)) {
            s.store_exp_ad(283, A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]));
        }

        s.v[792] = if ((s.v[85] * ((s.v[262] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85)))) < (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((s.v[418] != 0.0) && (s.v[771] != 0.0)) && (!(s.v[772] != 0.0))) && (!(s.v[791] != 0.0))) && (s.v[792] != 0.0)) {
            let assign25620_ad_e38163: A = A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85])), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85])), 0.3333333333333333), 1.0));
            s.store_div_from_scalar_ad(283, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85])), A::offset(A::scale(assign25620_ad_e38163, 0.5), 1.0)), 1.0));
        }

        if (((((s.v[418] != 0.0) && (s.v[771] != 0.0)) && (!(s.v[772] != 0.0))) && (!(s.v[791] != 0.0))) && (!(s.v[792] != 0.0))) {
            let assign25630_ad_e38242: A = A::mul(A::offset(A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]), (-230.25850929940458)), A::offset(A::scale(A::offset(A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]), (-230.25850929940458)), 0.3333333333333333), 1.0));
            s.store_scale_ad(283, A::offset(A::mul(A::offset(A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]), (-230.25850929940458)), A::offset(A::scale(assign25630_ad_e38242, 0.5), 1.0)), 1.0), 1e100);
        }

        if (((s.v[418] != 0.0) && (s.v[771] != 0.0)) && (!(s.v[772] != 0.0))) {
            s.store_scale_ad(367, A::add(A::div(A::sub(s.ad_value(359), A::mul(s.ad_value(262), s.ad_value(366))), A::square(s.ad_value(359))), A::div(A::mul(s.ad_value(362), s.ad_value(366)), A::scale(s.ad_value(350), p.p85))), s.v[85]);
        }

        if (((s.v[418] != 0.0) && (s.v[771] != 0.0)) && (!(s.v[772] != 0.0))) {
            s.store_mul_ad_lhs(372, A::offset(A::mul(A::sub(s.ad_value(194), s.ad_value(262)), s.ad_value(367)), 1.0), 283);
        }

        if ((s.v[418] != 0.0) && (s.v[771] != 0.0)) {
            s.store_offset(370, 370, (-1.0));
        }

        if ((s.v[418] != 0.0) && (s.v[771] != 0.0)) {
            s.store_offset(371, 371, (-1.0));
        }

        if ((s.v[418] != 0.0) && (s.v[771] != 0.0)) {
            s.store_offset(372, 372, (-1.0));
        }

        if ((s.v[418] != 0.0) && (s.v[771] != 0.0)) {
            s.store_div_from_scalar(429, 1.0, 430);
        }

        s.v[793] = if (s.v[194] > 0.0) { 1.0 } else { 0.0 };

        if (((s.v[418] != 0.0) && (s.v[771] != 0.0)) && (s.v[793] != 0.0)) {
            s.store_scale_ad(431, A::ln(A::add(A::offset(s.ad_value(429), 2.0), A::sqrt(A::mul(A::offset(s.ad_value(429), 1.0), A::offset(s.ad_value(429), 3.0))))), (s.v[84] * 2.0));
        }

        if (((s.v[418] != 0.0) && (s.v[771] != 0.0)) && (!(s.v[793] != 0.0))) {
            s.store_sub_ad_lhs(431, A::scale(A::ln(A::add(A::offset(A::scale(s.ad_value(430), 2.0), 1.0), A::sqrt(A::mul(A::offset(s.ad_value(430), 1.0), A::offset(A::scale(s.ad_value(430), 3.0), 1.0))))), (s.v[84] * 2.0)), 194);
        }

        if ((s.v[418] != 0.0) && (s.v[771] != 0.0)) {
            s.store_sub(432, 264, 431);
        }

        if ((s.v[418] != 0.0) && (s.v[771] != 0.0)) {
            s.store_scale_ad(433, A::sub(A::add(s.ad_value(194), s.ad_value(432)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(194), s.ad_value(432)), A::sub(s.ad_value(194), s.ad_value(432))), ((4.0 * s.v[84]) * s.v[84])))), 0.5);
        }

        if ((s.v[418] != 0.0) && (s.v[771] != 0.0)) {
            s.store_scale_ad(434, A::sub(A::add(s.ad_value(194), s.ad_value(267)), A::sqrt(A::add(A::mul(A::sub(s.ad_value(194), s.ad_value(267)), A::sub(s.ad_value(194), s.ad_value(267))), A::mul(A::scale(s.ad_value(82), 4.0), s.ad_value(82))))), 0.5);
        }

        if ((s.v[418] != 0.0) && (s.v[771] != 0.0)) {
            s.store_scale_ad(435, A::sub(s.ad_value(194), A::sqrt(A::offset(A::mul(s.ad_value(194), s.ad_value(194)), ((4.0 * 1e-6) * 1e-6)))), 0.5);
        }

        if ((s.v[418] != 0.0) && (!(s.v[771] != 0.0))) {
            s.store_scalar(370, 0.0);
        }

        if ((s.v[418] != 0.0) && (!(s.v[771] != 0.0))) {
            s.store_scalar(371, 0.0);
        }

        if ((s.v[418] != 0.0) && (!(s.v[771] != 0.0))) {
            s.store_scalar(372, 0.0);
        }

        if ((s.v[418] != 0.0) && (!(s.v[771] != 0.0))) {
            s.store_scalar(431, 0.0);
        }

        if ((s.v[418] != 0.0) && (!(s.v[771] != 0.0))) {
            s.store_scalar(428, 0.0);
        }

        if ((s.v[418] != 0.0) && (!(s.v[771] != 0.0))) {
            s.store_scalar(430, 0.0);
        }

        if ((s.v[418] != 0.0) && (!(s.v[771] != 0.0))) {
            s.store_scalar(433, 0.0);
        }

        if ((s.v[418] != 0.0) && (!(s.v[771] != 0.0))) {
            s.store_scalar(434, 0.0);
        }

        if ((s.v[418] != 0.0) && (!(s.v[771] != 0.0))) {
            s.store_scalar(435, 0.0);
        }

        s.v[794] = if (s.v[256] == 0.0) { 1.0 } else { 0.0 };

        if ((s.v[418] != 0.0) && (s.v[794] != 0.0)) {
            s.store_scalar(268, 0.0);
        }

        if ((s.v[418] != 0.0) && (s.v[794] != 0.0)) {
            s.store_scalar(291, 0.0);
        }

        if ((s.v[418] != 0.0) && (s.v[794] != 0.0)) {
            s.store_scalar(269, 0.0);
        }

        s.v[795] = if (s.v[122] == 0.5) { 1.0 } else { 0.0 };

        if (((s.v[418] != 0.0) && (!(s.v[794] != 0.0))) && (s.v[795] != 0.0)) {
            s.store_sqrt_ad(436, A::sub_from_scalar(1.0, A::mul(s.ad_value(428), s.ad_value(119))));
        }

        if (((s.v[418] != 0.0) && (!(s.v[794] != 0.0))) && (!(s.v[795] != 0.0))) {
            s.store_powf_ad(436, A::sub_from_scalar(1.0, A::mul(s.ad_value(428), s.ad_value(119))), s.v[122]);
        }

        if ((s.v[418] != 0.0) && (!(s.v[794] != 0.0))) {
            s.store_add_ad(269, A::mul(s.ad_value(131), A::sub_from_scalar(1.0, s.ad_value(436))), A::mul(s.ad_value(134), A::sub(s.ad_value(194), s.ad_value(428))));
        }

        if ((s.v[418] != 0.0) && (!(s.v[794] != 0.0))) {
            s.store_mul(437, 101, 370);
        }

        s.v[796] = if ((s.v[20] == 0.0) && (s.v[23] == 0.0)) { 1.0 } else { 0.0 };

        if (((s.v[418] != 0.0) && (!(s.v[794] != 0.0))) && (s.v[796] != 0.0)) {
            s.store_scalar(439, 0.0);
        }

        if (((s.v[418] != 0.0) && (!(s.v[794] != 0.0))) && (s.v[796] != 0.0)) {
            s.store_scalar(442, 0.0);
        }

        if (((s.v[418] != 0.0) && (!(s.v[794] != 0.0))) && (s.v[796] != 0.0)) {
            s.store_scalar(443, 0.0);
        }

        if (((s.v[418] != 0.0) && (!(s.v[794] != 0.0))) && (s.v[796] != 0.0)) {
            s.store_scalar(444, 0.0);
        }

        if (((s.v[418] != 0.0) && (!(s.v[794] != 0.0))) && (s.v[796] != 0.0)) {
            s.store_scalar(438, 0.0);
        }

        if (((s.v[418] != 0.0) && (!(s.v[794] != 0.0))) && (!(s.v[796] != 0.0))) {
            s.store_sub(439, 107, 433);
        }

        if (((s.v[418] != 0.0) && (!(s.v[794] != 0.0))) && (!(s.v[796] != 0.0))) {
            s.store_sub_from_scalar_ad(440, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(431), s.ad_value(439)))));
        }

        s.v[797] = if (s.v[9] == 0.5) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (!(s.v[794] != 0.0))) && (!(s.v[796] != 0.0))) && (s.v[797] != 0.0)) {
            s.store_scalar(441, 0.0);
        }

        if ((((s.v[418] != 0.0) && (!(s.v[794] != 0.0))) && (!(s.v[796] != 0.0))) && (!(s.v[797] != 0.0))) {
            s.store_scale_ad(441, A::add(A::div(A::mul(A::square(s.ad_value(440)), A::ln(s.ad_value(440))), A::sub_from_scalar(1.0, s.ad_value(440))), s.ad_value(440)), (1.0 - (2.0 * s.v[9])));
        }

        if (((s.v[418] != 0.0) && (!(s.v[794] != 0.0))) && (!(s.v[796] != 0.0))) {
            s.store_add(442, 440, 441);
        }

        s.v[798] = if (s.v[9] == 0.5) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (!(s.v[794] != 0.0))) && (!(s.v[796] != 0.0))) && (s.v[798] != 0.0)) {
            s.store_sqrt_ad(436, A::scale(s.ad_value(439), s.v[143]));
        }

        if ((((s.v[418] != 0.0) && (!(s.v[794] != 0.0))) && (!(s.v[796] != 0.0))) && (!(s.v[798] != 0.0))) {
            s.store_powf_ad(436, A::scale(s.ad_value(439), s.v[143]), s.v[9]);
        }

        if (((s.v[418] != 0.0) && (!(s.v[794] != 0.0))) && (!(s.v[796] != 0.0))) {
            s.store_scale(443, 436, s.v[137]);
        }

        if (((s.v[418] != 0.0) && (!(s.v[794] != 0.0))) && (!(s.v[796] != 0.0))) {
            s.store_mul_ad_rhs(444, 98, A::mul(A::offset(s.ad_value(430), (-1.0)), s.ad_value(443)));
        }

        if (((s.v[418] != 0.0) && (!(s.v[794] != 0.0))) && (!(s.v[796] != 0.0))) {
            s.store_scaled_mul(438, 444, 442, s.v[20]);
        }

        s.v[799] = if (s.v[23] == 0.0) { 1.0 } else { 0.0 };

        if (((s.v[418] != 0.0) && (!(s.v[794] != 0.0))) && (s.v[799] != 0.0)) {
            s.store_scalar(445, 0.0);
        }

        if (((s.v[418] != 0.0) && (!(s.v[794] != 0.0))) && (!(s.v[799] != 0.0))) {
            s.store_scale_ad(446, A::div(A::scale(s.ad_value(443), s.v[122]), s.ad_value(439)), s.v[152]);
        }

        if (((s.v[418] != 0.0) && (!(s.v[794] != 0.0))) && (!(s.v[799] != 0.0))) {
            s.store_div_from_scalar(447, (0.666666666666667 * s.v[149]), 446);
        }

        if (((s.v[418] != 0.0) && (!(s.v[794] != 0.0))) && (!(s.v[799] != 0.0))) {
            s.store_square(448, 447);
        }

        if (((s.v[418] != 0.0) && (!(s.v[794] != 0.0))) && (!(s.v[799] != 0.0))) {
            s.store_sqrt_ad(449, A::div(A::square(s.ad_value(448)), A::offset(A::square(s.ad_value(448)), 1.0)));
        }

        if (((s.v[418] != 0.0) && (!(s.v[794] != 0.0))) && (!(s.v[799] != 0.0))) {
            s.store_sqrt_ad(450, A::abs(s.ad_value(449)));
        }

        if (((s.v[418] != 0.0) && (!(s.v[794] != 0.0))) && (!(s.v[799] != 0.0))) {
            s.store_mul(451, 449, 450);
        }

        s.v[800] = if (((-s.v[9]) * s.v[125]) == (-1.0)) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (!(s.v[794] != 0.0))) && (!(s.v[799] != 0.0))) && (s.v[800] != 0.0)) {
            s.store_div_from_scalar_ad(452, 1.0, A::offset(A::mul(s.ad_value(446), s.ad_value(451)), 1.0));
        }

        if ((((s.v[418] != 0.0) && (!(s.v[794] != 0.0))) && (!(s.v[799] != 0.0))) && (!(s.v[800] != 0.0))) {
            s.store_powf_ad(452, A::offset(A::mul(s.ad_value(446), s.ad_value(451)), 1.0), ((-s.v[9]) * s.v[125]));
        }

        if (((s.v[418] != 0.0) && (!(s.v[794] != 0.0))) && (!(s.v[799] != 0.0))) {
            s.store_div_ad(453, A::mul(s.ad_value(442), s.ad_value(452)), A::add(s.ad_value(442), s.ad_value(452)));
        }

        if (((s.v[418] != 0.0) && (!(s.v[794] != 0.0))) && (!(s.v[799] != 0.0))) {
            s.store_sqrt_ad(454, A::scale(A::div(s.ad_value(446), s.ad_value(450)), 0.375));
        }

        if (((s.v[418] != 0.0) && (!(s.v[794] != 0.0))) && (!(s.v[799] != 0.0))) {
            s.store_sub_ad_lhs(455, A::scale(A::mul(s.ad_value(447), s.ad_value(450)), 2.0), 449);
        }

        if (((s.v[418] != 0.0) && (!(s.v[794] != 0.0))) && (!(s.v[799] != 0.0))) {
            s.store_add_ad(456, A::sub(A::mul(A::scale(s.ad_value(447), s.v[149]), s.ad_value(450)), A::scale(s.ad_value(449), s.v[149])), A::scale(A::mul(s.ad_value(446), s.ad_value(451)), 0.5));
        }

        if (((s.v[418] != 0.0) && (!(s.v[794] != 0.0))) && (!(s.v[799] != 0.0))) {
            s.store_mul_ad_lhs(457, A::offset(s.ad_value(455), (-1.0)), 454);
        }

        if (((s.v[418] != 0.0) && (!(s.v[794] != 0.0))) && (!(s.v[799] != 0.0))) {
            s.store_square(419, 457);
        }

        s.v[801] = if (s.v[457] > 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (!(s.v[794] != 0.0))) && (!(s.v[799] != 0.0))) && (s.v[801] != 0.0)) {
            s.store_div_from_scalar_ad(420, 1.0, A::offset(A::scale(s.ad_value(457), s.v[86]), 1.0));
        }

        if ((((s.v[418] != 0.0) && (!(s.v[794] != 0.0))) && (!(s.v[799] != 0.0))) && (!(s.v[801] != 0.0))) {
            s.store_div_from_scalar_ad(420, 1.0, A::sub_from_scalar(1.0, A::scale(s.ad_value(457), s.v[86])));
        }

        s.v[802] = if (((-s.v[419]) + s.v[456]) > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (!(s.v[794] != 0.0))) && (!(s.v[799] != 0.0))) && (s.v[802] != 0.0)) {
            s.store_exp_ad(436, A::sub(s.ad_value(456), s.ad_value(419)));
        }

        if ((((s.v[418] != 0.0) && (!(s.v[794] != 0.0))) && (!(s.v[799] != 0.0))) && (!(s.v[802] != 0.0))) {
            s.store_div_from_scalar_ad(436, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(456), s.ad_value(419))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(456), s.ad_value(419))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(456), s.ad_value(419))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if (((s.v[418] != 0.0) && (!(s.v[794] != 0.0))) && (!(s.v[799] != 0.0))) {
            s.store_mul_ad_lhs(421, A::add(A::add(A::scale(s.ad_value(420), 0.29214664), A::scale(A::square(s.ad_value(420)), s.v[87])), A::scale(A::mul(A::square(s.ad_value(420)), s.ad_value(420)), s.v[88])), 436);
        }

        s.v[803] = if (s.v[457] > 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (!(s.v[794] != 0.0))) && (!(s.v[799] != 0.0))) && (s.v[803] != 0.0)) {
            s.copy_ad(458, 421);
        }

        s.v[804] = if (s.v[456] > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((s.v[418] != 0.0) && (!(s.v[794] != 0.0))) && (!(s.v[799] != 0.0))) && (!(s.v[803] != 0.0))) && (s.v[804] != 0.0)) {
            s.store_exp(436, 456);
        }

        if (((((s.v[418] != 0.0) && (!(s.v[794] != 0.0))) && (!(s.v[799] != 0.0))) && (!(s.v[803] != 0.0))) && (!(s.v[804] != 0.0))) {
            s.store_div_from_scalar_ad(436, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(456)), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(456)), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), s.ad_value(456)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if ((((s.v[418] != 0.0) && (!(s.v[794] != 0.0))) && (!(s.v[799] != 0.0))) && (!(s.v[803] != 0.0))) {
            s.store_sub_ad_lhs(458, A::scale(s.ad_value(436), 2.0), 421);
        }

        if (((s.v[418] != 0.0) && (!(s.v[794] != 0.0))) && (!(s.v[799] != 0.0))) {
            s.store_scale_ad(459, A::div(A::scale(s.ad_value(458), s.v[149]), s.ad_value(454)), (1.772453850905516 * 0.5));
        }

        if (((s.v[418] != 0.0) && (!(s.v[794] != 0.0))) && (!(s.v[799] != 0.0))) {
            s.store_scale_ad(445, A::mul(A::mul(s.ad_value(444), s.ad_value(459)), s.ad_value(453)), s.v[23]);
        }

        s.v[805] = if (s.v[29] == 0.0) { 1.0 } else { 0.0 };

        if (((s.v[418] != 0.0) && (!(s.v[794] != 0.0))) && (s.v[805] != 0.0)) {
            s.store_scalar(460, 0.0);
        }

        s.v[806] = if (s.v[9] == 0.5) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (!(s.v[794] != 0.0))) && (!(s.v[805] != 0.0))) && (s.v[806] != 0.0)) {
            s.store_sqrt_ad(436, A::scale(A::sub_from_scalar(s.v[6], s.ad_value(434)), s.v[143]));
        }

        if ((((s.v[418] != 0.0) && (!(s.v[794] != 0.0))) && (!(s.v[805] != 0.0))) && (!(s.v[806] != 0.0))) {
            s.store_powf_ad(436, A::scale(A::sub_from_scalar(s.v[6], s.ad_value(434)), s.v[143]), s.v[9]);
        }

        if (((s.v[418] != 0.0) && (!(s.v[794] != 0.0))) && (!(s.v[805] != 0.0))) {
            s.store_scale_ad(461, A::div(A::scale(A::sub_from_scalar(s.v[6], s.ad_value(434)), s.v[140]), s.ad_value(436)), s.v[125]);
        }

        s.v[807] = if (((((-s.v[155]) / s.v[461])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (!(s.v[794] != 0.0))) && (!(s.v[805] != 0.0))) && (s.v[807] != 0.0)) {
            s.store_exp_ad(436, A::div(A::neg(s.ad_value(155)), s.ad_value(461)));
        }

        s.v[808] = if (((-s.v[155]) / s.v[461]) < (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((s.v[418] != 0.0) && (!(s.v[794] != 0.0))) && (!(s.v[805] != 0.0))) && (!(s.v[807] != 0.0))) && (s.v[808] != 0.0)) {
            let assign26540_ad_e39476: A = A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(155)), s.ad_value(461))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(155)), s.ad_value(461))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(155)), s.ad_value(461))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
            s.store_div_from_scalar_ad(436, 1e-100, assign26540_ad_e39476);
        }

        if (((((s.v[418] != 0.0) && (!(s.v[794] != 0.0))) && (!(s.v[805] != 0.0))) && (!(s.v[807] != 0.0))) && (!(s.v[808] != 0.0))) {
            let assign26550_ad_e39524: A = A::scale(A::offset(A::mul(A::offset(A::div(A::neg(s.ad_value(155)), s.ad_value(461)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::div(A::neg(s.ad_value(155)), s.ad_value(461)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::div(A::neg(s.ad_value(155)), s.ad_value(461)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
            s.store_ad(436, &assign26550_ad_e39524);
        }

        if (((s.v[418] != 0.0) && (!(s.v[794] != 0.0))) && (!(s.v[805] != 0.0))) {
            s.store_scale_ad(460, A::mul(A::mul(A::mul(s.ad_value(194), s.ad_value(461)), s.ad_value(461)), s.ad_value(436)), s.v[29]);
        }

        s.v[809] = if ((s.v[38] > 1000000.0) || (p.p80 == 0.0)) { 1.0 } else { 0.0 };

        if (((s.v[418] != 0.0) && (!(s.v[794] != 0.0))) && (s.v[809] != 0.0)) {
            s.store_scalar(462, 1.0);
        }

        s.v[810] = if (s.v[435] > ((-s.v[158]) * s.v[38])) { 1.0 } else { 0.0 };

        s.v[811] = if (s.v[41] == 4.0) { 1.0 } else { 0.0 };

        if (((((s.v[418] != 0.0) && (!(s.v[794] != 0.0))) && (!(s.v[809] != 0.0))) && (s.v[810] != 0.0)) && (s.v[811] != 0.0)) {
            s.store_mul_ad(436, A::mul(A::mul(A::abs(A::mul(s.ad_value(435), s.ad_value(162))), A::abs(A::mul(s.ad_value(435), s.ad_value(162)))), A::abs(A::mul(s.ad_value(435), s.ad_value(162)))), A::abs(A::mul(s.ad_value(435), s.ad_value(162))));
        }

        if (((((s.v[418] != 0.0) && (!(s.v[794] != 0.0))) && (!(s.v[809] != 0.0))) && (s.v[810] != 0.0)) && (!(s.v[811] != 0.0))) {
            s.store_powf_ad(436, A::abs(A::mul(s.ad_value(435), s.ad_value(162))), s.v[41]);
        }

        if ((((s.v[418] != 0.0) && (!(s.v[794] != 0.0))) && (!(s.v[809] != 0.0))) && (s.v[810] != 0.0)) {
            s.store_div_from_scalar_ad(462, 1.0, A::sub_from_scalar(1.0, s.ad_value(436)));
        }

        if ((((s.v[418] != 0.0) && (!(s.v[794] != 0.0))) && (!(s.v[809] != 0.0))) && (!(s.v[810] != 0.0))) {
            s.store_offset_ad(462, A::mul(A::add(s.ad_value(435), A::scale(s.ad_value(38), s.v[158])), s.ad_value(165)), s.v[159]);
        }

        if ((s.v[418] != 0.0) && (!(s.v[794] != 0.0))) {
            s.store_mul_ad_lhs(268, A::add(A::add(A::add(s.ad_value(437), s.ad_value(438)), s.ad_value(445)), s.ad_value(460)), 462);
        }

        if ((s.v[418] != 0.0) && (!(s.v[794] != 0.0))) {
            s.store_mul_ad_lhs(291, A::add(A::add(s.ad_value(438), s.ad_value(445)), s.ad_value(460)), 462);
        }

        s.v[812] = if (s.v[257] == 0.0) { 1.0 } else { 0.0 };

        if ((s.v[418] != 0.0) && (s.v[812] != 0.0)) {
            s.store_scalar(270, 0.0);
        }

        if ((s.v[418] != 0.0) && (s.v[812] != 0.0)) {
            s.store_scalar(292, 0.0);
        }

        if ((s.v[418] != 0.0) && (s.v[812] != 0.0)) {
            s.store_scalar(271, 0.0);
        }

        s.v[813] = if (s.v[123] == 0.5) { 1.0 } else { 0.0 };

        if (((s.v[418] != 0.0) && (!(s.v[812] != 0.0))) && (s.v[813] != 0.0)) {
            s.store_sqrt_ad(436, A::sub_from_scalar(1.0, A::mul(s.ad_value(428), s.ad_value(120))));
        }

        if (((s.v[418] != 0.0) && (!(s.v[812] != 0.0))) && (!(s.v[813] != 0.0))) {
            s.store_powf_ad(436, A::sub_from_scalar(1.0, A::mul(s.ad_value(428), s.ad_value(120))), s.v[123]);
        }

        if ((s.v[418] != 0.0) && (!(s.v[812] != 0.0))) {
            s.store_add_ad(271, A::mul(s.ad_value(132), A::sub_from_scalar(1.0, s.ad_value(436))), A::mul(s.ad_value(135), A::sub(s.ad_value(194), s.ad_value(428))));
        }

        if ((s.v[418] != 0.0) && (!(s.v[812] != 0.0))) {
            s.store_mul(437, 102, 371);
        }

        s.v[814] = if ((s.v[21] == 0.0) && (s.v[24] == 0.0)) { 1.0 } else { 0.0 };

        if (((s.v[418] != 0.0) && (!(s.v[812] != 0.0))) && (s.v[814] != 0.0)) {
            s.store_scalar(439, 0.0);
        }

        if (((s.v[418] != 0.0) && (!(s.v[812] != 0.0))) && (s.v[814] != 0.0)) {
            s.store_scalar(442, 0.0);
        }

        if (((s.v[418] != 0.0) && (!(s.v[812] != 0.0))) && (s.v[814] != 0.0)) {
            s.store_scalar(443, 0.0);
        }

        if (((s.v[418] != 0.0) && (!(s.v[812] != 0.0))) && (s.v[814] != 0.0)) {
            s.store_scalar(444, 0.0);
        }

        if (((s.v[418] != 0.0) && (!(s.v[812] != 0.0))) && (s.v[814] != 0.0)) {
            s.store_scalar(438, 0.0);
        }

        if (((s.v[418] != 0.0) && (!(s.v[812] != 0.0))) && (!(s.v[814] != 0.0))) {
            s.store_sub(439, 108, 433);
        }

        if (((s.v[418] != 0.0) && (!(s.v[812] != 0.0))) && (!(s.v[814] != 0.0))) {
            s.store_sub_from_scalar_ad(440, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(431), s.ad_value(439)))));
        }

        s.v[815] = if (s.v[10] == 0.5) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (!(s.v[812] != 0.0))) && (!(s.v[814] != 0.0))) && (s.v[815] != 0.0)) {
            s.store_scalar(441, 0.0);
        }

        if ((((s.v[418] != 0.0) && (!(s.v[812] != 0.0))) && (!(s.v[814] != 0.0))) && (!(s.v[815] != 0.0))) {
            s.store_scale_ad(441, A::add(A::div(A::mul(A::square(s.ad_value(440)), A::ln(s.ad_value(440))), A::sub_from_scalar(1.0, s.ad_value(440))), s.ad_value(440)), (1.0 - (2.0 * s.v[10])));
        }

        if (((s.v[418] != 0.0) && (!(s.v[812] != 0.0))) && (!(s.v[814] != 0.0))) {
            s.store_add(442, 440, 441);
        }

        s.v[816] = if (s.v[10] == 0.5) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (!(s.v[812] != 0.0))) && (!(s.v[814] != 0.0))) && (s.v[816] != 0.0)) {
            s.store_sqrt_ad(436, A::scale(s.ad_value(439), s.v[144]));
        }

        if ((((s.v[418] != 0.0) && (!(s.v[812] != 0.0))) && (!(s.v[814] != 0.0))) && (!(s.v[816] != 0.0))) {
            s.store_powf_ad(436, A::scale(s.ad_value(439), s.v[144]), s.v[10]);
        }

        if (((s.v[418] != 0.0) && (!(s.v[812] != 0.0))) && (!(s.v[814] != 0.0))) {
            s.store_scale(443, 436, s.v[138]);
        }

        if (((s.v[418] != 0.0) && (!(s.v[812] != 0.0))) && (!(s.v[814] != 0.0))) {
            s.store_mul_ad_rhs(444, 99, A::mul(A::offset(s.ad_value(430), (-1.0)), s.ad_value(443)));
        }

        if (((s.v[418] != 0.0) && (!(s.v[812] != 0.0))) && (!(s.v[814] != 0.0))) {
            s.store_scaled_mul(438, 444, 442, s.v[21]);
        }

        s.v[817] = if (s.v[24] == 0.0) { 1.0 } else { 0.0 };

        if (((s.v[418] != 0.0) && (!(s.v[812] != 0.0))) && (s.v[817] != 0.0)) {
            s.store_scalar(445, 0.0);
        }

    }

    pub(super) fn stamp_reactive_block_20(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if (((s.v[418] != 0.0) && (!(s.v[812] != 0.0))) && (!(s.v[817] != 0.0))) {
            s.store_scale_ad(446, A::div(A::scale(s.ad_value(443), s.v[123]), s.ad_value(439)), s.v[153]);
        }

        if (((s.v[418] != 0.0) && (!(s.v[812] != 0.0))) && (!(s.v[817] != 0.0))) {
            s.store_div_from_scalar(447, (0.666666666666667 * s.v[150]), 446);
        }

        if (((s.v[418] != 0.0) && (!(s.v[812] != 0.0))) && (!(s.v[817] != 0.0))) {
            s.store_square(448, 447);
        }

        if (((s.v[418] != 0.0) && (!(s.v[812] != 0.0))) && (!(s.v[817] != 0.0))) {
            s.store_sqrt_ad(449, A::div(A::square(s.ad_value(448)), A::offset(A::square(s.ad_value(448)), 1.0)));
        }

        if (((s.v[418] != 0.0) && (!(s.v[812] != 0.0))) && (!(s.v[817] != 0.0))) {
            s.store_sqrt_ad(450, A::abs(s.ad_value(449)));
        }

        if (((s.v[418] != 0.0) && (!(s.v[812] != 0.0))) && (!(s.v[817] != 0.0))) {
            s.store_mul(451, 449, 450);
        }

        s.v[818] = if (((-s.v[10]) * s.v[126]) == (-1.0)) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (!(s.v[812] != 0.0))) && (!(s.v[817] != 0.0))) && (s.v[818] != 0.0)) {
            s.store_div_from_scalar_ad(452, 1.0, A::offset(A::mul(s.ad_value(446), s.ad_value(451)), 1.0));
        }

        if ((((s.v[418] != 0.0) && (!(s.v[812] != 0.0))) && (!(s.v[817] != 0.0))) && (!(s.v[818] != 0.0))) {
            s.store_powf_ad(452, A::offset(A::mul(s.ad_value(446), s.ad_value(451)), 1.0), ((-s.v[10]) * s.v[126]));
        }

        if (((s.v[418] != 0.0) && (!(s.v[812] != 0.0))) && (!(s.v[817] != 0.0))) {
            s.store_div_ad(453, A::mul(s.ad_value(442), s.ad_value(452)), A::add(s.ad_value(442), s.ad_value(452)));
        }

        if (((s.v[418] != 0.0) && (!(s.v[812] != 0.0))) && (!(s.v[817] != 0.0))) {
            s.store_sqrt_ad(454, A::scale(A::div(s.ad_value(446), s.ad_value(450)), 0.375));
        }

        if (((s.v[418] != 0.0) && (!(s.v[812] != 0.0))) && (!(s.v[817] != 0.0))) {
            s.store_sub_ad_lhs(455, A::scale(A::mul(s.ad_value(447), s.ad_value(450)), 2.0), 449);
        }

        if (((s.v[418] != 0.0) && (!(s.v[812] != 0.0))) && (!(s.v[817] != 0.0))) {
            s.store_add_ad(456, A::sub(A::mul(A::scale(s.ad_value(447), s.v[150]), s.ad_value(450)), A::scale(s.ad_value(449), s.v[150])), A::scale(A::mul(s.ad_value(446), s.ad_value(451)), 0.5));
        }

        if (((s.v[418] != 0.0) && (!(s.v[812] != 0.0))) && (!(s.v[817] != 0.0))) {
            s.store_mul_ad_lhs(457, A::offset(s.ad_value(455), (-1.0)), 454);
        }

        if (((s.v[418] != 0.0) && (!(s.v[812] != 0.0))) && (!(s.v[817] != 0.0))) {
            s.store_square(419, 457);
        }

        s.v[819] = if (s.v[457] > 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (!(s.v[812] != 0.0))) && (!(s.v[817] != 0.0))) && (s.v[819] != 0.0)) {
            s.store_div_from_scalar_ad(420, 1.0, A::offset(A::scale(s.ad_value(457), s.v[86]), 1.0));
        }

        if ((((s.v[418] != 0.0) && (!(s.v[812] != 0.0))) && (!(s.v[817] != 0.0))) && (!(s.v[819] != 0.0))) {
            s.store_div_from_scalar_ad(420, 1.0, A::sub_from_scalar(1.0, A::scale(s.ad_value(457), s.v[86])));
        }

        s.v[820] = if (((-s.v[419]) + s.v[456]) > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (!(s.v[812] != 0.0))) && (!(s.v[817] != 0.0))) && (s.v[820] != 0.0)) {
            s.store_exp_ad(436, A::sub(s.ad_value(456), s.ad_value(419)));
        }

        if ((((s.v[418] != 0.0) && (!(s.v[812] != 0.0))) && (!(s.v[817] != 0.0))) && (!(s.v[820] != 0.0))) {
            s.store_div_from_scalar_ad(436, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(456), s.ad_value(419))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(456), s.ad_value(419))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(456), s.ad_value(419))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if (((s.v[418] != 0.0) && (!(s.v[812] != 0.0))) && (!(s.v[817] != 0.0))) {
            s.store_mul_ad_lhs(421, A::add(A::add(A::scale(s.ad_value(420), 0.29214664), A::scale(A::square(s.ad_value(420)), s.v[87])), A::scale(A::mul(A::square(s.ad_value(420)), s.ad_value(420)), s.v[88])), 436);
        }

        s.v[821] = if (s.v[457] > 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (!(s.v[812] != 0.0))) && (!(s.v[817] != 0.0))) && (s.v[821] != 0.0)) {
            s.copy_ad(458, 421);
        }

        s.v[822] = if (s.v[456] > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((s.v[418] != 0.0) && (!(s.v[812] != 0.0))) && (!(s.v[817] != 0.0))) && (!(s.v[821] != 0.0))) && (s.v[822] != 0.0)) {
            s.store_exp(436, 456);
        }

        if (((((s.v[418] != 0.0) && (!(s.v[812] != 0.0))) && (!(s.v[817] != 0.0))) && (!(s.v[821] != 0.0))) && (!(s.v[822] != 0.0))) {
            s.store_div_from_scalar_ad(436, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(456)), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(456)), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), s.ad_value(456)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if ((((s.v[418] != 0.0) && (!(s.v[812] != 0.0))) && (!(s.v[817] != 0.0))) && (!(s.v[821] != 0.0))) {
            s.store_sub_ad_lhs(458, A::scale(s.ad_value(436), 2.0), 421);
        }

        if (((s.v[418] != 0.0) && (!(s.v[812] != 0.0))) && (!(s.v[817] != 0.0))) {
            s.store_scale_ad(459, A::div(A::scale(s.ad_value(458), s.v[150]), s.ad_value(454)), (1.772453850905516 * 0.5));
        }

        if (((s.v[418] != 0.0) && (!(s.v[812] != 0.0))) && (!(s.v[817] != 0.0))) {
            s.store_scale_ad(445, A::mul(A::mul(s.ad_value(444), s.ad_value(459)), s.ad_value(453)), s.v[24]);
        }

        s.v[823] = if (s.v[30] == 0.0) { 1.0 } else { 0.0 };

        if (((s.v[418] != 0.0) && (!(s.v[812] != 0.0))) && (s.v[823] != 0.0)) {
            s.store_scalar(460, 0.0);
        }

        s.v[824] = if (s.v[10] == 0.5) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (!(s.v[812] != 0.0))) && (!(s.v[823] != 0.0))) && (s.v[824] != 0.0)) {
            s.store_sqrt_ad(436, A::scale(A::sub_from_scalar(s.v[7], s.ad_value(434)), s.v[144]));
        }

        if ((((s.v[418] != 0.0) && (!(s.v[812] != 0.0))) && (!(s.v[823] != 0.0))) && (!(s.v[824] != 0.0))) {
            s.store_powf_ad(436, A::scale(A::sub_from_scalar(s.v[7], s.ad_value(434)), s.v[144]), s.v[10]);
        }

        if (((s.v[418] != 0.0) && (!(s.v[812] != 0.0))) && (!(s.v[823] != 0.0))) {
            s.store_scale_ad(461, A::div(A::scale(A::sub_from_scalar(s.v[7], s.ad_value(434)), s.v[141]), s.ad_value(436)), s.v[126]);
        }

        s.v[825] = if (((((-s.v[156]) / s.v[461])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (!(s.v[812] != 0.0))) && (!(s.v[823] != 0.0))) && (s.v[825] != 0.0)) {
            s.store_exp_ad(436, A::div(A::neg(s.ad_value(156)), s.ad_value(461)));
        }

        s.v[826] = if (((-s.v[156]) / s.v[461]) < (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((s.v[418] != 0.0) && (!(s.v[812] != 0.0))) && (!(s.v[823] != 0.0))) && (!(s.v[825] != 0.0))) && (s.v[826] != 0.0)) {
            let assign27350_ad_e40632: A = A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(156)), s.ad_value(461))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(156)), s.ad_value(461))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(156)), s.ad_value(461))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
            s.store_div_from_scalar_ad(436, 1e-100, assign27350_ad_e40632);
        }

        if (((((s.v[418] != 0.0) && (!(s.v[812] != 0.0))) && (!(s.v[823] != 0.0))) && (!(s.v[825] != 0.0))) && (!(s.v[826] != 0.0))) {
            let assign27360_ad_e40680: A = A::scale(A::offset(A::mul(A::offset(A::div(A::neg(s.ad_value(156)), s.ad_value(461)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::div(A::neg(s.ad_value(156)), s.ad_value(461)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::div(A::neg(s.ad_value(156)), s.ad_value(461)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
            s.store_ad(436, &assign27360_ad_e40680);
        }

        if (((s.v[418] != 0.0) && (!(s.v[812] != 0.0))) && (!(s.v[823] != 0.0))) {
            s.store_scale_ad(460, A::mul(A::mul(A::mul(s.ad_value(194), s.ad_value(461)), s.ad_value(461)), s.ad_value(436)), s.v[30]);
        }

        s.v[827] = if ((s.v[39] > 1000000.0) || (p.p80 == 0.0)) { 1.0 } else { 0.0 };

        if (((s.v[418] != 0.0) && (!(s.v[812] != 0.0))) && (s.v[827] != 0.0)) {
            s.store_scalar(462, 1.0);
        }

        s.v[828] = if (s.v[435] > ((-s.v[158]) * s.v[39])) { 1.0 } else { 0.0 };

        s.v[829] = if (s.v[42] == 4.0) { 1.0 } else { 0.0 };

        if (((((s.v[418] != 0.0) && (!(s.v[812] != 0.0))) && (!(s.v[827] != 0.0))) && (s.v[828] != 0.0)) && (s.v[829] != 0.0)) {
            s.store_mul_ad(436, A::mul(A::mul(A::abs(A::mul(s.ad_value(435), s.ad_value(163))), A::abs(A::mul(s.ad_value(435), s.ad_value(163)))), A::abs(A::mul(s.ad_value(435), s.ad_value(163)))), A::abs(A::mul(s.ad_value(435), s.ad_value(163))));
        }

        if (((((s.v[418] != 0.0) && (!(s.v[812] != 0.0))) && (!(s.v[827] != 0.0))) && (s.v[828] != 0.0)) && (!(s.v[829] != 0.0))) {
            s.store_powf_ad(436, A::abs(A::mul(s.ad_value(435), s.ad_value(163))), s.v[42]);
        }

        if ((((s.v[418] != 0.0) && (!(s.v[812] != 0.0))) && (!(s.v[827] != 0.0))) && (s.v[828] != 0.0)) {
            s.store_div_from_scalar_ad(462, 1.0, A::sub_from_scalar(1.0, s.ad_value(436)));
        }

        if ((((s.v[418] != 0.0) && (!(s.v[812] != 0.0))) && (!(s.v[827] != 0.0))) && (!(s.v[828] != 0.0))) {
            s.store_offset_ad(462, A::mul(A::add(s.ad_value(435), A::scale(s.ad_value(39), s.v[158])), s.ad_value(166)), s.v[160]);
        }

        if ((s.v[418] != 0.0) && (!(s.v[812] != 0.0))) {
            s.store_mul_ad_lhs(270, A::add(A::add(A::add(s.ad_value(437), s.ad_value(438)), s.ad_value(445)), s.ad_value(460)), 462);
        }

        if ((s.v[418] != 0.0) && (!(s.v[812] != 0.0))) {
            s.store_mul_ad_lhs(292, A::add(A::add(s.ad_value(438), s.ad_value(445)), s.ad_value(460)), 462);
        }

        s.v[830] = if (s.v[258] == 0.0) { 1.0 } else { 0.0 };

        if ((s.v[418] != 0.0) && (s.v[830] != 0.0)) {
            s.store_scalar(272, 0.0);
        }

        if ((s.v[418] != 0.0) && (s.v[830] != 0.0)) {
            s.store_scalar(293, 0.0);
        }

        if ((s.v[418] != 0.0) && (s.v[830] != 0.0)) {
            s.store_scalar(273, 0.0);
        }

        s.v[831] = if (s.v[124] == 0.5) { 1.0 } else { 0.0 };

        if (((s.v[418] != 0.0) && (!(s.v[830] != 0.0))) && (s.v[831] != 0.0)) {
            s.store_sqrt_ad(436, A::sub_from_scalar(1.0, A::mul(s.ad_value(428), s.ad_value(121))));
        }

        if (((s.v[418] != 0.0) && (!(s.v[830] != 0.0))) && (!(s.v[831] != 0.0))) {
            s.store_powf_ad(436, A::sub_from_scalar(1.0, A::mul(s.ad_value(428), s.ad_value(121))), s.v[124]);
        }

        if ((s.v[418] != 0.0) && (!(s.v[830] != 0.0))) {
            s.store_add_ad(273, A::mul(s.ad_value(133), A::sub_from_scalar(1.0, s.ad_value(436))), A::mul(s.ad_value(136), A::sub(s.ad_value(194), s.ad_value(428))));
        }

        if ((s.v[418] != 0.0) && (!(s.v[830] != 0.0))) {
            s.store_mul(437, 103, 372);
        }

        s.v[832] = if ((s.v[22] == 0.0) && (s.v[25] == 0.0)) { 1.0 } else { 0.0 };

        if (((s.v[418] != 0.0) && (!(s.v[830] != 0.0))) && (s.v[832] != 0.0)) {
            s.store_scalar(439, 0.0);
        }

        if (((s.v[418] != 0.0) && (!(s.v[830] != 0.0))) && (s.v[832] != 0.0)) {
            s.store_scalar(442, 0.0);
        }

        if (((s.v[418] != 0.0) && (!(s.v[830] != 0.0))) && (s.v[832] != 0.0)) {
            s.store_scalar(443, 0.0);
        }

        if (((s.v[418] != 0.0) && (!(s.v[830] != 0.0))) && (s.v[832] != 0.0)) {
            s.store_scalar(444, 0.0);
        }

        if (((s.v[418] != 0.0) && (!(s.v[830] != 0.0))) && (s.v[832] != 0.0)) {
            s.store_scalar(438, 0.0);
        }

        if (((s.v[418] != 0.0) && (!(s.v[830] != 0.0))) && (!(s.v[832] != 0.0))) {
            s.store_sub(439, 109, 433);
        }

        if (((s.v[418] != 0.0) && (!(s.v[830] != 0.0))) && (!(s.v[832] != 0.0))) {
            s.store_sub_from_scalar_ad(440, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(431), s.ad_value(439)))));
        }

        s.v[833] = if (s.v[11] == 0.5) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (!(s.v[830] != 0.0))) && (!(s.v[832] != 0.0))) && (s.v[833] != 0.0)) {
            s.store_scalar(441, 0.0);
        }

        if ((((s.v[418] != 0.0) && (!(s.v[830] != 0.0))) && (!(s.v[832] != 0.0))) && (!(s.v[833] != 0.0))) {
            s.store_scale_ad(441, A::add(A::div(A::mul(A::square(s.ad_value(440)), A::ln(s.ad_value(440))), A::sub_from_scalar(1.0, s.ad_value(440))), s.ad_value(440)), (1.0 - (2.0 * s.v[11])));
        }

        if (((s.v[418] != 0.0) && (!(s.v[830] != 0.0))) && (!(s.v[832] != 0.0))) {
            s.store_add(442, 440, 441);
        }

        s.v[834] = if (s.v[11] == 0.5) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (!(s.v[830] != 0.0))) && (!(s.v[832] != 0.0))) && (s.v[834] != 0.0)) {
            s.store_sqrt_ad(436, A::scale(s.ad_value(439), s.v[145]));
        }

        if ((((s.v[418] != 0.0) && (!(s.v[830] != 0.0))) && (!(s.v[832] != 0.0))) && (!(s.v[834] != 0.0))) {
            s.store_powf_ad(436, A::scale(s.ad_value(439), s.v[145]), s.v[11]);
        }

        if (((s.v[418] != 0.0) && (!(s.v[830] != 0.0))) && (!(s.v[832] != 0.0))) {
            s.store_scale(443, 436, s.v[139]);
        }

        if (((s.v[418] != 0.0) && (!(s.v[830] != 0.0))) && (!(s.v[832] != 0.0))) {
            s.store_mul_ad_rhs(444, 100, A::mul(A::offset(s.ad_value(430), (-1.0)), s.ad_value(443)));
        }

        if (((s.v[418] != 0.0) && (!(s.v[830] != 0.0))) && (!(s.v[832] != 0.0))) {
            s.store_scaled_mul(438, 444, 442, s.v[22]);
        }

        s.v[835] = if (s.v[25] == 0.0) { 1.0 } else { 0.0 };

        if (((s.v[418] != 0.0) && (!(s.v[830] != 0.0))) && (s.v[835] != 0.0)) {
            s.store_scalar(445, 0.0);
        }

        if (((s.v[418] != 0.0) && (!(s.v[830] != 0.0))) && (!(s.v[835] != 0.0))) {
            s.store_scale_ad(446, A::div(A::scale(s.ad_value(443), s.v[124]), s.ad_value(439)), s.v[154]);
        }

        if (((s.v[418] != 0.0) && (!(s.v[830] != 0.0))) && (!(s.v[835] != 0.0))) {
            s.store_div_from_scalar(447, (0.666666666666667 * s.v[151]), 446);
        }

        if (((s.v[418] != 0.0) && (!(s.v[830] != 0.0))) && (!(s.v[835] != 0.0))) {
            s.store_square(448, 447);
        }

        if (((s.v[418] != 0.0) && (!(s.v[830] != 0.0))) && (!(s.v[835] != 0.0))) {
            s.store_sqrt_ad(449, A::div(A::square(s.ad_value(448)), A::offset(A::square(s.ad_value(448)), 1.0)));
        }

        if (((s.v[418] != 0.0) && (!(s.v[830] != 0.0))) && (!(s.v[835] != 0.0))) {
            s.store_sqrt_ad(450, A::abs(s.ad_value(449)));
        }

        if (((s.v[418] != 0.0) && (!(s.v[830] != 0.0))) && (!(s.v[835] != 0.0))) {
            s.store_mul(451, 449, 450);
        }

        s.v[836] = if (((-s.v[11]) * s.v[127]) == (-1.0)) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (!(s.v[830] != 0.0))) && (!(s.v[835] != 0.0))) && (s.v[836] != 0.0)) {
            s.store_div_from_scalar_ad(452, 1.0, A::offset(A::mul(s.ad_value(446), s.ad_value(451)), 1.0));
        }

        if ((((s.v[418] != 0.0) && (!(s.v[830] != 0.0))) && (!(s.v[835] != 0.0))) && (!(s.v[836] != 0.0))) {
            s.store_powf_ad(452, A::offset(A::mul(s.ad_value(446), s.ad_value(451)), 1.0), ((-s.v[11]) * s.v[127]));
        }

        if (((s.v[418] != 0.0) && (!(s.v[830] != 0.0))) && (!(s.v[835] != 0.0))) {
            s.store_div_ad(453, A::mul(s.ad_value(442), s.ad_value(452)), A::add(s.ad_value(442), s.ad_value(452)));
        }

        if (((s.v[418] != 0.0) && (!(s.v[830] != 0.0))) && (!(s.v[835] != 0.0))) {
            s.store_sqrt_ad(454, A::scale(A::div(s.ad_value(446), s.ad_value(450)), 0.375));
        }

        if (((s.v[418] != 0.0) && (!(s.v[830] != 0.0))) && (!(s.v[835] != 0.0))) {
            s.store_sub_ad_lhs(455, A::scale(A::mul(s.ad_value(447), s.ad_value(450)), 2.0), 449);
        }

        if (((s.v[418] != 0.0) && (!(s.v[830] != 0.0))) && (!(s.v[835] != 0.0))) {
            s.store_add_ad(456, A::sub(A::mul(A::scale(s.ad_value(447), s.v[151]), s.ad_value(450)), A::scale(s.ad_value(449), s.v[151])), A::scale(A::mul(s.ad_value(446), s.ad_value(451)), 0.5));
        }

        if (((s.v[418] != 0.0) && (!(s.v[830] != 0.0))) && (!(s.v[835] != 0.0))) {
            s.store_mul_ad_lhs(457, A::offset(s.ad_value(455), (-1.0)), 454);
        }

        if (((s.v[418] != 0.0) && (!(s.v[830] != 0.0))) && (!(s.v[835] != 0.0))) {
            s.store_square(419, 457);
        }

        s.v[837] = if (s.v[457] > 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (!(s.v[830] != 0.0))) && (!(s.v[835] != 0.0))) && (s.v[837] != 0.0)) {
            s.store_div_from_scalar_ad(420, 1.0, A::offset(A::scale(s.ad_value(457), s.v[86]), 1.0));
        }

        if ((((s.v[418] != 0.0) && (!(s.v[830] != 0.0))) && (!(s.v[835] != 0.0))) && (!(s.v[837] != 0.0))) {
            s.store_div_from_scalar_ad(420, 1.0, A::sub_from_scalar(1.0, A::scale(s.ad_value(457), s.v[86])));
        }

        s.v[838] = if (((-s.v[419]) + s.v[456]) > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (!(s.v[830] != 0.0))) && (!(s.v[835] != 0.0))) && (s.v[838] != 0.0)) {
            s.store_exp_ad(436, A::sub(s.ad_value(456), s.ad_value(419)));
        }

        if ((((s.v[418] != 0.0) && (!(s.v[830] != 0.0))) && (!(s.v[835] != 0.0))) && (!(s.v[838] != 0.0))) {
            s.store_div_from_scalar_ad(436, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(456), s.ad_value(419))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(456), s.ad_value(419))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(456), s.ad_value(419))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if (((s.v[418] != 0.0) && (!(s.v[830] != 0.0))) && (!(s.v[835] != 0.0))) {
            s.store_mul_ad_lhs(421, A::add(A::add(A::scale(s.ad_value(420), 0.29214664), A::scale(A::square(s.ad_value(420)), s.v[87])), A::scale(A::mul(A::square(s.ad_value(420)), s.ad_value(420)), s.v[88])), 436);
        }

        s.v[839] = if (s.v[457] > 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (!(s.v[830] != 0.0))) && (!(s.v[835] != 0.0))) && (s.v[839] != 0.0)) {
            s.copy_ad(458, 421);
        }

        s.v[840] = if (s.v[456] > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((s.v[418] != 0.0) && (!(s.v[830] != 0.0))) && (!(s.v[835] != 0.0))) && (!(s.v[839] != 0.0))) && (s.v[840] != 0.0)) {
            s.store_exp(436, 456);
        }

        if (((((s.v[418] != 0.0) && (!(s.v[830] != 0.0))) && (!(s.v[835] != 0.0))) && (!(s.v[839] != 0.0))) && (!(s.v[840] != 0.0))) {
            s.store_div_from_scalar_ad(436, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(456)), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(456)), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), s.ad_value(456)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if ((((s.v[418] != 0.0) && (!(s.v[830] != 0.0))) && (!(s.v[835] != 0.0))) && (!(s.v[839] != 0.0))) {
            s.store_sub_ad_lhs(458, A::scale(s.ad_value(436), 2.0), 421);
        }

        if (((s.v[418] != 0.0) && (!(s.v[830] != 0.0))) && (!(s.v[835] != 0.0))) {
            s.store_scale_ad(459, A::div(A::scale(s.ad_value(458), s.v[151]), s.ad_value(454)), (1.772453850905516 * 0.5));
        }

        if (((s.v[418] != 0.0) && (!(s.v[830] != 0.0))) && (!(s.v[835] != 0.0))) {
            s.store_scale_ad(445, A::mul(A::mul(s.ad_value(444), s.ad_value(459)), s.ad_value(453)), s.v[25]);
        }

        s.v[841] = if (s.v[31] == 0.0) { 1.0 } else { 0.0 };

        if (((s.v[418] != 0.0) && (!(s.v[830] != 0.0))) && (s.v[841] != 0.0)) {
            s.store_scalar(460, 0.0);
        }

        s.v[842] = if (s.v[11] == 0.5) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (!(s.v[830] != 0.0))) && (!(s.v[841] != 0.0))) && (s.v[842] != 0.0)) {
            s.store_sqrt_ad(436, A::scale(A::sub_from_scalar(s.v[8], s.ad_value(434)), s.v[145]));
        }

        if ((((s.v[418] != 0.0) && (!(s.v[830] != 0.0))) && (!(s.v[841] != 0.0))) && (!(s.v[842] != 0.0))) {
            s.store_powf_ad(436, A::scale(A::sub_from_scalar(s.v[8], s.ad_value(434)), s.v[145]), s.v[11]);
        }

        if (((s.v[418] != 0.0) && (!(s.v[830] != 0.0))) && (!(s.v[841] != 0.0))) {
            s.store_scale_ad(461, A::div(A::scale(A::sub_from_scalar(s.v[8], s.ad_value(434)), s.v[142]), s.ad_value(436)), s.v[127]);
        }

        s.v[843] = if (((((-s.v[157]) / s.v[461])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (!(s.v[830] != 0.0))) && (!(s.v[841] != 0.0))) && (s.v[843] != 0.0)) {
            s.store_exp_ad(436, A::div(A::neg(s.ad_value(157)), s.ad_value(461)));
        }

        s.v[844] = if (((-s.v[157]) / s.v[461]) < (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((s.v[418] != 0.0) && (!(s.v[830] != 0.0))) && (!(s.v[841] != 0.0))) && (!(s.v[843] != 0.0))) && (s.v[844] != 0.0)) {
            let assign28160_ad_e41788: A = A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(157)), s.ad_value(461))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(157)), s.ad_value(461))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(157)), s.ad_value(461))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
            s.store_div_from_scalar_ad(436, 1e-100, assign28160_ad_e41788);
        }

        if (((((s.v[418] != 0.0) && (!(s.v[830] != 0.0))) && (!(s.v[841] != 0.0))) && (!(s.v[843] != 0.0))) && (!(s.v[844] != 0.0))) {
            let assign28170_ad_e41836: A = A::scale(A::offset(A::mul(A::offset(A::div(A::neg(s.ad_value(157)), s.ad_value(461)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::div(A::neg(s.ad_value(157)), s.ad_value(461)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::div(A::neg(s.ad_value(157)), s.ad_value(461)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
            s.store_ad(436, &assign28170_ad_e41836);
        }

        if (((s.v[418] != 0.0) && (!(s.v[830] != 0.0))) && (!(s.v[841] != 0.0))) {
            s.store_scale_ad(460, A::mul(A::mul(A::mul(s.ad_value(194), s.ad_value(461)), s.ad_value(461)), s.ad_value(436)), s.v[31]);
        }

        s.v[845] = if ((s.v[40] > 1000000.0) || (p.p80 == 0.0)) { 1.0 } else { 0.0 };

        if (((s.v[418] != 0.0) && (!(s.v[830] != 0.0))) && (s.v[845] != 0.0)) {
            s.store_scalar(462, 1.0);
        }

        s.v[846] = if (s.v[435] > ((-s.v[158]) * s.v[40])) { 1.0 } else { 0.0 };

        s.v[847] = if (s.v[43] == 4.0) { 1.0 } else { 0.0 };

        if (((((s.v[418] != 0.0) && (!(s.v[830] != 0.0))) && (!(s.v[845] != 0.0))) && (s.v[846] != 0.0)) && (s.v[847] != 0.0)) {
            s.store_mul_ad(436, A::mul(A::mul(A::abs(A::mul(s.ad_value(435), s.ad_value(164))), A::abs(A::mul(s.ad_value(435), s.ad_value(164)))), A::abs(A::mul(s.ad_value(435), s.ad_value(164)))), A::abs(A::mul(s.ad_value(435), s.ad_value(164))));
        }

        if (((((s.v[418] != 0.0) && (!(s.v[830] != 0.0))) && (!(s.v[845] != 0.0))) && (s.v[846] != 0.0)) && (!(s.v[847] != 0.0))) {
            s.store_powf_ad(436, A::abs(A::mul(s.ad_value(435), s.ad_value(164))), s.v[43]);
        }

        if ((((s.v[418] != 0.0) && (!(s.v[830] != 0.0))) && (!(s.v[845] != 0.0))) && (s.v[846] != 0.0)) {
            s.store_div_from_scalar_ad(462, 1.0, A::sub_from_scalar(1.0, s.ad_value(436)));
        }

        if ((((s.v[418] != 0.0) && (!(s.v[830] != 0.0))) && (!(s.v[845] != 0.0))) && (!(s.v[846] != 0.0))) {
            s.store_offset_ad(462, A::mul(A::add(s.ad_value(435), A::scale(s.ad_value(40), s.v[158])), s.ad_value(167)), s.v[161]);
        }

        if ((s.v[418] != 0.0) && (!(s.v[830] != 0.0))) {
            s.store_mul_ad_lhs(272, A::add(A::add(A::add(s.ad_value(437), s.ad_value(438)), s.ad_value(445)), s.ad_value(460)), 462);
        }

        if ((s.v[418] != 0.0) && (!(s.v[830] != 0.0))) {
            s.store_mul_ad_lhs(293, A::add(A::add(s.ad_value(438), s.ad_value(445)), s.ad_value(460)), 462);
        }

        if (s.v[418] != 0.0) {
            s.store_add_ad(184, A::add(A::scale(s.ad_value(268), s.v[256]), A::scale(s.ad_value(270), s.v[257])), A::scale(s.ad_value(272), s.v[258]));
        }

        if (s.v[418] != 0.0) {
            s.copy_ad(300, 289);
        }

        if (s.v[418] != 0.0) {
            s.store_sub_ad_rhs(188, 183, A::mul(s.ad_value(300), A::offset(A::exp(A::mul(A::scale(s.ad_value(193), s.v[85]), s.ad_value(301))), (-1.0))));
        }

        if (s.v[418] != 0.0) {
            s.store_sub_ad_rhs(189, 184, A::mul(s.ad_value(300), A::offset(A::exp(A::mul(A::scale(s.ad_value(194), s.v[85]), s.ad_value(301))), (-1.0))));
        }

        s.v[848] = if !(((s.v[256] == 0.0) && (s.v[257] == 0.0)) && (s.v[258] == 0.0)) { 1.0 } else { 0.0 };

        s.v[849] = if ((s.v[183] > 0.0) && (s.v[184] > 0.0)) { 1.0 } else { 0.0 };

        s.v[850] = if (((((s.v[188] / s.v[183]) > 0.001) || ((s.v[189] / s.v[184]) > 0.001)) && (s.v[188] > 0.0)) && (s.v[189] > 0.0)) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (s.v[848] != 0.0)) && (s.v[849] != 0.0)) && (s.v[850] != 0.0)) {
            s.store_div(195, 188, 189);
        }

        if ((((s.v[418] != 0.0) && (s.v[848] != 0.0)) && (s.v[849] != 0.0)) && (s.v[850] != 0.0)) {
            s.store_div_ad(303, A::scale(A::ln(s.ad_value(195)), s.v[84]), A::sub(s.ad_value(193), s.ad_value(194)));
        }

        if ((((s.v[418] != 0.0) && (s.v[848] != 0.0)) && (s.v[849] != 0.0)) && (s.v[850] != 0.0)) {
            s.store_div_ad_rhs(302, 188, A::offset(A::exp(A::mul(A::scale(s.ad_value(193), s.v[85]), s.ad_value(303))), (-1.0)));
        }

    }
}
