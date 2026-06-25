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

        if ((s.v[418] != 0.0) && (s.v[848] != 0.0)) {
            s.store_sub_ad(185, A::sub(s.ad_value(180), A::mul(s.ad_value(300), A::offset(A::exp(A::mul(A::scale(s.ad_value(190), s.v[85]), s.ad_value(301))), (-1.0)))), A::mul(s.ad_value(302), A::offset(A::exp(A::mul(A::scale(s.ad_value(190), s.v[85]), s.ad_value(303))), (-1.0))));
        }

        if ((s.v[418] != 0.0) && (s.v[848] != 0.0)) {
            s.store_sub_ad(186, A::sub(s.ad_value(181), A::mul(s.ad_value(300), A::offset(A::exp(A::mul(A::scale(s.ad_value(191), s.v[85]), s.ad_value(301))), (-1.0)))), A::mul(s.ad_value(302), A::offset(A::exp(A::mul(A::scale(s.ad_value(191), s.v[85]), s.ad_value(303))), (-1.0))));
        }

        if ((s.v[418] != 0.0) && (s.v[848] != 0.0)) {
            s.store_sub_ad(187, A::sub(s.ad_value(182), A::mul(s.ad_value(300), A::offset(A::exp(A::mul(A::scale(s.ad_value(192), s.v[85]), s.ad_value(301))), (-1.0)))), A::mul(s.ad_value(302), A::offset(A::exp(A::mul(A::scale(s.ad_value(192), s.v[85]), s.ad_value(303))), (-1.0))));
        }

        s.v[851] = if (((s.v[180] < 0.0) && (s.v[181] < 0.0)) && (s.v[182] < 0.0)) { 1.0 } else { 0.0 };

        s.v[852] = if (((((((s.v[185] / s.v[180]) > 0.001) || ((s.v[186] / s.v[181]) > 0.001)) || ((s.v[187] / s.v[182]) > 0.001)) && (s.v[185] < 0.0)) && (s.v[186] < 0.0)) && (s.v[187] < 0.0)) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (s.v[848] != 0.0)) && (s.v[851] != 0.0)) && (s.v[852] != 0.0)) {
            s.store_div(195, 185, 186);
        }

        if ((((s.v[418] != 0.0) && (s.v[848] != 0.0)) && (s.v[851] != 0.0)) && (s.v[852] != 0.0)) {
            s.store_div_ad(196, A::scale(A::ln(s.ad_value(195)), (-s.v[84])), A::sub(s.ad_value(190), s.ad_value(191)));
        }

        if ((((s.v[418] != 0.0) && (s.v[848] != 0.0)) && (s.v[851] != 0.0)) && (s.v[852] != 0.0)) {
            s.store_div_ad_rhs(198, 191, A::sub(s.ad_value(191), s.ad_value(190)));
        }

        if ((((s.v[418] != 0.0) && (s.v[848] != 0.0)) && (s.v[851] != 0.0)) && (s.v[852] != 0.0)) {
            s.store_mul_ad(199, A::scale(A::offset(s.ad_value(195), (-1.0)), s.v[84]), A::offset(A::pow(s.ad_value(195), s.ad_value(198)), (-1.0)));
        }

        if ((((s.v[418] != 0.0) && (s.v[848] != 0.0)) && (s.v[851] != 0.0)) && (s.v[852] != 0.0)) {
            s.store_div_ad_rhs(198, 190, A::sub(s.ad_value(190), s.ad_value(191)));
        }

        if ((((s.v[418] != 0.0) && (s.v[848] != 0.0)) && (s.v[851] != 0.0)) && (s.v[852] != 0.0)) {
            s.store_sub_ad_lhs(200, A::add(A::mul(A::pow(s.ad_value(195), s.ad_value(198)), A::sub(s.ad_value(191), s.ad_value(190))), A::mul(s.ad_value(195), s.ad_value(190))), 191);
        }

        if ((((s.v[418] != 0.0) && (s.v[848] != 0.0)) && (s.v[851] != 0.0)) && (s.v[852] != 0.0)) {
            s.store_div(197, 199, 200);
        }

        if ((((s.v[418] != 0.0) && (s.v[848] != 0.0)) && (s.v[851] != 0.0)) && (s.v[852] != 0.0)) {
            s.store_add(305, 196, 197);
        }

        s.v[853] = if (((((s.v[192] * s.v[85]) * s.v[305])) as f64).abs() < 1e-6) { 1.0 } else { 0.0 };

        if (((((s.v[418] != 0.0) && (s.v[848] != 0.0)) && (s.v[851] != 0.0)) && (s.v[852] != 0.0)) && (s.v[853] != 0.0)) {
            s.store_scalar(306, 1.0);
        }

        if (((((s.v[418] != 0.0) && (s.v[848] != 0.0)) && (s.v[851] != 0.0)) && (s.v[852] != 0.0)) && (s.v[853] != 0.0)) {
            s.store_mul_ad_rhs(304, 187, A::add(A::div_from_scalar(1.0, s.ad_value(192)), A::scale(s.ad_value(305), (0.5 * s.v[85]))));
        }

        if (((((s.v[418] != 0.0) && (s.v[848] != 0.0)) && (s.v[851] != 0.0)) && (s.v[852] != 0.0)) && (s.v[853] != 0.0)) {
            s.store_div_ad_lhs(305, A::scale(A::mul(A::scale(s.ad_value(187), (-0.5)), s.ad_value(305)), s.v[85]), 192);
        }

        if (((((s.v[418] != 0.0) && (s.v[848] != 0.0)) && (s.v[851] != 0.0)) && (s.v[852] != 0.0)) && (!(s.v[853] != 0.0))) {
            s.store_scalar(306, 0.0);
        }

        if (((((s.v[418] != 0.0) && (s.v[848] != 0.0)) && (s.v[851] != 0.0)) && (s.v[852] != 0.0)) && (!(s.v[853] != 0.0))) {
            s.store_div_ad(304, A::neg(s.ad_value(187)), A::offset(A::exp(A::mul(A::scale(A::neg(s.ad_value(192)), s.v[85]), s.ad_value(305))), (-1.0)));
        }

        if (s.v[418] != 0.0) {
            s.store_scale_ad(208, A::add(A::add(A::scale(s.ad_value(128), s.v[256]), A::scale(s.ad_value(129), s.v[257])), A::scale(s.ad_value(130), s.v[258])), s.v[47]);
        }

        s.v[854] = if ((s.v[256] * s.v[128]) <= s.v[208]) { 1.0 } else { 0.0 };

        if ((s.v[418] != 0.0) && (s.v[854] != 0.0)) {
            s.store_scalar(259, 0.0);
        }

        s.v[855] = if ((s.v[257] * s.v[129]) <= s.v[208]) { 1.0 } else { 0.0 };

        if ((s.v[418] != 0.0) && (s.v[855] != 0.0)) {
            s.store_scalar(260, 0.0);
        }

        s.v[856] = if ((s.v[258] * s.v[130]) <= s.v[208]) { 1.0 } else { 0.0 };

        if ((s.v[418] != 0.0) && (s.v[856] != 0.0)) {
            s.store_scalar(261, 0.0);
        }

        s.v[857] = if !(((s.v[256] == 0.0) && (s.v[257] == 0.0)) && (s.v[258] == 0.0)) { 1.0 } else { 0.0 };

        if ((s.v[418] != 0.0) && (s.v[857] != 0.0)) {
            s.store_ln_ad(294, A::div_from_scalar((0.5 * s.v[2]), A::offset(s.ad_value(300), 1e-21)));
        }

        if ((s.v[418] != 0.0) && (s.v[857] != 0.0)) {
            s.store_ln_ad(296, A::div_from_scalar((0.5 * s.v[2]), A::offset(s.ad_value(302), 1e-21)));
        }

        if ((s.v[418] != 0.0) && (s.v[857] != 0.0)) {
            s.store_ln_ad(298, A::div_from_scalar((0.5 * s.v[2]), A::offset(A::abs(s.ad_value(304)), 1e-21)));
        }

        if (s.v[418] != 0.0) {
            s.store_ad(294, &A::min_with_scalar(s.ad_value(294), 230.25850929940458));
        }

        if (s.v[418] != 0.0) {
            s.store_exp(295, 294);
        }

        if (s.v[418] != 0.0) {
            s.store_ad(296, &A::min_with_scalar(s.ad_value(296), 230.25850929940458));
        }

        if (s.v[418] != 0.0) {
            s.store_exp(297, 296);
        }

        if (s.v[418] != 0.0) {
            s.store_ad(298, &A::min_with_scalar(s.ad_value(298), 230.25850929940458));
        }

        if (s.v[418] != 0.0) {
            s.store_exp(299, 298);
        }

        s.store_ad(277, &A::voltage(ctx, &nodes, Some(0), Some(2)));

        s.v[858] = if (s.v[45] == 1.0) { 1.0 } else { 0.0 };

        if (s.v[858] != 0.0) {
            s.store_mul_ad_lhs(201, A::scale(s.ad_value(277), s.v[85]), 301);
        }

        if (s.v[858] != 0.0) {
            let assign28780_ad_e42653: A = {
                if (s.v[201] < (-230.25850929940458)) {
                    A::div_from_scalar(1e-100, A::offset(A::sub_from_scalar((-230.25850929940458), s.ad_value(201)), 1.0))
                } else {
                    {
                        if (s.v[201] > s.v[294]) {
                            A::mul(s.ad_value(295), A::offset(A::sub(s.ad_value(201), s.ad_value(294)), 1.0))
                        } else {
                            A::exp(s.ad_value(201))
                        }
                    }
                }
            };
            s.store_ad(202, &assign28780_ad_e42653);
        }

        if (s.v[858] != 0.0) {
            s.store_mul_ad_rhs(209, 300, A::offset(s.ad_value(202), (-1.0)));
        }

        if (s.v[858] != 0.0) {
            s.store_mul_ad_lhs(201, A::scale(s.ad_value(277), s.v[85]), 303);
        }

        if (s.v[858] != 0.0) {
            let assign28810_ad_e42698: A = {
                if (s.v[201] < (-230.25850929940458)) {
                    A::div_from_scalar(1e-100, A::offset(A::sub_from_scalar((-230.25850929940458), s.ad_value(201)), 1.0))
                } else {
                    {
                        if (s.v[201] > s.v[296]) {
                            A::mul(s.ad_value(297), A::offset(A::sub(s.ad_value(201), s.ad_value(296)), 1.0))
                        } else {
                            A::exp(s.ad_value(201))
                        }
                    }
                }
            };
            s.store_ad(202, &assign28810_ad_e42698);
        }

        if (s.v[858] != 0.0) {
            s.store_mul_ad_rhs(210, 302, A::offset(s.ad_value(202), (-1.0)));
        }

        if (s.v[858] != 0.0) {
            s.store_scalar(211, 0.0);
        }

        s.v[859] = if (s.v[306] > 0.0) { 1.0 } else { 0.0 };

        if ((s.v[858] != 0.0) && (s.v[859] != 0.0)) {
            s.store_mul_ad_rhs(211, 277, A::add(s.ad_value(304), A::mul(s.ad_value(277), s.ad_value(305))));
        }

        if ((s.v[858] != 0.0) && (!(s.v[859] != 0.0))) {
            s.store_mul_ad_lhs(201, A::scale(A::neg(s.ad_value(277)), s.v[85]), 305);
        }

        if ((s.v[858] != 0.0) && (!(s.v[859] != 0.0))) {
            let assign28870_ad_e42769: A = {
                if (s.v[201] < (-230.25850929940458)) {
                    A::div_from_scalar(1e-100, A::offset(A::sub_from_scalar((-230.25850929940458), s.ad_value(201)), 1.0))
                } else {
                    {
                        if (s.v[201] > s.v[298]) {
                            A::mul(s.ad_value(299), A::offset(A::sub(s.ad_value(201), s.ad_value(298)), 1.0))
                        } else {
                            A::exp(s.ad_value(201))
                        }
                    }
                }
            };
            s.store_ad(202, &assign28870_ad_e42769);
        }

        if ((s.v[858] != 0.0) && (!(s.v[859] != 0.0))) {
            s.store_mul_ad(211, A::neg(s.ad_value(304)), A::offset(s.ad_value(202), (-1.0)));
        }

        if (s.v[858] != 0.0) {
            s.store_add_ad_lhs(274, A::add(s.ad_value(209), s.ad_value(210)), 211);
        }

        if (s.v[858] != 0.0) {
            s.store_add(290, 210, 211);
        }

        if (s.v[858] != 0.0) {
            s.store_scalar(268, 0.0);
        }

        if (s.v[858] != 0.0) {
            s.store_scalar(270, 0.0);
        }

        if (s.v[858] != 0.0) {
            s.store_scalar(272, 0.0);
        }

        if (s.v[858] != 0.0) {
            s.store_scalar(291, 0.0);
        }

        if (s.v[858] != 0.0) {
            s.store_scalar(292, 0.0);
        }

        if (s.v[858] != 0.0) {
            s.store_scalar(293, 0.0);
        }

        if (s.v[858] != 0.0) {
            s.store_mul_ad_lhs(215, A::scale(s.ad_value(265), 4.0), 265);
        }

        if (s.v[858] != 0.0) {
            s.store_div(216, 265, 266);
        }

        if (s.v[858] != 0.0) {
            s.store_add_ad_rhs(217, 277, A::mul(s.ad_value(265), s.ad_value(216)));
        }

        if (s.v[858] != 0.0) {
            s.store_add(218, 266, 217);
        }

        if (s.v[858] != 0.0) {
            s.store_sub(219, 266, 217);
        }

        if (s.v[858] != 0.0) {
            s.store_sqrt_ad(220, A::add(A::square(s.ad_value(219)), s.ad_value(215)));
        }

        if (s.v[858] != 0.0) {
            s.store_scale_ad(204, A::div(A::mul(s.ad_value(277), s.ad_value(266)), A::add(s.ad_value(218), s.ad_value(220))), 2.0);
        }

        s.v[860] = if (s.v[259] > 0.5) { 1.0 } else { 0.0 };

        s.v[861] = if (s.v[122] == 0.5) { 1.0 } else { 0.0 };

        if (((s.v[858] != 0.0) && (s.v[860] != 0.0)) && (s.v[861] != 0.0)) {
            s.store_sqrt_ad(203, A::sub_from_scalar(1.0, A::mul(s.ad_value(204), s.ad_value(119))));
        }

        if (((s.v[858] != 0.0) && (s.v[860] != 0.0)) && (!(s.v[861] != 0.0))) {
            s.store_powf_ad(203, A::sub_from_scalar(1.0, A::mul(s.ad_value(204), s.ad_value(119))), s.v[122]);
        }

        if ((s.v[858] != 0.0) && (s.v[860] != 0.0)) {
            s.store_add_ad(269, A::mul(s.ad_value(131), A::sub_from_scalar(1.0, s.ad_value(203))), A::mul(s.ad_value(134), A::sub(s.ad_value(277), s.ad_value(204))));
        }

        if ((s.v[858] != 0.0) && (!(s.v[860] != 0.0))) {
            s.store_scalar(269, 0.0);
        }

        s.v[862] = if (s.v[260] > 0.5) { 1.0 } else { 0.0 };

        s.v[863] = if (s.v[123] == 0.5) { 1.0 } else { 0.0 };

        if (((s.v[858] != 0.0) && (s.v[862] != 0.0)) && (s.v[863] != 0.0)) {
            s.store_sqrt_ad(203, A::sub_from_scalar(1.0, A::mul(s.ad_value(204), s.ad_value(120))));
        }

        if (((s.v[858] != 0.0) && (s.v[862] != 0.0)) && (!(s.v[863] != 0.0))) {
            s.store_powf_ad(203, A::sub_from_scalar(1.0, A::mul(s.ad_value(204), s.ad_value(120))), s.v[123]);
        }

        if ((s.v[858] != 0.0) && (s.v[862] != 0.0)) {
            s.store_add_ad(271, A::mul(s.ad_value(132), A::sub_from_scalar(1.0, s.ad_value(203))), A::mul(s.ad_value(135), A::sub(s.ad_value(277), s.ad_value(204))));
        }

        if ((s.v[858] != 0.0) && (!(s.v[862] != 0.0))) {
            s.store_scalar(271, 0.0);
        }

        s.v[864] = if (s.v[261] > 0.5) { 1.0 } else { 0.0 };

        s.v[865] = if (s.v[124] == 0.5) { 1.0 } else { 0.0 };

        if (((s.v[858] != 0.0) && (s.v[864] != 0.0)) && (s.v[865] != 0.0)) {
            s.store_sqrt_ad(203, A::sub_from_scalar(1.0, A::mul(s.ad_value(204), s.ad_value(121))));
        }

        if (((s.v[858] != 0.0) && (s.v[864] != 0.0)) && (!(s.v[865] != 0.0))) {
            s.store_powf_ad(203, A::sub_from_scalar(1.0, A::mul(s.ad_value(204), s.ad_value(121))), s.v[124]);
        }

        if ((s.v[858] != 0.0) && (s.v[864] != 0.0)) {
            s.store_add_ad(273, A::mul(s.ad_value(133), A::sub_from_scalar(1.0, s.ad_value(203))), A::mul(s.ad_value(136), A::sub(s.ad_value(277), s.ad_value(204))));
        }

        if ((s.v[858] != 0.0) && (!(s.v[864] != 0.0))) {
            s.store_scalar(273, 0.0);
        }

        s.v[866] = if !(((s.v[256] == 0.0) && (s.v[257] == 0.0)) && (s.v[258] == 0.0)) { 1.0 } else { 0.0 };

        if ((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) {
            s.store_mul_ad_lhs(215, A::scale(s.ad_value(265), 4.0), 265);
        }

        if ((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) {
            s.store_div(216, 265, 266);
        }

        if ((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) {
            s.store_add_ad_rhs(217, 277, A::mul(s.ad_value(265), s.ad_value(216)));
        }

        if ((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) {
            s.store_add(218, 266, 217);
        }

        if ((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) {
            s.store_sub(219, 266, 217);
        }

        if ((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) {
            s.store_sqrt_ad(220, A::add(A::square(s.ad_value(219)), s.ad_value(215)));
        }

        if ((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) {
            s.store_scale_ad(221, A::div(A::mul(s.ad_value(277), s.ad_value(266)), A::add(s.ad_value(218), s.ad_value(220))), 2.0);
        }

        s.v[867] = if (s.v[277] < s.v[262]) { 1.0 } else { 0.0 };

        s.v[868] = if ((((0.5 * (s.v[277] * s.v[85]))) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (s.v[867] != 0.0)) && (s.v[868] != 0.0)) {
            s.store_exp_ad(223, A::scale(s.ad_value(277), (s.v[85] * 0.5)));
        }

        s.v[869] = if ((0.5 * (s.v[277] * s.v[85])) < (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (s.v[867] != 0.0)) && (!(s.v[868] != 0.0))) && (s.v[869] != 0.0)) {
            let assign29340_ad_e43219: A = A::div_from_scalar(1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(s.ad_value(277), (s.v[85] * 0.5))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(s.ad_value(277), (s.v[85] * 0.5))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::scale(s.ad_value(277), (s.v[85] * 0.5))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
            s.store_ad(223, &assign29340_ad_e43219);
        }

        if (((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (s.v[867] != 0.0)) && (!(s.v[868] != 0.0))) && (!(s.v[869] != 0.0))) {
            s.store_scale_ad(223, A::offset(A::mul(A::offset(A::scale(s.ad_value(277), (s.v[85] * 0.5)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::scale(s.ad_value(277), (s.v[85] * 0.5)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::scale(s.ad_value(277), (s.v[85] * 0.5)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
        }

        if (((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (s.v[867] != 0.0)) {
            s.store_scale_ad(363, A::square(s.ad_value(318)), 1.0 / (s.v[308]));
        }

        if (((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (s.v[867] != 0.0)) {
            s.store_scale_ad(362, A::ln(A::div_from_scalar(s.v[308], s.ad_value(363))), (s.v[62] / s.v[85]));
        }

        s.v[870] = if (s.v[62] < p.p85) { 1.0 } else { 0.0 };

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (s.v[867] != 0.0)) && (s.v[870] != 0.0)) {
            s.store_offset_ad(360, A::scale(A::sub(s.ad_value(277), s.ad_value(362)), p.p86), s.v[62]);
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (s.v[867] != 0.0)) && (s.v[870] != 0.0)) {
            s.store_sub_from_scalar_ad(350, s.v[62], A::scale(s.ad_value(362), p.p86));
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (s.v[867] != 0.0)) && (s.v[870] != 0.0)) {
            s.store_offset_ad(314, A::sub_from_scalar(p.p85, s.ad_value(360)), (-0.01));
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (s.v[867] != 0.0)) && (s.v[870] != 0.0)) {
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (s.v[867] != 0.0)) && (s.v[870] != 0.0)) {
            s.store_ad(315, &{
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (s.v[867] != 0.0)) && (s.v[870] != 0.0)) {
            s.store_sqrt_ad(315, A::add(A::square(s.ad_value(314)), s.ad_value(315)));
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (s.v[867] != 0.0)) && (s.v[870] != 0.0)) {
            s.store_sub_from_scalar_ad(361, p.p85, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5));
        }

    }

    pub(super) fn stamp_transient_block_22(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (s.v[867] != 0.0)) && (s.v[870] != 0.0)) {
            s.store_offset(314, 361, (((-s.v[62])) + ((-0.01))));
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (s.v[867] != 0.0)) && (s.v[870] != 0.0)) {
            s.store_scalar(315, ((4.0 * s.v[62]) * 0.01));
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (s.v[867] != 0.0)) && (s.v[870] != 0.0)) {
            s.store_ad(315, &{
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (s.v[867] != 0.0)) && (s.v[870] != 0.0)) {
            s.store_sqrt_ad(315, A::add(A::square(s.ad_value(314)), s.ad_value(315)));
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (s.v[867] != 0.0)) && (s.v[870] != 0.0)) {
            s.store_offset_ad(359, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5), s.v[62]);
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (s.v[867] != 0.0)) && (s.v[870] != 0.0)) {
            s.store_offset_ad(314, A::sub_from_scalar(p.p85, s.ad_value(350)), (-0.01));
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (s.v[867] != 0.0)) && (s.v[870] != 0.0)) {
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (s.v[867] != 0.0)) && (s.v[870] != 0.0)) {
            s.store_ad(315, &{
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (s.v[867] != 0.0)) && (s.v[870] != 0.0)) {
            s.store_sqrt_ad(315, A::add(A::square(s.ad_value(314)), s.ad_value(315)));
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (s.v[867] != 0.0)) && (s.v[870] != 0.0)) {
            s.store_sub_from_scalar_ad(350, p.p85, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5));
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (s.v[867] != 0.0)) && (s.v[870] != 0.0)) {
            s.store_offset(314, 350, (((-s.v[62])) + ((-0.01))));
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (s.v[867] != 0.0)) && (s.v[870] != 0.0)) {
            s.store_scalar(315, ((4.0 * s.v[62]) * 0.01));
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (s.v[867] != 0.0)) && (s.v[870] != 0.0)) {
            s.store_ad(315, &{
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (s.v[867] != 0.0)) && (s.v[870] != 0.0)) {
            s.store_sqrt_ad(315, A::add(A::square(s.ad_value(314)), s.ad_value(315)));
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (s.v[867] != 0.0)) && (s.v[870] != 0.0)) {
            s.store_offset_ad(350, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5), s.v[62]);
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (s.v[867] != 0.0)) && (!(s.v[870] != 0.0))) {
            s.store_scalar(350, s.v[62]);
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (s.v[867] != 0.0)) && (!(s.v[870] != 0.0))) {
            s.store_scalar(359, s.v[62]);
        }

        s.v[871] = if ((((s.v[85] * ((s.v[277] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85))))) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (s.v[867] != 0.0)) && (s.v[871] != 0.0)) {
            s.store_exp_ad(370, A::scale(A::add(A::div(s.ad_value(277), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]));
        }

        s.v[872] = if ((s.v[85] * ((s.v[277] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85)))) < (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (s.v[867] != 0.0)) && (!(s.v[871] != 0.0))) && (s.v[872] != 0.0)) {
            let assign29660_ad_e43814: A = A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(A::add(A::div(s.ad_value(277), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85])), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::scale(A::add(A::div(s.ad_value(277), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85])), 0.3333333333333333), 1.0));
            s.store_div_from_scalar_ad(370, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(A::add(A::div(s.ad_value(277), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85])), A::offset(A::scale(assign29660_ad_e43814, 0.5), 1.0)), 1.0));
        }

        if (((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (s.v[867] != 0.0)) && (!(s.v[871] != 0.0))) && (!(s.v[872] != 0.0))) {
            let assign29670_ad_e43893: A = A::mul(A::offset(A::scale(A::add(A::div(s.ad_value(277), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]), (-230.25850929940458)), A::offset(A::scale(A::offset(A::scale(A::add(A::div(s.ad_value(277), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]), (-230.25850929940458)), 0.3333333333333333), 1.0));
            s.store_scale_ad(370, A::offset(A::mul(A::offset(A::scale(A::add(A::div(s.ad_value(277), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]), (-230.25850929940458)), A::offset(A::scale(assign29670_ad_e43893, 0.5), 1.0)), 1.0), 1e100);
        }

        if (((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (s.v[867] != 0.0)) {
            s.store_scale_ad(363, A::square(s.ad_value(318)), 1.0 / (s.v[310]));
        }

        if (((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (s.v[867] != 0.0)) {
            s.store_scale_ad(362, A::ln(A::div_from_scalar(s.v[310], s.ad_value(363))), (s.v[64] / s.v[85]));
        }

        s.v[873] = if (s.v[64] < p.p85) { 1.0 } else { 0.0 };

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (s.v[867] != 0.0)) && (s.v[873] != 0.0)) {
            s.store_offset_ad(360, A::scale(A::sub(s.ad_value(277), s.ad_value(362)), p.p86), s.v[64]);
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (s.v[867] != 0.0)) && (s.v[873] != 0.0)) {
            s.store_sub_from_scalar_ad(350, s.v[64], A::scale(s.ad_value(362), p.p86));
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (s.v[867] != 0.0)) && (s.v[873] != 0.0)) {
            s.store_offset_ad(314, A::sub_from_scalar(p.p85, s.ad_value(360)), (-0.01));
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (s.v[867] != 0.0)) && (s.v[873] != 0.0)) {
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (s.v[867] != 0.0)) && (s.v[873] != 0.0)) {
            s.store_ad(315, &{
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (s.v[867] != 0.0)) && (s.v[873] != 0.0)) {
            s.store_sqrt_ad(315, A::add(A::square(s.ad_value(314)), s.ad_value(315)));
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (s.v[867] != 0.0)) && (s.v[873] != 0.0)) {
            s.store_sub_from_scalar_ad(361, p.p85, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5));
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (s.v[867] != 0.0)) && (s.v[873] != 0.0)) {
            s.store_offset(314, 361, (((-s.v[64])) + ((-0.01))));
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (s.v[867] != 0.0)) && (s.v[873] != 0.0)) {
            s.store_scalar(315, ((4.0 * s.v[64]) * 0.01));
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (s.v[867] != 0.0)) && (s.v[873] != 0.0)) {
            s.store_ad(315, &{
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (s.v[867] != 0.0)) && (s.v[873] != 0.0)) {
            s.store_sqrt_ad(315, A::add(A::square(s.ad_value(314)), s.ad_value(315)));
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (s.v[867] != 0.0)) && (s.v[873] != 0.0)) {
            s.store_offset_ad(359, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5), s.v[64]);
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (s.v[867] != 0.0)) && (s.v[873] != 0.0)) {
            s.store_offset_ad(314, A::sub_from_scalar(p.p85, s.ad_value(350)), (-0.01));
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (s.v[867] != 0.0)) && (s.v[873] != 0.0)) {
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (s.v[867] != 0.0)) && (s.v[873] != 0.0)) {
            s.store_ad(315, &{
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (s.v[867] != 0.0)) && (s.v[873] != 0.0)) {
            s.store_sqrt_ad(315, A::add(A::square(s.ad_value(314)), s.ad_value(315)));
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (s.v[867] != 0.0)) && (s.v[873] != 0.0)) {
            s.store_sub_from_scalar_ad(350, p.p85, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5));
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (s.v[867] != 0.0)) && (s.v[873] != 0.0)) {
            s.store_offset(314, 350, (((-s.v[64])) + ((-0.01))));
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (s.v[867] != 0.0)) && (s.v[873] != 0.0)) {
            s.store_scalar(315, ((4.0 * s.v[64]) * 0.01));
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (s.v[867] != 0.0)) && (s.v[873] != 0.0)) {
            s.store_ad(315, &{
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (s.v[867] != 0.0)) && (s.v[873] != 0.0)) {
            s.store_sqrt_ad(315, A::add(A::square(s.ad_value(314)), s.ad_value(315)));
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (s.v[867] != 0.0)) && (s.v[873] != 0.0)) {
            s.store_offset_ad(350, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5), s.v[64]);
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (s.v[867] != 0.0)) && (!(s.v[873] != 0.0))) {
            s.store_scalar(350, s.v[64]);
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (s.v[867] != 0.0)) && (!(s.v[873] != 0.0))) {
            s.store_scalar(359, s.v[64]);
        }

        s.v[874] = if ((((s.v[85] * ((s.v[277] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85))))) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (s.v[867] != 0.0)) && (s.v[874] != 0.0)) {
            s.store_exp_ad(371, A::scale(A::add(A::div(s.ad_value(277), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]));
        }

        s.v[875] = if ((s.v[85] * ((s.v[277] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85)))) < (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (s.v[867] != 0.0)) && (!(s.v[874] != 0.0))) && (s.v[875] != 0.0)) {
            let assign29980_ad_e44444: A = A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(A::add(A::div(s.ad_value(277), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85])), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::scale(A::add(A::div(s.ad_value(277), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85])), 0.3333333333333333), 1.0));
            s.store_div_from_scalar_ad(371, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(A::add(A::div(s.ad_value(277), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85])), A::offset(A::scale(assign29980_ad_e44444, 0.5), 1.0)), 1.0));
        }

        if (((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (s.v[867] != 0.0)) && (!(s.v[874] != 0.0))) && (!(s.v[875] != 0.0))) {
            let assign29990_ad_e44523: A = A::mul(A::offset(A::scale(A::add(A::div(s.ad_value(277), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]), (-230.25850929940458)), A::offset(A::scale(A::offset(A::scale(A::add(A::div(s.ad_value(277), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]), (-230.25850929940458)), 0.3333333333333333), 1.0));
            s.store_scale_ad(371, A::offset(A::mul(A::offset(A::scale(A::add(A::div(s.ad_value(277), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]), (-230.25850929940458)), A::offset(A::scale(assign29990_ad_e44523, 0.5), 1.0)), 1.0), 1e100);
        }

        if (((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (s.v[867] != 0.0)) {
            s.store_scale_ad(363, A::square(s.ad_value(318)), 1.0 / (s.v[309]));
        }

        if (((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (s.v[867] != 0.0)) {
            s.store_scale_ad(362, A::ln(A::div_from_scalar(s.v[309], s.ad_value(363))), (s.v[63] / s.v[85]));
        }

        s.v[876] = if (s.v[63] < p.p85) { 1.0 } else { 0.0 };

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (s.v[867] != 0.0)) && (s.v[876] != 0.0)) {
            s.store_offset_ad(360, A::scale(A::sub(s.ad_value(277), s.ad_value(362)), p.p86), s.v[63]);
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (s.v[867] != 0.0)) && (s.v[876] != 0.0)) {
            s.store_sub_from_scalar_ad(350, s.v[63], A::scale(s.ad_value(362), p.p86));
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (s.v[867] != 0.0)) && (s.v[876] != 0.0)) {
            s.store_offset_ad(314, A::sub_from_scalar(p.p85, s.ad_value(360)), (-0.01));
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (s.v[867] != 0.0)) && (s.v[876] != 0.0)) {
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (s.v[867] != 0.0)) && (s.v[876] != 0.0)) {
            s.store_ad(315, &{
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (s.v[867] != 0.0)) && (s.v[876] != 0.0)) {
            s.store_sqrt_ad(315, A::add(A::square(s.ad_value(314)), s.ad_value(315)));
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (s.v[867] != 0.0)) && (s.v[876] != 0.0)) {
            s.store_sub_from_scalar_ad(361, p.p85, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5));
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (s.v[867] != 0.0)) && (s.v[876] != 0.0)) {
            s.store_offset(314, 361, (((-s.v[63])) + ((-0.01))));
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (s.v[867] != 0.0)) && (s.v[876] != 0.0)) {
            s.store_scalar(315, ((4.0 * s.v[63]) * 0.01));
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (s.v[867] != 0.0)) && (s.v[876] != 0.0)) {
            s.store_ad(315, &{
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (s.v[867] != 0.0)) && (s.v[876] != 0.0)) {
            s.store_sqrt_ad(315, A::add(A::square(s.ad_value(314)), s.ad_value(315)));
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (s.v[867] != 0.0)) && (s.v[876] != 0.0)) {
            s.store_offset_ad(359, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5), s.v[63]);
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (s.v[867] != 0.0)) && (s.v[876] != 0.0)) {
            s.store_offset_ad(314, A::sub_from_scalar(p.p85, s.ad_value(350)), (-0.01));
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (s.v[867] != 0.0)) && (s.v[876] != 0.0)) {
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (s.v[867] != 0.0)) && (s.v[876] != 0.0)) {
            s.store_ad(315, &{
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (s.v[867] != 0.0)) && (s.v[876] != 0.0)) {
            s.store_sqrt_ad(315, A::add(A::square(s.ad_value(314)), s.ad_value(315)));
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (s.v[867] != 0.0)) && (s.v[876] != 0.0)) {
            s.store_sub_from_scalar_ad(350, p.p85, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5));
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (s.v[867] != 0.0)) && (s.v[876] != 0.0)) {
            s.store_offset(314, 350, (((-s.v[63])) + ((-0.01))));
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (s.v[867] != 0.0)) && (s.v[876] != 0.0)) {
            s.store_scalar(315, ((4.0 * s.v[63]) * 0.01));
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (s.v[867] != 0.0)) && (s.v[876] != 0.0)) {
            s.store_ad(315, &{
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (s.v[867] != 0.0)) && (s.v[876] != 0.0)) {
            s.store_sqrt_ad(315, A::add(A::square(s.ad_value(314)), s.ad_value(315)));
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (s.v[867] != 0.0)) && (s.v[876] != 0.0)) {
            s.store_offset_ad(350, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5), s.v[63]);
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (s.v[867] != 0.0)) && (!(s.v[876] != 0.0))) {
            s.store_scalar(350, s.v[63]);
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (s.v[867] != 0.0)) && (!(s.v[876] != 0.0))) {
            s.store_scalar(359, s.v[63]);
        }

        s.v[877] = if ((((s.v[85] * ((s.v[277] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85))))) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (s.v[867] != 0.0)) && (s.v[877] != 0.0)) {
            s.store_exp_ad(372, A::scale(A::add(A::div(s.ad_value(277), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]));
        }

        s.v[878] = if ((s.v[85] * ((s.v[277] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85)))) < (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (s.v[867] != 0.0)) && (!(s.v[877] != 0.0))) && (s.v[878] != 0.0)) {
            let assign30300_ad_e45074: A = A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(A::add(A::div(s.ad_value(277), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85])), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::scale(A::add(A::div(s.ad_value(277), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85])), 0.3333333333333333), 1.0));
            s.store_div_from_scalar_ad(372, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(A::add(A::div(s.ad_value(277), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85])), A::offset(A::scale(assign30300_ad_e45074, 0.5), 1.0)), 1.0));
        }

        if (((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (s.v[867] != 0.0)) && (!(s.v[877] != 0.0))) && (!(s.v[878] != 0.0))) {
            let assign30310_ad_e45153: A = A::mul(A::offset(A::scale(A::add(A::div(s.ad_value(277), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]), (-230.25850929940458)), A::offset(A::scale(A::offset(A::scale(A::add(A::div(s.ad_value(277), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]), (-230.25850929940458)), 0.3333333333333333), 1.0));
            s.store_scale_ad(372, A::offset(A::mul(A::offset(A::scale(A::add(A::div(s.ad_value(277), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]), (-230.25850929940458)), A::offset(A::scale(assign30310_ad_e45153, 0.5), 1.0)), 1.0), 1e100);
        }

        if (((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (!(s.v[867] != 0.0))) {
            s.store_sqrt_ad(223, A::mul(A::offset(A::scale(A::sub(s.ad_value(277), s.ad_value(262)), s.v[85]), 1.0), s.ad_value(263)));
        }

        if (((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (!(s.v[867] != 0.0))) {
            s.store_scale_ad(363, A::square(s.ad_value(318)), 1.0 / (s.v[308]));
        }

        if (((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (!(s.v[867] != 0.0))) {
            s.store_scale_ad(362, A::ln(A::div_from_scalar(s.v[308], s.ad_value(363))), (s.v[62] / s.v[85]));
        }

        s.v[879] = if (s.v[62] < p.p85) { 1.0 } else { 0.0 };

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (!(s.v[867] != 0.0))) && (s.v[879] != 0.0)) {
            s.store_offset_ad(360, A::scale(A::sub(s.ad_value(262), s.ad_value(362)), p.p86), s.v[62]);
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (!(s.v[867] != 0.0))) && (s.v[879] != 0.0)) {
            s.store_sub_from_scalar_ad(350, s.v[62], A::scale(s.ad_value(362), p.p86));
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (!(s.v[867] != 0.0))) && (s.v[879] != 0.0)) {
            s.store_offset_ad(314, A::sub_from_scalar(p.p85, s.ad_value(360)), (-0.01));
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (!(s.v[867] != 0.0))) && (s.v[879] != 0.0)) {
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (!(s.v[867] != 0.0))) && (s.v[879] != 0.0)) {
            s.store_ad(315, &{
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (!(s.v[867] != 0.0))) && (s.v[879] != 0.0)) {
            s.store_sqrt_ad(315, A::add(A::square(s.ad_value(314)), s.ad_value(315)));
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (!(s.v[867] != 0.0))) && (s.v[879] != 0.0)) {
            s.store_scale_ad(364, A::offset(A::div(s.ad_value(314), s.ad_value(315)), 1.0), 0.5);
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (!(s.v[867] != 0.0))) && (s.v[879] != 0.0)) {
            s.store_sub_from_scalar_ad(361, p.p85, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5));
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (!(s.v[867] != 0.0))) && (s.v[879] != 0.0)) {
            s.store_offset(314, 361, (((-s.v[62])) + ((-0.01))));
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (!(s.v[867] != 0.0))) && (s.v[879] != 0.0)) {
            s.store_scalar(315, ((4.0 * s.v[62]) * 0.01));
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (!(s.v[867] != 0.0))) && (s.v[879] != 0.0)) {
            s.store_ad(315, &{
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (!(s.v[867] != 0.0))) && (s.v[879] != 0.0)) {
            s.store_sqrt_ad(315, A::add(A::square(s.ad_value(314)), s.ad_value(315)));
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (!(s.v[867] != 0.0))) && (s.v[879] != 0.0)) {
            s.store_scale_ad(365, A::offset(A::div(s.ad_value(314), s.ad_value(315)), 1.0), 0.5);
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (!(s.v[867] != 0.0))) && (s.v[879] != 0.0)) {
            s.store_offset_ad(359, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5), s.v[62]);
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (!(s.v[867] != 0.0))) && (s.v[879] != 0.0)) {
            s.store_offset_ad(314, A::sub_from_scalar(p.p85, s.ad_value(350)), (-0.01));
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (!(s.v[867] != 0.0))) && (s.v[879] != 0.0)) {
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (!(s.v[867] != 0.0))) && (s.v[879] != 0.0)) {
            s.store_ad(315, &{
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (!(s.v[867] != 0.0))) && (s.v[879] != 0.0)) {
            s.store_sqrt_ad(315, A::add(A::square(s.ad_value(314)), s.ad_value(315)));
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (!(s.v[867] != 0.0))) && (s.v[879] != 0.0)) {
            s.store_sub_from_scalar_ad(350, p.p85, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5));
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (!(s.v[867] != 0.0))) && (s.v[879] != 0.0)) {
            s.store_offset(314, 350, (((-s.v[62])) + ((-0.01))));
        }

    }

    pub(super) fn stamp_transient_block_23(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (!(s.v[867] != 0.0))) && (s.v[879] != 0.0)) {
            s.store_scalar(315, ((4.0 * s.v[62]) * 0.01));
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (!(s.v[867] != 0.0))) && (s.v[879] != 0.0)) {
            s.store_ad(315, &{
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (!(s.v[867] != 0.0))) && (s.v[879] != 0.0)) {
            s.store_sqrt_ad(315, A::add(A::square(s.ad_value(314)), s.ad_value(315)));
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (!(s.v[867] != 0.0))) && (s.v[879] != 0.0)) {
            s.store_offset_ad(350, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5), s.v[62]);
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (!(s.v[867] != 0.0))) && (s.v[879] != 0.0)) {
            s.store_mul_ad_lhs(366, A::scale(s.ad_value(364), p.p86), 365);
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (!(s.v[867] != 0.0))) && (!(s.v[879] != 0.0))) {
            s.store_scalar(350, s.v[62]);
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (!(s.v[867] != 0.0))) && (!(s.v[879] != 0.0))) {
            s.store_scalar(359, s.v[62]);
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (!(s.v[867] != 0.0))) && (!(s.v[879] != 0.0))) {
            s.store_scalar(366, 0.0);
        }

        s.v[880] = if ((((s.v[85] * ((s.v[262] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85))))) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (!(s.v[867] != 0.0))) && (s.v[880] != 0.0)) {
            s.store_exp_ad(281, A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]));
        }

        s.v[881] = if ((s.v[85] * ((s.v[262] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85)))) < (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (!(s.v[867] != 0.0))) && (!(s.v[880] != 0.0))) && (s.v[881] != 0.0)) {
            let assign30670_ad_e45816: A = A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85])), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85])), 0.3333333333333333), 1.0));
            s.store_div_from_scalar_ad(281, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85])), A::offset(A::scale(assign30670_ad_e45816, 0.5), 1.0)), 1.0));
        }

        if (((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (!(s.v[867] != 0.0))) && (!(s.v[880] != 0.0))) && (!(s.v[881] != 0.0))) {
            let assign30680_ad_e45896: A = A::mul(A::offset(A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]), (-230.25850929940458)), A::offset(A::scale(A::offset(A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]), (-230.25850929940458)), 0.3333333333333333), 1.0));
            s.store_scale_ad(281, A::offset(A::mul(A::offset(A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]), (-230.25850929940458)), A::offset(A::scale(assign30680_ad_e45896, 0.5), 1.0)), 1.0), 1e100);
        }

        if (((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (!(s.v[867] != 0.0))) {
            s.store_scale_ad(367, A::add(A::div(A::sub(s.ad_value(359), A::mul(s.ad_value(262), s.ad_value(366))), A::square(s.ad_value(359))), A::div(A::mul(s.ad_value(362), s.ad_value(366)), A::scale(s.ad_value(350), p.p85))), s.v[85]);
        }

        if (((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (!(s.v[867] != 0.0))) {
            s.store_mul_ad_lhs(370, A::offset(A::mul(A::sub(s.ad_value(277), s.ad_value(262)), s.ad_value(367)), 1.0), 281);
        }

        if (((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (!(s.v[867] != 0.0))) {
            s.store_scale_ad(363, A::square(s.ad_value(318)), 1.0 / (s.v[310]));
        }

        if (((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (!(s.v[867] != 0.0))) {
            s.store_scale_ad(362, A::ln(A::div_from_scalar(s.v[310], s.ad_value(363))), (s.v[64] / s.v[85]));
        }

        s.v[882] = if (s.v[64] < p.p85) { 1.0 } else { 0.0 };

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (!(s.v[867] != 0.0))) && (s.v[882] != 0.0)) {
            s.store_offset_ad(360, A::scale(A::sub(s.ad_value(262), s.ad_value(362)), p.p86), s.v[64]);
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (!(s.v[867] != 0.0))) && (s.v[882] != 0.0)) {
            s.store_sub_from_scalar_ad(350, s.v[64], A::scale(s.ad_value(362), p.p86));
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (!(s.v[867] != 0.0))) && (s.v[882] != 0.0)) {
            s.store_offset_ad(314, A::sub_from_scalar(p.p85, s.ad_value(360)), (-0.01));
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (!(s.v[867] != 0.0))) && (s.v[882] != 0.0)) {
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (!(s.v[867] != 0.0))) && (s.v[882] != 0.0)) {
            s.store_ad(315, &{
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (!(s.v[867] != 0.0))) && (s.v[882] != 0.0)) {
            s.store_sqrt_ad(315, A::add(A::square(s.ad_value(314)), s.ad_value(315)));
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (!(s.v[867] != 0.0))) && (s.v[882] != 0.0)) {
            s.store_scale_ad(364, A::offset(A::div(s.ad_value(314), s.ad_value(315)), 1.0), 0.5);
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (!(s.v[867] != 0.0))) && (s.v[882] != 0.0)) {
            s.store_sub_from_scalar_ad(361, p.p85, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5));
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (!(s.v[867] != 0.0))) && (s.v[882] != 0.0)) {
            s.store_offset(314, 361, (((-s.v[64])) + ((-0.01))));
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (!(s.v[867] != 0.0))) && (s.v[882] != 0.0)) {
            s.store_scalar(315, ((4.0 * s.v[64]) * 0.01));
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (!(s.v[867] != 0.0))) && (s.v[882] != 0.0)) {
            s.store_ad(315, &{
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (!(s.v[867] != 0.0))) && (s.v[882] != 0.0)) {
            s.store_sqrt_ad(315, A::add(A::square(s.ad_value(314)), s.ad_value(315)));
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (!(s.v[867] != 0.0))) && (s.v[882] != 0.0)) {
            s.store_scale_ad(365, A::offset(A::div(s.ad_value(314), s.ad_value(315)), 1.0), 0.5);
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (!(s.v[867] != 0.0))) && (s.v[882] != 0.0)) {
            s.store_offset_ad(359, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5), s.v[64]);
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (!(s.v[867] != 0.0))) && (s.v[882] != 0.0)) {
            s.store_offset_ad(314, A::sub_from_scalar(p.p85, s.ad_value(350)), (-0.01));
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (!(s.v[867] != 0.0))) && (s.v[882] != 0.0)) {
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (!(s.v[867] != 0.0))) && (s.v[882] != 0.0)) {
            s.store_ad(315, &{
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (!(s.v[867] != 0.0))) && (s.v[882] != 0.0)) {
            s.store_sqrt_ad(315, A::add(A::square(s.ad_value(314)), s.ad_value(315)));
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (!(s.v[867] != 0.0))) && (s.v[882] != 0.0)) {
            s.store_sub_from_scalar_ad(350, p.p85, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5));
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (!(s.v[867] != 0.0))) && (s.v[882] != 0.0)) {
            s.store_offset(314, 350, (((-s.v[64])) + ((-0.01))));
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (!(s.v[867] != 0.0))) && (s.v[882] != 0.0)) {
            s.store_scalar(315, ((4.0 * s.v[64]) * 0.01));
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (!(s.v[867] != 0.0))) && (s.v[882] != 0.0)) {
            s.store_ad(315, &{
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (!(s.v[867] != 0.0))) && (s.v[882] != 0.0)) {
            s.store_sqrt_ad(315, A::add(A::square(s.ad_value(314)), s.ad_value(315)));
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (!(s.v[867] != 0.0))) && (s.v[882] != 0.0)) {
            s.store_offset_ad(350, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5), s.v[64]);
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (!(s.v[867] != 0.0))) && (s.v[882] != 0.0)) {
            s.store_mul_ad_lhs(366, A::scale(s.ad_value(364), p.p86), 365);
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (!(s.v[867] != 0.0))) && (!(s.v[882] != 0.0))) {
            s.store_scalar(350, s.v[64]);
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (!(s.v[867] != 0.0))) && (!(s.v[882] != 0.0))) {
            s.store_scalar(359, s.v[64]);
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (!(s.v[867] != 0.0))) && (!(s.v[882] != 0.0))) {
            s.store_scalar(366, 0.0);
        }

        s.v[883] = if ((((s.v[85] * ((s.v[262] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85))))) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (!(s.v[867] != 0.0))) && (s.v[883] != 0.0)) {
            s.store_exp_ad(282, A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]));
        }

        s.v[884] = if ((s.v[85] * ((s.v[262] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85)))) < (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (!(s.v[867] != 0.0))) && (!(s.v[883] != 0.0))) && (s.v[884] != 0.0)) {
            let assign31050_ad_e46586: A = A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85])), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85])), 0.3333333333333333), 1.0));
            s.store_div_from_scalar_ad(282, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85])), A::offset(A::scale(assign31050_ad_e46586, 0.5), 1.0)), 1.0));
        }

        if (((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (!(s.v[867] != 0.0))) && (!(s.v[883] != 0.0))) && (!(s.v[884] != 0.0))) {
            let assign31060_ad_e46666: A = A::mul(A::offset(A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]), (-230.25850929940458)), A::offset(A::scale(A::offset(A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]), (-230.25850929940458)), 0.3333333333333333), 1.0));
            s.store_scale_ad(282, A::offset(A::mul(A::offset(A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]), (-230.25850929940458)), A::offset(A::scale(assign31060_ad_e46666, 0.5), 1.0)), 1.0), 1e100);
        }

        if (((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (!(s.v[867] != 0.0))) {
            s.store_scale_ad(367, A::add(A::div(A::sub(s.ad_value(359), A::mul(s.ad_value(262), s.ad_value(366))), A::square(s.ad_value(359))), A::div(A::mul(s.ad_value(362), s.ad_value(366)), A::scale(s.ad_value(350), p.p85))), s.v[85]);
        }

        if (((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (!(s.v[867] != 0.0))) {
            s.store_mul_ad_lhs(371, A::offset(A::mul(A::sub(s.ad_value(277), s.ad_value(262)), s.ad_value(367)), 1.0), 282);
        }

        if (((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (!(s.v[867] != 0.0))) {
            s.store_scale_ad(363, A::square(s.ad_value(318)), 1.0 / (s.v[309]));
        }

        if (((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (!(s.v[867] != 0.0))) {
            s.store_scale_ad(362, A::ln(A::div_from_scalar(s.v[309], s.ad_value(363))), (s.v[63] / s.v[85]));
        }

        s.v[885] = if (s.v[63] < p.p85) { 1.0 } else { 0.0 };

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (!(s.v[867] != 0.0))) && (s.v[885] != 0.0)) {
            s.store_offset_ad(360, A::scale(A::sub(s.ad_value(262), s.ad_value(362)), p.p86), s.v[63]);
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (!(s.v[867] != 0.0))) && (s.v[885] != 0.0)) {
            s.store_sub_from_scalar_ad(350, s.v[63], A::scale(s.ad_value(362), p.p86));
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (!(s.v[867] != 0.0))) && (s.v[885] != 0.0)) {
            s.store_offset_ad(314, A::sub_from_scalar(p.p85, s.ad_value(360)), (-0.01));
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (!(s.v[867] != 0.0))) && (s.v[885] != 0.0)) {
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (!(s.v[867] != 0.0))) && (s.v[885] != 0.0)) {
            s.store_ad(315, &{
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (!(s.v[867] != 0.0))) && (s.v[885] != 0.0)) {
            s.store_sqrt_ad(315, A::add(A::square(s.ad_value(314)), s.ad_value(315)));
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (!(s.v[867] != 0.0))) && (s.v[885] != 0.0)) {
            s.store_scale_ad(364, A::offset(A::div(s.ad_value(314), s.ad_value(315)), 1.0), 0.5);
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (!(s.v[867] != 0.0))) && (s.v[885] != 0.0)) {
            s.store_sub_from_scalar_ad(361, p.p85, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5));
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (!(s.v[867] != 0.0))) && (s.v[885] != 0.0)) {
            s.store_offset(314, 361, (((-s.v[63])) + ((-0.01))));
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (!(s.v[867] != 0.0))) && (s.v[885] != 0.0)) {
            s.store_scalar(315, ((4.0 * s.v[63]) * 0.01));
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (!(s.v[867] != 0.0))) && (s.v[885] != 0.0)) {
            s.store_ad(315, &{
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (!(s.v[867] != 0.0))) && (s.v[885] != 0.0)) {
            s.store_sqrt_ad(315, A::add(A::square(s.ad_value(314)), s.ad_value(315)));
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (!(s.v[867] != 0.0))) && (s.v[885] != 0.0)) {
            s.store_scale_ad(365, A::offset(A::div(s.ad_value(314), s.ad_value(315)), 1.0), 0.5);
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (!(s.v[867] != 0.0))) && (s.v[885] != 0.0)) {
            s.store_offset_ad(359, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5), s.v[63]);
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (!(s.v[867] != 0.0))) && (s.v[885] != 0.0)) {
            s.store_offset_ad(314, A::sub_from_scalar(p.p85, s.ad_value(350)), (-0.01));
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (!(s.v[867] != 0.0))) && (s.v[885] != 0.0)) {
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (!(s.v[867] != 0.0))) && (s.v[885] != 0.0)) {
            s.store_ad(315, &{
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (!(s.v[867] != 0.0))) && (s.v[885] != 0.0)) {
            s.store_sqrt_ad(315, A::add(A::square(s.ad_value(314)), s.ad_value(315)));
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (!(s.v[867] != 0.0))) && (s.v[885] != 0.0)) {
            s.store_sub_from_scalar_ad(350, p.p85, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5));
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (!(s.v[867] != 0.0))) && (s.v[885] != 0.0)) {
            s.store_offset(314, 350, (((-s.v[63])) + ((-0.01))));
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (!(s.v[867] != 0.0))) && (s.v[885] != 0.0)) {
            s.store_scalar(315, ((4.0 * s.v[63]) * 0.01));
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (!(s.v[867] != 0.0))) && (s.v[885] != 0.0)) {
            s.store_ad(315, &{
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (!(s.v[867] != 0.0))) && (s.v[885] != 0.0)) {
            s.store_sqrt_ad(315, A::add(A::square(s.ad_value(314)), s.ad_value(315)));
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (!(s.v[867] != 0.0))) && (s.v[885] != 0.0)) {
            s.store_offset_ad(350, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5), s.v[63]);
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (!(s.v[867] != 0.0))) && (s.v[885] != 0.0)) {
            s.store_mul_ad_lhs(366, A::scale(s.ad_value(364), p.p86), 365);
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (!(s.v[867] != 0.0))) && (!(s.v[885] != 0.0))) {
            s.store_scalar(350, s.v[63]);
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (!(s.v[867] != 0.0))) && (!(s.v[885] != 0.0))) {
            s.store_scalar(359, s.v[63]);
        }

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (!(s.v[867] != 0.0))) && (!(s.v[885] != 0.0))) {
            s.store_scalar(366, 0.0);
        }

        s.v[886] = if ((((s.v[85] * ((s.v[262] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85))))) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (!(s.v[867] != 0.0))) && (s.v[886] != 0.0)) {
            s.store_exp_ad(283, A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]));
        }

        s.v[887] = if ((s.v[85] * ((s.v[262] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85)))) < (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (!(s.v[867] != 0.0))) && (!(s.v[886] != 0.0))) && (s.v[887] != 0.0)) {
            let assign31430_ad_e47356: A = A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85])), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85])), 0.3333333333333333), 1.0));
            s.store_div_from_scalar_ad(283, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85])), A::offset(A::scale(assign31430_ad_e47356, 0.5), 1.0)), 1.0));
        }

        if (((((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (!(s.v[867] != 0.0))) && (!(s.v[886] != 0.0))) && (!(s.v[887] != 0.0))) {
            let assign31440_ad_e47436: A = A::mul(A::offset(A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]), (-230.25850929940458)), A::offset(A::scale(A::offset(A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]), (-230.25850929940458)), 0.3333333333333333), 1.0));
            s.store_scale_ad(283, A::offset(A::mul(A::offset(A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]), (-230.25850929940458)), A::offset(A::scale(assign31440_ad_e47436, 0.5), 1.0)), 1.0), 1e100);
        }

        if (((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (!(s.v[867] != 0.0))) {
            s.store_scale_ad(367, A::add(A::div(A::sub(s.ad_value(359), A::mul(s.ad_value(262), s.ad_value(366))), A::square(s.ad_value(359))), A::div(A::mul(s.ad_value(362), s.ad_value(366)), A::scale(s.ad_value(350), p.p85))), s.v[85]);
        }

        if (((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (!(s.v[867] != 0.0))) {
            s.store_mul_ad_lhs(372, A::offset(A::mul(A::sub(s.ad_value(277), s.ad_value(262)), s.ad_value(367)), 1.0), 283);
        }

        if ((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) {
            s.store_offset(370, 370, (-1.0));
        }

        if ((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) {
            s.store_offset(371, 371, (-1.0));
        }

        if ((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) {
            s.store_offset(372, 372, (-1.0));
        }

        if ((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) {
            s.store_div_from_scalar(222, 1.0, 223);
        }

        s.v[888] = if (s.v[277] > 0.0) { 1.0 } else { 0.0 };

        if (((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (s.v[888] != 0.0)) {
            s.store_scale_ad(224, A::ln(A::add(A::offset(s.ad_value(222), 2.0), A::sqrt(A::mul(A::offset(s.ad_value(222), 1.0), A::offset(s.ad_value(222), 3.0))))), (s.v[84] * 2.0));
        }

        if (((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) && (!(s.v[888] != 0.0))) {
            s.store_sub_ad_lhs(224, A::scale(A::ln(A::add(A::offset(A::scale(s.ad_value(223), 2.0), 1.0), A::sqrt(A::mul(A::offset(s.ad_value(223), 1.0), A::offset(A::scale(s.ad_value(223), 3.0), 1.0))))), (s.v[84] * 2.0)), 277);
        }

        if ((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) {
            s.store_sub(225, 264, 224);
        }

        if ((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) {
            s.store_scale_ad(226, A::sub(A::add(s.ad_value(277), s.ad_value(225)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(277), s.ad_value(225)), A::sub(s.ad_value(277), s.ad_value(225))), ((4.0 * s.v[84]) * s.v[84])))), 0.5);
        }

        if ((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) {
            s.store_scale_ad(227, A::sub(A::add(s.ad_value(277), s.ad_value(267)), A::sqrt(A::add(A::mul(A::sub(s.ad_value(277), s.ad_value(267)), A::sub(s.ad_value(277), s.ad_value(267))), A::mul(A::scale(s.ad_value(82), 4.0), s.ad_value(82))))), 0.5);
        }

        if ((!(s.v[858] != 0.0)) && (s.v[866] != 0.0)) {
            s.store_scale_ad(228, A::sub(s.ad_value(277), A::sqrt(A::offset(A::mul(s.ad_value(277), s.ad_value(277)), ((4.0 * 1e-6) * 1e-6)))), 0.5);
        }

        if ((!(s.v[858] != 0.0)) && (!(s.v[866] != 0.0))) {
            s.store_scalar(370, 0.0);
        }

        if ((!(s.v[858] != 0.0)) && (!(s.v[866] != 0.0))) {
            s.store_scalar(371, 0.0);
        }

        if ((!(s.v[858] != 0.0)) && (!(s.v[866] != 0.0))) {
            s.store_scalar(372, 0.0);
        }

        if ((!(s.v[858] != 0.0)) && (!(s.v[866] != 0.0))) {
            s.store_scalar(224, 0.0);
        }

        if ((!(s.v[858] != 0.0)) && (!(s.v[866] != 0.0))) {
            s.store_scalar(221, 0.0);
        }

        if ((!(s.v[858] != 0.0)) && (!(s.v[866] != 0.0))) {
            s.store_scalar(223, 0.0);
        }

        if ((!(s.v[858] != 0.0)) && (!(s.v[866] != 0.0))) {
            s.store_scalar(226, 0.0);
        }

        if ((!(s.v[858] != 0.0)) && (!(s.v[866] != 0.0))) {
            s.store_scalar(227, 0.0);
        }

        if ((!(s.v[858] != 0.0)) && (!(s.v[866] != 0.0))) {
            s.store_scalar(228, 0.0);
        }

        s.v[889] = if (s.v[256] == 0.0) { 1.0 } else { 0.0 };

        if ((!(s.v[858] != 0.0)) && (s.v[889] != 0.0)) {
            s.store_scalar(268, 0.0);
        }

        if ((!(s.v[858] != 0.0)) && (s.v[889] != 0.0)) {
            s.store_scalar(291, 0.0);
        }

        if ((!(s.v[858] != 0.0)) && (s.v[889] != 0.0)) {
            s.store_scalar(269, 0.0);
        }

        s.v[890] = if (s.v[122] == 0.5) { 1.0 } else { 0.0 };

        if (((!(s.v[858] != 0.0)) && (!(s.v[889] != 0.0))) && (s.v[890] != 0.0)) {
            s.store_sqrt_ad(229, A::sub_from_scalar(1.0, A::mul(s.ad_value(221), s.ad_value(119))));
        }

        if (((!(s.v[858] != 0.0)) && (!(s.v[889] != 0.0))) && (!(s.v[890] != 0.0))) {
            s.store_powf_ad(229, A::sub_from_scalar(1.0, A::mul(s.ad_value(221), s.ad_value(119))), s.v[122]);
        }

    }

    pub(super) fn stamp_transient_block_24(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if ((!(s.v[858] != 0.0)) && (!(s.v[889] != 0.0))) {
            s.store_add_ad(269, A::mul(s.ad_value(131), A::sub_from_scalar(1.0, s.ad_value(229))), A::mul(s.ad_value(134), A::sub(s.ad_value(277), s.ad_value(221))));
        }

        if ((!(s.v[858] != 0.0)) && (!(s.v[889] != 0.0))) {
            s.store_mul(230, 101, 370);
        }

        s.v[891] = if ((s.v[20] == 0.0) && (s.v[23] == 0.0)) { 1.0 } else { 0.0 };

        if (((!(s.v[858] != 0.0)) && (!(s.v[889] != 0.0))) && (s.v[891] != 0.0)) {
            s.store_scalar(232, 0.0);
        }

        if (((!(s.v[858] != 0.0)) && (!(s.v[889] != 0.0))) && (s.v[891] != 0.0)) {
            s.store_scalar(235, 0.0);
        }

        if (((!(s.v[858] != 0.0)) && (!(s.v[889] != 0.0))) && (s.v[891] != 0.0)) {
            s.store_scalar(236, 0.0);
        }

        if (((!(s.v[858] != 0.0)) && (!(s.v[889] != 0.0))) && (s.v[891] != 0.0)) {
            s.store_scalar(237, 0.0);
        }

        if (((!(s.v[858] != 0.0)) && (!(s.v[889] != 0.0))) && (s.v[891] != 0.0)) {
            s.store_scalar(231, 0.0);
        }

        if (((!(s.v[858] != 0.0)) && (!(s.v[889] != 0.0))) && (!(s.v[891] != 0.0))) {
            s.store_sub(232, 107, 226);
        }

        if (((!(s.v[858] != 0.0)) && (!(s.v[889] != 0.0))) && (!(s.v[891] != 0.0))) {
            s.store_sub_from_scalar_ad(233, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(224), s.ad_value(232)))));
        }

        s.v[892] = if (s.v[9] == 0.5) { 1.0 } else { 0.0 };

        if ((((!(s.v[858] != 0.0)) && (!(s.v[889] != 0.0))) && (!(s.v[891] != 0.0))) && (s.v[892] != 0.0)) {
            s.store_scalar(234, 0.0);
        }

        if ((((!(s.v[858] != 0.0)) && (!(s.v[889] != 0.0))) && (!(s.v[891] != 0.0))) && (!(s.v[892] != 0.0))) {
            s.store_scale_ad(234, A::add(A::div(A::mul(A::square(s.ad_value(233)), A::ln(s.ad_value(233))), A::sub_from_scalar(1.0, s.ad_value(233))), s.ad_value(233)), (1.0 - (2.0 * s.v[9])));
        }

        if (((!(s.v[858] != 0.0)) && (!(s.v[889] != 0.0))) && (!(s.v[891] != 0.0))) {
            s.store_add(235, 233, 234);
        }

        s.v[893] = if (s.v[9] == 0.5) { 1.0 } else { 0.0 };

        if ((((!(s.v[858] != 0.0)) && (!(s.v[889] != 0.0))) && (!(s.v[891] != 0.0))) && (s.v[893] != 0.0)) {
            s.store_sqrt_ad(229, A::scale(s.ad_value(232), s.v[143]));
        }

        if ((((!(s.v[858] != 0.0)) && (!(s.v[889] != 0.0))) && (!(s.v[891] != 0.0))) && (!(s.v[893] != 0.0))) {
            s.store_powf_ad(229, A::scale(s.ad_value(232), s.v[143]), s.v[9]);
        }

        if (((!(s.v[858] != 0.0)) && (!(s.v[889] != 0.0))) && (!(s.v[891] != 0.0))) {
            s.store_scale(236, 229, s.v[137]);
        }

        if (((!(s.v[858] != 0.0)) && (!(s.v[889] != 0.0))) && (!(s.v[891] != 0.0))) {
            s.store_mul_ad_rhs(237, 98, A::mul(A::offset(s.ad_value(223), (-1.0)), s.ad_value(236)));
        }

        if (((!(s.v[858] != 0.0)) && (!(s.v[889] != 0.0))) && (!(s.v[891] != 0.0))) {
            s.store_scaled_mul(231, 237, 235, s.v[20]);
        }

        s.v[894] = if (s.v[23] == 0.0) { 1.0 } else { 0.0 };

        if (((!(s.v[858] != 0.0)) && (!(s.v[889] != 0.0))) && (s.v[894] != 0.0)) {
            s.store_scalar(238, 0.0);
        }

        if (((!(s.v[858] != 0.0)) && (!(s.v[889] != 0.0))) && (!(s.v[894] != 0.0))) {
            s.store_scale_ad(239, A::div(A::scale(s.ad_value(236), s.v[122]), s.ad_value(232)), s.v[152]);
        }

        if (((!(s.v[858] != 0.0)) && (!(s.v[889] != 0.0))) && (!(s.v[894] != 0.0))) {
            s.store_div_from_scalar(240, (0.666666666666667 * s.v[149]), 239);
        }

        if (((!(s.v[858] != 0.0)) && (!(s.v[889] != 0.0))) && (!(s.v[894] != 0.0))) {
            s.store_square(241, 240);
        }

        if (((!(s.v[858] != 0.0)) && (!(s.v[889] != 0.0))) && (!(s.v[894] != 0.0))) {
            s.store_sqrt_ad(242, A::div(A::square(s.ad_value(241)), A::offset(A::square(s.ad_value(241)), 1.0)));
        }

        if (((!(s.v[858] != 0.0)) && (!(s.v[889] != 0.0))) && (!(s.v[894] != 0.0))) {
            s.store_sqrt_ad(243, A::abs(s.ad_value(242)));
        }

        if (((!(s.v[858] != 0.0)) && (!(s.v[889] != 0.0))) && (!(s.v[894] != 0.0))) {
            s.store_mul(244, 242, 243);
        }

        s.v[895] = if (((-s.v[9]) * s.v[125]) == (-1.0)) { 1.0 } else { 0.0 };

        if ((((!(s.v[858] != 0.0)) && (!(s.v[889] != 0.0))) && (!(s.v[894] != 0.0))) && (s.v[895] != 0.0)) {
            s.store_div_from_scalar_ad(245, 1.0, A::offset(A::mul(s.ad_value(239), s.ad_value(244)), 1.0));
        }

        if ((((!(s.v[858] != 0.0)) && (!(s.v[889] != 0.0))) && (!(s.v[894] != 0.0))) && (!(s.v[895] != 0.0))) {
            s.store_powf_ad(245, A::offset(A::mul(s.ad_value(239), s.ad_value(244)), 1.0), ((-s.v[9]) * s.v[125]));
        }

        if (((!(s.v[858] != 0.0)) && (!(s.v[889] != 0.0))) && (!(s.v[894] != 0.0))) {
            s.store_div_ad(246, A::mul(s.ad_value(235), s.ad_value(245)), A::add(s.ad_value(235), s.ad_value(245)));
        }

        if (((!(s.v[858] != 0.0)) && (!(s.v[889] != 0.0))) && (!(s.v[894] != 0.0))) {
            s.store_sqrt_ad(247, A::scale(A::div(s.ad_value(239), s.ad_value(243)), 0.375));
        }

        if (((!(s.v[858] != 0.0)) && (!(s.v[889] != 0.0))) && (!(s.v[894] != 0.0))) {
            s.store_sub_ad_lhs(248, A::scale(A::mul(s.ad_value(240), s.ad_value(243)), 2.0), 242);
        }

        if (((!(s.v[858] != 0.0)) && (!(s.v[889] != 0.0))) && (!(s.v[894] != 0.0))) {
            s.store_add_ad(249, A::sub(A::mul(A::scale(s.ad_value(240), s.v[149]), s.ad_value(243)), A::scale(s.ad_value(242), s.v[149])), A::scale(A::mul(s.ad_value(239), s.ad_value(244)), 0.5));
        }

        if (((!(s.v[858] != 0.0)) && (!(s.v[889] != 0.0))) && (!(s.v[894] != 0.0))) {
            s.store_mul_ad_lhs(250, A::offset(s.ad_value(248), (-1.0)), 247);
        }

        if (((!(s.v[858] != 0.0)) && (!(s.v[889] != 0.0))) && (!(s.v[894] != 0.0))) {
            s.store_square(212, 250);
        }

        s.v[896] = if (s.v[250] > 0.0) { 1.0 } else { 0.0 };

        if ((((!(s.v[858] != 0.0)) && (!(s.v[889] != 0.0))) && (!(s.v[894] != 0.0))) && (s.v[896] != 0.0)) {
            s.store_div_from_scalar_ad(213, 1.0, A::offset(A::scale(s.ad_value(250), s.v[86]), 1.0));
        }

        if ((((!(s.v[858] != 0.0)) && (!(s.v[889] != 0.0))) && (!(s.v[894] != 0.0))) && (!(s.v[896] != 0.0))) {
            s.store_div_from_scalar_ad(213, 1.0, A::sub_from_scalar(1.0, A::scale(s.ad_value(250), s.v[86])));
        }

        s.v[897] = if (((-s.v[212]) + s.v[249]) > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((!(s.v[858] != 0.0)) && (!(s.v[889] != 0.0))) && (!(s.v[894] != 0.0))) && (s.v[897] != 0.0)) {
            s.store_exp_ad(229, A::sub(s.ad_value(249), s.ad_value(212)));
        }

        if ((((!(s.v[858] != 0.0)) && (!(s.v[889] != 0.0))) && (!(s.v[894] != 0.0))) && (!(s.v[897] != 0.0))) {
            s.store_div_from_scalar_ad(229, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(249), s.ad_value(212))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(249), s.ad_value(212))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(249), s.ad_value(212))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if (((!(s.v[858] != 0.0)) && (!(s.v[889] != 0.0))) && (!(s.v[894] != 0.0))) {
            s.store_mul_ad_lhs(214, A::add(A::add(A::scale(s.ad_value(213), 0.29214664), A::scale(A::square(s.ad_value(213)), s.v[87])), A::scale(A::mul(A::square(s.ad_value(213)), s.ad_value(213)), s.v[88])), 229);
        }

        s.v[898] = if (s.v[250] > 0.0) { 1.0 } else { 0.0 };

        if ((((!(s.v[858] != 0.0)) && (!(s.v[889] != 0.0))) && (!(s.v[894] != 0.0))) && (s.v[898] != 0.0)) {
            s.copy_ad(251, 214);
        }

        s.v[899] = if (s.v[249] > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((!(s.v[858] != 0.0)) && (!(s.v[889] != 0.0))) && (!(s.v[894] != 0.0))) && (!(s.v[898] != 0.0))) && (s.v[899] != 0.0)) {
            s.store_exp(229, 249);
        }

        if (((((!(s.v[858] != 0.0)) && (!(s.v[889] != 0.0))) && (!(s.v[894] != 0.0))) && (!(s.v[898] != 0.0))) && (!(s.v[899] != 0.0))) {
            s.store_div_from_scalar_ad(229, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(249)), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(249)), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), s.ad_value(249)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if ((((!(s.v[858] != 0.0)) && (!(s.v[889] != 0.0))) && (!(s.v[894] != 0.0))) && (!(s.v[898] != 0.0))) {
            s.store_sub_ad_lhs(251, A::scale(s.ad_value(229), 2.0), 214);
        }

        if (((!(s.v[858] != 0.0)) && (!(s.v[889] != 0.0))) && (!(s.v[894] != 0.0))) {
            s.store_scale_ad(252, A::div(A::scale(s.ad_value(251), s.v[149]), s.ad_value(247)), (1.772453850905516 * 0.5));
        }

        if (((!(s.v[858] != 0.0)) && (!(s.v[889] != 0.0))) && (!(s.v[894] != 0.0))) {
            s.store_scale_ad(238, A::mul(A::mul(s.ad_value(237), s.ad_value(252)), s.ad_value(246)), s.v[23]);
        }

        s.v[900] = if (s.v[29] == 0.0) { 1.0 } else { 0.0 };

        if (((!(s.v[858] != 0.0)) && (!(s.v[889] != 0.0))) && (s.v[900] != 0.0)) {
            s.store_scalar(253, 0.0);
        }

        s.v[901] = if (s.v[9] == 0.5) { 1.0 } else { 0.0 };

        if ((((!(s.v[858] != 0.0)) && (!(s.v[889] != 0.0))) && (!(s.v[900] != 0.0))) && (s.v[901] != 0.0)) {
            s.store_sqrt_ad(229, A::scale(A::sub_from_scalar(s.v[6], s.ad_value(227)), s.v[143]));
        }

        if ((((!(s.v[858] != 0.0)) && (!(s.v[889] != 0.0))) && (!(s.v[900] != 0.0))) && (!(s.v[901] != 0.0))) {
            s.store_powf_ad(229, A::scale(A::sub_from_scalar(s.v[6], s.ad_value(227)), s.v[143]), s.v[9]);
        }

        if (((!(s.v[858] != 0.0)) && (!(s.v[889] != 0.0))) && (!(s.v[900] != 0.0))) {
            s.store_scale_ad(254, A::div(A::scale(A::sub_from_scalar(s.v[6], s.ad_value(227)), s.v[140]), s.ad_value(229)), s.v[125]);
        }

        s.v[902] = if (((((-s.v[155]) / s.v[254])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((((!(s.v[858] != 0.0)) && (!(s.v[889] != 0.0))) && (!(s.v[900] != 0.0))) && (s.v[902] != 0.0)) {
            s.store_exp_ad(229, A::div(A::neg(s.ad_value(155)), s.ad_value(254)));
        }

        s.v[903] = if (((-s.v[155]) / s.v[254]) < (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((!(s.v[858] != 0.0)) && (!(s.v[889] != 0.0))) && (!(s.v[900] != 0.0))) && (!(s.v[902] != 0.0))) && (s.v[903] != 0.0)) {
            let assign32350_ad_e48745: A = A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(155)), s.ad_value(254))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(155)), s.ad_value(254))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(155)), s.ad_value(254))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
            s.store_div_from_scalar_ad(229, 1e-100, assign32350_ad_e48745);
        }

        if (((((!(s.v[858] != 0.0)) && (!(s.v[889] != 0.0))) && (!(s.v[900] != 0.0))) && (!(s.v[902] != 0.0))) && (!(s.v[903] != 0.0))) {
            let assign32360_ad_e48794: A = A::scale(A::offset(A::mul(A::offset(A::div(A::neg(s.ad_value(155)), s.ad_value(254)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::div(A::neg(s.ad_value(155)), s.ad_value(254)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::div(A::neg(s.ad_value(155)), s.ad_value(254)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
            s.store_ad(229, &assign32360_ad_e48794);
        }

        if (((!(s.v[858] != 0.0)) && (!(s.v[889] != 0.0))) && (!(s.v[900] != 0.0))) {
            s.store_scale_ad(253, A::mul(A::mul(A::mul(s.ad_value(277), s.ad_value(254)), s.ad_value(254)), s.ad_value(229)), s.v[29]);
        }

        s.v[904] = if ((s.v[38] > 1000000.0) || (p.p80 == 0.0)) { 1.0 } else { 0.0 };

        if (((!(s.v[858] != 0.0)) && (!(s.v[889] != 0.0))) && (s.v[904] != 0.0)) {
            s.store_scalar(255, 1.0);
        }

        s.v[905] = if (s.v[228] > ((-s.v[158]) * s.v[38])) { 1.0 } else { 0.0 };

        s.v[906] = if (s.v[41] == 4.0) { 1.0 } else { 0.0 };

        if (((((!(s.v[858] != 0.0)) && (!(s.v[889] != 0.0))) && (!(s.v[904] != 0.0))) && (s.v[905] != 0.0)) && (s.v[906] != 0.0)) {
            s.store_mul_ad(229, A::mul(A::mul(A::abs(A::mul(s.ad_value(228), s.ad_value(162))), A::abs(A::mul(s.ad_value(228), s.ad_value(162)))), A::abs(A::mul(s.ad_value(228), s.ad_value(162)))), A::abs(A::mul(s.ad_value(228), s.ad_value(162))));
        }

        if (((((!(s.v[858] != 0.0)) && (!(s.v[889] != 0.0))) && (!(s.v[904] != 0.0))) && (s.v[905] != 0.0)) && (!(s.v[906] != 0.0))) {
            s.store_powf_ad(229, A::abs(A::mul(s.ad_value(228), s.ad_value(162))), s.v[41]);
        }

        if ((((!(s.v[858] != 0.0)) && (!(s.v[889] != 0.0))) && (!(s.v[904] != 0.0))) && (s.v[905] != 0.0)) {
            s.store_div_from_scalar_ad(255, 1.0, A::sub_from_scalar(1.0, s.ad_value(229)));
        }

        if ((((!(s.v[858] != 0.0)) && (!(s.v[889] != 0.0))) && (!(s.v[904] != 0.0))) && (!(s.v[905] != 0.0))) {
            s.store_offset_ad(255, A::mul(A::add(s.ad_value(228), A::scale(s.ad_value(38), s.v[158])), s.ad_value(165)), s.v[159]);
        }

        if ((!(s.v[858] != 0.0)) && (!(s.v[889] != 0.0))) {
            s.store_mul_ad_lhs(268, A::add(A::add(A::add(s.ad_value(230), s.ad_value(231)), s.ad_value(238)), s.ad_value(253)), 255);
        }

        if ((!(s.v[858] != 0.0)) && (!(s.v[889] != 0.0))) {
            s.store_mul_ad_lhs(291, A::add(A::add(s.ad_value(231), s.ad_value(238)), s.ad_value(253)), 255);
        }

        s.v[907] = if (s.v[257] == 0.0) { 1.0 } else { 0.0 };

        if ((!(s.v[858] != 0.0)) && (s.v[907] != 0.0)) {
            s.store_scalar(270, 0.0);
        }

        if ((!(s.v[858] != 0.0)) && (s.v[907] != 0.0)) {
            s.store_scalar(292, 0.0);
        }

        if ((!(s.v[858] != 0.0)) && (s.v[907] != 0.0)) {
            s.store_scalar(271, 0.0);
        }

        s.v[908] = if (s.v[123] == 0.5) { 1.0 } else { 0.0 };

        if (((!(s.v[858] != 0.0)) && (!(s.v[907] != 0.0))) && (s.v[908] != 0.0)) {
            s.store_sqrt_ad(229, A::sub_from_scalar(1.0, A::mul(s.ad_value(221), s.ad_value(120))));
        }

        if (((!(s.v[858] != 0.0)) && (!(s.v[907] != 0.0))) && (!(s.v[908] != 0.0))) {
            s.store_powf_ad(229, A::sub_from_scalar(1.0, A::mul(s.ad_value(221), s.ad_value(120))), s.v[123]);
        }

        if ((!(s.v[858] != 0.0)) && (!(s.v[907] != 0.0))) {
            s.store_add_ad(271, A::mul(s.ad_value(132), A::sub_from_scalar(1.0, s.ad_value(229))), A::mul(s.ad_value(135), A::sub(s.ad_value(277), s.ad_value(221))));
        }

        if ((!(s.v[858] != 0.0)) && (!(s.v[907] != 0.0))) {
            s.store_mul(230, 102, 371);
        }

        s.v[909] = if ((s.v[21] == 0.0) && (s.v[24] == 0.0)) { 1.0 } else { 0.0 };

        if (((!(s.v[858] != 0.0)) && (!(s.v[907] != 0.0))) && (s.v[909] != 0.0)) {
            s.store_scalar(232, 0.0);
        }

        if (((!(s.v[858] != 0.0)) && (!(s.v[907] != 0.0))) && (s.v[909] != 0.0)) {
            s.store_scalar(235, 0.0);
        }

        if (((!(s.v[858] != 0.0)) && (!(s.v[907] != 0.0))) && (s.v[909] != 0.0)) {
            s.store_scalar(236, 0.0);
        }

        if (((!(s.v[858] != 0.0)) && (!(s.v[907] != 0.0))) && (s.v[909] != 0.0)) {
            s.store_scalar(237, 0.0);
        }

        if (((!(s.v[858] != 0.0)) && (!(s.v[907] != 0.0))) && (s.v[909] != 0.0)) {
            s.store_scalar(231, 0.0);
        }

        if (((!(s.v[858] != 0.0)) && (!(s.v[907] != 0.0))) && (!(s.v[909] != 0.0))) {
            s.store_sub(232, 108, 226);
        }

        if (((!(s.v[858] != 0.0)) && (!(s.v[907] != 0.0))) && (!(s.v[909] != 0.0))) {
            s.store_sub_from_scalar_ad(233, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(224), s.ad_value(232)))));
        }

        s.v[910] = if (s.v[10] == 0.5) { 1.0 } else { 0.0 };

        if ((((!(s.v[858] != 0.0)) && (!(s.v[907] != 0.0))) && (!(s.v[909] != 0.0))) && (s.v[910] != 0.0)) {
            s.store_scalar(234, 0.0);
        }

        if ((((!(s.v[858] != 0.0)) && (!(s.v[907] != 0.0))) && (!(s.v[909] != 0.0))) && (!(s.v[910] != 0.0))) {
            s.store_scale_ad(234, A::add(A::div(A::mul(A::square(s.ad_value(233)), A::ln(s.ad_value(233))), A::sub_from_scalar(1.0, s.ad_value(233))), s.ad_value(233)), (1.0 - (2.0 * s.v[10])));
        }

        if (((!(s.v[858] != 0.0)) && (!(s.v[907] != 0.0))) && (!(s.v[909] != 0.0))) {
            s.store_add(235, 233, 234);
        }

        s.v[911] = if (s.v[10] == 0.5) { 1.0 } else { 0.0 };

        if ((((!(s.v[858] != 0.0)) && (!(s.v[907] != 0.0))) && (!(s.v[909] != 0.0))) && (s.v[911] != 0.0)) {
            s.store_sqrt_ad(229, A::scale(s.ad_value(232), s.v[144]));
        }

        if ((((!(s.v[858] != 0.0)) && (!(s.v[907] != 0.0))) && (!(s.v[909] != 0.0))) && (!(s.v[911] != 0.0))) {
            s.store_powf_ad(229, A::scale(s.ad_value(232), s.v[144]), s.v[10]);
        }

        if (((!(s.v[858] != 0.0)) && (!(s.v[907] != 0.0))) && (!(s.v[909] != 0.0))) {
            s.store_scale(236, 229, s.v[138]);
        }

        if (((!(s.v[858] != 0.0)) && (!(s.v[907] != 0.0))) && (!(s.v[909] != 0.0))) {
            s.store_mul_ad_rhs(237, 99, A::mul(A::offset(s.ad_value(223), (-1.0)), s.ad_value(236)));
        }

        if (((!(s.v[858] != 0.0)) && (!(s.v[907] != 0.0))) && (!(s.v[909] != 0.0))) {
            s.store_scaled_mul(231, 237, 235, s.v[21]);
        }

        s.v[912] = if (s.v[24] == 0.0) { 1.0 } else { 0.0 };

        if (((!(s.v[858] != 0.0)) && (!(s.v[907] != 0.0))) && (s.v[912] != 0.0)) {
            s.store_scalar(238, 0.0);
        }

        if (((!(s.v[858] != 0.0)) && (!(s.v[907] != 0.0))) && (!(s.v[912] != 0.0))) {
            s.store_scale_ad(239, A::div(A::scale(s.ad_value(236), s.v[123]), s.ad_value(232)), s.v[153]);
        }

        if (((!(s.v[858] != 0.0)) && (!(s.v[907] != 0.0))) && (!(s.v[912] != 0.0))) {
            s.store_div_from_scalar(240, (0.666666666666667 * s.v[150]), 239);
        }

        if (((!(s.v[858] != 0.0)) && (!(s.v[907] != 0.0))) && (!(s.v[912] != 0.0))) {
            s.store_square(241, 240);
        }

        if (((!(s.v[858] != 0.0)) && (!(s.v[907] != 0.0))) && (!(s.v[912] != 0.0))) {
            s.store_sqrt_ad(242, A::div(A::square(s.ad_value(241)), A::offset(A::square(s.ad_value(241)), 1.0)));
        }

        if (((!(s.v[858] != 0.0)) && (!(s.v[907] != 0.0))) && (!(s.v[912] != 0.0))) {
            s.store_sqrt_ad(243, A::abs(s.ad_value(242)));
        }

        if (((!(s.v[858] != 0.0)) && (!(s.v[907] != 0.0))) && (!(s.v[912] != 0.0))) {
            s.store_mul(244, 242, 243);
        }

        s.v[913] = if (((-s.v[10]) * s.v[126]) == (-1.0)) { 1.0 } else { 0.0 };

        if ((((!(s.v[858] != 0.0)) && (!(s.v[907] != 0.0))) && (!(s.v[912] != 0.0))) && (s.v[913] != 0.0)) {
            s.store_div_from_scalar_ad(245, 1.0, A::offset(A::mul(s.ad_value(239), s.ad_value(244)), 1.0));
        }

        if ((((!(s.v[858] != 0.0)) && (!(s.v[907] != 0.0))) && (!(s.v[912] != 0.0))) && (!(s.v[913] != 0.0))) {
            s.store_powf_ad(245, A::offset(A::mul(s.ad_value(239), s.ad_value(244)), 1.0), ((-s.v[10]) * s.v[126]));
        }

        if (((!(s.v[858] != 0.0)) && (!(s.v[907] != 0.0))) && (!(s.v[912] != 0.0))) {
            s.store_div_ad(246, A::mul(s.ad_value(235), s.ad_value(245)), A::add(s.ad_value(235), s.ad_value(245)));
        }

        if (((!(s.v[858] != 0.0)) && (!(s.v[907] != 0.0))) && (!(s.v[912] != 0.0))) {
            s.store_sqrt_ad(247, A::scale(A::div(s.ad_value(239), s.ad_value(243)), 0.375));
        }

        if (((!(s.v[858] != 0.0)) && (!(s.v[907] != 0.0))) && (!(s.v[912] != 0.0))) {
            s.store_sub_ad_lhs(248, A::scale(A::mul(s.ad_value(240), s.ad_value(243)), 2.0), 242);
        }

        if (((!(s.v[858] != 0.0)) && (!(s.v[907] != 0.0))) && (!(s.v[912] != 0.0))) {
            s.store_add_ad(249, A::sub(A::mul(A::scale(s.ad_value(240), s.v[150]), s.ad_value(243)), A::scale(s.ad_value(242), s.v[150])), A::scale(A::mul(s.ad_value(239), s.ad_value(244)), 0.5));
        }

        if (((!(s.v[858] != 0.0)) && (!(s.v[907] != 0.0))) && (!(s.v[912] != 0.0))) {
            s.store_mul_ad_lhs(250, A::offset(s.ad_value(248), (-1.0)), 247);
        }

        if (((!(s.v[858] != 0.0)) && (!(s.v[907] != 0.0))) && (!(s.v[912] != 0.0))) {
            s.store_square(212, 250);
        }

        s.v[914] = if (s.v[250] > 0.0) { 1.0 } else { 0.0 };

        if ((((!(s.v[858] != 0.0)) && (!(s.v[907] != 0.0))) && (!(s.v[912] != 0.0))) && (s.v[914] != 0.0)) {
            s.store_div_from_scalar_ad(213, 1.0, A::offset(A::scale(s.ad_value(250), s.v[86]), 1.0));
        }

        if ((((!(s.v[858] != 0.0)) && (!(s.v[907] != 0.0))) && (!(s.v[912] != 0.0))) && (!(s.v[914] != 0.0))) {
            s.store_div_from_scalar_ad(213, 1.0, A::sub_from_scalar(1.0, A::scale(s.ad_value(250), s.v[86])));
        }

        s.v[915] = if (((-s.v[212]) + s.v[249]) > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((!(s.v[858] != 0.0)) && (!(s.v[907] != 0.0))) && (!(s.v[912] != 0.0))) && (s.v[915] != 0.0)) {
            s.store_exp_ad(229, A::sub(s.ad_value(249), s.ad_value(212)));
        }

        if ((((!(s.v[858] != 0.0)) && (!(s.v[907] != 0.0))) && (!(s.v[912] != 0.0))) && (!(s.v[915] != 0.0))) {
            s.store_div_from_scalar_ad(229, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(249), s.ad_value(212))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(249), s.ad_value(212))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(249), s.ad_value(212))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if (((!(s.v[858] != 0.0)) && (!(s.v[907] != 0.0))) && (!(s.v[912] != 0.0))) {
            s.store_mul_ad_lhs(214, A::add(A::add(A::scale(s.ad_value(213), 0.29214664), A::scale(A::square(s.ad_value(213)), s.v[87])), A::scale(A::mul(A::square(s.ad_value(213)), s.ad_value(213)), s.v[88])), 229);
        }

        s.v[916] = if (s.v[250] > 0.0) { 1.0 } else { 0.0 };

        if ((((!(s.v[858] != 0.0)) && (!(s.v[907] != 0.0))) && (!(s.v[912] != 0.0))) && (s.v[916] != 0.0)) {
            s.copy_ad(251, 214);
        }

        s.v[917] = if (s.v[249] > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((!(s.v[858] != 0.0)) && (!(s.v[907] != 0.0))) && (!(s.v[912] != 0.0))) && (!(s.v[916] != 0.0))) && (s.v[917] != 0.0)) {
            s.store_exp(229, 249);
        }

        if (((((!(s.v[858] != 0.0)) && (!(s.v[907] != 0.0))) && (!(s.v[912] != 0.0))) && (!(s.v[916] != 0.0))) && (!(s.v[917] != 0.0))) {
            s.store_div_from_scalar_ad(229, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(249)), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(249)), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), s.ad_value(249)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if ((((!(s.v[858] != 0.0)) && (!(s.v[907] != 0.0))) && (!(s.v[912] != 0.0))) && (!(s.v[916] != 0.0))) {
            s.store_sub_ad_lhs(251, A::scale(s.ad_value(229), 2.0), 214);
        }

        if (((!(s.v[858] != 0.0)) && (!(s.v[907] != 0.0))) && (!(s.v[912] != 0.0))) {
            s.store_scale_ad(252, A::div(A::scale(s.ad_value(251), s.v[150]), s.ad_value(247)), (1.772453850905516 * 0.5));
        }

        if (((!(s.v[858] != 0.0)) && (!(s.v[907] != 0.0))) && (!(s.v[912] != 0.0))) {
            s.store_scale_ad(238, A::mul(A::mul(s.ad_value(237), s.ad_value(252)), s.ad_value(246)), s.v[24]);
        }

        s.v[918] = if (s.v[30] == 0.0) { 1.0 } else { 0.0 };

        if (((!(s.v[858] != 0.0)) && (!(s.v[907] != 0.0))) && (s.v[918] != 0.0)) {
            s.store_scalar(253, 0.0);
        }

        s.v[919] = if (s.v[10] == 0.5) { 1.0 } else { 0.0 };

        if ((((!(s.v[858] != 0.0)) && (!(s.v[907] != 0.0))) && (!(s.v[918] != 0.0))) && (s.v[919] != 0.0)) {
            s.store_sqrt_ad(229, A::scale(A::sub_from_scalar(s.v[7], s.ad_value(227)), s.v[144]));
        }

        if ((((!(s.v[858] != 0.0)) && (!(s.v[907] != 0.0))) && (!(s.v[918] != 0.0))) && (!(s.v[919] != 0.0))) {
            s.store_powf_ad(229, A::scale(A::sub_from_scalar(s.v[7], s.ad_value(227)), s.v[144]), s.v[10]);
        }

        if (((!(s.v[858] != 0.0)) && (!(s.v[907] != 0.0))) && (!(s.v[918] != 0.0))) {
            s.store_scale_ad(254, A::div(A::scale(A::sub_from_scalar(s.v[7], s.ad_value(227)), s.v[141]), s.ad_value(229)), s.v[126]);
        }

        s.v[920] = if (((((-s.v[156]) / s.v[254])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((((!(s.v[858] != 0.0)) && (!(s.v[907] != 0.0))) && (!(s.v[918] != 0.0))) && (s.v[920] != 0.0)) {
            s.store_exp_ad(229, A::div(A::neg(s.ad_value(156)), s.ad_value(254)));
        }

        s.v[921] = if (((-s.v[156]) / s.v[254]) < (-230.25850929940458)) { 1.0 } else { 0.0 };

    }

    pub(super) fn stamp_transient_block_25(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if (((((!(s.v[858] != 0.0)) && (!(s.v[907] != 0.0))) && (!(s.v[918] != 0.0))) && (!(s.v[920] != 0.0))) && (s.v[921] != 0.0)) {
            let assign33160_ad_e49964: A = A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(156)), s.ad_value(254))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(156)), s.ad_value(254))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(156)), s.ad_value(254))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
            s.store_div_from_scalar_ad(229, 1e-100, assign33160_ad_e49964);
        }

        if (((((!(s.v[858] != 0.0)) && (!(s.v[907] != 0.0))) && (!(s.v[918] != 0.0))) && (!(s.v[920] != 0.0))) && (!(s.v[921] != 0.0))) {
            let assign33170_ad_e50013: A = A::scale(A::offset(A::mul(A::offset(A::div(A::neg(s.ad_value(156)), s.ad_value(254)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::div(A::neg(s.ad_value(156)), s.ad_value(254)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::div(A::neg(s.ad_value(156)), s.ad_value(254)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
            s.store_ad(229, &assign33170_ad_e50013);
        }

        if (((!(s.v[858] != 0.0)) && (!(s.v[907] != 0.0))) && (!(s.v[918] != 0.0))) {
            s.store_scale_ad(253, A::mul(A::mul(A::mul(s.ad_value(277), s.ad_value(254)), s.ad_value(254)), s.ad_value(229)), s.v[30]);
        }

        s.v[922] = if ((s.v[39] > 1000000.0) || (p.p80 == 0.0)) { 1.0 } else { 0.0 };

        if (((!(s.v[858] != 0.0)) && (!(s.v[907] != 0.0))) && (s.v[922] != 0.0)) {
            s.store_scalar(255, 1.0);
        }

        s.v[923] = if (s.v[228] > ((-s.v[158]) * s.v[39])) { 1.0 } else { 0.0 };

        s.v[924] = if (s.v[42] == 4.0) { 1.0 } else { 0.0 };

        if (((((!(s.v[858] != 0.0)) && (!(s.v[907] != 0.0))) && (!(s.v[922] != 0.0))) && (s.v[923] != 0.0)) && (s.v[924] != 0.0)) {
            s.store_mul_ad(229, A::mul(A::mul(A::abs(A::mul(s.ad_value(228), s.ad_value(163))), A::abs(A::mul(s.ad_value(228), s.ad_value(163)))), A::abs(A::mul(s.ad_value(228), s.ad_value(163)))), A::abs(A::mul(s.ad_value(228), s.ad_value(163))));
        }

        if (((((!(s.v[858] != 0.0)) && (!(s.v[907] != 0.0))) && (!(s.v[922] != 0.0))) && (s.v[923] != 0.0)) && (!(s.v[924] != 0.0))) {
            s.store_powf_ad(229, A::abs(A::mul(s.ad_value(228), s.ad_value(163))), s.v[42]);
        }

        if ((((!(s.v[858] != 0.0)) && (!(s.v[907] != 0.0))) && (!(s.v[922] != 0.0))) && (s.v[923] != 0.0)) {
            s.store_div_from_scalar_ad(255, 1.0, A::sub_from_scalar(1.0, s.ad_value(229)));
        }

        if ((((!(s.v[858] != 0.0)) && (!(s.v[907] != 0.0))) && (!(s.v[922] != 0.0))) && (!(s.v[923] != 0.0))) {
            s.store_offset_ad(255, A::mul(A::add(s.ad_value(228), A::scale(s.ad_value(39), s.v[158])), s.ad_value(166)), s.v[160]);
        }

        if ((!(s.v[858] != 0.0)) && (!(s.v[907] != 0.0))) {
            s.store_mul_ad_lhs(270, A::add(A::add(A::add(s.ad_value(230), s.ad_value(231)), s.ad_value(238)), s.ad_value(253)), 255);
        }

        if ((!(s.v[858] != 0.0)) && (!(s.v[907] != 0.0))) {
            s.store_mul_ad_lhs(292, A::add(A::add(s.ad_value(231), s.ad_value(238)), s.ad_value(253)), 255);
        }

        s.v[925] = if (s.v[258] == 0.0) { 1.0 } else { 0.0 };

        if ((!(s.v[858] != 0.0)) && (s.v[925] != 0.0)) {
            s.store_scalar(272, 0.0);
        }

        if ((!(s.v[858] != 0.0)) && (s.v[925] != 0.0)) {
            s.store_scalar(293, 0.0);
        }

        if ((!(s.v[858] != 0.0)) && (s.v[925] != 0.0)) {
            s.store_scalar(273, 0.0);
        }

        s.v[926] = if (s.v[124] == 0.5) { 1.0 } else { 0.0 };

        if (((!(s.v[858] != 0.0)) && (!(s.v[925] != 0.0))) && (s.v[926] != 0.0)) {
            s.store_sqrt_ad(229, A::sub_from_scalar(1.0, A::mul(s.ad_value(221), s.ad_value(121))));
        }

        if (((!(s.v[858] != 0.0)) && (!(s.v[925] != 0.0))) && (!(s.v[926] != 0.0))) {
            s.store_powf_ad(229, A::sub_from_scalar(1.0, A::mul(s.ad_value(221), s.ad_value(121))), s.v[124]);
        }

        if ((!(s.v[858] != 0.0)) && (!(s.v[925] != 0.0))) {
            s.store_add_ad(273, A::mul(s.ad_value(133), A::sub_from_scalar(1.0, s.ad_value(229))), A::mul(s.ad_value(136), A::sub(s.ad_value(277), s.ad_value(221))));
        }

        if ((!(s.v[858] != 0.0)) && (!(s.v[925] != 0.0))) {
            s.store_mul(230, 103, 372);
        }

        s.v[927] = if ((s.v[22] == 0.0) && (s.v[25] == 0.0)) { 1.0 } else { 0.0 };

        if (((!(s.v[858] != 0.0)) && (!(s.v[925] != 0.0))) && (s.v[927] != 0.0)) {
            s.store_scalar(232, 0.0);
        }

        if (((!(s.v[858] != 0.0)) && (!(s.v[925] != 0.0))) && (s.v[927] != 0.0)) {
            s.store_scalar(235, 0.0);
        }

        if (((!(s.v[858] != 0.0)) && (!(s.v[925] != 0.0))) && (s.v[927] != 0.0)) {
            s.store_scalar(236, 0.0);
        }

        if (((!(s.v[858] != 0.0)) && (!(s.v[925] != 0.0))) && (s.v[927] != 0.0)) {
            s.store_scalar(237, 0.0);
        }

        if (((!(s.v[858] != 0.0)) && (!(s.v[925] != 0.0))) && (s.v[927] != 0.0)) {
            s.store_scalar(231, 0.0);
        }

        if (((!(s.v[858] != 0.0)) && (!(s.v[925] != 0.0))) && (!(s.v[927] != 0.0))) {
            s.store_sub(232, 109, 226);
        }

        if (((!(s.v[858] != 0.0)) && (!(s.v[925] != 0.0))) && (!(s.v[927] != 0.0))) {
            s.store_sub_from_scalar_ad(233, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(224), s.ad_value(232)))));
        }

        s.v[928] = if (s.v[11] == 0.5) { 1.0 } else { 0.0 };

        if ((((!(s.v[858] != 0.0)) && (!(s.v[925] != 0.0))) && (!(s.v[927] != 0.0))) && (s.v[928] != 0.0)) {
            s.store_scalar(234, 0.0);
        }

        if ((((!(s.v[858] != 0.0)) && (!(s.v[925] != 0.0))) && (!(s.v[927] != 0.0))) && (!(s.v[928] != 0.0))) {
            s.store_scale_ad(234, A::add(A::div(A::mul(A::square(s.ad_value(233)), A::ln(s.ad_value(233))), A::sub_from_scalar(1.0, s.ad_value(233))), s.ad_value(233)), (1.0 - (2.0 * s.v[11])));
        }

        if (((!(s.v[858] != 0.0)) && (!(s.v[925] != 0.0))) && (!(s.v[927] != 0.0))) {
            s.store_add(235, 233, 234);
        }

        s.v[929] = if (s.v[11] == 0.5) { 1.0 } else { 0.0 };

        if ((((!(s.v[858] != 0.0)) && (!(s.v[925] != 0.0))) && (!(s.v[927] != 0.0))) && (s.v[929] != 0.0)) {
            s.store_sqrt_ad(229, A::scale(s.ad_value(232), s.v[145]));
        }

        if ((((!(s.v[858] != 0.0)) && (!(s.v[925] != 0.0))) && (!(s.v[927] != 0.0))) && (!(s.v[929] != 0.0))) {
            s.store_powf_ad(229, A::scale(s.ad_value(232), s.v[145]), s.v[11]);
        }

        if (((!(s.v[858] != 0.0)) && (!(s.v[925] != 0.0))) && (!(s.v[927] != 0.0))) {
            s.store_scale(236, 229, s.v[139]);
        }

        if (((!(s.v[858] != 0.0)) && (!(s.v[925] != 0.0))) && (!(s.v[927] != 0.0))) {
            s.store_mul_ad_rhs(237, 100, A::mul(A::offset(s.ad_value(223), (-1.0)), s.ad_value(236)));
        }

        if (((!(s.v[858] != 0.0)) && (!(s.v[925] != 0.0))) && (!(s.v[927] != 0.0))) {
            s.store_scaled_mul(231, 237, 235, s.v[22]);
        }

        s.v[930] = if (s.v[25] == 0.0) { 1.0 } else { 0.0 };

        if (((!(s.v[858] != 0.0)) && (!(s.v[925] != 0.0))) && (s.v[930] != 0.0)) {
            s.store_scalar(238, 0.0);
        }

        if (((!(s.v[858] != 0.0)) && (!(s.v[925] != 0.0))) && (!(s.v[930] != 0.0))) {
            s.store_scale_ad(239, A::div(A::scale(s.ad_value(236), s.v[124]), s.ad_value(232)), s.v[154]);
        }

        if (((!(s.v[858] != 0.0)) && (!(s.v[925] != 0.0))) && (!(s.v[930] != 0.0))) {
            s.store_div_from_scalar(240, (0.666666666666667 * s.v[151]), 239);
        }

        if (((!(s.v[858] != 0.0)) && (!(s.v[925] != 0.0))) && (!(s.v[930] != 0.0))) {
            s.store_square(241, 240);
        }

        if (((!(s.v[858] != 0.0)) && (!(s.v[925] != 0.0))) && (!(s.v[930] != 0.0))) {
            s.store_sqrt_ad(242, A::div(A::square(s.ad_value(241)), A::offset(A::square(s.ad_value(241)), 1.0)));
        }

        if (((!(s.v[858] != 0.0)) && (!(s.v[925] != 0.0))) && (!(s.v[930] != 0.0))) {
            s.store_sqrt_ad(243, A::abs(s.ad_value(242)));
        }

        if (((!(s.v[858] != 0.0)) && (!(s.v[925] != 0.0))) && (!(s.v[930] != 0.0))) {
            s.store_mul(244, 242, 243);
        }

        s.v[931] = if (((-s.v[11]) * s.v[127]) == (-1.0)) { 1.0 } else { 0.0 };

        if ((((!(s.v[858] != 0.0)) && (!(s.v[925] != 0.0))) && (!(s.v[930] != 0.0))) && (s.v[931] != 0.0)) {
            s.store_div_from_scalar_ad(245, 1.0, A::offset(A::mul(s.ad_value(239), s.ad_value(244)), 1.0));
        }

        if ((((!(s.v[858] != 0.0)) && (!(s.v[925] != 0.0))) && (!(s.v[930] != 0.0))) && (!(s.v[931] != 0.0))) {
            s.store_powf_ad(245, A::offset(A::mul(s.ad_value(239), s.ad_value(244)), 1.0), ((-s.v[11]) * s.v[127]));
        }

        if (((!(s.v[858] != 0.0)) && (!(s.v[925] != 0.0))) && (!(s.v[930] != 0.0))) {
            s.store_div_ad(246, A::mul(s.ad_value(235), s.ad_value(245)), A::add(s.ad_value(235), s.ad_value(245)));
        }

        if (((!(s.v[858] != 0.0)) && (!(s.v[925] != 0.0))) && (!(s.v[930] != 0.0))) {
            s.store_sqrt_ad(247, A::scale(A::div(s.ad_value(239), s.ad_value(243)), 0.375));
        }

        if (((!(s.v[858] != 0.0)) && (!(s.v[925] != 0.0))) && (!(s.v[930] != 0.0))) {
            s.store_sub_ad_lhs(248, A::scale(A::mul(s.ad_value(240), s.ad_value(243)), 2.0), 242);
        }

        if (((!(s.v[858] != 0.0)) && (!(s.v[925] != 0.0))) && (!(s.v[930] != 0.0))) {
            s.store_add_ad(249, A::sub(A::mul(A::scale(s.ad_value(240), s.v[151]), s.ad_value(243)), A::scale(s.ad_value(242), s.v[151])), A::scale(A::mul(s.ad_value(239), s.ad_value(244)), 0.5));
        }

        if (((!(s.v[858] != 0.0)) && (!(s.v[925] != 0.0))) && (!(s.v[930] != 0.0))) {
            s.store_mul_ad_lhs(250, A::offset(s.ad_value(248), (-1.0)), 247);
        }

        if (((!(s.v[858] != 0.0)) && (!(s.v[925] != 0.0))) && (!(s.v[930] != 0.0))) {
            s.store_square(212, 250);
        }

        s.v[932] = if (s.v[250] > 0.0) { 1.0 } else { 0.0 };

        if ((((!(s.v[858] != 0.0)) && (!(s.v[925] != 0.0))) && (!(s.v[930] != 0.0))) && (s.v[932] != 0.0)) {
            s.store_div_from_scalar_ad(213, 1.0, A::offset(A::scale(s.ad_value(250), s.v[86]), 1.0));
        }

        if ((((!(s.v[858] != 0.0)) && (!(s.v[925] != 0.0))) && (!(s.v[930] != 0.0))) && (!(s.v[932] != 0.0))) {
            s.store_div_from_scalar_ad(213, 1.0, A::sub_from_scalar(1.0, A::scale(s.ad_value(250), s.v[86])));
        }

        s.v[933] = if (((-s.v[212]) + s.v[249]) > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((!(s.v[858] != 0.0)) && (!(s.v[925] != 0.0))) && (!(s.v[930] != 0.0))) && (s.v[933] != 0.0)) {
            s.store_exp_ad(229, A::sub(s.ad_value(249), s.ad_value(212)));
        }

        if ((((!(s.v[858] != 0.0)) && (!(s.v[925] != 0.0))) && (!(s.v[930] != 0.0))) && (!(s.v[933] != 0.0))) {
            s.store_div_from_scalar_ad(229, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(249), s.ad_value(212))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(249), s.ad_value(212))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(249), s.ad_value(212))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if (((!(s.v[858] != 0.0)) && (!(s.v[925] != 0.0))) && (!(s.v[930] != 0.0))) {
            s.store_mul_ad_lhs(214, A::add(A::add(A::scale(s.ad_value(213), 0.29214664), A::scale(A::square(s.ad_value(213)), s.v[87])), A::scale(A::mul(A::square(s.ad_value(213)), s.ad_value(213)), s.v[88])), 229);
        }

        s.v[934] = if (s.v[250] > 0.0) { 1.0 } else { 0.0 };

        if ((((!(s.v[858] != 0.0)) && (!(s.v[925] != 0.0))) && (!(s.v[930] != 0.0))) && (s.v[934] != 0.0)) {
            s.copy_ad(251, 214);
        }

        s.v[935] = if (s.v[249] > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((!(s.v[858] != 0.0)) && (!(s.v[925] != 0.0))) && (!(s.v[930] != 0.0))) && (!(s.v[934] != 0.0))) && (s.v[935] != 0.0)) {
            s.store_exp(229, 249);
        }

        if (((((!(s.v[858] != 0.0)) && (!(s.v[925] != 0.0))) && (!(s.v[930] != 0.0))) && (!(s.v[934] != 0.0))) && (!(s.v[935] != 0.0))) {
            s.store_div_from_scalar_ad(229, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(249)), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(249)), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), s.ad_value(249)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if ((((!(s.v[858] != 0.0)) && (!(s.v[925] != 0.0))) && (!(s.v[930] != 0.0))) && (!(s.v[934] != 0.0))) {
            s.store_sub_ad_lhs(251, A::scale(s.ad_value(229), 2.0), 214);
        }

        if (((!(s.v[858] != 0.0)) && (!(s.v[925] != 0.0))) && (!(s.v[930] != 0.0))) {
            s.store_scale_ad(252, A::div(A::scale(s.ad_value(251), s.v[151]), s.ad_value(247)), (1.772453850905516 * 0.5));
        }

        if (((!(s.v[858] != 0.0)) && (!(s.v[925] != 0.0))) && (!(s.v[930] != 0.0))) {
            s.store_scale_ad(238, A::mul(A::mul(s.ad_value(237), s.ad_value(252)), s.ad_value(246)), s.v[25]);
        }

        s.v[936] = if (s.v[31] == 0.0) { 1.0 } else { 0.0 };

        if (((!(s.v[858] != 0.0)) && (!(s.v[925] != 0.0))) && (s.v[936] != 0.0)) {
            s.store_scalar(253, 0.0);
        }

        s.v[937] = if (s.v[11] == 0.5) { 1.0 } else { 0.0 };

        if ((((!(s.v[858] != 0.0)) && (!(s.v[925] != 0.0))) && (!(s.v[936] != 0.0))) && (s.v[937] != 0.0)) {
            s.store_sqrt_ad(229, A::scale(A::sub_from_scalar(s.v[8], s.ad_value(227)), s.v[145]));
        }

        if ((((!(s.v[858] != 0.0)) && (!(s.v[925] != 0.0))) && (!(s.v[936] != 0.0))) && (!(s.v[937] != 0.0))) {
            s.store_powf_ad(229, A::scale(A::sub_from_scalar(s.v[8], s.ad_value(227)), s.v[145]), s.v[11]);
        }

        if (((!(s.v[858] != 0.0)) && (!(s.v[925] != 0.0))) && (!(s.v[936] != 0.0))) {
            s.store_scale_ad(254, A::div(A::scale(A::sub_from_scalar(s.v[8], s.ad_value(227)), s.v[142]), s.ad_value(229)), s.v[127]);
        }

        s.v[938] = if (((((-s.v[157]) / s.v[254])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((((!(s.v[858] != 0.0)) && (!(s.v[925] != 0.0))) && (!(s.v[936] != 0.0))) && (s.v[938] != 0.0)) {
            s.store_exp_ad(229, A::div(A::neg(s.ad_value(157)), s.ad_value(254)));
        }

        s.v[939] = if (((-s.v[157]) / s.v[254]) < (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((!(s.v[858] != 0.0)) && (!(s.v[925] != 0.0))) && (!(s.v[936] != 0.0))) && (!(s.v[938] != 0.0))) && (s.v[939] != 0.0)) {
            let assign33970_ad_e51183: A = A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(157)), s.ad_value(254))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(157)), s.ad_value(254))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(157)), s.ad_value(254))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
            s.store_div_from_scalar_ad(229, 1e-100, assign33970_ad_e51183);
        }

        if (((((!(s.v[858] != 0.0)) && (!(s.v[925] != 0.0))) && (!(s.v[936] != 0.0))) && (!(s.v[938] != 0.0))) && (!(s.v[939] != 0.0))) {
            let assign33980_ad_e51232: A = A::scale(A::offset(A::mul(A::offset(A::div(A::neg(s.ad_value(157)), s.ad_value(254)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::div(A::neg(s.ad_value(157)), s.ad_value(254)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::div(A::neg(s.ad_value(157)), s.ad_value(254)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
            s.store_ad(229, &assign33980_ad_e51232);
        }

        if (((!(s.v[858] != 0.0)) && (!(s.v[925] != 0.0))) && (!(s.v[936] != 0.0))) {
            s.store_scale_ad(253, A::mul(A::mul(A::mul(s.ad_value(277), s.ad_value(254)), s.ad_value(254)), s.ad_value(229)), s.v[31]);
        }

        s.v[940] = if ((s.v[40] > 1000000.0) || (p.p80 == 0.0)) { 1.0 } else { 0.0 };

        if (((!(s.v[858] != 0.0)) && (!(s.v[925] != 0.0))) && (s.v[940] != 0.0)) {
            s.store_scalar(255, 1.0);
        }

        s.v[941] = if (s.v[228] > ((-s.v[158]) * s.v[40])) { 1.0 } else { 0.0 };

        s.v[942] = if (s.v[43] == 4.0) { 1.0 } else { 0.0 };

        if (((((!(s.v[858] != 0.0)) && (!(s.v[925] != 0.0))) && (!(s.v[940] != 0.0))) && (s.v[941] != 0.0)) && (s.v[942] != 0.0)) {
            s.store_mul_ad(229, A::mul(A::mul(A::abs(A::mul(s.ad_value(228), s.ad_value(164))), A::abs(A::mul(s.ad_value(228), s.ad_value(164)))), A::abs(A::mul(s.ad_value(228), s.ad_value(164)))), A::abs(A::mul(s.ad_value(228), s.ad_value(164))));
        }

        if (((((!(s.v[858] != 0.0)) && (!(s.v[925] != 0.0))) && (!(s.v[940] != 0.0))) && (s.v[941] != 0.0)) && (!(s.v[942] != 0.0))) {
            s.store_powf_ad(229, A::abs(A::mul(s.ad_value(228), s.ad_value(164))), s.v[43]);
        }

        if ((((!(s.v[858] != 0.0)) && (!(s.v[925] != 0.0))) && (!(s.v[940] != 0.0))) && (s.v[941] != 0.0)) {
            s.store_div_from_scalar_ad(255, 1.0, A::sub_from_scalar(1.0, s.ad_value(229)));
        }

        if ((((!(s.v[858] != 0.0)) && (!(s.v[925] != 0.0))) && (!(s.v[940] != 0.0))) && (!(s.v[941] != 0.0))) {
            s.store_offset_ad(255, A::mul(A::add(s.ad_value(228), A::scale(s.ad_value(40), s.v[158])), s.ad_value(167)), s.v[161]);
        }

        if ((!(s.v[858] != 0.0)) && (!(s.v[925] != 0.0))) {
            s.store_mul_ad_lhs(272, A::add(A::add(A::add(s.ad_value(230), s.ad_value(231)), s.ad_value(238)), s.ad_value(253)), 255);
        }

        if ((!(s.v[858] != 0.0)) && (!(s.v[925] != 0.0))) {
            s.store_mul_ad_lhs(293, A::add(A::add(s.ad_value(231), s.ad_value(238)), s.ad_value(253)), 255);
        }

        if (!(s.v[858] != 0.0)) {
            s.store_add_ad(274, A::add(A::scale(s.ad_value(268), s.v[256]), A::scale(s.ad_value(270), s.v[257])), A::scale(s.ad_value(272), s.v[258]));
        }

        if (!(s.v[858] != 0.0)) {
            s.store_add_ad(290, A::add(A::scale(s.ad_value(291), s.v[256]), A::scale(s.ad_value(292), s.v[257])), A::scale(s.ad_value(293), s.v[258]));
        }

        s.store_add_ad(275, A::add(A::scale(s.ad_value(269), s.v[256]), A::scale(s.ad_value(271), s.v[257])), A::scale(s.ad_value(273), s.v[258]));

        s.store_ad(284, &A::voltage(ctx, &nodes, Some(2), Some(1)));

        s.v[945] = if (p.p84 > 0.0) { 1.0 } else { 0.0 };

        s.v[946] = if (s.v[313] < p.p85) { 1.0 } else { 0.0 };

        if ((s.v[945] != 0.0) && (s.v[946] != 0.0)) {
            s.store_offset_ad(349, A::scale(A::sub(s.ad_value(277), s.ad_value(348)), p.p86), s.v[313]);
        }

        if ((s.v[945] != 0.0) && (s.v[946] != 0.0)) {
            s.store_sub_from_scalar_ad(350, s.v[313], A::scale(s.ad_value(348), p.p86));
        }

        if ((s.v[945] != 0.0) && (s.v[946] != 0.0)) {
            s.store_offset_ad(314, A::sub_from_scalar(p.p85, s.ad_value(349)), (-0.01));
        }

        if ((s.v[945] != 0.0) && (s.v[946] != 0.0)) {
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if ((s.v[945] != 0.0) && (s.v[946] != 0.0)) {
            s.store_ad(315, &{
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((s.v[945] != 0.0) && (s.v[946] != 0.0)) {
            s.store_sqrt_ad(315, A::add(A::square(s.ad_value(314)), s.ad_value(315)));
        }

        if ((s.v[945] != 0.0) && (s.v[946] != 0.0)) {
            s.store_sub_from_scalar_ad(351, p.p85, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5));
        }

        if ((s.v[945] != 0.0) && (s.v[946] != 0.0)) {
            s.store_offset(314, 351, (((-s.v[313])) + ((-0.01))));
        }

        if ((s.v[945] != 0.0) && (s.v[946] != 0.0)) {
            s.store_scalar(315, ((4.0 * s.v[313]) * 0.01));
        }

        if ((s.v[945] != 0.0) && (s.v[946] != 0.0)) {
            s.store_ad(315, &{
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((s.v[945] != 0.0) && (s.v[946] != 0.0)) {
            s.store_sqrt_ad(315, A::add(A::square(s.ad_value(314)), s.ad_value(315)));
        }

        if ((s.v[945] != 0.0) && (s.v[946] != 0.0)) {
            s.store_offset_ad(352, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5), s.v[313]);
        }

        if ((s.v[945] != 0.0) && (s.v[946] != 0.0)) {
            s.store_offset_ad(314, A::sub_from_scalar(p.p85, s.ad_value(350)), (-0.01));
        }

        if ((s.v[945] != 0.0) && (s.v[946] != 0.0)) {
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if ((s.v[945] != 0.0) && (s.v[946] != 0.0)) {
            s.store_ad(315, &{
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((s.v[945] != 0.0) && (s.v[946] != 0.0)) {
            s.store_sqrt_ad(315, A::add(A::square(s.ad_value(314)), s.ad_value(315)));
        }

        if ((s.v[945] != 0.0) && (s.v[946] != 0.0)) {
            s.store_sub_from_scalar_ad(350, p.p85, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5));
        }

        if ((s.v[945] != 0.0) && (s.v[946] != 0.0)) {
            s.store_offset(314, 350, (((-s.v[313])) + ((-0.01))));
        }

        if ((s.v[945] != 0.0) && (s.v[946] != 0.0)) {
            s.store_scalar(315, ((4.0 * s.v[313]) * 0.01));
        }

        if ((s.v[945] != 0.0) && (s.v[946] != 0.0)) {
            s.store_ad(315, &{
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((s.v[945] != 0.0) && (s.v[946] != 0.0)) {
            s.store_sqrt_ad(315, A::add(A::square(s.ad_value(314)), s.ad_value(315)));
        }

        if ((s.v[945] != 0.0) && (s.v[946] != 0.0)) {
            s.store_offset_ad(350, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5), s.v[313]);
        }

        if ((s.v[945] != 0.0) && (!(s.v[946] != 0.0))) {
            s.store_scalar(352, s.v[313]);
        }

        if ((s.v[945] != 0.0) && (!(s.v[946] != 0.0))) {
            s.store_scalar(350, s.v[313]);
        }

        if (s.v[945] != 0.0) {
            s.copy_ad(353, 370);
        }

        s.v[947] = if ((s.v[277] - (s.v[348] - s.v[347])) > 0.0) { 1.0 } else { 0.0 };

        s.v[948] = if ((((s.v[85] * (((s.v[277] / s.v[352]) - ((s.v[348] - s.v[347]) / s.v[352])) + ((s.v[348] * (s.v[352] - s.v[350])) / (s.v[350] * p.p85))))) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((s.v[945] != 0.0) && (s.v[947] != 0.0)) && (s.v[948] != 0.0)) {
            s.store_exp_ad(354, A::scale(A::add(A::sub(A::div(s.ad_value(277), s.ad_value(352)), A::div(A::sub(s.ad_value(348), s.ad_value(347)), s.ad_value(352))), A::div(A::mul(s.ad_value(348), A::sub(s.ad_value(352), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]));
        }

        s.v[949] = if ((s.v[85] * (((s.v[277] / s.v[352]) - ((s.v[348] - s.v[347]) / s.v[352])) + ((s.v[348] * (s.v[352] - s.v[350])) / (s.v[350] * p.p85)))) < (-230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((s.v[945] != 0.0) && (s.v[947] != 0.0)) && (!(s.v[948] != 0.0))) && (s.v[949] != 0.0)) {
            let assign34480_ad_e51901: A = A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(A::add(A::sub(A::div(s.ad_value(277), s.ad_value(352)), A::div(A::sub(s.ad_value(348), s.ad_value(347)), s.ad_value(352))), A::div(A::mul(s.ad_value(348), A::sub(s.ad_value(352), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85])), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::scale(A::add(A::sub(A::div(s.ad_value(277), s.ad_value(352)), A::div(A::sub(s.ad_value(348), s.ad_value(347)), s.ad_value(352))), A::div(A::mul(s.ad_value(348), A::sub(s.ad_value(352), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85])), 0.3333333333333333), 1.0));
            let assign34480_ad_e51905: A = A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(A::add(A::sub(A::div(s.ad_value(277), s.ad_value(352)), A::div(A::sub(s.ad_value(348), s.ad_value(347)), s.ad_value(352))), A::div(A::mul(s.ad_value(348), A::sub(s.ad_value(352), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85])), A::offset(A::scale(assign34480_ad_e51901, 0.5), 1.0)), 1.0);
            s.store_div_from_scalar_ad(354, 1e-100, assign34480_ad_e51905);
        }

        if ((((s.v[945] != 0.0) && (s.v[947] != 0.0)) && (!(s.v[948] != 0.0))) && (!(s.v[949] != 0.0))) {
            let assign34490_ad_e51995: A = A::mul(A::offset(A::scale(A::add(A::sub(A::div(s.ad_value(277), s.ad_value(352)), A::div(A::sub(s.ad_value(348), s.ad_value(347)), s.ad_value(352))), A::div(A::mul(s.ad_value(348), A::sub(s.ad_value(352), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]), (-230.25850929940458)), A::offset(A::scale(A::offset(A::scale(A::add(A::sub(A::div(s.ad_value(277), s.ad_value(352)), A::div(A::sub(s.ad_value(348), s.ad_value(347)), s.ad_value(352))), A::div(A::mul(s.ad_value(348), A::sub(s.ad_value(352), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]), (-230.25850929940458)), 0.3333333333333333), 1.0));
            let assign34490_ad_e51999: A = A::offset(A::mul(A::offset(A::scale(A::add(A::sub(A::div(s.ad_value(277), s.ad_value(352)), A::div(A::sub(s.ad_value(348), s.ad_value(347)), s.ad_value(352))), A::div(A::mul(s.ad_value(348), A::sub(s.ad_value(352), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]), (-230.25850929940458)), A::offset(A::scale(assign34490_ad_e51995, 0.5), 1.0)), 1.0);
            s.store_scale_ad(354, assign34490_ad_e51999, 1e100);
        }

        if ((s.v[945] != 0.0) && (!(s.v[947] != 0.0))) {
            s.store_scalar(354, 1.0);
        }

        s.v[950] = if ((p.p91 == 0.0) || (s.v[277] < s.v[347])) { 1.0 } else { 0.0 };

        if ((s.v[945] != 0.0) && (s.v[950] != 0.0)) {
            s.store_scale(357, 353, p.p90);
        }

    }

    pub(super) fn stamp_transient_block_26(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if ((s.v[945] != 0.0) && (!(s.v[950] != 0.0))) {
            s.store_mul_ad(357, A::scale(s.ad_value(353), p.p90), A::exp(A::mul(A::mul(A::scale(A::sub(s.ad_value(277), s.ad_value(347)), (-p.p91)), A::sub(s.ad_value(277), s.ad_value(347))), A::exp(A::scale(A::ln(A::scale(s.ad_value(78), 1.0 / (s.v[79]))), p.p98)))));
        }

        if (s.v[945] != 0.0) {
            s.store_ad(357, &{
                if (s.v[357] > p.p79) {
                    A::constant(p.p79)
                } else {
                    s.ad_value(357)
                }
            });
        }

        if (s.v[945] != 0.0) {
            s.store_mul(355, 319, 357);
        }

        if (s.v[945] != 0.0) {
            s.store_scaled_sub(331, 355, 319, (1.6021918e-19 * s.v[256]));
        }

        s.v[951] = if (p.p92 > 0.0) { 1.0 } else { 0.0 };

        if ((s.v[945] != 0.0) && (s.v[951] != 0.0)) {
            s.store_scale(334, 331, (1e-23 / s.v[333]));
        }

        if ((s.v[945] != 0.0) && (s.v[951] != 0.0)) {
            s.store_ad(336, &A::voltage(ctx, &nodes, Some(3), None));
        }

        if ((s.v[945] != 0.0) && (s.v[951] != 0.0)) {
            s.store_scaled_sub(338, 336, 334, 1.0 / (p.p92));
        }

        if ((s.v[945] != 0.0) && (s.v[951] != 0.0)) {
            s.store_scale(340, 336, 1.0 / ((1e-23 / s.v[333])));
        }

        if ((s.v[945] != 0.0) && (!(s.v[951] != 0.0))) {
            s.copy_ad(334, 331);
        }

        if ((s.v[945] != 0.0) && (!(s.v[951] != 0.0))) {
            s.copy_ad(340, 334);
        }

        s.v[952] = if ((p.p91 == 0.0) || (s.v[277] < s.v[348])) { 1.0 } else { 0.0 };

        if ((s.v[945] != 0.0) && (s.v[952] != 0.0)) {
            s.store_scale(358, 354, p.p90);
        }

        if ((s.v[945] != 0.0) && (!(s.v[952] != 0.0))) {
            s.store_mul_ad(358, A::scale(s.ad_value(354), p.p90), A::exp(A::mul(A::mul(A::scale(A::sub(s.ad_value(277), s.ad_value(348)), (-p.p91)), A::sub(s.ad_value(277), s.ad_value(348))), A::exp(A::scale(A::ln(A::scale(s.ad_value(78), 1.0 / (s.v[79]))), p.p98)))));
        }

        if (s.v[945] != 0.0) {
            s.store_ad(358, &{
                if (s.v[358] > p.p79) {
                    A::constant(p.p79)
                } else {
                    s.ad_value(358)
                }
            });
        }

        if (s.v[945] != 0.0) {
            s.store_mul(356, 319, 358);
        }

        if (s.v[945] != 0.0) {
            s.store_scaled_sub(332, 356, 319, (1.6021918e-19 * s.v[256]));
        }

        s.v[953] = if (p.p92 > 0.0) { 1.0 } else { 0.0 };

        if ((s.v[945] != 0.0) && (s.v[953] != 0.0)) {
            s.store_scale(335, 332, (1e-23 / s.v[333]));
        }

        if ((s.v[945] != 0.0) && (s.v[953] != 0.0)) {
            s.store_ad(337, &A::voltage(ctx, &nodes, Some(4), None));
        }

        if ((s.v[945] != 0.0) && (s.v[953] != 0.0)) {
            s.store_scaled_sub(339, 337, 335, 1.0 / (p.p92));
        }

        if ((s.v[945] != 0.0) && (s.v[953] != 0.0)) {
            s.store_scale(341, 337, 1.0 / ((1e-23 / s.v[333])));
        }

        if ((s.v[945] != 0.0) && (!(s.v[953] != 0.0))) {
            s.copy_ad(335, 332);
        }

        if ((s.v[945] != 0.0) && (!(s.v[953] != 0.0))) {
            s.copy_ad(341, 335);
        }

        if (s.v[945] != 0.0) {
            s.store_sub_from_scalar(325, s.v[368], 277);
        }

        if (s.v[945] != 0.0) {
            s.store_sqrt_ad(315, A::offset(A::square(s.ad_value(325)), ((4.0 * s.v[369]) * s.v[369])));
        }

        if (s.v[945] != 0.0) {
            s.store_scaled_add(325, 325, 315, 0.5);
        }

        s.v[954] = if (s.v[325] < 0.0) { 1.0 } else { 0.0 };

        if ((s.v[945] != 0.0) && (s.v[954] != 0.0)) {
            s.store_scalar(325, 0.0);
        }

        if (s.v[945] != 0.0) {
            s.store_sqrt_ad(326, A::scale(s.ad_value(325), ((2.0 * s.v[0]) * 1.0 / ((1.6021918e-19 * s.v[307])))));
        }

        if (s.v[945] != 0.0) {
            s.store_offset_ad(314, A::sub_from_scalar(p.p94, s.ad_value(326)), (-1e-7));
        }

        if (s.v[945] != 0.0) {
            s.store_scalar(315, ((4.0 * p.p94) * 1e-7));
        }

        if (s.v[945] != 0.0) {
            s.store_ad(315, &{
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if (s.v[945] != 0.0) {
            s.store_sqrt_ad(315, A::add(A::square(s.ad_value(314)), s.ad_value(315)));
        }

        if (s.v[945] != 0.0) {
            s.store_sub_from_scalar_ad(326, p.p94, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5));
        }

        s.v[955] = if (p.p95 > 0.0) { 1.0 } else { 0.0 };

        if ((s.v[945] != 0.0) && (s.v[955] != 0.0)) {
            s.store_mul_ad_rhs(342, 326, A::div_from_scalar(1.0, s.ad_value(343)));
        }

        if ((s.v[945] != 0.0) && (s.v[955] != 0.0)) {
            s.store_ad(344, &A::voltage(ctx, &nodes, Some(5), None));
        }

        if ((s.v[945] != 0.0) && (s.v[955] != 0.0)) {
            s.store_scaled_sub(345, 344, 342, 1.0 / (p.p95));
        }

        if ((s.v[945] != 0.0) && (s.v[955] != 0.0)) {
            s.store_div_ad_rhs(346, 344, A::div_from_scalar(1.0, s.ad_value(343)));
        }

        if ((s.v[945] != 0.0) && (!(s.v[955] != 0.0))) {
            s.copy_ad(342, 326);
        }

        if ((s.v[945] != 0.0) && (!(s.v[955] != 0.0))) {
            s.copy_ad(346, 342);
        }

        if (s.v[945] != 0.0) {
            s.store_scalar(327, ((-((s.v[307] * s.v[256]) * 1.6021918e-19)) * p.p94));
        }

        if (s.v[945] != 0.0) {
            s.store_mul_ad(328, A::mul(s.ad_value(323), s.ad_value(340)), A::sub(A::exp(A::div_from_scalar((-p.p94), s.ad_value(323))), A::exp(A::div(A::neg(s.ad_value(346)), s.ad_value(323)))));
        }

        if (s.v[945] != 0.0) {
            s.store_mul_ad(329, A::mul(s.ad_value(323), s.ad_value(341)), A::offset(A::exp(A::div(A::neg(A::sub_from_scalar(p.p94, s.ad_value(346))), s.ad_value(323))), (-1.0)));
        }

        if (s.v[945] != 0.0) {
            s.store_neg_ad(330, A::add(A::add(s.ad_value(327), s.ad_value(328)), s.ad_value(329)));
        }

        if (s.v[945] != 0.0) {
            s.store_add(275, 275, 330);
        }

        if (s.v[945] != 0.0) {
            s.store_scalar(55, 0.0);
        }

        if (!(s.v[945] != 0.0)) {
            s.store_mul_ad_rhs(330, 55, A::sub(s.ad_value(274), s.ad_value(290)));
        }

        s.store_scale_ad(276, A::add(A::add(A::sub(s.ad_value(274), s.ad_value(290)), A::scale(s.ad_value(289), 2.0)), A::abs(s.ad_value(290))), (2.0 * 1.6021918e-19));

        s.store_scale_ad(286, A::powf(A::abs(s.ad_value(274)), s.v[54]), s.v[53]);

        s.v[956] = if ((s.v[171] > 0.0) && (s.v[171] >= p.p4)) { 1.0 } else { 0.0 };

        if (s.v[956] != 0.0) {
            s.store_div_from_scalar(287, ((4.0 * 1.3806505e-23) * s.v[79]), 171);
        }

        if (!(s.v[956] != 0.0)) {
            s.store_scalar(287, 0.0);
        }

        s.v[957] = if ((s.v[171] > 0.0) && (s.v[171] >= p.p4)) { 1.0 } else { 0.0 };

        s.v[958] = if ((p.p84 > 0.0) && (p.p92 > 0.0)) { 1.0 } else { 0.0 };

        s.v[959] = if ((p.p84 > 0.0) && (p.p95 > 0.0)) { 1.0 } else { 0.0 };

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
        s.v[0] = (8.8541878176e-12 * 11.8);

        s.v[1] = (if (p.p6 > (-250.0)) { p.p6 } else { (-250.0) });

        s.v[388] = if ((!(if self.param_given[6] { 1.0 } else { 0.0 } != 0.0)) && (if self.param_given[96] { 1.0 } else { 0.0 } != 0.0)) { 1.0 } else { 0.0 };

        if (s.v[388] != 0.0) {
            s.store_scalar(1, (if (p.p96 > (-250.0)) { p.p96 } else { (-250.0) }));
        }

        s.v[2] = (if (p.p5 > 1e-12) { p.p5 } else { 1e-12 });

        s.v[3] = (if (p.p8 > 1e-12) { p.p8 } else { 1e-12 });

        s.v[4] = (if (p.p9 > 1e-18) { p.p9 } else { 1e-18 });

        s.v[5] = (if (p.p10 > 1e-18) { p.p10 } else { 1e-18 });

        s.v[6] = (if (p.p11 > 0.05) { p.p11 } else { 0.05 });

        s.v[7] = (if (p.p12 > 0.05) { p.p12 } else { 0.05 });

        s.v[8] = (if (p.p13 > 0.05) { p.p13 } else { 0.05 });

        s.v[9] = (if (p.p14 > 0.05) { (if (p.p14 < 0.95) { p.p14 } else { 0.95 }) } else { 0.05 });

        s.v[10] = (if (p.p15 > 0.05) { (if (p.p15 < 0.95) { p.p15 } else { 0.95 }) } else { 0.05 });

        s.v[11] = (if (p.p16 > 0.05) { (if (p.p16 < 0.95) { p.p16 } else { 0.95 }) } else { 0.05 });

        s.v[12] = p.p17;

        s.v[13] = p.p18;

        s.v[14] = p.p19;

        s.v[15] = (if (p.p20 > 0.0) { p.p20 } else { 0.0 });

        s.v[16] = (if (p.p21 > 0.0) { p.p21 } else { 0.0 });

        s.v[17] = (if (p.p22 > 0.0) { p.p22 } else { 0.0 });

        s.v[20] = (if (p.p23 > 0.0) { p.p23 } else { 0.0 });

        s.v[21] = (if (p.p24 > 0.0) { p.p24 } else { 0.0 });

        s.v[22] = (if (p.p25 > 0.0) { p.p25 } else { 0.0 });

        s.v[18] = (if (p.p26 > 1e-9) { p.p26 } else { 1e-9 });

        s.v[19] = (if (p.p27 > 1e-9) { p.p27 } else { 1e-9 });

        s.v[23] = (if (p.p28 > 0.0) { p.p28 } else { 0.0 });

        s.v[24] = (if (p.p29 > 0.0) { p.p29 } else { 0.0 });

        s.v[25] = (if (p.p30 > 0.0) { p.p30 } else { 0.0 });

        s.v[26] = (if (p.p31 > 0.01) { p.p31 } else { 0.01 });

        s.v[27] = (if (p.p32 > 0.01) { p.p32 } else { 0.01 });

        s.v[28] = (if (p.p33 > 0.01) { p.p33 } else { 0.01 });

        s.v[29] = (if (p.p34 > 0.0) { p.p34 } else { 0.0 });

        s.v[30] = (if (p.p35 > 0.0) { p.p35 } else { 0.0 });

        s.v[31] = (if (p.p36 > 0.0) { p.p36 } else { 0.0 });

        s.v[32] = p.p37;

        s.v[33] = p.p38;

        s.v[34] = p.p39;

        s.v[35] = p.p40;

        s.v[36] = p.p41;

        s.v[37] = p.p42;

        s.v[38] = (if (p.p43 > 0.1) { p.p43 } else { 0.1 });

        s.v[39] = (if (p.p44 > 0.1) { p.p44 } else { 0.1 });

        s.v[40] = (if (p.p45 > 0.1) { p.p45 } else { 0.1 });

        s.v[41] = (if (p.p46 > 0.1) { p.p46 } else { 0.1 });

        s.v[42] = (if (p.p47 > 0.1) { p.p47 } else { 0.1 });

        s.v[43] = (if (p.p48 > 0.1) { p.p48 } else { 0.1 });

        s.v[44] = p.p7;

        s.v[55] = (if (p.p56 > 0.0) { p.p56 } else { 0.0 });

        s.v[56] = p.p57;

        s.v[57] = p.p58;

        s.v[58] = p.p59;

        s.v[59] = p.p60;

        s.v[60] = p.p61;

        s.v[61] = p.p62;

        s.v[62] = (if (p.p63 > 0.1) { p.p63 } else { 0.1 });

        s.v[64] = (if (p.p64 > 0.1) { p.p64 } else { 0.1 });

        s.v[63] = (if (p.p65 > 0.1) { p.p65 } else { 0.1 });

        s.v[75] = (if (p.p76 > 0.1) { p.p76 } else { 0.1 });

        s.v[76] = (if (p.p77 > 0.0) { p.p77 } else { 0.0 });

        s.v[77] = (if (p.p78 > 0.0) { p.p78 } else { 0.0 });

        s.v[45] = 0.0;

        s.v[389] = if (p.p81 > 0.5) { 1.0 } else { 0.0 };

        if (s.v[389] != 0.0) {
            s.store_scalar(45, 1.0);
        }

        if (!(s.v[389] != 0.0)) {
            s.store_scalar(45, 0.0);
        }

        s.v[46] = (if (p.p82 > 0.5) { p.p82 } else { 0.5 });

        s.v[47] = (if (p.p83 > 0.0) { p.p83 } else { 0.0 });

        s.store_offset(78, 1, 273.15);

        s.v[79] = ((ctx.temperature() + p.p102)).max((273.15 + (-250.0)));

        s.store_div_from_scalar(80, s.v[79], 78);

        s.v[81] = (1.3806505e-23 / 1.6021918e-19);

        s.store_scale(82, 78, s.v[81]);

        s.store_div_from_scalar(83, 1.0, 82);

        s.v[84] = (s.v[81] * s.v[79]);

        s.v[85] = (1.0 / s.v[84]);

        s.store_div_ad(89, A::neg(A::mul(A::scale(s.ad_value(78), 0.000702), s.ad_value(78))), A::offset(s.ad_value(78), 1108.0));

        s.store_offset(92, 89, s.v[12]);

        s.store_offset(93, 89, s.v[13]);

        s.store_offset(94, 89, s.v[14]);

        s.v[90] = ((-((0.000702 * s.v[79]) * s.v[79])) / (1108.0 + s.v[79]));

        s.v[95] = (s.v[12] + s.v[90]);

        s.v[96] = (s.v[13] + s.v[90]);

        s.v[97] = (s.v[14] + s.v[90]);

        s.store_mul_ad(98, A::powf(s.ad_value(80), (s.v[75] / 2.0)), A::exp(A::scale(A::offset(A::mul(s.ad_value(92), s.ad_value(83)), (-(s.v[95] * s.v[85]))), 0.5)));

        s.store_mul_ad(99, A::powf(s.ad_value(80), (s.v[75] / 2.0)), A::exp(A::scale(A::offset(A::mul(s.ad_value(93), s.ad_value(83)), (-(s.v[96] * s.v[85]))), 0.5)));

        s.store_mul_ad(100, A::powf(s.ad_value(80), (s.v[75] / 2.0)), A::exp(A::scale(A::offset(A::mul(s.ad_value(94), s.ad_value(83)), (-(s.v[97] * s.v[85]))), 0.5)));

        s.store_mul_ad(176, A::powf(s.ad_value(80), ((s.v[75] / 2.0) / s.v[62])), A::exp(A::scale(A::offset(A::mul(s.ad_value(92), s.ad_value(83)), (-(s.v[95] * s.v[85]))), (0.5 * 1.0 / (s.v[62])))));

        s.store_mul_ad(177, A::powf(s.ad_value(80), ((s.v[75] / 2.0) / s.v[64])), A::exp(A::scale(A::offset(A::mul(s.ad_value(93), s.ad_value(83)), (-(s.v[96] * s.v[85]))), (0.5 * 1.0 / (s.v[64])))));

        s.store_mul_ad(178, A::powf(s.ad_value(80), ((s.v[75] / 2.0) / s.v[63])), A::exp(A::scale(A::offset(A::mul(s.ad_value(94), s.ad_value(83)), (-(s.v[97] * s.v[85]))), (0.5 * 1.0 / (s.v[63])))));

        s.store_mul_ad_lhs(101, A::scale(s.ad_value(176), s.v[15]), 176);

        s.store_mul_ad_lhs(102, A::scale(s.ad_value(177), s.v[16]), 177);

        s.store_mul_ad_lhs(103, A::scale(s.ad_value(178), s.v[17]), 178);

        s.store_sub_ad(104, A::scale(s.ad_value(80), s.v[6]), A::scale(A::ln(s.ad_value(98)), (2.0 * s.v[84])));

        s.store_sub_ad(105, A::scale(s.ad_value(80), s.v[7]), A::scale(A::ln(s.ad_value(99)), (2.0 * s.v[84])));

        s.store_sub_ad(106, A::scale(s.ad_value(80), s.v[8]), A::scale(A::ln(s.ad_value(100)), (2.0 * s.v[84])));

        s.store_add_ad_rhs(107, 104, A::scale(A::ln(A::offset(A::exp(A::scale(A::sub_from_scalar(0.05, s.ad_value(104)), s.v[85])), 1.0)), s.v[84]));

        s.store_add_ad_rhs(108, 105, A::scale(A::ln(A::offset(A::exp(A::scale(A::sub_from_scalar(0.05, s.ad_value(105)), s.v[85])), 1.0)), s.v[84]));

        s.store_add_ad_rhs(109, 106, A::scale(A::ln(A::offset(A::exp(A::scale(A::sub_from_scalar(0.05, s.ad_value(106)), s.v[85])), 1.0)), s.v[84]));

        s.store_div_from_scalar(119, 1.0, 107);

        s.store_div_from_scalar(120, 1.0, 108);

        s.store_div_from_scalar(121, 1.0, 109);

        s.v[122] = (1.0 - s.v[9]);

        s.v[123] = (1.0 - s.v[10]);

        s.v[124] = (1.0 - s.v[11]);

        s.v[125] = (1.0 / s.v[122]);

        s.v[126] = (1.0 / s.v[123]);

        s.v[127] = (1.0 / s.v[124]);

        s.store_scale_ad(128, A::powf(A::scale(s.ad_value(119), s.v[6]), s.v[9]), s.v[3]);

        s.store_scale_ad(129, A::powf(A::scale(s.ad_value(120), s.v[7]), s.v[10]), s.v[4]);

        s.store_scale_ad(130, A::powf(A::scale(s.ad_value(121), s.v[8]), s.v[11]), s.v[5]);

        s.store_scaled_mul(131, 128, 107, s.v[125]);

        s.store_scaled_mul(132, 129, 108, s.v[126]);

        s.store_scaled_mul(133, 130, 109, s.v[127]);

        s.store_scale(134, 128, 2.0);

        s.store_scale(135, 129, 2.0);

        s.store_scale(136, 130, 2.0);

        s.v[137] = (s.v[0] / s.v[3]);

        s.v[138] = ((s.v[18] * s.v[0]) / s.v[4]);

        s.v[139] = ((s.v[19] * s.v[0]) / s.v[5]);

        s.v[140] = (1.0 / s.v[137]);

        s.v[141] = (1.0 / s.v[138]);

        s.v[142] = (1.0 / s.v[139]);

        s.v[143] = (1.0 / s.v[6]);

        s.v[144] = (1.0 / s.v[7]);

        s.v[145] = (1.0 / s.v[8]);

        s.v[86] = (1.772453850905516 * 0.29214664);

        s.v[87] = (((((-5.0) * 0.29214664) + 6.0) - ((s.v[86]) as f64).powf((-2.0))) / 3.0);

        s.v[88] = ((1.0 - 0.29214664) - s.v[87]);

        s.v[146] = ((0.5 * s.v[95])).max(s.v[84]);

        s.v[147] = ((0.5 * s.v[96])).max(s.v[84]);

        s.v[148] = ((0.5 * s.v[97])).max(s.v[84]);

        s.v[149] = (s.v[146] * s.v[85]);

        s.v[150] = (s.v[147] * s.v[85]);

        s.v[151] = (s.v[148] * s.v[85]);

        s.v[152] = (((((((32.0 * s.v[26]) * 9.1093826e-31) * 1.6021918e-19) * ((s.v[146] * s.v[146]) * s.v[146]))) as f64).sqrt() / (3.0 * 1.05457168e-34));

        s.v[153] = (((((((32.0 * s.v[27]) * 9.1093826e-31) * 1.6021918e-19) * ((s.v[147] * s.v[147]) * s.v[147]))) as f64).sqrt() / (3.0 * 1.05457168e-34));

        s.v[154] = (((((((32.0 * s.v[28]) * 9.1093826e-31) * 1.6021918e-19) * ((s.v[148] * s.v[148]) * s.v[148]))) as f64).sqrt() / (3.0 * 1.05457168e-34));

        s.store_scale_ad(155, A::offset(A::scale(A::sub_from_scalar(s.v[79], s.ad_value(78)), s.v[35]), 1.0), s.v[32]);

        s.store_scale_ad(156, A::offset(A::scale(A::sub_from_scalar(s.v[79], s.ad_value(78)), s.v[36]), 1.0), s.v[33]);

        s.store_scale_ad(157, A::offset(A::scale(A::sub_from_scalar(s.v[79], s.ad_value(78)), s.v[37]), 1.0), s.v[34]);

        if !(s.v[155] > 0.0) {
            s.store_scalar(155, 0.0);
        }

        if !(s.v[156] > 0.0) {
            s.store_scalar(156, 0.0);
        }

        if !(s.v[157] > 0.0) {
            s.store_scalar(157, 0.0);
        }

        s.v[158] = ((s.v[44] - 1.0) / s.v[44]);

        s.v[159] = (1.0 / (1.0 - ((s.v[158]) as f64).powf(s.v[41])));

        s.v[160] = (1.0 / (1.0 - ((s.v[158]) as f64).powf(s.v[42])));

        s.v[161] = (1.0 / (1.0 - ((s.v[158]) as f64).powf(s.v[43])));

        s.store_scale_ad(38, A::offset(A::mul(A::sub_from_scalar(s.v[79], s.ad_value(78)), A::offset(A::scale(A::sub_from_scalar(s.v[79], s.ad_value(78)), s.v[57]), s.v[56])), 1.0), s.v[38]);

        s.store_scale_ad(39, A::offset(A::mul(A::sub_from_scalar(s.v[79], s.ad_value(78)), A::offset(A::scale(A::sub_from_scalar(s.v[79], s.ad_value(78)), s.v[59]), s.v[58])), 1.0), s.v[39]);

        s.store_scale_ad(40, A::offset(A::mul(A::sub_from_scalar(s.v[79], s.ad_value(78)), A::offset(A::scale(A::sub_from_scalar(s.v[79], s.ad_value(78)), s.v[61]), s.v[60])), 1.0), s.v[40]);

        s.v[390] = if (s.v[38] <= 0.1) { 1.0 } else { 0.0 };

        if (s.v[390] != 0.0) {
            s.store_scalar(38, 0.1);
        }

        if (s.v[390] != 0.0) {
            s.store_scalar(162, 10.0);
        }

        if (!(s.v[390] != 0.0)) {
            s.store_div_from_scalar(162, 1.0, 38);
        }

        s.v[391] = if (s.v[39] <= 0.1) { 1.0 } else { 0.0 };

        if (s.v[391] != 0.0) {
            s.store_scalar(39, 0.1);
        }

        if (s.v[391] != 0.0) {
            s.store_scalar(163, 10.0);
        }

        if (!(s.v[391] != 0.0)) {
            s.store_div_from_scalar(163, 1.0, 39);
        }

        s.v[392] = if (s.v[40] <= 0.1) { 1.0 } else { 0.0 };

        if (s.v[392] != 0.0) {
            s.store_scalar(40, 0.1);
        }

        if (s.v[392] != 0.0) {
            s.store_scalar(164, 10.0);
        }

        if (!(s.v[392] != 0.0)) {
            s.store_div_from_scalar(164, 1.0, 40);
        }

        s.v[179] = (1.0 - (0.01 * s.v[77]));

        s.store_scale(165, 162, ((-((s.v[159] * s.v[159]) * ((s.v[158]) as f64).powf((s.v[41] - 1.0)))) * s.v[41]));

        s.store_scale(166, 163, ((-((s.v[160] * s.v[160]) * ((s.v[158]) as f64).powf((s.v[42] - 1.0)))) * s.v[42]));

        s.store_scale(167, 164, ((-((s.v[161] * s.v[161]) * ((s.v[158]) as f64).powf((s.v[43] - 1.0)))) * s.v[43]));

        s.v[308] = (p.p87 * 1000000.0);

        s.v[310] = (p.p89 * 1000000.0);

        s.v[309] = (p.p88 * 1000000.0);

        s.v[307] = s.v[308];

        s.v[313] = s.v[62];

        s.v[311] = (1450.0 * 0.0001);

        s.v[312] = (500.0 * 0.0001);

        s.v[368] = 0.6;

        s.v[369] = 0.001;

        s.store_scale(318, 176, 1.45e16);

        s.store_scale_ad(319, A::square(s.ad_value(318)), 1.0 / (s.v[307]));

        s.store_powf(316, 80, (-1.5));

        s.store_scale(320, 316, (s.v[311] * 1.0 / (s.v[85])));

        s.store_scale(321, 316, (s.v[312] * 1.0 / (s.v[85])));

        s.store_div_ad(322, A::mul(A::scale(s.ad_value(320), 2.0), s.ad_value(321)), A::add(s.ad_value(320), s.ad_value(321)));

        s.store_powf(317, 80, p.p97);

        s.store_scale(324, 317, p.p93);

        s.store_sqrt_ad(323, A::mul(s.ad_value(324), s.ad_value(322)));

        s.store_scale_ad(347, A::ln(A::div_from_scalar(s.v[307], s.ad_value(319))), (s.v[313] / s.v[85]));

        s.store_scale_ad(348, A::add(A::ln(A::div_from_scalar(s.v[307], s.ad_value(319))), A::div_from_scalar(p.p94, s.ad_value(323))), (s.v[313] / s.v[85]));

        s.v[256] = (((((if (p.p99 > 0.0) { p.p99 } else { 0.0 }) * s.v[76]) * s.v[76]) * s.v[179]) * s.v[179]);

        s.v[257] = (((if (p.p100 > 0.0) { p.p100 } else { 0.0 }) * s.v[76]) * s.v[179]);

        s.v[258] = (((if (p.p101 > 0.0) { p.p101 } else { 0.0 }) * s.v[76]) * s.v[179]);

        s.v[263] = 0.0;

        s.v[281] = 0.0;

        s.v[282] = 0.0;

        s.v[283] = 0.0;

        s.v[393] = if ((s.v[101] * s.v[256]) > 0.0) { 1.0 } else { 0.0 };

        if (s.v[393] != 0.0) {
            s.store_scale_ad(168, A::ln(A::offset(A::div_from_scalar(s.v[2], A::scale(s.ad_value(101), s.v[256])), 1.0)), (s.v[84] * s.v[62]));
        }

        if (!(s.v[393] != 0.0)) {
            s.store_scalar(168, 100000000.0);
        }

        s.v[394] = if ((s.v[102] * s.v[257]) > 0.0) { 1.0 } else { 0.0 };

        if (s.v[394] != 0.0) {
            s.store_scale_ad(169, A::ln(A::offset(A::div_from_scalar(s.v[2], A::scale(s.ad_value(102), s.v[257])), 1.0)), (s.v[84] * s.v[64]));
        }

        if (!(s.v[394] != 0.0)) {
            s.store_scalar(169, 100000000.0);
        }

        s.v[395] = if ((s.v[103] * s.v[258]) > 0.0) { 1.0 } else { 0.0 };

        if (s.v[395] != 0.0) {
            s.store_scale_ad(170, A::ln(A::offset(A::div_from_scalar(s.v[2], A::scale(s.ad_value(103), s.v[258])), 1.0)), (s.v[84] * s.v[63]));
        }

        if (!(s.v[395] != 0.0)) {
            s.store_scalar(170, 100000000.0);
        }

        s.store_ad(262, &A::min(A::min(s.ad_value(168), s.ad_value(169)), s.ad_value(170)));

        s.v[396] = if ((((s.v[262] * s.v[85])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if (s.v[396] != 0.0) {
            s.store_exp_ad(263, A::scale(s.ad_value(262), s.v[85]));
        }

        s.v[397] = if ((s.v[262] * s.v[85]) < (-230.25850929940458)) { 1.0 } else { 0.0 };

        if ((!(s.v[396] != 0.0)) && (s.v[397] != 0.0)) {
            s.store_div_from_scalar_ad(263, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(s.ad_value(262), s.v[85])), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(s.ad_value(262), s.v[85])), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::scale(s.ad_value(262), s.v[85])), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if ((!(s.v[396] != 0.0)) && (!(s.v[397] != 0.0))) {
            s.store_scale_ad(263, A::offset(A::mul(A::offset(A::scale(s.ad_value(262), s.v[85]), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::scale(s.ad_value(262), s.v[85]), (-230.25850929940458)), A::offset(A::scale(A::offset(A::scale(s.ad_value(262), s.v[85]), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
        }

        s.copy_ad(110, 107);

        s.copy_ad(111, 108);

        s.copy_ad(112, 109);

        s.v[113] = s.v[9];

        s.v[114] = s.v[10];

        s.v[115] = s.v[11];

        s.v[116] = s.v[6];

        s.v[117] = s.v[7];

        s.v[118] = s.v[8];

        s.v[398] = if (s.v[256] == 0.0) { 1.0 } else { 0.0 };

        if (s.v[398] != 0.0) {
            s.store_add(110, 108, 109);
        }

        if (s.v[398] != 0.0) {
            s.store_scalar(113, (0.9 * (s.v[10]).min(s.v[11])));
        }

        if (s.v[398] != 0.0) {
            s.store_scalar(116, (s.v[7] + s.v[8]));
        }

        s.v[399] = if (s.v[257] == 0.0) { 1.0 } else { 0.0 };

        if (s.v[399] != 0.0) {
            s.store_add(111, 107, 109);
        }

        if (s.v[399] != 0.0) {
            s.store_scalar(114, (0.9 * (s.v[9]).min(s.v[11])));
        }

        if (s.v[399] != 0.0) {
            s.store_scalar(117, (s.v[6] + s.v[8]));
        }

        s.v[400] = if (s.v[258] == 0.0) { 1.0 } else { 0.0 };

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
        if (s.v[400] != 0.0) {
            s.store_add(112, 107, 108);
        }

        if (s.v[400] != 0.0) {
            s.store_scalar(115, (0.9 * (s.v[9]).min(s.v[10])));
        }

        if (s.v[400] != 0.0) {
            s.store_scalar(118, (s.v[6] + s.v[7]));
        }

        s.store_ad(264, &A::min(A::min(s.ad_value(110), s.ad_value(111)), s.ad_value(112)));

        s.store_scale(265, 264, 0.1);

        s.store_ad(91, &A::max(A::max(s.ad_value(113), s.ad_value(114)), s.ad_value(115)));

        s.store_mul_ad_rhs(266, 264, A::sub_from_scalar(1.0, A::pow_from_scalar(2.0, A::div_from_scalar((-1.0), s.ad_value(91)))));

        s.store_offset_ad(267, A::min(A::min(s.ad_value(116), s.ad_value(117)), s.ad_value(118)), (-0.05));

        s.store_add_ad(289, A::add(A::scale(s.ad_value(101), s.v[256]), A::scale(s.ad_value(102), s.v[257])), A::scale(s.ad_value(103), s.v[258]));

        s.v[300] = 0.0;

        s.v[301] = 1.0;

        s.v[303] = 1.0;

        s.v[302] = 0.0;

        s.v[305] = 1.0;

        s.v[304] = 0.0;

        s.v[306] = 0.0;

        s.v[294] = 0.0;

        s.v[295] = 0.0;

        s.v[296] = 0.0;

        s.v[297] = 0.0;

        s.v[298] = 0.0;

        s.v[299] = 0.0;

        s.v[196] = 0.0;

        s.v[197] = 0.0;

        s.v[185] = 0.0;

        s.v[186] = 0.0;

        s.v[187] = 0.0;

        s.v[188] = 0.0;

        s.v[189] = 0.0;

        s.v[198] = 0.0;

        s.v[199] = 0.0;

        s.v[200] = 0.0;

        s.v[208] = 0.0;

        s.v[259] = 1.0;

        s.v[260] = 1.0;

        s.v[261] = 1.0;

        s.v[195] = 0.0;

        s.v[203] = 0.0;

        s.v[204] = 0.0;

        s.v[370] = 0.0;

        s.v[372] = 0.0;

        s.v[371] = 0.0;

        s.v[345] = 0.0;

        s.v[338] = 0.0;

        s.v[339] = 0.0;

        s.v[336] = 0.0;

        s.v[337] = 0.0;

        s.v[344] = 0.0;

        s.v[333] = (1.6021918e-19 * s.v[256]);

        s.v[343] = ((((2.0 * s.v[0]) / (1.6021918e-19 * s.v[307]))) as f64).sqrt();

        s.v[314] = ((p.p94 - s.v[343]) - 1e-7);

        s.v[315] = ((4.0 * p.p94) * 1e-7);

        if !(s.v[315] > 0.0) {
            s.store_scalar(315, (-s.v[315]));
        }

        s.store_sqrt_ad(315, A::offset(s.ad_value(315), (s.v[314] * s.v[314])));

        s.store_sub_from_scalar_ad(343, p.p94, A::scale(A::offset(s.ad_value(315), s.v[314]), 0.5));

        s.v[413] = if (s.v[45] > 0.9) { 1.0 } else { 0.0 };

        s.v[414] = if ((((((((s.v[62] - s.v[63])) as f64).abs() > 1e-6) && (s.v[256] > 0.0)) && (s.v[258] > 0.0)) || ((((((s.v[62] - s.v[64])) as f64).abs() > 1e-6) && (s.v[256] > 0.0)) && (s.v[257] > 0.0))) || ((((((s.v[63] - s.v[64])) as f64).abs() > 1e-6) && (s.v[258] > 0.0)) && (s.v[257] > 0.0))) { 1.0 } else { 0.0 };

        if ((s.v[413] != 0.0) && (s.v[414] != 0.0)) {
            s.store_scalar(45, 0.0);
        }

        s.v[415] = if (s.v[256] > 0.0) { 1.0 } else { 0.0 };

        if (((s.v[413] != 0.0) && (!(s.v[414] != 0.0))) && (s.v[415] != 0.0)) {
            s.store_scalar(301, s.v[62]);
        }

        s.v[416] = if (s.v[258] > 0.0) { 1.0 } else { 0.0 };

        if (((s.v[413] != 0.0) && (!(s.v[414] != 0.0))) && (s.v[416] != 0.0)) {
            s.store_scalar(301, s.v[63]);
        }

        s.v[417] = if (s.v[257] > 0.0) { 1.0 } else { 0.0 };

        if (((s.v[413] != 0.0) && (!(s.v[414] != 0.0))) && (s.v[417] != 0.0)) {
            s.store_scalar(301, s.v[64]);
        }

        s.v[418] = if (s.v[45] == 1.0) { 1.0 } else { 0.0 };

        if (s.v[418] != 0.0) {
            s.store_scalar(419, 0.0);
        }

        if (s.v[418] != 0.0) {
            s.store_scalar(420, 0.0);
        }

        if (s.v[418] != 0.0) {
            s.store_scalar(421, 0.0);
        }

        if (s.v[418] != 0.0) {
            s.store_scalar(422, 0.0);
        }

        if (s.v[418] != 0.0) {
            s.store_scalar(423, 0.0);
        }

        if (s.v[418] != 0.0) {
            s.store_scalar(424, 0.0);
        }

        if (s.v[418] != 0.0) {
            s.store_scalar(425, 0.0);
        }

        if (s.v[418] != 0.0) {
            s.store_scalar(426, 0.0);
        }

        if (s.v[418] != 0.0) {
            s.store_scalar(427, 0.0);
        }

        if (s.v[418] != 0.0) {
            s.store_scalar(277, 0.0);
        }

        if (s.v[418] != 0.0) {
            s.store_scalar(428, 0.0);
        }

        if (s.v[418] != 0.0) {
            s.store_scalar(429, 0.0);
        }

        if (s.v[418] != 0.0) {
            s.store_scalar(430, 0.0);
        }

        if (s.v[418] != 0.0) {
            s.store_scalar(431, 0.0);
        }

        if (s.v[418] != 0.0) {
            s.store_scalar(432, 0.0);
        }

        if (s.v[418] != 0.0) {
            s.store_scalar(433, 0.0);
        }

        if (s.v[418] != 0.0) {
            s.store_scalar(434, 0.0);
        }

        if (s.v[418] != 0.0) {
            s.store_scalar(435, 0.0);
        }

        if (s.v[418] != 0.0) {
            s.store_scalar(436, 0.0);
        }

        if (s.v[418] != 0.0) {
            s.store_scalar(437, 0.0);
        }

        if (s.v[418] != 0.0) {
            s.store_scalar(438, 0.0);
        }

        if (s.v[418] != 0.0) {
            s.store_scalar(439, 0.0);
        }

        if (s.v[418] != 0.0) {
            s.store_scalar(440, 0.0);
        }

        if (s.v[418] != 0.0) {
            s.store_scalar(441, 0.0);
        }

        if (s.v[418] != 0.0) {
            s.store_scalar(442, 0.0);
        }

        if (s.v[418] != 0.0) {
            s.store_scalar(443, 0.0);
        }

        if (s.v[418] != 0.0) {
            s.store_scalar(444, 0.0);
        }

        if (s.v[418] != 0.0) {
            s.store_scalar(445, 0.0);
        }

        if (s.v[418] != 0.0) {
            s.store_scalar(446, 0.0);
        }

        if (s.v[418] != 0.0) {
            s.store_scalar(447, 0.0);
        }

        if (s.v[418] != 0.0) {
            s.store_scalar(448, 0.0);
        }

        if (s.v[418] != 0.0) {
            s.store_scalar(449, 0.0);
        }

        if (s.v[418] != 0.0) {
            s.store_scalar(450, 0.0);
        }

        if (s.v[418] != 0.0) {
            s.store_scalar(451, 0.0);
        }

        if (s.v[418] != 0.0) {
            s.store_scalar(452, 0.0);
        }

        if (s.v[418] != 0.0) {
            s.store_scalar(453, 0.0);
        }

        if (s.v[418] != 0.0) {
            s.store_scalar(454, 0.0);
        }

        if (s.v[418] != 0.0) {
            s.store_scalar(455, 0.0);
        }

        if (s.v[418] != 0.0) {
            s.store_scalar(456, 0.0);
        }

        if (s.v[418] != 0.0) {
            s.store_scalar(457, 0.0);
        }

        if (s.v[418] != 0.0) {
            s.store_scalar(458, 0.0);
        }

        if (s.v[418] != 0.0) {
            s.store_scalar(459, 0.0);
        }

        if (s.v[418] != 0.0) {
            s.store_scalar(460, 0.0);
        }

        if (s.v[418] != 0.0) {
            s.store_scalar(461, 0.0);
        }

        if (s.v[418] != 0.0) {
            s.store_scalar(462, 0.0);
        }

        if (s.v[418] != 0.0) {
            s.store_scalar(205, 0.4);
        }

        if (s.v[418] != 0.0) {
            s.store_scalar(206, 0.65);
        }

        if (s.v[418] != 0.0) {
            s.store_scalar(207, 0.8);
        }

        if (s.v[418] != 0.0) {
            s.store_scale_ad(190, A::neg(s.ad_value(205)), s.v[46]);
        }

        if (s.v[418] != 0.0) {
            s.store_scale_ad(191, A::neg(s.ad_value(206)), s.v[46]);
        }

        if (s.v[418] != 0.0) {
            s.store_scale_ad(192, A::neg(s.ad_value(207)), s.v[46]);
        }

        if (s.v[418] != 0.0) {
            s.store_scalar(193, 0.1);
        }

        if (s.v[418] != 0.0) {
            s.store_scalar(194, 0.2);
        }

        s.v[463] = if !(((s.v[256] == 0.0) && (s.v[257] == 0.0)) && (s.v[258] == 0.0)) { 1.0 } else { 0.0 };

        if ((s.v[418] != 0.0) && (s.v[463] != 0.0)) {
            s.store_mul_ad_lhs(422, A::scale(s.ad_value(265), 4.0), 265);
        }

        if ((s.v[418] != 0.0) && (s.v[463] != 0.0)) {
            s.store_div(423, 265, 266);
        }

        if ((s.v[418] != 0.0) && (s.v[463] != 0.0)) {
            s.store_add_ad_rhs(424, 190, A::mul(s.ad_value(265), s.ad_value(423)));
        }

        if ((s.v[418] != 0.0) && (s.v[463] != 0.0)) {
            s.store_add(425, 266, 424);
        }

        if ((s.v[418] != 0.0) && (s.v[463] != 0.0)) {
            s.store_sub(426, 266, 424);
        }

        if ((s.v[418] != 0.0) && (s.v[463] != 0.0)) {
            s.store_sqrt_ad(427, A::add(A::square(s.ad_value(426)), s.ad_value(422)));
        }

        if ((s.v[418] != 0.0) && (s.v[463] != 0.0)) {
            s.store_scale_ad(428, A::div(A::mul(s.ad_value(190), s.ad_value(266)), A::add(s.ad_value(425), s.ad_value(427))), 2.0);
        }

        s.v[464] = if (s.v[190] < s.v[262]) { 1.0 } else { 0.0 };

        s.v[465] = if ((((0.5 * (s.v[190] * s.v[85]))) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (s.v[464] != 0.0)) && (s.v[465] != 0.0)) {
            s.store_exp_ad(430, A::scale(s.ad_value(190), (s.v[85] * 0.5)));
        }

        s.v[466] = if ((0.5 * (s.v[190] * s.v[85])) < (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (s.v[464] != 0.0)) && (!(s.v[465] != 0.0))) && (s.v[466] != 0.0)) {
            let assign3970_ad_e2385: A = A::div_from_scalar(1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(s.ad_value(190), (s.v[85] * 0.5))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(s.ad_value(190), (s.v[85] * 0.5))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::scale(s.ad_value(190), (s.v[85] * 0.5))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
            s.store_ad(430, &assign3970_ad_e2385);
        }

        if (((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (s.v[464] != 0.0)) && (!(s.v[465] != 0.0))) && (!(s.v[466] != 0.0))) {
            s.store_scale_ad(430, A::offset(A::mul(A::offset(A::scale(s.ad_value(190), (s.v[85] * 0.5)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::scale(s.ad_value(190), (s.v[85] * 0.5)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::scale(s.ad_value(190), (s.v[85] * 0.5)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
        }

        if (((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (s.v[464] != 0.0)) {
            s.store_scale_ad(363, A::square(s.ad_value(318)), 1.0 / (s.v[308]));
        }

        if (((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (s.v[464] != 0.0)) {
            s.store_scale_ad(362, A::ln(A::div_from_scalar(s.v[308], s.ad_value(363))), (s.v[62] / s.v[85]));
        }

        s.v[467] = if (s.v[62] < p.p85) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (s.v[464] != 0.0)) && (s.v[467] != 0.0)) {
            s.store_offset_ad(360, A::scale(A::sub(s.ad_value(190), s.ad_value(362)), p.p86), s.v[62]);
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (s.v[464] != 0.0)) && (s.v[467] != 0.0)) {
            s.store_sub_from_scalar_ad(350, s.v[62], A::scale(s.ad_value(362), p.p86));
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (s.v[464] != 0.0)) && (s.v[467] != 0.0)) {
            s.store_offset_ad(314, A::sub_from_scalar(p.p85, s.ad_value(360)), (-0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (s.v[464] != 0.0)) && (s.v[467] != 0.0)) {
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (s.v[464] != 0.0)) && (s.v[467] != 0.0)) {
            s.store_ad(315, &{
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (s.v[464] != 0.0)) && (s.v[467] != 0.0)) {
            s.store_sqrt_ad(315, A::add(A::square(s.ad_value(314)), s.ad_value(315)));
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (s.v[464] != 0.0)) && (s.v[467] != 0.0)) {
            s.store_sub_from_scalar_ad(361, p.p85, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5));
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (s.v[464] != 0.0)) && (s.v[467] != 0.0)) {
            s.store_offset(314, 361, (((-s.v[62])) + ((-0.01))));
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (s.v[464] != 0.0)) && (s.v[467] != 0.0)) {
            s.store_scalar(315, ((4.0 * s.v[62]) * 0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (s.v[464] != 0.0)) && (s.v[467] != 0.0)) {
            s.store_ad(315, &{
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (s.v[464] != 0.0)) && (s.v[467] != 0.0)) {
            s.store_sqrt_ad(315, A::add(A::square(s.ad_value(314)), s.ad_value(315)));
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (s.v[464] != 0.0)) && (s.v[467] != 0.0)) {
            s.store_offset_ad(359, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5), s.v[62]);
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (s.v[464] != 0.0)) && (s.v[467] != 0.0)) {
            s.store_offset_ad(314, A::sub_from_scalar(p.p85, s.ad_value(350)), (-0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (s.v[464] != 0.0)) && (s.v[467] != 0.0)) {
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (s.v[464] != 0.0)) && (s.v[467] != 0.0)) {
            s.store_ad(315, &{
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (s.v[464] != 0.0)) && (s.v[467] != 0.0)) {
            s.store_sqrt_ad(315, A::add(A::square(s.ad_value(314)), s.ad_value(315)));
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (s.v[464] != 0.0)) && (s.v[467] != 0.0)) {
            s.store_sub_from_scalar_ad(350, p.p85, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5));
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (s.v[464] != 0.0)) && (s.v[467] != 0.0)) {
            s.store_offset(314, 350, (((-s.v[62])) + ((-0.01))));
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (s.v[464] != 0.0)) && (s.v[467] != 0.0)) {
            s.store_scalar(315, ((4.0 * s.v[62]) * 0.01));
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
        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (s.v[464] != 0.0)) && (s.v[467] != 0.0)) {
            s.store_ad(315, &{
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (s.v[464] != 0.0)) && (s.v[467] != 0.0)) {
            s.store_sqrt_ad(315, A::add(A::square(s.ad_value(314)), s.ad_value(315)));
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (s.v[464] != 0.0)) && (s.v[467] != 0.0)) {
            s.store_offset_ad(350, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5), s.v[62]);
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (s.v[464] != 0.0)) && (!(s.v[467] != 0.0))) {
            s.store_scalar(350, s.v[62]);
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (s.v[464] != 0.0)) && (!(s.v[467] != 0.0))) {
            s.store_scalar(359, s.v[62]);
        }

        s.v[468] = if ((((s.v[85] * ((s.v[190] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85))))) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (s.v[464] != 0.0)) && (s.v[468] != 0.0)) {
            s.store_exp_ad(370, A::scale(A::add(A::div(s.ad_value(190), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]));
        }

        s.v[469] = if ((s.v[85] * ((s.v[190] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85)))) < (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (s.v[464] != 0.0)) && (!(s.v[468] != 0.0))) && (s.v[469] != 0.0)) {
            let assign4290_ad_e2951: A = A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(A::add(A::div(s.ad_value(190), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85])), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::scale(A::add(A::div(s.ad_value(190), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85])), 0.3333333333333333), 1.0));
            s.store_div_from_scalar_ad(370, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(A::add(A::div(s.ad_value(190), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85])), A::offset(A::scale(assign4290_ad_e2951, 0.5), 1.0)), 1.0));
        }

        if (((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (s.v[464] != 0.0)) && (!(s.v[468] != 0.0))) && (!(s.v[469] != 0.0))) {
            let assign4300_ad_e3029: A = A::mul(A::offset(A::scale(A::add(A::div(s.ad_value(190), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]), (-230.25850929940458)), A::offset(A::scale(A::offset(A::scale(A::add(A::div(s.ad_value(190), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]), (-230.25850929940458)), 0.3333333333333333), 1.0));
            s.store_scale_ad(370, A::offset(A::mul(A::offset(A::scale(A::add(A::div(s.ad_value(190), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]), (-230.25850929940458)), A::offset(A::scale(assign4300_ad_e3029, 0.5), 1.0)), 1.0), 1e100);
        }

        if (((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (s.v[464] != 0.0)) {
            s.store_scale_ad(363, A::square(s.ad_value(318)), 1.0 / (s.v[310]));
        }

        if (((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (s.v[464] != 0.0)) {
            s.store_scale_ad(362, A::ln(A::div_from_scalar(s.v[310], s.ad_value(363))), (s.v[64] / s.v[85]));
        }

        s.v[470] = if (s.v[64] < p.p85) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (s.v[464] != 0.0)) && (s.v[470] != 0.0)) {
            s.store_offset_ad(360, A::scale(A::sub(s.ad_value(190), s.ad_value(362)), p.p86), s.v[64]);
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (s.v[464] != 0.0)) && (s.v[470] != 0.0)) {
            s.store_sub_from_scalar_ad(350, s.v[64], A::scale(s.ad_value(362), p.p86));
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (s.v[464] != 0.0)) && (s.v[470] != 0.0)) {
            s.store_offset_ad(314, A::sub_from_scalar(p.p85, s.ad_value(360)), (-0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (s.v[464] != 0.0)) && (s.v[470] != 0.0)) {
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (s.v[464] != 0.0)) && (s.v[470] != 0.0)) {
            s.store_ad(315, &{
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (s.v[464] != 0.0)) && (s.v[470] != 0.0)) {
            s.store_sqrt_ad(315, A::add(A::square(s.ad_value(314)), s.ad_value(315)));
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (s.v[464] != 0.0)) && (s.v[470] != 0.0)) {
            s.store_sub_from_scalar_ad(361, p.p85, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5));
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (s.v[464] != 0.0)) && (s.v[470] != 0.0)) {
            s.store_offset(314, 361, (((-s.v[64])) + ((-0.01))));
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (s.v[464] != 0.0)) && (s.v[470] != 0.0)) {
            s.store_scalar(315, ((4.0 * s.v[64]) * 0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (s.v[464] != 0.0)) && (s.v[470] != 0.0)) {
            s.store_ad(315, &{
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (s.v[464] != 0.0)) && (s.v[470] != 0.0)) {
            s.store_sqrt_ad(315, A::add(A::square(s.ad_value(314)), s.ad_value(315)));
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (s.v[464] != 0.0)) && (s.v[470] != 0.0)) {
            s.store_offset_ad(359, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5), s.v[64]);
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (s.v[464] != 0.0)) && (s.v[470] != 0.0)) {
            s.store_offset_ad(314, A::sub_from_scalar(p.p85, s.ad_value(350)), (-0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (s.v[464] != 0.0)) && (s.v[470] != 0.0)) {
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (s.v[464] != 0.0)) && (s.v[470] != 0.0)) {
            s.store_ad(315, &{
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (s.v[464] != 0.0)) && (s.v[470] != 0.0)) {
            s.store_sqrt_ad(315, A::add(A::square(s.ad_value(314)), s.ad_value(315)));
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (s.v[464] != 0.0)) && (s.v[470] != 0.0)) {
            s.store_sub_from_scalar_ad(350, p.p85, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5));
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (s.v[464] != 0.0)) && (s.v[470] != 0.0)) {
            s.store_offset(314, 350, (((-s.v[64])) + ((-0.01))));
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (s.v[464] != 0.0)) && (s.v[470] != 0.0)) {
            s.store_scalar(315, ((4.0 * s.v[64]) * 0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (s.v[464] != 0.0)) && (s.v[470] != 0.0)) {
            s.store_ad(315, &{
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (s.v[464] != 0.0)) && (s.v[470] != 0.0)) {
            s.store_sqrt_ad(315, A::add(A::square(s.ad_value(314)), s.ad_value(315)));
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (s.v[464] != 0.0)) && (s.v[470] != 0.0)) {
            s.store_offset_ad(350, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5), s.v[64]);
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (s.v[464] != 0.0)) && (!(s.v[470] != 0.0))) {
            s.store_scalar(350, s.v[64]);
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (s.v[464] != 0.0)) && (!(s.v[470] != 0.0))) {
            s.store_scalar(359, s.v[64]);
        }

        s.v[471] = if ((((s.v[85] * ((s.v[190] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85))))) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (s.v[464] != 0.0)) && (s.v[471] != 0.0)) {
            s.store_exp_ad(371, A::scale(A::add(A::div(s.ad_value(190), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]));
        }

        s.v[472] = if ((s.v[85] * ((s.v[190] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85)))) < (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (s.v[464] != 0.0)) && (!(s.v[471] != 0.0))) && (s.v[472] != 0.0)) {
            let assign4610_ad_e3552: A = A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(A::add(A::div(s.ad_value(190), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85])), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::scale(A::add(A::div(s.ad_value(190), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85])), 0.3333333333333333), 1.0));
            s.store_div_from_scalar_ad(371, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(A::add(A::div(s.ad_value(190), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85])), A::offset(A::scale(assign4610_ad_e3552, 0.5), 1.0)), 1.0));
        }

        if (((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (s.v[464] != 0.0)) && (!(s.v[471] != 0.0))) && (!(s.v[472] != 0.0))) {
            let assign4620_ad_e3630: A = A::mul(A::offset(A::scale(A::add(A::div(s.ad_value(190), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]), (-230.25850929940458)), A::offset(A::scale(A::offset(A::scale(A::add(A::div(s.ad_value(190), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]), (-230.25850929940458)), 0.3333333333333333), 1.0));
            s.store_scale_ad(371, A::offset(A::mul(A::offset(A::scale(A::add(A::div(s.ad_value(190), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]), (-230.25850929940458)), A::offset(A::scale(assign4620_ad_e3630, 0.5), 1.0)), 1.0), 1e100);
        }

        if (((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (s.v[464] != 0.0)) {
            s.store_scale_ad(363, A::square(s.ad_value(318)), 1.0 / (s.v[309]));
        }

        if (((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (s.v[464] != 0.0)) {
            s.store_scale_ad(362, A::ln(A::div_from_scalar(s.v[309], s.ad_value(363))), (s.v[63] / s.v[85]));
        }

        s.v[473] = if (s.v[63] < p.p85) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (s.v[464] != 0.0)) && (s.v[473] != 0.0)) {
            s.store_offset_ad(360, A::scale(A::sub(s.ad_value(190), s.ad_value(362)), p.p86), s.v[63]);
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (s.v[464] != 0.0)) && (s.v[473] != 0.0)) {
            s.store_sub_from_scalar_ad(350, s.v[63], A::scale(s.ad_value(362), p.p86));
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (s.v[464] != 0.0)) && (s.v[473] != 0.0)) {
            s.store_offset_ad(314, A::sub_from_scalar(p.p85, s.ad_value(360)), (-0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (s.v[464] != 0.0)) && (s.v[473] != 0.0)) {
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (s.v[464] != 0.0)) && (s.v[473] != 0.0)) {
            s.store_ad(315, &{
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (s.v[464] != 0.0)) && (s.v[473] != 0.0)) {
            s.store_sqrt_ad(315, A::add(A::square(s.ad_value(314)), s.ad_value(315)));
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (s.v[464] != 0.0)) && (s.v[473] != 0.0)) {
            s.store_sub_from_scalar_ad(361, p.p85, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5));
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (s.v[464] != 0.0)) && (s.v[473] != 0.0)) {
            s.store_offset(314, 361, (((-s.v[63])) + ((-0.01))));
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (s.v[464] != 0.0)) && (s.v[473] != 0.0)) {
            s.store_scalar(315, ((4.0 * s.v[63]) * 0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (s.v[464] != 0.0)) && (s.v[473] != 0.0)) {
            s.store_ad(315, &{
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (s.v[464] != 0.0)) && (s.v[473] != 0.0)) {
            s.store_sqrt_ad(315, A::add(A::square(s.ad_value(314)), s.ad_value(315)));
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (s.v[464] != 0.0)) && (s.v[473] != 0.0)) {
            s.store_offset_ad(359, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5), s.v[63]);
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (s.v[464] != 0.0)) && (s.v[473] != 0.0)) {
            s.store_offset_ad(314, A::sub_from_scalar(p.p85, s.ad_value(350)), (-0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (s.v[464] != 0.0)) && (s.v[473] != 0.0)) {
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (s.v[464] != 0.0)) && (s.v[473] != 0.0)) {
            s.store_ad(315, &{
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (s.v[464] != 0.0)) && (s.v[473] != 0.0)) {
            s.store_sqrt_ad(315, A::add(A::square(s.ad_value(314)), s.ad_value(315)));
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (s.v[464] != 0.0)) && (s.v[473] != 0.0)) {
            s.store_sub_from_scalar_ad(350, p.p85, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5));
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (s.v[464] != 0.0)) && (s.v[473] != 0.0)) {
            s.store_offset(314, 350, (((-s.v[63])) + ((-0.01))));
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (s.v[464] != 0.0)) && (s.v[473] != 0.0)) {
            s.store_scalar(315, ((4.0 * s.v[63]) * 0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (s.v[464] != 0.0)) && (s.v[473] != 0.0)) {
            s.store_ad(315, &{
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (s.v[464] != 0.0)) && (s.v[473] != 0.0)) {
            s.store_sqrt_ad(315, A::add(A::square(s.ad_value(314)), s.ad_value(315)));
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (s.v[464] != 0.0)) && (s.v[473] != 0.0)) {
            s.store_offset_ad(350, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5), s.v[63]);
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (s.v[464] != 0.0)) && (!(s.v[473] != 0.0))) {
            s.store_scalar(350, s.v[63]);
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (s.v[464] != 0.0)) && (!(s.v[473] != 0.0))) {
            s.store_scalar(359, s.v[63]);
        }

        s.v[474] = if ((((s.v[85] * ((s.v[190] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85))))) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (s.v[464] != 0.0)) && (s.v[474] != 0.0)) {
            s.store_exp_ad(372, A::scale(A::add(A::div(s.ad_value(190), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]));
        }

        s.v[475] = if ((s.v[85] * ((s.v[190] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85)))) < (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (s.v[464] != 0.0)) && (!(s.v[474] != 0.0))) && (s.v[475] != 0.0)) {
            let assign4930_ad_e4153: A = A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(A::add(A::div(s.ad_value(190), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85])), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::scale(A::add(A::div(s.ad_value(190), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85])), 0.3333333333333333), 1.0));
            s.store_div_from_scalar_ad(372, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(A::add(A::div(s.ad_value(190), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85])), A::offset(A::scale(assign4930_ad_e4153, 0.5), 1.0)), 1.0));
        }

        if (((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (s.v[464] != 0.0)) && (!(s.v[474] != 0.0))) && (!(s.v[475] != 0.0))) {
            let assign4940_ad_e4231: A = A::mul(A::offset(A::scale(A::add(A::div(s.ad_value(190), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]), (-230.25850929940458)), A::offset(A::scale(A::offset(A::scale(A::add(A::div(s.ad_value(190), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]), (-230.25850929940458)), 0.3333333333333333), 1.0));
            s.store_scale_ad(372, A::offset(A::mul(A::offset(A::scale(A::add(A::div(s.ad_value(190), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]), (-230.25850929940458)), A::offset(A::scale(assign4940_ad_e4231, 0.5), 1.0)), 1.0), 1e100);
        }

        if (((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (!(s.v[464] != 0.0))) {
            s.store_sqrt_ad(430, A::mul(A::offset(A::scale(A::sub(s.ad_value(190), s.ad_value(262)), s.v[85]), 1.0), s.ad_value(263)));
        }

        if (((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (!(s.v[464] != 0.0))) {
            s.store_scale_ad(363, A::square(s.ad_value(318)), 1.0 / (s.v[308]));
        }

        if (((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (!(s.v[464] != 0.0))) {
            s.store_scale_ad(362, A::ln(A::div_from_scalar(s.v[308], s.ad_value(363))), (s.v[62] / s.v[85]));
        }

        s.v[476] = if (s.v[62] < p.p85) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (!(s.v[464] != 0.0))) && (s.v[476] != 0.0)) {
            s.store_offset_ad(360, A::scale(A::sub(s.ad_value(262), s.ad_value(362)), p.p86), s.v[62]);
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (!(s.v[464] != 0.0))) && (s.v[476] != 0.0)) {
            s.store_sub_from_scalar_ad(350, s.v[62], A::scale(s.ad_value(362), p.p86));
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (!(s.v[464] != 0.0))) && (s.v[476] != 0.0)) {
            s.store_offset_ad(314, A::sub_from_scalar(p.p85, s.ad_value(360)), (-0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (!(s.v[464] != 0.0))) && (s.v[476] != 0.0)) {
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (!(s.v[464] != 0.0))) && (s.v[476] != 0.0)) {
            s.store_ad(315, &{
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (!(s.v[464] != 0.0))) && (s.v[476] != 0.0)) {
            s.store_sqrt_ad(315, A::add(A::square(s.ad_value(314)), s.ad_value(315)));
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (!(s.v[464] != 0.0))) && (s.v[476] != 0.0)) {
            s.store_scale_ad(364, A::offset(A::div(s.ad_value(314), s.ad_value(315)), 1.0), 0.5);
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (!(s.v[464] != 0.0))) && (s.v[476] != 0.0)) {
            s.store_sub_from_scalar_ad(361, p.p85, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5));
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (!(s.v[464] != 0.0))) && (s.v[476] != 0.0)) {
            s.store_offset(314, 361, (((-s.v[62])) + ((-0.01))));
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (!(s.v[464] != 0.0))) && (s.v[476] != 0.0)) {
            s.store_scalar(315, ((4.0 * s.v[62]) * 0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (!(s.v[464] != 0.0))) && (s.v[476] != 0.0)) {
            s.store_ad(315, &{
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (!(s.v[464] != 0.0))) && (s.v[476] != 0.0)) {
            s.store_sqrt_ad(315, A::add(A::square(s.ad_value(314)), s.ad_value(315)));
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (!(s.v[464] != 0.0))) && (s.v[476] != 0.0)) {
            s.store_scale_ad(365, A::offset(A::div(s.ad_value(314), s.ad_value(315)), 1.0), 0.5);
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (!(s.v[464] != 0.0))) && (s.v[476] != 0.0)) {
            s.store_offset_ad(359, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5), s.v[62]);
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (!(s.v[464] != 0.0))) && (s.v[476] != 0.0)) {
            s.store_offset_ad(314, A::sub_from_scalar(p.p85, s.ad_value(350)), (-0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (!(s.v[464] != 0.0))) && (s.v[476] != 0.0)) {
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (!(s.v[464] != 0.0))) && (s.v[476] != 0.0)) {
            s.store_ad(315, &{
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (!(s.v[464] != 0.0))) && (s.v[476] != 0.0)) {
            s.store_sqrt_ad(315, A::add(A::square(s.ad_value(314)), s.ad_value(315)));
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (!(s.v[464] != 0.0))) && (s.v[476] != 0.0)) {
            s.store_sub_from_scalar_ad(350, p.p85, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5));
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (!(s.v[464] != 0.0))) && (s.v[476] != 0.0)) {
            s.store_offset(314, 350, (((-s.v[62])) + ((-0.01))));
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (!(s.v[464] != 0.0))) && (s.v[476] != 0.0)) {
            s.store_scalar(315, ((4.0 * s.v[62]) * 0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (!(s.v[464] != 0.0))) && (s.v[476] != 0.0)) {
            s.store_ad(315, &{
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (!(s.v[464] != 0.0))) && (s.v[476] != 0.0)) {
            s.store_sqrt_ad(315, A::add(A::square(s.ad_value(314)), s.ad_value(315)));
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (!(s.v[464] != 0.0))) && (s.v[476] != 0.0)) {
            s.store_offset_ad(350, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5), s.v[62]);
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (!(s.v[464] != 0.0))) && (s.v[476] != 0.0)) {
            s.store_mul_ad_lhs(366, A::scale(s.ad_value(364), p.p86), 365);
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (!(s.v[464] != 0.0))) && (!(s.v[476] != 0.0))) {
            s.store_scalar(350, s.v[62]);
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (!(s.v[464] != 0.0))) && (!(s.v[476] != 0.0))) {
            s.store_scalar(359, s.v[62]);
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (!(s.v[464] != 0.0))) && (!(s.v[476] != 0.0))) {
            s.store_scalar(366, 0.0);
        }

        s.v[477] = if ((((s.v[85] * ((s.v[262] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85))))) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (!(s.v[464] != 0.0))) && (s.v[477] != 0.0)) {
            s.store_exp_ad(281, A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]));
        }

        s.v[478] = if ((s.v[85] * ((s.v[262] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85)))) < (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (!(s.v[464] != 0.0))) && (!(s.v[477] != 0.0))) && (s.v[478] != 0.0)) {
            let assign5300_ad_e4861: A = A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85])), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85])), 0.3333333333333333), 1.0));
            s.store_div_from_scalar_ad(281, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85])), A::offset(A::scale(assign5300_ad_e4861, 0.5), 1.0)), 1.0));
        }

        if (((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (!(s.v[464] != 0.0))) && (!(s.v[477] != 0.0))) && (!(s.v[478] != 0.0))) {
            let assign5310_ad_e4940: A = A::mul(A::offset(A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]), (-230.25850929940458)), A::offset(A::scale(A::offset(A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]), (-230.25850929940458)), 0.3333333333333333), 1.0));
            s.store_scale_ad(281, A::offset(A::mul(A::offset(A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]), (-230.25850929940458)), A::offset(A::scale(assign5310_ad_e4940, 0.5), 1.0)), 1.0), 1e100);
        }

        if (((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (!(s.v[464] != 0.0))) {
            s.store_scale_ad(367, A::add(A::div(A::sub(s.ad_value(359), A::mul(s.ad_value(262), s.ad_value(366))), A::square(s.ad_value(359))), A::div(A::mul(s.ad_value(362), s.ad_value(366)), A::scale(s.ad_value(350), p.p85))), s.v[85]);
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
        if (((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (!(s.v[464] != 0.0))) {
            s.store_mul_ad_lhs(370, A::offset(A::mul(A::sub(s.ad_value(190), s.ad_value(262)), s.ad_value(367)), 1.0), 281);
        }

        if (((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (!(s.v[464] != 0.0))) {
            s.store_scale_ad(363, A::square(s.ad_value(318)), 1.0 / (s.v[310]));
        }

        if (((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (!(s.v[464] != 0.0))) {
            s.store_scale_ad(362, A::ln(A::div_from_scalar(s.v[310], s.ad_value(363))), (s.v[64] / s.v[85]));
        }

        s.v[479] = if (s.v[64] < p.p85) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (!(s.v[464] != 0.0))) && (s.v[479] != 0.0)) {
            s.store_offset_ad(360, A::scale(A::sub(s.ad_value(262), s.ad_value(362)), p.p86), s.v[64]);
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (!(s.v[464] != 0.0))) && (s.v[479] != 0.0)) {
            s.store_sub_from_scalar_ad(350, s.v[64], A::scale(s.ad_value(362), p.p86));
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (!(s.v[464] != 0.0))) && (s.v[479] != 0.0)) {
            s.store_offset_ad(314, A::sub_from_scalar(p.p85, s.ad_value(360)), (-0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (!(s.v[464] != 0.0))) && (s.v[479] != 0.0)) {
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (!(s.v[464] != 0.0))) && (s.v[479] != 0.0)) {
            s.store_ad(315, &{
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (!(s.v[464] != 0.0))) && (s.v[479] != 0.0)) {
            s.store_sqrt_ad(315, A::add(A::square(s.ad_value(314)), s.ad_value(315)));
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (!(s.v[464] != 0.0))) && (s.v[479] != 0.0)) {
            s.store_scale_ad(364, A::offset(A::div(s.ad_value(314), s.ad_value(315)), 1.0), 0.5);
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (!(s.v[464] != 0.0))) && (s.v[479] != 0.0)) {
            s.store_sub_from_scalar_ad(361, p.p85, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5));
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (!(s.v[464] != 0.0))) && (s.v[479] != 0.0)) {
            s.store_offset(314, 361, (((-s.v[64])) + ((-0.01))));
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (!(s.v[464] != 0.0))) && (s.v[479] != 0.0)) {
            s.store_scalar(315, ((4.0 * s.v[64]) * 0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (!(s.v[464] != 0.0))) && (s.v[479] != 0.0)) {
            s.store_ad(315, &{
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (!(s.v[464] != 0.0))) && (s.v[479] != 0.0)) {
            s.store_sqrt_ad(315, A::add(A::square(s.ad_value(314)), s.ad_value(315)));
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (!(s.v[464] != 0.0))) && (s.v[479] != 0.0)) {
            s.store_scale_ad(365, A::offset(A::div(s.ad_value(314), s.ad_value(315)), 1.0), 0.5);
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (!(s.v[464] != 0.0))) && (s.v[479] != 0.0)) {
            s.store_offset_ad(359, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5), s.v[64]);
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (!(s.v[464] != 0.0))) && (s.v[479] != 0.0)) {
            s.store_offset_ad(314, A::sub_from_scalar(p.p85, s.ad_value(350)), (-0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (!(s.v[464] != 0.0))) && (s.v[479] != 0.0)) {
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (!(s.v[464] != 0.0))) && (s.v[479] != 0.0)) {
            s.store_ad(315, &{
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (!(s.v[464] != 0.0))) && (s.v[479] != 0.0)) {
            s.store_sqrt_ad(315, A::add(A::square(s.ad_value(314)), s.ad_value(315)));
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (!(s.v[464] != 0.0))) && (s.v[479] != 0.0)) {
            s.store_sub_from_scalar_ad(350, p.p85, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5));
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (!(s.v[464] != 0.0))) && (s.v[479] != 0.0)) {
            s.store_offset(314, 350, (((-s.v[64])) + ((-0.01))));
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (!(s.v[464] != 0.0))) && (s.v[479] != 0.0)) {
            s.store_scalar(315, ((4.0 * s.v[64]) * 0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (!(s.v[464] != 0.0))) && (s.v[479] != 0.0)) {
            s.store_ad(315, &{
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (!(s.v[464] != 0.0))) && (s.v[479] != 0.0)) {
            s.store_sqrt_ad(315, A::add(A::square(s.ad_value(314)), s.ad_value(315)));
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (!(s.v[464] != 0.0))) && (s.v[479] != 0.0)) {
            s.store_offset_ad(350, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5), s.v[64]);
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (!(s.v[464] != 0.0))) && (s.v[479] != 0.0)) {
            s.store_mul_ad_lhs(366, A::scale(s.ad_value(364), p.p86), 365);
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (!(s.v[464] != 0.0))) && (!(s.v[479] != 0.0))) {
            s.store_scalar(350, s.v[64]);
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (!(s.v[464] != 0.0))) && (!(s.v[479] != 0.0))) {
            s.store_scalar(359, s.v[64]);
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (!(s.v[464] != 0.0))) && (!(s.v[479] != 0.0))) {
            s.store_scalar(366, 0.0);
        }

        s.v[480] = if ((((s.v[85] * ((s.v[262] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85))))) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (!(s.v[464] != 0.0))) && (s.v[480] != 0.0)) {
            s.store_exp_ad(282, A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]));
        }

        s.v[481] = if ((s.v[85] * ((s.v[262] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85)))) < (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (!(s.v[464] != 0.0))) && (!(s.v[480] != 0.0))) && (s.v[481] != 0.0)) {
            let assign5680_ad_e5596: A = A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85])), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85])), 0.3333333333333333), 1.0));
            s.store_div_from_scalar_ad(282, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85])), A::offset(A::scale(assign5680_ad_e5596, 0.5), 1.0)), 1.0));
        }

        if (((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (!(s.v[464] != 0.0))) && (!(s.v[480] != 0.0))) && (!(s.v[481] != 0.0))) {
            let assign5690_ad_e5675: A = A::mul(A::offset(A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]), (-230.25850929940458)), A::offset(A::scale(A::offset(A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]), (-230.25850929940458)), 0.3333333333333333), 1.0));
            s.store_scale_ad(282, A::offset(A::mul(A::offset(A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]), (-230.25850929940458)), A::offset(A::scale(assign5690_ad_e5675, 0.5), 1.0)), 1.0), 1e100);
        }

        if (((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (!(s.v[464] != 0.0))) {
            s.store_scale_ad(367, A::add(A::div(A::sub(s.ad_value(359), A::mul(s.ad_value(262), s.ad_value(366))), A::square(s.ad_value(359))), A::div(A::mul(s.ad_value(362), s.ad_value(366)), A::scale(s.ad_value(350), p.p85))), s.v[85]);
        }

        if (((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (!(s.v[464] != 0.0))) {
            s.store_mul_ad_lhs(371, A::offset(A::mul(A::sub(s.ad_value(190), s.ad_value(262)), s.ad_value(367)), 1.0), 282);
        }

        if (((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (!(s.v[464] != 0.0))) {
            s.store_scale_ad(363, A::square(s.ad_value(318)), 1.0 / (s.v[309]));
        }

        if (((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (!(s.v[464] != 0.0))) {
            s.store_scale_ad(362, A::ln(A::div_from_scalar(s.v[309], s.ad_value(363))), (s.v[63] / s.v[85]));
        }

        s.v[482] = if (s.v[63] < p.p85) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (!(s.v[464] != 0.0))) && (s.v[482] != 0.0)) {
            s.store_offset_ad(360, A::scale(A::sub(s.ad_value(262), s.ad_value(362)), p.p86), s.v[63]);
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (!(s.v[464] != 0.0))) && (s.v[482] != 0.0)) {
            s.store_sub_from_scalar_ad(350, s.v[63], A::scale(s.ad_value(362), p.p86));
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (!(s.v[464] != 0.0))) && (s.v[482] != 0.0)) {
            s.store_offset_ad(314, A::sub_from_scalar(p.p85, s.ad_value(360)), (-0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (!(s.v[464] != 0.0))) && (s.v[482] != 0.0)) {
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (!(s.v[464] != 0.0))) && (s.v[482] != 0.0)) {
            s.store_ad(315, &{
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (!(s.v[464] != 0.0))) && (s.v[482] != 0.0)) {
            s.store_sqrt_ad(315, A::add(A::square(s.ad_value(314)), s.ad_value(315)));
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (!(s.v[464] != 0.0))) && (s.v[482] != 0.0)) {
            s.store_scale_ad(364, A::offset(A::div(s.ad_value(314), s.ad_value(315)), 1.0), 0.5);
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (!(s.v[464] != 0.0))) && (s.v[482] != 0.0)) {
            s.store_sub_from_scalar_ad(361, p.p85, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5));
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (!(s.v[464] != 0.0))) && (s.v[482] != 0.0)) {
            s.store_offset(314, 361, (((-s.v[63])) + ((-0.01))));
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (!(s.v[464] != 0.0))) && (s.v[482] != 0.0)) {
            s.store_scalar(315, ((4.0 * s.v[63]) * 0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (!(s.v[464] != 0.0))) && (s.v[482] != 0.0)) {
            s.store_ad(315, &{
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (!(s.v[464] != 0.0))) && (s.v[482] != 0.0)) {
            s.store_sqrt_ad(315, A::add(A::square(s.ad_value(314)), s.ad_value(315)));
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (!(s.v[464] != 0.0))) && (s.v[482] != 0.0)) {
            s.store_scale_ad(365, A::offset(A::div(s.ad_value(314), s.ad_value(315)), 1.0), 0.5);
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (!(s.v[464] != 0.0))) && (s.v[482] != 0.0)) {
            s.store_offset_ad(359, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5), s.v[63]);
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (!(s.v[464] != 0.0))) && (s.v[482] != 0.0)) {
            s.store_offset_ad(314, A::sub_from_scalar(p.p85, s.ad_value(350)), (-0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (!(s.v[464] != 0.0))) && (s.v[482] != 0.0)) {
            s.store_scalar(315, ((4.0 * p.p85) * 0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (!(s.v[464] != 0.0))) && (s.v[482] != 0.0)) {
            s.store_ad(315, &{
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (!(s.v[464] != 0.0))) && (s.v[482] != 0.0)) {
            s.store_sqrt_ad(315, A::add(A::square(s.ad_value(314)), s.ad_value(315)));
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (!(s.v[464] != 0.0))) && (s.v[482] != 0.0)) {
            s.store_sub_from_scalar_ad(350, p.p85, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5));
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (!(s.v[464] != 0.0))) && (s.v[482] != 0.0)) {
            s.store_offset(314, 350, (((-s.v[63])) + ((-0.01))));
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (!(s.v[464] != 0.0))) && (s.v[482] != 0.0)) {
            s.store_scalar(315, ((4.0 * s.v[63]) * 0.01));
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (!(s.v[464] != 0.0))) && (s.v[482] != 0.0)) {
            s.store_ad(315, &{
                if (s.v[315] > 0.0) {
                    s.ad_value(315)
                } else {
                    A::neg(s.ad_value(315))
                }
            });
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (!(s.v[464] != 0.0))) && (s.v[482] != 0.0)) {
            s.store_sqrt_ad(315, A::add(A::square(s.ad_value(314)), s.ad_value(315)));
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (!(s.v[464] != 0.0))) && (s.v[482] != 0.0)) {
            s.store_offset_ad(350, A::scale(A::add(s.ad_value(314), s.ad_value(315)), 0.5), s.v[63]);
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (!(s.v[464] != 0.0))) && (s.v[482] != 0.0)) {
            s.store_mul_ad_lhs(366, A::scale(s.ad_value(364), p.p86), 365);
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (!(s.v[464] != 0.0))) && (!(s.v[482] != 0.0))) {
            s.store_scalar(350, s.v[63]);
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (!(s.v[464] != 0.0))) && (!(s.v[482] != 0.0))) {
            s.store_scalar(359, s.v[63]);
        }

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (!(s.v[464] != 0.0))) && (!(s.v[482] != 0.0))) {
            s.store_scalar(366, 0.0);
        }

        s.v[483] = if ((((s.v[85] * ((s.v[262] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85))))) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (!(s.v[464] != 0.0))) && (s.v[483] != 0.0)) {
            s.store_exp_ad(283, A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]));
        }

        s.v[484] = if ((s.v[85] * ((s.v[262] / s.v[359]) + ((s.v[362] * (s.v[359] - s.v[350])) / (s.v[350] * p.p85)))) < (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (!(s.v[464] != 0.0))) && (!(s.v[483] != 0.0))) && (s.v[484] != 0.0)) {
            let assign6060_ad_e6331: A = A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85])), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85])), 0.3333333333333333), 1.0));
            s.store_div_from_scalar_ad(283, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85])), A::offset(A::scale(assign6060_ad_e6331, 0.5), 1.0)), 1.0));
        }

        if (((((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (!(s.v[464] != 0.0))) && (!(s.v[483] != 0.0))) && (!(s.v[484] != 0.0))) {
            let assign6070_ad_e6410: A = A::mul(A::offset(A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]), (-230.25850929940458)), A::offset(A::scale(A::offset(A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]), (-230.25850929940458)), 0.3333333333333333), 1.0));
            s.store_scale_ad(283, A::offset(A::mul(A::offset(A::scale(A::add(A::div(s.ad_value(262), s.ad_value(359)), A::div(A::mul(s.ad_value(362), A::sub(s.ad_value(359), s.ad_value(350))), A::scale(s.ad_value(350), p.p85))), s.v[85]), (-230.25850929940458)), A::offset(A::scale(assign6070_ad_e6410, 0.5), 1.0)), 1.0), 1e100);
        }

        if (((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (!(s.v[464] != 0.0))) {
            s.store_scale_ad(367, A::add(A::div(A::sub(s.ad_value(359), A::mul(s.ad_value(262), s.ad_value(366))), A::square(s.ad_value(359))), A::div(A::mul(s.ad_value(362), s.ad_value(366)), A::scale(s.ad_value(350), p.p85))), s.v[85]);
        }

        if (((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (!(s.v[464] != 0.0))) {
            s.store_mul_ad_lhs(372, A::offset(A::mul(A::sub(s.ad_value(190), s.ad_value(262)), s.ad_value(367)), 1.0), 283);
        }

        if ((s.v[418] != 0.0) && (s.v[463] != 0.0)) {
            s.store_offset(370, 370, (-1.0));
        }

        if ((s.v[418] != 0.0) && (s.v[463] != 0.0)) {
            s.store_offset(371, 371, (-1.0));
        }

        if ((s.v[418] != 0.0) && (s.v[463] != 0.0)) {
            s.store_offset(372, 372, (-1.0));
        }

        if ((s.v[418] != 0.0) && (s.v[463] != 0.0)) {
            s.store_div_from_scalar(429, 1.0, 430);
        }

        s.v[485] = if (s.v[190] > 0.0) { 1.0 } else { 0.0 };

        if (((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (s.v[485] != 0.0)) {
            s.store_scale_ad(431, A::ln(A::add(A::offset(s.ad_value(429), 2.0), A::sqrt(A::mul(A::offset(s.ad_value(429), 1.0), A::offset(s.ad_value(429), 3.0))))), (s.v[84] * 2.0));
        }

        if (((s.v[418] != 0.0) && (s.v[463] != 0.0)) && (!(s.v[485] != 0.0))) {
            s.store_sub_ad_lhs(431, A::scale(A::ln(A::add(A::offset(A::scale(s.ad_value(430), 2.0), 1.0), A::sqrt(A::mul(A::offset(s.ad_value(430), 1.0), A::offset(A::scale(s.ad_value(430), 3.0), 1.0))))), (s.v[84] * 2.0)), 190);
        }

        if ((s.v[418] != 0.0) && (s.v[463] != 0.0)) {
            s.store_sub(432, 264, 431);
        }

        if ((s.v[418] != 0.0) && (s.v[463] != 0.0)) {
            s.store_scale_ad(433, A::sub(A::add(s.ad_value(190), s.ad_value(432)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(190), s.ad_value(432)), A::sub(s.ad_value(190), s.ad_value(432))), ((4.0 * s.v[84]) * s.v[84])))), 0.5);
        }

        if ((s.v[418] != 0.0) && (s.v[463] != 0.0)) {
            s.store_scale_ad(434, A::sub(A::add(s.ad_value(190), s.ad_value(267)), A::sqrt(A::add(A::mul(A::sub(s.ad_value(190), s.ad_value(267)), A::sub(s.ad_value(190), s.ad_value(267))), A::mul(A::scale(s.ad_value(82), 4.0), s.ad_value(82))))), 0.5);
        }

        if ((s.v[418] != 0.0) && (s.v[463] != 0.0)) {
            s.store_scale_ad(435, A::sub(s.ad_value(190), A::sqrt(A::offset(A::mul(s.ad_value(190), s.ad_value(190)), ((4.0 * 1e-6) * 1e-6)))), 0.5);
        }

        if ((s.v[418] != 0.0) && (!(s.v[463] != 0.0))) {
            s.store_scalar(370, 0.0);
        }

        if ((s.v[418] != 0.0) && (!(s.v[463] != 0.0))) {
            s.store_scalar(371, 0.0);
        }

        if ((s.v[418] != 0.0) && (!(s.v[463] != 0.0))) {
            s.store_scalar(372, 0.0);
        }

        if ((s.v[418] != 0.0) && (!(s.v[463] != 0.0))) {
            s.store_scalar(431, 0.0);
        }

        if ((s.v[418] != 0.0) && (!(s.v[463] != 0.0))) {
            s.store_scalar(428, 0.0);
        }

        if ((s.v[418] != 0.0) && (!(s.v[463] != 0.0))) {
            s.store_scalar(430, 0.0);
        }

        if ((s.v[418] != 0.0) && (!(s.v[463] != 0.0))) {
            s.store_scalar(433, 0.0);
        }

        if ((s.v[418] != 0.0) && (!(s.v[463] != 0.0))) {
            s.store_scalar(434, 0.0);
        }

        if ((s.v[418] != 0.0) && (!(s.v[463] != 0.0))) {
            s.store_scalar(435, 0.0);
        }

        s.v[486] = if (s.v[256] == 0.0) { 1.0 } else { 0.0 };

        if ((s.v[418] != 0.0) && (s.v[486] != 0.0)) {
            s.store_scalar(268, 0.0);
        }

        if ((s.v[418] != 0.0) && (s.v[486] != 0.0)) {
            s.store_scalar(291, 0.0);
        }

        if ((s.v[418] != 0.0) && (s.v[486] != 0.0)) {
            s.store_scalar(269, 0.0);
        }

        s.v[487] = if (s.v[122] == 0.5) { 1.0 } else { 0.0 };

        if (((s.v[418] != 0.0) && (!(s.v[486] != 0.0))) && (s.v[487] != 0.0)) {
            s.store_sqrt_ad(436, A::sub_from_scalar(1.0, A::mul(s.ad_value(428), s.ad_value(119))));
        }

        if (((s.v[418] != 0.0) && (!(s.v[486] != 0.0))) && (!(s.v[487] != 0.0))) {
            s.store_powf_ad(436, A::sub_from_scalar(1.0, A::mul(s.ad_value(428), s.ad_value(119))), s.v[122]);
        }

        if ((s.v[418] != 0.0) && (!(s.v[486] != 0.0))) {
            s.store_add_ad(269, A::mul(s.ad_value(131), A::sub_from_scalar(1.0, s.ad_value(436))), A::mul(s.ad_value(134), A::sub(s.ad_value(190), s.ad_value(428))));
        }

        if ((s.v[418] != 0.0) && (!(s.v[486] != 0.0))) {
            s.store_mul(437, 101, 370);
        }

        s.v[488] = if ((s.v[20] == 0.0) && (s.v[23] == 0.0)) { 1.0 } else { 0.0 };

        if (((s.v[418] != 0.0) && (!(s.v[486] != 0.0))) && (s.v[488] != 0.0)) {
            s.store_scalar(439, 0.0);
        }

        if (((s.v[418] != 0.0) && (!(s.v[486] != 0.0))) && (s.v[488] != 0.0)) {
            s.store_scalar(442, 0.0);
        }

        if (((s.v[418] != 0.0) && (!(s.v[486] != 0.0))) && (s.v[488] != 0.0)) {
            s.store_scalar(443, 0.0);
        }

        if (((s.v[418] != 0.0) && (!(s.v[486] != 0.0))) && (s.v[488] != 0.0)) {
            s.store_scalar(444, 0.0);
        }

        if (((s.v[418] != 0.0) && (!(s.v[486] != 0.0))) && (s.v[488] != 0.0)) {
            s.store_scalar(438, 0.0);
        }

        if (((s.v[418] != 0.0) && (!(s.v[486] != 0.0))) && (!(s.v[488] != 0.0))) {
            s.store_sub(439, 107, 433);
        }

        if (((s.v[418] != 0.0) && (!(s.v[486] != 0.0))) && (!(s.v[488] != 0.0))) {
            s.store_sub_from_scalar_ad(440, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(431), s.ad_value(439)))));
        }

        s.v[489] = if (s.v[9] == 0.5) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (!(s.v[486] != 0.0))) && (!(s.v[488] != 0.0))) && (s.v[489] != 0.0)) {
            s.store_scalar(441, 0.0);
        }

        if ((((s.v[418] != 0.0) && (!(s.v[486] != 0.0))) && (!(s.v[488] != 0.0))) && (!(s.v[489] != 0.0))) {
            s.store_scale_ad(441, A::add(A::div(A::mul(A::square(s.ad_value(440)), A::ln(s.ad_value(440))), A::sub_from_scalar(1.0, s.ad_value(440))), s.ad_value(440)), (1.0 - (2.0 * s.v[9])));
        }

        if (((s.v[418] != 0.0) && (!(s.v[486] != 0.0))) && (!(s.v[488] != 0.0))) {
            s.store_add(442, 440, 441);
        }

        s.v[490] = if (s.v[9] == 0.5) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (!(s.v[486] != 0.0))) && (!(s.v[488] != 0.0))) && (s.v[490] != 0.0)) {
            s.store_sqrt_ad(436, A::scale(s.ad_value(439), s.v[143]));
        }

        if ((((s.v[418] != 0.0) && (!(s.v[486] != 0.0))) && (!(s.v[488] != 0.0))) && (!(s.v[490] != 0.0))) {
            s.store_powf_ad(436, A::scale(s.ad_value(439), s.v[143]), s.v[9]);
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
        if (((s.v[418] != 0.0) && (!(s.v[486] != 0.0))) && (!(s.v[488] != 0.0))) {
            s.store_scale(443, 436, s.v[137]);
        }

        if (((s.v[418] != 0.0) && (!(s.v[486] != 0.0))) && (!(s.v[488] != 0.0))) {
            s.store_mul_ad_rhs(444, 98, A::mul(A::offset(s.ad_value(430), (-1.0)), s.ad_value(443)));
        }

        if (((s.v[418] != 0.0) && (!(s.v[486] != 0.0))) && (!(s.v[488] != 0.0))) {
            s.store_scaled_mul(438, 444, 442, s.v[20]);
        }

        s.v[491] = if (s.v[23] == 0.0) { 1.0 } else { 0.0 };

        if (((s.v[418] != 0.0) && (!(s.v[486] != 0.0))) && (s.v[491] != 0.0)) {
            s.store_scalar(445, 0.0);
        }

        if (((s.v[418] != 0.0) && (!(s.v[486] != 0.0))) && (!(s.v[491] != 0.0))) {
            s.store_scale_ad(446, A::div(A::scale(s.ad_value(443), s.v[122]), s.ad_value(439)), s.v[152]);
        }

        if (((s.v[418] != 0.0) && (!(s.v[486] != 0.0))) && (!(s.v[491] != 0.0))) {
            s.store_div_from_scalar(447, (0.666666666666667 * s.v[149]), 446);
        }

        if (((s.v[418] != 0.0) && (!(s.v[486] != 0.0))) && (!(s.v[491] != 0.0))) {
            s.store_square(448, 447);
        }

        if (((s.v[418] != 0.0) && (!(s.v[486] != 0.0))) && (!(s.v[491] != 0.0))) {
            s.store_sqrt_ad(449, A::div(A::square(s.ad_value(448)), A::offset(A::square(s.ad_value(448)), 1.0)));
        }

        if (((s.v[418] != 0.0) && (!(s.v[486] != 0.0))) && (!(s.v[491] != 0.0))) {
            s.store_sqrt_ad(450, A::abs(s.ad_value(449)));
        }

        if (((s.v[418] != 0.0) && (!(s.v[486] != 0.0))) && (!(s.v[491] != 0.0))) {
            s.store_mul(451, 449, 450);
        }

        s.v[492] = if (((-s.v[9]) * s.v[125]) == (-1.0)) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (!(s.v[486] != 0.0))) && (!(s.v[491] != 0.0))) && (s.v[492] != 0.0)) {
            s.store_div_from_scalar_ad(452, 1.0, A::offset(A::mul(s.ad_value(446), s.ad_value(451)), 1.0));
        }

        if ((((s.v[418] != 0.0) && (!(s.v[486] != 0.0))) && (!(s.v[491] != 0.0))) && (!(s.v[492] != 0.0))) {
            s.store_powf_ad(452, A::offset(A::mul(s.ad_value(446), s.ad_value(451)), 1.0), ((-s.v[9]) * s.v[125]));
        }

        if (((s.v[418] != 0.0) && (!(s.v[486] != 0.0))) && (!(s.v[491] != 0.0))) {
            s.store_div_ad(453, A::mul(s.ad_value(442), s.ad_value(452)), A::add(s.ad_value(442), s.ad_value(452)));
        }

        if (((s.v[418] != 0.0) && (!(s.v[486] != 0.0))) && (!(s.v[491] != 0.0))) {
            s.store_sqrt_ad(454, A::scale(A::div(s.ad_value(446), s.ad_value(450)), 0.375));
        }

        if (((s.v[418] != 0.0) && (!(s.v[486] != 0.0))) && (!(s.v[491] != 0.0))) {
            s.store_sub_ad_lhs(455, A::scale(A::mul(s.ad_value(447), s.ad_value(450)), 2.0), 449);
        }

        if (((s.v[418] != 0.0) && (!(s.v[486] != 0.0))) && (!(s.v[491] != 0.0))) {
            s.store_add_ad(456, A::sub(A::mul(A::scale(s.ad_value(447), s.v[149]), s.ad_value(450)), A::scale(s.ad_value(449), s.v[149])), A::scale(A::mul(s.ad_value(446), s.ad_value(451)), 0.5));
        }

        if (((s.v[418] != 0.0) && (!(s.v[486] != 0.0))) && (!(s.v[491] != 0.0))) {
            s.store_mul_ad_lhs(457, A::offset(s.ad_value(455), (-1.0)), 454);
        }

        if (((s.v[418] != 0.0) && (!(s.v[486] != 0.0))) && (!(s.v[491] != 0.0))) {
            s.store_square(419, 457);
        }

        s.v[493] = if (s.v[457] > 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (!(s.v[486] != 0.0))) && (!(s.v[491] != 0.0))) && (s.v[493] != 0.0)) {
            s.store_div_from_scalar_ad(420, 1.0, A::offset(A::scale(s.ad_value(457), s.v[86]), 1.0));
        }

        if ((((s.v[418] != 0.0) && (!(s.v[486] != 0.0))) && (!(s.v[491] != 0.0))) && (!(s.v[493] != 0.0))) {
            s.store_div_from_scalar_ad(420, 1.0, A::sub_from_scalar(1.0, A::scale(s.ad_value(457), s.v[86])));
        }

        s.v[494] = if (((-s.v[419]) + s.v[456]) > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (!(s.v[486] != 0.0))) && (!(s.v[491] != 0.0))) && (s.v[494] != 0.0)) {
            s.store_exp_ad(436, A::sub(s.ad_value(456), s.ad_value(419)));
        }

        if ((((s.v[418] != 0.0) && (!(s.v[486] != 0.0))) && (!(s.v[491] != 0.0))) && (!(s.v[494] != 0.0))) {
            s.store_div_from_scalar_ad(436, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(456), s.ad_value(419))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(456), s.ad_value(419))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(456), s.ad_value(419))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if (((s.v[418] != 0.0) && (!(s.v[486] != 0.0))) && (!(s.v[491] != 0.0))) {
            s.store_mul_ad_lhs(421, A::add(A::add(A::scale(s.ad_value(420), 0.29214664), A::scale(A::square(s.ad_value(420)), s.v[87])), A::scale(A::mul(A::square(s.ad_value(420)), s.ad_value(420)), s.v[88])), 436);
        }

        s.v[495] = if (s.v[457] > 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (!(s.v[486] != 0.0))) && (!(s.v[491] != 0.0))) && (s.v[495] != 0.0)) {
            s.copy_ad(458, 421);
        }

        s.v[496] = if (s.v[456] > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((s.v[418] != 0.0) && (!(s.v[486] != 0.0))) && (!(s.v[491] != 0.0))) && (!(s.v[495] != 0.0))) && (s.v[496] != 0.0)) {
            s.store_exp(436, 456);
        }

        if (((((s.v[418] != 0.0) && (!(s.v[486] != 0.0))) && (!(s.v[491] != 0.0))) && (!(s.v[495] != 0.0))) && (!(s.v[496] != 0.0))) {
            s.store_div_from_scalar_ad(436, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(456)), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(456)), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), s.ad_value(456)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if ((((s.v[418] != 0.0) && (!(s.v[486] != 0.0))) && (!(s.v[491] != 0.0))) && (!(s.v[495] != 0.0))) {
            s.store_sub_ad_lhs(458, A::scale(s.ad_value(436), 2.0), 421);
        }

        if (((s.v[418] != 0.0) && (!(s.v[486] != 0.0))) && (!(s.v[491] != 0.0))) {
            s.store_scale_ad(459, A::div(A::scale(s.ad_value(458), s.v[149]), s.ad_value(454)), (1.772453850905516 * 0.5));
        }

        if (((s.v[418] != 0.0) && (!(s.v[486] != 0.0))) && (!(s.v[491] != 0.0))) {
            s.store_scale_ad(445, A::mul(A::mul(s.ad_value(444), s.ad_value(459)), s.ad_value(453)), s.v[23]);
        }

        s.v[497] = if (s.v[29] == 0.0) { 1.0 } else { 0.0 };

        if (((s.v[418] != 0.0) && (!(s.v[486] != 0.0))) && (s.v[497] != 0.0)) {
            s.store_scalar(460, 0.0);
        }

        s.v[498] = if (s.v[9] == 0.5) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (!(s.v[486] != 0.0))) && (!(s.v[497] != 0.0))) && (s.v[498] != 0.0)) {
            s.store_sqrt_ad(436, A::scale(A::sub_from_scalar(s.v[6], s.ad_value(434)), s.v[143]));
        }

        if ((((s.v[418] != 0.0) && (!(s.v[486] != 0.0))) && (!(s.v[497] != 0.0))) && (!(s.v[498] != 0.0))) {
            s.store_powf_ad(436, A::scale(A::sub_from_scalar(s.v[6], s.ad_value(434)), s.v[143]), s.v[9]);
        }

        if (((s.v[418] != 0.0) && (!(s.v[486] != 0.0))) && (!(s.v[497] != 0.0))) {
            s.store_scale_ad(461, A::div(A::scale(A::sub_from_scalar(s.v[6], s.ad_value(434)), s.v[140]), s.ad_value(436)), s.v[125]);
        }

        s.v[499] = if (((((-s.v[155]) / s.v[461])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (!(s.v[486] != 0.0))) && (!(s.v[497] != 0.0))) && (s.v[499] != 0.0)) {
            s.store_exp_ad(436, A::div(A::neg(s.ad_value(155)), s.ad_value(461)));
        }

        s.v[500] = if (((-s.v[155]) / s.v[461]) < (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((s.v[418] != 0.0) && (!(s.v[486] != 0.0))) && (!(s.v[497] != 0.0))) && (!(s.v[499] != 0.0))) && (s.v[500] != 0.0)) {
            let assign6980_ad_e7644: A = A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(155)), s.ad_value(461))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(155)), s.ad_value(461))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(155)), s.ad_value(461))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
            s.store_div_from_scalar_ad(436, 1e-100, assign6980_ad_e7644);
        }

        if (((((s.v[418] != 0.0) && (!(s.v[486] != 0.0))) && (!(s.v[497] != 0.0))) && (!(s.v[499] != 0.0))) && (!(s.v[500] != 0.0))) {
            let assign6990_ad_e7692: A = A::scale(A::offset(A::mul(A::offset(A::div(A::neg(s.ad_value(155)), s.ad_value(461)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::div(A::neg(s.ad_value(155)), s.ad_value(461)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::div(A::neg(s.ad_value(155)), s.ad_value(461)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
            s.store_ad(436, &assign6990_ad_e7692);
        }

        if (((s.v[418] != 0.0) && (!(s.v[486] != 0.0))) && (!(s.v[497] != 0.0))) {
            s.store_scale_ad(460, A::mul(A::mul(A::mul(s.ad_value(190), s.ad_value(461)), s.ad_value(461)), s.ad_value(436)), s.v[29]);
        }

        s.v[501] = if ((s.v[38] > 1000000.0) || (p.p80 == 0.0)) { 1.0 } else { 0.0 };

        if (((s.v[418] != 0.0) && (!(s.v[486] != 0.0))) && (s.v[501] != 0.0)) {
            s.store_scalar(462, 1.0);
        }

        s.v[502] = if (s.v[435] > ((-s.v[158]) * s.v[38])) { 1.0 } else { 0.0 };

        s.v[503] = if (s.v[41] == 4.0) { 1.0 } else { 0.0 };

        if (((((s.v[418] != 0.0) && (!(s.v[486] != 0.0))) && (!(s.v[501] != 0.0))) && (s.v[502] != 0.0)) && (s.v[503] != 0.0)) {
            s.store_mul_ad(436, A::mul(A::mul(A::abs(A::mul(s.ad_value(435), s.ad_value(162))), A::abs(A::mul(s.ad_value(435), s.ad_value(162)))), A::abs(A::mul(s.ad_value(435), s.ad_value(162)))), A::abs(A::mul(s.ad_value(435), s.ad_value(162))));
        }

        if (((((s.v[418] != 0.0) && (!(s.v[486] != 0.0))) && (!(s.v[501] != 0.0))) && (s.v[502] != 0.0)) && (!(s.v[503] != 0.0))) {
            s.store_powf_ad(436, A::abs(A::mul(s.ad_value(435), s.ad_value(162))), s.v[41]);
        }

        if ((((s.v[418] != 0.0) && (!(s.v[486] != 0.0))) && (!(s.v[501] != 0.0))) && (s.v[502] != 0.0)) {
            s.store_div_from_scalar_ad(462, 1.0, A::sub_from_scalar(1.0, s.ad_value(436)));
        }

        if ((((s.v[418] != 0.0) && (!(s.v[486] != 0.0))) && (!(s.v[501] != 0.0))) && (!(s.v[502] != 0.0))) {
            s.store_offset_ad(462, A::mul(A::add(s.ad_value(435), A::scale(s.ad_value(38), s.v[158])), s.ad_value(165)), s.v[159]);
        }

        if ((s.v[418] != 0.0) && (!(s.v[486] != 0.0))) {
            s.store_mul_ad_lhs(268, A::add(A::add(A::add(s.ad_value(437), s.ad_value(438)), s.ad_value(445)), s.ad_value(460)), 462);
        }

        if ((s.v[418] != 0.0) && (!(s.v[486] != 0.0))) {
            s.store_mul_ad_lhs(291, A::add(A::add(s.ad_value(438), s.ad_value(445)), s.ad_value(460)), 462);
        }

        s.v[504] = if (s.v[257] == 0.0) { 1.0 } else { 0.0 };

        if ((s.v[418] != 0.0) && (s.v[504] != 0.0)) {
            s.store_scalar(270, 0.0);
        }

        if ((s.v[418] != 0.0) && (s.v[504] != 0.0)) {
            s.store_scalar(292, 0.0);
        }

        if ((s.v[418] != 0.0) && (s.v[504] != 0.0)) {
            s.store_scalar(271, 0.0);
        }

        s.v[505] = if (s.v[123] == 0.5) { 1.0 } else { 0.0 };

        if (((s.v[418] != 0.0) && (!(s.v[504] != 0.0))) && (s.v[505] != 0.0)) {
            s.store_sqrt_ad(436, A::sub_from_scalar(1.0, A::mul(s.ad_value(428), s.ad_value(120))));
        }

        if (((s.v[418] != 0.0) && (!(s.v[504] != 0.0))) && (!(s.v[505] != 0.0))) {
            s.store_powf_ad(436, A::sub_from_scalar(1.0, A::mul(s.ad_value(428), s.ad_value(120))), s.v[123]);
        }

        if ((s.v[418] != 0.0) && (!(s.v[504] != 0.0))) {
            s.store_add_ad(271, A::mul(s.ad_value(132), A::sub_from_scalar(1.0, s.ad_value(436))), A::mul(s.ad_value(135), A::sub(s.ad_value(190), s.ad_value(428))));
        }

        if ((s.v[418] != 0.0) && (!(s.v[504] != 0.0))) {
            s.store_mul(437, 102, 371);
        }

        s.v[506] = if ((s.v[21] == 0.0) && (s.v[24] == 0.0)) { 1.0 } else { 0.0 };

        if (((s.v[418] != 0.0) && (!(s.v[504] != 0.0))) && (s.v[506] != 0.0)) {
            s.store_scalar(439, 0.0);
        }

        if (((s.v[418] != 0.0) && (!(s.v[504] != 0.0))) && (s.v[506] != 0.0)) {
            s.store_scalar(442, 0.0);
        }

        if (((s.v[418] != 0.0) && (!(s.v[504] != 0.0))) && (s.v[506] != 0.0)) {
            s.store_scalar(443, 0.0);
        }

        if (((s.v[418] != 0.0) && (!(s.v[504] != 0.0))) && (s.v[506] != 0.0)) {
            s.store_scalar(444, 0.0);
        }

        if (((s.v[418] != 0.0) && (!(s.v[504] != 0.0))) && (s.v[506] != 0.0)) {
            s.store_scalar(438, 0.0);
        }

        if (((s.v[418] != 0.0) && (!(s.v[504] != 0.0))) && (!(s.v[506] != 0.0))) {
            s.store_sub(439, 108, 433);
        }

        if (((s.v[418] != 0.0) && (!(s.v[504] != 0.0))) && (!(s.v[506] != 0.0))) {
            s.store_sub_from_scalar_ad(440, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(431), s.ad_value(439)))));
        }

        s.v[507] = if (s.v[10] == 0.5) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (!(s.v[504] != 0.0))) && (!(s.v[506] != 0.0))) && (s.v[507] != 0.0)) {
            s.store_scalar(441, 0.0);
        }

        if ((((s.v[418] != 0.0) && (!(s.v[504] != 0.0))) && (!(s.v[506] != 0.0))) && (!(s.v[507] != 0.0))) {
            s.store_scale_ad(441, A::add(A::div(A::mul(A::square(s.ad_value(440)), A::ln(s.ad_value(440))), A::sub_from_scalar(1.0, s.ad_value(440))), s.ad_value(440)), (1.0 - (2.0 * s.v[10])));
        }

        if (((s.v[418] != 0.0) && (!(s.v[504] != 0.0))) && (!(s.v[506] != 0.0))) {
            s.store_add(442, 440, 441);
        }

        s.v[508] = if (s.v[10] == 0.5) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (!(s.v[504] != 0.0))) && (!(s.v[506] != 0.0))) && (s.v[508] != 0.0)) {
            s.store_sqrt_ad(436, A::scale(s.ad_value(439), s.v[144]));
        }

        if ((((s.v[418] != 0.0) && (!(s.v[504] != 0.0))) && (!(s.v[506] != 0.0))) && (!(s.v[508] != 0.0))) {
            s.store_powf_ad(436, A::scale(s.ad_value(439), s.v[144]), s.v[10]);
        }

        if (((s.v[418] != 0.0) && (!(s.v[504] != 0.0))) && (!(s.v[506] != 0.0))) {
            s.store_scale(443, 436, s.v[138]);
        }

        if (((s.v[418] != 0.0) && (!(s.v[504] != 0.0))) && (!(s.v[506] != 0.0))) {
            s.store_mul_ad_rhs(444, 99, A::mul(A::offset(s.ad_value(430), (-1.0)), s.ad_value(443)));
        }

        if (((s.v[418] != 0.0) && (!(s.v[504] != 0.0))) && (!(s.v[506] != 0.0))) {
            s.store_scaled_mul(438, 444, 442, s.v[21]);
        }

        s.v[509] = if (s.v[24] == 0.0) { 1.0 } else { 0.0 };

        if (((s.v[418] != 0.0) && (!(s.v[504] != 0.0))) && (s.v[509] != 0.0)) {
            s.store_scalar(445, 0.0);
        }

        if (((s.v[418] != 0.0) && (!(s.v[504] != 0.0))) && (!(s.v[509] != 0.0))) {
            s.store_scale_ad(446, A::div(A::scale(s.ad_value(443), s.v[123]), s.ad_value(439)), s.v[153]);
        }

        if (((s.v[418] != 0.0) && (!(s.v[504] != 0.0))) && (!(s.v[509] != 0.0))) {
            s.store_div_from_scalar(447, (0.666666666666667 * s.v[150]), 446);
        }

        if (((s.v[418] != 0.0) && (!(s.v[504] != 0.0))) && (!(s.v[509] != 0.0))) {
            s.store_square(448, 447);
        }

        if (((s.v[418] != 0.0) && (!(s.v[504] != 0.0))) && (!(s.v[509] != 0.0))) {
            s.store_sqrt_ad(449, A::div(A::square(s.ad_value(448)), A::offset(A::square(s.ad_value(448)), 1.0)));
        }

        if (((s.v[418] != 0.0) && (!(s.v[504] != 0.0))) && (!(s.v[509] != 0.0))) {
            s.store_sqrt_ad(450, A::abs(s.ad_value(449)));
        }

        if (((s.v[418] != 0.0) && (!(s.v[504] != 0.0))) && (!(s.v[509] != 0.0))) {
            s.store_mul(451, 449, 450);
        }

        s.v[510] = if (((-s.v[10]) * s.v[126]) == (-1.0)) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (!(s.v[504] != 0.0))) && (!(s.v[509] != 0.0))) && (s.v[510] != 0.0)) {
            s.store_div_from_scalar_ad(452, 1.0, A::offset(A::mul(s.ad_value(446), s.ad_value(451)), 1.0));
        }

        if ((((s.v[418] != 0.0) && (!(s.v[504] != 0.0))) && (!(s.v[509] != 0.0))) && (!(s.v[510] != 0.0))) {
            s.store_powf_ad(452, A::offset(A::mul(s.ad_value(446), s.ad_value(451)), 1.0), ((-s.v[10]) * s.v[126]));
        }

        if (((s.v[418] != 0.0) && (!(s.v[504] != 0.0))) && (!(s.v[509] != 0.0))) {
            s.store_div_ad(453, A::mul(s.ad_value(442), s.ad_value(452)), A::add(s.ad_value(442), s.ad_value(452)));
        }

        if (((s.v[418] != 0.0) && (!(s.v[504] != 0.0))) && (!(s.v[509] != 0.0))) {
            s.store_sqrt_ad(454, A::scale(A::div(s.ad_value(446), s.ad_value(450)), 0.375));
        }

        if (((s.v[418] != 0.0) && (!(s.v[504] != 0.0))) && (!(s.v[509] != 0.0))) {
            s.store_sub_ad_lhs(455, A::scale(A::mul(s.ad_value(447), s.ad_value(450)), 2.0), 449);
        }

        if (((s.v[418] != 0.0) && (!(s.v[504] != 0.0))) && (!(s.v[509] != 0.0))) {
            s.store_add_ad(456, A::sub(A::mul(A::scale(s.ad_value(447), s.v[150]), s.ad_value(450)), A::scale(s.ad_value(449), s.v[150])), A::scale(A::mul(s.ad_value(446), s.ad_value(451)), 0.5));
        }

        if (((s.v[418] != 0.0) && (!(s.v[504] != 0.0))) && (!(s.v[509] != 0.0))) {
            s.store_mul_ad_lhs(457, A::offset(s.ad_value(455), (-1.0)), 454);
        }

        if (((s.v[418] != 0.0) && (!(s.v[504] != 0.0))) && (!(s.v[509] != 0.0))) {
            s.store_square(419, 457);
        }

        s.v[511] = if (s.v[457] > 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (!(s.v[504] != 0.0))) && (!(s.v[509] != 0.0))) && (s.v[511] != 0.0)) {
            s.store_div_from_scalar_ad(420, 1.0, A::offset(A::scale(s.ad_value(457), s.v[86]), 1.0));
        }

        if ((((s.v[418] != 0.0) && (!(s.v[504] != 0.0))) && (!(s.v[509] != 0.0))) && (!(s.v[511] != 0.0))) {
            s.store_div_from_scalar_ad(420, 1.0, A::sub_from_scalar(1.0, A::scale(s.ad_value(457), s.v[86])));
        }

        s.v[512] = if (((-s.v[419]) + s.v[456]) > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (!(s.v[504] != 0.0))) && (!(s.v[509] != 0.0))) && (s.v[512] != 0.0)) {
            s.store_exp_ad(436, A::sub(s.ad_value(456), s.ad_value(419)));
        }

        if ((((s.v[418] != 0.0) && (!(s.v[504] != 0.0))) && (!(s.v[509] != 0.0))) && (!(s.v[512] != 0.0))) {
            s.store_div_from_scalar_ad(436, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(456), s.ad_value(419))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(456), s.ad_value(419))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(456), s.ad_value(419))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if (((s.v[418] != 0.0) && (!(s.v[504] != 0.0))) && (!(s.v[509] != 0.0))) {
            s.store_mul_ad_lhs(421, A::add(A::add(A::scale(s.ad_value(420), 0.29214664), A::scale(A::square(s.ad_value(420)), s.v[87])), A::scale(A::mul(A::square(s.ad_value(420)), s.ad_value(420)), s.v[88])), 436);
        }

        s.v[513] = if (s.v[457] > 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (!(s.v[504] != 0.0))) && (!(s.v[509] != 0.0))) && (s.v[513] != 0.0)) {
            s.copy_ad(458, 421);
        }

        s.v[514] = if (s.v[456] > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((s.v[418] != 0.0) && (!(s.v[504] != 0.0))) && (!(s.v[509] != 0.0))) && (!(s.v[513] != 0.0))) && (s.v[514] != 0.0)) {
            s.store_exp(436, 456);
        }

        if (((((s.v[418] != 0.0) && (!(s.v[504] != 0.0))) && (!(s.v[509] != 0.0))) && (!(s.v[513] != 0.0))) && (!(s.v[514] != 0.0))) {
            s.store_div_from_scalar_ad(436, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(456)), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(456)), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), s.ad_value(456)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if ((((s.v[418] != 0.0) && (!(s.v[504] != 0.0))) && (!(s.v[509] != 0.0))) && (!(s.v[513] != 0.0))) {
            s.store_sub_ad_lhs(458, A::scale(s.ad_value(436), 2.0), 421);
        }

        if (((s.v[418] != 0.0) && (!(s.v[504] != 0.0))) && (!(s.v[509] != 0.0))) {
            s.store_scale_ad(459, A::div(A::scale(s.ad_value(458), s.v[150]), s.ad_value(454)), (1.772453850905516 * 0.5));
        }

        if (((s.v[418] != 0.0) && (!(s.v[504] != 0.0))) && (!(s.v[509] != 0.0))) {
            s.store_scale_ad(445, A::mul(A::mul(s.ad_value(444), s.ad_value(459)), s.ad_value(453)), s.v[24]);
        }

        s.v[515] = if (s.v[30] == 0.0) { 1.0 } else { 0.0 };

        if (((s.v[418] != 0.0) && (!(s.v[504] != 0.0))) && (s.v[515] != 0.0)) {
            s.store_scalar(460, 0.0);
        }

        s.v[516] = if (s.v[10] == 0.5) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (!(s.v[504] != 0.0))) && (!(s.v[515] != 0.0))) && (s.v[516] != 0.0)) {
            s.store_sqrt_ad(436, A::scale(A::sub_from_scalar(s.v[7], s.ad_value(434)), s.v[144]));
        }

        if ((((s.v[418] != 0.0) && (!(s.v[504] != 0.0))) && (!(s.v[515] != 0.0))) && (!(s.v[516] != 0.0))) {
            s.store_powf_ad(436, A::scale(A::sub_from_scalar(s.v[7], s.ad_value(434)), s.v[144]), s.v[10]);
        }

        if (((s.v[418] != 0.0) && (!(s.v[504] != 0.0))) && (!(s.v[515] != 0.0))) {
            s.store_scale_ad(461, A::div(A::scale(A::sub_from_scalar(s.v[7], s.ad_value(434)), s.v[141]), s.ad_value(436)), s.v[126]);
        }

        s.v[517] = if (((((-s.v[156]) / s.v[461])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((((s.v[418] != 0.0) && (!(s.v[504] != 0.0))) && (!(s.v[515] != 0.0))) && (s.v[517] != 0.0)) {
            s.store_exp_ad(436, A::div(A::neg(s.ad_value(156)), s.ad_value(461)));
        }

        s.v[518] = if (((-s.v[156]) / s.v[461]) < (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((s.v[418] != 0.0) && (!(s.v[504] != 0.0))) && (!(s.v[515] != 0.0))) && (!(s.v[517] != 0.0))) && (s.v[518] != 0.0)) {
            let assign7790_ad_e8800: A = A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(156)), s.ad_value(461))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(156)), s.ad_value(461))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(156)), s.ad_value(461))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
            s.store_div_from_scalar_ad(436, 1e-100, assign7790_ad_e8800);
        }

        if (((((s.v[418] != 0.0) && (!(s.v[504] != 0.0))) && (!(s.v[515] != 0.0))) && (!(s.v[517] != 0.0))) && (!(s.v[518] != 0.0))) {
            let assign7800_ad_e8848: A = A::scale(A::offset(A::mul(A::offset(A::div(A::neg(s.ad_value(156)), s.ad_value(461)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::div(A::neg(s.ad_value(156)), s.ad_value(461)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::div(A::neg(s.ad_value(156)), s.ad_value(461)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
            s.store_ad(436, &assign7800_ad_e8848);
        }

        if (((s.v[418] != 0.0) && (!(s.v[504] != 0.0))) && (!(s.v[515] != 0.0))) {
            s.store_scale_ad(460, A::mul(A::mul(A::mul(s.ad_value(190), s.ad_value(461)), s.ad_value(461)), s.ad_value(436)), s.v[30]);
        }

        s.v[519] = if ((s.v[39] > 1000000.0) || (p.p80 == 0.0)) { 1.0 } else { 0.0 };

        if (((s.v[418] != 0.0) && (!(s.v[504] != 0.0))) && (s.v[519] != 0.0)) {
            s.store_scalar(462, 1.0);
        }

        s.v[520] = if (s.v[435] > ((-s.v[158]) * s.v[39])) { 1.0 } else { 0.0 };

        s.v[521] = if (s.v[42] == 4.0) { 1.0 } else { 0.0 };

        if (((((s.v[418] != 0.0) && (!(s.v[504] != 0.0))) && (!(s.v[519] != 0.0))) && (s.v[520] != 0.0)) && (s.v[521] != 0.0)) {
            s.store_mul_ad(436, A::mul(A::mul(A::abs(A::mul(s.ad_value(435), s.ad_value(163))), A::abs(A::mul(s.ad_value(435), s.ad_value(163)))), A::abs(A::mul(s.ad_value(435), s.ad_value(163)))), A::abs(A::mul(s.ad_value(435), s.ad_value(163))));
        }

        if (((((s.v[418] != 0.0) && (!(s.v[504] != 0.0))) && (!(s.v[519] != 0.0))) && (s.v[520] != 0.0)) && (!(s.v[521] != 0.0))) {
            s.store_powf_ad(436, A::abs(A::mul(s.ad_value(435), s.ad_value(163))), s.v[42]);
        }

        if ((((s.v[418] != 0.0) && (!(s.v[504] != 0.0))) && (!(s.v[519] != 0.0))) && (s.v[520] != 0.0)) {
            s.store_div_from_scalar_ad(462, 1.0, A::sub_from_scalar(1.0, s.ad_value(436)));
        }

        if ((((s.v[418] != 0.0) && (!(s.v[504] != 0.0))) && (!(s.v[519] != 0.0))) && (!(s.v[520] != 0.0))) {
            s.store_offset_ad(462, A::mul(A::add(s.ad_value(435), A::scale(s.ad_value(39), s.v[158])), s.ad_value(166)), s.v[160]);
        }

        if ((s.v[418] != 0.0) && (!(s.v[504] != 0.0))) {
            s.store_mul_ad_lhs(270, A::add(A::add(A::add(s.ad_value(437), s.ad_value(438)), s.ad_value(445)), s.ad_value(460)), 462);
        }

        if ((s.v[418] != 0.0) && (!(s.v[504] != 0.0))) {
            s.store_mul_ad_lhs(292, A::add(A::add(s.ad_value(438), s.ad_value(445)), s.ad_value(460)), 462);
        }

        s.v[522] = if (s.v[258] == 0.0) { 1.0 } else { 0.0 };

        if ((s.v[418] != 0.0) && (s.v[522] != 0.0)) {
            s.store_scalar(272, 0.0);
        }

        if ((s.v[418] != 0.0) && (s.v[522] != 0.0)) {
            s.store_scalar(293, 0.0);
        }

        if ((s.v[418] != 0.0) && (s.v[522] != 0.0)) {
            s.store_scalar(273, 0.0);
        }

        s.v[523] = if (s.v[124] == 0.5) { 1.0 } else { 0.0 };

    }
}
